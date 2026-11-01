//! This module's diary declaration — `Servers` §15.
//!
//! The rules are `Reporting a problem` **J2, J2a, J3, J4 and J4a**, taken as
//! they stand. **The list below is fixed** — what is not on it is not written —
//! and **nothing on it is a new act**: every one is an act this document already
//! performs.
//!
//! **Lines are written through the diary service `Desktop app and delivery`
//! §5.3 L14 names**, whose hosting rule — a module declares or writes nothing —
//! is that document's A7.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Reporting a problem` J2, J2a, J3, J4, J4a,
//! J6, U7. `Desktop app and delivery` A7, L14. `Queue & History` A1.
//! `Server protocol and transfers` F2.
//!
//! **`Queue & History` A5 is deliberately not listed above and is written out
//! with its document at the site**, because this plan defines an A5 of its own
//! and a bare one could mean either.

use smoozy_reporting::lines::{Line, LineKind};
use smoozy_state::identities::{Identity, IdentityKind};
use smoozy_state::observed::{Clock, Reading};
use std::time::SystemTime;

use crate::hardware::{card_count, Cards, FreeSpace, HardwareReading, MachineMemory};
use crate::period::OpeningAct;
use crate::store::TheErase;
use crate::work::StopAndClearResult;

/// **The two kinds this module declares**, and no others. **A7 makes a module
/// with no declaration write nothing, silently**, so a declaration that exists
/// only in this file's tests is a module that writes nothing.
pub const DECLARED: [LineKind; 2] = [LineKind::Act, LineKind::Told];

/// **What no line of this module's ever carries.**
///
/// - **The remembered host key or any fingerprint of one, barred outright**
///   (D9; `Reporting a problem` N2). `Security and licence` §7's closing
///   sentence is the rule —
///   *"nothing here reaches the user's eyes. No fingerprint at a first
///   connection, none at a changed one, none in the warning, and none in any
///   log"*.
/// - **Anything about a passphrase or a machine password**, no plaintext of
///   which any module here ever receives; an erase enters **as confirmed or
///   owed and never as a value** (`Reporting a problem` N2).
/// - **Anything about a history record**, which a removal never reaches (D21).
/// - **Anything read from the machine to compose a removal line**, which is
///   D19's rule from this side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NeverOnALine;

/// **Act one: a server connected.**
///
/// J2 names *connected* among the spec's own five and gives it to the module
/// whose spec promises it. It passes J2a's three tests. It is **Smoozy's own
/// act** — a connection opened on a transport Smoozy itself drives (K1), never
/// a reading or an ending that came back from a machine. **The owning spec names
/// its completion as something the user sees afterwards** — the row reads
/// **connected** for as long as its primary connection is live. And it runs **at
/// the user's own rate**: **one line per period-opening act, and that list is
/// closed at two.**
///
/// **When the line is written:** the first moment the observed value reaches
/// **connected** under the period that act opened. **Not at authentication** —
/// the not-Linux check's outcomes 2 and 3 abandon the sequence after it and S10
/// item 5 ends the period with no connection ever live, **so a line written at
/// B2's instant would say *connected* of a server that reads disconnected**.
///
/// **What the line does not carry: no second line for a connection under a
/// period that already stands** — a repair attempt and its return, the return
/// after a restart, and the additional connection each write nothing, **because
/// none is an act the user asked for and a standing repair runs for as long as
/// the app is open**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AServerConnected {
    /// **Which opening act it was** — the user's connect, or an add carrying a
    /// ticked setup option, **the edit put-back being O1 performed inside the
    /// user's own save**.
    pub opening_act: OpeningAct,
    /// **The local server identifier, with the row's label beside it and never
    /// in place of it.**
    pub local_server_identifier: String,
    pub label: Option<String>,
    /// **The row's host, port and username**, which together name the machine
    /// the act reached and which `Reporting a problem` §4's N3 **admits
    /// deliberately** — its N4 naming in terms the standing rule that must not
    /// be applied to delete them.
    pub host: String,
    pub port: u16,
    pub username: String,
}

/// **Act two: a server removed.** Row 41's *"Removing a server makes Smoozy
/// forget it"*.
///
/// It passes the three tests: it is Smoozy's own act, taken on the user's word
/// and reaching a machine only where they asked for that; the spec names its
/// completion as something the user sees afterwards — the row is gone from the
/// list; and **it is one line per removal, never one per unit of W25's three**.
///
/// **When the line is written:** at the **local forget**, which is the removal's
/// last unit and runs only on the user's word, and with which the period ends.
/// **So the line records the act completed and never the act asked for**, which
/// is what W21's *try again* and *try the remainder again* leave standing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AServerRemoved {
    /// **Which removal it was** — the one that reaches the machine, *stop and
    /// clear first*, or one that reaches none.
    pub reached_the_machine: bool,
    /// **Where it reached the machine, W21's result as one of that table's three
    /// declared values, read from the durable outcome record and never composed
    /// afresh.**
    pub result: Option<StopAndClearResult>,
    /// **Whether the machine password's erase was confirmed or left as
    /// `Security and licence` §5.1's recorded debt** — **the one leftover
    /// nothing afterwards would ever show.**
    pub the_erase: Option<TheErase>,
    /// **The local server identifier of the row removed, with its label beside
    /// it.**
    pub local_server_identifier: String,
    pub label: Option<String>,
    /// **The row's host and username as it held them at the forget** — the row
    /// itself is gone, **so without them nothing afterwards could name what was
    /// removed.**
    pub host: String,
    pub username: String,
}

/// **The told line, and there is one: a hardware reading.**
///
/// §8's reading **came back from a machine**, so it is J4a's kind and not an
/// act's, and **it is a reading the user could name** — it is what Row 349 puts
/// in front of them. **It is written where a fresh reading is taken and
/// written**, which is H18's three branches that take one, and a reading is
/// taken **once per primary connection and never on a schedule**. **So it is one
/// line per reading and never one per card, per figure or per mount**, which is
/// J4a's bound in this document's own terms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AHardwareReading {
    pub reading: HardwareReading,
    pub local_server_identifier: String,
    pub label: Option<String>,
}

/// The two acts, as one value, so a caller cannot write a third.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Act {
    AServerConnected(AServerConnected),
    AServerRemoved(AServerRemoved),
}

impl Act {
    pub fn name(&self) -> &'static str {
        match self {
            Act::AServerConnected(_) => "server connected",
            Act::AServerRemoved(_) => "server removed",
        }
    }

    /// **The fixed line form, filled with the declared values** — never free
    /// prose composed at run time (J3).
    fn text(&self) -> String {
        let mut fields = vec![self.name().to_string()];
        match self {
            Act::AServerConnected(connected) => {
                fields.push(
                    match connected.opening_act {
                        OpeningAct::TheUsersConnectAction => "the user's connect",
                        OpeningAct::AnAddWithASetupOptionTicked => {
                            "an add carrying a ticked setup option"
                        }
                    }
                    .to_string(),
                );
                fields.push(connected.local_server_identifier.clone());
                fields.push(label(connected.label.as_deref()));
                fields.push(format!(
                    "{}@{}:{}",
                    connected.username, connected.host, connected.port
                ));
            }
            Act::AServerRemoved(removed) => {
                fields.push(
                    if removed.reached_the_machine {
                        "stop and clear first"
                    } else {
                        "reaching no machine"
                    }
                    .to_string(),
                );
                if let Some(result) = removed.result {
                    fields.push(result_name(result).to_string());
                }
                fields.push(removed.local_server_identifier.clone());
                fields.push(label(removed.label.as_deref()));
                fields.push(format!("{}@{}", removed.username, removed.host));
                if let Some(erase) = removed.the_erase {
                    fields.push(
                        match erase {
                            TheErase::Confirmed => "the machine password was erased",
                            TheErase::LeftAsARecordedDebt => {
                                "the machine password's erasure is owed to the sweep"
                            }
                        }
                        .to_string(),
                    );
                }
            }
        }
        fields.join("\t")
    }

    /// The local server identifier the line is about.
    fn local_server_identifier(&self) -> &str {
        match self {
            Act::AServerConnected(connected) => &connected.local_server_identifier,
            Act::AServerRemoved(removed) => &removed.local_server_identifier,
        }
    }

    /// The line, ready for the diary service.
    ///
    /// **The moment is on the desktop's clock, stated as that clock**
    /// (`State, identity and recovery` D5), and **the identity it concerns is
    /// the local server identifier** ([`concerning_the_server`]). **The run
    /// identity travels in [`Line::run`] and is not repeated here.**
    pub fn line(&self, run: Identity, at: SystemTime) -> Line {
        Line {
            run,
            kind: LineKind::Act,
            at,
            clock: Clock::ThisComputer,
            text: self.text(),
            concerning: concerning_the_server(self.local_server_identifier()),
        }
    }
}

/// **What the line's identity list holds, and `Reporting a problem` §3 fixes
/// it**: *"the identities of what it concerns — job identity, group identity,
/// local server identifier, operation identity — where there are any, with the
/// user's own labels carried beside identities and never in place of them"*.
/// **Every line this module writes is about one server, so the list is that
/// server's identifier.**
///
/// **The run identity is not one of them.** `Reporting a problem` U7 already
/// carries it in [`Line::run`], which is what a report's own slicing rules cut
/// on; putting it here as well left every line this module wrote findable by
/// the session and by nothing else, **and a report a user sends about a server
/// is cut on the server**.
///
/// **The identifier is read back rather than held on the row as an identity.**
/// D3 holds it as the local record's text and changing the row's type would
/// reach the local record, which is another module's. **Where it does not read
/// back the list is empty and nothing is put in its place** — the line still
/// names the server in its own text, where it always did, and an identity
/// invented here would be a value nothing else in the project has ever issued.
fn concerning_the_server(local_server_identifier: &str) -> Vec<Identity> {
    Identity::read_back(IdentityKind::LocalServer, local_server_identifier)
        .into_iter()
        .collect()
}

impl AHardwareReading {
    /// **The fixed line form**: the reading's name; the **card count, read as a
    /// count**, and per card its **make as a value** and its **total memory**;
    /// the **machine's own memory** figure; the **free-space figure with the path
    /// it was taken against**; and **every stated absence the record holds** —
    /// because a reading with a stated absence is a complete reading and **the
    /// line says what the record says and no more**.
    fn text(&self) -> String {
        let mut fields = vec!["hardware reading".to_string(), self.local_server_identifier.clone()];
        fields.push(label(self.label.as_deref()));
        match &self.reading.cards {
            Cards::TheCards(cards) => {
                fields.push(format!(
                    "{} cards",
                    card_count(&self.reading.cards).expect("cards are counted")
                ));
                // **Which card it is, by the driver's own name for it** —
                // never the make's key, which is one word for every card this
                // reading can name (`Servers — values`' `CardMake`).
                for card in cards {
                    let which = card.make.the_drivers_name_for_the_card();
                    fields.push(match card.total_memory_bytes {
                        Some(bytes) => format!("{which}: {bytes} bytes"),
                        // H8's stated absence, written out rather than left off.
                        None => format!("{which}: no per-card memory"),
                    });
                }
            }
            Cards::PresentAndUnreadable(why) => fields.push(format!(
                "cards present and unreadable: {}",
                match why {
                    crate::hardware::WhyUnreadable::TheDriverIsAbsent => "the driver is absent",
                    crate::hardware::WhyUnreadable::TheDriverAnsweredWithNone =>
                        "the driver answered with none",
                }
            )),
            Cards::NoCard => fields.push("no card".to_string()),
            Cards::CouldNotBeEstablished => {
                fields.push("the cards could not be established".to_string())
            }
        }
        fields.push(match self.reading.machine_memory {
            MachineMemory::TheAllocation(bytes) => format!("the allocation: {bytes} bytes"),
            MachineMemory::TheWholeMachine(bytes) => format!("the whole machine: {bytes} bytes"),
            // H9a's third row, stated as an absence.
            MachineMemory::CouldNotBeEstablished => {
                "the allocation could not be established".to_string()
            }
        });
        fields.push(match &self.reading.free_space {
            Some(FreeSpace {
                available_bytes,
                the_path_it_was_taken_against,
            }) => format!("{available_bytes} bytes available at {the_path_it_was_taken_against}"),
            // **The free-space figure's absence, written out rather than left
            // off.** It is the same shape H8's per-card memory takes, and the
            // reason it is absent is `HardwareReading::free_space`'s own: the
            // server records no folder to take the reading against.
            None => "the free space could not be established".to_string(),
        });
        fields.join("\t")
    }

    /// **The identities and the two moments it carries**, which are J4's: the
    /// run identity, in [`Line::run`]; **the local server identifier, in the
    /// line's own identity list** ([`concerning_the_server`]), with the row's
    /// label beside it in the text; the
    /// **installation identity the reading was read under, or its explicit
    /// absence**; and the **machine's own moment on the machine's clock** beside
    /// the **desktop's own moment at which the reading was received**, each
    /// stated as its own clock and **never subtracted from one another**.
    ///
    /// **No journal sequence number**: the reading is taken on the primary
    /// connection and **never arrives as an event**, so J4's clause for one does
    /// not fire, **and its absence is the line's own reading rather than a
    /// gap**.
    pub fn line(&self, run: Identity) -> Line {
        let mut text = self.text();
        text.push('\t');
        text.push_str(&match &self.reading.read_under {
            Some(installation) => format!("read under {installation}"),
            None => "read under no installation".to_string(),
        });
        Line {
            run,
            kind: LineKind::Told,
            // **A told line's moment is the machine's own** (J4).
            at: self.reading.the_machines_own_moment.taken_at(),
            clock: self.reading.the_machines_own_moment.clock().clone(),
            text,
            concerning: concerning_the_server(&self.local_server_identifier),
        }
    }

    /// **The desktop's own moment travels with the line**, so the mark drawn
    /// from it is available beside the machine's. The two are **never subtracted
    /// from one another**.
    pub fn the_desktops_own_moment(&self) -> &Reading<()> {
        &self.reading.the_desktops_own_moment
    }
}

fn label(label: Option<&str>) -> String {
    // **The label beside the identifier and never in place of it**, and an
    // absence is written as one.
    match label {
        Some(label) => label.to_string(),
        None => "no label".to_string(),
    }
}

fn result_name(result: StopAndClearResult) -> &'static str {
    match result {
        StopAndClearResult::CouldNotBeginAtAll => "could not begin at all",
        StopAndClearResult::ClearedAndStopped => "cleared and stopped",
        StopAndClearResult::ClearedAndTheStopFailedOrExpired => {
            "cleared, and the stop was sent and the render had not ended when Smoozy stopped waiting"
        }
    }
}

/// **What this document writes no told line for, each absence declared so that
/// it is a decision rather than a silence.**
///
/// - **The queue reading and the adoption offer** — `Queue & History` A1's and
///   `Queue & History` A5's, consumed here and composed there; **the
///   declaration for them is that
///   document's too.**
/// - **The machine's answer to the not-Linux check** — **considered and declined
///   on J4's own requirement.** Every told line carries the machine's own moment
///   on the machine's clock, and **this reading is taken as a value carrying no
///   moment at all**, so declaring it would add to what this document does rather
///   than record it. **Nothing is lost**: outcomes 2 and 3 are sentences Smoozy
///   puts to the user, and a sentence put to the user reaches the diary as J6's
///   failure line, **captured verbatim at presentation by plumbing that is not
///   this document's.**
/// - **The connection's ending.** K2's announced ending and unanswered keepalive
///   are endings that came back from a machine, **but they arrive at the
///   machine's rate for as long as the app is open** — which is what J4a's bound
///   is structural against — and what the user is told about a run that failed is
///   N5's condition record, **captured once per condition identity as J6's
///   failure line**. This document writes no second record of it.
/// - **The installation identity read** at step 5 — its value is **a mechanism's
///   own identity rather than a reading a user could name**, which J3 keeps out.
/// - **The host key the scan returns** — **barred from the diary outright** by
///   `Reporting a problem` N2 and `Security and licence` §13 rule 3, **so it
///   could never be one whatever else held.**
/// - **The machine's Blender version and its Linux family** — J4 places both at
///   the acts that already read them, and **no Blender is in this document's
///   reading at all**.
///
/// **And three things this document does are not acts, each declined on J2a's
/// second test.** A **disconnect** ends a period with nothing said before it or
/// after it, and *disconnected* is what every server reads on a fresh launch
/// **with nothing having happened at all** — so a row reading it is not the
/// completion of anything. The **add's save** and the **edit's save** have no
/// completion the spec names to the user apart from what the row then holds and
/// the connection that follows, which is act one; **§14 already declares each as
/// a unit of the action it belongs to, and a unit is a step inside something else
/// rather than an act.**
pub const DECLARED_FOR_NOTHING_ELSE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::{Card, CardMake};
    use smoozy_reporting::lines::{ActTests, Declarations, ErasingActsAreNotDeclared};
    use smoozy_state::identities::IdentityKind;
    use std::time::Duration;

    fn a_run() -> Identity {
        Identity::issue(IdentityKind::Run).expect("randomness")
    }

    fn a_reading() -> HardwareReading {
        HardwareReading::taken(
            Cards::TheCards(vec![Card {
                unique_identifier: "GPU-aaa".into(),
                bus_location: "0000:01:00.0".into(),
                make: CardMake::from_the_drivers_own_query("NVIDIA GeForce RTX 4090"),
                total_memory_bytes: Some(25_757_220_864),
            }]),
            MachineMemory::TheAllocation(16_051_527_680),
            Some(FreeSpace {
                available_bytes: 20_000_000_000,
                the_path_it_was_taken_against: "/opt/smoozy/jobs".into(),
            }),
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_000),
            "srv-1",
            SystemTime::UNIX_EPOCH + Duration::from_secs(2_000),
            Some("inst-1".into()),
        )
    }

    fn a_connected_line() -> Act {
        Act::AServerConnected(AServerConnected {
            opening_act: OpeningAct::TheUsersConnectAction,
            local_server_identifier: "srv-1".into(),
            label: Some("hetzner".into()),
            host: "5.180.183.77".into(),
            port: 33510,
            username: "root".into(),
        })
    }

    #[test]
    fn this_module_declares_both_kinds() {
        let mut declarations: Declarations<&str> = Declarations::new();
        declarations.module_declares("Servers", DECLARED.to_vec());
        assert!(declarations.may_write(&"Servers", LineKind::Act));
        assert!(declarations.may_write(&"Servers", LineKind::Told));
    }

    #[test]
    fn both_declared_acts_pass_all_three_of_j2as_tests() {
        let tests = ActTests {
            smoozys_own: true,
            the_spec_names_its_completion_as_something_the_user_sees: true,
            at_the_users_own_rate: true,
        };
        // The bar excludes an act whose subject's own spec promises that
        // nothing afterwards shows the subject existed. Neither act here is
        // one: a connected server's row is the completion, and a removed
        // server's spec promises Smoozy forgets **the server**, not that the
        // removal is unshown.
        let bar = ErasingActsAreNotDeclared {
            the_subjects_spec_promises_no_trace_afterwards: false,
        };
        assert!(tests.is_diary_worthy(bar));
    }

    #[test]
    fn a_connected_line_names_the_opening_act_and_the_machine() {
        let text = a_connected_line().text();
        assert!(text.starts_with("server connected\tthe user's connect"));
        assert!(text.contains("root@5.180.183.77:33510"));
        assert!(text.contains("hetzner"));
    }

    #[test]
    fn the_label_is_beside_the_identifier_and_never_in_place_of_it() {
        let text = a_connected_line().text();
        assert!(text.contains("srv-1"));
        assert!(text.contains("hetzner"));

        let nameless = Act::AServerConnected(AServerConnected {
            label: None,
            ..match a_connected_line() {
                Act::AServerConnected(connected) => connected,
                _ => unreachable!(),
            }
        });
        assert!(nameless.text().contains("no label"));
    }

    #[test]
    fn no_line_carries_a_host_key_a_fingerprint_or_anything_about_a_secret() {
        let run = a_run();
        let removed = Act::AServerRemoved(AServerRemoved {
            reached_the_machine: true,
            result: Some(StopAndClearResult::ClearedAndTheStopFailedOrExpired),
            the_erase: Some(TheErase::LeftAsARecordedDebt),
            local_server_identifier: "srv-1".into(),
            label: None,
            host: "5.180.183.77".into(),
            username: "root".into(),
        });
        for act in [a_connected_line(), removed] {
            let line = act.line(run.clone(), SystemTime::now());
            assert!(!line.text.contains("SHA256:"));
            assert!(!line.text.contains("ssh-ed25519"));
            assert_eq!(line.clock, Clock::ThisComputer);
            assert_eq!(line.kind, LineKind::Act);
        }
    }

    #[test]
    fn an_erase_enters_as_confirmed_or_owed_and_never_as_a_value() {
        let owed = Act::AServerRemoved(AServerRemoved {
            reached_the_machine: false,
            result: None,
            the_erase: Some(TheErase::LeftAsARecordedDebt),
            local_server_identifier: "srv-1".into(),
            label: None,
            host: "h".into(),
            username: "u".into(),
        });
        assert!(owed.text().contains("owed to the sweep"));
        assert!(!owed.text().contains("password:"));
    }

    #[test]
    fn a_removal_that_reached_no_machine_names_no_result() {
        let removed = Act::AServerRemoved(AServerRemoved {
            reached_the_machine: false,
            result: None,
            the_erase: None,
            local_server_identifier: "srv-1".into(),
            label: None,
            host: "h".into(),
            username: "u".into(),
        });
        let text = removed.text();
        assert!(text.contains("reaching no machine"));
        assert!(!text.contains("cleared"));
    }

    #[test]
    fn the_middle_result_never_claims_the_render_could_not_be_stopped() {
        let words = result_name(StopAndClearResult::ClearedAndTheStopFailedOrExpired);
        assert!(words.contains("stopped waiting"));
        assert!(!words.contains("could not be stopped"));
    }

    #[test]
    fn the_told_line_is_on_the_machines_clock_and_carries_the_installation_identity() {
        let told = AHardwareReading {
            reading: a_reading(),
            local_server_identifier: "srv-1".into(),
            label: None,
        };
        let line = told.line(a_run());
        assert_eq!(line.kind, LineKind::Told);
        assert_eq!(
            line.clock,
            Clock::AMachine(smoozy_state::observed::MachineClock("srv-1".into()))
        );
        assert_eq!(line.at, SystemTime::UNIX_EPOCH + Duration::from_secs(1_000));
        assert!(line.text.contains("read under inst-1"));
    }

    #[test]
    fn an_explicit_absence_of_an_installation_identity_is_written_as_one() {
        let mut reading = a_reading();
        reading.read_under = None;
        let told = AHardwareReading {
            reading,
            local_server_identifier: "srv-1".into(),
            label: None,
        };
        assert!(told.line(a_run()).text.contains("read under no installation"));
    }

    #[test]
    fn the_told_line_carries_every_stated_absence_the_record_holds() {
        let mut reading = a_reading();
        reading.cards = Cards::CouldNotBeEstablished;
        reading.machine_memory = MachineMemory::CouldNotBeEstablished;
        let told = AHardwareReading {
            reading,
            local_server_identifier: "srv-1".into(),
            label: None,
        };
        let text = told.line(a_run()).text;
        assert!(text.contains("the cards could not be established"));
        assert!(text.contains("the allocation could not be established"));
    }

    #[test]
    fn the_told_line_names_the_free_space_path_it_was_taken_against() {
        let told = AHardwareReading {
            reading: a_reading(),
            local_server_identifier: "srv-1".into(),
            label: None,
        };
        assert!(told.line(a_run()).text.contains("at /opt/smoozy/jobs"));
    }

    #[test]
    fn the_told_line_is_one_per_reading_and_never_one_per_card() {
        // J4a's bound in this document's own terms.
        let told = AHardwareReading {
            reading: a_reading(),
            local_server_identifier: "srv-1".into(),
            label: None,
        };
        let line = told.line(a_run());
        assert_eq!(line.text.matches("hardware reading").count(), 1);
    }

    #[test]
    fn every_line_this_module_writes_names_the_server_and_never_the_session() {
        // `Reporting a problem` §3: the identities a line carries are the
        // identities of what it concerns, and every line here is about one
        // server. A report a user sends about a server is cut on the server, so
        // a list holding the session identity instead makes this module's lines
        // findable by nothing that matters.
        let run = a_run();
        let server = Identity::issue(IdentityKind::LocalServer).expect("randomness");
        let named = server.as_text();

        let connected = Act::AServerConnected(AServerConnected {
            local_server_identifier: named.clone(),
            ..match a_connected_line() {
                Act::AServerConnected(connected) => connected,
                _ => unreachable!(),
            }
        });
        let removed = Act::AServerRemoved(AServerRemoved {
            reached_the_machine: false,
            result: None,
            the_erase: None,
            local_server_identifier: named.clone(),
            label: None,
            host: "5.180.183.77".into(),
            username: "root".into(),
        });
        let told = AHardwareReading {
            reading: a_reading(),
            local_server_identifier: named.clone(),
            label: None,
        };

        let lines = [
            connected.line(run.clone(), SystemTime::UNIX_EPOCH),
            removed.line(run.clone(), SystemTime::UNIX_EPOCH),
            told.line(run.clone()),
        ];
        assert_eq!(lines.len(), DECLARED.len() + 1, "three line kinds");
        for line in lines {
            assert_eq!(line.concerning, vec![server.clone()]);
            assert!(!line.concerning.contains(&run), "the session is not what a line is about");
            // `Reporting a problem` U7's run identity still travels, on the
            // field that holds it.
            assert_eq!(line.run, run);
            // The label stays beside the identifier and never in place of it.
            assert!(line.text.contains(&named));
        }
    }

    #[test]
    fn an_identifier_that_does_not_read_back_leaves_the_list_empty_and_invents_nothing() {
        // Said plainly rather than met: nothing is put in the list in its
        // place, and the line still names the server in its own text.
        let told = AHardwareReading {
            reading: a_reading(),
            local_server_identifier: "srv-1".into(),
            label: None,
        };
        let line = told.line(a_run());
        assert!(line.concerning.is_empty());
        assert!(line.text.contains("srv-1"));
    }

    #[test]
    fn the_two_moments_travel_together_and_never_subtract() {
        let told = AHardwareReading {
            reading: a_reading(),
            local_server_identifier: "srv-1".into(),
            label: None,
        };
        assert_eq!(
            told.the_desktops_own_moment().clock(),
            &Clock::ThisComputer
        );
        assert_eq!(
            told.the_desktops_own_moment()
                .since(&told.reading.the_machines_own_moment),
            None
        );
    }
}
