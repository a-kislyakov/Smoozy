//! **A wrong key, refused by a real OpenSSH server on loopback, and what the
//! user is told about it.** Written 2026-10-03 for the Creator's note 14 on the
//! test copy: *"Deliberately chose the wrong key. It didn't connect, didn't
//! tell me that anything is wrong, nothing."*
//!
//! **What the far end is.** OpenSSH's own `sshd`, run by this test as the user
//! running it, one `sshd -i` per connection on a loopback socket, holding one
//! key in its authorised list. **The desk offers a different one**, through
//! the application's own transport (`Servers` K1a) and `Servers`' own
//! sequence, exactly as a user's connect does — so the refusal is the one a
//! rental gives, not a stand-in for it.
//!
//! **What it proves.** The attempt stops at the machine refusing the
//! credential, which is `Servers` S10 item 5 — the connection never became
//! live — and **the answer names the server**, so the page can say it where the
//! user is: *Could not connect to Hyperstack.* above the All servers
//! list, and the sentence the server's Set up card says. **Before the fix
//! (proved 2026-10-03 with this file's first form) the answer named no
//! server**, and the page wrote it only above the hidden All servers list.
//!
//! **Nothing of the Creator's is touched.** The keys and the server's
//! configuration are in a throwaway folder made for the test and deleted with
//! it, and every server is a child of the test.
//!
//! **How a clause marker is written in this crate.** A marker with no document
//! beside it means a clause of this crate's own plan, `Preparation/Architecture/
//! Desktop app and delivery.md`; every other document's marker is written out
//! with that document at its site.

#![cfg(unix)]

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use smoozy_desktop::events::{Delivers, Delivery, Said, TheOneQueue};
use smoozy_desktop::servers::a_period::{what_the_driving_left, WhatTheDrivingLeft};
use smoozy_desktop::surface::servers::{
    the_answer_about, what_a_failed_connect_said, what_an_ended_period_says, TheAnswer,
    TheServerItWasAbout,
};
use smoozy_desktop::typography::Prose;
use smoozy_server_protocol::transport::Credential;
use smoozy_servers::acting::{
    SupplyTheCredential, TheActingHalf, TheIdentityRead, TheInstallationIdentityRead,
    TheServerBeingConnectedTo, WhatServersPublishes,
};
use smoozy_servers::condition::ObservedValue;
use smoozy_servers::driving::HowTheDrivingEnded;
use smoozy_servers::first_connection::ChangedIdentityWarning;
use smoozy_servers::identity::{Host, KeyReference, Port, Username};
use smoozy_servers::key_moments::{TheTwoAnswers, WhoStartedIt};
use smoozy_servers::repair::{NextAttemptDue, PermanentRefusal, WhatTheRecordSays};
use smoozy_servers::sequence::{self, AnAttempt, Attempt, ConnectionRole};
use smoozy_servers::store::{RememberedHostKey, ServerRow};
use smoozy_servers::wiring::{TheDesktopsClock, TheTransport};
use smoozy_state::identities::InstallationCheck;

fn openssh() -> Option<PathBuf> {
    ["/usr/sbin/sshd", "/usr/local/sbin/sshd"]
        .iter()
        .map(PathBuf::from)
        .find(|path| path.exists())
}

struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn a_key(at: &std::path::Path, named: &str) -> Option<PathBuf> {
    let made = std::process::Command::new("ssh-keygen")
        .args(["-q", "-t", "ed25519", "-N", "", "-C", named, "-f"])
        .arg(at.join(named))
        .status()
        .ok()?;
    made.success().then(|| at.join(named))
}

struct TheFarEnd {
    port: u16,
    chosen: PathBuf,
    _scratch: Scratch,
}

fn the_far_end() -> Option<TheFarEnd> {
    let sshd = openssh()?;
    let folder = std::env::temp_dir().join(format!(
        "smoozy-wrong-key-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|since| since.as_nanos())
            .unwrap_or(0)
    ));
    std::fs::create_dir_all(&folder).ok()?;
    let scratch = Scratch(folder.clone());
    let host = a_key(&folder, "host")?;
    let held = a_key(&folder, "held")?;
    let chosen = a_key(&folder, "chosen")?;
    std::fs::copy(held.with_extension("pub"), folder.join("authorized_keys")).ok()?;
    let config = folder.join("sshd_config");
    std::fs::write(
        &config,
        format!(
            "HostKey {}\nAuthorizedKeysFile {}\nPidFile none\nUsePAM no\nStrictModes no\n\
             PasswordAuthentication no\nKbdInteractiveAuthentication no\n",
            host.display(),
            folder.join("authorized_keys").display(),
        ),
    )
    .ok()?;
    let checked = std::process::Command::new(&sshd)
        .args(["-t", "-f"])
        .arg(&config)
        .output()
        .ok()?;
    if !checked.status.success() {
        println!("sshd will not run here: {}", String::from_utf8_lossy(&checked.stderr).trim());
        return None;
    }
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).ok()?;
    let port = listener.local_addr().ok()?.port();
    let log = folder.join("sshd.log");
    std::thread::spawn(move || {
        for socket in listener.incoming().flatten() {
            let Ok(writing) = socket.try_clone() else { continue };
            let reading = std::process::Stdio::from(std::os::fd::OwnedFd::from(socket));
            let writing = std::process::Stdio::from(std::os::fd::OwnedFd::from(writing));
            let errors = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log)
                .map(std::process::Stdio::from)
                .unwrap_or_else(|_| std::process::Stdio::null());
            if let Ok(mut child) = std::process::Command::new(&sshd)
                .args(["-i", "-e", "-f"])
                .arg(&config)
                .stdin(reading)
                .stdout(writing)
                .stderr(errors)
                .spawn()
            {
                let _ = child.wait();
            }
        }
    });
    Some(TheFarEnd {
        port,
        chosen,
        _scratch: scratch,
    })
}

struct TheChosenKey(PathBuf);

impl SupplyTheCredential for TheChosenKey {
    fn the_two_answers(&mut self, _key: &KeyReference) -> TheTwoAnswers {
        TheTwoAnswers {
            the_key_is_usable: true,
            a_passphrase_is_available_without_asking: true,
        }
    }

    fn for_one_attempt(
        &mut self,
        _key: &KeyReference,
        _who: WhoStartedIt,
        _naming: TheServerBeingConnectedTo<'_>,
    ) -> Option<Credential> {
        Some(Credential {
            private_key_file: self.0.clone(),
            passphrase: None,
        })
    }

    fn the_refusal_is_permanent(&mut self, _key: &KeyReference) -> PermanentRefusal {
        PermanentRefusal {
            the_store_standing_reads_never_answered: false,
            the_rows_key_is_protected: false,
        }
    }
}

struct NeverRead;

impl<C: Send> TheInstallationIdentityRead<C> for NeverRead {
    async fn read(&mut self, _channel: &mut C) -> TheIdentityRead {
        TheIdentityRead {
            check: InstallationCheck::Absent,
            identity_on_the_machine: None,
        }
    }
}

struct Nothing;

impl WhatServersPublishes for Nothing {
    fn the_observed_value(&mut self, _server: &str, _value: ObservedValue) {}
    fn the_next_attempt_due(&mut self, _server: &str, _reading: NextAttemptDue) {}
    fn raise_the_connection_condition(&mut self, _server: &str, _says: WhatTheRecordSays) {}
    fn withdraw_the_connection_condition(&mut self, _server: &str) {}
    fn the_changed_identity_warning(&mut self, _server: &str) {}
    fn record_the_scanned_host_key(&mut self, _server: &str, _key: RememberedHostKey) {}
}

struct Held(Arc<Mutex<Vec<Delivery>>>);

impl Delivers for Held {
    fn deliver(&self, delivery: &Delivery) {
        if let Ok(mut held) = self.0.lock() {
            held.push(delivery.clone());
        }
    }
}

fn as_the_page_reads_it(event: smoozy_desktop::events::Event) -> serde_json::Value {
    let queue = TheOneQueue::new();
    let held = Arc::new(Mutex::new(Vec::new()));
    queue.the_page_is_ready(Box::new(Held(Arc::clone(&held))));
    let _ = queue.read_then_post::<(), std::convert::Infallible>(|| Ok(()), |_| event.clone());
    let delivery = held.lock().expect("held").pop().expect("delivered");
    let mut said = serde_json::Map::new();
    for (name, one) in &delivery.said {
        let text = match one {
            Said::Prose(prose) => prose.as_str().to_string(),
            Said::Value(value) => value.as_str().to_string(),
        };
        said.insert(name.to_string(), serde_json::Value::String(text));
    }
    serde_json::json!({ "kind": delivery.kind, "subject": delivery.subject, "said": said })
}

#[test]
fn a_wrong_key_refused_on_loopback_is_answered_naming_its_server() {
    let Some(far_end) = the_far_end() else {
        println!("SKIPPED: no OpenSSH sshd and ssh-keygen this test can run here");
        return;
    };
    let user = std::env::var("USER").expect("a user");
    let row = ServerRow::saved(
        "server-hyperstack".to_string(),
        Host::what_the_user_typed("127.0.0.1").expect("a host"),
        Port::what_the_user_typed(far_end.port).expect("a port"),
        Username::what_the_user_typed(&user).expect("a username"),
        KeyReference {
            local_key_identifier: "the-chosen-key".to_string(),
            fingerprint: String::new(),
        },
    );
    let mut half = TheActingHalf {
        opener: TheTransport,
        credentials: TheChosenKey(far_end.chosen.clone()),
        installation: NeverRead,
        publishes: Nothing,
        wait: TheDesktopsClock,
    };
    let attempt = AnAttempt {
        row: &row,
        who: WhoStartedIt::TheUser,
        the_first_connection_to_this_server: true,
        role: ConnectionRole::Primary,
        deadline: Duration::from_secs(20),
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("a runtime");
    let mut warning = ChangedIdentityWarning::none_yet();
    let went = runtime.block_on(sequence::run(attempt, &mut half, &mut warning, &mut |_: &mut Nothing| {}));
    let Attempt::ItStopped(stopped) = went else {
        panic!("the machine took a key it does not hold");
    };
    println!("the attempt stopped at: {stopped:?}");
    let left = what_the_driving_left(HowTheDrivingEnded::TheOpeningAttemptStopped(stopped));
    assert_eq!(left, WhatTheDrivingLeft::TheConnectionWasNotMade);
    let answer = what_an_ended_period_says(left, false, None).expect("something is said");
    assert_eq!(answer, TheAnswer::TheConnectionWasNotMade);

    // **The answer names its server**, as the page receives it.
    let page = as_the_page_reads_it(the_answer_about(
        answer,
        &TheServerItWasAbout {
            server: row.local_server_identifier.clone(),
            name: Some("Hyperstack".to_string()),
        },
    ));
    println!("THE-ANSWER {page}");
    let said = &page["said"];
    assert_eq!(said["server"], "server-hyperstack", "the answer names its server: {page}");
    assert_eq!(said["couldNotConnectTo"], Prose::new("Could not connect to").as_str());
    assert_eq!(said["couldNotConnectToNamed"], "Hyperstack");
    // **The Set up card keeps *this server***, which is the one on screen.
    assert_eq!(said["said"], Prose::new("Could not connect to this server.").as_str());
    let what = said["what"].as_str().expect("what it was");
    assert_eq!(
        what_a_failed_connect_said(what),
        Some("Could not connect to this server."),
        "the Set up tab, handed the answer back, says the same sentence"
    );
}
