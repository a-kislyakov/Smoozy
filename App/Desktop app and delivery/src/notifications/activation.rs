//! **What clicking a notification does** — `Desktop app and delivery` §13.7,
//! T28, T29 and T30.
//!
//! **What travels with the post is identities only** — never state, never a
//! path, and never a message re-derived at click time (T29). On activation the
//! application brings its window forward and goes to that subject's own
//! record, **which exists whatever became of the notification** because every
//! one of the four kinds also leaves its own durable in-app record (T22).
//! **That is what makes this a routing question rather than a storage one.**
//!
//! **The width is the running application** (T30). The spec's own bound is
//! that a notification reaches the user while Smoozy runs behind their work
//! and never once they have quit it, so **nothing here is built on a cold
//! start**, and §22 item 51 carries the residue: macOS documents delivery to
//! an exited application, the shell's Windows path from a click on a closed
//! application to its callback is established by no publication, and Linux is
//! undefined by publication.
//!
//! **T22 and T23 stand untouched.** Nothing is advanced, cleared, acknowledged
//! or recorded by the activation, and no state depends on it.

use std::collections::HashMap;

use smoozy_state::identities::{Identity, IdentityKind};
use tauri::{AppHandle, Manager, Runtime};

use crate::events::{Event, Posted, Said, Subject, TheOneQueue};
use crate::presentation::the_presentation_key::ThePresentationKey;
use crate::typography::Verbatim;

/// **The user clicked a notification and Smoozy is going to what it is
/// about.**
///
/// The kind is declared beside the service that posts it, which is the command
/// surface's own rule. **`web/a-notification-clicked.js` listens for it** and
/// goes where [`WhereAClickGoes`] says: the job's History entry, its Queue
/// entry, or the server. *(Until 2026-11-01 no page listened, and a click
/// brought Smoozy forward and went nowhere — the Mac watch of that day.)*
pub const THE_USER_CLICKED_A_NOTIFICATION: &str = "notifications:a-going";

/// **Where a click goes, read from the record at the moment of the click** —
/// T29's *"the job's history entry, the server's row"*.
///
/// **It is read, never remembered.** The register holds the subject alone;
/// which screen holds that subject now is the record's, so a job deleted since
/// the post, or a server removed, reads as nowhere and the click gets the
/// bringing-forward alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereAClickGoes {
    /// **The job's History entry** — a job whose render start was learned, or
    /// whose server went before it was heard (`Queue & History` C9's two ways
    /// in). Every ending the four kinds speak of is one of these.
    TheHistoryEntry(Identity),
    /// **The job's Queue entry** — a job the record holds that History does
    /// not, such as one that failed before its render start was learned.
    TheQueueEntry(Identity),
    /// **The server**, by its row — the connection kind and the idle kind.
    TheServer(Identity),
}

impl WhereAClickGoes {
    /// The identity of the thing gone to.
    pub fn the_destination(&self) -> &Identity {
        match self {
            WhereAClickGoes::TheHistoryEntry(job)
            | WhereAClickGoes::TheQueueEntry(job)
            | WhereAClickGoes::TheServer(job) => job,
        }
    }

    /// The word the page goes by — `web/a-notification-clicked.js`.
    pub fn the_word(&self) -> &'static str {
        match self {
            WhereAClickGoes::TheHistoryEntry(_) => "the-history",
            WhereAClickGoes::TheQueueEntry(_) => "the-queue",
            WhereAClickGoes::TheServer(_) => "the-server",
        }
    }
}

/// **T29's destination for one subject, read from the record now.**
///
/// - **A job** goes to its History entry where History holds it, and to its
///   Queue entry where only the record does.
/// - **A group** goes to one member's entry, by the same rule — **a finished
///   member first**, because the group's one notification says the batch
///   finished (T27).
/// - **A server row** goes to the server.
/// - **Anything else, or a subject the record no longer holds, goes nowhere.**
pub fn where_a_click_goes(
    connection: &rusqlite::Connection,
    subject: &Identity,
) -> Option<WhereAClickGoes> {
    use crate::record::job_records::JobRecords;
    match subject.kind() {
        IdentityKind::Job => the_entry_among(connection, std::slice::from_ref(subject)),
        IdentityKind::Group => {
            let mut members = JobRecords::over(connection).in_the_group(subject).ok()?;
            members.sort_by_key(|member| {
                member.the_ending_learned() != Some(smoozy_queue_history::ending::Ending::Finished)
            });
            let jobs: Vec<Identity> = members.into_iter().map(|member| member.job).collect();
            the_entry_among(connection, &jobs)
        }
        IdentityKind::LocalServer => crate::record::server_rows::one(connection, &subject.as_text())
            .ok()
            .flatten()
            .map(|_| WhereAClickGoes::TheServer(subject.clone())),
        _ => None,
    }
}

/// The first of `jobs` History holds, else the first the record holds.
fn the_entry_among(connection: &rusqlite::Connection, jobs: &[Identity]) -> Option<WhereAClickGoes> {
    use crate::record::job_records::JobRecords;
    let records = JobRecords::over(connection);
    if let Ok(history) = records.in_history() {
        if let Some(job) = jobs
            .iter()
            .find(|job| history.entries.iter().any(|entry| &entry.record.job == *job))
        {
            return Some(WhereAClickGoes::TheHistoryEntry(job.clone()));
        }
    }
    jobs.iter()
        .find(|job| matches!(records.read(job), Ok(Some(_))))
        .map(|job| WhereAClickGoes::TheQueueEntry(job.clone()))
}

/// **What travels with a post: identities and nothing else** — T29.
///
/// The job identity or group identity for the two job kinds, the server row
/// for the connection kind and the idle kind. **Never state, never a path,
/// never a message re-derived at click time.**
///
/// It is a type rather than a convention so that the rule is kept by what can
/// be put in it: there is no field a sentence, a path or a reading could go in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTravels {
    /// **The durable subject the notification is about.**
    pub subject: Identity,
}

/// **What the systems hand back on a click, and what it is looked up in.**
///
/// **All three hand back the post's own identity and an action**, normalised
/// by each of the three routes to one thing here (T28): macOS calls a
/// notification-centre delegate with a response object, Windows raises the
/// toast's own activation event in the running application (no component
/// server is registered — T28's correction of 2026-11-01), and Linux emits an
/// action-invoked signal to the client that posted. **A bare click on the notification body is itself
/// an action on all three.**
///
/// **The register is run-held and holds identities only.** It is not a store:
/// nothing durable is written, a fresh launch holds none of it, and losing it
/// costs the going and nothing else — which is T22 and T30 together, the going
/// being all a click ever promised.
#[derive(Default)]
pub struct WhatEachPostIsAbout {
    per_post: HashMap<String, WhatTravels>,
}

impl WhatEachPostIsAbout {
    pub fn new() -> Self {
        Self::default()
    }

    /// Remembered at the moment the post is attempted, **about `goes_to`** —
    /// the key's own subject for three of the four kinds, and **the server row
    /// for the connection kind**, whose key is its condition identity (T29:
    /// *"the server row for the connection kind and the idle kind"*).
    pub fn a_post_was_attempted(&mut self, key: &ThePresentationKey, goes_to: &Identity) {
        self.per_post.insert(
            key.as_text(),
            WhatTravels {
                subject: goes_to.clone(),
            },
        );
    }

    /// **Forgotten when the post is taken down** (T24a), so a click on a
    /// notification the system did not manage to remove finds nothing and gets
    /// the bringing-forward alone.
    pub fn a_post_was_taken_down(&mut self, key: &ThePresentationKey) {
        self.per_post.remove(&key.as_text());
    }

    /// What a post identity handed back by the system is about, where this run
    /// posted it.
    pub fn what_this_post_is_about(&self, the_post_identity: &str) -> Option<&WhatTravels> {
        self.per_post.get(the_post_identity)
    }
}

/// **T29, performed: bring the window forward, then go to the subject.**
///
/// **The bringing-forward happens whatever the register says.** A subject that
/// is gone by the time of the click — or a post this run did not make, which
/// is T30's cold-start case arriving by a different door — **gets the
/// bringing-forward and nothing else**, no error and no invented screen,
/// because the click's promise is the going and a gone subject has nowhere to
/// go.
///
/// **The going itself is a read rather than a write.** P7 forbids an event
/// carrying state that has not already been written durably, and what is read
/// here is the register's own record of what this run posted, then **the
/// record, for where that subject is now** ([`where_a_click_goes`]).
/// **Nothing is composed from the notification's own text**, which is T29's
/// *never a message re-derived at click time*.
///
/// **A record not yet opened is a going to nowhere** — a click that arrives
/// during launch, before the record is in the application's hands, gets the
/// bringing-forward alone.
pub fn a_notification_was_clicked<R: Runtime>(
    app: &AppHandle<R>,
    queue: &TheOneQueue,
    travels: Option<WhatTravels>,
) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
    let _ = the_going(queue, travels, |subject| {
        let started = app.try_state::<std::sync::Mutex<crate::start::Started>>()?;
        let started = started.lock().ok()?;
        where_a_click_goes(started.record.as_ref()?.connection(), subject)
    });
}

/// The going, on the one ordered queue — P6.
///
/// **T29's last sentence is kept by the read**: a subject the record no longer
/// holds goes nowhere and nothing is posted. **The destination is told to the
/// queue as existing before the post**, because the record has just said so
/// and P8 would otherwise discard an event about a job no screen had yet
/// named to the queue.
fn the_going(
    queue: &TheOneQueue,
    travels: Option<WhatTravels>,
    where_it_goes: impl FnOnce(&Identity) -> Option<WhereAClickGoes>,
) -> Result<Posted, ()> {
    let travels = travels.ok_or(())?;
    let Some(going) = where_it_goes(&travels.subject) else {
        return Ok(Posted::TheSubjectIsGone);
    };
    queue.the_subject_exists(going.the_destination().clone());
    let field = match going {
        WhereAClickGoes::TheServer(_) => "server",
        _ => "job",
    };
    queue.read_then_post(
        || Ok::<_, ()>(going),
        |going| {
            Event::about(
                Subject::Durable(going.the_destination().clone()),
                THE_USER_CLICKED_A_NOTIFICATION,
            )
            // An identity is a value, not prose — X7 and X9. It is what the
            // page matches a record on and nothing about it is readable.
            .saying("goesTo", Said::Value(Verbatim::new(going.the_word())))
            .saying(
                field,
                Said::Value(Verbatim::new(going.the_destination().as_text())),
            )
        },
    )
}

/// **T30: nothing is built on a cold start.**
///
/// macOS documents delivery to an exited application and the shell assigns its
/// delegate during launch, so such an activation is met as an ordinary launch
/// and routed under T29. **Windows has the mechanism by publication but the
/// path from a click on a closed application to a callback is established by
/// no publication and is treated as unproven. Linux is undefined by
/// publication** — the specification describes a signal to a connected client
/// and nothing else.
///
/// **So the register above is run-held on purpose rather than for want of a
/// store.** A post this run did not make is a post this run cannot route, and
/// that is the honest width of the promise rather than a gap to fill with a
/// durable table.
pub const NOTHING_IS_BUILT_ON_A_COLD_START: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    fn a_key() -> ThePresentationKey {
        ThePresentationKey::AJobsEnding(
            Identity::issue(IdentityKind::Job).expect("the desk has randomness"),
        )
    }

    /// **T29: identities only, and there is nowhere else for anything to go.**
    ///
    /// Read over this module's own source rather than asserted on a value,
    /// because the fault is a field somebody adds — a path so the click can
    /// open a folder, a sentence so the page need not look one up — and the
    /// type is the whole of what forbids it.
    #[test]
    fn nothing_but_an_identity_travels_with_a_post() {
        let source = include_str!("activation.rs");
        let struct_at = source
            .find("pub struct WhatTravels {")
            .expect("WhatTravels is a struct");
        let body = &source[struct_at..];
        let body = &body[..body.find('}').expect("the struct closes")];
        let fields: Vec<&str> = body
            .lines()
            .skip(1)
            .filter(|line| line.trim_start().starts_with("pub "))
            .collect();
        assert_eq!(
            fields.len(),
            1,
            "T29: what travels with the post is identities only — never state, never a path and \
             never a message re-derived at click time; WhatTravels has grown a second field: \
             {fields:?}"
        );
        assert!(fields[0].contains("Identity"));
    }

    /// The register answers for a post this run made, and for nothing else.
    #[test]
    fn a_post_this_run_did_not_make_is_about_nothing() {
        let mut register = WhatEachPostIsAbout::new();
        let key = a_key();
        register.a_post_was_attempted(&key, key.the_subject());
        assert!(register.what_this_post_is_about(&key.as_text()).is_some());
        assert!(register
            .what_this_post_is_about("job:0123456789abcdef0123456789abcdef")
            .is_none());
    }

    /// **T24a's other end: a taken-down post is forgotten**, so a click on one
    /// the system did not manage to remove routes nowhere rather than to a
    /// record whose reason for existing has ended.
    #[test]
    fn a_taken_down_post_is_forgotten() {
        let mut register = WhatEachPostIsAbout::new();
        let key = a_key();
        register.a_post_was_attempted(&key, key.the_subject());
        register.a_post_was_taken_down(&key);
        assert!(register.what_this_post_is_about(&key.as_text()).is_none());
    }

    /// **T29: the connection kind goes to the server row**, though its key is
    /// its condition identity.
    #[test]
    fn the_connection_kind_travels_with_its_server_row() {
        let mut register = WhatEachPostIsAbout::new();
        let condition = Identity::issue(IdentityKind::Condition).expect("the desk has randomness");
        let server = Identity::issue(IdentityKind::LocalServer).expect("the desk has randomness");
        let key = ThePresentationKey::ACondition(condition);
        register.a_post_was_attempted(&key, &server);
        assert_eq!(
            register.what_this_post_is_about(&key.as_text()),
            Some(&WhatTravels { subject: server })
        );
    }

    /// **The Mac watch of 2026-11-01, finding 1: a click brought Smoozy
    /// forward and went nowhere.** Each kind's subject reads, from a real
    /// record, as the place a click goes now — and a subject the record no
    /// longer holds reads as nowhere (T29's last sentence).
    #[test]
    fn a_click_goes_to_the_entry_or_the_server_the_record_holds_now() {
        use crate::record::deletions::tests::a_record_in_history;
        use crate::record::job_records::JobRecords;
        use crate::servers::the_machines_work::tests::{a_record_with_a_server, it_ended};
        use smoozy_queue_history::acting::note::TheOutcomeAsWritten;

        let (record, server) = a_record_with_a_server();
        let connection = &record.0;
        let write = |made: &smoozy_queue_history::record::LocalJobRecord| {
            JobRecords::over(connection)
                .write_the_provisional_record(made)
                .expect("the record takes it");
        };

        // **A finished job is in History.**
        let finished = a_record_in_history(&server, "Kitchen");
        write(&finished);
        it_ended(&record, &finished.job, TheOutcomeAsWritten::Finished, 60, None);
        assert_eq!(
            where_a_click_goes(connection, &finished.job),
            Some(WhereAClickGoes::TheHistoryEntry(finished.job.clone()))
        );

        // **A job whose render start was never learned is the Queue's.**
        let mut early = a_record_in_history(&server, "No camera");
        early.the_render_start_has_been_learned = false;
        write(&early);
        assert_eq!(
            where_a_click_goes(connection, &early.job),
            Some(WhereAClickGoes::TheQueueEntry(early.job.clone()))
        );

        // **A group goes to a finished member's entry**, whatever order the
        // record keeps them in.
        let group = Identity::issue(IdentityKind::Group).expect("the desk has randomness");
        let mut failed_member = a_record_in_history(&server, "Spins");
        failed_member.group = Some(group.clone());
        write(&failed_member);
        it_ended(&record, &failed_member.job, TheOutcomeAsWritten::Failed, 60, Some(3));
        let mut finished_member = a_record_in_history(&server, "Spins");
        finished_member.group = Some(group.clone());
        write(&finished_member);
        it_ended(&record, &finished_member.job, TheOutcomeAsWritten::Finished, 60, None);
        assert_eq!(
            where_a_click_goes(connection, &group),
            Some(WhereAClickGoes::TheHistoryEntry(finished_member.job.clone()))
        );

        // **The connection kind and the idle kind go to the server.**
        assert_eq!(
            where_a_click_goes(connection, &server),
            Some(WhereAClickGoes::TheServer(server.clone()))
        );

        // **Gone, or never there, is nowhere.**
        JobRecords::over(connection)
            .remove(&finished.job)
            .expect("the record lets it go");
        assert_eq!(where_a_click_goes(connection, &finished.job), None);
        let never = Identity::issue(IdentityKind::LocalServer).expect("the desk has randomness");
        assert_eq!(where_a_click_goes(connection, &never), None);
        let a_condition = Identity::issue(IdentityKind::Condition).expect("the desk has randomness");
        assert_eq!(where_a_click_goes(connection, &a_condition), None);
    }

    /// **The going is posted on the one queue, saying where and what** —
    /// the two words `web/a-notification-clicked.js` reads — even for a job
    /// no screen has named to the queue before.
    #[test]
    fn the_going_says_where_and_what_and_reaches_the_page() {
        use crate::events::{Delivers, Delivery};
        use std::sync::{Arc, Mutex};

        struct ThePage(Arc<Mutex<Vec<Delivery>>>);
        impl Delivers for ThePage {
            fn deliver(&self, delivery: &Delivery) {
                self.0.lock().unwrap().push(delivery.clone());
            }
        }

        let queue = TheOneQueue::new();
        let got = Arc::new(Mutex::new(Vec::new()));
        queue.the_page_is_ready(Box::new(ThePage(got.clone())));

        let job = Identity::issue(IdentityKind::Job).expect("the desk has randomness");
        let travels = WhatTravels { subject: job.clone() };
        assert_eq!(
            the_going(&queue, Some(travels), |_| Some(WhereAClickGoes::TheHistoryEntry(job.clone()))),
            Ok(Posted::InOrder(0))
        );
        let server = Identity::issue(IdentityKind::LocalServer).expect("the desk has randomness");
        let travels = WhatTravels { subject: server.clone() };
        assert_eq!(
            the_going(&queue, Some(travels), |_| Some(WhereAClickGoes::TheServer(server.clone()))),
            Ok(Posted::InOrder(1))
        );

        let got = got.lock().unwrap();
        assert_eq!(got.len(), 2);
        let said = |at: usize, name: &str| match got[at].said.get(name) {
            Some(Said::Value(value)) => value.as_str().to_string(),
            _ => String::new(),
        };
        assert_eq!(got[0].kind, THE_USER_CLICKED_A_NOTIFICATION);
        assert_eq!(said(0, "goesTo"), "the-history");
        assert_eq!(said(0, "job"), job.as_text());
        assert_eq!(said(1, "goesTo"), "the-server");
        assert_eq!(said(1, "server"), server.as_text());
    }

    /// **T29's last sentence, without a window: a subject the record no
    /// longer holds posts nothing**, and the user gets the bringing-forward
    /// alone.
    #[test]
    fn a_going_about_a_subject_the_record_no_longer_holds_is_nothing() {
        let queue = TheOneQueue::new();
        let travels = WhatTravels {
            subject: a_key().the_subject().clone(),
        };
        assert_eq!(
            the_going(&queue, Some(travels), |_| None),
            Ok(Posted::TheSubjectIsGone)
        );
    }

    /// A click on a post nothing in this run made composes no event at all —
    /// the reading door refuses and there is nothing to compose from.
    #[test]
    fn an_unknown_post_composes_no_event() {
        let queue = TheOneQueue::new();
        assert_eq!(the_going(&queue, None, |_| None), Err(()));
        assert!(NOTHING_IS_BUILT_ON_A_COLD_START);
    }

    /// **Something on the page listens for the going** — the Mac watch's
    /// first finding was that nothing did, so a click brought Smoozy forward
    /// and stayed on whatever page was showing.
    #[test]
    fn a_page_module_listens_for_the_going() {
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        let listening: Vec<String> = std::fs::read_dir(&web)
            .expect("the page folder is readable")
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().and_then(|e| e.to_str()) == Some("js"))
            .filter(|entry| {
                std::fs::read_to_string(entry.path())
                    .map(|text| {
                        text.contains(&format!("\"{THE_USER_CLICKED_A_NOTIFICATION}\""))
                            && text.contains("whenTold(")
                    })
                    .unwrap_or(false)
            })
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            listening,
            vec!["a-notification-clicked.js".to_string()],
            "T29: a click goes to the job or the server it is about, and that \
             needs exactly one page module listening for `{THE_USER_CLICKED_A_NOTIFICATION}`"
        );
    }

    /// **The page goes where the going says, and nowhere for anything else**
    /// — `web/a-notification-clicked.js` run under Node with the three
    /// screens stood in for. **Skipped, and says so, where no Node is
    /// installed.**
    #[test]
    fn the_page_goes_where_the_click_leads() {
        use std::fs;
        let folder = std::env::temp_dir().join(format!("smoozy-a-click-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        fs::copy(web.join("a-notification-clicked.js"), folder.join("a-notification-clicked.js"))
            .expect("the page is copied");
        let calls = "(globalThis.calls ||= []).push";
        for (name, text) in [
            ("package.json", r#"{ "type": "module" }"#.to_string()),
            (
                "event-route.js",
                "export function whenTold(kind, handler) { ((globalThis.told ||= {})[kind] ||= []).push(handler); }"
                    .to_string(),
            ),
            (
                "the-rail.js",
                format!(
                    "export const A_SERVER = 'machineview'; \
                     export function theCurrentPlaceIs(...args) {{ {calls}(['place', ...args]); }}"
                ),
            ),
            (
                "the-history.js",
                format!("export function theHistoryEntryIsOpened(...args) {{ {calls}(['history', ...args]); }}"),
            ),
            (
                "the-queue.js",
                format!("export function theQueueEntryIsOpened(...args) {{ {calls}(['queue', ...args]); }}"),
            ),
        ] {
            fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        let harness = format!(
            r#"
await import("./a-notification-clicked.js");
const tell = (said) => globalThis.told["{THE_USER_CLICKED_A_NOTIFICATION}"].forEach((handler) => handler({{ said }}));
tell({{ goesTo: "the-history", job: "j1" }});
tell({{ goesTo: "the-queue", job: "j2" }});
tell({{ goesTo: "the-server", server: "s1" }});
tell({{ goesTo: "the-history" }});
tell({{ goesTo: "somewhere-else", job: "j3" }});
console.log(JSON.stringify(globalThis.calls));
"#
        );
        fs::write(folder.join("harness.mjs"), harness).expect("the harness is written");
        let ran = std::process::Command::new("node")
            .arg(folder.join("harness.mjs"))
            .current_dir(&folder)
            .output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so web/a-notification-clicked.js was not run");
            return;
        };
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("one answer");
        assert_eq!(
            got,
            serde_json::json!([
                ["history", "j1"],
                ["queue", "j2"],
                ["place", "machineview", "s1"],
            ])
        );
    }
}
