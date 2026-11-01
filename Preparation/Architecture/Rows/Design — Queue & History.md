# Rows — Design: Queue & History

Rows **377 to 400**, from `Preparation/Design/Queue & History.md`, **Rows 432 to 434**, filed later by the both-ways pass over the Design documents, and **Rows 494 to 497**, filed 2026-08-25 by the Design close re-sweep. Part of `Preparation/Architecture/Coverage.md`; how to read a row is there, and the facts they wait on are in `Preparation/Architecture/Facts.md`.

Every number here is permanent, never reused and never renumbered.

**Filed at the stage close, 2026-08-20**, from the first sweep the Design documents have ever had, on the Creator's ruling of the same day. Every row says what kind of promise it rests on, because a drafted sentence may be reworded when Design resumes and the number stays either way.

**This is the largest of the twelve Design files, and it should be.** The queue and history entries are almost entirely readings — a frame count, a sample pair, a percentage, a duration, a memory figure, an ending — and each is a value something has to have been arranged to produce. **Most of them are answered**, and by one document: `Preparation/Architecture/Queue & History.md` was written against the spec and covers almost the whole screen.

**Three promises this document states are filed elsewhere**, because another Design document states them too and one promise takes one number. The setup's italic-and-bold convention, holding on a job's entry where the settings are read at a glance rather than worked in, is **Row 319**; the history row's badge for a server that no longer exists is **Row 320**; a server shown from memory reading as such is **Row 323**. All three are in `Rows/Design — Whole app.md`.

**At the stage close on 2026-08-20, Rows 379 and 380 were the two answered nowhere** — the two of the entry's four marks that nothing wrote down — **and Rows 377, 378, 381 and 382 were partly answered**, two of those four with the architecture saying the opposite of what the screen draws. **All six were answered on 2026-08-21**, and each row below carries its own answer and the date it was given.

---

**Row 377 — the top bar's label where Blender does not name the View Layer.** *A rendering entry.*
> "**The top bar is one View Layer of one frame, and the label is what makes that visible.** The [spec](../../Specs/Queue%20&%20History.md) counts both bars in samples: the top one over the piece being rendered now, which is one View Layer on one frame. On a frame with three active View Layers that bar therefore fills and empties three times while the frame number stays put — and a bar that resets for no visible reason is the worst thing on the entry. So the line above it names what it is measuring: *Frame 61 · Reflections*, or *Frame 61 · View Layer 2 of 3* where Blender does not give its name. Label: **Creator-stated** (2026-08-13)."

and
> "**Where Smoozy cannot name the View Layer at all, it says the count and not the name.** Blender 4.5 puts the View Layer on its progress line and the newer three do not — the [spec](../../Specs/Queue%20&%20History.md) owns that fact — so on those the line says which of how many rather than which one. It never leaves the reset unexplained."

Kind of promise: **a settled value** — the mockup draws `Reflections — View Layer 2 of 2` on the *Rendering now* line, with a comment beside it reading *"Where Blender does not give the View Layer's name — every version after 4.5 — it says which of how many instead."* The paragraph is additionally **Creator-stated** (2026-08-13).

Earns its row by the third test in its sharpest form: the entry promises a reading — *which of how many* — at a moment nothing tells it. **The name half is answered; the count half is a figure the machine never sends.**

Waits on: a Blender fact, already looked up rather than open. `Preparation/Architecture/Queue & History.md` R2 and R10 read it from the joined answer `While a render runs.md` — 4.5's progress line names frame and view layer, and in 5.0 and later *"the view layer comes from nowhere at all"*. Nothing further is owed from research.

**Partly answered, and the unanswered clause is contradicted.**

- **Answered:** Section 5 **R10** — *"In 4.5 the line names frame and view layer — one renderlet — and the entry names it."* So *Frame 61 · Reflections* holds on 4.5.
- **Not answered, and contradicted:** the same R10 continues — *"In 5.0 and later the view layer comes from nowhere at all, so the entry names the frame alone, which is the spec's own allowed fallback, taken here once and not revisited per job."* **The architecture's fallback is *the frame alone*; the Design document's and the mockup's fallback is *which of how many*.** The *of 3* is available — **B1** takes the per-frame active view layers from the recorded reading — **but the *2* is not**: nothing on the newer three's stream marks the boundary between one view layer and the next.
- **The sharper half rides with it.** **B3** has the first bar *"fill and finish once per renderlet — a frame with three active layers is three fills"*, while **R11** feeds that bar *"the largest sample count seen **for the frame in hand**, never the latest"*. **A largest-seen-per-frame reading cannot fall back at a view-layer boundary, so on 5.0 and later nothing has been arranged to make the bar reset at all**, let alone to label the reset. The Design document's stated reason for the label — *"a bar that resets for no visible reason is the worst thing on the entry"* — is aimed at exactly the case the architecture leaves open.

Where the sweep looked: `Queue & History` Sections 5 and 6 entire and Section 18 (item 19 is about tiled units, not this); `Rendering` Sections 1 and 5; `Job files and outputs` I6 for the per-frame view-layer set.

**Answer, 2026-08-21: `Preparation/Architecture/Queue & History.md` [B3a](../Queue%20&%20History.md#6-the-two-bars-and-the-countdown-that-is-not-offered), with R10's fallback standing.** **The row's second half is the one that decided it.**

- **B3 read as one rule for every version and cannot be**, which is what this row found: *"a frame with three active layers is three fills"* against R10's *"the view layer comes from nowhere at all"* on 5.0 and later. **B3a makes the bar's unit per version** — the renderlet on 4.5, **the frame on 5.0 and later**, filled against the frame's whole planned samples and not resetting inside a frame.
- **So the reset the Design sentence exists to explain does not happen on those versions**, and its *which of how many* fallback is a label for a boundary the stream does not carry. **The count is available from B1 and the ordinal is not**, and no ordinal is composed.
- **Nothing infers a boundary from the sample reading falling**, which would be the obvious move and is refused: R11 takes the largest seen for the frame in hand precisely because the latest is wrong at an ending, and R12 fixes the trailing zero as the stopped-short mark *"and as nothing else"*. **Reading a boundary out of either would be modelling a stream nobody has observed saying it** — the fault this project has already paid for once.
- **On 4.5 the promise is kept whole**: *Frame 61 · Reflections*, three fills, the label naming the layer.
- **The drafted *View Layer 2 of 3* fallback is parked for Design** rather than repaired here; B5 and B6 are untouched, since the plan total stays per renderlet whatever the bar's unit is.


**Row 378 — a bar that finishes short goes to its end.** *A rendering entry.*
> "**A bar that finishes short is right, not broken.** A renderlet with adaptive sampling on stops when the picture is clean, so it can end at 200 of 500 samples. The bar goes to its end when the work ends rather than sitting at 40% looking stalled, and nothing on screen treats the shortfall as a fault."

Kind of promise: **a drafted design sentence.**

Earns its row by the third test: filling a bar to its end at the moment a renderlet ends short requires the app to be told **the renderlet has ended** — a moment distinct from the last sample reading it received.

Waits on: a Blender fact, already looked up — that a frame stopping short prints one line reading zero of the total just before it ends. `Preparation/Architecture/Queue & History.md` R11 and R12 carry it from the joined answer. Nothing further is owed.

**Partly answered, and the architecture says the opposite.**

- **The signal exists and is read:** **R12** — *"The trailing zero is read as the frame's stopped-short mark, and as nothing else. It says the frame ended short of its count — converged, out of time, or stopped — and it does not say which; the entry states the fact without a cause."*
- **But B3 states what the bar does with it:** *"A short-stopping frame ends the bar where the largest-seen reading stands; the trailing zero never drags it back (R11, R12)."* **That leaves the bar at 200 of 500 — the 40% the Design document names as the thing to avoid.** The architecture keeps the bar from going *backwards*; the Design document asks it to go *forwards to full*.
- **The second half of the promise is answered.** *"nothing on screen treats the shortfall as a fault"* is **B5**: *"The bar's claim is bounded to the plan — it is progress against the planned units, and a job can correctly end short of it or pass it."*

**Answer, 2026-08-21: `Preparation/Architecture/Queue & History.md` [B3b](../Queue%20&%20History.md#6-the-two-bars-and-the-countdown-that-is-not-offered), and the architecture yields.** **The row is right and B3's last sentence was the fault.**

- **The rule is now two clauses, not one: the bar never moves backwards, and never sits short of full for a unit that has ended.** R11's *largest seen, never the latest* is kept whole; the second clause does not touch it, because **filling on an ending is not a sample reading at all** — which is what B3 had missed.
- **The ending signal is R12's trailing zero, used for exactly what R12 says it means** and not as a figure. Where no trailing zero was seen, the unit's end is the next unit beginning or the job's own ending, both already read.
- **The figures do not move.** The counts beside the bar still read 200 of 500, because R11's *both figures are shown as printed* is what stops the fill becoming a claim that 500 samples were taken.
- **B5 is what makes it honest rather than a lie**: a filled bar says *this unit is done*, not *this unit did all the work planned for it*.


**Row 379 — the mark that nothing checked the file's dependencies.** *The marks.*
> "- **Nothing checked this file's dependencies when it was submitted.** Same shape: a note of what was true at that moment, checked against nothing and re-checked never."

with the rule that binds it, two paragraphs above:
> "**A mark is one line saying something that holds true of this job**, sitting between the live readings and the setup. There are four, each shown only when it is true, and every one of them stays on the entry into history."

Kind of promise: **a settled value** — the mockup draws it on the rendering entry as `Nothing checked this file's dependencies when it was submitted.` The Design sentence carries no label; the spec's own version is **Creator-stated** in `Preparation/Specs/Queue & History.md` under *What an entry shows*.

Earns its row by the third test: **the mark is a fact about a moment that has passed, shown on an entry that outlives the machine** — so something has to have written it down at submission. Nothing has been arranged to.

Waits on: nothing outside. Everything it needs is Smoozy's own record.

**Not answered anywhere.**

- **The *warning* is fully answered.** `Preparation/Architecture/Rendering.md` Section 5 **W1**: *"The warning is evaluated at the moment of submitting, from the check standing alone, with no read of the file and no read of the machine."* W2 gives its three record states.
- **What is not answered is the mark on the entry.** W1's evaluation is against `Preparation/Architecture/Dependency Upload & Remap.md` **V10**'s check standing, which is keyed to *"(root file's machine path, machine under its installation identity)"* and is *"disqualified with everything else machine-keyed"* when the machine is recycled. **The standing is a live record about a file on a machine; the mark has to be a frozen note about a job, kept for ever. No document copies the one into the other.**
- `Preparation/Architecture/Queue & History.md` Section 17 enumerates what persists — *"the entry's disturbed-file and simplify marks persist into history unchanged"* — **and the dependency-check mark was not among them.** Its Section 2 and Section 10 field lists (the local job record at N11, the per-job note at N1, the recorded reading at N9) carried no such field either.

Where the sweep looked: `Queue & History` Sections 2, 10, 11, 16 and 17; `Rendering` Sections 4 and 5; `Dependency Upload & Remap` §11 and V10–V14; `File Browser` §8.4; `Coverage.md`; every row file. **No row names it** — Rows 47 to 87 and 302 do not, and `Rendering`'s Row 5 stops at the warning.

**Answer, 2026-08-21: `Preparation/Architecture/Queue & History.md` [N15](../Queue%20&%20History.md#10-the-note-the-figure-and-what-the-read-fills-in), first half, widened in place 2026-08-22.** The local job record carries, written at submission, **the record state the missing-assets evaluation found at that moment** — `Rendering` W2's three, or that a complete standing satisfying W1 stood, under which no mark is drawn. **One field wide enough for the state, never a boolean:** the Design sentence this row quotes is *one state of the mark and not the mark itself*, and a single bit worded as it would assert the thing W2 forbids — *"the warning is never allowed to say nobody checked where one stands"* — against exactly the job whose file was checked and then replaced. The wording of each state is Design's.

- **Nothing new is computed.** `Rendering` W1 already evaluates exactly that *"at the moment of submitting, from the check standing alone"* — **this is the same evaluation written down instead of only acted on.**
- **It has to be a copy and the row's own finding is why.** V10's check standing is keyed to *(root file's machine path, machine under its installation identity)* and is disqualified with everything else machine-keyed when the machine is recycled. **The standing is a live record about a file on a machine; the mark is a frozen note about a job, kept for ever.** Nothing re-checks it and no later check changes it.
- **Section 17's persistence list is what it joins**, beside the disturbed-file and simplify marks, so the four marks are one set on one rule. **A fifth joined that set on 2026-08-25 — N22's submitted-with-no-scene-named mark, on the same terms, answered at Row 494 — and N15's closing bullet now reads five.**
- **Absent on a record written before the rule**, and absent draws as no mark at all — the design's own *shown only when it is true*.


**Row 380 — the mark saying which job this one carries on from.** *The marks.*
> "- **It carries on from an earlier job**, named, with a way to that entry. Without it the user is looking at two entries of one file over one range with nothing to say which came first."

Kind of promise: **a drafted design sentence.** The spec's version is **Creator-stated** (2026-08-12) in `Preparation/Specs/Queue & History.md` under *What an entry shows*; the Design document restates it without a label.

Earns its row by the third test: **a carry-on makes a new job with a new identity**; the entry has to say which earlier job it continues and reach that entry. That is a field linking one job record to another, and a promise about the earlier entry still being reachable.

Waits on: nothing outside.

**Not answered anywhere.**

- **The action is answered.** `Preparation/Architecture/Rendering.md` **A4** gives the figure the new job starts after, read from the machine when the action is taken; A5, A6 and A9 give the fallbacks. `Preparation/Architecture/Queue & History.md` **N6** gives the kept figure. **Y5** confirms the shape: *"a fresh attempt the user asks for is a new job with a new identity and a new folder."*
- **What is not answered is the link.** **Nothing anywhere records, on the new job, the identity of the job it carries on from.** `Queue & History` **Q2** lists the machine-side job record's contents — job identity, group identity, sequence key, queue state, the record-directory reference — **and no predecessor field**; **N1** lists the per-job note's contents and has none; **N10–N11** describe the local record's kept setup and have none. `Rendering` Section 6 never writes one.
- **The second clause is unanswered with it: *with a way to that entry*.** Deletion is unconditional and leaves a tombstone (**X1–X5**), so **the earlier entry can be gone while the mark still names it**, and no document says what the link does then.

Searched every architecture document and every row file for *carries on from*, *carried on from*, *predecessor* and *the earlier*; no hit was this promise.

**Answer, 2026-08-21: `Preparation/Architecture/Queue & History.md` [N15](../Queue%20&%20History.md#10-the-note-the-figure-and-what-the-read-fills-in), second half.** The local job record carries, written at submission by the action that made the job, **the earlier job's identity and its label as it stood.**

- **It is the same copy-at-submission shape as N12's server label and hue**, answered the same day for Row 320, and for the same reason.
- **The label is copied because the link can outlive its target, which is the row's second clause.** Deletion is unconditional and leaves a tombstone (X1 to X5), **so the earlier entry can be gone while the mark still names it.** With the label copied the mark reads whole **where the earlier entry is gone, and the way to that entry is then simply not offered**; with an identity alone it would be a name Smoozy could not read.
- **The way itself was settled two days later: [N20](../Queue%20&%20History.md#10-the-note-the-figure-and-what-the-read-fills-in), added 2026-08-23 per Row 449.** It is a lookup at the draw, on the copied identity, against the local record alone — no connection opened, no machine read, nothing cached. **A standing record means the way is offered; a tombstoned identity and an identity no record names read alike as not offered.** One field serves both lists, the queue row drawing it by the same join a group heading uses, the machine holding no carry-on field of any kind. **The other direction is held nowhere**: the earlier job's record is never written to, so it can be carried on from more than once at no cost, and deleting it takes nothing of theirs.
- **N10's merge rule leaves it alone** — it is Smoozy's own record and no reading fills or overwrites it — and it is absent on a record written before the rule.


**Row 381 — the times an entry keeps, including *submitted*.** *History.*
> "**It keeps its times** — submitted, started, ended, and how long it rendered — and, where it can be had, which parts of the server did the work."

Kind of promise: **a settled value** — the mockup draws it three times in history as `Submitted · 4 Aug 21:40`, and once in the no-ending status line, `Ending not known — submitted 4 Aug 21:40`.

Earns its row by the third test: four figures on the screen. Three are settled; **the fourth — *submitted* — is a moment nothing has been arranged to record, and it is the one moment that can only be taken on the desktop's clock**, which the architecture forbids subtracting against the machine's.

Waits on: **Fact 13**, numbered and answered in part — **the half this row uses is the answered half**: the user's computer's clock is undefended and nothing disciplines it. Fact 92's qualifier (nothing disciplines a rented machine's clock either, and a container has no clock of its own) sits under the other three times.

**Partly answered.**

- **Answered: started, ended, and how long it rendered.** **N1** — the per-job note holds *"the **times** — the moment the render started and the moment it ended, on the machine's clock, each written at the event that makes it true"*. **N4** puts them under D5: *"the duration an entry shows is computed from those two readings and never across clocks"*. **C9** adds that *"The started moment shown is the note's, on the machine's clock, never the moment Smoozy learned it."* The device line is E12–E13.
- **Not answered: the submitted moment.** N1's note has no such field — correctly, since the note is written on the machine and submission happens on the desktop. **N11** names what the local job record holds at submission — scene name, typed percentage, no pixel size — **and no moment.** `Preparation/Architecture/State, identity and recovery.md` §7 step 2 has the provisional local job record *"written locally at submission and before the submission is sent"* but does not say it carries the moment, and §1.1's inventory names stores rather than fields.
- **The clock question rides with it, unanswered.** **D5**'s rule is that *"the desktop's clock is presentation and nothing a promise rests on"*, and a *submitted* moment can only be the desktop's — **so an entry showing all four times side by side is showing one figure from a clock the architecture has ruled presentation-only**, and nothing says whether that is acceptable or how it is marked.

No row covered it: Row 76 is the machine's note, Row 83 is the last-frame figure.

**Answer, 2026-08-21: `Preparation/Architecture/Queue & History.md` [N16](../Queue%20&%20History.md#10-the-note-the-figure-and-what-the-read-fills-in)**, with the other three times answered at N1, N4, C9 and E12–E13 as this row records.

- **The local job record carries the submitted moment, on the desktop's clock**, written when the provisional record is written and before the submission is sent — joining the write `State, identity and recovery` §7 step 2 already requires.
- **The row's clock worry is answered by D5's own sentence** — *"the desktop's clock is used for presentation and for nothing a promise rests on"* — **and nothing rests on it**: no ordering, no timeout, no reconciliation, no decision anywhere reads it. It is drawn and nothing else.
- **It is never subtracted against anything.** N4's *how long it rendered* stays the note's two machine-clock readings. **The entry shows four moments and exactly one duration, and the duration is machine-clock-only.** The gap between *submitted* and *started* is not computed, and where the two clocks disagree the entry shows two moments that do not line up rather than a number that is wrong.
- **Absent on a record written before the rule**, which is the mockup's own *Ending not known — submitted 4 Aug 21:40* shape.


**Row 382 — the catch-up notice as one message covering several servers.** *The catch-up notice.*
> "**One message, not a burst.** It is a two-column list — the job's name, then what happened to it — rather than a sentence naming several jobs in a row."

and
> "**One connection can find that more than one server has been busy, so the notice groups by server**, each group under a heading carrying that server's badge. That is the same shape the queue list already uses, and for the same reason: a flat list across servers invites the user to read an order that does not exist. Label: **Creator-stated** (2026-08-13)."

Kind of promise: **Creator-stated** (2026-08-13) for the grouping; the *one message* sentence carries no label and is **a drafted design sentence.**

Earns its row by the third test: **one message spanning several servers is a thing with a lifetime** — it is folded, it is dismissed, its heading counts what is behind it. The architecture's notice record is per server and ends with the server row, so a message spanning servers has no home.

Waits on: nothing outside.

**Partly answered.**

- **Answered: what the notice says and how it is composed.** **C1** — *"The catch-up notice is composed from exactly the events §7 step 3 applies, and nothing else ever composes one"* — naming the endings, drops and re-runs the two-column list needs, per job, by label from N1. **C4** answers the incomplete-account case. `Preparation/Architecture/Desktop app and delivery.md` **T18** draws the *now* boundary.
- **Not answered: the notice as one record across servers.** **C2** makes the record singular and **per machine** — *"its lifetime class is **subject-held on the server row** (§11.1) … it is about one machine's journal, it ends with the row, and dismissal is the user's own act on Smoozy's own record"* — and adds that a second connection in the same session *"composes a notice only of what is new, or none"*. **So three servers connected in one session produce three records at three moments**, and nothing arranges the single message the Design document promises, its fold state, its dismissal across servers, or the heading's *4 jobs on 2 servers* count taken across them.
- **C2 hands the rest over explicitly** — *"How it is shown and what dismissal does are Design's, per the spec's own pending"* — so this is a boundary question rather than a contradiction, and the Design document has parked half of it itself: `[PENDING: whether the catch-up notice survives being dismissed]`.

**Answer, 2026-08-21: `Preparation/Architecture/Queue & History.md` [C2a](../Queue%20&%20History.md#11-one-job-two-records-two-lifetimes).** **The message is a presentation over the standing records and never a record of its own, so C2 does not move by a word.**

- **Nothing spanning servers is stored anywhere.** There is no second lifetime to reason about and no record that could outlive a row it is about — which is exactly what the row found missing and what a spanning record would have cost.
- **The message is every standing, undismissed notice record at the moment it is drawn, grouped by server**, one group per record. **A second server connecting later in the same session adds its group to the same message**, because the message is composed at draw time and the earlier record is still standing. That is the row's *three servers at three moments* answered.
- **Dismissing the message dismisses every record behind it**, each on its own terms, which is what makes C2's *told once* survive a restart unchanged and stops a dismissed message reappearing. Dismissing one group dismisses that record alone.
- **The heading's count is composed at draw time** from the same set and is stored nowhere.
- **A server removed while its group stands takes its record with it** and the message is one group shorter next time it is drawn.
- **What it looks like, whether it folds, and what dismissal looks like stay Design's**, and the parked `[PENDING]` is untouched.


**Row 383 — the folded server heading still says what is running.** *The queue list.*
> "**A server's line folds away on its heading, and a folded heading still says what is running.** The job's name, how far through it is, and how many are waiting behind it. Folding a server whose work is settled is the point, and a heading that went blank when folded would make the user open it again to learn anything."

Kind of promise: **a drafted design sentence.**

Earns its row by the third test: three figures, of which two are readings — the running job's progress, and the count of jobs waiting behind it on that server.

Waits on: nothing outside.

**Answered already.** **A1** — *"The queue reading is one request, on demand … and it returns the machine's whole working state. Per job: its state, its admission point standing, its operation, its unreleased hold … plus the note (N1)"*. The line and its order are **Q4**'s — *"The line is the job records with state *waiting*, in ascending sequence-key order"* — so the count behind the running job is a count of that reading's *waiting* records. The progress figure is **B5**'s whole-job percentage; the job's name is the label N1 holds whole.


**Row 384 — the three states in the queue column.** *The queue list.*
> "What that column carries is the states that are *not* readable from position: *Rendering*, *Waiting on you*, *Not updating*. The last of those used to read *Last known*, which said last known **what** — and the server's own heading already said *not updating*, so one thing had two words. Label: **Creator-stated** (2026-08-13)."

Kind of promise: **Creator-stated** (2026-08-13), and **a settled value** — `Waiting on you` and `not updating` both appear in the mockup.

Earns its row by the third test: three status words, each of which has to be true of a job at the moment it is drawn.

Waits on: nothing outside.

**Answered already**, across three clauses of `Preparation/Architecture/Queue & History.md`:

- *Rendering* — **Q2**'s closed set of queue states: *"*waiting*, *held* (with the hold identity …), *rendering* (with the render operation's identity), *ended* (with the recorded outcome's identity) — each written at the event that makes it true and journalled."*
- *Waiting on you* — **H1**: *"A held job's entry says it is waiting on an answer, and offers both answers … The entry's state derives from the hold record … never from a second store."*
- *Not updating* — **C7**, re-keyed 2026-08-22: *"A last-known entry carries the moment it was taken and says it is not updating — the marking is composable because the reading carries its own moment, and its trigger is the break declaration, never the period."* **The stamp and the statement appear at the moment `Servers` declares the break and tears the connection down**, and stand until a later connection takes a fresh reading; they also reach a reading held while a connected server's reconciliation has not completed, **because the mark is about the reading's age and never about the connection's state**. The entry shows that stamp and that statement — *wording Design's* — and never its live readings as current.


**Row 385 — the status line, and how long the render has been going.** *A rendering entry.*
> "**It opens with a plain status line** — what it is doing, and how long it has been doing it — rather than starting in the middle of a chart."

Kind of promise: **a settled value** — the mockup draws `Rendering — started 2 hours 41 minutes ago`.

Earns its row by the third test: **an elapsed figure for a job that has not ended is a subtraction, and the architecture forbids subtracting across clocks.**

Waits on: **Fact 92**'s qualifier — nothing disciplines a rented machine's clock and a container has no clock of its own.

**Answered already.** **N1** puts the started moment in the note *"on the machine's clock"*; **A1** makes the queue reading itself *"stamped with moment and clock"*; **N4** binds the arithmetic — *"Both moments are the machine's own clock; the duration an entry shows is computed from those two readings and never across clocks; the desktop's clock is presentation only."* **W3** is the worked precedent for a still-running interval: *"Overdue is machine-now minus quiet-moment, both taken from the machine in one reading."* The honest limit is carried at Section 18 item 10.


**Row 386 — counts written as counts.** *A rendering entry.*
> "**Counts are written as counts.** *148/240*, *812/2048 samples* — not *frame 148 of 240*. The label beside them already says which is which, and a fraction is read at a glance where a sentence is read word by word. Label: **Creator-stated** (2026-08-10)."

Kind of promise: **Creator-stated** (2026-08-10), and **a settled value** — `Frame 148/240` and `812/2048 samples` in the mockup.

Earns its row by the third test: four numbers, two of them per-version readings of a stream.

Waits on: a Blender fact, already looked up — which figures the progress stream carries per version. **R2, R9 and R11** read it from `While a render runs.md`.

**Answered already.** **R9** gives the frame in hand — *"read off the line in 4.5, counted off the announcements in 5.0 and later"* — against the attempt record's frame list, which is where the *240* comes from. **R11** gives the sample pair: *"the largest seen for the frame in hand, never the latest, and both figures are shown as printed … the entry shows the line's own figures as Blender's, and never substitutes Render Settings' number for the printed total"*. **R4** states the dependence on how the render was started and hands it to `Rendering`'s Row 11, answered there.


**Row 387 — the tile count, only where Blender gives one.** *A rendering entry.*
> "**The tile count sits on the *Rendering now* line, beside the samples, and only where Blender gives one.** A frame small enough to be one tile has no tile figure and the line simply does not carry it; a frame split into tiles reads *780/53284 tiles* alongside the sample count. Label: **Creator-stated** (2026-08-11)."

*(Re-quoted 2026-08-25 at the Design close re-sweep. **The promise is unchanged; the line it names was renamed on screen** — *what is rendering now* became *Rendering now*, the same rename that broke Row 432. Nothing about what the entry must produce moved, and the answer — R13 with R7's absent-rather-than-wrong rule, and B2's threshold arithmetic — stands untouched.)*

Kind of promise: **Creator-stated** (2026-08-11), and **a settled value** — `780/53284 tiles` in the mockup.

Earns its row by the third test: a figure that appears on some frames and not others, decided by a threshold in Blender.

Waits on: a Blender fact, looked up and partly open — what a tiled progress line holds end to end. **R7** says every progress line on record *"came from a picture too small to be split"*, and carries the gap at Section 18 item 8.

**Answered already, with the limit stated.** **R13** — *"Tiles: shown where printed, and never combined with samples into one figure"* — and **R7**, which labels the grammar rather than trusting it: *"where a real tiled stream differs, the reading is absent rather than wrong, by R3's rule."* **B2** gives the threshold arithmetic and names its one unobserved case. **The Design promise's own shape — *only where Blender gives one* — is exactly what R7's absent-rather-than-wrong rule delivers.**


**Row 388 — the whole-job bar's percentage, and the withdrawn gloss.** *A rendering entry.*
> "**The whole-job bar shows a percentage and says nothing beside it.** It used to carry a line explaining what it was counting — *two layers per frame, so the bar counts both* — and that line was withdrawn on 2026-08-13 because it can simply be false: how many View Layers are active is decided per frame, so one frame can have none and the next ten, and there is no per-frame figure to state. A percentage of the whole job needs no gloss. Label: **Creator-stated** (2026-08-13)."

Kind of promise: **Creator-stated** (2026-08-13), and **a settled value** — `62%` on the *Whole job* line in the mockup.

Earns its row by the third test: a percentage of a whole job needs a total, and the total needs the per-frame active view layers **before the job runs**.

Waits on: **Fact J** — a whole job's renderlet total where the view-layer switch is driven or changed by a script — which left the stage for Blender Research and is unanswered.

**Answered already, with an unresolved item riding on it.** **B1** computes the plan total once from the recorded reading, *"per group member separately — a group is several counts, never one"*; **B5** makes the bar's claim bounded to the plan; **B6** advances it on completed frames. **The Design document's stated reason — that the per-frame count varies — is B1's own construction.** Fact J's residue is at Section 18 item 4, and **B8** states what it costs: *"a driven or scripted file can meanwhile carry a percentage of a wrong total, bounded by B5's plan-not-promise presentation."*


**Row 389 — the readings from the server.** *A rendering entry.*
> "**Then the readings from the server**: the whole GPU's memory against its capacity, and free disk."

Kind of promise: **a drafted design sentence**, drawn as **a settled value** in the mockup — `VRAM 18.2 / 24 GB`, `Free disk 61 GB`.

Earns its row by the third test: two live machine readings, neither of which comes from Blender.

Waits on: **Facts 43, 28 and 46**, all answered; Fact 43's remainder (two renders on two cards, one render spanning both) is carried unresolved.

**Answered already.** Section 7 entire. **G1** — *"The card reading is per card, card-level, taken by the supervisor through the driver's own per-card query, unprivileged, on request."* **G2** splits capacity from usage: *"capacity is the server list's — `Servers`' — and the live usage is this document's, per the spec's own split."* **G3** rules out the host-inherited routes on a container. **G4** — *"The disk figure is the available figure at the root's mount, read per-path, and shown as of its moment."* **G5** puts both on the health-readings route, period-held.


**Row 390 — the four marks, each shown only when it is true, each staying into history.** *The marks.*
> "**Most jobs carry none of this, and the entry shows none of it.** A job with a disturbed file, no dependency check, Simplify on and an earlier job behind it is a job in trouble four ways over, and drawing it as the ordinary case makes every entry look alarming. The plain entry is status, progress, readings, setup, output — and nothing else. Label: **Creator-stated** (2026-08-13)."

with the two marks not filed as rows of their own:
> "- **A file the job needs was disturbed**, naming the file. The [spec](../../Specs/Queue%20&%20History.md) is exact that the mark changes nothing about the job — it may still render perfectly — and exists so a failure hours later is not unexplained."

> "- **Simplify was on.** The frames will not match the file the artist saved, and nothing else on the entry would say why. It says Simplify was on and no more — what each setting was set to is part of the setup below."

Kind of promise: **Creator-stated** (2026-08-13) for the shown-only-when-true rule; the two mark sentences are **drafted design sentences** drawn as settled values in the mockup.

Earns its row by the third test: each mark is a condition that has to be knowable, per job, and has to survive into history.

Waits on: nothing outside.

**Answered already, for these two.**

- **The disturbed-file mark:** `Preparation/Architecture/File Browser.md` §8.4 **W8–W10** is Row 196's telling — a durable write landing *"**before the disturbing act's first machine effect**"*, *"enumerating the affected jobs (by job identity) and the act intended"*, with W9 composing two readings, *disturbed* and *may have been disturbed*. Its supply paragraph hands them over: *"the disturb action's ordered (telling, act) record in the local record, and the two composed readings"*. `Queue & History` Section 17 receives them *"carried onto the entry as fields, their wording Design's"* and states that they *"persist into history unchanged"*. **The mark's own field, its write and its lifetime are [N21](../Queue%20&%20History.md#10-the-note-the-figure-and-what-the-read-fills-in)'s — added 2026-08-23, per Row 450**: a copy on the job's own local record, written **per disturbance** and appended rather than replaced, landing in the same durable local write as W8's telling, the reading completed once when the act's per-job outcome is learned and the mark removed where none of that job's units took effect. It is Smoozy's own record, so nothing fills or overwrites it, it outlives the machine and the server row, and it goes only with the entry. **One thing is owed and named**: a sentence in `File Browser` §8.4 saying the telling's write also lands the mark — [Section 18](../Queue%20&%20History.md#18-unresolved) item 22.
- **The Simplify mark:** `Preparation/Architecture/Rendering.md` **L10** carries Simplify's switch and its three settings as settings pairs on the payload; `Preparation/Architecture/Render Settings.md` **Y15** carries it in the kept setup; `Queue & History` **N10** makes the kept setup the entry's, and Section 17 names the simplify mark as persisting into history.

**The other two of the four are Rows 379 and 380, and both were answered on 2026-08-21** at `Preparation/Architecture/Queue & History.md` **N15** — two frozen fields of the local job record, joining Section 17's persistence list on the same terms as these two. **So the *four marks, each staying into history* rule is kept for all four**, and a fifth joined the same set at **N22** on 2026-08-25, answered at Row 494.


**Row 391 — the six statements, and the two that hold the job.** *A rendering entry.*
> "**Six statements, and they arrive at the start of the render rather than at submission.** Where the user pre-loaded, they read all six on the settings page and this block only repeats what is already settled. Where they did not, this is the first they see of it — the read happens when the job starts, and the [What a job writes spec](../../Specs/What%20a%20job%20writes.md) puts all six on the entry at that moment."

and
> "**Two of the six hold the job, and those two carry the choice.** Writing nothing at all, and a colliding pair, stop the job before any rendering is paid for — so those two read as a question with **Stop** and **Carry on** beside them, while the other four are statements with nothing to answer."

Kind of promise: **a drafted design sentence.**

Earns its row by the third test: six statements that arrive at a specific moment, two of which change the job's state and offer answers.

Waits on: nothing outside.

**Answered already.** **N9** — *"The recorded reading — scene name, picture size, the six statements, the folders, the collision record, the writes-nothing mark, the video shape, the main output, a destination recorded as not mappable with its reason — is composed on the machine, journalled as an event when written, and sits in the job record directory for §7 step 4's fill-in … The entry learns it on the next connection however long that takes — or, where a connected period is standing, from the same journalled event on `Desktop app and delivery` 3.3 P10d's recurring read-forward; nothing polls and nothing is pushed is that clause's and §8.4's sense."* **The second route is what keeps the row's *at the start of the render* while the user is watching** — the reading's own journalled event is read forward on the standing period, so the six statements reach the entry as the job starts rather than at the next connection; it was added to N9 after this row was answered. The holding half is **H1** — the entry's two answers derive from the hold record — with `Preparation/Architecture/Rented-server runtime.md` §8 as the one hold mechanism, and `Preparation/Architecture/Job files and outputs.md` owning which reads hold.


**Row 392 — the picture size, and the job whose file was never read.** *A rendering entry.*
> "**The picture size is the percentage and the pixels together**, the pixels in the fixed-width face beside it — *50% · 1920 × 1080*. The [Render Settings spec](../../Specs/Render%20Settings.md) settles that the pixel figure is a reading rather than a setting, and the entry keeps the same pair the settings page showed. Where the file was never read there is no full size to take a percentage of, so the entry carries the percentage alone where the user typed one and says the size is not known."

Kind of promise: **a drafted design sentence.**

Earns its row by the third test: a pair of figures, one of which exists only after a read of the file, and a stated fallback for when there was none.

Waits on: nothing outside.

**Answered already.** **N10** — *"the picture size is not carried and fills in from the read, exactly as the spec splits them"* — and **N11**: *"No scene name — submitted rendering whichever scene the file was saved showing; a percentage only where the user typed one; no pixel size at all … they are fields of the local job record written at submission, so the entry reads them with no machine and no read."*


**Row 393 — what Blender is saying, and the render that printed nothing.** *A rendering entry.*
> "**Then what Blender is saying**, closing the failure shape and standing as the shape's own raw text, with the [whole-app Design](../../Design/Whole%20app.md)'s named control beside it: the control says whose words they are, and the panel it opens gives a render's two streams a tab each. Smoozy shows those words and draws no conclusion from them, apart from the one carve-out below."

*(**First quote re-quoted 2026-08-25** at the Design close re-sweep. **Only the first of this row's two quotes broke; the second — the printed-nothing paragraph below — is byte-identical to the document and was not touched.** The first promise genuinely moved and gained three things, and **the answer stands and covers all three, so nothing moves at Architecture**: ***a render's two streams a tab each*** is E7's — both streams *"captured apart by the worker into sinks the supervisor made, retained verbatim"* — and E9's fill-in, which takes them into the history record *"kept apart, unedited"*, per attempt; ***closing the failure shape*** and where it sits are the whole-app Design's and this screen's, not Architecture's; ***apart from the one carve-out below*** is the full-disk sentence, answered whole at `Preparation/Specs/Whole app.md`'s carve-out from the no-diagnosis rule and `Preparation/Architecture/Desktop app and delivery.md` M15a, which fixes the free-space reading as *"taken **on the machine, in the same act that writes that render's recorded outcome**, and reaches this side carried with that outcome"*, with *"A recorded outcome carrying no such reading is not one carrying a zero"*. E10's *Nothing reads the words* is untouched, because the carve-out is a machine reading and never a reading of the stream.)*

and
> "**Where a render ended having printed nothing, the entry says so in words rather than showing an empty panel.** On half the supported versions there is nothing to show — the [spec](../../Specs/Queue%20&%20History.md) has the machine's own memory running out killing Blender 4.5 and 5.0 outright with both streams empty — and a blank panel is exactly what a broken Smoozy would look like. So the panel is not shown at all; in its place one line says the render ended without printing anything and there is nothing further to read. It offers no cause. Label: **Creator-stated** (2026-08-13)."

Kind of promise: the second is **Creator-stated** (2026-08-13); the first is **a drafted design sentence.**

Earns its row by the third test: *its last line* is a live reading of a stream; **the printed-nothing case requires telling two silences apart, which no reading of the sink can do.**

Waits on: nothing outside. **The Blender fact the promise looks like it rests on is not one the answer consumes**, which the dated note below sets out: the two specs state it wrongly, and **E4** tells the silences apart from `Preparation/Architecture/Rented-server runtime.md` §11.3's four recorded collection outcomes (**M13**) and never from the stream.

**Answered already, and the harder half is answered better than the promise asks.** **E8** — *"While the job runs, the desktop reads the sink's current bytes over the connection, on request, as a period-held reading shown on the entry — reading material for display, never state."* **E9** carries the words into history whole. **E10** — *"Nothing reads the words"* — is the Design sentence's other half. And **E4** is the silence: *"*The render printed nothing* and *nothing of what the render printed reached Smoozy* are the same on a naive entry and are not the same event; §11.3's closed set of four — *collected*; *collected, and the stream was empty*; *collection incomplete*; *never established* — is what tells them apart … an empty panel is never what the user meets."*

**A dated note of its own, 2026-08-25 — a Blender fact two specs state wrongly, and it is the Creator's to correct.** Found by the Design close re-sweep of `Preparation/Design/Queue & History.md`, which looked the fact up by the fixed route rather than reasoning about it. **It is recorded here rather than only in the faults gathered for Design, because it is a spec correction and a faults pile is where it would be lost.** Nothing in this file edits a spec or a Design document.

- **What the research says.** `Preparation/Blender Research/Across topics/The map.md` sends *the job failed, what words come out* to the joined answer `When a render ends`, which names *Reports 10*; that question was opened and no other. `Preparation/Blender Research/Comparison/Reports.md` question 10: *"**The last line on standard output is a sampling line**, in every version"*; *"**Blender 4.5 and 5.0 fill the machine and are killed by the operating system.** Standard error is **empty**. Nothing whatever is printed about memory"*; and *"A program that watches only standard output learns nothing in either case: in both, the last thing it ever sees is an ordinary sampling line."* **So on a 4.5 or 5.0 out-of-memory kill exactly one stream is empty — the error stream — and the ordinary stream runs on to a sampling line.**
- **Where the specs have it wrong.** `Preparation/Specs/Queue & History.md` states it correctly once — *"Blender 4.5 and 5.0 are killed by the machine, with the error stream left empty and nothing said about memory anywhere… anything reading only Blender's ordinary output sees a sampling line and then nothing more"* — and **wrongly once, further down**: *"kills Blender 4.5 and 5.0 outright with nothing on either stream"*. `Preparation/Specs/Whole app.md` carries the wrong form too, *"leaving both streams empty"*.
- **Nothing built on it breaks.** This row's second quote is unchanged and **E4 answers it correctly**, resting on §11.3's four collection outcomes and `Preparation/Architecture/Rented-server runtime.md` M13's same four, rather than on the sink — so the entry says *printed nothing* only where the record says it collected and found none.
- **What it costs, stated exactly.** On a real 4.5 out-of-memory kill the entry a user meets shows the ordinary stream's words and an empty complaints tab. That is the right behaviour and it is **not** what either spec sentence leads a reader to expect. **The correction is the Creator's.**


**Row 394 — down the side: the latest frame, the finished ones, and auto-download.** *A rendering entry.*
> "**Down the side: the latest frame, with a way to step through the finished ones**, and whether auto-download is on."

Kind of promise: **a drafted design sentence.**

Earns its row by the third test: stepping through *the finished ones* requires **a picture per finished frame** to exist and to have come home; the auto-download state requires a per-job record readable with nothing connected.

Waits on: nothing outside.

**Answered already.** **P1** — *"Production is dense: one derivative per finished frame's chosen candidate … **No finished frame is passed over.**"* — chosen precisely so *"the user moves through a job's finished frames, and a set with holes does not let them"*. **P2** makes the running entry show *"the newest derivative the set holds"*. **P4** holds the fetched copies subject-held on the job record and **cleared when that record is deleted or when the ruling's age period has passed, whichever comes first — corrected 2026-08-22 by the Creator's ruling**, with **P4a** fixing that a copy cleared by age is never fetched again; **P7** keeps a finished job's preview **for as long as that age ending leaves it there**, corrected on the same date and carrying no qualifier before it. So the stepping the promise asks for is dense over the copies that stand ([Section 18](../Queue%20&%20History.md#18-unresolved) item 18 carries the ruling and records that the period itself is not set). The auto-download half is `Preparation/Architecture/Download.md` **R1** — *"An auto-download arrangement is one durable record in the local record, held against the local job record … created when auto-download is switched on — at submission (`Rendering`'s moment) or afterwards from the queue entry (`Queue & History`'s toggle)"* — and **R12** makes it survive every run.


**Row 395 — a running entry never says which parts of the server did the work.** *A rendering entry.*
> "**A running entry never says which parts of the server did the work.** That line is a closing report and does not exist until the job is over — the [spec](../../Specs/Queue%20&%20History.md)'s fact, and the reason the reading appears only in history."

Kind of promise: **a drafted design sentence.**

Earns its row by the third test: the entry has to know that a value exists only after an ending, which is the Blender fact below. **The *not known* wording is not this document's** — the quoted promise says only that the line does not exist until the job is over; saying *not known* rather than leaving the place blank is `Rendering`'s spec sentence, carried by that document's Row 10.

Waits on: a Blender fact, already looked up — that the device line is printed after the last sample and before the frame is written.

**Answered already.** **E12** — *"The device line exists only for a finished job, and the entry says *not known* everywhere else … a job stopped part-way never produces it; every observed result is Cycles' own, so nothing establishes an EEVEE job names anything; and the settings read back afterwards can name a card while the processor did the work, so nothing is ever guessed from them."* **E13** fixes the observation point at the worker's own copy loop and hands the route to `Rendering`'s Rows 10 and 11.


**Row 396 — the five endings, and the fifth that is not an ending.** *History.*
> "**Five endings, and the colour is on the word itself**: green finished, red failed, grey stopped by the user, amber given up. The fifth is **no ending Smoozy ever learned** — a server destroyed before Smoozy looked again — and it is the odd one, because it is not an ending at all but the absence of one."

Kind of promise: **a drafted design sentence** for the list; the grey-italic treatment of the fifth is **Creator-stated** (2026-08-13). `Ending not known` is **a settled value** in the mockup.

Earns its row by the third test: five status words, four of which have to be decided from records and **the fifth of which is a state nothing ever writes.**

Waits on: nothing outside.

**Answered already.** **E2** — *"The entry's four endings map from recorded outcomes and from nothing else. *Stopped* ← the outcome **left stopped** (L11); *given up* ← the outcome **given up** (D12); *finished* and *failed* ← the render kind's own ending, whose classification is `Rendering`'s to define"* — with **E1**'s rule that an ending with no status is failed. The fifth is **C11**: *"an entry whose machine never came back keeps the job with no ending Smoozy ever learned, shown as such, for good"*, and C5 and D2 name the two routes to it. **N7** states what such an entry lacks — no figure, no times past the start, and the actions that read the figure not offered.


**Row 397 — the frame count is the last frame Smoozy confirmed.** *History.*
> "**The frame count is the last frame Smoozy confirmed, not the range the job was submitted with.** A job that failed at 61 reads *60/240*. The entry says the rest."

and
> "**An entry that did not finish cleanly says the last frame Smoozy confirmed, and says the frame the job was on is not confirmed.** Without that figure a stopped job reads as the range it was submitted with and nothing else."

Kind of promise: **a drafted design sentence.**

Earns its row by the third test: a figure computed at an ending, on the machine, that has to survive the machine.

Waits on: **Fact 44** — what losing power outright leaves — unposable and closed unresolved; and C15's limit, that a file short but not empty still counts.

**Answered already, with the limits carried onto the figure rather than hidden.** **N6** — *"The last-frame figure is computed at the job's ending, on the machine, by the ending's own step, and written into the note. For every ending short of a clean finish … **the last frame, in the job's own asked order as the manifest records it, that counts as already there** … The figure is stated in frames, never renderlets."* **N8** carries C15's limit on it. **N7** is the no-figure case.

**The caveat this row carried is spent, and the seam Row 310 opened is closed.** `Preparation/Architecture/Queue & History.md` Section 18 item 17 was closed on 2026-08-21: `Preparation/Architecture/Rented-server runtime.md` §3.3 **S11** has the supervisor run `Job files and outputs` C1 and C7's reckoning and write its statement with the recorded outcome on exactly the two dispositions that end a job with no worker alive — ***left stopped*** written from a durable *should be stopped*, and ***given up*** at the second interruption — **S12** keeps §1.3 rule 5 by taking the walk after the disposition is decided from records alone, and **S13** has an outcome whose walk could not be taken say so rather than carry an empty statement. So N6's figure exists for a reconciliation-written ending too, and *Row 83 has no figure for one* no longer holds.


**Row 398 — the two ways to render it again, each naming its own range.** *History.*
> "**They sit side by side and each names its own range.** A pair of buttons reading *Submit again* and *Carry on* would leave the user working out the difference; a pair naming *240 frames* and *180 frames* does not. That is the same reasoning the settings page already uses for naming the count on its Render control."

and
> "**The carry-on names the frame it starts after, and says where that figure came from.** The [spec](../../Specs/Queue%20&%20History.md) keeps the last frame Smoozy can vouch for as finished, and *carry on* reads the server again before it uses it — so the line says which of the two it is working from. It can be moved: the [Rendering spec](../../Specs/Rendering.md) settles that the user changes the starting frame rather than Smoozy backing off a frame on their behalf, so it is a field and not a fixed number."

Kind of promise: **a drafted design sentence.**

Earns its row by the third test: the carry-on's range is a figure **read from the machine at the moment the action is taken**, and the entry has to say which of two figures it is showing.

Waits on: **Fact 7**, answered — telling a machine that did not answer from a folder that is gone from a login that may not read it.

**Answered already, and the provenance clause is answered in the words the promise asks for.** `Preparation/Architecture/Rendering.md` **A4** — *"*Carry on from where this stopped* opens the page with the range started after the last frame that counts as already there, and the figure is read from the machine when the action is taken — never remembered from when the job ended."* **A9** gives the other of the two: *"the page opens on the figure `Queue & History` kept and says plainly that it could not check the machine and this is what the entry recorded … a remembered figure is never presented as a checked one."* **A5** makes it a field the user types over: *"the page's figure is a starting point the user can type over, shown with what it rests on"*. **A6** covers the two entries that show only *Submit again*, matching the Design document's own two cases.


**Row 399 — work Smoozy is not tracking: one offer per server, and what declining costs.** *The queue list.*
> "**One offer per server, not one per job**, because all of it was left behind by the same event."

and
> "**It says what declining costs**: the work still holds the server, so anything submitted afterwards waits behind it."

Kind of promise: **a drafted design sentence.**

Earns its row by the third test: the offer requires reading a machine for work Smoozy has no record of, and the declining sentence requires the queue to state a fact about jobs it is not showing.

Waits on: the spec's own `[PENDING: whether a machine can be read for the work left on it — an outside fact, unconfirmed, to be settled at Architecture]`, answered by **Fact 40**.

**Answered already.** **A5** — *"The adoption offer is §7 step 6, composed from the reading, one per machine, covering everything at once … Declining writes nothing durable, so the next connection that finds the work offers again."* **A4** is the declining sentence: *"The queue states, from the reading, that the machine is busy with work the user chose not to track — the fact of it, the count of it, never the jobs themselves — so a tracked job sitting at the top of the queue and not starting has a stated cause."* **A3** covers the machine that cannot be read at all.


**Row 400 — what deleting an entry takes and what it leaves.** *History.*
> "**Three sentences, and the control is one word.** What goes: the record, and **Submit again** with it, since the entry is the only thing in Smoozy holding those settings. **The box says *settings* and not *setup***, which is what it said until 2026-08-23: *setup* is this app's word for a server setup, on three other screens, and one word doing two jobs is the vagueness Rule 9 exists to catch. What does not, and the [spec](../../Specs/Queue%20&%20History.md) names both: the frames on the server, and whatever of the job was already downloaded to the user's computer."

*(Re-quoted 2026-08-25 at the Design close re-sweep. **The promise genuinely moved**: two sentences became three, *that setup* became *those settings*, and the box gained a second thing it says is untouched. **The answer stands and the addition was answered before it was written** — X2's enumeration already ends *"It does not touch the job output folder or anything in it"*, and **N18**'s last bullet, written 2026-08-23 for Row 433, is the added clause in the architecture's own words: *"Deleting the entry takes both shortcuts and neither set of files… nothing on the user's computer is touched by any of it. That is the spec's own* what is untouched is named as well *— the frames on the machine, and whatever already came home."* **Nothing is re-decided; the citation is added here rather than in the answer paragraph below, which is untouched.**)*

Kind of promise: **Creator-stated** (2026-08-13) for the paragraph below it that records the withdrawal of the list; the sentences themselves are drafted.

Earns its row by the third test: two claims about what a destructive act reaches, **each of which has to be true of the mechanism or the sentence is a lie the user cannot check.**

Waits on: **Fact 41** — answered in part; its remainder, that the widened erase has never been observed to complete, is carried unresolved.

**Answered already, and enumerated.** **X2** — *"What the machine-side erase reaches is enumerated here, exactly, and it is the record — never the renders"* — with the full list, and then: *"**It does not touch the job output folder or anything in it**: the rendered frames are files of the user's, and the spec says in as many words that deleting history destroys the record and not the renders."* **X1** makes the local removal and the tombstone one atomic act; **X5** keeps a deleted entry from ever returning. **X7** records the check — *"Nothing survives a completed deletion that the spec does not tell the user survives."* — and Section 19 carries it under *Row 82's residue, checked hardest against the third escalation*. The *Submit again* half is **N10–N11** and **C11** — the kept setup lives on the local record and dies with it.

---

## Filed 2026-08-22 by the both-ways check over the Design documents

**The first both-ways ruling any Design document has ever had.** A fresh agent read this document against the rows above and ruled in both directions — what was missed and what does not belong. The rows below are what it found missing. Each carries the checker's own reasons and the constraints it named, because the runner filed them from the verdict and the verdict is not read again. The verdicts are in `Audits/Architecture/Both-ways check/`, consolidated at `Audits/Architecture/Both-ways check/Consolidated — the runner's worklist.md`.


**Row 432 — Blender's own estimate on the *Rendering now* line.** *A running entry.*
> "- **Rendering now** — the frame, the View Layer where a frame has more than one, the samples, and Blender's own estimate at the end of the same line, marked as Blender's. Under EEVEE there is no estimate and that part is simply absent."

*(Re-quoted 2026-08-25 at the Design close re-sweep. **The promise is unchanged**: the bullet's own label went from **What is rendering now** to **Rendering now** and not one word after the em dash moved. R15 answers it exactly as written and does not move. **The row's title was changed to follow the rename on 2026-08-25 by the runner's filing agent** — it read *Blender's own estimate on the* what is rendering now *line*, naming a line the app no longer has. The sweep left the title to the runner and scored only the quote. **The same promise is quoted a second time in `Preparation/Architecture/Queue & History.md`'s head table, and that quotation carries the renamed label too.**)*

Kind of promise: **a drafted design sentence**, resting on an outside fact.

Earns its row by the third test. **Whether the progress stream carries a remaining-time estimate at all, on which of the four versions, and whether it is genuinely absent under EEVEE rather than absent from what anyone has watched, is a Blender fact.** And the entry promises the figure is **marked as Blender's**, which means a figure that is read and never composed.

**The bar it is held to** is Row 387's: a figure on that line, present only where Blender gives one, absent without apology where it does not.

**Why no row reached it.** The word *estimate* does not occur once in Rows 377 to 400. Row 377 takes the View Layer clause of this same sentence and Row 387 the tile count that sits on the same line; the estimate clause sits between them with no row. `Preparation/Architecture/Coverage.md`'s Row 424 entry speaks of *"no estimate of Blender's"* for an unstarted job, which reads as though the started case were established somewhere — **this row is what establishes it or records that it is not.**

**And it moves with a scope correction to Row 386.** Row 386 is filed against the sentence that *formats* the live frame count and the sample pair; the sentence that *promises* them is this bullet, which no row quoted. So the frame and sample readings were answered at R9 and R11 through a typographic sentence while the reading beside them on the same line had no row at all. Filing this puts them on the sentence that makes the promise.

**Answer, 2026-08-23: `Preparation/Architecture/Queue & History.md` [R15](../Queue%20&%20History.md#5-the-reader-of-a-running-render), with the bullet's other three readings already answered.** The fact was looked up rather than asked: `Across topics/The map.md` sends *is there a time estimate* to the joined answer `While a render runs.md`, which names *Reports 5*, and that question was opened and no other.

- **The estimate exists, in all four versions, and is genuinely a field rather than an absence nobody watched.** *Reports 5*: a `Remaining:` field on Cycles' progress lines, between the elapsed time and the memory figure, covering the frame in hand and starting over at each new frame, not on every line — absent from the first sample line, the final sample line and the `Finished` line — in minutes, seconds and hundredths. Settled by tests in all four versions on two machines, which agreed on the spelling, the units, where it appears and what it covers; the versions' own sources document only a tooltip at the window. **EEVEE prints no estimate at all, in every version**, which is the Design sentence's *simply absent* established rather than assumed.
- **What was owed and is now written.** **R15** names the field, marks the reading **observed in a test**, and carries the one-character version split — `Remaining:00:00.05` in 4.5, `Remaining: 00:00.05` from 5.0 — into R2's two grammars, so no rule shared across the boundary reads it. It fixes that a line with no field leaves the reading **absent**, never the last figure held over (R3), and that the figure is shown as printed, marked as Blender's, and fed to nothing — not the bars (B3, B5), not a countdown (B9), not the machine's heading (W1a).
- **Row 387's bar is met.** A figure on that line, present only where Blender gives one, absent without apology where it does not — R15's third bullet is that rule for this figure exactly, on R3's absent-rather-than-wrong ground.
- **The rest of the bullet was already answered and is mapped here rather than re-answered**, which is the scope correction the checker filed with this row: the frame is **R9**, the View Layer where a frame has more than one is **R10** with **B3a**'s per-version bar unit, the samples are **R11**. Row 386 is not edited — its own answer stands and this row now carries the promise sentence.
- **One thing the research could not separate, named in place rather than parked:** whether the figure restarts at a view-layer boundary inside a multi-layer frame. Nothing rests on it — R15 computes nothing from the figure and states no unit beyond Blender's own. `Preparation/Architecture/Sources/Queue & History.sources.md` §3 carries it under *not covered*.

**Row 433 — the two shortcuts to the files, and the one on the user's own computer.** *A history entry.*
> "**Its two shortcuts to the files sit together**: on the server, and on your computer."

Kind of promise: **a drafted design sentence**, resting on a dependency.

Earns its row by the third test. A shortcut to the job's files *on your computer*, offered from a history entry, requires **a per-job record of where that job's outputs landed locally** — readable with nothing connected, and still readable for an entry whose server no longer exists, which is the case the surrounding paragraph exists for.

**What must not be mistaken for it.** Row 394 takes the running entry's side column and its auto-download **state**; `Download` R1's arrangement quoted there is the standing arrangement, not a record of where files arrived. Row 400 enumerates what deleting an entry reaches and names the record and the frames on the server, not what came home.

**Scope.** The server-side half of the same sentence is likewise unrowed, though the document has parked its own edge cases: `[PENDING: how *show outputs* reads when the job has written nothing yet, or its server is gone]`.

**Answer, 2026-08-23: `Preparation/Architecture/Queue & History.md` [N18](../Queue%20&%20History.md#10-the-note-the-figure-and-what-the-read-fills-in), and the record the row asks for already exists in `Download`.**

- **The per-job record of where the outputs landed locally is not new and is not this document's.** [Download](../Download.md#11-where-downloads-land-row-204) **H5** composes a job's set of landing places as a **derived reading over records already kept** — the arrangement's *fetched* entries, each carrying the destination in force when its own act was taken, and the manual request outcome records enumerating under that job's own output folder — **in the order each was first written to, with no new field and no stored list**. **H7** hands the control the set and never a folder — one place and it opens, and **where the set holds more than one the control opens every place in it and picks none**, narrowed to that act on 2026-08-29 from the wider *every place is offered*, which admitted a chooser the spec does not grant; the opening itself is `Desktop app and delivery` §18.6 **K20**–**K24**'s reveal service. **H8** keeps a place that no longer exists in the set and says so; **H9** assigns the control to this document's entry and the rule to that one. That is Row 311's answer, and this row does not re-decide a clause of it.
- **The two conditions this row named are met by lifetime rather than by a rule.** Both records behind the set are **user-held** in the local record (`Download` §12's lifetimes, R1, M1) and neither is a copy of a machine state, so §3 D4 is untouched: the set composes **with nothing connected** and **for an entry whose server row is gone**. N18 states it from this side, because the withholding rule for the entry's controls is this document's.
- **N18's own decisions, and there are three.** The computer-side shortcut is **not a reach-back**, so **N17 never withholds it** — the list of withheld controls is *show outputs*, the whole-job download and carry-on, and this one joins *submit again* and deletion on the other side of that line. **Where the set is empty there are two readings and the entry states whichever applies**, on `Download` **H7a**'s test, widened into N18 on 2026-08-29: where **no** manual entry under the job's output folder published, the entry states that **nothing of the job came home** rather than offering a dead control; where such entries **did** publish, it states instead that **what came home was picked out file by file and went wherever the user sent each of them**, and those destinations are not offered. Both are the spec's own sentences and neither costs a reading. And **deleting the entry takes both shortcuts and neither set of files**: X1 removes the arrangement record with the entry (`Download` R3), X2's erase reaches no job output folder, and nothing on the user's computer is touched.
- **The server-side half is answered where it already was**, and is mapped rather than re-answered: **N10** for what *show outputs* opens — the recorded reading's main output, or the job's own folder where the walk declared none — over **Y8**'s verbatim recorded path, and **N17** for its being withheld where the copied server identifier resolves to no standing row. The Design document's `[PENDING]` is Design's and is untouched.
- **One thing routed, and it was repaired in `Download` rather than left standing.** H5's second source tied a manual request to a job by the source path prefix and nothing else, which reached files the user picked out one at a time in the file browser from under that job's output folder — the case `Preparation/Specs/Queue & History.md` puts outside this control by name. **H5a** replaced the prefix tie with a component-count test over entries that published, so such a request never enters the set; **H7a** gives the control what to say where those are all there is; and **H11**'s third clause records it as that section's own. Nothing in N18 ever turned on the ruling, because N18 consumes whatever set H5 composes.

**Row 434 — a group of scene-jobs read as a group.** *The queue.*
> "**A group of scene-jobs reads as a group**, under its own heading, while each row still acts on its own."

**Re-quoted 2026-08-25 at the Design close re-sweep. The promise genuinely moved: the clause *under its own heading with one action across all of them* lost its middle**, and the reworded promise is narrower rather than wider. **The answer still serves it.** The document did not drop the group act; it moved and split it, three paragraphs down — *"**It offers two acts, not the three the spec lists.** *Cancel all 4* on the heading, and the heading itself as the grip that moves the whole group. **Stopping is not a group act**, because a machine runs one job at a time, so a group has at most one member rendering — stopping *the group* and stopping that one member are the same act, and it already sits on that member's own entry. A third control would be a second name for something the user can already do."* **Those are L19's two acts exactly** — *"Cancel-group … then stops its rendering member … reorder-group is L7"* — and L19's own last sentence, *"any single-member act is the ordinary act"*, is the reworded promise's *while each row still acts on its own* word for word. The Design document's ground for refusing a third act is Q4's exclusive render, which the architecture already holds.

**The answer's contested half is untouched by the rewording and is still what the document needs, and it is kept as it stands rather than smoothed.** The group identity is read off the **local** record and not off the queue — a reading that came back against the checker's own and is recorded as a disagreement rather than reconciled. L20 buys *"a group whose machine cannot be read is still drawn as a group"*, and the document's neighbouring sentence still requires it: *"A server Smoozy is not connected to is greyed and says* not updating*… Its jobs are shown from memory."* A group of scene-jobs on such a server is drawn from memory, with no queue reading to carry a group field. **The reading against the checker stands.**

Recorded rather than filed: the document now also says the group heading is a drag grip — *"Dragging the heading moves every waiting member as one block and keeps the order they already have among themselves"* — which is L7's gather-and-insert, *"keeps their current relative order among themselves"*, and L19's reorder-group. **It is answered where it stands and needs no row.** **The same promise is quoted a second time in `Preparation/Architecture/Queue & History.md`'s head table, and that quotation carries the reworded promise too.**

Kind of promise: **a drafted design sentence**, resting on a dependency.

Earns its row by the third test. Drawing several jobs under one group heading requires **the queue reading to carry the group identity the desktop groups on**, and an action across all of them requires **an act addressed to a set** while each row's own act stays addressed to one job.

**What the architecture already holds**, which is why this may close quickly: Q2's job record carries a *group identity* where Row 380 quotes it, and B1 counts *"per group member separately — a group is several counts, never one"* where Row 388 quotes it. **No row in this file mentions a group at all.**

**The checker's own hedge, carried rather than dropped.** This was the weakest of the three it filed here and it said so: it may well close as *answered already* on Q2 and A1 the moment it is filed. **What earns the number is that the grouping rests on a field of the reading and nothing in this file says so.**

**Out of scope.** `[PENDING: how a group of scene-jobs offers an action across the whole group beside the single-job ones]` is Design's.

**Answer, 2026-08-23: mostly answered already, with one clause added — `Preparation/Architecture/Queue & History.md` [L20](../Queue%20&%20History.md#4-acting-on-the-line). The checker's hedge was right about which clauses hold it and wrong about where the identity is read.**

- **Answered already, and pointed at rather than re-answered.** **Q2** puts the group identity on each machine-side job record beside the job identity, *"so a group is readable off the queue with no group store anywhere"*; **L18** fixes membership in exactly two places, both per-job — the machine's job records and manifests, and each member's **local** job record — and lists what it survives: scattering, a machine-side reorder, the cancel or stop of any member, Smoozy being closed, and the machine going down. **L19** is the act across the set: cancel-group and reorder-group as member acts under one command, with a single-member act staying the ordinary act and leaving the membership untouched — the promise's *while each row still acts on its own*, word for word. **B1** counts per member and `Desktop app and delivery` **T11** speaks once per group, both from the same identity.
- **What L20 adds, and it is the row's own question.** The row asked that *the queue reading carry the group identity the desktop groups on*. **It does not, and it does not need to**: the grouping is read off the **local** job record's copy, joined to the period-held reading by job identity and composed at the draw. That keeps §3 D4 — nothing durable copies a machine state — and it means **a group whose machine cannot be read is still drawn as a group**, its rows carrying C7's stamp or A3's could-not-check condition like any other. **An adopted member is grouped on the identity step 4's fill-in carries from its manifest** (**A7**), which is the one route by which a member's group identity is not already on the desktop.
- **Where the identity comes from is cited, not decided here:** `State, identity and recovery` §2.2 issues it when an all-scenes act is composed, which is `Rendering` **S2**.
- **Nothing is routed and nothing is owed.** The heading's appearance and which acts a group offers beside the single-job ones stay Design's, as the row says.

---

## Filed 2026-08-25 by the Design close re-sweep

**The Design stage finished and `Preparation/Design/Queue & History.md` gained 146 lines and lost 26.** Eight new sections or blocks arrived — the group's acts and dragging, *The job's name, and its folder*, two further marks, the entry's controls, *Clearing history*, *Jobs that leave without an entry*, the keyboard shortcuts — and the whole of *Left for later*, seventeen `[PENDING]` markers, was removed and replaced by *"Nothing. Every marker this document carried is settled."* The four rows below are what the re-sweep found: promises the document now makes that no row quotes and that no architecture clause has been asked about — **Rule 10's silence rather than its disagreement.** None is a mockup value and none is drawing. **All four arrive unanswered.**


**Row 494 — the mark that the job was submitted with no scene named, which survives the read that supplies the name.** *The marks.*
> "**A sixth: it was submitted with no scene named.** Nothing forces a user to read the file first, and a job submitted before the read carries no scene name at all — the [spec](../../Specs/Queue%20&%20History.md) requires the entry to say so, and to say that the job renders whichever scene the file was saved showing. That is a statement true of the job, not a fault and not a warning, so it is a mark like the others."

and, immediately below it, the half that is the finding:
> "**Its wording changes once the scene is known, and the mark does not go.** The file is read when the job starts, so the scene name arrives a moment later and the header can then show it. The mark stays and says which scene it turned out to be — *submitted with no scene named; Blender rendered **Interior**, the scene the file was saved showing.* Dropping it once the name arrived would leave a history entry that cannot answer the one question worth asking about such a job: whether the user chose that scene or Blender did."

Where it came from: `Preparation/Design/Queue & History.md`, under *The marks*.

Waits on: nothing outside. Everything it needs is Smoozy's own record.

Earns its row by the third test, and it is the exact shape N15 was written for. **N11 gives the absence and N10 then erases it**: the local job record is written at submission with *"No scene name — submitted rendering whichever scene the file was saved showing"*, and N10's merge rule is *"Smoozy's own record wins where it carries a fact, and the reading fills what it lacks"* — the record lacks a scene name, N9's recorded reading carries one, and the read fills it in. **After the fill-in nothing on the record tells a scene the user chose from a scene the read supplied**, so the mark cannot be drawn, cannot be reworded to name the scene it turned out to be, and cannot stay into history. The rule the mark is bound by is the one Row 379 already quotes — *"There are four, each shown only when it is true, and every one of them stays on the entry into history"* — now six.

**What has been checked and does not cover it.** N15's two frozen fields are the dependency-check state and the carried-on-from job, and neither is this. N10a's *user's own or resolved for them* field is scoped to the answers a settings payload carries with the Blender version as its named case, not to a scene name the read supplies where the submission carried none. `Preparation/Architecture/State, identity and recovery.md` §7 step 4 fills in *"endings, times, re-run marks, the reason a render failed, and the machine's own words"* and says nothing about preserving what the record lacked. Section 17's persistence list names the disturbed-file and simplify marks, and N15 joined two more; **this is a fifth field on that same list or it is nothing.**

*(Superseded 2026-08-25 — the paragraph below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Queue & History.md` §10 **N22** — the fifth frozen field, exactly the shape this row named. Clause by clause: the mark written at submission is N22's first bullet; *survives the read that supplies the name* is the second — the mark and the scene name are **two fields**, and **N10** fills only one; the rewording that names the scene is the third, two draws composed at the draw; and *stays into history* is the fourth, joining Section 17's persistence list, with **N15**'s closing bullet widened in place to five marks and dated.

**Cited rather than re-decided:** N10, **N11**, **N9**, **N13**, N15's terms, **C10**–**C11** and **X1**; `Preparation/Architecture/Render Settings.md` **F5**/**Y10** for when a submission names no scene; and `Preparation/Architecture/Rendering.md` **L15**/**S6** for the manifest carrying no scene name, which is what an adopted job's fill-in reads.

**Nothing routed. Nothing left unresolved.**



**Row 495 — a job that renders nothing at all reading as the writes-nothing statement.** *What the job writes.*
> "**A job that renders nothing at all is not a seventh statement.** It is the same one as a job that writes nothing, with a different reason named in it. The user is asking one question — *will this job produce any files* — and gets one answer; what differs is why, and the statement has room to say it. A frame with no active view layer renders nothing and writes nothing, and a whole range of those is a job that costs a machine's time for no files, which is word for word what the writes-nothing statement already exists to say. **Making it a seventh would also be a change to the [What a job writes spec](../../Specs/What%20a%20job%20writes.md)**, which owns the six and settled them; nothing here needs that. And the user's act is identical either way — stop, or carry on knowing — so a second shape would buy a distinction nobody acts on."

Where it came from: `Preparation/Design/Queue & History.md`, under *What the job writes*. It replaces the marker `[PENDING: whether a job that renders nothing at all reads as a seventh statement or as the same one as a job that writes nothing]`, which the close cleared.

Waits on: nothing outside; the reading is already taken.

Earns its row by the third test — a decision to take, and it is one this document has just taken on the architecture's behalf. **The writes-nothing statement is one of the two that hold the job**, so what establishes it decides whether a machine is paid to render a range that produces nothing. `Preparation/Architecture/Job files and outputs.md` §9.1's table establishes it from one thing and names it exactly: *"the **render's own output destinations** in 4.2 item 2 — the regular output path and the file output nodes — being empty across every frame"*. **A job whose every frame has no active view layer has destinations that are not empty**, so on that table the condition does not fire, the job is not held, and the statement is not drawn — while the same document's I10 says of exactly this case that *"A frame with no view layer active on it renders nothing and writes nothing"* and that such frames are counted as nothing. **So the reading already produces the fact and the condition does not consume it.**

**What must not be mistaken for it.** Row 391 quotes the six statements' arrival moment and the two that hold, answered at N9 and H1; it says nothing about what establishes either condition. `Preparation/Architecture/Rows/What a job writes.md`'s count rows take I6 to I14 for the per-frame count and stop there. The spec's own sentence at *What a job writes* — *"A job that will write nothing at all, which otherwise costs a machine's time for no files"* — is the wording the Design document is claiming covers both causes, and no architecture clause reads it that way.

*(Superseded 2026-08-25 — the paragraph below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Job files and outputs.md` §9.1 — **H1**'s second condition widened in place with the change dated, plus **H1a**.

**It is not a seventh statement.** H1a keeps the six as the spec's and adds none; what it supplies is the same statement with a different reason named in it, read off item 2's destination set and item 5's per-frame set, with **no new field** and the words Design's. **The condition consumes a reading it already had**: the cell now reads the count of **I12**–**I14** at zero, which **I10**'s no-view-layer frames produce.

**Cited rather than re-decided:** **I6**, **I8**, I10, I12–I14, **I20** (a video counts one, so it is not held — the same outcome as before), **C14** (the other side already calls such a job complete), and **H2** (the count is a field of the recorded reading, so nothing is evaluated twice). **H5**'s triggers are unchanged.

**Nothing routed. Nothing left unresolved.**



**Row 496 — the preview's heading naming the view layer its picture came from.** *The rest of the entry.*
> "**The preview names its own View Layer where the file holds more than one, and says nothing where it holds one.** The [architecture](../Queue%20&%20History.md) P10 fixes the pick and Design does not touch it: the preview is the Combined of the file's **first** view layer, chosen once and never recomputed. What that leaves is a collision on this entry — the progress line names the View Layer being rendered **now**, which after the first is not the one in the picture, so the same words would sit twice on one screen meaning two different things. So the preview's own heading carries the layer's name beside the frame it already names — *Frame 147 · Reflections* — rather than a second line under the picture, which would be the frame number twice. Where the file has one view layer there is nothing to tell apart and the heading names the frame alone."

Where it came from: `Preparation/Design/Queue & History.md`, under *The rest of the entry*. It replaces the marker `[PENDING: which beauty pass the preview shows where one file holds several View Layers]`.

Waits on: nothing outside.

Earns its row by the third test — a fact to get, on the desktop, that nothing carries there. **The pick is answered and the name is not.** P10 settles the pick — *"the first view layer is the pick"* — and `Preparation/Architecture/Job files and outputs.md` B19 makes it a reading by name on the machine: *"The beauty is `Combined` under the name of the view layer it came from."* **So the layer's name is known at the moment the pick is made, on the machine, and no clause records it, carries it with the derivative, or puts it on the entry.** B15 forbids recomputing a pick, and P4's fetch is one fetch per derivative through the transfer contract with no field beside it; the desktop receives a picture. The second half — *where the file holds more than one* — is reachable, from the recorded reading's per-frame view-layer set (`Job files and outputs` I6, which B1 already consumes); **the name is not.**

**What must not be mistaken for it.** Row 377 is the *progress line's* view layer, answered at R10 and B3a, and the Design document's own reason for this promise is that the two must not read as the same thing. Row 394 takes the side column's latest frame and stepper. Row 85, answered at P9–P10, is which pass is previewed, not what the entry calls it.

*(Superseded 2026-08-25 — the paragraph below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting, both halves.** The machine half is `Preparation/Architecture/Job files and outputs.md` §8.4 **B31a** — the name recorded with the production, only where the file holds more than one view layer, and established only where the choosing names a layer: **B19**'s multi-layer name, or the regular output's first view layer by **I9**/**P10**. **A node-fed picture establishes none.** The bytes are under **R4a** and the name is journalled with the production. The entry half is `Preparation/Architecture/Queue & History.md` §15 **P10a** — the heading reads the derivative's carried name, draws the frame alone where none is carried, draws an absence as an absence, and is **never merged with R10/B3a's rendering-now layer**. The wording is Design's.

**One limit falls out and it closes the design's own worry rather than opening one.** Where the preview fell past the beauty rung to a node-fed picture, no layer is established at all, so B31a records the absence and P10a draws the frame alone — and the two-meanings collision the design feared cannot arise there, because no layer name is drawn.

**Nothing routed. Nothing left unresolved.**



**Row 497 — clearing a whole server's history, and clearing all of it at once.** *Clearing history.*
> "**Two acts beside the single delete: *Clear this server's history* and *Clear all history*.** The [spec](../../Specs/Queue%20&%20History.md) names all three. Clearing every server at once is the Creator's addition. Label: **Creator-stated** (2026-08-24)."

and the box the heavier of the two carries:
> "**The two do not get the same box, and the heavier one goes on *Clear all history*.** Clearing one server's history is bounded and countable: the box names the server and says how many entries go, and the user can see the size of what they are about to do. Clearing all of it is the one act on this screen that cannot be undone across every machine at once, and a count alone does not show its reach. So that box lists every server it covers with that server's own count, and says plainly that afterwards nothing in Smoozy shows those jobs were submitted. Both boxes are statements ending in a named button, like every other box on this screen; neither asks Yes or No."

Where it came from: `Preparation/Design/Queue & History.md`, under *Clearing history* — a section that did not exist at `58eea975`.

Kind of promise: **Creator-stated** (2026-08-24) for the third act.

Waits on: nothing outside.

Earns its row by the third test, and the reason no row reached it is a date: **the third act entered `Preparation/Specs/Queue & History.md` on 2026-08-24** — *"They can delete individual entries, wipe a machine's history entirely, or clear the whole of history at once — the third of those added 2026-08-24"* — which is after every row in this file was answered. **The per-server half is answered: X6, *"Wiping a machine's history is the per-entry deletion over every entry for that machine, tombstones and all — one user act, partial across entries, atomic within each"*. Nothing answers the act across machines.** X6 is scoped to one machine because X1's atomic pair and X3's per-machine recorded erase debt are; an act spanning every server is one user act whose machine halves are owed to several machines at once, some of whose rows no longer stand, and no clause says whether it is one unit partial across machines, what it does with an entry whose copied server identifier dangles, or how *afterwards nothing in Smoozy shows those jobs were submitted* holds while erase debts stand unpaid on machines that may never be connected to again.

**And the box's own reading sits against a guarantee.** *That box lists every server it covers with that server's own count* has to be composed from history entries alone, including entries whose server row is gone — and N12 guarantees that *"nothing composes a live server list from entries, nothing matches a row to a server by a label or a hue, and **a removed server is never reconstructed out of an entry**."* Grouping on the copied local server identifier is probably the way through, and *probably* is what a row is for; **nothing today says which side of N12 this falls on.**

*(Superseded 2026-08-25 — the paragraph below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** The per-server half is `Preparation/Architecture/Queue & History.md` §14 **X6** as it stands, **cited rather than re-decided**; the across-machines half and both boxes' listings are §14 **X6a**, with §16's table gaining one row.

**The third stop was tested and not reached: the across-machines act is granted by the spec.** `Preparation/Specs/Queue & History.md` carries *"…or clear the whole of history at once"*, the third of the three added 2026-08-24 and labelled **Creator-stated**. So it is a promise to keep rather than a thing built.

**The row's three questions, answered.** One unit: the clear is **partial across entries and therefore across machines**, each entry being **X1**'s atomic unit, so an interrupted clear leaves what it left standing and is re-takeable. A dangling identifier is deleted like any other — the local half unconditional under X1 and **N17**, with §10.1 and **A5**'s erase debt paid on the tombstone if a machine ever holds the identity again. And *nothing shows* against an unpaid debt holds **at the local act** (**X4**–**X5**, A5, **N20**): the residue is a recorded debt and never a showing.

**N12's question is ruled on the safe side with the reasoning written in.** The box groups on the copied identifier and draws from the copied label and hue — no live list, no row consulted, no server reconstructed — with pre-N12 records grouping on the absent value under **N13**, and the counts composed at the draw and stored nowhere, in **C2a**'s shape.

**Nothing routed. Nothing left unresolved.**

