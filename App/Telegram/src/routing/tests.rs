//! **Routing on a simulated conversation, with a fleet of listeners.** The
//! simulation keeps the observed ground of `Telegram` §3 and §5, as the
//! listener's own tests do: the newest ask wins; a pin is a stack and produces
//! an update carrying the pinned message; `getChat` reports **the most recent
//! pinned message by sending date**, as `Telegram` B13 quotes the page, and not
//! the one pinned last — until 2026-09-13's second sitting this stand-in
//! reported the one pinned last, and so could not show why the first live run
//! gathered only the holder's reply; a
//! tap arrives as a `callback_query` carrying its message and the button's
//! exact data.

#[cfg(test)]
mod on_a_simulated_conversation {
    use std::collections::{HashMap, HashSet};
    use std::sync::{Arc, Mutex, MutexGuard};
    use std::time::Duration;

    use serde_json::{json, Value};

    use crate::actions::a_stand_in::AStandInSupervisor;
    use crate::actions::{TheJobMessages, TheListenersAcknowledgement, TheOwnersActs, TheSubject};
    use crate::board::instructions::{admit, AKindOfEntry, AnAdmission, AnEntry, THE_DEFAULT_RESOLUTION_DEADLINE};
    use crate::job_messages::STOP_THIS_JOB;
    use crate::board::staleness::THE_READ_CADENCE;
    use crate::board::text::{what_is_pinned, TheBoard, WhatIsPinned, THE_HEAD};
    use crate::listener::{ActsOnAnInstruction, ReadsItsOwnClock, TheListener, TheSettings, WhatTheListenerIsHanded, WhoThisMachineIs};
    use crate::pacing::{TheSendQueue, Waits};
    use crate::routing::context::{the_owner_finds_the_subject_live, AnActionContextRecord, SENDING_A_FRAME};
    use crate::routing::gather::{AJobAsHeld, ReadsWhatThisMachineHolds, TheProgress};
    use crate::routing::typed::{A_BUTTON_THAT_EXPIRED, NOT_A_REQUEST, NO_JOB_TO_SEND_A_FRAME_FROM, WHICH_JOB};
    use crate::routing::{TheRouter, WhatTheRouterIsHanded};
    use crate::transport::{ReachesTelegram, TheClient, WhatACallCarries};
    use crate::worker_state::{InMemory, TheTelegramWorkersState};

    const CHAT: i64 = 4242;

    fn lock<T>(held: &Mutex<T>) -> MutexGuard<'_, T> {
        held.lock().unwrap_or_else(|p| p.into_inner())
    }

    #[derive(Default)]
    struct TheConversation {
        next_message: i64,
        next_update: i64,
        next_callback: i64,
        texts: HashMap<i64, String>,
        buttons: HashMap<i64, Value>,
        pins: Vec<i64>,
        updates: Vec<Value>,
        confirmed_below: i64,
        open_ask: Option<String>,
        refused_next: HashSet<String>,
        heard: Vec<(String, String, Value)>,
        /// Pinned-message lines written in the conversation: one per message
        /// going from unpinned to pinned (B19).
        lines: usize,
        /// **How many reads after an unpin report nothing pinned.** Set by a
        /// test to reproduce what the second live watch of 2026-09-14 most
        /// likely met: for about a second after an unpin, the service reported
        /// no pinned message while the board stayed pinned. Zero is the old
        /// stand-in.
        blind_reads_after_an_unpin: usize,
        blind_for: usize,
        /// Each message's spoiler as last sent or edited (B25).
        entities: HashMap<i64, Value>,
        /// **B25: where set, a pin update and `getChat` hand back the pinned
        /// message's text with no entity list**, which Telegram's pages do not
        /// promise either way. Unset, they carry it.
        reads_drop_the_marking: bool,
        /// **R16**: the taps each caller was handed and has not yet answered,
        /// and every answer made, by caller and tap.
        taps_owed: HashMap<String, Vec<String>>,
        answered: Vec<(String, String)>,
        /// **`Telegram` R17 — the bot's command list as Telegram holds it.**
        commands: Value,
    }

    impl TheConversation {
        /// **`Telegram` R16, enforced by the stand-in: fails the test where a
        /// caller handed a tap from the proved conversation makes any other
        /// call before answering it, where an answer carries anything but the
        /// tap's identifier, or where a caller answers a tap it was not handed
        /// or answers one twice.** So a tap routed before it is answered, a tap
        /// answered by its owner rather than its receiver, and an answer to
        /// another conversation's tap all fail.
        fn the_taps_are_answered_first(&mut self, caller: &str, method: &str, fields: &Value) {
            let owed = self.taps_owed.entry(caller.to_string()).or_default();
            if method != "answerCallbackQuery" {
                assert!(owed.is_empty(), "R16: {caller} made a {method} call before answering the taps it was handed: {owed:?}");
                return;
            }
            let carried = fields.as_object().map_or(0, |f| f.len());
            assert!(
                carried == 1 && fields.get("callback_query_id").is_some(),
                "R16: an answer carried more than the tap's identifier: {fields}"
            );
            let id = fields["callback_query_id"].as_str().unwrap_or_default().to_string();
            let Some(at) = owed.iter().position(|o| *o == id) else {
                panic!("R16: {caller} answered tap {id}, which it was not handed or had already answered");
            };
            owed.remove(at);
            self.answered.push((caller.to_string(), id));
        }
        /// The taps from the proved conversation an ask hands a caller, which
        /// it has not already answered.
        fn owe_the_taps(&mut self, caller: &str, handed: &[Value]) {
            for update in handed {
                if update.pointer("/callback_query/message/chat/id").and_then(Value::as_i64) != Some(CHAT) {
                    continue;
                }
                let Some(id) = update.pointer("/callback_query/id").and_then(Value::as_str) else { continue };
                if self.answered.iter().any(|(c, i)| c == caller && i == id) {
                    continue;
                }
                let owed = self.taps_owed.entry(caller.to_string()).or_default();
                if !owed.iter().any(|o| o == id) {
                    owed.push(id.to_string());
                }
            }
        }
        /// **R16: nothing handed to this caller is left unanswered.**
        fn nothing_is_owed_by(&self, caller: &str) {
            let owed = self.taps_owed.get(caller);
            assert!(owed.is_none_or(Vec::is_empty), "R16: {caller} left taps unanswered: {owed:?}");
        }
        fn keep_the_marking(&mut self, id: i64, fields: &Value) {
            match fields.get("entities") {
                Some(entities) => self.entities.insert(id, entities.clone()),
                None => self.entities.remove(&id),
            };
        }
        /// The pinned message as a read hands it back.
        fn the_pinned_message(&self, id: i64) -> Value {
            let mut pinned = json!({ "message_id": id, "from": { "is_bot": true }, "text": self.texts[&id] });
            if let Some(entities) = self.entities.get(&id).filter(|_| !self.reads_drop_the_marking) {
                pinned["entities"] = entities.clone();
            }
            pinned
        }
    }

    #[derive(Default)]
    struct Sim(Mutex<TheConversation>);

    impl Sim {
        fn state(&self) -> MutexGuard<'_, TheConversation> {
            lock(&self.0)
        }
        fn an_update(&self, body: Value) -> i64 {
            let mut s = self.state();
            s.next_update += 1;
            let id = s.next_update;
            let mut update = body;
            update["update_id"] = json!(id);
            s.updates.push(update);
            id
        }
        fn the_user_types(&self, chat: i64, text: &str) -> i64 {
            self.an_update(json!({ "message": { "message_id": 90_000, "chat": { "id": chat }, "text": text } }))
        }
        fn the_user_replies(&self, to: i64, text: &str) -> i64 {
            self.an_update(json!({ "message": { "message_id": 90_001, "chat": { "id": CHAT }, "text": text,
                "reply_to_message": { "message_id": to } } }))
        }
        /// A reply typed with `/`, dated, as a command picked from the list.
        fn the_user_replies_with(&self, to: i64, text: &str, from_a_person: Option<bool>) -> i64 {
            let mut reply_to = json!({ "message_id": to });
            if let Some(person) = from_a_person {
                reply_to["from"] = json!({ "is_bot": !person });
            }
            self.an_update(json!({ "message": { "message_id": 90_002, "chat": { "id": CHAT }, "text": text, "date": 1_730_100_000,
                "reply_to_message": reply_to } }))
        }
        fn the_user_taps(&self, chat: i64, message: i64, data: &str) -> i64 {
            let callback = {
                let mut s = self.state();
                s.next_callback += 1;
                format!("cb{}", s.next_callback)
            };
            self.an_update(json!({ "callback_query": { "id": callback, "data": data,
                "message": { "message_id": message, "chat": { "id": chat } } } }))
        }
        fn a_message_from_the_bot(&self, text: &str) -> i64 {
            let mut s = self.state();
            s.next_message += 1;
            let id = s.next_message;
            s.texts.insert(id, text.to_string());
            id
        }
        /// What the bot said to the user: every text that is not a board or a
        /// machinery message, in order.
        fn said_to_the_user(&self) -> Vec<(i64, String)> {
            let s = self.state();
            let mut said: Vec<(i64, String)> =
                s.texts.iter().filter(|(_, t)| !t.starts_with(THE_HEAD)).map(|(id, t)| (*id, t.clone())).collect();
            said.sort();
            said
        }
        fn the_board_on_top(&self) -> Option<TheBoard> {
            let s = self.state();
            let top = s.pins.iter().max()?;
            match what_is_pinned(&json!({ "pinned_message": { "message_id": top, "from": { "is_bot": true }, "text": s.texts[top] } })) {
                WhatIsPinned::ABoard { board, .. } => Some(board),
                _ => None,
            }
        }
    }

    struct ACaller {
        name: String,
        sim: Arc<Sim>,
        /// How long every call takes on the fleet's clock, as a real line does.
        latency: Option<(Arc<ItsClock>, Duration)>,
    }

    fn refused(code: i64, description: &str) -> Result<String, String> {
        Ok(json!({ "ok": false, "error_code": code, "description": description }).to_string())
    }
    fn accepted(result: Value) -> Result<String, String> {
        Ok(json!({ "ok": true, "result": result }).to_string())
    }

    /// **`Telegram` B24, enforced by the stand-in: fails the test where a
    /// machinery send or any pin would notify, or a message for the user would
    /// not.** Every pin, and every board or machinery message (B2, B10 — both
    /// open with B23's head, which is how the stand-in tells them apart),
    /// carries `disable_notification`; what is composed for the user carries
    /// none.
    fn the_silence_is_right(method: &str, fields: &Value) {
        let silent = fields.get("disable_notification") == Some(&json!(true));
        match method {
            "pinChatMessage" => assert!(silent, "a pin was sent without disable_notification: {fields}"),
            "sendMessage" if fields["text"].as_str().is_some_and(|t| t.starts_with(THE_HEAD)) => {
                assert!(silent, "a machinery message was sent with its notification: {fields}")
            }
            "sendMessage" => assert!(
                fields.get("disable_notification").is_none(),
                "a message for the user was sent silently: {fields}"
            ),
            _ => {}
        }
    }

    impl ReachesTelegram for ACaller {
        fn call(&self, method: &str, carries: &WhatACallCarries, _: Duration) -> Result<String, String> {
            let fields = match carries {
                WhatACallCarries::Fields(f) => Value::Object(f.clone()),
                _ => json!({}),
            };
            if let Some((clock, took)) = &self.latency {
                *lock(&clock.0) += *took;
            }
            let mut s = self.sim.state();
            s.heard.push((self.name.clone(), method.to_string(), fields.clone()));
            s.the_taps_are_answered_first(&self.name, method, &fields);
            let id = fields.get("message_id").and_then(Value::as_i64).unwrap_or(0);
            the_silence_is_right(method, &fields);
            // `Telegram` B25: every board and machinery send or edit carries its
            // spoiler after the head, and nothing for the user carries one.
            crate::board::text::tests::the_spoiler_is_right(method, &fields);
            match method {
                "sendMessage" => {
                    s.next_message += 1;
                    let id = s.next_message;
                    s.texts.insert(id, fields["text"].as_str().unwrap().to_string());
                    s.keep_the_marking(id, &fields);
                    if let Some(markup) = fields.get("reply_markup") {
                        s.buttons.insert(id, markup.clone());
                    }
                    accepted(json!({ "message_id": id, "date": 1_730_000_000 + id }))
                }
                "editMessageText" => {
                    if !s.texts.contains_key(&id) {
                        return refused(400, "Bad Request: message to edit not found");
                    }
                    s.texts.insert(id, fields["text"].as_str().unwrap().to_string());
                    s.keep_the_marking(id, &fields);
                    if let Some(markup) = fields.get("reply_markup") {
                        s.buttons.insert(id, markup.clone());
                    }
                    accepted(json!({ "message_id": id }))
                }
                "pinChatMessage" => {
                    if !s.texts.contains_key(&id) {
                        return refused(400, "Bad Request: message to pin not found");
                    }
                    if !s.pins.contains(&id) {
                        let pinned = s.the_pinned_message(id);
                        s.pins.push(id);
                        // B19: a line in the conversation only here.
                        s.lines += 1;
                        s.next_update += 1;
                        let update = s.next_update;
                        s.updates.push(json!({ "update_id": update, "message": { "message_id": 50_000 + update, "chat": { "id": CHAT },
                            "pinned_message": pinned } }));
                    }
                    accepted(json!(true))
                }
                "unpinChatMessage" => {
                    if s.pins.contains(&id) {
                        s.blind_for = s.blind_reads_after_an_unpin;
                    }
                    s.pins.retain(|p| *p != id);
                    accepted(json!(true))
                }
                "getChat" if s.blind_for > 0 => {
                    s.blind_for -= 1;
                    accepted(json!({ "id": CHAT }))
                }
                "getChat" => match s.pins.iter().max() {
                    Some(top) => accepted(json!({ "id": CHAT, "pinned_message": s.the_pinned_message(*top) })),
                    None => accepted(json!({ "id": CHAT })),
                },
                "getUpdates" => {
                    if s.refused_next.remove(&self.name) {
                        return refused(409, "Conflict: terminated by other getUpdates request; make sure that only one bot instance is running");
                    }
                    if let Some(open) = s.open_ask.clone().filter(|o| *o != self.name) {
                        s.refused_next.insert(open);
                    }
                    s.open_ask = Some(self.name.clone());
                    if let Some(offset) = fields.get("offset").and_then(Value::as_i64) {
                        s.confirmed_below = s.confirmed_below.max(offset);
                    }
                    let below = s.confirmed_below;
                    let waiting: Vec<Value> = s.updates.iter().filter(|u| u["update_id"].as_i64().unwrap() >= below).cloned().collect();
                    s.owe_the_taps(&self.name, &waiting);
                    accepted(Value::Array(waiting))
                }
                "getWebhookInfo" => accepted(json!({ "url": "" })),
                "getMyCommands" => accepted(if s.commands.is_null() { json!([]) } else { s.commands.clone() }),
                "setMyCommands" => {
                    s.commands = fields["commands"].clone();
                    accepted(json!(true))
                }
                "editMessageReplyMarkup" => {
                    s.buttons.insert(id, fields["reply_markup"].clone());
                    accepted(json!(true))
                }
                _ => accepted(json!(true)),
            }
        }
    }

    struct ItsClock(Mutex<Duration>);
    impl ReadsItsOwnClock for ItsClock {
        fn now(&self) -> Duration {
            *lock(&self.0)
        }
        fn the_moment(&self) -> String {
            format!("t+{}", lock(&self.0).as_secs())
        }
        /// The machines' clock moves on by the pause, as it would.
        fn pause(&self, how_long: Duration) {
            *lock(&self.0) += how_long;
        }
    }

    struct NoWaits;
    impl Waits for NoWaits {
        fn wait(&self, _: Duration) {}
    }

    #[derive(Clone, Default)]
    struct Jobs(Arc<Mutex<Vec<AJobAsHeld>>>);
    impl ReadsWhatThisMachineHolds for Jobs {
        fn the_jobs(&self) -> Vec<AJobAsHeld> {
            lock(&self.0).clone()
        }
    }

    type Acted = Arc<Mutex<Vec<(String, AnEntry)>>>;

    struct RecordsActs {
        machine: String,
        recorded: HashSet<String>,
        acted: Acted,
        jobs: Jobs,
    }
    impl ActsOnAnInstruction for RecordsActs {
        fn act_once(&mut self, entry: &AnEntry) -> Result<(), String> {
            if !self.recorded.insert(entry.operation.clone()) {
                return Ok(());
            }
            let jobs = self.jobs.clone();
            assert!(the_owner_finds_the_subject_live(entry, &|s| jobs.the_live_generation(s)));
            lock(&self.acted).push((self.machine.clone(), entry.clone()));
            Ok(())
        }
    }

    /// **The owning machine's real acts, with one check before a stop: B8 — the
    /// full stop instruction is on the board a read-back accepted.**
    struct ChecksTheBoardFirst {
        sim: Arc<Sim>,
        inner: TheOwnersActs,
    }
    impl ActsOnAnInstruction for ChecksTheBoardFirst {
        fn act_once(&mut self, entry: &AnEntry) -> Result<(), String> {
            if entry.action == STOP_THIS_JOB {
                let board = self.sim.the_board_on_top().expect("a board is pinned");
                assert!(
                    board.instructions.iter().any(|e| e.operation == entry.operation && e.kind == AKindOfEntry::Action),
                    "B8: acted on before the board carried the full instruction"
                );
            }
            self.inner.act_once(entry)
        }
    }

    type Shared = Arc<Mutex<Box<dyn TheTelegramWorkersState>>>;

    struct AMember {
        name: String,
        listener: TheListener,
        router: TheRouter,
        store: Shared,
        jobs: Jobs,
        alive: bool,
        /// D2's first half, where this member owns jobs for real.
        acknowledgement: Option<TheListenersAcknowledgement>,
    }

    struct AFleet {
        sim: Arc<Sim>,
        clock: Arc<ItsClock>,
        members: Vec<AMember>,
        acted: Acted,
        latency: Option<Duration>,
    }

    fn the_machine(name: &str) -> String {
        format!("machine-{name}")
    }

    fn a_rendering_job(name: &str, generation: &str) -> AJobAsHeld {
        AJobAsHeld {
            job: format!("job-{name}"),
            label: format!("Job {}", name.to_uppercase()),
            generation: generation.into(),
            record_directory: true,
            rendering: Some(TheProgress { frame: 12, of: 40, percent: 30 }),
            waiting: None,
        }
    }

    impl AFleet {
        fn new() -> AFleet {
            AFleet { sim: Arc::default(), clock: Arc::new(ItsClock(Mutex::new(Duration::ZERO))), members: Vec::new(), acted: Arc::default(), latency: None }
        }

        fn start(&mut self, name: &str) {
            self.start_with(name, None);
        }

        /// A member whose acts are [`TheOwnersActs`] over `owner`, where given.
        fn start_with(&mut self, name: &str, owner: Option<AStandInSupervisor>) {
            let index = self.members.len() + 1;
            let latency = self.latency.map(|took| (self.clock.clone(), took));
            let client = TheClient::over(Arc::new(ACaller { name: name.into(), sim: self.sim.clone(), latency }));
            let queue = Arc::new(TheSendQueue::new(client.clone(), Arc::new(NoWaits)));
            let store: Shared = Arc::new(Mutex::new(Box::new(InMemory::with_routing_key(&format!("{index:032x}")))));
            let jobs = Jobs::default();
            let me = WhoThisMachineIs { machine: the_machine(name), display: format!("Server {}", name.to_uppercase()).into() };
            let router = TheRouter::new(WhatTheRouterIsHanded {
                conversation: CHAT,
                me: me.clone(),
                store: Box::new(store.clone()),
                holds: Box::new(jobs.clone()),
                queue: queue.clone(),
                clock: self.clock.clone(),
                notes: Box::new(|_| {}),
            });
            let (actor, acknowledgement): (Box<dyn ActsOnAnInstruction>, _) = match owner {
                None => (Box::new(RecordsActs { machine: name.into(), recorded: HashSet::new(), acted: self.acted.clone(), jobs: jobs.clone() }), None),
                Some(supervisor) => (
                    Box::new(ChecksTheBoardFirst {
                        sim: self.sim.clone(),
                        inner: TheOwnersActs {
                            conversation: CHAT,
                            supervisor: Box::new(supervisor.with_context_store(store.clone())),
                            words: Box::new(supervisor.clone()),
                            holds: Box::new(jobs.clone()),
                            queue: queue.clone(),
                            notes: Box::new(|_| {}),
                        },
                    }),
                    Some(TheListenersAcknowledgement { conversation: CHAT, queue: queue.clone(), said: HashSet::new() }),
                ),
            };
            let listener = TheListener::new(WhatTheListenerIsHanded {
                client,
                queue,
                clock: self.clock.clone(),
                me,
                settings: TheSettings::for_the_conversation(CHAT),
                store: Box::new(store.clone()),
                actor,
                hands_on: Box::new(router.hands()),
                notes: Box::new(|_| {}),
            });
            self.members.push(AMember { name: name.into(), listener, router, store, jobs, alive: true, acknowledgement });
        }

        fn member(&mut self, name: &str) -> &mut AMember {
            self.members.iter_mut().find(|m| m.name == name).unwrap()
        }

        fn a_round(&mut self) {
            for member in self.members.iter_mut().filter(|m| m.alive) {
                member.router.before_each_turn(&mut member.listener);
                if let Some(acknowledgement) = member.acknowledgement.as_mut() {
                    acknowledgement.before_each_turn(&member.listener);
                }
                member.listener.a_turn();
                if let Some(ask) = member.listener.the_ask_to_make() {
                    let got = member.listener.client().get_updates(ask.offset, ask.wait_seconds);
                    member.listener.what_the_ask_brought(ask, got);
                }
                // R16: nothing this member was handed is left unanswered.
                self.sim.state().nothing_is_owed_by(&member.name);
            }
            *lock(&self.clock.0) += THE_READ_CADENCE;
        }

        fn rounds_until(&mut self, most: usize, done: impl Fn(&AFleet) -> bool) -> Option<usize> {
            for round in 0..most {
                if done(self) {
                    return Some(round);
                }
                self.a_round();
            }
            done(self).then_some(most)
        }

        fn holder(&self) -> Option<String> {
            let holding: Vec<&AMember> = self.members.iter().filter(|m| m.alive && m.listener.is_holding()).collect();
            (holding.len() == 1).then(|| holding[0].name.clone())
        }

        fn settled(names: &[&str]) -> (AFleet, String) {
            AFleet::settled_with(names, None)
        }

        fn settled_with(names: &[&str], latency: Option<Duration>) -> (AFleet, String) {
            AFleet::settled_with_blind_reads(names, latency, 0, false)
        }

        /// Settled on a stand-in where every read for `blind` reads after an
        /// unpin reports nothing pinned — the second live watch's likeliest
        /// cause of its bursts, from before the first machine starts; and where
        /// `reads_drop_the_marking` is set, one whose reads hand back no
        /// spoiler (B25).
        fn settled_with_blind_reads(
            names: &[&str],
            latency: Option<Duration>,
            blind: usize,
            reads_drop_the_marking: bool,
        ) -> (AFleet, String) {
            let mut fleet = AFleet::new();
            fleet.sim.state().blind_reads_after_an_unpin = blind;
            fleet.sim.state().reads_drop_the_marking = reads_drop_the_marking;
            fleet.latency = latency;
            for name in names {
                fleet.start(name);
            }
            // **Since 2026-10-18 the machines that do not hold announce only
            // once they have seen the holder's first renewal** (B14), about
            // fifty seconds after it claims: the bound was 80 rounds.
            fleet
                .rounds_until(120, |f| f.holder().is_some() && f.sim.the_board_on_top().is_some_and(|b| b.roster.len() == f.members.len()))
                .expect("one machine holds and every machine is on the roster");
            let holder = fleet.holder().unwrap();
            (fleet, holder)
        }

        fn said(&self, text: &str) -> usize {
            self.sim.said_to_the_user().iter().filter(|(_, t)| t.as_str() == text).count()
        }
    }

    /// **The first of the milestone's three at the desk: *how is it going* is
    /// one message from three machines, and the one killed first is named as
    /// not answering** (`Telegram` R10, R12, B12). **R11**: nothing is gathered
    /// before anyone asks. **F3**: the request is confirmed only after its
    /// entries are on the board.
    #[test]
    fn how_is_it_going_is_one_message_naming_the_machine_that_did_not_answer() {
        how_is_it_going_with(None);
    }

    /// **The same, with every call taking a third of a second on the machines'
    /// clock**, the order of a real line. Written after the first live run,
    /// where only the holder's own reply arrived within R12's ten seconds.
    #[test]
    fn how_is_it_going_with_a_real_lines_latency() {
        how_is_it_going_with(Some(Duration::from_millis(300)));
    }

    fn how_is_it_going_with(latency: Option<Duration>) {
        let (mut fleet, holder) = AFleet::settled_with(&["a", "b", "c", "d"], latency);
        for name in ["a", "b", "c"].into_iter().filter(|n| *n != holder) {
            *lock(&fleet.member(name).jobs.0) = vec![a_rendering_job(name, "1")];
        }
        fleet.member("d").alive = false;
        for _ in 0..20 {
            fleet.a_round();
        }
        assert!(fleet.sim.said_to_the_user().is_empty(), "R11: nothing is said before anyone asks");

        let asked = fleet.sim.the_user_types(CHAT, "Status?");
        fleet.a_round();
        assert!(fleet.sim.state().confirmed_below <= asked, "F3: not confirmed before its entries are on the board");
        fleet
            .rounds_until(30, |f| f.sim.said_to_the_user().iter().any(|(_, t)| t.contains("Did not answer.")))
            .expect("one message came back");
        for _ in 0..10 {
            fleet.a_round();
        }
        let said = fleet.sim.said_to_the_user();
        assert_eq!(said.len(), 1, "one message: {said:?}");
        let lines: Vec<&str> = said[0].1.lines().collect();
        assert_eq!(lines.len(), 8);
        assert!(said[0].1.contains("Server D\nDid not answer."));
        assert!(said[0].1.contains(&format!("Server {}\nIdle.", holder.to_uppercase())));
        for name in ["a", "b", "c"].into_iter().filter(|n| *n != holder) {
            let upper = name.to_uppercase();
            assert!(said[0].1.contains(&format!("Server {upper}\nJob {upper} · frame 12 of 40 · 30%")), "{lines:?}");
        }
        assert!(fleet.sim.state().confirmed_below > asked, "confirmed once its work was done");
        assert!(fleet.sim.the_board_on_top().unwrap().instructions.is_empty(), "B12: the state-requests left at their deadline");
    }

    /// **The second and third: a tap on a button resolves across the board to
    /// the machine standing as the job's owner, and a tap whose issuing machine
    /// is gone is answered with *nothing was done*** (`Telegram` R1, R2, R4,
    /// R13, R14, B15). The records exist before the buttons do.
    #[test]
    fn a_tap_reaches_its_owner_and_a_tap_on_a_gone_issuer_says_nothing_was_done() {
        let (mut fleet, holder) = AFleet::settled(&["a", "b", "c"]);
        let others: Vec<String> = ["a", "b", "c"].into_iter().filter(|n| *n != holder).map(str::to_string).collect();
        for name in &others {
            *lock(&fleet.member(name).jobs.0) = vec![a_rendering_job(name, "1")];
        }
        fleet.sim.the_user_types(CHAT, "Frame 120");
        fleet
            .rounds_until(30, |f| f.sim.state().buttons.values().any(|b| b["inline_keyboard"].as_array().is_some_and(|r| r.len() == 2)))
            .expect("the choice of job was offered");
        let (question, buttons) = {
            let s = fleet.sim.state();
            let (id, markup) = s.buttons.iter().next().map(|(id, m)| (*id, m.clone())).unwrap();
            assert_eq!(s.texts[&id], WHICH_JOB);
            // Sent without buttons, the records written, and only then the buttons.
            let sent = s.heard.iter().position(|(_, m, f)| m == "sendMessage" && f["text"] == json!(WHICH_JOB)).unwrap();
            assert!(s.heard[sent].2.get("reply_markup").is_none());
            (id, markup["inline_keyboard"].as_array().unwrap().clone())
        };
        let records: Vec<AnActionContextRecord> = fleet
            .member(&holder)
            .store
            .the_action_contexts_bound_to(question)
            .unwrap();
        assert_eq!(records.len(), 4);
        assert_eq!(
            records
                .iter()
                .filter(|record| record.action == crate::job_messages::ASK_TO_STOP_THIS_JOB)
                .count(),
            2
        );
        let data: Vec<String> = buttons
            .iter()
            .map(|row| row[0]["callback_data"].as_str().unwrap().to_string())
            .collect();
        assert!(data.iter().all(|d| d.len() == 44));
        let first_owner = records.iter().find(|r| r.token == data[0]).unwrap().owning_machine.clone();
        let second_owner = records.iter().find(|r| r.token == data[1]).unwrap().owning_machine.clone();
        assert_ne!(first_owner, second_owner);

        fleet.sim.the_user_taps(CHAT, question, &data[0]);
        fleet
            .rounds_until(30, |f| !lock(&f.acted).is_empty() && f.sim.the_board_on_top().is_some_and(|b| b.instructions.is_empty()))
            .expect("the owner acted and the holder trimmed it");
        {
            let acted = lock(&fleet.acted);
            assert_eq!(acted.len(), 1);
            assert_eq!(the_machine(&acted[0].0), first_owner, "resolved to the owner, not the holder");
            assert_eq!(acted[0].1.action, SENDING_A_FRAME);
            assert_eq!(acted[0].1.payload["frame"], json!(120));
        }

        // The issuing machine is gone for good; the other button now has nobody
        // to resolve it.
        fleet.member(&holder).alive = false;
        fleet.sim.the_user_taps(CHAT, question, &data[1]);
        fleet.rounds_until(90, |f| f.said(A_BUTTON_THAT_EXPIRED) == 1).expect("the press was answered");
        for _ in 0..15 {
            fleet.a_round();
        }
        assert_eq!(fleet.said(A_BUTTON_THAT_EXPIRED), 1, "answered once");
        assert_eq!(lock(&fleet.acted).len(), 1, "nothing was done");
        assert!(the_machine(&holder) != second_owner);
    }

    /// **The eighteenth notes, note 16 (`Telegram` C1b as narrowed
    /// 2026-10-24): *Which job?* offers Stop only beside a job still live.**
    /// A job that has ended is still a frame's candidate (R13) and keeps its
    /// own button, with no Stop beside it and no Stop record written. Before
    /// 2026-10-25 every candidate had *Stop …* beside it.
    #[test]
    fn eighteenth_6_which_job_offers_stop_only_beside_a_live_job() {
        let (mut fleet, holder) = AFleet::settled(&["a", "b", "c"]);
        let others: Vec<String> = ["a", "b", "c"].into_iter().filter(|n| *n != holder).map(str::to_string).collect();
        *lock(&fleet.member(&others[0]).jobs.0) = vec![a_rendering_job(&others[0], "1")];
        *lock(&fleet.member(&others[1]).jobs.0) =
            vec![AJobAsHeld { rendering: None, waiting: None, ..a_rendering_job(&others[1], "1") }];
        fleet.sim.the_user_types(CHAT, "Frame 120");
        fleet
            .rounds_until(30, |f| f.sim.state().buttons.values().any(|b| b["inline_keyboard"].as_array().is_some_and(|r| r.len() == 2)))
            .expect("the choice of job was offered");
        let (question, rows) = {
            let s = fleet.sim.state();
            let (id, markup) = s.buttons.iter().next().map(|(id, m)| (*id, m.clone())).unwrap();
            assert_eq!(s.texts[&id], WHICH_JOB);
            (id, markup["inline_keyboard"].as_array().unwrap().clone())
        };
        let labels = |row: &Value| -> Vec<String> {
            row.as_array().unwrap().iter().map(|b| b["text"].as_str().unwrap().to_string()).collect()
        };
        let live = format!("Job {}", others[0].to_uppercase());
        let ended = format!("Job {}", others[1].to_uppercase());
        let rows: Vec<Vec<String>> = rows.iter().map(labels).collect();
        assert!(rows.contains(&vec![live.clone(), format!("Stop {live}")]), "{rows:?}");
        assert!(rows.contains(&vec![ended.clone()]), "the ended job has no Stop: {rows:?}");
        let records = fleet.member(&holder).store.the_action_contexts_bound_to(question).unwrap();
        assert_eq!(
            records.iter().filter(|record| record.action == crate::job_messages::ASK_TO_STOP_THIS_JOB).count(),
            1,
            "one Stop record, for the live job alone"
        );
    }

    /// **`Telegram` R3: a button that cannot be read and a message that is
    /// neither request are answered at once, and nothing is boarded.**
    #[test]
    fn what_cannot_be_resolved_is_answered_and_not_boarded() {
        let (mut fleet, _) = AFleet::settled(&["a"]);
        let message = fleet.sim.a_message_from_the_bot("an old message");
        fleet.sim.the_user_taps(CHAT, message, "not a token of ours");
        fleet.sim.the_user_types(CHAT, "hello");
        fleet.rounds_until(10, |f| f.said(A_BUTTON_THAT_EXPIRED) == 1 && f.said(NOT_A_REQUEST) == 1).expect("both answered");
        for _ in 0..10 {
            fleet.a_round();
        }
        assert_eq!(fleet.said(A_BUTTON_THAT_EXPIRED) + fleet.said(NOT_A_REQUEST), 2, "each answered once");
        let s = fleet.sim.state();
        assert!(s.heard.iter().all(|(_, m, f)| m != "editMessageText" || !f["text"].as_str().unwrap().contains("tap-")));
    }

    /// The rented pass only pressed Start. Pairing and service updates are
    /// not typed requests, and must not produce the unknown-request reply.
    #[test]
    fn rented_pass_start_and_service_updates_get_no_command_reply() {
        let (mut fleet, _) = AFleet::settled(&["a"]);
        for text in ["/start", "/start setup", "/start@example_bot", "/start@example_bot setup"] {
            fleet.sim.the_user_types(CHAT, text);
        }
        fleet.sim.an_update(json!({ "message": { "message_id": 90_010,
            "chat": { "id": CHAT }, "from": { "is_bot": true }, "text": "status" } }));
        fleet.sim.an_update(json!({ "message": { "message_id": 90_011,
            "chat": { "id": CHAT }, "from": { "is_bot": true }, "text": "coordination" } }));
        fleet.sim.an_update(json!({ "message": { "message_id": 90_012,
            "chat": { "id": CHAT }, "pinned_message": { "message_id": 12, "text": "a pin" } } }));
        for _ in 0..12 { fleet.a_round(); }
        assert!(fleet.sim.said_to_the_user().is_empty(), "pairing is silent: {:?}", fleet.sim.said_to_the_user());
        assert!(fleet.sim.state().confirmed_below > 0, "ignored updates are confirmed");
        fleet.sim.the_user_types(CHAT, "hello");
        for _ in 0..8 { fleet.a_round(); }
        assert_eq!(fleet.said(NOT_A_REQUEST), 1, "unknown human text still gets help");
    }

    /// **`Telegram` P2, watched against the stand-in: an update from any other
    /// conversation is ignored** — not answered, not routed, and confirmed.
    #[test]
    fn nothing_from_another_conversation_is_answered_or_routed() {
        let (mut fleet, _) = AFleet::settled(&["a", "b"]);
        let message = fleet.sim.a_message_from_the_bot("a message");
        fleet.sim.the_user_types(999, "status");
        fleet.sim.the_user_types(999, "hello");
        let last = fleet.sim.the_user_taps(999, message, "not a token of ours");
        for _ in 0..20 {
            fleet.a_round();
        }
        assert!(fleet.sim.said_to_the_user().iter().all(|(id, _)| *id == message), "nothing was said");
        assert!(fleet.sim.state().confirmed_below > last, "and each was confirmed as dismissed");
        assert!(fleet.sim.the_board_on_top().unwrap().instructions.is_empty());
    }

    /// **`Telegram` R1a: a reply resolves by the replied-to message on the
    /// machine holding its record, and to the typed request; a reply to a
    /// message nobody holds a record for falls back, at its deadline, to the
    /// typed request's own handling — here R13's one candidate, routed
    /// directly.** And with no job at all, the user is told.
    #[test]
    fn a_reply_resolves_by_its_message_and_falls_back_when_nobody_can() {
        let (mut fleet, holder) = AFleet::settled(&["a", "b"]);
        let owner = if holder == "a" { "b" } else { "a" };
        *lock(&fleet.member(owner).jobs.0) = vec![a_rendering_job(owner, "1")];
        let picture = fleet.sim.a_message_from_the_bot("a frame of job");
        let key = fleet.member(owner).store.the_routing_key().unwrap();
        let mut record = AnActionContextRecord::issued_under(&key, CHAT, &the_machine(owner), Some(&format!("job-{owner}")), None, "stop", json!({}), "1").unwrap();
        record.source_message = Some(picture);
        fleet.member(owner).store.record_an_action_context(record).unwrap();

        fleet.sim.the_user_replies(picture, "Frame 5");
        fleet.rounds_until(30, |f| lock(&f.acted).len() == 1).expect("the reply reached the owner");
        assert_eq!(lock(&fleet.acted)[0].1.payload["frame"], json!(5));
        assert_eq!(lock(&fleet.acted)[0].1.action, SENDING_A_FRAME, "the typed request, never the record's stored action");

        let unrelated = fleet.sim.a_message_from_the_bot("a message with no record");
        fleet.sim.the_user_replies(unrelated, "Frame 6");
        fleet.rounds_until(40, |f| lock(&f.acted).len() == 2).expect("the fallback routed the one candidate directly");
        assert_eq!(lock(&fleet.acted)[1].1.payload["frame"], json!(6));
        assert_eq!(the_machine(&lock(&fleet.acted)[1].0), the_machine(owner));

        lock(&fleet.member(owner).jobs.0).clear();
        fleet.sim.the_user_types(CHAT, "Frame 7");
        fleet.rounds_until(30, |f| f.said(NO_JOB_TO_SEND_A_FRAME_FROM) == 1).expect("told there is no job");
    }

    /// The pin calls one machine made.
    fn pins_by(fleet: &AFleet, name: &str) -> usize {
        fleet.sim.state().heard.iter().filter(|(n, m, _)| n == name && m == "pinChatMessage").count()
    }

    /// **The fourteenth notes, note 2, D1's other half: a lone server under a
    /// dead server's board answers *how is it going* with its own job**,
    /// having pinned once — its own board, which lists it (B3: the claimant
    /// lists itself; R10: the holder's own reply is never pinned). The dead
    /// board's roster is empty, as the polite stop leaves it.
    #[test]
    fn a_lone_server_under_a_dead_board_pins_once_and_answers_with_its_own_job() {
        let mut fleet = AFleet::new();
        let mut dead = crate::board::text::tests::a_board("machine-gone");
        dead.roster.clear();
        let id = fleet.sim.a_message_from_the_bot(&dead.compose().unwrap());
        fleet.sim.state().pins.push(id);
        fleet.start("a");
        *lock(&fleet.member("a").jobs.0) = vec![a_rendering_job("a", "1")];
        fleet.rounds_until(60, |f| f.holder().as_deref() == Some("a")).expect("it takes the slot");
        fleet.sim.the_user_types(CHAT, "status");
        fleet.rounds_until(30, |f| !f.sim.said_to_the_user().is_empty()).expect("answered");
        for _ in 0..90 {
            fleet.a_round();
        }
        let said = fleet.sim.said_to_the_user();
        assert_eq!(said.len(), 1, "{said:?}");
        assert_eq!(said[0].1, "Server A\nJob A · frame 12 of 40 · 30%");
        assert_eq!(pins_by(&fleet, "a"), 1, "one pin, its own board's");
    }

    /// **The fourteenth notes, note 2, D6's routing half (B14's stated cost):
    /// until a newcomer's announcement is admitted, a gather leaves it out and
    /// so does a plain *send a frame*'s choice of jobs, while a tap on one of
    /// its own messages reaches it and is acted on. Fresh requests after
    /// admission include it.** It starts just after the holder renewed, so its
    /// wait is the whole interval.
    #[test]
    fn a_newcomer_is_left_out_of_gathers_until_listed_and_a_tap_reaches_it_before() {
        let (mut fleet, holder) = AFleet::settled(&["a"]);
        let board_id = fleet.member(&holder).store.the_board().unwrap().unwrap();
        let heartbeat = |f: &AFleet| {
            let s = f.sim.state();
            match what_is_pinned(&json!({ "pinned_message": s.the_pinned_message(board_id) })) {
                WhatIsPinned::ABoard { board, .. } => board.heartbeat,
                _ => unreachable!("the holder's board reads"),
            }
        };
        let before = heartbeat(&fleet);
        fleet.rounds_until(40, |f| heartbeat(f) != before).expect("the holder renewed");
        fleet.start("b");
        *lock(&fleet.member("b").jobs.0) = vec![a_rendering_job("b", "1")];
        let listed = |f: &AFleet| f.sim.the_board_on_top().is_some_and(|b| b.roster.iter().any(|r| r.machine == the_machine("b")));

        // A tap on one of its messages, before it announces.
        let picture = fleet.sim.a_message_from_the_bot("a frame of job b");
        let key = fleet.member("b").store.the_routing_key().unwrap();
        let mut record =
            AnActionContextRecord::issued_under(&key, CHAT, &the_machine("b"), Some("job-b"), None, "stop", json!({}), "1").unwrap();
        record.source_message = Some(picture);
        let token = record.token.clone();
        fleet.member("b").store.record_an_action_context(record).unwrap();
        fleet.sim.the_user_taps(CHAT, picture, &token);
        fleet.rounds_until(15, |f| lock(&f.acted).len() == 1).expect("the tap reached it");
        assert_eq!(lock(&fleet.acted)[0].0, "b");
        assert!(!listed(&fleet), "and it has not announced");

        // A gather and a plain *send a frame* in the same window leave it out.
        fleet.sim.the_user_types(CHAT, "status");
        fleet.rounds_until(10, |f| f.said("Server A\nIdle.") == 1).expect("the gather was answered");
        fleet.sim.the_user_types(CHAT, "Frame 7");
        fleet.rounds_until(10, |f| f.said(NO_JOB_TO_SEND_A_FRAME_FROM) == 1).expect("no job to choose");
        assert!(!listed(&fleet), "still not listed: {:?}", fleet.sim.said_to_the_user());
        assert_eq!(lock(&fleet.acted).len(), 1);

        // Once listed, fresh requests include it.
        fleet.rounds_until(40, listed).expect("listed after the renewal it saw");
        fleet.sim.the_user_types(CHAT, "status");
        fleet
            .rounds_until(10, |f| f.sim.said_to_the_user().iter().any(|(_, t)| t.contains("Server B\nJob B")))
            .expect("the gather names it");
        fleet.sim.the_user_types(CHAT, "Frame 7");
        fleet.rounds_until(15, |f| lock(&f.acted).len() == 2).expect("frame reached it");
        let acted = lock(&fleet.acted);
        assert_eq!(acted[1].0, "b");
        assert_eq!(acted[1].1.action, SENDING_A_FRAME);
    }

    /// Pin calls accepted, and pinned-message lines written, so far.
    fn the_pins(fleet: &AFleet) -> (usize, usize) {
        let s = fleet.sim.state();
        (s.heard.iter().filter(|(_, m, _)| m == "pinChatMessage").count(), s.lines)
    }

    /// **The live watch's own sequence, with its pins counted** — four
    /// servers start and settle, one that is not holding is killed, *status*,
    /// *frame 1* and a tap, then the holder killed and a tap on the
    /// other button. Written after the first live run, where only the holder's
    /// reply reached the gather and 98 pin calls were made against the plan's
    /// worked pricing of about fifteen to twenty (`Telegram` B20).
    #[test]
    fn the_live_watchs_sequence_and_what_its_pins_cost() {
        the_live_watchs_sequence_with_blind_reads(0, false);
    }

    /// **`Telegram` B25: no reader relies on the spoiler.** The live watch's
    /// sequence on a conversation whose pin updates and `getChat` hand back no
    /// entity list says the same to the user and costs the same pins as one
    /// whose reads carry it.
    #[test]
    fn the_live_watchs_sequence_is_the_same_when_reads_hand_back_no_spoiler() {
        let kept = the_live_watchs_sequence_with_blind_reads(0, false);
        let dropped = the_live_watchs_sequence_with_blind_reads(0, true);
        assert_eq!(kept, dropped);
    }

    /// **The same sequence, on a stand-in where the six reads after every unpin
    /// report nothing pinned** — the likeliest cause of the second live watch's
    /// 33 extra pins of the holder's own board, in bursts of six (2026-09-14).
    /// The cap is the same, and so is the rule that every pin call writes a
    /// line: a holder that pinned its still-pinned board on such a read fails
    /// both.
    #[test]
    fn the_live_watchs_sequence_costs_the_same_when_reads_lag_after_an_unpin() {
        the_live_watchs_sequence_with_blind_reads(6, false);
    }

    /// Runs the sequence and gives back the pins it cost at the end and every
    /// message said to the user, in order.
    fn the_live_watchs_sequence_with_blind_reads(blind: usize, reads_drop_the_marking: bool) -> ((usize, usize), Vec<String>) {
        let (mut fleet, holder) = AFleet::settled_with_blind_reads(&["a", "b", "c", "d"], None, blind, reads_drop_the_marking);
        // A few reads for the last re-announcements to a new holder (B14).
        for _ in 0..10 {
            fleet.a_round();
        }
        let at_rest = the_pins(&fleet);
        let settled = at_rest;
        // The Creator's own minutes before they type: B20's *at rest, zero new
        // lines*, which a re-pin cadence would break.
        for _ in 0..150 {
            fleet.a_round();
        }
        assert_eq!(the_pins(&fleet), at_rest, "five minutes at rest pinned nothing");
        let killed = ["d", "c", "b", "a"].into_iter().find(|n| *n != holder).unwrap().to_string();
        fleet.member(&killed).alive = false;
        let owners: Vec<String> =
            ["a", "b", "c", "d"].into_iter().filter(|n| *n != holder && *n != killed).map(str::to_string).collect();
        for name in &owners {
            *lock(&fleet.member(name).jobs.0) = vec![a_rendering_job(name, "1")];
        }

        // 1. status
        fleet.sim.the_user_types(CHAT, "status");
        fleet
            .rounds_until(30, |f| f.sim.said_to_the_user().iter().any(|(_, t)| t.contains("Did not answer.")))
            .expect("one message came back");
        for _ in 0..10 {
            fleet.a_round();
        }
        let said = fleet.sim.said_to_the_user();
        assert_eq!(said.len(), 1, "one message: {said:?}");
        let lines: Vec<&str> = said[0].1.lines().collect();
        assert_eq!(lines.len(), 8, "{lines:?}");
        assert!(said[0].1.contains(&format!("Server {}\nDid not answer.", killed.to_uppercase())), "{lines:?}");
        assert!(said[0].1.contains(&format!("Server {}\nIdle.", holder.to_uppercase())), "{lines:?}");
        for name in &owners {
            let upper = name.to_uppercase();
            assert!(said[0].1.contains(&format!("Server {upper}\nJob {upper} · frame 12 of 40 · 30%")), "every live server answered: {lines:?}");
        }
        let step_one = the_pins(&fleet);

        // 2. frame 1, and a tap resolved to its owner
        fleet.sim.the_user_types(CHAT, "Frame 1");
        fleet
            .rounds_until(30, |f| f.sim.state().buttons.values().any(|b| b["inline_keyboard"].as_array().is_some_and(|r| r.len() == 2)))
            .expect("the choice of job was offered");
        let (question, data) = {
            let s = fleet.sim.state();
            let (id, markup) = s.buttons.iter().next().map(|(id, m)| (*id, m.clone())).unwrap();
            let data: Vec<String> = markup["inline_keyboard"].as_array().unwrap().iter().map(|row| row[0]["callback_data"].as_str().unwrap().to_string()).collect();
            (id, data)
        };
        fleet.sim.the_user_taps(CHAT, question, &data[0]);
        fleet
            .rounds_until(30, |f| !lock(&f.acted).is_empty() && f.sim.the_board_on_top().is_some_and(|b| b.instructions.is_empty()))
            .expect("the owner acted and the holder trimmed it");
        let step_two = the_pins(&fleet);

        // 3. the issuer gone, and a tap on its other button
        fleet.member(&holder).alive = false;
        fleet.sim.the_user_taps(CHAT, question, &data[1]);
        fleet.rounds_until(120, |f| f.said(A_BUTTON_THAT_EXPIRED) == 1).expect("the press was answered");
        for _ in 0..15 {
            fleet.a_round();
        }
        assert_eq!(fleet.said(A_BUTTON_THAT_EXPIRED), 1, "answered once");
        assert_eq!(lock(&fleet.acted).len(), 1, "nothing was done for the second tap");
        let step_three = the_pins(&fleet);

        eprintln!(
            "pin calls and lines: settled {settled:?}; after status {step_one:?}; after the tap {step_two:?}; after the expired tap {step_three:?}"
        );
        // B20's worked pricing for this sequence is about fifteen to twenty
        // lines; the first live run made 98 pin calls.
        assert!(step_one.0 <= 20 && step_three.0 <= 20, "pin calls: {step_one:?} {step_three:?}");
        assert_eq!(step_three.0, step_three.1, "every pin call wrote a line: nothing was pinned while still pinned");
        // B25: the pin updates carried the spoiler exactly where the stand-in
        // was set to keep it.
        let carried = fleet.sim.state().updates.iter().filter(|u| u.pointer("/message/pinned_message/entities").is_some()).count();
        assert_eq!(carried > 0, !reads_drop_the_marking, "{carried} pin updates carried the spoiler");
        (step_three, fleet.sim.said_to_the_user().into_iter().map(|(_, text)| text).collect())
    }

    /// **F3, B2 and fix-here item 1 — a tap the full board cannot admit is
    /// answered truthfully and its update confirmed, and every later update is
    /// still heard.** Before, the routing kept the refused proposal pending for
    /// ever and the listener never moved past that update.
    #[test]
    fn a_tap_the_full_board_cannot_admit_is_refused_and_nothing_after_it_waits() {
        let (mut fleet, holder) = AFleet::settled(&["a"]);
        // One instruction to a machine that is gone fills the board to within a
        // hundred units of Telegram's bound.
        let board = fleet.sim.the_board_on_top().unwrap();
        let mut filler = crate::listener::an_action_for("gone", "filler", json!("")).unwrap();
        let mut fits = |size: usize| {
            filler.payload = json!("x".repeat(size));
            matches!(admit(&board, &filler, "t+99999"), AnAdmission::Accepted(_))
        };
        let mut size = 0;
        while fits(size + 100) {
            size += 100;
        }
        assert!(fits(size) && size > 0);
        let filler_operation = filler.operation.clone();
        fleet.member(&holder).listener.propose(filler);
        fleet
            .rounds_until(5, |f| f.sim.the_board_on_top().is_some_and(|b| b.instructions.iter().any(|e| e.operation == filler_operation)))
            .expect("the filler was admitted");

        let message = fleet.sim.a_message_from_the_bot("a job message");
        let key = fleet.member(&holder).store.the_routing_key().unwrap();
        let mut record =
            AnActionContextRecord::issued_under(&key, CHAT, &the_machine(&holder), Some("job-x"), None, STOP_THIS_JOB, json!({}), "1").unwrap();
        record.source_message = Some(message);
        let token = record.token.clone();
        fleet.member(&holder).store.record_an_action_context(record).unwrap();
        let tapped = fleet.sim.the_user_taps(CHAT, message, &token);
        fleet
            .rounds_until(10, |f| f.sim.said_to_the_user().iter().any(|(_, t)| t.contains("the pinned message is full")))
            .expect("the refusal was said");
        fleet.rounds_until(10, |f| f.sim.state().confirmed_below > tapped).expect("F3: the refused tap's update was confirmed");
        let typed = fleet.sim.the_user_types(CHAT, "hello");
        fleet.rounds_until(10, |f| f.said(NOT_A_REQUEST) == 1).expect("a later update is still heard");
        fleet.rounds_until(10, |f| f.sim.state().confirmed_below > typed).expect("and confirmed");
        assert!(fleet.sim.said_to_the_user().iter().all(|(_, t)| !t.contains("resolve a button press")), "Rule 9: no internal name");
    }

    /// **D2, B8 and fix-here item 3 — on one machine, which both listens and
    /// owns the job, a tapped stop is said sent, and is submitted only once a
    /// read-back has accepted the full instruction.** Before, the holder acted
    /// before writing, and *Stop sent* was never said.
    #[test]
    fn on_one_machine_a_tapped_stop_is_said_sent_and_submitted_only_once_the_board_holds_it() {
        let mut fleet = AFleet::new();
        let supervisor = AStandInSupervisor::default();
        fleet.start_with("a", Some(supervisor.clone()));
        fleet.rounds_until(60, |f| f.holder().is_some()).expect("it holds");
        *lock(&fleet.member("a").jobs.0) = vec![a_rendering_job("a", "1")];
        let message = fleet.sim.a_message_from_the_bot("a picture of a frame");
        let client = TheClient::over(Arc::new(ACaller { name: "a".into(), sim: fleet.sim.clone(), latency: None }));
        let mut messages = TheJobMessages {
            conversation: CHAT,
            store: Box::new(fleet.member("a").store.clone()),
            queue: Arc::new(TheSendQueue::new(client, Arc::new(NoWaits))),
            notes: Box::new(|_| {}),
        };
        let subject = TheSubject {
            owning_machine: the_machine("a"),
            job: Some("job-a".into()),
            hold: None,
            generation: "1".into(),
        };
        let button = messages
            .a_stop_button(message, &subject, "Server A", "Job A")
            .unwrap();
        fleet.sim.the_user_taps(CHAT, message, button["callback_data"].as_str().unwrap());
        fleet.rounds_until(20, |f| f.sim.state().texts.values().any(|text|text.starts_with("Server A\nJob A\nStop this job?"))).expect("the confirmation was sent");
        assert!(supervisor.heard().commands.is_empty(),"opening Stop is no command");
        assert_eq!(fleet.said("Server A\nJob A\nStop sent."),0);
        let question=*fleet.sim.state().texts.iter().find(|(_,text)|text.starts_with("Server A\nJob A\nStop this job?")).unwrap().0;
        let record=fleet.member("a").store.the_action_contexts_bound_to(question).unwrap().into_iter().find(|record|record.payload["confirmed"]==json!(true)).unwrap();
        fleet.sim.the_user_taps(CHAT,question,&record.token);
        fleet.rounds_until(20, |f| f.said("Server A\nJob A\nStop sent.") == 1).expect("D2's first half was said");
        fleet.rounds_until(20, |_| !supervisor.heard().commands.is_empty()).expect("the stop was submitted");
        fleet
            .rounds_until(20, |f| f.sim.the_board_on_top().is_some_and(|b| b.instructions.is_empty()))
            .expect("acknowledged and trimmed");
        for _ in 0..10 {
            fleet.a_round();
        }
        assert_eq!(fleet.said("Server A\nJob A\nStop sent."), 1, "said once");
        assert_eq!(supervisor.heard().commands.len(), 1, "submitted once");
    }

    /// **B12, R10 and fix-here item 17 — a gather is answered as soon as every
    /// server asked has replied**, well before its ten-second deadline.
    #[test]
    fn a_gather_is_answered_as_soon_as_every_server_asked_has_replied() {
        let (mut fleet, holder) = AFleet::settled(&["a", "b", "c"]);
        for _ in 0..10 {
            fleet.a_round();
        }
        fleet.sim.the_user_types(CHAT, "status");
        // **The holder's own board, read whatever is pinned over it.** Until
        // 2026-10-18 this read the board on top, which shows the gather only
        // where every reply is pinned in a later round than the boarding —
        // so only where the holder takes its turn last. Since then the
        // machines that do not hold announce after the first renewal (B14),
        // the slot settles on the first machine to claim, and the replies
        // pinned over the board in the boarding's own round hid it.
        let board_id = fleet.member(&holder).store.the_board().unwrap().unwrap();
        let on_the_board = move |f: &AFleet| {
            let s = f.sim.state();
            matches!(
                what_is_pinned(&json!({ "pinned_message": s.the_pinned_message(board_id) })),
                WhatIsPinned::ABoard { board, .. } if board.instructions.iter().any(|e| e.kind == AKindOfEntry::StateRequest)
            )
        };
        fleet.rounds_until(10, on_the_board).expect("the gather was boarded");
        let boarded = *lock(&fleet.clock.0);
        fleet.rounds_until(30, |f| !f.sim.said_to_the_user().is_empty()).expect("answered");
        let answered_after = lock(&fleet.clock.0).saturating_sub(boarded);
        eprintln!("the gather was answered {answered_after:?} after it was boarded");
        assert!(
            answered_after < THE_DEFAULT_RESOLUTION_DEADLINE,
            "answered {answered_after:?} after boarding, which is not before R12's deadline"
        );
        for _ in 0..10 {
            fleet.a_round();
        }
        let said = fleet.sim.said_to_the_user();
        assert_eq!(said.len(), 1, "one message: {said:?}");
        assert!(!said[0].1.contains("Did not answer."), "{said:?}");
        assert!(fleet.sim.the_board_on_top().unwrap().instructions.is_empty());
    }

    /// **`Telegram` B9a — a stop one protocol version ahead, boarded for an
    /// older owner, is refused and acknowledged, with no act.** A newer machine
    /// proposes it through its own machinery message (B15), the holder admits
    /// it, and the owner reads it on the board. The owner submits nothing,
    /// records the refusal beside the identity, tells the user its Smoozy is
    /// older, and acknowledges it, so the entry leaves the board and nobody is
    /// told it *has not confirmed*. Before piece 11 the owner submitted the
    /// stop and recorded it done.
    #[test]
    fn an_entry_one_version_ahead_is_refused_and_acknowledged_with_no_act() {
        use crate::board::instructions::{TheAddressee, THE_PROTOCOL_VERSION};
        use crate::board::text::TheUtterance;
        let mut fleet = AFleet::new();
        let supervisor = AStandInSupervisor::default();
        fleet.start("a");
        fleet.rounds_until(60, |f| f.holder().is_some()).expect("a holds");
        fleet.start_with("b", Some(supervisor.clone()));
        *lock(&fleet.member("b").jobs.0) = vec![a_rendering_job("b", "1")];
        fleet
            .rounds_until(60, |f| f.sim.the_board_on_top().is_some_and(|b| b.roster.len() == 2))
            .expect("both on the roster");

        let newer = AnEntry {
            version: THE_PROTOCOL_VERSION + 1,
            operation: "newer-stop".into(),
            addressee: TheAddressee::Machine(the_machine("b")),
            kind: AKindOfEntry::Action,
            subjects: vec!["job-b".into()],
            action: STOP_THIS_JOB.into(),
            payload: json!({ "generation": "1", "labels": { "server": "Server B", "label": "Job B", "button": "Stop this job" } }),
            source_update: None,
            accepted_at: String::new(),
            deadline_seconds: 30,
        };
        let proposal = TheUtterance { machine: "machine-newer".into(), display: "Server NEWER".into(), proposed: vec![newer], ..Default::default() };
        {
            let mut s = fleet.sim.state();
            s.next_message += 1;
            let id = s.next_message;
            s.texts.insert(id, proposal.compose().unwrap());
            let pinned = s.the_pinned_message(id);
            s.pins.push(id);
            s.next_update += 1;
            let update = s.next_update;
            s.updates.push(json!({ "update_id": update, "message": { "message_id": 50_000 + update, "chat": { "id": CHAT }, "pinned_message": pinned } }));
        }
        fleet
            .rounds_until(30, |f| {
                f.sim.the_board_on_top().is_some_and(|b| b.instructions.iter().any(|e| e.operation == "newer-stop"))
                    || !supervisor.heard().recorded.is_empty()
            })
            .expect("the holder admits the newer machine's proposal");
        fleet
            .rounds_until(60, |f| f.sim.the_board_on_top().is_some_and(|b| b.instructions.is_empty()))
            .expect("the entry leaves the board");
        for _ in 0..30 {
            fleet.a_round();
        }
        let said = fleet.sim.said_to_the_user();
        assert!(supervisor.heard().commands.is_empty(), "B9a: an entry of a version the owner does not speak is never acted on");
        assert!(said.iter().all(|(_, t)| !t.contains("did not confirm")), "B9a: never *did not confirm*");
        assert_eq!(
            said.iter().filter(|(_, t)| t.as_str() == "Server B\nJob B\n\u{201c}Stop this job\u{201d} failed: update Smoozy on this server.").count(),
            1,
            "B9a: the user is told once that the server's Smoozy is older: {said:?}"
        );
        assert_eq!(
            supervisor.heard().recorded.get("newer-stop"),
            Some(&crate::actions::ARecordedOutcome::Failed(crate::board::instructions::WHY_AN_ENTRY_OF_ANOTHER_VERSION_IS_REFUSED.into())),
            "B9: the refusal is recorded beside the identity"
        );
    }

    /// **`Telegram` B9a — a newer listener's context-resolution entry for a tap
    /// on an older machine's button is not resolved there**, so nothing is
    /// submitted and the entry ends at its own deadline. Before piece 11 the
    /// older issuer resolved it by its own rules and the stop was submitted.
    #[test]
    fn a_newer_resolution_entry_is_not_resolved_by_an_older_issuer() {
        use crate::board::instructions::THE_PROTOCOL_VERSION;
        use crate::board::text::TheUtterance;
        use crate::routing::context::{a_taps_resolution, ATapsEvidence};
        use crate::routing::token::WhatATokenNames;
        let mut fleet = AFleet::new();
        let supervisor = AStandInSupervisor::default();
        fleet.start("a");
        fleet.rounds_until(60, |f| f.holder().is_some()).expect("a holds");
        fleet.start_with("b", Some(supervisor.clone()));
        *lock(&fleet.member("b").jobs.0) = vec![a_rendering_job("b", "1")];
        fleet
            .rounds_until(60, |f| f.sim.the_board_on_top().is_some_and(|b| b.roster.len() == 2))
            .expect("both on the roster");
        let message = fleet.sim.a_message_from_the_bot("a picture of a frame");
        let client = TheClient::over(Arc::new(ACaller { name: "b".into(), sim: fleet.sim.clone(), latency: None }));
        let mut messages = TheJobMessages {
            conversation: CHAT,
            store: Box::new(fleet.member("b").store.clone()),
            queue: Arc::new(TheSendQueue::new(client, Arc::new(NoWaits))),
            notes: Box::new(|_| {}),
        };
        let subject = TheSubject { owning_machine: the_machine("b"), job: Some("job-b".into()), hold: None, generation: "1".into() };
        let button = messages.a_stop_button(message, &subject, "Server B", "Job B").unwrap();
        let token = button["callback_data"].as_str().unwrap().to_string();
        let evidence = ATapsEvidence { token: token.clone(), callback: "cb-newer".into(), conversation: CHAT, source_message: message, label: None };
        let mut newer = a_taps_resolution(&evidence, 1, &WhatATokenNames::read(&token).unwrap());
        newer.version = THE_PROTOCOL_VERSION + 1;
        let proposal = TheUtterance { machine: "machine-newer".into(), display: "Server NEWER".into(), proposed: vec![newer], ..Default::default() };
        {
            let mut s = fleet.sim.state();
            s.next_message += 1;
            let id = s.next_message;
            s.texts.insert(id, proposal.compose().unwrap());
            let pinned = s.the_pinned_message(id);
            s.pins.push(id);
            s.next_update += 1;
            let update = s.next_update;
            s.updates.push(json!({ "update_id": update, "message": { "message_id": 50_000 + update, "chat": { "id": CHAT }, "pinned_message": pinned } }));
        }
        for _ in 0..40 {
            fleet.a_round();
        }
        assert!(supervisor.heard().commands.is_empty(), "B9a: a resolution entry of a version the issuer does not speak is never resolved");
    }


    // ------------------------------------------------------------ the Telegram discussion, 2026-10-31

    use crate::routing::context::{CHOOSING_THE_PHONES_CHOICE, EVERY_LIVE_JOBS_REPORTING, ONE_JOBS_REPORTING, SENDING_THE_NEWEST_FRAME};
    use crate::routing::typed::{COULD_NOT_TELL_WHICH_JOB, NOTHING_TO_STOP, WHOLE_MINUTES};
    use crate::worker_state::{ASentMessage, WhatAMessageIsAbout};

    fn the_acted(fleet: &AFleet) -> Vec<(String, AnEntry)> {
        lock(&fleet.acted).clone()
    }

    fn the_others(holder: &str, names: &[&str]) -> Vec<String> {
        names.iter().filter(|n| **n != holder).map(|n| n.to_string()).collect()
    }

    /// The buttons on the newest message saying `text`, with their data.
    fn the_buttons_under(fleet: &AFleet, text: &str) -> (i64, Vec<(String, String)>) {
        let s = fleet.sim.state();
        let id = *s.texts.iter().filter(|(_, t)| t.as_str() == text).map(|(id, _)| id).max().expect("the message was sent");
        let rows = s.buttons.get(&id).and_then(|m| m["inline_keyboard"].as_array().cloned()).unwrap_or_default();
        (id, rows.iter().flat_map(|row| row.as_array().unwrap().iter().map(|b| (b["text"].as_str().unwrap().to_string(), b["callback_data"].as_str().unwrap().to_string()))).collect())
    }

    /// **`Telegram` R17: the holder puts the command list in place, once, and
    /// only where Telegram's differs; no other machine calls either method.**
    #[test]
    fn telegram_discussion_the_holder_sets_the_command_list_only_where_it_differs() {
        let (mut fleet, holder) = AFleet::settled(&["a", "b"]);
        for _ in 0..40 {
            fleet.a_round();
        }
        let calls = |fleet: &AFleet, name: &str, method: &str| fleet.sim.state().heard.iter().filter(|(c, m, _)| c == name && m == method).count();
        let other = if holder == "a" { "b" } else { "a" };
        assert_eq!(calls(&fleet, &holder, "setMyCommands"), 1, "set once, where the list differed");
        assert!(calls(&fleet, &holder, "getMyCommands") >= 1);
        assert_eq!(calls(&fleet, other, "getMyCommands") + calls(&fleet, other, "setMyCommands"), 0, "only the holder");
        assert_eq!(fleet.sim.state().commands, crate::routing::typed::the_command_list());
        assert!(!crate::routing::typed::the_command_list_differs(&fleet.sim.state().commands));

        // A new conversation whose list already matches: compared, never set.
        let mut matching = AFleet::new();
        matching.sim.state().commands = crate::routing::typed::the_command_list();
        matching.start("a");
        matching.rounds_until(60, |f| f.holder().is_some()).expect("it holds");
        for _ in 0..10 {
            matching.a_round();
        }
        assert_eq!(calls(&matching, "a", "setMyCommands"), 0);
        assert!(calls(&matching, "a", "getMyCommands") >= 1);
    }

    /// **`Telegram` R13a: `stop` with none, one and several jobs.** None is
    /// answered in words; one goes straight to D0's question on its owner;
    /// several are offered, the server after the job where two are offered,
    /// and a tap is a press of *Stop this job* for that job.
    #[test]
    fn telegram_discussion_stop_with_no_job_one_job_and_several() {
        let names = ["a", "b", "c"];
        let (mut fleet, holder) = AFleet::settled(&names);
        let others = the_others(&holder, &names);
        fleet.sim.the_user_types(CHAT, "/stop");
        fleet.rounds_until(30, |f| f.said(NOTHING_TO_STOP) == 1).expect("none: said so");
        assert!(the_acted(&fleet).is_empty());

        *lock(&fleet.member(&others[0]).jobs.0) = vec![a_rendering_job(&others[0], "1")];
        fleet.sim.the_user_types(CHAT, "/stop@SmoozyBot");
        fleet.rounds_until(30, |f| !lock(&f.acted).is_empty()).expect("one: straight to its owner");
        let (owner, entry) = the_acted(&fleet)[0].clone();
        assert_eq!((owner, entry.action.as_str()), (others[0].clone(), crate::job_messages::ASK_TO_STOP_THIS_JOB));
        assert_eq!(entry.subjects, vec![format!("job-{}", others[0])]);

        *lock(&fleet.member(&others[1]).jobs.0) = vec![a_rendering_job(&others[1], "1")];
        fleet.sim.the_user_types(CHAT, "stop");
        fleet
            .rounds_until(30, |f| f.sim.state().texts.values().filter(|t| t.as_str() == WHICH_JOB).count() == 1 && !the_buttons_under(f, WHICH_JOB).1.is_empty())
            .expect("several: offered");
        let (question, buttons) = the_buttons_under(&fleet, WHICH_JOB);
        assert_eq!(buttons.len(), 2);
        assert!(buttons.iter().all(|(label, _)| label.contains(" — Server ")), "two servers: the server after the job: {buttons:?}");
        fleet.sim.the_user_taps(CHAT, question, &buttons[0].1);
        fleet.rounds_until(30, |f| lock(&f.acted).len() == 2).expect("the tap reached its owner");
        assert_eq!(the_acted(&fleet)[1].1.action, crate::job_messages::ASK_TO_STOP_THIS_JOB);
    }

    /// **`Telegram` R13a: `frame` alone asks for the newest frame**, its own
    /// action and no number.
    #[test]
    fn telegram_discussion_frame_alone_is_the_newest_frame() {
        let (mut fleet, holder) = AFleet::settled(&["a", "b"]);
        let owner = the_others(&holder, &["a", "b"]).remove(0);
        *lock(&fleet.member(&owner).jobs.0) = vec![a_rendering_job(&owner, "1")];
        fleet.sim.the_user_types(CHAT, "/frame");
        fleet.rounds_until(30, |f| !lock(&f.acted).is_empty()).expect("routed to its one job");
        let entry = the_acted(&fleet)[0].1.clone();
        assert_eq!(entry.action, SENDING_THE_NEWEST_FRAME);
        assert!(entry.payload.get("frame").is_none(), "no number rides it: {}", entry.payload);
    }

    /// **`Telegram` R1a, widened: `/frame`, `/stop` and `/reporting` sent as
    /// replies to a job's message act on that job, on another server and on
    /// the listening one.**
    #[test]
    fn telegram_discussion_replies_act_on_the_replied_to_job_on_either_server() {
        let (mut fleet, holder) = AFleet::settled(&["a", "b"]);
        let other = the_others(&holder, &["a", "b"]).remove(0);
        for name in [holder.clone(), other.clone()] {
            *lock(&fleet.member(&name).jobs.0) = vec![a_rendering_job(&name, "1")];
        }
        let on = |fleet: &mut AFleet, name: &str, frame: Option<u64>| {
            let message = fleet.sim.a_message_from_the_bot(&format!("a message about job-{name}"));
            let line = ASentMessage {
                message,
                conversation: CHAT,
                about: WhatAMessageIsAbout::AJob { job: format!("job-{name}"), generation: Some("1".into()), frame, layer: None },
            };
            fleet.member(name).store.register_a_sent_message(line).unwrap();
            message
        };
        for name in [other.clone(), holder.clone()] {
            let picture = on(&mut fleet, &name, Some(33));
            let before = the_acted(&fleet).len();
            fleet.sim.the_user_replies_with(picture, "/frame", None);
            fleet.sim.the_user_replies_with(picture, "/stop", None);
            fleet.sim.the_user_replies_with(picture, "/reporting 10", None);
            fleet.rounds_until(40, |f| lock(&f.acted).len() == before + 3).expect("all three reached the job's own server");
            let acted = the_acted(&fleet)[before..].to_vec();
            assert!(acted.iter().all(|(owner, e)| *owner == name && e.subjects == vec![format!("job-{name}")]), "{acted:?}");
            let by = |action: &str| acted.iter().find(|(_, e)| e.action == action).map(|(_, e)| e.clone()).unwrap_or_else(|| panic!("{action}: {acted:?}"));
            assert_eq!(by(crate::routing::context::SENDING_A_FRAME).payload["frame"], json!(33), "a reply to a picture sends that frame");
            by(crate::job_messages::ASK_TO_STOP_THIS_JOB);
            assert_eq!(by(ONE_JOBS_REPORTING).payload["value"], json!({ "APictureEveryFewMinutes": { "minutes": 10 } }));
        }
        assert!(the_acted(&fleet).iter().all(|(_, e)| e.action != EVERY_LIVE_JOBS_REPORTING), "a reply never changes every job");
    }

    /// **`Telegram` R1a, widened: a reply to the pinned note, to the bot's own
    /// answer or to the user's own message is handled at once as no reply**
    /// — here, with one job live, `/stop` going straight to its question — and
    /// nothing waits for a deadline.
    #[test]
    fn telegram_discussion_a_reply_to_the_pinned_note_or_the_bots_own_answer_is_as_no_reply() {
        let (mut fleet, holder) = AFleet::settled(&["a"]);
        *lock(&fleet.member("a").jobs.0) = vec![a_rendering_job("a", "1")];
        let board = fleet.member(&holder).store.the_board().unwrap().expect("the holder's board");
        fleet.sim.the_user_replies_with(board, "/stop", None);
        let rounds = fleet.rounds_until(30, |f| !lock(&f.acted).is_empty()).expect("as no reply");
        assert!(rounds < 5, "at once, not after the ten-second deadline: {rounds} rounds");
        assert_eq!(the_acted(&fleet)[0].1.action, crate::job_messages::ASK_TO_STOP_THIS_JOB);

        // The bot's own answer, registered by the listener as about no job.
        lock(&fleet.member("a").jobs.0).clear();
        fleet.sim.the_user_types(CHAT, "/stop");
        fleet.rounds_until(30, |f| f.said(NOTHING_TO_STOP) == 1).expect("answered");
        let answer = fleet.sim.said_to_the_user().iter().rev().find(|(_, t)| t == NOTHING_TO_STOP).unwrap().0;
        assert!(matches!(
            fleet.member("a").store.the_sent_message(answer).unwrap(),
            Some(ASentMessage { about: WhatAMessageIsAbout::NoJob, .. })
        ));
        fleet.sim.the_user_replies_with(answer, "/stop", None);
        let rounds = fleet.rounds_until(30, |f| f.said(NOTHING_TO_STOP) == 2).expect("as no reply");
        assert!(rounds < 5, "at once: {rounds} rounds");

        // The user's own message, where the reply shows it.
        fleet.sim.the_user_replies_with(90_000, "/stop", Some(true));
        let rounds = fleet.rounds_until(30, |f| f.said(NOTHING_TO_STOP) == 3).expect("as no reply");
        assert!(rounds < 5, "at once: {rounds} rounds");
        assert_eq!(fleet.said(COULD_NOT_TELL_WHICH_JOB), 0);
    }

    /// **`Telegram` R1a, widened: a reply nobody claims** — a message of a
    /// server that is gone, or from before the register — **fails closed for
    /// `/stop` and `/reporting`**, and nothing is done; `/frame` falls back as
    /// it always has.
    #[test]
    fn telegram_discussion_a_reply_to_an_unknown_message_fails_closed_for_stop_and_reporting() {
        let (mut fleet, holder) = AFleet::settled(&["a", "b"]);
        let other = the_others(&holder, &["a", "b"]).remove(0);
        *lock(&fleet.member(&other).jobs.0) = vec![a_rendering_job(&other, "1")];
        let unknown = fleet.sim.a_message_from_the_bot("a message nobody registered");
        fleet.sim.the_user_replies_with(unknown, "/stop", None);
        fleet.rounds_until(40, |f| f.said(COULD_NOT_TELL_WHICH_JOB) == 1).expect("failed closed");
        fleet.sim.the_user_replies_with(unknown, "/reporting 10", None);
        fleet.rounds_until(40, |f| f.said(COULD_NOT_TELL_WHICH_JOB) == 2).expect("failed closed");
        for _ in 0..10 {
            fleet.a_round();
        }
        assert!(the_acted(&fleet).is_empty(), "nothing was stopped or changed: {:?}", the_acted(&fleet));
        assert_eq!(fleet.said(WHICH_JOB), 0, "never widened to Which job?");
        fleet.sim.the_user_replies_with(unknown, "/frame", None);
        fleet.rounds_until(40, |f| !lock(&f.acted).is_empty()).expect("`/frame` fell back to its one job");
        assert_eq!(the_acted(&fleet)[0].1.action, SENDING_THE_NEWEST_FRAME);
    }

    /// **`Telegram` V1d: `/reporting` asks with five buttons; a tap, or
    /// `/reporting 10`, tells every machine that can take it — itself
    /// included — and one reply names the machine not reached and the one
    /// that needs an update.** A number outside 1 to 60 is refused.
    #[test]
    fn telegram_discussion_the_phones_choice_reaches_every_machine_and_one_reply_names_the_rest() {
        let names = ["a", "b", "c", "d"];
        let (mut fleet, holder) = AFleet::settled(&names);
        let others = the_others(&holder, &names);
        let (fine, older, gone) = (others[0].clone(), others[1].clone(), others[2].clone());
        fleet.member(&older).router.as_if_older_than_the_phones_choice();
        fleet.member(&gone).alive = false;

        fleet.sim.the_user_types(CHAT, "/reporting 61");
        fleet.rounds_until(20, |f| f.said(WHOLE_MINUTES) == 1).expect("refused with the range");

        fleet.sim.the_user_types(CHAT, "/reporting");
        let ask = format!("{}\n{}", crate::routing::THE_PHONES_ASK, crate::routing::FOR_OTHER_MINUTES);
        fleet.rounds_until(20, |f| f.said(&ask) == 1 && !the_buttons_under(f, &ask).1.is_empty()).expect("the ask");
        let (message, buttons) = the_buttons_under(&fleet, &ask);
        let labels: Vec<&str> = buttons.iter().map(|(l, _)| l.as_str()).collect();
        assert_eq!(labels, ["Everything", "Pictures only", "A picture every 5 minutes", "Alerts only", "Off"]);
        fleet.sim.the_user_taps(CHAT, message, &buttons[3].1);
        let display = |name: &str| format!("Server {}", name.to_uppercase());
        let reply = format!(
            "Reporting set to \u{201c}Alerts only\u{201d} for every job and new jobs.\nNot reached: {}. Its jobs keep their choice.\n{} needs a Smoozy update for this.",
            display(&gone),
            display(&older)
        );
        fleet.rounds_until(60, |f| f.said(&reply) == 1).unwrap_or_else(|| panic!("the one reply: {:?}", fleet.sim.said_to_the_user()));
        let told: Vec<(String, AnEntry)> = the_acted(&fleet).into_iter().filter(|(_, e)| e.action == EVERY_LIVE_JOBS_REPORTING).collect();
        let mut machines: Vec<String> = told.iter().map(|(m, _)| m.clone()).collect();
        machines.sort();
        let mut expected = vec![holder.clone(), fine.clone()];
        expected.sort();
        assert_eq!(machines, expected, "the holder and the machine that can take it, never the older or the gone");
        assert!(told.iter().all(|(_, e)| e.payload["value"] == json!("OnlyWhatNeedsAnAnswer") && e.payload["operation"] == told[0].1.payload["operation"]));
        assert_eq!(told[0].1.payload["date"], json!(1_730_000_000 + message), "a tap takes its ask's date");
        assert!(the_acted(&fleet).iter().any(|(_, e)| e.action == CHOOSING_THE_PHONES_CHOICE), "the press itself is recorded");

        // **B14, 2026-10-31 (the twenty-sixth notes, note 2)**: the gone
        // machine was named once, by the round above, and is off the roster,
        // so the second round neither asks it nor names it.
        fleet.sim.the_user_types(CHAT, "/reporting 10");
        let typed = format!(
            "Reporting set to \u{201c}A picture every 10 minutes\u{201d} for every job and new jobs.\n{} needs a Smoozy update for this.",
            display(&older)
        );
        fleet.rounds_until(60, |f| f.said(&typed) == 1).expect("the second reply");
    }

    // ------------------------------------------------------------ the twenty-sixth notes, 2026-10-31

    /// **The twenty-sixth notes, note 3 (`Telegram` C1b and V1b as changed
    /// 2026-10-31): the reply to *Status* offers no Stop**, though a job it
    /// names is rendering. Creator-stated: *"/status should not offer to stop
    /// jobs."* Before, the reply was edited to carry *Stop {server} — {job}*
    /// for every live job.
    #[test]
    fn twenty_sixth_3_the_status_reply_offers_no_stop() {
        let names = ["a", "b", "c"];
        let (mut fleet, holder) = AFleet::settled(&names);
        for name in the_others(&holder, &names) {
            *lock(&fleet.member(&name).jobs.0) = vec![a_rendering_job(&name, "1")];
        }
        fleet.sim.the_user_types(CHAT, "/status");
        fleet.rounds_until(30, |f| !f.sim.said_to_the_user().is_empty()).expect("the reply was sent");
        for _ in 0..10 {
            fleet.a_round();
        }
        let said = fleet.sim.said_to_the_user();
        assert_eq!(said.len(), 1, "one message: {said:?}");
        assert!(said[0].1.contains("· frame 12 of 40 · 30%"), "it names the rendering jobs: {said:?}");
        let (_, buttons) = the_buttons_under(&fleet, &said[0].1);
        assert!(buttons.is_empty(), "no Stop on the reply to Status: {buttons:?}");
        let records = fleet.member(&holder).store.the_action_contexts_bound_to(said[0].0).unwrap();
        assert!(records.is_empty(), "and no Stop record for it: {records:?}");
    }

    /// **The twenty-sixth notes, note 2 (`Telegram` B14 as written
    /// 2026-10-31): a server that does not answer a gather is named as not
    /// answering once, and is then off the roster until it is heard from
    /// again.** One server is gone for good; another is silent for one
    /// *Status* and comes back by its own announcement. Creator-stated:
    /// *"/status … talks about servers that do not exist."* Before, nothing
    /// took a silent server off, and every *Status* named it again.
    #[test]
    fn twenty_sixth_2_a_silent_server_is_named_once_and_a_live_one_puts_itself_back() {
        let names = ["a", "b", "c", "d"];
        let (mut fleet, holder) = AFleet::settled(&names);
        let others = the_others(&holder, &names);
        let (gone, slow, steady) = (others[0].clone(), others[1].clone(), others[2].clone());
        for name in [&slow, &steady] {
            *lock(&fleet.member(name).jobs.0) = vec![a_rendering_job(name, "1")];
        }
        let upper = |name: &str| name.to_uppercase();
        let listed = |fleet: &AFleet, name: &str| {
            let holder = fleet.members.iter().find(|m| m.listener.is_holding()).expect("one machine holds");
            holder.listener.the_board_as_last_read().is_some_and(|b| b.roster.iter().any(|r| r.machine == the_machine(name)))
        };
        fleet.member(&gone).alive = false;
        fleet.member(&slow).alive = false;

        // 1. Both silent servers are named once.
        fleet.sim.the_user_types(CHAT, "/status");
        fleet.rounds_until(30, |f| !f.sim.said_to_the_user().is_empty()).expect("the first reply");
        let first = fleet.sim.said_to_the_user()[0].1.clone();
        assert!(first.contains(&format!("Server {}\nDid not answer.", upper(&gone))), "{first}");
        assert!(first.contains(&format!("Server {}\nDid not answer.", upper(&slow))), "{first}");
        assert!(first.contains(&format!("Server {}\nJob {} · frame 12 of 40 · 30%", upper(&steady), upper(&steady))), "{first}");
        fleet.rounds_until(5, |f| !listed(f, &gone) && !listed(f, &slow)).expect("both are off the roster at the holder's next board edit");
        assert!(listed(&fleet, &holder) && listed(&fleet, &steady), "the servers that answered stay");

        // 2. The live one puts itself back by its own announcement; the gone
        // one stays off.
        fleet.member(&slow).alive = true;
        fleet.rounds_until(120, |f| listed(f, &slow)).expect("the live server announced itself back");
        assert!(!listed(&fleet, &gone), "nothing puts the gone server back");

        // 3. The next Status names the live one and says nothing of the gone one.
        let before = fleet.sim.said_to_the_user().len();
        fleet.sim.the_user_types(CHAT, "/status");
        fleet.rounds_until(30, |f| f.sim.said_to_the_user().len() > before).expect("the second reply");
        for _ in 0..10 {
            fleet.a_round();
        }
        let said = fleet.sim.said_to_the_user();
        assert_eq!(said.len(), before + 1, "one more message: {said:?}");
        let second = &said[before].1;
        assert!(!second.contains("Did not answer."), "{second}");
        assert!(!second.contains(&format!("Server {}", upper(&gone))), "the gone server is not named again: {second}");
        assert!(second.contains(&format!("Server {}\nJob {} · frame 12 of 40 · 30%", upper(&slow), upper(&slow))), "{second}");
        assert!(second.contains(&format!("Server {}\nIdle.", upper(&holder))), "{second}");
    }
}
