//! **The Telegram worker's state — `Telegram` F2, and `State, identity and
//! recovery` §1.2's store of that name — as a trait, with two stand-ins.**
//!
//! **The real store is not this crate's.** F2: the worker writes no durable
//! store itself; every durable fact it holds is written by the supervisor on the
//! worker's request, through the ordinary command path. That is step 10c's
//! machine half. **So nothing in this crate may assume the store is a file the
//! listener owns**: the listener reaches it only through
//! [`TheTelegramWorkersState`], and the two stand-ins here are for tests and for
//! a desk process standing in for a machine.
//!
//! This trait holds what piece 2 needs: the update position, the board and
//! machinery message identities, the machine-routing key, and whether an
//! action-context record is bound to a message (B8's third recognition). **The
//! action-context records themselves (`Telegram` R1) were added by piece 3**,
//! at the end of the trait. The journal cursor and the status-message identity
//! are piece 6's to add.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::routing::context::AnActionContextRecord;

/// A store that could not be read or written. **The act that needed it does not
/// happen**, and nothing that depends on it is confirmed (F3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhyTheStoreFailed(pub String);

/// **`State, identity and recovery` §1.2 — the Telegram worker's state**, as far
/// as the slot and the board read and write it.
pub trait TheTelegramWorkersState: Send {
    /// **`Telegram` F3** — the next update identity to ask from, or nothing
    /// where nothing has been confirmed.
    fn the_update_position(&self) -> Result<Option<i64>, WhyTheStoreFailed>;
    /// Written only after the work for every earlier update is done (F3).
    fn record_the_update_position(&mut self, next: i64) -> Result<(), WhyTheStoreFailed>;
    /// **`Telegram` B3** — the identity of the board this machine last created.
    fn the_board(&self) -> Result<Option<i64>, WhyTheStoreFailed>;
    fn record_the_board(&mut self, message: i64) -> Result<(), WhyTheStoreFailed>;
    /// **`Telegram` B10** — this machine's one machinery message.
    fn the_machinery_message(&self) -> Result<Option<i64>, WhyTheStoreFailed>;
    fn record_the_machinery_message(&mut self, message: i64) -> Result<(), WhyTheStoreFailed>;
    /// **`Telegram` B8 and R1** — this machine's 16-byte routing key, written
    /// as text. Issued by the supervisor (`State, identity and recovery` §2.2).
    fn the_routing_key(&self) -> Result<String, WhyTheStoreFailed>;
    /// **`Telegram` B8** — whether an action-context record here is bound to
    /// this source message.
    fn holds_a_context_record_bound_to(&self, source_message: i64) -> Result<bool, WhyTheStoreFailed>;

    // Piece 3 — `Telegram` R1's authoritative action-context records, keyed by
    // their 16-byte action-context identities (`State, identity and recovery`
    // §1.2). Written before the button that carries the token exists.

    /// Record one action context durably, replacing any under the same identity.
    fn record_an_action_context(&mut self, record: AnActionContextRecord) -> Result<(), WhyTheStoreFailed>;
    /// The record under one action-context identity, or nothing.
    fn the_action_context(&self, identity: &str) -> Result<Option<AnActionContextRecord>, WhyTheStoreFailed>;
    /// Durable contexts, including Stops on earlier messages (C1b).
    fn the_action_contexts(&self) -> Result<Vec<AnActionContextRecord>, WhyTheStoreFailed>;
    /// **`Telegram` R1a** — every record bound to one source message.
    fn the_action_contexts_bound_to(&self, source_message: i64) -> Result<Vec<AnActionContextRecord>, WhyTheStoreFailed>;
    /// **`Telegram` R1 — records are subject-held**: removed when their message
    /// is superseded or their subject generation ends. Gives back how many.
    fn remove_action_contexts_where(
        &mut self,
        which: &dyn Fn(&AnActionContextRecord) -> bool,
    ) -> Result<usize, WhyTheStoreFailed>;

    // Piece 6 — `Telegram` F2's journal cursor, and C7's status-message
    // identity (`State, identity and recovery` §1.2 names both).

    /// **`Telegram` F3** — the next journal sequence number to read, or
    /// nothing where nothing has been read. Written only after the work for
    /// every earlier event is done.
    fn the_journal_cursor(&self) -> Result<Option<u64>, WhyTheStoreFailed>;
    fn record_the_journal_cursor(&mut self, next: u64) -> Result<(), WhyTheStoreFailed>;
    /// **`Telegram` C7** — this machine's one status message, re-used across
    /// every restart.
    fn the_status_message(&self) -> Result<Option<i64>, WhyTheStoreFailed>;
    fn record_the_status_message(&mut self, message: i64) -> Result<(), WhyTheStoreFailed>;

    // The Telegram discussion, the server half (2026-10-31) — `Telegram` R1b's
    // register of sent messages and V1d's *phone's choice*. **Defaults read
    // as nothing and refuse to write**, so a store that keeps neither says so
    // rather than losing a line quietly; every store a machine or a desk uses
    // keeps both.

    /// **`Telegram` R1b — one line for a message Telegram accepted**, replacing
    /// any line under the same message. Over [`THE_MOST_LINES_THE_REGISTER_HOLDS`]
    /// the oldest message's line leaves.
    fn register_a_sent_message(&mut self, line: ASentMessage) -> Result<(), WhyTheStoreFailed> {
        let _ = line;
        Err(WhyTheStoreFailed("this store keeps no register of sent messages".into()))
    }
    /// **`Telegram` R1b — the line for one message**, or nothing.
    fn the_sent_message(&self, message: i64) -> Result<Option<ASentMessage>, WhyTheStoreFailed> {
        let _ = message;
        Ok(None)
    }
    /// **`Telegram` R1b — every line**, oldest message first.
    fn the_sent_messages(&self) -> Result<Vec<ASentMessage>, WhyTheStoreFailed> {
        Ok(Vec::new())
    }
    /// **`Telegram` R1b — lines removed**: a job's, once its record folder is
    /// gone. Gives back how many.
    fn forget_sent_messages_where(&mut self, which: &dyn Fn(&ASentMessage) -> bool) -> Result<usize, WhyTheStoreFailed> {
        let _ = which;
        Ok(0)
    }
    /// **`Telegram` V1d — the phone's choice**, the latest this machine heard.
    fn the_phones_choice(&self) -> Result<Option<ThePhonesChoice>, WhyTheStoreFailed> {
        Ok(None)
    }
    /// **`Telegram` V1d — the phone's choice written, replacing the one held
    /// only where it is later** ([`ThePhonesChoice::is_later_than`]). Gives
    /// back whether it was written.
    fn record_the_phones_choice(&mut self, choice: ThePhonesChoice) -> Result<bool, WhyTheStoreFailed> {
        let _ = choice;
        Err(WhyTheStoreFailed("this store keeps no phone's choice".into()))
    }

    /// **`Telegram` B8's third recognition, widened by R1a's widening and
    /// R1b**: this machine speaks for a message it holds an action-context
    /// record bound to, a register line about a job for, or that is its own
    /// status message (C7).
    fn speaks_for_the_message(&self, message: i64) -> bool {
        self.holds_a_context_record_bound_to(message).unwrap_or(false)
            || matches!(self.the_sent_message(message), Ok(Some(ASentMessage { about: WhatAMessageIsAbout::AJob { .. }, .. })))
            || self.the_status_message().ok().flatten() == Some(message)
    }
}

/// **`Telegram` R1b's bound on the register**: 2,000 lines, the oldest message
/// leaving first. A Build figure: a line is a few dozen bytes, and 2,000
/// messages is days of a busy fleet's conversation.
pub const THE_MOST_LINES_THE_REGISTER_HOLDS: usize = 2_000;

/// **`Telegram` R1b — one message this machine sent, and what it is about.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ASentMessage {
    pub message: i64,
    pub conversation: i64,
    pub about: WhatAMessageIsAbout,
}

/// **`Telegram` R1b's three kinds of line.** The status message needs none:
/// its identity is C7's and it is about the job it shows now.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhatAMessageIsAbout {
    /// A job and, where known, the attempt's generation; for a frame's
    /// picture, the frame and its View Layer.
    AJob {
        job: String,
        #[serde(default)]
        generation: Option<String>,
        #[serde(default)]
        frame: Option<u64>,
        #[serde(default)]
        layer: Option<String>,
    },
    /// The listener's own: *Which job?*, a status reply, a refusal, an ask.
    NoJob,
}

/// **`Telegram` V1d — the phone's choice**: the value in the manifest's own
/// form, the operation's identity, Telegram's `date` for the message or the
/// ask a tap was on, and the update's identity, which orders two choices of
/// one `date`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThePhonesChoice {
    pub value: serde_json::Value,
    pub operation: String,
    pub date: i64,
    pub update: i64,
}

impl ThePhonesChoice {
    /// **Later by Telegram's own `date`, and on one `date` by the update's
    /// identity**, which Telegram numbers in order. Both are Telegram's, so
    /// every machine and the desktop order two choices alike (`State,
    /// identity and recovery` D5: nothing compared across Smoozy's clocks).
    pub fn is_later_than(&self, other: &ThePhonesChoice) -> bool {
        (self.date, self.update) > (other.date, other.update)
    }
}

/// **The register's bound, kept by every store the same way.**
pub fn the_register_kept_within_its_bound<T>(lines: &mut BTreeMap<i64, T>) {
    while lines.len() > THE_MOST_LINES_THE_REGISTER_HOLDS {
        let Some(oldest) = lines.keys().next().copied() else { break };
        lines.remove(&oldest);
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
struct WhatIsHeld {
    update_position: Option<i64>,
    board: Option<i64>,
    machinery_message: Option<i64>,
    routing_key: String,
    context_records_bound_to: BTreeSet<i64>,
    #[serde(default)]
    action_contexts: BTreeMap<String, AnActionContextRecord>,
    #[serde(default)]
    journal_cursor: Option<u64>,
    #[serde(default)]
    status_message: Option<i64>,
    #[serde(default)]
    sent_messages: BTreeMap<i64, ASentMessage>,
    #[serde(default)]
    phones_choice: Option<ThePhonesChoice>,
}

impl WhatIsHeld {
    fn is_bound_to(&self, source_message: i64) -> bool {
        self.context_records_bound_to.contains(&source_message)
            || self.action_contexts.values().any(|r| r.source_message == Some(source_message))
    }

    fn bound_to(&self, source_message: i64) -> Vec<AnActionContextRecord> {
        self.action_contexts.values().filter(|r| r.source_message == Some(source_message)).cloned().collect()
    }

    fn remove_where(&mut self, which: &dyn Fn(&AnActionContextRecord) -> bool) -> usize {
        let before = self.action_contexts.len();
        self.action_contexts.retain(|_, r| !which(r));
        before - self.action_contexts.len()
    }

    fn register(&mut self, line: ASentMessage) {
        self.sent_messages.insert(line.message, line);
        the_register_kept_within_its_bound(&mut self.sent_messages);
    }

    fn forget_where(&mut self, which: &dyn Fn(&ASentMessage) -> bool) -> usize {
        let before = self.sent_messages.len();
        self.sent_messages.retain(|_, line| !which(line));
        before - self.sent_messages.len()
    }

    fn the_phones_choice_written(&mut self, choice: ThePhonesChoice) -> bool {
        if self.phones_choice.as_ref().is_some_and(|held| !choice.is_later_than(held)) {
            return false;
        }
        self.phones_choice = Some(choice);
        true
    }
}

/// **A stand-in held in memory.** For tests.
#[derive(Debug, Clone, Default)]
pub struct InMemory {
    held: WhatIsHeld,
}

impl InMemory {
    pub fn with_routing_key(routing_key: &str) -> InMemory {
        InMemory { held: WhatIsHeld { routing_key: routing_key.to_string(), ..WhatIsHeld::default() } }
    }

    /// Stands in for piece 3's issuing of an action-context record.
    pub fn a_context_record_bound_to(&mut self, source_message: i64) {
        self.held.context_records_bound_to.insert(source_message);
    }
}

impl TheTelegramWorkersState for InMemory {
    fn the_update_position(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        Ok(self.held.update_position)
    }
    fn record_the_update_position(&mut self, next: i64) -> Result<(), WhyTheStoreFailed> {
        self.held.update_position = Some(next);
        Ok(())
    }
    fn the_board(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        Ok(self.held.board)
    }
    fn record_the_board(&mut self, message: i64) -> Result<(), WhyTheStoreFailed> {
        self.held.board = Some(message);
        Ok(())
    }
    fn the_machinery_message(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        Ok(self.held.machinery_message)
    }
    fn record_the_machinery_message(&mut self, message: i64) -> Result<(), WhyTheStoreFailed> {
        self.held.machinery_message = Some(message);
        Ok(())
    }
    fn the_routing_key(&self) -> Result<String, WhyTheStoreFailed> {
        Ok(self.held.routing_key.clone())
    }
    fn holds_a_context_record_bound_to(&self, source_message: i64) -> Result<bool, WhyTheStoreFailed> {
        Ok(self.held.is_bound_to(source_message))
    }
    fn record_an_action_context(&mut self, record: AnActionContextRecord) -> Result<(), WhyTheStoreFailed> {
        self.held.action_contexts.insert(record.identity.clone(), record);
        Ok(())
    }
    fn the_action_context(&self, identity: &str) -> Result<Option<AnActionContextRecord>, WhyTheStoreFailed> {
        Ok(self.held.action_contexts.get(identity).cloned())
    }
    fn the_action_contexts(&self) -> Result<Vec<AnActionContextRecord>, WhyTheStoreFailed> {
        Ok(self.held.action_contexts.values().cloned().collect())
    }
    fn the_action_contexts_bound_to(&self, source_message: i64) -> Result<Vec<AnActionContextRecord>, WhyTheStoreFailed> {
        Ok(self.held.bound_to(source_message))
    }
    fn remove_action_contexts_where(
        &mut self,
        which: &dyn Fn(&AnActionContextRecord) -> bool,
    ) -> Result<usize, WhyTheStoreFailed> {
        Ok(self.held.remove_where(which))
    }
    fn the_journal_cursor(&self) -> Result<Option<u64>, WhyTheStoreFailed> {
        Ok(self.held.journal_cursor)
    }
    fn record_the_journal_cursor(&mut self, next: u64) -> Result<(), WhyTheStoreFailed> {
        self.held.journal_cursor = Some(next);
        Ok(())
    }
    fn the_status_message(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        Ok(self.held.status_message)
    }
    fn record_the_status_message(&mut self, message: i64) -> Result<(), WhyTheStoreFailed> {
        self.held.status_message = Some(message);
        Ok(())
    }
    fn register_a_sent_message(&mut self, line: ASentMessage) -> Result<(), WhyTheStoreFailed> {
        self.held.register(line);
        Ok(())
    }
    fn the_sent_message(&self, message: i64) -> Result<Option<ASentMessage>, WhyTheStoreFailed> {
        Ok(self.held.sent_messages.get(&message).cloned())
    }
    fn the_sent_messages(&self) -> Result<Vec<ASentMessage>, WhyTheStoreFailed> {
        Ok(self.held.sent_messages.values().cloned().collect())
    }
    fn forget_sent_messages_where(&mut self, which: &dyn Fn(&ASentMessage) -> bool) -> Result<usize, WhyTheStoreFailed> {
        Ok(self.held.forget_where(which))
    }
    fn the_phones_choice(&self) -> Result<Option<ThePhonesChoice>, WhyTheStoreFailed> {
        Ok(self.held.phones_choice.clone())
    }
    fn record_the_phones_choice(&mut self, choice: ThePhonesChoice) -> Result<bool, WhyTheStoreFailed> {
        Ok(self.held.the_phones_choice_written(choice))
    }
}

/// **A stand-in kept in one file**, for a desk process that must survive being
/// killed. Every write replaces the whole file through a sibling and a rename,
/// so a kill leaves the old content or the new, never half of one.
#[derive(Debug, Clone)]
pub struct AFileStandIn {
    path: PathBuf,
}

impl AFileStandIn {
    /// Opens the file, writing it with this routing key where it does not exist.
    pub fn at(path: PathBuf, routing_key: &str) -> Result<AFileStandIn, WhyTheStoreFailed> {
        let store = AFileStandIn { path };
        if !store.path.exists() {
            store.write(&WhatIsHeld { routing_key: routing_key.to_string(), ..WhatIsHeld::default() })?;
        }
        Ok(store)
    }

    fn read(&self) -> Result<WhatIsHeld, WhyTheStoreFailed> {
        let text = std::fs::read_to_string(&self.path).map_err(|e| WhyTheStoreFailed(e.to_string()))?;
        serde_json::from_str(&text).map_err(|e| WhyTheStoreFailed(e.to_string()))
    }

    fn write(&self, held: &WhatIsHeld) -> Result<(), WhyTheStoreFailed> {
        let text = serde_json::to_vec(held).map_err(|e| WhyTheStoreFailed(e.to_string()))?;
        let beside = self.path.with_extension("writing");
        let failed = |e: std::io::Error| WhyTheStoreFailed(e.to_string());
        {
            use std::io::Write as _;
            let mut file = std::fs::File::create(&beside).map_err(failed)?;
            file.write_all(&text).map_err(failed)?;
            file.sync_all().map_err(failed)?;
        }
        std::fs::rename(&beside, &self.path).map_err(failed)
    }

    fn change(&mut self, how: impl FnOnce(&mut WhatIsHeld)) -> Result<(), WhyTheStoreFailed> {
        let mut held = self.read()?;
        how(&mut held);
        self.write(&held)
    }
}

impl TheTelegramWorkersState for AFileStandIn {
    fn the_update_position(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        Ok(self.read()?.update_position)
    }
    fn record_the_update_position(&mut self, next: i64) -> Result<(), WhyTheStoreFailed> {
        self.change(|h| h.update_position = Some(next))
    }
    fn the_board(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        Ok(self.read()?.board)
    }
    fn record_the_board(&mut self, message: i64) -> Result<(), WhyTheStoreFailed> {
        self.change(|h| h.board = Some(message))
    }
    fn the_machinery_message(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        Ok(self.read()?.machinery_message)
    }
    fn record_the_machinery_message(&mut self, message: i64) -> Result<(), WhyTheStoreFailed> {
        self.change(|h| h.machinery_message = Some(message))
    }
    fn the_routing_key(&self) -> Result<String, WhyTheStoreFailed> {
        Ok(self.read()?.routing_key)
    }
    fn holds_a_context_record_bound_to(&self, source_message: i64) -> Result<bool, WhyTheStoreFailed> {
        Ok(self.read()?.is_bound_to(source_message))
    }
    fn record_an_action_context(&mut self, record: AnActionContextRecord) -> Result<(), WhyTheStoreFailed> {
        self.change(|h| {
            h.action_contexts.insert(record.identity.clone(), record);
        })
    }
    fn the_action_context(&self, identity: &str) -> Result<Option<AnActionContextRecord>, WhyTheStoreFailed> {
        Ok(self.read()?.action_contexts.get(identity).cloned())
    }
    fn the_action_contexts(&self) -> Result<Vec<AnActionContextRecord>, WhyTheStoreFailed> {
        Ok(self.read()?.action_contexts.into_values().collect())
    }
    fn the_action_contexts_bound_to(&self, source_message: i64) -> Result<Vec<AnActionContextRecord>, WhyTheStoreFailed> {
        Ok(self.read()?.bound_to(source_message))
    }
    fn remove_action_contexts_where(
        &mut self,
        which: &dyn Fn(&AnActionContextRecord) -> bool,
    ) -> Result<usize, WhyTheStoreFailed> {
        let mut removed = 0;
        self.change(|h| removed = h.remove_where(which))?;
        Ok(removed)
    }
    fn the_journal_cursor(&self) -> Result<Option<u64>, WhyTheStoreFailed> {
        Ok(self.read()?.journal_cursor)
    }
    fn record_the_journal_cursor(&mut self, next: u64) -> Result<(), WhyTheStoreFailed> {
        self.change(|h| h.journal_cursor = Some(next))
    }
    fn the_status_message(&self) -> Result<Option<i64>, WhyTheStoreFailed> {
        Ok(self.read()?.status_message)
    }
    fn record_the_status_message(&mut self, message: i64) -> Result<(), WhyTheStoreFailed> {
        self.change(|h| h.status_message = Some(message))
    }
    fn register_a_sent_message(&mut self, line: ASentMessage) -> Result<(), WhyTheStoreFailed> {
        self.change(|h| h.register(line))
    }
    fn the_sent_message(&self, message: i64) -> Result<Option<ASentMessage>, WhyTheStoreFailed> {
        Ok(self.read()?.sent_messages.get(&message).cloned())
    }
    fn the_sent_messages(&self) -> Result<Vec<ASentMessage>, WhyTheStoreFailed> {
        Ok(self.read()?.sent_messages.into_values().collect())
    }
    fn forget_sent_messages_where(&mut self, which: &dyn Fn(&ASentMessage) -> bool) -> Result<usize, WhyTheStoreFailed> {
        let mut forgotten = 0;
        self.change(|h| forgotten = h.forget_where(which))?;
        Ok(forgotten)
    }
    fn the_phones_choice(&self) -> Result<Option<ThePhonesChoice>, WhyTheStoreFailed> {
        Ok(self.read()?.phones_choice)
    }
    fn record_the_phones_choice(&mut self, choice: ThePhonesChoice) -> Result<bool, WhyTheStoreFailed> {
        let mut written = false;
        self.change(|h| written = h.the_phones_choice_written(choice))?;
        Ok(written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exercise(store: &mut dyn TheTelegramWorkersState) {
        assert_eq!(store.the_update_position().unwrap(), None);
        store.record_the_update_position(812).unwrap();
        store.record_the_board(40).unwrap();
        store.record_the_machinery_message(41).unwrap();
        assert_eq!(store.the_update_position().unwrap(), Some(812));
        assert_eq!(store.the_board().unwrap(), Some(40));
        assert_eq!(store.the_machinery_message().unwrap(), Some(41));
        assert_eq!(store.the_routing_key().unwrap(), "00112233445566778899aabbccddeeff");
        assert!(!store.holds_a_context_record_bound_to(5).unwrap());
        assert_eq!(store.the_journal_cursor().unwrap(), None);
        assert_eq!(store.the_status_message().unwrap(), None);
        store.record_the_journal_cursor(19).unwrap();
        store.record_the_status_message(52).unwrap();
        assert_eq!(store.the_journal_cursor().unwrap(), Some(19));
        assert_eq!(store.the_status_message().unwrap(), Some(52));
    }

    /// **`Telegram` R1b and V1d — the register and the phone's choice**, in
    /// every store the same way.
    fn the_register_and_the_phones_choice(store: &mut dyn TheTelegramWorkersState) {
        let about = |job: &str| WhatAMessageIsAbout::AJob { job: job.into(), generation: Some("g1".into()), frame: Some(7), layer: None };
        store.register_a_sent_message(ASentMessage { message: 60, conversation: 9, about: about("job-1") }).unwrap();
        store.register_a_sent_message(ASentMessage { message: 61, conversation: 9, about: WhatAMessageIsAbout::NoJob }).unwrap();
        assert_eq!(store.the_sent_message(60).unwrap().unwrap().about, about("job-1"));
        assert!(store.speaks_for_the_message(60), "a job's line speaks for its message");
        assert!(!store.speaks_for_the_message(61), "a line about no job speaks for nothing");
        assert!(store.speaks_for_the_message(52), "the status message is this machine's");
        assert_eq!(store.forget_sent_messages_where(&|line| line.message == 60).unwrap(), 1);
        assert_eq!(store.the_sent_message(60).unwrap(), None);
        let choice = |date: i64, update: i64| ThePhonesChoice { value: serde_json::json!("Nothing"), operation: format!("op-{date}-{update}"), date, update };
        assert!(store.record_the_phones_choice(choice(100, 5)).unwrap());
        assert!(!store.record_the_phones_choice(choice(99, 9)).unwrap(), "an earlier date is not written");
        assert!(!store.record_the_phones_choice(choice(100, 5)).unwrap(), "the same choice again changes nothing");
        assert!(store.record_the_phones_choice(choice(100, 6)).unwrap(), "one date, a later update");
        assert_eq!(store.the_phones_choice().unwrap().unwrap().operation, "op-100-6");
    }

    #[test]
    fn the_register_holds_at_most_its_bound_and_the_oldest_leaves() {
        let mut store = InMemory::with_routing_key("00112233445566778899aabbccddeeff");
        for message in 1..=(THE_MOST_LINES_THE_REGISTER_HOLDS as i64 + 3) {
            store.register_a_sent_message(ASentMessage { message, conversation: 9, about: WhatAMessageIsAbout::NoJob }).unwrap();
        }
        let lines = store.the_sent_messages().unwrap();
        assert_eq!(lines.len(), THE_MOST_LINES_THE_REGISTER_HOLDS);
        assert_eq!(lines.first().unwrap().message, 4);
    }

    #[test]
    fn the_memory_stand_in_keeps_what_it_is_given() {
        let mut store = InMemory::with_routing_key("00112233445566778899aabbccddeeff");
        exercise(&mut store);
        the_register_and_the_phones_choice(&mut store);
        store.a_context_record_bound_to(5);
        assert!(store.holds_a_context_record_bound_to(5).unwrap());
    }

    #[test]
    fn the_file_stand_in_survives_being_opened_again() {
        let folder = std::env::temp_dir().join(format!("smoozy-telegram-state-{}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        let path = folder.join("state.json");
        let _ = std::fs::remove_file(&path);
        let mut store = AFileStandIn::at(path.clone(), "00112233445566778899aabbccddeeff").unwrap();
        exercise(&mut store);
        the_register_and_the_phones_choice(&mut store);
        let again = AFileStandIn::at(path, "a different key is not written over the first").unwrap();
        assert_eq!(again.the_board().unwrap(), Some(40));
        assert_eq!(again.the_routing_key().unwrap(), "00112233445566778899aabbccddeeff");
        std::fs::remove_dir_all(&folder).unwrap();
    }
}
