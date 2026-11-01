//! **The macOS third** — `Desktop app and delivery` §13, T2, T6, T7, T16's
//! attachment, T24, T24a and T28.
//!
//! **It is platform code of this document's own, beside the shell**, the same
//! shape `src/reveal.rs` and `src/platform/` take. The shell reaches none of
//! it: what it publishes is a plugin whose desktop half answers *granted* to
//! every permission question without asking anything, carries no per-post
//! identity to replace on, offers no attachment and no removal, and has no
//! activation callback at all — so T6, T7, T24, T24a and T29 would every one
//! of them be unkept.
//!
//! # The precondition, and it is the one that kills the process
//!
//! **T2: the application must be a bundle with an identifier.** Apple
//! documents that precondition **nowhere** and it is decisive: an unbundled
//! executable is killed by an uncaught exception on the **very first call,
//! exit 134, with nothing to catch**; inside a bundle with an identifier it
//! works; remove the identifier and it dies with a different exception. **An
//! ad-hoc signature is enough** — no Developer ID and no notarization.
//!
//! **So [`TheMac::the_bundle_identity`] is read before anything else in this
//! file touches the notification centre**, and every route below goes through
//! it. That is not tidiness: `cargo test` runs this crate as a plain
//! executable with no bundle, so a call made without the guard would take the
//! whole test binary down at exit 134 with no failure anybody could read.
//!
//! # What is observed and what is not
//!
//! §22 item 11: **no notification had ever been posted and no permission ever
//! requested, deliberately, because the machine is the Creator's.** What was
//! observed before this file is the precondition, to the point of the process
//! being killed, and the permission model, which is documented. **Image
//! rendering, banner placement and the exact error returned when a request has
//! been denied are unobserved even here.**

use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::OnceLock;

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, AllocAnyThread};
use objc2_foundation::{NSArray, NSBundle, NSError, NSObject, NSObjectProtocol, NSString, NSURL};
use objc2_user_notifications::{
    UNAuthorizationOptions, UNAuthorizationStatus, UNMutableNotificationContent, UNNotification,
    UNNotificationAttachment, UNNotificationPresentationOptions, UNNotificationRequest,
    UNNotificationResponse, UNNotificationSettings, UNUserNotificationCenter,
    UNUserNotificationCenterDelegate,
};

use crate::presentation::the_presentation_key::ThePresentationKey;
use super::{APost, TheMachinery, ThePlatform, WhatIsMissing};
use crate::notifications::permission::WhatTheSystemSays;
use crate::notifications::picture;

/// **T6: the state, readable back.**
///
/// Apple's own read is asynchronous, so what is kept here is the last answer
/// the system gave. **A denial is silent** — the system *"ignores any attempts
/// to schedule local notifications"* — so this cell is the only way Smoozy can
/// know at all, and nothing about a refusal ever arrives at a post.
///
/// **Nothing in this file reads it without having waited for an answer first.**
/// It used to be read straight after the asynchronous read was started, which
/// made every read the previous read's answer; [`TheMac::read_the_state_back`]
/// carries what that cost. What it is now is the request's own answer as well as
/// the read's, and a last resort where a channel closes with nothing sent.
static THE_LAST_ANSWER: AtomicU8 = AtomicU8::new(NOT_DETERMINED);

const NOT_DETERMINED: u8 = 0;
const AUTHORISED: u8 = 1;
const DENIED: u8 = 2;

/// **T28: where a click goes.** Set once, during launch, by
/// [`super::the_activation_route_is_open`].
static WHAT_A_CLICK_DOES: OnceLock<Box<dyn Fn(String) + Send + Sync>> = OnceLock::new();

pub(super) struct TheMac;

pub(super) fn the_platform() -> Box<dyn ThePlatform> {
    Box::new(TheMac)
}

/// **There is nothing to make on this system.** T2's precondition is the bundle
/// and its identifier, which the shipped artifact either is or is not; it is a
/// build output rather than something the application can create at run time.
/// **The empty body is the clause** — see
/// [`super::the_machinery_is_made_at_first_start`].
pub(super) fn the_machinery_is_made() {}

impl TheMac {
    /// **T2, and every route in this file goes through it.**
    ///
    /// `None` means there is no bundle, or a bundle with no identifier, and in
    /// either case **nothing else here may be called**.
    fn the_bundle_identity() -> Option<Retained<NSString>> {
        NSBundle::mainBundle().bundleIdentifier()
    }

    /// The notification centre, or nothing where T2's precondition is not met.
    fn the_centre() -> Option<Retained<UNUserNotificationCenter>> {
        Self::the_bundle_identity()?;
        Some(UNUserNotificationCenter::currentNotificationCenter())
    }

    /// **T6's read, waited for.**
    ///
    /// **Repaired 2026-09-12, step 10a round 1 must-do F-7.** Until then this
    /// started the read and the caller then took [`THE_LAST_ANSWER`] **before
    /// the block could have run**, so the first read in any run said *nothing
    /// has been asked yet* whatever the Mac actually said, and every later read
    /// answered the read before it. **T6 says the state is readable back and T8
    /// says a denied state is read back where the system offers one; that kept
    /// neither.**
    ///
    /// **So it waits, and §3.2 P5 is what makes waiting allowed**: every
    /// notification call runs off the interface thread, so the one thread that
    /// must never block never reaches this. **Apple hands the settings to a
    /// block on a queue of its own**, which is why waiting here does not wait on
    /// the thread that would answer.
    ///
    /// **The cell is still written**, so a caller that cannot wait has the last
    /// answer rather than nothing — but nothing in this file reads it without
    /// having waited first.
    fn read_the_state_back(centre: &UNUserNotificationCenter) -> u8 {
        let (say, heard) = std::sync::mpsc::channel();
        let handler = RcBlock::new(move |settings: std::ptr::NonNull<UNNotificationSettings>| {
            // Safety: the system hands the settings object to this block for
            // the length of the call and nothing here outlives it.
            let settings = unsafe { settings.as_ref() };
            let status = settings.authorizationStatus();
            let answer = the_answer(status);
            THE_LAST_ANSWER.store(answer, Ordering::Relaxed);
            let _ = say.send(answer);
        });
        centre.getNotificationSettingsWithCompletionHandler(&handler);
        heard
            .recv()
            .unwrap_or_else(|_| THE_LAST_ANSWER.load(Ordering::Relaxed))
    }
}

/// Apple's five statuses, read into the three this document has a use for.
///
/// **Provisional and ephemeral are read as authorised**, which is what they
/// are: a post is attempted and the system decides how it appears. Nothing in
/// §13 turns on which of the three permitted forms a Mac is in.
fn the_answer(status: UNAuthorizationStatus) -> u8 {
    if status == UNAuthorizationStatus::Denied {
        DENIED
    } else if status == UNAuthorizationStatus::NotDetermined {
        NOT_DETERMINED
    } else {
        AUTHORISED
    }
}

impl ThePlatform for TheMac {
    /// **T2.**
    fn the_machinery(&self) -> TheMachinery {
        match TheMac::the_bundle_identity() {
            Some(_) => TheMachinery::ItIsInPlace,
            None => TheMachinery::ItIsNot(WhatIsMissing::TheBundleIdentity),
        }
    }

    /// **T6: readable back**, and reading never asks.
    fn the_permission(&self) -> WhatTheSystemSays {
        let Some(centre) = TheMac::the_centre() else {
            // T2's precondition is not met, so there is nothing to read and
            // nothing may be called to find out.
            return WhatTheSystemSays::NothingHasBeenAskedYet;
        };
        match TheMac::read_the_state_back(&centre) {
            AUTHORISED => WhatTheSystemSays::ItIsGranted,
            DENIED => WhatTheSystemSays::ItWasDenied,
            _ => WhatTheSystemSays::NothingHasBeenAskedYet,
        }
    }

    /// **T7: the request, made at the first moment a post would happen.**
    ///
    /// **The options are the alert and nothing else**, ruled here under the
    /// standing delegation of 2026-08-31: §13 asks for four things the user
    /// reads while working elsewhere, and a badge on an icon and a sound are
    /// each a second surface nothing in the plans asks for. **Adding one later
    /// re-prompts nobody** — the system remembers the grant — so the narrower
    /// ask costs nothing that cannot be widened.
    ///
    /// **It does not wait for the answer.** The prompt is drawn by the system
    /// and answered by a person; a call that blocked on it would block
    /// whichever thread posted.
    fn ask_for_permission_now(&self) {
        let Some(centre) = TheMac::the_centre() else {
            return;
        };
        let handler = RcBlock::new(|granted: objc2::runtime::Bool, _error: *mut NSError| {
            THE_LAST_ANSWER.store(
                if granted.as_bool() { AUTHORISED } else { DENIED },
                Ordering::Relaxed,
            );
        });
        centre.requestAuthorizationWithOptions_completionHandler(
            UNAuthorizationOptions::Alert,
            &handler,
        );
    }

    /// **T23: attempted and forgotten.**
    ///
    /// **T24's replace-by-identity is Apple's own rule for the request
    /// identifier**: adding a request whose identifier is already in use
    /// replaces it. So the identifier **is** the presentation key, and there
    /// is no second thing to keep in step. **Whether that replacement
    /// re-alerts the user, or restores one they dismissed, is not established**
    /// (§22 item 27) and nothing here depends on the answer.
    fn post(&self, post: &APost) {
        let Some(centre) = TheMac::the_centre() else {
            return;
        };
        // **Design's two lines, in the system's own two fields.** The first is
        // the name and what happened, joined after each half has taken its own
        // route; the second is the figures, or nothing where they are owed.
        let content = UNMutableNotificationContent::new();
        content.setTitle(&NSString::from_str(&post.headline()));
        content.setBody(&NSString::from_str(&post.the_second_line()));

        // **T16: an attachment, and it must be a file on disk.** There is no
        // in-memory supply, and the fetched derivative is already a file and
        // already PNG or JPEG, so the format holds by construction. The size
        // was checked against the 10 MB ceiling before this was composed.
        if let Some(path) = picture::as_path(&post.picture) {
            let url = NSURL::fileURLWithPath(&NSString::from_str(&path.to_string_lossy()));
            if let Ok(attachment) = unsafe {
                UNNotificationAttachment::attachmentWithIdentifier_URL_options_error(
                    &NSString::from_str("preview"),
                    &url,
                    None,
                )
            } {
                let attachments = NSArray::from_retained_slice(&[attachment]);
                content.setAttachments(&attachments);
            }
        }

        // A nil trigger is *deliver now*, which is the whole of what §13 asks
        // for: nothing here schedules anything, and §3.3 P10 already says
        // there is exactly one timer in this application and it is the update
        // check.
        let request = UNNotificationRequest::requestWithIdentifier_content_trigger(
            &NSString::from_str(&post.key.as_text()),
            &content,
            None,
        );
        centre.addNotificationRequest_withCompletionHandler(&request, None);
    }

    /// **T24a and T24b: attempted and forgotten.**
    ///
    /// **Both the delivered copy and any pending one**, because a post that
    /// has not yet been shown is as wrong as one that has once its condition
    /// record has ended. §22 item 61 says whether a posted notification can be
    /// taken down at all is **not established on any of the three**, so
    /// nothing waits on either call and nothing records them.
    fn take_down(&self, key: &ThePresentationKey) {
        let Some(centre) = TheMac::the_centre() else {
            return;
        };
        let identifiers = NSArray::from_retained_slice(&[NSString::from_str(&key.as_text())]);
        centre.removeDeliveredNotificationsWithIdentifiers(&identifiers);
        centre.removePendingNotificationRequestsWithIdentifiers(&identifiers);
    }
}

// ── T28: the activation route ───────────────────────────────────────────────

define_class!(
    /// **The notification-centre delegate** — T28.
    ///
    /// **Apple's one hard timing requirement is that the delegate is assigned
    /// before the application finishes launching**, because *"assigning a
    /// delegate after the system calls these methods might cause you to miss
    /// incoming notifications"*. [`the_activation_route_is_open`] is called
    /// from the application's own launch, before the window is built.
    #[unsafe(super(NSObject))]
    #[name = "SmoozyNotificationDelegate"]
    struct TheDelegate;

    unsafe impl NSObjectProtocol for TheDelegate {}

    unsafe impl UNUserNotificationCenterDelegate for TheDelegate {
        /// **A bare click on the notification body is itself an action** —
        /// Apple has a constant for it whose delivery *"doesn't require any
        /// special configuration of notification categories"*.
        ///
        /// **What is read out of the response is the request's identifier and
        /// nothing else** — T29: identities only, never state, never a path,
        /// and never a message re-derived at click time. The identifier is the
        /// presentation key, which is composed from identities alone.
        #[unsafe(method(userNotificationCenter:didReceiveNotificationResponse:withCompletionHandler:))]
        fn did_receive_a_response(
            &self,
            _centre: &UNUserNotificationCenter,
            response: &UNNotificationResponse,
            completion_handler: &block2::DynBlock<dyn Fn()>,
        ) {
            let identifier = response.notification().request().identifier().to_string();
            if let Some(go) = WHAT_A_CLICK_DOES.get() {
                go(identifier);
            }
            // Apple requires the handler be called when the work is done.
            // **Nothing waits on the going** — T23 and T22 both — so it is
            // called straight away.
            (*completion_handler).call(());
        }

        /// **Smoozy is running behind the user's work, which is the whole
        /// width of the promise** (T30), so a notification that arrives while
        /// Smoozy happens to be the front application is still shown.
        /// Without this the system withholds it, and the four kinds would go
        /// missing in exactly the case a user has just switched back.
        #[unsafe(method(userNotificationCenter:willPresentNotification:withCompletionHandler:))]
        fn will_present(
            &self,
            _centre: &UNUserNotificationCenter,
            _notification: &UNNotification,
            completion_handler: &block2::DynBlock<dyn Fn(UNNotificationPresentationOptions)>,
        ) {
            (*completion_handler).call((UNNotificationPresentationOptions::Banner,));
        }
    }
);

/// **T28: the delegate is assigned, once, during launch.**
///
/// **Where T2's precondition is not met nothing is assigned and nothing is
/// called**, which is the same guard every other route in this file takes and
/// for the same reason.
pub(super) fn the_activation_route_is_open(on_click: Box<dyn Fn(String) + Send + Sync>) {
    if WHAT_A_CLICK_DOES.set(on_click).is_err() {
        return;
    }
    let Some(centre) = TheMac::the_centre() else {
        return;
    };
    let delegate: Retained<TheDelegate> = unsafe { msg_send![TheDelegate::alloc(), init] };
    let protocol = ProtocolObject::from_ref(&*delegate);
    centre.setDelegate(Some(protocol));
    // The centre holds the delegate weakly, so the one instance is kept for
    // the life of the run rather than dropped at the end of this function —
    // which is the ordinary shape of a delegate and is the fault that would
    // otherwise show only as clicks silently doing nothing.
    let kept: &'static mut Option<Retained<TheDelegate>> = Box::leak(Box::new(Some(delegate)));
    let _ = kept;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **T2, watched at the desk in the only direction a test can see it.**
    ///
    /// A test binary is not a bundle, so the precondition is not met, and
    /// **that is what stops this test file taking the whole harness down at
    /// exit 134.** The guard is the test.
    #[test]
    fn a_test_binary_is_not_a_bundle_and_nothing_is_called() {
        assert!(TheMac::the_bundle_identity().is_none());
        assert_eq!(
            TheMac.the_machinery(),
            TheMachinery::ItIsNot(WhatIsMissing::TheBundleIdentity)
        );
        // Every route answers without touching the notification centre.
        assert_eq!(
            TheMac.the_permission(),
            WhatTheSystemSays::NothingHasBeenAskedYet
        );
        TheMac.ask_for_permission_now();
        TheMac.take_down(&ThePresentationKey::ACondition(
            smoozy_state::identities::Identity::issue(
                smoozy_state::identities::IdentityKind::Condition,
            )
            .expect("the desk has randomness"),
        ));
    }

    /// **T6: Apple's five statuses read into three, and the reading is here
    /// rather than at the call site.**
    #[test]
    fn apples_statuses_read_into_the_three_this_document_uses() {
        assert_eq!(the_answer(UNAuthorizationStatus::Denied), DENIED);
        assert_eq!(the_answer(UNAuthorizationStatus::NotDetermined), NOT_DETERMINED);
        assert_eq!(the_answer(UNAuthorizationStatus::Authorized), AUTHORISED);
        assert_eq!(the_answer(UNAuthorizationStatus::Provisional), AUTHORISED);
        assert_eq!(the_answer(UNAuthorizationStatus::Ephemeral), AUTHORISED);
    }

    /// **T24: the identifier is the presentation key and there is no second
    /// thing to keep in step.**
    ///
    /// This is the line that fails if somebody composes a separate identifier
    /// for the operating system, which would make replace-by-identity and
    /// take-down disagree in a way nothing else could see.
    #[test]
    fn the_request_identifier_is_the_presentation_key() {
        let source = include_str!("macos.rs");
        assert!(source.contains("&NSString::from_str(&post.key.as_text()),"));
        assert!(source.contains("NSString::from_str(&key.as_text())"));
    }
}
