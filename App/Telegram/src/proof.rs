//! **The conversation proof, and the filter every machine answers by —
//! `Telegram` §7 P1 to P5.**
//!
//! **P1: the proof is possession at setup.** The conversation the walkthrough's
//! Start arrived from is the user's, because the user made the bot, holds its
//! token, and pressed Start in it. The walkthrough that receives that Start is
//! [`crate::walkthrough`]; what this file holds is the shapes the proof is made
//! of, the one reading of which conversation an update came from, the filter,
//! and P4's rule for a replacement token.
//!
//! **P5 closes unresolved and nothing is built for it.** Whether a stranger can
//! reach the bot, and what the bot sees when they do, is answered on Telegram's
//! own pages and has never been produced here (`Telegram` §21 item 2). The
//! filter below is complete on Smoozy's side without it.
//!
//! **What this file relies on about the service**, from
//! `Preparation/Architecture/Sources/Telegram.sources.md` entry 20, **observed
//! in a test**: a private text message arrives as an `update_id` and a
//! `message` holding a `chat` with a numeric `id`; a button press arrives as a
//! `callback_query` carrying the whole of the bot's original message. Entry 15:
//! every update seen carried the conversation it came from.
//!
//! **Markers in this file are all `Telegram`'s**: P1, P2, P3, P4, P5, S3a, S5.

use std::sync::Mutex;

use serde_json::Value;

/// **One conversation, by the identifier every update carries** (P1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TheConversation(pub i64);

/// **The bot a token names, as `getMe` answered for it** — S3a's two fields.
/// The username is never the one Smoozy offered; it is the one the answer
/// carried.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheBot {
    pub id: i64,
    pub username: String,
}

/// **Which conversation an update came from, or nothing.**
///
/// The two kinds every ask names (P6a) are the two read: a `message` names its
/// conversation at `chat.id`, and a `callback_query` at the `chat.id` of the
/// message it carries. **Anything else, or either of those without a numeric
/// identifier there, is a conversation that cannot be established** — P3.
pub fn the_conversation_an_update_came_from(update: &Value) -> Option<TheConversation> {
    if let Some(message) = update.get("message") {
        return message
            .pointer("/chat/id")
            .and_then(Value::as_i64)
            .map(TheConversation);
    }
    if let Some(tap) = update.get("callback_query") {
        return tap
            .pointer("/message/chat/id")
            .and_then(Value::as_i64)
            .map(TheConversation);
    }
    None
}

/// **What an update gets from a machine or a desk listener.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAnUpdateGets {
    /// It came from the proved conversation.
    Answered,
    /// **P2 and P3: never answered, never logged with its content, never
    /// routed.** Answering would tell a stranger the bot is alive.
    Ignored,
}

/// **P2 and P3 — the filter, as one comparison.** An update from any other
/// conversation is ignored, and so is one whose conversation cannot be
/// established: the default is silence.
pub fn what_an_update_gets(update: &Value, proved: TheConversation) -> WhatAnUpdateGets {
    match the_conversation_an_update_came_from(update) {
        Some(conversation) if conversation == proved => WhatAnUpdateGets::Answered,
        _ => WhatAnUpdateGets::Ignored,
    }
}

/// **What a token for a bot already proved keeps — P4, with S3a's refresh.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatAReplacementKeeps {
    /// **The same bot: the proof stands and the walkthrough is not run
    /// again.** S3a: the username is taken from the new answer, never kept from
    /// the old one merely because the token was accepted.
    TheProofStands { refreshed: TheBot },
    /// **A different bot is a different conversation**, and the full proof
    /// runs again (S5).
    ANewBotNeedsANewProof,
}

/// **P4, decided by the bot's identity alone.** S3a orders it: both fields are
/// refreshed from the answer, then the identity is compared.
pub fn what_a_replacement_keeps(proved: &TheBot, answered: &TheBot) -> WhatAReplacementKeeps {
    if proved.id == answered.id {
        WhatAReplacementKeeps::TheProofStands {
            refreshed: answered.clone(),
        }
    } else {
        WhatAReplacementKeeps::ANewBotNeedsANewProof
    }
}

/// **Why the local record did not take or give back the proof**, in the
/// record's own words. The record is the desktop's, so its failure is carried
/// here as text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ARecordFailure(pub String);

/// **Where P1's proof is recorded: the local record, against the bot.** S3a
/// names what is written — the bot's identity and username beside the proved
/// conversation. The real one is the desktop's; this crate's tests use
/// [`a_stand_in::ARecordInMemory`].
pub trait KeepsTheProvedBot {
    /// The bot and its proved conversation, or nothing where no walkthrough
    /// has finished. A read that did not answer is a failure, never nothing.
    fn the_proved_bot(&self) -> Result<Option<(TheBot, TheConversation)>, ARecordFailure>;

    /// **P1: record the proof against the bot**, replacing whatever bot was
    /// there — a new bot is a new proof (P4).
    ///
    /// **`Telegram` L4, as amended 2026-09-14: where a bot is on record and
    /// this one is a different bot, the token generation moves on in the same
    /// step**, so every machine told the old generation is named as not
    /// reached (L3), and the record's fact that a token is stored is cleared,
    /// since it named the bot that left. At a first setup, with no bot on
    /// record, nothing moves.
    fn the_proof_was_recorded(
        &self,
        bot: &TheBot,
        conversation: TheConversation,
    ) -> Result<(), ARecordFailure>;

    /// **The token was stored, for this bot.** Written only after a save
    /// succeeded, and the one thing a *token held* reading is drawn from
    /// (`Security and licence` §3.0 rule 6: *"That an item is remembered is
    /// answered from the local record's own subject"*).
    fn the_token_was_stored_for(&self, bot: &TheBot) -> Result<(), ARecordFailure>;

    /// Reserve a separate item before saving. An item owed to cleanup is
    /// never reused, even when this is the same bot again.
    fn the_item_for_saving(&self, bot: &TheBot) -> Result<smoozy_security_and_licence::held::HeldSecret, ARecordFailure>;
}

/// A record held in memory, for tests of this crate and of later pieces.
pub mod a_stand_in {
    use super::*;

    #[derive(Default)]
    pub struct ARecordInMemory {
        held: Mutex<Option<(TheBot, TheConversation)>>,
        refuses: Mutex<Option<String>>,
        stored_for: Mutex<Option<i64>>,
        item: Mutex<Option<smoozy_security_and_licence::held::HeldSecret>>,
    }

    impl ARecordInMemory {
        pub fn new() -> ARecordInMemory {
            ARecordInMemory::default()
        }

        /// The bot the record says a token was stored for, if any.
        pub fn the_token_is_stored_for(&self) -> Option<i64> {
            *self.stored_for.lock().unwrap_or_else(|p| p.into_inner())
        }

        /// Every write from now on is refused with these words.
        pub fn refuses_every_write(&self, why: &str) {
            *self.refuses.lock().unwrap_or_else(|p| p.into_inner()) = Some(why.to_string());
        }
    }

    impl KeepsTheProvedBot for ARecordInMemory {
        fn the_proved_bot(&self) -> Result<Option<(TheBot, TheConversation)>, ARecordFailure> {
            Ok(self.held.lock().unwrap_or_else(|p| p.into_inner()).clone())
        }

        fn the_proof_was_recorded(
            &self,
            bot: &TheBot,
            conversation: TheConversation,
        ) -> Result<(), ARecordFailure> {
            if let Some(why) = self.refuses.lock().unwrap_or_else(|p| p.into_inner()).clone() {
                return Err(ARecordFailure(why));
            }
            let mut held = self.held.lock().unwrap_or_else(|p| p.into_inner());
            // L4 as amended: a different bot's proof clears the stored fact.
            // The generation is the desktop record's and is not kept here.
            if held.as_ref().is_some_and(|(before, _)| before.id != bot.id) {
                *self.stored_for.lock().unwrap_or_else(|p| p.into_inner()) = None;
            }
            *held = Some((bot.clone(), conversation));
            Ok(())
        }

        fn the_item_for_saving(&self, _: &TheBot) -> Result<smoozy_security_and_licence::held::HeldSecret, ARecordFailure> {
            let mut item = self.item.lock().unwrap_or_else(|p| p.into_inner());
            if item.is_none() {
                *item = Some(crate::walkthrough::a_fresh_bot_tokens_item()?);
            }
            Ok(item.clone().unwrap())
        }

        fn the_token_was_stored_for(&self, bot: &TheBot) -> Result<(), ARecordFailure> {
            if let Some(why) = self.refuses.lock().unwrap_or_else(|p| p.into_inner()).clone() {
                return Err(ARecordFailure(why));
            }
            *self.stored_for.lock().unwrap_or_else(|p| p.into_inner()) = Some(bot.id);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const OURS: TheConversation = TheConversation(9_001);

    /// **Entry 20's shape, read the way P1 needs it.**
    #[test]
    fn a_message_and_a_tap_each_name_their_conversation() {
        let message = json!({ "update_id": 1, "message": {
            "message_id": 4, "from": { "id": 9001 }, "chat": { "id": 9001 }, "date": 0, "text": "/start" } });
        let tap = json!({ "update_id": 2, "callback_query": {
            "id": "x", "from": { "id": 9001 }, "data": "d",
            "message": { "message_id": 5, "chat": { "id": 9001 } } } });
        assert_eq!(the_conversation_an_update_came_from(&message), Some(OURS));
        assert_eq!(the_conversation_an_update_came_from(&tap), Some(OURS));
    }

    /// **P2: any other conversation is ignored.**
    #[test]
    fn an_update_from_another_conversation_is_ignored() {
        let stranger = json!({ "update_id": 3, "message": { "chat": { "id": 42 }, "text": "hi" } });
        assert_eq!(what_an_update_gets(&stranger, OURS), WhatAnUpdateGets::Ignored);
        let ours = json!({ "update_id": 4, "message": { "chat": { "id": 9001 }, "text": "hi" } });
        assert_eq!(what_an_update_gets(&ours, OURS), WhatAnUpdateGets::Answered);
    }

    /// **P3: a conversation that cannot be established is silence**, whatever
    /// shape the update came in.
    #[test]
    fn an_update_whose_conversation_cannot_be_established_is_ignored() {
        for shape in [
            json!({ "update_id": 5 }),
            json!({ "update_id": 6, "message": { "text": "no chat" } }),
            json!({ "update_id": 7, "message": { "chat": { "id": "9001" } } }),
            json!({ "update_id": 8, "callback_query": { "id": "x", "data": "d" } }),
            json!({ "update_id": 9, "edited_message": { "chat": { "id": 9001 } } }),
        ] {
            assert_eq!(what_an_update_gets(&shape, OURS), WhatAnUpdateGets::Ignored, "{shape}");
        }
    }

    /// **P4 and S3a: the same bot keeps the proof and takes the new username;
    /// a different bot needs a new proof.**
    #[test]
    fn a_replacement_for_the_same_bot_keeps_the_proof_and_refreshes_the_username() {
        let proved = TheBot { id: 7, username: "smoozy_a7k2_bot".into() };
        let renamed = TheBot { id: 7, username: "smoozy_zz99_bot".into() };
        assert_eq!(
            what_a_replacement_keeps(&proved, &renamed),
            WhatAReplacementKeeps::TheProofStands { refreshed: renamed.clone() }
        );
        let other = TheBot { id: 8, username: "smoozy_a7k2_bot".into() };
        assert_eq!(
            what_a_replacement_keeps(&proved, &other),
            WhatAReplacementKeeps::ANewBotNeedsANewProof,
            "the same username on a different bot is a different bot"
        );
    }
}
