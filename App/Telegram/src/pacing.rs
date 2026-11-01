//! **`Telegram` C6, C6a and C6b — what a machine sends on its own goes out one
//! at a time, in a fixed order of classes, a refused send is honoured and sent
//! again, and a file upload goes in a lane of its own.**
//!
//! **Why this is piece 1's and not the clock's.** Pieces 2, 6 and 7 all hand
//! sends to one queue: the heartbeat, the status message, the pictures, the
//! endings and the answers. Building the queue once, before any of them, is
//! what lets them be built side by side without two queues.
//!
//! **The order, `Telegram` C6**: routed answers and instruction
//! acknowledgements; the holder's heartbeat; starts, endings, holds and drops; the
//! clock's edit; frame pictures and media. Within one class, first offered is
//! first sent. **So a picture backlog never delays an ending or an answer**,
//! and the heartbeat is never waiting behind a report.
//!
//! **At most one clock edit is outstanding, `Telegram` C6.** A tick offered
//! while one is waiting or in flight is dropped at once rather than queued: the
//! message shows the current state, so a superseded edit is worth nothing.
//!
//! **A refusal, `Telegram` C6a.** A refusal with `error_code` 429, or any that
//! names a wait, is waited on for `retry_after` seconds — one second where it
//! names none — and the same send is made again, until it is accepted. **The
//! first attempt is serial; a flood retry uses the upload lane after the
//! wait, with the heartbeat renewed first where needed (C6a, 2026-10-29).** A
//! refused clock edit is dropped instead, because a stale reading re-sent late
//! is worth nothing.
//!
//! **Three readings of C6a, decided here and written down in
//! `Audits/Build/Telegram/Decisions record.md`:**
//!
//! - **"Refused" means a flood refusal.** Any other refusal — a message that no
//!   longer exists, a text the service will not take — would be refused again
//!   for ever, so it is handed back to whoever offered the send, never counted
//!   as delivered and never silently dropped.
//! - **A send that did not reach the service at all is handed back too.** C6a
//!   is about refusals; what to do about a cut line is the offerer's, and the
//!   offerer is the one that knows whether the send still matters.
//! - **A flood retry must not hold the heartbeat.** Since 2026-10-29 it
//!   uses the upload lane with its whole timeout, after an early renewal
//!   through the listener where needed. A clock edit is still superseded.
//!
//! **A send in flight is not allowed to run past the moment the holder's
//! heartbeat falls due, `Telegram` C6** — added 2026-09-14. Piece 7 found that
//! a file upload held the one slot for up to 300 seconds, so a slow upload could
//! push the heartbeat past the 60-second renewal interval, which C6 forbids:
//! *"never deferred past the renewal interval by any report send"*. **C6 also
//! names one send in flight and the order of the classes, so both are kept**:
//! the heartbeat never goes beside another send and never jumps a queue it is
//! not already at the head of. Instead the listener tells the queue when the
//! next renewal falls due ([`TheSendQueue::the_heartbeat_falls_due`]), and every
//! other send is given a bound that ends there. **A send cut this way is handed
//! back as not reached**, the answer the queue already gives when a line runs
//! out of time, so its offerer decides as it does for any cut line — a
//! picture's journal event comes back (F3). **That cut now reaches only the
//! serial lane**: it cut every file taking longer than about fifty seconds on
//! the holder, and C6b below is what the plan ruled instead. The reasons are in
//! `Audits/Build/Telegram/Decisions record.md`.
//!
//! **`Telegram` C6b, added to the plan 2026-09-14 — a file upload runs in its
//! own lane beside the serial sends, and is never cut for the heartbeat.**
//! *"A send that uploads a file — a moving picture, a words file, an album —
//! goes in a lane of its own, one upload at a time, and runs to its end or to
//! its own failure."* So [`TheSendQueue::offer_an_upload`] and
//! [`TheSendQueue::send_the_next_upload`] are a second lane with its own one
//! slot: first offered is first sent, each upload keeps the client's own bound
//! for a file, and a flood refusal is waited on in that lane (C6a, the
//! refused send). **Every other send stays in the serial lane in C6's order**,
//! including frame pictures from the derivative set, which C6b keeps there
//! (*"small and stay in the serial lane"*). **C3 is not moved**: it counts
//! messages, an upload is one message however long it takes, and an upload
//! that is never cut is sent once rather than again after every cut.
//! **Whoever offers an upload must not wait for it on a thread the heartbeat
//! needs** — the listener's turn is one — or the lane buys nothing.

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use serde_json::Value;

use crate::listener::ReadsItsOwnClock;
use crate::transport::{ARefusal, TheClient, WhyTheCallFailed};

/// **`Telegram` C6's classes, highest first.** The order of the variants is the
/// order of sending.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AClassOfSend {
    RoutedAnswersAndAcknowledgements,
    TheHoldersHeartbeat,
    EndingsHoldsAndDrops,
    TheClocksEdit,
    FramePicturesAndMedia,
}

/// **One send.** It is called again, unchanged, after a flood refusal, so it
/// must make the same call each time it is called.
pub type ASend = Box<dyn FnMut(&TheClient) -> Result<Value, WhyTheCallFailed> + Send>;

/// **How a send ended**, told to whoever offered it.
#[derive(Debug, Clone, PartialEq)]
pub enum HowASendEnded {
    /// The service accepted it, and this is its `result`.
    Accepted(Value),
    /// A clock edit that was superseded — offered while another was
    /// outstanding, or refused for flooding — and not sent (`Telegram` C6, C6a).
    DroppedAsSuperseded,
    /// Not delivered, and not to be retried by this queue: a refusal that is
    /// not a flood refusal, or no answer at all.
    HandedBack(WhyTheCallFailed),
}

/// **`Telegram` C6a — the wait when a flood refusal names none.**
pub const THE_WAIT_WHEN_A_REFUSAL_NAMES_NONE: Duration = Duration::from_secs(1);

/// **`Telegram` C6a — how long a refusal asks this queue to wait, or nothing
/// where it is not a flood refusal.**
pub fn the_wait_a_refusal_asks_for(refusal: &ARefusal) -> Option<Duration> {
    if refusal.error_code == Some(429) || refusal.retry_after.is_some() {
        Some(
            refusal
                .retry_after
                .map(Duration::from_secs)
                .unwrap_or(THE_WAIT_WHEN_A_REFUSAL_NAMES_NONE),
        )
    } else {
        None
    }
}

/// **How the queue waits.** The real one sleeps; a test's records.
pub trait Waits: Send + Sync {
    fn wait(&self, how_long: Duration);
}

/// Waiting on this computer's own clock.
pub struct OnThisComputersClock;

impl Waits for OnThisComputersClock {
    fn wait(&self, how_long: Duration) {
        std::thread::sleep(how_long);
    }
}

struct Waiting {
    by_class: BTreeMap<AClassOfSend, VecDeque<(ASend, Sender<HowASendEnded>)>>,
    a_clock_edit_is_outstanding: bool,
}

/// **When the holder's next heartbeat falls due**, on the holder's own clock.
struct TheHeartbeatsDue {
    at: Duration,
    clock: Arc<dyn ReadsItsOwnClock>,
}

/// **The shortest time a send is given while a heartbeat is due or overdue.**
/// Longer than an ordinary text's round trip on this project's measured lines
/// (about half a second, `Telegram` §21 item 4), and short beside the ten
/// seconds the holder renews inside the stated interval, so the heartbeat
/// still lands inside it.
pub const THE_LEAST_A_SEND_IS_GIVEN: Duration = Duration::from_secs(2);

/// **The one send queue.** Share it with an `Arc`; any thread may offer, and
/// any thread may send, but only one send is ever in flight.
pub struct TheSendQueue {
    client: TheClient,
    waits: Arc<dyn Waits>,
    waiting: Mutex<Waiting>,
    the_one_in_flight: Mutex<()>,
    the_heartbeats_due: Mutex<Option<TheHeartbeatsDue>>,
    /// **`Telegram` C6b's upload lane**: its waiting uploads, first offered
    /// first, and its own one slot.
    uploads_waiting: Mutex<VecDeque<(ASend, Sender<HowASendEnded>)>>,
    the_one_upload_in_flight: Mutex<()>,
    before_flood_retry: Mutex<Option<Arc<dyn Fn() -> bool + Send + Sync>>>,
    failure_notes: Mutex<Option<Arc<dyn Fn(&str) + Send + Sync>>>,
    failure_context: Mutex<HashMap<std::thread::ThreadId, String>>,
}

impl TheSendQueue {
    pub fn new(client: TheClient, waits: Arc<dyn Waits>) -> TheSendQueue {
        TheSendQueue {
            client,
            waits,
            waiting: Mutex::new(Waiting {
                by_class: BTreeMap::new(),
                a_clock_edit_is_outstanding: false,
            }),
            the_one_in_flight: Mutex::new(()),
            the_heartbeats_due: Mutex::new(None),
            uploads_waiting: Mutex::new(VecDeque::new()),
            the_one_upload_in_flight: Mutex::new(()),
            before_flood_retry: Mutex::new(None),
            failure_notes: Mutex::new(None),
            failure_context: Mutex::new(HashMap::new()),
        }
    }

    /// `Telegram` F3a: only calls made through this worker's client can count a try.
    pub fn how_many_calls_reached_telegram(&self) -> u64 {
        self.client.how_many_calls_reached_telegram()
    }

    pub fn note_failed_sends_with(&self, notes: Arc<dyn Fn(&str) + Send + Sync>) {
        *lock(&self.failure_notes) = Some(notes);
    }

    pub fn the_event_being_sent(&self, context: String) {
        let mut contexts = lock(&self.failure_context);
        if context == "outside a journal event" {
            contexts.remove(&std::thread::current().id());
        } else {
            contexts.insert(std::thread::current().id(), context);
        }
    }

    fn noting_failures(&self, mut send: ASend) -> ASend {
        let notes = lock(&self.failure_notes).clone();
        let context = lock(&self.failure_context).get(&std::thread::current().id()).cloned()
            .unwrap_or_else(|| "outside a journal event".into());
        Box::new(move |client| {
            let result = send(client);
            if let Some(notes) = &notes {
                match &result {
                    Err(WhyTheCallFailed::NotReached(_)) => notes(&format!("a send was not reached: {context}")),
                    Err(WhyTheCallFailed::Refused(refusal)) => {
                        if let Some(wait) = the_wait_a_refusal_asks_for(refusal) {
                            notes(&format!("a send was refused for flooding; requested wait {} seconds: {context}", wait.as_secs()));
                        }
                    }
                    _ => {}
                }
            }
            result
        })
    }

    /// C6a: a renewal uses the listener's own board and exact read-back.
    pub fn before_a_flood_retry(&self, renew: Arc<dyn Fn() -> bool + Send + Sync>) {
        *lock(&self.before_flood_retry) = Some(renew);
    }

    fn a_full_retry_meets_the_heartbeat(&self) -> bool {
        lock(&self.the_heartbeats_due).as_ref().is_some_and(|due| {
            due.at.saturating_sub(due.clock.now()) <= crate::transport::THE_BOUND_ON_SENDING_A_FILE
        })
    }

    /// **`Telegram` C6b — offer a send that uploads a file**: a moving picture,
    /// a words file, an album. It waits behind any upload offered before it and
    /// beside every serial send, never behind one. The answer arrives on the
    /// returned receiver when it ends.
    pub fn offer_an_upload(&self, send: ASend) -> Receiver<HowASendEnded> {
        let (tell, told) = channel();
        lock(&self.uploads_waiting).push_back((self.noting_failures(send), tell));
        told
    }

    /// **`Telegram` C6b — send the upload that has waited longest, and see it
    /// to its end or its own failure.** It keeps the client's own bound for a
    /// file whatever the heartbeat is doing. Gives back whether one was
    /// waiting. A second caller waits for the first to finish, so two uploads
    /// are never beside each other.
    pub fn send_the_next_upload(&self) -> bool {
        let _the_slot = lock(&self.the_one_upload_in_flight);
        let Some((mut send, tell)) = lock(&self.uploads_waiting).pop_front() else {
            return false;
        };
        let ended = self.see_it_to_its_end(&mut send, false, || self.client.clone());
        let _ = tell.send(ended);
        true
    }

    /// How many uploads are waiting, not counting one in flight.
    pub fn how_many_uploads_are_waiting(&self) -> usize {
        lock(&self.uploads_waiting).len()
    }

    /// **`Telegram` C6 — tell the queue when the holder's next heartbeat falls
    /// due**, on the holder's own clock, or that none is owed because this
    /// machine is not holding. The listener says so when it takes the slot,
    /// after every renewal, and when it stands down.
    pub fn the_heartbeat_falls_due(&self, at: Option<(Duration, Arc<dyn ReadsItsOwnClock>)>) {
        *lock(&self.the_heartbeats_due) = at.map(|(at, clock)| TheHeartbeatsDue { at, clock });
    }

    /// **Offer a send.** The answer arrives on the returned receiver when it
    /// ends; an offerer that does not care may drop it.
    pub fn offer(&self, class: AClassOfSend, send: ASend) -> Receiver<HowASendEnded> {
        let (tell, told) = channel();
        let send = self.noting_failures(send);
        let mut waiting = lock(&self.waiting);
        if class == AClassOfSend::TheClocksEdit {
            if waiting.a_clock_edit_is_outstanding {
                let _ = tell.send(HowASendEnded::DroppedAsSuperseded);
                return told;
            }
            waiting.a_clock_edit_is_outstanding = true;
        }
        waiting
            .by_class
            .entry(class)
            .or_default()
            .push_back((send, tell));
        told
    }

    /// **Send the highest waiting send, and see it to its end.** Gives back its
    /// class, or nothing where nothing was waiting. A second caller waits for
    /// the first to finish.
    pub fn send_the_next(&self) -> Option<AClassOfSend> {
        let mut the_slot = lock(&self.the_one_in_flight);
        let (class, mut send, tell) = {
            let mut waiting = lock(&self.waiting);
            let class = *waiting
                .by_class
                .iter()
                .find(|(_, sends)| !sends.is_empty())?
                .0;
            let (send, tell) = waiting
                .by_class
                .get_mut(&class)
                .and_then(VecDeque::pop_front)
                .expect("the class was found holding a send");
            (class, send, tell)
        };

        let ended = match send(&self.the_client_for(class)) {
            Ok(result) => HowASendEnded::Accepted(result),
            Err(WhyTheCallFailed::Refused(refusal)) => match the_wait_a_refusal_asks_for(&refusal) {
                Some(_) if class == AClassOfSend::TheClocksEdit => HowASendEnded::DroppedAsSuperseded,
                Some(how_long) if class == AClassOfSend::TheHoldersHeartbeat => {
                    self.waits.wait(how_long);
                    self.see_it_to_its_end(&mut send, false, || self.client.clone())
                }
                Some(how_long) => {
                    // C6a: a whole retry uses C6b's upload lane, so the
                    // following heartbeat is not held by its file timeout.
                    drop(the_slot);
                    self.waits.wait(how_long);
                    let ready = if class != AClassOfSend::TheHoldersHeartbeat && self.a_full_retry_meets_the_heartbeat() {
                        let renew = lock(&self.before_flood_retry).clone();
                        renew.is_some_and(|renew| renew())
                    } else { true };
                    let ended = if ready {
                        let _upload_slot = lock(&self.the_one_upload_in_flight);
                        self.see_it_to_its_end(&mut send, false, || self.client.clone())
                    } else {
                        if let Some(notes) = lock(&self.failure_notes).as_ref() {
                            notes("a flood retry waits for an unconfirmed heartbeat; it was not sent");
                        }
                        HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(
                            "the heartbeat could not be confirmed before the flood retry".into(),
                        ))
                    };
                    the_slot = lock(&self.the_one_in_flight);
                    ended
                }
                None => HowASendEnded::HandedBack(WhyTheCallFailed::Refused(refusal)),
            },
            Err(other) => HowASendEnded::HandedBack(other),
        };

        if class == AClassOfSend::TheClocksEdit {
            lock(&self.waiting).a_clock_edit_is_outstanding = false;
        }
        let _ = tell.send(ended);
        drop(the_slot);
        Some(class)
    }

    /// **One send, made again after each flood wait until it ends** (C6a), in
    /// whichever lane holds its slot. A superseded send — a clock edit — is
    /// dropped on a flood refusal instead of waited on.
    fn see_it_to_its_end(
        &self,
        send: &mut ASend,
        dropped_on_a_flood: bool,
        the_client: impl Fn() -> TheClient,
    ) -> HowASendEnded {
        loop {
            match send(&the_client()) {
                Ok(result) => return HowASendEnded::Accepted(result),
                Err(WhyTheCallFailed::Refused(refusal)) => {
                    match the_wait_a_refusal_asks_for(&refusal) {
                        Some(_) if dropped_on_a_flood => return HowASendEnded::DroppedAsSuperseded,
                        Some(how_long) => self.waits.wait(how_long),
                        None => {
                            return HowASendEnded::HandedBack(WhyTheCallFailed::Refused(refusal))
                        }
                    }
                }
                Err(other) => return HowASendEnded::HandedBack(other),
            }
        }
    }

    /// **`Telegram` C6 — the client one attempt at a serial send is made
    /// through.** An upload never comes here (C6b). The heartbeat keeps every
    /// bound its own. Any other send, while a
    /// heartbeat is owed, may run only until it falls due, and never less than
    /// [`THE_LEAST_A_SEND_IS_GIVEN`]. A flood retry uses the full client in
    /// the upload lane instead (C6a).
    fn the_client_for(&self, class: AClassOfSend) -> TheClient {
        if class == AClassOfSend::TheHoldersHeartbeat {
            return self.client.clone();
        }
        match &*lock(&self.the_heartbeats_due) {
            None => self.client.clone(),
            Some(due) => {
                let left = due.at.saturating_sub(due.clock.now());
                self.client.no_call_longer_than(left.max(THE_LEAST_A_SEND_IS_GIVEN))
            }
        }
    }

    /// How many sends are waiting, not counting one in flight.
    pub fn how_many_are_waiting(&self) -> usize {
        lock(&self.waiting)
            .by_class
            .values()
            .map(VecDeque::len)
            .sum()
    }
}

fn lock<T>(held: &Mutex<T>) -> MutexGuard<'_, T> {
    held.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::a_stand_in::AStandInService;
    use crate::transport::{ReachesTelegram, WhatACallCarries};
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Default)]
    struct RecordsWaits(Mutex<Vec<Duration>>);
    impl Waits for RecordsWaits {
        fn wait(&self, how_long: Duration) {
            lock(&self.0).push(how_long);
        }
    }

    fn a_queue() -> (Arc<AStandInService>, Arc<RecordsWaits>, TheSendQueue) {
        let service = Arc::new(AStandInService::new());
        let waits = Arc::new(RecordsWaits::default());
        let queue = TheSendQueue::new(TheClient::over(service.clone()), waits.clone());
        (service, waits, queue)
    }

    fn a_message(text: &'static str) -> ASend {
        Box::new(move |client: &TheClient| client.send_message(1, text, None))
    }

    fn the_texts_sent(service: &AStandInService) -> Vec<String> {
        service
            .heard()
            .iter()
            .map(|call| match &call.carries {
                WhatACallCarries::Fields(fields) => fields["text"].as_str().unwrap().to_string(),
                _ => panic!("a text message carries fields"),
            })
            .collect()
    }

    #[test]
    fn twenty_first_a_flood_retry_renews_first_and_gets_its_whole_timeout() {
        struct FloodThenSlow { calls: Mutex<Vec<(String, Duration)>> }
        impl ReachesTelegram for FloodThenSlow {
            fn call(&self, method: &str, _: &WhatACallCarries, bound: Duration) -> Result<String, String> {
                let mut calls = lock(&self.calls);
                calls.push((method.into(), bound));
                if calls.len() == 1 {
                    return Ok(r#"{"ok":false,"error_code":429,"parameters":{"retry_after":3}}"#.into());
                }
                if method == "sendMessage" && bound < Duration::from_secs(5) { return Err("the two-second cut".into()); }
                Ok(r#"{"ok":true,"result":{"message_id":7}}"#.into())
            }
        }
        let wire = Arc::new(FloodThenSlow { calls: Mutex::new(Vec::new()) });
        let waits = Arc::new(RecordsWaits::default());
        let queue = Arc::new(TheSendQueue::new(TheClient::over(wire.clone()), waits.clone()));
        let notes = Arc::new(Mutex::new(Vec::<String>::new()));
        let copy = notes.clone();
        queue.note_failed_sends_with(Arc::new(move |line| lock(&copy).push(line.into())));
        queue.the_event_being_sent("event 12 job video kind ending".into());
        let clock = Arc::new(AClockMovedByHand(Mutex::new(Duration::from_secs(60))));
        queue.the_heartbeat_falls_due(Some((Duration::from_secs(50), clock)));
        let weak = Arc::downgrade(&queue);
        queue.before_a_flood_retry(Arc::new(move || {
            let queue = weak.upgrade().unwrap();
            let heartbeat = queue.offer(AClassOfSend::TheHoldersHeartbeat, a_heartbeat());
            queue.send_the_next();
            matches!(heartbeat.recv().unwrap(), HowASendEnded::Accepted(_))
        }));
        let receipt = queue.offer(AClassOfSend::EndingsHoldsAndDrops, a_message("video finished"));
        queue.send_the_next();
        assert!(matches!(receipt.recv().unwrap(), HowASendEnded::Accepted(_)));
        assert_eq!(*lock(&waits.0), [Duration::from_secs(3)]);
        assert!(lock(&notes).iter().any(|line| line.contains("flooding; requested wait 3 seconds: event 12 job video kind ending")));
        assert_eq!(*lock(&wire.calls), [
            ("sendMessage".into(), THE_LEAST_A_SEND_IS_GIVEN),
            ("editMessageText".into(), crate::transport::THE_BOUND_ON_AN_ORDINARY_CALL),
            ("sendMessage".into(), crate::transport::THE_BOUND_ON_AN_ORDINARY_CALL),
        ]);
    }

    #[test]
    fn twenty_first_a_long_flood_retry_does_not_hold_the_following_heartbeat() {
        let (service, waits, queue) = a_queue();
        let queue = Arc::new(queue);
        let (began, started) = channel();
        let (finish, finished) = channel();
        let mut first = true;
        let receipt = queue.offer(AClassOfSend::FramePicturesAndMedia, Box::new(move |_| {
            if std::mem::take(&mut first) {
                return Err(WhyTheCallFailed::Refused(ARefusal { error_code: Some(429), description: Some("flood".into()), retry_after: Some(3) }));
            }
            began.send(()).unwrap();
            finished.recv_timeout(Duration::from_secs(2)).expect("heartbeat can finish before the retry");
            Ok(Value::Bool(true))
        }));
        let copy = queue.clone();
        let sender = std::thread::spawn(move || copy.send_the_next());
        started.recv_timeout(Duration::from_secs(2)).unwrap();
        let heartbeat = queue.offer(AClassOfSend::TheHoldersHeartbeat, a_heartbeat());
        queue.send_the_next();
        assert!(matches!(heartbeat.recv().unwrap(), HowASendEnded::Accepted(_)));
        finish.send(()).unwrap();
        sender.join().unwrap();
        assert!(matches!(receipt.recv().unwrap(), HowASendEnded::Accepted(_)));
        assert_eq!(*lock(&waits.0), [Duration::from_secs(3)]);
        assert_eq!(service.heard().len(), 1, "the heartbeat reached during the retry");
    }

    /// **`Telegram` C6's order**, whatever order the sends were offered in, and
    /// first offered first sent inside a class.
    #[test]
    fn sends_go_out_in_the_order_of_their_classes() {
        let (service, _, queue) = a_queue();
        queue.offer(AClassOfSend::FramePicturesAndMedia, a_message("picture"));
        queue.offer(AClassOfSend::TheClocksEdit, a_message("clock"));
        queue.offer(AClassOfSend::EndingsHoldsAndDrops, a_message("ending one"));
        queue.offer(AClassOfSend::EndingsHoldsAndDrops, a_message("ending two"));
        queue.offer(AClassOfSend::TheHoldersHeartbeat, a_message("heartbeat"));
        queue.offer(
            AClassOfSend::RoutedAnswersAndAcknowledgements,
            a_message("answer"),
        );

        while queue.send_the_next().is_some() {}

        assert_eq!(
            the_texts_sent(&service),
            [
                "answer",
                "heartbeat",
                "ending one",
                "ending two",
                "clock",
                "picture"
            ]
        );
        assert_eq!(queue.how_many_are_waiting(), 0);
    }

    /// **`Telegram` C6a: the wait the refusal names, then the same send again.**
    #[test]
    fn a_flood_refusal_is_waited_on_and_the_same_send_is_made_again() {
        let (service, waits, queue) = a_queue();
        service.then_answers(
            r#"{"ok":false,"error_code":429,"description":"Too Many Requests: retry after 3","parameters":{"retry_after":3}}"#,
        );
        let told = queue.offer(AClassOfSend::EndingsHoldsAndDrops, a_message("ending"));

        queue.send_the_next();

        assert_eq!(*lock(&waits.0), [Duration::from_secs(3)]);
        assert_eq!(the_texts_sent(&service), ["ending", "ending"]);
        assert_eq!(
            told.recv().unwrap(),
            HowASendEnded::Accepted(Value::Bool(true))
        );
    }

    /// **`Telegram` C6a: one second where the refusal names no wait.**
    #[test]
    fn a_flood_refusal_naming_no_wait_is_waited_on_for_one_second() {
        let (service, waits, queue) = a_queue();
        service.then_answers(r#"{"ok":false,"error_code":429,"description":"Too Many Requests"}"#);
        queue.offer(AClassOfSend::FramePicturesAndMedia, a_message("picture"));
        queue.send_the_next();
        assert_eq!(*lock(&waits.0), [Duration::from_secs(1)]);
        assert_eq!(service.heard().len(), 2);
    }

    /// **`Telegram` C6a: a refused clock edit is dropped, not waited on.**
    #[test]
    fn a_refused_clock_edit_is_dropped_rather_than_sent_again() {
        let (service, waits, queue) = a_queue();
        service.then_answers(
            r#"{"ok":false,"error_code":429,"description":"Too Many Requests: retry after 5","parameters":{"retry_after":5}}"#,
        );
        let told = queue.offer(AClassOfSend::TheClocksEdit, a_message("clock"));
        queue.send_the_next();
        assert!(lock(&waits.0).is_empty());
        assert_eq!(service.heard().len(), 1);
        assert_eq!(told.recv().unwrap(), HowASendEnded::DroppedAsSuperseded);
    }

    /// **`Telegram` C6: at most one clock edit outstanding.**
    #[test]
    fn a_tick_offered_while_one_is_outstanding_is_dropped() {
        let (service, _, queue) = a_queue();
        let first = queue.offer(AClassOfSend::TheClocksEdit, a_message("first tick"));
        let second = queue.offer(AClassOfSend::TheClocksEdit, a_message("second tick"));
        assert_eq!(second.recv().unwrap(), HowASendEnded::DroppedAsSuperseded);
        assert_eq!(queue.how_many_are_waiting(), 1);

        queue.send_the_next();
        assert!(matches!(first.recv().unwrap(), HowASendEnded::Accepted(_)));

        let third = queue.offer(AClassOfSend::TheClocksEdit, a_message("third tick"));
        queue.send_the_next();
        assert!(matches!(third.recv().unwrap(), HowASendEnded::Accepted(_)));
        assert_eq!(the_texts_sent(&service), ["first tick", "third tick"]);
    }

    /// **A refusal that is not a flood is handed back**, once, never delivered.
    #[test]
    fn any_other_refusal_is_handed_back_and_not_sent_again() {
        let (service, waits, queue) = a_queue();
        service.then_answers(r#"{"ok":false,"error_code":400,"description":"Bad Request: message to edit not found"}"#);
        let told = queue.offer(AClassOfSend::EndingsHoldsAndDrops, a_message("ending"));
        queue.send_the_next();
        assert_eq!(service.heard().len(), 1);
        assert!(lock(&waits.0).is_empty());
        assert!(matches!(
            told.recv().unwrap(),
            HowASendEnded::HandedBack(WhyTheCallFailed::Refused(ARefusal {
                error_code: Some(400),
                ..
            }))
        ));
    }

    #[test]
    fn a_send_that_reached_nothing_is_handed_back() {
        let (service, _, queue) = a_queue();
        service.then_is_not_reached("connection failed");
        let told = queue.offer(AClassOfSend::TheHoldersHeartbeat, a_message("heartbeat"));
        queue.send_the_next();
        assert_eq!(service.heard().len(), 1);
        assert_eq!(
            told.recv().unwrap(),
            HowASendEnded::HandedBack(WhyTheCallFailed::NotReached("connection failed".into()))
        );
    }

    /// **A clock a test moves by hand**: every wait, and every call on
    /// [`SlowUploads`], moves it on.
    #[derive(Default)]
    struct AClockMovedByHand(Mutex<Duration>);
    impl ReadsItsOwnClock for AClockMovedByHand {
        fn now(&self) -> Duration {
            *lock(&self.0)
        }
        fn the_moment(&self) -> String {
            format!("t+{}", lock(&self.0).as_secs())
        }
    }

    /// **A line on which an upload takes `an_upload_takes`** and anything else
    /// takes no time. Like the real client, a call whose bound runs out first
    /// ends at its bound, not reached. It writes down what each call was, and
    /// when it ended.
    struct SlowUploads {
        clock: Arc<AClockMovedByHand>,
        an_upload_takes: Duration,
        ended: Mutex<Vec<(String, Duration, Duration)>>,
    }
    impl ReachesTelegram for SlowUploads {
        fn call(&self, method: &str, _: &WhatACallCarries, bound: Duration) -> Result<String, String> {
            let takes = if method == "sendDocument" { self.an_upload_takes } else { Duration::ZERO };
            let reached = takes <= bound;
            let now = {
                let mut now = lock(&self.clock.0);
                *now += takes.min(bound);
                *now
            };
            lock(&self.ended).push((method.to_string(), bound, now));
            if reached {
                Ok(r#"{"ok":true,"result":{"message_id":7}}"#.into())
            } else {
                Err("the call's bound passed".into())
            }
        }
    }

    fn an_upload() -> ASend {
        Box::new(|c: &TheClient| c.send_document(1, "what the render printed.txt", vec![0; 64], None, None))
    }

    fn a_heartbeat() -> ASend {
        Box::new(|c: &TheClient| c.edit_a_machinery_message(1, 5, "Smoozy — server coordination"))
    }

    /// **`Telegram` C6: *"The heartbeat is a board edit and therefore a send,
    /// and it is never deferred past the renewal interval by any report
    /// send."*** Piece 7 found that an upload held the queue's one slot for up
    /// to 300 seconds. Here the holder renewed at 0, the next renewal falls due
    /// at 50 on the holder's 60-second interval, and an upload that takes 120
    /// seconds is already in flight when it does. **Before the fix the
    /// heartbeat went at 120 seconds.** The upload is now cut when the
    /// heartbeat falls due, handed back as not reached — so its offerer's
    /// event comes back (F3) and nothing is lost — and the heartbeat goes
    /// inside the interval. **Since C6b this cut reaches only a send in the
    /// serial lane**; a real upload goes in the upload lane and is never cut
    /// (the C6b tests below). The send here stands for any slow serial send.
    #[test]
    fn a_slow_upload_does_not_hold_the_heartbeat_past_the_renewal_interval() {
        let clock = Arc::new(AClockMovedByHand::default());
        let line = Arc::new(SlowUploads {
            clock: clock.clone(),
            an_upload_takes: Duration::from_secs(120),
            ended: Mutex::default(),
        });
        let queue = TheSendQueue::new(TheClient::over(line.clone()), Arc::new(RecordsWaits::default()));
        let the_renewal_interval = Duration::from_secs(60);
        queue.the_heartbeat_falls_due(Some((Duration::from_secs(50), clock.clone())));

        let upload = queue.offer(AClassOfSend::FramePicturesAndMedia, an_upload());
        queue.send_the_next();
        let heartbeat = queue.offer(AClassOfSend::TheHoldersHeartbeat, a_heartbeat());
        queue.send_the_next();

        let ended = lock(&line.ended).clone();
        let (_, _, the_heartbeat_went_at) = ended.iter().find(|(m, _, _)| m == "editMessageText").cloned().unwrap();
        assert!(
            the_heartbeat_went_at < the_renewal_interval,
            "C6: the heartbeat went at {the_heartbeat_went_at:?}, past the {the_renewal_interval:?} interval"
        );
        assert!(matches!(heartbeat.recv().unwrap(), HowASendEnded::Accepted(_)));
        assert!(matches!(
            upload.recv().unwrap(),
            HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(_))
        ));
    }

    /// **The heartbeat itself is never shortened, and an upload that fits
    /// before the heartbeat falls due goes whole.** A machine that is not
    /// holding owes no heartbeat, so its uploads keep their own bound.
    #[test]
    fn only_what_would_cross_the_heartbeat_is_shortened() {
        let clock = Arc::new(AClockMovedByHand::default());
        let line = Arc::new(SlowUploads {
            clock: clock.clone(),
            an_upload_takes: Duration::from_secs(20),
            ended: Mutex::default(),
        });
        let queue = TheSendQueue::new(TheClient::over(line.clone()), Arc::new(RecordsWaits::default()));

        // Not holding: the upload's own bound.
        queue.offer(AClassOfSend::FramePicturesAndMedia, an_upload());
        queue.send_the_next();
        // Holding, the heartbeat due at 70: 20 seconds fit, so it goes whole.
        queue.the_heartbeat_falls_due(Some((Duration::from_secs(70), clock.clone())));
        let fits = queue.offer(AClassOfSend::FramePicturesAndMedia, an_upload());
        queue.send_the_next();
        // At 40, the heartbeat itself: its own bound, whatever is due.
        queue.offer(AClassOfSend::TheHoldersHeartbeat, a_heartbeat());
        queue.send_the_next();
        // Due and overdue: anything else is given the least.
        queue.the_heartbeat_falls_due(Some((Duration::from_secs(30), clock.clone())));
        queue.offer(AClassOfSend::RoutedAnswersAndAcknowledgements, Box::new(|c: &TheClient| c.send_message(1, "answer", None)));
        queue.send_the_next();

        let bounds: Vec<(String, Duration)> = lock(&line.ended).iter().map(|(m, b, _)| (m.clone(), *b)).collect();
        assert_eq!(
            bounds,
            [
                ("sendDocument".to_string(), crate::transport::THE_BOUND_ON_SENDING_A_FILE),
                ("sendDocument".to_string(), Duration::from_secs(50)),
                ("editMessageText".to_string(), crate::transport::THE_BOUND_ON_AN_ORDINARY_CALL),
                ("sendMessage".to_string(), THE_LEAST_A_SEND_IS_GIVEN),
            ]
        );
        assert!(matches!(fits.recv().unwrap(), HowASendEnded::Accepted(_)));
    }

    // ------------------------------------------------ C6b, the upload lane

    /// **A line on which an upload takes `an_upload_takes` on a clock the test
    /// moves by hand, and waits for it to move**, so an upload can be in flight
    /// on its own thread while the test sends in the serial lane. Anything else
    /// takes no time. A call whose bound runs out first ends at its bound, not
    /// reached, like the real client. It writes down every call and counts how
    /// many were in flight at once.
    struct UploadsInFlight {
        clock: Arc<AClockMovedByHand>,
        an_upload_takes: Duration,
        calls_inside: AtomicUsize,
        uploads_inside: AtomicUsize,
        most_calls_at_once: AtomicUsize,
        most_uploads_at_once: AtomicUsize,
        heard: Mutex<Vec<ACallMade>>,
    }

    #[derive(Debug, Clone)]
    struct ACallMade {
        method: String,
        /// The file's name and how many bytes it carried, for an upload.
        file: Option<(String, usize)>,
        bound: Duration,
        began: Duration,
        ended: Duration,
        reached: bool,
    }

    impl UploadsInFlight {
        fn new(clock: &Arc<AClockMovedByHand>, an_upload_takes: Duration) -> Arc<UploadsInFlight> {
            Arc::new(UploadsInFlight {
                clock: clock.clone(),
                an_upload_takes,
                calls_inside: AtomicUsize::new(0),
                uploads_inside: AtomicUsize::new(0),
                most_calls_at_once: AtomicUsize::new(0),
                most_uploads_at_once: AtomicUsize::new(0),
                heard: Mutex::default(),
            })
        }
        fn heard(&self) -> Vec<ACallMade> {
            lock(&self.heard).clone()
        }
        fn uploads_inside(&self) -> usize {
            self.uploads_inside.load(Ordering::SeqCst)
        }
    }

    impl ReachesTelegram for UploadsInFlight {
        fn call(&self, method: &str, carries: &WhatACallCarries, bound: Duration) -> Result<String, String> {
            let upload = matches!(method, "sendDocument" | "sendVideo" | "sendMediaGroup");
            let inside = self.calls_inside.fetch_add(1, Ordering::SeqCst) + 1;
            self.most_calls_at_once.fetch_max(inside, Ordering::SeqCst);
            if upload {
                let inside = self.uploads_inside.fetch_add(1, Ordering::SeqCst) + 1;
                self.most_uploads_at_once.fetch_max(inside, Ordering::SeqCst);
            }
            let began = self.clock.now();
            let takes = if upload { self.an_upload_takes } else { Duration::ZERO };
            let ends = began + takes.min(bound);
            let given = std::time::Instant::now();
            while self.clock.now() < ends {
                assert!(given.elapsed() < Duration::from_secs(30), "the test never moved the clock to the upload's end");
                std::thread::sleep(Duration::from_millis(1));
            }
            let file = match carries {
                WhatACallCarries::FieldsAndFiles { files, .. } => files.first().map(|f| (f.file_name.clone(), f.bytes.len())),
                _ => None,
            };
            let reached = takes <= bound;
            lock(&self.heard).push(ACallMade { method: method.into(), file, bound, began, ended: self.clock.now(), reached });
            if upload {
                self.uploads_inside.fetch_sub(1, Ordering::SeqCst);
            }
            self.calls_inside.fetch_sub(1, Ordering::SeqCst);
            if reached {
                Ok(r#"{"ok":true,"result":{"message_id":7}}"#.into())
            } else {
                Err("the call's bound passed".into())
            }
        }
    }

    fn an_upload_named(name: &'static str, bytes: usize) -> ASend {
        Box::new(move |c: &TheClient| c.send_document(1, name, vec![7; bytes], None, None))
    }

    fn until(what: &str, done: impl Fn() -> bool) {
        let given = std::time::Instant::now();
        while !done() {
            assert!(given.elapsed() < Duration::from_secs(10), "waited too long for {what}");
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    fn set(clock: &AClockMovedByHand, seconds: u64) {
        *lock(&clock.0) = Duration::from_secs(seconds);
    }

    /// **The holder's renewals, in the serial lane, while an upload is in
    /// flight**: at each moment the heartbeat falls due, the clock is there,
    /// the heartbeat is offered and sent, and the next renewal is told to the
    /// queue as the listener tells it. Gives back when each heartbeat went.
    fn renew_at(queue: &TheSendQueue, line: &UploadsInFlight, clock: &Arc<AClockMovedByHand>, at: &[u64], every: u64) -> Vec<Duration> {
        at.iter()
            .map(|at| {
                set(clock, *at);
                let heartbeat = queue.offer(AClassOfSend::TheHoldersHeartbeat, a_heartbeat());
                queue.send_the_next();
                assert!(matches!(heartbeat.recv().unwrap(), HowASendEnded::Accepted(_)));
                queue.the_heartbeat_falls_due(Some((Duration::from_secs(at + every), clock.clone())));
                line.heard().iter().rev().find(|c| c.method == "editMessageText").unwrap().began
            })
            .collect()
    }

    /// **`Telegram` C6b: *"a send that uploads a file … goes in a lane of its
    /// own, one upload at a time, and runs to its end or to its own
    /// failure"*, and C6: the heartbeat is never deferred.** The holder renews
    /// at 0 on its 50-second renewal time; an upload that takes 120 seconds is
    /// in flight across the renewals at 50 and 100. Both heartbeats go at the
    /// moment they fall due, and the upload arrives whole, once, at 120, with
    /// the client's own bound for a file. **Offered to the serial lane
    /// instead, the same upload is cut at 50** (the test above).
    #[test]
    fn a_120_second_upload_across_two_renewals_arrives_whole_and_the_heartbeat_goes_on_time_both_times() {
        let clock = Arc::new(AClockMovedByHand::default());
        let line = UploadsInFlight::new(&clock, Duration::from_secs(120));
        let queue = Arc::new(TheSendQueue::new(TheClient::over(line.clone()), Arc::new(RecordsWaits::default())));
        queue.the_heartbeat_falls_due(Some((Duration::from_secs(50), clock.clone())));

        let upload = queue.offer_an_upload(an_upload_named("Kitchen - final - Output.txt", 51_289));
        let uploading = { let queue = queue.clone(); std::thread::spawn(move || queue.send_the_next_upload()) };
        until("the upload to be in flight", || line.uploads_inside() == 1);

        let went = renew_at(&queue, &line, &clock, &[50, 100], 50);
        assert_eq!(went, [Duration::from_secs(50), Duration::from_secs(100)], "C6: a heartbeat was deferred");
        assert!(upload.try_recv().is_err(), "the upload was still in flight at both renewals");

        set(&clock, 120);
        assert!(uploading.join().unwrap());
        assert!(matches!(upload.recv().unwrap(), HowASendEnded::Accepted(_)));
        let uploads: Vec<ACallMade> = line.heard().into_iter().filter(|c| c.method == "sendDocument").collect();
        assert_eq!(uploads.len(), 1, "sent once, never cut and sent again");
        assert_eq!(uploads[0].file, Some(("Kitchen - final - Output.txt".to_string(), 51_289)), "arrived whole");
        assert!(uploads[0].reached);
        assert_eq!(uploads[0].bound, crate::transport::THE_BOUND_ON_SENDING_A_FILE, "C6b: never cut for the heartbeat");
        assert_eq!((uploads[0].began, uploads[0].ended), (Duration::ZERO, Duration::from_secs(120)));
    }

    /// **C6b: *"one upload at a time"*.** Two uploads offered together, and two
    /// threads asking to send them: the second begins only when the first has
    /// ended, first offered first.
    #[test]
    fn two_uploads_queue_one_behind_the_other_and_never_go_beside() {
        let clock = Arc::new(AClockMovedByHand::default());
        let line = UploadsInFlight::new(&clock, Duration::from_secs(30));
        let queue = Arc::new(TheSendQueue::new(TheClient::over(line.clone()), Arc::new(RecordsWaits::default())));
        let first = queue.offer_an_upload(an_upload_named("first.mp4", 10));
        let second = queue.offer_an_upload(an_upload_named("second.txt", 20));
        let senders: Vec<_> = (0..2).map(|_| { let queue = queue.clone(); std::thread::spawn(move || queue.send_the_next_upload()) }).collect();

        until("the first upload to be in flight", || line.uploads_inside() == 1);
        std::thread::sleep(Duration::from_millis(20));
        assert_eq!((line.uploads_inside(), queue.how_many_uploads_are_waiting()), (1, 1), "the second waits, not beside");
        set(&clock, 30);
        until("the second upload to be in flight", || line.heard().len() == 1 && line.uploads_inside() == 1);
        set(&clock, 60);
        for sender in senders {
            assert!(sender.join().unwrap());
        }

        assert!(matches!(first.recv().unwrap(), HowASendEnded::Accepted(_)));
        assert!(matches!(second.recv().unwrap(), HowASendEnded::Accepted(_)));
        assert_eq!(line.most_uploads_at_once.load(Ordering::SeqCst), 1);
        let heard = line.heard();
        let order: Vec<&str> = heard.iter().map(|c| c.file.as_ref().unwrap().0.as_str()).collect();
        assert_eq!(order, ["first.mp4", "second.txt"], "first offered, first sent");
        assert!(heard[1].began >= heard[0].ended, "the second began before the first ended");
        assert!(!queue.send_the_next_upload(), "nothing left in the lane");
    }

    /// **C6b: every other send stays in C6's serial lane in C6's order, and
    /// nothing in it waits on an upload.** An ending and an answer offered while
    /// a 120-second upload is in flight go out at once, answer first, and the
    /// upload is still in flight when they have.
    #[test]
    fn an_ending_or_answer_offered_during_an_upload_goes_out_without_waiting_for_it() {
        let clock = Arc::new(AClockMovedByHand::default());
        let line = UploadsInFlight::new(&clock, Duration::from_secs(120));
        let queue = Arc::new(TheSendQueue::new(TheClient::over(line.clone()), Arc::new(RecordsWaits::default())));
        let upload = queue.offer_an_upload(an_upload_named("a moving picture.mp4", 64));
        let uploading = { let queue = queue.clone(); std::thread::spawn(move || queue.send_the_next_upload()) };
        until("the upload to be in flight", || line.uploads_inside() == 1);

        set(&clock, 10);
        let ending = queue.offer(AClassOfSend::EndingsHoldsAndDrops, a_message("Kitchen — final failed"));
        let answer = queue.offer(AClassOfSend::RoutedAnswersAndAcknowledgements, a_message("Nothing was done."));
        while queue.send_the_next().is_some() {}
        assert!(matches!(answer.recv().unwrap(), HowASendEnded::Accepted(_)));
        assert!(matches!(ending.recv().unwrap(), HowASendEnded::Accepted(_)));
        assert_eq!(line.uploads_inside(), 1, "the upload was still in flight");
        assert!(upload.try_recv().is_err());
        let serial: Vec<(String, Duration)> = line.heard().iter().map(|c| (c.method.clone(), c.ended)).collect();
        assert_eq!(serial, [("sendMessage".to_string(), Duration::from_secs(10)), ("sendMessage".to_string(), Duration::from_secs(10))]);

        set(&clock, 120);
        assert!(uploading.join().unwrap());
        assert!(matches!(upload.recv().unwrap(), HowASendEnded::Accepted(_)));
    }

    /// **C3 is not exceeded.** C3's pace is counted in messages, and an upload
    /// is one message however long it takes. Over a 120-second upload, with the
    /// clock's edit every 5 seconds and the heartbeat at each renewal — C3's own
    /// figures for one machine — the upload lane makes exactly one call, never
    /// more than one upload and one serial send are in flight at once, and the
    /// messages begun in the stretch stay under C3's one a second.
    #[test]
    fn an_upload_is_one_message_however_long_it_takes_so_c3s_pace_is_kept() {
        let clock = Arc::new(AClockMovedByHand::default());
        let line = UploadsInFlight::new(&clock, Duration::from_secs(120));
        let queue = Arc::new(TheSendQueue::new(TheClient::over(line.clone()), Arc::new(RecordsWaits::default())));
        queue.the_heartbeat_falls_due(Some((Duration::from_secs(50), clock.clone())));
        let upload = queue.offer_an_upload(an_upload_named("an album.jpg", 64));
        let uploading = { let queue = queue.clone(); std::thread::spawn(move || queue.send_the_next_upload()) };
        until("the upload to be in flight", || line.uploads_inside() == 1);

        for second in (5..120).step_by(5) {
            if second % 50 == 0 {
                renew_at(&queue, &line, &clock, &[second], 50);
            }
            set(&clock, second);
            queue.offer(AClassOfSend::TheClocksEdit, a_message("Server A — rendering"));
            queue.send_the_next();
        }
        set(&clock, 120);
        assert!(uploading.join().unwrap());
        assert!(matches!(upload.recv().unwrap(), HowASendEnded::Accepted(_)));

        let heard = line.heard();
        assert_eq!(heard.iter().filter(|c| c.method == "sendDocument").count(), 1, "C3: an upload is one message");
        assert_eq!(line.most_uploads_at_once.load(Ordering::SeqCst), 1);
        assert_eq!(line.most_calls_at_once.load(Ordering::SeqCst), 2, "one upload beside one serial send, and no more");
        let begun = heard.iter().filter(|c| c.began < Duration::from_secs(120)).count();
        assert!(begun <= 120, "C3: {begun} messages begun in 120 seconds");
        assert_eq!(begun, 1 + 23 + 2, "the upload, 23 clock edits and 2 heartbeats, and nothing sent again");
    }

    /// **`Telegram` C6: one send in flight at a time**, however many threads
    /// ask to send.
    #[test]
    fn only_one_send_is_ever_in_flight() {
        struct CountsWhoIsInside {
            inside: AtomicUsize,
            most_at_once: AtomicUsize,
        }
        impl ReachesTelegram for CountsWhoIsInside {
            fn call(&self, _: &str, _: &WhatACallCarries, _: Duration) -> Result<String, String> {
                let now = self.inside.fetch_add(1, Ordering::SeqCst) + 1;
                self.most_at_once.fetch_max(now, Ordering::SeqCst);
                std::thread::sleep(Duration::from_millis(20));
                self.inside.fetch_sub(1, Ordering::SeqCst);
                Ok(r#"{"ok":true,"result":true}"#.into())
            }
        }
        let wire = Arc::new(CountsWhoIsInside {
            inside: AtomicUsize::new(0),
            most_at_once: AtomicUsize::new(0),
        });
        let queue = Arc::new(TheSendQueue::new(
            TheClient::over(wire.clone()),
            Arc::new(RecordsWaits::default()),
        ));
        for _ in 0..4 {
            queue.offer(AClassOfSend::FramePicturesAndMedia, a_message("picture"));
        }
        let senders: Vec<_> = (0..4)
            .map(|_| {
                let queue = queue.clone();
                std::thread::spawn(move || queue.send_the_next())
            })
            .collect();
        for sender in senders {
            sender.join().unwrap();
        }
        assert_eq!(wire.most_at_once.load(Ordering::SeqCst), 1);
        assert_eq!(queue.how_many_are_waiting(), 0);
    }
}
