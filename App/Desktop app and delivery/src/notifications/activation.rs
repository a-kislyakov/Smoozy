//! **What clicking a notification does** — `Desktop app and delivery` §13.7,
//! T28, T29 and T30.
//!
//! **What travels with the post is identities only** — never state, never a
//! path, and never a message re-derived at click time (T29). On activation the
//! application brings its window forward and goes to that subject's own
//! record, **which exists whatever became of the notification** because every
//! one of the four kinds also leaves its own durable in-app record (T22).
//! **That is what makes this a routing question rather than a storage one.**
//!
//! **The width is the running application** (T30). The spec's own bound is
//! that a notification reaches the user while Smoozy runs behind their work
//! and never once they have quit it, so **nothing here is built on a cold
//! start**, and §22 item 51 carries the residue: macOS documents delivery to
//! an exited application, the shell's Windows path from a click on a closed
//! application to its callback is established by no publication, and Linux is
//! undefined by publication.
//!
//! **T22 and T23 stand untouched.** Nothing is advanced, cleared, acknowledged
//! or recorded by the activation, and no state depends on it.

use std::collections::HashMap;

use smoozy_state::identities::Identity;
use tauri::{AppHandle, Manager, Runtime};

use crate::events::{Event, Posted, Said, Subject, TheOneQueue};
use crate::presentation::the_presentation_key::ThePresentationKey;
use crate::typography::Verbatim;

/// **The user clicked a notification and Smoozy is going to what it is
/// about.**
///
/// The kind is declared beside the service that posts it, which is the command
/// surface's own rule. **No page module names it yet**, the job history entry
/// and the server row being screens no step has built; the event is posted all
/// the same, because T29's destination is the in-app record and the route to
/// it is this one.
pub const THE_USER_CLICKED_A_NOTIFICATION: &str = "notifications:a-going";

/// **What travels with a post: identities and nothing else** — T29.
///
/// The job identity or group identity for the two job kinds, the server row
/// for the connection kind and the idle kind. **Never state, never a path,
/// never a message re-derived at click time.**
///
/// It is a type rather than a convention so that the rule is kept by what can
/// be put in it: there is no field a sentence, a path or a reading could go in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTravels {
    /// **The durable subject the notification is about.**
    pub subject: Identity,
}

/// **What the systems hand back on a click, and what it is looked up in.**
///
/// **All three hand back the post's own identity and an action**, normalised
/// by each of the three routes to one thing here (T28): macOS calls a
/// notification-centre delegate with a response object, Windows activates the
/// registered component server, and Linux emits an action-invoked signal to
/// the client that posted. **A bare click on the notification body is itself
/// an action on all three.**
///
/// **The register is run-held and holds identities only.** It is not a store:
/// nothing durable is written, a fresh launch holds none of it, and losing it
/// costs the going and nothing else — which is T22 and T30 together, the going
/// being all a click ever promised.
#[derive(Default)]
pub struct WhatEachPostIsAbout {
    per_post: HashMap<String, WhatTravels>,
}

impl WhatEachPostIsAbout {
    pub fn new() -> Self {
        Self::default()
    }

    /// Remembered at the moment the post is attempted.
    pub fn a_post_was_attempted(&mut self, key: &ThePresentationKey) {
        self.per_post.insert(
            key.as_text(),
            WhatTravels {
                subject: key.the_subject().clone(),
            },
        );
    }

    /// **Forgotten when the post is taken down** (T24a), so a click on a
    /// notification the system did not manage to remove finds nothing and gets
    /// the bringing-forward alone.
    pub fn a_post_was_taken_down(&mut self, key: &ThePresentationKey) {
        self.per_post.remove(&key.as_text());
    }

    /// What a post identity handed back by the system is about, where this run
    /// posted it.
    pub fn what_this_post_is_about(&self, the_post_identity: &str) -> Option<&WhatTravels> {
        self.per_post.get(the_post_identity)
    }
}

/// **T29, performed: bring the window forward, then go to the subject.**
///
/// **The bringing-forward happens whatever the register says.** A subject that
/// is gone by the time of the click — or a post this run did not make, which
/// is T30's cold-start case arriving by a different door — **gets the
/// bringing-forward and nothing else**, no error and no invented screen,
/// because the click's promise is the going and a gone subject has nowhere to
/// go.
///
/// **The going itself is a read rather than a write.** P7 forbids an event
/// carrying state that has not already been written durably, and what is read
/// here is the register's own record of what this run posted — identities that
/// were written durably by whoever created the subject, long before the post.
/// **Nothing is composed from the notification's own text**, which is T29's
/// *never a message re-derived at click time*.
pub fn a_notification_was_clicked<R: Runtime>(
    app: &AppHandle<R>,
    queue: &TheOneQueue,
    register: &WhatEachPostIsAbout,
    the_post_identity: &str,
) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
    let _ = the_going(queue, register, the_post_identity);
}

/// The going, on the one ordered queue — P6.
///
/// **P8 is what keeps T29's last sentence, and it keeps it for nothing.** The
/// event is about the durable subject, so a subject the queue has been told is
/// gone has its event discarded and the user is left with the window forward
/// and no navigation — which is exactly *a subject that is gone by the time of
/// the click gets the bringing-forward and nothing else*.
fn the_going(
    queue: &TheOneQueue,
    register: &WhatEachPostIsAbout,
    the_post_identity: &str,
) -> Result<Posted, ()> {
    queue.read_then_post(
        || {
            register
                .what_this_post_is_about(the_post_identity)
                .cloned()
                .ok_or(())
        },
        |travels| {
            Event::about(
                Subject::Durable(travels.subject.clone()),
                THE_USER_CLICKED_A_NOTIFICATION,
            )
            // An identity is a value, not prose — X7 and X9. It is what the
            // page matches a record on and nothing about it is readable.
            .saying(
                "subject",
                Said::Value(Verbatim::new(travels.subject.as_text())),
            )
        },
    )
}

/// **T30: nothing is built on a cold start.**
///
/// macOS documents delivery to an exited application and the shell assigns its
/// delegate during launch, so such an activation is met as an ordinary launch
/// and routed under T29. **Windows has the mechanism by publication but the
/// path from a click on a closed application to a callback is established by
/// no publication and is treated as unproven. Linux is undefined by
/// publication** — the specification describes a signal to a connected client
/// and nothing else.
///
/// **So the register above is run-held on purpose rather than for want of a
/// store.** A post this run did not make is a post this run cannot route, and
/// that is the honest width of the promise rather than a gap to fill with a
/// durable table.
pub const NOTHING_IS_BUILT_ON_A_COLD_START: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::IdentityKind;

    fn a_key() -> ThePresentationKey {
        ThePresentationKey::AJobsEnding(
            Identity::issue(IdentityKind::Job).expect("the desk has randomness"),
        )
    }

    /// **T29: identities only, and there is nowhere else for anything to go.**
    ///
    /// Read over this module's own source rather than asserted on a value,
    /// because the fault is a field somebody adds — a path so the click can
    /// open a folder, a sentence so the page need not look one up — and the
    /// type is the whole of what forbids it.
    #[test]
    fn nothing_but_an_identity_travels_with_a_post() {
        let source = include_str!("activation.rs");
        let struct_at = source
            .find("pub struct WhatTravels {")
            .expect("WhatTravels is a struct");
        let body = &source[struct_at..];
        let body = &body[..body.find('}').expect("the struct closes")];
        let fields: Vec<&str> = body
            .lines()
            .skip(1)
            .filter(|line| line.trim_start().starts_with("pub "))
            .collect();
        assert_eq!(
            fields.len(),
            1,
            "T29: what travels with the post is identities only — never state, never a path and \
             never a message re-derived at click time; WhatTravels has grown a second field: \
             {fields:?}"
        );
        assert!(fields[0].contains("Identity"));
    }

    /// The register answers for a post this run made, and for nothing else.
    #[test]
    fn a_post_this_run_did_not_make_is_about_nothing() {
        let mut register = WhatEachPostIsAbout::new();
        let key = a_key();
        register.a_post_was_attempted(&key);
        assert!(register.what_this_post_is_about(&key.as_text()).is_some());
        assert!(register
            .what_this_post_is_about("job:0123456789abcdef0123456789abcdef")
            .is_none());
    }

    /// **T24a's other end: a taken-down post is forgotten**, so a click on one
    /// the system did not manage to remove routes nowhere rather than to a
    /// record whose reason for existing has ended.
    #[test]
    fn a_taken_down_post_is_forgotten() {
        let mut register = WhatEachPostIsAbout::new();
        let key = a_key();
        register.a_post_was_attempted(&key);
        register.a_post_was_taken_down(&key);
        assert!(register.what_this_post_is_about(&key.as_text()).is_none());
    }

    /// **T29's last sentence, without a window: a subject the queue has not
    /// been told about posts nothing.**
    ///
    /// P8's register fails closed, so the going is discarded and the user gets
    /// the bringing-forward alone. **That is also why nothing routes today**:
    /// whoever creates a durable subject calls `the_subject_exists`, and the
    /// pieces that own job records and server rows have not been built.
    #[test]
    fn a_going_about_a_subject_the_queue_does_not_know_is_discarded() {
        let queue = TheOneQueue::new();
        let mut register = WhatEachPostIsAbout::new();
        let key = a_key();
        register.a_post_was_attempted(&key);

        assert_eq!(
            the_going(&queue, &register, &key.as_text()),
            Ok(Posted::TheSubjectIsGone)
        );

        // Told the subject exists, the same going is queued in order.
        queue.the_subject_exists(key.the_subject().clone());
        assert_eq!(
            the_going(&queue, &register, &key.as_text()),
            Ok(Posted::InOrder(0))
        );
    }

    /// A click on a post nothing in this run made composes no event at all —
    /// the reading door refuses and there is nothing to compose from.
    #[test]
    fn an_unknown_post_composes_no_event() {
        let queue = TheOneQueue::new();
        let register = WhatEachPostIsAbout::new();
        assert_eq!(the_going(&queue, &register, "job:whatever"), Err(()));
        assert!(NOTHING_IS_BUILT_ON_A_COLD_START);
    }
}
