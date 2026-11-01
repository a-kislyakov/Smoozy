# Rows — Design: Server Setup

Rows **355 to 365**, from `Preparation/Design/Server Setup.md`, **Rows 439 and 440**, filed later by the both-ways pass over the Design documents, and **Rows 487 to 490**, filed at the ninth sitting from the Design close re-sweep of the same document. Part of `Preparation/Architecture/Coverage.md`; how to read a row is there, and the facts they wait on are in `Preparation/Architecture/Facts.md`.

Every number here is permanent, never reused and never renumbered.

**Filed at the stage close, 2026-08-20**, from the first sweep the Design documents have ever had, on the Creator's ruling of the same day. Every row says what kind of promise it rests on, because a drafted sentence may be reworded when Design resumes and the number stays either way.

**This document is dense with readings because it is a status screen.** Nearly every sentence in *This server*, *A long step* and *Cut off part-way* promises the user a value the app has to hold at the moment the screen is drawn, and that is where the rows are. Colour, spacing, the tab's position, the table's column order, the wording of a warning, the shape of the install dialog — all left alone as Build's. `Preparation/Design/Mockup/Smoozy.html` draws every screen this document describes (`#setupview`, `#buadd`, `#burun`, `#bufail`, `#burmmod`), so several of these promises are **a settled value** rather than a drafted sentence.

**One promise this document states is filed elsewhere, because a spec states it too.** The GPU-fallback line — *a job that asked for the GPU rendered on the CPU, and the line says what happened and what to do about it* — is **Row 308** in `Rows/Server Setup.md`, whose constraint paragraph now carries the Design quote and the one thing the drawing is missing.

**None of the document's ten `[PENDING: …]` markers names Architecture**, so none meets the second test and none is filed. Six are nonetheless already answered and are listed at the end, so Design does not decide them from nothing.

**Two of these rows rest on one missing reading.** Rows 355 and 356 both need *has this server ever been set up*, and it is arranged nowhere; the two are separate promises and are cross-referenced rather than merged. **Row 362 was named here too and does not belong** — it rests on which builds Smoozy placed and from where, answered at **B8a**.

---

**Row 355 — the card knows a setup happened, when it happened, and what it consisted of.** *This server.*
> "**One state line, when it was done, one action, and a list of what was done.** Nothing else."

and
> "**The list is the point.** *Ready to render* is a word the user has to take on trust unless the screen says what Smoozy actually did — the graphics driver, and Telegram reporting where Telegram is on. So the card names them. That answers *what is this section about* without a sentence explaining anything. Label: **Creator-stated** (2026-08-11)."

and
> "**The action is called what it does.** *Set up this server* on a server never set up, *Set up again* afterwards. It was *Run it again*, and *it* was never named anywhere on the screen."

Kind of promise: **a settled value** for the card's contents — the mockup draws it on all four card states as `set up 6 Aug at 14:02` beside `Ready to render`, over the list `Graphics driver installed` / `Telegram reporting installed`. The *list is the point* paragraph is additionally **Creator-stated** (2026-08-11). The action's two names are **a drafted design sentence**, and the mockup draws only *Set up again*.

Earns its row by the third test in its sharpest form. **This card is drawn on a connection opened long after the setup ran, and it shows three things about a setup that is over**: that one happened, the moment it happened, and the steps it consisted of. None of the three can be drawn at Build from anything the app has been arranged to hold. It is one promise with three readings, so it is one row.

Waits on: nothing to get. No outside fact is involved; the question is whether anything **durably** records a finished setup.

**Not answered anywhere.** Where the sweep looked, and what each says:

- `Preparation/Architecture/Server Setup.md` 3.2 (**Y3, Y4, Y6**) records boundaries with the moment on the machine's clock and makes them readable — but the machine-level setup-in-progress state is **cleared at *done*** (S6, and `Rented-server runtime` §9 P5), and Y6's promise is explicitly about *how far a half-finished setup got*.
- Section 4.1 (**S2**) composes what the user is shown from the fresh reading — *"the boundary list, the last one reached with its moment, and the run's ending where one was recorded"* — but S2 is written for a **part-finished** setup, and says in the same breath that the desktop *"holds its pending-command records (C3) and nothing else durable about a sitting"*.
- `Preparation/Architecture/Rented-server runtime.md` Section 13 **R1** retains *"an operation record and its command records"* until *"the same moment as their subject's machine-side record"*. **A setup operation's subject is not a job, so that rule has no term for it** — the retention of a finished setup run's record is stated nowhere.
- `Preparation/Architecture/Servers.md` **D17** enumerates everything a saved server durably holds — details, connection, remembered identity, remembered setup bit, machine password — and **there is no *last set up* among them.** Section 7 (B1 to B4) is the only setup memory that document has, and it is the opposite thing: a ticked option remembered **only** where the connection never succeeded, cleared the instant one is established.
- `Preparation/Architecture/Server Setup.md` **B12** and Row 104 govern what a machine holds — read afresh, period-held, never durable — and `State, identity and recovery` **D2** forbids inferring a desired state from an observed reading.

**So the nearest thing that existed was a record on the machine whose retention was undefined for this kind, and no document said this card was composed from it.**

**Two consequences worth carrying with the row.** The action's name needs only *has this server ever been set up*, **and that reading is missing too** — it is Row 356's other half. And **the list's two items are wrong on the container kind, which is observed rather than assumed *(Narrowed 2026-08-23, out of the 6.8 re-run repair: the *norm* half is one seller's own documentation about that seller's own instances, and the three-seller batch of 2026-08-22 recorded the other two shipping virtual machines. Nothing turns on which kind is commoner — every rule reading it is written per machine kind off a reading taken at every placement. `Facts.md` Fact 48's headline was narrowed the same day and seven restatements with it.)***: `Preparation/Architecture/Server Setup.md` R3 and Fact 48 settle that a container's driver is the host's and is **never installed**, so `Graphics driver installed` is a line that machine never earns.

**Answer, 2026-08-21: `Preparation/Architecture/Server Setup.md` [Y6a](../Server%20Setup.md#32-the-recorded-safe-boundaries), with the retention term it needs added to `Preparation/Architecture/Rented-server runtime.md` [§13 R1](../Rented-server%20runtime.md#13-retention-on-the-machine).** **Nothing new is recorded anywhere. What changes is that the record is not thrown away, and that a document says the card is composed from it.**

- **The row's search was right about every place it looked, and right about the one that nearly worked.** The setup operation record already holds Y4's boundary list and the last-reached marker with its moment, and **S6 clears the machine-level setup-in-progress state at *done* and clears nothing else** — the operation record is a different thing. What was missing was a retention term, which R1's table had none of because a setup operation's subject is the machine rather than a job. **It now has one: replaced by the next setup run *of the same kind* at run start, one record per machine *per kind of setup act*, ended by nothing else — except where an owning clause ties the record to a subject, as B8a does for a published Blender install.** The term was written flat on 2026-08-21 and **narrowed to per kind on 2026-08-25 by [Y6b](../Server%20Setup.md#32-the-recorded-safe-boundaries), per Row 487**, because under the flat term a Telegram placement or an install run would have replaced the ready action's record at run start and stripped this card of the setup it reports.
- ***That a setup happened*** is **the ready action's** setup operation record existing with *done* among its boundaries — **which record the card reads is Y6b's, added 2026-08-25 per Row 487, and a run of any other kind replaces nothing of it.** ***When*** is that boundary's moment on the machine's clock, shown as a moment and **never as a duration**, since *three days ago* would be the cross-clock subtraction D5 forbids; **converting it to the user's zone is permitted and is not that subtraction**, because a zone shift reads the desktop's zone setting and not its clock.
- ***What it consisted of*** is Y4's boundary list, which is what the run actually contained — **so the row's second consequence is answered by construction**: a container's card has no driver line, because R3 and Fact 48 keep that step out of that machine's run and therefore out of its list. The card draws the run rather than a fixed pair of items.
- **The desktop stores none of it.** It is a fresh reading over the connection, so S2's *no second home* holds and D4 is untouched. **With no connection the card is last-known, greyed and stamped**, on the whole-app rule Row 323's answer already fixes, and never blank.


**Row 356 — the action is loud only where Smoozy can name a reason to press it.** *This server.*
> "**It is offered loudly only where Smoozy can name a reason to press it.** The question worth asking of it is: on a server that says ready, why would anyone? There are four answers and three of them are not this screen."

and the first of the four:
> "- **The server was never set up.** That is the whole of this case now, and it is narrower than it was written. **Architecture answered Row 90's setup-state half yes** — a machine can be read for how far a half-finished setup got, by name and with the moment (`Preparation/Architecture/Server Setup.md` Y6) — so *Smoozy cannot tell what has been done to it* is no longer a state Smoozy is ever in about a machine it is talking to. What is left is exact: **no setup record at all, or a machine that failed the identity check.** Both are the same reading, both are loud, and on both, running the action is the main thing on the card. Label: **settled at Architecture** (2026-08-23), carried into Design under [Rule 10](../../Groundwork/Rules.md)."

**Re-quoted 2026-08-25, at the Design close.** The first quote is unchanged and untouched; only the second moved. **The promise is unchanged and the answer is untouched.** The new bullet is this row's own answer written back into Design, citing Y6 by name and labelled *settled at Architecture* — Rule 10 working as written. The reading the bullet needs is still Y6a's *has this server ever been set up*, which is answered below. **Nothing to re-check.**

Kind of promise: **a drafted design sentence**; the paragraph beneath it — *"It is never greyed out"* — is **Creator-stated** (2026-08-11).

Earns its row by the third test: which of four cases the screen is in decides how prominent a control is, and the app has to be told which — including a case the design describes as the spec leaving something open.

Waits on: nothing to get. The outside question it names — whether a machine can be read for its setup state — is Row 90 and is answered.

**Partly answered.** `Preparation/Architecture/Server Setup.md` **Y6** answers the half the design treats as open, **and answers it the other way**: *"**That is Row 90's setup-state half answered yes** — the machine can be read for how far a half-finished setup got, by name, with the moment"*. Cases two, three and four are all answered — the part-way case at S1 and S2, the Telegram case at G2, the GPU case at W4 (Row 308).

**What was not answered is case one's other half, *the server was never set up*** — the same missing reading as Row 355. **And the design's stated ground for the case no longer holds**: the bullet rests on the spec leaving the setup-state question open, and the Architecture stage moved it.

**Answer, 2026-08-21: `Preparation/Architecture/Server Setup.md` [Y6a](../Server%20Setup.md#32-the-recorded-safe-boundaries)'s *has this server ever been set up* clause and the recycled-machine half beside it**, which are the same reading as Row 355's.

- ***Has this server ever been set up*** is **the ready action's setup operation record's presence and nothing else.** No record, no setup. **Narrowed 2026-08-25 by [Y6b](../Server%20Setup.md#32-the-recorded-safe-boundaries), per Row 487, and the narrowing keeps this row's case rather than moving it**: a run of any other kind — a Telegram placement, a Blender install — writes a record of its own kind and no ready-action record, so a machine that has only ever had Telegram put on it still reads *never set up*, which is the loud case unchanged.
- **A recycled machine reads as never set up with no rule of its own**, because §2.3's installation-identity gate disqualifies every remembered value before this is read, and the machine that answers is a different machine holding no record.
- **The row is right that the design's stated ground is spent**, and the case survives it. Y6 answered Row 90's setup-state half **yes**, so *Smoozy cannot tell what has been done to it* is no longer ignorance about a machine Smoozy is talking to. **What is left of case one is exact and is still loud**: no record at all, or a machine that failed the identity gate — and both are the one reading. **The bullet's reasoning is a drafted design sentence and may be reworded when Design resumes**; nothing about what the user gets changes, so it did not go to the Creator.
- **Cases two, three and four were already answered**, as this row records, and none of them moved.


**Row 357 — three lines, not two, and the third one.** *This server.*
> "So the card has **three** lines, not two, and only one of them is a hedge."

and the third:
> "- **Jobs have run and none of them said.** This is the one the design missed. The [spec](../../Specs/Server%20Setup.md) is exact that every result behind the line is Cycles' own report, and whether an EEVEE or Workbench job names anything is not established in any version — so a server that has only ever rendered EEVEE has finished jobs and still nothing to read. The line says that: jobs have run here, and none of them said what did the work."

Kind of promise: **Creator-stated** (2026-08-11), with the third line added 2026-08-13. Also **a settled value** — the mockup draws all three at `#suok`, `#suquiet` and `#sunone`.

Earns its row by the third test, and it is the cleanest example on the screen: telling *nothing has run here yet* from *jobs have run and none of them said* is **a reading across every job that has ever finished on this machine**, and the second line requires the app to represent *a device line was not observed* as a distinct thing from *no job*.

Waits on: nothing to get.

**Partly answered.** `Preparation/Architecture/Server Setup.md` **W4** answers the representation exactly — *"its absence concludes nothing — a job whose line was not observed is a job whose devices are not known, full stop"* — so the three states are distinguishable in the record, and W4's first sentence supplies the first line's content.

**What was not answered is the breadth of the reading.** W4 says *"this module reads one thing from a finished job — the recorded device line"*, **singular**. The third line needs **every** finished job on this machine and the fact that none of them carried one; the first line's *last job* needs them **ordered**. **Nothing in `Preparation/Architecture/Server Setup.md` Section 12 asks `Queue & History` for a per-machine finished-job history**, and `Queue & History`'s §17 live set — which this document does cite — holds *waiting*, *held* or *rendering* and so cannot supply it.

**Answer, 2026-08-21: `Preparation/Architecture/Server Setup.md` [W3a](../Server%20Setup.md#10-what-this-module-says-about-a-machine).** **The reading is over every finished job the machine still holds, on the machine, and never over the desktop's history.**

- **The row looked for a per-machine finished-job history in the wrong place, and so would anybody.** `Queue & History` cannot supply it: a history entry carries the server's **label** as a copy taken at submission (N12, answered the same day for Row 320) and **no server identity at all**, so the desktop cannot answer *on this machine*. **The machine's own finished-job records can**, they are already there, and the identity gate has already established it is the same machine.
- **It asks three questions and returns three answers, never a listing.** Does any finished job record exist; which is the most recent by the machine's own clock; does any carry a recorded device line, and which is the most recent that does. **Nothing is enumerated to the desktop**, and the reading is bounded by what `Rented-server runtime` §13 R1 has retained.
- **The three lines fall straight out** — none, one that says, records but none that says — **and the third is W4's *its absence concludes nothing* stated across the set rather than about one job.**
- **W4 is not widened.** It still reads one thing from a finished job and still concludes nothing from an absence; **what W3a widens is the set of jobs asked**, which is the row's own diagnosis acted on.
- **Ordering crosses no clocks**: *most recent* is by the machine's own clock across records the machine holds, so none of D5's care is needed.


**Row 358 — a long step shown as its steps, with the one under way marked.** *A long step.*
> "**Shown as its steps, with the one under way marked.** A setup can take a while and the user should be able to see where it is."

Kind of promise: **a drafted design sentence**, and **a settled value** for the shape — the mockup draws it at `#burun` with `done` / `now` markers on a five-item list.

Earns its row by the third test: the panel needs the list of steps **this run** will contain, in order, before the run has finished, and it needs to know which of them is **in flight right now** rather than which was last completed.

Waits on: nothing to get.

**Answered already.** `Preparation/Architecture/Server Setup.md` **Y4**: *"The boundary list is composed at run start from the steps the run contains, in R1's order, and recorded in the per-kind field beside the last-reached marker"*, readable over the connection per **Y6**. The step under way follows from **Y7**: the run is *"driven from the desktop, one step at a time, over the connection"*, so the driving side knows which step it issued, and **Y3** makes the boundary mean *"no later step has begun"*.

**One clause worth carrying to Design rather than a shortfall in the answer.** **R1** composes the steps *"per machine at run start from what the machine is found to need"*, and step 1 is the readings — so **the panel cannot show its list until the first step has run, and the list differs by machine.** The mockup's five steps are not R1's nine. **The libraries step has since reached the drawing** — the fifth mockup seam round put it into every panel list, which is Row 488's subject, and what a step with no subject is marked as is answered at **Y4a**.


**Row 359 — the restart panel's three readings.** *A long step.*
> "**A restart is announced, not asked.** The [Server Setup spec](../../Specs/Server%20Setup.md) makes it the one exception to Smoozy never touching a server's power, so the panel says the server is restarting and how long it has been waiting, and says plainly that the render which was running went down with it and starts again by itself."

Kind of promise: **a drafted design sentence**; **a settled value** for the wording of the wait — the mockup draws `waiting for it to restart — 40s` and `The render that was running stopped, and restarts by itself.`

Earns its row by the third test: three readings **at a moment when the machine cannot be asked anything at all** — that it is restarting, how long the wait has run, and that a render was going and will come back.

Waits on: nothing to get. **Fact 51** (a rented machine restarts over the connection Smoozy holds) and **Fact 55** (a running render dies with it) are both answered.

**Partly answered.** `Preparation/Architecture/Server Setup.md` **T4** supplies the *restarting, caused by Smoozy* mark *"set before the command leaves"* with its moment, and **T6** supplies *"**What the server reads as while it waits is *restarting*** — supplied to `Servers` and Design as a state, not a wording"*, so the first two readings are covered and the elapsed time is desktop-side arithmetic that crosses no clocks. **T7** answers the render's death as observed rather than assumed.

**The clause that is not answered as flatly as the design states it is *starts again by itself*.** **Y2 and S6** make the re-run depend on the setup-in-progress state being cleared, and only two things clear it — a run reaching *done*, or a recovery the user asked to abandon the machine-changing work. **A user who closes the panel and never reconnects leaves the state standing and the interrupted render is never re-run**; Section 13 item 16 describes a further case where nothing clears it at all. **The design's sentence is unconditional and the mechanism is not.**

**Answer, 2026-08-21: `Preparation/Architecture/Server Setup.md` [T6a](../Server%20Setup.md#8-the-restart), with the first two readings already answered at T4, T6 and T7 as this row records.**

- **The stopping half is unconditional and is T7's.** The row's finding is upheld on the other half and the boundary is exact rather than a hedge.
- **The sentence is true for as long as the run is being driven, which is for as long as the panel stands.** Y2 holds the interrupted job back on the durable state, and clearing that state at *done* runs reconciliation and re-runs it — **so a run that goes on to finish delivers the sentence exactly.**
- **The case that falsifies it is the user walking away**, and it is not a silence: Y7 drives the run one step at a time, so an undriven run leaves the state standing, **and the machine's own record is what S1 and S2 read on the next connection** — the user meets a part-finished setup naming what was reached and what to do. **The re-run is owed to a finished setup, and the panel is where the setup is finished.**
- **Section 13 item 16's case is worse and is not this one**, and the sentence is false there too. **Its route out is no longer owed**: the item closed on 2026-08-22 at the Architecture seam close, `State, identity and recovery` §5.3 step 6 requiring the supervisor to wait on every worker it starts and `Rented-server runtime` **X9a** stating that requirement from its own side, so the reap that clears the state exists and costs the user nothing.
- **What this document supplies the panel is *the render that was running stopped, and is held to be re-run when the setup finishes*** — the stop as fact, the re-run as consequent. **The wording is Design's**, the drafted sentence states the consequent unconditionally, and that is recorded rather than repaired in a document this stage does not own.


**Row 360 — the cut-off panel keeps the steps, and marks the three kinds.** *Cut off part-way.*
> "**The steps stay on screen, with where it stopped marked**, and what was never started marked as such."

Kind of promise: **a drafted design sentence**; **a settled value** for the shape — the mockup's `#bufail` marks `done`, `bad` and `skip` on the same five items, with `The connection went at 14:11.`

Earns its row by the third test: three distinctions on a machine reached again **on a new connection** — done, stopped here, never started.

Waits on: nothing to get.

**Answered already.** `Preparation/Architecture/Server Setup.md` **S1**: *"The reading distinguishes *a step never started* from *a step that stopped part-way*: the boundary names the last step completed; the operation record and the journal name the step that was in flight and how the run ended, where it ended at all."* **S2** composes exactly what this panel shows from a fresh reading, and **Y4** records the composed list beside the marker, which is what makes *never started* nameable at all. **Y6** makes it readable over the connection.


**Row 361 — the sentence the cut-off panel exists for.** *Cut off part-way.*
> "**Then the sentence the whole panel exists for**, and it is the opposite of what it used to be. The panel was written on the premise that Smoozy could not read how far a setup got, so the step list was a guess and running it again was the only way to be sure. **Architecture answered that half of Row 90 yes** (`Preparation/Architecture/Server Setup.md` Y6): the step list above **is** that reading, taken over the connection, by name and with the moment. So the sentence says where the list came from and that running the setup again is safe — it no longer apologises for not knowing. Label: **settled at Architecture** (2026-08-23), carried into Design under [Rule 10](../../Groundwork/Rules.md)."

**Re-quoted 2026-08-25, at the Design close. The promise moved, and it moved onto the answer.** The half the answer contradicted is withdrawn: the design no longer claims Smoozy cannot read how far a setup got, and says instead that the step list **is** Y6's reading. **The other half is still stated flat** — *running the setup again is safe* — so **this row's second finding below stands unrepaired and stays attached to the new quote: safe, yes; always available, no.** The new wording does not disagree with it; it simply does not carry it.

Kind of promise: **a drafted design sentence**, and **a settled value** — the mockup draws it at `#bufail` as `Smoozy can read what software is installed, not how far a setup got. Running it again is the way to be sure.`

**Earns its row by the first test as well as the third:** the document says outright what Smoozy can and cannot read, and builds a whole panel on it. That is a promise about a capability, not an appearance call, and both halves are this stage's to confirm.

Waits on: nothing to get. The two facts under it — Row 90's setup-state reading and Row 114's safe re-run — are both answered.

**Answered already, and the answer contradicts both halves. This is the most consequential row in the sweep of this document.**

- **On the first half**, `Preparation/Architecture/Server Setup.md` **Y6**: *"**That is Row 90's setup-state half answered yes** — the machine can be read for how far a half-finished setup got, by name, with the moment — on Fact 49's observation."* Fact 49 is the observation behind it: cut at a recorded safe boundary, the machine named the third of five boundaries with the moment it was reached; cut hard, it named the second. **The panel's own step list, which Row 360 says is answered by S1 and S2, is that reading — so the panel already draws the thing its closing sentence says Smoozy cannot do.**
- **On the second half**, Row 114's promise is kept — Section 4 entire, and Section 14 records that `Method`'s first escalation was not reached. **But *run it again, which is safe* is not the whole outcome**: **S5** refuses a recovery while a recorded group still validates, so a run again can be a wait rather than an act. **Safe, yes; always available, no.** **The worst shape of that wait is no longer unbounded** — Section 13 item 16, which recorded a dead setup group reading as live on a machine whose supervisor is up with *"nothing defined here clears the state"*, **closed on 2026-08-22**: `Rented-server runtime` **X9a** now requires the supervisor to wait on the workers it starts, and S5 names that reap as the route out.

**The consequence for Design is not this stage's to settle, and the runner carries it:** this sentence, Row 356's loud-action bullet, and the document's pending marker about the card's state line all rest on a question that has since been answered the other way.


**Row 362 — the two tags the version cannot say.** *Blender.*
> "**Two things the version cannot say get a small tag instead**: a build the user brought up themselves, and one below 4.5. The second is not a refusal — it can be picked for a job, and is never picked automatically."

Kind of promise: **a drafted design sentence**, and **a settled value** — the mockup draws both tags on one row: `4.2.9` with `your build` and `below 4.5`.

Earns its row by the third test in the sharpest form: the table is read from the server every time, **a tag on a row is a claim about a build, and one of these two claims is not in anything the machine can be asked.**

Waits on: nothing to get.

**Partly answered, and the part that is not is answered nowhere at all.**

- ***Below 4.5* is answered.** `Preparation/Architecture/Server Setup.md` **B1** takes the version from the `Blender ` line the build itself prints, and **B12** supplies the fresh search result to `Render Settings` as *"(path, answered version, and whether the answer was long-term-support or unstable)"*. The floor comparison is arithmetic on that version.
- ***Your build* is answered nowhere.** **B1** is explicit that nothing else is read: *"No folder name, path or channel is parsed for either — what a machine holds is read by asking the build"*. **B8** closes the other route: *"The folder's name carries an identity of this document's issue plus the answered version for a human reader; **nothing anywhere parses it back**."* B12 and Row 104 forbid remembering it across connections, and `Servers` D17's enumeration of what a saved server durably holds has nothing of the kind. **Whichever of the two meanings the tag has — a build Smoozy sent up from the user's own computer under B7, or a build the user put there without Smoozy — no architecture document arranged for the app to tell it from any other build in the table.** It was a tag on a screen that nobody arranged for the screen to know.

**Answer, 2026-08-21: `Preparation/Architecture/Server Setup.md` [B8a](../Server%20Setup.md#53-where-a-build-goes)**, with *below 4.5* answered as this row records.

- **The row's finding stands whole: the origin is not a property of the build and no reading of a string can recover it.** B1 asks the build and the build answers a version and two marks. B8 forbids parsing the folder name back. B12 and Row 104 forbid remembering. **So the reading comes from a record of the act that placed it**, and one already exists.
- **A Blender install is an operation and its subject is the published build.** `Rented-server runtime` §13 R1 already retains an operation record *"the same moment as their subject's machine-side record"*, **so naming the subject is the whole of the clause** — the record stands while the build stands and ends when it is removed. Nothing new is stored and no new record kind exists.
- **The join is by executable path**, B13's own identifier and the only one this document trusts; nothing is parsed and nothing is remembered on the desktop.
- **There are three answers and the design draws one tag.** Placed by Smoozy from the published listing — no tag. Placed by Smoozy from a build the user pointed at — *your build*. **No install record names this path — a build Smoozy did not place at all**, which is the apt or snap build Fact 45 already made this document plan for. **The third is real and common and the design has no tag for it**; what it is called, or whether it is called anything, is Design's.
- **One clause of the quote is matched by nothing above, and the row carries it rather than passing it over.** *"It can be picked for a job, and is never picked automatically"* is a promise about picking rather than about either tag, and **nothing here says where it is answered.** It is likely `Render Settings`' — **B2**'s list is narrowed to the whole-app floor of 4.5, and a build carrying this tag sits below that floor — but this row does not rest on it, and `Method` wants the match made clause by clause rather than assumed.


**Row 363 — the install list says which are already on this server.** *Installing.*
> "**The list is what Blender publishes, and it holds no daily builds.** The marker asked how daily builds are listed. They are not: the [spec](../../Specs/Server%20Setup.md) and the [architecture](../Server%20Setup.md) B6a both settle that the list comes from the shop-front and that daily builds sit somewhere else, on a listing Smoozy reads for the record and does not offer. **The mockup drew a *Daily build* entry and it is gone** — it had no source, which is exactly what Rule 10 is for."

and
> "**Each entry says whether that version is on this server**, which is a version comparison and nothing more: the [architecture](../Server%20Setup.md) B6a is firm that a published entry is a channel and a machine's build is a path, so the mark can say a version is present and can never say *this build*. Nothing acts on it — installing goes ahead whatever it says."

**Re-quoted 2026-08-25, at the Design close.** The single sentence this row was filed on is gone; it has become the `## Installing` section and its three clauses are now separate statements. **Two of them are the ones B6a answers and are the two quoted above.** **The third clause — *pointing at a build on the user's own computer offered beneath as the other way in* — is now the subsection heading `### Pointing at a build on your own computer` and its four paragraphs, so it cannot ride the re-quote and is left out** rather than forced; B7 answers it whole and this row's answer already records that.

**The promise moved, and both moves are the answer being carried in.** B6a said the mark claims a version and never a build; the second quote says that in the design's own words. B6 and the Creator's ruling of 2026-08-12 said the list holds no daily builds; the first quote says so. **Nothing in the answer needs re-checking, and one parked item is now done: the mockup's `Daily build` entry has been removed** — which was the one thing this row's answer parked for Design's resumption. The *Kind of promise* line below records the mockup as it stood when the row was filed.

Kind of promise: **a drafted design sentence**, and **a settled value** for the dialog — the mockup's `#buadd` draws four entries with per-entry states `not installed`, `on this server` and `4.5.9 installed`, and a `Daily build` entry.

Earns its row by the third test: **two lists have to be joined** — what is published, and what the machine holds — **and the join is not obvious**, because the two are read from different places by different means.

Waits on: nothing to get. **Facts 52 and 53** (what blender.org and the shop-front publish that a machine can read) are both posed and answered by reading, 2026-08-19.

**Partly answered.**

- The list exists and is narrowed. `Preparation/Architecture/Server Setup.md` **B2**: *"The list Smoozy offers holds **the shop-front's channels at or above the whole-app floor of 4.5, with their risk marks**"*. **B7** answers the second half of the sentence, the build the user points at.
- **Two clauses are not answered as the design draws them.** **B6** establishes that blender.org's own builder publishes a machine-readable daily listing, then says *"Nothing in this document builds on it — the ruling fixes the list's source"*, **so the mockup's `Daily build` entry has no source in Architecture**; the design's own pending marker about listing daily builds is settled by a narrowing rather than by a mechanism. And **nothing states how a published entry is matched to an installed build.** B2's entries carry a version; B10's search answers a version per path; but **B13 warns in the same document that *"A version number does not identify a build"***, so a version match can report *on this server* about a different build of the same version — and **the mockup's `4.5.9 installed` beside a `4.5.10` entry is a match by release line rather than by version, which nothing supplied.**

**Answer, 2026-08-21: `Preparation/Architecture/Server Setup.md` [B6a](../Server%20Setup.md#51-the-list-and-the-fetch)**, with the list itself and the point-at-a-build half answered at B2 and B7 as this row records.

- **The row's diagnosis is exact: a published entry is a *channel* and a machine's build is a *path with an answered version*, so the join can only ever be a version comparison.** B6a says so and says what follows — **the mark claims a version and never a build**, which is B13's *"A version number does not identify a build"* honoured rather than worked around.
- **The mockup's *4.5.9 installed* beside a 4.5.10 entry is exactly what B6a supplies**: the entry is marked with the version that **is** present where the machine holds a build of that release line at a different version, and marked as present where a build answers the entry's current version.
- **Nothing acts on the mark, and that is what makes the imprecision safe.** Installing proceeds whatever it says, B8 never publishes over an existing folder, and the removal warning is composed from recorded **paths** rather than versions (B14). It is a reading for the user's eye and no decision reads it.
- **The daily entry: the row is right that it has no source, and the reason is a ruling rather than an omission.** B6 records that the builder's daily listing exists and that nothing here builds on it, **because the Creator's ruling of 2026-08-12 fixes the list's source as the shop-front.** So the architecture's list holds no daily build; the mockup draws one. **It is recorded rather than repaired**, because the mockup is a Design document and the ruling under it is the Creator's own — and it is parked for Design's resumption.


**Row 364 — the removal warning tells each job the truth about itself.** *Removing a Blender.*
> "**Where a live job needs it, both jobs are named and each is told the truth about itself** — a render already going finishes untouched, a waiting job will have nothing to render with. That split is the [Server Setup spec](../../Specs/Server%20Setup.md)'s and it matters, because the two outcomes are not alike."

Kind of promise: **a drafted design sentence**, and **a settled value** — the mockup's `#burmmod` draws `rendering now — it will finish anyway, untouched` beside `waiting — it will have nothing to render with`.

Earns its row by the third test: naming the jobs needs the live set and the build each was submitted with; **telling each the truth needs an established outcome for a render in flight.**

Waits on: nothing to get for the naming. **For the *finishes untouched* half, `Preparation/Architecture/Server Setup.md` Section 13 item 17 has since split what is owed**: the package-manager half **is answered by the standard and needs no machine** — POSIX `unlink()` postpones the removal of a file's contents while a process still holds it, which is the same rule behind B14's folder-deletion observation — and **what is still not established is the snap route alone**, where the build is a read-only squashfs mount that removal must unmount. **It is answerable free and wants no rental**: one Ubuntu machine with snapd. It is numbered **Fact 110** in `Preparation/Architecture/Facts.md`, whose entry still frames the question at its old width.

**Partly answered.** **B13 and B14** answer the naming completely: the submission payload carries the picked build's path beside its version, the warning is composed from `Queue & History` §17's live set, and one predicate covers both removal routes.

**The clause that is not answered is *a render already going finishes untouched*, said flat.** **B14**: *"**On the manager route that is not established and is not claimed**: what a removal performed by the machine's own software manager does to a render already running from that build was never observed, and this document does not carry one route's observation onto the other in the sentence the user reads before deciding."* **So on one of the two routes the design's sentence is a claim Architecture refuses to make**, and B14 requires the warning to say the outcome is **not known** instead — a third thing to word, on a panel the design describes as having two.

**Answered, 2026-08-21, with the residual named inside the answer rather than the row left waiting: `Preparation/Architecture/Server Setup.md` B13 and B14, which this row already reads correctly.** **Nothing new was built for it, and that is the finding.**

- **The naming half is answered completely** — the submission payload carries the picked build's path beside its version, the warning is composed from `Queue & History` §17's live set, and one predicate covers both removal routes.
- **The *finishes untouched* half is answered per route and refuses to be flat**, which is B14's own decision and is the right one: on the folder-deletion route the outcome is established, and on the manager route it was never observed and **this document declines to carry one route's observation onto the other in a sentence the user reads before deciding.**
- **So the architecture supplies three outcomes and the design draws two.** The third is *not known*, on the manager route, and **what it is worded as is Design's.** The row's *a third thing to word* is the answer rather than a gap in it.
- **One correction to this row's own text.** It says the owed fact *"is not numbered in `Preparation/Architecture/Facts.md`"*. **It is: Fact 110**, numbered on 2026-08-19 out of `Server Setup`'s own Round 1, riding on any later batch that can place a Blender through the machine's own software manager. The row was written from Section 13 item 17 and did not check the register. **That is the thirty-first instance of the fault being the runner's own file.**
- **What a positive answer would change is one word on one panel**, and what a negative would change is nothing at all — B14's refusal is already the safe form. **So the row closes rather than waiting.**


**Row 365 — admin access, asked at the refusal, with the three lifetimes.** *Admin access.*
> "**Asked only when a step is actually refused, and never before.** The [Whole app spec](../../Specs/Whole%20app.md) owns the rule and its three choices; this screen is one of the two places it is reached."

and
> "**The three choices are radios, not boxes** — they are one decision with three answers, not three switches. They are: not remembered at all, remembered while the connection lasts, and remembered until cleared, with where it is cleared named."

Kind of promise: **a drafted design sentence.**

Earns its row by the third test on the last clause: *never before* requires the app to tell a **permission refusal** from any other failure at the moment it happens, and *with where it is cleared named* requires a real place elsewhere in the app for the screen to name.

Waits on: nothing to get. **Fact 27** (what each machine kind permits, at both weights) is answered.

**Answered already, with one consequence the design should carry.** `Preparation/Architecture/Server Setup.md` **E2** answers the timing in the same words — *"Elevation is offered only at the moment a step is refused for lack of permission, never before"* — and **E4** supplies the recognition, the validate-first order that separates an authentication result from the operation's own. The three lifetimes are `Preparation/Architecture/Security and licence.md` Section 2, items 3, 3b and 3c, and item 3's erasers name where it is cleared: *"server edited and cleared; username changed; server removed"*.

**The consequence, and it has narrowed since this row was answered.** E4's validate-first order separates the authentication step's result from the operation's; what it does not separate by exit number is a wrong password from an elevation the login is refused, all three refusals ending at 1. **The words do differ, and one of them may now be read**: `Security and licence` §13 rule 5 carries a **Creator-stated exception (2026-08-25)** reaching the elevation tool alone and, within it, the wrong-password ending alone as an **observed constant**, and §4 R8 states that the erase fires on it — so §2 item 3's erasers now number **four**, *"server edited and cleared; username changed; server removed; the password ceasing to work"*. **Where the refusal is anything else** — the not-an-administrator message names the host and is never matched — the invariant still withholds the erase and the password is kept. ***Where it is cleared* is therefore still load-bearing on this screen rather than a courtesy, and the erasers list still has no entry for the Server Setup screen itself.**

---

## The document's markers, ruled

None of the ten `[PENDING: …]` markers names Architecture, so none earns a row. Six are already answered by a written architecture document:

- *how daily builds are listed* — **B2 and B6**: they are not listed, because the Creator's ruling of 2026-08-12 fixes the list's source at the shop-front. The mockup draws one anyway (Row 363).
- *how a setup step that had nothing to do reads — a locked container whose drivers are already in place* — **W1**'s *Nothing to prepare* statement and its precedence rule against *Could not be made ready*; **R3** and Fact 48 make that container the container kind, which is observed rather than assumed rather than an edge case.
- *what the card's state line and its `when` read as where Smoozy cannot tell what a machine has already had done to it, which the spec treats as the ordinary case* — **the premise is gone.** Y6 answers Row 90's setup-state half yes. See Row 361.
- *what a second setup step meets while one is running* — **Y9**: refused at the admission point with the open run reported. But Section 13 item 15 says that for **additive** runs (an add-on install, a removal) nothing delivers it, so the answer holds for one class of step and not the other.
- *how a Blender the user points at on their own computer is chosen and sent* — **B7**'s three clauses.
- *the warning before a step that could disturb work on a busy server* — **W3**'s closed table, which puts a warning on the ready action and on a Blender install that must install the unpack tool, and forbids one on an ordinary Blender install, an add-on install or a Telegram placement.

---

## Filed 2026-08-22 by the both-ways check over the Design documents

**The first both-ways ruling any Design document has ever had.** A fresh agent read this document against the rows above and ruled in both directions — what was missed and what does not belong. The rows below are what it found missing. Each carries the checker's own reasons and the constraints it named, because the runner filed them from the verdict and the verdict is not read again. The verdicts are in `Audits/Architecture/Both-ways check/`, consolidated at `Audits/Architecture/Both-ways check/Consolidated — the runner's worklist.md`.


**Row 439 — the stop offered throughout a setup, and what a stop does to a run.** *Running a setup.*
> "**Stopping is offered throughout**, and a restart already begun cannot be called back — stopping then only stops Smoozy waiting."

Kind of promise: **a drafted design sentence**, resting on a decision and a dependency. On the same footing as Row 359 and adjacent to it in the document.

Earns its row by the third test. A setup run can be ended by the user at any point in it. The run is driven from the desktop one step at a time under `Preparation/Architecture/Server Setup.md` **Y7**, against a machine-level setup-in-progress state that **Y2** and **S6** leave standing until a run reaches *done* or a recovery clears it. **Row 359's own answer identifies an undriven run leaving that state standing as the case that falsifies the panel's other promise, and names it *the user walking away*. A stop offered on the panel is the same state reached by a control rather than by neglect** — and what the stop does to the state, to the step in flight, and to the interrupted render is stated nowhere.

**The second clause is a distinct promise** about a restart already issued: that it cannot be recalled, and that a stop during it stops only the desktop's waiting. It bears directly on **T4**'s restart-marked period and its two clearing events.

**Why no row reached it.** Nothing in Rows 355 to 365 quotes this sentence. Row 359 quotes only the announcement sentence beside it, and its answer covers the render's death and the re-run.

**Answer, 2026-08-23: `Preparation/Architecture/Server Setup.md` [Y8a](../Server%20Setup.md#33-the-worker-the-driving-and-the-stop) and [Y8b](../Server%20Setup.md#33-the-worker-the-driving-and-the-stop), beside the **Y8** the row already names.** Clause by clause:

- ***Stopping is offered throughout*** — **Y8a**: one control for the whole of a run's life, with **two outcomes decided by whether the machine can take the durable write** and never by the user choosing. Where the machine answers — a machine-side step, an elevated step, or the worker waiting at a boundary — the outcome is **Y8** unchanged. Where the machine is down, which is **T4**'s restart wait and nothing else, the outcome is the second clause below.
- ***A restart already begun cannot be called back — stopping then only stops Smoozy waiting*** — **Y8a**'s second bullet: the durable write cannot be made and no request leaves, **so the clause is a property of the machine's unreachability rather than a rule written to enforce it**. **And the stop is not held for delivery when the machine answers** — no request leaves, so there is no pending-command record to resolve (`Server protocol and transfers` C3, C20) — which is what leaves **T6** standing exactly as written: the mark stands, the state stands, the reconnect policy runs on, and **T5**'s recovery run picks the setup up on return. The earliest end available to the user after a restart is that recovery run's next boundary.
- **What the stop does to the step in flight** was already **Y8**'s and is confirmed rather than re-answered: honoured at a boundary, never by a signal, and an elevated step's channel closed with the run ending at the last boundary passed.
- **What it does to the state and to the interrupted render** is **Y8b**, which is what the row found stated nowhere. The state stands, because **S6**'s clearing set is closed at two and a stop is neither. The interrupted job stays held back on that standing state (**Y2**, P5), **so the row's own reading is upheld: a stopped machine-changing run reaches T6a's *user walking away* place by a control rather than by neglect**, and the re-run is owed when a later run reaches *done*. What the user meets next connection is **S1** and **S2**'s part-finished reading, with **S4**'s recovery run as the route on. **An additive run sets no state (Y1), so stopping one holds nothing back at all.**
- **Checked against `Method`'s third escalation and declined**, at `Server Setup` Section 14: the spec grants the stop and says in the same breath that it leaves what a lost connection leaves, and the job is held rather than lost.

**Row 440 — the folder column's link into `File Browser`.** *The folder column stays, and Where it sits.*
> "**It stays, and the [architecture](../Server%20Setup.md) B12a is what settles it.** The marker asked whether it survives, no spec granting it. B12a grants it in this stage's own terms: the folder is the directory of the path the search returned for that build, derived when the table is drawn and never remembered, and the path is the only machine value this screen composes."

and
> "**The link is on every row, including the ones it cannot help with.** A build the machine's own manager placed may sit on a mount nothing can be written to; the link is offered anyway, because withholding it would be Smoozy deciding which folders the user may look at. Where the folder has gone or cannot be read, [File Browser](../../Design/File%20Browser.md) says so in its own words — this screen hands over a path and claims nothing about it."

and, under *Where it sits*:
> "Its folders link into [File Browser](../../Design/File%20Browser.md)."

**Re-quoted 2026-08-25, at the Design close. Only the first quote moved; the second is unchanged word for word and is untouched.** The one line the row was filed on is now the subsection `### The folder column stays`, and **two of its three paragraphs carry the promise** — both are given, because B12a answers both and neither alone holds the whole of it.

**The promise is unchanged, and one thing this row carried is spent.** The document held `[PENDING: whether the folder column and its link into File Browser stay, which no spec grants]`; **Design resumed, kept the column on B12a's grant, and closed the marker.** So the row's *may be withdrawn when Design resumes* contingency is spent, and **the answer does not go with it** — B12a stands. The three paragraphs are B12a's bullets read back, the offered-on-every-row rule almost word for word.

Kind of promise: **a drafted design sentence**, resting on a dependency.

Earns its row by the third test. The Blender table's middle column **opens another module at a named folder on a named machine**, which requires an entry point into `File Browser` taking a machine and a path, and requires **the folder to be known per installed build** rather than per version.

**The precedent, in the calibration file itself.** Row 294 was filed by the previous both-ways check for the sentence *"Smoozy says so and opens the file browser on that machine so the user can point at the file"*, and its answer is *"the file browser opens on that machine through one entry point invoked with the machine and nothing decided inside it."* **This promise is the same kind against the same module and had no row.**

**One thing this row carries, stated by the checker as such.** The document also holds `[PENDING: whether the folder column and its link into File Browser stay, which no spec grants]`. The marker names no stage, so it earns nothing on the second test, and the sentence is stated flatly and earns the row on the third. **The row records that the promise may be withdrawn when Design resumes; that is not a reason to leave it unfiled.**

**Answer, 2026-08-23: `Preparation/Architecture/Server Setup.md` [B12a](../Server%20Setup.md#54-the-search).** Clause by clause:

- ***The folder is a link into File Browser at that folder*** — **B12a**'s **first bullet**. **The value is per build and comes from the reading that drew the row**: **B10**'s search returns a path and an answered version per candidate, **B13** makes the executable's path the build's identifier, and the folder is the directory containing that path, derived when the table is drawn and never remembered (**B12**, Row 104). **Never per version** — two builds answering one version have two paths and two folders, which is the row's own second requirement.
- ***Its folders link into File Browser*** — the same clause. **B12a** states outright that the two sentences are one promise, because the per-build folder is the only machine path this screen composes.
- **The row's first requirement — an entry point taking a machine and a path — is narrower than the row read it, and B12a says why.** `File Browser` is the same server's other tab (its Design document, *Where it sits*), so **the server row is the one whose screen this is and no machine is chosen; what crosses is the path alone**, as the browser's opening position (`File Browser` N1's run-held state). **Nothing is picked and nothing comes back**, so this is **not** the entry point `File Browser` §12 supplies to `Rendering` — which is where Row 294's precedent differs rather than repeats.
- **Landing on a path directly is already `File Browser`'s ordinary case and is cited rather than re-solved**: its **P3** covers a browser that lands on a path, names refusal from absence, and walks a refusal component by component. **So this document composes the path and claims nothing about it** — the reading is period-held, and a folder gone or unreadable is answered by that module.
- **B12a carries one case the design has no drawing for**: a build Smoozy did not place (**B8a**'s third case), where the folder is the machine's own — a snap's found path resolves to the snap launcher (**B10**) and the snap mount is read-only (**B15**). **The link is offered on every row all the same**, because withholding it would be this document deciding which folders the user may look at.
- **Checked against `Method`'s third escalation and declined**, at `Server Setup` Section 14. **The `[PENDING]` the row carries is untouched**: nothing here decides whether the column stays, and if Design withdraws it, B12a goes with it. *(2026-08-25: the marker is now closed and the column kept, so this last conditional is spent — see the re-quote note above.)*

---

## Filed 2026-08-25 by the Design close re-sweep

**The Design stage finished on 2026-08-25 and `Preparation/Design/Server Setup.md` was rewritten.** A fresh agent re-swept the whole document against the rows above. **All four rows below sit in sections that did not exist at the first sweep**, so no existing row quotes any of them and no quote broke on any of them. Each carries the sweep's own words for what it waits on and why it earns its row. **None of the four is answered**, and none waits on a fact — every one of them is a decision or a dependency inside the plan.


**Row 487 — the dedicated Telegram action, against the one setup record the card is composed from.** *Adding Telegram to a server already set up.*
> "**It appears only where it is owed** — Telegram is on in [Settings](../../Design/Settings.md) and this server has not got it. On a server that has it, the card's list already says so and there is nothing to offer."

and
> "**On a machine that has never been prepared at all, it still does only its own work.** The [architecture](../Server%20Setup.md) G3 has it place what it needs to run and nothing else — no driver, no restart, no ready action — so the card can go on saying the server is not set up while Telegram reporting is installed on it. That reads oddly until you say it plainly, so the line does: Telegram reporting is installed, and the rest of the setup has still not been run."

Kind of promise: **a drafted design sentence**, and **a settled value** for the two card states the fifth mockup seam round added.

Earns its row by the third test, and it is the sharpest thing in the re-sweep. **Both sentences read the *This server* card, and Row 355's answer fixed what that card is composed from.** `Preparation/Architecture/Server Setup.md` **Y6a** makes it the setup operation record: *that a setup happened* is the record existing with *done* among its boundaries, *what it consisted of* is **Y4**'s boundary list, and *has this server ever been set up* is *"the record's presence and nothing else. No record, no setup."* Y6a's retention term is equally exact — *"a setup operation record is retained until the next setup run on that machine replaces it at run start"*, ended by nothing else, **one per machine**, so the machine holds the last setup and never a history of them.

**And Y1 makes the dedicated Telegram entry point a setup run.** Its first sentence is *"Every setup act on a machine is a setup run: one setup-kind operation, one operation record written before its worker starts (W3), one recorded execution group"*, and its second class names *"Telegram material placement"* outright as an additive run. So the entry point writes a setup operation record like any other setup run, and **two consequences follow from Y6a as written, neither of them stated anywhere**:

- **On a machine never prepared, the run leaves a setup operation record, so the machine reads as set up** — the exact opposite of what the design's second sentence requires the card to go on saying.
- **On a machine prepared earlier, the run replaces the ready action's record at run start**, one per machine, so the card loses the ready run's boundary list and its *done* moment and shows the Telegram run's instead — while the design's first sentence has the card's list say Telegram is present *beside* what the setup did.

Waits on: nothing to get. No outside fact is involved. **What is needed is a decision:** whether the record the card reads is the last setup run of any class or the last *ready action* run, and whether an additive run's record is a separate subject under `Rented-server runtime` §13 **R1**'s retention table rather than the one Y6a made one-per-machine.

*(Superseded 2026-08-25 — the paragraph below is what the sweep found, and the answer follows it.)*

**Not answered anywhere the sweep could find.** **G2** and **G3** answer what the entry point *does* — material placed, desired state set, no other step re-run, no restart, the package placed first on a bare machine — and Row 356's answer records the Telegram case as answered at G2. **None of that is what this promise needs**: G2 and G3 say nothing about the operation record they write or about what the card reads afterwards, and Y6a was written on 2026-08-21 for Rows 355 and 356, before this section of the design existed; it contemplates the ready action's run and no other. `Preparation/Architecture/Server Setup.md` Section 12's supply list names no consumer for this and Section 13 carries nothing on it.

**This is silence rather than disagreement, and the distinction decides which rule applies.** The architecture's entry point clauses answer what the action does and say nothing about either consequence, so **Rule 10 does not touch it and Rule 7 puts it here**: the design says something the architecture has never been asked. **It is a genuine row and not a Design fault.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Server Setup.md` §3.2 **Y6b**, all five bullets, with the retention half at `Preparation/Architecture/Rented-server runtime.md` §13 **R1**'s setup-operation-record row, narrowed in place from *the next setup run* to *the next setup run of the same kind*. **Y6a**'s retention bullet carries an in-clause note pointing at the narrowing.

**The decision taken:** the card's record is the **ready action's**, not the last run of any class, and an additive run's record is a separate per-kind subject under R1 — one record per machine per kind of setup act, the kinds named at Y6b, with **B8a**'s subject-tied install records excepted where the subject came to exist. Both of the design's sentences then hold by construction, and **G2** and **G3** are untouched.

**And the card's Telegram line is not a line of the run at all.** Y6b's fourth bullet composes it from **G1**'s durable enabled-or-disabled desired state on the machine, read fresh over the connection and never from **Y4**'s boundary list — one source whichever entry point placed the material, since a ready run's step 8 and the dedicated entry point both set the same desired state. A ready run's *telegram* boundary stays in the list as what that run did; the line about the role **now** is the role's own reading. No file's presence is read for it, the desktop stores none of it (S2, D4), and the line's wording and where it sits in the list are Design's.

**Nothing routed. Nothing left unresolved.**



**Row 488 — the step list's per-step marks, and the library step shown where it never ran.** *When a step had nothing to do.*
> "**The state line says it, and the step list is what makes it believable.** Each step reads *nothing to do* rather than *done*, because *done* on a step that installed nothing claims work that never happened. The list is the same list; only the marks differ."

and
> "**One of the marks is not *nothing to do* and must not be.** Where the machine held no Blender and the run fetched none, the [architecture](../Server%20Setup.md) R4 and W1 are firm that the library check had no subject and *ready* claims nothing about it. So that step reads **not checked — there is no Blender here to check against**, and the ending says the same. A step that never ran is not a step that found nothing to do, and collapsing the two would have Smoozy claiming a check it never made."

Kind of promise: **a drafted design sentence**, and **a settled value** — the fifth mockup seam round put the library step into every panel list, worded one way, and gave the *nothing to prepare* run its own drawn state.

Earns its row by the third test. **W1's five endings are run-level and are answered; the marks these two paragraphs ask for are per step, and nothing records them.** `Preparation/Architecture/Server Setup.md` **Y3** defines a boundary as *"every step up to and including the named one is done and verified; no later step has begun"*, and **Y4** records the composed boundary list beside the last-reached marker. Between them they distinguish **done, in flight and never started** — which is exactly the three Row 360's *Cut off part-way* promise needs, and exactly why that row is answered. **They cannot distinguish a step that did work from a step that found nothing to do**, because a boundary is written when the step completed and its verification read passed, whatever the step found.

**And the second paragraph asks for a step to be in the list that R1 may never compose into it.** **R1** composes the sequence *"per machine at run start from what the machine is found to need"*, each step *"present only where the machine needs it"*, and of the library step says: *"Where this run fetches no Blender and the search finds none, the step has no subject and does not run"* — Y4's table repeating the rule as *"each present only where its step is"*. So on the machine this paragraph is written about, **there may be no libraries boundary in the list to mark at all.** R4 and W1 do carry the claim the design cites, but they carry it as a clause of the **ending** rather than as a mark on a step: R4 says *ready* claims nothing about the library half, and W1's *Nothing to prepare* says the same in its own words. **The design has moved that statement onto the list.**

Waits on: nothing to get. **A decision:** whether the setup operation record's per-kind field gains a per-step outcome beside Y4's boundary list, and whether a step with no subject is composed into the list and marked rather than left out of it.

*(Superseded 2026-08-25 — the paragraph below is what the sweep found, and the answer follows it.)*

**Not answered anywhere the sweep could find.** Y3, Y4, Y5 and Y6 are the whole of what the record holds and what is readable over the connection; **S1** and **S2** compose the part-finished panel from exactly those three things — *"the boundary list, the last one reached with its moment, and the run's ending where one was recorded"*. W1 is the endings. **Nothing in Section 3.2 or Section 4.1 carries a per-step outcome.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Server Setup.md` §3.2 **Y4a**, four bullets: the recorded per-step outcome; its readability and retention; the step with no subject; and **W1** left untouched.

**The distinction the row turned on is tellable, which is why this is neither an unresolved close nor a Rule 10 send-back.** `Preparation/Architecture/Server Setup.md` **R1** items 3 to 6 — **the document was mis-named here as `Rented-server runtime`, corrected 2026-09-02; the label was always right** — make every step that can have nothing to do **check first**, so the run knows the answer at the moment the boundary is written. **No mark is invented for a check that never ran**: the *not checked* line is composed from **Y4**'s composed list — a positive record that the run contained no libraries step — together with **R4**'s clause, and never from a mark.

**Nothing routed. Nothing left unresolved.**



**Row 489 — the below-4.5 warning's way out, between the version answer and the publication.** *The below-4.5 warning.*
> "**It fires after the build is up and before it lands**, which is where the [architecture](../Server%20Setup.md) B7 and B8 put the only moment Smoozy knows the version: the build is unpacked in staging, asked what it is, and renamed into place last. So the warning is not something the page could have said when the user pointed at the file — the file's name is not its version — and it is not an apology after the fact either."

and, for the clause that is not answered:
> "**It says the three things the [spec](../../Specs/Server%20Setup.md) names and then offers both ways.** The version is unsupported, nothing in the rest of the app is written for it, and Smoozy cannot help if a job on it goes wrong — with *Install anyway* and a way out. **The spec's own words are *the user's own affair*, which is a spec's way of speaking rather than a screen's**, so the line says the same thing in the plain form Rule 9 asks for. Offering the way out is not refusing: the build is still in staging, so a user who has just learnt what they fetched can still decline it, and a warning nobody can act on is a warning worth nothing."

Kind of promise: **a drafted design sentence**, closing a `[PENDING]` marker the first sweep ruled earned nothing because it named no stage. **It names one now by consequence rather than by wording.**

Earns its row by the third test, **on the way out rather than on the warning. The timing is answered.** **B7** clause 3 puts it exactly where the design does — *"the build is then asked its version under B1, which is owed anyway and is what catches a truncated or wrongly-unpacked archive, and the below-floor warning fires on the answer where it applies (the spec's own rule)"* — and **R1** item 7 has *"only on that answer is it renamed to its final folder"*. **What is not answered is that the run holds there for a human, and what a decline leaves behind.** Three things are owed and none is stated: that a setup run can be suspended between two of its own steps awaiting a user answer that is **not** an elevation; that a declined install **discards** the staged, unpacked build rather than publishing it; and **which of W1's endings a declined install has**, given that no step failed and no permission was refused.

Waits on: nothing to get. **A decision, and a dependency on the run's own driving.**

*(Superseded 2026-08-25 — the paragraph below is what the sweep found, and the answer follows it.)*

**Not answered anywhere the sweep could find.** **Y7** drives a run *"from the desktop, one step at a time, over the connection"*, so a hold between steps is mechanically available, and **E2**'s elevation flow is the one place a run already stops for a person — **but E2 is written for a step refused for lack of permission and for nothing else**, and R1 item 7's re-runnability argument turns on the rename **consuming** the staged material, which a decline leaves unconsumed. `Preparation/Architecture/Server Setup.md` Section 12's line to `Render Settings` names *"the below-floor case surfacing from the version answer at install"* and stops there. **Nothing in Section 5.2, Section 5.3 or Section 10 says what happens next if the user says no.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Server Setup.md` §5.2 **B7a**, seven bullets: route-only; the hold; the walk-away; the decline; the recorded outcome; that it is not a stop; and the cost check.

**The hold was already reachable and no new mechanism was built.** **Y7**'s *a refresh is not a step command* means the run holds between the ask and the rename; **Y5** keeps the ask boundary-free so repetition is free; §7.1 keeps the build off its final name; and **S4**'s resume re-reaches the question because **B7** clause 3 owes the ask before any rename. **What is genuinely new** is the decline as a step command revising the composed list — the publication struck, the discard recorded with its reason under **S24**'s abandonment shape — the run completing to *done* under **S6**'s first clear with the clearing set staying closed at two, and the recorded outcome *declined at the version answer*, which is none of W1's five.

**The third stop was reached and declined**, recorded at §14 as the twelfth such check: the discard costs the staged copy alone and the user's own file is untouched.

**Nothing routed. Nothing left unresolved.**



**Row 490 — the machine's own words for the step that failed, read on a later connection.** *The step that failed.*
> "**It takes the whole-app failure shape, on the step itself in the list.** The red dot, one plain sentence saying what that step did not do, then the machine's own words behind the control that names them — *What the server said*, which is the one control the whole app uses for raw text. This screen installs software and restarts machines, so it is where a user most needs the machine's own words: *step 4 stopped* is nothing to act on, and what the machine printed is."

Kind of promise: **a drafted design sentence**, and **a settled value** — the fifth mockup seam round drew the control on the failed step.

Earns its row by the third test, **on a dependency. The panel this sits on is not drawn at the moment the run fails.** **S1** and **S2** read it fresh from the machine on the connection the user opens, with no second home on the desktop, and the design says so itself two paragraphs earlier: *"closing the panel and coming back tomorrow shows the same thing because the machine still says it."* So the failed step's **raw machine output** has to be durably retained on the machine, beside Y4's boundary list, and readable over a later connection with it. **Nothing states that it is retained, where, or under what term.**

Waits on: nothing to get. **A decision on where a step's own output lives and how long, and a dependency on `Rented-server runtime` §13's retention table having a term for it.**

*(Superseded 2026-08-25 — the paragraph below is what the sweep found, and the answer follows it.)*

**Not answered anywhere the sweep could find.** **S1** gives *"the operation record and the journal name the step that was in flight and how the run ended"* — the step's identity and the run's ending, **not the machine's printed text**. **W1** does attach the machine's own words to two endings — *Could not be made ready* is *"named with the machine's own words behind it"*, and *Refused* is *"named with the step that was refused"* — but that is the ending's words at the moment the action ends, and **a cut-off run has no ending recorded at all in the case this panel is written for**. The event journal is structured events under `State, identity and recovery` §4, capped at 32,768 entries on a 512-byte budget (`Rented-server runtime` R2), and its captured-stream sibling is written for renders. **Y6a's new retention term covers the setup operation record; it says nothing about a step's output being part of it.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Server Setup.md` §4.1 **S2a**, five bullets: where the words live; who writes them on each route; the verbatim discipline; the cut-mid-flight case; and retention.

**Retention needed nothing new.** The words ride the ready-action record under **Y6b**'s per-kind term, and replacement at the next run's start is the design's own lifecycle. **B1** already has the worker capture both streams of every Blender run; **Y7** means an elevated step's streams land only in the desktop's loop, so the desktop writes that ending through **Y8**'s ordinary command path; **E4** classifies with the words in hand. `Preparation/Architecture/Security and licence.md` §13 rule 5 and `Preparation/Architecture/Rented-server runtime.md` **M7**/**M8** bind it — whole or not at all, apart, and never mined for a cause — and **R2** and §11 are why it is neither the journal nor the render sinks.

**The cut-off case is told from the failed case by the ending's kind, never by the words' absence.**

**Nothing routed. Nothing left unresolved.**


---

## Two things recorded at the ninth sitting and deliberately not filed as rows

**2026-08-25. The re-sweep found two Design faults under Rule 10 in this document, and neither is a row.** They are gathered with the sitting's other nine at `Audits/Architecture/The ninth sitting — what goes back to Design.md` and go back to Design; nothing here reopens. **One of them has a paper trail worth not losing, so it is named here and nowhere else in this file:** the design's *there is nothing standing for the user to clear*, with *Ignore* as a way out, is recorded in `Preparation/Groundwork/Open Threads.md` as a Design ruling taken on 2026-08-24 — **after Rule 10** — on a state question the architecture had already answered the other way at Row 439's **Y8b** and at **S6**, whose clearing set is closed at two. The other fault is the design's *nothing of it is drawn where Smoozy is not connected*, against Row 355's answer at **Y6a**. **The cross and the wording are Design's; the premises are not.**

**2026-08-25. One standing item was weighed and dropped, and is recorded so nobody files it again.** `Preparation/Groundwork/Open Threads.md` holds *"The running setup card has no way back to the panel that started it"*, left open by this document's first mockup seam round on 2026-08-24. **Nothing is owed to Architecture by it.** The route back to a panel is a screen's own arrangement, and the item's own closing line — *"the setup runs to its end either way, and the panel reports when it does"* — is `Preparation/Architecture/Server Setup.md` **W1**'s five endings, answered outright and readable over the connection under **Y6**. The design's own text on the running case is **Y4** and **Y6** and needs nothing new.
