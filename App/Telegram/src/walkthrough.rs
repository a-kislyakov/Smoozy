//! **The setup walkthrough, the desktop's half of setting up a bot — `Telegram`
//! §6 S1 to S6, §8 K1, and §19's check-then-store row and first diary act.**
//!
//! **S1: the walkthrough runs on the user's own computer**, and its exchanges
//! with Telegram are the desktop's, not a machine's. Every step is an act with
//! its own closed answer, so the screens (step 10 piece 8) call them in order
//! and draw what comes back. **S4's order is kept by the types rather than by
//! care**: the wait for Start can only be made from a check that passed, hello
//! back only from a Start that arrived, and keeping only after hello back.
//!
//! - **S1a, S1b** — [`TheOffer`]: the name `Smoozy`, and `smoozy_` plus four
//!   random characters plus `_bot`, redrawn on *Another* and never claimed free.
//! - **S2** — [`ThePaste`]: the token is found inside whatever was pasted, and
//!   the paste is never shown, logged or passed to a process.
//! - **S3, S3a** — [`check_the_bot_answers`]: `getMe`, sorted into answered,
//!   not accepted, or an ordinary failure with the paste kept.
//! - **S5, P1** — [`TheWaitForStart`]: the one desktop poll, only on a token
//!   for a bot not already proved.
//! - **S6** — [`TheProof::say_hello_back`].
//! - **S4, K1, §19** — [`TheHelloWasSaid::keep`]: the proof recorded, then the
//!   token stored as the fourth stored secret kind, then
//!   [`TheBotIsSetUp`], which carries the diary act's fixed form.
//!
//! **What it relies on about the service.** `Preparation/Architecture/Sources/
//! Telegram.sources.md` entry 23, **observed in a test**: a token that never
//! existed is refused with `error_code` 401 and `Unauthorized`, byte for byte
//! what a revoked one gets. Entry 17, **Creator-stated with the Start
//! requirement observed**: the user must press Start before the bot can be
//! messaged at all, so the first message a new bot receives is sent after
//! Start. Entry 24, **observed**: `getMe` consumes no update. The token's shape
//! — digits, a colon, then letters, digits, `-` and `_` — is the one
//! `tests/the_real_service_answers.rs` reads, from the example address on
//! `https://core.telegram.org/bots/api`.
//!
//! **Markers in this file are `Telegram`'s**: S1, S1a, S1b, S1c, S2, S3, S3a,
//! S4, S5, S6, K1, P1, P4, P6. `Security and licence`'s are written with their
//! document.

use std::fmt;
use std::time::{Duration, SystemTime};

use serde_json::Value;
use smoozy_security_and_licence::erasure::{self, EraseMarks};
use smoozy_security_and_licence::held::{HeldSecret, StoredKind, Subject};
use smoozy_security_and_licence::plaintext::Plaintext;
use smoozy_security_and_licence::secret_store::{SecretStore, StoreFailure};

use crate::proof::{
    the_conversation_an_update_came_from, what_a_replacement_keeps, ARecordFailure,
    KeepsTheProvedBot, TheBot, TheConversation, WhatAReplacementKeeps,
};
use crate::transport::{TheClient, WhyTheCallFailed};

// ------------------------------------------------------------ S1a, S1b: the offer

/// **S1a: the bot's name, the same for every user's bot.** Telegram publishes
/// no rule for a name beyond a length ceiling (Fact 133), so it is checked
/// against nothing.
pub const THE_BOTS_NAME: &str = "Smoozy";

/// S1a: the username's fixed stem and fixed ending. Neither ever moves (S1b).
pub const THE_USERNAMES_STEM: &str = "smoozy_";
pub const THE_USERNAMES_ENDING: &str = "_bot";

/// S1a: the block is drawn from the twenty-six lowercase letters and the ten
/// digits, never an underscore.
const THE_BLOCKS_CHARACTERS: &[u8; 36] = b"abcdefghijklmnopqrstuvwxyz0123456789";

/// This computer gave no random bytes, so no candidate could be drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoRandomBytes;

/// **The two names on offer, and every block offered so far in this
/// walkthrough.** S1b: *Another* redraws the block alone, and a draw matching
/// any block already offered is drawn again. **Nothing here claims a candidate
/// is free**, because nothing Smoozy can reach answers that (S1b).
#[derive(Debug, Clone)]
pub struct TheOffer {
    offered: Vec<[u8; 4]>,
}

impl TheOffer {
    /// The first offer of a walkthrough.
    pub fn made() -> Result<TheOffer, NoRandomBytes> {
        TheOffer::made_with(&mut |bytes| getrandom::fill(bytes).map_err(|_| NoRandomBytes))
    }

    /// The same, drawing from the source given — for tests.
    pub fn made_with(
        draw: &mut dyn FnMut(&mut [u8]) -> Result<(), NoRandomBytes>,
    ) -> Result<TheOffer, NoRandomBytes> {
        let mut offer = TheOffer { offered: Vec::new() };
        offer.another_with(draw)?;
        Ok(offer)
    }

    /// **S1b: *Another*.** The new candidate differs from every one before it.
    pub fn another(&mut self) -> Result<(), NoRandomBytes> {
        self.another_with(&mut |bytes| getrandom::fill(bytes).map_err(|_| NoRandomBytes))
    }

    pub fn another_with(
        &mut self,
        draw: &mut dyn FnMut(&mut [u8]) -> Result<(), NoRandomBytes>,
    ) -> Result<(), NoRandomBytes> {
        loop {
            let block = a_block(draw)?;
            if !self.offered.contains(&block) {
                self.offered.push(block);
                return Ok(());
            }
        }
    }

    /// S1a: `Smoozy`.
    pub fn the_name(&self) -> &'static str {
        THE_BOTS_NAME
    }

    /// S1a: the candidate on offer now — 15 characters, lowercase.
    pub fn the_username(&self) -> String {
        let block = self.offered.last().expect("an offer always holds one block");
        format!(
            "{THE_USERNAMES_STEM}{}{THE_USERNAMES_ENDING}",
            std::str::from_utf8(block).expect("the block is ASCII")
        )
    }
}

/// **Four characters, each uniform over the thirty-six.** A byte of 252 or
/// more is thrown away and drawn again, so no character is likelier than
/// another (252 is seven times 36).
fn a_block(
    draw: &mut dyn FnMut(&mut [u8]) -> Result<(), NoRandomBytes>,
) -> Result<[u8; 4], NoRandomBytes> {
    let mut block = [0u8; 4];
    let mut filled = 0;
    while filled < 4 {
        let mut byte = [0u8; 1];
        draw(&mut byte)?;
        if byte[0] < 252 {
            block[filled] = THE_BLOCKS_CHARACTERS[usize::from(byte[0] % 36)];
            filled += 1;
        }
    }
    Ok(block)
}

/// **S1a's published username rule** (Fact 133, documented): letters, digits
/// and underscores only, 5 to 32 characters, ending in `bot`. Every candidate
/// meets it by construction; S3a holds an answered username to it.
pub fn meets_the_published_username_rule(username: &str) -> bool {
    (5..=32).contains(&username.len())
        && username
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
        && username.to_ascii_lowercase().ends_with("bot")
}

// ------------------------------------------------------------ S2: the paste

/// **What the user pasted, held for the check and kept for a retry.** S2: it is
/// never logged, never echoed and never a process argument, so this type shows
/// nothing of itself and its bytes are overwritten when it goes.
pub struct ThePaste {
    text: String,
}

impl ThePaste {
    pub fn what_the_user_pasted(text: impl Into<String>) -> ThePaste {
        ThePaste { text: text.into() }
    }

    /// **Whether a token can be found in it** — for a screen deciding whether
    /// the check can be offered, and for a test that the paste was kept.
    pub fn holds_a_token(&self) -> bool {
        self.the_token_in_it().is_some()
    }

    /// **S2: the one token-shaped value in the paste.** The paste is split at
    /// every character a token cannot hold, and each piece is held to the
    /// token's shape. **Exactly one distinct value is a token; none, or two
    /// different ones, is no token**, because choosing between two would be a
    /// guess.
    fn the_token_in_it(&self) -> Option<TheToken> {
        let mut found: Option<&str> = None;
        for piece in self
            .text
            .split(|c: char| !(c.is_ascii_alphanumeric() || c == ':' || c == '-' || c == '_'))
        {
            if !is_shaped_like_a_bot_token(piece) {
                continue;
            }
            match found {
                None => found = Some(piece),
                Some(already) if already == piece => {}
                Some(_) => return None,
            }
        }
        found.map(|token| TheToken(token.to_string()))
    }
}

impl fmt::Debug for ThePaste {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ThePaste(held, not shown)")
    }
}

impl Drop for ThePaste {
    fn drop(&mut self) {
        overwrite(&mut self.text);
    }
}

/// Digits, one colon, then letters, digits, `-` and `_`. **Nothing that could
/// change the address it is put into** — no slash, no dot, no space.
fn is_shaped_like_a_bot_token(piece: &str) -> bool {
    match piece.split_once(':') {
        Some((bot, secret)) => {
            !bot.is_empty()
                && bot.bytes().all(|b| b.is_ascii_digit())
                && !secret.is_empty()
                && secret
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        }
        None => false,
    }
}

/// The token, once found. Shows nothing of itself and is overwritten when it
/// goes.
struct TheToken(String);

impl Drop for TheToken {
    fn drop(&mut self) {
        overwrite(&mut self.0);
    }
}

fn overwrite(text: &mut String) {
    // SAFETY: every byte becomes 0, which is valid UTF-8, before the string is
    // emptied.
    unsafe {
        for byte in text.as_mut_vec().iter_mut() {
            std::ptr::write_volatile(byte, 0);
        }
    }
    text.clear();
}

// ------------------------------------------------------------ S3, S3a: the check

/// **What asking the bot found — S3's three readings, and the ways an answer
/// can still fail to be usable.**
#[derive(Debug)]
pub enum WhatTheCheckFound {
    /// **S3 answered, with S3a's usable username.**
    Answered(TheCheckPassed),
    /// **S3 refused — the 401 body.** Telegram did not accept the token, and
    /// the user is asked to check the paste.
    NotAccepted,
    /// **S2: nothing in the paste is shaped like a token**, so nothing was
    /// asked of Telegram. Not the same as *not accepted*: Telegram was not
    /// asked.
    NoTokenInThePaste,
    /// **An ordinary failed action, never read as a wrong token.** The paste
    /// is kept, nothing is stored, and the check is offered again.
    DidNotFinish(WhyTheCheckDidNotFinish),
}

/// **Why a check ended as an ordinary failure** (S3, S3a).
#[derive(Debug)]
pub enum WhyTheCheckDidNotFinish {
    /// **S3 unreachable**: no answer came from Telegram at all.
    TelegramCouldNotBeReached,
    /// **S3a**: Telegram accepted the token and gave back no usable bot
    /// address — no identity, or no username meeting the published rule.
    NoUsableBotAddressCameBack,
    /// **Something came back that is not an answer.** The plan names three
    /// readings and the client has a fourth; it is placed here, with the
    /// ordinary failures, because it says nothing about the token.
    TheAnswerCouldNotBeRead,
    /// **A refusal that is not the 401 body** — a flood refusal, for one. It
    /// says nothing about the token either.
    RefusedForAnotherReason { error_code: Option<i64> },
}

/// **S3: ask the real service.**
pub fn check_the_bot_answers(paste: &ThePaste) -> WhatTheCheckFound {
    check_the_bot_answers_through(paste, &|token| TheClient::reaching_telegram_with(token))
}

/// **S3 through whatever reaches Telegram** — the real service, or a stand-in
/// in a test. The paste is borrowed and stays with the caller whatever comes
/// back, which is S3's *keeps the paste for the retry*.
pub fn check_the_bot_answers_through(
    paste: &ThePaste,
    reach: &dyn Fn(&str) -> TheClient,
) -> WhatTheCheckFound {
    let Some(token) = paste.the_token_in_it() else {
        return WhatTheCheckFound::NoTokenInThePaste;
    };
    let client = reach(&token.0);
    match client.get_me() {
        Ok(answer) => match the_bot_an_answer_names(&answer) {
            Some(bot) => WhatTheCheckFound::Answered(TheCheckPassed { token, bot, client }),
            None => WhatTheCheckFound::DidNotFinish(
                WhyTheCheckDidNotFinish::NoUsableBotAddressCameBack,
            ),
        },
        Err(WhyTheCallFailed::Refused(refusal)) if refusal.error_code == Some(401) => {
            WhatTheCheckFound::NotAccepted
        }
        Err(WhyTheCallFailed::Refused(refusal)) => WhatTheCheckFound::DidNotFinish(
            WhyTheCheckDidNotFinish::RefusedForAnotherReason {
                error_code: refusal.error_code,
            },
        ),
        Err(WhyTheCallFailed::NotReached(_)) => {
            WhatTheCheckFound::DidNotFinish(WhyTheCheckDidNotFinish::TelegramCouldNotBeReached)
        }
        Err(WhyTheCallFailed::TheAnswerCouldNotBeRead(_)) => {
            WhatTheCheckFound::DidNotFinish(WhyTheCheckDidNotFinish::TheAnswerCouldNotBeRead)
        }
    }
}

/// **S3a: the answered object is the authority**, and it must carry an
/// identity and a non-empty username meeting the published rule.
fn the_bot_an_answer_names(answer: &Value) -> Option<TheBot> {
    let id = answer.get("id").and_then(Value::as_i64)?;
    let username = answer.get("username").and_then(Value::as_str)?;
    if !meets_the_published_username_rule(username) {
        return None;
    }
    Some(TheBot {
        id,
        username: username.to_string(),
    })
}

/// **A check that passed: the token, the bot it names, and a client on it.**
/// Nothing is stored and nothing is delivered anywhere yet (S3, S4).
pub struct TheCheckPassed {
    token: TheToken,
    bot: TheBot,
    client: TheClient,
}

impl fmt::Debug for TheCheckPassed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TheCheckPassed")
            .field("bot", &self.bot)
            .field("token", &"not shown")
            .finish()
    }
}

impl TheCheckPassed {
    /// S3a: the bot the token names, as the answer carried it.
    pub fn the_bot(&self) -> &TheBot {
        &self.bot
    }

    /// **S6: the link, composed from S3a's answered username** and never from
    /// an offered candidate.
    pub fn the_link(&self) -> String {
        format!("https://t.me/{}", self.bot.username)
    }

    /// **S5 and P4: whether the desktop may wait for Start on this token.**
    ///
    /// **A token for the bot already proved is a replacement**: the standing
    /// proof is kept and the desktop never receives on it (S5, P4), so no wait
    /// is offered and what follows is the replacement path of `Telegram` L4,
    /// which is not this walkthrough's. **Any other bot is a new bot**, whose
    /// token no machine listens on, and the one poll is safe.
    pub fn what_comes_next(self, proved_on_record: Option<&TheBot>) -> WhatComesNext {
        if let Some(proved) = proved_on_record {
            if let WhatAReplacementKeeps::TheProofStands { refreshed } =
                what_a_replacement_keeps(proved, &self.bot)
            {
                return WhatComesNext::TheSameBotIsAlreadyProved {
                    refreshed,
                    check: self,
                };
            }
        }
        WhatComesNext::WaitForStart(TheWaitForStart {
            check: self,
            confirmed_below: None,
        })
    }
}

impl TheCheckPassed {
    /// **`Telegram` L4: a checked token handed on as the replacement for the bot
    /// already proved.** Added by step 10 piece 5, which stores it.
    ///
    /// **P4 decides it, by the bot's identity alone**: the same bot gives back
    /// its refreshed self (S3a) and the token as [`Plaintext`] for the store; any
    /// other bot is a new proof and the check is handed back untouched. **The
    /// token leaves this type only as [`Plaintext`]**, which is exposed once at
    /// the store and overwritten.
    // The check is handed back whole because the walkthrough goes on from it.
    #[allow(clippy::result_large_err)]
    pub fn the_replacement_for(self, proved: &TheBot) -> Result<(TheBot, Plaintext), TheCheckPassed> {
        match what_a_replacement_keeps(proved, &self.bot) {
            WhatAReplacementKeeps::TheProofStands { refreshed } => {
                Ok((refreshed, Plaintext::new(self.token.0.as_bytes().to_vec())))
            }
            WhatAReplacementKeeps::ANewBotNeedsANewProof => Err(self),
        }
    }
}

/// **What follows a passed check.**
#[derive(Debug)]
pub enum WhatComesNext {
    /// A new bot: wait for the user's Start (S5).
    WaitForStart(TheWaitForStart),
    /// **P4: the bot on record, with the username refreshed from this answer
    /// (S3a).** The walkthrough does not wait; the replacement path is
    /// `Telegram` L4's.
    TheSameBotIsAlreadyProved {
        refreshed: TheBot,
        check: TheCheckPassed,
    },
}

// ------------------------------------------------------------ S5: the one poll

/// **S5: the desktop's one poll for Start.**
///
/// **Each ask confirms only what the ask before it in this walkthrough
/// returned, and the Start itself is never confirmed.** The first ask confirms
/// nothing. A later ask has to move past what came back without being a Start,
/// or the service would hand the same updates straight back and the wait would
/// never wait; on a new bot nothing else holds the token (S5), so what comes
/// back is this walkthrough's. **The Start is left unconfirmed** because the
/// plan requires nothing of it after the proof, and taking it leaves the bot's
/// update position where the service put it.
#[derive(Debug)]
pub struct TheWaitForStart {
    check: TheCheckPassed,
    confirmed_below: Option<i64>,
}

/// What one ask found.
#[derive(Debug)]
pub enum WhatOneAskFound {
    /// **P1: the conversation the Start arrived from.**
    StartArrived(TheProof),
    /// Nothing yet; ask again, or the user cancels.
    NothingYet(TheWaitForStart),
    /// **An ordinary failure**: the wait is handed back and may be asked
    /// again.
    TheAskFailed(TheWaitForStart, WhyTheCallFailed),
}

impl TheWaitForStart {
    /// The check this wait was made from.
    pub fn the_check(&self) -> &TheCheckPassed {
        &self.check
    }

    /// **The update position this wait has confirmed below, where it has
    /// confirmed any** — so a live check can write down what it consumed.
    pub fn confirmed_below(&self) -> Option<i64> {
        self.confirmed_below
    }

    /// **One ask, holding the request open for up to `wait_seconds`.** It
    /// names the kinds, as every ask does (P6).
    ///
    /// **The Start is the first message with a conversation that can be
    /// established.** Before Start the bot cannot be messaged at all (entry
    /// 17), so on a new bot the first message is sent after it. An update of
    /// the other kind, or one whose conversation cannot be established, is not
    /// a Start (P3).
    pub fn ask_once(mut self, wait_seconds: u32) -> WhatOneAskFound {
        let answer = match self.check.client.get_updates(self.confirmed_below, wait_seconds) {
            Ok(answer) => answer,
            Err(why) => return WhatOneAskFound::TheAskFailed(self, why),
        };
        let Some(updates) = answer.as_array() else {
            return WhatOneAskFound::TheAskFailed(
                self,
                WhyTheCallFailed::TheAnswerCouldNotBeRead("an ask's answer was not a list"),
            );
        };
        for update in updates {
            if update.get("message").is_none() {
                continue;
            }
            if let Some(conversation) = the_conversation_an_update_came_from(update) {
                return WhatOneAskFound::StartArrived(TheProof {
                    check: self.check,
                    conversation,
                });
            }
        }
        if let Some(highest) = updates
            .iter()
            .filter_map(|update| update.get("update_id").and_then(Value::as_i64))
            .max()
        {
            self.confirmed_below = Some(highest + 1);
        }
        WhatOneAskFound::NothingYet(self)
    }
}

// ------------------------------------------------------------ S6: hello back

/// **S6: what Smoozy says back from the desktop** when the Start arrives.
/// **It says only that this chat is connected, and never that Telegram is set
/// up**: it is said before anything is kept (S4), so a keep that then fails
/// would leave the phone claiming a setup that did not happen. The desktop's
/// *"The bot replied. Telegram is set up."* is drawn only after keeping. Ruled
/// 2026-09-14 by the round *setting it up, and what the desktop keeps*.
/// Step 13's words sweep: *Hello from Smoozy. This chat is connected.* until
/// then (Rule 9: one sentence, no greeting).
/// *Changed 2026-11-06, the last words read before the release* (was *Smoozy is
/// connected to this chat.*): the bot is the one speaking.
pub const HELLO_BACK: &str = "Connected to this chat.";

/// **P1: a Start received, and the conversation it came from.** Nothing is
/// recorded or stored yet.
#[derive(Debug)]
pub struct TheProof {
    check: TheCheckPassed,
    conversation: TheConversation,
}

impl TheProof {
    pub fn the_bot(&self) -> &TheBot {
        &self.check.bot
    }

    pub fn the_conversation(&self) -> TheConversation {
        self.conversation
    }

    /// **S6: the received Start answered, from the desktop.** A failure hands
    /// the proof back unchanged, so hello can be said again; nothing is kept
    /// until it has been.
    // The proof is handed back whole because it is what the retry is made from;
    // boxing it would change nothing but the caller's spelling.
    #[allow(clippy::result_large_err)]
    pub fn say_hello_back(self) -> Result<TheHelloWasSaid, (TheProof, WhyTheCallFailed)> {
        match self
            .check
            .client
            .send_message(self.conversation.0, HELLO_BACK, None)
        {
            Ok(_) => Ok(TheHelloWasSaid { proof: self }),
            Err(why) => Err((self, why)),
        }
    }
}

// ------------------------------------------------------------ S4, K1: keeping

/// The legacy bot item's subject, retained for existing records and owed
/// removals. New saves reserve their own subjects in the local record.
pub fn the_bot_tokens_item() -> HeldSecret {
    HeldSecret::new(
        StoredKind::BotToken,
        Subject::issued_by_the_local_record("telegram-bot"),
    )
}

/// A fresh opaque subject for one saved bot key. Cleanup of any earlier
/// subject cannot reach it. Randomness is supplied by the operating system.
pub fn a_fresh_bot_tokens_item() -> Result<HeldSecret, ARecordFailure> {
    let mut bytes = [0u8; 16];
    getrandom::fill(&mut bytes).map_err(|_| ARecordFailure("no random bytes for a bot item".into()))?;
    let suffix: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    Ok(HeldSecret::new(StoredKind::BotToken,
        Subject::issued_by_the_local_record(format!("telegram-bot.{suffix}"))))
}

/// How long a store call is waited on (`Security and licence` §3.0 rule 5).
pub const THE_STORE_CALLS_DEADLINE: Duration = Duration::from_secs(5);

/// **Hello back said, and the proof ready to keep.**
#[derive(Debug)]
pub struct TheHelloWasSaid {
    proof: TheProof,
}

/// **Why keeping did not finish.** Both are ordinary failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyTheBotWasNotKept {
    /// **The local record did not take the proof, so the token was not
    /// stored**: S4 puts the proof first.
    TheRecordDidNotTakeTheProof(ARecordFailure),
    /// **The proof is recorded and the token is not stored.** Keeping again
    /// writes both again; nothing half-done is left that a second keep cannot
    /// finish.
    TheTokenWasNotStored(StoreFailure),
    /// **The store did not answer in time.** `Security and licence` §3.0 rule
    /// 5 as amended 2026-09-14, word for word: *"A put or an erase that times
    /// out is reported as **not confirmed** and never as done … the next read
    /// decides what is held."* **Every timed-out save is this, whatever the
    /// record named before**: the adapter decides whether to undo a late
    /// landing by what the *store* held, and the record cannot see that, so
    /// only *not confirmed* is true in every case. Only a plain refusal is
    /// *not stored*. Keeping again is still offered and still finishes it.
    /// Narrowed 2026-09-14 by the round *setting it up, and what the desktop
    /// keeps*.
    TheTokenWasNotConfirmed,
    /// The record could not read the removal debts, or offered an item
    /// already owed to cleanup. No save was attempted.
    TheRecordDidNotAnswerAboutAnOwedErase,
    /// **The token is stored and the record did not take the fact that it
    /// is**, so nothing will draw it as held. Keeping again writes both again.
    TheRecordDidNotTakeTheStoredToken(ARecordFailure),
}

impl TheHelloWasSaid {
    pub fn the_bot(&self) -> &TheBot {
        &self.proof.check.bot
    }

    pub fn the_conversation(&self) -> TheConversation {
        self.proof.conversation
    }

    /// **S4 and §19's check-then-store: the proof recorded, then the token
    /// stored.** The check has already passed by construction, so no store is
    /// ever attempted for a check that did not (§19). **On either failure this
    /// is handed back with the reason**, so the act can be offered again.
    ///
    /// **Nothing is delivered to any machine here.** Delivery is `Telegram`
    /// §20's material through `Server Setup`, and comes after this.
    ///
    /// Each save uses the record's reserved item. Earlier items are retried
    /// quietly and remain owed if the store cannot confirm their removal.
    pub fn keep(
        self,
        record: &dyn KeepsTheProvedBot,
        store: &dyn SecretStore,
        marks: &mut dyn EraseMarks,
    ) -> Result<TheBotIsSetUp, (TheHelloWasSaid, WhyTheBotWasNotKept)> {
        let bot = self.proof.check.bot.clone();
        let conversation = self.proof.conversation;
        if let Err(why) = record.the_proof_was_recorded(&bot, conversation) {
            return Err((self, WhyTheBotWasNotKept::TheRecordDidNotTakeTheProof(why)));
        }
        let item = match record.the_item_for_saving(&bot) {
            Ok(item) => item,
            Err(why) => return Err((self, WhyTheBotWasNotKept::TheRecordDidNotTakeTheProof(why))),
        };
        let owed = match marks.marks() {
            Ok(owed) if !owed.contains(&item) => owed,
            _ => return Err((self, WhyTheBotWasNotKept::TheRecordDidNotAnswerAboutAnOwedErase)),
        };
        for old in owed.into_iter().filter(|old| old.kind == StoredKind::BotToken) {
            // A refused or late old erase does not own the fresh save item.
            let _ = erasure::erase(marks, store, &old, THE_STORE_CALLS_DEADLINE);
        }
        let value = Plaintext::new(self.proof.check.token.0.as_bytes().to_vec());
        match store.put(&item, value, THE_STORE_CALLS_DEADLINE) {
            Ok(()) => {}
            // `Security and licence` §3.0 rule 5 as amended 2026-09-14: every
            // timed-out save is *not confirmed*; only a refusal is *not stored*.
            Err(StoreFailure::StoreUnavailable) => {
                return Err((self, WhyTheBotWasNotKept::TheTokenWasNotConfirmed));
            }
            Err(why) => return Err((self, WhyTheBotWasNotKept::TheTokenWasNotStored(why))),
        }
        if let Err(why) = record.the_token_was_stored_for(&bot) {
            return Err((self, WhyTheBotWasNotKept::TheRecordDidNotTakeTheStoredToken(why)));
        }
        Ok(TheBotIsSetUp { bot, conversation })
    }
}

// ------------------------------------------------------------ §19: the diary act

/// **§19's first act, *a bot set up*: its fixed line form.** The act's name;
/// the moment on the desktop clock; the bot's username; and the proved
/// conversation's identity. **Never the token or any part of one**, never a
/// candidate, a check or the poll — each of those is a step inside this act.
pub const A_BOT_SET_UP: &str = "a bot set up; {moment}; @{username}; conversation {conversation}";

/// **The walkthrough finished**: the check passed, the proof recorded, the
/// token stored (S4). This is the completion §19's line is written at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheBotIsSetUp {
    pub bot: TheBot,
    pub conversation: TheConversation,
}

impl TheBotIsSetUp {
    /// **The fixed form, filled.** The moment is seconds since the epoch on
    /// this computer's clock; the line's carrier says which clock.
    pub fn the_diary_act(&self, at: SystemTime) -> String {
        let seconds = at
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|since| since.as_secs())
            .unwrap_or(0);
        A_BOT_SET_UP
            .replace("{moment}", &seconds.to_string())
            .replace("{username}", &self.bot.username)
            .replace("{conversation}", &self.conversation.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proof::a_stand_in::ARecordInMemory;
    use crate::transport::a_stand_in::AStandInService;
    use crate::transport::WhatACallCarries;
    use serde_json::json;
    use smoozy_security_and_licence::secret_store::{GetFailure, ProbeOutcome};
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    const A_TOKEN: &str = "123456789:AAH-not_a_real_token_0123456789abcd";

    fn over(service: &Arc<AStandInService>) -> impl Fn(&str) -> TheClient + '_ {
        move |_token| TheClient::over(service.clone())
    }

    fn a_reply_carrying(token: &str) -> ThePaste {
        ThePaste::what_the_user_pasted(format!(
            "Done! Congratulations on your new bot. You will find it at t.me/smoozy_a7k2_bot.\n\
             Use this token to access the HTTP API:\n{token}\nKeep your token secure."
        ))
    }

    fn answered(id: i64, username: &str) -> String {
        json!({ "ok": true, "result": { "id": id, "is_bot": true, "first_name": "Smoozy", "username": username } }).to_string()
    }

    #[derive(Default)]
    struct AStore {
        held: Mutex<HashMap<String, Vec<u8>>>,
        refuses: Mutex<bool>,
        /// Every put answers as the adapter does when its deadline passes.
        does_not_answer_in_time: Mutex<bool>,
        /// Every erase is refused.
        erases_refused: Mutex<bool>,
    }

    impl SecretStore for AStore {
        fn put(&self, item: &HeldSecret, value: Plaintext, _: Duration) -> Result<(), StoreFailure> {
            if *self.refuses.lock().unwrap() {
                return Err(StoreFailure::LockedOrRefused);
            }
            if *self.does_not_answer_in_time.lock().unwrap() {
                return Err(StoreFailure::from_the_deadline_passing());
            }
            let bytes = value.expose(|b| b.to_vec());
            self.held.lock().unwrap().insert(format!("{:?}", item), bytes);
            Ok(())
        }
        fn get(&self, item: &HeldSecret, _: Duration) -> Result<Plaintext, GetFailure> {
            self.held
                .lock()
                .unwrap()
                .get(&format!("{:?}", item))
                .map(|b| Plaintext::new(b.clone()))
                .ok_or(GetFailure::NotThere)
        }
        fn erase(&self, item: &HeldSecret, _: Duration) -> Result<(), StoreFailure> {
            if *self.erases_refused.lock().unwrap() {
                return Err(StoreFailure::LockedOrRefused);
            }
            self.held.lock().unwrap().remove(&format!("{:?}", item));
            Ok(())
        }
        fn probe(&self, _: Duration) -> ProbeOutcome {
            ProbeOutcome::RoundTripWorked
        }
        fn enumerate(&self, _: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(Vec::new())
        }
    }

    /// `Security and licence` §5.1's register, in memory.
    #[derive(Default)]
    struct Marks(Vec<HeldSecret>);

    impl EraseMarks for Marks {
        fn mark_owing_an_erase(&mut self, item: &HeldSecret) -> Result<(), erasure::RecordFailure> {
            if !self.0.contains(item) {
                self.0.push(item.clone());
            }
            Ok(())
        }
        fn remove_reference(&mut self, _: &HeldSecret) -> Result<(), erasure::RecordFailure> {
            Ok(())
        }
        fn clear_reference_and_mark(&mut self, item: &HeldSecret) -> Result<(), erasure::RecordFailure> {
            self.0.retain(|held| held != item);
            Ok(())
        }
        fn marks(&self) -> Result<Vec<HeldSecret>, erasure::RecordFailure> {
            Ok(self.0.clone())
        }
    }

    /// **S1a: the name, and a 15-character candidate of the drawn shape that
    /// meets the published rule by construction.**
    #[test]
    fn every_candidate_has_the_drawn_shape_and_meets_the_published_rule() {
        let mut offer = TheOffer::made().expect("this computer gives random bytes");
        assert_eq!(offer.the_name(), "Smoozy");
        for _ in 0..200 {
            let username = offer.the_username();
            assert_eq!(username.len(), 15);
            assert!(username.starts_with("smoozy_") && username.ends_with("_bot"));
            let block = &username[7..11];
            assert!(block.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit()), "{username}");
            assert!(!username.contains("__"));
            assert!(meets_the_published_username_rule(&username));
            offer.another().unwrap();
        }
    }

    /// **S1b: *Another* never offers a block already offered**, and changes
    /// nothing but the block.
    #[test]
    fn another_never_repeats_a_block_already_offered() {
        // A source that gives "aaaa" twice, then "bbbb".
        let mut script: Vec<u8> = [0u8; 8].into_iter().chain([1u8; 4]).collect();
        script.reverse();
        let mut draw = |bytes: &mut [u8]| {
            for b in bytes.iter_mut() {
                *b = script.pop().ok_or(NoRandomBytes)?;
            }
            Ok(())
        };
        let mut offer = TheOffer::made_with(&mut draw).unwrap();
        assert_eq!(offer.the_username(), "smoozy_aaaa_bot");
        offer.another_with(&mut draw).unwrap();
        assert_eq!(offer.the_username(), "smoozy_bbbb_bot");
    }

    /// **S1a: a byte of 252 or more is thrown away**, so the draw is uniform.
    #[test]
    fn a_byte_that_would_bias_the_draw_is_thrown_away() {
        let mut script = vec![252u8, 255, 35, 36, 71, 0];
        script.reverse();
        let mut draw = |bytes: &mut [u8]| {
            bytes[0] = script.pop().ok_or(NoRandomBytes)?;
            Ok(())
        };
        let offer = TheOffer::made_with(&mut draw).unwrap();
        // 252 and 255 are thrown away; 35 is `9`, 36 is `a`, 71 is `9`, 0 is `a`.
        assert_eq!(offer.the_username(), "smoozy_9a9a_bot");
    }

    /// **S2: the token is found inside a longer paste**, alone, and on its own.
    #[test]
    fn the_token_is_found_inside_whatever_was_pasted() {
        assert!(a_reply_carrying(A_TOKEN).holds_a_token());
        assert!(ThePaste::what_the_user_pasted(A_TOKEN).holds_a_token());
        assert!(ThePaste::what_the_user_pasted(format!("  {A_TOKEN}\n")).holds_a_token());
        assert!(!ThePaste::what_the_user_pasted("Use this token to access the HTTP API:").holds_a_token());
        assert!(!ThePaste::what_the_user_pasted("").holds_a_token());
        // Two different tokens: choosing would be a guess.
        assert!(!ThePaste::what_the_user_pasted(format!("{A_TOKEN} 42:other")).holds_a_token());
        // The same one twice is one.
        assert!(ThePaste::what_the_user_pasted(format!("{A_TOKEN} and again {A_TOKEN}")).holds_a_token());
        // Nothing that could change the address it is put into.
        let paste = ThePaste::what_the_user_pasted("12:ab/../sendMessage");
        assert_eq!(paste.the_token_in_it().map(|t| t.0.clone()), Some("12:ab".to_string()));
    }

    /// **S2: the paste and the passed check show nothing of the token.**
    #[test]
    fn nothing_this_file_shows_carries_the_token() {
        let paste = a_reply_carrying(A_TOKEN);
        assert!(!format!("{paste:?}").contains("not_a_real_token"));
        let service = Arc::new(AStandInService::new());
        service.then_answers(&answered(7, "smoozy_a7k2_bot"));
        let found = check_the_bot_answers_through(&paste, &over(&service));
        assert!(!format!("{found:?}").contains("not_a_real_token"));
    }

    /// **S3 and S3a: answered, with the identity and username the answer
    /// carries, and the link built from that username.**
    #[test]
    fn an_answered_check_names_the_bot_the_answer_carries() {
        let service = Arc::new(AStandInService::new());
        service.then_answers(&answered(1_234_567_890, "studio_render_bot"));
        let WhatTheCheckFound::Answered(check) = check_the_bot_answers_through(&a_reply_carrying(A_TOKEN), &over(&service)) else {
            panic!("the check answered");
        };
        assert_eq!(check.the_bot(), &TheBot { id: 1_234_567_890, username: "studio_render_bot".into() });
        assert_eq!(check.the_link(), "https://t.me/studio_render_bot");
        assert_eq!(service.heard()[0].method, "getMe");
    }

    /// **S3: the 401 body is *not accepted*; nothing else is.** Every other
    /// outcome is an ordinary failure, and the paste is still there after
    /// every one of them.
    #[test]
    fn only_the_401_body_reads_as_a_token_not_accepted() {
        let paste = a_reply_carrying(A_TOKEN);
        let service = Arc::new(AStandInService::new());
        service
            .then_answers(r#"{"ok":false,"error_code":401,"description":"Unauthorized"}"#)
            .then_is_not_reached("connection refused")
            .then_answers(r#"{"ok":false,"error_code":429,"description":"Too Many Requests: retry after 1","parameters":{"retry_after":1}}"#)
            .then_answers("<html>502</html>")
            .then_answers(&json!({ "ok": true, "result": { "id": 7, "is_bot": true } }).to_string())
            .then_answers(&answered(7, "not a username"));

        assert!(matches!(check_the_bot_answers_through(&paste, &over(&service)), WhatTheCheckFound::NotAccepted));
        assert!(matches!(
            check_the_bot_answers_through(&paste, &over(&service)),
            WhatTheCheckFound::DidNotFinish(WhyTheCheckDidNotFinish::TelegramCouldNotBeReached)
        ));
        assert!(matches!(
            check_the_bot_answers_through(&paste, &over(&service)),
            WhatTheCheckFound::DidNotFinish(WhyTheCheckDidNotFinish::RefusedForAnotherReason { error_code: Some(429) })
        ));
        assert!(matches!(
            check_the_bot_answers_through(&paste, &over(&service)),
            WhatTheCheckFound::DidNotFinish(WhyTheCheckDidNotFinish::TheAnswerCouldNotBeRead)
        ));
        for _ in 0..2 {
            assert!(matches!(
                check_the_bot_answers_through(&paste, &over(&service)),
                WhatTheCheckFound::DidNotFinish(WhyTheCheckDidNotFinish::NoUsableBotAddressCameBack)
            ));
        }
        assert!(paste.holds_a_token(), "S3: the paste is kept for the retry");
    }

    /// **S2: with no token in the paste, Telegram is not asked at all.**
    #[test]
    fn a_paste_with_no_token_asks_nothing() {
        let service = Arc::new(AStandInService::new());
        let found = check_the_bot_answers_through(&ThePaste::what_the_user_pasted("hello"), &over(&service));
        assert!(matches!(found, WhatTheCheckFound::NoTokenInThePaste));
        assert!(service.heard().is_empty());
    }

    fn a_passed_check(service: &Arc<AStandInService>) -> TheCheckPassed {
        service.then_answers(&answered(7, "smoozy_a7k2_bot"));
        match check_the_bot_answers_through(&a_reply_carrying(A_TOKEN), &over(service)) {
            WhatTheCheckFound::Answered(check) => check,
            other => panic!("the check answered, not {other:?}"),
        }
    }

    /// **S5 and P4: the bot already proved gets no wait**, and its username is
    /// refreshed from this answer.
    #[test]
    fn a_token_for_the_bot_already_proved_is_never_waited_on() {
        let service = Arc::new(AStandInService::new());
        let proved = TheBot { id: 7, username: "an_old_name_bot".into() };
        match a_passed_check(&service).what_comes_next(Some(&proved)) {
            WhatComesNext::TheSameBotIsAlreadyProved { refreshed, .. } => {
                assert_eq!(refreshed.username, "smoozy_a7k2_bot")
            }
            other => panic!("no wait on a proved bot, not {other:?}"),
        }
        let another = TheBot { id: 99, username: "smoozy_a7k2_bot".into() };
        assert!(matches!(a_passed_check(&service).what_comes_next(Some(&another)), WhatComesNext::WaitForStart(_)));
        assert!(matches!(a_passed_check(&service).what_comes_next(None), WhatComesNext::WaitForStart(_)));
    }

    /// **S5: each ask confirms only what the one before it returned, the Start
    /// is never confirmed, and P3's unplaceable updates are not a Start.**
    #[test]
    fn the_wait_confirms_what_it_has_passed_and_leaves_the_start_alone() {
        let service = Arc::new(AStandInService::new());
        let WhatComesNext::WaitForStart(wait) = a_passed_check(&service).what_comes_next(None) else {
            panic!("a new bot waits");
        };
        service
            .then_answers(r#"{"ok":true,"result":[]}"#)
            .then_answers(&json!({ "ok": true, "result": [
                { "update_id": 40, "callback_query": { "id": "q", "data": "old" } },
                { "update_id": 41, "message": { "text": "no chat here" } } ] }).to_string())
            .then_answers(&json!({ "ok": true, "result": [
                { "update_id": 42, "message": { "message_id": 1, "chat": { "id": 9001 }, "text": "/start" } } ] }).to_string());

        let WhatOneAskFound::NothingYet(wait) = wait.ask_once(25) else { panic!("nothing yet") };
        assert_eq!(wait.confirmed_below(), None);
        let WhatOneAskFound::NothingYet(wait) = wait.ask_once(25) else { panic!("nothing placeable yet") };
        assert_eq!(wait.confirmed_below(), Some(42));
        let WhatOneAskFound::StartArrived(proof) = wait.ask_once(25) else { panic!("the Start arrived") };
        assert_eq!(proof.the_conversation(), TheConversation(9001));

        let asks: Vec<_> = service.heard().into_iter().filter(|call| call.method != "getMe").collect();
        assert_eq!(asks.len(), 3);
        let offset = |at: usize| match &asks[at].carries {
            WhatACallCarries::Fields(fields) => fields.get("offset").cloned(),
            _ => panic!("an ask carries fields"),
        };
        assert_eq!(offset(0), None, "the first ask confirms nothing");
        assert_eq!(offset(1), None, "nothing came back to pass");
        assert_eq!(offset(2), Some(json!(42)));
    }

    /// **An ask that fails hands the wait back**, to be asked again.
    #[test]
    fn a_failed_ask_hands_the_wait_back() {
        let service = Arc::new(AStandInService::new());
        let WhatComesNext::WaitForStart(wait) = a_passed_check(&service).what_comes_next(None) else {
            panic!("a new bot waits");
        };
        service.then_is_not_reached("the line dropped");
        assert!(matches!(wait.ask_once(25), WhatOneAskFound::TheAskFailed(_, WhyTheCallFailed::NotReached(_))));
    }

    fn a_proof(service: &Arc<AStandInService>) -> TheProof {
        let WhatComesNext::WaitForStart(wait) = a_passed_check(service).what_comes_next(None) else {
            panic!("a new bot waits");
        };
        service.then_answers(&json!({ "ok": true, "result": [
            { "update_id": 42, "message": { "chat": { "id": 9001 }, "text": "/start" } } ] }).to_string());
        match wait.ask_once(25) {
            WhatOneAskFound::StartArrived(proof) => proof,
            other => panic!("the Start arrived, not {other:?}"),
        }
    }

    /// **S6, S4, K1 and §19: hello back to the proved conversation, then the
    /// proof recorded, then the token stored under the fourth kind, then the
    /// act — with no token anywhere in its line.**
    #[test]
    fn hello_back_then_the_proof_then_the_token_then_the_act() {
        let service = Arc::new(AStandInService::new());
        let proof = a_proof(&service);
        service.then_answers(r#"{"ok":true,"result":{"message_id":2}}"#);
        let hello = proof.say_hello_back().expect("hello was said");
        let sent = service.heard().last().cloned().unwrap();
        assert_eq!(sent.method, "sendMessage");
        let WhatACallCarries::Fields(fields) = &sent.carries else { panic!("fields") };
        assert_eq!(fields["chat_id"], json!(9001));
        assert_eq!(fields["text"], json!(HELLO_BACK));

        let record = ARecordInMemory::new();
        let store = AStore::default();
        let set_up = hello.keep(&record, &store, &mut Marks::default()).expect("kept");
        assert_eq!(record.the_token_is_stored_for(), Some(7), "the stored fact is written after the save");
        assert_eq!(
            record.the_proved_bot().unwrap(),
            Some((TheBot { id: 7, username: "smoozy_a7k2_bot".into() }, TheConversation(9001)))
        );
        let stored = store
            .get(&record.the_item_for_saving(&set_up.bot).unwrap(), THE_STORE_CALLS_DEADLINE)
            .expect("the token is stored")
            .expose(|bytes| bytes.to_vec());
        assert_eq!(stored, A_TOKEN.as_bytes());
        assert_eq!(the_bot_tokens_item().kind, StoredKind::BotToken);

        let line = set_up.the_diary_act(SystemTime::UNIX_EPOCH + Duration::from_secs(1_789_300_000));
        assert_eq!(line, "a bot set up; 1789300000; @smoozy_a7k2_bot; conversation 9001");
        assert!(!line.contains("not_a_real_token") && !line.contains("123456789:"));
    }

    /// **S6: a hello that did not go hands the proof back, and nothing is
    /// kept.**
    #[test]
    fn a_hello_that_did_not_go_keeps_nothing() {
        let service = Arc::new(AStandInService::new());
        let proof = a_proof(&service);
        service.then_is_not_reached("the line dropped");
        let Err((proof, WhyTheCallFailed::NotReached(_))) = proof.say_hello_back() else {
            panic!("hello did not go");
        };
        assert_eq!(proof.the_conversation(), TheConversation(9001));
    }

    /// **S4: a record that did not take the proof stores no token; a store
    /// that refused leaves the proof recorded and the act offered again.**
    #[test]
    fn keeping_is_in_s4s_order_and_can_be_offered_again() {
        let service = Arc::new(AStandInService::new());
        let hello = a_proof(&service).say_hello_back().unwrap();

        let record = ARecordInMemory::new();
        record.refuses_every_write("the disk is full");
        let store = AStore::default();
        let Err((hello, WhyTheBotWasNotKept::TheRecordDidNotTakeTheProof(_))) = hello.keep(&record, &store, &mut Marks::default()) else {
            panic!("the record refused");
        };
        assert!(matches!(store.get(&the_bot_tokens_item(), THE_STORE_CALLS_DEADLINE), Err(GetFailure::NotThere)));

        let record = ARecordInMemory::new();
        *store.refuses.lock().unwrap() = true;
        let Err((hello, WhyTheBotWasNotKept::TheTokenWasNotStored(StoreFailure::LockedOrRefused))) = hello.keep(&record, &store, &mut Marks::default()) else {
            panic!("the store refused");
        };
        assert!(record.the_proved_bot().unwrap().is_some());
        assert_eq!(record.the_token_is_stored_for(), None, "no save, no stored fact");
        *store.refuses.lock().unwrap() = false;
        assert!(hello.keep(&record, &store, &mut Marks::default()).is_ok(), "a second keep finishes it");
    }

    /// **`Security and licence` §3.0 rule 5 as amended 2026-09-14, and fix-here item 3 of
    /// the round *setting it up, and what the desktop keeps*: every save that
    /// did not answer in time is *not confirmed*, whatever the record named
    /// before; only a refusal is *not stored*.**
    #[test]
    fn every_save_that_did_not_answer_is_not_confirmed_and_only_a_refusal_is_not_stored() {
        let service = Arc::new(AStandInService::new());
        let store = AStore::default();
        *store.does_not_answer_in_time.lock().unwrap() = true;

        let hello = a_proof(&service).say_hello_back().unwrap();
        let nothing_set_up = ARecordInMemory::new();
        let Err((hello, WhyTheBotWasNotKept::TheTokenWasNotConfirmed)) =
            hello.keep(&nothing_set_up, &store, &mut Marks::default())
        else {
            panic!("with nothing set up before, a save that did not answer is still not confirmed");
        };

        let set_up_before = ARecordInMemory::new();
        set_up_before
            .the_proof_was_recorded(&TheBot { id: 7, username: "smoozy_a7k2_bot".into() }, TheConversation(9001))
            .unwrap();
        let Err((hello, WhyTheBotWasNotKept::TheTokenWasNotConfirmed)) = hello.keep(&set_up_before, &store, &mut Marks::default()) else {
            panic!("over a held token, a save that did not answer is not confirmed");
        };

        *store.does_not_answer_in_time.lock().unwrap() = false;
        *store.refuses.lock().unwrap() = true;
        let Err((hello, WhyTheBotWasNotKept::TheTokenWasNotStored(StoreFailure::LockedOrRefused))) =
            hello.keep(&nothing_set_up, &store, &mut Marks::default())
        else {
            panic!("a refusal is an answer, and is not stored");
        };

        *store.refuses.lock().unwrap() = false;
        assert!(hello.keep(&set_up_before, &store, &mut Marks::default()).is_ok(), "keeping again still finishes it");
    }

    /// Old removal failures are debts, and do not block a separate save.
    #[test]
    fn keeping_leaves_an_old_erase_owed_and_saves_a_separate_item() {
        let service = Arc::new(AStandInService::new());
        let record = ARecordInMemory::new();
        let store = AStore::default();
        let old = the_bot_tokens_item();
        let mut marks = Marks(vec![old.clone()]);
        *store.erases_refused.lock().unwrap() = true;
        let hello = a_proof(&service).say_hello_back().unwrap();
        let set_up = hello.keep(&record, &store, &mut marks).unwrap();
        let new_item = record.the_item_for_saving(&set_up.bot).unwrap();
        assert_ne!(old, new_item);
        assert_eq!(marks.0, vec![old]);
        assert!(store.get(&new_item, THE_STORE_CALLS_DEADLINE).is_ok());
        assert_eq!(record.the_token_is_stored_for(), Some(7));
        *store.erases_refused.lock().unwrap() = false;
        let mut gate = erasure::SweepGate::for_a_run_whose_record_opened();
        gate.a_store_operation_succeeded();
        erasure::sweep(&gate, &mut marks, &store, THE_STORE_CALLS_DEADLINE);
        assert!(marks.0.is_empty());
        assert!(store.get(&new_item, THE_STORE_CALLS_DEADLINE).is_ok());
    }

    #[test]
    fn a_mark_that_would_not_clear_does_not_own_the_new_item() {
        use smoozy_security_and_licence::erasure::RecordFailure as EraseFailure;
        struct CannotClear(Marks);
        impl EraseMarks for CannotClear {
            fn mark_owing_an_erase(&mut self, item: &HeldSecret) -> Result<(), EraseFailure> {
                self.0.mark_owing_an_erase(item)
            }
            fn remove_reference(&mut self, item: &HeldSecret) -> Result<(), EraseFailure> {
                self.0.remove_reference(item)
            }
            fn clear_reference_and_mark(&mut self, _: &HeldSecret) -> Result<(), EraseFailure> {
                Err(EraseFailure)
            }
            fn marks(&self) -> Result<Vec<HeldSecret>, EraseFailure> { self.0.marks() }
        }
        let service = Arc::new(AStandInService::new());
        let record = ARecordInMemory::new();
        let store = AStore::default();
        let mut marks = CannotClear(Marks(vec![the_bot_tokens_item()]));
        let hello = a_proof(&service).say_hello_back().unwrap();
        let set_up = hello.keep(&record, &store, &mut marks).unwrap();
        let item = record.the_item_for_saving(&set_up.bot).unwrap();
        assert!(!marks.0.0.contains(&item));
        assert_eq!(marks.0.0, vec![the_bot_tokens_item()]);
        assert!(store.get(&item, THE_STORE_CALLS_DEADLINE).is_ok());
    }

    /// **fix-here item 4: hello back never says Telegram is set up**, because it is said
    /// before anything is kept.
    #[test]
    fn hello_back_says_only_that_the_chat_is_connected() {
        assert!(!HELLO_BACK.contains("set up"), "{HELLO_BACK}");
        assert!(HELLO_BACK.to_lowercase().contains("connected"), "{HELLO_BACK}");
        assert!(!HELLO_BACK.contains("Smoozy"), "{HELLO_BACK}");
    }
}
