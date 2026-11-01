//! What of this document is not built here, named clause by clause rather than
//! silently absent — and what was met and built rather than repaired.
//!
//! **`Preparation/Build/Plan.md` step 6 builds this module**, and **every one of
//! the plan's 239 clauses is either named by working code in this crate or stands
//! on [`NOT_BUILT_HERE`] with what removes it** — a test below finds each naming
//! site rather than trusting a section number, and reads that list against the
//! plan rather than taking it on trust.
//!
//! **[`NOT_BUILT_HERE`] is empty and every clause is named by working code
//! again.** It held one entry for one day: the plan gained G14a on 2026-09-06,
//! for a rollback this crate composed and nothing performed, and **the entry
//! retired on 2026-09-07 when the desktop's placement performer built the
//! rollback and G14a with it**, which is what that entry said would remove it.
//! **The second limb stays** — a clause may still stand on that list rather than
//! on a naming site — and it is not an escape: the list is checked against the
//! plan, and a clause that is neither built nor recorded fails exactly as it did
//! before.
//!
//! **Naming is not driving, and this page says where the two come apart.**
//! [`THE_DECLARED_AND_NOT_DRIVEN`] names the hold evaluation at U19's moment,
//! which nothing calls there, three ordered sequences the plan states that exist
//! here as arrays nothing performs, and the worker's own capture loop, which is
//! written nowhere. **They are not unbuilt clauses** — each is named,
//! each has its rule and its tests — **and they are not built things either**, so
//! a reader told to consult this page must meet them here rather than discover
//! them.
//!
//! **[`THE_ACTING_HALF`] is this crate's twenty-two traits, each with the clauses
//! it serves and where its real end is.** It used to say that none of them had
//! one anywhere in the workspace, then that eighteen did. **All twenty-two have
//! one now**, all in [`crate::acting`], which is the half that reaches the
//! machine. **The twenty-second is the role's own desired state** (W7, W8),
//! which landed on 2026-09-07 with the two acts `Server Setup` writes it
//! through.
//!
//! **The last three landed on 2026-09-04, when `Server protocol and transfers`
//! §2's request framing was built** — the health answer (G12), the worker's
//! report to the supervisor (W12) and V4's observed value, every one of which was
//! waiting on a frame rather than on a transport. **The record no longer carries
//! what each was missing, because none of them is missing anything**, and the
//! test below is what made that change compulsory rather than optional: an entry
//! saying it reaches no machine that something implements fails.
//!
//! **Z12's connect probe still does not stand in for V4, and that refusal
//! stands.** The probe asks whether anything holds the endpoint, which is a
//! reading of the connection — the one thing V4 says the observed value is not.
//! **A supervisor holding its endpoint and answering nothing would have read as
//! healthy.** [`crate::acting::readings`] sends a framed request instead.
//!
//! **It is not a list of unbuilt promises and must never be folded into one.**
//! The rule, the judgement over the reading and the tests are all in the rules
//! half; what an entry with no real end would be missing is the machine at the
//! other end of it.
//!
//! **[`THE_STORED_FIELD_HOLE`] is a third thing again**, and it is neither of the
//! two above. It is a permanently unresolved entry of the plan that this build
//! **met, built the *what holds* for, and did not rescue**. It is on this page
//! because a reader who meets the symptom in the field will otherwise go looking
//! for a bug that is not there.
//!
//! **[`WHAT_IS_ROUTED_TO_ANOTHER_CRATE`] is what this crate needs and cannot
//! write**, each with the document that owns it. **Nothing on it is stubbed.**
//! There is no always-yes answer, no empty value standing in for one, and no
//! default anywhere in this crate.
//!
//! **Markers used below that belong to another document, each with the document
//! it belongs to.** `Desktop app and delivery` §16.3. `Job files and outputs`
//! C27, and its §10. `Queue & History` — Row 54.
//!
//! **A marker this plan also defines is never listed here and is written out with
//! its document at the site**, because a header list cannot tell the two apart.

/// **A trait of this crate, and where its real end is.**
///
/// **An entry whose real end is absent is not an unbuilt clause and must never
/// be recorded as one.** The clause is built — the rule and the judgement over
/// the reading are in the rules half, with their tests — and what is missing is
/// the trait's real end: **nothing in the application implements it outside test
/// code**, so no caller can reach a machine through it.
///
/// **The clause accounting cannot catch either state, by design.** It asks
/// whether a clause is named by working code, not whether the named code can
/// reach a machine. What catches it is
/// [`the_record_of_real_ends_matches_the_application`](self::tests), which reads
/// every workspace member's source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ATraitOfThisCrate {
    /// The trait, by path.
    pub trait_path: &'static str,
    /// The clauses it serves.
    pub the_clauses_it_serves: &'static [&'static str],
    /// Where its real end is, or `None` where it has none.
    pub where_it_reaches_the_machine: Option<&'static str>,
    /// What is still missing. **Empty where nothing is.**
    pub what_is_still_missing: &'static str,
    /// What removes what is still missing. **Empty where nothing is owed.**
    pub what_removes_it: &'static str,
}

/// **The whole of this module's traits, trait by trait, with where each reaches
/// the machine.**
///
/// **An entry that is missing something says so at itself, and this note names
/// no count** — a count here is a second copy of a fact the list already holds,
/// and the entries move. **The one whose missing half is another document's is
/// the hook**: its own install on a container is `Server Setup`'s elevated step
/// (K11), which the end here leaves to that step rather than performing — K16's
/// record is written on both kinds and is not the missing half. **The others
/// each name their own remover**, and one of them names a routing question
/// rather than a piece of work.
///
/// **Both halves of the package claim exist now, and neither is owed.** G1a's
/// kernel-held lock on the claim file is taken on the machine, at
/// [`crate::acting::package`]; G1b's exclusive create through the file-transfer
/// subsystem is the desktop's, and `Server protocol and transfers` performs it
/// through the pre-package surface, which is the only claim a bare rented
/// machine ever has. *(Landed 2026-09-07 with the desktop-side placement
/// performer. Until then this note said two entries reached the machine only in
/// part, the claim being the second.)*
pub const THE_ACTING_HALF: [ATraitOfThisCrate; 22] = [
    ATraitOfThisCrate {
        trait_path: "crate::machine_kind::ReadsTheMachineKind",
        the_clauses_it_serves: &["K1"],
        where_it_reaches_the_machine: Some("crate::acting::machine_kind"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::registration::ReadsTheRegistrationBack",
        the_clauses_it_serves: &["K15", "K20"],
        where_it_reaches_the_machine: Some("crate::acting::registration"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::registration::ReadsTheLingerFlagBack",
        the_clauses_it_serves: &["K5", "K6", "K8"],
        where_it_reaches_the_machine: Some("crate::acting::registration"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::registration::PlacesTheRegistration",
        the_clauses_it_serves: &["K4", "K6", "K7", "K11", "K16", "K19"],
        where_it_reaches_the_machine: Some("crate::acting::registration"),
        what_is_still_missing: "the container's own write of the hook, which is root's. \
                                **The end here leaves it to that step naming K11 rather than \
                                performing it**, and K19 puts a second obligation on the same \
                                root step. **K16's record is written on both kinds** and is \
                                not what is missing. **From 2026-09-21 the desktop's own \
                                placement performs that root step** (`Server protocol and \
                                transfers`' `install_the_hook`, with `Server Setup`'s \
                                commands); **this end, which is the machine's own placement \
                                of a replacement, still does not**, having no elevation of its \
                                own",
        what_removes_it: "`Server Setup`, which owns the elevation",
    },
    ATraitOfThisCrate {
        trait_path: "crate::package::release::ReadsTheDurableRecordsAnInspectionIsComposedFrom",
        the_clauses_it_serves: &["G31a"],
        where_it_reaches_the_machine: Some("crate::acting::package"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::package::phase::ClaimsTheTransaction",
        the_clauses_it_serves: &["G1a", "G1b"],
        where_it_reaches_the_machine: Some("crate::acting::package"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::package::phase::WritesThePhaseRecord",
        the_clauses_it_serves: &["G1", "G2", "G3", "G4"],
        where_it_reaches_the_machine: Some("crate::acting::package"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::package::commit::ReadsWhatTheCommitRestsOn",
        the_clauses_it_serves: &["G11"],
        where_it_reaches_the_machine: Some("crate::acting::package"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::package::commit::AsksForTheHealthAnswer",
        the_clauses_it_serves: &["G12"],
        where_it_reaches_the_machine: Some("crate::acting::package"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::package::commit::PerformsTheCommit",
        the_clauses_it_serves: &["G7", "G25"],
        where_it_reaches_the_machine: Some("crate::acting::package"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::endpoint::DrivesTheEndpoint",
        the_clauses_it_serves: &["Z2", "Z6", "Z12"],
        where_it_reaches_the_machine: Some("crate::acting::endpoint"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::workers::ReportsToTheSupervisor",
        the_clauses_it_serves: &["W12"],
        where_it_reaches_the_machine: Some("crate::acting::workers"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::workers::StartsWorkers",
        the_clauses_it_serves: &["W3"],
        where_it_reaches_the_machine: Some("crate::acting::workers"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::execution_group::WaitsOnEveryWorkerItStarts",
        the_clauses_it_serves: &["X9a"],
        where_it_reaches_the_machine: Some("crate::acting::execution_group"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::execution_group::ValidatesTheRecordedPair",
        the_clauses_it_serves: &["X1", "X2", "X9"],
        where_it_reaches_the_machine: Some("crate::acting::execution_group"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::execution_group::IssuesTheKindsOwnTermination",
        the_clauses_it_serves: &["X8", "X11"],
        where_it_reaches_the_machine: Some("crate::acting::execution_group"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::holds::KeepsHoldRecords",
        the_clauses_it_serves: &["U2", "U12"],
        where_it_reaches_the_machine: Some("crate::acting::stores"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::setup_state::SetsAndClearsTheState",
        the_clauses_it_serves: &["P3", "P4", "P5"],
        where_it_reaches_the_machine: Some("crate::acting::stores"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::workers::KeepsTheRolesDesiredState",
        the_clauses_it_serves: &["W7", "W8"],
        where_it_reaches_the_machine: Some("crate::acting::stores"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::capture::MakesTheSinks",
        the_clauses_it_serves: &["M1"],
        where_it_reaches_the_machine: Some("crate::acting::capture"),
        what_is_still_missing: "",
        what_removes_it: "",
    },
    ATraitOfThisCrate {
        trait_path: "crate::capture::CopiesAStreamIntoItsSink",
        the_clauses_it_serves: &["M3", "M4", "M5"],
        where_it_reaches_the_machine: Some("crate::acting::capture"),
        what_is_still_missing: "the worker's own read-and-write loop. **The write reaches \
                                the machine and takes one buffer; nothing outside tests calls \
                                it in a loop**, so M4's count of what it writes and M5's \
                                keep-reading-and-discarding past the ceiling have no reader \
                                driving them, and `crate::capture::WhatTheWorkerObserved` is \
                                a shape nothing fills",
        what_removes_it: "the build step that writes the render worker; the entry is \
                          `THE_DECLARED_AND_NOT_DRIVEN`'s fifth",
    },
    ATraitOfThisCrate {
        trait_path: "crate::health_readings::TakesHealthReadings",
        the_clauses_it_serves: &["V4", "V6", "V13"],
        where_it_reaches_the_machine: Some("crate::acting::readings"),
        what_is_still_missing: "V6's processor and memory reading. **The trait's only V6 \
                                method is the card reading**, and there is no control-group \
                                reading of either generation anywhere in this crate, in \
                                either half — so V6's rule function has no reader behind it \
                                and the clause is served in part rather than whole. **The \
                                machine-side consumer it would be for is the `Telegram` role \
                                worker's status reading**, which is V2's single named \
                                exception and is built in no crate; `Servers` H9a performs \
                                the same reading from the desktop's own side",
        what_removes_it: "whichever document the reader is owed to — the routing question \
                          is the runner's, between this document's §12 and `Telegram` §10",
    },
];

/// **The known hole this step's milestone carries a qualifier for, met and built
/// rather than rescued.**
///
/// **`Rented-server runtime` §16 item 16 is permanently unresolved.** Three of the
/// four container providers read **hold a stored start-command field rather than a
/// boot-hook file path**, and on those the placement commits, the read-back
/// confirms Smoozy's own bytes, **and nothing ever runs them**.
///
/// **What was built is the *what holds*, and nothing else.**
/// [`crate::acting::registration`] reads the registration's own bytes back at the
/// path K10 names; [`crate::registration::confirm`] compares them against the
/// record Smoozy wrote; and **there is no second read-back, no probe, no marker,
/// no heartbeat and no reporting channel anywhere in this crate — in either half
/// — that would notice a hook which never ran.**
///
/// **Why there is none, structurally rather than as a gap in what was looked for.**
/// Every route this document holds reads either Smoozy's own bytes or the machine's
/// own state, and **neither carries the provider's start-up behaviour**; K1's two
/// container readings do not read which start-up mechanism a provider offers; and
/// **no health reading is of the hook**.
///
/// **What a builder sees instead, so nobody hunts for a bug that is not there.**
/// After a restart on such a provider the supervisor is not running, so the next
/// request on the command path goes unanswered and the machine **is** the
/// *supervisor unavailable* condition — which is `Desktop app and delivery`'s
/// automatic repair trigger. **So such a machine repairs, works until its next
/// restart, and repairs again, indefinitely, and nothing anywhere names the
/// cause.** A machine that needs repairing after every restart is this entry, seen
/// from the outside.
///
/// **Both routes out are spent** — no provider documents whether the hook runs
/// again on a restart, and the Creator ruled on 2026-08-31 that there will be no
/// further rented machine for answering a question. **This is not a pending item,
/// and no space is left for one in the code or in this record.**
pub const THE_STORED_FIELD_HOLE: &str = "three of the four container providers \
read hold a stored start-command field rather than a boot-hook file path; on \
those the placement commits, the read-back confirms Smoozy's own bytes, and \
nothing ever runs them; the symptom is a machine that repairs after every \
restart, and no clause here names the cause because no reading on the machine \
carries it";

/// **Something the plan states that is declared here and driven by nothing.**
///
/// **This is a third kind of entry again**, and it is neither an unbuilt clause
/// nor a trait with no real end. The thing is named, its rule is written, and its
/// order or its shape is held in this crate — **and nothing performs it.** A
/// reader who takes *every clause is named by working code* to mean *every clause
/// is driven* is reading this page for exactly the thing it must not leave out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeclaredAndNotDriven {
    /// What is declared, by path.
    pub what_is_declared: &'static str,
    /// The clauses it holds.
    pub the_clauses_it_holds: &'static [&'static str],
    /// What is missing, said plainly.
    pub what_nothing_performs: &'static str,
    /// Which build step drives it.
    pub which_step_drives_it: &'static str,
}

/// **The five things this crate declares and nothing here drives.**
///
/// **The first is the hold evaluation at `Rented-server runtime` U19's moment**,
/// which nothing calls there; its other moment, U19a's, is driven.
///
/// **Three are ordered sequences the plan states.** Each exists as an
/// array in the order the plan gives, each is referenced by one test and by
/// nothing else, and **no function performs one.** So G10's *the identity is
/// durable before the pointer switch* and X8 step 2's *the desired state is
/// written before the group is looked at* exist here **as the order of an array
/// and not as an order anything keeps.**
///
/// **The fifth is the worker's own capture loop.** M4's count of what it writes
/// and M5's keep-reading-and-discarding past the ceiling are the loop's, and the
/// loop is written nowhere: [`crate::capture::CopiesAStreamIntoItsSink::write`]
/// takes one buffer and has no looping caller outside tests, and
/// [`crate::capture::WhatTheWorkerObserved`] is a shape nothing fills.
///
/// **None of the five is written here and none is to be.** Writing a driver
/// means deciding what it takes and who calls it, which is the wiring a later
/// build step owns; **what this page owes is the truth about what is missing.**
pub const THE_DECLARED_AND_NOT_DRIVEN: [DeclaredAndNotDriven; 5] = [
    DeclaredAndNotDriven {
        what_is_declared: "crate::holds::evaluate",
        the_clauses_it_holds: &["U17", "U18", "U19", "U20", "U30"],
        what_nothing_performs: "**the evaluation at U19's moment**, the writing of the \
                                dependency's recorded outcome. The judgement is built and \
                                tested — the moment, the reading that must ride with the \
                                ending, the refusal to substitute a reading taken before the \
                                dependency ran — **and nothing calls it there.** That moment is \
                                reconciliation's two terminal dispositions and a worker's own \
                                ending step. **So a hold whose dependency ran and ended badly \
                                is never dropped**, and the waiter stands until it is answered. \
                                **U19a's moment is driven**: the removal of the dependency's \
                                job record calls it through the queue crate's \
                                `evaluate_the_predicate` (`Queue & History` H9, step 10c's S3). \
                                Added 2026-09-12, out of the step 8b code round, which found it \
                                named nowhere; narrowed to U19 on 2026-09-29",
        which_step_drives_it: "the build step that wires the render kind's own ending step and \
                               reconciliation's terminal dispositions to U19's moment",
    },
    DeclaredAndNotDriven {
        what_is_declared: "crate::package::commit::CommitStep::IN_ORDER",
        the_clauses_it_holds: &["G10", "G7", "G25"],
        what_nothing_performs: "the commit's own steps in that order. **The array is the \
                                order and nothing walks it**, so G10's *the identity is \
                                durable before the pointer switch* is an order written \
                                down rather than an order kept",
        which_step_drives_it: "the build step that wires the package transaction's caller",
    },
    DeclaredAndNotDriven {
        what_is_declared: "crate::execution_group::StopStep::IN_ORDER",
        the_clauses_it_holds: &["X8"],
        what_nothing_performs: "the seven-step stop. **The array is the order and nothing \
                                walks it**, so X8 step 2's *the desired state is written \
                                before the group is looked at* is an order written down \
                                rather than an order kept",
        which_step_drives_it: "the build step that wires the supervisor's own command path",
    },
    DeclaredAndNotDriven {
        what_is_declared: "crate::setup_state::TheOrdering::IN_ORDER",
        // **P6 is on this list from 2026-09-07**, because P5's own act is
        // performed now and the ordering's last step is P6's. The array holds
        // both and it always did; naming P5 alone had been enough while P5 was
        // what nothing performed.
        the_clauses_it_holds: &["P5", "P6"],
        what_nothing_performs: "**the fourth step, for five of the six kinds.** \
                                Narrowed three times. Twice on 2026-09-07: first when \
                                the machine's program wired P3's two acts to the command \
                                path, which performs the clear; then when reconciliation \
                                landed, which performs the second half of P5 — running \
                                immediately after the clear, at the moment the clear alone \
                                produces. **And again on 2026-09-19**, when step 10c \
                                piece 5's line began starting the render kind's second \
                                attempt off *re-run once*: this said no code starts a \
                                second attempt at all, and for a render that is no longer \
                                true. **What no code starts is a second attempt at an \
                                operation of the other five kinds** — a setup run's own \
                                re-run is `Server Setup` S4's recovery command, which the \
                                desktop issues. **The array is the order and nothing walks \
                                it**",
        which_step_drives_it: "the build step that wires reconciliation",
    },
    DeclaredAndNotDriven {
        what_is_declared: "crate::capture::CopiesAStreamIntoItsSink",
        the_clauses_it_holds: &["M4", "M5"],
        what_nothing_performs: "the worker's own read-and-write loop. **The write takes \
                                one buffer and there is no looping caller outside tests**, \
                                so M4's count of what it writes and M5's \
                                keep-reading-and-discarding past the ceiling are the rule \
                                and the arithmetic without the loop they are for, and \
                                `crate::capture::WhatTheWorkerObserved` is a shape nothing \
                                fills",
        which_step_drives_it: "the build step that writes the render worker",
    },
];

/// **One clause of this plan that no working file of this crate names, with
/// what removes it.**
///
/// **This is a fourth kind of entry and it is the one this page did not have.**
/// The three above are a clause that is built and cannot reach a machine, a
/// clause that is declared and driven by nothing, and a thing another document
/// owns. **This one is a clause nothing here keeps at all** — not named, not
/// declared, not routed away — and the entry is the whole account of why.
///
/// **It is not an escape from the accounting.** The forward-leg test reads this
/// list against the plan: an entry filed under the wrong section fails, an
/// entry missing any of its four sentences fails, an entry for a clause working
/// code does name fails, and a clause that is neither built nor recorded here
/// still fails the way it did before this list existed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotNamedByWorkingCode {
    /// The plan's own top-level section.
    pub section: &'static str,
    /// Every clause of that section this covers.
    pub clauses: &'static [&'static str],
    /// What the clause is.
    pub what: &'static str,
    /// Why no working file of this crate names it.
    pub why: &'static str,
    /// **What removes it** — the work that makes the clause true here, named
    /// exactly enough that a reader can go and find it.
    pub what_removes_it: &'static str,
}

/// **Nothing. The list is empty and stays here so a reader meets why.**
///
/// ~~**Added 2026-09-06, the day the plan gained G14a.** The clause completes
/// G14 with the half it never carried, and **the thing it completes is built
/// nowhere**: this crate composes the rollback decision and nothing carries it
/// out. So G14a is not a promise that was missed — it is a promise whose
/// performer has not been written, and building the record's restore without
/// the registration's restore beside it would put a half-act on a path nothing
/// walks.~~
///
/// **RETIRED 2026-09-07, the day after it was written, because G14a was built.**
/// The entry's own *what removes it* said whoever builds the rollback performer
/// builds G14a in the same act and that the two must never be scheduled apart,
/// **and that is what happened**: `Server protocol and transfers`' desktop-side
/// placement performer carries out the rollback through the pre-package surface,
/// and the record's own restore goes with it. **The rules half of both is here**
/// — [`crate::package::commit::the_record_a_rollback_restores`] composes the
/// record to put back, and [`crate::package::commit::TheRestoresTwoSteps`] is
/// K16b's order for the pair — and both name G14a now, which is why the clause
/// no longer needs this list.
///
/// **What is still not built here, said plainly rather than left to be inferred:
/// the machine's own performer has no restore.**
/// [`crate::package::commit::PerformsTheCommit`] still has the pointer rename,
/// the supervisor's end and its start, and no restore of anything — so a
/// rollback carried out by a machine's own live runtime, rather than by the
/// desktop through the surface, is a path nothing walks. **That is a performer
/// this crate has not been given and not a clause nobody kept**: G14 and G14a
/// are both kept, by the performer that exists. This sentence is the only place
/// the gap is written down, which is why the note stays.
///
/// **The list stays rather than being deleted**, because the forward leg's
/// second limb reads it and an empty list is the honest answer to *which clauses
/// stand on a record instead of on code*: none.
pub const NOT_BUILT_HERE: [NotNamedByWorkingCode; 0] = [];

/// **Something this crate needs and another document owns.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoutedElsewhere {
    /// What is wanted.
    pub what: &'static str,
    /// The document that owns it.
    pub owner: &'static str,
    /// What this crate does meanwhile — **never a stub, never a default**.
    pub what_this_crate_does_meanwhile: &'static str,
}

/// **What this crate carries opaquely or hands on, each with its owner.**
pub const WHAT_IS_ROUTED_TO_ANOTHER_CRATE: [RoutedElsewhere; 4] = [
    RoutedElsewhere {
        what: "the completeness statement a disposition that ends a job carries — `Job files and outputs` C27's four sets, produced by that document's §10 reckoning",
        owner: "Job files and outputs",
        what_this_crate_does_meanwhile: "carries it opaquely in `crate::supervisor::CompletenessStatement` and reads nothing inside it, which is what keeps the rule that nothing about state is decided from the walk. **The absence of one is recorded explicitly and is never four empty sets**",
    },
    RoutedElsewhere {
        what: "the release predicate a hold's dependency is judged by — `Queue & History`'s Row 54, with that document's completeness reading behind it",
        owner: "Queue & History",
        what_this_crate_does_meanwhile: "supplies the **record** and answers neither row: `crate::holds::evaluate` takes the predicate as an argument, and **with no reading carried, the hold stands rather than a lookup being taken at the supervisor**. **It does not supply the moment**, and this entry said it did until 2026-09-12: U19's moment is when the dependency's recorded outcome is written and U19a's is the removal of the dependency's job record, and nothing calls `evaluate` at U19's; U19a's is driven since 2026-09-29 — see the entry for it in `THE_DECLARED_AND_NOT_DRIVEN`",
    },
    RoutedElsewhere {
        what: "the sentence that a capture record whose collection outcome is still pending must never present as *collected* — `Desktop app and delivery` §16.3's",
        owner: "Desktop app and delivery",
        what_this_crate_does_meanwhile: "supplies the outcome and does not present it. **Nothing here renders anything to a user**",
    },
    RoutedElsewhere {
        what: "two machine-side stores this crate reads and does not own — the operation record on the machine, and the event journal — which `State, identity and recovery` §1.2 inventories and §4 governs",
        owner: "State, identity and recovery",
        what_this_crate_does_meanwhile: "reads each through a trait and opens neither, and **invents no written form for either**. **The four durable things in the package area that ARE this document's own are opened here**, at `crate::acting::package`, `crate::acting::stores` and `crate::acting::registration`: the package-phase record (G1), the hold record — U2's own words are *a durable store of this document's* — the setup-in-progress state (P1), and the boot-registration record (K16), which the registration step writes in the same act that places the registration. **The hold record is the one store kind that inventory did not name**, and it is a seam that closed at the Architecture seam close rather than an extension made here",
    },
];

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs;
    use std::path::{Path, PathBuf};

    use super::*;

    const THE_PLAN: &str =
        include_str!("../../../Preparation/Architecture/Rented-server runtime.md");

    const THIS_PLANS_NAME: &str = "Rented-server runtime";

    /// Every clause a plan defines, as (top-level section number, marker), in the
    /// plan's own order.
    ///
    /// **The same reading is applied to this plan and to every other**, so a
    /// marker's document is decided one way rather than two.
    ///
    /// **The heading's number keeps its letter** — `9b` is a top-level section of
    /// `Server Setup` and reading only its digits would file that section's
    /// clauses under §9 beside a different prefix's.
    ///
    /// **A clause is not required to be followed by a full stop.** `Server Setup`
    /// B1 opens `**B1 comes first — …**` and a reading that demanded the full stop
    /// missed it, which made that plan read as 99 clauses rather than 100 and would
    /// have called a correct citation of B1 a wrong attribution. **A clause a
    /// checker cannot see is a clause nothing guards.**
    fn the_clauses_defined_by(plan: &str) -> Vec<(String, String)> {
        let mut heading = "";
        let mut out = Vec::new();
        for line in plan.lines() {
            if let Some(rest) = line.strip_prefix("### ") {
                heading = rest;
            } else if let Some(rest) = line.strip_prefix("## ") {
                heading = rest;
            }
            let number = heading
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .trim_end_matches('.');
            let top = number.split('.').next().unwrap_or_default();
            if top.is_empty() || !top.starts_with(|c: char| c.is_ascii_digit()) {
                continue;
            }
            let body = line
                .trim_start()
                .trim_start_matches("- ")
                .trim_start_matches("* ");
            let Some(rest) = body.strip_prefix("**") else {
                continue;
            };
            let marker: String = rest.chars().take_while(char::is_ascii_alphanumeric).collect();
            // The marker is followed by its own punctuation or a space; a bolded
            // sentence opening with an ordinary word is not one.
            let after = rest[marker.len()..].chars().next().unwrap_or(' ');
            if looks_like_a_marker(&marker) && matches!(after, '.' | ' ' | ',' | ':' | '*' | '—') {
                out.push((top.to_string(), marker));
            }
        }
        out
    }

    /// Whether a word has a clause marker's shape.
    ///
    /// **One or two letters, one or two digits, and at most two trailing
    /// lower-case letters** — `K1`, `G31a`, `Server Setup` M1a, `Keys` U7ab.
    ///
    /// **It is deliberately tighter than a capital followed by a digit**, which
    /// matches `Blender5` inside a derived path and would put the attribution
    /// check below on prose rather than on a marker.
    fn looks_like_a_marker(word: &str) -> bool {
        let bytes = word.as_bytes();
        if bytes.is_empty() || !bytes[0].is_ascii_uppercase() {
            return false;
        }
        let mut at = 1;
        if at < bytes.len() && bytes[at].is_ascii_alphabetic() {
            at += 1;
        }
        let digits_from = at;
        while at < bytes.len() && bytes[at].is_ascii_digit() {
            at += 1;
        }
        if !(1..=2).contains(&(at - digits_from)) {
            return false;
        }
        let mut tail = 0;
        while at < bytes.len() && bytes[at].is_ascii_lowercase() && tail < 2 {
            at += 1;
            tail += 1;
        }
        at == bytes.len()
    }

    fn the_architecture_folder() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Preparation/Architecture")
    }

    /// Every other plan, by name, with its text.
    fn the_other_plans() -> Vec<(String, String)> {
        let mut out = Vec::new();
        for entry in fs::read_dir(the_architecture_folder()).expect("the folder is readable") {
            let path = entry.expect("the entry is readable").path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .expect("the plan has a name")
                .to_string();
            if name == THIS_PLANS_NAME {
                continue;
            }
            let text = fs::read_to_string(&path).expect("the plan is readable");
            out.push((name, text));
        }
        assert!(
            out.len() > 15,
            "the other plans were not found, so nothing below would be excluded"
        );
        out
    }

    /// Every `.rs` file under a folder, **however deep**, as (path, text).
    ///
    /// **The walk recurses and the count it feeds is asserted exactly, and both
    /// halves of that are the point.** This crate grew `src/acting/` and
    /// `src/package/` during the step that built it. A walk that read only the top
    /// level of `src/`, with a floor rather than an exact count, would have let
    /// every file in a new folder escape the marker check and the clause accounting
    /// in silence, with the floor still passing. **A floor cannot fail on a file
    /// that was never read.**
    fn every_rust_file_under(folder: &Path) -> Vec<(String, String)> {
        let mut out = Vec::new();
        let Ok(entries) = fs::read_dir(folder) else {
            return out;
        };
        for entry in entries {
            let path = entry.expect("the entry is readable").path();
            if path.is_dir() {
                out.extend(every_rust_file_under(&path));
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                out.push((
                    path.display().to_string(),
                    fs::read_to_string(&path).expect("the file is readable"),
                ));
            }
        }
        out
    }

    /// Every working file of **this module**, as (name, body, header list) —
    /// this crate's own `src/`, **and the program's beside it**.
    ///
    /// **This page and `lib.rs` are not working files** — this one is the record of
    /// what is *not* here, and `lib.rs` is the crate's front matter. **A clause
    /// named only in one of those is a clause nothing keeps.**
    ///
    /// # Why it reads two folders
    ///
    /// **`Preparation/Architecture/Rented-server runtime.md` is answered by two
    /// workspace members since step 8b**, and the accounting below is over a plan
    /// rather than over a package. The program the rented machine runs moved out
    /// of this package into `App/Rented-server runtime — program/`, because
    /// Cargo's dependencies belong to a package rather than to a target and a
    /// binary here could reach only what this library reaches. **A walk that read
    /// only this folder would have let every clause that program keeps — G12's
    /// health answer, S2, S3, W1's seven kinds — go unaccounted in silence, with
    /// the forward leg still passing.** So it reads both, and asserts each half's
    /// count exactly, for [`every_rust_file_under`]'s own reason: a floor cannot
    /// fail on a file that was never read.
    fn the_working_files() -> Vec<(String, String, String)> {
        // **Thirty-eight since worker start was wired**, 2026-09-07, which added
        // `acting/starting.rs` — §14's one unit, the caller the admission point,
        // the record, the sinks and the start had never had. It was thirty-seven
        // when the event journal landed on 2026-09-06 with `acting/journal.rs`,
        // thirty-six when the program moved out earlier that day —
        // `program.rs` left for the member beside this one and is counted there
        // instead — and thirty-seven before that, when the machine's own durable
        // operation record landed and `acting/operation_records.rs` came with it.
        // Thirty-three before step 8a, which added the program, what that program
        // is (`the_program.rs`) and the boot registration's own text
        // (`registration_content.rs`).
        let here = (Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), 38);
        // **Six**: `main.rs`; `the_render_worker.rs` since step 10c piece 5's
        // second sitting, when the render worker's own run became this
        // program's; `the_setup_worker.rs` since its piece 8, which is
        // `Server Setup`'s kind and the two things a setup step needs that only
        // a program can do — a request on the machine's own line and a process
        // run; and `the_file_inspection_worker.rs` since step 10c's dependency
        // wave piece 2, the kind `Render Settings` and `Dependency Upload &
        // Remap` share; and `the_copy_on_the_machine.rs` since step 10c's
        // Files' owed acts piece 7, the copy the SSH command adapter answers
        // itself; and **`the_preview_worker.rs` since the 38's piece 3**, the
        // preview kind `Job files and outputs` declares — six; and
        // **`the_telegram_worker.rs` since 2026-10-16**, the words the one
        // role worker and the supervisor speak — seven; and
        // **`the_telegram_journal.rs` since the same day**, the third piece of
        // the Telegram worker's map, the journal read forward and a job's
        // messages sent from it — eight; and **`the_folders_size.rs` since
        // 2026-10-30**, the walk that counts a folder's size on the machine
        // (the twenty-third notes, note 2) — nine. It was one before piece 5.
        // A tenth would fail here, which is what makes this an exact count
        // rather than a floor.
        let program = (
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../Rented-server runtime — program/src"),
            9,
        );

        let mut out = Vec::new();
        for (folder, expected) in [here, program] {
            let mut found = 0;
            for (path, text) in every_rust_file_under(&folder) {
                let name = Path::new(&path)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .expect("the file has a name")
                    .to_string();
                if name == "arriving_later.rs" || name == "lib.rs" {
                    continue;
                }
                found += 1;
                let (body, header) = split_off_the_header_list(&text);
                out.push((name, body, header));
            }
            assert_eq!(
                found,
                expected,
                "the working files of {} were not all found",
                folder.display()
            );
        }
        out
    }

    /// The file's text as one run with the comment markers taken off, split into
    /// the body and **the header list of markers that belong to another
    /// document** — that list names other documents' clauses and never keeps one.
    ///
    /// **Every line is trimmed before it is joined, and joined with exactly one
    /// space.** A document's name broken across two comment lines — `` `Rented- ``
    /// on one and `` server runtime` `` on the next — otherwise reads with a
    /// double space in it, matches no plan, and turns a correct citation into an
    /// unattributed one. Both halves are joined the same way for that reason.
    fn split_off_the_header_list(text: &str) -> (String, String) {
        let mut body = String::new();
        let mut header = String::new();
        let mut in_the_header_list = false;
        for line in text.lines() {
            let line = line.trim();
            if line.contains("Markers used") {
                in_the_header_list = true;
            }
            if in_the_header_list {
                match line.strip_prefix("//!").or_else(|| line.strip_prefix("///")) {
                    // **The list ends at the first empty comment line**, so a file
                    // may follow it with a note saying which markers it
                    // deliberately does not list and why. A note inside the list
                    // would put those markers back into it.
                    Some(rest) if !rest.trim().is_empty() => {
                        header.push_str(rest.trim());
                        header.push(' ');
                        continue;
                    }
                    _ => in_the_header_list = false,
                }
            }
            let line = line
                .strip_prefix("///")
                .or_else(|| line.strip_prefix("//!"))
                .or_else(|| line.strip_prefix("//"))
                .unwrap_or(line);
            body.push_str(line.trim());
            body.push(' ');
        }
        (body, header)
    }

    /// The header list read **as segments rather than as one flat list**: each
    /// entry is a marker with the document whose name last opened a segment.
    ///
    /// **A header list is written as a plan's name in backticks followed by that
    /// plan's markers, then the next plan's name and its markers**, so a backticked
    /// plan name opens a segment and every marker after it belongs to that plan
    /// until the next name. **Reading the list flat is what let a wrongly
    /// attributed marker through**: the flat list forgave any bare marker in the
    /// body that appeared anywhere in the header, whatever document it was filed
    /// under, and nothing ever asked whether that document defines it.
    ///
    /// A marker standing before any plan name carries `None` and is a fault of its
    /// own — the list's own heading says each marker is named with the document it
    /// belongs to.
    fn the_header_list_by_document(
        header: &str,
        plan_names: &[String],
    ) -> Vec<(Option<String>, String)> {
        let mut out = Vec::new();
        let mut current: Option<String> = None;
        let bytes = header.as_bytes();
        let mut at = 0;
        while at < bytes.len() {
            if bytes[at] == b'`' {
                let Some(closes) = header[at + 1..].find('`').map(|found| at + 1 + found) else {
                    break;
                };
                let named = &header[at + 1..closes];
                if let Some(plan) = plan_names.iter().find(|plan| plan.as_str() == named) {
                    current = Some(plan.clone());
                }
                at = closes + 1;
                continue;
            }
            if bytes[at].is_ascii_alphanumeric() {
                let mut end = at;
                while end < bytes.len() && bytes[end].is_ascii_alphanumeric() {
                    end += 1;
                }
                let word = &header[at..end];
                if looks_like_a_marker(word) {
                    out.push((current.clone(), word.to_string()));
                }
                at = end;
                continue;
            }
            at += 1;
        }
        out
    }

    /// Every marker-shaped word in `text`, with the document written beside it
    /// where one is, as (marker, that document).
    fn the_markers_in(text: &str, plan_names: &[String]) -> Vec<(String, Option<String>)> {
        let bytes = text.as_bytes();
        let mut out = Vec::new();
        let mut at = 0;
        while at < bytes.len() {
            let starts_a_word = bytes[at].is_ascii_uppercase()
                && (at == 0 || !(bytes[at - 1].is_ascii_alphanumeric() || bytes[at - 1] == b'_'));
            if !starts_a_word {
                at += 1;
                continue;
            }
            let mut end = at;
            while end < bytes.len() && bytes[end].is_ascii_alphanumeric() {
                end += 1;
            }
            let word = &text[at..end];
            let followed_by_underscore = end < bytes.len() && bytes[end] == b'_';
            if looks_like_a_marker(word) && !followed_by_underscore {
                out.push((word.to_string(), the_document_standing_beside(text, at, plan_names)));
            }
            at = end.max(at + 1);
        }
        out
    }

    /// The document written immediately before the marker at `at`, in backticks,
    /// where that is one of the plans.
    fn the_document_standing_beside(
        text: &str,
        at: usize,
        plan_names: &[String],
    ) -> Option<String> {
        let before = text[..at]
            .trim_end()
            .trim_end_matches(|c: char| "*([{\"'".contains(c))
            .trim_end();
        let inner = before.strip_suffix('`')?;
        let opens = inner.rfind('`')?;
        let named = &inner[opens + 1..];
        plan_names
            .iter()
            .find(|plan| plan.as_str() == named)
            .cloned()
    }

    #[test]
    fn the_plans_clause_count_has_not_moved_under_this_page() {
        // 236 when this was written; **239 from 2026-09-06**, when the plan
        // gained K16a and K16b in §1 and G14a in §2 — the three clauses a
        // builder's reading of the boot-registration record raised and a plan
        // repair wrote. A change to the plan that adds a clause fails here,
        // which is what sends whoever made it to this page.
        //
        // **The number is derived rather than adjusted until it agrees**: the
        // reading below is what produced it, and the three markers it gained
        // are the same three the forward-leg test named as unaccounted on the
        // same run.
        //
        // **Counted unique.** The reading admits a bold run that opens with a
        // marker and no full stop, which is what makes `Server Setup` B1 visible;
        // the price is that a marker bolded again as a reference in §16 or §17 is
        // read a second time. Nine markers of this plan are bolded more than once
        // and every second occurrence is such a reference.
        let unique: HashSet<String> = the_clauses_defined_by(THE_PLAN)
            .into_iter()
            .map(|(_, marker)| marker)
            .collect();
        // **240 from 2026-09-21, step 10c: §1.3 gained K10c** — whatever the
        // provider's container hook already holds is kept — which was built
        // the same day and said by no clause. It is named at
        // `registration_content`'s marker lines.
        //
        // **241 from 2026-09-28, step 10c's transfer wave piece 10: §2.4
        // gained G15a** — the machine's stores upgraded to a new program's
        // shapes only after *verified* is written — which the servers-follow
        // plan pass wrote the same day. It is named at `package/commit.rs`
        // (the rule) and at the machine program's supervisor start, which
        // performs it. G25's paragraph was struck and replaced the same day
        // and added no marker.
        //
        // **242 from 2026-09-30, step 10c's eleventh-pass fix piece 8b: §8.3
        // gained U12a** — a *no* removes its subject in the same act that
        // writes its release — on the build runner's ruling of that day. It is
        // named at `holds.rs`'s `the_answer_removes_its_subject` (the rule) and
        // performed by the machine program's answer to a hold.
        // 241 from 2026-10-27: wave 5 removes the withdrawn health
        // entitlement clause. Re-derived from the plan with this reader and
        // deduplicated as above: 242 minus one, with no new definition.
        assert_eq!(unique.len(), 241);
    }

    /// **The gate's forward leg, run over this one document at the desk: every
    /// clause the plan states has code that keeps it.**
    ///
    /// **The naming site is found, not inferred from a section number** — inferring
    /// it would pass for every clause of a built section whether or not any code
    /// existed, and this test would survive a working file being deleted whole.
    ///
    /// **A clause on [`NOT_BUILT_HERE`] is accounted for by that entry instead**,
    /// which is the second limb and is new on 2026-09-06. **It is not an escape**:
    /// the entry is read against the plan by the test below, an entry with no
    /// *what removes it* fails there, an entry working code also names fails
    /// there, and a clause that is neither built nor recorded still fails here.
    #[test]
    fn every_clause_the_plan_defines_is_named_by_working_code() {
        let plan_names: Vec<String> = the_other_plans()
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        let named = the_clauses_working_code_names(&plan_names);
        let recorded: Vec<&str> = NOT_BUILT_HERE
            .iter()
            .flat_map(|entry| entry.clauses.iter().copied())
            .collect();
        let mut unaccounted = Vec::new();
        for (section, marker) in the_clauses_defined_by(THE_PLAN) {
            if named.contains(&marker) || recorded.contains(&marker.as_str()) {
                continue;
            }
            unaccounted.push(format!("§{section} {marker}"));
        }
        assert!(
            unaccounted.is_empty(),
            "these clauses are named by no working file of this crate and are not \
             recorded as deliberately not built: {unaccounted:?}"
        );
    }

    /// Every marker of this plan that a working file of this crate names bare.
    fn the_clauses_working_code_names(plan_names: &[String]) -> HashSet<String> {
        let mut named: HashSet<String> = HashSet::new();
        for (_, body, _) in the_working_files() {
            for (marker, beside) in the_markers_in(&body, plan_names) {
                if beside.is_none() {
                    named.insert(marker);
                }
            }
        }
        named
    }

    /// **Every recorded absence is a clause of the section it is filed under,
    /// says all four of its sentences, and is named by no working file.**
    ///
    /// **The markers on this page are written as text rather than in comments**,
    /// so the sweep that reads working files does not reach them. This is what
    /// reaches them: the plan is asked, clause by clause, and the section an entry
    /// files a clause under has to be the section the plan defines it in. **An
    /// absence with no way out of it is a gap wearing a record's clothes**, so an
    /// entry that does not say what the clause is, why nothing here names it, and
    /// what removes it fails. **And an absence working code also names fails**,
    /// because the record and the code would then say different things with
    /// nothing to catch it.
    #[test]
    fn every_recorded_absence_is_a_clause_of_the_section_it_is_filed_under() {
        let plan_names: Vec<String> = the_other_plans()
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        let defined = the_clauses_defined_by(THE_PLAN);
        let named = the_clauses_working_code_names(&plan_names);
        let mut checked = 0;
        for entry in NOT_BUILT_HERE {
            assert!(!entry.clauses.is_empty());
            assert!(!entry.what.is_empty());
            assert!(!entry.why.is_empty());
            assert!(!entry.what_removes_it.is_empty());
            let filed_under = entry.section.trim_start_matches('§');
            for clause in entry.clauses {
                let sits_in = defined
                    .iter()
                    .find(|(_, marker)| marker == clause)
                    .map(|(section, _)| section.as_str())
                    .unwrap_or_else(|| panic!("the plan defines no {clause}"));
                assert_eq!(
                    sits_in, filed_under,
                    "{clause} is filed under {} and the plan defines it in §{sits_in}",
                    entry.section
                );
                assert!(
                    !named.contains(*clause),
                    "{clause} is recorded as not built and a working file names it"
                );
                checked += 1;
            }
        }
        // **Nought since 2026-09-07**, when G14a's entry retired with the
        // rollback performer that removed it. It was one for a day. **The
        // number is written out rather than derived from the list's own
        // length**, which would assert nothing.
        assert_eq!(checked, 0);
    }

    /// **The rule this crate's front matter states, made a test, because about 150
    /// bare markers were swept out of nine crates on 2026-09-03 having survived
    /// five gate runs.**
    ///
    /// **This is the settled shape, and it is the one the crates still to be
    /// written copy.** Settled 2026-09-04, out of the step 6 gate, which found two
    /// blind spots in the version that stood here. The five legs are below; the
    /// file walk that feeds them recurses and asserts an exact count, which is the
    /// second blind spot and is at [`the_working_files`].
    ///
    /// Five things fail here.
    ///
    /// - **A bare marker belonging to another document.** `M1` is defined in eight
    ///   documents, so a bare one does not fail to resolve: it resolves to a real
    ///   and wrong clause. A bare marker is allowed only where this plan defines it,
    ///   or where the file's own header list names it.
    /// - **A marker written with a document that does not define it.** That is the
    ///   wrongly-attributed case, and it reads exactly like a right one.
    /// - **A header list naming a marker this plan also defines.** `Servers`' own
    ///   rule: where a marker is a clause of this crate's plan *and* of the document
    ///   meant, **the header list is not enough** and the document is written out at
    ///   the site — because a bare use could then mean either.
    /// - **A header list naming a marker the document it is filed under does not
    ///   define.** **The first blind spot the gate found: the header list was an
    ///   unchecked whitelist.** Every marker in it was flattened into one list and
    ///   any bare marker in the body that appeared there was forgiven, whatever
    ///   document it was filed under, and nothing ever asked whether that document
    ///   defines it. `Server Setup` filed two markers under this crate's own plan
    ///   whose prefixes that plan does not define at all, and both survived a gate
    ///   run: the body use read as bare, and bare was forgiven by the list.
    /// - **A header entry filed under no document at all**, which the list's own
    ///   heading forbids: each marker is named with the document it belongs to.
    ///
    /// **What it cannot see, said rather than glossed. Three traps, and each has
    /// already caught somebody.**
    ///
    /// - **The scan is line-based and compiles nothing.** A marker inside a name
    ///   with an underscore in it is skipped, and so is one reached through a link
    ///   rather than written out.
    /// - **A capital followed by a digit is not a marker's shape.** That test
    ///   matches `Blender5` inside a derived path, which puts the attribution leg on
    ///   prose. [`looks_like_a_marker`] is the settled shape and is tighter.
    /// - **A plan's name broken across two comment lines reads with a double
    ///   space** unless every line is trimmed before it is joined — and a name with
    ///   a double space in it matches no plan, so a correct citation is called
    ///   unattributed. [`split_off_the_header_list`] trims both halves for that
    ///   reason.
    ///
    /// And it decides what a document defines by the same reading everywhere, so a
    /// clause some plan states in a shape [`the_clauses_defined_by`] does not catch
    /// is invisible to it in both directions.
    #[test]
    fn the_markers_are_written_with_their_documents() {
        let other_plans = the_other_plans();
        let plan_names: Vec<String> = other_plans.iter().map(|(name, _)| name.clone()).collect();
        let defined_by: Vec<(String, HashSet<String>)> = other_plans
            .iter()
            .map(|(name, text)| {
                (
                    name.clone(),
                    the_clauses_defined_by(text)
                        .into_iter()
                        .map(|(_, marker)| marker)
                        .collect(),
                )
            })
            .collect();
        let ours: HashSet<String> = the_clauses_defined_by(THE_PLAN)
            .into_iter()
            .map(|(_, marker)| marker)
            .collect();
        let anybody_elses: HashSet<String> = defined_by
            .iter()
            .flat_map(|(_, markers)| markers.iter().cloned())
            .collect();

        let mut faults = Vec::new();
        for (name, body, header) in the_working_files() {
            let filed = the_header_list_by_document(&header, &plan_names);
            let in_the_header: HashSet<String> =
                filed.iter().map(|(_, marker)| marker.clone()).collect();
            for (document, marker) in &filed {
                match document {
                    None => faults.push(format!(
                        "{name}: the header list names {marker} under no document — the list \
                         names each marker with the document it belongs to"
                    )),
                    Some(document) => {
                        let defines = defined_by
                            .iter()
                            .find(|(plan, _)| plan == document)
                            .map(|(_, markers)| markers.contains(marker))
                            .unwrap_or(false);
                        if !defines {
                            faults.push(format!(
                                "{name}: the header list files {marker} under `{document}`, \
                                 which does not define it"
                            ));
                        }
                    }
                }
            }
            for ambiguous in in_the_header.intersection(&ours) {
                faults.push(format!(
                    "{name}: the header list names {ambiguous}, which this plan also defines — \
                     write the document out at the site instead"
                ));
            }
            for (marker, beside) in the_markers_in(&body, &plan_names) {
                match beside {
                    Some(document) => {
                        let defines = defined_by
                            .iter()
                            .find(|(plan, _)| *plan == document)
                            .map(|(_, markers)| markers.contains(&marker))
                            .unwrap_or(false);
                        if !defines {
                            faults.push(format!(
                                "{name}: {marker} is written with `{document}`, which does not \
                                 define it"
                            ));
                        }
                    }
                    None => {
                        if !ours.contains(&marker)
                            && !in_the_header.contains(&marker)
                            && anybody_elses.contains(&marker)
                        {
                            faults.push(format!(
                                "{name}: {marker} stands bare and belongs to another document"
                            ));
                        }
                    }
                }
            }
        }
        assert!(faults.is_empty(), "{faults:#?}");
    }

    /// **A document's name governs the marker immediately after it and no
    /// further**, so every later marker in the same run is read as that
    /// document's and resolves to the wrong clause.
    ///
    /// Returns each run of markers that follows a backticked plan name, as
    /// (file, plan, markers in the run after the first).
    ///
    /// **A run is markers joined by nothing but separators** — a space, a
    /// comma, `and`, `or`, a dash, `to`. Anything else ends it, so a marker
    /// in the next sentence is not swept up and a bolded run that closes
    /// after the first marker is not read past.
    fn the_runs_after_a_document_name(
        text: &str,
        plan_names: &[String],
    ) -> Vec<(String, Vec<String>)> {
        let bytes = text.as_bytes();
        let mut out = Vec::new();
        let mut at = 0;
        while at < bytes.len() {
            if bytes[at] != b'`' {
                at += 1;
                continue;
            }
            let Some(closes) = text[at + 1..].find('`').map(|found| at + 1 + found) else {
                break;
            };
            let named = text[at + 1..closes].to_string();
            at = closes + 1;
            if !plan_names.contains(&named) {
                continue;
            }
            // The run: marker, separator, marker, separator, … Anything that is
            // neither ends it.
            let mut run: Vec<String> = Vec::new();
            let mut rest = &text[at..];
            loop {
                let rest_trimmed = rest.trim_start_matches(' ');
                let word: String = rest_trimmed
                    .chars()
                    .take_while(char::is_ascii_alphanumeric)
                    .collect();
                if word.is_empty() || !looks_like_a_marker(&word) {
                    break;
                }
                let mut after = &rest_trimmed[word.len()..];
                // A marker inside a name with an underscore in it is not a
                // citation, and the outer guard already skips one.
                if after.starts_with('_') {
                    break;
                }
                run.push(word);
                // A marker's own possessive is part of the citation.
                after = after.strip_prefix("'s").unwrap_or(after);
                // Exactly the separators a list of citations is written with.
                // Anything else — a full stop, a bold run closing, a word —
                // ends the run.
                let stepped = after
                    .strip_prefix(',')
                    .unwrap_or(after)
                    .trim_start_matches(' ');
                let stepped = ["and ", "or ", "to ", "– ", "- "]
                    .iter()
                    .find_map(|joiner| stepped.strip_prefix(joiner))
                    .unwrap_or(stepped);
                let separated = stepped.len() < after.len();
                if !separated {
                    break;
                }
                rest = stepped;
            }
            if run.len() > 1 {
                out.push((named, run[1..].to_vec()));
            }
        }
        out
    }

    /// **The neighbour rule, held rather than remembered: a document's name
    /// governs the marker immediately after it and lends itself to no
    /// other.**
    ///
    /// This is the class the guard above cannot see, named in its own list of
    /// limits: *"it cannot tell a bare marker that means this plan's clause
    /// from one that means a colliding clause of a document named earlier in
    /// the same sentence"*. On 2026-09-04 two citations in one crate each
    /// paired a plan's name with two markers, and in both pairs the second
    /// was one that plan does not define. The guard above forgave both: it
    /// read each second marker as bare, and a bare marker this plan defines
    /// is what it allows. The eye, meanwhile, takes the name as governing
    /// both. **No example is written out here on purpose** — a real plan's
    /// name beside a marker it does not define is the very fault this leg
    /// reports, and every leg reads this file too.
    ///
    /// **A marker in such a run that the named document does not define is a
    /// fault whatever it means**, and the fix is always the same: write the
    /// document out at the site, this plan's own name included.
    ///
    /// **What it cannot see.** A run that the named document *does* define
    /// every marker of is passed, which is right; and a marker separated from
    /// the run by anything but a plain separator is not in the run at all, so
    /// a citation spread across a clause is not reached.
    #[test]
    fn a_document_name_governs_only_the_run_it_opens_and_lends_itself_to_none() {
        let mut defined_by: Vec<(String, HashSet<String>)> = the_other_plans()
            .iter()
            .map(|(name, text)| {
                (
                    name.clone(),
                    the_clauses_defined_by(text)
                        .into_iter()
                        .map(|(_, marker)| marker)
                        .collect(),
                )
            })
            .collect();
        defined_by.push((
            THIS_PLANS_NAME.to_string(),
            the_clauses_defined_by(THE_PLAN)
                .into_iter()
                .map(|(_, marker)| marker)
                .collect(),
        ));
        let plan_names: Vec<String> = defined_by.iter().map(|(name, _)| name.clone()).collect();

        // **Every file, not only the working ones.** A run written in this
        // page or in the front matter misleads a reader exactly as readily.
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut faults = Vec::new();
        for (path, text) in every_rust_file_under(&src) {
            let name = Path::new(&path)
                .file_name()
                .and_then(|s| s.to_str())
                .expect("the file has a name")
                .to_string();
            let (body, header) = split_off_the_header_list(&text);
            for part in [&body, &header] {
                for (document, run) in the_runs_after_a_document_name(part, &plan_names) {
                    let defines = defined_by
                        .iter()
                        .find(|(plan, _)| *plan == document)
                        .map(|(_, markers)| markers.clone())
                        .unwrap_or_default();
                    for marker in run {
                        if !defines.contains(&marker) {
                            faults.push(format!(
                                "{name}: {marker} stands in a run opened by `{document}`, \
                                 which does not define it — the name governs only the marker \
                                 after it, so write this one's document out"
                            ));
                        }
                    }
                }
            }
        }
        assert!(faults.is_empty(), "{faults:#?}");
    }

    /// The members of the workspace, **read from `App/Cargo.toml`** rather than
    /// kept here, so a member added later is read without anybody remembering.
    ///
    /// **Every comment is taken off before the list is split, and that is a
    /// repair rather than a tidy.** The member list carries long `#` notes saying
    /// why a member exists, and those notes have commas in them. Splitting the
    /// raw text on a comma therefore glued the tail of a note onto the member
    /// that followed it, and an entry beginning `#` rather than `"` was dropped
    /// — silently, and with the count still above the floor below. **Two members
    /// were being skipped this way**, so the sweep that claims to read every
    /// member was not reading them. Found on 2026-09-06 when a third note was
    /// added.
    fn the_members_of_the_workspace() -> Vec<String> {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../Cargo.toml");
        let text = fs::read_to_string(&manifest).expect("the workspace manifest is readable");
        let after = text
            .split_once("members = [")
            .expect("the workspace manifest names its members")
            .1;
        let list = after.split_once(']').expect("the member list is closed").0;
        let list: String = list
            .lines()
            .map(|line| line.split('#').next().unwrap_or(""))
            .collect::<Vec<_>>()
            .join("\n");
        let members: Vec<String> = list
            .split(',')
            .filter_map(|entry| {
                let entry = entry.trim();
                entry
                    .strip_prefix('"')
                    .and_then(|rest| rest.strip_suffix('"'))
                    .map(str::to_string)
            })
            .collect();
        assert!(
            members.len() > 5 && members.iter().any(|member| member == "Rented-server runtime"),
            "the workspace members were not read, so nothing below would be searched"
        );
        members
    }

    /// Every trait the application implements outside test code, with the file it
    /// is implemented in — **read across every workspace member**, which is the
    /// reach the record above claims.
    ///
    /// **`#[cfg(test)]` is taken as beginning test code and running to the end of
    /// the file**, which is where every file in this workspace puts it.
    fn what_the_application_implements() -> Vec<(String, String)> {
        let app = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let mut out = Vec::new();
        for member in the_members_of_the_workspace() {
            let src = app.join(&member).join("src");
            let files = every_rust_file_under(&src);
            assert!(
                !files.is_empty(),
                "{member} is a workspace member and none of its source was read"
            );
            for (where_it_is, text) in files {
                let mut in_tests = false;
                for line in text.lines() {
                    let line = line.trim_start();
                    if line.starts_with("#[cfg(test)]") {
                        in_tests = true;
                    }
                    if in_tests || !line.starts_with("impl") {
                        continue;
                    }
                    let Some(at) = line.find(" for ") else {
                        continue;
                    };
                    if let Some(name) = line[..at].split_whitespace().last() {
                        out.push((
                            name.split('<')
                                .next()
                                .unwrap_or_default()
                                .rsplit("::")
                                .next()
                                .unwrap_or_default()
                                .to_string(),
                            where_it_is.clone(),
                        ));
                    }
                }
            }
        }
        out
    }

    /// **The record of real ends is exactly what the application does and does
    /// not implement.**
    ///
    /// Three things fail here.
    ///
    /// - **A trait this crate declares that is not on the record**, or a record
    ///   entry naming a trait this crate does not declare.
    /// - **An entry saying it reaches the machine that nothing implements** —
    ///   the record claiming an end that is not there.
    /// - **An entry saying it reaches no machine that something implements** —
    ///   which is what fired when the acting half landed, and is what brought
    ///   whoever wrote it back to this page.
    ///
    /// **The record is a claim about the whole application, so the test reads
    /// the whole application**, every workspace member's source, outside test
    /// code.
    #[test]
    fn the_record_of_real_ends_matches_the_application() {
        let recorded: HashSet<&str> = THE_ACTING_HALF
            .iter()
            .map(|entry| {
                entry
                    .trait_path
                    .rsplit("::")
                    .next()
                    .expect("the trait has a name")
            })
            .collect();

        let mut declared: Vec<String> = Vec::new();
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        for (_, text) in every_rust_file_under(&src) {
            for line in text.lines() {
                if let Some(rest) = line.trim_start().strip_prefix("pub trait ") {
                    declared.push(
                        rest.split_whitespace()
                            .next()
                            .unwrap_or_default()
                            .split(':')
                            .next()
                            .unwrap_or_default()
                            .to_string(),
                    );
                }
            }
        }
        assert_eq!(
            declared.len(),
            THE_ACTING_HALF.len(),
            "this crate declares {} traits and the record holds {}",
            declared.len(),
            THE_ACTING_HALF.len()
        );

        let implemented = what_the_application_implements();
        for name in &declared {
            assert!(
                recorded.contains(name.as_str()),
                "{name} is declared by this crate and is not on the record"
            );
        }
        for name in &recorded {
            assert!(
                declared.iter().any(|held| held == name),
                "{name} is on the record and this crate does not declare it"
            );
        }

        for entry in THE_ACTING_HALF.iter() {
            let name = entry
                .trait_path
                .rsplit("::")
                .next()
                .expect("the trait has a name");
            let where_it_is: Vec<String> = implemented
                .iter()
                .filter(|(held, _)| held == name)
                .map(|(_, at)| at.clone())
                .collect();
            match entry.where_it_reaches_the_machine {
                Some(module) => {
                    assert!(
                        !where_it_is.is_empty(),
                        "{name} is recorded as reaching the machine at {module} and nothing \
                         in the application implements it"
                    );
                    let file = module.rsplit("::").next().expect("the module has a name");
                    assert!(
                        where_it_is
                            .iter()
                            .any(|at| at.contains(&format!("{file}.rs"))),
                        "{name} is recorded at {module} and is implemented at {where_it_is:?}"
                    );
                }
                None => assert!(
                    where_it_is.is_empty(),
                    "{name} is recorded as reaching no machine and something implements it: \
                     {where_it_is:?}"
                ),
            }
        }
    }

    /// **Every clause the record names is a clause this plan actually defines**,
    /// so an entry cannot point at a marker that has moved or never existed.
    ///
    /// **And an entry that reaches no machine says what is missing and what
    /// removes it**, while one that reaches the machine whole says neither —
    /// which is what keeps a finished entry from carrying a debt nobody owes.
    #[test]
    fn the_record_names_only_clauses_this_plan_defines() {
        let defined: HashSet<String> = the_clauses_defined_by(THE_PLAN)
            .into_iter()
            .map(|(_, marker)| marker)
            .collect();
        let mut reaching = 0;
        for entry in THE_ACTING_HALF.iter() {
            assert!(!entry.the_clauses_it_serves.is_empty());
            match entry.where_it_reaches_the_machine {
                Some(_) => {
                    reaching += 1;
                    assert_eq!(
                        entry.what_is_still_missing.is_empty(),
                        entry.what_removes_it.is_empty(),
                        "{}: a part that is missing needs what removes it, and a whole one \
                         needs neither",
                        entry.trait_path
                    );
                }
                None => {
                    assert!(!entry.what_is_still_missing.is_empty());
                    assert!(!entry.what_removes_it.is_empty());
                }
            }
            for clause in entry.the_clauses_it_serves {
                assert!(
                    defined.contains(*clause),
                    "{clause} is named by {} and this plan does not define it",
                    entry.trait_path
                );
            }
        }
        // **The count is derived rather than written**, so an entry added to
        // the record cannot leave a number here saying otherwise. What is
        // asserted is the thing itself: every entry reaches the machine.
        assert_eq!(
            reaching,
            THE_ACTING_HALF.len(),
            "every trait on the record reaches the machine"
        );
    }

    /// **Every declared-and-not-driven entry names a real thing, real clauses
    /// and the step that drives it** — and the thing it names is genuinely in
    /// this crate, so an entry cannot go on saying something is undriven after
    /// the driver has been written under a different name.
    #[test]
    fn what_is_declared_and_not_driven_names_a_real_thing_and_the_step_that_drives_it() {
        let defined: HashSet<String> = the_clauses_defined_by(THE_PLAN)
            .into_iter()
            .map(|(_, marker)| marker)
            .collect();
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let whole: String = every_rust_file_under(&src)
            .into_iter()
            .map(|(_, text)| text)
            .collect();
        for entry in THE_DECLARED_AND_NOT_DRIVEN.iter() {
            assert!(!entry.what_nothing_performs.is_empty());
            assert!(!entry.which_step_drives_it.is_empty());
            assert!(!entry.the_clauses_it_holds.is_empty());
            for clause in entry.the_clauses_it_holds {
                assert!(
                    defined.contains(*clause),
                    "{clause} is named by {} and this plan does not define it",
                    entry.what_is_declared
                );
            }
            let named = entry
                .what_is_declared
                .rsplit("::")
                .next()
                .expect("the thing has a name");
            assert!(
                whole.contains(named),
                "{} names {named}, which is nowhere in this crate",
                entry.what_is_declared
            );
        }
    }

    /// **The page's own opening must not say that naming is driving.**
    ///
    /// The sentence that used to stand here said every clause is named by
    /// working code **and therefore** that no list of unbuilt clauses was
    /// needed — true about naming and read as true about building. **A page
    /// whose whole job is to say what is missing must not say nothing is.**
    #[test]
    fn the_page_says_that_naming_is_not_driving() {
        let here =
            fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/arriving_later.rs"))
                .expect("this page is readable");
        let opening: String = here
            .lines()
            .take_while(|line| line.starts_with("//!"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            opening.contains("Naming is not driving"),
            "the page's own opening must say plainly that a named clause is not a driven one"
        );
        assert!(
            opening.contains("THE_DECLARED_AND_NOT_DRIVEN"),
            "and it must send the reader to what is declared and not driven"
        );
    }

    #[test]
    fn every_routed_item_names_an_owner_that_is_not_this_document() {
        // **No two routed items are the same thing.** A length over an array
        // whose type fixes it cannot be false; a duplicated entry can.
        let named: HashSet<&str> = WHAT_IS_ROUTED_TO_ANOTHER_CRATE
            .iter()
            .map(|routed| routed.what)
            .collect();
        assert_eq!(named.len(), WHAT_IS_ROUTED_TO_ANOTHER_CRATE.len());
        let plans: HashSet<String> = the_other_plans()
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        for routed in WHAT_IS_ROUTED_TO_ANOTHER_CRATE {
            assert!(
                plans.contains(routed.owner),
                "{} is not one of the plans",
                routed.owner
            );
            assert!(!routed.what_this_crate_does_meanwhile.is_empty());
        }
    }

    #[test]
    fn the_known_hole_is_recorded_with_its_symptom_and_no_rescue() {
        assert!(THE_STORED_FIELD_HOLE.contains("nothing ever runs them"));
        assert!(THE_STORED_FIELD_HOLE.contains("repairs after every"));
    }
}
