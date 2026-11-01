//! What happens at application start, before the window is built — `Desktop
//! app and delivery` L3a, L6 and `Reporting a problem` §2 and §9.
//!
//! **Four acts, in this order, and the order is each clause's own.**
//!
//! **Before any of them, the launch has taken the one running copy's hold** —
//! P2, in `main.rs`, before the shell is built, through
//! [`crate::the_one_instance`]. A launch that finds another copy holding it
//! never reaches this file, so it performs no part of `Reporting a problem`
//! U3 step 1: no sweep, no record, no read or write of the run-mark. Added at
//! step 11's R1.
//!
//! 1. **The two throwaway areas are swept** (L3a, `Security and licence` §7) —
//!    *"anything found in it at application start is removed before any module
//!    uses it"*.
//! 2. **The local record is opened, which runs its migrations** (L6) —
//!    *"a migration runs at application start, before any module reads the
//!    record"*. **What the last run's downloads left is settled straight
//!    after**, before anything can admit a transfer — `Server protocol and
//!    transfers` S28's second bullet and C13, side by side as `Download` R6
//!    orders ([`what_the_last_run_left_is_settled`]).
//! 3. **The run-mark is read and then written** (`State, identity and
//!    recovery` I2) — read at application start before any feature module
//!    composes, then written durably as *(running, this run's identity)*.
//! 4. **The diary's session line is written** (`Reporting a problem` U7) —
//!    one per run, opening it, carrying the run identity, the moment on the
//!    desktop clock, and Smoozy's own version and which system it runs on.
//!
//! **How a clause marker is written in this crate, and it is a rule rather than
//! a habit.** A marker with no document beside it means a clause of this crate's
//! own plan, `Preparation/Architecture/Desktop app and delivery.md`. Every other
//! document's marker is written with that document — at the site, or in this
//! file's own header list below. **Where a marker is a clause of this crate's
//! own plan *and* of the document meant, the header list is not enough and the
//! document is written out at the site**, because a bare one there resolves to a
//! real and wrong clause.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `State, identity and recovery` D6. `Reporting a
//! problem` J2, J4a, U7.
//!
//! **A marker this crate's own plan also defines is deliberately not listed
//! above, whichever document is meant, and is written out with its document
//! at every site instead** — the header list cannot tell the two apart, and a
//! bare one would resolve to a real and wrong clause.
//!
//! **Nothing here stops the window from opening.** Rule 8 and step 1's own
//! finding both bind: a window that never appears is Smoozy going quiet,
//! whatever the trouble. So every act reports what it came to and the start
//! carries on.

use std::time::SystemTime;

use smoozy_reporting::lines::{Declarations, Line, LineKind};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::Clock;
use smoozy_state::run_mark::{what_the_launch_found, RunMark, RunState, WhatTheLaunchFound};

use crate::diary_service::{DiaryService, Wrote};
use crate::platform;
use crate::presentation::declarations::{Declared, TheDeclarations};
use crate::presentation::said_once::TheThingsSaidOnce;
use crate::record::LocalRecord;

/// What the start came to. **Every field is an outcome and none of them is a
/// reason to stop**, which is why this is a report rather than a result.
pub struct Started {
    /// L3a's sweep, and what it could not clear.
    pub the_sweep_left_behind: Vec<platform::files::Failure>,
    /// The record, where it could be opened. **An absent record is not a
    /// silent one**: L8a's rule is that a screen shows the failure rather than
    /// an emptied version of itself, and a caller holding `None` here has that
    /// failure to show.
    pub record: Option<LocalRecord>,
    /// Why the record could not be opened, in the machine's own terms.
    pub record_trouble: Option<String>,
    /// The diary, where its area could be made.
    pub diary: Option<DiaryService>,
    /// **Every feature module's diary declaration, registered at start.**
    ///
    /// `Reporting a problem` J2 and J4a declare per module and never at run
    /// time, and **`Desktop app and delivery` A7** makes **a module with no
    /// declaration write nothing, silently** — a hosting fault rather than a
    /// default, because a module
    /// writing nowhere satisfies its own spec and leaves a hole the Creator
    /// cannot tell from a thing that did not happen. **Nothing called
    /// `module_declares` at all**, so every declaration written in a feature
    /// module was a declaration only its own tests had seen.
    pub declarations: Declarations<&'static str>,
    /// **What each module declares that this document's services need** — §4
    /// A7, through [`crate::presentation::declarations`].
    ///
    /// **It is the diary declaration above's sibling and is filled the same
    /// way**, from each module's own crate, at start, before anything reads it.
    /// A7's hosting rule holds on both: **a module with no declaration answers
    /// *no* to everything**, which is why nothing here has a default and why a
    /// suppression written for a module this register does not carry is
    /// refused rather than written.
    pub what_modules_declare: TheDeclarations,
    /// **What the operating system has already been told about in this run** —
    /// §14 Q13.
    ///
    /// Run-held (`State, identity and recovery` §11.1), written nowhere, and
    /// its whole job is that a repeat resolving to a standing condition
    /// identity is met with silence rather than with a second notification.
    pub said_once: TheThingsSaidOnce,
    /// This run's identity, where the system supplied the random bytes.
    /// **`State, identity and recovery` Y5: random bytes**, issued before any
    /// feature module composes, and **an identity is never composed from
    /// anything else** — so a machine that will not supply them leaves this
    /// absent rather than filled with something that looks like one.
    pub run: Option<Identity>,
    /// What the launch found in the run-mark — `Reporting a problem` §9's one
    /// reader, and the whole of what it is read for.
    pub the_previous_run: WhatTheLaunchFound,
}

/// **The kind of the one event the spine posts, declared beside the service
/// that posts it.**
///
/// The command surface's own rule ([`crate::surface`]) puts a service's event
/// kinds in that service's file. The spine has no command of its own, so its
/// kind is here, which is the same rule applied to a service the page does not
/// call.
pub const THE_RUN_BEGAN: &str = "spine:the-run-began";

pub(crate) fn retry_the_owed_bot_keys_at_start_with(
    connection: &rusqlite::Connection,
    retry: impl FnOnce(&rusqlite::Connection),
) {
    use smoozy_security_and_licence::erasure::EraseMarks;
    let marks = crate::record::told_record::TheBotTokensMarks(connection);
    if marks.marks().is_ok_and(|owed| !owed.is_empty()) {
        retry(connection);
    }
}

/// Run the four acts.
pub fn start(queue: &crate::events::TheOneQueue) -> Started {
    // 1. The sweep. **It is the area's act rather than the writing module's**,
    //    because *discarded at once* does not cover a process that exits first.
    let the_sweep_left_behind = platform::sweep_the_throwaway_areas();
    // **And, beside the two areas, a throwaway copy's own items in the secret
    //   store** — see [`the_development_items_are_swept`]. Only a throwaway
    //   copy whose folder holds no record yet sweeps them; in an ordinary copy
    //   it does nothing, and in a release build it is not compiled at all.
    //   **It is asked before the record is opened**, because opening makes one.
    #[cfg(debug_assertions)]
    the_development_items_are_swept();

    // 2. The record, whose opening is what runs the migrations.
    let (record, record_trouble) = match LocalRecord::open() {
        Ok(record) => (Some(record), None),
        Err(failure) => (None, Some(format!("{failure:?}"))),
    };

    // **What the last run's downloads left, settled before any transfer is
    // admitted** — `Server protocol and transfers` S28's second bullet and C13,
    // side by side as `Download` R6 orders. Nothing admits a transfer until the
    // window is built, which is after this returns. **What it came to is not
    // carried further**: what it could not do stays on the record — an owed
    // cleanup, a publication still pending — and the next start asks again.
    if let Some(record) = record.as_ref() {
        let _ = what_the_last_run_left_is_settled(record);
    }

    // **`State, identity and recovery` R18: the age sweep at application
    // start** — the one moment certain to come where the application is never
    // left running long enough to cross the thirty days. The 38's piece 3.
    if let Some(record) = record.as_ref() {
        let _ = crate::the_previews::the_age_sweep(
            record.connection(),
            smoozy_state::lifetimes::SweepMoment::ApplicationStart,
            SystemTime::now(),
        );
    }

    // Fifteenth notes, note 2 again: an old bot key is retried quietly
    // on startup, without changing the new bot's saved marker or reporting.
    if let Some(record) = record.as_ref() {
        retry_the_owed_bot_keys_at_start_with(record.connection(), |connection| {
            if let Some(store) = crate::surface::telegram::the_store_for_this_system() {
                crate::record::told_record::retry_the_owed_bot_keys(connection, store.as_ref());
            }
        });
    }

    // The migration records the retired key's erase once. A failure leaves
    // its debt standing, and a settled debt never opens the store again.
    if let Some(record) = record.as_ref() {
        crate::record::key_rows::with_legacy_licence_erases_owed(record.connection(), || {
            if let Some(store) = crate::surface::keys::the_store_for_this_system() {
                crate::record::key_rows::retry_the_legacy_licence_erases(record.connection(), store.as_ref());
            }
        });
    }

    // The run identity. A machine with no randomness cannot issue one, and
    // §2.1 forbids composing one from anything else — so it stays absent and
    // is reported, which is still not a reason for the window not to open.
    let run = Identity::issue(IdentityKind::Run).ok();

    // 3. The run-mark: read what was there, then write this run's.
    let the_previous_run = the_mark_is_read_and_replaced(record.as_ref(), run.as_ref());

    // The declarations, before any line is written. **A module with no
    // declaration writes nothing, silently** (`Desktop app and delivery` A7),
    // so this is where a feature
    // module's own declaration stops being a constant its tests read and
    // becomes the thing the service checks.
    let declarations = the_declarations();
    // **§4 A7's other four declarations, on the route piece C built.**
    // Same moment, same rule, same reason: a module with no
    // declaration answers *no* to everything, and that is a hosting
    // fault rather than a default this document supplies.
    let what_modules_declare = what_the_modules_declare();

    // 4. The diary's session line — **and the first event this application has
    //    ever put on the interface's queue**, which is step 10a piece A's own
    //    proof. See [`the_run_began`] for the whole of it.
    let diary = DiaryService::open().ok();
    if let (Some(diary), Some(run)) = (diary.as_ref(), run.as_ref()) {
        the_run_began(queue, diary, run.clone());
    }

    Started {
        the_sweep_left_behind,
        record,
        record_trouble,
        diary,
        declarations,
        what_modules_declare,
        said_once: TheThingsSaidOnce::new(),
        run,
        the_previous_run,

    }
}

/// **A throwaway copy's own secret-store items, swept at start — the third
/// throwaway thing, beside the two areas — and only where its folder holds no
/// record yet.**
///
/// **Why it is here.** The throwaway launch setting redirects the record and
/// the home folder, which are two calls in one file, and it cannot redirect the
/// operating system's secret store, which `Desktop app and delivery` F11 does
/// not resolve to a path at all. What a throwaway copy gets instead is a
/// development mark in every item's subject
/// ([`crate::platform::secrets::THE_DEVELOPMENT_MARK`]), and this is what makes
/// that mark worth having: **a fresh throwaway copy never inherits the last
/// one's secrets.** The run that wrote them is gone and cannot clear up after
/// itself, which is the same reason L3a puts the two areas' sweep here.
///
/// **A throwaway folder that already holds a record sweeps nothing, at start or
/// at quit.** Changed 2026-10-17, the Creator's eleventh notes, note 2. The
/// Creator's test copy keeps its folder across launches, and this sweep ran at
/// every start and every quit: after a relaunch its record still
/// said *token stored, Telegram on* while the keychain no longer held the
/// token, and a new server could not be given Telegram until the same token
/// was pasted again. Its key passphrases and machine passwords went the same
/// way. **A record kept is secrets kept**, so the two are swept together or not
/// at all: the secrets go only with a folder that is new. *(Until that day a
/// second sweep ran when a throwaway copy was asked to close — step 10b's first
/// audit round, item 11 — for copies whose folder an agent deletes after use.
/// It is gone: such a copy's items are now swept by the next fresh throwaway
/// copy, which `crate::platform::locations` already tells an agent to clear
/// before it reports.)*
///
/// **An ordinary copy sweeps nothing**, and a release build does not compile
/// this at all.
#[cfg(debug_assertions)]
fn the_development_items_are_swept() {
    if !crate::platform::locations::this_copy_is_a_throwaway_one() {
        return;
    }
    // **The record's own place in this copy's folder, through F11's one
    // funnel.** A folder whose record cannot be named sweeps nothing: the
    // secrets are kept rather than taken on a guess.
    let Ok(the_record) =
        crate::platform::locations::resolve(crate::platform::locations::Location::LocalRecord)
    else {
        return;
    };
    if !a_throwaway_folder_is_swept_at_start(&the_record) {
        return;
    }
    // A store this computer has nowhere to remember on has nothing to sweep.
    let Some(store) = crate::surface::keys::the_store_for_this_system() else {
        return;
    };
    let _ = crate::platform::secrets::sweep_the_development_items(
        store.as_ref(),
        std::time::Duration::from_secs(5),
    );
}

/// **Whether a throwaway copy's start sweeps its development items: only where
/// nothing stands at its record's place yet.** A record there, or a place that
/// could not be looked at, keeps them.
#[cfg(debug_assertions)]
fn a_throwaway_folder_is_swept_at_start(the_record: &std::path::Path) -> bool {
    matches!(
        crate::platform::files::outside_program_wrote(the_record),
        crate::platform::files::Recheck::Absent
    )
}

/// **Every feature module's diary declaration, in one place, registered at
/// start.**
///
/// The names are the module names the plans use. **A module is added here when
/// its own declaration lands**, and a module absent from this list writes
/// nothing — which is **`Desktop app and delivery` A7**'s rule and is why the
/// list is here rather than left to each module to register itself.
pub(crate) fn the_declarations() -> Declarations<&'static str> {
    let mut declarations = Declarations::new();
    declarations.module_declares("Keys", smoozy_keys::diary::DECLARED.to_vec());
    // `Telegram` §19: acts, and no told lines. Step 10 piece 4.
    declarations.module_declares("Telegram", crate::record::telegram_bot::DECLARED.to_vec());
    // **`Servers` §12 and `Server Setup` §12, registered 2026-09-22.** Both
    // declarations had been written in their own crates and registered by
    // nothing, so `Desktop app and delivery` A7's hosting rule was refusing
    // every line either module composed — which is why step 10c's fourth pass
    // on Vast left a diary holding three lines after a whole pass on a real
    // rental.
    declarations.module_declares(
        crate::servers::published::THE_MODULE,
        smoozy_servers::diary::DECLARED.to_vec(),
    );
    declarations.module_declares(
        crate::setting_up::THE_MODULE,
        smoozy_server_setup::diary::DECLARED.to_vec(),
    );
    // **`Render Settings` §12, registered 2026-09-29**: one act line, *a
    // `.blend` read*, and no told lines. Until *Read the file* worked
    // (step 10c's dependency wave piece 3) nothing composed that line; once it
    // did, every stored reading's line was refused here for want of this one.
    declarations.module_declares(
        crate::reading_the_file::THE_MODULE,
        smoozy_render_settings::diary::DECLARED.to_vec(),
    );
    // **`File Browser` §11.1, registered 2026-10-02**: act lines, and no told
    // lines (`File Browser` B5). Nothing composed one until step 10c's Files'
    // owed acts piece 3 performed the first of that module's acts on the
    // machine (`File Browser` B3).
    declarations.module_declares(
        crate::managing_files::THE_MODULE,
        smoozy_file_browser::diary::DECLARED.to_vec(),
    );
    // **`Queue & History` §17, registered 2026-10-16** (owed piece 11): act
    // lines and the one told line. Nothing composed one until *Track it*
    // adopted work (owed piece 10), whose *untracked work taken over* line was
    // refused here for want of this one.
    declarations.module_declares(
        crate::servers::the_machines_work::THE_QUEUES_MODULE,
        smoozy_queue_history::diary::DECLARED.to_vec(),
    );
    // **`Reporting a problem` §11's own declaration, registered at step 11's
    // piece R4**: one act line, *a report written*, and no told lines. The act
    // belongs to no feature module — its two ways in are this document's menu
    // item and failure plumbing — so that document declares it itself, and
    // until this line every report the box wrote would have had its line
    // refused here.
    declarations.module_declares(
        crate::surface::reporting::THE_MODULE,
        vec![LineKind::Act],
    );
    declarations
}

/// **Every feature module's §4 A7 declaration that this document's services
/// read, in one place, registered at start.**
///
/// **This is the first writer to [`crate::presentation::declarations`]**, which
/// piece C built with nothing filling it. The rule is the diary list above's,
/// word for word: **a module is added here when its own declaration lands, and
/// a module absent from this list answers *no* to everything** — A7's *a
/// declaration missing is a module that cannot be hosted, not a default this
/// document supplies*.
///
/// **The declaration itself lives in the owning module's own crate** and is
/// read here rather than restated: `File Browser` §11's own
/// `THE_SUPPRESSION_ROWS` and `THE_PARTITION_UNITS` are its, and a copy of
/// either here would be the fact in two places.
///
/// **`Rendering` W4a's `rendering.missing-assets` is registered here, and the
/// ruling that once kept it out is spent.** ~~This crate does not name
/// `smoozy-rendering`, and naming it would pull `Rented-server runtime`,
/// `tokio` and the whole machine-side tree into the desktop application to read
/// one string.~~ **Struck 2026-09-17**: step 10a piece 8 put
/// `smoozy-rendering` in this crate's own dependency list for the submission
/// path, so the cost that ruling weighed is already paid and reading the row
/// adds nothing. **The condition the ruling named has also been met** — the
/// piece that builds that warning's own band is built, and
/// [`crate::surface::render_settings::the_missing_assets_warning_is_switched_off`]
/// is the caller that needs the row. Until this line existed that switch-off
/// was refused at the declaration check (G5), so the user ticked
/// *Don't warn me about this again* and the band came back next time.
fn what_the_modules_declare() -> TheDeclarations {
    let mut declarations = TheDeclarations::new();
    declarations.module_declares(
        "File Browser",
        Declared {
            // **`File Browser` M3's one dismissible warning**, read from that
            // module's own declaration.
            dismissible_warnings: smoozy_file_browser::declarations::THE_SUPPRESSION_ROWS.to_vec(),
            // **No partition unit is registered here, and that is M27 kept
            // rather than skipped.** M27's own words are that *"which kind an
            // action is comes from its own document's declaration and is never
            // decided at the moment of drawing it"*, and `File Browser` §11's
            // table declares each action's unit and its kind **in prose** and
            // declares no *the unit is the whole action* bit. Deriving one by
            // reading that prose would be a model, and *"an action whose
            // enumeration happens to hold one unit today is not the same thing
            // as an action that is atomic entire"* is the sentence that model
            // would get wrong. **What is owed is the bit, in that module's own
            // declaration**, and it belongs to whoever builds M27's drawing.
            partition_units: Vec::new(),
            // **No condition of `File Browser`'s is declared here yet**, and a
            // condition with no named supplier must not be raised
            // (`State, identity and recovery` F12). That module raises none
            // through this route today.
            conditions: Vec::new(),
        },
    );
    declarations.module_declares(
        // **`Rendering` W4a's one suppression identity**, read from that
        // module's own declaration — the module name and the identity are both
        // that crate's `THE_SUPPRESSION_ROW`, so neither is restated here.
        smoozy_rendering::warning::THE_SUPPRESSION_ROW.0,
        Declared {
            dismissible_warnings: vec![smoozy_rendering::warning::THE_SUPPRESSION_ROW.1],
            // **`Rendering` declares no partition unit through this route.**
            // M27's bit comes from a module's own declaration and is never
            // derived here; that document declares none.
            partition_units: Vec::new(),
            // **And no condition** — `State, identity and recovery` F12: a
            // condition with no named supplier must not be raised.
            conditions: Vec::new(),
        },
    );
    // **`Servers`' one condition — `State, identity and recovery` §8.3's *the
    // server connection*.** It is read from
    // [`crate::servers::published`], beside the raiser, so the four columns
    // F12 requires and the values the raise hands the store are the same
    // constants. **Without this line the raise is refused at the declaration
    // check** and every connection condition would be a record nobody could
    // raise — which is A7's hosting rule working, not a default.
    declarations.module_declares(
        crate::servers::published::THE_MODULE,
        crate::servers::published::what_servers_declares(),
    );
    // **`State, identity and recovery` §8.3's *the supervisor unavailable
    // while SSH is live*, under the supplier that row names** — raised on
    // `State, identity and recovery` Y9's fourth branch by [`crate::servers::published`] (step 10b's first
    // audit round, item 8). Declared here the way `Servers`' own is, for the
    // same reason.
    declarations.module_declares(
        crate::servers::published::THE_SUPERVISORS_SUPPLIER,
        crate::servers::published::what_the_supervisors_supplier_declares(),
    );
    declarations
}

/// **The session line goes down, and then the interface is told the run
/// began** — `Desktop app and delivery` P7, in the order the clause states.
///
/// **This is the first event this application has ever posted**, and it is
/// piece A's own proof at the desk: a fact the spine makes true, written
/// durably, put on the one ordered queue, and drawn by the page.
///
/// **The two halves of P7 are both here and neither is a convention.** The
/// write is the queue's own first argument, so it runs first and the event is
/// composed only from what it returned; and **the event is not posted at all
/// where the line did not go down**. That second half is worth stating plainly,
/// because it costs something: a computer whose diary area cannot be written
/// draws nothing here, and the pane stays as empty as it was before this piece
/// existed. **That is P7 rather than an oversight** — an event saying the run
/// began, with nothing durable saying so, is exactly the state the clause is
/// written to forbid. What says it out loud to the user is the failure shape,
/// which is step 10a piece C's.
///
/// **L16's answer is used rather than dropped**, and this
/// is not the fallback record that clause warns against: nothing is recorded
/// anywhere else and nothing is retried. The answer decides whether a volatile
/// event is posted, which is the one thing P7 gives a caller to do with it.
fn the_run_began(queue: &crate::events::TheOneQueue, diary: &DiaryService, run: Identity) {
    use crate::events::{Event, Said, Subject};
    use crate::typography::{Prose, Verbatim};

    let line = session_line(run);
    let posted: Result<_, String> = queue.write_then_post(
        || match diary.write(&line) {
            Wrote::Yes => Ok(()),
            Wrote::No { reported } => Err(reported),
        },
        |()| {
            Event::about(Subject::TheRun, THE_RUN_BEGAN)
                // Prose, so it goes through the typography point — A6 and X1.
                // One sentence handed over as one string, which is X10's rule
                // and what keeps the orphan binding true inside it.
                .saying(
                    "sentence",
                    Said::Prose(Prose::new("Smoozy started and is ready.")),
                )
                // A value, so it goes by the verbatim route — X7 and X9.
                // A version is a thing the user copies into a report, and a
                // substituted character travelling inside one is a value
                // Smoozy quietly changed.
                .saying(
                    "version",
                    Said::Value(Verbatim::new(env!("CARGO_PKG_VERSION"))),
                )
        },
    );
    // **Nothing is owed either way** — L16. The line is
    // in the diary or it is lost, and the event followed it or it did not.
    let _ = posted;
}

/// `Reporting a problem` U7's session line: the run identity, the moment on the
/// desktop clock, and **Smoozy's own version and which operating system it runs
/// on — exactly `Desktop app and delivery` E11's two values and nothing more.**
///
/// **One thing is added in a development build and only where this copy is a
/// throwaway one: that it is one, and where it is pointed.** A copy that quietly
/// points somewhere else is a copy an agent can mistake for the real one, and
/// the diary is the one place every run says what it was. **U7's two values are
/// exactly the two values in every build that goes out**, because the whole of
/// the throwaway setting is behind `#[cfg(debug_assertions)]` — so the clause is
/// kept structurally rather than by anybody remembering, and the test below
/// counts the fields for both cases.
fn session_line(run: Identity) -> Line {
    // **The `mut` is the throwaway copy's and nothing else's**, so a release
    // build, which compiles none of that, has nothing to add to the line.
    #[cfg_attr(not(debug_assertions), allow(unused_mut))]
    let mut text = format!(
        "session\t{}\t{}",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS
    );
    #[cfg(debug_assertions)]
    if let Some(folder) = crate::platform::locations::the_throwaway_folder() {
        text.push_str(&format!(
            "\tthrowaway copy, pointed at {}",
            folder.display()
        ));
    }
    Line {
        run: run.clone(),
        kind: LineKind::Session,
        at: std::time::SystemTime::now(),
        clock: Clock::ThisComputer,
        text,
        concerning: vec![run],
    }
}

/// What the start's settling of the last run's downloads came to.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WhatTheLastRunLeft {
    /// Staging objects removed on `Server protocol and transfers` S28's exact
    /// match.
    pub removed: usize,
    /// Removals tried and not confirmed — each an owed cleanup on its record
    /// (`Server protocol and transfers` S27).
    pub owed: usize,
    /// Objects left where they stand because the record could not claim them.
    pub left_alone: usize,
    /// Reservations C13 completed.
    pub publications_completed: usize,
    /// Reservations C13 removed.
    pub reservations_removed: usize,
    /// Publication-pending records that could not be settled and still stand.
    pub still_pending: usize,
    /// Why the record could not be read or written, where it could not.
    pub trouble: Option<String>,
}

/// **What the last run's downloads left, settled at application start and
/// before any transfer is admitted** — `Server protocol and transfers` S28's
/// second bullet and this plan's C13, run side by side as `Download` R6 orders.
/// Added 2026-09-28, step 10c's transfer wave piece 2.
///
/// **Both acts were written, tested and called by nothing.** `Server protocol
/// and transfers` S28's second bullet puts the desktop-side sweep here in so many
/// words — *"the desktop-side sweep runs at application start, before any
/// transfer is admitted"* — because a download's staging sits in the user's own
/// folder and no connection to any server reaches it once that server is gone.
/// Without it a part-file whose record was ended *stopped*, *failed* or
/// *stalled*, and whose removal did not go through or never ran, stayed in the
/// folder the user chose, for good. **C13's settling mattered more than its
/// window is wide**: a reservation left *publication pending* made `Download`'s
/// own admission refuse every download of that server, for good, and say
/// nothing.
///
/// **C13 first, then the sweep**, so a publication completed here is a
/// published record by the time the sweep reads it, and the sweep derives
/// nothing from its live path.
///
/// **What the sweep may touch is `Server protocol and transfers` S28's and
/// nothing else's**: a path derived
/// from a record in a removable state — the live attempt's where the ending is
/// one, every superseded attempt's always — removed only where the object's
/// stable identity now matches the one recorded for that attempt. **Nothing is
/// found by looking in a folder** (`Server protocol and transfers` S29), and
/// *interrupted* is never on the list (`Server protocol and transfers` S28a):
/// that material is the leftover the user is promised.
///
/// **An owed cleanup is cleared only on a confirmed removal** (`Server protocol
/// and transfers` S27): every derived path removed or already gone. A removal
/// that did not go through, or a path that could not be looked at, leaves the
/// debt standing for the next start. A path the record cannot claim — another
/// identity there, or none ever recorded — is left alone and leaves the debt as
/// it was.
///
/// **Nothing here stops the window from opening**, the start's own rule: a
/// record that will not read or write is reported and the start carries on.
pub(crate) fn what_the_last_run_left_is_settled(record: &LocalRecord) -> WhatTheLastRunLeft {
    use crate::platform::collision::{
        settle_the_reservation_of, the_record_once_settled, the_reservations_this_run_owes,
        WhatTheNextRunDoes,
    };
    use crate::record::transfer_records;
    use crate::transfers::ThePlatformFileAdapter;
    use smoozy_server_protocol::transfers::cleanup::{
        what_the_sweep_derives, what_the_sweep_does, WhatTheSweepDoes,
    };
    use smoozy_server_protocol::transfers::moving::TheUsersOwnComputer;

    let mut left = WhatTheLastRunLeft::default();
    let connection = record.connection();
    let mut records = match transfer_records::the_downloads_the_start_settles(connection) {
        Ok(records) => records,
        Err(failure) => {
            left.trouble = Some(format!("{failure:?}"));
            return left;
        }
    };
    let mut adapter = ThePlatformFileAdapter;
    let keep_the_trouble = |left: &mut WhatTheLastRunLeft, failure: crate::record::RecordFailure| {
        left.trouble.get_or_insert(format!("{failure:?}"));
    };

    // **C13.** Each reservation the last run owes is completed or removed, and
    // the record then says so.
    let owed: Vec<usize> = the_reservations_this_run_owes(&records)
        .into_iter()
        .filter_map(|owed| records.iter().position(|one| one.transfer == owed.transfer))
        .collect();
    for at in owed {
        let decided = settle_the_reservation_of(&records[at]);
        let the_final_object_now = adapter.read_the_final_object(&records[at].final_path);
        match the_record_once_settled(&records[at], decided, &the_final_object_now) {
            Some(settled) => match transfer_records::put(connection, &settled) {
                Ok(()) => {
                    match decided {
                        WhatTheNextRunDoes::CompleteThePublication => {
                            left.publications_completed += 1
                        }
                        WhatTheNextRunDoes::RemoveTheReservation => left.reservations_removed += 1,
                        _ => {}
                    }
                    records[at] = settled;
                }
                Err(failure) => {
                    keep_the_trouble(&mut left, failure);
                    left.still_pending += 1;
                }
            },
            None => left.still_pending += 1,
        }
    }

    // **`Server protocol and transfers` S28's desktop-side sweep.**
    for one in &records {
        let mut not_confirmed = false;
        let mut not_claimed = false;
        for derived in what_the_sweep_derives(std::slice::from_ref(one)) {
            let at = derived.path.as_text();
            // **A path that cannot be looked at is not a path that is gone**,
            // so it is read by kind rather than by `exists`, which answers
            // *no* to both.
            let identity_now = match std::fs::symlink_metadata(&at) {
                Ok(_) => Some(adapter.stable_identity_of(&at)),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
                Err(_) => {
                    not_confirmed = true;
                    continue;
                }
            };
            match what_the_sweep_does(&derived, identity_now.as_ref()) {
                WhatTheSweepDoes::Remove => match adapter.remove(&at) {
                    Ok(()) => left.removed += 1,
                    Err(_) => {
                        left.owed += 1;
                        not_confirmed = true;
                    }
                },
                WhatTheSweepDoes::NothingIsThere => {}
                WhatTheSweepDoes::LeaveItAndReportAnOwnershipMismatch
                | WhatTheSweepDoes::LeaveItBecauseTheIdentityWasNeverRecorded => {
                    left.left_alone += 1;
                    not_claimed = true;
                }
            }
        }
        // **`Server protocol and transfers` S27, in both directions.**
        let owed_now = if not_confirmed {
            true
        } else if not_claimed {
            one.cleanup_is_owed
        } else {
            false
        };
        if owed_now != one.cleanup_is_owed {
            let mut moved = one.clone();
            moved.cleanup_is_owed = owed_now;
            if let Err(failure) = transfer_records::put(connection, &moved) {
                keep_the_trouble(&mut left, failure);
            }
        }
    }
    left
}

/// `Reporting a problem` U3: the launch reads the old mark, then replaces it.
/// Keeping this one act callable also lets tests use a record of their own.
fn the_mark_is_read_and_replaced(record: Option<&LocalRecord>, run: Option<&Identity>) -> WhatTheLaunchFound {
    let previous = record.and_then(|record| read_run_mark(record).ok()).flatten();
    if let (Some(record), Some(run)) = (record, run) {
        // Without the overwrite, `Reporting a problem` C4's once-rule could not hold. Do not offer a
        // crash when this launch cannot establish the mark's required cycle.
        if write_run_mark(record, &RunMark::this_run_is_running(run.clone())).is_err() {
            return WhatTheLaunchFound::NoPreviousRun;
        }
    } else { return WhatTheLaunchFound::NoPreviousRun; }
    what_the_launch_found(previous)
}

/// `State, identity and recovery` I2's read. **The one durable value on this
/// side whose reading at launch takes a branch.**
fn read_run_mark(record: &LocalRecord) -> Result<Option<RunMark>, crate::record::RecordFailure> {
    let found: Option<(String, String)> = record
        .connection()
        .query_row(
            "SELECT state, written_by FROM run_mark WHERE only_row = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .ok();

    Ok(found.and_then(|(state, written_by)| {
        Some(RunMark {
            state: match state.as_str() {
                "clean" => RunState::Clean,
                "running" => RunState::Running,
                _ => return None,
            },
            // A value that will not read back is not an identity, and §2.1 rule 2
            // forbids inventing one — so a damaged mark reads as no mark, which
            // `Reporting a problem` §9 already handles as a fresh installation.
            written_by: Identity::read_back(IdentityKind::Run, &written_by)?,
        })
    }))
}

/// `State, identity and recovery` I2's write. **Written durably**, which for
/// this record is what [`crate::record`]'s own three engine settings mean.
pub fn write_run_mark(
    record: &LocalRecord,
    mark: &RunMark,
) -> Result<(), crate::record::RecordFailure> {
    let state = match mark.state {
        RunState::Running => "running",
        RunState::Clean => "clean",
    };
    record.connection().execute(
        "INSERT INTO run_mark (only_row, state, written_by) VALUES (1, ?1, ?2)
         ON CONFLICT(only_row) DO UPDATE SET state = ?1, written_by = ?2",
        rusqlite::params![state, mark.written_by.as_text()],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The Creator's eleventh notes, note 2: a throwaway copy
    /// sweeps nothing when it is asked to close.** Asserted over `src/main.rs`'s
    /// own text, because the close routes are the shell's and are reached only
    /// in a running window: neither way out — the window's close nor the run's
    /// own exit — reaches the sweep, and the start is the one place it is
    /// called. *(Until 2026-10-17 both ways out took it — step 10b's first
    /// audit round, item 11 — and the Creator's test copy forgot its stored
    /// token at every quit while its record still said *stored*.)*
    #[test]
    fn a_throwaway_copy_sweeps_nothing_when_asked_to_close() {
        let main = include_str!("main.rs");
        assert!(
            !main.contains("a_throwaway_copy_is_closing"),
            "a way out of the application still sweeps a throwaway copy's \
             development items"
        );
        assert!(!main.contains("the_development_items_are_swept"));
        assert!(!main.contains("sweep_the_development_items"));
        let working = include_str!("start.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the working half");
        assert!(
            !working.contains("pub fn a_throwaway_copy_is_closing()"),
            "the sweep at close is still offered to the shell"
        );
        // **One call, and it is the start's.**
        assert_eq!(working.matches("the_development_items_are_swept();").count(), 1);
        let the_start = working
            .find("pub fn start(")
            .expect("the start");
        let the_call = working
            .find("the_development_items_are_swept();")
            .expect("the call");
        let the_record_opens = working
            .find("LocalRecord::open()")
            .expect("the record's opening");
        assert!(
            the_start < the_call && the_call < the_record_opens,
            "the sweep is asked before the record is opened, inside the start"
        );
    }

    /// **The Creator's eleventh notes, note 2: a throwaway folder
    /// that already holds a record sweeps nothing at start, and a fresh one
    /// still does.** The record is a real one, opened where this copy's own
    /// record would sit under the folder (F11's composition), so *holds a
    /// record* is the thing a relaunch meets and not a stand-in file.
    #[cfg(debug_assertions)]
    #[test]
    fn a_throwaway_folder_holding_a_record_keeps_its_secrets_and_a_fresh_one_sweeps() {
        let mut folder = std::env::temp_dir();
        folder.push(format!(
            "smoozy-e2-a-throwaway-folder-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&folder);
        let root = crate::platform::locations::throwaway::the_application_data_root_under(&folder);
        std::fs::create_dir_all(&root).unwrap();
        let the_record = root.join("local-record.sqlite3");

        // **A fresh folder: nothing at the record's place, so it sweeps.**
        assert!(
            a_throwaway_folder_is_swept_at_start(&the_record),
            "a fresh throwaway folder no longer sweeps the last copy's secrets"
        );

        // **A relaunch: the record the first launch made is there.**
        let record = LocalRecord::open_at(&the_record).expect("the record opens");
        drop(record);
        assert!(the_record.exists());
        assert!(
            !a_throwaway_folder_is_swept_at_start(&the_record),
            "a throwaway folder holding a record swept its own secrets at start"
        );

        let _ = std::fs::remove_dir_all(&folder);
    }

    /// **`Reporting a problem` §11's own declaration is registered at
    /// start**, so the box's *report written* line is not refused by
    /// `Desktop app and delivery` A7's hosting rule. Step 11's piece R4.
    #[test]
    fn the_report_written_is_declared_at_start_and_nothing_else_is() {
        // **`Reporting a problem` §11: one act, *a report written*, and no told
        // lines** — the absence declared so that it is a decision. The act
        // passes `Reporting a problem` J2a's three tests and is not caught by
        // its bar.
        let declarations = the_declarations();
        let module = crate::surface::reporting::THE_MODULE;
        assert!(declarations.may_write(&module, LineKind::Act));
        for kind in LineKind::ALL {
            if kind != LineKind::Act {
                assert!(!declarations.may_write(&module, kind), "{kind:?}");
            }
        }
        assert!(smoozy_reporting::report_file::A_REPORT_WRITTEN
            .is_diary_worthy(smoozy_reporting::report_file::A_REPORT_WRITTEN_IS_NO_ERASURE));
    }

    /// **`Desktop app and delivery` A7: a module with no declaration writes
    /// nothing, and `Keys`' one is registered here rather than only in its own
    /// tests.** Nothing called
    /// `module_declares` at all, so the module's declaration was a constant
    /// nobody consulted and every line it composed would have been refused —
    /// silently, which is the failure the board's own list warns about.
    #[test]
    fn keys_declares_its_act_lines_at_start_and_no_told_lines() {
        let declarations = the_declarations();
        assert!(declarations.may_write(&"Keys", LineKind::Act));
        // §12's told half is none, and it is none structurally.
        assert!(!declarations.may_write(&"Keys", LineKind::Told));
        // **`Servers` §12 and `Server Setup` §12, registered 2026-09-22** —
        // each declares both kinds. *(This line read* a module that has not
        // declared writes nothing *over `Servers`, which had not declared here
        // and whose own crate carried a declaration nothing read.)*
        for module in ["Servers", "Server Setup"] {
            assert!(declarations.may_write(&module, LineKind::Act));
            assert!(declarations.may_write(&module, LineKind::Told));
        }
        // And a module that has not declared writes nothing.
        assert!(!declarations.may_write(&"Download", LineKind::Act));
        // **`File Browser` §11.1**: act lines, and no told lines (`File
        // Browser` B5), from step 10c's Files' owed acts piece 3.
        assert!(declarations.may_write(&"File Browser", LineKind::Act));
        assert!(!declarations.may_write(&"File Browser", LineKind::Told));
    }

    /// **§4 A7's warning declarations are registered at start**, and each
    /// identity is read from its own module's crate: `File Browser` M3's one,
    /// and `Rendering` W4a's one.
    ///
    /// **A module absent from the register answers *no* to everything** —
    /// A7's hosting rule, and the reason a suppression written for an
    /// unregistered module is refused rather than written.
    ///
    /// **The `Rendering` half is what makes the Render settings band's
    /// *Don't warn me about this again* take.** That switch-off goes through
    /// `said_once::a_warning_was_switched_off`, which checks this register
    /// before it writes (§15 G5), so without the row below the write is refused
    /// and the band comes back next time.
    #[test]
    fn the_declared_warnings_are_registered_at_start() {
        let declared = what_the_modules_declare();
        assert!(declared.the_warning_is_declared("File Browser", "file-browser.delete"));
        assert!(declared.the_warning_is_declared("Rendering", "rendering.missing-assets"));
        assert!(
            declared.the_warning_is_declared(
                smoozy_rendering::warning::THE_SUPPRESSION_ROW.0,
                smoozy_rendering::warning::THE_SUPPRESSION_IDENTITY,
            ),
            "the Render settings band's switch-off is refused at the \
             declaration check, so the user ticks *Don't warn me about this \
             again* and the band comes back next time"
        );
        assert_eq!(
            declared.every_declared_warning(),
            vec!["file-browser.delete", "rendering.missing-assets"]
        );
        // A7: not a default this document supplies.
        assert!(!declared.the_warning_is_declared("Servers", "servers.disconnected-removal"));
        assert!(!declared.the_warning_is_declared("File Browser", "file-browser.rename"));
        // And the identity belongs to its own module, never another's.
        assert!(!declared.the_warning_is_declared("File Browser", "rendering.missing-assets"));
    }

    /// **§15 G4, over the register the application actually builds: every
    /// declared warning identity names its own module.**
    ///
    /// **This is the check the one-column suppression row rests on.** G4 and
    /// `State, identity and recovery` §10.3 both say a row is *the module and
    /// the warning identity*; the row is one column, so the module is in the
    /// spelling, and **this route is the one place that binding can be
    /// checked**. The test in `crate::presentation::declarations` asks the same
    /// question of a hand-made register; this one asks it of the real one.
    #[test]
    fn every_warning_the_application_declares_names_its_own_module() {
        assert_eq!(
            what_the_modules_declare().every_declared_warning_names_its_module(),
            Vec::new(),
            "a module declared a warning identity that does not carry its own \
             module's name. **G4: a row is the module and the warning identity** \
             — the row is one column, so an identity that does not name its \
             module leaves nothing anywhere able to say which module it belongs \
             to"
        );
    }

    /// **`Telegram` §19's declaration is registered here**: acts, and no told
    /// lines, which that section declares so the absence is a decision.
    #[test]
    fn telegram_declares_its_act_lines_at_start_and_no_told_lines() {
        let declarations = the_declarations();
        assert!(declarations.may_write(&"Telegram", LineKind::Act));
        assert!(!declarations.may_write(&"Telegram", LineKind::Told));
    }

    /// **`Render Settings` §12's declaration is registered here, so a stored
    /// reading's *blend read* line is written** — step 10c's four small things
    /// after *Read the file*, item 4. Until it was, the page composed the line
    /// on every stored reading and `Desktop app and delivery` A7's hosting rule
    /// refused it, exactly as `Servers` and `Server Setup` were refused until
    /// 2026-09-22.
    #[test]
    fn a_stored_readings_blend_read_line_is_written_under_render_settings() {
        let declarations = the_declarations();
        assert!(declarations.may_write(&crate::reading_the_file::THE_MODULE, LineKind::Act));
        // §12's told half is none, and it is none structurally.
        assert!(!declarations.may_write(&crate::reading_the_file::THE_MODULE, LineKind::Told));

        // **As the page writes it**: the act line `Render Settings` composes,
        // through the module's own diary under the name it declared.
        let mut area = std::env::temp_dir();
        area.push(format!("smoozy-blend-read-line-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&area);
        let service = crate::diary_service::DiaryService::open_at(area.clone()).expect("a diary");
        let issue = |kind| Identity::issue(kind).expect("randomness");
        let run = issue(IdentityKind::Run);
        let act = smoozy_render_settings::diary::ABlendRead {
            the_blends_recorded_path: "/ephemeral/smoozy/assets/kitchen_v14.blend".to_string(),
            taken_by: smoozy_render_settings::file_half::BlenderIdentity {
                path: "/opt/blender-5.2.2/blender".to_string(),
                version: smoozy_server_setup::blender::Version { major: 5, minor: 2, patch: 2 },
            },
        };
        let wrote = crate::diary_service::ModulesDiary {
            service: &service,
            declarations: &declarations,
            run: run.clone(),
            module: crate::reading_the_file::THE_MODULE,
        }
        .write(&act.line(
            smoozy_render_settings::diary::Concerning {
                run,
                server_row: issue(IdentityKind::LocalServer),
                the_users_label_for_the_machine: "the big box".to_string(),
                operation: issue(IdentityKind::Operation),
            },
            SystemTime::now(),
        ));
        assert_eq!(wrote, crate::diary_service::Wrote::Yes);
        let mut kept = String::new();
        for entry in std::fs::read_dir(&area).expect("the area").flatten() {
            kept.push_str(&std::fs::read_to_string(entry.path()).unwrap_or_default());
        }
        let _ = std::fs::remove_dir_all(&area);
        assert!(kept.contains("blend read") && kept.contains("kitchen_v14.blend"), "{kept}");
    }

    /// **`Queue & History` §17's declaration is registered here, so *Track
    /// it*'s *untracked work taken over* line is written** — owed piece 11.
    /// Until it was, `Desktop app and delivery` A7's hosting rule refused every
    /// line the module composed, as it refused `Render Settings`' above.
    #[test]
    fn track_its_untracked_work_taken_over_line_is_written_under_queue_and_history() {
        let declarations = the_declarations();
        let module = crate::servers::the_machines_work::THE_QUEUES_MODULE;
        assert!(declarations.may_write(&module, LineKind::Act));
        assert!(declarations.may_write(&module, LineKind::Told), "§17's one told line, a job's ending");

        let mut area = std::env::temp_dir();
        area.push(format!("smoozy-taken-over-line-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&area);
        let service = crate::diary_service::DiaryService::open_at(area.clone()).expect("a diary");
        let issue = |kind| Identity::issue(kind).expect("randomness");
        let run = issue(IdentityKind::Run);
        let wrote = crate::diary_service::ModulesDiary {
            service: &service,
            declarations: &declarations,
            run: run.clone(),
            module,
        }
        .write(
            &smoozy_queue_history::diary::AnAct::UntrackedWorkTakenOver {
                the_servers_label: "the big box".to_string(),
                jobs_covered: 2,
            }
            .line(
                &smoozy_queue_history::diary::Concerning {
                    run,
                    server_row: Some(issue(IdentityKind::LocalServer)),
                    operations: Vec::new(),
                    group: None,
                },
                SystemTime::now(),
            ),
        );
        assert_eq!(wrote, crate::diary_service::Wrote::Yes);
        let mut kept = String::new();
        for entry in std::fs::read_dir(&area).expect("the area").flatten() {
            kept.push_str(&std::fs::read_to_string(entry.path()).unwrap_or_default());
        }
        let _ = std::fs::remove_dir_all(&area);
        assert!(kept.contains("untracked work taken over") && kept.contains("the big box"), "{kept}");
    }

    /// A local record of this test's own, at a path nothing else uses.
    fn a_local_record(name: &str) -> (LocalRecord, std::path::PathBuf) {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "smoozy-start-met-{name}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let file = path.join("local-record.sqlite3");
        (LocalRecord::open_at(&file).expect("the record opens"), path)
    }

    /// `Reporting a problem` U7: the session line carries the two values E11
    /// names and nothing more.
    ///
    /// **The fourth field is a throwaway copy's and nobody else's.** A test run
    /// is not a throwaway copy — no launch argument points it anywhere — so
    /// what is counted here is the ordinary line, which is the one every build
    /// that goes out writes.
    #[test]
    fn the_session_line_carries_the_version_and_the_system() {
        let run = Identity::issue(IdentityKind::Run).unwrap();
        let line = session_line(run);
        assert_eq!(line.kind, LineKind::Session);
        assert_eq!(line.clock, Clock::ThisComputer);
        let fields: Vec<&str> = line.text.split('\t').collect();
        assert!(
            !crate::platform::locations::this_copy_is_a_throwaway_one(),
            "a test run called itself a throwaway copy"
        );
        assert_eq!(fields.len(), 3, "{:?}", line.text);
        assert_eq!(fields[0], "session");
        assert_eq!(fields[1], env!("CARGO_PKG_VERSION"));
        assert_eq!(fields[2], std::env::consts::OS);
    }

    /// `State, identity and recovery` I2's cycle: an absent mark is a fresh
    /// installation, a *running* mark is the previous run not having ended
    /// cleanly, and a *clean* one is it having done.
    #[test]
    fn the_run_mark_survives_a_write_and_a_read() {
        let mut path = std::env::temp_dir();
        path.push(format!("smoozy-start-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        let path = path.join("local-record.sqlite3");

        let record = LocalRecord::open_at(&path).unwrap();
        assert_eq!(read_run_mark(&record).unwrap(), None);
        assert_eq!(
            what_the_launch_found(read_run_mark(&record).unwrap()),
            WhatTheLaunchFound::NoPreviousRun
        );

        let run = Identity::issue(IdentityKind::Run).unwrap();
        write_run_mark(&record, &RunMark::this_run_is_running(run.clone())).unwrap();
        match what_the_launch_found(read_run_mark(&record).unwrap()) {
            WhatTheLaunchFound::ThePreviousRunDidNot { the_run } => assert_eq!(the_run, run),
            other => panic!("{other:?}"),
        }

        write_run_mark(&record, &RunMark::the_user_asked_to_quit(run)).unwrap();
        assert_eq!(
            what_the_launch_found(read_run_mark(&record).unwrap()),
            WhatTheLaunchFound::ThePreviousRunEndedCleanly
        );

        drop(record);
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    mod the_last_runs_downloads {
        use super::*;
        use crate::record::transfer_records;
        use crate::transfers::ThePlatformFileAdapter;
        use smoozy_server_protocol::transfers::integrity::Digest;
        use smoozy_server_protocol::transfers::moving::TheUsersOwnComputer;
        use smoozy_server_protocol::transfers::publication::{Intent, PublicationPreState};
        use smoozy_server_protocol::transfers::{
            Direction, Ending, StableFileIdentity, StoppedReason, TransferRecord,
        };
        use std::path::{Path, PathBuf};

        const TWELVE: &[u8] = b"twelve bytes";

        fn a_download(server: &Identity, destination: &Path, name: &str) -> TransferRecord {
            TransferRecord::arranged(
                Identity::issue(IdentityKind::Transfer).unwrap(),
                Direction::Down,
                server.clone(),
                Some(Identity::issue(IdentityKind::Installation).unwrap()),
                format!("/machine/{name}"),
                destination.join(name).to_string_lossy().into_owned(),
                &destination.join(".smoozy-staging").to_string_lossy(),
                TWELVE.len() as u64,
                Identity::issue(IdentityKind::Run).unwrap(),
            )
        }

        /// A staging object made and named the way the byte loop makes it:
        /// the exclusive create, then its identity on the record.
        fn staged(transfer: &mut TransferRecord, bytes: &[u8]) -> PathBuf {
            let mut adapter = ThePlatformFileAdapter;
            let at = transfer.staging_path.as_text();
            adapter.create_staging_exclusively(&at).unwrap();
            adapter.write_at(&at, 0, bytes).unwrap();
            transfer.staging_identity = Some(adapter.stable_identity_of(&at));
            PathBuf::from(at)
        }

        fn pending_create_intended(transfer: &mut TransferRecord) {
            transfer.publication_pending = Some(PublicationPreState {
                intent: Intent::CreateIntended,
                staging_path: transfer.staging_path.as_text(),
                final_path: transfer.final_path.clone(),
                staged_identity: transfer
                    .staging_identity
                    .clone()
                    .unwrap_or(StableFileIdentity::TheSupplierCouldNotAnswer),
                staged_byte_length: TWELVE.len() as u64,
                staged_digest: Digest::of(TWELVE),
            });
        }

        /// **`Server protocol and transfers` S28's second bullet and this
        /// plan's C13, at the next start, before any transfer is admitted** —
        /// over a record holding one of each removable ending, an owed cleanup,
        /// a superseded attempt, an *interrupted* record, a file the record
        /// cannot claim, and two reservations a crash left.
        ///
        /// **The case this bites on is a start that settles nothing**, which
        /// is what the application did: both acts were written and tested and
        /// called by nothing, so every part-file a stop or a failure could not
        /// remove stayed in the user's own folder, and a reservation left
        /// *publication pending* made `Download`'s own admission refuse every
        /// download of that server, for good, with nothing said.
        #[test]
        fn the_next_start_settles_what_the_last_runs_downloads_left() {
            let (record, holder) = a_local_record("the-last-run-left");
            let destination = holder.join("Downloads");
            std::fs::create_dir_all(&destination).unwrap();
            let server = Identity::issue(IdentityKind::LocalServer).unwrap();

            // **Every removable ending, each with its material still there** —
            // the ending written first (`Server protocol and transfers` C23)
            // and the run gone before the removal.
            let mut to_be_removed = Vec::new();
            let mut the_records = Vec::new();
            for (at, ending) in [
                Ending::Stopped(StoppedReason::AbandonedByItsRequest),
                Ending::Stopped(StoppedReason::StagingRemovedByUser),
                Ending::Stopped(StoppedReason::TheSourceChangedUnderIt),
                Ending::Failed,
                Ending::Stalled,
            ]
            .into_iter()
            .enumerate()
            {
                let mut one = a_download(&server, &destination, &format!("frame_{at}.exr"));
                to_be_removed.push(staged(&mut one, b"part"));
                one.ending = Some(ending);
                the_records.push(one);
            }
            // **An owed cleanup** (`Server protocol and transfers` S27).
            let mut owed = a_download(&server, &destination, "owed.exr");
            to_be_removed.push(staged(&mut owed, b"part"));
            owed.ending = Some(Ending::Failed);
            owed.cleanup_is_owed = true;
            let owed_transfer = owed.transfer.clone();
            the_records.push(owed);
            // **A superseded attempt of a download that went on to publish.**
            let mut superseded = a_download(&server, &destination, "superseded.exr");
            to_be_removed.push(staged(&mut superseded, b"part"));
            superseded.start_over();
            superseded.ending = Some(Ending::Published);
            the_records.push(superseded);

            // **Interrupted is never swept** (`Server protocol and transfers`
            // S28a) — its material is the leftover the user is promised.
            let mut interrupted = a_download(&server, &destination, "interrupted.exr");
            let kept_for_the_user = staged(&mut interrupted, b"part");
            interrupted.ending = Some(Ending::Interrupted);
            the_records.push(interrupted);
            // **A file the record cannot claim**: another identity at its path.
            let mut not_ours = a_download(&server, &destination, "not-ours.exr");
            let not_claimed = staged(&mut not_ours, b"part");
            not_ours.staging_identity = Some(StableFileIdentity::The("0:0".into()));
            not_ours.ending = Some(Ending::Stalled);
            the_records.push(not_ours);

            // **C13's two cases**: a crash between the reservation and the
            // rename, with the staged file whole — and with it gone.
            let mut to_complete = a_download(&server, &destination, "to-complete.exr");
            let whole = staged(&mut to_complete, TWELVE);
            pending_create_intended(&mut to_complete);
            std::fs::write(destination.join("to-complete.exr"), b"").unwrap();
            let completed_transfer = to_complete.transfer.clone();
            the_records.push(to_complete);
            let mut to_remove = a_download(&server, &destination, "to-remove.exr");
            pending_create_intended(&mut to_remove);
            std::fs::write(destination.join("to-remove.exr"), b"").unwrap();
            let removed_transfer = to_remove.transfer.clone();
            the_records.push(to_remove);

            for one in &the_records {
                transfer_records::put(record.connection(), one).unwrap();
            }
            // **Before the start, the admission refuses this server.**
            let standing = transfer_records::the_unended_and_the_pending_without_their_marks(
                record.connection(),
                &server,
            )
            .unwrap();
            assert!(!smoozy_download::arrangement::transfers_may_be_admitted(&standing));

            let settled = what_the_last_run_left_is_settled(&record);
            assert_eq!(settled.trouble, None);

            for path in &to_be_removed {
                assert!(!path.exists(), "{} was left in the user's own folder", path.display());
            }
            assert!(kept_for_the_user.exists(), "an interrupted transfer's leftover was swept");
            assert!(not_claimed.exists(), "a file the record could not claim was removed");

            // **The debt is cleared once the removal is confirmed.**
            let owed_after = transfer_records::read(record.connection(), &owed_transfer)
                .unwrap()
                .unwrap();
            assert!(!owed_after.cleanup_is_owed);

            // **C13 completed the one and removed the other, and the records
            // say so** — which is what lifts the admission's refusal.
            assert_eq!(
                std::fs::read(destination.join("to-complete.exr")).unwrap(),
                TWELVE
            );
            assert!(!whole.exists());
            let completed = transfer_records::read(record.connection(), &completed_transfer)
                .unwrap()
                .unwrap();
            assert_eq!(completed.ending, Some(Ending::Published));
            assert_eq!(completed.publication_pending, None);
            assert!(!destination.join("to-remove.exr").exists());
            let removed = transfer_records::read(record.connection(), &removed_transfer)
                .unwrap()
                .unwrap();
            assert_eq!(removed.publication_pending, None);

            let standing = transfer_records::the_unended_and_the_pending_without_their_marks(
                record.connection(),
                &server,
            )
            .unwrap();
            assert!(
                smoozy_download::arrangement::transfers_may_be_admitted(&standing),
                "a reservation the last run left still refuses every download of its server"
            );
            assert_eq!(settled.publications_completed, 1);
            assert_eq!(settled.reservations_removed, 1);
            assert_eq!(settled.still_pending, 0);

            drop(record);
            let _ = std::fs::remove_dir_all(holder);
        }

        /// **`Server protocol and transfers` S15's paragraph of 2026-09-29, at
        /// the next start: a download already ended *failed*, *stalled* or
        /// *stopped* is settled by C13 without completing anything, and its
        /// pending mark then goes.**
        ///
        /// **The case this bites on is C13 read as though the work were still
        /// owed.** A failed publication with the user's own file at the name
        /// left the record pending for good — C13 rightly leaves that file,
        /// and so left the mark, and `Download`'s admission refused the
        /// server's downloads at every start after. And a stalled record whose
        /// staged file outlived its removal would have been *completed*, a
        /// start turning an ending that said the work was over into
        /// *published*.
        #[test]
        fn the_next_start_clears_the_mark_of_a_download_already_ended_and_completes_nothing() {
            let (record, holder) = a_local_record("already-ended");
            let destination = holder.join("Downloads");
            std::fs::create_dir_all(&destination).unwrap();
            let server = Identity::issue(IdentityKind::LocalServer).unwrap();

            // **Failed at the name the user took**: their file stands there.
            let mut taken = a_download(&server, &destination, "taken.exr");
            pending_create_intended(&mut taken);
            taken.ending = Some(Ending::Failed);
            let the_users_own = destination.join("taken.exr");
            std::fs::write(&the_users_own, b"the user's own compositing copy").unwrap();

            // **Stalled, its staged file still whole and Smoozy's zero-byte
            // reservation still at the name.**
            let mut stalled = a_download(&server, &destination, "stalled.exr");
            let staged_whole = staged(&mut stalled, TWELVE);
            pending_create_intended(&mut stalled);
            stalled.ending = Some(Ending::Stalled);
            stalled.cleanup_is_owed = true;
            let the_reservation = destination.join("stalled.exr");
            std::fs::write(&the_reservation, b"").unwrap();

            // **Stopped, with nothing at the name at all.**
            let mut stopped = a_download(&server, &destination, "stopped.exr");
            pending_create_intended(&mut stopped);
            stopped.ending = Some(Ending::Stopped(StoppedReason::AbandonedByItsRequest));

            for one in [&taken, &stalled, &stopped] {
                transfer_records::put(record.connection(), one).unwrap();
            }
            let settled = what_the_last_run_left_is_settled(&record);
            assert_eq!(settled.trouble, None);

            let read = |one: &TransferRecord| {
                transfer_records::read(record.connection(), &one.transfer)
                    .unwrap()
                    .unwrap()
            };
            assert_eq!(
                std::fs::read(&the_users_own).unwrap(),
                b"the user's own compositing copy"
            );
            assert_eq!(read(&taken).publication_pending, None, "the user's file kept the mark");
            assert_eq!(read(&taken).ending, Some(Ending::Failed));

            assert!(!the_reservation.exists(), "Smoozy's reservation was left at the name");
            assert!(!staged_whole.exists(), "the stalled transfer's staged file was not swept");
            assert_eq!(
                read(&stalled).ending,
                Some(Ending::Stalled),
                "a transfer whose ending said the work was over was completed at start"
            );
            assert_eq!(read(&stalled).publication_pending, None);

            assert_eq!(read(&stopped).publication_pending, None);

            let standing = transfer_records::the_unended_and_the_pending_without_their_marks(
                record.connection(),
                &server,
            )
            .unwrap();
            assert!(smoozy_download::arrangement::transfers_may_be_admitted(&standing));
            assert_eq!(settled.still_pending, 0);
            assert_eq!(settled.publications_completed, 0);

            drop(record);
            let _ = std::fs::remove_dir_all(holder);
        }

        /// **The start takes the settling, after the record opens and before
        /// it returns** — which is before anything can admit a transfer, since
        /// nothing does until the window is built. Asserted over the start's
        /// own text, for the reason the close's sweep above is: the whole start
        /// is reached only in a running application.
        #[test]
        fn the_start_settles_the_last_runs_downloads_once_the_record_is_open() {
            let working = include_str!("start.rs")
                .split("#[cfg(test)]")
                .next()
                .expect("the working half");
            let start = working.find("pub fn start(").expect("the start");
            let body = &working[start..];
            let body = &body[..body.find("\n}\n").expect("the end of the start")];
            let opened = body.find("LocalRecord::open()").expect("the record is opened");
            let settled = body
                .find("what_the_last_run_left_is_settled(")
                .expect("the start never settles what the last run's downloads left");
            assert!(opened < settled);
        }
    }
}

/// `State, identity and recovery` I2's second write, and there is no third.
///
/// **Written durably at the moment the user asks to quit** as *(clean, this
/// run's identity)* — **the ask, not the exit**, so `State, identity and
/// recovery` D6's second sentence is honoured rather than excepted — and
/// **nothing is written on the way out**.
///
/// **Why it is here and not left for a later step.** Without it every run ends
/// with the mark still reading *running*, so the launch-time offer
/// `Reporting a problem` §9 makes would fire after every ordinary quit. The
/// offer itself is step 11's; the mark's own cycle is this step's, and half a
/// cycle is worse than none because it is wrong rather than absent.
pub fn the_user_asked_to_quit(started: &Started) {
    let (Some(record), Some(run)) = (started.record.as_ref(), started.run.as_ref()) else {
        return;
    };
    let _ = write_run_mark(record, &RunMark::the_user_asked_to_quit(run.clone()));
}

/// **The update-launch mode's one mark** — the argument the update mechanism
/// puts on the probe and on the build a rollback starts, and nothing else
/// passes (`Reporting a problem` U5a and `Reporting a problem` U5b; Z21b).
pub const THE_UPDATE_LAUNCH_ARGUMENT: &str = "--smoozy-update-launch";

/// `Reporting a problem` U5a and `Reporting a problem` U5b: only the update mechanism passes this flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchMode { Ordinary, Update }
impl LaunchMode {
    pub fn from_arguments(arguments: impl IntoIterator<Item = std::ffi::OsString>) -> Self {
        if arguments.into_iter().any(|one| one == THE_UPDATE_LAUNCH_ARGUMENT) {
            Self::Update
        } else { Self::Ordinary }
    }
}

/// **What this launch owes an update in its probe** — Z21a, met at the start,
/// once the record is open. Added at step 11's Mac update piece.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatThisLaunchOwesTheProbe {
    /// Nothing: no record, or one already completed here.
    Nothing,
    /// **This launch is the probe of the release the record names**: the
    /// token is said once its window is built, and the record is cleared once
    /// the watcher lets go (`crate::updating::the_probe`).
    TheTokenOnceTheWindowIsBuilt {
        token: String,
        probe: crate::record::update_trust::TheUpdateInProbe,
    },
}

/// **Z21a's fourth bullet, at the start.** A probe whose record names this
/// build is owed its token later. **Any other launch that finds an uncleared
/// record completes it now**: one naming this build completes the probe
/// itself, one naming another build completes the rollback that did not
/// finish — both by [`crate::updating::the_probe::what_the_record_kept_is_cleared`].
/// **A probe that finds a record naming another build is owed nothing**: it
/// is not the release being published, so it never says the token, and its
/// watcher rolls back.
///
/// **This runs after the run-mark's read and write**, so a probe and the
/// build a rollback starts have already done `Reporting a problem` U3 step 1
/// when it is asked — and
/// neither makes an offer, which is `crate::surface::reporting`'s, by the
/// launch mode.
pub fn the_probe_record_is_met(started: &Started, the_probes_token: Option<String>) -> WhatThisLaunchOwesTheProbe {
    use crate::updating::the_probe::{what_the_record_asks, what_the_record_kept_is_cleared, WhatTheRecordAsks};
    let Some(record) = started.record.as_ref() else {
        return WhatThisLaunchOwesTheProbe::Nothing;
    };
    let Ok(asks) = what_the_record_asks(record.connection(), crate::updating::checking::THIS_BUILDS_VERSION) else {
        return WhatThisLaunchOwesTheProbe::Nothing;
    };
    let probe = match (asks, the_probes_token) {
        (WhatTheRecordAsks::ThisBuildIsTheRelease(probe), Some(token)) => {
            return WhatThisLaunchOwesTheProbe::TheTokenOnceTheWindowIsBuilt { token, probe }
        }
        (WhatTheRecordAsks::ThisBuildIsTheRelease(probe), None)
        | (WhatTheRecordAsks::ARollbackDidNotFinish(probe), None) => probe,
        _ => return WhatThisLaunchOwesTheProbe::Nothing,
    };
    let kept = crate::platform::locations::resolve(crate::platform::locations::Location::ThePreviousBundleKept).ok();
    let running = crate::platform::locations::resolve(crate::platform::locations::Location::TheInstalledBundle).ok();
    let _ = what_the_record_kept_is_cleared(record.connection(), record.path(), &probe, kept.as_deref(), running.as_deref());
    WhatThisLaunchOwesTheProbe::Nothing
}

/// `Reporting a problem` C1's three positive conditions.
pub fn the_crash_to_offer(holds_the_instance: bool, mode: LaunchMode, previous: &WhatTheLaunchFound) -> Option<Identity> {
    if !holds_the_instance || mode != LaunchMode::Ordinary { return None; }
    match previous {
        WhatTheLaunchFound::ThePreviousRunDidNot { the_run } => Some(the_run.clone()),
        _ => None,
    }
}

#[cfg(test)]
mod after_a_crash {
    use super::*;
    #[test]
    fn only_a_holder_after_a_dead_run_gets_the_offer() {
        let run = Identity::issue(IdentityKind::Run).unwrap();
        let previous = WhatTheLaunchFound::ThePreviousRunDidNot { the_run: run.clone() };
        assert_eq!(the_crash_to_offer(true, LaunchMode::Ordinary, &previous), Some(run));
        assert_eq!(the_crash_to_offer(false, LaunchMode::Ordinary, &previous), None);
        assert_eq!(the_crash_to_offer(true, LaunchMode::Update, &previous), None);
        assert_eq!(the_crash_to_offer(true, LaunchMode::Ordinary, &WhatTheLaunchFound::NoPreviousRun), None);
        assert_eq!(the_crash_to_offer(true, LaunchMode::Ordinary, &WhatTheLaunchFound::ThePreviousRunEndedCleanly), None);
    }
    #[test]
    fn the_durable_mark_offers_a_dead_run_once_and_clean_quits_offer_nothing() {
        let folder=crate::scratch_folders::a_folder(format!("smoozy-r5-marks-{}",std::process::id()));
        let path=folder.join("record.sqlite3");
        let record=LocalRecord::open_at(&path).unwrap();
        let first=Identity::issue(IdentityKind::Run).unwrap();
        assert_eq!(the_mark_is_read_and_replaced(Some(&record),Some(&first)),WhatTheLaunchFound::NoPreviousRun);
        drop(record);
        let record=LocalRecord::open_at(&path).unwrap();
        let second=Identity::issue(IdentityKind::Run).unwrap();
        let found=the_mark_is_read_and_replaced(Some(&record),Some(&second));
        assert_eq!(the_crash_to_offer(true,LaunchMode::Ordinary,&found),Some(first));
        assert_eq!(read_run_mark(&record).unwrap(),Some(RunMark::this_run_is_running(second.clone())));
        write_run_mark(&record,&RunMark::the_user_asked_to_quit(second)).unwrap();
        let probe=Identity::issue(IdentityKind::Run).unwrap();
        let found=the_mark_is_read_and_replaced(Some(&record),Some(&probe));
        assert_eq!(the_crash_to_offer(true,LaunchMode::Ordinary,&found),None);
        let rollback=Identity::issue(IdentityKind::Run).unwrap();
        let found=the_mark_is_read_and_replaced(Some(&record),Some(&rollback));
        assert_eq!(the_crash_to_offer(true,LaunchMode::Update,&found),None);
        assert_eq!(read_run_mark(&record).unwrap(),Some(RunMark::this_run_is_running(rollback.clone())));
        let ordinary=Identity::issue(IdentityKind::Run).unwrap();
        let found=the_mark_is_read_and_replaced(Some(&record),Some(&ordinary));
        assert_eq!(the_crash_to_offer(true,LaunchMode::Ordinary,&found),Some(rollback));
        record.connection().execute("UPDATE run_mark SET state='unreadable'",[]).unwrap();
        assert_eq!(the_mark_is_read_and_replaced(Some(&record),Some(&ordinary)),WhatTheLaunchFound::NoPreviousRun);
        assert_eq!(the_mark_is_read_and_replaced(None,Some(&ordinary)),WhatTheLaunchFound::NoPreviousRun);
        assert_eq!(the_mark_is_read_and_replaced(Some(&record),None),WhatTheLaunchFound::NoPreviousRun);
    }

    /// **Z21a at the start**: a probe of the release is owed its token and
    /// clears nothing yet; an ordinary launch, and the build a rollback
    /// starts, complete an uncleared record at once; a probe finding a record
    /// for another build is owed nothing and clears nothing.
    #[test]
    fn the_start_meets_an_uncleared_probe_record() {
        use crate::record::update_trust;
        let this = crate::updating::checking::THIS_BUILDS_VERSION;
        let folder = crate::scratch_folders::a_folder(format!("smoozy-start-probe-{}", std::process::id()));
        let path = folder.join("record.sqlite3");
        let started = |record: LocalRecord| Started {
            the_sweep_left_behind: Vec::new(),
            record: Some(record),
            record_trouble: None,
            diary: None,
            declarations: Declarations::new(),
            what_modules_declare: what_the_modules_declare(),
            said_once: TheThingsSaidOnce::new(),
            run: None,

            the_previous_run: WhatTheLaunchFound::NoPreviousRun,
        };
        let probe = |version: &str| update_trust::TheUpdateInProbe {
            release_sequence: 3,
            version: version.into(),
            the_previous_artifact: b"/nowhere/.Smoozy previous.app".to_vec(),
            the_record_copy: crate::updating::the_probe::path_bytes(&crate::record::copy_beside(&path)),
        };
        let it = started(LocalRecord::open_at(&path).unwrap());
        let connection = || it.record.as_ref().unwrap().connection();

        update_trust::an_update_goes_into_probe(connection(), &probe(this)).unwrap();
        assert_eq!(
            the_probe_record_is_met(&it, Some("t".into())),
            WhatThisLaunchOwesTheProbe::TheTokenOnceTheWindowIsBuilt { token: "t".into(), probe: probe(this) }
        );
        assert!(update_trust::the_update_in_probe(connection()).unwrap().is_some(), "the probe cleared before its token");

        // **An uncleared probe record completed at the next start.**
        crate::record::take_the_copy(&path).unwrap();
        assert_eq!(the_probe_record_is_met(&it, None), WhatThisLaunchOwesTheProbe::Nothing);
        assert_eq!(update_trust::the_update_in_probe(connection()).unwrap(), None);
        assert!(!crate::record::copy_beside(&path).exists(), "L8's copy outlived the completed probe");

        // The rollback that did not finish, met by the previous build.
        update_trust::an_update_goes_into_probe(connection(), &probe("999.0.0")).unwrap();
        assert_eq!(
            the_probe_record_is_met(&it, Some("t".into())),
            WhatThisLaunchOwesTheProbe::Nothing,
            "a probe of another release was owed its token"
        );
        assert!(update_trust::the_update_in_probe(connection()).unwrap().is_some(), "a wrong probe cleared");
        assert_eq!(the_probe_record_is_met(&it, None), WhatThisLaunchOwesTheProbe::Nothing);
        assert_eq!(update_trust::the_update_in_probe(connection()).unwrap(), None);
    }

    #[test]
    fn the_update_launch_argument_is_exact() {
        let mode = |args: &[&str]| LaunchMode::from_arguments(args.iter().map(std::ffi::OsString::from));
        assert_eq!(mode(&[]), LaunchMode::Ordinary);
        assert_eq!(mode(&["--smoozy-update-launch"]), LaunchMode::Update);
        assert_eq!(mode(&["--smoozy-update-launch-no"]), LaunchMode::Ordinary);
    }
}
