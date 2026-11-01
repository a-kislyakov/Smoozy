//! Permission: the refusal told apart, and elevated access — `File Browser`
//! §2, P1 to P10 with P2a and P6a, Rows 180, 181 and 342.
//!
//! **Constraint:** Row 181 — *"Where the login is not permitted to read
//! something, Smoozy shows what it can and says plainly what it could not"* and
//! *"Smoozy carries out the part it can and says plainly what it could not"* —
//! and Row 180 — *"And where the machine allows it, Smoozy can offer to try
//! again with **elevated access**, rather than only reporting the refusal."*
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Server protocol and transfers` 1.1, 1.2, 1.3, S1, S4, S5,
//! F2. `Server Setup` E2, E4, §1, §9. `Desktop app and delivery` M23, M27.
//! `State, identity and recovery` §9.2, §9.4.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. This plan defines an M1, an M3, a
//! K1, a K2 and a K3 of its own, which is every marker `Server Setup` and
//! `Rented-server runtime` are cited for below.

use smoozy_server_protocol::subsystem::SubsystemFailure;

/// **P1 and P2: which of the three Smoozy met, told apart on the subsystem
/// route by distinct protocol statuses, in one request.**
///
/// `Server protocol and transfers` 1.3: at readdir **and at stat alike** the
/// failures are told apart. The protocol's own codes are
/// `SSH_FX_NO_SUCH_FILE` and `SSH_FX_PERMISSION_DENIED` (documented).
/// **What was byte-identical was the transfer client program's rendering**,
/// which expands the path before it asks; that program is not on this route.
/// **So Smoozy knows which of the two it met, and no message anywhere may say
/// otherwise.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatSmoozyMet {
    /// **This login may not.** The elevation offer of §2.2 attaches here and
    /// nowhere else.
    Refused,
    /// **It is not there.**
    Gone,
    /// **The machine did not answer.** *"Not a statement about the folder at
    /// all: it is the connection's case, `Servers`' condition covers it, and
    /// the listing goes stale under §1.5 rather than the folder being reported
    /// broken."*
    TheMachineDidNotAnswer,
    /// The server answered something else. **Nothing is guessed from it** and
    /// it is neither of the two the promise separates; the caller says what
    /// happened and stops there.
    SomethingElse(SubsystemFailure),
}

impl WhatSmoozyMet {
    /// Read off the protocol's own status, and derived nowhere else.
    pub fn from(failure: &SubsystemFailure) -> WhatSmoozyMet {
        if failure.is_the_login_not_being_allowed() {
            WhatSmoozyMet::Refused
        } else if failure.is_the_path_not_being_there() {
            WhatSmoozyMet::Gone
        } else if matches!(failure, SubsystemFailure::TheChannelIsGone) {
            WhatSmoozyMet::TheMachineDidNotAnswer
        } else {
            WhatSmoozyMet::SomethingElse(failure.clone())
        }
    }

    /// **P2: the elevation offer of §2.2 is attached to the refusal case
    /// alone.**
    pub fn the_elevation_question_arises(&self) -> bool {
        matches!(self, WhatSmoozyMet::Refused)
    }
}

/// **P2a: what can be told before a folder is opened, and it was less than the
/// design drew** (Row 342).
///
/// - **The promise's own stated purpose is kept whole and for free.** *"It is
///   not silently absent"* — **a folder is listed because its parent's reading
///   names it, and being unable to read *into* a folder is a different
///   permission from seeing its name in the parent.** So the folder always
///   stays in the list.
/// - **What cannot be done is the marking.** L1 yields four fields in one
///   batched reading and states that no reading here ever issues a stat per
///   entry; X1 repeats the prohibition. **The per-entry probe that would answer
///   this in advance is the one thing both forbid, for a measured reason** — 52
///   round trips against 5010.
/// - **And the probe would not answer it even if it were free.** The question is
///   *can **this login** read into this folder*, which mode bits do not answer:
///   supplementary groups, access-control lists and the login's own privilege
///   all bear on it. **A mark composed from what the machine says about the
///   entry would be a prediction, and a wrong prediction here stops a user
///   opening a folder they could have opened.**
/// - **So the refusal is met on the click and told in as many words, which is
///   P2**, and *before it is opened* is not kept.
pub const NOTHING_IS_MARKED_IN_ADVANCE_AND_EVERY_FOLDER_STAYS_IN_THE_LIST: bool = true;

/// One directory reading, as the walk asks for it. The walk is the only caller.
///
/// **It is asynchronous because the reading is the machine's**, and its real
/// end — [`crate::wiring::TheReadingChannel`] — is the file-transfer
/// subsystem's own directory reading, the same operation L1's listing takes and
/// on the same channel value.
pub trait CanListAComponent {
    /// **On the subsystem route.** `Ok(())` where the component listed.
    fn list(
        &mut self,
        component_path: &str,
    ) -> impl std::future::Future<Output = Result<(), SubsystemFailure>> + Send;
}

/// **P3: the walk says *which component* is refused, and that is its whole
/// purpose.**
///
/// What one request does **not** give is which component of the path the
/// refusal arises at, and *this login may not read `/jobs`* is a sentence
/// Smoozy can only say by walking. So: **where the browser lands on a path
/// directly** — *show outputs* opening at a job's folder, `Rendering`'s
/// point-at-the-file entry, a path remembered from an earlier sitting — the
/// stat answer names refusal or absence, and **where it names refusal the
/// browser opens each component in turn, from the deepest ancestor it can
/// list.**
///
/// **The walk costs one directory reading per component and reports underway
/// like any reading.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalkOutcome {
    /// **The component that answered *permission denied*, named as the
    /// component it is** — *this login may not read `/jobs`* is the accurate
    /// sentence even when the user asked about `/jobs/render-3/frame.exr`. The
    /// elevation offer attaches here.
    RefusedAt(String),
    /// **A component absent from its parent's listing is *gone*, named at the
    /// component.**
    GoneAt(String),
    /// **P4: where the walk cannot name the component, the message says which
    /// of the two it met and stops there.**
    ///
    /// A refusal met at the very top, or readings that disagree between two
    /// steps of one walk. **The residue is ignorance of *where*, never of
    /// *which*** — the stat answer carries the distinct status whatever the
    /// walk does afterwards. **A message naming the wrong one of the two is
    /// worse than a message naming neither**, and *says plainly what it could
    /// not* is a promise about honesty, not about omniscience.
    CouldNotSayWhere(WhatSmoozyMet),
}

impl WalkOutcome {
    /// **The elevation offer attaches to a refusal, wherever the walk landed.**
    pub fn the_elevation_question_arises(&self) -> bool {
        match self {
            WalkOutcome::RefusedAt(_) => true,
            WalkOutcome::GoneAt(_) => false,
            WalkOutcome::CouldNotSayWhere(met) => met.the_elevation_question_arises(),
        }
    }
}

/// **P3's walk**, over the components of an absolute path.
///
/// The stat answer that started it is `first_answer` — the one request P1 says
/// carries the distinct status. **Where it names absence there is nothing to
/// walk**: the answer is already which of the two it was.
///
/// **The lister is a type parameter rather than `dyn`**, because a reading
/// taken on a machine is asynchronous and an asynchronous trait method has no
/// object-safe form.
pub async fn walk<L: CanListAComponent>(
    path: &str,
    first_answer: &WhatSmoozyMet,
    lister: &mut L,
) -> WalkOutcome {
    match first_answer {
        WhatSmoozyMet::Refused => {}
        other => return WalkOutcome::CouldNotSayWhere(other.clone()),
    }

    let mut walked = String::new();
    for component in path.split('/').filter(|part| !part.is_empty()) {
        walked.push('/');
        walked.push_str(component);
        match lister.list(&walked).await {
            Ok(()) => continue,
            Err(failure) => {
                return match WhatSmoozyMet::from(&failure) {
                    WhatSmoozyMet::Refused => WalkOutcome::RefusedAt(walked),
                    WhatSmoozyMet::Gone => WalkOutcome::GoneAt(walked),
                    // **A step that answers neither of the two. The residue
                    // keeps the answer the walk was started with, and never
                    // this step's.**
                    //
                    // The walk starts only where the first answer was
                    // *refused*, and that is what makes P4's promise true —
                    // *"the residue is ignorance of **where**, never of
                    // **which**"*. Carrying this step's answer instead would
                    // leave the residue naming neither of the two, so
                    // `the_elevation_question_arises` would read false and
                    // Row 180's offer to try again with elevated access would
                    // disappear on a machine where it would have worked. **The
                    // fall-through at the end of this loop already answers the
                    // first answer**, and this is the same rule at the same
                    // walk's other exit.
                    _ => WalkOutcome::CouldNotSayWhere(first_answer.clone()),
                };
            }
        }
    }

    // Every component listed, and yet the path was refused: the two readings
    // disagree, which is P4's second case in its own words.
    WalkOutcome::CouldNotSayWhere(WhatSmoozyMet::Refused)
}

/// The machine kind, as `Rented-server runtime` K1's table gives it.
///
/// **This document installs nothing**, so there is no kind on record to read:
/// `Server Setup` §1 takes the kind at a placement and *"re-taken at every
/// placement and never carried forward (K2)"* — the clause that quotation cites
/// is `Rented-server runtime` K2 — and **a machine the user is browsing may be
/// one no package was ever placed on**. P5 therefore takes the reading **at the
/// refusal**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineKind {
    /// A container, under a non-root login.
    AContainerUnderANonRootLogin,
    /// A systemd machine.
    ASystemdMachine,
    /// **The third row of `Rented-server runtime` K1's table, and
    /// `Rented-server runtime` K3's refusal.**
    TheTableDoesNotPlaceIt,
}

/// `Server Setup` M3's non-interactive probe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheProbe {
    /// Passwordless elevation.
    Succeeded,
    Failed,
}

/// **P5's two readings, taken at the refusal, beside each other, and never
/// before one.**
pub trait ReadsWhatElevationWouldDo {
    fn the_machine_kind(&mut self) -> MachineKind;
    fn the_non_interactive_probe(&mut self) -> TheProbe;
}

/// What the user is offered at a refusal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Offer {
    /// **The retry runs elevated with nothing asked.**
    OfferedAndNothingIsAsked,
    /// **Accepting starts `Server Setup` §9's flow** — the password asked at
    /// that moment, the validate-first separation (E4), the stdin route, the
    /// three lifetimes settled once in `Preparation/Specs/Whole app.md`.
    OfferedAndAPasswordIsAsked,
    /// **No offer, and the refusal is reported as it stands.**
    NoOffer(WhyThereIsNoOffer),
}

/// Why there is no offer. **A refusal shown with nothing beside it is a real
/// state** — one the design had no drawing for until 2026-08-24, when it took
/// the condition and gained the state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyThereIsNoOffer {
    /// **Fact 27, corrected 2026-08-22:** a container's non-root login has an
    /// elevation route that **demands a password Smoozy does not hold** —
    /// `sudo -n` demands a password, `/root/onstart.sh` and `/etc/rc.local` are
    /// both `Permission denied`, and switching to root fails authentication.
    /// The register's own follow-on sentence is why the outcome is still *no
    /// offer*: *"a password Smoozy does not have is as good as no route"*, and
    /// `Server Setup` §9's flow has no password to put down it.
    AContainersLoginDemandsAPasswordSmoozyDoesNotHold,
    /// **`Server Setup` M1:** *"A machine the table does not place is a machine
    /// no package is placed on (K3) and no setup runs on"* — the clause that
    /// quotation cites is `Rented-server runtime` K3 — so there is no flow to
    /// start and nothing is established about what elevation would do
    /// there. **Decided by the kind alone and does not wait on the probe.**
    /// **Browsing such a machine is unaffected**: it lists, it uploads, it
    /// holds files, and only the retry is absent.
    TheKindTableDoesNotPlaceTheMachine,
    /// **A login that is already root gets no offer either**: there is nothing
    /// to elevate to, and on the box where root was real root a permission
    /// refusal could not arise at all.
    TheLoginIsAlreadyRoot,
    /// **P6a.** The act is a transfer, and no transfer is retried elevated.
    TheActIsATransfer,
}

/// **P5's four outcomes, composed per machine from the machine kind and the
/// non-interactive probe.**
///
/// **P7: the offer is per act, at the moment of the refusal, never before** —
/// the spec's own rule, kept by `Server Setup` E2 for the heavy weight and kept
/// here for browsing. **Nothing is probed or elevated for a machine the user has
/// not met a refusal on**, which is why this takes the readings rather than a
/// remembered value.
pub fn the_offer(
    act: ElevatableAct,
    login_is_already_root: bool,
    readings: &mut dyn ReadsWhatElevationWouldDo,
) -> Offer {
    // P6a first: an upload has no elevated route at all, so nothing is probed
    // for one and no reading is taken.
    if act == ElevatableAct::ATransfer {
        return Offer::NoOffer(WhyThereIsNoOffer::TheActIsATransfer);
    }
    if login_is_already_root {
        return Offer::NoOffer(WhyThereIsNoOffer::TheLoginIsAlreadyRoot);
    }

    // The kind alone settles the fourth outcome and does not wait on the probe.
    let kind = readings.the_machine_kind();
    if kind == MachineKind::TheTableDoesNotPlaceIt {
        return Offer::NoOffer(WhyThereIsNoOffer::TheKindTableDoesNotPlaceTheMachine);
    }

    match (readings.the_non_interactive_probe(), kind) {
        (TheProbe::Succeeded, _) => Offer::OfferedAndNothingIsAsked,
        (TheProbe::Failed, MachineKind::AContainerUnderANonRootLogin) => {
            Offer::NoOffer(WhyThereIsNoOffer::AContainersLoginDemandsAPasswordSmoozyDoesNotHold)
        }
        (TheProbe::Failed, MachineKind::ASystemdMachine) => Offer::OfferedAndAPasswordIsAsked,
        (TheProbe::Failed, MachineKind::TheTableDoesNotPlaceIt) => {
            // Unreachable by the branch above, and written out rather than
            // folded into a catch-all so a fifth kind fails here.
            Offer::NoOffer(WhyThereIsNoOffer::TheKindTableDoesNotPlaceTheMachine)
        }
    }
}

/// **P6: the closed set of acts elevation covers, and a transfer is not among
/// them.**
///
/// *"The elevated act runs on the command channel, because no *configured*
/// subsystem carries elevation."* The elevating command wraps the machine's own
/// programs; the configured subsystem channel authenticates as the login and no
/// elevation reaches inside it.
///
/// **An elevated listing is a command-channel reading of the same four fields
/// in the epoch form**, and **an elevated file operation is the machine's own
/// remove, move, copy or make-directory under the elevating command.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElevatableAct {
    AListing,
    Remove,
    Move,
    Copy,
    MakeDirectory,
    /// **P6a: not elevatable, and it is on this list so the negative is a value
    /// rather than an omission.**
    ATransfer,
}

impl ElevatableAct {
    /// The five P6 names, in P6's own order.
    pub const THE_ELEVATED_ACTS: [ElevatableAct; 5] = [
        ElevatableAct::AListing,
        ElevatableAct::Remove,
        ElevatableAct::Move,
        ElevatableAct::Copy,
        ElevatableAct::MakeDirectory,
    ];
}

/// **P6a: there is no elevated upload, and the reason is stated so nobody
/// reaches for one.**
///
/// The elevated acts P6 enumerates are a listing and the machine's own remove,
/// move, copy or make-directory, and **no transfer is among them**. An upload's
/// bytes move on the configured subsystem channel, which authenticates as the
/// login and which no elevation reaches inside (P6); and the staging they move
/// into sits **beside the destination — inside the folder the user is
/// browsing** (`Server protocol and transfers` S1), so a folder the login
/// cannot write to refuses the staging as well as the publication.
///
/// **Composing one out of what exists is not available either**: staging
/// somewhere writable and publishing by the elevated move would need a staging
/// path this document chose, and `Server protocol and transfers` S4 and S5 put
/// that path inside Smoozy while S1 places it — a contract taken entire and not
/// re-decided here.
///
/// **So a write refused for permission on an upload is reported as it stands**,
/// which is P4's honesty and §2.3's partial report, and that is the first of
/// Row 181's two responses kept whole. **§13 item 16 carries what the promise
/// costs, and it is permanently unresolved**: item 9's wrapped elevated
/// file-transfer server wants a rental, and the Creator ruled on 2026-08-31
/// that there will be no further rental to answer a question — so **Row 181's
/// second response stays unkept for an upload for the life of this document**,
/// and no space is left anywhere for an answer that is coming.
pub const THERE_IS_NO_ELEVATED_UPLOAD: bool = true;

/// **P6's one limit, stated here rather than left to be met, because L2's means
/// are unavailable on the command channel.**
///
/// L2 keeps every listing off rendered lines by taking the protocol's own
/// fields; **the command channel has no protocol fields, only text**, and any
/// explicit output format separates one entry from the next by some record
/// separator of its own. **An elevated listing whose separator is a byte a
/// filename may hold cannot separate an entry that holds it** — and Fact 29's
/// seventeen include a name with a newline, proved to reach a machine byte for
/// byte, so the case is not contrived.
///
/// **The hazard is a property of a choice of separator rather than of text
/// output as such.** POSIX.1-2024's own definition of a filename: *"A sequence
/// of bytes consisting of 1 to {NAME_MAX} bytes used to name a file. The bytes
/// composing the name shall not contain the <NUL> or <slash> characters."* And
/// the channel carries that byte — RFC 4251 §5: *"Strings are allowed to
/// contain arbitrary binary data, including null characters and 8-bit
/// characters."*
///
/// **What that does not do is remove the caution**: whether an **arbitrary
/// rental** offers a program with such a format is §13 item 9's own
/// unestablished subject, and **that item is permanently unresolved**. **No
/// separator and no program is named here** — naming one would invent a fact.
///
/// **So an elevated listing whose records do not all carry exactly the fields
/// the format asked for is reported as unavailable against that connection,
/// with the reason said, and is never shown as a listing.** That direction is
/// `Server protocol and transfers` 1.2's own — *"A capability that is not
/// announced is never attempted and never guessed at"* — and X2's rule that a
/// count wrong in the quiet direction is worse than no count. **Nothing in the
/// text says which entry was split, so the answer is not narrowly wrong: the
/// whole reading's boundaries are untrustworthy.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnElevatedListing {
    /// Every record carried exactly the fields the format asked for.
    Accepted(Vec<Vec<Vec<u8>>>),
    /// **Reported as unavailable against that connection, with the reason
    /// said.** The folder keeps whatever the ordinary reading gave it, and the
    /// user is told what could not be read.
    UnavailableAgainstThisConnection(WhyTheElevatedListingIsUnavailable),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTheElevatedListingIsUnavailable {
    /// **The whole reading's boundaries are untrustworthy** — at least one
    /// record did not carry exactly the fields the format asked for.
    TheRecordsDoNotAllCarryTheFieldsTheFormatAskedFor,
}

/// **P6's rule, applied to a whole reading rather than to a record.**
///
/// One record short or long condemns the reading, because nothing in the text
/// says which entry was split.
pub fn accept_an_elevated_listing(
    records: Vec<Vec<Vec<u8>>>,
    fields_the_format_asked_for: usize,
) -> AnElevatedListing {
    if records
        .iter()
        .all(|record| record.len() == fields_the_format_asked_for)
    {
        AnElevatedListing::Accepted(records)
    } else {
        AnElevatedListing::UnavailableAgainstThisConnection(
            WhyTheElevatedListingIsUnavailable::TheRecordsDoNotAllCarryTheFieldsTheFormatAskedFor,
        )
    }
}

/// **P6's carriage rule, stated so a builder does not put a path in a command
/// string.**
///
/// *"Before hand-over, an elevated reading or file operation carries its target
/// path under `Server protocol and transfers` 1.1's admissibility rule — as the
/// target of a planted link, or as a Smoozy-synthesised token — and never in
/// the command string; the elevating command stays constant, and no mechanism
/// changes with the phase."*
pub const A_TARGET_PATH_NEVER_ENTERS_AN_ELEVATED_COMMAND_STRING: bool = true;

/// **P8: what an elevated act shows is what elevation could see, and it is not
/// remembered as the machine's ordinary truth.**
///
/// The cache entry a successful elevated listing produces carries the fact that
/// it was read elevated — [`crate::listing::CompleteListing::read_elevated`] —
/// **so a later unelevated refresh going back to *refused* is a change of
/// login's view rather than a change of the machine, and the two are never
/// presented as disagreeing readings of one thing.**
pub fn is_a_change_of_the_machine(previous_was_elevated: bool, now_refused: bool) -> bool {
    !(previous_was_elevated && now_refused)
}

/// **P9 and P10: the partial action.**
///
/// *"Every multi-file action here is declared partial across its units, and the
/// declaration table is §11"* — per `State, identity and recovery` §9.2,
/// **declared per action, never judged at run time**. A delete or an upload of
/// many files refused part-way reports which units happened, which failed, and
/// which were not attempted naming the unit that stopped them, from the §9.4
/// outcome record whose enumeration was durable before the first unit ran.
///
/// ***Carries out the part it can* means the failure of one unit abandons
/// nothing else**, except where §9.3 orders a pair. **The report's shape is
/// §9.4's** — [`smoozy_state::outcomes::Recorded`] — **its presentation is
/// `Desktop app and delivery`'s (M23–M27), and its wording is Design's.**
pub const THE_FAILURE_OF_ONE_UNIT_ABANDONS_NOTHING_ELSE_EXCEPT_AN_ORDERED_PAIR: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_server_protocol::subsystem::StatusCode;

    fn refused() -> SubsystemFailure {
        SubsystemFailure::TheServerRefused(StatusCode::PermissionDenied)
    }

    fn absent() -> SubsystemFailure {
        SubsystemFailure::TheServerRefused(StatusCode::NoSuchFile)
    }

    struct Lists {
        refuses_at: Option<&'static str>,
        absent_at: Option<&'static str>,
    }

    impl CanListAComponent for Lists {
        async fn list(&mut self, component_path: &str) -> Result<(), SubsystemFailure> {
            if self.refuses_at == Some(component_path) {
                return Err(refused());
            }
            if self.absent_at == Some(component_path) {
                return Err(absent());
            }
            Ok(())
        }
    }

    struct Says(MachineKind, TheProbe);

    impl ReadsWhatElevationWouldDo for Says {
        fn the_machine_kind(&mut self) -> MachineKind {
            self.0
        }
        fn the_non_interactive_probe(&mut self) -> TheProbe {
            self.1
        }
    }

    /// Counts the readings, so P7's *never before a refusal* and P5's *the kind
    /// alone settles the fourth outcome* are observable rather than asserted.
    struct Counts {
        kind: MachineKind,
        probe: TheProbe,
        kinds_read: usize,
        probes_read: usize,
    }

    impl ReadsWhatElevationWouldDo for Counts {
        fn the_machine_kind(&mut self) -> MachineKind {
            self.kinds_read += 1;
            self.kind
        }
        fn the_non_interactive_probe(&mut self) -> TheProbe {
            self.probes_read += 1;
            self.probe
        }
    }

    #[test]
    fn refusal_and_absence_are_two_answers_and_neither_is_the_other() {
        assert_eq!(WhatSmoozyMet::from(&refused()), WhatSmoozyMet::Refused);
        assert_eq!(WhatSmoozyMet::from(&absent()), WhatSmoozyMet::Gone);
        assert_eq!(
            WhatSmoozyMet::from(&SubsystemFailure::TheChannelIsGone),
            WhatSmoozyMet::TheMachineDidNotAnswer
        );
        assert!(WhatSmoozyMet::Refused.the_elevation_question_arises());
        assert!(!WhatSmoozyMet::Gone.the_elevation_question_arises());
        assert!(!WhatSmoozyMet::TheMachineDidNotAnswer.the_elevation_question_arises());
    }

    #[tokio::test]
    async fn the_walk_names_the_component_that_refused_and_not_the_path_that_was_asked_about() {
        let mut lister = Lists {
            refuses_at: Some("/jobs"),
            absent_at: None,
        };
        assert_eq!(
            walk(
                "/jobs/render-3/frame.exr",
                &WhatSmoozyMet::Refused,
                &mut lister
            )
            .await,
            WalkOutcome::RefusedAt("/jobs".to_string())
        );
    }

    #[tokio::test]
    async fn a_component_absent_from_its_parent_is_gone_named_at_the_component() {
        let mut lister = Lists {
            refuses_at: None,
            absent_at: Some("/jobs/render-3"),
        };
        assert_eq!(
            walk(
                "/jobs/render-3/frame.exr",
                &WhatSmoozyMet::Refused,
                &mut lister
            )
            .await,
            WalkOutcome::GoneAt("/jobs/render-3".to_string())
        );
    }

    #[tokio::test]
    async fn a_walk_that_cannot_name_the_component_still_names_which_of_the_two_it_met() {
        // Every component listed, and the path was refused: two readings that
        // disagree, which is P4's second case.
        let mut lister = Lists {
            refuses_at: None,
            absent_at: None,
        };
        let outcome = walk("/jobs/render-3", &WhatSmoozyMet::Refused, &mut lister).await;
        assert_eq!(
            outcome,
            WalkOutcome::CouldNotSayWhere(WhatSmoozyMet::Refused)
        );
        // And the residue never names the wrong one of the two.
        assert!(outcome.the_elevation_question_arises());

        let mut lister = Lists {
            refuses_at: None,
            absent_at: None,
        };
        assert_eq!(
            walk("/jobs", &WhatSmoozyMet::Gone, &mut lister).await,
            WalkOutcome::CouldNotSayWhere(WhatSmoozyMet::Gone)
        );
    }

    /// **P4: the residue is ignorance of *where*, never of *which*.**
    ///
    /// A middle step of the walk that answers neither of the two must not throw
    /// away the answer the walk was started with. Where it did, the residue
    /// named neither, `the_elevation_question_arises` read false, and Row 180's
    /// offer to try again with elevated access disappeared on a machine where
    /// it would have worked.
    #[tokio::test]
    async fn a_middle_step_answering_neither_of_the_two_keeps_the_answer_that_started_the_walk() {
        struct Disagrees;
        impl CanListAComponent for Disagrees {
            async fn list(&mut self, component_path: &str) -> Result<(), SubsystemFailure> {
                if component_path == "/jobs/render-3" {
                    // Neither a refusal nor an absence.
                    return Err(SubsystemFailure::TheChannelIsGone);
                }
                Ok(())
            }
        }
        let outcome = walk(
            "/jobs/render-3/frame.exr",
            &WhatSmoozyMet::Refused,
            &mut Disagrees,
        )
        .await;
        assert_eq!(
            outcome,
            WalkOutcome::CouldNotSayWhere(WhatSmoozyMet::Refused),
            "the residue names the refusal the stat answer carried"
        );
        assert!(
            outcome.the_elevation_question_arises(),
            "and the elevation offer still arises"
        );
    }

    #[tokio::test]
    async fn an_absence_is_never_walked_because_the_one_request_already_answered_it() {
        struct NeverAsked;
        impl CanListAComponent for NeverAsked {
            async fn list(&mut self, _component_path: &str) -> Result<(), SubsystemFailure> {
                panic!("the walk ran on an answer that was not a refusal");
            }
        }
        assert_eq!(
            walk("/jobs/a", &WhatSmoozyMet::Gone, &mut NeverAsked).await,
            WalkOutcome::CouldNotSayWhere(WhatSmoozyMet::Gone)
        );
    }

    #[test]
    fn the_four_outcomes_of_p5_are_what_p5_says_they_are() {
        // Row 1: the probe succeeds.
        assert_eq!(
            the_offer(
                ElevatableAct::AListing,
                false,
                &mut Says(MachineKind::ASystemdMachine, TheProbe::Succeeded)
            ),
            Offer::OfferedAndNothingIsAsked
        );
        // Row 2: a container under a non-root login.
        assert_eq!(
            the_offer(
                ElevatableAct::Remove,
                false,
                &mut Says(MachineKind::AContainerUnderANonRootLogin, TheProbe::Failed)
            ),
            Offer::NoOffer(WhyThereIsNoOffer::AContainersLoginDemandsAPasswordSmoozyDoesNotHold)
        );
        // Row 3: a systemd machine whose probe failed.
        assert_eq!(
            the_offer(
                ElevatableAct::Move,
                false,
                &mut Says(MachineKind::ASystemdMachine, TheProbe::Failed)
            ),
            Offer::OfferedAndAPasswordIsAsked
        );
        // Row 4: the table does not place the machine.
        assert_eq!(
            the_offer(
                ElevatableAct::MakeDirectory,
                false,
                &mut Says(MachineKind::TheTableDoesNotPlaceIt, TheProbe::Succeeded)
            ),
            Offer::NoOffer(WhyThereIsNoOffer::TheKindTableDoesNotPlaceTheMachine)
        );
    }

    #[test]
    fn the_unplaceable_kind_decides_without_waiting_on_the_probe() {
        let mut readings = Counts {
            kind: MachineKind::TheTableDoesNotPlaceIt,
            probe: TheProbe::Succeeded,
            kinds_read: 0,
            probes_read: 0,
        };
        let offer = the_offer(ElevatableAct::AListing, false, &mut readings);
        assert_eq!(
            offer,
            Offer::NoOffer(WhyThereIsNoOffer::TheKindTableDoesNotPlaceTheMachine)
        );
        assert_eq!(readings.kinds_read, 1);
        assert_eq!(
            readings.probes_read, 0,
            "the probe was run for a kind that decides alone"
        );
    }

    #[test]
    fn a_root_login_gets_no_offer_and_nothing_is_probed_for_it() {
        let mut readings = Counts {
            kind: MachineKind::ASystemdMachine,
            probe: TheProbe::Succeeded,
            kinds_read: 0,
            probes_read: 0,
        };
        assert_eq!(
            the_offer(ElevatableAct::AListing, true, &mut readings),
            Offer::NoOffer(WhyThereIsNoOffer::TheLoginIsAlreadyRoot)
        );
        assert_eq!(readings.kinds_read, 0);
        assert_eq!(readings.probes_read, 0);
    }

    #[test]
    fn an_upload_is_never_offered_an_elevated_retry_and_nothing_is_probed_for_one() {
        // P6a. The negative is a value, not an omission.
        let mut readings = Counts {
            kind: MachineKind::ASystemdMachine,
            probe: TheProbe::Succeeded,
            kinds_read: 0,
            probes_read: 0,
        };
        assert_eq!(
            the_offer(ElevatableAct::ATransfer, false, &mut readings),
            Offer::NoOffer(WhyThereIsNoOffer::TheActIsATransfer)
        );
        assert_eq!(readings.kinds_read, 0);
        assert_eq!(readings.probes_read, 0);
        const { assert!(THERE_IS_NO_ELEVATED_UPLOAD) };
        assert!(!ElevatableAct::THE_ELEVATED_ACTS.contains(&ElevatableAct::ATransfer));
        assert_eq!(ElevatableAct::THE_ELEVATED_ACTS.len(), 5);
    }

    #[test]
    fn an_elevated_listing_with_one_bad_record_condemns_the_whole_reading() {
        let good = vec![
            vec![
                b"a.txt".to_vec(),
                b"12".to_vec(),
                b"100".to_vec(),
                b"f".to_vec(),
            ],
            vec![
                b"b.txt".to_vec(),
                b"13".to_vec(),
                b"101".to_vec(),
                b"f".to_vec(),
            ],
        ];
        assert!(matches!(
            accept_an_elevated_listing(good.clone(), 4),
            AnElevatedListing::Accepted(_)
        ));

        // A name holding the separator splits one record into two, and nothing
        // in the text says which entry it was.
        let mut split = good;
        split.push(vec![b"with a newline".to_vec(), b"14".to_vec()]);
        assert_eq!(
            accept_an_elevated_listing(split, 4),
            AnElevatedListing::UnavailableAgainstThisConnection(
                WhyTheElevatedListingIsUnavailable::TheRecordsDoNotAllCarryTheFieldsTheFormatAskedFor
            )
        );
    }

    #[test]
    fn an_elevated_reading_going_back_to_refused_is_not_a_change_of_the_machine() {
        assert!(!is_a_change_of_the_machine(true, true));
        assert!(is_a_change_of_the_machine(false, true));
        assert!(is_a_change_of_the_machine(true, false));
    }
}
