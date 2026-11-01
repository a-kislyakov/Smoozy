//! **The client against Telegram's real service, on the Creator's own bot.**
//!
//! **Off by default, and it says so when skipped.** The hosted runners have no
//! token, and only one agent may use the bot at a time: Telegram hands a bot's
//! updates to one caller, and the newest caller wins. Run it by hand, at the
//! desk, alone:
//!
//! `cargo test -p smoozy-telegram --test the_real_service_answers -- --ignored --nocapture`
//!
//! **What it changes: nothing.** It reads the webhook state and the standing
//! filter first; asks who the bot is; asks for updates once, with no wait and
//! no `offset`, so no update is confirmed and the position stays the Creator's;
//! and reads the webhook state again, which must be exactly as found. **It
//! sends no message and pins nothing.**
//!
//! **Why it stops before asking where the filter is not already the two kinds.**
//! `Telegram` P6: an ask that names kinds replaces the token's standing filter.
//! Where the filter found is anything else, asking would change something that
//! is the Creator's, so the check stops and says what it found.
//!
//! **The token is read by this process from its file and goes nowhere else.**
//! It is never printed, and nothing this file prints can carry it.

use std::collections::BTreeSet;

use serde_json::Value;
use smoozy_telegram::transport::{TheClient, THE_UPDATE_KINDS_EVERY_ASK_NAMES};

/// **The one value in the key file shaped like a bot token.**
///
/// The keys folder's own README gives every file there the same layout: a
/// first line saying what the key is for, then one `NAME=value` per line. So
/// the file's whole text is not the token, and neither is a whole line. **The
/// value after the first `=` is read**, or the whole line where it has none,
/// since one file in that folder is known to be a bare value. The shape —
/// digits, a colon, then letters, digits, `-` and `_` — is the one the service's
/// own page shows in its example address. **Nothing about the value is
/// printed**, and where the file holds no such value, or more than one, the
/// check stops without saying what it found.
fn the_token() -> String {
    let home = std::env::var_os("HOME").expect("this desk has a home folder");
    let path = std::path::Path::new(&home)
        .join(".smoozy-keys")
        .join("telegram.txt");
    let text = std::fs::read_to_string(path).expect("the bot token file is readable");
    let shaped_like_a_token: Vec<&str> = text
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
                    && secret
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            }
            None => false,
        })
        .collect();
    assert!(
        shaped_like_a_token.len() == 1,
        "the key file does not hold exactly one line shaped like a bot token"
    );
    shaped_like_a_token[0].to_string()
}

fn as_a_set(filter: Option<&Value>) -> Option<BTreeSet<String>> {
    filter?.as_array().map(|kinds| {
        kinds
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect()
    })
}

#[test]
#[ignore = "uses the Creator's live bot token; run by hand at the desk, one agent at a time, with -- --ignored --nocapture"]
fn the_real_service_answers_the_three_calls_that_change_nothing() {
    let client = TheClient::reaching_telegram_with(&the_token());

    let before = client
        .get_webhook_info()
        .expect("getWebhookInfo was answered");
    let url_before = before.get("url").cloned();
    let filter_before = before.get("allowed_updates").cloned();
    eprintln!(
        "before: webhook address empty = {}, standing filter = {}, updates waiting = {}",
        url_before.as_ref().and_then(Value::as_str) == Some(""),
        filter_before
            .as_ref()
            .map_or("not stated".to_string(), Value::to_string),
        before
            .get("pending_update_count")
            .map_or("not stated".to_string(), Value::to_string),
    );

    let me = client.get_me().expect("getMe was answered");
    assert_eq!(
        me.get("is_bot"),
        Some(&Value::Bool(true)),
        "the token is a bot's"
    );
    eprintln!(
        "getMe: a bot, with a username = {}",
        me.get("username").and_then(Value::as_str).is_some()
    );

    // **Whether the ask may be made at all.** Where it may not, it is skipped,
    // the state is still read again and compared, and only then does the check
    // stop and say why — so a stopped check still proves it changed nothing.
    let no_webhook = url_before.as_ref().and_then(Value::as_str) == Some("");
    let ours: BTreeSet<String> = THE_UPDATE_KINDS_EVERY_ASK_NAMES
        .iter()
        .map(|kind| kind.to_string())
        .collect();
    let the_filter_is_already_ours = as_a_set(filter_before.as_ref()) == Some(ours);
    let the_ask_may_be_made = no_webhook && the_filter_is_already_ours;

    if the_ask_may_be_made {
        let updates = client
            .get_updates(None, 0)
            .expect("getUpdates was answered");
        eprintln!(
            "getUpdates, no wait, no offset: {} updates came back and none was confirmed",
            updates.as_array().map_or(0, Vec::len)
        );
    } else {
        eprintln!("getUpdates: not asked, because asking would change what was found");
    }

    let after = client
        .get_webhook_info()
        .expect("getWebhookInfo was answered again");
    eprintln!(
        "after: webhook address empty = {}, standing filter = {}",
        after.get("url").and_then(Value::as_str) == Some(""),
        after
            .get("allowed_updates")
            .map_or("not stated".to_string(), Value::to_string),
    );
    assert_eq!(
        after.get("url").cloned(),
        url_before,
        "the webhook address moved"
    );
    assert_eq!(
        after.get("allowed_updates").cloned(),
        filter_before,
        "the standing filter moved"
    );
}
