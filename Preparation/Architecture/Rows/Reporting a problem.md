# Rows — Reporting a problem

Rows **451 to 466**, from `Preparation/Specs/Reporting a problem.md`. Part of `Preparation/Architecture/Coverage.md`; how to read a row is there, and the facts they wait on are in `Preparation/Architecture/Facts.md`.

Every number here is permanent, never reused and never renumbered.

**Filed 2026-08-25 at the ninth sitting, from the first sweep this document has ever had.** The spec entered the Specs on 2026-08-25 and no row anywhere in this project touched it before these sixteen. ~~**All sixteen arrive unanswered**, and the count says so.~~ **All sixteen were answered on 2026-08-25 at the tenth sitting**, and every row below carries its own *(Superseded 2026-08-25)* note. *(This header was the filing-day statement and kept none of them, so the one place in this file still saying the sixteen are unanswered was its own head. Struck 2026-08-29, routed by the `Reporting a problem` stage-close gate.)*

**What is unusual about this file, and a reader should know it before reading a row.** `Preparation/Architecture/Method.md` names three things that earn a row: the document says outright that something is Architecture's; or it carries an unfinished-spot marker naming Architecture; or it promises an outcome that cannot be reached without a choice that is not obvious, or that rests on an outside fact, or that needs something else to hold before it can be kept. **In this source document the first two are met nowhere.** The word *Architecture* does not appear in it at all, and it carries no `[PENDING: …]` marker of any kind. **So all sixteen rows below were found by the third test alone**, and a sweep that collected only named markers would have returned an empty list. This is the extreme case the Method predicts when it says that the third test does most of the work and that an agent which only collects the named markers has swept nothing.

**Two labels in the source do name a stage, and both are answers rather than open spots.** *Where the user is offered this* and *After a crash, Smoozy speaks up* each carry **settled at Design** (2026-08-25) — the failure-side addition's appearance, and the box with two controls. Rows 466 and 464 are filed against what the entry point must be handed and against the mark and the launch-time reading, **not** against what either looks like.

**There is no companion sources file.** `Preparation/Specs/Sources/Reporting a problem.sources.md` does not exist and that folder holds thirteen files, none of them this one. Every label in the source is **Creator-stated**, **settled at Design** or **unresolved**, so nothing here is sourced by citation. Both *unresolved* labels were checked at the sweep and **neither is stale**: Row 456's has no answer anywhere in `Preparation/Architecture/`, and Row 463's was looked up by the fixed route through `Preparation/Blender Research/Across topics/The map.md` and is fresh, with what the research does settle quoted on the row so nobody pays for it twice.

**Three facts, numbered by the ninth sitting's consolidation plan: 129, 131 and 132.** ~~One wants a rented machine and rides free on a batch already owed.~~ **It rode free on the Pile 3 batch of 2026-08-25 and closed**: Fact 131 came back no — Blender's crash file carries no environment values — so §6 G5 takes the file and Row 463 closed answered. *(Corrected 2026-08-29 with the header above.)* The other two want a computer running each of the three operating systems Smoozy itself runs on, and the Windows third of each has not been posed — no Windows machine has been rented for it, and `Preparation/Groundwork/Open Threads.md` carries the named Windows list. A Windows machine can be rented, so this is untried rather than unposable; but every one this project has read reported a Server edition, so a desktop-only part of it is still out of reach of what has been rented.

**Quotes carry the source document's own markup**, so the coverage gate's quote search matches on a plain string. Links inside a quote are dead where they sit here and are left dead.

---

**Row 451 — the diary: one rolling record on the user's own computer, capped at 20 MB.** *What Smoozy keeps, so that there is anything to send.*
> Smoozy keeps a running record of what it did — **the diary**. It is Smoozy's own, on the user's own computer, and the user is never asked about it, never has to switch it on, and does not meet it as a feature. It exists so that when a report is made there is something in it.

and
> **One rolling record, capped at 20 MB, oldest lines dropped as it fills.** Label: **Creator-stated** (2026-08-24) — the size cap is the Creator's shape; the number is the runner's and is theirs to overrule. It is fixed rather than read off the disk, so no free-space reading decides anything.

Waits on: no outside fact. **Where it sits is already answered and must not be re-derived**: `Preparation/Architecture/Desktop app and delivery.md` L3 — the per-user application-data location comes from each system's own call for it and is *never* assembled from the login account name.

**The decision, and it is a shape decision rather than a detail.** This is a durable store on the user's computer that is **not** the local record, and two rules already standing say it cannot simply be added to one. `Preparation/Architecture/Desktop app and delivery.md` A2: *"No module has a durable store of its own. There is one local record … and this document builds it."* L1: *"There is one durable local record and this document builds it to the inventory of §1.1. No row is added to that inventory here."* And `Preparation/Architecture/State, identity and recovery.md` §1.1 owns that inventory. So the diary is either a new row in that inventory — which means opening the one document that owns it — or a fourth thing on this side that is not state, alongside L2's three (the secret store, the staged-transfer area, the fetched-derivative and throwaway known-hosts areas). Nothing anywhere takes that decision, **and the answer decides which document the rest of these rows are written in.**

Constraints the answer must honour.

- **It is line-oriented plain text, not a database.** *"oldest lines dropped as it fills"*, and the document says of the report cut out of it that *"it is plain text"*. `Preparation/Architecture/Desktop app and delivery.md` L1's store is a database with a schema version and migrations (L5–L8), which is a different shape from a file whose oldest lines are dropped.
- **Every local write goes through the platform file adapter** — A4 of the same document, *"A module that writes by any other route is outside Row 303 entirely"* — and A1 makes a module that reaches the operating system directly a broken boundary whose *"failure is invisible from inside that module"*.
- **The cap is fixed rather than read.** The spec says so outright, which is the rule `Preparation/Architecture/State, identity and recovery.md` §11.4 already imposes on every other bounded store on this side and `Rented-server runtime` R2/E6a imposes on the machine's journal — *"no free-space reading raises or lowers it"*. So the same rule, on a new store.
- **It is written while the application may die at any moment.** D6 requires every durable write at the moment the fact becomes true and nothing on exit. A record trimmed by rewriting itself has a window in which a crash loses the whole of it, which is the one crash this whole document exists to be able to report.
- **Smoozy never sends it.** *"Smoozy never sends anything, to anyone, ever. Not the diary, not a report, not a count of anything."*
- **What goes in it about Smoozy itself is bounded by what is already sent.** The spec's *"nothing new about the user is learned by writing them down"* rests on `Preparation/Architecture/Desktop app and delivery.md` E11's list being exactly which version is running and which system it is running on, and nothing more.

**Why it earns a row** Test 3, a decision to take and a dependency to satisfy. A durable store on the user's computer that no document's inventory holds, that the one rule against per-module stores forbids, and whose shape is not the shape of the store that rule points at.

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §1 R1–R8.

**The shape decision, which was this document's to take and which decides where the other fifteen are written: the diary is a fifth durable *non-state* thing on the desktop side**, beside `Desktop app and delivery` L2's four, and **not** a row in `State, identity and recovery` §1.1's inventory. The grounds are at R2 and they are two: L1's store is a database with a schema version and migrations while the diary is self-evicting line-oriented text, and **no state fact is ever read back out of the diary**. The inventory and lifetime-class naming was routed to that document and **landed there on 2026-08-25**: §1.1's durable table carries the diary at **I1** among the non-state entries, and §11.1 names it among the objects outside the five lifetime classes **as a store rather than a fourth object of the same kind**. §12 item 4 is closed.


---

**Row 452 — what Smoozy did, in order, with times, from every part of the app into one record.** *What Smoozy keeps, so that there is anything to send › What goes into it.*
> **What Smoozy did, in order, with times.** Its own acts, the ones a user could name: connected, read a file, started a job, sent a batch, published a download. Not its internals — a record of those is unreadable to the Creator and unshowable to anyone.

Waits on: nothing to get. A dependency to satisfy, and it binds documents that are already written.

**What the answer owes.** The five acts the spec names — connected, read a file, started a job, sent a batch, published a download — belong to five different modules, and `Preparation/Architecture/Desktop app and delivery.md` `Section 21` lists every service a feature module may reach and **there is no diary service in it**. A1 makes a module that reaches past that list a broken boundary. So either the diary becomes a named service of that document with rules of its own, or every module writes to a store nothing supplies — which is exactly the failure shape G2 already names for suppressed warnings: *"A module remembering its own satisfies its own spec and breaks this one, and nobody finds out until a user presses the control and one warning does not come back."* Here nobody finds out until the Creator reads a report with a hole in it and cannot tell the hole from a thing that did not happen.

**The decision nobody has taken.** *The acts a user could name* is not a list any document holds. A7 of the same document is the existing shape for this — a module declares, in its own document, the four things the services need — and a sixth declaration would be the same mechanism. Nothing says which it is, and a diary each module fills to its own taste produces a record that reads as complete and is not.

**And the boundary is not obvious in one direction.** *Not its internals* is a rule about what is left out, and every architecture document on this side is written in internals. Nothing says who draws that line or where.

**Why it earns a row** Test 3, a decision to take and a dependency to satisfy: a record fed by every module in the app, against a boundary rule that currently forbids any of them from feeding it.

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §3 J1–J3.

A **diary service** with one writer (§1 R4), routed into `Desktop app and delivery` §21's closed list and **landed there on 2026-08-25 as that document's §5.3 L14–L16**, with each module declaring its diary acts as **A7's fifth declaration — the A7 extension landed the same day, written into A7 itself**. §12 item 5 is closed. *The acts a user could name* is fixed as **the acts the owning spec names as outcomes**, and internals are excluded by the declared fixed line forms rather than by a judgement at the write.


---

**Row 453 — what Smoozy was told, and never on one clock.** *What Smoozy keeps, so that there is anything to send › What goes into it.*
> **What Smoozy was told.** The endings and readings that came back from a machine, and that machine's own Blender version and Linux family, which nothing else in Smoozy holds. Smoozy's acts and the machine's answers together are the thing neither side has alone. **But never on one clock.** The user's computer clock is undefended, so every moment says which clock it came from, and the machine's own order comes from its journal's own counting rather than from time.

Waits on: nothing to get. **Three of the four pieces are already answered and none of them is to be re-derived:**

- **The Blender version** is read and confirmed already — `Preparation/Architecture/Rendering.md` cites `Job files and outputs` D7, *"the material is composed for one named Blender version, names it, and the version is confirmed before Blender is launched"*, and L14 fails the attempt where the machine's Blender does not answer with its own name.
- **The Linux family** is read already — `Preparation/Architecture/Server Setup.md` M1a: *"every setup run reads the machine's *distribution family*"*, from `/etc/os-release`'s `ID`, documented against `os-release(5)`.
- **The two clocks** are D5 — `Preparation/Architecture/State, identity and recovery.md`: *"Every observed reading carries the moment it was taken and the clock it was taken on, and readings taken on different clocks are never subtracted from one another."* The machine's own counting is E1's per-machine sequence number, *"that only increases, before it is offered to any reader"*.

**The dependency, and it is the one thing that is new.** The diary sits on the desktop and the ordering evidence sits on the machine. E5 says a reader detects a gap by itself and *"nothing writes a marker for it"* — a reader whose cursor is below the journal's oldest retained number *"says so, and does not present the events it did receive as a complete account"*. A diary that copies a machine's events across without carrying that judgement produces a record the Creator reads as a complete sequence when it is not, which is the same fault Row 460's *quiet trim* rule refuses on the other axis.

**And one claim in the promise is the spec's, not this stage's to check.** *"which nothing else in Smoozy holds"* — the two readings above are taken today for other purposes; whether either is retained anywhere the diary could read is a question for whoever writes the answer, not for the sweep.

**Why it earns a row** Test 3, a dependency to satisfy. The facts under it are in hand; what nothing settles is a desktop-side record built out of a machine-side sequence whose own document says a reader must state where it is incomplete.

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §3 J4–J5.

Each told line carries the machine moment, the journal sequence number and the installation identity; **order comes from the sequence and never from time**, citing D5. **E5's gap is carried into the diary as a gap line**, so a reader's incompleteness travels into the report rather than being lost at the copy — which is what Row 460's *never a quiet trim* refuses on the other axis.


---

**Row 454 — what failed, in the same words Smoozy put it to the user.** *What Smoozy keeps, so that there is anything to send › What goes into it.*
> **What failed** — the failure as Smoozy already put it to the user, in the same words.

Waits on: nothing to get. A dependency to satisfy, and the promise's exact wording is what makes it one: *in the same words*.

**What stands in the way, stated rather than judged.** `Preparation/Architecture/State, identity and recovery.md` §8.2 makes a condition record *"a derivation, never a copy"* and §8.3 makes the condition identity opaque; the record holds an identity and the observed values it was raised over, not prose. The words the user actually met are composed at presentation — `Preparation/Architecture/Desktop app and delivery.md` `Section 16`, the presentation plumbing, with the wording itself Design's. So *the same words* exist at one end of the app and the record exists at the other, and nothing says which the diary holds. Composing them a second time at report time is not *the same words* if the presentation has changed since; storing the composed sentence is a second copy of something §8.2 deliberately does not keep.

**And a constraint the answer may not step over.** `Preparation/Architecture/Security and licence.md` §13 rule 6: a failure message *"names the thing that failed, never the credential it failed with"* and *"carries no passphrase, no fingerprint and no path beyond what the user already sees"*. Carrying the message verbatim inherits that rule; deriving a richer sentence for the Creator does not, and Row 458 on what the diary holds is where that collides.

**Why it earns a row** Test 3, a decision to take: the promise names a form the architecture's failure record explicitly does not hold.

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §3 J6.

The composed plain sentence is **captured verbatim at first presentation per condition identity**, by §16's plumbing — the hook was routed to `Desktop app and delivery` and **landed there on 2026-08-25 as its §16.8 M32** — and is **never recomposed at report time**. That is what keeps *in the same words* against §8.2's *a derivation, never a copy*; §13 rule 6 is inherited by construction rather than restated.


---

**Row 455 — what Smoozy assumed and found wrong.** *What Smoozy keeps, so that there is anything to send › What goes into it.*
> **What Smoozy assumed and found wrong.** The valuable one. A dozen places in the plan expect a fault of this kind to be caught while the app is being built, and nothing anywhere answers for one turning up while the app is running. These are Smoozy's own bugs rather than the user's or the machine's, and today they are silent everywhere.

Waits on: nothing to get. A decision to take, and it reaches every architecture document already written rather than one of them.

**What the promise is actually asking for.** The clauses it counts are real and they are addressed to a builder at design time — the pattern is *a builder that reaches here has found a fault*, in `Preparation/Architecture/State, identity and recovery.md`, `Preparation/Architecture/Desktop app and delivery.md` (P2, P3, P10, S8, X9), `Preparation/Architecture/Servers.md` D7, `Preparation/Architecture/Server protocol and transfers.md` (C6, G1), `Preparation/Architecture/Job files and outputs.md` N2 and `Preparation/Architecture/Keys.md`. None of them says what happens if the assumption is false while the app is running. The one clause that does is `Preparation/Architecture/Desktop app and delivery.md` H36 — the web-content boundary fails closed and *"The condition is recorded through the ordinary failure plumbing"* — and that records a condition to the **user**, not a cause to anyone else.

**The decision, in three parts, none of them taken anywhere.** What a run-time assumption failure *is* as a recordable thing; what the app does when it meets one, given that D6's *no durable state depends on shutting down cleanly* makes carrying on the ordinary case; and how it is written down without becoming an internals record, which Row 452 forbids. A design-time note has no shape at run time, and an answer that just widens H36's route turns every such fault into something the user meets — which no spec grants.

**A constraint that bounds it.** `Preparation/Architecture/Security and licence.md` §13 rule 5: *"A tool's own output is captured whole or not at all, and is never mined for a cause."* A record of assumptions found wrong is Smoozy reasoning about a cause, which is permitted about **its own** state and is not permitted about a tool's words.

**Why it earns a row** Test 3, a decision to take, and it is the shape-deciding one in this document: it is a requirement on every architecture document already written, and the spec says outright that today the thing it asks for is *"silent everywhere"*.

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §3 J7.

An **assumption line** carrying the declared site identity, written at every place an architecture document says *a builder that reaches here has found a fault*. **The app carries on** — D6 makes that the ordinary case — **H36's route is not widened**, so nothing new reaches the user, and §13 rule 5's *never mined for a cause* is untouched because this is Smoozy reasoning about its own state.


---

**Row 456 — what Smoozy managed to say on its way down.** *What Smoozy keeps, so that there is anything to send › What goes into it.*
> **What Smoozy managed to say on its way down, where it got the chance.** A failure record exists for a connection dropping or a write refused; by its nature none exists for the case where Smoozy itself died. Without this the Creator learns only that a run ended during an upload — not whether it was the transfer, the machine running out of memory, or the user force-quitting, which leave the identical absence of a mark. **Its limit is stated rather than hedged:** a process that is killed outright runs no code of its own, so nothing is written, and a force-quit and the machine running out of memory stay indistinguishable from each other and from any other death. What this catches is the narrower case of a fault Smoozy sees coming and survives long enough to write down. Label: **unresolved** — exactly what an operating system permits a dying process to write is an outside fact and needs a source before it becomes a promise.

Waits on: **Fact 129** — what each of Mac, Windows and Linux permits a process that is being ended to run before it is gone: which kinds of ending give a process any chance to run code at all, what a write started at that moment is guaranteed to reach disk, and which endings give nothing, with a force-quit and the machine running out of memory named specifically, since the promise rests on those two being indistinguishable.

**The *unresolved* label was checked and it is not stale.** The document has no sources file, nothing under `Preparation/Architecture/` answers it, and `Preparation/Groundwork/Open Threads.md` names it as one of two things this stage inherits from this spec rather than something already settled. It is an operating-system fact, not a Blender one, so Rule 2 puts it in this stage's own hands and forbids answering it from memory.

**What the promise already concedes, and the answer may not quietly widen.** The spec states its own ceiling — a process killed outright writes nothing, and force-quit, out-of-memory and any other death are indistinguishable on the mark alone. An answer that claims to tell them apart is answering a promise this document did not make. **A negative answer therefore does not break the promise**: this closes as unresolved with the catch narrowed rather than as an escalation.

**Two constraints already standing.** `Preparation/Architecture/State, identity and recovery.md` D6 — *"Every durable write happens at the moment the fact becomes true, and nothing is written on exit"* — which this is deliberately shaped to fit, since the fault is written when Smoozy sees it rather than on the way out. And A4 of `Preparation/Architecture/Desktop app and delivery.md`: this write goes through the platform file adapter like every other, at the one moment the application is least able to complete anything.

**Why it earns a row** Test 3, a fact to get. It is the only thing in this document that distinguishes a crash from a clean close beyond the bare mark, and the fact under it is unanswered for all three operating systems.

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §3 J8–J9.

Answered against **Fact 129** at the width its citation check fixed. The last line is written by a **raw write forced down** — the platform's own full-sync call on each system — because the check reproduced a buffered write in a handler producing a **zero-byte file** while the handler had demonstrably run. Endings that run no code write nothing, and the spec's own ceiling is carried rather than widened: **a force-quit and the machine running out of memory stay indistinguishable.** The macOS width qualifier — the programming interface was exercised and the Force Quit panel was not, and never against an unresponsive app — is carried at §12 item 3.


---

**Row 457 — what the diary never holds.** *What Smoozy keeps, so that there is anything to send › What goes into it.*

**Re-quoted 2026-10-26 for the Creator’s decision of 2026-10-04. Answered; all remaining exclusions stand.**

> **What it never holds.** Nothing from the secret store: no passphrase, no key of any kind, no fingerprint. And never the contents of the user’s files.

**Answer:** `Reporting a problem` N1–N2 and N5 enforce this at the write, never by filtering the report. The secret-value and host-key/fingerprint bars, machine-environment bar and file-contents exclusion remain. The licence-key and computer-identity exclusions and their §13 rule 4 dependency are withdrawn because neither is collected. `Desktop app and delivery` S6 and `Security and licence` B30 and §3.0 rule 6 still prevent stored values reaching feature modules.

**Former row, retained as history — superseded 2026-10-26.** Its licence-key/computer-identity dependency is withdrawn:

> **Row 457 — what the diary never holds, and the computer identity a report is not covered for.** *What Smoozy keeps, so that there is anything to send › What goes into it.*
> > **What it never holds.** Nothing from the secret store: no passphrase, no key of any kind, no fingerprint, no licence key, no computer identity. And never the contents of the user's files.
>
> Waits on: nothing to get. A dependency to satisfy, and one part of it is a hole the project already knows about.
>
> **Four of the five are already bound; the fifth is not, and this is `Preparation/Groundwork/Open Threads.md`'s own finding restated so the row's dependency is accurate.**
>
> - **The passphrase, the keys, and any stored value** — `Preparation/Architecture/Security and licence.md` §3.0 rule 6: *"No screen, notification, message, exported record or log carries a stored value or any part of one, and no get is ever performed in order to compose something to show."* An **exported record** is named. Covered.
> - **The fingerprint and any host key** — §13 rule 3: *"No host key, and no host key fingerprint, appears in any log, message or exported record."* Covered.
> - **The computer identity** — §13 rule 4: *"No licence key, no computer identity, and no part of either is written to any log, including a failed-check record."* **It binds *any log* and nothing wider.** A report file the user attaches to a mail is an **exported record**, not a log, so rule 4 does not reach it. Either a rule for this file says it, or rule 4 widens; both are changes to an audited document and need the Rule 7 line.
> - **The contents of the user's files** — nothing forbids it today because nothing has ever read a `.blend`'s contents into a record. It is a new prohibition on a new store.
>
> **And whether the diary is itself a log is the question underneath.** If it is, rule 4 covers the diary and not the report cut out of it, so a value permitted in the record becomes forbidden the moment it is exported — which is a rule that has to be applied at the cut rather than at the write. If it is not, neither rule 4 nor anything else reaches it. Nothing anywhere says which.
>
> **One thing that is structural rather than a rule to remember.** `Preparation/Architecture/Desktop app and delivery.md` §9, *The secret adapter and its helper*, S6 — *"No module receives a plaintext, and no interface of this document returns one"* — together with the handle-or-use-on-behalf rule that clause implements, `Preparation/Architecture/Security and licence.md` §1.4 B30 — *"No feature module receives a stored secret as a value it may keep; a module receives either a use of the secret carried out on its behalf, or a handle it can present again"* — mean a passphrase or a key has no value form for a diary to write, whoever forgets. The identity does not have that protection. *(Citation corrected 2026-09-02. This read `Security and licence` §9, which is **The local record** and says neither thing; the quoted words are `Desktop app and delivery` §9's and the handle-or-use-on-behalf rule is `Security and licence` §1.4 B30. Wrong document, right section number, and both cited sections were read as they stand on this date. Nothing the row concludes changes.)*
>
> **Why it earns a row** Test 3, a dependency to satisfy. Four prohibitions ride existing rules; one falls through a rule's stated width, and the failure is silent — a report reads correctly and carries an identifier the Creator was never meant to receive.
>
> *(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*
>
> **Not answered.**
>
> **Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §4 N1–N2, N5.
>
> **The boundary is enforced at the write and not at the cut**, on §13's own *never emitted* framing: **nothing enters the diary that may not leave in a report**. N2 states five exclusions and the split is three and two. **Three cite rules already standing**: a stored secret value (the adapter contract — `Desktop app and delivery` §9 **S6**, built on `Security and licence` §1.4 **B30** — together with `Security and licence` §3.0 rule 6), a host key or its fingerprint (§13 rule 3), and the machine's environment (§13 rule 2). *(Citation corrected 2026-09-02, the second copy of the same slip corrected ten lines above. This read *§9's adapter contract*, which in the run of `Security and licence` citations around it reads as that document's §9 —* The local record *, which states neither rule. The clause this sentence reports on, `Preparation/Architecture/Reporting a problem.md` §4 **N2**, was corrected the same day and carries both homes quoted; this line was changed only after N2 was read as it stands, so the row does not run ahead of the clause it quotes. Nothing the answer concludes changes.)* **Two are written as this document's own** — the computer identity and the licence key, because §13 rule 4 stops at *any log* and a report file is an exported record, and the contents of the user's files, which no standing rule reached at all. The widening of rule 4 itself was routed to `Preparation/Architecture/Security and licence.md` and **landed there on 2026-08-25**: §13 rule 4 now bars either value from *"any log, any message, the diary, or any exported record"* in its own text. §12 item 6 is closed.
>
>
> ---

**Row 458 — what the diary does hold that is the user's.** *What Smoozy keeps, so that there is anything to send › What goes into it.*
> **What it does hold that is the user's**, and this is deliberate: file paths, folder names, scene names, machine names, usernames. That is what makes a report worth reading. The user is the one who sends it, so nothing goes anywhere on their behalf that they did not send.

Waits on: nothing to get. A decision to take, and it is the one place in this document where a promise is a positive instruction to write down material every existing rule was written to keep out.

**What the answer has to reconcile, quoted so nobody answers half of it.**

- `Preparation/Architecture/Security and licence.md` §3.0 rule 1 — the secret store's subject is opaque and carries *"Never a host, a port, a username, a file path, a key comment or a fingerprint"*. That rule is about the secret store's own attribute set and reaches no further, but it is the exact list this promise inverts, so an agent reading both without care will apply the wrong one.
- §13 rule 6 — a failure message *"carries no passphrase, no fingerprint and no path beyond what the user already sees"*. Row 454 puts the failure message verbatim into the diary, so the two promises meet in one record: a path the user already saw is permitted there, and a path they did not see is not.
- §13 rule 2 — *"the machine's environment is never captured into any record Smoozy keeps or shows"*, on the observed ground that a rental's own settings held a third party's secrets in plain text. A machine name is not an environment value; a machine's own words can carry one.
- §13's own framing: *"Redaction here is *never emitted*, not *filtered before display*. A filter has to recognise what it is hiding."* So the boundary is decided at the moment something is written into the diary, not at the moment the report is cut — and that is the opposite of where the exported-record rules on Row 457 bite.

**The decision.** Which of those surfaces the diary counts as, and therefore which of the two ends binds each item on the list. Nothing says it. An answer that treats the diary as a log inherits four rules and loses the promise; one that treats it as neither inherits none and writes whatever is convenient.

**Why it earns a row** Test 3, a decision to take. The promise is deliberate and is quoted here as the document words it — it is not for this stage to say whether it is right — but nothing in the project tells a builder where the line between *what makes a report worth reading* and the redaction rules already in force actually falls.

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §4 N3–N5.

The deliberate list is kept, under the rule that what enters must be able to leave. **N4 names the two rules an agent would otherwise misapply** — §3.0 rule 1 is the secret store's own attribute set and §13 rule 6 is a failure message's width — and **N5 rules the diary its own surface, held to the exported standard**. That ruling is what keeps this row and Row 457 from contradicting each other, which was the risk in answering either alone.


---

**Row 459 — a report is a slice, and the three slices are cut by a thing nothing on this side has.** *A report is a slice of the diary, not the diary.*
> Label: **Creator-stated** (2026-08-24), and this is what keeps a mail attachment small.

and
> - Reporting a **failed job** takes that job's own acts and the session around them.
> - Reporting a **crash** takes the session that died.
> - Reporting **nothing in particular** — the Saturday drive above — takes the current session and the one before it.

Waits on: nothing to get. A dependency to satisfy, and it is the load-bearing one in the report half of this document.

**All three slices are cut on a *session*, and no such thing exists in this project.** Searched across `Preparation/Architecture/` at the sweep: there is no application-run identity, no session identity, and nothing that names one run of the desktop application apart from another. What does exist is adjacent and is not it — `Preparation/Architecture/State, identity and recovery.md` §11.1's **period-held** class, which the same section says explicitly is not the application run (*"the application run ending is one of the ways a period ends and not the only one"*), and its **run-held** class, which is in process memory and therefore gone by the time anyone reads a diary. So *the session that died* and *the one before it* both name a thing nothing writes down.

**And the first slice needs a second key.** *That job's own acts* means the diary's lines have to be attributable to a job identity, which means every module's writes carry one where there is one — which is Row 452's declaration question arriving here as a hard requirement rather than a tidiness point.

**The dependency runs the other way too.** The crash slice is cut for the report Row 464's quit mark offers at the next launch, so *the session that died* has to be identifiable **after** the run that held it is gone. That is the same fact D6 makes hardest to have.

**Why it earns a row** Test 3, a dependency to satisfy and a shape-deciding one: the record's keying is decided by these three sentences, and a diary written without them cannot be sliced at all afterwards.

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §2 U1, U7; §5 S2.

**A run identity is issued at application start**, under §2.1's rules, and every diary line carries it. **Nothing in this project named one run of the application apart from another before today** — that absence is what made this row and two others unanswerable. The three slices are cut on it and on job identities; the register row was routed to `State, identity and recovery` §2.2 and **landed there on 2026-08-25** — the register carries the run identity, issued by the desktop application at application start, unique across every run of this installation. §12 item 4 is closed.


---

**Row 460 — drops from the middle, says how much, and never a quiet trim.** *A report is a slice of the diary, not the diary.*
> **Three things keep it small enough to attach:** it is a slice and not the whole record; it is plain text; and where a slice would still be too large, Smoozy drops from the middle, where the least is happening, and **says how much it dropped**. That is the behaviour Smoozy already has for a machine's output that would not fit.

and
> **What must never happen is a quiet trim.** A report that silently left things out is one the Creator reads as complete. Whatever was dropped is said in the file itself, so the statement travels with it.

Waits on: nothing to get. A decision to take, and the promise's own claim about precedent is where it starts.

**The behaviour it cites is not the behaviour it describes, and this row names that rather than judging it.** The existing mechanism is `Preparation/Architecture/Rented-server runtime.md` R2 and M4a: a stream capture meets an immutable ceiling, and *"Reaching it removes or edits nothing already retained … the retained prefix stays for R1's whole lifetime, and the explicit *collection incomplete* outcome tells the user that Smoozy could not keep the rest."* **That keeps a prefix and abandons the rest**, on the machine, as a stream arrives. This promise cuts the **middle** out of a record already complete, on the desktop, at the moment a report is made. What carries across is the honest statement; the mechanism does not.

**Two decisions inside it.**

- **What the size limit on a report is.** The spec gives 20 MB for the diary and no figure at all for the report; *too large* is a mail attachment's limit, which is the receiving provider's and not Smoozy's to know. Nothing says what it is or where the number comes from, and the same *fixed rather than read* rule Row 451's cap carries would apply to it.
- **What *where the least is happening* means**, given that the diary is lines of text and the answer has to pick a span without reading the content for meaning — which the *never mined for a cause* rule (`Preparation/Architecture/Security and licence.md` §13 rule 5) already refuses for a tool's output and which the diary's own value is that it is dense where trouble is.

**Why it earns a row** Test 3, a decision to take. Two numbers and one selection rule, none of them stated, on a file whose only failure mode is being read as complete when it is not.

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §5 S3–S4, S7.

A **report ceiling of 5 MB, fixed rather than read**, this document's own constant and the runner's to overrule. The trim is **one contiguous span, whole lines, chosen by position and kind and never by mining content**, grown outward from the slice's midpoint so the head and tail always survive; **statement lines — session, failure, assumption, gap and last — are excluded from the span, and are taken only where the statements alone still exceed the ceiling, in which case the cut statement says so**; **the drop statement is written into the file at the cut**, so it travels with it. R2/M4a is carried at its true width — the honest statement carries across, the keep-a-prefix mechanism does not.


---

**Row 461 — and it says what it could not include.** *A report is a slice of the diary, not the diary.*
> **And it says what it could not include.** *Do what can be done, and say what can't* holds here too. A report made with no server reachable holds none of the machine's words, and it says which of three things is true — absent, unreachable, or never collected. Collapsing those costs the Creator a round trip back through the user, which is the one thing a report is meant to avoid.

Waits on: nothing to get. A dependency to satisfy, and the three states this promise names are not the four the architecture already has.

**What exists.** `Preparation/Architecture/Desktop app and delivery.md` M10 holds a closed set of four collection outcomes, *"one per stream per attempt"*: *collected*; *collected, and the stream was empty*; *collection incomplete*; *never established*. M11 says the two the row exists to separate rest on two different records and *"nothing here re-derives either"*.

**Where the promise's three do not map onto that four.** *Never collected* is M10's *never established*. *Absent* covers M10's *collected, and the stream was empty* and arguably *collection incomplete*, which M10 insists *"never collapses into *printed nothing***. And **unreachable is not a collection outcome at all** — it is a statement about the moment the report is cut, not about the attempt, and nothing in M10's closed set can express it. So the report needs a fourth state of its own, taken at report time, laid beside a closed four taken at attempt time.

**And it reaches wider than the machine's words.** The promise's *what it could not include* covers every part of the report that a report-time condition can empty, not only the streams — Row 462's journal tail and per-job note are the obvious ones, and neither has a collection-outcome vocabulary at all.

**Why it earns a row** Test 3, a dependency to satisfy. A closed set exists and this promise needs a different one; an answer that reuses the four silently reports a reachable machine's empty stream and an unreachable machine identically, which is the exact round trip the promise exists to prevent.

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §5 S5–S6.

Per-part statements. **M10's closed four are carried as recorded and M11 is honoured**, and ***unreachable* is added as a report-time state laid beside them**, never overwriting one — which is what stops a reachable machine's empty stream and an unreachable machine reading identically. The cap-evicted head and an absent previous session are stated the same way.


---

**Row 462 — where a server is reachable, the report takes the machine's journal tail and its note on the job.** *A report is a slice of the diary, not the diary.*
> **Where a server is reachable, the report also takes what that machine holds about the job** — its journal's tail and its note on the job.

Waits on: nothing to get — the machine's stores exist and their contracts are written. A dependency to satisfy, and two of them have teeth.

**The journal is capped and drops the oldest, and a reader has to say so.** `Preparation/Architecture/State, identity and recovery.md` E6a: 32,768 entries per machine at a 512-byte per-entry budget, 16 MiB, *"fixed rather than read"*. E6: the oldest **acknowledged** entries are dropped first, and where the cap forces dropping an entry no cursor has passed, *"it is dropped and the oldest-retained number moves, which is exactly the case E5 makes visible"*. E5 requires a reader below the oldest-retained number to say it missed events and *"not present the events it did receive as a complete account"*. So a *journal's tail* taken for a report is a reader like any other, and it either registers a cursor — changing what the journal retains for every other reader — or it does not and has to carry E5's statement into the file. Nothing says which, and the first choice makes making a report change the machine.

**A tail is a length nobody has named.** *Its journal's tail* is not a quantity. The journal is bounded in entries, not in time, and E6a states the cap *"in work rather than in hours"* — a job costs a small multiple of one entry per finished frame — so a tail chosen as a count means one thing on a ten-frame job and another on a ten-thousand-frame one.

**And the read is a new act on the connection.** It happens at report time, on a machine that may be mid-render, and `Preparation/Architecture/Security and licence.md` §13 rule 5 binds what comes back: *"A tool's own output is captured whole or not at all, and is never mined for a cause."*

**Why it earns a row** Test 3, a dependency to satisfy. Three contracts already standing — a cap, a cursor discipline, and a gap statement — all of which a report-time read either honours or silently breaks.

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §6 G1–G4.

A job-scoped take on the ordinary connection. The tail is the **last 512 entries, fixed rather than read**, and **no cursor is registered** — so making a report never changes what the machine retains for any other reader. **E5's honesty is kept by stating the span and the oldest-retained number in the file itself.** The note is taken whole under §13 rule 5; the streams are not taken.


---

**Row 463 — Blender's own crash file, and what the research does and does not settle about it.** *A report is a slice of the diary, not the diary.*
> **Blender's own crash file, which the machine writes where Blender died, is the obvious further thing to take**, and a backtrace is exactly what tells a Blender fault from a Smoozy one. Label: **unresolved** — whether such a file carries values out of the machine's own environment is not established, and capturing those into any record is forbidden elsewhere. Until that is answered the crash file is not taken.

Waits on: **Fact 131** — whether a Blender crash file's backtrace section or Python backtrace section carries values out of the machine's own environment. It is a reading of a produced file rather than a Blender research question: the research has been consulted and does not answer it, and `Preparation/Blender Research/` is not to be opened again for it.

**The label is fresh rather than stale, and this was checked by the fixed route.** `Preparation/Blender Research/Across topics/The map.md` names `When a render ends.md`; that joined answer's *Whether a crash file is written* section names *Reports 1* and *The machine 4*; `Preparation/Blender Research/Comparison/Reports.md` question 1 was read for that question alone. **What is already settled and must not be paid for twice:**

- *"A Blender made to crash writes a crash file and says so, at 139, in all four versions: on Linux in the temporary folder under the loaded file's name, on the Mac in that machine's own temporary folder."*
- The file *"held a version line, a backtrace section and a Python backtrace section"*, and on Linux it is named after the blend-file — *"`Writing: /tmp/simple.crash.txt`"* — **not** `blender.crash.txt`, which is the only name the manual gives. With no file loaded it printed `Writing: /tmp/blender.crash.txt`.
- *"Anything printed immediately before the crash never reaches the ordinary output."* So for this one ending the crash file is the only place those words exist at all, which is what the promise is reaching for.
- **The file is written for one kind of ending only.** *"Running the machine's own memory out writes no crash file at all, at either of the two statuses that ending gives"*, and *"Whether one is written when a Blender is replaced under a running render is not settled — two runs of one test on one machine disagree and both stand."*
- Windows is untried, which does not reach this promise: the machines Smoozy renders on run Linux.

**What none of it answers is the label's own question** — whether either backtrace section carries environment values. The three sections are named and their contents are not described. So the residue is exactly one question and this row does not widen it. Because the trigger, the location and the sections are already known, the fact is answered by making one crash and reading the file, and it rides free on any rental taken for another batch.

**The rule that decides the answer.** `Preparation/Architecture/Security and licence.md` §13 rule 2: *"the machine's environment is never captured into any record Smoozy keeps or shows"*, on the observed ground that a rental's own settings *"hold secrets in plain text — an access key for the provider's own service, and a password for a web page it runs"*, with the finding's own conclusion that *"Anything that reads the machine's settings and writes them into a log would be writing those out too."* A crash file taken whole and attached to a mail the user sends is precisely that act if the answer is yes.

**Constraint on any answer.** The promise's last sentence is a standing instruction, not a hedge: *"Until that is answered the crash file is not taken."* An unanswered fact closes this as unresolved with the crash file excluded, and the promise still stands — it is the closeable kind rather than the Method's second escalation.

**Why it earns a row** Test 3, a fact to get, and a decision the fact settles outright in one direction.

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting, as unresolved with the promise standing.** `Preparation/Architecture/Reporting a problem.md` §6 G5.

**CLOSED ANSWERED the same day, on the Pile 3 rental.** **Fact 131 came back no.** Two marker values were planted in the environment — one shaped like a licence key, one like a private path — and Blender 4.5.3 was crashed twice, once with no scene and once with `heavy.blend` open. **Neither marker appears in either file, and neither file holds assignment-shaped text of any kind.** Both are 1775 bytes: a version header, a backtrace whose every path is the Blender installation's own or a system library's, and a Python backtrace of source paths and line numbers. `Preparation/Architecture/The Pile 3 batch — a full mount under a running supervisor, an interrupted heavy frame, and an hour of silence.md` §5 is the answer of record.

**So §13 rule 2 does not bar the file and G5 takes it.** The standing instruction is discharged rather than worked around: the condition it named has been answered, in the direction that permits the take.

**One thing the reading added that the row did not ask for**, and G5 carries it: the file's **name** carries the user's scene name while its **contents** never do — confirming the research's *named after the blend-file* on a second machine, and identifying the only user-derived string the take moves. **Nothing was rented for this**; it rode free on the Pile 3 batch exactly as the row said it would.

**The unresolved-bucket marker this row carried was withdrawn on 2026-08-29, and it should have gone with the closing note above on 2026-08-25.** The marker is the **only** thing that bucket is derived from — the row files are searched for it rather than a list being kept, which four separate gates recorded as the only way it could be enumerated — so a row that closed answered while keeping its marker made the derived set read **ten where `Preparation/Architecture/Coverage.md` reads nine**, and any gate deriving the bucket by script would have got the wrong answer. Found by the architecture runner at the stage close, deriving the bucket to refresh `Preparation/Architecture/What Build inherits.md`. **Nothing about the answer moves**; this is the bookkeeping catching up with it.

*What this row said while it was unresolved, kept as the record of it:* unresolved with the promise standing, filed as that document's §12 item 1. The crash file is **not taken**, which is the spec's own standing instruction rather than a hedge, and **Fact 131 is neither widened nor guessed**. Both closing branches are pre-stated: a negative answer adds the fetch under G1's shape, and a positive one keeps the crash file out whole — **a filtered take is refused by N1**, since a filter has to recognise what it is hiding. **Fact 131 came back no and the first branch is the one that was taken.**


---

**Row 464 — the quit mark, and the launch that reads it.** *After a crash, Smoozy speaks up.*
> **Smoozy writes a mark at the moment the user asks to quit.** If the next launch finds no mark, the run before it ended badly, and Smoozy says so and offers to report it.

and
> **This is a named exception to two things this project has settled**, granted by the Creator on 2026-08-24 — *"Smoozy may speak up, yes."* [Whole app](../../Specs/Whole%20app.md) says a failure appears on the thing that failed, that Smoozy does not interrupt with a box the user is not waiting on, and that there is no separate place failures are collected. A run that crashed yesterday is not a thing on any screen, so a launch-time prompt is refused by both halves. It is written down here as the exception it is rather than slipped in, and it reaches no further: the crash of the previous run, at the next launch, and nothing else. Label: **settled at Design** (2026-08-25) — a box with two controls, whose most important line says nothing was lost, because a crash and a clean close leave Smoozy in the same state and a prompt that did not say so would frighten a user about work that is safe.

*(The link inside the second quote is the source document's own and does not resolve from this folder. It is part of the quoted string and is left as it stands.)*

Waits on: nothing to get. A dependency to satisfy, and it is a named exemption to a rule in an audited architecture document.

**The exemption, stated exactly.** `Preparation/Architecture/State, identity and recovery.md` D6: *"No durable state depends on the desktop application shutting down cleanly. Every durable write happens at the moment the fact becomes true, and nothing is written on exit. A crash and a clean close therefore leave the same durable state, and there is no recovery path for one that is not the ordinary path for the other."* **The exemption falls on the third sentence, not the second.** The mark is written when the user *asks* to quit — a fact becoming true — so *nothing is written on exit* is untouched; what changes is that a launch now has a path that differs depending on how the previous run ended. `Preparation/Groundwork/Open Threads.md` records this as a named exemption needing a decisions-record line in that document under Rule 7, and that line is the runner's rather than an answering agent's.

**Four decisions the promise does not take.**

- **When the mark is cleared.** *The next launch finds no mark* only works if a launch that found one removes it, or if the mark carries a run identity — and the second needs the session identity Row 459 says does not exist.
- **Which run the report is then cut from.** The offer produces the crash slice, so *the run before it* has to be findable in the diary after it is gone.
- **Two copies at once.** Nothing in this project says whether a second running copy is possible, and one mark shared between two runs reports a crash that did not happen or misses one that did.
- **What the mark is.** D6's whole architecture is that nothing on this side depends on a clean close, so this is the first durable thing on the desktop whose *absence* means something. An absence caused by a failed write, a cleared application-data folder or a rollback is indistinguishable from a crash.

**And the cost is bounded by the promise itself**, which is what makes it closeable rather than an escalation: losing the mark costs one prompt that should not have appeared or one that should have and did not, and no durable state depends on it.

**Why it earns a row** Test 3, a decision to take and a dependency to satisfy — a named exemption to a rule an audited document rests on, plus four unstated decisions around a mark whose meaning is carried by its absence.

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §2 U2–U6, with U5a added 2026-08-29.

A **two-valued run mark** — the state and the run identity — in the local record, written at launch and again at the moment the user asks to quit, and **nothing on exit**. **D6's second sentence is untouched and the exemption falls on its third**, at one field and one reader; the decisions-record line is the runner's. All four of the row's open decisions are answered: the mark is never cleared but overwritten, the crashed run is named by its identity, two copies are handled by P2 with the update handover, and an absent field reads as *no previous run*. **U5a carries the one exception the handover forces, added 2026-08-29:** a launch the update mechanism started in probe mode performs step 1's read and write but **makes no offer**, because the field then holds *(running, old run's identity)* for a run that is still alive — without it every ordinary successful update would open C1's box with nothing crashed. The suppression is exactly as wide as the probe, the rollback branch is unchanged, and the launch-side half is `Desktop app and delivery` Z21b, which closed §12 item 10 the same day.


---

**Row 465 — Smoozy writes a single file, offers a place to put it, and opens the folder it landed in.** *The whole act, in one run through.*
> They open the Smoozy menu and pick **Report a problem**. Smoozy asks for one sentence in their own words, and they type *the frames aren't on the drive but Smoozy says they arrived*. Smoozy writes a single file, offers a place to put it, opens the folder it landed in so the file is in front of them, and tells them to attach it to a mail to **hello@smoozy.app**. They send it from their own mail account.

and
> **The file is not put in front of them to read, and does not need to be.** It is plain text, it is in a folder they chose and Smoozy just opened, so anyone who wants to look already can. Smoozy does not make them look. Label: **Creator-stated** (2026-08-25) — *"they don't need it to be readable to them"*. The consent question that would otherwise ride on this is answered by the act itself: the user is the one who attaches and sends.

Waits on: **Fact 132** — what each of Mac, Windows and Linux offers an application for opening a folder with one named file shown in it, and whether each system's file picker can be run in a save-a-new-file mode where the application supplies the name and the user picks only the folder. It also closes the gap `Preparation/Architecture/Download.md` H7 already has, so one answer serves two promises; the ninth sitting's consolidation records four sweeps reaching this fact independently.

**The boundary gap is real and the project already knows it.** `Preparation/Architecture/Desktop app and delivery.md` A1: *"A feature module reaches the operating system only through a named service of this document, and never directly … A module that opens a file, posts a notification, reads a name, draws a menu or opens a picker for itself has broken this boundary, and the failure is invisible from inside that module."* **`Section 21`'s service table names no folder-opening service.** It holds the platform file adapter, the picker, the menus, the notification service and the rest, and nothing that reveals a file in its folder. `Preparation/Architecture/Download.md` H7 already promises the same act — *"One place, and it opens"* — with no service behind it, so this promise inherits an existing gap rather than creating one.

**The picker that exists is a chooser, and this is a save.** `Preparation/Architecture/Desktop app and delivery.md` K11 supplies *"one service for every module that wants one"* and K12 leaves Row 307 to `File Browser`; every use of it in the project picks existing files or a destination folder. **Whether that matters turns on one clause of the promise**: Smoozy *"writes a single file, offers a place to put it"* — if Smoozy names the file and the user picks only the folder, the existing chooser is enough and no save-a-new-file dialog is needed. Nothing states it that way, and the two readings need different platform calls.

**Three constraints already standing.**

- **A4** — the write goes through the platform file adapter, and **K13**: *"Whatever a picker returns is a path from the user and is judged like any other name before anything is written. A path the user chose is not a path that is safe to write into."*
- **This is the first file Smoozy authors and writes.** `Preparation/Groundwork/Open Threads.md` records it as such. `Preparation/Architecture/State, identity and recovery.md` §11.3 puts the user's downloaded outputs outside Smoozy's state entirely — no lifetime, no cleanup, no cap — and a report file written into a folder the user picked is the same kind of object, which the answer should say rather than assume.
- **The user's own sentence goes into the file verbatim.** It is the user's text, not prose Smoozy composed, and `Preparation/Architecture/Desktop app and delivery.md` X9's verbatim route is the existing shape for a string that is data — although that route is about strings reaching a *surface*, and a file is not one.

**And two stances of the source document bind this answer rather than earning rows of their own.** *"Smoozy never sends anything, to anyone, ever. Not the diary, not a report, not a count of anything."* And *Smoozy points at the file and never at a mail program*, which the document closes with *"So this is the floor and also the ceiling"* — so the answer may not reach for a share act or a mail handler on any system.

**Why it earns a row** Test 3, a fact to get and a dependency to satisfy. The promise's central act — the folder opening so the file is in front of the user — has no named service on any of the three operating systems, and the one that would substitute for it is a chooser rather than a save.

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §7 V1–V10.

The user's sentence goes in verbatim and does not pass the typography point. **Smoozy names the file and the user picks only the folder**, so K11's existing chooser is enough and no save-a-new-file dialog is needed — the clause the row said the promise turned on is settled by the drawing. The write is staged and published atomically; **the reveal is a named service of `Desktop app and delivery`, written at its §18.6 K20–K24 and carried in §21's table**, built to Fact 132's per-system constraints — the same service that closes `Download` H7's serviceless *"One place, and it opens"*. The file is the user's under §11.3's shape, and the spec's floor-and-ceiling — Smoozy points at the file and never at a mail program — is kept.


---

**Row 466 — where the user is offered this: two places, and what each one has to be handed.** *Where the user is offered this.*
> Two places, and no third. Label: **Creator-stated** (2026-08-25).

and
> **On any failure the user is looking at.** The report belongs beside the thing that went wrong, because that is where the user is and that is when they care. A failure has one shape everywhere it attaches, so this adds one thing to that shape rather than a route of its own. What it looks like there, and what it does to a failure that has collapsed onto a single line of a list, is Design's. Label: **settled at Design** (2026-08-25) — it is the second and last addition the failure shape permits, quiet and after the raw text; a failure collapsed onto one line does not carry it, by the rule that already keeps the second sentence and the raw text off a row.

and
> **In the Smoozy menu, always.** Because the largest class of bug leaves no failure anywhere: the download that says it arrived, the render that came out wrong, the app that is simply slow. Nobody presses a red dot for those, and a route that only exists beside a failure would never catch one.

Waits on: nothing to get. A dependency to satisfy: what each of the two routes hands the report act.

**The menu half is nearly supplied and the failure half is not.** `Preparation/Architecture/Desktop app and delivery.md` `Section 18` supplies the menus' integration per platform, so the Smoozy menu has a home. *(Read **five menus** until 2026-08-29. The Creator ruled the Help menu out on 2026-08-25 — this row's own subject moving into the Smoozy menu is what removed it — and K5, K6 and §21's table were narrowed to four the same day. Nothing about this row's finding moves.)* **What is not supplied is what an entry point reached from a failure hands over.** The failure-side route has to name which of the three slicing rules applies and which job or session it applies to, and the failure record is where that would come from: `Preparation/Architecture/State, identity and recovery.md` §8.1 declares two kinds — a live condition and a recorded ending — §8.7 has every failure record name a durable subject, and §8.9 supplies a repeat key. **A live condition on a server row and a recorded ending on a history entry are not the same slice**, and nothing says which of the three rules each maps to. The menu route is the third rule and needs no subject at all.

**And *always* is a real requirement rather than a filler word.** The menu entry has to work with no server connected, no job in the queue, and — since the largest class of bug is the app being wrong rather than broken — with nothing failed anywhere. That is the one path in this document that touches no machine, and it is the one the whole document was written for.

**Why it earns a row** Test 3, a dependency to satisfy. Two entry points into one act, with the failure-side one needing a mapping from a failure record's kind and subject onto a slicing rule that nothing anywhere states.

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §8 X1–X5.

**The menu route hands nothing and works with nothing**, which is what keeps *always* structurally rather than by care. **The failure route hands five things** — the kind, the identity, the durable subject, the presented sentence and the moment — and the subject-to-slice mapping is fixed. **X3 is the single sentence the menu ruling touched, and the ruling has been made:** the Creator ruled on 2026-08-25 that the item is in the Smoozy menu, the Help menu was removed for want of anything else to hold, Row 481 is struck, and nothing else in the answer moved for it.


---

*2026-08-25 — filed at the ninth sitting's consolidation, from the architecture sweep of `Preparation/Specs/Reporting a problem.md`.*
