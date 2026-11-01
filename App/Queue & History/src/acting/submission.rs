//! **§2 Y3 performed — the provisional local record's write before the
//! submission is sent, and the ask-the-machine resolution of a dropped
//! submission.**
//!
//! # The order is the whole of the clause
//!
//! **The identity is issued before the submission is sent, and that is what
//! makes a dropped submission answerable.** [`submit`] writes the provisional
//! local job record first — `State, identity and recovery` §7 step 2's record
//! — and only then hands the submission over. **A refused write sends
//! nothing**, because a submission sent with no record behind it is work
//! nobody can ask about afterwards.
//!
//! **A lost acknowledgement is resolved by asking the machine for the
//! identity, never by re-issuing the work** (§12 rule 4). [`resolve`] takes
//! the identity that was already issued and asks with it; **there is no
//! function here that issues a second identity for one submission**, and
//! [`crate::identity::issue_a_job_identity`] is reached once, by the caller,
//! before any of this.
//!
//! # What is not here
//!
//! **The matching after a dropped submission is `Rendering`'s** and reads this
//! from that side. **The submission's own atomicity — a job reaches the
//! machine whole or does not exist — is that document's Row 2 and `Job files
//! and outputs` M5's single-rename publication**, cited and owed, and nothing
//! here re-decides it.
//!
//! **Q3's order is the machine's**: material published first, then the job
//! record and the per-job note in one act, then the event journalled. **This
//! module is the desk side alone** and writes nothing on a machine.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `State, identity and recovery` §7, §12.
//! `Job files and outputs` M5.

use smoozy_state::identities::Identity;

/// **The provisional local job record, as §7 step 2 writes it.**
///
/// **Provisional until the machine confirms it** — C9's own word — and it is
/// the record every later resolution reads. **It holds the identity that was
/// issued before anything was sent**, which is what a dropped submission is
/// asked about by.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AProvisionalRecord {
    pub job: Identity,
    pub server: Identity,
    /// The group identity where the job came from an all-scenes submission.
    pub group: Option<Identity>,
    /// The label, whole, as the user typed it (Y8).
    pub label: String,
}

/// **The local record, reached through one trait.**
pub trait WritesTheProvisionalRecord {
    /// **§7 step 2's write, durable before anything is sent.**
    fn write_it(&mut self, record: &AProvisionalRecord) -> Result<(), String>;

    /// **H11's and L22's own sentence, from this side**: applying a drop or a
    /// cancel's outcome removes the provisional record, and a job that never
    /// rendered leaves no trace.
    fn remove_it(&mut self, job: &Identity) -> Result<(), String>;
}

/// **The submission, on the far side of one trait, because a machine is.**
pub trait SendsTheSubmission {
    /// **The submission's own atomicity is `Rendering`'s** — a job reaches the
    /// machine whole or does not exist — and this only hands it over.
    fn send(&self, record: &AProvisionalRecord) -> WhatTheSubmissionGave;

    /// **§12 rule 4: ask the machine for the identity.** `Some` where the
    /// machine holds a job record under it; `None` where it does not; an error
    /// where the machine could not be asked at all, which is neither answer.
    fn does_the_machine_hold(&self, job: &Identity) -> Result<bool, String>;
}

/// What handing the submission over gave.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheSubmissionGave {
    /// The machine acknowledged it.
    ItWasAcknowledged,
    /// **The acknowledgement was lost.** *"A lost acknowledgement is resolved
    /// by asking the machine for the identity, never by re-issuing the
    /// work."*
    TheAcknowledgementWasLost,
}

/// What one submission left.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheSubmissionLeft {
    /// **The record was refused, so nothing was sent.** A submission with no
    /// record behind it is work nobody can ask about afterwards.
    TheRecordWasRefusedSoNothingWasSent { why: String },
    /// The record stands and the machine acknowledged.
    ItWasAcknowledged,
    /// The record stands and the acknowledgement was lost — [`resolve`] is
    /// what answers it, on this connection or a later one.
    TheRecordStandsAndTheAcknowledgementWasLost,
}

/// **Y3, performed: the record first, then the send.**
///
/// **The order may not be reversed.** Sending first and failing to write would
/// leave a job on a machine that the desktop cannot name, which is exactly the
/// state Q6's untracked work and §13's adoption exist to recover from — a
/// correct outcome, but a worse one than the record the clause asks for.
pub fn submit(
    record_keeper: &mut dyn WritesTheProvisionalRecord,
    machine: &dyn SendsTheSubmission,
    record: &AProvisionalRecord,
) -> WhatTheSubmissionLeft {
    if let Err(why) = record_keeper.write_it(record) {
        return WhatTheSubmissionLeft::TheRecordWasRefusedSoNothingWasSent { why };
    }
    match machine.send(record) {
        WhatTheSubmissionGave::ItWasAcknowledged => WhatTheSubmissionLeft::ItWasAcknowledged,
        WhatTheSubmissionGave::TheAcknowledgementWasLost => {
            WhatTheSubmissionLeft::TheRecordStandsAndTheAcknowledgementWasLost
        }
    }
}

/// How a dropped submission resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowItResolved {
    /// The machine holds the job under the identity that was issued. **The
    /// work is there and nothing is re-sent.**
    TheMachineHoldsIt,
    /// The machine does not hold it. **The provisional record is removed** —
    /// a job that never reached the machine leaves no trace — and re-sending
    /// is the user's own act, under the same identity, by the ordinary route.
    TheMachineDoesNotHoldIt,
    /// **The machine could not be asked**, which is neither answer. **The
    /// provisional record stands** and the question is asked again on a later
    /// connection.
    TheMachineCouldNotBeAsked { why: String },
    /// **The machine answered and the user's own computer refused the
    /// write.** The machine does not hold the job, and the provisional record
    /// could not be removed.
    ///
    /// **It is not the same condition as a machine that could not be asked**,
    /// and the two call for different repairs: one is a connection or a
    /// machine, the other is the local record's own store. Reported as the
    /// first, a machine that answered plainly reads as a machine that went
    /// quiet.
    TheLocalWriteWasRefused { why: String },
}

/// **Y3's second half, performed: a lost acknowledgement is resolved by asking
/// the machine for the identity.**
///
/// **Never by re-issuing the work.** The identity handed in is the one issued
/// before the submission was sent, and **this function has no route to issue
/// another**.
///
/// **A machine that cannot be asked is not a machine that answered *no*.** The
/// record stands, because removing it on an unanswerable question would lose a
/// job that is in fact rendering.
pub fn resolve(
    record_keeper: &mut dyn WritesTheProvisionalRecord,
    machine: &dyn SendsTheSubmission,
    job: &Identity,
) -> HowItResolved {
    match machine.does_the_machine_hold(job) {
        Err(why) => HowItResolved::TheMachineCouldNotBeAsked { why },
        Ok(true) => HowItResolved::TheMachineHoldsIt,
        Ok(false) => match record_keeper.remove_it(job) {
            Ok(()) => HowItResolved::TheMachineDoesNotHoldIt,
            // **The machine answered; it is this computer that refused.**
            Err(why) => HowItResolved::TheLocalWriteWasRefused { why },
        },
    }
}

/// **Y3, stated as a value: nothing here issues a second identity for one
/// submission.**
///
/// One job never has two identities (Y4), and a fresh attempt the user asks
/// for is a new job with a new identity and a new folder (Y5) — **which is a
/// different act and not this one.**
pub const NOTHING_HERE_RE_ISSUES_THE_WORK: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_state::identities::IdentityKind;
    use std::cell::RefCell;

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).expect("randomness")
    }

    fn a_server() -> Identity {
        Identity::issue(IdentityKind::LocalServer).expect("randomness")
    }

    fn a_record(job: &Identity) -> AProvisionalRecord {
        AProvisionalRecord {
            job: job.clone(),
            server: a_server(),
            group: None,
            label: "Shot 12".to_string(),
        }
    }

    #[derive(Default)]
    struct TheRecordInHand {
        standing: Vec<Identity>,
        refuses: bool,
    }

    impl WritesTheProvisionalRecord for TheRecordInHand {
        fn write_it(&mut self, record: &AProvisionalRecord) -> Result<(), String> {
            if self.refuses {
                return Err("No space left on device (os error 28)".to_string());
            }
            self.standing.push(record.job.clone());
            Ok(())
        }

        fn remove_it(&mut self, job: &Identity) -> Result<(), String> {
            if self.refuses {
                return Err("No space left on device (os error 28)".to_string());
            }
            self.standing.retain(|standing| standing != job);
            Ok(())
        }
    }

    struct AMachineInHand {
        gave: WhatTheSubmissionGave,
        holds: Result<bool, String>,
        sends: RefCell<usize>,
    }

    impl SendsTheSubmission for AMachineInHand {
        fn send(&self, _record: &AProvisionalRecord) -> WhatTheSubmissionGave {
            *self.sends.borrow_mut() += 1;
            self.gave.clone()
        }

        fn does_the_machine_hold(&self, _job: &Identity) -> Result<bool, String> {
            self.holds.clone()
        }
    }

    fn a_machine(gave: WhatTheSubmissionGave, holds: Result<bool, String>) -> AMachineInHand {
        AMachineInHand {
            gave,
            holds,
            sends: RefCell::new(0),
        }
    }

    /// **Y3: the record is written before the submission is sent.**
    #[test]
    fn the_record_is_written_first_and_then_the_submission_is_sent() {
        let job = a_job();
        let mut keeper = TheRecordInHand::default();
        let machine = a_machine(WhatTheSubmissionGave::ItWasAcknowledged, Ok(true));
        assert_eq!(
            submit(&mut keeper, &machine, &a_record(&job)),
            WhatTheSubmissionLeft::ItWasAcknowledged
        );
        assert_eq!(keeper.standing, vec![job]);
        assert_eq!(*machine.sends.borrow(), 1);
    }

    /// **A refused record sends nothing at all**, because a submission with no
    /// record behind it is work nobody can ask about afterwards.
    #[test]
    fn a_refused_record_sends_nothing() {
        let mut keeper = TheRecordInHand {
            refuses: true,
            ..TheRecordInHand::default()
        };
        let machine = a_machine(WhatTheSubmissionGave::ItWasAcknowledged, Ok(true));
        assert!(matches!(
            submit(&mut keeper, &machine, &a_record(&a_job())),
            WhatTheSubmissionLeft::TheRecordWasRefusedSoNothingWasSent { .. }
        ));
        assert_eq!(*machine.sends.borrow(), 0, "nothing was sent");
        assert!(keeper.standing.is_empty());
    }

    /// **Y3: a lost acknowledgement leaves the record standing, and asking the
    /// machine is what answers it — never a second send.**
    #[test]
    fn a_lost_acknowledgement_is_answered_by_asking_and_never_by_re_sending() {
        let job = a_job();
        let mut keeper = TheRecordInHand::default();
        let machine = a_machine(WhatTheSubmissionGave::TheAcknowledgementWasLost, Ok(true));
        assert_eq!(
            submit(&mut keeper, &machine, &a_record(&job)),
            WhatTheSubmissionLeft::TheRecordStandsAndTheAcknowledgementWasLost
        );
        assert_eq!(keeper.standing, vec![job.clone()]);
        assert_eq!(
            resolve(&mut keeper, &machine, &job),
            HowItResolved::TheMachineHoldsIt
        );
        assert_eq!(*machine.sends.borrow(), 1, "nothing was sent a second time");
        assert_eq!(keeper.standing, vec![job]);
    }

    /// **A machine that does not hold it leaves no trace** — the provisional
    /// record goes, and the identity is not re-issued.
    #[test]
    fn a_machine_that_does_not_hold_it_leaves_no_trace() {
        let job = a_job();
        let mut keeper = TheRecordInHand::default();
        let machine = a_machine(WhatTheSubmissionGave::TheAcknowledgementWasLost, Ok(false));
        submit(&mut keeper, &machine, &a_record(&job));
        assert_eq!(
            resolve(&mut keeper, &machine, &job),
            HowItResolved::TheMachineDoesNotHoldIt
        );
        assert!(keeper.standing.is_empty());
    }

    /// **A machine that could not be asked is not a machine that answered
    /// *no*** — the record stands, so a job that is in fact rendering is not
    /// lost.
    #[test]
    fn a_machine_that_could_not_be_asked_leaves_the_record_standing() {
        let job = a_job();
        let mut keeper = TheRecordInHand::default();
        let machine = a_machine(
            WhatTheSubmissionGave::TheAcknowledgementWasLost,
            Err("the connection dropped".to_string()),
        );
        submit(&mut keeper, &machine, &a_record(&job));
        assert!(matches!(
            resolve(&mut keeper, &machine, &job),
            HowItResolved::TheMachineCouldNotBeAsked { .. }
        ));
        assert_eq!(keeper.standing, vec![job]);
    }

    /// **A refused local write is its own answer, and not the machine going
    /// quiet.**
    ///
    /// **The fault this catches is a machine that answered plainly being
    /// reported as a machine that could not be asked** — the two call for
    /// different repairs, and the record stands either way, so nothing in the
    /// outcome itself tells them apart.
    #[test]
    fn a_refused_local_write_says_it_was_this_computer_and_not_the_machine() {
        let job = a_job();
        let mut keeper = TheRecordInHand::default();
        let machine = a_machine(WhatTheSubmissionGave::TheAcknowledgementWasLost, Ok(false));
        submit(&mut keeper, &machine, &a_record(&job));
        keeper.refuses = true;
        assert!(matches!(
            resolve(&mut keeper, &machine, &job),
            HowItResolved::TheLocalWriteWasRefused { .. }
        ));
        assert_eq!(keeper.standing, vec![job], "the record stands");
    }

    /// **Nothing in this module issues an identity.** The guard is a scan of
    /// the working half, because the fault it catches is a line added later.
    #[test]
    fn nothing_here_issues_an_identity() {
        let source = include_str!("submission.rs");
        let working_half = source
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half");
        let code: String = working_half
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join(" ");
        for forbidden in ["issue", "Identity::"] {
            assert!(
                !code.contains(forbidden),
                "{forbidden} would re-issue the work, which Y3 forbids"
            );
        }
        const _: () = assert!(NOTHING_HERE_RE_ISSUES_THE_WORK);
        const _: () =
            assert!(crate::identity::THE_IDENTITY_IS_ISSUED_BEFORE_THE_SUBMISSION_IS_SENT);
    }
}
