//! **All servers — the list where servers are compared, and everything the
//! screen opens.** The page half is `web/servers.js`.
//!
//! **This file is the surface and not the deciding.** Every judgement below is
//! `Servers`', in `App/Servers/`, and this file translates: it reads the local
//! record, asks that module what it means, composes the words, and hands them
//! over. **Nothing here decides what a server is or what a warning says.**
//! `Servers` W43 says so in as many words — *"No screen is drawn here"* — and
//! names this document and Design as the owners of what a user meets.
//!
//! ## What this piece draws, cited the way the runner ruled on 2026-09-12
//!
//! **The Design document and the heading**, and **the mockup state names**.
//!
//! - `Design/Servers § A row`, `§ Actions on a row`, `§ Dropping a setup the
//!   server never ran`, `§ Connected and needing a new key at once`,
//!   `§ No servers yet`, `§ Adding a server`, `§ The Edit dialog`,
//!   `§ Removing a server`, `§ When stop and clear does not work`,
//!   `§ Keyboard shortcuts`, `§ A changed identity`.
//! - The mockup states, which are the `<option>` values of
//!   `Preparation/Design/Mockup/Smoozy.html`'s own state list and are what this
//!   half is counted by: `sv`, `sv-empty`, `sv-add`, `sv-edit`, `sv-blank`,
//!   `sv-port`, `sv-dupname`, `sv-sc`, `sv-scoff`, `sv-menus`, `sv-remove`,
//!   `sv-rmnone`, `sv-ident`, `sv-keyed`, `sv-both`, `sv-nostore`, `sv-notool`,
//!   `sv-upd`, `sv-updnow`, `sv-rename`, `sv-paste`, `sv-fallbad`, `sv-addkey`.
//!   **Twenty-three, and the drawing has no twenty-fourth in the `sv-` set.**
//!   *(Twenty-two until 2026-10-16: `sv-addkey`, the Add dialog with its key
//!   list open — Smoozy's own dropdown of the usable keys, each by name with
//!   its fingerprint's start, and *Open Keys…* last — drawn by the plan and
//!   drawing pass for the Creator's eighth notes, note 3, and built by piece
//!   E3: [`the_keys_a_dialog_offers`], [`the_keys_a_dialog_offers_are_checked`]
//!   and [`A_DIALOG_KEYS_VERDICT`].)*
//!   *(Twenty-one until 2026-10-04: `sv-fallbad`, the Edit dialog refusing a
//!   fallback folder that does not start with `/`, drawn by the drawing pass
//!   for the chosen fallback folder, and built by the Creator's second notes,
//!   piece 4: [`TheEditDialog`]'s field and [`TheAnswer::TheFallbackFolderWillNotDo`].)*
//!   *(Twenty until 2026-10-04: `sv-paste`, the provider's ssh line pasted into
//!   Host, drawn by the drawing pass for the Creator's second notes on the test
//!   copy, and built by that map's piece 15: [`the_pasted_line_is_read`] and
//!   [`ThePastedLine`].)*
//!   *(Nineteen until 2026-10-02: `sv-rename`, a row renamed in place — the
//!   Creator's note 1 on 0.1.8.)*
//!   *(Sixteen until 2026-09-18: `sv-nostore` — `Keys` S10's mark with
//!   `Security and licence` S34's sentence — was drawn for step 10b's first
//!   audit round, its second upstream item. Seventeen until 2026-09-28:
//!   `sv-upd` and `sv-updnow` — `Desktop app and delivery` V13's row lines and
//!   V15's box — were drawn by the plan pass for servers following the app's
//!   release, and built by step 10c's transfer wave piece 9, which also drew
//!   `sv-upd`'s sixth row.)*
//!
//! ## The one plan clause this piece keeps that the module's own crate does not
//!
//! **`Servers` W39** — the edit warning checked against the three escalations
//! and declined. It is kept at [`the_edit_warning`], which composes the warning
//! from the two sources `Servers` W31 names and **adds no switch-off**
//! (`Servers` W36). Every other `Servers` clause this file leans on is already
//! kept inside `App/Servers/`, and this file cites rather than re-decides.
//!
//! ## What can actually happen today, said plainly
//!
//! **This application opens connections to servers.** `Servers`' acting half
//! stands on five seams (`App/Servers/src/acting.rs`) and **all five have real
//! ends now**: the transport and the clock are that crate's own
//! (`App/Servers/src/wiring.rs`), the credential is `Keys`'
//! (`src/keys/for_a_connection.rs`), the machine's installation identity is
//! `src/servers/installation.rs`, and what the module publishes is
//! `src/servers/published.rs`. **Where a period runs is
//! `src/servers/a_period.rs`**, which also holds every period this run has.
//!
//! So of everything this screen offers:
//!
//! - **The list, the footer, the empty state, the key mark, the hardware
//!   readings and the SSH-tools statement** are composed from the local record
//!   and from `Keys`' local readings, and all of them run whole.
//! - **The add's save** runs whole. It is one durable write. **Where a setup
//!   option is ticked, the add connects and sets up** (`Servers` O2, since step
//!   10c piece 9), and the answer says nothing, as a plain add's does. *(Until
//!   step 10c's eleventh-pass fix piece 6 it went on saying the part was not
//!   finished, which piece 9 had made false.)*
//! - **The edit's save and its put-back** both run whole. The save ends the
//!   period (`Servers` E1) and the put-back opens a new one (`Servers` E2).
//! - **A removal that reaches no machine** runs whole — `Servers` D19, and it
//!   is one of the two places that clause's closed list does real work. The
//!   period ends at the local forget, which is `Servers` S10 item 6.
//! - **Connect and disconnect** run whole, and so does **accepting a changed
//!   identity**.
//! - **Stop and clear runs whole** — step 10c's S2. *Stop and clear first* and
//!   the `sv-sc` box's *Retry* run `Servers` W25's three units in order
//!   ([`stop_and_clear_first`]): `Queue & History` L13's pair through the one
//!   sender at `src/servers/stop_and_clear.rs`, Telegram's role off (`Servers`
//!   W24), and the local forget only on *cleared and stopped*. **Anything else
//!   raises the `sv-sc` box** with `Servers` W21's ending, on the list's next
//!   drawing, and *Remove anyway* forgets the server sending
//!   nothing.
//! - **No act this screen draws is owed.** *(Four until step 10c's piece 4,
//!   five until its S2.)* **Connect and set up** is the user's ordinary
//!   connect (`Servers` B6): the period takes the remembered setup at
//!   `Servers` B2's instant and hands it to `Server Setup` at `Servers` Q1
//!   step 8 (`src/servers/a_period.rs`). **Dropping a setup** is `Servers` B5's
//!   second clearing, one durable write ([`the_remembered_setup_is_dropped`]),
//!   followed by a plain connect, which is what the item's own words say.
//!   **The two ways out of a key that can no longer be used** are the page's
//!   own and reach no machine: *Change key…* opens Keys to pick
//!   one and saves it through the edit's own save; *Fix in Keys…*
//!   opens Keys at the key. Either way
//!   the mark clears at the list's next drawing (`Servers` P15).
//! - **`Desktop app and delivery` V15's *Stop jobs and update* is built.** It
//!   sends `Queue & History` L13's stop and clear through the removal's own
//!   sender ([`the_update_now`], [`TheRemovalsOwnPath`]), and the update
//!   follows at V12's next queue reading. *(Until step 10c's S2 that path sent
//!   nothing and the act answered that it was not built.)* **V13's six row lines are drawn** ([`the_update_line`]),
//!   each in `Design/Servers § A row`'s words and none of them *Smoozy isn't
//!   responding*. Step 10c's transfer wave piece 9.
//!
//! **Nothing is stubbed and nothing pretends.** A control whose act is owed
//! draws exactly as Design draws it and answers with the one thing this piece
//! can honestly say, which is that the act is not built — see
//! [`TheAnswer::TheActIsNotBuiltYet`].
//!
//! **And one thing a failed connect says that the others do not.** `Servers`
//! Q13 outcome 2 promises that a machine naming something other than Linux is
//! said plainly to be not a Linux machine, and this screen says it —
//! [`TheAnswer::ItIsNotALinuxMachine`]. **The reason an attempt stopped leaves
//! `App/Servers/` where the ending is the opening attempt's own**: its driving
//! half answers with the step that stopped the attempt, `src/servers/a_period.rs`
//! reads that into what the driving left, and `the_period_ended` below turns it
//! into the answer. **The other eight reasons, `Servers` Q13 outcome 3 among
//! them, still answer with the sentence that claims nothing about why** —
//! [`TheAnswer::TheConnectionWasNotMade`] — because `Preparation/Specs/Whole
//! app.md` forbids the claim whether or not Smoozy knows which of them it met.
//!
//! ## How a clause marker is written in this crate
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. **Every marker of
//! `Servers` below is written out with that document at the site**, because
//! this crate's plan defines a C-series, an E-series, an H-series, a K-series,
//! a P-series, a Q-series, an S-series and a W-series of its own, and this file
//! cites `Servers`' clauses in every one of those series, so a bare one would
//! resolve to a real and wrong clause.

use std::sync::Mutex;

use tauri::{AppHandle, Manager, Runtime, State};

use smoozy_keys::usable;
use smoozy_servers::condition::{self, Menu, Presentation, WhatIsTrueOfTheServer};
use smoozy_servers::edits::WhatChanged;
use smoozy_servers::hardware::{
    hardware_state, Cards, HardwareReading, HardwareState, WhichConnectionHoldsIt, WhyUnreadable,
};
use smoozy_servers::identity::{ConnectionIdentity, Host, KeyReference, Port, Username};
use smoozy_servers::store::ServerRow;
use smoozy_servers::work::{
    removal_warning, Offers, Warns, WhatTheLastKnownRecordHolds,
};
use smoozy_security_and_licence::secret_store::SecretStore;
use smoozy_state::identities::{Identity, IdentityKind, InstallationCheck};

use crate::events::{Event, Said, Subject as About, TheOneQueue};
use crate::servers::a_period::{self, HowItWent, ThePeriodsThisRun, WhatTheDrivingLeft, WhoAsked};
use crate::presentation::the_repairing_server::WhatTheColumnSays;
use crate::surface::the_queue::{WhatAServerIsDoing, WhatTheServersAreDoing};
use crate::record::{key_rows, server_rows};
use crate::start::Started;
use crate::typography::{Prose, Verbatim};

/// **The one kind this piece posts**, declared here beside the service that
/// posts it and copied into `web/servers.js`. Nothing holds the two together,
/// so copy carefully.
///
/// **One kind, because every act on this screen answers one question**: what
/// the user asked the server list to do. A second kind would be a second
/// drawing path for one surface. That is piece 1's own shape and its reasoning
/// carries here unchanged.
pub const THE_ANSWER: &str = "servers:the-answer";

/// **What a server is doing moved** — a queue reading was taken, or an ending
/// was learned, and the work column of at least one row now reads differently.
/// **It carries nothing**: the list and a server's own page ask for themselves
/// again, which is how every other change on those screens reaches them.
/// Posted by [`the_work_column_is_told_again`], and only on a change, so a
/// P10d pass that moved nothing asks for nothing. Added 2026-09-28, step 10c's
/// ninth-pass fix piece 2. **Also posted when whether a job in the queue needs
/// your answer moves**, which the rail's Queue dot draws (2026-10-31, the
/// twenty-fourth notes, note 2).
pub const THE_WORK_MOVED: &str = "servers:the-work-moved";

/// **One key's verdict on a server dialog's key list, arriving behind the
/// list** — `Keys` U10a, for the caller `Keys` U10 gained on 2026-10-12: *"a server
/// dialog's key list"*. Posted by [`the_keys_a_dialog_offers_are_checked`], one
/// per key, carrying the ask it answers so the page drops an answer to an ask
/// it has moved past. Added 2026-10-16, the Creator's eighth notes, piece E3.
pub const A_DIALOG_KEYS_VERDICT: &str = "servers:a-dialog-keys-verdict";

/// **`Servers` W20's suppression identity, declared there and read here.**
///
/// *"one row in the one suppression table, carrying this document and this
/// warning's identity, which is declared here and is stable for the life of the
/// application: `servers.disconnected-removal`"*. **The prefix is not
/// decoration**: the one table has one column, so the module lives inside the
/// identity string.
pub const THE_DISCONNECTED_REMOVAL_WARNING: &str = "servers.disconnected-removal";

/// The module name the suppression table and the declaration register know this
/// module by.
pub const THIS_MODULE: &str = "Servers";

/// **The durable subject a failure on one of this screen's rows is about**, as
/// `crate::record::failure_records::DurableSubject::kind_as_text` writes it.
pub const THE_SUBJECT_A_ROW_IS_ABOUT: &str = "server row";

// ════════════════════════════════════════════════════════════════════════════
// The words
// ════════════════════════════════════════════════════════════════════════════
//
// **Every word the user reads on this screen is here, each as its own string,
// and each goes through the typography point on its way out** (A6, X1, X10).
// **No value is ever joined into one** — X7, and the folder head of
// `src/presentation/mod.rs` names that join as the trap of this whole step.
//
// **Where a sentence names a server, a job or a figure, the sentence stops
// where the value begins** and the two travel side by side. `Design/Servers`
// draws *"Another server is already called Overnight box."*; what leaves here
// is the words, then the name, and the page sets them beside each other.

/// **Crate-visible because one sentence here is two screens'** — step 10a, the
/// screens round's loose ends, L6. Transfers says *Could not read your
/// servers.* for the same fact, and Design's *one fact gets one wording* makes
/// that one constant rather than two that agree today.
pub(crate) mod words {
    // `Design/Servers § Where it sits` and the mockup's `.svhead`.
    pub const THE_SCREEN: &str = "Servers";
    pub const ADD_A_SERVER: &str = "Add a server";

    // `Design/Servers § A row` — *"On a computer without the SSH tools, one line
    // above the list says why no key is checked. It is the Keys screen's
    // statement, word for word"*, and `Keys` U6a is the rule underneath it.
    // **It is the Keys screen's two strings and not a copy of them** (X10:
    // two sentences, two pieces) — step 10a round 1, fix-here item 30.
    pub const NO_TOOLS_LEAD: &str = crate::surface::keys::THE_TOOLS_ARE_NOT_INSTALLED;
    pub const NO_TOOLS_UNTIL: &str = crate::surface::keys::UNTIL_THEY_ARE;

    // `Design/Servers § A row`, the work column. **The four words are
    // `Desktop app and delivery` M29's own vocabulary** and are taken from
    // `crate::presentation::the_repairing_server::WhatTheColumnSays` rather
    // than written again here — see `the_work_column`.
    pub const RESTARTING: &str = "Restarting";
    // `Design/Servers § A row`: *"While a setup runs on a server, the column
    // says Setting up."* — the Creator's note 3 on 0.1.7. **The Set up card's
    // own state line for a running setup is the same word**, so it has one
    // home here (`src/surface/set_up.rs` takes it).
    pub const SETTING_UP: &str = "Setting up";
    // `Design/Servers § A row`: *"Where a server is connected and Smoozy's
    // service on it does not answer, the row's extra line says Smoozy isn't
    // responding."* — `Desktop app and delivery` V1's supervisor-unavailable
    // condition, whose words were left to Design. **The same words are the Set
    // up card's state line** (`Design/Server Setup § This server`) and the
    // condition's own sentence wherever the failure shape draws it, so they
    // have one home. Added 2026-09-27, step 10c's eighth-pass fix piece 6.
    pub const SMOOZY_ISNT_RESPONDING: &str = "Smoozy isn't responding.";

    // `Design/Servers § A row`: **Smoozy on a server older or newer than the
    // app** — `Desktop app and delivery` V13's conditions, whose words were
    // left to Design, and the drawing's `sv-upd` and `sv-updnow`. **None of
    // them is the not-responding line** (`Server protocol and transfers` P3b).
    // Added 2026-09-28, step 10c's transfer wave piece 9.
    pub const THE_UPDATE_WAITS: &str = "Smoozy on this server updates when its jobs finish.";
    pub const UPDATE_NOW: &str = "Update now…";
    pub const UPDATING: &str = "Updating Smoozy on this server…";
    pub const COULD_NOT_BE_UPDATED: &str = "Smoozy on this server could not be updated.";
    pub const TOO_OLD_ITS_JOBS: &str = "Smoozy on this server is out of date. Reconnect after its jobs finish to update it.";
    // **The sixth line, the build runner's ruling of 2026-09-28**: where
    // `Server protocol and transfers` P3b's reading of the records cannot
    // answer, V11 holds the server as one with work on it, and the line above
    // would say jobs stand when that is not known (Rule 8).
    pub const TOO_OLD_CANNOT_TELL: &str = "Out of date. Not updated: could not read its job list.";
    // **V14's words have one home**, where the Set up refusal says them.
    pub const TOO_NEW: &str = crate::servers::published::THE_MACHINE_IS_NEWER;
    // The `sv-updnow` box: *"Update Smoozy on RunPod 4090 now?"*, then the
    // removal box's own statement of what *stop and clear* does (`Servers`
    // W19a, unchanged — V15).
    pub const UPDATE_SMOOZY_ON: &str = "Update Smoozy on";
    pub const NOW: &str = "now?";
    pub const UPDATING_NOW_STOPS_THEM: &str =
        "Updating now stops its jobs first.";
    pub const STOP_JOBS_AND_UPDATE: &str = "Stop jobs and update";

    pub const WAITING: &str = "waiting";
    // `Design/Servers § A row`, added 2026-09-28: *"Waiting, 2 jobs"* — a
    // server whose reading holds jobs waiting and none rendering.
    pub const WAITING_LEAD: &str = "Waiting";
    pub const A_JOB: &str = "job";
    pub const JOBS: &str = "jobs";
    pub const TRYING_AGAIN_IN: &str = "retrying in";
    pub const WAITING_SO_FAR: &str = "waiting so far";
    pub const SETUP_STOPPED_PART_WAY: &str = "Setup stopped part-way";

    // `Design/Servers § A row`, the hardware column. **Rule 9: the industry's
    // words.** *"the GPUs and their VRAM, then the server's RAM and its free
    // disk — those words, since a screen uses the industry's."*
    pub const RAM: &str = "RAM";
    pub const FREE_DISK: &str = "free disk";
    /// The drawing's own word where more than one card is the same card.
    pub const EACH: &str = "each";
    pub const LAST_SEEN: &str = "last seen";
    // **Says what is true and names no cause** — a host or port edit, or a
    // connection that dropped before the read, leaves nothing known as well as
    // never having connected. Step 10a round 1, fix-here item 29; the words
    // are that round's call and Design's owed words.
    pub const NOTHING_IS_KNOWN_YET: &str = "Hardware not read yet";
    pub const CARDS_UNREADABLE: &str = "Could not read the GPUs";
    pub const NO_CARD: &str = "No GPU";
    pub const MEMORY_UNKNOWN: &str = "Could not read RAM";

    // `Design/Servers § Connected and needing a new key at once` — the line
    // *"says what it costs rather than what happened"*.
    pub const THE_KEY_IS_GONE: &str =
        "Key missing. Choose a new key to reconnect.";

    // `Design/Keys § On a computer with nowhere to keep a passphrase` — **`Keys`
    // S10's two marks**, the first where the computer has nowhere to keep one
    // and the second otherwise. **A permanent condition is never worded as a
    // passing one**, which is S10's whole bound. Mockup state `sv-nostore`.
    pub const THE_PASSPHRASE_CANNOT_BE_KEPT: &str =
        "No automatic reconnect: this computer cannot save passphrases.";
    pub const THE_PASSPHRASE_IS_NOT_HELD: &str =
        "No automatic reconnect until you connect: passphrase not saved.";

    // The connect control. `Design/Servers § Actions on a row` and
    // `Design/Whole app § Where things live`.
    pub const CONNECTED: &str = "Connected";
    pub const CONNECT: &str = "Connect";
    pub const CONNECT_AND_SET_UP: &str = "Connect and set up";
    pub const MORE: &str = "More";

    // The footer. `Design/Servers § A row`: *"The list carries a footer, with
    // how many servers there are and how many are connected."*
    pub const SERVERS: &str = "servers";
    pub const ONE_SERVER: &str = "server";
    pub const CONNECTED_COUNT: &str = "connected";

    // `Design/Servers § No servers yet`, and the mockup's `#svempty`. **The
    // order is the part that matters** — the public half goes to the provider
    // before the server is rented.
    pub const NO_SERVERS_YET: &str = "No servers yet";
    pub const HOW_IT_GOES: &str = "You rent the server yourself, from any provider:";
    pub const STEP_1_LEAD: &str = "Create an SSH key";
    pub const STEP_1_REST: &str = ", or use an existing one.";
    pub const OPEN_KEYS: &str = "Open Keys";
    pub const STEP_2_LEAD: &str = "Copy its public key.";
    pub const STEP_3_LEAD: &str = "Add the public key at your provider";
    pub const STEP_3_REST: &str = ", before you rent the server.";
    pub const STEP_4_LEAD: &str = "Rent an Ubuntu or Debian Linux server.";
    pub const STEP_4_REST: &str = " Others are not supported.";
    pub const STEP_5_LEAD: &str = "Add the server here";
    pub const STEP_5_REST: &str = " with its host, port and username.";

    // `Design/Servers § Adding a server`, and the mockup's `#svadd`.
    pub const HOST: &str = "Host";
    pub const PORT: &str = "Port";
    pub const USERNAME: &str = "Username";
    pub const KEY: &str = "Key";
    pub const NAME: &str = "Name";
    pub const COLOUR: &str = "Colour";
    pub const OPTIONAL: &str = "optional";
    pub const NEW_KEY: &str = "New key…";
    pub const THE_GUIDANCE_AGAIN: &str = "How to rent a server and add your key";
    pub const THE_SWATCH_NOTE: &str =
        "The first unused colour.";
    pub const GET_READY: &str = "Get the server ready to render";
    pub const INSTALL_BLENDER: &str = "Install the latest stable Blender";
    pub const TICKING_EITHER: &str = "Either option connects immediately.";
    pub const CANCEL: &str = "Cancel";
    pub const ADD: &str = "Add";
    pub const ADD_AND_SET_UP: &str = "Add and set up";
    // **The key control's own words before a key is picked**, the Keys screen's
    // own head control — the dialog opens that screen to pick one (`Keys` U11).
    pub const CHOOSE_A_KEY: &str = "Choose a key…";
    // **The key dropdown's own words** — `Design/Keys § Opened from a server
    // dialog` and the drawing's `sv-addkey`, the Creator's eighth notes, note 3.
    // Its last row opens the Keys screen to pick; a fingerprint's start runs on
    // past the eight characters shown; the Edit dialog's own unusable key
    // carries the gone mark (`Design/Servers § The Edit dialog`), drawn beside
    // the key's name and never joined to it. *Checking…* is the Keys screen's
    // own word, `crate::surface::keys::CHECKING`.
    pub const OPEN_KEYS_ITEM: &str = "Open Keys…";
    pub const THE_FINGERPRINT_RUNS_ON: &str = "…";
    pub const THE_GONE_MARK: &str = " — missing";
    // **The provider's connection line, pasted into Host** — `Design/Servers §
    // Adding a server` and the drawing's `sv-blank` (the placeholder) and
    // `sv-paste` (the line under Host, in the form's grey). The Creator's
    // second notes, piece 15; `Servers` §13.3.
    pub const HOST_OR_PASTE: &str = "Host, or paste the ssh line";
    pub const NO_USERNAME_IN_THAT_LINE: &str = "No username in that line.";
    pub const NO_HOST_IN_THAT_LINE: &str = "No host in that line.";
    // **The one form of the line that cannot carry files**, from the provider
    // reading of 2026-10-07 (`Servers` PL3). *SSH over exposed TCP* is the
    // name Runpod's own console gives its direct line.
    pub const RUNPODS_PROXIED_LINE: &str =
        "This command can't transfer files. Paste the one under SSH over exposed TCP.";

    // The four refusals. **The host one is Design's own example**; the other
    // three are ruled here in the same shape, each naming what is missing
    // rather than that something is — see this file's *rulings* section.
    pub const HOST_IS_NEEDED: &str = "Host is required.";
    pub const USERNAME_IS_NEEDED: &str = "Username is required.";
    pub const A_KEY_IS_NEEDED: &str = "Key is required.";
    pub const THE_PORT_RANGE: &str = "Port must be 1–65535.";

    // The two non-blocking statements. **They are checks on different things
    // and must not read alike** — `Design/Servers § Adding a server`.
    pub const MAY_ALREADY_HAVE_LEAD: &str = "You may already have this server.";
    pub const MAY_ALREADY_HAVE_REST: &str =
        "has the same host and port. It may be a different server.";
    pub const ALREADY_CALLED_LEAD: &str = "Another server is already called";
    pub const ALREADY_CALLED_REST: &str = "In the sidebar, only the badge tells them apart.";

    // `Design/Servers § The Edit dialog`, and the mockup's `#svedit`.
    pub const EDIT: &str = "Edit";
    pub const SAVE: &str = "Save";
    pub const THE_USERNAME_NOTE: &str = "Changing the username clears this server's saved password.";
    pub const THE_GONE_KEY_NOTE: &str = "Key missing; kept selected for reference.";
    pub const A_PASSWORD_IS_REMEMBERED: &str = "Password saved.";
    pub const FORGET_IT: &str = "Clear";
    // **The server's fallback folder, on Edit only** — `Design/Servers § The
    // Edit dialog`, the drawing's `sv-edit` and `sv-fallbad` (`Servers` D3,
    // `Job files and outputs` P23). The Creator's second notes, piece 4.
    pub const FALLBACK_FOLDER: &str = "Fallback folder";
    pub const SMOOZYS_FOLDER: &str = "Smoozy’s folder";
    pub const WHERE_JOBS_GO: &str = "Where jobs go when their output folder can’t be used here.";
    pub const ENTER_A_FULL_PATH: &str = "Enter a full path, starting with /.";

    // `Servers` W34: the warning names **jobs Smoozy has not seen end**, never
    // jobs queued or running, and *"the wording must carry that"*.
    pub const POINTS_AT_A_DIFFERENT_SERVER: &str = "This points at a different server.";
    pub const HAVE_NOT_ENDED_ONE: &str = "job on the old server has not ended.";
    pub const HAVE_NOT_ENDED_MANY: &str = "jobs on the old server have not ended.";
    pub const LEFT_WHERE_THEY_ARE: &str = "They stay there.";
    // `Design/Servers § The Edit dialog`'s username case (the Gemini catch-up
    // of step 10a, Area 5, U-1 and F-1): the jobs stay on this machine, and
    // Smoozy can no longer reach them.
    pub const LOGS_IN_AS_A_DIFFERENT_USER: &str = "This logs in as a different user.";
    pub const HAVE_NOT_ENDED_HERE_ONE: &str = "job on this server has not ended.";
    pub const HAVE_NOT_ENDED_HERE_MANY: &str = "jobs on this server have not ended.";
    pub const CANNOT_REACH_THEM_AFTER: &str = "They will be unreachable.";

    // `Design/Servers § Removing a server`, and the mockup's `#svremove`.
    pub const REMOVE: &str = "Remove";
    pub const SMOOZY_FORGETS_IT: &str =
        "Removes the server from Smoozy. Nothing on the server changes.";
    pub const LEAVE_IT_RUNNING: &str = "Leave it running";
    pub const LEAVE_IT_RUNNING_SAID: &str =
        "The render and queue keep running, untracked.";
    pub const STOP_AND_CLEAR: &str = "Stop and clear first";
    pub const STOP_AND_CLEAR_SAID_ONE: &str = "Clears the waiting job, then stops the render. The current frame may not be saved; rendered frames stay on the server.";
    pub const STOP_AND_CLEAR_SAID_MANY: &str = "waiting jobs, then stops the render. The current frame may not be saved; rendered frames stay on the server.";
    pub const STOP_AND_CLEAR_SAID_LEAD: &str = "Clears the";
    pub const RENDERING_AND_WAITING: &str = "on this server.";
    pub const ONE_JOB_IS_RENDERING: &str = "job is rendering and";
    pub const JOBS_ARE_RENDERING: &str = "jobs are rendering and";
    pub const ONE_IS_WAITING: &str = "is waiting";
    pub const MANY_ARE_WAITING: &str = "are waiting";
    // **Where one of the two counts is nought, its half is left out** rather
    // than said as *0 are waiting*, and *Stop and clear first* says only the
    // half it does. Step 10c's eleventh-pass fix piece 6, under Rule 9's
    // *Short, too*: until then the box never had a reading to count, so no
    // count was ever nought.
    pub const ONE_JOB_IS_RENDERING_ALONE: &str = "job is rendering";
    pub const JOBS_ARE_RENDERING_ALONE: &str = "jobs are rendering";
    pub const ONE_JOB_IS_WAITING: &str = "job is waiting";
    pub const JOBS_ARE_WAITING: &str = "jobs are waiting";
    pub const STOP_AND_CLEAR_SAID_NONE_WAITING: &str =
        "Stops the render. The current frame may not be saved; rendered frames stay on the server.";
    pub const STOP_AND_CLEAR_SAID_ONE_NONE_RENDERING: &str = "Clears the waiting job.";
    pub const STOP_AND_CLEAR_SAID_MANY_NONE_RENDERING: &str = "waiting jobs.";
    // **`Servers` W19 row three: the disconnected warning *"warns from what
    // Smoozy last knew, saying so."*** `Design/Servers § Removing a server`,
    // as carried from step 10a round 1's first fix-upstream item. Words and
    // values, never one string (X7).
    pub const WHEN_SMOOZY_WAS_LAST_CONNECTED: &str = "When last connected,";
    pub const ONE_JOB_WAS_RENDERING: &str = "job was rendering and";
    pub const JOBS_WERE_RENDERING: &str = "jobs were rendering and";
    pub const ONE_WAS_WAITING: &str = "was waiting.";
    pub const MANY_WERE_WAITING: &str = "were waiting.";
    pub const ONE_JOB_WAS_RENDERING_ALONE: &str = "job was rendering.";
    pub const JOBS_WERE_RENDERING_ALONE: &str = "jobs were rendering.";
    pub const ONE_JOB_WAS_WAITING: &str = "job was waiting.";
    pub const JOBS_WERE_WAITING: &str = "jobs were waiting.";
    pub const LEAVE_IT_RUNNING_SAID_LAST_KNOWN: &str =
        "Whatever is still running keeps running, untracked.";
    pub const DONT_WARN_AGAIN: &str = "Don't warn me again";
    pub const WHAT_IT_COVERS: &str =
        "Applies to all disconnected servers. Turn back on in Settings.";

    // `Design/Servers § When stop and clear does not work`, and the mockup's
    // `#svsc`. **Three endings and not two** — `Servers` W21, whose three rows
    // are `Queue & History` L17's verbatim.
    pub const STOPPING_WORK_ON: &str = "Stopping work on";
    // **The lead names no cause** — `Servers` W21's last paragraph, the stop
    // and clear plan pass's SC-2: a program that refuses the pair by name was
    // reached, so *could not reach the server* would be false of it. It said
    // exactly that until step 10c's S2, which is the piece that first sends
    // the pair to a program that may refuse it.
    pub const COULD_NOT_BEGIN: &str = "Could not stop and clear the jobs on this server.";
    pub const COULD_NOT_BEGIN_REST: &str = "Nothing changed.";
    pub const CLEAR_REFUSED: &str = "Could not clear the waiting jobs.";
    pub const CLEAR_REFUSED_REST: &str = "The render was not stopped.";
    pub const HALF_WORKED: &str = "The waiting jobs were cleared.";
    pub const HALF_WORKED_REST: &str = "The render is still running.";
    pub const TRY_AGAIN: &str = "Retry";
    pub const REMOVE_ANYWAY: &str = "Remove anyway";

    // `Design/Servers § A changed identity`, and the mockup's `#svident`.
    // **No fingerprint is shown, anywhere** — `Servers` D9 and the spec's own
    // policy agreeing.
    pub const IDENTITY_CHANGED: &str = "— host key changed";
    pub const IDENTITY_CHANGED_SAID: &str = "The server may have been rebuilt, or another machine may be at its address.";
    pub const ACCEPTING_IS_A_DECISION: &str =
        "Accepting trusts the server without verifying it.";
    pub const DO_NOT_CONNECT: &str = "Do not connect";
    pub const ACCEPT_AND_CONNECT: &str = "Accept and connect";

    // `Design/Servers § Actions on a row`, the five menus. **Every menu is the
    // same five slots in the same order** — Creator-stated (2026-08-13).
    pub const DISCONNECT: &str = "Disconnect";
    pub const BROWSE_FILES: &str = "Browse files";
    pub const SET_UP: &str = "Set up";
    pub const A_DIFFERENT_KEY: &str = "Change key…";
    pub const FIX_THE_KEY: &str = "Fix in Keys…";
    pub const DROP_THE_SETUP: &str = "Connect without setup";
    // *Rename…* edits the name in place on the row — `Design/Servers §
    // Actions on a row`, the Creator's note 1 on 0.1.8.
    pub const RENAME_ITEM: &str = "Rename…";
    pub const EDIT_ITEM: &str = "Edit…";
    pub const COPY_ADDRESS: &str = "Copy address";

    // `Servers` W40: *"the rented machine goes on running, and goes on being
    // billed by whoever it was rented from, until somebody destroys the rental
    // by hand"*. **Made at the add's save, and again wherever `Servers` W41's figure is a
    // stated absence** (`Servers` W40a). It is not a warning, not a condition and not a
    // gate.
    pub const RUNS_AND_BILLS: &str = "Your provider bills you for this server until you destroy it there.";
    pub const RUNNING_FOR: &str = "running for";

    // The one thing this screen can honestly say where an act is not built.
    // **It is not a failure record and it is not a stand-in for one**: it says
    // what is true.
    //
    // **It once explained itself — *Smoozy cannot reach servers yet* — and step
    // 10b piece 5 made that false** by building connect, disconnect and
    // accepting a changed identity, so a user looking at a connected server was
    // told servers could not be reached. The explanation went and the true half
    // stayed (the sixth-sentence ruling of 2026-09-18).
    //
    // **It stays this screen's own and does not join
    // `crate::surface::files::A_CONNECTION_IS_NOT_BUILT`**, which five other screens
    // share. Those five wait on one thing, a request travelling to the server's
    // own program; the acts still owed here wait on four different pieces, so a
    // shared sentence would be claiming a reason they do not share. Its words
    // were shortened in step 13's words sweep, and no longer open the way its
    // neighbour's do.
    pub const NOT_BUILT_YET: &str = "Not available yet.";

    // **Step 10a round 1's calls, each naming what happened and nothing more**,
    // and each Design's owed words at this place — the way
    // `src/surface/the_failure_shape.rs` marks its own.
    //
    // Fix-here item 4: a read of the server rows that did not answer is said
    // where the list goes, in the server screen's own shape.
    pub const THE_LIST_COULD_NOT_BE_READ: &str = "Could not read your servers.";
    // Fix-here item 12: a write the record refused.
    pub const THE_CHANGE_WAS_NOT_SAVED: &str = "Could not save the change.";
    // **`Servers` S10 item 5 and P3, and Design's owed words at this place.**
    // `Design/Servers` draws a failed connection as a line on the row composed
    // from the connection condition record, and an opening attempt raises none
    // — `Servers` N5 raises at a run's failure, and an opening attempt belongs
    // to no run. **So the act's own answer is where it is said.** It names what
    // happened and nothing more, which is `Preparation/Specs/Whole app.md`'s
    // rule that Smoozy never claims to know which of the three causes it was.
    pub const THE_CONNECTION_WAS_NOT_MADE: &str = "Could not connect to this server.";
    // **The same fact above the All servers list, where *this server* names
    // nothing** — `Design/Servers § Adding a server` and `Design/Keys`' line
    // 143 since 2026-10-03, the Creator's note 14 on the test copy: *Smoozy
    // could not connect to Hyperstack.* The sentence stops where the name
    // begins (X7), and the page sets the full stop after the name as it does
    // for *Another server is already called …*.
    pub const COULD_NOT_CONNECT_TO: &str = "Could not connect to";
    // **`Servers` Q13 outcome 2, and Design's owed words at this place.** The
    // clause is *"says plainly that it is not a Linux machine"* and this is
    // that sentence: what Smoozy found, then why it stops there. **The second
    // half is what makes the first worth saying** — a user told only that the
    // machine is not Linux is left to work out whether it matters.
    pub const IT_IS_NOT_A_LINUX_MACHINE: &str =
        "Not a Linux server. Only Linux is supported.";
    // Fix-here items 12 and 32: a server another act removed. **The server
    // screen's own sentence**, taken from there rather than written twice.
    pub const THIS_SERVER_HAS_BEEN_REMOVED: &str =
        crate::surface::a_server::THIS_SERVER_HAS_BEEN_REMOVED;
}

/// **The two setup options, as the remembered setup records them** — `Servers`
/// B1: *"It records which options were ticked."* **The spelling is this
/// document's and nobody else writes one**; `Server Setup` reads them from the
/// row on the connection that carries the bit (`Servers` B5), which nothing
/// builds yet. Step 10a round 1, fix-here item 13.
pub const GET_THE_SERVER_READY: &str = "get-the-server-ready";
pub const INSTALL_THE_LATEST_STABLE_BLENDER: &str = "install-the-latest-stable-blender";

/// **The three acts `Desktop app and delivery` V13's row lines carry.** Step
/// 10c's transfer wave piece 9.
///
/// - **The row's *Update now…*** opens V15's box, and the page takes it itself
///   — it reaches nothing.
/// - **The box's *Stop jobs and update*** is V15's act, sent here.
/// - **The failed line's cross** withdraws *the update failed* by hand.
pub const THE_UPDATE_NOW_IS_ASKED: &str = "update-now-asked";
pub const THE_UPDATE_NOW: &str = "update-now";
pub const THE_UPDATE_FAILURE_IS_CLEARED: &str = "clear-the-update-failure";

// ════════════════════════════════════════════════════════════════════════════
// What the page is given
// ════════════════════════════════════════════════════════════════════════════

/// **A piece of prose with the values that belong beside it, and never inside
/// it.**
///
/// **X7 is the whole reason this type exists.** A sentence naming a server, a
/// count or a moment is composed as words plus values, and the page sets them
/// side by side. A drawing that joined them here would put a non-breaking space
/// inside a name the user copies and nothing anywhere would report it.
///
/// **The page is also told whether the rest is bound to the value before it**
/// (`the_rest_binds`, below), so a figure and its unit, or a count and what it
/// counts, never part across two lines — `Design/Whole app § Typography`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Words {
    /// The words before the first value. **Prose, through the point.**
    pub lead: Prose,
    /// The values, in the order they are drawn, each by the verbatim route.
    pub values: Vec<Verbatim>,
    /// The words after the last value, where there are any.
    pub rest: Option<Prose>,
    /// **Whether the rest follows the value with nothing between them.**
    ///
    /// The drawing writes *Remove RunPod 4090?* with the question mark against
    /// the name. **Whether a sentence has a space there is the sentence's own
    /// fact and the application's to state**, so it travels with the words
    /// rather than being guessed at on the page — a page deciding it would be a
    /// page reading prose.
    pub the_rest_joins: bool,
}

/// **What the page reads**, which is the five fields and one more worked out
/// from them: `the_rest_binds`. It is worked out here, where every `Words`
/// becomes what the page reads, rather than set by each of the places that
/// compose one — **so a sentence composed later is bound without anybody
/// remembering to**, which is the ruling's own reason for holding the rule in
/// one place (`Design/Whole app § Typography`: *"nobody counts units while
/// designing a screen"*).
impl serde::Serialize for Words {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut said = serializer.serialize_struct("Words", 5)?;
        said.serialize_field("lead", &self.lead)?;
        said.serialize_field("values", &self.values)?;
        said.serialize_field("rest", &self.rest)?;
        said.serialize_field("the_rest_joins", &self.the_rest_joins)?;
        said.serialize_field("the_rest_binds", &self.the_rest_binds())?;
        said.end()
    }
}

impl Words {
    /// **Whether the rest follows the last value with a non-breaking space**
    /// rather than an ordinary one: where that value is a figure and words
    /// follow it — *24 GB each*, *2 waiting*, *Waiting 2 jobs*.
    /// `Design/Whole app § Typography`; the page writes
    /// [`crate::typography::BETWEEN_A_FIGURE_AND_ITS_UNIT`] where this says so.
    /// A rest that joins with nothing between has no space to bind.
    pub(crate) fn the_rest_binds(&self) -> bool {
        !self.the_rest_joins
            && self.rest.as_ref().is_some_and(|rest| !rest.as_str().is_empty())
            && self
                .values
                .last()
                .is_some_and(|value| crate::typography::is_a_figure(value.as_str()))
    }

    /// Words alone.
    fn plain(lead: &str) -> Words {
        Words {
            lead: Prose::new(lead),
            values: Vec::new(),
            rest: None,
            the_rest_joins: false,
        }
    }

    /// Words, then a value.
    fn with(lead: &str, value: impl Into<String>) -> Words {
        Words {
            lead: Prose::new(lead),
            values: vec![Verbatim::new(value.into())],
            rest: None,
            the_rest_joins: false,
        }
    }

    /// A value, then the words about it — *1 server*, *last seen 11:04*'s
    /// mirror. **The two are still two fields**; which comes first is a fact
    /// about the sentence and not a licence to join them.
    fn after(value: impl Into<String>, rest: &str) -> Words {
        Words {
            lead: Prose::new(""),
            values: vec![Verbatim::new(value.into())],
            rest: Some(Prose::new(rest)),
            the_rest_joins: false,
        }
    }

    /// **Words, a value, then a question mark against it** — the drawing's own
    /// *Remove RunPod 4090?*, where the mark belongs to the sentence and sits
    /// hard against the name.
    fn asked(lead: &str, value: impl Into<String>) -> Words {
        Words {
            lead: Prose::new(lead),
            values: vec![Verbatim::new(value.into())],
            rest: Some(Prose::new("?")),
            the_rest_joins: true,
        }
    }
}

/// One control the page draws, with the act it calls back with.
///
/// **The label and the act are two strings on two routes**, which is the shape
/// `crate::presentation::the_failure_shape::AControl` already uses: the label is
/// prose and the act is a token Smoozy never alters.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AControl {
    pub label: Prose,
    pub act: Verbatim,
    /// Whether the control is drawn unavailable. **Design: each menu shows the
    /// actions it cannot do rather than hiding them.**
    pub unavailable: bool,
}

impl AControl {
    fn live(label: &str, act: &str) -> AControl {
        AControl {
            label: Prose::new(label),
            act: Verbatim::new(act),
            unavailable: false,
        }
    }

    fn shown_unavailable(label: &str, act: &str) -> AControl {
        AControl {
            label: Prose::new(label),
            act: Verbatim::new(act),
            unavailable: true,
        }
    }
}

/// **One of Design's five menus, in its five slots and always in that order.**
///
/// *"Every menu is the same five slots in the same order, so a user who has
/// disconnected a server twice does not have to read the menu a third time."*
/// Label: **Creator-stated** (2026-08-13). **A slot with nothing in it is
/// skipped, not padded**, and the order still holds.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AMenu {
    /// Which of the five this is, for the page to key on. A value.
    pub which: Verbatim,
    /// Slot 1 — the connection. **Always exactly one item.**
    pub the_connection: AControl,
    /// Slot 2 — going into the server.
    pub going_in: Vec<AControl>,
    /// Slot 3 — what this shape can do about itself, where it has anything.
    pub about_itself: Vec<AControl>,
    /// Slot 4 — the server's own details.
    pub the_details: Vec<AControl>,
    /// Slot 5 — remove.
    pub removing: AControl,
}

/// The hardware column, as the page draws it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheHardware {
    /// **`Servers` H1's three states, written** — `never read`, `last seen`,
    /// `live`. The page marks the reading with it and never derives it.
    pub state: Verbatim,
    /// The cards line, where there is a reading.
    pub cards: Option<Words>,
    /// The machine's own memory, where the reading has one.
    pub memory: Option<Words>,
    /// The free disk, where the reading has one.
    pub free_disk: Option<Words>,
    /// **The whole statement, where there is nothing to show** — `Servers` H1's
    /// *never read* row shows *that there is nothing to show* rather than an
    /// empty row.
    pub nothing_to_show: Option<Words>,
    /// **`Servers` H2a's *last seen* mark**, drawn from the desktop's own
    /// moment. Absent in the *live* and *never read* states.
    pub last_seen: Option<Words>,
}

/// The rental line — `Servers` §11.8.
///
/// **`Servers` W40a: the line is never empty of both.** Either the figure, marked *live*
/// or *last seen*, or `Servers` W40's statement composed in its place.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheRentalLine {
    /// The running-time figure with its own three states, where one stands.
    pub running_for: Option<Words>,
    /// **`Servers` W40's statement**, where the figure is a stated absence.
    pub the_statement: Option<Words>,
}

/// One row of the list.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AServerRow {
    /// The row's identity, which every act hands back. A value.
    pub server: Verbatim,
    /// **The name, or `username@host` where the server has none** —
    /// `Design/Servers § A row`. A value: the user can copy it and X7 forbids
    /// the point touching it.
    pub name: Verbatim,
    /// **The address, in the fixed-width face.** A value.
    pub address: Verbatim,
    /// One of the drawing's eight badges, or nothing where the stored colour is
    /// not one of them.
    pub badge: Option<&'static str>,
    /// **The extra line** — *"either a failure that is still standing, or the
    /// mark saying the server needs a new key"*. **Where both are true the
    /// failure takes the line** (`Design/Servers`), and the failure itself is
    /// drawn by the one failure shape rather than by this row.
    pub the_key_mark: Option<Words>,
    /// **`Security and licence` S34's one sentence, on this row** — where the
    /// moment it is said at came inside a repair Smoozy made on its own, and no
    /// box was in front of the user (`Design/Keys § On a computer with nowhere
    /// to keep a passphrase`). **Drawn whether or not the mark has the line**,
    /// because it is said once.
    pub the_sentence: Option<Prose>,
    /// **A setup that stopped part-way**, where `Server Setup` has read one.
    /// Absent today: that reading is taken on a connection.
    pub the_setup_line: Option<Words>,
    /// **Smoozy on this server older or newer than the app** — one of
    /// `Desktop app and delivery` V13's conditions, in `Design/Servers § A
    /// row`'s words, **only while the server is connected**: each is a reading
    /// a connection took. The page gives it the extra line after a standing
    /// failure and before the key mark (see [`TheUpdateLine`]).
    pub the_update_line: Option<TheUpdateLine>,
    /// **What it is doing** — `Design/Servers § A row`, and
    /// `Desktop app and delivery` M29's four words. **The dash where Smoozy
    /// cannot say.**
    pub the_work: Vec<Words>,
    /// **`Servers` N10a's moment, where a repairing server has one** —
    /// `Design/Servers § A row`: *Reconnecting…* *"with when the next attempt
    /// falls"*. **The page draws it with the underway shape's own countdown**
    /// (`web/the-underway-shape.js`, `Desktop app and delivery` M30), which
    /// ticks against the moment and sends nothing. *(Step 10b's first audit
    /// round, item 9.)*
    pub the_next_attempt: Option<TheNextAttempt>,
    /// **What it has to render with.**
    pub the_hardware: TheHardware,
    /// The connect control, carrying the connection state.
    pub the_connection: AControl,
    /// **Whether the user's own *Connect* is being made now**, so the page
    /// puts Design's *on the control* mark on the control, with *Connect*
    /// held under it to keep its width — [`TheUsersConnects`]. Never true of a
    /// repairing server, whose control keeps its own state. Added 2026-09-27,
    /// eighth-pass fix piece 5.
    pub the_connect_is_underway: bool,
    /// **Whether Smoozy is bringing the connection back on its own** — a
    /// repair, or a restart it made — so the control keeps its amber, the
    /// drawing's `trying`, and not the green of a connection that is there.
    /// `Design/Whole app § Anything that isn't instant`: *"its Connect control
    /// keeps its amber — which is state rather than an underway mark"*. Added
    /// 2026-09-28, step 10c's ninth-pass fix piece 2, out of the ninth pass,
    /// where a repairing server's control was drawn green.
    pub the_connection_is_coming_back: bool,
    /// Which of the five menus this row is given. A value the page keys on.
    pub the_menu: Verbatim,
    /// **The label of the *More* control**, for anyone who cannot see it.
    pub more: Prose,
}

/// **One of `Desktop app and delivery` V13's conditions, as the row's extra
/// line** — `Design/Servers § A row` and the drawing's `sv-upd`. Added
/// 2026-09-28, step 10c's transfer wave piece 9.
///
/// **Which line takes the row's one extra line.** A standing failure record
/// first, as for every other line (`Design/Servers`: *"A failure takes the line
/// first"*); then this; then the key mark and the passphrase mark; then a setup
/// stopped part-way. **This goes before the key mark for the reason the failure
/// does**: it ends by itself when its condition ends, and the key mark stands
/// either way and takes the line back afterwards.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheUpdateLine {
    /// **Which of the six**, for the page to key on. A value: `waits`,
    /// `under-way`, `failed`, `too-old`, `too-old-unknown`, `too-new`.
    pub which: Verbatim,
    /// The line.
    pub said: Words,
    /// **Whether it is drawn in the failure face** — only *the update failed*,
    /// which V13 makes a failure against the server. The rest are statements
    /// in the ordinary face.
    pub a_failure: bool,
    /// **V15's *Update now…*, inside the line** — offered only where the update
    /// waits and the connection is live. **Never on *too old***: the stop
    /// cannot reach that program.
    pub update_now: Option<AControl>,
    /// **The box *Update now…* opens** (`sv-updnow`), composed with the line so
    /// the page asks for nothing more.
    pub the_box: Option<TheUpdateNowBox>,
    /// **The server's own words behind the failed line**, as every failure
    /// carries them — the transaction's own words (V13), and the cross.
    pub the_servers_words: Option<TheServersWords>,
}

/// **V15's box** — `Design/Servers § A row` and the drawing's `sv-updnow`: the
/// question, then the removal box's own statement of what *stop and clear*
/// does (`Servers` W19a, unchanged), then *Cancel* and *Stop jobs and update*.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheUpdateNowBox {
    /// *Update Smoozy on RunPod 4090 now?* — words, the name, words.
    pub heading: Words,
    pub said: Prose,
    /// **The removal's own warning from the reading** — *1 job is rendering and
    /// 2 are waiting on this server.* — in runs, where the reading has work.
    pub the_warning: Option<Vec<Words>>,
    /// **The removal's own *stop and clear first* statement**, word for word.
    pub the_statement: Words,
    pub cancel: Prose,
    /// *Stop jobs and update*, and the act it sends.
    pub stop_jobs_and_update: AControl,
}

/// **What stands behind *Smoozy on this server could not be updated.***
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheServersWords {
    /// *What the server said* — the control that opens the words.
    pub label: Prose,
    /// **The transaction's own words, whole** — a value, never prose.
    pub words: Verbatim,
    /// The panel's own *Close*.
    pub close: Prose,
    /// **The cross, and the act it sends**: *the update failed* is a condition
    /// against the server, and a condition carries one.
    pub clear: AControl,
}

/// **When the next attempt falls, as the countdown takes it.**
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheNextAttempt {
    /// **The moment, in seconds since the epoch, exactly as `Servers` N10a
    /// published it** — a moment and never a remaining time (M30).
    pub due_at: Verbatim,
    pub trying_again_in: Prose,
    pub trying_again_now: Prose,
}

/// The footer under the list.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheFooter {
    pub how_many: Words,
    pub how_many_connected: Words,
}

/// **The guidance for an empty list** — `Design/Servers § No servers yet`,
/// **and it stays reachable from the Add dialog once there are servers**
/// (settled at Design, 2026-08-23).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheGuidance {
    pub heading: Prose,
    pub how_it_goes: Prose,
    /// Five steps, each a bold head and the rest of the sentence.
    pub steps: Vec<TheStep>,
    pub open_keys: Prose,
    pub add_a_server: Prose,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheStep {
    pub lead: Prose,
    pub rest: Option<Prose>,
    /// Whether this step carries the *Open Keys* link.
    pub opens_keys: bool,
}

/// The two boxes a connection raises, composed and drawn by nothing today.
///
/// **They are on this answer rather than behind a command of their own**,
/// because the screen owns them and a command nothing can call is a command
/// nobody can find again.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheBoxesAConnectionRaises {
    /// `Design/Servers § A changed identity`, and the mockup's `sv-ident`.
    pub the_changed_identity: TheChangedIdentityBox,
    /// `Design/Servers § When stop and clear does not work`, and `sv-sc`.
    pub stop_and_clear: Vec<TheStopAndClearEnding>,
    /// The stop-and-clear box's own heading, which names the server.
    pub stopping_work_on: Prose,
    /// **Removing anyway stays available in all three** — `Design/Servers`.
    pub try_again: Prose,
    pub remove_anyway: Prose,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheChangedIdentityBox {
    /// The heading, which names the server. Words and a value.
    pub heading: Words,
    pub said: Prose,
    pub accepting: Prose,
    pub do_not_connect: Prose,
    pub accept: Prose,
}

/// **One of `Servers` W21's three distinguishable results.**
///
/// *"All three name what actually happened rather than saying it failed"* —
/// and the difference between them *"is the difference between a machine still
/// costing money and one that is not"*.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheStopAndClearEnding {
    /// Which of the three, for the page to key on. A value.
    pub which: Verbatim,
    pub lead: Prose,
    pub rest: Prose,
}

/// **One owed `sv-sc` box**: which server, and which of `Servers` W21's
/// endings. Both are values; the words are the box's own, on
/// [`TheBoxesAConnectionRaises::stop_and_clear`].
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheOwedEnding {
    pub server: Verbatim,
    pub which: Verbatim,
}

/// **The SSH-tools statement, in the Keys screen's own three sentences** —
/// three pieces of prose rather than one (X10), and the same three that screen
/// sends. Step 10a round 1, fix-here item 30.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheToolsStatement {
    pub lead: Prose,
    pub until: Prose,
}

/// **Why a screen or a box could not be shown, said** — L8a: a screen shows
/// the failure rather than an emptied version of itself, and a press that
/// opens nothing says so where it was pressed (step 10a round 1, fix-here
/// items 4 and 32).
///
/// **It is the error half of a command's answer**, so the page's own error
/// path is the one that draws it and is reachable.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ItCouldNotBeShown {
    pub said: Prose,
}

impl ItCouldNotBeShown {
    fn saying(said: &str) -> ItCouldNotBeShown {
        ItCouldNotBeShown {
            said: Prose::new(said),
        }
    }
}

/// **What `Keys` said about the keys the server rows name** — `Servers` P1
/// question 1 and `Servers` P13, read at the list draw.
///
/// **Read off the drawing thread, as the Keys screen reads it**, and **one
/// tools reading for the whole ask** (`Keys` G6). Step 10a round 1, fix-here
/// items 14 and 20.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatKeysSaid {
    /// `Keys` G6, asked once for the whole ask.
    pub the_tools_are_installed: bool,
    /// Each key row's own verdict, by its local key identifier.
    pub verdicts: Vec<(String, usable::Verdict)>,
    /// **The keys `Servers` P1's second question answers *no* for** — protected
    /// (`Keys` U8) with no passphrase available without asking (`Keys` U9) —
    /// by local key identifier. **`Servers` P3 leaves such a server marked with
    /// what is wrong**, and `Keys` S10 words the mark.
    pub the_passphrase_is_not_available: Vec<String>,
    /// **`Security and licence` §3.3's store standing**, read with the rows —
    /// or nothing where the read did not answer. `Keys` S10's mark turns on it.
    pub the_store_standing: Option<smoozy_security_and_licence::standing::StoreStanding>,
}

impl WhatKeysSaid {
    fn the_verdict_on(&self, identifier: &str) -> Option<&usable::Verdict> {
        self.verdicts
            .iter()
            .find(|(named, _)| named == identifier)
            .map(|(_, verdict)| verdict)
    }
}

/// The whole screen, in one answer.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheAllServersScreen {
    pub the_screens_own_name: Prose,
    pub add_a_server: Prose,
    /// **The one SSH-tools statement, said once and never per row** —
    /// `Design/Servers § A row`, and `Keys` U6a's rule underneath it.
    pub the_tools_statement: Option<TheToolsStatement>,
    pub rows: Vec<AServerRow>,
    pub footer: TheFooter,
    /// **The empty state, and it is always composed.** The Add dialog links to
    /// it, so it is not a thing only an empty list has.
    pub guidance: TheGuidance,
    /// The five menus, composed once and shared by every row.
    pub menus: Vec<AMenu>,
    pub the_boxes_a_connection_raises: TheBoxesAConnectionRaises,
    /// **`Servers` W21's ending of a removal's stop and clear that did not come
    /// to *cleared and stopped*, owed on this list** — the `sv-sc` box, raised
    /// by the page for the server named. **Handed over once**: the answer that
    /// carries it takes it off the run's register ([`THE_ENDINGS_OWED`]), so a
    /// list the user was away from when the act ended shows it the next time
    /// it opens (`Servers` W22), and a list already showing it does not show it
    /// twice. Step 10c's S2.
    pub the_stop_and_clear_ending: Option<TheOwedEnding>,
    /// **`Design/Servers § Keyboard shortcuts`: Return opens the chosen server,
    /// the delete key removes it, and nothing else is bound.**
    pub the_keys_that_are_bound: Vec<Verbatim>,
    /// **Which durable subject a failure on one of these rows is about**, as
    /// `State, identity and recovery` §8.7's list writes it.
    ///
    /// **It is supplied rather than written on the page** for the reason A6 and
    /// X1 give: a page holding the words itself is a page that has drifted from
    /// the record the day either moves, and the sweep at `src/surface/mod.rs`
    /// is what says so.
    pub the_subject_kind: Verbatim,
}

/// The Add dialog's own facts.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheAddDialog {
    pub heading: Prose,
    pub the_guidance_again: Prose,
    pub host: Prose,
    /// **The host field's placeholder says the paste works** —
    /// `Design/Servers § Adding a server`, drawn in `sv-blank`.
    pub host_placeholder: Prose,
    pub port: Prose,
    pub username: Prose,
    pub key: Prose,
    pub name: Prose,
    pub optional: Prose,
    pub new_key: Prose,
    pub colour: Prose,
    /// **`Design/Servers § Adding a server`: the port field starts at 22, the
    /// standard SSH port.** A value.
    pub the_port_starts_at: Verbatim,
    /// The eight badges, in the drawing's own order.
    pub the_eight: Vec<Verbatim>,
    /// Which of the eight a server already uses. Values.
    pub already_used: Vec<Verbatim>,
    /// **The colour arrives already picked, and it is one no other server is
    /// using** — Creator-stated (2026-08-13). **With more than eight servers
    /// Smoozy picks the hue used by the fewest** rather than refusing to add
    /// one.
    pub picked_for_you: Verbatim,
    pub the_swatch_note: Prose,
    /// **The key control before a key is picked** — it opens the Keys screen to
    /// pick one (`Keys` U11).
    pub choose_a_key: Prose,
    pub get_ready: Prose,
    pub install_blender: Prose,
    pub ticking_either: Prose,
    pub cancel: Prose,
    /// The label with neither option ticked.
    pub add: Prose,
    /// The label with either ticked.
    pub add_and_set_up: Prose,
    /// **`Servers` W40's statement**, made at the add's save and drawn here
    /// because the add is where it is made.
    pub runs_and_bills: Prose,
}

/// **What a pasted connection line fills the Add dialog with** — `Servers`
/// §13.3, read by `smoozy_servers::pasted_line`. The Creator's second
/// notes, piece 15.
///
/// **Nothing here is saved**: the fields are filled, the dialog's own checks
/// run on them as on typed values, and the server is written only at *Add*.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ThePastedLine {
    /// **Whether the fields take the line's values.** A line of a form that
    /// cannot carry files fills nothing (`Servers` PL3).
    pub fills: bool,
    /// The three fields, each empty where the line has nothing for it. The
    /// port is never empty: a line with no port means 22. Values.
    pub host: Verbatim,
    pub port: Verbatim,
    pub username: Verbatim,
    /// **The one line under Host, in the form's grey**: what the line lacks,
    /// or why it fills nothing. Empty where there is nothing to say.
    pub the_line_under: Vec<Prose>,
}

/// The Edit dialog's own facts, read from the row.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheEditDialog {
    /// The heading, which names the server. Words and a value.
    pub heading: Words,
    pub host: Prose,
    pub port: Prose,
    pub username: Prose,
    pub key: Prose,
    pub name: Prose,
    pub new_key: Prose,
    pub cancel: Prose,
    pub save: Prose,
    /// The values the fields start at, each verbatim.
    pub the_host: Verbatim,
    pub the_port: Verbatim,
    pub the_username: Verbatim,
    pub the_name: Verbatim,
    pub the_key_identifier: Verbatim,
    /// **The key the row names, as the key control shows it** — the key's own
    /// name where it has one, its fingerprint where it has none, and nothing
    /// where the key row is gone. A value.
    pub the_key_shows_as: Option<Verbatim>,
    /// The key control's words where nothing is picked.
    pub choose_a_key: Prose,
    /// **`Specs/Servers`: clearing the port puts 22 back.** A value.
    pub the_standard_port: Verbatim,
    /// **`Servers` D14, said under the username field as a plain statement**, so
    /// a user who changes a username is not surprised to be asked for a
    /// password next time.
    pub the_username_note: Prose,
    /// **`Design/Servers § The Edit dialog`: an unusable key stays selected
    /// rather than being cleared**, and the note says why.
    pub the_gone_key_note: Option<Prose>,
    /// **The gone mark beside that key's name**, in the closed control and on
    /// the top row of its list (`Design/Servers § The Edit dialog`; the
    /// drawing's *Hetzner — missing*). Present exactly where the note is. The
    /// Creator's eighth notes, piece E3.
    pub the_gone_mark: Option<Prose>,
    /// **The place a remembered machine password is forgotten**, absent where
    /// none is remembered rather than greyed.
    pub a_password_is_remembered: Option<Prose>,
    pub forget_it: Prose,
    /// **The server's fallback folder** (`Servers` D3, `Job files and
    /// outputs` P23): the label, the value the field starts at — verbatim, empty where
    /// none is set — the placeholder and the grey line under it.
    pub fallback_folder: Prose,
    pub the_fallback_folder: Verbatim,
    pub the_fallback_placeholder: Prose,
    pub the_fallback_note: Prose,
}

/// **The key a server dialog holds, read from its key row** — `Servers` P1a's
/// pair, and what the dialog's key control shows. Step 10a round 1, fix-here
/// item 2.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheKeyADialogHolds {
    /// The local key identifier. A value the dialog hands back on save.
    pub key: Verbatim,
    /// **The fingerprint the key row holds now** — never one the page made.
    pub fingerprint: Verbatim,
    /// The key's own name, or its fingerprint where it has none. A value.
    pub shows_as: Verbatim,
}

/// **A server dialog's key list, as the record holds it** — `Design/Keys §
/// Opened from a server dialog`, the drawing's `sv-addkey`: Smoozy's own
/// dropdown, each key by name with its fingerprint's start beside it, and *Open
/// Keys…* last. The Creator's eighth notes, note 3, piece E3.
///
/// **`Keys` U10a: it is drawn before a single reading is taken**, so it
/// carries no verdict. Every key arrives *Checking…* and each verdict follows
/// on [`A_DIALOG_KEYS_VERDICT`]; one that reads unusable leaves the list.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheKeysADialogOffers {
    /// **In the Keys screen's own order** — `Design/Keys § The list's order`,
    /// by name, so two keys sharing a name sit side by side.
    pub keys: Vec<AKeyADialogOffers>,
    /// What a key reads while its verdict has not arrived.
    pub checking: Prose,
    /// The last row's words.
    pub open_keys: Prose,
    /// The mark after a fingerprint's start that was cut short.
    pub the_fingerprint_runs_on: Prose,
}

/// **One key on a server dialog's list.** A key with no fingerprint is not
/// offered: the dialog hands back `Servers` P1a's pair, and it has no second
/// half.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AKeyADialogOffers {
    /// The local key identifier, which the page routes on. A value.
    pub key: Verbatim,
    /// The key's own name, or its fingerprint where it has none — what the
    /// closed control shows once it is picked. A value.
    pub shows_as: Verbatim,
    /// **The fingerprint's start, beside the name**, absent where the key has
    /// no name and its whole fingerprint already stands in its place. A value.
    pub fingerprint_start: Option<Verbatim>,
    /// Whether the fingerprint goes on past its start.
    pub it_runs_on: bool,
}

/// **`Servers` W34: the warning names jobs Smoozy has not seen end, never jobs
/// queued or running.** *"It is over-inclusive by construction and the wording
/// must carry that."*
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheEditWarning {
    pub lead: Prose,
    /// How many, as a value.
    pub how_many: Verbatim,
    pub have_not_ended: Prose,
    /// The jobs' own names, each verbatim and never joined into the sentence.
    pub named: Vec<Verbatim>,
    /// **What becomes of them**: left where they are, for a host or port
    /// change; out of Smoozy's reach, for a username change.
    pub what_becomes_of_them: Prose,
}

/// The removal box, composed from `Servers` W19's table.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheRemovalBox {
    /// The heading, which names the server. Words and a value.
    pub heading: Words,
    pub said: Prose,
    /// **The warning, where `Servers` W19's table gives one.** Words and two
    /// counts, in runs the page sets side by side — a sentence with two figures
    /// in it is more than one [`Words`] can hold without joining one in.
    pub the_warning: Option<Vec<Words>>,
    /// **Which of `Servers` W19's three offers this row gets**, written. A value.
    pub offers: Verbatim,
    /// *Leave it running*, where it is offered.
    pub leave_it_running: Option<TheChoice>,
    /// *Stop and clear first*, where it is offered.
    pub stop_and_clear: Option<TheChoice>,
    /// **`Servers` W20's switch-off**, on the one row of the table that may be
    /// turned off and on no other. Absent everywhere else, and **no path can
    /// write one for the connected warning.**
    pub the_switch_off: Option<TheSwitchOff>,
    pub cancel: Prose,
    /// **The plain confirmation's own control**, where no choice is offered.
    pub remove: Option<Prose>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheChoice {
    pub head: Prose,
    pub said: Words,
    pub act: Verbatim,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheSwitchOff {
    pub label: Prose,
    /// **What it covers, said** — *"a switch whose reach is not stated is a
    /// switch the user turns off once and is surprised by later"*.
    pub what_it_covers: Prose,
    /// Whether it is already switched off, read from the one table.
    pub already: bool,
}

/// **The answer to the user's last act, and it travels on the event.**
///
/// **A refusal is an answer to something the user just did.** There is nowhere
/// durable to read one from and inventing one would be a second copy of a fact
/// that has no first. This is piece 1's own reasoning and it carries here
/// unchanged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheAnswer {
    /// The save landed. The page asks for the list again.
    ItWasSaved,
    /// **The add landed carrying a ticked setup option, and the connection and
    /// the setup started with it** — `Servers` O2.
    ///
    /// *(It keeps its name, which is older than what it means: until step 10c
    /// piece 9 the add connected nothing, ticked or not, and the answer said
    /// so. The page reads the name, so the name stays. **It carries no
    /// sentence**: the words it kept saying after piece 9 were false, and step
    /// 10c's eleventh-pass fix piece 6 took them away.)*
    ItWasSavedAndNothingStarted,
    /// **The remembered machine password is forgotten** (`Servers` D18), so the
    /// Edit dialog's line goes. Step 10a round 1, fix-here item 19.
    ThePasswordWasForgotten,
    /// **The server this act was about is no longer there** — another act
    /// removed it. Step 10a round 1, fix-here item 12.
    TheServerIsGone,
    /// **The required fields that were left blank**, each answered on its own
    /// field. `Design/Servers`: *"A required field left blank says so on the
    /// field, and not before it is left."*
    TheseFieldsAreNeeded(Vec<(&'static str, &'static str)>),
    /// **The port refuses in place**, like every other number the app will not
    /// take.
    ThePortWillNotDo,
    /// **A fallback folder that does not start with `/` refuses in place**, in
    /// the port's red, and nothing is saved (`Servers` D3, as ruled 2026-10-04:
    /// *refused at the dialog and never written*).
    TheFallbackFolderWillNotDo,
    /// The record would not take the write.
    TheRecordWouldNotTakeIt,
    /// **The record would not read**, so the act found nothing to act on — the
    /// list's own words for the same fact (step 10b's first audit round, item
    /// 14).
    TheListCouldNotBeRead,
    /// **The act is one this application does not perform yet** — the five
    /// this file's head names as owed. Said rather than left silent: Rule 8
    /// makes Smoozy going quiet a fault however the trouble arose.
    TheActIsNotBuiltYet,
    /// **`Servers` S10 item 5: the connection the act was to make never became
    /// live**, and Smoozy says so.
    ///
    /// **It claims nothing about why, and that is not a shortcut.**
    /// `Preparation/Specs/Whole app.md` forbids the claim — Smoozy cannot tell
    /// a network problem from a machine switched off from a machine that no
    /// longer exists.
    ///
    /// *(Corrected 2026-09-18. This said `Servers`' driving half does not
    /// report which step stopped the attempt, which it now does. **That
    /// changes nothing here**: the claim is forbidden for these eight reasons
    /// whether or not Smoozy knows which of them it met.)*
    TheConnectionWasNotMade,
    /// **`Servers` Q13 outcome 2: the machine named an operating system that is
    /// not Linux**, so Smoozy says plainly that it is not a Linux machine and
    /// leaves the server disconnected.
    ///
    /// **This is the one reason out of nine that carries a claim, and it
    /// carries one because Smoozy read it.** The machine was asked to name its
    /// own operating system and answered with something else; every other
    /// reason takes [`TheAnswer::TheConnectionWasNotMade`], including a machine
    /// that answered nothing Smoozy could read.
    ItIsNotALinuxMachine,
    /// **The passphrase could not be handed to the SSH tools, and Smoozy knows
    /// why** — the sentence is `crate::keys::askpass`'s own, carried whole.
    ///
    /// *(Added 2026-09-18, step 10b's audit round's loose ends, L1.)* The
    /// channel a passphrase travels down could not be opened because its path
    /// is longer than the system takes, and this used to be said as
    /// [`TheAnswer::TheConnectionWasNotMade`]. **That sentence's reasons are the
    /// ones Smoozy cannot tell apart**; this one it can, it is about this
    /// computer rather than the server, and it will not go away by trying
    /// again, so saying *could not connect* told the user nothing true.
    TheKeysPassphraseCouldNotBeUsed(&'static str),
    /// **V15's stop and clear came to *cleared and stopped***, so the box
    /// closes and the update is V11's again. Nothing to say: the row says what
    /// happens next.
    TheUpdateNowWasTaken,
    /// **V15's stop and clear came to anything else**, named by the removal
    /// box's own ending key — said in `Servers` W21's words, as for a removal.
    TheStopAndClearDidNotFullyWork(TheRemovalsEnding),
    /// **The removal's stop and clear came to anything but *cleared and
    /// stopped***, so the server stays and `Servers` W21's ending is owed on
    /// the list. **Nothing is said on the event**: the list, asked for again,
    /// carries the ending ([`TheAllServersScreen::the_stop_and_clear_ending`]),
    /// which is where `Servers` W22's next presentation reads it too. Step
    /// 10c's S2.
    TheStopAndClearEndingIsOwed,
}

// ════════════════════════════════════════════════════════════════════════════
// The commands
// ════════════════════════════════════════════════════════════════════════════

/// **The whole All servers screen, in one answer.**
///
/// **A command rather than an event, and P6 is not being bent.** P6 is about a
/// module's *results*; this is the page asking for the facts it needs before it
/// can draw anything, which is the shape `the_rail` and `chrome_text` already
/// have. **A page that loads again asks again** (P8).
///
/// **It answers three ways and never two** — the rows, *there are none*, or
/// *the read did not answer* — which is the shape `src/surface/a_server.rs`
/// already has. A failed read is the error half, carrying its own sentence,
/// **and never an empty list**: an empty list draws the *rent a server*
/// guidance, and a user who follows it while the record will not read adds
/// every server a second time (step 10a round 1, fix-here item 4).
///
/// **The lock is held around the record's reads and nothing else**, and the
/// key readings are taken off the drawing thread — they run the SSH tools
/// (`Keys` G6, and `Servers` P13's derivation reads `Keys`' verdict). Step 10a
/// round 1, fix-here items 14 and 20.
#[tauri::command]
pub async fn the_all_servers_screen<R: Runtime>(
    app: AppHandle<R>,
    started: State<'_, Mutex<Started>>,
) -> Result<TheAllServersScreen, ItCouldNotBeShown> {
    let periods = the_periods_of(&app);
    let (rows, keys, standing, work) = {
        let Ok(started) = started.lock() else {
            return Err(ItCouldNotBeShown::saying(words::THE_LIST_COULD_NOT_BE_READ));
        };
        let Some(rows) = the_rows(&started) else {
            return Err(ItCouldNotBeShown::saying(words::THE_LIST_COULD_NOT_BE_READ));
        };
        // **`Keys` S10's mark turns on the store's standing**, read with the
        // rows and never assumed.
        let standing = started
            .record
            .as_ref()
            .and_then(|record| crate::surface::set_up::the_store_standing(record.connection()));
        // **The work column needs the job records beside the held reading**,
        // so it is composed here, under the record's lock — the same order
        // the Queue screen takes, and the reading's own lock takes no other.
        let work = periods
            .as_deref()
            .map(|periods| {
                crate::surface::the_queue::what_the_servers_are_doing(
                    periods.the_machines_work(),
                    started.record.as_ref(),
                )
            })
            .unwrap_or_default();
        (rows, the_key_rows(&started), standing, work)
    };
    let said = what_keys_say_off_the_drawing_thread(&rows, keys.as_deref())
        .await
        .map(|said| WhatKeysSaid {
            the_store_standing: standing,
            ..said
        });
    let mut screen = the_screen_from(
        &rows,
        keys.as_deref().unwrap_or_default(),
        said.as_ref(),
        periods.as_deref(),
        &work,
    );
    // **`Servers` W22's next presentation**, taken last so an answer that
    // never reaches the page cannot have taken it.
    screen.the_stop_and_clear_ending = the_owed_ending_is_taken(&rows);
    Ok(screen)
}

/// **P10d's on-screen condition, told by the two screens that draw a work
/// column** — the runner's ruling of 2026-09-28: the server list and a
/// server's own page are surfaces showing that server's queue, since
/// `Design/Servers § A row` draws its rendering job and how many wait. While
/// either is on screen the queue reading repeats, so the column does not go
/// stale while Queue is away. Step 10c's ninth-pass fix piece 2.
///
/// `which` is `the-list` or `a-server`; `server` is the page's server, and
/// nothing where the page is put away.
#[tauri::command]
pub fn the_work_column_is_on_screen(
    periods: State<'_, std::sync::Arc<ThePeriodsThisRun>>,
    which: String,
    server: Option<String>,
    shown: bool,
) {
    let holds = periods.the_machines_work();
    match which.as_str() {
        "the-list" => holds.the_server_list_is(shown),
        "a-server" => holds.a_servers_own_page_is(server.as_deref().filter(|_| shown)),
        _ => {}
    }
}

/// **The work column, told again where it moved** — after `Servers` Q1 step 7,
/// after a P10d pass's queue reading, and after a read-forward that applied
/// something (an ending learned among them). **Only a change is told**, so the
/// list is not asked for again on every pass.
///
/// **And where it moves without a reading**: at a declared break (`Servers` K3
/// and K5, `Queue & History` C7) and at a period's end (`Queue & History` C6).
/// **What was last told is what the detector remembers, not what the page last
/// drew**, and the pages draw again on the connection's own events, so a
/// column that went to the dash unheard stays there when the next reading
/// brings back the answer last told. Added 2026-09-28, step 10c's tenth-pass
/// fix piece 2: the tenth pass watched the dash stand for nine minutes after
/// the first setup's *done*.
pub fn the_work_column_is_told_again<R: Runtime>(app: &AppHandle<R>) {
    let Some(periods) = the_periods_of(app) else {
        return;
    };
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    let Some(started) = app.try_state::<Mutex<Started>>() else {
        return;
    };
    // **And whether a job in the queue needs your answer**, which moves the
    // rail's Queue dot on its own: a held job answered while another renders
    // leaves every work column where it was (the twenty-fourth notes, note 2,
    // `Design/Whole app § The server badge`).
    let now = {
        let Ok(started) = started.lock() else {
            return;
        };
        (
            what_is_told(
                crate::surface::the_queue::what_the_servers_are_doing(
                    periods.the_machines_work(),
                    started.record.as_ref(),
                ),
                the_rows(&started).as_deref().unwrap_or_default(),
            ),
            crate::surface::the_rail::a_job_in_the_queue_needs_your_answer(
                periods.the_machines_work(),
                started.record.as_ref(),
            ),
        )
    };
    if the_work_moved(&THE_WORK_LAST_TOLD, now) {
        the_queue_is_told(&queue, Event::about(About::TheRun, THE_WORK_MOVED));
    }
}

/// **What the row's work and its V13 line are told from**: each server's work,
/// and the condition `Desktop app and delivery` V13 has each carry.
///
/// **The condition is part of it because it moves the row on its own** — the
/// update taken in hand, landed, failed or waiting, or a program read too old
/// or too new — and a V13 move that left the work where it was would otherwise
/// tell the page nothing. Step 10c's transfer wave piece 9.
///
/// **And whether a setup is being driven there**, which moves the row's
/// *Setting up* on its own — the Creator's notes piece 3, 2026-10-02.
type WhatIsTold = (
    WhatTheServersAreDoing,
    Vec<(String, Option<crate::servers::published::WhatSmoozyOnTheServerIs>, bool)>,
);

fn what_is_told(doing: WhatTheServersAreDoing, rows: &[ServerRow]) -> WhatIsTold {
    let carried = rows
        .iter()
        .map(|row| {
            (
                row.local_server_identifier.clone(),
                crate::servers::published::what_the_server_carries(&row.local_server_identifier),
                a_setup_runs_on(&row.local_server_identifier),
            )
        })
        .collect();
    (doing, carried)
}

/// **Whether this copy is driving a setup run on that server now** — any kind
/// of setup run: the ready action, a Blender install, an abandon. Read from
/// `crate::setting_up`'s own exclusion (`Server Setup` Y9) and nothing kept
/// here.
fn a_setup_runs_on(server: &str) -> bool {
    crate::setting_up::the_kind_being_driven_on(server).is_some()
}

/// **`Design/Servers § A row`: while a setup runs, the column says *Setting
/// up*.** *"It takes the place of Idle and the dash, never of Rendering,
/// Waiting or Restarting."* The Creator's note 3 on 0.1.7: the row read *Idle*
/// for the two minutes a setup took.
fn the_setup_takes_the_column(column: Vec<Words>, a_setup_runs: bool) -> Vec<Words> {
    let idle = Words::plain(WhatTheColumnSays::Idle.the_word());
    if a_setup_runs && (column == vec![the_dash()] || column == vec![idle]) {
        return vec![Words::plain(words::SETTING_UP)];
    }
    column
}

/// What the work column was last told as, for the run, beside whether a job in
/// the queue needed your answer then (the rail's Queue dot).
static THE_WORK_LAST_TOLD: Mutex<Option<(WhatIsTold, bool)>> = Mutex::new(None);

/// **Whether `now` differs from what was last told**, keeping `now` as the
/// last told either way.
fn the_work_moved<T: PartialEq>(last: &Mutex<Option<T>>, now: T) -> bool {
    let Ok(mut last) = last.lock() else {
        return true;
    };
    let moved = last.as_ref() != Some(&now);
    *last = Some(now);
    moved
}

/// **The Add dialog's own facts.**
#[tauri::command]
pub fn the_add_dialog(started: State<'_, Mutex<Started>>) -> TheAddDialog {
    let Ok(started) = started.lock() else {
        return the_add_dialog_from(&[]);
    };
    let rows = the_rows(&started).unwrap_or_default();
    the_add_dialog_from(&rows)
}

/// **A connection line pasted into the Add dialog's host field, read** —
/// `Servers` PL1: parsed as text and never run. It reads nothing of the record
/// and writes nothing; the page fills the fields from the answer.
#[tauri::command]
pub fn the_pasted_line_is_read(line: String) -> ThePastedLine {
    the_pasted_line_from(&line)
}

/// **The Edit dialog, read off the row.**
///
/// **A server that has just gone is said where the user pressed**, rather than
/// the press opening nothing (step 10a round 1, fix-here item 32).
#[tauri::command]
pub async fn the_edit_dialog(
    started: State<'_, Mutex<Started>>,
    server: String,
) -> Result<TheEditDialog, ItCouldNotBeShown> {
    let (rows, keys) = {
        let Ok(started) = started.lock() else {
            return Err(ItCouldNotBeShown::saying(words::THE_LIST_COULD_NOT_BE_READ));
        };
        let Some(rows) = the_rows(&started) else {
            return Err(ItCouldNotBeShown::saying(words::THE_LIST_COULD_NOT_BE_READ));
        };
        (rows, the_key_rows(&started))
    };
    let Some(row) = rows
        .iter()
        .find(|row| row.local_server_identifier == server)
        .cloned()
    else {
        return Err(ItCouldNotBeShown::saying(words::THIS_SERVER_HAS_BEEN_REMOVED));
    };
    let said = what_keys_say_off_the_drawing_thread(
        std::slice::from_ref(&row),
        keys.as_deref(),
    )
    .await;
    Ok(the_edit_dialog_from(
        &row,
        &rows,
        keys.as_deref().unwrap_or_default(),
        said.as_ref(),
    ))
}

/// **The key a dialog holds, read from the key row again** — `Keys` U12: *"Its
/// key reference then names a row that is gone, and the field is empty when
/// the dialog is shown again."*
///
/// **The pair comes from the key row and never from the page** (`Servers`
/// P1a). An identifier that names no row, or a row with no fingerprint, is
/// answered with nothing, and the dialog's field is then empty. Step 10a round
/// 1, fix-here item 2.
#[tauri::command]
pub fn the_key_a_dialog_holds(
    started: State<'_, Mutex<Started>>,
    key: String,
) -> Option<TheKeyADialogHolds> {
    let started = started.lock().ok()?;
    let keys = the_key_rows(&started)?;
    let row = keys
        .iter()
        .find(|row| row.identifier.as_text() == key.trim())?;
    let fingerprint = row.fingerprint.as_ref()?;
    Some(TheKeyADialogHolds {
        key: Verbatim::new(row.identifier.as_text()),
        fingerprint: Verbatim::new(fingerprint.value.as_str().to_string()),
        shows_as: Verbatim::new(the_key_shows_as(row)),
    })
}

/// **A server dialog's key list, from the record and from nothing else** —
/// `Design/Keys § Opened from a server dialog`, the Creator's eighth notes,
/// note 3, piece E3.
///
/// **`Keys` U10a: no draw waits on a reading**, so this takes none. The page
/// draws the list it answers, every key *Checking…*, and then asks
/// [`the_keys_a_dialog_offers_are_checked`] for the verdicts. Asking in two
/// steps is what keeps them in order: a verdict posted while this answer was
/// still crossing would reach a list not yet drawn and be dropped.
#[tauri::command]
pub fn the_keys_a_dialog_offers(started: State<'_, Mutex<Started>>) -> TheKeysADialogOffers {
    let keys = started
        .lock()
        .ok()
        .and_then(|started| the_key_rows(&started))
        .unwrap_or_default();
    the_keys_a_dialog_offers_from(&keys)
}

/// **Each key on a dialog's list, checked off the drawing thread, one verdict
/// at a time** — `Keys` U10, the one function, and `Keys` U10a's order: *"each key's
/// answer reaches the interface as its own event on `Desktop app and delivery`
/// P6's one ordered queue, so a slow key holds up no other row."*
///
/// **One ask per appearance of the list.** The page asks when the dialog is
/// drawn and again when it is shown after Keys, and `ask` comes back on every
/// verdict so the page can drop the answers to an ask it has moved past.
/// **Nothing is cached**: each ask reads each key afresh.
#[tauri::command]
pub fn the_keys_a_dialog_offers_are_checked<R: Runtime>(app: AppHandle<R>, ask: String, keys: Vec<String>) {
    std::thread::spawn(move || the_dialogs_keys_are_checked(&app, &ask, &keys));
}

fn the_dialogs_keys_are_checked<R: Runtime>(app: &AppHandle<R>, ask: &str, asked: &[String]) {
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    let queue = queue.inner().clone();
    // **Where Smoozy's own temporary area cannot be named, nothing is read**,
    // as on the Keys screen: the record sits under the same location.
    let Some(area) = crate::platform::locations::smoozys_own_temporary_area() else {
        return;
    };
    // **The rows are read under the lock and checked after it is let go** — a
    // check is a filesystem read and a tool run.
    let rows = {
        let Some(started) = app.try_state::<Mutex<Started>>() else {
            return;
        };
        let Ok(started) = started.lock() else {
            return;
        };
        the_key_rows(&started).unwrap_or_default()
    };
    let readings = crate::keys::readings::OnThisComputer::over(area);
    for key in asked {
        // **A key whose row went since the list was drawn cannot be used**, and
        // says so by reading unusable, so it leaves the list.
        let verdict = rows
            .iter()
            .find(|row| row.identifier.as_text() == *key)
            .map(|row| usable::check(row, &readings).verdict)
            .unwrap_or(usable::Verdict::Unusable(usable::Unusable::FileMissing));
        let event = a_dialog_keys_verdict(ask, key, &verdict);
        let _ = queue.read_then_post::<(), std::convert::Infallible>(|| Ok(()), |_| event.clone());
    }
}

/// **The list, composed from the key rows handed in.**
pub fn the_keys_a_dialog_offers_from(keys: &[smoozy_keys::rows::KeyRow]) -> TheKeysADialogOffers {
    // **The Keys screen's own order**, taken from that screen's composer so the
    // two lists can never disagree. Nothing in its answer but the order and the
    // rows' own values is used here.
    let ordered = crate::surface::keys::the_screen_from(
        keys,
        None,
        false,
        crate::surface::keys::TheMode::Housekeeping,
    );
    let keys = ordered
        .rows
        .into_iter()
        .filter_map(|row| {
            let fingerprint = row.fingerprint?;
            let (start, it_runs_on) = the_fingerprints_start(&fingerprint);
            let named = row.name.filter(|name| !name.is_empty());
            Some(AKeyADialogOffers {
                key: Verbatim::new(row.identifier),
                shows_as: Verbatim::new(named.clone().unwrap_or_else(|| fingerprint.clone())),
                fingerprint_start: named.as_ref().map(|_| Verbatim::new(start)),
                it_runs_on: named.is_some() && it_runs_on,
            })
        })
        .collect();
    TheKeysADialogOffers {
        keys,
        checking: Prose::new(crate::surface::keys::CHECKING),
        open_keys: Prose::new(words::OPEN_KEYS_ITEM),
        the_fingerprint_runs_on: Prose::new(words::THE_FINGERPRINT_RUNS_ON),
    }
}

/// **How many characters of a fingerprint's hash the list shows** — the
/// drawing's *SHA256:qYt4x1Lp…*: enough to tell two keys of one name apart at a
/// glance, short enough to sit beside the name.
const A_FINGERPRINTS_START: usize = 8;

/// **A fingerprint's start: its algorithm, its colon, and the first eight
/// characters of its hash**, and whether the hash goes on past them. Nothing is
/// put into the value; it is only cut.
fn the_fingerprints_start(fingerprint: &str) -> (String, bool) {
    let (lead, hash) = match fingerprint.find(':') {
        Some(at) => fingerprint.split_at(at + 1),
        None => ("", fingerprint),
    };
    let mut characters = hash.chars();
    let mut start = lead.to_string();
    start.extend(characters.by_ref().take(A_FINGERPRINTS_START));
    (start, characters.next().is_some())
}

/// **One key's verdict, as the dialog's list is told it.** `verdict` is the
/// Keys screen's own word for it — *usable*, *unusable* or *not-checked* —
/// and only *usable* may be picked.
pub fn a_dialog_keys_verdict(ask: &str, key: &str, verdict: &usable::Verdict) -> Event {
    Event::about(About::TheRun, A_DIALOG_KEYS_VERDICT)
        .saying("ask", Said::Value(Verbatim::new(ask)))
        .saying("key", Said::Value(Verbatim::new(key)))
        .saying(
            "verdict",
            Said::Value(Verbatim::new(crate::surface::keys::the_verdicts_word(verdict))),
        )
}

/// **The removal box, composed from `Servers` W19's table.**
#[tauri::command]
pub fn the_removal_of<R: Runtime>(
    app: AppHandle<R>,
    started: State<'_, Mutex<Started>>,
    server: String,
) -> Result<TheRemovalBox, ItCouldNotBeShown> {
    // **The run's periods are read before the application's lock is taken** —
    // step 10b's first audit round, item 2: read under it, this command held
    // the application's lock and then took the publisher's, the opposite order
    // to a period publishing, and the two could freeze the window for good.
    // **`Servers` W19's table is keyed on whether a period stands and whether
    // the primary connection is live**, and the run is what holds both.
    let periods = the_periods_of(&app);
    let of = what_is_true_of(periods.as_deref(), &server);
    let Ok(started) = started.lock() else {
        return Err(ItCouldNotBeShown::saying(words::THE_LIST_COULD_NOT_BE_READ));
    };
    the_removal_box_for(
        &started,
        of,
        periods.as_deref().map(ThePeriodsThisRun::the_machines_work),
        &server,
    )
}

/// **The removal box as the command composes it**, with the run's facts handed
/// in — so the press a person makes can be driven at the desk. `of` is read
/// before the application's lock is taken (see [`the_removal_of`]); the held
/// work is read under it, the order the list takes.
pub fn the_removal_box_for(
    started: &Started,
    of: WhatIsTrueOfTheServer,
    holds: Option<&crate::servers::the_machines_work::WhatThisRunHolds>,
    server: &str,
) -> Result<TheRemovalBox, ItCouldNotBeShown> {
    let Some(rows) = the_rows(started) else {
        return Err(ItCouldNotBeShown::saying(words::THE_LIST_COULD_NOT_BE_READ));
    };
    let Some(row) = rows
        .iter()
        .find(|row| row.local_server_identifier == server)
    else {
        return Err(ItCouldNotBeShown::saying(words::THIS_SERVER_HAS_BEEN_REMOVED));
    };
    // **L8a: a read that failed is its own answer and never *no*.** The
    // switch-off's own reader is `said_once::the_warning_is_switched_off`,
    // which checks the declaration first — §15 G5's hosting rule — and answers
    // three ways rather than two.
    let already = matches!(
        crate::surface::said_once::the_warning_is_switched_off(
            started,
            THIS_MODULE,
            THE_DISCONNECTED_REMOVAL_WARNING,
        ),
        crate::surface::said_once::WhetherItIsSwitchedOff::Yes
    );
    Ok(the_removal_box_from(
        row,
        &rows,
        of,
        // **`Servers` W19: the period-held last-known queue reading**
        // (`Queue & History` C6), the one the row and the Queue screen read.
        // **Nothing here reaches for a reading and nothing infers one.**
        //
        // **Written for one case and now meeting the other.** This said *no
        // record at all* for every server, on the ground that nothing in the
        // application took a queue reading — true when step 10a drew the box,
        // and untrue from the day step 10c piece 7 built `Servers` Q1 step 7.
        // From then on the row and Queue said *1 rendering · 1 waiting* while
        // this box offered only *Remove*; step 10c's eleventh pass pressed it
        // three times (fault 10). Step 10c's eleventh-pass fix piece 6.
        the_last_known_reading_of(holds, started.record.as_ref(), server),
        already,
    ))
}

/// **`Servers` W19's third column: what the period-held last-known queue
/// reading holds** (`Queue & History` C6), counted as the row and the Queue
/// screen count it — the reading less every job whose ending Smoozy has
/// learned (`Queue & History` C11, [`crate::surface::the_queue::it_has_left_the_queue`]).
/// **So the box, the row and Queue cannot disagree about what is running**,
/// and the box never offers to stop a render Smoozy has already heard end.
/// Nothing is ever added from the local record: a job counts only where the
/// reading holds it. *Rendering* is the reading's own state; every other live
/// job waits, held ones among them, as the Queue counts them.
///
/// **No reading held is `Servers` W19's *no record at all*** — a period with
/// no step 7 yet, or no period, whose reading `Queue & History` C6 drops at
/// its end.
pub(crate) fn the_last_known_reading_of(
    holds: Option<&crate::servers::the_machines_work::WhatThisRunHolds>,
    record: Option<&crate::record::LocalRecord>,
    server: &str,
) -> TheLastKnownReading {
    use smoozy_queue_history::line::QueueState;
    let Some(period) = holds.and_then(|holds| {
        holds
            .every_server()
            .into_iter()
            .find(|(held, _)| held == server)
            .map(|(_, period)| period)
    }) else {
        return TheLastKnownReading::NoRecordAtAll;
    };
    let Some(reading) = period.reading.as_ref() else {
        return TheLastKnownReading::NoRecordAtAll;
    };
    let store = record.map(|record| crate::record::job_records::JobRecords::over(record.connection()));
    let (mut rendering, mut waiting) = (0, 0);
    for job in &reading.value().jobs {
        let held = store.as_ref().and_then(|store| {
            store
                .read(&job.job)
                .ok()
                .flatten()
                .filter(|held| held.server.local_server.as_text() == server)
        });
        if crate::surface::the_queue::it_has_left_the_queue(&job.state, held.as_ref()) {
            continue;
        }
        if matches!(job.state, QueueState::Rendering { .. }) {
            rendering += 1;
        } else {
            waiting += 1;
        }
    }
    TheLastKnownReading::Holds { rendering, waiting }
}

/// **The two checks the Add dialog makes as the user types, neither of which
/// blocks.**
///
/// `Design/Servers § Adding a server`: the possible-duplicate flag *"never
/// blocks, which is what makes the loose comparison the right one"*, and the
/// duplicate-name statement *"must not read like the first"*.
///
/// **H34: every value is validated before anything is touched.** A port that is
/// not a port is answered in place rather than reaching the comparison.
#[tauri::command]
pub fn the_add_is_checked(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    host: String,
    port: String,
    label: String,
) {
    let Ok(started) = started.lock() else {
        return;
    };
    let rows = the_rows(&started).unwrap_or_default();
    let event = the_checks_on(&rows, &host, &port, &label);
    the_queue_is_told(&queue, event);
}

/// **The Edit dialog runs the add's two checks on the edited fields, leaving
/// out the row being edited** — `Specs/Servers § Editing a server`: *"Edited
/// details go through the same duplicate check as a new add … Renaming a
/// server into a name already in use gives the same warning an add does."*
/// Step 10a round 1, fix-here item 19.
///
/// **The same answer also carries `Servers` W34's warning, composed from what
/// is typed** (`Servers` W37: host or port, and username, and nothing else).
/// The page draws it from each answer and removes it when an answer carries
/// none, and **composes no sentence**. Added 2026-09-19, the Gemini catch-up of
/// step 10a, Area 5 (F-1): until then the dialog drew a warning once at
/// opening, from an empty list, so it never showed.
#[tauri::command]
pub fn the_edit_is_checked(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    host: String,
    port: String,
    username: String,
    label: String,
) {
    let Ok(started) = started.lock() else {
        return;
    };
    let rows = the_rows(&started).unwrap_or_default();
    let others: Vec<ServerRow> = rows
        .iter()
        .filter(|row| row.local_server_identifier != server)
        .cloned()
        .collect();
    let warning = rows
        .iter()
        .find(|row| row.local_server_identifier == server)
        .and_then(|row| {
            the_edit_warning_as_typed(
                started.record.as_ref().map(|record| record.connection()),
                row,
                &host,
                &port,
                &username,
            )
        });
    let event = the_edit_warning_said(the_checks_on(&others, &host, &port, &label), warning.as_ref());
    the_queue_is_told(&queue, event);
}

/// **`Servers` W34's warning, on the check's own answer.** Each part travels
/// on its own route — the words as prose, the count and every name as a value
/// (X7) — and the names are indexed, the way the Keys screen's rows are.
pub fn the_edit_warning_said(event: Event, warning: Option<&TheEditWarning>) -> Event {
    let Some(warning) = warning else {
        return event;
    };
    let mut event = event
        .saying("theEditWarning", Said::Prose(warning.lead.clone()))
        .saying("theEditWarningHowMany", Said::Value(warning.how_many.clone()))
        .saying(
            "theEditWarningHaveNotEnded",
            Said::Prose(warning.have_not_ended.clone()),
        )
        .saying(
            "theEditWarningNamedCount",
            value(warning.named.len().to_string()),
        )
        .saying(
            "theEditWarningWhatBecomesOfThem",
            Said::Prose(warning.what_becomes_of_them.clone()),
        );
    for (at, named) in warning.named.iter().enumerate() {
        event = event.saying(
            crate::surface::keys::the_name("theEditWarningNamed", at, ""),
            Said::Value(named.clone()),
        );
    }
    event
}

/// **The add's save** — one of the two acts `App/Servers/src/capability.rs`
/// names as *"performed by callers that do not exist while the screens do
/// not"*.
///
/// **`Servers` §14's *Add* row: three independent units, and the server is
/// always saved whatever the other two do.** This performs the first. The
/// second and third — the connection where a setup option is ticked, and the
/// setup run — are `Servers`' and `Server Setup`'s, and this performs neither:
/// connecting here would clear the remembered setup with nowhere to hand it,
/// which is why *connect and set up* is owed too (this file's head says so).
///
/// **The two ticks travel separately** (step 10a round 1, fix-here item 13):
/// `Servers` B1's bit records *which* options were ticked.
#[tauri::command]
// **The form has five fields, a colour and two choices**, which is Design's
// own shape; a command taking fewer would be one the dialog has to make two
// calls to.
#[allow(clippy::too_many_arguments)]
pub fn a_server_is_added<R: Runtime>(
    app: AppHandle<R>,
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    host: String,
    port: String,
    username: String,
    key_identifier: String,
    label: String,
    colour: String,
    get_ready: bool,
    install_blender: bool,
) {
    let (answer, ticked) = {
        let Ok(started) = started.lock() else {
            return;
        };
        the_add_is_performed_naming_the_row(
            &started,
            &TheAdd {
                host: &host,
                port: &port,
                username: &username,
                key_identifier: &key_identifier,
                label: &label,
                colour: &colour,
                get_ready,
                install_blender,
            },
        )
    };
    // **`Servers` O2: an add with a setup option ticked connects right away to
    // carry the setup out.** **The spine's lock is given back first** (P5 and
    // L4), because opening a period reads the record on the thread the act
    // arrived on.
    //
    // **The connection it opens carries the mark on its control, as *Connect*
    // does** — `Design/Whole app § Anything that isn't instant`: the add's own
    // control is gone with its dialog, so the row's connect control is where
    // the work is seen. Held before the period is begun and let go where
    // nothing began, exactly as the user's own connect. Added 2026-09-28, step
    // 10c's ninth-pass fix piece 2, out of the ninth pass.
    //
    // **The mark goes down, and the period is begun, before the answer is
    // told** — the Creator's notes piece 3, 2026-10-02. The answer closes the
    // dialog and the page draws at once; drawn in the gap before the mark, the
    // row found no connection and `TheUsersConnects::is_underway` let the mark
    // go for good, which reads as nothing happening. *Expected*, not seen: the
    // gap is a record read long.
    if let Some(server) = &ticked {
        let newly = THE_USERS_CONNECTS.pressed(server);
        let went = a_period::a_period_is_opened(&app, server, WhoAsked::AnAddWithASetupOptionTicked);
        if newly && went != HowItWent::APeriodStands {
            THE_USERS_CONNECTS.did_not_begin(server);
        }
    }
    // **The answer names the server where a setup was ticked**, so the page
    // opens it on its Set up tab — `Design/Servers § Adding a server`: *"Add
    // and set up closes the dialog onto the new server's Set up tab."* A plain
    // add names nothing and stays on the list.
    the_queue_is_told(&queue, the_add_is_answered(answer, ticked.as_deref()));
}

/// **The add's answer, naming the server a setup was ticked for** — the
/// Creator's note 3 on 0.1.7.
pub fn the_add_is_answered(answer: TheAnswer, ticked: Option<&str>) -> Event {
    let event = the_answer_event(answer);
    match ticked {
        Some(server) => event
            .saying("server", value(server))
            .saying("itIsSetUp", value("yes")),
        None => event,
    }
}

/// **The edit's save.** `Servers` §14's *Edit* row: the save and the put-back,
/// ordered and independent, and **where the save fails the put-back is not
/// attempted**.
///
/// **Both run whole.** The save is answered here; the put-back is a connection
/// and answers on the period's own events, and **it follows only a connected
/// server's save** (`Servers` E1) — a server with no period standing is saved
/// and left as it was.
#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn an_edit_is_saved<R: Runtime>(
    app: AppHandle<R>,
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    host: String,
    port: String,
    username: String,
    key_identifier: String,
    label: String,
    // **`Servers` D3's fallback folder, as typed in the Edit dialog**; absent
    // from a save that does not carry the field (the rename on the row), and
    // then the row's is kept. It never ends a period (`Servers` E3).
    fallback_folder: Option<String>,
) {
    // **`Servers` E1: saving an edit to a connected server's host, port,
    // username or key ends the period**, and the user's own act of saving is
    // what ends it. **It is ended before the write** — `Servers` S10 item 3 is
    // the save, not the write landing — and a rename or a colour change is
    // `Servers` E3 and ends nothing, which is what `what_changed` decides
    // below.
    let changed = the_connection_details_changed(&started, &server, &host, &port, &username, &key_identifier);
    // **`Servers` E1: only a connected server's save is followed by the
    // put-back**, so it is owed only where a period stood to end. It is opened
    // by the old period's own thread once that period has left the register
    // and this save has said whether it landed — `src/servers/a_period.rs`.
    let the_put_back = if changed {
        the_periods_of(&app).and_then(|periods| periods.the_edits_save_ends_it(&server))
    } else {
        None
    };
    let answer = {
        let Ok(started) = started.lock() else {
            return;
        };
        let store = crate::surface::keys::the_store_for_this_system();
        the_edit_is_performed(
            &started,
            store.as_deref(),
            &server,
            &host,
            &port,
            &username,
            &key_identifier,
            &label,
            fallback_folder.as_deref(),
        )
    };
    // **`Servers` E2: the put-back is `Servers` O1 performed by Smoozy as part of that
    // same act**, and **`Servers` §14's *Edit* row: where the save fails the
    // put-back is not attempted.** The old period's thread is told which.
    let saved = matches!(answer, TheAnswer::ItWasSaved);
    if let Some(the_put_back) = the_put_back {
        the_put_back.the_save_landed(saved);
    }
    the_queue_is_told(&queue, the_answer_event(answer));
    // **The live Telegram pass, fault 11, carried to a rename** (`Telegram`
    // L2 as changed 2026-10-31): a new name reaches every server connected now,
    // at once, since a rename ends no period. Every live connection, because an
    // unnamed server's told name can change with its neighbours'. A save that
    // ended the period is told on the put-back's connection, as before.
    if saved {
        crate::setting_up::the_tellings_are_delivered_on_every_live_connection(&app);
    }
}

/// **Whether this save touches one of the four details a connection is made
/// on** — `Servers` E1, and `Servers` E3 for the two that are not.
///
/// **It is `Servers`' own `what_changed` and nothing is decided here**, which
/// is why the row is read first: the comparison is against what is saved, and
/// the period is ended on the answer that module gives.
fn the_connection_details_changed(
    started: &Mutex<Started>,
    server: &str,
    host: &str,
    port: &str,
    username: &str,
    key_identifier: &str,
) -> bool {
    let Ok(started) = started.lock() else {
        return false;
    };
    let Some(record) = started.record.as_ref() else {
        return false;
    };
    let Ok(Some(row)) = server_rows::one(record.connection(), server) else {
        return false;
    };
    let (Some(host), Some(port), Some(username)) = (
        Host::what_the_user_typed(host),
        the_port_typed(port),
        Username::what_the_user_typed(username),
    ) else {
        // **A value that will not parse never reaches the record**, so nothing
        // is saved and there is no period to end.
        return false;
    };
    let label = row.label.clone().unwrap_or_default();
    what_changed(&row, &host, port, &username, key_identifier, &label)
        .into_iter()
        .any(|changed| smoozy_servers::edits::ends_the_period(changed).is_some())
}

/// **A removal, as the user chose it.**
///
/// - **One that reaches no machine is performed whole** — `Servers` D19, and
///   §14's *Removal, not reaching the machine* row: **atomic entire**. *Leave it
///   running*, the plain confirmation, and the `sv-sc` box's *Remove the server
///   anyway* all land here. **The switch-off is written first and only where
///   the user ticked it**, which is `Servers` W20's one row in the one table.
/// - ***Stop and clear first* reaches the machine** and is
///   [`stop_and_clear_first`], on a thread of its own: `Rendering` K9's wait is
///   up to two minutes, and P5 keeps this command off it.
#[tauri::command]
pub fn a_server_is_removed<R: Runtime>(
    app: AppHandle<R>,
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    reaches_the_machine: bool,
    dont_warn_again: bool,
) {
    if reaches_the_machine {
        stop_and_clear_first_is_begun(&app, &server);
        return;
    }
    let history = crate::surface::the_history::what_history_is_drawn_over(&app);
    let answer = {
        let Ok(started) = started.lock() else {
            return;
        };
        let store = crate::surface::keys::the_store_for_this_system();
        let notifications = app.try_state::<std::sync::Arc<crate::notifications::TheNotificationService>>();
        a_removal_lands(
            &started,
            store.as_deref(),
            Some(&**queue),
            notifications.as_deref().map(|service| &**service),
            &server,
            dont_warn_again,
            Some(&history),
        )
    };
    // **`Servers` S10 item 6: the period ends at the moment the removal's local
    // forget is performed and not at the moment it is asked for.** The forget
    // is `the_removal_is_performed`'s last unit, so the period is ended on the
    // answer that it landed. **A removal the record refused leaves the period
    // standing**, which is right: the row is still there.
    if matches!(answer, TheAnswer::ItWasSaved) {
        if let Some(periods) = the_periods_of(&app) {
            periods.the_row_was_forgotten(&server);
        }
    }
    the_queue_is_told(&queue, the_answer_event(answer));
}

/// **The Edit dialog's *Clear*: the remembered machine password is erased**
/// — `Servers` D18, through `Security and licence` §5.1's three writes, the
/// same erase a removal takes. Step 10a round 1, fix-here item 19.
///
/// **Latent today**: nothing in this application can remember a machine
/// password, so no row carries a reference and the control is never drawn.
#[tauri::command]
pub fn a_remembered_password_is_forgotten(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
) {
    let Ok(started) = started.lock() else {
        return;
    };
    let store = crate::surface::keys::the_store_for_this_system();
    let answer = the_password_is_forgotten(&started, store.as_deref(), &server);
    the_queue_is_told(&queue, the_answer_event(answer));
}

/// **Every act on this screen that reaches a machine, and which of them is
/// built.**
///
/// **Connect and disconnect are built**, and so are **accepting a changed
/// identity** and **`stop-and-clear`** (step 10c's S2), and since step 10c's
/// piece 4 **`connect-and-set-up`** and **`drop-the-setup`** too. **An act
/// name nothing routes is answered rather
/// than met with silence** — Rule 8 makes Smoozy going quiet a fault however
/// the trouble arose, and a control that does nothing twice is the fault F-10
/// repaired in the failure shape.
///
/// **P5: the command is not `async`, so it runs on the thread that draws.**
/// Nothing here connects: the record is read, the period is begun, and the
/// connecting is a thread of `src/servers/a_period.rs`'s own.
#[tauri::command]
pub fn an_act_that_needs_a_connection<R: Runtime>(
    app: AppHandle<R>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    act: String,
    server: String,
) {
    let answer = the_act_is_performed(&app, &act, &server);
    if let Some(answer) = answer {
        the_queue_is_told(&queue, the_answer_event(answer));
    }
}

/// **Which act is which, in one place, so the page's own names and this file's
/// routes cannot drift apart.**
///
/// **Nothing is answered where the act started something.** The period's own
/// publishing is what the screen redraws from, and an answer saying *it was
/// begun* beside it would be the same fact twice.
pub(crate) fn the_act_is_performed<R: Runtime>(
    app: &AppHandle<R>,
    act: &str,
    server: &str,
) -> Option<TheAnswer> {
    match where_an_act_goes(act) {
        WhereAnActGoes::ABeginning => the_users_connect_is_begun(app, server),
        WhereAnActGoes::TheSetupIsDropped => {
            // **Dropped first, in its own durable write, and connected only
            // where the write landed** — a connect begun over a bit still
            // standing would carry the setup out, which is the one thing the
            // user said not to do.
            let dropped = match app.try_state::<Mutex<Started>>() {
                Some(started) => match started.lock() {
                    Ok(started) => the_remembered_setup_is_dropped(&started, server),
                    Err(_) => TheAnswer::TheRecordWouldNotTakeIt,
                },
                None => TheAnswer::TheRecordWouldNotTakeIt,
            };
            if dropped != TheAnswer::ItWasSaved {
                return Some(dropped);
            }
            the_users_connect_is_begun(app, server)
        }
        WhereAnActGoes::AnEnding => {
            the_period_is_ended(app, |periods| periods.the_user_disconnects(server))
        }
        WhereAnActGoes::TheAcceptedIdentity => {
            if a_period::the_changed_identity_is_accepted(app, server) {
                None
            } else {
                Some(TheAnswer::TheRecordWouldNotTakeIt)
            }
        }
        WhereAnActGoes::TheUpdateNow => {
            // **On a thread of its own**: the stop is waited on for up to
            // `Rendering` K9's two minutes, and this runs on the thread that
            // draws (P5). The answer is told when the act ends.
            let of = what_is_true_of(the_periods_of(app).as_deref(), server);
            let (app, server) = (app.clone(), server.to_string());
            std::thread::spawn(move || {
                let did = the_update_now(&server, of, &mut TheRemovalsOwnPath { app: &app });
                the_work_column_is_told_again(&app);
                if let (Some(answer), Some(queue)) =
                    (did.the_answer(), app.try_state::<std::sync::Arc<TheOneQueue>>())
                {
                    the_queue_is_told(&queue, the_answer_event(answer));
                }
            });
            None
        }
        WhereAnActGoes::StopAndClearFirst => {
            stop_and_clear_first_is_begun(app, server);
            None
        }
        WhereAnActGoes::TheUpdateFailureIsCleared => {
            use crate::servers::published::{the_server_carries, what_the_server_carries};
            if matches!(
                what_the_server_carries(server),
                Some(crate::servers::published::WhatSmoozyOnTheServerIs::TheUpdateFailed { .. })
            ) {
                the_server_carries(server, None);
            }
            the_work_column_is_told_again(app);
            None
        }
        WhereAnActGoes::ItIsNotBuilt => Some(TheAnswer::TheActIsNotBuiltYet),
    }
}

// ── V15: the update done anyway ─────────────────────────────────────────────

/// **What `Servers` W21 says a stop and clear came to**, as the removal says
/// it: cleared and stopped, or one of the removal box's own three endings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TheStopAndClearEnded {
    ClearedAndStopped,
    Otherwise(TheRemovalsEnding),
}

/// **`Servers` W21's three endings that are not *cleared and stopped***, which
/// the removal box and V15's act both say — one list, so the two cannot say
/// them differently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheRemovalsEnding {
    CouldNotBegin,
    TheClearWasRefused,
    ItHalfWorked,
}

impl TheRemovalsEnding {
    const ALL: [TheRemovalsEnding; 3] = [
        TheRemovalsEnding::CouldNotBegin,
        TheRemovalsEnding::TheClearWasRefused,
        TheRemovalsEnding::ItHalfWorked,
    ];

    /// The ending as the page keys it, and its two sentences.
    fn said(self) -> TheStopAndClearEnding {
        let (which, lead, rest) = match self {
            TheRemovalsEnding::CouldNotBegin => {
                ("could-not-begin", words::COULD_NOT_BEGIN, words::COULD_NOT_BEGIN_REST)
            }
            TheRemovalsEnding::TheClearWasRefused => {
                ("the-clear-was-refused", words::CLEAR_REFUSED, words::CLEAR_REFUSED_REST)
            }
            TheRemovalsEnding::ItHalfWorked => {
                ("it-half-worked", words::HALF_WORKED, words::HALF_WORKED_REST)
            }
        };
        TheStopAndClearEnding {
            which: Verbatim::new(which),
            lead: Prose::new(lead),
            rest: Prose::new(rest),
        }
    }
}

/// **V15's one end that is not a judgement**, as a seam: `Queue & History`
/// L13's stop-and-clear, sent **through the removal's own code path**. Its one
/// implementation in the running application is [`TheRemovalsOwnPath`]; the
/// tests stand a machine in that records what is asked of it.
pub(crate) trait TheOverridesEnds {
    /// The stop-and-clear pair, and what it came to — or the answer the
    /// removal's path gave in place of sending it.
    fn the_stop_and_clear(&mut self, server: &str) -> Result<TheStopAndClearEnded, TheAnswer>;
}

/// **The removal's own path to a stop and clear**: the pair, through the one
/// sender the removal's *Stop and clear first* uses
/// ([`TheRemovalsOwnEnds::the_pair`]), and what it came to, read the way the
/// removal reads it ([`the_ending_of`]).
///
/// **The pair alone.** V15 keeps the server, so neither of `Servers` W25's
/// other two units is its: Telegram's role off is the removal's (`Servers`
/// W24, *"on it — and on no other"*), and so is the forget.
///
/// *(Until step 10c's S2 this path sent nothing and answered
/// [`TheAnswer::TheActIsNotBuiltYet`], so *Stop jobs and update* said that and
/// nothing was stopped, cleared or updated.)*
pub(crate) struct TheRemovalsOwnPath<'a, R: Runtime> {
    pub app: &'a AppHandle<R>,
}

impl<R: Runtime> TheOverridesEnds for TheRemovalsOwnPath<'_, R> {
    fn the_stop_and_clear(&mut self, server: &str) -> Result<TheStopAndClearEnded, TheAnswer> {
        Ok(the_pair_alone(&mut TheRemovalsOwnEnds { app: self.app }, server))
    }
}

// ── The removal's stop and clear ────────────────────────────────────────────

/// **What the pair came to, as the removal says it** — `Servers` W21's three
/// results, with *could not begin at all* in the drawing's two endings.
///
/// **Where nothing that answers the clear came back and the wait ended, it is
/// said as *could not begin*.** `Design/Servers § When stop and clear does not
/// work` puts a dropped connection there in as many words, and it is the one
/// ending that says nothing happened for the user to go on from. **What is
/// not true of it is the record**: the sender leaves both halves unwritten,
/// because the request may have reached the machine and the machine then
/// carries the pair out with nobody connected (`Queue & History` L13). *This is
/// the one place the words may claim more than Smoozy knows, and it is
/// recorded as S2's residual rather than fixed here: no drawn ending says
/// "Smoozy does not know yet".*
pub(crate) fn the_ending_of(
    came_to: crate::servers::stop_and_clear::WhatThePairCameTo,
) -> TheStopAndClearEnded {
    use crate::servers::stop_and_clear::WhatThePairCameTo as Came;
    match came_to {
        Came::ClearedAndStopped => TheStopAndClearEnded::ClearedAndStopped,
        Came::NothingWasChanged | Came::NothingCameBack => {
            TheStopAndClearEnded::Otherwise(TheRemovalsEnding::CouldNotBegin)
        }
        Came::TheClearWasRefused => TheStopAndClearEnded::Otherwise(TheRemovalsEnding::TheClearWasRefused),
        Came::TheRenderHadNotStopped => TheStopAndClearEnded::Otherwise(TheRemovalsEnding::ItHalfWorked),
    }
}

/// **`Servers` W25's three units, as seams**, so the order is proved at the
/// desk against a stand-in and run in the application over the real ends
/// ([`TheRemovalsOwnEnds`]).
pub(crate) trait TheMachineReachingRemovalsEnds {
    /// **The first unit: `Queue & History` L13's pair**, with `the_units_after`
    /// enumerated in the same §9.4 record before anything is sent.
    fn the_pair(
        &mut self,
        server: &str,
        the_units_after: &[&str],
    ) -> crate::servers::stop_and_clear::ThePairWent;
    /// **The second: Telegram's role off** (`Servers` W24).
    fn telegrams_role_off(&mut self, server: &str) -> smoozy_state::outcomes::UnitOutcome;
    /// One of the act's own units, written into the pair's record.
    fn a_unit_ended(&mut self, record: &Identity, unit: &str, outcome: smoozy_state::outcomes::UnitOutcome);
    /// **What the machine journalled, read forward and applied** — `State,
    /// identity and recovery` V1 step 3, the pass P10d repeats — so the
    /// stopped job's local record learns its ending and each job the clear
    /// removed goes with it (`Queue & History` L23's last bullet), before the
    /// forget leaves them in history under a server that is gone.
    fn what_the_machine_journalled_is_applied(&mut self, server: &str);
    /// **The third: the local forget**, atomic entire.
    fn the_local_forget(&mut self, server: &str) -> TheAnswer;
}

/// **`Servers` W25: the machine-reaching removal's three units, in order.**
///
/// 1. **The clear-and-stop pair** (`Queue & History` L13 to `Queue & History` L17), which
///    `Queue & History` L16 makes one unit. Its §9.4 record also enumerates the
///    second unit, so `Servers` W22's *"the enumeration of units … durable
///    before the first unit is attempted"* holds for the act.
/// 2. **Telegram's role off**, *"an independent unit reported like any
///    other"* — sent whatever the pair came to, and its outcome written into
///    the same record. **Not sent where the record would not write**, since
///    then nothing of the act was attempted.
/// 3. **The local forget, *"only on the user's word"***, after what the
///    machine journalled is applied (see the body): here, only on
///    *cleared and stopped*, which `Servers` W21 says the removal proceeds on.
///    **Anything else keeps the server** and owes `Servers` W21's ending on the
///    list, where the user's word is *Retry* or *Remove
///    anyway* (`Servers` W22: *"the server row is removed only after the user
///    has answered, and only where they answered remove"*).
///
/// **What it was written for, and what it now also meets.** The removal box's
/// *Stop and clear first* was drawn for a server being forgotten, and until
/// this piece its press answered that it was not built. It now also meets the
/// `sv-sc` box's *Retry*, which is the same three units again under fresh
/// identities — `Servers` W21's *"try the remainder again"*, since a clear sent
/// again covers only what arrived meanwhile, which the user asked to have
/// cleared anyway.
pub(crate) fn stop_and_clear_first(
    server: &str,
    ends: &mut dyn TheMachineReachingRemovalsEnds,
) -> TheAnswer {
    use crate::servers::stop_and_clear::TELEGRAMS_ROLE_OFF;
    let went = ends.the_pair(server, &[TELEGRAMS_ROLE_OFF]);
    if let Some(record) = &went.the_record {
        let role = ends.telegrams_role_off(server);
        ends.a_unit_ended(record, TELEGRAMS_ROLE_OFF, role);
    }
    match the_ending_of(went.came_to) {
        TheStopAndClearEnded::ClearedAndStopped => {
            // **The job records outlive the server row** — the forget marks a
            // record whose start was never learned *start never heard* and
            // keeps it in history (the Creator's ruling C1 of 2026-09-18). So
            // what the machine journalled is applied first: the render's
            // *left stopped* reaches its entry, and each cleared job's local
            // record goes, rather than reading as a job Smoozy never heard
            // start.
            ends.what_the_machine_journalled_is_applied(server);
            ends.the_local_forget(server)
        }
        TheStopAndClearEnded::Otherwise(ending) => {
            the_ending_is_owed(server, ending);
            TheAnswer::TheStopAndClearEndingIsOwed
        }
    }
}

/// **`Servers` W22's next presentation, held for the run**: which servers owe
/// the `sv-sc` box, and which ending. **One per server, the latest**; the list
/// takes it when it is next drawn ([`the_owed_ending_is_taken`]).
///
/// **Held for the run and written nowhere else**, and that is the part of
/// `Servers` W22 this piece does not reach: the §9.4 record holds the fact
/// durably, but nothing in the local record ties it to its server or holds
/// the two operation identities, so a run that ends between the act and the
/// list's next drawing loses the saying and keeps the fact. **The table that
/// would tie them is a schema change outside this piece's files**, and is
/// recorded as owed.
pub(crate) static THE_ENDINGS_OWED: Mutex<Vec<(String, TheRemovalsEnding)>> = Mutex::new(Vec::new());

fn the_ending_is_owed(server: &str, ending: TheRemovalsEnding) {
    if let Ok(mut owed) = THE_ENDINGS_OWED.lock() {
        owed.retain(|(held, _)| held != server);
        owed.push((server.to_string(), ending));
    }
}

/// **The first owed ending of a server on the list, taken off the register.**
/// A server's owed ending goes with its row at the forget
/// ([`the_removal_is_performed`]), so none is left for a server that is gone.
pub(crate) fn the_owed_ending_is_taken(rows: &[ServerRow]) -> Option<TheOwedEnding> {
    let mut owed = THE_ENDINGS_OWED.lock().ok()?;
    let at = owed
        .iter()
        .position(|(server, _)| rows.iter().any(|row| &row.local_server_identifier == server))?;
    let (server, ending) = owed.remove(at);
    Some(TheOwedEnding {
        server: Verbatim::new(server),
        which: ending.said().which,
    })
}

/// **`Desktop app and delivery` V15's pair, and only the pair**, through the
/// removal's own first unit — so V15 never reaches Telegram's role or the
/// forget, which are the removal's alone (`Servers` W24, `Servers` W25).
pub(crate) fn the_pair_alone(
    ends: &mut dyn TheMachineReachingRemovalsEnds,
    server: &str,
) -> TheStopAndClearEnded {
    the_ending_of(ends.the_pair(server, &[]).came_to)
}

/// ***Stop and clear first*, begun**: [`stop_and_clear_first`] over the real
/// ends, on a thread of its own, and its answer told when it ends.
fn stop_and_clear_first_is_begun<R: Runtime>(app: &AppHandle<R>, server: &str) {
    let (app, server) = (app.clone(), server.to_string());
    std::thread::spawn(move || {
        let answer = stop_and_clear_first(&server, &mut TheRemovalsOwnEnds { app: &app });
        the_work_column_is_told_again(&app);
        if let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() {
            the_queue_is_told(&queue, the_answer_event(answer));
        }
    });
}

/// **The real ends of `Servers` W25's three units**, over the run's own record
/// and the server's own point.
pub(crate) struct TheRemovalsOwnEnds<'a, R: Runtime> {
    pub app: &'a AppHandle<R>,
}

impl<R: Runtime> TheRemovalsOwnEnds<'_, R> {
    /// **The machine, asked through this server's own point**, with the
    /// package area its row records — or, with no live connection or no
    /// placement recorded, a machine nobody can ask, which is the truth.
    /// **The one composition of it**: the queue's own acts
    /// (`src/servers/the_queues_own_acts.rs`) ask through this too.
    pub(crate) fn the_machine(&self, server: &str) -> Box<dyn crate::servers::the_machines_work::AsksTheSupervisor> {
        let point = the_periods_of(self.app)
            .and_then(|periods| periods.the_sessions_of(server))
            .filter(crate::servers::sessions::ThisServersSessions::a_connection_is_live);
        let package_area = self.app.try_state::<Mutex<Started>>().and_then(|started| {
            let started = started.lock().ok()?;
            let row = server_rows::one(started.record.as_ref()?.connection(), server).ok()??;
            match crate::servers::installation::where_the_package_area_is(&row) {
                crate::servers::installation::WhereThePackageAreaIs::At(area) => Some(area),
                _ => None,
            }
        });
        match (point, package_area) {
            (Some(sessions), Some(package_area)) => {
                Box::new(crate::servers::the_machines_work::OverThisServersPoint { sessions, package_area })
            }
            _ => Box::new(crate::servers::stop_and_clear::NoMachineToAsk),
        }
    }

    /// **The render the period-held queue reading last showed** — carried so
    /// that the machine's outcome names both where they differ, and never as a
    /// target (`Queue & History` L13).
    fn the_render_last_seen(&self, server: &str) -> Option<Identity> {
        let periods = the_periods_of(self.app)?;
        let (_, held) = periods
            .the_machines_work()
            .every_server()
            .into_iter()
            .find(|(held, _)| held == server)?;
        held.reading?.value().jobs.iter().find_map(|job| match &job.state {
            smoozy_queue_history::line::QueueState::Rendering { operation } => Some(operation.clone()),
            _ => None,
        })
    }
}

impl<R: Runtime> TheMachineReachingRemovalsEnds for TheRemovalsOwnEnds<'_, R> {
    fn the_pair(
        &mut self,
        server: &str,
        the_units_after: &[&str],
    ) -> crate::servers::stop_and_clear::ThePairWent {
        use crate::servers::stop_and_clear::{the_pair_is_sent, ThePairWent, TheWait, WhatThePairCameTo, WhatThePairCarries};
        let Some(started) = self.app.try_state::<Mutex<Started>>() else {
            return ThePairWent { came_to: WhatThePairCameTo::NothingWasChanged, the_record: None };
        };
        let carries = WhatThePairCarries {
            group: None,
            the_render_the_desktop_last_saw: self.the_render_last_seen(server),
        };
        let mut machine = self.the_machine(server);
        let record = crate::keys::TheRunsRecord { started: started.inner() };
        let mut keeper = crate::keys::TheOutcomeRecordsLent(&record);
        the_pair_is_sent(&mut keeper, machine.as_mut(), &carries, the_units_after, TheWait::THE_STOPS_OWN)
    }

    fn telegrams_role_off(&mut self, server: &str) -> smoozy_state::outcomes::UnitOutcome {
        crate::servers::stop_and_clear::the_telegram_role_is_switched_off(self.the_machine(server).as_mut())
    }

    fn a_unit_ended(&mut self, the_record: &Identity, unit: &str, outcome: smoozy_state::outcomes::UnitOutcome) {
        use smoozy_state::outcomes::Keeps;
        if let Some(started) = self.app.try_state::<Mutex<Started>>() {
            let record = crate::keys::TheRunsRecord { started: started.inner() };
            let _ = crate::keys::TheOutcomeRecordsLent(&record).a_unit_ended(the_record, unit, outcome);
        }
    }

    fn what_the_machine_journalled_is_applied(&mut self, server: &str) {
        let (Some(started), Some(periods), Some(identity)) = (
            self.app.try_state::<Mutex<Started>>(),
            the_periods_of(self.app),
            Identity::read_back(IdentityKind::LocalServer, server),
        ) else {
            return;
        };
        let record = crate::keys::TheRunsRecord { started: started.inner() };
        let mut machine = self.the_machine(server);
        let _ = crate::servers::the_machines_work::a_read_forward_pass(
            &identity,
            &record,
            machine.as_mut(),
            periods.the_machines_work(),
        );
    }

    fn the_local_forget(&mut self, server: &str) -> TheAnswer {
        let Some(started) = self.app.try_state::<Mutex<Started>>() else {
            return TheAnswer::TheRecordWouldNotTakeIt;
        };
        let queue = self.app.try_state::<std::sync::Arc<TheOneQueue>>();
        let history = crate::surface::the_history::what_history_is_drawn_over(self.app);
        let answer = {
            let Ok(started) = started.lock() else {
                return TheAnswer::TheRecordWouldNotTakeIt;
            };
            let store = crate::surface::keys::the_store_for_this_system();
            let notifications =
                self.app.try_state::<std::sync::Arc<crate::notifications::TheNotificationService>>();
            a_removal_lands(
                &started,
                store.as_deref(),
                queue.as_deref().map(|queue| &**queue),
                notifications.as_deref().map(|service| &**service),
                server,
                false,
                Some(&history),
            )
        };
        // **`Servers` W25: the period ends with this third unit and not
        // before** (`Servers` S10 item 6), since the first two reached the
        // machine over the connection it holds.
        if matches!(answer, TheAnswer::ItWasSaved) {
            if let Some(periods) = the_periods_of(self.app) {
                periods.the_row_was_forgotten(server);
            }
        }
        answer
    }
}

/// **What V15's act did.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum WhatTheUpdateNowDid {
    /// **Not offered** — the update no longer waits, or the connection is not
    /// live. Nothing is sent; the screen redraws from what is true.
    NotOffered,
    /// **The removal's path answered in place of sending**, and the answer is
    /// said. Nothing is stopped and no update is sent.
    TheRemovalsPathAnswered(TheAnswer),
    /// **Anything but *cleared and stopped*** — the update stays waiting, and
    /// the result is said as `Servers` W21 says it for a removal. Nothing is
    /// retried on its own.
    ItDidNotFullyWork(TheRemovalsEnding),
    /// **Cleared and stopped.** The update is V11's again: `Desktop app and
    /// delivery` V12 takes it at the next queue reading, which is the one that
    /// shows the work gone (see [`the_update_now`]).
    ClearedAndStopped,
}

impl WhatTheUpdateNowDid {
    fn the_answer(self) -> Option<TheAnswer> {
        match self {
            WhatTheUpdateNowDid::NotOffered => None,
            WhatTheUpdateNowDid::TheRemovalsPathAnswered(answer) => Some(answer),
            WhatTheUpdateNowDid::ItDidNotFullyWork(which) => {
                Some(TheAnswer::TheStopAndClearDidNotFullyWork(which))
            }
            WhatTheUpdateNowDid::ClearedAndStopped => Some(TheAnswer::TheUpdateNowWasTaken),
        }
    }
}

/// **`Desktop app and delivery` V15: the waiting update, done anyway.** Added
/// 2026-09-28, step 10c's transfer wave piece 9.
///
/// **The act sends `Queue & History` L13's stop-and-clear** — the request a
/// removal already sends (`Servers` W18) — **and then V11 runs again once both
/// outcomes are learned.** **Nothing is stored**: the next time a newer
/// release waits, it waits again. Where the result is anything but *cleared
/// and stopped* (`Servers` W21), the update stays waiting and the result is
/// said; nothing is retried on its own.
///
/// **Where V11 runs again, and why it is not called from here.** V11's work
/// reading is *"the one already taken"*, and the one this run holds was taken
/// before the clear, so it still shows the jobs: asked now, V11 would decide
/// that the update waits. **The reading that shows the work gone is V12's
/// moment** — the queue reading P10d repeats while this list is on screen —
/// and at it `src/servers/the_machines_work.rs` takes step 9 again, unchanged,
/// because *the update waits* is still what the server carries. So the
/// update follows the stop and clear by the transfer wave piece 8's own code
/// and nothing here repeats it.
///
/// **What it was written for, and what it now also meets.** The removal's
/// stop and clear was written for a server being forgotten. It now also
/// meets a server being kept, whose jobs are ended so that Smoozy on it can
/// be replaced without ending a render under it.
pub(crate) fn the_update_now(
    server: &str,
    of: WhatIsTrueOfTheServer,
    ends: &mut dyn TheOverridesEnds,
) -> WhatTheUpdateNowDid {
    let carries = crate::servers::published::what_the_server_carries(server);
    if !the_update_now_is_offered(carries.as_ref(), of) {
        return WhatTheUpdateNowDid::NotOffered;
    }
    match ends.the_stop_and_clear(server) {
        Err(answer) => WhatTheUpdateNowDid::TheRemovalsPathAnswered(answer),
        Ok(TheStopAndClearEnded::Otherwise(which)) => WhatTheUpdateNowDid::ItDidNotFullyWork(which),
        Ok(TheStopAndClearEnded::ClearedAndStopped) => WhatTheUpdateNowDid::ClearedAndStopped,
    }
}

/// **Where each of this screen's acts goes**, as a value, so that which acts
/// are built is one fact in one place and a test can read it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WhereAnActGoes {
    /// **`Servers` S5's first opening act, `Servers` O1: the user's connect
    /// action on a saved server.**
    ABeginning,
    /// **`Servers` S10 item 1: the user disconnects.** Available for as long as
    /// a period stands, and **taking it stops the repair**.
    AnEnding,
    /// **`Servers` Q6's accept path.**
    TheAcceptedIdentity,
    /// **`Desktop app and delivery` V15: *Stop jobs and update*.**
    TheUpdateNow,
    /// **The failed update line's cross** — V13's *the update failed*,
    /// withdrawn by hand.
    TheUpdateFailureIsCleared,
    /// **`Servers` W25's machine-reaching removal** — the `sv-sc` box's *Try
    /// again*. Step 10c's S2.
    StopAndClearFirst,
    /// **`Servers` B5 item 2: the user drops the remembered setup**, and then
    /// connects without it — the menu item's own words, *Connect without
    /// setup*. Step 10c's piece 4.
    TheSetupIsDropped,
    /// **An act name nothing routes.** Every act the drawing carries has a
    /// route since step 10c's piece 4, and the pinning test holds it so.
    ItIsNotBuilt,
}

pub(crate) fn where_an_act_goes(act: &str) -> WhereAnActGoes {
    match act {
        // **`Servers` B6: the connect action carries the remembered setup**,
        // and the user's ordinary connect is that action — the period takes
        // the setup at `Servers` B2's instant and hands it to `Server Setup` at
        // `Servers` Q1 step 8 (`src/servers/a_period.rs`). Step 10c's piece 4.
        "connect" | "connect-and-set-up" => WhereAnActGoes::ABeginning,
        "drop-the-setup" => WhereAnActGoes::TheSetupIsDropped,
        "disconnect" => WhereAnActGoes::AnEnding,
        "accept-the-identity" => WhereAnActGoes::TheAcceptedIdentity,
        "stop-and-clear" => WhereAnActGoes::StopAndClearFirst,
        THE_UPDATE_NOW => WhereAnActGoes::TheUpdateNow,
        THE_UPDATE_FAILURE_IS_CLEARED => WhereAnActGoes::TheUpdateFailureIsCleared,
        _ => WhereAnActGoes::ItIsNotBuilt,
    }
}

/// **What opening a period answers, told apart by its cause** — step 10b's
/// first audit round, item 14: all three used to be told *"This server has been
/// removed."*
///
/// - **A period already stands** (`Servers` S2): no answer. The screen redraws
///   from what the run holds, which is what a second press should meet.
/// - **The row is gone**: the removed sentence, kept for the one case it is
///   true of.
/// - **The record would not read**: the list's own *could not be read* words.
pub(crate) fn the_beginning_is_answered(went: HowItWent) -> Option<TheAnswer> {
    match went {
        HowItWent::APeriodStands | HowItWent::OneAlreadyStands => None,
        HowItWent::TheServerIsGone => Some(TheAnswer::TheServerIsGone),
        HowItWent::TheRecordWouldNotRead => Some(TheAnswer::TheListCouldNotBeRead),
    }
}

/// **`Servers` O1: the user's connect action on a saved server** — *Connect*,
/// *Connect and set up*, and the connect that follows a dropped setup.
fn the_users_connect_is_begun<R: Runtime>(app: &AppHandle<R>, server: &str) -> Option<TheAnswer> {
    // **Held before the period is begun**, so no drawing made once its
    // thread is running can miss it, and let go where nothing began.
    let newly = THE_USERS_CONNECTS.pressed(server);
    let went = a_period::a_period_is_opened(app, server, WhoAsked::TheUsersConnectAction);
    if newly && went != HowItWent::APeriodStands {
        THE_USERS_CONNECTS.did_not_begin(server);
    }
    the_beginning_is_answered(went)
}

/// **`Servers` B5 item 2: the user drops the remembered setup without carrying
/// it out** — one durable write that clears the bit and touches nothing else
/// on the row. Step 10c's piece 4.
///
/// **A row holding no setup is answered as saved**: there is nothing to drop,
/// and the connect that follows is the plain one either way. **A server
/// another act removed is said as that**, never as a refused write.
pub(crate) fn the_remembered_setup_is_dropped(started: &Started, server: &str) -> TheAnswer {
    let Some(record) = started.record.as_ref() else {
        return TheAnswer::TheRecordWouldNotTakeIt;
    };
    let mut row = match server_rows::one(record.connection(), server) {
        Ok(Some(row)) => row,
        Ok(None) => return TheAnswer::TheServerIsGone,
        Err(_) => return TheAnswer::TheRecordWouldNotTakeIt,
    };
    if row.remembered_setup.take().is_none() {
        return TheAnswer::ItWasSaved;
    }
    match server_rows::write(record.connection(), &row) {
        Ok(()) => TheAnswer::ItWasSaved,
        Err(_) => TheAnswer::TheRecordWouldNotTakeIt,
    }
}

/// End a standing period.
///
/// **Where none stood to end, nothing is answered** (`Servers` N13: a server no
/// period stands for is repaired by nothing, and there is nothing to end) — a
/// *Disconnect* pressed just as a failed opening attempt ended is the ordinary
/// case, and the removed sentence it used to get was false. The page asks for
/// the screen again after every act, so it redraws what is true.
fn the_period_is_ended<R: Runtime>(
    app: &AppHandle<R>,
    ending: impl FnOnce(&ThePeriodsThisRun) -> bool,
) -> Option<TheAnswer> {
    let periods = app.try_state::<std::sync::Arc<ThePeriodsThisRun>>()?;
    let _ = ending(periods.inner());
    None
}

/// **What a period that has ended says to the user, and mostly it says
/// nothing.** The choice is [`what_an_ended_period_says`]; this tells it.
///
/// Called from the thread that drove the period, once, after
/// `Servers`' driving half has answered.
///
/// **The answer names the server it is about** (the Creator's note 14 on the
/// test copy, 2026-10-03). Until then it named none, because it was an answer
/// to the act and the act's screen was the one showing — and *Add and set up*
/// takes the user to the new server's Set up tab, where nothing heard it. The
/// page keeps it for that server, above the list and on the Set up card.
pub(crate) fn the_period_ended<R: Runtime>(
    app: &AppHandle<R>,
    server: &str,
    ended: smoozy_servers::driving::HowTheDrivingEnded,
    the_box_was_raised: bool,
    the_passphrase_could_not_be_used: Option<&'static str>,
) {
    // **`Queue & History` C6: the period's end dropped its reading**, so the
    // work column moved to the dash without a reading, and it is told so
    // first, whichever ending follows. Otherwise the next period's first
    // reading says *Idle*, which is what the column was last told, and the
    // dash stands (step 10c's tenth pass, fault 4). Step 10c's tenth-pass fix
    // piece 2.
    the_work_column_is_told_again(app);
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    let Some(answer) = what_an_ended_period_says(
        a_period::what_the_driving_left(ended),
        the_box_was_raised,
        the_passphrase_could_not_be_used,
    ) else {
        return;
    };
    the_queue_is_told(
        &queue,
        the_answer_about(answer, &the_server_as_the_list_names_it(app, server)),
    );
}

/// **An answer said from outside this screen's own commands** — the
/// passphrase box's *Connect* is the connection's (`Preparation/Design/Keys.md`),
/// so what it answers is this screen's answer.
pub(crate) fn the_answer_is_said<R: Runtime>(app: &AppHandle<R>, answer: TheAnswer) {
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    the_queue_is_told(&queue, the_answer_event(answer));
}

/// **A connect that could not begin its work, or a put-back that could not
/// begin, says so** rather than leaving the server disconnected in silence —
/// step 10b's first audit round, items 5 and 15. It is the claimless sentence,
/// because nothing is known about why.
///
/// **It names the server**, as a period's own ending does (the Creator's note
/// 14 on the test copy), so it is kept for that server where the user is.
pub(crate) fn the_connection_was_not_made<R: Runtime>(app: &AppHandle<R>, server: &str) {
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    the_queue_is_told(
        &queue,
        the_answer_about(
            TheAnswer::TheConnectionWasNotMade,
            &the_server_as_the_list_names_it(app, server),
        ),
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Reading what there is
// ════════════════════════════════════════════════════════════════════════════

/// **The server rows, or nothing where the read did not answer** — and the
/// two are never the same answer (L8a; step 10a round 1, fix-here item 4).
fn the_rows(started: &Started) -> Option<Vec<ServerRow>> {
    started
        .record
        .as_ref()
        .and_then(|record| server_rows::all(record.connection()).ok())
}

/// **The key rows, or nothing where the read did not answer.** A key read that
/// did not answer marks nothing: no mark is derived from a read that did not
/// happen.
fn the_key_rows(started: &Started) -> Option<Vec<smoozy_keys::rows::KeyRow>> {
    started
        .record
        .as_ref()
        .and_then(|record| key_rows::all(record.connection()).ok())
}

/// **`Keys`' verdict on each key the rows name, and `Keys` G6, asked once for
/// the whole ask and never per row** — `Servers` P12 to `Servers` P14.
///
/// **One reading of the computer is shared by every key in the ask**, so the
/// tools probe runs once (`crate::keys::readings::OnThisComputer` holds its
/// answer for its own life). It is a filesystem read and a tool run, so it is
/// never called with the record's lock held and never on the drawing thread —
/// see [`what_keys_say_off_the_drawing_thread`].
pub fn what_keys_say(rows: &[ServerRow], keys: &[smoozy_keys::rows::KeyRow]) -> WhatKeysSaid {
    // **Smoozy's own temporary area and never a fallback** — step 10b's first
    // audit round, item 16: this fell back to an empty path, which is the
    // working directory. **Where the area cannot be named nothing is read**:
    // it sits under the same location as the local record, so the list itself
    // could not have been read either.
    let Some(area) = crate::platform::locations::smoozys_own_temporary_area() else {
        return WhatKeysSaid::default();
    };
    let readings = crate::keys::readings::OnThisComputer::over(area);
    let the_tools_are_installed = usable::Readings::there_is_a_generator(&readings);
    let mut verdicts = Vec::new();
    let mut the_passphrase_is_not_available = Vec::new();
    if the_tools_are_installed {
        for key in keys {
            let identifier = key.identifier.as_text();
            let named = rows
                .iter()
                .any(|row| row.key.local_key_identifier == identifier);
            if !named || verdicts.iter().any(|(held, _)| *held == identifier) {
                continue;
            }
            let checked = usable::check(key, &readings);
            // **`Servers` P1's second question, as `Keys` answers it**: a key
            // that needs a passphrase, with none available without asking.
            if checked.protected == Some(true)
                && !usable::a_passphrase_is_available_without_asking(key)
            {
                the_passphrase_is_not_available.push(identifier.clone());
            }
            verdicts.push((identifier, checked.verdict));
        }
    }
    WhatKeysSaid {
        the_tools_are_installed,
        verdicts,
        the_passphrase_is_not_available,
        the_store_standing: None,
    }
}

/// [`what_keys_say`], on the application's own blocking pool rather than on
/// the thread that draws (`Desktop app and delivery` P4 and P5). **Where the
/// key rows did not read, nothing is asked** and no mark is derived.
async fn what_keys_say_off_the_drawing_thread(
    rows: &[ServerRow],
    keys: Option<&[smoozy_keys::rows::KeyRow]>,
) -> Option<WhatKeysSaid> {
    let keys = keys?.to_vec();
    let rows = rows.to_vec();
    tauri::async_runtime::spawn_blocking(move || what_keys_say(&rows, &keys))
        .await
        .ok()
}

/// **The servers whose *Connect* the user pressed, while that connect is still
/// being made** — so the control carries Design's *on the control* mark for
/// exactly that long. Added 2026-09-27, step 10c's eighth-pass fix piece 5.
/// **An *Add* with a setup option ticked is held the same way**, since the
/// connection it opens is the user's act too (`Servers` O2) — added
/// 2026-09-28, the ninth-pass fix piece 2.
///
/// **`Design/Whole app § Anything that isn't instant shows that it's
/// underway` names *Connect* as the rule's own example of *on the control***,
/// and on the eighth pass it showed nothing while it worked. **`Servers` K6
/// cannot say it**: while the opening attempt runs, a period stands and the
/// primary connection is not live, which is exactly what a repair reads — and
/// a repairing server's control keeps its amber and takes no mark (*one thing
/// never carries two of them*). What tells the two apart is who began the
/// period and whether its connection has been live since. The first is known
/// here, at the act; **the second is read off `Servers`' own facts at each
/// drawing**: a connect is finished the moment its primary connection is live,
/// or the moment no period stands — **a period whose opening attempt fails
/// ends at once** (`Servers` S10 item 5 and `Servers` Q11, as
/// `App/Servers/src/driving.rs` drives them), so no repair ever follows a
/// connect that never connected.
///
/// **Run-held and nothing else** (`Servers` S3). **What it leaves, stated so
/// nobody rediscovers it**: the entry is let go when a drawing sees the
/// connection live, and every change of `Servers`' value asks this screen
/// again — so only a connection that goes live and drops again with no
/// drawing between them would show the mark over its repair instead of the
/// amber, and it is gone at the next drawing after the drop's own.
pub(crate) struct TheUsersConnects(Mutex<std::collections::BTreeSet<String>>);

pub(crate) static THE_USERS_CONNECTS: TheUsersConnects =
    TheUsersConnects(Mutex::new(std::collections::BTreeSet::new()));

impl TheUsersConnects {
    /// The user pressed *Connect* on this server. **Answers whether it was not
    /// already held**, so a press that begins nothing lets go only of what it
    /// put down itself.
    pub(crate) fn pressed(&self, server: &str) -> bool {
        self.0
            .lock()
            .map(|mut held| held.insert(server.to_string()))
            .unwrap_or(false)
    }

    /// The press began no period, so there is no connect to be under way.
    pub(crate) fn did_not_begin(&self, server: &str) {
        if let Ok(mut held) = self.0.lock() {
            held.remove(server);
        }
    }

    /// **Whether the user's connect is being made now**, read against what is
    /// true of the server — and let go of the moment it is not.
    pub(crate) fn is_underway(&self, server: &str, of: WhatIsTrueOfTheServer) -> bool {
        let Ok(mut held) = self.0.lock() else {
            return false;
        };
        if !of.a_period_stands || of.the_primary_connection_is_live {
            held.remove(server);
            return false;
        }
        held.contains(server)
    }
}

/// **The state of a server with no period.**
///
/// **`Servers` D22: nothing durable records that a server was connected**, so
/// on a fresh launch there is no period and every server's observed value is
/// *disconnected*. **It is a reading of what is true and not a default**: a
/// server the run holds no period for is a server no period stands for.
///
/// **What the run does hold is `src/servers/a_period.rs`'s**, and
/// [`what_is_true_of`] is how this screen asks. This is the answer where there
/// is nothing to ask — a caller with no register, which is every test of the
/// drawing and the rail's own read.
pub(crate) fn what_is_true_of_a_server_with_no_period() -> WhatIsTrueOfTheServer {
    WhatIsTrueOfTheServer {
        a_period_stands: false,
        the_primary_connection_is_live: false,
        a_run_of_attempts_is_in_flight: false,
        the_restart_mark_stands: false,
    }
}

/// **Every period this run holds, where the run has the register.**
///
/// **A caller that cannot reach it draws a server with no period**, which is
/// what is true of one: nothing has begun a period it cannot see.
pub(crate) fn the_periods_of<R: Runtime>(
    app: &AppHandle<R>,
) -> Option<std::sync::Arc<ThePeriodsThisRun>> {
    Some(app.try_state::<std::sync::Arc<ThePeriodsThisRun>>()?.inner().clone())
}

/// **What is true of one server, asked of the run that holds the periods.**
///
/// **`Servers` S2 is why it is per server**: a period is per server and one
/// server's period touches no other's, so the question is asked once for each
/// row rather than once for the list.
pub(crate) fn what_is_true_of(
    periods: Option<&ThePeriodsThisRun>,
    server: &str,
) -> WhatIsTrueOfTheServer {
    match periods {
        Some(periods) => periods.what_is_true_of(server),
        None => what_is_true_of_a_server_with_no_period(),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Composing the screen
// ════════════════════════════════════════════════════════════════════════════

/// The screen, with the rows handed in — **so the whole of it can be tested**.
/// The shell's own managed state has no constructor outside a running
/// application, which is the same split `panes::a_drag_ended_on` and
/// `the_rail::the_rail_from` already use.
///
/// **`said` is what `Keys` answered about the keys the rows name, or nothing
/// where it was not asked** — the server's own head asks nothing of `Keys`,
/// because it draws neither the key mark nor the tools statement, and a mark
/// derived from nothing is never drawn.
pub fn the_screen_from(
    rows: &[ServerRow],
    keys: &[smoozy_keys::rows::KeyRow],
    said: Option<&WhatKeysSaid>,
    periods: Option<&ThePeriodsThisRun>,
    work: &WhatTheServersAreDoing,
) -> TheAllServersScreen {
    let the_tools_are_missing = said.is_some_and(|said| !said.the_tools_are_installed);
    let connected = rows
        .iter()
        .filter(|row| {
            matches!(
                condition::presentation(what_is_true_of(periods, &row.local_server_identifier)),
                Presentation::Connected
            )
        })
        .count();

    TheAllServersScreen {
        the_screens_own_name: Prose::new(words::THE_SCREEN),
        add_a_server: Prose::new(words::ADD_A_SERVER),
        // **Said once and never per row**, and only where the tools are absent.
        the_tools_statement: the_tools_are_missing.then(|| TheToolsStatement {
            lead: Prose::new(words::NO_TOOLS_LEAD),
            until: Prose::new(words::NO_TOOLS_UNTIL),
        }),
        rows: rows
            .iter()
            .map(|row| {
                a_row(
                    row,
                    rows,
                    keys,
                    what_is_true_of(periods, &row.local_server_identifier),
                    said,
                    periods,
                    work,
                )
            })
            .collect(),
        footer: TheFooter {
            how_many: Words::after(
                rows.len().to_string(),
                if rows.len() == 1 {
                    words::ONE_SERVER
                } else {
                    words::SERVERS
                },
            ),
            how_many_connected: Words::after(connected.to_string(), words::CONNECTED_COUNT),
        },
        guidance: the_guidance(),
        menus: the_five_menus(),
        the_boxes_a_connection_raises: the_boxes(),
        // **Handed over by the command alone**, off the run's register, so a
        // composition for any other reason takes nothing from it.
        the_stop_and_clear_ending: None,
        // `Design/Servers § Keyboard shortcuts`: **Return opens the chosen
        // server, the delete key removes it, and nothing else is bound.**
        the_keys_that_are_bound: vec![Verbatim::new("Enter"), Verbatim::new("Delete")],
        the_subject_kind: Verbatim::new(THE_SUBJECT_A_ROW_IS_ABOUT),
    }
}

/// One row.
fn a_row(
    row: &ServerRow,
    all: &[ServerRow],
    keys: &[smoozy_keys::rows::KeyRow],
    of: WhatIsTrueOfTheServer,
    said: Option<&WhatKeysSaid>,
    periods: Option<&ThePeriodsThisRun>,
    work: &WhatTheServersAreDoing,
) -> AServerRow {
    let needing_a_new_key = the_key_mark(row, keys, said);
    // **Whether the live connection's check found no Smoozy of this
    // installation on the machine** — `State, identity and recovery` Y9's
    // absent or different branch, where `Queue & History` A1 reads nothing on
    // the machine as Smoozy's at all. Only then does a connected server with
    // no reading read *Idle*. Step 10c's eleventh-pass fix piece 6.
    let nothing_of_smoozys_is_there = periods
        .and_then(|periods| periods.what_the_last_connection_read(&row.local_server_identifier))
        .is_some_and(|read| {
            read.on_the_connection_that_is_live
                && matches!(
                    read.check,
                    InstallationCheck::Absent | InstallationCheck::PresentAndDifferent
                )
        });
    let the_connect_is_underway = THE_USERS_CONNECTS.is_underway(&row.local_server_identifier, of);
    // **`Desktop app and delivery` V13's condition, where this server carries
    // one** — raised and withdrawn by `src/servers/published.rs` and drawn
    // here. Step 10c's transfer wave piece 9.
    let carries = crate::servers::published::what_the_server_carries(&row.local_server_identifier);
    let the_repair_is_due = periods.and_then(|periods| {
        match periods.the_next_attempt_due_for(&row.local_server_identifier) {
            smoozy_servers::repair::NextAttemptDue::At(moment) => Some(moment),
            smoozy_servers::repair::NextAttemptDue::Absent => None,
        }
    });
    AServerRow {
        server: Verbatim::new(row.local_server_identifier.clone()),
        name: Verbatim::new(what_a_server_is_called_among(row, all)),
        address: Verbatim::new(the_address_of(row)),
        badge: the_badge(row.colour.as_deref()),
        // **`Design/Servers`: where a failure and the key mark are both true,
        // the failure takes the line.** The failure is drawn by the one shape
        // and arrives on its own event, so this row carries the key mark and
        // the page gives the line to the failure where one is standing.
        // **The gone key takes the line from `Keys` S10's mark**: it is
        // `Servers` P1's first question, and a key that is gone needs no
        // passphrase.
        the_key_mark: if needing_a_new_key {
            Some(Words::plain(words::THE_KEY_IS_GONE))
        } else {
            the_passphrase_mark(row, said)
        },
        the_sentence: periods
            .is_some_and(|periods| periods.the_row_carries_the_sentence(&row.local_server_identifier))
            .then(|| Prose::new(crate::surface::keys::NOWHERE_TO_KEEP_THEM)),
        // **`Server Setup` reads this state fresh from the machine on every
        // connection**, so a server with no connection says nothing about a
        // setup at all — `Design/Servers § A row`. **A connection is made, but
        // nothing on it reads the setup's state yet**: `Server Setup`'s reading
        // is not called from a period, so this is absent rather than guessed.
        // *(Corrected 2026-09-18, step 10b's audit round's loose ends, L5. This
        // said nothing connects, which step 10b made untrue.)*
        the_setup_line: None,
        the_update_line: the_update_line(
            carries.as_ref(),
            of,
            &what_a_server_is_called_among(row, all),
            work.get(&row.local_server_identifier),
        ),
        // **While the user's own connect is being made the column takes the
        // dash**, not *Reconnecting…*: nothing has been reached to read, and
        // the word says a repair that is not happening. Design gives the dash
        // for *"Smoozy cannot say what the server is doing"*. Added
        // 2026-09-27, eighth-pass fix piece 5.
        //
        // **Where the server is connected and Smoozy's service on it does not
        // answer, the column takes the dash rather than *Idle*** —
        // `Design/Servers § A row`: *"since no reading of its work stands"*.
        // *Idle* is a reading of the machine's queue, and on `State, identity
        // and recovery` Y9's fourth branch none is taken (`Servers` Q19,
        // `Servers` Q1 step 7). Added 2026-09-27, step 10c's eighth-pass fix
        // piece 6, out of the seventh pass on RunPod.
        //
        // **Otherwise the column reads the server's work**: the held queue
        // reading less every job whose ending Smoozy has learned, composed by
        // `crate::surface::the_queue::what_the_servers_are_doing` — the same
        // composition the Queue screen draws. Step 10c's ninth-pass fix piece
        // 2.
        the_work: the_setup_takes_the_column(if the_connect_is_underway {
            vec![the_dash()]
        } else if condition::presentation(of) == Presentation::Connected
            && periods.is_some_and(|periods| {
                periods.smoozy_does_not_answer_on(&row.local_server_identifier)
            })
        {
            vec![the_dash()]
        } else if condition::presentation(of) == Presentation::Connected
            && the_work_takes_the_dash_under(carries.as_ref())
        {
            // **`Design/Servers § A row`: the work column takes the dash while
            // the update runs, too old or too new** — the service is being
            // replaced, or speaks no protocol this build speaks, so no reading
            // of its work stands. Step 10c's transfer wave piece 9.
            vec![the_dash()]
        } else {
            the_work_column(of, work.get(&row.local_server_identifier), nothing_of_smoozys_is_there)
        }, a_setup_runs_on(&row.local_server_identifier)),
        the_next_attempt: the_next_attempt(of, the_repair_is_due),
        the_hardware: the_hardware_as_the_run_knows_it(
            row.remembered_hardware_reading.as_ref(),
            periods.and_then(|periods| periods.what_the_last_connection_read(&row.local_server_identifier)),
            of,
        ),
        the_connection: if the_connect_is_underway {
            // **The label the user pressed**, held under the mark so the
            // control keeps its width; the act is the standing period's own.
            AControl::live(words::CONNECT, "disconnect")
        } else {
            the_connect_control(row, of, needing_a_new_key)
        },
        the_connect_is_underway,
        the_connection_is_coming_back: !the_connect_is_underway
            && matches!(
                condition::presentation(of),
                Presentation::Repairing | Presentation::Restarting
            ),
        the_menu: Verbatim::new(which_menu(row, of, needing_a_new_key)),
        more: Prose::new(words::MORE),
    }
}

/// **The name, or `username@host` where the server has none** — Design's own
/// fallback, and the same one `the_rail` uses. **Composed out of two values and
/// never out of prose**, so X7's rule that the point never touches a host or a
/// username is kept by there being no prose on this path at all.
pub(crate) fn what_a_server_is_called(row: &ServerRow) -> String {
    match row.label.as_deref().map(str::trim) {
        Some(label) if !label.is_empty() => label.to_string(),
        _ => format!("{}@{}", row.username.as_str(), row.host.as_str()),
    }
}

/// **The one naming the rail, the list, the head, `Keys`' *Used by* line and
/// the removal heading all take** — `Specs/Servers`: *"The port is shown as
/// well wherever that is the only thing telling two unnamed entries apart."*
///
/// **`:port` is added only to an unnamed server whose `username@host` another
/// unnamed server shares**, so the ordinary list reads exactly as before. Two
/// values composed into one value, with no prose on the path (X7). Step 10a
/// round 1, fix-here item 21.
pub(crate) fn what_a_server_is_called_among(row: &ServerRow, all: &[ServerRow]) -> String {
    let called = what_a_server_is_called(row);
    let unnamed = |one: &ServerRow| {
        one.label
            .as_deref()
            .map(str::trim)
            .is_none_or(str::is_empty)
    };
    if !unnamed(row) {
        return called;
    }
    let shared = all.iter().any(|other| {
        other.local_server_identifier != row.local_server_identifier
            && unnamed(other)
            && other.username == row.username
            && other.host == row.host
    });
    if shared {
        format!("{called}:{}", row.port.number())
    } else {
        called
    }
}

/// **The address, in the fixed-width face.** A value entire: a username, a host
/// and a port, none of which the point may touch (X7).
fn the_address_of(row: &ServerRow) -> String {
    format!(
        "{}@{}:{}",
        row.username.as_str(),
        row.host.as_str(),
        row.port.number()
    )
}

/// The stored colour, where it is one of the drawing's eight.
fn the_badge(colour: Option<&str>) -> Option<&'static str> {
    let colour = colour?;
    crate::surface::the_rail::THE_EIGHT_BADGES
        .iter()
        .find(|badge| **badge == colour)
        .copied()
}

/// **`Servers` P12 to `Servers` P16, at the list draw: one local read per server, and
/// nothing polls.**
///
/// **`Servers` P13: the mark is a derivation and not a stored state**, computed at each
/// draw from `Keys`' answer about the row's key reference — **the pair of
/// `Servers` P1a**, the identifier *and* the fingerprint, because `Keys`' replacement act
/// keeps the identifier and changes the fingerprint.
///
/// **`Keys` U6a: where the tool a reading needs is absent, no row carries a
/// verdict at all**, and the cause is the one statement above the list. So a
/// computer with no SSH tools marks nothing.
///
/// **And `Servers` P1 question 1 is `Keys`' own verdict on the key**, read with
/// the pair (step 10a round 1, fix-here item 14): a key file that was moved,
/// deleted or had its permissions opened is *unusable* by `Keys` U6, and the
/// server that uses it needs a new key. **Where `Keys` was not asked, nothing
/// is marked** — a mark derived from nothing is never drawn.
fn the_key_mark(
    row: &ServerRow,
    keys: &[smoozy_keys::rows::KeyRow],
    said: Option<&WhatKeysSaid>,
) -> bool {
    let Some(said) = said else {
        return false;
    };
    if !said.the_tools_are_installed {
        return false;
    }
    let Some(key) = keys
        .iter()
        .find(|key| key.identifier.as_text() == row.key.local_key_identifier)
    else {
        // The row names a key row that is not there any more, which is exactly
        // what `Servers` P16's fingerprint is held for: the server wants a key Smoozy no
        // longer has.
        return true;
    };
    // **`Servers` P1a: the reference is the pair.** A row whose fingerprint no longer
    // matches names a key that left and was replaced, and an identifier-only
    // ask would go on answering yes.
    let the_same_key = key.fingerprint.as_ref().is_some_and(|reading| {
        smoozy_servers::key_moments::the_reference_still_names_the_same_key(
            &row.key,
            reading.value.as_str(),
        )
    });
    let usable = matches!(
        said.the_verdict_on(&row.key.local_key_identifier),
        Some(usable::Verdict::Usable)
    );
    !(the_same_key && usable)
}

/// **`Keys` S10's mark, from `Keys`' own two functions** — step 10b's first
/// audit round, item 6: built and tested in `App/Keys/`, and nothing called it.
///
/// **`Servers` P3 leaves a server marked with what is wrong** where P1's second
/// question is *no*: the key needs a passphrase and none is available without
/// asking, so no connection Smoozy makes on its own is attempted. **What the
/// mark says turns on the store's standing** (`Keys` S10): where it reads
/// *never answered*, the passphrase cannot be kept on this computer and the
/// standing repair stops for good (`Keys` S11) — so the mark says *cannot*;
/// anywhere else it is a miss the next connect fixes. **A standing that could
/// not be read is not *never answered***, the same cautious reading
/// `crate::keys::for_a_connection` takes for `Servers` N8b.
///
/// **Where `Keys` was not asked, or the tools are absent, nothing is marked** —
/// a mark derived from nothing is never drawn, the same rule as the gone-key
/// mark's.
fn the_passphrase_mark(row: &ServerRow, said: Option<&WhatKeysSaid>) -> Option<Words> {
    let said = said?;
    if !said.the_tools_are_installed
        || !said
            .the_passphrase_is_not_available
            .iter()
            .any(|named| *named == row.key.local_key_identifier)
    {
        return None;
    }
    let mark = smoozy_keys::passphrase::how_the_server_is_marked(
        said.the_store_standing
            .unwrap_or(smoozy_security_and_licence::standing::StoreStanding::NeverProbed),
        true,
    );
    Some(Words::plain(if smoozy_keys::passphrase::the_standing_repair_stops(mark) {
        words::THE_PASSPHRASE_CANNOT_BE_KEPT
    } else {
        words::THE_PASSPHRASE_IS_NOT_HELD
    }))
}

/// **When the next attempt falls, on a repairing row** — `Design/Servers § A
/// row` and the mockup's *"Reconnecting… retrying in 12s"*. **One route**:
/// the page draws it with the underway shape's own countdown, and the words
/// are that shape's own. *Absent* — an attempt in flight, or a repair stopped
/// for good — draws the word alone (`Desktop app and delivery` M30).
fn the_next_attempt(
    of: WhatIsTrueOfTheServer,
    due: Option<std::time::SystemTime>,
) -> Option<TheNextAttempt> {
    if condition::presentation(of) != Presentation::Repairing {
        return None;
    }
    let seconds = due?
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .ok()?
        .as_secs();
    Some(TheNextAttempt {
        due_at: Verbatim::new(seconds.to_string()),
        trying_again_in: Prose::new(crate::surface::the_underway_shape::TRYING_AGAIN_IN),
        trying_again_now: Prose::new(crate::surface::the_underway_shape::TRYING_AGAIN_NOW),
    })
}

/// **`Desktop app and delivery` M29's four words, and the dash.**
///
/// **The word is taken from `crate::presentation::the_repairing_server` rather
/// than written again here**, so Design's vocabulary has one home. `Design/
/// Servers`: *"Where Smoozy cannot say what the server is doing, the column
/// takes the dash — the mark Design values gives app-wide for a value Smoozy
/// cannot confirm. Never a colour."*
///
/// **Connected, the column is the server's work** — `Design/Servers § A row`:
/// *"Rendering and the job's name with how many are waiting, or Idle"*. **Written
/// for one case and now meeting the other.** Until 2026-09-28 it composed
/// *Idle* for every connected server, its comment leaving *Rendering* for a
/// period that never came; step 10c's ninth pass read *Idle* on the list while
/// a job rendered. **It now reads the held queue reading, less every job whose
/// ending Smoozy has learned** (`Queue & History` C11): *Rendering* and the
/// name where the reading has a job rendering, *Waiting* and how many where it
/// holds live jobs and none rendering, *Idle* only where it holds nothing
/// live, and the dash where Smoozy cannot say — a reading owed and not yet
/// taken, or a reading not updating. **It never says *Rendering* from the
/// local record alone.** *(Until 2026-09-28 live jobs with none rendering took
/// the dash, for want of a word; `Design/Servers § A row` gives it now.)*
fn the_work_column(
    of: WhatIsTrueOfTheServer,
    doing: Option<&WhatAServerIsDoing>,
    nothing_of_smoozys_is_there: bool,
) -> Vec<Words> {
    match condition::presentation(of) {
        // **`Servers` K8: *restarting* is a fourth presentation and not a
        // fourth observed value**, and `Design/Servers` says it is not a
        // failure. Its elapsed reading is the period's and there is none.
        Presentation::Restarting => vec![Words::plain(words::RESTARTING)],
        Presentation::Repairing => {
            let word = WhatTheColumnSays::Reconnecting;
            vec![Words::plain(word.the_word())]
        }
        // **Both words are the last-known queue reading's** (`Queue & History`
        // C6), which is period-held.
        Presentation::Connected => match doing {
            Some(WhatAServerIsDoing::Rendering { name, waiting }) => {
                let rendering = WhatTheColumnSays::Rendering(Verbatim::new(name.clone()));
                let mut said = vec![Words::with(rendering.the_word(), name.clone())];
                // **How many wait, where any do** — the drawing's *2 waiting*
                // beneath the name, and nothing for nought, as the Queue
                // screen's folded heading does.
                if *waiting > 0 {
                    said.push(Words::after(waiting.to_string(), words::WAITING));
                }
                said
            }
            Some(WhatAServerIsDoing::NothingIsLive) => {
                vec![Words::plain(WhatTheColumnSays::Idle.the_word())]
            }
            // **`Design/Servers § A row`: *Waiting*, *2 jobs*, in the same
            // shape as *Rendering*, with the count where the job's name goes.**
            // The count is a figure, so the word after it is bound to it.
            Some(WhatAServerIsDoing::Waiting { how_many }) => vec![Words {
                lead: Prose::new(words::WAITING_LEAD),
                values: vec![Verbatim::new(how_many.to_string())],
                rest: Some(Prose::new(if *how_many == 1 { words::A_JOB } else { words::JOBS })),
                the_rest_joins: false,
            }],
            Some(WhatAServerIsDoing::ItCannotBeSaid) => vec![the_dash()],
            // **No reading is held: *Idle* only where Smoozy knows the server
            // is idle.** Where the live connection's check found no Smoozy of
            // this installation on the machine — none placed, or another
            // installation's — nothing on it is read as Smoozy's at all
            // (`Queue & History` A1), so no work of Smoozy's is there. **Every
            // other connected server with no reading takes the dash until one
            // lands**: the check not yet settled, step 7's reading owed, or the
            // check read on a connection that is no longer the live one.
            //
            // **Written for one case and now meeting the other.** It read
            // *Idle* wherever step 7 was not already known to owe a reading,
            // which was written for `Queue & History` A1's machine and also
            // met the stretch between a *Connect* going live and its check
            // settling — where nothing had been read at all. Step 10c's
            // eleventh pass read *Idle* there with a job rendering (*Smaller*
            // 1), and once where the page header read the dash (*Smaller* 2).
            // Step 10c's eleventh-pass fix piece 6.
            None if nothing_of_smoozys_is_there => {
                vec![Words::plain(WhatTheColumnSays::Idle.the_word())]
            }
            None => vec![the_dash()],
        },
        // **The dash.** Smoozy cannot say what a server it has never reached is
        // doing, and Design gives the dash for exactly that.
        Presentation::Disconnected => vec![the_dash()],
    }
}

/// **Whether V13's condition takes the work column to the dash** —
/// `Design/Servers § A row`: while updating, too old or too new. *The update
/// waits* and *the update failed* leave the column reading as ever: the
/// program there answers, and its work is read.
fn the_work_takes_the_dash_under(
    carries: Option<&crate::servers::published::WhatSmoozyOnTheServerIs>,
) -> bool {
    use crate::servers::published::WhatSmoozyOnTheServerIs as Carries;
    matches!(
        carries,
        Some(Carries::TheUpdateIsUnderWay)
            | Some(Carries::TooOldAndItsWorkCannotBeControlledFromHere)
            | Some(Carries::TooNew)
    )
}

/// **V15: *Update now…* is offered only where the update waits and the
/// connection is live.** On *too old* the stop cannot reach the work, and on
/// a server with no live connection nothing reaches it at all.
pub(crate) fn the_update_now_is_offered(
    carries: Option<&crate::servers::published::WhatSmoozyOnTheServerIs>,
    of: WhatIsTrueOfTheServer,
) -> bool {
    matches!(
        carries,
        Some(crate::servers::published::WhatSmoozyOnTheServerIs::TheUpdateWaits)
    ) && condition::presentation(of) == Presentation::Connected
}

/// **The too-old line, in the one of its two forms the reading allows.**
///
/// **Where the jobs are known to stand**, Design's *its jobs can't be stopped
/// from here*. **Where they are not known**, the build runner's sixth line:
/// Smoozy can't tell whether this server has jobs, and says so rather than
/// claiming them (Rule 8).
pub(crate) fn the_too_old_line(its_jobs_are_known_to_stand: bool) -> (&'static str, Words) {
    if its_jobs_are_known_to_stand {
        ("too-old", Words::plain(words::TOO_OLD_ITS_JOBS))
    } else {
        ("too-old-unknown", Words::plain(words::TOO_OLD_CANNOT_TELL))
    }
}

/// **`Desktop app and delivery` V13's condition, as the row's extra line**, in
/// `Design/Servers § A row`'s words — and nothing where the server carries
/// none, or is not connected.
///
/// **Only while connected.** Each of the five is a reading a connection took —
/// package inspection, a queue reading, a request in hand — and the register
/// that holds them is the run's, so a server no longer connected would
/// otherwise go on showing what a closed connection read. The next
/// connection's step 9 raises or withdraws each again (V12).
///
/// **Too old always takes the sixth line in this build.** The condition is
/// raised where a program shares no protocol with this build, whether or not
/// work stands there (`src/servers/installation.rs`), and **nothing in this
/// build reads that program's records**: `Server protocol and transfers` P3b's
/// records reading is not built. So whether it has jobs is not known, and
/// V11 holds it as one with work on it.
pub(crate) fn the_update_line(
    carries: Option<&crate::servers::published::WhatSmoozyOnTheServerIs>,
    of: WhatIsTrueOfTheServer,
    name: &str,
    doing: Option<&WhatAServerIsDoing>,
) -> Option<TheUpdateLine> {
    use crate::servers::published::WhatSmoozyOnTheServerIs as Carries;
    if condition::presentation(of) != Presentation::Connected {
        return None;
    }
    let a_line = |which: &str, said: Words| TheUpdateLine {
        which: Verbatim::new(which),
        said,
        a_failure: false,
        update_now: None,
        the_box: None,
        the_servers_words: None,
    };
    Some(match carries? {
        Carries::TheUpdateWaits => TheUpdateLine {
            update_now: the_update_now_is_offered(carries, of)
                .then(|| AControl::live(words::UPDATE_NOW, THE_UPDATE_NOW_IS_ASKED)),
            the_box: the_update_now_is_offered(carries, of)
                .then(|| the_update_now_box(name, doing)),
            ..a_line("waits", Words::plain(words::THE_UPDATE_WAITS))
        },
        Carries::TheUpdateIsUnderWay => a_line("under-way", Words::plain(words::UPDATING)),
        Carries::TheUpdateFailed { said } => TheUpdateLine {
            a_failure: true,
            the_servers_words: Some(TheServersWords {
                label: Prose::new(crate::surface::transfers::words::WHAT_THE_SERVER_SAID),
                words: Verbatim::new(said.clone()),
                close: Prose::new(crate::surface::the_machines_own_words::CLOSE),
                clear: AControl::live(
                    crate::presentation::the_failure_shape::CLEAR_THIS,
                    THE_UPDATE_FAILURE_IS_CLEARED,
                ),
            }),
            ..a_line("failed", Words::plain(words::COULD_NOT_BE_UPDATED))
        },
        Carries::TooOldAndItsWorkCannotBeControlledFromHere => {
            let (which, said) = the_too_old_line(false);
            a_line(which, said)
        }
        Carries::TooNew => a_line("too-new", Words::plain(words::TOO_NEW)),
    })
}

/// **V15's box**, with the removal box's own words (`Servers` W19a,
/// unchanged): the warning from the reading, and the *stop and clear first*
/// statement with its count.
fn the_update_now_box(name: &str, doing: Option<&WhatAServerIsDoing>) -> TheUpdateNowBox {
    let (rendering, waiting) = match doing {
        Some(WhatAServerIsDoing::Rendering { waiting, .. }) => (1, *waiting as usize),
        Some(WhatAServerIsDoing::Waiting { how_many }) => (0, *how_many as usize),
        _ => (0, 0),
    };
    TheUpdateNowBox {
        heading: Words {
            lead: Prose::new(words::UPDATE_SMOOZY_ON),
            values: vec![Verbatim::new(name)],
            rest: Some(Prose::new(words::NOW)),
            the_rest_joins: false,
        },
        said: Prose::new(words::UPDATING_NOW_STOPS_THEM),
        the_warning: (rendering + waiting > 0)
            .then(|| the_warning_from_the_reading(rendering, waiting)),
        the_statement: the_stop_and_clear_statement(rendering, waiting),
        cancel: Prose::new(words::CANCEL),
        stop_jobs_and_update: AControl::live(words::STOP_JOBS_AND_UPDATE, THE_UPDATE_NOW),
    }
}

/// **The removal box's own *stop and clear first* statement** — one home for
/// it, which the removal and V15's box both draw. `Servers` W19a bounds what may
/// be said before the act: the frame being rendered ***may not arrive***, and
/// never that it *is lost*.
fn the_stop_and_clear_statement(rendering: usize, waiting: usize) -> Words {
    // **Only the half it does is said** where the other count is nought — a
    // render with nothing waiting is stopped, and jobs waiting with nothing
    // rendering are cleared. Step 10c's eleventh-pass fix piece 6.
    match (rendering, waiting) {
        (_, 0) => Words::plain(words::STOP_AND_CLEAR_SAID_NONE_WAITING),
        (0, 1) => Words::plain(words::STOP_AND_CLEAR_SAID_ONE_NONE_RENDERING),
        (0, _) => Words {
            lead: Prose::new(words::STOP_AND_CLEAR_SAID_LEAD),
            values: vec![Verbatim::new(waiting.to_string())],
            rest: Some(Prose::new(words::STOP_AND_CLEAR_SAID_MANY_NONE_RENDERING)),
            the_rest_joins: false,
        },
        (_, 1) => Words::plain(words::STOP_AND_CLEAR_SAID_ONE),
        _ => Words {
            lead: Prose::new(words::STOP_AND_CLEAR_SAID_LEAD),
            values: vec![Verbatim::new(waiting.to_string())],
            rest: Some(Prose::new(words::STOP_AND_CLEAR_SAID_MANY)),
            the_rest_joins: false,
        },
    }
}

/// **The dash** — the mark `Design values` gives app-wide for a value Smoozy
/// cannot confirm, and `Design/Servers § A row`'s for a server whose doing
/// Smoozy cannot say. A value, since it is a mark and not a word.
fn the_dash() -> Words {
    Words {
        lead: Prose::new(""),
        values: vec![Verbatim::new("—")],
        rest: None,
        the_rest_joins: false,
    }
}

/// **`Servers` H1's three states, and what each shows.**
///
/// *"A server never connected shows nothing and says so rather than looking
/// empty"* — `Design/Servers § A row`.
fn the_hardware(held: Option<&HardwareReading>) -> TheHardware {
    // **Where no connection in this run has read this server**, no branch of
    // `Servers` H18 has been taken and H1's own table governs — see
    // [`the_hardware_state`]. What a connection read is
    // [`the_hardware_as_the_run_knows_it`]'s.
    the_hardware_drawn(held, the_hardware_state(held, None))
}

/// **`Servers` H1's state, taken from what the last connection read where
/// one has** — step 10b's first audit round, item 8.
///
/// - **The fresh reading was kept** (`Servers` H18's three branches that take
///   one, and `Servers` H4: it replaced the held one whole): *live* while that
///   connection is the live one and the server reads connected, *last seen*
///   otherwise (`Servers` H1a).
/// - **No fresh reading was kept** — the fourth branch, or a reading that did
///   not come back: the branch decides, through `Servers`' own
///   `hardware_state` — the two that disqualify return the row to *never
///   read*, and the fourth leaves the held reading as *last seen*.
fn the_hardware_as_the_run_knows_it(
    held: Option<&HardwareReading>,
    read: Option<crate::servers::a_period::WhatTheLastConnectionRead>,
    of: WhatIsTrueOfTheServer,
) -> TheHardware {
    the_hardware_drawn(held, the_hardware_state_this_run_knows(held, read, of))
}

/// **The state this row's hardware column stands in, on its own** — the
/// judgement [`the_hardware_as_the_run_knows_it`] draws with, lifted out so
/// that **every reader of a row's hardware reads the same answer.**
///
/// **It is `pub(crate)` for one reason and it is the fifth pass's fault 4.**
/// `Render Settings` D5 says the card make on a submission is *"the hardware
/// reading `Servers` holds for this row"*, and until 2026-09-23 that page read
/// a map nothing ever wrote — so a server row reading *1 × NVIDIA GeForce GTX
/// 1650, 4 GB* submitted a job whose manifest said `NoReadingStood`, and the
/// render worker had no technology to name. **A second copy of this judgement
/// beside the page would be the same fault again in slower motion**, so the
/// page calls this.
pub(crate) fn the_hardware_state_this_run_knows(
    held: Option<&HardwareReading>,
    read: Option<crate::servers::a_period::WhatTheLastConnectionRead>,
    of: WhatIsTrueOfTheServer,
) -> HardwareState {
    let Some(read) = read else {
        // **Where no connection in this run has read this server**, no branch
        // of `Servers` H18 has been taken and H1's own table governs.
        return the_hardware_state(held, None);
    };
    if read.a_fresh_reading_was_kept && held.is_some() {
        if read.on_the_connection_that_is_live
            && condition::presentation(of) == Presentation::Connected
        {
            HardwareState::Live
        } else {
            HardwareState::LastSeen
        }
    } else {
        the_hardware_state(held, Some(read.check))
    }
}

/// The hardware column, drawn in the state it is in.
fn the_hardware_drawn(held: Option<&HardwareReading>, state: HardwareState) -> TheHardware {
    let written = match state {
        HardwareState::NeverRead => "never-read",
        HardwareState::LastSeen => "last-seen",
        HardwareState::Live => "live",
    };
    let Some(held) = held.filter(|_| !matches!(state, HardwareState::NeverRead)) else {
        return TheHardware {
            state: Verbatim::new(written),
            cards: None,
            memory: None,
            free_disk: None,
            nothing_to_show: Some(Words::plain(words::NOTHING_IS_KNOWN_YET)),
            last_seen: None,
        };
    };

    TheHardware {
        state: Verbatim::new(written),
        cards: Some(the_cards(&held.cards)),
        memory: the_memory(held),
        free_disk: the_free_disk(held),
        nothing_to_show: None,
        // **H2a: the mark is the desktop's own moment**, so it can be compared
        // with the user's own now. The page words it; **which of the three
        // granularities is chosen is Design's** and the moment is what this
        // supplies.
        last_seen: matches!(state, HardwareState::LastSeen).then(|| {
            Words::with(
                words::LAST_SEEN,
                the_moment(held.the_desktops_own_moment.taken_at()),
            )
        }),
    }
}

/// **`Servers` H1's table, and H18's is a different question.**
///
/// **H18 says what each of the installation check's four branches does to a
/// held reading**, and two of them disqualify it. **That check is a read taken
/// on a connection** (step 5 of the first-connection sequence), and the branch
/// it settled on is handed back by `Servers`' driving and held for the run
/// (`src/servers/a_period.rs`) — [`the_hardware_as_the_run_knows_it`] passes it
/// here. Where no connection has read the server in this run, no branch has
/// been taken, which is not the same as the *absent* branch having been taken.
///
/// Where no check is handed in, **H1's own table is what governs**: a held
/// reading whose *live* row's condition does not hold is *last seen*, and no
/// reading at all is *never read*. **Where the check is handed here,
/// `hardware_state` is what answers** — which is why it is called rather than
/// reimplemented.
fn the_hardware_state(
    held: Option<&HardwareReading>,
    check: Option<InstallationCheck>,
) -> HardwareState {
    match check {
        Some(check) => hardware_state(
            held,
            WhichConnectionHoldsIt {
                // **`Servers` H1a: keyed on the connection the reading was taken on**,
                // never on the period alone. No connection is live.
                taken_on_the_connection_that_is_live_now: false,
            },
            check,
        ),
        None if held.is_some() => HardwareState::LastSeen,
        None => HardwareState::NeverRead,
    }
}

/// **`Servers` H5 to H11, drawn.** Every figure is a value and every word is
/// prose, and the two never join.
fn the_cards(cards: &Cards) -> Words {
    match cards {
        Cards::TheCards(cards) if !cards.is_empty() => {
            // **The whole line is one value, composed out of values.** The
            // count is `Servers` H5's — *"the number of rows of a per-card
            // query"* — the make is H7's keyable value, and the memory is H8's.
            // **No word is on this path at all**, which is X7 kept by there
            // being no sentence for a figure to land inside.
            //
            // **Where the cards differ, each is named**: a count times one make
            // would be a statement about a machine that has two different
            // cards, and no reading says that.
            let first = the_card_reads(&cards[0]);
            let all_alike = cards.iter().all(|card| the_card_reads(card) == first);
            let said = if all_alike && cards.len() > 1 {
                format!("{} × {first}", cards.len())
            } else if all_alike {
                format!("1 × {first}")
            } else {
                cards
                    .iter()
                    .map(the_card_reads)
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            Words {
                lead: Prose::new(""),
                values: vec![Verbatim::new(said)],
                // **The drawing's own word for more than one of the same
                // card.** It is prose and it sits beside the figures.
                rest: (all_alike && cards.len() > 1).then(|| Prose::new(words::EACH)),
                the_rest_joins: false,
            }
        }
        // **A stated absence is a complete reading and never a failed one** —
        // H8, `Servers` H9a and H11 all take the same shape.
        Cards::PresentAndUnreadable(WhyUnreadable::TheDriverIsAbsent)
        | Cards::PresentAndUnreadable(WhyUnreadable::TheDriverAnsweredWithNone)
        | Cards::CouldNotBeEstablished => Words::plain(words::CARDS_UNREADABLE),
        Cards::NoCard => Words::plain(words::NO_CARD),
        Cards::TheCards(_) => Words::plain(words::NO_CARD),
    }
}

/// One card, as the drawing writes one — which card it is, then its memory.
/// **Every part is a value.**
///
/// **Which card it is, is the driver's own name for it and never the make's
/// key** — `Servers` H7: the key is what `Rendering` L23's table is looked up
/// by, and the name is what the spec's *"how many, which they are"* shows.
/// Until step 10c's sixth pass the two were one string; they are two fields
/// now, and the row reads the name.
fn the_card_reads(card: &smoozy_servers::hardware::Card) -> String {
    let which = card.make.the_drivers_name_for_the_card();
    match card.total_memory_bytes {
        Some(bytes) => format!("{which}, {}", in_gigabytes(bytes)),
        // **H8: where the per-card memory route is absent, the count and the
        // identity survive and the memory does not.**
        None => which.to_string(),
    }
}

/// **The machine's own memory, in Rule 9's own industry word.**
///
/// **`Servers` H9a's third outcome is a stated absence and never the `/proc`
/// figure**:
/// on a slice that figure is the host's, and a reader that reaches it because
/// the cgroup was silent gets the host's answer while believing it got the
/// rental's. So an allocation that could not be established says so.
fn the_memory(held: &HardwareReading) -> Option<Words> {
    use smoozy_servers::hardware::MachineMemory;
    match held.machine_memory {
        MachineMemory::TheAllocation(bytes) | MachineMemory::TheWholeMachine(bytes) => {
            Some(Words::after(in_gigabytes(bytes), words::RAM))
        }
        MachineMemory::CouldNotBeEstablished => Some(Words::plain(words::MEMORY_UNKNOWN)),
    }
}

/// **The free disk, which H3 says is explicitly allowed to be wrong** — a job
/// left running writes files, so a held figure is only what was true when the
/// reading was taken, and the *last seen* mark is what covers it.
fn the_free_disk(held: &HardwareReading) -> Option<Words> {
    held.free_space
        .as_ref()
        .map(|free| Words::after(in_gigabytes(free.available_bytes), words::FREE_DISK))
}

/// **`Servers` P1a's pair, composed from the key row rather than from the
/// page.**
///
/// *"The row's key reference is the pair — the local key identifier **and** the
/// key fingerprint beside it."* **The page holds neither half of that pair's
/// second value and must not**: a fingerprint is `Keys`' and what a page could
/// send is a string it made up. So the identifier arrives from the dialog and
/// **the fingerprint is read off the key row it names**.
///
/// **An identifier that names no key row, or a key row with no fingerprint, is
/// refused and never saved** — step 10a round 1, fix-here item 2(d), the
/// round's call: the picker offers only keys it can vouch for, and
/// `Specs/Servers` says *"a key that cannot be used at all is caught in the
/// dialog, since such keys are never offered there"*. **An empty fingerprint is
/// never written**, because a row holding one is marked *key gone* for good
/// and `Keys` cannot say which servers rely on its key.
///
/// **The one exception is the Edit dialog's own key, left as it is**:
/// `Design/Servers § The Edit dialog` keeps an unusable key selected rather
/// than clearing it, and *"the edit can then be saved with that unusable key
/// left exactly as it is"* — so an edit that did not change the key keeps the
/// pair the row already held, fingerprint and all (`Servers` P16).
fn the_key_reference(
    started: &Started,
    key_identifier: &str,
    held: Option<&KeyReference>,
) -> Option<KeyReference> {
    let identifier = key_identifier.trim();
    if let Some(held) = held.filter(|held| held.local_key_identifier == identifier) {
        return Some(held.clone());
    }
    let keys = the_key_rows(started)?;
    let key = keys
        .iter()
        .find(|key| !identifier.is_empty() && key.identifier.as_text() == identifier)?;
    let fingerprint = key.fingerprint.as_ref()?.value.as_str().to_string();
    (!fingerprint.is_empty()).then(|| KeyReference {
        local_key_identifier: identifier.to_string(),
        fingerprint,
    })
}

/// **What a key control shows**: the key's own name, or its fingerprint where
/// it has none. A value either way.
fn the_key_shows_as(key: &smoozy_keys::rows::KeyRow) -> String {
    match (&key.name, &key.fingerprint) {
        (Some(name), _) => name.as_str().to_string(),
        (None, Some(fingerprint)) => fingerprint.value.as_str().to_string(),
        (None, None) => key.path.display().to_string(),
    }
}

/// **The port the user typed, where it is a port at all.**
///
/// **H34: the value is validated here, before an adapter is touched**, and
/// **the field arrives as text rather than as a number on purpose.** A command
/// taking a whole number refuses anything outside the number's own range
/// before it reaches this application at all — so *70000* and *twenty-two* both
/// answer with silence, and `Design/Servers` requires the opposite in as many
/// words: *"the port is the exception and refuses in place, like every other
/// number the app will not take: it holds a whole number and says the range"*.
///
/// **The range is `Servers` C4's and is not restated here**: what this does is
/// read the text and hand whatever comes of it to
/// [`smoozy_servers::identity::Port::what_the_user_typed`], which owns the
/// refusal of nought.
///
/// **A field left empty is 22**, which is `Specs/Servers`' own rule: *"clearing
/// it puts 22 back rather than leaving it empty"* (step 10a round 1, fix-here
/// item 28). The page puts it back in the field too.
fn the_port_typed(typed: &str) -> Option<Port> {
    let typed = typed.trim();
    if typed.is_empty() {
        return Some(Port::STANDARD_SSH);
    }
    Port::what_the_user_typed(typed.parse::<u16>().ok()?)
}

/// **A byte count as the drawing writes one** — *24 GB*, *49 GB*, *104 GB*.
///
/// **It is a value entire and never touches the typography point** (X7, X9):
/// the figure and its unit are one thing the user reads off the machine, and
/// the word beside them — *RAM*, *free disk* — is the prose. **The rounding is
/// down**, so nothing claims a machine has more than it has.
///
/// **The divisor is 1024 three times over and that is a ruling with a reason.**
/// A card of 25,769,803,776 bytes is the card everybody calls a 24 GB card, and
/// the three tools these readings come from all write it that way: the driver's
/// own query reports 24576 MiB, `free -g` reports whole GiB, and a free-space
/// figure read through `statvfs` is what `df -h` writes with a `G`. **Dividing
/// by a thousand three times would put *25 GB* beside a card the user bought as
/// a 24**, and `Design/Servers`' own drawing writes 24. Rule 9: the word the
/// rest of the industry uses.
fn in_gigabytes(bytes: u64) -> String {
    let gigabytes = bytes / (1024 * 1024 * 1024);
    crate::typography::a_figure_with_its_unit(gigabytes, "GB")
}

/// **A moment, in seconds since the epoch, as a number the page formats.**
///
/// **Never an elapsed time.** `State, identity and recovery` D5 forbids
/// arithmetic across two clocks and an interval computed here would have
/// already done it; **which of Design's three granularities is chosen is
/// Design's**, and what this supplies is the moment they are chosen against.
fn the_moment(at: std::time::SystemTime) -> String {
    at.duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs().to_string())
        .unwrap_or_default()
}

/// **`Servers` §11.8's line, and where it goes is read rather than assumed.**
///
/// **`Servers` W40 makes the statement once, at the add's save**, and this
/// screen makes it there — [`the_add_dialog_from`] carries it and the drawing
/// puts it under the two setup options, which is where the add is.
///
/// **`Servers` W40a composes it a second time *in the figure's place*** —
/// *"wherever the machine is shown and the figure would otherwise have
/// stood"*. **`Design/Servers § A row` draws no running-time reading on a list
/// row**: a row is the badge, the name, the address, the one extra line, what
/// it is doing, what it has to render with, and the actions. **So the figure
/// would not have stood here, and neither does its substitute.**
///
/// **This is a reading of `Servers` W40a and not a refusal of it, and it is
/// ruled here with the cost stated.** The figure is *"wherever the machine is
/// shown"*, and where the machine is shown with room for a reading is **the
/// server's own screen**, whose head draws the hardware line again. **That head
/// carries it** — `src/surface/a_server.rs`, which is this function's one
/// caller, puts the line on the head's hardware line (step 10a round 1,
/// fix-here item 16) — rather than a three-line sentence going on every row of
/// a list whose whole job is comparing servers at a glance. **It was tried on
/// the list first and photographed**, which is how the cost of that reading was
/// measured.
///
/// **What is kept whole is the rule**: the line the figure stands on is never
/// empty of both, and this is the shape that keeps it so.
pub fn the_rental_line(running_for: Option<Words>) -> TheRentalLine {
    match running_for {
        Some(figure) => TheRentalLine {
            running_for: Some(figure),
            the_statement: None,
        },
        None => TheRentalLine {
            running_for: None,
            the_statement: Some(Words::plain(words::RUNS_AND_BILLS)),
        },
    }
}

/// **The connect control, carrying the connection state** —
/// `Design/Servers § Actions on a row`, and `Design/Whole app`.
///
/// *"Where a server carries a setup it never got to run, the control says so —
/// Connect and set up."* **And `Servers` P7: the connect action is withdrawn
/// for a server marked as needing a new key**, because connecting would fail.
fn the_connect_control(
    row: &ServerRow,
    of: WhatIsTrueOfTheServer,
    needing_a_new_key: bool,
) -> AControl {
    match condition::presentation(of) {
        Presentation::Connected => AControl::live(words::CONNECTED, "disconnect"),
        Presentation::Restarting => AControl::shown_unavailable(words::RESTARTING, "disconnect"),
        Presentation::Repairing => AControl::live(
            WhatTheColumnSays::Reconnecting.the_word(),
            "disconnect",
        ),
        Presentation::Disconnected if needing_a_new_key => {
            AControl::shown_unavailable(words::CONNECT, "connect")
        }
        Presentation::Disconnected if row.remembered_setup.is_some() => {
            AControl::live(words::CONNECT_AND_SET_UP, "connect-and-set-up")
        }
        Presentation::Disconnected => AControl::live(words::CONNECT, "connect"),
    }
}

/// **Which of Design's five menus this row is given.**
///
/// **`Servers` K8a is what makes this tellable**: a repairing server takes the
/// **connected** menu whose first slot reads *disconnect*, and a server whose
/// period does not stand takes the disconnected one. `Design/Servers` states
/// the same from its own side and gives the reason — *"the one thing the user
/// needs there is the way out, and the disconnected menu offers connect, which
/// is the opposite"*.
fn which_menu(row: &ServerRow, of: WhatIsTrueOfTheServer, needing_a_new_key: bool) -> &'static str {
    match (condition::menu(of), needing_a_new_key) {
        (Menu::TheConnectedMenu, true) => "connected-key-gone",
        (Menu::TheConnectedMenu, false) => "connected",
        (Menu::TheDisconnectedMenu, true) => "needs-a-new-key",
        (Menu::TheDisconnectedMenu, false) if row.remembered_setup.is_some() => {
            "waiting-to-be-set-up"
        }
        (Menu::TheDisconnectedMenu, false) => "disconnected",
    }
}

/// **Design's five menus, in Design's own five slots.**
///
/// *"Every menu is the same five slots in the same order … A slot with nothing
/// in it is skipped, not padded, and the order still holds. What matters is
/// that nothing above a slot ever moves, so the position of every item is the
/// same in all five menus."* Label: **Creator-stated** (2026-08-13).
///
/// **The key actions sit in slot 3 and that cost is stated in Design**: they
/// were first in the two menus that have them, which put connect third in those
/// two and first everywhere else.
fn the_five_menus() -> Vec<AMenu> {
    let details = || {
        vec![
            AControl::live(words::RENAME_ITEM, "rename"),
            AControl::live(words::EDIT_ITEM, "edit"),
            AControl::live(words::COPY_ADDRESS, "copy-address"),
        ]
    };
    let remove = || AControl::live(words::REMOVE, "remove");
    let the_two_ways_out = || {
        vec![
            AControl::live(words::A_DIFFERENT_KEY, "a-different-key"),
            AControl::live(words::FIX_THE_KEY, "fix-the-key"),
        ]
    };
    let going_in = |available: bool| {
        if available {
            vec![
                AControl::live(words::BROWSE_FILES, "browse"),
                AControl::live(words::SET_UP, "set-up"),
            ]
        } else {
            vec![
                AControl::shown_unavailable(words::BROWSE_FILES, "browse"),
                AControl::shown_unavailable(words::SET_UP, "set-up"),
            ]
        }
    };

    vec![
        AMenu {
            which: Verbatim::new("connected"),
            the_connection: AControl::live(words::DISCONNECT, "disconnect"),
            going_in: going_in(true),
            about_itself: Vec::new(),
            the_details: details(),
            removing: remove(),
        },
        AMenu {
            which: Verbatim::new("disconnected"),
            the_connection: AControl::live(words::CONNECT, "connect"),
            going_in: going_in(false),
            about_itself: Vec::new(),
            the_details: details(),
            removing: remove(),
        },
        AMenu {
            which: Verbatim::new("needs-a-new-key"),
            the_connection: AControl::shown_unavailable(words::CONNECT, "connect"),
            going_in: going_in(false),
            about_itself: the_two_ways_out(),
            the_details: details(),
            removing: remove(),
        },
        AMenu {
            which: Verbatim::new("connected-key-gone"),
            the_connection: AControl::live(words::DISCONNECT, "disconnect"),
            // **Nothing is shown unavailable, because nothing is**: disconnect,
            // browse and set up all work. `Design/Servers`.
            going_in: going_in(true),
            about_itself: the_two_ways_out(),
            the_details: details(),
            removing: remove(),
        },
        AMenu {
            which: Verbatim::new("waiting-to-be-set-up"),
            the_connection: AControl::live(words::CONNECT_AND_SET_UP, "connect-and-set-up"),
            going_in: going_in(false),
            // **`Servers` B5's third way out of the remembered setup bit**, and
            // `Design/Servers § Dropping a setup the server never ran` says why
            // it is in the menu rather than on the row.
            about_itself: vec![AControl::live(words::DROP_THE_SETUP, "drop-the-setup")],
            the_details: details(),
            removing: remove(),
        },
    ]
}

/// `Design/Servers § No servers yet`, and the mockup's `#svempty`.
fn the_guidance() -> TheGuidance {
    TheGuidance {
        heading: Prose::new(words::NO_SERVERS_YET),
        how_it_goes: Prose::new(words::HOW_IT_GOES),
        steps: vec![
            TheStep {
                lead: Prose::new(words::STEP_1_LEAD),
                rest: Some(Prose::new(words::STEP_1_REST)),
                opens_keys: true,
            },
            TheStep {
                lead: Prose::new(words::STEP_2_LEAD),
                rest: None,
                opens_keys: false,
            },
            TheStep {
                lead: Prose::new(words::STEP_3_LEAD),
                rest: Some(Prose::new(words::STEP_3_REST)),
                opens_keys: false,
            },
            TheStep {
                lead: Prose::new(words::STEP_4_LEAD),
                rest: Some(Prose::new(words::STEP_4_REST)),
                opens_keys: false,
            },
            TheStep {
                lead: Prose::new(words::STEP_5_LEAD),
                rest: Some(Prose::new(words::STEP_5_REST)),
                opens_keys: false,
            },
        ],
        open_keys: Prose::new(words::OPEN_KEYS),
        add_a_server: Prose::new(words::ADD_A_SERVER),
    }
}

/// The two boxes a connection raises.
fn the_boxes() -> TheBoxesAConnectionRaises {
    TheBoxesAConnectionRaises {
        the_changed_identity: TheChangedIdentityBox {
            // **The server's name is a value and the sentence stops where it
            // begins.** The mockup writes them as one line; the split is this
            // piece's real work.
            heading: Words {
                lead: Prose::new(""),
                values: Vec::new(),
                rest: Some(Prose::new(words::IDENTITY_CHANGED)),
                the_rest_joins: false,
            },
            said: Prose::new(words::IDENTITY_CHANGED_SAID),
            accepting: Prose::new(words::ACCEPTING_IS_A_DECISION),
            do_not_connect: Prose::new(words::DO_NOT_CONNECT),
            accept: Prose::new(words::ACCEPT_AND_CONNECT),
        },
        stop_and_clear: TheRemovalsEnding::ALL.iter().map(|ending| ending.said()).collect(),
        stopping_work_on: Prose::new(words::STOPPING_WORK_ON),
        try_again: Prose::new(words::TRY_AGAIN),
        remove_anyway: Prose::new(words::REMOVE_ANYWAY),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The Add dialog
// ════════════════════════════════════════════════════════════════════════════

/// The Add dialog, with the rows handed in.
pub fn the_add_dialog_from(rows: &[ServerRow]) -> TheAddDialog {
    let used: Vec<&str> = rows.iter().filter_map(|row| the_badge(row.colour.as_deref())).collect();
    TheAddDialog {
        heading: Prose::new(words::ADD_A_SERVER),
        the_guidance_again: Prose::new(words::THE_GUIDANCE_AGAIN),
        host: Prose::new(words::HOST),
        host_placeholder: Prose::new(words::HOST_OR_PASTE),
        port: Prose::new(words::PORT),
        username: Prose::new(words::USERNAME),
        key: Prose::new(words::KEY),
        name: Prose::new(words::NAME),
        optional: Prose::new(words::OPTIONAL),
        new_key: Prose::new(words::NEW_KEY),
        colour: Prose::new(words::COLOUR),
        the_port_starts_at: Verbatim::new(Port::STANDARD_SSH.number().to_string()),
        the_eight: crate::surface::the_rail::THE_EIGHT_BADGES
            .iter()
            .map(|badge| Verbatim::new(*badge))
            .collect(),
        already_used: used.iter().map(|badge| Verbatim::new(*badge)).collect(),
        picked_for_you: Verbatim::new(the_colour_picked_for_you(&used)),
        the_swatch_note: Prose::new(words::THE_SWATCH_NOTE),
        choose_a_key: Prose::new(words::CHOOSE_A_KEY),
        get_ready: Prose::new(words::GET_READY),
        install_blender: Prose::new(words::INSTALL_BLENDER),
        ticking_either: Prose::new(words::TICKING_EITHER),
        cancel: Prose::new(words::CANCEL),
        add: Prose::new(words::ADD),
        add_and_set_up: Prose::new(words::ADD_AND_SET_UP),
        runs_and_bills: Prose::new(words::RUNS_AND_BILLS),
    }
}

/// **What a pasted line fills the Add dialog with, and the line under Host.**
/// `Design/Servers § Adding a server`: *"A line missing a piece fills what it
/// has and names what it lacks in one line under Host"*, and a line with no
/// port fills 22 and says nothing. A text that is not a connection line fills
/// nothing and says nothing; the page hands over only one that is.
pub fn the_pasted_line_from(line: &str) -> ThePastedLine {
    use smoozy_servers::pasted_line::{the_line_read, WhatTheLineGives};
    let nothing = |the_line_under: Vec<Prose>| ThePastedLine {
        fills: false,
        host: Verbatim::new(""),
        port: Verbatim::new(""),
        username: Verbatim::new(""),
        the_line_under,
    };
    match the_line_read(line) {
        None => nothing(Vec::new()),
        Some(WhatTheLineGives::RunpodsProxiedLine) => {
            nothing(vec![Prose::new(words::RUNPODS_PROXIED_LINE)])
        }
        Some(WhatTheLineGives::TheFields { host, port, username }) => {
            let mut the_line_under = Vec::new();
            if host.is_none() {
                the_line_under.push(Prose::new(words::NO_HOST_IN_THAT_LINE));
            }
            if username.is_none() {
                the_line_under.push(Prose::new(words::NO_USERNAME_IN_THAT_LINE));
            }
            ThePastedLine {
                fills: true,
                host: Verbatim::new(host.unwrap_or_default()),
                port: Verbatim::new(port),
                username: Verbatim::new(username.unwrap_or_default()),
                the_line_under,
            }
        }
    }
}

/// **`Design/Servers § Adding a server`: the colour arrives already picked, and
/// it is one no other server is using.** Label: **Creator-stated**
/// (2026-08-13).
///
/// *"Smoozy takes the first of the eight nobody has taken … With more than
/// eight servers there is nothing left to take. Smoozy picks the hue used by
/// the fewest servers rather than refusing to add one."*
pub fn the_colour_picked_for_you(used: &[&str]) -> &'static str {
    let eight = crate::surface::the_rail::THE_EIGHT_BADGES;
    let count = |badge: &str| used.iter().filter(|taken| **taken == badge).count();
    if let Some(free) = eight.iter().find(|badge| count(badge) == 0) {
        return free;
    }
    // **The honest fallback rather than a case worth designing around.**
    eight
        .iter()
        .min_by_key(|badge| count(badge))
        .copied()
        .unwrap_or("b1")
}

/// **The two checks, neither of which blocks, and the port refusal, which does
/// — in place.**
pub fn the_checks_on(rows: &[ServerRow], host: &str, port: &str, label: &str) -> Event {
    let mut event = Event::about(About::TheRun, THE_ANSWER)
        .saying("what", value("the-add-was-checked"));

    // **The port is the exception and refuses in place**, like every other
    // number the app will not take: it holds a whole number, so a value it will
    // not take is a different mistake from a field left empty.
    let port = the_port_typed(port);
    if port.is_none() {
        event = event.saying("thePortWillNotDo", prose(words::THE_PORT_RANGE));
    }

    // **The possible-duplicate flag: C3's loose comparison on host and port.**
    // *"It never blocks, which is what makes the loose comparison the right
    // one."*
    if let (Some(host), Some(port)) = (Host::what_the_user_typed(host), port) {
        let asking = ConnectionIdentity {
            host,
            port,
            username: Username::what_the_user_typed("x").expect("a placeholder login"),
            key: KeyReference {
                local_key_identifier: String::new(),
                fingerprint: String::new(),
            },
        };
        if let Some(already) = rows.iter().find(|row| {
            smoozy_servers::identity::names_the_same_machine(
                &asking,
                &ConnectionIdentity {
                    host: row.host.clone(),
                    port: row.port,
                    username: row.username.clone(),
                    key: row.key.clone(),
                },
            )
        }) {
            event = event
                .saying("mayAlreadyHave", prose(words::MAY_ALREADY_HAVE_LEAD))
                .saying(
                    "mayAlreadyHaveNamed",
                    value(what_a_server_is_called_among(already, rows)),
                )
                .saying(
                    "mayAlreadyHaveRest",
                    prose(words::MAY_ALREADY_HAVE_REST),
                );
        }
    }

    // **The duplicate-name statement is a second, separate check and must not
    // read like the first**: one says two rows may be reaching the same
    // machine, the other says two rows will be hard to tell apart.
    let named = label.trim();
    if !named.is_empty()
        && rows
            .iter()
            .any(|row| row.label.as_deref().map(str::trim) == Some(named))
    {
        event = event
            .saying("alreadyCalled", prose(words::ALREADY_CALLED_LEAD))
            .saying("alreadyCalledNamed", value(named.to_string()))
            .saying("alreadyCalledRest", prose(words::ALREADY_CALLED_REST));
    }

    event
}

// ════════════════════════════════════════════════════════════════════════════
// The Edit dialog
// ════════════════════════════════════════════════════════════════════════════

/// The Edit dialog, read off the row.
pub fn the_edit_dialog_from(
    row: &ServerRow,
    all: &[ServerRow],
    keys: &[smoozy_keys::rows::KeyRow],
    said: Option<&WhatKeysSaid>,
) -> TheEditDialog {
    // **`Design/Servers § The Edit dialog`: it keeps an unusable key selected
    // rather than clearing it.** *"Clearing it would lose the one piece of
    // evidence the user needs — which key this server was using — at the exact
    // moment they are trying to replace it."*
    let the_key_is_gone = the_key_mark(row, keys, said);
    // **Where the key row itself is gone, the fingerprint the server row holds
    // stands in for its name** (`Servers` P16 holds it for exactly this), so the
    // gone key still shows which key it was rather than nothing. The Creator's
    // eighth notes, piece E3.
    let the_key_shows_as = keys
        .iter()
        .find(|key| key.identifier.as_text() == row.key.local_key_identifier)
        .map(|key| Verbatim::new(the_key_shows_as(key)))
        .or_else(|| {
            (the_key_is_gone && !row.key.fingerprint.is_empty())
                .then(|| Verbatim::new(row.key.fingerprint.clone()))
        });
    TheEditDialog {
        heading: Words::with(words::EDIT, what_a_server_is_called_among(row, all)),
        host: Prose::new(words::HOST),
        port: Prose::new(words::PORT),
        username: Prose::new(words::USERNAME),
        key: Prose::new(words::KEY),
        name: Prose::new(words::NAME),
        new_key: Prose::new(words::NEW_KEY),
        cancel: Prose::new(words::CANCEL),
        save: Prose::new(words::SAVE),
        the_host: Verbatim::new(row.host.as_str().to_string()),
        the_port: Verbatim::new(row.port.number().to_string()),
        the_username: Verbatim::new(row.username.as_str().to_string()),
        the_name: Verbatim::new(row.label.clone().unwrap_or_default()),
        the_key_identifier: Verbatim::new(row.key.local_key_identifier.clone()),
        the_key_shows_as,
        choose_a_key: Prose::new(words::CHOOSE_A_KEY),
        the_standard_port: Verbatim::new(Port::STANDARD_SSH.number().to_string()),
        the_username_note: Prose::new(words::THE_USERNAME_NOTE),
        the_gone_key_note: the_key_is_gone.then(|| Prose::new(words::THE_GONE_KEY_NOTE)),
        the_gone_mark: the_key_is_gone.then(|| Prose::new(words::THE_GONE_MARK)),
        // **`Design/Servers`: absent where none is remembered, rather than
        // greyed.** The whole-app password prompt names this dialog as the
        // place, so the control has to be here.
        a_password_is_remembered: row
            .machine_password_subject
            .as_ref()
            .map(|_| Prose::new(words::A_PASSWORD_IS_REMEMBERED)),
        forget_it: Prose::new(words::FORGET_IT),
        fallback_folder: Prose::new(words::FALLBACK_FOLDER),
        the_fallback_folder: Verbatim::new(row.chosen_fallback_folder.clone().unwrap_or_default()),
        the_fallback_placeholder: Prose::new(words::SMOOZYS_FOLDER),
        the_fallback_note: Prose::new(words::WHERE_JOBS_GO),
    }
}

/// **`Servers` W39 — the one clause of that plan this piece keeps that the
/// module's own crate does not.**
///
/// `Servers` W39 is the edit warning checked against the three escalations and declined,
/// and what it turns on is where the warning's knowledge comes from. **`Servers` W31
/// gives two sources, taken in order**: the period-held queue reading where a
/// period stands, and **the durable local record alone** where none does —
/// *"the jobs this desktop tracks on that server for which it has learned no
/// ending"*.
///
/// **`Servers` W34 bounds the wording: jobs Smoozy has **not seen end**, never jobs
/// queued or running.** *"The durable record carries no line position, and a
/// job with no learned ending may have ended hours ago. It is over-inclusive by
/// construction and the wording must carry that."*
///
/// **`Servers` W36: it cannot be switched off and no path can write a suppression row for
/// it.** There is no field here for one, which is the structural form of the
/// same refusal `Servers` W20 makes for the connected removal warning.
///
/// **`Servers` W35: where both sources are empty there is no warning, and that is a true
/// statement rather than the removal warning's ignorance.**
///
/// **The second source is read; the first is held by nothing.** The second is
/// `Queue & History`'s durable job records, which the local record holds since
/// step 10b piece 6 and which [`the_edit_warning_as_typed`] reads. The first is
/// the last-known queue reading, which a connected period holds (`Queue &
/// History` C6) and which is taken by `Servers` Q1 step 7 — one of the two
/// steps of that sequence that is not built, so no period holds one; it is
/// taken first once something holds it. *(Corrected 2026-09-19, the Gemini
/// catch-up of step 10a, Area 5, F-1: this said neither source reached the
/// screen and called the empty answer `Servers` W35's own. `Servers` W35 means both
/// sources checked and empty, not a source never read, and the dialog handed
/// this an empty list, so the warning never showed.)*
///
/// **`Servers` W37 decides which edits carry it**: host or port, and username.
/// Where the host or port changes the host words are used, with or without a
/// username change — leaving the jobs on another machine is the larger fact.
/// Where only the username changes, `Design/Servers § The Edit dialog`'s
/// username words are used. A key change or a rename carries none.
pub fn the_edit_warning(
    changed: &[WhatChanged],
    jobs_with_no_learned_ending: &[String],
) -> Option<TheEditWarning> {
    if jobs_with_no_learned_ending.is_empty() {
        return None;
    }
    let one = jobs_with_no_learned_ending.len() == 1;
    let (lead, have_not_ended, what_becomes_of_them) =
        if changed.contains(&WhatChanged::HostOrPort) {
            (
                words::POINTS_AT_A_DIFFERENT_SERVER,
                if one {
                    words::HAVE_NOT_ENDED_ONE
                } else {
                    words::HAVE_NOT_ENDED_MANY
                },
                words::LEFT_WHERE_THEY_ARE,
            )
        } else if changed.contains(&WhatChanged::Username) {
            (
                words::LOGS_IN_AS_A_DIFFERENT_USER,
                if one {
                    words::HAVE_NOT_ENDED_HERE_ONE
                } else {
                    words::HAVE_NOT_ENDED_HERE_MANY
                },
                words::CANNOT_REACH_THEM_AFTER,
            )
        } else {
            return None;
        };
    Some(TheEditWarning {
        lead: Prose::new(lead),
        how_many: Verbatim::new(jobs_with_no_learned_ending.len().to_string()),
        have_not_ended: Prose::new(have_not_ended),
        named: jobs_with_no_learned_ending
            .iter()
            .map(|name| Verbatim::new(name.clone()))
            .collect(),
        what_becomes_of_them: Prose::new(what_becomes_of_them),
    })
}

/// **The warning for what the user has typed so far**, against the row as it
/// is saved — `Servers` W37's changes read by [`what_changed`], and `Servers`
/// W31's second source read from the local record.
///
/// **Nothing is said where nothing typed would be saved**: a host, port or
/// username that will not parse never reaches the record, so it abandons
/// nothing. **Where the job records will not read, no warning is composed
/// either, and that is a gap rather than `Servers` W35's answer**: the dialog
/// then says nothing about a read that did not answer. Nothing asked for a
/// telling there, so none is invented; it is named in
/// `Audits/Build/Catch-up — Gemini/What the implementer did — Areas 4 and
/// 5.md`.
pub fn the_edit_warning_as_typed(
    record: Option<&rusqlite::Connection>,
    row: &ServerRow,
    host: &str,
    port: &str,
    username: &str,
) -> Option<TheEditWarning> {
    let (Some(host), Some(port), Some(username)) = (
        Host::what_the_user_typed(host),
        the_port_typed(port),
        Username::what_the_user_typed(username),
    ) else {
        return None;
    };
    let changed = what_changed(
        row,
        &host,
        port,
        &username,
        &row.key.local_key_identifier,
        row.label.as_deref().unwrap_or_default(),
    );
    if !changed
        .iter()
        .any(|one| matches!(one, WhatChanged::HostOrPort | WhatChanged::Username))
    {
        return None;
    }
    let server = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier)?;
    let jobs = crate::record::job_records::JobRecords::over(record?)
        .with_no_learned_ending_for(&server)
        .ok()?;
    let named: Vec<String> = jobs
        .iter()
        // **The entry's own name, as History draws it** — a job sent with no
        // name by its file (step 10c's Vast pass, fault 3).
        .map(crate::surface::the_queue::what_the_job_is_called)
        .collect();
    the_edit_warning(&changed, &named)
}

// ════════════════════════════════════════════════════════════════════════════
// The removal box
// ════════════════════════════════════════════════════════════════════════════

/// **What the last-known queue reading holds, with its two counts** —
/// `Queue & History` C7's period-held record, which is what `Servers` W19
/// composes the removal warning from and from nothing else.
///
/// **The counts travel with the reading so no path can warn without them**
/// (step 10a round 1, fix-here item 17): the warning used to read *Remove this
/// server?* with no figures, and *Stop and clear* always said *one* waiting job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheLastKnownReading {
    /// No record at all — `Servers` W19's rows five and six.
    NoRecordAtAll,
    /// A reading, with how many jobs were rendering and how many waiting.
    Holds { rendering: usize, waiting: usize },
}

impl TheLastKnownReading {
    fn what_it_holds(self) -> WhatTheLastKnownRecordHolds {
        match self {
            TheLastKnownReading::NoRecordAtAll => WhatTheLastKnownRecordHolds::NoRecordAtAll,
            TheLastKnownReading::Holds {
                rendering: 0,
                waiting: 0,
            } => WhatTheLastKnownRecordHolds::NothingQueuedAndNothingRunning,
            TheLastKnownReading::Holds { .. } => WhatTheLastKnownRecordHolds::QueuedOrRunningWork,
        }
    }
}

/// The removal box, composed from `Servers` W19's table with the two facts
/// handed in — **so every one of its six rows can be driven at the desk**.
pub fn the_removal_box_from(
    row: &ServerRow,
    all: &[ServerRow],
    of: WhatIsTrueOfTheServer,
    reading: TheLastKnownReading,
    already_switched_off: bool,
) -> TheRemovalBox {
    let warning = removal_warning(of, reading.what_it_holds());
    let (rendering, waiting) = match reading {
        TheLastKnownReading::Holds { rendering, waiting } => (rendering, waiting),
        TheLastKnownReading::NoRecordAtAll => (0, 0),
    };
    let last_known = matches!(warning.warns, Warns::FromWhatSmoozyLastKnew);
    TheRemovalBox {
        heading: Words::asked(words::REMOVE, what_a_server_is_called_among(row, all)),
        said: Prose::new(words::SMOOZY_FORGETS_IT),
        // **Composed from the last-known reading's own counts, as words beside
        // values (X7).** **Row three speaks of what Smoozy last knew and says
        // so** — `Servers` W19, and `Design/Servers § Removing a server` as
        // carried by step 10a round 1's first fix-upstream item.
        the_warning: match warning.warns {
            Warns::NotAtAll => None,
            Warns::FromTheReading => Some(the_warning_from_the_reading(rendering, waiting)),
            Warns::FromWhatSmoozyLastKnew => {
                Some(the_warning_from_what_smoozy_last_knew(rendering, waiting))
            }
        },
        offers: Verbatim::new(match warning.offers {
            Offers::BothChoices => "both",
            Offers::LeaveItBeOnly => "leave-it-be-only",
            Offers::APlainConfirmation => "a-plain-confirmation",
        }),
        leave_it_running: matches!(
            warning.offers,
            Offers::BothChoices | Offers::LeaveItBeOnly
        )
        .then(|| TheChoice {
            head: Prose::new(words::LEAVE_IT_RUNNING),
            said: Words::plain(if last_known {
                words::LEAVE_IT_RUNNING_SAID_LAST_KNOWN
            } else {
                words::LEAVE_IT_RUNNING_SAID
            }),
            act: Verbatim::new("leave-it-running"),
        }),
        stop_and_clear: matches!(warning.offers, Offers::BothChoices).then(|| TheChoice {
            head: Prose::new(words::STOP_AND_CLEAR),
            // **`Servers` W19a bounds what may be said before the act**: the
            // frame being rendered ***may not arrive***, and never that it *is
            // lost* — the flat form is false in the one case anybody has
            // measured.
            said: the_stop_and_clear_statement(rendering, waiting),
            act: Verbatim::new("stop-and-clear"),
        }),
        the_switch_off: warning.may_be_switched_off.then(|| TheSwitchOff {
            label: Prose::new(words::DONT_WARN_AGAIN),
            what_it_covers: Prose::new(words::WHAT_IT_COVERS),
            already: already_switched_off,
        }),
        cancel: Prose::new(words::CANCEL),
        remove: matches!(warning.offers, Offers::APlainConfirmation)
            .then(|| Prose::new(words::REMOVE)),
    }
}

/// **`Servers` W19 row one: *warns from the reading*.** *1 job is rendering and
/// 2 are waiting on this server.* — three runs, each a figure and its words.
fn the_warning_from_the_reading(rendering: usize, waiting: usize) -> Vec<Words> {
    // **A nought half is left out** — *1 job is rendering on this server.*,
    // *2 jobs are waiting on this server.* Step 10c's eleventh-pass fix piece 6.
    let mut said = match (rendering, waiting) {
        (_, 0) => vec![Words::after(
            rendering.to_string(),
            if rendering == 1 {
                words::ONE_JOB_IS_RENDERING_ALONE
            } else {
                words::JOBS_ARE_RENDERING_ALONE
            },
        )],
        (0, _) => vec![Words::after(
            waiting.to_string(),
            if waiting == 1 {
                words::ONE_JOB_IS_WAITING
            } else {
                words::JOBS_ARE_WAITING
            },
        )],
        _ => vec![
            Words::after(
                rendering.to_string(),
                if rendering == 1 {
                    words::ONE_JOB_IS_RENDERING
                } else {
                    words::JOBS_ARE_RENDERING
                },
            ),
            Words::after(
                waiting.to_string(),
                if waiting == 1 {
                    words::ONE_IS_WAITING
                } else {
                    words::MANY_ARE_WAITING
                },
            ),
        ],
    };
    said.push(Words::plain(words::RENDERING_AND_WAITING));
    said
}

/// **`Servers` W19 row three: *warns from what Smoozy last knew, saying so*.**
/// *When last connected, 1 job was rendering and 2 were waiting.*
fn the_warning_from_what_smoozy_last_knew(rendering: usize, waiting: usize) -> Vec<Words> {
    let mut said = vec![Words::plain(words::WHEN_SMOOZY_WAS_LAST_CONNECTED)];
    // **A nought half is left out**, as in the warning from the reading. Step
    // 10c's eleventh-pass fix piece 6.
    match (rendering, waiting) {
        (_, 0) => said.push(Words::after(
            rendering.to_string(),
            if rendering == 1 {
                words::ONE_JOB_WAS_RENDERING_ALONE
            } else {
                words::JOBS_WERE_RENDERING_ALONE
            },
        )),
        (0, _) => said.push(Words::after(
            waiting.to_string(),
            if waiting == 1 {
                words::ONE_JOB_WAS_WAITING
            } else {
                words::JOBS_WERE_WAITING
            },
        )),
        _ => {
            said.push(Words::after(
                rendering.to_string(),
                if rendering == 1 {
                    words::ONE_JOB_WAS_RENDERING
                } else {
                    words::JOBS_WERE_RENDERING
                },
            ));
            said.push(Words::after(
                waiting.to_string(),
                if waiting == 1 {
                    words::ONE_WAS_WAITING
                } else {
                    words::MANY_WERE_WAITING
                },
            ));
        }
    }
    said
}

// ════════════════════════════════════════════════════════════════════════════
// The acts that run whole
// ════════════════════════════════════════════════════════════════════════════

/// **What the Add dialog hands over**, as the user typed and ticked it.
pub struct TheAdd<'a> {
    pub host: &'a str,
    pub port: &'a str,
    pub username: &'a str,
    pub key_identifier: &'a str,
    pub label: &'a str,
    pub colour: &'a str,
    /// **The two setup options, each on its own** — `Servers` B1 records which
    /// were ticked (step 10a round 1, fix-here item 13).
    pub get_ready: bool,
    pub install_blender: bool,
}

/// **The add's save**, with the spine handed in so the whole of it can be
/// driven without the shell's own managed state.
pub fn the_add_is_performed(started: &Started, add: &TheAdd<'_>) -> TheAnswer {
    the_add_is_performed_naming_the_row(started, add).0
}

/// **The add's save, with the row it made named** — `Servers` O2's connect is
/// the caller's and it needs to know which server to open a period on.
pub fn the_add_is_performed_naming_the_row(
    started: &Started,
    add: &TheAdd<'_>,
) -> (TheAnswer, Option<String>) {
    // **H34: every value is validated before an adapter is touched**, and
    // `Servers` C5 refuses a save whose four fields are not all filled in.
    let mut needed = Vec::new();
    let host = Host::what_the_user_typed(add.host);
    if host.is_none() {
        needed.push(("host", words::HOST_IS_NEEDED));
    }
    let username = Username::what_the_user_typed(add.username);
    if username.is_none() {
        needed.push(("username", words::USERNAME_IS_NEEDED));
    }
    // **Step 10a round 1, fix-here item 2(d): a key identifier that names no key
    // row, or a key row with no fingerprint, is refused on the key field**, in
    // the refusal that field already has.
    let key = the_key_reference(started, add.key_identifier, None);
    if key.is_none() {
        needed.push(("key", words::A_KEY_IS_NEEDED));
    }
    if !needed.is_empty() {
        return (TheAnswer::TheseFieldsAreNeeded(needed), None);
    }
    let Some(port) = the_port_typed(add.port) else {
        return (TheAnswer::ThePortWillNotDo, None);
    };

    let Some(record) = started.record.as_ref() else {
        return (TheAnswer::TheRecordWouldNotTakeIt, None);
    };
    let Ok(identifier) = Identity::issue(IdentityKind::LocalServer) else {
        return (TheAnswer::TheRecordWouldNotTakeIt, None);
    };

    let mut row = ServerRow::saved(
        identifier.as_text(),
        host.expect("the host is filled in"),
        port,
        username.expect("the login is filled in"),
        key.expect("the key is filled in"),
    );
    let named = add.label.trim();
    row.label = (!named.is_empty()).then(|| named.to_string());
    row.colour = the_badge(Some(add.colour)).map(|badge| badge.to_string());

    // **`Servers` B1: the bit is written as part of the same durable write that
    // saves the server**, where a setup option was ticked and the period never
    // got a live connection — which is every add, because the add opens no
    // connection (`Servers` B2's *never reached the machine*): connecting here
    // would clear the remembered setup with nowhere to hand it.
    let ticked: Vec<String> = [
        (add.get_ready, GET_THE_SERVER_READY),
        (add.install_blender, INSTALL_THE_LATEST_STABLE_BLENDER),
    ]
    .into_iter()
    .filter(|(on, _)| *on)
    .map(|(_, option)| option.to_string())
    .collect();
    let a_setup_option_was_ticked = !ticked.is_empty();
    if smoozy_servers::setup_bit::the_bit_is_written(
        a_setup_option_was_ticked,
        smoozy_servers::setup_bit::WhereTheAddGotTo::TheConnectionNeverReachedTheMachine,
    ) {
        row.remembered_setup = Some(smoozy_servers::store::RememberedSetup { options: ticked });
    }

    // **One durable write, and the server is always saved whatever the other
    // two units do** — `Servers` §14's *Add* row.
    match server_rows::add(record.connection(), &row) {
        // **`Servers` O2: an add with a setup option ticked connects right
        // away to carry the setup out**, which is the Add dialog's own
        // sentence — *"Either option connects immediately."* **The connect
        // is the caller's**, because this function is handed the spine rather
        // than the application and opening a period wants the application.
        // *(Until step 10c piece 9 the add connected nothing, ticked or not,
        // and said so: `Servers` O2 with `Servers` B5 item 1 lets a tick
        // connect only with the setup hand-over, and the hand-over was not
        // built.)*
        Ok(()) if a_setup_option_was_ticked => (
            TheAnswer::ItWasSavedAndNothingStarted,
            Some(row.local_server_identifier.clone()),
        ),
        Ok(()) => (TheAnswer::ItWasSaved, None),
        Err(_) => (TheAnswer::TheRecordWouldNotTakeIt, None),
    }
}

/// **The edit's save.** `Servers` D16: the whole of an edit's write is one
/// durable write, made before the put-back is attempted.
///
/// **`store` is the secret store the remembered machine password is erased
/// through** (`Servers` D14 and `Servers` D18), handed in so a test drives a stand-in and
/// never the real one.
#[allow(clippy::too_many_arguments)]
pub fn the_edit_is_performed(
    started: &Started,
    store: Option<&dyn SecretStore>,
    server: &str,
    host: &str,
    port: &str,
    username: &str,
    key_identifier: &str,
    label: &str,
    fallback_folder: Option<&str>,
) -> TheAnswer {
    let Some(record) = started.record.as_ref() else {
        return TheAnswer::TheRecordWouldNotTakeIt;
    };
    let row = match server_rows::one(record.connection(), server) {
        Ok(Some(row)) => row,
        // **A server another act removed is said as that**, and never as a
        // refused write (step 10a round 1, fix-here item 12).
        Ok(None) => return TheAnswer::TheServerIsGone,
        Err(_) => return TheAnswer::TheRecordWouldNotTakeIt,
    };
    let mut row = row;

    let mut needed = Vec::new();
    let new_host = Host::what_the_user_typed(host);
    if new_host.is_none() {
        needed.push(("host", words::HOST_IS_NEEDED));
    }
    let new_username = Username::what_the_user_typed(username);
    if new_username.is_none() {
        needed.push(("username", words::USERNAME_IS_NEEDED));
    }
    let new_key = the_key_reference(started, key_identifier, Some(&row.key));
    if new_key.is_none() {
        needed.push(("key", words::A_KEY_IS_NEEDED));
    }
    if !needed.is_empty() {
        return TheAnswer::TheseFieldsAreNeeded(needed);
    }
    let (Some(new_host), Some(new_username), Some(new_key)) = (new_host, new_username, new_key)
    else {
        return TheAnswer::TheRecordWouldNotTakeIt;
    };
    let Some(new_port) = the_port_typed(port) else {
        return TheAnswer::ThePortWillNotDo;
    };
    // **`Servers` D3's fallback folder: a path that does not start with `/` is
    // refused where it is typed, and never written** (the build runner's
    // ruling of 2026-10-04). **Nothing handed in leaves the row's as it is** —
    // the rename on the row saves through here and carries no folder.
    let new_fallback_folder = match fallback_folder.map(the_fallback_folder_typed) {
        None => row.chosen_fallback_folder.clone(),
        Some(Ok(typed)) => typed,
        Some(Err(())) => return TheAnswer::TheFallbackFolderWillNotDo,
    };

    // **`Servers` D12: the rules are cut on the field that changed, not on
    // *the connection details changed*.** An answer that treats the edit as one
    // event gets three of the four wrong — **and an edit that changes two
    // fields is two rows of that table, both applied** (step 10a round 1,
    // fix-here item 3).
    let mut changed = what_changed(&row, &new_host, new_port, &new_username, key_identifier, label);
    // **`Servers` E3 as widened 2026-10-04: changing the fallback folder ends
    // no period and carries no warning**, and every `Servers` D12 edit
    // keeps it.
    if new_fallback_folder != row.chosen_fallback_folder {
        changed.push(WhatChanged::TheFallbackFolder);
    }
    if changed.is_empty() {
        // Nothing moved. Nothing is written and nothing is ended.
        return TheAnswer::ItWasSaved;
    }

    // **`Servers` D14 and `Servers` D18: the remembered machine password's erase is an
    // enumerated step of the edit's own write**, through `Security and
    // licence` §5.1's three writes. The reference is dropped from the row
    // whatever the store answered, and an erase the store did not confirm
    // leaves the owed-erase mark for the sweep.
    let the_password_goes = changed.iter().any(|one| {
        matches!(
            smoozy_servers::edits::what_the_edit_throws_away(*one).the_remembered_machine_password,
            smoozy_servers::edits::Fate::ClearedWithoutAsking
        )
    });
    if the_password_goes {
        if let Some(subject) = row.machine_password_subject.clone() {
            if the_password_is_erased(record.connection(), store, subject).is_err() {
                return TheAnswer::TheRecordWouldNotTakeIt;
            }
        }
    }

    for one in &changed {
        apply_what_the_edit_throws_away(
            &mut row,
            smoozy_servers::edits::what_the_edit_throws_away(*one),
        );
    }

    row.host = new_host;
    row.port = new_port;
    row.username = new_username;
    row.key = new_key;
    let named = label.trim();
    row.label = (!named.is_empty()).then(|| named.to_string());
    row.chosen_fallback_folder = new_fallback_folder;

    match server_rows::write(record.connection(), &row) {
        // **The save landed.** The put-back is not this function's: it is
        // opened by the old period's own thread where a period stood
        // (`Servers` E1 and E2, `src/servers/a_period.rs`), and where none
        // stood the server is left as it was, with the new details saved.
        Ok(()) => TheAnswer::ItWasSaved,
        Err(_) => TheAnswer::TheRecordWouldNotTakeIt,
    }
}

/// **`Servers` D3's fallback folder, as typed**: white space around it
/// removed; empty is *none set*, which is Smoozy's own job area (`Job files
/// and outputs` P23); **a path that does not start with `/` is refused** —
/// the one check the dialog makes. Whether the folder can be used is found at
/// render start.
pub fn the_fallback_folder_typed(typed: &str) -> Result<Option<String>, ()> {
    let typed = typed.trim();
    if typed.is_empty() {
        return Ok(None);
    }
    if !typed.starts_with('/') {
        return Err(());
    }
    Ok(Some(typed.to_string()))
}

/// **`Servers` D12's cut, over the fields the dialog can change — every one
/// that changed, and not only the first.**
///
/// **Host and port are one case** — `Servers` D13: together they name what
/// Smoozy is reaching. Step 10a round 1, fix-here item 3: *"work out the full
/// set of changed fields"* — the single answer this used to give let a key
/// change past the lock behind a port change, and kept a remembered password
/// behind a host change made with a username change.
pub fn what_changed(
    row: &ServerRow,
    host: &Host,
    port: Port,
    username: &Username,
    key_identifier: &str,
    label: &str,
) -> Vec<WhatChanged> {
    let mut changed = Vec::new();
    if &row.host != host || row.port != port {
        changed.push(WhatChanged::HostOrPort);
    }
    if &row.username != username {
        changed.push(WhatChanged::Username);
    }
    if row.key.local_key_identifier != key_identifier.trim() {
        changed.push(WhatChanged::TheKey);
    }
    let named = label.trim();
    let was = row.label.as_deref().map(str::trim).unwrap_or_default();
    if was != named {
        changed.push(WhatChanged::TheLabelOrTheColour);
    }
    changed
}

/// **`Servers` D12's four fates, applied to the row before it is written.**
fn apply_what_the_edit_throws_away(
    row: &mut ServerRow,
    throws_away: smoozy_servers::edits::WhatTheEditThrowsAway,
) {
    use smoozy_servers::edits::Fate;
    // **`Servers` D13: all four go in the same write** — the hardware reading, the event
    // cursor, the confirmed installation identity, and the machine's root.
    if matches!(throws_away.the_machine_derived_values, Fate::Dropped) {
        row.remembered_hardware_reading = None;
        row.event_cursor = None;
        row.last_confirmed_installation_identity = None;
        row.recorded_machine_root = None;
        // **And the driver's missed bar (`Servers` D3's row of 2026-10-11)**:
        // a swap that missed on the old machine says nothing about the new
        // one, so it goes with the reading it was told beside.
        row.the_driver_missed_the_optix_bar = false;
    }
    // **`Servers` D8: started over — the field is cleared, so the next connection is a
    // first connection and trusts silently.**
    if matches!(throws_away.the_remembered_host_key, Fate::StartedOver) {
        row.remembered_host_key = None;
    }
    // **`Servers` D14: cleared without asking, because a password belongs to a login.**
    // **The value's erase through the secret adapter is `Servers` D18's**, and
    // it is taken by [`the_edit_is_performed`] before this runs; what is
    // dropped here is the row's reference.
    if matches!(throws_away.the_remembered_machine_password, Fate::ClearedWithoutAsking) {
        row.machine_password_subject = None;
    }
    // **`Servers` B7: the remembered setup survives every edit**, which is what it is for.
}

/// **`Servers` D18: the remembered machine password is erased through the
/// secret store, in `Security and licence` §5.1's order** — the owed-erase mark
/// first, then the store, then the reference and the mark together.
///
/// **A store that did not confirm the erase is not a refusal**: the reference
/// goes, the mark stands, and the sweep retries (§5.1 E3). **Where there is no
/// store at all**, the erase is the same unconfirmed case. **Only a mark that
/// would not write stops the act**, because an erase with no mark behind it is
/// exactly what §5.1's first write exists to prevent. Step 10a round 1,
/// fix-here items 11 and 19.
fn the_password_is_erased(
    connection: &rusqlite::Connection,
    store: Option<&dyn SecretStore>,
    subject: smoozy_security_and_licence::held::Subject,
) -> Result<(), ()> {
    use smoozy_security_and_licence::erasure::{erase, EraseMarks, Erasure};
    use smoozy_security_and_licence::held::{HeldSecret, StoredKind};
    let item = HeldSecret::new(StoredKind::MachinePassword, subject);
    let mut marks = server_rows::ServerRowMarks(connection);
    let Some(store) = store else {
        // Nowhere to reach: the same as a store that did not confirm.
        marks.mark_owing_an_erase(&item).map_err(|_| ())?;
        return marks.remove_reference(&item).map_err(|_| ());
    };
    match erase(&mut marks, store, &item, THE_STORE_DEADLINE) {
        Erasure::Done | Erasure::ReferenceGoneAndOwedToTheSweep(_) => Ok(()),
        Erasure::NothingDoneTheMarkWouldNotWrite => Err(()),
    }
}

/// **How long a store call is waited on** — `Security and licence` §3.0 rule
/// 5, and the figure `crate::surface::keys` already uses for a call the user is
/// standing in front of.
const THE_STORE_DEADLINE: std::time::Duration = std::time::Duration::from_secs(5);

/// **The Edit dialog's *Clear*, performed** — `Servers` D18.
pub fn the_password_is_forgotten(
    started: &Started,
    store: Option<&dyn SecretStore>,
    server: &str,
) -> TheAnswer {
    let Some(record) = started.record.as_ref() else {
        return TheAnswer::TheRecordWouldNotTakeIt;
    };
    let row = match server_rows::one(record.connection(), server) {
        Ok(Some(row)) => row,
        Ok(None) => return TheAnswer::TheServerIsGone,
        Err(_) => return TheAnswer::TheRecordWouldNotTakeIt,
    };
    let Some(subject) = row.machine_password_subject.clone() else {
        return TheAnswer::ThePasswordWasForgotten;
    };
    match the_password_is_erased(record.connection(), store, subject) {
        Ok(()) => TheAnswer::ThePasswordWasForgotten,
        Err(()) => TheAnswer::TheRecordWouldNotTakeIt,
    }
}

/// **A removal that reaches no machine, and the machine-reaching removal's
/// last unit** — `Servers` D19 and §14, and `Servers` W25's *local forget*,
/// *"atomic entire"*.
///
/// **The remembered machine password is erased first**, which is `Servers`
/// `Servers` D18's *"the caller's step before this one"* — a leftover would be *"a
/// remembered secret against a machine the user believes Smoozy has
/// forgotten"* (step 10a round 1, fix-here item 11). **A store failure never
/// blocks the removal**: the mark stands and the sweep retries.
///
/// *(Until step 10c's S2 this also took whether the removal reached the
/// machine, and answered that it was not built where it did. The
/// machine-reaching removal is [`stop_and_clear_first`], which calls this for
/// its third unit.)*
pub fn the_removal_is_performed(
    started: &Started,
    store: Option<&dyn SecretStore>,
    server: &str,
    dont_warn_again: bool,
) -> TheAnswer {
    let Some(record) = started.record.as_ref() else {
        return TheAnswer::TheRecordWouldNotTakeIt;
    };
    // **`Servers` W20's one row, written only where the user ticked it, and
    // the removal carries on whatever it comes to.**
    //
    // **The switch-off and the removal are two units and the user asked for
    // both**; a tick that could not be written is not a reason to keep them
    // from the removal they came for. **§15 G5's hosting rule is what may
    // refuse it**: `Servers` declares no dismissible warning in its own crate
    // and is not in `start.rs`'s register, so a suppression written for this
    // identity is refused today. **What that costs is bounded and loud** — the
    // user ticks the switch-off and the warning comes back next time, which is
    // visible at once rather than the invisible failure G2 is written about,
    // and it is the same shape `start.rs` already records for
    // `rendering.missing-assets`. **What closes it is the declaration, in
    // `App/Servers/`'s own crate**, and it is recorded rather than reached for
    // from here.
    if dont_warn_again {
        crate::surface::said_once::a_warning_was_switched_off(
            started,
            THIS_MODULE,
            THE_DISCONNECTED_REMOVAL_WARNING,
        );
    }
    let row = match server_rows::one(record.connection(), server) {
        Ok(Some(row)) => row,
        Ok(None) => return TheAnswer::TheServerIsGone,
        Err(_) => return TheAnswer::TheRecordWouldNotTakeIt,
    };
    if let Some(subject) = row.machine_password_subject.clone() {
        if the_password_is_erased(record.connection(), store, subject).is_err() {
            return TheAnswer::TheRecordWouldNotTakeIt;
        }
    }
    // **`Servers` D17: the walk is the enumeration and not a list kept in step by hand.**
    // **The walk answers with the kinds whose module has not been built**, and
    // that is L1's reporting rather than a failure — a kind with no table is a
    // module that has not landed, not a record that would not take the write.
    match server_rows::remove_walking_the_enumeration(record.connection(), &row.local_server_identifier) {
        Ok(_) => {
            // **An owed `sv-sc` box goes with its server** — the user answered
            // *remove* (`Servers` W22).
            if let Ok(mut owed) = THE_ENDINGS_OWED.lock() {
                owed.retain(|(held, _)| held != server);
            }
            TheAnswer::ItWasSaved
        }
        Err(_) => TheAnswer::TheRecordWouldNotTakeIt,
    }
}

/// **The removal, and the drawings of what its forget takes go with it** —
/// `State, identity and recovery` F25's first ending, *"its subject is
/// removed"*, and `Desktop app and delivery` M4. Both removals the user can
/// press end here: [`a_server_is_removed`] and *Stop and clear first*'s third
/// unit.
///
/// **Written for**: the forget alone ([`the_removal_is_performed`]). **Now
/// also meets**: a condition standing against the row when it goes — a server
/// removed while its line is cut. **Where the record and the world parted
/// company**: the forget deleted the row's failure records inside its walk
/// and told no page, so the window's foot kept *Something is not working.*
/// with nothing behind it (step 10c's twelfth pass, fault 13; twelfth-pass
/// fix piece 7).
///
/// **What it takes is read under the same hold of the run as the forget**, so
/// nothing can be raised between the two, and the drawings go only once the
/// forget has landed (P7).
///
/// **The posts raised from what it took come down with it** — T24b's *its
/// subject is removed*, on the same identity Q10 replaces on, attempted and
/// forgotten as T24a's are (T23), and each key spent as Q12 spends it.
/// **Built 2026-10-24**, on the third finding of step 12's destroy scenario:
/// the forget deleted the condition record with the row and nothing took its
/// posted *could not be restored* down, so the system went on holding it for
/// a server Smoozy no longer has. `notifications` is `None` only where no
/// notification service runs.
pub fn the_removal_is_performed_and_its_drawings_go(
    started: &Started,
    store: Option<&dyn SecretStore>,
    queue: Option<&TheOneQueue>,
    notifications: Option<&crate::notifications::TheNotificationService>,
    server: &str,
    dont_warn_again: bool,
) -> TheAnswer {
    let taken = crate::servers::published::what_the_forget_will_take(started, server);
    let answer = the_removal_is_performed(started, store, server, dont_warn_again);
    if !matches!(answer, TheAnswer::ItWasSaved) {
        return answer;
    }
    if let Some(notifications) = notifications {
        for condition in &taken {
            crate::surface::said_once::the_condition_record_ended(
                &started.said_once,
                notifications,
                condition,
                crate::notifications::HowTheRecordEnded::ItsSubjectWasRemoved,
            );
        }
    }
    if let Some(queue) = queue {
        crate::servers::published::the_drawings_go_with_the_row(queue, server, &taken);
    }
    answer
}

/// **Both removals the user can press end here**: the removal and its
/// drawings ([`the_removal_is_performed_and_its_drawings_go`]), then **History
/// drawn again at once** where `history` is given, so the removed server's
/// jobs move into *Removed servers* while the user watches (step 11's D6b,
/// the eighteenth notes' piece 7 *For the runner*). Until then History moved
/// them only at its next draw, which for a server not connected was the next
/// time it was opened or acted on, since no connection changed.
#[allow(clippy::too_many_arguments)]
pub fn a_removal_lands(
    started: &Started,
    store: Option<&dyn SecretStore>,
    queue: Option<&TheOneQueue>,
    notifications: Option<&crate::notifications::TheNotificationService>,
    server: &str,
    dont_warn_again: bool,
    history: Option<&crate::surface::the_history::WhatHistoryIsDrawnOver>,
) -> TheAnswer {
    let answer = the_removal_is_performed_and_its_drawings_go(
        started,
        store,
        queue,
        notifications,
        server,
        dont_warn_again,
    );
    if let (TheAnswer::ItWasSaved, Some(queue), Some(history)) = (&answer, queue, history) {
        crate::surface::the_history::the_history_is_drawn_again(queue, started.record.as_ref(), history);
    }
    answer
}

// ════════════════════════════════════════════════════════════════════════════
// The event
// ════════════════════════════════════════════════════════════════════════════

/// **The one way this file puts an event on the queue.**
///
/// P7 gives the queue two doors and both run a durable act first. **An answer
/// to the user's last act has no durable act of its own** — it is a statement
/// about what just happened, and the act it answers has already run or has
/// already been refused — so it goes through the reading door with nothing to
/// read, which is the same thing `src/surface/access.rs` does with its own
/// answers.
fn the_queue_is_told(queue: &TheOneQueue, event: Event) {
    let _ = queue.read_then_post::<(), std::convert::Infallible>(|| Ok(()), |_| event.clone());
}

/// Prose on to the event, through the point (A6, X1).
fn prose(text: &str) -> Said {
    Said::Prose(Prose::new(text))
}

/// A value on to the event, by the verbatim route (X9).
fn value(text: impl Into<String>) -> Said {
    Said::Value(Verbatim::new(text.into()))
}

/// **The answer to the user's last act, as an event.**
///
/// **Every string is on one of the two routes and the two are the complete
/// set** (A6, X9).
pub fn the_answer_event(answer: TheAnswer) -> Event {
    let event = Event::about(About::TheRun, THE_ANSWER);
    match answer {
        TheAnswer::ItWasSaved => event.saying("what", value("it-was-saved")),
        TheAnswer::TheseFieldsAreNeeded(fields) => {
            let mut event = event
                .saying("what", value("these-fields-are-needed"))
                .saying("fieldCount", value(fields.len().to_string()));
            for (at, (field, said)) in fields.iter().enumerate() {
                let (which, what) = THE_FIELDS_OWN_KEYS[at.min(THE_FIELDS_OWN_KEYS.len() - 1)];
                event = event
                    .saying(which, value(*field))
                    .saying(what, prose(said));
            }
            event
        }
        TheAnswer::ThePortWillNotDo => event
            .saying("what", value("the-port-will-not-do"))
            .saying("thePortWillNotDo", prose(words::THE_PORT_RANGE)),
        TheAnswer::TheFallbackFolderWillNotDo => event
            .saying("what", value("the-fallback-folder-will-not-do"))
            .saying("theFallbackFolderWillNotDo", prose(words::ENTER_A_FULL_PATH)),
        // **Each case its own sentence** — step 10a round 1, fix-here item 12:
        // a refused write is not *Not available yet*.
        TheAnswer::TheRecordWouldNotTakeIt => event
            .saying("what", value("the-record-would-not-take-it"))
            .saying("said", prose(words::THE_CHANGE_WAS_NOT_SAVED)),
        TheAnswer::TheServerIsGone => event
            .saying("what", value("the-server-is-gone"))
            .saying("said", prose(words::THIS_SERVER_HAS_BEEN_REMOVED)),
        TheAnswer::TheListCouldNotBeRead => event
            .saying("what", value("the-list-could-not-be-read"))
            .saying("said", prose(words::THE_LIST_COULD_NOT_BE_READ)),
        TheAnswer::TheActIsNotBuiltYet => event
            .saying("what", value("the-act-is-not-built-yet"))
            .saying("said", prose(words::NOT_BUILT_YET)),
        // **Nothing said, as for a plain add.** Since step 10c piece 9 a ticked
        // add connects and sets up (`Servers` O2), and the list and the setup
        // say what is happening. **Written for one case and now meeting the
        // other**: it said *Smoozy cannot do this yet* when the add connected
        // nothing, and went on saying it after piece 9 made the add do
        // everything — step 10c's eleventh pass read it above a server that
        // was placing Smoozy and installing Blender (fault 5). Step 10c's
        // eleventh-pass fix piece 6.
        TheAnswer::ItWasSavedAndNothingStarted => {
            event.saying("what", value("it-was-saved-and-nothing-started"))
        }
        TheAnswer::ThePasswordWasForgotten => {
            event.saying("what", value("the-password-was-forgotten"))
        }
        TheAnswer::TheConnectionWasNotMade => event
            .saying("what", value(THE_CONNECTION_WAS_NOT_MADE_ANSWER))
            .saying("said", prose(words::THE_CONNECTION_WAS_NOT_MADE)),
        TheAnswer::ItIsNotALinuxMachine => event
            .saying("what", value(IT_IS_NOT_A_LINUX_MACHINE_ANSWER))
            .saying("said", prose(words::IT_IS_NOT_A_LINUX_MACHINE)),
        TheAnswer::TheKeysPassphraseCouldNotBeUsed(said) => event
            .saying("what", value(THE_PASSPHRASE_COULD_NOT_BE_USED_ANSWER))
            .saying("said", prose(said)),
        TheAnswer::TheUpdateNowWasTaken => event.saying("what", value("the-update-now-was-taken")),
        // **Nothing said**: the list, asked for again, carries the ending.
        TheAnswer::TheStopAndClearEndingIsOwed => {
            event.saying("what", value("the-stop-and-clear-ending-is-owed"))
        }
        // **The removal box's own ending, in its own two sentences** —
        // `Servers` W21, whose rows are `Queue & History` L17's verbatim.
        TheAnswer::TheStopAndClearDidNotFullyWork(ending) => {
            let said = ending.said();
            event
                .saying("what", value("the-stop-and-clear-did-not-fully-work"))
                .saying("which", Said::Value(said.which))
                .saying("said", Said::Prose(said.lead))
                .saying("saidNext", Said::Prose(said.rest))
        }
    }
}

// ── A connect that did not happen, said where the user is ──────────────────
//
// **The Creator's note 14 on the test copy** (2026-10-03): *"Deliberately chose
// the wrong key. It didn't connect, didn't tell me that anything is wrong,
// nothing."* *Add and set up* had moved the user to the new server's Set up
// tab (`Design/Servers § Adding a server`, 2026-10-02), and the sentence was
// written above the All servers list, which was hidden. **Rule 8: Smoozy going
// quiet where the user is looking is a fault whoever caused the trouble.**
//
// **So the three answers a connect can end in name their server**, and the
// page keeps the newest for each server it is told of — above the All servers
// list, naming the server, and on that server's Set up card
// (`Design/Server Setup § This server`, `Design/Servers § Adding a server`,
// `Design/Keys`' line 143; the map's holes 1 and 2). **Nothing is written to
// the record or the diary**: it is gone at the next act on that server and
// when Smoozy closes.

/// **The three answers a connect that never became live can end in**, as the
/// page reads them. Their one home: the event above writes them, the page
/// routes on them, and the Set up tab is handed one back
/// ([`what_a_failed_connect_said`]).
pub const THE_CONNECTION_WAS_NOT_MADE_ANSWER: &str = "the-connection-was-not-made";
pub const IT_IS_NOT_A_LINUX_MACHINE_ANSWER: &str = "it-is-not-a-linux-machine";
pub const THE_PASSPHRASE_COULD_NOT_BE_USED_ANSWER: &str = "the-passphrase-could-not-be-used";

/// **The server a connect was about, as the list names it.** The name is the
/// row's own — [`what_a_server_is_called_among`] — so the line above the list
/// and the row under it say the same thing. **Absent where the row could not
/// be read**, and the line then falls back to the sentence that names no
/// server rather than inventing a name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheServerItWasAbout {
    pub server: String,
    pub name: Option<String>,
}

/// **The answer to a connect that did not happen, naming its server** — the
/// event [`the_answer_event`] composes, with the server's identity beside it
/// as a value, and for the claimless sentence the lead and the name the line
/// above the list is drawn from. **The Set up card keeps *this server*** — the
/// event's own `said` — because there the server is the one on screen.
///
/// **Every other answer goes out exactly as [`the_answer_event`] composes
/// it**: those are about an act on this screen, not about a server's
/// connection.
pub fn the_answer_about(answer: TheAnswer, about: &TheServerItWasAbout) -> Event {
    let names_the_server = matches!(
        answer,
        TheAnswer::TheConnectionWasNotMade
            | TheAnswer::ItIsNotALinuxMachine
            | TheAnswer::TheKeysPassphraseCouldNotBeUsed(_)
    );
    let claimless = answer == TheAnswer::TheConnectionWasNotMade;
    let event = the_answer_event(answer);
    if !names_the_server {
        return event;
    }
    let event = event.saying("server", value(about.server.clone()));
    match (&about.name, claimless) {
        (Some(name), true) => event
            .saying("couldNotConnectTo", prose(words::COULD_NOT_CONNECT_TO))
            .saying("couldNotConnectToNamed", value(name.clone())),
        _ => event,
    }
}

/// **What a connect that ended said, from the page's own record of it** —
/// the Set up tab is handed the answer's `what` back, and this is the sentence
/// its card says (`Design/Server Setup § This server`: *"Where the connection
/// met a cause Keys has its own sentence for, the card says that sentence
/// instead, as All servers does"*). **A `what` that is not one of the three
/// says nothing**, so nothing the page hands in can put a word on the card.
///
/// **The passphrase sentence is the one this system's supply can give**: the
/// channel is refused for its path's length on a Unix system, and on Windows
/// for the limit (`crate::keys::askpass`), and no other sentence is ever said
/// under this answer.
pub fn what_a_failed_connect_said(what: &str) -> Option<&'static str> {
    match what {
        THE_CONNECTION_WAS_NOT_MADE_ANSWER => Some(words::THE_CONNECTION_WAS_NOT_MADE),
        IT_IS_NOT_A_LINUX_MACHINE_ANSWER => Some(words::IT_IS_NOT_A_LINUX_MACHINE),
        THE_PASSPHRASE_COULD_NOT_BE_USED_ANSWER => Some(THE_PASSPHRASE_SENTENCE_ON_THIS_SYSTEM),
        _ => None,
    }
}

/// **The one sentence this system's supply gives where a passphrase cannot be
/// handed to the SSH tools** — `crate::keys::askpass`'s own, chosen the way
/// that file chooses it.
#[cfg(unix)]
pub const THE_PASSPHRASE_SENTENCE_ON_THIS_SYSTEM: &str = crate::keys::askpass::THE_PATH_IS_TOO_LONG;
#[cfg(not(unix))]
pub const THE_PASSPHRASE_SENTENCE_ON_THIS_SYSTEM: &str = crate::keys::askpass::THE_WINDOWS_LIMIT;

/// **What a period that has ended says, and mostly it says nothing** — the
/// choice [`the_period_ended`] makes, kept apart from the application so it
/// can be read and tested on its own.
pub fn what_an_ended_period_says(
    left: WhatTheDrivingLeft,
    the_box_was_raised: bool,
    the_passphrase_could_not_be_used: Option<&'static str>,
) -> Option<TheAnswer> {
    match left {
        // **`Servers` Q13 outcome 2, and no passphrase box can be standing
        // beside it.** The box is raised where nothing was held to supply, and
        // an attempt that supplied nothing never reaches the not-Linux check —
        // so this is not the claimless sentence's case with a different word in
        // it, it is a different ending.
        WhatTheDrivingLeft::ItIsNotALinuxMachine => Some(TheAnswer::ItIsNotALinuxMachine),
        // **`Keys` S19's box is the answer**, and a sentence beside it would be
        // Smoozy saying one thing twice.
        WhatTheDrivingLeft::TheConnectionWasNotMade if the_box_was_raised => None,
        // **Where the supply knows why no passphrase could be handed over, that
        // is said instead of the claimless sentence** — L1, 2026-09-18.
        WhatTheDrivingLeft::TheConnectionWasNotMade => Some(match the_passphrase_could_not_be_used {
            Some(said) => TheAnswer::TheKeysPassphraseCouldNotBeUsed(said),
            None => TheAnswer::TheConnectionWasNotMade,
        }),
        // **`Servers` Q6's third path: the changed-identity warning is the
        // answer**, and a sentence beside it would be one act answered twice.
        WhatTheDrivingLeft::TheWarningIsTheAnswer => None,
        // **`Servers` N8b: the repair stopped for good**, and the period still
        // stands. Nothing is said here: the condition record is raised on that
        // path and the failure shape draws it.
        WhatTheDrivingLeft::TheRepairStoppedForGood => None,
        WhatTheDrivingLeft::NothingToSay => None,
    }
}

/// **The server as the list names it**, read from the record at the moment
/// the answer is composed. **The run's one lock is taken for this read alone.**
fn the_server_as_the_list_names_it<R: Runtime>(
    app: &AppHandle<R>,
    server: &str,
) -> TheServerItWasAbout {
    let name = app
        .try_state::<Mutex<Started>>()
        .and_then(|started| started.lock().ok().and_then(|started| the_rows(&started)))
        .and_then(|rows| {
            rows.iter()
                .find(|row| row.local_server_identifier == server)
                .map(|row| what_a_server_is_called_among(row, &rows))
        });
    TheServerItWasAbout {
        server: server.to_string(),
        name,
    }
}

/// **The keys one refused field travels on.** Three fields can be blank at
/// once — the host, the login and the key — and the port refuses in place
/// rather than here.
const THE_FIELDS_OWN_KEYS: [(&str, &str); 3] = [
    ("field0", "field0Said"),
    ("field1", "field1Said"),
    ("field2", "field2Said"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_servers::hardware::{Card, CardMake, FreeSpace, MachineMemory};
    use smoozy_state::observed::{Clock, Reading};
    use std::time::{Duration, SystemTime};

    /// This file's working half — everything before the tests.
    ///
    /// **The sweeps below read it rather than the whole file**, because the
    /// things they forbid are written out here in order to forbid them.
    fn the_working_half() -> &'static str {
        include_str!("servers.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half")
    }

    /// **The live Telegram pass, fault 11, carried to a rename: a server's new
    /// name reaches every server connected now, at once** — `Telegram` L2 as
    /// changed 2026-10-31. A rename ends no period (`Servers` E3), so until
    /// then a connected server went on calling itself by its old name until a
    /// reconnect. Every live connection is told, not only the one renamed,
    /// because an unnamed server's told name can change with its neighbours'
    /// (`the_name_a_machine_is_told`). **Only once the save has landed, and
    /// after the record's lock is given back.**
    #[test]
    fn fault_11_a_saved_rename_is_told_to_every_server_connected_now() {
        let working = the_working_half();
        let at = working.find("pub fn an_edit_is_saved<R: Runtime>(").expect("the command");
        let body = &working[at..at + working[at..].find("\n}\n").expect("it ends")];
        let telling = body
            .find("crate::setting_up::the_tellings_are_delivered_on_every_live_connection(&app)")
            .expect("a saved rename tells no connected server");
        let saved = body.find("TheAnswer::ItWasSaved").expect("the save's answer is read");
        let lock_let_go = body.find("the_edit_is_performed(").expect("the save");
        assert!(lock_let_go < telling && saved < telling);
        assert!(body[..telling].contains("if saved"), "told only where the save landed");
    }

    fn a_row_named(server: &str) -> ServerRow {
        let mut row = a_row();
        row.local_server_identifier = server.to_string();
        row
    }

    fn a_row() -> ServerRow {
        ServerRow::saved(
            Identity::issue(IdentityKind::LocalServer)
                .expect("randomness")
                .as_text(),
            Host::what_the_user_typed("213.181.122.44").expect("a host"),
            Port::what_the_user_typed(40122).expect("a port"),
            Username::what_the_user_typed("root").expect("a login"),
            KeyReference {
                local_key_identifier: "key-1".to_string(),
                fingerprint: "SHA256:aaaa".to_string(),
            },
        )
    }

    fn a_reading() -> HardwareReading {
        HardwareReading {
            cards: Cards::TheCards(vec![Card {
                unique_identifier: "GPU-1".to_string(),
                bus_location: "0000:01:00.0".to_string(),
                make: CardMake::from_the_drivers_own_query("NVIDIA GeForce RTX 4090"),
                total_memory_bytes: Some(24 * 1024 * 1024 * 1024),
            }]),
            machine_memory: MachineMemory::TheAllocation(79 * 1024 * 1024 * 1024),
            free_space: Some(FreeSpace {
                available_bytes: 61 * 1024 * 1024 * 1024,
                the_path_it_was_taken_against: "/smoozy".to_string(),
            }),
            the_machines_own_moment: Reading::taken(
                (),
                SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000),
                Clock::AMachine(smoozy_state::observed::MachineClock("box".to_string())),
            ),
            the_desktops_own_moment: Reading::taken(
                (),
                SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_050),
                Clock::ThisComputer,
            ),
            read_under: None,
            the_driver: None,
        }
    }

    /// **The twenty-three mockup states this piece draws are the twenty-three
    /// the drawing has, in both directions.** *(Twenty-two until 2026-10-16,
    /// when piece E3 built `sv-addkey`.)*
    ///
    /// **This is the runner's ruling of 2026-09-12 given teeth.** The gate's
    /// forward leg cannot fail on a missing screen, so the state names are what
    /// this half is counted by — and a name this file claims that the drawing
    /// does not have, or one the drawing has that this file does not claim, is
    /// what this fails on.
    #[test]
    fn the_twenty_three_states_are_the_drawings_own_twenty_three() {
        let mockup = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../Preparation/Design/Mockup/Smoozy.html"),
        )
        .expect("the drawing is readable");
        let opens = mockup
            .find("<optgroup label=\"Servers\">")
            .expect("the drawing has a Servers group");
        let group = &mockup[opens..opens + mockup[opens..].find("</optgroup>").expect("it closes")];
        let mut drawn: Vec<String> = Vec::new();
        for line in group.lines() {
            let Some(at) = line.find("<option value=\"") else {
                continue;
            };
            let rest = &line[at + "<option value=\"".len()..];
            let closes = rest.find('"').expect("the value closes");
            drawn.push(rest[..closes].to_string());
        }
        drawn.sort();

        let claimed = the_working_half();
        let mut named: Vec<String> = Vec::new();
        for state in &drawn {
            if claimed.contains(&format!("`{state}`")) {
                named.push(state.clone());
            }
        }
        assert_eq!(drawn.len(), 23, "the Servers group has twenty-three states");
        assert_eq!(
            named, drawn,
            "this file's head names the mockup states it draws and the two lists \
             have drifted. **The runner ruled on 2026-09-12 that the mockup state \
             names are what this half is counted by**, so a state the drawing has \
             and this file does not claim is a state nobody built and nothing else \
             would say so"
        );
    }

    /// **X7, over every sentence this file composes: no value is ever joined
    /// into one.**
    ///
    /// **This is the trap the whole step is written against.** A drawing that
    /// writes `format!("{} could not be reached", name)` has broken X7 and no
    /// other test anywhere would notice — the string is prose-shaped and the
    /// substitution lands silently inside a name the user copies.
    ///
    /// The check is narrow and is written out rather than glossed: **no string
    /// handed to `Prose::new` is built by `format!`**, and **the two `format!`
    /// calls in this file both compose a value out of values** and are named
    /// here so a third has to be argued for.
    #[test]
    fn no_sentence_is_built_out_of_a_value() {
        let source = the_working_half();
        for (at, _) in source.match_indices("Prose::new(") {
            let rest = &source[at..];
            let closes = rest.find(')').unwrap_or(rest.len());
            let inside = &rest[..closes];
            assert!(
                !inside.contains("format!"),
                "a sentence is composed with `format!`. **X7: the typography \
                 point never touches a job's name, a server's name, a path, a \
                 host or a port**, and a value joined in before the point is a \
                 name with a non-breaking space substituted inside it. Put the \
                 words in one field and the values in another"
            );
        }
        assert_eq!(
            source.matches("format!(").count(),
            6,
            "this file composes something with `format!` that the six named \
             values do not cover. **They are `username@host`, the same with its \
             port where two unnamed servers share it, the address, one card's \
             make and memory, and the two forms of a card line — one make times \
             a count, or every card named** — and every one composes a value out \
             of values with no prose on the path at all. *(A byte count with its \
             unit was the seventh until 2026-09-28, and is composed by the one \
             rule for a figure and its unit now.)*"
        );
    }

    /// **Every word the user reads is in `mod words` and nowhere else.**
    ///
    /// A string written straight into a composition is a word with no home, and
    /// the next piece that needs the same word writes a second copy of it —
    /// which is the fault this stage keeps meeting.
    #[test]
    fn every_word_is_in_one_place() {
        let source = the_working_half();
        let after = source
            .split("mod words {")
            .nth(1)
            .expect("the words are a module");
        let after = after
            .split("\n}\n")
            .nth(1)
            .expect("the words module closes");
        for (at, _) in after.match_indices("Prose::new(\"") {
            let rest = &after[at + "Prose::new(\"".len()..];
            let closes = rest.find('"').expect("the literal closes");
            let literal = &rest[..closes];
            assert!(
                literal.is_empty(),
                "a word is written into a composition rather than into `mod \
                 words`: {literal:?}. **Every word the user reads has one home**, \
                 and the empty string is the one exception — it is the absence of \
                 a lead rather than a word"
            );
        }
    }

    /// **`Servers` H1's three states, and the *never read* row shows that there
    /// is nothing to show rather than an empty row.**
    #[test]
    fn a_server_never_connected_says_so_rather_than_looking_empty() {
        let drawn = the_hardware(None);
        assert_eq!(drawn.state.as_str(), "never-read");
        assert!(drawn.cards.is_none());
        assert!(
            drawn.nothing_to_show.is_some(),
            "**`Design/Servers`: a server never connected shows nothing and says \
             so rather than looking empty**"
        );
    }

    /// **`Servers` H1a: a held reading taken on a connection that is not live now is
    /// *last seen*, and it carries the moment.**
    #[test]
    fn a_held_reading_is_marked_last_seen_and_carries_its_moment() {
        let held = a_reading();
        let drawn = the_hardware(Some(&held));
        assert_eq!(drawn.state.as_str(), "last-seen");
        let mark = drawn.last_seen.expect("the mark");
        assert_eq!(mark.lead.as_str(), Prose::new("last seen").as_str());
        // **H2a: the desktop's own moment**, not the machine's — the only one
        // that can be compared with the user's own now.
        assert_eq!(mark.values[0].as_str(), "1700000050");
        assert!(drawn.nothing_to_show.is_none());
    }

    /// **X7 on the hardware line: every figure is a value and every word is
    /// prose, and the two are separate fields.**
    #[test]
    fn the_hardware_line_keeps_its_figures_out_of_its_words() {
        let held = a_reading();
        let drawn = the_hardware(Some(&held));
        let cards = drawn.cards.expect("a card line");
        assert_eq!(cards.lead.as_str(), "");
        assert_eq!(cards.values[0].as_str(), "1 × NVIDIA GeForce RTX 4090, 24\u{a0}GB");
        let memory = drawn.memory.expect("the memory");
        let disk = drawn.free_disk.expect("the free disk");
        for said in [&memory, &disk] {
            for value in &said.values {
                assert!(
                    !value.as_str().contains("RAM") && !value.as_str().contains("disk"),
                    "a word has been joined into a figure"
                );
            }
        }
    }

    /// **`Servers` W40a: the line is never empty of both.**
    ///
    /// Nothing supplies the running-time figure, so `Servers` W40's statement stands in
    /// its place — which is the clause working rather than a gap.
    #[test]
    fn the_rental_line_is_never_empty_of_both() {
        let line = the_rental_line(None);
        assert!(line.running_for.is_none());
        assert!(
            line.the_statement.is_some(),
            "**`Servers` W40a: where the figure is a stated absence, `Servers` W40's statement is \
             composed in its place** — otherwise nothing the user meets says \
             anything about the cost at all from the day after the add onward"
        );
    }

    /// **Design's five menus, each with the same five slots in the same
    /// order.** Creator-stated (2026-08-13).
    #[test]
    fn every_menu_has_the_same_five_slots_in_the_same_order() {
        let menus = the_five_menus();
        assert_eq!(menus.len(), 5);
        for menu in &menus {
            // **Slot 1 is always exactly one item.**
            assert!(!menu.the_connection.label.as_str().is_empty());
            // **Slot 4 is always rename, edit, then copy address**, in that
            // order, and every one available in every shape (the Creator's
            // note 1 on 0.1.8).
            assert_eq!(menu.the_details.len(), 3);
            assert_eq!(menu.the_details[0].act.as_str(), "rename");
            assert_eq!(menu.the_details[0].label.as_str(), words::RENAME_ITEM);
            assert_eq!(menu.the_details[1].act.as_str(), "edit");
            assert_eq!(menu.the_details[2].act.as_str(), "copy-address");
            assert!(menu.the_details.iter().all(|item| !item.unavailable));
            // **Slot 5 is always remove.**
            assert_eq!(menu.removing.act.as_str(), "remove");
        }
    }

    /// A register with nothing behind it, for the tests below.
    fn an_empty_register() -> crate::servers::a_period::ThePeriodsThisRun {
        crate::servers::a_period::ThePeriodsThisRun::over(
            crate::servers::a_period::TheRunThroughTheApp::lending_nothing(),
            std::sync::Arc::new(crate::events::TheOneQueue::new()),
            std::sync::Arc::new(crate::notifications::TheNotificationService::on(Box::new(
                NothingIsPosted,
            ))),
        )
    }

    /// **Step 10b's first audit round, item 6: `Keys` S10's mark is drawn, in
    /// its two wordings, over the three standings.** Built and tested in
    /// `App/Keys/`, and nothing called it. **Never answered** says *cannot*;
    /// **never probed** and **has answered** say the ordinary miss; a standing
    /// that could not be read is not *never answered*. A key whose passphrase
    /// is available, or one `Keys` was not asked about, marks nothing.
    #[test]
    fn keys_s10s_mark_is_drawn_in_its_two_wordings_over_the_three_standings() {
        use smoozy_security_and_licence::standing::StoreStanding;
        let key = a_key_row(Some("SHA256:aaaa"));
        let mut row = a_row();
        row.key.local_key_identifier = key.identifier.as_text();
        let keys = vec![key.clone()];
        let said = |standing: Option<StoreStanding>| WhatKeysSaid {
            the_tools_are_installed: true,
            verdicts: vec![(key.identifier.as_text(), usable::Verdict::Usable)],
            the_passphrase_is_not_available: vec![key.identifier.as_text()],
            the_store_standing: standing,
        };
        let drawn = |standing: Option<StoreStanding>| {
            the_screen_from(std::slice::from_ref(&row), &keys, Some(&said(standing)), None, &Default::default()).rows[0]
                .the_key_mark
                .clone()
        };
        assert_eq!(
            drawn(Some(StoreStanding::NeverAnswered)),
            Some(Words::plain(words::THE_PASSPHRASE_CANNOT_BE_KEPT)),
            "**`Keys` S10: never answered and a protected key say *cannot***"
        );
        for standing in [
            Some(StoreStanding::NeverProbed),
            Some(StoreStanding::HasAnswered),
            None,
        ] {
            assert_eq!(
                drawn(standing),
                Some(Words::plain(words::THE_PASSPHRASE_IS_NOT_HELD)),
                "{standing:?}: **a passing miss is never worded as a permanent one**"
            );
        }
        let available = WhatKeysSaid {
            the_passphrase_is_not_available: Vec::new(),
            ..said(Some(StoreStanding::NeverAnswered))
        };
        assert!(the_passphrase_mark(&row, Some(&available)).is_none());
        assert!(the_passphrase_mark(&row, None).is_none(), "a mark derived from nothing");
        // **The gone key takes the line**: `Servers` P1's first question.
        let gone = WhatKeysSaid {
            verdicts: vec![(
                key.identifier.as_text(),
                usable::Verdict::Unusable(usable::Unusable::FileMissing),
            )],
            ..said(Some(StoreStanding::NeverAnswered))
        };
        assert_eq!(
            the_screen_from(std::slice::from_ref(&row), &keys, Some(&gone), None, &Default::default()).rows[0]
                .the_key_mark,
            Some(Words::plain(words::THE_KEY_IS_GONE))
        );
    }

    /// **Step 10b's first audit round, item 7: `Security and licence` S34's
    /// sentence reaches the row it is owed on**, and only that row — and it is
    /// drawn on every draw for as long as the application stays open, because
    /// a sentence drawn once and redrawn away a moment later was never said.
    #[test]
    fn the_once_said_sentence_is_on_the_row_it_is_owed_on_and_no_other() {
        let owed = a_row();
        let other = a_row();
        let register = an_empty_register();
        let rows = vec![owed.clone(), other.clone()];
        let before = the_screen_from(&rows, &[], None, Some(&register), &Default::default());
        assert!(before.rows.iter().all(|row| row.the_sentence.is_none()));
        register.the_sentence_is_owed_on(&owed.local_server_identifier);
        for _ in 0..2 {
            let drawn = the_screen_from(&rows, &[], None, Some(&register), &Default::default());
            assert_eq!(
                drawn.rows[0].the_sentence,
                Some(Prose::new(crate::surface::keys::NOWHERE_TO_KEEP_THEM))
            );
            assert!(drawn.rows[1].the_sentence.is_none());
        }
    }

    /// **Step 10b's first audit round, item 9: a repairing row says when the
    /// next attempt falls**, from the run's held moment, with the underway
    /// shape's own words. A connected row, and a repairing one whose moment is
    /// absent, carry none.
    #[test]
    fn a_repairing_row_says_when_the_next_attempt_falls() {
        use smoozy_servers::acting::WhatServersPublishes;
        use smoozy_servers::condition::ObservedValue;
        use smoozy_servers::driving::APeriod;
        use smoozy_servers::period::{OpeningAct, WhoTookIt};
        use smoozy_servers::repair::NextAttemptDue;

        let row = a_row();
        let server = row.local_server_identifier.clone();
        let register = an_empty_register();
        let period = APeriod::begun(
            &server,
            OpeningAct::TheUsersConnectAction,
            WhoTookIt::TheUser,
            SystemTime::UNIX_EPOCH,
        );
        register.a_period_has_begun(&server, &period);
        let mut publishes = register.where_a_period_publishes();
        let drawn = || the_screen_from(std::slice::from_ref(&row), &[], None, Some(&register), &Default::default()).rows[0].clone();

        publishes.the_observed_value(&server, ObservedValue::Connected);
        assert!(drawn().the_next_attempt.is_none(), "a connected row");

        publishes.the_observed_value(&server, ObservedValue::Disconnected);
        publishes.the_next_attempt_due(&server, NextAttemptDue::Absent);
        assert!(drawn().the_next_attempt.is_none(), "an attempt in flight: the word alone");

        let due = SystemTime::UNIX_EPOCH + Duration::from_secs(1_800_000_012);
        publishes.the_next_attempt_due(&server, NextAttemptDue::At(due));
        let next = drawn().the_next_attempt.expect("the moment is drawn");
        assert_eq!(next.due_at.as_str(), "1800000012");
        assert_eq!(
            next.trying_again_in,
            Prose::new(crate::surface::the_underway_shape::TRYING_AGAIN_IN)
        );
        assert_eq!(
            drawn().the_work,
            vec![Words::plain(WhatTheColumnSays::Reconnecting.the_word())]
        );
    }

    /// **`Design/Whole app § Anything that isn't instant shows that it's
    /// underway`: *Connect* is the rule's own *on the control* example.** The
    /// row says the user's connect is under way from the press until the
    /// connection is live, holds *Connect* under the mark, and puts the dash
    /// in the work column rather than *Reconnecting…*. **Once it has
    /// connected, a drop is a repair**, and a repairing row takes no mark —
    /// *one thing never carries two of them*. Step 10c's eighth-pass fix
    /// piece 5: on the pass *Connect* showed nothing while it worked.
    #[test]
    fn the_users_connect_is_marked_until_it_connects_and_a_repair_never_is() {
        use smoozy_servers::acting::WhatServersPublishes;
        use smoozy_servers::condition::ObservedValue;
        use smoozy_servers::driving::APeriod;
        use smoozy_servers::period::{OpeningAct, WhoTookIt};

        // **Its own server**, since the register is the run's one.
        let mut row = a_row();
        row.local_server_identifier = "c0ec7c0ec7c0ec7c0ec7c0ec7c0ec7c0".to_string();
        let server = row.local_server_identifier.clone();
        let register = an_empty_register();
        let period = APeriod::begun(
            &server,
            OpeningAct::TheUsersConnectAction,
            WhoTookIt::TheUser,
            SystemTime::UNIX_EPOCH,
        );
        assert!(THE_USERS_CONNECTS.pressed(&server));
        register.a_period_has_begun(&server, &period);
        let mut publishes = register.where_a_period_publishes();
        let drawn = || the_screen_from(std::slice::from_ref(&row), &[], None, Some(&register), &Default::default()).rows[0].clone();

        publishes.the_observed_value(&server, ObservedValue::Reconnecting);
        let connecting = drawn();
        assert!(connecting.the_connect_is_underway);
        assert!(!connecting.the_connection_is_coming_back, "the mark, not the amber");
        assert_eq!(connecting.the_connection.label, Prose::new(words::CONNECT));
        assert_eq!(connecting.the_work[0].values, vec![Verbatim::new("—")]);

        publishes.the_observed_value(&server, ObservedValue::Connected);
        let connected = drawn();
        assert!(!connected.the_connect_is_underway);
        assert!(!connected.the_connection_is_coming_back);
        assert_eq!(connected.the_connection.label, Prose::new(words::CONNECTED));

        publishes.the_observed_value(&server, ObservedValue::Reconnecting);
        let repairing = drawn();
        assert!(!repairing.the_connect_is_underway, "a drop after connecting is a repair");
        // **Ninth-pass fix piece 2: a repairing server's control keeps its
        // amber** — the pass drew it green.
        assert!(repairing.the_connection_is_coming_back);
        assert_eq!(
            repairing.the_work,
            vec![Words::plain(WhatTheColumnSays::Reconnecting.the_word())]
        );
    }

    /// **A connect whose opening attempt failed ends its period at once**, so
    /// the mark goes with the period; and **a press that began nothing lets go
    /// only of what it put down**.
    #[test]
    fn a_connect_with_no_period_standing_is_not_underway() {
        let server = "c0ec7c0ec7c0ec7c0ec7c0ec7c0ec7c1";
        let none = what_is_true_of_a_server_with_no_period();
        assert!(THE_USERS_CONNECTS.pressed(server));
        assert!(!THE_USERS_CONNECTS.pressed(server), "a second press puts nothing new down");
        assert!(!THE_USERS_CONNECTS.is_underway(server, none));
        assert!(THE_USERS_CONNECTS.pressed(server), "the drawing let it go");
        THE_USERS_CONNECTS.did_not_begin(server);
        assert!(THE_USERS_CONNECTS.pressed(server), "nothing was left behind");
        THE_USERS_CONNECTS.did_not_begin(server);
    }

    /// **Step 10b's first audit round, item 8: the list takes `Servers` H1's
    /// state from what the last connection read.** A branch that disqualifies,
    /// with no fresh reading kept, returns the row to *never read*; the fourth
    /// branch leaves the held reading *last seen*; a fresh reading kept on the
    /// live connection is *live*, and *last seen* once that connection is not.
    #[test]
    fn the_hardware_follows_the_branch_the_last_connection_took() {
        use crate::servers::a_period::WhatTheLastConnectionRead;
        use smoozy_servers::condition::WhatIsTrueOfTheServer;
        let connected = WhatIsTrueOfTheServer {
            a_period_stands: true,
            the_primary_connection_is_live: true,
            a_run_of_attempts_is_in_flight: false,
            the_restart_mark_stands: false,
        };
        let held = a_reading();
        let state = |read: WhatTheLastConnectionRead, of| {
            the_hardware_as_the_run_knows_it(Some(&held), Some(read), of).state
        };
        let read = |check, kept, live| WhatTheLastConnectionRead {
            check,
            a_fresh_reading_was_kept: kept,
            on_the_connection_that_is_live: live,
        };
        assert_eq!(
            state(read(InstallationCheck::Absent, false, true), connected).as_str(),
            "never-read",
            "**`Servers` H18: the absent branch disqualifies the held reading**"
        );
        assert_eq!(
            state(read(InstallationCheck::PresentAndDifferent, false, true), connected).as_str(),
            "never-read"
        );
        assert_eq!(
            state(
                read(InstallationCheck::PresentAndTheSupervisorDidNotAnswer, false, true),
                connected
            )
            .as_str(),
            "last-seen",
            "**the fourth branch discards nothing** and shows the held reading as last seen"
        );
        assert_eq!(
            state(read(InstallationCheck::Absent, true, true), connected).as_str(),
            "live",
            "the fresh reading taken under the absent branch replaced the held one"
        );
        assert_eq!(
            state(read(InstallationCheck::PresentAndEqualConfirmed, true, false), connected)
                .as_str(),
            "last-seen",
            "**`Servers` H1a: keyed on the connection the reading was taken on**"
        );
        // And no connection in this run: `Servers` H1's own table.
        assert_eq!(
            the_hardware_as_the_run_knows_it(Some(&held), None, connected).state.as_str(),
            "last-seen"
        );
    }

    /// **Step 10b's first audit round, item 14: opening a period answers by its
    /// cause**, and *removed* is kept for a row that is really gone.
    #[test]
    fn a_connect_that_began_nothing_is_answered_by_its_cause() {
        assert_eq!(the_beginning_is_answered(HowItWent::APeriodStands), None);
        assert_eq!(
            the_beginning_is_answered(HowItWent::OneAlreadyStands),
            None,
            "a second *Connect* is not told the server was removed"
        );
        assert_eq!(
            the_beginning_is_answered(HowItWent::TheServerIsGone),
            Some(TheAnswer::TheServerIsGone)
        );
        assert_eq!(
            the_beginning_is_answered(HowItWent::TheRecordWouldNotRead),
            Some(TheAnswer::TheListCouldNotBeRead)
        );
        let said = the_answer_event(TheAnswer::TheListCouldNotBeRead);
        assert!(format!("{said:?}").contains("the-list-could-not-be-read"));
    }

    /// **Where the server is connected and Smoozy's service on it does not
    /// answer, the work column takes the dash rather than *Idle*** —
    /// `Design/Servers § A row`, *"since no reading of its work stands"*. The
    /// condition is read from the record that holds it (`State, identity and
    /// recovery` §8.3), and **withdrawn, the row reads *Idle* again**. The
    /// seventh pass on RunPod read *Connected · Idle* over a service that had
    /// not come back. Step 10c's eighth-pass fix piece 6.
    #[test]
    fn a_connected_server_whose_service_does_not_answer_is_not_idle() {
        use crate::record::failure_records::{ConditionKey, DurableSubject, FailureRecords};
        use crate::servers::a_period::{ThePeriodsThisRun, TheRunThroughTheApp};
        use smoozy_servers::acting::WhatServersPublishes;
        use smoozy_servers::condition::ObservedValue;
        use smoozy_servers::driving::APeriod;
        use smoozy_servers::period::{OpeningAct, WhoTookIt};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-v1-row-{}-{}.sqlite3", std::process::id(), line!()));
        let _ = std::fs::remove_file(&path);
        let record = crate::record::LocalRecord::open_at(&path).expect("a record");
        let row = a_row();
        let server = row.local_server_identifier.clone();
        let subject = DurableSubject::ServerRow(
            Identity::read_back(IdentityKind::LocalServer, &server).expect("an identity"),
        );
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
            std::sync::Arc::new(crate::events::TheOneQueue::new()),
            std::sync::Arc::new(crate::notifications::TheNotificationService::on(Box::new(
                NothingIsPosted,
            ))),
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

        let drawn = the_screen_from(std::slice::from_ref(&row), &[], None, Some(&register), &Default::default());
        assert_eq!(drawn.rows[0].the_connection.label, Prose::new(words::CONNECTED));
        assert_eq!(drawn.rows[0].the_work.len(), 1);
        assert_eq!(drawn.rows[0].the_work[0].values[0].as_str(), "—", "the row read Idle");

        // **The supervisor answers, the condition goes, and the row reads Idle.**
        {
            let held = started.lock().expect("the start");
            FailureRecords::over(held.record.as_ref().expect("the record").connection())
                .the_observed_value_moved(&key, "the supervisor answering")
                .expect("withdrawn");
        }
        // *(A reading holding nothing, since step 10c's eleventh-pass fix piece
        // 6: with no reading held a connected server reads the dash.)*
        let nothing_live: WhatTheServersAreDoing =
            [(server.clone(), WhatAServerIsDoing::NothingIsLive)].into_iter().collect();
        let drawn = the_screen_from(std::slice::from_ref(&row), &[], None, Some(&register), &nothing_live);
        assert_eq!(
            drawn.rows[0].the_work[0].lead,
            Prose::new(WhatTheColumnSays::Idle.the_word())
        );
        drop(register);
        drop(started);
        let _ = std::fs::remove_file(&path);
    }

    /// **A connected server draws as one, and the whole row moves with it.**
    ///
    /// **This is the join the step is named for, drawn.** A period stands, the
    /// run holds *connected* for that server, and the screen composed from the
    /// register gives the row the connected menu, the connect control that
    /// reads *Connected* and offers *disconnect*, the work column's *Idle*, and
    /// a footer that counts the server as connected. **Every one of those was
    /// unreachable before this piece** — nothing drove a period, so the answer
    /// was always *a server with no period*.
    #[test]
    fn a_server_whose_period_stands_draws_as_a_connected_one() {
        use crate::servers::a_period::{ThePeriodsThisRun, TheRunThroughTheApp};
        use smoozy_servers::acting::WhatServersPublishes;
        use smoozy_servers::condition::ObservedValue;
        use smoozy_servers::driving::APeriod;
        use smoozy_servers::period::{OpeningAct, WhoTookIt};

        let row = a_row();
        let server = row.local_server_identifier.clone();
        let register = ThePeriodsThisRun::over(
            TheRunThroughTheApp::lending_nothing(),
            std::sync::Arc::new(crate::events::TheOneQueue::new()),
            std::sync::Arc::new(crate::notifications::TheNotificationService::on(Box::new(
                NothingIsPosted,
            ))),
        );

        // **With no period, the row reads as every row in this application read
        // before this piece.**
        let before = the_screen_from(std::slice::from_ref(&row), &[], None, Some(&register), &Default::default());
        assert_eq!(before.rows[0].the_connection.act.as_str(), "connect");
        assert_eq!(before.rows[0].the_menu.as_str(), "disconnected");
        assert_eq!(before.footer.how_many_connected.values[0].as_str(), "0");

        // **`Servers` S5: a period begins at the act**, and `Servers` Q1 step 3
        // publishes *connected* the moment the transport reports the connection
        // authenticated and usable.
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

        // *(With a reading holding nothing, since step 10c's eleventh-pass fix
        // piece 6: with none held a connected server reads the dash.)*
        let nothing_live: WhatTheServersAreDoing =
            [(server.clone(), WhatAServerIsDoing::NothingIsLive)].into_iter().collect();
        let drawn = the_screen_from(std::slice::from_ref(&row), &[], None, Some(&register), &nothing_live);
        assert_eq!(drawn.rows[0].the_connection.label, Prose::new(words::CONNECTED));
        assert_eq!(drawn.rows[0].the_connection.act.as_str(), "disconnect");
        assert_eq!(drawn.rows[0].the_menu.as_str(), "connected");
        assert_eq!(
            drawn.rows[0].the_work[0].lead,
            Prose::new(WhatTheColumnSays::Idle.the_word())
        );
        assert_eq!(drawn.footer.how_many_connected.values[0].as_str(), "1");

        // **`Servers` S10 item 1: the user disconnects, and the row goes back
        // to what it was** — the period is what the drawing turns on, and
        // nothing durable moved for any of it.
        register.a_period_has_ended(&server);
        let after = the_screen_from(std::slice::from_ref(&row), &[], None, Some(&register), &Default::default());
        assert_eq!(after.rows[0].the_connection.act.as_str(), "connect");
        assert_eq!(after.footer.how_many_connected.values[0].as_str(), "0");
    }

    /// **Step 10c's ninth pass, fault 3: the server list read *Idle* while a
    /// job rendered**, at 20:29:20 and 20:31:35 and again at 20:38:29 while
    /// Queue said *1 rendering*. `Design/Servers § A row`: *"Rendering and the
    /// job's name with how many are waiting, or Idle"*. **And a job whose
    /// ending Smoozy has learned leaves the column as it leaves the queue**
    /// (`Queue & History` C11), while the column never says *Rendering* from
    /// the local record alone.
    #[test]
    fn a_connected_server_with_a_job_rendering_reads_rendering_and_its_name() {
        use crate::servers::a_period::{ThePeriodsThisRun, TheRunThroughTheApp};
        use crate::surface::the_queue::what_the_servers_are_doing;
        use smoozy_queue_history::line::QueueState;
        use smoozy_servers::acting::WhatServersPublishes;
        use smoozy_servers::condition::ObservedValue;
        use smoozy_servers::driving::APeriod;
        use smoozy_servers::period::{OpeningAct, WhoTookIt};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-work-column-{}-{}", std::process::id(), line!()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let row = a_row();
        let server = row.local_server_identifier.clone();
        let register = ThePeriodsThisRun::over(
            TheRunThroughTheApp::lending_nothing(),
            std::sync::Arc::new(crate::events::TheOneQueue::new()),
            std::sync::Arc::new(crate::notifications::TheNotificationService::on(Box::new(
                NothingIsPosted,
            ))),
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
        let the_server = Identity::read_back(IdentityKind::LocalServer, &server).expect("an identity");
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let long2 = crate::record::deletions::tests::a_record_in_history(&the_server, "p12h9 long2");
        store.write_the_provisional_record(&long2).unwrap();
        let next = crate::record::deletions::tests::a_record_in_history(&the_server, "p12h9 next");
        store.write_the_provisional_record(&next).unwrap();
        let an_operation = || Identity::issue(IdentityKind::Operation).unwrap();
        let holds = register.the_machines_work();
        crate::surface::the_queue::tests::a_reading_of(
            holds,
            &the_server,
            vec![
                (long2.job.clone(), QueueState::Rendering { operation: an_operation() }),
                (next.job.clone(), QueueState::Waiting),
            ],
        );
        let the_work = || {
            let work = what_the_servers_are_doing(holds, Some(&record));
            the_screen_from(std::slice::from_ref(&row), &[], None, Some(&register), &work).rows[0]
                .the_work
                .clone()
        };

        let rendering = the_work();
        assert_eq!(
            rendering[0].lead,
            Prose::new(WhatTheColumnSays::Rendering(Verbatim::new("")).the_word()),
            "a job renders and the row reads {rendering:?}"
        );
        assert_eq!(rendering[0].values[0].as_str(), "p12h9 long2", "with the job's name");
        assert_eq!(rendering[1].values[0].as_str(), "1", "and how many wait");
        assert_eq!(rendering[1].rest, Some(Prose::new(words::WAITING)));

        // **The server's own page reads the same words**, off the same row.
        let head = crate::surface::a_server::the_head_from(
            std::slice::from_ref(&row),
            &server,
            Some(&register),
            &what_the_servers_are_doing(holds, Some(&record)),
        )
        .expect("the head");
        assert_eq!(head.the_work, rendering);

        // **The ending is learned and the reading is not taken again.** The
        // job that waited has most likely started, but only a reading may say
        // so, and the reading says it waits: never *Idle* and never
        // *Rendering*. *(Until 2026-09-28 this read the dash; `Design/Servers
        // § A row` gives live jobs with none rendering *Waiting* and how many.)*
        crate::surface::the_queue::tests::its_ending_is_learned(&store, &long2.job, "p12h9 long2");
        let between = the_work();
        assert_eq!(between.len(), 1);
        assert_eq!(between[0].lead, Prose::new(words::WAITING_LEAD), "the column read {between:?}");
        assert_eq!(between[0].values[0].as_str(), "1", "the column read {between:?}");
        assert_eq!(between[0].rest, Some(Prose::new(words::A_JOB)));

        // **The next reading says the next job renders.**
        crate::surface::the_queue::tests::a_reading_of(
            holds,
            &the_server,
            vec![
                (long2.job.clone(), QueueState::Ended { outcome: an_operation() }),
                (next.job.clone(), QueueState::Rendering { operation: an_operation() }),
            ],
        );
        let after = the_work();
        assert_eq!(after.len(), 1, "nothing waits, so no count is drawn");
        assert_eq!(after[0].values[0].as_str(), "p12h9 next");

        // **Idle only where nothing is live.**
        crate::surface::the_queue::tests::its_ending_is_learned(&store, &next.job, "p12h9 next");
        assert_eq!(the_work()[0].lead, Prose::new(WhatTheColumnSays::Idle.the_word()));
        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **`Design/Whole app § Typography`: a figure and the words after it are
    /// bound for the page**, and a name and the words after it are not. The
    /// mark is worked out where every `Words` becomes what the page reads, so
    /// no composer has to remember it; *24 GB* itself arrives as one value
    /// joined by U+00A0.
    #[test]
    fn a_figure_and_the_words_after_it_are_bound_for_the_page() {
        let ram = serde_json::to_value(Words::after(in_gigabytes(24 * 1024 * 1024 * 1024), words::RAM))
            .expect("it serialises");
        assert_eq!(ram["values"][0], serde_json::json!("24\u{00A0}GB"));
        assert_eq!(ram["the_rest_binds"], serde_json::json!(true));
        let waiting = serde_json::to_value(Words::after("2", words::WAITING)).expect("it serialises");
        assert_eq!(waiting["the_rest_binds"], serde_json::json!(true));
        let a_name = Words {
            lead: Prose::new(""),
            values: vec![Verbatim::new("kitchen_v14")],
            rest: Some(Prose::new("is rendering")),
            the_rest_joins: false,
        };
        assert!(!a_name.the_rest_binds(), "a name is not a figure");
        assert!(!Words::asked("Remove", "2080 box").the_rest_binds(), "a rest that joins has no space");
        assert!(!Words::with(words::WAITING_LEAD, "2").the_rest_binds(), "no rest, nothing to bind");
        let Cards::TheCards(cards) = a_reading().cards else {
            panic!("the reading has its card")
        };
        assert_eq!(
            the_card_reads(&cards[0]),
            "NVIDIA GeForce RTX 4090, 24\u{00A0}GB",
            "the card's memory is one value, joined"
        );
    }

    /// **`Design/Servers § A row`, added 2026-09-28: a server whose reading
    /// holds jobs waiting and none rendering reads *Waiting* and how many**, in
    /// *Rendering*'s shape with the count where the name goes — *Waiting*,
    /// *2 jobs*. Held jobs, which wait on the user, count as the Queue counts
    /// them. **A reading that is not updating still takes the dash**
    /// (`Queue & History` C7). Until this date both read the dash.
    #[test]
    fn a_server_whose_jobs_all_wait_reads_waiting_and_how_many() {
        use crate::servers::a_period::{ThePeriodsThisRun, TheRunThroughTheApp};
        use crate::surface::the_queue::what_the_servers_are_doing;
        use smoozy_queue_history::line::QueueState;
        use smoozy_servers::acting::WhatServersPublishes;
        use smoozy_servers::condition::ObservedValue;
        use smoozy_servers::driving::APeriod;
        use smoozy_servers::period::{OpeningAct, WhoTookIt};

        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-work-column-{}-{}", std::process::id(), line!()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3")).unwrap();
        let row = a_row();
        let server = row.local_server_identifier.clone();
        let register = ThePeriodsThisRun::over(
            TheRunThroughTheApp::lending_nothing(),
            std::sync::Arc::new(crate::events::TheOneQueue::new()),
            std::sync::Arc::new(crate::notifications::TheNotificationService::on(Box::new(
                NothingIsPosted,
            ))),
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
        let the_server = Identity::read_back(IdentityKind::LocalServer, &server).expect("an identity");
        let store = crate::record::job_records::JobRecords::over(record.connection());
        let first = crate::record::deletions::tests::a_record_in_history(&the_server, "held one");
        store.write_the_provisional_record(&first).unwrap();
        let second = crate::record::deletions::tests::a_record_in_history(&the_server, "held two");
        store.write_the_provisional_record(&second).unwrap();
        let an_operation = || Identity::issue(IdentityKind::Operation).unwrap();
        let holds = register.the_machines_work();
        crate::surface::the_queue::tests::a_reading_of(
            holds,
            &the_server,
            vec![
                (first.job.clone(), QueueState::Held { hold: an_operation() }),
                (second.job.clone(), QueueState::Held { hold: an_operation() }),
            ],
        );
        let the_work = || {
            let work = what_the_servers_are_doing(holds, Some(&record));
            the_screen_from(std::slice::from_ref(&row), &[], None, Some(&register), &work).rows[0]
                .the_work
                .clone()
        };

        let waiting = the_work();
        assert_eq!(waiting.len(), 1, "one line, in Rendering's shape: {waiting:?}");
        assert_eq!(waiting[0].lead, Prose::new(words::WAITING_LEAD), "the column read {waiting:?}");
        assert_eq!(waiting[0].values[0].as_str(), "2", "with how many where the name goes");
        assert_eq!(waiting[0].rest, Some(Prose::new(words::JOBS)));
        let drawn = serde_json::to_value(&waiting[0]).expect("it serialises");
        assert_eq!(drawn["the_rest_binds"], serde_json::json!(true), "2 and jobs stay on one line");

        // **The server's own page reads the same words**, off the same row.
        let head = crate::surface::a_server::the_head_from(
            std::slice::from_ref(&row),
            &server,
            Some(&register),
            &what_the_servers_are_doing(holds, Some(&record)),
        )
        .expect("the head");
        assert_eq!(head.the_work, waiting);

        // **Not updating: the dash, whatever the reading held.**
        holds.the_connection_ended(&server);
        let stale = the_work();
        assert_eq!(stale.len(), 1);
        assert_eq!(stale[0].values[0].as_str(), "—", "the column read {stale:?}");
        drop(record);
        let _ = std::fs::remove_dir_all(&path);
    }

    /// **An *Add* with a setup option ticked marks the connection it opens, as
    /// *Connect* does** — the ninth pass found it silent. The add holds the
    /// server in [`THE_USERS_CONNECTS`] before the period is begun, and lets go
    /// where nothing began, exactly as the connect act does.
    #[test]
    fn an_add_that_connects_marks_the_connection_it_opens() {
        let working = include_str!("servers.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the working half");
        let at = working.find("pub fn a_server_is_added").expect("the add");
        let body = &working[at..at + working[at..].find("\n}\n").expect("it closes")];
        let held = body.find("THE_USERS_CONNECTS.pressed(server)").expect("the add holds the mark");
        let opened = body
            .find("a_period::a_period_is_opened(&app, server, WhoAsked::AnAddWithASetupOptionTicked)")
            .expect("the add opens the period");
        assert!(held < opened, "the mark is held before the period is begun");
        assert!(body.contains("THE_USERS_CONNECTS.did_not_begin(server)"));
        // **The Creator's notes piece 3: the mark and the period go down before
        // the answer is told**, so a list drawn the moment the dialog closes
        // finds the connect under way and shows the turning *Connect*.
        let answered = body
            .find("the_queue_is_told(&queue, the_add_is_answered(answer, ticked.as_deref()))")
            .expect("the add answers");
        assert!(held < answered && opened < answered, "the answer goes out before the mark");
        assert_eq!(body.matches("the_queue_is_told(").count(), 1, "the add answers once");
    }

    /// **The add's answer names the server where a setup was ticked, and
    /// nothing where none was** — `Design/Servers § Adding a server`, the
    /// Creator's note 3 on 0.1.7.
    #[test]
    fn the_adds_answer_names_the_server_only_where_a_setup_was_ticked() {
        let printed = |event: Event| format!("{event:?}");
        let ticked = printed(the_add_is_answered(TheAnswer::ItWasSaved, Some("server-7")));
        assert!(ticked.contains("\"itIsSetUp\"") && ticked.contains("server-7"), "{ticked}");
        let plain = printed(the_add_is_answered(TheAnswer::ItWasSaved, None));
        assert!(!plain.contains("itIsSetUp") && !plain.contains("\"server\""), "{plain}");
    }

    /// **`Design/Servers § A row`: while a setup runs, the column says *Setting
    /// up*, in the place of *Idle* and the dash and never of *Rendering*,
    /// *Waiting* or *Restarting*** — the Creator's note 3 on 0.1.7. **And
    /// whether one runs is the driving's own exclusion**, so it moves the told
    /// work column on its own.
    #[test]
    fn a_row_with_a_setup_running_reads_setting_up_in_place_of_idle_and_the_dash() {
        let setting_up = vec![Words::plain(words::SETTING_UP)];
        let idle = vec![Words::plain(WhatTheColumnSays::Idle.the_word())];
        assert_eq!(the_setup_takes_the_column(vec![the_dash()], true), setting_up);
        assert_eq!(the_setup_takes_the_column(idle.clone(), true), setting_up);
        assert_eq!(the_setup_takes_the_column(idle.clone(), false), idle, "no setup, no change");
        for kept in [
            vec![Words::plain(words::RESTARTING)],
            vec![Words::with(WhatTheColumnSays::Rendering(Verbatim::new("Kitchen")).the_word(), "Kitchen".to_string())],
            vec![Words::after("2".to_string(), words::WAITING)],
        ] {
            assert_eq!(the_setup_takes_the_column(kept.clone(), true), kept);
        }

        let server = "server-setting-up-on-the-row";
        assert!(!a_setup_runs_on(server));
        let before = what_is_told(WhatTheServersAreDoing::default(), &[a_row_named(server)]);
        assert!(crate::setting_up::a_run_begins(server, smoozy_server_setup::run::SetupKind::TheReadyAction));
        assert!(a_setup_runs_on(server));
        let during = what_is_told(WhatTheServersAreDoing::default(), &[a_row_named(server)]);
        assert_ne!(before, during, "a setup beginning moves what the column is told");
        crate::setting_up::a_run_ends(server);
        assert!(!a_setup_runs_on(server));
    }

    /// **The on-screen condition's two new surfaces are told by their pages**,
    /// and each page is asked for again when the work moves.
    #[test]
    fn the_list_and_a_servers_page_say_when_their_work_column_is_on_screen() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for (page, which) in [("web/servers.js", "the-list"), ("web/a-server.js", "a-server")] {
            let text = std::fs::read_to_string(root.join(page)).expect("the page is readable");
            assert!(text.contains(&format!("\"{THE_WORK_MOVED}\"")), "{page} names the kind");
            assert!(text.contains("whenTold(THE_WORK_MOVED"), "{page} hears it");
            assert!(text.contains("\"the_work_column_is_on_screen\""), "{page} tells the condition");
            assert!(text.contains(&format!("which: \"{which}\"")), "{page} names itself");
            // **The page's own methods, at their indent** — a contract comment
            // higher in the file names them too.
            let away = text.find("\n  itIsPutAway() {").expect("the page is put away");
            let body = &text[away..away + text[away..].find("\n  },").expect("it closes")];
            assert!(body.contains("theWorkColumnIsOnScreen("), "{page} says when it is put away");
            let now = text.find("\n  itIsCurrentNow(").expect("the page is made current");
            let body = &text[now..now + text[now..].find("\n  },").expect("it closes")];
            assert!(body.contains("theWorkColumnIsOnScreen("), "{page} says when it is shown");
        }
    }

    /// **The work column is told again only where it moved**, so a P10d pass
    /// that changed nothing asks no screen for anything.
    #[test]
    fn the_work_column_is_told_again_only_on_a_change() {
        let last = Mutex::new(None);
        let mut doing = WhatTheServersAreDoing::new();
        doing.insert("a".to_string(), WhatAServerIsDoing::NothingIsLive);
        assert!(the_work_moved(&last, doing.clone()), "the first telling");
        assert!(!the_work_moved(&last, doing.clone()), "the same again");
        doing.insert(
            "a".to_string(),
            WhatAServerIsDoing::Rendering { name: "p12h9 long".to_string(), waiting: 0 },
        );
        assert!(the_work_moved(&last, doing), "a job started");
    }

    /// **Step 10c's tenth pass, fault 4: the work column read a dash for nine
    /// minutes after the first setup's *done***, connected, with nothing
    /// running, and read *Idle* before it. The setup's restart is a break
    /// (`Servers` K3 and K5), and a disconnect and connect is a period's end
    /// and a new period (`Servers` S11); both move the column to the dash
    /// without a reading (`Queue & History` C6 and C7), and the page, drawn
    /// again on the connection's own event, draws it. **The next reading says
    /// *Idle*, which is what the column was last told, so unless the break and
    /// the period's end were told too, nothing is posted and the dash stands.**
    ///
    /// Played as the user meets it: the page is drawn again on every
    /// connection event and wherever the column is told, and at no other
    /// moment. **The first round asks the detector only where a reading lands**,
    /// as the column was built for, and the dash stands. **The second asks it
    /// wherever this code asks it** — read off the two places the column moves
    /// without a reading, because both need a running shell to be called: the
    /// break's declaration, beside the queue being drawn again, and a period's
    /// end, before any ending there is said — and the page must read *Idle*
    /// within the reading after each.
    #[test]
    fn the_work_column_is_told_again_after_a_break_and_after_a_periods_end() {
        use crate::servers::the_machines_work::WhatThisRunHolds;
        use crate::surface::the_queue::tests::a_reading_of;
        use crate::surface::the_queue::what_the_servers_are_doing;

        let period = include_str!("../servers/a_period.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the working half");
        let at = period.find("fn the_machines_work_stops<").expect("the break's declaration");
        let the_break = &period[at..at + period[at..].find("\n}\n").expect("it closes")];
        let asked_at_the_break =
            the_break.contains("crate::surface::servers::the_work_column_is_told_again(&app);");
        let working = the_working_half();
        let at = working.find("pub(crate) fn the_period_ended<").expect("the period's end");
        let ended = &working[at..at + working[at..].find("\n}\n").expect("it closes")];
        let asked_at_a_periods_end = ended
            .find("the_work_column_is_told_again(app);")
            .is_some_and(|telling| ended.find("return").is_none_or(|first| telling < first));

        let server = Identity::issue(IdentityKind::LocalServer).expect("an identity");
        // **A connected server whose step 7 owes a reading**, as the pass's
        // server was once the repair or the new connect had gone live.
        let connected = WhatIsTrueOfTheServer {
            a_period_stands: true,
            the_primary_connection_is_live: true,
            a_run_of_attempts_is_in_flight: false,
            the_restart_mark_stands: false,
        };
        let idle = vec![Words::plain(WhatTheColumnSays::Idle.the_word())];
        let dash = vec![the_dash()];
        let drawn = |holds: &WhatThisRunHolds| {
            // `false`: the check found Smoozy's own installation, so no
            // reading held is the dash (step 10c's eleventh-pass fix piece 6
            // turned this argument from *a reading is owed* to its
            // `Queue & History` A1 case).
            the_work_column(connected, what_the_servers_are_doing(holds, None).get(&server.as_text()), false)
        };
        for (round, at_the_break, at_a_periods_end) in [
            ("as the column was built", false, false),
            ("as this code asks", asked_at_the_break, asked_at_a_periods_end),
        ] {
            let the_dash_stands = round == "as the column was built";
            let last = Mutex::new(None);
            let holds = WhatThisRunHolds::default();
            let told = |holds: &WhatThisRunHolds| the_work_moved(&last, what_the_servers_are_doing(holds, None));
            let mut on_screen = Vec::new();
            let the_column_is_told = |holds: &WhatThisRunHolds, on_screen: &mut Vec<Words>| {
                if told(holds) {
                    *on_screen = drawn(holds);
                }
            };

            // **Idle, told**: step 7's reading on the first connection.
            a_reading_of(&holds, &server, Vec::new());
            the_column_is_told(&holds, &mut on_screen);
            assert_eq!(on_screen, idle);

            // **The break**: the setup's restart. The repaired connection goes
            // live, and the page is drawn again on that event before its step 7
            // reading lands.
            holds.the_connection_ended(&server.as_text());
            if at_the_break {
                the_column_is_told(&holds, &mut on_screen);
            }
            on_screen = drawn(&holds);
            assert_eq!(on_screen, dash, "the column reads the dash between the break and the reading");
            // **The reading says nothing is live**, as it said before the break.
            a_reading_of(&holds, &server, Vec::new());
            the_column_is_told(&holds, &mut on_screen);
            assert_eq!(
                on_screen == dash,
                the_dash_stands,
                "{round}: after the break's reading the page reads {on_screen:?}"
            );

            // **The period's end**: a disconnect and a connect. The new
            // period's connection is drawn on its own event before its reading.
            on_screen = idle.clone();
            holds.the_period_ended(&server.as_text());
            if at_a_periods_end {
                the_column_is_told(&holds, &mut on_screen);
            }
            on_screen = drawn(&holds);
            assert_eq!(on_screen, dash, "the column reads the dash before the new period's reading");
            a_reading_of(&holds, &server, Vec::new());
            the_column_is_told(&holds, &mut on_screen);
            assert_eq!(
                on_screen == dash,
                the_dash_stands,
                "{round}: after the new period's reading the page reads {on_screen:?}"
            );
        }
    }

    /// **A repairing server's control keeps its amber** — `Design/Whole app §
    /// Anything that isn't instant`: *"its Connect control keeps its amber"*.
    /// The ninth pass drew it green.
    #[test]
    fn a_repairing_servers_control_keeps_its_amber() {
        let page = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/servers.js"),
        )
        .expect("the page module is readable");
        let at = page.find("function theConnectControl(").expect("the control is drawn");
        let drawn = &page[at..at + page[at..].find("\n}\n").expect("it closes")];
        assert!(drawn.contains("row.the_connection_is_coming_back"));
        assert!(drawn.contains("\"trying\""), "the drawing's amber is `trying`");
        let coming_back = drawn.find("the_connection_is_coming_back").unwrap();
        let green = drawn.find("\"live\"").expect("the green is drawn somewhere");
        assert!(coming_back < green, "the amber is decided before the green");
    }

    // ── Transfer wave piece 9: V13's row lines and V15's override ───────────

    /// **A row, and a register in which its server is connected** — the shape
    /// `a_server_whose_period_stands_draws_as_a_connected_one` uses.
    fn a_connected_server() -> (ServerRow, ThePeriodsThisRun) {
        use crate::servers::a_period::TheRunThroughTheApp;
        use smoozy_servers::acting::WhatServersPublishes;
        use smoozy_servers::condition::ObservedValue;
        use smoozy_servers::driving::APeriod;
        use smoozy_servers::period::{OpeningAct, WhoTookIt};
        let row = a_row();
        let register = ThePeriodsThisRun::over(
            TheRunThroughTheApp::lending_nothing(),
            std::sync::Arc::new(crate::events::TheOneQueue::new()),
            std::sync::Arc::new(crate::notifications::TheNotificationService::on(Box::new(
                NothingIsPosted,
            ))),
        );
        let period = APeriod::begun(
            &row.local_server_identifier,
            OpeningAct::TheUsersConnectAction,
            WhoTookIt::TheUser,
            std::time::SystemTime::UNIX_EPOCH,
        );
        register.a_period_has_begun(&row.local_server_identifier, &period);
        register
            .where_a_period_publishes()
            .the_observed_value(&row.local_server_identifier, ObservedValue::Connected);
        (row, register)
    }

    /// **`Design/Servers § A row`: the work column takes the dash while the
    /// update runs, and where Smoozy on the server is too old or too new** —
    /// the service is being replaced, or speaks no protocol this build speaks,
    /// so no reading of its work stands. **Where the update only waits, or has
    /// failed, the column reads as ever.**
    #[test]
    fn the_work_column_takes_the_dash_while_updating_too_old_or_too_new() {
        use crate::servers::published::{the_server_carries, WhatSmoozyOnTheServerIs as Carries};
        let (row, register) = a_connected_server();
        let server = row.local_server_identifier.clone();
        // *(A reading holding nothing, since step 10c's eleventh-pass fix piece
        // 6: with none held a connected server reads the dash.)*
        let nothing_live: WhatTheServersAreDoing =
            [(server.clone(), WhatAServerIsDoing::NothingIsLive)].into_iter().collect();
        let the_work = |carries: Option<Carries>| {
            the_server_carries(&server, carries);
            let drawn = the_screen_from(std::slice::from_ref(&row), &[], None, Some(&register), &nothing_live);
            drawn.rows[0].the_work.clone()
        };
        let dash = vec![the_dash()];
        let idle = vec![Words::plain(WhatTheColumnSays::Idle.the_word())];
        assert_eq!(the_work(Some(Carries::TheUpdateIsUnderWay)), dash, "updating");
        assert_eq!(
            the_work(Some(Carries::TooOldAndItsWorkCannotBeControlledFromHere)),
            dash,
            "too old"
        );
        assert_eq!(the_work(Some(Carries::TooNew)), dash, "too new");
        assert_eq!(the_work(Some(Carries::TheUpdateWaits)), idle, "the update waits");
        assert_eq!(
            the_work(Some(Carries::TheUpdateFailed { said: "rolled back".into() })),
            idle,
            "the update failed"
        );
        assert_eq!(the_work(None), idle, "none of the five");
        the_server_carries(&server, None);
    }

    /// **`Design/Servers § A row`: each of V13's conditions puts its own line
    /// on a connected row, in the drawing's words, and none says *isn't
    /// responding*** (`Server protocol and transfers` P3b). The row is read as
    /// the page receives it, so the test holds whatever shape the row takes.
    #[test]
    fn each_condition_puts_its_design_line_on_a_connected_row_and_none_says_isnt_responding() {
        use crate::servers::published::{the_server_carries, WhatSmoozyOnTheServerIs as Carries};
        let (row, register) = a_connected_server();
        let server = row.local_server_identifier.clone();
        let mut doing = WhatTheServersAreDoing::new();
        doing.insert(
            server.clone(),
            WhatAServerIsDoing::Rendering { name: "Kitchen — final".to_string(), waiting: 2 },
        );
        let the_row_reads = |carries: Carries| {
            the_server_carries(&server, Some(carries));
            let drawn = the_screen_from(std::slice::from_ref(&row), &[], None, Some(&register), &doing);
            serde_json::to_string(&drawn.rows[0]).expect("the row serialises")
        };
        let has = |json: &str, words: &str| json.contains(Prose::new(words).as_str());
        let cases: [(Carries, &[&str], &[&str]); 5] = [
            (
                Carries::TheUpdateWaits,
                &[
                    "Smoozy on this server updates when its jobs finish.",
                    "Update now…",
                    "Update Smoozy on",
                    "now?",
                    "Updating now stops its jobs first.",
                    "job is rendering and",
                    "are waiting",
                    "waiting jobs, then stops the render. The current frame may not be saved; rendered frames stay on the server.",
                    "Cancel",
                    "Stop jobs and update",
                ],
                &[],
            ),
            (Carries::TheUpdateIsUnderWay, &["Updating Smoozy on this server…"], &["Update now…"]),
            (
                Carries::TheUpdateFailed { said: "the new version did not start".into() },
                &["Smoozy on this server could not be updated.", "What the server said", "Dismiss"],
                &["Update now…"],
            ),
            (
                Carries::TooOldAndItsWorkCannotBeControlledFromHere,
                &["Out of date. Not updated: could not read its job list."],
                &["Update now…", "after its jobs finish"],
            ),
            (
                Carries::TooNew,
                &["This server has a newer version of Smoozy. Update Smoozy on this computer to use it."],
                &["Update now…"],
            ),
        ];
        for (carries, present, absent) in cases {
            let named = format!("{carries:?}");
            let json = the_row_reads(carries);
            for words in present {
                assert!(has(&json, words), "{named}: the row does not say {words:?}: {json}");
            }
            for words in absent {
                assert!(!json.contains(words), "{named}: the row says {words:?}");
            }
            assert!(!json.contains("isn't responding"), "{named}: the row says Smoozy isn't responding");
            assert!(json.contains("the new version did not start") == named.starts_with("TheUpdateFailed"));
        }
        the_server_carries(&server, None);
    }

    /// **The too-old line says jobs stand only where that is known.** The
    /// build runner's ruling of 2026-09-28: where the records reading cannot
    /// answer, *its jobs can't be stopped from here* would claim what Smoozy
    /// does not know (Rule 8), so the sixth line says Smoozy can't tell. **In
    /// this build nothing reads that program's records**, so the condition
    /// always takes the sixth.
    #[test]
    fn the_too_old_line_says_jobs_stand_only_where_that_is_known() {
        let (which, said) = the_too_old_line(true);
        assert_eq!(which, "too-old");
        assert_eq!(said.lead, Prose::new("Smoozy on this server is out of date. Reconnect after its jobs finish to update it."));
        let (which, said) = the_too_old_line(false);
        assert_eq!(which, "too-old-unknown");
        assert!(!said.lead.as_str().contains("its jobs"), "it claims the jobs");
        let connected = WhatIsTrueOfTheServer {
            a_period_stands: true,
            the_primary_connection_is_live: true,
            a_run_of_attempts_is_in_flight: false,
            the_restart_mark_stands: false,
        };
        let line = the_update_line(
            Some(&crate::servers::published::WhatSmoozyOnTheServerIs::TooOldAndItsWorkCannotBeControlledFromHere),
            connected,
            "box",
            None,
        )
        .expect("a line");
        assert_eq!(line.which.as_str(), "too-old-unknown");
    }

    /// **V15: *Update now…* is offered only where the update waits and the
    /// connection is live** — never on *too old*, where the stop cannot reach
    /// the work, and never on a server not connected. **And no line stands on
    /// a server not connected**: each is a reading a connection took.
    #[test]
    fn update_now_is_not_offered_on_too_old_or_while_disconnected() {
        use crate::servers::published::{the_server_carries, WhatSmoozyOnTheServerIs as Carries};
        let connected = WhatIsTrueOfTheServer {
            a_period_stands: true,
            the_primary_connection_is_live: true,
            a_run_of_attempts_is_in_flight: false,
            the_restart_mark_stands: false,
        };
        let disconnected = what_is_true_of_a_server_with_no_period();
        let waits = Carries::TheUpdateWaits;
        assert!(the_update_now_is_offered(Some(&waits), connected));
        assert!(!the_update_now_is_offered(Some(&waits), disconnected));
        for other in [
            Carries::TooOldAndItsWorkCannotBeControlledFromHere,
            Carries::TooNew,
            Carries::TheUpdateIsUnderWay,
            Carries::TheUpdateFailed { said: "x".into() },
        ] {
            assert!(!the_update_now_is_offered(Some(&other), connected), "{other:?}");
            let line = the_update_line(Some(&other), connected, "box", None).expect("a line");
            assert!(line.update_now.is_none() && line.the_box.is_none(), "{other:?}");
        }
        assert!(the_update_line(Some(&waits), disconnected, "box", None).is_none());

        // **And the act itself asks nothing of the machine where it is not
        // offered.**
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness").as_text();
        for (carries, of) in [
            (Some(Carries::TooOldAndItsWorkCannotBeControlledFromHere), connected),
            (Some(Carries::TheUpdateWaits), disconnected),
            (None, connected),
        ] {
            the_server_carries(&server, carries);
            let mut machine = AMachineThatStopsAndClears::answering(Ok(TheStopAndClearEnded::ClearedAndStopped));
            assert_eq!(the_update_now(&server, of, &mut machine), WhatTheUpdateNowDid::NotOffered);
            assert!(machine.asked.is_empty(), "a stop and clear was sent where none is offered");
        }
        the_server_carries(&server, None);
    }

    /// **A machine that records every stop and clear asked of it**, and
    /// answers each with `answer`.
    struct AMachineThatStopsAndClears {
        answer: Result<TheStopAndClearEnded, TheAnswer>,
        asked: Vec<String>,
        log: std::rc::Rc<std::cell::RefCell<Vec<&'static str>>>,
    }

    impl AMachineThatStopsAndClears {
        fn answering(answer: Result<TheStopAndClearEnded, TheAnswer>) -> Self {
            AMachineThatStopsAndClears { answer, asked: Vec::new(), log: Default::default() }
        }
    }

    impl TheOverridesEnds for AMachineThatStopsAndClears {
        fn the_stop_and_clear(&mut self, server: &str) -> Result<TheStopAndClearEnded, TheAnswer> {
            self.asked.push(server.to_string());
            self.log.borrow_mut().push("the stop and clear");
            self.answer.clone()
        }
    }

    /// **The same machine's update ends, for piece 8's step 9**, writing to
    /// the same log so the order of the two is read off one list.
    struct ItsUpdate {
        log: std::rc::Rc<std::cell::RefCell<Vec<&'static str>>>,
    }

    impl crate::servers::the_machines_work::TheUpdatesOwnEnds for ItsUpdate {
        fn what_the_package_area_holds(&mut self) -> crate::servers::installation::WhatThePackageAreaHolds {
            crate::servers::installation::WhatThePackageAreaHolds {
                it_was_read: true,
                its_release_sequence: Some(6),
                ..Default::default()
            }
        }
        fn this_applications_own_release(&mut self) -> Result<u64, String> {
            Ok(7)
        }
        fn the_update_is_sent(
            &mut self,
            _under: Option<Identity>,
            _held: &crate::servers::installation::WhatThePackageAreaHolds,
            _work: Vec<(String, smoozy_runtime::package::retry::AcceptedWorkStanding)>,
        ) -> crate::servers::the_machines_work::TheUpdatesEnding {
            self.log.borrow_mut().push("the update");
            crate::servers::the_machines_work::TheUpdatesEnding::ItLanded
        }
    }

    /// **The machine's work, as a queue reading shows it.**
    fn a_reading_of(jobs: &[smoozy_queue_history::line::QueueState]) -> smoozy_queue_history::acting::adoption::TheMachinesWorkingState {
        smoozy_queue_history::acting::adoption::TheMachinesWorkingState {
            jobs: jobs
                .iter()
                .map(|state| smoozy_queue_history::readings::AJobAsTheMachineReadsIt {
                    job: Identity::issue(IdentityKind::Job).expect("randomness"),
                    state: state.clone(),
                    live: smoozy_queue_history::readings::WhatTheEntryHasLive::NoReadings,
                    the_hold: None,
                })
                .collect(),
            the_admission_point_stands_at: None,
            notes: Vec::new(),
            the_history_copys_count: 0,
            the_history_copys_entries: Vec::new(),
        }
    }

    /// **V15: *Stop jobs and update* sends `Queue & History` L13's stop and
    /// clear first, and the update follows it — by piece 8's step 9,
    /// unchanged, at the reading that shows the work gone (V12).** The waiting
    /// line stays what the server carries until then, which is what V12's
    /// retry keys on.
    #[test]
    fn update_now_sends_the_stop_and_clear_and_then_the_update_follows() {
        use crate::servers::published::{the_server_carries, what_the_server_carries, WhatSmoozyOnTheServerIs as Carries};
        use smoozy_queue_history::line::QueueState;
        let (row, register) = a_connected_server();
        let server = row.local_server_identifier.clone();
        let of = register.what_is_true_of(&server);
        the_server_carries(&server, Some(Carries::TheUpdateWaits));

        let mut machine = AMachineThatStopsAndClears::answering(Ok(TheStopAndClearEnded::ClearedAndStopped));
        let did = the_update_now(&server, of, &mut machine);
        assert_eq!(did, WhatTheUpdateNowDid::ClearedAndStopped);
        assert_eq!(machine.asked, vec![server.clone()], "one stop and clear, for this server");
        assert_eq!(did.the_answer(), Some(TheAnswer::TheUpdateNowWasTaken));
        assert_eq!(
            what_the_server_carries(&server),
            Some(Carries::TheUpdateWaits),
            "the waiting line is what V12's retry keys on"
        );

        // **The reading that shows the work gone**: step 9, unchanged, sends
        // the update — after the stop and clear, on the same log.
        let mut update = ItsUpdate { log: machine.log.clone() };
        crate::servers::the_machines_work::the_step_nine(
            &server,
            Some(&a_reading_of(&[])),
            &|_: &Identity| None,
            &mut update,
        );
        assert_eq!(*machine.log.borrow(), vec!["the stop and clear", "the update"]);
        assert_eq!(what_the_server_carries(&server), None, "the update landed");

        // **A reading still showing the render sends nothing** — the order
        // is the promise, and nothing is updated from under a job.
        the_server_carries(&server, Some(Carries::TheUpdateWaits));
        let log = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let mut update = ItsUpdate { log: log.clone() };
        crate::servers::the_machines_work::the_step_nine(
            &server,
            Some(&a_reading_of(&[QueueState::Rendering {
                operation: Identity::issue(IdentityKind::Operation).expect("randomness"),
            }])),
            &|_: &Identity| None,
            &mut update,
        );
        assert!(log.borrow().is_empty());
        the_server_carries(&server, None);
    }

    /// **V15: a result other than *cleared and stopped* sends no update and
    /// keeps the waiting line, and says the result as `Servers` W21 says it for
    /// a removal** — the removal box's own two sentences. *(Until step 10c's
    /// S2 this also held that the removal's own path answered that it was not
    /// built.)*
    #[test]
    fn a_result_other_than_cleared_and_stopped_sends_no_update_and_keeps_the_waiting_line() {
        use crate::servers::published::{the_server_carries, what_the_server_carries, WhatSmoozyOnTheServerIs as Carries};
        let (row, register) = a_connected_server();
        let server = row.local_server_identifier.clone();
        let of = register.what_is_true_of(&server);
        for (ending, lead, rest) in [
            (TheRemovalsEnding::CouldNotBegin, words::COULD_NOT_BEGIN, words::COULD_NOT_BEGIN_REST),
            (TheRemovalsEnding::TheClearWasRefused, words::CLEAR_REFUSED, words::CLEAR_REFUSED_REST),
            (TheRemovalsEnding::ItHalfWorked, words::HALF_WORKED, words::HALF_WORKED_REST),
        ] {
            the_server_carries(&server, Some(Carries::TheUpdateWaits));
            let mut machine = AMachineThatStopsAndClears::answering(Ok(TheStopAndClearEnded::Otherwise(ending)));
            let did = the_update_now(&server, of, &mut machine);
            assert_eq!(did, WhatTheUpdateNowDid::ItDidNotFullyWork(ending));
            assert_eq!(what_the_server_carries(&server), Some(Carries::TheUpdateWaits), "{ending:?}");
            let said = the_pairs_of(&the_answer_event(did.the_answer().expect("an answer")));
            assert!(said.contains(&("said".to_string(), Prose::new(lead).as_str().to_string())), "{ending:?}");
            assert!(said.contains(&("saidNext".to_string(), Prose::new(rest).as_str().to_string())), "{ending:?}");
            assert_eq!(*machine.log.borrow(), vec!["the stop and clear"], "{ending:?}: something else was sent");
        }

        the_server_carries(&server, None);
    }

    /// **The acts V13's lines carry are routed**: *Stop jobs and update* to
    /// V15, and the failed line's cross to withdrawing *the update failed*.
    #[test]
    fn the_update_acts_are_routed() {
        assert_eq!(where_an_act_goes(THE_UPDATE_NOW), WhereAnActGoes::TheUpdateNow);
        assert_eq!(
            where_an_act_goes(THE_UPDATE_FAILURE_IS_CLEARED),
            WhereAnActGoes::TheUpdateFailureIsCleared
        );
    }

    /// **A V13 move alone tells the page**, since it moves the row's line with
    /// the work where it was — the update taken in hand, landed or failed, or a
    /// program read too old or too new.
    #[test]
    fn a_v13_move_alone_tells_the_page() {
        use crate::servers::published::{the_server_carries, WhatSmoozyOnTheServerIs as Carries};
        let row = a_row();
        let last = Mutex::new(None);
        let doing = WhatTheServersAreDoing::new();
        assert!(the_work_moved(&last, what_is_told(doing.clone(), std::slice::from_ref(&row))));
        assert!(!the_work_moved(&last, what_is_told(doing.clone(), std::slice::from_ref(&row))));
        the_server_carries(&row.local_server_identifier, Some(Carries::TheUpdateIsUnderWay));
        assert!(
            the_work_moved(&last, what_is_told(doing.clone(), std::slice::from_ref(&row))),
            "the update went under way and the page was not told"
        );
        the_server_carries(&row.local_server_identifier, None);
        assert!(the_work_moved(&last, what_is_told(doing, std::slice::from_ref(&row))));
    }

    /// **Whether a job in the queue needs your answer moving alone tells the
    /// page** — the rail reads itself again on it, so its Queue dot goes when a
    /// held job is answered while another renders and every work column stays
    /// where it was (the twenty-fourth notes, note 2). Compared as
    /// [`THE_WORK_LAST_TOLD`] compares it.
    #[test]
    fn a_job_needing_an_answer_alone_tells_the_page() {
        let row = a_row();
        let last: Mutex<Option<(WhatIsTold, bool)>> = Mutex::new(None);
        let told = |asking| (what_is_told(WhatTheServersAreDoing::new(), std::slice::from_ref(&row)), asking);
        assert!(the_work_moved(&last, told(true)));
        assert!(!the_work_moved(&last, told(true)));
        assert!(the_work_moved(&last, told(false)), "the job was answered and the page was not told");
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

    /// **Every act this screen draws has a route, and which of them are owed is
    /// one fact in one place.**
    ///
    /// **This is the guard against the fault the failure shape already paid
    /// for**: a control that does nothing. Every act name the five menus, the
    /// connect control and the two boxes carry is read out of the drawing
    /// itself, so a control added later with a name nothing routes fails here
    /// rather than on the user's press.
    #[test]
    fn every_act_the_drawing_carries_is_one_this_file_routes() {
        let mut acts: Vec<String> = Vec::new();
        for menu in the_five_menus() {
            acts.push(menu.the_connection.act.as_str().to_string());
            for control in menu
                .going_in
                .iter()
                .chain(menu.about_itself.iter())
                .chain(menu.the_details.iter())
            {
                acts.push(control.act.as_str().to_string());
            }
            acts.push(menu.removing.act.as_str().to_string());
        }
        // The two the boxes send, which are not menu items.
        acts.push("accept-the-identity".to_string());
        acts.push("stop-and-clear".to_string());
        // **And the three V13's row lines carry**, read off the lines
        // themselves. Step 10c's transfer wave piece 9.
        {
            use crate::servers::published::WhatSmoozyOnTheServerIs as Carries;
            let connected = WhatIsTrueOfTheServer {
                a_period_stands: true,
                the_primary_connection_is_live: true,
                a_run_of_attempts_is_in_flight: false,
                the_restart_mark_stands: false,
            };
            for carries in [Carries::TheUpdateWaits, Carries::TheUpdateFailed { said: "x".into() }] {
                let line = the_update_line(Some(&carries), connected, "box", None).expect("a line");
                if let Some(now) = &line.update_now {
                    acts.push(now.act.as_str().to_string());
                }
                if let Some(the_box) = &line.the_box {
                    acts.push(the_box.stop_jobs_and_update.act.as_str().to_string());
                }
                if let Some(words) = &line.the_servers_words {
                    acts.push(words.clear.act.as_str().to_string());
                }
            }
        }
        acts.sort();
        acts.dedup();

        // **The page takes nine of them itself** — the rename, the edit, the
        // removal, the address, the two ways into a server, *Update now…*,
        // which opens its box, and the two ways out of a gone key (step 10c's
        // piece 4), which reach no machine — and never reaches this file with
        // one.
        let the_pages_own = [
            "rename",
            "edit",
            "remove",
            "copy-address",
            "browse",
            "set-up",
            THE_UPDATE_NOW_IS_ASKED,
            "a-different-key",
            "fix-the-key",
        ];

        let mut owed: Vec<String> = Vec::new();
        for act in &acts {
            if the_pages_own.contains(&act.as_str()) {
                continue;
            }
            if matches!(where_an_act_goes(act), WhereAnActGoes::ItIsNotBuilt) {
                owed.push(act.clone());
            }
        }
        // **None is owed since step 10c's piece 4**, and the two the page
        // takes itself are handled there by name.
        let page = include_str!("../../web/servers.js");
        for own in ["a-different-key", "fix-the-key"] {
            assert!(
                page.contains(&format!("act === \"{own}\"")),
                "web/servers.js does not take {own} itself, so it would reach \
                 the application and be answered as not built"
            );
        }
        assert_eq!(
            owed,
            Vec::<String>::new(),
            "**an act this screen draws has no route, or one that had none has \
             gained one.** This file's head names what is owed and why, and a \
             list that has moved without the head moving with it is the fault \
             this build stage keeps meeting: a fact kept in two places"
        );
    }

    /// **Step 10c's piece 4, part 1: a server whose first *Add and set up*
    /// connection failed can be connected again.** `Servers` B1 writes the
    /// remembered setup on that failure, and from then on the row's only
    /// connect control — on the row and in the *Waiting to be set up* menu —
    /// is *Connect and set up* (`Servers` B6). **It answered that it was not
    /// built**, so the server could never connect at all. It goes to the
    /// user's ordinary connect, which carries the remembered setup: the period
    /// takes it at `Servers` B2's instant and hands it to `Server Setup` at
    /// `Servers` Q1 step 8 (`src/servers/a_period.rs`).
    #[test]
    fn a_server_waiting_to_be_set_up_has_a_connect_that_goes_somewhere() {
        let mut row = a_row();
        row.remembered_setup = Some(smoozy_servers::store::RememberedSetup {
            options: vec![INSTALL_THE_LATEST_STABLE_BLENDER.to_string()],
        });
        let control = the_connect_control(&row, what_is_true_of_a_server_with_no_period(), false);
        assert_eq!(control.act.as_str(), "connect-and-set-up");
        assert_eq!(
            where_an_act_goes(control.act.as_str()),
            WhereAnActGoes::ABeginning,
            "the row's connect control goes nowhere, so the server can never connect"
        );
        let menu = the_five_menus()
            .into_iter()
            .find(|menu| menu.which.as_str() == "waiting-to-be-set-up")
            .expect("the menu");
        assert_eq!(
            where_an_act_goes(menu.the_connection.act.as_str()),
            WhereAnActGoes::ABeginning,
            "the menu's connect goes nowhere"
        );
    }

    /// **Step 10c's piece 4, part 1: *Connect without setup* drops the
    /// remembered setup** — `Servers` B5 item 2, one
    /// durable write — so the connect that follows carries none: the period
    /// takes only what the row holds at `Servers` B2's instant
    /// (`src/servers/a_period.rs`), and the row now draws the plain *Connect*.
    #[test]
    fn dropping_the_setup_clears_it_and_the_row_connects_plainly() {

        let started = a_started("drop-the-setup");
        let connection = started.record.as_ref().expect("a record").connection();
        let key = a_key_row(Some("SHA256:good"));
        key_rows::add(connection, &key).expect("a key row");
        let identifier = key.identifier.as_text();
        let add = TheAdd {
            install_blender: true,
            ..an_add(&identifier)
        };
        the_add_is_performed(&started, &add);
        let before = the_rows_of(&started).remove(0);
        assert!(before.remembered_setup.is_some(), "the add remembered nothing");

        assert_eq!(where_an_act_goes("drop-the-setup"), WhereAnActGoes::TheSetupIsDropped);
        assert_eq!(
            the_remembered_setup_is_dropped(&started, &before.local_server_identifier),
            TheAnswer::ItWasSaved
        );
        let after = the_rows_of(&started).remove(0);
        assert!(after.remembered_setup.is_none(), "the setup was not dropped");
        let mut unchanged = after.clone();
        unchanged.remembered_setup = before.remembered_setup.clone();
        assert_eq!(unchanged, before, "the drop wrote more than the setup");
        assert_eq!(
            the_connect_control(&after, what_is_true_of_a_server_with_no_period(), false)
                .act
                .as_str(),
            "connect"
        );
        assert_eq!(
            the_remembered_setup_is_dropped(&started, "no-such-server"),
            TheAnswer::TheServerIsGone
        );
    }

    /// **Step 10c's piece 4, part 2: *Change key…* saves only the
    /// key, and the mark clears** (`Servers` P15).
    /// The page saves the picked key through the edit’s own save with
    /// the saved details unchanged, so that is what is driven here.
    #[test]
    fn a_different_key_saves_only_the_key_and_the_mark_clears() {

        let started = a_started("a-different-key");
        let connection = started.record.as_ref().expect("a record").connection();
        let gone = a_key_row(Some("SHA256:gone"));
        let good = a_key_row(Some("SHA256:good"));
        key_rows::add(connection, &gone).expect("a key row");
        key_rows::add(connection, &good).expect("a key row");
        let identifier = gone.identifier.as_text();
        let add = TheAdd {
            label: "Kitchen",
            ..an_add(&identifier)
        };
        the_add_is_performed(&started, &add);
        let before = the_rows_of(&started).remove(0);
        let keys = vec![gone.clone(), good.clone()];
        let said = WhatKeysSaid {
            the_tools_are_installed: true,
            verdicts: vec![
                (gone.identifier.as_text(), usable::Verdict::Unusable(usable::Unusable::FileMissing)),
                (good.identifier.as_text(), usable::Verdict::Usable),
            ],
            ..WhatKeysSaid::default()
        };
        assert!(the_key_mark(&before, &keys, Some(&said)), "the row was never marked");
        let dialog = the_edit_dialog_from(&before, std::slice::from_ref(&before), &keys, Some(&said));
        let save = |started: &Started| {
            the_edit_is_performed(
                started,
                None,
                &before.local_server_identifier,
                dialog.the_host.as_str(),
                dialog.the_port.as_str(),
                dialog.the_username.as_str(),
                &good.identifier.as_text(),
                dialog.the_name.as_str(), None,
            )
        };

        // A valid key repair needs no paid state.

        assert_eq!(save(&started), TheAnswer::ItWasSaved);
        let after = the_rows_of(&started).remove(0);
        assert_eq!(after.key.local_key_identifier, good.identifier.as_text());
        assert_eq!(after.key.fingerprint, "SHA256:good");
        let mut only_the_key = after.clone();
        only_the_key.key = before.key.clone();
        assert_eq!(only_the_key, before, "more than the key was saved");
        assert!(!the_key_mark(&after, &keys, Some(&said)), "the mark did not clear");
    }

    /// **Step 10c's piece 4, part 2, the page's half**: *Give it a different
    /// key…* is drawn closed under a lock and picks through Keys into the
    /// edit's own save; *Fix in Keys…* goes to Keys at the row's key.
    #[test]
    fn the_page_takes_the_two_ways_out_of_a_gone_key() {
        let page = include_str!("../../web/servers.js");
        let different = &page[page.find("function aDifferentKeyIsChosen(").expect("the act")..];
        let different = &different[..different.find("\n}\n").expect("its end")];
        assert!(different.contains("if (!server) return;"));
        assert!(different.contains("aServerDialogPicksAKey("));
        assert!(different.contains("\"an_edit_is_saved\""));
        assert!(different.contains("keyIdentifier: held.key"));
        let fix = &page[page.find("function theKeyIsFixedInKeys(").expect("the act")..];
        let fix = &fix[..fix.find("\n}\n").expect("its end")];
        assert!(fix.contains("theCurrentPlaceIs(KEYS);"));
        assert!(fix.contains("theKeysRowTakesTheKeyboard(said.the_key_identifier);"));
        assert!(!fix.contains("theKeyChoiceIsClosed"), "a lock closed the way out it leaves open");
    }

    /// **Fault 7 of the second pass, 2026-09-21: the Add dialog's Name field
    /// takes a press anywhere in its drawn box**, not only on the third of it
    /// the input fills. A page harness run against the committed page in a
    /// headless browser measured the cause: the drawn field 384 pixels wide,
    /// the input 150, and a press in the middle of the field landing on
    /// *optional*, which cannot take focus, so the dialog took it instead. The
    /// harness then measured the repair handing the press to the input. **This
    /// holds the page to the repair**, which is all a test at this desk can see
    /// of it.
    #[test]
    fn the_add_dialogs_name_field_hands_a_press_anywhere_in_it_to_its_input() {
        let page = include_str!("../../web/servers.js");
        let at = page
            .find("nameHolder.className = \"fld wide svnamefield\";")
            .expect("the Add dialog draws its Name field");
        let after = &page[at..];
        let handled = after
            .find("nameHolder.addEventListener(\"mousedown\", (press) => {")
            .expect("a press on the Name field's box is handled");
        let body = &after[handled..handled + 200];
        assert!(body.contains("if (press.target === name) return;"));
        assert!(body.contains("press.preventDefault();"));
        assert!(body.contains("name.focus();"));
        let sheet = include_str!("../../web/servers.css");
        assert!(sheet.contains(".svnamefield { display: flex; align-items: center; cursor: text; }"));
    }

    /// **The page sends the act names this file routes**, and nothing holds the
    /// two sides together but this.
    #[test]
    fn the_page_sends_the_acts_this_file_routes() {
        let page = include_str!("../../web/servers.js");
        // **The row's own acts travel by the name the drawing gave them**, so
        // the page names none of them: it forwards whatever the control
        // carried. What it does name are the two the boxes send, which are not
        // menu items and are written on the page's own side.
        for act in ["accept-the-identity", "stop-and-clear"] {
            assert!(
                page.contains(&format!("\"{act}\"")),
                "web/servers.js never names {act}, which the box that sends it \
                 has to carry"
            );
        }
        assert!(
            page.contains("an_act_that_needs_a_connection"),
            "web/servers.js does not call the one command every act on a row \
             goes through"
        );
        assert!(
            page.contains("servers:what-is-observed"),
            "**`Servers` K7: that module supplies the observed value and \
             presents neither**, so the page has to be listening for it or a \
             connected server is one nothing redraws"
        );
    }

    /// **`Design/Servers`: a server Smoozy is repairing takes the connected
    /// menu, and its first slot reads *disconnect*.** `Servers` K8a is the
    /// mechanism and Row 348 is the promise.
    #[test]
    fn a_repairing_server_takes_the_connected_menu() {
        let of = WhatIsTrueOfTheServer {
            a_period_stands: true,
            the_primary_connection_is_live: false,
            a_run_of_attempts_is_in_flight: true,
            the_restart_mark_stands: false,
        };
        assert_eq!(which_menu(&a_row(), of, false), "connected");
        let menus = the_five_menus();
        let connected = menus
            .iter()
            .find(|menu| menu.which.as_str() == "connected")
            .expect("the connected menu");
        assert_eq!(connected.the_connection.act.as_str(), "disconnect");
    }

    /// **`Servers` P7: the connect action is withdrawn for a server marked as
    /// needing a new key.** Connecting would fail, so the action is not
    /// offered.
    #[test]
    fn a_server_needing_a_new_key_is_offered_no_connect() {
        let of = what_is_true_of_a_server_with_no_period();
        let control = the_connect_control(&a_row(), of, true);
        assert!(control.unavailable);
        assert_eq!(which_menu(&a_row(), of, true), "needs-a-new-key");
    }

    /// **`Design/Servers § Connected and needing a new key at once`: the two
    /// readings are independent and the connected menu takes the two ways out
    /// in slot 3, with nothing shown unavailable.**
    #[test]
    fn connected_with_the_key_gone_shows_nothing_unavailable() {
        let menus = the_five_menus();
        let both = menus
            .iter()
            .find(|menu| menu.which.as_str() == "connected-key-gone")
            .expect("the pair's menu");
        assert_eq!(both.about_itself.len(), 2);
        assert!(!both.the_connection.unavailable);
        for item in &both.going_in {
            assert!(
                !item.unavailable,
                "**Nothing is shown unavailable, because nothing is**: disconnect, \
                 browse and set up all work"
            );
        }
    }

    /// **The colour arrives already picked and it is one no other server is
    /// using**, and with more than eight it is the hue used by the fewest.
    #[test]
    fn the_colour_is_picked_and_never_refused() {
        assert_eq!(the_colour_picked_for_you(&[]), "b1");
        assert_eq!(the_colour_picked_for_you(&["b1", "b2"]), "b3");
        let all_eight: Vec<&str> = crate::surface::the_rail::THE_EIGHT_BADGES.to_vec();
        // Every hue taken once, and `b3` taken twice: the fewest is any of the
        // other seven, and the answer is never a refusal.
        let mut crowded = all_eight.clone();
        crowded.push("b3");
        let picked = the_colour_picked_for_you(&crowded);
        assert_ne!(picked, "b3");
        assert!(all_eight.contains(&picked));
    }

    /// **`Servers` W19's table, over every one of its six rows.**
    ///
    /// The row that matters most is the third: **a repairing server whose
    /// last-known reading holds work is offered *leave it be* only**, and it is
    /// the one row that may be switched off.
    #[test]
    fn the_removal_box_follows_the_table_and_only_one_row_may_be_switched_off() {
        let row = a_row();
        let all = vec![row.clone()];
        let stands = |live: bool, in_flight: bool| WhatIsTrueOfTheServer {
            a_period_stands: true,
            the_primary_connection_is_live: live,
            a_run_of_attempts_is_in_flight: in_flight,
            the_restart_mark_stands: false,
        };
        let work = TheLastKnownReading::Holds {
            rendering: 1,
            waiting: 2,
        };

        let both = the_removal_box_from(&row, &all, stands(true, false), work, false);
        assert_eq!(both.offers.as_str(), "both");
        assert!(
            both.the_switch_off.is_none(),
            "**the connected warning has no switch at all**, and the difference \
             is stated rather than left to be noticed"
        );

        let repairing = the_removal_box_from(&row, &all, stands(false, true), work, false);
        assert_eq!(repairing.offers.as_str(), "leave-it-be-only");
        assert!(repairing.stop_and_clear.is_none());
        assert!(repairing.the_switch_off.is_some());

        let nothing_known = the_removal_box_from(
            &row,
            &all,
            what_is_true_of_a_server_with_no_period(),
            TheLastKnownReading::NoRecordAtAll,
            false,
        );
        assert_eq!(nothing_known.offers.as_str(), "a-plain-confirmation");
        assert!(nothing_known.the_warning.is_none());
        assert!(nothing_known.remove.is_some());
    }

    /// The prose and values of a run of [`Words`], in order, as the page sets
    /// them side by side.
    fn the_runs_read(runs: &[Words]) -> Vec<String> {
        let mut out = Vec::new();
        for run in runs {
            if !run.lead.as_str().is_empty() {
                out.push(run.lead.as_str().to_string());
            }
            for value in &run.values {
                out.push(value.as_str().to_string());
            }
            if let Some(rest) = &run.rest {
                out.push(rest.as_str().to_string());
            }
        }
        out
    }

    /// **Step 10a round 1, fix-here item 17: the warning and both statements are
    /// composed from the last-known reading's own counts**, as words beside
    /// values — **and it fails on the placeholder the box used to show**, which
    /// read *Remove this server?* with no counts and said *one* waiting job
    /// whatever the count was.
    #[test]
    fn the_removal_warning_carries_the_readings_own_counts() {
        let row = a_row();
        let all = vec![row.clone()];
        let connected = WhatIsTrueOfTheServer {
            a_period_stands: true,
            the_primary_connection_is_live: true,
            a_run_of_attempts_is_in_flight: false,
            the_restart_mark_stands: false,
        };
        let drawn = the_removal_box_from(
            &row,
            &all,
            connected,
            TheLastKnownReading::Holds {
                rendering: 1,
                waiting: 2,
            },
            false,
        );
        let warning = the_runs_read(&drawn.the_warning.expect("a warning"));
        assert_eq!(
            warning,
            vec![
                "1".to_string(),
                Prose::new(words::ONE_JOB_IS_RENDERING).as_str().to_string(),
                "2".to_string(),
                Prose::new(words::MANY_ARE_WAITING).as_str().to_string(),
                Prose::new(words::RENDERING_AND_WAITING).as_str().to_string(),
            ]
        );
        let stop = drawn.stop_and_clear.expect("stop and clear is offered");
        assert_eq!(stop.said.values[0].as_str(), "2", "the waiting count is not the reading's");
        assert_ne!(stop.said.lead.as_str(), Prose::new(words::STOP_AND_CLEAR_SAID_ONE).as_str());

        let one = the_removal_box_from(
            &row,
            &all,
            connected,
            TheLastKnownReading::Holds {
                rendering: 1,
                waiting: 1,
            },
            false,
        );
        assert_eq!(
            one.stop_and_clear.expect("offered").said.lead.as_str(),
            Prose::new(words::STOP_AND_CLEAR_SAID_ONE).as_str()
        );
    }

    /// **`Servers` W19 row three speaks of what Smoozy last knew and says so** —
    /// step 10a round 1's first fix-upstream item, carried into item 17. It fails on the
    /// present-tense words the disconnected box used to carry.
    #[test]
    fn the_disconnected_warning_says_it_is_what_smoozy_last_knew() {
        let row = a_row();
        let repairing = WhatIsTrueOfTheServer {
            a_period_stands: true,
            the_primary_connection_is_live: false,
            a_run_of_attempts_is_in_flight: true,
            the_restart_mark_stands: false,
        };
        let drawn = the_removal_box_from(
            &row,
            std::slice::from_ref(&row),
            repairing,
            TheLastKnownReading::Holds {
                rendering: 1,
                waiting: 2,
            },
            false,
        );
        let warning = the_runs_read(&drawn.the_warning.expect("a warning")).join(" ");
        assert!(warning.starts_with(Prose::new(words::WHEN_SMOOZY_WAS_LAST_CONNECTED).as_str()));
        assert!(!warning.contains(Prose::new(words::RENDERING_AND_WAITING).as_str()));
        let leave = drawn.leave_it_running.expect("leave it be is offered");
        assert_eq!(
            leave.said.lead.as_str(),
            Prose::new(words::LEAVE_IT_RUNNING_SAID_LAST_KNOWN).as_str()
        );
    }

    /// **`Servers` W35 and `Servers` W39: where both of `Servers` W31's sources are empty there is
    /// no warning, and that is a true statement rather than ignorance.**
    #[test]
    fn the_edit_warning_names_jobs_with_no_learned_ending_and_nothing_else() {
        assert!(the_edit_warning(&[WhatChanged::HostOrPort], &[]).is_none());
        let one = the_edit_warning(&[WhatChanged::HostOrPort], &["Kitchen — final".to_string()])
            .expect("a warning");
        assert_eq!(one.how_many.as_str(), "1");
        assert_eq!(one.named[0].as_str(), "Kitchen — final");
        let by_username =
            the_edit_warning(&[WhatChanged::Username], &["Kitchen — final".to_string()])
                .expect("a warning");
        // **`Servers` W34: never *queued or running*.**
        for said in [
            one.lead.as_str(),
            one.have_not_ended.as_str(),
            one.what_becomes_of_them.as_str(),
            by_username.lead.as_str(),
            by_username.have_not_ended.as_str(),
            by_username.what_becomes_of_them.as_str(),
        ] {
            assert!(
                !said.contains("running") && !said.contains("queued"),
                "**`Servers` W34: the warning names jobs Smoozy has not seen end, never \
                 jobs queued or running** — the durable record carries no line \
                 position and a job with no learned ending may have ended hours \
                 ago"
            );
        }
    }

    /// **`Servers` W31's second source is read, and `Servers` W37 decides which
    /// typed changes carry the warning** — the Gemini catch-up of step 10a,
    /// Area 5 (F-1).
    ///
    /// **Before 2026-09-19 every case below answered nothing**: the dialog
    /// handed the warning an empty list, so a host change over an unended job
    /// said nothing, and the check as the user typed returned no warning at
    /// all. The records are written through the record's own writers.
    #[test]
    fn the_edit_warning_is_composed_from_the_local_job_records_as_typed() {
        use crate::record::deletions::tests::a_record_in_history;
        use crate::record::job_records::JobRecords;
        use smoozy_queue_history::acting::note::{
            TheEndingOnTheNote, TheNote, TheOutcomeAsWritten,
        };

        let mut folder = std::env::temp_dir();
        folder.push(format!("smoozy-edit-warning-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder");
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3"))
            .expect("the record opens");
        let row = a_row();
        let server = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier)
            .expect("the row's identity");
        let as_typed = |host: &str, port: &str, username: &str| {
            the_edit_warning_as_typed(Some(record.connection()), &row, host, port, username)
        };

        // **No records at all: no warning** (`Servers` W35).
        assert!(as_typed("10.0.0.9", "40122", "root").is_none());

        // One job with no learned ending, and one that ended.
        let store = JobRecords::over(record.connection());
        let unended = a_record_in_history(&server, "Kitchen — final");
        store.write_the_provisional_record(&unended).expect("written");
        let ended = a_record_in_history(&server, "Hallway");
        store.write_the_provisional_record(&ended).expect("written");
        let mut note = TheNote::created_with_the_job_record("Hallway");
        note.ending = Some(TheEndingOnTheNote {
            outcome: TheOutcomeAsWritten::Finished,
            exit_status: Some(0),
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            the_device_line_as_observed: None,
        });
        store.the_fill_in(&ended.job, &note).expect("filled in");
        // And one on another server, which is no business of this one.
        let elsewhere = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        store
            .write_the_provisional_record(&a_record_in_history(&elsewhere, "Elsewhere"))
            .expect("written");

        // **Nothing changed: no warning** — the dialog at opening.
        assert!(as_typed("213.181.122.44", "40122", "root").is_none());

        // **Host changed: the warning names the one unended job.**
        let host = as_typed("10.0.0.9", "40122", "root").expect("a host change warns");
        assert_eq!(host.how_many.as_str(), "1");
        assert_eq!(
            host.named.iter().map(|one| one.as_str()).collect::<Vec<_>>(),
            vec!["Kitchen — final"]
        );
        assert_eq!(host.lead.as_str(), Prose::new(words::POINTS_AT_A_DIFFERENT_SERVER).as_str());
        // **Port changed: the same words.**
        let port = as_typed("213.181.122.44", "40123", "root").expect("a port change warns");
        assert_eq!(port.lead.as_str(), host.lead.as_str());

        // **Username only: `Design/Servers § The Edit dialog`'s username words.**
        let username = as_typed("213.181.122.44", "40122", "artist").expect("a username change warns");
        assert_eq!(
            username.lead.as_str(),
            Prose::new(words::LOGS_IN_AS_A_DIFFERENT_USER).as_str()
        );
        assert_eq!(
            username.what_becomes_of_them.as_str(),
            Prose::new(words::CANNOT_REACH_THEM_AFTER).as_str()
        );
        assert_eq!(username.how_many.as_str(), "1");
        // **Both: the host words**, the jobs being left on another machine.
        let both = as_typed("10.0.0.9", "40122", "artist").expect("both warn");
        assert_eq!(both.lead.as_str(), host.lead.as_str());

        // **A rename or a key change is not typed through this check at all**:
        // the key and the name are read off the saved row, so neither can
        // change what is composed (`Servers` W37).
        assert!(the_edit_warning(&[WhatChanged::TheLabelOrTheColour], &["Kitchen — final".to_string()]).is_none());
        assert!(the_edit_warning(&[WhatChanged::TheKey], &["Kitchen — final".to_string()]).is_none());

        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **The page draws the warning from each check's answer and removes it
    /// when an answer carries none** — the Gemini catch-up of step 10a, Area 5
    /// (F-1 (d)). Before 2026-09-19 the page drew it once at opening, from the
    /// dialog's own facts, and the check sent no username.
    #[test]
    fn the_edit_dialog_draws_the_warning_from_each_check() {
        let page = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("web")
                .join("servers.js"),
        )
        .expect("the page module is readable");
        let dialog = page
            .split_once("function theEditDialogIsDrawn(")
            .expect("the edit dialog is drawn")
            .1;
        let dialog = &dialog[..dialog.find("\n}\n").expect("it closes")];
        assert!(
            !dialog.contains("said.the_warning"),
            "the edit dialog still draws a warning from its opening facts"
        );
        assert!(dialog.contains("username: username.value"), "the check does not send the username");
        assert!(
            dialog.contains("for (const field of [host, port, username, name])"),
            "the check does not run on every field `Servers` W37 names"
        );
        assert!(dialog.contains("    warning,\n"), "the dialog does not hand its warning line to the check's drawing");
        let checks = page
            .split_once("function theChecksAreDrawn(said) {")
            .expect("the checks are drawn")
            .1;
        assert!(
            checks[..checks.find("\n}\n").expect("it closes")]
                .contains("theEditWarningIsDrawn(refusals.warning, said)"),
            "the check's answer does not draw the warning"
        );
        let drawn = page
            .split_once("function theEditWarningIsDrawn(warning, said) {")
            .expect("the warning is drawn from the answer")
            .1;
        assert!(
            drawn[..drawn.find("\n}\n").expect("it closes")]
                .contains("warning.hidden = !said.theEditWarning;"),
            "an answer carrying no warning does not remove the one drawn"
        );
    }

    /// **The warning travels on the check's answer, each part on its own
    /// route** — P6: results reach the page as events.
    #[test]
    fn the_edit_warning_travels_on_the_checks_answer() {
        let warning = the_edit_warning(
            &[WhatChanged::HostOrPort],
            &["Kitchen — final".to_string(), "Hallway".to_string()],
        );
        let delivered = |event: &Event| {
            use std::sync::{Arc, Mutex};
            struct Held(Arc<Mutex<Vec<crate::events::Delivery>>>);
            impl crate::events::Delivers for Held {
                fn deliver(&self, delivery: &crate::events::Delivery) {
                    if let Ok(mut held) = self.0.lock() {
                        held.push(delivery.clone());
                    }
                }
            }
            let held = Arc::new(Mutex::new(Vec::new()));
            let queue = TheOneQueue::new();
            queue.the_page_is_ready(Box::new(Held(held.clone())));
            let _ = queue.read_then_post::<(), std::convert::Infallible>(|| Ok(()), |_| event.clone());
            let out = held.lock().expect("the recorder").remove(0);
            out
        };
        let event = the_edit_warning_said(the_checks_on(&[], "10.0.0.9", "22", ""), warning.as_ref());
        let said = delivered(&event).said;
        assert_eq!(
            said.get("theEditWarningNamedCount"),
            Some(&Said::Value(Verbatim::new("2")))
        );
        assert_eq!(
            said.get("theEditWarningNamed1"),
            Some(&Said::Value(Verbatim::new("Hallway")))
        );
        assert!(matches!(said.get("theEditWarning"), Some(Said::Prose(_))));
        let none = the_edit_warning_said(the_checks_on(&[], "10.0.0.9", "22", ""), None);
        assert!(delivered(&none).said.get("theEditWarning").is_none());
    }

    /// **`Servers` D12's cut, over the four fields the dialog can change.**
    #[test]
    fn the_edit_is_cut_on_the_field_that_changed() {
        let row = a_row();
        let same_host = row.host.clone();
        let same_login = row.username.clone();
        let other_host = Host::what_the_user_typed("10.0.0.9").expect("a host");
        let other_login = Username::what_the_user_typed("artist").expect("a login");
        let other_port = Port::what_the_user_typed(2222).expect("a port");
        assert_eq!(
            what_changed(&row, &other_host, row.port, &same_login, "key-1", ""),
            vec![WhatChanged::HostOrPort]
        );
        assert_eq!(
            what_changed(&row, &same_host, other_port, &same_login, "key-1", ""),
            vec![WhatChanged::HostOrPort]
        );
        assert_eq!(
            what_changed(&row, &same_host, row.port, &other_login, "key-1", ""),
            vec![WhatChanged::Username]
        );
        assert_eq!(
            what_changed(&row, &same_host, row.port, &same_login, "key-2", ""),
            vec![WhatChanged::TheKey]
        );
        assert_eq!(
            what_changed(&row, &same_host, row.port, &same_login, "key-1", "Overnight box"),
            vec![WhatChanged::TheLabelOrTheColour]
        );
        assert!(what_changed(&row, &same_host, row.port, &same_login, "key-1", "").is_empty());
        // **Step 10a round 1, fix-here item 3: every changed field, not the
        // first.** The single answer this gave filed a port-and-key edit under
        // the port alone, so the key change passed the lock; and a host-and-
        // username edit under the host alone, so the password was kept.
        assert_eq!(
            what_changed(&row, &same_host, other_port, &same_login, "key-2", ""),
            vec![WhatChanged::HostOrPort, WhatChanged::TheKey]
        );
        assert_eq!(
            what_changed(&row, &other_host, row.port, &other_login, "key-1", ""),
            vec![WhatChanged::HostOrPort, WhatChanged::Username]
        );
    }

    /// **`Servers` D12 and `Servers` D13, applied: a host change drops all four
    /// machine-derived values in the same write, and the remembered setup
    /// survives every edit.**
    #[test]
    fn a_host_change_drops_all_four_and_the_setup_survives() {
        let mut row = a_row();
        row.remembered_hardware_reading = Some(a_reading());
        row.event_cursor = Some(9);
        row.last_confirmed_installation_identity = Some("inst".to_string());
        row.remembered_host_key =
            Some(smoozy_servers::store::RememberedHostKey::approved(vec![1, 2, 3]));
        row.remembered_setup = Some(smoozy_servers::store::RememberedSetup {
            options: vec!["ready".to_string()],
        });
        apply_what_the_edit_throws_away(
            &mut row,
            smoozy_servers::edits::what_the_edit_throws_away(WhatChanged::HostOrPort),
        );
        assert!(row.remembered_hardware_reading.is_none());
        assert!(row.event_cursor.is_none());
        assert!(row.last_confirmed_installation_identity.is_none());
        assert!(row.recorded_machine_root.is_none());
        assert!(row.remembered_host_key.is_none());
        assert!(
            row.remembered_setup.is_some(),
            "**`Servers` B7: the remembered setup survives every edit**, which is what it \
             is for — an add whose connection failed on a wrong address is put \
             right by editing the address and connecting"
        );
    }

    /// **`Servers` D15: changing the key clears nothing**, and in particular
    /// does not touch the remembered host key.
    #[test]
    fn a_key_change_clears_nothing() {
        let mut row = a_row();
        row.remembered_host_key =
            Some(smoozy_servers::store::RememberedHostKey::approved(vec![1]));
        row.remembered_hardware_reading = Some(a_reading());
        apply_what_the_edit_throws_away(
            &mut row,
            smoozy_servers::edits::what_the_edit_throws_away(WhatChanged::TheKey),
        );
        assert!(row.remembered_host_key.is_some());
        assert!(row.remembered_hardware_reading.is_some());
    }

    /// **The two non-blocking statements are two different statements and must
    /// not read alike** — `Design/Servers § Adding a server`.
    #[test]
    fn the_two_checks_are_two_statements_and_neither_blocks() {
        let mut row = a_row();
        row.label = Some("Overnight box".to_string());
        let rows = vec![row];

        let same_address = the_checks_on(&rows, "213.181.122.44", "40122", "");
        let said = the_said_of(&same_address);
        assert!(said.iter().any(|name| name == "mayAlreadyHave"));
        assert!(!said.iter().any(|name| name == "alreadyCalled"));

        let same_name = the_checks_on(&rows, "10.0.0.1", "22", "Overnight box");
        let said = the_said_of(&same_name);
        assert!(said.iter().any(|name| name == "alreadyCalled"));
        assert!(
            !said.iter().any(|name| name == "mayAlreadyHave"),
            "**the name one says exactly that and nothing about addresses**"
        );

        // **A different port is a different server** — `Servers` C3.
        let other_port = the_checks_on(&rows, "213.181.122.44", "40123", "");
        assert!(!the_said_of(&other_port).iter().any(|name| name == "mayAlreadyHave"));
    }

    /// **The port refuses in place, and it is the exception** — every other
    /// field says so only once the user has asked to add.
    #[test]
    fn the_port_refuses_in_place() {
        let refused = the_checks_on(&[], "10.0.0.1", "0", "");
        assert!(the_said_of(&refused).iter().any(|name| name == "thePortWillNotDo"));
        let taken = the_checks_on(&[], "10.0.0.1", "22", "");
        assert!(!the_said_of(&taken).iter().any(|name| name == "thePortWillNotDo"));
    }

    /// **Every name the event can carry is read by the page.**
    ///
    /// **This is piece 1's own test, and it catches the fault in the direction
    /// nothing else does**: a string the application sends and the page drops
    /// is a word the user does not see, with every other test passing.
    #[test]
    fn every_name_the_event_can_carry_is_read_by_the_page() {
        let page = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/servers.js"),
        )
        .expect("the page module is readable");

        let mut every = Vec::new();
        for answer in [
            TheAnswer::ItWasSaved,
            TheAnswer::TheseFieldsAreNeeded(vec![
                ("host", words::HOST_IS_NEEDED),
                ("username", words::USERNAME_IS_NEEDED),
                ("key", words::A_KEY_IS_NEEDED),
            ]),
            TheAnswer::ThePortWillNotDo,
            TheAnswer::TheRecordWouldNotTakeIt,
            TheAnswer::TheActIsNotBuiltYet,
            TheAnswer::TheServerIsGone,
            TheAnswer::ItWasSavedAndNothingStarted,
            TheAnswer::ThePasswordWasForgotten,
            TheAnswer::TheConnectionWasNotMade,
            TheAnswer::ItIsNotALinuxMachine,
            TheAnswer::TheUpdateNowWasTaken,
            TheAnswer::TheStopAndClearDidNotFullyWork(TheRemovalsEnding::ItHalfWorked),
        ] {
            every.extend(the_said_of(&the_answer_event(answer)));
        }
        let mut row = a_row();
        row.label = Some("Overnight box".to_string());
        every.extend(
            the_said_of(&the_checks_on(&[row], "213.181.122.44", "0", "Overnight box")),
        );
        // **A connect that did not happen, naming its server** — the Creator's
        // note 14 on the test copy.
        every.extend(the_said_of(&the_answer_about(
            TheAnswer::TheConnectionWasNotMade,
            &TheServerItWasAbout {
                server: "server-1".to_string(),
                name: Some("Hyperstack".to_string()),
            },
        )));

        for name in every {
            // **Two families are read by index rather than by name**, so the
            // page holds a template where the event holds `field0`. Their own
            // reader is asserted by name below.
            if name.starts_with("field") {
                assert!(page.contains("`field${at}`"));
                continue;
            }
            assert!(
                page.contains(&name),
                "the application can put {name:?} on the event and web/servers.js \
                 never reads it. **A string the application sends and the page \
                 drops is a word the user does not see**, with every other test \
                 passing"
            );
        }
    }

    /// **The page names the same event kind and the same act tokens.**
    ///
    /// Nothing holds the two sides together at compile time; this is the check
    /// written out, the way piece 2's account said it would have to be.
    #[test]
    fn the_page_names_the_same_kind_and_the_same_acts() {
        let page = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/servers.js"),
        )
        .expect("the page module is readable");
        assert!(page.contains(THE_ANSWER));
        // **The page routes a row to its menu by comparing two values the
        // application supplied**, so it holds none of the five names — which is
        // the right shape and is asserted rather than assumed.
        for menu in the_five_menus() {
            assert!(
                !page.contains(&format!("\"{}\"", menu.which.as_str())),
                "the page writes one of the five menu names as a literal. **The \
                 application says which menu a row takes and the page compares \
                 two values it was given**; a name written on the page is the \
                 same fact in two places"
            );
        }
        assert!(page.contains("row.the_menu"));
        // **The six acts the page answers itself** (the rename since the
        // Creator's note 1 on 0.1.8). Every other act on this screen reaches a
        // machine or a module this application does not call, and the page
        // hands each of those straight to `an_act_that_needs_a_connection`
        // rather than knowing its name — which is why the list below is six
        // rather than twelve.
        for act in ["rename", "edit", "remove", "copy-address", "browse", "set-up"] {
            assert!(
                page.contains(&format!("\"{act}\"")),
                "the application composes the act {act:?} and the page routes on \
                 nothing of that name"
            );
        }
        assert!(
            page.contains("an_act_that_needs_a_connection"),
            "the page has stopped answering the acts that need a connection. \
             **Rule 8 makes Smoozy going quiet a fault however the trouble \
             arose**, and a control that does nothing twice is what L8a is \
             written against"
        );
    }

    /// **Nothing on this piece's path composes a failure sentence, so
    /// `WhoseSentenceItIs` is not reached** — said here rather than left
    /// unmentioned, because a piece that leaves it unmentioned is
    /// indistinguishable from one that got it wrong.
    ///
    /// **The one sentence about something not working is not a failure record's
    /// and is not presented as one**: it says the act is not built, which is a
    /// fact about this application rather than a condition or an ending.
    #[test]
    fn no_path_here_composes_a_failure_sentence() {
        let source = the_working_half();
        assert!(
            !source.contains("WhoseSentenceItIs"),
            "this piece has begun composing failure sentences. **`Desktop app and \
             delivery` M32: the capture belongs to whatever composes the sentence \
             the user reads**, so pass `TheComposersOwn` — and get it wrong and \
             there is no error, no failing test and no visible fault, only a \
             problem report months later"
        );
    }

    // ── Step 10a round 1: the acts, driven over a throwaway record ──────────
    //
    // **Every one of these opens a record in the temporary folder and never the
    // Creator's**, and the one store any of them reaches is a stand-in.

    use smoozy_security_and_licence::held::{HeldSecret, Subject};
    use smoozy_security_and_licence::plaintext::Plaintext;
    use smoozy_security_and_licence::secret_store::{GetFailure, ProbeOutcome, StoreFailure};

    /// A stand-in secret store that counts erases and answers as told.
    struct AStandInStore {
        erases: Mutex<usize>,
        answer: Result<(), StoreFailure>,
    }

    impl AStandInStore {
        fn answering(answer: Result<(), StoreFailure>) -> AStandInStore {
            AStandInStore {
                erases: Mutex::new(0),
                answer,
            }
        }
        fn erased(&self) -> usize {
            *self.erases.lock().expect("the counter")
        }
    }

    impl SecretStore for AStandInStore {
        fn put(&self, _: &HeldSecret, _: Plaintext, _: Duration) -> Result<(), StoreFailure> {
            Ok(())
        }
        fn get(&self, _: &HeldSecret, _: Duration) -> Result<Plaintext, GetFailure> {
            Err(GetFailure::NotThere)
        }
        fn erase(&self, _: &HeldSecret, _: Duration) -> Result<(), StoreFailure> {
            *self.erases.lock().expect("the counter") += 1;
            self.answer
        }
        fn probe(&self, _: Duration) -> ProbeOutcome {
            ProbeOutcome::RoundTripWorked
        }
        fn enumerate(&self, _: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(Vec::new())
        }
    }

    /// A spine over a throwaway record, as the application's own would be.
    fn a_started(name: &str) -> Started {
        let path = crate::scratch_folders::a_folder(format!("smoozy-servers-surface-{name}-{}", std::process::id()));
        let record = crate::record::LocalRecord::open_at(&path.join("local-record.sqlite3"))
            .expect("a throwaway record");
        Started {
            the_sweep_left_behind: Vec::new(),
            record: Some(record),
            record_trouble: None,
            diary: None,
            declarations: smoozy_reporting::lines::Declarations::new(),
            what_modules_declare: crate::presentation::declarations::TheDeclarations::default(),
            said_once: crate::presentation::said_once::TheThingsSaidOnce::new(),
            run: None,
            the_previous_run: smoozy_state::run_mark::WhatTheLaunchFound::NoPreviousRun,

        }
    }

    fn a_key_row(fingerprint: Option<&str>) -> smoozy_keys::rows::KeyRow {
        smoozy_keys::rows::KeyRow::entering(
            Identity::issue(IdentityKind::LocalKey).expect("randomness"),
            std::path::PathBuf::from("/nowhere/id_ed25519"),
            fingerprint.map(|printed| smoozy_keys::fingerprint::Reading {
                value: smoozy_keys::fingerprint::Fingerprint::from_the_printed_form(printed)
                    .expect("a fingerprint"),
                read_from: smoozy_keys::fingerprint::ReadFrom::PrivateFile,
            }),
            None,
            false,
        )
        .expect("a key row")
    }

    fn an_add<'a>(key: &'a str) -> TheAdd<'a> {
        TheAdd {
            host: "213.181.122.44",
            port: "40122",
            username: "root",
            key_identifier: key,
            label: "",
            colour: "b1",
            get_ready: false,
            install_blender: false,
        }
    }

    fn the_rows_of(started: &Started) -> Vec<ServerRow> {
        the_rows(started).expect("the rows read")
    }

    /// **Fix-here item 2(d): an add whose key identifier names no key row, or
    /// a key row with no fingerprint, is refused on the key field and never
    /// saved.** It fails on the add that saved an empty fingerprint.
    #[test]
    fn an_add_naming_no_usable_key_row_is_refused_and_saves_nothing() {

        let started = a_started("no-usable-key");
        let connection = started.record.as_ref().expect("a record").connection();
        let unread = a_key_row(None);
        key_rows::add(connection, &unread).expect("a key row");

        for identifier in ["no-such-key".to_string(), unread.identifier.as_text(), String::new()] {
            let answer = the_add_is_performed(&started, &an_add(&identifier));
            assert_eq!(
                answer,
                TheAnswer::TheseFieldsAreNeeded(vec![("key", words::A_KEY_IS_NEEDED)]),
                "{identifier:?}"
            );
        }
        assert!(the_rows_of(&started).is_empty(), "a server was saved with no usable key");

        let good = a_key_row(Some("SHA256:good"));
        key_rows::add(connection, &good).expect("a key row");
        assert_eq!(
            the_add_is_performed(&started, &an_add(&good.identifier.as_text())),
            TheAnswer::ItWasSaved
        );
        let saved = the_rows_of(&started);
        assert_eq!(saved[0].key.fingerprint, "SHA256:good");
    }

    /// **Fix-here item 13: the ticked options are the remembered setup, written
    /// with the row.** It fails on the add that threw both ticks away. *(It also
    /// said the user is told nothing started, which step 10c piece 9 made
    /// false; step 10c's eleventh-pass fix piece 6 took the sentence away.)*
    #[test]
    fn an_add_with_a_setup_option_ticked_remembers_which() {

        let started = a_started("ticked");
        let connection = started.record.as_ref().expect("a record").connection();
        let key = a_key_row(Some("SHA256:good"));
        key_rows::add(connection, &key).expect("a key row");
        let identifier = key.identifier.as_text();
        let add = TheAdd {
            install_blender: true,
            ..an_add(&identifier)
        };
        assert_eq!(the_add_is_performed(&started, &add), TheAnswer::ItWasSavedAndNothingStarted);
        let saved = the_rows_of(&started);
        assert_eq!(
            saved[0].remembered_setup.as_ref().map(|setup| setup.options.clone()),
            Some(vec![INSTALL_THE_LATEST_STABLE_BLENDER.to_string()])
        );
        // **Nothing is said**: the add connects and sets up (`Servers` O2), so
        // *Not available yet* would be false. Step 10c's eleventh-pass
        // fix piece 6.
        let told = the_answer_event(TheAnswer::ItWasSavedAndNothingStarted);
        assert!(!the_said_of(&told).iter().any(|name| name == "said"));

        let plain = the_add_is_performed(&started, &an_add(&identifier));
        assert_eq!(plain, TheAnswer::ItWasSaved);
        assert!(the_rows_of(&started)[1].remembered_setup.is_none());
    }

    /// A valid combined port/key edit needs no paid state.
    #[test]
    fn an_edit_of_the_port_and_the_key_needs_no_paid_state() {

        let started = a_started("port-and-key");
        let connection = started.record.as_ref().expect("a record").connection();
        let first = a_key_row(Some("SHA256:first"));
        let second = a_key_row(Some("SHA256:second"));
        key_rows::add(connection, &first).expect("a key row");
        key_rows::add(connection, &second).expect("a key row");
        the_add_is_performed(&started, &an_add(&first.identifier.as_text()));
        let row = the_rows_of(&started).remove(0);

        let answer = the_edit_is_performed(
            &started,
            None,
            &row.local_server_identifier,
            "213.181.122.44",
            "2222",
            "root",
            &second.identifier.as_text(),
            "", None,
        );
        assert_eq!(answer, TheAnswer::ItWasSaved);
        let edited = the_rows_of(&started).remove(0);
        assert_eq!(edited.port.number(), 2222);
        assert_eq!(edited.key.local_key_identifier, second.identifier.as_text());

        // **The rest of the edit stays open.**
        let answer = the_edit_is_performed(
            &started,
            None,
            &row.local_server_identifier,
            "213.181.122.44",
            "2222",
            "root",
            &first.identifier.as_text(),
            "", None,
        );
        assert_eq!(answer, TheAnswer::ItWasSaved);
        assert_eq!(the_rows_of(&started)[0].port.number(), 2222);
    }

    /// **`Servers` D3 and E3's fallback folder in the Edit dialog** (`Job
    /// files and outputs` P23, the build runner's rulings of 2026-10-04): a
    /// path that does not start with `/` is refused in place and never saved;
    /// a full path is saved alone, trimmed, ending no period; emptying the
    /// field clears it; and a host edit keeps it. The dialog shows the field
    /// with the Design's own words. The Creator's second notes, piece 4.
    #[test]
    fn a_fallback_folder_is_saved_alone_kept_by_a_host_edit_and_refused_where_it_is_not_a_full_path() {

        let started = a_started("fallback-folder");
        let connection = started.record.as_ref().expect("a record").connection();
        let key = a_key_row(Some("SHA256:first"));
        key_rows::add(connection, &key).expect("a key row");
        the_add_is_performed(&started, &an_add(&key.identifier.as_text()));
        let row = the_rows_of(&started).remove(0);
        assert_eq!(row.chosen_fallback_folder, None, "Add has no such field");
        let edit = |fallback: Option<&str>, host: &str| {
            the_edit_is_performed(
                &started,
                None,
                &row.local_server_identifier,
                host,
                &row.port.number().to_string(),
                row.username.as_str(),
                &key.identifier.as_text(),
                "",
                fallback,
            )
        };

        assert_eq!(edit(Some("renders"), row.host.as_str()), TheAnswer::TheFallbackFolderWillNotDo);
        assert_eq!(the_rows_of(&started)[0], row, "a refused folder wrote nothing");

        // **Saved alone, and no period would end for it** (`Servers` E3):
        // what the save ends a period on is the four connection details, and
        // the folder is none of them.
        assert_eq!(edit(Some("  /mnt/renders  "), row.host.as_str()), TheAnswer::ItWasSaved);
        let saved = the_rows_of(&started).remove(0);
        assert_eq!(saved.chosen_fallback_folder.as_deref(), Some("/mnt/renders"));
        assert_eq!(saved.host, row.host);
        assert_eq!(smoozy_servers::edits::ends_the_period(WhatChanged::TheFallbackFolder), None);

        // **A host edit keeps it**, and a save that carries no folder (the
        // rename on the row) leaves it as it is.
        assert_eq!(edit(None, "10.0.0.9"), TheAnswer::ItWasSaved);
        assert_eq!(the_rows_of(&started)[0].chosen_fallback_folder.as_deref(), Some("/mnt/renders"));

        // **The dialog draws it**, with the Design's own words.
        let dialog = the_edit_dialog_from(&the_rows_of(&started)[0], &the_rows_of(&started), &[], None);
        assert_eq!(dialog.the_fallback_folder, Verbatim::new("/mnt/renders"));
        let design = include_str!("../../../../Preparation/Design/Servers.md").replace('’', "'");
        for said in [words::FALLBACK_FOLDER, words::SMOOZYS_FOLDER, words::WHERE_JOBS_GO, words::ENTER_A_FULL_PATH] {
            assert!(design.contains(&said.replace('’', "'")), "not the Design's words: {said}");
        }

        // **Emptying it clears it**: Smoozy's own folder again.
        assert_eq!(edit(Some("   "), "10.0.0.9"), TheAnswer::ItWasSaved);
        assert_eq!(the_rows_of(&started)[0].chosen_fallback_folder, None);
    }

    /// **Fix-here items 3 and 11: an edit of the host and the username together
    /// takes both rows of `Servers` D12's table** — the machine-derived values go, and the
    /// remembered password is erased through the store. It fails on the edit
    /// that filed the change under the host alone and kept the password.
    #[test]
    fn an_edit_of_the_host_and_the_username_erases_the_password() {

        let started = a_started("host-and-login");
        let connection = started.record.as_ref().expect("a record").connection();
        let key = a_key_row(Some("SHA256:first"));
        key_rows::add(connection, &key).expect("a key row");
        the_add_is_performed(&started, &an_add(&key.identifier.as_text()));
        let mut row = the_rows_of(&started).remove(0);
        row.machine_password_subject =
            Some(Subject::issued_by_the_local_record(&row.local_server_identifier));
        row.remembered_hardware_reading = Some(a_reading());
        server_rows::write(connection, &row).expect("written");

        let store = AStandInStore::answering(Ok(()));
        let answer = the_edit_is_performed(
            &started,
            Some(&store),
            &row.local_server_identifier,
            "10.0.0.9",
            "40122",
            "artist",
            &key.identifier.as_text(),
            "", None,
        );
        assert_eq!(answer, TheAnswer::ItWasSaved);
        assert_eq!(store.erased(), 1, "the remembered password was not erased");
        let after = the_rows_of(&started).remove(0);
        assert!(after.machine_password_subject.is_none());
        assert!(after.remembered_hardware_reading.is_none());
        assert_eq!(after.username.as_str(), "artist");
    }

    /// **Fix-here item 11: a removal erases the remembered password first, and
    /// a store that did not confirm leaves the owed-erase mark and never blocks
    /// the removal.** It fails on the removal that never reached the store.
    #[test]
    fn a_removal_erases_the_remembered_password_and_is_never_blocked() {

        let started = a_started("removal-erases");
        let connection = started.record.as_ref().expect("a record").connection();
        let key = a_key_row(Some("SHA256:first"));
        key_rows::add(connection, &key).expect("a key row");
        the_add_is_performed(&started, &an_add(&key.identifier.as_text()));
        let mut row = the_rows_of(&started).remove(0);
        row.machine_password_subject =
            Some(Subject::issued_by_the_local_record(&row.local_server_identifier));
        server_rows::write(connection, &row).expect("written");

        let store = AStandInStore::answering(Err(StoreFailure::LockedOrRefused));
        let answer = the_removal_is_performed(
            &started,
            Some(&store),
            &row.local_server_identifier,
            false,
        );
        assert_eq!(answer, TheAnswer::ItWasSaved);
        assert_eq!(store.erased(), 1);
        assert!(the_rows_of(&started).is_empty());
        use smoozy_security_and_licence::erasure::EraseMarks;
        assert_eq!(
            server_rows::ServerRowMarks(connection).marks().expect("read").len(),
            1,
            "an unconfirmed erase left no debt for the sweep"
        );
    }

    /// **Fix-here items 12 and 19: a server another act removed is said as
    /// that**, and a refused write is not *Not available yet*.
    #[test]
    fn a_server_that_is_gone_and_a_refused_write_each_say_their_own_thing() {
        let started = a_started("gone");
        assert_eq!(
            the_edit_is_performed(&started, None, "no-such-server", "h", "22", "u", "k", "", None),
            TheAnswer::TheServerIsGone
        );
        assert_eq!(
            the_password_is_forgotten(&started, None, "no-such-server"),
            TheAnswer::TheServerIsGone
        );
        let said = |answer: TheAnswer| {
            the_pairs_of(&the_answer_event(answer))
                .into_iter()
                .find(|(name, _)| name == "said")
                .map(|(_, words)| words)
                .expect("something is said")
        };
        let refused = said(TheAnswer::TheRecordWouldNotTakeIt);
        let gone = said(TheAnswer::TheServerIsGone);
        let not_built = said(TheAnswer::TheActIsNotBuiltYet);
        assert_ne!(refused, not_built, "a refused write says the app is not finished");
        assert_ne!(gone, not_built);
        assert_ne!(refused, gone);
    }

    /// **`Servers` Q13 outcome 2 reaches the user as its own sentence**, and
    /// outcome 3's is the claimless one beside it.
    ///
    /// **Both halves are asserted here rather than only the first**, because a
    /// sentence that named the machine for every failed connection would pass a
    /// test that only looked at outcome 2 — and would be Smoozy claiming what
    /// it does not know on eight of the nine reasons.
    #[test]
    fn a_machine_that_is_not_linux_is_said_to_be_one_and_nothing_else_is() {
        let said = |answer: TheAnswer| {
            the_pairs_of(&the_answer_event(answer))
                .into_iter()
                .find(|(name, _)| name == "said")
                .map(|(_, words)| words)
                .expect("something is said")
        };
        let not_linux = said(TheAnswer::ItIsNotALinuxMachine);
        let not_made = said(TheAnswer::TheConnectionWasNotMade);
        assert_ne!(
            not_linux, not_made,
            "**`Servers` Q13 outcome 2 and outcome 3 are two sentences.** The \
             clause exists to keep them apart."
        );
        // **Read back through the typography point**, which is the one place
        // every string of prose passes through and which puts its own spaces in.
        assert_eq!(
            not_linux,
            Prose::new(words::IT_IS_NOT_A_LINUX_MACHINE).as_str()
        );
        assert_eq!(
            not_made,
            Prose::new(words::THE_CONNECTION_WAS_NOT_MADE).as_str()
        );
    }

    /// **L1 (2026-09-18): where no passphrase could be handed to the SSH tools
    /// and Smoozy knows why, the channel's own sentence is what is said** — and
    /// it is not the claimless one.
    #[test]
    fn a_passphrase_that_could_not_be_used_is_said_with_its_reason() {
        let pairs = the_pairs_of(&the_answer_event(TheAnswer::TheKeysPassphraseCouldNotBeUsed(
            crate::keys::askpass::THE_PATH_IS_TOO_LONG,
        )));
        let said = pairs
            .iter()
            .find(|(name, _)| name == "said")
            .map(|(_, words)| words.clone())
            .expect("something is said");
        assert_eq!(
            said,
            Prose::new(crate::keys::askpass::THE_PATH_IS_TOO_LONG).as_str()
        );
        assert_ne!(said, Prose::new(words::THE_CONNECTION_WAS_NOT_MADE).as_str());
    }

    /// **The Creator's note 14 on the test copy: a connect that did not
    /// happen names its server**, so the page can say it where the user is —
    /// the line above the list names the server (*Could not connect to
    /// Hyperstack.*), and the Set up card keeps *this server*. **The two
    /// sentences with a cause of their own name the server and keep their own
    /// words**; every other answer goes out as it did. **Before the piece the
    /// answer carried no server at all.**
    #[test]
    fn a_connect_that_did_not_happen_names_its_server() {
        let about = TheServerItWasAbout {
            server: "server-1".to_string(),
            name: Some("Hyperstack".to_string()),
        };
        let said = |answer: TheAnswer, about: &TheServerItWasAbout| {
            the_pairs_of(&the_answer_about(answer, about))
                .into_iter()
                .collect::<std::collections::BTreeMap<_, _>>()
        };

        let not_made = said(TheAnswer::TheConnectionWasNotMade, &about);
        assert_eq!(not_made["what"], THE_CONNECTION_WAS_NOT_MADE_ANSWER);
        assert_eq!(not_made["server"], "server-1");
        assert_eq!(not_made["couldNotConnectTo"], Prose::new(words::COULD_NOT_CONNECT_TO).as_str());
        assert_eq!(not_made["couldNotConnectToNamed"], "Hyperstack");
        assert_eq!(not_made["said"], Prose::new(words::THE_CONNECTION_WAS_NOT_MADE).as_str());

        for (answer, what, words_said) in [
            (
                TheAnswer::ItIsNotALinuxMachine,
                IT_IS_NOT_A_LINUX_MACHINE_ANSWER,
                words::IT_IS_NOT_A_LINUX_MACHINE,
            ),
            (
                TheAnswer::TheKeysPassphraseCouldNotBeUsed(THE_PASSPHRASE_SENTENCE_ON_THIS_SYSTEM),
                THE_PASSPHRASE_COULD_NOT_BE_USED_ANSWER,
                THE_PASSPHRASE_SENTENCE_ON_THIS_SYSTEM,
            ),
        ] {
            let one = said(answer, &about);
            assert_eq!(one["what"], what);
            assert_eq!(one["server"], "server-1", "{what}");
            assert_eq!(one["said"], Prose::new(words_said).as_str(), "{what}");
            assert!(!one.contains_key("couldNotConnectTo"), "{what} keeps its own sentence");
            // **The Set up card, handed the `what` back, says the same.**
            assert_eq!(what_a_failed_connect_said(what), Some(words_said), "{what}");
        }

        // **A row that could not be read names no server**, and the line falls
        // back to the sentence as said rather than inventing a name.
        let unnamed = said(
            TheAnswer::TheConnectionWasNotMade,
            &TheServerItWasAbout { server: "server-1".to_string(), name: None },
        );
        assert_eq!(unnamed["server"], "server-1");
        assert!(!unnamed.contains_key("couldNotConnectTo"));

        // **Every other answer is about an act, not a server's connection.**
        for answer in [TheAnswer::TheRecordWouldNotTakeIt, TheAnswer::ItWasSaved, TheAnswer::TheServerIsGone] {
            let other = said(answer.clone(), &about);
            assert!(!other.contains_key("server"), "{answer:?}");
            assert_eq!(the_pairs_of(&the_answer_about(answer.clone(), &about)), the_pairs_of(&the_answer_event(answer)));
        }

        // **Nothing outside the three puts a word on the card.**
        for what in ["the-record-would-not-take-it", "", "Could not connect to this server."] {
            assert_eq!(what_a_failed_connect_said(what), None, "{what:?}");
        }

        // **The period's end and every claimless not-made name the server**:
        // the three places a connect that did not happen is told from.
        let working = the_working_half();
        for (at, call) in [
            ("pub(crate) fn the_period_ended<", "the_answer_about(answer, &the_server_as_the_list_names_it(app, server))"),
            ("pub(crate) fn the_connection_was_not_made<", "&the_server_as_the_list_names_it(app, server)"),
        ] {
            let opens = working.find(at).expect(at);
            let body = &working[opens..opens + working[opens..].find("\n}\n").expect("it closes")];
            assert!(body.contains(call), "{at} does not name its server");
            assert!(!body.contains("the_answer_event("), "{at} tells an answer that names nothing");
        }
    }

    /// **What an ended period says**, the choice kept apart from the
    /// application: `Keys` S19's box is the answer and nothing is said beside
    /// it; a passphrase that could not be handed over says why; `Servers` Q13
    /// outcome 2 says its own sentence; and the claimless sentence is the rest.
    #[test]
    fn what_an_ended_period_says_and_where_it_says_nothing() {
        use WhatTheDrivingLeft as Left;
        assert_eq!(what_an_ended_period_says(Left::TheConnectionWasNotMade, true, None), None);
        assert_eq!(
            what_an_ended_period_says(Left::TheConnectionWasNotMade, true, Some(crate::keys::askpass::THE_PATH_IS_TOO_LONG)),
            None,
            "the box raised is the answer"
        );
        assert_eq!(
            what_an_ended_period_says(Left::TheConnectionWasNotMade, false, None),
            Some(TheAnswer::TheConnectionWasNotMade)
        );
        assert_eq!(
            what_an_ended_period_says(Left::TheConnectionWasNotMade, false, Some(crate::keys::askpass::THE_PATH_IS_TOO_LONG)),
            Some(TheAnswer::TheKeysPassphraseCouldNotBeUsed(crate::keys::askpass::THE_PATH_IS_TOO_LONG))
        );
        assert_eq!(
            what_an_ended_period_says(Left::ItIsNotALinuxMachine, false, None),
            Some(TheAnswer::ItIsNotALinuxMachine)
        );
        for quiet in [Left::TheWarningIsTheAnswer, Left::TheRepairStoppedForGood, Left::NothingToSay] {
            assert_eq!(what_an_ended_period_says(quiet, false, None), None, "{quiet:?}");
        }
    }

    /// **The list's page half, run under Node: a connect that did not happen
    /// is kept for its server and drawn above the list, naming it — while the
    /// list is hidden and when the user arrives at it — the newest for each
    /// server, through any other answer, until the next act on that server.**
    /// The connect on that row takes its line away and leaves the other
    /// server's; the other server's goes when it is connected, however that
    /// connection was made. **Before the piece** the sentence went into the one notice
    /// line, named no server, and went at the next answer of any kind.
    #[test]
    fn the_list_names_a_connect_that_did_not_happen_until_the_next_act_on_that_server() {
        let as_said = |event: &Event| {
            serde_json::Value::Object(
                the_pairs_of(event)
                    .into_iter()
                    .map(|(name, said)| (name, serde_json::Value::String(said)))
                    .collect(),
            )
        };
        let hyperstack = TheServerItWasAbout {
            server: "server-1".to_string(),
            name: Some("Hyperstack".to_string()),
        };
        let vast = TheServerItWasAbout {
            server: "server-2".to_string(),
            name: Some("Vast 4090".to_string()),
        };
        let harness = THE_CONNECTS_NOT_MADE_HARNESS
            .replace("NOT_MADE_ON_ONE", &as_said(&the_answer_about(TheAnswer::TheConnectionWasNotMade, &hyperstack)).to_string())
            .replace("NOT_LINUX_ON_TWO", &as_said(&the_answer_about(TheAnswer::ItIsNotALinuxMachine, &vast)).to_string())
            .replace("ANOTHER_ANSWER", &as_said(&the_answer_event(TheAnswer::TheRecordWouldNotTakeIt)).to_string());
        let folder = std::env::temp_dir().join(format!("smoozy-servers-not-made-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/servers.js");
        std::fs::copy(&page, folder.join("servers.js")).expect("the page is copied");
        for (name, text) in THE_STAND_INS {
            let text = if name == "harness.js" { harness.clone() } else { text.to_string() };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let _ = std::fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the server list's page was not run");
            return;
        };
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let named = format!("{} Hyperstack.", Prose::new(words::COULD_NOT_CONNECT_TO).as_str());
        let not_linux = Prose::new(words::IT_IS_NOT_A_LINUX_MACHINE).as_str().to_string();
        assert_eq!(out["whileAway"], serde_json::json!([named]), "{out}");
        assert_eq!(out["onArriving"], serde_json::json!([named]), "{out}");
        assert_eq!(out["two"], serde_json::json!([named, not_linux]), "{out}");
        assert_eq!(out["afterAnotherAnswer"], serde_json::json!([named, not_linux]), "{out}");
        assert_eq!(out["kept"], THE_CONNECTION_WAS_NOT_MADE_ANSWER, "{out}");
        assert_eq!(out["notice"], 0, "a connect that did not happen is not the one notice line: {out}");
        assert_eq!(out["afterTheConnect"], serde_json::json!([not_linux]), "{out}");
        assert_eq!(out["acts"], serde_json::json!([["connect-and-set-up", "server-1"]]), "{out}");
        assert_eq!(out["whileConnecting"], serde_json::json!([not_linux]), "{out}");
        assert_eq!(
            out["afterConnected"],
            serde_json::json!([]),
            "a server connected however it was made has nothing standing: {out}"
        );
    }

    /// **The harness for the connects that did not happen**: two rows, the
    /// list hidden while the answers arrive, then shown, then the first row's
    /// connect pressed.
    const THE_CONNECTS_NOT_MADE_HARNESS: &str = r#"function anElement(tag) {
  const names = new Set();
  const listeners = new Map();
  const element = {
    tag, children: [], parentNode: null, dataset: {}, hidden: false, text: "",
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
    get textContent() { return element.text; },
    get childNodes() { return element.children; },
    get childElementCount() { return element.children.length; },
    appendChild(child) { child.parentNode = element; element.children.push(child); return child; },
    insertBefore(child) { return element.appendChild(child); },
    removeChild(child) { element.children = element.children.filter((one) => one !== child); return child; },
    replaceChild(fresh, old) { element.children = element.children.map((one) => (one === old ? fresh : one)); fresh.parentNode = element; return old; },
    replaceChildren(...fresh) { element.children = []; fresh.forEach((one) => element.appendChild(one)); },
    setAttribute() {}, removeAttribute() {}, getAttribute() { return null; }, focus() {},
    addEventListener(kind, heard) { listeners.set(kind, heard); },
    press() { const heard = listeners.get("click"); if (heard) heard({ stopPropagation() {} }); },
    querySelector() { return null; }, querySelectorAll() { return []; }, closest() { return null; },
    contains() { return false; },
    getBoundingClientRect() { return { left: 0, bottom: 0, width: 0, height: 0 }; },
  };
  return element;
}
function all(element, found) { found.push(element); for (const child of element.children || []) all(child, found); return found; }
function text(element) { return (element.text || "") + (element.children || []).map(text).join(""); }
globalThis.told = new Map();
globalThis.document = { createElement: anElement, createTextNode: (text) => ({ text, children: [] }), addEventListener() {}, removeEventListener() {} };
globalThis.window = { innerWidth: 1000, innerHeight: 800 };
const words = (lead) => ({ lead, values: [], rest: null, the_rest_joins: false });
const aRow = (server, name) => ({
  server, name, address: "", badge: null, the_key_mark: null, the_setup_line: null, the_work: [],
  the_hardware: { state: "never-read", nothing_to_show: words("") },
  the_connection: { label: "", act: "connect-and-set-up", unavailable: false },
  the_menu: "", more: "",
});
const theScreen = {
  the_screens_own_name: "", add_a_server: "", the_tools_statement: null,
  rows: [aRow("server-1", "Hyperstack"), aRow("server-2", "Vast 4090")],
  footer: { how_many: words(""), how_many_connected: words("") },
  guidance: { heading: "", how_it_goes: "", steps: [], open_keys: "", add_a_server: "" },
  menus: [], the_subject_kind: "server row",
};
const acts = [];
window.__TAURI__ = { core: { invoke(command, args) {
  if (command === "the_all_servers_screen") return Promise.resolve(theScreen);
  if (command === "an_act_that_needs_a_connection") acts.push([args.act, args.server]);
  return Promise.resolve(null);
} } };
const settle = async () => { for (let at = 0; at < 4; at += 1) await new Promise((done) => setImmediate(done)); };
const tell = (said) => (globalThis.told.get("servers:the-answer") || []).forEach((one) => one({ kind: "servers:the-answer", said }));
const servers = await import("./servers.js");
const host = anElement("div");
globalThis.screen.itIsDrawn(host);
await settle();
const lines = () => all(host, []).filter((one) => one.classList && one.classList.contains("svnotmade")).map(text);
const out = {};
host.hidden = true;
tell(NOT_MADE_ON_ONE);
await settle();
out.whileAway = lines();
host.hidden = false;
globalThis.screen.itIsCurrentNow();
await settle();
out.onArriving = lines();
tell(NOT_LINUX_ON_TWO);
await settle();
out.two = lines();
tell(ANOTHER_ANSWER);
await settle();
out.afterAnotherAnswer = lines();
out.notice = all(host, []).filter((one) => one.classList && one.classList.contains("svnotyet") && text(one).includes("connect")).length;
const kept = servers.theConnectionFailureOn("server-1");
out.kept = kept ? kept.what : null;
const row = all(host, []).find((one) => one.classList && one.classList.contains("svrow") && one.dataset.server === "server-1");
const connect = all(row, []).find((one) => one.classList && one.classList.contains("cbtn"));
connect.press();
await settle();
out.afterTheConnect = lines();
out.acts = acts;
// server-2 is connected by a way this page does not see asked (the
// passphrase box's own Connect): first its connect is under way, then live.
const observed = () => (globalThis.told.get("servers:what-is-observed") || []).forEach((one) => one({ kind: "servers:what-is-observed", subject: "server-2", said: {} }));
theScreen.rows[1].the_connection = { label: "", act: "disconnect", unavailable: false };
theScreen.rows[1].the_connect_is_underway = true;
observed();
await settle();
out.whileConnecting = lines();
theScreen.rows[1].the_connect_is_underway = false;
observed();
await settle();
out.afterConnected = lines();
console.log(JSON.stringify(out));"#;

    /// **Fix-here item 14: the mark is `Keys`' own verdict read with the pair**
    /// — a usable key with the same fingerprint marks nothing, an unusable one
    /// marks the server, and where `Keys` was not asked nothing is marked. It
    /// fails on the mark that read the pair alone.
    #[test]
    fn the_key_mark_follows_keys_own_verdict() {
        let key = a_key_row(Some("SHA256:aaaa"));
        let mut row = a_row();
        row.key.local_key_identifier = key.identifier.as_text();
        let keys = vec![key.clone()];
        let said = |verdict: usable::Verdict| WhatKeysSaid {
            the_tools_are_installed: true,
            verdicts: vec![(key.identifier.as_text(), verdict)],
            ..WhatKeysSaid::default()
        };
        assert!(!the_key_mark(&row, &keys, Some(&said(usable::Verdict::Usable))));
        assert!(the_key_mark(
            &row,
            &keys,
            Some(&said(usable::Verdict::Unusable(usable::Unusable::FileMissing)))
        ));
        assert!(!the_key_mark(&row, &keys, None), "a mark derived from nothing was drawn");
        let no_tools = WhatKeysSaid {
            the_tools_are_installed: false,
            verdicts: Vec::new(),
            ..WhatKeysSaid::default()
        };
        assert!(!the_key_mark(&row, &keys, Some(&no_tools)));
        let mut replaced = row.clone();
        replaced.key.fingerprint = "SHA256:bbbb".to_string();
        assert!(the_key_mark(&replaced, &keys, Some(&said(usable::Verdict::Usable))));
    }

    /// **Fix-here item 21: the port is added where it is the only thing telling
    /// two unnamed servers apart, and nowhere else.**
    #[test]
    fn the_port_is_named_only_where_it_tells_two_unnamed_servers_apart() {
        let one = a_row();
        let mut two = a_row();
        two.port = Port::what_the_user_typed(40123).expect("a port");
        let mut named = a_row();
        named.label = Some("Overnight box".to_string());
        let alone = vec![one.clone()];
        assert_eq!(what_a_server_is_called_among(&one, &alone), "root@213.181.122.44");
        let pair = vec![one.clone(), two.clone(), named.clone()];
        assert_eq!(what_a_server_is_called_among(&one, &pair), "root@213.181.122.44:40122");
        assert_eq!(what_a_server_is_called_among(&two, &pair), "root@213.181.122.44:40123");
        assert_eq!(what_a_server_is_called_among(&named, &pair), "Overnight box");
        let screen = the_screen_from(&pair, &[], None, None, &Default::default());
        assert_ne!(screen.rows[0].name, screen.rows[1].name, "two rows read identically");
        let rail = crate::surface::the_rail::the_rail_from(&pair, None, &Default::default(), false);
        assert_ne!(rail.servers[0].name, rail.servers[1].name, "two rail rows read identically");
    }

    /// **Fix-here items 28 and 29: a cleared port is 22, and a server nothing is
    /// known about says so without naming a cause.**
    #[test]
    fn a_cleared_port_is_the_standard_one() {
        assert_eq!(the_port_typed(""), Some(Port::STANDARD_SSH));
        assert_eq!(the_port_typed("  "), Some(Port::STANDARD_SSH));
        assert_eq!(the_port_typed("0"), None);
        let drawn = the_hardware(None);
        let said = drawn.nothing_to_show.expect("something is said");
        assert!(!said.lead.as_str().contains("Never"), "a cause is named");
    }

    /// **Fix-here item 4: a read that did not answer is never an empty list.**
    /// And the page's error path draws the application's sentence where the
    /// list goes, rather than marking the host and drawing nothing.
    #[test]
    fn a_list_that_did_not_read_is_never_an_empty_list() {
        let mut started = a_started("unread");
        started.record = None;
        assert!(the_rows(&started).is_none());
        let page = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/servers.js"),
        )
        .expect("the page module is readable");
        let at = page
            .find("function theScreenIsAskedFor()")
            .expect("the page asks for the screen");
        let asking = &page[at..at + page[at..].find("\n}\n").expect("it closes")];
        assert!(asking.contains(".catch((failure) =>"));
        assert!(asking.contains("theListCouldNotBeRead(failure)"));
        assert!(page.contains("failure && failure.said"));
    }

    /// **Fix-here item 30: the SSH-tools statement is the Keys screen's own two
    /// sentences**, two pieces rather than one with two sentences inside.
    #[test]
    fn the_tools_statement_is_the_keys_screens_two_sentences() {
        let said = WhatKeysSaid {
            the_tools_are_installed: false,
            verdicts: Vec::new(),
            ..WhatKeysSaid::default()
        };
        let screen = the_screen_from(&[], &[], Some(&said), None, &Default::default());
        let statement = screen.the_tools_statement.expect("the statement");
        for piece in [&statement.lead, &statement.until] {
            assert_eq!(
                piece.as_str().matches(". ").count() + piece.as_str().matches(".\u{a0}").count(),
                0,
                "two sentences travel as one piece: {:?}",
                piece.as_str()
            );
        }
        assert!(the_screen_from(&[], &[], None, None, &Default::default()).the_tools_statement.is_none());
    }

    /// **Fix-here items 8, 27 and 28, on the page**: the dialogs are dialogs
    /// and close on Escape; Enter and the delete key act only where the row
    /// itself has the keyboard; a cleared port puts 22 back.
    #[test]
    fn the_page_keeps_the_dialog_the_row_keys_and_the_port() {
        let page = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/servers.js"),
        )
        .expect("the page module is readable");
        assert!(page.contains("box.setAttribute(\"role\", \"dialog\");"));
        assert!(page.contains("press.key !== \"Escape\""));
        assert!(page.contains("if (!row || press.target !== row) return;"));
        assert!(page.contains("if (!port.value.trim()) port.value = standard;"));
        assert!(page.contains("aServerDialogPicksAKey(context()"), "the dialog never picks a key");
        assert!(page.contains("getReady: ready.on()"));
        assert!(page.contains("installBlender: blender.on()"));

    }

    /// **Fix-here item 1, and it runs the page code rather than reading it.**
    ///
    /// `web/servers.js` is loaded under Node beside stand-ins for the five
    /// modules it imports and the least of a document it needs. The screen is
    /// drawn once; **two standing failures then arrive for its one row**. The
    /// page must have asked for them exactly once, and must draw both on the
    /// row. **Before the fix, each arriving failure redrew the list and asked
    /// again** — the first auditor measured 237 asks in 300 milliseconds — and
    /// only the last failure was kept. **Skipped, and says so, where no Node is
    /// installed.**
    #[test]
    fn the_page_asks_for_the_failures_once_and_draws_every_one() {
        let folder = std::env::temp_dir().join(format!(
            "smoozy-servers-page-harness-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/servers.js");
        std::fs::copy(&page, folder.join("servers.js")).expect("the page is copied");
        for (name, text) in THE_STAND_INS {
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node")
            .arg(folder.join("harness.js"))
            .current_dir(&folder)
            .output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the server list's page was not run");
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
            r#"{"asksAfterTheDraw":1,"asksAfterTwoFailures":1,"failuresOnTheRow":2,"failuresAfterOneCleared":1}"#
        );
    }

    /// **The page draws each of V13's lines, as the application composed it,
    /// and gives it the row's one extra line ahead of the key mark.** Runs
    /// `web/servers.js` under Node beside the same stand-ins, with rows the
    /// application composed; skipped, and says so, where no Node is installed.
    /// Step 10c's transfer wave piece 9.
    #[test]
    fn the_page_draws_each_update_line_and_update_now_only_where_it_waits() {
        use crate::servers::published::WhatSmoozyOnTheServerIs as Carries;
        let (row, register) = a_connected_server();
        let drawn = the_screen_from(std::slice::from_ref(&row), &[], None, Some(&register), &Default::default());
        let connected = register.what_is_true_of(&row.local_server_identifier);
        let mut rows = Vec::new();
        for (at, carries) in [
            Carries::TheUpdateWaits,
            Carries::TheUpdateIsUnderWay,
            Carries::TheUpdateFailed { said: "rolled back".into() },
            Carries::TooOldAndItsWorkCannotBeControlledFromHere,
            Carries::TooNew,
        ]
        .into_iter()
        .enumerate()
        {
            let mut one = drawn.rows[0].clone();
            one.server = Verbatim::new(format!("row{at}"));
            one.the_update_line = the_update_line(Some(&carries), connected, "RunPod 4090", None);
            rows.push(one);
        }
        // **Too old with jobs known to stand**, the line no reading picks yet.
        let mut known = drawn.rows[0].clone();
        known.server = Verbatim::new("row5");
        let (which, said) = the_too_old_line(true);
        known.the_update_line = Some(TheUpdateLine {
            which: Verbatim::new(which),
            said,
            a_failure: false,
            update_now: None,
            the_box: None,
            the_servers_words: None,
        });
        rows.push(known);
        // **The update line takes the extra line from the key mark.**
        let mut both = rows[0].clone();
        both.server = Verbatim::new("row6");
        both.the_key_mark = Some(Words::plain(words::THE_KEY_IS_GONE));
        rows.push(both);
        let rows = serde_json::to_string(&rows).expect("the rows serialise");

        let folder = std::env::temp_dir().join(format!(
            "smoozy-servers-update-lines-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/servers.js");
        std::fs::copy(&page, folder.join("servers.js")).expect("the page is copied");
        for (name, text) in THE_STAND_INS {
            let text = if name == "harness.js" {
                THE_UPDATE_LINES_HARNESS.replace("THE_ROWS", &rows)
            } else {
                text.to_string()
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node")
            .arg(folder.join("harness.js"))
            .current_dir(&folder)
            .output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the server list's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: Vec<(String, String, bool, bool)> =
            serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let line = |at: usize| &out[at];
        let reads = |at: usize, words: &str| line(at).1.contains(Prose::new(words).as_str());
        assert_eq!(line(0).0, "waits");
        assert!(reads(0, words::THE_UPDATE_WAITS) && line(0).2, "the waiting line and its Update now…");
        assert!(reads(0, words::UPDATE_NOW));
        assert_eq!((line(1).0.as_str(), line(1).2), ("under-way", false));
        assert!(reads(1, words::UPDATING));
        assert_eq!((line(2).0.as_str(), line(2).2, line(2).3), ("failed", false, true), "the failed line is the failure shape");
        assert_eq!((line(3).0.as_str(), line(3).2), ("too-old-unknown", false));
        assert!(reads(3, words::TOO_OLD_CANNOT_TELL));
        assert_eq!((line(4).0.as_str(), line(4).2), ("too-new", false));
        assert!(reads(4, words::TOO_NEW));
        assert_eq!((line(5).0.as_str(), line(5).2), ("too-old", false));
        assert!(reads(5, words::TOO_OLD_ITS_JOBS));
        assert_eq!(line(6).0, "waits", "the key mark took the line");
        assert!(!reads(6, words::THE_KEY_IS_GONE));
        for (which, text, _, _) in &out {
            assert!(!text.contains("isn't responding"), "{which}");
        }
    }

    /// **The harness for the update lines**: the page draws the rows handed in,
    /// and each row's extra line is read back — which line, its words, whether
    /// *Update now…* is on it, and whether it is the failure shape.
    const THE_UPDATE_LINES_HARNESS: &str = r#"function anElement(tag) {
  const names = new Set();
  const element = {
    tag, children: [], parentNode: null, dataset: {}, hidden: false, text: "",
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
    get textContent() { return element.text; },
    get childNodes() { return element.children; },
    get childElementCount() { return element.children.length; },
    appendChild(child) { child.parentNode = element; element.children.push(child); return child; },
    insertBefore(child) { return element.appendChild(child); },
    removeChild(child) { element.children = element.children.filter((one) => one !== child); return child; },
    replaceChild(fresh, old) {
      element.children = element.children.map((one) => (one === old ? fresh : one));
      fresh.parentNode = element;
      return old;
    },
    replaceChildren(...fresh) { element.children = []; fresh.forEach((one) => element.appendChild(one)); },
    setAttribute() {}, removeAttribute() {}, addEventListener() {}, focus() {},
    querySelector() { return null; }, querySelectorAll() { return []; }, closest() { return null; },
    contains() { return false; },
    getBoundingClientRect() { return { left: 0, bottom: 0, width: 0, height: 0 }; },
  };
  return element;
}
function all(element, found) {
  found.push(element);
  for (const child of element.children || []) all(child, found);
  return found;
}
function text(element) {
  return (element.text || "") + (element.children || []).map(text).join("");
}
globalThis.told = new Map();
globalThis.document = {
  createElement: anElement,
  createTextNode: (text) => ({ text, children: [] }),
  addEventListener() {}, removeEventListener() {},
};
globalThis.window = { innerWidth: 1000, innerHeight: 800 };
const words = (lead) => ({ lead, values: [], rest: null, the_rest_joins: false });
const theScreen = {
  the_screens_own_name: "", add_a_server: "", the_tools_statement: null,
  rows: THE_ROWS,
  footer: { how_many: words(""), how_many_connected: words("") },
  guidance: { heading: "", how_it_goes: "", steps: [], open_keys: "", add_a_server: "" },
  menus: [], the_subject_kind: "server row",
};
window.__TAURI__ = {
  core: {
    invoke(command) {
      if (command === "the_all_servers_screen") return Promise.resolve(theScreen);
      return Promise.resolve(null);
    },
  },
};
const settle = () => new Promise((done) => setImmediate(done));
await import("./servers.js");
const host = anElement("div");
globalThis.screen.itIsDrawn(host);
await settle();
const rows = all(host, []).filter((one) => one.classList && one.classList.contains("svrow"));
console.log(JSON.stringify(rows.map((row) => {
  const extra = all(row, []).find((one) => one.dataset && one.dataset.update);
  if (!extra) return ["", "", false, false];
  const found = all(extra, []);
  return [
    extra.dataset.update,
    text(extra),
    found.some((one) => one.classList && one.classList.contains("lnkb")),
    found.some((one) => one.classList && one.classList.contains("afailure")),
  ];
})));"#;

    /// The stand-ins the harness above loads `web/servers.js` beside.
    const THE_STAND_INS: [(&str, &str); 10] = [
        // The real drawing of a value, which the page imports (step 10c,
        // dependency wave piece 10a).
        ("the-value-drawn.js", include_str!("../../web/the-value-drawn.js")),
        ("package.json", r#"{ "type": "module" }"#),
        (
            "the-machines-own-words.js",
            r#"export function theWordsPanel() { return { hidden: true }; }"#,
        ),
        (
            "event-route.js",
            r#"export function whenTold(kind, handler) {
  const held = globalThis.told.get(kind) || [];
  held.push(handler);
  globalThis.told.set(kind, held);
}"#,
        ),
        (
            "the-rail.js",
            r#"export const ALL_SERVERS = "serversview";
export const A_SERVER = "machineview";
export const KEYS = "keysview";
export function aScreenIsRegistered(screen) { globalThis.screen = screen; }
export function theCurrentPlace() { return ALL_SERVERS; }
export function theCurrentPlaceIs() {}
export function theRailIsReadAgain() {}"#,
        ),
        (
            "the-failure-shape.js",
            r#"export const EXPANDED = "expanded";
export const A_COLLAPSED_ROW = "a-collapsed-row";
export function theControlsFrom() { return []; }
export function theFailureShape() {
  const drawn = document.createElement("span");
  drawn.classList.add("afailure");
  return drawn;
}"#,
        ),
        ("keys.js", r#"export function aServerDialogPicksAKey() {}"#),
        (
            "a-server.js",
            r#"export const SET_UP = "setupview";
export function theServerWillOpenOn(server, tab) { (globalThis.openedOn ||= []).push([server, tab]); }
export function theServersAreAskedForAgain() { globalThis.paneAskedAgain = (globalThis.paneAskedAgain || 0) + 1; }"#,
        ),
        (
            "the-underway-shape.js",
            r#"export function theCountdown() { return { nodeType: 1 }; }
export function theMarkOnAControl(control) { control.classList.add("uway"); return control; }
export function theMarkComesOffAControl(control) { control.classList.remove("uway"); return control; }"#,
        ),
        (
            "harness.js",
            r#"function anElement(tag) {
  const names = new Set();
  const element = {
    tag, children: [], parentNode: null, dataset: {}, hidden: false, text: "",
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
    get textContent() { return element.text; },
    get childNodes() { return element.children; },
    get childElementCount() { return element.children.length; },
    appendChild(child) { child.parentNode = element; element.children.push(child); return child; },
    insertBefore(child) { return element.appendChild(child); },
    removeChild(child) { element.children = element.children.filter((one) => one !== child); return child; },
    replaceChild(fresh, old) {
      element.children = element.children.map((one) => (one === old ? fresh : one));
      fresh.parentNode = element;
      return old;
    },
    replaceChildren(...fresh) { element.children = []; fresh.forEach((one) => element.appendChild(one)); },
    setAttribute() {}, removeAttribute() {}, addEventListener() {}, focus() {},
    querySelector() { return null; }, querySelectorAll() { return []; }, closest() { return null; },
    contains() { return false; },
    getBoundingClientRect() { return { left: 0, bottom: 0, width: 0, height: 0 }; },
  };
  return element;
}
function count(element, name) {
  let found = element.classList && element.classList.contains(name) ? 1 : 0;
  for (const child of element.children || []) found += count(child, name);
  return found;
}
globalThis.told = new Map();
globalThis.document = {
  createElement: anElement,
  createTextNode: (text) => ({ text, children: [] }),
  addEventListener() {}, removeEventListener() {},
};
globalThis.window = { innerWidth: 1000, innerHeight: 800 };
const words = (lead) => ({ lead, values: [], rest: null, the_rest_joins: false });
const theScreen = {
  the_screens_own_name: "", add_a_server: "", the_tools_statement: null,
  rows: [{
    server: "one", name: "", address: "", badge: null, the_key_mark: null,
    the_setup_line: null, the_work: [],
    the_hardware: { state: "never-read", nothing_to_show: words("") },
    the_connection: { label: "", act: "connect", unavailable: false },
    the_menu: "", more: "",
  }],
  footer: { how_many: words(""), how_many_connected: words("") },
  guidance: { heading: "", how_it_goes: "", steps: [], open_keys: "", add_a_server: "" },
  menus: [], the_subject_kind: "server row",
};
let asks = 0;
window.__TAURI__ = {
  core: {
    invoke(command) {
      if (command === "the_failures_standing_against") asks += 1;
      if (command === "the_all_servers_screen") return Promise.resolve(theScreen);
      return Promise.resolve(null);
    },
  },
};
const settle = () => new Promise((done) => setImmediate(done));
const tell = (kind, said) => (globalThis.told.get(kind) || []).forEach((one) => one({ kind, said }));
await import("./servers.js");
const host = anElement("div");
globalThis.screen.itIsDrawn(host);
await settle();
const asksAfterTheDraw = asks;
for (const key of ["first", "second"]) {
  tell("presentation:a-failure-is-standing", { subject: "one", subjectKind: "server row", key });
}
await settle();
const asksAfterTwoFailures = asks;
const failuresOnTheRow = count(host, "afailure");
tell("presentation:a-failure-was-cleared", { subject: "one", key: "first" });
await settle();
console.log(JSON.stringify({
  asksAfterTheDraw, asksAfterTwoFailures, failuresOnTheRow,
  failuresAfterOneCleared: count(host, "afailure"),
}));"#,
        ),
    ];

    // ── Step 10c's S2: the desktop sends stop and clear ─────────────────────

    use crate::servers::stop_and_clear::tests::{ASupervisor, Script, QUICK};
    use crate::servers::stop_and_clear::{ThePairWent, WhatThePairCameTo as Came, TELEGRAMS_ROLE_OFF};
    use smoozy_queue_history::stop_and_clear::{THE_CLEAR, THE_STOP};
    use smoozy_server_protocol::requests::{
        IT_WAS_CLEARED, IT_WAS_LEFT_STOPPED, THE_CLEAR_REQUEST, THE_TELEGRAM_ROLE_IS_DISABLED_REQUEST,
    };
    use smoozy_state::outcomes::{Keeps, UnitOutcome};

    /// **`Servers` W25's three units, scripted**: what the pair comes to is
    /// handed in, and every unit asked is written on one log.
    struct TheUnitsInHand {
        came_to: Came,
        with_a_record: bool,
        log: Vec<String>,
    }

    impl TheUnitsInHand {
        fn coming_to(came_to: Came) -> Self {
            TheUnitsInHand { came_to, with_a_record: true, log: Vec::new() }
        }
    }

    impl TheMachineReachingRemovalsEnds for TheUnitsInHand {
        fn the_pair(&mut self, _server: &str, the_units_after: &[&str]) -> ThePairWent {
            self.log.push(format!("the pair, enumerating after it: {}", the_units_after.join(", ")));
            ThePairWent {
                came_to: self.came_to,
                the_record: self
                    .with_a_record
                    .then(|| Identity::issue(IdentityKind::Operation).expect("randomness")),
            }
        }
        fn telegrams_role_off(&mut self, _server: &str) -> UnitOutcome {
            self.log.push("Telegram's role off".to_string());
            UnitOutcome::Done
        }
        fn a_unit_ended(&mut self, _record: &Identity, unit: &str, outcome: UnitOutcome) {
            self.log.push(format!("{unit}: {outcome:?}"));
        }
        fn what_the_machine_journalled_is_applied(&mut self, _server: &str) {
            self.log.push("the journal applied".to_string());
        }
        fn the_local_forget(&mut self, _server: &str) -> TheAnswer {
            self.log.push("the local forget".to_string());
            TheAnswer::ItWasSaved
        }
    }

    /// **`Servers` W25's three units over the real ends that do not need the
    /// application**: the one sender over a real §9.4 store and a stand-in
    /// supervisor answering by identity.
    struct OverARecord {
        connection: std::rc::Rc<rusqlite::Connection>,
        machine: ASupervisor,
        forgotten: Vec<String>,
    }

    impl OverARecord {
        fn answering(the_clear: Script, the_stop: Script) -> Self {
            let connection = rusqlite::Connection::open_in_memory().expect("a record");
            for migration in crate::record::migrations::ALL {
                if let crate::record::migrations::WhatItRuns::TheseStatements(statements) = migration.what_it_runs {
                    for statement in statements {
                        connection.execute(statement, []).expect("the schema");
                    }
                }
            }
            let log = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
            OverARecord {
                connection: std::rc::Rc::new(connection),
                machine: ASupervisor::new(&log, the_clear, the_stop, 1),
                forgotten: Vec::new(),
            }
        }
        fn the_records(&self) -> Vec<smoozy_state::outcomes::Recorded> {
            let mut statement = self.connection.prepare("SELECT identity FROM outcome_record").unwrap();
            let identities: Vec<String> = statement
                .query_map([], |row| row.get(0))
                .unwrap()
                .map(|one| one.unwrap())
                .collect();
            identities
                .iter()
                .map(|text| {
                    let identity = Identity::read_back(IdentityKind::Operation, text).expect("an identity");
                    crate::record::outcome_records::OutcomeRecords::over(&self.connection)
                        .read(&identity)
                        .unwrap()
                        .unwrap()
                })
                .collect()
        }
    }

    impl TheMachineReachingRemovalsEnds for OverARecord {
        fn the_pair(&mut self, _server: &str, the_units_after: &[&str]) -> ThePairWent {
            let mut keeper = crate::record::outcome_records::OutcomeRecords::over(&self.connection);
            crate::servers::stop_and_clear::the_pair_is_sent(
                &mut keeper,
                &mut self.machine,
                &Default::default(),
                the_units_after,
                QUICK,
            )
        }
        fn telegrams_role_off(&mut self, _server: &str) -> UnitOutcome {
            crate::servers::stop_and_clear::the_telegram_role_is_switched_off(&mut self.machine)
        }
        fn a_unit_ended(&mut self, record: &Identity, unit: &str, outcome: UnitOutcome) {
            crate::record::outcome_records::OutcomeRecords::over(&self.connection)
                .a_unit_ended(record, unit, outcome)
                .expect("the unit is owed");
        }
        fn what_the_machine_journalled_is_applied(&mut self, _server: &str) {}
        fn the_local_forget(&mut self, server: &str) -> TheAnswer {
            self.forgotten.push(server.to_string());
            TheAnswer::ItWasSaved
        }
    }

    /// **V15's seam, over the removal's own first unit** — what
    /// [`TheRemovalsOwnPath`] is in the application, given ends a test can
    /// hold.
    struct ThePairAloneOver<'e>(&'e mut dyn TheMachineReachingRemovalsEnds);

    impl TheOverridesEnds for ThePairAloneOver<'_> {
        fn the_stop_and_clear(&mut self, server: &str) -> Result<TheStopAndClearEnded, TheAnswer> {
            Ok(the_pair_alone(self.0, server))
        }
    }

    /// **`Servers` W25: the pair, then Telegram's role off, then the local
    /// forget — and the forget only on *cleared and stopped*.** Anything else
    /// keeps the server and owes `Servers` W21's ending on the list; a pair
    /// whose record would not write attempted nothing, so nothing after it
    /// runs either.
    #[test]
    fn stop_and_clear_first_runs_the_three_units_in_order_and_forgets_only_on_cleared_and_stopped() {
        let mut units = TheUnitsInHand::coming_to(Came::ClearedAndStopped);
        assert_eq!(stop_and_clear_first("s2-units-one", &mut units), TheAnswer::ItWasSaved);
        assert_eq!(
            units.log,
            vec![
                format!("the pair, enumerating after it: {TELEGRAMS_ROLE_OFF}"),
                "Telegram's role off".to_string(),
                format!("{TELEGRAMS_ROLE_OFF}: Done"),
                "the journal applied".to_string(),
                "the local forget".to_string(),
            ]
        );
        for came_to in [Came::NothingWasChanged, Came::TheClearWasRefused, Came::TheRenderHadNotStopped, Came::NothingCameBack] {
            let mut units = TheUnitsInHand::coming_to(came_to);
            assert_eq!(
                stop_and_clear_first("s2-units-two", &mut units),
                TheAnswer::TheStopAndClearEndingIsOwed,
                "{came_to:?}"
            );
            assert!(!units.log.iter().any(|unit| unit == "the local forget"), "{came_to:?}: the server was forgotten");
            assert!(!units.log.iter().any(|unit| unit == "the journal applied"), "{came_to:?}");
            assert!(units.log.iter().any(|unit| unit == "Telegram's role off"), "{came_to:?}: `Servers` W25's second unit is independent");
        }
        let mut units = TheUnitsInHand::coming_to(Came::NothingWasChanged);
        units.with_a_record = false;
        stop_and_clear_first("s2-units-three", &mut units);
        assert_eq!(units.log.len(), 1, "a unit ran with no record behind it: {:?}", units.log);
        let _ = the_owed_ending_is_taken(&[ServerRow { local_server_identifier: "s2-units-two".into(), ..a_row() }]);
        let _ = the_owed_ending_is_taken(&[ServerRow { local_server_identifier: "s2-units-three".into(), ..a_row() }]);
    }

    /// **`Servers` W22 and `Queue & History` L17: the act's whole enumeration
    /// is durable in the local record before the first request leaves**, and
    /// what was learned is written as it lands — through the real §9.4 store.
    #[test]
    fn the_record_holds_the_whole_enumeration_before_the_first_request_leaves() {
        let mut ends = OverARecord::answering(Script::Word(IT_WAS_CLEARED), Script::Word(IT_WAS_LEFT_STOPPED));
        // **The machine reads the record at the moment the clear arrives.**
        let seen: std::rc::Rc<std::cell::RefCell<Option<Vec<(String, Option<UnitOutcome>)>>>> = Default::default();
        struct Looking<'a> {
            inner: &'a mut ASupervisor,
            connection: std::rc::Rc<rusqlite::Connection>,
            seen: std::rc::Rc<std::cell::RefCell<Option<Vec<(String, Option<UnitOutcome>)>>>>,
        }
        impl crate::servers::the_machines_work::AsksTheSupervisor for Looking<'_> {
            fn ask(&mut self, request: &smoozy_server_protocol::framing::Request) -> smoozy_server_protocol::asking_the_supervisor::WhatCameBack {
                if request.name == THE_CLEAR_REQUEST && self.seen.borrow().is_none() {
                    let identity: String = self
                        .connection
                        .query_row("SELECT identity FROM outcome_record", [], |row| row.get(0))
                        .expect("the record was not written before the clear left");
                    let identity = Identity::read_back(IdentityKind::Operation, &identity).unwrap();
                    let read = crate::record::outcome_records::OutcomeRecords::over(&self.connection)
                        .read(&identity)
                        .unwrap()
                        .unwrap();
                    *self.seen.borrow_mut() = Some(read.units);
                }
                self.inner.ask(request)
            }
        }
        let connection = ends.connection.clone();
        let mut looking = Looking { inner: &mut ends.machine, connection: connection.clone(), seen: seen.clone() };
        let mut keeper = crate::record::outcome_records::OutcomeRecords::over(&connection);
        let went = crate::servers::stop_and_clear::the_pair_is_sent(
            &mut keeper,
            &mut looking,
            &Default::default(),
            &[TELEGRAMS_ROLE_OFF],
            QUICK,
        );
        assert_eq!(went.came_to, Came::ClearedAndStopped);
        assert_eq!(
            seen.borrow().clone().expect("the clear was sent"),
            vec![
                (THE_CLEAR.to_string(), None),
                (THE_STOP.to_string(), None),
                (TELEGRAMS_ROLE_OFF.to_string(), None),
            ]
        );
        let records = ends.the_records();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].units[0].1, Some(UnitOutcome::Done));
        assert_eq!(records[0].units[1].1, Some(UnitOutcome::Done));
    }

    /// **Each of `Servers` W21's endings reaches the `sv-sc` box in its own
    /// words, and an older program's refusal reads SC-2's** — *could not stop
    /// and clear*, never *could not reach*, because Smoozy reached it.
    #[test]
    fn each_ending_reaches_the_box_in_its_own_words_and_a_refusal_reads_sc2s() {
        let boxes = the_boxes();
        for (came_to, which, lead) in [
            (Came::NothingWasChanged, "could-not-begin", words::COULD_NOT_BEGIN),
            (Came::NothingCameBack, "could-not-begin", words::COULD_NOT_BEGIN),
            (Came::TheClearWasRefused, "the-clear-was-refused", words::CLEAR_REFUSED),
            (Came::TheRenderHadNotStopped, "it-half-worked", words::HALF_WORKED),
        ] {
            let server = format!("s2-endings-{which}-{came_to:?}");
            let mut units = TheUnitsInHand::coming_to(came_to);
            assert_eq!(stop_and_clear_first(&server, &mut units), TheAnswer::TheStopAndClearEndingIsOwed);
            let row = ServerRow { local_server_identifier: server.clone(), ..a_row() };
            let owed = the_owed_ending_is_taken(std::slice::from_ref(&row)).expect("the ending is owed on the list");
            assert_eq!((owed.server.as_str(), owed.which.as_str()), (server.as_str(), which));
            let drawn = boxes
                .stop_and_clear
                .iter()
                .find(|ending| ending.which.as_str() == which)
                .expect("the box draws this ending");
            assert_eq!(drawn.lead.as_str(), Prose::new(lead).as_str());
        }
        // **Through the real sender: 0.1.6 refuses the clear by name.**
        let mut ends = OverARecord::answering(Script::RefusedByName, Script::RefusedByName);
        let server = "s2-endings-refused";
        assert_eq!(stop_and_clear_first(server, &mut ends), TheAnswer::TheStopAndClearEndingIsOwed);
        let row = ServerRow { local_server_identifier: server.into(), ..a_row() };
        assert_eq!(
            the_owed_ending_is_taken(std::slice::from_ref(&row)).map(|owed| owed.which.as_str().to_string()),
            Some("could-not-begin".to_string())
        );
        assert!(ends.forgotten.is_empty());
        assert_eq!(words::COULD_NOT_BEGIN, "Could not stop and clear the jobs on this server.");
        assert!(!words::COULD_NOT_BEGIN.contains("reach"), "a refusal is told as a machine Smoozy could not reach");
    }

    /// **`Servers` W24: Telegram's role comes off on *Stop and clear first* and
    /// on no other removal.** V15 keeps the server and sends the pair alone;
    /// *Leave it running* and *Remove anyway* reach no machine at all
    /// — the local forget takes no machine to ask.
    #[test]
    fn telegrams_role_off_is_sent_on_stop_and_clear_first_and_on_no_other_removal() {
        let mut ends = OverARecord::answering(Script::Word(IT_WAS_CLEARED), Script::Word(IT_WAS_LEFT_STOPPED));
        assert_eq!(stop_and_clear_first("s2-telegram", &mut ends), TheAnswer::ItWasSaved);
        assert_eq!(ends.machine.how_many(THE_TELEGRAM_ROLE_IS_DISABLED_REQUEST), 1);
        assert_eq!(ends.forgotten, vec!["s2-telegram".to_string()]);
        let records = ends.the_records();
        assert_eq!(records[0].units[2], (TELEGRAMS_ROLE_OFF.to_string(), Some(UnitOutcome::Done)));

        let mut ends = OverARecord::answering(Script::Word(IT_WAS_CLEARED), Script::Word(IT_WAS_LEFT_STOPPED));
        assert_eq!(the_pair_alone(&mut ends, "s2-telegram-v15"), TheStopAndClearEnded::ClearedAndStopped);
        assert_eq!(ends.machine.how_many(THE_TELEGRAM_ROLE_IS_DISABLED_REQUEST), 0, "V15 switched Telegram off");
        assert!(ends.forgotten.is_empty(), "V15 forgot the server");
        assert_eq!(ends.the_records()[0].units.len(), 2, "V15's record enumerates the pair alone");
    }

    /// **`Desktop app and delivery` V15 on *cleared and stopped*: the update
    /// follows at the next queue reading** — piece 9's order test, now through
    /// the real sender rather than a machine that answers the whole act at
    /// once.
    #[test]
    fn v15_on_cleared_and_stopped_the_update_follows_through_the_real_path() {
        use crate::servers::published::{the_server_carries, what_the_server_carries, WhatSmoozyOnTheServerIs as Carries};
        let (row, register) = a_connected_server();
        let server = row.local_server_identifier.clone();
        let of = register.what_is_true_of(&server);
        the_server_carries(&server, Some(Carries::TheUpdateWaits));
        let mut ends = OverARecord::answering(Script::Word(IT_WAS_CLEARED), Script::Word(IT_WAS_LEFT_STOPPED));
        let did = the_update_now(&server, of, &mut ThePairAloneOver(&mut ends));
        assert_eq!(did, WhatTheUpdateNowDid::ClearedAndStopped);
        assert_eq!(ends.machine.how_many(THE_CLEAR_REQUEST), 1);
        assert_eq!(what_the_server_carries(&server), Some(Carries::TheUpdateWaits));
        let log = std::rc::Rc::new(std::cell::RefCell::new(vec!["the stop and clear"]));
        let mut update = ItsUpdate { log: log.clone() };
        crate::servers::the_machines_work::the_step_nine(&server, Some(&a_reading_of(&[])), &|_: &Identity| None, &mut update);
        assert_eq!(*log.borrow(), vec!["the stop and clear", "the update"]);

        // **An older program's refusal keeps the update waiting and says SC-2's
        // words** — `Servers` W21's last paragraph, for V15.
        the_server_carries(&server, Some(Carries::TheUpdateWaits));
        let mut ends = OverARecord::answering(Script::RefusedByName, Script::RefusedByName);
        let did = the_update_now(&server, of, &mut ThePairAloneOver(&mut ends));
        assert_eq!(did, WhatTheUpdateNowDid::ItDidNotFullyWork(TheRemovalsEnding::CouldNotBegin));
        let said = the_pairs_of(&the_answer_event(did.the_answer().expect("an answer")));
        assert!(said.contains(&("said".to_string(), Prose::new(words::COULD_NOT_BEGIN).as_str().to_string())));
        assert_eq!(what_the_server_carries(&server), Some(Carries::TheUpdateWaits));
        the_server_carries(&server, None);
    }

    /// **`Servers` W22: the owed ending is handed over once, to the list's next
    /// drawing, and goes with its server** when the user answers *remove*.
    #[test]
    fn an_owed_ending_is_handed_over_once_and_goes_with_its_server() {

        let started = a_started("s2-owed");
        let connection = started.record.as_ref().expect("a record").connection();
        let key = a_key_row(Some("SHA256:owed"));
        key_rows::add(connection, &key).expect("a key row");
        the_add_is_performed(&started, &an_add(&key.identifier.as_text()));
        let row = the_rows_of(&started).remove(0);
        let server = row.local_server_identifier.clone();
        let mut units = TheUnitsInHand::coming_to(Came::TheRenderHadNotStopped);
        stop_and_clear_first(&server, &mut units);
        assert!(the_owed_ending_is_taken(&[]).is_none(), "an ending was handed over for a server not on the list");
        assert!(the_owed_ending_is_taken(std::slice::from_ref(&row)).is_some());
        assert!(the_owed_ending_is_taken(std::slice::from_ref(&row)).is_none(), "the box was raised twice");

        // ***Remove anyway***: the forget, sending nothing, and the
        // owed box goes with the row.
        stop_and_clear_first(&server, &mut TheUnitsInHand::coming_to(Came::NothingWasChanged));
        assert_eq!(the_removal_is_performed(&started, None, &server, false), TheAnswer::ItWasSaved);
        assert!(the_rows_of(&started).is_empty());
        assert!(the_owed_ending_is_taken(std::slice::from_ref(&row)).is_none());
    }

    /// ***Retry* in the `sv-sc` box is the removal's three units again.**
    #[test]
    fn try_again_is_routed_to_the_removals_stop_and_clear() {
        assert_eq!(where_an_act_goes("stop-and-clear"), WhereAnActGoes::StopAndClearFirst);
    }

    /// **The page raises the owed box, and its two ways on send what they
    /// should** — *Retry* the removal's stop and clear, marked while it
    /// waits, and *Remove anyway* the removal that reaches no
    /// machine. Runs `web/servers.js` under Node; skipped, and says so, where
    /// no Node is installed.
    #[test]
    fn the_page_raises_the_owed_box_and_its_two_ways_on_send_what_they_should() {
        let (row, register) = a_connected_server();
        let mut drawn = the_screen_from(std::slice::from_ref(&row), &[], None, Some(&register), &Default::default());
        drawn.the_stop_and_clear_ending = Some(TheOwedEnding {
            server: Verbatim::new(row.local_server_identifier.clone()),
            which: Verbatim::new("it-half-worked"),
        });
        let screen = serde_json::to_string(&drawn).expect("the screen serialises");
        let folder = std::env::temp_dir().join(format!("smoozy-servers-s2-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/servers.js");
        std::fs::copy(&page, folder.join("servers.js")).expect("the page is copied");
        for (name, text) in THE_STAND_INS {
            let text = if name == "harness.js" {
                THE_OWED_BOX_HARNESS.replace("THE_SCREEN", &screen)
            } else {
                text.to_string()
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the server list's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        assert_eq!(out["shown"], serde_json::json!(["it-half-worked"]), "{out}");
        assert!(out["heading"].as_str().unwrap_or_default().contains(Prose::new(words::STOPPING_WORK_ON).as_str()), "{out}");
        assert_eq!(
            out["again"],
            serde_json::json!({ "command": "an_act_that_needs_a_connection", "act": "stop-and-clear", "marked": true, "second": false }),
            "{out}"
        );
        assert_eq!(
            out["anyway"],
            serde_json::json!({ "command": "a_server_is_removed", "reachesTheMachine": false }),
            "{out}"
        );
        assert_eq!(out["afterTheAnswer"], serde_json::json!({ "marked": false }), "{out}");
    }

    /// **The removal box's switch-off, ticked, fills its box and shows a tick
    /// mark; unticked, it shows none** — the drawing's `.cb.on` with its ✓
    /// (the Creator's note 2 on the test copy, the same omission as Files' box,
    /// map piece 5). Before, only the label wore `on`, which no rule styles,
    /// so a tick drew nothing. Read off the page, whose removal box the
    /// harness above cannot open without a menu.
    #[test]
    fn the_removal_boxs_switch_off_shows_its_tick() {
        let page = include_str!("../../web/servers.js");
        let drawn = &page[page.find("function theRemovalBoxIsDrawn(").expect("the removal box")..];
        let drawn = &drawn[..drawn.find("\n}\n").expect("its end")];
        let shown = &drawn[drawn.find("const theTickIsShown = () => {").expect("the tick is drawn from the label")..];
        let shown = &shown[..shown.find("};").expect("its end")];
        assert!(shown.contains("tick.classList.toggle(\"on\", on);"), "the box does not fill");
        assert!(shown.contains("tick.textContent = on ? \"✓\" : \"\";"), "the box shows no tick mark");
        let pressed = &drawn[drawn.find("label.addEventListener(\"click\", () => {").expect("the press")..];
        let pressed = &pressed[..pressed.find("});").expect("its end")];
        assert!(pressed.contains("theTickIsShown();"), "a press does not draw the tick again");
        let already = drawn.find("if (said.the_switch_off.already) label.classList.add(\"on\");").expect("already");
        assert!(drawn[already..].starts_with("if (said.the_switch_off.already) label.classList.add(\"on\");\n      theTickIsShown();"),
            "a switch-off already ticked is not drawn ticked");
    }

    /// ***Stop and clear first* reaches the machine and is marked while it
    /// waits**, and *Stop jobs and update* too — read off the page, whose
    /// removal box the harness above cannot open without a menu.
    #[test]
    fn the_page_marks_each_press_that_waits_on_the_machine() {
        let page = include_str!("../../web/servers.js");
        let choice = &page[page.find("function aChoice(").expect("the choice")..];
        let choice = &choice[..choice.find("\n}\n").expect("its end")];
        assert!(choice.contains("if (reachesTheMachine) theControlIsUnderway(head, choice.head);"));
        assert!(choice.contains("theRemovalIsAsked(server, reachesTheMachine);"));
        let update = &page[page.find("function theUpdateNowIsOpened(").expect("the V15 box")..];
        let update = &update[..update.find("\n}\n").expect("its end")];
        assert!(update.contains("theControlIsUnderway(stop, said.stop_jobs_and_update.label);"));
        assert!(page.contains("if (said.what === \"the-stop-and-clear-ending-is-owed\") {"));
    }

    /// **The harness for the owed box**: the page draws the screen handed in,
    /// which owes one `sv-sc` box; the harness reads which ending shows, then
    /// presses *Retry* twice and *Remove anyway* once, and
    /// tells the page an answer arrived.
    const THE_OWED_BOX_HARNESS: &str = r#"function anElement(tag) {
  const names = new Set();
  const listeners = {};
  const element = {
    tag, children: [], parentNode: null, dataset: {}, hidden: false, text: "",
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
    get textContent() { return element.text; },
    get childNodes() { return element.children; },
    get childElementCount() { return element.children.length; },
    appendChild(child) { child.parentNode = element; element.children.push(child); return child; },
    insertBefore(child) { return element.appendChild(child); },
    removeChild(child) { element.children = element.children.filter((one) => one !== child); return child; },
    replaceChild(fresh, old) {
      element.children = element.children.map((one) => (one === old ? fresh : one));
      fresh.parentNode = element;
      return old;
    },
    replaceChildren(...fresh) { element.children = []; fresh.forEach((one) => element.appendChild(one)); },
    setAttribute() {}, removeAttribute() {}, focus() {},
    addEventListener(kind, handler) { (listeners[kind] = listeners[kind] || []).push(handler); },
    click() { (listeners.click || []).forEach((one) => one({ stopPropagation() {}, preventDefault() {} })); },
    querySelector() { return null; }, querySelectorAll() { return []; }, closest() { return null; },
    contains() { return false; },
    getBoundingClientRect() { return { left: 0, bottom: 0, width: 0, height: 0 }; },
  };
  return element;
}
function all(element, found) {
  found.push(element);
  for (const child of element.children || []) all(child, found);
  return found;
}
function text(element) {
  return (element.text || "") + (element.children || []).map(text).join("");
}
globalThis.told = new Map();
globalThis.document = {
  createElement: anElement,
  createTextNode: (text) => ({ text, children: [] }),
  addEventListener() {}, removeEventListener() {},
};
globalThis.window = { innerWidth: 1000, innerHeight: 800 };
const theScreen = THE_SCREEN;
const invoked = [];
window.__TAURI__ = {
  core: {
    invoke(command, args) {
      invoked.push({ command, ...(args || {}) });
      if (command === "the_all_servers_screen") return Promise.resolve(theScreen);
      return Promise.resolve(null);
    },
  },
};
const settle = () => new Promise((done) => setImmediate(done));
const tell = (kind, said) => (globalThis.told.get(kind) || []).forEach((one) => one({ kind, said }));
await import("./servers.js");
const host = anElement("div");
globalThis.screen.itIsDrawn(host);
await settle();
await settle();
const box = all(host, []).find((one) => one.classList && one.classList.contains("mbox"));
const shown = box
  ? all(box, []).filter((one) => one.classList && one.classList.contains("rdfail") && !one.hidden).map((one) => {
      const index = box.children.indexOf(one) - 1;
      return theScreen.the_boxes_a_connection_raises.stop_and_clear[index].which;
    })
  : [];
const heading = box ? text(box.children[0]) : "";
const buttons = box ? all(box, []).filter((one) => one.tag === "button") : [];
const [again, anyway] = buttons;
const before = invoked.length;
again.click();
const marked = again.classList.contains("uway");
again.click();
const pressedAgain = invoked.slice(before);
tell("servers:the-answer", { what: "the-stop-and-clear-ending-is-owed" });
await settle();
const afterTheAnswer = { marked: again.classList.contains("uway") };
const beforeAnyway = invoked.length;
anyway.click();
const pressedAnyway = invoked.slice(beforeAnyway);
console.log(JSON.stringify({
  shown,
  heading,
  again: { command: pressedAgain[0].command, act: pressedAgain[0].act, marked, second: pressedAgain.length > 1 },
  anyway: { command: pressedAnyway[0].command, reachesTheMachine: pressedAnyway[0].reachesTheMachine },
  afterTheAnswer,
}));"#;

    /// The names and words one event says, read back through the real queue.
    fn the_pairs_of(event: &Event) -> Vec<(String, String)> {
        use std::sync::{Arc, Mutex};
        struct Held(Arc<Mutex<Vec<(String, String)>>>);
        impl crate::events::Delivers for Held {
            fn deliver(&self, delivery: &crate::events::Delivery) {
                if let Ok(mut held) = self.0.lock() {
                    for (name, said) in &delivery.said {
                        let text = match said {
                            Said::Prose(prose) => prose.as_str().to_string(),
                            Said::Value(value) => value.as_str().to_string(),
                        };
                        held.push((name.to_string(), text));
                    }
                }
            }
        }
        let held: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));
        let queue = TheOneQueue::new();
        queue.the_page_is_ready(Box::new(Held(held.clone())));
        let _ = queue.read_then_post::<(), std::convert::Infallible>(|| Ok(()), |_| event.clone());
        let out = held.lock().expect("the recorder is not poisoned").clone();
        out
    }

    /// The names one event says, for the two tests above.
    ///
    /// **The event is driven through the real queue** rather than read out of
    /// a field, which is the shape `src/surface/access.rs`'s own test uses and
    /// is why nothing here needs the queue's private map to be public.
    fn the_said_of(event: &Event) -> Vec<String> {
        use std::sync::{Arc, Mutex};
        struct Held(Arc<Mutex<Vec<String>>>);
        impl crate::events::Delivers for Held {
            fn deliver(&self, delivery: &crate::events::Delivery) {
                if let Ok(mut held) = self.0.lock() {
                    for name in delivery.said.keys() {
                        held.push(name.to_string());
                    }
                }
            }
        }
        let held: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let queue = TheOneQueue::new();
        queue.the_page_is_ready(Box::new(Held(held.clone())));
        let _ = queue.read_then_post::<(), std::convert::Infallible>(|| Ok(()), |_| event.clone());
        let out = held.lock().expect("the recorder is not poisoned").clone();
        out
    }

    // ── Step 10c's eleventh-pass fix piece 6: the server list tells the truth ──

    /// A register holding one connected period for `server`, as a *Connect*
    /// that went live leaves it.
    fn a_register_connected_to(server: &str) -> crate::servers::a_period::ThePeriodsThisRun {
        use crate::servers::a_period::{ThePeriodsThisRun, TheRunThroughTheApp};
        use smoozy_servers::acting::WhatServersPublishes;
        use smoozy_servers::condition::ObservedValue;
        use smoozy_servers::driving::APeriod;
        use smoozy_servers::period::{OpeningAct, WhoTookIt};
        let register = ThePeriodsThisRun::over(
            TheRunThroughTheApp::lending_nothing(),
            std::sync::Arc::new(crate::events::TheOneQueue::new()),
            std::sync::Arc::new(crate::notifications::TheNotificationService::on(Box::new(
                NothingIsPosted,
            ))),
        );
        let period = APeriod::begun(
            server,
            OpeningAct::TheUsersConnectAction,
            WhoTookIt::TheUser,
            std::time::SystemTime::UNIX_EPOCH,
        );
        register.a_period_has_begun(server, &period);
        register
            .where_a_period_publishes()
            .the_observed_value(server, ObservedValue::Connected);
        register
    }

    /// **Step 10c's eleventh pass, fault 10: *Remove* on a server with jobs gave
    /// the plain box.** The row read *Rendering p11 s2a · 1 waiting* and Queue
    /// *1 rendering · 1 waiting*, and *More → Remove* three times offered only
    /// *Cancel · Remove*. `Servers` W19: the box is composed from the
    /// period-held last-known queue reading (`Queue & History` C6), which the
    /// row and Queue already read. **Driven the way the press composes it**, so
    /// it fails on the command that handed `Servers` W19 *no record at all*
    /// whatever the period held.
    #[test]
    fn remove_on_a_server_with_a_job_rendering_and_one_waiting_offers_both_choices_and_the_counts() {
        use smoozy_queue_history::line::QueueState;
        let started = a_started("p11-fix-6-remove");
        let row = a_row();
        let server = row.local_server_identifier.clone();
        server_rows::add(started.record.as_ref().expect("a record").connection(), &row).expect("the row");
        let register = a_register_connected_to(&server);
        let holds = register.the_machines_work();
        let of = what_is_true_of(Some(&register), &server);
        assert!(of.a_period_stands && of.the_primary_connection_is_live);
        let the_server = Identity::read_back(IdentityKind::LocalServer, &server).expect("an identity");
        let store = crate::record::job_records::JobRecords::over(
            started.record.as_ref().expect("a record").connection(),
        );
        let held = |label: &str| {
            let held = crate::record::deletions::tests::a_record_in_history(&the_server, label);
            store.write_the_provisional_record(&held).expect("a job record");
            held.job
        };
        let an_operation = || Identity::issue(IdentityKind::Operation).expect("randomness");
        let (s2a, s2b) = (held("p11 s2a"), held("p11 s2b"));
        crate::surface::the_queue::tests::a_reading_of(
            holds,
            &the_server,
            vec![
                (s2a.clone(), QueueState::Rendering { operation: an_operation() }),
                (s2b.clone(), QueueState::Waiting),
            ],
        );

        let drawn = the_removal_box_for(&started, of, Some(holds), &server).expect("the box");
        assert_eq!(drawn.offers.as_str(), "both", "the box offered {drawn:?}");
        assert!(drawn.leave_it_running.is_some());
        assert!(drawn.stop_and_clear.is_some());
        assert!(drawn.remove.is_none(), "a plain Remove beside the two choices");
        assert_eq!(
            the_runs_read(drawn.the_warning.as_deref().expect("the warning")),
            vec![
                "1".to_string(),
                Prose::new(words::ONE_JOB_IS_RENDERING).as_str().to_string(),
                "1".to_string(),
                Prose::new(words::ONE_IS_WAITING).as_str().to_string(),
                Prose::new(words::RENDERING_AND_WAITING).as_str().to_string(),
            ]
        );

        // **A job whose ending Smoozy has learned is not offered to be stopped**
        // — the same composition the row and Queue read
        // (`Queue & History` C11), before the next reading says so.
        crate::surface::the_queue::tests::its_ending_is_learned(&store, &s2a, "p11 s2a");
        let one_waits = the_removal_box_for(&started, of, Some(holds), &server).expect("the box");
        assert_eq!(one_waits.offers.as_str(), "both");
        assert_eq!(
            the_runs_read(one_waits.the_warning.as_deref().expect("the warning")),
            vec![
                "1".to_string(),
                Prose::new(words::ONE_JOB_IS_WAITING).as_str().to_string(),
                Prose::new(words::RENDERING_AND_WAITING).as_str().to_string(),
            ]
        );
        crate::surface::the_queue::tests::a_reading_of(
            holds,
            &the_server,
            vec![
                (s2a.clone(), QueueState::Ended { outcome: an_operation() }),
                (s2b.clone(), QueueState::Ended { outcome: an_operation() }),
            ],
        );
        let nothing = the_removal_box_for(&started, of, Some(holds), &server).expect("the box");
        assert_eq!(nothing.offers.as_str(), "a-plain-confirmation");
        assert!(nothing.the_warning.is_none());

        // **The break declared: what Smoozy last knew, saying so, and *leave it
        // running* only** — `Servers` W19's third row, from the reading the
        // break left.
        crate::surface::the_queue::tests::a_reading_of(
            holds,
            &the_server,
            vec![(s2b.clone(), QueueState::Rendering { operation: an_operation() })],
        );
        holds.the_connection_ended(&server);
        let repairing = WhatIsTrueOfTheServer {
            a_period_stands: true,
            the_primary_connection_is_live: false,
            a_run_of_attempts_is_in_flight: true,
            the_restart_mark_stands: false,
        };
        let last_known = the_removal_box_for(&started, repairing, Some(holds), &server).expect("the box");
        assert_eq!(last_known.offers.as_str(), "leave-it-be-only");
        assert_eq!(
            last_known.the_warning.as_deref().expect("the warning")[0].lead,
            Prose::new(words::WHEN_SMOOZY_WAS_LAST_CONNECTED)
        );

        // **The period ends and the reading goes with it**
        // (`Queue & History` C6): the plain box, `Servers` W19's last row.
        holds.the_period_ended(&server);
        let gone = the_removal_box_for(&started, what_is_true_of_a_server_with_no_period(), Some(holds), &server)
            .expect("the box");
        assert_eq!(gone.offers.as_str(), "a-plain-confirmation");
        assert!(gone.the_warning.is_none());
    }

    /// **Step 10c's eleventh pass, fault 5: *Add and set up* said *Smoozy cannot
    /// do this yet. This part of the app is not finished.*** while the setup
    /// placed Smoozy and installed Blender, and the line stayed for the whole
    /// session. Since step 10c piece 9 a ticked add connects and sets up
    /// (`Servers` O2), so the answer, like a plain add's, says nothing: the
    /// list and the setup say what is happening.
    #[test]
    fn add_and_set_up_posts_no_sentence() {
        let told = the_answer_event(TheAnswer::ItWasSavedAndNothingStarted);
        assert!(
            !the_said_of(&told).iter().any(|name| name == "said"),
            "Add and set up said {told:?}"
        );
        assert_eq!(
            the_said_of(&told),
            the_said_of(&the_answer_event(TheAnswer::ItWasSaved)),
            "a ticked add is answered as a plain one is"
        );
    }

    /// **Step 10c's eleventh pass, *Smaller* 1–2: the list read *Idle* while a
    /// job rendered, right after a *Connect*, and the page header read the dash
    /// where the list read *Idle*.** A connected server with no reading held
    /// said *Idle* unless step 7 was already known to owe one — so from the
    /// connect until the check had settled, *Idle* was said of a machine
    /// nobody had read. **The dash until a reading lands, except where the
    /// check has said the machine holds no Smoozy of this installation**
    /// (`Queue & History` A1: nothing on it is read as Smoozy's at all), which
    /// stays *Idle*. The server's own head reads the same words at every step.
    #[test]
    fn a_connected_server_with_no_reading_reads_the_dash_until_one_lands() {
        use crate::servers::a_period::WhatTheLastConnectionRead;
        let row = a_row();
        let server = row.local_server_identifier.clone();
        let register = a_register_connected_to(&server);
        let the_work = || {
            let work = crate::surface::the_queue::what_the_servers_are_doing(register.the_machines_work(), None);
            let listed = the_screen_from(std::slice::from_ref(&row), &[], None, Some(&register), &work).rows[0]
                .the_work
                .clone();
            let head = crate::surface::a_server::the_head_from(
                std::slice::from_ref(&row),
                &server,
                Some(&register),
                &work,
            )
            .expect("the head");
            assert_eq!(head.the_work, listed, "the head and the list read two ways");
            listed
        };
        let dash = || vec![the_dash()];
        let idle = || vec![Words::plain(WhatTheColumnSays::Idle.the_word())];

        // **Connected, and the check has not settled**: nothing is known.
        assert_eq!(the_work(), dash(), "connected with nothing read");

        // **The check confirmed the installation**: step 7 owes a reading.
        let read = |check: InstallationCheck, live: bool| WhatTheLastConnectionRead {
            check,
            a_fresh_reading_was_kept: false,
            on_the_connection_that_is_live: live,
        };
        register.a_connection_read(&server, read(InstallationCheck::PresentAndEqualConfirmed, true));
        assert_eq!(the_work(), dash(), "a reading owed");

        // **An earlier connection's check is not this one's.**
        register.a_connection_read(&server, read(InstallationCheck::Absent, false));
        assert_eq!(the_work(), dash(), "another connection's absent branch");

        // **`Queue & History` A1's case: no Smoozy of this installation on the
        // machine** — Idle.
        register.a_connection_read(&server, read(InstallationCheck::Absent, true));
        assert_eq!(the_work(), idle(), "a machine with no Smoozy on it");
        register.a_connection_read(&server, read(InstallationCheck::PresentAndDifferent, true));
        assert_eq!(the_work(), idle(), "another installation's machine");

        // **The reading lands holding nothing live** — Idle, because Smoozy knows.
        register.a_connection_read(&server, read(InstallationCheck::PresentAndEqualConfirmed, true));
        let the_server = Identity::read_back(IdentityKind::LocalServer, &server).expect("an identity");
        crate::surface::the_queue::tests::a_reading_of(register.the_machines_work(), &the_server, Vec::new());
        assert_eq!(the_work(), idle(), "a reading holding nothing");
    }

    /// **A nought count is left out of the removal box rather than said**, now
    /// that the box has a reading to count (Rule 9, *Short, too*): a render
    /// with nothing waiting reads *1 job is rendering on this server.* and
    /// *Stop and clear first* says it stops the render and clears nothing;
    /// jobs waiting with nothing rendering read *2 jobs are waiting* and are
    /// cleared, with no render said to stop. The eleventh pass's own case, one
    /// and one, keeps both halves. **V15's box says the same statement**, off
    /// the same function. Step 10c's eleventh-pass fix piece 6.
    #[test]
    fn a_nought_count_is_left_out_of_the_removal_box() {
        let row = a_row();
        let all = vec![row.clone()];
        let connected = WhatIsTrueOfTheServer {
            a_period_stands: true,
            the_primary_connection_is_live: true,
            a_run_of_attempts_is_in_flight: false,
            the_restart_mark_stands: false,
        };
        let repairing = WhatIsTrueOfTheServer {
            the_primary_connection_is_live: false,
            a_run_of_attempts_is_in_flight: true,
            ..connected
        };
        let boxed = |of, rendering, waiting| {
            the_removal_box_from(&row, &all, of, TheLastKnownReading::Holds { rendering, waiting }, false)
        };
        let prose = |said: &str| Prose::new(said).as_str().to_string();

        let one_rendering = boxed(connected, 1, 0);
        assert_eq!(
            the_runs_read(one_rendering.the_warning.as_deref().expect("a warning")),
            vec!["1".to_string(), prose(words::ONE_JOB_IS_RENDERING_ALONE), prose(words::RENDERING_AND_WAITING)]
        );
        assert_eq!(
            one_rendering.stop_and_clear.expect("stop and clear").said,
            Words::plain(words::STOP_AND_CLEAR_SAID_NONE_WAITING)
        );

        let two_waiting = boxed(connected, 0, 2);
        assert_eq!(
            the_runs_read(two_waiting.the_warning.as_deref().expect("a warning")),
            vec!["2".to_string(), prose(words::JOBS_ARE_WAITING), prose(words::RENDERING_AND_WAITING)]
        );
        assert_eq!(
            the_runs_read(std::slice::from_ref(&two_waiting.stop_and_clear.expect("stop and clear").said)),
            vec![prose(words::STOP_AND_CLEAR_SAID_LEAD), "2".to_string(), prose(words::STOP_AND_CLEAR_SAID_MANY_NONE_RENDERING)]
        );
        assert_eq!(
            boxed(connected, 0, 1).stop_and_clear.expect("stop and clear").said,
            Words::plain(words::STOP_AND_CLEAR_SAID_ONE_NONE_RENDERING)
        );

        let both = boxed(connected, 1, 1);
        assert_eq!(the_runs_read(both.the_warning.as_deref().expect("a warning")).len(), 5);
        assert_eq!(both.stop_and_clear.expect("stop and clear").said, Words::plain(words::STOP_AND_CLEAR_SAID_ONE));

        assert_eq!(
            the_runs_read(boxed(repairing, 1, 0).the_warning.as_deref().expect("a warning")),
            vec![prose(words::WHEN_SMOOZY_WAS_LAST_CONNECTED), "1".to_string(), prose(words::ONE_JOB_WAS_RENDERING_ALONE)]
        );
        assert_eq!(
            the_runs_read(boxed(repairing, 0, 3).the_warning.as_deref().expect("a warning")),
            vec![prose(words::WHEN_SMOOZY_WAS_LAST_CONNECTED), "3".to_string(), prose(words::JOBS_WERE_WAITING)]
        );

        // **V15's box draws the same statement**, off the same function.
        let rendering_alone = WhatAServerIsDoing::Rendering { name: "p11 s2a".to_string(), waiting: 0 };
        assert_eq!(
            the_update_now_box("A4000", Some(&rendering_alone)).the_statement,
            Words::plain(words::STOP_AND_CLEAR_SAID_NONE_WAITING)
        );
        let two_held = WhatAServerIsDoing::Waiting { how_many: 2 };
        assert_eq!(
            the_runs_read(std::slice::from_ref(&the_update_now_box("A4000", Some(&two_held)).the_statement)),
            vec![prose(words::STOP_AND_CLEAR_SAID_LEAD), "2".to_string(), prose(words::STOP_AND_CLEAR_SAID_MANY_NONE_RENDERING)]
        );
    }

    /// **The Creator's note 1 on 0.1.7, and it runs the page code rather than
    /// reading it: the Add dialog's *New key…* lands on Keys with the New key
    /// dialog showing** — `Design/Keys § Opened from a server dialog`. The
    /// harness loads the real `web/servers.js` **and the real `web/keys.js`**
    /// beside a rail stand-in that mounts each screen as the rail does, opens
    /// the Add dialog the application composes, and presses *New key…*; then
    /// cancels on Keys, which hands back nothing and leaves the key field as
    /// it was; then opens the key field's own list and presses its last row,
    /// *Open Keys…*, which opens the Keys screen alone, exactly as the field
    /// itself did until the Creator's eighth notes, piece E3.
    /// **Before the piece the New key dialog stayed hidden after the press.**
    /// It also reads the port's class off the drawn field (note 2). Skipped,
    /// and says so, where no Node is installed.
    #[test]
    fn the_dialogs_new_key_lands_on_keys_with_the_new_key_dialog_showing() {
        let dialog = serde_json::to_string(&the_add_dialog_from(&[])).expect("the dialog serialises");
        let list = serde_json::to_string(&the_keys_a_dialog_offers_from(&[])).expect("the list serialises");
        let folder = std::env::temp_dir().join(format!("smoozy-servers-new-key-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["servers.js", "keys.js", "the-value-drawn.js"] {
            std::fs::copy(web.join(page), folder.join(page)).expect("the page is copied");
        }
        for (name, text) in THE_KEY_CONTROLS_STAND_INS {
            let text = if name == "harness.js" {
                text.replace("THE_ADD_DIALOG", &dialog).replace("THE_KEY_LIST", &list)
            } else {
                text.to_string()
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Add dialog's key controls were not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        assert_eq!(out["afterNewKey"], serde_json::json!({ "place": "keysview", "theNewKeyDialogShows": true }), "{out}");
        assert_eq!(
            out["afterCancel"],
            serde_json::json!({ "place": "serversview", "theNewKeyDialogShows": false, "theKeyFieldReads": "choose" }),
            "a cancel hands back nothing: {out}"
        );
        assert_eq!(
            out["afterTheKeyField"],
            serde_json::json!({ "place": "keysview", "theNewKeyDialogShows": false }),
            "the key list's Open Keys… opens Keys alone: {out}"
        );
        assert_eq!(out["thePortsClasses"], "fld w-frame mono svport", "{out}");
    }

    /// **A key row with a name and a fingerprint**, for the dialog's key list.
    fn a_named_key(name: &str, fingerprint: Option<&str>) -> smoozy_keys::rows::KeyRow {
        smoozy_keys::rows::KeyRow::entering(
            Identity::issue(IdentityKind::LocalKey).expect("randomness"),
            std::path::PathBuf::from(format!("/nowhere/{name}")),
            fingerprint.map(|printed| smoozy_keys::fingerprint::Reading {
                value: smoozy_keys::fingerprint::Fingerprint::from_the_printed_form(printed)
                    .expect("a fingerprint"),
                read_from: smoozy_keys::fingerprint::ReadFrom::PrivateFile,
            }),
            smoozy_keys::rows::Name::from_the_comment_field(name),
            false,
        )
        .expect("a key row")
    }

    /// **The Creator's eighth notes, piece E3: a server dialog's key list is
    /// the record's keys in the Keys screen's own order, each by name with its
    /// fingerprint's start** — `Design/Keys § Opened from a server dialog`, the
    /// drawing's `sv-addkey` (*RunPod  SHA256:qYt4x1Lp…*). A key with no
    /// fingerprint is not offered, since the dialog hands back the pair; a key
    /// with no name shows its whole fingerprint and no start beside it. No
    /// verdict is in it (`Keys` U10a).
    #[test]
    fn the_dialogs_key_list_is_keys_own_order_with_each_fingerprints_start() {
        let runpod = a_named_key("RunPod", Some("SHA256:qYt4x1LpZZZZ"));
        let first = a_named_key("Studio MacBook Pro", Some("SHA256:8vBn2QwEZZZZ"));
        let short = a_named_key("Brief", Some("SHA256:12345678"));
        let none = a_named_key("No fingerprint", None);
        let unnamed = a_key_row(Some("SHA256:unnamedZZZZ"));
        let offered = the_keys_a_dialog_offers_from(&[
            runpod.clone(),
            none,
            first.clone(),
            short.clone(),
            unnamed.clone(),
        ]);
        let one = |row: &smoozy_keys::rows::KeyRow, shows: &str, start: Option<&str>, runs_on: bool| AKeyADialogOffers {
            key: Verbatim::new(row.identifier.as_text()),
            shows_as: Verbatim::new(shows),
            fingerprint_start: start.map(Verbatim::new),
            it_runs_on: runs_on,
        };
        assert_eq!(
            offered,
            TheKeysADialogOffers {
                keys: vec![
                    // A row with no name sorts by its path, here before the
                    // named ones.
                    one(&unnamed, "SHA256:unnamedZZZZ", None, false),
                    one(&short, "Brief", Some("SHA256:12345678"), false),
                    one(&runpod, "RunPod", Some("SHA256:qYt4x1Lp"), true),
                    one(&first, "Studio MacBook Pro", Some("SHA256:8vBn2QwE"), true),
                ],
                checking: Prose::new("Checking…"),
                open_keys: Prose::new(words::OPEN_KEYS_ITEM),
                the_fingerprint_runs_on: Prose::new(words::THE_FINGERPRINT_RUNS_ON),
            }
        );
        // **Each verdict travels with the ask it answers**, in the Keys screen's
        // own three words.
        let pairs = the_pairs_of(&a_dialog_keys_verdict("7", "key-1", &usable::Verdict::Usable));
        assert_eq!(
            pairs,
            vec![
                ("ask".to_string(), "7".to_string()),
                ("key".to_string(), "key-1".to_string()),
                ("verdict".to_string(), "usable".to_string()),
            ]
        );
        let word = |verdict: &usable::Verdict| {
            the_pairs_of(&a_dialog_keys_verdict("7", "key-1", verdict))[2].1.clone()
        };
        assert_eq!(word(&usable::Verdict::Unusable(usable::Unusable::FileMissing)), "unusable");
        assert_eq!(word(&usable::Verdict::NotChecked), "not-checked");
        // **The page listens on the same kind.**
        assert!(include_str!("../../web/servers.js").contains(A_DIALOG_KEYS_VERDICT));
    }

    /// **The Edit dialog's own unusable key keeps its place, with the gone
    /// mark, even where its key row is gone** — `Design/Servers § The Edit
    /// dialog`: *"A server needing a new key still shows the key it was added
    /// with, marked as gone."* Where the row is gone, the fingerprint the server
    /// row holds (`Servers` P16) stands in for its name. A usable key carries no
    /// mark. Piece E3.
    #[test]
    fn the_edit_dialogs_gone_key_carries_the_mark_and_still_names_itself() {
        let key = a_named_key("Hetzner", Some("SHA256:aaaa"));
        let mut row = a_row();
        row.key.local_key_identifier = key.identifier.as_text();
        let said = |verdict: usable::Verdict| WhatKeysSaid {
            the_tools_are_installed: true,
            verdicts: vec![(key.identifier.as_text(), verdict)],
            ..Default::default()
        };
        let unusable = said(usable::Verdict::Unusable(usable::Unusable::FileMissing));
        let gone = the_edit_dialog_from(&row, std::slice::from_ref(&row), std::slice::from_ref(&key), Some(&unusable));
        assert_eq!(gone.the_gone_mark, Some(Prose::new(words::THE_GONE_MARK)));
        assert_eq!(gone.the_key_shows_as, Some(Verbatim::new("Hetzner")));
        let no_row = the_edit_dialog_from(&row, std::slice::from_ref(&row), &[], Some(&unusable));
        assert_eq!(no_row.the_gone_mark, Some(Prose::new(words::THE_GONE_MARK)));
        assert_eq!(no_row.the_key_shows_as, Some(Verbatim::new("SHA256:aaaa")));
        let fine = said(usable::Verdict::Usable);
        let kept = the_edit_dialog_from(&row, std::slice::from_ref(&row), std::slice::from_ref(&key), Some(&fine));
        assert_eq!(kept.the_gone_mark, None);
        assert_eq!(kept.the_key_shows_as, Some(Verbatim::new("Hetzner")));
    }

    /// **The Creator's eighth notes, piece E3, and it runs the page code: the
    /// key field is Smoozy's own dropdown of the usable keys, and a pick stays
    /// in the dialog** — `Design/Keys § Opened from a server dialog`, the
    /// drawing's `sv-add`, `sv-addkey` and `sv-edit`. Over the application's
    /// own dialogs, list and verdict events, under Node:
    ///
    /// - **Add**: the dialog asks for the list once and for every key's verdict;
    ///   every key reads *Checking…* and cannot be picked until its verdict
    ///   arrives; a usable key shows its fingerprint's start, an unusable one
    ///   leaves, and an answer to another ask is dropped, so that key is still
    ///   checking and a press on it does nothing. A pick fills the field, closes
    ///   the list and leaves the dialog where it is; the picked row is in bold
    ///   next time. Escape closes the list and not the dialog. *Open Keys…*
    ///   opens Keys in its picking mode without the New key dialog, as the field
    ///   itself did, and the list is asked for again when Keys hands back.
    /// - **After a rejected Add**, choosing through the dropdown, Open Keys or
    ///   New key clears only the required-key line and its grid inset. Cancel
    ///   and a removed key leave the error standing. The next Add sends the
    ///   chosen key. Added for the twentieth notes' rented pass, 2026-10-29;
    ///   the three choices left the error visible before the correction.
    /// - **Edit**: the closed control and the top row read *Hetzner — missing*;
    ///   that row is picked, cannot be picked again, and stays whatever its
    ///   verdict says; a key the tools could not check stays, unpickable, and
    ///   stops saying *Checking…*.
    ///
    /// **Before the piece** the field was a button that opened Keys, and none
    /// of this was there. Skipped, and says so, where no Node is installed.
    #[test]
    fn the_key_dropdown_lists_the_usable_keys_and_a_pick_stays_in_the_dialog() {
        let hetzner = a_named_key("Hetzner", Some("SHA256:HHHHHHHHHHHH"));
        let runpod = a_named_key("RunPod", Some("SHA256:qYt4x1LpAAAA"));
        let first = a_named_key("Studio MacBook Pro", Some("SHA256:8vBn2QwEBBBB"));
        let second = a_named_key("Studio MacBook Pro", Some("SHA256:Lp0OiUyTCCCC"));
        let scaleway = a_named_key("Scaleway", Some("SHA256:ssssssssssss"));
        let keys = vec![hetzner.clone(), runpod.clone(), first.clone(), second.clone(), scaleway.clone()];
        let mut row = a_row();
        row.label = Some("Overnight box".to_string());
        row.key.local_key_identifier = hetzner.identifier.as_text();
        let unusable = WhatKeysSaid {
            the_tools_are_installed: true,
            verdicts: vec![(
                hetzner.identifier.as_text(),
                usable::Verdict::Unusable(usable::Unusable::FileMissing),
            )],
            ..Default::default()
        };
        let drawn = the_screen_from(std::slice::from_ref(&row), &[], None, None, &Default::default());
        let edit = the_edit_dialog_from(&row, std::slice::from_ref(&row), &keys, Some(&unusable));
        let mut held = serde_json::Map::new();
        held.insert(
            runpod.identifier.as_text(),
            serde_json::to_value(TheKeyADialogHolds {
                key: Verbatim::new(runpod.identifier.as_text()),
                fingerprint: Verbatim::new("SHA256:qYt4x1LpAAAA"),
                shows_as: Verbatim::new("RunPod"),
            })
            .expect("it serialises"),
        );
        let mut shape = serde_json::Map::new();
        for (name, text) in the_pairs_of(&a_dialog_keys_verdict("ASK", "KEY", &usable::Verdict::Usable)) {
            shape.insert(name, serde_json::Value::String(text));
        }
        let word = |verdict: usable::Verdict| {
            the_pairs_of(&a_dialog_keys_verdict("", "", &verdict))[2].1.clone()
        };
        let words_for = serde_json::json!({
            "usable": word(usable::Verdict::Usable),
            "unusable": word(usable::Verdict::Unusable(usable::Unusable::FileMissing)),
            "notChecked": word(usable::Verdict::NotChecked),
        });
        let ids = serde_json::json!({
            "hetzner": hetzner.identifier.as_text(),
            "runpod": runpod.identifier.as_text(),
            "first": first.identifier.as_text(),
            "second": second.identifier.as_text(),
            "scaleway": scaleway.identifier.as_text(),
        });
        let prefix = THE_RENAME_HARNESS
            .split("const theScreen = THE_SCREEN;")
            .next()
            .expect("the harness's document");
        let harness = format!("{prefix}{THE_KEY_LIST_SCENARIO}")
            .replace("THE_SCREEN", &serde_json::to_string(&drawn).expect("the screen serialises"))
            .replace("THE_ADD_DIALOG", &serde_json::to_string(&the_add_dialog_from(&[])).expect("it serialises"))
            .replace("THE_EDIT_DIALOG", &serde_json::to_string(&edit).expect("it serialises"))
            .replace("THE_LIST", &serde_json::to_string(&the_keys_a_dialog_offers_from(&keys)).expect("it serialises"))
            .replace("THE_HELD", &serde_json::Value::Object(held).to_string())
            .replace("THE_SHAPE", &serde_json::Value::Object(shape).to_string())
            .replace("THE_WORDS", &words_for.to_string())
            .replace("THE_IDS", &ids.to_string())
            .replace("THE_KIND", A_DIALOG_KEYS_VERDICT)
            .replace("THE_MISSING", &serde_json::to_string(&the_pairs_of(
                &the_answer_event(TheAnswer::TheseFieldsAreNeeded(vec![
                    ("host", words::HOST_IS_NEEDED),
                    ("username", words::USERNAME_IS_NEEDED),
                    ("key", words::A_KEY_IS_NEEDED),
                ])),
            )).expect("the refusal serialises"));
        let folder = std::env::temp_dir().join(format!("smoozy-servers-key-list-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/servers.js");
        std::fs::copy(&page, folder.join("servers.js")).expect("the page is copied");
        for (name, text) in THE_STAND_INS {
            let text = match name {
                "harness.js" => harness.clone(),
                "the-rail.js" => r#"export const ALL_SERVERS = "serversview";
export const A_SERVER = "machineview";
export const KEYS = "keysview";
export function aScreenIsRegistered(screen) { globalThis.screen = screen; }
export function theCurrentPlace() { return ALL_SERVERS; }
export function theCurrentPlaceIs(place, which) { (globalThis.went ||= []).push([place, which]); }
export function theRailIsReadAgain() {}"#
                    .to_string(),
                "keys.js" => r#"export function aServerDialogPicksAKey(context, handBack, withTheNewKeyDialog) {
  (globalThis.picks ||= []).push({ withTheNewKeyDialog: Boolean(withTheNewKeyDialog) });
  globalThis.handBack = handBack;
}"#
                .to_string(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the key dropdown was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let studio = "Studio MacBook Pro";
        let open_keys = serde_json::json!(["Open Keys…", false, false]);
        let wanted = serde_json::json!({
            "asked": {
                "list": 1,
                "checked": [
                    hetzner.identifier.as_text(),
                    runpod.identifier.as_text(),
                    scaleway.identifier.as_text(),
                    first.identifier.as_text(),
                    second.identifier.as_text(),
                ],
            },
            "closed": { "face": "Choose a key…", "listHidden": true },
            "beforeAnyVerdict": [
                ["Hetzner  Checking…", true, false],
                ["RunPod  Checking…", true, false],
                ["Scaleway  Checking…", true, false],
                [format!("{studio}  Checking…"), true, false],
                [format!("{studio}  Checking…"), true, false],
                open_keys,
            ],
            "afterTheVerdicts": [
                ["RunPod  SHA256:qYt4x1Lp…", false, false],
                ["Scaleway  Checking…", true, false],
                [format!("{studio}  SHA256:8vBn2QwE…"), false, false],
                open_keys,
            ],
            "aCheckingKeyIsPressed": { "open": true, "asked": false },
            "picked": {
                "dialog": true,
                "open": false,
                "face": "RunPod",
                "asked": [runpod.identifier.as_text()],
                "keys": 0,
            },
            "reopened": [
                ["RunPod  SHA256:qYt4x1Lp…", false, true],
                ["Scaleway  Checking…", true, false],
                [format!("{studio}  SHA256:8vBn2QwE…"), false, false],
                open_keys,
            ],
            "escaped": { "dialog": true, "open": false },
            "openKeys": { "picks": [{ "withTheNewKeyDialog": false }], "dialog": true, "open": false },
            "askedAgain": 1,
            "requiredKey": {
                "beforePick": [true, true, true],
                "dropdown": [false, false, true],
                "submitted": true,
                "cancel": [true, true, true],
                "gone": [true, true, true],
                "keys": [false, false, true],
                "newKey": [false, false, true],
            },
            "editClosed": "Hetzner — missing",
            "editRows": [
                ["Hetzner — missing", true, true],
                ["RunPod  SHA256:qYt4x1Lp…", false, false],
                ["Scaleway  SHA256:ssssssss…", true, false],
                [format!("{studio}  SHA256:8vBn2QwE…"), false, false],
                open_keys,
            ],
            "theGoneKeyIsPressed": { "open": true, "asked": false },
        });
        assert_eq!(got, wanted);
    }

    /// **The scenario for
    /// [`the_key_dropdown_lists_the_usable_keys_and_a_pick_stays_in_the_dialog`]**,
    /// run on the rename harness's document, whose events bubble as a
    /// browser's do — so Escape in the list reaches the dialog unless the list
    /// keeps it.
    const THE_KEY_LIST_SCENARIO: &str = r#"const theScreen = THE_SCREEN;
const theAddDialog = THE_ADD_DIALOG;
const theEditDialog = THE_EDIT_DIALOG;
const theListAnswer = THE_LIST;
const theHeld = THE_HELD;
const theShape = THE_SHAPE;
const theWords = THE_WORDS;
const ids = THE_IDS;
const missing = Object.fromEntries(THE_MISSING);
let invoked = [];
window.__TAURI__ = {
  core: {
    invoke(command, args) {
      invoked.push({ command, ...(args || {}) });
      if (command === "the_all_servers_screen") return Promise.resolve(theScreen);
      if (command === "the_add_dialog") return Promise.resolve(theAddDialog);
      if (command === "the_edit_dialog") return Promise.resolve(theEditDialog);
      if (command === "the_keys_a_dialog_offers") return Promise.resolve(theListAnswer);
      if (command === "the_key_a_dialog_holds") return Promise.resolve(theHeld[args.key] || null);
      if (command === "a_server_is_added" && !args.keyIdentifier) {
        queueMicrotask(() => tell("servers:the-answer", missing));
      }
      return Promise.resolve(null);
    },
  },
};
const settle = async () => { for (let at = 0; at < 4; at += 1) await new Promise((done) => setImmediate(done)); };
const tell = (kind, said) => (globalThis.told.get(kind) || []).forEach((one) => one({ kind, said }));
await import("./servers.js");
const host = anElement("div");
globalThis.screen.itIsDrawn(host);
await settle();
const flat = (one) => text(one).replace(/ /g, " ");
const theBox = () => all(host).find((one) => one.classList && one.classList.contains("mbox")) || null;
const theFace = () => all(theBox()).find((one) => one.classList && one.classList.contains("pkb")) || null;
const theKeyList = () => all(theBox()).find((one) => one.classList && one.classList.contains("ddl")) || null;
const theRows = () => theKeyList().children
  .filter((one) => one.tag === "button")
  .map((one) => [flat(one), one.classList.contains("off"), one.classList.contains("hd")]);
const theRowReading = (words) => theKeyList().children.find((one) => one.tag === "button" && flat(one).startsWith(words));
const theAsks = () => invoked.filter((one) => one.command === "the_keys_a_dialog_offers_are_checked");
const theLastAsk = () => theAsks()[theAsks().length - 1];
const verdict = (key, word, ask) => tell("THE_KIND", JSON.parse(
  JSON.stringify(theShape)
    .replace("ASK", ask === undefined ? theLastAsk().ask : ask)
    .replace("KEY", key)
    .replace(theWords.usable, theWords[word]),
));
const out = {};

// Add.
fire(all(host).find((one) => one.tag === "button" && text(one) === theScreen.add_a_server), "click");
await settle();
out.asked = {
  list: invoked.filter((one) => one.command === "the_keys_a_dialog_offers").length,
  checked: theLastAsk() ? theLastAsk().keys : null,
};
out.closed = { face: theFace() ? flat(theFace()) : null, listHidden: theKeyList() ? theKeyList().hidden : null };
if (!theFace()) { console.log(JSON.stringify(out)); process.exit(0); }
fire(theFace(), "click");
out.beforeAnyVerdict = theRows();
verdict(ids.hetzner, "unusable");
verdict(ids.runpod, "usable");
verdict(ids.first, "usable");
verdict(ids.second, "unusable");
// **An answer to an ask this list has moved past is dropped.**
verdict(ids.scaleway, "usable", "an earlier ask");
await settle();
out.afterTheVerdicts = theRows();
// Reject Add first, then choose a key. Only the key's error and its grid
// inset should disappear; the other missing fields must keep their errors.
const add = () => all(theBox()).find((one) => one.tag === "button" && text(one) === theAddDialog.add);
fire(add(), "click");
await settle();
const keyError = all(theBox()).find((one) => text(one) === missing.field2Said);
const hostError = all(theBox()).find((one) => text(one) === missing.field0Said);
const keyErrorReads = () => [!keyError.hidden, !keyError.theInset.hidden, !hostError.hidden];
out.requiredKey = { beforePick: keyErrorReads() };
invoked = [];
fire(theRowReading("Scaleway"), "click");
await settle();
out.aCheckingKeyIsPressed = {
  open: !theKeyList().hidden,
  asked: invoked.some((one) => one.command === "the_key_a_dialog_holds"),
};
fire(theRowReading("RunPod"), "click");
await settle();
out.picked = {
  dialog: theBox() !== null,
  open: !theKeyList().hidden,
  face: flat(theFace()),
  asked: invoked.filter((one) => one.command === "the_key_a_dialog_holds").map((one) => one.key),
  keys: (globalThis.picks || []).length,
};
out.requiredKey.dropdown = keyErrorReads();
fire(add(), "click");
await settle();
out.requiredKey.submitted = invoked.some((one) => one.command === "a_server_is_added" && one.keyIdentifier === ids.runpod);
fire(theFace(), "click");
out.reopened = theRows();
fire(theFace(), "keydown", { key: "Escape" });
await settle();
out.escaped = { dialog: theBox() !== null, open: !theKeyList().hidden };
fire(theFace(), "click");
fire(theRowReading("Open Keys"), "click");
await settle();
out.openKeys = { picks: (globalThis.picks || []).slice(), dialog: theBox() !== null, open: !theKeyList().hidden };
invoked = [];
globalThis.handBack(null);
await settle();
out.askedAgain = invoked.filter((one) => one.command === "the_keys_a_dialog_offers").length;

const openKeys = async () => {
  fire(theFace(), "click");
  fire(theRowReading("Open Keys"), "click");
  await settle();
};
// A key removed while away leaves the field empty. Neither Cancel nor a
// returned row that no longer exists may clear the required-key error.
await openKeys();
globalThis.handBack({ row: "removed-key" });
await settle();
fire(add(), "click");
await settle();
await openKeys();
globalThis.handBack(null);
await settle();
out.requiredKey.cancel = keyErrorReads();
await openKeys();
globalThis.handBack({ row: "removed-key" });
await settle();
out.requiredKey.gone = keyErrorReads();
await openKeys();
globalThis.handBack({ row: ids.runpod });
await settle();
out.requiredKey.keys = keyErrorReads();
// New key uses the same return path, with its own dialog open on Keys.
await openKeys();
globalThis.handBack({ row: "removed-key" });
await settle();
fire(add(), "click");
await settle();
fire(all(theBox()).find((one) => one.tag === "button" && text(one) === theAddDialog.new_key), "click");
await settle();
globalThis.handBack({ row: ids.runpod });
await settle();
out.requiredKey.newKey = keyErrorReads();

// Edit, on a server whose own key is gone.
fire(theBox(), "keydown", { key: "Escape" });
await settle();
const theRow = () => all(host).find((one) => one.classList && one.classList.contains("svrow"));
const theMenu = () => all(host).find((one) => one.classList && one.classList.contains("svctx"));
fire(theRow(), "contextmenu");
fire(all(theMenu()).find((one) => one.tag === "button" && text(one) === "Edit…"), "click");
await settle();
out.editClosed = theFace() ? flat(theFace()) : null;
fire(theFace(), "click");
verdict(ids.hetzner, "unusable");
verdict(ids.runpod, "usable");
verdict(ids.first, "usable");
verdict(ids.second, "unusable");
verdict(ids.scaleway, "notChecked");
await settle();
out.editRows = theRows();
invoked = [];
fire(theRowReading("Hetzner"), "click");
await settle();
out.theGoneKeyIsPressed = {
  open: !theKeyList().hidden,
  asked: invoked.some((one) => one.command === "the_key_a_dialog_holds"),
};
console.log(JSON.stringify(out));
"#;

    /// **The port reads from the left at the number field's width, and
    /// `.w-frame` is untouched** — the drawing's `sv-add` and `sv-edit` (P2,
    /// the Creator's note 2 on 0.1.7). **Both dialogs draw the port through
    /// the one class**, the sheet's rule names the element and both classes so
    /// it outranks `.w-frame` whichever sheet loads last, and `.w-frame` keeps
    /// its right alignment for every frame-number field. Measured once at the
    /// desk in a headless browser over the three sheets: the port *left, 58px*
    /// and a Render Settings frame field *right, 58px*
    /// (`Audits/Build/Step 10c/The Creator's notes piece 1 — what landed.md`).
    #[test]
    fn the_port_reads_from_the_left_and_the_frame_fields_do_not_move() {
        let page = include_str!("../../web/servers.js");
        assert_eq!(page.matches("port.classList.add(THE_PORTS_OWN_CLASS);").count(), 2, "the Add and the Edit dialog");
        assert!(page.contains("const THE_PORTS_OWN_CLASS = \"svport\";"));
        let sheet = include_str!("../../web/servers.css");
        assert!(sheet.contains("input.fld.svport { text-align: left; }"));
        let drawing = include_str!("../../web/the-drawing.css");
        assert!(drawing.contains(".w-frame { width: 58px; text-align: right; }"));
    }

    /// **The Creator's note 3 on 0.1.7, and it runs the page code: *Add and set
    /// up* lands on the new server's Set up tab, and a plain add stays on the
    /// list.** `web/servers.js` runs under Node beside the stand-ins above; the
    /// add's answers are the application's own (`the_add_is_answered`).
    /// **Before the piece** the answer named no server and the page stayed on
    /// All servers either way. Skipped, and says so, where no Node is
    /// installed.
    #[test]
    fn the_page_opens_the_new_server_on_its_set_up_tab_only_where_a_setup_was_ticked() {
        let as_the_page_hears = |event: Event| {
            let printed = format!("{event:?}");
            let mut out = serde_json::Map::new();
            for name in ["what", "server", "itIsSetUp"] {
                let key = format!("\"{name}\": ");
                if let Some(at) = printed.find(&key) {
                    let rest = &printed[at + key.len()..];
                    let opens = rest.find("(\"").expect("a said value") + 2;
                    let closes = opens + rest[opens..].find("\")").expect("it closes");
                    out.insert(name.to_string(), serde_json::Value::String(rest[opens..closes].to_string()));
                }
            }
            serde_json::Value::Object(out).to_string()
        };
        let ticked = as_the_page_hears(the_add_is_answered(TheAnswer::ItWasSaved, Some("server-7")));
        let plain = as_the_page_hears(the_add_is_answered(TheAnswer::ItWasSaved, None));
        let folder = std::env::temp_dir().join(format!("smoozy-servers-add-lands-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/servers.js");
        std::fs::copy(&page, folder.join("servers.js")).expect("the page is copied");
        for (name, text) in THE_STAND_INS {
            let text = match name {
                "harness.js" => text
                    .split("globalThis.screen.itIsDrawn(host);")
                    .next()
                    .expect("the harness's head")
                    .to_string()
                    + &format!(
                        r#"globalThis.screen.itIsDrawn(host);
await settle();
tell("servers:the-answer", {ticked});
await settle();
const afterTicked = {{ openedOn: globalThis.openedOn || [], went: globalThis.went || [] }};
tell("servers:the-answer", {plain});
await settle();
console.log(JSON.stringify({{ afterTicked, afterPlain: {{ openedOn: globalThis.openedOn || [], went: globalThis.went || [] }} }}));"#
                    ),
                "the-rail.js" => r#"export const ALL_SERVERS = "serversview";
export const A_SERVER = "machineview";
export const KEYS = "keysview";
export function aScreenIsRegistered(screen) { globalThis.screen = screen; }
export function theCurrentPlace() { return ALL_SERVERS; }
export function theCurrentPlaceIs(place, which) { (globalThis.went ||= []).push([place, which]); }
export function theRailIsReadAgain() {}"#
                    .to_string(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the server list's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let out: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let landed = serde_json::json!({
            "openedOn": [["server-7", "setupview"]],
            "went": [["machineview", "server-7"]],
        });
        assert_eq!(out["afterTicked"], landed, "{out}");
        assert_eq!(out["afterPlain"], landed, "a plain add went somewhere: {out}");
    }

    // ── The Creator's notes on 0.1.8, piece 5: *Rename…* on a server ────────

    /// **A rename saves the name alone, through the edit's own save** — the
    /// page sends the saved host, port, username and key exactly as the Edit
    /// dialog's reading gives them, with the typed name (`Design/Servers §
    /// Actions on a row`). Nothing else on the row moves, nothing Edit throws
    /// away goes, `Servers` W34's warning has nothing to say (`Servers` W37), an emptied
    /// name clears the name, and a server whose key row is gone renames too.
    #[test]
    fn a_rename_saves_the_name_alone_and_raises_no_warning() {

        let started = a_started("rename");
        let connection = started.record.as_ref().expect("a record").connection();
        let key = a_key_row(Some("SHA256:first"));
        key_rows::add(connection, &key).expect("a key row");
        the_add_is_performed(&started, &an_add(&key.identifier.as_text()));
        let mut row = the_rows_of(&started).remove(0);
        row.machine_password_subject = Some(Subject::issued_by_the_local_record(&row.local_server_identifier));
        row.remembered_hardware_reading = Some(a_reading());
        server_rows::write(connection, &row).expect("written");
        // **The row as the record holds it**, which is what a rename is
        // compared against.
        let row = the_rows_of(&started).remove(0);

        let renamed = |row: &ServerRow, name: &str, store: &AStandInStore| {
            let rows = the_rows_of(&started);
            let keys = key_rows::all(connection).expect("the keys read");
            let dialog = the_edit_dialog_from(row, &rows, &keys, None);
            let said = |value: &Verbatim| value.as_str().to_string();
            assert_eq!(
                the_edit_warning_as_typed(
                    Some(connection),
                    row,
                    &said(&dialog.the_host),
                    &said(&dialog.the_port),
                    &said(&dialog.the_username),
                ),
                None,
                "a rename raised Edit's warning"
            );
            the_edit_is_performed(
                &started,
                Some(store),
                &row.local_server_identifier,
                &said(&dialog.the_host),
                &said(&dialog.the_port),
                &said(&dialog.the_username),
                &said(&dialog.the_key_identifier),
                name, None,
            )
        };

        let store = AStandInStore::answering(Ok(()));
        assert_eq!(renamed(&row, "RunPod kitchen", &store), TheAnswer::ItWasSaved);
        let mut wanted = row.clone();
        wanted.label = Some("RunPod kitchen".to_string());
        assert_eq!(the_rows_of(&started)[0], wanted, "a rename moved something besides the name");
        assert_eq!(store.erased(), 0, "a rename erased the remembered password");

        // **An emptied name clears it**, as clearing it in Edit does.
        assert_eq!(renamed(&wanted, "", &store), TheAnswer::ItWasSaved);
        assert_eq!(the_rows_of(&started)[0], row, "an emptied name did not clear the name alone");

        // **A server whose key row is gone renames too** — *Needs a new key*
        // offers *Rename…* like every other shape.
        key_rows::remove(connection, &key.identifier).expect("the key row goes");
        assert_eq!(renamed(&row, "Vast spare", &store), TheAnswer::ItWasSaved);
        assert_eq!(the_rows_of(&started)[0].label.as_deref(), Some("Vast spare"));
        assert_eq!(the_rows_of(&started)[0].key, row.key, "the gone key was let go of");
    }

    /// **The page renames in place, on the row** (`Design/Servers § Actions on
    /// a row`, the drawing's `sv-rename`), run under Node over the
    /// application's own screen, Edit reading and check answer: *Rename…*
    /// sits first in slot 4; it turns the name into a field holding it,
    /// selected; a press in the field does not open the server; typing runs
    /// the add's check on the saved details and draws its duplicate-name line
    /// under the field; a redraw keeps the field, what is typed and the keys;
    /// Escape saves nothing and puts the name back; Return sends the name with
    /// the saved details unchanged; an emptied name is sent empty. **Skipped,
    /// and says so, where no Node is installed.**
    #[test]
    fn the_page_renames_a_server_in_place_on_its_row() {
        let (mut row, register) = a_connected_server();
        row.label = Some("RunPod 4090".to_string());
        let mut other = a_row();
        other.label = Some("Hyperstack A6000".to_string());
        let drawn = the_screen_from(std::slice::from_ref(&row), &[], None, Some(&register), &Default::default());
        let dialog = the_edit_dialog_from(&row, std::slice::from_ref(&row), &[], None);
        let checked = |label: &str| {
            let event = the_checks_on(std::slice::from_ref(&other), row.host.as_str(), &row.port.number().to_string(), label);
            let mut said = serde_json::Map::new();
            for (name, text) in the_pairs_of(&event) {
                said.insert(name, serde_json::Value::String(text));
            }
            serde_json::Value::Object(said).to_string()
        };
        let harness = THE_RENAME_HARNESS
            .replace("THE_SCREEN", &serde_json::to_string(&drawn).expect("the screen serialises"))
            .replace("THE_DIALOG", &serde_json::to_string(&dialog).expect("the reading serialises"))
            .replace("THE_SAME_NAME", &checked("Hyperstack A6000"))
            .replace("THE_NAME_FREE", &checked("RunPod kitchen"));
        let folder = std::env::temp_dir().join(format!("smoozy-servers-rename-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/servers.js");
        std::fs::copy(&page, folder.join("servers.js")).expect("the page is copied");
        for (name, text) in THE_STAND_INS {
            let text = match name {
                "harness.js" => harness.clone(),
                "the-rail.js" => r#"export const ALL_SERVERS = "serversview";
export const A_SERVER = "machineview";
export const KEYS = "keysview";
export function aScreenIsRegistered(screen) { globalThis.screen = screen; }
export function theCurrentPlace() { return ALL_SERVERS; }
export function theCurrentPlaceIs(place, which) { (globalThis.went ||= []).push([place, which]); }
export function theRailIsReadAgain() {}"#
                    .to_string(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the server list's page was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let saved = |label: &str| {
            serde_json::json!({
                "command": "an_edit_is_saved",
                "server": row.local_server_identifier,
                "host": "213.181.122.44",
                "port": "40122",
                "username": "root",
                "keyIdentifier": "key-1",
                "label": label,
            })
        };
        let wanted = serde_json::json!({
            "slotFour": ["Rename…", "Edit…", "Copy address"],
            "started": { "value": "RunPod 4090", "focused": true, "selected": true, "asked": "the_edit_dialog" },
            "wentOnAPress": 0,
            "checked": {
                "command": "the_edit_is_checked",
                "server": row.local_server_identifier,
                "host": "213.181.122.44",
                "port": "40122",
                "username": "root",
                "label": "Hyperstack A6000",
            },
            "sameName": "Another server is already called Hyperstack A6000. In the sidebar, only the badge tells them apart.",
            "nameFree": null,
            "afterARedraw": { "value": "Hyperstack A6000", "focused": true },
            "removalOnBackspace": false,
            "escaped": { "field": false, "saved": false },
            "saved": [saved("RunPod kitchen")],
            "afterTheAnswer": { "field": false },
            "emptied": [saved("")],
        });
        assert_eq!(got, wanted);
    }

    /// **The scenario for [`the_page_renames_a_server_in_place_on_its_row`]**: a
    /// document whose events bubble, as a browser's do, so a press in the
    /// field reaches the row unless the field stops it.
    const THE_RENAME_HARNESS: &str = r#"// **A browser lets go of the keys when the element holding them leaves the
// page**, and this document does the same.
function letGo(gone) {
  if (gone && gone.contains && gone.contains(document.activeElement)) document.activeElement = null;
}
function anElement(tag) {
  const names = new Set();
  const listeners = {};
  const element = {
    tag, children: [], parentNode: null, dataset: {}, hidden: false, text: "", value: "", type: "", tabIndex: 0,
    disabled: false, selected: false, listeners,
    style: { setProperty() {} },
    classList: {
      add: (...more) => more.forEach((one) => names.add(one)),
      remove: (...less) => less.forEach((one) => names.delete(one)),
      toggle: (one, on) => { if (on === undefined ? !names.has(one) : on) names.add(one); else names.delete(one); },
      contains: (one) => names.has(one),
    },
    set className(value) { names.clear(); String(value).split(" ").filter(Boolean).forEach((one) => names.add(one)); },
    get className() { return Array.from(names).join(" "); },
    set textContent(value) { for (const one of element.children) { letGo(one); one.parentNode = null; } element.children = []; element.text = String(value); },
    get textContent() { return element.text; },
    get childNodes() { return element.children; },
    get childElementCount() { return element.children.length; },
    appendChild(child) {
      if (child.parentNode && child.parentNode.removeChild) child.parentNode.removeChild(child);
      child.parentNode = element;
      element.children.push(child);
      return child;
    },
    insertBefore(child) { return element.appendChild(child); },
    removeChild(child) { letGo(child); element.children = element.children.filter((one) => one !== child); child.parentNode = null; return child; },
    replaceChild(fresh, old) {
      if (fresh.parentNode && fresh.parentNode !== element) fresh.parentNode.removeChild(fresh);
      letGo(old);
      element.children = element.children.map((one) => (one === old ? fresh : one));
      fresh.parentNode = element;
      old.parentNode = null;
      return old;
    },
    replaceChildren(...fresh) { for (const one of element.children) { letGo(one); one.parentNode = null; } element.children = []; fresh.forEach((one) => element.appendChild(one)); },
    setAttribute() {}, removeAttribute() {}, getAttribute() { return null; },
    focus() { document.activeElement = element; },
    select() { element.selected = true; },
    addEventListener(kind, handler) { (listeners[kind] = listeners[kind] || []).push(handler); },
    closest(selector) {
      const wanted = selector.replace(/^\./, "");
      for (let at = element; at; at = at.parentNode) if (at.classList && at.classList.contains(wanted)) return at;
      return null;
    },
    contains(other) { for (let at = other; at; at = at.parentNode) if (at === element) return true; return false; },
    querySelector() { return null; }, querySelectorAll() { return []; },
    getBoundingClientRect() { return { left: 0, bottom: 0, width: 0, height: 0 }; },
  };
  return element;
}
function fire(element, kind, extra) {
  let stopped = false;
  let prevented = false;
  const event = {
    target: element, key: extra && extra.key, clientX: 0, clientY: 0, button: 0,
    stopPropagation() { stopped = true; },
    preventDefault() { prevented = true; },
    get defaultPrevented() { return prevented; },
  };
  for (let at = element; at && !stopped; at = at.parentNode) {
    for (const one of (at.listeners && at.listeners[kind]) || []) one(event);
  }
}
function all(element, found = []) {
  found.push(element);
  for (const child of element.children || []) all(child, found);
  return found;
}
function text(element) {
  return (element.text || "") + (element.children || []).map(text).join("");
}
globalThis.told = new Map();
globalThis.document = {
  createElement: anElement,
  createTextNode: (text) => ({ text, children: [] }),
  addEventListener() {}, removeEventListener() {},
  activeElement: null,
};
globalThis.window = { innerWidth: 1000, innerHeight: 800 };
const theScreen = THE_SCREEN;
const theDialog = THE_DIALOG;
let invoked = [];
window.__TAURI__ = {
  core: {
    invoke(command, args) {
      invoked.push({ command, ...(args || {}) });
      if (command === "the_all_servers_screen") return Promise.resolve(theScreen);
      if (command === "the_edit_dialog") return Promise.resolve(theDialog);
      return Promise.resolve(null);
    },
  },
};
const settle = async () => { for (let at = 0; at < 4; at += 1) await new Promise((done) => setImmediate(done)); };
const tell = (kind, said) => (globalThis.told.get(kind) || []).forEach((one) => one({ kind, said }));
await import("./servers.js");
const host = anElement("div");
globalThis.screen.itIsDrawn(host);
await settle();
const theRow = () => all(host).find((one) => one.classList && one.classList.contains("svrow"));
const theField = () => all(theRow()).find((one) => one.tag === "input") || null;
const theMenu = () => all(host).find((one) => one.classList && one.classList.contains("svctx"));
const renameIsPressed = async () => {
  fire(theRow(), "contextmenu");
  const item = all(theMenu()).find((one) => one.tag === "button" && text(one) === "Rename…");
  if (!item) return false;
  fire(item, "click");
  await settle();
  return true;
};
const typed = async (value) => { theField().value = value; fire(theField(), "input"); await settle(); };
const out = {};
fire(theRow(), "contextmenu");
const slots = [];
let slot = [];
for (const one of theMenu().children) {
  if (one.tag === "hr") { slots.push(slot); slot = []; } else slot.push(text(one).replace(/\u00A0/g, " "));
}
slots.push(slot);
// **Slot 4 is the one before *Remove***: an empty slot is skipped, not padded.
out.slotFour = slots[slots.length - 2];
invoked = [];
const pressed = await renameIsPressed();
out.started = pressed && theField()
  ? { value: theField().value, focused: document.activeElement === theField(), selected: theField().selected, asked: invoked[0].command }
  : null;
if (!out.started) { console.log(JSON.stringify(out)); process.exit(0); }
fire(theField(), "click");
out.wentOnAPress = (globalThis.went || []).length;
invoked = [];
await typed("Hyperstack A6000");
out.checked = invoked.find((one) => one.command === "the_edit_is_checked") || null;
const theLineUnder = () => {
  const line = all(theRow()).find((one) => one.classList && one.classList.contains("svextra") && !one.hidden);
  return line ? text(line).replace(/\u00A0/g, " ") : null;
};
tell("servers:the-answer", THE_SAME_NAME);
await settle();
out.sameName = theLineUnder();
tell("servers:what-is-observed", {});
await settle();
out.afterARedraw = theField() ? { value: theField().value, focused: document.activeElement === theField() } : null;
tell("servers:the-answer", THE_NAME_FREE);
await settle();
out.nameFree = theLineUnder();
invoked = [];
fire(theField(), "keydown", { key: "Backspace" });
await settle();
out.removalOnBackspace = invoked.some((one) => one.command === "the_removal_of");
invoked = [];
fire(theField(), "keydown", { key: "Escape" });
await settle();
out.escaped = { field: Boolean(theField()), saved: invoked.some((one) => one.command === "an_edit_is_saved") };
await renameIsPressed();
await typed("RunPod kitchen");
invoked = [];
fire(theField(), "keydown", { key: "Enter" });
await settle();
out.saved = invoked.filter((one) => one.command === "an_edit_is_saved");
tell("servers:the-answer", { what: "it-was-saved" });
await settle();
out.afterTheAnswer = { field: Boolean(theField()) };
await renameIsPressed();
await typed("");
invoked = [];
fire(theField(), "keydown", { key: "Enter" });
await settle();
out.emptied = invoked.filter((one) => one.command === "an_edit_is_saved");
console.log(JSON.stringify(out));
"#;

    /// The stand-ins the New key harness loads the real `web/servers.js` and
    /// `web/keys.js` beside. **The rail mounts a screen the way
    /// `web/the-rail.js` does** — drawn on its first showing, put away when
    /// another is shown — which is what the New key dialog's opening rests on.
    const THE_KEY_CONTROLS_STAND_INS: [(&str, &str); 8] = [
        ("package.json", r#"{ "type": "module" }"#),
        (
            "a-server.js",
            r#"export const SET_UP = "setupview";
export function theServerWillOpenOn() {}
export function theServersAreAskedForAgain() {}"#,
        ),
        (
            "event-route.js",
            r#"export function whenTold(kind, handler) {
  const held = globalThis.told.get(kind) || [];
  held.push(handler);
  globalThis.told.set(kind, held);
}"#,
        ),
        (
            "the-rail.js",
            r#"export const ALL_SERVERS = "serversview";
export const A_SERVER = "machineview";
export const KEYS = "keysview";
const screens = new Map();
let current = null;
export function aScreenIsRegistered(screen) { screens.set(screen.place, screen); }
export function theCurrentPlace() { return current; }
export function theCurrentPlaceIs(place) {
  if (current === place) return;
  const leaving = screens.get(current);
  if (leaving && leaving.itIsPutAway) leaving.itIsPutAway();
  current = place;
  const screen = screens.get(place);
  if (!screen) return;
  if (!screen.host) { screen.host = document.createElement("div"); screen.itIsDrawn(screen.host); }
  if (screen.itIsCurrentNow) screen.itIsCurrentNow();
}
export function theRailIsReadAgain() {}
globalThis.theScreens = screens;"#,
        ),
        (
            "the-failure-shape.js",
            r#"export const EXPANDED = "expanded";
export const A_COLLAPSED_ROW = "a-collapsed-row";
export function theControlsFrom() { return []; }
export function theFailureShape() {
  const drawn = document.createElement("span");
  drawn.classList.add("afailure");
  return drawn;
}"#,
        ),
        (
            "the-machines-own-words.js",
            r#"export function theWordsPanel() { return { hidden: true }; }"#,
        ),
        (
            "the-underway-shape.js",
            r#"export function theCountdown() { return { nodeType: 1 }; }
export function theMarkOnAControl(control) { control.classList.add("uway"); return control; }
export function theMarkComesOffAControl(control) { control.classList.remove("uway"); return control; }"#,
        ),
        (
            "harness.js",
            r#"function anElement(tag) {
  const names = new Set();
  const listeners = new Map();
  const element = {
    tag, children: [], parentNode: null, dataset: {}, hidden: false, text: "",
    disabled: false, value: "", type: "", id: "", placeholder: "", tabIndex: 0,
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
    get textContent() { return element.text; },
    get childNodes() { return element.children; },
    get childElementCount() { return element.children.length; },
    appendChild(child) { child.parentNode = element; element.children.push(child); return child; },
    insertBefore(child) { return element.appendChild(child); },
    removeChild(child) { element.children = element.children.filter((one) => one !== child); return child; },
    replaceChild(fresh, old) {
      element.children = element.children.map((one) => (one === old ? fresh : one));
      fresh.parentNode = element;
      return old;
    },
    replaceChildren(...fresh) { element.children = []; fresh.forEach((one) => element.appendChild(one)); },
    setAttribute() {}, removeAttribute() {}, focus() {},
    addEventListener(kind, handler) {
      const held = listeners.get(kind) || [];
      held.push(handler);
      listeners.set(kind, held);
    },
    click() {
      if (element.disabled) return;
      for (const one of listeners.get("click") || []) one({ target: element, preventDefault() {} });
    },
    querySelector() { return null; }, querySelectorAll() { return []; }, closest() { return null; },
    contains() { return false; },
    getBoundingClientRect() { return { left: 0, bottom: 0, width: 0, height: 0 }; },
  };
  return element;
}
function findAll(element, fits, found = []) {
  if (element && element.classList && fits(element)) found.push(element);
  for (const child of (element && element.children) || []) findAll(child, fits, found);
  return found;
}
const theButtonReading = (host, words) =>
  findAll(host, (one) => one.tag === "button" && one.textContent === words)[0];
globalThis.told = new Map();
globalThis.document = {
  createElement: anElement,
  createTextNode: (text) => ({ text, children: [] }),
  addEventListener() {}, removeEventListener() {}, querySelector() { return null; },
};
globalThis.window = { innerWidth: 1000, innerHeight: 800 };
const words = (lead) => ({ lead, values: [], rest: null, the_rest_joins: false });
const theScreen = {
  the_screens_own_name: "", add_a_server: "ADD A SERVER", the_tools_statement: null,
  rows: [],
  footer: { how_many: words(""), how_many_connected: words("") },
  guidance: { heading: "", how_it_goes: "", steps: [], open_keys: "", add_a_server: "" },
  menus: [], the_subject_kind: "server row",
};
const theDialog = THE_ADD_DIALOG;
const theKeyList = THE_KEY_LIST;
window.__TAURI__ = {
  core: {
    invoke(command) {
      if (command === "the_all_servers_screen") return Promise.resolve(theScreen);
      if (command === "the_add_dialog") return Promise.resolve(theDialog);
      if (command === "the_keys_a_dialog_offers") return Promise.resolve(theKeyList);
      return Promise.resolve(null);
    },
  },
};
const settle = () => new Promise((done) => setImmediate(done));
const rail = await import("./the-rail.js");
await import("./servers.js");
rail.theCurrentPlaceIs(rail.ALL_SERVERS);
await settle();
const servers = globalThis.theScreens.get(rail.ALL_SERVERS).host;
const theDialogIsOpened = async () => {
  theButtonReading(servers, theScreen.add_a_server).click();
  await settle();
};
// The New key dialog is the first box the Keys screen builds.
const theNewKeyDialogShows = () => {
  const keys = globalThis.theScreens.get(rail.KEYS).host;
  return keys ? !findAll(keys, (one) => one.classList.contains("modal"))[0].hidden : false;
};
const out = {};

// 1. The dialog's *New key…*.
await theDialogIsOpened();
const port = findAll(servers, (one) => one.tag === "input" && one.value === theDialog.the_port_starts_at)[0];
out.thePortsClasses = port ? port.className : "";
theButtonReading(servers, theDialog.new_key).click();
await settle();
out.afterNewKey = { place: rail.theCurrentPlace(), theNewKeyDialogShows: theNewKeyDialogShows() };

// 2. A cancel on Keys hands back nothing and the dialog stands as it was.
const keys = globalThis.theScreens.get(rail.KEYS).host;
const feet = findAll(keys, (one) => one.classList.contains("svfoot"));
feet[feet.length - 1].children.find((one) => one.tag === "button").click();
await settle();
out.afterCancel = {
  place: rail.theCurrentPlace(),
  theNewKeyDialogShows: theNewKeyDialogShows(),
  theKeyFieldReads: theButtonReading(servers, theDialog.choose_a_key) ? "choose" : "something else",
};

// 3. The key field opens Smoozy's own list, and its last row, *Open Keys…*,
// opens Keys in its picking mode alone, as the field itself did until piece E3.
theButtonReading(servers, theDialog.choose_a_key).click();
await settle();
theButtonReading(servers, theKeyList.open_keys).click();
await settle();
out.afterTheKeyField = { place: rail.theCurrentPlace(), theNewKeyDialogShows: theNewKeyDialogShows() };
console.log(JSON.stringify(out));"#,
        ),
    ];

    /// **The Creator's second notes, piece 15: what a pasted line fills the
    /// dialog with, and the line under Host** — `Design/Servers § Adding a
    /// server`, `Servers` §13.3. A line missing a piece names it; a line
    /// with no port fills 22 and says nothing; Runpod's proxied line fills
    /// nothing and says why; a text that is not a line fills nothing.
    #[test]
    fn a_pasted_line_fills_the_three_fields_and_names_what_it_lacks() {
        let read = the_pasted_line_from;
        let filled = |host: &str, port: &str, username: &str, under: &[&'static str]| ThePastedLine {
            fills: true,
            host: Verbatim::new(host),
            port: Verbatim::new(port),
            username: Verbatim::new(username),
            the_line_under: under.iter().map(|said| Prose::new(*said)).collect(),
        };
        assert_eq!(read("ssh -p 1239 ubuntu@1.2.3.4"), filled("1.2.3.4", "1239", "ubuntu", &[]));
        assert_eq!(read("ssh root@host"), filled("host", "22", "root", &[]));
        assert_eq!(read("ssh -i ~/.ssh/k -L 8080:localhost:8080 -p 2 u@h"), filled("h", "2", "u", &[]));
        assert_eq!(read("ssh host"), filled("host", "22", "", &[words::NO_USERNAME_IN_THAT_LINE]));
        assert_eq!(
            read("ssh -p 41966"),
            filled("", "41966", "", &[words::NO_HOST_IN_THAT_LINE, words::NO_USERNAME_IN_THAT_LINE])
        );
        let nothing = |under: Vec<Prose>| ThePastedLine {
            fills: false,
            host: Verbatim::new(""),
            port: Verbatim::new(""),
            username: Verbatim::new(""),
            the_line_under: under,
        };
        assert_eq!(
            read("ssh 8y5rumuyb50m78-6441103b@ssh.runpod.io -i ~/.ssh/id_ed25519"),
            nothing(vec![Prose::new(words::RUNPODS_PROXIED_LINE)])
        );
        assert_eq!(read("203.0.113.7"), nothing(Vec::new()));
        // **The dialog says the paste works.**
        assert_eq!(the_add_dialog_from(&[]).host_placeholder, Prose::new(words::HOST_OR_PASTE));
    }

    /// **The Creator's second notes, piece 15, and the third notes' note 1, and
    /// it runs the page code: the Add dialog opens with the caret in Host,
    /// Escape still closes it, and an ssh line pasted into Host fills the three
    /// fields** — `Design/Whole app` (*"A box that asks for typing opens with
    /// the caret in its first field … Escape still closes it"*) and
    /// `Design/Servers § Adding a server`. The answers are the application's
    /// own ([`the_pasted_line_from`]). **Before the piece** the focus went to
    /// the box itself, and a pasted line stayed in Host whole. Skipped, and
    /// says so, where no Node is installed.
    #[test]
    fn the_add_dialog_opens_in_host_and_a_pasted_line_fills_its_fields() {
        const LINES: [&str; 6] = [
            "ssh -p 1239 ubuntu@1.2.3.4",
            "ssh root@host",
            "ssh -i ~/.ssh/k -L 8080:localhost:8080 -p 2 u@h",
            "ssh host",
            "ssh 8y5rumuyb50m78-6441103b@ssh.runpod.io -i ~/.ssh/id_ed25519",
            "  ssh root@213.173.108.12 -p 17445 -i ~/.ssh/id_ed25519\n",
        ];
        let drawn = the_screen_from(&[], &[], None, None, &Default::default());
        let dialog = the_add_dialog_from(&[]);
        let mut answers = serde_json::Map::new();
        for line in LINES {
            answers.insert(line.to_string(), serde_json::to_value(the_pasted_line_from(line)).expect("it serialises"));
        }
        let prefix = THE_RENAME_HARNESS
            .split("const theScreen = THE_SCREEN;")
            .next()
            .expect("the harness's document");
        let harness = format!("{prefix}{THE_PASTE_SCENARIO}")
            .replace("THE_SCREEN", &serde_json::to_string(&drawn).expect("the screen serialises"))
            .replace("THE_DIALOG", &serde_json::to_string(&dialog).expect("the dialog serialises"))
            .replace("THE_ANSWERS", &serde_json::Value::Object(answers).to_string())
            .replace("THE_LINES", &serde_json::to_string(&LINES).expect("the lines serialise"));
        let folder = std::env::temp_dir().join(format!("smoozy-servers-paste-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        let page = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/servers.js");
        std::fs::copy(&page, folder.join("servers.js")).expect("the page is copied");
        for (name, text) in THE_STAND_INS {
            let text = match name {
                "harness.js" => harness.clone(),
                "the-rail.js" => r#"export const ALL_SERVERS = "serversview";
export const A_SERVER = "machineview";
export const KEYS = "keysview";
export function aScreenIsRegistered(screen) { globalThis.screen = screen; }
export function theCurrentPlace() { return ALL_SERVERS; }
export function theCurrentPlaceIs(place, which) { (globalThis.went ||= []).push([place, which]); }
export function theRailIsReadAgain() {}"#
                    .to_string(),
                _ => text.to_string(),
            };
            std::fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let ran = std::process::Command::new("node").arg(folder.join("harness.js")).current_dir(&folder).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the Add dialog's paste was not run");
            let _ = std::fs::remove_dir_all(&folder);
            return;
        };
        let _ = std::fs::remove_dir_all(&folder);
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness answers");
        let said = |text: &'static str| serde_json::to_value(Prose::new(text)).expect("it serialises");
        let pasted = |host: &str, port: &str, username: &str, under: serde_json::Value| {
            serde_json::json!({ "taken": true, "host": host, "port": port, "username": username, "under": under })
        };
        let wanted = serde_json::json!({
            "opened": { "theCaretIsInHost": true, "placeholder": said(words::HOST_OR_PASTE) },
            "escapeCloses": true,
            "pasted": [
                pasted("1.2.3.4", "1239", "ubuntu", serde_json::Value::Null),
                pasted("host", "22", "root", serde_json::Value::Null),
                pasted("h", "2", "u", serde_json::Value::Null),
                pasted("host", "22", "", said(words::NO_USERNAME_IN_THAT_LINE)),
                // Runpod's proxied line leaves every field as it was.
                pasted("host", "22", "", said(words::RUNPODS_PROXIED_LINE)),
                pasted("213.173.108.12", "17445", "root", serde_json::Value::Null),
            ],
            "theChecksRanOnTheFilledFields": { "command": "the_add_is_checked", "host": "213.173.108.12", "port": "17445", "label": "" },
            "anAddressIsAnAddress": { "taken": false, "asked": false },
            "typingInHostTakesTheLineAway": null,
            "nothingWasSaved": true,
        });
        assert_eq!(got, wanted);
    }

    /// **The scenario for
    /// [`the_add_dialog_opens_in_host_and_a_pasted_line_fills_its_fields`]**,
    /// run on the rename harness's document, whose events bubble as a
    /// browser's do. A paste carries its text the way a browser's does, and
    /// whether the page took it is whether it stopped the browser's own.
    const THE_PASTE_SCENARIO: &str = r#"const theScreen = THE_SCREEN;
const theDialog = THE_DIALOG;
const theAnswers = THE_ANSWERS;
const theLines = THE_LINES;
let invoked = [];
window.__TAURI__ = {
  core: {
    invoke(command, args) {
      invoked.push({ command, ...(args || {}) });
      if (command === "the_all_servers_screen") return Promise.resolve(theScreen);
      if (command === "the_add_dialog") return Promise.resolve(theDialog);
      if (command === "the_pasted_line_is_read") return Promise.resolve(theAnswers[args.line]);
      return Promise.resolve(null);
    },
  },
};
const settle = async () => { for (let at = 0; at < 4; at += 1) await new Promise((done) => setImmediate(done)); };
await import("./servers.js");
const host = anElement("div");
globalThis.screen.itIsDrawn(host);
await settle();
const theBox = () => all(host).find((one) => one.classList && one.classList.contains("mbox")) || null;
const theAddIsPressed = async () => {
  fire(all(host).find((one) => one.tag === "button" && text(one) === theScreen.add_a_server), "click");
  await settle();
};
const theFields = () => all(theBox()).filter((one) => one.tag === "input");
const theLineUnder = () => {
  const line = all(theBox()).find((one) => one.classList && one.classList.contains("svpasted") && !one.hidden);
  return line ? text(line) : null;
};
function paste(field, words) {
  let prevented = false;
  const event = {
    target: field,
    clipboardData: { getData: (kind) => (kind === "text/plain" || kind === "text" ? words : "") },
    stopPropagation() {}, preventDefault() { prevented = true; },
    get defaultPrevented() { return prevented; },
  };
  for (let at = field; at; at = at.parentNode) {
    for (const one of (at.listeners && at.listeners.paste) || []) one(event);
  }
  return prevented;
}
const out = {};
await theAddIsPressed();
out.opened = { theCaretIsInHost: document.activeElement === theFields()[0], placeholder: theFields()[0].placeholder };
fire(theFields()[0], "keydown", { key: "Escape" });
await settle();
out.escapeCloses = theBox() === null;
await theAddIsPressed();
out.pasted = [];
for (const line of theLines) {
  const [hostField, port, username] = theFields();
  invoked = [];
  const taken = paste(hostField, line);
  await settle();
  out.pasted.push({ taken, host: hostField.value, port: port.value, username: username.value, under: theLineUnder() });
}
out.theChecksRanOnTheFilledFields = invoked.find((one) => one.command === "the_add_is_checked") || null;
invoked = [];
const taken = paste(theFields()[0], "203.0.113.7");
await settle();
out.anAddressIsAnAddress = { taken, asked: invoked.some((one) => one.command === "the_pasted_line_is_read") };
paste(theFields()[0], "ssh host");
await settle();
theFields()[0].value = "host2";
fire(theFields()[0], "input");
await settle();
out.typingInHostTakesTheLineAway = theLineUnder();
out.nothingWasSaved = !invoked.some((one) => one.command === "a_server_is_added");
console.log(JSON.stringify(out));
"#;

    /// **Step 11's D6b: removing a server that is not connected draws History
    /// again at once**, its jobs moved into *Removed servers* — the eighteenth
    /// notes' piece 7 found them moving only at History's next draw, since no
    /// connection changed. Both removals the user can press end in
    /// [`a_removal_lands`].
    #[test]
    fn d6b_removing_a_server_draws_history_again_at_once() {
        use std::sync::Arc;

        let started = a_started("d6b-removal-history");
        let connection = started.record.as_ref().expect("a record").connection();
        let key = a_key_row(Some("SHA256:d6b"));
        key_rows::add(connection, &key).expect("a key row");
        the_add_is_performed(&started, &an_add(&key.identifier.as_text()));
        let row = the_rows_of(&started).remove(0);
        let server = row.local_server_identifier.clone();
        let identity = Identity::read_back(IdentityKind::LocalServer, &server).expect("an identity");
        let mut job = crate::record::deletions::tests::a_record_in_history(&identity, "Kitchen");
        job.what_the_note_carried.ending = Some(smoozy_queue_history::acting::note::TheEndingOnTheNote {
            outcome: smoozy_queue_history::acting::note::TheOutcomeAsWritten::Finished,
            exit_status: Some(0),
            the_attempt_whose_capture_record_holds_the_words: 1,
            the_reason_the_machine_recorded: None,
            the_device_line_as_observed: None,
        });
        crate::record::job_records::JobRecords::over(connection)
            .write_the_provisional_record(&job)
            .expect("a job in history");

        struct Held(Arc<Mutex<Vec<crate::events::Delivery>>>);
        impl crate::events::Delivers for Held {
            fn deliver(&self, delivery: &crate::events::Delivery) {
                if let Ok(mut held) = self.0.lock() {
                    held.push(delivery.clone());
                }
            }
        }
        let held = Arc::new(Mutex::new(Vec::new()));
        let queue = TheOneQueue::new();
        queue.the_page_is_ready(Box::new(Held(held.clone())));
        let not_connected = crate::surface::the_history::WhatHistoryIsDrawnOver::default();
        let answer = a_removal_lands(&started, None, Some(&queue), None, &server, false, Some(&not_connected));
        assert_eq!(answer, TheAnswer::ItWasSaved);
        let held = held.lock().expect("the deliveries");
        let history: Vec<_> = held
            .iter()
            .filter(|one| one.kind == crate::surface::the_history::THE_HISTORY_SAYS)
            .collect();
        assert_eq!(history.len(), 1, "History was not drawn again when the server was removed");
        let drawn = history[0];
        assert!(drawn.said.contains_key("hrow0Removed"), "the job is not in Removed servers: {:?}", drawn.said.keys());
        assert!(drawn.said.contains_key("theRemovedServers"));
    }
}
