//! What of this document is not built here, named clause by clause rather than
//! silently absent.
//!
//! **`Preparation/Build/Plan.md`, step 4, says which part of this document step
//! 4 builds**: *"the two mechanisms the pre-package surface is made of, request
//! framing, correlation so that sending a thing twice does not do it twice, and
//! the one staging-resume-integrity-publication contract that holds in both
//! directions for the machine's whole life."*
//!
//! **This list is here so that a reader of this crate meets the gap rather than
//! discovering it**, and it is the only place an absence in this document is
//! recorded. It follows the shape `Job files and outputs` uses in its own
//! `arriving_later.rs`, with the clause markers written out, because these
//! absences sit inside sections that are otherwise built and a section on its
//! own would not find them again.
//!
//! **There are three lists and they are not the same thing.**
//! [`EVERYTHING_ARRIVING_LATER`] holds promises nobody has built yet, each with
//! the step it lands at. [`NOT_THIS_CODES_AT_ALL`] holds clauses that are
//! records rather than promises, which **no step builds**. A record filed under
//! a step sends whoever builds that step looking for work that does not exist,
//! and that is why the two are kept apart.
//!
//! **[`WAITING_ON_THE_PATH_SEAM`] is the third, added 2026-09-06, and it is
//! kept apart from the first for the same reason the second is.** Its one
//! clause is a promise, so it is not a record — but what removes it is not a
//! step of `Preparation/Build/Plan.md` either. It is a piece of scheduled work
//! with its own work list, and naming a step for it would send whoever builds
//! that step looking for something that is not filed there. So the step field
//! is dropped and a *what removes it* field takes its place. **This is the
//! shape `File Browser` used for the same clause on the same day**, in that
//! crate's own `arriving_later.rs`, and it is copied rather than invented.
//!
//! **Every clause on this page whose step had already closed was read against
//! the workspace on 2026-09-06, one at a time, and the page was brought into
//! line with what the search found.** Twenty-six clauses were filed here at
//! steps 5, 6 and 7 and named by no working file anywhere; all three steps have
//! closed. **They were not all the same thing, and lumping them together is
//! what had hidden them.**
//!
//! - **Three had landed and the entry was stale**, and each is struck below
//!   with the file that names it: **C20** at `App/Server Setup/src/driving.rs`,
//!   **T5a** at `App/Desktop app and delivery/src/scheduling.rs`, and **F10** at
//!   `App/File Browser/src/free_space.rs`. **T2** came off with T5a on the same
//!   ground, at `App/File Browser/src/pool.rs`.
//! - **Eleven are genuinely owed** — a promise the step was meant to build and
//!   did not. They are **B5**, **A6**, **C7**, **C12**, **C15**, **T11**,
//!   **T11a**, **G1**, **G2**, **G3** and **G9**, and **C9** and **C10** sit
//!   with them with half of each kept elsewhere and unnamed. Each entry below
//!   says what closed without it and what is missing.
//!
//! - **Twelve were filed under a condition that was wrong when it was written**
//!   and never belonged to the step named. Eight are the disk-full carve-out —
//!   **F6, F8, F9, F11a, F12, F14, F15, F17** — which wait on a surface no step
//!   of `Preparation/Build/Plan.md` names, and **R9** and **R14** wait on
//!   nothing that is work. Each entry below carries the true condition.
//!
//! **ALL ELEVEN WERE BUILT ON 2026-09-06 and the entries below say where each
//! landed.** Nine came first; **B5** and **A6** followed later the same day,
//! under a brief of their own. **T11** and **T11a** first, that pair being the only
//! one of the eleven where something was answering wrongly rather than
//! missing — the transfer-support kind was arbitrated under
//! `Rented-server runtime` A3's default, which calls it *not cancellable*, and
//! T11 says outright that the value is false of it. Then **C7**, **G1**,
//! **G2**, **G3** and **G9**, which are one thing between them: the machine's
//! own durable operation record, which `Preparation/Build/Plan.md` step 4
//! assigned to step 6 and step 6 did not build. Then **C12** and **C15**.
//! **Then B5 and A6, and they were not the same kind of thing.** **B5** was
//! work — the bootstrap-and-repair request itself, which existed nowhere in the
//! workspace, and it is built at [`crate::bootstrap`] with B1, B2, B3, B4, B6
//! and B7 beside it. **A6 was a naming site rather than work**: what keeps it is
//! the gate C14 and C15 compose between them, C15 having landed hours earlier,
//! and no working file named the clause. It is named at
//! [`crate::reconnection::AN_AUTHENTICATED_CONNECTION_IS_EVIDENCE_ABOUT_NOTHING_BUT_ITSELF`].
//!
//! **Entries whose clauses are partly built and correctly cited in working code
//! were not re-dated on 2026-09-06** — C11, C14, S13, R11 and R12a — because
//! those are the partly-built case
//! [`WHY_THIS_PAGE_CARRIES_NO_FORWARD_LEG_TEST`] describes and not an absence.
//! **Not re-dating them cost something and the sweep of 2026-09-07 found it**:
//! the C11 and C14 entry still said this crate had no reconnection to hang
//! C11's ordering on, and that `State, identity and recovery` §2.3 was not
//! built, hours after both stopped being true — and the S13 entry still spoke
//! of step 8 as coming. Both are corrected in place. **The lesson is that an
//! entry left alone because its clause is only partly built still has to have
//! its reason re-read**, since the reason is where the staleness collects.
//!
//! **An entry whose step has landed is struck in place rather than deleted.**
//! [`EVERYTHING_ARRIVING_LATER`] carries one — §6.3's T9 and T10, struck
//! 2026-09-05 — saying what it said, what closed it and where. Deleting it
//! would take T9 and T10 off this page altogether, and a reader of this crate
//! would then meet neither the clauses nor their answer.
//!
//! **What bounds the second list is the section gate, and only that.** A clause
//! may sit there only if it comes from a section somebody has read and found
//! holds no promise, and [`THE_SECTIONS_WHOSE_CLAUSES_MAY_BE_RECORDS`] holds
//! §8.5 alone. §8.5 holds exactly the three clauses on this page, and a test
//! reads the plan and says so, so a fourth entry has to widen that list
//! first — which is its own act.
//!
//! **The ground is a second test and a narrower one.** [`WhatQualifiesIt`] is a
//! closed set of three, and a test works the ground out from the plan itself
//! and compares it with the one the entry claims, so an entry filed under the
//! wrong ground fails rather than reads well. **The ground is not the bar and
//! must not be read as one**: the same reading gives one of the three grounds
//! for 75 of the plan's 152 clauses, which the step 4 gate's fifth run
//! measured. It catches a wrong filing; it does not keep a clause out.
//!
//! **Every clause named in either list is checked against the plan too**, which
//! is read at the test rather than copied here: the plan has to define the
//! clause, in the section the entry files it under. That is what holds the
//! markers on this page, which are written as text rather than in comments and
//! are therefore outside any sweep that looks at comments.
//!
//! **Nothing below is stubbed**: there is no always-yes answer, no empty value
//! standing in for one, and no default anywhere. The one place where a built
//! thing would otherwise hold a figure it cannot take carries a statement of
//! absence instead — [`crate::free_space::WhyThereIsNoReading`] — and it says
//! so at itself.
//!
//! **Markers used below that belong to another document, each with the
//! document it belongs to.** `Desktop app and delivery` §16.4 and M15.
//! `Rented-server runtime` V12, G25, K11, K19.
//!
//! **A marker this document also defines is never listed there, because the
//! list would then say the wrong thing about it.** Three below are of that
//! kind. Each use of one either means this document's own clause, or writes
//! the document meant out where it stands.

/// One part of the document that is not built here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotBuiltHere {
    /// The plan's own section.
    pub section: &'static str,
    /// Every clause of that section this covers.
    pub clauses: &'static [&'static str],
    /// What it is.
    pub what: &'static str,
    /// Why it cannot be built at step 4.
    pub why: &'static str,
    /// The step of `Preparation/Build/Plan.md` it was filed at.
    ///
    /// **Where that step has closed without the clause, the entry's `why`
    /// opens by saying so and names what actually removes it**, and this field
    /// is then the step it was owed at rather than a step still to come. It is
    /// left standing rather than blanked because *which step let it through* is
    /// the fact an auditor needs.
    pub it_lands_at_step: u8,
}

/// **Every part, with its clauses and the step it lands at.**
pub const EVERYTHING_ARRIVING_LATER: [NotBuiltHere; 13] = [
    NotBuiltHere {
        section: "§4.4 and §4.5",
        clauses: &["C11", "C14"],
        what: "the two gates that decide whether a transfer record may be resumed at all — which application run last wrote it, and the installation identity it was arranged under — and C14's *installation-replaced* removable state",
        why: "**CORRECTED 2026-09-07, and what is left is the driving rather than the deciding.** ~~Both fields are written into the record and read back out and neither is compared with anything; no driver that re-establishes transfer channels exists, and this crate has no reconnection to hang C11's ordering on; and C14 also needs `State, identity and recovery` §2.3's reading of the machine's installation identity, which is not built either.~~ **Three of those four sentences had stopped being true.** **This crate has a reconnection**, at `crate::reconnection`: C11's four steps in order at its `WhichStep::IN_ORDER`, C12's step-3-and-not-step-4 difference beside it, and C14's branch at its `what_a_replaced_connection_does`. **Both fields are compared**: the run that wrote the record at `App/File Browser/src/continuation.rs` and `App/Download/src/arrangement.rs`, and the installation at four sites in `App/File Browser/` and at `App/Servers/src/transfers.rs`. **§2.3's check is built**, at `App/State, identity and recovery/src/identities.rs` as `InstallationCheck`, with the reading half here at `crate::reconnection::read_the_installation_identity`; what §2.3 still waits on is the supervisor's own answer, which that module names at itself. **What is still true is the removable state**: `RemovableState::InstallationReplaced` is declared at `crate::transfers::cleanup` and nothing in this workspace produces one. **And nothing drives the four steps**: every decision function above has test callers only, so no production path runs C11's ordering on a replaced connection. **The risk if this is not written down is concrete**: whoever builds that driver reads this list, finds these two absent, concludes the gates are kept, and resumes a record an earlier run wrote over material a recycle destroyed",
        it_lands_at_step: 6,
    },
    NotBuiltHere {
        section: "§7.4",
        clauses: &["S13"],
        what: "the four-value unchanged-source reading over the command channel, and the upload half's own gate",
        why: "**the decision is built and both readings are not.** What is here is the comparison and what each of its three answers does per direction, asked at the head of the byte loop on a download resume. **The four values — size, inode, modification time and change time — are recorded *when a download is arranged*, and the module that arranges downloads is `Download`, which landed at step 8**; the reading itself is a command-channel reading, because revision 3's attribute structure carries neither the inode nor the change time, and it sits on `File Browser` §13 item 9's unestablished ground. **Corrected 2026-09-07: that step has landed and this entry has not moved with it.** `smoozy_download::arrangement::TheServerFileIdentity` is the field the four values would sit in and nothing fills it, and `smoozy_render_settings::acting::four_values` takes this very reading for another clause and yields this very type — **so the mechanism exists and what is missing is a caller at the moment a download is arranged**. With nothing recorded and nothing read the comparison answers *could not be confirmed*, which is S13's own branch: the resume does not happen and the download starts over. **The upload half is not this document's gate at all** — S13 puts the unchanged-source question for a file on the user's own computer in `File Browser`'s hands, whose own pool is Row 188's and lands at **step 5** — and applying the unavailable branch there would make every upload resume start over. **The later of the two steps is the one this is filed at**, on the same rule T9 and T10 are filed under: before step 8 the clause is missing one of its two owners",
        it_lands_at_step: 8,
    },
    NotBuiltHere {
        section: "§3",
        clauses: &["A6"],
        what: "the rule that an authenticated connection is not evidence about the machine at the other end of it",
        why: "~~**OWED. Step 6 closed without it; read 2026-09-06 and re-read later the same day.**~~ **STRUCK 2026-09-06: A6 is named and the site is `crate::reconnection`.** \
**It was a naming site rather than work, and that is the whole of what it was.** A6 says §4.5 is the whole of what this document does about the clause, and what keeps its substance is the gate C14 and C15 compose between them: C13 and C14 are named by working code at `App/Servers/src/transfers.rs`, which builds C11's four steps and runs C14's gate before anything is resumed, and **C15 landed at `crate::reconnection` earlier the same day** — the identity read on the pre-package surface from Smoozy's own state area, run against a real file-transfer server. **What was missing was that no working file named the clause**, and the sitting that built C15 left it because A6 was not among the nine it was given. \
**The site is [`crate::reconnection::AN_AUTHENTICATED_CONNECTION_IS_EVIDENCE_ABOUT_NOTHING_BUT_ITSELF`]**, in the file that is §4.5, with a test holding it: no outcome of the gate can be reached from an authenticated connection, and a machine that answers while holding no record of Smoozy's reads *absent* whether or not a supervisor answered. \
**The substance was checked before the site was written rather than assumed.** `State, identity and recovery`'s gate composes its four outcomes from two halves — the record read off the machine, and the supervisor's own answer — and **the connection is neither of them**. \
**Nothing was invented for it**, which is what A6 asks: the clause states what this document does not do, and C14 and C15 are what does it. \
**A2 and A3 came off on 2026-09-04 with §2's framing**: A2 is a rule about a request field and the framing drops that field as it reads a frame, with a test that composes one by hand; A3 is a rule about what a result may be read as, and it is kept at `crate::framing::Answered` by the absence it asks for — nothing in this crate composes a permission, a licence state or a capability grant from a result. A1, A4 and A5 were already built",
        it_lands_at_step: 6,
    },
    NotBuiltHere {
        section: "§4",
        clauses: &[
            "C3", "C4", "C5", "C8", "C9", "C10", "C13", "C17", "C17a", "C21", "C22",
        ],
        what: "the machine's answer to a lost acknowledgement, and the ten clauses of this section that already have code, kept here so a reader meets the two that do not",
        why: "**C9 and C10 are what is left of this entry, and the three that were owed beside them landed on 2026-09-06.** *(The reading that found them is kept below, struck where it has stopped being true.)* ~~The original reason — *every one of them is answered at the supervisor's own record, which arrives with the supervisor* — **half came true and the half that matters did not.** The supervisor arrived at step 8a; **its record did not.**~~ *(The path this named — `App/Rented-server runtime/src/program.rs` — was corrected out on 2026-09-07: no such file exists. The library's is `the_program.rs` and the program itself is the separate package `App/Rented-server runtime — program/`, split out at step 8b.)* **STRUCK: the record landed the same day.** `State, identity and recovery` §5.1's two records have their form at `smoozy_state::machine_operations` — the eight fields, the disposition-against-outcome split, and the rule that *ended* and the recorded outcome are written in one act and never apart — and their store at `App/Rented-server runtime/src/acting/operation_records.rs`, which a supervisor makes at every start. **C7 is built with it**, at `App/Server protocol and transfers — framing/src/acknowledgement.rs`: an acknowledgement carries the identity the request was framed with and **has no variant, field or function meaning *done***, and C8's three answers are composed from the record and from nothing else. **C12 and C15 are built too**, at `crate::reconnection` — C15's read is a file-transfer-subsystem read of the record in the package area the placement recorded, run against OpenSSH's own file-transfer server in this crate's own integration test, and C12 answers which of C11's four steps a supervisor that does not answer stops: step 3 and not step 4. **C9 and C10 stay, with half of each kept elsewhere and unnamed**: `App/Rendering/src/acting/resolving.rs` resolves a lost acknowledgement by identity and re-issues nothing, which is C9's rule and C10's *it is C8's three answers on the next connection*, and that file names C4 and C8 and neither C9 nor C10 — **what is still owed there is the request that carries the ask**, which is the one thing the record did not bring: §1.2 item 3 makes the request set a package version declares package inspection's, and package inspection is B5's and is on this page. **C20 came off this entry on 2026-09-06 and is built**, at `App/Server Setup/src/driving.rs`, where `Server Setup` Y8a keeps it by name — a stop made during the restart wait leaves no pending-command record because a sent request cannot be un-sent. The ten the transport and the transfer contract already reach are built",
        it_lands_at_step: 6,
    },
    NotBuiltHere {
        section: "§5",
        clauses: &["B1", "B2", "B3", "B4", "B5", "B6", "B7", "B8b", "B10a"],
        what: "the bootstrap-and-repair request itself, and package inspection",
        why: "~~**B5 is OWED and step 6 closed without it, read 2026-09-06.** The bootstrap-and-repair request itself exists nowhere in the workspace: `App/Server protocol and transfers — framing/src/framing.rs` composes the frame for the route and nothing sends one, and `App/Server Setup/src/arriving_later.rs` files *\"the package inspection, the bootstrap-and-repair request and the plain start of the registered service\"* as left to this document's adapter. **Both halves of step 6 record the request as the other's, and step 6 closed.**~~ \
**STRUCK 2026-09-06, later the same day: B1 to B7 are built, at [`crate::bootstrap`].** \
**B1** is one request with one identity that is both the correlation identity and the idempotency key, one verified release descriptor of seven values, and no variant, sub-command or mode flag — kept by there being one request name, one type and one constructor, and that constructor takes a verified release rather than a version string, which is B1's *no request can select bytes by a version string alone* held by the compiler. \
**B2** is the route: two mechanisms and no third, and nothing in that module names the machine-local endpoint at all. \
**B3** answers idempotency from the durable package-phase record's own attempt identity and from nothing else, and **no operation record is reachable from that module**, which is what B3 says outright a builder must not look for. \
**B4** is package inspection, composed from the installation's own durable records over the file-transfer subsystem — the phase record, the boot-registration record, the installation identity and the active-package pointer — **content and never presence**, with an absent record and one that will not read back kept apart, because the second is C14's *present and different* branch and folding it into the first would discard every machine-derived value on a reading that failed. It answers honestly on a machine that has never held a package. \
**B5** is the reading's own rule: [`crate::bootstrap::inspect`] returns a reading carrying the moment and the clock, and the only shape in which the answer is ever kept carries the installation identity it was read under and is subject-held on the server row. Nothing in this crate writes one down. \
**B6** carries a deferred commit as one of the request's ordinary results, with the machine's own reason carried whole and interpreted nowhere. \
**B7** is the absence of anything automatic: the trigger is `Desktop app and delivery`'s and the fallback re-run is `Server Setup`'s. \
**It is proved against OpenSSH's own file-transfer server**, in this crate's own `a_package_is_released_and_verified.rs`, on a bare area and on one a placement wrote records into. \
~~**What the request does not do is the transaction**: B2 divides the two performers and this module is neither, and the staging, the commit, the rollback and the boot registration are `Rented-server runtime`'s.~~ \
**CORRECTED 2026-09-07: B2's second performer is built, at [`crate::placement`].** That clause says *\"where none is, the desktop performs it through the surface\"*, and on a bare rented machine there is never a live runtime, so it is always the desktop's — and until that day **nothing anywhere performed the placement**. What is built is the claim, the phase record at every phase, the staging with B10's at-rest read-back, the boot registration and its record, the single rename that is the commit, the health exchange through §1.1's planted-link carriage, and the rollback. **The transaction's shape is still `Rented-server runtime`'s**: every branch is taken from that crate's own rules half and none is re-decided here. \
**What is still not performed on this side is the machine's own half**, which that document's own arriving-later page carries. \
**AND THREE THINGS ABOUT THE PLACEMENT ARE NOT BUILT, WRITTEN HERE 2026-09-12 out of step 8b's round 1, which found this page silent on all three.** \
**One: nothing in the application drives the placement at all.** [`crate::placement::place_a_verified_release`] is reached only from this crate's own `the_placement_is_performed_through_the_surface.rs`. **Every decision above it is built and no production path calls any of them**, which is the same absence this page's §4.4 entry records for the reconnection's four steps and in the same words. **The risk of not writing it down is the same too**: a reader meets a built transaction and concludes a rented machine can be given a package today, and no part of Smoozy can ask for one. **What removes it is the caller that sends the bootstrap-and-repair request**, which is `Server Setup` F1's and is on that crate's own arriving-later page. \
**Two: nothing calls the release fetch either.** The entry above says the fetcher is built and it reads as though it is in service. It is not: [`crate::release::fetching`] has test callers only, and the board says why in as many words — *\"nothing calls the fetch, because no screen exists to ask for it\"*. **What removes it is the screen**, which is `Preparation/Build/Plan.md` step 10a's. \
**Three: `Rented-server runtime` G25's ending is performed as nothing, ~~and on a container the activation with it~~ (struck 2026-09-21: the container's activation is built, below).** That clause's commit has an ordered step that ends the running supervisor, and this side has no route to a signal; the site says so and points a reader here. ~~**On a container the restarter is that document's own keeper**, and **the keeper exists nowhere in this workspace** — no file of it names one — so there is nothing here to start and inventing a second restarter is what §3.2 and Rule 7 both forbid. **The plan does not say what starts the keeper on a first placement either**: the hook that launches it has just been written and the machine has not restarted, so the commit's start step starts nothing, the health exchange reaches nothing, and the placement rolls back on every attempt. **That is an open question in `Rented-server runtime` rather than a fault of this code**, raised by step 8b's round 1 and owed to that document. **What removes it is two things and neither is this crate's**: that plan's own repair, and then the keeper together with the elevated hook install `Rented-server runtime` K11 and K19 make root's, which `Server Setup` owns.~~ **CORRECTED 2026-09-19, by the Gemini catch-up's Area 2 implementer: both premises of the struck sentences were false or have stopped being true.** The keeper has existed since 2026-09-06, written out inside the container hook at `App/Rented-server runtime/src/registration_content.rs`, `the_container_hook`. **The plan was repaired on 2026-09-12**: `Rented-server runtime` G25 says *\"On a container, the commit invokes the keeper itself as that start step, once\"*, and on 2026-09-19 S5 gained what makes a second start harmless — **the keeper holds an exclusive lock on its own file in the package area for as long as it runs, and a keeper that cannot take it ends at once**. \
~~**So on a container the start step is owed, not blocked by the plan, and [`crate::placement::registration::start_the_supervisor_the_pointer_names`] still starts nothing there.** \
**What stands in the way is one reading.** A shell loop takes that lock with an outside command, and `Rented-server runtime` S5 and Rule 2 require that command's own publication to be quoted at the site, with its address and the date it was read — as the hook's own use of `su` already is. **The sitting that got this far was not allowed to reach the network**, so it read nothing and built nothing rather than write a remembered command into the machine's boot return. The rest is decided: the keeper's loop written once and used by both the hook and this step; the step run once over the command channel as the supervisor's own login, detached the way `Rented-server runtime` S5a observed a process outliving its connection; and nothing in the keeper naming the carriage link, which is taken away after use. **What removes it is that reading, and then the build.** \
**And a container placement fails earlier than this step while that stands**, at `Rented-server runtime` K15's hook read-back, because nothing installs the hook: the elevated install `Rented-server runtime` K11 and K19 make root's is `Server Setup`'s and is on that crate's own arriving-later page.~~ \
**STRUCK 2026-09-21, step 10c: both are built, found by the second pass on a real Vast container**, where every placement failed at the read-back with the record's path empty. **The reading is taken**: util-linux's own `flock(1)` at tag v2.41, quoted at `smoozy_runtime::registration_content::the_keepers_loop` with its address and date, in the descriptor form that page gives for shell scripts. **The keeper's loop is written once**, in that function, and the hook and the start step both run it. **The start step runs it once over the command channel as the supervisor's own login, detached the way `Rented-server runtime` S5a observed** — [`crate::placement::registration::THE_KEEPERS_START`] — **and nothing in the keeper names the carriage link**: it is written as a file in the Smoozy-named directory and names the package area's real paths. **The hook is installed** by [`crate::placement::registration::install_the_hook`] with `Server Setup`'s commands, between `Rented-server runtime` K16b's record and `Rented-server runtime` K15's read-back. **What is still not performed is the ending of a running supervisor**, above, which is this entry's first half and is unchanged. \
**On a systemd machine the same step makes no claim at all now**: what a plain start does to a unit that is already active is established nowhere in this project, the sentence that stated it as fact carried no source, and the question is parked in `Preparation/Groundwork/Open Threads.md`. \
**B8, B9 and B10 were on this entry until 2026-09-06 and are built**, at [`crate::release`]: the signed release manifest and its exact-tuple selection, the offline release root and its key-set statement with the non-rollback memory, and all three of B10's verifications with the at-rest read-back over the subsystem. \
~~**B8a is the one clause of this entry still owed.** It joined on 2026-09-06, the day the plan gained it: it names the Creator's own public repository as the one place a released artifact, its manifest and its key-set statement are served from, and **nothing in this crate names a repository or fetches over HTTPS at all** — the fetch is a trait and the one implementation of it reads a local file. **What it needs is the fetcher that speaks to that repository**, which arrives with the step that first has a package to place.~~ \
**STRUCK 2026-09-07: the fetcher is built, at [`crate::release::fetching`].** It speaks HTTPS to an exact source a verified manifest carried as a value, with a ranged resume, and it **contains no address, no repository name and no rule for making one** — which is what B8a leaves outside every document downstream of it. **What is still owed is not this crate's**: the address itself, which is the Creator's, and the first release, which is their word rather than a builder's. **The local-file fetch stays exactly where it was** and is still what the release test runs against, because nothing is published. \
**The signing key it never needed**: that half closed on 2026-09-06, and the public half of the release root is carried in the build. \
~~**B8b IS OWED, and it joined this entry on 2026-09-19, the day the plan gained it.** It says where a desktop reads the current release file from: the repository's fixed latest address, trusting nothing about the answer beyond B9 and B10, with a redirect followed over HTTPS only, and watched once at the first publish. **Nothing in this crate reads that address** — [`crate::release::fetching`] fetches an exact source it is handed and knows of no latest one. **It is filed here under step 6 only because this is §5's entry; it was never step 6's.** What removes it is step 10c of `Preparation/Build/Plan.md`: piece 9 for the read, and piece 11's first publish for the one watch.~~ \
**CORRECTED 2026-09-27: B8b is built, in the desktop crate, since step 10c's piece 9 (2026-09-20)**, at `App/Desktop app and delivery/src/setting_up.rs`: the address is a build constant there, and `TheCurrentReleaseOverHttps` reads it. **This crate still reads no latest address** — [`crate::release::fetching`] still fetches only an exact source it is handed. The one watch was taken at piece 11's first publish, on 2026-09-21. **The clause stays listed on this entry**, as B10a does, so that a reader of this crate still meets it here. \
**B10a joined this entry on 2026-09-27, the day the plan gained it, and it is not built in this crate by design.** It lets a development copy of the desktop, pointed at a throwaway folder, place a server program built on the user's own computer in place of B8's selected artifact. **The route is the desktop's**, at `App/Desktop app and delivery/src/the_desk_program.rs`, compiled only with debug assertions: this crate is also what the release tool is built from, so nothing of the route belongs here. **What it reuses from this crate is unchanged**: B8c's check, B8's exact selection, the composition of an entry and a manifest, and B10's read-back at rest in [`crate::placement`], which compares the placed bytes against the desk file's own length and digest exactly as it compares a release's. It is filed under step 6 for the reason B8b is",
        it_lands_at_step: 6,
    },
    NotBuiltHere {
        section: "§6.1 and §6.2",
        clauses: &["T2", "T5a"],
        what: "the learned session cap, and which of the two admission points counts sessions and which counts bytes",
        why: "~~both are rules about an admission point this document does not hold. T5's own code says admission is the caller's, and the desktop-side point is `Desktop app and delivery`'s. `Servers` S14 already builds the additional transfer-only connection T2 falls back to at a cap of one.~~ **STRUCK 2026-09-06: step 5 landed and both are named by working code.** **T5a is at `App/Desktop app and delivery/src/scheduling.rs`**, which files it under this document in its own header list and then says at the site that T5a states the same split from its own side and that half is closed — the desktop point takes transfer channels and the update fetch alone, and the per-server point counts every session. **T2 is at `App/File Browser/src/pool.rs`**, which files it under this document and says at the site that T2 makes the session cap the server's. **Nothing is owed at any step and nothing is built here**; the entry is struck rather than deleted so that a reader of this crate still meets T2 and T5a on this page",
        it_lands_at_step: 5,
    },
    NotBuiltHere {
        section: "§6.3",
        clauses: &["T8"],
        what: "the declared priority class",
        why: "a class is declared by the module that arranges the transfer, and step 5 builds the first one that does",
        it_lands_at_step: 5,
    },
    NotBuiltHere {
        section: "§6.3",
        clauses: &["T9", "T10"],
        what: "the order between the priority classes, and the share that keeps no class starved",
        why: "~~T9 says in as many words that the order is not settled here, and T10 says the share is declared against this mechanism rather than here. Both name Rows 189 and 207, which are `File Browser`'s and `Download`'s. **Row 189 is step 5's and Row 207 is step 8's, so the later of the two is the step these land at** — before step 8 the order is missing one of its two owners, and a gate at step 5 that read step 5 here would be reading a record that is broken.~~ **STRUCK 2026-09-05: step 8 landed and both are answered.** Row 207's half is `Download`'s rules code, which answers **by citation and not by code**: `Download` Q2 takes the order as `File Browser` O4 already declares it, declaring no rank into it, and `Download` Q4 takes `File Browser` O5's share rule at the desktop-side point, naming T10's own sentence — *the share is Rows 189 and 207's to declare against this mechanism* — as discharged. The sites are `smoozy_download::classes::the_two_download_classes_as_o4_declares_them` and `smoozy_download::classes::THE_TWO_DOWNLOAD_CLASSES_TAKE_O5S_SHARE_RULE_AS_DECLARED`, and that crate's fetch queue asks `File Browser`'s own admission point over all five declared classes, with a test driving twenty grants inside `File Browser` O5's 2N−1 bound. **Nothing is owed at any step and nothing is built here**; the entry is struck rather than deleted so that a reader of this crate still meets T9 and T10 on this page",
        it_lands_at_step: 8,
    },
    NotBuiltHere {
        section: "§6.4",
        clauses: &["T11", "T11a"],
        what: "the transfer-support worker's five declarations, and why the two admission points are not one",
        why: "~~**OWED, and this is the one of the twenty-six with a wrong answer running rather than an absence. Step 6 closed without it; read 2026-09-06.** The original reason — the worker is a machine-side worker and the arbitration its declarations are made against is `Rented-server runtime` A1's — **describes a dependency that has since arrived and been left unanswered.** `App/Rented-server runtime/src/workers.rs` names this document the owner of the transfer-support kind's declarations, in code, at its `declarations_owned_by`; **this document has declared none, so the kind is arbitrated under `Rented-server runtime` A3's default**, which `App/Rented-server runtime/src/arbitration.rs` builds and whose test fixes its cancellability at *not cancellable*. **T11's own words say that value is false of this kind**, because C22 gives the transfer-support worker a termination contract. **So a machine arbitrating a transfer-support worker today is using a declaration T11 says is wrong**, and nothing anywhere says so. **T11a is owed a site rather than work**: `App/Desktop app and delivery/src/scheduling.rs` files it under this document in its header list and never uses it, while the neighbour on that list, T5a, is used and kept — so the one clause saying the two admission points are different instruments is declared and unwritten~~ **STRUCK 2026-09-06, later the same day: both are built and named, and the wrong answer is gone.** **T11 is at `App/Rented-server runtime/src/arbitration.rs`**, at `the_transfer_support_kinds_declarations`, which makes all five against `Rented-server runtime` A1 — the processors and the machine's memory shared with no card, a rank below the render kind and above the preview and media kinds, **cancellable at any moment**, atomic with one transfer as the unit, and nothing declared at the exclusion. **The kind no longer reaches `Rented-server runtime` A3's default at all**: that same file's `the_declaration_in_force_for` is the one place a kind's declaration is composed, it takes A2's two first and this document's T11 second, and a test asserts that what it answers for the transfer-support kind is not the default and that the default still says *not cancellable* — so the assertion that would have failed before this landed is the one that guards it now. The number in the rank is this build's and the ordering is T11's, held by a test against the declarations the crate carries rather than against a literal. **T11a is at `App/Desktop app and delivery/src/scheduling.rs`**, beside `THE_SEAM_R18_RECORDS`, and at `App/Rented-server runtime/src/arbitration.rs` in the same words from the machine's end — the desktop's points admit sessions and bytes, `Rented-server runtime` A4's admits worker starts against the contended resources, a transfer after hand-over passes both, and neither figure is derived from the other. **The two halves are held together** by the desktop's test reading the machine file's own sentence, which is the habit `crate::release::composing` already uses for the entry point's two copies. **Nothing is owed at any step and nothing is built in this crate**; the entry is struck rather than deleted so that a reader of this crate still meets T11 and T11a on this page",
        it_lands_at_step: 6,
    },
    NotBuiltHere {
        section: "§8",
        clauses: &["R6b", "R9", "R10", "R11", "R12", "R12a", "R13", "R14"],
        what: "the shared rate budget, how a cap reaches transfers already running, and §8.4's achieved-rate reading with the window it is averaged over",
        why: "**TWO OF THESE WERE FILED UNDER A CONDITION THAT WAS WRONG, and neither belongs to a step. Read 2026-09-06, after step 5 closed.** **R9 is discharged and only a citation is owed.** It is the clause listing the four things the mid-flight answer follows from — R1, R3, R6 and R6b — and **all four are built and named at `App/Desktop app and delivery/src/scheduling.rs`**, R6b among them. Nothing further is to be made for R9; what removes it from this page is a line at that file naming it, and no step of `Preparation/Build/Plan.md` was ever going to supply one. **R14 is a record of a measurement and not a promise at all**, in the shape of §8.5's three: it says caps were observed holding at 409 KB/s and at 402.8 and 405.5 KB/s, so the achieved rate was read. **No step builds it**, and the reading it is evidence for is composed at that same scheduler, which `Desktop app and delivery` R15a takes whole. It is not moved to [`NOT_THIS_CODES_AT_ALL`] because that list is bounded by [`THE_SECTIONS_WHOSE_CLAUSES_MAY_BE_RECORDS`] and §8.4 holds promises beside it; **widening that gate to fit one clause is the fault the gate exists to stop**, so the truth is written here instead. The other six of this entry stand as filed. R1 puts both budgets in `Desktop app and delivery`'s single desktop-side scheduler and says outright that this document holds no second budget, divides none and caches none. The enforcement point — R1 to R8 with R6a — is built, and draws through a seam. **R12a joined this list on 2026-09-04 and the line above lost it**: it read *R1 to R8 with R6a and R12a is built*, which was never true. R12a fixes the trailing window the achieved rate is averaged over, and no reading is composed in this crate at all — R12 is on this list, so the window it belongs to was on the wrong side of it. `Desktop app and delivery` R15a takes R12a whole, which is where it landed. **The step was not written down until the step 4 gate asked for it**: it is the step that builds the first module to arrange transfers, and Row 188's pool, which is `File Browser`'s. **Row 206's pool is `Download`'s and arrives at step 8**, and the budget does not wait on it: a scheduler with one module drawing on it is the whole mechanism, and the second module joins it. §8.5's R15, R16 and R17 were listed here until the gate's second run and are not this code's at all; they are in [`NOT_THIS_CODES_AT_ALL`]",
        it_lands_at_step: 5,
    },
    NotBuiltHere {
        section: "§9.2 and §9.3",
        clauses: &["F6", "F8", "F9", "F11a", "F12"],
        what: "the disk-full carve-out's two routes, the moment each is fixed at, and what each supplies to the condition",
        why: "**THE STEP WAS THE WRONG CONDITION, and step 6 has closed. Read 2026-09-06.** Step 6 built the only thing this entry said was missing — route (b)'s machine-side reading, at `App/Rented-server runtime/src/health_readings.rs`, where V12 carries a free-space reading against the job's own output folder taken in the same act that writes the outcome, with the moment on the machine's clock. **F6a came off this entry the same day and is named there — that file cites it by document at the site, and F6a is what route (b)'s reading is, what carries it and what happens where it is absent. The other five did not come off, because what they bind is not a machine-side reading at all: they bind whoever composes and shows a failed job's message from it, and nothing in the workspace does.** A search of the whole of `App/` for the carve-out finds the reading and no consumer. **What actually removes them is `Desktop app and delivery` §16.4's message surface — M15's — which has no code and which no step of `Preparation/Build/Plan.md` names**: step 2 built that document's adapters, its boundary and its local writes, and step 11 is delivery. **So these five wait on a surface nobody has been given**, and F14, F15 and F17 wait on the same one from the entry below. **Two of the five are meanwhile kept in substance and named nowhere**: the doc comment on V12 says route (b) does not fire on a missing reading, that nothing is re-read afterwards, and that nothing infers a full disk from it, which is F11a and F12's own ground, and that file's header list declares F11a under this document without ever using it. Neither route can be taken from where this code stands. Route (a) wants a refusal that names the out-of-space condition, and revision 3's status set has no member for one — a full disk arrives over this surface as a bare failure, and §1.3 forbids reading the message for a cause, which is what `crate::subsystem::wire`'s own constant says. Route (b)'s reading is taken on the machine, in the act that writes a render's recorded outcome, which is `Rented-server runtime` V12's. The reading itself is built",
        it_lands_at_step: 6,
    },
    NotBuiltHere {
        section: "§9.3 and §9.4",
        clauses: &["F14", "F15", "F17"],
        what: "what a presented reading may not claim",
        why: "**THE STEP WAS THE WRONG CONDITION, and step 7 has closed. Read 2026-09-06.** The entry said each binds a presentation, which is `Desktop app and delivery` §16.4's and Design's, and that nothing presents a reading until a job can fail. **Step 7 made a job able to fail and no presentation followed**, because the condition was never *a job can fail* — it is **`Desktop app and delivery` §16.4's message surface, M15's, which has no code and which no step of `Preparation/Build/Plan.md` names.** These three wait on the same surface as F6, F8, F9, F11a and F12 in the entry above, and nothing in the workspace composes a failed job's message from a free-space reading. **F10 came off this entry on 2026-09-06 and is built**, at `App/File Browser/src/free_space.rs`, which files it under this document and keeps its own sentence at the site — *it is never re-read to keep a message fresh* — and keeps it a second time where there is nowhere else to ask. **That is the fault this entry carried**: F10 has two homes, the browser's figure and the carve-out's, and filing it beside three presentation clauses hid the one that had landed. **What whoever builds the surface must carry**: F10 wants the moment the reading was taken, on the machine's own clock, and `Rented-server runtime` V12 now carries that moment for route (b). F16 is built",
        it_lands_at_step: 7,
    },
    NotBuiltHere {
        section: "§10.1 and §10.3",
        clauses: &["G1", "G2", "G3", "G9"],
        what: "the floor of one report before a result, and what a waiter's expired deadline does not change about the work",
        why: "~~**OWED. Step 6 closed without them; read 2026-09-06.** The entry said all four rest on §4's C3, C7 and C17 — recorded locally before it is sent, acknowledged as recorded before the work completes, and a waiter's expiry reported — and that none of the three is built. **Two of the three now are**: C3 is named at `App/Server Setup/src/driving.rs` and C17 at five files, among them `App/Queue & History/src/acts.rs` and `App/Rendering/src/acting/stopping.rs`. **C7 is not, and it is the one these four cannot do without** — G2 says in as many words that C7 is what makes G1 more than a restatement. **C7 waits on the machine-side durable operation record, which `Preparation/Build/Plan.md`'s step 4 assigned to step 6 and step 6 did not build**: `App/Rented-server runtime/src/acting/stores.rs` says the operation record on the machine is another document's and invents no form for it, and the supervisor's own program says the same on its complaints stream at every start. **So these four are owed rather than waiting**, and what is owed is one record.~~ **STRUCK 2026-09-06, later the same day: the record landed and all four are built.** **They are at `App/Server protocol and transfers — framing/src/reporting.rs`**, in this crate's other half, because both ends speak them — the machine owes the acknowledgement and the desktop owes the local record and the reading of what comes back, and neither opens a connection to do it. **G1's floor is a value a request carries and a function that answers it**: a request with none of the three is `WhetherTheFloorIsMet::ItIsAFaultInSectionTen`, which is the clause's own *a fault in this section rather than a case it does not cover* written as an answer with a name rather than a `false`. **G2 is kept by C7's own type**, next door at that crate's `acknowledgement.rs`: an acknowledgement carries the identity and has no variant, field or function meaning *done*, so *it has started* and *it has finished* really are two correlated answers, and a request whose only answer is the finished one keeps none of the three and fails the floor. **G3 is a waiter's expiry as a reportable event**, the deadline itself staying the waiter's own and per-waiter under C16. **G9 is kept by the shape rather than by a rule**: what is reported about the work is composed from what the record and the journal say, and that function has no parameter for a waiter's expiry, so there is no route by which one could reach the answer. **What C7 waited on is now built too** — `State, identity and recovery` §5.1's two records have their form at `smoozy_state::machine_operations` and their store at `App/Rented-server runtime/src/acting/operation_records.rs`, which a supervisor makes at every start. **Two things are still absent and are named rather than glossed**: the event journal, which is that document's §4 and which G4's second route needs, and the request that carries C9's ask, which is B5's package inspection. The transfer's own reporting, G4 to G4b and G5 to G8, was already built",
        it_lands_at_step: 6,
    },
];

/// One clause of this document that is not this code's at all.
///
/// **This is a different thing from an absence.** A clause in
/// [`EVERYTHING_ARRIVING_LATER`] is a promise nobody has built yet and a step
/// is named for it. A clause here is a record — of what was measured, of what
/// was not observed, of a question that has been answered — and **no step
/// builds one, at step 4 or anywhere**. Recording one as arriving at a step
/// would send whoever builds that step looking for work that does not exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotThisCodesAtAll {
    /// The plan's own section.
    pub section: &'static str,
    /// The clause.
    pub clause: &'static str,
    /// What it is.
    pub what: &'static str,
    /// What rests on it, and who owns that.
    pub what_rests_on_it: &'static str,
    /// **Which of the three grounds lets it sit here.**
    pub qualifies_as: WhatQualifiesIt,
    /// **Nothing anywhere makes this clause happen** — not here, not at a
    /// later step, not in another module. That is the whole difference
    /// between this list and the one beside it, and it is stated per entry
    /// rather than left to the list's name.
    pub nothing_is_made_for_it: bool,
}

/// **What has to be true of a clause before it may sit in
/// [`NOT_THIS_CODES_AT_ALL`], and the set is closed.**
///
/// A clause that is none of these three is a promise, and a promise belongs in
/// [`EVERYTHING_ARRIVING_LATER`] with the step it lands at. **This exists so a
/// fourth entry cannot be put in that list on a rule nobody stated**: the step
/// 4 gate's third run judged the list honest on all three of its entries and
/// found that nothing held what entry to it requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatQualifiesIt {
    /// **A record of evidence.** The clause states what was measured, or what
    /// was not observed, and nothing is composed from it here.
    ARecordOfEvidence,
    /// **A question already answered.** The clause says so in its own opening,
    /// and whatever it produced is a separate clause that *is* a promise.
    AQuestionAlreadyAnswered,
    /// **An observation supplied to another document**, which composes whatever
    /// the user is shown from it. The owning documents are named in the entry.
    AnObservationSuppliedElsewhere,
}

/// **The sections whose clauses may be records at all.**
///
/// §8.5 is called *What is not observed*, and every clause under it is a
/// statement about the evidence. **Adding a section here is its own act**: it
/// means somebody read that section and found it holds no promise, and it is
/// not a thing to widen because an entry would otherwise not fit.
pub const THE_SECTIONS_WHOSE_CLAUSES_MAY_BE_RECORDS: [&str; 1] = ["§8.5"];

/// **§8.5's three, which no step builds.**
///
/// The section is called *What is not observed*, and each of the three is a
/// statement about the evidence rather than a thing to make. They were recorded
/// as arriving at step 5 until the step 4 gate's second run, which is the same
/// fault as a hole hidden from an audit, pointing the other way.
///
/// **`Servers` W39 and `Servers` W44 are treated this way by that document
/// and by the gate**, so this is the shape already in use rather than a new
/// one.
pub const NOT_THIS_CODES_AT_ALL: [NotThisCodesAtAll; 3] = [
    NotThisCodesAtAll {
        section: "§8.5",
        clause: "R15",
        what: "the record that the shared budget was exercised on uploads only, and that a downward reading counts bytes Smoozy accepted rather than bytes the line carried",
        what_rests_on_it: "§12 item 6 states what breaks and the download half is permanently unresolved. The caution binds §8.4's reading, which arrives with the rest of §8; nothing is built for R15 itself, and while no reading is composed there is nothing for its bound to bound",
        qualifies_as: WhatQualifiesIt::ARecordOfEvidence,
        nothing_is_made_for_it: true,
    },
    NotThisCodesAtAll {
        section: "§8.5",
        clause: "R16",
        what: "a question answered and closed, opening *ANSWERED 2026-08-22* — the mid-flight cap change was run four times, and what it exposed is written as a requirement elsewhere",
        what_rests_on_it: "the requirement it produced is R6b, which is a promise and is listed as arriving at step 5. R16 also records that R12a's three-second window has never been watched at any length, and §12 item 14 carries that. Neither is work",
        qualifies_as: WhatQualifiesIt::AQuestionAlreadyAnswered,
        nothing_is_made_for_it: true,
    },
    NotThisCodesAtAll {
        section: "§8.5",
        clause: "R17",
        what: "a measurement of what the link actually carries — the asymmetry about six to one on one rented box, and the seller's advertised figure wrong by an order of magnitude",
        what_rests_on_it: "the clause says in as many words that this document supplies the observation and composes no estimate. A duration shown to a user is `Queue & History`'s and `Download`'s, so what rests on R17 is owed by those two documents and by nothing here",
        qualifies_as: WhatQualifiesIt::AnObservationSuppliedElsewhere,
        nothing_is_made_for_it: true,
    },
];

/// One clause of this document that no working file of this crate names, whose
/// removal is scheduled work rather than a step.
///
/// **This is [`NotBuiltHere`] with the step field dropped and a *what removes
/// it* field in its place**, and the swap is the whole reason it is a separate
/// shape: a step number is what a reader of `Preparation/Build/Plan.md` goes
/// and looks up, and there is nothing there to find.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaitingOnScheduledWork {
    /// The plan's own section.
    pub section: &'static str,
    /// Every clause of that section this covers.
    pub clauses: &'static [&'static str],
    /// What the clause is.
    pub what: &'static str,
    /// Why no working file of this crate names it.
    pub why: &'static str,
    /// **What removes it** — named exactly enough that a reader can go and find
    /// it.
    pub what_removes_it: &'static str,
}

/// **The one clause waiting on the path seam.**
///
/// **Added 2026-09-06, the day the plan gained P7.** The clause was added, no
/// code was written for it, and nothing in this crate said so for the rest of
/// that day. **`File Browser` caught the same ruling's clause in its own plan
/// within the hour**, because that crate's arriving-later page carries a
/// forward-leg test that reads its plan and demands a naming site for every
/// clause; this page carries no such test, so the absence here was found by a
/// search rather than by the crate. Why it still carries none is
/// [`WHY_THIS_PAGE_CARRIES_NO_FORWARD_LEG_TEST`].
pub const WAITING_ON_THE_PATH_SEAM: [WaitingOnScheduledWork; 1] = [WaitingOnScheduledWork {
    section: "§2",
    clauses: &["P7"],
    what: "the whole path seam settled once — a path is bytes on the machine side, the receiving system's own path unit on the desktop side, and text nowhere but where a name is shown. **It binds three things this crate holds**: the wire, the transfer record's own path fields, and the hand-over at either end. `Desktop app and delivery` C3's name check is named as the single place a machine-side path's bytes become a local one, and in the other direction `File Browser` N2 already puts the source name's own bytes on the wire, so there is no conversion at all. The clause's fifth part is that text appears once and it is display, which is `File Browser` L1a and `File Browser` X1 stated for this surface. **P7 also says outright that the staging path is not exempt**: S4 synthesises the last component and S1 puts the whole thing beside the final name, so a staging path's leading components are exactly as unsafe as a final path's",
    why: "**this crate holds both spellings side by side today, and naming P7 anywhere in it would record a promise as kept where the code contradicts it.** The wire is already right and is the one half that is — `crate::subsystem::client` takes `&[u8]` for every path — and everything above it is text. The record's three path fields are `String`, `String` and a `StagingPath` whose directory is a `String`; the local-computer surface `crate::transfers::moving::TheUsersOwnComputer` takes `&str` in all nine of its methods and ten of its path parameters; and the byte loop converts between the two on the same record field at twenty production sites in that one file. **The pair that shows it is five lines apart**: a stop removes the staging path by its bytes on the machine side and by its text on the local side, from the same value, in one function. **A clause marker written at any of those would be false**",
    what_removes_it: "**the carrying out of the ruling, whose work list is `Audits/Build/Seam decisions/Paths across the transfer seam — the work list.md`.** This crate is step 1 of that list's own order and everything else waits on it: the record's three path fields and `StagingPath` take the shape of the end they name, with the direction deciding it; the nine trait methods take a local path rather than text; and the twenty conversions go, because the wire takes the machine-path field as it stands and the disk takes the local-path field as it stands. That list calls this the largest single simplification in the ruling. The text-to-bytes hops at `crate::free_space` and `crate::carriage` disappear with them, and the test implementors move with the trait — this crate's in-memory disk, its own integration test, and `Servers`' plain disk. **When it lands, P7 is named at the record and at the trait**, and the work list's three markers to write in the code are P7 here, `File Browser` L1b at that crate's crossing and, where a name is shown, `File Browser` L1a",
}];

/// **Why this page carries no forward-leg test, ruled 2026-09-06 under the
/// Creator's standing delegation of 2026-08-31.**
///
/// **The question.** `File Browser`'s own `arriving_later.rs` carries a test
/// that reads its plan, finds the naming site of every clause the plan defines,
/// and fails where a clause is neither named by a working file nor recorded as
/// waiting — with a companion guard that no working file may name a clause the
/// page records as waiting. That test is why the path ruling's clause in that
/// plan was met the same day it was written. **This page's absence was found by
/// a search a day later instead, and the question is whether that test belongs
/// here too.**
///
/// **The answer is no, and the reason is not cost.** The companion guard's
/// premise is that *recorded as waiting* and *named by working code* are
/// exclusive. **That premise holds next door and is false here.** This page's
/// absences are mostly partial: a clause whose decision is built and whose two
/// readings are not; a record field that is written and read back and compared
/// with nothing; a window that is stated where no reading is composed. Working
/// files of this crate name four such clauses correctly and at the right sites
/// — C11, C14, S13 and R12a — while this page correctly records those four and
/// R11 as arriving later. *(Corrected 2026-09-07: this said five, counting R11
/// among the clauses working files of this crate name. No working file of this
/// crate names R11 at all; the one that does is `App/Desktop app and
/// delivery/src/scheduling.rs`. The argument is unaffected — four partly-built
/// and cited clauses break the premise as thoroughly as five.)* **Under the
/// guard as it is written next door, every one of those four is a failure**,
/// and the only ways to make it pass are to
/// delete a correct clause citation from working code or a correct entry from
/// this page. Both are weakenings, and a test bought by weakening what it
/// guards is worth less than no test.
///
/// **What would actually make it right is a different piece of work**: a way to
/// record a clause as *partly* built, which neither this page nor `File
/// Browser`'s has, and which is a change to the shape all the arriving-later
/// pages share rather than something to improvise in one crate. Until somebody
/// does that, the forward leg over this document is the build gate's, run by a
/// fresh agent over the module, which is what
/// `Preparation/Build/Method.md` already gives it.
///
/// **What is not deferred is the specific failure that produced this entry.**
/// A clause added to the plan with nothing here saying so is caught by this
/// page's own test `the_plans_clause_count_has_not_moved_under_this_page`,
/// which counts and does not read. It cannot tell which clause moved and it cannot
/// see a clause that was always unbuilt; what it does is fail on the day the
/// plan gains or loses one, and send whoever changed it to this page.
pub const WHY_THIS_PAGE_CARRIES_NO_FORWARD_LEG_TEST: &str =
    "the companion guard treats a recorded absence and a working file's clause \
     citation as exclusive, and five of this page's entries are partly built \
     and correctly cited in working code; making the guard pass would mean \
     deleting a true citation or a true entry, and recording a clause as partly \
     built is a change to the shape every arriving-later page shares";

/// **Nothing here is stubbed, and this is the difference stated plainly.**
///
/// A stub answers. These do not exist at all: there is no function to call, no
/// value that stands in for one, and no caller that would silently take a
/// default. Where a built thing would otherwise hold a figure it cannot take,
/// it holds a statement of absence a reader can tell from a value.
pub const NOTHING_HERE_IS_A_STUB: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    /// **The plan itself, read here rather than summarised.**
    ///
    /// Everything below asks the plan a question instead of asking whether
    /// somebody filled a field in. A clause that is not there, or is there in
    /// another section, or is a record on a different ground from the one the
    /// entry claims, fails.
    const THE_PLAN: &str =
        include_str!("../../../Preparation/Architecture/Server protocol and transfers.md");

    /// The number of the plan's section a clause is defined in, or nothing
    /// where the plan defines no such clause.
    fn the_section_a_clause_sits_in(clause: &str) -> Option<String> {
        let opens_with = format!("**{clause}.");
        let mut heading = "";
        for line in THE_PLAN.lines() {
            if let Some(rest) = line.strip_prefix("### ") {
                heading = rest;
            } else if let Some(rest) = line.strip_prefix("## ") {
                heading = rest;
            }
            let body = line
                .trim_start()
                .trim_start_matches("- ")
                .trim_start_matches("* ");
            if body.starts_with(&opens_with) {
                let number = heading
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .trim_end_matches('.');
                return Some(number.to_string());
            }
        }
        None
    }

    /// The clause's own words in the plan.
    fn the_clauses_own_words(clause: &str) -> Option<&'static str> {
        let opens_with = format!("**{clause}.");
        THE_PLAN
            .lines()
            .find(|line| line.trim_start().starts_with(&opens_with))
    }

    /// Whether a section the plan defines a clause in is one an entry filed it
    /// under. `§8` covers `8.2`; `§1` does not cover `10.1`.
    fn the_entry_files_it_under(entry_section: &str, plans_section: &str) -> bool {
        entry_section.split(" and ").any(|part| {
            let part = part.trim().trim_start_matches('§');
            plans_section == part || plans_section.starts_with(&format!("{part}."))
        })
    }

    /// **The ground worked out from the clause's own words**, in one fixed
    /// order, so that it is a reading of the plan rather than a label anybody
    /// chose. Nothing means the clause is a promise and belongs in the other
    /// list.
    fn the_ground_the_plan_gives(clause: &str) -> Option<WhatQualifiesIt> {
        let words = the_clauses_own_words(clause)?;
        if words.contains("ANSWERED") {
            return Some(WhatQualifiesIt::AQuestionAlreadyAnswered);
        }
        let names_another_document = words.split('`').skip(1).step_by(2).any(|quoted| {
            let quoted = quoted.trim();
            quoted.len() > 3
                && quoted != "Server protocol and transfers"
                && quoted.starts_with(char::is_uppercase)
        });
        if names_another_document {
            return Some(WhatQualifiesIt::AnObservationSuppliedElsewhere);
        }
        let states_evidence = ["observed", "measured", "established", "exercised"]
            .iter()
            .any(|word| words.contains(word));
        if states_evidence {
            return Some(WhatQualifiesIt::ARecordOfEvidence);
        }
        None
    }

    #[test]
    fn every_clause_named_here_is_a_clause_of_the_section_it_is_filed_under() {
        // The markers on this page are written as text rather than in
        // comments, so no sweep of the comments reaches them. This is what
        // reaches them: the plan is asked, clause by clause.
        let mut checked = 0;
        for part in EVERYTHING_ARRIVING_LATER {
            for clause in part.clauses {
                let sits_in = the_section_a_clause_sits_in(clause)
                    .unwrap_or_else(|| panic!("the plan defines no {clause}"));
                assert!(
                    the_entry_files_it_under(part.section, &sits_in),
                    "{clause} is filed under {} and the plan defines it in {sits_in}",
                    part.section
                );
                checked += 1;
            }
        }
        for record in NOT_THIS_CODES_AT_ALL {
            let sits_in = the_section_a_clause_sits_in(record.clause)
                .unwrap_or_else(|| panic!("the plan defines no {}", record.clause));
            assert!(
                the_entry_files_it_under(record.section, &sits_in),
                "{} is filed under {} and the plan defines it in {sits_in}",
                record.clause,
                record.section
            );
            checked += 1;
        }
        for part in WAITING_ON_THE_PATH_SEAM {
            for clause in part.clauses {
                let sits_in = the_section_a_clause_sits_in(clause)
                    .unwrap_or_else(|| panic!("the plan defines no {clause}"));
                assert!(
                    the_entry_files_it_under(part.section, &sits_in),
                    "{clause} is filed under {} and the plan defines it in {sits_in}",
                    part.section
                );
                checked += 1;
            }
        }
        // 60 from 2026-09-06, when B8a joined the §5 entry.
        //
        // **57 later the same day**, when the clause-by-clause read of the
        // twenty-six absences whose step had closed took **C20**, **F10** and
        // **F6a** off this page — each named by a working file of another
        // crate, each recorded here as unbuilt for as long as a closed step
        // had stood over it.
        //
        // **54 later still, the same day**, when the nine owed clauses this
        // crate was given were built and **C7**, **C12** and **C15** came off
        // the §4 entry on C20's own precedent — a clause named by working code
        // is not a promise nobody has built. The other six went with entries
        // that are struck in place rather than removed, so their clauses are
        // still counted here: **T11** and **T11a** at §6.4, and **G1**, **G2**,
        // **G3** and **G9** at §10.
        //
        // **53 on 2026-09-07**, when **B8a** came off the §5 entry on the same
        // precedent — the fetcher that speaks to the repository is built, at
        // `crate::release::fetching`.
        //
        // **54 on 2026-09-19**, when **B8b** joined the §5 entry, the day the
        // plan gained it.
        //
        // **55 on 2026-09-27**, when **B10a** joined the §5 entry, the day the
        // plan gained it. It is built in the desktop crate and not here.
        assert_eq!(checked, 55);
    }

    #[test]
    fn every_unbuilt_part_names_its_clauses_its_reason_and_its_step() {
        for part in EVERYTHING_ARRIVING_LATER {
            assert!(!part.clauses.is_empty());
            assert!(!part.what.is_empty());
            assert!(!part.why.is_empty());
            assert!((5..=8).contains(&part.it_lands_at_step));
        }
        assert!(NOTHING_HERE_IS_A_STUB);
    }

    #[test]
    fn the_clause_waiting_on_the_seam_says_what_removes_it_and_where_that_is_written() {
        // An absence with no way out of it is a gap wearing a record's
        // clothes, and a way out nobody can find is the same thing. The work
        // list is named by its path and the path is checked to exist, because
        // a moved or renamed file would leave this entry pointing at nothing.
        // The audit trails are not part of the public copy, so the file is
        // checked only where the `Audits/` folder is there; the name is
        // checked everywhere.
        let audits = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Audits");
        for part in WAITING_ON_THE_PATH_SEAM {
            assert!(!part.clauses.is_empty());
            assert!(!part.what.is_empty());
            assert!(!part.why.is_empty());
            assert!(!part.what_removes_it.is_empty());
            let list =
                "Audits/Build/Seam decisions/Paths across the transfer seam — the work list.md";
            assert!(
                part.what_removes_it.contains(list),
                "{:?} does not name the work list that removes it",
                part.clauses
            );
            if audits.is_dir() {
                let at = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../..")
                    .join(list);
                assert!(at.exists(), "the work list is not at {}", at.display());
            }
        }
        assert!(!WHY_THIS_PAGE_CARRIES_NO_FORWARD_LEG_TEST.is_empty());
    }

    #[test]
    fn no_clause_is_listed_twice() {
        // A clause listed twice would let one of the two entries be wrong
        // without anything saying so.
        let mut seen: Vec<&str> = Vec::new();
        for part in EVERYTHING_ARRIVING_LATER {
            for clause in part.clauses {
                assert!(!seen.contains(clause), "{clause} is listed twice");
                seen.push(clause);
            }
        }
        for part in WAITING_ON_THE_PATH_SEAM {
            for clause in part.clauses {
                assert!(!seen.contains(clause), "{clause} is listed twice");
                seen.push(clause);
            }
        }
        // 57 from 2026-09-06, when B8a joined the §5 entry. **54 later the
        // same day**, when C20, F10 and F6a came off as built. **51 later
        // still**, when C7, C12 and C15 came off the same way. **50 on
        // 2026-09-07**, when B8a itself came off, the fetcher being built.
        // **51 on 2026-09-19**, when B8b joined the §5 entry. **52 on
        // 2026-09-27**, when B10a joined it.
        assert_eq!(seen.len(), 52);
    }

    #[test]
    fn a_record_is_never_also_recorded_as_arriving_at_a_step() {
        // This is the fault the gate's second run found: three records of §8.5
        // filed as arriving at step 5, where nothing builds them.
        for record in NOT_THIS_CODES_AT_ALL {
            assert!(!record.what.is_empty());
            assert!(!record.what_rests_on_it.is_empty());
            for part in EVERYTHING_ARRIVING_LATER {
                assert!(
                    !part.clauses.contains(&record.clause),
                    "{} is recorded both as arriving at a step and as no step's",
                    record.clause
                );
            }
            for part in WAITING_ON_THE_PATH_SEAM {
                assert!(
                    !part.clauses.contains(&record.clause),
                    "{} is recorded both as waiting on scheduled work and as no step's",
                    record.clause
                );
            }
        }
    }

    #[test]
    fn what_qualifies_a_clause_for_the_no_step_list_is_held_rather_than_assumed() {
        // The step 4 gate's third run: the list is honest on all three of its
        // entries and nothing tested what entry to it requires. A fourth entry
        // put in on a rule nobody stated would be the fault the list exists to
        // prevent, arriving from the other side.
        //
        // Four things have to hold, and the ground itself is a closed set —
        // `WhatQualifiesIt` — so a fourth rule cannot be written down at all.
        //
        // The gate's fourth run: the first three below were here and the
        // ground was not. Two of them only checked that a field had been
        // filled in, so an entry filed under the wrong ground passed.
        for record in NOT_THIS_CODES_AT_ALL {
            // 0. The ground the entry claims is the ground the clause's own
            //    words in the plan give. This catches a wrong filing and it
            //    does not bound the list: the gate's fifth run ran the same
            //    reading over all 152 clauses of the plan and got one of the
            //    three grounds for 75 of them. The bar is the section gate at
            //    1 below, and the test after this one says what it admits.
            assert_eq!(
                the_ground_the_plan_gives(record.clause),
                Some(record.qualifies_as),
                "{} claims a ground its own clause does not give",
                record.clause
            );
            // 1. It comes from a section that holds no promise. A record from
            //    anywhere else needs that section read first.
            assert!(
                THE_SECTIONS_WHOSE_CLAUSES_MAY_BE_RECORDS.contains(&record.section),
                "{} is in {}, which nobody has ruled holds only records",
                record.clause,
                record.section
            );
            // 2. Nothing is made for it, anywhere, at any step.
            assert!(
                record.nothing_is_made_for_it,
                "{} has something made for it, so it is a promise",
                record.clause
            );
            // 3. Whatever rests on it is owed by somebody named — another
            //    document, or an unresolved entry that says what breaks.
            assert!(
                record.what_rests_on_it.contains('`')
                    || record.what_rests_on_it.contains("§12 item"),
                "{} does not name who owns what rests on it",
                record.clause
            );
        }
    }

    /// Every clause the plan defines under one section number, in the plan's
    /// own order.
    fn the_clauses_of_section(section: &str) -> Vec<String> {
        let want = section.trim_start_matches('§');
        let mut heading = "";
        let mut out = Vec::new();
        for line in THE_PLAN.lines() {
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
            if number != want {
                continue;
            }
            let body = line
                .trim_start()
                .trim_start_matches("- ")
                .trim_start_matches("* ");
            let Some(rest) = body.strip_prefix("**") else {
                continue;
            };
            let Some(marker) = rest.split('.').next() else {
                continue;
            };
            let looks_like_a_marker = marker.starts_with(char::is_uppercase)
                && marker.chars().any(|c| c.is_ascii_digit())
                && marker.chars().all(|c| c.is_ascii_alphanumeric() || c == '-');
            if looks_like_a_marker {
                out.push(marker.to_string());
            }
        }
        out
    }

    #[test]
    fn the_bar_on_the_no_step_list_is_the_section_gate() {
        // The gate's fifth run measured what the ground reading is worth as a
        // membership bar: run over all 152 clauses of the plan it gives one of
        // the three grounds for 75 of them, so it is not the bar. The bar is
        // the section gate, and this holds what that gate lets in — the plan
        // itself is read, so a clause added to or dropped from an admitted
        // section fails here rather than passing quietly.
        for section in THE_SECTIONS_WHOSE_CLAUSES_MAY_BE_RECORDS {
            let admitted = the_clauses_of_section(section);
            assert!(
                !admitted.is_empty(),
                "the plan defines no clause in {section}"
            );
            let listed: Vec<String> = NOT_THIS_CODES_AT_ALL
                .iter()
                .filter(|record| record.section == section)
                .map(|record| record.clause.to_string())
                .collect();
            assert_eq!(
                admitted, listed,
                "{section} holds {admitted:?} and this page lists {listed:?}"
            );
        }
    }

    /// Every clause the plan defines, in the plan's own order, **by the same
    /// line rule [`the_clauses_of_section`] uses**, so this file reads the plan
    /// one way rather than two.
    fn the_clauses_the_plan_defines() -> Vec<String> {
        let mut out = Vec::new();
        for line in THE_PLAN.lines() {
            let body = line
                .trim_start()
                .trim_start_matches("- ")
                .trim_start_matches("* ");
            let Some(rest) = body.strip_prefix("**") else {
                continue;
            };
            let Some(marker) = rest.split('.').next() else {
                continue;
            };
            let looks_like_a_marker = marker.starts_with(char::is_uppercase)
                && marker.chars().any(|c| c.is_ascii_digit())
                && marker.chars().all(|c| c.is_ascii_alphanumeric() || c == '-');
            if looks_like_a_marker {
                out.push(marker.to_string());
            }
        }
        out
    }

    #[test]
    fn the_plans_clause_count_has_not_moved_under_this_page() {
        // **This is not the forward leg and must not be read as one.**
        // [`WHY_THIS_PAGE_CARRIES_NO_FORWARD_LEG_TEST`] says why there is
        // none. This counts: it fails on the day the plan gains or loses a
        // clause, and sends whoever changed it to the lists above. It cannot
        // say which clause moved and it cannot see a clause that has been
        // unbuilt and unrecorded all along.
        //
        // **The number is what the plan held on 2026-09-06**, the day it
        // gained P7 and the day this test was written. A ruling that adds a
        // clause to this plan and nothing to this crate now stops at the desk
        // rather than at a gate months later, which is the exact failure that
        // produced [`WAITING_ON_THE_PATH_SEAM`].
        //
        // **155 is this reader's count and not the gate's**, which read 152
        // over the same plan before P7 was written. The two readings differ on
        // what they admit and neither is wrong; **what this test needs is one
        // reading that moves when the plan moves**, and comparing the two
        // numbers is a mistake.
        //
        // **156 from 2026-09-06**, when the plan gained B8a — who serves a
        // released server package. **This test did exactly what it was written
        // to do**: the clause landed in the plan, nothing landed on this page,
        // and the desk went red the same day rather than a gate going red
        // months later. B8a is now on the §5 entry above.
        //
        // **157 from 2026-09-19**, when the plan gained B8b — where a desktop
        // reads the current release file from. It went onto the §5 entry above
        // in the sitting that wrote it.
        //
        // **158 from 2026-09-26**, when the plan gained B8c — the machine
        // program's artifact needs no C library from the machine. **It is
        // built, not owed**, so it goes on no entry above: the reading is
        // [`crate::release::composing::the_artifact_needs_no_c_library_from_the_machine`]
        // and the release tool calls it before anything is signed.
        //
        // **159 from 2026-09-27**, when the plan gained B10a — a development
        // build may place a server program built on the user's own computer,
        // and no release build can. It went onto the §5 entry above in the
        // sitting that built it, because the route is the desktop's.
        //
        // **161 from 2026-09-28**, when the plan gained **B8d** — a desktop
        // places the server package of its own release and of no other — and
        // **P3b** — an interval with no overlap is reported as *incompatible*,
        // with its direction, and never as the supervisor not answering. Both
        // are the Creator's ruling that servers follow the app's release (Row
        // 519). **Both are built, and on the desktop's side**, so, like B8c,
        // they go on no entry above: B8d at `App/Desktop app and
        // delivery/src/setting_up.rs` (the own-release address and the refusal
        // of a manifest naming another version), and P3b at that crate's
        // `src/servers/installation.rs` (the fourth branch) and
        // `src/servers/published.rs` (the direction). **This crate's part is
        // unchanged**: `crate::bootstrap::inspect` gives the active release
        // those two compare, and it gained its first desktop caller the same
        // day. Moved by step 10c's transfer wave piece 8, on the build
        // runner's ruling.
        //
        // **162 from 2026-10-04**, when the plan gained **S15c** — a transfer
        // keeps its source's modification time, set on the staged file before
        // publication. ~~**S15c is owed**: nothing in this workspace sets a
        // file's time yet, and piece 10 of `Audits/Build/Step 10c/The
        // Creator's second notes on the test copy — what each piece is, which
        // files it owns, and in what order.md` builds it. **It goes on no list
        // above** because [`NotBuiltHere::it_lands_at_step`] names steps 5 to
        // 8 and this clause arrived at step 10c; this line is where the page
        // records it, and piece 10 strikes it when it lands.~~ Moved by that
        // map's piece 1, on the build runner's ruling. **STRUCK 2026-10-08:
        // S15c is built**, by that map's piece 10, so it goes on no list above:
        // the loop's own steps in `crate::transfers::moving` (the call handed
        // over beside the close, the download's reading on the handle it
        // opened, and `the_time_is_kept` before the pre-state), with
        // `crate::subsystem::client`'s `send_the_times` and
        // `send_a_reading_of`.
        //
        // **163 from 2026-10-16**, when the plan gained **S12a** — a fresh
        // file is not asked for its staging folder, its staged size or the
        // removal before its staging object. **It is built, not owed**, so it
        // goes on no list above: `crate::transfers::moving`'s
        // `carry_out_a_fresh_file_under_a_publication_gate` and the loop's
        // `fresh` reading, with the desktop's upload and the dependency check's
        // send as its callers. Step 10c's owed piece 1, on the build runner's
        // ruling of that day.
        //
        // **164 from 2026-10-23**, when the seventeenth notes' design pass
        // gave the plan **S0a** — one saved start, and so one order, across
        // every kind of transfer Transfers shows. **Built** by piece 3:
        // desktop record version 48 saves parent stamps with one allocator;
        // Transfers shows local start times and sorts each group newest first.
        // Dependency-send parents use that same writer, including on reopen.
        assert_eq!(the_clauses_the_plan_defines().len(), 164);
    }

    #[test]
    fn the_two_lists_together_still_account_for_sixty_eight() {
        // 65 arrive later, 3 are no step's. **R12a was added on 2026-09-04**,
        // when the desktop-side scheduler was built and the window it fixes
        // landed there: this entry's own reason had claimed R12a was built
        // here, and nothing in this crate composes an achieved-rate reading at
        // all. **Three were added on 2026-09-03**
        // by this crate's first code audit — C11 and C14, the two
        // installation-and-run gates whose fields were written and read back
        // and compared with nothing, and S13, whose decision is built and whose
        // two readings are not. Each was absent from both lists, so the crate's
        // own record said all three were built.
        //
        // **61 from 2026-09-06**, when the plan gained P7 and
        // [`WAITING_ON_THE_PATH_SEAM`] became the third list.
        //
        // **59 later the same day**, when B8 and B9 came off the §5 entry
        // because they were built — the signed release manifest, the offline
        // release root and the non-rollback memory, at [`crate::release`]. B10
        // was never on either list.
        //
        // **60 on 2026-09-06**, when B8a joined the §5 entry — the clause that
        // says who serves a released package, which no working file of this
        // crate keeps.
        //
        // **57 later that day.** Every clause on this page whose step had
        // closed was read against the workspace one at a time, and three of
        // them turned out to be built and named: **C20** at
        // `App/Server Setup/src/driving.rs`, **F10** at
        // `App/File Browser/src/free_space.rs`, and **F6a** at
        // `App/Rented-server runtime/src/health_readings.rs`. **T2 and T5a
        // were found built too** and their whole entry is struck in place
        // rather than removed, so they still count here.
        //
        // **54 later still, the same day**, when the nine owed clauses were
        // built and C7, C12 and C15 came off the §4 entry. **T11, T11a, G1,
        // G2, G3 and G9 were built with them** and their two entries are
        // struck in place rather than removed, so those six still count.
        //
        // **53 on 2026-09-07**, when **B8a** came off the §5 entry: the fetcher
        // that speaks HTTPS to the source a verified manifest names is built,
        // at [`crate::release::fetching`]. What is still owed by that clause is
        // the address itself, which is the Creator's and is not a builder's to
        // write down.
        //
        // **54 on 2026-09-19**, when **B8b** joined the §5 entry: nothing reads
        // the repository's fixed latest address, and step 10c is what does.
        //
        // **55 on 2026-09-27**, when **B10a** joined the §5 entry: its route is
        // the desktop's, at `App/Desktop app and delivery/src/the_desk_program.rs`.
        let arriving: usize = EVERYTHING_ARRIVING_LATER
            .iter()
            .map(|part| part.clauses.len())
            .sum();
        let waiting: usize = WAITING_ON_THE_PATH_SEAM
            .iter()
            .map(|part| part.clauses.len())
            .sum();
        assert_eq!(arriving + waiting + NOT_THIS_CODES_AT_ALL.len(), 55);
    }
}
