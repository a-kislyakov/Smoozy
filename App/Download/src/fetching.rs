//! The fetch pipeline — `Download` §5, P1 to P11, Rows 205, 208, 212 and 417.
//!
//! **Constraint:** Row 205 — the structure and the names a download lands
//! with, and Row 212 — *"the reckoning says by what"*, kept at the only moment
//! it can be kept.
//!
//! **What is here is every judgement the pipeline makes and none of its
//! acts.** The transfer itself is `Server protocol and transfers` §7's,
//! unmodified, and this crate neither opens a channel nor moves a byte
//! ([`crate::arriving_later`]).
//!
//! # The adapter's two judgements are consumed, never taken a second time
//!
//! `Desktop app and delivery` §7 draws them: **can this name be written at
//! all**, before the transfer is arranged; **is this name already taken**, at
//! publication, as an exclusive create of the final name. The adapter hands on
//! an outcome and never a prompt, a rename or a decision.
//!
//! **That document's crate cannot be reached from here and the reason is
//! structural**: it depends on the feature modules and supplies their
//! adapters, so a feature module cannot depend on it back.
//! `Dependency Upload & Remap` F13 met the same wall and its answer is taken
//! here — **the
//! outcome's shape is written out once, in this file, as a value the adapter
//! fills**, and **nothing in this crate classifies a system's own error, byte
//! or name into it**. [`TheNameCheckSaid`] is that value and
//! [`TheReceivingSystemsRules`] is the one route to the adapter's judgement.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Desktop app and delivery` C3, C4, C5, C6, C10,
//! C11, C12, C13, C14, C15, C18, C19, C20, C22. `Server protocol and
//! transfers` S1, S4, S9, S15, S18, S21, S25, S26, S27. `State, identity and
//! recovery` §9.5. `File Browser` O7, X5, X6.
//!
//! **A marker this plan also defines is deliberately not listed above,
//! whichever document is meant, and is written out with its document at every
//! site instead** — the header list cannot tell the two apart, and a bare one
//! would resolve to a real and wrong clause. `Job files and outputs` N3 and
//! `File Browser` N2 are the two that arise here.

use smoozy_server_protocol::transfers::publication::Intent;
use smoozy_state::identities::{Identity, IdentityKind};

use crate::classes::WhatArrangedIt;

/// **`Desktop app and delivery` C4's four name-check outcomes, closed set, as
/// the adapter hands them on.**
///
/// **Nothing in this crate produces one from a byte, a name or a system
/// error.** It is the adapter's own value, carried across the wall the module
/// header describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheNameCheckSaid {
    /// The name may be written as given.
    TheNameMayBeWrittenAsGiven,
    /// The receiving system will not represent it at all.
    NotRepresentable,
    /// **In this system's own unit** — `Desktop app and delivery` C5, and the
    /// unit is the adapter's rather than a count of anything chosen here.
    TooLongInThisSystemsOwnUnit,
    /// The system would take the name and store a different one.
    SilentlyTransformed,
}

impl TheNameCheckSaid {
    /// **P4: an outcome of *not representable*, *too long in this system's own
    /// unit* or *silently transformed* is not acceptable as given**, and the
    /// file is handled under [`WhichRefusalItWas`].
    pub fn is_acceptable_as_given(self) -> bool {
        matches!(self, TheNameCheckSaid::TheNameMayBeWrittenAsGiven)
    }

    /// The refusal this outcome is, where it is one.
    pub fn as_a_refusal(self) -> Option<WhichRefusalItWas> {
        match self {
            TheNameCheckSaid::TheNameMayBeWrittenAsGiven => None,
            TheNameCheckSaid::NotRepresentable => Some(WhichRefusalItWas::NotRepresentable),
            TheNameCheckSaid::TooLongInThisSystemsOwnUnit => {
                Some(WhichRefusalItWas::TooLongInThisSystemsOwnUnit)
            }
            TheNameCheckSaid::SilentlyTransformed => Some(WhichRefusalItWas::SilentlyTransformed),
        }
    }
}

/// **P8: which of the three refusals it was, said in the request or
/// arrangement outcome.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichRefusalItWas {
    NotRepresentable,
    TooLongInThisSystemsOwnUnit,
    SilentlyTransformed,
}

/// **The one route to the adapter's own judgement and its own substitution.**
///
/// **No implementation of this trait exists outside this crate's own tests**
/// ([`crate::arriving_later`]): the real end is the platform file adapter's,
/// and this is the seam.
pub trait TheReceivingSystemsRules {
    /// **`Desktop app and delivery` C3, per component, in the receiving
    /// system's own unit, against the receiving volume.**
    fn check(&self, component: &[u8]) -> TheNameCheckSaid;

    /// **What the adapter puts in place of what cannot enter a name on this
    /// system**, used only where P10 composes a name of Smoozy's own.
    ///
    /// **It is never applied to a render output's name** — P8 is why, and a
    /// test holds it.
    fn substitute(&self, component: &[u8]) -> Vec<u8>;

    /// **`Desktop app and delivery` C5's unit**: how many of the receiving
    /// system's own units this component costs. It is the adapter's count and
    /// not a byte count chosen here.
    fn length_in_this_systems_own_unit(&self, component: &[u8]) -> usize;

    /// The longest component this system takes, in that same unit.
    fn the_longest_component(&self) -> usize;
}

/// **P4: every landing path is judged by the adapter's name check before the
/// transfer is arranged.**
///
/// The check runs **at enumeration for a manual request** and **at the moment
/// a file first becomes fetchable for an arrangement**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhenTheNameCheckRuns {
    AtEnumeration,
    AtTheMomentAFileFirstBecomesFetchable,
}

impl WhenTheNameCheckRuns {
    pub fn for_(arranged_by: WhatArrangedIt) -> WhenTheNameCheckRuns {
        match arranged_by {
            WhatArrangedIt::AManualRequest => WhenTheNameCheckRuns::AtEnumeration,
            WhatArrangedIt::AnAutoDownloadArrangement => {
                WhenTheNameCheckRuns::AtTheMomentAFileFirstBecomesFetchable
            }
        }
    }
}

/// **P4, as one judgement over a whole landing path.**
///
/// The check is per component, so the first component the adapter refuses is
/// the refusal — **and `Desktop app and delivery` C6's rule that the adapter's
/// writes descend rather than composing one long absolute path covers the
/// deep-tree case, so depth is not a refusal.**
pub fn judge_the_landing_path<R: TheReceivingSystemsRules + ?Sized>(
    rules: &R,
    components: &[&[u8]],
) -> Result<(), (usize, WhichRefusalItWas)> {
    for (at, component) in components.iter().enumerate() {
        if let Some(refusal) = rules.check(component).as_a_refusal() {
            return Err((at, refusal));
        }
    }
    Ok(())
}

/// **P8: a render output's name is never touched, so a name the receiving
/// system will not take as given is refused per file, not repaired.**
///
/// Substituting an artist's output name would be the silent transformation
/// Row 303 forbids arriving by Smoozy's own hand. The file is marked **cannot
/// land**, with which of the three refusals it was; **the rest of the folder
/// lands** — the partition unit is the file — and **the file stays on the
/// server.**
pub const A_RENDER_OUTPUTS_NAME_IS_NEVER_TOUCHED: bool = true;

/// **P9: a *cannot land* file will never arrive, and the bookkeeping says so
/// rather than waiting forever.**
///
/// The unit is terminal, it leaves the whole and is reported separately — the
/// interaction with Row 215 stated in one sentence: **an arrangement never
/// waits on a file the name check has already refused.**
pub const AN_ARRANGEMENT_NEVER_WAITS_ON_A_FILE_THE_NAME_CHECK_HAS_REFUSED: bool = true;

/// **P5: every publication of this document is create-intended.**
///
/// The taken question is the exclusive create of the final name
/// (`Desktop app and delivery` C10), and **this document never answers
/// *replace* on its own.** The reservation the create makes is the entry
/// publication renames over — and because it stands there, the publishing
/// rename **always replaces and never lands on an absent name, in every case**
/// (`Desktop app and delivery` C22); what create-intended governs is what the
/// exclusive create answers, not which operation runs underneath.
///
/// **The one deliberate replacement is a manual request's user-answered
/// Replace**, which publishes with the replacing rename and takes no
/// reservation (`Desktop app and delivery` C14) — *Row 303 forbids silence,
/// not replacement.*
pub fn the_publication_intent(the_user_answered_replace_for_this_file: bool) -> Intent {
    if the_user_answered_replace_for_this_file {
        Intent::ReplaceIntended
    } else {
        Intent::CreateIntended
    }
}

/// **P7: auto-download asks no question at any of this.**
///
/// The already-exists question is the manual half's; **the arrangement's
/// answer to a taken name is always the truth-plus-carry-on**, because it runs
/// with nobody watching and quietly overwriting is the one outcome the spec
/// names as destroying work unseen.
pub fn an_arrangement_may_answer_replace() -> bool {
    false
}

/// **What the exclusive create found the final name taken by**, as the record
/// and the adapter together say — which is what P6 classifies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheTakenNameIs {
    /// Nothing the record names.
    NothingTheRecordNames,
    /// **This document's own fetch under another spelling** — the stored-name
    /// outcome (`Desktop app and delivery` C15/C19) or a byte comparison maps
    /// the taken name to a *fetched* entry. **On Windows the outcome says
    /// *taken* without the stored name** (`Desktop app and delivery` C18) and
    /// the saying carries no name, which is that limit and not a new one.
    ThisDocumentsOwnFetchUnderAnotherSpelling,
    /// **A *fetched* entry of this arrangement at the byte-equal landing
    /// path** — the same spelling, whatever put the create in flight.
    AFetchedEntryAtTheByteEqualLandingPath,
    /// **Its own reservation.**
    ItsOwnReservation,
}

/// **P6: where the exclusive create fails, the name is taken, and the
/// reckoning says by what — which is Row 212's second promise kept at the only
/// moment it can be kept.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheRefusalMeans {
    /// **Left alone.** The transfer ends *stopped* (`Server protocol and
    /// transfers` S26's second row, the record written before the channel is
    /// closed), the staged material is removed at once or owed under S27, and
    /// the file is said once on the request or arrangement with the list of
    /// such files behind it — the spec's *"passes over that one file, carries
    /// on with the rest, and tells the user which files it left behind"*.
    ///
    /// **The bytes crossed once and were discarded**; the down direction is
    /// the fast one, and the cost is one crossing per file the arrangement did
    /// not put there — bounded by how much of the job the user fetched by hand
    /// and by nothing else. The alternatives are the already-exists question
    /// the reckoning forbids and a reservation held for the length of every
    /// transfer, **so the crossing is paid.**
    LeftAlone,
    /// **Cannot land.** The destination treats two of the machine's names as
    /// one — the seventeen-to-fourteen loss arriving at this document's door —
    /// **nothing is overwritten**, and the file is said with that cause.
    CannotLand,
    /// **Nothing is written over**, the transfer ends *stopped* with its
    /// staged material removed at once or owed, the entry stays *fetched*, and
    /// **it is said as the newer-version telling**.
    ///
    /// Because the fetch decision consults the reckoning by predicted path and
    /// ignores the attempt, that telling normally reaches this before any
    /// transfer is arranged; **this arm exists because the exclusive create is
    /// the last word and every way it can fail is owed an answer here rather
    /// than left to a builder to invent.**
    NothingIsWrittenOverAndItIsToldAsANewerVersion,
    /// **Cannot arise at fetch time**: the run-start requirement has every
    /// publication-pending record completed, or its reservation removed under
    /// `Desktop app and delivery` C13, before this document admits any
    /// transfer.
    CannotAriseAtFetchTime,
}

/// **P6, as one total function.**
pub fn what_the_refusal_means(taken_by: &WhatTheTakenNameIs) -> WhatTheRefusalMeans {
    match taken_by {
        WhatTheTakenNameIs::NothingTheRecordNames => WhatTheRefusalMeans::LeftAlone,
        WhatTheTakenNameIs::ThisDocumentsOwnFetchUnderAnotherSpelling => {
            WhatTheRefusalMeans::CannotLand
        }
        WhatTheTakenNameIs::AFetchedEntryAtTheByteEqualLandingPath => {
            WhatTheRefusalMeans::NothingIsWrittenOverAndItIsToldAsANewerVersion
        }
        WhatTheTakenNameIs::ItsOwnReservation => WhatTheRefusalMeans::CannotAriseAtFetchTime,
    }
}

/// **P2: the fetch order within an arrangement is frame order, earliest
/// judged-finished first; within a manual request, the enumeration's order.**
///
/// The second is the plain reading that a folder arrives roughly front to back
/// (`File Browser` O7's shape). **Admission is by class then arrival at the
/// per-server point**, which is [`crate::classes`]'s and not this function's.
///
/// **The arrangement's order is the attempt's order and nothing else, and this
/// function does not sort.** J2 forbids a sort in terms — *"a range is never
/// the order … nothing re-sorts, re-derives or infers the order from a
/// range"* — so *earliest judged-finished first* means earliest **in the
/// attempt's order**, not lowest by frame number. A list written `50, 10, 20`
/// is that order, and sorting it would fetch frame 10 first while the render
/// wrote 50 first.
///
/// **This is the one answer to the question, and
/// [`crate::acting::running`]'s own ordering value is read through the same
/// order.**
///
/// **A judged frame the order does not name keeps the tail**, which is the
/// same place the sequencer gives it. It arises where the attempt record could
/// not be read at all, and where it could not the frames keep the order they
/// were judged in — **nothing is inferred to put them in another one.**
pub fn the_fetch_order(
    arranged_by: WhatArrangedIt,
    order: &crate::vouching::TheOrder,
    the_frames_judged_finished: &[i64],
    the_enumerations_order: &[usize],
) -> Vec<TheNextUnit> {
    match arranged_by {
        WhatArrangedIt::AnAutoDownloadArrangement => {
            let crate::vouching::TheOrder::The(order) = order else {
                return the_frames_judged_finished
                    .iter()
                    .copied()
                    .map(TheNextUnit::TheFrame)
                    .collect();
            };
            let mut out: Vec<TheNextUnit> = order
                .frames()
                .iter()
                .filter(|frame| the_frames_judged_finished.contains(frame))
                .map(|frame| TheNextUnit::TheFrame(*frame))
                .collect();
            out.extend(
                the_frames_judged_finished
                    .iter()
                    .filter(|frame| order.position_of(**frame).is_none())
                    .map(|frame| TheNextUnit::TheFrame(*frame)),
            );
            out
        }
        WhatArrangedIt::AManualRequest => the_enumerations_order
            .iter()
            .map(|at| TheNextUnit::TheEnumerationEntryAt(*at))
            .collect(),
    }
}

/// What the fetch order names next, which is a frame on one side and an
/// enumeration position on the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheNextUnit {
    TheFrame(i64),
    TheEnumerationEntryAt(usize),
}

/// **P3: a failed fetch is a recorded unit outcome, and trying again is a new
/// transfer with a new identity** (`State, identity and recovery` §9.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAFailedFetchBecomes {
    /// **For an arrangement**: a unit that failed for a server-side or
    /// transfer reason returns to *owed* and is re-arranged at a later walk
    /// cycle — **the arrangement is standing work, and each fresh fetch is its
    /// own action.**
    ItReturnsToOwedAndIsReArrangedAtALaterWalkCycle,
    /// **For a manual request**: the unit stays failed, with its per-file Try
    /// again being the user's act (Design's control).
    ItStaysFailedAndTryingAgainIsTheUsersAct,
}

/// **P3, by which half arranged it.**
pub fn what_a_failed_fetch_becomes(arranged_by: WhatArrangedIt) -> WhatAFailedFetchBecomes {
    match arranged_by {
        WhatArrangedIt::AnAutoDownloadArrangement => {
            WhatAFailedFetchBecomes::ItReturnsToOwedAndIsReArrangedAtALaterWalkCycle
        }
        WhatArrangedIt::AManualRequest => {
            WhatAFailedFetchBecomes::ItStaysFailedAndTryingAgainIsTheUsersAct
        }
    }
}

/// **P3's own rule, held as a value: no unit is retried inside its action.**
///
/// A fresh fetch is a new transfer with a new identity, which is
/// `State, identity and recovery` §9.5's own sentence and is why the
/// arrangement's standing work is a sequence of actions rather than one action
/// that keeps trying.
pub const TRYING_AGAIN_IS_A_NEW_TRANSFER_WITH_A_NEW_IDENTITY: bool = true;

/// **P1: a fetch is one file's transfer under `Server protocol and transfers`
/// §7, unmodified.**
///
/// Bound to the server row, its own transfer identity, staged under S4's path
/// in the staging directory inside the destination folder (S1), recorded under
/// S9 before its first byte, integrity-checked under S21 to S25, published
/// under S15 and S18 through the platform file adapter, ended under S26.
/// **The arrangement and the manual request add the layer above: which
/// transfers exist, what class they carry, and what the whole means.**
pub const A_FETCH_IS_ONE_FILES_TRANSFER_UNDER_SECTION_SEVEN_UNMODIFIED: bool = true;

/// **The staging directory a fetch stages into: `Server protocol and
/// transfers` S1's, which is inside the user's own destination folder.**
///
/// The reserved name is `File Browser`'s already — one fact, one home — and
/// this crate composes the path rather than choosing the name.
pub fn the_staging_directory_inside(destination: &[u8]) -> Vec<u8> {
    let mut out = destination.to_vec();
    if !out.ends_with(b"/") {
        out.push(b'/');
    }
    out.extend_from_slice(smoozy_file_browser::partial::THE_RESERVED_STAGING_FOLDER.as_bytes());
    out
}

/// **P11: inside the folder, the structure the files had on the server is the
/// structure they have when they land.**
///
/// Each file's landing path is the destination plus its path relative to what
/// was asked for, component for component, **names carried as the machine's
/// bytes and never rewritten in flight** (`File Browser` N2's discipline in
/// the other direction) — except where a component fails the name check, which
/// is P8's per-file refusal for everything beneath it, reported on that
/// branch's files.
pub fn the_landing_path(destination: &[u8], relative_to_what_was_asked_for: &[u8]) -> Vec<u8> {
    let mut out = destination.to_vec();
    if !out.ends_with(b"/") {
        out.push(b'/');
    }
    out.extend_from_slice(
        relative_to_what_was_asked_for
            .strip_prefix(b"/".as_ref())
            .unwrap_or(relative_to_what_was_asked_for),
    );
    out
}

/// **P10, P11, M9 and H5a as one composition: where one file of a whole job
/// lands.**
///
/// **The destination is the folder that *contains* the job's folder, and this
/// is the one place the job's folder is put between them.** Both routes come
/// through here — an arrangement, whose destination was chosen at the
/// switch-on, and a whole-job download the user starts by hand from a queue or
/// history entry — because the spec says the folder holds *"whichever way they
/// were asked for"*, and *"two jobs' outputs no more mix on the user's disk
/// than they do on the server's"*.
///
/// **The folder's name is P10's composition and never the machine's**, so a
/// label legal on the machine and unwritable on the destination costs the
/// label's rendering and never the folder.
///
/// **H5a's arithmetic is what fixes the destination's depth**: a manual
/// request brought a job home as a whole job where the landing path relative
/// to the destination is one component longer than the source path relative to
/// the job's output folder, **which is true only where the destination sits
/// above the job's folder.** And H5 draws an arrangement's per-entry
/// destination and a request's destination side by side as places of one kind,
/// so the two must mean the same thing.
pub fn the_landing_path_inside_the_jobs_folder<R: TheReceivingSystemsRules + ?Sized>(
    rules: &R,
    the_destination: &[u8],
    job: &Identity,
    label: Option<&str>,
    relative_to_the_jobs_output_folder: &[u8],
) -> Vec<u8> {
    let the_jobs_folder = the_landing_path(
        the_destination,
        &the_job_folders_landing_name(rules, job, label),
    );
    the_landing_path(&the_jobs_folder, relative_to_the_jobs_output_folder)
}

/// **P10: the job folder's landing name is Smoozy's own composition and is
/// rendered against the receiving system's rules, by the same rule as on the
/// machine.**
///
/// The machine-side name is `Job files and outputs` N3's: the job identity's
/// textual form plus a bounded rendering of the label, **substitution rather
/// than refusal**. The landing name is composed the same way — the identity's
/// textual form (fixed-length lowercase letters and digits, safe in every
/// system's unit) plus a bounded rendering of the label judged by
/// `Desktop app and delivery` C3 against the **receiving** system, with what
/// cannot enter the name substituted and **the composed name judged again like
/// any other** (`Desktop app and delivery` C20).
///
/// **So a label legal on Linux and unwritable as given on the destination — a
/// trailing period on Windows is the case that will bite — costs the label's
/// rendering and never the folder or the files**, and two jobs given one label
/// land in two folders because the identity is in the name.
///
/// **The full label stays authoritative on the entry, and the folder's real
/// name is what paths show** (`Queue & History` Y8's presentation rule,
/// unchanged). **Nothing parses a folder name back into anything.**
pub fn the_job_folders_landing_name<R: TheReceivingSystemsRules + ?Sized>(
    rules: &R,
    job: &Identity,
    label: Option<&str>,
) -> Vec<u8> {
    debug_assert_eq!(job.kind(), IdentityKind::Job);
    let identity = job.as_text().into_bytes();
    let Some(label) = label else {
        return identity;
    };
    // The identity's own text is safe in every system's unit, so what is left
    // over is the label's whole budget — the same arithmetic the machine side
    // uses, taken in the receiving system's own unit rather than in bytes.
    let taken = rules.length_in_this_systems_own_unit(&identity);
    let Some(room) = rules.the_longest_component().checked_sub(taken + 1) else {
        return identity;
    };
    let substituted = rules.substitute(label.as_bytes());
    let rendered = trim_to(rules, &substituted, room);
    if rendered.is_empty() {
        return identity;
    }
    let mut composed = identity.clone();
    composed.push(b' ');
    composed.extend_from_slice(&rendered);
    // C20: the composed name is judged again like any other.
    if rules.check(&composed).is_acceptable_as_given() {
        composed
    } else {
        identity
    }
}

/// Trim a rendering to the room left, **in the receiving system's own unit**,
/// and at a character boundary where the bytes decode — never mid-character,
/// so the rendering is always text the user can read back.
fn trim_to<R: TheReceivingSystemsRules + ?Sized>(
    rules: &R,
    rendering: &[u8],
    room: usize,
) -> Vec<u8> {
    if rules.length_in_this_systems_own_unit(rendering) <= room {
        return rendering.to_vec();
    }
    let mut at = rendering.len();
    while at > 0 {
        at -= 1;
        if std::str::from_utf8(rendering).is_ok() {
            let text = std::str::from_utf8(rendering).expect("just checked");
            if !text.is_char_boundary(at) {
                continue;
            }
        }
        if rules.length_in_this_systems_own_unit(&rendering[..at]) <= room {
            return rendering[..at].to_vec();
        }
    }
    Vec::new()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A stand-in for the platform file adapter, with one system's rules
    /// written by hand. **It is a test double for a judgement this crate does
    /// not make**, which is the whole point of the trait.
    pub(crate) struct ASystem {
        pub the_longest: usize,
        pub refuses: Vec<u8>,
        /// A component ending in one of these is silently transformed — the
        /// Windows trailing-period case P10 names.
        pub transforms_a_trailing: Vec<u8>,
    }

    impl ASystem {
        pub(crate) fn plain() -> ASystem {
            ASystem {
                the_longest: 255,
                refuses: vec![b'/', 0],
                transforms_a_trailing: Vec::new(),
            }
        }
    }

    impl TheReceivingSystemsRules for ASystem {
        fn check(&self, component: &[u8]) -> TheNameCheckSaid {
            if component.iter().any(|byte| self.refuses.contains(byte)) {
                return TheNameCheckSaid::NotRepresentable;
            }
            if component
                .last()
                .is_some_and(|byte| self.transforms_a_trailing.contains(byte))
            {
                return TheNameCheckSaid::SilentlyTransformed;
            }
            if component.len() > self.the_longest {
                return TheNameCheckSaid::TooLongInThisSystemsOwnUnit;
            }
            TheNameCheckSaid::TheNameMayBeWrittenAsGiven
        }

        fn substitute(&self, component: &[u8]) -> Vec<u8> {
            component
                .iter()
                .map(|byte| {
                    if self.refuses.contains(byte) {
                        b'_'
                    } else {
                        *byte
                    }
                })
                .collect()
        }

        fn length_in_this_systems_own_unit(&self, component: &[u8]) -> usize {
            component.len()
        }

        fn the_longest_component(&self) -> usize {
            self.the_longest
        }
    }

    fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).expect("the desk has randomness")
    }

    #[test]
    fn only_one_of_the_four_name_check_outcomes_is_acceptable_as_given() {
        assert!(TheNameCheckSaid::TheNameMayBeWrittenAsGiven.is_acceptable_as_given());
        for refused in [
            TheNameCheckSaid::NotRepresentable,
            TheNameCheckSaid::TooLongInThisSystemsOwnUnit,
            TheNameCheckSaid::SilentlyTransformed,
        ] {
            assert!(!refused.is_acceptable_as_given());
            assert!(refused.as_a_refusal().is_some());
        }
    }

    #[test]
    fn the_check_names_the_component_it_refused_and_depth_alone_is_never_a_refusal() {
        let system = ASystem::plain();
        let deep: Vec<&[u8]> = vec![b"a", b"b", b"c", b"d", b"e", b"f", b"g"];
        assert!(judge_the_landing_path(&system, &deep).is_ok());
        let bad: Vec<&[u8]> = vec![b"a", b"b\0c", b"d"];
        assert_eq!(
            judge_the_landing_path(&system, &bad),
            Err((1, WhichRefusalItWas::NotRepresentable))
        );
    }

    #[test]
    fn an_arrangement_never_answers_replace_and_a_manual_replace_is_the_one_exception() {
        assert!(!an_arrangement_may_answer_replace());
        assert_eq!(the_publication_intent(false), Intent::CreateIntended);
        assert_eq!(the_publication_intent(true), Intent::ReplaceIntended);
    }

    #[test]
    fn every_way_the_exclusive_create_can_fail_has_an_answer_and_none_writes_over_anything() {
        assert_eq!(
            what_the_refusal_means(&WhatTheTakenNameIs::NothingTheRecordNames),
            WhatTheRefusalMeans::LeftAlone
        );
        assert_eq!(
            what_the_refusal_means(&WhatTheTakenNameIs::ThisDocumentsOwnFetchUnderAnotherSpelling),
            WhatTheRefusalMeans::CannotLand
        );
        assert_eq!(
            what_the_refusal_means(&WhatTheTakenNameIs::AFetchedEntryAtTheByteEqualLandingPath),
            WhatTheRefusalMeans::NothingIsWrittenOverAndItIsToldAsANewerVersion
        );
        assert_eq!(
            what_the_refusal_means(&WhatTheTakenNameIs::ItsOwnReservation),
            WhatTheRefusalMeans::CannotAriseAtFetchTime
        );
    }

    #[test]
    fn the_name_check_runs_at_enumeration_for_a_request_and_at_fetchability_for_an_arrangement() {
        assert_eq!(
            WhenTheNameCheckRuns::for_(WhatArrangedIt::AManualRequest),
            WhenTheNameCheckRuns::AtEnumeration
        );
        assert_eq!(
            WhenTheNameCheckRuns::for_(WhatArrangedIt::AnAutoDownloadArrangement),
            WhenTheNameCheckRuns::AtTheMomentAFileFirstBecomesFetchable
        );
    }

    /// **P2 read against J2: the arrangement's order is the attempt's order,
    /// and it is never the frame's number.**
    ///
    /// The attempt was written `50, 10, 20`, so the render wrote 50 first and
    /// the fetch order follows it. A sort by number would fetch 10 first —
    /// which is the reading J2 refuses in terms.
    #[test]
    fn an_arrangement_fetches_in_the_attempts_order_and_a_request_in_the_enumerations_order() {
        let order = crate::vouching::TheOrder::The(
            crate::vouching::TheAttemptsOrder::as_the_attempt_record_carries_it(vec![50, 10, 20]),
        );
        assert_eq!(
            the_fetch_order(
                WhatArrangedIt::AnAutoDownloadArrangement,
                &order,
                &[20, 50, 10],
                &[]
            ),
            vec![
                TheNextUnit::TheFrame(50),
                TheNextUnit::TheFrame(10),
                TheNextUnit::TheFrame(20)
            ]
        );
        // Only the frames judged finished are named, in that same order.
        assert_eq!(
            the_fetch_order(
                WhatArrangedIt::AnAutoDownloadArrangement,
                &order,
                &[20, 50],
                &[]
            ),
            vec![TheNextUnit::TheFrame(50), TheNextUnit::TheFrame(20)]
        );
        assert_eq!(
            the_fetch_order(WhatArrangedIt::AManualRequest, &order, &[], &[3, 0, 1]),
            vec![
                TheNextUnit::TheEnumerationEntryAt(3),
                TheNextUnit::TheEnumerationEntryAt(0),
                TheNextUnit::TheEnumerationEntryAt(1)
            ]
        );
    }

    /// **A frame the order does not name keeps the tail, and an unreadable
    /// order sorts nothing.**
    #[test]
    fn nothing_is_sorted_where_the_order_could_not_be_read() {
        assert_eq!(
            the_fetch_order(
                WhatArrangedIt::AnAutoDownloadArrangement,
                &crate::vouching::TheOrder::ItCouldNotBeRead,
                &[7, 2, 5],
                &[]
            ),
            vec![
                TheNextUnit::TheFrame(7),
                TheNextUnit::TheFrame(2),
                TheNextUnit::TheFrame(5)
            ]
        );
        let order = crate::vouching::TheOrder::The(
            crate::vouching::TheAttemptsOrder::as_the_attempt_record_carries_it(vec![50, 10]),
        );
        assert_eq!(
            the_fetch_order(
                WhatArrangedIt::AnAutoDownloadArrangement,
                &order,
                &[99, 10, 50],
                &[]
            ),
            vec![
                TheNextUnit::TheFrame(50),
                TheNextUnit::TheFrame(10),
                TheNextUnit::TheFrame(99)
            ]
        );
    }

    #[test]
    fn a_failed_unit_returns_to_owed_for_an_arrangement_and_stays_failed_for_a_request() {
        assert_eq!(
            what_a_failed_fetch_becomes(WhatArrangedIt::AnAutoDownloadArrangement),
            WhatAFailedFetchBecomes::ItReturnsToOwedAndIsReArrangedAtALaterWalkCycle
        );
        assert_eq!(
            what_a_failed_fetch_becomes(WhatArrangedIt::AManualRequest),
            WhatAFailedFetchBecomes::ItStaysFailedAndTryingAgainIsTheUsersAct
        );
    }

    #[test]
    fn staging_sits_inside_the_destination_folder_under_the_name_file_browser_already_reserved() {
        assert_eq!(
            the_staging_directory_inside(b"/Users/a/Downloads"),
            b"/Users/a/Downloads/.smoozy-staging".to_vec()
        );
        assert_eq!(
            the_staging_directory_inside(b"/Users/a/Downloads/"),
            b"/Users/a/Downloads/.smoozy-staging".to_vec()
        );
    }

    #[test]
    fn the_structure_on_the_server_is_the_structure_that_lands() {
        assert_eq!(
            the_landing_path(b"/Users/a/Downloads", b"beauty/frame_0001.exr"),
            b"/Users/a/Downloads/beauty/frame_0001.exr".to_vec()
        );
        assert_eq!(
            the_landing_path(b"/Users/a/Downloads/", b"/beauty/frame_0001.exr"),
            b"/Users/a/Downloads/beauty/frame_0001.exr".to_vec()
        );
    }

    #[test]
    fn the_job_folders_landing_name_carries_the_identity_so_two_labels_alike_land_in_two_folders() {
        let system = ASystem::plain();
        let one = a_job();
        let other = a_job();
        let first = the_job_folders_landing_name(&system, &one, Some("night render"));
        let second = the_job_folders_landing_name(&system, &other, Some("night render"));
        assert_ne!(first, second);
        assert!(first.starts_with(one.as_text().as_bytes()));
        assert!(second.starts_with(other.as_text().as_bytes()));
    }

    #[test]
    fn a_label_the_receiving_system_will_not_take_costs_the_label_and_never_the_folder() {
        // The Windows trailing-period case P10 names by name.
        let windows = ASystem {
            the_longest: 255,
            refuses: vec![b'/', 0],
            transforms_a_trailing: vec![b'.'],
        };
        let job = a_job();
        assert_eq!(
            the_job_folders_landing_name(&windows, &job, Some("shot 12.")),
            job.as_text().into_bytes()
        );
        // And the same label on a system that takes it keeps its rendering.
        let linux = ASystem::plain();
        let kept = the_job_folders_landing_name(&linux, &job, Some("shot 12."));
        assert!(kept.ends_with(b"shot 12."));
    }

    #[test]
    fn a_label_is_substituted_rather_than_refused_and_a_long_one_costs_only_its_rendering() {
        let system = ASystem::plain();
        let job = a_job();
        let substituted = the_job_folders_landing_name(&system, &job, Some("a/b"));
        assert!(substituted.ends_with(b"a_b"));

        let long = "x".repeat(400);
        let composed = the_job_folders_landing_name(&system, &job, Some(&long));
        assert!(composed.len() <= system.the_longest);
        assert!(composed.starts_with(job.as_text().as_bytes()));
        assert!(composed.len() > job.as_text().len());
    }

    #[test]
    fn a_job_with_no_label_lands_under_the_identity_alone() {
        let system = ASystem::plain();
        let job = a_job();
        assert_eq!(
            the_job_folders_landing_name(&system, &job, None),
            job.as_text().into_bytes()
        );
    }
}
