//! **Set up — a server's second tab: what getting a machine ready looks like.**
//! The page half is `web/set-up.js`, and it hangs off the tab strip
//! `web/a-server.js` draws.
//!
//! **This file is the surface and not the deciding.** Every judgement below is
//! `Server Setup`'s, in `App/Server Setup/`, and this file translates: it takes
//! a reading of the machine, asks that module what it means, composes the
//! words, and hands them over. **Nothing here decides what a setup is, what a
//! step did, which line the card shows, or what a removal costs.**
//!
//! ## What this piece draws, cited the way the runner ruled on 2026-09-12
//!
//! **The Design document and the heading**, and **the mockup state names**.
//!
//! - `Design/Server Setup § Where it sits`, `§ Two sections, and no third`,
//!   `§ This server`, `§ Blender`, `§ The folder column stays`, `§ Installing`,
//!   `§ Pointing at a build on your own computer`, `§ The below-4.5 warning`,
//!   `§ Removing a Blender`, `§ Adding Telegram to a server already set up`,
//!   `§ When a step had nothing to do`, `§ A second setup while one is
//!   running`, `§ A long step`, `§ Cut off part-way`, `§ The step that failed`,
//!   `§ The two ways out, and the control the second one lost`,
//!   `§ Admin access`, `§ Keyboard shortcuts`.
//! - The mockup states, which are the `<option>` values of
//!   `Preparation/Design/Mockup/Smoozy.html`'s own state list and are what this
//!   half is counted by: `su`, `su-connect`, `su-connectfail`, `su-notset`, `su-add`,
//!   `su-unpack`, `su-point`,
//!   `su-low`, `su-notbegun`, `su-run`, `su-cpu`, `su-quiet`, `su-none`, `su-nothing`,
//!   `su-optixoff`, `su-drvold`,
//!   `su-tg`, `su-tgnew`, `su-going`, `su-busy`, `su-fail`, `su-step`, `su-rm`,
//!   `su-elev`, `pw-nokeep`, `pw-nostore`, `pw-again`, `su-out`, `su-outlog`,
//!   `su-fetch`, `su-logempty`.
//!   **Thirty-one in the *Setting a server up* group.** `su-logempty` draws
//!   the selected stream without printed words and therefore without Copy
//!   (the seventeenth notes, piece 5). `su-fetch` draws Blender's download.
//!   *(Twenty-seven until 2026-10-18: `su-out` — a long step while the server
//!   prints, its newest lines beside the steps — and `su-outlog` — its
//!   *Server output* opened — were drawn by the plan pass for the Creator's
//!   thirteenth notes and built by their piece H.)*
//!   *(Twenty-six until 2026-10-16: `su-notbegun` — the card naming the
//!   installs Smoozy closed before it began — was drawn and built by step
//!   10c's owed piece 4.)*
//!   *(Twenty-four until 2026-10-11: `su-optixoff` — a Blender the server's
//!   driver is too old for — and `su-drvold` — the same on a server whose
//!   driver Smoozy can't update — were drawn by the plan pass for OptiX and the
//!   driver and built by its fourth piece.)*
//!   *(Twenty-three until 2026-10-03: `su-connectfail`, the card where the
//!   connection an add asked for was not made, was drawn by the drawing pass for
//!   the Creator's notes on the test copy, after their piece 1 had built it.)*
//!   *(Twenty-two until 2026-10-02: `su-connect`, the card while the
//!   connection an add asked for is being made, was drawn by the plan pass for
//!   the Creator's notes on 0.1.7 and is their piece 3's.)* *(Twenty-one
//!   until 2026-09-18: `pw-nostore` — `Security and licence` S34's sentence in
//!   the admin access box — was drawn for step 10b's first audit round,
//!   its second upstream item.)*
//!
//! ## The two Telegram cards
//!
//! **`su-tg` and `su-tgnew` are drawn from `Server Setup` Y6b's reading.** This
//! piece refused them on 2026-09-12 because there was no `Telegram` module;
//! **step 10 piece 8 added *Install it* on 2026-09-14**, and pressing it gets
//! this tab's one answer, because `Server Setup` G2's placement is made over a
//! connection and nothing in this copy takes it over one.
//!
//! **`App/Server Setup/src/telegram.rs` is not the module and is not reached
//! for.** It is `Server Setup`'s own placement of Telegram's material — G2 and
//! G3 — and it says what a setup run places, not what Telegram is.
//!
//! **`su-tg` is reachable since 2026-10-03 (the 38's piece 6)**: the tab
//! takes the role's own reading off the machine. **`su-tgnew`'s offer is drawn
//! since 2026-10-04 (the 38's piece 11)**: Settings' switch is handed to the
//! card ([`the_tab_with`]). **Its *Install it* runs `Server Setup` G2's
//! dedicated entry point** ([`crate::setting_up::the_telegram_placement_on`]),
//! wired 2026-10-04 once the machine's final check stopped ending that run.
//!
//! ## What can actually happen today, said plainly
//!
//! **Rewritten 2026-09-21**, after the first pass on a real rental found the
//! card saying *Smoozy has not read this server yet* on a connected server with
//! a setup that had just failed on it. **Where a connection stands, the tab
//! reads the machine** — `Server Setup` S1, S2 and Y6a, fresh and never from
//! anything stored — through [`crate::setting_up::the_setup_on`]: the run the
//! machine-level setup-in-progress state stands for (`Rented-server runtime`
//! V9), or else the ready action's run this copy last drove on that server.
//! **Since 2026-10-03 (the 38's piece 6) it also takes the card's own reading
//! of the machine** — `smoozy_framing`'s `THE_SET_UP_CARDS_READING_REQUEST`:
//! the ready action's retained run (`Server Setup` Y6b), which is what a copy
//! opened after the setup reads its card from; `Server Setup` W3a's three
//! answers with W4's line and ask; and the Telegram role's own state. A
//! machine program from before it does not answer, and the card is then the
//! run's alone, as before.
//!
//! **Where no connection stands**, the card is what was last read from that
//! server in this run, greyed and stamped with when ([`WhatTheMachineSaid::LastSeen`],
//! `Server Setup` Y6a), held in memory and written nowhere; where nothing was
//! read in this run it answers [`WhatTheMachineSaid::NothingHasEverBeenReadFromIt`].
//!
//! **Two things on this page run whole and reach outside nothing of `Servers`':**
//!
//! - **The install list.** `Server Setup` B2's read of the shop-front's own
//!   listing is this module's one outside fetch and it touches no machine, so
//!   [`the_install_list`] is real on any computer with a line to the internet.
//! - **The admin-access box's own shape.** `Security and licence` S36 composes
//!   the offer from the secret store's standing, which is a durable value of
//!   this computer's — so how many choices the box offers is a real reading
//!   here.
//!
//! **The acts** are [`a_set_up_act_that_needs_a_connection`]'s: the ready
//! action, the stop, a Blender picked from the list, the abandon and *Install
//! it* are driven; `Server
//! Setup` E2's offer is answered by [`the_admin_access_is_answered`]; **every
//! other act says it is not built** rather than doing nothing twice — in the one
//! sentence every piece of a server's screens says for it
//! ([`crate::surface::files::A_CONNECTION_IS_NOT_BUILT`]). **The folder column
//! and the *Transfers* link are not acts**: the first opens Files at that folder
//! and the second goes to Transfers (must-do 30 of the step 10a round 1 audit of
//! these pieces).
//!
//! ## How a clause marker is written in this crate
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. **Every other
//! document's marker is written out with that document at the site, or filed in
//! the list below** — this crate's plan defines a B-series, an E-series, an
//! R-series, an S-series, a T-series, a W-series and a Y-series of its own, and
//! a bare marker does not fail to resolve: **it resolves to a real and wrong
//! clause, which nobody reading it can tell from a right one.**
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server Setup` B1, B2, B6a, B7, B7a, B8a, B11, B12, B12a,
//! B13, B14, B15, S2a, T6a, Y4a, Y6a, Y6b. `Security and licence` S36.
//!
//! **Every other marker of that plan is written out with the document at the
//! site**, which is what the list is not allowed to stand in for wherever this
//! crate's own plan defines one of the same name.

use std::sync::Mutex;

use tauri::State;

use smoozy_security_and_licence::standing::{MachinePasswordChoice, StoreStanding};
use smoozy_server_setup::blender::{PublishedChannel, Risk, TheMark, Version};
use smoozy_server_setup::listing::WhyTheListingCouldNotBeRead;
use smoozy_server_setup::recovery::{HowTheRunEnded, TheFailedStepsWords, WhereAStepGot};
use smoozy_server_setup::run::{
    Boundary, RecordedBoundary, SetupKind, TheSetupRecord, WhatTheStepDid,
};
use smoozy_server_setup::search::{
    what_the_warning_says, ALiveJob, LiveState, RemovalRoute, WhatTheWarningSays,
};
use smoozy_server_setup::statements::{
    what_the_device_line_says, which_line, RecordedOutcome, WhatTheManifestAskedFor,
    WhatTheThreeQuestionsAnswered, WhatTheLineNames, WhatW4Concludes, WhichLine,
};

use crate::events::{Event, Said, Subject as About, TheOneQueue};
use crate::presentation::as_of_that_moment::AsOfThatMoment;
use crate::presentation::the_machines_own_words::Stream;
use crate::start::Started;
use crate::surface::servers::Words;
use crate::typography::{Prose, Verbatim};

/// **The one kind this piece posts**, declared here beside the service that
/// posts it and copied into `web/set-up.js`. Nothing holds the two together, so
/// copy carefully.
///
/// **One kind, because every act on this tab answers one question**: what the
/// user asked the setup to do. That is piece 1's own shape and its reasoning
/// carries here unchanged.
pub const THE_ANSWER: &str = "set-up:the-answer";

// ════════════════════════════════════════════════════════════════════════════
// Every word the user reads
// ════════════════════════════════════════════════════════════════════════════

pub(crate) mod words {
    // ── The two sections — `Design/Server Setup § Two sections, and no third`
    pub const THIS_SERVER: &str = "This server";
    pub const BLENDER: &str = "Blender";
    pub const READ_FROM_THE_SERVER: &str = "live from the server";

    // ── The card's state line — `Server Setup` W1's endings, in Design's words
    pub const READY_TO_RENDER: &str = "Ready to render";
    /// **A setup that reached its end on a server where the search found no
    /// Blender** — the run is done, and nothing renders without a Blender, so
    /// the card says what is true and no more (Rule 8). Added 2026-10-26, the
    /// worldwide pass's setup faults, fault 3: *Set up this server* after a
    /// refused add read *Ready to render* on World A with no Blender on it.
    /// *Ready to render* stays where a Blender was found.
    pub const SET_UP_STATE: &str = "Set up";
    pub const NOT_SET_UP: &str = "Not set up";
    pub const NOTHING_TO_PREPARE: &str = "Nothing to prepare";
    pub const SETTING_UP: &str = crate::surface::servers::words::SETTING_UP;
    pub const SETUP_STOPPED_PART_WAY: &str = "Setup stopped part-way";
    pub const THE_SERVER_IS_FINISHING_THE_CUT_RUN: &str =
        "Waiting for the interrupted setup to finish.";
    /// **`Design/Server Setup § This server`: just after *Add and set up*,
    /// while the connection it asked for is being made** — the drawing's
    /// `suconnect`, the Creator's note 3 on 0.1.7.
    pub const CONNECTING: &str = "Connecting\u{2026}";

    // ── The moment beside it. **Prose beside a value, never joined into one**
    // (X7): the moment is a reading on the machine's own clock.
    pub const SET_UP: &str = "set up";
    pub const CHECKED: &str = "checked";
    pub const STARTED: &str = "started";
    pub const ON_THE_CLOCK: &str = "on the clock of";

    // ── What was done to the machine — the card's list
    pub const TELEGRAM_INSTALLED: &str = "Telegram reporting installed";
    /// **A removal that reached *done*.** Ruled here under the Creator's
    /// standing delegation of 2026-08-31. The 38's piece 14c.
    pub const BLENDER_REMOVED: &str = "Blender removed.";
    pub const TELEGRAM_NOT_INSTALLED: &str = "Telegram reporting not installed";
    pub const THE_SERVER_IS_NOT_RESTARTED: &str = "no restart";
    /// `Design/Server Setup § Adding Telegram to a server already set up`, the
    /// drawing's own control.
    pub const INSTALL_IT: &str = "Install it";

    // ── `Server Setup` W3a's three lines, in `Design/Server Setup § This
    // server`'s own words.
    pub const LAST_JOB_RENDERED_ON: &str = "Last job rendered on";
    /// **Between the device and its route** on that line: *NVIDIA GeForce RTX
    /// 4090 · OptiX* (`Design/Server Setup` line 56, changed 2026-10-17, the
    /// Creator's tenth notes, note 3).
    pub const BETWEEN_THE_DEVICE_AND_ITS_ROUTE: &str = " \u{b7} ";
    pub const NONE_OF_THEM_SAID: &str =
        "Jobs rendered here; none reported the render device.";
    pub const NOTHING_HAS_RENDERED_YET: &str =
        "Nothing rendered yet. GPU rendering not confirmed.";

    // ── `Servers` H11b and `Server Setup` B12b, in `Design/Server Setup § This
    // server`'s and `§ Blender`'s own words. Added 2026-10-11, the fourth
    // piece of the OptiX and driver work.
    /// **The card's driver line**: *NVIDIA driver 595.91.07 · CUDA 13.2*, the
    /// two figures the driver's own and drawn as values.
    pub const NVIDIA_DRIVER: &str = "NVIDIA driver";
    pub const CUDA_AFTER_THE_DRIVER: &str = "\u{b7} CUDA";
    /// **The amber warning's first sentence, around its values**: *This
    /// server's NVIDIA driver, 550.163.01 (CUDA 12.4), is too old for OptiX in
    /// Blender 5.2 and later.*
    pub const THIS_SERVERS_DRIVER: &str = "This server's NVIDIA driver,";
    pub const CUDA: &str = "CUDA";
    pub const IS_TOO_OLD_FOR_OPTIX_IN: &str = "is too old for OptiX in Blender";
    pub const AND_LATER: &str = "and later.";
    /// **Its second sentence**: on a container, and where a setup tried and
    /// did not get the driver there.
    pub const SMOOZY_CANT_UPDATE_IT_HERE: &str = "It can't be updated here.";
    pub const SMOOZY_COULDNT_UPDATE_IT: &str = "It could not be updated.";
    /// **The Blender table's tag**, drawn like *below 4.5*.
    pub const DRIVER_TOO_OLD_FOR_OPTIX: &str = "driver too old for OptiX";

    // ── `Server Setup` W4's setup fault
    pub const THE_LAST_JOB_ASKED_FOR_THE_GPU: &str =
        "The last job asked for the GPU and rendered on the CPU.";
    pub const SET_UP_AGAIN_TO_REINSTALL: &str = "Set up again to reinstall the GPU drivers.";

    // ── The action — `Design/Server Setup § This server`
    pub const SET_UP_THIS_SERVER: &str = "Set up this server";
    /// **The row's own words for the row's own act** — `Design/Server Setup §
    /// This server`, the card where an add's connection was not made (the
    /// Creator's note 14 on the test copy). One fact, one wording.
    pub const CONNECT_AND_SET_UP: &str = crate::surface::servers::words::CONNECT_AND_SET_UP;
    pub const SET_UP_AGAIN: &str = "Set up again";

    // ── `Design/Server Setup § A second setup while one is running`
    pub const ONE_SETUP_AT_A_TIME: &str =
        "Other actions on this server are queued.";

    // ── `Design/Server Setup § When a step had nothing to do`
    pub const NOTHING_WAS_INSTALLED: &str = "Nothing installed or changed.";
    pub const TELEGRAM_AND_THE_REST_IS_NOT_RUN: &str = "Only Telegram reporting is installed.";

    // ── The card where no reading of the machine has ever existed. **The words
    // are Design's and are owed** (§24): `Design/Server Setup § Cut off
    // part-way` settles that such a server shows nothing about a setup and does
    // not say what stands in its place, and Rule 8 makes an empty card Smoozy
    // going quiet.
    pub const NEVER_READ: &str = "Never connected.";
    // ── Connected, and no setup run found to read — 2026-09-21, **not drawn**.
    pub const NO_SETUP_WAS_READ: &str = "No setup found";
    pub const NO_SETUP_WAS_READ_WHY: &str =
        "No setup since Smoozy opened.";
    pub const NEVER_READ_WHY: &str = "Connect to see its setup.";

    // ── `Server Setup` Y4's boundary names. **Six are the drawing's own
    // words**, read off the mockup's two step lists at 6469–6476 and 6733–6740.
    pub const READINGS: &str = "Check what is installed";
    pub const LIBRARIES: &str = "Check system libraries";
    pub const DRIVER_PLACED: &str = "Install the GPU driver";
    pub const RESTART_ISSUED: &str = "Restart the server";
    pub const RESTARTING: &str = "Restarting the server";
    pub const CARD_ANSWERING: &str = "Check the driver works";
    pub const TELEGRAM_STEP: &str = "Install Telegram reporting";
    // **Four the drawing does not carry, and they are ruled here.** See
    // [`super::the_boundarys_name`].
    /// **The download's step, named for the build** — `Design/Server Setup §
    /// A long step`: *Download Blender 5.2.2*, the build following as the list
    /// names it, in every state where this copy knows the build. *Fetch the
    /// Blender build* until 2026-10-18 (the build runner's ruling on the
    /// Creator's fourteenth notes, note 1): *Download* is the standard word.
    pub const FETCHED: &str = "Download Blender";
    pub const TOOLING: &str = "Install the unpack tool";
    /// **The tooling step while its unpack runs and says how far it has got**
    /// — `Design/Server Setup § A long step`: *Unpack Blender 5.2.2*, with
    /// *41%*. Added 2026-10-18, piece U; see
    /// [`super::the_builds_steps_are_named_and_counted`] for why it is the
    /// tooling step's name while the unpack runs and not a step of its own.
    pub const UNPACK_BLENDER: &str = "Unpack Blender";
    /// **The unit of the download's figure** — *214 of 352 MB*.
    pub const MB: &str = "MB";
    pub const BLENDER_STEP: &str = "Install Blender";
    /// **`Server Setup` B15's removal, as a step of its own run.** Ruled here
    /// under the Creator's standing delegation of 2026-08-31, in the shape of
    /// the steps above it. The 38's piece 14c.
    pub const REMOVED_STEP: &str = "Remove Blender";
    /// **The run's last step, which the panel drew nothing for at all.** Added
    /// 2026-09-23, step 10c's fifth pass on Vast: a Blender install whose final
    /// check failed drew every step before it as done under the heading *Setup
    /// stopped part-way*, with no line anywhere for the step that failed — so
    /// the panel said a run that stopped had finished everything. **Ruled here
    /// under the Creator's standing delegation of 2026-08-31**, in the same
    /// imperative shape as the nine above it and in the words the offer box
    /// already uses for this step.
    pub const DONE_STEP: &str = "Run the final check";

    // ── The marks against a step
    pub const DONE: &str = "done";
    pub const NOTHING_TO_DO: &str = "nothing to do";
    pub const NOT_CHECKED_NO_BLENDER: &str = "not checked — no Blender installed";
    pub const NOT_STARTED: &str = "not started";
    pub const DID_NOT_FINISH: &str = "failed";
    pub const WHAT_THE_SERVER_SAID: &str = "What the server said";

    // ── One plain sentence per step that did not finish. **One is the
    // drawing's own**; the other eight are ruled — see
    // [`super::the_boundarys_failure_sentence`].
    pub const FAILED_READINGS: &str = "Could not check what is installed.";
    pub const FAILED_FETCH: &str = "Could not download Blender.";
    /// **The fetch's failure, naming the build** — `Design/Server Setup §
    /// Installing`: *Could not download Blender 5.3.0 Alpha.* The
    /// build's name follows as a value. The Creator's note 7 on 0.1.7.
    pub const COULD_NOT_FETCH_BLENDER: &str = "Could not download Blender";
    pub const FAILED_TOOLING: &str = "Could not install the unpack tool.";
    pub const BUILD_COULD_NOT_BE_SENT: &str = "Your build was not uploaded.";
    pub const UPLOAD_YOUR_BUILD: &str = "Upload your build";
    /// **The unpack itself, told apart from the tool's install.** Added
    /// 2026-09-22 — see `setting_up::TheRunsEnding::TheUnpackFailed`. It is also
    /// the run panel's sentence for a tooling step that did not finish, except
    /// where the reading shows the tool's own install was the act in hand
    /// ([`super::the_tools_own_install_is_named`], 2026-10-25): the record names
    /// the step and not which of its two acts failed.
    pub const FAILED_UNPACK: &str = "Could not unpack Blender.";
    pub const FAILED_LIBRARIES: &str = "Could not install system libraries.";
    pub const FAILED_DRIVER: &str = "Could not install the GPU driver.";
    pub const FAILED_RESTART: &str = "The server did not restart.";
    pub const FAILED_CARD_CHECK: &str = "The GPU driver did not respond after the restart.";
    /// **A recovery Smoozy opened and did not carry on**, said as what Smoozy
    /// did rather than as a step's failure. Added 2026-09-27, fix piece 6.
    pub const FAILED_NOT_CARRIED_ON: &str = "Cannot resume mid-step.";
    pub const FAILED_BLENDER: &str = "Could not install Blender.";
    /// The removal's own, ruled in the same shape. The 38's piece 14c.
    pub const FAILED_REMOVAL: &str = "Could not remove Blender.";
    pub const FAILED_TELEGRAM: &str = "Could not install Telegram reporting.";
    pub const THE_CONNECTION_DROPPED_HERE: &str = "connection lost";
    pub const STOPPED_HERE: &str = "stopped";
    pub const WAITING_FOR_THE_RESTART: &str = "waiting";

    // ── The Blender table — `Design/Server Setup § Blender`
    pub const VERSION: &str = "Version";
    pub const WHERE: &str = "Where";
    pub const REMOVE: &str = "Remove";
    pub const YOUR_BUILD: &str = "your build";
    pub const NOT_INSTALLED_BY_SMOOZY: &str = "not installed by Smoozy";
    pub const BELOW_THE_FLOOR: &str = "below 4.5";
    pub const INSTALL_BLENDER: &str = "Install Blender…";
    pub const NO_BLENDER_READING: &str = "Not read yet.";

    // ── The install box — `Design/Server Setup § Installing`
    pub const INSTALL_BLENDER_TITLE: &str = "Install Blender";
    pub const FROM_BLENDERS_OWN: &str = "From Blender's official downloads.";
    pub const LATEST_STABLE: &str = "Latest stable";
    pub const RELEASE: &str = "Release";
    pub const LONG_TERM_SUPPORT: &str = "Long-term support";
    // **Blender's own word for each unfinished kind** — `Design/Server Setup §
    // Installing`, the Creator's note 5 on 0.1.7. *Unstable* is gone with the
    // shop-front's unfinished channels.
    pub const ALPHA: &str = "Alpha";
    pub const BETA: &str = "Beta";
    pub const RELEASE_CANDIDATE: &str = "Release candidate";
    pub const NOT_INSTALLED: &str = "not installed";
    pub const ON_THIS_SERVER: &str = "on this server";
    pub const INSTALLED: &str = "installed";
    /// **The heading under the rule, with nothing explaining it** —
    /// `Design/Server Setup § Installing`, Creator-stated 2026-09-27:
    /// *"everybody knows they are just called experimental builds."*
    pub const EXPERIMENTAL_BUILDS: &str = "Experimental builds";
    pub const OR_CHOOSE_A_BUILD: &str = "Or";
    pub const CHOOSE_A_BUILD: &str = "choose a build on your computer";
    /// The system file picker's title for B7's pick. Added 2026-10-09.
    pub const CHOOSE_A_BLENDER_BUILD: &str = "Choose a Blender build";
    pub const WHAT_SMOOZY_CHECKS: &str =
        "— for a Blender you compiled yourself. The upload and version are checked; its \
         contents and origin are not.";
    pub const CANCEL: &str = "Cancel";
    pub const INSTALL: &str = "Install";
    /// **The button's count where several are ticked** — `Design/Server Setup
    /// § Installing`: *Install 5.2.2*, *Install 3 versions*. The Creator's
    /// third notes, note 2, 2026-10-06.
    pub const VERSIONS: &str = "versions";
    pub const THE_LIST_COULD_NOT_BE_READ: &str = "Could not read Blender's download list.";
    pub const ENTRIES_UNREAD: &str = "entries could not be read";

    // ── The unpack-tool warning — `Server Setup` W3's own row
    pub const UNPACK_TOOL_FIRST: &str = "This build needs the unpack tool first";
    pub const THERE_IS_WORK: &str = "This server is busy.";
    pub const NOT_NOW: &str = "Not now";
    pub const INSTALL_ANYWAY: &str = "Install anyway";

    // ── The ready action's warning — `Server Setup` W3
    pub const THIS_SERVER_HAS_WORK: &str = "This server is busy";
    pub const SETTING_UP_RESTARTS: &str = "Setup installs the GPU driver and restarts the server.";
    pub const SET_UP_ANYWAY: &str = "Set up anyway";

    // ── What each live job is told about itself — `Design/Server Setup § A long
    // step`, three outcomes and never one sentence covering all three.
    pub const RENDERING_AND_A_RESTART: &str = "rendering — stops, then restarts";
    pub const WAITING_AND_A_RESTART: &str = "waiting — delayed";
    pub const UPLOADING_AND_A_RESTART: &str = "uploading — interrupted, then resumes";
    pub const RENDERING: &str = "rendering";
    pub const WAITING: &str = "waiting";
    pub const UPLOADING: &str = "uploading";

    // ── Pointing at a build — `Design/Server Setup § Pointing at a build on
    // your own computer`
    pub const INSTALLING_YOUR_BUILD: &str = "Installing your build";
    pub const SENDING: &str = "Uploading —";
    pub const OF: &str = "of";
    pub const IT_SHOWS_IN: &str = "It shows in";
    pub const TRANSFERS: &str = "Transfers";
    pub const WITH_YOUR_OTHER_UPLOADS: &str = "with your other uploads.";
    pub const STOP: &str = "Stop";

    // ── Below the floor — `Design/Server Setup § The below-4.5 warning`
    pub const THIS_BUILD_IS_BLENDER: &str = "This build is Blender";
    pub const SMOOZY_SUPPORTS: &str = "Supported:";
    pub const AND_ABOVE: &str = "and above.";
    pub const NOTHING_IS_WRITTEN_FOR: &str = "Not built for";
    pub const AND_SMOOZY_CANNOT_HELP: &str = "; jobs on it are unsupported.";
    pub const IT_WILL_APPEAR: &str = "It can be picked for a job, but never automatically.";
    pub const DONT_INSTALL_IT: &str = "Don't install it";

    // ── The run, on the card — `Design/Server Setup § A long step`. **On the
    // card and never in a box** since 2026-10-02 (the Creator's note 4 on
    // 0.1.7): *Setting up* is the card's own state line, as on `sugoing` and
    // `burun`, and *Cancel setup* its one control.
    pub const INSTALLING_BLENDER: &str = "Installing Blender";
    pub const ABANDONING_SETUP: &str = "Abandoning setup";
    pub const CANCEL_SETUP: &str = "Cancel setup";
    pub const CANCEL_SETUP_LEAVES: &str = "Cancel setup keeps what is installed.";
    pub const THE_RESTART_HAS_BEGUN: &str = "The restart has begun; Cancel only stops the wait.";
    // **`Server Setup` T6a, carried into the wording under Rule 10.** The
    // drawing writes *"The render that was running stopped, and restarts by
    // itself."* T6a bounds that exactly: **the stopping half is unconditional
    // and is T7's; the restarting half is a consequent of the setup
    // finishing**, and a user who closes the panel and never comes back leaves
    // the run undriven and the job never re-run. **What the architecture
    // supplies the panel is the stop as fact and the re-run as consequent**,
    // and it records that the drafted sentence states the consequent
    // unconditionally rather than repairing a document it does not own.
    pub const THE_RENDER_STOPPED: &str =
        "The running render stopped; it restarts when setup finishes.";

    // ── Cut off part-way, and the step that failed
    // **Lower case, as the drawing's `bufail` and `bustep` write them**: on the
    // card the moment sits after the state line, where *set up*, *checked* and
    // *started* already sit.
    pub const THE_CONNECTION_DROPPED_AT: &str = "connection lost at";
    pub const A_STEP_DID_NOT_FINISH_AT: &str = "a step failed at";
    pub const THE_RUN_WAS_STOPPED_AT: &str = "stopped at";
    /// **The closing paragraph, word for word from `Design/Server Setup § The
    /// two ways out`**, which writes it out in full and says it is the same
    /// paragraph on both panels.
    pub const THE_CLOSING_PARAGRAPH_ONE: &str = "Server checked just now. Set up again is safe.";
    pub const THE_CLOSING_PARAGRAPH_TWO: &str =
        "ends the setup unfinished. Neither removes what is installed.";
    pub const THE_RETRY_PARAGRAPH_ONE: &str = "Server checked just now.";
    pub const THE_RETRY_PARAGRAPH_TWO: &str = "keeps what is installed.";
    pub const ABANDON_SETUP: &str = "Abandon setup";
    pub const CLOSE: &str = "Close";

    // ── What the server printed, beside the steps — `Design/Server Setup § A
    // long step`, added 2026-10-18 on the Creator's thirteenth notes, note 7.
    // **The control names whose words they are**, as *Blender output* does on
    // a rendering entry. The two empty-stream sentences are M10's *collected,
    // and the stream was empty*, worded to the server as the render's are
    // worded to the render.
    pub const SERVER_OUTPUT: &str = "Server output";
    pub const THE_SERVER_PRINTED_NO_OUTPUT: &str = "No output.";
    pub const THE_SERVER_WROTE_NOTHING_HERE: &str = "No output.";
    /// **Under a tab's words, where one step printed more than Smoozy could
    /// bring back from the server at once** — `Design/Server Setup § A long
    /// step`, *Every step's words, not only an install's*, added 2026-10-24 on
    /// the Creator's ruling on the sixteenth notes, note 1. **Not among the
    /// printed words and never copied.**
    pub const SOME_OF_WHAT_THE_SERVER_PRINTED_COULD_NOT_BE_SHOWN: &str =
        "Output truncated.";
    /// **The panel's own words, the ones every raw-text panel uses.**
    pub const THE_PANELS_NOTE: &str = crate::surface::the_machines_own_words::THE_PANELS_NOTE;
    pub const COPY: &str = crate::surface::the_machines_own_words::COPY;

    // ── Removing a Blender — `Design/Server Setup § Removing a Blender`
    pub const REMOVE_BLENDER: &str = "Remove Blender";
    pub const ONE_JOB_WAS_SUBMITTED: &str = "One job uses it.";
    pub const JOBS_WERE_SUBMITTED: &str = "jobs use it.";
    pub const RENDERING_WILL_FINISH: &str = "rendering now — will finish";
    pub const RENDERING_UNKNOWN: &str = "rendering now — effect unknown";
    pub const WAITING_NOTHING_TO_RENDER: &str = "waiting — will have no Blender";
    pub const SETTINGS_FOLDER_STAYS: &str = "Its settings and add-ons are kept.";
    pub const REMOVE_ANYWAY: &str = "Remove anyway";
    pub const THE_MANAGER_IS_THE_ROUTE: &str =
        "Installed by the package manager. Remove it there.";

    // ── Admin access — `Design/Server Setup § Admin access`
    pub const NEEDS_ADMIN_ACCESS: &str = "This step needs admin access";
    pub const WAS_REFUSED: &str = "was refused: permission denied.";
    pub const THE_SYSTEM_WOULD_NOT_HAND_IT_BACK: &str = "Access to the saved password was denied.";
    pub const PASSWORD: &str = "Password";
    pub const DO_NOT_REMEMBER_IT: &str = "Don't save";
    pub const DO_NOT_REMEMBER_IT_SUB: &str = "used once";
    pub const WHILE_CONNECTED: &str = "While connected";
    pub const WHILE_CONNECTED_SUB: &str = "asked once per connection";
    pub const REMEMBER_IT: &str = "Save";
    pub const REMEMBER_IT_SUB: &str = "until cleared on this server's Edit page";
    pub const TRY_AGAIN: &str = "Retry";

    // ── The one thing every act here can honestly say today. **The sentence
    // Files, the check, the render settings and the Queue all say, shared**
    // (must-do 19). A sentence about this copy holding no connection is not
    // something a user can act on (Rule 9), which is why it says what is not
    // built instead. **The words changed on 2026-09-18** by the build runner's
    // ruling: opening a connection had been built, so the old wording named the
    // wrong absent thing.
    pub const THE_ACT_IS_NOT_BUILT_YET: &str = crate::surface::files::A_CONNECTION_IS_NOT_BUILT;

    // ── What a driven setup says when it ends — step 10c piece 9.
    // **Standard software words, and vague is the real failure** (Rule 9): each
    // one names what happened rather than how it was reached.
    pub const THE_SERVER_IS_READY: &str = "Ready to render.";
    /// **A setup that reached *done* with no Blender installed by it.** Added
    /// 2026-09-22: step 10c's third pass on Vast showed *ready to render* on a
    /// machine holding no Blender at all. **Whether the machine holds one from
    /// an earlier install is not what this run read**, so the sentence says
    /// what the run did and no more (Rule 8), and the Blender section below it
    /// says what is on the server.
    pub const THE_SERVER_IS_SET_UP: &str = "Setup complete.";
    pub const SETUP_STOPPED: &str = "Setup stopped.";
    // ── `Server Setup` T6's second ending: the restart went out and the user
    // stopped waiting for the machine. **That clears nothing** — the mark
    // stands, the machine-level state stands, the reconnect policy goes on
    // running, and the setup is picked up when the server answers. So the
    // sentence says what is still true rather than what has stopped.
    pub const THE_SERVER_IS_RESTARTING: &str =
        "The server is restarting for the GPU driver. Setup resumes when it reconnects.";
    // ── `Server Setup` T6's third ending: the reconnect policy gave up.
    // **Smoozy says only what it knows** — the connection is down and could not
    // be restored — **with no claim to know why, because a machine still
    // starting cannot be told from one that will never answer.** The setup is
    // read back off the machine's own record on the next connection
    // (`Server Setup` S1, S2).
    pub const THE_SERVER_DID_NOT_COME_BACK: &str =
        "Could not reconnect after the restart.";
    pub const NOT_A_MACHINE_SMOOZY_CAN_SET_UP: &str = "Unsupported operating system.";
    pub const NOTHING_WAS_CHANGED_ON_IT: &str = "Nothing was changed.";
    pub const NO_SERVER_IS_CONNECTED: &str = "Connect to this server first.";
    pub const SETUP_IS_ALREADY_RUNNING: &str =
        "A setup is already running on this server.";

    // ── What a stopped setup names, 2026-09-21. **Each sentence says what
    // Smoozy knows and nothing it does not** (Rule 8): the machine's own words
    // go behind *What the server said*, whole, and never into the sentence.
    // **Never a debugging rendering** — that is what the first pass on a real
    // rental showed in this box.
    pub const COULD_NOT_MAKE_ITS_FOLDER: &str = "Could not create Smoozy's folder on the server.";
    pub const COULD_NOT_RECORD_ITS_FOLDER: &str = "Could not save Smoozy's folder location.";
    pub const COULD_NOT_GET_ITS_PROGRAM: &str =
        "Could not download or verify Smoozy's server software. Nothing was installed.";
    pub const COULD_NOT_INSTALL_ITS_PROGRAM: &str = "Could not install Smoozy's server software.";
    pub const ITS_PROGRAM_WAS_PUT_BACK: &str =
        "Could not install Smoozy's server software. Rolled back.";
    pub const PUTTING_IT_BACK_DID_NOT_FINISH: &str =
        "Could not install Smoozy's server software. Rollback incomplete.";
    pub const WORK_STILL_NEEDS_ITS_PROGRAM: &str =
        "Smoozy's server software not replaced: jobs on the server still use it.";
    /// **Held jobs of an earlier setup, which no new setup can answer** —
    /// `Design/Server Setup § When work on the server holds the setup back`,
    /// word for word, with its singular form. Added 2026-10-01, step 10c's
    /// thirteenth-pass fix piece 2: the thirteenth pass was told
    /// [`WORK_STILL_NEEDS_ITS_PROGRAM`] twice for a reason that would never go.
    pub fn held_jobs_of_an_earlier_setup(how_many: usize) -> String {
        use crate::typography::a_figure_with_its_unit;
        if how_many == 1 {
            format!(
                "Smoozy's server software cannot be replaced: {} from an earlier setup needs an \
                 answer this computer cannot give. A new server will not have it.",
                a_figure_with_its_unit(1, "job")
            )
        } else {
            format!(
                "Smoozy's server software cannot be replaced: {} from an earlier setup need an \
                 answer this computer cannot give. A new server will not have them.",
                a_figure_with_its_unit(how_many, "jobs")
            )
        }
    }
    pub const ITS_PROGRAM_IS_BEING_INSTALLED: &str =
        "Smoozy's server software is already being installed.";
    pub const THE_CONNECTION_ENDED: &str = "Connection lost.";
    pub const THE_RUN_WOULD_NOT_START: &str = "Setup did not start on the server.";
    /// **Why a run was refused, and what to do, where the machine said why.**
    /// Added 2026-09-21, step 10c: the second pass on Hyperstack was told only
    /// [`THE_RUN_WOULD_NOT_START`] for a refusal whose reason the machine had
    /// given. **The words are `Design/Server Setup § Cut off part-way`'s** —
    /// *Setup stopped part-way* is that section's row line and *Set up again*
    /// its control — and **Rule 8's**: nothing else runs on the server while the
    /// unfinished setup stands (`Rented-server runtime` A11), which is why the
    /// thing asked for did not start.
    pub const A_SETUP_STOPPED_PART_WAY: &str =
        "A setup stopped part-way. Set up again to finish it.";
    /// **A setup run again that stopped genuinely inside a step** — the step
    /// is named before it, in the drawing's own shape (*"Installing the
    /// GPU driver …"*). *Abandon setup* is `Design/Server Setup § Cut off
    /// part-way`'s second way out, and it says what the check found.
    /// *(Until 2026-09-27 this named the graphics driver's install whatever
    /// step the record named, and it fired on a setup whose driver install had
    /// finished — step 10c's seventh pass on Hyperstack, fault 3.)*
    pub const STOPPED_INSIDE_THE_STEP: &str =
        "stopped mid-step and cannot resume. Abandon setup, then set up again.";
    /// **The runner's ruling 1 of 2026-09-27: a restart command the server
    /// refused while it stayed connected.** The machine's words go behind
    /// *What the server said*, whole.
    pub const THE_RESTART_WAS_REFUSED: &str =
        "The server refused the restart. GPU driver install incomplete.";
    pub const FAILED_FINAL_CHECK: &str = "Final check failed.";
    /// **Smoozy's program on the server stopped answering while a step was
    /// under way.** Added 2026-09-22, step 10c's fourth pass on Vast: a run
    /// that ended that way was told *The server did not pass Smoozy's final
    /// check* when nothing had been checked. **The sentence says what Smoozy
    /// knows and no more** (Rule 8) — the step that was running is named
    /// before it, in the drawing's own shape.
    /// **The server ended the run while Smoozy was driving it** — the run's
    /// own recorded ending, `Design/Server Setup § Cut off part-way`, told for
    /// the step that was under way rather than as that step failing. Added
    /// 2026-09-27, step 10c's eighth-pass fix piece 1: the eighth pass on
    /// Hyperstack was told *The server could not install the graphics driver.*
    /// when the driver had installed and the server had ended the setup.
    /// **Ruled here under the Creator's standing delegation of 2026-08-31**,
    /// in the shape of the two sentences beside it; **it says what Smoozy
    /// knows and no cause** (Rule 8), and the machine's words go behind *What
    /// the server said*.
    pub const THE_SERVER_ENDED_THE_SETUP: &str =
        "stopped: the server ended the setup.";
    pub const THE_SERVER_STOPPED_ANSWERING: &str =
        "stopped: Smoozy's server software stopped responding.";
    // ── *Abandon setup* — `Server Setup` S6's second clear, 2026-09-22.
    // **`Design/Server Setup § The two ways out`**: the run goes to the server,
    // makes the setup's final check, says what that check found, and only then
    // gives the unfinished setup up — so the answer says what the check found,
    // with the server's own words behind *What the server said* where it did not
    // pass.
    pub const ABANDONED_AND_THE_CHECK_PASSED: &str =
        "Setup abandoned. Final check passed.";
    pub const ABANDONED_AND_THE_CHECK_DID_NOT_PASS: &str = "Setup abandoned. Final check failed.";
    pub const NOTHING_TO_ABANDON: &str = "No unfinished setup to abandon.";
    pub const THE_ABANDON_DID_NOT_FINISH: &str = "Could not abandon the setup.";
    pub const NEEDS_ADMIN_ACCESS_AND_NONE_WAS_GIVEN: &str = "needs admin access; no password given.";
    // ── A Blender install from the Blender section, 2026-09-21.
    pub const BLENDER_IS_INSTALLED: &str = "installed.";
    pub const INSTALL_STOPPED: &str = "Install stopped.";
    /// **What the card's line names where an install stopped the ones still
    /// waiting** — `Design/Server Setup § Installing`: *Not installed: Blender
    /// 5.2.2, Blender 4.5.14.* The Creator's third notes, note 2.
    pub const NOT_INSTALLED_THESE: &str = "Not installed:";
    /// **After the names, where Smoozy closed before they were begun** —
    /// `Design/Server Setup § Installing`: *Not installed: Blender 5.2.2,
    /// Blender 4.5.14. Smoozy closed before they started.* One for several,
    /// one for a single version. Step 10c's owed piece 4, 2026-10-16.
    pub const SMOOZY_CLOSED_BEFORE_THEY_STARTED: &str = "Smoozy closed before they started.";
    pub const SMOOZY_CLOSED_BEFORE_IT_STARTED: &str = "Smoozy closed before it started.";
    // ── Asking for more while a setup runs — `Design/Server Setup § Asking
    // for more while a setup runs`, the nineteenth notes, note 2 (2026-10-27).
    /// **The heading over the acts waiting their turn**, on the running card.
    pub const THE_WAITING_HEADING: &str = "Queued";
    /// **Each act waiting, named as the act it is**: *Install Blender 4.5.15*
    /// (with [`INSTALL`] and [`BLENDER`]), *Remove Blender 5.2.2* (with
    /// [`REMOVE`]), *Install your build blender-4.2-linux.tar.xz*, *Set up
    /// again* ([`SET_UP_AGAIN`]) and this.
    pub const INSTALL_YOUR_BUILD: &str = "Install your build";
    pub const INSTALL_TELEGRAM_REPORTING: &str = "Install Telegram reporting";
    /// **What the card's line names where a run stopped the acts behind it**:
    /// *Not started: Remove Blender 5.2.2, Install Blender 4.5.15.*
    pub const NOT_STARTED_THESE: &str = "Not started:";
    /// **After the names, where the run in hand restarted the server.**
    pub const THE_SERVER_RESTARTED_FIRST: &str = "The server restarted first.";
    /// **After the names, where Smoozy closed before they were begun** and
    /// anything but installs from the list was waiting.
    pub const SMOOZY_CLOSED_FIRST: &str = "Smoozy closed first.";
    /// **A user's own build that was not installed, named as theirs by its file
    /// name** — `Design/Server Setup § Pointing at a build on your own
    /// computer`: *Your build blender-4.2-linux.tar.xz was not installed.*,
    /// never *Blender* and the file name, since a file name is not a version.
    /// The build runner's ruling on these words, step 10c's owed piece 5,
    /// 2026-10-16.
    pub const YOUR_BUILD_NAMED: &str = "Your build";
    pub const WAS_NOT_INSTALLED: &str = "was not installed.";
    /// **Not drawn** — `Design/Server Setup` draws no state for a Blender asked
    /// of a server Smoozy has not set up, and the account names that as a hole.
    /// Rule 8 makes a silence the fault, so the sentence says what is true.
    pub const SET_UP_THIS_SERVER_FIRST: &str = "Set up this server first.";
    pub const NOTHING_WAS_INSTALLED_ON_IT: &str = "Nothing was installed.";
    // ── The step an offer names, in the drawing's own shape: *"Installing the
    // GPU driver was refused: …"* The other three are ruled here under the
    // Creator's standing delegation of 2026-08-31, each naming what the step
    // does in the words its failure sentence already uses.
    pub const DOING_THE_TOOLING: &str = "Installing the unpack tool";
    pub const DOING_THE_LIBRARIES: &str = "Installing system libraries";
    pub const DOING_THE_DRIVER: &str = "Installing the GPU driver";
    pub const DOING_THE_RESTART: &str = "Restarting the server";
    pub const DOING_A_STEP: &str = "This step";
    // ── The other five R1 steps, in the same shape, added 2026-09-22 under the
    // same standing delegation. **A box that says which step stopped needs a
    // name for every step**, and until this the five answered *This step*.
    pub const DOING_THE_READINGS: &str = "Reading the server";
    pub const DOING_THE_FETCH: &str = "Downloading Blender";
    pub const DOING_THE_PUBLICATION: &str = "Installing Blender";
    pub const DOING_THE_REMOVAL: &str = "Removing Blender";
    pub const DOING_THE_TELEGRAM: &str = "Installing Telegram reporting";
    pub const DOING_THE_FINAL_CHECK: &str = "The final check";

    // ── Admin access, where the store's standing could not be read — must-do
    // 32. The box offers what a store never proven is offered
    // (`Security and licence` S25, S36) and says why.
    #[cfg(target_os = "macos")]
    pub const THE_STANDING_COULD_NOT_BE_READ: &str = "Saving unavailable: could not check the keychain.";
    #[cfg(not(target_os = "macos"))]
    pub const THE_STANDING_COULD_NOT_BE_READ: &str =
        "Saving unavailable: could not check the credential store.";
}

// ════════════════════════════════════════════════════════════════════════════
// What one reading of a machine carries
// ════════════════════════════════════════════════════════════════════════════

/// **A reading of a machine's setup, taken over a connection.**
///
/// **Every field is a reading and none of them is remembered** — `Server Setup`
/// S1, S2 and Y6a: read fresh on the connection the user opens, with no second
/// home on the desktop. [`smoozy_server_setup::run::THE_DESKTOP_STORES_NONE_OF_THE_CARD`]
/// is the clause said as a constant, and this type is a value that lives for
/// the length of one answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ASetUpReading {
    /// **`Server Setup` Y6b: the card reads the ready action's own run record**
    /// and a run of any other kind replaces nothing of it. Every setup record
    /// the machine holds is handed in; which one the card reads is that
    /// clause's own answer.
    pub the_records: Vec<TheSetupRecord>,
    /// **`Server Setup` Y6b: the Telegram line is the role's own reading**, and
    /// never a line of the run.
    pub telegram_reporting_is_installed: bool,
    /// **`Server Setup` G2: whether Telegram is switched on at all**, which is
    /// what decides whether the card owes an offer.
    pub telegram_is_switched_on: bool,
    /// **A setup running now**, with the boundary it has reached.
    pub a_setup_is_running: Option<TheSetupRecord>,
    /// **`Server Setup` S1: which step was in flight**, where the reading says.
    pub the_step_in_flight: Option<Boundary>,
    /// **`Server Setup` W3a's three questions, over every finished job the
    /// machine still holds.**
    pub the_finished_jobs: WhatTheThreeQuestionsAnswered,
    /// **`Server Setup` W4's three conditions**, for the most recent finished
    /// record that carries a device line.
    pub the_device_reading: Option<ADeviceReading>,
    /// **`Server Setup` B10's search**: every build the machine holds.
    pub the_builds: Vec<ABuildAsRead>,
    /// **`Server Setup` R4 and W1: whether the library check had a subject at
    /// all.** Where the machine held no Blender and the run fetched none, the
    /// check did not run, and *ready* claims nothing about it. **It is a
    /// reading of the run and is not derivable from the boundary list** — a
    /// boundary is written when its step completed, whatever the step found.
    pub the_library_half_was_read: bool,
    /// **`Server Setup` B11: folders the search could not enter.**
    pub folders_the_search_could_not_enter: usize,
    /// Which machine's clock every moment in this reading is on.
    pub the_clock: String,
    /// **`Server Setup` W1: Smoozy's program was placed before the run the
    /// card reads**, in the same press. The placement is not one of the run's
    /// boundaries, so the record alone cannot say it. Step 10c's twelfth-pass
    /// fix piece 6.
    pub a_placement_came_first: bool,
}

/// **`Server Setup` W4's three conditions, as the machine's own records carry
/// them.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ADeviceReading {
    pub outcome: RecordedOutcome,
    /// **`None` where the machine could not read what the job asked for** —
    /// and then nothing is concluded from the line (`Server Setup` W4: the
    /// fault needs all three conditions). The 38's piece 6.
    pub asked_for: Option<WhatTheManifestAskedFor>,
    pub line: Option<WhatTheLineNames>,
    /// **The line's own words**, which is what *Last job rendered on* reads. A
    /// value: it is the machine's own text and X7 forbids the point touching
    /// it.
    pub the_lines_words: String,
}

/// One build the search found — `Server Setup` B10, B12 and B8a.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ABuildAsRead {
    /// **B13: a build is identified by its executable's path.** A value.
    pub path: String,
    /// **B1: what the build answered when it was asked.**
    pub version: Version,
    /// **B1's further mark, taken from the same line.**
    pub long_term_support: bool,
    /// **B8a: which of three ways it got onto the machine**, read from the
    /// record of the act that placed it and never off a name.
    ///
    /// **`None` is *no record of the placing act was read at all***, which is
    /// not one of B8a's three and is not drawn as one. Added 2026-09-23: the
    /// search that feeds this table reads no install record anywhere, and each
    /// of the three answers is an assertion about an act — so a row composed
    /// from the search alone carries no tag rather than the wrong one.
    pub origin: Option<HowItGotThere>,
    /// **The package the machine's own software manager holds this build
    /// under**, where it holds one at all.
    ///
    /// **It carries B15's route as well as the name**, and that is deliberate:
    /// *"a path under the system package areas is the manager's"*, so the one
    /// reading decides both which route a removal takes and whether this module
    /// has a carriage for it. **A build with no manager package is the plain
    /// folder deletion.**
    pub the_managers_package: Option<String>,
}

/// **`Server Setup` B8a: three answers rather than two.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowItGotThere {
    /// Smoozy installed it from the list. **No tag** — it is the ordinary case.
    SmoozyInstalledItFromTheList,
    /// The user pointed at it. *your build*.
    TheUserPointedAtIt,
    /// **No install record names this path at all** — the machine's own
    /// software manager put it there. *not installed by Smoozy*.
    NoInstallRecordNamesIt,
}

/// **What the machine said, or that nothing was ever asked.**
///
/// **A read of a machine answers two ways and never one** — L8a. The reading
/// and *nothing has ever been read* are different news, and a tab that drew the
/// second as an empty version of the first would be Smoozy going quiet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheMachineSaid {
    /// **Smoozy has never connected to this server, so no reading of its setup
    /// has ever existed.** `Design/Server Setup § Cut off part-way` names this
    /// as the one server that shows nothing about a setup.
    NothingHasEverBeenReadFromIt,
    /// A reading, taken over a connection.
    AReading(Box<ASetUpReading>),
    /// **One setup run's record, read fresh over the connection that stands,
    /// and nothing else of the machine** — `Server Setup` S1, S2 and Y6a's
    /// reading of the run, without `Server Setup` B10's search, `Server Setup` W3a's finished
    /// jobs or the Telegram role's state, none of which is taken. **So the card
    /// is drawn from the run alone and claims nothing about the rest**, and the
    /// Blender table says no reading of it exists. Added 2026-09-21.
    ASetupRunAlone(Box<crate::setting_up::TheSetupAsRead>),
    /// **One setup run's record and the rest of the card's reading beside
    /// it** — `Server Setup` W3a's three answers with W4's line and ask, and
    /// the Telegram role's own state (`Server Setup` Y6b), all read fresh over
    /// the connection that stands. **The card is the run's, with the lines the
    /// run alone could not give.** The 38's piece 6.
    ASetupRunAndTheMachine(
        Box<crate::setting_up::TheSetupAsRead>,
        Box<crate::setting_up::TheCardsReading>,
    ),
    /// **What was last read from this server in this run, while no connection
    /// stands** — `Server Setup` Y6a: *"With no connection the card is
    /// last-known, greyed and stamped"*, and never blank. **Held in memory for
    /// the run and written nowhere** (the 38's map, hole 7: the desktop stores
    /// none of it), stamped with this computer's own moment of reading.
    LastSeen(Box<WhatTheMachineSaid>, std::time::SystemTime),
    /// **Connected, and no setup run was found to read** — no machine-level
    /// state standing, no run of the ready action's kind driven by this copy,
    /// and **no ready-action run named by the machine**: a machine program
    /// from before the card's reading (2026-10-03), or a run it named that did
    /// not read back. **It is not *never set up***, so the card says what
    /// Smoozy knows. Added 2026-09-21.
    ConnectedAndNoRunWasFound,
    /// **Connected, no run found, and the machine holds no installation of
    /// Smoozy's at all** — [`crate::setting_up::the_machine_holds_no_installation`].
    /// **This one is *never set up***, `Server Setup` Y6a's own reading, and it
    /// is what a setup whose placement failed before any run leaves behind — the
    /// second pass on a real rental's fault 4, 2026-09-21, where the card read
    /// *No setup read from this server* on exactly that machine.
    ConnectedAndNeverSetUp,
    /// **Connected, and Smoozy's service on the server does not answer** —
    /// `State, identity and recovery` §8.3's supervisor-unavailable condition
    /// standing (`Desktop app and delivery` V1), on `State, identity and
    /// recovery` Y9's fourth branch while `Servers` Q19 re-takes it. **It is
    /// not *never read***: Smoozy has connected, and the service is what did
    /// not answer — `Design/Server Setup § This server`. Added 2026-09-27, step
    /// 10c's eighth-pass fix piece 6, out of the seventh pass on RunPod, where
    /// this card said *Smoozy has not read this server yet* under a header
    /// saying *Connected*.
    ConnectedAndSmoozyIsNotResponding,
}

// ════════════════════════════════════════════════════════════════════════════
// What the page is handed
// ════════════════════════════════════════════════════════════════════════════

/// The whole tab, in one answer.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheSetUpTab {
    /// The server this reading is of. A value.
    pub server: Verbatim,
    /// The two section headings — `Design/Server Setup § Two sections, and no
    /// third`, and there is no add-ons section.
    pub this_server: Prose,
    pub blender: Prose,
    pub read_from_the_server: Prose,
    /// **The *This server* card**, in whichever of its readings the machine
    /// answered.
    pub the_card: TheCard,
    /// **The Blender table.**
    pub the_table: TheBlenderTable,
    /// **The panel a setup that stopped part-way opens on** —
    /// `Design/Server Setup § This server`: *"The setup stopped part-way. Its
    /// own screen, and there the action is the whole point"*, and `§ The two
    /// ways out`: both of `Server Setup` S6's runs have a control on it, *Set up
    /// again* and *Abandon setup*. **Absent on every other reading.** Added
    /// 2026-09-22: step 10c's third pass found the panel composed and tested and
    /// drawn by nothing, so *Abandon setup* was on no screen at all.
    pub the_stopped_setup: Option<TheRunPanel>,
    /// **`Server Setup` Y6a's stamp on a card drawn from what was last seen** —
    /// *last seen* and this computer's own moment of reading, which the page
    /// writes as a moment. **Absent on every card read now.** The 38's piece 6.
    pub the_card_was_last_seen: Option<Words>,
    /// **The card's driver line** — `Servers` H11b, `Design/Server Setup §
    /// This server`: *"The card always shows the server's graphics driver,
    /// under the steps, beside Telegram reporting's line"*. Drawn on the card,
    /// and carried beside it because it is the server's own reading rather
    /// than the setup run's: there whether or not a setup ran. **Absent where
    /// no NVIDIA driver answered.** Added 2026-10-11.
    pub the_driver: Option<TheDriverLine>,
    /// **`Servers` H11b's statement, in amber, under the driver line** — only
    /// where the driver is below a bar and Smoozy cannot change it. Added
    /// 2026-10-11.
    pub the_driver_warning: Option<TheDriverWarning>,
    /// **The installs Smoozy closed before it began, named on the card under
    /// its state line** — `Server Setup` Y9a, `Design/Server Setup §
    /// Installing`: *Not installed: Blender 5.2.2, Blender 4.5.14. Smoozy
    /// closed before they started.* **Absent where none are kept, and while
    /// this copy drives a run on the server.** Added 2026-10-16, step 10c's
    /// owed piece 4.
    pub the_installs_never_begun: Option<Prose>,
    /// ***Server output*, opening what the last run on this server printed** —
    /// `Design/Server Setup § A long step`: *"The window goes when the run
    /// ends; the control stays, opening what that run printed, until the next
    /// run or until Smoozy is closed."* **Absent where this copy keeps no
    /// run's words for the server** (`Server Setup` Y7a). Added 2026-10-18.
    pub server_output: Option<Prose>,
}

/// **The driver line**: the driver's version and, where the driver gave it,
/// the CUDA version, each a value beside its words.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheDriverLine {
    /// *NVIDIA driver* and the version.
    pub the_driver: Words,
    /// *· CUDA* and the CUDA version. **Absent where the driver's summary did
    /// not give it**, and the version above still stands.
    pub the_cuda: Option<Words>,
    /// **Greyed, with when it was read, where Smoozy is not connected** —
    /// `Servers` H11b's *shown greyed as last seen*, `Design/Server Setup §
    /// This server`. *Last seen* and this computer's own moment of the
    /// reading (`Servers` H2a), which the page writes as a moment. **Absent
    /// where the reading is this connection's own.**
    pub last_seen: Option<Words>,
}

/// **`Servers` H11b's statement, as two sentences** — each its own string,
/// because the typography point binds the last pair of one string and never
/// parses sentences out of two (`Desktop app and delivery` X2).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheDriverWarning {
    /// *This server's NVIDIA driver, 550.163.01 (CUDA 12.4), is too old for
    /// OptiX in Blender 5.2 and later.*
    pub said: Prose,
    /// *It can't be updated here.*, or *It could not be updated.*
    pub and: Prose,
    /// **Greyed where the reading is last seen**, as the line above it is.
    pub last_seen: bool,
}

/// **What this tab is handed of the driver** — `Servers` H11b's part of the
/// reading the server's row holds, the missed bar beside it (`Servers` D3's
/// row of 2026-10-11), and whether that reading is this connection's own.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TheDriverAsHeld {
    /// The driver's own answer, where the held reading carries one.
    pub the_driver: Option<smoozy_servers::hardware::TheDriversOwnAnswer>,
    /// `Server Setup` R1 item 5's miss, as the last run that checked the
    /// driver left it.
    pub the_driver_missed_the_optix_bar: bool,
    /// **Where the reading is not this connection's own**, this computer's
    /// own moment of it (`Servers` H2a) — *last seen*. `None` on a live one.
    pub last_seen_at: Option<std::time::SystemTime>,
}

/// **The *This server* card** — one state line, when it was done, one action,
/// and a list of what was done. Nothing else.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheCard {
    /// **Which of the drawing's dots** — `ok`, `idle` or `busy`, and `bad`
    /// where the connection an add asked for was not made (`Design/Server
    /// Setup § This server`, *"with the red dot"*). A value the page marks
    /// with and never derives.
    pub dot: Verbatim,
    /// The state line.
    pub state: Prose,
    /// **The moment, as of the moment it was taken, on the clock it was taken
    /// on** — §16.1 M3a. Absent where there is no moment to name.
    pub when: Option<TheMoment>,
    /// **What was done to the machine.** Empty where nothing was.
    pub what_was_done: Vec<ALineOfWhatWasDone>,
    /// **`Server Setup` W3a's one line about the cards.**
    pub the_render_device: Option<Words>,
    /// **`Server Setup` W4's setup fault**, which is the one state where the
    /// action earns being loud.
    pub the_setup_fault: Option<TheSetupFault>,
    /// A note under the list — the *nothing to prepare* summary, the running
    /// note, the Telegram-only note.
    pub the_note: Option<Words>,
    /// **The action.** Always present and never greyed out —
    /// `Design/Server Setup § This server`.
    pub the_action: TheAction,
    /// **Where nothing has ever been read from the machine**, what stands in
    /// the card's place.
    pub instead: Option<Words>,
}

/// A moment on somebody's clock — §16.1 M3a, and never an elapsed time.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheMoment {
    /// The words before it — *set up*, *checked*, *started*.
    pub lead: Prose,
    /// The moment itself, with its clock.
    pub reading: AsOfThatMoment,
    /// The words the page puts after the moment, naming the clock it is on.
    pub on_the_clock: Prose,
}

/// **The step marks' four colours, as the drawing's own classes** —
/// `Design/Server Setup § This server`: *done* green, *nothing to do* grey, *not
/// checked …* amber, a step that failed red, the step's name in ordinary ink.
/// The tokens are `Design values`' (`--ok` widened by one phrase for this).
/// **A mark with none of the four stays plain**: a step under way, one not
/// started, and a line that is not a step's mark. Added 2026-10-03, the
/// Creator's note 3 on the test copy.
pub mod the_marks_colour {
    pub const DONE: &str = "stm-ok";
    pub const NOTHING_TO_DO: &str = "stm-idle";
    pub const NOT_CHECKED: &str = "stm-busy";
    pub const FAILED: &str = "stm-bad";
    pub const PLAIN: &str = "";
}

/// One line of the card's list of what was done.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ALineOfWhatWasDone {
    pub said: Words,
    /// The small mark at the end of the line, where one belongs.
    pub mark: Option<Prose>,
    /// **The mark's colour** — one of [`the_marks_colour`], plain where the
    /// mark is not a step's. A value the page marks with and never derives.
    pub colour: Verbatim,
    /// **Whether the line is a thing that is missing rather than done** — the
    /// drawing's own `sumiss`.
    pub missing: bool,
    /// **The control on the line, where the drawing puts one** — only *Install
    /// it*, on the Telegram line.
    pub offer: Option<ALinesOffer>,
}

/// A control on one line of the card: its words, and the act it hands back.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ALinesOffer {
    pub name: Prose,
    pub act: &'static str,
}

/// **`Server Setup` W4's setup fault, with the one thing Smoozy knows to do.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheSetupFault {
    pub said: Prose,
    pub what_to_do: Prose,
}

/// The card's one action.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheAction {
    pub name: Prose,
    /// **Loud only where Smoozy can name a reason to press it** —
    /// `Design/Server Setup § This server` names the two.
    pub loud: bool,
    /// **`Design/Server Setup § A second setup while one is running`: the
    /// action is never offered twice**, so it goes underway on itself rather
    /// than being pressed and then refused.
    pub underway: bool,
    /// Which act the page hands back.
    pub act: Verbatim,
}

/// **The Blender table** — read from the server every time.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheBlenderTable {
    pub version: Prose,
    pub location: Prose,
    pub rows: Vec<ABuildRow>,
    /// **Where no reading exists**, what stands in the table's place.
    pub instead: Option<Words>,
    // **No line about the folders the search could not read** — `Server
    // Setup` B11 keeps the count in the search's answer and the diary and does
    // not draw it; `Design/Server Setup § Blender`: *"Nothing under the table
    // says how the server was searched."* Creator-stated 2026-10-02, note 13
    // on 0.1.7. *(The table carried that line until then.)*
    pub install: Prose,
}

/// One row of it — three columns: version, where it sits, and Remove.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ABuildRow {
    /// **B13: the path is what identifies the build**, and it is what every act
    /// on this row hands back. A value.
    pub path: Verbatim,
    /// **The version string, which already says the kind** —
    /// `Design/Server Setup § Blender`: there is no kind column because an
    /// artist reads long-term support off the number. A value.
    pub version: Verbatim,
    /// **B12a: the directory of the path the search returned**, derived when
    /// the table is drawn and never remembered. A value, and the link into
    /// `File Browser`.
    pub folder: Verbatim,
    /// **The same folder as bytes**, which is what `File Browser` is opened at
    /// (`File Browser` L1a, must-do 30).
    pub folder_bytes: Vec<u8>,
    /// **B8a's tag, where the origin earns one.**
    pub tag: Option<Prose>,
    /// **Below the floor**, which is not a refusal.
    pub below_the_floor: Option<Prose>,
    /// **`Server Setup` B12b's tag** — *driver too old for OptiX*, where this
    /// connection's driver is below the OptiX bar of the version this build
    /// answered. **It refuses nothing**: the build can still be picked for a
    /// job. Added 2026-10-11.
    pub driver_too_old: Option<Prose>,
    pub remove: Prose,
}

// ════════════════════════════════════════════════════════════════════════════
// The commands
// ════════════════════════════════════════════════════════════════════════════

/// **The whole Set up tab, in one answer.**
///
/// **A command rather than an event, and P6 is not being bent.** P6 is about a
/// module's *results*; this is the page asking for the facts it needs before it
/// can draw anything, which is the shape `the_servers_own_screen`,
/// `the_all_servers_screen` and `the_rail` already have. **A page that comes
/// back asks again** (P8).
///
/// **H34: the values in are a server identity and, where the page holds one
/// for that server, the `what` of the connect that did not happen** — read
/// against the closed set of three
/// ([`crate::surface::servers::what_a_failed_connect_said`]), so nothing the
/// page hands in can put a word on the card. Added 2026-10-03, the Creator's
/// note 14 on the test copy.
#[tauri::command]
pub async fn the_set_up_tab<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    server: String,
    failed: Option<String>,
) -> TheSetUpTab {
    // **Off the thread that draws, on a thread of its own**: the reading
    // crosses the connection on a wait of its own, which is the shape the
    // ready action's own act already takes (P5).
    let asked = server.clone();
    let (told, said) = tokio::sync::oneshot::channel();
    let here = app.clone();
    std::thread::spawn(move || {
        // **Two readings, not one, and they answer different questions.** The
        // card is `Server Setup` Y6a and Y6b's reading of a setup run; the
        // Blender table is `Server Setup` B10's whole-disk search, which
        // `Server Setup` B12 runs at exactly three moments and **this is the
        // first of them —
        // *when Server Setup's own view is opened*.**
        //
        // **The two are taken side by side** (2026-10-26, the worldwide pass's
        // setup faults, fault 4): neither reads what the other found, and the
        // card waited for both one after the other — on a far server most of
        // the 27 to 36 seconds the pass watched between *Idle* and the finished
        // card. The point serves both as it serves any two asks.
        let searching = {
            let (app, asked) = (app.clone(), asked.clone());
            std::thread::spawn(move || the_blender_search_now(&app, &asked))
        };
        let said = what_the_machine_said(&app, &asked);
        let found = searching.join().unwrap_or(TheBlenderSearch::ItWasNotTaken);
        let _ = told.send((said, found));
    });
    let (said, found) = said.await.unwrap_or((
        WhatTheMachineSaid::NothingHasEverBeenReadFromIt,
        TheBlenderSearch::ItWasNotTaken,
    ));
    let nothing_was_read = matches!(
        said,
        WhatTheMachineSaid::NothingHasEverBeenReadFromIt | WhatTheMachineSaid::LastSeen(..)
    );
    let telegram_is_on = telegram_is_switched_on_here(&here);
    let builds = the_builds_the_table_is_drawn_from(&said, &found);
    let mut tab = the_tab_with(said, &server, found, telegram_is_on);
    // **Read after the machine was**, so a connection that went live
    // meanwhile is not drawn as still connecting, nor as not made. **An add's
    // setup still waiting to be handed over is read whatever the machine
    // said with nothing lent** — `Servers` Q19's re-take included — because
    // until step 8 takes it the card is *Connecting…* (piece M).
    //
    // **An add's setup that the first checks stopped is read beside it**
    // (piece M2): the row no longer remembers it, so this is what lets the
    // card say why.
    let waits = crate::setting_up::an_adds_setup_waits_on(&server);
    if nothing_was_read || waits {
        let connecting = an_add_is_connecting_to_set_up(&here, &server);
        let remembered = the_row_remembers_a_setup(&here, &server);
        let stopped = crate::setting_up::an_adds_setup_never_reached_step_8_on(&server);
        the_card_where_no_connection_stands(
            &mut tab,
            connecting,
            waits,
            stopped,
            failed.as_deref(),
            remembered,
        );
    }
    // **`Servers` H11b and `Server Setup` B12b, drawn last**, once the card is
    // chosen, from the server's row. Added 2026-10-11.
    the_driver_is_drawn_on(&mut tab, &the_driver_as_held_for(&here, &server), &builds);
    // **`Server Setup` Y9a: the installs an earlier run closed before it
    // began**, from the local record. Added 2026-10-16, owed piece 4.
    tab.the_installs_never_begun = the_record_says(&here, |record| the_installs_smoozy_closed_on(record, &server)).flatten();
    // **The running card offers the Telegram placement where this card does**
    // (`Server Setup` Y9a item 6). Added 2026-10-27.
    the_telegram_offer_is_held(&server, &tab.the_card);
    // **`Server Setup` Y7a: the last run's words, one control away**, for as
    // long as this copy keeps them. Added 2026-10-18.
    the_servers_words_beside_the_tab(&mut tab, crate::setting_up::the_words_are_kept_for(&server));
    tab
}

/// ***Server output* on the card and on a stopped setup's card, where a run's
/// words are kept** — and never the window: that is only on a run still going
/// (`Desktop app and delivery` M13b's second narrowing). Added 2026-10-18, the
/// Creator's thirteenth notes, piece H.
pub fn the_servers_words_beside_the_tab(tab: &mut TheSetUpTab, the_words_are_kept: bool) {
    let control = the_words_are_kept.then(|| Prose::new(words::SERVER_OUTPUT));
    tab.server_output = control.clone();
    if let Some(stopped) = tab.the_stopped_setup.as_mut() {
        stopped.server_output = control;
        stopped.the_servers_newest_lines = Vec::new();
        stopped.which_lines_are_from_the_error_stream = Vec::new();
    }
}

/// **The local record, lent for one reading or write**, where it was opened.
/// Nothing where it was not — `Desktop app and delivery` L8a's failure is the
/// start's to show, and this answers nothing rather than an emptied version.
fn the_record_says<R: tauri::Runtime, T>(
    app: &tauri::AppHandle<R>,
    reading: impl FnOnce(&rusqlite::Connection) -> T,
) -> Option<T> {
    use tauri::Manager;
    let started = app.try_state::<Mutex<Started>>()?;
    let started = started.lock().ok()?;
    let record = started.record.as_ref()?;
    Some(reading(record.connection()))
}

/// **`Server Setup` Y9a's write: the installs still waiting on that server, as
/// the press or the chain leaves them.** A write that fails costs the card's
/// line on a later opening and nothing else, so it stops nothing here.
pub fn the_installs_waiting_are_kept<R: tauri::Runtime>(app: &tauri::AppHandle<R>, server: &str, waiting: &[String]) {
    let _ = the_record_says(app, |record| {
        crate::record::installs_not_begun::the_ones_waiting_are_kept(record, server, waiting)
    });
}

/// **`Server Setup` Y9a item 10's write: the acts still waiting on that
/// server, as the press or the line leaves them.** Added 2026-10-27, the
/// nineteenth notes, note 2.
pub fn the_acts_waiting_are_kept<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    waiting: &[crate::setting_up::ASetUpAct],
) {
    let kept = waiting.iter().map(crate::setting_up::ASetUpAct::as_kept).collect::<Vec<_>>();
    let _ = the_record_says(app, |record| {
        crate::record::installs_not_begun::the_acts_waiting_are_kept(record, server, &kept)
    });
}

/// **`Server Setup` Y9a's reading: the installs an earlier run of Smoozy
/// closed before it began on that server**, as the card's line. Added
/// 2026-10-16, step 10c's owed piece 4. **From 2026-10-27 every act kept**
/// (`Server Setup` Y9a item 10): where only installs from the list were waiting the line
/// reads as it always has, and otherwise [`the_acts_never_begun`].
///
/// **Whether to name them is `Server Setup`'s judgement**, at
/// `smoozy_server_setup::blender::a_closed_run_left_them`: a list kept while
/// this copy drives no run on the server can only be one an earlier run left,
/// since this run's own line rewrites it as each act begins and empties it
/// when it ends ([`the_acts_are_driven_in_turn`]). **While a
/// run is driven here, nothing** — the list is that run's, and it is still
/// going.
pub fn the_installs_smoozy_closed_on(record: &rusqlite::Connection, server: &str) -> Option<Prose> {
    use crate::record::installs_not_begun::TheActsKind;
    let waiting = crate::record::installs_not_begun::the_acts_waiting_on(record, server).ok()?;
    let named = waiting.iter().map(|act| act.named.clone()).collect::<Vec<_>>();
    let a_run_is_driven_here = crate::setting_up::the_kind_being_driven_on(server).is_some();
    if !smoozy_server_setup::blender::a_closed_run_left_them(&named, a_run_is_driven_here) {
        return None;
    }
    if waiting.iter().all(|act| act.kind == TheActsKind::AnInstallFromTheList) {
        return the_installs_never_begun(&named);
    }
    the_acts_never_begun(&waiting)
}

/// **The line where anything but installs from the list was waiting** —
/// `Design/Server Setup § Installing`: *Not started: Remove Blender 5.2.2, Set
/// up again. Smoozy closed first.* Each act named as the card's *Waiting*
/// list named it. Nothing where none are kept. Added 2026-10-27, the
/// nineteenth notes, note 2.
pub fn the_acts_never_begun(waiting: &[crate::record::installs_not_begun::AnActKept]) -> Option<Prose> {
    if waiting.is_empty() {
        return None;
    }
    let named = waiting.iter().map(the_act_is_named).collect::<Vec<_>>().join(", ");
    Some(Prose::new(&format!("{} {named}. {}", words::NOT_STARTED_THESE, words::SMOOZY_CLOSED_FIRST)))
}

/// **An act waiting its turn, named as the act it is** — `Design/Server Setup
/// § Asking for more while a setup runs`: *Install Blender 4.5.15* (named as
/// the list names it), *Install your build blender-4.2-linux.tar.xz*, *Remove
/// Blender 5.2.2*, *Set up again*, *Install Telegram reporting*. Added
/// 2026-10-27, the nineteenth notes, note 2.
pub fn the_act_is_named(act: &crate::record::installs_not_begun::AnActKept) -> String {
    use crate::record::installs_not_begun::TheActsKind;
    match act.kind {
        TheActsKind::AnInstallFromTheList => {
            format!("{} {} {}", words::INSTALL, words::BLENDER, the_build_asked_for_is_named(&act.named))
        }
        TheActsKind::TheUsersBuild => format!("{} {}", words::INSTALL_YOUR_BUILD, act.named),
        TheActsKind::ARemoval => format!("{} {} {}", words::REMOVE, words::BLENDER, act.named),
        TheActsKind::TheReadyAction => words::SET_UP_AGAIN.to_string(),
        TheActsKind::TheTelegramPlacement => words::INSTALL_TELEGRAM_REPORTING.to_string(),
    }
}

/// **The acts a run stopped, named after its ending's own sentence** —
/// `Design/Server Setup § Asking for more while a setup runs`: *Not started:
/// Remove Blender 5.2.2, Install Blender 4.5.15.*, and *The server restarted
/// first.* where the run in hand restarted it. Nothing where none were
/// stopped. Added 2026-10-27, the nineteenth notes, note 2.
pub fn the_acts_not_started_are_named(
    came: WhatItCameTo,
    not_started: &[crate::setting_up::ASetUpAct],
    the_server_restarted: bool,
) -> WhatItCameTo {
    if not_started.is_empty() {
        return came;
    }
    let named = not_started
        .iter()
        .map(|act| the_act_is_named(&act.as_kept()))
        .collect::<Vec<_>>()
        .join(", ");
    let mut said = format!("{} {} {named}.", came.said, words::NOT_STARTED_THESE);
    if the_server_restarted {
        said = format!("{said} {}", words::THE_SERVER_RESTARTED_FIRST);
    }
    WhatItCameTo {
        said: said.trim_start().to_string(),
        ..came
    }
}

/// **The line itself** — `Design/Server Setup § Installing`: *Not installed:
/// Blender 5.2.2, Blender 4.5.14. Smoozy closed before they started.* Each is
/// named as the list named it, as the line after a failed install names them.
/// Nothing where none are kept.
pub fn the_installs_never_begun(waiting: &[String]) -> Option<Prose> {
    if waiting.is_empty() {
        return None;
    }
    let named = waiting
        .iter()
        .map(|asked| format!("{} {}", words::BLENDER, the_build_asked_for_is_named(asked)))
        .collect::<Vec<_>>()
        .join(", ");
    let closed = if waiting.len() == 1 {
        words::SMOOZY_CLOSED_BEFORE_IT_STARTED
    } else {
        words::SMOOZY_CLOSED_BEFORE_THEY_STARTED
    };
    Some(Prose::new(&format!("{} {named}. {closed}", words::NOT_INSTALLED_THESE)))
}

/// **Where no connection stands, the two cards an add's own connection
/// leaves** — `Design/Server Setup § This server`:
///
/// - **while it is being made**, *Connecting…* with the action under way on
///   itself (the drawing's `suconnect`, the Creator's note 3 on 0.1.7);
/// - **where it was not made**, the sentence the connect ended in, the red
///   dot, and *Connect and set up* — the row's own act, which carries the
///   setup the add asked for (`Servers` B1). Added 2026-10-03, the Creator's
///   note 14 on the test copy: the sentence had been said above the All
///   servers list, which *Add and set up* had just moved the user away from.
///
/// **From the press until step 8 takes the setup, *Connecting…***, and the
/// action under way, so nothing between can be pressed — the Creator's fourth
/// note 1, piece M. At the login `Servers` B5 clears the row's bit and the
/// connect mark lets go, but the add's setup still waits in the transport
/// until the connection is lent and step 8 hands it over
/// ([`crate::setting_up::an_adds_setup_waits_on`]); until 2026-10-07 the card
/// fell back for those seconds to *Smoozy has not read this server yet.* with
/// *Set up this server* pressable. **Where the period ends first, it no longer
/// waits**, and *Connecting…* goes.
///
/// **Where the login carried the add's setup and the first checks then stopped
/// the connection** — `Servers` Q1 step 4, not a Linux machine or nothing it
/// could read, or the connection lost at step 5 — **the card says the sentence
/// the connect ended in**, with the red dot, as where the connection was not
/// made (piece M2, the Creator's fourth note 1). `Servers` B5 has already
/// cleared the row's bit and `Servers` B4 remembers nothing of the setup, so the card
/// cannot offer *Connect and set up*: that act would now only connect. **Its
/// action stays what the card had, *Set up this server***, which asks for a
/// connection first. Until 2026-10-08 the card read *Smoozy has not read this
/// server yet.* here.
///
/// **Otherwise only for a row that still remembers a setup**, which is the add's case:
/// a plain *Connect* that failed carries no setup, so *Connect and set up*
/// would name an act it is not, and the line above the list says it. **A
/// connect being made outranks one that was not**: a new attempt is the next
/// act. **A server not read now is the only one this reaches**, and the last
/// seen stamp and the stopped panel of an earlier reading go with the card
/// they belonged to.
pub fn the_card_where_no_connection_stands(
    tab: &mut TheSetUpTab,
    an_add_is_connecting: bool,
    an_adds_setup_waits: bool,
    an_adds_setup_never_reached_step_8: bool,
    failed: Option<&str>,
    the_row_remembers_a_setup: bool,
) {
    if !the_row_remembers_a_setup && !an_adds_setup_waits && !an_adds_setup_never_reached_step_8 {
        return;
    }
    // **The connect mark is an add's only where the row still remembers the
    // setup** — a plain *Connect* after the first checks stopped an add is not
    // drawn as a setup about to run.
    let card = if (an_add_is_connecting && the_row_remembers_a_setup) || an_adds_setup_waits {
        the_card_while_an_add_connects()
    } else if let Some(said) = failed.and_then(crate::surface::servers::what_a_failed_connect_said) {
        if the_row_remembers_a_setup {
            the_card_where_the_connection_was_not_made(said)
        } else {
            the_card_where_the_first_checks_stopped_an_add(said)
        }
    } else {
        return;
    };
    tab.the_card = card;
    tab.the_card_was_last_seen = None;
    tab.the_stopped_setup = None;
}

/// **Whether the row still remembers the setup an add asked for** —
/// `Servers` B1, cleared when the run is taken or the user drops it.
fn the_row_remembers_a_setup<R: tauri::Runtime>(app: &tauri::AppHandle<R>, server: &str) -> bool {
    use tauri::Manager;
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return false;
    };
    let Ok(started) = started.lock() else {
        return false;
    };
    let Some(record) = started.record.as_ref() else {
        return false;
    };
    crate::record::server_rows::all(record.connection())
        .ok()
        .and_then(|rows| rows.into_iter().find(|row| row.local_server_identifier == server))
        .is_some_and(|row| row.remembered_setup.is_some())
}

/// **Whether Telegram is on in Settings** — `Telegram`'s own switch, read from
/// the local record, which is what `Server Setup` G2's *"while Telegram is on"*
/// asks. A record that does not answer reads as off, so nothing is offered on a
/// guess. The 38's piece 11.
fn telegram_is_switched_on_here<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> bool {
    use tauri::Manager;
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return false;
    };
    let Ok(started) = started.lock() else {
        return false;
    };
    let Some(record) = started.record.as_ref() else {
        return false;
    };
    crate::record::settings::the_telegram_settings(record.connection())
        .is_ok_and(|settings| settings.switched_on)
}

/// **Whether the connection an add with a setup ticked asked for is being
/// made now**: the user's connect mark stands on the server (`Servers` O2's
/// connection carries it, `src/surface/servers.rs` `a_server_is_added`) and
/// the row still remembers the setup it is to carry out (`Servers` B1, cleared
/// when the run is taken). A plain *Connect* remembers no setup, so it is not
/// drawn as a setup about to run.
///
/// **The row's half is [`the_row_remembers_a_setup`]**, read beside this one
/// and weighed in [`the_card_where_no_connection_stands`].
fn an_add_is_connecting_to_set_up<R: tauri::Runtime>(app: &tauri::AppHandle<R>, server: &str) -> bool {
    let periods = crate::surface::servers::the_periods_of(app);
    let of = crate::surface::servers::what_is_true_of(periods.as_deref(), server);
    crate::surface::servers::THE_USERS_CONNECTS.is_underway(server, of)
}

/// **The card while an add's connection is made** — `Design/Server Setup §
/// This server`: *"the state line reads Connecting… and the action is under
/// way on itself. Then the run's steps take the card."* The drawing's
/// `suconnect`.
pub fn the_card_while_an_add_connects() -> TheCard {
    TheCard {
        dot: Verbatim::new("busy"),
        state: Prose::new(words::CONNECTING),
        when: None,
        what_was_done: Vec::new(),
        the_render_device: None,
        the_setup_fault: None,
        the_note: None,
        the_action: TheAction {
            name: Prose::new(words::SET_UP_THIS_SERVER),
            loud: true,
            underway: true,
            act: Verbatim::new(THE_READY_ACTION),
        },
        instead: None,
    }
}

/// **The card where the first checks stopped an add's connection** — the
/// sentence the connect ended in and the red dot, as
/// [`the_card_where_the_connection_was_not_made`], **and the action the card
/// had before, *Set up this server***, since `Servers` B5 has already
/// forgotten the setup and *Connect and set up* would only connect. Added
/// 2026-10-08, the Creator's fourth note 1, piece M2.
pub fn the_card_where_the_first_checks_stopped_an_add(said: &str) -> TheCard {
    TheCard {
        the_action: the_card_where_nothing_was_read().the_action,
        ..the_card_where_the_connection_was_not_made(said)
    }
}

/// **The card where the connection an add asked for was not made** —
/// `Design/Server Setup § This server`: *"the card says so: Smoozy could not
/// connect to this server., with the red dot, and its action is Connect and
/// set up"*, or the sentence `Keys` has for the cause where it has one. **It
/// claims nothing about why** beyond what that sentence claims. The drawing's
/// `su-connectfail` (card `suconnectfail`), drawn after this was built. Added
/// 2026-10-03, the Creator's note 14 on the test copy.
pub fn the_card_where_the_connection_was_not_made(said: &str) -> TheCard {
    TheCard {
        dot: Verbatim::new("bad"),
        state: Prose::new(said),
        when: None,
        what_was_done: Vec::new(),
        the_render_device: None,
        the_setup_fault: None,
        the_note: None,
        the_action: TheAction {
            name: Prose::new(words::CONNECT_AND_SET_UP),
            loud: true,
            underway: false,
            act: Verbatim::new(CONNECT_AND_SET_UP),
        },
        instead: None,
    }
}

/// **`Server Setup` B10's search, run for this tab at B12's first moment.**
///
/// **Added 2026-09-23, out of step 10c's fifth pass on Vast, fault 3.** The
/// table read *No Blender has been read from this server.* with two working
/// builds on the machine, before the installs, after them, after a tab switch
/// and after a fresh connection period — because
/// [`WhatTheMachineSaid::AReading`] was constructed nowhere in the workspace
/// and every path drew [`the_table_where_nothing_was_read`]. **The search
/// itself was never at fault**: `Render Settings` runs this same ask over the
/// same point and gets the builds.
///
/// **It is the same ask, called and not copied** —
/// [`crate::submitting::the_blenders_on_the_machine`], which is B1's ask over
/// `Server Setup` B10's and `Server Setup` B11's search. **Smoozy's own
/// Blender area is not a candidate** at
/// the exact path `Server Setup` B9 composes, or nothing where no placement is
/// recorded, which is that document's rule and is passed straight through.
///
/// **Nothing is claimed where no connection stands or the machine could not be
/// asked**, and the table says a reading does not exist rather than showing an
/// empty one.
pub(crate) fn the_blender_search_now<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
) -> TheBlenderSearch {
    use tauri::Manager;
    let Some(point) = the_live_point(app, server) else {
        // **`Server Setup` B12: the reading is period-held**, so a tab drawn
        // with no live connection holds none.
        the_search_is_taken_again_on(server);
        return TheBlenderSearch::ItWasNotTaken;
    };
    // **It is taken once and held, and never once per drawing.** This tab is
    // asked again on every act's ending, every box close, every Escape and
    // every server switch, and **that search costs a whole-disk pass and a run
    // per candidate on a machine billed by the hour** — 5.3 seconds observed
    // for the pass alone, with a sixty-second limit on each candidate's run.
    // B12 makes the reading period-held for exactly that reason.
    if let Ok(held) = the_searches_held().lock() {
        if let Some(found) = held.get(server) {
            return found.clone();
        }
    }
    let the_row = app
        .try_state::<std::sync::Mutex<crate::start::Started>>()
        .and_then(|started| {
            let started = started.lock().ok()?;
            let rows = crate::record::server_rows::all(started.record.as_ref()?.connection()).ok()?;
            rows.into_iter()
                .find(|row| row.local_server_identifier == server)
        });
    let own_area = the_row
        .as_ref()
        .and_then(crate::servers::installation::the_layout_recorded_for)
        .map(|layout| smoozy_server_setup::blender::the_blender_area(&layout));
    let package_area = the_row.as_ref().and_then(crate::setting_up::the_package_area_of);
    let Ok(runtime) = tokio::runtime::Builder::new_current_thread().enable_all().build() else {
        return TheBlenderSearch::ItWasNotTaken;
    };
    // **The server is named, and the ask takes turns by it** with the Render
    // Settings page's own search, and with this tab drawn twice (step 10c's
    // eleventh-pass fix piece 4c).
    let asked = runtime.block_on(crate::submitting::the_blenders_on_the_machine(
        &point,
        server,
        own_area.as_deref(),
    ));
    // **`Server Setup` R1b's comparison, at `Server Setup` B12's moment** —
    // the view opened, which is also the table a removal is offered from —
    // and after an install. Taken where the search is taken and never from
    // the held one, which was compared when it was taken. Added 2026-10-12.
    crate::servers::the_driver_swap::the_search_is_compared(app, server, &asked);
    // **`Server Setup` B8a: the install records, read beside the search** —
    // the 38's piece 14b. Nothing read is nothing claimed.
    let records = match &package_area {
        Some(area) => runtime.block_on(crate::setting_up::the_install_records(&point, area)),
        None => None,
    };
    let found = TheBlenderSearch::from_the_ask(asked).with_the_install_records(records.as_deref());
    // **A machine that could not be asked is not held**, so the next ask tries
    // again rather than a failure standing for the period (L8a: it is not a
    // reading).
    if let TheBlenderSearch::ItFound { .. } = &found {
        if let Ok(mut held) = the_searches_held().lock() {
            held.insert(server.to_string(), found.clone());
        }
    }
    found
}

/// **`Server Setup` B12's period-held search, per server.**
///
/// **Run-held and never durable** — `State, identity and recovery` D4: the
/// reading is taken fresh at the moment of use and nothing is written from it.
fn the_searches_held(
) -> &'static std::sync::Mutex<std::collections::BTreeMap<String, TheBlenderSearch>> {
    static HELD: std::sync::OnceLock<
        std::sync::Mutex<std::collections::BTreeMap<String, TheBlenderSearch>>,
    > = std::sync::OnceLock::new();
    HELD.get_or_init(|| std::sync::Mutex::new(std::collections::BTreeMap::new()))
}

/// **`Server Setup` W1: whether the last ready action this copy drove on a
/// server placed Smoozy's program before its run**, per server. Step 10c's
/// twelfth-pass fix piece 6.
///
/// **Why the card needs it.** The placement — `Server Setup` R2's hand-over,
/// release and placement — comes before the run and is not one of the run's
/// boundaries, so a run whose every step found nothing to do reads, on its own
/// record, exactly like a check that found nothing to prepare. W1 keeps
/// *Nothing to prepare* for a run where no act was needed, and a placement is
/// an act. The twelfth pass on Hyperstack met it: *Set up this server* on a
/// server added plainly placed a new program and the card read *Nothing to
/// prepare … Nothing was installed and nothing was changed.*
///
/// **Run-held and never durable.** A finished ready action is read only as
/// the run this copy last drove on that server (see
/// [`crate::setting_up::the_setup_as_read`]), and every ready action this copy
/// drives writes this afresh when it ends, so what is held always speaks of
/// the run the card reads.
fn the_placements_before_the_last_run() -> &'static Mutex<std::collections::BTreeMap<String, bool>> {
    static HELD: std::sync::OnceLock<Mutex<std::collections::BTreeMap<String, bool>>> =
        std::sync::OnceLock::new();
    HELD.get_or_init(|| Mutex::new(std::collections::BTreeMap::new()))
}

/// **A ready action ended, and whether it placed Smoozy's program first is
/// read off the server row** — the installation the row confirmed before the
/// press against the one it confirms after. A placement records a new
/// installation (`State, identity and recovery` Y8 issues one at the
/// hand-over), and `Server Setup` R1's run alone on a machine this copy set up
/// records none, so the row moving is the placement and nothing else.
pub fn the_ready_action_ended_on(server: &str, confirmed_before: Option<&str>, confirmed_after: Option<&str>) {
    let placed = confirmed_after.is_some() && confirmed_after != confirmed_before;
    if let Ok(mut held) = the_placements_before_the_last_run().lock() {
        held.insert(server.to_string(), placed);
    }
}

fn a_placement_came_before_the_last_run_on(server: &str) -> bool {
    the_placements_before_the_last_run()
        .lock()
        .ok()
        .and_then(|held| held.get(server).copied())
        .unwrap_or(false)
}

/// **The installation the server row confirms**, read the way the rest of
/// this tab reads the row.
fn the_installation_the_row_confirms<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
) -> Option<String> {
    use tauri::Manager;
    let started = app.try_state::<Mutex<Started>>()?;
    let started = started.lock().ok()?;
    let rows = crate::record::server_rows::all(started.record.as_ref()?.connection()).ok()?;
    rows.into_iter()
        .find(|row| row.local_server_identifier == server)
        .and_then(|row| row.last_confirmed_installation_identity)
}

/// **What is held for that server is let go**, so the next opening of the tab
/// takes the search again.
///
/// **Its two callers are the two things that make the held reading wrong**: an
/// act of Smoozy's own ending on that machine (`crate::setting_up::a_run_ends`
/// — a Blender may have been installed or removed), and the connected period
/// ending. **Not a timer** (P9, P10).
pub fn the_search_is_taken_again_on(server: &str) {
    if let Ok(mut held) = the_searches_held().lock() {
        held.remove(server);
    }
}

/// **What a reading of this machine says.**
///
/// **Where a connection to the server stands, the machine is read** —
/// `Server Setup` S1, S2 and Y6a: fresh, on that connection, and never from
/// anything remembered — through [`crate::setting_up::the_setup_on`]. *(Added
/// 2026-09-21. Until then this answered *never read* for every server, which
/// was false while connected, and stayed false after a setup had run and
/// failed on it — the first pass on a real rental found it.)*
///
/// **Where none stands, the card is what was last read in this run**, greyed
/// and stamped (`Server Setup` Y6a; the 38's piece 6), and where nothing was,
/// the tab says nothing has been read.
fn what_the_machine_said<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
) -> WhatTheMachineSaid {
    let Some(point) = the_live_point(app, server) else {
        // **Connected, and nothing lent**: the connection is live and `Servers`
        // Q19 is re-taking step 5 on it, because Smoozy's service did not
        // answer — so the card says that, and not that nothing was ever read.
        use tauri::Manager;
        let periods =
            app.try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>();
        return what_was_last_seen_or(
            server,
            what_is_said_with_nothing_lent(periods.as_deref().map(|it| it.as_ref()), server),
        );
    };
    let said = match crate::setting_up::the_setup_on(app, &point, server) {
        Some(crate::setting_up::TheCardAsRead {
            the_run: Some(mut read),
            the_rest,
        }) => {
            // **§16.1 M3a names the clock a moment is on, and a person names a
            // machine by its server's name** — `AsOfThatMoment`'s own field
            // is *"one machine, named by the server row it was read from"*.
            // The run's record names its clock by the machine's installation
            // identity, which is right for the record and wrong for a screen:
            // step 10c's sixth pass read *on the clock of ad67db67…* on the
            // card. **So the moment is drawn on the clock of the server as the
            // list names it**, and where no name can be read nothing is named
            // rather than the identity.
            read.the_clock = the_servers_name(app, server).unwrap_or_default();
            match the_rest {
                Some(rest) => WhatTheMachineSaid::ASetupRunAndTheMachine(Box::new(read), Box::new(rest)),
                None => WhatTheMachineSaid::ASetupRunAlone(Box::new(read)),
            }
        }
        // **The machine answered the card's reading and holds no ready-action
        // record**: `Server Setup` Y6a's *never set up*, read rather than
        // inferred. A record it named and that could not be read is not this.
        Some(crate::setting_up::TheCardAsRead {
            the_run: None,
            the_rest: Some(rest),
        }) if rest.the_ready_actions_run.is_none() => WhatTheMachineSaid::AReading(Box::new(
            the_reading_where_no_setup_record_stands(
                &rest,
                the_servers_name(app, server).unwrap_or_default(),
            ),
        )),
        _ if crate::setting_up::the_machine_holds_no_installation(app, &point, server) => {
            WhatTheMachineSaid::ConnectedAndNeverSetUp
        }
        _ => WhatTheMachineSaid::ConnectedAndNoRunWasFound,
    };
    the_reading_is_held_as_last_seen(server, &said);
    said
}

/// **What was last read from each server in this run, with this computer's
/// own moment of reading** — `Server Setup` Y6a's last-known card, and the
/// 38's map, hole 7: **held in memory for the run, never written**, so after a
/// relaunch with no connection the card says the server has not been read
/// yet. The 38's piece 6.
fn the_readings_last_seen(
) -> &'static Mutex<std::collections::BTreeMap<String, (WhatTheMachineSaid, std::time::SystemTime)>>
{
    static HELD: std::sync::OnceLock<
        Mutex<std::collections::BTreeMap<String, (WhatTheMachineSaid, std::time::SystemTime)>>,
    > = std::sync::OnceLock::new();
    HELD.get_or_init(|| Mutex::new(std::collections::BTreeMap::new()))
}

/// **A reading taken over a connection is held as the one last seen**, in
/// place of whatever was held before.
pub fn the_reading_is_held_as_last_seen(server: &str, said: &WhatTheMachineSaid) {
    if let Ok(mut held) = the_readings_last_seen().lock() {
        held.insert(server.to_string(), (said.clone(), std::time::SystemTime::now()));
    }
}

/// **With no connection, what was last seen, where something was** — and
/// otherwise what was said already. `Server Setup` Y6a: *"With no connection
/// the card is last-known, greyed and stamped"*, and never blank.
pub fn what_was_last_seen_or(server: &str, said: WhatTheMachineSaid) -> WhatTheMachineSaid {
    if !matches!(said, WhatTheMachineSaid::NothingHasEverBeenReadFromIt) {
        return said;
    }
    the_readings_last_seen()
        .lock()
        .ok()
        .and_then(|held| held.get(server).cloned())
        .map(|(what_was_read, at)| WhatTheMachineSaid::LastSeen(Box::new(what_was_read), at))
        .unwrap_or(said)
}

/// **What the card is drawn from where no connection to the server is lent** —
/// *never read*, or, where the server reads connected and Smoozy's service on
/// it does not answer, that. **The two facts are the row's own**
/// (`Design/Servers § A row`), read the one way the row reads them. Added
/// 2026-09-27, step 10c's eighth-pass fix piece 6.
pub fn what_is_said_with_nothing_lent(
    periods: Option<&crate::servers::a_period::ThePeriodsThisRun>,
    server: &str,
) -> WhatTheMachineSaid {
    let not_answering = periods.is_some_and(|periods| {
        smoozy_servers::condition::presentation(periods.what_is_true_of(server))
            == smoozy_servers::condition::Presentation::Connected
            && periods.smoozy_does_not_answer_on(server)
    });
    if not_answering {
        WhatTheMachineSaid::ConnectedAndSmoozyIsNotResponding
    } else {
        WhatTheMachineSaid::NothingHasEverBeenReadFromIt
    }
}

/// **The server's name as the list draws it** — `Design/Servers`: by its
/// name, or by its username and host where it has none.
fn the_servers_name<R: tauri::Runtime>(app: &tauri::AppHandle<R>, server: &str) -> Option<String> {
    use tauri::Manager;
    let started = app.try_state::<std::sync::Mutex<crate::start::Started>>()?;
    let started = started.lock().ok()?;
    let rows = crate::record::server_rows::all(started.record.as_ref()?.connection()).ok()?;
    let row = rows.iter().find(|row| row.local_server_identifier == server)?;
    Some(crate::surface::servers::what_a_server_is_called_among(row, &rows))
}

/// **`Server Setup` B2's list of what Blender publishes.**
///
/// **This is the one outside fetch this module makes and it touches no
/// machine**, so it runs whole on any computer with a line to the internet.
/// **The mark beside each entry is B6a's and needs the machine's own answered
/// versions**, so where nothing has been read from the machine no entry carries
/// one — which B6a permits, nothing anywhere acting on the mark.
#[tauri::command]
pub async fn the_install_list(server: String) -> TheInstallList {
    let read = smoozy_server_setup::listing::the_list_smoozy_offers().await;
    let list = the_list(read, the_versions_answered_on(&server));
    the_entries_offered_are_held(&list);
    list
}

/// **What each entry the list offered is called**, keyed by the text its
/// install carries — so an install's ending names the build the way the list
/// did, *Blender 5.3.0 Alpha*, without reading anything off a file's name
/// (`Server Setup` B2). Period-held, replaced by every list composed, and
/// nothing durable.
fn the_entries_offered() -> &'static Mutex<std::collections::BTreeMap<String, String>> {
    static HELD: std::sync::OnceLock<Mutex<std::collections::BTreeMap<String, String>>> =
        std::sync::OnceLock::new();
    HELD.get_or_init(|| Mutex::new(std::collections::BTreeMap::new()))
}

fn the_entries_offered_are_held(list: &TheInstallList) {
    if let Ok(mut held) = the_entries_offered().lock() {
        held.clear();
        for entry in list.finished.iter().chain(list.unfinished.iter()) {
            let named = if list.unfinished.contains(entry) {
                format!("{} {}", entry.version.as_str(), entry.line.as_str())
            } else {
                entry.version.as_str().to_string()
            };
            held.insert(entry.asked.as_str().to_string(), named);
        }
    }
}

/// **The build an install asked for, named as the list named it** — the
/// version, and Blender's own word for an unfinished kind. Where the list that
/// offered it is not held, the version alone, read from the carried text.
pub fn the_build_asked_for_is_named(asked: &str) -> String {
    if let Some(named) = the_entries_offered().lock().ok().and_then(|held| held.get(asked).cloned()) {
        return named;
    }
    smoozy_server_setup::blender::TheVersionAsked::read(asked)
        .map(|it| format!("{}.{}.{}", it.version.major, it.version.minor, it.version.patch))
        .unwrap_or_else(|| asked.to_string())
}

/// **`Server Setup` B13 and B14: what removing a build costs, composed before
/// anything is removed.**
///
/// **The live jobs are this copy's own local records of that server** whose
/// ending is not learned — rendering where the render's start is learned,
/// waiting otherwise — each with the build path it was submitted with (`Server Setup` B13).
/// **The set the warning names is held**, so *Remove anyway* marks that same
/// set and never one composed a second time (`Queue & History` L21). The 38's
/// piece 14c; until then the warning named no job at all.
#[tauri::command]
pub fn what_removing_a_blender_costs<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    server: String,
    path: String,
) -> TheRemovalWarning {
    let version = the_version_of(&server, &path);
    let live = the_live_jobs_on(&app, &server);
    let (warning, covered) = the_removal_warning_and_its_set(&path, version, &live);
    let the_managers_package = smoozy_server_setup::blender::the_managed_package_at(&path);
    if let Ok(mut held) = the_removals_warned().lock() {
        held.insert(
            (server, path.clone()),
            crate::setting_up::TheRemovalAsked {
                executable: path,
                the_answered_version: version
                    .map(|it| format!("{}.{}.{}", it.major, it.minor, it.patch))
                    .unwrap_or_default(),
                covered,
                the_managers_package,
            },
        );
    }
    warning
}

/// **`Server Setup` B14's warning and the set it names, composed once together** — the jobs
/// named are exactly the jobs the removal will mark (`Queue & History` L21).
/// **The route is read off the path**: a build the machine's own software
/// manager holds under the one fixed package (`Server Setup`'s own
/// `blender::the_managed_package_at`) is that manager's, and every other is the
/// folder deletion (`Server Setup` B15). *(Until 2026-10-16 the folder deletion
/// was the route for every row, so a snap row's warning told a rendering job it
/// would finish — the folder route's observation, which `Server Setup` B14 does
/// not carry onto the manager's. Step 10c's owed piece 6.)*
pub fn the_removal_warning_and_its_set(
    path: &str,
    version: Option<Version>,
    live: &[(String, ALiveJob, smoozy_state::identities::Identity)],
) -> (TheRemovalWarning, Vec<smoozy_state::identities::Identity>) {
    let named: Vec<(String, ALiveJob)> =
        live.iter().map(|(name, job, _)| (name.clone(), job.clone())).collect();
    let the_managers_package = smoozy_server_setup::blender::the_managed_package_at(path);
    let warning = the_removal_warning(path, version, &named, the_managers_package);
    let covered_jobs = smoozy_server_setup::search::the_warning_names(
        &live.iter().map(|(_, job, _)| job.clone()).collect::<Vec<_>>(),
        path,
        match the_managers_package {
            Some(_) => RemovalRoute::TheMachinesOwnSoftwareManager,
            None => RemovalRoute::TheFolderDeletion,
        },
    );
    let mut covered = Vec::new();
    let mut taken = vec![false; live.len()];
    for job in &covered_jobs {
        if let Some(at) = (0..live.len()).find(|at| !taken[*at] && live[*at].1 == *job) {
            taken[at] = true;
            covered.push(live[at].2.clone());
        }
    }
    (warning, covered)
}

/// **The live jobs this copy holds for a server, as `Server Setup` B14 reads them** — each
/// job's name, its state and its recorded build path, and its identity.
pub fn the_live_jobs_of(
    records: &[smoozy_queue_history::record::LocalJobRecord],
) -> Vec<(String, ALiveJob, smoozy_state::identities::Identity)> {
    records
        .iter()
        .filter(|record| record.the_ending_learned().is_none())
        .map(|record| {
            (
                crate::surface::the_queue::what_the_job_is_called(record),
                ALiveJob {
                    state: if record.the_render_start_has_been_learned {
                        LiveState::Rendering
                    } else {
                        LiveState::Waiting
                    },
                    recorded_build_path: record.blender.pick.build_path.clone(),
                },
                record.job.clone(),
            )
        })
        .collect()
}

/// [`the_live_jobs_of`], read from this copy's record. **Nothing read is no
/// job named**, never a guess.
fn the_live_jobs_on<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
) -> Vec<(String, ALiveJob, smoozy_state::identities::Identity)> {
    use tauri::Manager;
    let Some(identity) = smoozy_state::identities::Identity::read_back(
        smoozy_state::identities::IdentityKind::LocalServer,
        server,
    ) else {
        return Vec::new();
    };
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return Vec::new();
    };
    let Ok(started) = started.lock() else {
        return Vec::new();
    };
    let Some(record) = started.record.as_ref() else {
        return Vec::new();
    };
    let records = crate::record::job_records::JobRecords::over(record.connection())
        .with_no_learned_ending_for(&identity)
        .unwrap_or_default();
    the_live_jobs_of(&records)
}

/// **The removal each warning composed, per server and build**, until *Remove
/// anyway* takes it. Period-held and never durable.
fn the_removals_warned(
) -> &'static Mutex<std::collections::BTreeMap<(String, String), crate::setting_up::TheRemovalAsked>> {
    static HELD: std::sync::OnceLock<
        Mutex<std::collections::BTreeMap<(String, String), crate::setting_up::TheRemovalAsked>>,
    > = std::sync::OnceLock::new();
    HELD.get_or_init(|| Mutex::new(std::collections::BTreeMap::new()))
}

/// **What *Remove anyway* asks for, or nothing where this application does not
/// perform the removal.** `Server Setup` B15's plain deletion is performed only
/// for a build under Smoozy's own Blender area (`Server Setup` B9), which is what the machine
/// itself refuses outside of. **Every other row** — the system package areas,
/// the snap mount, a folder unpacked by hand elsewhere — **keeps the sentence
/// it had**, that the act is not built: the software manager's route is a
/// later piece. The 38's piece 14c.
///
/// **From 2026-10-16 the software manager's route is built for the one
/// package `Server Setup` may name** (step 10c's owed piece 6): a row at one of
/// the snap's two observed paths asks for that manager's removal, carrying the
/// package and the same covered set the warning named. **The rest keep the
/// sentence** — the distribution's own package, for want of a removal command
/// in any plan and of a way to tell its path from a link, and a folder
/// unpacked by hand outside Smoozy's area.
pub fn what_remove_anyway_asks_for(
    path: &str,
    smoozys_own_blender_area: Option<&str>,
    warned: Option<crate::setting_up::TheRemovalAsked>,
) -> Option<crate::setting_up::TheRemovalAsked> {
    let asked_for = |the_managers_package: Option<&'static str>| {
        let mut asked = warned.unwrap_or(crate::setting_up::TheRemovalAsked {
            executable: path.to_string(),
            the_answered_version: String::new(),
            covered: Vec::new(),
            the_managers_package: None,
        });
        asked.the_managers_package = the_managers_package;
        asked
    };
    if let Some(package) = smoozy_server_setup::blender::the_managed_package_at(path)
        .filter(|package| smoozy_server_setup::search::this_module_can_remove_the_managed_build(package))
    {
        return Some(asked_for(Some(package)));
    }
    let area = smoozys_own_blender_area?;
    let under = path.strip_prefix(area)?.strip_prefix('/')?;
    if under.is_empty() || path.split('/').any(|part| part == ".." || part == ".") {
        return None;
    }
    Some(asked_for(None))
}

/// **`Server Setup` B15's removal, started off the drawing thread** (P5) and
/// answered when it ends — the Telegram placement's shape. **The Blender
/// table is read again when it ends** (`crate::setting_up::a_run_ends`). The
/// 38's piece 14c.
fn a_removal_is_driven<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    queue: &State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    path: Option<String>,
) {
    let own_area = the_blender_area_of(&app, &server);
    let warned = path.as_ref().and_then(|path| {
        the_removals_warned()
            .lock()
            .ok()
            .and_then(|mut held| held.remove(&(server.clone(), path.clone())))
    });
    let Some(removal) = path
        .as_deref()
        .and_then(|path| what_remove_anyway_asks_for(path, own_area.as_deref(), warned))
    else {
        // **A row neither route reaches says the act is not built**, in the
        // words it always had — the snap's own manager route is built from
        // 2026-10-16 (step 10c's owed piece 6), and the rest is not.
        the_queue_is_told(queue, the_answer_for(&server));
        return;
    };
    if the_live_point(&app, &server).is_none() {
        the_queue_is_told(queue, one_line_about(&server, words::NO_SERVER_IS_CONNECTED));
        return;
    }
    // **`Server Setup` Y9a item 6: pressed while a run stands, it waits its
    // turn**, carrying the set its warning named (item 9). Its ending is
    // [`a_removals_ending_saying`], told by the line.
    the_acts_are_pressed(app, queue.inner(), server, vec![crate::setting_up::ASetUpAct::ARemoval(removal)]);
}

/// The confirmed removal follows its answer, before the new search returns.
fn a_removals_ending(server: &str, path: &str, came_to: &crate::setting_up::WhereTheSetUpGot) -> Event {
    a_removals_ending_saying(server, path, came_to, what_the_removal_came_to(came_to))
}

/// **[`a_removals_ending`], with the sentence handed in** — so the line can
/// name the acts it stopped after it. Written apart 2026-10-27.
fn a_removals_ending_saying(
    server: &str,
    path: &str,
    came_to: &crate::setting_up::WhereTheSetUpGot,
    came: WhatItCameTo,
) -> Event {
    let answer = the_answer_about(server, &came);
    let answer = a_blender_runs_ending(server, answer, false);
    if matches!(came_to, crate::setting_up::WhereTheSetUpGot::TheRunEnded {
        ending: crate::setting_up::TheRunsEnding::ItReachedDone { .. }, ..
    }) {
        answer
            .saying("the_removed_blender", Said::Value(Verbatim::new(path.to_string())))
            .saying("no_blender_reading", Said::Prose(Prose::new(words::NO_BLENDER_READING)))
    } else { answer }
}

/// **What a removal says** (Rule 9): *Blender removed.*, or the same reasons
/// every setup run gives, the step's own failure sentence among them (*The
/// server could not remove Blender.*). The 38's piece 14c.
pub fn what_the_removal_came_to(came_to: &crate::setting_up::WhereTheSetUpGot) -> WhatItCameTo {
    use crate::setting_up::{TheRunsEnding, WhereTheSetUpGot};
    match came_to {
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::ItReachedDone { .. },
            ..
        } => WhatItCameTo::only(words::BLENDER_REMOVED),
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::TheUserStoppedIt,
            ..
        } => WhatItCameTo::only(words::SETUP_STOPPED),
        other => the_reason_in(other),
    }
}

/// **`Server Setup` B9's Blender area on that server**, composed from the
/// layout its row recorded — the same reading the search takes.
fn the_blender_area_of<R: tauri::Runtime>(app: &tauri::AppHandle<R>, server: &str) -> Option<String> {
    use tauri::Manager;
    let started = app.try_state::<Mutex<Started>>()?;
    let started = started.lock().ok()?;
    let rows = crate::record::server_rows::all(started.record.as_ref()?.connection()).ok()?;
    let row = rows.into_iter().find(|row| row.local_server_identifier == server)?;
    crate::servers::installation::the_layout_recorded_for(&row)
        .map(|layout| smoozy_server_setup::blender::the_blender_area(&layout))
}

/// **`Server Setup` E2's box, asked only at the moment a step is refused.**
///
/// **How many choices it offers is `Security and licence` S36's**, composed
/// from the secret store's standing — two where it reads *never answered*,
/// three otherwise. **The standing is read here and never assumed**; a record
/// that holds no row reads as the standing's own default, which is *never
/// probed* — **and a read that failed is not that**: it is `None`, and the box
/// offers what a store never proven is offered and says the standing could not
/// be read (must-do 32, `Security and licence` S25; the shared-layer Verdict's
/// F-10).
///
/// **Composed off the thread that draws** — `Security and licence` §3.0 rule 3
/// puts every store call off it, and the probe is a put, a get and an erase,
/// each of which may prompt on Linux. **A command that is not `async` runs on
/// the thread that draws**, which is what this crate says of itself, so this
/// one is `async` and hands the work to the shell's blocking pool, the way
/// [`crate::surface::keys`] takes its own store calls off it. *(Step 10b's
/// first audit round, item 13.)*
#[tauri::command]
pub async fn the_admin_access_box<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    the_step_that_was_refused: String,
    the_system_would_not_hand_the_password_back: bool,
) -> TheAdminAccessBox {
    let asked = the_step_that_was_refused.clone();
    tauri::async_runtime::spawn_blocking(move || {
        the_admin_access_box_is_composed(
            &app,
            &the_step_that_was_refused,
            the_system_would_not_hand_the_password_back,
        )
    })
    .await
    .unwrap_or_else(|_| the_box(&asked, None, the_system_would_not_hand_the_password_back))
}

/// **The box, composed on the blocking pool.**
fn the_admin_access_box_is_composed<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    the_step_that_was_refused: &str,
    the_system_would_not_hand_the_password_back: bool,
) -> TheAdminAccessBox {
    use tauri::Manager;
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return the_box(
            the_step_that_was_refused,
            None,
            the_system_would_not_hand_the_password_back,
        );
    };
    // **`Security and licence` S36: the offer is composed *after* the probe.**
    // A machine password is about to be asked for, which is a moment in this run
    // when a secret is actually needed — `Security and licence` S26 — and the
    // standing this box reads
    // is only worth reading once something has written it. **The record is lent
    // for each read rather than held across the probe**, because reaching the
    // store may prompt (L4).
    let lent = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let store = crate::surface::keys::the_store_for_this_system();
    // **`Security and licence` S34's sentence is said here where this box is
    // the first act in the run to find there is nowhere to keep a password** —
    // `Preparation/Design/Keys.md`, *On a computer with nowhere to keep a
    // passphrase*, above the field (mockup state `pw-nostore`). The writer
    // hands the notice to the first caller alone, so this is the one time.
    let the_sentence_is_owed = crate::the_store_standing::at_the_first_use_of_a_secret(
        &crate::the_store_standing::THIS_RUN,
        &lent,
        crate::surface::keys::the_store(&store),
        crate::surface::keys::THE_STORE_DEADLINE,
    )
    .is_some_and(|write| write.say_it_once);
    let mut standing = None;
    crate::keys::LendsTheRecord::lend(&lent, &mut |connection| {
        standing = the_store_standing(connection);
    });
    let mut composed = the_box(
        the_step_that_was_refused,
        standing,
        the_system_would_not_hand_the_password_back,
    );
    composed.nowhere_to_keep_them = the_sentence_is_owed
        .then(|| Prose::new(crate::surface::keys::NOWHERE_TO_KEEP_THEM));
    composed
}

/// **Every act on this tab, answered.** *(The command keeps its name, which is
/// older than what it now does: the page calls it by name, and the wording it
/// answered with changed on 2026-09-18, at step 10c piece 9 and on 2026-09-21.)*
///
/// - **The ready action** — `Server Setup` R2's whole order, driven by
///   [`crate::setting_up`]. **Built at step 10c piece 9.**
/// - **The stop** — `Server Setup` Y8's, honoured at a boundary and never by a
///   signal. Built with it.
/// - **Installing a Blender picked from the list** — `Server Setup` Y6b's own
///   kind of run, with the version the user picked (`version`). **Built
///   2026-09-21**; until then it answered that the act was not built.
/// - Setting up again answers as the ready action, which is what it is.
/// - **Abandoning a stopped setup** — `Server Setup` S6's second clear, driven
///   by [`crate::setting_up::a_setup_is_abandoned`]. **Built 2026-09-22**; until
///   then it answered that the act was not built.
/// - **Installing Telegram's material** — `Server Setup` G2's dedicated entry
///   point, *Install it* on the card, driven by
///   [`crate::setting_up::the_telegram_placement_on`]. **Wired 2026-10-04**;
///   until then it answered that the act was not built.
/// - **Removing a Blender** — `Server Setup` B15's plain deletion, a run of
///   `Server Setup` Y6b's own kind, for a build under Smoozy's own Blender area. **Built
///   2026-10-10, the 38's piece 14c.** A build anywhere else — the software
///   manager's route — **still answers the one sentence this application says
///   for an act nobody has built**
///   ([`crate::surface::files::A_CONNECTION_IS_NOT_BUILT`]). **Trying an
///   elevated step again is not an act of this command any more**: it is
///   [`the_admin_access_is_answered`], which hands the password to the run that
///   is waiting for it.
///
/// **Each is answered rather than met with silence** — Rule 8 makes Smoozy
/// going quiet a fault however the trouble arose, and a control that does
/// nothing twice is the fault F-10 repaired in the failure shape.
#[tauri::command]
pub fn a_set_up_act_that_needs_a_connection<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    act: String,
    server: String,
    version: Option<String>,
    waiting: Option<Vec<String>>,
    path: Option<String>,
) {
    match (act.as_str(), version) {
        (THE_READY_ACTION, _) => the_ready_action_is_driven(app, &queue, server, false),
        // **`Server Setup` W3's *Set up anyway***: the warning has been shown
        // and the user went ahead, so it is not shown again.
        (THE_READY_ACTION_AFTER_THE_WARNING, _) => {
            the_ready_action_is_driven(app, &queue, server, true)
        }
        // **The stop names the run it is stopping.** Added 2026-09-23, step
        // 10c's fifth pass on Vast: *Stop* on a **Blender install**'s own box
        // answered *Setup stopped.*, which is a sentence about a different act.
        // The kind is the one this copy is driving on that server; **where it
        // is driving none, nothing is said about a run that is not there** and
        // the stop is written down all the same — `Server Setup` Y8's stop is
        // honoured at a boundary, not at the press.
        //
        // **Since 2026-10-02 it says nothing of its own at the press** (the
        // Creator's notes piece 2): the run is on the card, so the card is
        // asked again and draws *Cancel setup* under way until the run reaches
        // the boundary and ends, and the run's own ending says *Setup stopped.*
        // or *Install stopped.* — one sentence, not that sentence twice.
        (STOP, _) => {
            crate::setting_up::the_user_asked_to_stop(&server);
            the_queue_is_told(&queue, the_run_moved_on(&server));
        }
        // **`Server Setup` B2's pick, installed.** The version is the entry's
        // own text as the list gave it; nothing else about the entry travels,
        // and `Server Setup` B4 derives every address from it on the machine.
        // **Several picked install one after another, each its own run**
        // (`Specs/Server Setup`, the Creator's third notes, note 2): the first
        // is `version` and the rest wait behind it, in the order the list
        // showed them.
        (INSTALL_A_BLENDER, Some(version)) => {
            a_blender_install_is_driven(app, &queue, server, version, waiting.unwrap_or_default())
        }
        (ABANDON, _) => an_abandon_is_driven(app, &queue, server),
        // **`Server Setup` B7: the user's own build.** Added 2026-10-09, the
        // 38's piece 14b; until then the link answered that it was not built.
        (CHOOSE_A_BUILD, _) => a_build_is_chosen(app, &queue, server),
        // **`Server Setup` B7a's answer goes back to the run that waits on it**,
        // and the card is asked again.
        (INSTALL_ANYWAY, _) | (DONT_INSTALL_IT, _) => {
            if crate::setting_up::the_question_is_answered(&server, act == INSTALL_ANYWAY) {
                the_queue_is_told(&queue, the_run_moved_on(&server));
            }
        }
        // **`Server Setup` G2's dedicated entry point — *Install it*.** Built
        // by the 38's piece 11 and wired 2026-10-04, once the machine's final
        // check stopped ending the Telegram placement's run (`Audits/Build/Step
        // 10c/The setup's final check and Install Telegram — what landed.md`).
        (INSTALL_TELEGRAM, _) => the_telegram_placement_is_driven(app, &queue, server),
        // **`Server Setup` B15's *Remove anyway***, with the build's path.
        // Added 2026-10-10, the 38's piece 14c; until then it answered that
        // the act was not built — which a build outside Smoozy's own Blender
        // area still answers.
        (REMOVE_A_BLENDER, _) => a_removal_is_driven(app, &queue, server, path),
        // **The card's *Connect and set up* is the row's own act**
        // (`Servers` B6), handed to `Servers` whole and answered on its own
        // event. The Creator's note 14 on the test copy.
        (CONNECT_AND_SET_UP, _) => {
            if let Some(answer) = crate::surface::servers::the_act_is_performed(&app, &act, &server) {
                crate::surface::servers::the_answer_is_said(&app, answer);
            }
        }
        _ => {
            let event = the_answer_for(&server);
            let _ =
                queue.read_then_post::<(), std::convert::Infallible>(|| Ok(()), |_| event.clone());
        }
    }
}

/// **The ready action, started off the drawing thread** (P5) and answered
/// when it ends — `Server Setup` R2's whole order on a machine never prepared,
/// and `Server Setup` R1's run alone on one this copy already set up
/// ([`crate::setting_up::a_machine_is_set_up`], step 10c's ninth-pass fix
/// piece 3).
///
/// **The run is started only where a connection to that server stands**, which
/// is `Servers` S9's rule that the acts of its own tables are the ones that
/// reach a machine: this act does not open one.
///
/// **`Server Setup` W3's warning is shown first where the table says it warns**
/// — `Design/Server Setup § A long step`: *"A step that could disturb work
/// warns first"*. It is judged off the drawing thread, because it reads the
/// machine ([`the_warning_owed_before_the_ready_action`]); **where it is owed
/// the run does not start**, the page is told to ask for it, and its *Set up
/// anyway* asks for the act again as
/// [`THE_READY_ACTION_AFTER_THE_WARNING`]. **Where the warning named a render
/// running, the run carries that** — the one reading the running panel's render
/// sentence is drawn on (`Server Setup` T6a, T7). *(Added 2026-09-27: until
/// then the warning was composed and tested and nothing called it, so a setup
/// restarted a server with work on it without a word.)*
///
/// **The control carries the mark from the press until the warning's reading
/// answers** — `Design/Whole app § Anything that isn't instant shows that it's
/// underway`, *on the control*. The page is told before the reading is taken
/// ([`the_press_is_marked_while`]), and whatever answers the press next takes
/// the mark off. *(Added 2026-09-28, step 10c's ninth-pass fix piece 4: the
/// ninth pass on Hyperstack pressed *Set up* with a job rendering and saw
/// nothing change for 11 seconds.)*
fn the_ready_action_is_driven<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    queue: &State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    after_the_warning: bool,
) {
    let Some(point) = the_live_point(&app, &server) else {
        the_queue_is_told(queue, one_line_about(&server, words::NO_SERVER_IS_CONNECTED));
        return;
    };
    // **`Server Setup` Y9a item 6: pressed while a run stands, it waits its
    // turn** rather than being refused — and W3's warning is judged at the
    // press all the same (item 9). Changed 2026-10-27, the nineteenth notes,
    // note 2: until then a second press was refused here, and the page wiped
    // the refusal at once.
    let told = std::sync::Arc::clone(queue.inner());
    std::thread::spawn(move || {
        let a_render_was_running = if after_the_warning {
            the_warning_was_gone_past(&server)
        } else {
            let owed = the_press_is_marked_while(&told, &server, || {
                the_warning_owed_before_the_ready_action(&app, &point, &server)
            });
            match owed {
                Some(owed) => {
                    the_warning_is_held(&server, owed);
                    the_queue_is_told(&told, the_warning_is_owed_on(&server));
                    return;
                }
                None => false,
            }
        };
        // **`Server Setup` Y9a: a fresh setup ends the line about installs an
        // earlier run closed before it began**, by keeping nothing — or joins
        // the line where a run stands. The run itself is
        // [`the_act_is_performed`]'s.
        the_acts_are_pressed(
            app,
            &told,
            server,
            vec![crate::setting_up::ASetUpAct::TheReadyAction { a_render_was_running }],
        );
    });
}

/// **A Blender install picked from the list, started off the drawing thread**
/// (P5) and answered when it ends. Added 2026-09-21.
///
/// **The same two gates the ready action has**: a connection must stand, and
/// `Server Setup` Y9's one run at a time holds for every kind of setup run.
///
/// **Several picked install one after another** — `Specs/Server Setup`,
/// Creator-stated 2026-10-06 (the third notes, note 2): `waiting` are the ones
/// picked after `version`, in the order the list showed them, and each is its
/// own run, begun when the last one's run ends
/// ([`the_installs_are_driven_one_after_another`]). **Where none could begin,
/// the line says why and names every one picked**, none of them having been
/// installed.
fn a_blender_install_is_driven<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    queue: &State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    version: String,
    waiting: Vec<String>,
) {
    let all_of_them = || std::iter::once(version.clone()).chain(waiting.iter().cloned()).collect::<Vec<_>>();
    let none_began = |said: &str| {
        let named = if waiting.is_empty() { Vec::new() } else { all_of_them() };
        the_answer_about(&server, &the_ones_not_installed_are_named(WhatItCameTo::only(said), &named))
    };
    if the_live_point(&app, &server).is_none() {
        the_queue_is_told(queue, none_began(words::NO_SERVER_IS_CONNECTED));
        return;
    }
    // **`Server Setup` Y9a: each pick is its own act of the line**, kept in
    // the local record as well as in memory, so a Smoozy that closes before
    // they are begun names them when it opens again — **and every one joins
    // the line where a run already stands** (item 6, 2026-10-27).
    let acts = std::iter::once(version)
        .chain(waiting)
        .map(crate::setting_up::ASetUpAct::AnInstallFromTheList)
        .collect();
    the_acts_are_pressed(app, queue.inner(), server, acts);
}

/// **`Server Setup` B7: a build on the user's own computer, picked and
/// installed.** Added 2026-10-09, the 38's piece 14b.
///
/// **The pick is the system's own file picker** (K11 to K13), put up off the
/// drawing thread. **Nothing picked is nothing done.** Then the same gates a
/// Blender from the list meets — a connection standing and `Server Setup` Y9's
/// one run at a time — and the run of
/// [`crate::setting_up::the_users_build_install_on`], whose ending is said the
/// way a list install's is.
fn a_build_is_chosen<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    queue: &State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
) {
    let told = std::sync::Arc::clone(queue.inner());
    std::thread::spawn(move || {
        let asking = crate::picker::Asking {
            what: crate::picker::WhatMayBeChosen::Files,
            how_many: crate::picker::HowMany::One,
            starting_at: None,
            title: Prose::new(words::CHOOSE_A_BLENDER_BUILD),
        };
        let chosen = match crate::picker::ask(&app, &asking) {
            crate::picker::Picked::TheUserChose(paths) => paths
                .into_iter()
                .next()
                .map(|path| path.as_the_user_gave_it().to_path_buf()),
            _ => None,
        };
        let Some(chosen) = chosen else { return };
        if the_live_point(&app, &server).is_none() {
            the_queue_is_told(&told, one_line_about(&server, words::NO_SERVER_IS_CONNECTED));
            return;
        }
        // **`Server Setup` Y9a: an install of the user's own build replaces
        // what an earlier run left**, with nothing waiting behind it — **or
        // joins the line where a run stands**, the file picked now and carried
        // (item 6, 2026-10-27).
        the_acts_are_pressed(app, &told, server, vec![crate::setting_up::ASetUpAct::TheUsersBuild(chosen)]);
    });
}

/// **`Server Setup` R1b and `Server Setup` B8: once an install that published a build
/// reaches *done*, by either route, the comparison follows** — the version the
/// published build answered, read by the search beside every other build, so
/// the bar is still the newest Blender's. **The search is the Set up tab's own
/// and is held**, so the table drawn after the install reads it rather than
/// taking it a second time. Where a swap is owed and nothing is live, it is
/// made at once by a run of its own; otherwise it waits. **Nothing where no
/// build was published** — a build declined at the version answer (`Server
/// Setup` B7a) was never published and is not compared. Added 2026-10-12.
fn the_driver_follows_the_install<R: tauri::Runtime>(app: &tauri::AppHandle<R>, server: &str, published: bool) {
    if published {
        let _ = the_blender_search_now(app, server);
    }
}

/// **`Server Setup` Y9a item 6: a press on the Set up card, taken** — begun
/// at once where no run stands on that server, and appended to the line where
/// one does, rather than refused (the nineteenth notes, note 2, 2026-10-27:
/// a removal pressed while an install ran was refused and its line wiped).
/// **Either way the tab is told to ask for the run**, whose panel now lists
/// what waits, so the press is answered at once. Where it began, the line is
/// driven off the drawing thread (P5).
fn the_acts_are_pressed<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    told: &std::sync::Arc<TheOneQueue>,
    server: String,
    acts: Vec<crate::setting_up::ASetUpAct>,
) {
    let Some(first) = acts.first().cloned() else {
        return;
    };
    let taken = crate::setting_up::the_acts_are_taken_from_the_card(&server, acts, &mut |line| {
        the_acts_waiting_are_kept(&app, &server, line)
    });
    the_queue_is_told(told, the_run_moved_on(&server));
    crate::surface::servers::the_work_column_is_told_again(&app);
    if taken == crate::setting_up::HowThePressWasTaken::ItBegan {
        let told = std::sync::Arc::clone(told);
        std::thread::spawn(move || the_line_is_driven(&app, &told, &server, first));
    }
}

/// **The next act of a line, begun by a run that does not drive the line
/// itself** — the add's own setup and an abandon — told and driven off this
/// thread (P5). Added 2026-10-27.
fn the_next_act_is_driven<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    next: crate::setting_up::ASetUpAct,
) {
    use tauri::Manager;
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        // **Nothing can be told, so nothing is begun**, and the line goes.
        crate::setting_up::a_run_ends(server);
        let _ = crate::setting_up::the_line_stopped_on(server);
        the_acts_waiting_are_kept(app, server, &[]);
        return;
    };
    let told = std::sync::Arc::clone(queue.inner());
    the_queue_is_told(&told, the_run_moved_on(server));
    crate::surface::servers::the_work_column_is_told_again(app);
    let (app, server) = (app.clone(), server.to_string());
    std::thread::spawn(move || the_line_is_driven(&app, &told, &server, next));
}

/// **What follows a run that does not drive the line itself, kept** — the
/// local record written as the line now stands — and handed back as the acts
/// it stopped, to name, and the next act, to begin once the ending is told.
fn the_line_follows<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    follows: crate::setting_up::WhatFollowsARun,
) -> (Vec<crate::setting_up::ASetUpAct>, Option<crate::setting_up::ASetUpAct>) {
    use crate::setting_up::WhatFollowsARun;
    match follows {
        WhatFollowsARun::TheNextBegins(next) => {
            the_acts_waiting_are_kept(app, server, &crate::setting_up::the_acts_still_waiting_on(server));
            (Vec::new(), Some(next))
        }
        WhatFollowsARun::NothingWaits => {
            the_acts_waiting_are_kept(app, server, &[]);
            (Vec::new(), None)
        }
        WhatFollowsARun::TheseWereNotBegun(stopped) => {
            the_acts_waiting_are_kept(app, server, &[]);
            (stopped, None)
        }
    }
}

/// **The line driven from its first act to its end on this thread** — each
/// act performed by [`the_act_is_performed`], its ending told, and the next
/// begun ([`the_acts_are_driven_in_turn`]). **Once it ends, `Server Setup`
/// R1b's comparison follows where any install published a build**, as it
/// followed a chain of installs. Added 2026-10-27, the nineteenth notes,
/// note 2.
fn the_line_is_driven<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    told: &std::sync::Arc<TheOneQueue>,
    server: &str,
    first: crate::setting_up::ASetUpAct,
) {
    let mut perform = |act: &crate::setting_up::ASetUpAct| the_act_is_performed(app, server, act);
    let mut is_told = |event: Event| {
        the_queue_is_told(told, event);
        crate::surface::servers::the_work_column_is_told_again(app);
    };
    let mut keep = |line: &[crate::setting_up::ASetUpAct]| the_acts_waiting_are_kept(app, server, line);
    let published = the_acts_are_driven_in_turn(
        server,
        first,
        &mut perform,
        &mut the_jobs_in_the_record(app),
        &mut is_told,
        &mut keep,
    );
    the_driver_follows_the_install(app, server, published);
}

/// **One act's run, performed on the server's point as it stands now** — so
/// an act begun after the connection went says so rather than driving a
/// connection that is not there. Each is the run its own control always
/// began; only the moment it begins moved (`Server Setup` Y9a item 6).
fn the_act_is_performed<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    act: &crate::setting_up::ASetUpAct,
) -> crate::setting_up::WhereTheSetUpGot {
    use crate::setting_up::{ASetUpAct, WhereTheSetUpGot};
    let Some(point) = the_live_point(app, server) else {
        return WhereTheSetUpGot::TheConnectionWent;
    };
    match act {
        ASetUpAct::AnInstallFromTheList(version) => {
            crate::setting_up::a_blender_install_on(app, &point, server, version.clone())
        }
        ASetUpAct::TheUsersBuild(file) => {
            crate::setting_up::the_users_build_install_on(app, &point, server, file.clone())
        }
        ASetUpAct::ARemoval(asked) => crate::setting_up::the_removal_on(app, &point, server, asked.clone()),
        ASetUpAct::TheReadyAction { a_render_was_running } => {
            // **`Server Setup` T6a: the warning shown at the press named a
            // render**, and the panel's render sentence is drawn on it.
            if *a_render_was_running {
                crate::setting_up::the_run_began_with_a_render_running(server);
            }
            // **`Server Setup` W1: whether this run placed Smoozy's program is
            // the row's confirmed installation before and after it.**
            let confirmed_before = the_installation_the_row_confirms(app, server);
            // **The card's own action asks for no Blender** — `Server Setup`
            // Y6b: a Blender picked in the Blender section is a run of its
            // own kind.
            let came_to = crate::setting_up::the_ready_action_on(app, &point, server, None);
            the_ready_action_ended_on(
                server,
                confirmed_before.as_deref(),
                the_installation_the_row_confirms(app, server).as_deref(),
            );
            came_to
        }
        ASetUpAct::TheTelegramPlacement => {
            let came_to = crate::setting_up::the_telegram_placement_on(app, &point, server);
            // **Settings' *not reached* list is drawn from the told-record**,
            // which this run moved (`Telegram` L3).
            if let (Some(started), Some(queue)) = (
                tauri::Manager::try_state::<Mutex<Started>>(app),
                tauri::Manager::try_state::<std::sync::Arc<TheOneQueue>>(app),
            ) {
                let held = started.lock();
                if let Ok(held) = held.as_deref() {
                    crate::surface::telegram::the_section_is_posted(held, &queue, None);
                }
            }
            came_to
        }
    }
}

/// **`Server Setup` Y9a item 7: whether an act's run finished, so the next may
/// begin** — *done*, and for a Blender install from the list its Blender
/// published (Row 526's rule). A user's own build declined at B7a is *done*:
/// the decline was the user's own answer. A restart is asked apart
/// ([`crate::setting_up::the_run_restarted_the_server`]).
pub fn the_act_finished(act: &crate::setting_up::ASetUpAct, came_to: &crate::setting_up::WhereTheSetUpGot) -> bool {
    use crate::setting_up::{ASetUpAct, TheRunsEnding, WhereTheSetUpGot};
    match came_to {
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::ItReachedDone { a_blender_was_published, .. },
            ..
        } => !matches!(act, ASetUpAct::AnInstallFromTheList(_)) || *a_blender_was_published,
        _ => false,
    }
}

/// **The acts of a line, driven in turn on a server whose first run is
/// already taken** — each its own run of `Server Setup` Y6b's kind, the next
/// begun only when the last one finished, no stop was asked and no restart was
/// issued ([`crate::setting_up::a_run_in_the_line_ends`], `Server Setup` Y9a
/// item 7). Added 2026-10-08 for installs picked together, the Creator's third
/// notes, piece H; every card act from 2026-10-27, the nineteenth notes,
/// note 2.
///
/// **Each act's ending is said as its own run's always was**
/// ([`the_acts_ending`]), with the acts it stopped named after it.
///
/// **What comes back is whether any install published a build**, which is
/// what `Server Setup` R1b's comparison after an install is taken on once the
/// line has let the server go. Added 2026-10-12.
///
/// **`keep` is `Server Setup` Y9a's write**, handed the acts still waiting
/// each time that changes: the ones after the next, when the next begins, and
/// nothing when the line ends — every ending already names its own on the
/// card. **So the list in the local record is only ever one that a Smoozy
/// closed before it could begin.** Added 2026-10-16, step 10c's owed piece 4.
pub fn the_acts_are_driven_in_turn(
    server: &str,
    first: crate::setting_up::ASetUpAct,
    perform: &mut dyn FnMut(&crate::setting_up::ASetUpAct) -> crate::setting_up::WhereTheSetUpGot,
    what_a_job_is_called: &mut dyn FnMut(&smoozy_state::identities::Identity) -> Option<String>,
    told: &mut dyn FnMut(Event),
    keep: &mut dyn FnMut(&[crate::setting_up::ASetUpAct]),
) -> bool {
    use crate::setting_up::{TheRunsEnding, WhatFollowsARun, WhereTheSetUpGot};
    let mut act = first;
    let mut any_was_published = false;
    loop {
        let came_to = perform(&act);
        any_was_published |= matches!(
            came_to,
            WhereTheSetUpGot::TheRunEnded {
                ending: TheRunsEnding::ItReachedDone { a_blender_was_published: true, .. },
                ..
            }
        );
        // **Asked while the run is still held**, since its ending lets the
        // reading go.
        let restarted = crate::setting_up::the_run_restarted_the_server(server);
        let follows =
            crate::setting_up::a_run_in_the_line_ends(server, the_act_finished(&act, &came_to) && !restarted);
        let stopped = match &follows {
            WhatFollowsARun::TheseWereNotBegun(stopped) => stopped.clone(),
            _ => Vec::new(),
        };
        told(the_acts_ending(server, &act, &came_to, &stopped, restarted, what_a_job_is_called));
        match follows {
            WhatFollowsARun::TheNextBegins(next) => {
                keep(&crate::setting_up::the_acts_still_waiting_on(server));
                told(the_run_moved_on(server));
                act = next;
            }
            _ => {
                keep(&[]);
                return any_was_published;
            }
        }
    }
}

/// **One act's ending, as its own run's always was, with the acts it stopped
/// named after it.** Added 2026-10-27, the nineteenth notes, note 2.
///
/// **Where only installs from the list were waiting behind an install from
/// the list, the line reads as it always has** — `Design/Server Setup §
/// Installing`: *Not installed: Blender 5.2.2, Blender 4.5.14.*, the run's
/// own build named too where it did not install and its sentence does not
/// already name it, so with several picked the line never leaves the user to
/// work out which stopped. **Otherwise the acts stopped are named as acts**
/// ([`the_acts_not_started_are_named`]).
fn the_acts_ending(
    server: &str,
    act: &crate::setting_up::ASetUpAct,
    came_to: &crate::setting_up::WhereTheSetUpGot,
    stopped: &[crate::setting_up::ASetUpAct],
    restarted: bool,
    what_a_job_is_called: &mut dyn FnMut(&smoozy_state::identities::Identity) -> Option<String>,
) -> Event {
    use crate::setting_up::{ASetUpAct, TheRunsEnding, WhereTheSetUpGot};
    let installed = matches!(
        came_to,
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::ItReachedDone { a_blender_was_published: true, .. },
            ..
        }
    );
    let only_installs_waited = matches!(act, ASetUpAct::AnInstallFromTheList(_))
        && stopped.iter().all(|one| matches!(one, ASetUpAct::AnInstallFromTheList(_)));
    let not_started: &[ASetUpAct] = if only_installs_waited { &[] } else { stopped };
    match act {
        ASetUpAct::AnInstallFromTheList(version) => {
            let mut not_installed = Vec::new();
            if !stopped.is_empty() {
                let its_sentence_names_it = matches!(
                    came_to,
                    WhereTheSetUpGot::TheRunEnded {
                        ending: TheRunsEnding::AStepFailed { step: smoozy_server_setup::ready_action::Step::Fetch, .. },
                        ..
                    }
                );
                if !installed && !its_sentence_names_it {
                    not_installed.push(version.clone());
                }
                if only_installs_waited {
                    not_installed.extend(stopped.iter().filter_map(|one| match one {
                        ASetUpAct::AnInstallFromTheList(version) => Some(version.clone()),
                        _ => None,
                    }));
                }
            }
            let came = the_acts_not_started_are_named(
                the_ones_not_installed_are_named(
                    the_jobs_are_named(
                        what_the_install_came_to(came_to, &the_build_asked_for_is_named(version)),
                        came_to,
                        what_a_job_is_called,
                    ),
                    &not_installed,
                ),
                not_started,
                restarted,
            );
            an_installs_ending_is_kept_on_its_step(server, came_to, &came);
            // **A Blender installed is said by the table that lists it**, so
            // the card carries no line for it — unless the line names others.
            let the_card_says_it = installed && not_installed.is_empty() && not_started.is_empty();
            a_blender_runs_ending(server, the_answer_about(server, &came), the_card_says_it)
        }
        ASetUpAct::TheUsersBuild(path) => {
            let file = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            let came = the_acts_not_started_are_named(
                the_users_build_not_installed_is_named(
                    the_jobs_are_named(what_the_install_came_to(came_to, &file), came_to, what_a_job_is_called),
                    came_to,
                    &file,
                    !installed,
                ),
                not_started,
                restarted,
            );
            an_installs_ending_is_kept_on_its_step(server, came_to, &came);
            a_blender_runs_ending(server, the_answer_about(server, &came), installed && not_started.is_empty())
        }
        ASetUpAct::ARemoval(asked) => a_removals_ending_saying(
            server,
            &asked.executable,
            came_to,
            the_acts_not_started_are_named(what_the_removal_came_to(came_to), not_started, restarted),
        ),
        ASetUpAct::TheReadyAction { .. } => {
            let came = the_acts_not_started_are_named(
                the_jobs_are_named(what_it_came_to(came_to), came_to, what_a_job_is_called),
                not_started,
                restarted,
            );
            a_runs_ending(
                the_answer_about(server, &came),
                the_card_says_the_ready_actions_ending(came_to) && not_started.is_empty(),
            )
        }
        ASetUpAct::TheTelegramPlacement => {
            let came = the_acts_not_started_are_named(
                what_the_telegram_placement_came_to(came_to),
                not_started,
                restarted,
            );
            a_runs_ending(
                the_answer_about(server, &came),
                the_card_says_the_ready_actions_ending(came_to) && not_started.is_empty(),
            )
        }
    }
}

/// **A failed install's own sentence is kept for its step**
/// ([`the_installs_failure_is_said_on_its_step`]): the line's, after *Install
/// stopped.*, which the stopped card's heading already says.
fn an_installs_ending_is_kept_on_its_step(
    server: &str,
    came_to: &crate::setting_up::WhereTheSetUpGot,
    came: &WhatItCameTo,
) {
    an_installs_failure_is_kept(
        server,
        the_step_its_failure_is_recorded_on(came_to).and_then(|step| {
            let said = came.said.strip_prefix(words::INSTALL_STOPPED)?.trim_start();
            (!said.is_empty()).then(|| (step, said.to_string()))
        }),
    );
}

/// **The ones not installed, named after the line** — `Design/Server Setup §
/// Installing`: *Not installed: Blender 5.2.2, Blender 4.5.14.* Each is named
/// as the list named it. Nothing where none is owed. Added 2026-10-08, the
/// Creator's third notes, piece H.
pub fn the_ones_not_installed_are_named(came: WhatItCameTo, not_installed: &[String]) -> WhatItCameTo {
    if not_installed.is_empty() {
        return came;
    }
    let named = not_installed
        .iter()
        .map(|asked| format!("{} {}", words::BLENDER, the_build_asked_for_is_named(asked)))
        .collect::<Vec<_>>()
        .join(", ");
    WhatItCameTo {
        said: format!("{} {} {named}.", came.said, words::NOT_INSTALLED_THESE),
        ..came
    }
}

/// **A user's own build that was not installed, named as theirs by its file
/// name** — the build runner's ruling on its words, `Design/Server Setup § Pointing at a
/// build on your own computer`: *Your build blender-4.2-linux.tar.xz was not
/// installed.* After the run's own sentence where it failed or was stopped;
/// alone where it reached *done* without publishing it, which is the user's
/// own *Don't install it* and has nothing else to say. Nothing where it was
/// installed. Added 2026-10-16, step 10c's owed piece 5.
pub fn the_users_build_not_installed_is_named(
    came: WhatItCameTo,
    came_to: &crate::setting_up::WhereTheSetUpGot,
    file: &str,
    it_was_not_installed: bool,
) -> WhatItCameTo {
    if !it_was_not_installed {
        return came;
    }
    let line = format!("{} {file} {}", words::YOUR_BUILD_NAMED, words::WAS_NOT_INSTALLED);
    let declined = matches!(
        came_to,
        crate::setting_up::WhereTheSetUpGot::TheRunEnded {
            ending: crate::setting_up::TheRunsEnding::ItReachedDone { .. },
            ..
        }
    );
    WhatItCameTo {
        said: if declined { line } else { format!("{} {line}", came.said) },
        ..came
    }
}

/// **`Server Setup` G2's dedicated entry point, started off the drawing
/// thread** (P5) and answered when it ends — *Install it* on the card. The 38's
/// piece 11, wired 2026-10-04 ([`a_set_up_act_that_needs_a_connection`]) once
/// the machine's final check read the run's composed list beside its marker.
///
/// **The same two gates every setup run has**: a connection must stand, and
/// `Server Setup` Y9's one run at a time holds, the Telegram placement being a
/// setup run of its own kind (`Server Setup` Y6b). **Where it reached *done*
/// the card says it**: its list reads *Telegram reporting installed* from the
/// role's own reading, so no second line is drawn.
fn the_telegram_placement_is_driven<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    queue: &State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
) {
    if the_live_point(&app, &server).is_none() {
        the_queue_is_told(queue, one_line_about(&server, words::NO_SERVER_IS_CONNECTED));
        return;
    }
    // **`Server Setup` Y9a item 6: pressed while a run stands, it waits its
    // turn.** The run and its ending are the line's
    // ([`the_act_is_performed`], [`the_acts_ending`]).
    the_acts_are_pressed(app, queue.inner(), server, vec![crate::setting_up::ASetUpAct::TheTelegramPlacement]);
}

/// **What the Telegram placement says** (Rule 9). Reaching *done* is said by
/// the card's own list; anything else is the same reasons every setup run
/// gives, the step's failure sentence among them (*The server could not
/// install Telegram reporting.*). The 38's piece 11.
pub fn what_the_telegram_placement_came_to(came_to: &crate::setting_up::WhereTheSetUpGot) -> WhatItCameTo {
    use crate::setting_up::{TheRunsEnding, WhereTheSetUpGot};
    match came_to {
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::ItReachedDone { .. },
            ..
        } => WhatItCameTo::only(format!("{}.", words::TELEGRAM_INSTALLED)),
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::TheUserStoppedIt,
            ..
        } => WhatItCameTo::only(words::SETUP_STOPPED),
        other => the_reason_in(other),
    }
}

/// **`Server Setup` S6's abandon, started off the drawing thread** (P5) and
/// answered when it ends. Added 2026-09-22.
///
/// **The same two gates the ready action has**: a connection must stand, and
/// `Server Setup` Y9's one run at a time holds, an abandon being a setup run.
fn an_abandon_is_driven<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    queue: &State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
) {
    let Some(point) = the_live_point(&app, &server) else {
        the_queue_is_told(queue, one_line_about(&server, words::NO_SERVER_IS_CONNECTED));
        return;
    };
    // **An abandon is a run of the ready action's own kind**: it is that run's
    // second way out (`Server Setup` S6) and it is what *Set up again* would
    // otherwise take over.
    if !crate::setting_up::a_run_begins(&server, SetupKind::TheReadyAction) {
        the_queue_is_told(queue, one_line_about(&server, words::SETUP_IS_ALREADY_RUNNING));
        return;
    }
    an_abandon_is_under_way_on(&server, true);
    the_queue_is_told(queue, the_run_moved_on(&server));
    crate::surface::servers::the_work_column_is_told_again(&app);

    let told = std::sync::Arc::clone(queue.inner());
    std::thread::spawn(move || {
        let came_to = crate::setting_up::the_setup_is_abandoned_on(&app, &point, &server);
        an_abandon_is_under_way_on(&server, false);
        // **`Server Setup` Y9a item 6: what was asked for meanwhile waited its
        // turn**, and begins where the abandon was performed (item 7).
        let restarted = crate::setting_up::the_run_restarted_the_server(&server);
        let performed = matches!(came_to, crate::setting_up::WhatTheAbandonCameTo::Abandoned { .. });
        let (stopped, next) = the_line_follows(
            &app,
            &server,
            crate::setting_up::a_run_in_the_line_ends(&server, performed && !restarted),
        );
        // **An abandon says what the check found** — `Design/Server Setup §
        // The two ways out` — so its line is always drawn.
        let answer = the_answer_about(
            &server,
            &the_acts_not_started_are_named(what_the_abandon_came_to(&came_to), &stopped, restarted),
        );
        the_queue_is_told(&told, a_runs_ending(answer, false));
        crate::surface::servers::the_work_column_is_told_again(&app);
        if let Some(next) = next {
            the_next_act_is_driven(&app, &server, next);
        }
    });
}

/// **What an abandon says** — `Design/Server Setup § The two ways out`: *"says
/// what that check found"*. Added 2026-09-22.
pub fn what_the_abandon_came_to(came_to: &crate::setting_up::WhatTheAbandonCameTo) -> WhatItCameTo {
    use crate::setting_up::{TheRunsEnding, WhatTheAbandonCameTo};
    match came_to {
        WhatTheAbandonCameTo::NothingStoodToAbandon => WhatItCameTo::only(words::NOTHING_TO_ABANDON),
        WhatTheAbandonCameTo::Abandoned {
            what_the_final_check_found: None,
        } => WhatItCameTo::only(words::ABANDONED_AND_THE_CHECK_PASSED),
        WhatTheAbandonCameTo::Abandoned {
            what_the_final_check_found: Some(found),
        } => WhatItCameTo::with(words::ABANDONED_AND_THE_CHECK_DID_NOT_PASS, found),
        WhatTheAbandonCameTo::ItDidNotFinish(ending) => match ending {
            TheRunsEnding::AStepFailed { said, .. }
            | TheRunsEnding::NoRunCouldBeOpened { said }
            | TheRunsEnding::ASetupThatStoppedPartWayStands { said, .. } => {
                WhatItCameTo::with(words::THE_ABANDON_DID_NOT_FINISH, said)
            }
            _ => WhatItCameTo::only(words::THE_ABANDON_DID_NOT_FINISH),
        },
    }
}

/// **This server's point, where a connection to it is live**, and nothing
/// where none is.
fn the_live_point<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
) -> Option<crate::servers::sessions::ThisServersSessions> {
    use tauri::Manager;
    app.try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>()
        .and_then(|periods| periods.the_sessions_of(server))
        .filter(crate::servers::sessions::ThisServersSessions::a_connection_is_live)
}

/// **What a setup came to, as the box carries it**: one sentence of Smoozy's,
/// and the machine's own words where it printed any, kept apart from it.
///
/// **The two are never joined** — the sentence says what Smoozy knows (Rule 8)
/// and the machine's words go behind *What the server said*, whole, the one
/// control the whole app uses for raw text (`Design/Server Setup § The step that
/// failed`). *(Added 2026-09-21. Until then the two were one string, and where
/// a lower layer had nothing better it was a debugging rendering: the first pass
/// on a real rental showed `TheMachineWouldNot("…")` with its escapes written
/// out.)*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatItCameTo {
    pub said: String,
    pub the_machines_own_words: Option<String>,
}

impl WhatItCameTo {
    fn only(said: impl Into<String>) -> Self {
        WhatItCameTo {
            said: said.into(),
            the_machines_own_words: None,
        }
    }

    fn with(said: impl Into<String>, words: &str) -> Self {
        WhatItCameTo {
            said: said.into(),
            the_machines_own_words: (!words.trim().is_empty()).then(|| words.to_string()),
        }
    }
}

/// **What a driven setup says, in standard software words** (Rule 9).
pub fn what_it_came_to(came_to: &crate::setting_up::WhereTheSetUpGot) -> WhatItCameTo {
    use crate::setting_up::{TheRunsEnding, WhereTheSetUpGot};
    match came_to {
        WhereTheSetUpGot::TheKindOrTheFamilyWouldNotRead { .. } => WhatItCameTo::only(format!(
            "{} {}",
            words::NOT_A_MACHINE_SMOOZY_CAN_SET_UP,
            words::NOTHING_WAS_CHANGED_ON_IT
        )),
        // **Ready to render only where this run published a Blender.** Added
        // 2026-09-22 — a server with no Blender cannot render, and a run that
        // installed none cannot say one is there.
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::ItReachedDone { a_blender_was_published, .. },
            ..
        } => WhatItCameTo::only(if *a_blender_was_published {
            words::THE_SERVER_IS_READY
        } else {
            words::THE_SERVER_IS_SET_UP
        }),
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::TheUserStoppedIt,
            ..
        } => WhatItCameTo::only(words::SETUP_STOPPED),
        other => {
            let reason = the_reason_in(other);
            WhatItCameTo {
                said: format!("{} {}", words::SETUP_STOPPED, reason.said),
                the_machines_own_words: reason.the_machines_own_words,
            }
        }
    }
}

/// The failed build send, including its own words. This same sentence is
/// recorded at the opening boundary so a later reading keeps its origin.
pub fn what_a_failed_build_send_says(said: &str) -> String {
    format!("{} {said}", words::BUILD_COULD_NOT_BE_SENT)
}

/// **What a Blender install says** — the same reasons, under its own first
/// words. Added 2026-09-21.
pub fn what_the_install_came_to(
    came_to: &crate::setting_up::WhereTheSetUpGot,
    version: &str,
) -> WhatItCameTo {
    use crate::setting_up::{TheRunsEnding, WhereTheSetUpGot};
    match came_to {
        // **A build the server could not fetch is named** — `Design/Server
        // Setup § Installing` and the spec's *"names it"*, the Creator's note 7
        // on 0.1.7: *the Blender build* named nothing.
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::AStepFailed { step: smoozy_server_setup::ready_action::Step::Fetch, said },
            ..
        } => WhatItCameTo::with(
            format!("{} {} {version}.", words::INSTALL_STOPPED, words::COULD_NOT_FETCH_BLENDER),
            said,
        ),
        // **Installed only where the run published a Blender.** A run that
        // reached *done* without one did not install it, whatever took it over.
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::ItReachedDone { a_blender_was_published: true, .. },
            ..
        } => WhatItCameTo::only(format!("Blender {version} {}", words::BLENDER_IS_INSTALLED)),
        WhereTheSetUpGot::TheRunEnded {
            ending: TheRunsEnding::TheUserStoppedIt,
            ..
        } => WhatItCameTo::only(words::INSTALL_STOPPED),
        WhereTheSetUpGot::TheServerIsNotSetUpYet
        | WhereTheSetUpGot::TheKindOrTheFamilyWouldNotRead { .. } => the_reason_in(came_to),
        other => {
            let reason = the_reason_in(other);
            WhatItCameTo {
                said: format!("{} {}", words::INSTALL_STOPPED, reason.said),
                the_machines_own_words: reason.the_machines_own_words,
            }
        }
    }
}

/// **The jobs a held-back setup names, called what Queue calls them.** Added
/// 2026-10-01, step 10c's Vast pass, fault 4.
///
/// **The case it was first written for, and the one it now also meets.**
/// Behind *What the server said* a setup held back by work shows the server
/// program's own list of what holds the commit (`Rented-server runtime` G21
/// and `Rented-server runtime` G22) — written for a reader who could match an
/// identity to a job. **The person reading it cannot**: the Vast pass was shown
/// `ed2edae4fdd86f822f35bf73a0bf25dd` alone, the job *pv busy*, rendering
/// under a server row removed with *Leave it running* while the same machine
/// was added again and set up. **This computer's record still held the job
/// under its name.** So each entry in the list that the record holds is
/// called what Queue calls it ([`crate::surface::the_queue::what_the_job_is_called`]);
/// an entry the record does not hold stays as the server gave it, which is
/// still the truth about what holds the setup. Only these two endings carry
/// that list, and nothing else in the box is touched.
pub fn the_jobs_are_named(
    came: WhatItCameTo,
    came_to: &crate::setting_up::WhereTheSetUpGot,
    what_a_job_is_called: &mut dyn FnMut(&smoozy_state::identities::Identity) -> Option<String>,
) -> WhatItCameTo {
    use crate::setting_up::WhereTheSetUpGot;
    use smoozy_server_protocol::placement::performing::WhatThePlacementReached as Reached;
    use smoozy_state::identities::{Identity, IdentityKind};
    let (WhereTheSetUpGot::ThePlacement {
        reached: Reached::Deferred { because },
    }
    | WhereTheSetUpGot::JobsOfAnEarlierSetupAreHeld { because, .. }) = came_to
    else {
        return came;
    };
    let named = because
        .split(", ")
        .map(|one| {
            Identity::read_back(IdentityKind::Job, one)
                .and_then(|job| what_a_job_is_called(&job))
                .filter(|name| !name.is_empty())
                .unwrap_or_else(|| one.to_string())
        })
        .collect::<Vec<_>>()
        .join(", ");
    WhatItCameTo {
        the_machines_own_words: Some(named),
        ..came
    }
}

/// **Each job called what Queue calls it, from this computer's record** —
/// [`the_jobs_are_named`]'s reader for the two runs that can end held back.
/// Nothing where the record cannot be lent.
fn the_jobs_in_the_record<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> impl FnMut(&smoozy_state::identities::Identity) -> Option<String> + '_ {
    move |job| {
        use tauri::Manager;
        let started = app.try_state::<std::sync::Mutex<crate::start::Started>>()?;
        let record = crate::keys::TheRunsRecord {
            started: started.inner(),
        };
        let mut called = None;
        crate::keys::LendsTheRecord::lend(&record, &mut |connection| {
            called = crate::record::job_records::JobRecords::over(connection)
                .read(job)
                .ok()
                .flatten()
                .map(|held| crate::surface::the_queue::what_the_job_is_called(&held));
        });
        called
    }
}

/// **The reason, in Smoozy's own words, with the machine's kept apart.**
fn the_reason_in(came_to: &crate::setting_up::WhereTheSetUpGot) -> WhatItCameTo {
    use crate::setting_up::{TheRunsEnding, WhereTheSetUpGot};
    use smoozy_server_protocol::placement::performing::WhatThePlacementReached as Reached;
    match came_to {
        WhereTheSetUpGot::TheKindOrTheFamilyWouldNotRead { .. } => WhatItCameTo::only(format!(
            "{} {}",
            words::NOT_A_MACHINE_SMOOZY_CAN_SET_UP,
            words::NOTHING_WAS_CHANGED_ON_IT
        )),
        WhereTheSetUpGot::TheRentalWouldNotNameItself => {
            WhatItCameTo::only("Could not detect the operating system.")
        }
        WhereTheSetUpGot::NoCandidateCouldBeOffered { .. }
        | WhereTheSetUpGot::NoRootCouldBeChosen { .. } => {
            WhatItCameTo::only("No writable folder found on the server.")
        }
        WhereTheSetUpGot::TheHandOverDidNotLand { said } => {
            WhatItCameTo::with(words::COULD_NOT_MAKE_ITS_FOLDER, said)
        }
        WhereTheSetUpGot::TheRootCouldNotBeRecorded { said } => {
            WhatItCameTo::with(words::COULD_NOT_RECORD_ITS_FOLDER, said)
        }
        WhereTheSetUpGot::TheReleaseWasNotAdmitted { said } => {
            WhatItCameTo::with(words::COULD_NOT_GET_ITS_PROGRAM, said)
        }
        WhereTheSetUpGot::ThePlacementCouldNotBeCarriedOut { said } => {
            WhatItCameTo::with(words::COULD_NOT_INSTALL_ITS_PROGRAM, said)
        }
        // **`Rented-server runtime` G3's phases, each said for what it is.**
        // The `because` beside two of them is Smoozy's own record of the
        // phase, not the machine's words, and it goes behind the same control
        // rather than into the sentence.
        WhereTheSetUpGot::ThePlacement { reached } => match reached {
            Reached::RolledBack => WhatItCameTo::only(words::ITS_PROGRAM_WAS_PUT_BACK),
            Reached::FailedBeforeCommit { because } => {
                WhatItCameTo::with(words::COULD_NOT_INSTALL_ITS_PROGRAM, because)
            }
            Reached::RollingBack { because } => {
                WhatItCameTo::with(words::PUTTING_IT_BACK_DID_NOT_FINISH, because)
            }
            Reached::Deferred { because } => {
                WhatItCameTo::with(words::WORK_STILL_NEEDS_ITS_PROGRAM, because)
            }
            Reached::APackageTransactionIsInProgress { .. }
            | Reached::AClaimStandsWhoseOwnerCouldNotBeRead => {
                WhatItCameTo::only(words::ITS_PROGRAM_IS_BEING_INSTALLED)
            }
            // Not reached: a verified placement goes on to the run.
            Reached::Verified => WhatItCameTo::only(""),
        },
        WhereTheSetUpGot::JobsOfAnEarlierSetupAreHeld { how_many, because } => {
            WhatItCameTo::with(words::held_jobs_of_an_earlier_setup(*how_many), because)
        }
        WhereTheSetUpGot::TheConnectionWent => WhatItCameTo::only(words::THE_CONNECTION_ENDED),
        WhereTheSetUpGot::TheServerIsNotSetUpYet => {
            WhatItCameTo::only(words::SET_UP_THIS_SERVER_FIRST)
        }
        WhereTheSetUpGot::TheBlenderListCouldNotBeRead { said } => WhatItCameTo::only(format!(
            "{} {}",
            said,
            words::NOTHING_WAS_INSTALLED_ON_IT
        )),
        WhereTheSetUpGot::TheRunEnded { ending, .. } => match ending {
            TheRunsEnding::AStepFailed { step, said } => {
                WhatItCameTo::with(the_steps_failure_sentence(*step), said)
            }
            // **The box names what actually failed** — step 10c's fourth pass
            // on Vast, fault 2. The run ended because Smoozy's program on the
            // server stopped answering, and the step's own failure sentence
            // would claim a check that never ran.
            TheRunsEnding::TheServerStoppedAnswering { step, said } => WhatItCameTo::with(
                format!(
                    "{} {}",
                    what_the_step_does(*step),
                    words::THE_SERVER_STOPPED_ANSWERING
                ),
                said,
            ),
            TheRunsEnding::TheUnpackFailed { said } => WhatItCameTo::with(words::FAILED_UNPACK, said),
            TheRunsEnding::TheBuildCouldNotBeSent { said } => {
                WhatItCameTo::only(what_a_failed_build_send_says(said))
            },
            // **Smoozy's own sentence, whole, and nothing behind *What the
            // server said*** — the material is composed on this computer.
            // The Creator's eleventh notes, note 2.
            TheRunsEnding::TheTelegramMaterialWasNotComposed { said } => {
                WhatItCameTo::only(said.as_str())
            }
            // **The run's own recorded ending, for the step that was under
            // way** — eighth-pass fix piece 1, 2026-09-27.
            TheRunsEnding::TheServerEndedTheRun { step, said } => WhatItCameTo::with(
                format!(
                    "{} {}",
                    what_the_step_does(*step),
                    words::THE_SERVER_ENDED_THE_SETUP
                ),
                said.as_deref().unwrap_or(""),
            ),
            TheRunsEnding::NoRunCouldBeOpened { said } => {
                WhatItCameTo::with(words::THE_RUN_WOULD_NOT_START, said)
            }
            TheRunsEnding::ASetupThatStoppedPartWayStands { said, .. } => {
                WhatItCameTo::with(words::A_SETUP_STOPPED_PART_WAY, said)
            }
            // **The step named is the one the record names** — step 10c's
            // seventh pass on Hyperstack was told the graphics driver's install
            // stopped part-way when that install had finished and the restart
            // was what stopped. Changed 2026-09-27, fix piece 6.
            TheRunsEnding::ItStoppedInsideAStep { step } => WhatItCameTo::only(format!(
                "{} {}",
                what_the_step_does(*step),
                words::STOPPED_INSIDE_THE_STEP
            )),
            // **The runner's ruling 1 of 2026-09-27: the user is told the
            // restart was refused, in the machine's words where it gave any.**
            TheRunsEnding::TheRestartWasRefused { said } => {
                WhatItCameTo::with(words::THE_RESTART_WAS_REFUSED, said)
            }
            TheRunsEnding::AStepNeedsAdminAccess { step } => WhatItCameTo::only(format!(
                "{} {}",
                what_the_step_does(*step),
                words::NEEDS_ADMIN_ACCESS_AND_NONE_WAS_GIVEN
            )),
            TheRunsEnding::TheMachineIsRestarting => {
                WhatItCameTo::only(words::THE_SERVER_IS_RESTARTING)
            }
            TheRunsEnding::TheMachineDidNotComeBack { .. } => {
                WhatItCameTo::only(words::THE_SERVER_DID_NOT_COME_BACK)
            }
            TheRunsEnding::ItReachedDone { a_blender_was_published: true, .. } => {
                WhatItCameTo::only(words::THE_SERVER_IS_READY)
            }
            TheRunsEnding::ItReachedDone { a_blender_was_published: false, .. } => {
                WhatItCameTo::only(words::THE_SERVER_IS_SET_UP)
            }
            TheRunsEnding::TheUserStoppedIt => WhatItCameTo::only(""),
        },
    }
}

/// **`Design/Server Setup § The step that failed`'s one plain sentence, for an
/// R1 step** — the sentence of the first boundary the step feeds, and the
/// final verification's own where the step is *done*.
fn the_steps_failure_sentence(step: smoozy_server_setup::ready_action::Step) -> &'static str {
    // **The tooling step fails here only at the tool's install**: its unpack
    // fails as `TheRunsEnding::TheUnpackFailed`, which names itself.
    if step == smoozy_server_setup::ready_action::Step::Tooling {
        return words::FAILED_TOOLING;
    }
    Boundary::IN_ORDER
        .into_iter()
        .find(|boundary| boundary.its_step() == step && *boundary != Boundary::Done)
        .and_then(the_boundarys_failure_sentence)
        .unwrap_or(words::FAILED_FINAL_CHECK)
}

/// **What the step an offer names does**, in the drawing's own shape —
/// *"Installing the GPU driver was refused: …"*
pub fn what_the_step_does(step: smoozy_server_setup::ready_action::Step) -> &'static str {
    use smoozy_server_setup::ready_action::Step;
    match step {
        Step::Tooling => words::DOING_THE_TOOLING,
        Step::Libraries => words::DOING_THE_LIBRARIES,
        Step::Driver => words::DOING_THE_DRIVER,
        Step::Restart => words::DOING_THE_RESTART,
        Step::Readings => words::DOING_THE_READINGS,
        Step::Fetch => words::DOING_THE_FETCH,
        Step::BlenderPublication => words::DOING_THE_PUBLICATION,
        Step::BlenderRemoval => words::DOING_THE_REMOVAL,
        Step::TelegramMaterial => words::DOING_THE_TELEGRAM,
        Step::Done => words::DOING_THE_FINAL_CHECK,
    }
}

/// **A listing that could not be read, in the list's own sentence** — and
/// never the service's own words, which is what the install list already
/// holds itself to.
pub fn why_the_list_could_not_be_read(_why: &WhyTheListingCouldNotBeRead) -> String {
    words::THE_LIST_COULD_NOT_BE_READ.to_string()
}

/// **What a run started from somewhere other than this tab's own control came
/// to**, told on this tab's one kind — `Servers` Q1 step 8's remembered setup
/// is the one such caller.
///
/// **From 2026-10-27 the run may have acts waiting behind it** (`Server Setup`
/// Y9a item 6): `follows` is what its ending left of the line. The acts it
/// stopped are named after its sentence, and the next one is begun once the
/// ending is told. Where no point stood there is no ending to tell, and the
/// card asks for the run again; a line it stopped is said on its own.
pub fn the_run_is_told_what_it_came_to<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    came_to: Option<&crate::setting_up::WhereTheSetUpGot>,
    follows: crate::setting_up::WhatFollowsARun,
    restarted: bool,
) {
    use tauri::Manager;
    let (stopped, next) = the_line_follows(app, server, follows);
    let Some(came_to) = came_to else {
        the_run_under_way_is_told(app, server);
        if let (false, Some(queue)) = (stopped.is_empty(), app.try_state::<std::sync::Arc<TheOneQueue>>()) {
            let came = the_acts_not_started_are_named(WhatItCameTo::only(""), &stopped, restarted);
            the_queue_is_told(queue.inner(), one_line_about(server, &came.said));
        }
        return;
    };
    // **`Server Setup` W1, on the Add dialog's route.** The remembered setup
    // belongs to a row the add has just written, which confirms no
    // installation before its first setup (`Servers` B1, and `State, identity
    // and recovery` Y9: another installation's program is never adopted), so
    // any installation the row confirms now was placed by this act.
    the_ready_action_ended_on(server, None, the_installation_the_row_confirms(app, server).as_deref());
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    the_queue_is_told(
        queue.inner(),
        a_runs_ending(
            the_answer_about(server, &the_acts_not_started_are_named(what_it_came_to(came_to), &stopped, restarted)),
            the_card_says_the_ready_actions_ending(came_to) && stopped.is_empty(),
        ),
    );
    crate::surface::servers::the_work_column_is_told_again(app);
    if let Some(next) = next {
        the_next_act_is_driven(app, server, next);
    }
}

/// **`Server Setup` R1b's swap run ended, and the tab is told as any setup
/// run's ending is** — the restart was told as every setup restart is, on the
/// card, while it ran. **It placed nothing** (`Server Setup` W1): the run
/// opens on the package area the row records and never on a placement, so
/// the installation before and after is the same one. Added 2026-10-12.
pub fn the_swap_runs_ending_is_told<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    came_to: &crate::setting_up::WhereTheSetUpGot,
) {
    use tauri::Manager;
    let confirmed = the_installation_the_row_confirms(app, server);
    the_ready_action_ended_on(server, confirmed.as_deref(), confirmed.as_deref());
    // **`Server Setup` Y9a item 8: what joined the line during the swap is not
    // begun**, and its ending names it — after the restart, as a restart
    // stops any line (item 7).
    let stopped = crate::setting_up::the_line_stopped_on(server);
    if !stopped.is_empty() {
        the_acts_waiting_are_kept(app, server, &[]);
    }
    let restarted = crate::setting_up::the_run_as_it_ended_on(server).is_some_and(|(_, read)| {
        read.record
            .the_boundaries_reached
            .iter()
            .any(|reached| reached.boundary == smoozy_server_setup::run::Boundary::RestartIssued)
    });
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    the_queue_is_told(
        queue.inner(),
        a_runs_ending(
            the_answer_about(server, &the_acts_not_started_are_named(what_it_came_to(came_to), &stopped, restarted)),
            the_card_says_the_ready_actions_ending(came_to) && stopped.is_empty(),
        ),
    );
    crate::surface::servers::the_work_column_is_told_again(app);
}

/// **`Server Setup` E2's offer, told to this server's tab**: the box is
/// `Design/Server Setup § Admin access`'s, and the step it names is the only
/// thing that changes between callers. Added 2026-09-21.
pub fn the_offer_is_made<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    step: smoozy_server_setup::ready_action::Step,
) {
    use tauri::Manager;
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        // **Nobody can be asked, so the offer is declined** rather than left
        // standing with no box to answer it.
        crate::setting_up::the_offer_is_answered(server, None);
        return;
    };
    the_queue_is_told(queue.inner(), the_offer_about(server, step));
}

/// The offer, as the tab is told it.
pub fn the_offer_about(server: &str, step: smoozy_server_setup::ready_action::Step) -> Event {
    Event::about(About::TheRun, THE_ANSWER)
        .saying(
            "the_step_that_was_refused",
            Said::Value(Verbatim::new(what_the_step_does(step))),
        )
        .saying("server", Said::Value(Verbatim::new(server)))
}

/// **The admin-access box, answered: *Try again* with a password, or *Cancel*
/// with none.** Added 2026-09-21; until then *Try again* started the whole
/// setup over and carried nothing.
///
/// **The password is handed to the run that asked and to nothing else** — it
/// crosses the page once, becomes `Security and licence`'s own origin at this
/// line, and is never written, logged or put on a screen (§13 rules 1, 2 and
/// 4). **Which lifetime the user chose is taken and is not yet kept past the
/// run**: the run holds it for its own length, and a later run asks again —
/// see [`crate::setting_up::TheRunBeingDriven::driven`], where that is said.
#[tauri::command]
pub fn the_admin_access_is_answered(
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    password: Option<String>,
) {
    let answer = password
        .filter(|typed| !typed.is_empty())
        .map(smoozy_security_and_licence::plaintext::Origin::what_the_user_typed);
    let given = answer.is_some();
    let taken = crate::setting_up::the_offer_is_answered(&server, answer);
    // **The run carries on, and the card is asked again** — the run is drawn
    // on the card since 2026-10-02 (the Creator's notes piece 2), not as a line
    // in a box.
    if taken && given {
        the_queue_is_told(&queue, the_run_moved_on(&server));
    }
}

/// One line about a server, on this tab's one kind.
fn one_line_about(server: &str, said: &str) -> Event {
    the_answer_about(server, &WhatItCameTo::only(said))
}

/// **The page is told the press is being answered, and then the reading the
/// answer waits on is taken** — `Design/Whole app § Anything that isn't
/// instant shows that it's underway`: *"on the control"*, from the press. Added
/// 2026-09-28, step 10c's ninth-pass fix piece 4.
///
/// **The event carries no words**: the control keeps its own name under the
/// mark. **The mark comes off on the next answer about that server**, which
/// every way the reading can end posts — the warning owed, a run begun, or the
/// one line saying why nothing began — so no clock is needed to end it (P9,
/// P10).
fn the_press_is_marked_while<T>(
    told: &std::sync::Arc<TheOneQueue>,
    server: &str,
    the_reading: impl FnOnce() -> T,
) -> T {
    the_queue_is_told(told, the_action_is_underway_on(server));
    the_reading()
}

/// See [`the_press_is_marked_while`].
fn the_action_is_underway_on(server: &str) -> Event {
    Event::about(About::TheRun, THE_ANSWER)
        .saying("the_action_is_underway", Said::Value(Verbatim::new("yes")))
        .saying("server", Said::Value(Verbatim::new(server)))
}

/// **A run on this server began or moved on, and the tab is told to ask for
/// it** — the run is drawn on the card (`Design/Server Setup § A long step`),
/// and [`the_setup_under_way`] answers it. **The event carries no reading and
/// no words**: P6's one route carries prose and values and nothing composite.
///
/// *(Until 2026-10-02 the start of an act was a line in a box — *Setting up
/// this server.* with *Stop* — over the server's whole screen, tabs included.
/// The Creator's note 4 on 0.1.7: "setting up the server should not stop the
/// user from … browsing files on the one being set up".)*
fn the_run_moved_on(server: &str) -> Event {
    Event::about(About::TheRun, THE_ANSWER)
        .saying("the_run_is_under_way", Said::Value(Verbatim::new("yes")))
        .saying("server", Said::Value(Verbatim::new(server)))
}

/// **A run's own ending, as the tab is told it.** Two values ride on the
/// answer, and the page decides neither:
///
/// - **the Blender table is read again** — `Design/Whole app § Anything that
///   isn't instant`, *over the pane*: an act of Smoozy's own ending on the
///   machine is what lets `Server Setup` B12's held search go
///   ([`crate::setting_up::a_run_ends`]), and while it runs again the table
///   carries the mark and the card does not (the Creator's note 8 on 0.1.7);
/// - **whether the card already says it** — a setup that reached its end is
///   said by the card's *Ready to render* and its steps, and a Blender
///   installed by the table that lists it, so the card carries no second line
///   for those. Every other ending is said on the card as one line, above its
///   steps, until the user acts again or the tab is shown again.
fn a_runs_ending(answer: Event, the_card_says_it: bool) -> Event {
    let answer = answer.saying("the_blenders_are_read_again", Said::Value(Verbatim::new("yes")));
    if the_card_says_it {
        answer.saying("the_card_says_it", Said::Value(Verbatim::new("yes")))
    } else {
        answer
    }
}

/// `Server Setup` B12: both install routes and removal drop this server's
/// Render Settings list before the existing ending tells an open page to ask
/// again. Other setup runs leave that list alone.
fn a_blender_runs_ending(server: &str, answer: Event, the_card_says_it: bool) -> Event {
    if let Ok(mut held) = crate::surface::render_settings::the_run_holds().lock() {
        crate::surface::render_settings::the_blender_run_ended_for(&mut held, server);
    }
    a_runs_ending(answer, the_card_says_it)
        .saying("the_render_versions_are_read_again", Said::Value(Verbatim::new("yes")))
}

/// **A ready action that reached *done* is said by the card itself — and so is
/// one that ended on a failed step**, which the card draws on that step, with
/// the same sentence and the same *What the server said*.
///
/// `Design/Server Setup § The step that failed`: *"It shows once, on the step,
/// and nowhere else on the card … nothing above the steps repeats the sentence
/// or the control."* The three endings counted here are the ones recorded on
/// the machine as a failed step (`Server Setup` S2a) whose sentence on the step
/// is the line's own: a step that failed, the unpack, and Telegram's material
/// not composed. *(Added 2026-10-25, the eighteenth notes, notes 3 and 4: the
/// Creator's stopped setup drew *Setup stopped. The server could not fetch the
/// Blender build.* with its own *What the server said* above the steps, and the
/// same pair on the step.)*
///
/// **A refused restart keeps its line**, because the step does not say what the
/// line says: the line says it was refused (the runner's ruling 1 of
/// 2026-09-27). Every ending that is not a failed step — stopped, the server
/// not answering, the run ended by the machine — keeps its line as before.
/// *(Until 2026-10-25 the unpack tool's own install kept its line too, because
/// its step read as the unpack's; the step now names the install
/// ([`the_tools_own_install_is_named`]), and the line went.)*
fn the_card_says_the_ready_actions_ending(came_to: &crate::setting_up::WhereTheSetUpGot) -> bool {
    use crate::setting_up::{TheRunsEnding, WhereTheSetUpGot};
    matches!(came_to, WhereTheSetUpGot::TheRunEnded { ending: TheRunsEnding::ItReachedDone { .. }, .. })
        || the_step_its_failure_is_recorded_on(came_to).is_some()
}

/// **The step an ending is recorded on the machine as having failed**
/// (`Server Setup` S2a) — a step that failed, the unpack (the tooling step's),
/// and Telegram's material not composed — and nothing for every other ending.
fn the_step_its_failure_is_recorded_on(
    came_to: &crate::setting_up::WhereTheSetUpGot,
) -> Option<smoozy_server_setup::ready_action::Step> {
    use crate::setting_up::{TheRunsEnding, WhereTheSetUpGot};
    use smoozy_server_setup::ready_action::Step;
    match came_to {
        WhereTheSetUpGot::TheRunEnded { ending, .. } => match ending {
            TheRunsEnding::AStepFailed { step, .. } => Some(*step),
            TheRunsEnding::TheUnpackFailed { .. } => Some(Step::Tooling),
            TheRunsEnding::TheTelegramMaterialWasNotComposed { .. } => Some(Step::TelegramMaterial),
            _ => None,
        },
        _ => None,
    }
}

/// **The servers an abandon is being driven on**, so the card names the act it
/// is drawing — an abandon is a run of the ready action's own kind (`Server
/// Setup` S6), and *Setting up* would name the wrong act. Period-held and
/// nothing durable.
fn the_abandons_under_way() -> &'static Mutex<Vec<String>> {
    static HELD: std::sync::OnceLock<Mutex<Vec<String>>> = std::sync::OnceLock::new();
    HELD.get_or_init(|| Mutex::new(Vec::new()))
}

fn an_abandon_is_under_way_on(server: &str, it_is: bool) {
    if let Ok(mut held) = the_abandons_under_way().lock() {
        held.retain(|one| one != server);
        if it_is {
            held.push(server.to_string());
        }
    }
}

/// **What the card calls the run this copy is driving on that server**, or
/// nothing where it drives none.
fn what_the_run_is_called_on(server: &str) -> Option<&'static str> {
    let kind = crate::setting_up::the_kind_being_driven_on(server)?;
    let abandoning = the_abandons_under_way()
        .lock()
        .map(|held| held.iter().any(|one| one == server))
        .unwrap_or(false);
    Some(if abandoning {
        words::ABANDONING_SETUP
    } else if kind == SetupKind::ABlenderInstall {
        words::INSTALLING_BLENDER
    } else if kind == SetupKind::TheTelegramPlacement {
        // **The act's own name** — *Setting up* would name the ready action,
        // which this run is given a route of its own so as not to be.
        words::DOING_THE_TELEGRAM
    } else {
        words::SETTING_UP
    })
}

/// **The answer, with the machine's own words beside it where it printed any**
/// — behind *What the server said*, whole, never inside the sentence.
fn the_answer_about(server: &str, came_to: &WhatItCameTo) -> Event {
    let mut event = Event::about(About::TheRun, THE_ANSWER)
        .saying("the_act_is_not_built_yet", Said::Prose(Prose::new(&came_to.said)))
        .saying("close", Said::Prose(Prose::new(words::CLOSE)))
        .saying("server", Said::Value(Verbatim::new(server)));
    if let Some(said) = &came_to.the_machines_own_words {
        event = event
            .saying("the_machines_own_words", Said::Value(Verbatim::new(said)))
            .saying(
                "what_the_server_said",
                Said::Prose(Prose::new(words::WHAT_THE_SERVER_SAID)),
            );
    }
    event
}

/// **A run on this server moved on, and the tab is told to ask for its panel**
/// — `Design/Server Setup § A long step`. Added 2026-09-27, fix piece 9.
///
/// **The event carries no reading.** P6's one route carries prose and values
/// and nothing composite, so the tab is told and asks
/// [`the_setup_under_way`], which answers from what the driving last read off
/// the machine (`Server Setup` S1) and nothing remembered past the run.
pub fn the_run_under_way_is_told<R: tauri::Runtime>(app: &tauri::AppHandle<R>, server: &str) {
    use tauri::Manager;
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    the_queue_is_told(queue.inner(), the_run_moved_on(server));
    // **The server row reads *Setting up* while a run is driven there** —
    // `Design/Servers § A row`, the Creator's note 3 on 0.1.7 — so the list
    // is told where that moved.
    crate::surface::servers::the_work_column_is_told_again(app);
}

/// **The panel of a run still going on this server** — `Design/Server Setup §
/// A long step`: *"Shown as its steps, with the one under way marked"*, and
/// `§ A second setup while one is running`: *"the panel shows the run that is
/// already going with what it has reached"*. **Nothing where this copy drives
/// no run there.** Added 2026-09-27, fix piece 9.
///
/// **A command, for the reason [`the_set_up_tab`] is one**: the page asking
/// for what it needs to draw. **It takes no reading of its own** — the
/// driving took it, before the step it marks (`Server Setup` S1) — so it
/// answers at once, which is the point: the tab's own reading waits on
/// `Server Setup` B10's whole-disk search.
///
/// **From the moment the run is taken until its first reading**, which on a
/// machine never prepared is `Server Setup` R2's whole placement, there are no
/// steps to show, so the card says the run is under way on its state line and
/// offers *Cancel setup* (the Creator's notes piece 2, 2026-10-02).
#[tauri::command]
pub fn the_setup_under_way<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    server: String,
) -> Option<TheRunPanel> {
    the_setup_under_way_on(&server, || the_servers_name(&app, &server).unwrap_or_default())
}

/// **[`the_setup_under_way`]'s answer, composed** — with the server's name
/// asked for only where a reading is drawn, so the desk composes the same
/// panel without a window. Written apart 2026-10-18, the Creator's
/// fourteenth notes, piece S.
pub fn the_setup_under_way_on(server: &str, the_name: impl FnOnce() -> String) -> Option<TheRunPanel> {
    let server = server.to_string();
    // **A run that has ended answers its steps as its last reading left them,
    // marked as ended** — until the next run begins there (2026-10-26, the
    // worldwide pass's setup faults, fault 4). The card keeps them while its
    // own reading is taken, and never draws them as a run still going.
    if crate::setting_up::the_kind_being_driven_on(&server).is_none() {
        let (kind, mut read) = crate::setting_up::the_run_as_it_ended_on(&server)?;
        read.the_step_in_flight = None;
        let mut panel = the_panel_of_a_run_under_way(&read, &the_name(), false);
        panel.title = Prose::new(match kind {
            SetupKind::ABlenderInstall => words::INSTALLING_BLENDER,
            SetupKind::TheTelegramPlacement => words::DOING_THE_TELEGRAM,
            _ => words::SETTING_UP,
        });
        panel.cancel = None;
        panel.the_run_has_ended = true;
        the_servers_window_beside_the_run(&mut panel, crate::setting_up::the_words_are_kept_for(&server), Vec::new());
        return Some(panel);
    }
    let called = what_the_run_is_called_on(&server)?;
    let cancelled = crate::setting_up::AsksToStop::the_user_asked_to_stop(
        &crate::setting_up::TheStopThisServerWasAsked { server: server.clone() },
    );
    if the_waits_for_cut_runs().lock().ok()?.iter().any(|one| one == &server) {
        let mut panel = the_panel_while_the_cut_run_finishes(cancelled);
        the_line_is_drawn_on(&mut panel, &server);
        return Some(panel);
    }
    let kept = crate::setting_up::the_words_are_kept_for(&server);
    let Some(read) = crate::setting_up::the_run_under_way_on(&server) else {
        let mut panel = the_run_before_its_first_reading(called, cancelled);
        the_servers_window_beside_the_run(&mut panel, kept, Vec::new());
        the_line_is_drawn_on(&mut panel, &server);
        return Some(panel);
    };
    let clock = the_name();
    // **`Server Setup` W3's reading is carried from the warning shown before
    // the run** — true only where that warning named a render running and the
    // user went ahead. **Where no warning was shown, nothing is carried and the
    // render sentence is absent**, because an absent sentence claims nothing.
    // Eighth-pass fix piece 2, and the caller W3's warning was owed, both
    // 2026-09-27.
    let mut panel = the_panel_of_a_run_under_way(
        &read,
        &clock,
        crate::setting_up::a_render_was_running_when_the_run_began(&server),
    );
    panel.title = Prose::new(called);
    panel.the_cancel_was_asked = cancelled;
    panel.the_question = the_question_on(&server);
    // **`Server Setup` Y7a: the newest lines the run has printed**, kept
    // while the steps after them print nothing (`Design/Server Setup § A long
    // step`). Changed 2026-10-18, the Creator's fourteenth notes, piece S:
    // until then only the step under way's were asked for, so they went the
    // moment the next step was marked — on a container, a few seconds of an
    // install at most.
    // **From 2026-10-25, either stream's**, each line with its stream (M13b,
    // the eighteenth notes, piece 2c).
    let window = crate::setting_up::the_window_on(&server);
    the_servers_window_beside_the_run(&mut panel, kept, window);
    // **`Server Setup` Y7b and `Server Setup` Y7c: the download's step named for the build,
    // and how far the download or the unpack has got.** Added 2026-10-18, the
    // Creator's fourteenth notes, piece U.
    the_builds_steps_are_named_and_counted(
        &mut panel,
        crate::setting_up::the_builds_figures_on(&server).as_ref(),
        the_build_asked_for_is_named,
    );
    the_line_is_drawn_on(&mut panel, &server);
    Some(panel)
}

/// **`Server Setup` Y9a's line, on the card of a run still going** — the acts
/// waiting, named as the acts they are, and *Install Telegram reporting*
/// offered quietly where owed so it can join it (`Design/Server Setup §
/// Asking for more while a setup runs`). Added 2026-10-27, the nineteenth
/// notes, note 2. **No *Set up again* while it runs** — gone, not greyed, and
/// back when the run ends (`Server Setup` Y9a item 6; the Creator's
/// twenty-fourth notes, note 1, 2026-10-30).
pub fn the_line_is_drawn_on(panel: &mut TheRunPanel, server: &str) {
    panel.the_waiting = crate::setting_up::the_acts_still_waiting_on(server)
        .iter()
        .map(|act| Prose::new(&the_act_is_named(&act.as_kept())))
        .collect();
    panel.the_waiting_heading = (!panel.the_waiting.is_empty()).then(|| Prose::new(words::THE_WAITING_HEADING));
    panel.install_telegram_while_it_runs =
        the_telegram_offer_stands_on(server).then(|| Prose::new(words::INSTALL_TELEGRAM_REPORTING));
}

/// **Which servers' cards last offered *Install it* on Telegram reporting** —
/// so the running card can offer it too (`Design/Server Setup § Asking for
/// more while a setup runs`). Written each time the tab is composed;
/// run-held. Added 2026-10-27.
fn the_telegram_offers() -> &'static Mutex<std::collections::BTreeSet<String>> {
    static HELD: std::sync::OnceLock<Mutex<std::collections::BTreeSet<String>>> = std::sync::OnceLock::new();
    HELD.get_or_init(|| Mutex::new(std::collections::BTreeSet::new()))
}

/// **Whether the card on that server last offered the Telegram placement.**
pub fn the_telegram_offer_stands_on(server: &str) -> bool {
    the_telegram_offers().lock().is_ok_and(|held| held.contains(server))
}

/// **The card's offer of the Telegram placement, held as the tab drew it.**
pub fn the_telegram_offer_is_held(server: &str, card: &TheCard) {
    let offered = card
        .what_was_done
        .iter()
        .any(|line| line.offer.as_ref().is_some_and(|offer| offer.act == INSTALL_TELEGRAM));
    if let Ok(mut held) = the_telegram_offers().lock() {
        if offered {
            held.insert(server.to_string());
        } else {
            held.remove(server);
        }
    }
}

/// **`Server Setup` S5 and `Server Setup` Y9: one press waits while the previous run ends.**
fn the_waits_for_cut_runs() -> &'static Mutex<Vec<String>> {
    static WAITING: std::sync::OnceLock<Mutex<Vec<String>>> = std::sync::OnceLock::new();
    WAITING.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn the_wait_for_the_cut_run_is_kept(server: &str, waiting: bool) {
    if let Ok(mut held) = the_waits_for_cut_runs().lock() {
        held.retain(|one| one != server);
        if waiting {
            held.push(server.to_string());
        }
    }
}

/// **`Server Setup` S5's wait, in the running card's existing shape.**
pub fn the_panel_while_the_cut_run_finishes(cancelled: bool) -> TheRunPanel {
    the_run_before_its_first_reading(words::THE_SERVER_IS_FINISHING_THE_CUT_RUN, cancelled)
}

/// **The server's words beside a run's steps** — `Design/Server Setup § A long
/// step`, `Desktop app and delivery` M13b's second narrowing, `Server Setup`
/// Y7a. Added 2026-10-18, the Creator's thirteenth notes, piece H.
///
/// - ***Server output* wherever this copy keeps the run's words**, which is
///   from the run being taken: before anything is printed it sits alone.
/// - **The newest complete lines of the ordinary stream the run has printed,
///   kept until another step prints (from 2026-10-18, piece S; they were the
///   step under way's), only while the run goes, and never beside a sentence
///   about a failure** —
///   a card with a failed step on it gets none. The lines arrive already taken
///   from the ordinary stream alone ([`crate::setting_up::TheRunsWords`]);
///   nothing of the complaints stream is ever handed here.
pub fn the_servers_words_beside_the_run(panel: &mut TheRunPanel, the_words_are_kept: bool, lines: Vec<String>) {
    let window = lines
        .into_iter()
        .map(|line| (smoozy_server_setup::elevated::WhichStream::TheOrdinaryOutput, line))
        .collect();
    the_servers_window_beside_the_run(panel, the_words_are_kept, window);
}

/// **[`the_servers_words_beside_the_run`], each line with the stream it came
/// from** — the window as `Desktop app and delivery` M13b's second narrowing
/// has it from 2026-10-24: either stream, in arrival order, marked by stream.
/// Added 2026-10-25, the eighteenth notes, piece 2c. **Still never beside a
/// sentence about a failure, and only while the run goes.**
pub fn the_servers_window_beside_the_run(
    panel: &mut TheRunPanel,
    the_words_are_kept: bool,
    window: Vec<(smoozy_server_setup::elevated::WhichStream, String)>,
) {
    panel.server_output = the_words_are_kept.then(|| Prose::new(words::SERVER_OUTPUT));
    let still_going = panel.cancel.is_some();
    let beside_a_failure = panel.steps.iter().any(|step| step.the_failure.is_some());
    let shown = if the_words_are_kept && still_going && !beside_a_failure { window } else { Vec::new() };
    panel.which_lines_are_from_the_error_stream = shown
        .iter()
        .map(|(stream, _)| *stream == smoozy_server_setup::elevated::WhichStream::TheComplaints)
        .collect();
    panel.the_servers_newest_lines = shown.into_iter().map(|(_, line)| Verbatim::new(line)).collect();
}

/// **What the server printed arrived, and the tab is told to ask for the run's
/// panel again** — the window's lines ride it. **The event carries no words**:
/// P6's one route carries prose and values and nothing composite, and the lines
/// are the panel's. Added 2026-10-18, the Creator's thirteenth notes, piece H.
pub fn the_servers_words_are_told<R: tauri::Runtime>(app: &tauri::AppHandle<R>, server: &str) {
    use tauri::Manager;
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    the_queue_is_told(queue.inner(), the_servers_words_moved_on(server));
}

/// See [`the_servers_words_are_told`]. **It answers no press**, so the page
/// takes no mark off for it and counts it as no answer.
pub fn the_servers_words_moved_on(server: &str) -> Event {
    Event::about(About::TheRun, THE_ANSWER)
        .saying("the_servers_words_moved", Said::Value(Verbatim::new("yes")))
        .saying("server", Said::Value(Verbatim::new(server)))
}

/// **The whole of what the last run on this server printed, the two streams
/// apart** — what ***Server output*** opens: `Design/Server Setup § A long
/// step`, *"the whole of what this run printed, its two streams apart, with the
/// copy icon in the box's bottom-right corner"*. Every string is on one of the
/// two routes (A6, X9). Added 2026-10-18, the Creator's thirteenth notes,
/// piece H.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheServersOutput {
    pub title: Prose,
    /// The server's name, as the list shows it. A value.
    pub subtitle: Verbatim,
    /// **One tab per stream, under the stream's own name, never merged**
    /// (`Desktop app and delivery` M13, M13a).
    pub tabs: Vec<AStreamsTab>,
    pub note: Prose,
    pub close: Prose,
    pub copy: Prose,
}

/// One stream's tab.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AStreamsTab {
    pub name: Prose,
    /// **What the tab is**: `the-record`, `not-recorded-yet` while the run
    /// goes and the stream has printed nothing, or `empty` once it ended.
    pub says: Verbatim,
    /// **The stream, whole and unedited** (M9), where it printed anything.
    pub record: Option<Verbatim>,
    /// **The sentences where it printed nothing** — never an empty box (M10).
    pub said: Option<Prose>,
    pub said_next: Option<Prose>,
    /// **`Design/Server Setup § A long step`: *Some of what the server printed
    /// could not be shown.***, under the tab's words, where a step's words
    /// could not be brought back (`Server Setup` Y7a item 5). **Outside the
    /// record**, so *Copy*, which takes the record alone, never takes it.
    /// Added 2026-10-24.
    pub not_shown: Option<Prose>,
    /// **The stream as printed, every byte, where it differs from
    /// [`Self::record`]** — which shows each line a program rewrote in place
    /// in its newest form, as a terminal shows it (`Desktop app and delivery`
    /// M13b's second narrowing as changed on 2026-10-24). **This is what
    /// *Copy* takes**: *"Copy takes the words as they were printed"*
    /// (`Design/Server Setup § Nothing is silent while a setup runs`).
    /// Nothing where the two are the same. Added 2026-10-25, the eighteenth
    /// notes, piece 2c.
    pub as_printed: Option<Verbatim>,
}

/// ***Server output*, opened**: the last run's words on that server, or
/// nothing where this copy keeps none.
#[tauri::command]
pub fn the_servers_output<R: tauri::Runtime>(app: tauri::AppHandle<R>, server: String) -> Option<TheServersOutput> {
    the_servers_output_on(&server, &the_servers_name(&app, &server).unwrap_or_default())
}

/// **[`the_servers_output`]'s answer for a server named `name`** — written
/// apart 2026-10-18 (piece S) so the desk opens the same panel without a
/// window.
pub fn the_servers_output_on(server: &str, name: &str) -> Option<TheServersOutput> {
    let (output, complaints) = crate::setting_up::the_whole_of_what_was_printed_on(server)?;
    let still_going = crate::setting_up::the_kind_being_driven_on(server).is_some();
    let some_were_not_kept = crate::setting_up::some_of_what_was_printed_was_not_kept_on(server);
    Some(the_servers_output_kept(name, &output, &complaints, still_going, some_were_not_kept))
}

/// **The panel's words, composed** — so it can be tested without a window.
pub fn the_servers_output_from(name: &str, output: &str, complaints: &str, still_going: bool) -> TheServersOutput {
    the_servers_output_kept(name, output, complaints, still_going, false)
}

/// **[`the_servers_output_from`], saying where some of what the server printed
/// could not be brought back** (`Server Setup` Y7a item 5) — each tab then says
/// so under its words. Added 2026-10-24, the Creator's ruling on the sixteenth
/// notes, note 1.
pub fn the_servers_output_kept(
    name: &str,
    output: &str,
    complaints: &str,
    still_going: bool,
    some_were_not_kept: bool,
) -> TheServersOutput {
    use crate::presentation::the_machines_own_words::{SMOOZY_DOES_NOT_KNOW_YET, THE_OUTCOME_HAS_NOT_BEEN_RECORDED_YET};
    let not_shown = some_were_not_kept.then(|| Prose::new(words::SOME_OF_WHAT_THE_SERVER_PRINTED_COULD_NOT_BE_SHOWN));
    let tab = |stream: Stream, record: &str, empty: &str| {
        let name = Prose::new(stream.the_name_of_the_stream());
        if !record.is_empty() {
            // **The record shown as a terminal shows it, and the words as
            // printed kept for *Copy* where the two differ** (M13b, M9; the
            // eighteenth notes, piece 2c).
            let shown = smoozy_server_setup::elevated::as_a_terminal_shows_it(record);
            let as_printed = (shown != record).then(|| Verbatim::new(record));
            return AStreamsTab {
                name,
                says: Verbatim::new("the-record"),
                record: Some(Verbatim::new(shown)),
                said: None,
                said_next: None,
                not_shown: not_shown.clone(),
                as_printed,
            };
        }
        if still_going {
            return AStreamsTab {
                name,
                says: Verbatim::new("not-recorded-yet"),
                record: None,
                said: Some(Prose::new(THE_OUTCOME_HAS_NOT_BEEN_RECORDED_YET)),
                said_next: Some(Prose::new(SMOOZY_DOES_NOT_KNOW_YET)),
                not_shown: not_shown.clone(),
                as_printed: None,
            };
        }
        AStreamsTab {
            name,
            says: Verbatim::new("empty"),
            record: None,
            said: Some(Prose::new(empty)),
            said_next: None,
            not_shown: not_shown.clone(),
            as_printed: None,
        }
    };
    TheServersOutput {
        title: Prose::new(words::SERVER_OUTPUT),
        subtitle: Verbatim::new(name),
        tabs: vec![
            tab(Stream::TheOrdinaryOutput, output, words::THE_SERVER_PRINTED_NO_OUTPUT),
            tab(Stream::TheComplaints, complaints, words::THE_SERVER_WROTE_NOTHING_HERE),
        ],
        note: Prose::new(words::THE_PANELS_NOTE),
        close: Prose::new(words::CLOSE),
        copy: Prose::new(words::COPY),
    }
}

/// **A run taken and not yet read**: its name on the state line, under way on
/// itself, and *Cancel setup* — `Design/Whole app § Anything that isn't
/// instant`: *"What the rule forbids is showing nothing at all while work is
/// happening."*
pub fn the_run_before_its_first_reading(called: &str, the_cancel_was_asked: bool) -> TheRunPanel {
    TheRunPanel {
        this_server: Prose::new(words::THIS_SERVER),
        dot: Verbatim::new("busy"),
        title: Prose::new(called),
        the_state_is_underway: true,
        when_it_started: None,
        how_it_ended: None,
        steps: Vec::new(),
        about_the_render: None,
        what_cancel_leaves: Some(Prose::new(words::CANCEL_SETUP_LEAVES)),
        the_closing_paragraph: None,
        cancel: Some(Prose::new(words::CANCEL_SETUP)),
        the_cancel_was_asked,
        abandon: None,
        set_up_again: None,
        the_question: None,
        the_servers_newest_lines: Vec::new(),
        which_lines_are_from_the_error_stream: Vec::new(),
        server_output: None,
        the_last_act_is_said_on_its_steps: false,
        the_run_has_ended: false,
        the_waiting_heading: None,
        the_waiting: Vec::new(),
        install_telegram_while_it_runs: None,
    }
}

/// **The running panel, composed from one reading** — so it can be tested
/// without a window.
///
/// **`a_render_was_running` is `Server Setup` W3's reading, carried from the
/// warning the user was shown before the setup began** — true only where that
/// warning named a job rendering. Added 2026-09-27, step 10c's eighth-pass fix
/// piece 2.
pub fn the_panel_of_a_run_under_way(
    read: &crate::setting_up::TheSetupAsRead,
    clock: &str,
    a_render_was_running: bool,
) -> TheRunPanel {
    let mut panel = the_run_panel(
        &read.record,
        read.the_step_in_flight,
        clock,
        true,
        read.record.the_composed_list.contains(&Boundary::Libraries),
    );
    panel.about_the_render = the_render_sentence(&read.record, a_render_was_running);
    panel
}

/// **`Server Setup` T6a and T7's sentence about the render, only where it is
/// true.** Added 2026-09-27, step 10c's eighth-pass fix piece 2.
///
/// *"The render that was running stopped, and starts again when the setup
/// finishes."* **Both halves assume a render was running**: T7's *"a running
/// render dies with the machine"* is the stopping half, and T6a's re-run is
/// owed only to a job that was interrupted. `Design/Server Setup § A long
/// step` has the panel say *"plainly that the render which was running went
/// down with it"* — with *it* the restart. **So the sentence is drawn only
/// where a render was running when the setup began, and only once *restart
/// issued* stands in the run's record** — before the restart nothing has gone
/// down. Step 10c's eighth pass on Hyperstack drew it from the driver step on,
/// before any restart, on a machine that had never rendered (fault 5).
///
/// **T6a's own bound is kept as it was**: the sentence stands for as long as
/// the run is being driven, which is the running panel's, and names the
/// finish rather than promising the re-run flatly.
///
/// **Where no warning was shown because nothing was running, nothing is
/// carried and the sentence is absent** — the brief's thing to settle.
/// **Nothing is re-read from the machine for it.**
fn the_render_sentence(record: &TheSetupRecord, a_render_was_running: bool) -> Option<Prose> {
    let the_restart_was_issued = record
        .the_boundaries_reached
        .iter()
        .any(|reached| reached.boundary == Boundary::RestartIssued);
    (a_render_was_running && the_restart_was_issued).then(|| Prose::new(words::THE_RENDER_STOPPED))
}

/// The queue, told one thing.
fn the_queue_is_told(queue: &std::sync::Arc<TheOneQueue>, event: Event) {
    let _ = queue.read_then_post::<(), std::convert::Infallible>(|| Ok(()), |_| event.clone());
}

/// **The answer, and it names its server** — so only that server's Set up tab
/// draws it (must-do 30) — **and it carries the box's one control, *Close***,
/// which is what the control does (must-do 29; until the step 10a round 1 audit
/// of these pieces the page labelled it with the card's own action).
pub fn the_answer_for(server: &str) -> Event {
    Event::about(About::TheRun, THE_ANSWER)
        .saying(
            "the_act_is_not_built_yet",
            Said::Prose(Prose::new(words::THE_ACT_IS_NOT_BUILT_YET)),
        )
        .saying("close", Said::Prose(Prose::new(words::CLOSE)))
        .saying("server", Said::Value(Verbatim::new(server)))
}

/// **`Security and licence` S32's one row, read.**
///
/// **Its one writer is [`crate::the_store_standing`]**, which takes the probe
/// at the first moment in a run when a secret is actually needed
/// (`Security and licence` S26) and writes back what `Security and licence`
/// S25's reading rule hands it. *(Until step 10b piece 3a
/// nothing wrote it at all, so this read answered *never probed* for ever and
/// `Servers` N8b and `Keys` S10 could never fire.)*
///
/// **Three answers and never two** — L8a: no row is *never probed*, which is a
/// fresh installation and correct; **a read that failed is `None`**, and is
/// never turned into a fresh installation (must-do 32). A row holding a value
/// this reader does not know is a read it cannot vouch for, and is `None` too.
///
/// **It takes the record's own connection rather than the spine, and there is
/// one reader of this value in the crate.** `Servers` N8b's permanent refusal
/// reads the same standing from `crate::keys::for_a_connection`, over a lent
/// record and not over the spine's lock — and a second copy of this three-answer
/// rule is exactly the fault this build stage keeps paying for, a fact kept in
/// two places. Step 10b piece 2.
pub(crate) fn the_store_standing(
    connection: &rusqlite::Connection,
) -> Option<StoreStanding> {
    let read: Result<String, _> = connection.query_row(
        "SELECT standing FROM store_standing WHERE only_row = 1",
        [],
        |row| row.get(0),
    );
    the_standing_from(read)
}

/// The reading of [`the_store_standing`], with the row's answer handed in so a
/// test can stand where it stands.
pub(crate) fn the_standing_from(read: Result<String, rusqlite::Error>) -> Option<StoreStanding> {
    match read.as_deref() {
        Ok("has answered") => Some(StoreStanding::HasAnswered),
        Ok("never answered") => Some(StoreStanding::NeverAnswered),
        Ok(_) => None,
        Err(rusqlite::Error::QueryReturnedNoRows) => Some(StoreStanding::default()),
        Err(_) => None,
    }
}

/// **No machine has been read, so no version has been answered on one.**
fn the_versions_answered_on(_server: &str) -> Vec<Version> {
    Vec::new()
}

/// **The version that build answered, as the held search read it** — `None`
/// where no search is held or it did not find that path. The 38's piece 14c;
/// until then this was always `None` and the warning named the path.
fn the_version_of(server: &str, path: &str) -> Option<Version> {
    let held = the_searches_held().lock().ok()?;
    match held.get(server)? {
        TheBlenderSearch::ItFound { builds, .. } => {
            builds.iter().find(|build| build.path == path).map(|build| build.version)
        }
        TheBlenderSearch::ItWasNotTaken => None,
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Composing the tab
// ════════════════════════════════════════════════════════════════════════════

/// **`Server Setup` B10's search, as this tab is handed it.**
///
/// **It is a second reading beside [`WhatTheMachineSaid`] and not a limb of
/// it**, because the two answer different questions of the machine: the card
/// reads one setup run (`Server Setup` Y6a, Y6b) and the table is the search
/// (`Server Setup` B10, B11, B12). A run that was never driven says nothing
/// about what
/// Blenders the disk holds, and a search says nothing about a setup.
///
/// **Period-held and never durable** — `Server Setup` B12 and
/// `State, identity and recovery` D4: it is taken fresh when the view opens
/// and nothing is written from it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TheBlenderSearch {
    /// **It did not run** — no connection to the server stands, or the machine
    /// could not be asked. **L8a: this is not *there is no Blender***, and the
    /// table says a reading does not exist.
    #[default]
    ItWasNotTaken,
    /// What it found, with B11's count of folders it could not enter.
    ItFound {
        builds: Vec<ABuildAsRead>,
        folders_the_search_could_not_enter: usize,
    },
}

impl TheBlenderSearch {
    /// **`Server Setup` B1's ask over `Server Setup` B10's search, in the
    /// shape the table draws from.**
    ///
    /// **The candidates that answered are the builds** — `Server Setup`'s own
    /// [`smoozy_server_setup::search::the_listing`], which is the half that
    /// must not list a candidate that did not answer. The ones that did not
    /// answer are B11's naming and are not builds.
    pub fn from_the_ask(
        asked: smoozy_render_settings::acting::versions::TheVersionListAsk,
    ) -> TheBlenderSearch {
        let Some(result) = asked.as_the_page_reads_it() else {
            return TheBlenderSearch::ItWasNotTaken;
        };
        let builds = smoozy_server_setup::search::the_listing(&result.entries)
            .into_iter()
            .filter_map(|candidate| {
                Some(ABuildAsRead {
                    path: candidate.path,
                    version: candidate.answered?,
                    // **B1's long-term-support mark is not carried by the
                    // search**: `ACandidate` keeps the path and the parsed
                    // version and the reading of the `Blender ` line drops the
                    // rest, so the table states the number it was told and
                    // claims nothing about the kind. **Under-stating rather
                    // than guessing**, and it is recorded for the runner.
                    long_term_support: false,
                    // **B8a is read from the record of the act that placed the
                    // build and never off a name**, and no install record names
                    // a path anywhere in this application today — so nothing
                    // was read and nothing is claimed. `None` draws no tag,
                    // which is the only honest row: *your build* and *not
                    // installed by Smoozy* are both assertions about an act
                    // nobody looked up.
                    origin: None,
                    // **B15's route is the same reading and was not taken
                    // either.** Nothing acts on this today: the removal warning
                    // composes its own.
                    the_managers_package: None,
                })
            })
            .collect();
        TheBlenderSearch::ItFound {
            builds,
            folders_the_search_could_not_enter: result.answer.folders_refused,
        }
    }
}

impl TheBlenderSearch {
    /// **`Server Setup` B8a, read from the record of the act that placed the
    /// build and never off a name** — the 38's piece 14b. A build whose path
    /// an install record names takes that record's route: *your build*, or
    /// the ordinary case with no tag.
    ///
    /// **A build no record names keeps no tag** rather than B8a's *not
    /// installed by Smoozy*: install records were first written on 2026-10-08
    /// (piece 14a), so a build Smoozy installed before then has none, and
    /// calling it the software manager's would be false. **Where no records
    /// were read at all, nothing changes.**
    pub fn with_the_install_records(self, records: Option<&[(String, String)]>) -> TheBlenderSearch {
        let (mut builds, folders_the_search_could_not_enter, records) = match (self, records) {
            (TheBlenderSearch::ItFound { builds, folders_the_search_could_not_enter }, Some(records)) => {
                (builds, folders_the_search_could_not_enter, records)
            }
            (other, _) => return other,
        };
        for build in &mut builds {
            if let Some((_, route)) = records.iter().find(|(path, _)| *path == build.path) {
                build.origin = match route.as_str() {
                    smoozy_server_protocol::requests::YOUR_BUILD => Some(HowItGotThere::TheUserPointedAtIt),
                    smoozy_server_protocol::requests::FROM_THE_PUBLISHED_LISTING => {
                        Some(HowItGotThere::SmoozyInstalledItFromTheList)
                    }
                    _ => build.origin,
                };
            }
        }
        TheBlenderSearch::ItFound { builds, folders_the_search_could_not_enter }
    }
}

/// The tab, with the reading handed in — **so the whole of it can be tested**.
pub fn the_tab(said: WhatTheMachineSaid, server: &str) -> TheSetUpTab {
    the_tab_with(said, server, TheBlenderSearch::ItWasNotTaken, false)
}

/// **The tab, with both readings handed in** — the setup run the card is drawn
/// from, and `Server Setup` B10's search the Blender table is drawn from —
/// **and whether Telegram is on in Settings**, which is the desktop's own
/// switch and never the machine's (`Server Setup` G2; `Design/Server Setup §
/// Adding Telegram to a server already set up`: *"It appears only where it is
/// owed — Telegram is on in Settings and this server has not got it"*). The
/// 38's piece 11.
pub fn the_tab_with(
    said: WhatTheMachineSaid,
    server: &str,
    found: TheBlenderSearch,
    telegram_is_on: bool,
) -> TheSetUpTab {
    // **What was last seen is drawn as it was read, greyed and stamped**, and
    // without the stopped setup's panel: that panel says its steps were read
    // from the server just now, and both its controls act on the machine.
    if let WhatTheMachineSaid::LastSeen(what_was_read, at) = said {
        // **No *Install it* while last seen**, for the stopped panel's reason:
        // it acts on the machine, and there is no connection to act over.
        let mut tab = the_tab_with(*what_was_read, server, found, false);
        tab.the_stopped_setup = None;
        tab.the_card_was_last_seen = Some(Words {
            lead: Prose::new(crate::surface::servers::words::LAST_SEEN),
            values: vec![Verbatim::new(
                at.duration_since(std::time::UNIX_EPOCH)
                    .map(|since| since.as_secs().to_string())
                    .unwrap_or_default(),
            )],
            rest: None,
            the_rest_joins: false,
        });
        return tab;
    }
    let the_stopped_setup = match &said {
        WhatTheMachineSaid::ASetupRunAlone(read)
        | WhatTheMachineSaid::ASetupRunAndTheMachine(read, _) => {
            the_panel_a_stopped_setup_opens_on(read).map(|mut panel| {
                the_installs_failure_is_said_on_its_step(&mut panel, read, server);
                panel
            })
        }
        _ => None,
    };
    // **The search is what the table is drawn from wherever one was taken.**
    // Where none was, a full reading's own builds still draw it, and otherwise
    // the table says no reading of this machine's Blenders exists.
    let the_table_where_no_search_was_taken = match &said {
        WhatTheMachineSaid::AReading(reading) => the_table_from(reading),
        _ => the_table_where_nothing_was_read(),
    };
    // **`Server Setup` B10's search taken, and it found no Blender**: the card
    // may not say *Ready to render* (2026-10-26, fault 3). Where no search was
    // taken nothing is known either way, and the card is as before.
    let the_search_found_no_blender =
        matches!(&found, TheBlenderSearch::ItFound { builds, .. } if builds.is_empty());
    let mut the_card = match said {
        WhatTheMachineSaid::NothingHasEverBeenReadFromIt => the_card_where_nothing_was_read(),
        WhatTheMachineSaid::AReading(reading) => {
            let mut reading = *reading;
            reading.telegram_is_switched_on = telegram_is_on;
            the_card_from(&reading)
        }
        WhatTheMachineSaid::ASetupRunAlone(read) => the_card_from_the_run_alone(
            &read,
            a_placement_came_before_the_last_run_on(server),
            None,
            telegram_is_on,
        ),
        WhatTheMachineSaid::ASetupRunAndTheMachine(read, rest) => the_card_from_the_run_alone(
            &read,
            a_placement_came_before_the_last_run_on(server),
            Some(&rest),
            telegram_is_on,
        ),
        // **Reached above and never here.**
        WhatTheMachineSaid::LastSeen(..) => the_card_where_nothing_was_read(),
        WhatTheMachineSaid::ConnectedAndNoRunWasFound => the_card_where_no_run_was_found(),
        WhatTheMachineSaid::ConnectedAndNeverSetUp => the_never_set_up_card_with(false),
        WhatTheMachineSaid::ConnectedAndSmoozyIsNotResponding => {
            the_card_where_smoozy_does_not_answer()
        }
    };
    if the_search_found_no_blender {
        the_card_says_no_blender_was_found(&mut the_card);
    }
    let the_table = match found {
        TheBlenderSearch::ItWasNotTaken => the_table_where_no_search_was_taken,
        TheBlenderSearch::ItFound {
            builds,
            ..
        } => the_table_the_search_found(&builds),
    };
    TheSetUpTab {
        server: Verbatim::new(server),
        this_server: Prose::new(words::THIS_SERVER),
        blender: Prose::new(words::BLENDER),
        read_from_the_server: Prose::new(words::READ_FROM_THE_SERVER),
        the_card,
        the_table,
        the_stopped_setup,
        the_card_was_last_seen: None,
        // **Drawn beside the tab**, from the server's row rather than either
        // reading — see [`the_driver_is_drawn_on`].
        the_driver: None,
        the_driver_warning: None,
        // **Drawn beside the tab**, from the local record — see
        // [`the_installs_smoozy_closed_on`].
        the_installs_never_begun: None,
        // **Drawn beside the tab**, from the run's words this copy keeps —
        // see [`the_servers_words_beside_the_tab`].
        server_output: None,
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The driver — `Servers` H11b and `Server Setup` B12b
// ════════════════════════════════════════════════════════════════════════════

/// **The builds the table is drawn from, by path and answered version** — the
/// search where one was taken, and otherwise a full reading's own builds, as
/// [`the_tab_with`] chooses. Taken before the readings are handed over, so the
/// driver can be drawn beside the rows they become.
pub fn the_builds_the_table_is_drawn_from(
    said: &WhatTheMachineSaid,
    found: &TheBlenderSearch,
) -> Vec<(String, Version)> {
    let builds: &[ABuildAsRead] = match (found, said) {
        (TheBlenderSearch::ItFound { builds, .. }, _) => builds,
        (TheBlenderSearch::ItWasNotTaken, WhatTheMachineSaid::AReading(reading)) => {
            &reading.the_builds
        }
        _ => &[],
    };
    builds
        .iter()
        .map(|build| (build.path.clone(), build.version))
        .collect()
}

/// **The tab with the driver drawn on it** — [`the_tab_with`], and then
/// [`the_driver_is_drawn_on`] beside the same builds. **So the whole of it can
/// be tested**, as the tab is.
pub fn the_tab_with_the_driver(
    said: WhatTheMachineSaid,
    server: &str,
    found: TheBlenderSearch,
    telegram_is_on: bool,
    the_driver: &TheDriverAsHeld,
) -> TheSetUpTab {
    let builds = the_builds_the_table_is_drawn_from(&said, &found);
    let mut tab = the_tab_with(said, server, found, telegram_is_on);
    the_driver_is_drawn_on(&mut tab, the_driver, &builds);
    tab
}

/// **`Servers` H11b and `Server Setup` B12b, drawn on the tab** — the card's
/// driver line, the amber warning and its dot, and the Blender table's tag.
/// Added 2026-10-11, the fourth piece of the OptiX and driver work.
///
/// - **The line** is on every card where the driver answered, whether or not
///   a setup ran, greyed with *last seen* where the reading is not this
///   connection's own (`Design/Server Setup § This server`).
/// - **The warning** is `Servers` H11b's statement, `smoozy_servers::hardware::what_the_bar_says`'
///   own, with the missed bar the row holds as its third input: said on a
///   container, or where a run left the driver below the bar, and nowhere
///   else. **The amber dot comes with it**, on a card the reading is live
///   for and that would otherwise be green or grey — a card that says the
///   connection failed keeps its red one. Where the reading is last seen the
///   warning is drawn greyed with the line, and the dot is left as it is.
/// - **The tag** is `Server Setup` B12b's, **read live**: drawn only beside
///   this connection's own reading, so it stands while a swap is owed and
///   goes by itself once a reading finds the driver at the bar. A table drawn
///   while no connection stands has no rows to carry it.
///
/// **It is drawn after every card is chosen**, the cards an add's own
/// connection leaves included, so no card goes without the line.
pub fn the_driver_is_drawn_on(
    tab: &mut TheSetUpTab,
    held: &TheDriverAsHeld,
    builds: &[(String, Version)],
) {
    let Some(the_driver) = held.the_driver.as_ref() else {
        return;
    };
    let last_seen = held.last_seen_at.map(|at| Words {
        lead: Prose::new(crate::surface::servers::words::LAST_SEEN),
        values: vec![Verbatim::new(
            at.duration_since(std::time::UNIX_EPOCH)
                .map(|since| since.as_secs().to_string())
                .unwrap_or_default(),
        )],
        rest: None,
        the_rest_joins: false,
    });
    let is_live = last_seen.is_none();
    tab.the_driver = Some(TheDriverLine {
        the_driver: Words {
            lead: Prose::new(words::NVIDIA_DRIVER),
            values: vec![Verbatim::new(the_driver.version.as_the_driver_wrote_it())],
            rest: None,
            the_rest_joins: false,
        },
        the_cuda: the_driver.cuda_version.as_ref().map(|cuda| Words {
            lead: Prose::new(words::CUDA_AFTER_THE_DRIVER),
            values: vec![Verbatim::new(cuda.clone())],
            rest: None,
            the_rest_joins: false,
        }),
        last_seen,
    });

    // **The statement is said whatever Blenders are on the server**: it names
    // the earliest bar the driver misses, *and later* (`Servers` H11b).
    let said = smoozy_servers::hardware::what_the_bar_says(
        the_driver,
        &[],
        held.the_driver_missed_the_optix_bar,
    );
    tab.the_driver_warning = said
        .the_statement
        .map(|statement| the_driver_warning(&statement, the_driver.where_k1_placed_the_machine, !is_live));
    if is_live
        && tab.the_driver_warning.is_some()
        && matches!(tab.the_card.dot.as_str(), "ok" | "idle")
    {
        tab.the_card.dot = Verbatim::new("busy");
    }

    if !is_live {
        return;
    }
    let versions: Vec<Version> = builds.iter().map(|(_, version)| *version).collect();
    let below =
        smoozy_server_setup::search::the_builds_the_optix_note_falls_on(Some(the_driver), &versions);
    for row in &mut tab.the_table.rows {
        let it_falls_on_this_row = builds
            .iter()
            .any(|(path, version)| path == row.path.as_str() && below.contains(version));
        if it_falls_on_this_row {
            row.driver_too_old = Some(Prose::new(words::DRIVER_TOO_OLD_FOR_OPTIX));
        }
    }
}

/// **`Servers` H11b's statement in `Design/Server Setup § This server`'s
/// words**: the driver's version, the CUDA version where the driver gave one,
/// and the earliest Blender the missed bar belongs to, *and later*.
///
/// **The second sentence says which case it is.** On a container, *It can't
/// be updated here.* Where a run tried and did not get the driver there, *It
/// could not be updated.*
pub fn the_driver_warning(
    statement: &smoozy_servers::hardware::TheDriverStatement,
    placed: smoozy_servers::hardware::WhereK1PlacesIt,
    last_seen: bool,
) -> TheDriverWarning {
    let cuda = statement
        .cuda_version
        .as_ref()
        .map(|cuda| format!(" ({} {cuda})", words::CUDA))
        .unwrap_or_default();
    let (major, minor, patch) = statement.from_blender;
    // **A Blender is named the way Blender names its own lines**: *5.2*, and
    // *5.2.1* only where the bar begins part-way through one.
    let blender = if patch == 0 {
        format!("{major}.{minor}")
    } else {
        format!("{major}.{minor}.{patch}")
    };
    TheDriverWarning {
        said: Prose::new(format!(
            "{} {}{cuda}, {} {blender} {}",
            words::THIS_SERVERS_DRIVER,
            statement.the_drivers_version,
            words::IS_TOO_OLD_FOR_OPTIX_IN,
            words::AND_LATER,
        )),
        and: Prose::new(match placed {
            smoozy_servers::hardware::WhereK1PlacesIt::AContainer => {
                words::SMOOZY_CANT_UPDATE_IT_HERE
            }
            _ => words::SMOOZY_COULDNT_UPDATE_IT,
        }),
        last_seen,
    }
}

/// **The driver as the server's row holds it**, in the state `Servers`' own
/// judgement puts the row's reading in ([`crate::surface::servers::the_hardware_state_this_run_knows`],
/// called rather than repeated): live, last seen with this computer's moment
/// of it, or nothing where no reading may be shown.
pub fn the_driver_as_the_row_holds_it(
    row: &smoozy_servers::store::ServerRow,
    state: smoozy_servers::hardware::HardwareState,
) -> TheDriverAsHeld {
    use smoozy_servers::hardware::HardwareState;
    let reading = row.remembered_hardware_reading.as_ref();
    let (the_driver, last_seen_at) = match (state, reading) {
        (HardwareState::NeverRead, _) | (_, None) => (None, None),
        (HardwareState::Live, Some(reading)) => (reading.the_driver.clone(), None),
        (HardwareState::LastSeen, Some(reading)) => (
            reading.the_driver.clone(),
            Some(reading.the_desktops_own_moment.taken_at()),
        ),
    };
    TheDriverAsHeld {
        the_driver,
        the_driver_missed_the_optix_bar: row.the_driver_missed_the_optix_bar,
        last_seen_at,
    }
}

/// **The same, read for one server from the application's own state.**
/// Nothing is drawn where the record will not read or holds no row.
pub(crate) fn the_driver_as_held_for<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
) -> TheDriverAsHeld {
    use tauri::Manager;
    let row = (|| {
        let started = app.try_state::<Mutex<Started>>()?;
        let started = started.lock().ok()?;
        crate::record::server_rows::one(started.record.as_ref()?.connection(), server)
            .ok()
            .flatten()
    })();
    let Some(row) = row else {
        return TheDriverAsHeld::default();
    };
    let periods = crate::surface::servers::the_periods_of(app);
    let periods = periods.as_deref();
    let state = crate::surface::servers::the_hardware_state_this_run_knows(
        row.remembered_hardware_reading.as_ref(),
        periods.and_then(|periods| periods.what_the_last_connection_read(server)),
        crate::surface::servers::what_is_true_of(periods, server),
    );
    the_driver_as_the_row_holds_it(&row, state)
}

/// **The run panel for a setup that stopped part-way, and for nothing else.**
///
/// **Stopped part-way** is `Server Setup` S1's reading: a run with a recorded
/// ending that never reached *done*. **Not** a run still going — that one is
/// being driven and has its own shape — **not** one that finished, and **not**
/// `Server Setup` S6's abandon, after which the card reads *Not set up* and
/// nothing stands to be finished or given up. Added 2026-09-22.
fn the_panel_a_stopped_setup_opens_on(
    read: &crate::setting_up::TheSetupAsRead,
) -> Option<TheRunPanel> {
    let record = &read.record;
    let abandoned = matches!(record.ending, Some(HowTheRunEnded::Abandoned { .. }));
    if read.still_open || record.a_setup_finished() || abandoned {
        return None;
    }
    let mut panel = the_run_panel(
        record,
        read.the_step_in_flight,
        &read.the_clock,
        false,
        record.the_composed_list.contains(&Boundary::Libraries),
    );
    the_tools_own_install_is_named(&mut panel, read);
    // `Server Setup` S6 clears the machine-level hold, not the failed operation record.
    // The RunPod pass found this offer on a record with no hold to clear.
    if !read.the_run_holds_the_machine {
        panel.abandon = None;
        panel.the_closing_paragraph = Some(TheClosingParagraph {
            before: Prose::new(words::THE_RETRY_PARAGRAPH_ONE),
            the_controls_name: Prose::new(words::SET_UP_AGAIN),
            after: Prose::new(words::THE_RETRY_PARAGRAPH_TWO),
        });
    }
    Some(panel)
}

/// **A tooling step that failed at the unpack tool's own install says so** —
/// *Could not install the unpack tool.*, `Design/Server Setup § The step that
/// failed`. The record names the step and not which of its two acts failed, so
/// the step read the unpack's sentence (now *Could not unpack Blender.*)
/// whichever it was. **The reading tells them apart**: the tool's install is an
/// admin step the desktop records as started, and that mark still stands when
/// its failure ends the run; the unpack is a worker step, commanded after the
/// mark is cleared ([`crate::setting_up::TheSetupAsRead::the_admin_step_recorded_as_started`]).
/// Where the mark is not there the unpack's sentence stands, as before. Added
/// 2026-10-25, the small owed fixes after the eighteenth notes.
fn the_tools_own_install_is_named(panel: &mut TheRunPanel, read: &crate::setting_up::TheSetupAsRead) {
    let at_the_install = matches!(
        read.record.ending,
        Some(HowTheRunEnded::AStepFailed { step: smoozy_server_setup::ready_action::Step::Tooling, .. })
    ) && read.the_admin_step_recorded_as_started == Some(Boundary::Tooling);
    if !at_the_install {
        return;
    }
    for step in panel.steps.iter_mut().filter(|step| step.boundary == Boundary::Tooling) {
        if let Some(failed) = step.the_failure.as_mut() {
            if failed.said.lead.as_str() == Prose::new(words::FAILED_UNPACK).as_str() {
                failed.said = plainly(words::FAILED_TOOLING);
            }
        }
    }
}

/// **A failed Blender install's own sentence, kept for the step it failed on**
/// — `Design/Server Setup § Installing`: the build's name and the versions not
/// installed join the step's failure line, and no line above the steps repeats
/// it. By server: the step the record names, and the line's sentence after
/// *Install stopped.*, which the card's heading already says. **Each install's
/// ending replaces what is kept**, so it is only ever the last one's. Memory
/// only: a copy opened later draws the step's own sentence. Added 2026-10-25,
/// the small owed fixes after the eighteenth notes.
fn the_installs_failures_kept() -> &'static Mutex<Vec<(String, smoozy_server_setup::ready_action::Step, String)>> {
    static KEPT: std::sync::OnceLock<Mutex<Vec<(String, smoozy_server_setup::ready_action::Step, String)>>> =
        std::sync::OnceLock::new();
    KEPT.get_or_init(|| Mutex::new(Vec::new()))
}

fn an_installs_failure_is_kept(server: &str, kept: Option<(smoozy_server_setup::ready_action::Step, String)>) {
    if let Ok(mut held) = the_installs_failures_kept().lock() {
        held.retain(|(one, _, _)| one != server);
        if let Some((step, said)) = kept {
            held.push((server.to_string(), step, said));
        }
    }
}

/// **Where the card's stopped setup is that failed install, its failed step
/// says the install's own sentence, and the last act's line is not drawn**
/// ([`TheRunPanel::the_last_act_is_said_on_its_steps`]). This is the one case
/// an install's steps stay on the card once its result is read: the run the
/// machine-level state stands for (`Server Setup` S1), which an install leaves
/// where it installed a package before it failed. **Everywhere else the card
/// keeps no steps once the result is read, and the line is where the failure
/// shows.** Added 2026-10-25, the small owed fixes after the eighteenth notes.
fn the_installs_failure_is_said_on_its_step(panel: &mut TheRunPanel, read: &crate::setting_up::TheSetupAsRead, server: &str) {
    let Some(HowTheRunEnded::AStepFailed { step, .. }) = &read.record.ending else {
        return;
    };
    if read.record.kind != SetupKind::ABlenderInstall {
        return;
    }
    let Some(said) = the_installs_failures_kept().lock().ok().and_then(|held| {
        held.iter()
            .find(|(one, its_step, _)| one == server && its_step == step)
            .map(|(_, _, said)| said.clone())
    }) else {
        return;
    };
    let Some(failed) = panel.steps.iter_mut().find_map(|step| step.the_failure.as_mut()) else {
        return;
    };
    failed.said = plainly(&said);
    panel.the_last_act_is_said_on_its_steps = true;
}

/// **The card where no reading of the machine has ever existed** — L8a, and
/// `Design/Server Setup § Cut off part-way`'s one server that shows nothing
/// about a setup.
///
/// **The action is still offered and is not loud.** `Design/Server Setup § This
/// server` is Creator-stated that it is never greyed out and that the loud
/// reading is reserved for a machine Smoozy can name a reason about; a machine
/// nothing was ever read from is not one of those, because *Not set up* is
/// itself a reading.
fn the_card_where_nothing_was_read() -> TheCard {
    TheCard {
        dot: Verbatim::new("idle"),
        state: Prose::new(words::NEVER_READ),
        when: None,
        what_was_done: Vec::new(),
        the_render_device: None,
        the_setup_fault: None,
        the_note: None,
        the_action: TheAction {
            name: Prose::new(words::SET_UP_THIS_SERVER),
            loud: false,
            underway: false,
            act: Verbatim::new(THE_READY_ACTION),
        },
        instead: Some(plainly(words::NEVER_READ_WHY)),
    }
}

/// **The card drawn from one run's record and nothing else.** Added
/// 2026-09-21.
///
/// **The card's own composition is used whole** — [`the_card_from`], so a run
/// still open reads *Setting up*, one stopped part-way reads *Setup stopped
/// part-way* with the loud *Set up again*, and one that finished reads what it
/// did. **The rest of the card's reading, where the machine gave one, fills
/// the Telegram line and `Server Setup` W3a's and W4's lines** (the 38's piece
/// 6); **where the machine's finished jobs were not read, those two lines are
/// taken out** rather than drawn from nothing.
fn the_card_from_the_run_alone(
    read: &crate::setting_up::TheSetupAsRead,
    a_placement_came_first: bool,
    the_rest: Option<&crate::setting_up::TheCardsReading>,
    telegram_is_on: bool,
) -> TheCard {
    // **`Server Setup` Y6b: the card reads the ready action's record, and a run
    // of another kind is not one.** The run read here is the one the standing
    // state names, or the one this copy last drove — and on step 10c's third
    // pass on Vast that was a Blender install, so the card found no ready-action
    // record in a reading that never looked for one and said *Not set up* on a
    // machine with a verified installation. **A reading of another kind says
    // nothing about whether the server was set up**, so the card says what it
    // does know: a run of that kind still going reads as running, one stopped
    // part-way reads as stopped (it holds the machine exactly as a stopped setup
    // does, and *Set up again* takes it over), and anything else is the card
    // for a server where no setup was read. Added 2026-09-22.
    if read.record.kind != smoozy_server_setup::run::SetupKind::TheReadyAction && !read.still_open {
        let over = matches!(
            read.record.ending,
            Some(HowTheRunEnded::Abandoned { .. }) | Some(HowTheRunEnded::Completed)
        );
        if read.record.a_setup_finished() || over {
            return the_card_where_no_run_was_found();
        }
    }
    let what_else = the_rest.map(what_else_the_card_reads);
    let the_jobs = what_else.as_ref().and_then(|it| it.the_jobs.clone());
    let reading = ASetUpReading {
        the_records: if read.still_open {
            Vec::new()
        } else {
            vec![read.record.clone()]
        },
        telegram_reporting_is_installed: what_else
            .as_ref()
            .is_some_and(|it| it.telegram_reporting_is_installed),
        // **Only where the machine's own reading of the role was taken**: with
        // no reading of it, *not installed* would be a claim nobody read.
        telegram_is_switched_on: telegram_is_on && what_else.is_some(),
        a_setup_is_running: read.still_open.then(|| read.record.clone()),
        the_step_in_flight: read.the_step_in_flight,
        the_finished_jobs: the_jobs
            .as_ref()
            .map(|(answered, _)| answered.clone())
            .unwrap_or(WhatTheThreeQuestionsAnswered {
                any_finished_job_record_exists: false,
                the_most_recent: None,
                the_most_recent_finished_record_carrying_a_device_line: None,
            }),
        the_device_reading: the_jobs.and_then(|(_, device)| device),
        the_builds: Vec::new(),
        the_library_half_was_read: read.record.the_composed_list.contains(&Boundary::Libraries),
        folders_the_search_could_not_enter: 0,
        the_clock: read.the_clock.clone(),
        a_placement_came_first,
    };
    let mut card = if read.record.kind != smoozy_server_setup::run::SetupKind::TheReadyAction && !read.still_open {
        the_stopped_card(&reading, &read.record)
    } else {
        the_card_from(&reading)
    };
    // **Where the machine's finished jobs were not read**, the two lines that
    // are readings of them are taken out rather than drawn from nothing.
    if what_else.and_then(|it| it.the_jobs).is_none() {
        card.the_render_device = None;
        card.the_setup_fault = None;
    }
    card
}

/// **The rest of the card's reading, in the card's own terms** —
/// `Server Setup` Y6b's Telegram line, and `Server Setup` W3a's three answers
/// with `Server Setup` W4's three
/// conditions for the job that carries the line. The 38's piece 6.
struct WhatElseTheCardReads {
    telegram_reporting_is_installed: bool,
    /// `None` where the machine's jobs could not be read.
    the_jobs: Option<(WhatTheThreeQuestionsAnswered, Option<ADeviceReading>)>,
}

fn what_else_the_card_reads(rest: &crate::setting_up::TheCardsReading) -> WhatElseTheCardReads {
    use crate::setting_up::DeviceAsked;
    WhatElseTheCardReads {
        // **`Server Setup` Y6b: the role's own desired state, never a line of
        // the run**; a
        // state that could not be read draws no line.
        telegram_reporting_is_installed: rest.the_role_is_enabled == Some(true),
        the_jobs: rest.the_jobs.as_ref().map(|jobs| {
            let answered = WhatTheThreeQuestionsAnswered {
                any_finished_job_record_exists: jobs.the_most_recent.is_some(),
                the_most_recent: jobs.the_most_recent.clone(),
                the_most_recent_finished_record_carrying_a_device_line: jobs
                    .the_finished_job_with_a_line
                    .as_ref()
                    .map(|(job, ..)| job.clone()),
            };
            // **The machine selects only a job whose recorded outcome is
            // *finished*** (`Server Setup` W3a's third question,
            // `Server Setup` W4's gate), so that is the outcome here; the line is read for W4's third condition and
            // carried whole for the words.
            let device = jobs
                .the_finished_job_with_a_line
                .as_ref()
                .map(|(_, line, asked_for)| ADeviceReading {
                    outcome: RecordedOutcome::Finished,
                    asked_for: asked_for.map(|asked| match asked {
                        DeviceAsked::TheCards => WhatTheManifestAskedFor::TheCards,
                        DeviceAsked::TheProcessor => WhatTheManifestAskedFor::TheProcessor,
                    }),
                    line: Some(smoozy_server_setup::statements::what_the_line_names(line)),
                    the_lines_words: line.clone(),
                });
            (answered, device)
        }),
    }
}

/// **`Server Setup` Y6a's *never set up* on a machine that answered the card's
/// reading and holds no ready-action record** — the record's presence and
/// nothing else — with the Telegram line beside it, which `Server Setup` G3
/// makes true on a machine never prepared. The 38's piece 6.
pub fn the_reading_where_no_setup_record_stands(
    rest: &crate::setting_up::TheCardsReading,
    the_clock: String,
) -> ASetUpReading {
    let what_else = what_else_the_card_reads(rest);
    let (the_finished_jobs, the_device_reading) = what_else.the_jobs.unwrap_or((
        WhatTheThreeQuestionsAnswered {
            any_finished_job_record_exists: false,
            the_most_recent: None,
            the_most_recent_finished_record_carrying_a_device_line: None,
        },
        None,
    ));
    ASetUpReading {
        the_records: Vec::new(),
        telegram_reporting_is_installed: what_else.telegram_reporting_is_installed,
        telegram_is_switched_on: false,
        a_setup_is_running: None,
        the_step_in_flight: None,
        the_finished_jobs,
        the_device_reading,
        the_builds: Vec::new(),
        the_library_half_was_read: false,
        folders_the_search_could_not_enter: 0,
        the_clock,
        a_placement_came_first: false,
    }
}

/// **Connected, and no run found to read.** Added 2026-09-21. **The words are
/// not drawn** — `Design/Server Setup` draws no state for it, and the account
/// names that — so they say what Smoozy knows and nothing more (Rule 8).
/// **The card where the server is connected and Smoozy's service on it does
/// not answer** — `Design/Server Setup § This server`: *"the card's state line
/// says the row's words, Smoozy isn't responding., and never that Smoozy has
/// not read the server yet"*. **It promises nothing about the provider starting
/// the service again** (`Rented-server runtime` §16 item 16), and **nothing
/// here repairs**. The action stays on the card, quiet and never greyed out,
/// as `Design/Server Setup § This server` keeps it on every card; the server
/// was set up, so it is *Set up again*. Added 2026-09-27, step 10c's
/// eighth-pass fix piece 6.
fn the_card_where_smoozy_does_not_answer() -> TheCard {
    TheCard {
        dot: Verbatim::new("idle"),
        state: Prose::new(crate::surface::servers::words::SMOOZY_ISNT_RESPONDING),
        when: None,
        what_was_done: Vec::new(),
        the_render_device: None,
        the_setup_fault: None,
        the_note: None,
        the_action: TheAction {
            name: Prose::new(words::SET_UP_AGAIN),
            loud: false,
            underway: false,
            act: Verbatim::new(THE_READY_ACTION),
        },
        instead: None,
    }
}

fn the_card_where_no_run_was_found() -> TheCard {
    TheCard {
        dot: Verbatim::new("idle"),
        state: Prose::new(words::NO_SETUP_WAS_READ),
        when: None,
        what_was_done: Vec::new(),
        the_render_device: None,
        the_setup_fault: None,
        the_note: None,
        the_action: TheAction {
            name: Prose::new(words::SET_UP_THIS_SERVER),
            loud: false,
            underway: false,
            act: Verbatim::new(THE_READY_ACTION),
        },
        instead: Some(plainly(words::NO_SETUP_WAS_READ_WHY)),
    }
}

fn the_table_where_nothing_was_read() -> TheBlenderTable {
    TheBlenderTable {
        version: Prose::new(words::VERSION),
        location: Prose::new(words::WHERE),
        rows: Vec::new(),
        instead: Some(plainly(words::NO_BLENDER_READING)),
        install: Prose::new(words::INSTALL_BLENDER),
    }
}

/// **The five acts this tab's controls hand back.** Values the page returns
/// unaltered, so no act's name is written on the page.
pub const THE_READY_ACTION: &str = "the-ready-action";
/// **The ready action, asked for again from `Server Setup` W3's warning** — its
/// *Set up anyway*. The same act, with the warning already shown. Added
/// 2026-09-27.
pub const THE_READY_ACTION_AFTER_THE_WARNING: &str = "the-ready-action-after-the-warning";
pub const ABANDON: &str = "abandon";
pub const STOP: &str = "stop";
pub const INSTALL_A_BLENDER: &str = "install-a-blender";
pub const REMOVE_A_BLENDER: &str = "remove-a-blender";
/// **`Server Setup` G2's dedicated entry point** — *Install it*. Added by step
/// 10 piece 8.
pub const INSTALL_TELEGRAM: &str = "install-telegram";
/// **`Servers` B6's connect that carries the remembered setup** — the action
/// of the card where the connection an add asked for was not made
/// (`Design/Server Setup § This server`, the Creator's note 14 on the test
/// copy). **It is the row's own act, under the row's own name**, and this tab
/// hands it to `Servers` rather than performing it.
pub const CONNECT_AND_SET_UP: &str = "connect-and-set-up";
/// **`Server Setup` B7's *choose a build on your computer***: the system's
/// own file picker (`Desktop app and delivery` K11 to K13), then the upload
/// and the run. Added 2026-10-09, the 38's piece 14b.
pub const CHOOSE_A_BUILD: &str = "choose-a-build";
/// **`Server Setup` B7a's two answers**, from the card's question (`su-low`).
pub const INSTALL_ANYWAY: &str = "install-anyway";
pub const DONT_INSTALL_IT: &str = "dont-install-it";

/// **The card, composed from one reading** — `Server Setup` Y6a and Y6b.
pub fn the_card_from(reading: &ASetUpReading) -> TheCard {
    // **A setup running now outranks every other reading of the card**, because
    // the action is never offered twice — `Design/Server Setup § A second setup
    // while one is running`.
    if let Some(running) = &reading.a_setup_is_running {
        return the_running_card(reading, running);
    }

    // **Y6b: the card reads the ready action's own record and no other kind.**
    let record = smoozy_server_setup::run::the_card_reads(&reading.the_records);
    match record {
        None => the_never_set_up_card(reading),
        // **`Server Setup` S6's abandon reads *Not set up*** —
        // `Design/Server Setup § The two ways out`: *"The card is the half that
        // still says something, and it says Not set up"*, on `Server Setup`
        // Y6a's own terms, a setup that happened being a run whose record
        // reached the last step and an abandoned run never reaching it. Added
        // 2026-09-22.
        Some(record) if matches!(record.ending, Some(HowTheRunEnded::Abandoned { .. })) => {
            the_never_set_up_card(reading)
        }
        Some(record) if !record.a_setup_finished() => the_stopped_card(reading, record),
        Some(record) => the_finished_card(reading, record),
    }
}

/// **A finished setup's card on a server where the search found no Blender
/// reads *Set up*, and never *Ready to render*** — Rule 8: a server with no
/// Blender cannot render, whatever the run did. **Nothing else on the card
/// moves**: its steps still say what the run did, and the Blender section
/// below says what is on the server. Added 2026-10-26, the worldwide pass's
/// setup faults, fault 3.
fn the_card_says_no_blender_was_found(card: &mut TheCard) {
    if card.state == Prose::new(words::READY_TO_RENDER) {
        card.state = Prose::new(words::SET_UP_STATE);
    }
}

/// **`Server Setup` Y6a: no ready-action record at all is *never set up*.**
///
/// **The two readings Design names are one reading with one answer** — no
/// record, or a machine that failed the identity check, the second being
/// disqualified before any of this is read (`State, identity and recovery`
/// §2.3). **Running the action is the main thing on the card, so it is the loud
/// control.**
fn the_never_set_up_card(reading: &ASetUpReading) -> TheCard {
    the_never_set_up_card_with(reading.telegram_reporting_is_installed)
}

/// The same card, with whether Telegram reporting is installed handed in.
fn the_never_set_up_card_with(telegram_only: bool) -> TheCard {
    TheCard {
        dot: Verbatim::new("idle"),
        state: Prose::new(words::NOT_SET_UP),
        when: None,
        what_was_done: if telegram_only {
            vec![ALineOfWhatWasDone {
                said: plainly(words::TELEGRAM_INSTALLED),
                mark: None,
                colour: Verbatim::new(the_marks_colour::PLAIN),
                missing: false,
                offer: None,
            }]
        } else {
            Vec::new()
        },
        the_render_device: None,
        the_setup_fault: None,
        // **`Server Setup` G3: the dedicated entry point does only its own
        // work, so the card goes on saying the server is not set up while
        // Telegram reporting is installed on it. That reads oddly until it is
        // said plainly, so the line says it.**
        the_note: telegram_only.then(|| plainly(words::TELEGRAM_AND_THE_REST_IS_NOT_RUN)),
        the_action: TheAction {
            name: Prose::new(words::SET_UP_THIS_SERVER),
            loud: true,
            underway: false,
            act: Verbatim::new(THE_READY_ACTION),
        },
        instead: None,
    }
}

/// **A ready action that finished** — `Server Setup` Y6a's *that a setup
/// happened, when, and what it consisted of*.
fn the_finished_card(reading: &ASetUpReading, record: &TheSetupRecord) -> TheCard {
    // **W1: *Nothing to prepare* is the run's own ending and not a quiet kind
    // of failure**, and the step list is what makes it believable.
    let every_step_had_nothing_to_do = matches!(record.ending, Some(HowTheRunEnded::Completed))
        && record
            .the_boundaries_reached
            .iter()
            .filter(|it| the_boundarys_name(it.boundary).is_some())
            .all(|it| it.what_the_step_did == WhatTheStepDid::NothingToDo);
    // **W1: *every check passed before any act was needed* — and a placement
    // of Smoozy's program before the run is an act**, so a run after one reads
    // as a setup that did something, in the words the card already has, and
    // its list still says what each step did. Step 10c's twelfth-pass fix
    // piece 6: the twelfth pass read *Nothing to prepare … Nothing was
    // installed and nothing was changed.* after a placement.
    let nothing_to_prepare = every_step_had_nothing_to_do && !reading.a_placement_came_first;

    let fault = the_setup_fault(reading);
    TheCard {
        // **The drawing's amber dot on the fault reading**, its own `ok`
        // otherwise.
        dot: Verbatim::new(if fault.is_some() { "busy" } else { "ok" }),
        state: Prose::new(if nothing_to_prepare {
            words::NOTHING_TO_PREPARE
        } else {
            words::READY_TO_RENDER
        }),
        when: the_moment(
            if nothing_to_prepare {
                words::CHECKED
            } else {
                words::SET_UP
            },
            record.when_it_finished().map(|(at, _)| at),
            &reading.the_clock,
        ),
        what_was_done: what_was_done(reading, record),
        // **The fault reading draws no device line**: the drawing puts the
        // amber sentence in its place, and one card saying both would say the
        // same fact twice.
        the_render_device: if fault.is_some() {
            None
        } else {
            the_render_device_line(reading)
        },
        the_setup_fault: fault,
        the_note: nothing_to_prepare.then(|| plainly(words::NOTHING_WAS_INSTALLED)),
        the_action: TheAction {
            name: Prose::new(words::SET_UP_AGAIN),
            // **Loud only where Smoozy can name a reason to press it**, which
            // on a finished setup is the setup fault and nothing else.
            loud: the_setup_fault(reading).is_some(),
            underway: false,
            act: Verbatim::new(THE_READY_ACTION),
        },
        instead: None,
    }
}

/// **A ready action whose record exists and never reached *done*.**
///
/// **`Server Setup` Y6a is what settles that nothing on the card may say the
/// server is ready**: a setup that happened is a run whose record reached the
/// last step, and an abandoned or stopped run never reaches it.
fn the_stopped_card(reading: &ASetUpReading, record: &TheSetupRecord) -> TheCard {
    TheCard {
        dot: Verbatim::new("idle"),
        state: Prose::new(words::SETUP_STOPPED_PART_WAY),
        when: the_moment(
            words::SET_UP,
            record.the_last_boundary_reached().map(|it| it.at),
            &reading.the_clock,
        ),
        what_was_done: Vec::new(),
        the_render_device: None,
        the_setup_fault: None,
        the_note: None,
        the_action: TheAction {
            name: Prose::new(words::SET_UP_AGAIN),
            loud: true,
            underway: false,
            act: Verbatim::new(THE_READY_ACTION),
        },
        instead: None,
    }
}

/// **A setup running now** — `Design/Server Setup § A second setup while one is
/// running`.
///
/// **The card carries no step list of its own.** The panel is the one place a
/// run's steps are drawn, and one thing never wears two underway shapes at
/// once. **No red: nothing has gone wrong.**
fn the_running_card(reading: &ASetUpReading, running: &TheSetupRecord) -> TheCard {
    TheCard {
        dot: Verbatim::new("busy"),
        state: Prose::new(words::SETTING_UP),
        // **`Design/Server Setup` writes *started 2 minutes ago* and this says
        // the moment instead.** An elapsed time is the machine's clock
        // subtracted from the desktop's, which `State, identity and recovery`
        // D5 forbids outright and §16.1 M3a is written to stop. The moment, its
        // clock named beside it, says the same thing and is true.
        when: the_moment(
            words::STARTED,
            running.the_boundaries_reached.first().map(|it| it.at),
            &reading.the_clock,
        ),
        what_was_done: Vec::new(),
        the_render_device: None,
        the_setup_fault: None,
        the_note: Some(plainly(words::ONE_SETUP_AT_A_TIME)),
        the_action: TheAction {
            name: Prose::new(words::SET_UP_AGAIN),
            loud: false,
            underway: true,
            act: Verbatim::new(THE_READY_ACTION),
        },
        instead: None,
    }
}

/// **The card's list: always the last setup's steps, each with its mark** —
/// `Design/Server Setup § This server`: *done*, *nothing to do*, *not checked
/// …*, whether that setup installed everything or found nothing to do, and
/// *Install the GPU driver — done* is what says the driver went in.
/// **Telegram reporting's line sits under the steps**, because it is the
/// server's own reading and not a step of the run (`Server Setup` Y6b), **and
/// only where it says something the steps do not**: where the run lists the
/// Telegram step and the server reads it installed, the line would say it
/// twice. *(2026-10-29, the Creator's twenty-first notes, note 1.)*
///
/// *(Until 2026-10-02 the steps were listed only on a run where every step had
/// nothing to do, and any other run listed a* Graphics driver installed *line
/// and Telegram's — so a first setup that installed a Blender showed no list at
/// all, and* Set up again *showed every step. The Creator's note 6 on 0.1.7:
/// "it should be consistent in that regard".)*
fn what_was_done(reading: &ASetUpReading, record: &TheSetupRecord) -> Vec<ALineOfWhatWasDone> {
    let mut lines: Vec<ALineOfWhatWasDone> = record
        .the_boundaries_reached
        .iter()
        .filter_map(|it| {
            // **The run's own last step is a line of this list too**, as it is
            // of the running card's: the list is *always the last setup's
            // steps*. A finished card is a run that completed, so its final
            // check passed, and it reads *done*. The pass before 0.1.9 on
            // Vast: *Run Smoozy's final check* was listed while the run went
            // and gone from the finished card.
            let (name, (mark, colour)) = match it.boundary {
                Boundary::Done => (words::DONE_STEP, (words::DONE, the_marks_colour::DONE)),
                other => (
                    the_boundarys_name(other)?,
                    the_marks_words(it, reading.the_library_half_was_read),
                ),
            };
            Some(ALineOfWhatWasDone {
                said: plainly(name),
                mark: Some(Prose::new(mark)),
                colour: Verbatim::new(colour),
                missing: false,
                offer: None,
            })
        })
        .collect();

    // **Y6b: the Telegram line is the role's own reading and never a line of
    // the run.** `smoozy_server_setup::run::the_telegram_line_is_read_from_the_boundary_list`
    // answers false, and this is that clause kept.
    //
    // **`Design/Server Setup § This server`: drawn only where it says
    // something the steps do not.** A run that reached the Telegram step
    // already lists *Install Telegram reporting* with its mark, so where the
    // server agrees it is installed nothing more is drawn. Telegram added later
    // by *Install it* leaves the ready action's steps without it, and keeps
    // the line. The Creator's twenty-first notes, note 1, 2026-10-29.
    let the_steps_list_telegram = record
        .the_boundaries_reached
        .iter()
        .any(|it| it.boundary == Boundary::Telegram);
    if reading.telegram_reporting_is_installed {
        if !the_steps_list_telegram {
            lines.push(ALineOfWhatWasDone {
                said: plainly(words::TELEGRAM_INSTALLED),
                mark: None,
                colour: Verbatim::new(the_marks_colour::PLAIN),
                missing: false,
                offer: None,
            });
        }
    } else if reading.telegram_is_switched_on {
        // **`Design/Server Setup § Adding Telegram to a server already set up`:
        // it appears only where it is owed**, and it sits in the list rather
        // than beside the state line, with *Install it* on it.
        //
        // **Pressing it is answered, not acted on, until the machine's program
        // can finish `Server Setup` G2's run** (the 38's piece 11's account,
        // section 1): it gets this tab's one sentence. Added by step 10 piece 8.
        lines.push(ALineOfWhatWasDone {
            said: plainly(words::TELEGRAM_NOT_INSTALLED),
            mark: Some(Prose::new(words::THE_SERVER_IS_NOT_RESTARTED)),
            // **Not a step's mark**: it says what *Install it* will not do, and
            // the drawing leaves it plain.
            colour: Verbatim::new(the_marks_colour::PLAIN),
            missing: true,
            offer: Some(ALinesOffer { name: Prose::new(words::INSTALL_IT), act: INSTALL_TELEGRAM }),
        });
    }
    lines
}

/// **`Server Setup` Y4a's two marks, and the third that is not a mark** —
/// each with its colour (`Design/Server Setup § This server`).
fn the_marks_words(
    recorded: &RecordedBoundary,
    the_library_half_was_read: bool,
) -> (&'static str, &'static str) {
    // **`Server Setup` R4 and W1: where the machine held no Blender and the run
    // fetched none, the library check had no subject.** That step reads
    // differently from every other, because a step that never ran is not a step
    // that found nothing to do — and collapsing the two would have Smoozy
    // claiming a check it never made.
    //
    // **It is read off the run and never off the boundary list.** Y4a's mark
    // says whether the step acted; **Y3 writes the boundary whatever the step
    // found**, so no reading of the list can tell a check that passed from one
    // that had nothing to check.
    if recorded.boundary == Boundary::Libraries && !the_library_half_was_read {
        return (words::NOT_CHECKED_NO_BLENDER, the_marks_colour::NOT_CHECKED);
    }
    match recorded.what_the_step_did {
        WhatTheStepDid::Performed => (words::DONE, the_marks_colour::DONE),
        WhatTheStepDid::NothingToDo => (words::NOTHING_TO_DO, the_marks_colour::NOTHING_TO_DO),
    }
}

/// **`Server Setup` Y4's boundary names, one per boundary.**
///
/// **Six are the drawing's own words** and are used exactly as it writes them.
/// **Four are ruled here, under the Creator's standing delegation of
/// 2026-08-31**, because Y4's list is ten and the mockup's two step lists draw
/// only the six a run that fetches no Blender contains. Each is the standard
/// software word for what the step does and asserts nothing else.
///
/// **`Done` has no name because it is not drawn, and answering nothing is what
/// keeps it out of every list.** It is the run's own final verification, and
/// the panel's state line already says the run finished; a list line reading
/// *done — done* is the ending said twice. **The drawing's own two step lists
/// omit it**, which is what this follows — and both lists here are built by
/// keeping the boundaries this function names, so no caller has to remember.
pub fn the_boundarys_name(boundary: Boundary) -> Option<&'static str> {
    match boundary {
        Boundary::Started => Some(words::READINGS),
        Boundary::Fetched => Some(words::FETCHED),
        Boundary::Tooling => Some(words::TOOLING),
        Boundary::Libraries => Some(words::LIBRARIES),
        Boundary::DriverPlaced => Some(words::DRIVER_PLACED),
        Boundary::RestartIssued => Some(words::RESTART_ISSUED),
        Boundary::CardAnswering => Some(words::CARD_ANSWERING),
        Boundary::Blender => Some(words::BLENDER_STEP),
        Boundary::Removed => Some(words::REMOVED_STEP),
        Boundary::Telegram => Some(words::TELEGRAM_STEP),
        // **`None` here, and it is not an omission.** The card's own *what was
        // done to the machine* is a list of changes, and the final check
        // changes nothing — so it has no line there. **The run panel's step
        // list is a different question** — how far the run got — and it names
        // this step itself, at [`a_step_line`].
        Boundary::Done => None,
    }
}

/// **One plain sentence saying what that step did not do** —
/// `Design/Server Setup § The step that failed`.
///
/// **Smoozy writes no second sentence here.** The failure shape grants one only
/// where Smoozy knows what the user should do next, and a step refused on a
/// rented machine is not something it can read a cause off. What it has is the
/// machine's words, one control away.
///
/// **One of these is the drawing's own** — *The server could not install the
/// GPU driver.* **The other eight are ruled here**, under the Creator's
/// standing delegation of 2026-08-31, because the drawing draws one failed step
/// and Y4's list is ten. Each names the step that did not finish and asserts
/// nothing else about why.
pub fn the_boundarys_failure_sentence(boundary: Boundary) -> Option<&'static str> {
    match boundary {
        Boundary::Started => Some(words::FAILED_READINGS),
        Boundary::Fetched => Some(words::FAILED_FETCH),
        Boundary::Tooling => Some(words::FAILED_UNPACK),
        Boundary::Libraries => Some(words::FAILED_LIBRARIES),
        Boundary::DriverPlaced => Some(words::FAILED_DRIVER),
        Boundary::RestartIssued => Some(words::FAILED_RESTART),
        Boundary::CardAnswering => Some(words::FAILED_CARD_CHECK),
        Boundary::Blender => Some(words::FAILED_BLENDER),
        Boundary::Removed => Some(words::FAILED_REMOVAL),
        Boundary::Telegram => Some(words::FAILED_TELEGRAM),
        // **The sentence the failure box already says for this step**, so the
        // step's own failure shape and the box agree word for word.
        Boundary::Done => Some(words::FAILED_FINAL_CHECK),
    }
}

/// **`Server Setup` W3a's one line about the cards, and W4's conclusion inside
/// it.**
fn the_render_device_line(reading: &ASetUpReading) -> Option<Words> {
    match which_line(&reading.the_finished_jobs) {
        WhichLine::NothingHasRunHereYet => Some(plainly(words::NOTHING_HAS_RENDERED_YET)),
        WhichLine::JobsHaveRunHereAndNoneOfThemSaid => Some(plainly(words::NONE_OF_THEM_SAID)),
        WhichLine::TheDeviceLinesConclusion => {
            let device = reading.the_device_reading.as_ref()?;
            let the_line_stated_plainly = || {
                Some(Words {
                    lead: Prose::new(words::LAST_JOB_RENDERED_ON),
                    values: vec![the_device_line_as_shown(&device.the_lines_words)],
                    rest: None,
                    the_rest_joins: false,
                })
            };
            // **What the job asked for could not be read**, so nothing is
            // concluded about the machine and the line is stated plainly —
            // which is W4's *nothing is said about the machine*, the ask being
            // the condition that is missing. The 38's piece 6.
            let Some(asked_for) = device.asked_for else {
                return if matches!(device.outcome, RecordedOutcome::Finished) {
                    the_line_stated_plainly()
                } else {
                    Some(plainly(words::NONE_OF_THEM_SAID))
                };
            };
            match what_the_device_line_says(device.outcome, asked_for, device.line) {
                WhatW4Concludes::ItRanAsAsked | WhatW4Concludes::NothingIsSaidAboutTheMachine => {
                    the_line_stated_plainly()
                }
                // **W4: the line's absence concludes nothing**, and a device
                // line from a run that did not finish is not evidence — so the
                // card falls to the third line rather than to the first.
                WhatW4Concludes::TheJobsDevicesAreNotKnown => {
                    Some(plainly(words::NONE_OF_THEM_SAID))
                }
                // The fault has its own line and is composed at
                // [`the_setup_fault`].
                WhatW4Concludes::ASetupFault => None,
            }
        }
    }
}

/// **What *Last job rendered on* shows of the line: the device and its
/// route, *NVIDIA GeForce RTX 4090 · OptiX*, where the line is one of the four
/// shapes the plans hold, and the line whole, as printed, where it is any
/// other** — `Design/Server Setup` line 56, changed 2026-10-17 (the Creator's
/// tenth notes, note 3: Cycles' whole line was a mouthful). **The shapes are
/// read by History's own reading**,
/// `crate::surface::the_history::the_device_and_its_route`, the one *Ran on*
/// takes, so the two never read a line differently. **No count of cards is
/// shown**: how the line reads with several is not on record.
fn the_device_line_as_shown(the_lines_words: &str) -> Verbatim {
    match crate::surface::the_history::the_device_and_its_route(the_lines_words) {
        Some((the_device, the_route)) => Verbatim::new(format!(
            "{the_device}{}{the_route}",
            words::BETWEEN_THE_DEVICE_AND_ITS_ROUTE
        )),
        None => Verbatim::new(the_lines_words),
    }
}

/// **`Server Setup` W4: the one state where the action earns being loud.**
///
/// **It is what the job asked for that makes this a fault, not the CPU on its
/// own** — a job submitted with the scene set to the processor and rendered on
/// it is the artist getting exactly what they asked for, and
/// [`what_the_device_line_says`] is where that is decided.
fn the_setup_fault(reading: &ASetUpReading) -> Option<TheSetupFault> {
    let device = reading.the_device_reading.as_ref()?;
    if !matches!(
        what_the_device_line_says(device.outcome, device.asked_for?, device.line),
        WhatW4Concludes::ASetupFault
    ) {
        return None;
    }
    Some(TheSetupFault {
        said: Prose::new(words::THE_LAST_JOB_ASKED_FOR_THE_GPU),
        what_to_do: Prose::new(words::SET_UP_AGAIN_TO_REINSTALL),
    })
}

/// **A moment, with its clock** — §16.1 M3a. **Never an elapsed time.**
fn the_moment(lead: &str, at: Option<std::time::SystemTime>, clock: &str) -> Option<TheMoment> {
    let at = at?;
    let reading = smoozy_state::observed::Reading::taken(
        String::new(),
        at,
        smoozy_state::observed::Clock::AMachine(smoozy_state::observed::MachineClock(
            clock.to_string(),
        )),
    );
    Some(TheMoment {
        lead: Prose::new(lead),
        reading: AsOfThatMoment::from(&reading),
        // **Where no server's name could be read, no clock is named** — the
        // words before a name are not drawn with nothing after them.
        on_the_clock: Prose::new(if clock.is_empty() { "" } else { words::ON_THE_CLOCK }),
    })
}

// ════════════════════════════════════════════════════════════════════════════
// The Blender table
// ════════════════════════════════════════════════════════════════════════════

/// **The table, composed from one reading** — `Server Setup` B10 to B12a.
pub fn the_table_from(reading: &ASetUpReading) -> TheBlenderTable {
    the_table_the_search_found(&reading.the_builds)
}

/// **The same table, drawn from `Server Setup` B10's search alone** — which is
/// what the tab is handed when it opens, the card's own reading saying nothing
/// about what Blenders the disk holds.
pub fn the_table_the_search_found(builds: &[ABuildAsRead]) -> TheBlenderTable {
    TheBlenderTable {
        version: Prose::new(words::VERSION),
        location: Prose::new(words::WHERE),
        rows: builds.iter().map(a_build_row).collect(),
        instead: builds.is_empty().then(|| plainly(words::NO_BLENDER_READING)),
        install: Prose::new(words::INSTALL_BLENDER),
    }
}

fn a_build_row(build: &ABuildAsRead) -> ABuildRow {
    // **B12a: the folder is the directory of the path the search returned,
    // derived when the table is drawn and never remembered.**
    let folder = smoozy_server_setup::search::the_folder_for(&build.path);
    ABuildRow {
        path: Verbatim::new(build.path.clone()),
        version: Verbatim::new(the_version_text(build)),
        folder_bytes: folder.as_bytes().to_vec(),
        folder: Verbatim::new(folder),
        tag: match build.origin {
            None | Some(HowItGotThere::SmoozyInstalledItFromTheList) => None,
            Some(HowItGotThere::TheUserPointedAtIt) => Some(Prose::new(words::YOUR_BUILD)),
            Some(HowItGotThere::NoInstallRecordNamesIt) => {
                Some(Prose::new(words::NOT_INSTALLED_BY_SMOOZY))
            }
        },
        below_the_floor: (build.version < smoozy_server_setup::blender::THE_FLOOR)
            .then(|| Prose::new(words::BELOW_THE_FLOOR)),
        // **Drawn beside the driver**, which this row knows nothing of — see
        // [`the_driver_is_drawn_on`].
        driver_too_old: None,
        remove: Prose::new(words::REMOVE),
    }
}

/// **The version string, which already says the kind** —
/// `Design/Server Setup § Blender`, Creator-stated: there is no kind column
/// because an artist reads long-term support straight off the number, the way
/// Blender writes it everywhere else.
///
/// **It is a value and not prose.** It is what the build answered when B1 asked
/// it, and the typography point must never touch it (X7).
fn the_version_text(build: &ABuildAsRead) -> String {
    let version = format!(
        "{}.{}.{}",
        build.version.major, build.version.minor, build.version.patch
    );
    if build.long_term_support {
        format!("{version} LTS")
    } else {
        version
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Installing — `Server Setup` B2 and B6a
// ════════════════════════════════════════════════════════════════════════════

/// The install box's whole content.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheInstallList {
    pub title: Prose,
    pub source: Prose,
    /// **Finished releases, above the rule.**
    pub finished: Vec<APublishedEntry>,
    /// **The heading under the rule, *Experimental builds*, and the unfinished
    /// builds under it** — two parts rather than a tag on each, because the
    /// question here is which of these is safe to render a night's work with.
    /// **Nothing explains the heading** (`Design/Server Setup § Installing`,
    /// Creator-stated 2026-09-27); until step 10c's ninth-pass fix piece 7 this
    /// was a sentence saying what unfinished builds are.
    pub the_heading: Prose,
    pub unfinished: Vec<APublishedEntry>,
    pub or: Prose,
    pub choose_a_build: Prose,
    pub what_smoozy_checks: Prose,
    pub cancel: Prose,
    pub install: Prose,
    /// **The button's count where several are ticked**, after the figure —
    /// *Install 3 versions*. Added 2026-10-08, the Creator's third notes,
    /// piece H.
    pub versions: Prose,
    /// **Where the list could not be read**, what stands in its place. **The
    /// service's own words are not mined for a cause and are not drawn**: they
    /// are a machine's text and this box has no room for it.
    pub instead: Option<Words>,
    /// **B2: an entry that could not be read is said rather than dropped
    /// quietly.**
    pub short: Option<Words>,
}

/// One published entry — a channel or a listed file, never a build.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct APublishedEntry {
    /// The version. A value.
    pub version: Verbatim,
    /// **What the install carries for it** — the version, and for an
    /// unfinished build from Blender's build service that file's bare name
    /// after one space (`Server Setup` B4, `smoozy_server_setup::blender::TheVersionAsked`).
    /// A value the page hands back unaltered. Added 2026-10-02.
    pub asked: Verbatim,
    /// **The line it belongs to**, named as `Design/Server Setup § Installing`
    /// names it.
    pub line: Prose,
    /// **B6a's mark**, which says a version is present and can never say *this
    /// build*. Absent where nothing has been read from the machine.
    pub mark: Option<Words>,
}

/// **B2 and B6a, composed.**
pub fn the_list(
    read: Result<smoozy_server_setup::listing::TheListingAsRead, WhyTheListingCouldNotBeRead>,
    answered_on_the_machine: Vec<Version>,
) -> TheInstallList {
    let (finished, unfinished, short, instead) = match read {
        Err(_) => (
            Vec::new(),
            Vec::new(),
            None,
            Some(plainly(words::THE_LIST_COULD_NOT_BE_READ)),
        ),
        Ok(listing) => {
            // **One version is one row in each part, naming every line that
            // publishes it** — `Design/Server Setup § Installing`, added
            // 2026-09-27, fix piece 10. The shop-front publishes 5.2.2 on
            // `latest` and on `5.2lts`, as stable and as candidate on both, and
            // one row per line drew one build twice in each part. **No line is
            // dropped here**: each the offered list holds is named, and a line
            // named twice is named once. **The one kind B2 leaves out — an
            // unfinished build of a released series — is left out by
            // `Server Setup` B2's `the_offered_list` before this, and never
            // here.** The rows keep the offered list's own order, which is
            // `Server Setup` B2's sort on version components.
            let latest = smoozy_server_setup::blender::the_latest_finished_release(&listing.channels)
                .map(|entry| entry.version);
            let mut finished: Vec<(&PublishedChannel, Vec<&'static str>)> = Vec::new();
            let mut unfinished: Vec<(&PublishedChannel, Vec<&'static str>)> = Vec::new();
            for entry in &listing.channels {
                // **The shop-front's own unfinished channels are never offered**
                // — `Server Setup` B2's revision of 2026-10-02: none ever had a
                // file Smoozy could fetch (`Server Setup` B5).
                if entry.risk == Risk::Edge {
                    continue;
                }
                let part = if entry.risk.is_unstable() {
                    &mut unfinished
                } else {
                    &mut finished
                };
                let lines_for_entry = the_lines_name(entry, latest);
                // **One row per version, and per file where the build service
                // lists one** — two kinds of one version are two files.
                match part.iter_mut().find(|(first, _)| {
                    first.version == entry.version
                        && first.the_build_services_file == entry.the_build_services_file
                }) {
                    Some((_, lines)) => {
                        for line in lines_for_entry {
                            if !lines.contains(&line) {
                                lines.push(line);
                            }
                        }
                    }
                    None => part.push((entry, lines_for_entry)),
                }
            }
            let drawn = |part: Vec<(&PublishedChannel, Vec<&'static str>)>| -> Vec<APublishedEntry> {
                part.into_iter()
                    .map(|(first, lines)| a_published_entry(first, &lines, &answered_on_the_machine))
                    .collect()
            };
            let finished = drawn(finished);
            let unfinished = drawn(unfinished);
            let short = (!listing.entries_that_could_not_be_read.is_empty()).then(|| Words {
                lead: Prose::new(""),
                values: vec![Verbatim::new(
                    listing.entries_that_could_not_be_read.len().to_string(),
                )],
                rest: Some(Prose::new(words::ENTRIES_UNREAD)),
                the_rest_joins: false,
            });
            (finished, unfinished, short, None)
        }
    };
    TheInstallList {
        title: Prose::new(words::INSTALL_BLENDER_TITLE),
        source: Prose::new(words::FROM_BLENDERS_OWN),
        finished,
        the_heading: Prose::new(words::EXPERIMENTAL_BUILDS),
        unfinished,
        or: Prose::new(words::OR_CHOOSE_A_BUILD),
        choose_a_build: Prose::new(words::CHOOSE_A_BUILD),
        what_smoozy_checks: Prose::new(words::WHAT_SMOOZY_CHECKS),
        cancel: Prose::new(words::CANCEL),
        install: Prose::new(words::INSTALL),
        versions: Prose::new(words::VERSIONS),
        instead,
        short,
    }
}

/// **One row: a version, every line that publishes it, and B6a's mark** — the
/// mark read off the row's first entry, which carries the same version as every
/// other entry of the row. The lines are prose joined with prose, and the
/// version stands apart from them as a value (X7).
fn a_published_entry(
    first: &PublishedChannel,
    lines: &[&'static str],
    answered: &[Version],
) -> APublishedEntry {
    APublishedEntry {
        version: Verbatim::new(format!(
            "{}.{}.{}",
            first.version.major, first.version.minor, first.version.patch
        )),
        asked: Verbatim::new(
            smoozy_server_setup::blender::TheVersionAsked {
                version: first.version,
                the_build_services_file: first.the_build_services_file.clone(),
            }
            .as_carried(),
        ),
        line: Prose::new(lines.join(" \u{b7} ")),
        mark: (!answered.is_empty()).then(|| the_mark(first, answered)),
    }
}

/// **`Design/Server Setup § Installing`: Latest stable names `Server Setup` B3's chosen
/// version, retaining its long-term-support line where present. Other finished
/// rows name their line, and unfinished entries name their own kind.**
///
/// **The long-term-support lines are marked in the track's own name** —
/// `Server Setup` B2, observed 2026-08-19 — so that is where this reads it,
/// and never off a version number.
///
/// **An unfinished entry names its kind in Blender's own word** — *Alpha*,
/// *Beta*, *Release candidate* — the build service's `risk_id` (the Creator's
/// note 5 on 0.1.7). *(The third kind read *Unstable* until 2026-10-02, when it
/// came from the shop-front.)* **The shop-front's `edge` is never offered**
/// ([`the_list`] leaves it out), so its word is only the heading's.
fn the_lines_name(entry: &PublishedChannel, latest: Option<Version>) -> Vec<&'static str> {
    match entry.risk {
        Risk::Candidate => vec![words::RELEASE_CANDIDATE],
        Risk::Beta => vec![words::BETA],
        Risk::Alpha => vec![words::ALPHA],
        Risk::Edge => vec![words::EXPERIMENTAL_BUILDS],
        Risk::Stable => {
            // The nineteenth notes, note 1: the label follows `Server Setup` B3's choice,
            // including a newest release published only on an LTS track.
            let mut lines = Vec::new();
            if Some(entry.version) == latest {
                lines.push(words::LATEST_STABLE);
            }
            if entry.track.contains("lts") {
                lines.push(words::LONG_TERM_SUPPORT);
            } else if lines.is_empty() {
                lines.push(words::RELEASE);
            }
            lines
        }
    }
}

/// **B6a's mark: a version comparison and nothing more.**
fn the_mark(entry: &PublishedChannel, answered: &[Version]) -> Words {
    match smoozy_server_setup::blender::the_mark_for(entry, answered) {
        TheMark::ThisVersionIsPresent => plainly(words::ON_THIS_SERVER),
        TheMark::AnotherVersionOfThisLineIsPresent(version) => Words {
            lead: Prose::new(""),
            values: vec![Verbatim::new(format!(
                "{}.{}.{}",
                version.major, version.minor, version.patch
            ))],
            rest: Some(Prose::new(words::INSTALLED)),
            the_rest_joins: false,
        },
        TheMark::Unmarked => plainly(words::NOT_INSTALLED),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Removing a Blender — `Server Setup` B13 to B15
// ════════════════════════════════════════════════════════════════════════════

/// What removing a build costs.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheRemovalWarning {
    /// *Remove Blender 5.1.0?* — the version is a value and the question mark
    /// is part of the sentence.
    pub title: Words,
    /// **B14: the live jobs, each told the truth about itself.** Empty where
    /// none is.
    pub the_live_jobs: Vec<ALiveJobLine>,
    /// The lead over the list, where there is one.
    pub how_many: Option<Words>,
    /// **B15: the version-numbered settings folder is left behind in every
    /// case.**
    pub what_is_left: Prose,
    pub cancel: Prose,
    /// **Absent where this module has no carriage for the removal** — see
    /// [`TheRemovalWarning::smoozy_cannot_perform_it`]. A control that is
    /// pressed and then refused is a control that should not have been
    /// pressable.
    pub remove: Option<Prose>,
    /// **`Server Setup` §13 item 21: a managed Blender under a package name
    /// outside the fixed set has no admissible removal carriage**, and the
    /// entry's own answer is that *"the removal is reported as one this module
    /// cannot perform, and the user's own manager is the route"*. **The words
    /// are Design's and are owed** (§24); what is fixed here is that the box
    /// says so rather than offering a control that would refuse.
    pub smoozy_cannot_perform_it: Option<Prose>,
    /// The path the act hands back. A value.
    pub path: Verbatim,
}

/// One live job the removal would touch.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ALiveJobLine {
    /// The job's own name. A value — X7 forbids the point touching it.
    pub job: Verbatim,
    /// What is true of that job.
    pub said: Prose,
}

/// **B13 and B14, composed.**
pub fn the_removal_warning(
    path: &str,
    version: Option<Version>,
    live: &[(String, ALiveJob)],
    the_managers_package: Option<&str>,
) -> TheRemovalWarning {
    // **B15's two routes, decided by the one reading that carries the manager's
    // package.** *"A path under the system package areas is the manager's."*
    let route = match the_managers_package {
        Some(_) => RemovalRoute::TheMachinesOwnSoftwareManager,
        None => RemovalRoute::TheFolderDeletion,
    };
    // **§13 item 21, performed by the module's own answer** — the fixed set is
    // that crate's and is not restated here.
    let it_can_be_performed = match the_managers_package {
        Some(package) => {
            smoozy_server_setup::search::this_module_can_remove_the_managed_build(package)
        }
        None => true,
    };
    // **B14: the warning names every live job whose recorded build path is the
    // path of the build being removed, or sits under it where the removal is a
    // folder deletion** — the module's own predicate, not a second one here.
    let named = smoozy_server_setup::search::the_warning_names(
        &live.iter().map(|(_, job)| job.clone()).collect::<Vec<_>>(),
        path,
        route,
    );
    let lines: Vec<ALiveJobLine> = live
        .iter()
        .filter(|(_, job)| named.contains(job))
        .map(|(name, job)| ALiveJobLine {
            job: Verbatim::new(name.clone()),
            said: Prose::new(match what_the_warning_says(job, route) {
                WhatTheWarningSays::ARenderingJobWillFinish => words::RENDERING_WILL_FINISH,
                WhatTheWarningSays::ARenderingJobsFateIsNotKnownOnThisRoute => {
                    words::RENDERING_UNKNOWN
                }
                WhatTheWarningSays::AWaitingJobWillHaveNothingToRenderWith => {
                    words::WAITING_NOTHING_TO_RENDER
                }
            }),
        })
        .collect();

    TheRemovalWarning {
        title: Words {
            lead: Prose::new(words::REMOVE_BLENDER),
            values: vec![Verbatim::new(match version {
                Some(version) => format!("{}.{}.{}", version.major, version.minor, version.patch),
                None => path.to_string(),
            })],
            rest: Some(Prose::new("?")),
            the_rest_joins: true,
        },
        how_many: match lines.len() {
            0 => None,
            1 => Some(plainly(words::ONE_JOB_WAS_SUBMITTED)),
            many => Some(Words {
                lead: Prose::new(""),
                values: vec![Verbatim::new(many.to_string())],
                rest: Some(Prose::new(words::JOBS_WERE_SUBMITTED)),
                the_rest_joins: false,
            }),
        },
        the_live_jobs: lines,
        what_is_left: Prose::new(words::SETTINGS_FOLDER_STAYS),
        cancel: Prose::new(words::CANCEL),
        remove: it_can_be_performed.then(|| Prose::new(words::REMOVE_ANYWAY)),
        smoozy_cannot_perform_it: (!it_can_be_performed)
            .then(|| Prose::new(words::THE_MANAGER_IS_THE_ROUTE)),
        path: Verbatim::new(path),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Admin access — `Server Setup` E2, and `Security and licence` S36
// ════════════════════════════════════════════════════════════════════════════

/// The admin-access box.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheAdminAccessBox {
    pub title: Prose,
    /// **The line that names the step that was refused**, which is the only
    /// thing that changes between callers.
    pub which_step: Words,
    /// **The reading where the system would not hand the password back.** It
    /// asks for nothing: it reports, keeps the password remembered, and offers
    /// the step again.
    pub the_password_could_not_be_read: Option<Prose>,
    /// The field's label, absent on that reading.
    pub password: Option<Prose>,
    /// **`Security and licence` S36: two choices or three**, composed from the
    /// store's standing and never fixed here.
    pub choices: Vec<AChoice>,
    /// **Where the standing could not be read**, the box says so — and offers
    /// the two choices a store never proven is offered.
    pub the_standing_could_not_be_read: Option<Prose>,
    /// **`Security and licence` S34's one sentence**, where this box is the
    /// act that found there is nowhere to keep a password — once for the life
    /// of the installation.
    pub nowhere_to_keep_them: Option<Prose>,
    pub cancel: Prose,
    pub try_again: Prose,
}

/// One of the three lifetimes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AChoice {
    /// Which choice this is, as a value the page hands back.
    pub which: Verbatim,
    pub name: Prose,
    pub said: Prose,
    /// **`Whole app`'s own default** — the middle one.
    pub chosen: bool,
}

/// **E2 and S36, composed.**
pub fn the_box(
    the_step_that_was_refused: &str,
    standing: Option<StoreStanding>,
    the_system_would_not_hand_the_password_back: bool,
) -> TheAdminAccessBox {
    // **`Security and licence` S25's cautious reading for a standing nobody could read**: a store not
    // proven here is offered what *never answered* is offered — no promise to
    // keep the password across restarts.
    let offered = standing.unwrap_or(StoreStanding::NeverAnswered);
    TheAdminAccessBox {
        title: Prose::new(words::NEEDS_ADMIN_ACCESS),
        which_step: Words {
            lead: Prose::new(""),
            values: vec![Verbatim::new(the_step_that_was_refused)],
            rest: Some(Prose::new(words::WAS_REFUSED)),
            the_rest_joins: false,
        },
        the_password_could_not_be_read: the_system_would_not_hand_the_password_back
            .then(|| Prose::new(words::THE_SYSTEM_WOULD_NOT_HAND_IT_BACK)),
        password: (!the_system_would_not_hand_the_password_back)
            .then(|| Prose::new(words::PASSWORD)),
        choices: if the_system_would_not_hand_the_password_back {
            Vec::new()
        } else {
            offered
                .machine_password_choices()
                .iter()
                .map(|choice| a_choice(*choice))
                .collect()
        },
        the_standing_could_not_be_read: (standing.is_none() && !the_system_would_not_hand_the_password_back)
            .then(|| Prose::new(words::THE_STANDING_COULD_NOT_BE_READ)),
        nowhere_to_keep_them: None,
        cancel: Prose::new(words::CANCEL),
        try_again: Prose::new(words::TRY_AGAIN),
    }
}

fn a_choice(choice: MachinePasswordChoice) -> AChoice {
    match choice {
        MachinePasswordChoice::DoNotRememberIt => AChoice {
            which: Verbatim::new("do-not-remember-it"),
            name: Prose::new(words::DO_NOT_REMEMBER_IT),
            said: Prose::new(words::DO_NOT_REMEMBER_IT_SUB),
            chosen: false,
        },
        MachinePasswordChoice::WhileThisConnectionLasts => AChoice {
            which: Verbatim::new("while-connected"),
            name: Prose::new(words::WHILE_CONNECTED),
            said: Prose::new(words::WHILE_CONNECTED_SUB),
            chosen: true,
        },
        MachinePasswordChoice::RememberIt => AChoice {
            which: Verbatim::new("remember-it"),
            name: Prose::new(words::REMEMBER_IT),
            said: Prose::new(words::REMEMBER_IT_SUB),
            chosen: false,
        },
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The run panel — `Design/Server Setup § A long step`, `§ Cut off part-way`,
// `§ The step that failed`
// ════════════════════════════════════════════════════════════════════════════

/// **A run shown as its steps, drawn on the card in the card's place and never
/// in a box** — `Design/Server Setup § A long step` and `§ Cut off part-way`,
/// the drawing's `sugoing`, `burun`, `bufail` and `bustep`. *(A box over the
/// server's screen, tabs included, until 2026-10-02 — the Creator's note 4 on
/// 0.1.7.)* Its name stays *panel* because it is the same composition.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheRunPanel {
    /// The card's section heading, so the card can be drawn before the tab's
    /// own reading lands.
    pub this_server: Prose,
    /// **Which of the drawing's dots** — `busy` on every run card, as the
    /// drawing draws all four.
    pub dot: Verbatim,
    /// The card's state line — *Setting up*, *Installing Blender*, *Abandoning
    /// setup*, or *Setup stopped part-way*.
    pub title: Prose,
    /// **The state line carries the mark itself**, where no step does.
    pub the_state_is_underway: bool,
    /// *started …*, on a run still going.
    pub when_it_started: Option<TheMoment>,
    /// The moment after the state line, where the run ended.
    pub how_it_ended: Option<TheMoment>,
    pub steps: Vec<AStepLine>,
    /// **`Server Setup` T6a: what the card may say about the render.**
    pub about_the_render: Option<Prose>,
    /// **What *Cancel setup* leaves** — `Design/Server Setup § A long step`:
    /// what is already installed stays.
    pub what_cancel_leaves: Option<Prose>,
    /// **The closing paragraph** — `Design/Server Setup § The two ways out`,
    /// which writes it out in full and says it is the same paragraph on both
    /// stopped cards. Absent while the run is still going.
    pub the_closing_paragraph: Option<TheClosingParagraph>,
    /// ***Cancel setup*, the one control while the run goes** — `Server Setup`
    /// Y8's stop.
    pub cancel: Option<Prose>,
    /// **The user has pressed it**, so it is drawn under way until the run
    /// reaches its boundary and ends.
    pub the_cancel_was_asked: bool,
    pub abandon: Option<Prose>,
    pub set_up_again: Option<Prose>,
    /// **`Server Setup` B7a's question, while the run waits on it** — drawn on
    /// the card as `su-low` draws it. Added 2026-10-09, the 38's piece 14b.
    pub the_question: Option<TheBelowTheFloorWarning>,
    /// **The newest complete lines of the ordinary stream of the step under
    /// way**, oldest first — the window to the right of the steps
    /// (`Design/Server Setup § A long step`; `Desktop app and delivery` M13b's
    /// second narrowing). Values,
    /// each the machine's own line. **Empty before anything is printed, once
    /// the run ends, and on a card with a failed step**, and then there is no
    /// window. Added 2026-10-18.
    ///
    /// **From 2026-10-25 they are the newest lines of either stream**, in the
    /// order they arrived, the last of them perhaps a line not yet ended and
    /// each rewritten line in its newest form (`Desktop app and delivery`
    /// M13b's second narrowing as changed on 2026-10-24; the eighteenth notes,
    /// piece 2c).
    pub the_servers_newest_lines: Vec<Verbatim>,
    /// **Which of [`Self::the_servers_newest_lines`] came from the error
    /// stream**, one mark per line, so the page draws those in the brighter
    /// ink and the two streams are never read as one (M13b: *"each line drawn
    /// so that which stream it came from is never lost"*). Added 2026-10-25.
    pub which_lines_are_from_the_error_stream: Vec<bool>,
    /// ***Server output***, under the window, or alone where there is none.
    pub server_output: Option<Prose>,
    /// **The last act's line is said on these steps, so the page draws none
    /// above them** — a failed Blender install's stopped setup, whose failed
    /// step carries the install's own sentence
    /// ([`the_installs_failure_is_said_on_its_step`]). The application decides;
    /// the page only leaves the line out. Added 2026-10-25.
    pub the_last_act_is_said_on_its_steps: bool,
    /// **The run has ended, and these are its steps as its last reading left
    /// them** — for the card to keep while its own reading is taken, and never
    /// to draw as a run still going. Added 2026-10-26, the worldwide pass's
    /// setup faults, fault 4.
    pub the_run_has_ended: bool,
    /// **The acts waiting their turn behind this run**, each named as the act
    /// it is, in the order asked, under the heading — `Design/Server Setup §
    /// Asking for more while a setup runs` (`Server Setup` Y9a). Empty, and no
    /// heading, where nothing waits or the run has ended. Added 2026-10-27,
    /// the nineteenth notes, note 2.
    pub the_waiting_heading: Option<Prose>,
    pub the_waiting: Vec<Prose>,
    /// **The card's own act offered quietly while the run goes**, joining the
    /// line when pressed: *Install Telegram reporting* where the card last
    /// offered it. Added 2026-10-27. *Set up again* is not offered while a
    /// run goes (`Server Setup` Y9a item 6; the twenty-fourth notes, note 1).
    pub install_telegram_while_it_runs: Option<Prose>,
}

/// The closing paragraph, in three parts because the control's own name is
/// written inside it exactly as it reads on the control.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheClosingParagraph {
    pub before: Prose,
    pub the_controls_name: Prose,
    pub after: Prose,
}

/// One step of the run.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AStepLine {
    /// **Which boundary the line is**, for the composition here; the page is
    /// never handed it.
    #[serde(skip)]
    pub boundary: Boundary,
    pub name: Prose,
    /// **`Server Setup` S1's three kinds** — `done`, `bad`, `skip` — plus the
    /// one under way, `now`. A value the page marks with and never derives.
    pub mark: Verbatim,
    /// The small words at the end of the line.
    pub said: Option<Words>,
    /// **The colour of those words** — one of [`the_marks_colour`], plain for
    /// a step under way or not started. A value the page marks with and never
    /// derives.
    pub colour: Verbatim,
    /// **The note under the restarting step** — `Design/Server Setup § A long
    /// step`: a restart already begun cannot be called back.
    pub note: Option<Prose>,
    /// **The one failure shape, on the step itself** — `Design/Server Setup §
    /// The step that failed`. **Composed by `web/the-failure-shape.js` and
    /// never rebuilt here**; what the application supplies is the sentence and
    /// the subject the machine's own words are read from.
    pub the_failure: Option<TheFailedStep>,
    /// **How far the download or the unpack has got, in the step's small
    /// note** — `Design/Server Setup § A long step`, `Server Setup` Y7b and
    /// `Server Setup` Y7c: *214 of 352 MB*, *214 MB*, *41%*. Smoozy's own figure, never in
    /// the server's window. Nothing on every other step and wherever the
    /// reading carries no count. Added 2026-10-18, the Creator's fourteenth
    /// notes, piece U.
    pub the_figure: Option<AStepsFigure>,
}

/// **A step's figure**: what it has got to, and, where the whole is known and
/// the figure is not a share of it, *of* and the whole. **The figures are
/// values and *of* is prose**, so the page sets them side by side and joins
/// nothing (X7).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AStepsFigure {
    pub so_far: Verbatim,
    pub of: Option<Prose>,
    pub whole: Option<Verbatim>,
}

/// **The step that failed** — one plain sentence, and the machine's own words
/// behind the control that names them.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheFailedStep {
    /// **One plain sentence saying what that step did not do.**
    pub said: Words,
    /// **The one control the whole app uses for raw text**, named as the
    /// drawing names it — **and only where there are words to open behind
    /// it.** *(Until 2026-10-18 it was on every failed step, Smoozy's own
    /// sentence behind it where a Telegram step stopped on this computer: the
    /// Creator's twelfth notes, note 2.)*
    pub what_the_server_said: Option<Prose>,
    /// **The machine's own words, whole and unedited** — S2a. Two streams,
    /// kept apart. Values: they are the machine's text and X7 forbids the point
    /// touching them.
    pub the_output: Option<Verbatim>,
    pub the_complaint: Option<Verbatim>,
    /// **Each stream's tab is named for its stream**, in the words the shared
    /// two-stream panel already uses (`Desktop app and delivery` M13a, piece D)
    /// — must-do 33. *(Until the step 10a round 1 audit of these pieces both tabs
    /// read *What the server said*.)*
    pub the_output_tab: Prose,
    pub the_complaint_tab: Prose,
    /// **The panel's own control, *Close*** — `Design/Server Setup § The step
    /// that failed`: *"The panel's Close only closes the panel"*. Added
    /// 2026-09-21: the second pass on a real rental found the control drawn as
    /// an empty pill on every panel this tab opened (fault 6), because nothing
    /// handed the page its word.
    pub close: Prose,
}

/// Only Smoozy's recorded failure of this build send has this opening
/// sentence. Its details remain Smoozy's on a later reading of the run.
fn the_recorded_build_send_failure(record: &TheSetupRecord) -> Option<&str> {
    let Some(HowTheRunEnded::AStepFailed {
        step: smoozy_server_setup::ready_action::Step::Readings,
        words: TheFailedStepsWords::SmoozysOwnVerification { what_was_read },
    }) = &record.ending else { return None };
    // The opening sentence before the step 13 words sweep (2026-10-28), so a
    // failure recorded on a server before then still reads as this one.
    const THE_OPENING_BEFORE_THE_SWEEP: &str = "Your build could not be sent to the server.";
    (record.kind == SetupKind::ABlenderInstall
        && [words::BUILD_COULD_NOT_BE_SENT, THE_OPENING_BEFORE_THE_SWEEP]
            .iter()
            .any(|opening| what_was_read.starts_with(&format!("{opening} "))))
        .then_some(what_was_read.as_str())
}

/// **The run panel, composed from one run** — `Server Setup` S1, S2, S2a and
/// T6a.
pub fn the_run_panel(
    record: &TheSetupRecord,
    the_step_in_flight: Option<Boundary>,
    the_clock: &str,
    still_going: bool,
    the_library_half_was_read: bool,
) -> TheRunPanel {
    // **One step stopped part-way and never two, and the module's own answer
    // can name two.** `Server Setup` Y5 gives the driver step **three**
    // boundaries — placed, restarted, answering — so
    // [`smoozy_server_setup::recovery::how_far_it_got`], which matches a
    // boundary's *step* against the step in flight, marks every boundary of
    // that step as stopped-part-way. **That is right about the step and wrong
    // about the list**: a run died in one place, and drawing the failure shape
    // twice would say it died in two. **The first is the one it died in**,
    // because the boundaries are in the run's own order and every one before it
    // was reached.
    let mut how_far = record.how_far_it_got(the_step_in_flight.map(|it| it.its_step()));
    if the_recorded_build_send_failure(record).is_some() {
        for (boundary, got) in &mut how_far {
            *got = if *boundary == Boundary::Started {
                WhereAStepGot::StoppedPartWay
            } else {
                WhereAStepGot::NeverStarted
            };
        }
    }
    let mut already = false;
    for (_, got) in how_far.iter_mut() {
        if *got != WhereAStepGot::StoppedPartWay {
            continue;
        }
        if already {
            *got = WhereAStepGot::NeverStarted;
        }
        already = true;
    }
    let steps = how_far
        .iter()
        .filter_map(|(boundary, got)| {
            a_step_line(
                record,
                *boundary,
                *got,
                the_step_in_flight,
                still_going,
                the_library_half_was_read,
            )
        })
        .collect();

    let ended = !still_going;
    TheRunPanel {
        this_server: Prose::new(words::THIS_SERVER),
        dot: Verbatim::new("busy"),
        title: Prose::new(if still_going {
            words::SETTING_UP
        } else {
            words::SETUP_STOPPED_PART_WAY
        }),
        // **Where a step is under way, the step carries the mark**; a run still
        // going whose reading marks none says so on its state line.
        the_state_is_underway: still_going && the_step_in_flight.is_none(),
        // **`Design/Server Setup` writes *started 2 minutes ago*, and this says
        // the moment instead**, for the reason the running card does (§16.1
        // M3a).
        when_it_started: still_going
            .then(|| {
                the_moment(
                    words::STARTED,
                    record.the_boundaries_reached.first().map(|it| it.at),
                    the_clock,
                )
            })
            .flatten(),
        how_it_ended: ended
            .then(|| {
                the_moment(
                    match record.ending {
                        Some(HowTheRunEnded::AStepFailed { .. }) => words::A_STEP_DID_NOT_FINISH_AT,
                        Some(HowTheRunEnded::LeftStopped { .. }) => words::THE_RUN_WAS_STOPPED_AT,
                        _ => words::THE_CONNECTION_DROPPED_AT,
                    },
                    record.the_last_boundary_reached().map(|it| it.at),
                    the_clock,
                )
            })
            .flatten(),
        steps,
        // **Nothing about a render is known from the record alone**, so the
        // record's panel says nothing about one — the running panel adds the
        // sentence where it is true ([`the_render_sentence`]). *(Until
        // 2026-09-27 this drew it on every running panel whose list carried a
        // restart, from the driver step on and before any restart — step 10c's
        // eighth pass on Hyperstack, fault 5, on a machine that had never
        // rendered.)*
        about_the_render: None,
        what_cancel_leaves: still_going.then(|| Prose::new(words::CANCEL_SETUP_LEAVES)),
        the_closing_paragraph: ended.then(|| TheClosingParagraph {
            before: Prose::new(words::THE_CLOSING_PARAGRAPH_ONE),
            the_controls_name: Prose::new(words::ABANDON_SETUP),
            after: Prose::new(words::THE_CLOSING_PARAGRAPH_TWO),
        }),
        // ***Cancel setup* is offered throughout** — `Design/Server Setup § A
        // long step`. **There is no *Close***: there is no box to close.
        // *(Struck 2026-10-02 with the box: *Close* sat beside *Stop* from
        // 2026-09-27, fix piece 9.)*
        cancel: still_going.then(|| Prose::new(words::CANCEL_SETUP)),
        the_cancel_was_asked: false,
        abandon: ended.then(|| Prose::new(words::ABANDON_SETUP)),
        set_up_again: ended.then(|| Prose::new(words::SET_UP_AGAIN)),
        the_question: None,
        the_servers_newest_lines: Vec::new(),
        which_lines_are_from_the_error_stream: Vec::new(),
        server_output: None,
        the_last_act_is_said_on_its_steps: false,
        the_run_has_ended: false,
        the_waiting_heading: None,
        the_waiting: Vec::new(),
        install_telegram_while_it_runs: None,
    }
}

fn a_step_line(
    record: &TheSetupRecord,
    boundary: Boundary,
    got: WhereAStepGot,
    the_step_in_flight: Option<Boundary>,
    still_going: bool,
    the_library_half_was_read: bool,
) -> Option<AStepLine> {
    // **The run's own last step is a line of this list.** Added 2026-09-23,
    // step 10c's fifth pass on Vast: a Blender install whose final check failed
    // drew five steps all marked done under the heading *Setup stopped
    // part-way*, and nothing at all for the step that failed — **the panel said
    // a stopped run had finished everything.** `the_boundarys_name` answers
    // `None` for it because the card's list of what was done to the machine
    // owes no line to a check, and this list is the other question.
    let name = match boundary {
        Boundary::Started if the_recorded_build_send_failure(record).is_some() => words::UPLOAD_YOUR_BUILD,
        Boundary::Done => words::DONE_STEP,
        other => the_boundarys_name(other)?,
    };
    let under_way = still_going && the_step_in_flight == Some(boundary);
    let restarting = under_way && boundary == Boundary::RestartIssued;
    let recorded = record
        .the_boundaries_reached
        .iter()
        .find(|it| it.boundary == boundary);

    // **A run still going draws a step it has not reached with no mark at
    // all**, which is the drawing's own `su-run`: *not started* is what a run
    // that stopped says of a step it never reached, and said of a step still
    // to come it would read as though the run had given up on it. Added
    // 2026-09-27, fix piece 9.
    let still_to_come = still_going && got == WhereAStepGot::NeverStarted;
    let mark = if under_way {
        "now"
    } else if still_to_come {
        ""
    } else {
        match got {
            WhereAStepGot::Done => "done",
            WhereAStepGot::StoppedPartWay => "bad",
            WhereAStepGot::NeverStarted => "skip",
        }
    };

    // **The words at the end of the line, and their colour.** A step under way
    // or not started stays plain; the step where the run stopped is red, as
    // the stopped card draws it, whichever way it stopped — its whole row is
    // already red.
    let (said, colour) = match (under_way, got) {
        (true, _) if restarting => (Some(words::WAITING_FOR_THE_RESTART), the_marks_colour::PLAIN),
        (true, _) => (None, the_marks_colour::PLAIN),
        (_, _) if still_to_come => (None, the_marks_colour::PLAIN),
        (_, WhereAStepGot::Done) => {
            let (said, colour) = match recorded {
                Some(it) => the_marks_words(it, the_library_half_was_read),
                None => (words::DONE, the_marks_colour::DONE),
            };
            (Some(said), colour)
        }
        (_, WhereAStepGot::StoppedPartWay) => (
            Some(match record.ending {
                Some(HowTheRunEnded::AStepFailed { .. }) => words::DID_NOT_FINISH,
                Some(HowTheRunEnded::LeftStopped { .. }) => words::STOPPED_HERE,
                _ => words::THE_CONNECTION_DROPPED_HERE,
            }),
            the_marks_colour::FAILED,
        ),
        (_, WhereAStepGot::NeverStarted) => (Some(words::NOT_STARTED), the_marks_colour::PLAIN),
    };

    Some(AStepLine {
        boundary,
        name: Prose::new(if restarting { words::RESTARTING } else { name }),
        mark: Verbatim::new(mark),
        said: said.map(plainly),
        colour: Verbatim::new(colour),
        note: restarting.then(|| Prose::new(words::THE_RESTART_HAS_BEGUN)),
        // **The one failure shape, on the step that failed and nowhere else.**
        //
        // **A run the connection cut carries none**, and that is the difference
        // between the drawing's two panels: nothing failed there, the
        // connection went, and the step it went during is marked as where it
        // stopped rather than as a failure.
        the_failure: match (&record.ending, got) {
            (Some(HowTheRunEnded::AStepFailed { words: said, .. }), WhereAStepGot::StoppedPartWay) => {
                // **The sentence says what the record says.** Where the record's
                // words are Smoozy's own record that it did not carry a recovery
                // on (`Server Setup` S2a's third bullet), the step's own failure
                // sentence would claim something that did not happen — step 10c's
                // seventh pass on Hyperstack read *The graphics driver did not
                // answer after the restart.* with the card answering the whole
                // time (fault 4). **Smoozy's own constant is matched, and
                // nothing of a machine's words is read.** Added 2026-09-27, fix
                // piece 6.
                let not_carried_on = matches!(
                    said,
                    TheFailedStepsWords::SmoozysOwnVerification { what_was_read }
                        if crate::setting_up::the_recovery_was_not_carried_on(what_was_read)
                );
                // **A Telegram step that stopped on this computer says Smoozy's
                // own sentence, and nothing is claimed as the server's.** The
                // material is composed here, and a composing that fails is
                // written onto the machine as the step's words (S2a), so the
                // step's own sentence — *The server could not install Telegram
                // reporting.* — would blame a server that did nothing, with
                // Smoozy's sentence behind *What the server said*. **The one
                // closed list of composing sentences is matched, whole**, and
                // the list's own sentence is what is drawn; a machine's words
                // are never read for a cause. Added 2026-10-18, the Creator's
                // twelfth notes, note 2.
                let composing = match (boundary, said) {
                    (Boundary::Telegram, TheFailedStepsWords::SmoozysOwnVerification { what_was_read }) => {
                        crate::record::told_record::the_composing_sentences::the_one_said(what_was_read)
                    }
                    _ => None,
                };
                // A failed build send is recorded before an install step
                // starts. Keep its own sentence and origin on later readings.
                let build_send = (boundary == Boundary::Started)
                    .then(|| the_recorded_build_send_failure(record)).flatten();
                let own_sentence = composing.or(build_send);
                let sentence = match own_sentence {
                    Some(own) => Some(own),
                    None if not_carried_on => Some(words::FAILED_NOT_CARRIED_ON),
                    None => the_boundarys_failure_sentence(boundary),
                };
                sentence.map(|sentence| {
                    // **S2a: the machine's own words are part of the run's
                    // recorded ending, read back with the boundary list** —
                    // **verbatim, whole or not at all, the two streams kept
                    // apart**, and never mined for a cause. **Smoozy's own
                    // sentence is none of them.**
                    let (the_output, the_complaint) = match own_sentence {
                        Some(_) => (None, None),
                        None => (
                            the_stream(said, WhichStream::Output),
                            the_stream(said, WhichStream::Complaint),
                        ),
                    };
                    TheFailedStep {
                        said: plainly(sentence),
                        what_the_server_said: (the_output.is_some() || the_complaint.is_some())
                            .then(|| Prose::new(words::WHAT_THE_SERVER_SAID)),
                        the_output,
                        the_complaint,
                        the_output_tab: Prose::new(Stream::TheOrdinaryOutput.the_name_of_the_stream()),
                        the_complaint_tab: Prose::new(Stream::TheComplaints.the_name_of_the_stream()),
                        close: Prose::new(words::CLOSE),
                    }
                })
            }
            _ => None,
        },
        // **Drawn only on a run still going**, from its own reading
        // ([`the_builds_steps_are_named_and_counted`]).
        the_figure: None,
    })
}

/// **`Design/Server Setup § A long step`, the download and the unpack: the
/// download's step named for the build, and the step under way saying how far
/// it has got** — `Server Setup` Y7b and `Server Setup` Y7c. Added 2026-10-18, the Creator's
/// fourteenth notes, piece U.
///
/// - **The download's step is *Download Blender* and the build**, as the list
///   named it, under way, done and not started alike, wherever this copy knows
///   the build the run downloads; *Download Blender* alone where it does not.
///   **While it runs, its note is the figure**: *214 of 352 MB*, or *214 MB*
///   where the download announced no size.
/// - **The unpack is not a step of its own in the run's record**: it runs
///   inside the tooling step, whose one boundary (`Server Setup` Y4's
///   *tooling*) it feeds, after the unpack tool's install where that is
///   needed. **So the leanest reading, ruled here, is that the tooling step is
///   named *Unpack Blender* and the build while the unpack is counting, with
///   *41%* in its note**, and *Install the unpack tool* in every other state —
///   before the unpack, while the tool installs, and once the step is done or
///   stopped. A step of its own would want a boundary the record does not
///   have; the drawing's `su-fetch` still lists *Install the unpack tool* as
///   to come, and it is.
/// - **Nothing where the reading carries no count**, so a machine program
///   older than the count shows the name alone. **Never in the server's
///   window**, which holds only what the server printed.
pub fn the_builds_steps_are_named_and_counted(
    panel: &mut TheRunPanel,
    figures: Option<&crate::setting_up::TheBuildsFigures>,
    the_builds_name: impl Fn(&str) -> String,
) {
    use crate::setting_up::TheCountedAct;
    let Some(figures) = figures else {
        return;
    };
    let named = figures.the_build.as_deref().map(&the_builds_name);
    let called = |words: &str| match &named {
        Some(build) => format!("{words} {build}"),
        None => words.to_string(),
    };
    for step in &mut panel.steps {
        let under_way = step.mark.as_str() == "now";
        let counting = figures.how_far.filter(|_| under_way);
        match (step.boundary, counting) {
            (Boundary::Fetched, counting) => {
                step.name = Prose::new(&called(words::FETCHED));
                if let Some(how_far) = counting.filter(|it| it.act == TheCountedAct::TheDownload) {
                    step.the_figure = Some(the_downloads_figure(how_far.count));
                }
            }
            (Boundary::Tooling, Some(how_far)) if how_far.act == TheCountedAct::TheUnpack => {
                if let Some(figure) = the_unpacks_figure(how_far.count) {
                    step.name = Prose::new(&called(words::UNPACK_BLENDER));
                    step.the_figure = Some(figure);
                }
            }
            _ => {}
        }
    }
}

/// **The download's figure** — *214 of 352 MB*, or *214 MB* with no whole.
/// **Whole megabytes, a million bytes each, rounded down**, so the figure
/// never says more has arrived than has.
fn the_downloads_figure(count: crate::setting_up::TheStepsCount) -> AStepsFigure {
    let megabytes = |bytes: u64| bytes / 1_000_000;
    match count.whole {
        Some(whole) => AStepsFigure {
            so_far: Verbatim::new(megabytes(count.so_far).to_string()),
            of: Some(Prose::new(words::OF)),
            whole: Some(Verbatim::new(crate::typography::a_figure_with_its_unit(
                megabytes(whole),
                words::MB,
            ))),
        },
        None => AStepsFigure {
            so_far: Verbatim::new(crate::typography::a_figure_with_its_unit(
                megabytes(count.so_far),
                words::MB,
            )),
            of: None,
            whole: None,
        },
    }
}

/// **The unpack's figure** — *41%* of the archive fed, rounded down and never
/// past a hundred. **Nothing where the whole is not known or is nothing**: a
/// share of an unknown size says nothing.
fn the_unpacks_figure(count: crate::setting_up::TheStepsCount) -> Option<AStepsFigure> {
    let whole = count.whole.filter(|whole| *whole > 0)?;
    let share = (u128::from(count.so_far) * 100 / u128::from(whole)).min(100);
    Some(AStepsFigure {
        so_far: Verbatim::new(format!("{share}%")),
        of: None,
        whole: None,
    })
}

/// Which of the two streams a machine's words came from — **kept apart and
/// never merged** (`Security and licence` §13 rule 5).
enum WhichStream {
    Output,
    Complaint,
}

/// **S2a's four shapes, read for one stream.** A step cut mid-flight has no
/// words anywhere and none are claimed.
fn the_stream(said: &TheFailedStepsWords, which: WhichStream) -> Option<Verbatim> {
    let bytes = match (said, which) {
        (TheFailedStepsWords::TheWorkersOwnCapture { output, .. }, WhichStream::Output)
        | (TheFailedStepsWords::TheDesktopsOwnLoop { output, .. }, WhichStream::Output) => output,
        (TheFailedStepsWords::TheWorkersOwnCapture { complaint, .. }, WhichStream::Complaint)
        | (TheFailedStepsWords::TheDesktopsOwnLoop { complaint, .. }, WhichStream::Complaint) => {
            complaint
        }
        (TheFailedStepsWords::SmoozysOwnVerification { what_was_read }, WhichStream::Output) => {
            return Some(Verbatim::new(what_was_read.clone()))
        }
        _ => return None,
    };
    if bytes.is_empty() {
        return None;
    }
    Some(Verbatim::new(String::from_utf8_lossy(bytes).into_owned()))
}

// ════════════════════════════════════════════════════════════════════════════
// The two warnings — `Server Setup` W3's closed table
// ════════════════════════════════════════════════════════════════════════════

/// A warning before a step that could disturb work.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheWorkWarning {
    pub title: Prose,
    pub said: Prose,
    /// **Each live job told the truth about itself** — three outcomes rather
    /// than one sentence covering all three.
    pub the_live_jobs: Vec<ALiveJobLine>,
    pub not_now: Prose,
    pub go_ahead: Prose,
}

/// **W3's ready-action row**, which warns where the run carries driver work or
/// a restart.
///
/// **W3's first row names a transfer that would be interrupted and carried
/// on**, so the server's live transfers are an input beside its jobs, each told
/// the truth about itself (must-do 31 of the step 10a round 1 audit of these
/// pieces). **They are what the one transfer queue the Transfers screen reads
/// holds for this server** — and nothing in this application arranges a
/// transfer yet, so that queue holds none and the caller hands none; the board's
/// stubbed list says what fills it.
pub fn the_ready_actions_warning(
    live: &[(String, LiveState)],
    the_live_transfers: &[String],
) -> TheWorkWarning {
    TheWorkWarning {
        title: Prose::new(words::THIS_SERVER_HAS_WORK),
        said: Prose::new(words::SETTING_UP_RESTARTS),
        the_live_jobs: live
            .iter()
            .map(|(name, state)| ALiveJobLine {
                job: Verbatim::new(name.clone()),
                said: Prose::new(match state {
                    LiveState::Rendering => words::RENDERING_AND_A_RESTART,
                    LiveState::Waiting | LiveState::Held => words::WAITING_AND_A_RESTART,
                }),
            })
            .chain(the_live_transfers.iter().map(|named| ALiveJobLine {
                job: Verbatim::new(named.clone()),
                said: Prose::new(words::UPLOADING_AND_A_RESTART),
            }))
            .collect(),
        not_now: Prose::new(words::NOT_NOW),
        go_ahead: Prose::new(words::SET_UP_ANYWAY),
    }
}

/// **W3's install row**, which warns **only** where the unpack tool has to be
/// installed on the server first.
///
/// **Nothing on this path restarts the server, so the box names the work that
/// is live and claims nothing about what the change does to it** — the three
/// outcomes belong to the restart and this path has none.
pub fn the_unpack_tools_warning(live: &[(String, LiveState)]) -> TheWorkWarning {
    TheWorkWarning {
        title: Prose::new(words::UNPACK_TOOL_FIRST),
        said: Prose::new(words::THERE_IS_WORK),
        the_live_jobs: live
            .iter()
            .map(|(name, state)| ALiveJobLine {
                job: Verbatim::new(name.clone()),
                said: Prose::new(match state {
                    LiveState::Rendering => words::RENDERING,
                    LiveState::Waiting | LiveState::Held => words::WAITING,
                }),
            })
            .collect(),
        not_now: Prose::new(words::NOT_NOW),
        go_ahead: Prose::new(words::INSTALL_ANYWAY),
    }
}

/// **Whether `Server Setup` W3's ready-action row warns, and what it says** —
/// composed from the live work named and whether the run carries driver work or
/// a restart, so the judgement is driven at the desk whole. Added 2026-09-27.
///
/// **It warns only where both hold**: the step could disturb work (the
/// composition carries driver work or a restart) and there is work to disturb.
/// **The silence is as binding as the warning** — `Design/Server Setup § A long
/// step`: warning where nothing can be disturbed trains the user to click
/// through the one that matters. **A live job the machine holds and nothing
/// names still counts as work**: the box is drawn, without a line for it,
/// rather than a restart going ahead unannounced.
///
/// **What comes back beside the box is whether a render was among the live
/// jobs**, which is what the running panel's render sentence is later drawn on.
pub fn the_warning_before_the_ready_action(
    live: &[(Option<String>, LiveState)],
    it_carries_driver_work_or_a_restart: bool,
) -> Option<(TheWorkWarning, bool)> {
    if !it_carries_driver_work_or_a_restart || live.is_empty() {
        return None;
    }
    let named: Vec<(String, LiveState)> = live
        .iter()
        .filter_map(|(name, state)| name.clone().map(|name| (name, *state)))
        .collect();
    let a_render_is_running = live
        .iter()
        .any(|(_, state)| matches!(state, LiveState::Rendering));
    Some((the_ready_actions_warning(&named, &[]), a_render_is_running))
}

/// **`Queue & History` §17's live set, off one queue reading**: every job the
/// machine holds waiting, held or rendering, named from the local record or,
/// for work this copy did not submit, from the machine's own note
/// (`Queue & History` N1). **A job neither names is kept, unnamed** — it is
/// still work a restart would disturb.
pub fn the_live_set_named(
    reading: &smoozy_queue_history::acting::adoption::TheMachinesWorkingState,
    the_name_of: &mut dyn FnMut(&smoozy_state::identities::Identity) -> Option<String>,
) -> Vec<(Option<String>, LiveState)> {
    use smoozy_queue_history::line::QueueState;
    reading
        .jobs
        .iter()
        .filter_map(|job| {
            let state = match job.state {
                QueueState::Waiting => LiveState::Waiting,
                QueueState::Held { .. } => LiveState::Held,
                QueueState::Rendering { .. } => LiveState::Rendering,
                QueueState::Ended { .. } => return None,
            };
            Some((the_name_of(&job.job), state))
        })
        .collect()
}

/// **`Server Setup` W3's ready-action warning, judged against the machine now**
/// — the live set off a fresh queue reading, and the run's composition off the
/// ready action's own opening readings. **Nothing where no placement is
/// recorded**: a machine Smoozy has not placed itself on holds no work of
/// Smoozy's to disturb. **Where the machine's work cannot be read, the
/// period's last reading stands in for it**, and where there is none, no
/// warning is claimed. Added 2026-09-27.
fn the_warning_owed_before_the_ready_action<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    point: &crate::servers::sessions::ThisServersSessions,
    server: &str,
) -> Option<(TheWorkWarning, bool)> {
    use tauri::Manager;
    let started = app.try_state::<std::sync::Mutex<crate::start::Started>>()?;
    let record = crate::keys::TheRunsRecord {
        started: started.inner(),
    };
    let mut row = None;
    crate::keys::LendsTheRecord::lend(&record, &mut |connection| {
        row = crate::record::server_rows::one(connection, server).ok().flatten();
    });
    let crate::servers::installation::WhereThePackageAreaIs::At(package_area) =
        crate::servers::installation::where_the_package_area_is(&row?)
    else {
        return None;
    };
    let mut machine = crate::servers::the_machines_work::OverThisServersPoint {
        sessions: point.clone(),
        package_area,
    };
    let reading = match crate::servers::the_machines_work::the_queue_reading_from(&mut machine) {
        Ok(reading) => reading,
        Err(_) => app
            .try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>()?
            .the_machines_work()
            .every_server()
            .into_iter()
            .find(|(held, _)| held == server)
            .and_then(|(_, period)| period.reading)?
            .value()
            .clone(),
    };
    let live = the_live_set_named(&reading, &mut |job| {
        let mut label = None;
        crate::keys::LendsTheRecord::lend(&record, &mut |connection| {
            label = crate::record::job_records::JobRecords::over(connection)
                .read(job)
                .ok()
                .flatten()
                .map(|held| crate::surface::the_queue::what_the_job_is_called(&held))
                .filter(|called| !called.is_empty());
        });
        // **Called what Queue calls it** — a job sent with no name by its
        // file (step 10c's Vast pass, fault 3) — and by the machine's note,
        // which carries the label and no file, where the record holds none.
        label.or_else(|| {
            crate::servers::the_machines_work::the_note_of(&mut machine, job)
                .map(|note| crate::surface::the_queue::what_a_job_is_called(Some(&note.label), None))
        })
    });
    if live.is_empty() {
        return None;
    }
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .ok()?;
    let mut taking = crate::setting_up::TheOpeningReadingsOverThePoint { sessions: point };
    let it_restarts = runtime.block_on(crate::setting_up::the_ready_action_carries_a_restart(
        point,
        &mut taking,
    ));
    the_warning_before_the_ready_action(&live, it_restarts)
}

/// **The warnings shown and not yet answered, per server**, with whether each
/// named a render running. **Run-held, and which box is standing rather than
/// anything durable.** Added 2026-09-27.
fn the_warnings_shown() -> &'static Mutex<std::collections::HashMap<String, (TheWorkWarning, bool)>> {
    static HELD: std::sync::OnceLock<Mutex<std::collections::HashMap<String, (TheWorkWarning, bool)>>> =
        std::sync::OnceLock::new();
    HELD.get_or_init(|| Mutex::new(std::collections::HashMap::new()))
}

fn the_warning_is_held(server: &str, owed: (TheWorkWarning, bool)) {
    if let Ok(mut held) = the_warnings_shown().lock() {
        held.insert(server.to_string(), owed);
    }
}

/// **The user went ahead past the warning**: it is let go, and what comes back
/// is whether it named a render running.
fn the_warning_was_gone_past(server: &str) -> bool {
    the_warnings_shown()
        .lock()
        .ok()
        .and_then(|mut held| held.remove(server))
        .is_some_and(|(_, a_render_was_running)| a_render_was_running)
}

/// **The page is told a warning stands for this server, and asks for it.** The
/// event carries no box — P6's one route carries prose and values and nothing
/// composite — so the page asks [`the_work_warning`].
fn the_warning_is_owed_on(server: &str) -> Event {
    Event::about(About::TheRun, THE_ANSWER)
        .saying("the_work_warning_is_owed", Said::Value(Verbatim::new("yes")))
        .saying("server", Said::Value(Verbatim::new(server)))
}

/// **`Server Setup` W3's warning standing for this server**, or nothing where
/// none does. A command for the reason [`the_setup_under_way`] is one: the page
/// asking for what it has been told to draw. Added 2026-09-27.
#[tauri::command]
pub fn the_work_warning(server: String) -> Option<TheWorkWarning> {
    the_warnings_shown()
        .lock()
        .ok()
        .and_then(|held| held.get(&server).map(|(warning, _)| warning.clone()))
}

// ════════════════════════════════════════════════════════════════════════════
// The two boxes on the pointed-at route — `Server Setup` B7 and B7a
// ════════════════════════════════════════════════════════════════════════════

/// **The send, which is an ordinary upload** — a fifth kind for one file would
/// make the user look in two places for one answer.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheBuildGoingUp {
    pub title: Prose,
    /// The file's own name and size. Values.
    pub file: Verbatim,
    pub size: Verbatim,
    pub sent: Words,
    pub it_shows_in: Prose,
    pub transfers: Prose,
    pub with_your_other_uploads: Prose,
    pub stop: Prose,
    /// How far along, nought to one hundred. A value.
    pub how_far_along: Verbatim,
}

/// **B7a: the below-floor warning, fired between the version answer and the
/// publication.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheBelowTheFloorWarning {
    pub title: Words,
    pub supported: Words,
    pub nothing_is_written_for: Words,
    pub it_will_appear: Prose,
    pub dont_install_it: Prose,
    pub install_anyway: Prose,
}

/// **B7a, composed.** The version is a value in every one of the three
/// sentences that names it.
pub fn the_below_the_floor_warning(version: Version) -> TheBelowTheFloorWarning {
    let text = format!("{}.{}.{}", version.major, version.minor, version.patch);
    let floor = format!(
        "{}.{}",
        smoozy_server_setup::blender::THE_FLOOR.major,
        smoozy_server_setup::blender::THE_FLOOR.minor
    );
    TheBelowTheFloorWarning {
        title: Words {
            lead: Prose::new(words::THIS_BUILD_IS_BLENDER),
            values: vec![Verbatim::new(text.clone())],
            rest: None,
            the_rest_joins: false,
        },
        supported: Words {
            lead: Prose::new(words::SMOOZY_SUPPORTS),
            values: vec![Verbatim::new(floor)],
            rest: Some(Prose::new(words::AND_ABOVE)),
            the_rest_joins: false,
        },
        nothing_is_written_for: Words {
            lead: Prose::new(words::NOTHING_IS_WRITTEN_FOR),
            values: vec![Verbatim::new(text)],
            rest: Some(Prose::new(words::AND_SMOOZY_CANNOT_HELP)),
            the_rest_joins: true,
        },
        it_will_appear: Prose::new(words::IT_WILL_APPEAR),
        dont_install_it: Prose::new(words::DONT_INSTALL_IT),
        install_anyway: Prose::new(words::INSTALL_ANYWAY),
    }
}

/// **`Server Setup` B7a's question standing on `server`, composed** — nothing
/// where none stands, or where its version does not read. Added 2026-10-09,
/// the 38's piece 14b.
pub fn the_question_on(server: &str) -> Option<TheBelowTheFloorWarning> {
    crate::setting_up::the_question_standing_on(server)
        .as_deref()
        .and_then(Version::parse)
        .map(the_below_the_floor_warning)
}

/// **B7's send, composed.**
pub fn the_build_going_up(name: &str, size: &str, sent: &str, how_far_along: u8) -> TheBuildGoingUp {
    TheBuildGoingUp {
        title: Prose::new(words::INSTALLING_YOUR_BUILD),
        file: Verbatim::new(name),
        size: Verbatim::new(size),
        sent: Words {
            lead: Prose::new(words::SENDING),
            values: vec![Verbatim::new(sent), Verbatim::new(size)],
            rest: None,
            the_rest_joins: false,
        },
        it_shows_in: Prose::new(words::IT_SHOWS_IN),
        transfers: Prose::new(words::TRANSFERS),
        with_your_other_uploads: Prose::new(words::WITH_YOUR_OTHER_UPLOADS),
        stop: Prose::new(words::STOP),
        how_far_along: Verbatim::new(how_far_along.to_string()),
    }
}

/// Words with no value in them.
fn plainly(said: &str) -> Words {
    Words {
        lead: Prose::new(said),
        values: Vec::new(),
        rest: None,
        the_rest_joins: false,
    }
}

/// **`Design/Server Setup § Installing`: the word between *Uploading —* and the
/// size.** It is written here so the page holds none of it.
pub const OF: &str = words::OF;

#[cfg(test)]
mod tests {

    /// **Installs from the list alone, driven in turn** — the line's own driving
    /// with every act an install from the list, kept for the desk's tests of the
    /// chain that came before the line (2026-10-08 to 2026-10-27).
    pub fn the_installs_are_driven_one_after_another(
        server: &str,
        first: String,
        install: &mut dyn FnMut(String) -> crate::setting_up::WhereTheSetUpGot,
        what_a_job_is_called: &mut dyn FnMut(&smoozy_state::identities::Identity) -> Option<String>,
        told: &mut dyn FnMut(Event),
        keep: &mut dyn FnMut(&[String]),
    ) -> bool {
        use crate::setting_up::ASetUpAct;
        the_acts_are_driven_in_turn(
            server,
            ASetUpAct::AnInstallFromTheList(first),
            &mut |act| match act {
                ASetUpAct::AnInstallFromTheList(version) => install(version.clone()),
                other => panic!("only installs from the list are driven here, not {other:?}"),
            },
            what_a_job_is_called,
            told,
            &mut |line| {
                let versions = line
                    .iter()
                    .filter_map(|act| match act {
                        ASetUpAct::AnInstallFromTheList(version) => Some(version.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                keep(&versions)
            },
        )
    }

    /// **The user's own build, driven as one act of the line** — with its ending
    /// naming the build as the user's, by its file name, wherever it was not
    /// installed (`Design/Server Setup § Pointing at a build on your own
    /// computer`). `install` is handed the file's name. Kept for the desk's tests.
    pub fn the_users_build_is_driven(
        server: &str,
        file: String,
        install: &mut dyn FnMut(String) -> crate::setting_up::WhereTheSetUpGot,
        what_a_job_is_called: &mut dyn FnMut(&smoozy_state::identities::Identity) -> Option<String>,
        told: &mut dyn FnMut(Event),
        keep: &mut dyn FnMut(&[String]),
    ) -> bool {
        use crate::setting_up::ASetUpAct;
        the_acts_are_driven_in_turn(
            server,
            ASetUpAct::TheUsersBuild(std::path::PathBuf::from(&file)),
            &mut |act| match act {
                ASetUpAct::TheUsersBuild(_) => install(file.clone()),
                other => panic!("only the user's build is driven here, not {other:?}"),
            },
            what_a_job_is_called,
            told,
            &mut |line| keep(&line.iter().map(|act| act.as_kept().named).collect::<Vec<_>>()),
        )
    }

    /// **Step 10b's first audit round, item 13: the admin access box takes its
    /// store probe off the thread that draws.** A command that is not `async`
    /// runs on that thread, and the probe is a put, a get and an erase that may
    /// each prompt on Linux. Tested by shape, as the period's P5 test is: the
    /// command is `async`, and the probe is reached only from the function the
    /// blocking pool runs.
    #[test]
    fn the_admin_access_boxs_probe_runs_off_the_thread_that_draws() {
        let working = include_str!("set_up.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the working half");
        assert!(working.contains("pub async fn the_admin_access_box<"));
        let at = working
            .find("pub async fn the_admin_access_box<")
            .expect("the command");
        let command = &working[at..at + working[at..].find("\n}\n").expect("it closes")];
        assert!(command.contains("spawn_blocking"));
        assert!(!command.contains("at_the_first_use_of_a_secret"));
        let composed = working
            .find("fn the_admin_access_box_is_composed<")
            .expect("the composition");
        assert!(working[composed..].contains("at_the_first_use_of_a_secret"));
        assert_eq!(
            working.matches("at_the_first_use_of_a_secret(").count(),
            1,
            "the probe is taken in one place on this tab"
        );
    }

    /// **Step 10b's first audit round, item 7: the admin access box carries
    /// `Security and licence` S34's sentence only where it is told to**, and
    /// the box composed from the standing alone never does.
    #[test]
    fn the_admin_access_box_says_the_once_said_sentence_only_where_told() {
        for standing in [
            Some(StoreStanding::NeverAnswered),
            Some(StoreStanding::HasAnswered),
            None,
        ] {
            assert!(the_box("Installing the GPU driver", standing, false)
                .nowhere_to_keep_them
                .is_none());
        }
        let page = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js"),
        )
        .expect("the page");
        assert!(page.contains("asked.nowhere_to_keep_them"));
    }
    use super::*;
    use smoozy_server_setup::run::SetupKind;
    use std::fs;
    use std::path::Path;

    /// **What the page was delivered**, off the real queue.
    struct WhatThePageWasTold(std::sync::Arc<std::sync::Mutex<Vec<crate::events::Delivery>>>);

    impl crate::events::Delivers for WhatThePageWasTold {
        fn deliver(&self, delivery: &crate::events::Delivery) {
            self.0.lock().expect("the deliveries").push(delivery.clone());
        }
    }

    fn a_queue_the_page_listens_to() -> (
        std::sync::Arc<TheOneQueue>,
        std::sync::Arc<std::sync::Mutex<Vec<crate::events::Delivery>>>,
    ) {
        let queue = std::sync::Arc::new(TheOneQueue::new());
        let told = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        queue.the_page_is_ready(Box::new(WhatThePageWasTold(std::sync::Arc::clone(&told))));
        (queue, told)
    }

    /// **One event, as the page is handed it.**
    fn delivered(event: Event) -> crate::events::Delivery {
        let (queue, told) = a_queue_the_page_listens_to();
        the_queue_is_told(&queue, event);
        let told = told.lock().expect("the deliveries").clone();
        told.into_iter().next().expect("the event was delivered")
    }

    fn it_says(delivery: &crate::events::Delivery, name: &str) -> bool {
        delivery.said.get(name) == Some(&Said::Value(Verbatim::new("yes")))
    }

    /// **Step 10c's ninth-pass fix piece 4: the control carries the mark from
    /// the press, so the page is told before the warning's reading is taken** —
    /// the ninth pass on Hyperstack saw 11 seconds of nothing on *Set up* while
    /// `Server Setup` W3's reading ran. **No words travel**: the control keeps
    /// its own name under the mark.
    #[test]
    fn the_page_is_told_the_press_is_under_way_before_the_warnings_reading_is_taken() {
        let (queue, told) = a_queue_the_page_listens_to();
        let at_the_reading = the_press_is_marked_while(&queue, "server-1", || {
            told.lock().expect("the deliveries").clone()
        });
        assert_eq!(at_the_reading.len(), 1, "the page was not told before the reading");
        let marked = &at_the_reading[0];
        assert_eq!(marked.kind, THE_ANSWER);
        assert!(it_says(marked, "the_action_is_underway"), "{marked:?}");
        assert_eq!(
            marked.said.get("server"),
            Some(&Said::Value(Verbatim::new("server-1")))
        );
        assert!(
            !marked.said.values().any(|said| matches!(said, Said::Prose(_))),
            "the mark carried words: {marked:?}"
        );
    }

    /// **Step 10c's ninth-pass fix piece 4, on the card since 2026-10-02: a run
    /// taken and not yet read carries the mark on the card's state line until
    /// its steps come** — `Server Setup` R2's placement stood about 40 seconds
    /// on the ninth pass with no mark. **The start of a run is told as the run
    /// moving on, with no words of its own**, and the card asks for it; *Cancel
    /// setup* is its one control (the Creator's notes piece 2).
    #[test]
    fn a_setup_just_started_is_drawn_on_the_card_with_its_state_line_marked() {
        let (queue, told) = a_queue_the_page_listens_to();
        the_queue_is_told(&queue, the_run_moved_on("server-1"));
        let told = told.lock().expect("the deliveries").clone();
        assert!(it_says(&told[0], "the_run_is_under_way"), "{:?}", told[0]);
        assert!(!told[0].said.values().any(|said| matches!(said, Said::Prose(_))), "{:?}", told[0]);
        for called in [words::SETTING_UP, words::INSTALLING_BLENDER, words::ABANDONING_SETUP] {
            let card = the_run_before_its_first_reading(called, false);
            assert_eq!(card.title, Prose::new(called));
            assert!(card.the_state_is_underway, "nothing else on the card moves");
            assert!(card.steps.is_empty());
            assert_eq!(card.cancel, Some(Prose::new(words::CANCEL_SETUP)));
            assert_eq!(card.what_cancel_leaves, Some(Prose::new(words::CANCEL_SETUP_LEAVES)));
            assert!(card.abandon.is_none() && card.set_up_again.is_none());
        }
        assert!(the_run_before_its_first_reading(words::SETTING_UP, true).the_cancel_was_asked);
    }

    /// This file's working half — everything before the tests. **The sweeps
    /// below read it rather than the whole file**, because the things they
    /// forbid are written out here in order to forbid them.
    fn the_working_half() -> &'static str {
        include_str!("set_up.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half")
    }

    fn the_page() -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js");
        fs::read_to_string(path).expect("the page module is readable")
    }

    fn the_mockup() -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../Preparation/Design/Mockup/Smoozy.html");
        fs::read_to_string(path).expect("the drawing is readable")
    }

    /// **The thirty-seven mockup states this piece is counted by**, which the
    /// runner ruled on 2026-09-12 are what this half is measured with.
    /// *(Thirty-three until 2026-10-24, when the eighteenth notes' design pass
    /// drew `su-fetchlog`, `su-stepten` and `su-steptenlog` — Server output
    /// while Blender downloads, and the stopped ten-step card with its failure
    /// once on the step. **None of the three is built yet**: they are owed to
    /// that work's piece 2, the Set up card.)*
    /// *(Thirty-one until 2026-10-24, when the sixteenth notes' note 1 built
    /// `su-words` and `su-wordslog`, drawn by its plan pass.)*
    /// *(Twenty-nine until 2026-10-18, when the fourteenth notes' piece U
    /// built `su-fetch`.)*
    /// *(Twenty-seven until 2026-10-18, when the thirteenth notes' piece H
    /// built `su-out` and `su-outlog`.)*
    /// *(Twenty-six until 2026-10-16, when step 10c's owed piece 4 drew and
    /// built `su-notbegun`.)*
    /// *(Twenty-four until 2026-10-11, when the plan pass for OptiX and the
    /// driver drew `su-optixoff` and `su-drvold`, built by that work's fourth
    /// piece.)*
    /// *(Twenty-three until 2026-10-03, when the drawing pass for the
    /// Creator's notes on the test copy drew `su-connectfail` — the card where
    /// an add's connection was not made, their piece 1.)*
    /// *(Twenty-two until 2026-10-02, when the plan pass for the Creator's
    /// notes drew `su-connect` — the card while an add's connection is made,
    /// the Creator's notes piece 3.)*
    const THE_DRAWN_STATES: [&str; 39] = [
        "su",
        "su-connect",
        "su-connectfail",
        "su-notset",
        "su-add",
        "su-unpack",
        "su-point",
        "su-low",
        "su-notbegun",
        "su-run",
        "su-queue",
        // The finished card, drawn 2026-10-29 by the twenty-first notes'
        // design pass (note 1).
        "su-done",
        "su-cpu",
        "su-quiet",
        "su-none",
        "su-no-blender",
        "su-nothing",
        "su-optixoff",
        "su-drvold",
        "su-tg",
        "su-tgnew",
        "su-going",
        "su-busy",
        "su-fail",
        "su-step",
        "su-rm",
        "su-elev",
        "pw-nokeep",
        "pw-nostore",
        "pw-again",
        "su-out",
        "su-outlog",
        "su-fetch",
        "su-logempty",
        "su-words",
        "su-wordslog",
        // Owed to the eighteenth notes' piece 2; see above.
        "su-fetchlog",
        "su-stepten",
        "su-steptenlog",
    ];

    fn at(seconds: u64) -> std::time::SystemTime {
        std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(seconds)
    }

    fn a_boundary(boundary: Boundary, did: WhatTheStepDid) -> RecordedBoundary {
        RecordedBoundary {
            boundary,
            what_the_step_did: did,
            at: at(1_754_000_000),
            clock: smoozy_state::observed::Clock::AMachine(
                smoozy_state::observed::MachineClock("RunPod 4090".into()),
            ),
        }
    }

    /// A ready action that ran the six steps the mockup draws.
    fn a_finished_run(did: WhatTheStepDid) -> TheSetupRecord {
        TheSetupRecord {
            kind: SetupKind::TheReadyAction,
            the_composed_list: vec![
                Boundary::Started,
                Boundary::Libraries,
                Boundary::DriverPlaced,
                Boundary::RestartIssued,
                Boundary::CardAnswering,
                Boundary::Telegram,
                Boundary::Done,
            ],
            the_boundaries_reached: vec![
                a_boundary(Boundary::Started, did),
                a_boundary(Boundary::Libraries, did),
                a_boundary(Boundary::DriverPlaced, did),
                a_boundary(Boundary::RestartIssued, did),
                a_boundary(Boundary::CardAnswering, did),
                a_boundary(Boundary::Telegram, did),
                a_boundary(Boundary::Done, WhatTheStepDid::Performed),
            ],
            ending: Some(HowTheRunEnded::Completed),
        }
    }

    fn a_reading() -> ASetUpReading {
        ASetUpReading {
            the_records: vec![a_finished_run(WhatTheStepDid::Performed)],
            telegram_reporting_is_installed: true,
            telegram_is_switched_on: true,
            a_setup_is_running: None,
            the_step_in_flight: None,
            the_finished_jobs: WhatTheThreeQuestionsAnswered {
                any_finished_job_record_exists: true,
                the_most_recent: Some("job-1".to_string()),
                the_most_recent_finished_record_carrying_a_device_line: Some("job-1".to_string()),
            },
            the_device_reading: Some(ADeviceReading {
                outcome: RecordedOutcome::Finished,
                asked_for: Some(WhatTheManifestAskedFor::TheCards),
                line: Some(WhatTheLineNames::TheCards),
                the_lines_words: "1 × RTX 4090 · CUDA".to_string(),
            }),
            the_builds: vec![ABuildAsRead {
                path: "/opt/blender/5.2.1/blender".to_string(),
                version: Version {
                    major: 5,
                    minor: 2,
                    patch: 1,
                },
                long_term_support: false,
                origin: Some(HowItGotThere::SmoozyInstalledItFromTheList),
                the_managers_package: None,
            }],
            the_library_half_was_read: true,
            folders_the_search_could_not_enter: 0,
            the_clock: "RunPod 4090".to_string(),
            a_placement_came_first: false,
        }
    }

    /// **`su` — the tab as the drawing opens it.**
    #[test]
    fn a_ready_server_reads_ready_with_the_device_line_and_a_quiet_control() {
        let card = the_card_from(&a_reading());
        assert_eq!(card.state, Prose::new(words::READY_TO_RENDER));
        assert_eq!(card.dot, Verbatim::new("ok"));
        assert!(!card.the_action.loud, "a healthy server's control is not the loud one");
        assert_eq!(card.the_action.name, Prose::new(words::SET_UP_AGAIN));
        let line = card.the_render_device.expect("a device line");
        assert_eq!(line.lead, Prose::new(words::LAST_JOB_RENDERED_ON));
        assert_eq!(line.values[0], Verbatim::new("1 × RTX 4090 · CUDA"));
    }

    /// **`su-notset` — no ready-action record at all.**
    ///
    /// `Server Setup` Y6a: *has this server ever been set up* is the record's
    /// presence and nothing else.
    #[test]
    fn a_server_with_no_ready_action_record_is_not_set_up_and_the_control_is_loud() {
        let mut reading = a_reading();
        reading.the_records = Vec::new();
        reading.telegram_reporting_is_installed = false;
        let card = the_card_from(&reading);
        assert_eq!(card.state, Prose::new(words::NOT_SET_UP));
        assert!(card.the_action.loud);
        assert_eq!(card.the_action.name, Prose::new(words::SET_UP_THIS_SERVER));
        assert!(card.when.is_none(), "nothing was done to it, so no moment is named");
        assert!(card.what_was_done.is_empty());
    }

    /// **`su-tgnew` — the card the Telegram module would draw, minus its
    /// control.**
    ///
    /// **`Server Setup` Y6b and G3 are kept whole**: a run of the dedicated
    /// entry point's own kind replaces no ready-action record, so the card goes
    /// on saying the server is not set up while Telegram reporting is installed
    /// on it. **What is missing is the offer and not the reading.**
    #[test]
    fn a_machine_never_prepared_with_telegram_on_it_says_both_things() {
        let mut reading = a_reading();
        reading.the_records = vec![TheSetupRecord {
            kind: SetupKind::TheTelegramPlacement,
            the_composed_list: vec![Boundary::Telegram, Boundary::Done],
            the_boundaries_reached: vec![
                a_boundary(Boundary::Telegram, WhatTheStepDid::Performed),
                a_boundary(Boundary::Done, WhatTheStepDid::Performed),
            ],
            ending: Some(HowTheRunEnded::Completed),
        }];
        let card = the_card_from(&reading);
        assert_eq!(card.state, Prose::new(words::NOT_SET_UP));
        assert_eq!(card.what_was_done.len(), 1);
        assert_eq!(
            card.the_note.expect("the odd-reading line").lead,
            Prose::new(words::TELEGRAM_AND_THE_REST_IS_NOT_RUN)
        );
    }

    /// **`su-cpu` — `Server Setup` W4's setup fault, and the one state where
    /// the action earns being loud.**
    #[test]
    fn a_finished_cards_job_that_rendered_on_the_processor_is_a_setup_fault() {
        let mut reading = a_reading();
        reading.the_device_reading = Some(ADeviceReading {
            outcome: RecordedOutcome::Finished,
            asked_for: Some(WhatTheManifestAskedFor::TheCards),
            line: Some(WhatTheLineNames::TheProcessorAlone),
            the_lines_words: "CPU".to_string(),
        });
        let card = the_card_from(&reading);
        let fault = card.the_setup_fault.expect("a setup fault");
        assert_eq!(fault.said, Prose::new(words::THE_LAST_JOB_ASKED_FOR_THE_GPU));
        assert!(card.the_action.loud);
        assert_eq!(card.dot, Verbatim::new("busy"));
        assert!(
            card.the_render_device.is_none(),
            "the amber line stands in the device line's place rather than beside it"
        );
    }

    /// **A job that asked for the processor and got it is not a fault** —
    /// `Design/Server Setup § This server`, Creator-stated: firing on every
    /// processor render would tell somebody with a healthy server to set it up
    /// again, and pressing that can restart the server.
    #[test]
    fn a_job_that_asked_for_the_processor_says_nothing_about_the_machine() {
        let mut reading = a_reading();
        reading.the_device_reading = Some(ADeviceReading {
            outcome: RecordedOutcome::Finished,
            asked_for: Some(WhatTheManifestAskedFor::TheProcessor),
            line: Some(WhatTheLineNames::TheProcessorAlone),
            the_lines_words: "CPU".to_string(),
        });
        let card = the_card_from(&reading);
        assert!(card.the_setup_fault.is_none());
        assert!(!card.the_action.loud);
    }

    /// **`su-quiet` — jobs ran here and none of them said**, which is a third
    /// case and not the never-run-anything one.
    ///
    /// **And a device line from a run that did not finish reaches it too**,
    /// because W4 concludes nothing from one.
    #[test]
    fn jobs_that_said_nothing_read_as_the_third_line_and_never_as_the_first() {
        let mut reading = a_reading();
        reading.the_finished_jobs.the_most_recent_finished_record_carrying_a_device_line = None;
        assert_eq!(
            the_card_from(&reading).the_render_device.expect("a line").lead,
            Prose::new(words::NONE_OF_THEM_SAID)
        );

        let mut stopped = a_reading();
        stopped.the_device_reading = Some(ADeviceReading {
            outcome: RecordedOutcome::AnythingElse,
            asked_for: Some(WhatTheManifestAskedFor::TheCards),
            line: Some(WhatTheLineNames::TheProcessorAlone),
            the_lines_words: "CPU".to_string(),
        });
        assert_eq!(
            the_card_from(&stopped).the_render_device.expect("a line").lead,
            Prose::new(words::NONE_OF_THEM_SAID)
        );
        assert!(
            the_card_from(&stopped).the_setup_fault.is_none(),
            "a device line printed by a run that did not finish is not evidence of anything"
        );
    }

    /// **`su-none` — nothing has rendered here yet.**
    #[test]
    fn a_server_nothing_has_rendered_on_says_so() {
        let mut reading = a_reading();
        reading.the_finished_jobs.any_finished_job_record_exists = false;
        reading.the_finished_jobs.the_most_recent_finished_record_carrying_a_device_line = None;
        assert_eq!(
            the_card_from(&reading).the_render_device.expect("a line").lead,
            Prose::new(words::NOTHING_HAS_RENDERED_YET)
        );
    }

    /// **`su-nothing` — a run that had nothing to do reads *nothing to do* on
    /// every step and never *done*.**
    ///
    /// **And the library step reads differently again**, because it had no
    /// subject: a step that never ran is not a step that found nothing to do,
    /// and collapsing the two would have Smoozy claiming a check it never made.
    #[test]
    fn a_run_with_nothing_to_do_marks_every_step_and_the_library_step_differently() {
        let mut reading = a_reading();
        reading.the_records = vec![a_finished_run(WhatTheStepDid::NothingToDo)];
        // **The library check had no subject**, which is the one case that
        // reads differently — and it is a reading of the run, never of the
        // boundary list.
        reading.the_library_half_was_read = false;
        let card = the_card_from(&reading);
        assert_eq!(card.state, Prose::new(words::NOTHING_TO_PREPARE));
        // **Every step carries its mark; Telegram's line under them is the
        // role's own reading and carries none** (the Creator's note 6 on 0.1.7).
        let marks: Vec<String> = card
            .what_was_done
            .iter()
            .filter(|line| line.said != plainly(words::TELEGRAM_INSTALLED))
            // **The final check ran, so it reads *done*** — a check, not an
            // install (the pass before 0.1.9 on Vast put it on this card).
            .filter(|line| line.said != plainly(words::DONE_STEP))
            .map(|line| line.mark.clone().expect("a mark").as_str().to_string())
            .collect();
        assert!(!marks.is_empty());
        assert!(marks.iter().any(|it| it.replace('\u{a0}', " ").contains("no Blender installed")));
        assert!(!marks.iter().any(|it| it == "done"));
        assert_eq!(
            card.the_note.expect("the summary").lead,
            Prose::new(words::NOTHING_WAS_INSTALLED)
        );
    }

    /// **`Server Setup` W1: a *Set up* that placed Smoozy's program never reads
    /// *Nothing to prepare* or *Nothing was installed and nothing was
    /// changed.*** Step 10c's twelfth-pass fix piece 6, in the twelfth pass's
    /// own order: a server added plainly confirms no installation, *Set up this
    /// server* places one, and the run after it finds every step with nothing
    /// to do. **The same run after a press that placed nothing still reads
    /// *Nothing to prepare***, which is W1's own case.
    #[test]
    fn a_set_up_that_placed_smoozy_reads_as_a_setup_that_did_something() {
        const SERVER: &str = "twelfth-pass fix piece 6 — Hyperstack A6000 two";
        let the_run_read = || {
            let mut record = a_finished_run(WhatTheStepDid::NothingToDo);
            record.the_composed_list.retain(|it| *it != Boundary::Telegram);
            record.the_boundaries_reached.retain(|it| it.boundary != Boundary::Telegram);
            WhatTheMachineSaid::ASetupRunAlone(Box::new(crate::setting_up::TheSetupAsRead {
                the_run_holds_the_machine: false,
                record,
                still_open: false,
                the_step_in_flight: None,
                the_admin_step_recorded_as_started: None,
                the_clock: "Hyperstack A6000 two".to_string(),
            }))
        };

        // The press placed a new program: the row moved from no installation
        // to one.
        the_ready_action_ended_on(SERVER, None, Some("7e7f4734"));
        let card = the_tab(the_run_read(), SERVER).the_card;
        assert_eq!(card.state, Prose::new(words::READY_TO_RENDER));
        assert!(card.the_note.is_none(), "a setup that placed Smoozy says nothing was changed");
        assert_eq!(
            card.when.as_ref().expect("the moment").lead,
            Prose::new(words::SET_UP),
            "a setup that placed Smoozy reads as checked"
        );
        // **The list says what each step did, and every one found nothing to
        // do** — the steps, each with its mark, as every finished card lists
        // them since the Creator's note 6 on 0.1.7.
        assert!(!card.what_was_done.is_empty());
        assert!(card
            .what_was_done
            .iter()
            .filter(|line| line.said != plainly(words::DONE_STEP))
            .all(|line| line.mark.as_ref().is_some_and(|mark| mark.as_str() != words::DONE)));

        // *Set up again* on the same installation placed nothing.
        the_ready_action_ended_on(SERVER, Some("7e7f4734"), Some("7e7f4734"));
        let card = the_tab(the_run_read(), SERVER).the_card;
        assert_eq!(card.state, Prose::new(words::NOTHING_TO_PREPARE));
        assert_eq!(card.the_note.expect("the summary").lead, Prose::new(words::NOTHING_WAS_INSTALLED));

        // A press that ended with no installation confirmed placed nothing.
        the_ready_action_ended_on(SERVER, None, None);
        assert_eq!(the_tab(the_run_read(), SERVER).the_card.state, Prose::new(words::NOTHING_TO_PREPARE));
    }

    /// **The card always lists the last setup's steps, each with its mark, and
    /// Telegram's line after them** — `Design/Server Setup § This server`, the
    /// Creator's note 6 on 0.1.7. **A first setup that installed a Blender**
    /// listed nothing before this (on a server with Telegram off), and *Set up
    /// again* listed every step: one shape now. *Install the GPU driver —
    /// done* is what says the driver went in, and over a working driver it
    /// reads *nothing to do* (twelfth-pass fix piece 6b, `Server Setup` Y4a).
    #[test]
    fn a_finished_card_lists_every_step_with_its_mark_and_telegram_after_them() {
        let mut reading = a_reading();
        let mut record = a_finished_run(WhatTheStepDid::NothingToDo);
        let at = record.the_composed_list.iter().position(|it| *it == Boundary::Telegram).expect("telegram");
        record.the_composed_list.insert(at, Boundary::Blender);
        let at = record.the_boundaries_reached.iter().position(|it| it.boundary == Boundary::Telegram).expect("telegram");
        record.the_boundaries_reached.insert(at, a_boundary(Boundary::Blender, WhatTheStepDid::Performed));
        reading.the_records = vec![record.clone()];
        reading.telegram_reporting_is_installed = false;
        reading.telegram_is_switched_on = false;
        let card = the_card_from(&reading);
        assert_eq!(card.state, Prose::new(words::READY_TO_RENDER), "the run did something");
        let line = |name: &str| {
            card.what_was_done
                .iter()
                .find(|line| line.said == plainly(name))
                .and_then(|line| line.mark.clone())
        };
        assert_eq!(line(words::BLENDER_STEP), Some(Prose::new(words::DONE)), "{:?}", card.what_was_done);
        assert_eq!(line(words::DRIVER_PLACED), Some(Prose::new(words::NOTHING_TO_DO)));
        let named = record
            .the_boundaries_reached
            .iter()
            .filter(|it| the_boundarys_name(it.boundary).is_some() || it.boundary == Boundary::Done)
            .count();
        assert_eq!(card.what_was_done.len(), named, "every step, and nothing else");
        // **The final check is one of them**, as it is while the run goes —
        // the pass before 0.1.9 on Vast found it missing from the finished card.
        assert_eq!(line(words::DONE_STEP), Some(Prose::new(words::DONE)), "{:?}", card.what_was_done);

        // **A run whose driver step installed one says so on that step.**
        let mut performed = a_finished_run(WhatTheStepDid::NothingToDo);
        for it in performed.the_boundaries_reached.iter_mut().filter(|it| it.boundary == Boundary::DriverPlaced) {
            it.what_the_step_did = WhatTheStepDid::Performed;
        }
        reading.the_records = vec![performed];
        let card = the_card_from(&reading);
        let driver = card.what_was_done.iter().find(|line| line.said == plainly(words::DRIVER_PLACED));
        assert_eq!(driver.and_then(|it| it.mark.clone()), Some(Prose::new(words::DONE)));

        // **Telegram's step says it, and no second line is drawn** where the
        // server reads it installed (the twenty-first notes, note 1).
        reading.the_records = vec![record];
        reading.telegram_reporting_is_installed = true;
        let card = the_card_from(&reading);
        assert_eq!(line(words::TELEGRAM_STEP), Some(Prose::new(words::NOTHING_TO_DO)));
        assert!(!card.what_was_done.iter().any(|line| line.said == plainly(words::TELEGRAM_INSTALLED)));
        assert_eq!(card.what_was_done.len(), named);
    }

    /// **The Creator's twenty-first notes, note 1: Telegram is said once.**
    /// `Design/Server Setup § This server`: the role's own line is drawn only
    /// where it says something the steps do not — never under a run that
    /// lists *Install Telegram reporting* while the server reads it installed;
    /// still where the steps do not list it (Telegram added later by *Install
    /// it*), and still where the reading differs from the step. Drawn as
    /// `su-done`. Before the change the first card carried both lines.
    #[test]
    fn twenty_first_a_finished_card_says_telegram_once() {
        let said = |card: &TheCard, words: &str| card.what_was_done.iter().filter(|line| line.said == plainly(words)).count();

        // The Creator's A6000: the step *done*, and the server agrees.
        let reading = a_reading();
        let card = the_card_from(&reading);
        assert_eq!(said(&card, words::TELEGRAM_STEP), 1, "{:?}", card.what_was_done);
        assert_eq!(said(&card, words::TELEGRAM_INSTALLED), 0, "{:?}", card.what_was_done);
        assert_eq!(
            card.what_was_done.last().map(|line| line.said.clone()),
            Some(plainly(words::DONE_STEP)),
            "the final check ends the list"
        );

        // Set up with Telegram off, then *Install it*: the steps do not say
        // it, so the reading's line does, after them.
        let mut reading = a_reading();
        let mut record = a_finished_run(WhatTheStepDid::Performed);
        record.the_composed_list.retain(|it| *it != Boundary::Telegram);
        record.the_boundaries_reached.retain(|it| it.boundary != Boundary::Telegram);
        reading.the_records = vec![record];
        let card = the_card_from(&reading);
        assert_eq!(said(&card, words::TELEGRAM_STEP), 0);
        assert_eq!(
            card.what_was_done.last().map(|line| line.said.clone()),
            Some(plainly(words::TELEGRAM_INSTALLED))
        );

        // The step reads *done* and the server no longer reads it installed:
        // the reading differs, so its line and *Install it* stay.
        let mut reading = a_reading();
        reading.telegram_reporting_is_installed = false;
        let card = the_card_from(&reading);
        assert_eq!(said(&card, words::TELEGRAM_STEP), 1);
        assert_eq!(said(&card, words::TELEGRAM_NOT_INSTALLED), 1, "{:?}", card.what_was_done);
    }

    /// **The page puts each step mark's colour on the mark, from the value the
    /// application sent**, in both lists that carry step marks — the card's and
    /// the run's (`Design/Server Setup § This server`, the Creator's note 3 on
    /// the test copy). **And every colour is one the drawing's stylesheet
    /// already draws**, so none is chosen here.
    #[test]
    fn the_page_puts_each_marks_colour_on_the_mark() {
        let page = the_page();
        assert!(page.contains("if (line.colour) mark.classList.add(line.colour);"), "the card's list");
        assert!(page.contains("if (step.colour) mark.classList.add(step.colour);"), "the run's list");
        let drawing = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("web/the-drawing.css"))
            .expect("the drawing's stylesheet is readable");
        for colour in [
            the_marks_colour::DONE,
            the_marks_colour::NOTHING_TO_DO,
            the_marks_colour::NOT_CHECKED,
            the_marks_colour::FAILED,
        ] {
            assert!(drawing.contains(&format!(".{colour} {{")), "the drawing draws {colour}");
            assert!(the_mockup().contains(&format!("class=\"stm {colour}\"")), "the mockup marks with {colour}");
        }
    }

    /// **The eighteenth notes, note 4: a step number of two digits lines up
    /// with the rest by its point and stays inside the card** —
    /// `Design/Server Setup § The step that failed`, the drawing's
    /// `su-stepten`. The Creator's stopped card of ten steps had *10.* hanging
    /// out of the card's edge (screenshot 1), because the number was the web
    /// view's own marker, whose gap after the point is the engine's.
    ///
    /// **What holds now**: the drawing's column is the one the step names and
    /// *Server output* share; `web/set-up.css` draws the number itself in that
    /// column, from the list's count, ending at the step name's edge with one
    /// space after its point, in figures of one width — and chooses no width.
    /// **`the-drawing.css` is left the mockup's own.** Before the piece there
    /// was no such rule, and this fails.
    #[test]
    fn eighteenth_two_digit_step_numbers_sit_in_the_drawings_column() {
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        let read = |name: &str| fs::read_to_string(web.join(name)).expect("a stylesheet is readable");
        let rules_of = |css: &str| -> Vec<(String, String)> {
            let mut bare = String::new();
            let mut rest = css;
            while let Some(at) = rest.find("/*") {
                bare.push_str(&rest[..at]);
                rest = rest[at..].find("*/").map_or("", |end| &rest[at + end + 2..]);
            }
            bare.push_str(rest);
            bare.split('}')
                .filter_map(|rule| rule.split_once('{'))
                .map(|(selector, body)| {
                    (
                        selector.split_whitespace().collect::<Vec<_>>().join(" "),
                        body.split_whitespace().collect::<Vec<_>>().join(" "),
                    )
                })
                .collect()
        };
        let the_last = |rules: &[(String, String)], selector: &str| -> String {
            rules
                .iter()
                .filter(|(one, _)| one == selector)
                .map(|(_, body)| body.clone())
                .last()
                .unwrap_or_else(|| panic!("no rule for {selector}"))
        };
        // **The drawing's column, shared by the names and *Server output*.**
        let drawing = rules_of(&read("the-drawing.css"));
        assert!(the_last(&drawing, ".steps").contains("padding-left: 28px"));
        assert!(the_last(&drawing, ".suout").contains("padding-left: 28px"));
        assert!(!read("the-drawing.css").contains("counter("), "the drawing's own sheet was changed");

        // **The number, drawn in that column and sized by nothing here.**
        let set_up = rules_of(&read("set-up.css"));
        let list = the_last(&set_up, ".sucard .steps");
        assert!(list.contains("list-style: none") && list.contains("counter-reset: su-step"), "{list}");
        let step = the_last(&set_up, ".sucard .steps > li");
        assert!(step.contains("position: relative") && step.contains("counter-increment: su-step"), "{step}");
        let number = the_last(&set_up, ".sucard .steps > li::before");
        for part in [
            "content: counter(su-step) \". \";",
            "white-space: pre;",
            "position: absolute;",
            "right: 100%;",
            "font-variant-numeric: tabular-nums;",
        ] {
            assert!(number.contains(part), "{part} is not in {number}");
        }
        for chosen in ["px", "em", "width", "margin", "padding", "color", "font-style", "font-weight"] {
            assert!(!number.contains(chosen), "the number sets {chosen} of its own: {number}");
        }

        // **And the run's list is drawn inside the card those rules name.**
        let page = the_page();
        assert!(page.contains("one.className = \"sucard\";"), "the card");
        assert!(page.contains("steps.className = \"steps\";"), "the run's list");
    }

    /// **The same, measured in a real browser**: a stopped card of ten steps,
    /// drawn in the shapes the page draws and styled by the stylesheets the
    /// app loads, has every number inside the card's edge — the state line's
    /// left edge — and ending at its step name's edge, so *10.* lines up with
    /// *9.* by its point. Before the piece there was no drawn number to
    /// measure and this fails. **Chrome is not the app's web view**, so this
    /// proves the rule's geometry, not every engine's marker; that is why the
    /// number no longer is the marker. A computer with no Chrome skips it.
    #[test]
    fn eighteenth_two_digit_step_numbers_stay_inside_the_card_in_a_real_browser() {
        let Some(chrome) = a_chrome() else {
            eprintln!("SKIPPED: no Chrome on this computer, so the step numbers were not measured");
            return;
        };
        let folder = std::env::temp_dir().join(format!("smoozy-eighteenth-ten-steps-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for entry in fs::read_dir(&web).expect("the page folder") {
            let path = entry.expect("an entry").path();
            if path.extension().and_then(|one| one.to_str()) == Some("css") {
                fs::copy(&path, folder.join(path.file_name().expect("a name"))).expect("a stylesheet is copied");
            }
        }
        let steps: String = (1..=10)
            .map(|at| {
                let mark = if at == 1 { "done" } else if at == 2 { "bad" } else { "skip" };
                format!("<li class=\"{mark}\"><span>Run Smoozy&#8217;s final check</span> <span class=\"stm\">not started</span></li>")
            })
            .collect();
        let page = r#"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
</head><body>
<div class="setup"><div class="suscroll"><section class="sus"><div class="sucard">
<div class="sustat"><span class="sdot" data-dot="busy"></span><span class="stx"><b>Setup stopped part-way</b></span></div>
<ol class="steps">STEPS</ol>
</div></section></div></div>
<pre id="out"></pre>
<script>
  const edge = document.querySelector(".sustat").getBoundingClientRect().left;
  const numbers = Array.from(document.querySelectorAll(".steps > li")).map((li) => {
    const drawn = getComputedStyle(li, "::before");
    const width = parseFloat(drawn.width);
    const right = li.getBoundingClientRect().left;
    return { content: drawn.content, left: right - width, right, width };
  });
  document.getElementById("out").textContent = JSON.stringify({ edge, numbers });
</script></body></html>"#
            .replace("STEPS", &steps);
        fs::write(folder.join("index.html"), page).expect("the page");
        let dumped = folder.join("measured.html");
        let mut command = std::process::Command::new(&chrome);
        command.args([
            "--headless",
            "--disable-gpu",
            "--no-first-run",
            "--hide-scrollbars",
            "--allow-file-access-from-files",
            "--virtual-time-budget=3000",
            "--window-size=1280,900",
            "--dump-dom",
        ]);
        command.arg(format!("--user-data-dir={}", folder.join("profile").display()));
        let mut chrome = command
            .arg(format!("file://{}", folder.join("index.html").display()))
            .stdout(fs::File::create(&dumped).expect("the dump is written"))
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Chrome starts");
        // A fixed deadline, never an open wait.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
        while chrome.try_wait().expect("Chrome is watched").is_none() {
            if std::time::Instant::now() > deadline {
                let _ = chrome.kill();
                let _ = chrome.wait();
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        let dump = fs::read_to_string(&dumped).unwrap_or_default();
        let _ = fs::remove_dir_all(&folder);
        let said = dump
            .split("<pre id=\"out\">")
            .nth(1)
            .and_then(|rest| rest.split("</pre>").next())
            .filter(|said| !said.is_empty())
            .unwrap_or_else(|| panic!("the page drew no measurement: {dump}"));
        let said: serde_json::Value = serde_json::from_str(said).expect("one JSON answer");
        let edge = said["edge"].as_f64().expect("the card's edge");
        let numbers = said["numbers"].as_array().expect("the numbers");
        assert_eq!(numbers.len(), 10, "{said}");
        let first_right = numbers[0]["right"].as_f64().expect("a right edge");
        for (at, number) in numbers.iter().enumerate() {
            let (Some(left), Some(right), Some(width)) =
                (number["left"].as_f64(), number["right"].as_f64(), number["width"].as_f64())
            else {
                panic!("step {} has no drawn number: {said}", at + 1);
            };
            assert!(width > 0.0, "step {}'s number has no width: {said}", at + 1);
            assert!(left >= edge - 0.5, "step {}'s number sticks out of the card: {said}", at + 1);
            assert!((right - first_right).abs() < 0.5, "step {}'s number does not end where the others do: {said}", at + 1);
        }
        let width = |at: usize| numbers[at]["width"].as_f64().unwrap_or_default();
        assert!(width(9) > width(8), "*10.* is no wider than *9.*, so nothing two digits wide was measured: {said}");
    }

    /// The first Chrome this computer has, where it has one.
    fn a_chrome() -> Option<std::path::PathBuf> {
        [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/usr/bin/google-chrome",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        ]
        .into_iter()
        .map(std::path::PathBuf::from)
        .find(|one| one.exists())
    }

    /// **Every step mark on the card and in the run is italic and a step
    /// heavier (weight 500), in its colour, so it stands apart from the step's
    /// name by shape as well** (`Design/Server Setup § This server`, the
    /// Creator's note 1 on their second notes, 2026-10-04). **The drawing's own
    /// stylesheet carries the rule**, so nothing is added here: this reads the
    /// style where the page gets it. `.stm` alone stays upright, so the boxes
    /// that borrow it (the busy-server box) are unchanged, and **no stylesheet
    /// the app loads after the drawing's sets a mark's shape back**.
    #[test]
    fn the_step_marks_are_italic_and_a_step_heavier() {
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        let read = |name: &str| fs::read_to_string(web.join(name)).expect("a stylesheet is readable");
        // Each rule as (its selectors, its body), comments left out.
        let rules_of = |css: &str| -> Vec<(Vec<String>, String)> {
            let mut bare = String::new();
            let mut rest = css;
            while let Some(at) = rest.find("/*") {
                bare.push_str(&rest[..at]);
                rest = rest[at..].find("*/").map_or("", |end| &rest[at + end + 2..]);
            }
            bare.push_str(rest);
            bare.split('}')
                .filter_map(|rule| rule.split_once('{'))
                .map(|(selectors, body)| {
                    let selectors = selectors.rsplit('{').next().unwrap_or(selectors);
                    let selectors = selectors.split(',').map(|one| one.split_whitespace().collect::<Vec<_>>().join(" ")).collect();
                    (selectors, body.split_whitespace().collect::<Vec<_>>().join(" "))
                })
                .collect()
        };
        let drawing = rules_of(&read("the-drawing.css"));
        let the_rule_for = |selector: &str| -> Vec<String> {
            drawing.iter().filter(|(selectors, _)| selectors.iter().any(|it| it == selector)).map(|(_, body)| body.clone()).collect()
        };
        for selector in [".sudid .stm", ".steps .stm"] {
            let bodies = the_rule_for(selector);
            assert!(
                bodies.iter().any(|body| body.contains("font-style: italic") && body.contains("font-weight: 500")),
                "{selector}: {bodies:?}"
            );
        }
        let alone = the_rule_for(".stm");
        assert!(
            alone.iter().any(|body| body.contains("font-style: normal") && body.contains("font-weight: 400")),
            "{alone:?}"
        );

        // **The page puts the card's marks in `.sudid` and the run's in `.steps`.**
        let page = the_page();
        assert!(page.contains("list.className = \"sudid\";"), "the card's list");
        assert!(page.contains("steps.className = \"steps\";"), "the run's list");

        // **Nothing loaded after the drawing undoes it.**
        let head = read("smoozy.css");
        let imported: Vec<&str> = head
            .lines()
            .filter_map(|line| line.trim().strip_prefix("@import \"")?.strip_suffix("\";"))
            .collect();
        assert_eq!(imported.first(), Some(&"the-drawing.css"), "{imported:?}");
        assert!(imported.contains(&"set-up.css"), "{imported:?}");
        for name in imported.iter().skip(1).copied().chain(["smoozy.css"]) {
            for (selectors, body) in rules_of(&read(name)) {
                let a_mark = selectors.iter().any(|it| it.contains(".stm"));
                assert!(
                    !(a_mark && (body.contains("font-style") || body.contains("font-weight"))),
                    "{name} sets a mark's shape: {selectors:?} {{ {body} }}"
                );
            }
        }
    }

    /// **The card's list: each step mark carries its meaning's colour** —
    /// *done* green, *nothing to do* grey, *not checked …* amber — and the
    /// lines that are not a step's mark stay plain (`Design/Server Setup §
    /// This server`, the Creator's note 3 on the test copy).
    #[test]
    fn the_cards_marks_carry_their_meanings_colour() {
        let colour_of = |card: &TheCard, mark: &str| -> Vec<String> {
            card.what_was_done
                .iter()
                .filter(|line| line.mark.as_ref().is_some_and(|it| *it == Prose::new(mark)))
                .map(|line| line.colour.as_str().to_string())
                .collect()
        };

        // A first setup: every step did something.
        let card = the_card_from(&a_reading());
        let done = colour_of(&card, words::DONE);
        assert!(done.len() > 1, "{:?}", card.what_was_done);
        assert!(done.iter().all(|it| it == the_marks_colour::DONE), "{done:?}");

        // A second setup with no Blender on the machine: nothing to do, and
        // the library step not checked.
        let mut reading = a_reading();
        reading.the_records = vec![a_finished_run(WhatTheStepDid::NothingToDo)];
        reading.the_library_half_was_read = false;
        let card = the_card_from(&reading);
        let idle = colour_of(&card, words::NOTHING_TO_DO);
        assert!(!idle.is_empty() && idle.iter().all(|it| it == the_marks_colour::NOTHING_TO_DO), "{idle:?}");
        assert_eq!(colour_of(&card, words::NOT_CHECKED_NO_BLENDER), vec![the_marks_colour::NOT_CHECKED]);
        // **The final check ran, so it reads *done*, and green.**
        assert_eq!(colour_of(&card, words::DONE), vec![the_marks_colour::DONE]);
        // **Telegram's line is the role's own reading and carries no mark** —
        // drawn where the steps do not list Telegram (the twenty-first notes,
        // note 1).
        let mut without = reading.clone();
        for record in without.the_records.iter_mut() {
            record.the_boundaries_reached.retain(|it| it.boundary != Boundary::Telegram);
        }
        let shown = the_card_from(&without);
        let telegram = shown.what_was_done.iter().find(|line| line.said == plainly(words::TELEGRAM_INSTALLED));
        assert_eq!(telegram.map(|line| line.colour.as_str()), Some(the_marks_colour::PLAIN));

        // **The Telegram offer's *the server is not restarted* is not a step's
        // mark** and stays plain, as the drawing leaves it.
        reading.telegram_reporting_is_installed = false;
        let card = the_card_from(&reading);
        let offer = card.what_was_done.iter().find(|line| line.missing).expect("the offer line");
        assert_eq!(offer.colour.as_str(), the_marks_colour::PLAIN);
    }

    /// **The run's list: the steps done are green, the step where it stopped
    /// is red, and a step not started or under way stays plain** — `su-fail`
    /// and `su-run` (`Design/Server Setup § This server`).
    #[test]
    fn the_runs_marks_carry_their_meanings_colour() {
        let record = TheSetupRecord {
            kind: SetupKind::TheReadyAction,
            the_composed_list: vec![
                Boundary::Started,
                Boundary::Libraries,
                Boundary::DriverPlaced,
                Boundary::RestartIssued,
                Boundary::CardAnswering,
                Boundary::Done,
            ],
            the_boundaries_reached: vec![
                a_boundary(Boundary::Started, WhatTheStepDid::Performed),
                a_boundary(Boundary::Libraries, WhatTheStepDid::NothingToDo),
                a_boundary(Boundary::DriverPlaced, WhatTheStepDid::Performed),
            ],
            ending: Some(HowTheRunEnded::EndedAtBoundaryNotCompleted { at: Some(Boundary::DriverPlaced) }),
        };
        let colours = |panel: &TheRunPanel| -> Vec<String> {
            panel.steps.iter().map(|it| it.colour.as_str().to_string()).collect()
        };

        // Cut off: done, nothing to do, done, stopped here, then not started.
        let stopped = the_run_panel(&record, Some(Boundary::RestartIssued), "RunPod 4090", false, true);
        assert_eq!(
            colours(&stopped),
            vec![
                the_marks_colour::DONE,
                the_marks_colour::NOTHING_TO_DO,
                the_marks_colour::DONE,
                the_marks_colour::FAILED,
                the_marks_colour::PLAIN,
                the_marks_colour::PLAIN,
            ]
        );
        // **A run that failed at a step colours that step red too.**
        let mut failed = record.clone();
        failed.ending = Some(HowTheRunEnded::AStepFailed {
            step: smoozy_server_setup::ready_action::Step::Driver,
            words: TheFailedStepsWords::None,
        });
        let failed = the_run_panel(&failed, Some(Boundary::RestartIssued), "RunPod 4090", false, true);
        let red: Vec<&AStepLine> =
            failed.steps.iter().filter(|it| it.colour.as_str() == the_marks_colour::FAILED).collect();
        assert_eq!(red.len(), 1);
        assert_eq!(red[0].mark, Verbatim::new("bad"));

        // Under way: the steps done keep their colour, and the one going and
        // those still to come are plain.
        let mut going = record.clone();
        going.ending = None;
        let going = the_run_panel(&going, Some(Boundary::RestartIssued), "RunPod 4090", true, true);
        assert_eq!(
            colours(&going),
            vec![
                the_marks_colour::DONE,
                the_marks_colour::NOTHING_TO_DO,
                the_marks_colour::DONE,
                the_marks_colour::PLAIN,
                the_marks_colour::PLAIN,
                the_marks_colour::PLAIN,
            ]
        );
        // **A step under way that is not the restart** takes its own branch,
        // and it is plain too.
        let mut early = record.clone();
        early.ending = None;
        early.the_boundaries_reached.truncate(1);
        let early = the_run_panel(&early, Some(Boundary::Libraries), "RunPod 4090", true, true);
        assert_eq!(early.steps[1].mark, Verbatim::new("now"));
        assert_eq!(
            colours(&early),
            vec![
                the_marks_colour::DONE,
                the_marks_colour::PLAIN,
                the_marks_colour::PLAIN,
                the_marks_colour::PLAIN,
                the_marks_colour::PLAIN,
                the_marks_colour::PLAIN,
            ]
        );
    }

    /// **`su-going` — the action goes underway on itself and is never offered
    /// twice.**
    ///
    /// **And the moment is a moment and never an elapsed time.** The drawing
    /// writes *started 2 minutes ago*; that is the machine's clock subtracted
    /// from the desktop's, which `State, identity and recovery` D5 forbids.
    #[test]
    fn a_setup_already_running_puts_the_action_underway_and_names_a_moment() {
        let mut reading = a_reading();
        reading.a_setup_is_running = Some(a_finished_run(WhatTheStepDid::Performed));
        let card = the_card_from(&reading);
        assert_eq!(card.state, Prose::new(words::SETTING_UP));
        assert!(card.the_action.underway);
        assert!(!card.the_action.loud, "nothing has gone wrong, so there is no red and no loud");
        let when = card.when.expect("a moment");
        assert!(when.reading.taken_at.is_some());
        assert_eq!(when.reading.on_the_clock, "RunPod 4090");
    }

    /// **Step 10c's sixth pass: the card named its clock by the machine's
    /// installation identity** — *checked … on the clock of ad67db67…*. The
    /// reading the tab draws from now takes the server's name as the list
    /// draws it, and where none can be read no clock is named at all.
    #[test]
    fn the_cards_clock_is_the_servers_name_and_never_an_identity() {
        let mut reading = a_reading();
        reading.a_setup_is_running = Some(a_finished_run(WhatTheStepDid::Performed));
        reading.the_clock = String::new();
        let when = the_card_from(&reading).when.expect("a moment");
        assert_eq!(when.reading.on_the_clock, "");
        assert_eq!(when.on_the_clock.as_str(), "", "no words before a name that is not there");

        let source = include_str!("set_up.rs");
        let reading_it = source
            .split("fn what_the_machine_said<R: tauri::Runtime>(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the reading the tab draws from");
        assert!(
            reading_it.contains("read.the_clock = the_servers_name(app, server).unwrap_or_default();"),
            "the tab draws the run's own clock name, which is the machine's identity"
        );
    }

    /// **`su-fail` and `su-step` — the three marks, and the failure shape on
    /// the step that failed.**
    #[test]
    fn a_cut_off_run_marks_where_it_stopped_and_what_was_never_started() {
        let record = TheSetupRecord {
            kind: SetupKind::TheReadyAction,
            the_composed_list: vec![
                Boundary::Started,
                Boundary::Libraries,
                Boundary::DriverPlaced,
                Boundary::RestartIssued,
                Boundary::CardAnswering,
                Boundary::Telegram,
                Boundary::Done,
            ],
            the_boundaries_reached: vec![
                a_boundary(Boundary::Started, WhatTheStepDid::Performed),
                a_boundary(Boundary::Libraries, WhatTheStepDid::Performed),
                a_boundary(Boundary::DriverPlaced, WhatTheStepDid::Performed),
            ],
            ending: Some(HowTheRunEnded::EndedAtBoundaryNotCompleted {
                at: Some(Boundary::DriverPlaced),
            }),
        };
        let panel = the_run_panel(&record, Some(Boundary::RestartIssued), "RunPod 4090", false, true);
        let marks: Vec<&str> = panel.steps.iter().map(|it| it.mark.as_str()).collect();
        assert_eq!(marks, vec!["done", "done", "done", "bad", "skip", "skip", "skip"]);
        assert!(panel.the_closing_paragraph.is_some());
        assert!(panel.abandon.is_some(), "S6's second way out has a control");
        assert!(panel.set_up_again.is_some());
        assert!(panel.cancel.is_none());
        assert_eq!(panel.title, Prose::new(words::SETUP_STOPPED_PART_WAY));
        assert_eq!(panel.dot, Verbatim::new("busy"), "the drawing's bufail");
        assert!(
            panel.steps.iter().all(|it| it.the_failure.is_none()),
            "a run the connection cut carries no failure shape"
        );
    }

    /// **`su-step` — a step that ran and failed carries the one failure shape,
    /// on the step itself.**
    #[test]
    fn a_step_that_failed_carries_the_failure_shape_on_itself_and_no_cross() {
        let record = TheSetupRecord {
            kind: SetupKind::TheReadyAction,
            the_composed_list: vec![
                Boundary::Started,
                Boundary::Libraries,
                Boundary::DriverPlaced,
                Boundary::RestartIssued,
                Boundary::Done,
            ],
            the_boundaries_reached: vec![
                a_boundary(Boundary::Started, WhatTheStepDid::Performed),
                a_boundary(Boundary::Libraries, WhatTheStepDid::Performed),
            ],
            ending: Some(HowTheRunEnded::AStepFailed {
                step: smoozy_server_setup::ready_action::Step::Driver,
                words: smoozy_server_setup::recovery::TheFailedStepsWords::None,
            }),
        };
        let panel = the_run_panel(&record, Some(Boundary::DriverPlaced), "RunPod 4090", false, true);
        let failed: Vec<&AStepLine> = panel
            .steps
            .iter()
            .filter(|it| it.the_failure.is_some())
            .collect();
        assert_eq!(failed.len(), 1, "the shape sits on the step that failed and nowhere else");
        assert_eq!(failed[0].mark, Verbatim::new("bad"));
        // **And exactly one step is marked stopped-part-way**, though the
        // driver step has three boundaries and the module's own reading marks
        // every one of them. A run died in one place.
        let marks: Vec<&str> = panel.steps.iter().map(|it| it.mark.as_str()).collect();
        assert_eq!(marks.iter().filter(|it| **it == "bad").count(), 1);
        assert_eq!(marks, vec!["done", "done", "bad", "skip", "skip"]);
    }

    /// **`su-run` — a long step, with the one under way marked and stopping
    /// offered throughout.**
    #[test]
    fn a_run_in_flight_marks_the_step_under_way_and_offers_stopping() {
        let record = TheSetupRecord {
            kind: SetupKind::TheReadyAction,
            the_composed_list: vec![
                Boundary::Started,
                Boundary::Libraries,
                Boundary::DriverPlaced,
                Boundary::RestartIssued,
                Boundary::CardAnswering,
                Boundary::Telegram,
                Boundary::Done,
            ],
            the_boundaries_reached: vec![
                a_boundary(Boundary::Started, WhatTheStepDid::Performed),
                a_boundary(Boundary::Libraries, WhatTheStepDid::Performed),
                a_boundary(Boundary::DriverPlaced, WhatTheStepDid::Performed),
            ],
            ending: None,
        };
        let panel = the_run_panel(&record, Some(Boundary::RestartIssued), "RunPod 4090", true, true);
        assert_eq!(panel.title, Prose::new(words::SETTING_UP), "the drawing's burun");
        let now: Vec<&AStepLine> = panel.steps.iter().filter(|it| it.mark.as_str() == "now").collect();
        assert_eq!(now.len(), 1);
        assert_eq!(now[0].name, Prose::new(words::RESTARTING));
        assert!(now[0].note.is_some(), "a restart already begun cannot be called back");
        assert_eq!(panel.cancel, Some(Prose::new(words::CANCEL_SETUP)), "Cancel setup, offered throughout");
        assert_eq!(panel.what_cancel_leaves, Some(Prose::new(words::CANCEL_SETUP_LEAVES)));
        assert!(!panel.the_state_is_underway, "the step carries the mark");
        assert_eq!(
            panel.when_it_started.as_ref().map(|it| it.lead.clone()),
            Some(Prose::new(words::STARTED))
        );
        assert!(panel.the_closing_paragraph.is_none());
        // **The record alone says nothing about a render** — eighth-pass fix
        // piece 2, 2026-09-27. Until then this read `is_some()`, which was the
        // fault: the sentence drawn on every running panel whose list carried a
        // restart. See the test below.
        assert!(panel.about_the_render.is_none());
        // **Steps still to come carry no mark and no words** — the drawing's
        // `su-run` draws them bare. Added 2026-09-27, fix piece 9.
        let marks: Vec<&str> = panel.steps.iter().map(|it| it.mark.as_str()).collect();
        assert_eq!(marks, vec!["done", "done", "done", "now", "", "", ""]);
        assert!(panel.steps[4..].iter().all(|it| it.said.is_none()));
        assert!(panel.abandon.is_none() && panel.set_up_again.is_none());
    }

    /// **`Server Setup` R1b's swap run, a run nobody pressed for, reads
    /// *Restarting the server* on the card at its restart** — the three things
    /// the fourth piece's account said it owes: it registers under the ready action's
    /// own kind, which the card names; the tab is told it is under way; and
    /// it takes `Server Setup` T4's mark the way the ready action does. Added
    /// 2026-10-12.
    #[test]
    fn the_swap_run_shows_restarting_on_the_card() {
        use smoozy_server_setup::ready_action::Step;
        let composed = smoozy_server_setup::run::compose_the_boundary_list(&[
            Step::Readings,
            Step::Driver,
            Step::Restart,
            Step::Done,
        ]);
        let record = TheSetupRecord {
            kind: SetupKind::TheReadyAction,
            the_composed_list: composed,
            the_boundaries_reached: vec![
                a_boundary(Boundary::Started, WhatTheStepDid::Performed),
                a_boundary(Boundary::DriverPlaced, WhatTheStepDid::Performed),
            ],
            ending: None,
        };
        let panel = the_run_panel(&record, Some(Boundary::RestartIssued), "Hyperstack H100", true, true);
        let now: Vec<&AStepLine> = panel.steps.iter().filter(|it| it.mark.as_str() == "now").collect();
        assert_eq!(now.len(), 1);
        assert_eq!(now[0].name, Prose::new(words::RESTARTING));

        let working = include_str!("../servers/the_driver_swap.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the working half");
        let started = working
            .split("fn the_swap_run_is_started<R: tauri::Runtime>(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the start");
        let begins = started
            .find("a_run_nobody_pressed_for_begins(&server, smoozy_server_setup::run::SetupKind::TheReadyAction)")
            .expect("the run registers under the ready action's kind, keeping the setup's words");
        let told = started
            .find("crate::surface::set_up::the_run_under_way_is_told(&app, &server);")
            .expect("the tab is told the run is under way");
        assert!(begins < told);
        assert!(started.contains("crate::setting_up::a_run_ends(&server);"), "the run is let go");
        let the_run = include_str!("../setting_up.rs")
            .split("pub fn the_swap_run_on<R: tauri::Runtime>(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the swap run");
        assert!(the_run.contains("periods.the_restart_mark_of(server)"), "T4's mark is not taken");
        assert!(the_run.contains("the_driver_miss_is_kept(&record, server, &came_to);"), "its miss is not kept");
    }

    /// **The step list draws the run's own last step, and the card's list of
    /// what was done to the machine does not.**
    ///
    /// **Both halves matter and they were one until 2026-09-23.** The final
    /// check changes nothing on the machine, so it owes the card no line; it is
    /// a step the run either reached or did not, so the panel owes it one.
    /// Step 10c's fifth pass on Vast watched a Blender install fail at exactly
    /// that step and list five steps done with nothing said about the sixth.
    #[test]
    fn the_final_verification_is_a_line_of_the_step_list_and_not_of_the_card() {
        let record = a_finished_run(WhatTheStepDid::Performed);
        let panel = the_run_panel(&record, None, "RunPod 4090", false, true);
        assert_eq!(panel.steps.len(), record.the_composed_list.len());
        assert_eq!(
            panel.steps.last().expect("a last step").name,
            Prose::new(words::DONE_STEP)
        );

        // **A run that failed at the final check says so on that step**, with
        // the failure shape on it and the sentence the box already uses.
        let stopped = TheSetupRecord {
            kind: SetupKind::ABlenderInstall,
            the_composed_list: vec![
                Boundary::Started,
                Boundary::Fetched,
                Boundary::Tooling,
                Boundary::Libraries,
                Boundary::Blender,
                Boundary::Done,
            ],
            the_boundaries_reached: vec![
                a_boundary(Boundary::Started, WhatTheStepDid::Performed),
                a_boundary(Boundary::Fetched, WhatTheStepDid::Performed),
                a_boundary(Boundary::Tooling, WhatTheStepDid::Performed),
                a_boundary(Boundary::Libraries, WhatTheStepDid::Performed),
                a_boundary(Boundary::Blender, WhatTheStepDid::Performed),
            ],
            ending: Some(HowTheRunEnded::AStepFailed {
                step: smoozy_server_setup::ready_action::Step::Done,
                words: smoozy_server_setup::recovery::TheFailedStepsWords::None,
            }),
        };
        let panel = the_run_panel(&stopped, Some(Boundary::Done), "Vast 1650", false, true);
        let marks: Vec<&str> = panel.steps.iter().map(|it| it.mark.as_str()).collect();
        assert_eq!(marks, vec!["done", "done", "done", "done", "done", "bad"]);
        let failed = panel.steps.last().expect("a last step");
        assert_eq!(
            failed.the_failure.as_ref().expect("the failure shape").said,
            plainly(words::FAILED_FINAL_CHECK)
        );
    }

    /// **`su-rm` — the removal warning names the jobs and tells each of them
    /// the truth about itself.**
    #[test]
    fn the_removal_warning_tells_each_live_job_the_truth_about_itself() {
        let rendering = ALiveJob {
            state: LiveState::Rendering,
            recorded_build_path: "/opt/blender/5.1.0/blender".to_string(),
        };
        let waiting = ALiveJob {
            state: LiveState::Waiting,
            recorded_build_path: "/opt/blender/5.1.0/blender".to_string(),
        };
        let warning = the_removal_warning(
            "/opt/blender/5.1.0/blender",
            Some(Version {
                major: 5,
                minor: 1,
                patch: 0,
            }),
            &[
                ("Kitchen — final".to_string(), rendering),
                ("Kitchen — clay".to_string(), waiting),
            ],
            None,
        );
        assert_eq!(warning.the_live_jobs.len(), 2);
        assert_eq!(
            warning.the_live_jobs[0].said,
            Prose::new(words::RENDERING_WILL_FINISH)
        );
        assert_eq!(
            warning.the_live_jobs[1].said,
            Prose::new(words::WAITING_NOTHING_TO_RENDER)
        );
        assert_eq!(warning.title.values[0], Verbatim::new("5.1.0"));
        assert_eq!(
            warning.what_is_left,
            Prose::new(words::SETTINGS_FOLDER_STAYS)
        );
    }

    /// **The 38's piece 14c: *Remove anyway* sends the removal run with the
    /// executable and the covered jobs** — the very set the warning named,
    /// composed once (`Queue & History` L21) — and a build outside Smoozy's
    /// own Blender area still answers that the act is not built.
    #[test]
    fn remove_anyway_sends_the_removal_with_the_executable_and_the_covered_jobs() {
        use smoozy_server_protocol::framing::Fields;
        use smoozy_server_protocol::requests as asked;
        use smoozy_state::identities::{Identity, IdentityKind};
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let area = "/root/smoozy/blender";
        let executable = "/root/smoozy/blender/blender-4.6.0-1790000000/blender";
        let mut rendering = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen — final");
        let mut waiting = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen — clay");
        let mut elsewhere = crate::record::deletions::tests::a_record_in_history(&server, "Hall");
        rendering.the_render_start_has_been_learned = true;
        waiting.the_render_start_has_been_learned = false;
        elsewhere.the_render_start_has_been_learned = false;
        rendering.blender.pick.build_path = executable.to_string();
        waiting.blender.pick.build_path = executable.to_string();
        elsewhere.blender.pick.build_path = "/opt/blender/blender".to_string();
        let live = the_live_jobs_of(&[rendering.clone(), waiting.clone(), elsewhere.clone()]);
        assert_eq!(live.len(), 3, "a live job was read as ended");

        let version = Some(Version { major: 4, minor: 6, patch: 0 });
        let (warning, covered) = the_removal_warning_and_its_set(executable, version, &live);
        assert_eq!(warning.the_live_jobs.len(), 2);
        assert_eq!(warning.the_live_jobs[0].said, Prose::new(words::RENDERING_WILL_FINISH));
        assert_eq!(warning.the_live_jobs[1].said, Prose::new(words::WAITING_NOTHING_TO_RENDER));
        assert_eq!(covered, vec![rendering.job.clone(), waiting.job.clone()], "the set marked is not the set named");

        let warned = crate::setting_up::TheRemovalAsked {
            executable: executable.to_string(),
            the_answered_version: "4.6.0".to_string(),
            covered: covered.clone(),
            the_managers_package: None,
        };
        let asked_for = what_remove_anyway_asks_for(executable, Some(area), Some(warned.clone()))
            .expect("a build under the Blender area is removed");
        assert_eq!(asked_for, warned);
        let fields = crate::setting_up::the_removal_steps_fields(Fields::none(), &asked_for);
        assert_eq!(fields.text(asked::THE_EXECUTABLE_FIELD), Some(executable));
        assert_eq!(fields.text(asked::THE_BLENDER_VERSION_FIELD), Some("4.6.0"));
        assert_eq!(fields.text(asked::HOW_MANY_COVERED_JOBS_FIELD), Some("2"));
        for (at, job) in covered.iter().enumerate() {
            assert_eq!(
                fields.text(&asked::the_field_of_item(asked::A_COVERED_JOB, at, asked::THE_JOB_FIELD)),
                Some(job.as_text().as_str())
            );
        }

        // **Outside Smoozy's own area, or with no area recorded, nothing is
        // asked for** and the press keeps its not-built answer. *(The snap's
        // own path left this list on 2026-10-16: its manager's route is built,
        // step 10c's owed piece 6.)*
        for outside in ["/usr/bin/blender", "/opt/blender/blender", area, "/root/smoozy/blender/../x/blender"] {
            assert_eq!(what_remove_anyway_asks_for(outside, Some(area), None), None, "{outside}");
        }
        assert_eq!(what_remove_anyway_asks_for(executable, None, None), None);

        // **The press drives the removal run**, composed of `Server Setup`'s
        // own list, and its step carries what was asked for.
        let source = include_str!("set_up.rs");
        assert!(source.contains("(REMOVE_A_BLENDER, _) => a_removal_is_driven(app, &queue, server, path)"));
        let driving = include_str!("../setting_up.rs");
        assert!(driving.contains("SetupKind::ABlenderRemoval,\n        ready_action::compose_a_blender_removal(),"));
        assert!(driving.contains("fields = the_removal_steps_fields(fields, removal);"));
        // **The mark is the manager's route's, and nothing on the plain
        // deletion** (step 10c's owed piece 6).
        assert!(driving.contains("self.a_worker_step(step, asked::THE_BUILD_IS_REMOVED_STEP, asked_for, mark)"));
    }

    /// **Step 10c's owed piece 6: *Remove anyway* on a row the machine's own
    /// software manager holds asks for that manager's removal**, and no longer
    /// answers that the act is not built. The warning is the manager route's —
    /// a rendering job is told its fate is not known on this route, never that
    /// it will finish — and the set it names is the set the removal carries,
    /// the snap's other path included. **The step carries the package**, so the
    /// machine reads the removal rather than deleting anything. A row at the
    /// distribution's own path still asks for nothing.
    #[test]
    fn remove_anyway_on_a_managed_row_asks_for_the_managers_removal() {
        use smoozy_server_protocol::framing::Fields;
        use smoozy_server_protocol::requests as asked;
        use smoozy_state::identities::{Identity, IdentityKind};
        let server = Identity::issue(IdentityKind::LocalServer).unwrap();
        let area = "/root/smoozy/blender";
        let executable = "/snap/bin/blender";
        let mut rendering = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen — final");
        let mut waiting = crate::record::deletions::tests::a_record_in_history(&server, "Kitchen — clay");
        let mut elsewhere = crate::record::deletions::tests::a_record_in_history(&server, "Hall");
        rendering.the_render_start_has_been_learned = true;
        waiting.the_render_start_has_been_learned = false;
        elsewhere.the_render_start_has_been_learned = false;
        rendering.blender.pick.build_path = executable.to_string();
        // **The snap's own folder is the same package**, so a job submitted
        // with it is covered too.
        waiting.blender.pick.build_path = "/snap/blender/7599/blender".to_string();
        elsewhere.blender.pick.build_path = "/usr/bin/blender".to_string();
        let live = the_live_jobs_of(&[rendering.clone(), waiting.clone(), elsewhere.clone()]);

        let version = Some(Version { major: 5, minor: 2, patch: 0 });
        let (warning, covered) = the_removal_warning_and_its_set(executable, version, &live);
        assert_eq!(warning.the_live_jobs.len(), 2);
        assert_eq!(warning.the_live_jobs[0].said, Prose::new(words::RENDERING_UNKNOWN));
        assert_eq!(warning.the_live_jobs[1].said, Prose::new(words::WAITING_NOTHING_TO_RENDER));
        assert!(warning.remove.is_some() && warning.smoozy_cannot_perform_it.is_none());
        assert_eq!(covered, vec![rendering.job.clone(), waiting.job.clone()], "the set marked is not the set named");

        let warned = crate::setting_up::TheRemovalAsked {
            executable: executable.to_string(),
            the_answered_version: "5.2.0".to_string(),
            covered: covered.clone(),
            the_managers_package: Some("blender"),
        };
        let asked_for = what_remove_anyway_asks_for(executable, Some(area), Some(warned.clone()))
            .expect("a managed row asks for its manager's removal");
        assert_eq!(asked_for, warned);
        // **Even with no Blender area recorded**: the manager's route needs none.
        assert_eq!(
            what_remove_anyway_asks_for(executable, None, None).map(|it| it.the_managers_package),
            Some(Some("blender"))
        );
        let fields = crate::setting_up::the_removal_steps_fields(Fields::none(), &asked_for);
        assert_eq!(fields.text(asked::THE_MANAGERS_PACKAGE_FIELD), Some("blender"));
        assert_eq!(fields.text(asked::THE_EXECUTABLE_FIELD), Some(executable));
        assert_eq!(fields.text(asked::HOW_MANY_COVERED_JOBS_FIELD), Some("2"));
        // **The plain deletion carries no package.**
        let plain = what_remove_anyway_asks_for("/root/smoozy/blender/b-1/blender", Some(area), None)
            .expect("a build under the area");
        assert_eq!(plain.the_managers_package, None);
        assert_eq!(
            crate::setting_up::the_removal_steps_fields(Fields::none(), &plain)
                .text(asked::THE_MANAGERS_PACKAGE_FIELD),
            None
        );
        assert_eq!(what_remove_anyway_asks_for("/usr/bin/blender", Some(area), None), None);
    }

    /// **B14: the manager route's own sentence, which does not carry the
    /// folder route's observation.**
    #[test]
    fn the_manager_route_does_not_borrow_the_folder_routes_observation() {
        let rendering = ALiveJob {
            state: LiveState::Rendering,
            recorded_build_path: "/snap/blender/current/blender".to_string(),
        };
        let warning = the_removal_warning(
            "/snap/blender/current/blender",
            None,
            &[("Kitchen — final".to_string(), rendering)],
            Some("blender"),
        );
        assert_eq!(
            warning.the_live_jobs[0].said,
            Prose::new(words::RENDERING_UNKNOWN)
        );
        // **The one package the fixed set holds is removable**, so this route
        // still offers the control.
        assert!(warning.remove.is_some());
        assert!(warning.smoozy_cannot_perform_it.is_none());
    }

    /// **`Server Setup` §13 item 21: a managed Blender under a package name
    /// outside the fixed set has no admissible removal carriage.**
    ///
    /// The entry's own answer is that *"the removal is reported as one this
    /// module cannot perform, and the user's own manager is the route"* — and
    /// the reason it cannot is `Server protocol and transfers` 1.1's ban on
    /// echoing a machine-read name into a command string. **The box says so and
    /// offers no control**: one that is pressed and then refused should not have
    /// been pressable.
    #[test]
    fn a_managed_build_smoozy_has_no_carriage_for_says_so_and_offers_no_control() {
        let warning = the_removal_warning(
            "/opt/vendor/blender-lts/blender",
            None,
            &[],
            Some("some-vendors-blender"),
        );
        assert!(warning.remove.is_none());
        assert_eq!(
            warning.smoozy_cannot_perform_it,
            Some(Prose::new(words::THE_MANAGER_IS_THE_ROUTE))
        );
    }

    /// **Removing the last Blender is not warned about on its own account** —
    /// somebody taking the only Blender off is doing it on purpose.
    #[test]
    fn a_removal_with_no_live_job_names_nothing_and_still_says_what_is_left() {
        let warning = the_removal_warning(
            "/opt/blender/5.1.0/blender",
            Some(Version {
                major: 5,
                minor: 1,
                patch: 0,
            }),
            &[],
            None,
        );
        assert!(warning.the_live_jobs.is_empty());
        assert!(warning.how_many.is_none());
        assert_eq!(warning.what_is_left, Prose::new(words::SETTINGS_FOLDER_STAYS));
    }

    /// **`su-elev`, `pw-nokeep`, `pw-again` — the three readings of the
    /// admin-access box.**
    ///
    /// **How many choices it offers is `Security and licence` S36's and is
    /// never fixed here.**
    #[test]
    fn the_admin_access_box_has_three_readings_and_two_choice_counts() {
        let ordinary = the_box("Installing the GPU driver", Some(StoreStanding::HasAnswered), false);
        assert_eq!(ordinary.choices.len(), 3);
        assert!(ordinary.password.is_some());
        assert!(ordinary.the_password_could_not_be_read.is_none());
        assert_eq!(
            ordinary.which_step.values[0],
            Verbatim::new("Installing the GPU driver")
        );

        let nowhere = the_box("Installing the GPU driver", Some(StoreStanding::NeverAnswered), false);
        assert_eq!(nowhere.choices.len(), 2, "S36: two where the store never answered");
        assert!(
            nowhere.choices.iter().all(|it| it.which.as_str() != "remember-it"),
            "the withheld choice is the one that would have kept a password across restarts"
        );

        let again = the_box("Installing the GPU driver", Some(StoreStanding::HasAnswered), true);
        assert!(again.the_password_could_not_be_read.is_some());
        assert!(again.password.is_none(), "this shape asks for nothing");
        assert!(again.choices.is_empty());
    }

    /// **A standing nobody could read is not a fresh installation** — must-do
    /// 32. No row is *never probed*; a failed read, or a value this reader does
    /// not know, is `None`, and the box then withholds *Remember it* and says
    /// why.
    #[test]
    fn a_standing_that_could_not_be_read_is_not_taken_for_a_fresh_installation() {
        assert_eq!(the_standing_from(Err(rusqlite::Error::QueryReturnedNoRows)), Some(StoreStanding::NeverProbed));
        assert_eq!(the_standing_from(Ok("has answered".to_string())), Some(StoreStanding::HasAnswered));
        assert_eq!(the_standing_from(Err(rusqlite::Error::InvalidQuery)), None);
        assert_eq!(the_standing_from(Ok("something else".to_string())), None);
        let unread = the_box("Installing the GPU driver", None, false);
        assert_eq!(unread.choices.len(), 2);
        assert!(unread.choices.iter().all(|it| it.which.as_str() != "remember-it"));
        assert!(unread.the_standing_could_not_be_read.is_some());
        let read = the_box("Installing the GPU driver", Some(StoreStanding::NeverProbed), false);
        assert!(read.the_standing_could_not_be_read.is_none());
        assert_eq!(read.choices.len(), 3);
        assert!(the_page().contains("asked.the_standing_could_not_be_read"));
    }

    /// **The work warning before a setup names a transfer it would interrupt**
    /// — `Server Setup` W3's first row, with the sentence that was never used
    /// (must-do 31).
    #[test]
    fn the_work_warning_names_a_transfer_that_would_be_interrupted() {
        let warning = the_ready_actions_warning(
            &[("Kitchen — final".to_string(), LiveState::Rendering)],
            &["kitchen_v14.blend".to_string()],
        );
        assert_eq!(warning.the_live_jobs.len(), 2);
        assert_eq!(warning.the_live_jobs[1].job, Verbatim::new("kitchen_v14.blend"));
        assert_eq!(warning.the_live_jobs[1].said, Prose::new(words::UPLOADING_AND_A_RESTART));
    }

    /// **The *cannot do this yet* box closes with *Close*, and its answer names
    /// its server** (must-dos 29 and 30), and **the sentence is the one every
    /// piece says** (must-do 19).
    #[test]
    fn the_answer_names_its_server_and_its_one_control_is_close() {
        let event = format!("{:?}", the_answer_for("server-1"));
        assert!(event.contains("server-1"));
        assert!(event.contains(&format!("{:?}", Prose::new(words::CLOSE))));
        assert!(event.contains(&format!("{:?}", Prose::new(crate::surface::files::A_CONNECTION_IS_NOT_BUILT))));
        let page = the_page();
        assert!(page.contains("answer.server !== current"), "an answer about another server is drawn here");
        // **Since 2026-10-02 the answer is one line on the card and never a
        // box** (the Creator's notes piece 2), so its *Close* is the words
        // panel's alone.
        let handler = page
            .split("whenTold(THE_ANSWER, (delivery) => {")
            .nth(1)
            .and_then(|rest| rest.split("\n});\n").next())
            .expect("the answer's handler");
        assert!(!handler.contains("aBox()"), "an answer opens a box");
        assert!(page.contains("close: word.close,"));
        assert!(!page.contains("said.the_card.the_action.name, () => theBoxIsClosed()"));
    }

    /// **The folder link opens Files at that folder, and the Transfers link
    /// goes to Transfers** — `Server Setup` B12a and the links into Transfers
    /// (must-do 30). Neither is answered as an act any more.
    #[test]
    fn the_folder_and_transfers_links_go_where_they_say() {
        let page = the_page();
        assert!(page.contains("import { theBrowserIsOpenedAt } from \"./files.js\""));
        assert!(page.contains("theBrowserIsOpenedAt(current, build.folder_bytes)"));
        assert!(page.contains("theCurrentTabIs(FILES)"));
        assert!(page.contains("theCurrentPlaceIs(TRANSFERS)"));
        assert!(!page.contains("theActIsAskedFor(\"browse\""));
        assert!(!page.contains("theActIsAskedFor(\"transfers\""));
        let files = include_str!("../../web/files.js");
        assert!(files.contains("export function theBrowserIsOpenedAt("));
    }

    /// **Fault 6 of the second pass, 2026-09-21: every *What the server said*
    /// panel this tab opens hands the panel its *Close*** — the drawing's own
    /// word, `Preparation/Design/Mockup/Smoozy.html` line 3112, and
    /// `Design/Server Setup § The step that failed`'s *"The panel's Close only
    /// closes the panel"*. Until then the control was an empty pill.
    #[test]
    fn every_words_panel_this_tab_opens_carries_its_close() {
        let page = the_page();
        let opened = page.matches("theWordsPanel({").count();
        assert_eq!(opened, 3, "a panel was added or removed; hold it to this too");
        assert!(page.contains("close: word.close,"));
        assert!(page.contains("close: failed.close,"));
        // ***Server output*'s, from 2026-10-18** (the thirteenth notes,
        // piece H).
        assert!(page.contains("close: output.close,"));
        let failed = TheFailedStep {
            said: plainly(words::WHAT_THE_SERVER_SAID),
            what_the_server_said: Some(Prose::new(words::WHAT_THE_SERVER_SAID)),
            the_output: None,
            the_complaint: None,
            the_output_tab: Prose::new(Stream::TheOrdinaryOutput.the_name_of_the_stream()),
            the_complaint_tab: Prose::new(Stream::TheComplaints.the_name_of_the_stream()),
            close: Prose::new(words::CLOSE),
        };
        assert_eq!(failed.close, Prose::new(words::CLOSE));
        assert_eq!(words::CLOSE, "Close");
    }

    /// **The two tabs of a failed step's words are named for their streams**
    /// (must-do 33), in the shared panel's own words.
    #[test]
    fn the_two_stream_tabs_are_named_for_their_streams() {
        let page = the_page();
        assert!(page.contains("name: failed.the_output_tab"));
        assert!(page.contains("name: failed.the_complaint_tab"));
        assert_ne!(
            Stream::TheOrdinaryOutput.the_name_of_the_stream(),
            Stream::TheComplaints.the_name_of_the_stream()
        );
    }

    /// **The one field the box changes between callers is the step's name.**
    /// A refusal in the file browser must not open a box about this screen's
    /// step, and the step travels as a value rather than inside a sentence
    /// (X7).
    #[test]
    fn the_step_that_was_refused_travels_as_a_value() {
        let box_ = the_box("Deleting sim_cache", Some(StoreStanding::HasAnswered), false);
        assert_eq!(box_.which_step.values[0], Verbatim::new("Deleting sim_cache"));
        assert!(!box_.which_step.rest.as_ref().unwrap().as_str().contains("sim_cache"));
    }

    /// **`su-add` — the install list, in two parts with a rule between them.**
    #[test]
    fn the_install_list_splits_the_finished_releases_from_the_unfinished_builds() {
        let listing = smoozy_server_setup::listing::TheListingAsRead {
            channels: vec![
                PublishedChannel {
                    track: "latest".to_string(),
                    risk: Risk::Stable,
                    version: Version { major: 5, minor: 2, patch: 2 },
                    released_at: String::new(),
                    size_bytes: 0,
                    the_build_services_file: None,
                },
                PublishedChannel {
                    track: "4.5lts".to_string(),
                    risk: Risk::Stable,
                    version: Version { major: 4, minor: 5, patch: 10 },
                    released_at: String::new(),
                    size_bytes: 0,
                    the_build_services_file: None,
                },
                PublishedChannel {
                    track: String::new(),
                    risk: Risk::Alpha,
                    version: Version { major: 5, minor: 3, patch: 0 },
                    released_at: String::new(),
                    size_bytes: 0,
                    the_build_services_file: Some("blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz".to_string()),
                },
                // **The shop-front's own unfinished channel is never offered**
                // (`Server Setup` B2's revision of 2026-10-02).
                PublishedChannel {
                    track: "latest".to_string(),
                    risk: Risk::Edge,
                    version: Version { major: 5, minor: 3, patch: 0 },
                    released_at: String::new(),
                    size_bytes: 0,
                    the_build_services_file: None,
                },
                PublishedChannel {
                    track: "5.2".to_string(),
                    risk: Risk::Beta,
                    version: Version { major: 5, minor: 2, patch: 4 },
                    released_at: String::new(),
                    size_bytes: 0,
                    the_build_services_file: None,
                },
                PublishedChannel {
                    track: "5.2".to_string(),
                    risk: Risk::Candidate,
                    version: Version { major: 5, minor: 2, patch: 3 },
                    released_at: String::new(),
                    size_bytes: 0,
                    the_build_services_file: None,
                },
            ],
            entries_that_could_not_be_read: vec!["one entry".to_string()],
        };
        let list = the_list(
            Ok(listing),
            vec![Version { major: 4, minor: 5, patch: 9 }],
        );
        assert_eq!(list.finished.len(), 2);
        assert_eq!(list.unfinished.len(), 3);
        assert_eq!(list.finished[0].line, Prose::new(words::LATEST_STABLE));
        assert_eq!(list.finished[1].line, Prose::new(words::LONG_TERM_SUPPORT));
        assert_eq!(list.unfinished[0].line, Prose::new(words::ALPHA), "Blender's own word, the Creator's note 5");
        assert_eq!(list.unfinished[1].line, Prose::new(words::BETA));
        assert_eq!(list.unfinished[2].line, Prose::new(words::RELEASE_CANDIDATE));
        // **What the install carries**: the version, and the build service's
        // bare file name beside it for an unfinished build from there.
        assert_eq!(list.unfinished[0].asked, Verbatim::new(format!("5.3.0 {}", "blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz")));
        assert_eq!(list.finished[0].asked, Verbatim::new("5.2.2"));
        // **B6a's mark: a version comparison and never a build comparison.**
        assert_eq!(
            list.finished[1].mark.as_ref().expect("a mark").values[0],
            Verbatim::new("4.5.9")
        );
        // **An entry that could not be read is said rather than dropped
        // quietly.**
        assert!(list.short.is_some());
    }

    /// **A build the server could not fetch is named, by version and kind** —
    /// `Design/Server Setup § Installing`: *Could not download Blender 5.3.0
    /// Alpha.* The Creator's note 7 on 0.1.7: the step read *the Blender
    /// build* and named nothing. **And the page hands back what the install
    /// carries, never the version alone.**
    #[test]
    fn a_fetch_that_failed_names_the_build_and_the_page_sends_what_the_install_carries() {
        use crate::setting_up::{TheRunsEnding, WhereTheSetUpGot};
        let came_to = WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Started,
            ending: TheRunsEnding::AStepFailed {
                step: smoozy_server_setup::ready_action::Step::Fetch,
                said: "Could not read the published checksum: the checksum file returned HTTP 404".to_string(),
            },
        };
        let told = what_the_install_came_to(&came_to, "5.3.0 Alpha");
        assert_eq!(told.said, "Install stopped. Could not download Blender 5.3.0 Alpha.");
        assert!(told.the_machines_own_words.as_deref().is_some_and(|it| it.contains("404")));
        // **Named as the list named it**, and by the version alone where no list
        // is held.
        let carried = format!("5.3.0 {}", "blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz");
        assert_eq!(the_build_asked_for_is_named("5.2.2"), "5.2.2");
        let mut reading = smoozy_server_setup::listing::TheListingAsRead {
            channels: vec![PublishedChannel {
                track: String::new(),
                risk: Risk::Alpha,
                version: Version { major: 5, minor: 3, patch: 0 },
                released_at: String::new(),
                size_bytes: 0,
                the_build_services_file: Some("blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz".to_string()),
            }],
            entries_that_could_not_be_read: Vec::new(),
        };
        the_entries_offered_are_held(&the_list(Ok(reading.clone()), Vec::new()));
        assert_eq!(the_build_asked_for_is_named(&carried), "5.3.0 Alpha");
        reading.channels.clear();
        the_entries_offered_are_held(&the_list(Ok(reading), Vec::new()));
        assert_eq!(the_build_asked_for_is_named(&carried), "5.3.0");
        assert!(the_page().contains("picked[0].asked,"));
    }

    /// **The list that could not be read says so, and the service's own words
    /// are not drawn.**
    #[test]
    fn a_listing_that_could_not_be_read_says_so_without_the_services_words() {
        let list = the_list(
            Err(WhyTheListingCouldNotBeRead::TheShopFrontAnswered(503)),
            Vec::new(),
        );
        assert!(list.finished.is_empty());
        assert!(list.unfinished.is_empty());
        assert_eq!(
            list.instead.expect("something is said").lead,
            Prose::new(words::THE_LIST_COULD_NOT_BE_READ)
        );
    }

    /// **`su-low` — the below-floor warning, with the version a value in every
    /// sentence that names it.**
    #[test]
    fn the_below_the_floor_warning_carries_the_version_as_a_value_three_times() {
        let warning = the_below_the_floor_warning(Version { major: 4, minor: 2, patch: 9 });
        assert_eq!(warning.title.values[0], Verbatim::new("4.2.9"));
        assert_eq!(warning.supported.values[0], Verbatim::new("4.5"));
        assert_eq!(warning.nothing_is_written_for.values[0], Verbatim::new("4.2.9"));
        assert!(
            !warning.title.lead.as_str().contains("4.2.9"),
            "X7: a version never travels inside a sentence the point has touched"
        );
        assert!(warning.dont_install_it.as_str().contains("Don"));
    }

    /// **`su-busy` and `su-unpack` — `Server Setup` W3's two warning rows, and
    /// they say different things because they are different acts.**
    #[test]
    fn the_two_warnings_differ_because_the_unpack_route_restarts_nothing() {
        let live = vec![
            ("Kitchen — final".to_string(), LiveState::Rendering),
            ("Bedroom test".to_string(), LiveState::Waiting),
        ];
        let ready = the_ready_actions_warning(&live, &[]);
        let unpack = the_unpack_tools_warning(&live);
        assert_eq!(ready.title, Prose::new(words::THIS_SERVER_HAS_WORK));
        assert_eq!(unpack.title, Prose::new(words::UNPACK_TOOL_FIRST));
        assert_eq!(
            ready.the_live_jobs[0].said,
            Prose::new(words::RENDERING_AND_A_RESTART)
        );
        assert_eq!(unpack.the_live_jobs[0].said, Prose::new(words::RENDERING));
        assert!(
            !unpack.the_live_jobs[0].said.as_str().contains("restart"),
            "nothing on the install path restarts the server, so the box claims nothing about \
             what the change does to the work"
        );
    }

    /// **`Server Setup` W3's table is closed at seven rows and three of them
    /// warn, and this piece draws a warning for exactly those it owns.**
    ///
    /// **The silence is as binding as the warning** — warning where nothing can
    /// be disturbed is what teaches the user to click through the one that
    /// matters.
    #[test]
    fn this_piece_warns_on_the_two_rows_it_owns_and_is_silent_on_the_rest() {
        use smoozy_server_setup::statements::WarnableStep;
        assert!(WarnableStep::AReadyActionContainingDriverWorkOrARestart.warns());
        assert!(WarnableStep::ABlenderInstallWhereTheUnpackToolMustBeInstalled.warns());
        assert!(WarnableStep::ABlenderRemovalALiveJobsBuildMatches.warns());
        assert!(!WarnableStep::ABlenderInstallOnAMachineThatAlreadyHoldsTheUnpackTool.warns());
        assert!(!WarnableStep::TelegramMaterialOrADesiredStateChange.warns());
    }

    /// **The Blender table's three columns, its two tags and its folder
    /// link.**
    #[test]
    fn the_table_carries_the_origin_tag_the_floor_tag_and_the_folder() {
        let mut reading = a_reading();
        reading.the_builds = vec![
            ABuildAsRead {
                path: "/opt/blender/4.5.9/blender".to_string(),
                version: Version { major: 4, minor: 5, patch: 9 },
                long_term_support: true,
                origin: Some(HowItGotThere::SmoozyInstalledItFromTheList),
                the_managers_package: None,
            },
            ABuildAsRead {
                path: "/opt/blender/custom/blender".to_string(),
                version: Version { major: 4, minor: 2, patch: 9 },
                long_term_support: false,
                origin: Some(HowItGotThere::TheUserPointedAtIt),
                the_managers_package: None,
            },
            ABuildAsRead {
                path: "/snap/blender/current/blender".to_string(),
                version: Version { major: 4, minor: 5, patch: 2 },
                long_term_support: false,
                origin: Some(HowItGotThere::NoInstallRecordNamesIt),
                the_managers_package: Some("blender".to_string()),
            },
        ];
        let table = the_table_from(&reading);
        assert_eq!(table.rows[0].version, Verbatim::new("4.5.9 LTS"));
        assert!(table.rows[0].tag.is_none(), "Smoozy's own install carries no tag");
        assert_eq!(table.rows[1].tag, Some(Prose::new(words::YOUR_BUILD)));
        assert_eq!(
            table.rows[1].below_the_floor,
            Some(Prose::new(words::BELOW_THE_FLOOR))
        );
        assert_eq!(
            table.rows[2].tag,
            Some(Prose::new(words::NOT_INSTALLED_BY_SMOOZY))
        );
        // **B12a: the folder is derived from the path, per build and never per
        // version**, and the link is on every row including the ones it cannot
        // help with.
        assert_eq!(table.rows[2].folder, Verbatim::new("/snap/blender/current"));
        assert!(table.rows.iter().all(|row| !row.folder.as_str().is_empty()));
    }

    /// **The tag says the origin and never the version** — two builds
    /// answering one version are two builds with two paths (B13).
    #[test]
    fn nothing_in_the_table_is_keyed_to_a_version_number() {
        let mut reading = a_reading();
        reading.the_builds = vec![
            ABuildAsRead {
                path: "/opt/blender/5.2.1/blender".to_string(),
                version: Version { major: 5, minor: 2, patch: 1 },
                long_term_support: false,
                origin: Some(HowItGotThere::SmoozyInstalledItFromTheList),
                the_managers_package: None,
            },
            ABuildAsRead {
                path: "/home/artist/mine/blender".to_string(),
                version: Version { major: 5, minor: 2, patch: 1 },
                long_term_support: false,
                origin: Some(HowItGotThere::TheUserPointedAtIt),
                the_managers_package: None,
            },
        ];
        let table = the_table_from(&reading);
        assert_eq!(table.rows[0].version, table.rows[1].version);
        assert_ne!(table.rows[0].tag, table.rows[1].tag);
        assert_ne!(table.rows[0].folder, table.rows[1].folder);
    }

    /// **A read that has never happened says so rather than drawing an empty
    /// machine** — L8a, and `Design/Server Setup § Cut off part-way`'s one
    /// server that shows nothing about a setup.
    #[test]
    fn a_machine_nothing_was_ever_read_from_says_so_on_both_halves() {
        let tab = the_tab(WhatTheMachineSaid::NothingHasEverBeenReadFromIt, "server-1");
        assert_eq!(tab.the_card.state, Prose::new(words::NEVER_READ));
        assert!(tab.the_card.instead.is_some());
        assert!(tab.the_table.instead.is_some());
        assert!(tab.the_table.rows.is_empty());
        // **The action is never greyed out** — the user may want it for a
        // reason Smoozy cannot see.
        assert!(!tab.the_card.the_action.name.as_str().is_empty());
        assert!(
            !tab.the_card.the_action.loud,
            "a machine nothing was read from is not one Smoozy can name a reason about"
        );
    }

    /// **This is what the command actually answers today**, said in a test so
    /// nobody has to take the head's word for it.
    #[test]
    fn the_command_answers_that_nothing_has_ever_been_read() {
        // **Connected and nothing found to read is not *never read*.** Until
        // 2026-09-21 the command answered *never read* for every server
        // whatever stood — the first pass on a real rental found the card
        // saying *Smoozy has not read this server yet* on a connected server.
        let connected = the_tab(WhatTheMachineSaid::ConnectedAndNoRunWasFound, "server-1");
        assert_ne!(connected.the_card.state, Prose::new(words::NEVER_READ));
        assert_eq!(connected.the_card.state, Prose::new(words::NO_SETUP_WAS_READ));
        assert!(!connected.the_card.the_action.loud, "nothing read is no reason to be loud");
        let never = the_tab(WhatTheMachineSaid::NothingHasEverBeenReadFromIt, "server-1");
        assert_eq!(never.the_card.state, Prose::new(words::NEVER_READ));
    }

    /// **Fault 3 of the fifth pass on Vast, 2026-09-23: the Blender table can
    /// show a Blender.**
    ///
    /// **The search feeds the table whatever the card's own reading is** — two
    /// builds on a machine whose setup run stopped part-way are still two
    /// builds — and B11's count of folders it could not enter is said beside
    /// them. **A search nobody took says a reading does not exist**, which is
    /// L8a and is not the same news as *there is no Blender*.
    #[test]
    fn the_blender_table_draws_what_the_search_found() {
        let found = TheBlenderSearch::from_the_ask(
            smoozy_render_settings::acting::versions::TheVersionListAsk::Answered(
                smoozy_server_setup::search::TheSearchResult {
                    entries: vec![
                        smoozy_server_setup::search::ACandidate {
                            path: "/root/smoozy/blender/a-blender-5.2.2/blender".to_string(),
                            answered: Version::parse("5.2.2"),
                        },
                        smoozy_server_setup::search::ACandidate {
                            path: "/root/smoozy/blender/b-blender-5.1.2/blender".to_string(),
                            answered: Version::parse("5.1.2"),
                        },
                        // **A candidate that did not answer is not a build**,
                        // and B11 names it rather than the table drawing it.
                        smoozy_server_setup::search::ACandidate {
                            path: "/usr/local/blender".to_string(),
                            answered: None,
                        },
                    ],
                    answer: smoozy_server_setup::search::WhatTheAnswerSays {
                        the_search_is_by_name: true,
                        folders_refused: 2,
                        did_not_answer: vec!["/usr/local/blender".to_string()],
                    },
                    at: std::time::SystemTime::UNIX_EPOCH,
                },
            ),
        );
        let tab = the_tab_with(
            WhatTheMachineSaid::ConnectedAndNoRunWasFound,
            "server-1",
            found,
            false,
        );
        assert_eq!(tab.the_table.rows.len(), 2);
        assert_eq!(tab.the_table.rows[0].version, Verbatim::new("5.2.2"));
        assert_eq!(
            tab.the_table.rows[0].folder,
            Verbatim::new("/root/smoozy/blender/a-blender-5.2.2")
        );
        // **No tag is claimed about how a build got there**, because no record
        // of the placing act was read.
        assert!(tab.the_table.rows[0].tag.is_none());
        assert!(tab.the_table.instead.is_none(), "a table with rows says nothing instead");
        // **B11's two refused folders and the search going by name are kept in
        // the answer and not drawn** — Creator-stated 2026-10-02, note 13 on
        // 0.1.7. Nothing under the table says how the server was searched.
        let printed = format!("{:?}", tab.the_table);
        assert!(!printed.contains("could not be read") && !printed.contains("by name"), "{printed}");
        assert!(!the_page().contains("drawn.short"), "the page draws a line under the table");

        // **And a search that was not taken draws the empty table**, with the
        // card unchanged.
        let none = the_tab_with(
            WhatTheMachineSaid::ConnectedAndNoRunWasFound,
            "server-1",
            TheBlenderSearch::from_the_ask(
                smoozy_render_settings::acting::versions::TheVersionListAsk::TheMachineCouldNotBeAsked,
            ),
            false,
        );
        assert!(none.the_table.rows.is_empty());
        assert_eq!(none.the_table.instead, Some(plainly(words::NO_BLENDER_READING)));
        assert_eq!(none.the_card.state, Prose::new(words::NO_SETUP_WAS_READ));
    }

    /// **Fault 4 of the second pass, 2026-09-21: a machine holding no
    /// installation of Smoozy's is *Not set up*, with the loud action** — what
    /// `Design/Server Setup § This server` draws for *no setup record at all*,
    /// and what a setup whose placement failed before any run leaves. **Not the
    /// undrawn *No setup read from this server***, which is for a machine that
    /// may hold a finished setup this copy cannot ask for.
    #[test]
    fn a_machine_holding_no_installation_reads_not_set_up_with_the_loud_action() {
        let tab = the_tab(WhatTheMachineSaid::ConnectedAndNeverSetUp, "server-1");
        assert_eq!(tab.the_card.state, Prose::new(words::NOT_SET_UP));
        assert!(tab.the_card.the_action.loud);
        assert_eq!(tab.the_card.the_action.name, Prose::new(words::SET_UP_THIS_SERVER));
        assert!(tab.the_card.what_was_done.is_empty());
        assert!(tab.the_card.the_note.is_none());
    }

    /// **The states are the drawing's own thirty-seven**, read
    /// out of the mockup in both directions. *(Twenty-six until 2026-10-16,
    /// owed piece 4; twenty-seven until 2026-10-18, the thirteenth notes'
    /// piece H; thirty-one until 2026-10-24, the sixteenth notes' note 1;
    /// thirty-three until the eighteenth notes' design pass the same day,
    /// whose three are owed to that work's piece 2.)*
    ///
    /// **This is the reverse leg the runner's ruling of 2026-09-12 asked for.**
    /// A state this piece names that the drawing does not have is a state
    /// nobody drew; a state the drawing has in this group that this piece does
    /// not name is one nobody built and nothing else would say so.
    #[test]
    fn the_thirty_states_are_the_drawings_own() {
        let drawing = the_mockup();
        let group = drawing
            .split("<optgroup label=\"Setting a server up\">")
            .nth(1)
            .expect("the drawing has that group")
            .split("</optgroup>")
            .next()
            .expect("the group closes");
        let mut drawn: Vec<&str> = Vec::new();
        for (at, _) in group.match_indices("<option value=\"") {
            let rest = &group[at + "<option value=\"".len()..];
            drawn.push(&rest[..rest.find('"').expect("the value closes")]);
        }
        let mut ours = THE_DRAWN_STATES.to_vec();
        drawn.sort_unstable();
        ours.sort_unstable();
        assert_eq!(drawn, ours, "the drawing's Setting a server up group has moved");
    }

    // ── `Servers` H11b and `Server Setup` B12b — the fourth piece of the
    // OptiX and driver work, 2026-10-11.

    fn the_driver(
        version: &str,
        cuda: Option<&str>,
        placed: smoozy_servers::hardware::WhereK1PlacesIt,
    ) -> smoozy_servers::hardware::TheDriversOwnAnswer {
        smoozy_servers::hardware::TheDriversOwnAnswer {
            version: smoozy_servers::hardware::DriverVersion::parse(version).expect("a driver version"),
            cuda_version: cuda.map(str::to_string),
            where_k1_placed_the_machine: placed,
        }
    }

    fn held_live(
        driver: smoozy_servers::hardware::TheDriversOwnAnswer,
        missed: bool,
    ) -> TheDriverAsHeld {
        TheDriverAsHeld {
            the_driver: Some(driver),
            the_driver_missed_the_optix_bar: missed,
            last_seen_at: None,
        }
    }

    fn a_ready_tab_with(held: &TheDriverAsHeld) -> TheSetUpTab {
        the_tab_with_the_driver(
            WhatTheMachineSaid::AReading(Box::new(a_reading())),
            "server-1",
            TheBlenderSearch::ItWasNotTaken,
            false,
            held,
        )
    }

    /// **The card's driver line** — *NVIDIA driver 595.91.07 · CUDA 13.2*,
    /// the two figures as values, on a card read now; greyed with *last seen*
    /// and this computer's moment where the reading is not this connection's
    /// own; without the CUDA half where the driver did not give it; and absent
    /// where no driver answered. `Design/Server Setup § This server`.
    #[test]
    fn the_card_carries_the_driver_line_live_and_last_seen() {
        use smoozy_servers::hardware::WhereK1PlacesIt::ItsOwnKernel;
        let tab = a_ready_tab_with(&held_live(the_driver("595.91.07", Some("13.2"), ItsOwnKernel), false));
        let line = tab.the_driver.expect("the driver line");
        assert_eq!(line.the_driver.lead, Prose::new(words::NVIDIA_DRIVER));
        assert_eq!(line.the_driver.values, vec![Verbatim::new("595.91.07")]);
        let cuda = line.the_cuda.expect("the CUDA half");
        assert_eq!(cuda.lead, Prose::new(words::CUDA_AFTER_THE_DRIVER));
        assert_eq!(cuda.values, vec![Verbatim::new("13.2")]);
        assert!(line.last_seen.is_none(), "a live reading carries no stamp");
        assert!(tab.the_driver_warning.is_none(), "a driver at every bar says nothing more");

        // **Not connected: the line still stands, greyed and stamped**, on the
        // card that says Smoozy has not read the server yet.
        let held = TheDriverAsHeld {
            last_seen_at: Some(at(1_754_000_000)),
            ..held_live(the_driver("595.91.07", None, ItsOwnKernel), false)
        };
        let tab = the_tab_with_the_driver(
            WhatTheMachineSaid::NothingHasEverBeenReadFromIt,
            "server-1",
            TheBlenderSearch::ItWasNotTaken,
            false,
            &held,
        );
        let line = tab.the_driver.expect("the line, last seen");
        assert!(line.the_cuda.is_none());
        let stamp = line.last_seen.expect("the stamp");
        assert_eq!(stamp.lead, Prose::new(crate::surface::servers::words::LAST_SEEN));
        assert_eq!(stamp.values, vec![Verbatim::new("1754000000")]);

        assert!(a_ready_tab_with(&TheDriverAsHeld::default()).the_driver.is_none());
    }

    /// **`su-drvold` — `Servers` H11b's statement on a container, in amber,
    /// with the amber dot**, and its second case, a run that missed the bar —
    /// and nothing at all on a machine Smoozy can update, on a driver at the
    /// bar, or where no driver answered.
    #[test]
    fn the_warning_is_said_on_a_container_and_after_a_missed_swap_and_nowhere_else() {
        use smoozy_servers::hardware::WhereK1PlacesIt::{AContainer, ItsOwnKernel};
        let before = a_ready_tab_with(&TheDriverAsHeld::default()).the_card.dot;
        assert_eq!(before.as_str(), "ok");

        let tab = a_ready_tab_with(&held_live(the_driver("550.163.01", Some("12.4"), AContainer), false));
        let warning = tab.the_driver_warning.expect("said on a container");
        assert_eq!(
            warning.said,
            Prose::new(
                "This server's NVIDIA driver, 550.163.01 (CUDA 12.4), is too old for OptiX in \
                 Blender 5.2 and later."
            )
        );
        assert_eq!(warning.and, Prose::new("It can't be updated here."));
        assert!(!warning.last_seen);
        assert_eq!(tab.the_card.dot.as_str(), "busy", "with the amber dot");

        let tab = a_ready_tab_with(&held_live(the_driver("570.211.01", Some("12.8"), ItsOwnKernel), true));
        let warning = tab.the_driver_warning.expect("said where a run missed the bar");
        assert_eq!(
            warning.said,
            Prose::new(
                "This server's NVIDIA driver, 570.211.01 (CUDA 12.8), is too old for OptiX in \
                 Blender 5.2.1 and later."
            )
        );
        assert_eq!(warning.and, Prose::new("It could not be updated."));
        assert_eq!(tab.the_card.dot.as_str(), "busy");

        // **`su-optixoff` — a machine Smoozy can update says nothing**: setup
        // brings the driver up, and the Blender rows carry the tag instead.
        let tab = a_ready_tab_with(&held_live(the_driver("570.211.01", Some("12.8"), ItsOwnKernel), false));
        assert!(tab.the_driver_warning.is_none());
        assert_eq!(tab.the_card.dot.as_str(), "ok");
        // **A driver at the bar is not told on a container either.**
        let tab = a_ready_tab_with(&held_live(the_driver("595.91.07", Some("13.2"), AContainer), true));
        assert!(tab.the_driver_warning.is_none());
        assert_eq!(tab.the_card.dot.as_str(), "ok");
        // **A missed bar with no driver to read says nothing.**
        let tab = a_ready_tab_with(&TheDriverAsHeld {
            the_driver_missed_the_optix_bar: true,
            ..TheDriverAsHeld::default()
        });
        assert!(tab.the_driver_warning.is_none());

        // **Last seen: drawn greyed with the line, and the dot left alone.**
        let held = TheDriverAsHeld {
            last_seen_at: Some(at(1_754_000_000)),
            ..held_live(the_driver("550.163.01", Some("12.4"), AContainer), false)
        };
        let tab = a_ready_tab_with(&held);
        assert!(tab.the_driver_warning.expect("still said").last_seen);
        assert_eq!(tab.the_card.dot.as_str(), "ok");
        // **A card that already says something is wrong keeps its red dot.**
        let mut tab = the_tab(WhatTheMachineSaid::AReading(Box::new(a_reading())), "server-1");
        tab.the_card.dot = Verbatim::new("bad");
        the_driver_is_drawn_on(
            &mut tab,
            &held_live(the_driver("550.163.01", Some("12.4"), AContainer), false),
            &[],
        );
        assert_eq!(tab.the_card.dot.as_str(), "bad");
    }

    /// **`Server Setup` B12b's tag on the Blender table — `su-optixoff`'s
    /// rows.** On a 570 driver it falls on 5.2.1 and a 5.3 build and on
    /// nothing earlier; **read live**, so it is gone from the next tab drawn
    /// once the reading finds the driver at the bar, and it is not drawn from
    /// a reading that is only last seen.
    #[test]
    fn the_tag_falls_on_the_rows_below_their_bar_and_only_beside_a_live_reading() {
        use smoozy_servers::hardware::WhereK1PlacesIt::ItsOwnKernel;
        let a_build = |path: &str, version: &str| ABuildAsRead {
            path: path.to_string(),
            version: Version::parse(version).expect("a version"),
            long_term_support: false,
            origin: None,
            the_managers_package: None,
        };
        let found = || TheBlenderSearch::ItFound {
            builds: vec![
                a_build("/opt/blender/4.5.3/blender", "4.5.3"),
                a_build("/opt/blender/5.1.1/blender", "5.1.1"),
                a_build("/opt/blender/5.2.1/blender", "5.2.1"),
                a_build("/opt/blender/5.3.0/blender", "5.3.0"),
            ],
            folders_the_search_could_not_enter: 0,
        };
        let tagged = |tab: &TheSetUpTab| -> Vec<String> {
            tab.the_table
                .rows
                .iter()
                .filter(|row| row.driver_too_old == Some(Prose::new(words::DRIVER_TOO_OLD_FOR_OPTIX)))
                .map(|row| row.version.as_str().to_string())
                .collect()
        };
        let a_tab = |held: &TheDriverAsHeld| {
            the_tab_with_the_driver(
                WhatTheMachineSaid::ConnectedAndNoRunWasFound,
                "server-1",
                found(),
                false,
                held,
            )
        };
        let old = held_live(the_driver("570.211.01", Some("12.8"), ItsOwnKernel), false);
        assert_eq!(tagged(&a_tab(&old)), vec!["5.2.1", "5.3.0"]);
        // **After the swap**, the next reading answers at the bar.
        let new = held_live(the_driver("595.91.07", Some("13.2"), ItsOwnKernel), false);
        assert!(tagged(&a_tab(&new)).is_empty());
        let last_seen = TheDriverAsHeld {
            last_seen_at: Some(at(1_754_000_000)),
            ..old.clone()
        };
        assert!(tagged(&a_tab(&last_seen)).is_empty());
        assert!(tagged(&a_tab(&TheDriverAsHeld::default())).is_empty());
        // **A full reading's own builds carry it too**, where no search was
        // taken.
        assert_eq!(tagged(&a_ready_tab_with(&old)), vec!["5.2.1"]);
    }

    /// **What the tab is handed of the driver follows `Servers`' own judgement
    /// of the row's reading**: the driver and no stamp where it is live, the
    /// driver and this computer's moment of it where it is last seen, nothing
    /// where no reading may be shown — and the missed bar the row holds,
    /// whichever.
    #[test]
    fn the_driver_is_handed_over_in_the_state_the_rows_reading_stands_in() {
        use smoozy_servers::hardware::{HardwareReading, HardwareState, MachineMemory, Cards};
        use smoozy_servers::hardware::WhereK1PlacesIt::AContainer;
        let mut row = smoozy_servers::store::ServerRow::saved(
            "server-1",
            smoozy_servers::identity::Host::what_the_user_typed("5.180.183.77").expect("a host"),
            smoozy_servers::identity::Port::what_the_user_typed(33510).expect("a port"),
            smoozy_servers::identity::Username::what_the_user_typed("root").expect("a username"),
            smoozy_servers::identity::KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
        );
        let driver = the_driver("550.163.01", Some("12.4"), AContainer);
        row.remembered_hardware_reading = Some(
            HardwareReading::taken(
                Cards::NoCard,
                MachineMemory::CouldNotBeEstablished,
                None,
                at(1_000),
                "server-1",
                at(1_754_000_000),
                None,
            )
            .with_the_driver(Some(driver.clone())),
        );
        row.the_driver_missed_the_optix_bar = true;
        let live = the_driver_as_the_row_holds_it(&row, HardwareState::Live);
        assert_eq!(live.the_driver, Some(driver.clone()));
        assert_eq!(live.last_seen_at, None);
        assert!(live.the_driver_missed_the_optix_bar);
        let seen = the_driver_as_the_row_holds_it(&row, HardwareState::LastSeen);
        assert_eq!(seen.the_driver, Some(driver));
        assert_eq!(seen.last_seen_at, Some(at(1_754_000_000)));
        let never = the_driver_as_the_row_holds_it(&row, HardwareState::NeverRead);
        assert_eq!(never.the_driver, None);
    }

    /// **The page draws all three, from the application's own values** — the
    /// line, the warning and the tag — and writes none of their words.
    #[test]
    fn the_page_draws_the_driver_line_the_warning_and_the_tag() {
        let page = the_page();
        assert!(page.contains("theDriverLine(answer.the_driver)"));
        assert!(page.contains("answer.the_driver_warning"));
        assert!(page.contains("build.driver_too_old"));
        for word in [
            words::NVIDIA_DRIVER,
            words::DRIVER_TOO_OLD_FOR_OPTIX,
            words::SMOOZY_CANT_UPDATE_IT_HERE,
            words::SMOOZY_COULDNT_UPDATE_IT,
        ] {
            assert!(!page.contains(word), "{word} is written on the page");
        }
    }

    /// **`su-tg`: the not-installed line carries *Install it*, the installed
    /// line carries nothing, and the page hands the act back** — `Design/Server
    /// Setup § Adding Telegram to a server already set up`. Replaces the test
    /// that failed if the control was written, step 10 piece 8.
    #[test]
    fn the_telegram_line_carries_install_it_only_where_it_is_owed() {
        let mut reading = a_reading();
        reading.telegram_reporting_is_installed = false;
        reading.telegram_is_switched_on = true;
        let card = the_card_from(&reading);
        let line = card.what_was_done.iter().find(|line| line.missing).expect("the not-installed line");
        assert_eq!(line.offer, Some(ALinesOffer { name: Prose::new(words::INSTALL_IT), act: INSTALL_TELEGRAM }));
        reading.telegram_reporting_is_installed = true;
        assert!(the_card_from(&reading).what_was_done.iter().all(|line| line.offer.is_none()));
        reading.telegram_reporting_is_installed = false;
        reading.telegram_is_switched_on = false;
        assert!(the_card_from(&reading).what_was_done.iter().all(|line| !line.missing));
        assert!(the_page().contains(&format!("\"{INSTALL_TELEGRAM}\"")), "the page never hands the act back");
        assert!(
            !the_working_half().contains("smoozy_server_setup::telegram"),
            "`App/Server Setup/src/telegram.rs` is that module's placement of Telegram's \
             material and is not reached from this page"
        );
    }

    /// **Settings' switch reaches the card through the tab, and nowhere
    /// else** — `Design/Server Setup § Adding Telegram…`: *"Telegram is on in
    /// Settings and this server has not got it"*. Off, nothing is offered; and
    /// **a card last seen offers nothing**, because *Install it* acts on the
    /// machine and there is no connection to act over. The 38's piece 11.
    #[test]
    fn settings_switch_reaches_the_card_and_a_last_seen_card_offers_nothing() {
        let mut reading = a_reading();
        reading.telegram_reporting_is_installed = false;
        reading.telegram_is_switched_on = false;
        let offered = |tab: &TheSetUpTab| {
            tab.the_card
                .what_was_done
                .iter()
                .any(|line| line.offer.as_ref().is_some_and(|offer| offer.act == INSTALL_TELEGRAM))
        };
        let read = || WhatTheMachineSaid::AReading(Box::new(reading.clone()));
        assert!(offered(&the_tab_with(read(), "server-1", TheBlenderSearch::ItWasNotTaken, true)));
        assert!(!offered(&the_tab_with(read(), "server-1", TheBlenderSearch::ItWasNotTaken, false)));
        let last_seen = WhatTheMachineSaid::LastSeen(Box::new(read()), std::time::SystemTime::UNIX_EPOCH);
        assert!(!offered(&the_tab_with(last_seen, "server-1", TheBlenderSearch::ItWasNotTaken, true)));

        // **A set-up server's card, drawn from its run and the machine's
        // reading**: offered where the role reads off, not where it reads on,
        // and **not where the role was not read at all** — *not installed*
        // would then be a claim nobody read.
        let set_up = |role: Option<bool>| {
            WhatTheMachineSaid::ASetupRunAndTheMachine(
                Box::new(the_finished_run_as_read()),
                Box::new(the_rest_with(None, None, role)),
            )
        };
        assert!(offered(&the_tab_with(set_up(Some(false)), "server-1", TheBlenderSearch::ItWasNotTaken, true)));
        assert!(!offered(&the_tab_with(set_up(Some(false)), "server-1", TheBlenderSearch::ItWasNotTaken, false)));
        assert!(!offered(&the_tab_with(set_up(Some(true)), "server-1", TheBlenderSearch::ItWasNotTaken, true)));
        let the_run_alone = WhatTheMachineSaid::ASetupRunAlone(Box::new(the_finished_run_as_read()));
        assert!(!offered(&the_tab_with(the_run_alone, "server-1", TheBlenderSearch::ItWasNotTaken, true)));
    }

    #[test]
    fn recheck_a_removed_blender_goes_before_the_new_search_returns() {
        let path = "/home/user/smoozy/blender/5.2.2/blender";
        let reached = crate::setting_up::WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Done,
            ending: crate::setting_up::TheRunsEnding::ItReachedDone {
                a_blender_was_published: false, the_driver_missed_the_optix_bar: false, the_swap_waits: false,
            },
        };
        let ending = delivered(a_removals_ending("recheck-removal-page", path, &reached));
        assert_eq!(ending.said.get("the_removed_blender"), Some(&Said::Value(Verbatim::new(path))));
        let stopped = delivered(a_removals_ending("recheck-removal-page", path, &crate::setting_up::WhereTheSetUpGot::TheConnectionWent));
        assert!(!stopped.said.contains_key("the_removed_blender"), "an unanswered removal claimed the build gone");
        let found = TheBlenderSearch::ItFound {
            builds: vec![ABuildAsRead {
                path: path.to_string(), version: Version { major: 5, minor: 2, patch: 2 }, origin: None,
                long_term_support: false, the_managers_package: None,
            }], folders_the_search_could_not_enter: 0,
        };
        let tab = the_tab_with(WhatTheMachineSaid::ConnectedAndNoRunWasFound, "recheck-removal-page", found, false);
        let harness = THE_RUN_ON_THE_CARD_HARNESS.split("const running = THE_RUNNING;").next().unwrap().to_string()
            + &r#"const tab = TAB;
let first = true;
let oldReading;
globalThis.window = { __TAURI__: { core: { invoke(command) {
  if (command === "the_set_up_tab") {
    if (first) { first = false; return Promise.resolve(tab); }
    return new Promise((done) => { oldReading = done; });
  }
  return Promise.resolve(null);
} } } };
const settle = async () => { for (let at = 0; at < 6; at += 1) await new Promise((done) => setImmediate(done)); };
const tell = (said) => (globalThis.told.get("set-up:the-answer") || []).forEach((one) => one({ said }));
await import("./set-up.js");
const host = anElement("div"); globalThis.tab.itIsDrawn(host); globalThis.tab.itIsShowing("recheck-removal-page");
await settle();
const versions = () => every(host).filter((one) => one.className === "burow").map(theTextOf);
const before = versions();
// A reading already in flight must not bring the removed build back.
tell({ server: "recheck-removal-page", the_act_is_not_built_yet: "yes" }); await settle();
const previous = oldReading;
tell(ENDING);
await settle(); const after = versions();
previous(tab); await settle();
console.log(JSON.stringify({ before, after, late: versions() }));"#
                .replace("TAB", &serde_json::to_string(&tab).unwrap())
                .replace("ENDING", &serde_json::to_string(&ending.said).unwrap());
        let folder = std::env::temp_dir().join(format!("smoozy-recheck-removal-page-{}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::copy(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js"), folder.join("set-up.js")).unwrap();
        for (name, text) in THE_SET_UP_STAND_INS {
            std::fs::write(folder.join(name), if name == "harness.js" { &harness } else { text }).unwrap();
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output().expect("Node runs the page");
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "{}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).unwrap();
        assert_eq!(out["before"].as_array().unwrap().len(), 1, "{out}");
        assert_eq!(out["after"], serde_json::json!([]), "the removed build stayed while the search waited: {out}");
        assert_eq!(out["late"], serde_json::json!([]), "an older reading restored the removed build: {out}");
    }

    /// **The worldwide pass's fault 3: *Ready to render* never shows where the
    /// search found no Blender.** Added 2026-10-26. On World A, *Set up this
    /// server* after a refused add installed no Blender — the add's choice is
    /// not remembered once the machine was reached (`Servers` B2 and `Servers` B5, the
    /// Servers spec's *"nothing about the setup is remembered"*), so that is
    /// right — and the finished card then read *Ready to render* over a Blender
    /// section that read *No Blender has been read from this server.*
    #[test]
    fn worldwide_fault_3_a_finished_setup_reads_set_up_and_not_ready_to_render_where_no_blender_was_found() {
        let set_up = || {
            WhatTheMachineSaid::ASetupRunAndTheMachine(
                Box::new(the_finished_run_as_read()),
                Box::new(the_rest_with(None, None, Some(false))),
            )
        };
        let nothing_found = TheBlenderSearch::ItFound {
            builds: Vec::new(),
            folders_the_search_could_not_enter: 0,
        };
        let a_build_found = TheBlenderSearch::ItFound {
            builds: vec![ABuildAsRead {
                path: "/home/user/smoozy/blender/5.2.2/blender".to_string(),
                version: Version::parse("5.2.2").expect("a version"),
                long_term_support: false,
                origin: None,
                the_managers_package: None,
            }],
            folders_the_search_could_not_enter: 0,
        };
        let none = the_tab_with(set_up(), "server-1", nothing_found, false);
        assert_eq!(none.the_card.state, Prose::new(words::SET_UP_STATE));
        assert_ne!(none.the_card.state, Prose::new(words::READY_TO_RENDER));
        // **Nothing else on the card moves**: the steps still say what the run
        // did, and the table says no Blender was read.
        assert!(!none.the_card.what_was_done.is_empty());
        assert_eq!(none.the_table.instead, Some(plainly(words::NO_BLENDER_READING)));
        // **Where a Blender was found, *Ready to render* stands.**
        let one = the_tab_with(set_up(), "server-1", a_build_found, false);
        assert_eq!(one.the_card.state, Prose::new(words::READY_TO_RENDER));
        // **Where no search was taken, nothing is known either way**, and the
        // card is as it was.
        let unknown = the_tab_with(set_up(), "server-1", TheBlenderSearch::ItWasNotTaken, false);
        assert_eq!(unknown.the_card.state, Prose::new(words::READY_TO_RENDER));
        // **And a card that did not say ready is left alone.**
        let stopped = the_tab_with(
            WhatTheMachineSaid::ConnectedAndNeverSetUp,
            "server-1",
            TheBlenderSearch::ItFound {
                builds: Vec::new(),
                folders_the_search_could_not_enter: 0,
            },
            false,
        );
        assert_eq!(stopped.the_card.state, Prose::new(words::NOT_SET_UP));
    }

    /// **What the Telegram placement says** (Rule 9): *done* is said by the
    /// card and by the one short sentence; its own step failing names it. The
    /// 38's piece 11.
    #[test]
    fn the_telegram_placement_says_what_it_came_to() {
        use crate::setting_up::{TheRunsEnding, WhereTheSetUpGot};
        let done = WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Done,
            ending: TheRunsEnding::ItReachedDone { a_blender_was_published: false, the_driver_missed_the_optix_bar: false, the_swap_waits: false },
        };
        assert_eq!(what_the_telegram_placement_came_to(&done).said, "Telegram reporting installed.");
        assert!(the_card_says_the_ready_actions_ending(&done));
        let failed = WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Started,
            ending: TheRunsEnding::AStepFailed {
                step: smoozy_server_setup::ready_action::Step::TelegramMaterial,
                said: "no such folder".to_string(),
            },
        };
        let came = what_the_telegram_placement_came_to(&failed);
        assert_eq!(came.said, words::FAILED_TELEGRAM);
        assert_eq!(came.the_machines_own_words.as_deref(), Some("no such folder"));
    }

    /// **Pressing *Install it* runs `Server Setup` G2's dedicated entry point**
    /// and never answers that the act is not built. Wired 2026-10-04; until
    /// then the act fell to the catch-all arm, which says the one sentence for
    /// an act nobody has built.
    #[test]
    fn install_it_runs_the_telegram_placement() {
        let acts = the_working_half()
            .split("pub fn a_set_up_act_that_needs_a_connection<R: tauri::Runtime>(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the acts");
        assert!(
            acts.contains("(INSTALL_TELEGRAM, _) => the_telegram_placement_is_driven(app, &queue, server),"),
            "*Install it* is not driven"
        );
        let driven = the_working_half()
            .split("fn the_telegram_placement_is_driven<R: tauri::Runtime>(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the driving");
        // **From 2026-10-27 it is an act of the line** (`Server Setup` Y9a),
        // performed where every act is.
        assert!(driven.contains("crate::setting_up::ASetUpAct::TheTelegramPlacement"));
        let performed = the_working_half()
            .split("fn the_act_is_performed<R: tauri::Runtime>(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the act performed");
        assert!(performed.contains("crate::setting_up::the_telegram_placement_on(app, &point, server)"));
    }

    /// **The 38's piece 14b: `Server Setup` B7a's question, drawn on the run
    /// panel while it stands, and both answers reach the run that waits** —
    /// the page draws `su-low` from the panel and sends the two acts. **Undone
    /// (the panel not carrying it, or an answer going nowhere), this fails.**
    #[test]
    fn the_question_below_the_floor_is_drawn_and_both_answers_reach_the_run() {
        for (server, install_anyway) in [("server-b7a-yes", true), ("server-b7a-no", false)] {
            assert_eq!(the_question_on(server), None, "a question stood before one was asked");
            let mut answered = crate::setting_up::the_question_is_asked(server, "4.2.0");
            let question = the_question_on(server).expect("the question is drawn while it stands");
            assert_eq!(question, the_below_the_floor_warning(Version { major: 4, minor: 2, patch: 0 }));
            assert!(crate::setting_up::the_question_is_answered(server, install_anyway));
            assert_eq!(answered.try_recv(), Ok(install_anyway), "the answer did not reach the run");
            assert_eq!(the_question_on(server), None, "the question outlived its answer");
            assert!(!crate::setting_up::the_question_is_answered(server, install_anyway));
        }
        let page = the_page();
        assert!(page.contains("theBelowTheFloorWarningIsDrawn(panel.the_question, server)"));
        assert!(page.contains("answered(DONT_INSTALL_IT)") && page.contains("answered(INSTALL_ANYWAY)"));
        assert!(page.contains("theActIsAskedFor(CHOOSE_A_BUILD, server)"));
    }

    /// **The 38's piece 14b: the table says *your build* for a build whose
    /// install record says so**, read beside the search at connection — and
    /// the published listing's route stays untagged, and a build no record
    /// names claims nothing. **Undone, this fails.**
    #[test]
    fn the_table_says_your_build_for_a_record_that_says_so() {
        let a_build = |path: &str| ABuildAsRead {
            path: path.to_string(),
            version: Version { major: 4, minor: 6, patch: 0 },
            long_term_support: false,
            origin: None,
            the_managers_package: None,
        };
        let found = TheBlenderSearch::ItFound {
            builds: vec![a_build("/s/blender-a/blender"), a_build("/s/blender-b/blender"), a_build("/opt/blender")],
            folders_the_search_could_not_enter: 0,
        };
        let records = vec![
            ("/s/blender-a/blender".to_string(), smoozy_server_protocol::requests::YOUR_BUILD.to_string()),
            (
                "/s/blender-b/blender".to_string(),
                smoozy_server_protocol::requests::FROM_THE_PUBLISHED_LISTING.to_string(),
            ),
        ];
        let TheBlenderSearch::ItFound { builds, .. } = found.clone().with_the_install_records(Some(&records))
        else {
            panic!("the search was lost");
        };
        let table = the_table_the_search_found(&builds);
        let tags: Vec<_> = table.rows.iter().map(|row| row.tag.clone()).collect();
        assert_eq!(tags, vec![Some(Prose::new(words::YOUR_BUILD)), None, None]);
        assert_eq!(found.clone().with_the_install_records(None), found, "nothing read changed the table");
        // **Read at connection**, beside the search, and not from a run.
        let source = include_str!("set_up.rs");
        let search = &source[source.find("fn the_blender_search_now").expect("the search")..];
        let search = &search[..search.find("\n}\n").expect("its end")];
        assert!(search.contains("crate::setting_up::the_install_records(&point, area)"));
        assert!(search.contains(".with_the_install_records(records.as_deref())"));
    }

    /// **The page names this file's one event kind and its five acts**, so
    /// neither side writes a name the other does not have.
    #[test]
    fn the_page_and_this_file_name_the_same_kind_and_the_same_acts() {
        let page = the_page();
        assert!(page.contains(THE_ANSWER));
        for act in [
            THE_READY_ACTION,
            ABANDON,
            STOP,
            INSTALL_A_BLENDER,
            REMOVE_A_BLENDER,
            CONNECT_AND_SET_UP,
            CHOOSE_A_BUILD,
            INSTALL_ANYWAY,
            DONT_INSTALL_IT,
        ] {
            assert!(
                page.contains(&format!("\"{act}\"")),
                "the application composes the act {act:?} and the page routes on nothing of \
                 that name"
            );
        }
        for command in [
            "the_set_up_tab",
            "the_install_list",
            "what_removing_a_blender_costs",
            "the_admin_access_box",
            "a_set_up_act_that_needs_a_connection",
            // **`Server Setup` E2's offer, answered** — 2026-09-21.
            "the_admin_access_is_answered",
        ] {
            assert!(
                page.contains(command),
                "web/set-up.js does not call {command}"
            );
        }
    }

    /// **Every word the user reads is in `mod words` and nowhere else.**
    ///
    /// A string written straight into a composition is a word with no home, and
    /// the next piece that needs the same word writes a second copy of it.
    #[test]
    fn every_word_is_in_one_place() {
        let source = the_working_half();
        let after = source
            .split("mod words {")
            .nth(1)
            .expect("the words are a module");
        let after = after.split("\n}\n").nth(1).expect("the words module closes");
        let mut faults = Vec::new();
        for (at, _) in after.match_indices("Prose::new(\"") {
            let rest = &after[at + "Prose::new(\"".len()..];
            let said = rest[..rest.find('"').unwrap_or(0)].to_string();
            // The empty lead of a value-first sentence is not a word.
            if said.is_empty() || said == "?" {
                continue;
            }
            faults.push(said);
        }
        assert!(
            faults.is_empty(),
            "a word is written into a composition rather than into `mod words`: {faults:#?}"
        );
    }

    /// **This piece composes no failure sentence of a failure record's, so
    /// `WhoseSentenceItIs` is not reached** — said here rather than left
    /// unmentioned.
    ///
    /// **The one failure this tab draws is a step of a setup run**, and it is
    /// drawn by the one failure shape from what the machine recorded. It raises
    /// no failure record and spends no diary capture, so M32's *"the capture
    /// belongs to whatever composes the sentence the user reads"* is answered
    /// by there being no record here at all. **A later agent that raises one
    /// from this page has to pass `TheComposersOwn`**, and this is the line
    /// that will fail when it does.
    #[test]
    fn nothing_here_reaches_the_diary_capture() {
        assert!(
            !the_working_half().contains("WhoseSentenceItIs"),
            "this piece composes a failure record's sentence. **M32 decides whether the diary \
             keeps what the user actually read**, and a screen that gets it wrong produces no \
             error, no failing test and no visible fault"
        );
    }

    /// **Nothing in this piece rebuilds a shape the shared half already
    /// exports.** The mockup draws the failure shape eleven times in five
    /// modules with five class prefixes, and a piece that copies a subtree
    /// copies one of them.
    #[test]
    fn no_shared_shape_is_rebuilt_here() {
        let page = the_page();
        for prefix in ["qfail", "svfail", "tffail", "rdfail", "cbad", "stfail"] {
            assert!(
                !page.contains(prefix),
                "web/set-up.js writes {prefix}, which is one of the mockup's copies of the one \
                 failure shape. **Use the shape that exists** — `theFailureShape` in \
                 web/the-failure-shape.js"
            );
        }
        assert!(
            page.contains("the-failure-shape.js"),
            "the step that failed takes the one failure shape and this page does not import it"
        );
    }

    /// **No inline style, no inline script, no `innerHTML` and no `data:`
    /// address** — H32, H33, and the content policy this application runs
    /// under. The mockup carries 88 inline `style=` attributes and four uses of
    /// `innerHTML`, every one of them inert here, and a piece that copies a
    /// subtree copies all of them.
    #[test]
    fn the_page_copies_none_of_the_mockups_own_faults() {
        let page = the_page();
        for forbidden in ["innerHTML", "style=\"", "data:", "outerHTML", "insertAdjacentHTML"] {
            assert!(!page.contains(forbidden), "web/set-up.js writes {forbidden}");
        }
    }

    /// **Nothing on this page repeats on a timer** — P9 and P10.
    #[test]
    fn nothing_on_the_page_polls_or_ticks() {
        let page = the_page();
        for forbidden in ["setInterval", "setTimeout", "requestAnimationFrame"] {
            assert!(
                !page.contains(forbidden),
                "web/set-up.js writes {forbidden}. **Nothing here polls and nothing repeats on \
                 a timer** (P9, P10), and a run's progress arrives as events"
            );
        }
    }

    /// **Every one of this tab's boxes carries `role="dialog"`.**
    ///
    /// **Piece 5's Escape handler steps aside for anything carrying it**, and
    /// its own account asks for exactly this: *"Whoever adds a box over a panel
    /// should give it `role="dialog"`."* A box of this tab's over an open panel
    /// with no role would be collapsed by the panel's own Escape instead of
    /// being closed.
    #[test]
    fn every_box_this_tab_opens_says_it_is_a_box() {
        let page = the_page();
        let boxes = page.matches("className = \"modal top\"").count();
        let roles = page.matches("\"dialog\"").count();
        assert!(boxes > 0, "this tab opens boxes");
        assert!(
            roles >= 1,
            "a box of this tab's carries no dialog role, so piece 5's Escape would collapse the \
             panel under it instead of closing the box"
        );
    }

    /// **Fault 4 of the first pass on a real rental, 2026-09-21: the box
    /// showed `TheMachineWouldNot("…")` with its newline and quotes escaped,
    /// and named a cause Smoozy did not know.** Tested with the exact text the
    /// machine gave that morning.
    ///
    /// **What must hold**: the sentence is Smoozy's own and says only what it
    /// knows — its program could not be installed — and the machine's words go
    /// behind *What the server said*, whole, with the newline and the quotes
    /// as the machine wrote them.
    #[test]
    fn the_failure_box_speaks_in_the_users_words_and_keeps_the_machines_apart() {
        use crate::setting_up::WhereTheSetUpGot;
        let the_rental_said = crate::setting_up::tests::WHAT_THE_RENTAL_SAID;
        let why = smoozy_server_protocol::placement::performing::WhyThePlacementCouldNotBeCarriedOut::TheMachineWouldNot(
            the_rental_said.to_string(),
        );
        let came_to = WhereTheSetUpGot::ThePlacementCouldNotBeCarriedOut {
            said: crate::setting_up::the_words_of(&why),
        };
        let told = what_it_came_to(&came_to);
        assert_eq!(
            told.said,
            format!("{} {}", words::SETUP_STOPPED, words::COULD_NOT_INSTALL_ITS_PROGRAM)
        );
        assert_eq!(told.the_machines_own_words.as_deref(), Some(the_rental_said));
        for debugging in ["TheMachineWouldNot", "(\"", "\\n", "\\\"", "supervisor"] {
            assert!(!told.said.contains(debugging), "{debugging:?} in {:?}", told.said);
        }
        // **The event carries the two apart**: the sentence as prose, the
        // machine's words as a value beside it.
        let event = format!("{:?}", the_answer_about("server-1", &told));
        assert!(event.contains("the_machines_own_words"));
        assert!(event.contains("what_the_server_said"));
    }

    /// **Step 10c, 2026-09-21: a setup or an install refused because a setup
    /// that stopped part-way holds the server says so, and says what to do** —
    /// `Design/Server Setup § Cut off part-way`'s own words, with the machine's
    /// words kept behind *What the server said*. **The second pass on
    /// Hyperstack was told only that the program would not start the setup.**
    #[test]
    fn a_refusal_by_a_setup_that_stopped_part_way_says_why_and_what_to_do() {
        let came_to = crate::setting_up::WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Started,
            ending: crate::setting_up::TheRunsEnding::ASetupThatStoppedPartWayStands {
                standing: smoozy_state::identities::Identity::issue(
                    smoozy_state::identities::IdentityKind::Operation,
                )
                .expect("randomness"),
                said: "the setup worker was not admitted: a setup of this server did not \
                       finish, and nothing else runs until it does"
                    .into(),
            },
        };
        let told = what_the_install_came_to(&came_to, "5.2.2");
        assert!(told.said.starts_with(words::INSTALL_STOPPED), "{}", told.said);
        assert!(told.said.contains("A setup stopped part-way"), "{}", told.said);
        assert!(told.said.contains("Set up again"), "{}", told.said);
        assert!(!told.said.contains(words::THE_RUN_WOULD_NOT_START));
        assert!(told
            .the_machines_own_words
            .as_deref()
            .is_some_and(|said| !said.contains("Refused(")));
    }

    /// **No ending's sentence is a debugging rendering**, whichever way a
    /// setup stops. Every value of the endings the driving can hand back is
    /// walked, and none may carry a type's name or a debugging quote.
    #[test]
    fn no_ending_is_said_in_a_debugging_rendering() {
        use crate::setting_up::{TheRunsEnding, WhereTheSetUpGot, WhichReadingRefused};
        use smoozy_server_protocol::placement::performing::WhatThePlacementReached as Reached;
        use smoozy_server_setup::ready_action::Step;
        let ended = |ending| WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Started,
            ending,
        };
        let every = vec![
            WhereTheSetUpGot::TheKindOrTheFamilyWouldNotRead {
                which: WhichReadingRefused::TheKind,
            },
            WhereTheSetUpGot::TheRentalWouldNotNameItself,
            WhereTheSetUpGot::TheHandOverDidNotLand { said: "x".into() },
            WhereTheSetUpGot::TheRootCouldNotBeRecorded { said: "x".into() },
            WhereTheSetUpGot::TheReleaseWasNotAdmitted { said: "x".into() },
            WhereTheSetUpGot::ThePlacement {
                reached: Reached::RolledBack,
            },
            WhereTheSetUpGot::ThePlacement {
                reached: Reached::FailedBeforeCommit { because: "x".into() },
            },
            WhereTheSetUpGot::ThePlacement {
                reached: Reached::Deferred { because: "x".into() },
            },
            WhereTheSetUpGot::JobsOfAnEarlierSetupAreHeld {
                how_many: 1,
                because: "x".into(),
            },
            WhereTheSetUpGot::JobsOfAnEarlierSetupAreHeld {
                how_many: 2,
                because: "x".into(),
            },
            WhereTheSetUpGot::ThePlacement {
                reached: Reached::RollingBack { because: "x".into() },
            },
            WhereTheSetUpGot::ThePlacement {
                reached: Reached::AClaimStandsWhoseOwnerCouldNotBeRead,
            },
            WhereTheSetUpGot::ThePlacementCouldNotBeCarriedOut { said: "x".into() },
            WhereTheSetUpGot::TheConnectionWent,
            WhereTheSetUpGot::TheServerIsNotSetUpYet,
            WhereTheSetUpGot::TheBlenderListCouldNotBeRead {
                said: words::THE_LIST_COULD_NOT_BE_READ.into(),
            },
            ended(TheRunsEnding::AStepFailed {
                step: Step::Driver,
                said: "x".into(),
            }),
            ended(TheRunsEnding::AStepFailed {
                step: Step::Done,
                said: "x".into(),
            }),
            ended(TheRunsEnding::TheServerStoppedAnswering {
                step: Step::Done,
                said: "x".into(),
            }),
            ended(TheRunsEnding::TheServerStoppedAnswering {
                step: Step::Libraries,
                said: "x".into(),
            }),
            ended(TheRunsEnding::NoRunCouldBeOpened { said: "x".into() }),
            ended(TheRunsEnding::ASetupThatStoppedPartWayStands {
                standing: smoozy_state::identities::Identity::issue(
                    smoozy_state::identities::IdentityKind::Operation,
                )
                .expect("randomness"),
                said: "x".into(),
            }),
            ended(TheRunsEnding::ItStoppedInsideAStep { step: Step::Driver }),
            ended(TheRunsEnding::AStepNeedsAdminAccess { step: Step::Driver }),
            ended(TheRunsEnding::TheMachineIsRestarting),
        ];
        for came_to in every {
            for told in [what_it_came_to(&came_to), what_the_install_came_to(&came_to, "5.2.1")] {
                assert!(!told.said.trim().is_empty(), "{came_to:?} says nothing");
                // **Every sentence ends as a sentence**, and no word in it is a
                // type's name — a capital inside a word that also has small
                // letters is the mark of one (*GPU* is a word; *RolledBack* is
                // not).
                assert!(told.said.ends_with('.'), "{came_to:?} is said as {:?}", told.said);
                for word in told.said.split_whitespace() {
                    assert!(
                        !(word.chars().skip(1).any(char::is_uppercase)
                            && word.chars().any(char::is_lowercase)),
                        "{came_to:?} is said as {:?}",
                        told.said
                    );
                }
                for debugging in ["{", "(\"", "::", "Some(", "None"] {
                    assert!(
                        !told.said.contains(debugging),
                        "{came_to:?} is said as {:?}",
                        told.said
                    );
                }
            }
        }
        // **The step an offer names is the drawing's own shape.**
        assert_eq!(
            what_it_came_to(&ended(TheRunsEnding::AStepNeedsAdminAccess { step: Step::Driver }))
                .said,
            format!(
                "{} {} {}",
                words::SETUP_STOPPED,
                words::DOING_THE_DRIVER,
                words::NEEDS_ADMIN_ACCESS_AND_NONE_WAS_GIVEN
            )
        );
    }

    /// **Step 10c's Vast pass, fault 4: *What the server said* under a setup
    /// held back by work showed only `ed2edae4fdd86f822f35bf73a0bf25dd`** — the
    /// identity of *pv busy*, which was rendering under a server row removed
    /// with *Leave it running* while the same machine was added again and set
    /// up. **The local record still held that job under its name.** The words
    /// are the server program's list of the work that holds the commit
    /// (`Rented-server runtime` G21 and `Rented-server runtime` G22), and every job in it that this
    /// computer's record holds is called what Queue calls it; an identity the
    /// record does not hold stays as the server gave it.
    #[test]
    fn the_work_holding_a_setup_back_is_named_as_queue_names_it() {
        use crate::setting_up::WhereTheSetUpGot;
        use smoozy_server_protocol::placement::performing::WhatThePlacementReached as Reached;
        use smoozy_state::identities::{Identity, IdentityKind};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-set-up-held-back-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let removed_row = Identity::issue(IdentityKind::LocalServer).unwrap();
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let busy = crate::record::deletions::tests::a_record_in_history(&removed_row, "pv busy");
        store.write_the_provisional_record(&busy).unwrap();
        let mut unnamed = crate::record::deletions::tests::a_record_in_history(&removed_row, "");
        unnamed.label = None;
        unnamed.blend_path_on_the_machine = Some("/pv-huge/blend/pv-anim.blend".to_string());
        store.write_the_provisional_record(&unnamed).unwrap();
        let a_stranger = Identity::issue(IdentityKind::Job).unwrap().as_text();
        let mut from_the_record = |job: &Identity| {
            store
                .read(job)
                .ok()
                .flatten()
                .map(|held| crate::surface::the_queue::what_the_job_is_called(&held))
        };

        let deferred = WhereTheSetUpGot::ThePlacement {
            reached: Reached::Deferred { because: busy.job.as_text() },
        };
        let told = the_jobs_are_named(what_it_came_to(&deferred), &deferred, &mut from_the_record);
        assert_eq!(told.said, format!("{} {}", words::SETUP_STOPPED, words::WORK_STILL_NEEDS_ITS_PROGRAM));
        assert_eq!(told.the_machines_own_words.as_deref(), Some("pv busy"), "the bare identity is shown");
        let answer = format!("{:?}", the_answer_about("a-server", &told));
        assert!(answer.contains("\"pv busy\""), "{answer}");
        assert!(!answer.contains(&busy.job.as_text()), "{answer}");

        let held = WhereTheSetUpGot::JobsOfAnEarlierSetupAreHeld {
            how_many: 1,
            because: format!("{}, {}, {a_stranger}", busy.job.as_text(), unnamed.job.as_text()),
        };
        let told = the_jobs_are_named(what_it_came_to(&held), &held, &mut from_the_record);
        let _ = fs::remove_dir_all(&path);
        assert_eq!(
            told.the_machines_own_words.as_deref(),
            Some(format!("pv busy, pv-anim.blend, {a_stranger}").as_str())
        );
    }

    /// **Step 10c's thirteenth-pass fix piece 2: the card says how many held
    /// jobs of an earlier setup stopped it, and that this copy cannot answer
    /// them** — `Design/Server Setup § When work on the server holds the setup
    /// back`, word for word, singular and plural. **Waiting or rendering work
    /// keeps its own sentence**, which goes away by itself.
    #[test]
    fn held_jobs_of_an_earlier_setup_are_counted_on_the_card() {
        use crate::setting_up::WhereTheSetUpGot;
        use smoozy_server_protocol::placement::performing::WhatThePlacementReached as Reached;
        let two = what_it_came_to(&WhereTheSetUpGot::JobsOfAnEarlierSetupAreHeld {
            how_many: 2,
            because: "j1, j2".into(),
        });
        assert_eq!(
            two.said,
            format!(
                "Setup stopped. Smoozy's server software cannot be replaced: {} from an earlier \
                 setup need an answer this computer cannot give. A new server will not have them.",
                crate::typography::a_figure_with_its_unit(2, "jobs")
            )
        );
        assert_eq!(two.the_machines_own_words.as_deref(), Some("j1, j2"));
        let one = what_it_came_to(&WhereTheSetUpGot::JobsOfAnEarlierSetupAreHeld {
            how_many: 1,
            because: "j1".into(),
        });
        assert_eq!(
            one.said,
            format!(
                "Setup stopped. Smoozy's server software cannot be replaced: {} from an earlier \
                 setup needs an answer this computer cannot give. A new server will not have it.",
                crate::typography::a_figure_with_its_unit(1, "job")
            )
        );
        let live = what_it_came_to(&WhereTheSetUpGot::ThePlacement {
            reached: Reached::Deferred { because: "j3".into() },
        });
        assert_eq!(
            live.said,
            format!("{} {}", words::SETUP_STOPPED, words::WORK_STILL_NEEDS_ITS_PROGRAM)
        );
    }

    /// **Fault 2 of step 10c's fourth pass on Vast: a failed run's box names
    /// what actually failed.**
    ///
    /// The third *Set up again* ended at *done* because the machine's
    /// supervisor stopped answering the run's state, and the box read *The
    /// server did not pass Smoozy's final check* — **nothing had been
    /// checked**. The ending is its own now, and the sentence says which step
    /// was under way and that Smoozy does not know what the server did.
    #[test]
    fn a_supervisor_that_stopped_answering_is_not_said_as_a_failed_final_check() {
        use crate::setting_up::{TheRunsEnding, WhereTheSetUpGot};
        use smoozy_server_setup::ready_action::Step;
        let came_to = WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Started,
            ending: TheRunsEnding::TheServerStoppedAnswering {
                step: Step::Done,
                said: "Smoozy's service on the server stopped responding.".into(),
            },
        };
        let told = what_it_came_to(&came_to);
        assert!(
            !told.said.contains(words::FAILED_FINAL_CHECK),
            "{}",
            told.said
        );
        assert!(told.said.contains(words::DOING_THE_FINAL_CHECK), "{}", told.said);
        assert!(
            told.said.contains("stopped responding"),
            "{}",
            told.said
        );
        // **The machine's own words stay behind the control**, never inside the
        // sentence.
        assert_eq!(
            told.the_machines_own_words.as_deref(),
            Some("Smoozy's service on the server stopped responding.")
        );
        // **And the step's own failure sentence is still said where the step
        // itself failed.**
        let failed = what_it_came_to(&WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Started,
            ending: TheRunsEnding::AStepFailed {
                step: Step::Done,
                said: "x".into(),
            },
        });
        assert!(failed.said.contains(words::FAILED_FINAL_CHECK), "{}", failed.said);
    }

    /// **A run's ending says whether the card already says it, and that the
    /// Blender table is read again** — the Creator's notes 4 and 8 on 0.1.7. A
    /// setup that reached its end is said by the card's own *Ready to render*
    /// and its steps; a stopped one is said as one line on the card. **Neither
    /// carries a stop or a cancel**: the run is over.
    #[test]
    fn a_runs_ending_marks_the_table_and_says_whether_the_card_says_it() {
        use crate::setting_up::{TheRunsEnding, WhereTheSetUpGot};
        let reached = WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Done,
            ending: TheRunsEnding::ItReachedDone { a_blender_was_published: true, the_driver_missed_the_optix_bar: false, the_swap_waits: false },
        };
        let stopped = WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Started,
            ending: TheRunsEnding::TheUserStoppedIt,
        };
        assert!(the_card_says_the_ready_actions_ending(&reached));
        assert!(!the_card_says_the_ready_actions_ending(&stopped));
        let said = |came_to: &WhereTheSetUpGot| {
            let ending = delivered(a_runs_ending(
                the_answer_about("server-1", &what_it_came_to(came_to)),
                the_card_says_the_ready_actions_ending(came_to),
            ));
            (
                it_says(&ending, "the_card_says_it"),
                it_says(&ending, "the_blenders_are_read_again"),
                format!("{ending:?}"),
            )
        };
        let (says, read_again, printed) = said(&reached);
        assert!(says && read_again, "{printed}");
        let (says, read_again, printed) = said(&stopped);
        assert!(!says && read_again, "{printed}");
        for printed in [said(&reached).2, said(&stopped).2] {
            assert!(!printed.contains(&format!("\"{}\"", words::CANCEL_SETUP)), "{printed}");
            assert!(!printed.contains("\"stop\""), "{printed}");
        }
    }

    /// **Fault 5 of the first pass: a setup that stopped part-way is drawn from
    /// the machine's own record after its box is closed** — `Server Setup` S1,
    /// S2 and Y6a — and the card claims nothing this reading did not take.
    #[test]
    fn a_run_read_off_the_machine_draws_the_card_and_claims_nothing_else() {
        let record = TheSetupRecord {
            kind: smoozy_server_setup::run::SetupKind::TheReadyAction,
            the_composed_list: vec![Boundary::Started, Boundary::DriverPlaced, Boundary::Done],
            the_boundaries_reached: vec![a_boundary(Boundary::Started, WhatTheStepDid::Performed)],
            ending: Some(HowTheRunEnded::EndedAtBoundaryNotCompleted {
                at: Some(Boundary::Started),
            }),
        };
        let stopped = the_tab(
            WhatTheMachineSaid::ASetupRunAlone(Box::new(crate::setting_up::TheSetupAsRead {
                the_run_holds_the_machine: true,
                record: record.clone(),
                still_open: false,
                the_step_in_flight: Some(Boundary::DriverPlaced),
                the_admin_step_recorded_as_started: None,
                the_clock: "server-1".into(),
            })),
            "server-1",
        );
        assert_eq!(stopped.the_card.state, Prose::new(words::SETUP_STOPPED_PART_WAY));
        assert!(stopped.the_card.the_action.loud);
        assert_eq!(stopped.the_card.the_render_device, None);
        assert_eq!(stopped.the_card.the_setup_fault, None);
        // **The Blender table was not read and says so.**
        assert!(stopped.the_table.instead.is_some());

        let running = the_tab(
            WhatTheMachineSaid::ASetupRunAlone(Box::new(crate::setting_up::TheSetupAsRead {
                the_run_holds_the_machine: true,
                record: TheSetupRecord {
                    ending: None,
                    ..record
                },
                still_open: true,
                the_step_in_flight: Some(Boundary::DriverPlaced),
                the_admin_step_recorded_as_started: None,
                the_clock: "server-1".into(),
            })),
            "server-1",
        );
        assert_eq!(running.the_card.state, Prose::new(words::SETTING_UP));
        assert!(running.the_card.the_action.underway);
    }

    /// **The third pass's machine, on the tab: a setup that stopped at the
    /// driver step opens on its panel, with both of `Server Setup` S6's ways
    /// out — and an abandoned one reads *Not set up* and opens on nothing.**
    /// Added 2026-09-22: until then the panel was composed and drawn by
    /// nothing, and *Abandon setup* was on no screen.
    #[test]
    fn a_setup_stopped_at_the_driver_opens_on_its_panel_and_an_abandoned_one_reads_not_set_up() {
        use crate::setting_up::TheSetupAsRead;
        use smoozy_server_setup::recovery::TheFailedStepsWords;
        let a_run_ending = |ending| TheSetupAsRead {
            the_run_holds_the_machine: true,
            record: TheSetupRecord {
                kind: SetupKind::TheReadyAction,
                the_composed_list: vec![
                    Boundary::Started,
                    Boundary::DriverPlaced,
                    Boundary::RestartIssued,
                    Boundary::CardAnswering,
                    Boundary::Done,
                ],
                the_boundaries_reached: vec![a_boundary(Boundary::Started, WhatTheStepDid::NothingToDo)],
                ending: Some(ending),
            },
            still_open: false,
            the_step_in_flight: Some(Boundary::DriverPlaced),
            the_admin_step_recorded_as_started: None,
            the_clock: "Hyperstack A6000".to_string(),
        };

        let failed = a_run_ending(HowTheRunEnded::AStepFailed {
            step: smoozy_server_setup::ready_action::Step::Driver,
            words: TheFailedStepsWords::SmoozysOwnVerification {
                what_was_read: "E: Failed to fetch … 404  Not Found".to_string(),
            },
        });
        let tab = the_tab(WhatTheMachineSaid::ASetupRunAlone(Box::new(failed)), "Hyperstack A6000");
        assert_eq!(tab.the_card.state, Prose::new(words::SETUP_STOPPED_PART_WAY));
        let panel = tab.the_stopped_setup.expect("the stopped setup opens on its panel");
        assert_eq!(panel.abandon, Some(Prose::new(words::ABANDON_SETUP)));
        assert_eq!(panel.set_up_again, Some(Prose::new(words::SET_UP_AGAIN)));
        assert!(panel.cancel.is_none(), "nothing is running to cancel");

        let abandoned = a_run_ending(HowTheRunEnded::Abandoned {
            what_the_check_found: TheFailedStepsWords::None,
        });
        let tab = the_tab(WhatTheMachineSaid::ASetupRunAlone(Box::new(abandoned)), "Hyperstack A6000");
        assert_eq!(tab.the_card.state, Prose::new(words::NOT_SET_UP));
        assert!(tab.the_card.the_action.loud);
        assert!(tab.the_stopped_setup.is_none(), "an abandoned setup has nothing left to finish");

        let mut going = a_run_ending(HowTheRunEnded::Completed);
        going.still_open = true;
        going.record.ending = None;
        let tab = the_tab(WhatTheMachineSaid::ASetupRunAlone(Box::new(going)), "Hyperstack A6000");
        assert!(tab.the_stopped_setup.is_none(), "a run still going is not a stopped one");
    }

    /// **The Set up card never reads *Not set up* off a run of another kind.**
    /// Added 2026-09-22, out of step 10c's third pass on Vast: the run the
    /// standing state named was a Blender install, the card looked for a ready
    /// action's record in it, found none and read *Not set up* on a machine with
    /// a verified installation — from 11:30 to the end of the pass.
    #[test]
    fn a_blender_install_read_alone_never_reads_not_set_up() {
        use crate::setting_up::TheSetupAsRead;
        use smoozy_server_setup::recovery::TheFailedStepsWords;
        let an_install = |ending| TheSetupAsRead {
            the_run_holds_the_machine: true,
            record: TheSetupRecord {
                kind: SetupKind::ABlenderInstall,
                the_composed_list: vec![
                    Boundary::Started,
                    Boundary::Fetched,
                    Boundary::Tooling,
                    Boundary::Blender,
                    Boundary::Done,
                ],
                the_boundaries_reached: vec![
                    a_boundary(Boundary::Started, WhatTheStepDid::NothingToDo),
                    a_boundary(Boundary::Fetched, WhatTheStepDid::Performed),
                ],
                ending: Some(ending),
            },
            still_open: false,
            the_step_in_flight: Some(Boundary::Tooling),
            the_admin_step_recorded_as_started: None,
            the_clock: "Vast 2060S".to_string(),
        };
        // **Stopped part-way, holding the machine**: the stopped card and its
        // panel, never *Not set up*.
        let failed = an_install(HowTheRunEnded::AStepFailed {
            step: smoozy_server_setup::ready_action::Step::Tooling,
            words: TheFailedStepsWords::SmoozysOwnVerification {
                what_was_read: "the unpack was commanded with no version to name its archive"
                    .to_string(),
            },
        });
        let tab = the_tab(WhatTheMachineSaid::ASetupRunAlone(Box::new(failed)), "Vast 2060S");
        assert_eq!(tab.the_card.state, Prose::new(words::SETUP_STOPPED_PART_WAY));
        assert!(tab.the_stopped_setup.is_some());
        // **Finished, or abandoned**: nothing about whether the server was set
        // up was read, and the card says only that.
        for ending in [
            HowTheRunEnded::Completed,
            HowTheRunEnded::Abandoned {
                what_the_check_found: TheFailedStepsWords::None,
            },
        ] {
            let tab = the_tab(
                WhatTheMachineSaid::ASetupRunAlone(Box::new(an_install(ending))),
                "Vast 2060S",
            );
            assert_ne!(tab.the_card.state, Prose::new(words::NOT_SET_UP));
            assert_eq!(tab.the_card.state, Prose::new(words::NO_SETUP_WAS_READ));
        }
    }

    /// **The box names what failed, and *ready to render* only follows a
    /// published Blender.** Added 2026-09-22.
    #[test]
    fn the_box_names_the_unpack_and_says_ready_to_render_only_with_a_blender() {
        use crate::setting_up::{TheRunsEnding, WhereTheSetUpGot};
        let ended = |ending| WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Fetched,
            ending,
        };
        let unpack = what_the_install_came_to(
            &ended(TheRunsEnding::TheUnpackFailed {
                said: "the unpack was commanded with no version to name its archive".to_string(),
            }),
            "5.2.2",
        );
        assert!(unpack.said.contains(words::FAILED_UNPACK), "{}", unpack.said);
        assert!(!unpack.said.contains(words::FAILED_TOOLING), "{}", unpack.said);
        assert!(unpack
            .the_machines_own_words
            .is_some_and(|words| words.contains("no version")));
        let tool = what_the_install_came_to(
            &ended(TheRunsEnding::AStepFailed {
                step: smoozy_server_setup::ready_action::Step::Tooling,
                said: "E: Unable to locate package xz-utils".to_string(),
            }),
            "5.2.2",
        );
        assert!(tool.said.contains(words::FAILED_TOOLING), "{}", tool.said);
        let done = |a_blender_was_published| WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Done,
            ending: TheRunsEnding::ItReachedDone { a_blender_was_published, the_driver_missed_the_optix_bar: false, the_swap_waits: false },
        };
        assert_eq!(what_it_came_to(&done(true)).said, words::THE_SERVER_IS_READY);
        assert_eq!(what_it_came_to(&done(false)).said, words::THE_SERVER_IS_SET_UP);
        assert!(!what_the_install_came_to(&done(false), "5.2.2").said.contains(&format!("5.2.2 {}", words::BLENDER_IS_INSTALLED)));
    }

    /// **What an abandon says** — `Design/Server Setup § The two ways out`:
    /// *"says what that check found"*, the server's own words behind the control
    /// where the check did not pass.
    #[test]
    fn an_abandon_says_what_the_final_check_found() {
        use crate::setting_up::{TheRunsEnding, WhatTheAbandonCameTo};
        let passed = what_the_abandon_came_to(&WhatTheAbandonCameTo::Abandoned {
            what_the_final_check_found: None,
        });
        assert_eq!(passed.said, words::ABANDONED_AND_THE_CHECK_PASSED);
        assert_eq!(passed.the_machines_own_words, None);
        let not = what_the_abandon_came_to(&WhatTheAbandonCameTo::Abandoned {
            what_the_final_check_found: Some("NVIDIA-SMI has failed".to_string()),
        });
        assert_eq!(not.said, words::ABANDONED_AND_THE_CHECK_DID_NOT_PASS);
        assert_eq!(not.the_machines_own_words.as_deref(), Some("NVIDIA-SMI has failed"));
        assert_eq!(
            what_the_abandon_came_to(&WhatTheAbandonCameTo::NothingStoodToAbandon).said,
            words::NOTHING_TO_ABANDON
        );
        let refused = what_the_abandon_came_to(&WhatTheAbandonCameTo::ItDidNotFinish(
            TheRunsEnding::NoRunCouldBeOpened {
                said: "the setup worker was not admitted".to_string(),
            },
        ));
        assert_eq!(refused.said, words::THE_ABANDON_DID_NOT_FINISH);
        assert!(refused.the_machines_own_words.is_some());
    }

    /// **Fix piece 6, 2026-09-27: the refusal and the panel say what the
    /// machine's record says** — step 10c's seventh pass on Hyperstack, faults 3
    /// and 4, and the runner's ruling 1 of that day.
    #[test]
    fn the_refusal_and_the_panel_say_what_the_record_says() {
        use crate::setting_up::{TheRunsEnding, WhereTheSetUpGot};
        use smoozy_server_setup::ready_action::Step;
        // **Fault 3: the box names the step the record names**, and never the
        // graphics driver's install whatever the step was.
        for step in [Step::Driver, Step::Restart] {
            let said = what_it_came_to(&WhereTheSetUpGot::TheRunEnded {
                at: Boundary::DriverPlaced,
                ending: TheRunsEnding::ItStoppedInsideAStep { step },
            })
            .said;
            assert!(said.contains(what_the_step_does(step)), "{said}");
            assert!(said.contains(words::ABANDON_SETUP), "{said}");
        }
        // **The ruling: the user is told the restart was refused, in the
        // machine's words where it gave any.**
        let refused = what_it_came_to(&WhereTheSetUpGot::TheRunEnded {
            at: Boundary::DriverPlaced,
            ending: TheRunsEnding::TheRestartWasRefused {
                said: "Failed to reboot: Access denied".to_string(),
            },
        });
        assert!(refused.said.contains(words::THE_RESTART_WAS_REFUSED), "{}", refused.said);
        assert_eq!(
            refused.the_machines_own_words.as_deref(),
            Some("Failed to reboot: Access denied")
        );
        // **Fault 4: a recovery Smoozy did not carry on is not drawn as a card
        // that did not answer.**
        let record = TheSetupRecord {
            kind: SetupKind::TheReadyAction,
            the_composed_list: vec![
                Boundary::Started,
                Boundary::DriverPlaced,
                Boundary::CardAnswering,
                Boundary::Done,
            ],
            the_boundaries_reached: vec![
                a_boundary(Boundary::Started, WhatTheStepDid::Performed),
                a_boundary(Boundary::DriverPlaced, WhatTheStepDid::Performed),
            ],
            ending: Some(HowTheRunEnded::AStepFailed {
                step: Step::Driver,
                words: TheFailedStepsWords::SmoozysOwnVerification {
                    what_was_read: crate::setting_up::THE_RECOVERY_WAS_NOT_CARRIED_ON_EARLIER.to_string(),
                },
            }),
        };
        let panel = the_run_panel(&record, Some(Boundary::DriverPlaced), "Hyperstack A6000", false, false);
        let failed: Vec<&TheFailedStep> =
            panel.steps.iter().filter_map(|it| it.the_failure.as_ref()).collect();
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0].said.lead, Prose::new(words::FAILED_NOT_CARRIED_ON));
        // **And a card that really did not answer still says so.**
        let answered_nothing = TheSetupRecord {
            ending: Some(HowTheRunEnded::AStepFailed {
                step: Step::Driver,
                words: TheFailedStepsWords::SmoozysOwnVerification {
                    what_was_read: "The server is back, but its GPU still does not \
                                    respond."
                        .to_string(),
                },
            }),
            ..record
        };
        let panel = the_run_panel(
            &answered_nothing,
            Some(Boundary::DriverPlaced),
            "Hyperstack A6000",
            false,
            false,
        );
        let failed: Vec<&TheFailedStep> =
            panel.steps.iter().filter_map(|it| it.the_failure.as_ref()).collect();
        assert_eq!(failed[0].said.lead, Prose::new(words::FAILED_CARD_CHECK));
    }

    /// **Fix piece 6, 2026-09-27, on the card since 2026-10-02: *Abandon
    /// setup* is on the card wherever a stopped setup stands** — step 10c's
    /// seventh pass on Hyperstack, fault 5, and the Creator's note 4 on 0.1.7.
    /// The page draws the two controls the application hands it, in
    /// `Design/Server Setup § The two ways out`'s order, and every run's ending
    /// goes through [`a_runs_ending`].
    #[test]
    fn the_stopped_card_carries_the_two_ways_out_and_every_ending_is_a_runs_ending() {
        let page = the_page();
        let the_card = page
            .split("function theRunCardIsDrawn(panel, server) {")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the run card");
        let abandon = the_card
            .find("aControl(panel.abandon, () => theActIsAskedFor(ABANDON, server), true)")
            .expect("*Abandon setup* is not on the card");
        let again = the_card
            .find("aControl(panel.set_up_again, () => theActIsAskedFor(THE_READY_ACTION, server))")
            .expect("*Set up again* is not on the card");
        assert!(abandon < again, "the two ways out are not in Design's order");
        assert!(!the_card.contains("aBox()"), "the run is drawn in a box");
        let source = include_str!("set_up.rs");
        let body_of = |opening: &str| {
            source
                .split(opening)
                .nth(1)
                .and_then(|rest| rest.split("\n}\n").next())
                .expect("the act")
                .to_string()
        };
        assert!(body_of("fn an_abandon_is_driven<R: tauri::Runtime>(").contains("a_runs_ending(answer, "));
        // **Every card act is an act of the line from 2026-10-27** (`Server
        // Setup` Y9a), and its ending is said where the line says them all.
        for act in [
            "the_ready_action_is_driven",
            "a_blender_install_is_driven",
            "a_build_is_chosen",
            "a_removal_is_driven",
            "the_telegram_placement_is_driven",
        ] {
            let body = body_of(&format!("fn {act}<R: tauri::Runtime>("));
            assert!(body.contains("the_acts_are_pressed("), "{act} is not taken by the line");
        }
        let endings = body_of("fn the_acts_ending(");
        assert_eq!(endings.matches("a_runs_ending(").count(), 2, "the ready action's and Telegram's");
        assert_eq!(
            endings.matches("a_blender_runs_ending(server, the_answer_about(server, &came), ").count(),
            2,
            "an install's ending is not a Blender run's ending"
        );
        assert!(body_of("fn a_blender_runs_ending(")
            .contains("a_runs_ending(answer, "), "a Blender run's ending does not keep the setup ending");
    }

    /// **Eighth-pass fix piece 2, 2026-09-27: *"The render that was running
    /// stopped…"* only where a render was running when the setup began, and
    /// only from *restart issued* on** — `Server Setup` T6a and T7, and
    /// `Design/Server Setup § A long step`'s *"the render which was running
    /// went down with it"*.
    ///
    /// **Step 10c's eighth pass on Hyperstack drew it from the driver step on**,
    /// before any restart, on a machine that had never rendered. **Proved to
    /// bite** in a copy outside the repository.
    #[test]
    fn the_render_sentence_is_drawn_only_where_a_render_ran_and_only_from_restart_issued() {
        let a_run_that_reached = |reached: &[Boundary], in_flight: Option<Boundary>| {
            crate::setting_up::TheSetupAsRead {
                the_run_holds_the_machine: true,
                record: TheSetupRecord {
                    kind: SetupKind::TheReadyAction,
                    the_composed_list: vec![
                        Boundary::Started,
                        Boundary::DriverPlaced,
                        Boundary::RestartIssued,
                        Boundary::CardAnswering,
                        Boundary::Done,
                    ],
                    the_boundaries_reached: reached
                        .iter()
                        .map(|it| a_boundary(*it, WhatTheStepDid::Performed))
                        .collect(),
                    ending: None,
                },
                still_open: true,
                the_step_in_flight: in_flight,
                the_admin_step_recorded_as_started: None,
                the_clock: String::new(),
            }
        };
        let the_sentence = Some(Prose::new(words::THE_RENDER_STOPPED));
        // **The driver step under way, and just after it: nothing has gone
        // down**, whether or not a render was running.
        for (reached, in_flight) in [
            (vec![Boundary::Started], Some(Boundary::DriverPlaced)),
            (vec![Boundary::Started, Boundary::DriverPlaced], None),
        ] {
            let read = a_run_that_reached(&reached, in_flight);
            for a_render_was_running in [true, false] {
                assert_eq!(
                    the_panel_of_a_run_under_way(&read, "Hyperstack L40", a_render_was_running)
                        .about_the_render,
                    None,
                    "the sentence was drawn before the restart, at {reached:?}"
                );
            }
        }
        // **From *restart issued* on, only where a render was running.**
        for reached in [
            vec![Boundary::Started, Boundary::DriverPlaced, Boundary::RestartIssued],
            vec![
                Boundary::Started,
                Boundary::DriverPlaced,
                Boundary::RestartIssued,
                Boundary::CardAnswering,
            ],
        ] {
            let read = a_run_that_reached(&reached, None);
            assert_eq!(
                the_panel_of_a_run_under_way(&read, "Hyperstack L40", true).about_the_render,
                the_sentence,
                "a render was running and the restart went out, at {reached:?}"
            );
            assert_eq!(
                the_panel_of_a_run_under_way(&read, "Hyperstack L40", false).about_the_render,
                None,
                "no render was running, at {reached:?}"
            );
        }
        // **And the command the page asks carries what the warning was shown
        // with, and nothing else**: the run's own carried reading. Changed
        // 2026-09-27, when `Server Setup` W3's warning got its caller.
        let source = include_str!("set_up.rs");
        let the_command = source
            .split("pub fn the_setup_under_way_on(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the command");
        assert!(the_command
            .contains("crate::setting_up::a_render_was_running_when_the_run_began(&server)"));
    }

    /// **What the warning named is carried with the run it was shown before,
    /// and with no other** — set only on a run being driven, let go when that
    /// run ends, and never carried into the next one. Added 2026-09-27.
    #[test]
    fn a_render_named_by_the_warning_is_carried_with_that_run_alone() {
        use crate::setting_up::{
            a_render_was_running_when_the_run_began, a_run_begins, a_run_ends,
            the_run_began_with_a_render_running,
        };
        let server = "server-carried-render";
        the_run_began_with_a_render_running(server);
        assert!(
            !a_render_was_running_when_the_run_began(server),
            "carried with no run being driven"
        );
        assert!(a_run_begins(server, SetupKind::TheReadyAction));
        assert!(!a_render_was_running_when_the_run_began(server));
        the_run_began_with_a_render_running(server);
        assert!(a_render_was_running_when_the_run_began(server));
        a_run_ends(server);
        assert!(!a_render_was_running_when_the_run_began(server));
        assert!(a_run_begins(server, SetupKind::TheReadyAction));
        assert!(
            !a_render_was_running_when_the_run_began(server),
            "the last run's reading was carried into the next"
        );
        a_run_ends(server);
    }

    /// **`Server Setup` S5 and `Server Setup` Y9: a cut run's wait replaces stale steps,
    /// belongs to one server, and clears before the new run is drawn.**
    #[test]
    fn a_cut_runs_wait_is_shown_on_its_server_until_the_new_run_opens() {
        let server = "server-waiting-for-cut-run";
        let other = "server-not-waiting-for-cut-run";
        assert!(crate::setting_up::a_run_begins(server, SetupKind::TheReadyAction));
        assert!(crate::setting_up::a_run_begins(other, SetupKind::TheReadyAction));
        the_wait_for_the_cut_run_is_kept(server, true);
        let waiting = the_setup_under_way_on(server, String::new).unwrap();
        assert_eq!(waiting.title, Prose::new(words::THE_SERVER_IS_FINISHING_THE_CUT_RUN));
        assert!(waiting.the_state_is_underway && waiting.cancel.is_some());
        assert!(waiting.steps.is_empty() && waiting.set_up_again.is_none());
        assert_eq!(the_setup_under_way_on(other, String::new).unwrap().title, Prose::new(words::SETTING_UP));
        the_wait_for_the_cut_run_is_kept(server, false);
        assert_eq!(the_setup_under_way_on(server, String::new).unwrap().title, Prose::new(words::SETTING_UP));
        crate::setting_up::a_run_ends(server);
        crate::setting_up::a_run_ends(other);
    }

    /// **The worldwide pass's fault 4: once a run ends, its steps are answered
    /// as its own last reading left them, marked as ended**, until the next run
    /// begins on that server — so the card keeps the last step *done* while its
    /// own reading is taken, and never the panel it happened to draw last.
    /// Added 2026-10-26. **And the page takes them as the ended run's, and
    /// never as a run still going.**
    #[test]
    fn worldwide_fault_4_an_ended_runs_steps_are_answered_as_its_last_reading_left_them() {
        use crate::setting_up::{a_run_begins, a_run_ends, the_run_is_read_under_way};
        let server = "server-worldwide-fault-4";
        let every_step_done = crate::setting_up::TheSetupAsRead {
            the_run_holds_the_machine: true,
            record: TheSetupRecord {
                kind: SetupKind::TheReadyAction,
                the_composed_list: vec![Boundary::Started, Boundary::Fetched, Boundary::Done],
                the_boundaries_reached: vec![
                    a_boundary(Boundary::Started, WhatTheStepDid::Performed),
                    a_boundary(Boundary::Fetched, WhatTheStepDid::Performed),
                    a_boundary(Boundary::Done, WhatTheStepDid::Performed),
                ],
                ending: None,
            },
            still_open: true,
            the_step_in_flight: None,
            the_admin_step_recorded_as_started: None,
            the_clock: String::new(),
        };
        assert!(the_setup_under_way_on(server, String::new).is_none(), "nothing before any run");
        assert!(a_run_begins(server, SetupKind::TheReadyAction));
        assert!(the_run_is_read_under_way(server, &every_step_done));
        let going = the_setup_under_way_on(server, String::new).expect("the run's panel");
        assert!(!going.the_run_has_ended);
        a_run_ends(server);
        let ended = the_setup_under_way_on(server, String::new).expect("the ended run's steps");
        assert!(ended.the_run_has_ended);
        assert_eq!(ended.steps.iter().map(|it| it.mark.as_str()).collect::<Vec<_>>(), vec!["done"; 3]);
        assert!(ended.cancel.is_none(), "an ended run offers no Cancel setup");
        assert!(ended.the_servers_newest_lines.is_empty(), "the window goes when the run ends");
        // **The next run on that server lets them go.**
        assert!(a_run_begins(server, SetupKind::TheReadyAction));
        assert!(!the_setup_under_way_on(server, String::new).expect("the new run's panel").the_run_has_ended);
        a_run_ends(server);
        // **The page**: an ended run's steps are what it keeps, and a panel
        // marked ended is never drawn as the run going.
        let page = the_page();
        assert!(page.contains("if (!panel || panel.the_run_has_ended) {"));
        assert!(page.contains("theStepsAsTheRunLeftThem(panel || theRun.panel)"));
        assert!(page.contains("if (panel && !panel.the_run_has_ended && current === server"));
        // **An ended run told before its answer holds its own steps** until
        // the answer comes.
        assert!(page.contains("if (panel && theTabLanded >= theTabAsks) {"));
    }

    /// **Fix piece 9, 2026-09-27: a run still going is drawn as its steps, with
    /// the one under way marked** — `Design/Server Setup § A long step` and the
    /// drawing's `su-run`, fed from the run's own record.
    #[test]
    fn a_run_under_way_is_drawn_as_its_steps_with_the_one_under_way_marked() {
        let read = crate::setting_up::TheSetupAsRead {
            the_run_holds_the_machine: true,
            record: TheSetupRecord {
                kind: SetupKind::ABlenderInstall,
                the_composed_list: vec![
                    Boundary::Started,
                    Boundary::Fetched,
                    Boundary::Tooling,
                    Boundary::Libraries,
                    Boundary::Blender,
                    Boundary::Done,
                ],
                the_boundaries_reached: vec![
                    a_boundary(Boundary::Started, WhatTheStepDid::Performed),
                    a_boundary(Boundary::Fetched, WhatTheStepDid::Performed),
                ],
                ending: None,
            },
            still_open: true,
            the_step_in_flight: Some(Boundary::Tooling),
            the_admin_step_recorded_as_started: None,
            the_clock: String::new(),
        };
        let panel = the_panel_of_a_run_under_way(&read, "Vast 4090", false);
        let marks: Vec<&str> = panel.steps.iter().map(|it| it.mark.as_str()).collect();
        assert_eq!(marks, vec!["done", "done", "now", "", "", ""]);
        assert_eq!(panel.title, Prose::new(words::SETTING_UP));
        assert_eq!(panel.cancel, Some(Prose::new(words::CANCEL_SETUP)), "offered throughout");
        assert!(panel.abandon.is_none() && panel.set_up_again.is_none());
        assert!(panel.about_the_render.is_none(), "an install restarts nothing");

        // **The page puts the shared shape on the step under way, asks for the
        // panel when told, covers the pane during the first reading, and never
        // draws a panel over a later answer.**
        let page = the_page();
        assert!(page.contains("if (step.mark === \"now\") line.appendChild(theMarkOnARow(null));"));
        assert!(page.contains("if (answer.the_run_is_under_way) {"));
        assert!(page.contains("shell.core.invoke(\"the_setup_under_way\", { server })"));
        assert!(page.contains("if (asked !== theAnswersSoFar) return;"));
        assert!(page.contains("theCover = theMarkOverAPane();"));
        // **Not while an ended run's steps stand** (the build runner's ruling (b)
        // of 2026-10-03): they are the card, and the result's reading is not the
        // first.
        assert!(page.contains("if (theDrawnCard === null && !theEndedRun) theCoverIsShown();"));
        assert!(
            !page.contains("underway-pane"),
            "the page draws the over-the-pane shape itself rather than calling the shared one"
        );
    }

    /// **Fix piece 10, 2026-09-27: one version is one row in each part, naming
    /// every line that publishes it** — `Design/Server Setup § Installing`.
    /// **The channels are the shop-front's own answer read that day**, at or
    /// above the floor and in their order: 5.2.2 on `latest` and on `5.2lts`,
    /// stable and candidate on both. **Before the change the list drew 5.2.2
    /// twice above the rule and twice, in the same words, below it.** They are
    /// handed to the box as read, **without** `Server Setup` B2's narrowing of
    /// 2026-09-28, so the joining below the rule is still exercised; the
    /// narrowed list is the next test's.
    #[test]
    fn one_version_published_on_two_lines_is_one_row_naming_both() {
        use smoozy_server_setup::listing::TheListingAsRead;
        let a = |track: &str, risk: Risk, major: u32, minor: u32, patch: u32| PublishedChannel {
            track: track.to_string(),
            risk,
            version: Version { major, minor, patch },
            released_at: String::new(),
            size_bytes: 0,
            the_build_services_file: None,
        };
        let mut alpha = a("", Risk::Alpha, 5, 3, 0);
        alpha.the_build_services_file = Some("blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz".to_string());
        let channels = vec![
            alpha,
            a("latest", Risk::Stable, 5, 2, 2),
            a("latest", Risk::Candidate, 5, 2, 2),
            a("5.2lts", Risk::Stable, 5, 2, 2),
            a("5.2lts", Risk::Candidate, 5, 2, 2),
            a("5.1", Risk::Stable, 5, 1, 2),
            a("5.0", Risk::Stable, 5, 0, 1),
            a("4.5lts", Risk::Stable, 4, 5, 14),
            a("4.5lts", Risk::Candidate, 4, 5, 14),
            a("4.5lts", Risk::Beta, 4, 5, 0),
        ];
        let list = the_list(
            Ok(TheListingAsRead {
                channels,
                entries_that_could_not_be_read: Vec::new(),
            }),
            Vec::new(),
        );
        let rows = |part: &[APublishedEntry]| -> Vec<(String, String)> {
            part.iter()
                .map(|row| {
                    (
                        row.version.as_str().to_string(),
                        row.line.as_str().replace('\u{a0}', " "),
                    )
                })
                .collect()
        };
        let joined = |one: &str, two: &str| format!("{one} \u{b7} {two}");
        assert_eq!(
            rows(&list.finished),
            vec![
                ("5.2.2".to_string(), joined(words::LATEST_STABLE, words::LONG_TERM_SUPPORT)),
                ("5.1.2".to_string(), words::RELEASE.to_string()),
                ("5.0.1".to_string(), words::RELEASE.to_string()),
                ("4.5.14".to_string(), words::LONG_TERM_SUPPORT.to_string()),
            ]
        );
        assert_eq!(
            rows(&list.unfinished),
            vec![
                ("5.3.0".to_string(), words::ALPHA.to_string()),
                ("5.2.2".to_string(), words::RELEASE_CANDIDATE.to_string()),
                ("4.5.14".to_string(), words::RELEASE_CANDIDATE.to_string()),
                ("4.5.0".to_string(), words::BETA.to_string()),
            ]
        );
    }

    /// The nineteenth notes, note 1: the label follows the quick path's version.
    #[test]
    fn latest_stable_names_the_quick_paths_version_when_the_default_track_lags() {
        use smoozy_server_setup::listing::TheListingAsRead;
        let channel = |track: &str, risk: Risk, version: &str| PublishedChannel {
            track: track.to_string(),
            risk,
            version: Version::parse(version).unwrap(),
            released_at: String::new(),
            size_bytes: 0,
            the_build_services_file: None,
        };
        let list = the_list(
            Ok(TheListingAsRead {
                channels: vec![
                    channel("5.2lts", Risk::Stable, "5.2.3"),
                    channel("latest", Risk::Stable, "5.2.2"),
                    channel("4.5lts", Risk::Stable, "4.5.23"),
                    channel("", Risk::Beta, "5.3.0"),
                ],
                entries_that_could_not_be_read: Vec::new(),
            }),
            Vec::new(),
        );
        assert_eq!(list.finished[0].version, Verbatim::new("5.2.3"));
        assert_eq!(
            list.finished[0].line,
            Prose::new(format!("{} \u{b7} {}", words::LATEST_STABLE, words::LONG_TERM_SUPPORT))
        );
        assert_eq!(list.finished[1].line, Prose::new(words::RELEASE));
        assert_eq!(list.finished[2].line, Prose::new(words::LONG_TERM_SUPPORT));
        assert_eq!(list.unfinished[0].line, Prose::new(words::BETA));
    }

    /// **The ninth pass's own list, through `Server Setup` B2's offered list and
    /// into the box: the unfinished builds of a released series are gone, and
    /// the part below the rule is headed *Experimental builds* with nothing
    /// explaining it** — `Design/Server Setup § Installing`, Creator-stated
    /// 2026-09-27 and 2026-09-28. The ninth pass on Hyperstack drew *5.2.2
    /// Release candidate*, *4.5.14 Release candidate* and *4.5.0 Beta* under the
    /// sentence *Unfinished builds Blender publishes for testing.*; step 10c's
    /// ninth-pass fix piece 7.
    #[test]
    fn the_ninth_passs_list_keeps_only_experimental_builds_of_unreleased_series() {
        use smoozy_server_setup::listing::TheListingAsRead;
        let a = |track: &str, risk: Risk, major: u32, minor: u32, patch: u32| PublishedChannel {
            track: track.to_string(),
            risk,
            version: Version { major, minor, patch },
            released_at: String::new(),
            size_bytes: 0,
            the_build_services_file: None,
        };
        let mut alpha = a("", Risk::Alpha, 5, 3, 0);
        alpha.the_build_services_file = Some("blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz".to_string());
        let published = vec![
            alpha,
            a("latest", Risk::Stable, 5, 2, 2),
            a("latest", Risk::Candidate, 5, 2, 2),
            a("5.2lts", Risk::Stable, 5, 2, 2),
            a("5.2lts", Risk::Candidate, 5, 2, 2),
            a("5.1", Risk::Stable, 5, 1, 2),
            a("5.0", Risk::Stable, 5, 0, 1),
            a("4.5lts", Risk::Stable, 4, 5, 14),
            a("4.5lts", Risk::Candidate, 4, 5, 14),
            a("4.5lts", Risk::Beta, 4, 5, 0),
        ];
        let list = the_list(
            Ok(TheListingAsRead {
                channels: smoozy_server_setup::blender::the_offered_list(&published),
                entries_that_could_not_be_read: Vec::new(),
            }),
            Vec::new(),
        );
        assert_eq!(list.the_heading, Prose::new("Experimental builds"));
        let versions = |part: &[APublishedEntry]| -> Vec<String> {
            part.iter().map(|row| row.version.as_str().to_string()).collect()
        };
        assert_eq!(versions(&list.finished), vec!["5.2.2", "5.1.2", "5.0.1", "4.5.14"]);
        assert_eq!(
            versions(&list.unfinished),
            vec!["5.3.0"],
            "only the unfinished build of a series with no finished release is offered"
        );
        assert_eq!(list.unfinished[0].line, Prose::new(words::ALPHA));

        // **The page draws it as the drawing's heading, `h4.buexp`, and nothing
        // else under the rule.**
        let page = the_page();
        assert!(page.contains("const heading = document.createElement(\"h4\");"));
        assert!(page.contains("heading.className = \"buexp\";"));
        assert!(page.contains("heading.textContent = list.the_heading;"));
        assert!(!page.contains("list.the_rule"), "the old sentence is still drawn");
    }

    // ─────────────────── Smoozy's service not answering, on the card

    struct NothingIsPostedHere;

    impl crate::notifications::ThePlatform for NothingIsPostedHere {
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

    /// **Where the server is connected and Smoozy's service on it does not
    /// answer, the card's state line says the row's words, *Smoozy isn't
    /// responding.*, and never that Smoozy has not read the server yet** —
    /// `Design/Server Setup § This server`. **With the service answering, or
    /// the server not connected, the card is what it was.** The seventh pass
    /// on RunPod read *Smoozy has not read this server yet* under a header
    /// saying *Connected*. Step 10c's eighth-pass fix piece 6.
    #[test]
    fn a_connected_server_whose_service_does_not_answer_says_so_on_the_card() {
        use crate::record::failure_records::{ConditionKey, DurableSubject, FailureRecords};
        use crate::servers::a_period::{ThePeriodsThisRun, TheRunThroughTheApp};
        use smoozy_servers::acting::WhatServersPublishes;
        use smoozy_servers::condition::ObservedValue;
        use smoozy_servers::driving::APeriod;
        use smoozy_servers::period::{OpeningAct, WhoTookIt};
        use smoozy_state::identities::{Identity, IdentityKind};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-v1-card-{}-{}.sqlite3", std::process::id(), line!()));
        let _ = std::fs::remove_file(&path);
        let record = crate::record::LocalRecord::open_at(&path).expect("a record");
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let subject = DurableSubject::ServerRow(server.clone());
        let key = ConditionKey::over(&subject, crate::servers::published::THE_SUPERVISORS_KIND, None);
        FailureRecords::over(record.connection())
            .a_condition_was_raised(&key, &subject, &["the supervisor not answering"], true)
            .expect("raised");
        let started = std::sync::Arc::new(std::sync::Mutex::new(Started {
            the_sweep_left_behind: Vec::new(),
            record: Some(record),
            record_trouble: None,
            diary: None,
            declarations: smoozy_reporting::lines::Declarations::new(),
            what_modules_declare: crate::presentation::declarations::TheDeclarations::default(),
            said_once: crate::presentation::said_once::TheThingsSaidOnce::new(),
            run: None,
            the_previous_run: smoozy_state::run_mark::WhatTheLaunchFound::NoPreviousRun,

        }));
        let register = ThePeriodsThisRun::over(
            TheRunThroughTheApp::lending_the_record_of(std::sync::Arc::clone(&started)),
            std::sync::Arc::new(TheOneQueue::new()),
            std::sync::Arc::new(crate::notifications::TheNotificationService::on(Box::new(
                NothingIsPostedHere,
            ))),
        );
        let server = server.as_text();

        // **Not connected: the card says nothing was read, as before.**
        assert_eq!(
            what_is_said_with_nothing_lent(Some(&register), &server),
            WhatTheMachineSaid::NothingHasEverBeenReadFromIt
        );

        let period = APeriod::begun(
            &server,
            OpeningAct::TheUsersConnectAction,
            WhoTookIt::TheUser,
            std::time::SystemTime::UNIX_EPOCH,
        );
        register.a_period_has_begun(&server, &period);
        register
            .where_a_period_publishes()
            .the_observed_value(&server, ObservedValue::Connected);

        let said = what_is_said_with_nothing_lent(Some(&register), &server);
        assert_eq!(said, WhatTheMachineSaid::ConnectedAndSmoozyIsNotResponding);
        let card = the_tab(said, &server).the_card;
        assert_eq!(
            card.state,
            Prose::new(crate::surface::servers::words::SMOOZY_ISNT_RESPONDING)
        );
        assert_ne!(card.state, Prose::new(words::NEVER_READ));
        assert!(card.instead.is_none(), "the card still says why nothing was read");
        assert_eq!(card.the_action.name, Prose::new(words::SET_UP_AGAIN));
        assert!(!card.the_action.loud);

        // **The service answers, the condition goes, and nothing is claimed.**
        {
            let held = started.lock().expect("the start");
            FailureRecords::over(held.record.as_ref().expect("the record").connection())
                .the_observed_value_moved(&key, "the supervisor answering")
                .expect("withdrawn");
        }
        assert_eq!(
            what_is_said_with_nothing_lent(Some(&register), &server),
            WhatTheMachineSaid::NothingHasEverBeenReadFromIt
        );
        drop(register);
        drop(started);
        let _ = std::fs::remove_file(&path);
    }

    // ─────────────────────────────── `Server Setup` W3's warning, shown

    fn a_live_job(name: Option<&str>, state: LiveState) -> (Option<String>, LiveState) {
        (name.map(str::to_string), state)
    }

    /// **`Server Setup` W3's ready-action row: it warns where the run carries
    /// driver work or a restart and there is work to disturb, and nowhere
    /// else** — the silence as binding as the warning. Each live job is told
    /// the truth about itself, and **whether a render was running comes back
    /// beside the box**, which is what the running panel's sentence is drawn
    /// on. Added 2026-09-27.
    #[test]
    fn the_ready_action_warns_only_where_it_restarts_a_server_with_work_on_it() {
        let live = vec![
            a_live_job(Some("Kitchen — final"), LiveState::Rendering),
            a_live_job(Some("Bedroom test"), LiveState::Waiting),
        ];
        let (warning, a_render_was_running) =
            the_warning_before_the_ready_action(&live, true).expect("it warns");
        assert!(a_render_was_running);
        assert_eq!(warning.title, Prose::new(words::THIS_SERVER_HAS_WORK));
        assert_eq!(warning.go_ahead, Prose::new(words::SET_UP_ANYWAY));
        assert_eq!(
            warning
                .the_live_jobs
                .iter()
                .map(|line| (line.job.as_str().to_string(), line.said.clone()))
                .collect::<Vec<_>>(),
            vec![
                ("Kitchen — final".to_string(), Prose::new(words::RENDERING_AND_A_RESTART)),
                ("Bedroom test".to_string(), Prose::new(words::WAITING_AND_A_RESTART)),
            ]
        );

        // **Nothing to disturb, or nothing that disturbs: silence.**
        assert!(the_warning_before_the_ready_action(&[], true).is_none());
        assert!(the_warning_before_the_ready_action(&live, false).is_none());

        // **Work waiting only: the box, and no render to carry.**
        let (_, a_render_was_running) = the_warning_before_the_ready_action(
            &[a_live_job(Some("Bedroom test"), LiveState::Held)],
            true,
        )
        .expect("it warns");
        assert!(!a_render_was_running);

        // **A live job nothing names is still work**: the box is drawn with no
        // line for it rather than the restart going ahead unannounced.
        let (warning, a_render_was_running) =
            the_warning_before_the_ready_action(&[a_live_job(None, LiveState::Rendering)], true)
                .expect("it warns");
        assert!(warning.the_live_jobs.is_empty());
        assert!(a_render_was_running);
    }

    /// **`Queue & History` §17's live set, off one queue reading**: waiting,
    /// held and rendering are live and an ended job is not.
    #[test]
    fn the_live_set_is_every_job_the_machine_holds_that_has_not_ended() {
        use smoozy_queue_history::acting::adoption::TheMachinesWorkingState;
        use smoozy_queue_history::line::QueueState;
        use smoozy_queue_history::readings::{AJobAsTheMachineReadsIt, WhatTheEntryHasLive};
        use smoozy_state::identities::{Identity, IdentityKind};
        let job = || Identity::issue(IdentityKind::Job).expect("randomness");
        let operation = || Identity::issue(IdentityKind::Operation).expect("randomness");
        let (ours, theirs, held, ended) = (job(), job(), job(), job());
        let one = |job: &Identity, state: QueueState| AJobAsTheMachineReadsIt {
            job: job.clone(),
            state,
            live: WhatTheEntryHasLive::NoReadings,
            the_hold: None,
        };
        let reading = TheMachinesWorkingState {
            jobs: vec![
                one(&ours, QueueState::Rendering { operation: operation() }),
                one(&theirs, QueueState::Waiting),
                one(&held, QueueState::Held { hold: operation() }),
                one(&ended, QueueState::Ended { outcome: operation() }),
            ],
            the_admission_point_stands_at: None,
            notes: Vec::new(),
            the_history_copys_count: 0,
            the_history_copys_entries: Vec::new(),
        };
        let live = the_live_set_named(&reading, &mut |job| {
            (*job == ours).then(|| "Kitchen".to_string())
        });
        assert_eq!(
            live,
            vec![
                (Some("Kitchen".to_string()), LiveState::Rendering),
                (None, LiveState::Waiting),
                (None, LiveState::Held),
            ]
        );
    }

    /// **The page shows W3's warning where it is told one stands, and its *Set
    /// up anyway* asks for the ready action again as the act that has had its
    /// warning** — so the run goes ahead without warning twice, and *Not now*
    /// asks for nothing. **It runs the page code rather than reading it**:
    /// `web/set-up.js` is loaded under Node beside stand-ins for the modules
    /// it imports. **Before 2026-09-27 nothing drew the box at all.** Skipped,
    /// and says so, where no Node is installed.
    #[test]
    fn the_page_draws_the_warning_it_is_told_of_and_goes_ahead_only_when_asked() {
        let folder = std::env::temp_dir().join(format!(
            "smoozy-set-up-page-harness-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js");
        std::fs::copy(&page, folder.join("set-up.js")).expect("the page is copied");
        for (name, text) in THE_SET_UP_STAND_INS {
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        // **The box as the application hands it over**, in the shape the
        // command answers with — every word the application's own.
        let warning = format!(
            r#"{{"title":"{}","said":"{}","the_live_jobs":[{{"job":"Kitchen — final","said":"{}"}}],"not_now":"{}","go_ahead":"{}"}}"#,
            words::THIS_SERVER_HAS_WORK,
            words::SETTING_UP_RESTARTS,
            words::RENDERING_AND_A_RESTART,
            words::NOT_NOW,
            words::SET_UP_ANYWAY
        );
        std::fs::write(folder.join("the-warning.json"), warning).expect("the warning is kept");
        let ran = std::process::Command::new("node")
            .arg(folder.join("harness.js"))
            .current_dir(&folder)
            .output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Set up tab's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(
            ran.status.success(),
            "the page did not run: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&ran.stdout).trim(),
            format!(
                r#"{{"asked":["the_work_warning"],"title":"{}","lines":["Kitchen — final"],"afterNotNow":[],"afterGoingAhead":[{{"act":"{}","server":"server-1"}}]}}"#,
                words::THIS_SERVER_HAS_WORK,
                THE_READY_ACTION_AFTER_THE_WARNING
            )
        );
    }

    /// **`su-connect` — just after *Add and set up*, while the connection it
    /// asked for is being made, the card reads *Connecting…* and its action is
    /// under way on itself** (`Design/Server Setup § This server`, the
    /// Creator's note 3 on 0.1.7). The words are the drawing's own `suconnect`.
    /// **Before the piece** the card read *Smoozy has not read this server
    /// yet.* for the whole of that connection.
    #[test]
    fn the_card_while_an_add_connects_reads_connecting_with_its_action_under_way() {
        let card = the_card_while_an_add_connects();
        assert_eq!(card.state, Prose::new(words::CONNECTING));
        assert_eq!(card.dot, Verbatim::new("busy"));
        assert!(card.the_action.underway && card.the_action.loud);
        assert_eq!(card.the_action.name, Prose::new(words::SET_UP_THIS_SERVER));
        assert!(card.what_was_done.is_empty() && card.instead.is_none());
        let drawing = the_mockup();
        let drawn = drawing
            .split("id=\"suconnect\"")
            .nth(1)
            .and_then(|rest| rest.split("</div>\n              </div>").next())
            .expect("the drawing's connecting card");
        assert!(drawn.contains("<b>Connecting&#8230;</b>"), "{drawn}");
        assert_eq!(words::CONNECTING, "Connecting\u{2026}");
        assert!(drawn.contains(words::SET_UP_THIS_SERVER));
        // **The tab is told when the add's run is taken and after it is let
        // go**, so the card goes from *Connecting…* to the run and back.
        let source = include_str!("../setting_up.rs");
        let carried = source
            .split("pub fn the_remembered_setup_is_carried_out<R: tauri::Runtime>(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the remembered setup");
        let told = carried.find("crate::surface::set_up::the_run_under_way_is_told(&app, &server);").expect("the tab is told");
        // **From 2026-10-27 the run is let go by the line's own ending**
        // (`Server Setup` Y9a item 6), which hands on to an act waiting.
        let ended = carried.find("let follows = a_run_in_the_line_ends(&server,").expect("the run is let go");
        let answered = carried
            .find("the_run_is_told_what_it_came_to(&app, &server, came_to.as_ref(), follows, restarted)")
            .expect("the ending is told");
        assert!(told < ended && ended < answered, "the ending is told before the run is let go");
    }

    // ── The 38's piece 6: the card read from the machine ───────────────────

    /// A finished ready action, as the run's own reading carries it.
    fn the_finished_run_as_read() -> crate::setting_up::TheSetupAsRead {
        crate::setting_up::TheSetupAsRead {
            the_run_holds_the_machine: false,
            record: a_finished_run(WhatTheStepDid::Performed),
            still_open: false,
            the_step_in_flight: None,
            the_admin_step_recorded_as_started: None,
            the_clock: "Vast 4090".to_string(),
        }
    }

    /// The rest of the card's reading, with the last finished job's line and
    /// what it asked for.
    fn the_rest_with(
        line: Option<&str>,
        asked: Option<crate::setting_up::DeviceAsked>,
        role: Option<bool>,
    ) -> crate::setting_up::TheCardsReading {
        crate::setting_up::TheCardsReading {
            the_ready_actions_run: Some(
                smoozy_state::identities::Identity::issue(
                    smoozy_state::identities::IdentityKind::Operation,
                )
                .expect("randomness"),
            ),
            the_jobs: Some(crate::setting_up::TheJobsThatEnded {
                the_most_recent: line.map(|_| "job-1".to_string()),
                the_finished_job_with_a_line: line
                    .map(|line| ("job-1".to_string(), line.to_string(), asked)),
            }),
            the_role_is_enabled: role,
        }
    }

    const A_PROCESSOR_LINE: &str = "Path tracing on: AMD EPYC 7B13 64-Core Processor (CPU) (61 threads) [CPU]";
    const A_CARD_LINE: &str =
        "Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]";

    /// **The Creator's tenth notes, note 3: *Last job rendered on* names the
    /// device and its route, *NVIDIA GeForce RTX 4090 · OptiX*, in the four
    /// shapes the plans hold, and any other line whole** — `Design/Server
    /// Setup` line 56 as changed 2026-10-17, and the drawing's three lines.
    /// **The shapes are read by History's own reading**, the one *Ran on*
    /// takes, so the two cannot read a line differently. It fails if the card
    /// puts Cycles' whole printed line back, or guesses at a shape nobody has
    /// read.
    #[test]
    fn the_last_job_line_names_the_device_and_its_route_and_any_other_line_whole() {
        let the_line_for = |printed: &str| -> Words {
            let mut reading = a_reading();
            if let Some(device) = reading.the_device_reading.as_mut() {
                device.the_lines_words = printed.to_string();
            }
            the_card_from(&reading).the_render_device.expect("a device line")
        };
        for (printed, shown) in [
            (A_CARD_LINE, "NVIDIA GeForce RTX 4090 \u{b7} OptiX"),
            (
                "Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]",
                "NVIDIA GeForce RTX 4090 \u{b7} CUDA",
            ),
            (A_PROCESSOR_LINE, "AMD EPYC 7B13 64-Core Processor \u{b7} CPU"),
            (
                "Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]",
                "Apple M1 Pro (GPU - 14 cores) \u{b7} Metal",
            ),
            // **Any other shape, whole, as printed.**
            (
                "Path tracing on: AMD Radeon PRO W7900 (HIP) [HIP_AMD Radeon PRO W7900_0000:03:00]",
                "Path tracing on: AMD Radeon PRO W7900 (HIP) [HIP_AMD Radeon PRO W7900_0000:03:00]",
            ),
            ("Path tracing on: NVIDIA GeForce RTX 4090 (CUDA)", "Path tracing on: NVIDIA GeForce RTX 4090 (CUDA)"),
            ("Path tracing on: (OptiX) [CUDA_OptiX]", "Path tracing on: (OptiX) [CUDA_OptiX]"),
            ("Path tracing on: a card and a processor [CUDA_a card, CPU]", "Path tracing on: a card and a processor [CUDA_a card, CPU]"),
        ] {
            let line = the_line_for(printed);
            assert_eq!(line.lead, Prose::new(words::LAST_JOB_RENDERED_ON));
            assert_eq!(line.values, vec![Verbatim::new(shown)], "{printed}");
        }
        // **One reading, History's**: nothing here reads the line's shape.
        let working = include_str!("set_up.rs").split("#[cfg(test)]").next().unwrap_or_default();
        assert!(!working.contains("\"Path tracing on: \""), "a second copy of the reading");
        assert!(working.contains("the_history::the_device_and_its_route("));
    }

    /// **`Server Setup` W4: the last job asked for the GPU and rendered on the
    /// CPU — the loud line, and the action loud beside it.** Drawn from the
    /// machine's reading, which the run alone never carried. **Undone (the
    /// rest of the reading dropped, as before the piece), this fails**: the
    /// card drew no device line and no fault at all.
    #[test]
    fn a_last_job_asked_for_the_gpu_and_rendered_on_the_cpu_draws_the_loud_line() {
        use crate::setting_up::DeviceAsked;
        let tab = the_tab(
            WhatTheMachineSaid::ASetupRunAndTheMachine(
                Box::new(the_finished_run_as_read()),
                Box::new(the_rest_with(Some(A_PROCESSOR_LINE), Some(DeviceAsked::TheCards), Some(true))),
            ),
            "server-1",
        );
        let fault = tab.the_card.the_setup_fault.as_ref().expect("the setup fault is drawn");
        assert_eq!(fault.said, Prose::new(words::THE_LAST_JOB_ASKED_FOR_THE_GPU));
        assert!(tab.the_card.the_action.loud);
        assert!(tab.the_card.the_render_device.is_none(), "the fault stands in the line's place");
        // **`Server Setup` Y6b: the role's own reading draws the Telegram line**,
        // where the run's steps do not already say it (the twenty-first notes,
        // note 1).
        let mut without_the_step = the_finished_run_as_read();
        without_the_step.record.the_boundaries_reached.retain(|it| it.boundary != Boundary::Telegram);
        let shown = the_tab(
            WhatTheMachineSaid::ASetupRunAndTheMachine(
                Box::new(without_the_step),
                Box::new(the_rest_with(Some(A_PROCESSOR_LINE), Some(DeviceAsked::TheCards), Some(true))),
            ),
            "server-1",
        );
        assert!(shown
            .the_card
            .what_was_done
            .iter()
            .any(|line| line.said.lead == Prose::new(words::TELEGRAM_INSTALLED)));

        // **Asked for the processor**: the artist got what they asked for.
        let asked_cpu = the_tab(
            WhatTheMachineSaid::ASetupRunAndTheMachine(
                Box::new(the_finished_run_as_read()),
                Box::new(the_rest_with(Some(A_PROCESSOR_LINE), Some(DeviceAsked::TheProcessor), Some(false))),
            ),
            "server-1",
        );
        assert!(asked_cpu.the_card.the_setup_fault.is_none());
        assert!(!asked_cpu.the_card.the_action.loud);
        let said = asked_cpu.the_card.the_render_device.expect("the line is stated");
        assert_eq!(said.lead, Prose::new(words::LAST_JOB_RENDERED_ON));
        // **Shown as the device and its route** (the tenth notes, note 3).
        assert_eq!(said.values, vec![Verbatim::new("AMD EPYC 7B13 64-Core Processor \u{b7} CPU")]);
        assert!(!asked_cpu
            .the_card
            .what_was_done
            .iter()
            .any(|line| line.said.lead == Prose::new(words::TELEGRAM_INSTALLED)));

        // **What the job asked for could not be read**: the line is stated —
        // the device and its route, since the tenth notes, note 3 — and no
        // fault is claimed on a missing condition.
        let unknown = the_tab(
            WhatTheMachineSaid::ASetupRunAndTheMachine(
                Box::new(the_finished_run_as_read()),
                Box::new(the_rest_with(Some(A_PROCESSOR_LINE), None, None)),
            ),
            "server-1",
        );
        assert!(unknown.the_card.the_setup_fault.is_none());
        assert_eq!(
            unknown.the_card.the_render_device.expect("the line is stated").values,
            vec![Verbatim::new("AMD EPYC 7B13 64-Core Processor \u{b7} CPU")]
        );

        // **A card ran it, as asked**: *Last job rendered on …*, flat.
        let on_the_card = the_tab(
            WhatTheMachineSaid::ASetupRunAndTheMachine(
                Box::new(the_finished_run_as_read()),
                Box::new(the_rest_with(Some(A_CARD_LINE), Some(DeviceAsked::TheCards), None)),
            ),
            "server-1",
        );
        assert!(on_the_card.the_card.the_setup_fault.is_none());
        assert_eq!(
            on_the_card.the_card.the_render_device.expect("the line is stated").values,
            vec![Verbatim::new("NVIDIA GeForce RTX 4090 \u{b7} OptiX")]
        );
    }

    /// **`Server Setup` Y6a: a machine that answered the card's reading and
    /// holds no ready-action record is *Not set up*, loud** — with the
    /// Telegram line where the role is on (`Server Setup` G3). Not *No setup
    /// read*.
    #[test]
    fn a_machine_holding_no_ready_record_reads_not_set_up_with_its_telegram_line() {
        let mut rest = the_rest_with(None, None, Some(true));
        rest.the_ready_actions_run = None;
        let tab = the_tab(
            WhatTheMachineSaid::AReading(Box::new(the_reading_where_no_setup_record_stands(
                &rest,
                "Vast 4090".to_string(),
            ))),
            "server-1",
        );
        assert_eq!(tab.the_card.state, Prose::new(words::NOT_SET_UP));
        assert!(tab.the_card.the_action.loud);
        assert_eq!(
            tab.the_card.the_note,
            Some(plainly(words::TELEGRAM_AND_THE_REST_IS_NOT_RUN))
        );
    }

    /// **`Server Setup` Y6a and the 38's map, hole 7: disconnected in the same
    /// run, the card is what was last seen — greyed and stamped with when it
    /// was read; after a relaunch with no connection, it says the server has
    /// not been read yet, and never *No setup read*.** **Undone (nothing
    /// held), this fails**: the card read *Smoozy has not read this server
    /// yet.* the moment the connection went.
    #[test]
    fn disconnected_the_card_is_last_seen_and_after_a_relaunch_it_is_not_read_yet() {
        let server = "piece-6-last-seen";
        let read = WhatTheMachineSaid::ASetupRunAndTheMachine(
            Box::new(the_finished_run_as_read()),
            Box::new(the_rest_with(None, None, Some(false))),
        );
        let before = std::time::SystemTime::now();
        the_reading_is_held_as_last_seen(server, &read);
        let after = std::time::SystemTime::now();

        let said = what_was_last_seen_or(server, WhatTheMachineSaid::NothingHasEverBeenReadFromIt);
        let WhatTheMachineSaid::LastSeen(what_was_read, at) = &said else {
            panic!("not last seen: {said:?}");
        };
        assert_eq!(**what_was_read, read);
        assert!(*at >= before && *at <= after, "stamped with the moment it was read");
        let tab = the_tab(said, server);
        let connected = the_tab(read.clone(), server);
        assert_eq!(tab.the_card, connected.the_card, "the card is what was read");
        let stamp = tab.the_card_was_last_seen.expect("the card is stamped");
        assert_eq!(stamp.lead, Prose::new(crate::surface::servers::words::LAST_SEEN));
        assert_eq!(stamp.values.len(), 1);
        assert!(connected.the_card_was_last_seen.is_none());

        // **Smoozy's service not answering is its own card**, never a
        // last-seen one.
        assert_eq!(
            what_was_last_seen_or(server, WhatTheMachineSaid::ConnectedAndSmoozyIsNotResponding),
            WhatTheMachineSaid::ConnectedAndSmoozyIsNotResponding
        );

        // **A relaunch holds nothing**: a server never read in this run.
        let relaunched = the_tab(
            what_was_last_seen_or("piece-6-never-read-this-run", WhatTheMachineSaid::NothingHasEverBeenReadFromIt),
            "piece-6-never-read-this-run",
        );
        assert_eq!(relaunched.the_card.state, Prose::new(words::NEVER_READ));
        assert_ne!(relaunched.the_card.state, Prose::new(words::NO_SETUP_WAS_READ));
        assert!(relaunched.the_card_was_last_seen.is_none());
    }

    /// **A stopped setup last seen is the card's one line, without the
    /// panel** — the panel says its steps were read just now and both its
    /// controls act on the machine.
    #[test]
    fn a_stopped_setup_last_seen_draws_no_panel() {
        let stopped = WhatTheMachineSaid::ASetupRunAlone(Box::new(crate::setting_up::TheSetupAsRead {
            the_run_holds_the_machine: true,
            record: TheSetupRecord {
                kind: SetupKind::TheReadyAction,
                the_composed_list: vec![Boundary::Started, Boundary::DriverPlaced, Boundary::Done],
                the_boundaries_reached: vec![a_boundary(Boundary::Started, WhatTheStepDid::Performed)],
                ending: Some(HowTheRunEnded::LeftStopped { at: Some(Boundary::Started) }),
            },
            still_open: false,
            the_step_in_flight: None,
            the_admin_step_recorded_as_started: None,
            the_clock: "Vast 4090".to_string(),
        }));
        assert!(the_tab(stopped.clone(), "server-1").the_stopped_setup.is_some());
        let seen = the_tab(
            WhatTheMachineSaid::LastSeen(Box::new(stopped), std::time::SystemTime::now()),
            "server-1",
        );
        assert!(seen.the_stopped_setup.is_none());
        assert_eq!(seen.the_card.state, Prose::new(words::SETUP_STOPPED_PART_WAY));
        assert!(seen.the_card_was_last_seen.is_some());
    }

    /// **The page draws a last-seen card greyed, with *last seen* and the
    /// moment** — it runs the page code under Node, as the tests below do.
    #[test]
    fn the_page_greys_a_last_seen_card_and_stamps_it() {
        let seen = the_tab(
            WhatTheMachineSaid::LastSeen(
                Box::new(WhatTheMachineSaid::ASetupRunAndTheMachine(
                    Box::new(the_finished_run_as_read()),
                    Box::new(the_rest_with(None, None, Some(false))),
                )),
                std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_790_000_000),
            ),
            "server-1",
        );
        let now = the_tab(
            WhatTheMachineSaid::ASetupRunAndTheMachine(
                Box::new(the_finished_run_as_read()),
                Box::new(the_rest_with(None, None, Some(false))),
            ),
            "server-1",
        );
        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("await import(\"./set-up.js\");")
            .next()
            .expect("the harness's opening")
            .replace("THE_RUNNING", "null")
            .replace("THE_FINISHED", &serde_json::to_string(&seen).expect("serialises"))
            .replace("THE_STOPPED", "null")
            + &r#"await import("./set-up.js");
const host = anElement("div");
globalThis.tab.itIsDrawn(host);
globalThis.tab.itIsShowing("server-1");
await settle();
await theReadingLands(finished);
const [scroll] = host.children;
const [card] = scroll.children;
const theCard = () => every(card).find((one) => one.classList && one.classList.contains("sucard"));
const out = { seen: { stale: theCard().classList.contains("stale"), text: theTextOf(card) } };
globalThis.tab.itIsShowing("server-1");
await settle();
await theReadingLands(THE_NOW);
out.now = { stale: theCard().classList.contains("stale"), text: theTextOf(card) };
console.log(JSON.stringify(out));"#
                .replace("THE_NOW", &serde_json::to_string(&now).expect("serialises"));
        let folder = std::env::temp_dir().join(format!("smoozy-set-up-last-seen-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js");
        std::fs::copy(&page, folder.join("set-up.js")).expect("the page is copied");
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => harness.clone(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Set up tab's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let last_seen = Prose::new(crate::surface::servers::words::LAST_SEEN);
        assert_eq!(out["seen"]["stale"], true, "{out}");
        assert!(out["seen"]["text"].as_str().unwrap_or_default().contains(last_seen.as_str()), "{out}");
        assert!(out["seen"]["text"].as_str().unwrap_or_default().contains("2026"), "the moment is drawn: {out}");
        assert_eq!(out["now"]["stale"], false, "{out}");
        assert!(!out["now"]["text"].as_str().unwrap_or_default().contains(last_seen.as_str()), "{out}");
    }

    /// **The page draws each step mark in its colour, on the card and on a
    /// setup that stopped part-way** — it runs the page code under Node, as the
    /// tests beside it do (the Creator's note 3 on the test copy). Skipped, and
    /// says so, where no Node is installed.
    #[test]
    fn the_page_draws_each_step_mark_in_its_colour() {
        let mut reading = a_reading();
        reading.the_records = vec![a_finished_run(WhatTheStepDid::NothingToDo)];
        reading.the_library_half_was_read = false;
        let card = the_tab(WhatTheMachineSaid::AReading(Box::new(reading)), "server-1");
        let stopped = the_tab(
            WhatTheMachineSaid::ASetupRunAlone(Box::new(crate::setting_up::TheSetupAsRead {
                the_run_holds_the_machine: true,
                record: TheSetupRecord {
                    kind: SetupKind::TheReadyAction,
                    the_composed_list: vec![Boundary::Started, Boundary::DriverPlaced, Boundary::RestartIssued, Boundary::Done],
                    the_boundaries_reached: vec![
                        a_boundary(Boundary::Started, WhatTheStepDid::Performed),
                        a_boundary(Boundary::DriverPlaced, WhatTheStepDid::Performed),
                    ],
                    ending: Some(HowTheRunEnded::EndedAtBoundaryNotCompleted { at: Some(Boundary::DriverPlaced) }),
                },
                still_open: false,
                the_step_in_flight: Some(Boundary::RestartIssued),
                the_admin_step_recorded_as_started: None,
                the_clock: "Vast 4090".to_string(),
            })),
            "server-1",
        );
        assert!(stopped.the_stopped_setup.is_some());
        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("await import(\"./set-up.js\");")
            .next()
            .expect("the harness's opening")
            .replace("THE_RUNNING", "null")
            .replace("THE_FINISHED", &serde_json::to_string(&card).expect("serialises"))
            .replace("THE_STOPPED", &serde_json::to_string(&stopped).expect("serialises"))
            + r#"await import("./set-up.js");
const host = anElement("div");
globalThis.tab.itIsDrawn(host);
globalThis.tab.itIsShowing("server-1");
await settle();
const marks = () => every(host)
  .filter((one) => one.classList && one.classList.contains("stm"))
  .map((one) => [one.className, theTextOf(one)]);
await theReadingLands(finished);
const out = { card: marks() };
globalThis.tab.itIsShowing("server-1");
await settle();
await theReadingLands(stopped);
out.stopped = marks();
console.log(JSON.stringify(out));"#;
        let folder = std::env::temp_dir().join(format!("smoozy-set-up-mark-colours-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js");
        std::fs::copy(&page, folder.join("set-up.js")).expect("the page is copied");
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => harness.clone(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Set up tab's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let class_of = |list: &str, words: &str| -> Vec<String> {
            let words = Prose::new(words);
            out[list]
                .as_array()
                .map(|marks| {
                    marks
                        .iter()
                        .filter(|mark| mark[1].as_str() == Some(words.as_str()))
                        .filter_map(|mark| mark[0].as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default()
        };
        let with = |colour: &str| format!("stm {colour}");
        // The card: green *done*, grey *nothing to do*, amber *not checked*.
        assert_eq!(class_of("card", words::DONE), vec![with(the_marks_colour::DONE)], "{out}");
        let idle = class_of("card", words::NOTHING_TO_DO);
        assert!(!idle.is_empty() && idle.iter().all(|it| *it == with(the_marks_colour::NOTHING_TO_DO)), "{out}");
        assert_eq!(class_of("card", words::NOT_CHECKED_NO_BLENDER), vec![with(the_marks_colour::NOT_CHECKED)], "{out}");
        // A setup stopped part-way: green *done*, red where it stopped, and
        // *not started* plain.
        let done = class_of("stopped", words::DONE);
        assert!(!done.is_empty() && done.iter().all(|it| *it == with(the_marks_colour::DONE)), "{out}");
        assert_eq!(class_of("stopped", words::THE_CONNECTION_DROPPED_HERE), vec![with(the_marks_colour::FAILED)], "{out}");
        assert_eq!(class_of("stopped", words::NOT_STARTED), vec!["stm".to_string()], "{out}");
    }

    /// **A failed step draws *What the server said* only where there are the
    /// server's words to open behind it** — and a Telegram step that stopped
    /// on this computer has none, so its failure reads Smoozy's own sentence
    /// and carries no control. **It runs the page code**, with a failure shape
    /// that keeps what it was handed. Until 2026-10-18 the control was on every
    /// failed step (the Creator's twelfth notes, note 2). Skipped, and says so,
    /// where no Node is installed.
    #[test]
    fn the_page_offers_the_servers_words_only_where_there_are_some() {
        use smoozy_server_setup::recovery::TheFailedStepsWords;
        let stopped_at_telegram = |words: &str| {
            the_tab(
                WhatTheMachineSaid::ASetupRunAlone(Box::new(crate::setting_up::TheSetupAsRead {
                    the_run_holds_the_machine: true,
                    record: TheSetupRecord {
                        kind: SetupKind::TheReadyAction,
                        the_composed_list: vec![Boundary::Started, Boundary::Telegram, Boundary::Done],
                        the_boundaries_reached: vec![a_boundary(Boundary::Started, WhatTheStepDid::Performed)],
                        ending: Some(HowTheRunEnded::AStepFailed {
                            step: smoozy_server_setup::ready_action::Step::TelegramMaterial,
                            words: TheFailedStepsWords::SmoozysOwnVerification {
                                what_was_read: words.to_string(),
                            },
                        }),
                    },
                    still_open: false,
                    the_step_in_flight: Some(Boundary::Telegram),
                    the_admin_step_recorded_as_started: None,
                    the_clock: "3060".to_string(),
                })),
                "server-1",
            )
        };
        let own = crate::record::told_record::the_composing_sentences::THE_TOKEN_IS_NO_LONGER_STORED;
        const THE_MACHINES_WORDS: &str = "the role worker could not be started";
        let smoozys = stopped_at_telegram(own);
        let the_machines = stopped_at_telegram(THE_MACHINES_WORDS);
        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("await import(\"./set-up.js\");")
            .next()
            .expect("the harness's opening")
            .replace("THE_RUNNING", "null")
            .replace("THE_FINISHED", &serde_json::to_string(&smoozys).expect("serialises"))
            .replace("THE_STOPPED", &serde_json::to_string(&the_machines).expect("serialises"))
            + r#"await import("./set-up.js");
const host = anElement("div");
globalThis.tab.itIsDrawn(host);
globalThis.tab.itIsShowing("server-1");
await settle();
globalThis.shapes = [];
await theReadingLands(finished);
const out = { smoozys: globalThis.shapes.splice(0) };
globalThis.tab.itIsShowing("server-1");
await settle();
await theReadingLands(stopped);
out.theMachines = globalThis.shapes.splice(0);
console.log(JSON.stringify(out));"#;
        let folder = std::env::temp_dir().join(format!("smoozy-set-up-servers-words-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js");
        std::fs::copy(&page, folder.join("set-up.js")).expect("the page is copied");
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => harness.clone(),
                // **The failure shape keeps the sentence and the controls it
                // was handed**, which is the whole of what this page decides.
                "the-failure-shape.js" => r#"export const EXPANDED = "expanded";
export function theFailureShape(parts) {
  (globalThis.shapes ||= []).push({ sentence: parts.sentence, controls: (parts.controls || []).map((one) => one.label) });
  return document.createElement("div");
}"#
                .to_string(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Set up tab's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let the_shape = |drawn: &str| {
            let shapes = out[drawn].as_array().cloned().unwrap_or_default();
            assert_eq!(shapes.len(), 1, "{drawn}: not one failure shape: {out}");
            shapes[0].clone()
        };
        let smoozys = the_shape("smoozys");
        assert_eq!(smoozys["sentence"].as_str(), Some(Prose::new(own).as_str()), "{out}");
        assert_eq!(
            smoozys["controls"],
            serde_json::json!([]),
            "*What the server said* is offered on Smoozy's own sentence: {out}"
        );
        let the_machines = the_shape("theMachines");
        assert_eq!(
            the_machines["sentence"].as_str(),
            Some(Prose::new(words::FAILED_TELEGRAM).as_str()),
            "{out}"
        );
        assert_eq!(
            the_machines["controls"],
            serde_json::json!([Prose::new(words::WHAT_THE_SERVER_SAID).as_str()]),
            "the machine's words have no *What the server said*: {out}"
        );
    }

    /// **The eighteenth notes, notes 3 and 4 — the failure shows once, on the
    /// failed step, as the drawing's own line** (`Design/Server Setup § The
    /// step that failed`; the drawing's `su-step` and `su-stepten`).
    ///
    /// - **Said by the card**: a ready action that ended on a failed step — and
    ///   on the unpack, and on Telegram's material not composed — tells the
    ///   page the card says it, so no line goes above the steps; the endings
    ///   whose line says what the step does not keep it.
    /// - **Drawn once, on the step**: the Creator's stopped card of ten steps,
    ///   run through the real page and the real failure shape under Node,
    ///   carries one failure and one *What the server said*, under the failed
    ///   step, as `.stfail` — the dot, the sentence in bold and the control as
    ///   a link on the same line — and no `.qfail` block or pill anywhere.
    ///   The link opens the server's words.
    ///
    /// **Before the piece** the card drew the ending's sentence and its own
    /// *What the server said* above the steps, and the step's failure as the
    /// general block with a pill (the Creator's screenshot 1). Skipped, and
    /// says so, where no Node is installed.
    #[test]
    fn eighteenth_a_step_failure_shows_once_on_its_step_as_the_drawings_line() {
        use crate::setting_up::{TheRunsEnding, WhereTheSetUpGot};
        use smoozy_server_setup::ready_action::Step;
        use smoozy_server_setup::recovery::TheFailedStepsWords;
        let ended = |ending: TheRunsEnding| WhereTheSetUpGot::TheRunEnded { at: Boundary::Started, ending };
        const THE_PROGRAMS_SENTENCE: &str = "Download failed: HTTP 404.";
        let failed = ended(TheRunsEnding::AStepFailed { step: Step::Fetch, said: THE_PROGRAMS_SENTENCE.to_string() });
        for said_by_the_card in [
            failed.clone(),
            ended(TheRunsEnding::AStepFailed { step: Step::Driver, said: "E: dpkg was interrupted".to_string() }),
            ended(TheRunsEnding::TheUnpackFailed { said: "xz: (stdin): File format not recognized".to_string() }),
            ended(TheRunsEnding::TheTelegramMaterialWasNotComposed { said: "Smoozy's own sentence".to_string() }),
            // **The tool's own install**, from 2026-10-25: its step names the
            // install (`small_owed_the_unpack_tools_own_install_says_so_on_its_step`).
            ended(TheRunsEnding::AStepFailed { step: Step::Tooling, said: "E: Unable to locate package xz-utils".to_string() }),
        ] {
            assert!(the_card_says_the_ready_actions_ending(&said_by_the_card), "{said_by_the_card:?}");
        }
        for kept_as_a_line in [
            ended(TheRunsEnding::TheRestartWasRefused { said: "Failed to talk to init daemon.".to_string() }),
            ended(TheRunsEnding::TheServerStoppedAnswering { step: Step::Done, said: "silence".to_string() }),
            ended(TheRunsEnding::TheUserStoppedIt),
        ] {
            assert!(!the_card_says_the_ready_actions_ending(&kept_as_a_line), "{kept_as_a_line:?}");
        }

        // **The Creator's stopped card**: ten steps, the download failed with
        // the program's own sentence behind *What the server said*.
        let stopped = the_tab(
            WhatTheMachineSaid::ASetupRunAlone(Box::new(crate::setting_up::TheSetupAsRead {
                the_run_holds_the_machine: true,
                record: TheSetupRecord {
                    kind: SetupKind::TheReadyAction,
                    the_composed_list: vec![
                        Boundary::Started,
                        Boundary::Fetched,
                        Boundary::Tooling,
                        Boundary::Libraries,
                        Boundary::DriverPlaced,
                        Boundary::RestartIssued,
                        Boundary::CardAnswering,
                        Boundary::Blender,
                        Boundary::Telegram,
                        Boundary::Done,
                    ],
                    the_boundaries_reached: vec![a_boundary(Boundary::Started, WhatTheStepDid::NothingToDo)],
                    ending: Some(HowTheRunEnded::AStepFailed {
                        step: Step::Fetch,
                        words: TheFailedStepsWords::SmoozysOwnVerification {
                            what_was_read: THE_PROGRAMS_SENTENCE.to_string(),
                        },
                    }),
                },
                still_open: false,
                the_step_in_flight: Some(Boundary::Fetched),
                the_admin_step_recorded_as_started: None,
                the_clock: "RTX 3060".to_string(),
            })),
            "server-1",
        );
        assert!(stopped.the_stopped_setup.is_some());
        // **The ending as the ready action's own route tells it.**
        let the_ending = a_runs_ending(
            the_answer_about("server-1", &what_it_came_to(&failed)),
            the_card_says_the_ready_actions_ending(&failed),
        );
        let mut heard = serde_json::Map::new();
        for (name, said) in &delivered(the_ending).said {
            let text = match said {
                Said::Prose(prose) => prose.as_str().to_string(),
                Said::Value(value) => value.as_str().to_string(),
            };
            heard.insert(name.to_string(), serde_json::Value::String(text));
        }
        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("await import(\"./set-up.js\");")
            .next()
            .expect("the harness's opening")
            .replace("THE_RUNNING", "null")
            .replace("THE_FINISHED", &serde_json::to_string(&stopped).expect("serialises"))
            .replace("THE_STOPPED", "null")
            + &r#"await import("./set-up.js");
const host = anElement("div");
globalThis.tab.itIsDrawn(host);
globalThis.tab.itIsShowing("server-1");
await settle();
await theReadingLands(finished);
tell(THE_ENDING);
await settle();
await theReadingLands(finished);
const drawn = every(host);
const named = (name) => drawn.filter((one) => one.classList && one.classList.contains(name));
const theSaid = drawn.filter((one) => (one.tag === "a" || one.tag === "button") && theTextOf(one) === WHAT_THE_SERVER_SAID);
const [line] = named("stfail");
const out = {
  stfail: named("stfail").length,
  qfail: named("qfail").length,
  said: theSaid.map((one) => one.tag),
  steps: drawn.filter((one) => one.tag === "li").length,
};
if (line) {
  out.onTheFailedStep = line.parentNode.tag === "li" && line.parentNode.classList.contains("bad");
  out.line = line.children.map((one) => [one.tag, one.className, theTextOf(one)]);
  line.children.find((one) => one.tag === "a").press();
  out.opened = (globalThis.panels || []).slice();
}
console.log(JSON.stringify(out));"#
                .replace("THE_ENDING", &serde_json::Value::Object(heard).to_string())
                .replace(
                    "WHAT_THE_SERVER_SAID",
                    &serde_json::to_string(Prose::new(words::WHAT_THE_SERVER_SAID).as_str()).expect("serialises"),
                );
        let folder = std::env::temp_dir().join(format!("smoozy-set-up-eighteenth-once-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js");
        std::fs::copy(&page, folder.join("set-up.js")).expect("the page is copied");
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => harness.clone(),
                // **The real failure shape**, which is what draws the line.
                "the-failure-shape.js" => std::fs::read_to_string(
                    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/the-failure-shape.js"),
                )
                .expect("the failure shape is readable"),
                "the-machines-own-words.js" => r#"export function theWordsPanel(parts) { (globalThis.panels ||= []).push(parts.title); return document.createElement("div"); }
export function theNewestWordsStayInView() {}"#
                    .to_string(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Set up tab's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let said = Prose::new(words::WHAT_THE_SERVER_SAID).as_str().to_string();
        assert_eq!(out["steps"], 10, "{out}");
        assert_eq!(out["stfail"], 1, "the failure is not one line on the step: {out}");
        assert_eq!(out["qfail"], 0, "a failure block is drawn on the card: {out}");
        assert_eq!(out["said"], serde_json::json!(["a"]), "*What the server said* is not once, as a link: {out}");
        assert_eq!(out["onTheFailedStep"], true, "{out}");
        assert_eq!(
            out["line"],
            serde_json::json!([
                ["span", "fdot", ""],
                ["b", "selectable", Prose::new(words::FAILED_FETCH).as_str()],
                ["a", "", said.as_str()],
            ]),
            "{out}"
        );
        assert_eq!(out["opened"], serde_json::json!([said.as_str()]), "the link opens no words: {out}");
    }

    /// **The Creator's notes 4, 6 and 8 on 0.1.7, and it runs the page code:
    /// a run is drawn on the card, never in a box, and the Blender table alone
    /// carries the mark while it is read again.** `web/set-up.js` is loaded
    /// under Node beside the stand-ins above, with the readings the application
    /// composes:
    ///
    /// - a run under way draws **no `.modal`** anywhere on the tab — nothing
    ///   covers the server's screen, so the tab strip stays pressable — and
    ///   draws its steps in the card's own section with *Cancel setup* its one
    ///   control, which sends the stop;
    /// - the run's ending, a Blender installed, covers **the table and only
    ///   the table** until the next reading lands, and draws no line the card
    ///   already says;
    /// - the finished card lists the run's steps;
    /// - a setup that stopped part-way is the card, with its two ways out, and
    ///   a stopped act's own sentence is one line on it.
    ///
    /// **Before the piece** the run and the stopped setup each opened a
    /// `.modal top` box, the run's carried *Stop* and *Close*, and the table
    /// was never covered. Skipped, and says so, where no Node is installed.
    #[test]
    fn the_run_is_drawn_on_the_card_and_only_the_table_is_covered_while_it_is_read_again() {
        let running = {
            let read = crate::setting_up::TheSetupAsRead {
                the_run_holds_the_machine: true,
                record: TheSetupRecord {
                    kind: SetupKind::ABlenderInstall,
                    the_composed_list: vec![
                        Boundary::Started,
                        Boundary::Fetched,
                        Boundary::Tooling,
                        Boundary::Libraries,
                        Boundary::Blender,
                        Boundary::Done,
                    ],
                    the_boundaries_reached: vec![
                        a_boundary(Boundary::Started, WhatTheStepDid::Performed),
                        a_boundary(Boundary::Fetched, WhatTheStepDid::Performed),
                    ],
                    ending: None,
                },
                still_open: true,
                the_step_in_flight: Some(Boundary::Tooling),
                the_admin_step_recorded_as_started: None,
                the_clock: String::new(),
            };
            the_panel_of_a_run_under_way(&read, "Vast 4090", false)
        };
        let mut reading = a_reading();
        reading.the_records = vec![a_finished_run(WhatTheStepDid::Performed)];
        let finished = the_tab(WhatTheMachineSaid::AReading(Box::new(reading)), "server-1");
        let stopped = the_tab(
            WhatTheMachineSaid::ASetupRunAlone(Box::new(crate::setting_up::TheSetupAsRead {
                the_run_holds_the_machine: true,
                record: TheSetupRecord {
                    kind: SetupKind::TheReadyAction,
                    the_composed_list: vec![Boundary::Started, Boundary::DriverPlaced, Boundary::Done],
                    the_boundaries_reached: vec![a_boundary(Boundary::Started, WhatTheStepDid::Performed)],
                    ending: Some(HowTheRunEnded::AStepFailed {
                        step: smoozy_server_setup::ready_action::Step::Driver,
                        words: TheFailedStepsWords::None,
                    }),
                },
                still_open: false,
                the_step_in_flight: Some(Boundary::DriverPlaced),
                the_admin_step_recorded_as_started: None,
                the_clock: "Vast 4090".to_string(),
            })),
            "server-1",
        );
        assert!(stopped.the_stopped_setup.is_some());
        let installed = a_runs_ending(
            the_answer_about("server-1", &WhatItCameTo::only(format!("Blender 5.2.2 {}", words::BLENDER_IS_INSTALLED))),
            true,
        );
        let a_stop = a_runs_ending(the_answer_about("server-1", &WhatItCameTo::only(words::SETUP_STOPPED)), false);
        let as_the_page_hears = |event: Event| {
            let mut out = serde_json::Map::new();
            for (name, said) in &delivered(event).said {
                let text = match said {
                    Said::Prose(prose) => prose.as_str().to_string(),
                    Said::Value(value) => value.as_str().to_string(),
                };
                out.insert(name.to_string(), serde_json::Value::String(text));
            }
            serde_json::Value::Object(out).to_string()
        };

        let folder = std::env::temp_dir().join(format!("smoozy-set-up-run-card-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js");
        std::fs::copy(&page, folder.join("set-up.js")).expect("the page is copied");
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => THE_RUN_ON_THE_CARD_HARNESS
                    .replace("THE_RUNNING", &serde_json::to_string(&running).expect("serialises"))
                    .replace("THE_FINISHED", &serde_json::to_string(&finished).expect("serialises"))
                    .replace("THE_STOPPED", &serde_json::to_string(&stopped).expect("serialises"))
                    .replace("THE_INSTALLED", &as_the_page_hears(installed.clone()))
                    .replace("THE_STOPS_ENDING", &as_the_page_hears(a_stop.clone())),
                "the-underway-shape.js" => r#"export function theMarkOnAControl(control) { control.classList.add("uway"); return control; }
export function theMarkOnARow() { return document.createElement("i"); }
export function theMarkOverAPane() { const cover = document.createElement("div"); cover.className = "underway-pane"; return cover; }"#
                    .to_string(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Set up tab's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let prose = |words: &str| serde_json::Value::String(Prose::new(words).as_str().to_string());
        assert_eq!(out["running"]["modals"], 0, "a run opened a box: {out}");
        assert_eq!(out["running"]["state"], prose(words::SETTING_UP), "{out}");
        assert_eq!(out["running"]["steps"], 6, "{out}");
        assert_eq!(out["running"]["buttons"], serde_json::json!([Prose::new(words::CANCEL_SETUP).as_str()]), "{out}");
        assert_eq!(out["afterCancel"], serde_json::json!(["stop"]), "{out}");
        assert_eq!(
            out["afterTheEnding"],
            serde_json::json!({ "tableCovered": true, "cardCovered": false, "modals": 0, "lines": 0 }),
            "{out}"
        );
        assert_eq!(out["afterTheReading"]["tableCovered"], false, "{out}");
        assert!(out["afterTheReading"]["listed"].as_u64().unwrap_or(0) > 1, "the card lists the steps: {out}");
        assert_eq!(out["stopped"]["modals"], 0, "{out}");
        assert_eq!(out["stopped"]["state"], prose(words::SETUP_STOPPED_PART_WAY), "{out}");
        assert_eq!(
            out["stopped"]["buttons"],
            serde_json::json!([Prose::new(words::ABANDON_SETUP).as_str(), Prose::new(words::SET_UP_AGAIN).as_str()]),
            "{out}"
        );
        assert_eq!(out["stopped"]["theLine"], prose(words::SETUP_STOPPED), "a stopped act says so on the card: {out}");
    }

    /// **A server's Set up tab never shows another server's card or table
    /// while its own reading is taken** — after 0.1.8, the checks' leftovers,
    /// item 3. The re-check before 0.1.8 saw a new server's tab read *Nothing
    /// to prepare … on the clock of Hyperstack A6000*, with the other
    /// machine's Blender table, for 11 seconds while its own reading was
    /// taken: the tab is one host for every server, and arriving at a second
    /// server covered the first one's drawing with the translucent mark
    /// rather than taking it away. It runs the page code, as the test above.
    #[test]
    fn another_servers_card_is_not_left_under_the_mark_while_this_ones_is_read() {
        let mut reading = a_reading();
        reading.the_records = vec![a_finished_run(WhatTheStepDid::Performed)];
        let first = the_tab(WhatTheMachineSaid::AReading(Box::new(reading.clone())), "server-1");
        let second = the_tab(WhatTheMachineSaid::AReading(Box::new(reading)), "server-2");
        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("await import(\"./set-up.js\");")
            .next()
            .expect("the harness's opening")
            .replace("THE_RUNNING", "null")
            .replace("THE_FINISHED", &serde_json::to_string(&first).expect("serialises"))
            .replace("THE_STOPPED", "null")
            + &r#"await import("./set-up.js");
const host = anElement("div");
globalThis.tab.itIsDrawn(host);
globalThis.tab.itIsShowing("server-1");
await settle();
await theReadingLands(finished);
const [scroll] = host.children;
const [card, table] = scroll.children;
const out = { first: { card: theTextOf(card).length, table: theTextOf(table).length } };
globalThis.tab.itIsShowing("server-2");
await settle();
out.whileTheSecondIsRead = { card: theTextOf(card), table: theTextOf(table) };
await theReadingLands(THE_SECOND);
out.second = { card: theTextOf(card).length, table: theTextOf(table).length };
console.log(JSON.stringify(out));"#
                .replace("THE_SECOND", &serde_json::to_string(&second).expect("serialises"));

        let folder = std::env::temp_dir().join(format!("smoozy-set-up-other-server-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js");
        std::fs::copy(&page, folder.join("set-up.js")).expect("the page is copied");
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => harness.clone(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Set up tab's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        assert!(out["first"]["card"].as_u64().unwrap_or(0) > 0, "{out}");
        assert!(out["first"]["table"].as_u64().unwrap_or(0) > 0, "{out}");
        assert_eq!(
            out["whileTheSecondIsRead"],
            serde_json::json!({ "card": "", "table": "" }),
            "the first server's drawing stands on the second server's tab: {out}"
        );
        assert!(out["second"]["card"].as_u64().unwrap_or(0) > 0, "{out}");
        assert!(out["second"]["table"].as_u64().unwrap_or(0) > 0, "{out}");
    }

    /// **The runner's rulings (a) and (b) of 2026-10-03, and it runs the page
    /// code.** Out of the Vast pass before 0.1.9, which watched two things the
    /// plans were silent on:
    ///
    /// - **(b)** between a run's ending and its result being read, the card
    ///   went back to the reading taken before the run — *No setup read from
    ///   this server* for 10 seconds. **It keeps the steps it already showed**,
    ///   with nothing only a running setup offers, until a reading asked after
    ///   the ending replaces them;
    /// - **(a)** *Install stopped.* left the card 9 seconds after the stop with
    ///   no press, when the ending moved the work column and the server's
    ///   screen showed the tab again. **It stays while it is true** — through
    ///   the screen showing the tab again, the result's reading, and a visit to
    ///   another server — **until the next setup act on that server replaces
    ///   it**, never on a timer.
    ///
    /// Skipped, and says so, where no Node is installed.
    #[test]
    fn an_ended_runs_steps_and_its_line_stay_until_what_replaces_them() {
        let running = {
            let read = crate::setting_up::TheSetupAsRead {
                the_run_holds_the_machine: true,
                record: TheSetupRecord {
                    kind: SetupKind::ABlenderInstall,
                    the_composed_list: vec![
                        Boundary::Started,
                        Boundary::Fetched,
                        Boundary::Tooling,
                        Boundary::Libraries,
                        Boundary::Blender,
                        Boundary::Done,
                    ],
                    the_boundaries_reached: vec![
                        a_boundary(Boundary::Started, WhatTheStepDid::Performed),
                        a_boundary(Boundary::Fetched, WhatTheStepDid::Performed),
                    ],
                    ending: None,
                },
                still_open: true,
                the_step_in_flight: Some(Boundary::Tooling),
                the_admin_step_recorded_as_started: None,
                the_clock: String::new(),
            };
            the_panel_of_a_run_under_way(&read, "Vast 4090", false)
        };
        // **The reading taken before the run, as the Vast pass met it.**
        let before = the_tab(WhatTheMachineSaid::ConnectedAndNoRunWasFound, "server-1");
        let mut reading = a_reading();
        reading.the_records = vec![a_finished_run(WhatTheStepDid::Performed)];
        let after = the_tab(WhatTheMachineSaid::AReading(Box::new(reading.clone())), "server-1");
        let elsewhere = the_tab(WhatTheMachineSaid::AReading(Box::new(reading)), "server-2");
        let the_ending = {
            let event =
                a_runs_ending(the_answer_about("server-1", &WhatItCameTo::only(words::INSTALL_STOPPED)), false);
            let mut out = serde_json::Map::new();
            for (name, said) in &delivered(event).said {
                let text = match said {
                    Said::Prose(prose) => prose.as_str().to_string(),
                    Said::Value(value) => value.as_str().to_string(),
                };
                out.insert(name.to_string(), serde_json::Value::String(text));
            }
            serde_json::Value::Object(out).to_string()
        };
        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("await import(\"./set-up.js\");")
            .next()
            .expect("the harness's opening")
            .replace("THE_RUNNING", &serde_json::to_string(&running).expect("serialises"))
            .replace("THE_FINISHED", &serde_json::to_string(&after).expect("serialises"))
            .replace("THE_STOPPED", &serde_json::to_string(&before).expect("serialises"))
            + &r#"await import("./set-up.js");
const host = anElement("div");
globalThis.tab.itIsDrawn(host);
globalThis.tab.itIsShowing("server-1");
await settle();
await theReadingLands(stopped);
const [scroll] = host.children;
const [card] = scroll.children;
const steps = () => every(card).filter((one) => one.tag === "li").length;
const buttons = () => every(card).filter((one) => one.tag === "button").map((one) => one.text);
const out = { before: theTextOf(card) };
theRun = running;
tell({ server: "server-1", the_run_is_under_way: "yes" });
await settle();
out.running = steps();
theRun = null;
tell(THE_ENDING);
await settle();
out.whileTheResultIsRead = { text: theTextOf(card), steps: steps(), buttons: buttons() };
// The ending moves the work column, and the server's screen shows the tab again.
globalThis.tab.itIsShowing("server-1");
await settle();
out.afterTheScreenShowedTheTabAgain = { text: theTextOf(card), steps: steps() };
await theReadingLands(finished);
out.afterTheResult = { text: theTextOf(card), buttons: buttons() };
globalThis.tab.itIsShowing("server-1");
await settle();
await theReadingLands(finished);
out.afterAnotherReading = theTextOf(card);
globalThis.tab.itIsShowing("server-2");
await settle();
await theReadingLands(THE_ELSEWHERE);
out.onAnotherServer = theTextOf(card);
globalThis.tab.itIsShowing("server-1");
await settle();
await theReadingLands(finished);
out.backAgain = theTextOf(card);
const action = every(card).find((one) => one.tag === "button" && one.text === THE_ACTION);
action.press();
await settle();
out.afterANewAct = { text: theTextOf(card), acts: acts.slice() };
console.log(JSON.stringify(out));"#
                .replace("THE_ENDING", &the_ending)
                .replace("THE_ELSEWHERE", &serde_json::to_string(&elsewhere).expect("serialises"))
                .replace(
                    "THE_ACTION",
                    &serde_json::to_string(Prose::new(words::SET_UP_AGAIN).as_str()).expect("serialises"),
                );

        let folder = std::env::temp_dir().join(format!("smoozy-set-up-ended-run-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js");
        std::fs::copy(&page, folder.join("set-up.js")).expect("the page is copied");
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => harness.clone(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Set up tab's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let text = |at: &serde_json::Value| at.as_str().unwrap_or_default().to_string();
        let never_read = Prose::new(words::NO_SETUP_WAS_READ).as_str().to_string();
        let stopped = Prose::new(words::INSTALL_STOPPED).as_str().to_string();
        assert!(text(&out["before"]).contains(&never_read), "the reading before the run: {out}");
        let running_steps = out["running"].as_u64().unwrap_or(0);
        assert!(running_steps > 1, "{out}");

        // (b) The ended run's steps, until the result is read.
        for at in ["whileTheResultIsRead", "afterTheScreenShowedTheTabAgain"] {
            assert!(!text(&out[at]["text"]).contains(&never_read), "{at}: the card went back to the reading before the run: {out}");
            assert_eq!(out[at]["steps"].as_u64(), Some(running_steps), "{at}: {out}");
        }
        assert!(
            !out["whileTheResultIsRead"]["buttons"]
                .as_array()
                .map(|all| all.iter().any(|one| one.as_str() == Some(Prose::new(words::CANCEL_SETUP).as_str())))
                .unwrap_or(true),
            "an ended run offers no Cancel setup: {out}"
        );
        assert!(
            !text(&out["afterTheResult"]["text"]).contains(&never_read),
            "the result replaced the steps: {out}"
        );

        // (a) The line, while it is true.
        for at in ["whileTheResultIsRead", "afterTheScreenShowedTheTabAgain"] {
            assert!(text(&out[at]["text"]).contains(&stopped), "{at}: {out}");
        }
        for at in ["afterTheResult", "afterAnotherReading", "backAgain"] {
            let said = if at == "afterTheResult" { text(&out[at]["text"]) } else { text(&out[at]) };
            assert!(said.contains(&stopped), "{at}: the line left with no act: {out}");
        }
        assert!(!text(&out["onAnotherServer"]).contains(&stopped), "another server's card: {out}");
        assert!(!text(&out["afterANewAct"]["text"]).contains(&stopped), "the next act replaces it: {out}");
        assert_eq!(out["afterANewAct"]["acts"], serde_json::json!([THE_READY_ACTION]), "{out}");
    }

    /// **The Creator's note 14 on the test copy: where the connection an add
    /// asked for was not made, the card says so** — the sentence the connect
    /// ended in, the red dot, and *Connect and set up*, the row's own act
    /// (`Design/Server Setup § This server`). **And only there**: a server
    /// with no failure kept, a failure on a row that remembers no setup, a
    /// `what` outside the three, and a connect being made all leave the card
    /// as it was. **Before the piece** the card read *Smoozy has not read this
    /// server yet.* after a wrong key, with nothing saying why.
    #[test]
    fn where_an_adds_connection_was_not_made_the_card_says_so() {
        let never = || the_tab(WhatTheMachineSaid::NothingHasEverBeenReadFromIt, "server-1");
        let not_made = crate::surface::servers::THE_CONNECTION_WAS_NOT_MADE_ANSWER;

        let mut tab = never();
        the_card_where_no_connection_stands(&mut tab, false, false, false, Some(not_made), true);
        let card = &tab.the_card;
        assert_eq!(card.dot, Verbatim::new("bad"));
        assert_eq!(
            card.state,
            Prose::new(crate::surface::servers::words::THE_CONNECTION_WAS_NOT_MADE)
        );
        assert_eq!(card.the_action.name, Prose::new(words::CONNECT_AND_SET_UP));
        assert_eq!(card.the_action.act, Verbatim::new(CONNECT_AND_SET_UP));
        assert!(card.the_action.loud && !card.the_action.underway);
        assert!(card.instead.is_none() && card.what_was_done.is_empty() && card.the_note.is_none());

        // **A cause `Keys` or `Servers` Q13 has its own sentence for is said
        // in that sentence**, as All servers says it.
        for (what, said) in [
            (
                crate::surface::servers::IT_IS_NOT_A_LINUX_MACHINE_ANSWER,
                crate::surface::servers::words::IT_IS_NOT_A_LINUX_MACHINE,
            ),
            (
                crate::surface::servers::THE_PASSPHRASE_COULD_NOT_BE_USED_ANSWER,
                crate::surface::servers::THE_PASSPHRASE_SENTENCE_ON_THIS_SYSTEM,
            ),
        ] {
            let mut tab = never();
            the_card_where_no_connection_stands(&mut tab, false, false, false, Some(what), true);
            assert_eq!(tab.the_card.state, Prose::new(said), "{what}");
            assert_eq!(tab.the_card.dot, Verbatim::new("bad"), "{what}");
        }

        let plain = never().the_card;
        for (connecting, failed, remembered) in [
            (false, None, true),
            (false, Some(not_made), false),
            (false, Some("the-record-would-not-take-it"), true),
            (false, Some("Could not connect to this server."), true),
        ] {
            let mut tab = never();
            the_card_where_no_connection_stands(&mut tab, connecting, false, false, failed, remembered);
            assert_eq!(tab.the_card, plain, "{failed:?}, remembers a setup: {remembered}");
        }

        // **A connect being made is the next act**, so it outranks the one
        // that was not made.
        let mut tab = never();
        the_card_where_no_connection_stands(&mut tab, true, false, false, Some(not_made), true);
        assert_eq!(tab.the_card, the_card_while_an_add_connects());

        // **A card last seen earlier in the run gives way whole**: no stamp
        // stays on a card that was not read.
        let mut tab = the_tab(
            WhatTheMachineSaid::LastSeen(
                Box::new(WhatTheMachineSaid::ASetupRunAndTheMachine(
                    Box::new(the_finished_run_as_read()),
                    Box::new(the_rest_with(None, None, Some(false))),
                )),
                std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_790_000_000),
            ),
            "server-1",
        );
        assert!(tab.the_card_was_last_seen.is_some());
        the_card_where_no_connection_stands(&mut tab, false, false, false, Some(not_made), true);
        assert_eq!(tab.the_card.dot, Verbatim::new("bad"));
        assert!(tab.the_card_was_last_seen.is_none() && tab.the_stopped_setup.is_none());

        // **The tab reaches it only for a server not read now**, and the card's
        // act is handed to `Servers` whole.
        let working = include_str!("set_up.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the working half");
        let command = working
            .split("pub async fn the_set_up_tab<R: tauri::Runtime>(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the command");
        let reached = command
            .find("if nothing_was_read || waits {")
            .expect("only where nothing was read, or an add's setup waits");
        let weighed = command
            .find("the_card_where_no_connection_stands(\n            &mut tab,\n            connecting,\n            waits,\n            stopped,\n            failed.as_deref(),\n            remembered,\n        );")
            .expect("the failure is weighed");
        assert!(reached < weighed);
        let acts = working
            .split("pub fn a_set_up_act_that_needs_a_connection<R: tauri::Runtime>(")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the acts");
        assert!(
            acts.contains("(CONNECT_AND_SET_UP, _) => {")
                && acts.contains("crate::surface::servers::the_act_is_performed(&app, &act, &server)"),
            "the card's Connect and set up is the row's own act"
        );
    }

    /// **The Creator's fourth note 1, piece M: from *Add and set up* until the
    /// run takes the card, the card reads *Connecting…* with its action under
    /// way** — `Design/Server Setup § This server`: *"while the connection it
    /// asked for is being made, the state line reads Connecting… … Then the
    /// run's steps take the card."* Walked through each moment with the
    /// transport's own memory and the run's own exclusion:
    ///
    /// - **at the press**, the connect mark stands and the row remembers;
    /// - **after the login**, `Servers` B5 has cleared the row's bit and the
    ///   mark has let go, nothing is lent, and the setup waits in the
    ///   transport. **Before the piece** the card read *Smoozy has not read
    ///   this server yet.* here, with *Set up this server* pressable;
    /// - **at the hand-over**, the setup still waits while step 8 runs, and
    ///   once it has, the run has the card and nothing waits;
    /// - **where the period ends first**, or a later login of it carries no
    ///   setup, nothing waits and *Connecting…* goes.
    #[test]
    fn an_adds_card_reads_connecting_from_the_press_until_the_run_takes_it() {
        use crate::servers::a_period::{the_login_carried, the_setup_is_handed_over};
        use crate::setting_up::an_adds_setup_waits_on;
        let connecting = the_card_while_an_add_connects();
        let never = |server: &str| the_tab(WhatTheMachineSaid::NothingHasEverBeenReadFromIt, server);
        let the_card_with = |server: &str, mark: bool, remembered: bool| {
            let mut tab = never(server);
            the_card_where_no_connection_stands(&mut tab, mark, an_adds_setup_waits_on(server), false, None, remembered);
            tab.the_card
        };
        let ticked = vec![crate::surface::servers::INSTALL_THE_LATEST_STABLE_BLENDER.to_string()];

        // **At the press.**
        let server = "server-m-walk";
        let held: crate::setting_up::TheSetupHeldForTheHandOver = Default::default();
        assert_eq!(the_card_with(server, true, true), connecting);

        // **After the login, with nothing lent.**
        the_login_carried(&held, server, Some(ticked.clone()));
        assert!(an_adds_setup_waits_on(server));
        let card = the_card_with(server, false, false);
        assert_eq!(card, connecting, "{card:?}");
        assert!(card.the_action.underway, "nothing on the card can be pressed");
        assert!(card.instead.is_none());

        // **The hand-over: the setup waits while step 8 runs, and the run has
        // the card when it is done.**
        let mut handed = None;
        the_setup_is_handed_over(&held, |remembered| {
            assert!(an_adds_setup_waits_on(server), "still Connecting… while step 8 runs");
            handed = remembered;
            assert!(crate::setting_up::a_run_begins(server, SetupKind::TheReadyAction));
        });
        assert_eq!(handed, Some(ticked.clone()), "the add's ticks reach step 8 whole");
        assert!(!an_adds_setup_waits_on(server));
        assert_eq!(what_the_run_is_called_on(server), Some(words::SETTING_UP));
        let panel = the_run_before_its_first_reading(words::SETTING_UP, false);
        assert!(panel.the_state_is_underway && panel.cancel.is_some());
        let mut again = Some(Vec::new());
        the_setup_is_handed_over(&held, |remembered| again = remembered);
        assert_eq!(again, None, "handed over exactly once");
        crate::setting_up::a_run_ends(server);

        // **The period ends before the hand-over**: the transport goes with
        // it, and so does *Connecting…*.
        let server = "server-m-period-ends";
        let held: crate::setting_up::TheSetupHeldForTheHandOver = Default::default();
        the_login_carried(&held, server, Some(ticked.clone()));
        assert_eq!(the_card_with(server, false, false), connecting);
        drop(held);
        assert!(!an_adds_setup_waits_on(server));
        let card = the_card_with(server, false, false);
        assert_ne!(card.state, Prose::new(words::CONNECTING), "{card:?}");
        assert!(!card.the_action.underway);

        // **A later login of the same period that carries no setup** leaves
        // nothing waiting, and nothing is ever handed over from it.
        let server = "server-m-a-later-login";
        let held: crate::setting_up::TheSetupHeldForTheHandOver = Default::default();
        the_login_carried(&held, server, Some(ticked));
        the_login_carried(&held, server, None);
        assert!(!an_adds_setup_waits_on(server));
        assert_ne!(the_card_with(server, false, false), connecting);

        // **A plain *Connect*** carries no setup and never reads *Connecting…*
        // on this card.
        let server = "server-m-plain-connect";
        let held: crate::setting_up::TheSetupHeldForTheHandOver = Default::default();
        the_login_carried(&held, server, None);
        assert_ne!(the_card_with(server, true, false), connecting);

        // **The transport keeps the memory this reads**: the login fills it,
        // and step 8 is handed it inside the hand-over that lets it go.
        let period = include_str!("../servers/a_period.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the working half");
        assert!(period.contains(
            "the_login_carried(&self.the_remembered_setup, &self.the_server, carried);"
        ));
        assert!(period.contains(
            "the_setup_is_handed_over(the_remembered_setup, |remembered| {\n                when_it_is_lent(sessions, remembered)\n            });"
        ));
        assert!(!period.contains("held.take());"), "nothing takes the setup before the hand-over");
    }

    /// **The Creator's fourth note 1, piece M2: an add whose login worked and
    /// which the first checks then stopped says why on the card** —
    /// `Servers` Q1 step 4 (not a Linux machine, or nothing it could read) or
    /// the connection lost at step 5, so step 8 never ran. `Servers` B5 had
    /// cleared the row's bit at the login, so **before the piece** the card
    /// read *Smoozy has not read this server yet.*, grey, with *Set up this
    /// server*, and not the sentence the connect ended in. Walked with the
    /// transport's own login and hand-over:
    ///
    /// - **the period ends after the login and before step 8**: the
    ///   sentence, the red dot, and *Set up this server* (not *Connect and set
    ///   up*, which would now only connect);
    /// - **a setup that reached step 8**, a plain *Connect*, a `what` outside
    ///   the three and no failure kept all leave the card as it was;
    /// - **the server's next login** takes it away, another server's does not,
    ///   and a login carrying nothing never ends piece M's *Connecting…*;
    /// - **a plain *Connect* mark** after it is not drawn as *Connecting…*.
    #[test]
    fn an_add_that_the_first_checks_stop_says_why_on_the_card() {
        use crate::servers::a_period::{the_login_carried, the_setup_is_handed_over};
        use crate::setting_up::{an_adds_setup_never_reached_step_8_on, an_adds_setup_waits_on};
        use crate::surface::servers::{
            words as said, IT_IS_NOT_A_LINUX_MACHINE_ANSWER, THE_CONNECTION_WAS_NOT_MADE_ANSWER,
        };
        let never = |server: &str| the_tab(WhatTheMachineSaid::NothingHasEverBeenReadFromIt, server);
        let the_card_with = |server: &str, mark: bool, failed: Option<&str>| {
            let mut tab = never(server);
            the_card_where_no_connection_stands(
                &mut tab,
                mark,
                an_adds_setup_waits_on(server),
                an_adds_setup_never_reached_step_8_on(server),
                failed,
                false,
            );
            tab.the_card
        };
        let ticked = || Some(vec![crate::surface::servers::GET_THE_SERVER_READY.to_string()]);
        let plain = never("any").the_card;
        assert_eq!(plain.state, Prose::new(words::NEVER_READ));

        // **The period ends after the login, at step 4 or 5.**
        for (server, what, sentence) in [
            ("server-m2-not-linux", IT_IS_NOT_A_LINUX_MACHINE_ANSWER, said::IT_IS_NOT_A_LINUX_MACHINE),
            ("server-m2-not-made", THE_CONNECTION_WAS_NOT_MADE_ANSWER, said::THE_CONNECTION_WAS_NOT_MADE),
        ] {
            let held: crate::setting_up::TheSetupHeldForTheHandOver = Default::default();
            the_login_carried(&held, server, ticked());
            assert!(!an_adds_setup_never_reached_step_8_on(server), "still waiting, not stopped");
            drop(held);
            assert!(an_adds_setup_never_reached_step_8_on(server));
            let card = the_card_with(server, false, Some(what));
            assert_eq!(card.state, Prose::new(sentence), "{card:?}");
            assert_eq!(card.dot, Verbatim::new("bad"), "{card:?}");
            assert_eq!(card.the_action.name, Prose::new(words::SET_UP_THIS_SERVER));
            assert_eq!(card.the_action.act, Verbatim::new(THE_READY_ACTION));
            assert!(!card.the_action.underway && !card.the_action.loud);
            assert!(card.instead.is_none() && card.what_was_done.is_empty());
            assert_eq!(card, the_card_where_the_first_checks_stopped_an_add(sentence));
            // **No failure kept, or a `what` outside the three**: as it was.
            assert_eq!(the_card_with(server, false, None), plain);
            assert_eq!(the_card_with(server, false, Some("the-record-would-not-take-it")), plain);
            // **A plain *Connect* mark is not an add's setup about to run.**
            assert_eq!(the_card_with(server, true, None), plain);
        }

        // **Another server's login leaves it**, and that server's own next
        // login, carrying nothing, takes it away.
        let server = "server-m2-not-linux";
        let other: crate::setting_up::TheSetupHeldForTheHandOver = Default::default();
        the_login_carried(&other, "server-m2-another", ticked());
        assert!(an_adds_setup_never_reached_step_8_on(server));
        let next: crate::setting_up::TheSetupHeldForTheHandOver = Default::default();
        the_login_carried(&next, server, None);
        assert!(!an_adds_setup_never_reached_step_8_on(server));
        assert_eq!(the_card_with(server, false, Some(IT_IS_NOT_A_LINUX_MACHINE_ANSWER)), plain);

        // **A setup that reached step 8** is not one the first checks stopped.
        let server = "server-m2-reached-step-8";
        let held: crate::setting_up::TheSetupHeldForTheHandOver = Default::default();
        the_login_carried(&held, server, ticked());
        the_setup_is_handed_over(&held, |_| {});
        drop(held);
        assert!(!an_adds_setup_never_reached_step_8_on(server));
        assert_eq!(the_card_with(server, false, Some(THE_CONNECTION_WAS_NOT_MADE_ANSWER)), plain);

        // **A plain *Connect* that failed** carried no setup.
        let server = "server-m2-plain";
        let held: crate::setting_up::TheSetupHeldForTheHandOver = Default::default();
        the_login_carried(&held, server, None);
        drop(held);
        assert_eq!(the_card_with(server, false, Some(IT_IS_NOT_A_LINUX_MACHINE_ANSWER)), plain);

        // **A login carrying nothing never ends piece M's *Connecting…***
        // while the add's own transport still stands.
        let server = "server-m2-still-waiting";
        let held: crate::setting_up::TheSetupHeldForTheHandOver = Default::default();
        the_login_carried(&held, server, ticked());
        let beside: crate::setting_up::TheSetupHeldForTheHandOver = Default::default();
        the_login_carried(&beside, server, None);
        assert!(an_adds_setup_waits_on(server));
        assert_eq!(the_card_with(server, false, None), the_card_while_an_add_connects());
        drop(held);

        // **The tab reads it, and the connection's hand-over says step 8 ran.**
        let working = include_str!("set_up.rs").split("#[cfg(test)]").next().expect("the working half");
        assert!(working.contains(
            "let stopped = crate::setting_up::an_adds_setup_never_reached_step_8_on(&server);"
        ));
        let period = include_str!("../servers/a_period.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the working half");
        assert!(period.contains("crate::setting_up::the_setup_reached_step_8(held);"));
    }

    /// **The card's page half, run under Node: the tab hears a connect that
    /// did not happen on the server showing, asks again with what it ended in,
    /// and draws the card the application answers** — red dot, the sentence,
    /// *Connect and set up* loud. **Pressing it hands the row's act back and
    /// takes the kept failure away.** An answer about another server, or one
    /// that is not a failed connect, asks nothing. **Before the piece** the tab
    /// listened for nothing of the server list's, and the card stayed on
    /// *Smoozy has not read this server yet.*
    #[test]
    fn the_page_asks_again_when_a_connect_on_its_server_was_not_made_and_draws_the_card() {
        let not_made = crate::surface::servers::THE_CONNECTION_WAS_NOT_MADE_ANSWER;
        let mut failed = the_tab(WhatTheMachineSaid::NothingHasEverBeenReadFromIt, "server-1");
        the_card_where_no_connection_stands(&mut failed, false, false, false, Some(not_made), true);
        let never = the_tab(WhatTheMachineSaid::NothingHasEverBeenReadFromIt, "server-1");
        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("await import(\"./set-up.js\");")
            .next()
            .expect("the harness's opening")
            .replace("THE_RUNNING", "null")
            .replace("THE_FINISHED", &serde_json::to_string(&never).expect("serialises"))
            .replace("THE_STOPPED", "null")
            + &r#"const failedTab = THE_FAILED;
const asked = [];
const theInvoke = window.__TAURI__.core.invoke;
window.__TAURI__.core.invoke = (command, args) => {
  if (command === "the_set_up_tab") asked.push(args.failed);
  return theInvoke(command, args);
};
await import("./set-up.js");
const host = anElement("div");
globalThis.tab.itIsDrawn(host);
globalThis.tab.itIsShowing("server-1");
await settle();
await theReadingLands(finished);
const [scroll] = host.children;
const [card] = scroll.children;
const told = (said) => (globalThis.told.get("servers:the-answer") || []).forEach((one) => one({ said }));
const out = { before: theTextOf(card) };
asked.splice(0);
// Another server's failed connect, and this server's answer that is not one.
globalThis.failedConnects = { "server-2": { what: "NOT_MADE", server: "server-2" } };
told({ what: "NOT_MADE", server: "server-2" });
told({ what: "the-record-would-not-take-it", server: "server-1" });
await settle();
out.askedForOthers = asked.length;
// This server's.
globalThis.failedConnects["server-1"] = { what: "NOT_MADE", server: "server-1" };
told({ what: "NOT_MADE", server: "server-1" });
await settle();
out.askedWith = asked.slice();
await theReadingLands(failedTab);
const dot = every(card).find((one) => one.dataset && one.dataset.dot);
const button = every(card).find((one) => one.tag === "button");
out.after = { text: theTextOf(card), dot: dot ? dot.dataset.dot : null, button: button ? button.text : null, loud: button ? button.className : null };
acts.splice(0);
button.press();
out.pressed = { acts: acts.slice(), takenAway: globalThis.takenAway || [] };
console.log(JSON.stringify(out));"#
                .replace("THE_FAILED", &serde_json::to_string(&failed).expect("serialises"))
                .replace("NOT_MADE", not_made);
        let out = the_set_up_page_run(&harness, "failed-connect");
        let Some(out) = out else { return };
        let sentence = Prose::new(crate::surface::servers::words::THE_CONNECTION_WAS_NOT_MADE);
        let never_read = Prose::new(words::NEVER_READ);
        assert!(out["before"].as_str().unwrap_or_default().contains(never_read.as_str()), "{out}");
        assert_eq!(out["askedForOthers"], 0, "{out}");
        assert_eq!(out["askedWith"], serde_json::json!([not_made]), "{out}");
        let after = out["after"]["text"].as_str().unwrap_or_default();
        assert!(after.contains(sentence.as_str()), "{out}");
        assert!(!after.contains(never_read.as_str()), "{out}");
        assert_eq!(out["after"]["dot"], "bad", "{out}");
        assert_eq!(out["after"]["button"], Prose::new(words::CONNECT_AND_SET_UP).as_str(), "{out}");
        assert_eq!(out["after"]["loud"], "btn", "{out}");
        assert_eq!(out["pressed"]["acts"], serde_json::json!([CONNECT_AND_SET_UP]), "{out}");
        assert_eq!(out["pressed"]["takenAway"], serde_json::json!(["server-1"]), "{out}");
    }

    /// **Runs one harness beside `web/set-up.js` and its stand-ins**, and
    /// answers what it printed — or nothing, saying so, where no Node is
    /// installed.
    fn the_set_up_page_run(harness: &str, named: &str) -> Option<serde_json::Value> {
        let folder = std::env::temp_dir().join(format!("smoozy-set-up-{named}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js");
        std::fs::copy(&page, folder.join("set-up.js")).expect("the page is copied");
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = if name == "harness.js" { harness.to_string() } else { text.to_string() };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let _ = std::fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Set up tab's page was not run");
            return None;
        };
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        Some(serde_json::from_slice(&ran.stdout).expect("the harness answers"))
    }

    /// The harness for the run on the card: it answers the tab and the run
    /// from the readings composed above, tells the page what the application
    /// would, and reads what was drawn where.
    // ── Several Blenders, one after another — the Creator's third notes,
    // note 2 (piece H, 2026-10-08)

    /// **A run's ending as the page is told it**: the line, and whether the
    /// card says it instead.
    fn the_line_of(event: &Event) -> (Option<String>, bool) {
        let delivery = delivered(event.clone());
        let line = match delivery.said.get("the_act_is_not_built_yet") {
            Some(Said::Prose(said)) => Some(said.as_str().to_string()),
            _ => None,
        };
        (line, it_says(&delivery, "the_card_says_it"))
    }

    fn it_moved_on(event: &Event) -> bool {
        it_says(&delivered(event.clone()), "the_run_is_under_way")
    }

    fn an_install_that_reached_done() -> crate::setting_up::WhereTheSetUpGot {
        crate::setting_up::WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Done,
            ending: crate::setting_up::TheRunsEnding::ItReachedDone { a_blender_was_published: true, the_driver_missed_the_optix_bar: false, the_swap_waits: false },
        }
    }

    // ── The nineteenth notes, note 2 — every setup act waits its turn ──────
    //
    // `Server Setup` Y9a items 6 to 10 (2026-10-27): a removal pressed while
    // an install ran was refused and its line wiped at once.

    fn a_removal_of(version: &str) -> crate::setting_up::ASetUpAct {
        crate::setting_up::ASetUpAct::ARemoval(crate::setting_up::TheRemovalAsked {
            executable: format!("/root/smoozy/blender/{version}/blender"),
            the_answered_version: version.to_string(),
            covered: Vec::new(),
            the_managers_package: None,
        })
    }

    fn an_install_of(version: &str) -> crate::setting_up::ASetUpAct {
        crate::setting_up::ASetUpAct::AnInstallFromTheList(version.to_string())
    }

    fn plain(said: &str) -> String {
        said.replace('\u{a0}', " ")
    }

    /// **A removal and a second install, pressed while an install runs, wait
    /// their turn, are listed on the running card in the order pressed, and
    /// run one after another once it finishes** — each its own run of its own
    /// kind, with nothing slipping between them. **Undone (the press refused,
    /// as before note 2), the press answers *A setup is already running* and
    /// nothing is listed.**
    #[test]
    fn note_2_a_removal_and_a_second_install_pressed_during_an_install_wait_and_then_run() {
        use crate::setting_up::{the_acts_are_taken_from_the_card, the_kind_being_driven_on, HowThePressWasTaken};
        let server = "note-2-removal-during-install";
        assert_eq!(
            the_acts_are_taken_from_the_card(server, vec![an_install_of("5.2.3")], &mut |_| {}),
            HowThePressWasTaken::ItBegan
        );
        let mut performed = Vec::new();
        let mut drawn = None;
        let mut events = Vec::new();
        let mut kept = Vec::new();
        the_acts_are_driven_in_turn(
            server,
            an_install_of("5.2.3"),
            &mut |act| {
                if performed.is_empty() {
                    // **The user presses *Remove anyway*, then installs
                    // another, while 5.2.3 installs.**
                    for pressed in [a_removal_of("5.2.2"), an_install_of("4.5.15")] {
                        assert_eq!(
                            the_acts_are_taken_from_the_card(server, vec![pressed], &mut |_| {}),
                            HowThePressWasTaken::ItWaitsItsTurn,
                            "the press was refused"
                        );
                    }
                    drawn = crate::surface::set_up::the_setup_under_way_on(server, || "Server".to_string());
                }
                performed.push((act.clone(), the_kind_being_driven_on(server)));
                an_install_that_reached_done()
            },
            &mut |_| None,
            &mut |event| events.push(event),
            &mut |line| kept.push(line.to_vec()),
        );
        let panel = drawn.expect("the running card");
        assert_eq!(panel.the_waiting_heading.as_ref().map(|it| it.as_str()), Some(words::THE_WAITING_HEADING));
        assert_eq!(
            panel.the_waiting.iter().map(|it| plain(it.as_str())).collect::<Vec<_>>(),
            ["Remove Blender 5.2.2", "Install Blender 4.5.15"],
            "the card does not list what waits, in the order pressed"
        );
        // **No *Set up again* while it runs** — gone, not greyed; *Cancel
        // setup* is the card's control (the twenty-fourth notes, note 1).
        assert!(panel.cancel.is_some() && panel.set_up_again.is_none(), "the running card offers Set up again");
        assert_eq!(
            performed,
            [
                (an_install_of("5.2.3"), Some(SetupKind::ABlenderInstall)),
                (a_removal_of("5.2.2"), Some(SetupKind::ABlenderRemoval)),
                (an_install_of("4.5.15"), Some(SetupKind::ABlenderInstall)),
            ],
            "the acts did not run in turn, each as its own kind"
        );
        assert_eq!(events.len(), 5, "an ending, the next run, an ending, the next run, an ending");
        assert!(it_moved_on(&events[1]) && it_moved_on(&events[3]));
        assert_eq!(
            plain(&the_line_of(&events[2]).0.expect("the removal's line")),
            words::BLENDER_REMOVED,
            "the removal's ending is not its own"
        );
        assert_eq!(
            kept,
            vec![vec![an_install_of("4.5.15")], vec![], vec![]],
            "the record did not follow the line: the one behind the removal, none behind the last, none at the end"
        );
        assert_eq!(the_kind_being_driven_on(server), None, "the run is let go");
        assert!(crate::surface::set_up::the_setup_under_way_on(server, || "Server".to_string())
            .is_none_or(|ended| ended.the_waiting.is_empty() && ended.cancel.is_none()));
        // **Back when the run ends**: the card offers *Set up again* again.
        assert_eq!(the_card_from(&a_reading()).the_action.name, Prose::new(words::SET_UP_AGAIN));
    }

    /// **A failed act stops the ones behind it, and the card names each as
    /// the act it is** — `Server Setup` Y9a item 7, today's rule for installs
    /// applied to every act. Nothing behind it is begun, and the record is
    /// emptied.
    #[test]
    fn note_2_a_failed_act_stops_the_ones_behind_it_and_names_them() {
        use crate::setting_up::{the_acts_are_taken_from_the_card, ASetUpAct, HowThePressWasTaken};
        let server = "note-2-a-failure-stops-the-rest";
        assert_eq!(
            the_acts_are_taken_from_the_card(server, vec![an_install_of("5.2.3")], &mut |_| {}),
            HowThePressWasTaken::ItBegan
        );
        for pressed in [a_removal_of("5.2.2"), ASetUpAct::TheReadyAction { a_render_was_running: false }] {
            assert_eq!(
                the_acts_are_taken_from_the_card(server, vec![pressed], &mut |_| {}),
                HowThePressWasTaken::ItWaitsItsTurn
            );
        }
        let mut performed = Vec::new();
        let mut events = Vec::new();
        let mut kept = Vec::new();
        the_acts_are_driven_in_turn(
            server,
            an_install_of("5.2.3"),
            &mut |act| {
                performed.push(act.clone());
                crate::setting_up::WhereTheSetUpGot::TheRunEnded {
                    at: Boundary::Fetched,
                    ending: crate::setting_up::TheRunsEnding::AStepFailed {
                        step: smoozy_server_setup::ready_action::Step::Libraries,
                        said: "the package manager refused".to_string(),
                    },
                }
            },
            &mut |_| None,
            &mut |event| events.push(event),
            &mut |line| kept.push(line.to_vec()),
        );
        assert_eq!(performed, [an_install_of("5.2.3")], "an act behind a failure was begun");
        assert_eq!(events.len(), 1);
        let (line, the_card_says_it) = the_line_of(&events[0]);
        let line = plain(&line.expect("the line"));
        assert!(!the_card_says_it);
        assert!(
            line.ends_with("Not installed: Blender 5.2.3. Not started: Remove Blender 5.2.2, Set up again."),
            "{line}"
        );
        assert_eq!(kept, vec![Vec::<ASetUpAct>::new()]);
        assert_eq!(crate::setting_up::the_kind_being_driven_on(server), None);
    }

    /// **A run that restarts the server stops the line the same way**, and the
    /// card says why — `Server Setup` Y9a item 7: *Set up again* restarted the
    /// server, so the install pressed behind it is not begun even though the
    /// setup reached *done*.
    #[test]
    fn note_2_a_restart_stops_the_ones_behind_it_and_says_so() {
        use crate::setting_up::{the_acts_are_taken_from_the_card, ASetUpAct, HowThePressWasTaken};
        let server = "note-2-a-restart-stops-the-rest";
        let set_up_again = ASetUpAct::TheReadyAction { a_render_was_running: false };
        assert_eq!(
            the_acts_are_taken_from_the_card(server, vec![set_up_again.clone()], &mut |_| {}),
            HowThePressWasTaken::ItBegan
        );
        assert_eq!(
            the_acts_are_taken_from_the_card(server, vec![an_install_of("4.5.15")], &mut |_| {}),
            HowThePressWasTaken::ItWaitsItsTurn
        );
        let restarted = crate::setting_up::TheSetupAsRead {
            the_run_holds_the_machine: true,
            record: TheSetupRecord {
                kind: SetupKind::TheReadyAction,
                the_composed_list: vec![Boundary::Started, Boundary::DriverPlaced, Boundary::RestartIssued, Boundary::CardAnswering, Boundary::Done],
                the_boundaries_reached: [Boundary::Started, Boundary::DriverPlaced, Boundary::RestartIssued, Boundary::CardAnswering, Boundary::Done]
                    .into_iter()
                    .map(|boundary| a_boundary(boundary, WhatTheStepDid::Performed))
                    .collect(),
                ending: None,
            },
            still_open: false,
            the_step_in_flight: None,
            the_admin_step_recorded_as_started: None,
            the_clock: "Hyperstack H100".to_string(),
        };
        let mut performed = Vec::new();
        let mut events = Vec::new();
        the_acts_are_driven_in_turn(
            server,
            set_up_again.clone(),
            &mut |act| {
                performed.push(act.clone());
                assert!(crate::setting_up::the_run_is_read_under_way(server, &restarted));
                crate::setting_up::WhereTheSetUpGot::TheRunEnded {
                    at: Boundary::Done,
                    ending: crate::setting_up::TheRunsEnding::ItReachedDone {
                        a_blender_was_published: false,
                        the_driver_missed_the_optix_bar: false,
                        the_swap_waits: false,
                    },
                }
            },
            &mut |_| None,
            &mut |event| events.push(event),
            &mut |_| {},
        );
        assert_eq!(performed, [set_up_again], "the install was begun on a server that restarted");
        assert_eq!(events.len(), 1);
        let (line, the_card_says_it) = the_line_of(&events[0]);
        let line = plain(&line.expect("the line"));
        assert!(!the_card_says_it, "the card would say only the setup's own ending");
        assert!(line.ends_with("Not started: Install Blender 4.5.15. The server restarted first."), "{line}");
        assert_eq!(crate::setting_up::the_kind_being_driven_on(server), None);
    }

    /// **Acts a closed Smoozy never began are named as acts on reopening**,
    /// where anything but installs from the list waited — `Server Setup` Y9a
    /// item 10 — and installs alone still read as they always have.
    #[test]
    fn note_2_the_acts_smoozy_closed_before_are_named_as_acts() {
        use crate::record::installs_not_begun::the_acts_waiting_are_kept;
        let folder = std::env::temp_dir().join(format!("smoozy-note-2-closed-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder");
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("the record");
        let c = record.connection();
        let server = "note-2-closed-first";
        let acts = [a_removal_of("5.2.2"), crate::setting_up::ASetUpAct::TheReadyAction { a_render_was_running: false }]
            .iter()
            .map(crate::setting_up::ASetUpAct::as_kept)
            .collect::<Vec<_>>();
        the_acts_waiting_are_kept(c, server, &acts).expect("kept");
        assert_eq!(
            the_installs_smoozy_closed_on(c, server).map(|it| plain(it.as_str())).as_deref(),
            Some("Not started: Remove Blender 5.2.2, Set up again. Smoozy closed first.")
        );
        crate::record::installs_not_begun::the_ones_waiting_are_kept(c, server, &["4.5.14".to_string()]).expect("kept");
        assert_eq!(
            the_installs_smoozy_closed_on(c, server).map(|it| plain(it.as_str())).as_deref(),
            Some("Not installed: Blender 4.5.14. Smoozy closed before it started.")
        );
        drop(record);
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// **The page draws the line the application hands it, and the drawing has
    /// it** — the *Waiting* list under the steps and the quiet *Install
    /// Telegram reporting* after *Cancel setup*, nothing of either on a run
    /// that ended, and `su-queue` in the drawing. **No *Set up again* on the
    /// running card**, on the page or in the drawing (the twenty-fourth notes,
    /// note 1, 2026-10-30).
    #[test]
    fn note_2_the_page_draws_the_waiting_list_and_the_quiet_acts() {
        let page = the_page();
        let the_card = page
            .split("function theRunCardIsDrawn(panel, server) {")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the run card");
        assert!(the_card.contains("waiting.className = \"suwait\";"));
        assert!(the_card.contains("heading.textContent = panel.the_waiting_heading;"));
        assert!(!the_card.contains("set_up_again_while_it_runs"), "the running card still draws Set up again");
        assert!(the_card.contains("aControl(panel.install_telegram_while_it_runs, () => theActIsAskedFor(INSTALL_TELEGRAM, server), true)"));
        let ended = page
            .split("function theStepsAsTheRunLeftThem(panel) {")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the ended run's steps");
        assert!(ended.contains("the_waiting: [],") && !ended.contains("set_up_again_while_it_runs"));
        let sheet = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/the-drawing.css"))
            .expect("the drawing's sheet");
        assert!(sheet.contains(".suwait { padding-left: 28px; }"));
        let drawing = the_mockup();
        let queued = drawing.split("id=\"buqueue\"").nth(1).expect("the drawing's su-queue card");
        assert!(queued.contains("<p class=\"suwh\">Queued</p>"));
        assert!(
            !drawing.contains("Cancel setup</button><button type=\"button\" class=\"btn quiet\">Set up again</button>"),
            "a running card in the drawing still offers Set up again"
        );
        assert!(drawing.contains(".suwait { padding-left: 28px; }"), "the page's rule is not the drawing's");
    }

    /// **The small owed fixes after the eighteenth notes, item 2 — a tooling
    /// step that failed at the unpack tool's own install says so on the step**:
    /// *Could not install the unpack tool.*, read off the reading's admin step
    /// still recorded as started; a tooling step that failed at the unpack,
    /// with no such mark, still says *Could not unpack Blender.* And the ready action's ending at the tool's install is
    /// said by the card, so no line repeats it above the steps. Until then the
    /// step read as the unpack's whichever act failed, and the line stayed for
    /// that reason.
    #[test]
    fn small_owed_the_unpack_tools_own_install_says_so_on_its_step() {
        use smoozy_server_protocol::framing::{Answered, Fields, Request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS};
        use smoozy_server_protocol::requests as asked;
        use smoozy_state::identities::{Identity, IdentityKind};
        let request = Request::speaking(
            THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS,
            Identity::issue(IdentityKind::Operation).expect("randomness"),
            asked::THE_SETUP_RUNS_STATE_REQUEST,
        );
        let item = |at, name| asked::the_field_of_item(asked::A_BOUNDARY_REACHED, at, name);
        let composed = |at| asked::the_field_of_item(asked::A_COMPOSED_BOUNDARY, at, asked::THE_BOUNDARY_FIELD);
        let a_run_failed_at_tooling = |more: Fields| {
            let answered = Answered::to(&request, THE_PROTOCOL_VERSION_THIS_BUILD_SPEAKS, "0.1.0").carrying(
                more.and(asked::ON_THIS_MACHINE_FIELD, asked::IT_IS_PRESENT)
                    .and(asked::THE_RUNS_KIND_FIELD, asked::THE_READY_ACTION)
                    .and(asked::HOW_MANY_COMPOSED_BOUNDARIES_FIELD, "4")
                    .and(&composed(0), asked::THE_BOUNDARY_STARTED)
                    .and(&composed(1), asked::THE_BOUNDARY_FETCHED)
                    .and(&composed(2), asked::THE_BOUNDARY_TOOLING)
                    .and(&composed(3), asked::THE_BOUNDARY_DONE)
                    .and(asked::HOW_MANY_BOUNDARIES_REACHED_FIELD, "2")
                    .and(&item(0, asked::THE_BOUNDARY_FIELD), asked::THE_BOUNDARY_STARTED)
                    .and(&item(0, asked::WHAT_THE_STEP_DID_FIELD), asked::IT_HAD_NOTHING_TO_DO)
                    .and(&item(0, asked::THE_BOUNDARY_AT_FIELD), "1790000000")
                    .and(&item(0, asked::THE_BOUNDARY_CLOCK_FIELD), "server-1")
                    .and(&item(1, asked::THE_BOUNDARY_FIELD), asked::THE_BOUNDARY_FETCHED)
                    .and(&item(1, asked::WHAT_THE_STEP_DID_FIELD), asked::IT_PERFORMED)
                    .and(&item(1, asked::THE_BOUNDARY_AT_FIELD), "1790000060")
                    .and(&item(1, asked::THE_BOUNDARY_CLOCK_FIELD), "server-1")
                    .and(asked::THE_RUNS_ENDING_FIELD, asked::A_STEP_FAILED)
                    .and(asked::THE_FAILED_STEP_FIELD, asked::THE_BOUNDARY_TOOLING)
                    .and(asked::THE_MACHINES_OWN_WORDS_FIELD, "E: Unable to locate package xz-utils"),
            );
            crate::setting_up::the_setup_as_the_machine_answered(&answered).expect("a reading")
        };
        let the_tooling_steps_sentence = |read: crate::setting_up::TheSetupAsRead| {
            let tab = the_tab(WhatTheMachineSaid::ASetupRunAlone(Box::new(read)), "server-1");
            let panel = tab.the_stopped_setup.expect("the stopped setup");
            let failed: Vec<_> = panel.steps.iter().filter(|step| step.the_failure.is_some()).collect();
            assert_eq!(failed.len(), 1, "one failed step");
            assert_eq!(failed[0].boundary, Boundary::Tooling);
            failed[0].the_failure.as_ref().map(|failed| failed.said.lead.as_str().to_string()).unwrap_or_default()
        };

        // **The tool's install**: the admin step the desk recorded as started
        // still stands, as the machine writes it at the failure the desk
        // recorded.
        let at_the_install = a_run_failed_at_tooling(Fields::none().and(asked::THE_STEP_FIELD, asked::THE_BOUNDARY_TOOLING));
        assert_eq!(at_the_install.the_admin_step_recorded_as_started, Some(Boundary::Tooling));
        assert_eq!(the_tooling_steps_sentence(at_the_install), Prose::new(words::FAILED_TOOLING).as_str());

        // **The unpack**: a worker step, commanded after the mark was cleared,
        // ends the run with none.
        let at_the_unpack = a_run_failed_at_tooling(Fields::none());
        assert_eq!(at_the_unpack.the_admin_step_recorded_as_started, None);
        assert_eq!(the_tooling_steps_sentence(at_the_unpack), Prose::new(words::FAILED_UNPACK).as_str());

        // **A worker step under way is not an admin step**, whatever it is.
        let a_worker_step = a_run_failed_at_tooling(Fields::none().and(asked::THE_STEP_FIELD, asked::THE_UNPACK_STEP));
        assert_eq!(a_worker_step.the_admin_step_recorded_as_started, None);

        // **The ready action's own ending at the tool's install is said by the
        // card**, so no line goes above the steps.
        assert!(the_card_says_the_ready_actions_ending(&crate::setting_up::WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Fetched,
            ending: crate::setting_up::TheRunsEnding::AStepFailed {
                step: smoozy_server_setup::ready_action::Step::Tooling,
                said: "E: Unable to locate package xz-utils".to_string(),
            },
        }));
    }

    /// **The small owed fixes after the eighteenth notes, item 3 — a failed
    /// Blender install shows its failure once, on the step, where the card
    /// keeps its steps**: the failed step says the install's own sentence, the
    /// build named and the versions not installed after it, and the panel tells
    /// the page to draw no line above the steps. **Where the card keeps no
    /// steps once the result is read**, the line is where it shows, so the
    /// ending is still told as a line. A ready action's stopped setup is not
    /// touched. Until then the stopped install's step said *The server could
    /// not fetch the Blender build.* and the line above repeated it, with its
    /// own *What the server said*.
    #[test]
    fn small_owed_a_failed_installs_sentence_is_on_its_step_and_once() {
        use smoozy_server_setup::recovery::TheFailedStepsWords;
        let server = "small-owed-install-fetch";
        const THE_PROGRAMS_SENTENCE: &str = "Download failed: HTTP 404.";
        assert!(crate::setting_up::a_run_begins(server, SetupKind::ABlenderInstall));
        crate::setting_up::the_installs_wait_on(server, vec!["4.5.14".to_string()]);
        let mut events = Vec::new();
        the_installs_are_driven_one_after_another(
            server,
            "5.2.2".to_string(),
            &mut |_| crate::setting_up::WhereTheSetUpGot::TheRunEnded {
                at: Boundary::Started,
                ending: crate::setting_up::TheRunsEnding::AStepFailed {
                    step: smoozy_server_setup::ready_action::Step::Fetch,
                    said: THE_PROGRAMS_SENTENCE.to_string(),
                },
            },
            &mut |_| None,
            &mut |event| events.push(event),
            &mut |_| {},
        );
        assert_eq!(events.len(), 1, "one ending, and nothing begun after it");
        let (line, the_card_says_it) = the_line_of(&events[0]);
        let line = line.expect("the line, for a card that keeps no steps");
        assert!(!the_card_says_it, "the ending is still told as a line");
        let plain = |text: &str| text.replace('\u{a0}', " ");
        let the_steps_own = plain(&line)
            .strip_prefix(words::INSTALL_STOPPED)
            .map(str::trim_start)
            .expect("the line opens with Install stopped.")
            .to_string();
        assert!(the_steps_own.contains("5.2.2") && the_steps_own.contains("4.5.14"), "{the_steps_own:?}");

        let a_stopped = |kind: SetupKind| {
            WhatTheMachineSaid::ASetupRunAlone(Box::new(crate::setting_up::TheSetupAsRead {
                the_run_holds_the_machine: true,
                record: TheSetupRecord {
                    kind,
                    the_composed_list: vec![Boundary::Started, Boundary::Fetched, Boundary::Tooling, Boundary::Blender, Boundary::Done],
                    the_boundaries_reached: vec![a_boundary(Boundary::Started, WhatTheStepDid::Performed)],
                    ending: Some(HowTheRunEnded::AStepFailed {
                        step: smoozy_server_setup::ready_action::Step::Fetch,
                        words: TheFailedStepsWords::SmoozysOwnVerification { what_was_read: THE_PROGRAMS_SENTENCE.to_string() },
                    }),
                },
                still_open: false,
                the_step_in_flight: Some(Boundary::Fetched),
                the_admin_step_recorded_as_started: None,
                the_clock: "RTX 3060".to_string(),
            }))
        };
        // **The install's stopped setup, as the card reads it where the
        // machine-level state stands for that run.**
        let panel = the_tab(a_stopped(SetupKind::ABlenderInstall), server).the_stopped_setup.expect("the stopped install");
        assert!(panel.the_last_act_is_said_on_its_steps, "the line above the steps would repeat the step");
        let failed: Vec<_> = panel.steps.iter().filter_map(|step| step.the_failure.as_ref().map(|it| (step.boundary, it))).collect();
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0].0, Boundary::Fetched);
        assert_eq!(plain(failed[0].1.said.lead.as_str()), plain(Prose::new(&the_steps_own).as_str()));
        assert!(the_steps_own.starts_with(&format!("{} 5.2.2.", words::COULD_NOT_FETCH_BLENDER)), "{the_steps_own:?}");
        assert!(failed[0].1.what_the_server_said.is_some(), "the step keeps What the server said");

        // **A ready action's stopped setup is its own**, whatever was kept.
        let ready = the_tab(a_stopped(SetupKind::TheReadyAction), server).the_stopped_setup.expect("the stopped setup");
        assert!(!ready.the_last_act_is_said_on_its_steps);
        let ready_failed = ready.steps.iter().find_map(|step| step.the_failure.as_ref()).expect("its failed step");
        assert_eq!(ready_failed.said.lead.as_str(), Prose::new(words::FAILED_FETCH).as_str());

        // **On another server nothing was kept.**
        let elsewhere = the_tab(a_stopped(SetupKind::ABlenderInstall), "small-owed-elsewhere").the_stopped_setup.expect("a stopped install");
        assert!(!elsewhere.the_last_act_is_said_on_its_steps);

        // **The page leaves the line out where the panel says so**, and only there.
        let page = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js")).expect("the page");
        assert!(page.contains("if (!panel.the_last_act_is_said_on_its_steps) theLastWordIsDrawnInto(one);"));
        assert_eq!(crate::setting_up::the_kind_being_driven_on(server), None, "the run is let go");
    }

    /// `Server Setup` B12: an install's end drops only its server's held list
    /// before telling an open Render Settings page to ask again.
    #[test]
    fn piece_f_an_installs_end_drops_the_held_version_list() {
        use crate::surface::render_settings::{the_run_holds, TheSearchStands};
        let server = "piece-f-installed";
        let other = "piece-f-other-server";
        let list = crate::surface::render_settings::tests::a_run_with_a_reading()
            .version_lists.into_values().next().expect("a version list");
        {
            let mut held = the_run_holds().lock().expect("the run");
            held.version_lists.insert(server.to_string(), list.clone());
            held.version_lists.insert(other.to_string(), list);
            held.version_searches.insert(server.to_string(), TheSearchStands::Searching);
        }
        assert!(crate::setting_up::a_run_begins(server, SetupKind::ABlenderInstall));
        let mut events = Vec::new();
        the_installs_are_driven_one_after_another(
            server,
            "5.4.0".to_string(),
            &mut |_| an_install_that_reached_done(),
            &mut |_| None,
            &mut |event| events.push(event),
            &mut |_| {},
        );
        let (list_stays, search_stays, other_stays) = {
            let mut held = the_run_holds().lock().expect("the run");
            let found = (
                held.version_lists.contains_key(server),
                held.version_searches.contains_key(server),
                held.version_lists.contains_key(other),
            );
            held.version_lists.remove(server);
            held.version_lists.remove(other);
            held.version_searches.remove(server);
            found
        };
        assert!(!list_stays, "the installed server still holds its old version list");
        assert!(!search_stays, "an older search still blocks the page's fresh ask");
        assert!(other_stays, "another server's list was dropped");
        assert_eq!(events.len(), 1);
        assert!(it_says(&delivered(events.remove(0)), "the_render_versions_are_read_again"));
    }

    /// Removal uses the same ending, and ordinary setup news leaves Render
    /// Settings alone (`Server Setup` B12).
    #[test]
    fn piece_f_a_removals_ending_also_drops_its_list() {
        use crate::surface::render_settings::the_run_holds;
        let server = "piece-f-removed";
        let list = crate::surface::render_settings::tests::a_run_with_a_reading()
            .version_lists.into_values().next().expect("a list");
        the_run_holds().lock().expect("the run").version_lists.insert(server.to_string(), list);
        // Re-check, 2026-10-26: the removal's own ending now carries its path too.
        let answer = a_removals_ending(server, "/opt/blender/5.2.2/blender", &an_install_that_reached_done());
        assert!(!the_run_holds().lock().expect("the run").version_lists.contains_key(server));
        assert!(it_says(&delivered(answer), "the_render_versions_are_read_again"));
        let ordinary = a_runs_ending(the_answer_about(server, &WhatItCameTo::only(words::SETUP_STOPPED)), false);
        assert!(!it_says(&delivered(ordinary), "the_render_versions_are_read_again"));
        let working = include_str!("set_up.rs").split("#[cfg(test)]").next().expect("the working half");
        let removal = working.split("fn a_removal_is_driven<").nth(1).expect("the removal route")
            .split("\n}\n").next().expect("the removal's end");
        assert!(removal.contains("crate::setting_up::ASetUpAct::ARemoval(removal)"), "removal is not an act of the line");
        let ending = working.split("fn the_acts_ending(").nth(1).expect("the line's endings")
            .split("\n}\n").next().expect("its end");
        assert!(ending.contains("ASetUpAct::ARemoval(asked) => a_removals_ending_saying("), "removal never tells Render Settings");
    }

    /// **Two picked: the first is installed, and the second goes when the
    /// first one's run ends** — each its own run, and `Server Setup` Y9's one
    /// run at a time held across the two (nothing else can begin between
    /// them). Before piece H the act carried one version and nothing waited.
    #[test]
    fn the_second_install_goes_when_the_first_run_ends() {
        let server = "piece-h-two-installed";
        assert!(crate::setting_up::a_run_begins(server, SetupKind::ABlenderInstall));
        crate::setting_up::the_installs_wait_on(server, vec!["4.5.14".to_string()]);
        let mut asked = Vec::new();
        let mut held_between = Vec::new();
        let mut events = Vec::new();
        the_installs_are_driven_one_after_another(
            server,
            "5.2.2".to_string(),
            &mut |version| {
                held_between.push((
                    crate::setting_up::the_kind_being_driven_on(server),
                    crate::setting_up::a_run_begins(server, SetupKind::TheReadyAction),
                ));
                asked.push(version);
                an_install_that_reached_done()
            },
            &mut |_| None,
            &mut |event| events.push(event),
            &mut |_| {},
        );
        assert_eq!(asked, ["5.2.2", "4.5.14"]);
        assert_eq!(
            held_between,
            [(Some(SetupKind::ABlenderInstall), false), (Some(SetupKind::ABlenderInstall), false)],
            "each install runs alone on the server"
        );
        assert_eq!(events.len(), 3, "an ending, the next run, an ending");
        assert_eq!(the_line_of(&events[0]).1, true, "a Blender installed is said by the table");
        assert!(it_moved_on(&events[1]));
        assert_eq!(the_line_of(&events[2]).1, true);
        assert_eq!(crate::setting_up::the_kind_being_driven_on(server), None, "the run is let go");
    }

    #[test]
    fn a_recorded_build_send_failure_keeps_the_uploads_words_as_smoozys_own() {
        use smoozy_server_setup::recovery::TheFailedStepsWords;
        let said = what_a_failed_build_send_says("The upload did not finish.");
        let record = TheSetupRecord {
            kind: SetupKind::ABlenderInstall,
            the_composed_list: vec![Boundary::Started, Boundary::Tooling, Boundary::Blender, Boundary::Done],
            the_boundaries_reached: vec![a_boundary(Boundary::Started, WhatTheStepDid::NothingToDo)],
            ending: Some(HowTheRunEnded::AStepFailed {
                step: smoozy_server_setup::ready_action::Step::Readings,
                words: TheFailedStepsWords::SmoozysOwnVerification { what_was_read: said.clone() },
            }),
        };
        let panel = the_run_panel(&record, Some(Boundary::Started), "server", false, true);
        let row = panel.steps.into_iter().find(|row| row.boundary == Boundary::Started).unwrap();
        assert_eq!(row.name.as_str().replace('\u{a0}', " "), words::UPLOAD_YOUR_BUILD);
        let failure = row.the_failure.expect("the failed send");
        assert_eq!(failure.said.lead.as_str().replace('\u{a0}', " "), said);
        assert!(failure.what_the_server_said.is_none());
        assert!(failure.the_output.is_none() && failure.the_complaint.is_none());
    }

    /// **A user's own build that was not installed is named as theirs, by its
    /// file name** — the build runner's ruling on its words, `Design/Server Setup §
    /// Pointing at a build on your own computer`: *Your build
    /// blender-4.2-linux.tar.xz was not installed.* After the run's own
    /// sentence where it failed or was stopped; alone where the user declined
    /// it; nothing on the card where it was installed. **Never *Blender* and
    /// the file name.** Step 10c's owed piece 5, 2026-10-16.
    #[test]
    fn a_users_build_not_installed_is_named_as_theirs_by_its_file_name() {
        let file = "blender-4.2-linux.tar.xz";
        let the_line_when = |server: &str, came_to: crate::setting_up::WhereTheSetUpGot| {
            assert!(crate::setting_up::a_run_begins(server, SetupKind::ABlenderInstall));
            let mut events = Vec::new();
            let mut asked = Vec::new();
            the_users_build_is_driven(
                server,
                file.to_string(),
                &mut |named| {
                    asked.push(named);
                    came_to.clone()
                },
                &mut |_| None,
                &mut |event| events.push(event),
                &mut |_| {},
            );
            assert_eq!(asked, [file], "the build was not driven once, by its file");
            assert_eq!(events.len(), 1, "one ending, and nothing begun after it");
            assert_eq!(crate::setting_up::the_kind_being_driven_on(server), None, "the run is let go");
            let (line, the_card_says_it) = the_line_of(&events[0]);
            (line.map(|line| line.replace('\u{a0}', " ")), the_card_says_it)
        };
        let typed = |text: &str| text.to_string();

        let declined = crate::setting_up::WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Done,
            ending: crate::setting_up::TheRunsEnding::ItReachedDone {
                a_blender_was_published: false,
                the_driver_missed_the_optix_bar: false,
                the_swap_waits: false,
            },
        };
        let (line, the_card_says_it) = the_line_when("r4-declined", declined);
        assert_eq!(line, Some(typed("Your build blender-4.2-linux.tar.xz was not installed.")));
        assert!(!the_card_says_it, "a build not installed is left to the table");

        let failed = crate::setting_up::WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Tooling,
            ending: crate::setting_up::TheRunsEnding::TheUnpackFailed { said: "tar: not an archive".to_string() },
        };
        let (line, _) = the_line_when("r4-failed", failed);
        let line = line.expect("a line");
        assert!(
            line.starts_with(&typed(words::INSTALL_STOPPED))
                && line.ends_with(&typed("Your build blender-4.2-linux.tar.xz was not installed.")),
            "the failed build's line reads {line:?}"
        );
        assert!(!line.contains(&typed(&format!("{} {file}", words::BLENDER))), "the file is named as a version: {line:?}");
        assert!(!line.contains(words::NOT_INSTALLED_THESE), "the list's line was used: {line:?}");

        let (line, _) = the_line_when("r4-stopped", crate::setting_up::WhereTheSetUpGot::TheRunEnded {
            at: Boundary::Started,
            ending: crate::setting_up::TheRunsEnding::TheUserStoppedIt,
        });
        assert_eq!(
            line,
            Some(typed("Install stopped. Your build blender-4.2-linux.tar.xz was not installed."))
        );

        let (_, the_card_says_it) = the_line_when("r4-installed", an_install_that_reached_done());
        assert!(the_card_says_it, "an installed build is said by the table");
    }

    /// **A failed install stops the ones still waiting, and the card's line
    /// names them** — `Design/Server Setup § Installing`: *Not installed:
    /// Blender 5.2.2, Blender 4.5.14.*
    #[test]
    fn a_failed_install_stops_the_rest_and_the_card_names_them() {
        let server = "piece-h-the-first-failed";
        assert!(crate::setting_up::a_run_begins(server, SetupKind::ABlenderInstall));
        crate::setting_up::the_installs_wait_on(server, vec!["5.2.2".to_string(), "4.5.14".to_string()]);
        let mut asked = Vec::new();
        let mut events = Vec::new();
        the_installs_are_driven_one_after_another(
            server,
            "5.2.1".to_string(),
            &mut |version| {
                asked.push(version);
                crate::setting_up::WhereTheSetUpGot::TheRunEnded {
                    at: Boundary::Started,
                    ending: crate::setting_up::TheRunsEnding::AStepFailed {
                        step: smoozy_server_setup::ready_action::Step::Fetch,
                        said: "the checksum file returned HTTP 404".to_string(),
                    },
                }
            },
            &mut |_| None,
            &mut |event| events.push(event),
            &mut |_| {},
        );
        assert_eq!(asked, ["5.2.1"], "nothing more is begun");
        assert_eq!(events.len(), 1);
        let (line, the_card_says_it) = the_line_of(&events[0]);
        assert_eq!(
            line.as_deref(),
            Some(
                Prose::new(
                    "Install stopped. Could not download Blender 5.2.1. Not installed: Blender \
                     5.2.2, Blender 4.5.14."
                )
                .as_str()
            )
        );
        assert!(!the_card_says_it);
        assert_eq!(crate::setting_up::the_kind_being_driven_on(server), None);
    }

    /// **The run ending any other way stops the rest too, and names them** —
    /// the user's own stop and the connection lost, the safe side: nothing is
    /// installed that the user is not watching begin. **A stop pressed while
    /// a Blender was being installed lets that one finish and begins no
    /// more.**
    #[test]
    fn a_stop_or_a_lost_connection_stops_the_rest_and_names_them() {
        let server = "piece-h-stopped";
        assert!(crate::setting_up::a_run_begins(server, SetupKind::ABlenderInstall));
        crate::setting_up::the_installs_wait_on(server, vec!["4.5.14".to_string()]);
        let mut events = Vec::new();
        the_installs_are_driven_one_after_another(
            server,
            "5.2.2".to_string(),
            &mut |_| {
                crate::setting_up::the_user_asked_to_stop(server);
                an_install_that_reached_done()
            },
            &mut |_| None,
            &mut |event| events.push(event),
            &mut |_| {},
        );
        assert_eq!(events.len(), 1);
        let (line, the_card_says_it) = the_line_of(&events[0]);
        assert_eq!(
            line.as_deref(),
            Some(Prose::new("Blender 5.2.2 installed. Not installed: Blender 4.5.14.").as_str())
        );
        assert!(!the_card_says_it, "the line names the one not installed");
        assert_eq!(crate::setting_up::the_kind_being_driven_on(server), None);

        // **Stopped before its Blender was installed**: the run's own is named
        // with the one waiting, *Install stopped.* naming none.
        assert!(crate::setting_up::a_run_begins(server, SetupKind::ABlenderInstall));
        crate::setting_up::the_installs_wait_on(server, vec!["4.5.14".to_string()]);
        let mut events = Vec::new();
        the_installs_are_driven_one_after_another(
            server,
            "5.2.2".to_string(),
            &mut |_| crate::setting_up::WhereTheSetUpGot::TheRunEnded {
                at: Boundary::Started,
                ending: crate::setting_up::TheRunsEnding::TheUserStoppedIt,
            },
            &mut |_| None,
            &mut |event| events.push(event),
            &mut |_| {},
        );
        assert_eq!(
            the_line_of(&events[0]).0.as_deref(),
            Some(Prose::new("Install stopped. Not installed: Blender 5.2.2, Blender 4.5.14.").as_str())
        );

        let server = "piece-h-the-connection-went";
        assert!(crate::setting_up::a_run_begins(server, SetupKind::ABlenderInstall));
        crate::setting_up::the_installs_wait_on(server, vec!["4.5.14".to_string(), "4.2.9".to_string()]);
        let mut events = Vec::new();
        let mut asked = 0;
        the_installs_are_driven_one_after_another(
            server,
            "5.2.2".to_string(),
            &mut |_| {
                asked += 1;
                if asked == 1 {
                    an_install_that_reached_done()
                } else {
                    crate::setting_up::WhereTheSetUpGot::TheConnectionWent
                }
            },
            &mut |_| None,
            &mut |event| events.push(event),
            &mut |_| {},
        );
        assert_eq!(asked, 2);
        assert_eq!(events.len(), 3);
        assert_eq!(
            the_line_of(&events[2]).0.as_deref(),
            Some(
                Prose::new(
                    "Install stopped. Connection lost. Not installed: Blender 4.5.14, Blender \
                     4.2.9."
                )
                    .as_str()
            )
        );
        assert_eq!(crate::setting_up::the_kind_being_driven_on(server), None);
    }

    // ── `Server Setup` Y9a — installs Smoozy closed before it began, step
    // 10c's owed piece 4, 2026-10-16.

    /// **The chain keeps the ones still waiting each time the next begins, and
    /// keeps nothing once it ends** — however it ends, since every ending
    /// names its own on the card. So what the local record holds is only ever
    /// a list that a Smoozy closed before it could begin. **Undone (the keep
    /// left out of either arm), this fails.**
    #[test]
    fn the_chain_keeps_the_ones_still_waiting_and_nothing_once_it_ends() {
        let server = "piece-4-the-chain-keeps";
        assert!(crate::setting_up::a_run_begins(server, SetupKind::ABlenderInstall));
        crate::setting_up::the_installs_wait_on(server, vec!["4.5.14".to_string(), "4.2.9".to_string()]);
        let mut kept: Vec<Vec<String>> = Vec::new();
        let mut asked = Vec::new();
        the_installs_are_driven_one_after_another(
            server,
            "5.2.2".to_string(),
            &mut |version| {
                asked.push(version);
                an_install_that_reached_done()
            },
            &mut |_| None,
            &mut |_| {},
            &mut |waiting| kept.push(waiting.to_vec()),
        );
        assert_eq!(asked, ["5.2.2", "4.5.14", "4.2.9"]);
        assert_eq!(
            kept,
            vec![vec!["4.2.9".to_string()], Vec::new(), Vec::new()],
            "the ones after the next as each begins, and nothing at the end"
        );

        let server = "piece-4-the-chain-fails";
        assert!(crate::setting_up::a_run_begins(server, SetupKind::ABlenderInstall));
        crate::setting_up::the_installs_wait_on(server, vec!["4.5.14".to_string()]);
        let mut kept: Vec<Vec<String>> = Vec::new();
        the_installs_are_driven_one_after_another(
            server,
            "5.2.2".to_string(),
            &mut |_| crate::setting_up::WhereTheSetUpGot::TheConnectionWent,
            &mut |_| None,
            &mut |_| {},
            &mut |waiting| kept.push(waiting.to_vec()),
        );
        assert_eq!(kept, vec![Vec::<String>::new()], "a failed chain names its own and keeps nothing");
    }

    /// **The card's line names the installs kept for that server, in the
    /// existing line's shape, with the words saying Smoozy closed first** —
    /// `Design/Server Setup § Installing`. One for several and one for a
    /// single version. **Nothing while this copy drives a run there**, since
    /// the list is then that run's own. **Undone (no line, or the line drawn
    /// while a run is driven), this fails.**
    #[test]
    fn the_line_names_the_installs_smoozy_closed_before_and_none_while_a_run_is_driven() {
        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-set-up-piece-4-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("a scratch folder");
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).expect("a record");
        let c = record.connection();
        let server = "piece-4-the-line";
        assert_eq!(the_installs_smoozy_closed_on(c, server), None, "nothing kept, no line");
        crate::record::installs_not_begun::the_ones_waiting_are_kept(
            c,
            server,
            &["5.2.2".to_string(), "4.5.14".to_string()],
        )
        .expect("kept");
        assert_eq!(
            the_installs_smoozy_closed_on(c, server),
            Some(Prose::new("Not installed: Blender 5.2.2, Blender 4.5.14. Smoozy closed before they started."))
        );
        assert!(crate::setting_up::a_run_begins(server, SetupKind::ABlenderInstall));
        assert_eq!(the_installs_smoozy_closed_on(c, server), None, "the list is the run's own while it goes");
        crate::setting_up::a_run_ends(server);
        crate::record::installs_not_begun::the_ones_waiting_are_kept(c, server, &["4.5.14".to_string()])
            .expect("kept");
        assert_eq!(
            the_installs_smoozy_closed_on(c, server),
            Some(Prose::new("Not installed: Blender 4.5.14. Smoozy closed before it started."))
        );
        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **The page draws the line under the card's state line, as the line
    /// after an install that stopped is drawn**, and draws nothing where the
    /// tab carries none. It runs the page code under Node, as the tests beside
    /// it do; skipped, and says so, where no Node is installed. **Undone (the
    /// page not drawing it), this fails.**
    #[test]
    fn the_card_draws_the_installs_smoozy_closed_before_under_its_state_line() {
        let mut tab = the_tab(
            WhatTheMachineSaid::ASetupRunAndTheMachine(
                Box::new(the_finished_run_as_read()),
                Box::new(the_rest_with(None, None, Some(false))),
            ),
            "server-1",
        );
        let without = tab.clone();
        tab.the_installs_never_begun = the_installs_never_begun(&["5.2.2".to_string(), "4.5.14".to_string()]);
        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("const running = THE_RUNNING;")
            .next()
            .expect("the harness's opening")
            .to_string()
            + &r#"const withIt = WITH_IT;
const withoutIt = WITHOUT_IT;
const readings = [];
globalThis.window = {
  __TAURI__: {
    core: {
      invoke(command) {
        if (command === "the_set_up_tab") return new Promise((done) => readings.push(done));
        return Promise.resolve(null);
      },
    },
  },
};
const settle = async () => { for (let at = 0; at < 4; at += 1) await new Promise((done) => setImmediate(done)); };
const theReadingLands = async (tab) => { readings.splice(0).forEach((done) => done(tab)); await settle(); };
await import("./set-up.js");
const host = anElement("div");
globalThis.tab.itIsDrawn(host);
globalThis.tab.itIsShowing("server-1");
await settle();
const theCard = () => every(host).find((one) => one.classList && one.classList.contains("sucard"));
const theLines = () => theCard().children.map((one) => [one.className, theTextOf(one)]);
await theReadingLands(withIt);
const out = { withIt: theLines() };
globalThis.tab.itIsShowing("server-1");
await settle();
await theReadingLands(withoutIt);
out.withoutIt = theLines();
console.log(JSON.stringify(out));"#
                .replace("WITH_IT", &serde_json::to_string(&tab).expect("serialises"))
                .replace("WITHOUT_IT", &serde_json::to_string(&without).expect("serialises"));
        let folder = std::env::temp_dir().join(format!("smoozy-set-up-closed-first-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js");
        std::fs::copy(&page, folder.join("set-up.js")).expect("the page is copied");
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => harness.clone(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Set up tab's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let line = Prose::new("Not installed: Blender 5.2.2, Blender 4.5.14. Smoozy closed before they started.");
        assert_eq!(
            out["withIt"][1],
            serde_json::json!(["sunote", line.as_str()]),
            "the line is not the one under the state line: {out}"
        );
        assert!(
            !out["withoutIt"]
                .as_array()
                .expect("the card's lines")
                .iter()
                .any(|one| one[1].as_str().is_some_and(|text| text.contains(words::NOT_INSTALLED_THESE))),
            "a line was drawn with none kept: {out}"
        );
    }

    /// **Ticking two sends the first install with the second waiting behind
    /// it, in the order the list shows them, and the button counts what is
    /// ticked** — *Install 5.2.2*, *Install 3 versions*. It runs the page code
    /// under Node, as the tests beside it do; skipped, and says so, where no
    /// Node is installed. Before piece H the marks were radios and the act
    /// carried one version.
    #[test]
    fn ticking_several_sends_the_first_with_the_rest_waiting_and_the_button_counts() {
        let channel = |track: &str, risk: Risk, major, minor, patch, file: Option<&str>| PublishedChannel {
            track: track.to_string(),
            risk,
            version: Version { major, minor, patch },
            released_at: String::new(),
            size_bytes: 0,
            the_build_services_file: file.map(str::to_string),
        };
        let alpha = "blender-5.3.0-alpha+main.64d181010e0d-linux.x86_64-release.tar.xz";
        let list = the_list(
            Ok(smoozy_server_setup::listing::TheListingAsRead {
                channels: vec![
                    channel("latest", Risk::Stable, 5, 2, 2, None),
                    channel("4.5lts", Risk::Stable, 4, 5, 14, None),
                    channel("", Risk::Alpha, 5, 3, 0, Some(alpha)),
                ],
                entries_that_could_not_be_read: Vec::new(),
            }),
            Vec::new(),
        );
        let tab = the_tab(
            WhatTheMachineSaid::ASetupRunAndTheMachine(
                Box::new(the_finished_run_as_read()),
                Box::new(the_rest_with(None, None, Some(false))),
            ),
            "server-1",
        );
        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("const running = THE_RUNNING;")
            .next()
            .expect("the harness's opening")
            .to_string()
            + &r#"const finished = THE_FINISHED;
const theList = THE_LIST;
const readings = [];
const acts = [];
globalThis.window = {
  __TAURI__: {
    core: {
      invoke(command, args) {
        if (command === "the_set_up_tab") return new Promise((done) => readings.push(done));
        if (command === "the_setup_under_way") return Promise.resolve(null);
        if (command === "the_install_list") return Promise.resolve(theList);
        if (command === "a_set_up_act_that_needs_a_connection") acts.push(args);
        return Promise.resolve(null);
      },
    },
  },
};
const settle = async () => { for (let at = 0; at < 4; at += 1) await new Promise((done) => setImmediate(done)); };
const theReadingLands = async (tab) => { readings.splice(0).forEach((done) => done(tab)); await settle(); };
await import("./set-up.js");
const host = anElement("div");
globalThis.tab.itIsDrawn(host);
globalThis.tab.itIsShowing("server-1");
await settle();
await theReadingLands(finished);
const [scroll] = host.children;
const [, table] = scroll.children;
const buttonsIn = (section) => every(section).filter((one) => one.tag === "button");
buttonsIn(table).at(-1).press();
await settle();
const box = every(host).find((one) => one.classList && one.classList.contains("mbox"));
const marks = every(box).filter((one) => one.tag === "label");
const install = () => buttonsIn(box).at(-1);
const out = { marks: marks.length, counts: [theTextOf(install())] };
install().press();
out.actsWithNone = acts.length;
marks[1].press(); out.counts.push(theTextOf(install()));
marks[0].press(); out.counts.push(theTextOf(install()));
marks[2].press(); out.counts.push(theTextOf(install()));
out.ticks = marks.map((one) => theTextOf(one.children[0]));
out.boxes = marks.map((one) => one.children[0].className);
marks[2].press(); out.counts.push(theTextOf(install()));
install().press();
out.acts = acts;
console.log(JSON.stringify(out));"#
                .replace("THE_FINISHED", &serde_json::to_string(&tab).expect("serialises"))
                .replace("THE_LIST", &serde_json::to_string(&list).expect("serialises"));
        let folder = std::env::temp_dir().join(format!("smoozy-set-up-several-blenders-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js");
        std::fs::copy(&page, folder.join("set-up.js")).expect("the page is copied");
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => harness.clone(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Set up tab's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let install = Prose::new(words::INSTALL);
        let versions = Prose::new(words::VERSIONS);
        assert_eq!(out["marks"], 3, "{out}");
        assert_eq!(out["actsWithNone"], 0, "nothing ticked asks for nothing: {out}");
        assert_eq!(
            out["counts"],
            serde_json::json!([
                install.as_str(),
                format!("{} 4.5.14", install.as_str()),
                format!("{} 2 {}", install.as_str(), versions.as_str()),
                format!("{} 3 {}", install.as_str(), versions.as_str()),
                format!("{} 2 {}", install.as_str(), versions.as_str()),
            ]),
            "{out}"
        );
        assert_eq!(out["ticks"], serde_json::json!(["✓", "✓", "✓"]), "{out}");
        assert_eq!(out["boxes"], serde_json::json!(["cb on", "cb on", "cb on"]), "tick boxes, not radios: {out}");
        assert_eq!(
            out["acts"],
            serde_json::json!([{
                "act": INSTALL_A_BLENDER,
                "server": "server-1",
                "version": "5.2.2",
                "waiting": ["4.5.14"],
                "path": null,
            }]),
            "the first in the list's order goes now, the rest wait: {out}"
        );
    }

    // ── `Server Setup` Y7a and `Desktop app and delivery` M13b's second
    // narrowing: what the server
    // printed, beside the steps — the thirteenth notes, piece H, 2026-10-18.

    /// A run going, at the driver step, read as the driving reads it.
    fn a_run_at_the_driver_step() -> TheRunPanel {
        let read = crate::setting_up::TheSetupAsRead {
            the_run_holds_the_machine: true,
            record: TheSetupRecord {
                kind: SetupKind::TheReadyAction,
                the_composed_list: vec![Boundary::Started, Boundary::DriverPlaced, Boundary::RestartIssued, Boundary::Done],
                the_boundaries_reached: vec![a_boundary(Boundary::Started, WhatTheStepDid::Performed)],
                ending: None,
            },
            still_open: true,
            the_step_in_flight: Some(Boundary::DriverPlaced),
            the_admin_step_recorded_as_started: None,
            the_clock: String::new(),
        };
        the_panel_of_a_run_under_way(&read, "Vast 4090", false)
    }

    fn three_lines() -> Vec<String> {
        vec![
            "Unpacking nvidia-driver-595 (595.91.07-0ubuntu1) over (590.48.01-0ubuntu1) ...".to_string(),
            "Setting up nvidia-dkms-595 (595.91.07-0ubuntu1) ...".to_string(),
            "Building initial module nvidia/595.91.07 for 6.8.0-45-generic".to_string(),
        ]
    }

    /// **The window and its control, as the card is handed them** — `Design/
    /// Server Setup § A long step`: the newest lines while the run goes and
    /// *Server output* under them; **nothing offered before the first line
    /// but the control, alone**; no window once the run ends, the control
    /// staying; and **no window beside a sentence about a failure**. Before
    /// piece H the card had neither.
    #[test]
    fn y7a_the_window_is_offered_only_while_the_run_goes_and_never_beside_a_failure() {
        // Before the first line: the control alone, no window.
        let mut alone = a_run_at_the_driver_step();
        the_servers_words_beside_the_run(&mut alone, true, Vec::new());
        assert_eq!(alone.server_output, Some(Prose::new(words::SERVER_OUTPUT)));
        assert!(alone.the_servers_newest_lines.is_empty(), "a window before anything printed");
        let mut first = the_run_before_its_first_reading(words::SETTING_UP, false);
        the_servers_words_beside_the_run(&mut first, true, Vec::new());
        assert_eq!(first.server_output, Some(Prose::new(words::SERVER_OUTPUT)));
        assert!(first.the_servers_newest_lines.is_empty());

        // The lines, while it goes.
        let mut going = a_run_at_the_driver_step();
        the_servers_words_beside_the_run(&mut going, true, three_lines());
        assert_eq!(
            going.the_servers_newest_lines,
            three_lines().into_iter().map(Verbatim::new).collect::<Vec<_>>()
        );

        // The run ended: no window, the control stays.
        let mut ended = the_run_panel(
            &TheSetupRecord {
                kind: SetupKind::TheReadyAction,
                the_composed_list: vec![Boundary::Started, Boundary::DriverPlaced, Boundary::Done],
                the_boundaries_reached: vec![a_boundary(Boundary::Started, WhatTheStepDid::Performed)],
                ending: Some(HowTheRunEnded::EndedAtBoundaryNotCompleted { at: Some(Boundary::Started) }),
            },
            Some(Boundary::DriverPlaced),
            "Vast 4090",
            false,
            false,
        );
        the_servers_words_beside_the_run(&mut ended, true, three_lines());
        assert!(ended.the_servers_newest_lines.is_empty(), "a window on a run that ended");
        assert_eq!(ended.server_output, Some(Prose::new(words::SERVER_OUTPUT)));

        // A failed step on the card: never beside its sentence.
        let mut failed = a_run_at_the_driver_step();
        failed.steps[0].the_failure = Some(TheFailedStep {
            said: plainly(words::FAILED_READINGS),
            what_the_server_said: Some(Prose::new(words::WHAT_THE_SERVER_SAID)),
            the_output: None,
            the_complaint: Some(Verbatim::new("E: a complaint")),
            the_output_tab: Prose::new("Output"),
            the_complaint_tab: Prose::new("Errors"),
            close: Prose::new(words::CLOSE),
        });
        the_servers_words_beside_the_run(&mut failed, true, three_lines());
        assert!(failed.the_servers_newest_lines.is_empty(), "lines beside a failure sentence");

        // No words kept: neither.
        let mut none = a_run_at_the_driver_step();
        the_servers_words_beside_the_run(&mut none, false, three_lines());
        assert!(none.server_output.is_none() && none.the_servers_newest_lines.is_empty());

        // The tab, after the run: the control on the card and on a stopped
        // setup's card, never a window.
        let mut tab = the_tab(WhatTheMachineSaid::ConnectedAndNoRunWasFound, "server-1");
        tab.the_stopped_setup = Some(going.clone());
        the_servers_words_beside_the_tab(&mut tab, true);
        assert_eq!(tab.server_output, Some(Prose::new(words::SERVER_OUTPUT)));
        let stopped = tab.the_stopped_setup.as_ref().expect("the stopped card");
        assert!(stopped.the_servers_newest_lines.is_empty());
        assert_eq!(stopped.server_output, Some(Prose::new(words::SERVER_OUTPUT)));
        the_servers_words_beside_the_tab(&mut tab, false);
        assert!(tab.server_output.is_none());
    }

    /// **The lines reach the card's event as they print, from either stream**:
    /// a piece kept tells the tab, and the panel the tab then asks for carries
    /// the newest lines of both streams in the order they arrived, each marked
    /// by its stream, the line still being printed as it stands. **The event
    /// itself carries no words** (P6), and answers no press. **Until
    /// 2026-10-25 the complaints stream never reached the window and a line
    /// not yet ended was never shown** (M13b's second narrowing as changed on
    /// 2026-10-24; the eighteenth notes, piece 2c).
    #[test]
    fn y7a_the_lines_reach_the_cards_event_as_they_print_from_either_stream() {
        use crate::servers::sessions::APiecePrinted;
        use smoozy_server_setup::ready_action::Step;
        let server = "piece-h-the-cards-event";
        assert!(crate::setting_up::a_run_begins(server, SetupKind::TheReadyAction));
        let window = || {
            let mut panel = a_run_at_the_driver_step();
            the_servers_window_beside_the_run(
                &mut panel,
                crate::setting_up::the_words_are_kept_for(server),
                crate::setting_up::the_window_on(server),
            );
            panel
        };
        assert!(window().the_servers_newest_lines.is_empty(), "nothing before the first line");
        assert!(window().server_output.is_some(), "the control, alone");

        crate::setting_up::a_piece_is_kept(server, Step::Driver, &APiecePrinted::Complaint(b"E: dpkg was interrupted\n".to_vec()));
        assert_eq!(window().the_servers_newest_lines, vec![Verbatim::new("E: dpkg was interrupted")]);
        assert_eq!(window().which_lines_are_from_the_error_stream, vec![true]);

        crate::setting_up::a_piece_is_kept(server, Step::Driver, &APiecePrinted::Output(b"Setting up nvidia-dkms-595 ...\nBuilding".to_vec()));
        assert_eq!(
            window().the_servers_newest_lines,
            vec![
                Verbatim::new("E: dpkg was interrupted"),
                Verbatim::new("Setting up nvidia-dkms-595 ..."),
                Verbatim::new("Building"),
            ],
            "the line still being printed is not shown as it stands"
        );
        assert_eq!(window().which_lines_are_from_the_error_stream, vec![true, false, false]);
        crate::setting_up::a_piece_is_kept(server, Step::Driver, &APiecePrinted::Output(b" initial module\n".to_vec()));
        assert_eq!(
            window().the_servers_newest_lines,
            vec![
                Verbatim::new("E: dpkg was interrupted"),
                Verbatim::new("Setting up nvidia-dkms-595 ..."),
                Verbatim::new("Building initial module"),
            ],
            "each line as it is completed"
        );

        let told = delivered(the_servers_words_moved_on(server));
        assert!(it_says(&told, "the_servers_words_moved"));
        assert_eq!(told.said.get("server"), Some(&Said::Value(Verbatim::new(server))));
        assert_eq!(told.said.len(), 2, "the event carries words: {:?}", told.said);
        crate::setting_up::a_run_ends(server);
    }

    /// ***Server output* opens the whole of what the run printed, the two
    /// streams apart** — each under its own name, whole and unedited, with the
    /// copy icon's word; and a stream that printed nothing says so, never an
    /// empty box (M10, M10a).
    #[test]
    fn y7a_server_output_holds_both_streams_apart_whole() {
        let output = "Reading package lists...\r\nSetting up zstd ...\n";
        let opened = the_servers_output_from("Vast 4090", output, "debconf: delaying\n", false);
        // **From 2026-10-25 the record is shown as a terminal shows it** —
        // the carriage return before the line feed changes nothing on screen
        // — **and the words as printed ride beside it for Copy** (M13b, M9;
        // the eighteenth notes, piece 2c).
        assert_eq!(opened.title, Prose::new(words::SERVER_OUTPUT));
        assert_eq!(opened.subtitle, Verbatim::new("Vast 4090"));
        assert_eq!(opened.copy, Prose::new(words::COPY));
        assert_eq!(opened.tabs.len(), 2);
        assert_eq!(opened.tabs[0].name, Prose::new(Stream::TheOrdinaryOutput.the_name_of_the_stream()));
        assert_eq!(opened.tabs[0].record, Some(Verbatim::new("Reading package lists...\nSetting up zstd ...\n")));
        assert_eq!(opened.tabs[0].as_printed, Some(Verbatim::new(output)));
        assert_eq!(opened.tabs[1].name, Prose::new(Stream::TheComplaints.the_name_of_the_stream()));
        assert_eq!(opened.tabs[1].record, Some(Verbatim::new("debconf: delaying\n")));
        assert_eq!(opened.tabs[1].as_printed, None, "the words as printed were carried twice");

        let going = the_servers_output_from("Vast 4090", "", "", true);
        assert!(going.tabs.iter().all(|tab| tab.record.is_none() && tab.said.is_some() && tab.said_next.is_some()));
        assert_eq!(going.tabs[0].says, Verbatim::new("not-recorded-yet"));
        let ended = the_servers_output_from("Vast 4090", "", "", false);
        assert_eq!(ended.tabs[0].said, Some(Prose::new(words::THE_SERVER_PRINTED_NO_OUTPUT)));
        assert_eq!(ended.tabs[1].said, Some(Prose::new(words::THE_SERVER_WROTE_NOTHING_HERE)));
    }

    /// **The page draws it**: the control alone before anything printed; the
    /// lines to the right of the steps, each one isolated run, with the control
    /// under them; lines that move drawn in place without the card being built
    /// again; the control opening the panel; and the control on the card once
    /// the run has ended. It runs the page code under Node, as the tests
    /// beside it do; skipped, and says so, where no Node is installed.
    #[test]
    fn y7a_the_page_draws_the_window_beside_the_steps_and_opens_server_output() {
        let mut alone = a_run_at_the_driver_step();
        the_servers_words_beside_the_run(&mut alone, true, Vec::new());
        let mut running = a_run_at_the_driver_step();
        the_servers_words_beside_the_run(&mut running, true, three_lines()[..2].to_vec());
        let mut moved = a_run_at_the_driver_step();
        the_servers_words_beside_the_run(&mut moved, true, three_lines());
        let mut finished = the_tab(WhatTheMachineSaid::ConnectedAndNoRunWasFound, "server-1");
        the_servers_words_beside_the_tab(&mut finished, true);
        let output = the_servers_output_from("Vast 4090", "one\n", "", false);
        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("const running = THE_RUNNING;")
            .next()
            .expect("the harness's opening")
            .to_string()
            + &r#"const alone = ALONE;
const running = RUNNING;
const moved = MOVED;
const finished = FINISHED;
const output = OUTPUT;
let theRun = null;
const readings = [];
const asked = [];
globalThis.window = {
  __TAURI__: {
    core: {
      invoke(command, args) {
        asked.push(command);
        if (command === "the_set_up_tab") return new Promise((done) => readings.push(done));
        if (command === "the_setup_under_way") return Promise.resolve(theRun);
        if (command === "the_servers_output") return Promise.resolve(output);
        return Promise.resolve(null);
      },
    },
  },
};
const settle = async () => { for (let at = 0; at < 6; at += 1) await new Promise((done) => setImmediate(done)); };
const tell = (said) => (globalThis.told.get("set-up:the-answer") || []).forEach((one) => one({ said }));
const theReadingLands = async (tab) => { readings.splice(0).forEach((done) => done(tab)); await settle(); };
await import("./set-up.js");
const host = anElement("div");
globalThis.tab.itIsDrawn(host);
globalThis.tab.itIsShowing("server-1");
await settle();
await theReadingLands(finished);
const [scroll, boxes] = host.children;
const [card] = scroll.children;
const find = (name) => every(card).find((one) => one.classList && one.classList.contains(name));
const theControl = () => every(find("log")).find((one) => one.tag === "button");
const out = {};
// **From 2026-10-25 the finished card's control sits on its own line under
// the list**, the drawing's `.suout`, and no `.log` is drawn (piece 2c).
const under = find("suout");
out.finished = {
  log: find("log") ? find("log").className : null,
  suout: under ? under.children.map((one) => [one.tag, one.className, theTextOf(one)]) : null,
};

theRun = alone;
tell({ server: "server-1", the_run_is_under_way: "yes" });
await settle();
out.alone = { log: find("log").className, lines: find("qlines") ? 1 : 0, control: theTextOf(theControl()) };

theRun = running;
tell({ server: "server-1", the_servers_words_moved: "yes" });
await settle();
const before = theControl();
out.running = {
  work: find("suwork").children.map((one) => one.className),
  lines: find("qlines").children.map(theTextOf),
  isolated: find("qlines").children.map((one) => one.children[0] && one.children[0].tag),
  control: theTextOf(before),
};

theRun = moved;
tell({ server: "server-1", the_servers_words_moved: "yes" });
await settle();
out.moved = { lines: find("qlines").children.map(theTextOf), theSameCard: theControl() === before };

theControl().press();
await settle();
out.opened = { asked: asked.filter((one) => one === "the_servers_output").length, boxes: boxes.children.length };
console.log(JSON.stringify(out));"#
                .replace("ALONE", &serde_json::to_string(&alone).expect("serialises"))
                .replace("RUNNING", &serde_json::to_string(&running).expect("serialises"))
                .replace("MOVED", &serde_json::to_string(&moved).expect("serialises"))
                .replace("FINISHED", &serde_json::to_string(&finished).expect("serialises"))
                .replace("OUTPUT", &serde_json::to_string(&output).expect("serialises"));
        let folder = std::env::temp_dir().join(format!("smoozy-set-up-server-output-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js");
        std::fs::copy(&page, folder.join("set-up.js")).expect("the page is copied");
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => harness.clone(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Set up tab's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let control = Prose::new(words::SERVER_OUTPUT);
        let lines: Vec<String> = three_lines();
        assert_eq!(
            out["finished"],
            serde_json::json!({ "log": null, "suout": [["button", "mini", control.as_str()]] }),
            "{out}"
        );
        assert_eq!(out["alone"], serde_json::json!({ "log": "log nolines", "lines": 0, "control": control.as_str() }), "{out}");
        assert_eq!(out["running"]["work"], serde_json::json!(["steps", "log"]), "the window is not beside the steps: {out}");
        assert_eq!(out["running"]["lines"], serde_json::json!(lines[..2]), "{out}");
        assert_eq!(out["running"]["isolated"], serde_json::json!(["bdi", "bdi"]), "each line one isolated run: {out}");
        assert_eq!(out["moved"]["lines"], serde_json::json!(lines), "{out}");
        assert_eq!(out["moved"]["theSameCard"], true, "the card was built again for lines that moved: {out}");
        assert_eq!(out["opened"], serde_json::json!({ "asked": 1, "boxes": 1 }), "{out}");
    }

    /// **The Creator's fourteenth notes, note 1, piece S, on the page: an
    /// install's lines stay in the window while the step after it runs and
    /// prints nothing.** The two panels are the application's own, composed
    /// by [`the_setup_under_way_on`] from the real store — the install's
    /// pieces kept under its step, then the run read at the next step with
    /// nothing printed — and the real `web/set-up.js` draws them under Node.
    /// **Before piece S the second panel carried no lines**, so the window went
    /// the moment the next step was marked; the page itself is unchanged.
    #[test]
    fn piece_s_the_page_keeps_an_installs_lines_beside_the_next_step() {
        use crate::servers::sessions::APiecePrinted;
        use smoozy_server_setup::ready_action::Step;
        let server = "piece-s-the-page";
        let composed = vec![
            Boundary::Started,
            Boundary::Fetched,
            Boundary::Tooling,
            Boundary::Libraries,
            Boundary::Blender,
            Boundary::Done,
        ];
        let read_at = |under_way: Boundary, reached: Vec<Boundary>| crate::setting_up::TheSetupAsRead {
            the_run_holds_the_machine: true,
            record: TheSetupRecord {
                kind: SetupKind::TheReadyAction,
                the_composed_list: composed.clone(),
                the_boundaries_reached: reached
                    .into_iter()
                    .map(|boundary| a_boundary(boundary, WhatTheStepDid::Performed))
                    .collect(),
                ending: None,
            },
            still_open: true,
            the_step_in_flight: Some(under_way),
            the_admin_step_recorded_as_started: None,
            the_clock: String::new(),
        };
        assert!(crate::setting_up::a_run_begins(server, SetupKind::TheReadyAction));
        assert!(crate::setting_up::the_run_is_read_under_way(
            server,
            &read_at(Boundary::Tooling, vec![Boundary::Started, Boundary::Fetched])
        ));
        crate::setting_up::a_piece_is_kept(
            server,
            Step::Tooling,
            &APiecePrinted::Output(
                b"Reading package lists...\nUnpacking xz-utils (5.2.5-2ubuntu1) ...\nSetting up xz-utils (5.2.5-2ubuntu1) ...\n"
                    .to_vec(),
            ),
        );
        let installing = the_setup_under_way_on(server, || "Vast 4090".to_string()).expect("the run's panel");
        assert!(crate::setting_up::the_run_is_read_under_way(
            server,
            &read_at(Boundary::Libraries, vec![Boundary::Started, Boundary::Fetched, Boundary::Tooling])
        ));
        let the_next_step = the_setup_under_way_on(server, || "Vast 4090".to_string()).expect("the run's panel");
        crate::setting_up::a_run_ends(server);
        let mut finished = the_tab(WhatTheMachineSaid::ConnectedAndNoRunWasFound, server);
        the_servers_words_beside_the_tab(&mut finished, true);

        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("const running = THE_RUNNING;")
            .next()
            .expect("the harness's opening")
            .to_string()
            + &r#"const installing = INSTALLING;
const theNextStep = THE_NEXT_STEP;
const finished = FINISHED;
let theRun = null;
const readings = [];
globalThis.window = {
  __TAURI__: {
    core: {
      invoke(command) {
        if (command === "the_set_up_tab") return new Promise((done) => readings.push(done));
        if (command === "the_setup_under_way") return Promise.resolve(theRun);
        return Promise.resolve(null);
      },
    },
  },
};
const settle = async () => { for (let at = 0; at < 6; at += 1) await new Promise((done) => setImmediate(done)); };
const tell = (said) => (globalThis.told.get("set-up:the-answer") || []).forEach((one) => one({ said }));
await import("./set-up.js");
const host = anElement("div");
globalThis.tab.itIsDrawn(host);
globalThis.tab.itIsShowing("SERVER");
await settle();
readings.splice(0).forEach((done) => done(finished));
await settle();
const [scroll] = host.children;
const [card] = scroll.children;
const find = (name) => every(card).find((one) => one.classList && one.classList.contains(name));
const theStepUnderWay = () => {
  const now = every(card).find((one) => one.tag === "li" && one.classList.contains("now"));
  return now ? theTextOf(now.children[0]) : null;
};
const theWindow = () => (find("qlines") ? find("qlines").children.map(theTextOf) : []);
const out = {};
theRun = installing;
tell({ server: "SERVER", the_run_is_under_way: "yes" });
await settle();
out.installing = { step: theStepUnderWay(), lines: theWindow() };
theRun = theNextStep;
tell({ server: "SERVER", the_run_is_under_way: "yes" });
await settle();
out.next = { step: theStepUnderWay(), lines: theWindow() };
console.log(JSON.stringify(out));"#
                .replace("SERVER", server)
                .replace("INSTALLING", &serde_json::to_string(&installing).expect("serialises"))
                .replace("THE_NEXT_STEP", &serde_json::to_string(&the_next_step).expect("serialises"))
                .replace("FINISHED", &serde_json::to_string(&finished).expect("serialises"));
        let folder = std::env::temp_dir().join(format!("smoozy-set-up-piece-s-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js");
        std::fs::copy(&page, folder.join("set-up.js")).expect("the page is copied");
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => harness.clone(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Set up tab's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let lines = serde_json::json!([
            "Reading package lists...",
            "Unpacking xz-utils (5.2.5-2ubuntu1) ...",
            "Setting up xz-utils (5.2.5-2ubuntu1) ...",
        ]);
        assert_eq!(out["installing"]["lines"], lines, "{out}");
        assert_ne!(out["installing"]["step"], out["next"]["step"], "the run did not move on: {out}");
        assert_eq!(out["next"]["lines"], lines, "the install's lines went when the next step was marked: {out}");
    }

    /// The real Set up page and shared words panel: Copy follows the selected
    /// stream through empty readings, first words, later words and the ending.
    #[test]
    fn seventeenth_piece_5_copy_follows_the_open_setup_stream_as_words_arrive() {
        let mut finished = the_tab(WhatTheMachineSaid::ConnectedAndNoRunWasFound, "server-1");
        the_servers_words_beside_the_tab(&mut finished, true);
        let readings = [
            the_servers_output_from("Container", "", "", true),
            the_servers_output_from("Container", "", "first complaint\n", true),
            the_servers_output_from("Container", "first output\n", "first complaint\n", true),
            the_servers_output_from("Container", "first output\nmore output\n", "first complaint\n", true),
            the_servers_output_from("Container", "", "", true),
            the_servers_output_from("Container", "", "", false),
        ];
        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("const running = THE_RUNNING;").next().unwrap()
            .replace("function anElement(tag)", "function aBareElement(tag)")
            + &r#"
function anElement(tag) {
  const one = aBareElement(tag);
  one.attributes = {};
  one.setAttribute = (key, value) => { one.attributes[key] = String(value); };
  one.getAttribute = key => one.attributes[key];
  one.querySelectorAll = selector => every(one).slice(1).filter(node => {
    if (!node.classList) return false;
    const [tag, ...classes] = selector.split('.');
    return (!tag || node.tag === tag) && classes.every(name => node.classList.contains(name));
  });
  one.querySelector = selector => one.querySelectorAll(selector)[0] || null;
  one.closest = selector => {
    for (let up = one; up; up = up.parentNode) {
      if (selector[0] === '.' && up.classList && up.classList.contains(selector.slice(1))) return up;
    }
    return null;
  };
  one.click = one.press;
  return one;
}
document.createElement = anElement;
document.createElementNS = (namespace, tag) => anElement(tag);
const outputs = OUTPUTS;
const finished = FINISHED;
let output = outputs[0];
globalThis.window = {__TAURI__:{core:{invoke(command) {
  if (command === 'the_set_up_tab') return Promise.resolve(finished);
  if (command === 'the_servers_output') return Promise.resolve(output);
  return Promise.resolve(null);
}}}};
const settle = async () => { for (let at = 0; at < 6; at++) await new Promise(setImmediate); };
const tell = () => (told.get('set-up:the-answer') || []).forEach(one => one({said:{server:'server-1',the_servers_words_moved:'yes'}}));
await import('./set-up.js');
const host = anElement('div');
tab.itIsDrawn(host); tab.itIsShowing('server-1');
await settle();
// The finished card still offers the run's kept output.
const card = host.children[0].children[0];
// Exercise the actual open route through the card's control.
const open = every(card).find(one => one.tag === 'button' && one.textContent === OUTPUT_LABEL);
open.press(); await settle();
const panel = () => host.querySelector('.words-panel');
const bodies = () => panel().querySelectorAll('.words-body');
const tabs = () => panel().querySelectorAll('.ltab');
const state = () => ({copy:!panel().querySelector('.words-copy').hidden, selected:bodies().findIndex(one => !one.hidden), text:bodies().find(one => !one.hidden).textContent});
const out = {empty:state()};
tabs()[1].press(); out.emptySwitch = state();
output = outputs[1]; tell(); await settle(); out.firstComplaint = state();
tabs()[0].press(); out.backToEmpty = state();
output = outputs[2]; tell(); await settle(); out.firstOutput = state();
tabs()[1].press(); out.printedSwitch = state();
output = outputs[3]; tell(); await settle(); out.otherStreamGrows = state();
tabs()[0].press(); out.appended = state();
output = outputs[4]; tell(); await settle(); out.quiet = state();
output = outputs[5]; tell(); await settle(); out.ended = state();
console.log(JSON.stringify(out));
"#.replace("OUTPUTS", &serde_json::to_string(&readings).unwrap())
                .replace("FINISHED", &serde_json::to_string(&finished).unwrap())
                .replace("OUTPUT_LABEL", &serde_json::to_string(&Prose::new(words::SERVER_OUTPUT)).unwrap());
        let folder = std::env::temp_dir().join(format!("smoozy-setup-piece-5-{}", std::process::id()));
        fs::create_dir_all(&folder).unwrap();
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => harness.as_str(),
                "the-machines-own-words.js" => include_str!("../../web/the-machines-own-words.js"),
                "the-failure-shape.js" => "export const EXPANDED='expanded'; export function theFailureShape() {} export function theControlsFrom() {}",
                _ => text,
            };
            fs::write(folder.join(name), text).unwrap();
        }
        fs::write(folder.join("set-up.js"), the_page()).unwrap();
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let ran = match ran {
            Ok(ran) => ran,
            Err(why) if why.kind() == std::io::ErrorKind::NotFound => {
                eprintln!("SKIPPED: no Node on this computer, so Set up's Copy check was not run");
                return;
            }
            Err(why) => panic!("Node could not run Set up's Copy check: {why}"),
        };
        assert!(ran.status.success(), "{}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).unwrap();
        for name in ["empty", "emptySwitch", "backToEmpty", "quiet", "ended"] {
            assert_eq!(out[name]["copy"], false, "{name}: {out}");
        }
        for name in ["firstComplaint", "firstOutput", "printedSwitch", "otherStreamGrows", "appended"] {
            assert_eq!(out[name]["copy"], true, "{name}: {out}");
        }
        assert_eq!(out["firstComplaint"]["selected"], 1, "first words changed the chosen tab: {out}");
        assert_eq!(out["otherStreamGrows"]["selected"], 1, "another stream took selection: {out}");
        assert_eq!(out["appended"]["text"], "first output\nmore output\n", "{out}");
        assert_eq!(out["ended"]["text"], Prose::new(words::THE_SERVER_PRINTED_NO_OUTPUT).as_str(), "{out}");
    }

    /// **The eighteenth notes, piece 2c, on the page — the window's lines
    /// from the error stream carry the drawing's `err`, the brighter ink, and
    /// the others do not; and once the run has stopped, *Server output* sits
    /// on its own line under the steps** — the drawing's `.suout` holding the
    /// `mini` control, straight after the step list and before the card's
    /// sentence, with no `.log` left at the top right (`Design/Server Setup §
    /// A long step`; drawn as `su-out` and `su-stepten`). The real
    /// `web/set-up.js` under Node. **Before piece 2c every window line was
    /// drawn alike, and the stopped card's control floated in an empty
    /// `.log` beside step 1** (the Creator's screenshot 1, note 4).
    #[test]
    fn eighteenth_the_page_marks_the_error_streams_lines_and_puts_server_output_under_a_stopped_cards_steps() {
        use smoozy_server_setup::elevated::WhichStream;
        let mut running = a_run_at_the_driver_step();
        the_servers_window_beside_the_run(
            &mut running,
            true,
            vec![
                (WhichStream::TheOrdinaryOutput, "Setting up nvidia-dkms-595 (595.91.07-0ubuntu1) ...".to_string()),
                (WhichStream::TheComplaints, "debconf: delaying package configuration, since apt-utils is not installed".to_string()),
                (WhichStream::TheOrdinaryOutput, "Building initial module nvidia/595.91.07 for 6.8.0-45-gen".to_string()),
            ],
        );
        assert_eq!(running.which_lines_are_from_the_error_stream, vec![false, true, false]);
        let mut stopped = a_run_at_the_driver_step();
        stopped.cancel = None;
        stopped.set_up_again = Some(Prose::new(words::SET_UP_AGAIN));
        let mut finished = the_tab(WhatTheMachineSaid::ConnectedAndNoRunWasFound, "server-1");
        finished.the_stopped_setup = Some(stopped);
        the_servers_words_beside_the_tab(&mut finished, true);
        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("const running = THE_RUNNING;")
            .next()
            .expect("the harness's opening")
            .to_string()
            + &r#"const running = RUNNING;
const finished = FINISHED;
let theRun = null;
const readings = [];
globalThis.window = { __TAURI__: { core: { invoke(command) {
  if (command === "the_set_up_tab") return new Promise((done) => readings.push(done));
  if (command === "the_setup_under_way") return Promise.resolve(theRun);
  return Promise.resolve(null);
} } } };
const settle = async () => { for (let at = 0; at < 6; at += 1) await new Promise((done) => setImmediate(done)); };
const tell = (said) => (globalThis.told.get("set-up:the-answer") || []).forEach((one) => one({ said }));
await import("./set-up.js");
const host = anElement("div");
globalThis.tab.itIsDrawn(host);
globalThis.tab.itIsShowing("server-1");
await settle();
readings.splice(0).forEach((done) => done(finished));
await settle();
const [scroll] = host.children;
const [card] = scroll.children;
const named = (name) => every(card).filter((one) => one.classList && one.classList.contains(name));
const sucard = named("sucard")[0];
const out = {};
out.stopped = {
  children: sucard.children.map((one) => one.tag + (one.className ? "." + one.className : "")),
  suout: named("suout").map((one) => one.children.map((child) => [child.tag, child.className, theTextOf(child)])),
  logs: named("log").length,
};
theRun = running;
tell({ server: "server-1", the_run_is_under_way: "yes" });
await settle();
const window_ = named("qlines")[0];
out.running = {
  classes: window_.children.map((one) => one.className),
  lines: window_.children.map(theTextOf),
  suout: named("suout").length,
};
console.log(JSON.stringify(out));"#
                .replace("RUNNING", &serde_json::to_string(&running).expect("serialises"))
                .replace("FINISHED", &serde_json::to_string(&finished).expect("serialises"));
        let folder = std::env::temp_dir().join(format!("smoozy-set-up-eighteenth-2c-page-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        std::fs::copy(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js"), folder.join("set-up.js"))
            .expect("the page is copied");
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => harness.clone(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Set up tab's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let control = Prose::new(words::SERVER_OUTPUT);
        assert_eq!(out["stopped"]["logs"], 0, "a .log was drawn on the stopped card: {out}");
        assert_eq!(
            out["stopped"]["suout"],
            serde_json::json!([[["button", "mini", control.as_str()]]]),
            "Server output is not on its own line on the stopped card: {out}"
        );
        let children: Vec<String> = serde_json::from_value(out["stopped"]["children"].clone()).expect("a list");
        let steps = children.iter().position(|one| one == "ol.steps").expect("the steps are drawn");
        assert_eq!(children.get(steps + 1).map(String::as_str), Some("div.suout"), "not straight under the steps: {out}");
        assert_eq!(out["running"]["classes"], serde_json::json!(["", "err", ""]), "{out}");
        assert_eq!(out["running"]["suout"], 0, "a running card drew Server output under its steps: {out}");
    }

    /// **The eighteenth notes, piece 2c — *Server output* shows a line a
    /// program rewrites in place in its newest form, as a terminal shows it,
    /// and Copy takes the words as they were printed** (`Desktop app and
    /// delivery` M13b as changed on 2026-10-24; `Design/Server Setup § Nothing
    /// is silent while a setup runs`: *"Copy takes the words as they were
    /// printed"*). The real Set up page and shared words panel: opened on a
    /// line part-way through, the line rewritten twice as the run prints, the
    /// words put in place in the same panel each time, and Copy handing over
    /// every byte as printed, carriage returns and all. **Before piece 2c the
    /// panel showed the carriage returns' every form run together, and a
    /// rewrite drew the whole panel again.**
    #[test]
    fn eighteenth_server_output_shows_a_rewritten_line_newest_and_copy_takes_it_as_printed() {
        let mut finished = the_tab(WhatTheMachineSaid::ConnectedAndNoRunWasFound, "server-1");
        the_servers_words_beside_the_tab(&mut finished, true);
        let readings = [
            the_servers_output_from("Container", "Reading 12%", "", true),
            the_servers_output_from("Container", "Reading 12%\rReading 57%", "", true),
            the_servers_output_from("Container", "Reading 12%\rReading 57%\rReading 100%\ndone\n", "", true),
        ];
        assert_eq!(readings[1].tabs[0].record, Some(Verbatim::new("Reading 57%")));
        assert_eq!(readings[1].tabs[0].as_printed, Some(Verbatim::new("Reading 12%\rReading 57%")));
        assert_eq!(readings[0].tabs[0].as_printed, None, "the words as printed were carried where they are the same");
        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("const running = THE_RUNNING;").next().unwrap()
            .replace("function anElement(tag)", "function aBareElement(tag)")
            + &r#"
function anElement(tag) {
  const one = aBareElement(tag);
  one.attributes = {};
  one.setAttribute = (key, value) => { one.attributes[key] = String(value); };
  one.getAttribute = key => one.attributes[key];
  one.querySelectorAll = selector => every(one).slice(1).filter(node => {
    if (!node.classList) return false;
    const [tag, ...classes] = selector.split('.');
    return (!tag || node.tag === tag) && classes.every(name => node.classList.contains(name));
  });
  one.querySelector = selector => one.querySelectorAll(selector)[0] || null;
  one.closest = selector => {
    for (let up = one; up; up = up.parentNode) {
      if (selector[0] === '.' && up.classList && up.classList.contains(selector.slice(1))) return up;
    }
    return null;
  };
  one.click = one.press;
  return one;
}
document.createElement = anElement;
document.createElementNS = (namespace, tag) => anElement(tag);
const copied = [];
Object.defineProperty(globalThis, 'navigator', { configurable: true, value: { clipboard: { writeText(text) { copied.push(text); return Promise.resolve(); } } } });
const outputs = OUTPUTS;
const finished = FINISHED;
let output = outputs[0];
globalThis.window = {__TAURI__:{core:{invoke(command) {
  if (command === 'the_set_up_tab') return Promise.resolve(finished);
  if (command === 'the_servers_output') return Promise.resolve(output);
  return Promise.resolve(null);
}}}};
const settle = async () => { for (let at = 0; at < 6; at++) await new Promise(setImmediate); };
const tell = () => (told.get('set-up:the-answer') || []).forEach(one => one({said:{server:'server-1',the_servers_words_moved:'yes'}}));
await import('./set-up.js');
const host = anElement('div');
tab.itIsDrawn(host); tab.itIsShowing('server-1');
await settle();
const card = host.children[0].children[0];
const open = every(card).find(one => one.tag === 'button' && one.textContent === OUTPUT_LABEL);
open.press(); await settle();
const panel = () => host.querySelector('.words-panel');
const first = panel();
const shown = () => panel().querySelector('pre.lbody').textContent;
const copy = async () => { panel().querySelector('.words-copy').press(); await settle(); return copied[copied.length - 1]; };
const out = { opened: shown(), openedCopy: await copy() };
output = outputs[1]; tell(); await settle();
out.rewritten = { shown: shown(), samePanel: panel() === first, panels: host.querySelectorAll('.words-panel').length, copy: await copy() };
output = outputs[2]; tell(); await settle();
out.ended = { shown: shown(), samePanel: panel() === first, copy: await copy() };
console.log(JSON.stringify(out));
"#.replace("OUTPUTS", &serde_json::to_string(&readings).unwrap())
                .replace("FINISHED", &serde_json::to_string(&finished).unwrap())
                .replace("OUTPUT_LABEL", &serde_json::to_string(&Prose::new(words::SERVER_OUTPUT)).unwrap());
        let folder = std::env::temp_dir().join(format!("smoozy-setup-eighteenth-rewritten-{}", std::process::id()));
        fs::create_dir_all(&folder).unwrap();
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => harness.as_str(),
                "the-machines-own-words.js" => include_str!("../../web/the-machines-own-words.js"),
                "the-failure-shape.js" => "export const EXPANDED='expanded'; export function theFailureShape() {} export function theControlsFrom() {}",
                _ => text,
            };
            fs::write(folder.join(name), text).unwrap();
        }
        fs::write(folder.join("set-up.js"), the_page()).unwrap();
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let ran = match ran {
            Ok(ran) => ran,
            Err(why) if why.kind() == std::io::ErrorKind::NotFound => {
                eprintln!("SKIPPED: no Node on this computer, so Set up's rewritten-line check was not run");
                return;
            }
            Err(why) => panic!("Node could not run Set up's rewritten-line check: {why}"),
        };
        assert!(ran.status.success(), "{}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).unwrap();
        assert_eq!(out["opened"], "Reading 12%", "{out}");
        assert_eq!(out["openedCopy"], "Reading 12%", "{out}");
        assert_eq!(out["rewritten"]["shown"], "Reading 57%", "the rewritten line is not shown newest: {out}");
        assert_eq!(out["rewritten"]["samePanel"], true, "a rewrite drew the panel again: {out}");
        assert_eq!(out["rewritten"]["panels"], 1, "{out}");
        assert_eq!(out["rewritten"]["copy"], "Reading 12%\rReading 57%", "Copy did not take the words as printed: {out}");
        assert_eq!(out["ended"]["shown"], "Reading 100%\ndone\n", "{out}");
        assert_eq!(out["ended"]["samePanel"], true, "{out}");
        assert_eq!(out["ended"]["copy"], "Reading 12%\rReading 57%\rReading 100%\ndone\n", "{out}");
    }

    /// **The sixteenth notes, note 1 — `Design/Server Setup § A long step`:
    /// where a step's words could not be brought back, each tab says *Some of
    /// what the server printed could not be shown.* under the words it has, and
    /// Copy never takes it.** The real Set up page and shared words panel,
    /// opened through the card's own control: the sentence stands under the
    /// Output tab's words and in the Errors tab that printed nothing, Copy on
    /// Output hands over the printed words alone, and Errors offers no Copy.
    /// Where nothing was lost, no tab says it. Added 2026-10-24.
    #[test]
    fn y7a_the_not_shown_sentence_stands_under_the_words_and_copy_never_takes_it() {
        let lost = the_servers_output_kept("Container", "first output\n", "", false, true);
        for tab in &lost.tabs {
            assert_eq!(
                tab.not_shown,
                Some(Prose::new(words::SOME_OF_WHAT_THE_SERVER_PRINTED_COULD_NOT_BE_SHOWN)),
                "{tab:?}"
            );
            assert!(
                tab.record.as_ref().is_none_or(|record| !record.as_str().contains("truncated")),
                "the sentence went into the printed words"
            );
        }
        assert!(the_servers_output_from("Container", "first output\n", "", false)
            .tabs
            .iter()
            .all(|tab| tab.not_shown.is_none()));
        let mut finished = the_tab(WhatTheMachineSaid::ConnectedAndNoRunWasFound, "server-1");
        the_servers_words_beside_the_tab(&mut finished, true);
        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("const running = THE_RUNNING;").next().unwrap()
            .replace("function anElement(tag)", "function aBareElement(tag)")
            + &r#"
function anElement(tag) {
  const one = aBareElement(tag);
  one.attributes = {};
  one.setAttribute = (key, value) => { one.attributes[key] = String(value); };
  one.getAttribute = key => one.attributes[key];
  one.querySelectorAll = selector => every(one).slice(1).filter(node => {
    if (!node.classList) return false;
    const [tag, ...classes] = selector.split('.');
    return (!tag || node.tag === tag) && classes.every(name => node.classList.contains(name));
  });
  one.querySelector = selector => one.querySelectorAll(selector)[0] || null;
  one.closest = selector => {
    for (let up = one; up; up = up.parentNode) {
      if (selector[0] === '.' && up.classList && up.classList.contains(selector.slice(1))) return up;
    }
    return null;
  };
  one.click = one.press;
  return one;
}
document.createElement = anElement;
document.createElementNS = (namespace, tag) => anElement(tag);
const copied = [];
Object.defineProperty(globalThis, 'navigator', { configurable: true, value: { clipboard: { writeText(text) { copied.push(text); return Promise.resolve(); } } } });
const output = OUTPUT;
const finished = FINISHED;
globalThis.window = {__TAURI__:{core:{invoke(command) {
  if (command === 'the_set_up_tab') return Promise.resolve(finished);
  if (command === 'the_servers_output') return Promise.resolve(output);
  return Promise.resolve(null);
}}}};
const settle = async () => { for (let at = 0; at < 6; at++) await new Promise(setImmediate); };
await import('./set-up.js');
const host = anElement('div');
tab.itIsDrawn(host); tab.itIsShowing('server-1');
await settle();
const card = host.children[0].children[0];
const open = every(card).find(one => one.tag === 'button' && one.textContent === OUTPUT_LABEL);
open.press(); await settle();
const panel = () => host.querySelector('.words-panel');
const bodies = () => panel().querySelectorAll('.words-body');
const tabs = () => panel().querySelectorAll('.ltab');
const sentence = body => { const one = body.querySelector('p.words-not-shown'); return one ? one.textContent : null; };
const copy = () => panel().querySelector('.words-copy');
const out = { outputSentence: sentence(bodies()[0]), outputCopy: !copy().hidden, outputPrinted: bodies()[0].querySelector('pre.lbody').textContent };
copy().press(); await settle();
out.copied = copied.slice();
tabs()[1].press();
out.errorsSentence = sentence(bodies()[1]);
out.errorsCopy = !copy().hidden;
console.log(JSON.stringify(out));
"#.replace("OUTPUT_LABEL", &serde_json::to_string(&Prose::new(words::SERVER_OUTPUT)).unwrap())
                .replace("OUTPUT", &serde_json::to_string(&lost).unwrap())
                .replace("FINISHED", &serde_json::to_string(&finished).unwrap());
        let folder = std::env::temp_dir().join(format!("smoozy-setup-y7a-not-shown-{}", std::process::id()));
        fs::create_dir_all(&folder).unwrap();
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => harness.as_str(),
                "the-machines-own-words.js" => include_str!("../../web/the-machines-own-words.js"),
                "the-failure-shape.js" => "export const EXPANDED='expanded'; export function theFailureShape() {} export function theControlsFrom() {}",
                _ => text,
            };
            fs::write(folder.join(name), text).unwrap();
        }
        fs::write(folder.join("set-up.js"), the_page()).unwrap();
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let ran = match ran {
            Ok(ran) => ran,
            Err(why) if why.kind() == std::io::ErrorKind::NotFound => {
                eprintln!("SKIPPED: no Node on this computer, so Set up's sentence check was not run");
                return;
            }
            Err(why) => panic!("Node could not run Set up's sentence check: {why}"),
        };
        assert!(ran.status.success(), "{}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).unwrap();
        let the_sentence = Prose::new(words::SOME_OF_WHAT_THE_SERVER_PRINTED_COULD_NOT_BE_SHOWN);
        assert_eq!(out["outputSentence"], the_sentence.as_str(), "{out}");
        assert_eq!(out["errorsSentence"], the_sentence.as_str(), "{out}");
        assert_eq!(out["outputPrinted"], "first output\n", "the sentence went into the printed words: {out}");
        assert_eq!(out["outputCopy"], true, "{out}");
        assert_eq!(out["copied"], serde_json::json!(["first output\n"]), "Copy took more than the printed words: {out}");
        assert_eq!(out["errorsCopy"], false, "Copy was offered for a tab holding only the sentence: {out}");
    }

    /// **The panels piece U's two tests draw**, each composed by
    /// [`the_setup_under_way_on`] from the real store: a Blender install of
    /// 5.2.2 under way at its download with a count and a whole, with the
    /// count gone, with a count and no whole, and at its unpack with a count.
    fn piece_u_the_panels(server: &str) -> [TheRunPanel; 4] {
        use crate::setting_up::{HowFarTheStepHasGot, TheCountedAct, TheStepsCount};
        let composed = vec![
            Boundary::Started,
            Boundary::Fetched,
            Boundary::Tooling,
            Boundary::Libraries,
            Boundary::Blender,
            Boundary::Done,
        ];
        let read_at = |under_way: Boundary, reached: Vec<Boundary>| crate::setting_up::TheSetupAsRead {
            the_run_holds_the_machine: true,
            record: TheSetupRecord {
                kind: SetupKind::ABlenderInstall,
                the_composed_list: composed.clone(),
                the_boundaries_reached: reached
                    .into_iter()
                    .map(|boundary| a_boundary(boundary, WhatTheStepDid::Performed))
                    .collect(),
                ending: None,
            },
            still_open: true,
            the_step_in_flight: Some(under_way),
            the_admin_step_recorded_as_started: None,
            the_clock: String::new(),
        };
        let counted = |act, so_far, whole| Some(HowFarTheStepHasGot { act, count: TheStepsCount { so_far, whole } });
        assert!(crate::setting_up::a_run_begins(server, SetupKind::ABlenderInstall));
        let told = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let go = {
            let told = std::sync::Arc::clone(&told);
            crate::setting_up::the_figure_goes_to_the_card(server, Some("5.2.2"), move || {
                told.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            })
        };
        assert!(crate::setting_up::the_run_is_read_under_way(server, &read_at(Boundary::Fetched, vec![Boundary::Started])));
        let panel = || the_setup_under_way_on(server, || "Vast 4090".to_string()).expect("the run's panel");
        go(counted(TheCountedAct::TheDownload, 214_000_000, Some(352_123_456)));
        let downloading = panel();
        // **The tab is told where the figure moved, and only there.**
        go(counted(TheCountedAct::TheDownload, 214_000_000, Some(352_123_456)));
        assert_eq!(told.load(std::sync::atomic::Ordering::SeqCst), 1, "the tab was told of a figure that did not move");
        go(None);
        let no_count = panel();
        go(counted(TheCountedAct::TheDownload, 214_999_999, None));
        let no_size = panel();
        assert!(crate::setting_up::the_run_is_read_under_way(
            server,
            &read_at(Boundary::Tooling, vec![Boundary::Started, Boundary::Fetched])
        ));
        go(counted(TheCountedAct::TheUnpack, 144_320_000, Some(352_000_000)));
        let unpacking = panel();
        assert_eq!(told.load(std::sync::atomic::Ordering::SeqCst), 4);
        crate::setting_up::a_run_ends(server);
        // **Nothing is kept once the run is over**, and a figure arriving
        // after it is not held against the next run.
        go(counted(TheCountedAct::TheUnpack, 1, Some(2)));
        assert_eq!(crate::setting_up::the_builds_figures_on(server), None);
        [downloading, no_count, no_size, unpacking]
    }

    fn the_figures_text(figure: &AStepsFigure) -> String {
        [Some(figure.so_far.as_str()), figure.of.as_ref().map(|it| it.as_str()), figure.whole.as_ref().map(|it| it.as_str())]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// **`Design/Server Setup § A long step`, `Server Setup` Y7b and `Server Setup` Y7c:
    /// the download's step is named for the build and its note reads the
    /// count — *214 of 352 MB*, *214 MB* with no size — and nothing without
    /// one; the unpack's step reads *Unpack Blender 5.2.2* with *41%*; no
    /// other step carries a figure and none reaches the server's window.** The
    /// Creator's fourteenth notes, piece U.
    #[test]
    fn piece_u_the_downloads_step_reads_the_count_and_shows_nothing_without_one() {
        let [downloading, no_count, no_size, unpacking] = piece_u_the_panels("piece-u-the-count");
        let the_step = |panel: &TheRunPanel, boundary: Boundary| {
            panel.steps.iter().find(|step| step.boundary == boundary).cloned().expect("the step is listed")
        };
        let mb = crate::typography::BETWEEN_A_FIGURE_AND_ITS_UNIT;
        // **The step's name through the typography point**, which binds the
        // build's figure to the word before it.
        let download_5_2_2 = Prose::new("Download Blender 5.2.2");
        let unpack_5_2_2 = Prose::new("Unpack Blender 5.2.2");

        let download = the_step(&downloading, Boundary::Fetched);
        assert_eq!(download.name, download_5_2_2);
        assert_eq!(download.mark.as_str(), "now");
        assert_eq!(download.the_figure.as_ref().map(the_figures_text), Some(format!("214 of 352{mb}MB")));
        assert_eq!(download.the_figure.as_ref().map(|it| it.so_far.as_str()), Some("214"), "the figure is a value of its own");
        assert_eq!(download.said, None);

        let without = the_step(&no_count, Boundary::Fetched);
        assert_eq!(without.name, download_5_2_2, "the step is named for the build without a count too");
        assert_eq!(without.the_figure, None, "a figure was drawn with no count in the reading");

        assert_eq!(
            the_step(&no_size, Boundary::Fetched).the_figure.as_ref().map(the_figures_text),
            Some(format!("214{mb}MB"))
        );

        let unpack = the_step(&unpacking, Boundary::Tooling);
        assert_eq!(unpack.name, unpack_5_2_2);
        assert_eq!(unpack.the_figure.as_ref().map(the_figures_text), Some("41%".to_string()));
        let done = the_step(&unpacking, Boundary::Fetched);
        assert_eq!(done.name, download_5_2_2, "the download is named for the build once done");
        assert_eq!(done.the_figure, None);

        for panel in [&downloading, &no_count, &no_size, &unpacking] {
            for step in &panel.steps {
                if step.mark.as_str() != "now" {
                    assert_eq!(step.the_figure, None, "{} carries a figure and is not under way", step.name.as_str());
                }
            }
            assert!(panel.the_servers_newest_lines.is_empty(), "the figure reached the server's window");
        }
        // **Before the unpack counts, the tooling step keeps its own name.**
        assert_eq!(the_step(&downloading, Boundary::Tooling).name, Prose::new(words::TOOLING));
    }

    /// **Piece U on the page: the real `web/set-up.js` draws the download's
    /// figure in the step's small note, plain and beside the step's name, and
    /// draws nothing where the reading carries no count.** The panels are the
    /// application's own ([`piece_u_the_panels`]).
    #[test]
    fn piece_u_the_page_draws_the_steps_figure_in_its_note() {
        let server = "piece-u-the-page";
        let [downloading, no_count, no_size, unpacking] = piece_u_the_panels(server);
        let mut finished = the_tab(WhatTheMachineSaid::ConnectedAndNoRunWasFound, server);
        the_servers_words_beside_the_tab(&mut finished, true);
        let harness = THE_RUN_ON_THE_CARD_HARNESS
            .split("const running = THE_RUNNING;")
            .next()
            .expect("the harness's opening")
            .to_string()
            + &r#"const panels = { downloading: DOWNLOADING, noCount: NO_COUNT, noSize: NO_SIZE, unpacking: UNPACKING };
const finished = FINISHED;
let theRun = null;
const readings = [];
globalThis.window = {
  __TAURI__: {
    core: {
      invoke(command) {
        if (command === "the_set_up_tab") return new Promise((done) => readings.push(done));
        if (command === "the_setup_under_way") return Promise.resolve(theRun);
        return Promise.resolve(null);
      },
    },
  },
};
const settle = async () => { for (let at = 0; at < 6; at += 1) await new Promise((done) => setImmediate(done)); };
const tell = (said) => (globalThis.told.get("set-up:the-answer") || []).forEach((one) => one({ said }));
await import("./set-up.js");
const host = anElement("div");
globalThis.tab.itIsDrawn(host);
globalThis.tab.itIsShowing("SERVER");
await settle();
readings.splice(0).forEach((done) => done(finished));
await settle();
const [scroll] = host.children;
const [card] = scroll.children;
const out = {};
for (const [name, panel] of Object.entries(panels)) {
  theRun = panel;
  tell({ server: "SERVER", the_run_is_under_way: "yes" });
  await settle();
  const now = every(card).find((one) => one.tag === "li" && one.classList.contains("now"));
  const notes = now.children.filter((one) => one.classList && one.classList.contains("stm"));
  out[name] = {
    name: theTextOf(now.children[0]),
    notes: notes.map(theTextOf),
    bold: notes.flatMap((one) => every(one)).filter((one) => one.tag === "b").length,
    window: every(card).filter((one) => one.classList && one.classList.contains("qlines")).length,
  };
}
console.log(JSON.stringify(out));"#
                .replace("SERVER", server)
                .replace("DOWNLOADING", &serde_json::to_string(&downloading).expect("serialises"))
                .replace("NO_COUNT", &serde_json::to_string(&no_count).expect("serialises"))
                .replace("NO_SIZE", &serde_json::to_string(&no_size).expect("serialises"))
                .replace("UNPACKING", &serde_json::to_string(&unpacking).expect("serialises"))
                .replace("FINISHED", &serde_json::to_string(&finished).expect("serialises"));
        let folder = std::env::temp_dir().join(format!("smoozy-set-up-piece-u-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/set-up.js");
        std::fs::copy(&page, folder.join("set-up.js")).expect("the page is copied");
        for (name, text) in THE_SET_UP_STAND_INS {
            let text = match name {
                "harness.js" => harness.clone(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Set up tab's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let mb = crate::typography::BETWEEN_A_FIGURE_AND_ITS_UNIT;
        let download_5_2_2 = Prose::new("Download Blender 5.2.2");
        let unpack_5_2_2 = Prose::new("Unpack Blender 5.2.2");
        assert_eq!(
            out["downloading"],
            serde_json::json!({ "name": download_5_2_2.as_str(), "notes": [format!("214 of 352{mb}MB")], "bold": 0, "window": 0 }),
            "{out}"
        );
        assert_eq!(
            out["noCount"],
            serde_json::json!({ "name": download_5_2_2.as_str(), "notes": [], "bold": 0, "window": 0 }),
            "a note was drawn with no count: {out}"
        );
        assert_eq!(out["noSize"]["notes"], serde_json::json!([format!("214{mb}MB")]), "{out}");
        assert_eq!(
            out["unpacking"],
            serde_json::json!({ "name": unpack_5_2_2.as_str(), "notes": ["41%"], "bold": 0, "window": 0 }),
            "{out}"
        );
    }

    const THE_RUN_ON_THE_CARD_HARNESS: &str = r#"function anElement(tag) {
  const names = new Set();
  const listeners = new Map();
  const element = {
    tag, children: [], parentNode: null, dataset: {}, hidden: false, text: "", type: "",
    style: { setProperty() {} },
    classList: {
      add: (...more) => more.forEach((one) => names.add(one)),
      remove: (...less) => less.forEach((one) => names.delete(one)),
      toggle: (one, on) => { if (on === undefined ? !names.has(one) : on) names.add(one); else names.delete(one); },
      contains: (one) => names.has(one),
    },
    set className(value) { names.clear(); String(value).split(" ").filter(Boolean).forEach((one) => names.add(one)); },
    get className() { return Array.from(names).join(" "); },
    set textContent(value) { element.children = []; element.text = String(value); },
    get textContent() { return element.text + element.children.map((one) => one.textContent || one.text || "").join(""); },
    get firstChild() { return element.children[0] || null; },
    get childElementCount() { return element.children.length; },
    appendChild(child) { child.parentNode = element; element.children.push(child); return child; },
    insertBefore(child) { return element.appendChild(child); },
    remove() { if (element.parentNode) element.parentNode.children = element.parentNode.children.filter((one) => one !== element); },
    setAttribute() {}, removeAttribute() {}, focus() {},
    addEventListener(kind, heard) { listeners.set(kind, heard); },
    press() { const heard = listeners.get("click"); if (heard) heard({}); },
    querySelector() { return null; }, querySelectorAll() { return []; }, closest() { return null; },
  };
  return element;
}
function every(element, found = []) {
  found.push(element);
  for (const child of element.children || []) every(child, found);
  return found;
}
const theTextOf = (one) => (one.text || "") + (one.children || []).map(theTextOf).join("");
globalThis.told = new Map();
globalThis.document = {
  createElement: anElement,
  createTextNode: (text) => ({ text, children: [], textContent: text }),
  addEventListener() {},
};
const running = THE_RUNNING;
const finished = THE_FINISHED;
const stopped = THE_STOPPED;
let theRun = null;
const readings = [];
const acts = [];
globalThis.window = {
  __TAURI__: {
    core: {
      invoke(command, args) {
        if (command === "the_set_up_tab") return new Promise((done) => readings.push(done));
        if (command === "the_setup_under_way") return Promise.resolve(theRun);
        if (command === "a_set_up_act_that_needs_a_connection") acts.push(args.act);
        return Promise.resolve(null);
      },
    },
  },
};
const settle = async () => { for (let at = 0; at < 4; at += 1) await new Promise((done) => setImmediate(done)); };
const tell = (said) => (globalThis.told.get("set-up:the-answer") || []).forEach((one) => one({ said }));
const theReadingLands = async (tab) => { readings.splice(0).forEach((done) => done(tab)); await settle(); };
await import("./set-up.js");
const host = anElement("div");
globalThis.tab.itIsDrawn(host);
globalThis.tab.itIsShowing("server-1");
await settle();
await theReadingLands(finished);
const [scroll] = host.children;
const [card, table] = scroll.children;
const modals = () => every(host).filter((one) => one.classList && one.classList.contains("modal")).length;
const covered = (section) => every(section).some((one) => one.classList && one.classList.contains("underway-pane"));
const buttons = (section) => every(section).filter((one) => one.tag === "button");
const stateOf = (section) => {
  const bold = every(section).find((one) => one.tag === "b");
  return bold ? bold.text : null;
};
const out = {};

// A run under way, told as the application tells it.
theRun = running;
tell({ server: "server-1", the_run_is_under_way: "yes" });
await settle();
out.running = {
  modals: modals(),
  state: stateOf(card),
  steps: every(card).filter((one) => one.tag === "li").length,
  buttons: buttons(card).map((one) => one.text),
};
buttons(card)[0].press();
out.afterCancel = acts.slice();

// The run ends: a Blender installed, and the table is read again.
theRun = null;
tell(THE_INSTALLED);
await settle();
out.afterTheEnding = {
  tableCovered: covered(table),
  cardCovered: covered(card),
  modals: modals(),
  lines: every(card).filter((one) => one.tag === "p" && one.parentNode && one.parentNode.classList.contains("sucard") && one.parentNode.children[1] === one && one.className === "sunote").length,
};
await theReadingLands(finished);
out.afterTheReading = {
  tableCovered: covered(table),
  listed: every(card).filter((one) => one.tag === "li").length,
};

// A stop's ending, and a setup that stopped part-way.
tell(THE_STOPS_ENDING);
await settle();
await theReadingLands(stopped);
const one = card.children[1];
out.stopped = {
  modals: modals(),
  state: stateOf(card),
  buttons: buttons(card).map((control) => control.text),
  theLine: one && one.children[1] ? one.children[1].text : null,
};
console.log(JSON.stringify(out));"#;

    /// The stand-ins the harness above loads `web/set-up.js` beside.
    const THE_SET_UP_STAND_INS: [(&str, &str); 11] = [
        // The real drawing of a value, which the page imports (step 10c,
        // dependency wave piece 10a).
        ("the-value-drawn.js", include_str!("../../web/the-value-drawn.js")),
        // **The server list's page keeps the connects that did not happen**,
        // and this stand-in answers from what a harness set and records what
        // was taken away (the Creator's note 14 on the test copy).
        (
            "servers.js",
            r#"export function theConnectionFailureOn(server) { return (globalThis.failedConnects || {})[server] || null; }
export function theConnectionFailureIsTakenAway(server) { (globalThis.takenAway ||= []).push(server); }"#,
        ),
        ("package.json", r#"{ "type": "module" }"#),
        (
            "event-route.js",
            r#"export function whenTold(kind, handler) {
  const held = globalThis.told.get(kind) || [];
  held.push(handler);
  globalThis.told.set(kind, held);
}"#,
        ),
        (
            "a-server.js",
            r#"export const FILES = "files";
export const SET_UP = "set-up";
export function aTabIsRegistered(tab) { globalThis.tab = tab; }
export function theCurrentTabIs() { return SET_UP; }"#,
        ),
        ("files.js", r#"export function theBrowserIsOpenedAt() {}"#),
        (
            "the-rail.js",
            r#"export const TRANSFERS = "transfers";
export function theCurrentPlaceIs() {}"#,
        ),
        (
            "the-failure-shape.js",
            r#"export const EXPANDED = "expanded";
export function theFailureShape() { return document.createElement("div"); }"#,
        ),
        (
            "the-machines-own-words.js",
            r#"export function theWordsPanel() { return document.createElement("div"); }
export function theNewestWordsStayInView() {}"#,
        ),
        (
            "the-underway-shape.js",
            r#"export function theMarkOnAControl(control) { return control; }
export function theMarkOnARow() { return document.createElement("i"); }
export function theMarkOverAPane() { return document.createElement("div"); }"#,
        ),
        (
            "harness.js",
            r#"import { readFileSync } from "node:fs";
function anElement(tag) {
  const names = new Set();
  const listeners = new Map();
  const element = {
    tag, children: [], parentNode: null, dataset: {}, hidden: false, text: "", type: "",
    style: { setProperty() {} },
    classList: {
      add: (...more) => more.forEach((one) => names.add(one)),
      remove: (...less) => less.forEach((one) => names.delete(one)),
      toggle: (one, on) => { if (on === undefined ? !names.has(one) : on) names.add(one); else names.delete(one); },
      contains: (one) => names.has(one),
    },
    set className(value) { names.clear(); String(value).split(" ").filter(Boolean).forEach((one) => names.add(one)); },
    get className() { return Array.from(names).join(" "); },
    set textContent(value) { element.children = []; element.text = String(value); },
    get textContent() { return element.text + element.children.map((one) => one.textContent || one.text || "").join(""); },
    get firstChild() { return element.children[0] || null; },
    get childElementCount() { return element.children.length; },
    appendChild(child) { child.parentNode = element; element.children.push(child); return child; },
    insertBefore(child) { return element.appendChild(child); },
    remove() { if (element.parentNode) element.parentNode.children = element.parentNode.children.filter((one) => one !== element); },
    setAttribute() {}, removeAttribute() {}, focus() {},
    addEventListener(kind, heard) { listeners.set(kind, heard); },
    press() { const heard = listeners.get("click"); if (heard) heard({}); },
    querySelector() { return null; }, querySelectorAll() { return []; }, closest() { return null; },
  };
  return element;
}
function every(element, found = []) {
  found.push(element);
  for (const child of element.children || []) every(child, found);
  return found;
}
globalThis.told = new Map();
globalThis.document = {
  createElement: anElement,
  createTextNode: (text) => ({ text, children: [], textContent: text }),
  addEventListener() {},
};
const theWarning = JSON.parse(readFileSync("./the-warning.json", "utf8"));
const asked = [];
const acts = [];
globalThis.window = {
  __TAURI__: {
    core: {
      invoke(command, args) {
        if (command === "the_set_up_tab") return new Promise(() => {});
        if (command === "the_setup_under_way") return Promise.resolve(null);
        if (command === "the_work_warning") { asked.push(command); return Promise.resolve(theWarning); }
        if (command === "a_set_up_act_that_needs_a_connection") acts.push({ act: args.act, server: args.server });
        return Promise.resolve(null);
      },
    },
  },
};
const settle = () => new Promise((done) => setImmediate(done));
const tell = (kind, said) => (globalThis.told.get(kind) || []).forEach((one) => one({ kind, said }));
await import("./set-up.js");
const host = anElement("div");
globalThis.tab.itIsDrawn(host);
globalThis.tab.itIsShowing("server-1");
await settle();
tell("set-up:the-answer", { server: "server-1", the_work_warning_is_owed: "yes" });
await settle();
await settle();
const drawn = every(host);
const title = drawn.find((one) => one.tag === "h3");
// A job's name is drawn inside an isolate of its own (step 10c, dependency
// wave piece 10a), so a line's text is its own and its children's, in order.
const theTextOf = (one) => (one.text || "") + (one.children || []).map(theTextOf).join("");
const lines = drawn.filter((one) => one.tag === "b").map(theTextOf);
const buttons = drawn.filter((one) => one.tag === "button");
buttons.find((one) => one.text === theWarning.not_now).press();
const afterNotNow = acts.slice();
buttons.find((one) => one.text === theWarning.go_ahead).press();
console.log(JSON.stringify({
  asked, title: title ? title.text : null, lines, afterNotNow, afterGoingAhead: acts,
}));"#,
        ),
    ];
}
