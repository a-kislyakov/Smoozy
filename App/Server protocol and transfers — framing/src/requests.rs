//! The requests the render path sends a machine, each named once, with the
//! fields each one carries.
//!
//! **What this is.** One name and one field shape for each request that step
//! 10c's render path adds, in the one crate both ends read — so the desktop's
//! sender and the machine's answerer cannot spell one thing two ways. **Nothing
//! here sends, answers or dispatches anything.** A name here is a promise that
//! a request of that name exists; the program the machine runs refuses every
//! one of them by name until the piece that answers it lands, and its two lists
//! say so and say who.
//!
//! **The names are Build's; no plan names a request.** A request's contents
//! belong to the document that owns it. **Where an owning clause fixes a field,
//! the field is kept and the clause is cited at it. Where no clause fixes one,
//! the smallest shape that carries what the clause says is chosen and the
//! choice is written at the site**, and in
//! `Audits/Build/Server protocol and transfers/Decisions record.md`.
//!
//! # How a request is framed, and what this file adds to it
//!
//! Every request here rides §2's frame unchanged: P1's one request per
//! connection, P2's protocol version, and the operation identity C1 has the
//! initiating side choose before it is sent — **the frame's own identity is
//! that operation identity and it is never repeated as a field.** C2 matches
//! the result by it and by nothing else. **What this file adds is the name and
//! the named fields**, each field's value being text (P5's list read by name).
//! An identity is written in its own textual form, which is
//! `State, identity and recovery` §2.1's and is read back with the kind the
//! field names; a sequence number is written as a decimal number.
//!
//! **C6 classes every request as idempotent under C4 or a pure reading, and
//! each one here says which.** A request that acts is recorded on the machine
//! before it acts (C4), and repeating its identity never acts again; a reading
//! carries its identity for correlation and reporting only.
//!
//! # The answers, named here too
//!
//! **A result's fields are named in this file for the same reason the request
//! names are** — the desktop reads them and the machine writes them. Step 10c
//! piece 5 composed the first answers and put their names here, under *the
//! answers* below, and not in the machine's own crate. **An answer that
//! carries a list** — the queue's jobs, the journal's events, a note's re-run
//! marks — carries one field per item and value, named by
//! [`the_field_of_item`], with a count beside it, because §2's frame is a list
//! of fields read by name (P5) and a name read twice would give its first
//! value both times.
//!
//! # What is deliberately not here
//!
//! - ~~**Stop.** No milestone this path is built for stops a job, so C21's
//!   command is not named here, and the program's list still carries it under
//!   its own words.~~ **Named since 2026-09-29, step 10c's first stop-and-clear piece**: C21's stop,
//!   `Queue & History` L13's clear and the stop paired with it, and
//!   `Queue & History` L1's cancel are all named below, under *the queue's own acts*, with the two
//!   rewrites — the reorder and the block move, which the machine answers
//!   since 2026-10-04, the 38's piece 8.
//! - **The setup run's own field is here now.** It was spelled only in the
//!   machine's own crate until step 10c piece 8, which is where that crate's
//!   two acts on the setup-in-progress state needed it; a setup step and a
//!   run's state name the same run, and the desktop cannot reach that crate on
//!   Windows. **It is declared here as [`THE_SETUP_RUN_FIELD`] and that crate
//!   re-exports it**, so there is still one spelling.
//! - **A stop of a setup run.** `Server Setup` Y8's stop is the durable desired
//!   state written through the ordinary command path, which is
//!   `Rented-server runtime` X8's and is already named there; it is not a
//!   request of this file's.
//!
//! # Markers
//!
//! **A bare marker in this file is a clause of `Server protocol and transfers`.
//! Every other document's marker has its document written immediately before
//! it**, every time — there is no header list here, because nearly every other
//! document this file cites shares a letter series with that plan, and a
//! header list cannot tell the two apart.

/// **Where a request stands under C6.** *"Every request this document defines
/// is idempotent under C4 or is a pure reading."*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnderC6 {
    /// **A pure reading.** It carries an identity for correlation and reporting
    /// rather than for idempotency, and it changes nothing on the machine.
    APureReading,
    /// **It acts, so it is recorded before it acts** — C4: *"The machine
    /// records the operation identity durably before acting on it … A request
    /// repeating an identity already recorded never acts again."*
    RecordedBeforeItActs,
    /// **Neither, and it is the one request here that is.** Added 2026-10-02,
    /// step 10c's Files' owed acts piece 7, for
    /// [`THE_FILE_IS_COPIED_ON_THE_MACHINE_REQUEST`]. `Server protocol and
    /// transfers` §1.1's 2026-10-02 paragraph has the entry point answer it
    /// itself and never pass it to the supervisor, and the paragraph after
    /// `Rented-server runtime` W2 gives it no record — **so nothing is recorded
    /// before it acts, which is what C4 asks, and it is no reading.** C6 says a
    /// builder who finds such a request has found a fault in that section; the
    /// fault is routed to that plan by the piece's account rather than hidden
    /// under one of the two above. **What keeps a repeat from doing harm
    /// instead**: it writes one file only, at a staging path the desktop
    /// synthesised from an identity of its own, created exclusively — so a
    /// repeat finds the path taken and writes nothing.
    OutsideC6,
}

/// One request named here: its name, where it stands under C6, and the fields
/// named here that it carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ARequestNamedHere {
    pub name: &'static str,
    pub under_c6: UnderC6,
    /// **The fields this file names that the request carries.** Each is
    /// required unless the request's own constant says when it is absent, and
    /// a field this file does not name is said there too.
    pub carries: &'static [&'static str],
}

// ------------------------------------------------------------ the fields

/// **The job identity**, `Rendering` S3's: issued at the desk before anything
/// is sent, in `Queue & History` Y2's textual form.
pub const THE_JOB_FIELD: &str = "job";

/// **The job's label, whole** — `Queue & History` Y8 and
/// `Queue & History` Y9: *"the label travels in the record and the note"*, and
/// nothing parses a folder name back into one. **Present exactly where a label was given**, which is
/// `Job files and outputs` N3's own condition, and never an empty stand-in.
pub const THE_LABEL_FIELD: &str = "label";

/// **A reader's cursor**, `State, identity and recovery` E2's, as a decimal
/// sequence number.
pub const THE_CURSOR_FIELD: &str = "cursor";

/// **Which kind of setup act a run is** — `Server Setup` Y6b's per-kind record:
/// the ready action, a Blender install, a Blender removal, an add-on install,
/// the Telegram placement.
pub const THE_KIND_OF_SETUP_FIELD: &str = "kind of setup";

/// **`Rented-server runtime` P1's own field: the operation identity of the
/// setup run.**
///
/// **Declared here from step 10c piece 8, and re-exported by the machine's own
/// crate**, where it was spelled until then. Four requests name a run — the two
/// acts on the setup-in-progress state, one setup step, and the run's state —
/// and two of the four are the desktop's to send from a build that cannot reach
/// that crate at all.
pub const THE_SETUP_RUN_FIELD: &str = "setup run";

/// **The setup run this run takes over** — `Server Setup` S4 item 2's recovery
/// command, *"an ordinary command with its own operation identity, naming the
/// setup operation the state names"*.
///
/// **Present exactly on a recovery run, and its presence is what makes the run
/// one.** **Chosen here**: a request carrying it carries no
/// [`THE_KIND_OF_SETUP_FIELD`], because `Server Setup` Y6b gives a recovery run
/// *"the taken-over run's kind"* — so the kind is read off the taken-over run's
/// own record and the two can never disagree. A request carrying both is
/// refused by name.
pub const THE_RUN_TAKEN_OVER_FIELD: &str = "the run taken over";

/// **The recovery the user asked to abandon the machine-changing work** —
/// `Server Setup` S6's second clear, as the recovery command carries it. Added
/// 2026-09-22.
///
/// **Present only beside [`THE_RUN_TAKEN_OVER_FIELD`]**, with the one value
/// [`THE_RECOVERY_ABANDONS`]: an abandon *is* a recovery (`Server Setup` S6: *"a recovery run
/// the user asked to abandon"*), so it takes the standing state over exactly as
/// `Server Setup` S4 item 2 has any recovery do, and differs in what it then
/// drives. **Chosen here, the plan naming no field**: a second request would be
/// a second take-over route beside the one `Server Setup` S4 item 2 already has.
pub const THE_RECOVERY_ABANDONS_FIELD: &str = "the recovery abandons";

/// **The program that honours the mark says it back**, the same field and
/// value on its answer, so a desktop can tell it from a program placed before
/// 2026-09-22, which reads the recovery and not the mark.
///
/// The one value [`THE_RECOVERY_ABANDONS_FIELD`] carries.
pub const THE_RECOVERY_ABANDONS: &str = "abandon the machine-changing work";

/// **Which step of a setup run is commanded** — one of the machine-side steps
/// `Server Setup` Y7 gives the setup worker, from
/// [`EVERY_SETUP_STEP_THE_WORKER_IS_COMMANDED`] below.
///
/// **On the run's state reading it is the step under way, and it has two
/// spellings, one per side of `Server Setup` Y7's split. Added 2026-09-29,
/// step 10c's transfer wave piece 12.** A step the worker is commanded is
/// named as it was commanded; **an admin step the desktop recorded as started
/// ([`THE_ELEVATED_STEPS_START_IS_RECORDED_REQUEST`]) is named by the first of
/// `Server Setup` Y4's boundaries it reaches**, from [`EVERY_SETUP_BOUNDARY`] —
/// the naming [`THE_FAILED_STEP_FIELD`] already uses for a step. **The two
/// sets share no word**, which a test below holds, so one field carries either
/// with nothing to tell apart.
pub const THE_STEP_FIELD: &str = "step";

/// **The Blender version a step is about** — `Server Setup` B4's fetch derives
/// its whole address from it, and `Server Setup` R1 item 7's publication
/// requires the staged build to answer it.
///
/// **Chosen here, and it is the security-carrying choice of this piece: no
/// address is ever carried.** B4 fixes the archive's folder and file name and
/// the checksum file beside it, and that derivation is built on the machine at
/// `Server Setup`'s own `blender`. A desktop that sent an address would be a
/// second spelling of B4 **and would let whatever composed the request point
/// the machine's own line at any host**. The version is written in the same
/// three-part textual form the build answers.
pub const THE_BLENDER_VERSION_FIELD: &str = "blender version";

/// **The executable a library step reads against** — `Server Setup` R1 item 4:
/// *"`ldd` is run over the Blender executable the machine will render with —
/// the staged build where this run fetched one, otherwise each build
/// `Server Setup` B10's search found"*.
///
/// **Absent means the build this run staged**, which the worker holds in its
/// own run state and the desktop need not name. **Present names a build the
/// search found**, which is the desktop's knowledge and no reading of the
/// worker's. That is `Server Setup` R1 item 4's two cases in one field.
///
/// **It is a protocol field and never a command argument** (`Security and
/// licence` §10), so a machine-origin path crosses without touching
/// `Server protocol and transfers` §1.1's admissibility rule.
pub const THE_EXECUTABLE_FIELD: &str = "executable";

// ---- `Server Setup` Y6b's five kinds of setup act, as the wire spells them.
//
// **The words are Build's**; the five acts are `Server Setup` Y6b's and the set is closed
// there. The machine's own `retention::SetupActKind` and `Server Setup`'s
// `run::SetupKind` are the same five, and the program holds this list to one of
// them by name so a kind cannot be added on one side alone.

/// `Server Setup` §2's machine-preparation run.
pub const THE_READY_ACTION: &str = "the ready action";
/// A Blender install — `Server Setup` §5.1's fetch, or `Server Setup` B7's
/// build the user pointed at.
pub const A_BLENDER_INSTALL: &str = "a blender install";
/// `Server Setup` B15's removal.
pub const A_BLENDER_REMOVAL: &str = "a blender removal";
/// `Server Setup` §6's add-on install.
pub const AN_ADD_ON_INSTALL: &str = "an add-on install";
/// `Server Setup` §7's dedicated entry point.
pub const THE_TELEGRAM_PLACEMENT: &str = "the telegram placement";

/// **`Server Setup` Y6b's five kinds, closed.**
pub const EVERY_KIND_OF_SETUP: [&str; 5] = [
    THE_READY_ACTION,
    A_BLENDER_INSTALL,
    A_BLENDER_REMOVAL,
    AN_ADD_ON_INSTALL,
    THE_TELEGRAM_PLACEMENT,
];

// ---- `Server Setup` Y7's machine-side steps, as the wire spells them.
//
// **These are `Server Setup` Y7's own list and not `Server Setup` R1's nine steps**, and the
// two are different granularities on purpose. `Server Setup` R1's step is what a boundary
// hangs on; `Server Setup` Y7's machine-side list — *"fetch, unpack, `ldd` reads, publication,
// read-backs"* — is what the worker is commanded. The tooling step's install
// and the libraries step's install are elevated and are the desktop's
// (`Server Setup`'s own `driving::who_performs`), so the worker's part of each
// is named separately here and each names the `Server Setup` R1 step whose
// boundary it feeds.
//
// **A step names its step rather than meaning *the next one***, so a worker and
// a desktop that disagree about where a run has got to find it out at the
// command rather than by performing the wrong act.

/// **`Server Setup` R1 item 1's readings.** **Refused by name**: see
/// [`EVERY_SETUP_STEP_THE_WORKER_IS_COMMANDED`].
pub const THE_READINGS_STEP: &str = "the readings";
/// **`Server Setup` B4's fetch**, on the machine's own line, staged, resumable
/// and checked against the published checksum. Carries
/// [`THE_BLENDER_VERSION_FIELD`].
pub const THE_FETCH_STEP: &str = "the fetch";
/// **`Server Setup` R1 item 3's second half**: the staged archive unpacked with
/// the one tool Blender's documentation names and nothing else. Carries
/// [`THE_BLENDER_VERSION_FIELD`], because the archive's name is derived from
/// the version (`Server Setup` B4).
///
/// *(Corrected 2026-09-22, step 10c's third pass on Vast. This said the step
/// carries nothing, the desktop believed it, and every Blender install fetched
/// its archive and then stopped at the unpack with "the unpack was commanded
/// with no version to name its archive". Which steps read the version is now
/// [`THE_STEPS_THAT_READ_THE_BLENDER_VERSION`], one list both sides read.)*
pub const THE_UNPACK_STEP: &str = "the unpack";
/// **`Server Setup` R1 item 4's first read**: `ldd` over the executable, **the
/// whole not-found list at once**. Carries [`THE_EXECUTABLE_FIELD`] where the
/// subject is not this run's staged build.
pub const THE_LIBRARY_CHAIN_IS_READ_STEP: &str = "the library chain is read";
/// **`Server Setup` R1 item 4's re-read**, after the desktop's elevated
/// install, which **must come back clean**. Carries the same field on the same
/// terms.
pub const THE_LIBRARY_CHAIN_IS_READ_AGAIN_STEP: &str = "the library chain is read again";
/// **`Server Setup` R1 item 7, whole**: the staged build asked its version in
/// staging under `Server Setup` B1's discipline, required to answer
/// [`THE_BLENDER_VERSION_FIELD`], and renamed to its final folder **only on
/// that answer**.
///
/// **One step and not two.** `Server Setup` B7a splits the ask from the rename
/// so a below-floor answer can hold for a human — and `Server Setup` B7a's warning *"can arise
/// on this route only"*, the route being `Server Setup` B7's build the user
/// pointed at, which is not built here.
pub const THE_BUILD_IS_PUBLISHED_STEP: &str = "the build is published";
/// **`Server Setup` B15's removal: the plain deletion of the install's
/// folder** — the folder under `Server Setup` B9's Blender area that holds
/// [`THE_EXECUTABLE_FIELD`]. **Carries** that field, the answered version in
/// [`THE_BLENDER_VERSION_FIELD`], and `Server Setup` B14's covered jobs as
/// [`HOW_MANY_COVERED_JOBS_FIELD`] with each item [`A_COVERED_JOB`] — the one
/// set the warning named the user, composed once (`Queue & History` L21).
/// **A path outside the Blender area is refused**: the software manager's
/// route is not this step. Added 2026-10-10, the 38's piece 14c. *(From
/// 2026-10-16, step 10c's owed piece 6, the manager's route rides this step
/// too, where [`THE_MANAGERS_PACKAGE_FIELD`] is carried: the desktop has
/// performed the manager's removal, and the step reads that the build is gone
/// and deletes nothing.)*
pub const THE_BUILD_IS_REMOVED_STEP: &str = "the build is removed";
/// **How many covered jobs the removal step carries** — `Server Setup` B14's
/// set, each as the item [`A_COVERED_JOB`] with [`THE_JOB_FIELD`].
pub const HOW_MANY_COVERED_JOBS_FIELD: &str = "covered jobs";
/// The item word for one covered job.
pub const A_COVERED_JOB: &str = "covered job";
/// **`Server Setup` B15's second route: the package the machine's own software
/// manager holds the build under**, carried on [`THE_BUILD_IS_REMOVED_STEP`]
/// where the build is that manager's. **Present, the step deletes nothing**:
/// the manager's removal was the desktop's elevated act (`Server Setup` Y7),
/// and the step reads that the build is gone, refusing any package name, or
/// any path, outside `Server Setup — machine`'s fixed pairing (`Server Setup`
/// §13 item 21). Absent, the step is the plain deletion. Added 2026-10-16,
/// step 10c's owed piece 6.
pub const THE_MANAGERS_PACKAGE_FIELD: &str = "managed package";
/// **`Server Setup` R1 item 8's Telegram material.** **Refused by name**: see
/// [`EVERY_SETUP_STEP_THE_WORKER_IS_COMMANDED`].
pub const THE_TELEGRAM_MATERIAL_STEP: &str = "the telegram material";
/// **`Server Setup` R1 item 9's final verification**, before the state is
/// cleared.
pub const THE_FINAL_VERIFICATION_STEP: &str = "the final verification";

/// **Every worker step that reads [`THE_BLENDER_VERSION_FIELD`], and so every
/// step the desktop sends it with.** Added 2026-09-22.
///
/// **One list, read by both sides.** The desktop sends the version with exactly
/// these steps; the machine's own crate holds a test that a step refuses for
/// want of a version exactly when it is on this list, so neither side can gain
/// or lose a reader alone. The three: `Server Setup` B4's fetch derives its
/// address from the version, the unpack names its archive by it, and
/// `Server Setup` R1 item 7's publication requires the staged build to answer
/// it.
pub const THE_STEPS_THAT_READ_THE_BLENDER_VERSION: [&str; 3] = [
    THE_FETCH_STEP,
    THE_UNPACK_STEP,
    THE_BUILD_IS_PUBLISHED_STEP,
];

/// **`Server Setup` S5: the run that stopped is still running on the machine,
/// so a recovery of it is refused for now and not for good.** Added 2026-09-22.
///
/// It rides a refused run opening and carries the identity of the run that is
/// still running. **A desktop that reads it waits and asks again**, which is
/// `Server Setup` S5's *"costs a wait, never a second run"* and
/// `Design/Server Setup § The two ways out`'s *"either one waits if the server
/// has not finished with the run that stopped"*. A program placed before this
/// date never sends it, and its refusal is then shown in its own words.
pub const THE_RUN_THAT_STOPPED_IS_STILL_RUNNING_FIELD: &str = "the run that stopped is still running";

/// **Every step the setup worker is commanded, closed.**
///
/// **Two of the eight are refused by name and the reason is recorded rather
/// than hidden.** `Server Setup`'s own `driving::who_performs` gives both to
/// the setup worker, and neither can be performed there:
///
/// - [`THE_READINGS_STEP`] — `Server Setup` R1 item 1's readings are
///   `Servers` H11's table, and both command strings it needs
///   (the PCI vendor scan and the driver's own query) are declared in
///   `smoozy-servers`, a crate the machine's program may never reach. The
///   desktop takes them over the command channel and writes the boundary, as it
///   does for an elevated step.
/// - [`THE_TELEGRAM_MATERIAL_STEP`] — the material's placement is built on the
///   desktop (`Server Setup`'s own `acts::place_the_telegram_material`, inside
///   the authenticated connection, every path a protocol field), and
///   `Server Setup` G1's desired state is `Rented-server runtime` W8's, written
///   through the ordinary command path. `Server Setup`'s §7 entry on its own
///   acting-half list stands.
pub const EVERY_SETUP_STEP_THE_WORKER_IS_COMMANDED: [&str; 9] = [
    THE_READINGS_STEP,
    THE_FETCH_STEP,
    THE_UNPACK_STEP,
    THE_LIBRARY_CHAIN_IS_READ_STEP,
    THE_LIBRARY_CHAIN_IS_READ_AGAIN_STEP,
    THE_BUILD_IS_PUBLISHED_STEP,
    THE_BUILD_IS_REMOVED_STEP,
    THE_TELEGRAM_MATERIAL_STEP,
    THE_FINAL_VERIFICATION_STEP,
];

/// **The hold instruction's dependency: the job the submitted job is held
/// behind** — `Rendering` S13: *"The second job's submission carries the hold
/// instruction: in the same supervisor act that writes its job record, the
/// supervisor places the hold record — origin submission, subject the second
/// job's identity, reason the check, dependency the first job's identity with
/// the release predicate `Queue & History` H7 declares."*
///
/// **Added by step 10c piece 5**, settling the finding piece 6's builder made:
/// piece 1 named no field for the instruction, so a second job could only be
/// refused or sent unheld. **The subject is the request's own job and the
/// origin is the submission itself**, so neither is carried; the predicate is
/// `Queue & History`'s and the machine supplies it.
pub const THE_HELD_BEHIND_FIELD: &str = "held behind";

/// **The hold instruction's reason, in the placing document's own words** —
/// `Rented-server runtime` U2: *"the reason, from the closed set the placing
/// document declares; this document carries the reason and never interprets
/// it."* **It travels because the placing document is `Rendering`, whose words
/// are on the desktop**, and the machine writing a copy of them would be a
/// second spelling of one fact. **It rides with [`THE_HELD_BEHIND_FIELD`] and
/// never alone**: a request carrying one of the two is refused by name.
///
/// **Both hold fields also ride on a held job's item in the queue reading**
/// (`Queue & History` A1's *its unreleased hold*), read off the hold record
/// the machine placed (`Rented-server runtime` U2): the same two facts under
/// the same two names. `Queue & History` H1 and `Queue & History` H6 build the
/// held entry from them, which a Smoozy opened after the submission can do no
/// other way. **A reading from a program older than step 10c's eleventh-pass
/// fix piece 8b carries neither.** Added 2026-09-30.
pub const THE_HOLD_REASON_FIELD: &str = "hold reason";

// ------------------------------------------------------------ the requests

/// **`Rented-server runtime` G12's health answer, by name, and it has one home
/// now.**
///
/// It was written twice until step 10c's first piece — once in the machine's
/// own crate and once in the desktop's placement performer, with a test holding
/// the second to the first — because the machine's copy sits in a module gated
/// to a unix and the desktop runs on three systems. **Both sites now re-export
/// this one.** It carries no field: `Rented-server runtime` G12's four things
/// ride the result.
///
/// **C6: a pure reading.** Answered today.
pub const THE_HEALTH_ANSWER_REQUEST: &str = "the health answer";

/// **The submission: the job record and its per-job note written, in one act.**
///
/// `Rendering` S4's ordered unit ends with it — the remap material placed, the
/// job's material published, **then the job record written** — and
/// `Queue & History` Q3 fixes what the machine does with it: the job record and
/// the per-job note written by the supervisor in one act, then the event
/// journalled. `Queue & History` N1 creates the note *"with the label and a null
/// ending."*
///
/// **Carries** [`THE_JOB_FIELD`] always, [`THE_LABEL_FIELD`] where a label
/// was given, and **the hold instruction** — [`THE_HELD_BEHIND_FIELD`] and
/// [`THE_HOLD_REASON_FIELD`], both or neither — on the first-frame check's
/// second job (`Rendering` S13). **Chosen here, and nothing else rides it:**
///
/// - **No group identity.** `Job files and outputs` M1's manifest already holds
///   the job and group pair, and it is published before this request is sent,
///   so a group carried here would be a second copy of a fact the machine
///   already holds and could disagree with it.
/// - **No record directory.** `Job files and outputs` N3 names a job record
///   directory by the job identity's textual form alone, so the machine
///   composes it.
/// - **The label is carried** because no published material holds it whole —
///   the output folder's name holds only a bounded rendering of it, and that is
///   never read back.
///
/// **C6: recorded before it acts** — C6 names a submission among them.
/// Answered by step 10c piece 5. **Its desktop seam is `Rendering`'s, which
/// takes the job identity alone**; the runner's third ruling for step 10c has
/// one sender implement it and `Queue & History`'s together, and that sender
/// takes the label from the provisional record the other seam holds.
pub const THE_JOB_RECORD_IS_WRITTEN_REQUEST: &str = "the job record is written";

/// **The ask after a lost acknowledgement: does the machine hold this job?**
///
/// `State, identity and recovery` §12 rule 4 — *"A lost acknowledgement is
/// resolved by asking the machine for the identity, never by re-issuing the
/// work under a fresh one"* — and `State, identity and recovery` V1 step 2,
/// which asks it on every connection for each provisional local job record:
/// *"ask the machine for that job identity"*, and reads one of three things
/// back — present; absent and the submission refused or never taken; absent
/// and the machine cannot say. C9 is the same rule from this document's side.
///
/// **Carries** [`THE_JOB_FIELD`] and nothing else. **Chosen here**: the ask
/// names the job, as `State, identity and recovery` V1 step 2 does, and not
/// the submission's operation identity. The machine's command record for the
/// submission names the job it acts on as its subject
/// (`State, identity and recovery` §5.1), so the machine can answer C8's three
/// about that submission from the job identity alone, and the desktop seam
/// that asks — `Queue & History`'s — holds the job identity and no operation
/// identity.
///
/// **This is not C9's general ask.** A lost acknowledgement for anything other
/// than a job's submission is still asked about by operation identity, and
/// that request has no name yet.
///
/// **C6: a pure reading.** Answered by step 10c piece 5.
pub const THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST: &str = "does the machine hold this job";

/// **The queue reading** — `Queue & History` A1: *"one request, on demand,
/// after the installation identity confirms, and it returns the machine's
/// whole working state"*, with `Queue & History` R5's live figures in the
/// answer — the worker's readings the supervisor holds in memory, *"answers the
/// desktop's queue reading with them on request."*
///
/// **Carries nothing.** A1 asks for the whole working state, so there is
/// nothing to choose between, and the installation identity is confirmed by
/// the desktop before the request is sent rather than by a field in it.
///
/// **C6: a pure reading**, period-held on the desktop. Answered by step 10c
/// piece 5.
pub const THE_QUEUE_READING_REQUEST: &str = "the queue reading";

/// **One job's per-job note, read** — `Queue & History` N1: *"the read surface
/// an entry's lasting facts come from"*, which `State, identity and recovery`
/// V1 step 4 fills the local record in from.
///
/// **Carries** [`THE_JOB_FIELD`]. **Chosen here**: one job per request, the
/// smallest shape; a reader wanting several asks several times, one request per
/// connection being P1's rule anyway.
///
/// **C6: a pure reading.** Answered by step 10c piece 5.
pub const THE_PER_JOB_NOTE_READING_REQUEST: &str = "the per-job note reading";

/// **The read forward from a cursor** — `State, identity and recovery` V1 step 3,
/// *"Read forward from the event cursor"*, under that document's §4: the
/// journal reports its oldest retained sequence number with every read
/// (`State, identity and recovery` E5), and **a read advances nothing**
/// (`State, identity and recovery` E3).
///
/// **Carries** [`THE_CURSOR_FIELD`]: the first sequence number the reader has
/// not yet done the work for. **Chosen here, three things:**
///
/// - **No bound on how many.** The answer is bounded by what one frame can
///   carry, which is the answering side's own limit, and a reader that got
///   fewer than there are asks again from past the last one it got.
/// - **No reader named.** Over the connection the only reader is the desktop —
///   the Telegram worker's cursor lives on the machine
///   (`State, identity and recovery` E2) — so the answering side reads for the
///   desktop, and no field exists that could name another reader.
/// - **A reader with no cursor yet asks from zero**, which is before every
///   sequence number, so the answer starts at the oldest the journal retains.
///
/// **C6: a pure reading.** Answered by step 10c piece 5. **This is the absence
/// the step 10c reading's fifth carry-forward found**: the journal's read is
/// built on the machine and no request reached it.
pub const THE_READ_FORWARD_REQUEST: &str = "read forward from the cursor";

/// **The acknowledgement that moves the desktop's cursor** —
/// `State, identity and recovery` E3: *"A cursor advances after the work the
/// event asked for is done, never on receipt"*, and
/// `State, identity and recovery` V1 step 3 advances it
/// *"only after the application is durable."*
///
/// **Carries** [`THE_CURSOR_FIELD`]: every event below it has had its work
/// done. **No reader is named**, for the reason at [`THE_READ_FORWARD_REQUEST`]
/// — and here it is also `State, identity and recovery` E2's own rule, *"A
/// reader never advances another reader's cursor"*: a field that could name the
/// Telegram worker would let the desktop do exactly that.
///
/// **C6: recorded before it acts**, because it writes. A cursor never goes
/// backwards on the machine, so a repeated or late one changes nothing.
/// Answered by step 10c piece 5.
///
/// **Its command record ends at its close** — `Rented-server runtime` R1's own
/// row for this one command, written into the plan on 2026-09-19 because
/// `Rented-server runtime` R1's
/// general term ends a command record with its subject's machine-side record
/// and this command has no subject. **So a repeat after the close acts again
/// and moves nothing, and an ask after the close finds *no record of that
/// identity*** (the paragraph under C8). **A pending move resolved that way is
/// dropped by the desktop** — never read as a failure, never re-sent under a
/// fresh identity to make up for it; the next pass moves the cursor anyway.
pub const THE_CURSOR_MOVES_FORWARD_REQUEST: &str = "the cursor moves forward";

/// **A setup run opened**, which starts a setup worker — `Server Setup` Y7: the
/// run is driven from the desktop one step at a time, and its machine-side
/// steps are the setup worker's.
///
/// **Carries** [`THE_KIND_OF_SETUP_FIELD`], from [`EVERY_KIND_OF_SETUP`], **or**
/// [`THE_RUN_TAKEN_OVER_FIELD`] on a recovery run — **one or the other and
/// never both**. **The frame's own operation identity is the run's**: C4
/// records it in an operation record because the request starts a worker, and
/// that identity is what `Rented-server runtime` P1 calls the operation
/// identity of the setup run.
///
/// **Settled at step 10c piece 8**, which piece 1 left open:
///
/// - **The kind's wire words are [`EVERY_KIND_OF_SETUP`]**, `Server Setup`
///   `Server Setup` Y6b's five and no more.
/// - **A recovery run's take-over does need a field, and it is the only one it
///   needs.** `Server Setup` S4 item 2 has the recovery name the setup
///   operation the state names; its own kind follows from that run's record
///   (`Server Setup` Y6b), so carrying a kind beside it would be one fact in
///   two places.
/// - **A recovery may carry [`THE_RECOVERY_ABANDONS_FIELD`] beside it** —
///   `Server Setup` S6's abandon, added 2026-09-22 — and nothing else may.
///
/// **C6: recorded before it acts.** Answered by step 10c piece 8.
pub const THE_SETUP_RUN_IS_OPENED_REQUEST: &str = "a setup run is opened";

/// **One setup step commanded** — `Server Setup` Y7: *"Machine-side steps
/// (fetch, unpack, `ldd` reads, publication, read-backs) are performed by the
/// setup worker, commanded one step at a time; each runs to its boundary and
/// the worker then waits for the next command."*
///
/// **Carries** [`THE_SETUP_RUN_FIELD`] and [`THE_STEP_FIELD`], the step being
/// one of [`EVERY_SETUP_STEP_THE_WORKER_IS_COMMANDED`], **and the step's own
/// inputs where it has any**: [`THE_BLENDER_VERSION_FIELD`] on the fetch and
/// the publication, [`THE_EXECUTABLE_FIELD`] on either library step where the
/// subject is not this run's staged build.
///
/// **Chosen here**: the step is named rather than left as *the next one*, so a
/// worker and a desktop that disagree about where the run is find it out at
/// the command rather than by performing the wrong step.
///
/// **Settled at step 10c piece 8**, which piece 1 left open: **a step's own
/// inputs are the version and, on the library steps, the executable — and
/// nothing else, and no address anywhere.** `Server Setup` B4 derives the
/// archive's address and the checksum file's beside it from the version alone,
/// and that derivation is on the machine; see [`THE_BLENDER_VERSION_FIELD`] for
/// why sending an address would be worse than a second spelling.
///
/// **`Server Setup` Y7's refresh is not this request**: *"The refresh is not a
/// step command."* It is [`THE_SETUP_RUNS_STATE_REQUEST`] — settled there.
///
/// **It also carries [`WHAT_THE_STEP_DID_FIELD`], and only where the
/// `Server Setup` R1 step's own act was the desktop's. Added at step 10c.**
/// `Server Setup` Y4a's mark is the *step's*, and its test is the act — the
/// same test `Server Setup` Y1 classes a whole run by, applied per step.
/// **Two of `Server Setup` R1's steps have their act at the desk and their boundary fed
/// by the worker's half: the tooling step's install with the unpack, and the
/// libraries step's install with the `ldd` re-read (`Server Setup` Y7's two
/// granularities). **The worker cannot say whether such a step performed its
/// act, because it did not perform it**, and a *performed* mark on a libraries
/// step that installed nothing is exactly the false claim `Server Setup` Y4a
/// exists to stop. **Absent for every other step**, whose act is the worker's
/// own and whose mark the worker reports.
///
/// **C6: recorded before it acts.** Answered by step 10c piece 8.
pub const THE_ONE_SETUP_STEP_REQUEST: &str = "one setup step";

/// **The setup run's state, read** — `Server Setup` S1: *"the machine is read
/// for its setup state fresh, on the connection the user opens"*, and
/// `Server Setup` S2: *what it managed* is the machine's own record and has no
/// second home.
///
/// **Carries** [`THE_SETUP_RUN_FIELD`] and nothing else. **Chosen here**: the
/// run is named rather than asked for as *the machine's current run*; a desktop
/// that has none to name learns it from the health answer, which carries the
/// setup-in-progress state and its run (`Rented-server runtime` V9).
///
/// **And this request is `Server Setup` Y7's refresh, settled at step 10c piece
/// 8 rather than given a request of its own.** `Server Setup` Y7 forbids the refresh being a
/// *step* command and says nothing more; this is not one.
///
/// - **It already names the run**, which is all a refresh has to identify.
/// - **It is a pure reading, so it records nothing.** A refresh at
///   `Servers` K3's keepalive cadence that wrote a command record each time
///   would be the fault the cursor's move cost a plan fix for on 2026-09-19,
///   once per run rather than once per rental.
/// - **One request means a desktop cannot refresh without also learning where
///   the run has got to**, which is what the driving side wants anyway.
///
/// **So the worker's refresh clock is moved by the arrival of this reading
/// naming its run**, and `Server Setup`'s own `driving::the_window_has_elapsed`
/// measures the gap between two of them.
///
/// **C6: a pure reading.** Answered by step 10c piece 8.
///
/// **It may also carry what of the step in hand's words the desktop already
/// holds** — `Server Setup` Y7a item 2, as changed on 2026-10-24: *"the
/// reading request names those two lengths, and the reading answers with what
/// follows them"* — in [`THE_STEPS_WORDS_TURN_FIELD`],
/// [`THE_ORDINARY_WORDS_HELD_FIELD`] and [`THE_COMPLAINT_WORDS_HELD_FIELD`].
/// **A reading naming none, or another turn, is answered from the start of the
/// step's words**, which is what every desktop older than this asks. Added
/// 2026-10-25, the eighteenth notes, piece 2b.
pub const THE_SETUP_RUNS_STATE_REQUEST: &str = "the setup run's state";

/// **The Set up card's reading of the machine** — `Server Setup` Y6a,
/// `Server Setup` Y6b, `Server Setup` W3a and `Server Setup` W4: what the
/// *This server* card is composed from beside the run.
/// Added 2026-10-03, the 38's piece 6, for `Open Threads`' *The Set up card
/// says No setup read from this server after every reopen*: *"It needs a new
/// machine request."*
///
/// **Carries nothing.** The machine answers three things, each its own:
///
/// - **`Server Setup` Y6b's record: which run is the ready action's retained
///   one**, as [`THE_READY_ACTIONS_RUN_FIELD`], absent where the machine holds
///   none. **Only the identity**: the record itself is read by
///   [`THE_SETUP_RUNS_STATE_REQUEST`] naming it, so the record's fields have one
///   composition and one reader rather than two. A desktop that drove no run on
///   this machine — a copy opened after the setup — learns the run here and
///   nowhere else.
/// - **`Server Setup` W3a's three answers, never a listing**:
///   [`THE_JOBS_THAT_ENDED_HERE_FIELD`], [`THE_MOST_RECENT_JOB_TO_END_FIELD`],
///   and for the most recent job whose recorded outcome is *finished* and
///   which carries a device line, [`THE_FINISHED_JOB_WITH_A_DEVICE_LINE_FIELD`]
///   with [`THE_DEVICE_LINE_FIELD`] verbatim and, where it can be read,
///   [`THE_DEVICE_ASKED_FOR_FIELD`] — `Server Setup` W4's second condition.
///   **The line is never judged on the machine**; `Server Setup` W4's
///   conclusion is drawn
///   where the card is.
/// - **`Server Setup` Y6b's Telegram line: the role's own durable desired
///   state** (`Server Setup` G1), as [`THE_TELEGRAM_ROLES_STATE_FIELD`], absent
///   where it could not be read.
///
/// **What could not be read is left out and never answered as *none***: a
/// store that does not read leaves its part's fields off and says why in
/// [`THE_MACHINES_OWN_WORDS_FIELD`].
///
/// **C6: a pure reading.** Nothing is recorded, moved or started.
pub const THE_SET_UP_CARDS_READING_REQUEST: &str = "the Set up card's reading";

/// **`Server Setup` B8a's install records, across every setup run the machine
/// holds** — what the Blender table's tag is read from at connection. Added
/// 2026-10-09, the 38's piece 14b: until then an install record was read only
/// for one named run, and a desktop opened later remembers no run.
///
/// **Carries nothing.** Answers [`HOW_MANY_INSTALL_RECORDS_FIELD`], then each
/// record as the item [`AN_INSTALL_RECORD`] with
/// [`THE_INSTALLED_EXECUTABLE_FIELD`] and [`THE_INSTALL_ROUTE_FIELD`]. **One
/// per executable, the newest standing**, by the run's last boundary on the
/// machine's own clock. **A store that will not read says so** in
/// [`THE_MACHINES_OWN_WORDS_FIELD`] and carries no count, never an empty list.
///
/// **C6: a pure reading.** Nothing is recorded, moved or started.
pub const THE_INSTALL_RECORDS_REQUEST: &str = "the install records";
/// **How many install records [`THE_INSTALL_RECORDS_REQUEST`] answered.**
pub const HOW_MANY_INSTALL_RECORDS_FIELD: &str = "install records";
/// The item word for one of them.
pub const AN_INSTALL_RECORD: &str = "install record";

/// **The ready action's retained run** — `Server Setup` Y6b's record, by its
/// operation identity. Absent where the machine holds no record of that kind,
/// which is `Server Setup` Y6a's *never set up*.
pub const THE_READY_ACTIONS_RUN_FIELD: &str = "the ready action's run";
/// **`Server Setup` W3a's first question**: whether any job has ended on this
/// machine, as [`SOME_JOBS_HAVE_ENDED_HERE`] or [`NO_JOB_HAS_ENDED_HERE`].
/// Absent where the jobs could not be read.
pub const THE_JOBS_THAT_ENDED_HERE_FIELD: &str = "the jobs that ended here";
/// At least one job's ending is recorded on this machine.
pub const SOME_JOBS_HAVE_ENDED_HERE: &str = "some";
/// No job's ending is recorded on this machine.
pub const NO_JOB_HAS_ENDED_HERE: &str = "none";
/// **`Server Setup` W3a's second question**: the most recent job to end, by
/// the machine's own clock, as a job identity.
pub const THE_MOST_RECENT_JOB_TO_END_FIELD: &str = "the most recent job to end";
/// **`Server Setup` W3a's third question**: the most recent job whose recorded
/// outcome is *finished* and which carries a device line, as a job identity.
/// Absent where no such job is held.
pub const THE_FINISHED_JOB_WITH_A_DEVICE_LINE_FIELD: &str = "the finished job with a device line";
/// **`Server Setup` W4's second condition: what that job asked to render on**
/// — the user's own answer, else the job's scene's own setting as the reading
/// in force gave it, the same resolution the render worker made (`Render
/// Settings` D1). [`IT_ASKED_FOR_THE_CARDS`] or [`IT_ASKED_FOR_THE_PROCESSOR`];
/// absent where neither the payload nor a reading says.
pub const THE_DEVICE_ASKED_FOR_FIELD: &str = "the device asked for";
/// The job asked for the cards.
pub const IT_ASKED_FOR_THE_CARDS: &str = "the cards";
/// The job asked for the processor.
pub const IT_ASKED_FOR_THE_PROCESSOR: &str = "the processor";
/// **The Telegram role's durable desired state** (`Server Setup` G1), as
/// [`THE_ROLE_IS_ENABLED`] or [`THE_ROLE_IS_DISABLED`].
pub const THE_TELEGRAM_ROLES_STATE_FIELD: &str = "the Telegram role's desired state";
/// The role is enabled.
pub const THE_ROLE_IS_ENABLED: &str = "enabled";
/// The role is disabled, which is also what an absent state reads as
/// (`Rented-server runtime` W7).
pub const THE_ROLE_IS_DISABLED: &str = "disabled";

/// **One boundary of a setup run, written from the desktop** — the
/// elevated-step bullet of `Server Setup` Y7: *"the desktop records the step's
/// start against the run, and the boundary is written when the desktop's
/// verification read passes."*
///
/// **Why it exists.** `Server Setup` Y3 has the supervisor write a boundary
/// when the step it names has completed and its verification read has passed,
/// and `Server Setup` Y7 puts five of `Server Setup` Y4's ten on the desktop —
/// *driver placed*, *restart issued*, *card answering*, *telegram* and *done*.
/// **Until this request, the machine wrote a boundary only on its own worker's
/// report**, so those five could never be reached on any machine and no setup
/// run could complete its own list. Step 10c piece 9 found it and could not
/// close it; this is the route.
///
/// **Carries** [`THE_SETUP_RUN_FIELD`], [`THE_BOUNDARY_FIELD`] — one of
/// [`EVERY_SETUP_BOUNDARY`] — and [`WHAT_THE_STEP_DID_FIELD`], from
/// `Server Setup` Y4a's closed set of two. **The mark is required rather than
/// optional**, because `Server Setup` Y4a writes it *"in the same durable
/// act"* as the boundary and a boundary arriving without one could not be
/// written at all without inventing a third mark, which that clause forbids.
///
/// **The moment is not carried and never could be.** `Server Setup` Y3 puts the
/// boundary's moment on **the machine's own clock**, and
/// `State, identity and recovery` D5 forbids reading one clock into another —
/// so the machine takes the moment itself when it writes what it is told.
///
/// **Chosen here**: one boundary per request, named rather than meant as *the
/// next one*, for the reason [`THE_ONE_SETUP_STEP_REQUEST`] names — a desktop
/// and a machine that disagree about where a run stands find it out at the
/// request rather than by writing the wrong boundary.
///
/// **This gives the machine no way to write an elevated step's boundary
/// itself.** `Server Setup` Y7 puts that write on the desktop on purpose: the
/// machine cannot learn its connection ended in any useful time (Fact 57), so a
/// machine that wrote those boundaries would claim a step whose driver had gone.
///
/// **C6: recorded before it acts**, because it writes durably. A repeat of its
/// identity never writes again.
pub const THE_BOUNDARY_IS_WRITTEN_REQUEST: &str = "the boundary is written";

/// **`Rented-server runtime` P3's first act: the machine-level
/// setup-in-progress state is set, through the ordinary command path.**
///
/// *"`Server Setup` sets it and clears it through the ordinary command path, and
/// the supervisor never sets or clears it itself."* **So the setting is a
/// request and never something the machine decides**, and
/// `Rented-server runtime` P1 makes it name the operation identity of the setup
/// run that set it — which is [`THE_SETUP_RUN_FIELD`], the run's own.
///
/// **Who sends it, and when.** `Server Setup` Y1 fixes the class of a run **by
/// the act a step performs, never by the step's number**, and `Server Setup` Y7
/// puts every elevated act on the desktop. So the one side that can know an act
/// is about to be performed is the side about to perform it, and this is sent
/// immediately before the first such act.
///
/// **It moved here from the machine's own crate, for the reason
/// [`THE_SETUP_RUN_FIELD`] moved at step 10c piece 8**: that crate's acting half
/// is built for the machine alone and the desktop that sends this cannot reach
/// it at all. **It kept its path there**, so nothing on the machine's side
/// moved with it.
///
/// **C6: recorded before it acts**, because it writes durably.
pub const THE_SETUP_IN_PROGRESS_IS_SET_REQUEST: &str = "the setup in progress state is set";

/// **`Rented-server runtime` P3's second act, and P5's two in one: the state
/// goes, and reconciliation runs immediately afterwards.**
///
/// **`Server Setup` S6 closes the clearing set at two** — a run reaching
/// `Server Setup` Y4's *done*, and a recovery run the user asked to abandon the
/// machine-changing work — and `Rented-server runtime` P7 clears it on the
/// strength of nothing else. **The run names itself** ([`THE_SETUP_RUN_FIELD`]),
/// because `Rented-server runtime` P8 is decided on that name and P3 gives the
/// supervisor no clearing route that does not carry one.
///
/// **What it answers with says which happened**: a clear that landed carries
/// [`THE_SETUP_RUN_THE_CLEAR_REMOVED_FIELD`], a clear the standing state refused
/// carries [`THE_SETUP_RUN_FIELD`] naming the run the state holds, and an answer
/// carrying neither is trouble.
///
/// **C6: recorded before it acts.**
pub const THE_SETUP_IN_PROGRESS_IS_CLEARED_REQUEST: &str = "the setup in progress state is cleared";

/// **`Server Setup` Y8's stop, as `Rented-server runtime` X8's ordinary command
/// path carries it — and it is that path rather than a second one invented for
/// it.**
///
/// *"The user's stop is the durable desired state, honoured at a boundary and
/// never by a signal. The stop command writes *should be stopped* durably first
/// (`Rented-server runtime` X8 step 2); the worker checks it at each boundary
/// and ends there."*
///
/// **`Rented-server runtime` X7 fixes the shape**: the stop is its own command
/// with its own operation identity, naming the target's — which is
/// [`THE_SETUP_RUN_FIELD`], the setup run being stopped.
///
/// **What this kind does with `Rented-server runtime` X8's seven steps, and it
/// is `Rented-server runtime` X15's rule rather than a shortcut.** Step 2's
/// durable write is the whole of the act. **Step 4
/// issues no signal at all**, because `Server Setup` Y8 forbids one outright —
/// *"a setup step's children include the package system mid-transaction, and
/// killing that is how a machine gets into the state Row 114 exists to get it
/// out of"* — and `Rented-server runtime` X15 says termination for this kind is
/// *"at, or recovery from, a recorded safe boundary"*. **`Rented-server
/// runtime` X14 is the precedent**:
/// the transfer-support kind issues no signal at step 4 either, and its
/// sequence completes on its own cleanup's confirmation.
///
/// **Step 6's *left stopped* is recorded when the worker ends at its boundary**,
/// which is the next time it asks the supervisor what to do.
///
/// **Nothing is cleared by it.** `Server Setup` Y8b closes `Server Setup` S6's
/// clearing set at two and a stop is neither, so the machine-level state stands
/// and the interrupted job stays held back.
///
/// **C6: recorded before it acts**, because it writes durably.
pub const THE_SETUP_RUN_IS_STOPPED_REQUEST: &str = "a setup run is stopped";

/// **`Server Setup` S2a's desk half: a step the desktop held has failed, and
/// the run's recorded ending is written with the step named and its words
/// beside it — through `Rented-server runtime` X8's ordinary command path, the
/// same route that writes the stop's durable state.**
///
/// *"An elevated step's process is the connection's child, so its streams land
/// in the desktop's own loop and nowhere on the machine; the desktop, which
/// classifies the failure with those words in hand, writes the ending with the
/// words through the ordinary command path."* **Until this request nothing
/// did**, so a run whose elevated step failed was left open on the machine
/// until `Server Setup` Y7's window ended it as *ended at its boundary, not
/// completed*, with no words — and the card said *cut off* where
/// `Server Setup` S2a says *failed, with the words*.
///
/// **Carries** [`THE_SETUP_RUN_FIELD`], [`THE_FAILED_STEP_FIELD`] and, where
/// any were printed, [`THE_MACHINES_OWN_WORDS_FIELD`] — verbatim and whole.
/// **Where the failure was Smoozy's own verification rather than a tool's
/// refusal, the words are the recorded fact of that read**
/// (`Server Setup` S2a's third bullet), and nothing claims they are a tool's.
///
/// **What the machine does with it is write what it is told and nothing
/// more**: the ending *a step failed* and the words, in the same durable act
/// (`Server Setup` S2a's first bullet). **A run the machine does not hold
/// open is not written** — a run that already ended, by its own worker's failure, by the
/// stop, or by the window, keeps the ending it has, because
/// `Rented-server runtime` X8's recorded outcome takes no further disposition.
///
/// **The machine-level state is not touched.** `Server Setup` S6 closes the
/// clearing set at two and a failed step is neither.
///
/// **C6: recorded before it acts**, because it writes durably.
pub const THE_FAILED_STEP_IS_RECORDED_REQUEST: &str = "a failed step is recorded";

/// **An admin step's start, recorded against the run from the desktop** — the
/// elevated-step bullet of `Server Setup` Y7: *"the desktop records the step's
/// start against the run, and the boundary is written when the desktop's
/// verification read passes."* Added 2026-09-29, step 10c's transfer wave piece
/// 12 (the eighth-pass map's piece 7).
///
/// **Why it exists.** `Server Setup` S1's reading *"distinguishes a step never
/// started from a step that stopped part-way"* from *"the step that was in
/// flight"*, and **until this request the machine recorded a step as in flight
/// only when it commanded its own worker**. An admin step — a package install,
/// the driver's work — is the connection's child and is never the worker's, so
/// a connection cut during the driver install left a record naming no step in
/// flight, and every later reading said *Install the graphics driver — not
/// started* over a driver install that had been cut part-way. Step 10c's eighth
/// pass on Hyperstack saw exactly that panel.
///
/// **Carries** [`THE_SETUP_RUN_FIELD`] and [`THE_BOUNDARY_FIELD`], **the
/// boundary naming the step: the first of `Server Setup` Y4's boundaries the
/// step reaches**, as [`THE_FAILED_STEP_FIELD`] names a step. **Chosen here**:
/// no new field and no new set of words. `Server Setup` Y4's boundaries already
/// name every one of `Server Setup` R1's steps once, and a step spelled a third
/// way would be one more list to keep in step with the other two.
///
/// **What the machine does with it is write what it is told and nothing
/// more**: the step is written into the run's record as the step under way, in
/// one durable act, and it stays there until the next step is commanded or a
/// boundary is written. **It writes no boundary and ends nothing**, so a start
/// recorded for a step that then fails, stops or is cut changes nothing about
/// how the run ends — only which step the ending is read against.
///
/// **What it refuses**: a run the machine does not hold open, a step with a
/// worker step still in hand (`Server Setup` Y7's one step at a time), an
/// abandoned run's (`Server Setup` S6), and a step whose boundary is not in the
/// run's composed list, is already reached, or comes after one not yet reached
/// (`Server Setup` Y3).
///
/// **C6: recorded before it acts**, because it writes durably.
pub const THE_ELEVATED_STEPS_START_IS_RECORDED_REQUEST: &str = "an elevated step's start is recorded";

/// **`Rented-server runtime` G25's ending on a container, asked of the
/// machine's entry point after the pointer has moved: the entry point finds
/// whatever supervisor holds the machine-local endpoint and ends it if it runs
/// another program than the pointer names.** *(Corrected 2026-09-30: this said
/// the supervisor records the request and ends itself. Since transfer wave
/// piece 10, on 2026-09-28, the program's own adapter answers the request and
/// never passes it on.)*
///
/// `Rented-server runtime` G25's order is *"the running supervisor is ended
/// where there is one; the restarter of §3.2 starts a supervisor"*. **On a
/// systemd machine the restarter's own `restart` is both acts and nothing is
/// asked over this path.** **On a container there was no ending at all until
/// this request**: the keeper already running holds `Rented-server runtime`
/// S5's lock, so the keeper `Rented-server runtime` G25's start step launches
/// ends at once and starts nothing, and the supervisor the *old* pointer
/// started went on serving. Step 10c's third and
/// fourth passes on Vast both watched it, and on the fourth it cost a run — the
/// desktop asked the machine for a setup run's state and the answer came from a
/// program that knew nothing about it.
///
/// **Why the desktop asks rather than signals.** Nothing the desktop can send
/// over `Server protocol and transfers` §1.1 ends a process: a signal would
/// want the running supervisor's own process number in a command string, which
/// is machine-origin and inadmissible under §1.1. **So the desktop sends this
/// request through the constant entry-point command, and the signal is sent on
/// the machine.**
///
/// **What answers it is the adapter of the program the pointer now names**,
/// because the pointer has already moved when the desktop asks. That adapter
/// answers the request itself and asks no supervisor anything, so a supervisor
/// of any release — one that knows no such request, or speaks no protocol
/// this build speaks — can be ended. It reads from the kernel which process
/// holds the machine-local endpoint and which program file that process runs,
/// compares that file with the one the pointer names, and only where they
/// differ sends that one process the termination signal and waits up to 20
/// seconds for the endpoint to be let go. The keeper already running then
/// starts the one the pointer names.
///
/// **What comes back.** Where nothing held the endpoint, the holder already
/// runs the pointer's program, or the holder was ended, the answer carries
/// [`WHAT_BECAME_OF_IT_FIELD`] as [`IT_HAS_A_RECORDED_OUTCOME`], which is what
/// the desktop reads as an ending, although nothing is recorded. Where the
/// holder could not be identified, the signal could not be sent, or the holder
/// still held the endpoint when the wait ran out, that field is left off. The
/// machine's own words always say which case it was.
///
/// **Carries nothing.** The supervisor to end is whichever one holds the
/// machine-local endpoint, and naming it would be naming a thing the machine
/// already finds.
///
/// **A repeat ends nothing**, and not because of a record: by then the holder
/// runs the pointer's program, so the comparison leaves it alone. That is what
/// keeps a retried ending from killing the newly started program.
///
/// **C6's entry below still reads *recorded before it acts*.** That is the
/// route of an older release's adapter, which passes the request on to the
/// supervisor: the supervisor still records it before it acts and ends itself
/// after answering. This build's adapter records nothing.
pub const THE_SUPERVISOR_ENDS_ITSELF_REQUEST: &str = "the supervisor ends itself";

/// **`Rented-server runtime` W8's first act: the role's durable
/// enabled-or-disabled desired state, written to *enabled*.**
///
/// *"`Server Setup` sets that desired state through the same command path as
/// every other command, and never starts or stops the process."* **`Rented-server runtime` W9: a
/// change of it restarts nothing but the role worker.**
///
/// **`Server Setup` G1 makes it the third of `Telegram` §20's three things**,
/// and the one that is not a file: the token file and the role configuration are
/// placed inside the authenticated connection as protocol fields
/// (`Security and licence` §10), and this is a command.
///
/// **It moved here from the machine's own crate, for the reason
/// [`THE_SETUP_IN_PROGRESS_IS_SET_REQUEST`] moved**: that crate's acting half is
/// built for the machine alone, so a desktop that must build for Windows could
/// never name it, and `Server Setup` R1 item 8 is the desktop's own step.
/// **It kept its path there**, so nothing on the machine's side moved with it.
/// **This is the third time this same fault has been met.**
///
/// **Two names rather than one name with a value inside it**, which is
/// `Rented-server runtime` W8's own shape: a caller asking for something the
/// package does not serve is refused by name.
///
/// **C6: recorded before it acts**, because it writes durably.
pub const THE_TELEGRAM_ROLE_IS_ENABLED_REQUEST: &str = "the Telegram role is enabled";

/// **`Rented-server runtime` W8's other act, and that plan's `Rented-server
/// runtime` W10's half**: the supervisor never resurrects the role worker after
/// this.
///
/// **C6: recorded before it acts.**
pub const THE_TELEGRAM_ROLE_IS_DISABLED_REQUEST: &str = "the Telegram role is disabled";

/// What the supervisor did with the role worker after the state was written —
/// `Rented-server runtime` W9 and `Rented-server runtime` W10, reported to the
/// caller rather than left to be discovered.
pub const WHAT_THE_ROLE_WORKER_DID_FIELD: &str = "the role worker";

/// **`Rented-server runtime` §12's readings, asked for rather than pushed** —
/// `Rented-server runtime` V2: *"Every health reading is taken on request and never on a schedule"*.
///
/// **`Servers` W41a names the caller and it is the desktop**: one request per
/// primary connection, at that document's first-connection step 7, on the
/// confirmed branch only, where the supervisor is reachable.
/// `Server protocol and transfers` §1.1 puts it on the entry point that
/// *"carries the SSH command adapter, and with it readiness, **hardware
/// readings**, digest computation over a byte range"*.
///
/// **It moved here from the machine's own crate on 2026-09-21, for the reason
/// [`THE_SETUP_RUN_FIELD`] moved at step 10c piece 8**: that crate's acting half
/// is built for the machine alone, so the desktop — which must also build for
/// Windows — could not name it at all. **The desktop side had recorded that as
/// the reason `Servers` W41a's request is not made**, in as many words. **It kept its path
/// there**, so nothing on the machine's side moved with it.
///
/// **C6: a pure reading.** It changes nothing on the machine, and
/// `Rented-server runtime` V5 forbids
/// anything durable being written from a health reading.
pub const THE_HEALTH_READINGS_REQUEST: &str = "the health readings";

/// **`Rented-server runtime` V6's count**, as one field of the answer: the
/// number of rows the driver's own per-card query answered with, in decimal.
///
/// **It is a count and never a parsed total** — `Rented-server runtime` V6's
/// named trap is a caller
/// parsing the whole output as an integer and reading `22` off a two-card
/// machine.
pub const THE_CARD_COUNT_FIELD: &str = "cards";

/// **`Rented-server runtime` V13's second half**: the machine's own running
/// time in whole seconds.
///
/// **The field is absent where the reading would not be the machine's own**,
/// which is `Rented-server runtime` V13's ruled conduct — on a container the
/// figure is the host's and
/// would lie, so the line says it is unavailable rather than showing the
/// host's. **An absent field is the stated absence and never a zero.**
pub const THE_RUNNING_TIME_FIELD: &str = "running time";

/// **`Rented-server runtime` V9's field: the machine-level setup-in-progress
/// state, reported with every health answer**, *"so the state is learned on the
/// first request rather than discovered by a machine doing nothing"*.
///
/// **The value is the operation identity of the setup run that set it, and the
/// field is absent where no setup is in progress** — `Rented-server runtime` P1
/// makes the identity part
/// of the state rather than a decoration on it.
pub const THE_SETUP_IN_PROGRESS_FIELD: &str = "setup in progress";

/// **`Rented-server runtime` U9's command: an answer to a hold, with its own
/// operation identity, naming the hold identity and carrying the answer.**
///
/// **`Rented-server runtime` U10 is why the desktop must name it**: *"Both routes arrive at the same
/// command path, and there is no second one. The desk's answer arrives over SSH
/// through the command adapter and the machine-local endpoint"*; the phone's
/// arrives at the Telegram role worker, which submits to that same path.
///
/// **It moved here from the machine's own crate on 2026-09-21**, with §12's
/// readings above and for the same reason: the desk is one of its two callers
/// and could not reach that crate at all. **It kept its path there**, so the
/// machine's own dispatch and everything that calls it are untouched.
///
/// **C6: recorded before it acts** — `Rented-server runtime` U9 puts it under
/// C4 like any other
/// command, the identity recorded durably before the act and a repeat never
/// acting again.
pub const THE_ANSWER_TO_A_HOLD_REQUEST: &str = "the answer to a hold";

/// The hold identity the answer names (`Rented-server runtime` U9).
pub const THE_HOLD_FIELD: &str = "hold";
/// The answer itself, and its two values below.
pub const THE_ANSWER_FIELD: &str = "answer";
/// **`Rented-server runtime` U12's release.**
pub const THE_ANSWER_IS_YES: &str = "yes";
/// **`Rented-server runtime` U15: a *no* is not a stop.** It writes *stopped*
/// as the release outcome and converts nothing.
///
/// **`Rented-server runtime` U12a: and it removes its subject, in the same
/// act.** The result then carries [`THE_CANCELS_OUTCOME_FIELD`] as
/// [`IT_WAS_REMOVED`] — the cancel's own field and word, since *"Cancelling it
/// and saying no are one act, not two"* (`Specs/Queue & History`) — or, where
/// the job was not removed, [`THE_MACHINES_OWN_WORDS_FIELD`] saying why. **A
/// program older than step 10c's eleventh-pass fix piece 8b carries neither**,
/// and removed nothing; the desk tells the two apart by that and not by a
/// version.
pub const THE_ANSWER_IS_NO: &str = "no";
/// **`Rented-server runtime` U10's route**, carried because the record's release
/// says what produced it and the two routes differ in how they reached the
/// machine and in nothing after that.
///
/// **The field and its closed set of two are moved together and kept together.**
/// `Rented-server runtime` U10's *"there is no second one"* is only checkable where all three sit in one
/// place, and a closed set split across two crates is the fact-in-two-places
/// shape this stage keeps being bitten by.
pub const THE_ROUTE_FIELD: &str = "route";
/// The desk's route, over SSH through the command adapter.
pub const THE_ROUTE_IS_THE_DESK: &str = "the desk";
/// The phone's route, submitted by the role worker to the same command path.
pub const THE_ROUTE_IS_THE_PHONE: &str = "the phone";

/// **`Rented-server runtime` U14's result: the hold's recorded release outcome,
/// which answer was applied and the moment on the machine's clock.**
///
/// **A second answer is not an error.** It is C8's *recorded outcome* answer,
/// one of the three closed answers a machine gives about an identity — so the
/// one that lost is told what happened rather than that it failed.
pub const THE_RELEASE_OUTCOME_FIELD: &str = "release";
/// What produced the outcome — the answer, or the dependency's ending
/// (`Rented-server runtime` U20).
pub const THE_PRODUCED_BY_FIELD: &str = "produced by";
/// **`Rented-server runtime` U14's second half: the moment on the machine's
/// clock.** How it is composed is the machine's own and stays there.
pub const THE_RELEASE_MOMENT_FIELD: &str = "release moment";

// ---- One reading of a `.blend`, for both readings. Added 2026-09-29, step
// 10c's dependency wave piece 2.

/// **One reading of a `.blend` on the machine, by a file-inspection worker** —
/// `Render Settings` Q4: *"the read it starts is one file-inspection operation
/// driven from the desktop through the supervisor's command path"*, admitted
/// under the declarations of `Render Settings` W1; and `Dependency Upload & Remap` G1: the check's
/// reading is *"driven as a file-inspection worker through the supervisor"*.
/// **One request for both**, because `Rented-server runtime` W1 houses both in
/// one kind and their declarations agree value for value (`Render Settings`
/// W5); [`THE_WHICH_READING_FIELD`] says which.
///
/// **Carries** [`THE_WHICH_READING_FIELD`], [`THE_BUILD_FIELD`],
/// [`THE_BLEND_FIELD`] and [`THE_BLENDER_VERSION_FIELD`]. **The frame's own
/// identity is the reading's operation identity** — `Dependency Upload &
/// Remap` G15, *"each reading is one operation with its own operation
/// identity"*, issued before the request leaves (C1).
///
/// **Its answer is C8's, and a repeat of its identity is how the desktop learns
/// the reading ended. Chosen here.** The supervisor answers one caller at a
/// time, so it cannot hold this request open while a Blender reads a file: it
/// starts the worker and answers *still in hand* at once — the shape
/// [`THE_ONE_SETUP_STEP_REQUEST`] already has. **C4 makes a repeat of the
/// identity never act again**, so the same request asked again reads where the
/// reading stands: *still in hand* while the worker runs, and *a recorded
/// outcome* with the fields below once it has reported. **A second request
/// name for that would be a reading of one operation's state beside the
/// operation's own request, and the two could disagree.**
///
/// **What comes back is where the document is, never the document. The
/// runner's ruling of 2026-09-28 on how a reading's document comes back.** A reading's document can be larger than
/// [`crate::framing::THE_LARGEST_FRAME`] — the 5.2 splash scene's is 272,295
/// bytes for 250 frames, so an animation of about 950 frames passes one
/// frame's ceiling — and a document carried inside the answer would make every
/// long animation unreadable. **So the answer says where the worker's captured
/// ordinary and complaints streams sit and how long each is, the exit status,
/// whether the reading's start mark was seen, and whether Smoozy ended the
/// run**, and the desktop reads the streams back over the file session and
/// finds the document between the marks itself. `Render Settings` W2 item 3
/// keeps a reading's captured streams *"under the ordinary rules"*, so they are
/// on the machine's disk anyway.
///
/// **C6: recorded before it acts** — the operation record `Rented-server
/// runtime` W3 has written before any worker starts is that record.
pub const THE_READING_OF_A_BLEND_REQUEST: &str = "a reading of a .blend";

/// **Which of the two readings is asked for**: one of [`EVERY_READING`].
pub const THE_WHICH_READING_FIELD: &str = "which reading";
/// **`Render Settings` Q4's read, the file's reading**: `Job files and outputs`
/// R1's one document, whose script `Job files and outputs` R3 puts on both
/// sides, launched the way the render launches (`Job files and outputs` R7).
pub const THE_FILES_READING: &str = "the file's reading";
/// `Render Settings` Q4: optional saved library pairs for the reading's opening folder.
pub const THE_READINGS_LIBRARY_PAIRS_FIELD: &str = "the reading's library pairs";
/// **`Dependency Upload & Remap` G1's reading, the check's reading**, run with
/// that module's own launch line.
pub const THE_CHECKS_READING: &str = "the check's reading";
/// The two, closed.
pub const EVERY_READING: [&str; 2] = [THE_FILES_READING, THE_CHECKS_READING];

/// **The Blender build to start, by its path on the machine** — `Server Setup`
/// B13: a version alone cannot identify a build. **A protocol field and never a
/// command argument** (`Security and licence` §10), which is what lets the
/// worker start Blender by the build's own path rather than through a link.
pub const THE_BUILD_FIELD: &str = "build";
/// **The `.blend` to read, by its path on the machine**, the same way.
pub const THE_BLEND_FIELD: &str = "blend";

/// **Where the reading's captured ordinary stream sits on the machine**, on a
/// recorded outcome — `Rented-server runtime` M1's sink, named in the
/// operation record before the worker started. **The queue reading carries the
/// same two facts for a rendering job's sinks** (`Queue & History` E8), with the
/// bytes standing in each when the answer was composed. Widened 2026-10-03,
/// the Creator's notes on 0.1.8, piece 6a.
pub const THE_ORDINARY_STREAM_FIELD: &str = "ordinary stream";
/// **How many bytes of it were retained**, in decimal.
pub const THE_ORDINARY_STREAM_LENGTH_FIELD: &str = "ordinary stream length";
/// Where the captured complaints stream sits.
pub const THE_COMPLAINTS_STREAM_FIELD: &str = "complaints stream";
/// How many bytes of it were retained.
pub const THE_COMPLAINTS_STREAM_LENGTH_FIELD: &str = "complaints stream length";
/// **Whether the reading's own start mark was seen on either stream** — `Render
/// Settings` Q8: *"the second from the third by the start sentinel, which is
/// Smoozy's own frame"*. [`THE_ANSWER_IS_YES`] or [`THE_ANSWER_IS_NO`].
pub const THE_START_MARK_SEEN_FIELD: &str = "start mark seen";
/// **Whether Smoozy ended the run itself** — `Render Settings` Q8: *"a read
/// Smoozy ended itself is not diagnosed as any of the three"*.
/// [`THE_ANSWER_IS_YES`] or [`THE_ANSWER_IS_NO`].
pub const THE_RUN_WAS_ENDED_BY_SMOOZY_FIELD: &str = "ended by Smoozy";

// ------------------------------------------------------------ the queue's own acts
//
// **Named 2026-09-29, step 10c's first stop-and-clear piece**, which is the piece that answers them.
// Each was spelled until then as `queue.<something>` in `Queue & History —
// machine`'s own `AQueueCommand::name`, which no program answered; that
// function now reads these.

/// **`Queue & History` L13 and `Queue & History` L14: the clear, the first half of
/// stop-and-clear.** It removes every job record on the machine that is not the
/// running one — waiting and held alike — or, under `Queue & History` L19's
/// group form, every such record of one group.
///
/// **Carries** [`THE_PAIRED_STOP_FIELD`], always; [`THE_GROUP_FIELD`] where the
/// clear is `Queue & History` L19's; and [`THE_RENDER_THE_DESKTOP_LAST_SAW_FIELD`]
/// where the desktop's reading showed a render. **The frame's own identity is
/// the clear's operation identity.**
///
/// **Why the clear carries its stop's identity. Chosen here, and it is what
/// `Queue & History` L13's *a connection dropping mid-way changes nothing*
/// needs.** The two halves are two requests, and a line that drops between
/// them would otherwise leave the machine knowing of a clear and never of the
/// stop. **So each half names the other**, and whichever arrives first has the
/// machine record both before it acts; the second to arrive is then a repeat
/// of an identity already recorded and is answered from the record (C4).
///
/// **C6: recorded before it acts.**
pub const THE_CLEAR_REQUEST: &str = "the line is cleared";

/// **`Queue & History` L13's second half: the stop paired with a clear.**
///
/// **It names no target** — `Queue & History` L13: the desktop cannot name a
/// render that is still true when the clear lands, so the machine resolves the
/// stop's subject at the clear's write. **Carries** [`THE_PAIRED_CLEAR_FIELD`],
/// always, and the same [`THE_GROUP_FIELD`] and
/// [`THE_RENDER_THE_DESKTOP_LAST_SAW_FIELD`] its clear carries, for the reason
/// the clear's own constant gives.
///
/// **C6: recorded before it acts.**
pub const THE_STOP_PAIRED_WITH_A_CLEAR_REQUEST: &str = "the stop paired with a clear";

/// **C21: stopping work the machine has taken, its own command with its own
/// operation identity, naming the target's** — the queue's own *Stop*
/// (`Queue & History` L10) and the phone's (`Rendering` K5).
///
/// **Carries** [`THE_OPERATION_TO_STOP_FIELD`]. The sequence is
/// `Rented-server runtime` X8's and the render kind's own half is `Rendering`
/// K5; **what this machine stops is a render operation, or a file-inspection
/// operation it started** — the check's own *Stop* reaching a reading already
/// on the machine (`Dependency Upload & Remap` G15, with `Dependency Upload &
/// Remap` O1 the termination contract; added 2026-10-04, the 38's piece 7). **A stop
/// naming any other operation is refused before it is recorded.**
///
/// **C6: recorded before it acts.**
pub const THE_STOP_REQUEST: &str = "work the machine has taken is stopped";

/// **`Queue & History` L1 and `Queue & History` L5: a cancel of a waiting or held job.**
///
/// **Carries** [`THE_JOB_FIELD`]. A named job that is rendering is not
/// cancelled — `Queue & History` L4 makes that a stop — and one that has ended
/// is not cancelled either; the answer says which and why.
///
/// **C6: recorded before it acts.**
pub const THE_CANCEL_REQUEST: &str = "a waiting job is cancelled";

/// **`Queue & History` L6: a reorder** — one waiting job moved to a position
/// in its machine's line. **Answered by the machine from 2026-10-04, the 38's
/// piece 8**, which is also when the desktop first sent one; until then the
/// machine refused it by name.
///
/// **Carries** [`THE_JOB_FIELD`] and [`THE_POSITION_FIELD`], with
/// [`THE_JOB_IT_GOES_AFTER_FIELD`] where the position is [`AFTER_A_JOB`] —
/// `Queue & History` L6's *"after job X or at the front of the waiting line"*,
/// never an index. **The answer is the machine's**: a job that is no longer
/// waiting when it arrives is not moved, and a named predecessor no longer
/// waiting resolves to the front (`Queue & History` L6, `Queue & History` L8).
///
/// **C6: recorded before it acts.**
pub const THE_REORDER_REQUEST: &str = "the line is reordered";

/// **`Queue & History` L7: a group moved as one block.** **Carries**
/// [`THE_GROUP_FIELD`] and the same position as [`THE_REORDER_REQUEST`]. Answered
/// from 2026-10-04, the 38's piece 8.
///
/// **C6: recorded before it acts.**
pub const THE_GROUP_BLOCK_MOVE_REQUEST: &str = "a group is moved as one block";

/// **Where a reorder or a block move puts what it moves** — one of
/// [`AT_THE_FRONT_OF_THE_WAITING_LINE`] and [`AFTER_A_JOB`]
/// (`Queue & History` L6). Build's spelling, the 38's piece 8: two words rather than one field
/// that is sometimes an identity, so a position that did not arrive is never
/// read as *the front*.
pub const THE_POSITION_FIELD: &str = "position";
/// The front of the machine's waiting line.
pub const AT_THE_FRONT_OF_THE_WAITING_LINE: &str = "at the front of the waiting line";
/// Directly after the job [`THE_JOB_IT_GOES_AFTER_FIELD`] names.
pub const AFTER_A_JOB: &str = "after a job";
/// **The job a reorder or a block move goes after**, carried only with
/// [`AFTER_A_JOB`].
pub const THE_JOB_IT_GOES_AFTER_FIELD: &str = "after job";

/// **What a reorder or a block move came to**, on a recorded outcome:
/// [`IT_WAS_MOVED`] or [`IT_WAS_NOT_MOVED`], with the machine's own words
/// beside it naming what was not moved and why (`Queue & History` L6,
/// `Queue & History` L7). A move refused part-way is neither: it is *still in
/// hand*, its record standing until the supervisor finishes it
/// (`Queue & History` L24, `Queue & History` D14).
pub const THE_MOVES_OUTCOME_FIELD: &str = "what the move came to";
/// The keys were written: the job, or the group's waiting members as one block,
/// stand where the position put them.
pub const IT_WAS_MOVED: &str = "moved";
/// Nothing was moved — the named job, or every member of the group, was not
/// waiting when the move arrived.
pub const IT_WAS_NOT_MOVED: &str = "not moved";

/// **The stop a clear is paired with**, on the clear (`Queue & History` L13's
/// *marked as one ordered pair*).
pub const THE_PAIRED_STOP_FIELD: &str = "the paired stop";
/// **The clear a stop is paired with**, on the stop.
pub const THE_PAIRED_CLEAR_FIELD: &str = "the paired clear";
/// **The group identity** — `Queue & History` L19's clear scoped to one group,
/// and `Queue & History` L7's block move.
pub const THE_GROUP_FIELD: &str = "group";
/// **The render operation the desktop's period-held reading last showed**, where
/// it showed one. **Never a target** — `Queue & History` L13 forbids naming
/// one — and read only so that *where the resolved operation is not the one the
/// desktop's reading last showed, the outcome names both*.
pub const THE_RENDER_THE_DESKTOP_LAST_SAW_FIELD: &str = "the render the desktop last saw";
/// **C21's target: the operation identity of the work to stop.**
pub const THE_OPERATION_TO_STOP_FIELD: &str = "the operation to stop";

/// **What a clear came to**, on a recorded outcome: [`IT_WAS_CLEARED`] or
/// [`IT_WAS_NOT_CLEARED`]. **A clear refused part-way is neither**: it is *still
/// in hand*, its record standing until the supervisor finishes it
/// (`Queue & History` L23, `Queue & History` D14).
pub const THE_CLEARS_OUTCOME_FIELD: &str = "what the clear came to";
/// Every job record the clear covered is gone.
pub const IT_WAS_CLEARED: &str = "cleared";
/// **The clear's own record could not be written, so nothing was removed** —
/// `Queue & History` L16's refused write.
pub const IT_WAS_NOT_CLEARED: &str = "not cleared";

/// **What a stop came to**, on a recorded outcome — one of
/// [`IT_WAS_LEFT_STOPPED`], [`ITS_OWN_ENDING`], [`NOTHING_WAS_RENDERING`],
/// [`THE_RENDER_IS_NOT_A_MEMBER`], [`THE_STOP_WAS_NOT_ATTEMPTED`] and
/// [`THE_STOP_FAILED`]. **The subject it resolved to rides beside it** in
/// [`THE_RENDER_OPERATION_FIELD`].
pub const THE_STOPS_OUTCOME_FIELD: &str = "what the stop came to";
/// **`State, identity and recovery` §5.3 step 7**: the render ended on its own
/// before the interrupt was issued, and its ending is its own.
pub const ITS_OWN_ENDING: &str = "its own ending";
/// **`State, identity and recovery` §5.3 step 1**: *a rule that resolves to
/// nothing is a recorded outcome of nothing was rendering*.
pub const NOTHING_WAS_RENDERING: &str = "nothing was rendering";
/// **`Queue & History` L19**: the operation running at the group clear's write
/// is not a member, so no stop is attempted.
pub const THE_RENDER_IS_NOT_A_MEMBER: &str = "the render is not a member of the group";
/// **`Queue & History` L16**: the clear's write was refused, so the stop was
/// not attempted, naming the clear.
pub const THE_STOP_WAS_NOT_ATTEMPTED: &str = "not attempted, because the clear was not done";
/// **A durable write the stop needs was refused**, with the machine's own
/// words beside it — `Rented-server runtime` X8's *the sequence ends where it
/// stands*.
pub const THE_STOP_FAILED: &str = "the stop failed";

/// **What a cancel came to**, on a recorded outcome: [`IT_WAS_REMOVED`], which
/// is `Queue & History` L2's *removed*, or [`IT_WAS_NOT_CANCELLED`] with the
/// machine's own words saying why.
pub const THE_CANCELS_OUTCOME_FIELD: &str = "what the cancel came to";
/// The job record is gone.
pub const IT_WAS_REMOVED: &str = "removed";
/// `Queue & History` L1 and `Queue & History` L4: the named job was not waiting or held.
pub const IT_WAS_NOT_CANCELLED: &str = "not cancelled";

// ------------------------------------------------------------ the copy on the machine
//
// **Named 2026-10-02, step 10c's Files' owed acts piece 7**, which is the piece
// that answers it.

/// **`Server protocol and transfers` §1.1's request *a file is copied on the
/// machine*** — `File Browser` M7's route where the file service does not
/// announce copy-data: the machine copies the source to the staging path, makes
/// it durable, and answers. **Publication stays the desktop's**, by `File
/// Browser`'s own rename, so the staging, the collision answers and the writes
/// of `File Browser` X7a and `File Browser` W11 are the same on either route.
///
/// **Carries** [`THE_COPYS_SOURCE_FIELD`] and [`THE_COPYS_STAGING_PATH_FIELD`],
/// both paths on the machine as protocol fields and never words of a command
/// (`Server protocol and transfers` §1.1's rule).
///
/// **Answered by the entry point in the invocation that carries it, and never
/// passed to the supervisor** — the first exception to P1's passing, because the
/// supervisor serves its callers one after another and a copy of many
/// gigabytes there would hold every other request behind it. **A package that
/// does not know it passes it on as every other name, and its supervisor
/// refuses it by name**, which is how the desktop learns that the program on
/// that server is older than the request.
///
/// **What comes back**: [`THE_COPYS_OUTCOME_FIELD`], with
/// [`THE_MACHINES_OWN_WORDS_FIELD`] beside a copy that did not land.
///
/// **C6: neither** — [`UnderC6::OutsideC6`] says why.
pub const THE_FILE_IS_COPIED_ON_THE_MACHINE_REQUEST: &str = "a file is copied on the machine";

/// **The file to copy, by its path on the machine.**
pub const THE_COPYS_SOURCE_FIELD: &str = "the copy's source";
/// **Where the copy is written, by its path on the machine** — `File Browser`
/// M7's staging path, synthesised by the desktop in Smoozy's staging folder
/// beside the destination. **Created exclusively**: a path already taken is
/// written over by nothing.
pub const THE_COPYS_STAGING_PATH_FIELD: &str = "the copy's staging path";

/// **What the copy came to**: [`IT_WAS_COPIED`], or [`IT_WAS_NOT_COPIED`] with
/// the machine's own words beside it.
pub const THE_COPYS_OUTCOME_FIELD: &str = "what the copy came to";
/// The whole file sits at the staging path and is on the disk.
pub const IT_WAS_COPIED: &str = "copied";
/// **Nothing was left at the staging path by this request** — the source
/// could not be read, the staging path could not be made, or the bytes or the
/// flush to disk failed, and what this request had made there was removed.
pub const IT_WAS_NOT_COPIED: &str = "not copied";

// ------------------------------------------------ the check's two machine acts
//
// **Named 2026-10-04, the 38's piece 7**, when `Dependency Upload & Remap` O2
// was settled: the contents reading and the fixing run as file-inspection
// workers, each admitted under its own row of that document's §9.

/// **§1.1 item 1's *digest computation over a byte range*: `Dependency Upload
/// & Remap` Q3's contents reading, the digest of a file on the machine
/// computed by Smoozy's own program** — never by a tool found on the machine.
/// **Housed as a file-inspection operation** (`Dependency Upload & Remap` O2,
/// `Rented-server runtime` W1), whose group is the worker alone: no Blender.
///
/// **Carries** [`THE_DIGESTED_FILE_FIELD`], [`THE_RANGE_STARTS_AT_FIELD`] and,
/// where the range does not run to the end of the file,
/// [`THE_RANGE_LENGTH_FIELD`]. **The frame's own identity is the operation's.**
///
/// **Its answer is C8's, and a repeat of its identity is how the desktop learns
/// it ended**, as [`THE_READING_OF_A_BLEND_REQUEST`]'s is: *still in hand* at
/// the first ask, and *a recorded outcome* with [`THE_DIGESTS_OUTCOME_FIELD`],
/// and where something was digested [`THE_DIGEST_FIELD`] and
/// [`THE_BYTES_DIGESTED_FIELD`], once the worker has reported.
///
/// **C6: recorded before it acts.** C6 names a digest over a byte range among
/// its readings, and it changes nothing on the machine; **it is recorded first
/// all the same because it is housed as an operation** (`Rented-server
/// runtime` W3: every worker is started against a record written before it),
/// and that record is what a repeat of its identity is answered from.
pub const THE_DIGEST_OVER_A_BYTE_RANGE_REQUEST: &str = "a digest over a byte range";

/// **The file to digest, by its path on the machine** — a protocol field and
/// never a word of a command.
pub const THE_DIGESTED_FILE_FIELD: &str = "the file digested";
/// **Where the range starts**, a byte offset in decimal.
pub const THE_RANGE_STARTS_AT_FIELD: &str = "range starts at";
/// **How many bytes the range holds**, in decimal. **Absent: to the end of the
/// file.**
pub const THE_RANGE_LENGTH_FIELD: &str = "range length";
/// **What the digest came to**: [`IT_WAS_DIGESTED`], [`NOTHING_IS_THERE`] or
/// [`IT_WAS_NOT_DIGESTED`], with [`THE_MACHINES_OWN_WORDS_FIELD`] beside the
/// last.
pub const THE_DIGESTS_OUTCOME_FIELD: &str = "what the digest came to";
/// The range was read through and digested.
pub const IT_WAS_DIGESTED: &str = "digested";
/// **The system said no such file** — and only that. `Dependency Upload &
/// Remap` Q7 turns it into *not found*, so no other failure may answer it.
pub const NOTHING_IS_THERE: &str = "nothing there";
/// **Anything else** — a refusal, a read cut short, a range past the end. *A
/// digest that did not complete is nothing* (`Dependency Upload & Remap` §9).
pub const IT_WAS_NOT_DIGESTED: &str = "not digested";
/// **The digest, as the sixty-four hexadecimal digits of `Server protocol and
/// transfers` S23's function** — the written form the desktop's own digest
/// reads back.
pub const THE_DIGEST_FIELD: &str = "digest";
/// **How many bytes were digested**, in decimal — the range's own length, or
/// the file's from the start where the range ran to its end.
pub const THE_BYTES_DIGESTED_FIELD: &str = "bytes digested";

/// **`Dependency Upload & Remap` M14: one fixed copy of a linked `.blend`,
/// produced on the machine by a worker driving the job's Blender.** Housed as
/// a file-inspection operation (`Dependency Upload & Remap` O2); its
/// termination contract is `Dependency Upload & Remap` O1.
///
/// **Carries** [`THE_BUILD_FIELD`], [`THE_BLEND_FIELD`] (the machine copy the
/// fixed copy is derived from), [`THE_BLENDER_VERSION_FIELD`],
/// [`THE_FIXINGS_STAGING_FIELD`] and [`THE_FIXINGS_PAIRS_FIELD`]. **The worker
/// writes the copy to the staging path and no further**: the publication, one
/// rename, is the desktop's, after it has read the fixing's own report back.
///
/// **Its answer is [`THE_READING_OF_A_BLEND_REQUEST`]'s, field for field**:
/// where the two streams sit and how long each is, the exit status, whether
/// the fixing's own start mark was seen and whether Smoozy ended the run.
///
/// **C6: recorded before it acts.**
pub const THE_FIXED_COPY_IS_PRODUCED_REQUEST: &str = "a fixed copy is produced";

/// **Where the fixing saves its copy**, by its path on the machine — beside
/// the fixed copy's address, so the publication's rename does not cross a
/// mount.
pub const THE_FIXINGS_STAGING_FIELD: &str = "the fixing's staging path";
/// **The pairs the fixing writes**, as the one JSON document `Dependency
/// Upload & Remap` M10's material carries its own in.
pub const THE_FIXINGS_PAIRS_FIELD: &str = "the fixing's pairs";

// ------------------------------------------------------------ the erase
//
// **Named 2026-10-08, the 38's piece 16**, which is the piece that answers it
// and the first that sends it.

/// **`Queue & History` X2 and `Queue & History` X3: the owed erase of a deleted job's record on
/// the machine** — the machine's history copy, the per-job note, the queue
/// record, the hold record naming it, the operation and command records, the
/// capture records and both streams, the job record directory and the remap
/// material. **Never the renders**: the job output folder is not touched.
/// Sent at a connection for every history tombstone whose machine-side copy is
/// still owed (`State, identity and recovery` H5, `Queue & History` X4).
///
/// **Carries** [`THE_JOB_FIELD`].
///
/// **What comes back**: [`THE_ERASES_OUTCOME_FIELD`] —
/// [`NOTHING_OF_IT_IS_HERE`] where no unit stands on this machine, so it is
/// not a machine holding that identity; otherwise [`IT_WAS_ERASED`] or
/// [`IT_WAS_NOT_WHOLLY_ERASED`] with [`THE_UNITS_NOT_DONE_FIELD`] naming the
/// units, as `Queue & History`'s erase spells them.
///
/// **C6: recorded before it acts** — where anything of the job stands. **The
/// erase's own command record is never among the records it removes**
/// (`Queue & History` X2). **Where nothing stands it records nothing**, so a
/// machine that never held the job is left holding no record naming it.
pub const THE_ERASE_REQUEST: &str = "a deleted job's record is erased";

/// **What the erase came to**: [`IT_WAS_ERASED`], [`IT_WAS_NOT_WHOLLY_ERASED`]
/// or [`NOTHING_OF_IT_IS_HERE`].
pub const THE_ERASES_OUTCOME_FIELD: &str = "what the erase came to";
/// Every unit is done — removed, or already absent.
pub const IT_WAS_ERASED: &str = "erased";
/// At least one unit is not done; [`THE_UNITS_NOT_DONE_FIELD`] names them.
pub const IT_WAS_NOT_WHOLLY_ERASED: &str = "not wholly erased";
/// **No unit of the job stands here**, and nothing was recorded or done.
pub const NOTHING_OF_IT_IS_HERE: &str = "nothing of it here";
/// **The units not done, by their names in the desktop's outcome record**,
/// written and read by `Queue & History`'s erase alone.
pub const THE_UNITS_NOT_DONE_FIELD: &str = "units not done";

// ------------------------------------------------------- a job's crash files
//
// **Named 2026-10-24, step 11's R3b, machine half**, which is the piece that
// answers it. The desktop's take of it is R3b's other half and comes later.

/// **A job's kept crash files, read** — `Reporting a problem` G5a: *"a new
/// pure reading … returns the job's kept files whole, each named by its
/// attempt, or answers that there are none"*, and C6's own sentence for it.
/// The render worker keeps them, one per failed attempt that left one
/// (`Rendering` K13's crash-file bullet); this reading only reads what it kept.
///
/// **Carries** [`THE_JOB_FIELD`]. **Chosen here**: one job per request, as the
/// note reading is, since `Reporting a problem` G1's take names one job.
///
/// **What comes back**: [`ON_THIS_MACHINE_FIELD`] — [`IT_IS_ABSENT`] where the
/// machine holds no record of the job, [`THE_MACHINE_CANNOT_SAY`] with
/// [`THE_MACHINES_OWN_WORDS_FIELD`] where it could not look — and, where the
/// job is present, [`HOW_MANY_CRASH_FILE_STATEMENTS_FIELD`] with one
/// [`A_CRASH_FILE_STATEMENT`] per failed attempt the worker took the step for,
/// newest attempt first. **None at all is the answer that there are none.**
/// Each item carries [`THE_ATTEMPT_FIELD`], [`THE_CRASH_FILES_OWN_NAME_FIELD`]
/// where the name is known, [`WHAT_BECAME_OF_THE_CRASH_FILE_FIELD`], and then
/// either [`THE_CRASH_FILE_WHOLE_FIELD`] or [`WHY_NO_CRASH_FILE_WAS_KEPT_FIELD`].
///
/// **Nothing is trimmed or filtered** (`Reporting a problem` G4): a file is
/// carried whole or, where the files kept for the job would not fit in one
/// answer together, not carried and said to be so.
///
/// **C6: a pure reading.** It registers nothing and changes nothing.
pub const THE_KEPT_CRASH_FILES_REQUEST: &str = "a job's kept crash files";

/// **How many crash-file statements the answer carries**, as a decimal number.
/// One per failed attempt the render worker took the crash-file step for.
pub const HOW_MANY_CRASH_FILE_STATEMENTS_FIELD: &str = "crash-file statements";
/// **One attempt's crash-file statement**, as an item of the answer
/// ([`the_field_of_item`]).
pub const A_CRASH_FILE_STATEMENT: &str = "crash-file statement";
/// **The name Blender gives the crash file**, as bytes — `Reporting a problem`
/// G5's first bound: *"The name is the one user-derived string in the
/// arrangement and travels with the take."* Absent where the name could not
/// be established for the file the attempt loaded.
pub const THE_CRASH_FILES_OWN_NAME_FIELD: &str = "the crash file's own name";
/// **What became of the attempt's crash file**: one of the four values below.
pub const WHAT_BECAME_OF_THE_CRASH_FILE_FIELD: &str = "what became of the crash file";
/// **A copy was kept**, and [`THE_CRASH_FILE_WHOLE_FIELD`] carries it.
pub const THE_CRASH_FILE_WAS_KEPT: &str = "kept";
/// **No copy was kept**, and [`WHY_NO_CRASH_FILE_WAS_KEPT_FIELD`] says why —
/// `Reporting a problem` S5's *absent*, with the reason `Reporting a problem`
/// G5a has stated.
pub const NO_CRASH_FILE_WAS_KEPT: &str = "none kept";
/// **What the worker wrote for the attempt does not read back whole now** —
/// the statement, or the copy it names — and [`THE_MACHINES_OWN_WORDS_FIELD`]
/// says what the machine found. Never read as *none kept*.
pub const THE_KEPT_CRASH_FILE_DID_NOT_READ_BACK: &str = "it does not read back";
/// **A copy was kept and it is not carried in this answer**, because the
/// files kept for the job do not fit in one answer together. Never trimmed.
pub const THE_KEPT_CRASH_FILE_DID_NOT_FIT: &str = "kept, and it does not fit in this answer";
/// **The kept copy, whole**, as Blender wrote it.
pub const THE_CRASH_FILE_WHOLE_FIELD: &str = "the crash file, whole";
/// **Why no copy was kept, in the machine's words** — the reason
/// `Reporting a problem` G5a has stated for a file not taken.
pub const WHY_NO_CRASH_FILE_WAS_KEPT_FIELD: &str = "why no crash file was kept";

// ------------------------------------------- a live job's Telegram reporting
//
// **Named 2026-10-30, the twenty-first notes, note 7** (`Telegram` V1c).

/// **`Telegram` V1c: a live job's reporting choice is changed from the desk.**
///
/// **Carries** [`THE_JOB_FIELD`] and [`THE_REPORTING_CHOICE_FIELD`]. The
/// supervisor changes a job that is waiting, held or rendering and refuses one
/// that has ended, saying so. **For a first-frame check the desk names the held
/// range job, and the supervisor writes both jobs of the pair in one act.** The
/// value is written into the job record as its current choice, durably, before
/// the answer, and [`THE_TELEGRAM_CHOICE_CHANGED_EVENT`] is journalled. **The
/// manifest is never rewritten.**
///
/// **What comes back**: [`THE_CHANGES_OUTCOME_FIELD`] on a recorded outcome —
/// [`IT_WAS_CHANGED`], or [`IT_WAS_NOT_CHANGED`] with
/// [`THE_MACHINES_OWN_WORDS_FIELD`] saying why — then [`THE_JOB_FIELD`] and,
/// where the change landed, [`THE_REPORTING_CHOICE_FIELD`] as the record now
/// holds it, which is what the desk's own record takes.
///
/// **A program older than this refuses it as unknown**, and the desk offers
/// the change only on a server whose release knows it, so nothing is guessed
/// from that refusal.
///
/// **C6: recorded before it acts.**
pub const THE_TELEGRAM_CHOICE_IS_CHANGED_REQUEST: &str = "a live job's Telegram reporting is changed";

/// **The first release sequence whose server program answers
/// [`THE_TELEGRAM_CHOICE_IS_CHANGED_REQUEST`]** — `Telegram` V1c: *"The desktop
/// offers the change only on a server whose program's release (`Server
/// protocol and transfers` P3b, B8d) is at or above the release that brings
/// this clause."* 0.1.8 was published at sequence 1812 without it, and every
/// later release is published at a greater sequence (`Server protocol and
/// transfers` B9's floor only rises), so the first release built from this
/// code is at 1813 or above. Added 2026-10-30, the twenty-first notes, note 7.
pub const THE_FIRST_RELEASE_THAT_CHANGES_A_LIVE_JOBS_REPORTING: u64 = 1813;

/// **One `Telegram` V1a value in the manifest's own form**, written as JSON
/// text — `"Nothing"`, or `{"APictureEveryFewMinutes":{"minutes":5}}` — so the
/// manifest's own reader (`smoozy_telegram::job_messages::TheReportingChoice`)
/// reads the record's value exactly as it reads the manifest's.
pub const THE_REPORTING_CHOICE_FIELD: &str = "reporting choice";

/// **`Queue & History` A1, for `Telegram` V1d — the machine's phone's choice**,
/// carried once at the top of the queue reading where the machine holds one,
/// the four fields together or none: the value in
/// [`THE_REPORTING_CHOICE_FIELD`]'s form, the operation's identity (the same
/// on every machine that heard that one choice), Telegram's own `date` for the
/// message or the ask a tap was on (whole seconds since 1970), and the
/// update's identity, which orders two choices of one `date`. **Later is the
/// greater `date`, then the greater update.** Per job, the reading carries
/// [`THE_REPORTING_CHOICE_FIELD`] where the job record holds a current choice.
/// An older desktop ignores all of them. Added 2026-10-31, the Telegram
/// discussion, the server half.
pub const THE_PHONES_REPORTING_CHOICE_FIELD: &str = "the phone's reporting choice";
pub const THE_PHONES_CHOICE_OPERATION_FIELD: &str = "the phone's choice's operation";
pub const THE_PHONES_CHOICE_MOMENT_FIELD: &str = "the phone's choice's moment";
pub const THE_PHONES_CHOICE_UPDATE_FIELD: &str = "the phone's choice's update";
/// **What a change of a live job's reporting came to**, on a recorded outcome.
pub const THE_CHANGES_OUTCOME_FIELD: &str = "what the change came to";
/// The job record, and its pair's, now hold the new choice.
pub const IT_WAS_CHANGED: &str = "changed";
/// The job had ended, or the value was not one the manifest can hold; the
/// machine's own words say which.
pub const IT_WAS_NOT_CHANGED: &str = "not changed";

// ------------------------------------------------ a folder's size, counted
//
// **Named 2026-10-30, the twenty-third notes, note 2** (`File Browser`'s
// section 1.7).

/// **`Server protocol and transfers` §1.1: a folder's size is counted on the
/// machine** — `File Browser` L25.
///
/// **Carries** [`THE_COUNTED_FOLDER_FIELD`], one path on the machine as a
/// protocol field and never a word of a command. The entry point walks
/// everything below it by `File Browser` L26's rules — apparent size, links
/// never followed, the system's in-memory pseudo filesystems not descended,
/// other mounts descended, an unreadable folder skipped and the count marked
/// incomplete.
///
/// **Answered by the entry point in the invocation that carries it, and never
/// passed to the supervisor**, as [`THE_FILE_IS_COPIED_ON_THE_MACHINE_REQUEST`]
/// is: it reads no store the supervisor keeps, and a walk of minutes there
/// would hold every other request behind it. A package that does not know it
/// passes it on, and its supervisor refuses it by name; the desktop offers the
/// act only on a server whose release carries it
/// ([`THE_FIRST_RELEASE_THAT_COUNTS_A_FOLDERS_SIZE`]), so nothing is guessed
/// from that refusal.
///
/// **What comes back**: [`THE_COUNTS_OUTCOME_FIELD`] — [`IT_WAS_COUNTED`] with
/// [`THE_BYTES_COUNTED_FIELD`], [`THE_FILES_COUNTED_FIELD`],
/// [`THE_FOLDERS_COUNTED_FIELD`] and [`THE_COUNT_IS_INCOMPLETE_FIELD`];
/// otherwise [`NOTHING_IS_THERE`], [`IT_IS_NOT_A_FOLDER`] or
/// [`IT_WAS_NOT_READ`], with [`THE_MACHINES_OWN_WORDS_FIELD`] beside the last.
///
/// **C6: a pure reading.** Nothing on the machine is written.
pub const THE_FOLDERS_SIZE_IS_COUNTED_REQUEST: &str = "a folder's size is counted on the machine";

/// **The first release sequence whose server program answers
/// [`THE_FOLDERS_SIZE_IS_COUNTED_REQUEST`]** — `File Browser` L28, gated as
/// `Telegram` V1c's change is. 0.1.8 was published at sequence 1812 without
/// it, and nothing has been published since, so the next release — the same
/// one that first carries
/// [`THE_FIRST_RELEASE_THAT_CHANGES_A_LIVE_JOBS_REPORTING`] — is the first to
/// carry this too. Added 2026-10-30, the twenty-third notes, note 2.
pub const THE_FIRST_RELEASE_THAT_COUNTS_A_FOLDERS_SIZE: u64 = 1813;

/// **The folder to count, by its path on the machine.**
pub const THE_COUNTED_FOLDER_FIELD: &str = "the counted folder";
/// **What the count came to**: one of the four values below.
pub const THE_COUNTS_OUTCOME_FIELD: &str = "what the count came to";
/// The walk ended and the four figures ride beside this.
pub const IT_WAS_COUNTED: &str = "counted";
/// **The path names something that is not a folder.**
pub const IT_IS_NOT_A_FOLDER: &str = "not a folder";
/// **The folder itself could not be opened or read**, and the machine's own
/// words say why. A folder below it that could not be read is not this: it
/// is skipped and marks the count incomplete.
pub const IT_WAS_NOT_READ: &str = "not read";
/// **Total bytes, the apparent size**, in decimal.
pub const THE_BYTES_COUNTED_FIELD: &str = "bytes counted";
/// **How many files were counted**, in decimal.
pub const THE_FILES_COUNTED_FIELD: &str = "files counted";
/// **How many folders below the counted one were counted**, in decimal.
pub const THE_FOLDERS_COUNTED_FIELD: &str = "folders counted";
/// **Whether a folder below could not be read**: [`THE_ANSWER_IS_YES`], or
/// [`THE_COUNT_IS_WHOLE`]. On *yes* the figures are a lower bound.
pub const THE_COUNT_IS_INCOMPLETE_FIELD: &str = "the count is incomplete";
/// Nothing below the counted folder was skipped.
pub const THE_COUNT_IS_WHOLE: &str = "no";

// --------------------------------------- the four values of files on the machine
//
// **Built 2026-11-04**, out of a real folder of 480 files (`Dependency Upload &
// Remap` Q5 route 1, §1.1's paragraph of 2026-10-03).

/// **§1.1's request *the four values of files on the machine***: for each path,
/// `Job files and outputs` R10's four values, read by the filesystem call that
/// follows a link, in the units the command route reads.
///
/// **Carries** [`THE_PATHS_READ_FIELD`]; **answers** [`THE_VALUES_READ_FIELD`].
///
/// **Answered by the entry point in the invocation that carries it, and never
/// passed to the supervisor**, as [`THE_FILE_IS_COPIED_ON_THE_MACHINE_REQUEST`]
/// is. **A package that does not know it passes it on and its supervisor
/// refuses it by name**, and that refusal is how the desktop learns to use the
/// command route: a release's declared request set is written empty as built.
///
/// **C6: a pure reading.** Nothing on the machine is written.
pub const THE_FOUR_VALUES_OF_FILES_REQUEST: &str = "the four values of files on the machine";

/// **The paths, each ended by a zero byte**, which no path can hold (`Server
/// protocol and transfers` P7's bytes). A protocol field and never a word of a
/// command.
pub const THE_PATHS_READ_FIELD: &str = "the paths read";
/// **One line of text for each path, in the order asked** — see
/// [`the_values_written`].
pub const THE_VALUES_READ_FIELD: &str = "the values read";

/// **The most paths one ask carries.** With [`THE_LONGEST_WORDS_BESIDE_NOT_READ`]
/// this keeps the worst answer inside `framing::THE_LARGEST_FRAME`.
pub const THE_MOST_PATHS_IN_ONE_ASK: usize = 4_000;
/// **The most path bytes one ask carries**, under `framing::THE_LARGEST_FRAME`
/// with room for the frame's own fields.
pub const THE_MOST_PATH_BYTES_IN_ONE_ASK: usize = 512 * 1024;
/// **The machine's own words beside *not read* are cut here**, on a character's
/// edge, so one answer's size is known before it is asked.
pub const THE_LONGEST_WORDS_BESIDE_NOT_READ: usize = 180;

/// **What one path answered.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnePathAnswered {
    /// Size, inode, modification time, change time: R10's order and units.
    Values { size: u64, inode: u64, modification_time: i64, change_time: i64 },
    /// **The system said no such file**, a dangling link among them, as a file
    /// `stat -L` could not find.
    NothingThere,
    /// Anything else, with the machine's own words.
    NotRead(String),
}

/// **The paths, as the one field carries them.**
pub fn the_paths_written(paths: &[Vec<u8>]) -> Vec<u8> {
    let mut field = Vec::with_capacity(paths.iter().map(|path| path.len() + 1).sum());
    for path in paths {
        field.extend_from_slice(path);
        field.push(0);
    }
    field
}

/// **The paths the one field carries.** Each ends at its zero byte, so an empty
/// field holds none.
pub fn the_paths_read_back(field: &[u8]) -> Vec<&[u8]> {
    field.split(|byte| *byte == 0).take(field.iter().filter(|byte| **byte == 0).count()).collect()
}

/// **The answer, as the one field carries it**: for each path one line, the
/// four numbers, or [`NOTHING_IS_THERE`], or [`IT_WAS_NOT_READ`] and the words.
pub fn the_values_written(answers: &[OnePathAnswered]) -> String {
    let mut out = String::new();
    for answer in answers {
        match answer {
            OnePathAnswered::Values { size, inode, modification_time, change_time } => {
                out.push_str(&format!("{size} {inode} {modification_time} {change_time}"));
            }
            OnePathAnswered::NothingThere => out.push_str(NOTHING_IS_THERE),
            OnePathAnswered::NotRead(words) => {
                out.push_str(IT_WAS_NOT_READ);
                out.push_str(": ");
                let words = words.replace(['\n', '\r'], " ");
                let mut cut = words.len().min(THE_LONGEST_WORDS_BESIDE_NOT_READ);
                while !words.is_char_boundary(cut) {
                    cut -= 1;
                }
                out.push_str(&words[..cut]);
            }
        }
        out.push('\n');
    }
    out
}

/// **The answer, read back**, or nothing where a line reads as none of the three
/// or the count is not the count asked. **Four fields or nothing**, as the
/// command route's own reader has it: no partly-read identity.
pub fn the_values_read_back(field: &str, asked: usize) -> Option<Vec<OnePathAnswered>> {
    let answers = field
        .lines()
        .map(|line| {
            if line == NOTHING_IS_THERE {
                return Some(OnePathAnswered::NothingThere);
            }
            if let Some(words) = line.strip_prefix(IT_WAS_NOT_READ) {
                return Some(OnePathAnswered::NotRead(words.trim_start_matches(": ").to_string()));
            }
            let mut fields = line.split(' ');
            let answered = OnePathAnswered::Values {
                size: fields.next()?.parse().ok()?,
                inode: fields.next()?.parse().ok()?,
                modification_time: fields.next()?.parse().ok()?,
                change_time: fields.next()?.parse().ok()?,
            };
            fields.next().is_none().then_some(answered)
        })
        .collect::<Option<Vec<_>>>()?;
    (answers.len() == asked).then_some(answers)
}

/// **A long list, split into the asks that carry it**, in its own order, by
/// [`THE_MOST_PATHS_IN_ONE_ASK`] and [`THE_MOST_PATH_BYTES_IN_ONE_ASK`].
pub fn the_asks_over(paths: &[Vec<u8>]) -> Vec<&[Vec<u8>]> {
    let mut asks = Vec::new();
    let (mut from, mut bytes) = (0, 0);
    for (at, path) in paths.iter().enumerate() {
        let this_one = path.len() + 1;
        if at > from && (at - from == THE_MOST_PATHS_IN_ONE_ASK || bytes + this_one > THE_MOST_PATH_BYTES_IN_ONE_ASK) {
            asks.push(&paths[from..at]);
            from = at;
            bytes = 0;
        }
        bytes += this_one;
    }
    if from < paths.len() {
        asks.push(&paths[from..]);
    }
    asks
}

/// **Every request named in this file**, with where it stands under C6 and
/// the fields named here that it carries.
///
/// **The tests below hold this list to the file's own constants**, so a name
/// added without an entry fails, and hold each name to one spelling in the
/// workspace and to the machine program's two lists.
pub const EVERY_REQUEST_NAMED_HERE: [ARequestNamedHere; 38] = [
    ARequestNamedHere {
        name: THE_HEALTH_ANSWER_REQUEST,
        under_c6: UnderC6::APureReading,
        carries: &[],
    },
    ARequestNamedHere {
        name: THE_JOB_RECORD_IS_WRITTEN_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[
            THE_JOB_FIELD,
            THE_LABEL_FIELD,
            THE_HELD_BEHIND_FIELD,
            THE_HOLD_REASON_FIELD,
        ],
    },
    ARequestNamedHere {
        name: THE_DOES_THE_MACHINE_HOLD_THIS_JOB_REQUEST,
        under_c6: UnderC6::APureReading,
        carries: &[THE_JOB_FIELD],
    },
    ARequestNamedHere {
        name: THE_QUEUE_READING_REQUEST,
        under_c6: UnderC6::APureReading,
        carries: &[],
    },
    ARequestNamedHere {
        name: THE_PER_JOB_NOTE_READING_REQUEST,
        under_c6: UnderC6::APureReading,
        carries: &[THE_JOB_FIELD],
    },
    ARequestNamedHere {
        name: THE_READ_FORWARD_REQUEST,
        under_c6: UnderC6::APureReading,
        carries: &[THE_CURSOR_FIELD],
    },
    ARequestNamedHere {
        name: THE_CURSOR_MOVES_FORWARD_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[THE_CURSOR_FIELD],
    },
    ARequestNamedHere {
        name: THE_SETUP_RUN_IS_OPENED_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[
            THE_KIND_OF_SETUP_FIELD,
            THE_RUN_TAKEN_OVER_FIELD,
            THE_RECOVERY_ABANDONS_FIELD,
        ],
    },
    ARequestNamedHere {
        name: THE_ONE_SETUP_STEP_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[
            THE_SETUP_RUN_FIELD,
            THE_STEP_FIELD,
            THE_BLENDER_VERSION_FIELD,
            THE_EXECUTABLE_FIELD,
            WHAT_THE_STEP_DID_FIELD,
            HOW_MANY_COVERED_JOBS_FIELD,
            THE_MANAGERS_PACKAGE_FIELD,
        ],
    },
    ARequestNamedHere {
        name: THE_SETUP_RUNS_STATE_REQUEST,
        under_c6: UnderC6::APureReading,
        carries: &[
            THE_SETUP_RUN_FIELD,
            THE_STEPS_WORDS_TURN_FIELD,
            THE_ORDINARY_WORDS_HELD_FIELD,
            THE_COMPLAINT_WORDS_HELD_FIELD,
        ],
    },
    ARequestNamedHere {
        name: THE_SET_UP_CARDS_READING_REQUEST,
        under_c6: UnderC6::APureReading,
        carries: &[],
    },
    ARequestNamedHere {
        name: THE_INSTALL_RECORDS_REQUEST,
        under_c6: UnderC6::APureReading,
        carries: &[],
    },
    ARequestNamedHere {
        name: THE_BOUNDARY_IS_WRITTEN_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[
            THE_SETUP_RUN_FIELD,
            THE_BOUNDARY_FIELD,
            WHAT_THE_STEP_DID_FIELD,
        ],
    },
    ARequestNamedHere {
        name: THE_SETUP_IN_PROGRESS_IS_SET_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[THE_SETUP_RUN_FIELD],
    },
    ARequestNamedHere {
        name: THE_SETUP_IN_PROGRESS_IS_CLEARED_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[THE_SETUP_RUN_FIELD],
    },
    ARequestNamedHere {
        name: THE_SETUP_RUN_IS_STOPPED_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[THE_SETUP_RUN_FIELD],
    },
    ARequestNamedHere {
        name: THE_FAILED_STEP_IS_RECORDED_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[
            THE_SETUP_RUN_FIELD,
            THE_FAILED_STEP_FIELD,
            THE_MACHINES_OWN_WORDS_FIELD,
        ],
    },
    ARequestNamedHere {
        name: THE_ELEVATED_STEPS_START_IS_RECORDED_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[THE_SETUP_RUN_FIELD, THE_BOUNDARY_FIELD],
    },
    ARequestNamedHere {
        name: THE_SUPERVISOR_ENDS_ITSELF_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[],
    },
    ARequestNamedHere {
        name: THE_TELEGRAM_ROLE_IS_ENABLED_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[],
    },
    ARequestNamedHere {
        name: THE_TELEGRAM_ROLE_IS_DISABLED_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[],
    },
    ARequestNamedHere {
        name: THE_HEALTH_READINGS_REQUEST,
        under_c6: UnderC6::APureReading,
        carries: &[],
    },
    ARequestNamedHere {
        name: THE_ANSWER_TO_A_HOLD_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[THE_HOLD_FIELD, THE_ANSWER_FIELD, THE_ROUTE_FIELD],
    },
    ARequestNamedHere {
        name: THE_READING_OF_A_BLEND_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[
            THE_WHICH_READING_FIELD,
            THE_BUILD_FIELD,
            THE_READINGS_LIBRARY_PAIRS_FIELD,
            THE_BLEND_FIELD,
            THE_BLENDER_VERSION_FIELD,
        ],
    },
    ARequestNamedHere {
        name: THE_CLEAR_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[
            THE_PAIRED_STOP_FIELD,
            THE_GROUP_FIELD,
            THE_RENDER_THE_DESKTOP_LAST_SAW_FIELD,
        ],
    },
    ARequestNamedHere {
        name: THE_STOP_PAIRED_WITH_A_CLEAR_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[
            THE_PAIRED_CLEAR_FIELD,
            THE_GROUP_FIELD,
            THE_RENDER_THE_DESKTOP_LAST_SAW_FIELD,
        ],
    },
    ARequestNamedHere {
        name: THE_STOP_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[THE_OPERATION_TO_STOP_FIELD],
    },
    ARequestNamedHere {
        name: THE_CANCEL_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[THE_JOB_FIELD],
    },
    ARequestNamedHere {
        name: THE_REORDER_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[THE_JOB_FIELD, THE_POSITION_FIELD, THE_JOB_IT_GOES_AFTER_FIELD],
    },
    ARequestNamedHere {
        name: THE_GROUP_BLOCK_MOVE_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[THE_GROUP_FIELD, THE_POSITION_FIELD, THE_JOB_IT_GOES_AFTER_FIELD],
    },
    ARequestNamedHere {
        name: THE_FILE_IS_COPIED_ON_THE_MACHINE_REQUEST,
        under_c6: UnderC6::OutsideC6,
        carries: &[THE_COPYS_SOURCE_FIELD, THE_COPYS_STAGING_PATH_FIELD],
    },
    ARequestNamedHere {
        name: THE_DIGEST_OVER_A_BYTE_RANGE_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[
            THE_DIGESTED_FILE_FIELD,
            THE_RANGE_STARTS_AT_FIELD,
            THE_RANGE_LENGTH_FIELD,
        ],
    },
    ARequestNamedHere {
        name: THE_FIXED_COPY_IS_PRODUCED_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[
            THE_BUILD_FIELD,
            THE_BLEND_FIELD,
            THE_BLENDER_VERSION_FIELD,
            THE_FIXINGS_STAGING_FIELD,
            THE_FIXINGS_PAIRS_FIELD,
        ],
    },
    ARequestNamedHere {
        name: THE_ERASE_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[THE_JOB_FIELD],
    },
    ARequestNamedHere {
        name: THE_KEPT_CRASH_FILES_REQUEST,
        under_c6: UnderC6::APureReading,
        carries: &[THE_JOB_FIELD],
    },
    ARequestNamedHere {
        name: THE_TELEGRAM_CHOICE_IS_CHANGED_REQUEST,
        under_c6: UnderC6::RecordedBeforeItActs,
        carries: &[THE_JOB_FIELD, THE_REPORTING_CHOICE_FIELD],
    },
    ARequestNamedHere {
        name: THE_FOLDERS_SIZE_IS_COUNTED_REQUEST,
        under_c6: UnderC6::APureReading,
        carries: &[THE_COUNTED_FOLDER_FIELD],
    },
    ARequestNamedHere {
        name: THE_FOUR_VALUES_OF_FILES_REQUEST,
        under_c6: UnderC6::APureReading,
        carries: &[THE_PATHS_READ_FIELD],
    },
];

// ------------------------------------------------------------ the answers
//
// **Named by step 10c piece 5, which composes them.** Every name and value
// below is Build's; what each answer must say is its owning clause's, cited at
// the constant. A value is text, as a request's is, and a moment is written as
// whole seconds since 1970 beside the name of the clock it was taken on
// (`State, identity and recovery` D5: a reading carries its clock).

/// **The name of one field of one item in an answer that carries a list** —
/// `"<item> <at>: <field>"`, counting from zero.
///
/// **Both ends compose the name here**, so neither spells the pattern.
pub fn the_field_of_item(item: &str, at: usize, field: &str) -> String {
    format!("{item} {at}: {field}")
}

/// **C8's answer about the request's own identity** — *"a recorded outcome;
/// still in hand; no record of that identity"*, the set closed at three.
/// **Carried by the two requests here that act**: the job record's and the
/// cursor's move.
pub const WHAT_BECAME_OF_IT_FIELD: &str = "what became of it";
/// C8's first.
pub const IT_HAS_A_RECORDED_OUTCOME: &str = "a recorded outcome";
/// C8's second. *Still in hand* is an answer, not a silence.
pub const IT_IS_STILL_IN_HAND: &str = "still in hand";
/// C8's third: *"arrived and was refused before it was recorded. The work did
/// not happen."*
pub const THERE_IS_NO_RECORD_OF_IT: &str = "no record of that identity";

/// **What the machine said, in its own words**, where an answer has to say why
/// — a store's refusal, a request the machine could not act on. **Never parsed
/// by the desktop**; it is shown or kept as the machine gave it.
pub const THE_MACHINES_OWN_WORDS_FIELD: &str = "in the machine's own words";

/// **Whether a job, or its note, stands on the machine** —
/// `State, identity and recovery` V1 step 2's three: *present*; *absent, and
/// the submission was refused or never taken*; *absent, and the machine cannot
/// say*. Carried by the ask after a lost acknowledgement, by the job record's
/// answer and by the note reading.
pub const ON_THIS_MACHINE_FIELD: &str = "on this machine";
/// The job record stands (`Queue & History` Q3: *"A job is in the line exactly
/// when its job record exists"*).
pub const IT_IS_PRESENT: &str = "present";
/// **No job record stands, and the store that would hold it read whole.**
/// `Queue & History` Q3 writes the record only after the submission's own
/// command record, in one act, so a job with no record is one the machine
/// refused, never took, or has since removed — and in each case the job is not
/// on the machine.
pub const IT_IS_ABSENT: &str = "absent";
/// **The store could not be read**, so the machine says nothing either way and
/// [`THE_MACHINES_OWN_WORDS_FIELD`] says why.
pub const THE_MACHINE_CANNOT_SAY: &str = "the machine cannot say";

/// **`Queue & History` Q2's queue state**, from its closed set.
pub const THE_QUEUE_STATE_FIELD: &str = "queue state";
/// In the line.
pub const IT_IS_WAITING: &str = "waiting";
/// Held, with [`THE_HOLD_IT_IS_HELD_UNDER_FIELD`] beside it.
pub const IT_IS_HELD: &str = "held";
/// Rendering, with [`THE_RENDER_OPERATION_FIELD`] beside it.
pub const IT_IS_RENDERING: &str = "rendering";
/// Ended, with [`THE_OUTCOME_IT_ENDED_WITH_FIELD`] beside it.
pub const IT_HAS_ENDED: &str = "ended";
/// **The hold identity a held job is held under** (`Rented-server runtime`
/// U2), which is what the answer to a hold names.
pub const THE_HOLD_IT_IS_HELD_UNDER_FIELD: &str = "held under";
/// The render operation's identity, for a job that is rendering.
pub const THE_RENDER_OPERATION_FIELD: &str = "render operation";
/// The recorded outcome's identity, for a job that has ended.
pub const THE_OUTCOME_IT_ENDED_WITH_FIELD: &str = "ended with";
/// **A waiting job's place in its machine's line, counting from one.**
/// **Never the sequence key**: `Queue & History` Q7 keeps keys opaque outside
/// the supervisor, and *"the desktop never composes one"*.
pub const THE_PLACE_IN_THE_LINE_FIELD: &str = "place in the line";

/// **The queue reading's count of jobs**, each then carried as the item
/// [`THE_JOB_FIELD`] with the queue-state fields above — `Queue & History` A1:
/// *"Per job: its state, its admission point standing, its operation, its
/// unreleased hold."*
pub const HOW_MANY_JOBS_FIELD: &str = "jobs";
/// **The job the admission point would start next** (`Queue & History` Q4),
/// absent where the line holds nothing startable.
pub const THE_JOB_THAT_STARTS_NEXT_FIELD: &str = "starts next";
/// **Records in the store that did not read back**, counted rather than left
/// out, so a reading that is missing a job says so.
pub const HOW_MANY_DID_NOT_READ_BACK_FIELD: &str = "records that did not read back";

// ------------------------------------------- a rendering job's live figures
//
// **`Queue & History` R5's figures, as the render worker reports them to the
// supervisor and as the queue reading carries them for a rendering job** —
// the same names on both legs, so the worker, the supervisor and the desktop
// spell each one once. Added 2026-10-03, the Creator's notes on 0.1.8, piece
// 6a. **Each is held in the supervisor's memory and never written down**
// (`Queue & History` R5: period-held), and every one but the moment, its clock
// and the frames counted is absent where the stream printed nothing for it
// (`Queue & History` R4: absent, never zero). The composition and the reading
// back are `smoozy_queue_history_machine::readings::AWorkersReport`'s, so no
// end spells the set itself.
//
// **The attempt the figures were read on** rides under [`THE_ATTEMPT_FIELD`].
// **A running job's two sinks** ride under [`THE_ORDINARY_STREAM_FIELD`],
// [`THE_COMPLAINTS_STREAM_FIELD`] and their two lengths: each is the path on
// the machine and the bytes standing in it when the answer was composed, so the
// desktop builds no machine path itself (`Queue & History` E8 reads them).

/// **When the figures were taken**, as whole seconds since 1970 on the clock
/// named beside it (`State, identity and recovery` D5).
pub const THE_FIGURES_TAKEN_AT_FIELD: &str = "figures taken at";
/// **The clock the figures were taken on**: the machine's own.
pub const THE_FIGURES_CLOCK_FIELD: &str = "figures taken on the clock of";
/// **`Queue & History` R9: the frame in hand.**
pub const THE_FRAME_IN_HAND_FIELD: &str = "frame in hand";
/// **`Queue & History` R10: the view layer, in 4.5 only**, as printed.
pub const THE_VIEW_LAYER_IN_HAND_FIELD: &str = "view layer in hand";
/// **`Queue & History` R11: the largest sample count seen for the frame in
/// hand**, and the total printed beside it, each as printed.
pub const THE_SAMPLES_SEEN_FIELD: &str = "samples seen";
pub const THE_SAMPLES_PRINTED_TOTAL_FIELD: &str = "samples printed total";
/// **`Queue & History` R12's stopped-short mark**, carried only where it is
/// set, as [`THE_ANSWER_IS_YES`].
pub const THE_FRAME_STOPPED_SHORT_FIELD: &str = "frame stopped short";
/// **`Queue & History` R13: the tile figures where a tiled line carried
/// them**, done against total.
pub const THE_TILES_DONE_FIELD: &str = "tiles done";
pub const THE_TILES_TOTAL_FIELD: &str = "tiles total";
/// **`Queue & History` R15: Blender's own estimate for the frame in hand**,
/// as printed — Blender's, and rough.
pub const BLENDERS_OWN_ESTIMATE_FIELD: &str = "Blender's estimate";
/// **`Queue & History` R6's count of frames announced or completed.**
pub const THE_FRAMES_COUNTED_FIELD: &str = "frames counted";
/// **`Queue & History` P1a's figure: the newest frame of the attempt's list
/// whose save has been announced.**
pub const THE_NEWEST_FRAME_SAVED_FIELD: &str = "newest frame saved";
/// **`Queue & History` B1's plan total for the job**, in plan units — worked
/// out once on the machine from the reading the render took at its start.
pub const THE_PLAN_TOTAL_FIELD: &str = "plan total";
/// **`Queue & History` B6's finished part, in the same plan units** — the
/// frames the attempt does not ask for and its own frames up to the newest
/// saved, worked out on the machine by `Queue & History` B1's own arithmetic,
/// so the second bar is this over [`THE_PLAN_TOTAL_FIELD`], both from the
/// machine. Added
/// 2026-10-03, the Creator's notes on 0.1.8, piece 6b.
pub const THE_UNITS_FINISHED_FIELD: &str = "plan units finished";
/// **`Queue & History` R5's *stopping* mark: a stop's interrupt has gone out on
/// the render**, carried in that job's part of the queue reading only where it
/// is set, as [`THE_ANSWER_IS_YES`], until the render ends. **It is the
/// supervisor's own fact about its own interrupt, whoever sent the stop**, and
/// not one of the reader's readings (`Queue & History` R6), so the worker never
/// reports it and the figures beside it are the ones held as they stood when
/// the interrupt went out. `Telegram` C1 reads it as *Stopping.* Added
/// 2026-10-19, the live pass's faults, piece E.
pub const A_STOPS_INTERRUPT_WENT_OUT_FIELD: &str = "a stop's interrupt went out";

/// **The per-job note, read** — `Queue & History` N1's fields. The label rides
/// under [`THE_LABEL_FIELD`], empty where none was given.
pub const THE_STARTED_FIELD: &str = "started";
/// The clock the start was taken on.
pub const THE_STARTED_CLOCK_FIELD: &str = "started on the clock of";
/// The moment it ended.
pub const THE_ENDED_FIELD: &str = "ended";
/// The clock the ending was taken on.
pub const THE_ENDED_CLOCK_FIELD: &str = "ended on the clock of";
/// **The ending's outcome, absent while the ending is null** (`Queue & History` N1: *"a null
/// ending for a job that has not ended, never an invented one"*).
pub const THE_ENDING_FIELD: &str = "ending";
/// The render kind's own ending, finished.
pub const IT_FINISHED: &str = "finished";
/// The render kind's own ending, failed.
pub const IT_FAILED: &str = "failed";
/// `Rented-server runtime` M21: the group was gone and no status was reaped.
pub const NO_STATUS_WAS_REAPED: &str = "no status was reaped";
/// Left stopped.
pub const IT_WAS_LEFT_STOPPED: &str = "left stopped";
/// Given up.
pub const IT_WAS_GIVEN_UP: &str = "given up";
/// **The exit status, verbatim** (`Rented-server runtime` M22), absent where
/// none was reaped.
pub const THE_EXIT_STATUS_FIELD: &str = "exit status";
/// **`Queue & History` N3's pointer**: the attempt whose capture record holds
/// the words. The words are not in the note.
pub const THE_ATTEMPT_THAT_HOLDS_THE_WORDS_FIELD: &str = "the attempt that holds the words";
/// **The reason the machine recorded with the outcome**, in the words the
/// machine wrote at the ending — `Rendering` K14's reason, which that clause records
/// verbatim for every failed ending. **Absent where the ending recorded
/// none**, which is every ending that is not a failure, and never an empty
/// stand-in.
///
/// **the paragraph after `Queue & History` N3 (added 2026-09-27)**: the
/// note's ending carries it and the answer that reads the note sends it. It is
/// the machine's own record of why the ending was classified as it was, and
/// **nothing mined from a stream** (`Queue & History` N3). Added at step 10c's fix piece 2.
pub const THE_REASON_THE_MACHINE_RECORDED_FIELD: &str = "the reason the machine recorded";
/// **Whether the attempt ended before any render began** (`Rendering` K14's
/// *before the render instruction*; the paragraph after `Queue & History` N3),
/// which the machine knows and no outcome word says. **Sent only where it did**,
/// and read as *a render began* where it is absent, so a note from an older machine
/// program keeps the words it always had. Added 2026-11-04.
pub const NO_RENDER_BEGAN_FIELD: &str = "no render began";
/// The one value [`NO_RENDER_BEGAN_FIELD`] is sent with.
pub const IT_IS_YES: &str = "yes";
/// **The device line as the render worker observed it at its own loop**
/// (`Queue & History` E13), verbatim — the line Blender printed, never a
/// reading of it. **Absent where no line passed.**
///
/// **Carried on every ending where it was observed, and stated on none but a
/// finished one**: `Queue & History` E12 makes a line from a run that did not
/// finish no evidence, and that judgement is the entry's
/// (the paragraph after `Queue & History` N3). It rides two things: the
/// render worker's report to the supervisor, and the note reading's answer.
/// Added at step 10c's fix piece 2.
pub const THE_DEVICE_LINE_FIELD: &str = "device line";
/// The attempt count.
pub const THE_ATTEMPTS_FIELD: &str = "attempts";
/// **The count of re-run marks**, each carried as the item [`A_RE_RUN`].
pub const HOW_MANY_RE_RUNS_FIELD: &str = "re-runs";
/// The item word for one re-run mark.
pub const A_RE_RUN: &str = "re-run";
/// One re-run mark's attempt.
pub const THE_ATTEMPT_FIELD: &str = "attempt";
/// **From which frame that re-run proceeded**, absent where its list was
/// empty.
pub const THE_FRAME_IT_PROCEEDED_FROM_FIELD: &str = "proceeded from frame";
/// **Given up, where it was.**
pub const THE_FRAME_IT_WAS_GIVEN_UP_AT_FIELD: &str = "given up at frame";
/// **`Queue & History` N6's last-frame figure**, absent where there is none
/// (`Queue & History` N7 and `Queue & History` N8).
pub const THE_LAST_FRAME_FIELD: &str = "last frame";
/// **`Job files and outputs` C27's four sets as the note carries them, by
/// count** — the files expected, present, absent and present at nothing — and
/// **absent altogether where the ending carried no statement**, which then
/// carries [`WHY_NO_COMPLETENESS_STATEMENT_FIELD`] instead (`Rented-server
/// runtime` S13: *"an outcome carrying no statement is not an outcome carrying
/// an empty one"*). Added at step 10c piece 5's second sitting.
///
/// **Counts and not the per-path sets.** A set names one path per expected
/// file, and a long job's would not fit one frame; carrying the sets, or
/// `Rendering` K18's per-frame set composed from them, wants a reading with a
/// budget of its own, which is the desktop reader's to ask for.
pub const THE_FILES_EXPECTED_FIELD: &str = "files expected";
/// The second of the four sets, counted.
pub const THE_FILES_PRESENT_FIELD: &str = "files present";
/// The third of the four sets, counted.
pub const THE_FILES_ABSENT_FIELD: &str = "files absent";
/// The fourth of the four sets, counted.
pub const THE_FILES_PRESENT_AT_ZERO_BYTES_FIELD: &str = "files present at zero bytes";
/// **Why the ending carried no completeness statement**, in the machine's own
/// words — present exactly where the four counts are not.
pub const WHY_NO_COMPLETENESS_STATEMENT_FIELD: &str = "why no completeness statement";

/// **`Queue & History` E9's half of the note reading's answer: every attempt
/// of the job that carries a capture record**, counted, each carried as the
/// item [`A_CAPTURE_RECORD`] — the attempt under [`THE_ATTEMPT_FIELD`], its two
/// sinks under [`THE_ORDINARY_STREAM_FIELD`], [`THE_COMPLAINTS_STREAM_FIELD`]
/// and their two lengths, and what each stream came to under the six names
/// below. **Carried only on a note whose ending is written**, because
/// `Queue & History` E9's fetch is made by the connection that learns the
/// ending; **absent from a
/// program older than the Creator's notes on 0.1.8, piece 6d**, and then the
/// desktop fetches nothing. The composition and the reading back are
/// `smoozy_queue_history_machine::acting::words`'s, so no end spells the set
/// itself. Added 2026-10-03, piece 6d.
pub const HOW_MANY_CAPTURE_RECORDS_FIELD: &str = "capture records";
/// The item word for one attempt's capture record.
pub const A_CAPTURE_RECORD: &str = "capture record";
/// **`Rented-server runtime` M13's outcome for the ordinary stream**, in
/// `smoozy_runtime::capture::CollectionOutcome`'s own words. **Absent where no
/// outcome is written yet** (`Desktop app and delivery` M10a), never a word
/// standing in for one.
pub const THE_ORDINARY_OUTCOME_FIELD: &str = "ordinary stream's outcome";
/// The same, for the complaints stream.
pub const THE_COMPLAINTS_OUTCOME_FIELD: &str = "complaints stream's outcome";
/// **`Rented-server runtime` M5's retained and discarded counts and the first
/// loss reason, for the ordinary stream**, where the worker reported them.
pub const THE_ORDINARY_RETAINED_COUNT_FIELD: &str = "ordinary stream retained";
pub const THE_ORDINARY_DISCARDED_COUNT_FIELD: &str = "ordinary stream discarded";
pub const THE_ORDINARY_FIRST_LOSS_FIELD: &str = "ordinary stream's first loss";
/// The same three, for the complaints stream.
pub const THE_COMPLAINTS_RETAINED_COUNT_FIELD: &str = "complaints stream retained";
pub const THE_COMPLAINTS_DISCARDED_COUNT_FIELD: &str = "complaints stream discarded";
pub const THE_COMPLAINTS_FIRST_LOSS_FIELD: &str = "complaints stream's first loss";

/// **The read forward's count of events**, each carried as the item
/// [`AN_EVENT`] — `State, identity and recovery` E1: its identity, its
/// sequence number and what happened.
pub const HOW_MANY_EVENTS_FIELD: &str = "events";
/// The item word for one event.
pub const AN_EVENT: &str = "event";
/// An event's sequence number (`State, identity and recovery` E1).
pub const THE_SEQUENCE_FIELD: &str = "sequence";
/// **An event's own identity**, which a reader applies it idempotently by
/// (`State, identity and recovery` E4).
pub const THE_EVENT_IDENTITY_FIELD: &str = "event identity";
/// The owning document's word for what happened.
pub const THE_KIND_FIELD: &str = "kind";
/// The durable subject the event is about, absent where it has none.
pub const THE_ABOUT_FIELD: &str = "about";
/// The owning document's own words.
pub const THE_SAYS_FIELD: &str = "says";
/// **The moment it happened** (`State, identity and recovery` E7).
pub const THE_HAPPENED_AT_FIELD: &str = "happened at";
/// The clock that moment was taken on.
pub const THE_HAPPENED_ON_CLOCK_FIELD: &str = "on the clock of";
/// **`State, identity and recovery` E5: the oldest sequence the journal still
/// holds, reported with every read**, so a reader detects a gap itself. Absent
/// where the journal holds no entry.
pub const THE_OLDEST_RETAINED_FIELD: &str = "oldest retained";
/// **One past the newest sequence the journal holds** — the cursor a reader
/// that has done all the work would next ask from (`State, identity and recovery` E3).
pub const THE_NEXT_SEQUENCE_FIELD: &str = "next sequence";
/// **The machine's own now, taken as the read forward is answered** —
/// `Queue & History` W3: *"Overdue is machine-now minus quiet-moment, both
/// taken from the machine in one reading."* The quiet moment is an event's own
/// moment (`Queue & History` W2), so the now rides the same answer, on the
/// same clock. **Absent where the machine holds no installation identity to
/// name its clock by**, and absent from a program older than the 38's piece 1
/// — and then no half hour is reckoned at all. Chosen here, 2026-10-03.
pub const THE_READ_AT_FIELD: &str = "read at";
/// The clock [`THE_READ_AT_FIELD`] was taken on.
pub const THE_READ_ON_CLOCK_FIELD: &str = "read on the clock of";

/// **The event a job record's write is journalled under** — `Queue & History`
/// Q3: the record and the note written in one act, *"then the event
/// journalled"*, about the job, saying the queue state it was written with.
pub const THE_JOB_RECORD_WRITTEN_EVENT: &str = "a job record was written";

/// **The event a hold's placing is journalled under** — `Rented-server
/// runtime` U26, about the hold, saying the job it holds.
pub const THE_HOLD_PLACED_EVENT: &str = "a hold was placed";

/// **The event a render attempt's start is journalled under** — `Queue &
/// History` Q2's *rendering*, written at the event that makes it true and
/// journalled, and `Queue & History` N2's started moment. About the job,
/// saying the attempt's number. Added at step 10c piece 5's second sitting.
pub const THE_RENDER_ATTEMPT_STARTED_EVENT: &str = "a render attempt started";

/// **The event the recorded reading's write is journalled under** — `Job files
/// and outputs` H9: *"the recorded reading is journalled as an event when it is
/// written, so a reader learns of it by reading forward from its own durable
/// cursor"*. About the job, saying the attempt it was written at.
///
/// **It is one of that clause's two routes and the only one that is an
/// event.** The other is the job record itself, which `State, identity and
/// recovery` §7 step 4 fills in from; **nothing polls and nothing is pushed.**
/// Added at step 10c piece 14, when the render worker's step 4 first wrote
/// one.
pub const THE_RECORDED_READING_WRITTEN_EVENT: &str = "a recorded reading was written";

/// **The event a render's recorded outcome is journalled under** —
/// `Rendering` P1 step 9: *"The supervisor writes the recorded outcome per 3.6 and
/// journals it"*, and `Queue & History` N2's ending. About the job, saying the
/// ending in the note's own words. Added at step 10c piece 5's second sitting.
pub const THE_RECORDED_OUTCOME_WRITTEN_EVENT: &str = "a render's recorded outcome was written";

/// **`State, identity and recovery` §6 step 4's event kind**: a
/// reconciliation disposition written, about the operation it was written for.
/// Journalled by the machine's program since step 8b and spelled there alone
/// until step 10c piece 7 moved it here, where the desktop's reader reads it
/// (step 10c piece 5, sitting 1, item 5). **The program re-exports this one**
/// at the path it had.
pub const THE_DISPOSITION_EVENT: &str = "a reconciliation disposition was written";

/// **`Rented-server runtime` U26's event kind for a hold's release**, about the
/// hold. Moved here with the disposition's, for the same reason; **the program
/// re-exports this one** at the path it had.
pub const THE_RELEASE_EVENT: &str = "a hold's release outcome was written";

/// **The event a job record's removal is journalled under** —
/// `Queue & History` L1's *journals the removal* for a cancel, and
/// `Queue & History` L23's *the clear's removals are journalled one by one* for
/// a clear. **About the job and nothing that names a
/// path**, saying [`IT_WAS_CLEARED`] or [`IT_WAS_REMOVED`] for which act it was.
/// The desktop, applying it at `State, identity and recovery` §7 step 3, removes
/// that job's local record. Added 2026-09-29, step 10c's first stop-and-clear piece.
pub const THE_JOB_RECORD_REMOVED_EVENT: &str = "a job record was removed";

/// **`Queue & History` L22: what [`THE_JOB_RECORD_REMOVED_EVENT`] says where
/// a waiting job was dropped at its turn because the Blender it was submitted
/// with was removed** — the leading word of [`the_drops_words`]. Added
/// 2026-10-10, the 38's piece 14c.
pub const IT_WAS_DROPPED_FOR_A_REMOVED_BLENDER: &str = "dropped, its blender was removed";

/// **`Queue & History` L22's telling, composed once for both ends**: the
/// answered version the job's mark holds and a bounded rendering of its label,
/// after [`IT_WAS_DROPPED_FOR_A_REMOVED_BLENDER`]. **It names no record and
/// carries no path.** The label comes last, so whatever it holds reads back
/// whole ([`what_a_drop_says`]).
pub fn the_drops_words(the_answered_version: &str, a_bounded_label: &str) -> String {
    format!("{IT_WAS_DROPPED_FOR_A_REMOVED_BLENDER}: {the_answered_version}: {a_bounded_label}")
}

/// **[`the_drops_words`], read back**: the answered version and the label —
/// or nothing where the event says anything else (a cancel, a clear).
pub fn what_a_drop_says(says: &str) -> Option<(String, String)> {
    let rest = says.strip_prefix(IT_WAS_DROPPED_FOR_A_REMOVED_BLENDER)?.strip_prefix(": ")?;
    let (version, label) = rest.split_once(": ")?;
    Some((version.to_string(), label.to_string()))
}

/// **The event a derivative's production is journalled under** — `Queue &
/// History` P3: *"Each derivative's production is journalled as its
/// operation's outcome, which is what the desktop's fetch triggers on."*
/// **About the job**, saying the frame, the preview operation whose record
/// holds the production, and `Queue & History` P10a's view layer name where
/// one was recorded — composed and read back by
/// `smoozy_queue_history_machine::previews::ThePublishedPreview`, so neither
/// side spells it. **Journalled only for a derivative published**: a
/// production that made nothing is its record's alone. Added 2026-10-03, the
/// 38's piece 3.
pub const THE_PREVIEW_PUBLISHED_EVENT: &str = "a preview was published";

/// **The event a live job's reporting choice changing is journalled under** —
/// `Telegram` V1c. **About the job**, saying the new value in
/// [`THE_REPORTING_CHOICE_FIELD`]'s form. A first-frame pair changed together
/// journals one event per job. Added 2026-10-30, the twenty-first notes, note 7.
pub const THE_TELEGRAM_CHOICE_CHANGED_EVENT: &str = "a job's Telegram reporting was changed";

// ---- the setup run's answers, named at step 10c piece 8.

/// **`Server Setup` Y3's boundary the step reached**, one of
/// [`EVERY_SETUP_BOUNDARY`]. **Absent where the step reached none**, which is a
/// step that failed and a step that is the worker's part of a
/// `Server Setup` R1 step whose
/// boundary the desktop writes.
///
/// **It is a request field too, since step 10c's boundary route for
/// `Server Setup` Y7**: it is
/// what [`THE_BOUNDARY_IS_WRITTEN_REQUEST`] names, and what one entry of the
/// run's composed list and of its reached list is read by.
pub const THE_BOUNDARY_FIELD: &str = "boundary";
/// **`Server Setup` Y4a's mark beside the boundary**, from the closed set of
/// two: [`IT_PERFORMED`] or [`IT_HAD_NOTHING_TO_DO`].
pub const WHAT_THE_STEP_DID_FIELD: &str = "what the step did";
/// `Server Setup` Y4a's first.
pub const IT_PERFORMED: &str = "performed";
/// `Server Setup` Y4a's second. **Not a failure and not a third mark** — the set is closed at
/// two and no third is invented.
pub const IT_HAD_NOTHING_TO_DO: &str = "nothing to do";
/// **`Server Setup` Y3's moment on the machine's own clock**, whole seconds since 1970.
pub const THE_BOUNDARY_AT_FIELD: &str = "boundary at";
/// The clock that moment was taken on (`State, identity and recovery` D5).
pub const THE_BOUNDARY_CLOCK_FIELD: &str = "boundary on the clock of";

/// **Whether the step this command named ran to its end.**
pub const THE_STEP_FINISHED_FIELD: &str = "the step finished";
/// It did, and [`THE_BOUNDARY_FIELD`] says where the run now stands.
pub const THE_STEP_RAN_TO_ITS_END: &str = "it ran to its end";
/// It did not, and [`THE_MACHINES_OWN_WORDS_FIELD`] carries `Server Setup`
/// S2a's words where any were printed.
pub const THE_STEP_DID_NOT: &str = "it did not";
/// **The worker does not perform that step**, the two being
/// [`THE_READINGS_STEP`] and [`THE_TELEGRAM_MATERIAL_STEP`], whose reasons are
/// at [`EVERY_SETUP_STEP_THE_WORKER_IS_COMMANDED`]. **Refused by name, never
/// answered emptily.**
pub const THE_WORKER_DOES_NOT_PERFORM_THAT_STEP: &str = "the worker does not perform that step";

/// **`Server Setup` R1 item 4's whole not-found list, counted**, each entry
/// then carried as the item [`A_LIBRARY_NOT_FOUND`]. **Carried by either
/// library step and by nothing else.** *"The **whole** not-found list is read
/// at once"* — never one library at a time, never from the launch error.
pub const HOW_MANY_LIBRARIES_NOT_FOUND_FIELD: &str = "libraries not found";
/// The item word for one library the chain did not resolve.
pub const A_LIBRARY_NOT_FOUND: &str = "library not found";
/// One not-found library's own name, as `ldd` printed it.
pub const THE_LIBRARY_FIELD: &str = "library";

/// **The run whose setup-in-progress state the clear removed** — what
/// [`THE_SETUP_IN_PROGRESS_IS_CLEARED_REQUEST`] answers with where the clear
/// landed.
///
/// **It is a different name from the set's answer** — `Rented-server runtime`
/// P3's first act answers with the run it *set* the state under — because the
/// two say different things and a caller reading one name for both could not
/// tell a set from a clear on a result it had lost the request for. **And it is
/// what tells a clear that landed from one the standing state refused**
/// (`Rented-server runtime` P8), which answers with [`THE_SETUP_RUN_FIELD`]
/// naming the run the state holds.
///
/// **It moved here from the machine program at step 10c**, with the two request
/// names above and for the same reason: the desktop that sends the clear has to
/// read the answer, and it cannot reach that program at all.
pub const THE_SETUP_RUN_THE_CLEAR_REMOVED_FIELD: &str = "the setup run this clear removed";

/// **The version the staged build answered** (`Server Setup` B1's
/// `Blender ` line), carried by the publication step so the desktop sees what
/// was required rather than being told only that it matched.
pub const THE_VERSION_ANSWERED_FIELD: &str = "version answered";
/// **Where the build was published** (`Server Setup` B8's rename target),
/// carried by the publication step. It is what `Server Setup` B8a's install
/// record joins on and what `Server Setup` B10's search will find.
pub const THE_PUBLISHED_AT_FIELD: &str = "published at";

/// **`Server Setup` Y7b and `Server Setup` Y7c: how far the step in hand has got, as the
/// worker last counted it** — the bytes written to the staged archive while
/// the fetch runs, or the bytes of it fed to the unpacker while the unpack
/// runs, as a whole number in decimal. **Carried by
/// [`THE_SETUP_RUNS_STATE_REQUEST`]'s answer while the fetch or the unpack is
/// the step in hand and the worker has counted**, and by nothing else.
/// **Never the server's output** (`Server Setup` Y7a), never a boundary, never
/// durable. A machine program older than this carries none, and the step then
/// shows its name alone. Added 2026-10-18, the Creator's fourteenth notes,
/// piece U.
pub const THE_BYTES_SO_FAR_FIELD: &str = "bytes so far";
/// **Beside [`THE_BYTES_SO_FAR_FIELD`]: the whole it is counted against**, in
/// decimal — the size the download's answer announced (`Server Setup` Y7b), or
/// the staged archive's own size (`Server Setup` Y7c). **Absent where the
/// download announced none.**
pub const THE_BYTES_IN_THE_WHOLE_FIELD: &str = "bytes in the whole";

/// **`Server Setup` Y7a item 2: what a worker step's commands printed on their
/// ordinary stream**, every command's in the order the commands ran, whole and
/// unedited. **Carried twice, on two routes**: by the worker's report of the
/// step to the supervisor, and by [`THE_SETUP_RUNS_STATE_REQUEST`]'s answer
/// beside [`THE_STEP_THE_WORDS_BELONG_TO_FIELD`], from the step's report until
/// the run's next step is commanded or the next setup run opens. **Never
/// written onto the machine and never Smoozy's own words** (`Server Setup`
/// Y7a item 3): a failed step's `Server Setup` S2a words still ride
/// [`THE_MACHINES_OWN_WORDS_FIELD`] as before. A machine program older than this carries none. Added 2026-10-24,
/// the Creator's ruling on the sixteenth notes, note 1.
///
/// **From 2026-10-25 it is a piece** (the eighteenth notes, piece 2b): on the
/// state reading's answer, what the step printed after the length the desktop
/// named in [`THE_ORDINARY_WORDS_HELD_FIELD`] — from the step's start, while
/// it runs — up to the room under the largest frame, with
/// [`MORE_OF_THE_STEPS_WORDS_FIELD`] where more follows. The step's words are
/// the worker's own progress lines and a failing step's own sentence as well
/// as its programs' (`Server Setup` Y7a item 2, as changed on 2026-10-24).
pub const THE_STEPS_ORDINARY_WORDS_FIELD: &str = "what the step printed";
/// **The same, for the commands' complaint stream**, kept apart from the
/// ordinary one and never merged with it (`Security and licence` §13 rule 5).
pub const THE_STEPS_COMPLAINT_WORDS_FIELD: &str = "what the step printed on its error stream";
/// **Which worker step the words on [`THE_SETUP_RUNS_STATE_REQUEST`]'s answer
/// belong to**, one of [`EVERY_SETUP_STEP_THE_WORKER_IS_COMMANDED`] — apart
/// from [`THE_STEP_FIELD`], which on that answer names the step in flight.
pub const THE_STEP_THE_WORDS_BELONG_TO_FIELD: &str = "the step the words belong to";
/// **`Server Setup` Y7a item 5: a step's words were not kept**, carried in
/// their place where the words would have passed
/// [`crate::framing::THE_LARGEST_FRAME`] — on the worker's report, or on the
/// run's state reading. **Whole or not at all** (`Security and licence` §13
/// rule 5): never a part of either stream. Its one value is
/// [`THE_WORDS_WERE_NOT_KEPT`].
pub const THE_STEPS_WORDS_WERE_NOT_KEPT_FIELD: &str = "the step's words";
/// [`THE_STEPS_WORDS_WERE_NOT_KEPT_FIELD`]'s one value.
///
/// **From 2026-10-25 it says the rest of the step's words were not kept**
/// (`Server Setup` Y7a item 5, as changed on 2026-10-24): it rides the state
/// reading beside the pieces once the step's words passed what the supervisor
/// may hold, and the words already held still come.
pub const THE_WORDS_WERE_NOT_KEPT: &str = "not kept";

/// **Whose words a state reading's pieces are** — `Server Setup` Y7a item 2:
/// the step command they belong to, as its identity's text, so a desktop can
/// tell one commanding of a step from the next. **Carried both ways**: on
/// [`THE_SETUP_RUNS_STATE_REQUEST`]'s answer beside the pieces, and on the
/// request beside the two lengths the desktop names, which it names only for
/// the turn the answer gave. Added 2026-10-25, the eighteenth notes, piece 2b.
pub const THE_STEPS_WORDS_TURN_FIELD: &str = "the step's words' turn";
/// **How much of what the step printed on its ordinary stream the desktop
/// already holds**, in bytes of text, decimal — `Server Setup` Y7a item 2's
/// *"the reading request names those two lengths"*. Added 2026-10-25.
pub const THE_ORDINARY_WORDS_HELD_FIELD: &str = "what the step printed, held";
/// **The same, for its error stream.** Added 2026-10-25.
pub const THE_COMPLAINT_WORDS_HELD_FIELD: &str = "what the step printed on its error stream, held";
/// **`Server Setup` Y7a item 5: more of the step's words follow what this
/// reading carries**, which the next reading brings. Its one value is
/// [`THEY_FOLLOW`]. A desktop keeps a step's last piece before it issues
/// anything further, so it reads again at once where a step has ended and
/// more follows. Added 2026-10-25.
pub const MORE_OF_THE_STEPS_WORDS_FIELD: &str = "more of the step's words";
/// [`MORE_OF_THE_STEPS_WORDS_FIELD`]'s one value.
pub const THEY_FOLLOW: &str = "they follow";

// ---------------------------------------------------------------------------
// `Server Setup` B7, `Server Setup` B7a and `Server Setup` B8a: a build the user pointed at (the 38's piece
// 14a, 2026-10-08)
// ---------------------------------------------------------------------------

/// **The archive the user uploaded, in place of a fetch** — its bare file
/// name, carried on the unpack and publication commands of a run that installs
/// the user's own build. **It lands in this run's own staging area** (the
/// runner's ruling on piece 14, 2026-10-08), at
/// `<the run's staging area>/<the run>-<this name>`; the staging area itself is
/// carried by [`THE_SETUP_RUNS_STAGING_AREA_FIELD`]. **A name that is not bare
/// is refused**, so nothing outside the staging area is ever unpacked. A run
/// carrying it fetches nothing.
pub const THE_USERS_OWN_ARCHIVE_FIELD: &str = "the user's own archive";
/// **Where an open setup run's staging area is on the machine**, carried by
/// the state reading of a run this supervisor holds open, so the desktop can
/// upload the user's archive into it ([`THE_USERS_OWN_ARCHIVE_FIELD`]).
pub const THE_SETUP_RUNS_STAGING_AREA_FIELD: &str = "the run's staging area";

/// **The step is held for a person** — a third answer of
/// [`THE_STEP_FINISHED_FIELD`], `Server Setup` B7a's. The user's build
/// answered a version below 4.5 ([`THE_VERSION_ANSWERED_FIELD`]), nothing was
/// published, the build stays in staging, and the run waits at its last
/// boundary for the publication to be commanded again with
/// [`THE_USERS_ANSWER_FIELD`]. **Not a failure and not a boundary.**
pub const THE_STEP_IS_HELD_FOR_A_PERSON: &str = "it is held for a person";
/// **The below-4.5 version a held run is asking the user about**, carried by
/// the state reading while the question stands.
pub const THE_QUESTION_STANDS_AT_VERSION_FIELD: &str = "the question stands at version";
/// **The user's answer to `Server Setup` B7a's question**, carried on the
/// publication command that follows a hold.
pub const THE_USERS_ANSWER_FIELD: &str = "the user's answer";
/// *Install anyway*: the build is published under the version it answered.
pub const INSTALL_ANYWAY: &str = "install anyway";
/// The way out: the staged build is discarded and never published; the
/// publication is struck from the run's composed list and the run goes on to
/// *done*.
pub const THE_USER_DECLINED: &str = "declined";
/// **`Server Setup` B7a's recorded discard**, in Smoozy's own words, carried
/// by the publication step's report and then by the state reading.
pub const THE_DISCARD_FIELD: &str = "the staged build was discarded";

/// **`Server Setup` B8a: which route placed the published build**, carried by
/// the state reading once a publication ran. One of the two words below.
pub const THE_INSTALL_ROUTE_FIELD: &str = "install route";
/// The build came from Blender's published listing — no tag.
pub const FROM_THE_PUBLISHED_LISTING: &str = "the published listing";
/// The build is one the user pointed at on their own computer — the design's
/// *your build*.
pub const YOUR_BUILD: &str = "your build";
/// **The published build's executable**, which `Server Setup` B8a joins on — carried by the
/// state reading beside [`THE_INSTALL_ROUTE_FIELD`].
pub const THE_INSTALLED_EXECUTABLE_FIELD: &str = "installed executable";

/// **The run's own kind**, one of [`EVERY_KIND_OF_SETUP`], carried by the
/// state reading.
pub const THE_RUNS_KIND_FIELD: &str = "the run's kind";
/// **`Server Setup` Y4's composed list, counted** — *what it consisted of* —
/// each entry then carried as the item [`A_COMPOSED_BOUNDARY`].
pub const HOW_MANY_COMPOSED_BOUNDARIES_FIELD: &str = "composed boundaries";
/// The item word for one entry of the composed list.
pub const A_COMPOSED_BOUNDARY: &str = "composed boundary";
/// **`Server Setup` Y3's boundaries actually reached, counted**, each then
/// carried as the item [`A_BOUNDARY_REACHED`] with its mark, its moment and
/// its clock. **The last of these is the marker.**
pub const HOW_MANY_BOUNDARIES_REACHED_FIELD: &str = "boundaries reached";
/// The item word for one boundary reached.
pub const A_BOUNDARY_REACHED: &str = "boundary reached";
/// **The run's recorded ending, where one was recorded** — `Server Setup` S2a:
/// *"the ending never waits on the words"*, so this may stand with no words
/// beside it.
pub const THE_RUNS_ENDING_FIELD: &str = "the run's ending";
/// `Server Setup` Y7's ending for a run whose refresh gap elapsed.
pub const IT_ENDED_AT_ITS_BOUNDARY_NOT_COMPLETED: &str = "ended at its boundary, not completed";
/// A step failed, and the failed step's own words ride
/// [`THE_MACHINES_OWN_WORDS_FIELD`] where any were printed (`Server Setup`
/// S2a).
pub const A_STEP_FAILED: &str = "a step failed";
/// **Which step failed, named by the first of `Server Setup` Y4's boundaries
/// that step reaches** — carried by [`THE_FAILED_STEP_IS_RECORDED_REQUEST`] and
/// answered beside [`A_STEP_FAILED`] by the run's state reading.
///
/// **Why a boundary's word names a step.** `Server Setup` S2a's ending *names
/// the failed step*, and the wire's own step words
/// ([`EVERY_SETUP_STEP_THE_WORKER_IS_COMMANDED`]) are the worker's only — the
/// elevated steps, which are the desk's, have none. **Every one of
/// `Server Setup` R1's steps reaches at least one of `Server Setup` Y4's ten**, the first of
/// them is that step's alone, and `Server Setup`'s own `Boundary::its_step`
/// reads it back — so no third list of words is invented for it.
pub const THE_FAILED_STEP_FIELD: &str = "the step that failed";
/// `Server Setup` Y8's stop, honoured at a boundary.
pub const IT_WAS_STOPPED: &str = "left stopped";
/// The run reached *done*.
pub const IT_COMPLETED: &str = "completed";
/// **`Server Setup` S6's second clear: the recovery the user asked to abandon
/// the machine-changing work, ended.** Added 2026-09-22.
///
/// **It is the run's ending whatever its final verification found**, because
/// `Server Setup` S6 has the abandon *"still run the final verification first and report what
/// it found"* — the finding is what the abandon reports, not a step it failed —
/// and the finding's words ride [`THE_MACHINES_OWN_WORDS_FIELD`] where the
/// verification did not pass. **It never reaches *done***, so `Server Setup`
/// `Server Setup` Y6a's card reads the server as not set up.
pub const IT_WAS_ABANDONED: &str = "abandoned";

/// **`Server Setup` Y4's boundaries, as the wire spells them**, in
/// `Server Setup` R1's order. **The words are Build's and the set is `Server Setup` Y4's**;
/// `Server Setup`'s own `run::Boundary` is the same ten and the program holds
/// this list to it by name.
pub const THE_BOUNDARY_STARTED: &str = "started";
/// `Server Setup` Y4's second.
pub const THE_BOUNDARY_FETCHED: &str = "fetched";
/// `Server Setup` Y4's third.
pub const THE_BOUNDARY_TOOLING: &str = "tooling";
/// `Server Setup` Y4's fourth.
pub const THE_BOUNDARY_LIBRARIES: &str = "libraries";
/// `Server Setup` Y4's fifth.
pub const THE_BOUNDARY_DRIVER_PLACED: &str = "driver placed";
/// `Server Setup` Y4's sixth.
pub const THE_BOUNDARY_RESTART_ISSUED: &str = "restart issued";
/// `Server Setup` Y4's seventh.
pub const THE_BOUNDARY_CARD_ANSWERING: &str = "card answering";
/// `Server Setup` Y4's eighth.
pub const THE_BOUNDARY_BLENDER: &str = "blender";
/// **`Server Setup` B15's removal reached its end: the install's folder is
/// gone.** A removal run's own boundary, between the eighth and the ninth.
/// Added 2026-10-10, the 38's piece 14c.
pub const THE_BOUNDARY_REMOVED: &str = "removed";
/// `Server Setup` Y4's ninth.
pub const THE_BOUNDARY_TELEGRAM: &str = "telegram";
/// `Server Setup` Y4's tenth.
pub const THE_BOUNDARY_DONE: &str = "done";

/// **`Server Setup` Y4's full set, closed, in `Server Setup` R1's order.**
pub const EVERY_SETUP_BOUNDARY: [&str; 11] = [
    THE_BOUNDARY_STARTED,
    THE_BOUNDARY_FETCHED,
    THE_BOUNDARY_TOOLING,
    THE_BOUNDARY_LIBRARIES,
    THE_BOUNDARY_DRIVER_PLACED,
    THE_BOUNDARY_RESTART_ISSUED,
    THE_BOUNDARY_CARD_ANSWERING,
    THE_BOUNDARY_BLENDER,
    THE_BOUNDARY_REMOVED,
    THE_BOUNDARY_TELEGRAM,
    THE_BOUNDARY_DONE,
];

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use super::*;
    use crate::framing::{
        THE_FIELD_NAME_RESERVED_FOR_A_CALLERS_OWN_IDENTITY,
        THE_FIELD_NAME_RESERVED_FOR_A_PROTOCOL_VERSION_THIS_PACKAGE_DOES_NOT_SPEAK,
        THE_FIELD_NAME_RESERVED_FOR_A_REQUEST_THIS_PACKAGE_DOES_NOT_ANSWER,
    };

    const THIS_FILE: &str = include_str!("requests.rs");

    fn the_workspace() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
    }

    /// Every request constant this file declares, as (its name in the code, the
    /// value it spells), **read out of the file's own text** so the list above
    /// cannot leave one out.
    fn the_request_constants_declared_here() -> Vec<(String, String)> {
        let mut out = Vec::new();
        for line in THIS_FILE.lines() {
            let Some(rest) = line.strip_prefix("pub const ") else {
                continue;
            };
            let Some((name, value)) = rest.split_once(": &str = ") else {
                continue;
            };
            if !name.ends_with("_REQUEST") {
                continue;
            }
            let value = value
                .trim_end_matches(';')
                .trim_matches('"')
                .to_string();
            out.push((name.to_string(), value));
        }
        out
    }

    /// Every `.rs` file of the workspace that is not a test, with its text cut
    /// at its test module.
    ///
    /// **What counts as a test, and what that cannot see.** A folder named
    /// `tests` is skipped whole, and so is a folder named `target`. Inside a
    /// file, everything from a `#[cfg(test)]` line followed by a block module
    /// (`mod <name> {`) to the end of the file is cut, which is where this
    /// workspace keeps its unit tests. **A file that puts working code after its
    /// test module would have that code missed**, and a spelling there would
    /// escape this check.
    fn every_working_file() -> Vec<(PathBuf, String)> {
        fn walk(folder: &Path, out: &mut Vec<(PathBuf, String)>) {
            let Ok(entries) = fs::read_dir(folder) else {
                return;
            };
            for entry in entries {
                let path = entry.expect("the entry is readable").path();
                let named = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if path.is_dir() {
                    if named == "target" || named == "tests" || named.starts_with('.') {
                        continue;
                    }
                    walk(&path, out);
                } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    let text = fs::read_to_string(&path).expect("the file is readable");
                    out.push((path, the_part_before_the_test_module(&text)));
                }
            }
        }
        let mut out = Vec::new();
        walk(&the_workspace(), &mut out);
        assert!(
            out.len() > 100,
            "the workspace's files were not found, so nothing below would be checked"
        );
        out
    }

    fn the_part_before_the_test_module(text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();
        for (at, line) in lines.iter().enumerate() {
            if line.trim() != "#[cfg(test)]" {
                continue;
            }
            let next = lines[at + 1..].iter().find(|l| !l.trim().is_empty());
            if let Some(next) = next {
                let next = next.trim();
                if next.starts_with("mod ") && next.ends_with('{') {
                    return lines[..at].join("\n");
                }
            }
        }
        text.to_string()
    }

    /// **The list above names every request constant in this file, and
    /// nothing else.**
    /// **`Server protocol and transfers` §1.1's four-value request, 2026-11-04:
    /// any path's bytes cross whole, and each answer reads back as itself.**
    #[test]
    fn the_paths_and_their_values_read_back_as_written() {
        let paths = vec![b"/a b/c".to_vec(), vec![b'/', 0xff, b'\n', b'x'], b"/".to_vec()];
        let field = the_paths_written(&paths);
        assert_eq!(the_paths_read_back(&field), vec![paths[0].as_slice(), paths[1].as_slice(), paths[2].as_slice()]);
        assert!(the_paths_read_back(&[]).is_empty());

        let answers = vec![
            OnePathAnswered::Values { size: 0, inode: u64::MAX, modification_time: i64::MIN, change_time: -1 },
            OnePathAnswered::NothingThere,
            OnePathAnswered::NotRead("Permission denied (os error 13)".to_string()),
        ];
        let written = the_values_written(&answers);
        assert_eq!(the_values_read_back(&written, 3), Some(answers.clone()));
        // **A count that is not the count asked, or a short line, is no answer.**
        assert_eq!(the_values_read_back(&written, 2), None);
        assert_eq!(the_values_read_back("4096 912 1\n", 1), None);
        assert_eq!(the_values_read_back("4096 912 1 2 3\n", 1), None);
    }

    /// **The brief's *very long list*: one frame is the limit, so a long list
    /// goes in several asks, none past either limit, none empty, in its order —
    /// and the worst answer to the largest ask still fits one frame.**
    #[test]
    fn a_long_list_is_split_so_every_ask_and_every_answer_fits_one_frame() {
        use crate::framing::THE_LARGEST_FRAME;
        let by_count: Vec<Vec<u8>> = (0..THE_MOST_PATHS_IN_ONE_ASK * 2 + 1).map(|at| format!("/f/{at}").into_bytes()).collect();
        let asks = the_asks_over(&by_count);
        assert_eq!(asks.iter().map(|ask| ask.len()).collect::<Vec<_>>(), vec![4_000, 4_000, 1]);
        assert_eq!(asks.concat(), by_count, "in its own order, nothing lost");

        let by_size = vec![vec![b'x'; 4096]; 300];
        let asks = the_asks_over(&by_size);
        assert_eq!(asks.len(), 3, "128 of 4,097 bytes go in 512 KiB");
        for ask in &asks {
            assert!(the_paths_written(ask).len() <= THE_MOST_PATH_BYTES_IN_ONE_ASK);
        }
        assert_eq!(asks.concat(), by_size);
        assert!(the_asks_over(&[]).is_empty());

        // **The worst answer**: every path refused, with the longest words.
        let worst = vec![OnePathAnswered::NotRead("x".repeat(10_000)); THE_MOST_PATHS_IN_ONE_ASK];
        let written = the_values_written(&worst);
        assert!(written.len() < THE_LARGEST_FRAME - 4096, "{} bytes", written.len());
        assert!(THE_MOST_PATH_BYTES_IN_ONE_ASK < THE_LARGEST_FRAME - 4096);
    }

    #[test]
    fn the_list_holds_every_request_this_file_names() {
        let declared: Vec<String> = the_request_constants_declared_here()
            .into_iter()
            .map(|(_, value)| value)
            .collect();
        let listed: Vec<String> = EVERY_REQUEST_NAMED_HERE
            .iter()
            .map(|request| request.name.to_string())
            .collect();
        assert_eq!(declared, listed, "the list and the file's own constants have come apart");
    }

    /// **Each name is spelled once in the whole workspace outside the tests,
    /// and its constant is declared once.**
    ///
    /// This is the fault the file exists to end: `THE_HEALTH_ANSWER_REQUEST`
    /// was written twice, with a test holding one copy to the other. **A second
    /// spelling of any name here fails**, in any crate, whether as a second
    /// constant or as the bare value written into working code.
    #[test]
    fn every_name_is_spelled_once_in_the_workspace_outside_the_tests() {
        let files = every_working_file();
        let mut faults = Vec::new();
        for (constant, value) in the_request_constants_declared_here() {
            let spelled = format!("\"{value}\"");
            let declared = format!("const {constant}:");
            let mut spellings = Vec::new();
            let mut declarations = Vec::new();
            for (path, text) in &files {
                for _ in text.matches(&spelled) {
                    spellings.push(path.display().to_string());
                }
                for _ in text.matches(&declared) {
                    declarations.push(path.display().to_string());
                }
            }
            if spellings.len() != 1 {
                faults.push(format!("{value} is spelled {} times: {spellings:?}", spellings.len()));
            }
            if declarations.len() != 1 {
                faults.push(format!(
                    "{constant} is declared {} times: {declarations:?}",
                    declarations.len()
                ));
            }
        }
        assert!(faults.is_empty(), "{faults:#?}");
    }

    /// The body of one of the program's two lists, from its declaration to the
    /// end of the array.
    fn the_body_of<'a>(program: &'a str, list: &str) -> &'a str {
        let opens = format!("pub const {list}:");
        let after = program
            .split_once(&opens)
            .unwrap_or_else(|| panic!("the program declares no {list}"))
            .1;
        after
            .split_once("\n    ];")
            .unwrap_or_else(|| panic!("{list} has no end the reading can find"))
            .0
    }

    /// **Every name here is on the machine program's record of what it knows
    /// about, and a refused one is on both of its lists — never on one alone.**
    ///
    /// The program's own guard holds its two lists to each other by their
    /// words, and its dispatch to the answered half. **What it cannot do is
    /// notice a request nobody wrote down** — its own documentation says so,
    /// and the read forward was one. This holds the other end: every request
    /// this file names is written down there, by the name of its constant.
    ///
    /// - **Answered**: on the record as answered, under its constant's name,
    ///   and not on the list of refusals.
    /// - **Refused**: on the record as refused, and on the list of refusals.
    ///
    /// **What it reads is the program's source as text**, the way that
    /// program's own guard reads its dispatch; it compiles nothing, and an
    /// entry that names a constant only in a comment would be read as naming
    /// it.
    #[test]
    fn every_name_here_is_on_the_programs_record_and_a_refused_one_is_on_both_of_its_lists() {
        let program = fs::read_to_string(
            the_workspace().join("Rented-server runtime — program/src/main.rs"),
        )
        .expect("the machine program's source is readable");
        let the_record = the_body_of(&program, "EVERY_REQUEST_THIS_MODULE_KNOWS_ABOUT");
        let the_refusals = the_body_of(&program, "WHAT_IS_NOT_WIRED_AND_WHO_OWNS_IT");
        let entries: Vec<&str> = the_record
            .split("ARequestThisModuleKnowsAbout {")
            .skip(1)
            .collect();
        assert!(!entries.is_empty(), "the program's record was not read");

        let mut faults = Vec::new();
        for (constant, _) in the_request_constants_declared_here() {
            let quoted = format!("`{constant}`");
            let answered = format!("AnsweredHere(\"{constant}\")");
            let on_the_record: Vec<&&str> = entries
                .iter()
                .filter(|entry| entry.contains(&quoted) || entry.contains(&answered))
                .collect();
            let on_the_refusals = the_refusals.contains(&quoted);
            match on_the_record.as_slice() {
                [] => faults.push(format!(
                    "{constant} is not on the program's record of what it knows about{}",
                    if on_the_refusals { ", though it is on its list of refusals" } else { "" }
                )),
                [entry] => {
                    let refused = entry.contains("WhereARequestStands::RefusedAndOnTheListAbove");
                    if refused && !on_the_refusals {
                        faults.push(format!(
                            "{constant} is recorded as refused and is not on the list of refusals"
                        ));
                    }
                    if !refused && on_the_refusals {
                        faults.push(format!(
                            "{constant} is on the list of refusals and is not recorded as refused"
                        ));
                    }
                }
                many => faults.push(format!(
                    "{constant} is on the program's record {} times",
                    many.len()
                )),
            }
        }
        assert!(faults.is_empty(), "{faults:#?}");
    }

    /// **P5: the one field kind that is forbidden is a field naming the
    /// caller's own identity** — and no field here may take a name the frame
    /// reserves for a result's own statements either.
    ///
    /// **And no request carries one field twice**, because a list read by name
    /// would take whichever it met first.
    #[test]
    fn no_field_named_here_is_one_the_frame_reserves_and_none_is_carried_twice() {
        let reserved = [
            THE_FIELD_NAME_RESERVED_FOR_A_CALLERS_OWN_IDENTITY,
            THE_FIELD_NAME_RESERVED_FOR_A_REQUEST_THIS_PACKAGE_DOES_NOT_ANSWER,
            THE_FIELD_NAME_RESERVED_FOR_A_PROTOCOL_VERSION_THIS_PACKAGE_DOES_NOT_SPEAK,
        ];
        for request in EVERY_REQUEST_NAMED_HERE {
            let mut seen = std::collections::HashSet::new();
            for field in request.carries {
                assert!(
                    !reserved.contains(field),
                    "{} carries {field}, which the frame reserves",
                    request.name
                );
                assert!(seen.insert(*field), "{} carries {field} twice", request.name);
            }
        }
    }

    /// **No two fields named in this file share a name, and none takes a
    /// name the frame reserves** — requests' and answers' alike, read out of
    /// the file's own text so a field added later is held too.
    #[test]
    fn every_field_named_here_is_named_once_and_none_is_reserved() {
        let reserved = [
            THE_FIELD_NAME_RESERVED_FOR_A_CALLERS_OWN_IDENTITY,
            THE_FIELD_NAME_RESERVED_FOR_A_REQUEST_THIS_PACKAGE_DOES_NOT_ANSWER,
            THE_FIELD_NAME_RESERVED_FOR_A_PROTOCOL_VERSION_THIS_PACKAGE_DOES_NOT_SPEAK,
        ];
        let mut seen = std::collections::HashMap::new();
        for line in THIS_FILE.lines() {
            let Some(rest) = line.strip_prefix("pub const ") else {
                continue;
            };
            let Some((name, value)) = rest.split_once(": &str = ") else {
                continue;
            };
            if !name.ends_with("_FIELD") {
                continue;
            }
            let value = value.trim_end_matches(';').trim_matches('"').to_string();
            assert!(!reserved.contains(&value.as_str()), "{name} takes a reserved name");
            if let Some(before) = seen.insert(value.clone(), name.to_string()) {
                panic!("{name} and {before} are both named {value:?}");
            }
        }
        assert!(seen.len() > 30, "the fields were not read");
    }

    /// **An item's field name is composed one way**, and it cannot be read
    /// back as a plain field of the same word.
    #[test]
    fn an_items_field_is_composed_once_and_is_not_a_plain_field() {
        assert_eq!(the_field_of_item(AN_EVENT, 3, THE_KIND_FIELD), "event 3: kind");
        assert_ne!(the_field_of_item(THE_JOB_FIELD, 0, THE_JOB_FIELD), THE_JOB_FIELD);
    }

    /// **C6 in both directions, over the requests the plan names by class.**
    ///
    /// C6 names *"a submission"* among what is recorded before it acts, and a
    /// hardware reading among the readings; the submission and the health
    /// answer are held to those classes here. **The others are classed at their
    /// own constants and are held by nothing but this list** — which of them
    /// acts is a reading of their owning documents, not something a test can
    /// derive.
    #[test]
    fn the_submission_is_recorded_before_it_acts_and_the_health_answer_is_a_reading() {
        let class_of = |name: &str| {
            EVERY_REQUEST_NAMED_HERE
                .iter()
                .find(|request| request.name == name)
                .map(|request| request.under_c6)
        };
        assert_eq!(
            class_of(THE_JOB_RECORD_IS_WRITTEN_REQUEST),
            Some(UnderC6::RecordedBeforeItActs)
        );
        assert_eq!(class_of(THE_HEALTH_ANSWER_REQUEST), Some(UnderC6::APureReading));
    }

    /// **C6's own sentence for a job's kept crash files: one more pure
    /// reading** (`Reporting a problem` G5a), and it carries the job alone.
    #[test]
    fn a_jobs_kept_crash_files_are_a_pure_reading_of_one_job() {
        let named = EVERY_REQUEST_NAMED_HERE
            .iter()
            .find(|request| request.name == THE_KEPT_CRASH_FILES_REQUEST)
            .expect("it is on the list");
        assert_eq!(named.under_c6, UnderC6::APureReading);
        assert_eq!(named.carries, &[THE_JOB_FIELD]);
    }

    /// **`Server Setup` Y4a: the boundary and its mark are one act, so the
    /// request that carries a boundary carries the mark with it** — and the
    /// mark's set is closed at two, with no third to invent.
    ///
    /// **And the moment is not among the fields**, because `Server Setup` Y3
    /// puts it on the machine's own clock and
    /// `State, identity and recovery` D5 forbids reading one clock into
    /// another.
    #[test]
    fn the_boundary_write_carries_y4as_mark_and_never_a_moment() {
        let carries = EVERY_REQUEST_NAMED_HERE
            .iter()
            .find(|request| request.name == THE_BOUNDARY_IS_WRITTEN_REQUEST)
            .expect("the boundary write is named here")
            .carries;
        assert!(carries.contains(&THE_BOUNDARY_FIELD));
        assert!(carries.contains(&WHAT_THE_STEP_DID_FIELD));
        assert!(carries.contains(&THE_SETUP_RUN_FIELD));
        assert!(!carries.contains(&THE_BOUNDARY_AT_FIELD));
        assert!(!carries.contains(&THE_BOUNDARY_CLOCK_FIELD));
        // The closed set of two, spelled once each.
        assert_ne!(IT_PERFORMED, IT_HAD_NOTHING_TO_DO);
    }

    /// **One field, two spellings, and no word in both** — the step under way
    /// on the run's state reading is a commanded worker step's own name or an
    /// admin step's first boundary ([`THE_STEP_FIELD`]), and a word in both
    /// sets would read as either. Added 2026-09-29, step 10c's transfer wave
    /// piece 12.
    #[test]
    fn no_worker_steps_name_is_also_a_boundarys() {
        for step in EVERY_SETUP_STEP_THE_WORKER_IS_COMMANDED {
            assert!(
                !EVERY_SETUP_BOUNDARY.contains(&step),
                "{step} names a worker step and a boundary"
            );
        }
    }

    /// **`Server Setup` Y7's second bullet: an admin step's start names the
    /// run and the step, and nothing else** — no moment, which is the
    /// machine's own clock's (`Server Setup` Y3), and no mark, which is the
    /// boundary's (`Server Setup` Y4a).
    #[test]
    fn an_admin_steps_start_names_the_run_and_the_step_and_nothing_else() {
        let request = EVERY_REQUEST_NAMED_HERE
            .iter()
            .find(|request| request.name == THE_ELEVATED_STEPS_START_IS_RECORDED_REQUEST)
            .expect("the start is named here");
        assert_eq!(request.carries, &[THE_SETUP_RUN_FIELD, THE_BOUNDARY_FIELD]);
        assert_eq!(request.under_c6, UnderC6::RecordedBeforeItActs);
    }

    /// **`Queue & History` L13: each half of stop-and-clear names the other, and
    /// the stop names no target** — so whichever half arrives first can have
    /// both recorded before anything acts, and nothing the desktop's reading
    /// last showed can become the stop's subject. **C21's own stop names its
    /// target and nothing else.** Added 2026-09-29, step 10c's first
    /// stop-and-clear piece.
    #[test]
    fn each_half_of_the_pair_names_the_other_and_the_paired_stop_names_no_target() {
        let carries = |name: &str| {
            EVERY_REQUEST_NAMED_HERE
                .iter()
                .find(|request| request.name == name)
                .expect("named here")
                .carries
        };
        assert!(carries(THE_CLEAR_REQUEST).contains(&THE_PAIRED_STOP_FIELD));
        assert!(carries(THE_STOP_PAIRED_WITH_A_CLEAR_REQUEST).contains(&THE_PAIRED_CLEAR_FIELD));
        assert!(!carries(THE_STOP_PAIRED_WITH_A_CLEAR_REQUEST).contains(&THE_OPERATION_TO_STOP_FIELD));
        assert_eq!(carries(THE_STOP_REQUEST), &[THE_OPERATION_TO_STOP_FIELD]);
        for name in [
            THE_CLEAR_REQUEST,
            THE_STOP_PAIRED_WITH_A_CLEAR_REQUEST,
            THE_STOP_REQUEST,
            THE_CANCEL_REQUEST,
        ] {
            let request = EVERY_REQUEST_NAMED_HERE
                .iter()
                .find(|request| request.name == name)
                .expect("named here");
            assert_eq!(request.under_c6, UnderC6::RecordedBeforeItActs, "{name}");
        }
    }

    /// **`Server Setup` Y4's set is closed**, and its words are spelled once
    /// each — ten, and *removed* since the 38's piece 14c.
    #[test]
    fn the_boundaries_are_eleven_and_each_is_spelled_once() {
        let mut seen = std::collections::HashSet::new();
        for boundary in EVERY_SETUP_BOUNDARY {
            assert!(seen.insert(boundary), "{boundary} is spelled twice");
        }
        assert_eq!(seen.len(), 11);
    }

    /// **`Queue & History` L22: the drop's words read back to what composed
    /// them**, a label holding the separator included, and a cancel's or a
    /// clear's word reads as no drop. The 38's piece 14c.
    #[test]
    fn the_drops_words_read_back_and_nothing_else_reads_as_a_drop() {
        let says = the_drops_words("5.1.0", "Kitchen: final");
        assert_eq!(
            what_a_drop_says(&says),
            Some(("5.1.0".to_string(), "Kitchen: final".to_string()))
        );
        assert_eq!(what_a_drop_says(IT_WAS_REMOVED), None);
        assert_eq!(what_a_drop_says(IT_WAS_CLEARED), None);
        assert!(!says.contains('/'), "the event names no path");
    }
}
