//! Identities — `State, identity and recovery` §2.
//!
//! §2.1's seven rules bind every one of them, and the two that decide the
//! shape of this file are rule 1 — **issued once, by one named issuer, and
//! never reused** — and rule 2, **opaque: it carries no meaning that anything
//! reads**. Nothing parses one, sorts by one, or infers anything from two
//! being adjacent.
//!
//! The other five are rules about what an identity is **not** made of, and
//! random bytes keep all five by construction: never derived from anything the
//! user typed (rule 3), never from anything read off a machine (rule 4), never
//! a process identifier (rule 5), never from a reading of the user's computer
//! (rule 6), and never a path (rule 7).

use std::fmt;

/// The rows of §2.2's register a kind is needed for.
///
/// **Most of them are issued on this side of the wire**, and the machine-side
/// issuers — the supervisor's execution-group runtime identity, the Telegram
/// keys — are `Rented-server runtime`'s and `Telegram`'s and are not made here.
///
/// **[`IdentityKind::Event`] is the one machine-side row that is here, and it
/// entered on 2026-09-06 with the event journal.** §2.2's register makes the
/// **supervisor** its issuer, so nothing on this side ever calls
/// [`Identity::issue`] for one — but §4 is this document's, the journal's rules
/// and form are [`crate::journal`]'s, and an event identity is an identity under
/// §2.1's seven rules like every other. A kind of its own is what keeps rule 1
/// checkable: [`crate::journal::AnEvent::journalled`] refuses any identity that
/// is not one, so an event cannot be journalled under a job's or an operation's
/// identity by a caller that reached for the wrong one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdentityKind {
    /// Issued by the local record when a server row is created.
    LocalServer,
    /// Issued by the local record when a key is added.
    LocalKey,
    /// Issued by the local record at the first store of a kind against a
    /// subject. The value is in the operating system's store; this names it.
    SecretStoreSubject,
    /// Issued by the desktop application at submission, **before the
    /// submission is sent**. Y1: unique across every machine and every rental,
    /// not per machine.
    Job,
    /// Issued by the desktop application when an all-scenes submission is
    /// composed.
    Group,
    /// Issued by the desktop application at hand-over (§2.3), unique across
    /// every installation Smoozy has ever placed.
    Installation,
    /// Issued by the desktop application at application start, before any
    /// feature module composes. Y5: **random bytes**, and Y7 warns that it is
    /// not the supervisor's run identifier — the two are never compared and
    /// neither stands in for the other.
    Run,
    /// Issued by whatever raises a condition, when a condition key is first
    /// raised against (§8.3).
    Condition,
    /// Issued by the initiating side when a transfer is arranged.
    Transfer,
    /// Issued by the side that initiates an operation, when the command is
    /// composed and before it is sent. Y3: it is the idempotency key, and
    /// there is not a second value beside it.
    Operation,
    /// Issued by **the supervisor**, *"when the event is journalled, before it
    /// is offered to any reader"*, unique within one machine's journal, and held
    /// in the event journal beside its sequence number. §4 E4 is what it is for:
    /// **every reader applies an event idempotently by its event identity.**
    Event,
}

/// How many random bytes an identity is made of.
///
/// Sixteen, which is what `Telegram` R1 already fixes for its own two
/// identities and what makes a collision not a case anybody has to handle.
/// Nothing reads the number and nothing parses the value; it is here so every
/// issuer on this side makes the same size rather than each choosing.
pub const IDENTITY_BYTES: usize = 16;

/// One identity. Opaque (§2.1 rule 2): the only things anything does with one
/// are compare it, carry it, and write it down.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Identity {
    kind: IdentityKind,
    bytes: [u8; IDENTITY_BYTES],
}

impl Identity {
    /// Issues a fresh one. **The issuer is whoever calls this**, and §2.2's
    /// register says which that is for each kind; naming the kind here is what
    /// makes a mismatched comparison a compile-time impossibility rather than
    /// a bug nobody sees.
    pub fn issue(kind: IdentityKind) -> Result<Self, NoRandomness> {
        let mut bytes = [0u8; IDENTITY_BYTES];
        getrandom::fill(&mut bytes).map_err(|_| NoRandomness)?;
        Ok(Identity { kind, bytes })
    }

    /// Reads one back from the local record, a machine's state area, or a
    /// message. It is not an issuing and does not pretend to be — Y2 names the
    /// case: adoption takes the identity the machine already holds, and that
    /// is the second entry of one identity rather than a second identity.
    pub fn read_back(kind: IdentityKind, text: &str) -> Option<Self> {
        if text.len() != IDENTITY_BYTES * 2 {
            return None;
        }
        let mut bytes = [0u8; IDENTITY_BYTES];
        for (byte, pair) in bytes.iter_mut().zip(text.as_bytes().chunks(2)) {
            let digits = std::str::from_utf8(pair).ok()?;
            *byte = u8::from_str_radix(digits, 16).ok()?;
        }
        Some(Identity { kind, bytes })
    }

    pub fn kind(&self) -> IdentityKind {
        self.kind
    }

    /// The written form: lower-case hexadecimal, which is a rendering and not
    /// a meaning. **Nothing parses it for anything but reading it back.**
    pub fn as_text(&self) -> String {
        let mut text = String::with_capacity(IDENTITY_BYTES * 2);
        for byte in self.bytes {
            text.push_str(&format!("{byte:02x}"));
        }
        text
    }
}

impl fmt::Debug for Identity {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "{:?}({})", self.kind, self.as_text())
    }
}

/// The system would not supply random bytes. An identity is never composed
/// from anything else instead — §2.1 rules 3 to 6 close every substitute a
/// builder would reach for — so the act that wanted one does not happen and
/// says so.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoRandomness;

/// The installation identity check — `State, identity and recovery` §2.3 Y9.
///
/// The check has two halves: the value read from Smoozy's own state area, and
/// the supervisor answering to confirm it. **These four outcomes are the
/// complete set.** Y14: the check is never made against a file at a path a
/// provider also writes, and never against reachability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallationCheck {
    /// This is the installation Smoozy placed. Every machine-derived value the
    /// local record holds against this server row is current and may be used.
    PresentAndEqualConfirmed,
    /// Nothing of Smoozy's is on this machine — never prepared, recycled,
    /// reimaged, or a different machine at a re-used address.
    Absent,
    /// A different installation. **No state written by another installation is
    /// ever adopted, merged, or read as Smoozy's**, which is also the only
    /// defence against a previous tenant's leavings.
    PresentAndDifferent,
    /// The identity was read and the check is unconfirmed. **Nothing is
    /// discarded**: every machine-derived value stays exactly as it is,
    /// neither used as current nor thrown away. The supervisor-unavailable
    /// condition is raised and desktop reconciliation stops at step 1.
    PresentAndTheSupervisorDidNotAnswer,
}

/// What Y13 means by *discarded*, and **it is not deleted**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineDerivedValue {
    /// May be read, shown and used as authority for what this server is.
    Current,
    /// **Disqualified**: never read, never shown, never used as authority for
    /// anything, and never counted as state Smoozy holds about that machine.
    /// It is not removed on the strength of the mismatch — nothing sweeps for
    /// it — and it ends with its subject under the subject-held rule, the
    /// server row being that subject.
    Disqualified,
    /// Neither. The reading was unconfirmed, so the value stands as it is.
    StandsUnchanged,
}

impl InstallationCheck {
    /// Y10: **every durably remembered machine-derived value carries the
    /// installation identity it was read under**, which is what makes this a
    /// single act instead of a list a builder must keep in step.
    ///
    /// Y11 names the two values in the server row that are outside it, and
    /// each for a different reason: the remembered host key, which is read
    /// before any connection and cannot carry one; and the remembered setup
    /// bit, which is the user's own instruction rather than a machine-derived
    /// value at all.
    pub fn what_becomes_of_machine_derived_values(self) -> MachineDerivedValue {
        match self {
            InstallationCheck::PresentAndEqualConfirmed => MachineDerivedValue::Current,
            InstallationCheck::Absent | InstallationCheck::PresentAndDifferent => {
                MachineDerivedValue::Disqualified
            }
            InstallationCheck::PresentAndTheSupervisorDidNotAnswer => {
                MachineDerivedValue::StandsUnchanged
            }
        }
    }

    /// Whether desktop reconciliation may go past its first step. Only the
    /// unconfirmed outcome stops it there; the two that discard carry on, and
    /// `Server Setup` owns what the user is offered.
    pub fn reconciliation_stops_at_step_one(self) -> bool {
        matches!(self, InstallationCheck::PresentAndTheSupervisorDidNotAnswer)
    }

    /// **Y9's four outcomes, composed from the check's own two halves and from
    /// nothing else.**
    ///
    /// The two halves are the clause's own: **the value read from Smoozy's own
    /// state area**, and **the supervisor answering to confirm it**. Whoever
    /// takes the two readings passes them here rather than composing an outcome
    /// itself, so the set stays the complete set Y9 says it is.
    ///
    /// **The order the three arguments are weighed in is the clause's.**
    /// *Absent* is decided by the reading alone and the supervisor is not
    /// consulted for it — an absent identity is *"never prepared, recycled,
    /// reimaged, or a different machine at a re-used address"* whatever the
    /// supervisor is doing, and Y14 forbids the check resting on reachability.
    /// **A value that is present and different is likewise decided by the
    /// reading alone**: no state written by another installation is ever read as
    /// Smoozy's, and a supervisor answering would not make it Smoozy's.
    /// **Only the equal case asks the second half**, which is where the
    /// confirmation is the whole of what separates the fourth outcome from the
    /// first.
    ///
    /// **`None` for what the local record remembers is a server row that has
    /// never had an installation placed against it.** A machine holding an
    /// identity Smoozy does not remember is *present and different*, which is
    /// the previous-tenant case; a machine holding nothing where Smoozy
    /// remembers nothing is *absent*.
    pub fn from_the_two_halves(
        read_from_smoozys_own_state_area: Option<&Identity>,
        what_the_local_record_remembers: Option<&Identity>,
        the_supervisor_answered: bool,
    ) -> InstallationCheck {
        match read_from_smoozys_own_state_area {
            None => InstallationCheck::Absent,
            Some(on_the_machine) => {
                if what_the_local_record_remembers != Some(on_the_machine) {
                    return InstallationCheck::PresentAndDifferent;
                }
                if the_supervisor_answered {
                    InstallationCheck::PresentAndEqualConfirmed
                } else {
                    InstallationCheck::PresentAndTheSupervisorDidNotAnswer
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_issued_identities_never_match() {
        let one = Identity::issue(IdentityKind::Job).unwrap();
        let two = Identity::issue(IdentityKind::Job).unwrap();
        assert_ne!(one, two);
    }

    #[test]
    fn an_identity_reads_back_as_itself() {
        let issued = Identity::issue(IdentityKind::Run).unwrap();
        let read = Identity::read_back(IdentityKind::Run, &issued.as_text()).unwrap();
        assert_eq!(issued, read);
    }

    #[test]
    fn one_kind_never_equals_another_however_alike_the_bytes() {
        // Y7: the run identity and the supervisor's run identifier are one
        // word apart, and neither ever stands in for the other. Kinds are part
        // of the value so a comparison across two of them cannot come out
        // true by accident.
        let run = Identity::issue(IdentityKind::Run).unwrap();
        let same_bytes_other_kind = Identity::read_back(IdentityKind::Job, &run.as_text()).unwrap();
        assert_ne!(run, same_bytes_other_kind);
    }

    #[test]
    fn a_malformed_text_reads_back_as_nothing() {
        assert!(Identity::read_back(IdentityKind::Job, "").is_none());
        assert!(Identity::read_back(IdentityKind::Job, "not hexadecimal!").is_none());
        assert!(Identity::read_back(IdentityKind::Job, "ab").is_none());
    }

    #[test]
    fn the_written_form_is_the_full_sixteen_bytes() {
        let issued = Identity::issue(IdentityKind::Installation).unwrap();
        assert_eq!(issued.as_text().len(), IDENTITY_BYTES * 2);
        assert!(issued.as_text().chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn only_a_confirmed_check_makes_machine_derived_values_current() {
        assert_eq!(
            InstallationCheck::PresentAndEqualConfirmed.what_becomes_of_machine_derived_values(),
            MachineDerivedValue::Current
        );
    }

    #[test]
    fn both_discarding_outcomes_disqualify_rather_than_delete() {
        for outcome in [
            InstallationCheck::Absent,
            InstallationCheck::PresentAndDifferent,
        ] {
            assert_eq!(
                outcome.what_becomes_of_machine_derived_values(),
                MachineDerivedValue::Disqualified
            );
        }
    }

    #[test]
    fn an_unconfirmed_reading_never_stands_in_for_either_branch() {
        let unconfirmed = InstallationCheck::PresentAndTheSupervisorDidNotAnswer;
        assert_eq!(
            unconfirmed.what_becomes_of_machine_derived_values(),
            MachineDerivedValue::StandsUnchanged
        );
        assert!(unconfirmed.reconciliation_stops_at_step_one());
    }

    #[test]
    fn nothing_but_the_unconfirmed_reading_stops_reconciliation() {
        for outcome in [
            InstallationCheck::PresentAndEqualConfirmed,
            InstallationCheck::Absent,
            InstallationCheck::PresentAndDifferent,
        ] {
            assert!(!outcome.reconciliation_stops_at_step_one());
        }
    }

    /// **Y9's four outcomes, one by one, from the two halves.**
    #[test]
    fn the_four_outcomes_are_composed_from_the_two_halves_and_nothing_else() {
        let placed = Identity::issue(IdentityKind::Installation).unwrap();
        let somebody_elses = Identity::issue(IdentityKind::Installation).unwrap();

        assert_eq!(
            InstallationCheck::from_the_two_halves(Some(&placed), Some(&placed), true),
            InstallationCheck::PresentAndEqualConfirmed
        );
        assert_eq!(
            InstallationCheck::from_the_two_halves(Some(&placed), Some(&placed), false),
            InstallationCheck::PresentAndTheSupervisorDidNotAnswer
        );
        assert_eq!(
            InstallationCheck::from_the_two_halves(Some(&somebody_elses), Some(&placed), true),
            InstallationCheck::PresentAndDifferent
        );
        assert_eq!(
            InstallationCheck::from_the_two_halves(None, Some(&placed), true),
            InstallationCheck::Absent
        );
    }

    /// **Y14: the check never rests on reachability, and the two branches that
    /// discard are decided by the reading alone.**
    ///
    /// A supervisor answering does not make somebody else's installation
    /// Smoozy's, and a supervisor not answering does not turn an absent identity
    /// into an unconfirmed one — *"an unconfirmed reading is never allowed to
    /// stand in for either of the two branches that discard."*
    #[test]
    fn the_supervisors_answer_moves_neither_of_the_two_discarding_branches() {
        let placed = Identity::issue(IdentityKind::Installation).unwrap();
        let somebody_elses = Identity::issue(IdentityKind::Installation).unwrap();
        for answered in [true, false] {
            assert_eq!(
                InstallationCheck::from_the_two_halves(None, Some(&placed), answered),
                InstallationCheck::Absent
            );
            assert_eq!(
                InstallationCheck::from_the_two_halves(
                    Some(&somebody_elses),
                    Some(&placed),
                    answered
                ),
                InstallationCheck::PresentAndDifferent
            );
        }
    }

    /// **A machine holding an identity against a row that remembers none is
    /// *present and different***, which is the previous-tenant case Y9 names.
    #[test]
    fn a_machine_holding_an_identity_smoozy_never_placed_is_present_and_different() {
        let a_leaving = Identity::issue(IdentityKind::Installation).unwrap();
        assert_eq!(
            InstallationCheck::from_the_two_halves(Some(&a_leaving), None, true),
            InstallationCheck::PresentAndDifferent
        );
        assert_eq!(
            InstallationCheck::from_the_two_halves(None, None, true),
            InstallationCheck::Absent
        );
    }
}
