//! **The told-record and a replacement token, watched on a copy of the
//! Creator's local record, with `getMe` against the real service and the real
//! keychain.** Step 10 piece 5.
//!
//! **Off by default and says so when skipped.** It needs `SMOOZY_RECORD_COPY`
//! set to a copy of the record outside the repository and refuses the real one.
//! **It asks Telegram for no updates**: its only calls are two `getMe`, one on a
//! deliberately wrong token and one on the Creator's own, so it needs no turn on
//! the token.
//!
//! What it watches:
//!
//! - the copy migrated to this binary's version;
//! - two seeded servers told everything, and the derivation naming neither;
//! - a wrong token read as not accepted, with nothing moved;
//! - the Creator's own token accepted as a replacement for the bot on record,
//!   the old keychain item erased, the token stored and read back, and both
//!   servers named as left on the old generation;
//! - the role's material composed from the copy for one server;
//! - §19's line written to a diary beside the copy, carrying no token;
//! - Telegram switched off, and the keychain item checked gone from inside and
//!   from outside;
//! - one server removed, and its told entries gone with it.
//!
//! **The proof on the copy is seeded, not proved**: the Creator's record holds
//! no bot, and a replacement needs one on record. The seeded conversation is
//! written as such.
//!
//! **The token is read by this process from its file and goes nowhere else.**
//!
//! **Markers in this file belong to other documents and are written with their
//! document at every site.**

#[cfg(target_os = "macos")]
mod on_the_desk {
    use std::path::PathBuf;
    use std::time::{Duration, SystemTime};

    use smoozy_desktop::diary_service::DiaryService;
    use smoozy_desktop::platform::secrets::MacKeychain;
    use smoozy_desktop::record::server_rows;
    use smoozy_desktop::record::telegram_bot::{TheProvedBotRecord, the_current_bot_tokens_item};
    use smoozy_desktop::record::told_record::{
        a_replacement_token_accepted, a_replacement_token_accepted_line, the_current_generation,
        the_material_for, the_servers_not_reached,
        the_telling_was_acknowledged, the_replaced_token_is_erased,
        what_this_server_was_told,
    };
    use smoozy_desktop::record::LocalRecord;
    use smoozy_security_and_licence::erasure::Erasure;
    use smoozy_security_and_licence::secret_store::{GetFailure, SecretStore};
    use smoozy_servers::identity::{Host, KeyReference, Port, Username};
    use smoozy_servers::store::{RecordedMachineRoot, ServerRow};
    use smoozy_state::identities::{Identity, IdentityKind};
    use smoozy_telegram::proof::{KeepsTheProvedBot, TheConversation};
    use smoozy_telegram::told::{AToldThing, TokenGeneration, WhatSmoozyTells};
    use smoozy_telegram::walkthrough::{
        check_the_bot_answers, the_bot_tokens_item, ThePaste, WhatTheCheckFound, THE_STORE_CALLS_DEADLINE,
    };

    /// The one value in the key file shaped like a bot token — the same reading
    /// as the walkthrough's live test. Nothing about it is printed.
    fn the_token() -> String {
        let home = std::env::var_os("HOME").expect("this desk has a home folder");
        let text = std::fs::read_to_string(PathBuf::from(home).join(".smoozy-keys").join("telegram.txt"))
            .expect("the bot token file is readable");
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

    fn a_reply_carrying(token: &str) -> ThePaste {
        ThePaste::what_the_user_pasted(format!("Use this token to access the HTTP API:\n{token}\nKeep it secure."))
    }

    fn nothing_under_the_service_name_from_outside() -> bool {
        // Asks for the item's attributes and never its value.
        let asked = std::process::Command::new("/usr/bin/security")
            .args(["find-generic-password", "-s", "Smoozy.bot-token"])
            .output()
            .expect("the security tool runs");
        !asked.status.success()
    }

    /// **Whatever happens, the keychain item this test may write is erased.**
    struct PutBack(PathBuf);

    impl Drop for PutBack {
        fn drop(&mut self) {
            // Only the opted-in scratch record supplies this item's name.
            // New saves have their own subject, rather than the legacy one.
            let Ok(record) = LocalRecord::open_at(&self.0) else { return };
            let Ok(item) = the_current_bot_tokens_item(record.connection()) else { return };
            let _ = MacKeychain.erase(&item, THE_STORE_CALLS_DEADLINE);
            let gone = matches!(
                MacKeychain.get(&item, THE_STORE_CALLS_DEADLINE),
                Err(GetFailure::NotThere)
            );
            eprintln!(
                "put back: keychain item gone from inside = {gone}, from outside = {}",
                nothing_under_the_service_name_from_outside()
            );
        }
    }

    fn a_server(label: &str) -> ServerRow {
        let installation = Identity::issue(IdentityKind::Installation).unwrap().as_text();
        let mut row = ServerRow::saved(
            Identity::issue(IdentityKind::LocalServer).unwrap().as_text(),
            Host::what_the_user_typed("192.0.2.10").expect("a host"),
            Port::what_the_user_typed(22).expect("a port"),
            Username::what_the_user_typed("root").expect("a username"),
            KeyReference { local_key_identifier: "seeded-key".into(), fingerprint: "SHA256:seeded".into() },
        );
        row.label = Some(label.into());
        row.last_confirmed_installation_identity = Some(installation.clone());
        row.recorded_machine_root = Some(RecordedMachineRoot {
            path: "/mnt/one/smoozy".into(),
            the_package_areas_own_location: None,
            read_under: installation,
        });
        row
    }

    #[test]
    #[ignore = "uses a copy of the Creator's record, the real keychain and two getMe calls on the live bot; run by hand at the desk with SMOOZY_RECORD_COPY set, -- --ignored --nocapture"]
    fn the_told_record_and_a_replacement_on_a_copy_of_the_creators_record() {
        let copy = PathBuf::from(std::env::var_os("SMOOZY_RECORD_COPY").expect("SMOOZY_RECORD_COPY names a copy"));
        let home = PathBuf::from(std::env::var_os("HOME").unwrap());
        assert!(
            !copy.starts_with(home.join("Library/Application Support")),
            "this is the Creator's real application data; the test runs only against a copy"
        );
        assert!(
            matches!(MacKeychain.get(&the_bot_tokens_item(), THE_STORE_CALLS_DEADLINE), Err(GetFailure::NotThere)),
            "a bot token item is already in the keychain; this test writes none over it"
        );
        assert!(nothing_under_the_service_name_from_outside());
        let _put_back = PutBack(copy.clone());

        let record = LocalRecord::open_at(&copy).expect("the copy opens");
        let c = record.connection();
        eprintln!("the copy is at version {}", record.schema_version().unwrap());

        // Two seeded servers, told everything at the first generation.
        let now = |generation| {
            vec![
                WhatSmoozyTells::the_roles_desired_state(true),
                WhatSmoozyTells::the_token_generation(generation),
                WhatSmoozyTells::the_proved_conversation(TheConversation(1)),
                WhatSmoozyTells::the_reporting_settings([("a-server-has-had-nothing-to-do", true)]),
            ]
        };
        let rows = [a_server("seeded one"), a_server("seeded two")];
        let at = SystemTime::now();
        for row in &rows {
            server_rows::add(c, row).unwrap();
            server_rows::write(c, row).unwrap();
            let installation = row.last_confirmed_installation_identity.as_deref().unwrap();
            let mut values = now(TokenGeneration::THE_FIRST);
            values.push(WhatSmoozyTells::the_label(row.label.as_deref()));
            for value in &values {
                the_telling_was_acknowledged(c, &row.local_server_identifier, value, at, installation).unwrap();
            }
        }
        assert!(the_servers_not_reached(c, &now(TokenGeneration::THE_FIRST)).unwrap().is_empty());
        eprintln!("`Telegram` L3: two seeded servers told everything; none is named as not reached");
        // A deliberately wrong token: not accepted, nothing moves.
        let wrong = check_the_bot_answers(&a_reply_carrying("1000000001:NotTheCreatorsTokenNotTheCreatorsTok"));
        eprintln!("getMe on a wrong token: {wrong:?}");
        assert!(matches!(wrong, WhatTheCheckFound::NotAccepted));
        assert_eq!(the_current_generation(c).unwrap(), TokenGeneration::THE_FIRST);

        // The Creator's own token, for the bot seeded on the copy.
        let token = the_token();
        let WhatTheCheckFound::Answered(check) = check_the_bot_answers(&a_reply_carrying(&token)) else {
            panic!("`Telegram` S3: the real token answers");
        };
        eprintln!("getMe on the Creator's token: answered, @{}", check.the_bot().username);
        TheProvedBotRecord::over(c)
            .the_proof_was_recorded(check.the_bot(), TheConversation(1))
            .expect("the seeded proof is written to the copy");

        let accepted = a_replacement_token_accepted(c, &MacKeychain, check).expect("`Telegram` L4: accepted");
        assert_eq!(accepted.generation, TokenGeneration(2));
        assert_eq!(accepted.left_on_an_old_generation.len(), 2);
        let saved_item = the_current_bot_tokens_item(c).expect("the reserved bot item");
        let stored = MacKeychain
            .get(&saved_item, THE_STORE_CALLS_DEADLINE)
            .expect("`Telegram` K1: the token is in the keychain")
            .expose(|bytes| bytes == token.as_bytes());
        assert!(stored, "the keychain holds the token that was checked");
        let not_reached = the_servers_not_reached(c, &now(TokenGeneration(2))).unwrap();
        assert_eq!(not_reached.len(), 2);
        assert!(not_reached.iter().all(|server| server.not_told == vec![AToldThing::TheTokenGeneration]));
        eprintln!(
            "`Telegram` L4: generation 2 stored and read back; both servers named as left on the old generation: {:?}",
            accepted.left_on_an_old_generation
        );

        // The material for one server, composed from the copy.
        let material = the_material_for(c, &MacKeychain, &rows[0], true).expect("`Telegram` §20: composed");
        assert_eq!(material.the_files().len(), 2);
        assert!(material.the_files()[0].1 == token.as_bytes(), "the token file carries the token");
        assert!(!String::from_utf8_lossy(&material.the_files()[1].1).contains(&token));
        eprintln!("`Telegram` §20: composed for one server: {material:?}");
        drop(material);

        // §19's second act.
        let line = a_replacement_token_accepted_line(Identity::issue(IdentityKind::Run).unwrap(), SystemTime::now(), &accepted);
        assert!(!line.text.contains(&token), "the act line carries no token");
        let diary = DiaryService::open_at(copy.parent().unwrap().join("diary")).expect("a diary beside the copy");
        eprintln!("`Telegram` §19: the act line handed to a diary beside the copy: {:?}", diary.write(&line));

        // The token replaced: `Security and licence` E14's erase, which since
        // 2026-10-31 switching Telegram off no longer makes.
        assert_eq!(the_replaced_token_is_erased(c, &MacKeychain), Erasure::Done);
        assert!(matches!(MacKeychain.get(&saved_item, THE_STORE_CALLS_DEADLINE), Err(GetFailure::NotThere)));
        assert!(nothing_under_the_service_name_from_outside());
        eprintln!("`Security and licence` E14: the item erased, and checked gone from inside and outside");

        // A removal takes the told entries with it.
        server_rows::remove_walking_the_enumeration(c, &rows[0].local_server_identifier).unwrap();
        assert!(what_this_server_was_told(c, &rows[0].local_server_identifier).unwrap().is_empty());
        assert_eq!(what_this_server_was_told(c, &rows[1].local_server_identifier).unwrap().len(), 5);
        eprintln!("a removed server's told entries are gone; the other's five stand");

        drop(record);
        std::thread::sleep(Duration::from_millis(10));
    }
}
