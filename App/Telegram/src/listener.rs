//! **The listener — the slot and the board acted out.** One role worker's
//! conduct under `Telegram` §3 and §4 J3, against piece 1's one client and one
//! send queue, and a store handed in through [`TheTelegramWorkersState`].
//!
//! **What a turn is.** Every [`THE_READ_CADENCE`] the listener reads the
//! reported pin (B13). **Holding**, it pops the stack to its own board (B11),
//! takes in what the machinery messages said, acts on what is addressed to it,
//! measures deadlines (B12), renews the heartbeat (B4), and writes all of that
//! as one board edit confirmed by read-back (`Rented-server runtime` T12). **Not holding**, it watches
//! the heartbeat without subtracting across clocks (B5), acts on and
//! acknowledges what is addressed to it (B9), announces itself to a holder it
//! has seen alive (B14), proposes (B15), and asks for the slot only when B5
//! allows (B6).
//!
//! **Asking is separate from the turn**, so a holder keeps one ask open to
//! Telegram for most of its life ([`TheListener::the_ask_to_make`]). That is
//! what lets a newer asker's request refuse the holder's (`Rented-server runtime` T4, T6): an ask that
//! has returned cannot be refused.
//!
//! **Markers in this file are all `Telegram`'s** unless another document is
//! named: B1, B3 to B15, B16, B18, B19, J3, F3, R16, C6's heartbeat never
//! deferred by a send in flight, and §19's first three rows.
//! **§19's declarations, kept here:** a slot claim is atomic at Telegram and
//! writes nothing until the ask is won; a board write is one edit, accepted only
//! after read-back; a routed instruction is accepted on the confirmed board,
//! recorded at the addressee before acting, and applied once by identity.
//!
//! **B16: only the board and machinery messages are ever pinned here.** A
//! machine's status message is piece 6's and is never pinned. **B18: the one
//! thing ever deleted is the line a pin of Smoozy's own writes**, by that
//! line's identity, once the holder has processed the pin
//! ([`the_line_a_pin_of_smoozys_wrote`]). A pinned message is never deleted,
//! nor a line from a pin the user made.

use crate::job_messages::{AServerLabel, AMarkedText, a_user_message};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::TryRecvError;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant, SystemTime};

use serde_json::Value;
use smoozy_state::identities::{Identity, IdentityKind};

use crate::board::hand_over::{
    the_successor, what_the_claimant_does, TheClaimant, TheUnreadableBoardWatch, WhatTheClaimantDoes,
};
use crate::board::instructions::{
    what_the_deadline_owes, without, AKindOfEntry, AnEntry, TheAddressee, WhatADeadlineOwes, WhoIAm,
    THE_DEFAULT_REPORTING_DEADLINE, THE_PROTOCOL_VERSION,
};
use crate::board::janitor::{what_an_utterance_asks, what_the_janitor_does, NotAdmitted, WhatTheJanitorDoes};
use crate::board::roster::{must_announce, the_display_of, without_the_silent};
use crate::board::staleness::{
    how_often_the_holder_renews, TheProofOfLife, TheStalenessWatch, WhatACompletedReadSaw,
    THE_DEFAULT_RENEWAL_INTERVAL, THE_READ_CADENCE,
};
use crate::board::text::{the_moment_in_utc, what_is_pinned, TheBoard, TheUtterance, WhatIsPinned};
use crate::pacing::{AClassOfSend, ASend, HowASendEnded, TheSendQueue};
use crate::proof::{what_an_update_gets, TheConversation, WhatAnUpdateGets};
use crate::routing::context::SENDING_A_FRAME;
use crate::transport::{TheClient, WhatTheServiceGaveBack, WhyTheCallFailed};
use crate::worker_state::TheTelegramWorkersState;

/// How many pins one turn pops before it gives the rest to the next turn.
const THE_MOST_POPS_IN_ONE_TURN: usize = 20;

/// **How long completed reads must go on showing nothing pinned before this
/// machine believes it** — before a holder pins its board again (`Telegram`
/// B11, *"re-pinning the board where it is absent"*) or a claimant starts a
/// fresh board (B3). Measured between this machine's own reading moments, the
/// shape of B5.
///
/// **Why.** The second live watch of 2026-09-14 saw the holder pin its own,
/// still-pinned board 33 times, in bursts that began right after it unpinned a
/// machinery message and ended within one to three seconds. The likeliest
/// reading — inferred, not logged — is that the service reports nothing pinned
/// for about a second after an unpin. **One read showing nothing is therefore
/// not evidence the board is absent.** For a claimant it is worse than a
/// wasted pin: a fresh board would drop every accepted instruction on the old
/// one, which B3 exists to prevent. Twice the read cadence: longer than the
/// longest burst seen, and short beside the renewal interval every other
/// absence is measured against.
pub const THE_SETTLE_BEFORE_NOTHING_PINNED_IS_BELIEVED: Duration = Duration::from_secs(4);

/// **How many reads one turn makes, with no pin between them, while nothing
/// pinned is not yet believed.** The bursts the second live watch saw were
/// about six pin-and-read pairs, over one to three seconds, so ten reads
/// usually outlast the lag. Past it the turn ends unfinished, and the next turn
/// reads again. A read is not a send and pins nothing, so no line is written.
const THE_MOST_UNSETTLED_READS_IN_ONE_TURN: usize = 10;

/// **How far apart those reads are**, on this machine's own clock — `Telegram`
/// B3 and B11 as written 2026-10-18 (the Creator's fourteenth notes, note 2,
/// with the two independent checks of that day folded in): *"spaced about half
/// a second apart, so one turn's reads cover the whole wait before nothing
/// pinned is believed"*. Ten reads with no pause took one to three seconds,
/// short of the four-second settle, so a claimant that had just unpinned a
/// machinery message could never wait out a lag longer than its reads: its
/// claim ended unfinished, and the next turn pinned its announcement again —
/// the repeating loud notice the checks traced. Ten reads half a second apart
/// span five seconds, past the settle.
pub const THE_SPACING_OF_UNSETTLED_READS: Duration = Duration::from_millis(500);

/// What happened, in words that never carry a token or a message's content.
pub type Notes = Box<dyn FnMut(&str) + Send>;

/// **A machine's own clock.** `now` is for intervals on this machine only;
/// `the_moment` is what is written beside a heartbeat or an acceptance.
pub trait ReadsItsOwnClock: Send + Sync {
    fn now(&self) -> Duration;
    fn the_moment(&self) -> String;
    /// **Let `how_long` pass on this clock** — the pause between the reads
    /// that wait out a report of nothing pinned
    /// ([`THE_SPACING_OF_UNSETTLED_READS`]). A clock a test moves by hand moves
    /// itself on here instead of sleeping.
    fn pause(&self, how_long: Duration) {
        std::thread::sleep(how_long);
    }
}

/// This computer's clock.
pub struct OnThisComputer {
    origin: Instant,
}

impl OnThisComputer {
    pub fn new() -> OnThisComputer {
        OnThisComputer { origin: Instant::now() }
    }
}

impl Default for OnThisComputer {
    fn default() -> Self {
        OnThisComputer::new()
    }
}

impl ReadsItsOwnClock for OnThisComputer {
    fn now(&self) -> Duration {
        self.origin.elapsed()
    }
    fn the_moment(&self) -> String {
        the_moment_in_utc(SystemTime::now())
    }
}

/// **`Telegram` B9 — acting on an instruction addressed here.** On a machine
/// both halves are the supervisor's command path (`Server protocol and
/// transfers` C4; `Rented-server runtime` U10), which is step 10c's; a desk process hands in its own.
pub trait ActsOnAnInstruction: Send {
    /// **B9 and `Server protocol and transfers` C4 — act on an instruction
    /// once, with its outcome, success or failure, recorded beside its
    /// identity.** Gives back `Ok` once an outcome is recorded; a repeated
    /// delivery finds the recorded outcome and acts on nothing, and a recorded
    /// failure has been told to the user in plain words. **`Err` means nothing
    /// was recorded**, so the instruction stays unacknowledged and is tried
    /// again at the next read. Until the round-1 code audit of the bot and the
    /// chat (U6, fix-here item 2) the identity alone was recorded before acting,
    /// so an act that failed once was acknowledged at the next read and never
    /// done, with nobody told.
    fn act_once(&mut self, entry: &AnEntry) -> Result<(), String>;
}

/// **What the listener hands on to the pieces that route and answer.**
/// Piece 3's routing and gather, and piece 4's conversation rule (`Telegram`
/// P2), meet the listener here.
pub trait HandsOn: Send {
    /// An update from the proved conversation that is not a pin. **Its work is
    /// done when this gives back `Ok`**, and only then is it confirmed (F3).
    fn an_update_from_the_conversation(&mut self, update: &Value) -> Result<(), String>;
    /// A state-request or context-resolution entry addressed to this machine.
    /// **Gives back this machine's reply to a state-request (R10), where it
    /// has one**, so the reply is said in the same turn the request was read,
    /// and the holder takes its own in, and completes its own entry, in the
    /// same board write that boarded the request (B12). Added by the round-1
    /// code audit of the bot and the chat, with fix-here item 17: routed a turn
    /// later, the holder's own reply cost a board write of its own, which on a
    /// slow line pushed another server's reply past R12's deadline.
    fn an_entry_addressed_here(&mut self, _entry: &AnEntry) -> Option<Value> {
        None
    }
    /// A state-request or context-resolution entry this holder removed at its
    /// deadline (B12); answering the user is R1, R1a and R12's.
    fn an_entry_ended_at_its_deadline(&mut self, _entry: &AnEntry) {}
    /// A gather's reply, taken in by the janitor (B11, R11).
    fn a_reply_to_a_gather(&mut self, _from: &str, _reply: &Value) {}
    /// **B12 and R10 — a state-request this holder removed because its reply
    /// was taken in**: completed, not expired. Added by the round-1 code audit
    /// of the bot and the chat, fix-here item 17.
    fn an_entry_was_completed(&mut self, _entry: &AnEntry) {}
    /// **B2 and F3 — an entry this machine proposed that the board could not
    /// admit.** The truthful refusal has been sent, so the work of the update
    /// behind it is done. Added by the same round, fix-here item 1.
    fn an_entry_was_not_admitted(&mut self, _entry: &AnEntry) {}
}

/// Who this role worker is: the machine identity from its role configuration,
/// and the display material it announces (B14).
#[derive(Debug, Clone)]
pub struct WhoThisMachineIs {
    pub machine: String,
    pub display: AServerLabel,
}

/// **The listener's numbers.** Every default is the plan's, or a choice
/// recorded at `Audits/Build/Telegram/Decisions record.md`.
#[derive(Debug, Clone)]
pub struct TheSettings {
    /// The proved conversation (`Telegram` P1). Updates from any other are
    /// dismissed (P2, P3).
    pub conversation: i64,
    /// **B2's stated renewal interval**, default 60 seconds.
    pub renewal: Duration,
    /// **How long a claimant's ask stays open.** Long enough that claimants
    /// asking within a few seconds of each other are settled by Telegram — the
    /// newest wins and the rest are refused — before any of them sends a board.
    pub the_claims_wait_seconds: u32,
    /// How long a holder's ask stays open. The observed ground carries a full
    /// 25-second poll undisturbed by reads.
    pub the_holders_wait_seconds: u32,
    /// B12's reporting deadline for an action instruction this machine proposes.
    pub the_reporting_deadline: Duration,
    /// **B15 and B19** — how long an unchanged utterance waits before it is
    /// pinned again **to the same holder**. A pin reaches a live holder through
    /// its update, which F3 brings back until the holder's work on it is done,
    /// so a re-pin to that holder is owed only where the holder could not write
    /// what it took in — and a holder that cannot write its board is replaced
    /// within one renewal interval and a read (B7), when this machine pins again
    /// for the new holder (B14). **So the default is the stated renewal
    /// interval.** Ten seconds until 2026-09-13's second sitting; the reasons
    /// are in `Audits/Build/Telegram/Decisions record.md`. **Since 2026-10-18
    /// every such repeat also waits for proof that the holder is alive** (B14,
    /// B15): a re-pin to a dead holder was never read and cost a loud notice.
    pub say_again_after: Duration,
}

impl TheSettings {
    pub fn for_the_conversation(conversation: i64) -> TheSettings {
        TheSettings {
            conversation,
            renewal: THE_DEFAULT_RENEWAL_INTERVAL,
            the_claims_wait_seconds: 5,
            the_holders_wait_seconds: 25,
            the_reporting_deadline: THE_DEFAULT_REPORTING_DEADLINE,
            say_again_after: THE_DEFAULT_RENEWAL_INTERVAL,
        }
    }
}

/// Everything a listener is handed.
pub struct WhatTheListenerIsHanded {
    pub client: TheClient,
    pub queue: Arc<TheSendQueue>,
    pub clock: Arc<dyn ReadsItsOwnClock>,
    pub me: WhoThisMachineIs,
    pub settings: TheSettings,
    pub store: Box<dyn TheTelegramWorkersState>,
    pub actor: Box<dyn ActsOnAnInstruction>,
    pub hands_on: Box<dyn HandsOn>,
    /// What happened, in words that never carry a token or a message's content.
    pub notes: Box<dyn FnMut(&str) + Send>,
}

struct Holding {
    /// Which win of the slot this is, so an ask made under an earlier win is
    /// not read as this one's.
    win: u64,
    board_id: i64,
    /// The board as last confirmed by read-back.
    board: TheBoard,
    /// An edit accepted by the service and not yet read back.
    pending: Option<TheBoard>,
    last_renewal: Duration,
    /// B12 and R1: when each entry entered this holder's board, on its clock.
    entered: HashMap<String, Duration>,
    told_unconfirmed: HashSet<String>,
    /// **`Telegram` R17 — whether this holding has its command list in
    /// place**: compared, and set where it differed. Tried once the board is
    /// confirmed, and again at each renewal until a try completes.
    commands_in_place: bool,
}

enum Standing {
    StoodDown,
    Holding(Box<Holding>),
}

/// One ask a holder makes, and which win it was made under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheAsk {
    win: u64,
    pub offset: Option<i64>,
    pub wait_seconds: u32,
}

/// **One role worker's slot and board.**
pub struct TheListener {
    client: TheClient,
    queue: Arc<TheSendQueue>,
    clock: Arc<dyn ReadsItsOwnClock>,
    me: WhoThisMachineIs,
    settings: TheSettings,
    store: Box<dyn TheTelegramWorkersState>,
    actor: Box<dyn ActsOnAnInstruction>,
    hands_on: Box<dyn HandsOn>,
    notes: Mutex<Notes>,
    standing: Standing,
    wins: u64,
    watch: TheStalenessWatch,
    /// **`Telegram` B14 — whether another holder has been seen alive**, kept
    /// apart from `watch`'s count, which a lost claim restarts.
    alive: TheProofOfLife,
    utterance: TheUtterance,
    /// The utterance last pinned, when, and for which holder.
    last_said: Option<(String, Duration, String)>,
    last_announced_to: Option<String>,
    told: HashSet<String>,
    /// The board the last completed read showed, while not holding. Added at
    /// piece 3, for the routing to read what is on the board.
    last_read: Option<TheBoard>,
    /// **When completed reads began to show nothing pinned**, on this
    /// machine's own clock, and `None` once a read shows anything. See
    /// [`THE_SETTLE_BEFORE_NOTHING_PINNED_IS_BELIEVED`].
    nothing_pinned_since: Option<Duration>,
    /// The pin update whose machinery message this holder has already
    /// unpinned, so a retry of that same update does not unpin it again.
    unpinned_for_update: Option<i64>,
    /// **`Telegram` R16 — this process's tap answerer**, holding every tap it
    /// has answered by the tap's identifier, so an update F3 brings back is not
    /// answered twice. Shared with [`run_until_stopped`]'s asker, which answers
    /// before it takes the turn's lock.
    taps: TheTapAnswerer,
    /// **`Telegram` B3 — what completed reads have shown of an unreadable
    /// board**, for its stale-board rule.
    unreadable: TheUnreadableBoardWatch,
    /// **The entries this holder's confirmed boards newly carried**, newest
    /// last, at most [`THE_ACCEPTED_KEPT`]: accepted by read-back (B8). D2's
    /// *Stop sent* and the routing's F3 read them.
    accepted: VecDeque<AnEntry>,
}

/// How many newly accepted entries the listener keeps for its readers.
const THE_ACCEPTED_KEPT: usize = 256;

impl TheListener {
    pub fn new(handed: WhatTheListenerIsHanded) -> TheListener {
        let taps = TheTapAnswerer {
            client: handed.client.clone(),
            conversation: handed.settings.conversation,
            answered: Arc::default(),
        };
        let utterance = TheUtterance {
            machine: handed.me.machine.clone(),
            display: handed.me.display.to_string(),
            colour: handed.me.display.colour().map(str::to_string),
            ..TheUtterance::default()
        };
        TheListener {
            client: handed.client,
            queue: handed.queue,
            clock: handed.clock,
            me: handed.me,
            settings: handed.settings,
            store: handed.store,
            actor: handed.actor,
            hands_on: handed.hands_on,
            notes: Mutex::new(handed.notes),
            standing: Standing::StoodDown,
            wins: 0,
            watch: TheStalenessWatch::new(),
            alive: TheProofOfLife::new(),
            utterance,
            last_said: None,
            last_announced_to: None,
            told: HashSet::new(),
            last_read: None,
            nothing_pinned_since: None,
            unpinned_for_update: None,
            taps,
            unreadable: TheUnreadableBoardWatch::default(),
            accepted: VecDeque::new(),
        }
    }

    pub fn is_holding(&self) -> bool {
        matches!(self.standing, Standing::Holding(_))
    }

    pub fn client(&self) -> TheClient {
        self.client.clone()
    }

    /// **`Telegram` B15 — propose an instruction.** A holder appends it to its
    /// own board at its next turn; any other machine pins it in its machinery
    /// message until it observes the identity on the board.
    pub fn propose(&mut self, entry: AnEntry) {
        self.utterance.proposed.push(entry);
    }

    /// **`Telegram` B8 and B15 — propose the full instruction that replaces a
    /// context-resolution entry under the same operation identity.**
    pub fn propose_a_replacement(&mut self, entry: AnEntry) {
        self.utterance.replacing.push(entry);
    }

    /// **`Telegram` R10, B10 and B11 — answer a gather** through this machine's
    /// machinery message. The reply names the state-request it answers under
    /// `operation`, and is kept while that entry is on the board; a holder takes
    /// its own reply in at its next turn. Added at step 10 piece 3: the
    /// machinery message already carried replies and nothing could put one in.
    pub fn reply_to_a_gather(&mut self, reply: Value) {
        if !self.utterance.replies.contains(&reply) {
            self.utterance.replies.push(reply);
        }
    }

    /// **The board as this machine last knew it**: its own confirmed board
    /// while holding, and otherwise the board its last completed read showed.
    /// Added at step 10 piece 3, so the routing can see what the board holds.
    pub fn the_board_as_last_read(&self) -> Option<TheBoard> {
        match &self.standing {
            Standing::Holding(h) => Some(h.board.clone()),
            Standing::StoodDown => self.last_read.clone(),
        }
    }

    /// **The entries this holder's confirmed boards newly carried, lately** —
    /// each accepted by exact read-back (B8), newest last. **`Telegram` D2's
    /// *Stop sent* is said from these**, the moment the full stop instruction is
    /// accepted, whichever machine owns the job; and the routing counts a
    /// proposal's work done from these as well as from the board (F3). Added by
    /// the round-1 code audit of the bot and the chat, fix-here items 1 and 3.
    pub fn the_instructions_accepted_lately(&self) -> Vec<AnEntry> {
        self.accepted.iter().cloned().collect()
    }

    fn accepted_one(&mut self, entry: AnEntry) {
        if self.accepted.len() == THE_ACCEPTED_KEPT {
            self.accepted.pop_front();
        }
        self.accepted.push_back(entry);
    }

    /// **`Telegram` R16 — this process's tap answerer**, to be used on the
    /// thread that asks, before the turn's lock is taken, so a tap is never
    /// left spinning behind a turn (fix-here item 4).
    pub fn the_tap_answerer(&self) -> TheTapAnswerer {
        self.taps.clone()
    }

    fn note(&self, what: &str) {
        let mut noted = lock(&self.notes);
        (*noted)(what);
    }

    // ------------------------------------------------------------ the turn

    /// **One turn, every [`THE_READ_CADENCE`] (B13).**
    pub fn a_turn(&mut self) {
        let now = self.clock.now();
        if self.is_holding() {
            self.a_holders_turn(now, None, false);
        } else {
            self.a_stood_down_turn(now);
        }
    }

    /// `Telegram` C6a: make room for a whole retry through the ordinary holder path.
    pub fn renew_before_a_flood_retry(&mut self) -> bool {
        if !self.is_holding() { return true; }
        self.a_holders_turn(self.clock.now(), None, true)
    }

    /// **B13 — the read is a `getChat`**, the only published route to the
    /// current pin. A read that did not complete gives back nothing.
    fn read_the_pin(&mut self) -> Option<WhatIsPinned> {
        let read = match self.client.get_chat(self.settings.conversation) {
            Ok(chat) => what_is_pinned(&chat),
            Err(_) => return None,
        };
        // B3's stale-board test, on B5's terms: completed reads only.
        self.unreadable.a_read_completed(self.clock.now(), &read);
        if matches!(read, WhatIsPinned::Nothing) {
            self.nothing_pinned_since.get_or_insert(self.clock.now());
        } else {
            self.nothing_pinned_since = None;
        }
        Some(read)
    }

    /// **Whether completed reads have shown nothing pinned for the whole
    /// [`THE_SETTLE_BEFORE_NOTHING_PINNED_IS_BELIEVED`].** A read that did not
    /// complete moves this neither way (B5's rule).
    fn nothing_pinned_is_believed(&self) -> bool {
        self.nothing_pinned_since
            .is_some_and(|since| self.clock.now().saturating_sub(since) >= THE_SETTLE_BEFORE_NOTHING_PINNED_IS_BELIEVED)
    }

    // ------------------------------------------------------------ not holding

    fn a_stood_down_turn(&mut self, now: Duration) {
        let Some(top) = self.read_the_pin() else {
            // B5: a read that did not complete counts for nothing either way.
            self.watch.a_read_did_not_complete();
            return;
        };
        let mut renewal = self.settings.renewal;
        match &top {
            WhatIsPinned::ABoard { id, board } => {
                self.watch
                    .a_read_completed(now, WhatACompletedReadSaw::AHeartbeat(board.heartbeat.clone()));
                // B14: only another machine's board is ever proof of life.
                if board.holder != self.me.machine {
                    self.alive.another_holders_board_was_read(now, *id, &board.heartbeat);
                }
                renewal = Duration::from_secs(board.renewal_seconds);
                let (id, board) = (*id, board.clone());
                self.last_read = Some(board.clone());
                self.what_the_board_asks_of_this_machine(now, id, &board);
            }
            _ => {
                self.last_read = None;
                self.watch.a_read_completed(now, WhatACompletedReadSaw::NoReadableBoard)
            }
        }
        if self.watch.may_ask(now, renewal) {
            self.claim(now);
        }
    }

    /// **B9, B12, B14 and B15, for a machine that is not holding.**
    ///
    /// **B14 as written 2026-10-18** (the Creator's fourteenth notes, note 2,
    /// with the two independent checks of that day folded in): **a board naming
    /// this machine as its holder is never proof of life, and nothing is pinned
    /// to it** — no announcement, acknowledgement, reply or proposal. Its
    /// entries for this machine are acted on once this machine holds again, on
    /// the holder's path (`keep_the_board`), and B9 makes acting twice
    /// harmless. That is a restarted holder's own old board, or one whose
    /// read-back failed (B3). **Otherwise only the announcement waits for proof
    /// that the holder is alive**: acknowledgements, replies, proposals and
    /// replacements are said as before, and only their re-pins wait (B15).
    fn what_the_board_asks_of_this_machine(&mut self, now: Duration, board_id: i64, board: &TheBoard) {
        if board.holder == self.me.machine {
            return;
        }
        let alive = self.alive.the_holder_is_alive(board_id, now, Duration::from_secs(board.renewal_seconds));
        for entry in self.the_entries_addressed_here(board) {
            match entry.kind {
                AKindOfEntry::Action => {
                    if !self.act_once(&entry) {
                        continue;
                    }
                    // B12: an instruction already acted on and still on the
                    // board is acknowledged again, harmlessly.
                    if !self.utterance.acknowledged.contains(&entry.operation) {
                        self.utterance.acknowledged.push(entry.operation.clone());
                    }
                }
                _ => {
                    // R10: the reply is said this turn, below.
                    if let Some(reply) = self.hands_on.an_entry_addressed_here(&entry) {
                        self.reply_to_a_gather(reply);
                    }
                }
            }
        }
        let on_the_board = |operation: &str| board.instructions.iter().find(|e| e.operation == operation);
        self.utterance.acknowledged.retain(|op| on_the_board(op).is_some());
        // B15: the proposer observes its identity on the board as the relay
        // acknowledgement, and from then on the board owns it.
        self.utterance.proposed.retain(|p| on_the_board(&p.operation).is_none());
        // B15: a replacement is done once the full instruction stands, and
        // abandoned when neither it nor its resolution entry does.
        self.utterance.replacing.retain(|r| {
            matches!(on_the_board(&r.operation), Some(e) if e.kind == AKindOfEntry::ContextResolution)
        });
        // R10 and B12: a reply is kept while the state-request it answers
        // stands (piece 3).
        self.utterance.replies.retain(|reply| {
            reply.get("operation").and_then(Value::as_str).is_some_and(|op| on_the_board(op).is_some())
        });
        // B14: an announcement, and a re-announcement, only to a holder seen
        // alive. Before 2026-10-18 a machine announced to whatever board it
        // read, and a fresh server under a dead server's board pinned an
        // announcement nobody would read: a loud notice on the phone.
        self.utterance.announce =
            alive && must_announce(board, &self.me.machine, self.last_announced_to.as_deref());
        self.say(now, &board.holder, alive);
    }

    fn the_entries_addressed_here(&self, board: &TheBoard) -> Vec<AnEntry> {
        let Ok(routing_key) = self.store.the_routing_key() else {
            return Vec::new();
        };
        let me = WhoIAm { machine: &self.me.machine, routing_key: &routing_key };
        board
            .instructions
            .iter()
            // **R1a, widened, and R1b**: a message this machine registered as
            // about a job, or its own status message, is its to claim too.
            .filter(|e| e.is_addressed_to(me, |m| self.store.speaks_for_the_message(m)))
            .cloned()
            .collect()
    }

    /// **B9 — act once, and acknowledge only an instruction whose outcome,
    /// success or failure, is recorded beside its identity** (`Server protocol
    /// and transfers` C4's shape; [`ActsOnAnInstruction::act_once`]). Where
    /// acting failed and nothing was recorded, the instruction is not
    /// acknowledged, so it is tried again at the next read. Gives back whether
    /// the instruction may be acknowledged.
    fn act_once(&mut self, entry: &AnEntry) -> bool {
        match self.actor.act_once(entry) {
            Ok(()) => {
                self.note(&format!("instruction {} has its outcome recorded", entry.operation));
                true
            }
            Err(why) => {
                self.note(&format!("instruction {} has no outcome recorded and will be tried again: {why}", entry.operation));
                false
            }
        }
    }

    /// **B10 — say what this machine must, through its one machinery message**:
    /// edit it (`Rented-server runtime` T10: its own), and pin it. The first time, it is sent and its
    /// identity recorded. **B19**: an unchanged utterance is pinned again only
    /// after [`TheSettings::say_again_after`].
    ///
    /// **B15 as written 2026-10-18: a repeat waits for proof that the holder
    /// is alive** (`holder_is_alive`, B14). An utterance already pinned — to
    /// this holder after the interval, or to a new holder — is pinned again
    /// only where that holder has been seen alive; the first pin of each
    /// utterance does not wait. A holder not seen alive is about to be
    /// replaced, and its successor takes the popped message in (B3).
    fn say(&mut self, now: Duration, holder: &str, holder_is_alive: bool) {
        if self.utterance.has_nothing_to_say() {
            return;
        }
        let Ok(text) = self.utterance.compose() else {
            self.note("this machine's machinery message would be over the bound and was not sent");
            return;
        };
        if let Some((said, at, to)) = &self.last_said {
            if *said == text
                && (!holder_is_alive || (to == holder && now.saturating_sub(*at) < self.settings.say_again_after))
            {
                return;
            }
        }
        let chat = self.settings.conversation;
        let pinned = match self.store.the_machinery_message() {
            Ok(Some(id)) => {
                let edited = text.clone();
                // An identical text is refused by the service and changes
                // nothing, so the pin is tried whatever the edit's answer.
                self.send(
                    AClassOfSend::RoutedAnswersAndAcknowledgements,
                    // B25: the spoiler is sent again with the edit.
                    Box::new(move |c: &TheClient| c.edit_a_machinery_message(chat, id, &edited)),
                );
                // B19: a pin of a message still pinned produces no update, and
                // the holder takes an utterance in from its pin update's copy.
                // Unpinned first, so this pin always makes an update carrying
                // the text just written; unpinning a message not pinned is
                // harmless.
                self.unpin(id);
                self.pin(id)
            }
            Ok(None) => false,
            Err(_) => return,
        };
        if !pinned {
            let first = text.clone();
            let HowASendEnded::Accepted(sent) = self.send(
                AClassOfSend::RoutedAnswersAndAcknowledgements,
                // Silent: what one machine tells another (B10, B21).
                Box::new(move |c: &TheClient| c.send_a_machinery_message(chat, &first)),
            ) else {
                return;
            };
            let Some(id) = sent.get("message_id").and_then(Value::as_i64) else {
                return;
            };
            if self.store.record_the_machinery_message(id).is_err() || !self.pin(id) {
                return;
            }
            self.note(&format!("sent machinery message {id}"));
        }
        self.last_said = Some((text, now, holder.to_string()));
        if self.utterance.announce {
            self.last_announced_to = Some(holder.to_string());
        }
    }

    // ------------------------------------------------------------ the claim

    /// **`Rented-server runtime` T4, and `Telegram` B6 and J3 — ask, and take the slot where the ask is won.**
    fn claim(&mut self, now: Duration) {
        let Ok(offset) = self.store.the_update_position() else {
            return;
        };
        match self.client.get_updates(offset, self.settings.the_claims_wait_seconds) {
            Ok(updates) => {
                // R16: before the claim's reads, unpins and board.
                self.answer_the_taps(&updates);
                self.take_the_slot(now, updates)
            }
            Err(WhyTheCallFailed::Refused(refusal))
                if refusal.error_code == Some(409)
                    && refusal.description.as_deref().is_some_and(|d| d.contains("webhook")) =>
            {
                self.remove_a_stale_registration(now, offset)
            }
            Err(WhyTheCallFailed::Refused(_)) => {
                self.watch.stand_down();
                self.note("the ask for the slot was refused; standing down (`Rented-server runtime` T4)");
            }
            Err(_) => self.note("the ask for the slot reached nothing"),
        }
    }

    /// **`Telegram` J3 — the stale-registration removal.** Reached only where
    /// B5's staleness held and the ask was refused in the webhook form. **The
    /// pair of readings around the removal decides, never the removal's own
    /// description**: a registration read before it, and an ask succeeding
    /// after. `deleteWebhook` never passes `drop_pending_updates` (J2).
    fn remove_a_stale_registration(&mut self, now: Duration, offset: Option<i64>) {
        let registered = self
            .client
            .get_webhook_info()
            .ok()
            .and_then(|info| info.get("url").and_then(Value::as_str).map(|url| !url.is_empty()))
            .unwrap_or(false);
        if !registered {
            self.watch.stand_down();
            return;
        }
        if let Err(WhyTheCallFailed::NotReached(_)) = self.client.delete_webhook() {
            return;
        }
        match self.client.get_updates(offset, self.settings.the_claims_wait_seconds) {
            Ok(updates) => {
                self.answer_the_taps(&updates);
                self.note("a stale webhook registration was removed (J3)");
                self.take_the_slot(now, updates);
            }
            Err(_) => self.watch.stand_down(),
        }
    }

    /// **`Telegram` B3 — take the slot, preserving every accepted instruction
    /// before the pin moves, and unpin the old board by its identity only after
    /// the new one is read back.**
    fn take_the_slot(&mut self, now: Duration, updates: Value) {
        let mut taken_in = Vec::new();
        let mut previous: Option<(i64, TheBoard)> = None;
        let mut reached_a_board = false;
        // B3's stale-board rule: the stale unreadable board, where one is met.
        let mut stale: Option<i64> = None;
        let mut unsettled = 0;
        for _ in 0..THE_MOST_POPS_IN_ONE_TURN {
            let Some(top) = self.read_the_pin() else {
                self.watch.stand_down();
                return;
            };
            let the_unreadable_board_is_stale = self.unreadable.is_stale(&top, self.clock.now(), self.settings.renewal);
            match what_the_claimant_does(&top, the_unreadable_board_is_stale) {
                WhatTheClaimantDoes::TakeInAndUnpin { id, utterance } => {
                    if !self.unpin(id) {
                        self.watch.stand_down();
                        return;
                    }
                    taken_in.push(utterance);
                }
                WhatTheClaimantDoes::Unpin { id } => {
                    if !self.unpin(id) {
                        self.watch.stand_down();
                        return;
                    }
                }
                WhatTheClaimantDoes::CopyIt { id, board } => {
                    previous = Some((id, board));
                    reached_a_board = true;
                    break;
                }
                WhatTheClaimantDoes::StartAFreshBoard => {
                    // B3: a fresh board is started only on a slot believed
                    // empty. A read showing nothing straight after an unpin
                    // may be the service's lag with the old board still
                    // pinned, and a fresh board would drop its accepted
                    // instructions. So it reads again. **It does not give the
                    // claim up**: its ask has already won, the old holder's
                    // is refused (`Rented-server runtime` T4), and a claim given up here leaves the
                    // slot with nobody — which the stand-in showed as a fleet
                    // that never settled. Past the bound it ends as any other
                    // unfinished claim does.
                    if !self.nothing_pinned_is_believed() {
                        unsettled += 1;
                        if unsettled > THE_MOST_UNSETTLED_READS_IN_ONE_TURN {
                            self.note("nothing was reported pinned, too briefly to believe; the claim did not finish (B3)");
                            self.watch.stand_down();
                            return;
                        }
                        // B3 as written 2026-10-18: the reads are spaced, so
                        // this turn's reads cover the whole settle.
                        self.clock.pause(THE_SPACING_OF_UNSETTLED_READS);
                        continue;
                    }
                    reached_a_board = true;
                    break;
                }
                WhatTheClaimantDoes::StartAFreshBoardOverAStaleOne { id } => {
                    // **B3: an unreadable board unchanged across a whole
                    // interval is stale.** Nothing on it can be copied, and a
                    // live holder of any version would have changed it.
                    stale = Some(id);
                    reached_a_board = true;
                    break;
                }
                WhatTheClaimantDoes::StandDownAndSayTheBoardIsUnreadable => {
                    self.board_unreadable(top.the_identity().unwrap_or_default());
                    return;
                }
            }
        }
        if !reached_a_board {
            self.watch.stand_down();
            return;
        }

        let Some(heartbeat) = a_fresh_heartbeat() else {
            self.watch.stand_down();
            return;
        };
        let moment = self.clock.the_moment();
        let claimant = TheClaimant {
            machine: &self.me.machine,
            display: &self.me.display,
            colour: self.me.display.colour(),
            heartbeat: &heartbeat,
            moment: &moment,
            renewal_seconds: self.settings.renewal.as_secs(),
        };
        let Ok((board, not_admitted)) = the_successor(previous.as_ref().map(|(_, b)| b), &claimant, &taken_in)
        else {
            self.board_unreadable(previous.as_ref().map(|(id, _)| *id).unwrap_or_default());
            return;
        };
        let Ok(text) = board.compose() else {
            self.watch.stand_down();
            return;
        };

        let chat = self.settings.conversation;
        let HowASendEnded::Accepted(sent) = self.send(
            AClassOfSend::RoutedAnswersAndAcknowledgements,
            // Silent: the board is what one machine tells another (B1, B21).
            Box::new(move |c: &TheClient| c.send_a_machinery_message(chat, &text)),
        ) else {
            self.watch.stand_down();
            return;
        };
        let Some(board_id) = sent.get("message_id").and_then(Value::as_i64) else {
            self.watch.stand_down();
            return;
        };
        self.note(&format!("sent board {board_id}"));
        let my_older_board = self.store.the_board().ok().flatten();
        if self.store.record_the_board(board_id).is_err() || !self.pin(board_id) {
            self.watch.stand_down();
            return;
        }
        match self.read_the_pin() {
            Some(WhatIsPinned::ABoard { id, board: read }) if id == board_id && read == board => {}
            _ => {
                // Not confirmed: the old board stays authoritative, untouched.
                self.watch.stand_down();
                self.note("the new board could not be read back; standing down (B3)");
                return;
            }
        }
        self.note(&format!("pinned board {board_id}"));
        // B3: only now, and only by identity.
        let previous_id = previous.as_ref().map(|(id, _)| *id);
        if let Some(id) = previous_id.filter(|id| *id != board_id) {
            self.unpin(id);
        }
        if let Some(id) = my_older_board.filter(|id| *id != board_id && Some(*id) != previous_id) {
            self.unpin(id);
        }
        // B3: the stale unreadable board, unpinned by its identity only now
        // that the fresh board is confirmed.
        if let Some(id) = stale.filter(|id| *id != board_id) {
            self.unpin(id);
        }
        // B8: what the popped machinery messages proposed is accepted now.
        let copied = |entry: &AnEntry| {
            previous.as_ref().is_some_and(|(_, old)| old.instructions.iter().any(|e| e.operation == entry.operation && e.kind == entry.kind))
        };
        let newly: Vec<AnEntry> = board.instructions.iter().filter(|e| !copied(e)).cloned().collect();
        for entry in newly {
            self.accepted_one(entry);
        }

        self.wins += 1;
        let entered = board.instructions.iter().map(|e| (e.operation.clone(), now)).collect();
        let the_first_renewal = now + how_often_the_holder_renews(Duration::from_secs(board.renewal_seconds));
        self.standing = Standing::Holding(Box::new(Holding {
            win: self.wins,
            board_id,
            board,
            pending: None,
            last_renewal: now,
            entered,
            told_unconfirmed: HashSet::new(),
            commands_in_place: false,
        }));
        // C6: no send in flight may run past the moment the heartbeat falls due.
        self.queue.the_heartbeat_falls_due(Some((the_first_renewal, self.clock.clone())));
        self.utterance.acknowledged.clear();
        self.utterance.announce = false;
        self.note(&format!("holding the slot with board {board_id}"));
        self.put_the_command_list_in_place();
        self.tell_what_was_not_admitted(not_admitted);
        if let Some(id) = stale {
            // B3: the user is told plainly the instructions could not be carried over.
            self.tell_once(
                format!("stale {id}"),
                a_user_message(&self.me.display, "", "", &format!(
                    "The pinned message was unreadable for {} seconds and was replaced. Requests sent in that time were lost.",
                    self.settings.renewal.as_secs()
                )),
            );
        }
        self.take_in_updates(&updates);
    }

    /// **`Telegram` R17 — the holder's command list, put in place**: read
    /// with `getMyCommands` (the default scope, no language code), and set
    /// with `setMyCommands` only where it differs. Nothing is said to the
    /// user either way: nothing routes by the list, and every request in it is
    /// still read when typed (R15). Only the holder calls this.
    fn put_the_command_list_in_place(&mut self) {
        let Standing::Holding(_) = &self.standing else { return };
        let in_place = match self.client.get_my_commands() {
            Ok(standing) if !crate::routing::typed::the_command_list_differs(&standing) => true,
            Ok(_) => self.client.set_my_commands(crate::routing::typed::the_command_list()).is_ok(),
            Err(_) => false,
        };
        self.note(&format!("the command list: in place = {in_place}"));
        if let Standing::Holding(h) = &mut self.standing {
            h.commands_in_place = in_place;
        }
    }

    fn board_unreadable(&mut self, id: i64) {
        self.watch.stand_down();
        self.note("the board could not be read; standing down (B3)");
        self.tell_once(
            format!("unreadable {id}"),
            a_user_message(&self.me.display, "", "", "Could not read the pinned message. No server is answering the bot."),
        );
    }

    fn stand_down(&mut self, why: &str) {
        self.standing = Standing::StoodDown;
        // C6: a machine not holding owes no heartbeat.
        self.queue.the_heartbeat_falls_due(None);
        self.watch.stand_down();
        self.note(&format!("standing down: {why}"));
    }

    // ------------------------------------------------------------ holding

    /// **B4 and B11 — the holder's turn.** Gives back whether the turn ended
    /// with the holder's own board on top and every change confirmed.
    ///
    /// **`carried` is a machinery message taken from a pin update's own copy**
    /// (B11: *on any pin update the holder processes what was pinned*; B19: the
    /// update carries the pinned message whole). It is unpinned by its identity
    /// and taken in before the reported pin is read, **because the reported pin
    /// is the most recent pinned message by sending date** (B13, quoting
    /// `ChatFullInfo.pinned_message`): a machinery message sent before this
    /// holder's board never shows on top, so a janitor that only popped the
    /// reported pin never heard it. That is why the first live run of
    /// 2026-09-13 gathered only the holder's own reply.
    ///
    /// **`carried` names its pin update**, so a retry of the same update — its
    /// board write not confirmed, F3 bringing it back — does not unpin the
    /// message again. An unpin may leave reads reporting nothing pinned for a
    /// while ([`THE_SETTLE_BEFORE_NOTHING_PINNED_IS_BELIEVED`]), and a retry
    /// that unpinned again could keep that going.
    fn a_holders_turn(&mut self, now: Duration, carried: Option<(i64, i64, TheUtterance)>, renew_early: bool) -> bool {
        let mut taken_in = Vec::new();
        if let Some((update, id, utterance)) = carried {
            if self.unpinned_for_update != Some(update) {
                if !self.unpin(id) {
                    return false;
                }
                self.unpinned_for_update = Some(update);
            }
            self.note(&format!("took in machinery message {id} from its pin update"));
            taken_in.push(utterance);
        }
        let Some(mut top) = self.read_the_pin() else {
            return false;
        };
        let mut pinned_the_board_again = false;
        let mut unsettled = 0;
        for _ in 0..THE_MOST_POPS_IN_ONE_TURN {
            let (board_id, board) = match &mut self.standing {
                Standing::StoodDown => return false,
                Standing::Holding(h) => {
                    if let WhatIsPinned::ABoard { id, board } = &top {
                        if *id == h.board_id && h.pending.as_ref() == Some(board) {
                            h.board = board.clone();
                            h.pending = None;
                        }
                    }
                    (h.board_id, h.board.clone())
                }
            };
            match what_the_janitor_does(&top, board_id, &board) {
                WhatTheJanitorDoes::Nothing => return self.keep_the_board(now, taken_in, renew_early),
                WhatTheJanitorDoes::StandDown => {
                    self.stand_down("another board is pinned over this machine's own (B4)");
                    return false;
                }
                WhatTheJanitorDoes::PinTheBoardAgain => {
                    // B11 re-pins the board *where it is absent*. Absent is
                    // believed only after reads have shown nothing pinned for
                    // the settle; until then the holder reads again, with no
                    // pin, up to the bound, and past it the turn ends
                    // unfinished and its update comes back (F3). Once believed,
                    // the board is pinned at most once a turn. Without this the
                    // holder pinned its own still-pinned board in bursts after
                    // each unpin — 33 times in the second live watch of
                    // 2026-09-14.
                    if !self.nothing_pinned_is_believed() {
                        unsettled += 1;
                        if unsettled > THE_MOST_UNSETTLED_READS_IN_ONE_TURN {
                            return false;
                        }
                        // B11 waits the same way as B3's claimant (2026-10-18).
                        self.clock.pause(THE_SPACING_OF_UNSETTLED_READS);
                    } else {
                        if pinned_the_board_again || !self.pin(board_id) {
                            return false;
                        }
                        pinned_the_board_again = true;
                    }
                }
                WhatTheJanitorDoes::Unpin { id } => {
                    if !self.unpin(id) {
                        return false;
                    }
                }
                WhatTheJanitorDoes::TakeInAndUnpin { id, utterance } => {
                    // Popped first, so the board's read-back can find the board
                    // on top. A death before the edit loses nothing: the
                    // proposer pins again until it sees its identity (B15), and
                    // an addressee acknowledges again (B12).
                    if !self.unpin(id) {
                        return false;
                    }
                    taken_in.push(utterance);
                }
            }
            top = match self.read_the_pin() {
                Some(read) => read,
                None => return false,
            };
        }
        false
    }

    /// **B8, B9, B11, B12 and B4 — everything the board owes this turn, as one
    /// candidate, written once and confirmed by read-back.**
    fn keep_the_board(&mut self, now: Duration, taken_in: Vec<TheUtterance>, renew_early: bool) -> bool {
        let moment = self.clock.the_moment();
        let Standing::Holding(h) = &self.standing else {
            return false;
        };
        let last_confirmed = h.board.clone();
        let mut candidate = h.board.clone();
        let mut not_admitted = Vec::new();
        let mut not_admitted_of_mine: Vec<AnEntry> = Vec::new();
        let mut completed: Vec<AnEntry> = Vec::new();
        let own = TheUtterance {
            machine: self.me.machine.clone(),
            display: self.me.display.to_string(),
            colour: self.me.display.colour().map(str::to_string),
            proposed: self.utterance.proposed.clone(),
            replacing: self.utterance.replacing.clone(),
            replies: self.utterance.replies.clone(),
            ..TheUtterance::default()
        };
        for (mine, utterance) in taken_in.iter().map(|u| (false, u)).chain(std::iter::once((true, &own))) {
            let asks = what_an_utterance_asks(&candidate, utterance, &moment);
            if let Some(changed) = asks.candidate {
                candidate = changed;
            }
            for (from, reply) in &asks.replies {
                self.hands_on.a_reply_to_a_gather(from, reply);
                // **B12 and R10 — a state-request is completed by its reply**,
                // so it leaves the board now, and the gather is answered as
                // soon as every server asked has replied. Until the round-1
                // code audit of the bot and the chat (fix-here item 17) it stood
                // until its deadline, and every gather waited ten seconds.
                let answers = reply.get("operation").and_then(Value::as_str);
                let the_request = candidate
                    .instructions
                    .iter()
                    .find(|e| {
                        Some(e.operation.as_str()) == answers
                            && e.kind == AKindOfEntry::StateRequest
                            && e.addressee == TheAddressee::Machine(from.clone())
                    })
                    .cloned();
                if let Some(entry) = the_request {
                    candidate = without(&candidate, &entry.operation);
                    completed.push(entry);
                }
            }
            for refused in asks.not_admitted {
                if let (true, NotAdmitted::AnInstruction(entry)) = (mine, &refused) {
                    not_admitted_of_mine.push((**entry).clone());
                }
                not_admitted.push(refused);
            }
        }
        // The holder's own replies are taken in above, and need no pin (piece 3).
        self.utterance.replies.clear();

        for entry in self.the_entries_addressed_here(&candidate) {
            match entry.kind {
                // **B8 — the holder is its own addressee, and acts only on an
                // instruction a read-back has accepted**: one on the board it
                // last confirmed. One taken in this turn is written first and
                // acted on at a later turn. Until the round-1 code audit (fix-here
                // item 3) the holder acted before writing, so no confirmed board
                // carried its own stop, and D2's *Stop sent* was never said on a
                // one-server fleet.
                AKindOfEntry::Action => {
                    let accepted = last_confirmed
                        .instructions
                        .iter()
                        .any(|e| e.operation == entry.operation && e.kind == AKindOfEntry::Action);
                    if accepted && self.act_once(&entry) {
                        candidate = without(&candidate, &entry.operation);
                    }
                }
                _ => {
                    // **B12 and R10 — the holder's own reply is taken in now**,
                    // and its own state-request completed in this same write.
                    if let Some(reply) = self.hands_on.an_entry_addressed_here(&entry) {
                        let me = self.me.machine.clone();
                        self.hands_on.a_reply_to_a_gather(&me, &reply);
                        if entry.kind == AKindOfEntry::StateRequest && entry.addressee == TheAddressee::Machine(me) {
                            candidate = without(&candidate, &entry.operation);
                            completed.push(entry.clone());
                        }
                    }
                }
            }
        }

        let Standing::Holding(h) = &mut self.standing else {
            return false;
        };
        let mut ended = Vec::new();
        let mut unconfirmed = Vec::new();
        for entry in &candidate.instructions {
            let entered = *h.entered.entry(entry.operation.clone()).or_insert(now);
            match what_the_deadline_owes(entry, entered, now) {
                Some(WhatADeadlineOwes::TellTheUserItIsUnconfirmed) => {
                    if h.told_unconfirmed.insert(entry.operation.clone()) {
                        unconfirmed.push(entry.clone());
                    }
                }
                Some(WhatADeadlineOwes::RemoveIt) => ended.push(entry.clone()),
                None => {}
            }
        }
        for entry in &ended {
            candidate = without(&candidate, &entry.operation);
            // **B14, 2026-10-31 — a roster machine that did not answer a
            // gather by its deadline is taken off in this same edit**, so the
            // gather that named it as not answering is the last that does; a
            // machine still alive re-announces, since the board no longer
            // lists it. The twenty-sixth notes, note 2.
            if let (AKindOfEntry::StateRequest, TheAddressee::Machine(machine)) = (&entry.kind, &entry.addressee) {
                if *machine != self.me.machine {
                    candidate = without_the_silent(&candidate, machine);
                }
            }
        }
        let renew = renew_early || now.saturating_sub(h.last_renewal)
            >= how_often_the_holder_renews(Duration::from_secs(candidate.renewal_seconds));
        if renew {
            if let Some(heartbeat) = a_fresh_heartbeat() {
                candidate.heartbeat = heartbeat;
                candidate.heartbeat_moment = moment;
            }
        }
        let changed = candidate != h.board;

        for entry in &unconfirmed {
            let TheAddressee::Machine(machine) = &entry.addressee else { continue };
            let server = the_display_of(&candidate, machine).unwrap_or("A server").to_string();
            // R7 and Rule 9: the button or request and the job, never *an
            // action* (fix-here item 13).
            self.tell(a_notice_about(entry, &AServerLabel::with_colour(&server, candidate.roster.iter().find(|r| r.machine == *machine).and_then(|r| r.colour.as_deref())), "was not confirmed. It may not have been done."));
        }
        self.tell_what_was_not_admitted(not_admitted);
        // **F3's third kind**: the truthful refusal is sent, so the work of the
        // update behind each refused proposal of this machine's own is done.
        // Until fix-here item 1 the routing never heard, and every later update
        // waited behind that one for good.
        for entry in &not_admitted_of_mine {
            self.hands_on.an_entry_was_not_admitted(entry);
        }
        let refused = |p: &AnEntry| not_admitted_of_mine.iter().any(|e| e.operation == p.operation);
        self.utterance.proposed.retain(|p| !refused(p));
        self.utterance.replacing.retain(|p| !refused(p));

        let confirmed = !changed
            || self.write_the_board(
                candidate.clone(),
                if renew { AClassOfSend::TheHoldersHeartbeat } else { AClassOfSend::RoutedAnswersAndAcknowledgements },
            );
        if confirmed {
            self.utterance.proposed.clear();
            self.utterance.replacing.clear();
            let mut try_the_commands = false;
            if let Standing::Holding(h) = &mut self.standing {
                if renew {
                    h.last_renewal = now;
                    // C6: the next renewal is when a send in flight must end.
                    let next = now + how_often_the_holder_renews(Duration::from_secs(candidate.renewal_seconds));
                    self.queue.the_heartbeat_falls_due(Some((next, self.clock.clone())));
                    try_the_commands = !h.commands_in_place;
                }
                h.entered.retain(|op, _| candidate.instructions.iter().any(|e| &e.operation == op));
            }
            if try_the_commands {
                // R17: a refused or unanswered try is tried again at a renewal.
                self.put_the_command_list_in_place();
            }
            // B8: what this read-back newly accepted.
            let newly: Vec<AnEntry> = candidate
                .instructions
                .iter()
                .filter(|e| !last_confirmed.instructions.iter().any(|o| o.operation == e.operation && o.kind == e.kind))
                .cloned()
                .collect();
            for entry in newly {
                self.accepted_one(entry);
            }
            for entry in &ended {
                self.hands_on.an_entry_ended_at_its_deadline(entry);
            }
            for entry in &completed {
                self.hands_on.an_entry_was_completed(entry);
            }
        }
        confirmed
    }

    /// **`Rented-server runtime` T12 — one complete edit of the board, accepted only after exact
    /// read-back.** A read-back showing another board is displacement (B4).
    fn write_the_board(&mut self, candidate: TheBoard, class: AClassOfSend) -> bool {
        let Standing::Holding(h) = &self.standing else {
            return false;
        };
        let board_id = h.board_id;
        let Ok(text) = candidate.compose() else {
            return false;
        };
        let chat = self.settings.conversation;
        // B25: the spoiler is sent again with every edit of the board. The
        // read-back below compares the text alone, never the marking.
        let edited = self.send(class, Box::new(move |c: &TheClient| c.edit_a_machinery_message(chat, board_id, &text)));
        if !matches!(edited, HowASendEnded::Accepted(_)) {
            self.note("a board edit was not accepted");
            return false;
        }
        if let Standing::Holding(h) = &mut self.standing {
            h.pending = Some(candidate.clone());
        }
        match self.read_the_pin() {
            Some(WhatIsPinned::ABoard { id, board }) if id == board_id && board == candidate => {
                if let Standing::Holding(h) = &mut self.standing {
                    h.board = candidate;
                    h.pending = None;
                }
                true
            }
            Some(WhatIsPinned::ABoard { id, .. }) if id != board_id => {
                self.stand_down("another board was read back (B4)");
                false
            }
            _ => false,
        }
    }

    // ------------------------------------------------------------ asking

    /// **`Rented-server runtime` T4 — a holder's ask, to be made off the turn**, or nothing where this
    /// machine is not holding.
    pub fn the_ask_to_make(&self) -> Option<TheAsk> {
        let Standing::Holding(h) = &self.standing else {
            return None;
        };
        let offset = self.store.the_update_position().ok()?;
        Some(TheAsk { win: h.win, offset, wait_seconds: self.settings.the_holders_wait_seconds })
    }

    /// **What a holder's ask brought.** A refusal for conflict is `Rented-server runtime` T4's: stand
    /// down rather than retry.
    pub fn what_the_ask_brought(&mut self, ask: TheAsk, got: WhatTheServiceGaveBack) {
        // R16: this process received these taps, whether or not it still holds.
        if let Ok(updates) = &got {
            self.answer_the_taps(updates);
        }
        match &self.standing {
            Standing::Holding(h) if h.win == ask.win => {}
            _ => return,
        }
        match got {
            Ok(updates) => self.take_in_updates(&updates),
            Err(WhyTheCallFailed::Refused(refusal)) if refusal.error_code == Some(409) => {
                self.stand_down("the ask was refused for conflict (`Rented-server runtime` T4)")
            }
            Err(_) => self.note("a holder's ask came back without updates"),
        }
    }

    /// **F3 — each update is confirmed only after its work is done.** A pin's
    /// work is the janitor's pass (B11); an update from the proved
    /// conversation's is whatever it is handed on to; one from any other
    /// conversation, or whose conversation cannot be established, is dismissed,
    /// and the decision is the work (P2, P3).
    fn take_in_updates(&mut self, updates: &Value) {
        let Some(updates) = updates.as_array() else { return };
        for update in updates {
            let Some(update_id) = update.get("update_id").and_then(Value::as_i64) else {
                continue;
            };
            let message = update.get("message");
            // P2 and P3 are piece 4's rule, wired in here at piece 3.
            let proved = TheConversation(self.settings.conversation);
            let done = if what_an_update_gets(update, proved) == WhatAnUpdateGets::Ignored {
                self.note(&format!("dismissed update {update_id}: not from the proved conversation"));
                true
            } else if let Some((message, pinned)) = message.and_then(|m| Some((m, m.get("pinned_message")?))) {
                let now = self.clock.now();
                let carried = match what_is_pinned(&serde_json::json!({ "pinned_message": pinned })) {
                    WhatIsPinned::AMachineryMessage { id, utterance } => Some((update_id, id, utterance)),
                    _ => None,
                };
                // B18: the pin's line goes only once the pin is processed, and
                // a line not reached comes back with its update (F3).
                self.a_holders_turn(now, carried, false) && self.the_pins_line_is_dealt_with(update_id, message)
            } else {
                match self.hands_on.an_update_from_the_conversation(update) {
                    Ok(()) => true,
                    Err(why) => {
                        self.note(&format!("update {update_id} was not handled: {why}"));
                        false
                    }
                }
            };
            if !done || self.store.record_the_update_position(update_id + 1).is_err() {
                return;
            }
            self.note(&format!("confirmed update {update_id}"));
        }
    }

    // ------------------------------------------------------------ taps

    /// **`Telegram` R16 — every tap from the proved conversation is answered at
    /// once, with no text, by this process, which received it.** Called the
    /// moment an ask comes back, before anything else is done with what it
    /// brought: before routing, before the claim's reads and board, and whether
    /// or not this machine still holds. The answer's result is ignored. **Never
    /// the owner's to do**: the owner may be another process, slow, or dead.
    ///
    /// **Once per tap in this process**, so an update F3 brings back is not
    /// answered again; Telegram's pages do not say a repeated answer works.
    /// **Not through the send queue**: an answer puts nothing in the
    /// conversation, and the queue's one slot may be held by an upload (C6b) or a
    /// flood wait (C6a), which would make *at once* untrue. **A tap from any
    /// other conversation, or whose conversation cannot be established, is not
    /// answered** (P2, P3): answering would tell a stranger the bot is alive.
    fn answer_the_taps(&mut self, updates: &Value) {
        for update_id in self.taps.answer_the_taps(updates) {
            self.note(&format!("answered the tap in update {update_id} (R16)"));
        }
    }

    // ------------------------------------------------------------ sends

    /// **Every send goes through the one queue** (`Telegram` C6). This waits for
    /// its own send to end, sending in its place where no other thread is.
    fn send(&self, class: AClassOfSend, send: ASend) -> HowASendEnded {
        let told = self.queue.offer(class, send);
        loop {
            match told.try_recv() {
                Ok(ended) => return ended,
                Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {
                    if self.queue.send_the_next().is_none() {
                        return told.recv().unwrap_or_else(|_| the_queue_ended());
                    }
                }
            }
        }
        the_queue_ended()
    }

    fn pin(&self, id: i64) -> bool {
        let chat = self.settings.conversation;
        let pinned = matches!(
            self.send(
                AClassOfSend::RoutedAnswersAndAcknowledgements,
                Box::new(move |c: &TheClient| c.pin_chat_message(chat, id))
            ),
            HowASendEnded::Accepted(_)
        );
        if pinned {
            self.note(&format!("pinned message {id}"));
        }
        pinned
    }

    /// **Always by identity** (B3, B11). A refusal — a message no longer pinned
    /// — still leaves it not pinned; only a send that reached nothing fails.
    fn unpin(&self, id: i64) -> bool {
        let chat = self.settings.conversation;
        !matches!(
            self.send(
                AClassOfSend::RoutedAnswersAndAcknowledgements,
                Box::new(move |c: &TheClient| c.unpin_chat_message(chat, id))
            ),
            HowASendEnded::HandedBack(WhyTheCallFailed::NotReached(_))
        )
    }

    /// **`Telegram` B18 — the line a processed pin wrote, deleted where the pin
    /// was Smoozy's own.** Gives back whether the update's work is done: a line
    /// that is not Smoozy's is left and done; a deleted line is done; **a
    /// refusal is done too, and the line stays** — Telegram refuses a line sent
    /// 48 hours or more before (`Sources` entry 87), and that is expected, not
    /// a fault. Only a call that came back with no answer leaves the work
    /// undone, so F3 brings the update back and the deletion is tried again.
    fn the_pins_line_is_dealt_with(&self, update_id: i64, message: &Value) -> bool {
        let Some(line) = the_line_a_pin_of_smoozys_wrote(message) else {
            return true;
        };
        let chat = self.settings.conversation;
        match self.send(
            AClassOfSend::RoutedAnswersAndAcknowledgements,
            Box::new(move |c: &TheClient| c.delete_a_pins_line(chat, line)),
        ) {
            HowASendEnded::Accepted(_) => {
                self.note(&format!("deleted pin line {line}, of update {update_id}"));
                true
            }
            HowASendEnded::HandedBack(WhyTheCallFailed::Refused(_)) => {
                self.note(&format!("pin line {line}, of update {update_id}, was refused deletion and stays (B18)"));
                true
            }
            _ => {
                self.note(&format!("pin line {line}, of update {update_id}, was not reached; the update comes back (F3)"));
                false
            }
        }
    }

    fn tell(&self, text: AMarkedText) {
        self.note("sent a message to the user");
        let chat = self.settings.conversation;
        self.send(
            AClassOfSend::RoutedAnswersAndAcknowledgements,
            Box::new(move |c: &TheClient| c.send_marked_message(chat, &text, None)),
        );
    }

    fn tell_once(&mut self, key: String, text: AMarkedText) {
        if self.told.insert(key) {
            self.tell(text);
        }
    }

    /// **B2 and B14 — one truthful message for what was not admitted.**
    fn tell_what_was_not_admitted(&mut self, not_admitted: Vec<NotAdmitted>) {
        for refused in not_admitted {
            match refused {
                // R7 and Rule 9: the button or request and the job, never an
                // internal action name (fix-here item 13).
                NotAdmitted::AnInstruction(entry) => self.tell_once(
                    format!("full {}", entry.operation),
                    a_notice_about(&entry, &self.me.display, "not accepted: the pinned message is full. Try again in a minute."),
                ),
                NotAdmitted::AMachine { machine, display } => self.tell_once(
                    format!("full machine {machine}"),
                    a_user_message(&display, "", "", "Not added: the pinned message is full. It won't answer /status until there is room."),
                ),
            }
        }
    }
}

fn the_queue_ended() -> HowASendEnded {
    HowASendEnded::HandedBack(WhyTheCallFailed::NotReached("the send queue ended".into()))
}

/// **`Telegram` B18 — the identity of the line a pin update's service message
/// is, where that pin was Smoozy's own**, and nothing otherwise. `message` is
/// the update's `message`, the service message itself (`Sources` entries 43
/// and 49: it carries its own identity, and the pinned message inside it).
///
/// **Smoozy's own pin** is told by three things, each of which alone keeps a
/// line:
/// - **the pinned message is the bot's own** — B18's own words;
/// - **it opens with the head**, which every message Smoozy pins does (B23)
///   and nothing else Smoozy sends does, since only the board and machinery
///   messages are pinned (B16) — so a bot's message the user pinned is not
///   taken for one of Smoozy's pins;
/// - **the service message is not a person's**: where it says who it is from
///   and that is not a bot, the user made the pin, and its line is never
///   touched.
///
/// **The identity given back is never the pinned message's**: a service
/// message naming itself as what was pinned gives back nothing.
pub fn the_line_a_pin_of_smoozys_wrote(message: &Value) -> Option<i64> {
    let pinned = message.get("pinned_message")?;
    let line = message.get("message_id").and_then(Value::as_i64)?;
    let the_bots_own = pinned.pointer("/from/is_bot").and_then(Value::as_bool) == Some(true);
    let opens_with_the_head = pinned
        .get("text")
        .and_then(Value::as_str)
        .is_some_and(|text| {
            text.starts_with(crate::board::text::THE_HEAD)
                || text.starts_with(crate::board::text::THE_HEAD_BEFORE_2026_10_28)
        });
    let a_persons_pin = message.pointer("/from/is_bot").and_then(Value::as_bool) == Some(false);
    let names_itself = pinned.get("message_id").and_then(Value::as_i64) == Some(line);
    (the_bots_own && opens_with_the_head && !a_persons_pin && !names_itself).then_some(line)
}

/// **B2 — a fresh opaque heartbeat value**: random bytes, compared for
/// equality only.
fn a_fresh_heartbeat() -> Option<String> {
    let mut bytes = [0u8; 8];
    getrandom::fill(&mut bytes).ok()?;
    Some(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

/// **`Telegram` B8 — a full action instruction to one machine**, with a fresh
/// operation identity (`State, identity and recovery` §2.2, Operation) and
/// B12's default reporting deadline.
pub fn an_action_for(machine: &str, action: &str, payload: Value) -> Option<AnEntry> {
    Some(AnEntry {
        version: THE_PROTOCOL_VERSION,
        operation: Identity::issue(IdentityKind::Operation).ok()?.as_text(),
        addressee: TheAddressee::Machine(machine.to_string()),
        kind: AKindOfEntry::Action,
        subjects: Vec::new(),
        action: action.to_string(),
        payload,
        source_update: None,
        accepted_at: String::new(),
        deadline_seconds: THE_DEFAULT_REPORTING_DEADLINE.as_secs(),
    })
}

/// **R7 and Rule 9 — what an entry was, in the user's own words**: the button
/// they tapped or the request they typed, and the job it was for, where the
/// entry carries them; never an internal action name. Added by the round-1
/// code audit of the bot and the chat, fix-here item 13. **The board's
/// *Pending* row uses it too** (`crate::board::text`), from step 13's words
/// sweep, where it had printed the internal action name.
fn a_notice_about(entry: &AnEntry, fallback: &AServerLabel, result: &str) -> AMarkedText {
    let labels = entry.payload.get("labels");
    let server = labels.and_then(|l| l.get("server")).cloned()
        .and_then(|s| serde_json::from_value::<AServerLabel>(s).ok()).unwrap_or_else(|| fallback.clone());
    let job = labels.and_then(|l| l.get("label")).and_then(Value::as_str).unwrap_or("");
    let mut no_job = entry.clone();
    if let Some(labels) = no_job.payload.get_mut("labels").and_then(Value::as_object_mut) { labels.remove("label"); }
    a_user_message(&server, job, "", &format!("{} {result}", in_the_users_words(&no_job)))
}

pub(crate) fn in_the_users_words(entry: &AnEntry) -> String {
    let labels = entry.payload.get("labels");
    let job = labels.and_then(|l| l.get("label")).and_then(Value::as_str).filter(|l| !l.is_empty());
    let frame = entry.payload.get("frame").and_then(Value::as_u64);
    let asked = match entry.kind {
        AKindOfEntry::StateRequest => Some(frame.map_or("/status".to_string(), |f| format!("/frame {f}"))),
        AKindOfEntry::ContextResolution => entry
            .payload
            .get("label")
            .and_then(Value::as_str)
            .map(str::to_string)
            .or_else(|| frame.map(|f| format!("/frame {f}"))),
        AKindOfEntry::Action => labels
            .and_then(|l| l.get("button"))
            .and_then(Value::as_str)
            .map(str::to_string)
            .or_else(|| frame.filter(|_| entry.action == SENDING_A_FRAME).map(|f| format!("/frame {f}"))),
    };
    let what = asked.map_or("an action".to_string(), |asked| format!("\u{201c}{asked}\u{201d}"));
    match job {
        Some(job) => format!("{what} for {job}"),
        None => what,
    }
}

/// **`Telegram` R16 — the answerer of one process's taps**: every tap from the
/// proved conversation answered once, with no text, by the process that
/// received it, the answer's result ignored. **It needs no turn's lock**, so
/// [`run_until_stopped`]'s asker answers the moment its ask comes back, before
/// it waits for the turn (fix-here item 4); the listener's own calls land on
/// the same record of what was answered.
#[derive(Clone)]
pub struct TheTapAnswerer {
    client: TheClient,
    conversation: i64,
    answered: Arc<Mutex<HashSet<String>>>,
}

impl TheTapAnswerer {
    /// Answers every tap in `updates` not answered before, and gives back the
    /// updates it answered. A tap from any other conversation, or one whose
    /// conversation cannot be established, is not answered (P2, P3).
    pub fn answer_the_taps(&self, updates: &Value) -> Vec<i64> {
        let Some(updates) = updates.as_array() else { return Vec::new() };
        let proved = TheConversation(self.conversation);
        let mut answered = Vec::new();
        for update in updates {
            let Some(id) = update.pointer("/callback_query/id").and_then(Value::as_str) else {
                continue;
            };
            if what_an_update_gets(update, proved) == WhatAnUpdateGets::Ignored || !lock(&self.answered).insert(id.to_string()) {
                continue;
            }
            let _ = self.client.answer_callback_query(id);
            answered.push(update.get("update_id").and_then(Value::as_i64).unwrap_or_default());
        }
        answered
    }
}

fn lock<T>(held: &Mutex<T>) -> MutexGuard<'_, T> {
    held.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// **Run a listener until `stop` is set** — the process a desk stands in for a
/// machine with. Four threads: the turn every [`THE_READ_CADENCE`], the
/// holder's ask, the send queue's one serial sender, and its one uploader
/// (`Telegram` C6b), so an upload never holds a serial send. `each_turn` runs
/// before every
/// turn. **The asker may be inside an open ask when this returns**; it ends when
/// that ask comes back or the process ends, and nothing it does then is
/// confirmed under a win that is over.
pub fn run_until_stopped(
    listener: Arc<Mutex<TheListener>>,
    queue: Arc<TheSendQueue>,
    stop: Arc<AtomicBool>,
    mut each_turn: impl FnMut(&mut TheListener),
) {
    let sender = {
        let (queue, stop) = (queue.clone(), stop.clone());
        std::thread::spawn(move || {
            while !stop.load(Ordering::SeqCst) {
                if queue.send_the_next().is_none() {
                    std::thread::sleep(Duration::from_millis(50));
                }
            }
        })
    };
    let uploader = {
        let (queue, stop) = (queue.clone(), stop.clone());
        std::thread::spawn(move || {
            while !stop.load(Ordering::SeqCst) {
                if !queue.send_the_next_upload() {
                    std::thread::sleep(Duration::from_millis(50));
                }
            }
        })
    };
    {
        let (listener, stop) = (listener.clone(), stop.clone());
        let answerer = lock(&listener).the_tap_answerer();
        std::thread::spawn(move || {
            while !stop.load(Ordering::SeqCst) {
                let ask = lock(&listener).the_ask_to_make();
                match ask {
                    None => std::thread::sleep(Duration::from_millis(500)),
                    Some(ask) => {
                        let client = lock(&listener).client();
                        let got = client.get_updates(ask.offset, ask.wait_seconds);
                        // **R16 — the taps are answered before the turn's lock
                        // is taken**, so a tap never spins behind a turn
                        // (fix-here item 4).
                        if let Ok(updates) = &got {
                            answerer.answer_the_taps(updates);
                        }
                        let brought = matches!(&got, Ok(updates) if updates.as_array().is_some_and(|u| !u.is_empty()));
                        let mut held = lock(&listener);
                        held.what_the_ask_brought(ask, got);
                        let confirmed_nothing = held.the_ask_to_make().is_some_and(|next| next.offset == ask.offset);
                        drop(held);
                        // An update whose work is not yet done stays unconfirmed
                        // and would come back at once; it is asked for again
                        // after one read, when its work may be done (F3, piece 3).
                        if brought && confirmed_nothing {
                            std::thread::sleep(THE_READ_CADENCE);
                        }
                    }
                }
            }
        });
    }
    while !stop.load(Ordering::SeqCst) {
        let started = Instant::now();
        {
            let mut held = lock(&listener);
            each_turn(&mut held);
            held.a_turn();
        }
        std::thread::sleep(THE_READ_CADENCE.saturating_sub(started.elapsed()));
    }
    let _ = sender.join();
    let _ = uploader.join();
}

#[cfg(test)]
mod tests {
    //! **A fleet on a simulated conversation.** The simulation keeps the
    //! observed ground of `Telegram` §3: the newest ask wins and an older open
    //! ask is refused for conflict; a pin is a stack and produces an update
    //! carrying the pinned message; a re-pin of a pinned message produces
    //! nothing (B19); `getChat` reports the most recent pinned message **by
    //! sending date** (B13), not the one pinned last.
    use super::*;
    use crate::board::instructions::THE_DEFAULT_RESOLUTION_DEADLINE;
    use crate::pacing::Waits;
    use crate::transport::{ReachesTelegram, WhatACallCarries};
    use crate::worker_state::InMemory;
    use serde_json::json;

    const CHAT: i64 = 4242;

    #[derive(Default)]
    struct TheConversation {
        next_message: i64,
        next_update: i64,
        texts: HashMap<i64, String>,
        pins: Vec<i64>,
        updates: Vec<Value>,
        confirmed_below: i64,
        open_ask: Option<String>,
        refused_next: HashSet<String>,
        webhook: String,
        cut: HashSet<String>,
        heard: Vec<(String, String, Value)>,
        /// How many reads after an unpin that took a pin away report nothing
        /// pinned — what the second live watch of 2026-09-14 most likely met.
        /// Zero is the old stand-in.
        blind_reads_after_an_unpin: usize,
        blind_for: usize,
        /// **The same lag measured on the machines' clock**: for this long
        /// after an unpin that took a pin away, every read reports nothing
        /// pinned. Added 2026-10-18 for the spaced reads (B3, B11): reads
        /// made back to back, with the clock standing still, never outlast
        /// it; reads spaced on the clock do.
        blind_seconds_after_an_unpin: Duration,
        blind_until: Duration,
        /// **How many reads after the next pin miss it**, reporting the pin
        /// beneath — Sources entry 86's older reading, as a claimant's
        /// read-back would meet it (B3). Taken by the next pin.
        the_next_pin_is_missed_by: usize,
        missed: Option<(i64, usize)>,
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
        /// **How long a file upload takes on the machines' clock.** Zero is
        /// instant. A call whose bound runs out first ends at its bound, not
        /// reached, as the real client does. **The upload waits for the clock
        /// to move rather than moving it**, so it can run on its own thread
        /// beside the rounds (C6b).
        an_upload_takes: Duration,
        uploads_cut: usize,
        uploads_arrived: usize,
        /// **`Telegram` B19 and B18 — every pin's line still in the
        /// conversation**, by the line's own identity: when it was written on
        /// the machines' clock, whether a person made the pin, what was
        /// pinned, and how many calls had been heard when it was written.
        lines: HashMap<i64, ALine>,
        /// Every pin's line ever written, deleted or not.
        written: HashMap<i64, ALine>,
        /// The lines a `deleteMessage` took away, in order.
        deleted: Vec<i64>,
        /// The lines a `deleteMessage` was refused for, in order.
        refused_deletions: Vec<i64>,
        /// The messages the user typed, whose sender is not a bot.
        from_the_user: HashSet<i64>,
        /// How many of the next `deleteMessage` calls reach nothing.
        the_next_deletions_reach_nothing: usize,
    }

    /// One pin's line.
    #[derive(Debug, Clone, Copy)]
    struct ALine {
        at: Duration,
        by_a_person: bool,
        pinned: i64,
        heard_at: usize,
    }

    /// **`Sources` entry 87: a message can be deleted only if it was sent less
    /// than 48 hours ago.** The stand-in refuses a line this old or older.
    const TWO_DAYS: Duration = Duration::from_secs(48 * 60 * 60);

    impl TheConversation {
        /// **A pin, as Telegram makes one**: a message going from unpinned to
        /// pinned writes a line and an update whose service message carries
        /// the line's own identity, who pinned, when, and the pinned message
        /// (B19; `Sources` entries 43 and 49). A pin of a message already
        /// pinned writes nothing.
        fn a_pin(&mut self, id: i64, now: Duration, by_a_person: bool) {
            if self.pins.contains(&id) {
                return;
            }
            let pinned = self.the_pinned_message(id);
            self.pins.push(id);
            self.next_update += 1;
            let update = self.next_update;
            let line = 50_000 + update;
            let written = ALine { at: now, by_a_person, pinned: id, heard_at: self.heard.len() };
            self.lines.insert(line, written);
            self.written.insert(line, written);
            self.updates.push(json!({ "update_id": update, "message": { "message_id": line, "chat": { "id": CHAT },
                "from": { "is_bot": !by_a_person }, "date": now.as_secs(), "pinned_message": pinned } }));
        }
        /// The lines still standing that a pin of the bot's own wrote.
        fn the_bots_lines_standing(&self) -> Vec<i64> {
            let mut standing: Vec<i64> = self.lines.iter().filter(|(_, l)| !l.by_a_person).map(|(id, _)| *id).collect();
            standing.sort();
            standing
        }
        /// How many calls of this method were heard.
        fn how_many(&self, method: &str) -> usize {
            self.heard.iter().filter(|(_, m, _)| m == method).count()
        }
        /// **`Telegram` R16, enforced by the stand-in: fails the test where a
        /// caller handed a tap from the proved conversation makes any other
        /// call before answering it, where an answer carries anything but the
        /// tap's identifier, or where a caller answers a tap it was not handed
        /// or answers one twice.** So a tap routed before it is answered, or
        /// answered by its owner rather than its receiver, fails.
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
            let from_a_bot = !self.from_the_user.contains(&id);
            let mut pinned = json!({ "message_id": id, "from": { "is_bot": from_a_bot }, "text": self.texts[&id] });
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
        fn a_message_from_the_bot(&self, text: &str) -> i64 {
            let mut s = self.state();
            s.next_message += 1;
            let id = s.next_message;
            s.texts.insert(id, text.to_string());
            id
        }
        /// A message the user typed in the conversation, which is not handed
        /// to anyone as an update (only its pin is, in these tests).
        fn a_message_from_the_user(&self, text: &str) -> i64 {
            let mut s = self.state();
            s.next_message += 1;
            let id = s.next_message;
            s.texts.insert(id, text.to_string());
            s.from_the_user.insert(id);
            id
        }
        /// **The user pins a message** — theirs or the bot's — at `now`.
        fn the_user_pins(&self, id: i64, now: Duration) {
            self.state().a_pin(id, now, true);
        }
        /// **A pin of the bot's own**, made at `now` by no machine of the
        /// test's — as one made by a machine that has since gone.
        fn the_bot_pins(&self, id: i64, now: Duration) {
            self.state().a_pin(id, now, false);
        }
        fn an_update(&self, chat: i64, text: &str) {
            let mut s = self.state();
            s.next_update += 1;
            let id = s.next_update;
            s.updates.push(json!({ "update_id": id, "message": { "message_id": 9000 + id, "chat": { "id": chat }, "text": text } }));
        }
        /// A tap on a button of message 1, in `chat`; gives back its update.
        fn the_user_taps(&self, chat: i64, callback: &str) -> i64 {
            let mut s = self.state();
            s.next_update += 1;
            let id = s.next_update;
            s.updates.push(json!({ "update_id": id, "callback_query": { "id": callback, "data": "d",
                "message": { "message_id": 1, "chat": { "id": chat } } } }));
            id
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
        clock: Arc<ItsClock>,
    }

    fn refused(code: i64, description: &str) -> Result<String, String> {
        Ok(json!({ "ok": false, "error_code": code, "description": description }).to_string())
    }
    fn accepted(result: Value) -> Result<String, String> {
        Ok(json!({ "ok": true, "result": result }).to_string())
    }

    impl ReachesTelegram for ACaller {
        fn call(&self, method: &str, carries: &WhatACallCarries, bound: Duration) -> Result<String, String> {
            let fields = match carries {
                WhatACallCarries::Fields(f) => Value::Object(f.clone()),
                _ => json!({}),
            };
            let mut s = self.sim.state();
            s.heard.push((self.name.clone(), method.to_string(), fields.clone()));
            s.the_taps_are_answered_first(&self.name, method, &fields);
            if method == "sendDocument" {
                let takes = s.an_upload_takes;
                drop(s);
                let ends = *lock(&self.clock.0) + takes.min(bound);
                let given = Instant::now();
                while *lock(&self.clock.0) < ends {
                    assert!(given.elapsed() < Duration::from_secs(30), "the machines' clock never reached the upload's end");
                    std::thread::sleep(Duration::from_millis(1));
                }
                let mut s = self.sim.state();
                if takes > bound {
                    s.uploads_cut += 1;
                    return Err("the call's bound passed".into());
                }
                s.uploads_arrived += 1;
                s.next_message += 1;
                return accepted(json!({ "message_id": s.next_message }));
            }
            if s.cut.contains(&self.name) {
                return Err("the line is cut".into());
            }
            let id = fields.get("message_id").and_then(Value::as_i64).unwrap_or(0);
            // `Telegram` B24: every pin and every board or machinery message is
            // silent; a message composed for the user is not.
            let silent = fields.get("disable_notification") == Some(&json!(true));
            match method {
                "pinChatMessage" => assert!(silent, "a pin was sent without disable_notification"),
                "sendMessage" if fields["text"].as_str().is_some_and(|t| t.starts_with(crate::board::text::THE_HEAD)) => {
                    assert!(silent, "a machinery message was sent with its notification")
                }
                "sendMessage" => assert!(fields.get("disable_notification").is_none(), "a message for the user was sent silently"),
                _ => {}
            }
            // `Telegram` B25: every board and machinery send or edit carries its
            // spoiler after the head, and nothing for the user carries one.
            crate::board::text::tests::the_spoiler_is_right(method, &fields);
            match method {
                "sendMessage" => {
                    s.next_message += 1;
                    let id = s.next_message;
                    let text = fields["text"].as_str().unwrap().to_string();
                    s.texts.insert(id, text);
                    s.keep_the_marking(id, &fields);
                    accepted(json!({ "message_id": id }))
                }
                "editMessageText" => {
                    if !s.texts.contains_key(&id) {
                        return refused(400, "Bad Request: message to edit not found");
                    }
                    s.texts.insert(id, fields["text"].as_str().unwrap().to_string());
                    s.keep_the_marking(id, &fields);
                    accepted(json!({ "message_id": id }))
                }
                "pinChatMessage" => {
                    if !s.texts.contains_key(&id) {
                        return refused(400, "Bad Request: message to pin not found");
                    }
                    let now = *lock(&self.clock.0);
                    if !s.pins.contains(&id) && s.the_next_pin_is_missed_by > 0 {
                        let reads = std::mem::take(&mut s.the_next_pin_is_missed_by);
                        s.missed = Some((id, reads));
                    }
                    s.a_pin(id, now, false);
                    accepted(json!(true))
                }
                // **`Telegram` B18, enforced by the stand-in**: the test fails
                // where a pinned message, any message that is not a pin's line,
                // or a line from a pin the user made is asked to be deleted. A
                // line written 48 hours or more before is refused and stays
                // (`Sources` entry 87).
                "deleteMessage" if s.the_next_deletions_reach_nothing > 0 => {
                    s.the_next_deletions_reach_nothing -= 1;
                    Err("the line is down".into())
                }
                "deleteMessage" => {
                    assert!(!s.pins.contains(&id), "B18: pinned message {id} was asked to be deleted");
                    assert!(!s.texts.contains_key(&id), "B18: message {id}, which is not a pin's line, was asked to be deleted");
                    let Some(line) = s.lines.get(&id).copied() else {
                        return refused(400, "Bad Request: message to delete not found");
                    };
                    assert!(!line.by_a_person, "B18: line {id}, of a pin the user made, was asked to be deleted");
                    if lock(&self.clock.0).saturating_sub(line.at) >= TWO_DAYS {
                        s.refused_deletions.push(id);
                        return refused(400, "Bad Request: message can't be deleted");
                    }
                    s.lines.remove(&id);
                    s.deleted.push(id);
                    accepted(json!(true))
                }
                "unpinChatMessage" => {
                    if s.pins.contains(&id) {
                        s.blind_for = s.blind_reads_after_an_unpin;
                        s.blind_until = *lock(&self.clock.0) + s.blind_seconds_after_an_unpin;
                    }
                    s.pins.retain(|p| *p != id);
                    accepted(json!(true))
                }
                "getChat" if s.blind_for > 0 => {
                    s.blind_for -= 1;
                    accepted(json!({ "id": CHAT }))
                }
                "getChat" if *lock(&self.clock.0) < s.blind_until => accepted(json!({ "id": CHAT })),
                "getChat" if s.missed.is_some_and(|(_, reads)| reads > 0) => {
                    let (hidden, reads) = s.missed.take().unwrap();
                    s.missed = Some((hidden, reads - 1));
                    match s.pins.iter().filter(|p| **p != hidden).max() {
                        Some(top) => accepted(json!({ "id": CHAT, "pinned_message": s.the_pinned_message(*top) })),
                        None => accepted(json!({ "id": CHAT })),
                    }
                }
                "getChat" => match s.pins.iter().max() {
                    Some(top) => accepted(json!({ "id": CHAT, "pinned_message": s.the_pinned_message(*top) })),
                    None => accepted(json!({ "id": CHAT })),
                },
                "getUpdates" => {
                    if !s.webhook.is_empty() {
                        return refused(409, "Conflict: can't use getUpdates method while webhook is active; use deleteWebhook to delete the webhook first");
                    }
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
                "getWebhookInfo" => accepted(json!({ "url": s.webhook })),
                "deleteWebhook" => {
                    s.webhook.clear();
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

    type Acted = Arc<Mutex<Vec<(String, String)>>>;

    /// **B9 as `ActsOnAnInstruction::act_once` states it**: the outcome recorded
    /// beside the identity, and a repeat acting on nothing. **An operation in
    /// `fails_once` fails its first try with nothing recorded**, as a command
    /// path that reached nothing would.
    struct RecordsActs {
        machine: String,
        recorded: HashSet<String>,
        acted: Acted,
        fails_once: Arc<Mutex<HashSet<String>>>,
        tries: Arc<Mutex<Vec<String>>>,
    }
    impl ActsOnAnInstruction for RecordsActs {
        fn act_once(&mut self, entry: &AnEntry) -> Result<(), String> {
            if self.recorded.contains(&entry.operation) {
                return Ok(());
            }
            lock(&self.tries).push(entry.operation.clone());
            if lock(&self.fails_once).remove(&entry.operation) {
                return Err("the act reached nothing, and nothing was recorded".into());
            }
            lock(&self.acted).push((self.machine.clone(), entry.operation.clone()));
            self.recorded.insert(entry.operation.clone());
            Ok(())
        }
    }

    /// Records what is handed on. **A tap is refused the first time**, so F3
    /// brings its update back, **and fails the test where the process handing
    /// it on has not already answered it** (R16).
    struct RecordsHandedOn {
        handed_on: Arc<Mutex<Vec<i64>>>,
        name: String,
        sim: Arc<Sim>,
        taps_refused_once: HashSet<i64>,
    }
    impl HandsOn for RecordsHandedOn {
        fn an_update_from_the_conversation(&mut self, update: &Value) -> Result<(), String> {
            let update_id = update["update_id"].as_i64().unwrap();
            if let Some(tap) = update.pointer("/callback_query/id").and_then(Value::as_str) {
                assert!(
                    self.sim.state().answered.iter().any(|(c, t)| *c == self.name && t == tap),
                    "R16: {} routed tap {tap} before answering it",
                    self.name
                );
                if self.taps_refused_once.insert(update_id) {
                    return Err("not yet".into());
                }
            }
            lock(&self.handed_on).push(update_id);
            Ok(())
        }
    }

    struct AMember {
        name: String,
        listener: TheListener,
        queue: Arc<TheSendQueue>,
        alive: bool,
    }

    struct AFleet {
        sim: Arc<Sim>,
        clock: Arc<ItsClock>,
        members: Vec<AMember>,
        acted: Acted,
        handed_on: Arc<Mutex<Vec<i64>>>,
        fails_once: Arc<Mutex<HashSet<String>>>,
        tries: Arc<Mutex<Vec<String>>>,
    }

    impl AFleet {
        fn new() -> AFleet {
            AFleet {
                sim: Arc::new(Sim::default()),
                clock: Arc::new(ItsClock(Mutex::new(Duration::ZERO))),
                members: Vec::new(),
                acted: Arc::default(),
                handed_on: Arc::default(),
                fails_once: Arc::default(),
                tries: Arc::default(),
            }
        }

        fn start(&mut self, name: &str) {
            let client = TheClient::over(Arc::new(ACaller { name: name.into(), sim: self.sim.clone(), clock: self.clock.clone() }));
            let queue = Arc::new(TheSendQueue::new(client.clone(), Arc::new(NoWaits)));
            let listener = TheListener::new(WhatTheListenerIsHanded {
                client,
                queue: queue.clone(),
                clock: self.clock.clone(),
                me: WhoThisMachineIs { machine: name.into(), display: format!("Server {name}").into() },
                settings: TheSettings::for_the_conversation(CHAT),
                store: Box::new(InMemory::with_routing_key(&format!("key-{name}"))),
                actor: Box::new(RecordsActs {
                    machine: name.into(),
                    recorded: HashSet::new(),
                    acted: self.acted.clone(),
                    fails_once: self.fails_once.clone(),
                    tries: self.tries.clone(),
                }),
                hands_on: Box::new(RecordsHandedOn {
                    handed_on: self.handed_on.clone(),
                    name: name.into(),
                    sim: self.sim.clone(),
                    taps_refused_once: HashSet::new(),
                }),
                notes: Box::new(|_| {}),
            });
            self.members.push(AMember { name: name.into(), listener, queue, alive: true });
        }

        fn member(&mut self, name: &str) -> &mut AMember {
            self.members.iter_mut().find(|m| m.name == name).unwrap()
        }

        /// One member's turn, then its ask, **with nothing it was handed left
        /// unanswered** (R16).
        fn a_turn_of(&mut self, name: &str) {
            let member = self.members.iter_mut().find(|m| m.name == name).unwrap();
            member.listener.a_turn();
            if let Some(ask) = member.listener.the_ask_to_make() {
                let got = member.listener.client().get_updates(ask.offset, ask.wait_seconds);
                member.listener.what_the_ask_brought(ask, got);
            }
            self.sim.state().nothing_is_owed_by(name);
        }

        /// One read cadence: every live member's turn, then its ask.
        fn a_round(&mut self) {
            let alive: Vec<String> = self.members.iter().filter(|m| m.alive).map(|m| m.name.clone()).collect();
            for name in alive {
                self.a_turn_of(&name);
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

        fn holders(&self) -> Vec<String> {
            self.members.iter().filter(|m| m.alive && m.listener.is_holding()).map(|m| m.name.clone()).collect()
        }
    }

    fn a_fleet_with_one_holder(names: &[&str]) -> (AFleet, String) {
        a_fleet_with_one_holder_reading(names, false)
    }

    /// The same, on a conversation whose reads drop the spoiler (B25) where
    /// `reads_drop_the_marking` is set, from before the first machine starts.
    fn a_fleet_with_one_holder_reading(names: &[&str], reads_drop_the_marking: bool) -> (AFleet, String) {
        let mut fleet = AFleet::new();
        fleet.sim.state().reads_drop_the_marking = reads_drop_the_marking;
        for name in names {
            fleet.start(name);
        }
        // **Since 2026-10-18 the machines that do not hold announce only once
        // they have seen the holder's first renewal** (B14), about fifty
        // seconds after it claims: the bound was 60 rounds.
        fleet
            .rounds_until(100, |f| {
                f.holders().len() == 1
                    && f.sim.the_board_on_top().is_some_and(|b| b.roster.len() == f.members.len())
            })
            .expect("one machine holds and every machine is on the roster");
        let holder = fleet.holders().remove(0);
        (fleet, holder)
    }

    /// **The milestone's second half, first watch: one holds, the rest stand
    /// down, and the heartbeat moves inside every stated interval.**
    #[test]
    fn one_machine_holds_and_the_rest_stand_down() {
        let (mut fleet, holder) = a_fleet_with_one_holder(&["a", "b", "c"]);
        let board = fleet.sim.the_board_on_top().unwrap();
        assert_eq!(board.holder, holder);
        for _ in 0..30 {
            fleet.a_round();
            assert_eq!(fleet.holders(), vec![holder.clone()]);
        }
        assert_ne!(fleet.sim.the_board_on_top().unwrap().heartbeat, board.heartbeat);
        // B11 and B20: the machinery pins were popped and the board is the one pin.
        assert_eq!(fleet.sim.state().pins.len(), 1);
    }

    /// **`Telegram` R16 and P2: a tap is answered by the process that received
    /// it, with nothing but its identifier, before it is handed on, and once**
    /// — though its update comes back (F3). A tap from another conversation is
    /// neither answered nor handed on. The stand-in fails the test on any call
    /// made before the answer; the hand-on fails it where the tap is routed
    /// first.
    #[test]
    fn a_tap_is_answered_by_the_process_that_received_it_before_it_is_handed_on() {
        let (mut fleet, holder) = a_fleet_with_one_holder(&["a", "b", "c"]);
        let here = fleet.sim.the_user_taps(CHAT, "tap-here");
        let elsewhere = fleet.sim.the_user_taps(999, "tap-elsewhere");
        fleet
            .rounds_until(10, |f| lock(&f.handed_on).contains(&here))
            .expect("the tap was handed on");
        for _ in 0..5 {
            fleet.a_round();
        }
        assert_eq!(fleet.sim.state().answered, [(holder, "tap-here".to_string())]);
        assert_eq!(lock(&fleet.handed_on).iter().filter(|u| **u == here).count(), 1);
        assert!(!lock(&fleet.handed_on).contains(&elsewhere), "P2: another conversation's tap was handed on");
        let answers = fleet.sim.state().heard.iter().filter(|(_, m, _)| m == "answerCallbackQuery").count();
        assert_eq!(answers, 1, "R16: a tap is answered once, and another conversation's never");
    }

    /// **`Telegram` C6 and C6b: a long upload runs whole in its own lane, and
    /// the holder's heartbeat is never deferred past the renewal interval.**
    /// Piece 7's finding, watched on the fleet: the holder uploads back to back,
    /// each upload taking 120 seconds on the machines' clock — past two of the
    /// holder's renewals — on its own thread, while every machine takes its
    /// turns and the others read the board (B13) and ask when its heartbeat has
    /// not changed for a full interval (B5). **Before the upload lane the slot
    /// moved in the first round**, and once the send queue cut the upload at the
    /// renewal the slot stayed but no upload ever arrived. Now the slot stays
    /// and every upload arrives, none cut.
    #[test]
    fn a_long_upload_runs_whole_beside_the_heartbeat_and_the_slot_stays_with_its_holder() {
        let (mut fleet, holder) = a_fleet_with_one_holder(&["a", "b", "c"]);
        fleet.sim.state().an_upload_takes = Duration::from_secs(120);
        let the_others: Vec<String> = fleet.members.iter().map(|m| m.name.clone()).filter(|n| *n != holder).collect();
        let queue = fleet.member(&holder).queue.clone();
        let mut uploading: Option<std::thread::JoinHandle<bool>> = None;
        let mut offered = 0;
        // **The machines' clock is moved by hand here, so none of it may be
        // spent on an upload that has not started.** The upload waits for the
        // clock rather than moving it, and its 120 seconds are counted from the
        // moment the send reaches the line — so every round taken between the
        // spawn and that moment is a round the upload does not get. A thread
        // just spawned can wait a long while for a core, and on a loaded
        // four-core runner that wait ran to most of the 150 rounds: one upload
        // was offered where three were meant to be, and the clause below — two
        // at least, one behind the other — read as broken when nothing was.
        // So the rounds do not begin until the send is in the stand-in's hands,
        // which makes the count below a count of the upload's own seconds
        // rather than a hope about real ones.
        let sends_begun = |f: &AFleet| f.sim.state().heard.iter().filter(|(_, m, _)| m == "sendDocument").count();
        // 150 reads: 300 seconds on the machines' clock, and an upload takes 60
        // of them, so three are offered and the first two run whole.
        for round in 0..150 {
            if uploading.as_ref().is_none_or(|u| u.is_finished()) {
                if let Some(ended) = uploading.take() {
                    assert!(ended.join().unwrap());
                }
                queue.offer_an_upload(Box::new(|c: &TheClient| {
                    c.send_document(CHAT, "what the render printed.txt", vec![0; 64], None, None)
                }));
                offered += 1;
                let queue = queue.clone();
                uploading = Some(std::thread::spawn(move || queue.send_the_next_upload()));
                let waited = Instant::now();
                while sends_begun(&fleet) < offered {
                    assert!(waited.elapsed() < Duration::from_secs(30), "the upload never reached the line");
                    std::thread::sleep(Duration::from_millis(1));
                }
            }
            for name in &the_others {
                fleet.a_turn_of(name);
            }
            fleet.a_turn_of(&holder);
            *lock(&fleet.clock.0) += THE_READ_CADENCE;
            assert_eq!(fleet.holders(), vec![holder.clone()], "C6: the slot moved from a live holder in round {round}");
        }
        let last = uploading.take().unwrap();
        while !last.is_finished() {
            *lock(&fleet.clock.0) += THE_READ_CADENCE;
            std::thread::sleep(Duration::from_millis(2));
        }
        assert!(last.join().unwrap());
        let s = fleet.sim.state();
        assert!(offered >= 2, "two uploads at least, one behind the other");
        assert_eq!(s.uploads_cut, 0, "C6b: an upload is never cut for the heartbeat");
        assert_eq!(s.uploads_arrived, offered, "C6b: every upload runs to its end");
    }

    /// **The rest of the milestone's second half**: the holder killed and the
    /// slot moving within one interval and a read; an instruction surviving the
    /// killing of its sender and of the holder, acknowledged once by its
    /// addressee; the pins popped back to the board; and the old board unpinned
    /// by its identity only after the new board was pinned.
    #[test]
    fn an_instruction_survives_its_sender_and_its_holder_and_is_acknowledged() {
        an_instruction_survives_its_sender_and_its_holder(false);
    }

    /// **`Telegram` B25: no reader relies on the spoiler.** The same watch —
    /// a hand-over, a proposal taken in from its pin update, the janitor's pops
    /// and every read-back — on a conversation whose pin updates and `getChat`
    /// hand back the text with no entity list.
    #[test]
    fn the_same_holds_when_reads_hand_back_no_spoiler() {
        an_instruction_survives_its_sender_and_its_holder(true);
    }

    fn an_instruction_survives_its_sender_and_its_holder(reads_drop_the_marking: bool) {
        let (mut fleet, holder) = a_fleet_with_one_holder_reading(&["a", "b", "c"], reads_drop_the_marking);
        let sender = ["a", "b", "c"].into_iter().find(|n| *n != holder).unwrap().to_string();
        let survivor = ["a", "b", "c"].into_iter().find(|n| *n != holder && *n != sender).unwrap().to_string();
        let entry = an_action_for("d", "a test instruction", json!({ "n": 1 })).unwrap();
        let operation = entry.operation.clone();
        fleet.member(&sender).listener.propose(entry);
        fleet
            .rounds_until(20, |f| f.sim.the_board_on_top().is_some_and(|b| b.instructions.iter().any(|e| e.operation == operation)))
            .expect("the proposal was accepted onto the board");
        let old_board_id = *fleet.sim.state().pins.iter().max().unwrap();

        fleet.member(&sender).alive = false;
        fleet.member(&holder).alive = false;
        let rounds = fleet
            .rounds_until(60, |f| f.holders() == vec![survivor.clone()])
            .expect("the survivor took the slot");
        assert!(rounds as u64 * 2 <= 60 + 2 * 4, "the slot moved after {} seconds", rounds * 2);
        let board = fleet.sim.the_board_on_top().unwrap();
        assert!(board.instructions.iter().any(|e| e.operation == operation), "the instruction survived");

        fleet.start("d");
        fleet
            .rounds_until(30, |f| {
                f.sim.the_board_on_top().is_some_and(|b| b.instructions.is_empty()) && f.sim.state().pins.len() == 1
            })
            .expect("acknowledged, trimmed, and the pins popped back to the board");
        for _ in 0..10 {
            fleet.a_round();
        }
        assert_eq!(*lock(&fleet.acted), vec![("d".to_string(), operation)], "acted on once");

        let s = fleet.sim.state();
        let unpins: Vec<usize> = s.heard.iter().enumerate()
            .filter(|(_, (_, m, f))| m == "unpinChatMessage" && f["message_id"] == json!(old_board_id))
            .map(|(at, _)| at).collect();
        let new_board_id = *s.pins.iter().max().unwrap();
        let pinned_new = s.heard.iter().position(|(_, m, f)| m == "pinChatMessage" && f["message_id"] == json!(new_board_id)).unwrap();
        assert!(!unpins.is_empty() && unpins.iter().all(|at| *at > pinned_new));
        assert!(s.heard.iter().all(|(_, m, f)| m != "unpinChatMessage" || f.get("message_id").is_some()));
        // B18: nothing deleted but pins' lines (the stand-in fails any other
        // deletion), and every line a pin of Smoozy's wrote is gone. B16 and
        // B23: everything pinned opens with the head.
        assert!(s.heard.iter().all(|(_, m, _)| !m.starts_with("delete") || m == "deleteMessage"));
        assert!(!s.deleted.is_empty());
        assert_eq!(s.the_bots_lines_standing(), Vec::<i64>::new(), "B18: every line Smoozy's pins wrote is deleted");
        for (_, m, f) in &s.heard {
            if m == "pinChatMessage" {
                let id = f["message_id"].as_i64().unwrap();
                assert!(s.texts[&id].starts_with("Smoozy \u{2014} server coordination"));
            }
        }
        // B25: the pin updates carried the spoiler exactly where the stand-in
        // was set to keep it, so the dropped run really read without it.
        let carried = s.updates.iter().filter(|u| u.pointer("/message/pinned_message/entities").is_some()).count();
        assert_eq!(carried > 0, !reads_drop_the_marking, "{carried} pin updates carried the spoiler");
    }

    /// **The brief's first thing, in a fleet: a machine whose reads do not
    /// complete never takes the slot from a live holder**, and never asks.
    #[test]
    fn a_machine_that_cannot_read_never_takes_the_slot() {
        let (mut fleet, holder) = a_fleet_with_one_holder(&["a", "b"]);
        let other = if holder == "a" { "b" } else { "a" };
        fleet.sim.state().cut.insert(other.to_string());
        for _ in 0..120 {
            fleet.a_round();
        }
        fleet.sim.state().cut.clear();
        let asked_before = fleet.sim.state().heard.iter().filter(|(n, m, _)| n == other && m == "getUpdates").count();
        for _ in 0..40 {
            fleet.a_round();
        }
        let asked_after = fleet.sim.state().heard.iter().filter(|(n, m, _)| n == other && m == "getUpdates").count();
        assert_eq!(fleet.holders(), vec![holder]);
        assert_eq!(asked_before, asked_after, "it never asked while the holder lived");
    }

    /// **B3, on a stand-in where reads lag after an unpin: a claimant that has
    /// just popped a machinery message and reads nothing pinned does not start
    /// a fresh board**, which would drop the accepted instruction still on the
    /// old one. Written 2026-09-14 from the second live watch's bursts.
    #[test]
    fn a_claimant_never_drops_the_board_on_a_read_that_lags_after_an_unpin() {
        let (mut fleet, holder) = a_fleet_with_one_holder(&["a", "b"]);
        let other = if holder == "a" { "b" } else { "a" }.to_string();
        let entry = an_action_for("gone", "a test instruction", json!({ "n": 1 })).unwrap();
        let operation = entry.operation.clone();
        fleet.member(&holder).listener.propose(entry);
        fleet
            .rounds_until(10, |f| f.sim.the_board_on_top().is_some_and(|b| b.instructions.iter().any(|e| e.operation == operation)))
            .expect("the proposal was accepted onto the board");
        fleet.member(&holder).alive = false;
        // A machinery message sent after the board and still pinned over it,
        // as one in flight is when its holder dies.
        let text = TheUtterance { machine: "late".into(), display: "Server late".into(), announce: true, ..TheUtterance::default() }
            .compose()
            .unwrap();
        let late = fleet.sim.a_message_from_the_bot(&text);
        {
            let mut s = fleet.sim.state();
            s.pins.push(late);
            s.blind_reads_after_an_unpin = 3;
        }
        fleet.rounds_until(150, |f| f.holders() == vec![other.clone()]).expect("the other machine took the slot");
        let board = fleet.sim.the_board_on_top().unwrap();
        assert_eq!(board.holder, other);
        assert!(
            board.instructions.iter().any(|e| e.operation == operation),
            "B3: the accepted instruction was carried to the new board"
        );
    }

    /// **B3: a malformed old board that is still changing — as a live holder's
    /// board of a later version would — is not replaced.** Its text changes
    /// every 40 seconds, inside the stated interval, so it is never stale.
    #[test]
    fn a_malformed_board_that_is_still_changing_is_not_replaced() {
        let mut fleet = AFleet::new();
        let broken = fleet.sim.a_message_from_the_bot("Smoozy \u{2014} server coordination\n\nsmoozy-board 1\n{broken");
        fleet.sim.state().pins.push(broken);
        fleet.start("a");
        for round in 0..100 {
            if round % 20 == 0 {
                fleet.sim.state().texts.insert(broken, format!("Smoozy \u{2014} server coordination\n\nsmoozy-board 1\n{{broken {round}"));
            }
            fleet.a_round();
        }
        assert!(fleet.holders().is_empty());
        let s = fleet.sim.state();
        assert_eq!(s.pins, vec![broken]);
        let told: Vec<&String> = s.heard.iter().filter(|(_, m, _)| m == "sendMessage")
            .map(|(_, _, f)| f["text"].as_str().unwrap()).map(|t| s.texts.values().find(|v| *v == t).unwrap()).collect();
        assert_eq!(told.len(), 1, "told once, and no board sent");
        assert!(told[0].contains("Could not read the pinned message."));
    }

    /// **B3's stale-board rule and fix-here item 9: a board from a later
    /// version whose holder is gone — its text unchanged for a whole interval —
    /// is replaced**: a fresh board, the old one unpinned by its identity after
    /// the fresh one is confirmed, and the user told once. Before the rule this
    /// machine stood down for good.
    #[test]
    fn a_board_from_a_later_version_that_stops_changing_is_replaced_and_the_user_told() {
        let mut fleet = AFleet::new();
        let later = fleet.sim.a_message_from_the_bot("Smoozy \u{2014} server coordination\n\nsmoozy-board 2\n{\"from a later version\":true}");
        fleet.sim.state().pins.push(later);
        fleet.start("a");
        fleet.rounds_until(80, |f| f.holders().len() == 1).expect("the stale board was replaced");
        for _ in 0..5 {
            fleet.a_round();
        }
        let s = fleet.sim.state();
        assert!(!s.pins.contains(&later), "B3: the stale board is no longer pinned");
        let pinned_new = s.heard.iter().position(|(_, m, _)| m == "pinChatMessage").unwrap();
        let unpinned_old = s.heard.iter().position(|(_, m, f)| m == "unpinChatMessage" && f["message_id"] == json!(later)).unwrap();
        assert!(unpinned_old > pinned_new, "unpinned by its identity only after the fresh board");
        assert_eq!(s.texts.values().filter(|t| t.contains("was unreadable for 60 seconds")).count(), 1);
        assert!(s.texts.values().all(|t| !t.contains("No server took over")));
    }

    /// **R16 and fix-here item 4: a tap is answered without the listener's
    /// turn lock**, which a turn doing slow work holds. The listener's own later
    /// look at the same updates answers nothing twice (the stand-in fails a
    /// second answer).
    #[test]
    fn a_tap_is_answered_without_waiting_for_the_listeners_turn() {
        let (mut fleet, holder) = a_fleet_with_one_holder(&["a"]);
        let answerer = fleet.member(&holder).listener.the_tap_answerer();
        let member = fleet.members.remove(0);
        let shared = Arc::new(Mutex::new(member.listener));
        let held = lock(&shared);
        let tap = fleet.sim.the_user_taps(CHAT, "tap-during-a-turn");
        let ask = held.the_ask_to_make().expect("still holding");
        let got = held.client().get_updates(ask.offset, 0);
        assert_eq!(answerer.answer_the_taps(got.as_ref().unwrap()), vec![tap], "answered while the turn's lock is held");
        assert!(fleet.sim.state().answered.contains(&(holder.clone(), "tap-during-a-turn".to_string())));
        drop(held);
        lock(&shared).what_the_ask_brought(ask, got);
        assert_eq!(fleet.sim.state().answered.len(), 1);
    }

    /// **B9, U6 and fix-here item 2: an act that fails before its outcome is
    /// recorded is tried again, and the instruction is acknowledged only once
    /// an outcome is recorded.** Before, the identity alone was recorded, so
    /// the next read acknowledged it with nothing done.
    #[test]
    fn an_act_that_fails_before_its_outcome_is_recorded_is_tried_again_and_only_then_acknowledged() {
        let (mut fleet, holder) = a_fleet_with_one_holder(&["a", "b"]);
        let other = if holder == "a" { "b" } else { "a" }.to_string();
        let entry = an_action_for(&other, "a test instruction", json!({ "n": 1 })).unwrap();
        let operation = entry.operation.clone();
        lock(&fleet.fails_once).insert(operation.clone());
        fleet.member(&holder).listener.propose(entry);
        fleet
            .rounds_until(40, |f| !lock(&f.acted).is_empty() && f.sim.the_board_on_top().is_some_and(|b| b.instructions.is_empty()))
            .expect("done, then acknowledged and trimmed");
        for _ in 0..5 {
            fleet.a_round();
        }
        assert_eq!(*lock(&fleet.tries), vec![operation.clone(), operation.clone()], "tried again after the failure");
        assert_eq!(*lock(&fleet.acted), vec![(other, operation)], "done once");
        assert!(fleet.sim.state().texts.values().all(|t| !t.contains("was not confirmed")));
    }

    /// **J3: a stale registration is removed, without dropping queued
    /// updates, and the slot is taken.**
    #[test]
    fn a_stale_registration_is_removed_and_the_slot_taken() {
        let mut fleet = AFleet::new();
        fleet.sim.state().webhook = "https://example.invalid/hook".into();
        fleet.start("a");
        fleet.rounds_until(40, |f| f.holders().len() == 1).expect("the slot was taken");
        let s = fleet.sim.state();
        let removal = s.heard.iter().find(|(_, m, _)| m == "deleteWebhook").unwrap();
        assert_eq!(removal.2, json!({}));
    }

    /// **B2: an instruction that does not fit is not accepted, nothing is
    /// evicted, and the user is told once.**
    #[test]
    fn an_instruction_that_does_not_fit_evicts_nothing() {
        let (mut fleet, holder) = a_fleet_with_one_holder(&["a"]);
        let small = an_action_for("gone", "stop", json!({})).unwrap();
        let operation = small.operation.clone();
        fleet.member(&holder).listener.propose(small);
        fleet.a_round();
        let big = an_action_for("gone", "a large one", json!("x".repeat(5000))).unwrap();
        fleet.member(&holder).listener.propose(big);
        for _ in 0..3 {
            fleet.a_round();
        }
        let board = fleet.sim.the_board_on_top().unwrap();
        assert_eq!(board.instructions.len(), 1);
        assert_eq!(board.instructions[0].operation, operation);
        let s = fleet.sim.state();
        assert_eq!(s.texts.values().filter(|t| t.contains("pinned message is full")).count(), 1);
    }

    /// **B12: an accepted action never expires and is reported once; a
    /// context-resolution entry leaves at its deadline; and a replacement that
    /// arrives after that is refused (B15).**
    #[test]
    fn deadlines_report_actions_and_remove_resolutions() {
        let (mut fleet, holder) = a_fleet_with_one_holder(&["a"]);
        let action = an_action_for("gone", "stop", json!({})).unwrap();
        let mut resolution = an_action_for("gone", "resolve", json!({})).unwrap();
        resolution.kind = AKindOfEntry::ContextResolution;
        resolution.addressee = TheAddressee::RoutingKey("key-of-a-machine-that-is-gone".into());
        resolution.deadline_seconds = THE_DEFAULT_RESOLUTION_DEADLINE.as_secs();
        let late = AnEntry { kind: AKindOfEntry::Action, addressee: TheAddressee::Machine("gone".into()), ..resolution.clone() };
        fleet.member(&holder).listener.propose(action.clone());
        fleet.member(&holder).listener.propose(resolution.clone());
        for _ in 0..60 {
            fleet.a_round();
        }
        let board = fleet.sim.the_board_on_top().unwrap();
        assert_eq!(board.instructions.iter().map(|e| e.operation.clone()).collect::<Vec<_>>(), vec![action.operation.clone()]);
        assert_eq!(fleet.sim.state().texts.values().filter(|t| t.contains("was not confirmed")).count(), 1);

        fleet.member(&holder).listener.propose_a_replacement(late);
        for _ in 0..3 {
            fleet.a_round();
        }
        assert_eq!(fleet.sim.the_board_on_top().unwrap().instructions.len(), 1, "the late replacement was refused");
    }

    /// **F3, P2 and P3: an update from another conversation is dismissed; one
    /// from the proved conversation is handed on; both are confirmed only
    /// after.**
    #[test]
    fn updates_are_handed_on_or_dismissed_and_then_confirmed() {
        let (mut fleet, _) = a_fleet_with_one_holder(&["a"]);
        fleet.sim.an_update(CHAT, "status");
        fleet.sim.an_update(999, "someone else");
        for _ in 0..3 {
            fleet.a_round();
        }
        let s = fleet.sim.state();
        let ours = s.updates.iter().find(|u| u["message"]["chat"]["id"] == json!(CHAT) && u["message"].get("text").is_some()).unwrap();
        assert_eq!(*lock(&fleet.handed_on), vec![ours["update_id"].as_i64().unwrap()]);
        assert_eq!(s.confirmed_below, s.next_update + 1);
    }

    /// **`Telegram` B18 — every line a pin of Smoozy's wrote is deleted once
    /// the pin is processed, by the line's identity, and nothing pinned is
    /// deleted.** Three machines settle, one proposes an instruction, it is
    /// accepted, and a fourth starts, acts and acknowledges — boards and
    /// machinery messages pinned again and again. **Every line goes**, the
    /// board's own (B21) among them. **Each machinery message's line goes only
    /// after the deleting machine unpinned that message** (B11's taking in),
    /// and each line is deleted once. The stand-in fails the test on any
    /// deletion of a pinned message or of anything that is not a pin's line.
    #[test]
    fn every_line_a_pin_of_smoozys_wrote_is_deleted_once_processed_and_nothing_pinned_is() {
        let (mut fleet, holder) = a_fleet_with_one_holder(&["a", "b", "c"]);
        let sender = ["a", "b", "c"].into_iter().find(|n| *n != holder).unwrap().to_string();
        let entry = an_action_for("d", "a test instruction", json!({ "n": 1 })).unwrap();
        fleet.member(&sender).listener.propose(entry);
        fleet.start("d");
        fleet
            .rounds_until(60, |f| {
                !lock(&f.acted).is_empty() && f.sim.the_board_on_top().is_some_and(|b| b.instructions.is_empty())
            })
            .expect("accepted, acted on and acknowledged");
        // **Since 2026-10-18 `d` announces only once it has seen the holder
        // renew** (B14), so its announcement's line is waited for too.
        fleet
            .rounds_until(40, |f| f.sim.the_board_on_top().is_some_and(|b| b.roster.iter().any(|r| r.machine == "d")))
            .expect("d announced and was listed");
        for _ in 0..10 {
            fleet.a_round();
        }
        let s = fleet.sim.state();
        assert_eq!(s.the_bots_lines_standing(), Vec::<i64>::new(), "B18: a line of Smoozy's pin still stands");
        let mut written: Vec<i64> = s.written.iter().filter(|(_, l)| !l.by_a_person).map(|(id, _)| *id).collect();
        written.sort();
        let mut deleted = s.deleted.clone();
        deleted.sort();
        assert_eq!(deleted, written, "B18: each line Smoozy's pins wrote is deleted, once");
        assert!(written.len() >= 6, "the watch wrote {} lines", written.len());
        assert!(s.refused_deletions.is_empty());
        for (at, (who, _, fields)) in s.heard.iter().enumerate().filter(|(_, (_, m, _))| m == "deleteMessage") {
            let id = fields["message_id"].as_i64().unwrap();
            let line = s.written[&id];
            assert!(!s.pins.contains(&id) && id != line.pinned, "B18: the line, never the pinned message");
            if let WhatIsPinned::AMachineryMessage { .. } = what_is_pinned(&json!({ "pinned_message": s.the_pinned_message(line.pinned) })) {
                let unpinned = s.heard[line.heard_at..at]
                    .iter()
                    .any(|(w, m, f)| w == who && m == "unpinChatMessage" && f["message_id"] == json!(line.pinned));
                assert!(unpinned, "B11 before B18: {who} deleted line {id} before it unpinned machinery message {}", line.pinned);
            }
        }
        assert_eq!(s.pins.len(), 1, "the board alone is pinned");
    }

    /// **`Telegram` B18 — a line from a pin the user made is never touched**,
    /// whether they pinned their own message or one of the bot's that opens
    /// with Smoozy's head. Both pins are processed and confirmed (F3), and
    /// Smoozy's own lines from the same stretch are deleted. The stand-in fails
    /// the test on any deletion of a line the user's pin wrote.
    #[test]
    fn a_line_from_a_pin_the_user_made_is_left() {
        let (mut fleet, _) = a_fleet_with_one_holder(&["a"]);
        let theirs = fleet.sim.a_message_from_the_user("remember the lights");
        let headed = TheUtterance { machine: "gone".into(), display: "Server gone".into(), announce: true, ..TheUtterance::default() }
            .compose()
            .unwrap();
        let the_bots = fleet.sim.a_message_from_the_bot(&headed);
        let now = fleet.clock.now();
        fleet.sim.the_user_pins(theirs, now);
        fleet.sim.the_user_pins(the_bots, now);
        let last = fleet.sim.state().next_update;
        fleet.rounds_until(20, |f| f.sim.state().confirmed_below > last).expect("both pins processed and confirmed");
        for _ in 0..5 {
            fleet.a_round();
        }
        let s = fleet.sim.state();
        let mut the_users: Vec<i64> = s.lines.iter().filter(|(_, l)| l.by_a_person).map(|(id, _)| *id).collect();
        the_users.sort();
        assert_eq!(the_users, vec![50_000 + last - 1, 50_000 + last], "B18: the lines of the user's pins stand");
        assert!(!s.deleted.is_empty());
        assert_eq!(s.the_bots_lines_standing(), Vec::<i64>::new(), "and Smoozy's own are gone");
    }

    /// **`Telegram` B18 and `Sources` entry 87 — a line the holder reaches 48
    /// hours or more after it was written is refused, stays, and is not asked
    /// for again**, and its update is done all the same (F3). A line reached
    /// at 47 hours is deleted. The stand-in is what refuses past two days.
    #[test]
    fn a_line_reached_past_two_days_is_refused_and_stays_and_its_update_is_done() {
        let (mut fleet, _) = a_fleet_with_one_holder(&["a"]);
        for _ in 0..5 {
            fleet.a_round();
        }
        let utterance = |machine: &str| {
            TheUtterance { machine: machine.into(), display: format!("Server {machine}"), announce: true, ..TheUtterance::default() }
                .compose()
                .unwrap()
        };
        let older = fleet.sim.a_message_from_the_bot(&utterance("older"));
        let newer = fleet.sim.a_message_from_the_bot(&utterance("newer"));
        let start = fleet.clock.now();
        fleet.sim.the_bot_pins(older, start);
        fleet.sim.the_bot_pins(newer, start + Duration::from_secs(2 * 60 * 60));
        let last = fleet.sim.state().next_update;
        let (older_line, newer_line) = (50_000 + last - 1, 50_000 + last);
        *lock(&fleet.clock.0) = start + Duration::from_secs(49 * 60 * 60);
        fleet.rounds_until(20, |f| f.sim.state().confirmed_below > last).expect("both pins processed and confirmed");
        for _ in 0..30 {
            fleet.a_round();
        }
        let s = fleet.sim.state();
        assert_eq!(s.refused_deletions, vec![older_line], "asked once, refused, and not asked again");
        assert!(s.deleted.contains(&newer_line), "a line 47 hours old is deleted");
        assert_eq!(s.the_bots_lines_standing(), vec![older_line], "the refused line stays");
    }

    /// **`Telegram` B18 and F3 — a deletion that reaches nothing leaves the
    /// pin's update undone**, so it comes back and the line is deleted at the
    /// next try. Taken as done, the line would stand for good.
    #[test]
    fn a_deletion_that_reaches_nothing_is_tried_again_with_its_update() {
        let (mut fleet, _) = a_fleet_with_one_holder(&["a"]);
        for _ in 0..5 {
            fleet.a_round();
        }
        let text = TheUtterance { machine: "late".into(), display: "Server late".into(), announce: true, ..TheUtterance::default() }
            .compose()
            .unwrap();
        let late = fleet.sim.a_message_from_the_bot(&text);
        let now = fleet.clock.now();
        {
            let mut s = fleet.sim.state();
            s.the_next_deletions_reach_nothing = 1;
            s.a_pin(late, now, false);
        }
        let line = 50_000 + fleet.sim.state().next_update;
        fleet.rounds_until(20, |f| f.sim.state().deleted.contains(&line)).expect("deleted at the next try");
        let s = fleet.sim.state();
        let asked = s.heard.iter().filter(|(_, m, f)| m == "deleteMessage" && f["message_id"] == json!(line)).count();
        assert_eq!(asked, 2, "once not reached, once deleted");
        assert_eq!(s.the_bots_lines_standing(), Vec::<i64>::new());
    }

    /// **`Telegram` B20 and B18 — at rest no pin happens and no line stands**,
    /// across two hours of idle reads by three machines: the heartbeat is an
    /// edit and the reads are reads, so nothing is written, and every line
    /// written while they settled is already gone.
    #[test]
    fn at_rest_no_pin_happens_and_no_line_stands_across_a_long_idle_stretch() {
        let (mut fleet, _) = a_fleet_with_one_holder(&["a", "b", "c"]);
        for _ in 0..10 {
            fleet.a_round();
        }
        let (pins, deletions) = {
            let s = fleet.sim.state();
            assert_eq!(s.the_bots_lines_standing(), Vec::<i64>::new(), "settled, and no line of Smoozy's stands");
            (s.how_many("pinChatMessage"), s.how_many("deleteMessage"))
        };
        for _ in 0..3600 {
            fleet.a_round();
        }
        let s = fleet.sim.state();
        assert_eq!(s.how_many("pinChatMessage"), pins, "B20: no pin at rest");
        assert_eq!(s.how_many("deleteMessage"), deletions, "nothing to delete at rest");
        assert_eq!(s.the_bots_lines_standing(), Vec::<i64>::new());
        assert_eq!(s.pins.len(), 1, "the board alone is pinned");
    }

    // ------------------------------------------------ one pin per server

    /// **A board left pinned by a holder that has gone** — its heartbeat never
    /// changes again, as the Creator's deleted test server left one
    /// (the fourteenth notes, note 2). Pinned straight onto the stack, as a
    /// board settled long ago is, with its line long gone.
    fn a_dead_board(fleet: &AFleet, holder: &str, roster: &[&str]) -> i64 {
        let mut board = crate::board::text::tests::a_board(holder);
        board.roster = roster
            .iter()
            .map(|m| crate::board::roster::ARosterEntry { colour: None, machine: m.to_string(), display: format!("Server {m}") })
            .collect();
        let id = fleet.sim.a_message_from_the_bot(&board.compose().unwrap());
        fleet.sim.state().pins.push(id);
        id
    }

    /// The messages one caller pinned, in order, from the call numbered
    /// `since` on.
    fn pinned_by(fleet: &AFleet, name: &str, since: usize) -> Vec<i64> {
        fleet.sim.state().heard[since..]
            .iter()
            .filter(|(n, m, _)| n == name && m == "pinChatMessage")
            .map(|(_, _, f)| f["message_id"].as_i64().unwrap())
            .collect()
    }

    /// The machinery messages one caller sent — its announcements and every
    /// other utterance, never a board — from the call numbered `since` on.
    fn machinery_sent_by(fleet: &AFleet, name: &str, since: usize) -> Vec<String> {
        fleet.sim.state().heard[since..]
            .iter()
            .filter(|(n, m, f)| {
                n == name && m == "sendMessage" && f["text"].as_str().is_some_and(|t| t.contains("smoozy-machinery"))
            })
            .map(|(_, _, f)| f["text"].as_str().unwrap().to_string())
            .collect()
    }

    fn heard_so_far(fleet: &AFleet) -> usize {
        fleet.sim.state().heard.len()
    }

    fn listed(board: &TheBoard, machine: &str) -> bool {
        board.roster.iter().any(|r| r.machine == machine)
    }

    /// **D1, the Creator's case (the fourteenth notes, note 2): a lone server
    /// under a dead server's board makes exactly one `pinChatMessage`, its own
    /// board's, and never announces.** It holds, its own board lists it (B3,
    /// the claimant lists itself), and nothing more is pinned across three
    /// renewal intervals. Before, it announced to the dead board first: two
    /// loud notices at setup, and the repeating path besides.
    #[test]
    fn a_lone_server_under_a_dead_board_pins_once_its_own_board_and_never_announces() {
        let mut fleet = AFleet::new();
        let dead = a_dead_board(&fleet, "gone", &["gone"]);
        fleet.start("a");
        fleet.rounds_until(60, |f| f.holders() == ["a"]).expect("it takes the slot under B5");
        for _ in 0..90 {
            fleet.a_round();
        }
        let board_id = *fleet.sim.state().pins.iter().max().unwrap();
        assert_eq!(pinned_by(&fleet, "a", 0), vec![board_id], "one pin, its own board's");
        assert!(machinery_sent_by(&fleet, "a", 0).is_empty(), "B14: no announcement to a holder never seen alive");
        let board = fleet.sim.the_board_on_top().unwrap();
        assert_eq!(board.holder, "a");
        assert!(listed(&board, "a"), "B3: the claimant lists itself");
        assert!(!fleet.sim.state().pins.contains(&dead), "the dead board unpinned by its identity");
    }

    /// **D2: the same with the claim's read-back missing the new board once**
    /// (Sources entry 86's older reading). The claimant stands down with its
    /// own board pinned and nobody holding; **that board names it, so it is
    /// never proof of life and nothing is pinned to it** (B14, Opus's case 1).
    /// It ends up holding, with no announcement to itself.
    #[test]
    fn a_read_back_that_misses_its_board_draws_no_announcement_to_itself() {
        let mut fleet = AFleet::new();
        a_dead_board(&fleet, "gone", &["gone"]);
        fleet.sim.state().the_next_pin_is_missed_by = 1;
        fleet.start("a");
        fleet.rounds_until(120, |f| f.holders() == ["a"]).expect("it holds in the end");
        for _ in 0..90 {
            fleet.a_round();
        }
        assert!(machinery_sent_by(&fleet, "a", 0).is_empty(), "nothing pinned to its own board");
        let pins = pinned_by(&fleet, "a", 0);
        assert_eq!(pins.len(), 2, "its two boards, and nothing else: {pins:?}");
        let s = fleet.sim.state();
        assert!(pins.iter().all(|id| s.texts[id].contains("smoozy-board")));
        drop(s);
        assert!(listed(&fleet.sim.the_board_on_top().unwrap(), "a"));
    }

    /// **D3: the holder renews once, a new server sees it and announces, and
    /// the holder dies before taking the announcement in.** The newcomer pins
    /// its announcement once, **never again after the interval** (B15: a
    /// repeat waits for proof of life, which has expired), and its claim pops
    /// it and succeeds despite the lag after the unpin (B3's spaced reads). Two
    /// pins in all. Run with a lag of three reads, which the old back-to-back
    /// reads already outlasted, and with a lag of three seconds, which they
    /// never could — the repeating path the checks traced. **And once with the
    /// dying holder unpinning the announcement before it could write it in**
    /// (B11: popped first; *"a death before the edit loses nothing"*), so the
    /// dead board is on top again when the interval ends: that is where the old
    /// rule pinned the same announcement a second time.
    #[test]
    fn a_holder_that_dies_after_a_newcomer_saw_it_alive_costs_one_announcement_and_no_repeat() {
        for (reads, seconds, popped) in [(3, 0, false), (0, 3, false), (0, 0, true)] {
            let (mut fleet, holder) = a_fleet_with_one_holder(&["a"]);
            assert_eq!(holder, "a");
            {
                let mut s = fleet.sim.state();
                s.blind_reads_after_an_unpin = reads;
                s.blind_seconds_after_an_unpin = Duration::from_secs(seconds);
            }
            let since = heard_so_far(&fleet);
            fleet.start("b");
            fleet
                .rounds_until(40, |f| !pinned_by(f, "b", since).is_empty())
                .expect("b saw a renewal and announced");
            fleet.member("a").alive = false;
            let board = fleet.sim.the_board_on_top();
            assert!(board.is_none(), "b's announcement is on top, not taken in");
            if popped {
                let announcement = *pinned_by(&fleet, "b", since).last().unwrap();
                fleet.sim.state().pins.retain(|p| *p != announcement);
                assert!(fleet.sim.the_board_on_top().is_some_and(|b| b.holder == "a"));
            }
            fleet.rounds_until(60, |f| f.holders() == ["b"]).expect("b took the slot");
            for _ in 0..90 {
                fleet.a_round();
            }
            let pins = pinned_by(&fleet, "b", since);
            assert_eq!(
                pins.len(),
                2,
                "lag {reads} reads, {seconds} s, popped {popped}: the announcement once and its board once: {pins:?}"
            );
            assert_eq!(machinery_sent_by(&fleet, "b", since).len(), 1);
            let board = fleet.sim.the_board_on_top().unwrap();
            assert_eq!(board.holder, "b");
            assert!(listed(&board, "b"));
        }
    }

    /// **B15 as written 2026-10-18 — a repeat of any utterance waits for proof
    /// that the holder is alive, not only an announcement's.** The holder
    /// renews and boards an instruction for `b` in one edit; `b` sees both in
    /// one read, acts, and pins its acknowledgement. The holder unpins it to
    /// take it in and dies before writing, so the dead board, the instruction
    /// still on it, is on top again. A full interval after that renewal the
    /// proof has expired: `b` does not pin the same acknowledgement again, and
    /// takes the slot instead — its acknowledgement once and its board once.
    /// The old rule pinned it a second time at that moment, to a holder that
    /// was gone.
    #[test]
    fn a_repeated_acknowledgement_waits_for_proof_that_the_holder_is_alive() {
        let (mut fleet, holder) = a_fleet_with_one_holder(&["a", "b"]);
        assert_eq!(holder, "a", "the first to claim holds");
        let heartbeat = |f: &AFleet| f.sim.the_board_on_top().map(|b| b.heartbeat);
        let before = heartbeat(&fleet);
        fleet.rounds_until(30, |f| heartbeat(f).is_some_and(|h| Some(h) != before)).expect("a renewed");
        for _ in 0..24 {
            fleet.a_round();
        }
        let renewed = heartbeat(&fleet);
        let entry = an_action_for("b", "a test instruction", json!({ "n": 3 })).unwrap();
        let operation = entry.operation.clone();
        fleet.member("a").listener.propose(entry);
        let since = heard_so_far(&fleet);
        fleet.a_round();
        let acknowledgement = *pinned_by(&fleet, "b", since).last().expect("b acknowledged in the same round");
        assert_eq!(*lock(&fleet.acted), vec![("b".to_string(), operation.clone())]);
        fleet.member("a").alive = false;
        fleet.sim.state().pins.retain(|p| *p != acknowledgement);
        let board = fleet.sim.the_board_on_top().expect("the dead board is on top again");
        assert_ne!(Some(board.heartbeat), renewed, "the instruction came with a renewal");
        assert!(board.instructions.iter().any(|e| e.operation == operation));
        fleet.rounds_until(40, |f| f.holders() == ["b"]).expect("b took the slot");
        for _ in 0..60 {
            fleet.a_round();
        }
        let pins = pinned_by(&fleet, "b", since);
        assert_eq!(pins.len(), 2, "its acknowledgement once and its board once: {pins:?}");
        assert_eq!(*lock(&fleet.acted), vec![("b".to_string(), operation)], "acted on once");
        assert!(fleet.sim.the_board_on_top().unwrap().instructions.is_empty());
    }

    /// **D4: three servers start together under a dead board.** One holds,
    /// none announces before the winner's first renewal (a new board is no
    /// change, B14), and all three are listed after it.
    #[test]
    fn three_servers_under_a_dead_board_settle_on_one_holder_and_all_are_listed() {
        let mut fleet = AFleet::new();
        a_dead_board(&fleet, "gone", &["gone"]);
        for name in ["a", "b", "c"] {
            fleet.start(name);
        }
        fleet.rounds_until(60, |f| f.holders().len() == 1).expect("one holds");
        for name in ["a", "b", "c"] {
            assert!(machinery_sent_by(&fleet, name, 0).is_empty(), "{name} announced in the claim's own round");
        }
        let holder = fleet.holders().remove(0);
        let board_id = *fleet.sim.state().pins.iter().max().unwrap();
        // The holder's board as last edited, whatever is pinned over it.
        let its_board = |f: &AFleet| {
            let s = f.sim.state();
            match what_is_pinned(&json!({ "pinned_message": s.the_pinned_message(board_id) })) {
                WhatIsPinned::ABoard { board, .. } => board,
                _ => unreachable!("the holder's board reads"),
            }
        };
        let first = its_board(&fleet).heartbeat;
        let since = heard_so_far(&fleet);
        let mut renewed = false;
        for _ in 0..60 {
            fleet.a_round();
            if its_board(&fleet).heartbeat != first {
                renewed = true;
            }
            if !renewed {
                for name in ["a", "b", "c"] {
                    assert!(machinery_sent_by(&fleet, name, since).is_empty(), "{name} announced before the first renewal");
                }
            }
        }
        assert!(renewed);
        assert_eq!(fleet.holders(), vec![holder]);
        let board = fleet.sim.the_board_on_top().unwrap();
        for name in ["a", "b", "c"] {
            assert!(listed(&board, name), "{name} is listed: {:?}", board.roster);
        }
    }

    /// **D5: a holder restarted alone pins once** — after the polite stop,
    /// which took it off its board's roster, and after a crash, which did not;
    /// and with an entry addressed to itself left on its board, which is acted
    /// on once, on the holder's path, after it holds again (B14, B9). Before,
    /// the restarted process announced to its own old board, and acknowledged
    /// the entry onto it.
    #[test]
    fn a_lone_holder_restarted_pins_once_and_acts_on_its_own_entry_once() {
        for (polite, with_an_entry) in [(true, false), (false, false), (false, true), (true, true)] {
            let (mut fleet, _) = a_fleet_with_one_holder(&["a"]);
            let mut operation = None;
            if with_an_entry {
                let entry = an_action_for("a", "a test instruction", json!({ "n": 1 })).unwrap();
                operation = Some(entry.operation.clone());
                fleet.member("a").listener.propose(entry);
                fleet.member("a").listener.a_turn();
                let board = fleet.sim.the_board_on_top().unwrap();
                assert!(board.instructions.iter().any(|e| Some(&e.operation) == operation.as_ref()));
                assert!(lock(&fleet.acted).is_empty(), "written, not yet acted on");
            }
            fleet.members.clear();
            let old = *fleet.sim.state().pins.iter().max().unwrap();
            let mut board = fleet.sim.the_board_on_top().expect("its board is on top");
            if polite {
                board.roster.retain(|r| r.machine != "a");
                fleet.sim.state().texts.insert(old, board.compose().unwrap());
            }
            let since = heard_so_far(&fleet);
            fleet.start("a");
            fleet.rounds_until(80, |f| f.holders() == ["a"]).expect("it holds again");
            for _ in 0..90 {
                fleet.a_round();
            }
            let pins = pinned_by(&fleet, "a", since);
            assert_eq!(pins.len(), 1, "polite {polite}, entry {with_an_entry}: one pin, its new board: {pins:?}");
            assert!(machinery_sent_by(&fleet, "a", since).is_empty(), "nothing pinned to its own old board");
            let board = fleet.sim.the_board_on_top().unwrap();
            assert!(listed(&board, "a"));
            assert!(board.instructions.is_empty());
            if let Some(operation) = operation {
                assert_eq!(*lock(&fleet.acted), vec![("a".to_string(), operation)], "acted on once");
            }
        }
    }

    /// **D6: a new server joining a live holder announces only after the
    /// first renewal it sees** — and an instruction addressed to it before
    /// then reaches it, is acted on and is acknowledged, with no announcement
    /// (B14: only the announcement waits; nothing is routed by the roster).
    #[test]
    fn a_newcomer_to_a_live_holder_acts_before_it_announces_and_announces_after_a_renewal() {
        let (mut fleet, _) = a_fleet_with_one_holder(&["a"]);
        // Start it just after a renewal, so its wait is the whole interval.
        let before = fleet.sim.the_board_on_top().unwrap().heartbeat;
        fleet
            .rounds_until(40, |f| f.sim.the_board_on_top().is_some_and(|b| b.heartbeat != before))
            .expect("the holder renewed");
        let renewed = fleet.sim.the_board_on_top().unwrap().heartbeat;
        let since = heard_so_far(&fleet);
        fleet.start("b");
        let entry = an_action_for("b", "a test instruction", json!({ "n": 2 })).unwrap();
        let operation = entry.operation.clone();
        fleet.member("a").listener.propose(entry);
        fleet
            .rounds_until(15, |f| !lock(&f.acted).is_empty() && f.sim.the_board_on_top().is_some_and(|b| b.instructions.is_empty()))
            .expect("acted on and acknowledged before any renewal");
        assert_eq!(*lock(&fleet.acted), vec![("b".to_string(), operation)]);
        let board = fleet.sim.the_board_on_top().unwrap();
        assert_eq!(board.heartbeat, renewed, "no renewal yet");
        assert!(!listed(&board, "b"), "not announced yet");
        assert!(machinery_sent_by(&fleet, "b", since).iter().all(|t| !t.contains("\"announce\":true")));
        fleet
            .rounds_until(40, |f| f.sim.the_board_on_top().is_some_and(|b| listed(&b, "b")))
            .expect("announced and listed after the renewal");
        assert_ne!(fleet.sim.the_board_on_top().unwrap().heartbeat, renewed);
    }

    /// **D8, GPT's change 4: a takeover with work outstanding** — an accepted
    /// instruction on the dead holder's board and a machinery message pinned
    /// above it — **ends in takeover with the instruction kept, inside B7's
    /// interval and a read**, on a lag after the unpin of three seconds, which
    /// the old back-to-back reads never outlasted: their claim ended unfinished
    /// and waited out a whole interval more.
    #[test]
    fn a_takeover_with_work_outstanding_finishes_inside_the_interval_despite_a_long_lag() {
        let (mut fleet, holder) = a_fleet_with_one_holder(&["a", "b"]);
        let other = if holder == "a" { "b" } else { "a" }.to_string();
        let entry = an_action_for("gone", "a test instruction", json!({ "n": 1 })).unwrap();
        let operation = entry.operation.clone();
        fleet.member(&holder).listener.propose(entry);
        fleet
            .rounds_until(10, |f| f.sim.the_board_on_top().is_some_and(|b| b.instructions.iter().any(|e| e.operation == operation)))
            .expect("the proposal was accepted onto the board");
        fleet.member(&holder).alive = false;
        let text = TheUtterance { machine: "late".into(), display: "Server late".into(), announce: true, ..TheUtterance::default() }
            .compose()
            .unwrap();
        let late = fleet.sim.a_message_from_the_bot(&text);
        let since = heard_so_far(&fleet);
        {
            let mut s = fleet.sim.state();
            s.pins.push(late);
            s.blind_seconds_after_an_unpin = Duration::from_secs(3);
        }
        let rounds = fleet.rounds_until(60, |f| f.holders() == vec![other.clone()]).expect("the other took the slot");
        assert!(rounds as u64 * 2 <= 60 + 2 * 4, "took the slot after {} seconds", rounds * 2);
        for _ in 0..60 {
            fleet.a_round();
        }
        let board = fleet.sim.the_board_on_top().unwrap();
        assert!(board.instructions.iter().any(|e| e.operation == operation), "B3: the instruction was kept");
        assert_eq!(pinned_by(&fleet, &other, since).len(), 1, "its board, once, and no repeating cycle");
    }

    /// **B3 and B11 as written 2026-10-18: ten unsettled reads, spaced, span
    /// the whole settle.**
    #[test]
    fn the_spaced_reads_cover_the_settle() {
        assert!(THE_SPACING_OF_UNSETTLED_READS * (THE_MOST_UNSETTLED_READS_IN_ONE_TURN as u32 - 1) >= THE_SETTLE_BEFORE_NOTHING_PINNED_IS_BELIEVED);
    }

    /// **`Telegram` B18 — which pin's line is Smoozy's to delete**, each guard
    /// on its own: the bot's own message, opening with the head, pinned by no
    /// person; the line's identity, never the pinned message's.
    #[test]
    fn a_line_is_smoozys_to_delete_only_where_all_three_hold() {
        let head = format!("{} \u{00b7} Server a\n\nsmoozy-machinery 1\n{{}}", crate::board::text::THE_HEAD);
        let a_pin = |from: Option<bool>, is_bot: bool, text: &str| {
            let mut message = json!({ "message_id": 50_007, "chat": { "id": CHAT },
                "pinned_message": { "message_id": 12, "from": { "is_bot": is_bot }, "text": text } });
            if let Some(from) = from {
                message["from"] = json!({ "is_bot": from });
            }
            message
        };
        assert_eq!(the_line_a_pin_of_smoozys_wrote(&a_pin(Some(true), true, &head)), Some(50_007));
        assert_eq!(the_line_a_pin_of_smoozys_wrote(&a_pin(None, true, &head)), Some(50_007), "no sender stated");
        assert_eq!(the_line_a_pin_of_smoozys_wrote(&a_pin(Some(false), true, &head)), None, "the user pinned it");
        assert_eq!(the_line_a_pin_of_smoozys_wrote(&a_pin(Some(true), false, &head)), None, "not the bot's own");
        assert_eq!(the_line_a_pin_of_smoozys_wrote(&a_pin(Some(true), true, "Frame 12 done")), None, "no head");
        // A pin an older server made, under the head before step 13's words
        // sweep, is still Smoozy's own.
        let the_older_head = format!("{} \u{00b7} Server a\n\nsmoozy-machinery 1\n{{}}", crate::board::text::THE_HEAD_BEFORE_2026_10_28);
        assert_eq!(the_line_a_pin_of_smoozys_wrote(&a_pin(Some(true), true, &the_older_head)), Some(50_007), "the older head");
        let mut naming_itself = a_pin(Some(true), true, &head);
        naming_itself["pinned_message"]["message_id"] = json!(50_007);
        assert_eq!(the_line_a_pin_of_smoozys_wrote(&naming_itself), None, "never the pinned message");
        assert_eq!(the_line_a_pin_of_smoozys_wrote(&json!({ "message_id": 50_007, "text": "hello" })), None, "not a pin");
    }
}
