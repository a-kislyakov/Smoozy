//! The acts this module performs on a machine from the desktop, over the
//! connection Smoozy already holds.
//!
//! **Y7 splits the run's work in two and this file is one of the two halves.**
//! *"Machine-side steps (fetch, unpack, `ldd` reads, publication, read-backs)
//! are performed by the setup worker, commanded one step at a time"*, and
//! **elevated steps — package installs, driver work, hand-over, the restart —
//! are performed over the SSH command channel** by the machine's own elevating
//! command. **The second half is here.** The first half is the setup worker's
//! and is `Rented-server runtime`'s to host; what this crate leaves to it is
//! named clause by clause at [`crate::arriving_later`].
//!
//! **Every judgement is next door.** What this file does is carry an act out and
//! report what the machine said. Which act to carry out, in what order, and what
//! its outcome means are [`crate::ready_action`], [`crate::run`],
//! [`crate::recovery`] and [`crate::statements`]'s, and none of it is retaken
//! here.
//!
//! **Two surfaces and no third.** A value that is a path rides the
//! file-transfer subsystem, where it is a protocol field; a value that must
//! reach a program's arguments rides one of `Server protocol and transfers`
//! 1.1's carriages or is fixed at build time. **No command string in this file
//! is composed** — the type forbids it, and every one of them is
//! [`crate::elevated`]'s.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` §1.1, §7, §7.8,
//! S24, S27. `Job files and outputs` L4a, L10, L11, N11. `Rented-server runtime` K1,
//! K2, P4, V9. `Security and licence` §10, §13. `State, identity and recovery`
//! §1.3.

use smoozy_job_files::layout::TheLayout;
use smoozy_server_protocol::carriage::CarriageSite;
use smoozy_server_protocol::transport::CarriesAValueOnACommandsStandardInput;
use smoozy_servers::readings::CommandChannel;

use crate::elevated::{
    issue, AnElevatedStep, HowElevationIsObtained, WhatTheElevatedStepSaid, WhyTheStepDidNotRun,
};
use crate::placement::{the_areas_to_create, the_hand_overs, OneHandOver};
use crate::readings::{TheMachinesFileSurface, WhatTheMachineSaid};
use crate::search::ACandidate;

/// What one act of this file came to.
///
/// **The machine's own words ride every failure and are never mined for a
/// cause** — `Security and licence` §13 rule 5 and `Rented-server runtime` M24's
/// discipline, cited rather than restated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowTheActWent {
    /// It was carried out and its own check passed.
    ItWasCarriedOut,
    /// The machine refused it, with what it said kept whole.
    TheMachineRefusedIt { the_machines_own_words: String },
    /// The act could not be attempted at all.
    ItCouldNotBeAttempted(WhyTheStepDidNotRun),
    /// A path the act needed could not be reached.
    ThePathWouldNotTake(WhatTheMachineSaid),
}

impl HowTheActWent {
    pub fn it_was_carried_out(&self) -> bool {
        matches!(self, HowTheActWent::ItWasCarriedOut)
    }

    fn from(said: Result<WhatTheElevatedStepSaid, WhyTheStepDidNotRun>) -> HowTheActWent {
        match said {
            Ok(said) if said.it_ended_cleanly => HowTheActWent::ItWasCarriedOut,
            Ok(said) => HowTheActWent::TheMachineRefusedIt {
                the_machines_own_words: said.the_machines_own_words,
            },
            Err(why) => HowTheActWent::ItCouldNotBeAttempted(why),
        }
    }

    /// **What the act came to, in words a screen may carry.** The machine's
    /// own words travel whole where it printed any, and are never mined for a
    /// cause; where it printed none, this application's own knowledge of what
    /// happened is said instead. **Never a debugging rendering** — that is
    /// what reached the screen on the first pass on a real rental, 2026-09-21.
    pub fn in_words(&self) -> String {
        match self {
            HowTheActWent::ItWasCarriedOut => String::new(),
            HowTheActWent::TheMachineRefusedIt {
                the_machines_own_words,
            } => the_machines_own_words.clone(),
            HowTheActWent::ItCouldNotBeAttempted(why) => why.in_words().to_string(),
            HowTheActWent::ThePathWouldNotTake(said) => said.in_words().to_string(),
        }
    }
}

// ------------------------------------------------------------- the hand-over

/// **R7's landing check, and its name is Smoozy's own.**
///
/// *"Each confirmed by its own real write of at least one byte at its exact
/// path — never a writability test and never an empty file"* (L11; `State,
/// identity and recovery` §1.3 rule 4). **One byte is what is written**, and the
/// file is read back before it is taken away, because a write that reported
/// success and left nothing is exactly what an empty-file test cannot tell from
/// one that worked.
pub const THE_LANDING_CHECKS_NAME: &str = ".smoozy-landing-check";

/// The one byte the landing check writes.
pub const THE_LANDING_CHECKS_BYTE: &[u8] = b"s";

/// What one hand-over came to, per mount.
///
/// **The carriage is per mount, so nothing about the second is inferred from
/// the first's exit or from the first's landing check** (R7a).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HowAHandOverWent {
    /// The exact path the hand-over was for.
    pub the_target_path: String,
    /// **R7: how the creation of that directory went**, which is the first step
    /// of the placement and comes before the confirming write and before the
    /// five areas.
    pub the_directory_was_created: HowTheActWent,
    /// How the elevated command itself went.
    pub the_command: HowTheActWent,
    /// **R7's own evidence.** *"The command's exit is never that evidence."*
    pub the_landing_check: HowTheActWent,
    /// **§7.8's discipline**: a link whose removal could not run is an owed
    /// cleanup, never a forgotten one.
    pub the_links_removal_is_owed: bool,
}

impl HowAHandOverWent {
    /// **R7a: the landing check is what proves the ownership landed**, for the
    /// owner half exactly as for the path half.
    pub fn the_ownership_landed(&self) -> bool {
        self.the_landing_check.it_was_carried_out()
    }
}

/// **The directory every carriage token of this module sits in, made before the
/// first one is written into it.**
///
/// It is `Server protocol and transfers` §1.1's own constant — the link *"lives
/// at its token path under a Smoozy-named directory in the login's own writable
/// area"* — and **a fresh machine has none**, so something has to make it. That
/// crate's own [`smoozy_server_protocol::carriage::plant`] makes it as the first
/// act of planting, and this module plants through its own file seam rather than
/// through that call, so the same act belongs here. **Nothing in this crate made
/// it until 2026-09-04**: the function existed, in `crate::wiring`, and no caller
/// anywhere reached it, while four tests handed the stand-in a machine that
/// already had the directory.
///
/// **A directory that is already there is the ordinary case on a re-run and is
/// not a fault**, which is the same reading the placement's own creation takes.
///
/// **It is above the seam and not inside its real end**, so a test at the desk
/// runs the act rather than the real machine's version of it — which is what a
/// stand-in answering `Ok(())` to every plant hid twice.
pub async fn make_the_carriage_directory(
    files: &mut impl TheMachinesFileSurface,
) -> Result<(), WhatTheMachineSaid> {
    match files
        .make_one_directory(smoozy_server_protocol::carriage::THE_DIRECTORY_ON_THE_SUBSYSTEM)
        .await
    {
        Ok(()) | Err(WhatTheMachineSaid::ItIsAlreadyThere) => Ok(()),
        Err(said) => Err(said),
    }
}

/// **One carriage of this module, planted whole**: the directory first, then the
/// link at the site's own token.
///
/// **The two are one act because the second cannot land without the first.**
/// `Server protocol and transfers` §1.1's own performance of this carriage takes
/// the same order for the same reason, and a site that plants without it is a
/// site that works only on a machine somebody else prepared.
pub async fn plant_the_carriage(
    files: &mut impl TheMachinesFileSurface,
    site: CarriageSite,
    target: &str,
) -> Result<(), WhatTheMachineSaid> {
    make_the_carriage_directory(files).await?;
    files
        .plant_a_link(target, site.link_path_on_the_subsystem)
        .await
}

/// **A carriage site's turn on one server**, taken before the link is planted
/// and held until it is removed. Added 2026-09-30, step 10c's eleventh-pass fix
/// piece 4c, moving piece 4b's turn out of `Render Settings`' four-value
/// reading so **every site more than one act reaches takes turns the same
/// way**: that reading and [`ask_one_build_its_version`] both hold it.
///
/// **What it stops.** A site is one fixed name in the login's home. **Two acts
/// at one site on one server share it**: the second's link replaces the
/// first's before the first's command runs, so the first reads the second's
/// target — at exit 0, a real answer about the wrong file — or the first's
/// removal takes the second's link away and the second reads nothing.
///
/// **Why a turn and not a token per act.** *"One site, one token, one carriage
/// live at a time"* is the carriage's own rule (`Server protocol and transfers`
/// §1.1, `smoozy_server_protocol::carriage`): a command string is a build-time
/// constant, so its token is too.
///
/// **Why per server**: the link sits in each machine's own home, so acts on two
/// servers never meet, and a slow server must not hold back another. **Why per
/// site**: two sites are two links, and a version ask of up to a minute must
/// not hold back a page's four-value reading on the same server.
///
/// **`on_the_server` is the local server identifier as the record writes it**,
/// and every caller has to name a server the same way — a caller that named it
/// differently would get a turn of its own.
///
/// **What it costs**: an act waits for another act at the same site on the
/// same server to end.
pub async fn take_the_turn_at(
    site: CarriageSite,
    on_the_server: &str,
) -> tokio::sync::OwnedMutexGuard<()> {
    type TheTurns =
        std::collections::HashMap<(&'static str, String), std::sync::Arc<tokio::sync::Mutex<()>>>;
    static TURNS: std::sync::OnceLock<std::sync::Mutex<TheTurns>> = std::sync::OnceLock::new();
    let turn = {
        let mut turns = TURNS
            .get_or_init(Default::default)
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        std::sync::Arc::clone(
            turns
                .entry((site.link_path_on_the_subsystem, on_the_server.to_string()))
                .or_default(),
        )
    };
    turn.lock_owned().await
}

/// **R7, performed: the directory this placement's hand-over lands on is made
/// before anything is written into it.**
///
/// **Nothing anywhere in the application made it until 2026-09-04**, so on a
/// machine never prepared the confirming write had nowhere to land and the
/// placement could never finish. **R7 gained the creation out of this module's
/// round-1 code audit and this is that clause performed.**
///
/// **The plain create over the file subsystem is tried first**, because on a
/// container whose login cannot install packages nothing elevated runs at all
/// (R3, M2) and such a machine must still be able to place a root under a
/// location it owns. **A directory that is already there is the ordinary case on
/// a re-run and is not a fault**, which is S7's own safety.
///
/// **Where the plain create is refused it is made by an elevated constant
/// command through R7a's carriage**: the link's target is the location the
/// directory sits in, the command names the token and `Job files and outputs`'
/// own root segment, and **nothing chosen enters a command string**.
///
/// **The command's exit is not the evidence.** R7's real write of at least one
/// byte at the exact target path is, exactly as it is for the hand-over itself —
/// so this reports what the machine said and decides nothing on it.
pub async fn create_the_directory(
    channel: &mut (impl CommandChannel + CarriesAValueOnACommandsStandardInput),
    files: &mut impl TheMachinesFileSurface,
    owed: &OneHandOver,
    step: AnElevatedStep,
    how: HowElevationIsObtained<'_>,
) -> HowTheActWent {
    match files.make_one_directory(&owed.the_target_path).await {
        Ok(()) => return HowTheActWent::ItWasCarriedOut,
        Err(WhatTheMachineSaid::ItIsAlreadyThere) => return HowTheActWent::ItWasCarriedOut,
        Err(_) => {}
    }
    // **The elevated route is composed only where the path is composed the way
    // `Job files and outputs` composes one**, because the command's segment is
    // that document's build constant and naming another would be composing a
    // path here.
    let Some(location) = owed.the_location_it_sits_in.as_deref() else {
        return HowTheActWent::ThePathWouldNotTake(WhatTheMachineSaid::TheLoginMayNot);
    };
    if let Err(said) = plant_the_carriage(files, owed.created_at, location).await {
        return HowTheActWent::ThePathWouldNotTake(said);
    }
    let made = HowTheActWent::from(issue(channel, step, how).await);
    let _ = files
        .remove_one_file(owed.created_at.link_path_on_the_subsystem)
        .await;
    made
}

/// **R7 and R7a, performed for one mount.**
///
/// In order: the link is planted at the site's own token, the elevated `chown`
/// names the token, the real write at the exact target path is what proves the
/// ownership landed, and the link is taken away when the step ends.
///
/// **The link lives in the login's own writable area and never on the mount it
/// points at**, which is `Server protocol and transfers` 1.1's own rule and is
/// that crate's to keep — [`smoozy_server_protocol::carriage::plant`] is what
/// keeps it.
pub async fn hand_one_mount_over(
    channel: &mut (impl CommandChannel + CarriesAValueOnACommandsStandardInput),
    files: &mut impl TheMachinesFileSurface,
    owed: &OneHandOver,
    step: AnElevatedStep,
    creation: AnElevatedStep,
    how: HowElevationIsObtained<'_>,
) -> HowAHandOverWent {
    // **R7: the directory is made before anything is written into it**, and
    // before the five areas.
    let the_directory_was_created =
        create_the_directory(channel, files, owed, creation, how).await;

    let planted = plant_the_carriage(files, owed.carried_at, &owed.the_target_path).await;

    if let Err(said) = planted {
        return HowAHandOverWent {
            the_target_path: owed.the_target_path.clone(),
            the_directory_was_created,
            the_command: HowTheActWent::ThePathWouldNotTake(said),
            the_landing_check: HowTheActWent::ThePathWouldNotTake(
                WhatTheMachineSaid::ThePathIsNotThere,
            ),
            the_links_removal_is_owed: false,
        };
    }

    let the_command = HowTheActWent::from(issue(channel, step, how).await);
    // **The landing check runs whatever the command's exit said**, because
    // R7a's whole point is that a recursive hand-over through a link without the
    // traverse option changes nothing on the mount and **nothing in its exit
    // status says so.**
    let the_landing_check = confirm_the_landing(files, &owed.the_target_path).await;
    let the_links_removal_is_owed = files
        .remove_one_file(owed.carried_at.link_path_on_the_subsystem)
        .await
        .is_err();

    HowAHandOverWent {
        the_target_path: owed.the_target_path.clone(),
        the_directory_was_created,
        the_command,
        the_landing_check,
        the_links_removal_is_owed,
    }
}

/// **R7's real write of at least one byte at the exact target path**, read back
/// and then taken away.
pub async fn confirm_the_landing(
    files: &mut impl TheMachinesFileSurface,
    at_the_exact_path: &str,
) -> HowTheActWent {
    let path = format!("{at_the_exact_path}/{THE_LANDING_CHECKS_NAME}");
    if let Err(said) = files.write_whole(&path, THE_LANDING_CHECKS_BYTE).await {
        return HowTheActWent::ThePathWouldNotTake(said);
    }
    match files.read_whole(&path).await {
        Ok(read_back) if read_back == THE_LANDING_CHECKS_BYTE => {
            // Smoozy's own byte in Smoozy's own root; a removal that could not
            // run leaves one byte and nothing else.
            let _ = files.remove_one_file(&path).await;
            HowTheActWent::ItWasCarriedOut
        }
        Ok(_) => HowTheActWent::TheMachineRefusedIt {
            the_machines_own_words: String::new(),
        },
        Err(said) => HowTheActWent::ThePathWouldNotTake(said),
    }
}

/// **R7, performed whole: the directory made, the hand-over, and then the
/// areas — twice where there are two mounts.**
///
/// **The creation comes first and it is R7's own step**: the confirming write
/// lands at the root's exact path and the five areas sit inside it, so a
/// placement that does not make it can never finish on a machine that has never
/// been prepared. See [`create_the_directory`].
///
/// **The areas' creation and the real writes are file-transfer-subsystem
/// operations whose paths are protocol fields and need no command at all**
/// (R7a's closing sentence). **The five areas are created so no deep creation is
/// ever needed** (`Job files and outputs` 1.3, 2.4 N11).
pub async fn carry_out_the_placement(
    channel: &mut (impl CommandChannel + CarriesAValueOnACommandsStandardInput),
    files: &mut impl TheMachinesFileSurface,
    layout: &TheLayout,
    how: HowElevationIsObtained<'_>,
) -> ThePlacement {
    let owed = the_hand_overs(layout);
    let mut hand_overs = Vec::new();
    for (at, one) in owed.iter().enumerate() {
        let (step, creation) = if at == 0 {
            (
                crate::elevated::THE_HAND_OVER_OF_THE_ROOT,
                crate::elevated::THE_CREATION_OF_THE_ROOT,
            )
        } else {
            (
                crate::elevated::THE_HAND_OVER_OF_THE_PACKAGE_AREA,
                crate::elevated::THE_CREATION_OF_THE_PACKAGE_AREAS_LOCATION,
            )
        };
        hand_overs.push(hand_one_mount_over(channel, files, one, step, creation, how).await);
    }

    let mut areas = Vec::new();
    if hand_overs.iter().all(HowAHandOverWent::the_ownership_landed) {
        for area in the_areas_to_create(layout) {
            let made = match files.make_one_directory(&area).await {
                Ok(()) => HowTheActWent::ItWasCarriedOut,
                // A directory that is already there is the ordinary case on a
                // re-run and is not a fault — S7's own safety.
                Err(WhatTheMachineSaid::ItIsAlreadyThere) => HowTheActWent::ItWasCarriedOut,
                Err(said) => HowTheActWent::ThePathWouldNotTake(said),
            };
            areas.push((area, made));
        }
    }

    ThePlacement { hand_overs, areas }
}

/// What the placement came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePlacement {
    /// One entry per mount a choice actually landed on — **never more** (R7).
    pub hand_overs: Vec<HowAHandOverWent>,
    /// The five areas, each with how its creation went. **Empty where a
    /// hand-over did not land**, because creating an area under a mount the
    /// login does not own is a write that fails for a reason the hand-over
    /// already named.
    pub areas: Vec<(String, HowTheActWent)>,
}

impl ThePlacement {
    /// **R7's whole promise: every mount handed over and confirmed, and the
    /// five areas made.**
    pub fn it_is_complete(&self) -> bool {
        !self.hand_overs.is_empty()
            && self.hand_overs.iter().all(HowAHandOverWent::the_ownership_landed)
            && !self.areas.is_empty()
            && self.areas.iter().all(|(_, went)| went.it_was_carried_out())
    }
}

// -------------------------------------------- M1b's Debian components step

/// The three components M1b requires enabled on Debian, in M1b's own order.
pub const THE_COMPONENTS_M1B_REQUIRES: [&str; 3] = ["contrib", "non-free", "non-free-firmware"];

/// **The file M1b's own evidence names**, which recorded the state it found —
/// *"`/etc/apt/sources.list.d/debian.sources` reads `Components: main`"*.
pub const THE_DEBIAN_SOURCES_FILE: &str = "/etc/apt/sources.list.d/debian.sources";

/// The carriage site the rewritten file crosses on, before the elevated copy.
///
/// **The content crosses the subsystem to a Smoozy-synthesised staging token
/// and the elevated command names the token and the fixed destination** — the
/// same admissible pair E7 already declares for the container boot hook.
pub const THE_COMPONENTS_STAGING_SITE: CarriageSite = CarriageSite {
    link_path_on_the_subsystem: ".smoozy-carriage/debian-sources",
    named_in_a_command_as: "~/.smoozy-carriage/debian-sources",
};

/// **M1b's first Debian step, as an edit rather than a command — and that is
/// what the reading settled rather than what anybody guessed.**
///
/// **M1b requires `contrib non-free non-free-firmware` enabled and names no
/// command for it**, and the rules half answered `None` at
/// [`crate::kind::DriverStep::the_command`] rather than writing one from memory.
/// **Debian's own publication was read on 2026-09-04 and it names no command
/// either**: enabling a component is an edit of the sources file's own
/// `Components` field.
///
/// - **Debian's own instruction**, `https://wiki.debian.org/SourcesList`, read
///   2026-09-04: *"If you want access to contrib and non-free components, add "
///   contrib non-free" after "non-free-firmware""* — presented as a manual
///   editing step, with no command named for it anywhere on the page. The one
///   command that page does name, `apt modernize-sources`, converts a file's
///   format and enables nothing.
/// - **The field itself**, `sources.list(5)`, APT 3.0.3 (Debian trixie), page
///   dated 2025-06-24, read 2026-09-04 at
///   `https://manpages.debian.org/trixie/apt/sources.list.5.en.html`: files in
///   the newer format *"have the extension .sources"*, and *"component is one of
///   main, contrib, non-free or non-free-firmware"*, the field's values being
///   separated by whitespace.
///
/// **So the step is a file edit and this is that edit, as a pure function.**
/// Every line that is not a `Components` field is returned byte for byte; a
/// `Components` field gains the names it lacks, in M1b's own order, and loses
/// nothing. **A file that names no component at all is not repaired** — that is
/// a file this reading does not describe, and inventing a field for it would be
/// writing a sources file from memory.
pub fn rewrite_the_components(text: &str) -> Option<String> {
    const FIELD: &str = "Components:";
    let mut found = false;
    let mut changed = false;
    let mut out = String::new();
    for line in text.split_inclusive('\n') {
        let (body, ending) = match line.strip_suffix('\n') {
            Some(body) => (body, "\n"),
            None => (line, ""),
        };
        let Some(rest) = body.strip_prefix(FIELD) else {
            out.push_str(line);
            continue;
        };
        found = true;
        let mut named: Vec<&str> = rest.split_whitespace().collect();
        for component in THE_COMPONENTS_M1B_REQUIRES {
            if !named.contains(&component) {
                named.push(component);
                changed = true;
            }
        }
        out.push_str(FIELD);
        for component in &named {
            out.push(' ');
            out.push_str(component);
        }
        out.push_str(ending);
    }
    if !found {
        return None;
    }
    if changed {
        Some(out)
    } else {
        // **Already enabled is a step with nothing to do**, which is Y4a's
        // *nothing to do* rather than a failure.
        None
    }
}

/// **M1b's components step, performed.**
///
/// The file is read over the file-transfer subsystem, rewritten at the desk,
/// written to the staging token, copied to its fixed destination by an elevated
/// constant command, and the lists are updated. **Nothing of machine origin
/// enters a command string**: the rewritten content is a file, and both paths in
/// the command are fixed at build time.
pub async fn enable_the_components(
    channel: &mut (impl CommandChannel + CarriesAValueOnACommandsStandardInput),
    files: &mut impl TheMachinesFileSurface,
    how: HowElevationIsObtained<'_>,
) -> HowTheComponentsStepWent {
    let held = match files.read_whole(THE_DEBIAN_SOURCES_FILE).await {
        Ok(bytes) => bytes,
        Err(said) => {
            return HowTheComponentsStepWent::TheFileCouldNotBeRead(said);
        }
    };
    let Ok(text) = String::from_utf8(held) else {
        // **A sources file that is not text is a file this reading does not
        // describe**, and nothing is written over it.
        return HowTheComponentsStepWent::TheFileIsNotOneThisReadingDescribes;
    };
    let Some(rewritten) = rewrite_the_components(&text) else {
        return HowTheComponentsStepWent::NothingToDo;
    };
    // **The staging token sits under the carriage directory too**, and a write
    // into a folder nobody made is refused — so the directory is made here for
    // the same reason a planted link's is.
    if let Err(said) = make_the_carriage_directory(files).await {
        return HowTheComponentsStepWent::TheStagingWriteWasRefused(said);
    }
    if let Err(said) = files
        .write_whole(
            THE_COMPONENTS_STAGING_SITE.link_path_on_the_subsystem,
            rewritten.as_bytes(),
        )
        .await
    {
        // **A refused staging write is not a file that could not be read**, and
        // it stopped being a naming fault the moment F-5's gate landed: the four
        // cases now decide whether a driver install goes out at all, and the
        // machine's own words are what the user is shown.
        return HowTheComponentsStepWent::TheStagingWriteWasRefused(said);
    }
    let went = carry_out_the_two_elevated_acts(channel, how).await;
    // **The staged file is taken back when the step ends, on every route out of
    // it** — `Server protocol and transfers` 1.1's staging discipline for a
    // Smoozy-synthesised token, which its §7.8 governs and which the version
    // ask's planted link already keeps at [`ask_one_build_its_version`].
    //
    // **It was left behind until 2026-09-04**, when a rented machine was looked
    // at after the step and still held it: a copy of the machine's own sources
    // file sat under the login's home for the life of the rental, written by
    // Smoozy and removed by nothing. **Observed in a controlled test** on
    // Hyperstack `Debian 12 Plain`, 2026-09-04.
    //
    // **A removal that could not run is that document's owed cleanup and no
    // sweep is invented here** — the same shape, and the same silence about the
    // failure, as the version ask's own removal. **What breaks where it does not
    // run** is exactly what broke before: the file stays.
    let _ = files
        .remove_one_file(THE_COMPONENTS_STAGING_SITE.link_path_on_the_subsystem)
        .await;
    went
}

/// M1b's two elevated acts, in order, so the staging removal above has one
/// place to sit rather than three.
async fn carry_out_the_two_elevated_acts(
    channel: &mut (impl CommandChannel + CarriesAValueOnACommandsStandardInput),
    how: HowElevationIsObtained<'_>,
) -> HowTheComponentsStepWent {
    let written =
        HowTheActWent::from(issue(channel, crate::elevated::THE_COMPONENTS_WRITE, how).await);
    if !written.it_was_carried_out() {
        return HowTheComponentsStepWent::ItWasRefused(written);
    }
    let updated = HowTheActWent::from(issue(channel, crate::elevated::THE_LIST_UPDATE, how).await);
    if !updated.it_was_carried_out() {
        return HowTheComponentsStepWent::ItWasRefused(updated);
    }
    HowTheComponentsStepWent::ItWasCarriedOut
}

/// What M1b's components step came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowTheComponentsStepWent {
    /// The components were enabled and the lists updated.
    ItWasCarriedOut,
    /// **Y4a's second mark**: every component M1b requires was already named.
    NothingToDo,
    /// The machine refused a step of it, with its own words behind it.
    ItWasRefused(HowTheActWent),
    /// **The sources file could not be read.**
    ///
    /// **It used to cover the staging write as well**, and its own comment said
    /// so, so a refused write of the rewritten file was reported as a file that
    /// could not be read. **The staging write got its own value on 2026-09-04**,
    /// out of this module's round-1 code audit.
    TheFileCouldNotBeRead(WhatTheMachineSaid),
    /// **The write of the rewritten file to the staging token was refused.**
    ///
    /// **It ends the driver step exactly as the reading failure does** — no
    /// install and no restart go out — and what differs is what the user is
    /// told happened.
    TheStagingWriteWasRefused(WhatTheMachineSaid),
    /// **The file is not text, so the edit Debian's own instruction describes
    /// does not apply to it and nothing is written over it.**
    TheFileIsNotOneThisReadingDescribes,
}

impl HowTheComponentsStepWent {
    /// **What the components step came to, in words a screen may carry** —
    /// the machine's own where it printed any, and never a debugging rendering.
    pub fn in_words(&self) -> String {
        match self {
            HowTheComponentsStepWent::ItWasCarriedOut | HowTheComponentsStepWent::NothingToDo => {
                String::new()
            }
            HowTheComponentsStepWent::ItWasRefused(went) => went.in_words(),
            HowTheComponentsStepWent::TheFileCouldNotBeRead(said) => {
                format!("Could not read the package sources file: {}", said.in_words())
            }
            HowTheComponentsStepWent::TheStagingWriteWasRefused(said) => format!(
                "Could not write the package sources file: {}",
                said.in_words()
            ),
            HowTheComponentsStepWent::TheFileIsNotOneThisReadingDescribes => {
                "The package sources file is not a text file. Nothing was changed.".to_string()
            }
        }
    }
}

// --------------------------------------------------------- the driver route

/// What the driver step came to, step by step, in R1 item 5's own order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HowTheDriverStepWent {
    /// **The check, run first.** *"Where it answers, nothing is installed and
    /// nothing is touched."*
    pub the_check_before: crate::readings::WhatTheDriverCheckSaid,
    /// **Which of R1 item 5's branches the step took**, which Y4a's closed set
    /// of two cannot say: *performed* and *nothing to do* do not tell a restart
    /// tried before an install from an install.
    ///
    /// **The branch is [`crate::ready_action::what_the_driver_step_does`]'s**,
    /// asked here rather than decided again.
    pub which_branch: crate::ready_action::WhatTheDriverStepDoes,
    /// M1b's components step, on Debian and nowhere else.
    pub the_components: Option<HowTheComponentsStepWent>,
    /// **R1 item 5's Ubuntu first act, the detection tool made reachable**, on
    /// Ubuntu and nowhere else. Added 2026-09-21.
    pub the_tool: Option<HowTheActWent>,
    /// The install, where one was reached.
    pub the_install: Option<HowTheActWent>,
    /// **R1 item 5's Ubuntu last act, the driver support binaries the check is
    /// run from**, on Ubuntu and nowhere else. Added 2026-09-23, out of step
    /// 10c's fifth pass on Hyperstack, where the driver installed, loaded and
    /// answered the kernel — and `nvidia-smi`, which is the only thing R1 item
    /// 5 checks a driver with, was on the machine nowhere.
    ///
    /// **It is the publication's own next command** and the reasoning, the
    /// quotations and the addresses are at
    /// [`crate::elevated::THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL`].
    pub the_support_binaries: Option<HowTheActWent>,
    /// **R1 item 5's Ubuntu fifth act, the OptiX library put on where the
    /// driver left it out**, on Ubuntu and nowhere else. Added 2026-10-03,
    /// with the Creator's ruling that NVIDIA cards render with OptiX
    /// (`Rendering` L23); the lookup, the package and their publications are
    /// at [`crate::elevated::THE_UBUNTU_OPTIX_LIBRARY_INSTALL`].
    pub the_optix_library: Option<HowTheActWent>,
    /// The check again, where anything was done.
    pub the_check_after: Option<crate::readings::WhatTheDriverCheckSaid>,
    /// **R1 item 5's swap up to the OptiX bar**, where the step took
    /// [`crate::ready_action::WhatTheDriverStepDoes::TheSwapUpToTheBar`]. Added
    /// 2026-10-11; the act is [`crate::elevated::THE_UBUNTU_DRIVER_SWAP`].
    pub the_swap: Option<HowTheActWent>,
    /// **The driver's version, read after the check again answered**, and only
    /// where the run has a bar to read it against (`Servers` H11b's query).
    /// What the step's last word on the bar is read from where the check was
    /// run again; where it was not, the version the step was given stands.
    pub the_version_after: Option<smoozy_servers::hardware::DriverVersion>,
}

impl HowTheDriverStepWent {
    /// **M1b: whether the components step let the driver install go out at
    /// all.**
    ///
    /// *"The Debian route needs `contrib non-free non-free-firmware` enabled
    /// first, because the stock image reaches `main` only and the driver is
    /// otherwise not merely uninstalled but unreachable."* **The step's answer
    /// was kept in a value and looked at by nothing until 2026-09-04**: where
    /// the sources file could not be read, or is not one the reading describes,
    /// or the elevated copy was refused, the install went out anyway, failed
    /// because the driver was unreachable, the check after it failed, and **the
    /// restart was issued — a paid machine went down for between nine and ninety
    /// seconds to fix nothing.**
    pub fn the_components_let_the_install_go_out(
        components: &Option<HowTheComponentsStepWent>,
    ) -> bool {
        match components {
            // Not this family, so there is no components step to have failed.
            None => true,
            Some(HowTheComponentsStepWent::ItWasCarriedOut) => true,
            Some(HowTheComponentsStepWent::NothingToDo) => true,
            Some(_) => false,
        }
    }

    /// **Y4a's closed set of two, decided by the act the step performed.**
    ///
    /// **The restart is not one of this step's acts and never was.** R1 item 6
    /// is the restart and Y4's *driver placed* row guarantees *"no restart has
    /// been issued"* at this step's boundary, so a restart cannot be part of
    /// what this step did.
    pub fn what_the_step_did(&self) -> crate::run::WhatTheStepDid {
        if self.the_install.is_some()
            || self.the_swap.is_some()
            || self.the_tool.is_some()
            || self.the_support_binaries.is_some()
            || self.the_optix_library.is_some()
        {
            crate::run::WhatTheStepDid::Performed
        } else {
            crate::run::WhatTheStepDid::NothingToDo
        }
    }

    /// **Whether the step's own words say anything went wrong**, so the caller
    /// can end the run in the machine's own words rather than carry on to the
    /// restart on a step that did not run.
    pub fn what_went_wrong(&self) -> Option<String> {
        // **R1 item 5's swap: a refusal is not what went wrong while the
        // driver still answers** — the run carries on and the bar is recorded
        // as missed. **Where the driver no longer answers after it**, the old
        // branch went and the new one did not come, and that is the item's
        // existing failure, in the swap's own words. Added 2026-10-11.
        if let Some(went) = &self.the_swap {
            if !went.it_was_carried_out() && !self.the_driver_answers() {
                return Some(went.in_words());
            }
            return None;
        }
        if let Some(components) = &self.the_components {
            if !HowTheDriverStepWent::the_components_let_the_install_go_out(&self.the_components) {
                return Some(components.in_words());
            }
        }
        if let Some(went) = &self.the_tool {
            if !went.it_was_carried_out() {
                return Some(went.in_words());
            }
        }
        if let Some(went) = &self.the_install {
            if !went.it_was_carried_out() {
                return Some(went.in_words());
            }
        }
        // **R1 item 5's Ubuntu last act, and a refusal here ends the step.**
        // Without the support binaries the check has nothing to run, so the
        // step cannot reach its own end — and a restart cannot put a missing
        // package on a machine, so none goes out. Added 2026-09-23.
        if let Some(went) = &self.the_support_binaries {
            if !went.it_was_carried_out() {
                return Some(went.in_words());
            }
        }
        // **R1 item 5's Ubuntu fifth act: a refused install ends the step in
        // apt's own words, like every other act here.** Added 2026-10-03. A
        // library still not found after a clean install is not a refusal and
        // does not end the setup — the act's own command answers `0` then, and
        // `Rendering` L24b says it when a cards job reaches the machine.
        match &self.the_optix_library {
            Some(went) if !went.it_was_carried_out() => Some(went.in_words()),
            _ => None,
        }
    }

    /// **R4's claim, and no more**: the driver answers. Where the check after
    /// the work did not pass, the machine is not ready and W4's refusal is the
    /// ending rather than a wrong render.
    pub fn the_driver_answers(&self) -> bool {
        match self.the_check_after.as_ref().unwrap_or(&self.the_check_before) {
            crate::readings::WhatTheDriverCheckSaid::ItAnswered => true,
            crate::readings::WhatTheDriverCheckSaid::ItDidNot { .. } => false,
        }
    }

    /// **The driver's version at the step's end**: read after the check again
    /// where it was run, and otherwise the one the step was given.
    pub fn the_drivers_version_at_the_end<'a>(
        &'a self,
        needs: &'a crate::ready_action::WhatTheMachineNeeds,
    ) -> Option<&'a smoozy_servers::hardware::DriverVersion> {
        if self.the_check_after.is_some() {
            self.the_version_after.as_ref()
        } else {
            needs.the_drivers_version.as_ref()
        }
    }

    /// **R1 item 5's check, as item 6 reads it: whether the step's work needs
    /// no restart.** Added 2026-10-11.
    ///
    /// **After a swap that went through, the check passes only where the
    /// driver answers at or above the bar** — *"Otherwise step 6's restart
    /// follows, and the check runs again on return."* **After a swap that was
    /// refused, nothing went on, so no restart is owed** wherever the driver
    /// still answers (*"the run carries on"*). **Everywhere else it is the
    /// driver answering, as before.**
    pub fn the_check_passes(&self, needs: &crate::ready_action::WhatTheMachineNeeds) -> bool {
        if !self.the_driver_answers() {
            return false;
        }
        match &self.the_swap {
            Some(went) if went.it_was_carried_out() => !crate::ready_action::the_driver_is_below_the_bar(
                needs,
                self.the_drivers_version_at_the_end(needs),
            ),
            _ => true,
        }
    }

    /// **Whether the step ends with a driver that answers below the bar** of
    /// the Blender the run installs, on a machine Smoozy manages — R1 item 5's
    /// *the step record carries that the bar was not reached*. Added
    /// 2026-10-11. Read where no restart follows; after a restart, the check
    /// on return is what says it.
    ///
    /// **A swap left out because work is live is not a miss** (R1b,
    /// 2026-10-12): the swap waits, and the bar is not recorded as missed.
    pub fn the_driver_missed_the_bar(&self, needs: &crate::ready_action::WhatTheMachineNeeds) -> bool {
        !needs.anything_is_live
            && self.the_driver_answers()
            && crate::ready_action::the_driver_is_below_the_bar(
                needs,
                self.the_drivers_version_at_the_end(needs),
            )
    }
}

/// **The driver's version, asked where the run has a bar to read it against
/// and the check answered** — `Servers` H11b's own query, and nothing of it is
/// re-derived here. Added 2026-10-11.
pub async fn the_drivers_version_where_a_bar_applies(
    channel: &mut impl CommandChannel,
    needs: &crate::ready_action::WhatTheMachineNeeds,
    check: &crate::readings::WhatTheDriverCheckSaid,
) -> Option<smoozy_servers::hardware::DriverVersion> {
    if needs.the_optix_bar.is_none()
        || !matches!(needs.kind, crate::kind::MachineKind::ASystemdMachine)
        || !matches!(check, crate::readings::WhatTheDriverCheckSaid::ItAnswered)
    {
        return None;
    }
    smoozy_servers::readings::ask_the_drivers_versions(channel).await.0
}

/// **R1 item 5, performed in the order the clause fixes, and the order is the
/// whole of the safety.**
///
/// **Check first.** *"`nvidia-smi` run and its exit read; where it answers,
/// nothing is installed and nothing is touched."* R5 forbids repeating a driver
/// install whose check passes — the reinstall-becomes-upgrade observation is the
/// exact shape of *running it again makes it worse* — and this order is what
/// makes S4's re-run unable to reach it.
///
/// **Then, where the check failed**: on Debian the components are enabled first,
/// because the stock image reaches `main` only and the driver is otherwise *not
/// merely uninstalled but unreachable*; then the family's own detection route.
///
/// **The restart is not this step's and is not issued here.** R1 item 6 is the
/// restart — *"only where step 5's check failed after its work"* — and Y4's
/// *driver placed* boundary, which is written when this step ends, guarantees
/// *"no restart has been issued"*. **So this function performs item 5 and stops,
/// and the caller reads [`HowTheDriverStepWent::the_driver_answers`] to decide
/// item 6.** On Debian M1b makes that restart not optional, because the module
/// may be built against a kernel the machine is not running and *"the route
/// looks broken and is not"* — which is honoured by the check after the install
/// being what decides, since on that family it fails until the boot.
///
/// **This used to issue the restart itself**, inside both of the branches below,
/// which put a restart before the boundary that promises none has been issued
/// and left T4's mark, T6's wait and T5's recovery with nowhere to sit.
///
/// **Never a module unload and reload in place of the restart** (R5): the
/// observed result oscillates and poisons every later reading. **Nothing here
/// does it, and there is no command for it in this crate at all.**
///
/// **Whether a driver is already installed is a reading and it is
/// [`crate::readings::WhatTheDriverCheckSaid::a_driver_is_already_installed`]'s**
/// — the check's own published exit, taken off the check this step is given,
/// **and, since 2026-09-30, the running kernel's module**: R1 item 5 as
/// corrected that day, *"A driver counts as installed for this rule only where
/// the running kernel has its module"*, because on step 10c's eleventh pass a
/// driver's utilities stood with no module for any kernel and the restart
/// could never mend it. The exit was ruled on 2026-09-20 under the Creator's
/// standing delegation; the module reading is the clause's own; the whole
/// reasoning, with the publications behind both, is written at that method and
/// at [`crate::readings::read_the_running_kernels_module`]. **The caller takes
/// both** and hands the answer in on `needs`. **Until then it arrived as a parameter
/// nothing in the application ever supplied**, this function having no caller
/// but its own tests.
///
/// **The check is given rather than taken here**, because the caller has already
/// run it: R1 item 5's *check first* is the caller's own first act, and running
/// it twice would ask a paid machine the same question twice for nothing.
pub async fn carry_out_the_driver_step(
    channel: &mut (impl CommandChannel + CarriesAValueOnACommandsStandardInput),
    files: &mut impl TheMachinesFileSurface,
    needs: &crate::ready_action::WhatTheMachineNeeds,
    the_check_before: crate::readings::WhatTheDriverCheckSaid,
    how: HowElevationIsObtained<'_>,
) -> HowTheDriverStepWent {
    // **The judgement is [`crate::ready_action::what_the_driver_step_does`]'s
    // and it is asked here**, against the readings the caller took at this step
    // rather than against anything a previous sitting recorded (Row 104).
    let which_branch = crate::ready_action::what_the_driver_step_does(needs);

    match which_branch {
        // **Nothing is installed and nothing is touched.**
        crate::ready_action::WhatTheDriverStepDoes::NothingTheCheckAnswered
        | crate::ready_action::WhatTheDriverStepDoes::Nothing => {
            return HowTheDriverStepWent {
                the_check_before,
                which_branch,
                the_components: None,
                the_tool: None,
                the_install: None,
                the_support_binaries: None,
                the_optix_library: None,
                the_swap: None,
                the_version_after: None,
                the_check_after: None,
            };
        }
        // **R1 item 5: where a driver is already installed, the restart is
        // tried before any install** — both observed failure mechanisms are
        // fixed by the boot, and a reinstall on a working-but-unbooted install
        // silently becomes an upgrade the running kernel does not match. **The
        // step performs nothing at all**, and the restart that follows is item
        // 6's, issued by the caller after this step's boundary.
        crate::ready_action::WhatTheDriverStepDoes::TheRestartBeforeAnyInstall => {
            return HowTheDriverStepWent {
                the_check_before,
                which_branch,
                the_components: None,
                the_tool: None,
                the_install: None,
                the_support_binaries: None,
                the_optix_library: None,
                // **Nothing is read again on a machine nothing was done to**;
                // the check after a restart is taken on the machine that came
                // back.
                the_swap: None,
                the_version_after: None,
                the_check_after: None,
            };
        }
        // **R1 item 5's swap up to the OptiX bar, added 2026-10-11**: the one
        // act, then the check again — and, where it answers, the version read
        // again for the bar.
        crate::ready_action::WhatTheDriverStepDoes::TheSwapUpToTheBar => {
            let the_swap = Some(HowTheActWent::from(
                issue(channel, crate::elevated::THE_UBUNTU_DRIVER_SWAP, how).await,
            ));
            let after = crate::readings::run_the_driver_check(channel).await;
            let the_version_after =
                the_drivers_version_where_a_bar_applies(channel, needs, &after).await;
            return HowTheDriverStepWent {
                the_check_before,
                which_branch,
                the_components: None,
                the_tool: None,
                the_install: None,
                the_support_binaries: None,
                the_optix_library: None,
                the_swap,
                the_version_after,
                the_check_after: Some(after),
            };
        }
        crate::ready_action::WhatTheDriverStepDoes::TheDistributionsOwnDetectionRoute => {}
    }

    let family = needs.family;
    let route = crate::kind::TheDriverRoute::for_family(family);
    let the_components = if route.the_components_must_be_enabled_first() {
        Some(enable_the_components(channel, files, how).await)
    } else {
        None
    };

    // **M1b: where the components step came back anything but carried-out or
    // nothing-to-do, the driver step ends here** — the driver is unreachable
    // rather than merely uninstalled, so an install would fail, the check after
    // it would fail, and the restart would take a paid machine down to fix
    // nothing. **No install and no restart go out.**
    if !HowTheDriverStepWent::the_components_let_the_install_go_out(&the_components) {
        return HowTheDriverStepWent {
            the_check_before,
            which_branch,
            the_components,
            the_tool: None,
            the_install: None,
            the_support_binaries: None,
            the_optix_library: None,
            the_swap: None,
            the_version_after: None,
            the_check_after: None,
        };
    }

    // **R1 item 5's Ubuntu first act: the detection tool made reachable.**
    // Added 2026-09-21, from Hyperstack's plain 24.04 image, which had no
    // `ubuntu-drivers`. **Where it is refused the step ends here** in the
    // machine's own words, and neither the install nor the restart goes out —
    // the same rule the Debian components step keeps above.
    let the_tool = match family {
        crate::kind::DistributionFamily::Ubuntu => Some(HowTheActWent::from(
            issue(channel, crate::elevated::THE_UBUNTU_DRIVER_TOOL_INSTALL, how).await,
        )),
        crate::kind::DistributionFamily::Debian => None,
    };
    if the_tool.as_ref().is_some_and(|went| !went.it_was_carried_out()) {
        return HowTheDriverStepWent {
            the_check_before,
            which_branch,
            the_components,
            the_tool,
            the_install: None,
            the_support_binaries: None,
            the_optix_library: None,
            the_swap: None,
            the_version_after: None,
            the_check_after: None,
        };
    }

    let install_step = match family {
        crate::kind::DistributionFamily::Ubuntu => crate::elevated::THE_UBUNTU_DRIVER_INSTALL,
        crate::kind::DistributionFamily::Debian => crate::elevated::THE_DEBIAN_DRIVER_INSTALL,
    };
    let the_install = Some(HowTheActWent::from(issue(channel, install_step, how).await));

    // **R1 item 5's Ubuntu last act: the driver support binaries.** Added
    // 2026-09-23, from step 10c's fifth pass on Hyperstack, where this route
    // ran to its end on a real machine for the first time — thirteen packages,
    // the module loaded, the card answering the kernel — **and `nvidia-smi`,
    // which is the only thing this clause checks a driver with, was on the
    // machine nowhere.**
    //
    // **It is the publication's own next command**, quoted with its address and
    // the date read at
    // [`crate::elevated::THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL`], and it
    // is issued **after** the install because that is the order the publication
    // puts it in and because the branch it names is the one the detection route
    // has just chosen — **read, since 2026-09-30, from the driver package the
    // install left and no longer from the recommended list** (R1 item 5 and
    // E7's fifth carriage as corrected that day). On the eleventh pass the two
    // named different branches and each removed the other's module.
    //
    // **It goes out whatever the install answered**, and deliberately: a
    // machine that already carried its driver gets *"All the available drivers
    // are already installed."* from the tool at exit 0, and that is the other
    // way a machine arrives here with a working driver and no check to run it
    // against. **A refusal here ends the step** — see
    // [`HowTheDriverStepWent::what_went_wrong`] — and no restart goes out,
    // because a restart cannot put a missing package on a machine.
    let the_support_binaries = match family {
        crate::kind::DistributionFamily::Ubuntu => Some(HowTheActWent::from(
            issue(
                channel,
                crate::elevated::THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL,
                how,
            )
            .await,
        )),
        // **Not this family.** Debian's route installs the `nvidia-driver`
        // metapackage its own detection tool names, and the pass of 2026-08-22
        // that ran that route end to end read the driver, the card and its size
        // back off `nvidia-smi` after the restart — so on that family the check
        // is already on the machine the install put it there.
        crate::kind::DistributionFamily::Debian => None,
    };
    if the_support_binaries
        .as_ref()
        .is_some_and(|went| !went.it_was_carried_out())
    {
        return HowTheDriverStepWent {
            the_check_before,
            which_branch,
            the_components,
            the_tool,
            the_install,
            the_support_binaries,
            the_optix_library: None,
            // **Nothing is read again where the step could not finish its own
            // work**, and Y4's *driver placed* is not written over it.
            the_swap: None,
            the_version_after: None,
            the_check_after: None,
        };
    }

    // **R1 item 5's Ubuntu fifth act: the OptiX library, where the driver left
    // it out.** Added 2026-10-03, with the Creator's ruling that NVIDIA cards
    // render with OptiX (`Rendering` L23). Fact 47 saw this route leave
    // `libnvoptix` out on a Hyperstack image, and every cards job on such a
    // machine would end at `Rendering` L24b. **The act looks first, by
    // `Rendering` L24b's own lookup, and installs only where the library is
    // not found** — inside its one command, so a machine that has it is not
    // touched — and it reads the branch the way the fourth act does. It goes
    // after the fourth act because the branch it reads is the one the install
    // has just left, and before the check, which it does not change.
    let the_optix_library = match family {
        crate::kind::DistributionFamily::Ubuntu => Some(HowTheActWent::from(
            issue(
                channel,
                crate::elevated::THE_UBUNTU_OPTIX_LIBRARY_INSTALL,
                how,
            )
            .await,
        )),
        // **Not this family**: whether Debian's `nvidia-driver` metapackage
        // carries the library is not established (§13 item 27), and
        // `Rendering` L24b is what a Debian machine without it meets.
        crate::kind::DistributionFamily::Debian => None,
    };
    if the_optix_library
        .as_ref()
        .is_some_and(|went| !went.it_was_carried_out())
    {
        return HowTheDriverStepWent {
            the_check_before,
            which_branch,
            the_components,
            the_tool,
            the_install,
            the_support_binaries,
            the_optix_library,
            the_swap: None,
            the_version_after: None,
            the_check_after: None,
        };
    }

    // **R1 item 5's *then the check is run again*.** It is the step's own end,
    // and Y4's *driver placed* row is *"the check's outcome after it is
    // recorded"*.
    let after_the_install = crate::readings::run_the_driver_check(channel).await;
    // **The version, where a bar applies and the driver answers**, so a
    // driver the family's route put on below the bar is recorded as having
    // missed it. Added 2026-10-11.
    let the_version_after =
        the_drivers_version_where_a_bar_applies(channel, needs, &after_the_install).await;

    HowTheDriverStepWent {
        the_check_before,
        which_branch,
        the_components,
        the_tool,
        the_install,
        the_support_binaries,
        the_optix_library,
        the_swap: None,
        the_version_after,
        the_check_after: Some(after_the_install),
    }
}

/// **T1's own bound, decided by the caller and never here.**
///
/// A restart is issued *"only on the systemd kind, and only as part of a setup
/// step that needs it"*, the mark is set on the connected period **before the
/// command leaves**, and on a container none is ever issued because it would
/// restart the container and repair nothing.
/// **T1's permission and T4's mark, in one value, and the mark half is a
/// witness rather than a flag.**
///
/// **`Yes` cannot be made without [`crate::restart::TheMarkWasSetBeforeTheCommandLeft`]**,
/// which only that module's own setter produces — so a caller that forgets the
/// mark cannot issue the restart at all. Until 2026-09-04 this was a plain
/// enumeration made out of a machine kind and a `true`, carrying no evidence the
/// mark was ever set.
#[derive(Debug)]
pub enum TheRestartMayBeIssued {
    Yes(crate::restart::TheMarkWasSetBeforeTheCommandLeft),
    No,
}

impl TheRestartMayBeIssued {
    /// **T1, read from the kind and the step rather than decided here — and
    /// T4's mark, which the caller must already have set.**
    pub fn on(
        kind: crate::kind::MachineKind,
        it_is_part_of_a_setup_step: bool,
        the_mark: crate::restart::TheMarkWasSetBeforeTheCommandLeft,
    ) -> TheRestartMayBeIssued {
        if crate::restart::a_restart_may_be_issued(kind, it_is_part_of_a_setup_step) {
            TheRestartMayBeIssued::Yes(the_mark)
        } else {
            TheRestartMayBeIssued::No
        }
    }

    /// Whether a restart may go out, read without consuming the witness.
    pub fn it_may(&self) -> bool {
        matches!(self, TheRestartMayBeIssued::Yes(_))
    }
}

// ------------------------------------------------------------- the search

/// **B10, performed: the whole-disk search and one asking-run per candidate.**
///
/// **The search is by name, crosses filesystems, and deduplicates by the
/// path-and-answer pair rather than by a resolved path** — all three of Fact 45's
/// traps, and all three are [`crate::search`]'s rules rather than this
/// function's.
///
/// **The refusals are counted from the complaint stream's own lines and no line
/// of it is read for a cause.** `Security and licence` §13 rule 5 forbids mining
/// a tool's output, and a count is not a mining: one line is one thing the
/// search could not read. **The narrowing is stated rather than glossed** — a
/// complaint that was not a permission refusal is counted too, which makes the
/// answer say *the machine could not be searched completely* where it might have
/// been searched completely. **That is the safe direction**, and B11's whole
/// point is that the count exists so the shortfall is said rather than hidden.
///
/// **Smoozy's own Blender area is not a candidate**, at the exact path B9
/// composes and never as a prefix — [`crate::search::the_candidates`] holds that
/// rule and says what was observed behind it. **The caller supplies the path
/// because this crate reads no machine**, and passes nothing on a machine it has
/// placed no root on.
pub async fn run_the_search(
    channel: &mut (impl CommandChannel + CarriesAValueOnACommandsStandardInput),
    elevated: Option<HowElevationIsObtained<'_>>,
    smoozys_own_blender_area: Option<&str>,
) -> TheSearchsOwnAnswer {
    // **The elevated retry goes through [`issue`] rather than round it**, so
    // B11's retry gets E4's validate-first order like every other elevated step
    // and there is one place that knows what a held password does.
    let (output, complaint, it_could_not_be_run) = match elevated {
        None => match channel.run(crate::readings::THE_SEARCH).await {
            Ok(result) => (result.output, result.complaint, None),
            Err(_) => (
                Vec::new(),
                Vec::new(),
                Some(WhyTheStepDidNotRun::TheRequestDidNotReachTheMachine),
            ),
        },
        Some(how) => match issue(channel, crate::elevated::THE_ELEVATED_SEARCH, how).await {
            Ok(said) => (
                said.the_standard_output,
                said.the_machines_own_words.into_bytes(),
                None,
            ),
            Err(why) => (Vec::new(), Vec::new(), Some(why)),
        },
    };
    TheSearchsOwnAnswer {
        found: crate::search::the_candidates(
            String::from_utf8_lossy(&output)
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::to_string)
                .collect(),
            smoozys_own_blender_area,
        ),
        folders_refused: String::from_utf8_lossy(&complaint)
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count(),
        it_could_not_be_run,
    }
}

/// What the search itself returned, before any build has been asked anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheSearchsOwnAnswer {
    /// The paths it found, as it found them. **Never resolved** (B10's second
    /// trap).
    pub found: Vec<String>,
    /// **B11's count.**
    pub folders_refused: usize,
    /// Where it could not be run at all.
    pub it_could_not_be_run: Option<WhyTheStepDidNotRun>,
}

/// **B1's asking-run, through the planted-link carriage.**
///
/// A build's path is machine-origin, so it cannot be written into a command
/// string in either phase; it rides the link and the command names the token.
///
/// **Both streams are captured, always** — the refusal to open a window goes to
/// the error stream in one version and the ordinary output in the other three,
/// and *"an empty error stream is never read as a property of having no
/// window"*. The version is read from the `Blender ` line and never from the
/// first line.
///
/// **A run that answered nothing is recorded as answering nothing**, which is
/// what B11 names as *a candidate that did not answer*.
///
/// **B1's time limit is carried here and it is the one run this module makes.**
/// [`crate::blender::THE_TIME_LIMIT_EVERY_BLENDER_RUN_CARRIES`] bounds the wait;
/// without it B12's search — run when `Render Settings` asks for the version list
/// — could stop the desktop with no ending and no message on a machine billed by
/// the hour, which is a hang rather than a failure. **B1's kill half is unkept
/// and is recorded at its own site**, at
/// [`crate::blender::B1S_KILL_HALF_IS_UNKEPT_AND_WHAT_BREAKS_IS_A_BLENDER_LEFT_RUNNING`].
///
/// **`on_the_server` is the local server identifier of the machine asked, and
/// no two asks on one server are at the site at once, whoever calls** — see
/// [`take_the_turn_at`]. Added 2026-09-30, step 10c's eleventh-pass fix piece
/// 4c. **Two callers reach this site** — `Render Settings`' version list
/// (B12's *when `Render Settings` asks for the version list*) and the Set up
/// tab's Blender table (B12's *when Server Setup's own view is opened*) — each
/// with its own guard against running twice and neither against the other.
/// Watched at the desk against OpenSSH's own `sftp-server`, a search begun half
/// a round trip after another recorded the 4.2 build as answering 4.5.1 and the
/// 4.5 build as not answering: **B1's pairing of a path with its answered
/// version, the entry B10 deduplicates by and
/// `Render Settings` B1 offers, would carry another build's version**, and a job
/// could render on a different Blender than the one the user picked.
pub async fn ask_one_build_its_version(
    channel: &mut (impl CommandChannel + CarriesAValueOnACommandsStandardInput),
    files: &mut impl TheMachinesFileSurface,
    on_the_server: &str,
    path: &str,
) -> ACandidate {
    // **Held from the plant to the removal**, and let go however the ask ends.
    // **The time limit below is inside it**, so a build that never answers
    // holds the next ask on this server for the limit and no longer.
    let _held = take_the_turn_at(
        crate::readings::THE_VERSION_ASKS_CARRIAGE_SITE,
        on_the_server,
    )
    .await;
    if plant_the_carriage(
        files,
        crate::readings::THE_VERSION_ASKS_CARRIAGE_SITE,
        path,
    )
    .await
    .is_err()
    {
        return ACandidate {
            path: path.to_string(),
            answered: None,
        };
    }
    let asked = tokio::time::timeout(
        crate::blender::THE_TIME_LIMIT_EVERY_BLENDER_RUN_CARRIES,
        channel.run(crate::readings::ASK_THE_BUILD_ITS_VERSION),
    )
    .await;
    let answered = match asked {
        Ok(Ok(result)) => {
            let both = format!(
                "{}\n{}",
                String::from_utf8_lossy(&result.output),
                String::from_utf8_lossy(&result.complaint)
            );
            crate::blender::read_the_version_answer(&both)
        }
        // **A run that hit the limit answers nothing**, which is what B11 names
        // as a candidate that did not answer — the same value a run that
        // answered nothing for any other reason produces.
        Ok(Err(_)) | Err(_) => None,
    };
    let _ = files
        .remove_one_file(crate::readings::THE_VERSION_ASKS_CARRIAGE_SITE.link_path_on_the_subsystem)
        .await;
    ACandidate {
        path: path.to_string(),
        answered,
    }
}

// ------------------------------------------------------------- the removal

/// **B15's folder-deletion route, performed bottom-up.**
///
/// *"The removal itself is the plain deletion of the install's folder"*, and its
/// paths are file-transfer-subsystem protocol fields and need no command at all.
/// **The version-numbered settings folder is left behind in every case** — it is
/// shared by every build of that version and holds add-ons the user may have
/// placed, and a build removed leaving it behind is Blender's own documented
/// behaviour. **Nothing here goes near it.**
pub async fn delete_the_install_folder(
    files: &mut impl TheMachinesFileSurface,
    folder: &str,
) -> HowTheActWent {
    match remove_a_tree(files, folder).await {
        Ok(()) => HowTheActWent::ItWasCarriedOut,
        Err(said) => HowTheActWent::ThePathWouldNotTake(said),
    }
}

async fn remove_a_tree(
    files: &mut impl TheMachinesFileSurface,
    folder: &str,
) -> Result<(), WhatTheMachineSaid> {
    // **Bottom-up**, so every folder is empty when it is removed. The subsystem
    // has no recursive removal and inventing one in a command string would put a
    // machine-origin path there.
    let mut stack = vec![folder.to_string()];
    let mut in_order: Vec<String> = Vec::new();
    while let Some(at) = stack.pop() {
        let held = files.list_one_directory(&at).await?;
        for thing in held {
            let name = String::from_utf8_lossy(&thing.name).into_owned();
            if name == "." || name == ".." {
                continue;
            }
            let below = format!("{at}/{name}");
            if thing.it_is_a_directory {
                stack.push(below);
            } else {
                files.remove_one_file(&below).await?;
            }
        }
        in_order.push(at);
    }
    for folder in in_order.iter().rev() {
        files.remove_one_empty_directory(folder).await?;
    }
    Ok(())
}

/// **B15's manager route, and §13 item 21's bound kept by construction.**
///
/// **The manager, its package name and its own removal command are one pairing**
/// ([`crate::search::THE_MANAGERS_SMOOZY_MAY_REMOVE_FROM`]), so the command
/// cannot come apart from the name it is for. Until 2026-09-04 the three were
/// apart: the set was one bare name, the command was a separate constant naming
/// one manager, and nothing read which manager put a found build on the machine
/// — so an apt-installed `/usr/bin/blender`, which Fact 45 records as a real
/// case, passed the set and got the snap command.
///
/// The package name is a constant fixed at build time — the store's own, which
/// B2 already reads the store by — because `Server protocol and transfers` 1.1
/// forbids echoing a machine-read name into a command string in either phase.
/// **A managed build whose manager is not established is reported as one this
/// module cannot remove**, and the user's own manager is the route.
pub async fn remove_the_managed_build(
    channel: &mut (impl CommandChannel + CarriesAValueOnACommandsStandardInput),
    package_name: &str,
    how: HowElevationIsObtained<'_>,
) -> HowTheManagerRouteWent {
    let Some(pairing) = crate::search::the_manager_route_for(package_name) else {
        return HowTheManagerRouteWent::ThisModuleCannotRemoveIt;
    };
    HowTheManagerRouteWent::ItWasAttempted(HowTheActWent::from(
        issue(channel, pairing.removal, how).await,
    ))
}

/// What the manager route came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowTheManagerRouteWent {
    /// **§13 item 21**: the build is under a name outside the fixed set and has
    /// no admissible removal carriage.
    ThisModuleCannotRemoveIt,
    /// The removal was issued, with what the machine said.
    ItWasAttempted(HowTheActWent),
}

// ------------------------------------------------- the Telegram material

/// **G1's material, placed inside the authenticated connection.**
///
/// **The token file is placed per `Security and licence` §10** — inside the
/// authenticated connection as part of this placement, **never a command
/// argument**, in the package state area, outside every job folder, never under
/// a shared temporary area. **Every path here is a protocol field**, which is
/// what keeps that rule by construction rather than by care.
///
/// **What is placed is `Telegram` §20's list and this document does not decide
/// it**; what this function does is put the bytes where the caller says, in one
/// act, and report what the machine said about each.
pub async fn place_the_telegram_material(
    files: &mut impl TheMachinesFileSurface,
    material: &[(String, Vec<u8>)],
) -> Vec<(String, HowTheActWent)> {
    let went = match place_the_complete_telegram_set(files, material).await {
        Ok(()) => HowTheActWent::ItWasCarriedOut,
        Err(said) => HowTheActWent::ThePathWouldNotTake(said),
    };
    // Acknowledge a complete set, never one file of a failed placement (`Telegram` L1).
    material
        .iter()
        .map(|(path, _)| (path.clone(), went.clone()))
        .collect()
}

/// Keep the public filenames while switching both through one shared link.
/// Legacy files first become links to a verified copy of the same old bytes.
/// A cut anywhere before the last rename therefore leaves the old set usable.
async fn place_the_complete_telegram_set(
    files: &mut impl TheMachinesFileSurface,
    material: &[(String, Vec<u8>)],
) -> Result<(), WhatTheMachineSaid> {
    use smoozy_state::identities::{Identity, IdentityKind};
    let Some((first, _)) = material.first() else {
        return Err(WhatTheMachineSaid::TheReplacementCouldNotBeVerified);
    };
    let Some((area, _)) = first.rsplit_once('/') else {
        return Err(WhatTheMachineSaid::TheReplacementCouldNotBeVerified);
    };
    let names: Vec<&str> = material
        .iter()
        .map(|(path, _)| path.rsplit_once('/'))
        .map(|part| {
            part.filter(|(parent, name)| *parent == area && !name.is_empty())
                .map(|(_, name)| name)
        })
        .collect::<Option<_>>()
        .ok_or(WhatTheMachineSaid::TheReplacementCouldNotBeVerified)?;
    if names
        .iter()
        .enumerate()
        .any(|(at, name)| names[..at].contains(name))
    {
        return Err(WhatTheMachineSaid::TheReplacementCouldNotBeVerified);
    }
    // Both placement entry points share this turn, including every file read.
    let site = CarriageSite {
        link_path_on_the_subsystem: ".smoozy-carriage/telegram-set",
        named_in_a_command_as: "~/.smoozy-carriage/telegram-set",
    };
    let _turn = take_the_turn_at(site, area).await;
    let id = Identity::issue(IdentityKind::Operation)
        .map_err(|_| WhatTheMachineSaid::TheReplacementCouldNotBeVerified)?
        .as_text();
    let new = format!("{area}/.telegram-set-{id}");
    files.make_one_directory(&new).await?;
    for ((_, bytes), name) in material.iter().zip(&names) {
        write_and_check(files, &format!("{new}/{name}"), bytes).await?;
    }
    let current = format!("{area}/.telegram-current");
    let has_current = match files.read_link(&current).await {
        Ok(target) if target.starts_with(format!("{area}/.telegram-set-").as_bytes()) => true,
        Err(WhatTheMachineSaid::ThePathIsNotThere) => false,
        Err(said) => return Err(said),
        Ok(_) => return Err(WhatTheMachineSaid::TheReplacementCouldNotBeVerified),
    };
    let switch = format!("{new}/switch");
    if !has_current {
        let mut old = Vec::new();
        for (path, _) in material {
            match files.read_whole(path).await {
                Ok(bytes) => old.push(Some(bytes)),
                Err(WhatTheMachineSaid::ThePathIsNotThere) => old.push(None),
                Err(said) => return Err(said),
            }
        }
        if old.iter().any(Option::is_some) && old.iter().any(Option::is_none) {
            return Err(WhatTheMachineSaid::TheReplacementCouldNotBeVerified);
        }
        let target = if old.iter().all(Option::is_some) {
            let previous = format!("{area}/.telegram-set-{id}-previous");
            files.make_one_directory(&previous).await?;
            for (bytes, name) in old.iter_mut().zip(&names) {
                let bytes = bytes.as_mut().expect("the whole old set");
                let wrote = write_and_check(files, &format!("{previous}/{name}"), bytes).await;
                bytes.fill(0);
                wrote?;
            }
            previous
        } else {
            new.clone()
        };
        files.plant_a_link(&target, &switch).await?;
        files.replace_by_rename(&switch, &current).await?;
    }
    // Installing each alias preserves its old bytes, even on a legacy server.
    for ((path, _), name) in material.iter().zip(&names) {
        let alias = format!("{new}/alias-{name}");
        files
            .plant_a_link(&format!("{current}/{name}"), &alias)
            .await?;
        files.replace_by_rename(&alias, path).await?;
    }
    files.plant_a_link(&new, &switch).await?;
    files.replace_by_rename(&switch, &current).await?;
    Ok(())
}

async fn write_and_check(
    files: &mut impl TheMachinesFileSurface,
    path: &str,
    bytes: &[u8],
) -> Result<(), WhatTheMachineSaid> {
    files.write_whole(path, bytes).await?;
    let mut read = files.read_whole(path).await?;
    let equal = read == bytes;
    read.fill(0);
    if equal {
        Ok(())
    } else {
        Err(WhatTheMachineSaid::TheReplacementCouldNotBeVerified)
    }
}

/// **G4: replacement deliveries reuse the same placement vehicle**
/// (`Telegram` L4). **There is no second route and none is built.**
pub fn a_replacement_uses_a_second_vehicle(_it_is_a_replacement: bool) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::readings::tests::{block_on, complained, said, AMachine};
    use smoozy_job_files::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};

    fn a_layout() -> TheLayout {
        TheLayout::of(
            MachineRoot::chosen("/mnt/one/smoozy").expect("a root"),
            WhereThePackageAreaSits::UnderTheRoot,
        )
    }

    #[test]
    fn the_hand_over_plants_names_the_token_and_proves_the_landing_by_a_real_write() {
        let mut channel = AMachine::answering(vec![(
            crate::elevated::THE_HAND_OVER_OF_THE_ROOT.elevated.as_str(),
            said("", 0),
        )]);
        // **The mount is there and Smoozy's own root is not**, which is what a
        // machine that has never been prepared looks like.
        let mut files = AMachine::default().with_directory("/mnt/one");
        let placement = block_on(carry_out_the_placement(
            &mut channel,
            &mut files,
            &a_layout(),
            HowElevationIsObtained::Passwordless,
        ));
        assert_eq!(placement.hand_overs.len(), 1, "one mount, one hand-over");
        // **R7: the root's own directory is made before anything lands in it.**
        assert!(placement.hand_overs[0].the_directory_was_created.it_was_carried_out());
        assert!(placement.hand_overs[0].the_ownership_landed());
        assert!(placement.it_is_complete());
        // **The real write of at least one byte at the exact target path.**
        assert!(files
            .written
            .iter()
            .any(|(path, bytes)| path == "/mnt/one/smoozy/.smoozy-landing-check"
                && bytes == THE_LANDING_CHECKS_BYTE));
        // **The command named the token and never the path.**
        assert!(channel
            .asked
            .iter()
            .all(|command| !command.contains("/mnt/one")));
    }

    #[test]
    fn a_machine_that_has_never_been_prepared_gets_its_root_made_before_anything_lands_in_it() {
        // **R7, and the fault this test exists for.** Until 2026-09-04 nothing
        // in the application created Smoozy's own root, so the confirming write
        // had nowhere to land and the placement could never finish on a machine
        // with nothing on it. Take the creation out and this fails at the
        // landing check.
        let mut channel = AMachine::answering(vec![(
            crate::elevated::THE_HAND_OVER_OF_THE_ROOT.elevated.as_str(),
            said("", 0),
        )]);
        let mut files = AMachine::default().with_directory("/mnt/one");
        assert!(!files.directories.iter().any(|held| held == "/mnt/one/smoozy"));
        let placement = block_on(carry_out_the_placement(
            &mut channel,
            &mut files,
            &a_layout(),
            HowElevationIsObtained::Passwordless,
        ));
        assert!(placement.it_is_complete());
        // The root, and then the five areas inside it.
        assert!(files.directories.iter().any(|held| held == "/mnt/one/smoozy"));
        assert_eq!(placement.areas.len(), 5);
        assert!(placement.areas.iter().all(|(_, went)| went.it_was_carried_out()));
    }

    #[test]
    fn a_machine_that_has_never_been_prepared_gets_the_carriage_directory_made_before_a_token_is_written_into_it(
    ) {
        // **`Server protocol and transfers` §1.1, and the fault this test exists
        // for.** Every carriage token of this module sits under a Smoozy-named
        // directory in the login's own writable area, and a fresh machine has
        // none. **Nothing in this crate made it until 2026-09-04**: the function
        // was written, it took the real end rather than the seam, and no caller
        // anywhere reached it — while four tests hid that by handing the
        // stand-in a machine that already had the directory. All three routes
        // below are exercised on a machine with nothing on it.
        let held = smoozy_server_protocol::carriage::THE_DIRECTORY_ON_THE_SUBSYSTEM;

        // **B1's version ask**, whose token carries the build's own path.
        let mut channel = AMachine::answering(vec![(
            crate::readings::ASK_THE_BUILD_ITS_VERSION.as_str(),
            said("Blender 4.5.12 LTS\n", 0),
        )]);
        let mut files = AMachine::default();
        let candidate = block_on(ask_one_build_its_version(
            &mut channel,
            &mut files,
            "4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c01",
            "/opt/blender-4.5/blender",
        ));
        assert!(candidate.answered.is_some());
        assert!(files.directories.iter().any(|made| made == held));

        // **M1b's staging token**, which is a write into that directory rather
        // than a link in it, and is refused the same way where nobody made it.
        let mut channel = AMachine::answering(vec![
            (crate::elevated::THE_COMPONENTS_WRITE.elevated.as_str(), said("", 0)),
            (crate::elevated::THE_LIST_UPDATE.elevated.as_str(), said("", 0)),
        ]);
        let mut files = AMachine::default().holding(THE_DEBIAN_SOURCES_FILE, "Components: main\n");
        assert_eq!(
            block_on(enable_the_components(
                &mut channel,
                &mut files,
                HowElevationIsObtained::Passwordless
            )),
            HowTheComponentsStepWent::ItWasCarriedOut
        );
        assert!(files.directories.iter().any(|made| made == held));

        // **R7's elevated create, through R7a's carriage**, on a machine whose
        // login may not make the directory on the mount.
        let mut channel = AMachine::answering(vec![(
            crate::elevated::THE_CREATION_OF_THE_ROOT.elevated.as_str(),
            said("", 0),
        )]);
        let mut files =
            AMountTheLoginMayNotMakeADirectoryOn(AMachine::default().with_directory("/mnt/one"));
        let owed = the_hand_overs(&a_layout());
        let made = block_on(create_the_directory(
            &mut channel,
            &mut files,
            &owed[0],
            crate::elevated::THE_CREATION_OF_THE_ROOT,
            HowElevationIsObtained::Passwordless,
        ));
        assert!(made.it_was_carried_out());
        assert!(files.0.directories.iter().any(|held_here| held_here == held));
    }

    /// A machine whose login may not make a directory on the mount at all —
    /// the case R7's elevated create exists for.
    struct AMountTheLoginMayNotMakeADirectoryOn(AMachine);

    impl CommandChannel for AMountTheLoginMayNotMakeADirectoryOn {
        async fn run(
            &mut self,
            command: smoozy_server_protocol::transport::CommandString,
        ) -> Result<
            smoozy_server_protocol::transport::CommandResult,
            smoozy_server_protocol::transport::ssh::CommandFailure,
        > {
            self.0.run(command).await
        }
    }

    impl TheMachinesFileSurface for AMountTheLoginMayNotMakeADirectoryOn {
        async fn read_whole(&mut self, path: &str) -> Result<Vec<u8>, WhatTheMachineSaid> {
            self.0.read_whole(path).await
        }
        async fn write_whole(&mut self, path: &str, bytes: &[u8]) -> Result<(), WhatTheMachineSaid> {
            self.0.write_whole(path, bytes).await
        }
        /// **It is the mount the login may not make a directory on, and only
        /// the mount.** A relative path is the login's own starting area, which
        /// is where `Server protocol and transfers` §1.1 puts the carriage and
        /// never on the mount the link points at — so refusing that too would
        /// be a machine that refuses the login its own home, which is not the
        /// case R7's elevated route exists for.
        async fn make_one_directory(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
            if path.starts_with('/') {
                return Err(WhatTheMachineSaid::TheLoginMayNot);
            }
            self.0.make_one_directory(path).await
        }
        async fn remove_one_file(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
            self.0.remove_one_file(path).await
        }
        async fn remove_one_empty_directory(
            &mut self,
            path: &str,
        ) -> Result<(), WhatTheMachineSaid> {
            self.0.remove_one_empty_directory(path).await
        }
        async fn list_one_directory(
            &mut self,
            path: &str,
        ) -> Result<Vec<crate::readings::AListedThing>, WhatTheMachineSaid> {
            self.0.list_one_directory(path).await
        }
        async fn plant_a_link(&mut self, target: &str, at: &str) -> Result<(), WhatTheMachineSaid> {
            self.0.plant_a_link(target, at).await
        }
    }

    #[test]
    fn a_root_the_plain_create_is_refused_is_made_by_the_elevated_command_through_the_link() {
        // **R7: tried unelevated first, and where the plain create is refused it
        // is made by an elevated constant command through R7a's carriage** — the
        // link's target is the chosen mount and the command names the token and
        // `Job files and outputs`' own root segment.
        let mut channel = AMachine::answering(vec![(
            crate::elevated::THE_CREATION_OF_THE_ROOT.elevated.as_str(),
            said("", 0),
        )]);
        let mut files =
            AMountTheLoginMayNotMakeADirectoryOn(AMachine::default().with_directory("/mnt/one"));
        let owed = the_hand_overs(&a_layout());
        let made = block_on(create_the_directory(
            &mut channel,
            &mut files,
            &owed[0],
            crate::elevated::THE_CREATION_OF_THE_ROOT,
            HowElevationIsObtained::Passwordless,
        ));
        assert!(made.it_was_carried_out());
        // **The link's target is the chosen mount and never the root**, and no
        // chosen path is in the command.
        assert_eq!(owed[0].the_location_it_sits_in.as_deref(), Some("/mnt/one"));
        assert!(channel
            .asked
            .iter()
            .all(|command| !command.contains("/mnt/one")));
        assert!(channel.asked.iter().any(|command| command
            .contains(crate::placement::THE_ROOTS_CREATION_SITE.named_in_a_command_as)));
    }

    #[test]
    fn a_hand_over_whose_command_passed_but_whose_write_did_not_has_not_landed() {
        // **R7a's whole reason**: a recursive hand-over through a link without
        // the traverse option changes nothing on the mount and nothing in its
        // exit status says so. So the landing check has to run regardless, and
        // it is what decides.
        struct AMountThatWillNotTakeAWrite(AMachine);
        impl CommandChannel for AMountThatWillNotTakeAWrite {
            async fn run(
                &mut self,
                command: smoozy_server_protocol::transport::CommandString,
            ) -> Result<
                smoozy_server_protocol::transport::CommandResult,
                smoozy_server_protocol::transport::ssh::CommandFailure,
            > {
                self.0.run(command).await
            }
        }
        impl TheMachinesFileSurface for AMountThatWillNotTakeAWrite {
            async fn read_whole(&mut self, path: &str) -> Result<Vec<u8>, WhatTheMachineSaid> {
                self.0.read_whole(path).await
            }
            async fn write_whole(
                &mut self,
                _path: &str,
                _bytes: &[u8],
            ) -> Result<(), WhatTheMachineSaid> {
                Err(WhatTheMachineSaid::TheLoginMayNot)
            }
            async fn make_one_directory(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
                self.0.make_one_directory(path).await
            }
            async fn remove_one_file(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
                self.0.remove_one_file(path).await
            }
            async fn remove_one_empty_directory(
                &mut self,
                path: &str,
            ) -> Result<(), WhatTheMachineSaid> {
                self.0.remove_one_empty_directory(path).await
            }
            async fn list_one_directory(
                &mut self,
                path: &str,
            ) -> Result<Vec<crate::readings::AListedThing>, WhatTheMachineSaid> {
                self.0.list_one_directory(path).await
            }
            async fn plant_a_link(
                &mut self,
                target: &str,
                at: &str,
            ) -> Result<(), WhatTheMachineSaid> {
                self.0.plant_a_link(target, at).await
            }
        }

        let mut machine = AMountThatWillNotTakeAWrite(AMachine::answering(vec![(
            crate::elevated::THE_HAND_OVER_OF_THE_ROOT.elevated.as_str(),
            said("", 0),
        )]));
        let owed = the_hand_overs(&a_layout());
        let mut second = AMountThatWillNotTakeAWrite(AMachine::default());
        let went = block_on(async {
            let mut channel = AMachine::answering(vec![(
                crate::elevated::THE_HAND_OVER_OF_THE_ROOT.elevated.as_str(),
                said("", 0),
            )]);
            hand_one_mount_over(
                &mut channel,
                &mut second,
                &owed[0],
                crate::elevated::THE_HAND_OVER_OF_THE_ROOT,
                crate::elevated::THE_CREATION_OF_THE_ROOT,
                HowElevationIsObtained::Passwordless,
            )
            .await
        });
        assert!(went.the_command.it_was_carried_out());
        assert!(!went.the_ownership_landed());
        let _ = &mut machine;
    }

    #[test]
    fn a_hand_over_whose_elevation_is_owed_sends_nothing_and_says_so() {
        // **E4's fourth bullet**: a password is owed and none is held, which is
        // the moment the offer is made. Nothing reaches the machine at all and
        // the hand-over records why rather than reporting a refusal it never
        // got. **E6 is untouched by the route having landed**: no second route
        // is opened here either.
        let mut channel = AMachine::default();
        let mut files = AMachine::default();
        let placement = block_on(carry_out_the_placement(
            &mut channel,
            &mut files,
            &a_layout(),
            HowElevationIsObtained::APasswordIsOwedAndNoneIsHeld,
        ));
        assert!(channel.asked.is_empty());
        assert!(channel.carried_on_standard_input.is_empty());
        assert_eq!(
            placement.hand_overs[0].the_command,
            HowTheActWent::ItCouldNotBeAttempted(WhyTheStepDidNotRun::NoPasswordIsHeldYet)
        );
        // **The landing check still decides**, which is R7a's rule: a mount
        // already owned by an earlier run is handed over by a step with nothing
        // to do, and the write at the exact target path is the only thing that
        // says which case a machine is in.
    }


    /// A machine of the kind and shape the driver step is composed for, with
    /// **a driver already on it** — R1 item 5's *restart before any install*
    /// case.
    fn a_machine_with_a_card_and_no_working_driver(
        family: crate::kind::DistributionFamily,
    ) -> crate::ready_action::WhatTheMachineNeeds {
        crate::ready_action::WhatTheMachineNeeds {
            kind: crate::kind::MachineKind::ASystemdMachine,
            family,
            this_run_fetches_a_blender: false,
            the_machine_already_holds_a_blender: false,
            the_unpack_tool_is_present: true,
            the_driver_answers: false,
            a_driver_is_already_installed: true,
            cards: smoozy_servers::hardware::Cards::PresentAndUnreadable(
                smoozy_servers::hardware::WhyUnreadable::TheDriverIsAbsent,
            ),
            telegram_is_on: false,
            the_drivers_version: None,
            the_optix_bar: None,
            anything_is_live: false,
        }
    }

    /// The same machine with **no driver on it at all**, which is R1 item 5's
    /// other branch.
    fn a_machine_with_a_card_and_no_driver_at_all(
        family: crate::kind::DistributionFamily,
    ) -> crate::ready_action::WhatTheMachineNeeds {
        crate::ready_action::WhatTheMachineNeeds {
            a_driver_is_already_installed: false,
            ..a_machine_with_a_card_and_no_working_driver(family)
        }
    }

    /// The check, as the caller takes it before this step runs.
    fn the_check_answered() -> crate::readings::WhatTheDriverCheckSaid {
        crate::readings::WhatTheDriverCheckSaid::ItAnswered
    }

    /// The check having failed, at whichever exit the machine ended at.
    fn the_check_failed_at(exit: u32) -> crate::readings::WhatTheDriverCheckSaid {
        crate::readings::WhatTheDriverCheckSaid::ItDidNot {
            the_exit: Some(exit),
            the_machines_own_words: "NVIDIA-SMI has failed".to_string(),
            what_it_wrote_to_standard_output: String::new(),
        }
    }

    /// T4's mark, set before the command leaves on a standing connected
    /// period, as every caller of a restart must set it.
    fn the_mark_is_set() -> TheRestartMayBeIssued {
        on_a_marked_period(crate::kind::MachineKind::ASystemdMachine, true)
    }

    /// The same, for a kind or a moment T1 refuses — **the mark is still set on
    /// a real period**, because the witness is evidence rather than a token.
    fn on_a_marked_period(
        kind: crate::kind::MachineKind,
        it_is_part_of_a_setup_step: bool,
    ) -> TheRestartMayBeIssued {
        let period = smoozy_servers::restart::TheRestartMark::none_is_set();
        TheRestartMayBeIssued::on(
            kind,
            it_is_part_of_a_setup_step,
            crate::restart::mark_the_connected_period_restarting(&period),
        )
    }

    #[test]
    fn a_driver_that_answers_is_never_touched() {
        // **R5's own prohibition, kept by R1 item 5's order rather than by a
        // guard**: a driver that answers is never reinstalled, so the
        // reinstall-becomes-upgrade harm is unreachable from a re-run.
        let mut channel = AMachine::default();
        let mut files = AMachine::default();
        let needs = crate::ready_action::WhatTheMachineNeeds {
            the_driver_answers: true,
            ..a_machine_with_a_card_and_no_working_driver(crate::kind::DistributionFamily::Ubuntu)
        };
        let went = block_on(carry_out_the_driver_step(
            &mut channel,
            &mut files,
            &needs,
            the_check_answered(),
            HowElevationIsObtained::Passwordless,
        ));
        assert!(went.the_install.is_none());
        assert!(went.the_driver_answers());
        assert_eq!(
            went.which_branch,
            crate::ready_action::WhatTheDriverStepDoes::NothingTheCheckAnswered
        );
        assert_eq!(went.what_the_step_did(), crate::run::WhatTheStepDid::NothingToDo);
        // **Nothing at all was asked of the machine** — the check is the
        // caller's and this step touched nothing after it.
        assert!(channel.asked.is_empty());
    }

    // ---- R1 item 5's swap up to the OptiX bar, added 2026-10-11 ----

    /// `Servers` H11b's version query, as the machine is asked it.
    const THE_VERSION_QUERY: &str = "nvidia-smi --query-gpu=driver_version --format=csv,noheader";

    /// **A Hyperstack R550 machine as the OptiX check met it**: Ubuntu, the
    /// systemd kind, its driver answering at 550.163.01, and this run
    /// installing Blender 5.2.2, whose bar is 575.
    fn a_550_machine_installing(blender: &str) -> crate::ready_action::WhatTheMachineNeeds {
        crate::ready_action::WhatTheMachineNeeds {
            the_driver_answers: true,
            a_driver_is_already_installed: false,
            this_run_fetches_a_blender: true,
            the_drivers_version: smoozy_servers::hardware::DriverVersion::parse("550.163.01"),
            the_optix_bar: crate::ready_action::the_bar_for_the_blender_this_run_installs(Some(blender)),
            ..a_machine_with_a_card_and_no_working_driver(crate::kind::DistributionFamily::Ubuntu)
        }
    }

    fn the_swap_on(
        needs: &crate::ready_action::WhatTheMachineNeeds,
        answers: Vec<(&'static str, smoozy_server_protocol::transport::CommandResult)>,
    ) -> (HowTheDriverStepWent, Vec<&'static str>) {
        let mut channel = AMachine::answering(answers);
        let mut files = AMachine::default();
        let went = block_on(carry_out_the_driver_step(
            &mut channel,
            &mut files,
            needs,
            the_check_answered(),
            HowElevationIsObtained::Passwordless,
        ));
        (went, channel.asked)
    }

    /// **550.163.01 before, 580.178.04 after: the swap goes out, the check
    /// runs again, and the version is read for the bar.** Where the check
    /// after answers at the bar no restart is owed; where it does not answer
    /// yet — the old module still loaded — the restart is owed and nothing
    /// went wrong.
    #[test]
    fn a_driver_below_the_bar_is_swapped_and_checked_again() {
        let needs = a_550_machine_installing("5.2.2");
        let swap = crate::elevated::THE_UBUNTU_DRIVER_SWAP.elevated.as_str();
        let (went, asked) = the_swap_on(
            &needs,
            vec![
                (swap, said("", 0)),
                ("nvidia-smi", said("", 0)),
                (THE_VERSION_QUERY, said("580.178.04\n", 0)),
            ],
        );
        assert_eq!(
            went.which_branch,
            crate::ready_action::WhatTheDriverStepDoes::TheSwapUpToTheBar
        );
        assert_eq!(asked[0], swap, "the swap is the one act, and it goes first");
        assert!(went.the_install.is_none() && went.the_tool.is_none());
        assert_eq!(went.what_the_step_did(), crate::run::WhatTheStepDid::Performed);
        assert_eq!(went.what_went_wrong(), None);
        assert!(went.the_check_passes(&needs));
        assert!(!went.the_driver_missed_the_bar(&needs));

        // **The usual shape: the old module is still loaded, so the check
        // fails until the restart.** The restart is owed; nothing went wrong.
        let (went, _) = the_swap_on(
            &needs,
            vec![(swap, said("", 0)), ("nvidia-smi", complained("Driver/library version mismatch", 18))],
        );
        assert_eq!(went.what_went_wrong(), None);
        assert!(!went.the_check_passes(&needs));
    }

    /// **550 before and after.** A swap that went through and still reads
    /// below the bar owes the restart; a swap that was refused leaves the old
    /// driver answering, owes no restart, does not fail the step, and is
    /// recorded as having missed the bar.
    #[test]
    fn a_swap_that_misses_the_bar_carries_on_and_is_recorded() {
        let needs = a_550_machine_installing("5.2.2");
        let swap = crate::elevated::THE_UBUNTU_DRIVER_SWAP.elevated.as_str();
        let (went, _) = the_swap_on(
            &needs,
            vec![
                (swap, said("", 0)),
                ("nvidia-smi", said("", 0)),
                (THE_VERSION_QUERY, said("550.163.01\n", 0)),
            ],
        );
        assert!(!went.the_check_passes(&needs), "went through, still below: the restart");

        let (went, _) = the_swap_on(
            &needs,
            vec![
                (swap, complained("E: Unable to locate package nvidia-fabricmanager-580", 100)),
                ("nvidia-smi", said("", 0)),
                (THE_VERSION_QUERY, said("550.163.01\n", 0)),
            ],
        );
        assert_eq!(went.what_went_wrong(), None, "setup does not fail for it");
        assert!(went.the_check_passes(&needs), "nothing went on, so no restart");
        assert!(went.the_driver_missed_the_bar(&needs));

        // **And where the old driver went and the new one did not come**, the
        // card no longer answers: the item's existing failure, in the swap's
        // own words.
        let (went, _) = the_swap_on(
            &needs,
            vec![
                (swap, complained("E: Sub-process /usr/bin/dpkg returned an error code (1)", 100)),
                ("nvidia-smi", complained("NVIDIA-SMI has failed", 9)),
            ],
        );
        assert!(went
            .what_went_wrong()
            .is_some_and(|words| words.contains("dpkg returned an error")));
    }

    /// **Nothing is touched where nothing is below the bar**: a driver
    /// already at it, a run installing only 5.1 (535's row) on 550, and a run
    /// installing no Blender at all.
    #[test]
    fn a_driver_at_its_bar_is_never_touched() {
        let mut at = a_550_machine_installing("5.2.2");
        at.the_drivers_version = smoozy_servers::hardware::DriverVersion::parse("595.91.07");
        for needs in [
            at,
            a_550_machine_installing("5.1.2"),
            crate::ready_action::WhatTheMachineNeeds {
                the_optix_bar: None,
                this_run_fetches_a_blender: false,
                ..a_550_machine_installing("5.2.2")
            },
        ] {
            let (went, asked) = the_swap_on(&needs, Vec::new());
            assert!(asked.is_empty(), "{asked:?}");
            assert!(went.the_swap.is_none());
            assert!(went.the_check_passes(&needs));
            assert!(!went.the_driver_missed_the_bar(&needs));
        }
    }

    /// **A container below the bar is never touched** (R3): its driver is
    /// the host's, and `Servers` H11b's statement on the card tells the user.
    #[test]
    fn a_container_below_the_bar_is_never_touched() {
        let needs = crate::ready_action::WhatTheMachineNeeds {
            kind: crate::kind::MachineKind::AContainer,
            ..a_550_machine_installing("5.2.2")
        };
        let (went, asked) = the_swap_on(&needs, Vec::new());
        assert!(asked.is_empty());
        assert_eq!(went.which_branch, crate::ready_action::WhatTheDriverStepDoes::Nothing);
        assert!(!went.the_driver_missed_the_bar(&needs), "the setup does not record a container's");
    }

    /// **Debian is not swapped** (§13 item 28) and is recorded as missed.
    #[test]
    fn a_debian_driver_below_the_bar_is_recorded_and_not_swapped() {
        let needs = crate::ready_action::WhatTheMachineNeeds {
            family: crate::kind::DistributionFamily::Debian,
            ..a_550_machine_installing("5.2.2")
        };
        let (went, asked) = the_swap_on(&needs, Vec::new());
        assert!(asked.is_empty());
        assert!(went.the_check_passes(&needs));
        assert!(went.the_driver_missed_the_bar(&needs));
    }

    #[test]
    fn an_installed_driver_that_does_not_answer_gets_no_install_at_all() {
        // **R1 item 5, and the harm this test exists for.** The step took no
        // reading of whether a driver was already installed and went straight to
        // the install until 2026-09-04 — and *"a reinstall on a
        // working-but-unbooted install silently becomes an upgrade the running
        // kernel does not match"*. **The restart that follows is R1 item 6's**
        // and is issued by the caller after this step's boundary, which is what
        // Y4's *driver placed* row requires.
        let mut channel = AMachine::answering(vec![
            (crate::elevated::THE_RESTART.elevated.as_str(), said("", 0)),
            (
                crate::elevated::THE_UBUNTU_DRIVER_INSTALL.elevated.as_str(),
                said("", 0),
            ),
        ]);
        let mut files = AMachine::default();
        let went = block_on(carry_out_the_driver_step(
            &mut channel,
            &mut files,
            &a_machine_with_a_card_and_no_working_driver(crate::kind::DistributionFamily::Ubuntu),
            the_check_failed_at(
                crate::readings::THE_EXIT_THAT_MEANS_A_DRIVER_IS_INSTALLED_AND_NOT_LOADED,
            ),
            HowElevationIsObtained::Passwordless,
        ));
        assert_eq!(
            went.which_branch,
            crate::ready_action::WhatTheDriverStepDoes::TheRestartBeforeAnyInstall
        );
        assert!(went.the_install.is_none());
        assert!(went.the_check_after.is_none());
        assert!(!went.the_driver_answers(), "R1 item 6 is owed");
        assert_eq!(went.what_the_step_did(), crate::run::WhatTheStepDid::NothingToDo);
        // **Nothing elevated went out at all**, which is the whole of the harm.
        assert!(channel.asked.is_empty());
    }

    /// **R1 item 5 as corrected on 2026-09-21: on Ubuntu the detection tool is
    /// made reachable before it is called** — Hyperstack's plain 24.04 image
    /// had no `ubuntu-drivers` and the step stopped at *"sudo: ubuntu-drivers:
    /// command not found"*.
    #[test]
    fn the_ubuntu_route_installs_its_tool_before_it_calls_it() {
        let mut channel = AMachine::answering(vec![
            (crate::elevated::THE_LIST_UPDATE.elevated.as_str(), said("", 0)),
            (
                crate::elevated::THE_UBUNTU_DRIVER_TOOL_INSTALL.elevated.as_str(),
                said("", 0),
            ),
            (
                crate::elevated::THE_UBUNTU_DRIVER_INSTALL.elevated.as_str(),
                said("", 0),
            ),
        ]);
        let mut files = AMachine::default();
        let went = block_on(carry_out_the_driver_step(
            &mut channel,
            &mut files,
            &a_machine_with_a_card_and_no_driver_at_all(crate::kind::DistributionFamily::Ubuntu),
            the_check_failed_at(12),
            HowElevationIsObtained::Passwordless,
        ));
        let at = |command: &str| {
            channel
                .asked
                .iter()
                .position(|asked| *asked == command)
                .unwrap_or_else(|| panic!("{command} was not asked: {:?}", channel.asked))
        };
        // **S7d, added 2026-09-22: the lists are refreshed before the tool is
        // installed**, which the third pass's image, its lists a year old, had
        // needed.
        assert!(
            at(crate::elevated::THE_LIST_UPDATE.elevated.as_str())
                < at(crate::elevated::THE_UBUNTU_DRIVER_TOOL_INSTALL.elevated.as_str())
        );
        assert!(
            at(crate::elevated::THE_UBUNTU_DRIVER_TOOL_INSTALL.elevated.as_str())
                < at(crate::elevated::THE_UBUNTU_DRIVER_INSTALL.elevated.as_str())
        );
        assert_eq!(went.the_tool, Some(HowTheActWent::ItWasCarriedOut));
        assert!(went.the_install.is_some());
        assert_eq!(went.what_the_step_did(), crate::run::WhatTheStepDid::Performed);
    }

    /// **R1 item 5 as corrected on 2026-09-23: the driver install is followed
    /// by the driver support binaries, and the check is run only after
    /// them.**
    ///
    /// **This is the fifth pass's first fault.** On Hyperstack's plain 24.04
    /// the three acts above ran to their end, thirteen packages landed and the
    /// card answered the kernel — and `nvidia-smi`, which is the only thing
    /// this clause checks a driver with, was on the machine nowhere. **Every
    /// test of this route passed without it**, because the stand-in machine
    /// answers a check nobody had made depend on a package.
    #[test]
    fn the_ubuntu_route_installs_the_check_it_is_about_to_run() {
        let mut channel = AMachine::answering(vec![
            (crate::elevated::THE_LIST_UPDATE.elevated.as_str(), said("", 0)),
            (
                crate::elevated::THE_UBUNTU_DRIVER_TOOL_INSTALL.elevated.as_str(),
                said("", 0),
            ),
            (
                crate::elevated::THE_UBUNTU_DRIVER_INSTALL.elevated.as_str(),
                said("", 0),
            ),
            (
                crate::elevated::THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL
                    .elevated
                    .as_str(),
                said("Setting up nvidia-utils-595-server", 0),
            ),
            (
                crate::elevated::THE_UBUNTU_OPTIX_LIBRARY_INSTALL.elevated.as_str(),
                said("Setting up libnvidia-gl-595-server", 0),
            ),
            (crate::readings::THE_DRIVER_CHECK.as_str(), said("a card", 0)),
        ]);
        let mut files = AMachine::default();
        let went = block_on(carry_out_the_driver_step(
            &mut channel,
            &mut files,
            &a_machine_with_a_card_and_no_driver_at_all(crate::kind::DistributionFamily::Ubuntu),
            the_check_failed_at(127),
            HowElevationIsObtained::Passwordless,
        ));
        let at = |command: &str| {
            channel
                .asked
                .iter()
                .position(|asked| *asked == command)
                .unwrap_or_else(|| panic!("{command} was not asked: {:?}", channel.asked))
        };
        // **The publication's own order**: the driver, then the additional
        // components, then the check.
        assert!(
            at(crate::elevated::THE_UBUNTU_DRIVER_INSTALL.elevated.as_str())
                < at(crate::elevated::THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL
                    .elevated
                    .as_str())
        );
        // **R1 item 5's fifth act, from 2026-10-03: the OptiX library, after
        // the support binaries and before the check.**
        assert!(
            at(crate::elevated::THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL
                .elevated
                .as_str())
                < at(crate::elevated::THE_UBUNTU_OPTIX_LIBRARY_INSTALL.elevated.as_str())
        );
        assert!(
            at(crate::elevated::THE_UBUNTU_OPTIX_LIBRARY_INSTALL.elevated.as_str())
                < at(crate::readings::THE_DRIVER_CHECK.as_str())
        );
        assert_eq!(
            went.the_support_binaries,
            Some(HowTheActWent::ItWasCarriedOut)
        );
        assert_eq!(went.the_optix_library, Some(HowTheActWent::ItWasCarriedOut));
        assert!(went.the_driver_answers());
        assert!(went.what_went_wrong().is_none());
    }

    /// **R1 item 5's fifth act refused ends the step in apt's own words, and
    /// no check and no restart follow** — the rule every act of this route
    /// keeps. Added 2026-10-03, with `Rendering` L23's ruling that NVIDIA cards
    /// render with OptiX.
    #[test]
    fn an_ubuntu_route_whose_optix_library_will_not_install_ends_there() {
        let mut channel = AMachine::answering(vec![
            (crate::elevated::THE_LIST_UPDATE.elevated.as_str(), said("", 0)),
            (
                crate::elevated::THE_UBUNTU_DRIVER_TOOL_INSTALL.elevated.as_str(),
                said("", 0),
            ),
            (
                crate::elevated::THE_UBUNTU_DRIVER_INSTALL.elevated.as_str(),
                said("", 0),
            ),
            (
                crate::elevated::THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL
                    .elevated
                    .as_str(),
                said("", 0),
            ),
            (
                crate::elevated::THE_UBUNTU_OPTIX_LIBRARY_INSTALL.elevated.as_str(),
                complained("E: Unable to locate package libnvidia-gl-", 100),
            ),
            (crate::elevated::THE_RESTART.elevated.as_str(), said("", 0)),
            (crate::readings::THE_DRIVER_CHECK.as_str(), said("a card", 0)),
        ]);
        let mut files = AMachine::default();
        let went = block_on(carry_out_the_driver_step(
            &mut channel,
            &mut files,
            &a_machine_with_a_card_and_no_driver_at_all(crate::kind::DistributionFamily::Ubuntu),
            the_check_failed_at(127),
            HowElevationIsObtained::Passwordless,
        ));
        let said = went.what_went_wrong().expect("the step says what went wrong");
        assert!(said.contains("Unable to locate package libnvidia-gl-"), "{said}");
        assert!(went.the_check_after.is_none());
        assert!(!channel
            .asked
            .iter()
            .any(|command| command.contains("reboot")));
    }

    /// **Where the support binaries will not install, the step ends there in
    /// the machine's own words and no restart goes out** — a restart cannot
    /// put a missing package on a machine, which is the same reason the tool
    /// install's own refusal ends the step.
    #[test]
    fn an_ubuntu_route_whose_support_binaries_will_not_install_ends_there() {
        let mut channel = AMachine::answering(vec![
            (crate::elevated::THE_LIST_UPDATE.elevated.as_str(), said("", 0)),
            (
                crate::elevated::THE_UBUNTU_DRIVER_TOOL_INSTALL.elevated.as_str(),
                said("", 0),
            ),
            (
                crate::elevated::THE_UBUNTU_DRIVER_INSTALL.elevated.as_str(),
                said("", 0),
            ),
            (
                crate::elevated::THE_UBUNTU_DRIVER_SUPPORT_BINARIES_INSTALL
                    .elevated
                    .as_str(),
                complained("E: Unable to locate package nvidia-utils-", 100),
            ),
            (crate::elevated::THE_RESTART.elevated.as_str(), said("", 0)),
            (crate::readings::THE_DRIVER_CHECK.as_str(), said("a card", 0)),
        ]);
        let mut files = AMachine::default();
        let went = block_on(carry_out_the_driver_step(
            &mut channel,
            &mut files,
            &a_machine_with_a_card_and_no_driver_at_all(crate::kind::DistributionFamily::Ubuntu),
            the_check_failed_at(127),
            HowElevationIsObtained::Passwordless,
        ));
        let said = went.what_went_wrong().expect("the step says what went wrong");
        assert!(said.contains("Unable to locate package"), "{said}");
        // **No check was taken over a step that could not finish its work**,
        // and Y4's *driver placed* is not written over it.
        assert!(went.the_check_after.is_none());
        assert!(!channel
            .asked
            .iter()
            .any(|command| command.contains("reboot")));
    }

    /// **The Debian route installs no support binaries**, because the
    /// `nvidia-driver` metapackage its own detection tool names brings the
    /// check with it — read back off `nvidia-smi` after the restart on
    /// 2026-08-22.
    #[test]
    fn the_debian_route_installs_no_support_binaries() {
        let mut channel = AMachine::answering(vec![
            (crate::elevated::THE_LIST_UPDATE.elevated.as_str(), said("", 0)),
            (crate::elevated::THE_COMPONENTS_WRITE.elevated.as_str(), said("", 0)),
            (
                crate::elevated::THE_DEBIAN_DRIVER_INSTALL.elevated.as_str(),
                said("", 0),
            ),
            (crate::readings::THE_DRIVER_CHECK.as_str(), said("a card", 0)),
        ]);
        let mut files = AMachine::default().holding(THE_DEBIAN_SOURCES_FILE, "Components: main\n");
        let went = block_on(carry_out_the_driver_step(
            &mut channel,
            &mut files,
            &a_machine_with_a_card_and_no_driver_at_all(crate::kind::DistributionFamily::Debian),
            the_check_failed_at(127),
            HowElevationIsObtained::Passwordless,
        ));
        assert_eq!(went.the_support_binaries, None);
        assert!(!channel.asked.iter().any(|command| command.contains("nvidia-utils")));
        // **Nor the OptiX library's act** (§13 item 27): whether Debian's
        // metapackage carries it is not established, and `Rendering` L24b is
        // what a Debian machine without it meets.
        assert_eq!(went.the_optix_library, None);
        assert!(!channel.asked.iter().any(|command| command.contains("libnvoptix")));
        assert!(went.the_install.is_some());
    }

    /// **A tool that could not be installed ends the step there, in the
    /// machine's own words**, and neither the install nor the restart goes
    /// out — the rule the Debian components step keeps.
    #[test]
    fn an_ubuntu_route_whose_tool_would_not_install_issues_no_install_and_no_restart() {
        let mut channel = AMachine::answering(vec![
            (crate::elevated::THE_LIST_UPDATE.elevated.as_str(), said("", 0)),
            (
                crate::elevated::THE_UBUNTU_DRIVER_TOOL_INSTALL.elevated.as_str(),
                complained("E: Unable to locate package ubuntu-drivers-common", 100),
            ),
            (
                crate::elevated::THE_UBUNTU_DRIVER_INSTALL.elevated.as_str(),
                said("", 0),
            ),
            (crate::elevated::THE_RESTART.elevated.as_str(), said("", 0)),
        ]);
        let mut files = AMachine::default();
        let went = block_on(carry_out_the_driver_step(
            &mut channel,
            &mut files,
            &a_machine_with_a_card_and_no_driver_at_all(crate::kind::DistributionFamily::Ubuntu),
            the_check_failed_at(12),
            HowElevationIsObtained::Passwordless,
        ));
        assert!(went.the_install.is_none());
        let said = went.what_went_wrong().expect("the step says what went wrong");
        assert!(said.contains("Unable to locate package"), "{said}");
        assert!(!channel
            .asked
            .iter()
            .any(|command| command.contains("ubuntu-drivers install") || command.contains("reboot")));
    }

    #[test]
    fn a_debian_route_whose_components_step_did_not_take_issues_no_install_and_no_restart() {
        // **M1b, and the fault this test exists for.** The components step's
        // answer was kept in a value and read by nothing until 2026-09-04, so
        // the install went out against a driver that was unreachable, the check
        // after it failed, and the restart took a paid machine down for between
        // nine and ninety seconds to fix nothing.
        let mut channel = AMachine::answering(vec![
            (
                crate::elevated::THE_DEBIAN_DRIVER_INSTALL.elevated.as_str(),
                said("", 0),
            ),
            (crate::elevated::THE_RESTART.elevated.as_str(), said("", 0)),
        ]);
        // The sources file is not there at all, so the components step cannot
        // read it.
        let mut files = AMachine::default();
        let went = block_on(carry_out_the_driver_step(
            &mut channel,
            &mut files,
            &a_machine_with_a_card_and_no_driver_at_all(crate::kind::DistributionFamily::Debian),
            the_check_failed_at(12),
            HowElevationIsObtained::Passwordless,
        ));
        assert!(matches!(
            went.the_components,
            Some(HowTheComponentsStepWent::TheFileCouldNotBeRead(_))
        ));
        assert!(went.the_install.is_none());
        assert!(went.what_went_wrong().is_some());
        assert!(!channel
            .asked
            .iter()
            .any(|command| command.contains("nvidia-driver") || command.contains("reboot")));
        // And the four cases are told apart: a file that already names all
        // three is *nothing to do* and lets the install go out.
        assert!(HowTheDriverStepWent::the_components_let_the_install_go_out(&Some(
            HowTheComponentsStepWent::NothingToDo
        )));
        assert!(HowTheDriverStepWent::the_components_let_the_install_go_out(&Some(
            HowTheComponentsStepWent::ItWasCarriedOut
        )));
        assert!(!HowTheDriverStepWent::the_components_let_the_install_go_out(&Some(
            HowTheComponentsStepWent::TheFileIsNotOneThisReadingDescribes
        )));
        assert!(!HowTheDriverStepWent::the_components_let_the_install_go_out(&Some(
            HowTheComponentsStepWent::ItWasRefused(HowTheActWent::ItWasCarriedOut)
        )));
    }

    #[test]
    fn a_restart_cannot_be_permitted_without_the_mark_having_been_set() {
        // **T4, and the guarantee is the type's**: `Yes` carries
        // `TheMarkWasSetBeforeTheCommandLeft`, which only
        // `crate::restart::mark_the_connected_period_restarting` produces, so a
        // caller that forgets the mark cannot build one and the code does not
        // compile. **The caller exists from step 10c's §8** and it is the
        // desktop's restart step; until then the restart went out around this
        // type rather than through it, which is what the type was written to
        // make impossible.
        assert!(the_mark_is_set().it_may());
        assert!(matches!(
            on_a_marked_period(crate::kind::MachineKind::AContainer, true),
            TheRestartMayBeIssued::No
        ));
        assert!(matches!(
            on_a_marked_period(crate::kind::MachineKind::ASystemdMachine, false),
            TheRestartMayBeIssued::No
        ));
    }

    #[test]
    fn a_debian_route_enables_the_components_before_it_installs() {
        let mut channel = AMachine::answering(vec![
            (crate::elevated::THE_COMPONENTS_WRITE.elevated.as_str(), said("", 0)),
            (crate::elevated::THE_LIST_UPDATE.elevated.as_str(), said("", 0)),
            (
                crate::elevated::THE_DEBIAN_DRIVER_INSTALL.elevated.as_str(),
                said("", 0),
            ),
            (
                crate::readings::THE_DRIVER_CHECK.as_str(),
                said("NVIDIA-SMI has failed", 9),
            ),
        ]);
        let mut files =
            AMachine::default().holding(THE_DEBIAN_SOURCES_FILE, "Components: main\n");
        let went = block_on(carry_out_the_driver_step(
            &mut channel,
            &mut files,
            &a_machine_with_a_card_and_no_driver_at_all(crate::kind::DistributionFamily::Debian),
            the_check_failed_at(12),
            HowElevationIsObtained::Passwordless,
        ));
        assert_eq!(went.the_components, Some(HowTheComponentsStepWent::ItWasCarriedOut));
        assert!(went.the_install.as_ref().unwrap().it_was_carried_out());
        assert_eq!(went.what_the_step_did(), crate::run::WhatTheStepDid::Performed);
        // **M1b: the restart is not optional on this family**, and the check
        // after the install is what says so — it still fails until the boot, so
        // the caller's R1 item 6 is owed.
        assert!(!went.the_driver_answers());
        // **No restart went out from this step**, which is Y4's *driver placed*
        // row: *"no restart has been issued"*.
        assert!(!channel
            .asked
            .iter()
            .any(|c| c.contains("reboot")));
        // The components were enabled before the install was issued.
        let at_components = channel
            .asked
            .iter()
            .position(|c| *c == crate::elevated::THE_LIST_UPDATE.elevated.as_str())
            .expect("the lists were updated");
        let at_install = channel
            .asked
            .iter()
            .position(|c| *c == crate::elevated::THE_DEBIAN_DRIVER_INSTALL.elevated.as_str())
            .expect("the driver was installed");
        assert!(at_components < at_install);
    }

    #[test]
    fn no_restart_is_issued_where_the_kind_forbids_one() {
        // **T1 and Fact 48**: on a container a restart would restart the
        // container and repair nothing, so none is ever issued — and R3 keeps
        // the driver step off that kind altogether, so the step performs
        // nothing there either.
        assert!(!on_a_marked_period(crate::kind::MachineKind::AContainer, true).it_may());
        let mut channel = AMachine::default();
        let mut files = AMachine::default();
        let needs = crate::ready_action::WhatTheMachineNeeds {
            kind: crate::kind::MachineKind::AContainer,
            ..a_machine_with_a_card_and_no_driver_at_all(crate::kind::DistributionFamily::Ubuntu)
        };
        let went = block_on(carry_out_the_driver_step(
            &mut channel,
            &mut files,
            &needs,
            the_check_failed_at(12),
            HowElevationIsObtained::Passwordless,
        ));
        assert_eq!(
            went.which_branch,
            crate::ready_action::WhatTheDriverStepDoes::Nothing
        );
        assert!(went.the_install.is_none());
        assert!(!went.the_driver_answers());
        assert!(channel.asked.is_empty());
    }

    #[test]
    fn the_components_edit_adds_what_is_missing_and_moves_nothing_else() {
        let held = "Types: deb\nURIs: http://deb.debian.org/debian\nSuites: bookworm\n\
                    Components: main\nSigned-By: /usr/share/keyrings/debian-archive-keyring.gpg\n";
        let rewritten = rewrite_the_components(held).expect("it changes");
        assert!(rewritten.contains("Components: main contrib non-free non-free-firmware\n"));
        assert!(rewritten.contains("Types: deb\n"));
        assert!(rewritten.contains("Signed-By: /usr/share/keyrings/debian-archive-keyring.gpg\n"));
        // Every other line is byte for byte what it was.
        assert_eq!(rewritten.lines().count(), held.lines().count());
    }

    #[test]
    fn a_file_that_already_names_all_three_is_a_step_with_nothing_to_do() {
        let held = "Components: main contrib non-free non-free-firmware\n";
        assert_eq!(rewrite_the_components(held), None);
    }

    #[test]
    fn a_file_with_no_components_field_is_not_repaired() {
        // **Inventing the field would be writing a sources file from memory**,
        // which is exactly what Rule 2 forbids and what the reading does not
        // license.
        assert_eq!(rewrite_the_components("Types: deb\nURIs: x\n"), None);
    }

    #[test]
    fn the_components_step_reads_edits_stages_and_copies_and_never_names_the_content() {
        let mut channel = AMachine::answering(vec![
            (crate::elevated::THE_COMPONENTS_WRITE.elevated.as_str(), said("", 0)),
            (crate::elevated::THE_LIST_UPDATE.elevated.as_str(), said("", 0)),
        ]);
        // **A machine with nothing on it but the sources file**, which is what a
        // fresh rental is: the carriage directory is made by the step itself,
        // and until 2026-09-04 this test handed the stand-in a machine that
        // already had one.
        let mut files =
            AMachine::default().holding(THE_DEBIAN_SOURCES_FILE, "Components: main\n");
        let went = block_on(enable_the_components(
            &mut channel,
            &mut files,
            HowElevationIsObtained::Passwordless,
        ));
        assert_eq!(went, HowTheComponentsStepWent::ItWasCarriedOut);
        assert!(files.written.iter().any(|(path, bytes)| path
            == THE_COMPONENTS_STAGING_SITE.link_path_on_the_subsystem
            && String::from_utf8_lossy(bytes).contains("non-free-firmware")));
        for command in &channel.asked {
            assert!(!command.contains("non-free"), "{command} names the content");
        }
        // **The staged file is taken back when the step ends**, so the machine
        // is not left holding a copy of its own sources file under the login's
        // home. **This is the assertion the fault of 2026-09-04 did not have**:
        // a rented machine was looked at after the step and still held it.
        assert!(
            !files
                .files
                .iter()
                .any(|(path, _)| path == THE_COMPONENTS_STAGING_SITE.link_path_on_the_subsystem),
            "the staged file was left on the machine: {:?}",
            files.files
        );
    }

    /// **The staged file is taken back on the refused route too**, which is the
    /// route a machine that will not take the elevated copy leaves behind.
    #[test]
    fn a_components_step_the_machine_refused_still_takes_its_staged_file_back() {
        let mut channel = AMachine::answering(vec![(
            crate::elevated::THE_COMPONENTS_WRITE.elevated.as_str(),
            complained("cp: cannot create regular file", 1),
        )]);
        let mut files = AMachine::default().holding(THE_DEBIAN_SOURCES_FILE, "Components: main\n");
        let went = block_on(enable_the_components(
            &mut channel,
            &mut files,
            HowElevationIsObtained::Passwordless,
        ));
        assert!(matches!(went, HowTheComponentsStepWent::ItWasRefused(_)));
        assert!(files
            .written
            .iter()
            .any(|(path, _)| path == THE_COMPONENTS_STAGING_SITE.link_path_on_the_subsystem));
        assert!(
            !files
                .files
                .iter()
                .any(|(path, _)| path == THE_COMPONENTS_STAGING_SITE.link_path_on_the_subsystem),
            "the staged file was left on the machine after a refusal: {:?}",
            files.files
        );
    }

    #[test]
    fn a_refused_staging_write_is_not_reported_as_a_file_that_could_not_be_read() {
        // **The fault this test exists for.** The staging write's refusal
        // returned *the file could not be read*, and the value's own comment
        // said it covered both — so the naming was wrong and the user was told
        // the wrong thing about a machine that refused a write.
        struct AMachineThatWillNotTakeTheStagingWrite(AMachine);
        impl CommandChannel for AMachineThatWillNotTakeTheStagingWrite {
            async fn run(
                &mut self,
                command: smoozy_server_protocol::transport::CommandString,
            ) -> Result<
                smoozy_server_protocol::transport::CommandResult,
                smoozy_server_protocol::transport::ssh::CommandFailure,
            > {
                self.0.run(command).await
            }
        }
        impl TheMachinesFileSurface for AMachineThatWillNotTakeTheStagingWrite {
            async fn read_whole(&mut self, path: &str) -> Result<Vec<u8>, WhatTheMachineSaid> {
                self.0.read_whole(path).await
            }
            async fn write_whole(
                &mut self,
                _path: &str,
                _bytes: &[u8],
            ) -> Result<(), WhatTheMachineSaid> {
                Err(WhatTheMachineSaid::TheLoginMayNot)
            }
            async fn make_one_directory(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
                self.0.make_one_directory(path).await
            }
            async fn remove_one_file(&mut self, path: &str) -> Result<(), WhatTheMachineSaid> {
                self.0.remove_one_file(path).await
            }
            async fn remove_one_empty_directory(
                &mut self,
                path: &str,
            ) -> Result<(), WhatTheMachineSaid> {
                self.0.remove_one_empty_directory(path).await
            }
            async fn list_one_directory(
                &mut self,
                path: &str,
            ) -> Result<Vec<crate::readings::AListedThing>, WhatTheMachineSaid> {
                self.0.list_one_directory(path).await
            }
            async fn plant_a_link(
                &mut self,
                target: &str,
                at: &str,
            ) -> Result<(), WhatTheMachineSaid> {
                self.0.plant_a_link(target, at).await
            }
        }

        let mut channel = AMachine::default();
        let mut files = AMachineThatWillNotTakeTheStagingWrite(
            AMachine::default().holding(THE_DEBIAN_SOURCES_FILE, "Components: main\n"),
        );
        let went = block_on(enable_the_components(
            &mut channel,
            &mut files,
            HowElevationIsObtained::Passwordless,
        ));
        assert_eq!(
            went,
            HowTheComponentsStepWent::TheStagingWriteWasRefused(WhatTheMachineSaid::TheLoginMayNot)
        );
        // **Both end the driver step the same way**, which is what stops the
        // install and the restart.
        assert!(!HowTheDriverStepWent::the_components_let_the_install_go_out(&Some(went)));
        assert!(channel.asked.is_empty());
    }

    #[test]
    fn the_components_step_on_a_machine_that_already_has_them_does_nothing_at_all() {
        let mut channel = AMachine::default();
        let mut files = AMachine::default().holding(
            THE_DEBIAN_SOURCES_FILE,
            "Components: main contrib non-free non-free-firmware\n",
        );
        assert_eq!(
            block_on(enable_the_components(
                &mut channel,
                &mut files,
                HowElevationIsObtained::Passwordless
            )),
            HowTheComponentsStepWent::NothingToDo
        );
        assert!(channel.asked.is_empty());
    }

    #[test]
    fn the_search_counts_every_complaint_line_and_reads_none_of_them() {
        let mut channel = AMachine::answering(vec![(
            crate::readings::THE_SEARCH.as_str(),
            smoozy_server_protocol::transport::CommandResult {
                output: b"/opt/blender-4.5/blender\n/snap/blender/current/blender\n".to_vec(),
                complaint: b"find: '/root': Permission denied\nfind: '/lost+found': Permission denied\n"
                    .to_vec(),
                status: Some(1),
            },
        )]);
        let answer = block_on(run_the_search(&mut channel, None, None));
        assert_eq!(answer.found.len(), 2);
        assert_eq!(answer.folders_refused, 2);
        let entries: Vec<ACandidate> = answer
            .found
            .iter()
            .map(|path| ACandidate {
                path: path.clone(),
                answered: None,
            })
            .collect();
        let says = crate::search::what_the_answer_says(&entries, answer.folders_refused);
        assert!(!crate::search::the_machine_was_searched_completely(&says));
    }

    /// **B10: the search does not offer Smoozy its own Blender area as a
    /// candidate, and it drops nothing else.**
    ///
    /// **This is the test the fault of 2026-09-04 did not have.** On a rented
    /// Debian machine the whole-disk search returned five paths, three of them
    /// directories, and one of those was the `blender` area this module's own
    /// placement had made — so on every prepared machine Smoozy would spend an
    /// asking run on its own folder and then name it to the user as a candidate
    /// that did not answer.
    #[test]
    fn the_search_does_not_offer_smoozy_its_own_blender_area() {
        let area = crate::placement::the_blender_area(&a_layout());
        let mut channel = AMachine::answering(vec![(
            crate::readings::THE_SEARCH.as_str(),
            smoozy_server_protocol::transport::CommandResult {
                output: format!(
                    "/opt/blender-4.5/blender\n{area}\n{area}/blender-4.5.12/blender\n\
                     /home/debian/blender\n"
                )
                .into_bytes(),
                complaint: Vec::new(),
                status: Some(0),
            },
        )]);
        let answer = block_on(run_the_search(&mut channel, None, Some(&area)));
        // **The area's own path is gone.**
        assert!(!answer.found.iter().any(|path| path == &area));
        // **And nothing else is** — a build inside the area is still a
        // candidate, and so is a folder of the machine's own that happens to
        // carry the name.
        assert_eq!(
            answer.found,
            vec![
                "/opt/blender-4.5/blender".to_string(),
                format!("{area}/blender-4.5.12/blender"),
                "/home/debian/blender".to_string(),
            ]
        );
        // **A machine this module has placed no root on loses nothing.**
        let mut again = AMachine::answering(vec![(
            crate::readings::THE_SEARCH.as_str(),
            smoozy_server_protocol::transport::CommandResult {
                output: format!("{area}\n").into_bytes(),
                complaint: Vec::new(),
                status: Some(0),
            },
        )]);
        assert_eq!(block_on(run_the_search(&mut again, None, None)).found.len(), 1);
    }

    #[test]
    fn the_version_ask_names_the_token_and_reads_both_streams() {
        let mut channel = AMachine::answering(vec![(
            crate::readings::ASK_THE_BUILD_ITS_VERSION.as_str(),
            smoozy_server_protocol::transport::CommandResult {
                output: b"Color management: using fallback mode\n".to_vec(),
                complaint: b"Blender 4.5.12 LTS\n".to_vec(),
                status: Some(0),
            },
        )]);
        let mut files = AMachine::default();
        let candidate = block_on(ask_one_build_its_version(
            &mut channel,
            &mut files,
            "4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c02",
            "/opt/blender-4.5/blender",
        ));
        // **Both streams, always** — the answer here is on the complaint stream
        // and a reader of one stream alone would have missed it.
        assert!(candidate.answered.is_some());
        assert_eq!(candidate.path, "/opt/blender-4.5/blender");
        assert!(channel
            .asked
            .iter()
            .all(|command| !command.contains("/opt/blender")));
    }

    #[test]
    fn a_version_ask_that_never_answers_ends_at_its_limit_rather_than_hanging() {
        // **B1, and the fault this test exists for.** No Blender run this module
        // makes carried a time limit until 2026-09-04, and the command channel
        // underneath put no bound on the wait either — so B12's search, run when
        // `Render Settings` asks for the version list, could stop the desktop
        // with no ending and no message on a machine billed by the hour.
        //
        // **The clock is paused, so this test ends at once and a bound that
        // stopped firing would fail here rather than run for ever.**
        use crate::readings::tests::{block_on_with_time_paused, AMachineThatNeverAnswers};
        let mut channel = AMachineThatNeverAnswers;
        let mut files = AMachineThatNeverAnswers;
        // **The outer wait is what makes a broken bound fail rather than
        // hang.** Take the ask's own limit away and this fires instead, at ten
        // times the limit of virtual time, and the test fails.
        let candidate = block_on_with_time_paused(async {
            tokio::time::timeout(
                crate::blender::THE_TIME_LIMIT_EVERY_BLENDER_RUN_CARRIES * 10,
                ask_one_build_its_version(
                    &mut channel,
                    &mut files,
                    "4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c03",
                    "/opt/never/blender",
                ),
            )
            .await
            .expect("the ask's own limit ends it, and this outer wait never fires")
        });
        // **A run that hit the limit answers nothing**, which is the value B11
        // already names as a candidate that did not answer.
        assert_eq!(candidate.answered, None);
        assert_eq!(candidate.path, "/opt/never/blender");
        let says = crate::search::what_the_answer_says(&[candidate], 0);
        assert_eq!(says.did_not_answer, vec!["/opt/never/blender".to_string()]);
    }

    #[test]
    fn a_build_that_did_not_answer_is_recorded_as_one_that_did_not() {
        let mut channel = AMachine::default();
        let mut files = AMachine::default();
        let candidate = block_on(ask_one_build_its_version(
            &mut channel,
            &mut files,
            "4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c04",
            "/opt/renamed/blender",
        ));
        assert_eq!(candidate.answered, None);
        let says = crate::search::what_the_answer_says(&[candidate], 0);
        assert_eq!(says.did_not_answer, vec!["/opt/renamed/blender".to_string()]);
    }

    #[test]
    fn the_folder_deletion_removes_the_tree_bottom_up() {
        let mut files = AMachine::default()
            .holding("/mnt/one/smoozy/blender/b1/blender", "x")
            .holding("/mnt/one/smoozy/blender/b1/readme", "y");
        let went = block_on(delete_the_install_folder(
            &mut files,
            "/mnt/one/smoozy/blender/b1",
        ));
        assert_eq!(went, HowTheActWent::ItWasCarriedOut);
        assert!(files.files.is_empty());
    }

    #[test]
    fn a_managed_build_outside_the_fixed_set_is_one_this_module_cannot_remove() {
        let mut channel = AMachine::default();
        assert_eq!(
            block_on(remove_the_managed_build(
                &mut channel,
                "blender-bin",
                HowElevationIsObtained::Passwordless
            )),
            HowTheManagerRouteWent::ThisModuleCannotRemoveIt
        );
        assert!(channel.asked.is_empty());
    }

    #[test]
    fn the_removal_issues_the_command_of_the_manager_the_name_is_paired_with() {
        // **The fault this test exists for.** The removal knew one package name
        // and one command and nothing tied them, so a build under a name the set
        // happened to hold would have got whichever command the code named — and
        // an apt-installed `/usr/bin/blender`, Fact 45's real case, got the snap
        // command.
        let pairing = crate::search::the_manager_route_for("blender").expect("the store's own");
        let mut channel = AMachine::answering(vec![(pairing.removal.elevated.as_str(), said("", 0))]);
        let went = block_on(remove_the_managed_build(
            &mut channel,
            "blender",
            HowElevationIsObtained::Passwordless,
        ));
        assert_eq!(
            went,
            HowTheManagerRouteWent::ItWasAttempted(HowTheActWent::ItWasCarriedOut)
        );
        // **The command issued is that manager's own and no other.**
        assert_eq!(channel.asked, vec![pairing.removal.elevated.as_str()]);
    }

    #[test]
    fn the_telegram_material_is_placed_as_paths_and_never_as_arguments() {
        // The package area is one of R7's five, made by the placement before
        // any material is put in it.
        let mut files = AMachine::default().with_directory("/mnt/one/smoozy/package");
        let placed = block_on(place_the_telegram_material(
            &mut files,
            &[("/mnt/one/smoozy/package/telegram-token".to_string(), b"t".to_vec())],
        ));
        assert!(placed.iter().all(|(_, went)| went.it_was_carried_out()));
        assert_eq!(files.written.len(), 1);
    }
}

// ------------------------------------------------------------------
// Two version asks on one server — step 10c's eleventh-pass fix piece 4c.
// ------------------------------------------------------------------

#[cfg(test)]
mod two_version_asks_on_one_server {
    use super::*;
    use smoozy_security_and_licence::plaintext::Plaintext;
    use smoozy_server_protocol::transport::ssh::CommandFailure;
    use smoozy_server_protocol::transport::{CommandResult, CommandString};
    use std::future::Future;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use crate::readings::AListedThing;

    /// **One server's home, as two asks on it see it**: one link at the
    /// site, and the builds it may point at, each with its own `Blender ` line.
    #[derive(Default)]
    struct AServersHome {
        the_link_points_at: Option<String>,
        builds: Vec<(String, String)>,
    }

    /// **How many version asks are running at one moment**, across every
    /// server, and the most there ever were.
    #[derive(Default)]
    struct AtOnce {
        now: AtomicUsize,
        most: AtomicUsize,
    }

    /// **One ask's handle on a server**: the file operations land at once,
    /// and the command runs a round trip after it is asked — the moment
    /// another ask's link can land first.
    struct AnAskOn {
        home: Arc<Mutex<AServersHome>>,
        at_once: Arc<AtOnce>,
    }

    const A_ROUND_TRIP: std::time::Duration = std::time::Duration::from_millis(40);

    impl CommandChannel for AnAskOn {
        async fn run(&mut self, command: CommandString) -> Result<CommandResult, CommandFailure> {
            assert_eq!(
                command.as_str(),
                crate::readings::ASK_THE_BUILD_ITS_VERSION.as_str()
            );
            let now = self.at_once.now.fetch_add(1, Ordering::SeqCst) + 1;
            self.at_once.most.fetch_max(now, Ordering::SeqCst);
            tokio::time::sleep(A_ROUND_TRIP).await;
            self.at_once.now.fetch_sub(1, Ordering::SeqCst);
            let home = self.home.lock().expect("the home");
            let line = home.the_link_points_at.as_ref().and_then(|target| {
                home.builds
                    .iter()
                    .find(|(path, _)| path == target)
                    .map(|(_, line)| line.clone())
            });
            Ok(match line {
                Some(line) => CommandResult {
                    output: line.into_bytes(),
                    complaint: Vec::new(),
                    status: Some(0),
                },
                None => CommandResult {
                    output: Vec::new(),
                    complaint: b"No such file or directory".to_vec(),
                    status: Some(127),
                },
            })
        }
    }

    impl CarriesAValueOnACommandsStandardInput for AnAskOn {
        fn run_carrying_a_value_on_its_standard_input(
            &mut self,
            _command: CommandString,
            _value: Plaintext,
        ) -> impl Future<Output = Result<CommandResult, CommandFailure>> + Send {
            std::future::ready(Err(CommandFailure::TheRequestDidNotReachTheMachine))
        }
    }

    impl TheMachinesFileSurface for AnAskOn {
        fn read_whole(
            &mut self,
            _path: &str,
        ) -> impl Future<Output = Result<Vec<u8>, WhatTheMachineSaid>> + Send {
            std::future::ready(Err(WhatTheMachineSaid::ThePathIsNotThere))
        }

        fn write_whole(
            &mut self,
            _path: &str,
            _bytes: &[u8],
        ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send {
            std::future::ready(Ok(()))
        }

        fn make_one_directory(
            &mut self,
            _path: &str,
        ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send {
            std::future::ready(Err(WhatTheMachineSaid::ItIsAlreadyThere))
        }

        fn remove_one_file(
            &mut self,
            path: &str,
        ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send {
            assert_eq!(
                path,
                crate::readings::THE_VERSION_ASKS_CARRIAGE_SITE.link_path_on_the_subsystem
            );
            self.home.lock().expect("the home").the_link_points_at = None;
            std::future::ready(Ok(()))
        }

        fn remove_one_empty_directory(
            &mut self,
            _path: &str,
        ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send {
            std::future::ready(Ok(()))
        }

        fn list_one_directory(
            &mut self,
            _path: &str,
        ) -> impl Future<Output = Result<Vec<AListedThing>, WhatTheMachineSaid>> + Send {
            std::future::ready(Ok(Vec::new()))
        }

        /// **A link left at the site is replaced**, as the real one is.
        fn plant_a_link(
            &mut self,
            target: &str,
            at: &str,
        ) -> impl Future<Output = Result<(), WhatTheMachineSaid>> + Send {
            assert_eq!(
                at,
                crate::readings::THE_VERSION_ASKS_CARRIAGE_SITE.link_path_on_the_subsystem
            );
            self.home.lock().expect("the home").the_link_points_at = Some(target.to_string());
            std::future::ready(Ok(()))
        }
    }

    const THE_OLD_ONE: &str = "/opt/blender-4.2/blender";
    const THE_NEW_ONE: &str = "/opt/blender-4.5/blender";

    fn a_home_holding(builds: &[(&str, &str)]) -> Arc<Mutex<AServersHome>> {
        Arc::new(Mutex::new(AServersHome {
            the_link_points_at: None,
            builds: builds
                .iter()
                .map(|(path, line)| (path.to_string(), line.to_string()))
                .collect(),
        }))
    }

    fn an_ask_on(home: &Arc<Mutex<AServersHome>>, at_once: &Arc<AtOnce>) -> AnAskOn {
        AnAskOn {
            home: Arc::clone(home),
            at_once: Arc::clone(at_once),
        }
    }

    fn the_version(text: &str) -> Option<crate::blender::Version> {
        crate::blender::Version::parse(text)
    }

    #[test]
    fn two_version_asks_on_one_server_at_once_each_answer_for_their_own_build() {
        let home = a_home_holding(&[
            (THE_OLD_ONE, "Blender 4.2.3 LTS\n"),
            (THE_NEW_ONE, "Blender 4.5.1\n"),
        ]);
        let at_once = Arc::new(AtOnce::default());
        let (mut old_channel, mut old_files) =
            (an_ask_on(&home, &at_once), an_ask_on(&home, &at_once));
        let (mut new_channel, mut new_files) =
            (an_ask_on(&home, &at_once), an_ask_on(&home, &at_once));
        // **Its own server identifier**, so no other test of this binary
        // shares its turn.
        const THIS_SERVER: &str = "4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c10";

        let (old, new) = crate::readings::tests::block_on(async {
            tokio::join!(
                ask_one_build_its_version(
                    &mut old_channel,
                    &mut old_files,
                    THIS_SERVER,
                    THE_OLD_ONE
                ),
                // **Half a round trip later**, as a second search reaching the
                // site while the first's command is on its way.
                async {
                    tokio::time::sleep(A_ROUND_TRIP / 2).await;
                    ask_one_build_its_version(
                        &mut new_channel,
                        &mut new_files,
                        THIS_SERVER,
                        THE_NEW_ONE,
                    )
                    .await
                },
            )
        });

        assert_eq!(
            (old, new),
            (
                crate::search::ACandidate {
                    path: THE_OLD_ONE.to_string(),
                    answered: the_version("4.2.3"),
                },
                crate::search::ACandidate {
                    path: THE_NEW_ONE.to_string(),
                    answered: the_version("4.5.1"),
                },
            ),
            "(the 4.2 build's ask, the 4.5 build's ask)"
        );
        assert_eq!(
            at_once.most.load(Ordering::SeqCst),
            1,
            "one ask at the site at a time"
        );
        assert_eq!(home.lock().expect("the home").the_link_points_at, None);
    }

    #[test]
    fn version_asks_on_two_servers_do_not_wait_for_each_other() {
        let (first_home, second_home) = (
            a_home_holding(&[(THE_OLD_ONE, "Blender 4.2.3 LTS\n")]),
            a_home_holding(&[(THE_NEW_ONE, "Blender 4.5.1\n")]),
        );
        let at_once = Arc::new(AtOnce::default());
        let (mut first_channel, mut first_files) = (
            an_ask_on(&first_home, &at_once),
            an_ask_on(&first_home, &at_once),
        );
        let (mut second_channel, mut second_files) = (
            an_ask_on(&second_home, &at_once),
            an_ask_on(&second_home, &at_once),
        );

        let (old, new) = crate::readings::tests::block_on(async {
            tokio::join!(
                ask_one_build_its_version(
                    &mut first_channel,
                    &mut first_files,
                    "4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c11",
                    THE_OLD_ONE,
                ),
                ask_one_build_its_version(
                    &mut second_channel,
                    &mut second_files,
                    "4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c12",
                    THE_NEW_ONE,
                ),
            )
        });

        assert_eq!(old.answered, the_version("4.2.3"));
        assert_eq!(new.answered, the_version("4.5.1"));
        assert_eq!(
            at_once.most.load(Ordering::SeqCst),
            2,
            "two servers' asks ran at once"
        );
    }

    #[test]
    fn a_turn_at_one_site_does_not_hold_another_site_on_the_same_server() {
        // **A version ask of up to a minute must not hold back a four-value
        // reading on the same server**: two sites are two links.
        const THIS_SERVER: &str = "4c4c4c4c4c4c4c4c4c4c4c4c4c4c4c13";
        const ANOTHER_SITE: CarriageSite = CarriageSite {
            link_path_on_the_subsystem: ".smoozy-carriage/another-site",
            named_in_a_command_as: "~/.smoozy-carriage/another-site",
        };
        crate::readings::tests::block_on(async {
            let _held =
                take_the_turn_at(crate::readings::THE_VERSION_ASKS_CARRIAGE_SITE, THIS_SERVER)
                    .await;
            tokio::time::timeout(A_ROUND_TRIP, take_the_turn_at(ANOTHER_SITE, THIS_SERVER))
                .await
                .expect("another site's turn is free while this one is held");
            assert!(
                tokio::time::timeout(
                    A_ROUND_TRIP,
                    take_the_turn_at(crate::readings::THE_VERSION_ASKS_CARRIAGE_SITE, THIS_SERVER),
                )
                .await
                .is_err(),
                "the same site's turn on the same server waits"
            );
        });
    }
}
