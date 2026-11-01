//! **The walkthrough against Telegram's real service, on the Creator's own bot,
//! with the proof recorded in a copy of the local record.** Step 10 piece 4.
//!
//! **Every test that reaches the bot is off by default and says so when skipped.** The hosted
//! runners have no token, and only one agent may use the bot at a time. Run one
//! at a time, by hand, at the desk.
//!
//! - **`a_wrong_token_is_not_accepted_and_the_real_one_answers`** asks no
//!   updates. It reads the webhook state before and after and they must match.
//! - **`with_the_network_cut_the_check_is_unreachable_and_the_paste_is_kept`**
//!   is run inside a sandbox that denies this one process the network, and
//!   fails if it is run with the network there.
//! - **`the_creator_presses_start_and_the_proof_is_recorded`** asks for updates
//!   and so runs only on the runner's turn. It needs `SMOOZY_RECORD_COPY` set
//!   to a copy of the record outside the repository, and refuses the real one.
//!   It ends by asking once more with the standing filter exactly as found,
//!   erasing the keychain item it wrote and checking it is gone. **Since
//!   record version 46 each bot's key has its own item name**, reserved in the
//!   record when it is saved, so the item is named from the copy of the record
//!   and never from the old fixed name.
//! - **`the_start_tests_clean_up_erases_the_key_under_the_name_the_copy_gave_it`**
//!   is the one test here that runs by default. It reaches no bot and no
//!   keychain: it checks the Start test's naming and clean-up over a scratch
//!   record and a store held in memory.
//!
//! **The token is read by this process from its file and goes nowhere else.**
//! Nothing this file prints can carry it.
//!
//! **Markers in this file are `Telegram`'s and are written with that document
//! at every site.**

use std::path::{Path, PathBuf};
use std::time::Duration;

use smoozy_desktop::record::telegram_bot::the_current_bot_tokens_item;
use smoozy_desktop::record::LocalRecord;
use smoozy_security_and_licence::held::HeldSecret;
use smoozy_security_and_licence::secret_store::{GetFailure, SecretStore};

use smoozy_telegram::transport::TheClient;
use smoozy_telegram::walkthrough::{
    check_the_bot_answers, ThePaste, WhatTheCheckFound, WhyTheCheckDidNotFinish,
};

/// The one value in the key file shaped like a bot token — the same reading as
/// `App/Telegram/tests/the_real_service_answers.rs`. Nothing about it is
/// printed.
fn the_token() -> String {
    let home = std::env::var_os("HOME").expect("this desk has a home folder");
    let path = PathBuf::from(home).join(".smoozy-keys").join("telegram.txt");
    let text = std::fs::read_to_string(path).expect("the bot token file is readable");
    let shaped: Vec<String> = text
        .lines()
        .map(|line| match line.split_once('=') {
            Some((_, value)) => value.trim(),
            None => line.trim(),
        })
        .filter(|value| match value.split_once(':') {
            Some((bot, secret)) => {
                !bot.is_empty()
                    && bot.chars().all(|c| c.is_ascii_digit())
                    && !secret.is_empty()
                    && secret.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            }
            None => false,
        })
        .map(str::to_string)
        .collect();
    assert!(shaped.len() == 1, "the key file does not hold exactly one line shaped like a bot token");
    shaped.into_iter().next().unwrap()
}

/// The reply the bot-making chat gives, with the token somewhere inside it
/// (`Telegram` S2).
fn a_reply_carrying(token: &str) -> ThePaste {
    ThePaste::what_the_user_pasted(format!(
        "Done! Congratulations on your new bot.\nUse this token to access the HTTP API:\n{token}\nKeep your token secure."
    ))
}

/// **The webhook state as read**: whether the address is empty, the standing
/// filter as its kinds in order, and how many updates wait.
#[derive(Debug, PartialEq, Eq)]
struct TheWebhookState {
    address_empty: bool,
    filter: Option<Vec<String>>,
    waiting: Option<u64>,
}

fn the_webhook_state(client: &TheClient, when: &str) -> TheWebhookState {
    let info = client.get_webhook_info().expect("getWebhookInfo was answered");
    let state = TheWebhookState {
        address_empty: info.get("url").and_then(|url| url.as_str()) == Some(""),
        filter: info.get("allowed_updates").and_then(|kinds| kinds.as_array()).map(|kinds| {
            kinds
                .iter()
                .filter_map(|kind| kind.as_str().map(str::to_string))
                .collect()
        }),
        waiting: info.get("pending_update_count").and_then(|count| count.as_u64()),
    };
    eprintln!(
        "{when}: webhook address empty = {}, standing filter = {:?}, updates waiting = {:?}",
        state.address_empty, state.filter, state.waiting
    );
    state
}


// ------------------------------------------------------------ the bot key's item

/// **The bot key item the copy of the record names**: the one a save reserves
/// and the app reads afterwards (record version 46 gives each bot its own).
/// A copy that has never reserved one names the old fixed item, as the app
/// does.
fn the_item_the_copy_names(record: &LocalRecord) -> HeldSecret {
    the_current_bot_tokens_item(record.connection()).expect("the copy of the record names its bot key item")
}

/// **The Start test's clean-up: the key it saved, erased and checked gone.**
/// The item is named from the copy at the moment of putting back, so a save
/// that reserved a fresh item is found however the test ended: the record
/// reserves the item before the store is written. Answers whether the erase
/// answered ok and whether the item was then checked gone.
fn erase_the_key_the_copy_names(store: &dyn SecretStore, copy: &Path) -> Result<(bool, bool), String> {
    let record = LocalRecord::open_at(copy).map_err(|why| format!("the copy of the record did not open: {why:?}"))?;
    let item = the_current_bot_tokens_item(record.connection())
        .map_err(|why| format!("the copy of the record did not name the item: {why:?}"))?;
    drop(record);
    let erased = store.erase(&item, smoozy_telegram::walkthrough::THE_STORE_CALLS_DEADLINE);
    let gone = matches!(
        store.get(&item, smoozy_telegram::walkthrough::THE_STORE_CALLS_DEADLINE),
        Err(GetFailure::NotThere)
    );
    Ok((erased.is_ok(), gone))
}

#[test]
#[ignore = "uses the Creator's live bot token and asks no updates; run by hand at the desk, one agent at a time, with -- --ignored --nocapture"]
fn a_wrong_token_is_not_accepted_and_the_real_one_answers() {
    let token = the_token();
    let reader = TheClient::reaching_telegram_with(&token);
    let before = the_webhook_state(&reader, "before");

    // A token of the right shape that is not the Creator's.
    let wrong = a_reply_carrying("1000000001:NotTheCreatorsTokenNotTheCreatorsTok");
    let found = check_the_bot_answers(&wrong);
    eprintln!("a wrong token: {found:?}");
    assert!(matches!(found, WhatTheCheckFound::NotAccepted), "`Telegram` S3: a wrong token reads as refused");

    let right = a_reply_carrying(&token);
    match check_the_bot_answers(&right) {
        WhatTheCheckFound::Answered(check) => {
            eprintln!("the real token: answered, bot @{}, link {}", check.the_bot().username, check.the_link());
        }
        other => panic!("`Telegram` S3: the real token reads as answered, not {other:?}"),
    }

    let after = the_webhook_state(&reader, "after");
    assert_eq!(after.address_empty, before.address_empty, "the webhook address moved");
    assert_eq!(after.filter, before.filter, "the standing filter moved");
}

#[test]
#[ignore = "run inside a sandbox that denies this process the network; it fails if the network is there"]
fn with_the_network_cut_the_check_is_unreachable_and_the_paste_is_kept() {
    let paste = a_reply_carrying(&the_token());
    let found = check_the_bot_answers(&paste);
    eprintln!("with the network cut: {found:?}");
    assert!(
        matches!(found, WhatTheCheckFound::DidNotFinish(WhyTheCheckDidNotFinish::TelegramCouldNotBeReached)),
        "`Telegram` S3: unreachable is an ordinary failure and never a wrong token"
    );
    assert!(paste.holds_a_token(), "`Telegram` S3: the paste is kept for the retry");
}

#[cfg(target_os = "macos")]
mod on_the_runners_turn {
    use super::*;
    use std::time::{Instant, SystemTime};

    use smoozy_desktop::diary_service::DiaryService;
    use smoozy_desktop::platform::secrets::MacKeychain;
    use smoozy_desktop::record::telegram_bot::{a_bot_set_up, TheProvedBotRecord};
    use smoozy_state::identities::{Identity, IdentityKind};
    use smoozy_telegram::proof::KeepsTheProvedBot;
    use smoozy_telegram::transport::{OverHttps, ReachesTelegram, WhatACallCarries};
    use smoozy_telegram::walkthrough::{WhatComesNext, WhatOneAskFound, THE_STORE_CALLS_DEADLINE};

    /// **What must be put back however the Start test ends**: the keychain item
    /// it may have written, named from the copy of the record, and the standing
    /// filter as found. Runs on the way out, including after a failure.
    struct PutBack {
        token: String,
        copy: PathBuf,
        filter_as_found: Vec<String>,
        done: bool,
    }

    impl PutBack {
        fn now(&mut self) {
            if self.done {
                return;
            }
            self.done = true;

            match erase_the_key_the_copy_names(&MacKeychain, &self.copy) {
                Ok((erased, gone)) => {
                    eprintln!("keychain item: erase answered ok = {erased}, checked gone = {gone}")
                }
                Err(why) => eprintln!("keychain item: not erased, the copy of the record did not name it: {why}"),
            }

            // **The ask with the kinds exactly as found**, through the one wire,
            // with no offset, so it confirms nothing.
            let wire = OverHttps::reaching_telegram_with(&self.token);
            let mut carries = WhatACallCarries::Fields(Default::default());
            if let WhatACallCarries::Fields(fields) = &mut carries {
                fields.insert("timeout".into(), 0.into());
                fields.insert("allowed_updates".into(), self.filter_as_found.clone().into());
            }
            let asked = wire.call("getUpdates", &carries, Duration::from_secs(30));
            eprintln!("the filter put back: the ask was answered = {}", asked.is_ok());
        }
    }

    impl Drop for PutBack {
        fn drop(&mut self) {
            self.now();
        }
    }

    #[test]
    #[ignore = "asks for updates on the Creator's live bot and needs the Creator to press Start; only on the runner's turn, with SMOOZY_RECORD_COPY set"]
    fn the_creator_presses_start_and_the_proof_is_recorded() {
        let copy = PathBuf::from(
            std::env::var_os("SMOOZY_RECORD_COPY").expect("SMOOZY_RECORD_COPY names a copy of the record"),
        );
        let home = PathBuf::from(std::env::var_os("HOME").unwrap());
        assert!(
            !copy.starts_with(home.join("Library/Application Support")),
            "this is the Creator's real application data; the test runs only against a copy"
        );

        let token = the_token();
        let reader = TheClient::reaching_telegram_with(&token);
        let before = the_webhook_state(&reader, "before");
        assert!(before.address_empty, "a webhook is registered; this test does not remove one");
        let filter_as_found = before.filter.clone().expect("the standing filter is stated");

        let store = MacKeychain;
        {
            // **Checked before anything is written**, so the clean-up can only
            // ever erase what this test saved: the item the copy names now is
            // empty, and any item a save reserves later is fresh.
            let record = LocalRecord::open_at(&copy).expect("the copy of the record opens");
            assert!(
                matches!(
                    store.get(&the_item_the_copy_names(&record), THE_STORE_CALLS_DEADLINE),
                    Err(GetFailure::NotThere)
                ),
                "the bot key item the copy names is already in the keychain; this test writes none over it"
            );
        }

        let mut put_back = PutBack { token: token.clone(), copy: copy.clone(), filter_as_found, done: false };

        let record = LocalRecord::open_at(&copy).expect("the copy of the record opens");
        eprintln!("the copy of the record is at version {}", record.schema_version().unwrap());
        let keeper = TheProvedBotRecord::over(record.connection());
        let proved_on_record = keeper.the_proved_bot().expect("the copy reads").map(|(bot, _)| bot);

        let paste = a_reply_carrying(&token);
        let WhatTheCheckFound::Answered(check) = check_the_bot_answers(&paste) else {
            panic!("`Telegram` S3: the real token answers");
        };
        let WhatComesNext::WaitForStart(mut wait) = check.what_comes_next(proved_on_record.as_ref()) else {
            panic!("`Telegram` S5: the copy holds no proved bot, so the walkthrough waits for Start");
        };
        eprintln!("waiting for Start: open {} and press Start", wait.the_check().the_link());

        let deadline = Instant::now() + Duration::from_secs(20 * 60);
        let proof = loop {
            assert!(Instant::now() < deadline, "no Start arrived in twenty minutes");
            wait = match wait.ask_once(20) {
                WhatOneAskFound::StartArrived(proof) => break proof,
                WhatOneAskFound::NothingYet(wait) => {
                    eprintln!("nothing yet; confirmed below {:?}", wait.confirmed_below());
                    wait
                }
                WhatOneAskFound::TheAskFailed(wait, why) => {
                    eprintln!("an ask failed: {why:?}");
                    std::thread::sleep(Duration::from_secs(2));
                    wait
                }
            };
        };
        eprintln!("`Telegram` P1: a Start arrived from a conversation that could be established");

        let hello = proof
            .say_hello_back()
            .map_err(|(_, why)| why)
            .expect("`Telegram` S6: hello back was sent");
        eprintln!("`Telegram` S6: hello back was sent");
        let bot = hello.the_bot().clone();
        let conversation = hello.the_conversation();

        let set_up = hello
            .keep(&keeper, &store, &mut smoozy_desktop::record::told_record::TheBotTokensMarks(record.connection()))
            .map_err(|(_, why)| why)
            .expect("`Telegram` S4: the proof was recorded and the token stored");
        assert_eq!(keeper.the_proved_bot().unwrap(), Some((bot.clone(), conversation)), "the copy reads back the proof");
        let saved = the_item_the_copy_names(&record);
        let matches = store
            .get(&saved, THE_STORE_CALLS_DEADLINE)
            .expect("`Telegram` K1: the token is in the keychain")
            .expose(|bytes| bytes == token.as_bytes());
        assert!(matches, "the keychain holds the token that was checked");
        eprintln!("`Telegram` S4: proof recorded in the copy for @{}, token stored and read back", bot.username);

        let run = Identity::issue(IdentityKind::Run).expect("random bytes");
        let line = a_bot_set_up(run, SystemTime::now(), &set_up);
        assert!(!line.text.contains(&token), "the act line carries no token");
        let diary = DiaryService::open_at(copy.parent().unwrap().join("diary")).expect("a diary beside the copy");
        eprintln!("`Telegram` §19: the act line was handed to a diary beside the copy: {:?}", diary.write(&line));

        drop(record);
        put_back.now();
        assert!(matches!(store.get(&saved, THE_STORE_CALLS_DEADLINE), Err(GetFailure::NotThere)));
        let after = the_webhook_state(&reader, "at the end");
        assert_eq!(after.address_empty, before.address_empty, "the webhook address moved");
        assert_eq!(after.filter, before.filter, "the standing filter was not put back as found");
    }
}

/// **The Start test's naming and clean-up, checked without the bot or the
/// keychain.** The save is what `keep` does in order: the proof recorded, the
/// item reserved, the key written, the record told. The clean-up runs twice
/// over: once after a whole save, and once after a save that stopped with the
/// key written and the record not yet told, as a failed run would leave it.
/// Each time nothing may be left in the store.
mod the_clean_up_without_the_keychain {
    use super::*;
    use std::cell::RefCell;

    use smoozy_desktop::record::telegram_bot::TheProvedBotRecord;
    use smoozy_security_and_licence::plaintext::Plaintext;
    use smoozy_security_and_licence::secret_store::{ProbeOutcome, StoreFailure};
    use smoozy_telegram::proof::{KeepsTheProvedBot, TheBot, TheConversation};
    use smoozy_telegram::walkthrough::{the_bot_tokens_item, THE_STORE_CALLS_DEADLINE};

    /// A store held in memory.
    #[derive(Default)]
    struct AStore(RefCell<Vec<(HeldSecret, Vec<u8>)>>);

    impl SecretStore for AStore {
        fn put(&self, item: &HeldSecret, value: Plaintext, _: Duration) -> Result<(), StoreFailure> {
            let bytes = value.expose(<[u8]>::to_vec);
            let mut held = self.0.borrow_mut();
            held.retain(|(there, _)| there != item);
            held.push((item.clone(), bytes));
            Ok(())
        }

        fn get(&self, item: &HeldSecret, _: Duration) -> Result<Plaintext, GetFailure> {
            self.0
                .borrow()
                .iter()
                .find(|(there, _)| there == item)
                .map(|(_, bytes)| Plaintext::new(bytes.clone()))
                .ok_or(GetFailure::NotThere)
        }

        fn erase(&self, item: &HeldSecret, _: Duration) -> Result<(), StoreFailure> {
            self.0.borrow_mut().retain(|(there, _)| there != item);
            Ok(())
        }

        fn probe(&self, _: Duration) -> ProbeOutcome {
            ProbeOutcome::RoundTripWorked
        }

        fn enumerate(&self, _: Duration) -> Result<Vec<HeldSecret>, StoreFailure> {
            Ok(self.0.borrow().iter().map(|(there, _)| there.clone()).collect())
        }
    }

    /// A scratch folder that takes itself away however the test ends.
    struct Scratch(PathBuf);

    impl Scratch {
        fn new(named: &str) -> Scratch {
            let folder = std::env::temp_dir().join(format!("smoozy-bot-clean-up-{named}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&folder);
            std::fs::create_dir_all(&folder).expect("a scratch folder");
            Scratch(folder)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn the_start_tests_clean_up_erases_the_key_under_the_name_the_copy_gave_it() {
        for (ending, the_record_is_told) in [("whole", true), ("stopped", false)] {
            let scratch = Scratch::new(ending);
            let copy = scratch.0.join("local-record.sqlite3");
            let store = AStore::default();
            let bot = TheBot { id: 1_234_567_890, username: "smoozy_a7k2_bot".into() };
            {
                let record = LocalRecord::open_at(&copy).expect("the scratch record opens");
                assert!(
                    matches!(
                        store.get(&the_item_the_copy_names(&record), THE_STORE_CALLS_DEADLINE),
                        Err(GetFailure::NotThere)
                    ),
                    "{ending}: the check before names an item that is empty"
                );
                let keeper = TheProvedBotRecord::over(record.connection());
                keeper.the_proof_was_recorded(&bot, TheConversation(-9_001)).unwrap();
                let item = keeper.the_item_for_saving(&bot).unwrap();
                assert_ne!(item, the_bot_tokens_item(), "{ending}: a save goes under the bot's own item");
                store.put(&item, Plaintext::new(b"1000000001:AKeyForTheTest".to_vec()), THE_STORE_CALLS_DEADLINE).unwrap();
                if the_record_is_told {
                    keeper.the_token_was_stored_for(&bot).unwrap();
                }
                assert_eq!(the_item_the_copy_names(&record), item, "{ending}: the copy names the item the save used");
            }

            let (erased, gone) = erase_the_key_the_copy_names(&store, &copy).expect("the copy names the item");
            assert!(erased, "{ending}: the erase answered ok");
            assert!(gone, "{ending}: the item was checked gone");
            assert!(
                store.enumerate(THE_STORE_CALLS_DEADLINE).unwrap().is_empty(),
                "{ending}: the clean-up left a saved key behind"
            );
        }
    }
}
