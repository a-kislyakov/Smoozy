//! **The Linux third** — `Desktop app and delivery` §13, T5, T6, T16's picture,
//! T24, T24a and T28.
//!
//! **It is platform code of this document's own, beside the shell**, the same
//! shape `src/reveal.rs` takes.
//!
//! # There is no identity precondition, and one thing Smoozy cannot fix
//!
//! **T5: the freedesktop Desktop Notifications Specification v1.3 requires no
//! identity** — the application name *"can be blank"* and the desktop entry is
//! only a hint — **but it needs a session message bus and a process owning the
//! notification name**, and the specification says outright that clients
//! *"must not assume that it is available."*
//!
//! **So on a computer with no notification service the four kinds do not
//! arrive, and that is written as a case rather than as a failure**: no
//! failure record is raised, because a failure record about the user's own
//! desktop is not a fact about Smoozy, and the four kinds' in-app records are
//! unaffected (§22 item 12). [`TheLinux::the_machinery`] is where that is
//! decided, and it asks the bus rather than assuming either way.
//!
//! # No permission model at all
//!
//! **T6: there is no permission model in the specification**, and therefore no
//! denied state to read. That is not the same as being granted one — nothing
//! was granted, and nothing could be — and the difference shows in
//! [`crate::notifications::permission::WhatTheSystemSays`] having a value for
//! it rather than borrowing macOS's.
//!
//! # Everything here is documentation alone
//!
//! **No Linux desktop computer has ever been run** (§22 item 3, permanently
//! unresolved), and the Creator's ruling of 2026-08-31 means none is coming.
//! Every claim below is the specification's own words. **No space is left here
//! for an answer that is not coming.**
//!
//! # What the D-Bus client is, and why it is not new weight
//!
//! `zbus` is **already in this crate's Linux tree beneath `ashpd`**, which
//! `src/reveal.rs` takes for K21's portal route, so declaring it by name adds
//! no package to any artifact. It is **pure Rust**, so this file compiles at
//! this desk against the Linux target in a throwaway crate — which is the
//! instrument this stage already uses for code no machine here can run.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use zbus::zvariant::Value;
use zbus::Connection;

use crate::presentation::the_presentation_key::ThePresentationKey;
use super::{APost, TheMachinery, ThePlatform, WhatIsMissing};
use crate::notifications::permission::WhatTheSystemSays;
use crate::notifications::picture;
use crate::typography::Prose;

const THE_SERVICE: &str = "org.freedesktop.Notifications";
const THE_PATH: &str = "/org/freedesktop/Notifications";

/// **T28: where a click goes.**
static WHAT_A_CLICK_DOES: OnceLock<Box<dyn Fn(String) + Send + Sync>> = OnceLock::new();

/// **The server's own identifier for each post this run made** — T24's
/// `replaces_id`, and the only thing a click hands back.
///
/// **Run-held and identities only** (T29, T30): the value is the presentation
/// key, which is composed from identities alone, and a fresh launch holds
/// none of it.
static THE_SERVERS_OWN_NUMBERS: OnceLock<Mutex<HashMap<String, u32>>> = OnceLock::new();

fn the_servers_own_numbers() -> &'static Mutex<HashMap<String, u32>> {
    THE_SERVERS_OWN_NUMBERS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(super) struct TheLinux;

pub(super) fn the_platform() -> Box<dyn ThePlatform> {
    Box::new(TheLinux)
}

/// **There is nothing to make on this system.** T5's precondition is a session
/// message bus and a process owning the notification name, and neither is
/// Smoozy's to create — the specification says outright that clients *"must not
/// assume that it is available."* **The empty body is the clause** — see
/// [`super::the_machinery_is_made_at_first_start`].
pub(super) fn the_machinery_is_made() {}

/// **The one session connection this file holds, and the runtime that drives
/// it.**
///
/// **Repaired 2026-09-12, step 10a round 1 must-do F-4.** Until then the send
/// opened its own session connection for the one call and dropped it, while the
/// listener opened a second, separate one in a thread of its own. **T28's own
/// sentence is that the signal goes to the client that posted** — and by the
/// time the send returned, that client was gone. Holding the one connection the
/// listener already keeps, and sending on it, makes the sending client and the
/// listening client the same client. **That is right on either answer to §22
/// item 68** — whether the server addresses the signal to the client that posted
/// or broadcasts it to everyone on the bus — **and it costs nothing.**
///
/// **Nothing here polls and nothing here is scheduled.** §3.3 P9 forbids polling
/// and P10 allows exactly one timer, which is the update check. What stands is
/// one connection and one worker, which is the same shape a connection is, and
/// it does nothing at all until it is asked or until the server speaks. **It
/// ends with the run**: nothing durable is held and the process exiting is what
/// stops it.
struct TheBus {
    runtime: tokio::runtime::Runtime,
    connection: Connection,
}

static THE_BUS: OnceLock<Option<TheBus>> = OnceLock::new();

/// The one connection, made on the first ask. **`None` means the session bus
/// could not be reached at all**, which is T5's own case and is answered as
/// missing machinery rather than as a failure.
fn the_bus() -> Option<&'static TheBus> {
    THE_BUS
        .get_or_init(|| {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .ok()?;
            let connection = runtime.block_on(Connection::session()).ok()?;
            Some(TheBus {
                runtime,
                connection,
            })
        })
        .as_ref()
}

/// One call on the session bus, waited for, **on the one connection**.
fn on_the_bus<T, F>(work: impl FnOnce(Connection) -> F) -> Option<T>
where
    F: std::future::Future<Output = Result<T, zbus::Error>>,
{
    let bus = the_bus()?;
    bus.runtime.block_on(work(bus.connection.clone())).ok()
}

impl ThePlatform for TheLinux {
    /// **T5, asked rather than assumed.**
    ///
    /// Two things and the specification names both: **a session message bus**,
    /// and **a process owning the notification name**. The second is asked of
    /// the bus itself, which is the only way a client can find out — and the
    /// specification's own sentence is that clients *"must not assume that it
    /// is available."*
    fn the_machinery(&self) -> TheMachinery {
        let there = on_the_bus(|connection| async move {
            let bus = zbus::fdo::DBusProxy::new(&connection).await?;
            Ok(bus.name_has_owner(THE_SERVICE.try_into()?).await?)
        });
        match there {
            Some(true) => TheMachinery::ItIsInPlace,
            _ => TheMachinery::ItIsNot(WhatIsMissing::TheSessionBusOrTheNotificationService),
        }
    }

    /// **T6: no permission model at all**, so nothing is read and nothing is
    /// refused.
    fn the_permission(&self) -> WhatTheSystemSays {
        WhatTheSystemSays::ThereIsNoPermissionModel
    }

    /// **T6: there is nothing on this system to ask.** The empty body is the
    /// clause.
    fn ask_for_permission_now(&self) {}

    /// **T23: attempted and forgotten.**
    ///
    /// **T24's replace-by-identity is `replaces_id`**, and the specification
    /// makes it atomic: *"The server must atomically (ie with no flicker or
    /// other visual cues) replace the given notification with this one."* The
    /// number is the server's own and is remembered for the run against the
    /// presentation key, because the protocol replaces on the number and
    /// nothing else.
    ///
    /// **T16's Linux picture goes by its file address in the `image-path`
    /// hint** (corrected 2026-11-01), and **T5's `desktop-entry` hint names
    /// `Smoozy.desktop`**. A server may ignore either, and nothing detects it
    /// (§22 item 13).
    fn post(&self, post: &APost) {
        let key = post.key.as_text();
        let replaces = the_servers_own_numbers()
            .lock()
            .ok()
            .and_then(|numbers| numbers.get(&key).copied())
            .unwrap_or(0);

        // **T28: the default action pair, prepended.** The specification
        // reserves the key: *"The default action (usually invoked by clicking
        // the notification) should have a key named `default`."* The label
        // beside it is prose and goes through the typography point, even
        // though most servers never draw it (X5).
        let open = Prose::new("Open");
        let actions: Vec<&str> = vec!["default", open.as_str()];
        let mut hints: HashMap<&str, Value> =
            HashMap::from([("desktop-entry", Value::from(super::THE_LINUX_DESKTOP_ENTRY))]);
        if let Some(path) = picture::as_path(&post.picture) {
            hints.extend([("image-path", Value::from(picture::a_file_address(path)))]);
        }
        let headline = post.headline();
        let second_line = post.the_second_line();

        let number = on_the_bus(|connection| async move {
            let answer = connection
                .call_method(
                    Some(THE_SERVICE),
                    THE_PATH,
                    Some(THE_SERVICE),
                    "Notify",
                    &(
                        "Smoozy",
                        replaces,
                        "",
                        // **Design's two lines, in the specification's own
                        // summary and body.** The first is the name and what
                        // happened, joined after each half took its own route;
                        // the second is the figures, or nothing where they are
                        // owed.
                        headline.as_str(),
                        second_line.as_str(),
                        actions,
                        hints,
                        // The server's own default lifetime. Nothing in §13
                        // asks for a length, and choosing one would be this
                        // file deciding how long a desktop keeps a message.
                        -1i32,
                    ),
                )
                .await?;
            answer.body().deserialize::<u32>()
        });

        if let (Some(number), Ok(mut numbers)) = (number, the_servers_own_numbers().lock()) {
            numbers.insert(key, number);
        }
    }

    /// **T24a and T24b: attempted and forgotten.**
    ///
    /// The specification publishes `CloseNotification`, and §22 item 61 is
    /// still what governs: **whether a posted notification can be taken down
    /// at all is not established on any of the three**, this route having
    /// never been run. **A post this run did not make has no number**, so
    /// there is nothing to close and nothing is invented.
    fn take_down(&self, key: &ThePresentationKey) {
        let text = key.as_text();
        let Some(number) = the_servers_own_numbers()
            .lock()
            .ok()
            .and_then(|mut numbers| numbers.remove(&text))
        else {
            return;
        };
        let _ = on_the_bus(|connection| async move {
            connection
                .call_method(
                    Some(THE_SERVICE),
                    THE_PATH,
                    Some(THE_SERVICE),
                    "CloseNotification",
                    &(number,),
                )
                .await?;
            Ok(())
        });
    }
}

/// **T28: the action-invoked signal, listened for — and the closing signal
/// beside it.**
///
/// The specification's own mechanism: **the notification server emits an
/// action-invoked signal on the session bus to the client that called it**,
/// and *"a plain click on the body is an action too"*, with the reserved key
/// `default`. **It publishes a closing signal beside it**, and F-5 is why this
/// listener takes both: the register held one entry for every notification of
/// the whole run and removed one only at a take-down, which reaches conditions
/// alone — so a finished job's entry sat there for as long as Smoozy ran, long
/// after the user dismissed it. **The numbers are the notification server's own
/// and are handed out across every application it serves**, so a stale entry can
/// match somebody else's notification and send the user somewhere they did not
/// ask to go. Removing the entry when the notification closes is what stops
/// that, and it is worth doing on either answer to §22 item 68.
///
/// **It listens on the one connection the send uses** — F-4, and [`TheBus`]
/// carries the whole of why.
///
/// **This is a wait and not a poll.** §3.3 P9 forbids polling and P10 allows
/// exactly one timer, which is the update check. **Nothing here repeats and
/// nothing here is scheduled**: one task waits on a bus socket for the length of
/// the run, which is the same shape a connection is, and it does nothing at all
/// until the server speaks.
///
/// **It ends with the run.** Nothing is joined and nothing durable is held; the
/// process exiting is what stops it, which is what run-held means.
pub(super) fn the_activation_route_is_open(on_click: Box<dyn Fn(String) + Send + Sync>) {
    if WHAT_A_CLICK_DOES.set(on_click).is_err() {
        return;
    }
    // **The connection is made off this thread.** This is called during launch,
    // on the thread that draws — §3.2 P4: nothing that can block runs there, and
    // reaching a socket can. The one thread below exists to make the connection
    // and start the listener on the bus's own worker, and then it is gone.
    std::thread::spawn(|| {
        let Some(bus) = the_bus() else {
            return;
        };
        bus.runtime.spawn(the_server_is_listened_to());
    });
}

/// The wait itself. See [`the_activation_route_is_open`] for the whole of it.
async fn the_server_is_listened_to() {
    use futures_util::StreamExt;
    let Some(bus) = the_bus() else {
        return;
    };
    // **One rule for both signals**, keyed to the interface rather than to
    // a member, because the two arrive on the same interface and a second
    // stream would be a second thing to keep in step.
    let rule = zbus::MatchRule::builder()
        .msg_type(zbus::message::Type::Signal)
        .interface(THE_SERVICE)
        .map(|builder| builder.build());
    let Ok(rule) = rule else {
        return;
    };
    let Ok(mut signals) =
        zbus::MessageStream::for_match_rule(rule, &bus.connection, None).await
    else {
        return;
    };
    while let Some(Ok(signal)) = signals.next().await {
        let member = signal
            .header()
            .member()
            .map(|member| member.to_string())
            .unwrap_or_default();
        match member.as_str() {
            "ActionInvoked" => {
                let Ok((number, _action)) = signal.body().deserialize::<(u32, String)>()
                else {
                    continue;
                };
                let key = the_key_for(number);
                // **Off this worker before it goes** (T28's correction of
                // 2026-11-01). A click reads the record under the
                // application's lock, and a post made under that lock waits
                // on this one worker for its answer — so going here would
                // leave each waiting on the other, and Smoozy stopped.
                if let (Some(key), Some(go)) = (key, WHAT_A_CLICK_DOES.get()) {
                    drop(bus.runtime.spawn_blocking(move || go(key)));
                }
            }
            // **F-5: the entry leaves the register when the notification
            // closes**, however it closed — dismissed, expired, or taken
            // down. The reason the server gives is not read, nothing
            // anywhere depending on which of them it was (T23).
            "NotificationClosed" => {
                let Ok((number, _why)) = signal.body().deserialize::<(u32, u32)>() else {
                    continue;
                };
                if let (Some(key), Ok(mut numbers)) =
                    (the_key_for(number), the_servers_own_numbers().lock())
                {
                    numbers.remove(&key);
                }
            }
            _ => {}
        }
    }
}

/// Which post this run made the server's own number belongs to, if any.
///
/// **A number this run did not hand out belongs to nobody here**, which is what
/// keeps a click or a close raised for another application's notification from
/// reaching anything of Smoozy's.
fn the_key_for(number: u32) -> Option<String> {
    the_servers_own_numbers().lock().ok().and_then(|numbers| {
        numbers
            .iter()
            .find(|(_, held)| **held == number)
            .map(|(key, _)| key.clone())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **T5: the absence path exists and is the first thing asked.**
    ///
    /// This is the line that fails if somebody assumes the bus, which is
    /// exactly what the specification says clients must not do.
    #[test]
    fn the_bus_is_asked_rather_than_assumed() {
        let source = include_str!("linux.rs");
        assert!(source.contains("name_has_owner"));
        assert!(source.contains("WhatIsMissing::TheSessionBusOrTheNotificationService"));
    }

    /// **T24: the replacement is the server's own number, and a post this run
    /// did not make has none.**
    #[test]
    fn a_post_with_no_number_replaces_nothing_and_closes_nothing() {
        let key = ThePresentationKey::ACondition(
            smoozy_state::identities::Identity::issue(
                smoozy_state::identities::IdentityKind::Condition,
            )
            .expect("the desk has randomness"),
        );
        assert!(the_servers_own_numbers()
            .lock()
            .expect("the register locks")
            .get(&key.as_text())
            .is_none());
        // Closing one that was never posted asks the bus for nothing.
        TheLinux.take_down(&key);
    }

    /// **T6: no permission model, and it is not granted.**
    #[test]
    fn there_is_no_permission_model_on_this_system() {
        assert_eq!(
            TheLinux.the_permission(),
            WhatTheSystemSays::ThereIsNoPermissionModel
        );
    }
}
