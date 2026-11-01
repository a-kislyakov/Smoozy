//! §4.4 and §4.5 — what a replaced connection needs, and the identity read that
//! gates it.
//!
//! **Two things are here and they are the two this crate owed.**
//!
//! **C15's read**, which is the first half of `State, identity and recovery`
//! §2.3's check: the installation identity taken **on the pre-package surface**,
//! **from Smoozy's own state area**, and **never against a path a provider also
//! writes**. Until this landed nothing in the workspace read the identity at
//! all — `Servers`' own `TheInstallationIdentityRead` had no implementor outside
//! its tests, and its acting half said in as many words that both halves of the
//! check belong to modules built at step 6.
//!
//! **C12's difference**, which is what a supervisor that does not answer while
//! SSH is live stops and what it does not.
//!
//! # Both halves of §2.3's check are built, and this file is one of them
//!
//! **Nothing is owed here any more, and this paragraph replaces a constant that
//! said otherwise.** That constant named two things as missing — the framed
//! request a desktop sends to reach a supervisor, and `Servers`' seam having no
//! real end at all — and both were built on 2026-09-18. **The sending is
//! [`crate::placement::ask_for_the_health_answer`]**, in this crate, which
//! plants the carriage, sends `Rented-server runtime` G12's request to the entry
//! point on the command channel, and takes the link away again. **The seam's
//! real end is the desktop's own `servers::installation`**, which calls this
//! file's read for the first half and that function for the second, and composes
//! neither: the four outcomes stay `State, identity and recovery`'s.
//!
//! # Why the read is a subsystem read and not a command
//!
//! **§1.1 makes the pre-package surface two mechanisms**, and this is the second
//! one: the file-transfer subsystem. **The identity is a record Smoozy wrote and
//! the read is a read of that record**, so nothing is run on the machine for it
//! and nothing about the answer depends on a supervisor being alive — which is
//! exactly what C12 turns on, because the reading half of §2.3's check has to
//! work in the branch where the supervisor is not answering.
//!
//! # What *never against a path a provider also writes* is kept by
//!
//! **The recycle observation is exact about why the rule exists**: the boot hook
//! *"was not deleted but replaced"* by an 82-byte provider default, so **presence
//! is not a check**. C15 answers it by naming where the reading is taken from
//! rather than by testing what came back.
//!
//! **So the read takes the package area the placement recorded and composes the
//! record's own path inside it**, and there is no other route through this file:
//! no default, no search, no listing, and no path a caller can hand in whole.
//! **A machine that has had no placement has no such area to name**, which is the
//! absent branch and not a place to look harder.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §2.3, Y9, Y14.
//! `Rented-server runtime` G10, G12.

use smoozy_state::identities::{Identity, IdentityKind, InstallationCheck};

use crate::subsystem::client::Subsystem;
use crate::subsystem::SubsystemFailure;

/// **A6. An authenticated connection is not evidence about the machine at the
/// other end of it.**
///
/// **§4.5 is the whole of what this document does about that**, which is why the
/// clause is named here and nowhere else: this file is §4.5. **What keeps it is
/// the gate C14 and C15 compose between them**, and it is kept by a shape rather
/// than by a rule anybody has to remember.
///
/// - **C15 is the reading**, and it is a read of a record Smoozy wrote, in the
///   package area the placement recorded. A connection that opened, answered and
///   authenticated produces no value it can be answered from.
/// - **C14 is the branch**, and [`what_a_replaced_connection_does`] resumes
///   correlation on one outcome only. `State, identity and recovery`'s gate
///   composes that outcome from two halves — the record read off the machine and
///   the supervisor's own answer — and **the connection is neither of them**, so
///   there is no route by which reachability reaches *confirmed*.
///
/// **The recycle observation is why the clause exists**: a container's whole
/// installation was destroyed and the machine went on answering on the same
/// address and port, byte-identically, *"with nothing in the connection to
/// reveal it"*. **A Smoozy that reconnects and finds its address answering has
/// no signal that its whole installation is gone. It must check for its own
/// state, not for reachability.**
///
/// **This carries no behaviour of its own and must not be given any.** A6 states
/// what the document does not do, and the two clauses above are what does it.
pub const AN_AUTHENTICATED_CONNECTION_IS_EVIDENCE_ABOUT_NOTHING_BUT_ITSELF: () = ();

/// **The record the installation identity sits in, by name, inside the package
/// area.**
///
/// **`Rented-server runtime` G10 writes it and this reads it**, and the two
/// halves have to name the same thing. **Neither crate can name the other** —
/// that one takes this module's framing half and none of this crate — so the
/// name is written twice and a test below reads that crate's own source to hold
/// the two together. **That is the habit [`crate::release::composing`] already
/// uses** for the entry point's two copies.
pub const THE_RECORD_THE_IDENTITY_SITS_IN: &str = "installation";

/// **How many bytes of that record are read.**
///
/// An identity's written form is thirty-two characters and nothing else is in
/// the record. **The read is bounded rather than open** because a length taken
/// off a far end is a number the far end chose, which is the same reason
/// `smoozy_framing::framing::THE_LARGEST_FRAME` exists — and a record longer
/// than this is refused rather than truncated into something that reads back.
pub const THE_MOST_THE_RECORD_MAY_HOLD: usize = 256;

/// What C15's read found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheStateAreaHeld {
    /// The identity Smoozy's own state area holds on this machine.
    AnIdentity(Identity),
    /// **No record at all.** `State, identity and recovery` §2.3's *absent*
    /// branch: nothing of Smoozy's is on this machine — never prepared,
    /// recycled, reimaged, or a different machine at a re-used address.
    Nothing,
    /// **The record is there and does not read back as an identity.**
    ///
    /// **This is not the absent branch and must never be folded into it.** An
    /// absent record says nothing of Smoozy's is here; a record that will not
    /// read back says something is here and Smoozy cannot say what — and taking
    /// the absent branch for it would discard every machine-derived value on the
    /// strength of a reading that failed rather than of a machine that changed.
    ARecordThatDoesNotReadBack,
}

/// **C15. The identity is read on the pre-package surface, from Smoozy's own
/// state area, and never against a path a provider also writes.**
///
/// **The area is the one the placement recorded**, and the record's own path is
/// composed here from it and from [`THE_RECORD_THE_IDENTITY_SITS_IN`]. A caller
/// hands in the area and never the path, which is what keeps a provider-written
/// path from reaching this read at all.
///
/// **The path goes on the wire as bytes**, which every path on this surface does
/// — a leading dash, whitespace, a newline and a non-UTF-8 name are values
/// rather than syntax.
///
/// **A refusal from the far end is not read as absence.** The subsystem's own
/// *no such file* is the absent branch; anything else is the failure it is, and
/// is returned rather than turned into a reading.
pub async fn read_the_installation_identity<S>(
    subsystem: &mut Subsystem<S>,
    the_package_area_the_placement_recorded: &[u8],
) -> Result<WhatTheStateAreaHeld, SubsystemFailure>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send,
{
    let mut at = the_package_area_the_placement_recorded.to_vec();
    if !at.ends_with(b"/") {
        at.push(b'/');
    }
    at.extend_from_slice(THE_RECORD_THE_IDENTITY_SITS_IN.as_bytes());

    let handle = match subsystem.open_to_read(&at).await {
        Ok(handle) => handle,
        Err(SubsystemFailure::TheServerRefused(crate::subsystem::StatusCode::NoSuchFile)) => {
            return Ok(WhatTheStateAreaHeld::Nothing)
        }
        Err(why) => return Err(why),
    };
    let read = subsystem
        .read_at(&handle, 0, THE_MOST_THE_RECORD_MAY_HOLD)
        .await;
    subsystem.close(&handle).await?;
    let bytes = read?;

    Ok(the_identity_in(&bytes))
}

/// **What a record's bytes read back as**, split out so the reading rule is
/// testable without a machine.
///
/// **The trim is the one the writer's own side already does** — `Rented-server
/// runtime` G10 publishes the identity's text and its reader trims before
/// reading it back — so a trailing newline is not a record that does not read
/// back.
pub fn the_identity_in(bytes: &[u8]) -> WhatTheStateAreaHeld {
    if bytes.is_empty() {
        // **An empty record is not an absent one.** The record was opened, so
        // something is there; nothing readable came back.
        return WhatTheStateAreaHeld::ARecordThatDoesNotReadBack;
    }
    let Ok(text) = std::str::from_utf8(bytes) else {
        return WhatTheStateAreaHeld::ARecordThatDoesNotReadBack;
    };
    match Identity::read_back(IdentityKind::Installation, text.trim()) {
        Some(identity) => WhatTheStateAreaHeld::AnIdentity(identity),
        None => WhatTheStateAreaHeld::ARecordThatDoesNotReadBack,
    }
}

/// **The four steps C11 puts a replaced connection through**, named so that C12
/// has something to be about.
///
/// **This document does not perform the sequence** — `Servers` §12 decides when
/// a connection is a replacement and hands the label over, and that crate's
/// `transfers` module builds the four steps in order. What is here is C12's
/// answer about two of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichStep {
    /// **Step 1.** §4.5's gate. Nothing below runs before it.
    TheInstallationIdentityGate,
    /// **Step 2.** Capabilities negotiated afresh. Nothing negotiated on the
    /// previous connection is carried over.
    CapabilitiesNegotiatedAfresh,
    /// **Step 3.** Every outstanding pending-command record resolved by asking
    /// the machine for its identity under C8.
    PendingCommandRecordsResolved,
    /// **Step 4.** Transfer channels re-established for every transfer whose
    /// record says *interrupted* and which was interrupted within the current
    /// application run, each resumed under §7.4.
    TransferChannelsReEstablished,
}

impl WhichStep {
    /// C11's own order.
    pub const IN_ORDER: [WhichStep; 4] = [
        WhichStep::TheInstallationIdentityGate,
        WhichStep::CapabilitiesNegotiatedAfresh,
        WhichStep::PendingCommandRecordsResolved,
        WhichStep::TransferChannelsReEstablished,
    ];

    /// **C12. Steps 3 and 4 have different requirements, and the difference is
    /// deliberate.**
    ///
    /// **Step 3 needs the supervisor, because the record it asks about is the
    /// supervisor's.** **Step 4 does not**, because a transfer's staged material
    /// and its staging path are Smoozy's own, and the pre-package surface
    /// performs the same contract before hand-over as the transfer-support
    /// worker does after it.
    pub fn needs_the_supervisor(self) -> bool {
        matches!(self, WhichStep::PendingCommandRecordsResolved)
    }
}

/// **What a supervisor that does not answer while SSH is live stops, and what it
/// does not.**
///
/// **C12's own sentence**, and it is `The shape`'s: *"Browsing, file operations,
/// readings, and transfers remain available; the pre-package surface carries
/// transfers through publication and cleanup under the same contract while the
/// supervisor is unavailable."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhatAReplacedConnectionDoes {
    /// **Step 3.** Whether outstanding pending-command records are resolved on
    /// this connection.
    pub command_correlation_resumes: bool,
    /// **Step 4.** Whether interrupted transfers are re-established and resumed.
    pub transfers_resume: bool,
    /// **C14's branch, from `State, identity and recovery` §2.3.** Whether every
    /// pending-command record, transfer record and staged partial held against
    /// this server row is disqualified.
    pub everything_held_against_the_row_is_disqualified: bool,
}

/// **C12, against `State, identity and recovery` §2.3's four outcomes.**
///
/// **The gate is step 1 and it runs first**, so this takes the outcome of that
/// gate and answers for the two steps C12 is about.
///
/// - **Present and equal, confirmed.** Correlation resumes, staged material is
///   Smoozy's, transfers resume.
/// - **Absent**, or **present and different.** Everything held against the row
///   is disqualified: not resumed, not asked about, not published, and not read
///   as a partial of anything.
/// - **Present, and the supervisor did not answer.** **Nothing is discarded.**
///   Command correlation does not resume, because its records are the
///   supervisor's. **Transfers do resume**, because the identity being present
///   is what says the material is Smoozy's and the contract needs the supervisor
///   neither before hand-over nor during a supervisor outage.
pub fn what_a_replaced_connection_does(gate: InstallationCheck) -> WhatAReplacedConnectionDoes {
    match gate {
        InstallationCheck::PresentAndEqualConfirmed => WhatAReplacedConnectionDoes {
            command_correlation_resumes: true,
            transfers_resume: true,
            everything_held_against_the_row_is_disqualified: false,
        },
        InstallationCheck::Absent | InstallationCheck::PresentAndDifferent => {
            WhatAReplacedConnectionDoes {
                command_correlation_resumes: false,
                transfers_resume: false,
                everything_held_against_the_row_is_disqualified: true,
            }
        }
        InstallationCheck::PresentAndTheSupervisorDidNotAnswer => WhatAReplacedConnectionDoes {
            command_correlation_resumes: false,
            transfers_resume: true,
            everything_held_against_the_row_is_disqualified: false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn an_identity() -> Identity {
        Identity::issue(IdentityKind::Installation).expect("the desk has randomness")
    }

    /// **C15's read reads back exactly what `Rented-server runtime` G10
    /// wrote**, trailing newline and all.
    #[test]
    fn the_record_reads_back_as_the_identity_the_machine_holds() {
        let placed = an_identity();
        assert_eq!(
            the_identity_in(placed.as_text().as_bytes()),
            WhatTheStateAreaHeld::AnIdentity(placed.clone())
        );
        assert_eq!(
            the_identity_in(format!("{}\n", placed.as_text()).as_bytes()),
            WhatTheStateAreaHeld::AnIdentity(placed)
        );
    }

    /// **A record that will not read back is its own answer and never the
    /// absent one**, because the absent branch discards every machine-derived
    /// value and this one is a reading that failed.
    #[test]
    fn a_record_that_does_not_read_back_is_never_folded_into_absent() {
        for bytes in [
            &b""[..],
            &b"\n"[..],
            &b"not an identity"[..],
            &[0xff, 0xfe, 0xfd][..],
        ] {
            assert_eq!(
                the_identity_in(bytes),
                WhatTheStateAreaHeld::ARecordThatDoesNotReadBack,
                "{bytes:?}"
            );
        }
    }

    /// **The two crates name the same record.**
    ///
    /// `Rented-server runtime` G10 writes it and this reads it, and neither
    /// crate can name the other — so the guard is a reading of that crate's own
    /// source.
    #[test]
    fn the_records_two_copies_agree() {
        let machine_side = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../Rented-server runtime/src/the_program.rs");
        let text = std::fs::read_to_string(&machine_side)
            .expect("the machine program's own file is readable");
        let expected = format!("\"{THE_RECORD_THE_IDENTITY_SITS_IN}\"");
        assert!(
            text.contains(&expected),
            "the machine no longer holds its installation identity at {expected}, so this \
             crate's copy has drifted"
        );
    }

    /// **C11's four steps are four and in the plan's own order**, which is what
    /// C12 is about two of.
    #[test]
    fn the_four_steps_are_in_the_plans_own_order() {
        for (at, step) in WhichStep::IN_ORDER.iter().enumerate() {
            let where_the_plan_puts_it = match step {
                WhichStep::TheInstallationIdentityGate => 0,
                WhichStep::CapabilitiesNegotiatedAfresh => 1,
                WhichStep::PendingCommandRecordsResolved => 2,
                WhichStep::TransferChannelsReEstablished => 3,
            };
            assert_eq!(at, where_the_plan_puts_it, "{step:?}");
        }
    }

    /// **C12: exactly one of the four steps needs the supervisor.**
    #[test]
    fn step_three_needs_the_supervisor_and_step_four_does_not() {
        assert!(WhichStep::PendingCommandRecordsResolved.needs_the_supervisor());
        assert!(!WhichStep::TransferChannelsReEstablished.needs_the_supervisor());
        assert_eq!(
            WhichStep::IN_ORDER
                .iter()
                .filter(|step| step.needs_the_supervisor())
                .count(),
            1
        );
    }

    /// **C12's own sentence: a supervisor that does not answer while SSH is
    /// live stops command correlation and does not stop transfers.**
    #[test]
    fn a_supervisor_that_does_not_answer_stops_correlation_and_never_transfers() {
        let what =
            what_a_replaced_connection_does(InstallationCheck::PresentAndTheSupervisorDidNotAnswer);
        assert!(!what.command_correlation_resumes);
        assert!(what.transfers_resume);
        assert!(!what.everything_held_against_the_row_is_disqualified);
    }

    /// **The confirmed branch resumes both**, and the two that discard resume
    /// neither.
    #[test]
    fn the_confirmed_branch_resumes_both_and_the_two_that_discard_resume_neither() {
        let confirmed =
            what_a_replaced_connection_does(InstallationCheck::PresentAndEqualConfirmed);
        assert!(confirmed.command_correlation_resumes);
        assert!(confirmed.transfers_resume);
        assert!(!confirmed.everything_held_against_the_row_is_disqualified);

        for gate in [
            InstallationCheck::Absent,
            InstallationCheck::PresentAndDifferent,
        ] {
            let what = what_a_replaced_connection_does(gate);
            assert!(!what.command_correlation_resumes);
            assert!(!what.transfers_resume);
            assert!(what.everything_held_against_the_row_is_disqualified);
        }
    }

    /// **The gate's own answer about machine-derived values and this one agree**
    /// on which outcomes discard, so the two are not a place two readers could
    /// differ.
    #[test]
    fn what_this_answers_agrees_with_the_gates_own_rule_about_machine_derived_values() {
        use smoozy_state::identities::MachineDerivedValue;
        for gate in [
            InstallationCheck::PresentAndEqualConfirmed,
            InstallationCheck::Absent,
            InstallationCheck::PresentAndDifferent,
            InstallationCheck::PresentAndTheSupervisorDidNotAnswer,
        ] {
            assert_eq!(
                what_a_replaced_connection_does(gate).everything_held_against_the_row_is_disqualified,
                gate.what_becomes_of_machine_derived_values() == MachineDerivedValue::Disqualified,
                "{gate:?}"
            );
        }
    }

    /// **A6, held rather than stated: no outcome of the gate can be reached
    /// from an authenticated connection.**
    ///
    /// The gate is composed from the record read off the machine and the
    /// supervisor's own answer. **Where the record is absent, no answer from
    /// anything — a supervisor included — makes it *confirmed***, which is the
    /// case a machine that is answering and is not Smoozy's takes.
    #[test]
    fn reachability_reaches_no_outcome_of_the_gate() {
        let placed = an_identity();
        for the_supervisor_answered in [true, false] {
            assert_eq!(
                InstallationCheck::from_the_two_halves(
                    None,
                    Some(&placed),
                    the_supervisor_answered
                ),
                InstallationCheck::Absent,
                "a machine that answers and holds no record of Smoozy's is absent"
            );
        }
        let what = what_a_replaced_connection_does(InstallationCheck::Absent);
        assert!(!what.command_correlation_resumes);
        assert!(!what.transfers_resume);
        assert!(what.everything_held_against_the_row_is_disqualified);
    }
}
