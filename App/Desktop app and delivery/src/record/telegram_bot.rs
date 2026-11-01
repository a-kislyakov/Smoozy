//! The bot Telegram was set up with, and the conversation proved for it, in the
//! local record — `Telegram` P1 and `Telegram` S3a. Added 2026-09-13 by step 10
//! piece 4.
//!
//! **What is kept is what `Telegram` S3a names**: the bot's identity and its
//! username as the answered `getMe` carried them, beside the proved
//! conversation's identity, *"written to the local record with the proved
//! conversation identity"* when the proof succeeds. **`Telegram` P1**: the
//! conversation is *"recorded in the local record against the bot"*. The Settings
//! *Bot* reading and the walkthrough link are both read from this row.
//!
//! **One row.** One installation has one bot; a new bot is a new proof and
//! replaces the row (`Telegram` P4). **No token is here** — `Security and
//! licence` §9 keeps every secret out of the record, and the token is the
//! secret store's fourth kind (`Telegram` K1).
//!
//! **The diary declaration sits here too**, because the one act this module
//! writes from the desktop — *a bot set up*, `Telegram` §19 — is written at the
//! completion this row is part of, and this crate is where `Reporting a
//! problem`'s line type can be named. The act's fixed form is the Telegram
//! crate's.

use std::time::SystemTime;

use smoozy_reporting::lines::{Line, LineKind};
use smoozy_state::identities::Identity;
use smoozy_state::observed::Clock;
use smoozy_telegram::proof::{ARecordFailure, KeepsTheProvedBot, TheBot, TheConversation};
use smoozy_telegram::walkthrough::{TheBotIsSetUp, the_bot_tokens_item, a_fresh_bot_tokens_item};
use smoozy_security_and_licence::held::{HeldSecret, StoredKind, Subject};
use rusqlite::OptionalExtension;

/// **The one kind `Telegram` §19 declares for the desktop: acts.** Its two acts
/// are *a bot set up* and *a replacement token accepted*; **it writes no told
/// lines**, which §19 declares so the absence is a decision.
pub const DECLARED: [LineKind; 1] = [LineKind::Act];

/// **The local record's keeper of the proved bot.**
pub struct TheProvedBotRecord<'a> {
    connection: &'a rusqlite::Connection,
}

impl<'a> TheProvedBotRecord<'a> {
    pub fn over(connection: &'a rusqlite::Connection) -> TheProvedBotRecord<'a> {
        TheProvedBotRecord { connection }
    }
}

fn failed(error: rusqlite::Error) -> ARecordFailure {
    ARecordFailure(error.to_string())
}

impl KeepsTheProvedBot for TheProvedBotRecord<'_> {
    /// **A missing row is no bot; a read that did not answer is a failure**,
    /// never *no bot* — reading it as none would offer a walkthrough over a bot
    /// that is set up.
    fn the_proved_bot(&self) -> Result<Option<(TheBot, TheConversation)>, ARecordFailure> {
        let mut asking = self
            .connection
            .prepare(
                "SELECT bot_id, username, proved_conversation FROM telegram_bot
                 WHERE the_one_row = 1",
            )
            .map_err(failed)?;
        let mut rows = asking.query([]).map_err(failed)?;
        match rows.next().map_err(failed)? {
            None => Ok(None),
            Some(row) => Ok(Some((
                TheBot {
                    id: row.get(0).map_err(failed)?,
                    username: row.get(1).map_err(failed)?,
                },
                TheConversation(row.get(2).map_err(failed)?),
            ))),
        }
    }

    /// **`Telegram` P1: the proof recorded against the bot**, replacing any bot
    /// before it.
    ///
    /// **`Telegram` L4, as amended 2026-09-14: a different bot moves the token
    /// generation on in the same step.** Where a bot is on record and this one
    /// has a different identity (`Telegram` P4), the generation moves and the
    /// fact that a token is stored is cleared, inside one savepoint with the
    /// proof, so every server told the old generation is named as not reached
    /// (`Telegram` L3). A savepoint rather than a transaction, because
    /// `Telegram` L4's replacement records its refreshed proof inside a
    /// transaction of its own. **At a first setup, and for the same bot,
    /// nothing moves here.**
    fn the_proof_was_recorded(
        &self,
        bot: &TheBot,
        conversation: TheConversation,
    ) -> Result<(), ARecordFailure> {
        let c = self.connection;
        c.execute_batch("SAVEPOINT a_proof_is_recorded").map_err(failed)?;
        let attempt = (|| -> Result<(), ARecordFailure> {
            let before = self.the_proved_bot()?;
            c.execute(
                "INSERT INTO telegram_bot (the_one_row, bot_id, username, proved_conversation)
                 VALUES (1, ?1, ?2, ?3)
                 ON CONFLICT(the_one_row) DO UPDATE SET
                    bot_id = excluded.bot_id,
                    username = excluded.username,
                    proved_conversation = excluded.proved_conversation",
                rusqlite::params![bot.id, bot.username, conversation.0],
            )
            .map_err(failed)?;
            if before.is_some_and(|(was, _)| was.id != bot.id) {
                let next = super::told_record::the_current_generation(c)
                    .map_err(|why| ARecordFailure(format!("{why:?}")))?
                    .the_next()
                    .ok_or_else(|| ARecordFailure("the token generation cannot go on".into()))?;
                super::told_record::hold_the_generation(c, next)
                    .map_err(|why| ARecordFailure(format!("{why:?}")))?;
                the_stored_token_is_forgotten(c)?;
            }
            Ok(())
        })();
        match attempt {
            Ok(()) => c.execute_batch("RELEASE a_proof_is_recorded").map_err(failed),
            Err(why) => {
                let _ = c.execute_batch("ROLLBACK TO a_proof_is_recorded; RELEASE a_proof_is_recorded");
                Err(why)
            }
        }
    }

    fn the_item_for_saving(&self, bot: &TheBot) -> Result<HeldSecret, ARecordFailure> {
        let c = self.connection;
        let transaction = c.unchecked_transaction().map_err(failed)?;
        let before: Option<(i64, String)> = c.query_row(
            "SELECT bot_id, subject FROM telegram_token_item WHERE the_one_row = 1",
            [], |row| Ok((row.get(0)?, row.get(1)?)),
        ).optional().map_err(failed)?;
        if let Some((id, subject)) = &before {
            let owed: bool = c.query_row(
                "SELECT EXISTS(SELECT 1 FROM owed_erase WHERE kind = 'bot token' AND subject = ?1)",
                [subject], |row| row.get(0),
            ).map_err(failed)?;
            if *id == bot.id && !owed {
                transaction.commit().map_err(failed)?;
                return Ok(HeldSecret::new(StoredKind::BotToken, Subject::issued_by_the_local_record(subject)));
            }
            // Reserve the debt and the replacement together, before a store
            // call. A crash never loses track of an old item.
            c.execute("INSERT OR IGNORE INTO owed_erase (kind, subject) VALUES ('bot token', ?1)",
                [subject]).map_err(failed)?;
        }
        let item = a_fresh_bot_tokens_item()?;
        c.execute("INSERT INTO telegram_token_item (the_one_row, bot_id, subject) VALUES (1, ?1, ?2)
            ON CONFLICT(the_one_row) DO UPDATE SET bot_id = excluded.bot_id, subject = excluded.subject",
            rusqlite::params![bot.id, item.subject.as_str()]).map_err(failed)?;
        the_stored_token_is_forgotten(c)?;
        transaction.commit().map_err(failed)?;
        Ok(item)
    }

    /// **The token was stored, for this bot** — written only after a save
    /// succeeded (`Security and licence` §3.0 rule 6).
    fn the_token_was_stored_for(&self, bot: &TheBot) -> Result<(), ARecordFailure> {
        self.connection
            .execute(
                "INSERT INTO telegram_token_stored (the_one_row, bot_id) VALUES (1, ?1)
                 ON CONFLICT(the_one_row) DO UPDATE SET bot_id = excluded.bot_id",
                [bot.id],
            )
            .map_err(failed)?;
        Ok(())
    }
}

impl TheProvedBotRecord<'_> {
    /// **The bot whose token the record says is stored, or none.** A read that
    /// did not answer is a failure, and a caller drawing from it draws *not
    /// stored*, the side that claims nothing.
    pub fn the_bot_whose_token_is_stored(&self) -> Result<Option<i64>, ARecordFailure> {
        let mut asking = self
            .connection
            .prepare("SELECT bot_id FROM telegram_token_stored WHERE the_one_row = 1")
            .map_err(failed)?;
        let mut rows = asking.query([]).map_err(failed)?;
        match rows.next().map_err(failed)? {
            None => Ok(None),
            Some(row) => Ok(Some(row.get(0).map_err(failed)?)),
        }
    }
}

/// The saved or reserved item's subject. A record from before separate
/// items keeps using the legacy subject until the next save rotates it.
pub fn the_current_bot_tokens_item(c: &rusqlite::Connection) -> Result<HeldSecret, ARecordFailure> {
    let subject: Option<String> = c.query_row(
        "SELECT subject FROM telegram_token_item WHERE the_one_row = 1", [], |row| row.get(0),
    ).optional().map_err(failed)?;
    Ok(subject.map(|subject| HeldSecret::new(StoredKind::BotToken,
        Subject::issued_by_the_local_record(subject))).unwrap_or_else(the_bot_tokens_item))
}

/// Clearing an old item's debt must not clear a newer item's saved marker.
pub fn the_stored_item_is_forgotten(c: &rusqlite::Connection, item: &HeldSecret) -> Result<(), ARecordFailure> {
    if the_current_bot_tokens_item(c)? == *item {
        the_stored_token_is_forgotten(c)?;
    }
    Ok(())
}

/// **The fact that a token is stored, forgotten** — when the item's erase goes
/// ahead (`Security and licence` §5.1's reference) or a different bot's proof
/// is recorded.
pub fn the_stored_token_is_forgotten(connection: &rusqlite::Connection) -> Result<(), ARecordFailure> {
    connection
        .execute("DELETE FROM telegram_token_stored", [])
        .map_err(failed)?;
    Ok(())
}

/// **`Telegram` §19's first act, as a line.** The moment is this computer's;
/// the text is the declared fixed form, filled; the one identity carried as an
/// identity is the run's, the proved conversation travelling inside the fixed
/// form because it is Telegram's identifier and not one this application
/// issues.
pub fn a_bot_set_up(run: Identity, at: SystemTime, set_up: &TheBotIsSetUp) -> Line {
    Line {
        run: run.clone(),
        kind: LineKind::Act,
        at,
        clock: Clock::ThisComputer,
        text: set_up.the_diary_act(at),
        concerning: vec![run],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::{migrations, schema, LocalRecord};
    use smoozy_state::identities::IdentityKind;

    fn a_folder(name: &str) -> std::path::PathBuf {
        let mut folder = std::env::temp_dir();
        folder.push(format!("smoozy-telegram-bot-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder to put the record in");
        folder
    }

    /// **No bot is set up in a fresh record, and a proof reads back as it was
    /// written.**
    #[test]
    fn a_proof_is_recorded_and_read_back() {
        let folder = a_folder("proof");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let keeper = TheProvedBotRecord::over(record.connection());
        assert_eq!(keeper.the_proved_bot().unwrap(), None);

        let bot = TheBot { id: 1_234_567_890, username: "smoozy_a7k2_bot".into() };
        keeper.the_proof_was_recorded(&bot, TheConversation(-9_001)).unwrap();
        assert_eq!(keeper.the_proved_bot().unwrap(), Some((bot, TheConversation(-9_001))));

        // `Telegram` P4: a new bot is a new proof, and it replaces the row.
        let other = TheBot { id: 1, username: "smoozy_zz99_bot".into() };
        keeper.the_proof_was_recorded(&other, TheConversation(5)).unwrap();
        assert_eq!(keeper.the_proved_bot().unwrap(), Some((other, TheConversation(5))));
        let rows: i64 = record
            .connection()
            .query_row("SELECT count(*) FROM telegram_bot", [], |row| row.get(0))
            .unwrap();
        assert_eq!(rows, 1);

        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **`Telegram` L4 as amended, and fix-here item 2: a different bot moves the
    /// generation and forgets the stored token; the same bot and a first setup
    /// move nothing.**
    #[test]
    fn a_different_bot_moves_the_generation_and_forgets_the_stored_token() {
        use super::super::told_record::the_current_generation;
        use smoozy_telegram::told::TokenGeneration;
        let folder = a_folder("different");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        let keeper = TheProvedBotRecord::over(c);
        let a = TheBot { id: 7, username: "smoozy_a7k2_bot".into() };
        keeper.the_proof_was_recorded(&a, TheConversation(9001)).unwrap();
        assert_eq!(the_current_generation(c).unwrap(), TokenGeneration::THE_FIRST, "a first setup moves nothing");
        assert_eq!(keeper.the_bot_whose_token_is_stored().unwrap(), None, "nothing is stored until a save succeeds");
        keeper.the_token_was_stored_for(&a).unwrap();
        assert_eq!(keeper.the_bot_whose_token_is_stored().unwrap(), Some(7));

        let renamed = TheBot { id: 7, username: "smoozy_zz99_bot".into() };
        keeper.the_proof_was_recorded(&renamed, TheConversation(9001)).unwrap();
        assert_eq!(the_current_generation(c).unwrap(), TokenGeneration::THE_FIRST, "the same bot moves nothing");
        assert_eq!(keeper.the_bot_whose_token_is_stored().unwrap(), Some(7));

        let b = TheBot { id: 8, username: "smoozy_b8b8_bot".into() };
        keeper.the_proof_was_recorded(&b, TheConversation(9001)).unwrap();
        assert_eq!(the_current_generation(c).unwrap(), TokenGeneration(2), "a different bot moves the generation");
        assert_eq!(keeper.the_bot_whose_token_is_stored().unwrap(), None, "the old bot's stored token is forgotten");
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **The generation and the proof move together or not at all.**
    #[test]
    fn a_different_bot_whose_generation_cannot_move_records_no_proof() {
        let folder = a_folder("together");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        let keeper = TheProvedBotRecord::over(c);
        let a = TheBot { id: 7, username: "smoozy_a7k2_bot".into() };
        keeper.the_proof_was_recorded(&a, TheConversation(1)).unwrap();
        c.execute_batch("DROP TABLE telegram_token_generation").unwrap();
        let b = TheBot { id: 8, username: "smoozy_b8b8_bot".into() };
        assert!(keeper.the_proof_was_recorded(&b, TheConversation(2)).is_err());
        assert_eq!(keeper.the_proved_bot().unwrap(), Some((a, TheConversation(1))), "the proof went back with the generation");
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **A read that does not answer is a failure, never *no bot*.**
    #[test]
    fn a_read_that_does_not_answer_is_not_no_bot() {
        let folder = a_folder("unread");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        record.connection().execute_batch("DROP TABLE telegram_bot").unwrap();
        assert!(TheProvedBotRecord::over(record.connection()).the_proved_bot().is_err());
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **A record left at version 17, holding the settings the user set, opens
    /// at this binary's version with the bot's table added and the settings
    /// untouched.** L6 runs the missing migration; nothing earlier is edited.
    #[test]
    fn a_version_seventeen_record_gains_the_bot_and_keeps_its_settings() {
        let folder = a_folder("seventeen");
        let path = folder.join("local-record.sqlite3");
        {
            let connection = rusqlite::Connection::open(&path).unwrap();
            for migration in migrations::ALL.iter().filter(|one| one.produces <= 17) {
                migrations::run_one_for_tests(&connection, migration).unwrap();
            }
            assert_eq!(schema::version_in(&connection).unwrap(), 17);
            connection
                .execute_batch(
                    "INSERT INTO setting (the_one_row, upload_cap_bytes_per_second) VALUES (1, 1000);
                     INSERT INTO notification_switch (kind, attempted) VALUES ('a-job-fails', 0);",
                )
                .unwrap();
        }
        let record = LocalRecord::open_at(&path).unwrap();
        assert_eq!(record.schema_version().unwrap(), schema::BINARY_KNOWS);
        // **At least 18**, since a later version is added on top of this one and
        // never edits it (step 10 piece 5 added 19).
        assert!(schema::BINARY_KNOWS >= 18);
        let cap: i64 = record
            .connection()
            .query_row("SELECT upload_cap_bytes_per_second FROM setting", [], |row| row.get(0))
            .unwrap();
        assert_eq!(cap, 1000);
        assert_eq!(TheProvedBotRecord::over(record.connection()).the_proved_bot().unwrap(), None);
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **The table refuses a second row**, so two bots can never be on record.
    #[test]
    fn the_table_holds_one_bot_at_most() {
        let folder = a_folder("one");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        assert!(record
            .connection()
            .execute(
                "INSERT INTO telegram_bot (the_one_row, bot_id, username, proved_conversation)
                 VALUES (2, 1, 'smoozy_aaaa_bot', 1)",
                []
            )
            .is_err());
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **`Telegram` §19's line: an act, on this computer's clock, the fixed
    /// form filled, and the run identity carried.**
    #[test]
    fn the_act_line_is_the_declared_form_on_this_computers_clock() {
        let run = Identity::issue(IdentityKind::Run).expect("random bytes");
        let at = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_789_300_000);
        let set_up = TheBotIsSetUp {
            bot: TheBot { id: 7, username: "smoozy_a7k2_bot".into() },
            conversation: TheConversation(9001),
        };
        let line = a_bot_set_up(run.clone(), at, &set_up);
        assert_eq!(line.kind, LineKind::Act);
        assert_eq!(line.clock, Clock::ThisComputer);
        assert_eq!(line.text, "a bot set up; 1789300000; @smoozy_a7k2_bot; conversation 9001");
        assert_eq!(line.concerning, vec![run]);
    }
}
