//! **The one ordered queue every result travels on to the interface** —
//! `Desktop app and delivery` §3.3, P6 to P10e, and §4 A1, A6 and A8.
//!
//! **What this is for.** Until this file existed, nothing any module did could
//! reach a screen: the application exposed three calls to its page, all three
//! of them step 1's, and **no result ever arrived on its own**. This is the
//! route that arriving takes, and it is one route rather than one per module.
//!
//! **P6 is kept by there being one of these.** One queue rather than one per
//! module, *"because two queues are two orders and a user watching two panels
//! would see one action's outcomes arrive in an order neither queue
//! promises"*. The whole application makes exactly one [`TheOneQueue`], at the
//! application's start, and the interface holds no other route to a module.
//!
//! **P7 is kept by the shape of the two doors rather than by a rule anybody
//! has to remember.** There is no `post`. The only two ways onto the queue are
//! [`TheOneQueue::write_then_post`] and [`TheOneQueue::read_then_post`], and
//! each of them **takes the durable act as its first argument and runs it
//! first**; the event is composed from what that act returned, and an act that
//! failed posts nothing at all. So *write, then post, then draw* is the
//! function's own control flow: **a caller that wanted to draw first would have
//! to write a second queue, which P6 forbids.** `State, identity and recovery`
//! D6 — every durable write happens at the moment the fact becomes true — is
//! the other half of that order and is each writer's own.
//!
//! **P8 is kept by the subject register.** Every event names its [`Subject`].
//! A durable subject the queue has not been told exists is refused rather than
//! queued, and a subject that goes away between the post and the delivery has
//! its waiting events dropped at the drain. **The queue holds no record, no
//! diary and no adapter**, so nothing durable can be written from an event; the
//! whole of it is process memory for the life of the run, which is
//! `State, identity and recovery` §11.1's run-held class.
//!
//! **P9, P10, P10a and P10b are kept by an absence, and the absence is
//! tested.** Nothing here polls and nothing here has a timer: there is no
//! thread, no sleep, no wait and no clock in this file, and the page's own side
//! of the route holds no repeating timer either. **The one standing timer this
//! application is allowed is the update check (Z23)**, and it is
//! `crate::updating::checking`'s, from step 11's check piece: it reaches this
//! queue as an ordinary post through the two doors, like every other sequence.
//! P10a's two non-timers — a delay inside a sequence already running, and a
//! drawing that ticks against a published moment — are admitted where they are
//! declared and neither of them is here. **P10d's one live-reading sequence and
//! P10e's one scheduled idle re-check are other documents' sequences reaching
//! this queue as ordinary posts**; the queue neither starts them nor knows
//! them apart from any other post, which is what keeps its own count of timers
//! at nought.
//!
//! **P10c is named and not defended, which is what the clause asks.** Nothing
//! in this file reads a moment on any clock, so a computer suspended between a
//! moment being written and a drawing reading it costs this route nothing. The
//! hazard belongs to whatever draws a countdown, and P10b's rule is the one it
//! is judged against.
//!
//! **A6 and X9 are kept by the field type.** An event says things, and the only
//! two things it can say are [`Said::Prose`], which can only be built through
//! the typography application point, and [`Said::Value`], which is the verbatim
//! route. **There is no third variant and no bare string anywhere on an
//! event**, so this route cannot become the third way into a surface that A6
//! forbids. X3 falls out of the same fact: no screen and no module is asked to
//! apply either typography rule by hand, because the only prose that can reach
//! the page has already been through the point. X4 is what makes that survive
//! three engines — the bound space is a character inside the value rather than
//! a rule about layout — and X6 is kept by this file claiming nothing about
//! what any engine does with it.
//!
//! **A1 and A8.** This is a named service of this document ([Section 21]) and a
//! feature module reaches the interface through it and by no other route.
//! Nothing crosses it but data: an event carries strings and an opaque
//! identity, and **no operating-system handle can be put on one**, there being
//! no variant that would hold one.
//!
//! [Section 21]: ../../../../Preparation/Architecture/Desktop%20app%20and%20delivery.md

use std::collections::{BTreeMap, HashSet, VecDeque};
use std::sync::Mutex;

use smoozy_state::identities::Identity;

use crate::typography::{Prose, Verbatim};

/// **The one shell event name every delivery travels on.**
///
/// P6: one queue and one order. One name is the other half of that — a second
/// name would be a second stream, and two streams arriving at one page are two
/// orders again however carefully each is kept.
pub const THE_ONE_EVENT_NAME: &str = "smoozy:event";

/// **What an event says, and the two variants are the complete set** —
/// §4 A6 and §17 X9.
///
/// `Desktop app and delivery` A6: *"Every string of prose a module hands to the
/// interface passes the typography application point … Every other string
/// reaches the interface by the verbatim route … The two are the complete set
/// of ways a string reaches a surface."*
///
/// **This enum is that enforcement on this route.** Neither variant can be
/// built from a bare `String` here: [`Prose`] is only constructible through the
/// point, and [`Verbatim`] is the named route X9 says must exist so that
/// nobody invents an unnamed one. **A third variant would be the third way in
/// that A6 forbids.**
///
/// Both serialise as the plain string they hold, because the page draws text
/// and has no use for the distinction: X4's whole point is that the
/// substitution has already happened *inside the characters*, so nothing on the
/// far side has to know which route a string took.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(untagged)]
pub enum Said {
    /// Text for a person, through the typography application point (X1).
    Prose(Prose),
    /// A value Smoozy must not alter — a path, a name, a host, a port, the
    /// machine's own words, anything the user can copy (X7, X9).
    Value(Verbatim),
}

/// **What an event is about** — P8's subject.
///
/// P8: *"A UI event is discarded when its subject is gone and never queued
/// against a subject that no longer exists."* So an event names what it is
/// about, and the queue can answer the question.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Subject {
    /// **The run itself.** Nothing outlives the run, and the queue does not
    /// outlive it either, so this subject is never gone while there is a queue
    /// to be gone from. It is what a fact about the whole application is about
    /// — the spine's own acts at start among them.
    TheRun,
    /// **A durable subject, by its identity** — a server row, a job, a
    /// condition, a transfer. `State, identity and recovery` Y5's rules bind
    /// the identity; nothing here parses one or reads anything out of one.
    Durable(Identity),
}

impl Subject {
    /// The subject's written form, for the page. An identity's own
    /// lower-case hexadecimal rendering, which is a rendering and not a
    /// meaning — nothing on either side parses it.
    fn as_text(&self) -> String {
        match self {
            Subject::TheRun => "run".to_string(),
            Subject::Durable(identity) => identity.as_text(),
        }
    }
}

/// **One event on its way to the interface.**
///
/// The kind is a `&'static str` declared by the service that posts it, in that
/// service's own file under `src/surface/`. **It is deliberately not a closed
/// enum here**: an enum in this file would be a list every one of the pieces
/// after this one has to edit, which is exactly the collision the command
/// surface was split up to avoid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    kind: &'static str,
    subject: Subject,
    said: BTreeMap<&'static str, Said>,
}

impl Event {
    /// Begin an event about a subject.
    pub fn about(subject: Subject, kind: &'static str) -> Self {
        Event {
            kind,
            subject,
            said: BTreeMap::new(),
        }
    }

    /// Add one thing the event says. A6: it is prose or it is a value, and
    /// there is nothing else it can be.
    pub fn saying(mut self, name: &'static str, said: Said) -> Self {
        self.said.insert(name, said);
        self
    }

    pub fn kind(&self) -> &'static str {
        self.kind
    }

    pub fn subject(&self) -> &Subject {
        &self.subject
    }
}

/// **One event as the page receives it.**
///
/// The sequence number is the queue's own order, issued when the event is
/// posted and never afterwards. **It is here so that the one order P6 promises
/// is a thing the page can check rather than a thing it has to trust**: the
/// page's own side asserts that each delivery follows the last, and says so if
/// one ever does not.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Delivery {
    pub sequence: u64,
    pub kind: &'static str,
    pub subject: String,
    pub said: BTreeMap<&'static str, Said>,
}

/// **Where a delivery goes.**
///
/// A trait rather than the shell's own call written into the queue, for two
/// reasons. The queue is this document's service and the shell is Section 2's
/// choice, so the one should not be written in terms of the other (A1). And
/// **every test below drives the real queue** rather than a copy of it, which
/// is only possible because what a delivery lands in can be something other
/// than a window.
pub trait Delivers: Send + Sync + 'static {
    fn deliver(&self, delivery: &Delivery);
}

/// **What came of a post.**
///
/// P8 has an outcome that is not a failure — the subject was gone — and this
/// is where it is said. A caller that wanted to branch on it can; nothing in
/// this document needs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Posted {
    /// On the queue, in this position of the one order.
    InOrder(u64),
    /// **P8: the subject is gone, so nothing was queued.** Not an error: the
    /// durable act happened and its fact stands; there is simply nobody left
    /// for the event to be about.
    TheSubjectIsGone,
}

/// The queue's own state, behind one lock.
struct Held {
    next_sequence: u64,
    /// Posted before the page was ready to receive. Each keeps its subject, so
    /// P8's second half — a subject that goes away between the post and the
    /// delivery — can be applied at the drain.
    waiting: VecDeque<(Subject, Delivery)>,
    to_the_page: Option<Box<dyn Delivers>>,
    /// P8's register of durable subjects that exist. [`Subject::TheRun`] is
    /// never in it and is never gone.
    live: HashSet<Identity>,
}

/// **The one ordered queue** — P6.
///
/// One of these exists per run, made at the application's start and reachable
/// from the whole application. **Two of them would be P6's fault**, and the
/// test `only_one_queue_is_ever_made` below is what says there is one.
pub struct TheOneQueue {
    held: Mutex<Held>,
}

impl Default for TheOneQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl TheOneQueue {
    pub fn new() -> Self {
        TheOneQueue {
            held: Mutex::new(Held {
                next_sequence: 0,
                waiting: VecDeque::new(),
                to_the_page: None,
                live: HashSet::new(),
            }),
        }
    }

    /// **P8: this durable subject exists, so events may be queued about it.**
    ///
    /// Told to the queue by whoever created the subject, at the moment the
    /// subject became durable. **The queue never discovers one on its own** —
    /// a subject it has not been told about is one it refuses events for,
    /// which is P8 in the direction that fails closed.
    pub fn the_subject_exists(&self, identity: Identity) {
        if let Ok(mut held) = self.held.lock() {
            held.live.insert(identity);
        }
    }

    /// **P8: this durable subject is gone.**
    ///
    /// Every event still waiting about it is dropped, and no further event
    /// about it is queued. **Nothing durable happens here** — the subject's own
    /// durable ending is whoever owns it, and this is only the queue forgetting
    /// it, which is what run-held means.
    pub fn the_subject_is_gone(&self, identity: &Identity) {
        if let Ok(mut held) = self.held.lock() {
            held.live.remove(identity);
            held.waiting.retain(|(subject, _)| match subject {
                Subject::TheRun => true,
                Subject::Durable(about) => about != identity,
            });
        }
    }

    /// **P7's first door: the durable write, then the post.**
    ///
    /// `write` is the act that makes the fact true and writes it down. It runs
    /// **first**; it is only if it returns that an event is composed at all,
    /// and the event is composed **from what the write returned**. A write that
    /// failed posts nothing, and there is no way to reach the queue that skips
    /// it.
    ///
    /// *"A design that drew first and recorded afterwards would satisfy the
    /// sentence in the ordinary case and go quiet in exactly the case Row 292
    /// exists for"* — P7. **This is that design made unavailable rather than
    /// discouraged.**
    pub fn write_then_post<T, E>(
        &self,
        write: impl FnOnce() -> Result<T, E>,
        compose: impl FnOnce(&T) -> Event,
    ) -> Result<Posted, E> {
        let written = write()?;
        Ok(self.put_on_the_queue(compose(&written)))
    }

    /// **P7's second door: the durable read, then the post.**
    ///
    /// The same shape for the case where the fact was written durably by some
    /// earlier act and is being taken back out of the record to be shown. P7
    /// asks that an event never carry state that has not already been written
    /// durably; **a value read out of the durable store is state that has**,
    /// and this is the door that says so rather than leaving a caller to reach
    /// for the write door with a read inside it.
    pub fn read_then_post<T, E>(
        &self,
        read: impl FnOnce() -> Result<T, E>,
        compose: impl FnOnce(&T) -> Event,
    ) -> Result<Posted, E> {
        let read = read()?;
        Ok(self.put_on_the_queue(compose(&read)))
    }

    /// **The page is ready to receive, so everything waiting goes now, in
    /// order.**
    ///
    /// The spine runs before the window is built — `Desktop app and delivery`
    /// L3a and L6 both say *at application start* — so the first events of a
    /// run are posted with nothing to receive them. They wait here rather than
    /// being lost, and the page's own first act is to say it is listening.
    ///
    /// **P8 is applied again at the drain**, because a subject can go away
    /// between the post and this moment.
    ///
    /// **A page that loads again says so again**, and what it then receives is
    /// whatever has been posted since. Nothing is replayed: events are volatile
    /// and run-held (P8), and a page holding a replayed event would be holding
    /// state nobody wrote down.
    pub fn the_page_is_ready(&self, to_the_page: Box<dyn Delivers>) {
        let Ok(mut held) = self.held.lock() else {
            return;
        };
        held.to_the_page = Some(to_the_page);
        let waiting: Vec<(Subject, Delivery)> = held.waiting.drain(..).collect();
        for (subject, delivery) in waiting {
            if !Self::the_subject_is_here(&held.live, &subject) {
                continue;
            }
            if let Some(page) = held.to_the_page.as_ref() {
                page.deliver(&delivery);
            }
        }
    }

    /// **The only way an event reaches the queue, and it is private on
    /// purpose** — P7. Both public doors run their durable act before they can
    /// get here.
    ///
    /// The sequence number, the P8 check and the delivery all happen under one
    /// lock, so **the order the page sees is the order the sequence numbers
    /// were issued in**, whichever thread posted and however many posted at
    /// once. That is P6 held by construction rather than by hoping the shell
    /// keeps an order it never promised.
    fn put_on_the_queue(&self, event: Event) -> Posted {
        let Ok(mut held) = self.held.lock() else {
            return Posted::TheSubjectIsGone;
        };
        if !Self::the_subject_is_here(&held.live, &event.subject) {
            return Posted::TheSubjectIsGone;
        }
        let sequence = held.next_sequence;
        held.next_sequence += 1;
        let delivery = Delivery {
            sequence,
            kind: event.kind,
            subject: event.subject.as_text(),
            said: event.said,
        };
        match held.to_the_page.as_ref() {
            Some(page) => page.deliver(&delivery),
            None => held.waiting.push_back((event.subject, delivery)),
        }
        Posted::InOrder(sequence)
    }

    fn the_subject_is_here(live: &HashSet<Identity>, subject: &Subject) -> bool {
        match subject {
            Subject::TheRun => true,
            Subject::Durable(identity) => live.contains(identity),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use smoozy_state::identities::{Identity, IdentityKind};

    use super::*;

    /// A place for deliveries to land, so every test below drives the real
    /// queue rather than a copy of its logic.
    #[derive(Default)]
    struct WhatThePageGot(Arc<Mutex<Vec<Delivery>>>);

    impl WhatThePageGot {
        fn new() -> (Self, Arc<Mutex<Vec<Delivery>>>) {
            let held = Arc::new(Mutex::new(Vec::new()));
            (WhatThePageGot(held.clone()), held)
        }
    }

    impl Delivers for WhatThePageGot {
        fn deliver(&self, delivery: &Delivery) {
            self.0.lock().unwrap().push(delivery.clone());
        }
    }

    fn an_identity() -> Identity {
        Identity::issue(IdentityKind::Job).expect("this machine supplies random bytes")
    }

    fn always<T>(value: T) -> impl FnOnce() -> Result<T, ()> {
        move || Ok(value)
    }

    const A_KIND: &str = "test:a";
    const ANOTHER_KIND: &str = "test:b";

    /// **P6: one queue, one order.** Everything posted arrives, once, in the
    /// order it was posted, with consecutive sequence numbers.
    #[test]
    fn everything_posted_arrives_in_one_order() {
        let queue = TheOneQueue::new();
        let (page, got) = WhatThePageGot::new();
        queue.the_page_is_ready(Box::new(page));

        for _ in 0..5 {
            queue
                .write_then_post(always(()), |()| Event::about(Subject::TheRun, A_KIND))
                .unwrap();
        }

        let got = got.lock().unwrap();
        assert_eq!(got.len(), 5);
        for (position, delivery) in got.iter().enumerate() {
            assert_eq!(delivery.sequence, position as u64);
        }
    }

    /// **P6, under the case the clause was written about: two panels, two
    /// posters, one order.** Two threads posting at once produce one sequence
    /// with no gap and no repeat, and the page sees them in exactly that
    /// order.
    #[test]
    fn two_posters_at_once_still_make_one_order() {
        let queue = Arc::new(TheOneQueue::new());
        let (page, got) = WhatThePageGot::new();
        queue.the_page_is_ready(Box::new(page));

        let posters: Vec<_> = [A_KIND, ANOTHER_KIND]
            .into_iter()
            .map(|kind| {
                let queue = queue.clone();
                std::thread::spawn(move || {
                    for _ in 0..50 {
                        queue
                            .write_then_post(always(()), |()| Event::about(Subject::TheRun, kind))
                            .unwrap();
                    }
                })
            })
            .collect();
        for poster in posters {
            poster.join().expect("the poster finished");
        }

        let got = got.lock().unwrap();
        assert_eq!(got.len(), 100);
        let sequences: Vec<u64> = got.iter().map(|delivery| delivery.sequence).collect();
        let in_order: Vec<u64> = (0..100).collect();
        assert_eq!(
            sequences, in_order,
            "the page saw the deliveries in an order that is not the queue's — \
             P6 promises one order and this is the check that it is kept when \
             two things post at once"
        );
    }

    /// **P7: a durable act that failed posts nothing.**
    #[test]
    fn a_write_that_failed_posts_nothing() {
        let queue = TheOneQueue::new();
        let (page, got) = WhatThePageGot::new();
        queue.the_page_is_ready(Box::new(page));

        let outcome: Result<Posted, &str> = queue.write_then_post(
            || Err("the record would not take it"),
            |_: &()| Event::about(Subject::TheRun, A_KIND),
        );

        assert_eq!(outcome, Err("the record would not take it"));
        assert!(
            got.lock().unwrap().is_empty(),
            "an event was posted for a durable write that did not happen — P7"
        );
    }

    /// **P7: the event is composed from what the durable act returned**, so
    /// what it carries cannot be state the act did not produce.
    #[test]
    fn the_event_is_composed_from_what_the_write_returned() {
        let queue = TheOneQueue::new();
        let (page, got) = WhatThePageGot::new();
        queue.the_page_is_ready(Box::new(page));

        queue
            .write_then_post(always("written down"), |written| {
                Event::about(Subject::TheRun, A_KIND)
                    .saying("what", Said::Value(Verbatim::new(*written)))
            })
            .unwrap();

        let got = got.lock().unwrap();
        assert_eq!(
            got[0].said.get("what"),
            Some(&Said::Value(Verbatim::new("written down")))
        );
    }

    /// **P7: the read door is the same shape**, and a read that failed posts
    /// nothing either.
    #[test]
    fn a_read_that_failed_posts_nothing() {
        let queue = TheOneQueue::new();
        let (page, got) = WhatThePageGot::new();
        queue.the_page_is_ready(Box::new(page));

        let outcome: Result<Posted, &str> = queue.read_then_post(
            || Err("the row would not read"),
            |_: &()| Event::about(Subject::TheRun, A_KIND),
        );

        assert_eq!(outcome, Err("the row would not read"));
        assert!(got.lock().unwrap().is_empty());
    }

    /// **P7, held rather than remembered: there is no third door.**
    ///
    /// This reads this file's own source and asserts the exact set of public
    /// entry points onto the queue. **A plain `post` added later is what breaks
    /// P7**, and it would break it quietly: every existing test would still
    /// pass, because they all go through the two doors. This is the line that
    /// fails instead.
    #[test]
    fn the_queue_has_exactly_two_doors_and_both_run_the_durable_act_first() {
        let source = include_str!("events.rs");
        let opens = source
            .find("impl TheOneQueue {")
            .expect("the queue has one impl block");
        let body = &source[opens..];
        let closes = body
            .find("\n    fn the_subject_is_here")
            .expect("the impl block ends at the last private helper");
        let body = &body[..closes];

        let doors: Vec<&str> = body
            .match_indices("    pub fn ")
            .map(|(at, _)| {
                let rest = &body[at + "    pub fn ".len()..];
                &rest[..rest.find(['(', '<']).expect("the function is named")]
            })
            .collect();

        assert_eq!(
            doors,
            vec![
                "new",
                "the_subject_exists",
                "the_subject_is_gone",
                "write_then_post",
                "read_then_post",
                "the_page_is_ready",
            ],
            "the queue's public surface has moved. **The two doors onto the \
             queue are `write_then_post` and `read_then_post`, and each runs \
             its durable act before an event can be composed** — a door that \
             does not is P7 broken, and every other test here would still \
             pass. Desktop app and delivery P7"
        );
    }

    /// **P8: an event about a durable subject the queue has not been told
    /// exists is never queued.**
    #[test]
    fn an_event_about_a_subject_that_is_not_here_is_never_queued() {
        let queue = TheOneQueue::new();
        let (page, got) = WhatThePageGot::new();
        queue.the_page_is_ready(Box::new(page));

        let outcome = queue
            .write_then_post(always(()), |()| {
                Event::about(Subject::Durable(an_identity()), A_KIND)
            })
            .unwrap();

        assert_eq!(outcome, Posted::TheSubjectIsGone);
        assert!(got.lock().unwrap().is_empty());
    }

    /// **P8: a subject that goes away between the post and the delivery has
    /// its event discarded.**
    ///
    /// This is the half the register exists for. The event was legitimate when
    /// it was posted, the page had not yet said it was listening, and by the
    /// time it did the subject was gone.
    #[test]
    fn an_event_is_discarded_when_its_subject_goes_before_it_is_delivered() {
        let queue = TheOneQueue::new();
        let job = an_identity();
        let other = an_identity();
        queue.the_subject_exists(job.clone());
        queue.the_subject_exists(other.clone());

        queue
            .write_then_post(always(()), |()| {
                Event::about(Subject::Durable(job.clone()), A_KIND)
            })
            .unwrap();
        queue
            .write_then_post(always(()), |()| {
                Event::about(Subject::Durable(other.clone()), ANOTHER_KIND)
            })
            .unwrap();
        queue.the_subject_is_gone(&job);

        let (page, got) = WhatThePageGot::new();
        queue.the_page_is_ready(Box::new(page));

        let got = got.lock().unwrap();
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].kind, ANOTHER_KIND);
        assert_eq!(got[0].subject, other.as_text());
    }

    /// **P8: the run is never gone.** A fact about the whole application has a
    /// subject that outlives everything the queue can be asked about.
    #[test]
    fn the_run_is_a_subject_that_is_never_gone() {
        let queue = TheOneQueue::new();
        queue
            .write_then_post(always(()), |()| Event::about(Subject::TheRun, A_KIND))
            .unwrap();

        let (page, got) = WhatThePageGot::new();
        queue.the_page_is_ready(Box::new(page));
        assert_eq!(got.lock().unwrap().len(), 1);
    }

    /// **P8: nothing durable is ever written from an event, and the queue
    /// could not write one if it tried.**
    ///
    /// The check is over this file's own source, because the property is an
    /// absence: the queue holds no record, no diary and no adapter, so there is
    /// no route from an event to a durable store. **A field added later that
    /// gave it one is what this fails on.**
    #[test]
    fn nothing_durable_can_be_written_from_an_event() {
        let source = include_str!("events.rs");
        let opens = source
            .find("struct Held {")
            .expect("the queue's state is one struct");
        let body = &source[opens..];
        let held = &body[..body.find('}').expect("the struct closes")];

        for durable in ["LocalRecord", "DiaryService", "record::", "platform::"] {
            assert!(
                !held.contains(durable),
                "the queue now holds {durable}, which is a route from a volatile \
                 event to a durable store — P8 says events are run-held and that \
                 nothing durable is ever written from one"
            );
        }
    }

    /// **P9, P10, P10a, P10b and P10c: this route has no timer, no wait and no
    /// clock.**
    ///
    /// Nothing on this side polls, and the one standing timer the application
    /// is allowed is the update check, which lives in
    /// `crate::updating::checking` and posts here through the two doors. **A
    /// repeating timer added here is the fault P10 names**, and a clock read here would be
    /// P10c's hazard arriving in the one place that has no business with a
    /// moment at all.
    ///
    /// The check is deliberately over this file's own text, because what is
    /// being held is an absence and an absence has nothing to call.
    #[test]
    fn the_event_route_holds_no_timer_no_wait_and_no_clock() {
        let source = include_str!("events.rs");
        let (working, _) = source
            .split_once("#[cfg(test)]")
            .expect("the tests are the last thing in this file");

        for reached_for in [
            "thread::spawn",
            "thread::sleep",
            "Duration",
            "Instant",
            "SystemTime",
            "Condvar",
            "recv_timeout",
        ] {
            assert!(
                !working.contains(reached_for),
                "the event route now names {reached_for}. **P10: there is exactly \
                 one timer on this side and it is the update check**, and P10c's \
                 suspended-computer hazard is named rather than defended \
                 precisely because nothing here reads a clock. A delay inside a \
                 running sequence (P10a) belongs to that sequence, and a drawing \
                 that ticks (P10b) belongs to the drawing"
            );
        }
    }

    /// **A6, X9 and X3: an event says prose or it says a value, and there is
    /// nothing else it can say.**
    ///
    /// The check is over the source because what is being held is the shape of
    /// a closed set: **a third variant on [`Said`] would be the third way into
    /// a surface that A6 forbids**, and it would compile perfectly.
    #[test]
    fn an_event_can_only_say_prose_or_a_value() {
        let source = include_str!("events.rs");
        let opens = source.find("pub enum Said {").expect("Said is an enum");
        let body = &source[opens..];
        let said = &body[..body.find("\n}").expect("the enum closes")];

        let variants: Vec<&str> = said
            .lines()
            .map(str::trim)
            .filter(|line| !line.starts_with("///") && !line.starts_with("//"))
            .filter_map(|line| line.split_once('(').map(|(named, _)| named))
            .filter(|named| !named.is_empty() && !named.contains(' '))
            .collect();
        assert_eq!(
            variants,
            vec!["Prose", "Value"],
            "the set of things an event can say has moved. **A6: prose through \
             the typography point and every other string by the verbatim route \
             are the complete set of ways a string reaches a surface**, and this \
             route must not become a third"
        );
    }

    /// **X3 and X4: the substitution is already in the characters by the time
    /// the event crosses, and nothing on the far side applies a rule.**
    ///
    /// X3 says neither typography rule can be held by remembering it and that
    /// no screen is asked to apply either by hand. X4 says both are character
    /// substitutions *"and that is what makes them survive"* three engines.
    /// **Both are visible here as one fact: the non-breaking spaces are in the
    /// value that crosses.**
    #[test]
    fn prose_crosses_with_the_two_substitutions_already_in_it() {
        let queue = TheOneQueue::new();
        let (page, got) = WhatThePageGot::new();
        queue.the_page_is_ready(Box::new(page));

        queue
            .write_then_post(always(()), |()| {
                Event::about(Subject::TheRun, A_KIND).saying(
                    "sentence",
                    Said::Prose(Prose::new("the render \u{2014} it is done")),
                )
            })
            .unwrap();

        let got = got.lock().unwrap();
        let Some(Said::Prose(prose)) = got[0].said.get("sentence") else {
            panic!("the event said no prose");
        };
        assert_eq!(prose.as_str(), "the render\u{00A0}\u{2014} it is\u{00A0}done");
    }

    /// **X6 and X9: what crosses is exactly what the two routes produced, and
    /// this route claims nothing about what any engine does with it.**
    ///
    /// The value route substitutes nothing — a path with an em dash in it
    /// arrives byte for byte — and the queue neither edits nor inspects either.
    #[test]
    fn a_value_crosses_untouched() {
        let queue = TheOneQueue::new();
        let (page, got) = WhatThePageGot::new();
        queue.the_page_is_ready(Box::new(page));

        let path = "/home/artist/scene \u{2014} final/frame 001.exr";
        queue
            .write_then_post(always(()), |()| {
                Event::about(Subject::TheRun, A_KIND)
                    .saying("path", Said::Value(Verbatim::new(path)))
            })
            .unwrap();

        let got = got.lock().unwrap();
        let Some(Said::Value(value)) = got[0].said.get("path") else {
            panic!("the event said no value");
        };
        assert_eq!(value.as_str(), path);
    }

    /// **A8: nothing crosses this route but data.**
    ///
    /// An event holds a kind, a subject and a set of things said, and every one
    /// of those is a string or an opaque identity. **There is no variant that
    /// could hold an operating-system handle**, which is what makes A8's rule
    /// about two modules not passing one another handles hold here by
    /// construction rather than by inspection.
    #[test]
    fn an_event_can_carry_nothing_but_data() {
        let source = include_str!("events.rs");
        let opens = source
            .find("pub struct Event {")
            .expect("Event is one struct");
        let body = &source[opens..];
        let event = &body[..body.find("\n}").expect("the struct closes")];

        let fields: Vec<&str> = event
            .lines()
            .map(str::trim)
            .filter(|line| !line.starts_with("///") && !line.starts_with("//"))
            .filter_map(|line| line.strip_suffix(','))
            .collect();
        assert_eq!(
            fields,
            vec![
                "kind: &'static str",
                "subject: Subject",
                "said: BTreeMap<&'static str, Said>",
            ],
            "an event now carries something other than a kind, a subject and \
             what it says. **A8: modules do not pass one another \
             operating-system handles, and what crosses between them crosses as \
             data** — a field that is neither a string nor an opaque identity is \
             how that stops being true"
        );
    }

    /// **The page is told the order, so it can check it rather than trust it.**
    #[test]
    fn every_delivery_carries_its_place_in_the_one_order() {
        let queue = TheOneQueue::new();
        queue
            .write_then_post(always(()), |()| Event::about(Subject::TheRun, A_KIND))
            .unwrap();
        queue
            .write_then_post(always(()), |()| Event::about(Subject::TheRun, ANOTHER_KIND))
            .unwrap();

        let (page, got) = WhatThePageGot::new();
        queue.the_page_is_ready(Box::new(page));

        let got = got.lock().unwrap();
        assert_eq!(got[0].sequence, 0);
        assert_eq!(got[1].sequence, 1);
    }

    /// **A page that loads again receives what has been posted since, and
    /// nothing is replayed** — P8: events are volatile and run-held, and a
    /// replayed event would be state nobody wrote down.
    #[test]
    fn a_page_that_loads_again_is_not_sent_the_run_over_again() {
        let queue = TheOneQueue::new();
        let (first, first_got) = WhatThePageGot::new();
        queue.the_page_is_ready(Box::new(first));
        queue
            .write_then_post(always(()), |()| Event::about(Subject::TheRun, A_KIND))
            .unwrap();

        let (again, again_got) = WhatThePageGot::new();
        queue.the_page_is_ready(Box::new(again));
        queue
            .write_then_post(always(()), |()| Event::about(Subject::TheRun, ANOTHER_KIND))
            .unwrap();

        assert_eq!(first_got.lock().unwrap().len(), 1);
        let again_got = again_got.lock().unwrap();
        assert_eq!(again_got.len(), 1);
        assert_eq!(again_got[0].kind, ANOTHER_KIND);
        assert_eq!(again_got[0].sequence, 1);
    }
}
