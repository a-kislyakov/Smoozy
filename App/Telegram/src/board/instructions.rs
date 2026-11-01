//! **The instruction list — `Telegram` B8, B9, B12 and B15.** What one entry
//! carries, whose it is, how it is admitted, replaced and trimmed, and what its
//! deadline owes.
//!
//! **`Telegram` B8: an accepted instruction's durable home before
//! acknowledgement is the board.** Nothing here keeps a sender-local copy; an
//! entry exists once it is on a board that was read back (`Rented-server runtime` T12, T13).

use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::board::text::TheBoard;

/// **`Telegram` B8 — the protocol version every entry carries**, and **B9a —
/// the last one this build speaks.** It speaks every version from
/// [`THE_FIRST_PROTOCOL_VERSION`] to this one, and composes only this one.
pub const THE_PROTOCOL_VERSION: u32 = 1;

/// **`Telegram` B9a — the first protocol version there has been.** A version
/// before it was never written by anybody, so it is not spoken either.
pub const THE_FIRST_PROTOCOL_VERSION: u32 = 1;

/// **`Telegram` B9a — the reason recorded beside the identity of an entry
/// refused for its version** (B9; `Server protocol and transfers` C4's shape).
pub const WHY_AN_ENTRY_OF_ANOTHER_VERSION_IS_REFUSED: &str =
    "the instruction's protocol version is not one this server's Smoozy speaks";

/// **`Telegram` B9a and B9 — what the user is told about an entry refused for
/// its version**: the server named, the button or request and the job where
/// known, and that the server's Smoozy is an older version. **Never *did not
/// confirm*** (B12), which would say the machine was silent when it answered.
/// Standard words (Rule 9): *… failed: update Smoozy on this server.*, the
/// fix written as an instruction, as everywhere else (the second words sweep,
/// 2026-11-01; it read *… failed: Smoozy on this server is out of date.*, and
/// before step 13's words sweep *… was not done, because this server runs an
/// older version of Smoozy.*).
pub fn an_older_version_could_not_do_it(server: &(impl crate::job_messages::TheServersName + ?Sized), label: &str, asked: &str) -> crate::job_messages::AMarkedText {
    let server = if server.label().is_empty() { crate::job_messages::AServerLabel::from("A server") } else { server.named() };
    let asked = if asked.is_empty() { "The action".to_string() } else { format!("\u{201c}{asked}\u{201d}") };
    crate::job_messages::a_user_message(&server, label, "", &format!("{asked} failed: update Smoozy on this server."))
}

/// **`Telegram` B9a — whether a machine speaking up to `its_own` speaks
/// `version`.**
fn a_machine_speaks(version: u32, its_own: u32) -> bool {
    (THE_FIRST_PROTOCOL_VERSION..=its_own).contains(&version)
}

/// **`Telegram` B12 — how long an action instruction may wait for its
/// acknowledgement before the holder says it has not been confirmed.** The
/// plan fixes that an entry carries its own reporting deadline and names no
/// default. **Thirty seconds is this build's choice**: an addressee reads every
/// two seconds (B13), so a healthy acknowledgement takes a few reads, and a
/// longer silence is worth telling the user about. Recorded at
/// `Audits/Build/Telegram/Decisions record.md`.
pub const THE_DEFAULT_REPORTING_DEADLINE: Duration = Duration::from_secs(30);

/// **`Telegram` R1, R1a and R12 — the one default for a tap's resolution, a
/// reply's resolution and a gather**, ten seconds. Composing those entries is
/// piece 3's; the board's half of their expiry (B12) is here.
pub const THE_DEFAULT_RESOLUTION_DEADLINE: Duration = Duration::from_secs(10);

/// **`Telegram` B8 — the addressee address.** The machine identity for a full
/// action; for a context-resolution instruction, the one address the listener
/// holds: the routing key for a tap (R1), the source message for a reply (R1a).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TheAddressee {
    Machine(String),
    RoutingKey(String),
    SourceMessage(i64),
}

/// **`Telegram` B12's three kinds of entry**, each ending its own way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AKindOfEntry {
    /// Ends only by its acknowledgement; never expires.
    Action,
    /// R10's gather: ends at its reply or its deadline.
    StateRequest,
    /// R1 and R1a: ends by its replacement or its deadline.
    ContextResolution,
}

/// **`Telegram` B8 — one list entry.** Protocol version, operation identity,
/// addressee address, subject identities, action, complete payload, source
/// update identity and acceptance moment, with B12's deadline and kind.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnEntry {
    pub version: u32,
    pub operation: String,
    pub addressee: TheAddressee,
    pub kind: AKindOfEntry,
    pub subjects: Vec<String>,
    pub action: String,
    pub payload: Value,
    pub source_update: Option<i64>,
    /// The moment on the accepting holder's own clock. Written by the holder
    /// at acceptance; never subtracted from anything (`State, identity and recovery` D5).
    pub accepted_at: String,
    /// B12 and R1: carried as an interval, measured by each holder on its own
    /// clock from the moment the entry entered its board.
    pub deadline_seconds: u64,
}

/// **Who a role worker is, for `Telegram` B8's recognition by equality.** The
/// machine identity from its role configuration and the routing key from the
/// Telegram worker's state store.
#[derive(Debug, Clone, Copy)]
pub struct WhoIAm<'a> {
    pub machine: &'a str,
    pub routing_key: &'a str,
}

impl AnEntry {
    /// **`Telegram` B8 — an entry is this machine's by equality against its
    /// own machine identity or routing key, or because it holds an
    /// action-context record bound to the source message.** Nothing is
    /// translated through the roster.
    ///
    /// **`Telegram` B9a — a state-request or context-resolution entry of a
    /// version this machine does not speak is not taken as its own**, because
    /// answering or resolving it would be acting on it. It ends at its own
    /// deadline (B12's carve-out) with that deadline's own answer. **An action
    /// entry of such a version is still taken as its own**, so the addressee
    /// refuses it, records why and acknowledges it (B9) rather than going
    /// quiet — see [`AnEntry::is_spoken_here`].
    pub fn is_addressed_to(&self, me: WhoIAm<'_>, holds_a_record_for: impl Fn(i64) -> bool) -> bool {
        if self.kind != AKindOfEntry::Action && !self.is_spoken_here() {
            return false;
        }
        match &self.addressee {
            TheAddressee::Machine(machine) => machine == me.machine,
            TheAddressee::RoutingKey(key) => key == me.routing_key,
            TheAddressee::SourceMessage(message) => holds_a_record_for(*message),
        }
    }

    /// **`Telegram` B9a — whether this build speaks the entry's protocol
    /// version**: every one from the first to its own. **An action entry it
    /// does not speak is never acted on, never partly acted on and never
    /// guessed at**: the addressee records it as refused
    /// ([`WHY_AN_ENTRY_OF_ANOTHER_VERSION_IS_REFUSED`]), tells the user
    /// ([`an_older_version_could_not_do_it`]) and acknowledges it (B9).
    pub fn is_spoken_here(&self) -> bool {
        a_machine_speaks(self.version, THE_PROTOCOL_VERSION)
    }

    pub fn deadline(&self) -> Duration {
        Duration::from_secs(self.deadline_seconds)
    }
}

/// How an entry offered to the board came out.
#[derive(Debug, Clone, PartialEq)]
pub enum AnAdmission {
    /// The candidate board with the entry appended. Accepted only once this
    /// candidate is read back (`Rented-server runtime` T12).
    Accepted(TheBoard),
    AlreadyOnTheBoard,
    /// **`Telegram` B2: not accepted, and nothing evicted.**
    DoesNotFit,
}

/// **`Telegram` B8 and B15 — append one entry through a complete bounded
/// candidate.** An operation already on the board is not appended twice.
pub fn admit(board: &TheBoard, entry: &AnEntry, moment: &str) -> AnAdmission {
    if board.instructions.iter().any(|e| e.operation == entry.operation) {
        return AnAdmission::AlreadyOnTheBoard;
    }
    let mut candidate = board.clone();
    let mut entry = entry.clone();
    entry.accepted_at = moment.to_string();
    candidate.instructions.push(entry);
    if candidate.fits() {
        AnAdmission::Accepted(candidate)
    } else {
        AnAdmission::DoesNotFit
    }
}

/// How a replacement of a context-resolution entry came out.
#[derive(Debug, Clone, PartialEq)]
pub enum AReplacement {
    /// The standing resolution entry exchanged for the full instruction under
    /// the same operation identity.
    Exchanged(TheBoard),
    /// The full instruction already stands under that identity.
    AlreadyReplaced,
    /// **`Telegram` B15 — no resolution entry stands under that identity, so
    /// nothing is appended.** This refusal is what keeps R1's *nothing was
    /// done* true.
    Refused,
    DoesNotFit,
}

/// **`Telegram` B8's exchange by identity, and B15's late-replacement refusal.**
pub fn replace_a_resolution(board: &TheBoard, replacement: &AnEntry, moment: &str) -> AReplacement {
    let Some(at) = board
        .instructions
        .iter()
        .position(|e| e.operation == replacement.operation)
    else {
        return AReplacement::Refused;
    };
    if board.instructions[at].kind != AKindOfEntry::ContextResolution {
        return AReplacement::AlreadyReplaced;
    }
    let mut candidate = board.clone();
    let mut full = replacement.clone();
    full.kind = AKindOfEntry::Action;
    full.accepted_at = moment.to_string();
    candidate.instructions[at] = full;
    if candidate.fits() {
        AReplacement::Exchanged(candidate)
    } else {
        AReplacement::DoesNotFit
    }
}

/// **`Telegram` B9 and B11 — an observed acknowledgement trims the matching
/// action instruction.** Only action instructions end this way (B12). Gives
/// back nothing where nothing changed, so no needless edit is made.
pub fn without_the_acknowledged(board: &TheBoard, operations: &[String]) -> Option<TheBoard> {
    let mut candidate = board.clone();
    candidate
        .instructions
        .retain(|e| !(e.kind == AKindOfEntry::Action && operations.contains(&e.operation)));
    (candidate.instructions.len() != board.instructions.len()).then_some(candidate)
}

/// The board with one entry removed, whatever its kind.
pub fn without(board: &TheBoard, operation: &str) -> TheBoard {
    let mut candidate = board.clone();
    candidate.instructions.retain(|e| e.operation != operation);
    candidate
}

/// What an entry's deadline owes the holder that measured it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatADeadlineOwes {
    /// **`Telegram` B12 and `Rented-server runtime` T14**: say plainly that the addressee has not
    /// confirmed. The entry stays; an accepted action never expires.
    TellTheUserItIsUnconfirmed,
    /// **`Telegram` B12's carve-out**: a state-request or a context-resolution
    /// entry leaves the board at its deadline. Nothing durable changes.
    RemoveIt,
}

/// **`Telegram` B12 and R1 — measured on this holder's own clock from the
/// moment the entry entered this holder's board**, so nothing is subtracted
/// across clocks and a hand-over restarts the measurement.
pub fn what_the_deadline_owes(entry: &AnEntry, entered_this_board: Duration, now: Duration) -> Option<WhatADeadlineOwes> {
    if now.saturating_sub(entered_this_board) < entry.deadline() {
        return None;
    }
    Some(match entry.kind {
        AKindOfEntry::Action => WhatADeadlineOwes::TellTheUserItIsUnconfirmed,
        AKindOfEntry::StateRequest | AKindOfEntry::ContextResolution => WhatADeadlineOwes::RemoveIt,
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::board::text::tests::a_board;
    use serde_json::json;

    pub(crate) fn an_entry(operation: &str, addressee: TheAddressee, kind: AKindOfEntry) -> AnEntry {
        AnEntry {
            version: THE_PROTOCOL_VERSION,
            operation: operation.into(),
            addressee,
            kind,
            subjects: vec!["job-1".into()],
            action: "stop".into(),
            payload: json!({}),
            source_update: Some(7),
            accepted_at: String::new(),
            deadline_seconds: 30,
        }
    }

    #[test]
    fn an_entry_is_recognised_by_equality_and_by_a_held_record() {
        let me = WhoIAm { machine: "m1", routing_key: "k1" };
        let to = |a| an_entry("op", a, AKindOfEntry::Action);
        assert!(to(TheAddressee::Machine("m1".into())).is_addressed_to(me, |_| false));
        assert!(!to(TheAddressee::Machine("m2".into())).is_addressed_to(me, |_| false));
        assert!(to(TheAddressee::RoutingKey("k1".into())).is_addressed_to(me, |_| false));
        assert!(to(TheAddressee::SourceMessage(5)).is_addressed_to(me, |m| m == 5));
        assert!(!to(TheAddressee::SourceMessage(6)).is_addressed_to(me, |m| m == 5));
    }

    /// **`Telegram` B9a — every version from the first to this build's own is
    /// spoken; one ahead is not, and neither is one before the first.** The
    /// rule is also tried as the next release would hold it, so an entry of
    /// this build's version stays spoken there.
    #[test]
    fn an_entry_one_version_ahead_is_not_spoken_and_every_earlier_one_is() {
        let at = |version| AnEntry { version, ..an_entry("op", TheAddressee::Machine("m1".into()), AKindOfEntry::Action) };
        for version in THE_FIRST_PROTOCOL_VERSION..=THE_PROTOCOL_VERSION {
            assert!(at(version).is_spoken_here(), "version {version}");
        }
        assert!(!at(THE_PROTOCOL_VERSION + 1).is_spoken_here());
        assert!(!at(THE_FIRST_PROTOCOL_VERSION - 1).is_spoken_here());
        assert!(a_machine_speaks(THE_PROTOCOL_VERSION, THE_PROTOCOL_VERSION + 1), "the next release speaks this one");
    }

    /// **`Telegram` B9a — a newer action entry is still recognised, so its
    /// addressee can refuse and acknowledge it; a newer state-request or
    /// context-resolution entry is not taken up at all**, so nothing answers or
    /// resolves it. Before piece 11 an older issuer resolved a newer tap's
    /// entry by its own rules and submitted the stop.
    #[test]
    fn a_newer_request_or_resolution_is_not_taken_up_and_a_newer_action_is_kept_to_be_refused() {
        let me = WhoIAm { machine: "m1", routing_key: "k1" };
        let newer = |addressee, kind| AnEntry { version: THE_PROTOCOL_VERSION + 1, ..an_entry("op", addressee, kind) };
        assert!(newer(TheAddressee::Machine("m1".into()), AKindOfEntry::Action).is_addressed_to(me, |_| false));
        assert!(!newer(TheAddressee::Machine("m1".into()), AKindOfEntry::StateRequest).is_addressed_to(me, |_| false));
        assert!(!newer(TheAddressee::RoutingKey("k1".into()), AKindOfEntry::ContextResolution).is_addressed_to(me, |_| false));
        assert!(!newer(TheAddressee::SourceMessage(5), AKindOfEntry::ContextResolution).is_addressed_to(me, |m| m == 5));
        // This build's own are taken up as before.
        assert!(an_entry("op", TheAddressee::Machine("m1".into()), AKindOfEntry::StateRequest).is_addressed_to(me, |_| false));
        assert!(an_entry("op", TheAddressee::RoutingKey("k1".into()), AKindOfEntry::ContextResolution).is_addressed_to(me, |_| false));
    }

    /// **`Telegram` B9a — the refusal's words** name the server, the button
    /// and the job, say the server's Smoozy is older, and never say it has not
    /// confirmed.
    #[test]
    fn the_refusal_names_the_server_and_says_its_smoozy_is_older() {
        assert_eq!(
            an_older_version_could_not_do_it("Vast 5060", "Kitchen", "Stop this job"),
            "Vast 5060\nKitchen\n“Stop this job” failed: update Smoozy on this server."
        );
        assert_eq!(
            an_older_version_could_not_do_it("", "", ""),
            "A server\nThe action failed: update Smoozy on this server."
        );
        assert!(!an_older_version_could_not_do_it("Vast 5060", "Kitchen", "Stop this job").contains("confirmed"));
    }

    /// **`Telegram` B2: an entry that does not fit is not accepted and nothing
    /// is evicted.**
    #[test]
    fn an_entry_that_does_not_fit_evicts_nothing() {
        let board = a_board("m1");
        let mut big = an_entry("big", TheAddressee::Machine("m2".into()), AKindOfEntry::Action);
        big.payload = json!("x".repeat(5000));
        assert_eq!(admit(&board, &big, "now"), AnAdmission::DoesNotFit);

        let small = an_entry("small", TheAddressee::Machine("m2".into()), AKindOfEntry::Action);
        let AnAdmission::Accepted(with) = admit(&board, &small, "now") else { panic!() };
        assert_eq!(with.instructions.len(), 1);
        assert_eq!(with.instructions[0].accepted_at, "now");
        assert_eq!(admit(&with, &small, "later"), AnAdmission::AlreadyOnTheBoard);
    }

    /// **`Telegram` B15 — a replacement with no standing resolution entry is
    /// refused.**
    #[test]
    fn a_late_replacement_is_refused_and_a_timely_one_exchanged() {
        let board = a_board("m1");
        let resolution = an_entry("op", TheAddressee::RoutingKey("k".into()), AKindOfEntry::ContextResolution);
        let full = an_entry("op", TheAddressee::Machine("m2".into()), AKindOfEntry::Action);
        assert_eq!(replace_a_resolution(&board, &full, "t"), AReplacement::Refused);

        let AnAdmission::Accepted(with) = admit(&board, &resolution, "t") else { panic!() };
        let AReplacement::Exchanged(exchanged) = replace_a_resolution(&with, &full, "t2") else { panic!() };
        assert_eq!(exchanged.instructions.len(), 1);
        assert_eq!(exchanged.instructions[0].kind, AKindOfEntry::Action);
        assert_eq!(replace_a_resolution(&exchanged, &full, "t3"), AReplacement::AlreadyReplaced);
    }

    #[test]
    fn an_acknowledgement_trims_only_action_instructions() {
        let mut board = a_board("m1");
        board.instructions.push(an_entry("a", TheAddressee::Machine("m2".into()), AKindOfEntry::Action));
        board.instructions.push(an_entry("r", TheAddressee::RoutingKey("k".into()), AKindOfEntry::ContextResolution));
        let trimmed = without_the_acknowledged(&board, &["a".into(), "r".into()]).unwrap();
        assert_eq!(trimmed.instructions.len(), 1);
        assert_eq!(trimmed.instructions[0].operation, "r");
        assert!(without_the_acknowledged(&trimmed, &["a".into()]).is_none());
    }

    /// **`Telegram` B12: an action is reported and never expires; the other
    /// two kinds leave at their deadline.**
    #[test]
    fn the_deadline_reports_an_action_and_removes_the_other_kinds() {
        let action = an_entry("a", TheAddressee::Machine("m2".into()), AKindOfEntry::Action);
        let mut gather = an_entry("g", TheAddressee::Machine("m2".into()), AKindOfEntry::StateRequest);
        gather.deadline_seconds = 10;
        let s = Duration::from_secs;
        assert_eq!(what_the_deadline_owes(&action, s(100), s(129)), None);
        assert_eq!(what_the_deadline_owes(&action, s(100), s(130)), Some(WhatADeadlineOwes::TellTheUserItIsUnconfirmed));
        assert_eq!(what_the_deadline_owes(&gather, s(100), s(110)), Some(WhatADeadlineOwes::RemoveIt));
    }
}
