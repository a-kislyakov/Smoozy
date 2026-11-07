//! **Telegram, in Smoozy's own window: Settings' Telegram section and the
//! walkthrough that makes a bot.** Step 10 piece 8. The page half is
//! `web/telegram.js`, which `web/settings.js` hands the section to.
//!
//! **This file is the surface and not the deciding.** Every rule is somebody
//! else's and is called: the walkthrough's steps are `smoozy_telegram::
//! walkthrough`'s (step 10 piece 4), the told-record, the replacement and its
//! erase are `crate::record::told_record`'s (piece 5), and the four
//! notification switches are `crate::record::settings`'. **Nothing here
//! re-decides any of them.**
//!
//! ## What this piece draws, cited the way the runner ruled on 2026-09-12
//!
//! - `Design/Settings § Telegram` — the switch, the bot, the token held and
//!   never shown; *Change bot or token…* on the Bot row, *What a job reports* as five radios with its
//!   two lines (three until 2026-10-31; the third carries its minutes field, the
//!   drawing's `set-tgdefault`), the idle message's switch, and the servers not
//!   yet reached.
//! - `§ What this document left open`, last bullet — before Telegram is on, the
//!   switch and one paragraph and nothing else.
//! - The mockup's `#s-tg`, `#stgbot`, `#botreplace` and `#stgbotok`, and the
//!   fourteen states in [`THE_STATES_THIS_PIECE_DRAWS`].
//!
//! **The idle message's switch is the fourth notification switch, drawn a
//! second time and never kept twice.** `Telegram` V2: *"Two delivery paths sit
//! under the one list the user meets"*. So the section draws it from
//! `settings:the-screen` and turns it with Settings' own command.
//!
//! ## Rulings made here, in full in the desktop decisions record, 2026-09-14
//!
//! - **The switch and the reporting default are kept in version 20**
//!   ([`crate::record::settings::TheTelegramSettings`]). The settings store had
//!   no home for either.
//! - **The switch is written only once its act is done**: on, when a bot has
//!   been kept or a replacement accepted; off, at once. Pressing it on opens
//!   the walkthrough and draws nothing new until then — **unless a bot is
//!   already set up**: since 2026-10-31 off keeps the token, and on uses that
//!   bot with no walkthrough (the Creator's fifteenth notes, note 1).
//! - ***Change bot or token…* opens the one walkthrough**, the
//!   only token dialog Design draws, and `Telegram` P4 decides by the bot's
//!   identity whether what was pasted is a replacement (`Telegram` L4) or a
//!   new bot (`Telegram` S5).
//! - **Two sentences Design does not draw**: a paste with no token in it, and a
//!   check that ended in a way other than the two drawn readings. Each is its
//!   own plain sentence and neither says *Telegram did not accept*.
//! - **Nothing here raises a failure record**, so `WhoseSentenceItIs` is not
//!   reached: every failure is the answer to the press just made, in the one
//!   failure shape, gone at the next reading — Settings' own rule.
//! - **The link to the bot is drawn as an address with nowhere to go.** This
//!   page is granted no opener, so the user opens it in Telegram.
//!
//! ## The bot step as drawn again on 2026-10-24 — the seventeenth notes' piece 7
//!
//! `Design/Settings § Telegram` and `Telegram` S5, `Telegram` S6, `Telegram` S6a,
//! `Telegram` P4 and `Telegram` K1, as the
//! two design passes of 2026-10-23 and 2026-10-24 drew them.
//!
//! - **The route is the page's and never reaches this file.** *Create a new bot*
//!   and *Use an existing bot* each hand their paste to the one check, and the
//!   bot the check returned decides what follows (`Telegram` S3a, `Telegram` P4) — the route cannot,
//!   because a user can paste any token on either. **S5 holds either way**: the
//!   desktop waits for Start only on a bot this computer has no proof for, and
//!   the existing-bot route's instructions are what make that token one no
//!   server holds (`Telegram` S6a: *"No affirmation is asked"*).
//! - **The bot this computer already uses waits for *Save token*.** Until this
//!   day it was stored the moment the check passed, with nothing drawn between;
//!   the drawing's `set-bot-replace` puts the box and the press there, and
//!   Cancel stores nothing.
//! - **The name copied on entering Start is the check's answer, bare, once per
//!   checked bot** (`Telegram` S6a), and this file says when: the first drawing of the
//!   wait carries `copyTheName`, and no later one does. **The copying is the
//!   page's**, because this application binds no clipboard service
//!   (`web/keys.js` says the same), so a test drives it with a stand-in and
//!   never the real clipboard.
//!
//! **Clause markers in this file belong to other documents and are written with
//! that document at every site.**

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, SystemTime};

use tauri::{AppHandle, Manager, Runtime};

use smoozy_security_and_licence::secret_store::SecretStore;
use smoozy_telegram::proof::{KeepsTheProvedBot, TheBot, TheConversation};
use smoozy_telegram::told::WhatSmoozyTells;
use smoozy_telegram::walkthrough::{
    check_the_bot_answers, TheCheckPassed, TheHelloWasSaid, TheOffer, ThePaste, WhatComesNext,
    WhatOneAskFound, WhatTheCheckFound, WhyTheBotWasNotKept, WhyTheCheckDidNotFinish,
};

use crate::diary_service::ModulesDiary;
use crate::events::{Event, Said, Subject, TheOneQueue};
use crate::notifications::kinds::Kind;
use crate::record::settings::{self as the_store, TheTelegramSettings};
use crate::record::telegram_bot::{a_bot_set_up, TheProvedBotRecord};
use crate::record::told_record;
use crate::start::Started;
use crate::typography::{Prose, Verbatim};

/// **The section.** `web/telegram.js` names the same string.
pub const THE_SECTION: &str = "telegram:the-section";

/// **The walkthrough dialog.** `web/telegram.js` names the same string.
pub const THE_WALKTHROUGH: &str = "telegram:the-walkthrough";

/// **The fourteen mockup states this file draws**, the drawing's own names: the
/// section before Telegram is on, and the thirteen of the bot step as drawn
/// again on 2026-10-24 (the seventeenth notes' piece 7). A test reads them out
/// of `Preparation/Design/Mockup/Smoozy.html`.
pub const THE_STATES_THIS_PIECE_DRAWS: [&str; 14] = [
    "set-tgoff",
    "set-bot",
    "set-bot-new",
    "set-bot-existing",
    "set-bot-existingcheck",
    "set-bot-existingbad",
    "set-bot-existingnet",
    "set-bot-replace",
    "set-bot-copyfail",
    "set-botcheck",
    "set-botbad",
    "set-botnet",
    "set-bot-ok",
    "set-bot-start",
];

/// **The diary declaration piece 4 registered** at start.
const THE_MODULE: &str = "Telegram";

/// **H34: a paste longer than this is refused before anything is touched.** The
/// bot-making chat's whole reply is a few hundred characters.
const THE_LONGEST_A_PASTE_MAY_BE: usize = 8_192;

/// **How long one ask for Start is held open**, and how long a failed ask or a
/// hello that did not go waits before the next.
const ONE_ASK_WAITS_SECONDS: u32 = 20;
const AFTER_A_FAILED_CALL: Duration = Duration::from_secs(2);

mod words {
    // `Design/Settings § Telegram`, in the drawing's words.
    pub const TELEGRAM: &str = "Telegram";
    pub const BEFORE_IT_IS_ON: &str = "Job progress and finished frames on Telegram.";
    pub const SMOOZY_CANNOT_MAKE_THE_BOT: &str = "You set up the bot in Telegram.";
    pub const THE_SWITCH: &str = "Report to Telegram";
    pub const BOT: &str = "Bot";
    pub const TOKEN: &str = "Token";
    pub const USE_ANOTHER_BOT: &str = "Change bot or token…";
    pub const YOU_MAKE_THE_BOT: &str = "Keep the token secret.";
    pub const WHOEVER_HOLDS_IT: &str = "Anyone with it can control the bot.";
    pub const WHAT_A_JOB_REPORTS: &str = "What a job reports";
    pub const EVERYTHING: &str = "Everything";
    pub const EVERYTHING_IS: &str = "progress, frames, a short video and a contact sheet";
    pub const PICTURES_ONLY: &str = "Pictures only";
    pub const PICTURES_ONLY_IS: &str = "the same, without progress";
    pub const ONLY_WHAT_NEEDS_YOU: &str = "Alerts only";
    pub const ONLY_WHAT_NEEDS_YOU_IS: &str = "endings and questions";
    // The five from 2026-10-31 (`Design/Settings § Telegram`, the drawing's
    // `set-tgdefault`): the third reads *A picture every [5] minutes*, its
    // field between the words.
    pub const A_PICTURE_EVERY: &str = "A picture every";
    pub const MINUTES: &str = "minutes";
    pub const MINUTES_BETWEEN_PICTURES: &str = "Minutes between pictures";
    pub const A_PICTURE_EVERY_FEW_MINUTES_IS: &str = "the same as Everything, with fewer pictures";
    pub const OFF: &str = "Off";
    pub const OFF_IS: &str = "nothing, unless you ask";
    pub const WHOLE_MINUTES_1_TO_60: &str = "Whole minutes, 1 to 60.";
    pub const ALWAYS_SENT: &str = "All but Off send endings and questions.";
    pub const THIS_IS_THE_DEFAULT: &str = "This is the default.";
    pub const A_JOB_CAN_BE_SET_DIFFERENTLY: &str = "Change it per job when you submit, or for all jobs with /reporting in Telegram.";
    pub const SERVERS_NOT_REACHED: &str = "servers without this change yet:";
    pub const ONE_SERVER_NOT_REACHED: &str = "server without this change yet:";
    pub const THEY_GET_IT: &str = "Sent when you next connect to them.";
    pub const IT_GETS_IT: &str = "Sent when you next connect to it.";
    // Answers to a press in the section. **Design gives no words for these.**
    pub const THE_SETTINGS_WOULD_NOT_READ: &str = "Could not read your Telegram settings.";
    pub const THE_SWITCH_DID_NOT_MOVE: &str = "Could not switch Telegram off.";
    // The fifteenth notes, note 1: on, with the bot already set up, where the
    // switch would not save.
    pub const THE_SWITCH_DID_NOT_MOVE_ON: &str = "Could not switch Telegram on.";
    pub const THE_SETTING_DID_NOT_SAVE: &str = "Could not save the setting.";
    // **fix-here item 2 of the same round**: the record does not say a token is stored for
    // the bot on record, so no dots are drawn.
    pub const THE_TOKEN_IS_NOT_STORED: &str = "Token not stored on this computer.";
    pub const HOW_TO_STORE_IT: &str = "Replace the token to store it.";

    // The mockup's `#stgbot`, drawn again 2026-10-24 (the seventeenth notes'
    // piece 7): the choice alone first, then each route's own steps.
    pub const SET_UP_TELEGRAM: &str = "Set up Telegram";
    pub const DO_THIS_WHERE_YOU_CAN_REACH_TELEGRAM: &str = "Do this on a computer where you use Telegram.";
    pub const CHOOSE_A_BOT_ROUTE: &str = "How to set up the bot";
    pub const MAKE_A_NEW_BOT: &str = "Create a new bot";
    pub const USE_AN_EXISTING_BOT: &str = "Use an existing bot";
    // *Use an existing bot* — `Design/Settings § Telegram`, `Telegram` S6a.
    pub const A_NEW_TOKEN_IS_NEEDED: &str = "Your bot needs a new token, one no server is using.";
    pub const AND_REVOKE_ITS_TOKEN: &str = "and revoke your bot\u{2019}s token.";
    pub const YOU_GET_A_NEW_ONE: &str = "You get a new one.";
    pub const PASTE_THE_NEW_TOKEN_HERE: &str = "Paste the new token here.";
    pub const THE_OLD_TOKENS_SERVERS_STOP: &str = "Servers on the old token stop reporting until you next connect to them.";
    // *Create a new bot*.
    pub const IN_TELEGRAM_OPEN: &str = "In Telegram, open";
    pub const AND_SEND: &str = "and send";
    pub const IT_ASKS_FOR_TWO_NAMES: &str = "It asks for two names.";
    pub const COPY_EACH_OF_THESE: &str = "Copy each of these:";
    pub const NAME: &str = "Name";
    pub const USERNAME: &str = "Username";
    pub const COPY_NAME: &str = "Copy name";
    pub const COPY_USERNAME: &str = "Copy username";
    pub const ANOTHER: &str = "Suggest another";
    pub const THE_TOKEN_LETS_SMOOZY_USE_YOUR_BOT: &str = "BotFather replies with a token.";
    pub const PASTE_BOTFATHERS_REPLY_HERE: &str = "Paste the reply here.";
    pub const PASTE_HERE: &str = "Paste here";
    pub const WHAT_BOTFATHER_SENT_BACK: &str = "What BotFather sent back";
    pub const NOTHING_IS_SAVED_BEFORE: &str = "Nothing is saved until the bot is checked.";
    pub const CANCEL: &str = "Cancel";
    pub const CHECK_BOT: &str = "Check bot";
    pub const TRY_AGAIN: &str = "Retry";
    // The mockup's `#botreplace`: the bot this computer already uses
    // (`Telegram` P4), so no Start.
    pub const IS_THE_BOT_SMOOZY_ALREADY_USES: &str = "is already set up.";
    pub const NO_START_IS_NEEDED: &str = "No Start needed.";
    pub const SERVERS_GET_THE_NEW_TOKEN: &str = "Servers get the new token when you next connect to them.";
    pub const SAVE_TOKEN: &str = "Save token";
    // `Design/Settings § Telegram`: *"its failure has two readings, not one"*.
    pub const NOT_ACCEPTED: &str = "Telegram did not accept that token.";
    pub const CHECK_THE_PASTE: &str = "Check the paste and try again.";
    pub const NOT_REACHED: &str = "Could not reach Telegram.";
    pub const YOUR_PASTE_IS_KEPT: &str = "Your paste is kept.";
    // **Neither reading is drawn for these, and neither may be said for them.**
    pub const NO_TOKEN_IN_THE_PASTE: &str = "No token in that paste.";
    pub const THE_ANSWER_COULD_NOT_BE_USED: &str = "Unexpected response from Telegram.";
    pub const THE_BOT_RECORD_DID_NOT_SAVE: &str = "Could not save the bot to Smoozy’s database. Check free disk space and try again.";
    #[cfg(target_os = "macos")]
    pub const THE_TOKEN_STORE_REFUSED: &str = "Your Mac’s keychain refused to save the token. Allow Smoozy access and try again.";
    #[cfg(not(target_os = "macos"))]
    pub const THE_TOKEN_STORE_REFUSED: &str = "Your computer refused to save the token. Allow Smoozy access and try again.";
    pub const THE_TOKEN_REMOVAL_RECORD_FAILED: &str = "Could not read Smoozy’s database. Try again.";
    pub const THE_SAVED_TOKEN_RECORD_FAILED: &str = "The token was saved, but Smoozy’s database was not updated. Check free disk space and try again.";
    #[cfg(target_os = "macos")]
    pub const THE_SECRET_STORE_UNAVAILABLE: &str = "Your Mac’s keychain is unavailable. Restore access and try again.";
    #[cfg(not(target_os = "macos"))]
    pub const THE_SECRET_STORE_UNAVAILABLE: &str = "Your computer’s credential store is unavailable. Restore access and try again.";
    pub const TELEGRAM_COULD_NOT_BE_ENABLED: &str = "The bot was saved, but Telegram reporting could not be enabled. Check free disk space and try again.";
    pub const THE_TOKEN_WAS_NOT_STORED: &str = "Could not save the new token.";
    // **fix-here item 5 of the round *setting it up, and what the desktop keeps***, in the
    // licence key screen's pattern: a save that did not answer in time
    // (`Security and licence` §3.0 rule 5 as amended).
    pub const THE_TOKEN_WAS_NOT_CONFIRMED: &str = "Saving the token was not confirmed. Allow Smoozy access and try again.";

    // The mockup's `#stgbotok`.
    pub const THE_BOT_ANSWERED: &str = "Token accepted.";
    pub const TWO_THINGS_LEFT: &str = "Two steps left in Telegram.";
    pub const OPEN_YOUR_BOT: &str = "Open your bot.";
    // `Telegram` S6a: the checked name, copied once on entering Start and
    // again from the icon; a failed copy is said in place and blocks nothing.
    pub const COPY_BOT_NAME: &str = "Copy bot name";
    pub const BOT_NAME_COPIED: &str = "Bot name copied.";
    pub const YOU_CAN_PASTE_IT: &str = "Paste it into Telegram\u{2019}s search.";
    pub const THE_BOT_NAME_WAS_NOT_COPIED: &str = "Could not copy the bot name.";
    pub const USE_THE_COPY_ICON: &str = "Use the copy icon.";
    pub const PRESS: &str = "Press";
    pub const START: &str = "Start";
    pub const UNTIL_YOU_DO: &str = "The bot cannot message you until you do.";
    pub const WAITING_FOR_START: &str = "Waiting for you to press Start.";
    pub const THE_BOT_REPLIED: &str = "The bot replied.";
    pub const TELEGRAM_IS_SET_UP: &str = "Telegram is set up.";
    pub const DONE: &str = "Done";
}

fn prose(event: Event, name: &'static str, said: &'static str) -> Event {
    event.saying(name, Said::Prose(Prose::new(said)))
}

fn value(event: Event, name: &'static str, said: impl Into<String>) -> Event {
    event.saying(name, Said::Value(Verbatim::new(said.into())))
}

/// An indexed field name, made once and kept — the same reason as
/// `crate::surface::settings`' own.
fn a_name(prefix: &str, at: usize) -> &'static str {
    static NAMES: OnceLock<Mutex<Vec<&'static str>>> = OnceLock::new();
    let key = format!("{prefix}{at}");
    let mut held = NAMES.get_or_init(Default::default).lock().unwrap_or_else(|p| p.into_inner());
    if let Some(name) = held.iter().find(|name| **name == key) {
        return name;
    }
    let name: &'static str = Box::leak(key.into_boxed_str());
    held.push(name);
    name
}

// ════════════════════════════════════════════════════════════════════════════
// What a job reports — H34's closed request type
// ════════════════════════════════════════════════════════════════════════════

/// **The five, as the page hands one back** — three until 2026-10-31, when
/// `Telegram` V1d made the app-wide item any of the five (`Telegram` V1a as
/// amended). Their identifiers are
/// `crate::surface::render_settings::REPORTING_CHOICES`' own, in its order, and
/// a test holds the two together. **The third's minutes are not in the
/// identifier**: they are the record's own item beside it (version 56), typed
/// in the row's field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TheReportingChoice {
    Everything,
    PicturesOnly,
    APictureEveryFewMinutes,
    OnlyWhatNeedsYou,
    Nothing,
}

impl TheReportingChoice {
    pub const ALL: [TheReportingChoice; 5] = [
        TheReportingChoice::Everything,
        TheReportingChoice::PicturesOnly,
        TheReportingChoice::APictureEveryFewMinutes,
        TheReportingChoice::OnlyWhatNeedsYou,
        TheReportingChoice::Nothing,
    ];

    pub fn stored_as(self) -> &'static str {
        match self {
            TheReportingChoice::Everything => "everything",
            TheReportingChoice::PicturesOnly => "pictures-only",
            TheReportingChoice::APictureEveryFewMinutes => crate::surface::render_settings::A_PICTURE_EVERY_FEW_MINUTES,
            TheReportingChoice::OnlyWhatNeedsYou => "only-what-needs-you",
            TheReportingChoice::Nothing => "nothing",
        }
    }

    /// The row's name and what it sends. **The third's name is the words
    /// before its minutes field**; *minutes* follows the field.
    fn words(self) -> (&'static str, &'static str) {
        match self {
            TheReportingChoice::Everything => (words::EVERYTHING, words::EVERYTHING_IS),
            TheReportingChoice::PicturesOnly => (words::PICTURES_ONLY, words::PICTURES_ONLY_IS),
            TheReportingChoice::APictureEveryFewMinutes => (words::A_PICTURE_EVERY, words::A_PICTURE_EVERY_FEW_MINUTES_IS),
            TheReportingChoice::OnlyWhatNeedsYou => (words::ONLY_WHAT_NEEDS_YOU, words::ONLY_WHAT_NEEDS_YOU_IS),
            TheReportingChoice::Nothing => (words::OFF, words::OFF_IS),
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// The section
// ════════════════════════════════════════════════════════════════════════════

/// **What the section is drawn from.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheSectionHolds {
    /// **`None` is a record that did not answer**, and then nothing but the
    /// sentence saying so is drawn (L8a).
    pub the_settings: Option<TheTelegramSettings>,
    /// The bot on record (`Telegram` S3a).
    pub the_bot: Option<TheBot>,
    /// **Whether the record says a token is stored for the bot on record**
    /// (`Security and licence` §3.0 rule 6). The dots are drawn from this and
    /// from nothing else — fix-here item 2 of the round *setting it up, and what the
    /// desktop keeps*.
    pub the_token_is_held: bool,
    /// **`Telegram` L3: each server not reached**, by the name the user reads.
    /// **`None` is a told-record that did not read**, which is said rather
    /// than drawn as nobody (fix-here item 11).
    pub not_reached: Option<Vec<String>>,
    /// The answer to the press just made, where it did not land.
    pub the_answer: Option<&'static str>,
    /// **Minutes typed that Smoozy will not take**, said under the radios in
    /// the refusal colour, the field turned red, the minutes that stood kept
    /// (`Design/Settings § Telegram`: *"refused outside 1 to 60 the way the
    /// Render Settings field is"*). Gone at the next reading.
    pub the_minutes_refused: Option<&'static str>,
}

/// **The minutes the third row's field shows**: the record's, else 5
/// (`Specs/Telegram`, *Fewer pictures, on a clock*: *"5 until the user types
/// another"*).
fn the_default_minutes_shown(settings: &TheTelegramSettings) -> u8 {
    settings
        .the_default_minutes
        .unwrap_or(smoozy_render_settings::reporting::THE_MINUTES_TO_BEGIN_WITH)
}

/// **What the section says.** Before Telegram is on it says the paragraph and
/// the switch and nothing else (`Design/Settings § What this document left
/// open`); once on, the bot, the token, the reporting default and its lines.
/// **Every name is a value** (X7): the username, the token's mask, each
/// server not reached.
pub fn the_section_says(holds: &WhatTheSectionHolds) -> Event {
    let mut event = Event::about(Subject::TheRun, THE_SECTION);
    event = prose(event, "heading", words::TELEGRAM);
    if let Some(sentence) = holds.the_answer {
        event = prose(event, "theAnswer", sentence);
    }
    let Some(settings) = holds.the_settings.as_ref() else {
        return prose(event, "theSettingsWouldNotRead", words::THE_SETTINGS_WOULD_NOT_READ);
    };
    event = prose(event, "theSwitch", words::THE_SWITCH);
    event = prose(event, "theSwitchsName", words::TELEGRAM);
    event = value(event, "switchedOn", if settings.switched_on { "yes" } else { "no" });
    if !settings.switched_on {
        event = prose(event, "beforeItIsOn", words::BEFORE_IT_IS_ON);
        event = prose(event, "smoozyCannotMakeTheBot", words::SMOOZY_CANNOT_MAKE_THE_BOT);
    } else {
        for (name, said) in [
            ("bot", words::BOT),
            ("token", words::TOKEN),
            ("useAnotherBot", words::USE_ANOTHER_BOT),
            ("youMakeTheBot", words::YOU_MAKE_THE_BOT),
            ("whoeverHoldsIt", words::WHOEVER_HOLDS_IT),
            ("whatAJobReports", words::WHAT_A_JOB_REPORTS),
            ("alwaysSent", words::ALWAYS_SENT),
            ("thisIsTheDefault", words::THIS_IS_THE_DEFAULT),
            ("aJobCanBeSetDifferently", words::A_JOB_CAN_BE_SET_DIFFERENTLY),
        ] {
            event = prose(event, name, said);
        }
        if let Some(bot) = holds.the_bot.as_ref() {
            // `Telegram` S3a: *"The Settings Bot reading is @ plus that stored
            // username."*
            event = value(event, "theBotsUsername", format!("@{}", bot.username));
        }
        // **The token is held and never shown** — a mask, and no part of it.
        // **Drawn only where the record says a token is stored for this bot**;
        // otherwise a plain line with the way to finish (fix-here item 2).
        if holds.the_token_is_held {
            event = value(event, "theTokenIsHeld", "\u{2022}".repeat(20));
        } else {
            event = prose(event, "theTokenIsNotStored", words::THE_TOKEN_IS_NOT_STORED);
            event = prose(event, "howToStoreIt", words::HOW_TO_STORE_IT);
        }
        // **No default chosen is the first of the five**, which is the choice
        // a submission already carries when nothing was chosen.
        let chosen = settings
            .the_reporting_default
            .as_deref()
            .unwrap_or(TheReportingChoice::Everything.stored_as());
        for (at, choice) in TheReportingChoice::ALL.iter().enumerate() {
            let (name, what) = choice.words();
            event = value(event, a_name("choiceIs", at), choice.stored_as());
            event = prose(event, a_name("choiceName", at), name);
            event = prose(event, a_name("choiceWhat", at), what);
        }
        event = value(event, "theDefaultIs", chosen);
        // **The third row's field, drawn whichever row is picked** (the
        // drawing's `set-tgdefault`): the minutes as a value, the word after
        // it and the field's name as prose.
        event = value(event, "theMinutesChoiceIs", TheReportingChoice::APictureEveryFewMinutes.stored_as());
        event = value(event, "theDefaultMinutes", the_default_minutes_shown(settings).to_string());
        event = prose(event, "minutes", words::MINUTES);
        event = prose(event, "minutesBetweenPictures", words::MINUTES_BETWEEN_PICTURES);
        if let Some(refused) = holds.the_minutes_refused {
            event = prose(event, "theMinutesAreRefused", refused);
        }
    }
    match holds.not_reached.as_ref() {
        // **fix-here item 11: the told-record did not read**, and the line says so in the
        // section's own words rather than drawing nobody.
        None => {
            event = prose(event, "notReachedWouldNotRead", words::THE_SETTINGS_WOULD_NOT_READ);
        }
        Some(not_reached) if !not_reached.is_empty() => {
            let one = not_reached.len() == 1;
            event = value(event, "notReachedCount", not_reached.len().to_string());
            event = prose(event, "notReachedLead", if one { words::ONE_SERVER_NOT_REACHED } else { words::SERVERS_NOT_REACHED });
            event = prose(event, "notReachedTail", if one { words::IT_GETS_IT } else { words::THEY_GET_IT });
            for (at, name) in not_reached.iter().enumerate() {
                event = value(event, a_name("notReached", at), name.clone());
            }
        }
        Some(_) => {}
    }
    event
}

/// **`Telegram` L3's current values, for every server alike.** On: the role
/// enabled, the generation, the proved conversation and the idle switch. Off:
/// the role disabled, and nothing else is owed to a server. **Also what
/// `crate::setting_up` delivers on a connection** (the 38's piece 11), so the
/// current values have one home.
pub(crate) fn what_smoozy_tells_now(
    connection: &rusqlite::Connection,
    settings: &TheTelegramSettings,
    conversation: Option<TheConversation>,
) -> Option<Vec<WhatSmoozyTells>> {
    if !settings.switched_on {
        return Some(vec![WhatSmoozyTells::the_roles_desired_state(false)]);
    }
    let generation = told_record::the_current_generation(connection).ok()?;
    let idle = the_store::read(connection).ok()?.the_switches.attempts(Kind::AMachineHasBeenIdle);
    let mut now = vec![
        WhatSmoozyTells::the_roles_desired_state(true),
        WhatSmoozyTells::the_token_generation(generation),
        WhatSmoozyTells::the_reporting_settings([(the_store::the_stored_name_of(Kind::AMachineHasBeenIdle), idle)]),
    ];
    if let Some(conversation) = conversation {
        now.push(WhatSmoozyTells::the_proved_conversation(conversation));
    }
    Some(now)
}

/// **What the section holds now**, read fresh.
///
/// **A server Smoozy has never told anything is not named, with Telegram on or
/// off**: it was never given Telegram, so the next connection does not bring
/// it, and the set-up card's *Install* line is the honest place for it.
/// Ruled for the off state by piece 8 and narrowed to both states on
/// 2026-09-14 by fix-here item 10 of the round *setting it up, and what the desktop keeps*.
pub fn what_the_section_holds(connection: &rusqlite::Connection, the_answer: Option<&'static str>) -> WhatTheSectionHolds {
    let Ok(settings) = the_store::the_telegram_settings(connection) else {
        return WhatTheSectionHolds {
            the_settings: None,
            the_bot: None,
            the_token_is_held: false,
            not_reached: Some(Vec::new()),
            the_answer,
            the_minutes_refused: None,
        };
    };
    let keeper = TheProvedBotRecord::over(connection);
    let proved = keeper.the_proved_bot().ok().flatten();
    // `Security and licence` §3.0 rule 6: a stored token for the bot on
    // record, answered from the record; a read that did not answer claims
    // nothing.
    let the_token_is_held = match (proved.as_ref(), keeper.the_bot_whose_token_is_stored()) {
        (Some((bot, _)), Ok(Some(stored_for))) => bot.id == stored_for,
        _ => false,
    };
    let not_reached = what_smoozy_tells_now(connection, &settings, proved.as_ref().map(|(_, c)| *c))
        .and_then(|now| the_servers_named_as_not_reached(connection, &now));
    WhatTheSectionHolds {
        the_settings: Some(settings),
        the_bot: proved.map(|(bot, _)| bot),
        the_token_is_held,
        not_reached,
        the_answer,
        the_minutes_refused: None,
    }
}

/// **`Telegram` L3's servers, by the name the user reads, or `None` where the
/// told-record did not read** (fix-here item 11). Only servers that have been told
/// something are named (fix-here item 10), and each is named the way the servers screen
/// names it, never by its internal identifier (fix-here item 12).
fn the_servers_named_as_not_reached(connection: &rusqlite::Connection, now: &[WhatSmoozyTells]) -> Option<Vec<String>> {
    let rows = crate::record::server_rows::all(connection).ok()?;
    let mut names = Vec::new();
    for server in told_record::the_servers_not_reached(connection, now).ok()? {
        if told_record::what_this_server_was_told(connection, &server.server).ok()?.is_empty() {
            continue;
        }
        if let Some(row) = rows.iter().find(|row| row.local_server_identifier == server.server) {
            names.push(crate::surface::servers::what_a_server_is_called(row));
        }
    }
    Some(names)
}

/// **The section, posted** — from `the_settings_screen` and after every act
/// here. A record that never opened posts nothing, as Settings' own sections do.
pub fn the_section_is_posted(started: &Started, queue: &TheOneQueue, the_answer: Option<&'static str>) {
    let Some(record) = started.record.as_ref() else {
        return;
    };
    let holds = what_the_section_holds(record.connection(), the_answer);
    let _ = queue.read_then_post::<WhatTheSectionHolds, std::convert::Infallible>(|| Ok(holds), the_section_says);
}

/// **The section drawn again with the minutes typed refused** — the range said
/// under the radios, the minutes that stood in the field.
fn the_section_is_posted_with_the_minutes_refused<R: Runtime>(app: &AppHandle<R>) {
    let (Some(started), Some(queue)) = (app.try_state::<Mutex<Started>>(), app.try_state::<Arc<TheOneQueue>>()) else {
        return;
    };
    let Ok(started) = started.lock() else {
        return;
    };
    let Some(record) = started.record.as_ref() else {
        return;
    };
    let mut holds = what_the_section_holds(record.connection(), None);
    holds.the_minutes_refused = Some(words::WHOLE_MINUTES_1_TO_60);
    let _ = queue.read_then_post::<WhatTheSectionHolds, std::convert::Infallible>(|| Ok(holds), the_section_says);
}

// ════════════════════════════════════════════════════════════════════════════
// The walkthrough dialog
// ════════════════════════════════════════════════════════════════════════════

/// **Which of the three boxes shows, and in which of its states.**
///
/// **Which route the user chose is the page's and never comes back here**
/// (`Design/Settings § Telegram`, drawn again 2026-10-24). Both routes hand
/// their paste to the one check, and what follows it is decided by the bot the
/// check returned (`Telegram` S3a, `Telegram` P4), never by the route: a paste on *Make a
/// new bot* that names the bot this computer already uses is still that bot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheStage {
    /// `set-bot`, `set-bot-new`, `set-bot-existing`: the choice, and whichever
    /// route the page has open.
    ThePaste,
    /// `set-botbad`, `set-botnet`, `set-bot-existingbad`,
    /// `set-bot-existingnet`, and the readings Design does not draw: back at
    /// the paste, on the route it was on, with a failure. `retry` is the act its
    /// control makes.
    AFailureAtThePaste { sentence: &'static str, what_to_do: Option<&'static str>, retry: Option<&'static str> },
    /// `set-bot-replace`: **the check returned the bot this computer already
    /// uses** (`Telegram` P4), so no Start; *Save token* keeps the new token.
    /// `username` is the check's answer (`Telegram` S3a), bare.
    TheBotAlreadyUsed { username: String },
    /// `set-bot-ok`, `set-bot-copyfail`: the link, and waiting for Start.
    /// `username` is the check's answer, bare. **`not_reached`: the last ask or
    /// hello did not reach Telegram**, and Design's *Could not reach
    /// Telegram* is shown while the retrying goes on (fix-here item 8).
    /// **`copy_the_name`: this is the step's first drawing for this checked
    /// bot**, and the page copies the name now (`Telegram` S6a: *"On the first
    /// entry to S6 per checked candidate"*); every later drawing of the same
    /// wait copies nothing.
    WaitingForStart { username: String, not_reached: bool, copy_the_name: bool },
    /// `set-bot-start`: the reply arrived and the bot is kept.
    TheBotReplied { username: String },
    /// **The Start arrived and hello was said, and keeping did not finish** —
    /// its own stage, with no waiting line (fix-here item 14), and the sentence saying why
    /// (fix-here item 5).
    NotKept { username: String, sentence: &'static str },
    /// Every box shut.
    Shut,
}

/// The two acts a failure's control can make.
pub const CHECK_AGAIN: &str = "check-again";
pub const KEEP_AGAIN: &str = "keep-again";

/// **What the walkthrough says.** `username` is the candidate on offer
/// (`Telegram` S1a);
/// **the paste is never on it** (`Telegram` S2).
pub fn the_walkthrough_says(stage: &TheStage, username: Option<&str>) -> Event {
    let mut event = Event::about(Subject::TheRun, THE_WALKTHROUGH);
    for (name, said) in [
        ("title", words::SET_UP_TELEGRAM),
        ("doThisWhereYouCanReachTelegram", words::DO_THIS_WHERE_YOU_CAN_REACH_TELEGRAM),
        ("chooseABotRoute", words::CHOOSE_A_BOT_ROUTE),
        ("makeANewBot", words::MAKE_A_NEW_BOT),
        ("useAnExistingBot", words::USE_AN_EXISTING_BOT),
        ("aNewTokenIsNeeded", words::A_NEW_TOKEN_IS_NEEDED),
        ("andRevokeItsToken", words::AND_REVOKE_ITS_TOKEN),
        ("youGetANewOne", words::YOU_GET_A_NEW_ONE),
        ("pasteTheNewTokenHere", words::PASTE_THE_NEW_TOKEN_HERE),
        ("theOldTokensServersStop", words::THE_OLD_TOKENS_SERVERS_STOP),
        ("inTelegramOpen", words::IN_TELEGRAM_OPEN),
        ("andSend", words::AND_SEND),
        ("itAsksForTwoNames", words::IT_ASKS_FOR_TWO_NAMES),
        ("copyEachOfThese", words::COPY_EACH_OF_THESE),
        ("name", words::NAME),
        ("username", words::USERNAME),
        ("copyName", words::COPY_NAME),
        ("copyUsername", words::COPY_USERNAME),
        ("another", words::ANOTHER),
        ("theTokenLetsSmoozyUseYourBot", words::THE_TOKEN_LETS_SMOOZY_USE_YOUR_BOT),
        ("pasteBotFathersReplyHere", words::PASTE_BOTFATHERS_REPLY_HERE),
        ("pasteHere", words::PASTE_HERE),
        ("whatBotFatherSentBack", words::WHAT_BOTFATHER_SENT_BACK),
        ("nothingIsSavedBefore", words::NOTHING_IS_SAVED_BEFORE),
        ("cancel", words::CANCEL),
        ("checkBot", words::CHECK_BOT),
        ("tryAgain", words::TRY_AGAIN),
        ("isTheBotSmoozyAlreadyUses", words::IS_THE_BOT_SMOOZY_ALREADY_USES),
        ("noStartIsNeeded", words::NO_START_IS_NEEDED),
        ("serversGetTheNewToken", words::SERVERS_GET_THE_NEW_TOKEN),
        ("saveToken", words::SAVE_TOKEN),
        ("theBotAnswered", words::THE_BOT_ANSWERED),
        ("twoThingsLeft", words::TWO_THINGS_LEFT),
        ("openYourBot", words::OPEN_YOUR_BOT),
        ("copyBotName", words::COPY_BOT_NAME),
        ("botNameCopied", words::BOT_NAME_COPIED),
        ("youCanPasteIt", words::YOU_CAN_PASTE_IT),
        ("theBotNameWasNotCopied", words::THE_BOT_NAME_WAS_NOT_COPIED),
        ("useTheCopyIcon", words::USE_THE_COPY_ICON),
        ("press", words::PRESS),
        ("start", words::START),
        ("untilYouDo", words::UNTIL_YOU_DO),
        ("waitingForStart", words::WAITING_FOR_START),
        ("theBotReplied", words::THE_BOT_REPLIED),
        ("telegramIsSetUp", words::TELEGRAM_IS_SET_UP),
        ("done", words::DONE),
    ] {
        event = prose(event, name, said);
    }
    // Names Telegram and Smoozy fix, each a value (X7).
    event = value(event, "theBotMaker", "@BotFather");
    event = value(event, "theCommand", "/newbot");
    event = value(event, "theName", smoozy_telegram::walkthrough::THE_BOTS_NAME);
    if let Some(username) = username {
        event = value(event, "theUsername", username);
    }
    let (stage_is, checked) = match stage {
        TheStage::ThePaste => ("paste", None),
        TheStage::AFailureAtThePaste { sentence, what_to_do, retry } => {
            event = prose(event, "theFailure", sentence);
            if let Some(what_to_do) = what_to_do {
                event = prose(event, "whatToDoAboutIt", what_to_do);
            }
            if let Some(retry) = retry {
                event = value(event, "retry", *retry);
            }
            ("paste", None)
        }
        TheStage::TheBotAlreadyUsed { username } => {
            // `Telegram` S3a: *"The Settings Bot reading is @ plus that stored
            // username"*, and this box names the bot the way that reading does.
            event = value(event, "theBotsUsername", format!("@{username}"));
            ("replace", None)
        }
        TheStage::WaitingForStart { username, not_reached, copy_the_name } => {
            if *not_reached {
                event = prose(event, "theFailure", words::NOT_REACHED);
            }
            if *copy_the_name {
                event = value(event, "copyTheName", "yes");
            }
            ("waiting", Some(username))
        }
        TheStage::TheBotReplied { username } => ("replied", Some(username)),
        TheStage::NotKept { username, sentence } => {
            event = prose(event, "theFailure", sentence);
            event = value(event, "retry", KEEP_AGAIN);
            ("notkept", Some(username))
        }
        TheStage::Shut => ("shut", None),
    };
    event = value(event, "stage", stage_is);
    if let Some(username) = checked {
        // `Telegram` S6: composed from `Telegram` S3a's answered username,
        // **drawn as the drawing draws it**, without the scheme. **`Telegram`
        // S6a: the name copied is that same answered username, bare** — no `@`
        // and no `t.me/`.
        event = value(event, "theLink", format!("t.me/{username}"));
        event = value(event, "theBotsName", username.clone());
    }
    event
}

/// **What a check that did not answer puts in front of the user**,
/// `Telegram` S3's two drawn readings kept apart and nothing else read as either.
pub fn the_stage_after_a_check_that_did_not_answer(found: &WhatTheCheckFound) -> Option<TheStage> {
    let (sentence, what_to_do, retry) = match found {
        WhatTheCheckFound::Answered(_) => return None,
        WhatTheCheckFound::NotAccepted => (words::NOT_ACCEPTED, Some(words::CHECK_THE_PASTE), None),
        WhatTheCheckFound::NoTokenInThePaste => (words::NO_TOKEN_IN_THE_PASTE, Some(words::CHECK_THE_PASTE), None),
        WhatTheCheckFound::DidNotFinish(WhyTheCheckDidNotFinish::TelegramCouldNotBeReached) => {
            (words::NOT_REACHED, Some(words::YOUR_PASTE_IS_KEPT), Some(CHECK_AGAIN))
        }
        WhatTheCheckFound::DidNotFinish(_) => {
            (words::THE_ANSWER_COULD_NOT_BE_USED, Some(words::YOUR_PASTE_IS_KEPT), Some(CHECK_AGAIN))
        }
    };
    Some(TheStage::AFailureAtThePaste { sentence, what_to_do, retry })
}

/// **fix-here item 13: a paste that cannot be checked is answered, never left spinning.**
/// An empty paste, or one longer than the limit, holds no token Smoozy will
/// look for, so it gets the *no token in that paste* stage.
pub fn the_stage_for_a_paste_that_cannot_be_checked(paste: &str) -> Option<TheStage> {
    if paste.is_empty() || paste.len() > THE_LONGEST_A_PASTE_MAY_BE {
        return the_stage_after_a_check_that_did_not_answer(&WhatTheCheckFound::NoTokenInThePaste);
    }
    None
}

/// Every failed step names its reason and the next action, without passing
/// the record engine's words or any secret to the page.
pub fn the_sentence_for_a_bot_not_kept(why: &WhyTheBotWasNotKept) -> &'static str {
    match why {
        WhyTheBotWasNotKept::TheTokenWasNotConfirmed => words::THE_TOKEN_WAS_NOT_CONFIRMED,
        WhyTheBotWasNotKept::TheRecordDidNotTakeTheProof(_) => words::THE_BOT_RECORD_DID_NOT_SAVE,
        WhyTheBotWasNotKept::TheTokenWasNotStored(smoozy_security_and_licence::secret_store::StoreFailure::LockedOrRefused) => words::THE_TOKEN_STORE_REFUSED,
        WhyTheBotWasNotKept::TheTokenWasNotStored(smoozy_security_and_licence::secret_store::StoreFailure::StoreUnavailable) => words::THE_TOKEN_WAS_NOT_CONFIRMED,
        WhyTheBotWasNotKept::TheRecordDidNotAnswerAboutAnOwedErase => words::THE_TOKEN_REMOVAL_RECORD_FAILED,
        WhyTheBotWasNotKept::TheRecordDidNotTakeTheStoredToken(_) => words::THE_SAVED_TOKEN_RECORD_FAILED,
    }
}

/// **fix-here item 3 for `Telegram` L4's replacement**, the same rule: a save that did not
/// answer in time is *not confirmed*, and only the rest are *not stored*.
pub fn the_sentence_for_a_replacement_not_accepted(why: &told_record::WhyTheReplacementWasNotAccepted) -> &'static str {
    match why {
        told_record::WhyTheReplacementWasNotAccepted::TheTokenWasNotConfirmed => words::THE_TOKEN_WAS_NOT_CONFIRMED,
        _ => words::THE_TOKEN_WAS_NOT_STORED,
    }
}

// ── What the walkthrough holds between presses ──────────────────────────────

/// **One walkthrough at a time, in this process.** The serial moves on every
/// opening and every cancel, so a check or a wait begun under an earlier one
/// posts nothing and stops at its next ask.
struct Held {
    offer: Option<TheOffer>,
    kept_again: Option<(TheKeepToRetry, String)>,
    /// **`set-bot-replace`: a checked token for the bot this computer already
    /// uses, waiting for *Save token*** (`Telegram` P4, L4). Nothing is stored
    /// until it is pressed; Cancel, Escape and a new opening drop it, and the
    /// token is overwritten as it goes.
    the_replacement: Option<TheCheckPassed>,
}

/// Retain the completed save when only the reporting switch failed. Retrying
/// that last step must not ask Telegram, write the token or move its generation.
enum TheKeepToRetry {
    Keep(TheHelloWasSaid),
    Enable(smoozy_telegram::walkthrough::TheBotIsSetUp),
}

fn the_bot_is_kept_in(
    connection: &rusqlite::Connection,
    store: Option<&dyn SecretStore>,
    retry: TheKeepToRetry,
) -> Result<smoozy_telegram::walkthrough::TheBotIsSetUp, (TheKeepToRetry, &'static str)> {
    let set_up = match retry {
        TheKeepToRetry::Enable(set_up) => set_up,
        TheKeepToRetry::Keep(hello) => {
            let Some(store) = store else {
                return Err((TheKeepToRetry::Keep(hello), words::THE_SECRET_STORE_UNAVAILABLE));
            };
            match hello.keep(&TheProvedBotRecord::over(connection), store, &mut told_record::TheBotTokensMarks(connection)) {
                Ok(set_up) => set_up,
                Err((hello, why)) => return Err((TheKeepToRetry::Keep(hello), the_sentence_for_a_bot_not_kept(&why))),
            }
        }
    };
    if the_store::telegram_was_switched(connection, true).is_err() {
        return Err((TheKeepToRetry::Enable(set_up), words::TELEGRAM_COULD_NOT_BE_ENABLED));
    }
    Ok(set_up)
}

static SERIAL: AtomicU64 = AtomicU64::new(0);

fn held() -> &'static Mutex<Held> {
    static HELD: OnceLock<Mutex<Held>> = OnceLock::new();
    HELD.get_or_init(|| Mutex::new(Held { offer: None, kept_again: None, the_replacement: None }))
}

fn the_username_on_offer() -> Option<String> {
    held().lock().ok()?.offer.as_ref().map(TheOffer::the_username)
}

fn posted<R: Runtime>(app: &AppHandle<R>, event: Event) {
    let Some(queue) = app.try_state::<Arc<TheOneQueue>>() else {
        return;
    };
    let _ = queue.inner().read_then_post::<(), std::convert::Infallible>(|| Ok(()), move |_| event.clone());
}

fn the_walkthrough_is_posted<R: Runtime>(app: &AppHandle<R>, serial: u64, stage: TheStage) {
    if SERIAL.load(Ordering::SeqCst) != serial {
        return;
    }
    posted(app, the_walkthrough_says(&stage, the_username_on_offer().as_deref()));
}

/// **The section drawn again from the record, with no answer** — for a change
/// made elsewhere: `Telegram` V1d's phone's choice taken as the default from a
/// server's reading.
pub(crate) fn the_section_is_told_again<R: Runtime>(app: &AppHandle<R>) {
    the_section_is_posted_from(app, None);
}

fn the_section_is_posted_from<R: Runtime>(app: &AppHandle<R>, the_answer: Option<&'static str>) {
    let (Some(started), Some(queue)) = (app.try_state::<Mutex<Started>>(), app.try_state::<Arc<TheOneQueue>>()) else {
        return;
    };
    let Ok(started) = started.lock() else {
        return;
    };
    the_section_is_posted(&started, &queue, the_answer);
}

/// **The secret store for this system** — the same three answers
/// `crate::surface::keys` constructs.
pub(crate) fn the_store_for_this_system() -> Option<Box<dyn SecretStore>> {
    #[cfg(target_os = "macos")]
    {
        Some(Box::new(crate::platform::secrets::MacKeychain))
    }
    #[cfg(target_os = "windows")]
    {
        Some(Box::new(crate::platform::secrets::WindowsCredentialManager))
    }
    #[cfg(target_os = "linux")]
    {
        crate::platform::secrets::LinuxSecretService::open().map(|store| Box::new(store) as Box<dyn SecretStore>)
    }
}

fn the_line_is_written(started: &Started, line: &smoozy_reporting::lines::Line) {
    let (Some(service), Some(run)) = (started.diary.as_ref(), started.run.as_ref()) else {
        return;
    };
    let diary = ModulesDiary { service, declarations: &started.declarations, run: run.clone(), module: THE_MODULE };
    let _ = diary.write(line);
}

// ── The acts ─────────────────────────────────────────────────────────────────

/// **Switching Telegram off: the switch, and nothing erased.** Until
/// 2026-10-31 it erased the token first (`Security and licence` E14 as it then
/// read), so turning Telegram on again asked for a revoked token, a paste and
/// every server reconnected. The Creator, the fifteenth notes, note 1: *"when I
/// turned it back on immediately, all the information about the old bot was
/// lost … Needs fixing."* **Off is a pause**: the same token stays on every
/// machine it was placed on (Specs `Telegram` :33, Architecture `Telegram` K1),
/// and replacing it is what revokes. What comes back is the answer to the
/// press, or nothing where it landed.
pub fn telegram_is_switched_off_in(connection: &rusqlite::Connection) -> Option<&'static str> {
    the_store::telegram_was_switched(connection, false)
        .err()
        .map(|_| words::THE_SWITCH_DID_NOT_MOVE)
}

/// **What pressing the switch on did.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatSwitchingOnDid {
    /// **The bot already set up is used again**: the switch was written (or,
    /// with the sentence, would not save). No walkthrough, nothing received.
    TheBotIsUsedAgain(Option<&'static str>),
    /// No bot is set up here, or its token is not stored for it: the
    /// walkthrough opens, as it always did.
    TheWalkthroughOpens,
}

/// **Switching Telegram on: the bot already set up, where there is one** —
/// the fifteenth notes, note 1, and Architecture `Telegram` K1 as changed
/// 2026-10-31. **Where the proof (`Telegram` S4) and the record's stored-token fact for
/// that bot both stand**, the switch is written and nothing else: the token is
/// the one every machine already holds, so nothing is received and `Telegram`
/// S5 is untouched. Where either is missing the walkthrough opens. **The store
/// is not asked**: the stored fact is the record's answer (`Security and
/// licence` §3.0 rule 6), and a token the store has lost is found and said at
/// the next composing, as it always is.
pub fn telegram_is_switched_on_in(connection: &rusqlite::Connection) -> WhatSwitchingOnDid {
    let keeper = TheProvedBotRecord::over(connection);
    let set_up = match (keeper.the_proved_bot(), keeper.the_bot_whose_token_is_stored()) {
        (Ok(Some((bot, _))), Ok(Some(stored_for))) => bot.id == stored_for,
        _ => false,
    };
    if !set_up {
        return WhatSwitchingOnDid::TheWalkthroughOpens;
    }
    WhatSwitchingOnDid::TheBotIsUsedAgain(
        the_store::telegram_was_switched(connection, true)
            .err()
            .map(|_| words::THE_SWITCH_DID_NOT_MOVE_ON),
    )
}

/// **The switch was pressed.** Off writes the switch; on uses the bot already
/// set up, or opens the walkthrough where there is none.
///
/// **Both run off the command's own thread**, the way the token check
/// already does: `Desktop app and delivery` P4 and P5 put every record write
/// off the thread that draws (fix-here item 15). Whether the shell runs a plain
/// command on that thread is not established, and this holds either way.
#[tauri::command]
pub fn telegram_is_switched<R: Runtime>(app: AppHandle<R>, on: bool) {
    std::thread::spawn(move || {
        let answer = {
            let Some(started) = app.try_state::<Mutex<Started>>() else {
                return;
            };
            let Ok(started) = started.lock() else {
                return;
            };
            let Some(record) = started.record.as_ref() else {
                return;
            };
            if on {
                match telegram_is_switched_on_in(record.connection()) {
                    WhatSwitchingOnDid::TheBotIsUsedAgain(answer) => answer,
                    WhatSwitchingOnDid::TheWalkthroughOpens => {
                        drop(started);
                        the_walkthrough_opens(&app);
                        return;
                    }
                }
            } else {
                telegram_is_switched_off_in(record.connection())
            }
        };
        the_section_is_posted_from(&app, answer);
        // **The live Telegram pass, fault 11: every server connected now is
        // told at once**, and only the rest wait for their next connection.
        if answer.is_none() {
            crate::setting_up::the_tellings_are_delivered_on_every_live_connection(&app);
        }
    });
}

/// ***Change bot or token…*** — the one walkthrough.
#[tauri::command]
pub fn the_telegram_walkthrough_is_opened<R: Runtime>(app: AppHandle<R>) {
    the_walkthrough_opens(&app);
}

fn the_walkthrough_opens<R: Runtime>(app: &AppHandle<R>) {
    let serial = SERIAL.fetch_add(1, Ordering::SeqCst) + 1;
    let offer = TheOffer::made().ok();
    if let Ok(mut held) = held().lock() {
        held.offer = offer;
        held.kept_again = None;
        held.the_replacement = None;
    }
    the_walkthrough_is_posted(app, serial, TheStage::ThePaste);
}

/// **`Telegram` S1b: *Suggest another*.**
#[tauri::command]
pub fn another_username_is_offered<R: Runtime>(app: AppHandle<R>) {
    if let Ok(mut held) = held().lock() {
        if let Some(offer) = held.offer.as_mut() {
            let _ = offer.another();
        }
    }
    the_walkthrough_is_posted(&app, SERIAL.load(Ordering::SeqCst), TheStage::ThePaste);
}

/// **Cancel, Escape and Done** — the walkthrough ends and whatever it began
/// stops at its next ask.
#[tauri::command]
pub fn the_telegram_walkthrough_is_shut<R: Runtime>(app: AppHandle<R>) {
    SERIAL.fetch_add(1, Ordering::SeqCst);
    if let Ok(mut held) = held().lock() {
        held.offer = None;
        held.kept_again = None;
        held.the_replacement = None;
    }
    posted(&app, the_walkthrough_says(&TheStage::Shut, None));
}

/// **`Telegram` S3: *Check bot*, on either route.** The paste goes to the check
/// and nowhere else; the check, the wait for Start, hello back and keeping run
/// off the interface thread and answer on the queue.
#[tauri::command]
pub fn the_pasted_token_is_checked<R: Runtime>(app: AppHandle<R>, paste: String) {
    let serial = SERIAL.load(Ordering::SeqCst);
    if let Some(stage) = the_stage_for_a_paste_that_cannot_be_checked(&paste) {
        the_walkthrough_is_posted(&app, serial, stage);
        return;
    }
    let app = app.clone();
    std::thread::spawn(move || the_check_runs(&app, serial, ThePaste::what_the_user_pasted(paste)));
}

fn the_proved_bot<R: Runtime>(app: &AppHandle<R>) -> Option<TheBot> {
    let started = app.try_state::<Mutex<Started>>()?;
    let started = started.lock().ok()?;
    let record = started.record.as_ref()?;
    TheProvedBotRecord::over(record.connection()).the_proved_bot().ok().flatten().map(|(bot, _)| bot)
}

/// **What a passed check leads to, and what is held for the next press.**
/// `Telegram` P4: the bot this computer already uses is held for *Save token*
/// and nothing is stored yet; any other bot goes to Start (`Telegram` S5), copying its
/// answered name once (`Telegram` S6a). Kept apart from the thread that runs
/// it so a test can drive it with a stand-in Telegram.
enum AfterTheCheck {
    TheBotAlreadyUsed(TheStage, TheCheckPassed),
    WaitForStart(TheStage, smoozy_telegram::walkthrough::TheWaitForStart),
}

fn after_the_check(check: TheCheckPassed, proved: Option<&TheBot>) -> AfterTheCheck {
    match check.what_comes_next(proved) {
        WhatComesNext::TheSameBotIsAlreadyProved { refreshed, check } => {
            AfterTheCheck::TheBotAlreadyUsed(TheStage::TheBotAlreadyUsed { username: refreshed.username }, check)
        }
        WhatComesNext::WaitForStart(wait) => {
            let username = wait.the_check().the_bot().username.clone();
            AfterTheCheck::WaitForStart(TheStage::WaitingForStart { username, not_reached: false, copy_the_name: true }, wait)
        }
    }
}

fn the_check_runs<R: Runtime>(app: &AppHandle<R>, serial: u64, paste: ThePaste) {
    let found = check_the_bot_answers(&paste);
    drop(paste);
    if let Some(stage) = the_stage_after_a_check_that_did_not_answer(&found) {
        the_walkthrough_is_posted(app, serial, stage);
        return;
    }
    let WhatTheCheckFound::Answered(check) = found else {
        return;
    };
    if SERIAL.load(Ordering::SeqCst) != serial {
        return;
    }
    match after_the_check(check, the_proved_bot(app).as_ref()) {
        // `Telegram` P4 and L4: the same bot, so a replacement, kept only when
        // *Save token* is pressed.
        AfterTheCheck::TheBotAlreadyUsed(stage, check) => {
            if let Ok(mut held) = held().lock() {
                held.the_replacement = Some(check);
            }
            the_walkthrough_is_posted(app, serial, stage);
        }
        // `Telegram` S5: a new bot, and the one poll for Start.
        AfterTheCheck::WaitForStart(stage, mut wait) => {
            let username = wait.the_check().the_bot().username.clone();
            the_walkthrough_is_posted(app, serial, stage);
            // **fix-here item 8: a failed ask or hello shows *Could not reach
            // Telegram*, and the next call that succeeds clears it.** The
            // retrying goes on underneath either way.
            let mut showing_not_reached = false;
            let mut proof = loop {
                if SERIAL.load(Ordering::SeqCst) != serial {
                    return;
                }
                wait = match wait.ask_once(ONE_ASK_WAITS_SECONDS) {
                    WhatOneAskFound::StartArrived(proof) => {
                        whether_telegram_was_reached_is_posted(app, serial, &username, &mut showing_not_reached, true);
                        break proof;
                    }
                    WhatOneAskFound::NothingYet(wait) => {
                        whether_telegram_was_reached_is_posted(app, serial, &username, &mut showing_not_reached, true);
                        wait
                    }
                    WhatOneAskFound::TheAskFailed(wait, _) => {
                        whether_telegram_was_reached_is_posted(app, serial, &username, &mut showing_not_reached, false);
                        std::thread::sleep(AFTER_A_FAILED_CALL);
                        wait
                    }
                };
            };
            // `Telegram` S6: hello back, said again until it goes or the user
            // leaves; nothing is kept until it has gone.
            let hello = loop {
                if SERIAL.load(Ordering::SeqCst) != serial {
                    return;
                }
                match proof.say_hello_back() {
                    Ok(hello) => {
                        whether_telegram_was_reached_is_posted(app, serial, &username, &mut showing_not_reached, true);
                        break hello;
                    }
                    Err((again, _)) => {
                        whether_telegram_was_reached_is_posted(app, serial, &username, &mut showing_not_reached, false);
                        proof = again;
                        std::thread::sleep(AFTER_A_FAILED_CALL);
                    }
                }
            };
            the_bot_is_kept(app, serial, TheKeepToRetry::Keep(hello), username);
        }
    }
}

/// **`set-bot-replace`'s *Save token*** — `Telegram` P4 and L4: the new token
/// for the bot this computer already uses is stored, every server is marked as
/// holding the old one, and no Start is asked for.
#[tauri::command]
pub fn the_replacement_token_is_saved<R: Runtime>(app: AppHandle<R>) {
    let Some(check) = held().lock().ok().and_then(|mut held| held.the_replacement.take()) else {
        return;
    };
    let serial = SERIAL.load(Ordering::SeqCst);
    let app = app.clone();
    std::thread::spawn(move || the_replacement_is_saved(&app, serial, check));
}

/// **The replacement stored and the switch on, or the reason it was not**, in
/// the store this system has.
fn the_replacement_is_saved_in(
    connection: &rusqlite::Connection,
    store: Option<&dyn SecretStore>,
    check: TheCheckPassed,
) -> Result<smoozy_telegram::told::AReplacementAccepted, &'static str> {
    let Some(store) = store else {
        return Err(words::THE_TOKEN_WAS_NOT_STORED);
    };
    let accepted = told_record::a_replacement_token_accepted(connection, store, check)
        .map_err(|why| the_sentence_for_a_replacement_not_accepted(&why))?;
    let _ = the_store::telegram_was_switched(connection, true);
    Ok(accepted)
}

fn the_replacement_is_saved<R: Runtime>(app: &AppHandle<R>, serial: u64, check: TheCheckPassed) {
    if SERIAL.load(Ordering::SeqCst) != serial {
        return;
    }
    let failed = {
        let Some(started) = app.try_state::<Mutex<Started>>() else {
            return;
        };
        let Ok(started) = started.lock() else {
            return;
        };
        let Some(record) = started.record.as_ref() else {
            return;
        };
        let store = the_store_for_this_system();
        match the_replacement_is_saved_in(record.connection(), store.as_deref(), check) {
            Ok(accepted) => {
                if let Some(run) = started.run.clone() {
                    the_line_is_written(&started, &told_record::a_replacement_token_accepted_line(run, SystemTime::now(), &accepted));
                }
                None
            }
            Err(sentence) => Some(sentence),
        }
    };
    if let Some(sentence) = failed {
        // **Back at the paste, on the route it was on, with the paste kept**:
        // the check was spent on the save, so *Try again* checks it again and
        // comes back to *Save token*.
        the_walkthrough_is_posted(
            app,
            serial,
            TheStage::AFailureAtThePaste {
                sentence,
                what_to_do: Some(words::YOUR_PASTE_IS_KEPT),
                retry: Some(CHECK_AGAIN),
            },
        );
        // **The section behind the box is drawn again on a failure
        // too.** Added 2026-09-19, from the Gemini catch-up's Area 3
        // Verdict, F-1. A replacement moves the generation and forgets
        // the stored-token fact before it stores anything
        // (`told_record::a_replacement_token_accepted`), so a failed
        // one leaves the record saying *no token stored* while the
        // section still showed the row of dots — the old round's H2
        // stops those dots only when the section is drawn again. The
        // application lock was let go at the end of the block above.
        the_section_is_posted_from(app, None);
    } else {
        the_walkthrough_is_posted(app, serial, TheStage::Shut);
        the_section_is_posted_from(app, None);
        // Fault 11: the new token reaches every server connected now.
        crate::setting_up::the_tellings_are_delivered_on_every_live_connection(app);
    }
}

/// **fix-here item 8: the waiting box is posted again only when whether Telegram was
/// reached changes**, so a string of empty asks posts nothing — **and none of
/// those drawings copies the name again** (`Telegram` S6a).
fn whether_telegram_was_reached_is_posted<R: Runtime>(
    app: &AppHandle<R>,
    serial: u64,
    username: &str,
    showing_not_reached: &mut bool,
    reached: bool,
) {
    if *showing_not_reached == reached {
        *showing_not_reached = !reached;
        the_walkthrough_is_posted(
            app,
            serial,
            TheStage::WaitingForStart { username: username.to_string(), not_reached: !reached, copy_the_name: false },
        );
    }
}

/// **`Telegram` S4, K1 and §19: the proof recorded, the token stored, the act
/// written, then the switch.**
fn the_bot_is_kept<R: Runtime>(app: &AppHandle<R>, serial: u64, retry: TheKeepToRetry, username: String) {
    if SERIAL.load(Ordering::SeqCst) != serial {
        return;
    }
    let outcome = {
        let Some(started) = app.try_state::<Mutex<Started>>() else {
            return;
        };
        let Ok(started) = started.lock() else {
            return;
        };
        let Some(record) = started.record.as_ref() else {
            return;
        };
        let store = the_store_for_this_system();
        match the_bot_is_kept_in(record.connection(), store.as_deref(), retry) {
            Ok(set_up) => {
                if let Some(run) = started.run.clone() {
                    the_line_is_written(&started, &a_bot_set_up(run, SystemTime::now(), &set_up));
                }
                Ok(())
            }
            Err(failure) => Err(failure),
        }
    };
    match outcome {
        Ok(()) => {
            the_walkthrough_is_posted(app, serial, TheStage::TheBotReplied { username });
            the_section_is_posted_from(app, None);
            // Fault 11: the bot, and the switch on, reach every server
            // connected now.
            crate::setting_up::the_tellings_are_delivered_on_every_live_connection(app);
        }
        Err((retry, sentence)) => {
            if let Ok(mut held) = held().lock() {
                held.kept_again = Some((retry, username.clone()));
            }
            the_walkthrough_is_posted(app, serial, TheStage::NotKept { username, sentence });
            // **The section behind the box is drawn again on a failure too.**
            // Added 2026-09-19, from the Gemini catch-up's Area 3 Verdict,
            // F-1. Keeping records the new bot's proof, moves the generation
            // and forgets the stored-token fact before the save that failed,
            // so without this the section went on showing the old bot, the row
            // of dots for a held token and no server as not reached — none of
            // which the record says. The application lock was let go at the
            // end of the block above.
            the_section_is_posted_from(app, None);
        }
    }
}

/// **Keeping again**, after it did not finish — the failure's own control.
#[tauri::command]
pub fn the_bot_is_kept_again<R: Runtime>(app: AppHandle<R>) {
    let Some((hello, username)) = held().lock().ok().and_then(|mut held| held.kept_again.take()) else {
        return;
    };
    let serial = SERIAL.load(Ordering::SeqCst);
    let app = app.clone();
    std::thread::spawn(move || the_bot_is_kept(&app, serial, hello, username));
}

/// **`Telegram` V1a: what a job reports by default**, written, read back and
/// drawn.
#[tauri::command]
pub fn the_reporting_default_is_set<R: Runtime>(app: AppHandle<R>, choice: TheReportingChoice) {
    let answer = {
        let Some(started) = app.try_state::<Mutex<Started>>() else {
            return;
        };
        let Ok(started) = started.lock() else {
            return;
        };
        let Some(record) = started.record.as_ref() else {
            return;
        };
        the_store::the_reporting_default_was_set(record.connection(), choice.stored_as())
            .err()
            .map(|_| words::THE_SETTING_DID_NOT_SAVE)
    };
    the_section_is_posted_from(&app, answer);
}

/// **What typing in the third row's field comes to** — the minutes taken,
/// with the row; or the refusal, the minutes that stood kept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheMinutesTypedCameTo {
    Taken,
    Refused,
    DidNotSave,
}

/// **The minutes typed for the default, taken or refused** — `Telegram` V1a as
/// amended (2026-10-31): the app-wide item holds *a picture every few minutes*
/// with its minutes. Whole minutes from 1 to 60, refused otherwise by `Render
/// Settings`' own reader, exactly as that page's field is. **Minutes taken
/// pick their row**, because the field is inside it: typing *10* there is
/// choosing a picture every 10 minutes.
pub fn the_default_minutes_typed_into(connection: &rusqlite::Connection, typed: &str) -> WhatTheMinutesTypedCameTo {
    let Ok(minutes) = smoozy_render_settings::reporting::the_minutes_typed(typed) else {
        return WhatTheMinutesTypedCameTo::Refused;
    };
    match the_store::the_default_minutes_were_set(
        connection,
        TheReportingChoice::APictureEveryFewMinutes.stored_as(),
        minutes,
    ) {
        Ok(()) => WhatTheMinutesTypedCameTo::Taken,
        Err(_) => WhatTheMinutesTypedCameTo::DidNotSave,
    }
}

/// **The third row's field, typed in** — [`the_default_minutes_typed_into`],
/// then the section drawn again with its answer.
#[tauri::command]
pub fn the_default_minutes_are_typed<R: Runtime>(app: AppHandle<R>, typed: String) {
    let came_to = {
        let Some(started) = app.try_state::<Mutex<Started>>() else {
            return;
        };
        let Ok(started) = started.lock() else {
            return;
        };
        let Some(record) = started.record.as_ref() else {
            return;
        };
        the_default_minutes_typed_into(record.connection(), &typed)
    };
    match came_to {
        WhatTheMinutesTypedCameTo::Taken => the_section_is_posted_from(&app, None),
        WhatTheMinutesTypedCameTo::DidNotSave => the_section_is_posted_from(&app, Some(words::THE_SETTING_DID_NOT_SAVE)),
        WhatTheMinutesTypedCameTo::Refused => the_section_is_posted_with_the_minutes_refused(&app),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::LocalRecord;
    use smoozy_security_and_licence::held::HeldSecret;
    use smoozy_security_and_licence::plaintext::Plaintext;
    use smoozy_security_and_licence::secret_store::{GetFailure, ProbeOutcome, StoreFailure};
    use smoozy_telegram::transport::a_stand_in::AStandInService;
    use smoozy_telegram::transport::TheClient;
    use std::fs;
    use std::path::Path;

    fn the_page() -> String {
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("web/telegram.js")).expect("the page module is readable")
    }

    fn a_record(name: &str) -> (std::path::PathBuf, LocalRecord) {
        let mut folder = std::env::temp_dir();
        folder.push(format!("smoozy-telegram-surface-test-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).unwrap();
        let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
        (folder, record)
    }

    fn said(event: &Event) -> String {
        format!("{event:?}")
    }

    /// *(Until 2026-10-31 `one_bot_or_token_control_and_the_three_choices_floor`,
    /// the floor line reading* All three send endings and questions.*)*
    #[test]
    fn one_bot_or_token_control_and_the_floor_under_all_but_off() {
        assert_eq!(words::USE_ANOTHER_BOT, "Change bot or token…");
        assert_eq!(words::ALWAYS_SENT, "All but Off send endings and questions.");
        let page = include_str!("../../web/telegram.js");
        let row = page.split("const tokenRow =").nth(1).unwrap().split("const whoever").next().unwrap();
        assert!(!row.contains("aControl("), "the token row offers a second button");
    }

    /// **The wait for Start carries the one turning mark every other screen
    /// draws**, not the drawing's `.ckdot`, whose animation, `wksp`, nothing
    /// defines, so the line stood still (the Open Thread *Nothing shows that
    /// work is happening…*). `Design/Whole app § Anything that isn't instant
    /// shows that it's underway`: the reading carries the mark itself. The page
    /// harness measured it turning. Step 10c's tenth-pass fix piece 6.
    #[test]
    fn the_wait_for_start_carries_the_shared_turning_mark() {
        let page = the_page();
        let working: String = page
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(!working.contains("ckdot"), "the page still draws the dot that never turns");
        let at = working.find("b.waiting.replaceChildren(").expect("the waiting line is drawn");
        let line = &working[at..at + working[at..].find(";\n").expect("it ends")];
        assert!(line.contains("made(\"i\", { className: \"spin\" })"), "{line}");
        assert!(working.contains("b.waiting.classList.add(\"underway-reading\")"));
    }

    /// **The fourteen are the drawing's own Telegram states in its state list.**
    #[test]
    fn the_fourteen_are_the_drawings_own() {
        let drawing = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Preparation/Design/Mockup/Smoozy.html")).unwrap();
        let list = drawing.split("<select id=\"state\"").nth(1).and_then(|rest| rest.split("</select>").next()).unwrap();
        for state in THE_STATES_THIS_PIECE_DRAWS {
            assert!(list.contains(&format!("<option value=\"{state}\">")), "{state} is not in the drawing's list");
        }
    }

    /// **`§ What this document left open`: before Telegram is on, the
    /// paragraph and the switch and nothing else.**
    #[test]
    fn before_telegram_is_on_the_section_is_the_switch_and_one_paragraph() {
        let event = the_section_says(&WhatTheSectionHolds {
            the_settings: Some(TheTelegramSettings::default()),
            the_bot: Some(TheBot { id: 7, username: "smoozy_a7k2_bot".into() }),
            the_token_is_held: true,
            not_reached: Some(Vec::new()),
            the_answer: None,
            the_minutes_refused: None,
        });
        let text = said(&event);
        assert!(text.contains("beforeItIsOn") && text.contains("theSwitch"));
        for absent in ["theBotsUsername", "theTokenIsHeld", "whatAJobReports", "choiceName", "notReached"] {
            assert!(!text.contains(absent), "{absent} was drawn before Telegram is on");
        }
    }

    /// **Once on: the username and the mask by the verbatim route, the default,
    /// and each server not reached as a value** (X7).
    #[test]
    fn once_on_every_name_is_a_value_and_the_token_is_never_shown() {
        let event = the_section_says(&WhatTheSectionHolds {
            the_settings: Some(TheTelegramSettings {
                switched_on: true,
                the_reporting_default: Some("pictures-only".into()),
                ..TheTelegramSettings::default()
            }),
            the_bot: Some(TheBot { id: 7, username: "smoozy_a7k2_bot".into() }),
            the_token_is_held: true,
            not_reached: Some(vec!["Hetzner box".into(), "Old 4080".into()]),
            the_answer: None,
            the_minutes_refused: None,
        });
        let text = said(&event);
        assert!(text.contains(&format!("{:?}", Verbatim::new("@smoozy_a7k2_bot"))));
        assert!(text.contains(&format!("{:?}", Verbatim::new("Hetzner box"))));
        assert!(!text.contains(&format!("{:?}", Prose::new("Hetzner box"))));
        assert!(text.contains(&format!("{:?}", Verbatim::new("pictures-only"))));
        assert!(text.contains(&format!("{:?}", Prose::new(words::SERVERS_NOT_REACHED))));
        assert!(!text.contains("beforeItIsOn"));
    }

    /// **`Design/Settings § Telegram`, the drawing's `set-tgdefault`: five
    /// radios in the Render Settings list's order, the third's minutes field,
    /// and the two lines' words**; a stored default of any of the five is the
    /// one picked, the minutes the record's or 5.
    #[test]
    fn the_section_draws_five_radios_and_the_thirds_minutes() {
        let on = |default: Option<&str>, minutes: Option<u8>| WhatTheSectionHolds {
            the_settings: Some(TheTelegramSettings {
                switched_on: true,
                the_reporting_default: default.map(str::to_string),
                the_default_minutes: minutes,
                the_phones_choice_taken: None,
            }),
            the_bot: Some(TheBot { id: 7, username: "smoozy_a7k2_bot".into() }),
            the_token_is_held: true,
            not_reached: Some(Vec::new()),
            the_answer: None,
            the_minutes_refused: None,
        };
        let text = said(&the_section_says(&on(None, None)));
        for (name, what) in [
            ("Everything", "progress, frames, a short video and a contact sheet"),
            ("Pictures only", "the same, without progress"),
            ("A picture every", "the same as Everything, with fewer pictures"),
            ("Alerts only", "endings and questions"),
            ("Off", "nothing, unless you ask"),
        ] {
            assert!(text.contains(&format!("{:?}", Prose::new(name))), "{name}");
            assert!(text.contains(&format!("{:?}", Prose::new(what))), "{what}");
        }
        assert!(text.contains(&format!("{:?}", Prose::new("All but Off send endings and questions."))));
        assert!(text.contains(&format!(
            "{:?}",
            Prose::new("Change it per job when you submit, or for all jobs with /reporting in Telegram.")
        )));
        assert!(text.contains(&format!("{:?}", Verbatim::new("5"))), "5 to begin with");
        assert!(text.contains(&format!("{:?}", Verbatim::new("everything"))));
        assert!(!text.contains("theMinutesAreRefused"));

        let text = said(&the_section_says(&on(Some("a-picture-every-few-minutes"), Some(10))));
        assert!(text.contains(&format!("{:?}", Verbatim::new("10"))));
        let event = the_section_says(&on(Some("nothing"), Some(10)));
        let text = said(&event);
        assert!(text.contains("theDefaultIs") && text.contains(&format!("{:?}", Verbatim::new("nothing"))));

        let mut refused = on(None, Some(12));
        refused.the_minutes_refused = Some(words::WHOLE_MINUTES_1_TO_60);
        let text = said(&the_section_says(&refused));
        assert!(text.contains("theMinutesAreRefused"));
        assert!(text.contains(&format!("{:?}", Verbatim::new("12"))), "the minutes that stood");
    }

    /// **Settings round-trips all five**, the third with its typed minutes;
    /// minutes Smoozy will not take are refused and the ones that stood stay.
    #[test]
    fn settings_round_trips_all_five_and_the_minutes_typed() {
        let (folder, record) = a_record("five");
        let c = record.connection();
        for choice in TheReportingChoice::ALL {
            the_store::the_reporting_default_was_set(c, choice.stored_as()).unwrap();
            let held = what_the_section_holds(c, None);
            assert_eq!(
                held.the_settings.unwrap().the_reporting_default.as_deref(),
                Some(choice.stored_as())
            );
        }
        assert_eq!(the_default_minutes_typed_into(c, " 15 "), WhatTheMinutesTypedCameTo::Taken);
        let settings = the_store::the_telegram_settings(c).unwrap();
        assert_eq!(settings.the_reporting_default.as_deref(), Some("a-picture-every-few-minutes"), "the minutes pick their row");
        assert_eq!(settings.the_default_minutes, Some(15));
        for typed in ["0", "61", "", "2.5", "five"] {
            assert_eq!(the_default_minutes_typed_into(c, typed), WhatTheMinutesTypedCameTo::Refused, "{typed:?}");
        }
        assert_eq!(the_store::the_telegram_settings(c).unwrap().the_default_minutes, Some(15));
        // Another row picked and the third again: the minutes are still there.
        the_store::the_reporting_default_was_set(c, "pictures-only").unwrap();
        the_store::the_reporting_default_was_set(c, "a-picture-every-few-minutes").unwrap();
        assert_eq!(the_store::the_telegram_settings(c).unwrap().the_default_minutes, Some(15));
        drop(record);
        let _ = fs::remove_dir_all(&folder);
    }

    /// **L8a: a record that would not read draws no switch, on or off.**
    #[test]
    fn a_section_that_would_not_read_draws_no_switch() {
        let text = said(&the_section_says(&WhatTheSectionHolds {
            the_settings: None,
            the_bot: None,
            the_token_is_held: false,
            not_reached: Some(Vec::new()),
            the_answer: None,
            the_minutes_refused: None,
        }));
        assert!(text.contains("theSettingsWouldNotRead"));
        assert!(!text.contains("switchedOn"));
    }

    /// **The five are the submit strip's own identifiers, in its order**, one
    /// closed set (`Design/Settings`, *From 2026-10-31 the radios are five*).
    /// *(Equal until 2026-10-19; Settings' three were the list less paced
    /// pictures and Off from then until 2026-10-31.)*
    #[test]
    fn the_five_choices_are_the_render_settings_own() {
        let ours: Vec<&str> = TheReportingChoice::ALL.iter().map(|c| c.stored_as()).collect();
        let theirs: Vec<&str> = crate::surface::render_settings::REPORTING_CHOICES
            .iter()
            .map(|(id, _)| *id)
            .collect();
        assert_eq!(ours, theirs);
        for id in ours {
            let parsed: Result<TheReportingChoice, serde::de::value::Error> = serde::Deserialize::deserialize(
                serde::de::value::StrDeserializer::<serde::de::value::Error>::new(id),
            );
            assert_eq!(parsed.map(TheReportingChoice::stored_as), Ok(id));
        }
    }

    fn a_check(service: &Arc<AStandInService>) -> WhatTheCheckFound {
        let reach = |_: &str| TheClient::over(service.clone());
        smoozy_telegram::walkthrough::check_the_bot_answers_through(
            &ThePaste::what_the_user_pasted("Use this token: 123456789:AAH-not_a_real_token_0123456789abcd"),
            &reach,
        )
    }

    /// **`Telegram` S3's two readings stay apart**, and a check that ended any
    /// other way says neither of them.
    #[test]
    fn the_two_drawn_readings_are_kept_apart_and_nothing_else_is_read_as_either() {
        let service = Arc::new(AStandInService::new());
        service
            .then_answers(r#"{"ok":false,"error_code":401,"description":"Unauthorized"}"#)
            .then_is_not_reached("connection refused")
            .then_answers("<html>502</html>");
        let refused = the_stage_after_a_check_that_did_not_answer(&a_check(&service)).unwrap();
        let unreached = the_stage_after_a_check_that_did_not_answer(&a_check(&service)).unwrap();
        let unreadable = the_stage_after_a_check_that_did_not_answer(&a_check(&service)).unwrap();
        assert_eq!(refused, TheStage::AFailureAtThePaste { sentence: words::NOT_ACCEPTED, what_to_do: Some(words::CHECK_THE_PASTE), retry: None });
        assert_eq!(unreached, TheStage::AFailureAtThePaste { sentence: words::NOT_REACHED, what_to_do: Some(words::YOUR_PASTE_IS_KEPT), retry: Some(CHECK_AGAIN) });
        let TheStage::AFailureAtThePaste { sentence, .. } = unreadable else { panic!() };
        assert!(sentence != words::NOT_ACCEPTED && sentence != words::NOT_REACHED);
        let no_token = the_stage_after_a_check_that_did_not_answer(&WhatTheCheckFound::NoTokenInThePaste).unwrap();
        let TheStage::AFailureAtThePaste { sentence, .. } = no_token else { panic!() };
        assert_ne!(sentence, words::NOT_ACCEPTED, "Telegram was never asked, so it did not refuse");
    }

    /// **`Telegram` S2: nothing the walkthrough posts carries the paste**, and
    /// the link is drawn without its scheme.
    #[test]
    fn the_walkthrough_never_carries_the_paste() {
        let event = the_walkthrough_says(
            &TheStage::WaitingForStart { username: "smoozy_a7k2_bot".into(), not_reached: false, copy_the_name: true },
            Some("smoozy_zz99_bot"),
        );
        let text = said(&event);
        assert!(text.contains(&format!("{:?}", Verbatim::new("t.me/smoozy_a7k2_bot"))));
        assert!(!text.contains("https://"));
        assert!(text.contains(&format!("{:?}", Verbatim::new("smoozy_zz99_bot"))));
        assert!(!text.contains("123456789:"));
        let source = include_str!("telegram.rs").split("#[cfg(test)]").next().unwrap();
        assert!(!source.contains("value(event, \"paste"), "a paste is put on an event");
    }

    #[derive(Default)]
    struct AStore {
        held: std::cell::RefCell<std::collections::HashMap<HeldSecret, Vec<u8>>>,
    }

    impl SecretStore for AStore {
        fn put(&self, item: &HeldSecret, value: Plaintext, _: Duration) -> Result<(), StoreFailure> {
            self.held.borrow_mut().insert(item.clone(), value.expose(|b| b.to_vec()));
            Ok(())
        }
        fn get(&self, item: &HeldSecret, _: Duration) -> Result<Plaintext, GetFailure> {
            self.held.borrow().get(item).cloned().map(Plaintext::new).ok_or(GetFailure::NotThere)
        }
        fn erase(&self, item: &HeldSecret, _: Duration) -> Result<(), StoreFailure> {
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

    /// **Off: the switch, and the token kept** (the fifteenth notes, note 1;
    /// until 2026-10-31 the token was erased first); and with Telegram off, a
    /// server told it was on is named and a server never told anything is not.
    #[test]
    fn switching_off_keeps_the_token_and_names_only_servers_that_had_telegram() {
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        use smoozy_servers::store::ServerRow;
        use smoozy_state::identities::{Identity, IdentityKind};
        let (folder, record) = a_record("off");
        let c = record.connection();
        let mut rows = Vec::new();
        for label in ["Told", "Never told"] {
            let mut row = ServerRow::saved(
                Identity::issue(IdentityKind::LocalServer).unwrap().as_text(),
                Host::what_the_user_typed("192.0.2.30").unwrap(),
                Port::what_the_user_typed(22).unwrap(),
                Username::what_the_user_typed("root").unwrap(),
                KeyReference { local_key_identifier: "key-1".into(), fingerprint: "SHA256:aaa".into() },
            );
            row.label = Some(label.into());
            row.last_confirmed_installation_identity = Some("i1".into());
            crate::record::server_rows::add(c, &row).unwrap();
            crate::record::server_rows::write(c, &row).unwrap();
            rows.push(row);
        }
        told_record::the_telling_was_acknowledged(c, &rows[0].local_server_identifier, &WhatSmoozyTells::the_roles_desired_state(true), SystemTime::UNIX_EPOCH, "i1").unwrap();
        the_store::telegram_was_switched(c, true).unwrap();
        let store = AStore::default();
        store.put(&smoozy_telegram::walkthrough::the_bot_tokens_item(), Plaintext::new(b"7:t".to_vec()), Duration::from_secs(1)).unwrap();

        assert_eq!(telegram_is_switched_off_in(c), None);
        assert!(!store.held.borrow().is_empty(), "the token was erased");
        let holds = what_the_section_holds(c, None);
        assert!(!holds.the_settings.unwrap().switched_on);
        assert_eq!(holds.not_reached, Some(vec!["Told".to_string()]));
        drop(record);
        let _ = fs::remove_dir_all(folder);
    }

    fn a_server_in(c: &rusqlite::Connection, label: Option<&str>, host: &str) -> smoozy_servers::store::ServerRow {
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        use smoozy_servers::store::ServerRow;
        use smoozy_state::identities::{Identity, IdentityKind};
        let mut row = ServerRow::saved(
            Identity::issue(IdentityKind::LocalServer).unwrap().as_text(),
            Host::what_the_user_typed(host).unwrap(),
            Port::what_the_user_typed(22).unwrap(),
            Username::what_the_user_typed("root").unwrap(),
            KeyReference { local_key_identifier: "key-1".into(), fingerprint: "SHA256:aaa".into() },
        );
        row.label = label.map(str::to_string);
        row.last_confirmed_installation_identity = Some("i1".into());
        crate::record::server_rows::add(c, &row).unwrap();
        crate::record::server_rows::write(c, &row).unwrap();
        row
    }

    /// **fix-here item 10 and fix-here item 12: with Telegram on, a server never told anything is not
    /// named, and an unlabelled server is named the way the servers screen
    /// names it, never by its identifier.**
    #[test]
    fn with_telegram_on_only_servers_given_telegram_are_named_and_by_their_screen_name() {
        let (folder, record) = a_record("on-names");
        let c = record.connection();
        let told = a_server_in(c, None, "192.0.2.31");
        let _never_told = a_server_in(c, Some("Never told"), "192.0.2.32");
        told_record::the_telling_was_acknowledged(c, &told.local_server_identifier, &WhatSmoozyTells::the_roles_desired_state(true), SystemTime::UNIX_EPOCH, "i1").unwrap();
        the_store::telegram_was_switched(c, true).unwrap();
        let holds = what_the_section_holds(c, None);
        assert_eq!(holds.not_reached, Some(vec!["root@192.0.2.31".to_string()]));
        assert!(!format!("{:?}", holds.not_reached).contains(&told.local_server_identifier));
        drop(record);
        let _ = fs::remove_dir_all(folder);
    }

    /// **fix-here item 11: a told-record that does not read says so**, rather than drawing
    /// every server as reached.
    #[test]
    fn a_told_record_that_does_not_read_is_said_not_drawn_as_nobody() {
        let (folder, record) = a_record("unread-told");
        let c = record.connection();
        the_store::telegram_was_switched(c, true).unwrap();
        // With no server there is nothing to read, and nobody is honestly
        // the answer; a server makes the told-record's read happen.
        let _ = a_server_in(c, Some("One"), "192.0.2.33");
        c.execute_batch("DROP TABLE told_entry").unwrap();
        let holds = what_the_section_holds(c, None);
        assert_eq!(holds.not_reached, None);
        let text = said(&the_section_says(&holds));
        assert!(text.contains("notReachedWouldNotRead"));
        assert!(!text.contains("notReachedCount"));
        drop(record);
        let _ = fs::remove_dir_all(folder);
    }

    /// **fix-here item 2: the dots are drawn only where the record says a token is stored
    /// for the bot on record**; otherwise the plain line and the way to finish.
    #[test]
    fn the_token_is_drawn_as_held_only_from_the_stored_fact_for_this_bot() {
        let (folder, record) = a_record("held");
        let c = record.connection();
        the_store::telegram_was_switched(c, true).unwrap();
        let keeper = TheProvedBotRecord::over(c);
        let a = TheBot { id: 7, username: "smoozy_a7k2_bot".into() };
        keeper.the_proof_was_recorded(&a, TheConversation(1)).unwrap();
        let text = said(&the_section_says(&what_the_section_holds(c, None)));
        assert!(text.contains("theTokenIsNotStored") && !text.contains("theTokenIsHeld"), "no save, no dots");

        keeper.the_token_was_stored_for(&a).unwrap();
        let text = said(&the_section_says(&what_the_section_holds(c, None)));
        assert!(text.contains("theTokenIsHeld") && !text.contains("theTokenIsNotStored"));

        let b = TheBot { id: 8, username: "smoozy_b8b8_bot".into() };
        keeper.the_proof_was_recorded(&b, TheConversation(1)).unwrap();
        let text = said(&the_section_says(&what_the_section_holds(c, None)));
        assert!(text.contains("theTokenIsNotStored"), "a new bot whose save has not succeeded is not drawn as held");
        drop(record);
        let _ = fs::remove_dir_all(folder);
    }

    /// **A switch that would not save says so, either way, and moves nothing**
    /// — the record refusing the write. (Until 2026-10-31 this was fix-here
    /// item 6's owed erase on switching off, which switching off no longer
    /// makes.)
    #[test]
    fn a_switch_that_would_not_save_says_so_either_way() {
        let (folder, record) = a_record("switch-refused");
        let c = record.connection();
        let store = AStore::default();
        assert!(the_bot_is_kept_in(c, Some(&store), TheKeepToRetry::Keep(a_new_bot_says_hello(7))).is_ok());
        c.execute_batch("CREATE TRIGGER refuse_the_switch BEFORE UPDATE ON telegram_setting BEGIN SELECT RAISE(ABORT, 'test refusal'); END;").unwrap();
        assert_eq!(telegram_is_switched_off_in(c), Some(words::THE_SWITCH_DID_NOT_MOVE));
        assert!(the_store::the_telegram_settings(c).unwrap().switched_on);
        c.execute_batch("DROP TRIGGER refuse_the_switch").unwrap();
        assert_eq!(telegram_is_switched_off_in(c), None);
        c.execute_batch("CREATE TRIGGER refuse_the_switch BEFORE UPDATE ON telegram_setting BEGIN SELECT RAISE(ABORT, 'test refusal'); END;").unwrap();
        assert_eq!(telegram_is_switched_on_in(c), WhatSwitchingOnDid::TheBotIsUsedAgain(Some(words::THE_SWITCH_DID_NOT_MOVE_ON)));
        assert!(!the_store::the_telegram_settings(c).unwrap().switched_on);
        drop(record);
        let _ = fs::remove_dir_all(folder);
    }

    /// The text of one function in this file, from its signature to the next
    /// item, comments left out.
    fn the_body_of(signature: &str) -> String {
        let text = include_str!("telegram.rs");
        let at = text.find(signature).unwrap_or_else(|| panic!("{signature} is in this file"));
        let rest = &text[at..];
        let end = rest.find("\n}\n").expect("it ends");
        rest[..end]
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// **The live Telegram pass, fault 11: every act that changes what servers
    /// are told tells every server connected now, at once** (Specs `Telegram`
    /// :37 as changed 2026-10-31). The switch both ways, a bot kept and a
    /// replacement saved each start `crate::setting_up`'s telling over every
    /// live connection, **after** the change is written and the lock let go.
    /// Asserted over the acts' own text because each runs inside the shell;
    /// which servers that reaches is `crate::setting_up`'s own test.
    #[test]
    fn fault_11_every_act_that_changes_what_servers_are_told_tells_the_connected_ones_now() {
        let telling = "crate::setting_up::the_tellings_are_delivered_on_every_live_connection(";
        for signature in [
            "pub fn telegram_is_switched<R: Runtime>(",
            "fn the_bot_is_kept<R: Runtime>(",
            "fn the_replacement_is_saved<R: Runtime>(",
        ] {
            let body = the_body_of(signature);
            assert!(body.contains(telling), "{signature} tells no connected server");
        }
    }

    /// **fix-here item 5 and fix-here item 14: keeping that did not finish is its own stage, with the
    /// reason said and keeping again offered; fix-here item 3 for the replacement.**
    #[test]
    fn a_bot_not_kept_is_its_own_stage_and_says_why() {
        let username = "smoozy_a7k2_bot".to_string();
        let sentence = the_sentence_for_a_bot_not_kept(&WhyTheBotWasNotKept::TheTokenWasNotConfirmed);
        assert_eq!(sentence, words::THE_TOKEN_WAS_NOT_CONFIRMED);
        assert_eq!(
            the_sentence_for_a_bot_not_kept(&WhyTheBotWasNotKept::TheTokenWasNotStored(StoreFailure::LockedOrRefused)),
            words::THE_TOKEN_STORE_REFUSED
        );
        let text = said(&the_walkthrough_says(&TheStage::NotKept { username, sentence }, None));
        assert!(text.contains(&format!("{:?}", Verbatim::new("notkept"))));
        assert!(!text.contains(&format!("{:?}", Verbatim::new("waiting"))));
        assert!(text.contains(&format!("{:?}", Prose::new(words::THE_TOKEN_WAS_NOT_CONFIRMED))));
        assert!(text.contains(&format!("{:?}", Verbatim::new(KEEP_AGAIN))));
        assert_eq!(
            the_sentence_for_a_replacement_not_accepted(&told_record::WhyTheReplacementWasNotAccepted::TheTokenWasNotConfirmed),
            words::THE_TOKEN_WAS_NOT_CONFIRMED
        );
        assert_eq!(
            the_sentence_for_a_replacement_not_accepted(&told_record::WhyTheReplacementWasNotAccepted::TheTokenWasNotStored(StoreFailure::StoreUnavailable)),
            words::THE_TOKEN_WAS_NOT_STORED
        );
    }

    #[test]
    fn every_failure_to_keep_a_bot_names_the_step_and_what_to_do() {
        use smoozy_telegram::proof::ARecordFailure;
        let failures = [
            WhyTheBotWasNotKept::TheRecordDidNotTakeTheProof(ARecordFailure("private engine detail".into())),
            WhyTheBotWasNotKept::TheTokenWasNotStored(StoreFailure::LockedOrRefused),
            WhyTheBotWasNotKept::TheTokenWasNotStored(StoreFailure::StoreUnavailable),
            WhyTheBotWasNotKept::TheTokenWasNotConfirmed,
            WhyTheBotWasNotKept::TheRecordDidNotAnswerAboutAnOwedErase,
            WhyTheBotWasNotKept::TheRecordDidNotTakeTheStoredToken(ARecordFailure("private engine detail".into())),
        ];
        for why in failures {
            let sentence = the_sentence_for_a_bot_not_kept(&why);
            assert!(sentence.to_lowercase().contains("try again"), "no next step: {sentence}");
            assert!(!sentence.contains("private engine detail"));
        }
    }

    fn a_new_bot_says_hello(id: i64) -> TheHelloWasSaid {
        use smoozy_telegram::walkthrough::{check_the_bot_answers_through, ThePaste};
        let service = Arc::new(AStandInService::new());
        service.then_answers(&serde_json::json!({"ok":true,"result":{
            "id":id,"is_bot":true,"username":"smoozy_a7k2_bot"
        }}).to_string());
        let reach = |_: &str| TheClient::over(service.clone());
        let WhatTheCheckFound::Answered(check) = check_the_bot_answers_through(
            &ThePaste::what_the_user_pasted(format!("{id}:AAH-not_a_real_token_0123456789abcd")), &reach,
        ) else { panic!("the stand-in bot answered") };
        let WhatComesNext::WaitForStart(wait) = check.what_comes_next(None) else { panic!("a new bot waits") };
        service.then_answers(r#"{"ok":true,"result":[]}"#);
        let WhatOneAskFound::NothingYet(wait) = wait.ask_once(0) else { panic!("still waiting for Start") };
        service.then_answers(r#"{"ok":true,"result":[{"update_id":1,"message":{"chat":{"id":9001},"text":"/start"}}]}"#);
        let WhatOneAskFound::StartArrived(proof) = wait.ask_once(0) else { panic!("Start arrived") };
        service.then_answers(r#"{"ok":true,"result":{"message_id":2}}"#);
        proof.say_hello_back().unwrap()
    }

    #[derive(Default)]
    struct AFailingStore {
        inner: AStore,
        erase_failure: std::cell::Cell<Option<StoreFailure>>,
        put_failure: std::cell::Cell<Option<StoreFailure>>,
        erased: std::cell::RefCell<Vec<HeldSecret>>,
    }

    impl SecretStore for AFailingStore {
        fn put(&self, item: &HeldSecret, value: Plaintext, deadline: Duration) -> Result<(), StoreFailure> {
            if let Some(why) = self.put_failure.get() { return Err(why) }
            self.inner.put(item, value, deadline)
        }
        fn get(&self, item: &HeldSecret, deadline: Duration) -> Result<Plaintext, GetFailure> { self.inner.get(item, deadline) }
        fn erase(&self, item: &HeldSecret, deadline: Duration) -> Result<(), StoreFailure> {
            self.erased.borrow_mut().push(item.clone());
            if let Some(why) = self.erase_failure.get() { return Err(why) }
            self.inner.erase(item, deadline)
        }
        fn probe(&self, _: Duration) -> ProbeOutcome { ProbeOutcome::RoundTripWorked }
        fn enumerate(&self, _: Duration) -> Result<Vec<HeldSecret>, StoreFailure> { Ok(Vec::new()) }
    }

    #[test]
    fn fifteenth_note_2_off_on_and_a_new_bot_finish_after_an_owed_erase() {
        use smoozy_security_and_licence::erasure::EraseMarks;
        use smoozy_telegram::told::TokenGeneration;
        for failure in [None, Some(StoreFailure::LockedOrRefused), Some(StoreFailure::StoreUnavailable)] {
            let (folder, record) = a_record(&format!("note2-{failure:?}"));
            let c = record.connection();
            let store = AFailingStore::default();
            assert!(the_bot_is_kept_in(c, Some(&store), TheKeepToRetry::Keep(a_new_bot_says_hello(7))).is_ok());
            c.execute("INSERT INTO telegram_token_generation (the_one_row, generation) VALUES (1, 4)", []).unwrap();
            store.erase_failure.set(failure);
            // Off erases nothing since 2026-10-31 (the fifteenth notes, note
            // 1); the old key's erase is now owed by keeping a different bot.
            assert_eq!(telegram_is_switched_off_in(c), None);
            assert!(!the_store::the_telegram_settings(c).unwrap().switched_on);
            assert_eq!(TheProvedBotRecord::over(c).the_bot_whose_token_is_stored().unwrap(), Some(7));

            let new_bot = a_new_bot_says_hello(1_234_567_890);
            let result = the_bot_is_kept_in(c, Some(&store), TheKeepToRetry::Keep(new_bot));
            assert!(result.is_ok(), "an old key that cannot be removed must not block the new bot");
            let section = what_the_section_holds(c, None);
            assert!(section.the_settings.unwrap().switched_on && section.the_token_is_held);
            assert_eq!(section.the_bot.unwrap().id, 1_234_567_890);
            assert_eq!(told_record::TheBotTokensMarks(c).marks().unwrap().is_empty(), failure.is_none());
            assert_eq!(told_record::the_current_generation(c).unwrap(), TokenGeneration(5));

            let new_item = crate::record::telegram_bot::the_current_bot_tokens_item(c).unwrap();
            assert!(!told_record::TheBotTokensMarks(c).marks().unwrap().contains(&new_item));
            let old_attempts = store.erased.borrow().len();
            // A later startup reads the same disk record. Even a permanently
            // unconfirmed erase retries without touching the new saved fact.
            drop(record);
            let record = LocalRecord::open_at(&folder.join("local-record.sqlite3")).unwrap();
            let c = record.connection();
            crate::start::retry_the_owed_bot_keys_at_start_with(c, |c| told_record::retry_the_owed_bot_keys(c, &store));
            assert_eq!(store.erased.borrow().len(), old_attempts + usize::from(failure.is_some()));
            assert!(what_the_section_holds(c, None).the_token_is_held);
            assert!(the_store::the_telegram_settings(c).unwrap().switched_on);
            if failure.is_some() {
                assert_eq!(told_record::TheBotTokensMarks(c).marks().unwrap().len(), 1);
                // A late old erase lands after the new save. Its subject is
                // different, so it cannot reach the new item.
                let old = told_record::TheBotTokensMarks(c).marks().unwrap().remove(0);
                store.inner.erase(&old, Duration::from_secs(1)).unwrap();
                store.erase_failure.set(None);
                crate::start::retry_the_owed_bot_keys_at_start_with(c, |c| told_record::retry_the_owed_bot_keys(c, &store));
                assert!(told_record::TheBotTokensMarks(c).marks().unwrap().is_empty());
                assert!(what_the_section_holds(c, None).the_token_is_held);
            }
            assert!(!store.erased.borrow().contains(&new_item), "no cleanup may ever erase the new key");

            // The ordinary server composer can use the new proof and token.
            let mut server = a_server_in(c, Some("One"), "192.0.2.40");
            server.recorded_machine_root = Some(smoozy_servers::store::RecordedMachineRoot {
                path: "/mnt/one/smoozy".into(), the_package_areas_own_location: None, read_under: "i1".into(),
            });
            let material = told_record::the_material_for(c, &store, &server, false).unwrap();
            assert!(material.the_files()[0].1 == b"1234567890:AAH-not_a_real_token_0123456789abcd",
                "server material must carry the new bot's synthetic token");
            drop(record);
            let _ = fs::remove_dir_all(folder);
        }
    }

    /// **The fifteenth notes, note 1: turning Telegram off keeps the bot, and
    /// turning it on again uses it with no walkthrough** (Specs `Telegram` :33
    /// and :37, Architecture `Telegram` K1 and `Security and licence` E14 as
    /// changed 2026-10-31). The Creator: *"when I turned it back on
    /// immediately, all the information about the old bot was lost … Needs
    /// fixing."* Off erased the token, so on needed a revoked token, a paste
    /// and every server reconnected.
    #[test]
    fn fifteenth_note_1_off_keeps_the_bot_and_on_uses_it_with_no_walkthrough() {
        use smoozy_security_and_licence::erasure::EraseMarks;
        let (folder, record) = a_record("note1-off-on");
        let c = record.connection();
        let store = AFailingStore::default();
        assert!(the_bot_is_kept_in(c, Some(&store), TheKeepToRetry::Keep(a_new_bot_says_hello(7))).is_ok());
        let item = crate::record::telegram_bot::the_current_bot_tokens_item(c).unwrap();
        let generation = told_record::the_current_generation(c).unwrap();

        assert_eq!(telegram_is_switched_off_in(c), None);
        assert!(!the_store::the_telegram_settings(c).unwrap().switched_on);
        assert!(store.erased.borrow().is_empty(), "switching off erased the token");
        assert!(store.inner.get(&item, Duration::from_secs(1)).is_ok(), "the token is still in the store");
        assert!(what_the_section_holds(c, None).the_token_is_held, "the record still says it is stored");
        assert!(told_record::TheBotTokensMarks(c).marks().unwrap().is_empty(), "no erase is owed");

        // On again: the same bot, no walkthrough, nothing received, and the
        // token and generation the machines hold unchanged.
        assert_eq!(telegram_is_switched_on_in(c), WhatSwitchingOnDid::TheBotIsUsedAgain(None));
        let section = what_the_section_holds(c, None);
        assert!(section.the_settings.unwrap().switched_on && section.the_token_is_held);
        assert_eq!(section.the_bot.unwrap().id, 7);
        assert_eq!(told_record::the_current_generation(c).unwrap(), generation, "nothing to tell the machines but the switch");
        assert_eq!(crate::record::telegram_bot::the_current_bot_tokens_item(c).unwrap(), item);

        // **Where there is no bot to use, or its token is not stored for it,
        // the walkthrough opens as before.**
        the_store::telegram_was_switched(c, false).unwrap();
        crate::record::telegram_bot::the_stored_token_is_forgotten(c).unwrap();
        assert_eq!(telegram_is_switched_on_in(c), WhatSwitchingOnDid::TheWalkthroughOpens);
        assert!(!the_store::the_telegram_settings(c).unwrap().switched_on);
        drop(record);
        let (folder_2, record) = a_record("note1-no-bot");
        assert_eq!(telegram_is_switched_on_in(record.connection()), WhatSwitchingOnDid::TheWalkthroughOpens);
        drop(record);
        let _ = fs::remove_dir_all(folder);
        let _ = fs::remove_dir_all(folder_2);
    }

    #[test]
    fn fifteenth_note_2_a_save_can_be_retried_without_another_start() {
        for failure in [StoreFailure::LockedOrRefused, StoreFailure::StoreUnavailable] {
            let (folder, record) = a_record(&format!("note2-save-{failure:?}"));
            let c = record.connection();
            let store = AFailingStore::default();
            store.put_failure.set(Some(failure));
            let Err((retry, sentence)) = the_bot_is_kept_in(c, Some(&store), TheKeepToRetry::Keep(a_new_bot_says_hello(7))) else { panic!("the save failed") };
            assert_eq!(sentence, if failure == StoreFailure::LockedOrRefused { words::THE_TOKEN_STORE_REFUSED } else { words::THE_TOKEN_WAS_NOT_CONFIRMED });
            assert!(!the_store::the_telegram_settings(c).unwrap().switched_on);
            assert!(!what_the_section_holds(c, None).the_token_is_held);
            store.put_failure.set(None);
            assert!(the_bot_is_kept_in(c, Some(&store), retry).is_ok());
            assert!(what_the_section_holds(c, None).the_token_is_held);
            drop(record);
            let _ = fs::remove_dir_all(folder);
        }
    }

    #[test]
    fn fifteenth_note_2_no_store_keeps_the_proof_for_retry_and_says_why() {
        let (folder, record) = a_record("note2-no-store");
        let c = record.connection();
        let Err((retry, sentence)) = the_bot_is_kept_in(c, None, TheKeepToRetry::Keep(a_new_bot_says_hello(7))) else { panic!("no store") };
        assert_eq!(sentence, words::THE_SECRET_STORE_UNAVAILABLE);
        assert_eq!(TheProvedBotRecord::over(c).the_proved_bot().unwrap(), None);
        assert!(the_bot_is_kept_in(c, Some(&AStore::default()), retry).is_ok());
        drop(record);
        let _ = fs::remove_dir_all(folder);
    }

    #[test]
    fn fifteenth_note_2_a_failed_enable_does_not_claim_setup_and_retries_only_the_switch() {
        let (folder, record) = a_record("note2-enable");
        let c = record.connection();
        the_store::telegram_was_switched(c, false).unwrap();
        c.execute_batch("CREATE TRIGGER refuse_telegram_enable BEFORE UPDATE ON telegram_setting BEGIN SELECT RAISE(ABORT, 'test refusal'); END;").unwrap();
        let store = AStore::default();
        let Err((retry, sentence)) = the_bot_is_kept_in(c, Some(&store), TheKeepToRetry::Keep(a_new_bot_says_hello(7))) else { panic!("the switch did not save") };
        assert_eq!(sentence, words::TELEGRAM_COULD_NOT_BE_ENABLED);
        assert!(matches!(&retry, TheKeepToRetry::Enable(_)));
        assert!(!the_store::the_telegram_settings(c).unwrap().switched_on);
        assert!(what_the_section_holds(c, None).the_token_is_held);
        c.execute_batch("DROP TRIGGER refuse_telegram_enable").unwrap();
        assert!(the_bot_is_kept_in(c, None, retry).is_ok(), "enabling again needs no secret store");
        assert!(the_store::the_telegram_settings(c).unwrap().switched_on);
        drop(record);
        let _ = fs::remove_dir_all(folder);
    }

    /// **fix-here item 8: the waiting box can carry Design's *could not reach Telegram*
    /// and still says it is waiting.**
    #[test]
    fn the_waiting_box_can_say_telegram_was_not_reached() {
        let username = "smoozy_a7k2_bot".to_string();
        let reached = said(&the_walkthrough_says(&TheStage::WaitingForStart { username: username.clone(), not_reached: false, copy_the_name: false }, None));
        assert!(!reached.contains("theFailure"));
        let unreached = said(&the_walkthrough_says(&TheStage::WaitingForStart { username, not_reached: true, copy_the_name: false }, None));
        assert!(unreached.contains(&format!("{:?}", Prose::new(words::NOT_REACHED))));
        assert!(unreached.contains(&format!("{:?}", Verbatim::new("waiting"))));
    }

    /// **fix-here item 13: an over-long or empty paste is answered with the *no token*
    /// stage**; anything else goes to the check.
    #[test]
    fn a_paste_that_cannot_be_checked_is_answered() {
        for paste in [String::new(), "1".repeat(THE_LONGEST_A_PASTE_MAY_BE + 1)] {
            let Some(TheStage::AFailureAtThePaste { sentence, .. }) = the_stage_for_a_paste_that_cannot_be_checked(&paste) else {
                panic!("a paste of {} characters was not answered", paste.len());
            };
            assert_eq!(sentence, words::NO_TOKEN_IN_THE_PASTE);
        }
        assert_eq!(the_stage_for_a_paste_that_cannot_be_checked("123:abc"), None);
    }

    /// **The page and this file name the same kinds and commands.**
    #[test]
    fn the_page_and_this_file_name_the_same_things() {
        let page = the_page();
        for named in [
            THE_SECTION,
            THE_WALKTHROUGH,
            CHECK_AGAIN,
            KEEP_AGAIN,
            "telegram_is_switched",
            "the_telegram_walkthrough_is_opened",
            "another_username_is_offered",
            "the_telegram_walkthrough_is_shut",
            "the_pasted_token_is_checked",
            "the_bot_is_kept_again",
            "the_replacement_token_is_saved",
            // The seventeenth notes' piece 7: the replace box's stage and the
            // one-time copy's mark.
            "\"replace\"",
            "copyTheName",
            "theBotsName",
            "the_reporting_default_is_set",
            // The five of 2026-10-31: the third's minutes field and its refusal.
            "the_default_minutes_are_typed",
            "theMinutesChoiceIs",
            "theDefaultMinutes",
            "theMinutesAreRefused",
            "minutesBetweenPictures",
            "a_notification_switch_is_turned",
            // The round *setting it up, and what the desktop keeps*: fix-here item 14's
            // stage, fix-here item 2's line and fix-here item 11's sentence.
            "\"notkept\"",
            "theTokenIsNotStored",
            "howToStoreIt",
            "notReachedWouldNotRead",
        ] {
            assert!(page.contains(named), "the page does not name {named}");
        }
    }

    /// **Nothing here raises a failure record**, so `WhoseSentenceItIs` is not
    /// reached.
    #[test]
    fn no_failure_record_is_raised_here() {
        let working = include_str!("telegram.rs").split("#[cfg(test)]").next().unwrap();
        for raising in ["the_two_hooks", "failure_records", "a_condition_was_raised"] {
            assert!(!working.contains(&format!("{raising}::")), "this file reaches {raising}");
        }
    }

    // ════════════════════════════════════════════════════════════════════════
    // The bot step as drawn again on 2026-10-24 — the seventeenth notes' piece 7
    // ════════════════════════════════════════════════════════════════════════

    fn a_passed_check(service: &Arc<AStandInService>, id: i64, username: &str) -> TheCheckPassed {
        use smoozy_telegram::walkthrough::check_the_bot_answers_through;
        service.then_answers(&serde_json::json!({"ok":true,"result":{"id":id,"is_bot":true,"username":username}}).to_string());
        let reach = |_: &str| TheClient::over(service.clone());
        let WhatTheCheckFound::Answered(check) = check_the_bot_answers_through(
            &ThePaste::what_the_user_pasted(format!("{id}:AAH-not_a_real_token_0123456789abcd")),
            &reach,
        ) else {
            panic!("the stand-in bot answered")
        };
        check
    }

    /// **`set-bot-replace` — `Telegram` P4: the bot this computer already
    /// uses saves with no Start.** The check returns the same bot, renamed since
    /// (`Telegram` S3a's answer, not the old name); the box names it the way Settings' *Bot*
    /// reading does and carries no link and no copy; *Save token* stores the new
    /// token, keeps the proof and asks nothing more of Telegram, so the desktop
    /// never receives on a token a server may hold (`Telegram` S5).
    #[test]
    fn the_bot_this_computer_already_uses_saves_with_no_start() {
        let (folder, record) = a_record("already-used");
        let c = record.connection();
        let store = AFailingStore::default();
        assert!(the_bot_is_kept_in(c, Some(&store), TheKeepToRetry::Keep(a_new_bot_says_hello(7))).is_ok());
        let (proved, conversation) = TheProvedBotRecord::over(c).the_proved_bot().unwrap().unwrap();

        let service = Arc::new(AStandInService::new());
        let check = a_passed_check(&service, 7, "studio_render_bot");
        let AfterTheCheck::TheBotAlreadyUsed(stage, check) = after_the_check(check, Some(&proved)) else {
            panic!("the bot this computer already uses was sent to Start")
        };
        assert_eq!(stage, TheStage::TheBotAlreadyUsed { username: "studio_render_bot".into() });
        let text = said(&the_walkthrough_says(&stage, None));
        assert!(text.contains(&format!("{:?}", Verbatim::new("@studio_render_bot"))), "{text}");
        assert!(text.contains(&format!("{:?}", Verbatim::new("replace"))));
        for drawn in [words::IS_THE_BOT_SMOOZY_ALREADY_USES, words::NO_START_IS_NEEDED, words::SERVERS_GET_THE_NEW_TOKEN, words::SAVE_TOKEN] {
            assert!(text.contains(&format!("{:?}", Prose::new(drawn))), "{drawn} is not said");
        }
        for absent in ["theLink", "copyTheName", "theBotsName"] {
            assert!(!text.contains(absent), "{absent} is drawn where no Start is needed");
        }

        assert!(the_replacement_is_saved_in(c, Some(&store), check).is_ok());
        let methods: Vec<String> = service.heard().into_iter().map(|call| call.method).collect();
        assert_eq!(methods, vec!["getMe".to_string()], "Smoozy received on the replacement token");
        let (bot, kept) = TheProvedBotRecord::over(c).the_proved_bot().unwrap().unwrap();
        assert_eq!((bot.id, bot.username.as_str(), kept), (7, "studio_render_bot", conversation));
        assert!(the_store::the_telegram_settings(c).unwrap().switched_on);
        let item = crate::record::telegram_bot::the_current_bot_tokens_item(c).unwrap();
        assert_eq!(
            store.inner.held.borrow().get(&item).cloned(),
            Some(b"7:AAH-not_a_real_token_0123456789abcd".to_vec()),
            "the new token was not the one stored"
        );
        drop(record);
        let _ = fs::remove_dir_all(folder);
    }

    /// **A save with no store says so, and keeps nothing.** The page goes back
    /// to the paste on its route with the paste kept, which the stage already
    /// says for a failed save.
    #[test]
    fn a_replacement_with_no_store_is_not_saved_and_says_why() {
        let service = Arc::new(AStandInService::new());
        let (folder, record) = a_record("already-used-no-store");
        let c = record.connection();
        let check = a_passed_check(&service, 7, "studio_render_bot");
        assert_eq!(the_replacement_is_saved_in(c, None, check).err(), Some(words::THE_TOKEN_WAS_NOT_STORED));
        drop(record);
        let _ = fs::remove_dir_all(folder);
    }

    /// **`set-bot-ok` — any other bot goes to Start, and the name copied is the
    /// one the check returned**, bare, once: the first drawing of the wait
    /// carries `copyTheName`, the later ones and the reply do not (`Telegram`
    /// S6a). With no bot on record, after switching off erased it (`Telegram`
    /// K1), the same.
    #[test]
    fn any_other_bot_goes_to_start_and_its_answered_name_is_copied_once() {
        for proved in [Some(TheBot { id: 7, username: "studio_render_bot".into() }), None] {
            let service = Arc::new(AStandInService::new());
            let check = a_passed_check(&service, 8, "studio_render_bot");
            let AfterTheCheck::WaitForStart(stage, _wait) = after_the_check(check, proved.as_ref()) else {
                panic!("a bot with no proof here skipped Start")
            };
            assert_eq!(
                stage,
                TheStage::WaitingForStart { username: "studio_render_bot".into(), not_reached: false, copy_the_name: true }
            );
            let first = said(&the_walkthrough_says(&stage, Some("smoozy_a7k2_bot")));
            assert!(first.contains(&format!("\"copyTheName\": Value({:?})", Verbatim::new("yes"))), "{first}");
            assert!(first.contains(&format!("\"theBotsName\": Value({:?})", Verbatim::new("studio_render_bot"))), "{first}");
            assert!(first.contains(&format!("\"theLink\": Value({:?})", Verbatim::new("t.me/studio_render_bot"))), "{first}");
            for later in [
                TheStage::WaitingForStart { username: "studio_render_bot".into(), not_reached: true, copy_the_name: false },
                TheStage::WaitingForStart { username: "studio_render_bot".into(), not_reached: false, copy_the_name: false },
                TheStage::TheBotReplied { username: "studio_render_bot".into() },
                TheStage::NotKept { username: "studio_render_bot".into(), sentence: words::THE_TOKEN_WAS_NOT_CONFIRMED },
            ] {
                let text = said(&the_walkthrough_says(&later, None));
                assert!(!text.contains("copyTheName"), "{later:?} copies the name again");
                assert!(text.contains(&format!("\"theBotsName\": Value({:?})", Verbatim::new("studio_render_bot"))), "{later:?}");
            }
        }
    }

    /// **The drawn words, and the ones that went with the redraw.** The
    /// existing-bot route asks nothing but the new token (`Telegram` S6a: *"No
    /// affirmation is asked"*), and the old dialog's words are gone.
    #[test]
    fn the_bot_step_says_the_drawings_words_and_not_the_old_ones() {
        let text = said(&the_walkthrough_says(&TheStage::ThePaste, Some("smoozy_a7k2_bot")));
        for drawn in [
            words::SET_UP_TELEGRAM,
            words::DO_THIS_WHERE_YOU_CAN_REACH_TELEGRAM,
            words::MAKE_A_NEW_BOT,
            words::USE_AN_EXISTING_BOT,
            words::A_NEW_TOKEN_IS_NEEDED,
            words::THE_OLD_TOKENS_SERVERS_STOP,
            words::CHECK_BOT,
        ] {
            assert!(text.contains(&format!("{:?}", Prose::new(drawn))), "{drawn} is not said");
        }
        let source = include_str!("telegram.rs").split("#[cfg(test)]").next().unwrap();
        for gone in ["No server has this token", "Make a Telegram bot", "\"Continue\"", "\"Copy\""] {
            assert!(!source.contains(gone), "{gone} is still said");
        }
    }

    /// **One delivery as the page receives it**, through the real queue.
    fn as_the_page_gets_it(event: Event) -> serde_json::Value {
        struct Kept(Arc<Mutex<Vec<serde_json::Value>>>);
        impl crate::events::Delivers for Kept {
            fn deliver(&self, delivery: &crate::events::Delivery) {
                self.0.lock().unwrap().push(serde_json::to_value(delivery).unwrap());
            }
        }
        let kept = Arc::new(Mutex::new(Vec::new()));
        let queue = TheOneQueue::new();
        queue.the_page_is_ready(Box::new(Kept(kept.clone())));
        let _ = queue.read_then_post::<(), std::convert::Infallible>(|| Ok(()), move |_| event.clone());
        let one = kept.lock().unwrap().pop().expect("the event was delivered");
        one
    }

    /// **Every route and state of the bot step, in a real browser** — headless
    /// Chrome draws the dialog from `web/telegram.js` and the application's own
    /// stylesheets, fed the walkthrough's own deliveries, and the page script
    /// presses what a user presses. **The clipboard is a stand-in** that takes
    /// the copies and can refuse them; the page proves it took before anything
    /// is pressed, so no test reaches the computer's real clipboard.
    ///
    /// The thirteen drawn bot states, each checked as the drawing draws it:
    /// the choice alone (`set-bot`); each route with only its own steps
    /// (`set-bot-new`, `set-bot-existing`), the names' copy icons beside them
    /// and no tick-box anywhere; checking locks route, paste and Check bot
    /// (`set-botcheck`, `set-bot-existingcheck`); both failures keep route and
    /// paste (`set-botbad`, `set-botnet`, `set-bot-existingbad`,
    /// `set-bot-existingnet`); the bot already used saves with no Start
    /// (`set-bot-replace`); Start shows one link with the icon right beside
    /// it and copies the bare name once (`set-bot-ok`), again from the icon,
    /// and says in place when the copy fails without blocking Start
    /// (`set-bot-copyfail`); the reply brings Done (`set-bot-start`). A
    /// computer with no Chrome skips it.
    #[test]
    fn every_route_and_state_of_the_bot_step_is_drawn_in_a_real_browser() {
        let chrome = [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/usr/bin/google-chrome",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        ]
        .into_iter()
        .map(std::path::PathBuf::from)
        .find(|one| one.exists());
        let Some(chrome) = chrome else {
            eprintln!("SKIPPED: no Chrome on this computer, so the bot step was not drawn");
            return;
        };
        let name = "studio_render_bot".to_string();
        let offer = Some("smoozy_a7k2_bot");
        let refused = the_stage_after_a_check_that_did_not_answer(&WhatTheCheckFound::NotAccepted).unwrap();
        let unreached = the_stage_after_a_check_that_did_not_answer(&WhatTheCheckFound::DidNotFinish(
            WhyTheCheckDidNotFinish::TelegramCouldNotBeReached,
        ))
        .unwrap();
        let deliveries = serde_json::json!({
            "open": as_the_page_gets_it(the_walkthrough_says(&TheStage::ThePaste, offer)),
            "refused": as_the_page_gets_it(the_walkthrough_says(&refused, offer)),
            "unreached": as_the_page_gets_it(the_walkthrough_says(&unreached, offer)),
            "replace": as_the_page_gets_it(the_walkthrough_says(&TheStage::TheBotAlreadyUsed { username: name.clone() }, offer)),
            "waiting": as_the_page_gets_it(the_walkthrough_says(
                &TheStage::WaitingForStart { username: name.clone(), not_reached: false, copy_the_name: true }, offer)),
            "waitingUnreached": as_the_page_gets_it(the_walkthrough_says(
                &TheStage::WaitingForStart { username: name.clone(), not_reached: true, copy_the_name: false }, offer)),
            "waitingAgain": as_the_page_gets_it(the_walkthrough_says(
                &TheStage::WaitingForStart { username: name.clone(), not_reached: false, copy_the_name: false }, offer)),
            "replied": as_the_page_gets_it(the_walkthrough_says(&TheStage::TheBotReplied { username: name.clone() }, offer)),
            "shut": as_the_page_gets_it(the_walkthrough_says(&TheStage::Shut, None)),
        });

        let folder = std::env::temp_dir().join(format!("smoozy-the-bot-step-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for entry in fs::read_dir(&web).expect("the page folder is readable") {
            let path = entry.expect("an entry").path();
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if path.is_file() && (extension == "js" || extension == "css") {
                fs::copy(&path, folder.join(path.file_name().unwrap())).expect("the page is copied");
            }
        }
        fs::write(folder.join("deliveries.js"), format!("export default {deliveries};\n")).expect("the deliveries are written");
        fs::write(folder.join("index.html"), THE_BOT_STEP_PAGE).expect("the page is written");
        let dumped = folder.join("dumped.html");
        let mut chrome = std::process::Command::new(chrome)
            .args([
                "--headless",
                "--disable-gpu",
                "--no-first-run",
                "--allow-file-access-from-files",
                "--virtual-time-budget=8000",
                "--window-size=1280,900",
                "--dump-dom",
            ])
            .arg(format!("--user-data-dir={}", folder.join("profile").display()))
            .arg(format!("file://{}", folder.join("index.html").display()))
            .stdout(fs::File::create(&dumped).expect("the dump is written"))
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Chrome starts");
        // A fixed deadline, never an open wait.
        let deadline = std::time::Instant::now() + Duration::from_secs(60);
        while chrome.try_wait().expect("Chrome is watched").is_none() {
            if std::time::Instant::now() > deadline {
                let _ = chrome.kill();
                let _ = chrome.wait();
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        let page = fs::read_to_string(&dumped).unwrap_or_default();
        // **Where `SMOOZY_THE_BOT_STEP_PAGE` names a folder, the page and its
        // deliveries are left there** — for the builder's own look beside the
        // drawing's pictures, which no assertion stands in for.
        if let Some(keep) = std::env::var_os("SMOOZY_THE_BOT_STEP_PAGE") {
            let keep = std::path::PathBuf::from(keep);
            let _ = fs::create_dir_all(&keep);
            for entry in fs::read_dir(&folder).into_iter().flatten().flatten() {
                if entry.path().is_file() {
                    let _ = fs::copy(entry.path(), keep.join(entry.file_name()));
                }
            }
        }
        let _ = fs::remove_dir_all(&folder);
        let out = page
            .split("<pre id=\"out\">")
            .nth(1)
            .and_then(|rest| rest.split("</pre>").next())
            .filter(|said| !said.is_empty())
            .unwrap_or_else(|| panic!("the page drew nothing: {page}"));
        let out = out
            .replace("&nbsp;", "\u{a0}")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&amp;", "&");
        let out: serde_json::Value = serde_json::from_str(&out).expect("one JSON answer");
        assert!(out["error"].is_null(), "the page stopped: {} {}", out["error"], out["troubles"]);
        assert_eq!(out["troubles"], serde_json::json!([]), "the page reported trouble");
        assert_eq!(out["standInTook"], true, "the stand-in clipboard did not take, so nothing was pressed");
        let states = &out["states"];
        let at = |state: &str| -> &serde_json::Value {
            let one = &states[state];
            assert!(!one.is_null(), "{state} was not reached: {out}");
            one
        };
        let flat = |text: &serde_json::Value| text.as_str().unwrap_or("").replace('\u{a0}', " ");
        let has = |state: &str, which: &str, words: &str| {
            let text = flat(&at(state)["text"][which]);
            assert!(text.contains(&words.replace('\u{a0}', " ")), "{state}: the {which} box does not say {words:?}: {text:?}");
        };
        let lacks = |state: &str, which: &str, words: &str| {
            let text = flat(&at(state)["text"][which]);
            assert!(!text.contains(words), "{state}: the {which} box says {words:?}: {text:?}");
        };
        let shown = |state: &str, boxes: [bool; 3]| {
            let one = at(state);
            assert_eq!([one["paste"].as_bool(), one["replace"].as_bool(), one["ok"].as_bool()], boxes.map(Some), "{state}: which box shows: {one}");
        };
        let pressed = |state: &str| -> Vec<String> {
            at(state)["routes"].as_array().unwrap().iter().filter(|r| r["pressed"] == "true").map(|r| flat(&r["text"])).collect()
        };
        let route = |state: &str, existing: bool, creating: bool| {
            let one = at(state);
            assert_eq!((one["existing"].as_bool(), one["creating"].as_bool()), (Some(existing), Some(creating)), "{state}: which route's steps show: {one}");
        };
        let fields = |state: &str| -> Vec<(String, String, bool)> {
            at(state)["fields"].as_array().unwrap().iter()
                .map(|f| (flat(&f["id"]), flat(&f["value"]), f["disabled"].as_bool().unwrap()))
                .collect()
        };
        let locked = |state: &str, yes: bool| {
            let one = at(state);
            assert_eq!(one["go"]["underway"], yes, "{state}: Check bot's mark: {one}");
            assert_eq!(one["go"]["disabled"], yes, "{state}: Check bot: {one}");
            for r in one["routes"].as_array().unwrap() {
                assert_eq!(r["disabled"], yes, "{state}: a route button: {one}");
            }
            for (id, _, disabled) in fields(state) {
                assert_eq!(disabled, yes, "{state}: {id}");
            }
        };
        let token = "123456789:AAH-not_a_real_token_0123456789abcd".to_string();
        for state in states.as_object().unwrap().keys() {
            assert_eq!(at(state)["stepsInTheListsInk"], true, "{state}: a step is not drawn in the list's own ink");
            assert_eq!(at(state)["checkboxes"], 0, "{state} has a tick-box");
            assert_eq!(at(state)["wordCopy"], 0, "{state} has a button that says Copy");
        }

        // `set-bot`: the choice alone.
        shown("set-bot", [true, false, false]);
        let routes: Vec<String> = at("set-bot")["routes"].as_array().unwrap().iter().map(|r| flat(&r["text"])).collect();
        assert_eq!(routes, vec![words::MAKE_A_NEW_BOT, words::USE_AN_EXISTING_BOT]);
        assert!(pressed("set-bot").is_empty());
        route("set-bot", false, false);
        assert!(fields("set-bot").is_empty(), "a field shows before a route is chosen");
        assert_eq!(at("set-bot")["go"]["shown"], false, "Check bot shows before a route is chosen");
        has("set-bot", "paste", words::DO_THIS_WHERE_YOU_CAN_REACH_TELEGRAM);
        has("set-bot", "paste", words::SET_UP_TELEGRAM);

        // `set-bot-new`: creation's three steps and nothing of the other route.
        shown("set-bot-new", [true, false, false]);
        assert_eq!(pressed("set-bot-new"), vec![words::MAKE_A_NEW_BOT]);
        route("set-bot-new", false, true);
        assert_eq!(fields("set-bot-new"), vec![("botnewtoken".to_string(), String::new(), false)]);
        assert_eq!(at("set-bot-new")["go"]["shown"], true);
        assert_eq!(at("set-bot-new")["go"]["disabled"], false);
        for words in ["/newbot", "@BotFather", "Smoozy", "smoozy_a7k2_bot", words::ANOTHER, words::THE_TOKEN_LETS_SMOOZY_USE_YOUR_BOT, words::NOTHING_IS_SAVED_BEFORE] {
            has("set-bot-new", "paste", words);
        }
        lacks("set-bot-new", "paste", "revoke");
        let names = at("set-bot-new")["names"].as_array().unwrap();
        assert_eq!(names.len(), 2);
        for (one, label) in names.iter().zip([words::COPY_NAME, words::COPY_USERNAME]) {
            assert_eq!(one["isIcon"], true, "the name's control is not the copy icon: {one}");
            assert_eq!(flat(&one["label"]), label);
            let gap = one["gap"].as_f64().unwrap();
            assert!((0.0..=12.0).contains(&gap), "the copy icon is not right beside its name: {one}");
            assert_eq!(one["sameLine"], true, "{one}");
        }
        assert_eq!(at("set-bot-new")["anotherAfterTheIcon"], true, "Another does not follow the username's icon");
        assert_eq!(at("copied-username")["copied"], serde_json::json!(["smoozy_a7k2_bot"]));

        // `set-bot-existing`: the reason, the two steps, the cost, no tick-box.
        shown("set-bot-existing", [true, false, false]);
        assert_eq!(pressed("set-bot-existing"), vec![words::USE_AN_EXISTING_BOT]);
        route("set-bot-existing", true, false);
        assert_eq!(fields("set-bot-existing"), vec![("botexistingtoken".to_string(), String::new(), false)]);
        assert_eq!(at("set-bot-existing")["fieldType"], "password");
        assert_eq!(at("set-bot-existing")["go"]["disabled"], false, "Check bot is not available at once");
        assert_eq!(at("set-bot-existing")["warning"], true, "the cost is not drawn as the warning");
        for words in [words::A_NEW_TOKEN_IS_NEEDED, words::AND_REVOKE_ITS_TOKEN, words::YOU_GET_A_NEW_ONE, words::PASTE_THE_NEW_TOKEN_HERE, words::NOTHING_IS_SAVED_BEFORE, words::THE_OLD_TOKENS_SERVERS_STOP] {
            has("set-bot-existing", "paste", words);
        }
        lacks("set-bot-existing", "paste", "/newbot");
        lacks("set-bot-existing", "paste", "smoozy_a7k2_bot");

        // `set-bot-existingcheck`, `-existingbad`, `-existingnet`.
        locked("set-bot-existingcheck", true);
        assert_eq!(at("set-bot-existingcheck")["lastPaste"], serde_json::json!(token));
        for state in ["set-bot-existingbad", "set-bot-existingnet"] {
            shown(state, [true, false, false]);
            route(state, true, false);
            assert_eq!(pressed(state), vec![words::USE_AN_EXISTING_BOT]);
            assert_eq!(fields(state), vec![("botexistingtoken".to_string(), token.clone(), false)], "{state}: the paste is not kept");
            locked(state, false);
        }
        has("set-bot-existingbad", "paste", words::NOT_ACCEPTED);
        has("set-bot-existingnet", "paste", words::NOT_REACHED);
        has("set-bot-existingnet", "paste", words::TRY_AGAIN);
        locked("existing-try-again", true);
        assert_eq!(at("existing-try-again")["checks"], 3, "Retry did not check again");

        // `set-bot-replace`: the bot Smoozy already uses — Save token, no Start.
        shown("set-bot-replace", [false, true, false]);
        for words in ["@studio_render_bot", words::IS_THE_BOT_SMOOZY_ALREADY_USES, words::NO_START_IS_NEEDED, words::SERVERS_GET_THE_NEW_TOKEN, words::SAVE_TOKEN, words::CANCEL] {
            has("set-bot-replace", "replace", words);
        }
        lacks("set-bot-replace", "replace", words::PRESS);
        assert_eq!(at("set-bot-replace")["copied"], serde_json::json!(["smoozy_a7k2_bot"]), "something was copied where no Start is needed");
        assert!(at("saved")["calls"].as_array().unwrap().contains(&serde_json::json!("the_replacement_token_is_saved")));
        shown("shut", [false, false, false]);

        // A new opening forgets the route and the paste.
        shown("reopened", [true, false, false]);
        assert!(pressed("reopened").is_empty());
        route("reopened", false, false);
        assert_eq!(at("reopened")["anyValue"], false, "a paste outlived the dialog");

        // `set-botcheck`, `set-botbad`, `set-botnet` on the new-bot route.
        locked("set-botcheck", true);
        for state in ["set-botbad", "set-botnet"] {
            shown(state, [true, false, false]);
            route(state, false, true);
            assert_eq!(fields(state), vec![("botnewtoken".to_string(), token.clone(), false)], "{state}: the paste is not kept");
            locked(state, false);
        }
        has("set-botbad", "paste", words::NOT_ACCEPTED);
        has("set-botnet", "paste", words::NOT_REACHED);

        // `set-bot-ok`: one link, the icon right beside it, the bare name copied once.
        shown("set-bot-ok", [false, false, true]);
        assert_eq!(at("set-bot-ok")["links"], serde_json::json!(["t.me/studio_render_bot"]));
        let link = &at("set-bot-ok")["link"];
        assert_eq!(link["isIcon"], true, "the link's control is not the copy icon: {link}");
        assert_eq!(flat(&link["label"]), words::COPY_BOT_NAME);
        assert!((0.0..=12.0).contains(&link["gap"].as_f64().unwrap()), "the copy icon is not right beside the link: {link}");
        assert_eq!(link["sameLine"], true, "{link}");
        let copies = |state: &str| at(state)["copied"].as_array().unwrap().iter().filter(|c| **c == serde_json::json!(name)).count();
        assert_eq!(copies("set-bot-ok"), 1, "the name was not copied once on entering Start");
        for one in at("set-bot-ok")["copied"].as_array().unwrap() {
            assert!(!flat(one).contains("t.me") && !flat(one).starts_with('@'), "more than the bare name was copied: {one}");
        }
        let line = |state: &str| (flat(&at(state)["copyResult"]["text"]), at(state)["copyResult"]["bad"].as_bool());
        let copied_line = format!("{} {}", words::BOT_NAME_COPIED, words::YOU_CAN_PASTE_IT);
        let failed_line = format!("{} {}", words::THE_BOT_NAME_WAS_NOT_COPIED, words::USE_THE_COPY_ICON);
        assert_eq!(line("set-bot-ok"), (copied_line.clone(), Some(false)));
        assert_eq!(at("set-bot-ok")["done"], false, "Done shows before Start");
        for words in [words::THE_BOT_ANSWERED, words::TWO_THINGS_LEFT, words::OPEN_YOUR_BOT, words::PRESS, words::UNTIL_YOU_DO, "Waiting for you to press Start."] {
            has("set-bot-ok", "ok", words);
        }
        // The wait drawn again, with and without Telegram reached, copies nothing.
        has("waiting-unreached", "ok", words::NOT_REACHED);
        assert_eq!(copies("waiting-unreached"), 1, "a later drawing of the wait copied again");
        assert_eq!(copies("waiting-again"), 1, "a later drawing of the wait copied again");
        assert_eq!(line("waiting-again"), (copied_line.clone(), Some(false)));
        assert_eq!(copies("icon-pressed"), 2, "the icon did not copy the name again");

        // `set-bot-start`: the reply, and Done.
        shown("set-bot-start", [false, false, true]);
        assert_eq!(at("set-bot-start")["done"], true);
        assert_eq!(at("set-bot-start")["links"], serde_json::json!(["t.me/studio_render_bot"]));
        has("set-bot-start", "ok", words::THE_BOT_REPLIED);
        lacks("set-bot-start", "ok", "Waiting for you");

        // `set-bot-copyfail`: said in place, and Start is not blocked.
        shown("set-bot-copyfail", [false, false, true]);
        assert_eq!(line("set-bot-copyfail"), (failed_line, Some(true)));
        has("set-bot-copyfail", "ok", words::PRESS);
        has("set-bot-copyfail", "ok", "Waiting for you to press Start.");
        assert_eq!(at("set-bot-copyfail")["links"], serde_json::json!(["t.me/studio_render_bot"]));
        assert_eq!(line("copyfail-retried"), (copied_line.clone(), Some(false)));
        assert_eq!(copies("copyfail-retried"), copies("set-bot-copyfail") + 1);

        // Where the engine has no way to ready a copy at the press, the plain
        // write still copies on entering Start.
        assert_eq!(copies("set-bot-ok-plain"), copies("copyfail-retried") + 1);
        assert_eq!(line("set-bot-ok-plain"), (copied_line, Some(false)));
    }

    /// **The page that presses through the bot step** and writes what it saw
    /// into `#out`. Its stand-in clipboard is installed before any module runs.
    const THE_BOT_STEP_PAGE: &str = r##"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
</head><body><div id="screen"><section class="stgs" id="tgsection"></section></div><pre id="out"></pre>
<script>
  window.__calls = [];
  window.__TAURI__ = {
    core: { invoke(command, args) { window.__calls.push({ command, args: args || {} }); return Promise.resolve(null); } },
    event: { listen(name, back) { window.__listener = back; return Promise.resolve(() => {}); } },
  };
  // The stand-in clipboard. Nothing here reaches the computer's own.
  window.__copied = [];
  window.__clipboardWorks = true;
  const standIn = {
    writeText(text) {
      if (!window.__clipboardWorks) return Promise.reject(new Error("refused"));
      window.__copied.push(text);
      return Promise.resolve();
    },
    async write(items) {
      const blob = await items[0].kept["text/plain"];
      if (!window.__clipboardWorks) throw new Error("refused");
      window.__copied.push(await blob.text());
    },
  };
  Object.defineProperty(navigator, "clipboard", { value: standIn, configurable: true });
  window.ClipboardItem = class { constructor(kept) { this.kept = kept; } };
  // The words a readied copy carries, read back at once rather than by the
  // engine's own reading, so the page's turn and the test's agree.
  window.Blob = class { constructor(parts) { this.parts = parts; } text() { return Promise.resolve(this.parts.join("")); } };
  document.execCommand = () => false;
  window.__standInTook = navigator.clipboard === standIn && document.execCommand() === false;
  // What the page's own handlers report as trouble.
  window.__troubles = [];
  console.error = (...said) => window.__troubles.push(said.map((one) => String(one && one.stack || one)).join(" "));
</script>
<script type="module">
import { theRouteIsOpen } from "./event-route.js";
import { theTelegramSectionIsBuilt } from "./telegram.js";
import deliveries from "./deliveries.js";
const out = { states: {}, standInTook: window.__standInTook };
const write = () => { out.troubles = window.__troubles; document.getElementById("out").textContent = JSON.stringify(out); };
window.addEventListener("error", (event) => { out.error = String(event.message); write(); });
window.addEventListener("unhandledrejection", (event) => { out.error = String(event.reason && event.reason.stack || event.reason); write(); });
try {
  if (!window.__standInTook) throw new Error("the stand-in clipboard did not take");
  theTelegramSectionIsBuilt(document.getElementById("tgsection"), document.getElementById("screen"));
  await theRouteIsOpen();
  let sequence = 0;
  const settle = async () => { for (let at = 0; at < 12; at += 1) await new Promise((go) => setTimeout(go, 0)); };
  const tell = async (name) => {
    const delivery = structuredClone(deliveries[name]);
    sequence += 1;
    delivery.sequence = 1000 + sequence;
    window.__listener({ payload: delivery });
    await settle();
  };
  const shown = (el) => Boolean(el) && !el.closest("[hidden]") && el.getClientRects().length > 0;
  const buttons = () => [...document.querySelectorAll("button")].filter(shown);
  const press = async (words) => {
    const control = buttons().find((b) => b.textContent.replace(/\u00a0/g, " ").trim() === words);
    if (!control) throw new Error("nothing to press says " + words);
    control.click();
    await settle();
  };
  const pressLabelled = async (label) => {
    const control = buttons().find((b) => (b.getAttribute("aria-label") || "").replace(/\u00a0/g, " ") === label);
    if (!control) throw new Error("nothing to press is labelled " + label);
    control.click();
    await settle();
  };
  const type = (words) => {
    const field = [...document.querySelectorAll(".modal input")].find(shown);
    if (!field) throw new Error("no field to type in");
    field.value = words;
    field.dispatchEvent(new Event("input", { bubbles: true }));
  };
  const beside = (thing) => {
    if (!thing) return null;
    const icon = thing.nextElementSibling;
    if (!icon) return { isIcon: false };
    const a = thing.getBoundingClientRect();
    const b = icon.getBoundingClientRect();
    return {
      isIcon: icon.classList.contains("ib") && Boolean(icon.querySelector("svg")),
      label: (icon.getAttribute("aria-label") || "").replace(/\u00a0/g, " "),
      gap: Math.round((b.left - a.right) * 10) / 10,
      sameLine: Math.abs((a.top + a.bottom) / 2 - (b.top + b.bottom) / 2) < 6,
    };
  };
  const look = (state) => {
    const paste = document.getElementById("stgbot");
    const replace = document.getElementById("botreplace");
    const ok = document.getElementById("stgbotok");
    const go = document.getElementById("stgbotgo");
    const result = document.getElementById("botcopyresult");
    const checks = window.__calls.filter((c) => c.command === "the_pasted_token_is_checked");
    const text = (root) => (shown(root) ? root.innerText : "");
    const existingField = document.getElementById("botexistingtoken");
    const newField = document.getElementById("botnewtoken");
    const usernameRow = document.querySelectorAll("#botcreate .tg-nm")[1];
    out.states[state] = {
      paste: shown(paste),
      replace: shown(replace),
      ok: shown(ok),
      routes: [...document.querySelectorAll("#stgbot .botroute .btn")].filter(shown)
        .map((b) => ({ text: b.textContent.replace(/\u00a0/g, " ").trim(), pressed: b.getAttribute("aria-pressed"), disabled: b.disabled })),
      existing: shown(document.getElementById("botexisting")),
      creating: shown(document.getElementById("botcreate")),
      fields: [...document.querySelectorAll("#stgbot input")].filter(shown)
        .map((f) => ({ id: f.id, value: f.value, disabled: f.disabled })),
      fieldType: existingField ? existingField.type : null,
      anyValue: Boolean((existingField && existingField.value) || (newField && newField.value)),
      checkboxes: document.querySelectorAll(".modal input[type=checkbox]").length,
      wordCopy: buttons().filter((b) => b.textContent.trim() === "Copy").length,
      warning: shown(document.querySelector("#botexisting .mwarn")),
      stepsInTheListsInk: [...document.querySelectorAll(".modal .bsteps > li")].filter(shown)
        .every((li) => getComputedStyle(li).color === getComputedStyle(li.parentElement).color),
      go: go ? { shown: shown(go), disabled: go.disabled, underway: go.classList.contains("uway") } : null,
      text: { paste: text(paste), replace: text(replace), ok: text(ok) },
      links: shown(ok) ? [...ok.querySelectorAll("a")].filter(shown).map((a) => a.textContent) : [],
      link: shown(ok) ? beside(ok.querySelector(".botstartlink a")) : null,
      names: shown(document.getElementById("botcreate"))
        ? [...document.querySelectorAll("#botcreate .tg-nm .mono")].map(beside) : [],
      anotherAfterTheIcon: Boolean(usernameRow) && shown(usernameRow)
        && Boolean(usernameRow.querySelector(".ib + .mini")),
      copyResult: result ? { text: shown(result) ? result.textContent : "", bad: result.classList.contains("bad") } : null,
      done: buttons().some((b) => b.closest("#stgbotok .mact") && !b.classList.contains("quiet")),
      copied: [...window.__copied],
      calls: window.__calls.map((c) => c.command),
      checks: checks.length,
      lastPaste: checks.length ? checks[checks.length - 1].args.paste : null,
    };
  };
  const token = "123456789:AAH-not_a_real_token_0123456789abcd";

  await tell("open"); look("set-bot");
  await press("Create a new bot"); look("set-bot-new");
  await pressLabelled("Copy username"); look("copied-username");
  await press("Use an existing bot"); look("set-bot-existing");
  type(token);
  await press("Check bot"); look("set-bot-existingcheck");
  await tell("refused"); look("set-bot-existingbad");
  await press("Check bot"); await tell("unreached"); look("set-bot-existingnet");
  await press("Retry"); look("existing-try-again");
  await tell("replace"); look("set-bot-replace");
  await press("Save token"); look("saved");
  await tell("shut"); look("shut");

  await tell("open"); look("reopened");
  await press("Create a new bot"); type(token);
  await press("Check bot"); look("set-botcheck");
  await tell("refused"); look("set-botbad");
  await press("Check bot"); await tell("unreached"); look("set-botnet");
  await press("Retry");
  await tell("waiting"); look("set-bot-ok");
  await tell("waitingUnreached"); look("waiting-unreached");
  await tell("waitingAgain"); look("waiting-again");
  await pressLabelled("Copy bot name"); look("icon-pressed");
  await tell("replied"); look("set-bot-start");
  await tell("shut");

  await tell("open"); await press("Create a new bot"); type(token);
  window.__clipboardWorks = false;
  await press("Check bot"); await tell("waiting"); look("set-bot-copyfail");
  window.__clipboardWorks = true;
  await pressLabelled("Copy bot name"); look("copyfail-retried");
  await tell("shut");

  window.ClipboardItem = undefined;
  await tell("open"); await press("Create a new bot"); type(token);
  await press("Check bot"); await tell("waiting"); look("set-bot-ok-plain");
  await tell("shut");
} catch (trouble) {
  out.error = String(trouble && trouble.stack || trouble);
}
write();
</script></body></html>"##;
}
