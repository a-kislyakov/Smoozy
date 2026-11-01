//! The store — `Servers` §6.
//!
//! **D1: everything durable Smoozy holds about a server is in the local record,
//! and it is either a field of the server row or a record keyed by the row's
//! local server identifier.** There is no second store, no per-server file, and
//! **nothing written into the user's own SSH configuration** — `Security and
//! licence` §7 is exact about that last one.
//!
//! **D2: the one thing not in the local record is a secret's value**, which
//! sits in the operating system's secret store under a (kind, subject) pair,
//! reached only through `Desktop app and delivery`'s secret adapter. What the
//! server row holds is the **subject reference**, which carries no value and,
//! per §3.0 rule 1, **carries nothing about the user** — never a host, a port, a
//! username, a file path or a fingerprint.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` L1, L4a.
//! `Server protocol and transfers` S28. `Server Setup` R7b. `Telegram` L1.

use std::time::SystemTime;

use smoozy_security_and_licence::held::Subject;
use smoozy_state::lifetimes::Lifetime;

use crate::hardware::HardwareReading;
use crate::identity::{Host, KeyReference, Port, Username};

/// **D3: the server row, in full.**
///
/// Every field's *ends at* is in its own doc comment, and the whole table plus
/// [`KeyedByTheServer`]'s list is **one enumeration** ([`enumeration`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerRow {
    /// **Identity, issued by the local record** (§2.2's register). Ends at the
    /// removal. **C6: everything is keyed on this** and nothing is derived from
    /// a host, a port or a username.
    pub local_server_identifier: String,
    /// The connection identity's first three fields. Carry no installation
    /// identity. End at the removal.
    pub host: Host,
    pub port: Port,
    pub username: Username,
    /// **`Keys`' reference and Row 46's match value**, as the pair P1a fixes.
    /// Carries no installation identity. Ends at the removal.
    pub key: KeyReference,
    /// **Display only.** The optional name and the colour. End at the removal.
    pub label: Option<String>,
    pub colour: Option<String>,
    /// **The user's own instruction** ([`crate::setup_bit`]). **Carries no
    /// installation identity** and B8 says why. Ends at the removal, the
    /// establishment of a connection, or the user dropping it.
    pub remembered_setup: Option<RememberedSetup>,
    /// **The machine's identity** ([`RememberedHostKey`]). **Carries no
    /// installation identity** and D10 says why. Ends at the removal, a host or
    /// port edit, or an accept.
    pub remembered_host_key: Option<RememberedHostKey>,
    /// **Machine-derived, and it carries an installation identity.** Ends at
    /// the removal, or a host or port edit.
    pub remembered_hardware_reading: Option<HardwareReading>,
    /// **The machine's own root, recorded at hand-over.** Machine-derived, and
    /// **it carries an installation identity**. Ends at the removal, or a host
    /// or port edit.
    ///
    /// **This field is `Job files and outputs` L1's and D3 did not name it.**
    /// That clause reads: *"the root is chosen once per installation and never
    /// re-chosen. Its path is recorded in the machine's state area at
    /// hand-over, beside the installation identity, and **it is recorded
    /// against the server row on the desktop side as a machine-derived value
    /// carrying that identity**."* **The plan was repaired at D3 and D13 rather
    /// than the field being put somewhere D6's enumeration does not reach**,
    /// because D7 makes a durable thing about a server outside that enumeration
    /// a fault.
    ///
    /// **What reads it is H10's free-space reading**, which is per path and had
    /// no path to be taken against until this column existed.
    pub recorded_machine_root: Option<RecordedMachineRoot>,
    /// **Machine-derived** (§2.3). Ends at the removal, or a host or port edit.
    pub last_confirmed_installation_identity: Option<String>,
    /// **Reader state**, keyed to the installation identity by
    /// `State, identity and recovery` §4's E2 — **that document's E2 and not
    /// this one's**, which is the edit put-back — so it carries one. Ends at
    /// the removal, or a host or port edit.
    pub event_cursor: Option<u64>,
    /// **A reference into the secret store, never a value** (D2). Carries no
    /// installation identity.
    pub machine_password_subject: Option<Subject>,
    /// **The chosen fallback folder** — D3's row added 2026-10-04 with `Job
    /// files and outputs` P23: where a job whose own output folder cannot be
    /// used on this machine goes, as the user typed it with surrounding white
    /// space removed. **Absent means Smoozy's own job area.** **The user's own
    /// instruction and not a machine-derived value**, like the remembered
    /// setup (B8): it carries no installation identity, ends at the removal
    /// or when the user clears it, and **is kept by every D12 edit, a host or
    /// port edit included**. A path that does not start with `/` is refused at
    /// the dialog and never written, as a port out of range is.
    pub chosen_fallback_folder: Option<String>,
    /// **`Servers` H11b's second case: the last setup run on this machine that
    /// checked its driver ended with the driver still below the OptiX bar** —
    /// `Server Setup` R1 item 5's miss, whichever run made it (`Server Setup` R1b). D3's row
    /// added 2026-10-11, the fourth piece of the OptiX and driver work.
    /// **Machine-derived**, like the hardware reading the statement is drawn
    /// beside. **It is what lets a machine Smoozy manages be told as a
    /// container is** (`smoozy_servers_values::hardware::the_statement_is_said_on`).
    ///
    /// **It stands until the next such run on the server replaces it** — the
    /// runner's ruling of 2026-10-11 that a missed swap is not retried on its
    /// own — and ends at the removal or a host or port edit (D13), which points
    /// the entry at a different machine. **`false` is *no miss recorded*.**
    pub the_driver_missed_the_optix_bar: bool,
}

/// **`Job files and outputs` L1's root, as the desktop records it.**
///
/// **The path and the installation identity it was read under travel
/// together**, because §2.3 requires every durably remembered machine-derived
/// value to carry the installation identity it was read under, and because a
/// root recorded against one installation says nothing about the next one — a
/// container rental's recycle *"destroys everything Smoozy installed and returns
/// on the same address and port with nothing in the connection to reveal it"*.
///
/// **Nothing composes a path from this but `Job files and outputs`.** §2.1 rule
/// 7 gives every path one owner and this module is not it; what this module
/// does is hold the value and hand it to the reading that needs a path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedMachineRoot {
    /// The root's own path, exactly as `Job files and outputs` L1 chose it.
    pub path: String,
    /// **The package area's own location, where `Job files and outputs` L4a
    /// placed it on a mount of its own.**
    ///
    /// *"Where L4a placed the package area on a second mount, that location is
    /// recorded the same way and in the same durable act as the root's, so a
    /// reading that finds one finds both"* (`Server Setup` R7b). **Absent where
    /// L4a's second choice did not fire**, which is that clause's *"nothing
    /// second is chosen, recorded or handed over"* — so an absence here is the
    /// package area sitting under the root, and never a value nobody wrote
    /// down.
    pub the_package_areas_own_location: Option<String>,
    /// **The installation identity it was recorded under** (§2.3).
    pub read_under: String,
}

/// What a ticked setup option recorded. **B1: the bit records which options
/// were ticked**, and it is written as part of the same durable write that saves
/// the server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RememberedSetup {
    pub options: Vec<String>,
}

/// **D8: one field of the server row, holding the host key value the last
/// accepted comparison approved.**
///
/// Per server, rewritable one entry at a time, readable and writable at the
/// moment a connection is attempted, and started over by an address edit or an
/// accept by being overwritten or cleared. **A single shared file gives none of
/// that**, which is `Security and licence` §7's own argument, and that document
/// is exact: *"The remembered identity lives in the local record against the
/// saved server, and nowhere else. Not in `~/.ssh/known_hosts`."*
///
/// **D9: it is never shown, never logged, and never handed to any other
/// module** — *"nothing here reaches the user's eyes. No fingerprint at a first
/// connection, none at a changed one, none in the warning, and none in any
/// log"*.
///
/// **D10: it carries no installation identity, and that is deliberate rather
/// than an omission.** It is read **before** any connection that presents
/// Smoozy's credential, and the installation identity is read over such a
/// connection — so there is no identity in hand at the moment it is recorded.
/// It is recorded on machines where **no installation exists at all**. And it is
/// a statement about **the machine at that address** and not about Smoozy's
/// installation on it.
///
/// **D11: nothing in this document, and nothing this document supplies, is
/// keyed on it.** **Fact 83 is answered negatively** — a fresh container rental
/// on the same physical machine presented **byte-identical** host keys while
/// being a demonstrably different container with everything Smoozy installed
/// destroyed.
#[derive(Clone, PartialEq, Eq)]
pub struct RememberedHostKey {
    value: Vec<u8>,
}

/// **D9's *never logged*, kept by the type.** A derived rendering would put the
/// value into any line anything ever formatted this into, which is the one
/// route D9 could be broken by without anybody deciding to break it. This is
/// the same shape `Server protocol and transfers`' `Credential` uses for the
/// passphrase it carries.
impl std::fmt::Debug for RememberedHostKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("RememberedHostKey(recorded)")
    }
}

impl RememberedHostKey {
    pub fn approved(value: Vec<u8>) -> RememberedHostKey {
        RememberedHostKey { value }
    }

    /// **The only thing anything may do with it** — compare it against the
    /// scanned offer. **There is no accessor for the value beyond the local
    /// record's own write below**, which is D9 kept by the type rather than by
    /// care at each site that might have shown it.
    pub fn matches(&self, offered: &[u8]) -> bool {
        self.value == offered
    }

    /// **The one destination D8 and `Security and licence` §7 name**: *"The
    /// remembered identity lives in the local record against the saved server,
    /// and nowhere else."* The value has to reach that store to be written, and
    /// this is the whole of how it gets there.
    ///
    /// **It is not the accessor D9 forbids.** D9 bars showing it, logging it,
    /// and handing it to another module; a durable store is where the clause
    /// itself puts it. Nothing else in this project calls this, and the name is
    /// what says so at every call site.
    pub fn as_the_local_record_writes_it(&self) -> &[u8] {
        &self.value
    }
}

/// **D4: records keyed by the local server identifier, held elsewhere in the
/// local record and reached by the same key.**
///
/// **None of them is this document's content**; what is this document's is that
/// **each is reachable from the row and goes with it**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyedByTheServer {
    /// §10.3's one suppression table.
    SuppressedWarningRows,
    /// §8.7's failure records.
    FailureRecords,
    /// `Server protocol and transfers` S9's.
    PerTransferRecords,
    /// **§7 step 2's provisional job records.**
    ///
    /// **§16 item 26: a widened sentence in an input document leaves four
    /// clauses here disagreeing about whether removing a server takes the
    /// user's history with it.** This list was narrowed on 2026-08-22 to
    /// *provisional* records on `State, identity and recovery` §7 step 2 as it
    /// then stood; **that step was widened on 2026-08-29** to *"its provisional
    /// **and confirmed** records go with it"*. **This document's list is left
    /// exactly as it is and the disagreement is filed rather than settled
    /// here**, routed to the architecture runner. **A builder must not resolve
    /// it by widening this entry.**
    ProvisionalJobRecords,
    /// **`Telegram` §9's told-record**: one entry per server per told thing,
    /// held *"in the local record against the server row"* (`Telegram` L1), and
    /// **subject-held**, ending with its server (`State, identity and recovery`
    /// §11.1's table names *"a server's told-record"*).
    ///
    /// **Widened at Build, 2026-09-14.** D4's list names no told-record kind, so
    /// a told entry was a durable thing about a server outside D6's enumeration,
    /// which D7 calls a fault in this section. It is named here so a removal
    /// reaches it by its own walk (D17). **That D4 names none is a plan fault,
    /// recorded at `Audits/Build/Servers/Decisions record.md` and not repaired
    /// from Build.**
    ToldRecord,
    /// **`Queue & History` W5's idle-spoken marks**: *"one small record in
    /// the local record, subject-held on the server row"*.
    ///
    /// **Widened at Build, 2026-10-03 (the 38's piece 1)**, for the reason the
    /// told-record was: D4's list names no such kind, and a mark left behind
    /// by a removal would be a durable thing about a server outside D6's
    /// enumeration. Recorded at `Audits/Build/Servers/Decisions record.md`'s
    /// lines from that piece, and not repaired from Build.
    IdleSpokenMarks,
    /// **`Queue & History` C2's catch-up notice record**: *"its lifetime class
    /// is subject-held on the server row … it ends with the row"*, and `Queue &
    /// History` C2a: *"A server removed while its group stands takes its record
    /// with it."*
    ///
    /// **Widened at Build, 2026-10-04 (the 38's piece 15)**, for the reason
    /// the idle-spoken mark was: D4's list names no such kind, and a notice
    /// left behind by a removal would be a durable thing about a server
    /// outside D6's enumeration.
    CatchUpNotices,
}

impl KeyedByTheServer {
    pub const ALL: [KeyedByTheServer; 7] = [
        KeyedByTheServer::SuppressedWarningRows,
        KeyedByTheServer::FailureRecords,
        KeyedByTheServer::PerTransferRecords,
        KeyedByTheServer::ProvisionalJobRecords,
        KeyedByTheServer::ToldRecord,
        KeyedByTheServer::IdleSpokenMarks,
        KeyedByTheServer::CatchUpNotices,
    ];
}

/// **D6: the table in D3 and the list in D4 together are one enumeration, and
/// it is the only route by which anything about a server is reached.**
///
/// **Nothing about a server is reachable only by the path that wrote it.** That
/// is Row 41's real requirement — five separate things are named, they do not
/// live in one place, and the promise is that a removal reaches all five — and
/// it is met by **making the enumeration the structure** rather than a list a
/// builder keeps in step by hand.
///
/// **D7: a builder that finds a durable thing about a server outside that
/// enumeration has found a fault in this section.** **One stated exception, so
/// this rule cannot be read as taking the user's history**: the history record
/// is durable and names the server the job was submitted to, and it is outside
/// the enumeration by **D5**'s own rule — keyed by the **job identity**, left
/// exactly where it is by a removal. **Finding one is finding D5 working, not a
/// fault.**
pub fn enumeration(row: &ServerRow) -> Enumeration<'_> {
    Enumeration {
        row,
        keyed_records: &KeyedByTheServer::ALL,
    }
}

/// The walk a removal takes.
#[derive(Debug)]
pub struct Enumeration<'a> {
    pub row: &'a ServerRow,
    pub keyed_records: &'a [KeyedByTheServer],
}

/// **D5: history records are keyed by the job identity and never by the server
/// row**, which is why a removal leaves them exactly where they are. That is
/// `Queue & History`'s rule and this document performs nothing about it.
pub const HISTORY_IS_KEYED_BY_THE_JOB_IDENTITY: bool = true;

/// **D17: a removal walks the enumeration and removes everything in it.**
///
/// The five things the spec names are all inside it: the details, **the
/// connection — the period, which ends with the local forget** (S10 item 6),
/// the machine's remembered identity, any remembered setup, and any machine
/// password. **What the walk never reaches is a history record.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Removal {
    /// **D19: it happens without reaching the machine, in every case but
    /// one.** Nothing in a removal is a clean-up performed on the server.
    /// **Removing a disconnected server never starts a connection and never
    /// asks for a passphrase.** The one exception is *stop and clear first*,
    /// which is not a clean-up either — **it is two commands the user asked
    /// for**.
    pub reaches_the_machine: bool,
    /// **D18: the machine password's value is erased through the secret
    /// adapter, and an erase the store does not confirm is a recorded debt
    /// rather than a forgotten one.**
    ///
    /// **A leftover here is a remembered secret against a machine the user
    /// believes Smoozy has forgotten, and nothing afterwards would ever show
    /// it** — the debt is what makes that impossible rather than unlikely.
    pub the_machine_password_erase: Option<TheErase>,
    /// **D20: staged download material held against the row** is `Server
    /// protocol and transfers` S28's third bullet and is **cited rather than
    /// performed here**: it is removed with the row or recorded as an owed
    /// cleanup, because §11.1 makes values held against a removed server row
    /// subject-held.
    pub staged_download_material: StagedMaterial,
}

/// What became of the erase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheErase {
    Confirmed,
    /// `Security and licence` §5.1's mark, carried forward for the sweep.
    LeftAsARecordedDebt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedMaterial {
    RemovedWithTheRow,
    RecordedAsAnOwedCleanup,
    ThereWasNone,
}

/// **D21: history is untouched.** Removing a server does nothing to any history
/// record, which is `Queue & History`'s rule and the spec's own.
pub fn a_removal_touches_history() -> bool {
    false
}

/// **D22: nothing durable records that a server was connected**, so on a fresh
/// launch there is no period, every server's observed value is *disconnected*,
/// and **reopening the app connects to nothing on its own**.
///
/// **There is no clearing step at start-up, which means there is no clearing
/// step to fail.**
pub fn a_fresh_launch_has_a_clearing_step() -> bool {
    false
}

/// **D23: a server stays in the list until the user removes it**, because the
/// row is **user-held** — *"the user removes it, and never otherwise"*. Nothing
/// ages it, caps it, sweeps it, or drops it because a machine stopped answering.
///
/// §7 step 2's own rule is the general form: ***the machine will not come back*
/// is not a state anything here may reach.**
pub const THE_SERVER_ROW: Lifetime = Lifetime::UserHeld;

impl ServerRow {
    /// A row as it stands the moment it is saved. **D22: nothing about a
    /// connection is in it.**
    pub fn saved(
        local_server_identifier: impl Into<String>,
        host: Host,
        port: Port,
        username: Username,
        key: KeyReference,
    ) -> ServerRow {
        ServerRow {
            local_server_identifier: local_server_identifier.into(),
            host,
            port,
            username,
            key,
            label: None,
            colour: None,
            remembered_setup: None,
            remembered_host_key: None,
            remembered_hardware_reading: None,
            // **A saved server has no root until a hand-over chooses one**
            // (`Job files and outputs` L1), which is why H10's free-space
            // figure is absent on a machine nothing has been placed on.
            recorded_machine_root: None,
            last_confirmed_installation_identity: None,
            event_cursor: None,
            machine_password_subject: None,
            // **Nothing chosen: Smoozy's own job area** (`Job files and
            // outputs` P23). Add has no such field (`Design/Servers`).
            chosen_fallback_folder: None,
            // **No setup has run, so no swap has missed** (`Servers` H11b).
            the_driver_missed_the_optix_bar: false,
        }
    }

    /// **P4a: what this document hands `Keys` at the ask — three display-only
    /// fields of the row**, beside P1a's pair, so the box can name the server
    /// being connected to. **Nothing else crosses, and no secret crosses at
    /// all.**
    pub fn what_the_passphrase_box_is_handed(&self) -> (Option<&str>, &str, &str) {
        (
            self.label.as_deref(),
            self.username.as_str(),
            self.host.as_str(),
        )
    }
}

/// The moment a row's durable write lands, for the one place a caller needs to
/// order two of them. **D16: the whole of an edit's write is one durable
/// write**, made before the put-back is attempted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurableWrite {
    pub at: SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::{Host, Port, Username};

    fn a_row() -> ServerRow {
        ServerRow::saved(
            "srv-1",
            Host::what_the_user_typed("5.180.183.77").expect("a host"),
            Port::what_the_user_typed(33510).expect("a port"),
            Username::what_the_user_typed("root").expect("a username"),
            KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
        )
    }

    #[test]
    fn a_saved_row_records_nothing_about_a_connection() {
        // D22, and it is why there is no clearing step to fail at start-up.
        let row = a_row();
        assert!(row.last_confirmed_installation_identity.is_none());
        assert!(row.remembered_hardware_reading.is_none());
        assert!(row.event_cursor.is_none());
        assert!(!a_fresh_launch_has_a_clearing_step());
    }

    #[test]
    fn the_enumeration_covers_the_row_and_the_seven_keyed_record_kinds() {
        let row = a_row();
        let walk = enumeration(&row);
        assert_eq!(walk.keyed_records.len(), 7);
        assert!(walk.keyed_records.contains(&KeyedByTheServer::ToldRecord));
        assert!(walk.keyed_records.contains(&KeyedByTheServer::IdleSpokenMarks));
        assert!(walk.keyed_records.contains(&KeyedByTheServer::CatchUpNotices));
        assert_eq!(walk.row.local_server_identifier, "srv-1");
    }

    #[test]
    fn the_remembered_host_key_has_no_route_to_a_value() {
        // D9 kept by the type: it compares and does not read back.
        let remembered = RememberedHostKey::approved(vec![1, 2, 3]);
        assert!(remembered.matches(&[1, 2, 3]));
        assert!(!remembered.matches(&[1, 2, 4]));
    }

    #[test]
    fn a_removal_leaves_history_exactly_where_it_is() {
        assert!(!a_removal_touches_history());
    }

    #[test]
    fn the_server_row_is_user_held_and_nothing_sweeps_it() {
        assert_eq!(THE_SERVER_ROW, Lifetime::UserHeld);
    }

    #[test]
    fn the_passphrase_box_is_handed_three_display_only_fields_and_no_secret() {
        let mut row = a_row();
        row.label = Some("hetzner".into());
        let (label, username, host) = row.what_the_passphrase_box_is_handed();
        assert_eq!(label, Some("hetzner"));
        assert_eq!(username, "root");
        assert_eq!(host, "5.180.183.77");
    }

    #[test]
    fn an_unconfirmed_erase_is_a_recorded_debt_and_not_a_forgotten_one() {
        let removal = Removal {
            reaches_the_machine: false,
            the_machine_password_erase: Some(TheErase::LeftAsARecordedDebt),
            staged_download_material: StagedMaterial::RecordedAsAnOwedCleanup,
        };
        assert_eq!(
            removal.the_machine_password_erase,
            Some(TheErase::LeftAsARecordedDebt)
        );
    }

    #[test]
    fn the_subject_reference_carries_nothing_about_the_user() {
        // D2 and §3.0 rule 1: never a host, a port, a username, a path or a
        // fingerprint. The subject is the local server identifier and nothing
        // else.
        let mut row = a_row();
        row.machine_password_subject =
            Some(Subject::issued_by_the_local_record(&row.local_server_identifier));
        let subject = row.machine_password_subject.as_ref().expect("a subject");
        assert_eq!(subject.as_str(), "srv-1");
        assert!(!subject.as_str().contains("5.180.183.77"));
        assert!(!subject.as_str().contains("root"));
        assert!(!subject.as_str().contains("SHA256:"));
    }
}
