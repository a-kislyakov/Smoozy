//! §7 — the common arbitration mechanism.
//!
//! **Each owning feature document declares an operation's resource class,
//! priority, cancellability, atomic-or-partial boundary and machine-level
//! exclusion of its own kind; this document owns the common arbitration
//! mechanism.**
//!
//! **A4. One admission point per machine, held by the supervisor, and every
//! worker start passes through it.** There is no second scheduler and no per-kind
//! queue: **a single point is what makes exclusion a property rather than a
//! convention.**
//!
//! **A13. So the mechanism is built to be tightened rather than to be right.**
//! A2's declaration lets a derivative run beside a render today and lets it be
//! excluded tomorrow by one change to that declaration, with nothing in this
//! section moving. **What would be unfixable is a design in which the render and
//! the derivative are not both arbitrated at one point**, and that is what A4
//! exists for.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Server protocol and transfers` — its §6.
//! `State, identity and recovery` §9 and §9.2. `Job files and outputs` B24,
//! B24a. `Server Setup` Y7, Y9 and its §11. `Telegram` — its §19.
//! `Render Settings` §10. `Dependency Upload & Remap` O4. `Queue & History` —
//! the ordering of what waits.
//!
//! **A marker this plan also defines is never listed here and is written out
//! with its document at the site**, because a header list cannot tell the two
//! apart and a bare one there would resolve to a real and wrong clause. Eight of
//! this file's citations are of that kind.

use smoozy_state::identities::Identity;

use crate::workers::WorkerKind;

/// **A1 item 1's contended resources, and the list is closed at three.**
///
/// **The disk and the connection are not arbitrated here**: transfer concurrency
/// and priority are `Server protocol and transfers` §6's, and **nothing here
/// duplicates that admission point.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    Cards,
    Processors,
    MachineMemory,
}

impl Resource {
    pub const ALL: [Resource; 3] = [Resource::Cards, Resource::Processors, Resource::MachineMemory];
}

/// Whether a class takes a resource exclusively or shares it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum How {
    Exclusively,
    Shared,
}

/// Whether a kind may be cancelled, and when.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cancellability {
    /// A1 item 3's *cancellable, at any moment*.
    AtAnyMoment,
    /// A1 item 3's *cancellable, only at a recorded safe boundary* (X11).
    OnlyAtARecordedSafeBoundary,
    NotCancellable,
}

/// **A1. Five things, per operation kind, in the owning document, and none of
/// them is decided at run time.**
///
/// **A class decided at run time is a class two callers decide differently**,
/// which is the same discipline `Server protocol and transfers` T8 requires of a
/// transfer's priority and `State, identity and recovery` §9.2 of a partition
/// unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration {
    /// **1. Resource class** — which of the machine's contended resources the
    /// operation takes, and whether it takes each exclusively or shared.
    pub resources: Vec<(Resource, How)>,
    /// **2. Priority**, as a rank against the other kinds. Lower is preferred.
    pub priority: u8,
    /// **3. Cancellable or not**, and where cancellable, whether at any moment or
    /// only at a recorded safe boundary.
    pub cancellability: Cancellability,
    /// **4. Atomic or partial**, against `State, identity and recovery` §9. Named
    /// here as the owning document's word; §14 carries this document's own.
    pub atomic_or_partial: &'static str,
    /// **5. Machine-level exclusion of its own kind** — *whether the machine
    /// admits more than one operation of this kind at a time*.
    ///
    /// **Why a fifth thing rather than a fourth resource**: `Server Setup` Y9
    /// requires one setup run per machine, and the only way to say that in the
    /// four above would be to declare the processors or the memory exclusive —
    /// **which would exclude renders as well, which is not what Y9 says.**
    ///
    /// **It is declared, like the four above, and it is not a resource**: the
    /// contended list stays closed at the cards, the processors and the machine's
    /// memory.
    pub excludes_its_own_kind: bool,
    /// **A5b's exception**: whether the kind's own document states that a further
    /// operation of that kind **queues**.
    ///
    /// **`Telegram` M4 states it** — *"At most one media production runs per
    /// machine at a time; further completed stretches queue at the admission
    /// point"* — and `Server Setup` Y9 states the opposite for the setup kind.
    pub its_document_states_that_a_further_operation_queues: bool,
}

impl Declaration {
    /// **A3. A kind whose owning document has not declared is arbitrated under a
    /// default declaration rather than shut out of the mechanism.**
    ///
    /// **The default is the most conservative one that still lets the kind run**:
    /// it takes no resource exclusively — the processors and the machine's memory
    /// shared, no card — it is ranked last, and it is treated as not cancellable.
    /// So it competes at the admission point under A5 and A8 exactly as a
    /// declared class does, and **it is excluded only where it names a resource
    /// something else is holding exclusively.**
    ///
    /// **That is what makes §16 item 6's stated cost true**: a missing
    /// declaration costs throughput and never a render. **Barring an undeclared
    /// kind from admission whenever anything exclusive is running would not be
    /// conservatism but starvation** — on a machine with a queue of renders,
    /// nothing-exclusive-running is a gap of seconds, and A8 forbids a class that
    /// never runs there.
    pub fn the_default() -> Declaration {
        Declaration {
            resources: vec![
                (Resource::Processors, How::Shared),
                (Resource::MachineMemory, How::Shared),
            ],
            priority: u8::MAX,
            cancellability: Cancellability::NotCancellable,
            atomic_or_partial: "not declared by its owning document",
            excludes_its_own_kind: false,
            its_document_states_that_a_further_operation_queues: false,
        }
    }
}

/// **A2. Two declarations are already given and are not chosen here.**
///
/// **A derivative worker is low priority, cancellable, never blocking a render**
/// (`Job files and outputs` B24). **And a rendering job gets the whole machine's
/// cards**, which is `Queue & History`'s allotment promise and **makes the
/// render's card class exclusive.**
pub fn the_two_declarations_already_given(kind: WorkerKind) -> Option<Declaration> {
    match kind {
        WorkerKind::Preview => Some(Declaration {
            resources: vec![
                (Resource::Processors, How::Shared),
                (Resource::MachineMemory, How::Shared),
            ],
            priority: 200,
            cancellability: Cancellability::AtAnyMoment,
            atomic_or_partial: "`Job files and outputs` B24a's",
            excludes_its_own_kind: false,
            its_document_states_that_a_further_operation_queues: false,
        }),
        WorkerKind::Render => Some(Declaration {
            resources: vec![
                (Resource::Cards, How::Exclusively),
                (Resource::Processors, How::Shared),
                (Resource::MachineMemory, How::Shared),
            ],
            priority: 0,
            cancellability: Cancellability::AtAnyMoment,
            atomic_or_partial: "`Rendering`'s",
            excludes_its_own_kind: false,
            its_document_states_that_a_further_operation_queues: false,
        }),
        _ => None,
    }
}

/// **`Server protocol and transfers` T11: the transfer-support kind's five
/// declarations, made by the document W1 names as their owner.**
///
/// **This is A1's five things for the transfer-support kind, and it is written
/// here for the same reason [`the_two_declarations_already_given`] writes
/// `Job files and outputs` B24's and `Queue & History`'s here**: A1 puts the
/// declaring in the owning document and A4 puts the arbitrating at this one
/// point, so the values are that document's and the place they are read is this
/// one.
///
/// **Why it exists rather than the kind falling to [`Declaration::the_default`],
/// and this is a wrong answer being removed rather than a gap being filled.**
/// A3's default treats an undeclared kind as **not cancellable**, and
/// `Server protocol and transfers` T11 says in as many words that the value is
/// false of this kind: `Server protocol and transfers` C22 gives the
/// transfer-support worker a termination contract — X14's staging and cleanup
/// rather than a signal — so W2's *cancellation is refused* never applied to it,
/// and a machine arbitrating it under the default was describing it wrongly with
/// nothing saying so. **Two of A3's three values were what this document would
/// have declared anyway; the third was false.**
///
/// **The five, in `Server protocol and transfers` T11's own order.**
///
/// 1. **Resource class** — the processors and the machine's memory **shared**,
///    **no card**. The worker runs the byte loop, the digest and the single
///    publication, and takes nothing else A1 arbitrates: the disk and the
///    connection are not on A1's contended list.
/// 2. **Priority** — **below the render kind; above the preview and media
///    kinds.** [`THE_TRANSFER_SUPPORT_KINDS_RANK`] carries the number and says
///    why the number itself decides nothing.
/// 3. **Cancellable at any moment**, the termination being
///    `Server protocol and transfers` §7.8's staging and cleanup rather than a
///    signal. That is `Server protocol and transfers` C22. X14 says the same
///    from this document's own side. **A cancellation A6 itself
///    performs does not arise**: A6 cancels only an operation holding what an
///    exclusive operation needs, and this class holds nothing exclusively.
/// 4. **Atomic**, the unit being one transfer.
/// 5. **Exclusion: nothing declared, which A1 reads as unlimited.** What bounds
///    machine-side transfer concurrency is `Server protocol and transfers` §6.2's
///    two admission points, all of them on the caller's side, and **a declaration
///    of one would cap machine-side transfers at one** and contradict every one
///    of them. A5a's count of kinds that declare the fifth thing is setup and
///    media and no others, and this is one of the three that were checked and
///    found to hide no exclusion.
pub fn the_transfer_support_kinds_declarations() -> Declaration {
    Declaration {
        resources: vec![
            (Resource::Processors, How::Shared),
            (Resource::MachineMemory, How::Shared),
        ],
        priority: THE_TRANSFER_SUPPORT_KINDS_RANK,
        cancellability: Cancellability::AtAnyMoment,
        atomic_or_partial: "`Server protocol and transfers` T11's — atomic, the unit being one \
                            transfer",
        excludes_its_own_kind: false,
        its_document_states_that_a_further_operation_queues: false,
    }
}

/// **The rank the transfer-support kind is given, and the number is this build's
/// while the ordering is `Server protocol and transfers` T11's.**
///
/// `Server protocol and transfers` T11 states the rank as a relation and not as a figure — *"below the render
/// kind; above the preview and media kinds"* — and A1 item 2 asks for a rank
/// against the other kinds. **So the ordering is held by a test against the
/// declarations this crate actually carries**, and the number is a Build
/// decision recorded here.
///
/// **`Server protocol and transfers` T11 says outright that the rank barely bites**: A5's test is resources,
/// this class holds none exclusively, and A8 makes priority a preference within
/// an admission decision rather than a gate. Nothing anywhere is sized on the
/// number.
pub const THE_TRANSFER_SUPPORT_KINDS_RANK: u8 = 100;

/// **The declaration in force for a kind, which is what A4's one admission point
/// arbitrates against.**
///
/// **A2's two, then `Server protocol and transfers` T11's, then A3's default** —
/// in that order, because A3 is what a kind falls to when its owning document has
/// declared nothing, and a kind whose document has declared must never reach it.
///
/// **This exists because nothing composed it.** The two given declarations and
/// the default each had a way to be reached and no code anywhere said which one
/// arbitrates a given kind, so a caller building an [`Operation`] reached for the
/// default — which for the transfer-support kind is the answer
/// `Server protocol and transfers` T11 says is
/// wrong.
///
/// **Four kinds still fall to A3's default and that is correct rather than
/// missing**: §16 item 6 closed on 2026-08-22 naming where the setup, media,
/// file-inspection and Telegram kinds' declarations live — `Server Setup`
/// §11, `Telegram` M5 in its §12, `Render Settings` W1 in its §10 with
/// `Dependency Upload & Remap` §9, and `Telegram` §19 — and **none of those
/// four documents' code carries its declaration yet.** Each arrives with its own
/// module, and A3 is what makes the wait cost throughput and never a render.
pub fn the_declaration_in_force_for(kind: WorkerKind) -> Declaration {
    if let Some(given) = the_two_declarations_already_given(kind) {
        return given;
    }
    if kind == WorkerKind::TransferSupport {
        return the_transfer_support_kinds_declarations();
    }
    if kind == WorkerKind::FileInspection {
        return the_file_inspection_kinds_declarations();
    }
    Declaration::the_default()
}

/// **`Render Settings` §10 W1: the file-inspection kind's declarations, made by
/// the document W1 names as their owner — with `Dependency Upload & Remap` §9,
/// whose declarations for its check reading agree value for value.** Added
/// 2026-09-29, step 10c's dependency wave piece 2, when a reading of a `.blend`
/// first started a worker of this kind.
///
/// **Written here for the reason [`the_transfer_support_kinds_declarations`]
/// is**: A1 puts the declaring in the owning document and A4 puts the
/// arbitrating at this one point, and `Render Settings`' own statement of them
/// (its crate's `declarations`) is in a crate this one may never reach.
///
/// **Two of A3's values were false of this kind**, which is why it is not left
/// to the default: A3 ranks an undeclared kind last and treats it as not
/// cancellable, and `Render Settings` §10 W1 declares it *"below render; above
/// preview and media"* and *"cancellable: yes, at any moment"*.
///
/// 1. **Resource class** — the processors and the machine's memory **shared**;
///    no card. *"A reading drives a headless Blender that renders nothing."*
/// 2. **Priority** — below the render kind, above the preview and media kinds.
///    [`THE_FILE_INSPECTION_KINDS_RANK`] carries the number.
/// 3. **Cancellable at any moment**, `Render Settings` §10 W2 being its
///    termination contract.
/// 4. **Atomic**, the unit being the sentinel-framed document (`Render
///    Settings` Q7, `Dependency Upload & Remap` G3).
/// 5. **Exclusion: nothing declared, which A1 reads as unlimited** —
///    `Dependency Upload & Remap` O4: several readings of different files may
///    run at once.
pub fn the_file_inspection_kinds_declarations() -> Declaration {
    Declaration {
        resources: vec![
            (Resource::Processors, How::Shared),
            (Resource::MachineMemory, How::Shared),
        ],
        priority: THE_FILE_INSPECTION_KINDS_RANK,
        cancellability: Cancellability::AtAnyMoment,
        atomic_or_partial: "`Render Settings` §10 W1's — atomic, the unit being the \
                            sentinel-framed document",
        excludes_its_own_kind: false,
        its_document_states_that_a_further_operation_queues: false,
    }
}

/// **The rank the file-inspection kind is given.** The ordering is `Render
/// Settings` §10 W1's — below the render kind, above the preview and media
/// kinds — and the number is this build's, the same as the transfer-support
/// kind's, **because no plan orders the two against each other** and A8 makes a
/// rank a preference within an admission decision rather than a gate. Neither
/// kind holds anything exclusively, so the tie decides nothing.
pub const THE_FILE_INSPECTION_KINDS_RANK: u8 = THE_TRANSFER_SUPPORT_KINDS_RANK;

/// **`Server protocol and transfers` T11a: the two admission points are
/// different instruments and neither stands in for the other.**
///
/// `Server protocol and transfers` T5 admits **sessions** against `MaxSessions` and **bytes**
/// against the line, on the desktop side; **A4's is one point per machine
/// admitting worker starts against the contended resources.** A transfer after
/// hand-over passes both — that document's for its channel, A4's for its worker —
/// and **neither figure is derived from the other.**
///
/// **What it forbids here, concretely.** Nothing in this file reads a session
/// count, a `MaxSessions` figure or a rate, and nothing composes A4's answer from
/// one: [`admit`] takes the asking operation, what is admitted, and the
/// setup-in-progress state, and there is no route through it to a number the
/// desktop's own points hold. **A1 item 1 already says the connection is not on
/// the contended list**, and this is the same line drawn from the other side.
pub const THE_TWO_ADMISSION_POINTS_ARE_DIFFERENT_INSTRUMENTS: &str =
    "`Server protocol and transfers` T5's points admit sessions and bytes on the desktop side; \
     A4's admits worker starts against the cards, the processors and the machine's memory. A \
     transfer after hand-over passes both and neither figure is derived from the other";

/// One operation standing at, or admitted through, the admission point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operation {
    pub identity: Identity,
    pub kind: WorkerKind,
    pub declaration: Declaration,
}

/// What the machine currently holds, as the admission point sees it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatIsAdmitted {
    pub operations: Vec<Operation>,
}

impl WhatIsAdmitted {
    /// Whether some admitted operation holds `resource` exclusively.
    fn held_exclusively(&self, resource: Resource) -> Option<&Operation> {
        self.operations.iter().find(|admitted| {
            admitted
                .declaration
                .resources
                .iter()
                .any(|(held, how)| *held == resource && *how == How::Exclusively)
        })
    }

    /// Whether some admitted operation holds `resource` at all, **shared or
    /// exclusively.**
    ///
    /// **This is what an exclusive asker is checked against.** A5 says a
    /// resource declared exclusive is held by **at most one** admitted
    /// operation, so an asker naming a resource exclusively is blocked by any
    /// holder of it and not only by an exclusive one — checking only the
    /// exclusive holders would admit an exclusive asker beside a shared holder
    /// and leave two operations on the resource.
    fn held_at_all(&self, resource: Resource) -> Option<&Operation> {
        self.operations.iter().find(|admitted| {
            admitted
                .declaration
                .resources
                .iter()
                .any(|(held, _)| *held == resource)
        })
    }

    /// **A5a's count: of admitted operations of the same kind on the same
    /// machine.**
    fn one_of_the_same_kind(&self, kind: WorkerKind) -> Option<&Operation> {
        self.operations.iter().find(|admitted| admitted.kind == kind)
    }
}

/// What the admission point answers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Admission {
    /// **A5. An operation is admitted when every resource its class names is
    /// available under the exclusion rule.** A resource declared exclusive is
    /// held by at most one admitted operation; a resource declared shared is held
    /// by any number.
    Admitted,
    /// **A7. Where the holder is not cancellable, the higher-priority operation
    /// waits at the admission point.** It is **not started and then blocked**, so
    /// nothing that has been started is ever holding a resource it cannot use.
    ///
    /// **A9. The one exception to A8 is exclusion, and it is not starvation.**
    /// While a render holds the cards exclusively, another render is not admitted
    /// however long it waits — **that is the allotment promise working, not a
    /// scheduling fault**, and the ordering of what waits is `Queue & History`'s.
    Waits { for_resource: Resource },
    /// **A6. Where an exclusive operation cannot be admitted because a
    /// cancellable, lower-priority operation holds what it needs, that operation
    /// is cancelled and the exclusive one is admitted.** Cancellation is the
    /// kind's own termination contract (X11) and **its outcome is recorded as the
    /// kind's own, never as a failure of the work that displaced it.**
    CancelAndAdmit { cancel: Identity },
    /// **A5b. An operation that A5a or A11 holds back is refused at the admission
    /// point rather than queued, and the refusal names the operation standing in
    /// its way.**
    Refused(WhatTheRefusalCarries),
    /// **A5b's exception**: a kind whose own document states that a further
    /// operation of that kind queues **waits under A7 and is not refused.**
    WaitsBecauseItsDocumentSaysAFurtherOperationQueues { behind: Identity },
}

impl Admission {
    /// **What the admission point answered, in plain words** — for the
    /// machine's own answer to a request, which reaches a person behind *What
    /// the server said*. Added 2026-09-21, step 10c: until then the answer was
    /// this type's debugging rendering, and the second pass on Hyperstack put
    /// *Refused(TheSetupRunTheStandingStateNames(Operation(…* on the screen.
    ///
    /// **The identity a refusal names is kept in the words**, because A5b's
    /// refusal exists to name the operation standing in the way; a caller that
    /// acts on it reads it from its own field rather than from these words.
    pub fn in_words(&self) -> String {
        match self {
            Admission::Admitted => "it was admitted".to_string(),
            Admission::Waits { for_resource } => format!(
                "it is waiting for {} another operation is using",
                match for_resource {
                    Resource::Cards => "the GPUs",
                    Resource::Processors => "the CPUs",
                    Resource::MachineMemory => "the machine's memory",
                }
            ),
            Admission::CancelAndAdmit { cancel } => format!(
                "it is admitted once operation {} is cancelled",
                cancel.as_text()
            ),
            Admission::Refused(WhatTheRefusalCarries::TheAdmittedOperationOfThatKind(it)) => {
                format!(
                    "another operation of the same kind is already running ({})",
                    it.as_text()
                )
            }
            Admission::Refused(WhatTheRefusalCarries::TheSetupRunTheStandingStateNames(run)) => {
                format!(
                    "a setup of this server did not finish, and nothing else runs until it \
                     does (setup run {})",
                    run.as_text()
                )
            }
            Admission::WaitsBecauseItsDocumentSaysAFurtherOperationQueues { behind } => {
                format!("it is queued behind operation {}", behind.as_text())
            }
        }
    }
}

/// **What A5b's refusal carries.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheRefusalCarries {
    /// **Under A5a**: the identity of the admitted operation of that kind.
    TheAdmittedOperationOfThatKind(Identity),
    /// **Under A11**: the operation identity the standing state names (P1),
    /// **which is the report P8 describes.**
    TheSetupRunTheStandingStateNames(Identity),
}

/// **A11. While the machine-level *setup in progress* state is set, nothing is
/// admitted.** That is **an exclusion at machine level rather than a priority**.
///
/// **A11a. One thing is admitted under it, and exactly one: the worker start of
/// the setup run the state itself names**, after that run has taken the state
/// over — `Server Setup` S4 item 2's recovery run, **and nothing else**. Not a
/// render, not a reading, not a derivative, not a role worker's work, and not a
/// second setup run.
///
/// **Why the exception is necessary rather than convenient**: the state is set
/// before the steps it holds back, so **the run that takes it over would
/// otherwise be refused by the rule it is carrying out** — the state is then never
/// cleared, and `Server Setup`'s Rows 106, 112 and 114 fail together, leaving a
/// machine that admits nothing for good.
///
/// **An ordinary setup run needs no exception**: A4 gates worker starts, and its
/// worker was started before the state was set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupInProgress {
    /// **P1.** The operation identity of the setup run that set it.
    pub the_run_it_names: Identity,
}

/// **The admission point — A4, A5, A5a, A5b, A6, A7, A9, A10, A11 and A11a in
/// one place, because a single point is what makes exclusion a property.**
///
/// **A10. A held job is not at the admission point at all.** §8.2 is why: a hold
/// is not a low-priority admission, **it is an absence from the line** — so a
/// caller never passes a held subject to this function, and there is no *held*
/// answer among the outcomes.
pub fn admit(
    asking: &Operation,
    admitted: &WhatIsAdmitted,
    setup_in_progress: Option<&SetupInProgress>,
) -> Admission {
    // A11 and A11a first: the machine-level exclusion is not a priority and
    // nothing competes with it.
    if let Some(state) = setup_in_progress {
        if asking.identity == state.the_run_it_names {
            return Admission::Admitted;
        }
        return Admission::Refused(WhatTheRefusalCarries::TheSetupRunTheStandingStateNames(
            state.the_run_it_names.clone(),
        ));
    }

    // A5a: the fifth declaration, counted, and it is a test beside A5 rather
    // than a sixth resource.
    if asking.declaration.excludes_its_own_kind {
        if let Some(standing) = admitted.one_of_the_same_kind(asking.kind) {
            return if asking
                .declaration
                .its_document_states_that_a_further_operation_queues
            {
                Admission::WaitsBecauseItsDocumentSaysAFurtherOperationQueues {
                    behind: standing.identity.clone(),
                }
            } else {
                Admission::Refused(WhatTheRefusalCarries::TheAdmittedOperationOfThatKind(
                    standing.identity.clone(),
                ))
            };
        }
    }

    // A5: every resource the class names, under the exclusion rule.
    for (resource, how) in &asking.declaration.resources {
        // **A shared claim is blocked by an exclusive holder; an exclusive claim
        // is blocked by any holder at all.** The second half is what makes A5's
        // *at most one admitted operation* true of a resource declared
        // exclusive. **The first half is unchanged and is correct**: a shared
        // asker is not blocked by another shared holder.
        //
        // **A13 is why this matters while nothing declares a card shared**: the
        // mechanism is built to be tightened by one change to a declaration with
        // nothing in this section moving, and the day one is tightened, a check
        // that only looked at exclusive holders would silently stop being
        // exclusive.
        let asking_exclusively = *how == How::Exclusively;
        let holder = if asking_exclusively {
            admitted.held_at_all(*resource)
        } else {
            admitted.held_exclusively(*resource)
        };
        let Some(holder) = holder else {
            continue;
        };
        // A6: an exclusive asker displaces a cancellable, lower-priority
        // holder. Lower priority is the larger number.
        let holder_is_lower_priority = holder.declaration.priority > asking.declaration.priority;
        let holder_is_cancellable =
            holder.declaration.cancellability != Cancellability::NotCancellable;
        if asking_exclusively && holder_is_lower_priority && holder_is_cancellable {
            return Admission::CancelAndAdmit {
                cancel: holder.identity.clone(),
            };
        }
        return Admission::Waits {
            for_resource: *resource,
        };
    }
    Admission::Admitted
}

/// **A8. No class is starved indefinitely.**
///
/// **A class admitted only when no higher class is waiting is a class that never
/// runs on a machine with a queue.** Every class is guaranteed a share of
/// admissions, and **priority is a preference within each admission decision
/// rather than a gate** — the same rule and the same reason as `Server protocol
/// and transfers` T10.
///
/// **A refused operation is not a waiting one**, so this guarantee does not bear
/// on one, and A9's exception is untouched.
pub const EVERY_CLASS_IS_GUARANTEED_A_SHARE_OF_ADMISSIONS: bool = true;

/// **A12. Whether a reading or a derivative disturbs a running render is NOT
/// answered.**
///
/// Five reads taken during a 21-frame render all succeeded and the render
/// finished all 21 frames and all 105 files, **but the run totalled 155.0 s
/// against a control's 79.8 s while the reads spanned only the first 40 s, and
/// three testers shared four cores — *"the two cannot be separated"*.**
///
/// **What stands is that every such run completed correctly; the magnitude is
/// untested** (§16 item 7). Nothing in this crate is written as though the cost
/// were known, and this constant is the record of that rather than a figure.
pub const WHAT_A_READING_COSTS_A_RENDER_BESIDE_IT_IS_UNMEASURED: bool = true;

#[cfg(test)]
mod tests {
    use smoozy_state::identities::IdentityKind;

    use super::*;

    fn an_identity() -> Identity {
        Identity::issue(IdentityKind::Operation).expect("randomness")
    }

    fn a_render() -> Operation {
        Operation {
            identity: an_identity(),
            kind: WorkerKind::Render,
            declaration: the_two_declarations_already_given(WorkerKind::Render)
                .expect("the render's declaration is given"),
        }
    }

    fn a_derivative() -> Operation {
        Operation {
            identity: an_identity(),
            kind: WorkerKind::Preview,
            declaration: the_two_declarations_already_given(WorkerKind::Preview)
                .expect("B24's declaration is given"),
        }
    }

    fn a_setup_run() -> Operation {
        Operation {
            identity: an_identity(),
            kind: WorkerKind::Setup,
            declaration: Declaration {
                resources: vec![
                    (Resource::Processors, How::Shared),
                    (Resource::MachineMemory, How::Shared),
                ],
                priority: 10,
                cancellability: Cancellability::OnlyAtARecordedSafeBoundary,
                atomic_or_partial: "`Server Setup` §11's",
                excludes_its_own_kind: true,
                its_document_states_that_a_further_operation_queues: false,
            },
        }
    }

    fn a_media_production() -> Operation {
        Operation {
            identity: an_identity(),
            kind: WorkerKind::Media,
            declaration: Declaration {
                resources: vec![
                    (Resource::Processors, How::Shared),
                    (Resource::MachineMemory, How::Shared),
                ],
                priority: 150,
                cancellability: Cancellability::AtAnyMoment,
                atomic_or_partial: "`Telegram` M5's",
                excludes_its_own_kind: true,
                its_document_states_that_a_further_operation_queues: true,
            },
        }
    }

    #[test]
    fn a_derivative_is_admitted_beside_a_render_because_b24_says_never_blocking_one() {
        let render = a_render();
        let admitted = WhatIsAdmitted {
            operations: vec![render],
        };
        assert_eq!(
            admit(&a_derivative(), &admitted, None),
            Admission::Admitted
        );
    }

    #[test]
    fn a_second_render_waits_on_the_cards_however_long_and_is_never_refused() {
        let admitted = WhatIsAdmitted {
            operations: vec![a_render()],
        };
        assert_eq!(
            admit(&a_render(), &admitted, None),
            Admission::Waits {
                for_resource: Resource::Cards
            }
        );
    }

    /// A6: an exclusive asker displaces a cancellable, lower-priority holder.
    #[test]
    fn an_exclusive_asker_cancels_a_cancellable_lower_priority_holder_of_what_it_needs() {
        let mut a_card_holding_derivative = a_derivative();
        a_card_holding_derivative
            .declaration
            .resources
            .push((Resource::Cards, How::Exclusively));
        let its_identity = a_card_holding_derivative.identity.clone();
        let admitted = WhatIsAdmitted {
            operations: vec![a_card_holding_derivative],
        };
        assert_eq!(
            admit(&a_render(), &admitted, None),
            Admission::CancelAndAdmit { cancel: its_identity }
        );
    }

    /// A7: where the holder is not cancellable, the higher-priority operation
    /// waits rather than displacing it.
    #[test]
    fn a_higher_priority_asker_waits_on_a_holder_that_cannot_be_cancelled() {
        let mut stubborn = a_derivative();
        stubborn.declaration.cancellability = Cancellability::NotCancellable;
        stubborn
            .declaration
            .resources
            .push((Resource::Cards, How::Exclusively));
        let admitted = WhatIsAdmitted {
            operations: vec![stubborn],
        };
        assert_eq!(
            admit(&a_render(), &admitted, None),
            Admission::Waits {
                for_resource: Resource::Cards
            }
        );
    }

    /// A5a and A5b: the setup kind's second submission is refused, naming the
    /// open run — `Server Setup` Y9's own sentence.
    #[test]
    fn a_second_setup_submission_is_refused_naming_the_open_run() {
        let open = a_setup_run();
        let its_identity = open.identity.clone();
        let admitted = WhatIsAdmitted {
            operations: vec![open],
        };
        assert_eq!(
            admit(&a_setup_run(), &admitted, None),
            Admission::Refused(WhatTheRefusalCarries::TheAdmittedOperationOfThatKind(
                its_identity
            ))
        );
    }

    /// A5b's narrowing: a further media production waits, because `Telegram` M4
    /// says its further completed stretches queue.
    #[test]
    fn a_second_media_production_waits_rather_than_being_refused() {
        let running = a_media_production();
        let its_identity = running.identity.clone();
        let admitted = WhatIsAdmitted {
            operations: vec![running],
        };
        assert_eq!(
            admit(&a_media_production(), &admitted, None),
            Admission::WaitsBecauseItsDocumentSaysAFurtherOperationQueues {
                behind: its_identity
            }
        );
    }

    /// **A11: while the state stands, nothing is admitted — and A11a's one
    /// exception is the run the state names, held over every kind of worker
    /// this machine knows rather than over the three somebody thought of.**
    ///
    /// **A11a names what is not admitted and the list is the point**: *"not a
    /// render, not a reading, not a derivative, not a role worker's work, and
    /// not a second setup run."* **Those are five of [`WorkerKind::ALL`]'s
    /// seven**, so the test walks all seven and a kind added later fails here
    /// until somebody says which side of A11a it falls on.
    ///
    /// **The one admitted is the setup run the state names** — `Server Setup`
    /// S4 item 2's recovery, after the take-over re-pointed the state at it.
    /// Without that exception the recovery would be refused by the rule it is
    /// carrying out, the state would never be cleared, and `Server Setup`'s
    /// Rows 106, 112 and 114 would fail together.
    #[test]
    fn the_setup_in_progress_state_admits_only_the_run_it_names() {
        let named = an_identity();
        let state = SetupInProgress {
            the_run_it_names: named.clone(),
        };
        for kind in WorkerKind::ALL {
            let declaration =
                the_two_declarations_already_given(kind).unwrap_or_else(Declaration::the_default);
            let somebody_else = Operation {
                identity: an_identity(),
                kind,
                declaration,
            };
            assert_eq!(
                admit(&somebody_else, &WhatIsAdmitted::default(), Some(&state)),
                Admission::Refused(WhatTheRefusalCarries::TheSetupRunTheStandingStateNames(
                    named.clone()
                )),
                "a worker of kind {kind:?} was admitted while the setup-in-progress state stood"
            );
        }
        // **The exception, and it is the whole of it.**
        let recovery = Operation {
            identity: named.clone(),
            ..a_setup_run()
        };
        assert_eq!(
            admit(&recovery, &WhatIsAdmitted::default(), Some(&state)),
            Admission::Admitted
        );
        // **And a second setup run that is not the named one is refused**,
        // which is the case A11a calls out by name.
        assert!(matches!(
            admit(&a_setup_run(), &WhatIsAdmitted::default(), Some(&state)),
            Admission::Refused(_)
        ));
        // **Nothing changes when the state is gone**: a render is admitted
        // again, so the refusals above are the state's and not the machine's
        // ordinary arbitration.
        assert_eq!(
            admit(&a_render(), &WhatIsAdmitted::default(), None),
            Admission::Admitted
        );
    }

    /// §16 item 30: a media production held back by A11 rather than by A5a is
    /// still refused, and the same picture is lost. **Built as the plan writes
    /// it, and recorded as a residual rather than repaired here.**
    #[test]
    fn a_media_production_held_back_by_the_setup_state_is_refused_and_that_is_item_thirty() {
        let state = SetupInProgress {
            the_run_it_names: an_identity(),
        };
        assert!(matches!(
            admit(&a_media_production(), &WhatIsAdmitted::default(), Some(&state)),
            Admission::Refused(WhatTheRefusalCarries::TheSetupRunTheStandingStateNames(_))
        ));
    }

    #[test]
    fn the_default_declaration_takes_no_resource_exclusively_and_ranks_last() {
        let default = Declaration::the_default();
        assert!(default
            .resources
            .iter()
            .all(|(_, how)| *how == How::Shared));
        assert!(!default
            .resources
            .iter()
            .any(|(resource, _)| *resource == Resource::Cards));
        assert_eq!(default.priority, u8::MAX);
        assert_eq!(default.cancellability, Cancellability::NotCancellable);
        assert!(!default.excludes_its_own_kind);
    }

    /// **The one value A3's default had wrong for this kind.**
    ///
    /// `Server protocol and transfers` T11 says the default's *not cancellable*
    /// is false of the transfer-support kind, because
    /// `Server protocol and transfers` C22 gives
    /// it a termination contract — X14's staging and cleanup rather than a
    /// signal. **This is the assertion that would have failed before the
    /// declaration was made**, and it is written against the default rather
    /// than against a literal so that it goes on meaning the same thing if
    /// A3's default ever moves.
    #[test]
    fn the_transfer_support_kind_is_cancellable_and_the_default_says_it_is_not() {
        let declared = the_declaration_in_force_for(WorkerKind::TransferSupport);
        assert_eq!(declared.cancellability, Cancellability::AtAnyMoment);
        assert_eq!(
            Declaration::the_default().cancellability,
            Cancellability::NotCancellable,
            "if this ever stops being true, the wrong answer this test guards has moved"
        );
        assert_ne!(declared, Declaration::the_default());
    }

    /// **The other four of `Server protocol and transfers` T11's five**, each
    /// against the clause's own words.
    #[test]
    fn the_transfer_support_kinds_other_four_declarations_are_t11s() {
        let declared = the_transfer_support_kinds_declarations();
        // 1. The processors and the machine's memory shared; no card.
        assert!(declared.resources.iter().all(|(_, how)| *how == How::Shared));
        assert!(!declared
            .resources
            .iter()
            .any(|(resource, _)| *resource == Resource::Cards));
        // 4. Atomic, the unit being one transfer.
        assert!(declared.atomic_or_partial.contains("atomic"));
        assert!(declared.atomic_or_partial.contains("one transfer"));
        // 5. Nothing declared, which A1 reads as unlimited — so A5a never
        //    counts this kind and A5b never refuses one behind another.
        assert!(!declared.excludes_its_own_kind);
        assert!(!declared.its_document_states_that_a_further_operation_queues);
        let running = Operation {
            identity: an_identity(),
            kind: WorkerKind::TransferSupport,
            declaration: declared.clone(),
        };
        let admitted = WhatIsAdmitted {
            operations: vec![running],
        };
        let another = Operation {
            identity: an_identity(),
            kind: WorkerKind::TransferSupport,
            declaration: declared,
        };
        assert_eq!(admit(&another, &admitted, None), Admission::Admitted);
    }

    /// **2. Below the render kind; above the preview and media kinds** — held
    /// against the declarations this crate carries rather than against the
    /// number, because the number is this build's and the ordering is
    /// `Server protocol and transfers` T11's. Lower is preferred.
    #[test]
    fn the_transfer_support_kind_ranks_below_the_render_and_above_the_derivatives() {
        let transfers = the_declaration_in_force_for(WorkerKind::TransferSupport).priority;
        let render = the_declaration_in_force_for(WorkerKind::Render).priority;
        let preview = the_declaration_in_force_for(WorkerKind::Preview).priority;
        assert!(render < transfers, "below the render kind");
        assert!(transfers < preview, "above the preview kind");
        // **The media kind's own declaration is `Telegram` M5's and no code in
        // this workspace carries it**, so the media half of the ordering is
        // held against the fixture this file builds one from rather than
        // against a declaration that does not exist. When `Telegram`'s code
        // lands, this line moves to it.
        assert!(
            transfers < a_media_production().declaration.priority,
            "above the media kind"
        );
    }

    /// **A6 does not arise for this kind, which is
    /// `Server protocol and transfers` T11's own sentence.** A6 cancels only an
    /// operation holding what an exclusive operation needs, and this class holds
    /// nothing exclusively — so a render asking for the cards waits on whatever
    /// holds them and never displaces a transfer.
    #[test]
    fn a_render_never_cancels_a_transfer_support_worker_because_it_holds_nothing_exclusively() {
        let transfers = Operation {
            identity: an_identity(),
            kind: WorkerKind::TransferSupport,
            declaration: the_declaration_in_force_for(WorkerKind::TransferSupport),
        };
        let admitted = WhatIsAdmitted {
            operations: vec![transfers],
        };
        assert_eq!(admit(&a_render(), &admitted, None), Admission::Admitted);
    }

    /// **Which kinds reach A3's default, said as a list rather than left to be
    /// discovered.** Four declarations exist in code — A2's two,
    /// `Server protocol and transfers` T11's and, from step 10c's dependency
    /// wave piece 2, `Render Settings` §10 W1's — and the other three kinds
    /// fall to the default until their own documents' code carries theirs.
    #[test]
    fn four_kinds_are_declared_in_code_and_the_other_three_fall_to_the_default() {
        for kind in WorkerKind::ALL {
            let declared = the_declaration_in_force_for(kind) != Declaration::the_default();
            let should_be = matches!(
                kind,
                WorkerKind::Render
                    | WorkerKind::Preview
                    | WorkerKind::TransferSupport
                    | WorkerKind::FileInspection
            );
            assert_eq!(declared, should_be, "{kind:?}");
        }
    }

    /// **`Render Settings` §10 W1, held against its own words**: shared and no
    /// card, below the render and above the derivatives, cancellable at any
    /// moment, atomic, and nothing declared at the fifth — so a reading is
    /// admitted beside a render and beside another reading.
    #[test]
    fn the_file_inspection_kind_is_render_settings_w1s_and_a_reading_runs_beside_a_render() {
        let declared = the_declaration_in_force_for(WorkerKind::FileInspection);
        assert!(declared.resources.iter().all(|(_, how)| *how == How::Shared));
        assert!(!declared
            .resources
            .iter()
            .any(|(resource, _)| *resource == Resource::Cards));
        let render = the_declaration_in_force_for(WorkerKind::Render).priority;
        let preview = the_declaration_in_force_for(WorkerKind::Preview).priority;
        assert!(render < declared.priority && declared.priority < preview);
        assert!(declared.priority < a_media_production().declaration.priority);
        assert_eq!(declared.cancellability, Cancellability::AtAnyMoment);
        assert!(declared.atomic_or_partial.contains("atomic"));
        assert!(!declared.excludes_its_own_kind);
        assert!(!declared.its_document_states_that_a_further_operation_queues);

        let a_reading = || Operation {
            identity: an_identity(),
            kind: WorkerKind::FileInspection,
            declaration: declared.clone(),
        };
        let admitted = WhatIsAdmitted {
            operations: vec![a_render(), a_reading()],
        };
        assert_eq!(admit(&a_reading(), &admitted, None), Admission::Admitted);
    }

    /// **`Server protocol and transfers` T11a**: nothing here reads a session
    /// count or a rate, and the sentence says which two points are meant.
    #[test]
    fn the_two_admission_points_are_named_as_two_and_neither_derives_the_other() {
        assert!(THE_TWO_ADMISSION_POINTS_ARE_DIFFERENT_INSTRUMENTS.contains("sessions"));
        assert!(THE_TWO_ADMISSION_POINTS_ARE_DIFFERENT_INSTRUMENTS.contains("worker starts"));
        assert!(
            THE_TWO_ADMISSION_POINTS_ARE_DIFFERENT_INSTRUMENTS
                .contains("neither figure is derived from the other")
        );
    }

    /// A3's own reason: an undeclared kind is admitted beside a render rather
    /// than barred whenever anything exclusive is running, which would be
    /// starvation.
    #[test]
    fn an_undeclared_kind_is_admitted_beside_a_render() {
        let undeclared = Operation {
            identity: an_identity(),
            kind: WorkerKind::FileInspection,
            declaration: Declaration::the_default(),
        };
        let admitted = WhatIsAdmitted {
            operations: vec![a_render()],
        };
        assert_eq!(admit(&undeclared, &admitted, None), Admission::Admitted);
    }

    #[test]
    fn the_contended_list_is_closed_at_three() {
        // **The list is closed by the compiler rather than by a count.** The
        // match is total and carries no default arm, so a resource added to the
        // enum has none, the crate stops compiling, and whoever added it is sent
        // to this list — where the length assertion this replaced could never
        // have been false, the array's own type having fixed it.
        for (at, resource) in Resource::ALL.iter().enumerate() {
            let where_the_list_puts_it = match resource {
                Resource::Cards => 0,
                Resource::Processors => 1,
                Resource::MachineMemory => 2,
            };
            assert_eq!(at, where_the_list_puts_it, "{resource:?} is not where the list puts it");
        }
    }

    /// **A5's *at most one*, from the other side.** An asker naming a resource
    /// **exclusively** is blocked by a holder that named it **shared**, not only
    /// by one that named it exclusively — otherwise both are admitted and the
    /// resource is held by two.
    ///
    /// **A13 is why this matters while nothing declares a card shared**: one
    /// change to a declaration tightens the mechanism with nothing in this
    /// section moving.
    #[test]
    fn an_exclusive_asker_is_refused_against_a_shared_holder() {
        let mut a_shared_card_holder = a_derivative();
        a_shared_card_holder
            .declaration
            .resources
            .push((Resource::Cards, How::Shared));
        // Not cancellable, so A6's displacement is not what answers here.
        a_shared_card_holder.declaration.cancellability = Cancellability::NotCancellable;
        let admitted = WhatIsAdmitted {
            operations: vec![a_shared_card_holder],
        };
        assert_eq!(
            admit(&a_render(), &admitted, None),
            Admission::Waits {
                for_resource: Resource::Cards
            },
            "an exclusive claim is blocked by any holder of that resource"
        );
    }

    /// **And the direction that was already right is untouched**: a shared asker
    /// is not blocked by another shared holder.
    #[test]
    fn a_shared_asker_is_not_blocked_by_another_shared_holder() {
        let admitted = WhatIsAdmitted {
            operations: vec![a_derivative()],
        };
        let mut another = a_derivative();
        another.declaration.excludes_its_own_kind = false;
        assert_eq!(admit(&another, &admitted, None), Admission::Admitted);
    }
    /// **Step 10c, 2026-09-21: what the admission point answered reaches a
    /// person in words, and never in this type's debugging rendering** — the
    /// second pass on Hyperstack put *Refused(TheSetupRunTheStandingStateNames(*
    /// behind *What the server said*.
    #[test]
    fn every_admission_answer_has_words_and_none_is_a_debugging_rendering() {
        let someone = Identity::issue(IdentityKind::Operation).expect("randomness");
        for answer in [
            Admission::Admitted,
            Admission::Waits { for_resource: Resource::Cards },
            Admission::CancelAndAdmit { cancel: someone.clone() },
            Admission::Refused(WhatTheRefusalCarries::TheAdmittedOperationOfThatKind(
                someone.clone(),
            )),
            Admission::Refused(WhatTheRefusalCarries::TheSetupRunTheStandingStateNames(
                someone.clone(),
            )),
            Admission::WaitsBecauseItsDocumentSaysAFurtherOperationQueues {
                behind: someone.clone(),
            },
        ] {
            let said = answer.in_words();
            assert!(!said.is_empty());
            for debugging in ["Refused(", "TheSetupRun", "Operation(", "Waits {"] {
                assert!(!said.contains(debugging), "{said}");
            }
        }
    }

}
