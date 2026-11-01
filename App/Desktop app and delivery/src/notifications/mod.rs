//! **The notification service** — `Desktop app and delivery` §13 entire, T1 to
//! T30.
//!
//! **READ THIS BEFORE ADDING A CALLER.** Four things reach the user while
//! Smoozy runs behind their work: a job finishing, a job failing or being
//! given up, a connection Smoozy opened and could not restore, and a machine
//! that has had nothing to do for half an hour. **Nothing else is ever
//! posted** (T9), and **nothing anywhere in the application is advanced,
//! cleared, acknowledged or recorded by a post being made** (T22).
//!
//! ## Where each part is
//!
//! - **The key a post is replaced on** (§14 Q1, Q10) is **not here.** It is
//!   [`crate::presentation::the_presentation_key`], which is where piece C put
//!   it on 2026-09-09 — it was parked in this folder only because
//!   `src/presentation/` did not exist when piece G needed it.
//! - [`kinds`] — the four kinds, what composes each, and the words (T9–T14,
//!   T27, and §17 X5).
//! - [`picture`] — the frame preview and its three routes (T15–T17a).
//! - [`when`] — telling now from catching up, with no clock (T18–T21).
//! - [`permission`] — three permission models rather than one (T6–T8).
//! - [`activation`] — what a click does, and what travels with a post
//!   (T28–T30).
//! - `macos.rs`, `windows.rs`, `linux.rs` — **the machinery each system needs
//!   before a plain downloaded binary can post at all** (T1–T5) and the
//!   posting itself. They are platform code of this document's own, beside the
//!   shell, the same shape `src/reveal.rs` and `src/platform/` take.
//!
//! ## The thing that surprises everybody, said first
//!
//! **T1: a plain downloaded binary cannot post a notification on any of the
//! three systems without extra machinery, and in each case the machinery is
//! ordinary.** **It is a build output rather than run-time work**, which is why
//! §19 carries it — so a builder who looks only in `src/` will conclude the
//! Windows third cannot be done and be wrong. **macOS: the application ships
//! as a bundle with an identifier**, which `tauri.conf.json` already carries;
//! an unbundled executable is killed by an uncaught exception on the **very
//! first call, exit 134, with nothing to catch**, which is why
//! [`macos`](macos)'s route reads the bundle before it touches anything else.
//! **Windows: a per-user Start-menu shortcut carrying an application
//! identifier**, which §19 E3 puts in the application's own code at first
//! successful start, and **not** the runtime-package route. **Linux: no
//! identity precondition at all**, but a session message bus and a process
//! owning the notification name, and the specification says outright that
//! clients *"must not assume that it is available."*
//!
//! ## What a post does not do
//!
//! **A post is attempted and forgotten** (T23). [`TheNotificationService::post`]
//! returns nothing, and that is the clause rather than a simplification: on
//! Windows the post fails silently, on Linux the picture may be dropped with
//! no signal, and on macOS nothing had ever been posted at all — so **no state
//! may depend on the attempt's result**, and the way to keep that is for there
//! to be no result.
//!
//! ## What is unobserved, which is everything
//!
//! §22 item 11: **no notification had ever been posted and no permission ever
//! requested, deliberately, because the machine is the Creator's.** Four items
//! name what that leaves open and each is **permanently unresolved** rather
//! than pending, per the Creator's ruling of 2026-08-31: whether replacing on
//! a used identity re-alerts or restores a dismissed one (item 27), whether a
//! post can be taken down at all (item 61), whether activation routes off the
//! Mac (item 51), and whether the picture survives on Linux or Windows
//! (item 13). **No space is left in these files for an answer that is not
//! coming.**
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` F25.

pub mod activation;
pub mod kinds;
pub mod permission;
pub mod picture;
pub mod when;

use std::sync::Mutex;

use crate::typography::{Prose, Verbatim};
/// **The application identifier the shipped artifact carries** — §19 E2 and
/// E3, and T2 and T3 are what make it decisive rather than decorative.
///
/// **It is `tauri.conf.json`'s `identifier` and must stay that**, which is why
/// [`tests::the_application_identifier_is_the_one_the_artifact_is_built_with`]
/// reads the configuration file rather than trusting this line. The Windows
/// third names it on the Start-menu shortcut and on every toast; macOS reads
/// the bundle's own rather than this constant, because on that system the
/// value the process is actually running under is the one the notification
/// centre answers to.
pub const THE_APPLICATION_IDENTIFIER: &str = "com.smoozy.desktop";

use activation::WhatEachPostIsAbout;
use kinds::{Kind, WhatToPost};
use permission::WhatTheSystemSays;
use picture::ThePicture;
use crate::presentation::the_presentation_key::ThePresentationKey;
use crate::record::settings::TheSwitches;

// ── The machinery each system needs before anything can be posted ───────────

/// **T1 to T5: whether this system's precondition is met.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheMachinery {
    /// The precondition is met and a post may be attempted.
    ItIsInPlace,
    /// It is not, and **nothing is posted and no failure record is raised** —
    /// T5's Linux case, applied on all three because the shape is the same:
    /// a fact about the user's own computer is not a fact about Smoozy.
    ItIsNot(WhatIsMissing),
}

/// **What each system's precondition is, and there is one per system** — T2,
/// T3, T5.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatIsMissing {
    /// **T2, macOS: the bundle identity.** Apple documents that precondition
    /// **nowhere** and it is decisive: an unbundled executable dies on the very
    /// first call at exit 134 with nothing to catch; inside a bundle with an
    /// identifier it works. **An ad-hoc signature is enough** — no Developer
    /// ID and no notarization — so the precondition is the bundle, not the
    /// certificate.
    TheBundleIdentity,
    /// **T3, Windows: the Start-menu shortcut carrying an application
    /// identifier.** Microsoft's words: *"Without a valid shortcut installed
    /// in the Start screen or in All Programs, you cannot raise a toast
    /// notification from a desktop app."*
    TheStartMenuShortcut,
    /// **T5, Linux: a session message bus and a process owning the
    /// notification name.** The specification says outright that clients
    /// *"must not assume that it is available."* **On a computer with no
    /// notification service the four kinds do not arrive, and that is written
    /// as a case rather than as a failure** — §22 item 12.
    TheSessionBusOrTheNotificationService,
}

// ── What one post is ────────────────────────────────────────────────────────

/// **One notification, composed and ready to be attempted.**
///
/// **Three strings and they take two different routes** — §4 A6 and §17 X9.
/// [`APost::said`] and [`APost::body`] are **prose** and have each been through
/// the typography point on their own (X5: notification text is drawn by the
/// operating system and by no engine). The name of the thing it is about is a
/// **value** — a job's, a batch's or a server's own name — and takes the
/// verbatim route, because a non-breaking space substituted into a name is a
/// name Smoozy quietly changed (X7).
///
/// **Two lines, which is Design's shape** — see [`kinds`]'s own head, which
/// carries Design's four kinds with their worked examples. The first line is the
/// name and what happened; the second is the figures a user wants before
/// deciding whether to go and look.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct APost {
    /// **T24's identity: the presentation key.** One post per key, replaced
    /// rather than added to (§14 Q10).
    pub key: ThePresentationKey,
    pub kind: Kind,
    /// **The rest of the first line, after the name** — *finished*, *failed*,
    /// *abandoned*, *disconnected*, *is idle*.
    pub said: Prose,
    /// The name of the job, batch or server this is about. **May be empty**
    /// where the caller has no name to give; nothing is invented for it.
    pub about: Verbatim,
    /// **The second line's figures**, or Design's words where it gives words
    /// rather than figures. May be empty where no figure is known; [`kinds`]
    /// says which, kind by kind.
    pub body: Prose,
    /// **The second line's last part: the server the job rendered on** —
    /// Design's *· RunPod 4090*. **A value and not prose** (X7): it is the
    /// server's own label as the job's record copied it, and it takes the
    /// verbatim route for the reason [`APost::about`] does. Empty for the two
    /// kinds whose second line is words, and where the record holds no label.
    /// Added 2026-10-03, the 38's piece 1.
    pub on: Verbatim,
    pub picture: ThePicture,
}

impl APost {
    /// Finish a composed [`WhatToPost`] with the two things the composition
    /// deliberately does not hold: the subject's own name, and the picture.
    pub fn from(what: WhatToPost, about: Verbatim, picture: ThePicture) -> Self {
        APost {
            key: what.key,
            kind: what.kind,
            said: what.said,
            about,
            body: what.body,
            on: Verbatim::new(""),
            picture,
        }
    }

    /// **The server the job rendered on, as the second line's last part.**
    pub fn on_the_server(self, label: Verbatim) -> Self {
        APost { on: label, ..self }
    }

    /// **The second line, as a system draws it: the figures, then the server**
    /// — Design's *240 frames in 6 hours 2 minutes · RunPod 4090*.
    ///
    /// **Joined here for the reason [`APost::headline`] joins**: each half has
    /// already taken its own route, and X2 binds the point and not the caller.
    /// **A part that is absent leaves no separator hanging**, and nothing is
    /// invented for it.
    pub fn the_second_line(&self) -> String {
        match (self.body.as_str().is_empty(), self.on.as_str().is_empty()) {
            (false, false) => format!("{} \u{00B7} {}", self.body.as_str(), self.on.as_str()),
            (false, true) => self.body.as_str().to_string(),
            (true, false) => self.on.as_str().to_string(),
            (true, true) => String::new(),
        }
    }

    /// **The first line, as a system draws it: the name, then what happened.**
    ///
    /// *Kitchen — final finished*. *Old 4080 disconnected*. *Vast 5060 is
    /// idle*.
    ///
    /// **Joining two strings that have each already taken their own route is not
    /// the typography point parsing anything**, which is the only thing X2
    /// forbids — **X2 binds the point, not the caller.** Composing one sentence
    /// and handing *that* to the point would put a substituted character inside
    /// a name Smoozy is forbidden to alter (X7); joining after the fact cannot,
    /// because neither half is touched here.
    ///
    /// **A post with no name is its prose alone.** Nothing is invented to fill
    /// the gap and no separator is left hanging.
    pub fn headline(&self) -> String {
        if self.about.as_str().is_empty() {
            return self.said.as_str().to_string();
        }
        format!("{} {}", self.about.as_str(), self.said.as_str())
    }
}

// ── The seam each system's own code sits behind ─────────────────────────────

/// **What one system's own notification code does, and there are five things
/// and no sixth.**
///
/// A trait rather than three `cfg` blocks inside the service, for the reason
/// [`crate::events::Delivers`] gives: the judgements above stay testable at
/// this desk on all three systems' shapes, and **every test below drives the
/// real service** rather than a copy of it.
///
/// **Nothing here returns an outcome except the two reads** — T23: delivery is
/// never confirmed, so a post and a take-down have nothing to answer with.
pub trait ThePlatform: Send + Sync {
    /// T1–T5. Read afresh each time rather than cached: on Linux the
    /// notification service can come and go inside one run.
    fn the_machinery(&self) -> TheMachinery;
    /// T6. **Read, never asked** — the asking is the next method and T7 says
    /// when it happens.
    fn the_permission(&self) -> WhatTheSystemSays;
    /// **T7: the request is made at the first moment a notification would
    /// actually be posted, and never at application start.** On the two
    /// systems with no request interface this does nothing, which is T6's
    /// three-models-not-one arriving as an empty body rather than as a
    /// pretence.
    fn ask_for_permission_now(&self);
    /// **T23: attempted and forgotten.** No outcome, because no state may
    /// depend on one.
    fn post(&self, post: &APost);
    /// **T24a and T24b: attempted and forgotten on exactly the same terms.**
    /// §22 item 61: whether a posted notification can be taken down at all is
    /// **not established on any of the three**, which is precisely why nothing
    /// waits on this and nothing records it.
    fn take_down(&self, key: &ThePresentationKey);
}

// ── The service ─────────────────────────────────────────────────────────────

/// **The one notification service.**
///
/// One per run, reached by every module through §4 A1's named-service rule and
/// by no other route.
pub struct TheNotificationService {
    platform: Box<dyn ThePlatform>,
    held: Mutex<Held>,
}

#[derive(Default)]
struct Held {
    /// **T7: whether the request has been made in this run.** It is asked once
    /// and the system remembers the answer; asking again is what the system's
    /// own state is read for.
    the_request_has_been_made: bool,
    register: WhatEachPostIsAbout,
    /// **The per-kind switches, as the local record last answered them** — T8
    /// and T17a's third bullet. `None` until the record has answered, and
    /// `None` again where it did not.
    ///
    /// **They are held here rather than read from the record at each post**,
    /// and that is a ruling with one reason: a post is reached by callers that
    /// already hold the application's one lock on the record, and reading
    /// through that lock from inside a post would wait on itself. The record
    /// stays the only durable home (`State, identity and recovery` J5); this is
    /// the copy [`TheNotificationService::the_switches_are`] is handed at start
    /// and after every switch the user's own press has written.
    the_switches: Option<TheSwitches>,
}

impl TheNotificationService {
    /// The service on this computer's own system.
    pub fn for_this_system() -> Self {
        TheNotificationService::on(per_system::the_platform())
    }

    /// The service over a given platform. **Tests use this**, and so would a
    /// system this document's three do not cover.
    pub fn on(platform: Box<dyn ThePlatform>) -> Self {
        TheNotificationService {
            platform,
            held: Mutex::new(Held::default()),
        }
    }

    /// **Post one of the four kinds.**
    ///
    /// **It answers nothing** (T23), and nothing in the application is
    /// advanced, cleared, acknowledged or recorded by it (T22). Where the
    /// machinery is not in place or the permission is refused, **nothing is
    /// posted and no failure record is raised** (T5, T8).
    ///
    /// **The user's switch is asked first, before the machinery and before
    /// T7's permission request.** T17a's third bullet: *"A switch governs
    /// whether Smoozy **attempts** the post"* — so a kind the user switched off
    /// is not attempted, and a post that is not attempted is not *"the first
    /// moment a notification would actually be posted"*, which is the only
    /// moment T7 lets the macOS prompt appear. Asked after the permission step,
    /// a switched-off kind could still bring up a prompt about a notification
    /// the user said they do not want.
    ///
    /// **Where the record has not answered, no kind is attempted.** Attempting
    /// would be guessing at the user's own record, and the guess could be the
    /// prompt above; not attempting costs the reaching and nothing else, which
    /// T22 already prices — every kind leaves its own durable in-app record.
    pub fn post(&self, post: APost) {
        if !self.the_user_has_this_kind_attempted(post.kind) {
            return;
        }
        if !matches!(self.platform.the_machinery(), TheMachinery::ItIsInPlace) {
            return;
        }
        if !self.the_permission_is_in_hand() {
            return;
        }
        if let Ok(mut held) = self.held.lock() {
            held.register.a_post_was_attempted(&post.key);
        }
        self.platform.post(&post);
    }

    /// **The switches, as the local record answered them** — handed over at
    /// start, and again after every switch the user's press has written. `None`
    /// is a record that did not answer, and nothing is attempted under it.
    pub fn the_switches_are(&self, read: Option<TheSwitches>) {
        if let Ok(mut held) = self.held.lock() {
            held.the_switches = read;
        }
    }

    /// Whether this kind is attempted. **Reading never asks** (T7).
    fn the_user_has_this_kind_attempted(&self, kind: Kind) -> bool {
        match self.held.lock() {
            Ok(held) => held
                .the_switches
                .map(|switches| switches.attempts(kind))
                .unwrap_or(false),
            Err(_) => false,
        }
    }

    /// **T24a and T24b: a post raised from a condition record is taken down at
    /// whichever of the record's three endings ends it.**
    ///
    /// **It reaches the connection kind and any later kind raised from a
    /// condition record, and nothing else.** The other three of the four are
    /// endings, which §14 Q3 already keeps out of this machinery: an ending
    /// happened once and has nothing to withdraw. [`the_condition_records_own_endings`]
    /// is the closed set, and a key that is not a condition's is refused here
    /// rather than quietly removed.
    ///
    /// **Attempted and forgotten** (T23): nothing waits on it, nothing records
    /// it, and nothing is advanced or cleared by it succeeding or failing.
    pub fn the_condition_record_ended(
        &self,
        key: &ThePresentationKey,
        _how: HowTheRecordEnded,
    ) {
        if !matches!(key, ThePresentationKey::ACondition(_)) {
            debug_assert!(
                false,
                "T24a reaches a post raised from a condition record and nothing else — \
                 section 14 Q3 keeps endings out of this machinery"
            );
            return;
        }
        if let Ok(mut held) = self.held.lock() {
            held.register.a_post_was_taken_down(key);
        }
        self.platform.take_down(key);
    }

    /// **T7, and it is the whole reason this is not read at start.**
    ///
    /// At start nothing is happening and nothing is being told, so a prompt
    /// there is a question about nothing. **This is the same shape as the
    /// Linux store probe of §9 S5 and for the same reason**, and it keeps the
    /// prompt attached to the event that needs it.
    fn the_permission_is_in_hand(&self) -> bool {
        let permission = self.platform.the_permission();
        if !permission.the_asking_comes_first() {
            return permission.a_post_is_attempted();
        }
        let ask_now = match self.held.lock() {
            Ok(mut held) if !held.the_request_has_been_made => {
                held.the_request_has_been_made = true;
                true
            }
            Ok(_) => false,
            Err(_) => return false,
        };
        if ask_now {
            self.platform.ask_for_permission_now();
        }
        // **The request and the post it was made for are one act, and that is
        // a ruling rather than an obvious reading of T7.** The clause puts the
        // request at *"the first moment a notification would actually be
        // posted"*; a design that asked and then dropped that very post would
        // make the request a question about a notification the user never
        // gets, which is the same fault T7 names about asking at start.
        //
        // **So the post is attempted unless the system has actually refused.**
        // An answer that has not arrived is not a refusal — on macOS the
        // prompt is being read by a person while this returns — and **a denial
        // is silent** (T6), so a post made under an unanswered prompt is
        // ignored by the system and costs the reaching alone.
        !matches!(
            self.platform.the_permission(),
            WhatTheSystemSays::ItWasDenied | WhatTheSystemSays::TheSystemHasItSwitchedOff
        )
    }

    /// **T28 and T29: the user clicked one.**
    ///
    /// The window comes forward and the going is posted on the one ordered
    /// queue. See [`activation`] for what travels and what happens when the
    /// subject is gone.
    pub fn a_notification_was_clicked<R: tauri::Runtime>(
        &self,
        app: &tauri::AppHandle<R>,
        queue: &crate::events::TheOneQueue,
        the_post_identity: &str,
    ) {
        let Ok(held) = self.held.lock() else {
            return;
        };
        activation::a_notification_was_clicked(app, queue, &held.register, the_post_identity);
    }

    /// What this computer says about permission, for a caller that wants to
    /// read it without posting. **Reading never asks** (T7).
    pub fn the_permission(&self) -> WhatTheSystemSays {
        self.platform.the_permission()
    }

    /// What this computer's machinery is, for the same reason.
    pub fn the_machinery(&self) -> TheMachinery {
        self.platform.the_machinery()
    }
}

/// **The three endings a condition record has.** T24b is what brings all
/// three here, and the three are `State, identity and recovery` F25's.
///
/// T24a alone fired at §16.2 M4's value change, and M4 defines withdrawal as
/// one thing: the observed value moving outside the set the record was raised
/// over. **On a machine that is destroyed the value never leaves the set**, so
/// that trigger cannot fire and the record ends at the removal of the server
/// row — leaving a posted *could not be restored* naming a server that no
/// longer exists in Smoozy, on a surface Smoozy has already stopped owning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HowTheRecordEnded {
    /// §16.2 M4: the observed value moved outside the set the record was
    /// raised over — the connection restored, however it came back.
    TheObservedValueMovedOutOfTheSet,
    /// F25: its subject was removed — the server row deleted, the machine
    /// destroyed.
    ItsSubjectWasRemoved,
    /// F25: the user cleared the record themselves.
    TheUserClearedIt,
}

/// The closed set, so that a builder adding a fourth ending has to come here.
pub const fn the_condition_records_own_endings() -> [HowTheRecordEnded; 3] {
    [
        HowTheRecordEnded::TheObservedValueMovedOutOfTheSet,
        HowTheRecordEnded::ItsSubjectWasRemoved,
        HowTheRecordEnded::TheUserClearedIt,
    ]
}

/// **T28: the one place each system's activation route is joined to the
/// application.**
///
/// **Called during launch and not later.** macOS's requirement is the hard
/// one — the notification-centre delegate must be assigned **before the
/// application finishes launching**, because *"assigning a delegate after the
/// system calls these methods might cause you to miss incoming
/// notifications"* — and the other two are no worse off for being opened at
/// the same moment.
///
/// **What it is handed is a going, not a screen.** The closure receives the
/// post's own identity and nothing else, and what it does with it is
/// [`TheNotificationService::a_notification_was_clicked`].
pub fn the_activation_route_is_open(on_click: Box<dyn Fn(String) + Send + Sync>) {
    per_system::the_activation_route_is_open(on_click);
}

/// **T1's machinery, made — at the application's first successful start, which
/// is where §19 E3 puts it.**
///
/// **It is a separate act from reading whether the machinery is there, and that
/// is F-10's repair of 2026-09-12.** The read is published as a read — *"for a
/// caller that wants to read it without posting"* — and on Windows the function
/// behind it composed a path under the user's own Start menu and, where nothing
/// was there, **wrote a file**. So asking whether Smoozy could post put something
/// in the user's Start menu, at whatever moment anybody happened to ask.
///
/// **Two of the three systems have nothing to make.** macOS's precondition is
/// the bundle, which the artifact either is or is not, and Linux's is the user's
/// own session bus; neither is Smoozy's to create, and the empty body is that
/// rather than an omission.
///
/// **It answers nothing** (T23), and nothing anywhere depends on it having
/// worked: the read is what decides whether a post is attempted, and it is taken
/// afresh at every post.
pub fn the_machinery_is_made_at_first_start() {
    per_system::the_machinery_is_made();
}

/// **Z15a: T3's Start-menu shortcut, its target rewritten** — the one act the
/// Windows update route takes on it. A complete new shortcut naming `program`,
/// with the same application identifier (Z15a: *"The shortcut's application
/// identifier does not change between versions"*), is written beside the old
/// one and renamed over it through the platform file adapter (Z24). Made where
/// none was there. **Windows only**: elsewhere there is no such shortcut and
/// this refuses. Step 11's Windows update piece.
pub fn the_shortcut_is_pointed_at(program: &std::path::Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        per_system::the_shortcut_is_pointed_at(program)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = program;
        Err("the Start-menu shortcut is Windows'".into())
    }
}

/// **T22: nothing in the application is advanced, cleared, acknowledged or
/// recorded by a notification being posted.**
///
/// No cursor moves, no failure record clears, no outcome record changes.
/// **Every one of the four kinds also leaves its own durable in-app record** —
/// a job's ending in its history record, a failure record on the thing that
/// failed, the catch-up notice, `Queue & History`'s idle reading — so the
/// promise the notification serves is *reaching a user who is looking
/// elsewhere*, and losing one costs the reaching and nothing else.
pub const NOTHING_IS_ADVANCED_BY_A_POST: bool = true;

/// **T25: nothing is assumed to be coalesced for Smoozy.**
///
/// **No rate limit or coalescing is documented on any of the three**, so every
/// suppression the application needs is its own — which is §14's, keyed on the
/// presentation key, and not something borrowed from a platform.
pub const NOTHING_IS_ASSUMED_COALESCED: bool = true;

/// **T26: grouping is documented on macOS and Windows and absent on Linux, and
/// nothing here rests on it.**
///
/// T11's *speaks once* is composed from Smoozy's own records rather than from
/// a platform grouping facility, **which is what makes it hold on all three.**
pub const GROUPING_IS_NOT_RELIED_ON: bool = true;

// ── The three systems' own code ─────────────────────────────────────────────

#[cfg(target_os = "macos")]
#[path = "macos.rs"]
mod per_system;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod per_system;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod per_system;

/// **There is no fourth system in the release matrix**, and this arm exists so
/// that the crate compiles wherever somebody points a compiler at it rather
/// than failing in a way that reads like a fault in the service. **It is not a
/// route and claims nothing.**
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
mod per_system {
    use super::*;

    struct NoSuchSystem;

    impl ThePlatform for NoSuchSystem {
        fn the_machinery(&self) -> TheMachinery {
            TheMachinery::ItIsNot(WhatIsMissing::TheSessionBusOrTheNotificationService)
        }
        fn the_permission(&self) -> WhatTheSystemSays {
            WhatTheSystemSays::ThereIsNoPermissionModel
        }
        fn ask_for_permission_now(&self) {}
        fn post(&self, _post: &APost) {}
        fn take_down(&self, _key: &ThePresentationKey) {}
    }

    pub(super) fn the_platform() -> Box<dyn ThePlatform> {
        Box::new(NoSuchSystem)
    }

    pub(super) fn the_activation_route_is_open(_on_click: Box<dyn Fn(String) + Send + Sync>) {}

    pub(super) fn the_machinery_is_made() {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::{Identity, IdentityKind};
    use std::sync::Arc;

    /// A platform that records what it was asked to do, so that **the real
    /// service** can be driven through every shape all three systems have.
    struct WhatTheSystemWasAsked {
        machinery: Mutex<TheMachinery>,
        permission: Mutex<WhatTheSystemSays>,
        asked: Mutex<usize>,
        posted: Mutex<Vec<String>>,
        taken_down: Mutex<Vec<String>>,
        /// What the permission becomes once the request has been made, which
        /// is how a granted and a denied Mac are told apart in a test.
        after_asking: Mutex<WhatTheSystemSays>,
    }

    impl Default for WhatTheSystemWasAsked {
        fn default() -> Self {
            WhatTheSystemWasAsked {
                machinery: Mutex::new(TheMachinery::ItIsInPlace),
                permission: Mutex::new(WhatTheSystemSays::ItIsGranted),
                asked: Mutex::new(0),
                posted: Mutex::new(Vec::new()),
                taken_down: Mutex::new(Vec::new()),
                after_asking: Mutex::new(WhatTheSystemSays::ItIsGranted),
            }
        }
    }

    impl ThePlatform for Arc<WhatTheSystemWasAsked> {
        fn the_machinery(&self) -> TheMachinery {
            *self.machinery.lock().unwrap()
        }
        fn the_permission(&self) -> WhatTheSystemSays {
            *self.permission.lock().unwrap()
        }
        fn ask_for_permission_now(&self) {
            *self.asked.lock().unwrap() += 1;
            *self.permission.lock().unwrap() = *self.after_asking.lock().unwrap();
        }
        fn post(&self, post: &APost) {
            self.posted.lock().unwrap().push(post.key.as_text());
        }
        fn take_down(&self, key: &ThePresentationKey) {
            self.taken_down.lock().unwrap().push(key.as_text());
        }
    }

    fn a_condition() -> ThePresentationKey {
        ThePresentationKey::ACondition(
            Identity::issue(IdentityKind::Condition).expect("the desk has randomness"),
        )
    }

    /// One post, in Design's own two lines for the connection kind: *Old 4080
    /// disconnected* over *Could not reconnect.*
    fn a_post(key: ThePresentationKey) -> APost {
        APost {
            key,
            kind: Kind::AConnectionCouldNotBeRestored,
            said: Prose::new("disconnected"),
            about: Verbatim::new("Old 4080"),
            body: Prose::new("Could not reconnect."),
            on: Verbatim::new(""),
            picture: ThePicture::None(picture::WhyThereIsNoPicture::ThisSystemTakesNone),
        }
    }

    /// A service whose record has answered with every kind at its default.
    fn a_service() -> (Arc<WhatTheSystemWasAsked>, TheNotificationService) {
        let system = Arc::new(WhatTheSystemWasAsked::default());
        let service = TheNotificationService::on(Box::new(system.clone()));
        service.the_switches_are(Some(TheSwitches::nobody_has_turned_one()));
        (system, service)
    }

    /// **T17a's third bullet and T7 together: a kind the user switched off is
    /// not attempted, and it never brings up the permission request.**
    ///
    /// **This is the line that fails if the switch is asked after the
    /// permission step.** The system has been asked nothing yet, which is every
    /// first post on a Mac; the switched-off kind must leave the asking count at
    /// nought, and the machinery unread, and a kind still on must go on as
    /// before and be the one that asks.
    #[test]
    fn a_switched_off_kind_is_not_attempted_and_asks_for_no_permission() {
        let (system, service) = a_service();
        *system.permission.lock().unwrap() = WhatTheSystemSays::NothingHasBeenAskedYet;
        service.the_switches_are(Some(
            TheSwitches::nobody_has_turned_one().with(Kind::AConnectionCouldNotBeRestored, false),
        ));

        service.post(a_post(a_condition()));
        assert_eq!(*system.asked.lock().unwrap(), 0, "a switched-off kind asked for permission");
        assert!(system.posted.lock().unwrap().is_empty());

        let finished = APost {
            kind: Kind::AJobFinished,
            ..a_post(a_condition())
        };
        service.post(finished);
        assert_eq!(*system.asked.lock().unwrap(), 1, "a kind still on did not ask");
        assert_eq!(system.posted.lock().unwrap().len(), 1);
    }

    /// **The switch is asked before the machinery too**, so a switched-off kind
    /// reaches nothing of the system's at all. A platform whose machinery read
    /// panics stands in for any reach.
    #[test]
    fn a_switched_off_kind_reaches_nothing_of_the_system() {
        struct NothingMayBeTouched;
        impl ThePlatform for NothingMayBeTouched {
            fn the_machinery(&self) -> TheMachinery {
                panic!("the machinery was read for a kind the user switched off")
            }
            fn the_permission(&self) -> WhatTheSystemSays {
                panic!("the permission was read for a kind the user switched off")
            }
            fn ask_for_permission_now(&self) {
                panic!("permission was asked for a kind the user switched off")
            }
            fn post(&self, _post: &APost) {
                panic!("a switched-off kind was posted")
            }
            fn take_down(&self, _key: &ThePresentationKey) {}
        }
        let service = TheNotificationService::on(Box::new(NothingMayBeTouched));
        let mut switches = TheSwitches::nobody_has_turned_one();
        for kind in Kind::ALL {
            switches = switches.with(kind, false);
        }
        service.the_switches_are(Some(switches));
        for kind in Kind::ALL {
            service.post(APost {
                kind,
                ..a_post(a_condition())
            });
        }
    }

    /// **A record that has not answered attempts nothing** — neither a post
    /// nor the request. And once it answers, the kinds it holds on go.
    #[test]
    fn nothing_is_attempted_until_the_record_has_answered() {
        let system = Arc::new(WhatTheSystemWasAsked::default());
        *system.permission.lock().unwrap() = WhatTheSystemSays::NothingHasBeenAskedYet;
        let service = TheNotificationService::on(Box::new(system.clone()));

        service.post(a_post(a_condition()));
        service.the_switches_are(None);
        service.post(a_post(a_condition()));
        assert_eq!(*system.asked.lock().unwrap(), 0);
        assert!(system.posted.lock().unwrap().is_empty());

        service.the_switches_are(Some(TheSwitches::nobody_has_turned_one()));
        service.post(a_post(a_condition()));
        assert_eq!(system.posted.lock().unwrap().len(), 1);
    }

    /// **T5 and T2 and T3 together: no machinery, no post, no failure
    /// record.**
    ///
    /// This is the line that fails if somebody makes a missing notification
    /// service into something Smoozy reports. §22 item 12: **it is written as
    /// a case rather than as a failure**, because a failure record about the
    /// user's own desktop is not a fact about Smoozy.
    #[test]
    fn no_machinery_posts_nothing_and_says_nothing() {
        let (system, service) = a_service();
        *system.machinery.lock().unwrap() =
            TheMachinery::ItIsNot(WhatIsMissing::TheSessionBusOrTheNotificationService);
        service.post(a_post(a_condition()));
        assert!(system.posted.lock().unwrap().is_empty());
        assert_eq!(*system.asked.lock().unwrap(), 0);
    }

    /// **T7: the request is made at the first moment a post would happen, and
    /// never at application start.**
    ///
    /// Making the service asks nothing. The first post asks once. The second
    /// post does not ask again.
    #[test]
    fn the_request_happens_at_the_first_post_and_once() {
        let (system, service) = a_service();
        *system.permission.lock().unwrap() = WhatTheSystemSays::NothingHasBeenAskedYet;
        *system.after_asking.lock().unwrap() = WhatTheSystemSays::ItIsGranted;

        assert_eq!(*system.asked.lock().unwrap(), 0, "making it asks nothing");

        let key = a_condition();
        service.post(a_post(key.clone()));
        assert_eq!(*system.asked.lock().unwrap(), 1);
        assert_eq!(system.posted.lock().unwrap().len(), 1);

        service.post(a_post(key));
        assert_eq!(*system.asked.lock().unwrap(), 1, "asked once in a run");
        assert_eq!(system.posted.lock().unwrap().len(), 2);
    }

    /// **T6 and T8: a denial is silent and the kinds simply do not arrive.**
    ///
    /// The request is made, the answer is no, and **nothing is posted, nothing
    /// is said, and nothing else changes.**
    #[test]
    fn a_denial_is_silent_and_the_kinds_do_not_arrive() {
        let (system, service) = a_service();
        *system.permission.lock().unwrap() = WhatTheSystemSays::NothingHasBeenAskedYet;
        *system.after_asking.lock().unwrap() = WhatTheSystemSays::ItWasDenied;

        service.post(a_post(a_condition()));
        assert_eq!(*system.asked.lock().unwrap(), 1);
        assert!(system.posted.lock().unwrap().is_empty());

        // And it is not asked a second time on the next post.
        service.post(a_post(a_condition()));
        assert_eq!(*system.asked.lock().unwrap(), 1);
        assert!(system.posted.lock().unwrap().is_empty());
    }

    /// **T7's ruling: the request and the post it was made for are one act.**
    ///
    /// Where the answer has not arrived — which is every first post on a Mac,
    /// the prompt being read by a person — the post is attempted rather than
    /// dropped. **A denial is silent** (T6), so the cost of attempting under
    /// an unanswered prompt is nothing, and the cost of dropping it is the one
    /// notification the request was made for.
    #[test]
    fn an_answer_that_has_not_arrived_is_not_a_refusal() {
        let (system, service) = a_service();
        *system.permission.lock().unwrap() = WhatTheSystemSays::NothingHasBeenAskedYet;
        *system.after_asking.lock().unwrap() = WhatTheSystemSays::NothingHasBeenAskedYet;

        service.post(a_post(a_condition()));
        assert_eq!(*system.asked.lock().unwrap(), 1);
        assert_eq!(system.posted.lock().unwrap().len(), 1);
    }

    /// **Windows: the setting is read and never asked.**
    #[test]
    fn a_system_with_no_request_interface_is_never_asked() {
        let (system, service) = a_service();
        *system.permission.lock().unwrap() = WhatTheSystemSays::TheSystemHasItSwitchedOff;
        service.post(a_post(a_condition()));
        assert_eq!(*system.asked.lock().unwrap(), 0);
        assert!(system.posted.lock().unwrap().is_empty());

        *system.permission.lock().unwrap() = WhatTheSystemSays::ItIsGranted;
        service.post(a_post(a_condition()));
        assert_eq!(*system.asked.lock().unwrap(), 0);
        assert_eq!(system.posted.lock().unwrap().len(), 1);
    }

    /// **Linux: no permission model at all, so the post is attempted and
    /// nothing is ever asked.**
    #[test]
    fn a_system_with_no_permission_model_posts_without_asking() {
        let (system, service) = a_service();
        *system.permission.lock().unwrap() = WhatTheSystemSays::ThereIsNoPermissionModel;
        service.post(a_post(a_condition()));
        assert_eq!(*system.asked.lock().unwrap(), 0);
        assert_eq!(system.posted.lock().unwrap().len(), 1);
    }

    /// **T24: replace-by-identity is the mechanism, so the same key twice is
    /// the same post twice and never two posts.**
    ///
    /// What the systems do with a replacement on a used identity — whether it
    /// re-alerts, or restores one the user dismissed — **is not established on
    /// any of the three** (§22 item 27), and nothing here depends on the
    /// answer: §14 Q13 removes the dependence by making a repeat resolving to
    /// a standing identity need no post at all.
    #[test]
    fn the_same_key_posts_on_the_same_identity_every_time() {
        let (system, service) = a_service();
        let key = a_condition();
        service.post(a_post(key.clone()));
        service.post(a_post(key.clone()));
        let posted = system.posted.lock().unwrap().clone();
        assert_eq!(posted, vec![key.as_text(), key.as_text()]);
    }

    /// **T24a and T24b: the take-down fires at each of the record's three
    /// endings, on the same identity Q10 replaces on.**
    #[test]
    fn a_condition_post_is_taken_down_at_every_one_of_the_records_three_endings() {
        for ending in the_condition_records_own_endings() {
            let (system, service) = a_service();
            let key = a_condition();
            service.post(a_post(key.clone()));
            service.the_condition_record_ended(&key, ending);
            assert_eq!(
                system.taken_down.lock().unwrap().clone(),
                vec![key.as_text()],
                "{ending:?}"
            );
        }
        assert_eq!(the_condition_records_own_endings().len(), 3);
    }

    /// **§14 Q3: endings are kept out of this machinery.**
    ///
    /// A job's ending happened once and has nothing to withdraw, so a
    /// take-down aimed at one is refused rather than performed.
    #[test]
    fn an_ending_is_never_taken_down() {
        let (system, service) = a_service();
        let key = ThePresentationKey::AJobsEnding(
            Identity::issue(IdentityKind::Job).expect("the desk has randomness"),
        );
        service.post(a_post(key.clone()));
        // Under `debug_assert` this is the failing branch, so it is checked
        // the way the release build reaches it.
        if !cfg!(debug_assertions) {
            service.the_condition_record_ended(&key, HowTheRecordEnded::TheUserClearedIt);
            assert!(system.taken_down.lock().unwrap().is_empty());
        }
    }

    /// **T22 and T23, held on the signature rather than on a promise.**
    ///
    /// `post` and `the_condition_record_ended` both answer with nothing, so
    /// there is no result for any state to depend on. This is the line that
    /// fails the moment somebody gives either of them a return value, which is
    /// the natural thing to write and the one thing T23 forbids.
    #[test]
    fn a_post_is_attempted_and_forgotten_and_answers_nothing() {
        let source = include_str!("mod.rs");
        for signature in [
            "pub fn post(&self, post: APost) {",
            "fn post(&self, post: &APost);",
            "fn take_down(&self, key: &ThePresentationKey);",
        ] {
            assert!(
                source.contains(signature),
                "T23: a post is attempted and forgotten and no state depends on the attempt's \
                 result — `{signature}` is no longer in this file"
            );
        }
        assert!(NOTHING_IS_ADVANCED_BY_A_POST);
        assert!(NOTHING_IS_ASSUMED_COALESCED);
        assert!(GROUPING_IS_NOT_RELIED_ON);
    }

    /// **The first line is the name and then what happened, and the name is
    /// untouched** — F-3, X7 and X2.
    ///
    /// **The join is the caller's and not the point's.** Composing one sentence
    /// and handing *that* to the typography point would put a substituted
    /// character inside a name Smoozy is forbidden to alter; joining two strings
    /// that have each already taken their own route cannot, and **X2 binds the
    /// point rather than the caller.** This is the line that fails if somebody
    /// puts the join back through the point.
    #[test]
    fn the_first_line_is_the_name_then_what_happened_and_the_name_is_untouched() {
        // A name with a space in it, because a name is exactly where the point's
        // substitution would show and exactly where it is forbidden.
        let post = APost {
            key: a_condition(),
            kind: Kind::AConnectionCouldNotBeRestored,
            said: Prose::new("disconnected"),
            about: Verbatim::new("Old 4080"),
            body: Prose::new("Could not reconnect."),
            on: Verbatim::new(""),
            picture: ThePicture::None(picture::WhyThereIsNoPicture::ThisSystemTakesNone),
        };
        assert_eq!(post.headline(), "Old 4080 disconnected");
        assert!(
            !post.headline().contains('\u{00A0}'),
            "a substitution reached the name. **X7: a non-breaking space put \
             inside a name is a name Smoozy quietly changed** — the join happens \
             after each half has taken its own route, and nothing may put the \
             joined string back through the point"
        );

        // **A post with no name is its prose alone**, with no separator left
        // hanging; nothing is invented to fill the gap.
        let nameless = APost {
            about: Verbatim::new(""),
            ..post
        };
        assert_eq!(nameless.headline(), "disconnected");
    }

    /// **T2 and T3: the identifier the artifact is built with is the one the
    /// code names.**
    ///
    /// A fact in two places is the fault this stage keeps meeting. On macOS
    /// the bundle carries it and posting fails outright without it; on Windows
    /// the Start-menu shortcut carries it and the toast names it, and a
    /// mismatch there is a post that goes nowhere with nothing saying why.
    #[test]
    fn the_application_identifier_is_the_one_the_artifact_is_built_with() {
        let configuration = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tauri.conf.json"),
        )
        .expect("the shell configuration is readable");
        assert!(
            configuration.contains(&format!("\"identifier\": \"{THE_APPLICATION_IDENTIFIER}\"")),
            "T2 and T3: the bundle identifier and the Windows application identifier are the \
             same value, and tauri.conf.json no longer carries {THE_APPLICATION_IDENTIFIER}"
        );
    }

    /// **The service on this computer's own system builds and answers**, which
    /// is the one thing about the per-system half that can be checked without
    /// posting anything.
    ///
    /// **At the desk this runs unbundled**, so on a Mac it reports T2's
    /// missing bundle identity — which is the precondition doing its job and
    /// is why the test harness is not killed at exit 134.
    #[test]
    fn this_computers_own_service_reads_its_machinery_without_posting() {
        let service = TheNotificationService::for_this_system();
        let machinery = service.the_machinery();
        if cfg!(target_os = "macos") {
            assert_eq!(
                machinery,
                TheMachinery::ItIsNot(WhatIsMissing::TheBundleIdentity),
                "T2: a test binary is not a bundle, so the macOS precondition is not met and \
                 nothing may be called — an unbundled executable dies on the very first call \
                 at exit 134 with nothing to catch"
            );
        }
        let _ = service.the_permission();
    }

    /// **The Windows property value's string is allocated by the allocator
    /// that frees it, and this test exists because the alternative killed a
    /// whole hosted run.**
    ///
    /// Run `34690208297` died at `STATUS_HEAP_CORRUPTION` with **not one test
    /// reported as failed**, because
    /// [`this_computers_own_service_reads_its_machinery_without_posting`]
    /// above reaches the Windows Start-menu shortcut, and the shortcut handed
    /// the property value a pointer into a Rust `Vec`. **A `PROPVARIANT` is
    /// cleared when it goes out of scope and clearing a `VT_LPWSTR` frees its
    /// string with `CoTaskMemFree`** — so the buffer was freed by the
    /// component system and then again by Rust.
    ///
    /// **It is read out of the source rather than run**, and it is here rather
    /// than in `windows.rs` for the reason `reveal.rs`'s own Windows check
    /// gives: nobody at this desk can see the Windows build, so a Windows-only
    /// test is one nobody watches fail. This one fails at the desk.
    #[test]
    fn the_windows_property_value_owns_a_string_the_component_system_allocated() {
        let source = include_str!("windows.rs");
        let working: String = source
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            working.contains("CoTaskMemAlloc(std::mem::size_of_val(&identifier[..])) as *mut u16"),
            "the Windows shortcut no longer allocates the property value's string with \
             `CoTaskMemAlloc`. **Clearing a `VT_LPWSTR` frees its string with `CoTaskMemFree`**, \
             so any other allocator here is a heap corruption — it exited run 34690208297 at \
             `STATUS_HEAP_CORRUPTION` with nothing reported as failed"
        );
        assert_eq!(
            working.matches("pwszVal").count(),
            1,
            "a second string is being put into a `PROPVARIANT` in the Windows notification code. \
             **Every one of them is freed by `CoTaskMemFree` when the value is cleared**, so \
             every one of them has to come from `CoTaskMemAlloc`"
        );
        assert!(
            !working.contains("as_mut_ptr()"),
            "the Windows notification code hands a Rust buffer's own pointer to a call. **That \
             is what corrupted the heap in run 34690208297**: a pointer given away here is \
             freed by the component system's allocator and then again by Rust"
        );
    }
}
