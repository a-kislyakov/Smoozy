//! **`Telegram`'s told-record in the local record, the replacement token, and
//! the role's material** — `Telegram` §9 and §20's paragraph *To `Server
//! Setup`*, with §19's second diary act. Added 2026-09-13 by step 10 piece 5.
//!
//! **Markers in this file belong to other documents, and every one is written
//! with its document at its site**, because this crate's own plan defines
//! markers of the same names.
//!
//! **What is here.**
//!
//! - **The record**: one entry per server per told thing, written on the
//!   acknowledgement and made only from what Smoozy told (`Telegram` L1,
//!   `State, identity and recovery` D4). The rules are the Telegram crate's, at
//!   `smoozy_telegram::told`.
//! - **Which servers Smoozy has not reached**, derived when asked and never
//!   stored (`Telegram` L3).
//! - **A replacement token accepted** (`Telegram` L4): the old token's item
//!   erased in `Security and licence` §5.1's order for `Security and licence`
//!   E14's event, the
//!   generation moved on, the new token stored, and every server left on the old
//!   generation named. **Telegram switched off erases nothing** since 2026-10-31
//!   (`Telegram` K1, the Creator's fifteenth notes, note 1).
//! - **The material composed and handed to `Server Setup`'s one placement act**,
//!   `smoozy_server_setup::acts::place_the_telegram_material`, and the told
//!   entries moved on what the machine answered for each file.
//!
//! **What is not here, and is owed.** Nothing opens a connection: a telling
//! reaches a machine only over a connection `Servers` opens, and that connect
//! act is owed by step 10a piece 3.
//! **Nothing writes the diary act yet**: the screen that runs a replacement is
//! step 10 piece 8's, and it writes the line this file composes.

use std::time::SystemTime;

use rusqlite::OptionalExtension;
use smoozy_job_files::layout::{Area, MachineRoot, TheLayout, WhereThePackageAreaSits};
use smoozy_reporting::lines::{Line, LineKind};
use smoozy_security_and_licence::erasure::{
    self, EraseMarks, Erasure, ErasesStoreItem, ErasureEvent, RecordFailure as EraseFailure,
};
use smoozy_security_and_licence::held::{HeldSecret, StoredKind};
use smoozy_security_and_licence::secret_store::{GetFailure, SecretStore, StoreFailure};
use smoozy_server_setup::acts::{place_the_telegram_material, HowTheActWent};
use smoozy_server_setup::readings::TheMachinesFileSurface;
use smoozy_servers::store::ServerRow;
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::Clock;
use smoozy_telegram::material::{TheMaterial, TheRoleConfiguration, WhyTheMaterialWasNotComposed};
use smoozy_telegram::proof::KeepsTheProvedBot;
use smoozy_telegram::told::{
    the_machines_left_on_an_old_generation, the_machines_not_reached, AMachineAndWhatItWasTold,
    AReplacementAccepted, AStepOfATelling, AToldEntry, AToldThing, HowFarATellingGot,
    TokenGeneration, WhatSmoozyTells,
};
use smoozy_telegram::walkthrough::{TheCheckPassed, THE_STORE_CALLS_DEADLINE};
#[cfg(test)]
use smoozy_telegram::walkthrough::the_bot_tokens_item;

use super::server_rows::{self, ServerRowMarks};
use super::settings::the_stored_name_of;
use super::telegram_bot::{the_stored_token_is_forgotten, the_stored_item_is_forgotten, the_current_bot_tokens_item, TheProvedBotRecord};
use super::{moment, nanoseconds, RecordFailure};
use crate::notifications::kinds::Kind;

// ------------------------------------------------------------ the generation

/// **The generation the store holds.** Before any replacement it is the first:
/// the token a walkthrough kept is the first generation, and no row is written
/// for it (`Telegram` L4).
pub fn the_current_generation(connection: &rusqlite::Connection) -> Result<TokenGeneration, RecordFailure> {
    let held: Option<i64> = connection
        .query_row(
            "SELECT generation FROM telegram_token_generation WHERE the_one_row = 1",
            [],
            |row| row.get(0),
        )
        .optional()?;
    match held {
        None => Ok(TokenGeneration::THE_FIRST),
        Some(value) => u64::try_from(value).map(TokenGeneration).map_err(|_| RecordFailure::Engine {
            reported: format!("the token generation reads {value}"),
        }),
    }
}

pub(super) fn hold_the_generation(connection: &rusqlite::Connection, generation: TokenGeneration) -> Result<(), RecordFailure> {
    let value = i64::try_from(generation.0).map_err(|_| RecordFailure::Engine {
        reported: format!("generation {} is more than the record holds", generation.0),
    })?;
    connection.execute(
        "INSERT INTO telegram_token_generation (the_one_row, generation) VALUES (1, ?1)
         ON CONFLICT(the_one_row) DO UPDATE SET generation = excluded.generation",
        [value],
    )?;
    Ok(())
}

// ------------------------------------------------------------ the entries

/// **`Telegram` L1 and `Telegram` L2: the entry is written on the
/// acknowledgement**, from the value Smoozy told and nothing a machine said.
pub fn the_telling_was_acknowledged(
    connection: &rusqlite::Connection,
    server: &str,
    told: &WhatSmoozyTells,
    at: SystemTime,
    installation: &str,
) -> Result<(), RecordFailure> {
    let entry = AToldEntry::acknowledged(told, at, installation);
    let moment_written = nanoseconds(entry.acknowledged_at).ok_or_else(|| RecordFailure::AMomentTheRecordCannotHold {
        column: "told_entry.acknowledged_at",
        reported: format!("{:?}", entry.acknowledged_at),
    })?;
    connection.execute(
        "INSERT INTO told_entry (server, thing, value, acknowledged_at, installation)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(server, thing) DO UPDATE SET
            value = excluded.value,
            acknowledged_at = excluded.acknowledged_at,
            installation = excluded.installation",
        rusqlite::params![server, entry.thing.stored_as(), entry.value, moment_written, entry.installation],
    )?;
    Ok(())
}

/// Everything one server was told. **A thing this binary does not know is
/// passed over**, since nothing here could say what it means.
pub fn what_this_server_was_told(connection: &rusqlite::Connection, server: &str) -> Result<Vec<AToldEntry>, RecordFailure> {
    let mut asking = connection.prepare(
        "SELECT thing, value, acknowledged_at, installation FROM told_entry
         WHERE server = ?1 ORDER BY thing",
    )?;
    let rows = asking.query_map([server], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, String>(3)?,
        ))
    })?;
    let mut told = Vec::new();
    for row in rows {
        let (thing, value, at, installation) = row?;
        if let Some(thing) = AToldThing::stored_as_this(&thing) {
            told.push(AToldEntry {
                thing,
                value,
                acknowledged_at: moment(at),
                installation,
            });
        }
    }
    Ok(told)
}

/// **Each machine's label is the name the server list shows**
/// ([`the_name_a_machine_is_told`]), so the derivation compares what the
/// material tells.
fn the_fleet(connection: &rusqlite::Connection) -> Result<Vec<AMachineAndWhatItWasTold>, RecordFailure> {
    let rows = server_rows::all(connection)?;
    let mut fleet = Vec::new();
    for row in &rows {
        fleet.push(AMachineAndWhatItWasTold {
            told: what_this_server_was_told(connection, &row.local_server_identifier)?,
            server: row.local_server_identifier.clone(),
            label: Some(the_name_a_machine_is_told(row, &rows)),
            installation: row.last_confirmed_installation_identity.clone(),
        });
    }
    Ok(fleet)
}

/// **The name a machine is told: the one the server list shows** — the
/// Creator's thirteenth notes, note 11. `Specs/Telegram`: *"A machine is named
/// by the label the user gave its server, which Servers owns"*, and
/// `Specs/Servers`: a server left blank *"shows as `username@host`"*, with the
/// port where that is what tells two unnamed servers apart. Until then an
/// unnamed server was told it had no label, and the machine called itself *A
/// server* in every message. **A machine already told *no label* now reads as
/// not told its label**, so its next connection tells it again (`Telegram`
/// L2). The bot's own *A server*, for a machine not on its roster at all, is a
/// different case and stays.
fn the_name_a_machine_is_told(row: &ServerRow, all: &[ServerRow]) -> String {
    crate::surface::servers::what_a_server_is_called_among(row, all)
}

/// **The live Telegram pass, fault 1: a server's colour is told by its name.**
/// The desk keeps it as the drawing's badge code, `b1` to `b8`
/// (`web/the-drawing.css`, `crate::surface::the_rail::THE_EIGHT_BADGES`);
/// `Telegram` L1 holds it as its name, which is what the machine's colour square
/// is drawn from (`Telegram` R7a). Until 2026-10-31 the code went to the
/// machine as it was, and no message carried a square. **A code that is none of
/// the eight, or no colour, is told as none.**
pub fn the_colour_a_server_is_told(colour: Option<&str>) -> Option<&'static str> {
    const THE_EIGHT_NAMES: [&str; 8] = ["red", "gold", "green", "jade", "cyan", "blue", "violet", "magenta"];
    let badge = colour?;
    crate::surface::the_rail::THE_EIGHT_BADGES
        .iter()
        .position(|one| *one == badge)
        .map(|at| THE_EIGHT_NAMES[at])
}

// ------------------------------------------------------------ the derivations

/// **One server Smoozy has not reached, and what it has not been told.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AServerNotReached {
    pub server: String,
    pub label: Option<String>,
    pub not_told: Vec<AToldThing>,
}

/// **`Telegram` L3: which servers have not been told the current values**,
/// computed when asked. `now` holds the values that are the same for every
/// server; each server's own label is added from its row.
pub fn the_servers_not_reached(
    connection: &rusqlite::Connection,
    now: &[WhatSmoozyTells],
) -> Result<Vec<AServerNotReached>, RecordFailure> {
    let fleet = the_fleet(connection)?;
    let rows = server_rows::all(connection)?;
    Ok(the_machines_not_reached(&fleet, |machine| {
        let mut values = now.to_vec();
        values.push(WhatSmoozyTells::the_label_and_colour(machine.label.as_deref(), the_colour_a_server_is_told(rows.iter().find(|row| row.local_server_identifier == machine.server).and_then(|row| row.colour.as_deref()))));
        values
    })
    .into_iter()
    .map(|(machine, not_told)| AServerNotReached {
        server: machine.server.clone(),
        label: machine.label.clone(),
        not_told,
    })
    .collect())
}

// ------------------------------------------------------------ the erase marks

/// The register's word for the bot token. **The register's one reader is
/// [`ServerRowMarks::marks`]**, and a test holds this spelling to it.
const THE_REGISTERS_WORD: &str = "bot token";

/// The bot key's removal marks, each naming its own store item.
///
/// **The record's reference to the item is the fact that a token is stored**
/// (version 22, fix-here item 2 of the round *setting it up, and what the desktop keeps*):
/// removing the reference forgets it, so a token whose erase went ahead is
/// never drawn as held. The mark in the shared register is the whole of the
/// debt.
pub struct TheBotTokensMarks<'a>(pub &'a rusqlite::Connection);

impl EraseMarks for TheBotTokensMarks<'_> {
    fn mark_owing_an_erase(&mut self, item: &HeldSecret) -> Result<(), EraseFailure> {
        self.0
            .execute(
                "INSERT OR IGNORE INTO owed_erase (kind, subject) VALUES (?1, ?2)",
                rusqlite::params![THE_REGISTERS_WORD, item.subject.as_str()],
            )
            .map(|_| ())
            .map_err(|_| EraseFailure)
    }

    fn remove_reference(&mut self, item: &HeldSecret) -> Result<(), EraseFailure> {
        the_stored_item_is_forgotten(self.0, item).map_err(|_| EraseFailure)
    }

    fn clear_reference_and_mark(&mut self, item: &HeldSecret) -> Result<(), EraseFailure> {
        the_stored_item_is_forgotten(self.0, item).map_err(|_| EraseFailure)?;
        self.0
            .execute(
                "DELETE FROM owed_erase WHERE kind = ?1 AND subject = ?2",
                rusqlite::params![THE_REGISTERS_WORD, item.subject.as_str()],
            )
            .map(|_| ())
            .map_err(|_| EraseFailure)
    }

    fn marks(&self) -> Result<Vec<HeldSecret>, EraseFailure> {
        ServerRowMarks(self.0).marks().map(|items| items.into_iter().filter(|item| item.kind == StoredKind::BotToken).collect())
    }
}

/// Quiet retry at startup and before changing a bot. Only recorded bot
/// subjects are visited. Clearing an old debt leaves the current save intact.
pub fn retry_the_owed_bot_keys(connection: &rusqlite::Connection, store: &dyn SecretStore) {
    let mut marks = TheBotTokensMarks(connection);
    let Ok(owed) = marks.marks() else { return };
    for item in owed {
        if store.erase(&item, THE_STORE_CALLS_DEADLINE).is_ok() {
            let _ = marks.clear_reference_and_mark(&item);
        }
    }
}

/// **`Security and licence` E14, the bot token replaced: the desktop token
/// item, in §5.1's order** — the first step of a replacement
/// ([`a_replacement_token_accepted`]). What becomes of the copies on machines
/// is that document's §10.
///
/// **Switching Telegram off is no longer this event** (2026-10-31, `Telegram`
/// K1, the Creator's fifteenth notes, note 1: *"Needs fixing."*). The same
/// token stays on every machine it was placed on, so the erase ended nobody's
/// hold on the bot, and turning Telegram on again cost a revoked token, a paste
/// and every server reconnected. The no-store half that switching off needed
/// went with it: a replacement is never attempted without a store.
pub fn the_replaced_token_is_erased(connection: &rusqlite::Connection, store: &dyn SecretStore) -> Erasure {
    // **What the event erases is `Security and licence`'s answer and is read
    // from it**: the fourth kind's item, read from the local record.
    // Where that answer ever names no such item, nothing is owed.
    let item = match the_current_bot_tokens_item(connection) {
        Ok(item) => item,
        Err(_) => return Erasure::NothingDoneTheMarkWouldNotWrite,
    };
    match ErasureEvent::BotTokenReplaced.erases_store_item() {
        ErasesStoreItem::OfKind(kind) if kind == item.kind => {
            erasure::erase(&mut TheBotTokensMarks(connection), store, &item, THE_STORE_CALLS_DEADLINE)
        }
        _ => Erasure::Done,
    }
}

// ------------------------------------------------------------ Telegram L4: a replacement

/// **Why a replacement token was not accepted.**
#[derive(Debug)]
pub enum WhyTheReplacementWasNotAccepted {
    /// **No bot is on record**, so there is nothing to replace; the walkthrough
    /// is the route (`Telegram` S5).
    NoBotIsSetUp,
    /// **`Telegram` P4: the token names a different bot**, which is a new proof
    /// and not a replacement. The check is handed back for the walkthrough.
    ADifferentBot(Box<TheCheckPassed>),
    /// The local record did not answer.
    TheRecordDidNotAnswer(RecordFailure),
    /// **`Security and licence` E1's first write would not land**, so the old
    /// item was not touched and nothing was stored.
    TheOldTokensEraseCouldNotBeMarked,
    /// **The generation has moved and the new token is not stored.** Every
    /// server is already named as holding the old one, which is true: the old
    /// token was retired when the replacement was issued. Accepting again stores
    /// it.
    TheTokenWasNotStored(StoreFailure),
    /// **The new token's save did not answer in time**: `Security and
    /// licence` §3.0 rule 5 as amended, *not confirmed* and never done or *not
    /// stored*; the next read decides. fix-here item 3 of the round *setting it up, and
    /// what the desktop keeps*, which makes every timed-out save this.
    TheTokenWasNotConfirmed,
    /// The count cannot go on.
    TheGenerationCannotGoOn,
}

impl From<RecordFailure> for WhyTheReplacementWasNotAccepted {
    fn from(failure: RecordFailure) -> Self {
        WhyTheReplacementWasNotAccepted::TheRecordDidNotAnswer(failure)
    }
}

/// **`Telegram` L4: a replacement token accepted.**
///
/// The check has already run (`Telegram` S3), because only a passed check can
/// be handed in. Then, in this order:
///
/// 1. **`Telegram` P4**: the token must name the bot on record.
/// 2. The old key's erase is attempted. A refusal or unconfirmed erase is
///    recorded for a later retry; the replacement uses a separate item.
/// 3. **The generation moved on, and the bot's username refreshed**
///    (`Telegram` S3a), in one transaction. **This comes before the store**,
///    because every old holder was retired at the moment the replacement was
///    issued, so the record's mark is already true whatever the store does.
/// 4. The new key stored under its reserved subject, with its saved marker.
///
/// **What comes back names every server left on the old generation**, which
/// is what the user is shown and what §19's line records.
pub fn a_replacement_token_accepted(
    connection: &rusqlite::Connection,
    store: &dyn SecretStore,
    check: TheCheckPassed,
) -> Result<AReplacementAccepted, WhyTheReplacementWasNotAccepted> {
    retry_the_owed_bot_keys(connection, store);
    let keeper = TheProvedBotRecord::over(connection);
    let proved = keeper.the_proved_bot().map_err(|why| {
        WhyTheReplacementWasNotAccepted::TheRecordDidNotAnswer(RecordFailure::Engine { reported: why.0 })
    })?;
    let Some((bot, conversation)) = proved else {
        return Err(WhyTheReplacementWasNotAccepted::NoBotIsSetUp);
    };
    let (refreshed, token) = check
        .the_replacement_for(&bot)
        .map_err(|check| WhyTheReplacementWasNotAccepted::ADifferentBot(Box::new(check)))?;
    let next = the_current_generation(connection)?
        .the_next()
        .ok_or(WhyTheReplacementWasNotAccepted::TheGenerationCannotGoOn)?;

    if the_replaced_token_is_erased(connection, store) == Erasure::NothingDoneTheMarkWouldNotWrite {
        return Err(WhyTheReplacementWasNotAccepted::TheOldTokensEraseCouldNotBeMarked);
    }

    let marking = connection.unchecked_transaction().map_err(RecordFailure::from)?;
    keeper
        .the_proof_was_recorded(&refreshed, conversation)
        .map_err(|why| RecordFailure::Engine { reported: why.0 })?;
    hold_the_generation(connection, next)?;
    marking.commit().map_err(RecordFailure::from)?;

    // The old item may still be owed, including a late erase. Reserve a
    // separate item and keep the replacement regardless of that old result.
    let item = keeper.the_item_for_saving(&refreshed)
        .map_err(|why| RecordFailure::Engine { reported: why.0 })?;
    match store.put(&item, token, THE_STORE_CALLS_DEADLINE) {
        Ok(()) => {}
        // `Security and licence` §3.0 rule 5 as amended 2026-09-14.
        Err(StoreFailure::StoreUnavailable) => return Err(WhyTheReplacementWasNotAccepted::TheTokenWasNotConfirmed),
        Err(why) => return Err(WhyTheReplacementWasNotAccepted::TheTokenWasNotStored(why)),
    }
    // **`Security and licence` §3.0 rule 6: the fact that a token is stored,
    // written only after the save succeeded**, for the bot it names.
    keeper
        .the_token_was_stored_for(&refreshed)
        .map_err(|why| RecordFailure::Engine { reported: why.0 })?;

    let fleet = the_fleet(connection)?;
    Ok(AReplacementAccepted {
        generation: next,
        left_on_an_old_generation: the_machines_left_on_an_old_generation(&fleet, next),
    })
}

/// **`Telegram` §19's second act, as a line.** The run identity, and the local
/// server identifier of every server left behind carried as identities; the
/// labels ride inside the fixed form beside them.
pub fn a_replacement_token_accepted_line(run: Identity, at: SystemTime, accepted: &AReplacementAccepted) -> Line {
    let mut concerning = vec![run.clone()];
    concerning.extend(
        accepted
            .left_on_an_old_generation
            .iter()
            .filter_map(|(server, _)| Identity::read_back(IdentityKind::LocalServer, server)),
    );
    Line {
        run,
        kind: LineKind::Act,
        at,
        clock: Clock::ThisComputer,
        text: accepted.the_diary_act(at),
        concerning,
    }
}

// ------------------------------------------------------------ the material

/// **Why the material for one server could not be composed.**
#[derive(Debug)]
pub enum WhyTheMaterialCannotBeComposed {
    NoBotIsSetUp,
    TheRecordDidNotAnswer(RecordFailure),
    /// **No root is recorded for the server**, so there is no package area to
    /// name, and none is guessed.
    NoRootIsRecorded,
    /// The recorded root does not read as one.
    TheRecordedRootIsNotARoot,
    /// **No installation is confirmed**, so there is no machine identity to tell.
    NoInstallationIsConfirmed,
    /// **The record does not say a token is stored for the bot on record** —
    /// none was stored, its save failed, or it was stored for a bot that has
    /// since been replaced. **Nothing is composed**, so an old bot's token is
    /// never paired with a new bot's conversation (`Security and licence` §3.0
    /// rule 6; fix-here item 2 of the round *setting it up, and what the desktop keeps*).
    TheTokenIsNotStoredForThisBot,
    /// The token could not be read from the store.
    TheTokenIsNotInTheStore(GetFailure),
    Composed(WhyTheMaterialWasNotComposed),
}

impl WhyTheMaterialCannotBeComposed {
    /// **Why the material was not composed, in the one plain sentence the user
    /// reads** (Rule 9) — on the Set up card, in place of the step's own
    /// failure sentence, because composing is this computer's and no server
    /// said anything. **Never the reason's own name.** Added 2026-10-17, the
    /// Creator's eleventh notes, note 2: until then the card showed
    /// *The server could not install Telegram reporting.* with
    /// `TheTokenIsNotInTheStore(NotThere)` behind *What the server said*.
    ///
    /// **The three store answers are the runner's ruling of 2026-10-17**, under
    /// `Security and licence` §3.0 rule 4: *not there* asks for the token again
    /// (and [`the_material_for`] has forgotten the stored fact, so Settings
    /// draws *not stored* and *Replace…*); the other two are ordinary failures,
    /// the act offered again and nothing forgotten.
    pub fn in_words(&self) -> &'static str {
        use the_composing_sentences as said;
        use WhyTheMaterialCannotBeComposed as Why;
        match self {
            Why::NoBotIsSetUp => said::NO_BOT_IS_SET_UP,
            Why::TheRecordDidNotAnswer(_) => said::THE_SETTINGS_COULD_NOT_BE_READ,
            Why::NoRootIsRecorded
            | Why::TheRecordedRootIsNotARoot
            | Why::Composed(WhyTheMaterialWasNotComposed::ThePackageAreaIsNotAPathSmoozyRecorded) => {
                said::NO_FOLDER_IS_RECORDED
            }
            Why::NoInstallationIsConfirmed => said::SET_UP_THIS_SERVER_FIRST,
            Why::TheTokenIsNotStoredForThisBot => said::THE_TOKEN_IS_NOT_STORED,
            Why::TheTokenIsNotInTheStore(GetFailure::NotThere) => said::THE_TOKEN_IS_NO_LONGER_STORED,
            Why::TheTokenIsNotInTheStore(GetFailure::LockedOrRefused | GetFailure::StoreUnavailable) => {
                said::THE_TOKEN_COULD_NOT_BE_READ
            }
            Why::Composed(WhyTheMaterialWasNotComposed::TheTokenIsEmpty) => said::THE_TOKEN_IS_EMPTY,
        }
    }
}

/// **Every sentence Smoozy says where `Telegram` §20's material was not
/// composed on this computer — one closed list, kept here and nowhere else.**
///
/// Composing is this computer's, so where it fails no server said anything.
/// The sentence is written onto the machine as the failed step's words
/// (`Server Setup` S2a), and **the run panel reads it back as Smoozy's own
/// sentence, never as words the server said** — which it can only do by
/// knowing every sentence this list holds. Kept in one place so a new reason
/// cannot be added without the panel knowing it. Added 2026-10-18, the
/// Creator's twelfth notes, note 2: the panel read *The server could not
/// install Telegram reporting.* with Smoozy's sentence behind *What the server
/// said*.
pub mod the_composing_sentences {
    pub const NO_BOT_IS_SET_UP: &str = "No Telegram bot set up. Add one in Settings.";
    pub const THE_SETTINGS_COULD_NOT_BE_READ: &str = "Could not read your Telegram settings.";
    pub const NO_FOLDER_IS_RECORDED: &str = "Smoozy's folder on this server is unknown.";
    pub const SET_UP_THIS_SERVER_FIRST: &str = "Set up this server first.";
    pub const THE_TOKEN_IS_NOT_STORED: &str =
        "Telegram token not saved on this computer. Replace it in Settings.";
    pub const THE_TOKEN_IS_NO_LONGER_STORED: &str =
        "Telegram token no longer saved on this computer. Replace it in Settings.";
    pub const THE_TOKEN_COULD_NOT_BE_READ: &str = "Could not read the saved Telegram token.";
    pub const THE_TOKEN_IS_EMPTY: &str = "The saved Telegram token is empty. Replace it in Settings.";
    /// The three said by the run's own composing, before the record is asked
    /// (`crate::setting_up`'s `TheMaterialFromTheRecord`).
    #[cfg(target_os = "macos")]
    pub const NOWHERE_TO_STORE_THE_TOKEN: &str = "No keychain on this computer for the Telegram token.";
    #[cfg(not(target_os = "macos"))]
    pub const NOWHERE_TO_STORE_THE_TOKEN: &str =
        "No credential store on this computer for the Telegram token.";
    pub const NO_RECORD_OF_THIS_SERVER: &str = "Server not found in Smoozy's database.";

    /// **The whole list.** `THE_SETTINGS_COULD_NOT_BE_READ` is said both by
    /// the record and by the run's own composing, and is here once.
    pub const EVERY_ONE: [&str; 10] = [
        NO_BOT_IS_SET_UP,
        THE_SETTINGS_COULD_NOT_BE_READ,
        NO_FOLDER_IS_RECORDED,
        SET_UP_THIS_SERVER_FIRST,
        THE_TOKEN_IS_NOT_STORED,
        THE_TOKEN_IS_NO_LONGER_STORED,
        THE_TOKEN_COULD_NOT_BE_READ,
        THE_TOKEN_IS_EMPTY,
        NOWHERE_TO_STORE_THE_TOKEN,
        NO_RECORD_OF_THIS_SERVER,
    ];

    /// **The same ten as they were worded before 2026-10-28, index for index
    /// with [`EVERY_ONE`].** A failed step's words already written onto a
    /// server keep the earlier sentence, so it is still matched, and read back
    /// as the sentence that stands in its place now. Never shortened or
    /// reordered: a record on a server may hold any of them.
    pub const EVERY_EARLIER_ONE: [&str; 10] = [
        "No Telegram bot is set up. Set one up in Settings.",
        "Smoozy could not read your Telegram settings.",
        "Smoozy has not recorded where its folder is on this server.",
        "Set up this server first.",
        "The Telegram token is not stored on this computer. Replace it in Settings.",
        "The Telegram token is no longer stored on this computer. Replace it in Settings.",
        "Smoozy could not read the stored Telegram token.",
        "The stored Telegram token is empty. Replace it in Settings.",
        "This computer has nowhere to store the Telegram token.",
        "Smoozy has no record of this server.",
    ];

    /// **Two sentences the second words sweep changed on 2026-11-01, as they
    /// read from 2026-10-28, and the token store's sentence as the other
    /// systems word it**, each with the place it stands at in [`EVERY_ONE`].
    /// A server may hold a sentence another computer wrote, so a Mac matches
    /// the *credential store* wording and the other systems the *keychain*
    /// one. Never shortened: a record on a server may hold any of them.
    pub const ALSO_MATCHED: [(&str, usize); 3] = [
        ("Smoozy's folder on this server is not recorded.", 2),
        ("No password storage on this computer for the Telegram token.", 8),
        (NOWHERE_TO_STORE_THE_TOKEN_ON_THE_OTHER_SYSTEMS, 8),
    ];
    #[cfg(target_os = "macos")]
    const NOWHERE_TO_STORE_THE_TOKEN_ON_THE_OTHER_SYSTEMS: &str =
        "No credential store on this computer for the Telegram token.";
    #[cfg(not(target_os = "macos"))]
    const NOWHERE_TO_STORE_THE_TOKEN_ON_THE_OTHER_SYSTEMS: &str =
        "No keychain on this computer for the Telegram token.";

    /// **The list's own sentence where `words` is one of them, whole and
    /// exactly**, and nothing otherwise. Words a machine wrote are matched and
    /// never read for a cause. An earlier wording answers with today's
    /// sentence for the same reason.
    pub fn the_one_said(words: &str) -> Option<&'static str> {
        EVERY_ONE.into_iter().find(|one| *one == words).or_else(|| {
            EVERY_EARLIER_ONE
                .iter()
                .position(|earlier| *earlier == words)
                .or_else(|| ALSO_MATCHED.iter().find(|(also, _)| *also == words).map(|(_, at)| *at))
                .map(|at| EVERY_ONE[at])
        })
    }
}

impl From<RecordFailure> for WhyTheMaterialCannotBeComposed {
    fn from(failure: RecordFailure) -> Self {
        WhyTheMaterialCannotBeComposed::TheRecordDidNotAnswer(failure)
    }
}

/// **`Telegram` §20: the role's material for one server**, from the bot on
/// record, the generation, the token in the store, the server's recorded root
/// and installation, its label, and the idle message's switch (`Telegram` V2).
pub fn the_material_for(
    connection: &rusqlite::Connection,
    store: &dyn SecretStore,
    row: &ServerRow,
    the_idle_switch_is_on: bool,
) -> Result<TheMaterial, WhyTheMaterialCannotBeComposed> {
    let proved = TheProvedBotRecord::over(connection).the_proved_bot().map_err(|why| {
        WhyTheMaterialCannotBeComposed::TheRecordDidNotAnswer(RecordFailure::Engine { reported: why.0 })
    })?;
    let Some((bot, conversation)) = proved else {
        return Err(WhyTheMaterialCannotBeComposed::NoBotIsSetUp);
    };
    let stored_for = TheProvedBotRecord::over(connection).the_bot_whose_token_is_stored().map_err(|why| {
        WhyTheMaterialCannotBeComposed::TheRecordDidNotAnswer(RecordFailure::Engine { reported: why.0 })
    })?;
    if stored_for != Some(bot.id) {
        return Err(WhyTheMaterialCannotBeComposed::TheTokenIsNotStoredForThisBot);
    }
    let root = row
        .recorded_machine_root
        .as_ref()
        .ok_or(WhyTheMaterialCannotBeComposed::NoRootIsRecorded)?;
    let installation = row
        .last_confirmed_installation_identity
        .clone()
        .ok_or(WhyTheMaterialCannotBeComposed::NoInstallationIsConfirmed)?;
    let not_a_root = |_| WhyTheMaterialCannotBeComposed::TheRecordedRootIsNotARoot;
    let package_area = match &root.the_package_areas_own_location {
        Some(apart) => WhereThePackageAreaSits::ApartAt(MachineRoot::chosen(apart).map_err(not_a_root)?),
        None => WhereThePackageAreaSits::UnderTheRoot,
    };
    let layout = TheLayout::of(MachineRoot::chosen(&root.path).map_err(not_a_root)?, package_area);
    let generation = the_current_generation(connection)?;
    let token = match store.get(&the_current_bot_tokens_item(connection)
        .map_err(|why| RecordFailure::Engine { reported: why.0 })?, THE_STORE_CALLS_DEADLINE) {
        Ok(token) => token,
        Err(failure) => {
            // **`Security and licence` §3.0 rule 4: only *not there* is asked
            // for again, as though nothing was remembered** — so the record's
            // *token stored* fact goes (rule 6 answers *remembered* from it),
            // and Settings draws *not stored* and *Replace…*. **The other two
            // forget nothing.** Added 2026-10-17, the Creator's eleventh notes,
            // note 2. **A forgetting the record will not take is not a second
            // failure**: the sentence said is still true, and the next
            // composing asks the store again.
            if failure.asks_the_user_again() {
                let _ = the_stored_token_is_forgotten(connection);
            }
            return Err(WhyTheMaterialCannotBeComposed::TheTokenIsNotInTheStore(failure));
        }
    };
    let every_row = server_rows::all(connection)?;
    let configuration = TheRoleConfiguration {
        conversation,
        machine: installation,
        // The live Telegram pass, fault 1: the name, never the badge code.
        colour: the_colour_a_server_is_told(row.colour.as_deref()).map(str::to_string),
        label: Some(the_name_a_machine_is_told(row, &every_row)),
        reporting: vec![(
            the_stored_name_of(Kind::AMachineHasBeenIdle).to_string(),
            the_idle_switch_is_on,
        )],
    };
    TheMaterial::composed(&layout.area(Area::Package), token, generation, configuration)
        .map_err(WhyTheMaterialCannotBeComposed::Composed)
}

/// **`Server Setup` G1 and `Server Setup` G4: the material placed by that
/// module's one placement act**, a first placement and a replacement alike.
pub async fn place_the_material(
    files: &mut impl TheMachinesFileSurface,
    material: &TheMaterial,
) -> Vec<(String, HowTheActWent)> {
    place_the_telegram_material(files, material.the_files()).await
}

/// **What each file's placement tells, and how far each telling got**
/// (`Telegram` §19): the token file tells the generation; the role
/// configuration tells the conversation, the label and the switches.
///
/// **The machine's answer for a file is the acknowledgement of what that file
/// carries**, and the told entries move on it and on nothing else. A file the
/// machine did not take moves nothing, and its later steps are reported as not
/// attempted.
pub fn the_placement_was_answered(
    connection: &rusqlite::Connection,
    server: &str,
    material: &TheMaterial,
    placed: &[(String, HowTheActWent)],
    at: SystemTime,
) -> Result<Vec<(AToldThing, HowFarATellingGot)>, RecordFailure> {
    let installation = material.the_configuration().machine.clone();
    let mut tellings = vec![(
        material.the_token_files_path().to_string(),
        vec![WhatSmoozyTells::the_token_generation(material.the_generation())],
    )];
    tellings.push((
        material.the_role_configurations_path().to_string(),
        material.the_configuration().what_it_tells().to_vec(),
    ));

    let mut reached = Vec::new();
    for (path, told) in tellings {
        let went = placed.iter().find(|(placed_at, _)| *placed_at == path).map(|(_, went)| went);
        for value in told {
            let how_far = match went {
                Some(went) if went.it_was_carried_out() => {
                    the_telling_was_acknowledged(connection, server, &value, at, &installation)?;
                    HowFarATellingGot::all_done()
                }
                Some(went) => HowFarATellingGot::stopped_at(AStepOfATelling::PlacedOnTheMachine, format!("{went:?}")),
                None => HowFarATellingGot::stopped_at(AStepOfATelling::PlacedOnTheMachine, "the file was not placed"),
            };
            reached.push((value.thing(), how_far));
        }
    }
    Ok(reached)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::LocalRecord;
    use smoozy_security_and_licence::held::StoredKind;
    use smoozy_security_and_licence::plaintext::Plaintext;
    use smoozy_security_and_licence::secret_store::ProbeOutcome;
    use smoozy_servers::identity::{Host, KeyReference, Port, Username};
    use smoozy_servers::store::RecordedMachineRoot;
    use smoozy_telegram::proof::{TheBot, TheConversation};
    use std::cell::RefCell;
    use std::time::Duration;

    fn a_folder(name: &str) -> std::path::PathBuf {
        let mut folder = std::env::temp_dir();
        folder.push(format!("smoozy-told-record-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder to put the record in");
        folder
    }

    fn a_server(connection: &rusqlite::Connection, label: &str, installation: &str) -> ServerRow {
        let mut row = ServerRow::saved(
            Identity::issue(IdentityKind::LocalServer).expect("randomness").as_text(),
            Host::what_the_user_typed("5.180.183.77").expect("a host"),
            Port::what_the_user_typed(22).expect("a port"),
            Username::what_the_user_typed("root").expect("a username"),
            KeyReference { local_key_identifier: "key-1".into(), fingerprint: "SHA256:aaa".into() },
        );
        row.label = Some(label.into());
        row.last_confirmed_installation_identity = Some(installation.into());
        row.recorded_machine_root = Some(RecordedMachineRoot {
            path: "/mnt/one/smoozy".into(),
            the_package_areas_own_location: None,
            read_under: installation.into(),
        });
        server_rows::add(connection, &row).unwrap();
        server_rows::write(connection, &row).unwrap();
        row
    }

    fn at(seconds: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(seconds)
    }

    /// A store in memory that can refuse its puts, and can leave its erases
    /// unanswered.
    #[derive(Default)]
    struct AStore {
        held: RefCell<std::collections::HashMap<HeldSecret, Vec<u8>>>,
        refuse_puts: bool,
        erases_do_not_answer: std::cell::Cell<bool>,
    }

    impl SecretStore for AStore {
        fn put(&self, item: &HeldSecret, value: Plaintext, _: Duration) -> Result<(), StoreFailure> {
            if self.refuse_puts {
                return Err(StoreFailure::LockedOrRefused);
            }
            self.held.borrow_mut().insert(item.clone(), value.expose(|bytes| bytes.to_vec()));
            Ok(())
        }
        fn get(&self, item: &HeldSecret, _: Duration) -> Result<Plaintext, GetFailure> {
            self.held.borrow().get(item).cloned().map(Plaintext::new).ok_or(GetFailure::NotThere)
        }
        fn erase(&self, item: &HeldSecret, _: Duration) -> Result<(), StoreFailure> {
            if self.erases_do_not_answer.get() {
                return Err(StoreFailure::from_the_deadline_passing());
            }
            self.held.borrow_mut().remove(item);
            Ok(())
        }
        fn probe(&self, _: Duration) -> ProbeOutcome {
            ProbeOutcome::RoundTripWorked
        }
        fn enumerate(&self, _: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(Vec::new())
        }
    }

    fn every_value(generation: TokenGeneration) -> Vec<WhatSmoozyTells> {
        vec![
            WhatSmoozyTells::the_roles_desired_state(true),
            WhatSmoozyTells::the_token_generation(generation),
            WhatSmoozyTells::the_proved_conversation(TheConversation(9001)),
            WhatSmoozyTells::the_reporting_settings([("a-server-has-had-nothing-to-do", true)]),
        ]
    }

    /// **`Telegram` L1 and `Telegram` L3 over two seeded servers**: told
    /// everything, neither is unreached; a changed current value names both,
    /// and each server's own label counts.
    #[test]
    fn two_servers_told_everything_are_reached_and_a_change_names_both() {
        let folder = a_folder("two");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        let one = a_server(c, "one", "i1");
        let two = a_server(c, "two", "i2");
        for row in [&one, &two] {
            let installation = row.last_confirmed_installation_identity.as_deref().unwrap();
            let mut values = every_value(TokenGeneration::THE_FIRST);
            values.push(WhatSmoozyTells::the_label(row.label.as_deref()));
            for value in &values {
                the_telling_was_acknowledged(c, &row.local_server_identifier, value, at(10), installation).unwrap();
            }
        }
        // Older rows stay on disk until wave 2's migration, but are not read.
        c.execute(
            "INSERT INTO told_entry (server, thing, value, acknowledged_at, installation) VALUES (?1, 'the lock policy', '', 0, 'i1')",
            [&one.local_server_identifier],
        ).unwrap();
        assert_eq!(the_current_generation(c).unwrap(), TokenGeneration::THE_FIRST);
        assert!(the_servers_not_reached(c, &every_value(TokenGeneration::THE_FIRST)).unwrap().is_empty());
        assert_eq!(what_this_server_was_told(c, &one.local_server_identifier).unwrap().len(), 5);

        let not_reached = the_servers_not_reached(c, &every_value(TokenGeneration(2))).unwrap();
        assert_eq!(not_reached.len(), 2);
        assert!(not_reached.iter().all(|server| server.not_told == vec![AToldThing::TheTokenGeneration]));

        // A renamed server has not been told its new label (`Telegram` R8).
        let mut renamed = one.clone();
        renamed.label = Some("renamed".into());
        server_rows::write(c, &renamed).unwrap();
        let not_reached = the_servers_not_reached(c, &every_value(TokenGeneration::THE_FIRST)).unwrap();
        assert_eq!(not_reached.len(), 1);
        assert_eq!(not_reached[0].not_told, vec![AToldThing::TheLabel]);
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **`Telegram` L4 over two servers told generation 1**: the replacement
    /// erases the old item, moves the generation, stores the token, and names
    /// both; the line names both in one line.
    #[test]
    fn a_replacement_marks_both_seeded_servers() {
        let folder = a_folder("replacement");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        for (label, installation) in [("one", "i1"), ("two", "i2")] {
            let row = a_server(c, label, installation);
            the_telling_was_acknowledged(
                c,
                &row.local_server_identifier,
                &WhatSmoozyTells::the_token_generation(TokenGeneration::THE_FIRST),
                at(1),
                installation,
            )
            .unwrap();
        }
        let store = AStore::default();
        store.put(&the_bot_tokens_item(), Plaintext::new(b"7:old".to_vec()), THE_STORE_CALLS_DEADLINE).unwrap();
        let bot = TheBot { id: 7, username: "smoozy_a7k2_bot".into() };
        TheProvedBotRecord::over(c).the_proof_was_recorded(&bot, TheConversation(9001)).unwrap();

        // The check has to be a real passed check, which only the walkthrough
        // makes; the stand-in service answers `getMe` for bot 7 under a new name.
        let check = a_passed_check_for(7, "smoozy_zz99_bot", "7:new");
        let accepted = a_replacement_token_accepted(c, &store, check).unwrap();
        assert_eq!(accepted.generation, TokenGeneration(2));
        assert_eq!(the_current_generation(c).unwrap(), TokenGeneration(2));
        assert_eq!(accepted.left_on_an_old_generation.len(), 2);
        assert_eq!(store.held.borrow().get(&the_current_bot_tokens_item(c).unwrap()).map(Vec::as_slice), Some(&b"7:new"[..]));
        assert_eq!(TheProvedBotRecord::over(c).the_bot_whose_token_is_stored().unwrap(), Some(7), "fix-here item 2: the stored fact is written");
        assert_eq!(
            TheProvedBotRecord::over(c).the_proved_bot().unwrap().unwrap().0.username,
            "smoozy_zz99_bot",
            "`Telegram` S3a: the username is refreshed from the answer"
        );
        let run = Identity::issue(IdentityKind::Run).unwrap();
        let line = a_replacement_token_accepted_line(run, at(1_789_300_000), &accepted);
        assert_eq!(line.kind, LineKind::Act);
        assert!(line.text.starts_with("a replacement token accepted; 1789300000; generation 2; not reached: "));
        assert!(!line.text.contains("7:new"));
        assert_eq!(line.concerning.len(), 3, "the run and both servers");
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **A store that refuses leaves the mark true and the token unstored**, and
    /// accepting again finishes it.
    #[test]
    fn a_refused_store_leaves_the_generation_moved_and_no_token() {
        let folder = a_folder("refused");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        TheProvedBotRecord::over(c)
            .the_proof_was_recorded(&TheBot { id: 7, username: "smoozy_a7k2_bot".into() }, TheConversation(1))
            .unwrap();
        let store = AStore { refuse_puts: true, ..AStore::default() };
        let why = a_replacement_token_accepted(c, &store, a_passed_check_for(7, "smoozy_a7k2_bot", "7:new")).unwrap_err();
        assert!(matches!(why, WhyTheReplacementWasNotAccepted::TheTokenWasNotStored(_)));
        assert_eq!(the_current_generation(c).unwrap(), TokenGeneration(2));
        assert!(store.held.borrow().is_empty());
        assert_eq!(TheProvedBotRecord::over(c).the_bot_whose_token_is_stored().unwrap(), None, "fix-here item 2: no save, no stored fact");
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// An unconfirmed erase leaves a debt, while a replacement is saved in
    /// its own item. A later cleanup cannot remove the replacement.
    #[test]
    fn an_owed_erase_of_the_old_key_does_not_block_the_replacement() {
        let folder = a_folder("owed");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        TheProvedBotRecord::over(c)
            .the_proof_was_recorded(&TheBot { id: 7, username: "smoozy_a7k2_bot".into() }, TheConversation(1))
            .unwrap();
        let store = AStore::default();
        let old = the_bot_tokens_item();
        store.put(&old, Plaintext::new(b"7:old".to_vec()), THE_STORE_CALLS_DEADLINE).unwrap();
        store.erases_do_not_answer.set(true);
        let accepted = a_replacement_token_accepted(c, &store, a_passed_check_for(7, "smoozy_a7k2_bot", "7:new")).unwrap();
        assert_eq!(accepted.generation, TokenGeneration(2));
        let new_item = the_current_bot_tokens_item(c).unwrap();
        assert_ne!(new_item, old);
        assert_eq!(store.held.borrow().get(&old).map(Vec::as_slice), Some(&b"7:old"[..]));
        assert_eq!(store.held.borrow().get(&new_item).map(Vec::as_slice), Some(&b"7:new"[..]));
        assert_eq!(TheBotTokensMarks(c).marks().unwrap(), vec![old]);
        store.erases_do_not_answer.set(false);
        retry_the_owed_bot_keys(c, &store);
        assert!(TheBotTokensMarks(c).marks().unwrap().is_empty());
        assert_eq!(store.held.borrow().get(&new_item).map(Vec::as_slice), Some(&b"7:new"[..]));
        assert_eq!(TheProvedBotRecord::over(c).the_bot_whose_token_is_stored().unwrap(), Some(7));
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **`Telegram` P4: a token for another bot is not a replacement**, and
    /// nothing moves.
    #[test]
    fn a_token_for_another_bot_is_handed_back_and_nothing_moves() {
        let folder = a_folder("other");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        let store = AStore::default();
        assert!(matches!(
            a_replacement_token_accepted(c, &store, a_passed_check_for(8, "smoozy_b8b8_bot", "8:x")).unwrap_err(),
            WhyTheReplacementWasNotAccepted::NoBotIsSetUp
        ));
        TheProvedBotRecord::over(c)
            .the_proof_was_recorded(&TheBot { id: 7, username: "smoozy_a7k2_bot".into() }, TheConversation(1))
            .unwrap();
        store.put(&the_bot_tokens_item(), Plaintext::new(b"7:old".to_vec()), THE_STORE_CALLS_DEADLINE).unwrap();
        assert!(matches!(
            a_replacement_token_accepted(c, &store, a_passed_check_for(8, "smoozy_b8b8_bot", "8:x")).unwrap_err(),
            WhyTheReplacementWasNotAccepted::ADifferentBot(_)
        ));
        assert_eq!(the_current_generation(c).unwrap(), TokenGeneration::THE_FIRST);
        assert_eq!(store.held.borrow().get(&the_bot_tokens_item()).map(Vec::as_slice), Some(&b"7:old"[..]), "the old token is untouched");
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **`Security and licence` E14 for the token replaced, and the mark's
    /// spelling held to the register's one reader.**
    #[test]
    fn a_replaced_token_is_erased_and_the_registers_reader_reads_the_mark() {
        let folder = a_folder("off");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        TheBotTokensMarks(c).mark_owing_an_erase(&the_bot_tokens_item()).unwrap();
        let marks = ServerRowMarks(c).marks().unwrap();
        assert_eq!(marks.len(), 1);
        assert_eq!(marks[0].kind, StoredKind::BotToken);
        TheBotTokensMarks(c).clear_reference_and_mark(&the_bot_tokens_item()).unwrap();

        let store = AStore::default();
        store.put(&the_bot_tokens_item(), Plaintext::new(b"7:t".to_vec()), THE_STORE_CALLS_DEADLINE).unwrap();
        assert_eq!(the_replaced_token_is_erased(c, &store), Erasure::Done);
        assert!(store.held.borrow().is_empty());
        assert!(ServerRowMarks(c).marks().unwrap().is_empty(), "a completed erase leaves no mark");
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **A removed server takes its told entries with it** (`Servers` D7).
    #[test]
    fn a_removed_server_leaves_no_told_entry() {
        let folder = a_folder("removed");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        let row = a_server(c, "one", "i1");
        let other = a_server(c, "two", "i2");
        for server in [&row, &other] {
            the_telling_was_acknowledged(
                c,
                &server.local_server_identifier,
                &WhatSmoozyTells::the_roles_desired_state(true),
                at(1),
                "i",
            )
            .unwrap();
        }
        server_rows::remove_walking_the_enumeration(c, &row.local_server_identifier).unwrap();
        assert!(what_this_server_was_told(c, &row.local_server_identifier).unwrap().is_empty());
        assert_eq!(what_this_server_was_told(c, &other.local_server_identifier).unwrap().len(), 1);
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **The other two of `Security and licence` §3.0 rule 4's classes are
    /// ordinary failures: the stored fact is kept** — the Creator's eleventh
    /// notes, note 2, and the runner's ruling of 2026-10-17.
    #[test]
    fn a_store_that_will_not_give_the_token_up_forgets_nothing() {
        struct AStoreThatAnswers(GetFailure);
        impl SecretStore for AStoreThatAnswers {
            fn put(&self, _: &HeldSecret, _: Plaintext, _: Duration) -> Result<(), StoreFailure> {
                Ok(())
            }
            fn get(&self, _: &HeldSecret, _: Duration) -> Result<Plaintext, GetFailure> {
                Err(self.0)
            }
            fn erase(&self, _: &HeldSecret, _: Duration) -> Result<(), StoreFailure> {
                Ok(())
            }
            fn probe(&self, _: Duration) -> ProbeOutcome {
                ProbeOutcome::RoundTripWorked
            }
            fn enumerate(&self, _: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
                Ok(Vec::new())
            }
        }
        for failure in [GetFailure::LockedOrRefused, GetFailure::StoreUnavailable, GetFailure::NotThere] {
            let folder = a_folder(&format!("ordinary-{failure:?}"));
            let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
            let c = record.connection();
            let row = a_server(c, "one", "i1");
            let bot = TheBot { id: 7, username: "smoozy_a7k2_bot".into() };
            TheProvedBotRecord::over(c).the_proof_was_recorded(&bot, TheConversation(9001)).unwrap();
            TheProvedBotRecord::over(c).the_token_was_stored_for(&bot).unwrap();
            let why = the_material_for(c, &AStoreThatAnswers(failure), &row, true).unwrap_err();
            assert!(matches!(why, WhyTheMaterialCannotBeComposed::TheTokenIsNotInTheStore(f) if f == failure));
            let kept = TheProvedBotRecord::over(c).the_bot_whose_token_is_stored().unwrap();
            if failure == GetFailure::NotThere {
                assert_eq!(kept, None, "not there kept the stored fact");
            } else {
                assert_eq!(kept, Some(7), "{failure:?} forgot the stored fact");
            }
            drop(record);
            let _ = std::fs::remove_dir_all(folder);
        }
    }

    /// **Every reason the material is not composed is said in plain words**
    /// (Rule 9), and never as the reason's own name — the Creator's eleventh
    /// notes, note 2, where the card showed `TheTokenIsNotInTheStore(NotThere)`.
    #[test]
    fn every_reason_the_material_is_not_composed_is_said_in_words() {
        use WhyTheMaterialCannotBeComposed as Why;
        let every = [
            Why::NoBotIsSetUp,
            Why::TheRecordDidNotAnswer(RecordFailure::Engine { reported: "SQLITE_BUSY".into() }),
            Why::NoRootIsRecorded,
            Why::TheRecordedRootIsNotARoot,
            Why::NoInstallationIsConfirmed,
            Why::TheTokenIsNotStoredForThisBot,
            Why::TheTokenIsNotInTheStore(GetFailure::NotThere),
            Why::TheTokenIsNotInTheStore(GetFailure::LockedOrRefused),
            Why::TheTokenIsNotInTheStore(GetFailure::StoreUnavailable),
            Why::Composed(WhyTheMaterialWasNotComposed::ThePackageAreaIsNotAPathSmoozyRecorded),
            Why::Composed(WhyTheMaterialWasNotComposed::TheTokenIsEmpty),
        ];
        for why in &every {
            let said = why.in_words();
            let named = format!("{why:?}");
            assert!(said.ends_with('.') && said.starts_with(char::is_uppercase), "{said}");
            assert!(!said.contains('_') && !said.contains('(') && !said.contains("SQLITE"), "{said}");
            let the_name = named.split('(').next().unwrap_or(&named);
            assert!(!said.contains(the_name), "{named} reached the user as its own name");
        }
        // **The runner's ruling of 2026-10-17, word for word.**
        assert_eq!(
            Why::TheTokenIsNotInTheStore(GetFailure::NotThere).in_words(),
            "Telegram token no longer saved on this computer. Replace it in Settings."
        );
        for failure in [GetFailure::LockedOrRefused, GetFailure::StoreUnavailable] {
            assert_eq!(
                Why::TheTokenIsNotInTheStore(failure).in_words(),
                "Could not read the saved Telegram token."
            );
        }
        // **Every reason's sentence is on the one closed list** the run panel
        // reads a failed Telegram step by (the Creator's twelfth notes, note 2).
        for why in &every {
            assert_eq!(
                the_composing_sentences::the_one_said(why.in_words()),
                Some(why.in_words()),
                "{why:?} is said in words the closed list does not hold"
            );
        }
    }

    /// **The closed list holds every sentence once, each one plain, and
    /// matches only a sentence whole** — the Creator's twelfth notes, note 2.
    #[test]
    fn the_composing_sentences_are_one_closed_list() {
        use the_composing_sentences::{the_one_said, EVERY_ONE};
        for (at, one) in EVERY_ONE.iter().enumerate() {
            assert!(one.ends_with('.') && one.starts_with(char::is_uppercase), "{one}");
            assert!(!EVERY_ONE[at + 1..].contains(one), "{one} is on the list twice");
            assert_eq!(the_one_said(one), Some(*one));
            assert_eq!(the_one_said(&format!("{one} ")), None, "a sentence matched with more words");
            assert_eq!(the_one_said(&one[1..]), None, "a sentence matched in part");
        }
        assert_eq!(the_one_said(""), None);
        assert_eq!(the_one_said("E: Failed to fetch … 404  Not Found"), None);
    }

    /// **A sentence written onto a server before the 2026-10-28 words sweep
    /// still reads back as Smoozy's own**, as the sentence that replaced it.
    #[test]
    fn an_earlier_wording_is_still_matched_index_for_index() {
        use the_composing_sentences::{the_one_said, EVERY_EARLIER_ONE, EVERY_ONE};
        for (earlier, now) in EVERY_EARLIER_ONE.iter().zip(EVERY_ONE) {
            assert_eq!(the_one_said(earlier), Some(now), "{earlier}");
            assert_eq!(the_one_said(&format!("{earlier} ")), None);
        }
        assert_eq!(
            the_one_said("The Telegram token is no longer stored on this computer. Replace it in Settings."),
            Some("Telegram token no longer saved on this computer. Replace it in Settings.")
        );
    }

    /// **A sentence written onto a server before the 2026-11-01 words sweep,
    /// or by a computer on another system, still reads back as Smoozy's own**,
    /// as the sentence that stands in its place here.
    #[test]
    fn the_second_sweeps_earlier_wordings_are_still_matched() {
        use the_composing_sentences::{the_one_said, ALSO_MATCHED, EVERY_ONE, NOWHERE_TO_STORE_THE_TOKEN};
        for (also, at) in ALSO_MATCHED {
            assert_eq!(the_one_said(also), Some(EVERY_ONE[at]), "{also}");
            assert_eq!(the_one_said(&format!("{also} ")), None);
            assert!(!EVERY_ONE.contains(&also), "{also} is today's wording too");
        }
        assert_eq!(
            the_one_said("Smoozy's folder on this server is not recorded."),
            Some("Smoozy's folder on this server is unknown.")
        );
        for either in [
            "No keychain on this computer for the Telegram token.",
            "No credential store on this computer for the Telegram token.",
        ] {
            assert_eq!(the_one_said(either), Some(NOWHERE_TO_STORE_THE_TOKEN), "{either}");
        }
    }

    /// **`Telegram` §20 composed from the record and placed on a stand-in
    /// machine**: a file the machine refuses moves nothing and reports its later
    /// steps as not attempted.
    #[test]
    fn the_material_is_composed_from_the_record_and_the_answer_moves_the_entries() {
        let folder = a_folder("material");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        let row = a_server(c, "one", "i1");
        let store = AStore::default();
        assert!(matches!(
            the_material_for(c, &store, &row, true).unwrap_err(),
            WhyTheMaterialCannotBeComposed::NoBotIsSetUp
        ));
        TheProvedBotRecord::over(c)
            .the_proof_was_recorded(&TheBot { id: 7, username: "smoozy_a7k2_bot".into() }, TheConversation(9001))
            .unwrap();
        TheProvedBotRecord::over(c)
            .the_token_was_stored_for(&TheBot { id: 7, username: "smoozy_a7k2_bot".into() })
            .unwrap();
        assert!(matches!(
            the_material_for(c, &store, &row, true).unwrap_err(),
            WhyTheMaterialCannotBeComposed::TheTokenIsNotInTheStore(GetFailure::NotThere)
        ));
        // **`Security and licence` §3.0 rule 4: *not there* forgets the stored
        // fact itself** (the Creator's eleventh notes, note 2). Until
        // 2026-10-17 this test forgot it by hand here.
        assert_eq!(
            TheProvedBotRecord::over(c).the_bot_whose_token_is_stored().unwrap(),
            None,
            "a token the store no longer holds left the record saying it was stored"
        );
        store.put(&the_bot_tokens_item(), Plaintext::new(b"7:t".to_vec()), THE_STORE_CALLS_DEADLINE).unwrap();
        assert!(matches!(
            the_material_for(c, &store, &row, true).unwrap_err(),
            WhyTheMaterialCannotBeComposed::TheTokenIsNotStoredForThisBot
        ), "fix-here item 2: a token in the store with no stored fact composes nothing");
        let other = TheBot { id: 8, username: "smoozy_b8b8_bot".into() };
        TheProvedBotRecord::over(c).the_token_was_stored_for(&other).unwrap();
        assert!(matches!(
            the_material_for(c, &store, &row, true).unwrap_err(),
            WhyTheMaterialCannotBeComposed::TheTokenIsNotStoredForThisBot
        ), "fix-here item 2: a fact naming a different bot composes nothing");
        TheProvedBotRecord::over(c)
            .the_token_was_stored_for(&TheBot { id: 7, username: "smoozy_a7k2_bot".into() })
            .unwrap();
        let material = the_material_for(c, &store, &row, true).unwrap();
        assert_eq!(material.the_token_files_path(), "/mnt/one/smoozy/package/telegram-token");
        assert_eq!(material.the_configuration().machine, "i1");

        let placed = vec![
            (material.the_token_files_path().to_string(), HowTheActWent::ItWasCarriedOut),
            (
                material.the_role_configurations_path().to_string(),
                HowTheActWent::TheMachineRefusedIt { the_machines_own_words: "no".into() },
            ),
        ];
        let reached = the_placement_was_answered(c, &row.local_server_identifier, &material, &placed, at(5)).unwrap();
        assert_eq!(reached.len(), 4);
        assert!(reached[0].1.reached_the_mark());
        assert!(reached[1..].iter().all(|(_, how)| how.not_attempted.len() == 2));
        let told = what_this_server_was_told(c, &row.local_server_identifier).unwrap();
        assert_eq!(told.len(), 1, "only the file the machine took moved the record");
        assert_eq!(told[0].thing, AToldThing::TheTokenGeneration);
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// An unnamed server at `root@5.180.183.77` on `port`, holding `installation`.
    fn an_unnamed_server(connection: &rusqlite::Connection, port: u16, installation: &str) -> ServerRow {
        let mut row = a_server(connection, "unused", installation);
        row.label = None;
        row.port = Port::what_the_user_typed(port).expect("a port");
        server_rows::write(connection, &row).unwrap();
        row
    }

    #[test]
    fn twenty_first_a_colour_change_retells_the_label_item_alone() {
        let folder = a_folder("telegram-colour");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        let mut row = a_server(c, "Kitchen server", "i1");
        let mut values = every_value(TokenGeneration::THE_FIRST);
        values.push(WhatSmoozyTells::the_label(row.label.as_deref()));
        for value in values { the_telling_was_acknowledged(c, &row.local_server_identifier, &value, at(10), "i1").unwrap(); }
        assert!(the_servers_not_reached(c, &every_value(TokenGeneration::THE_FIRST)).unwrap().is_empty());
        // **The desk stores the badge code** (`src/surface/servers.rs`), never
        // a name: this test set a name until the live Telegram pass's fault 1.
        row.colour = Some("b3".into());
        server_rows::write(c, &row).unwrap();
        let not_told = the_servers_not_reached(c, &every_value(TokenGeneration::THE_FIRST)).unwrap();
        assert_eq!(not_told.len(), 1);
        assert_eq!(not_told[0].not_told, [AToldThing::TheLabel]);
        // Told the name the square is drawn from, and then reached.
        the_telling_was_acknowledged(c, &row.local_server_identifier, &WhatSmoozyTells::the_label_and_colour(row.label.as_deref(), Some("green")), at(11), "i1").unwrap();
        assert!(the_servers_not_reached(c, &every_value(TokenGeneration::THE_FIRST)).unwrap().is_empty());
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **The live Telegram pass, fault 1: the colour square was never sent.**
    /// The desk keeps a server's colour as the drawing's badge code and told it
    /// as that code; the machine's square knows only the eight names
    /// (`Telegram` L1, `Telegram` R7a). **The desk now tells the name**, from one table
    /// held here to the drawing's own palette, where each badge is named in
    /// order; a code that is none of the eight, or no colour, is told as none.
    #[test]
    fn fault_1_a_servers_colour_is_told_by_its_name_and_the_names_are_the_drawings() {
        let css = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web/the-drawing.css")).unwrap();
        for badge in crate::surface::the_rail::THE_EIGHT_BADGES {
            let line = css.lines().find(|line| line.trim_start().starts_with(&format!("--{badge}:"))).expect("the badge is in the palette");
            let named = line.split("/*").nth(1).and_then(|rest| rest.split("*/").next()).map(str::trim).expect("the palette names it");
            assert_eq!(the_colour_a_server_is_told(Some(badge)), Some(named), "{badge}");
        }
        assert_eq!(the_colour_a_server_is_told(None), None);
        assert_eq!(the_colour_a_server_is_told(Some("b9")), None);
        assert_eq!(the_colour_a_server_is_told(Some("green")), None, "a row never holds a name");
    }

    /// **The Creator's thirteenth notes, note 11: an unnamed server is told the
    /// name the server list shows**, never nothing — which the machine drew as
    /// *A server*. `username@host`, with the port only where that is what tells
    /// two unnamed servers apart (`Specs/Servers`).
    #[test]
    fn an_unnamed_servers_material_carries_the_name_the_server_list_shows() {
        let folder = a_folder("unnamed");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        let bot = TheBot { id: 7, username: "smoozy_a7k2_bot".into() };
        TheProvedBotRecord::over(c).the_proof_was_recorded(&bot, TheConversation(9001)).unwrap();
        TheProvedBotRecord::over(c).the_token_was_stored_for(&bot).unwrap();
        let store = AStore::default();
        store.put(&the_bot_tokens_item(), Plaintext::new(b"7:t".to_vec()), THE_STORE_CALLS_DEADLINE).unwrap();

        let mut alone = an_unnamed_server(c, 22, "i1");
        let material = the_material_for(c, &store, &alone, true).unwrap();
        assert_eq!(material.the_configuration().label.as_deref(), Some("root@5.180.183.77"));
        assert!(
            material.the_configuration().what_it_tells().contains(&WhatSmoozyTells::the_label(Some("root@5.180.183.77"))),
            "the told value is the name shown, not *no label*"
        );

        alone.colour = Some("b3".into());
        let coloured = the_material_for(c, &store, &alone, true).unwrap();
        assert_eq!(coloured.the_configuration().colour.as_deref(), Some("green"));
        assert!(coloured.the_configuration().what_it_tells().contains(&WhatSmoozyTells::the_label_and_colour(Some("root@5.180.183.77"), Some("green"))));

        // A second unnamed server at the same address on another port: the
        // port is what tells them apart, so both carry it.
        let beside = an_unnamed_server(c, 2222, "i2");
        let material = the_material_for(c, &store, &alone, true).unwrap();
        assert_eq!(material.the_configuration().label.as_deref(), Some("root@5.180.183.77:22"));
        let material = the_material_for(c, &store, &beside, true).unwrap();
        assert_eq!(material.the_configuration().label.as_deref(), Some("root@5.180.183.77:2222"));

        // A named server is told its name as before.
        let named = a_server(c, "Big GPU", "i3");
        let material = the_material_for(c, &store, &named, true).unwrap();
        assert_eq!(material.the_configuration().label.as_deref(), Some("Big GPU"));
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **Note 11, the servers already set up**: one told *no label* reads as not
    /// told its label, so its next connection tells it again (`Telegram` L2);
    /// told the name shown, it is reached.
    #[test]
    fn a_server_told_no_label_reads_as_not_told() {
        let folder = a_folder("nolabel");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        let row = an_unnamed_server(c, 22, "i1");
        let mut values = every_value(TokenGeneration::THE_FIRST);
        values.push(WhatSmoozyTells::the_label(None));
        for value in &values {
            the_telling_was_acknowledged(c, &row.local_server_identifier, value, at(10), "i1").unwrap();
        }
        let not_reached = the_servers_not_reached(c, &every_value(TokenGeneration::THE_FIRST)).unwrap();
        assert_eq!(not_reached.len(), 1, "a server told *no label* read as told");
        assert_eq!(not_reached[0].not_told, vec![AToldThing::TheLabel]);

        let shown = WhatSmoozyTells::the_label(Some("root@5.180.183.77"));
        the_telling_was_acknowledged(c, &row.local_server_identifier, &shown, at(11), "i1").unwrap();
        assert!(the_servers_not_reached(c, &every_value(TokenGeneration::THE_FIRST)).unwrap().is_empty());
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **fix-here item 3: a replacement save that did not answer in time is *not
    /// confirmed*, never *not stored*.**
    #[test]
    fn a_replacement_save_that_did_not_answer_is_not_confirmed() {
        struct APutThatTimesOut;
        impl SecretStore for APutThatTimesOut {
            fn put(&self, _: &HeldSecret, _: Plaintext, _: Duration) -> Result<(), StoreFailure> {
                Err(StoreFailure::from_the_deadline_passing())
            }
            fn get(&self, _: &HeldSecret, _: Duration) -> Result<Plaintext, GetFailure> {
                Err(GetFailure::NotThere)
            }
            fn erase(&self, _: &HeldSecret, _: Duration) -> Result<(), StoreFailure> {
                Ok(())
            }
            fn probe(&self, _: Duration) -> ProbeOutcome {
                ProbeOutcome::RoundTripWorked
            }
            fn enumerate(&self, _: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
                Ok(Vec::new())
            }
        }
        let folder = a_folder("late");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        TheProvedBotRecord::over(c)
            .the_proof_was_recorded(&TheBot { id: 7, username: "smoozy_a7k2_bot".into() }, TheConversation(1))
            .unwrap();
        let why = a_replacement_token_accepted(c, &APutThatTimesOut, a_passed_check_for(7, "smoozy_a7k2_bot", "7:new")).unwrap_err();
        assert!(matches!(why, WhyTheReplacementWasNotAccepted::TheTokenWasNotConfirmed), "{why:?}");
        assert_eq!(TheProvedBotRecord::over(c).the_bot_whose_token_is_stored().unwrap(), None);
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// **fix-here item 2 and `Security and licence` §5.1: an erase that goes ahead forgets
    /// the stored fact, whether the store confirmed it or it is owed.**
    #[test]
    fn erasing_a_replaced_token_forgets_the_stored_fact_whether_or_not_the_erase_was_confirmed() {
        let folder = a_folder("forgets");
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        let c = record.connection();
        let bot = TheBot { id: 7, username: "smoozy_a7k2_bot".into() };
        TheProvedBotRecord::over(c).the_proof_was_recorded(&bot, TheConversation(1)).unwrap();

        TheProvedBotRecord::over(c).the_token_was_stored_for(&bot).unwrap();
        assert_eq!(the_replaced_token_is_erased(c, &AStore::default()), Erasure::Done);
        assert_eq!(TheProvedBotRecord::over(c).the_bot_whose_token_is_stored().unwrap(), None);

        TheProvedBotRecord::over(c).the_token_was_stored_for(&bot).unwrap();
        let refusing = AStore::default();
        refusing.erases_do_not_answer.set(true);
        assert!(matches!(the_replaced_token_is_erased(c, &refusing), Erasure::ReferenceGoneAndOwedToTheSweep(_)));
        assert_eq!(TheProvedBotRecord::over(c).the_bot_whose_token_is_stored().unwrap(), None);
        drop(record);
        let _ = std::fs::remove_dir_all(folder);
    }

    /// A passed check for a bot, made through the walkthrough against the
    /// Telegram crate's stand-in service.
    fn a_passed_check_for(id: i64, username: &str, token: &str) -> TheCheckPassed {
        use smoozy_telegram::transport::a_stand_in::AStandInService;
        use smoozy_telegram::transport::TheClient;
        use smoozy_telegram::walkthrough::{check_the_bot_answers_through, ThePaste, WhatTheCheckFound};
        use std::sync::Arc;
        let service = Arc::new(AStandInService::default());
        service.then_answers(&format!(
            r#"{{"ok":true,"result":{{"id":{id},"is_bot":true,"first_name":"Smoozy","username":"{username}"}}}}"#
        ));
        let reach = move |_: &str| TheClient::over(service.clone());
        match check_the_bot_answers_through(&ThePaste::what_the_user_pasted(token), &reach) {
            WhatTheCheckFound::Answered(check) => check,
            other => panic!("the stand-in answers: {other:?}"),
        }
    }
}
