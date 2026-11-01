# Rows — Design: Render Settings

Rows **366 to 376**, from `Preparation/Design/Render Settings.md`, **Rows 435 to 438**, filed later by the both-ways pass over the Design documents, and **Rows 491 to 493**, filed by the Design close re-sweep of 2026-08-25. Part of `Preparation/Architecture/Coverage.md`; how to read a row is there, and the facts they wait on are in `Preparation/Architecture/Facts.md`.

Every number here is permanent, never reused and never renumbered.

**Filed at the stage close, 2026-08-20**, from the first sweep the Design documents have ever had, on the Creator's ruling of the same day. Every row says what kind of promise it rests on, because a drafted sentence may be reworded when Design resumes and the number stays either way.

**`Preparation/Design/Mockup/Smoozy.html` carries a Render Settings panel in full** — the header and its version dropdown, the scene boxes, the frame field, the View Layer boxes with their diamonds and tags, the samples line, the resolution reading and its unknown state, the engine's three rows, the device pair with its tag, the writes table, the dependency band with its switch-off, and the submission strip. **So most of what this document promises is a settled value rather than a drafted sentence.**

**One promise this document states is filed elsewhere.** The keyframe diamond is stated in `Preparation/Design/Whole app.md` too and is **Row 326** in `Rows/Design — Whole app.md`.

**At the stage close this document carried eighteen `[PENDING]` markers**, in *The dependency warning*, *Submitting* and *Left for later* — one, two and fifteen; the fifteen counted here at first was the last section's alone. **None named Architecture**, so none met the second test; each was a hole in the Design document rather than a promise it makes. Two of them — what the page looks like before Pre-load has run, and what the user sees on reconnecting — sat over machinery this stage has already answered at §2.3 J10 and §1.3 Q9.

**Corrected 2026-08-25 at the Design close: there are now none.** The Design close re-sweep of `Preparation/Design/Render Settings.md` found every one of the eighteen closed and the *Left for later* heading gone, replaced by *What this document left open, and how it was answered*. The paragraph above counted markers that no longer exist and named a heading the document no longer has; it is kept in the past tense as the record of what the first sweep met. **The coverage gate cannot see a staleness of this kind** — it is a count in a preamble rather than a quote — which is why it is corrected by hand and dated.

**One gap left open beside Row 373 is ruled 2026-09-02, and the ruling is that the decline is sustained — no row is filed and this is not to be re-opened.** The both-ways check of 2026-08-22 declined the *three ways on* candidate and recorded, inside that decline, the one part it did not think obviously covered: **what the band does when a check started from it completes while the page is still open**, given Row 373's answer evaluates the warning at the moment of submitting (`Audits/Architecture/Both-ways check/Design — Render Settings.md`, item 5, and carried forward on the runner's worklist). **The architecture answer is untouched by it.** `Preparation/Architecture/Rendering.md` W1 fixes the evaluation at submitting, from the check standing alone, and a completing check changes the standing rather than that rule — so what the band shows in the interval between the two is **ordinary screen behaviour, which is Build's**, not a decision this stage owes. The implementer who raised it was right not to file it either way on its own; the runner files it neither way, deliberately.

---

**Row 366 — the pixel figure beside the resolution percentage.** *The picture size.*
> "**The percentage is the setting; the pixels are a reading beside it.** The [Render Settings spec](../../Specs/Render%20Settings.md) settles that the figure cannot be typed into, dragged or overridden — it moves as the percentage moves and never the other way about — so it is set in the fixed-width face, unboxed, with no control on it."

Kind of promise: **a settled value** — the mockup carries both states: `1920 × 1080` in the reading face beside a typed `50 %`, and `size not known` beside a typed `40000 %`.

Earns its row by the third test in its sharpest form: **the figure moves as the user types the percentage**, so the page has to produce it for percentages nobody has read a file at. That is a value the page shows which nothing has been arranged to give it.

Waits on: **a Blender fact that is not numbered in `Preparation/Architecture/Facts.md` and not posed as a comparison question** — how the pixel size a render comes out at follows from a scene's full pixel dimensions and a resolution percentage: **the arithmetic itself, and its rounding.**

**Not answered anywhere.** Where the sweep looked:

- `Preparation/Architecture/Render Settings.md` §1.1 **Q2 item 1** itemises what the recorded reading holds per scene, including *"the full size in pixels (both axes), the resolution percentage, and the size the percentage works out to"*. **That is Blender's own arithmetic, taken at the file's own percentage at the moment of the read.** Nothing carries a worked-out size for a percentage the user typed afterwards, and **a re-read is not available for it** — Q5 prices a read at 3.2 to 3.8 seconds end to end.
- The same document's other two mentions of the percentage are about something else: §5 **C7** is the *imposition* of a typed percentage on the render — §13 item 24 parked the per-version spelling that pair needs at the sweep and was **closed 2026-08-29**, the spelling `resolution_percentage` written into C7 from *Settings 17* — and §9.1 **Y1** holds the page's box to whole numbers from 1 to 32767. **Neither computes a figure for display.**
- No other architecture document mentions it; a search of `Preparation/Architecture/` for the worked-out size returns only Q2 item 1.

**Answer, 2026-08-21: the arithmetic was already established and the row was right that nobody had written it down. `Preparation/Architecture/Render Settings.md` [Q2a](../Render%20Settings.md#11-the-settings-half-of-the-one-recorded-reading), on `Preparation/Blender Research/Comparison/Settings.md` questions 14 and 15, which `The map.md` names for this.**

- **The rule: each axis on its own — the axis's full pixel size times the percentage, divided by a hundred, the fraction discarded, never rounded.** It is read off four pairs measured **out of written files** in all four versions on both machines: 1920×1080 at 200 % → 3840×2160, at 33 % → 633×356, at 1 % → 19×10, and 4×4 at 32767 % → 1310×1310. **633.6 → 633 and 10.8 → 10 are the two products that rule rounding out.**
- **It is computed on the desktop and asks the machine nothing.** The full size for the scene in hand comes from Q2 item 1's recorded reading, the percentage from the store's entry or the reading's own (J1) — **so typing a percentage starts no read**, which is what respects Q5's 3.2-to-3.8-second price. Q2 item 1's carried figure is not a second source: it is Blender's own answer at the file's own percentage, and the same number this rule gives there.
- **Every whole percentage from 1 to 32767 is producible against a standing reading, and no other value ever reaches the arithmetic**, because Y1's box holds the store to that range and a refused value never becomes an entry. So the silent pull-back to the nearest limit and the outright refusal of a fraction, both measured, have nothing to bite on in a Smoozy job.
- **There are exactly two no-figure states**: no reading in force — before a pre-load and after any of J10's three drops — and a typed value the page has not taken, where the store still holds the last answered percentage.
- **The row expected the mockup's `size not known` to be the problem and it is not.** Design's own sentence gives two unknown cases, and Q2a's second no-figure state **is** the second of them. **Nothing is parked.** What Q2a adds is that the last answered percentage still stands underneath a refused one, so whether the page draws the standing figure or nothing is Design's and both are supported.
- **A zero on an axis is producible and is left as the figure it is.** *Image too small* refuses such a render and writes nothing, measured at 4×4 and 1 % in all four versions on both machines, and **the clause adds no warning beside it because no spec grants the user one** — §14 records that weighed and declined.
- **One limit rides**, at §13 item 25: the rule is read off four measured pairs and is stated in no version's documentation, and no pair poses a product landing exactly on a whole number or an odd full size. **What breaks is a figure one pixel out on an axis — a display fault, never a wrong render**, since what is imposed is the user's own percentage.


**Row 367 — the version list, including a build below the floor.** *The header carries the Blender version.*
> "The version list holds every Blender on the server, including one below 4.5 named as such — which can be picked and is never picked automatically."

Kind of promise: **a settled value** — the mockup's version dropdown carries `4.2.9 — below 4.5` beside three ordinary builds, with the header line reading *matched to the file*.

Earns its row by the third test: the list is not the page's own knowledge, it is **a reading of the machine taken by another module**, and *named as such* and *never picked automatically* are properties of that reading rather than of the drawing.

Waits on: nothing to get. Fact 45's untested placements reach it through `Server Setup` and are carried there.

**Answered already.** `Preparation/Architecture/Render Settings.md` §7.1, **B1 to B5**. **B1** takes `Server Setup` §12's fresh search result whole — *"(path, answered version, long-term-support-or-unstable) per build — asked for when this page wants it, period-held, never durable, shown with its moment and with B11's honesty marks"*. **B2** closes the below-floor half in its own words: *"Builds below the whole-app floor of 4.5 appear in the list, can be picked, and are never the automatic pick."* **B5** covers the machine that holds nothing else and the machine that holds nothing at all; **B6** covers a pick made against a stale list.


**Row 368 — a View Layer's own sample count, said and said overridden.** *View Layers.*
> "- **A View Layer carrying its own sample count says the number**, and says it is overridden where the user has typed one."

Kind of promise: **a settled value** — the mockup draws `1024 samples, overridden` and `4096 samples, overridden` as tags on two View Layer boxes.

Earns its row by the third test: the number is a per-layer file value, and *overridden* is **a claim about what a typed count will actually do to the render.**

Waits on: nothing to get.

**Answered already.** `Preparation/Architecture/Render Settings.md` §1.1 **Q2 item 2** carries *"the layer's own sample count with whether it overrides the scene's"*. §5 **C1 item 4** is what makes *overridden* true: the samples-imposition set sets the layer-samples rule to `IGNORE`, *"the one value that throws every per-view-layer count away in a single act"*. **C6** keeps `Queue & History` B1's per-renderlet count recoverable both ways.

**One named residue rides with it, and it grew on 2026-08-29.** §13 item 10 asks, and does not answer, whether the scene-level layer-samples rule governs EEVEE from 5.0, and states what breaks while it is open — *"a second layer rendering at its own count, silently, at exit 0"* under EEVEE with 5.0 or later. **§5 C3 now carries that split as observed** (*The compositor 9*, added with Row 492): a scene at 64 with its layer overridden to 8 rendered at 64 under EEVEE in 4.5 and at 8 in 5.0, 5.1 and 5.2, on the direct render path. **And item 10 gained a documented sentence that sharpens the break rather than closing it** — the three later versions' manual page says a view layer's override is *always* used under EEVEE, which read literally answers the question against the imposition, and would make the tag's word *overridden* false there. **So *overridden* is established under Cycles and is open in that one corner, with observation and documentation now both pointing at it.** **A second residue was added the same day:** §13 item 29 records that the property C1 item 4's rule is assigned against has no cited spelling, so until one lookup lands every attempt of every typed-sample-count job fails at a failure status with nothing written — loud and total, never a wrong count.


**Row 369 — the typed-samples warning names the file's own values.** *The sample count.*
> "**Typing a number replaces that line rather than adding to it.** The typed line names what the number sets aside — the same settings by the same names, plus any View Layer's own count — and then what that costs: every frame now renders the whole count, so the job can run longer than the file's own setup would have, and the user pays for the extra time. Two lines saying *this file stops early at 0.01* and *you have just switched off stopping early at 0.01* are one fact twice."

and, the sentence *the same settings by the same names* refers back to, which is where the file's own two numbers are actually stated:
> "**What the file was going to do about stopping short is said either way.** The [spec](../../Specs/Render%20Settings.md) shows the file's early-stop or time-stop setup whether or not the box has been touched, because whether a file stops itself short changes what a job costs on a machine charged by the hour. So a line under the box names Blender's own settings and their values — *This file stops early at noise threshold 0.01, and after a time limit of 30s on each frame.* Where the file carries neither, there is no line."

Kind of promise: **a settled value** — the mockup's hint reads *Overrides noise threshold 0.01, time limit 30s, and FX's own 1024.*

Earns its row by the third test: **the line quotes two of the file's own numbers back at the user**, and neither is a setting this page offers, so nothing about the page's own controls supplies them.

Waits on: nothing to get.

**Answered already.** `Preparation/Architecture/Render Settings.md` §1.1 **Q2 item 1** carries, per scene, *"the early-stop setup (whether adaptive sampling is on, its threshold and its minimum); the time limit"* — the threshold with its value and the limit with its value, which is what the line reads. §5 **C1 items 2 and 3** are the pairs that make *overrides* true, and §13 item 10's table places each disabler against what *Settings 16* establishes it disables. **One thing rides with those two pairs, added 2026-08-29.** §13 item 29 records that the property names C1 items 2, 3 and 5 are assigned against have no cited spelling — *Settings 16* answers in the manual's interface words and carries no property name — so until one lookup lands, every attempt of every typed-sample-count job ends at a failure status with nothing written. **What the line *says* is unaffected**, both in the untouched state and the typed one; what is conditional is the imposition the line describes.

**Re-quoted 2026-08-25 at the Design close, and the row's *where it came from* moved with it.** The *Warnings* bullet this row used to quote was emptied and now only points elsewhere; the promise moved into the new section *The sample count* and **widened** — the file's early-stop and time-stop setup is now stated **whether or not the box has been typed into**, with the typed line replacing that setup rather than joining it. That is why the row's heading names *The sample count* rather than *Warnings*, and why the row now carries two strings rather than one. **The answer is unmoved and covers the wider draw as written**: Q2 item 1 carries the early-stop setup and the time limit per scene, which is what both lines read, in both states, with no read added and nothing new asked of the reading; C1 items 2 and 3 remain what makes *sets aside* true. **Nothing here reopens.**


**Row 370 — the engine control's third row, the file's own engine by name.** *The render engine.*
> "**So the control has a third row that is not a choice.** Above Cycles and EEVEE sits the file's own engine, by name, marked as the file's in the same italic grey every other file value uses — and it is what the setting is on until the user picks one of the two. Picking Cycles or EEVEE is an override like any other and carries the cross that hands it back."

Kind of promise: **a settled value** — the mockup draws three rows, the first reading *Octane — the file's* in the file's face with the two offered engines beneath, and a note reading *Leave it alone and the render uses what the file named.*

Earns its row by the third test: the third row shows a name that **can only come out of the file**, and *leaving it alone renders with what the file named* is a claim about what the submission does when the control is on a row that is not one of the two the payload can carry.

Waits on: nothing to get.

**Answered already.** `Preparation/Architecture/Render Settings.md` §6.1, **E1 to E4**. **E4** is the third row itself: the page shows *"the reading's `engine_as_read` per scene … plus the file's own engine name where it is neither of the two offered, per the spec and the Design document's third-row control."* **E3** is *leaving it alone*: *"An untouched engine is no entry, and no engine argument is composed"*. **E1 and E2** keep the picked engine's identifier the chosen version's own bytes, and fail the attempt where the reading in force cannot anchor it.

**The honest limit the answer carries** is the spec's own and is not this row's to move: **the name shown is the file's engine as the chosen Blender read it**, so a Blender lacking that engine reports EEVEE and the third row shows EEVEE. §13 item 5 records that the case was never posed.


**Row 371 — how much the job reports to the phone: the default it starts from, and the field it travels in.** *Submitting.*
> "**How much the job reports to the phone is chosen here too**, beside the download choices, because it is settled at the same moment and for the same job. It belongs to [Telegram](../../Specs/Telegram.md)."

and, from `Preparation/Design/Settings.md`, under *Telegram*, which is where the value it starts from lives:
> "The choice here is the default; the [Render Settings](../../Design/Render%20Settings.md) page settles it per job."

Kind of promise: **a drafted design sentence**, both. This page carries a `[PENDING]` beneath its own sentence for how the choice reads, and the mockup's submission strip draws no phone-reporting control.

**One decision, two documents, one number.** Both sentences are unanswered on the same thing — a value with no stated home — and the answer is one decision about where that value lives from the default through to the machine. Merged at the consolidation.

Earns its row by the third test: **a choice made on this page has to travel with a submission and be readable on the machine by a worker with nobody connected**, which is exactly *being told something at a moment nothing currently tells it*; and the value it starts from has to exist somewhere before the page opens.

Waits on: nothing to get.

**Partly answered.** `Preparation/Architecture/Telegram.md` §15 **V1** answers the requirement whole: *"Per-job reporting choices are fixed at submission and travel with the submission, durable on the machine for the job's life, read from the job's submission-fixed record."* **V2** answers the machine-level half — the idle switch travels by the told-record. Row 271's answer records the split cleanly and ends *"the one list is Design's."*

**The part that is not answered is where they ride, and where they start.** **V1 itself records the first half**: the immutable submission manifest is the natural home, and *"that `Job files and outputs` §3.1's field list does not name a reporting field is a seam recorded at Section 21 item 12 rather than an edit of a closed document"* — with the job record of `Queue & History` Q2 as the other candidate and both documents closed. `Preparation/Architecture/Render Settings.md` §9.1 **Y1**'s payload table names no reporting entry either. **And the second half is the Settings sentence's:** `Preparation/Architecture/State, identity and recovery.md` §1.1 puts *settings* in the local record generically, so there is a place for an app-wide default to live, but **no architecture document names this setting, says it is user-held, or says that a submission reads it.** So the mechanism for carrying the choice exists; the setting it starts from and the field it rides in do not.

**Answer, 2026-08-21: both halves, at `Preparation/Architecture/Telegram.md` [V1a](../Telegram.md#15-reporting-settings-and-how-they-travel), which picks between the two homes V1 named and left open rather than editing a closed document.**

- **The field rides on the immutable submission manifest, as a field of its own beside the settings payload and never inside it.** The settings payload is `Render Settings` Y1's statement of overrides, **every entry of which must be deliverable to a Blender inside the launch window** (Y3), and a reporting choice reaches no render.
- **`Queue & History` Q2's job record is refused on one ground**: V1 reads from a **submission-fixed** record, and that record's queue state is written after submission where the manifest is written once and never rewritten (M2).
- **The field carries one value from the closed set V2's one list names, held as an identifier and never a wording, and is present on every job's manifest** — whether or not the user touched the choice and whether or not Telegram is on anywhere — **so the role worker always reads a value and never an absence.**
- **The value it starts from is one item of the local record's *settings***, a class `State, identity and recovery` §1.1 already names, so **no store is added and no inventory line is owed.** It is user-held and ends only when the user changes it. **A submission reads it once, at composition**; a later change never moves a submitted job and never reaches a machine, V2's told-record carrying the machine-level switches alone. Where the page offers no per-job control the field carries that value alone.
- **The one line that was owed in a closed document has since been written.** §21 item 12, rewritten on 2026-08-21 to carry the decision and the debt, was **closed 2026-08-22 at the Architecture seam close**: `Job files and outputs` §3.1 **M1**'s field list now names *the per-job reporting settings fixed at submission (`Telegram` V1)*, and **M1a** states why the manifest is the vehicle — the one per-job material written once and never rewritten, which is V1's own requirement. Nothing is owed and nothing is parked.


**Row 372 — the tag saying the server's cards will be idle.** *What the render runs on.*
> "**Where the scene says CPU on a server with GPUs, the CPU option carries a tag and nothing else is said.** *2 GPUs unused*, beside the option, in the same small tag a View Layer's own sample count uses."

Kind of promise: **Creator-stated** — the document labels the paragraph beneath it *Creator-stated* (2026-08-13), and the mockup carries `2 GPUs unused` as a tag on the CPU option.

Earns its row by the third test at its sharpest: **the tag states a number about the machine's hardware, on a page whose every other value comes out of a `.blend`.**

Waits on: **Fact 46** — the count, the per-card identity and the per-card memory read over the connection on a machine with no Blender. **Answered in part**: the count half is closed on a two-card box; the no-card half is unanswered and was established unposable on the provider tested.

**Answered already.** `Preparation/Architecture/Render Settings.md` §8.2, **D5 to D7**, on `Servers` §15's hardware reading. **D5** makes the tag show only where both halves stand — the scene's device value, which needs the read, and the remembered hardware reading, which needs no Blender. **D6** writes the answer **across both of `Servers`' axes at once**, in a table of **six rows** against *live* and *last seen* — `Servers` H11's four card states carried over five rows, plus H12's container case, the sixth row added 2026-08-23 for a machine that is not a container and on which the bus scan has no route either — with *never read* showing nothing in any state, and states that **no count is shown in either cannot-be-read state**, since `Servers` H5 is explicit about what may be read as a count.

**The answer is wider than the tag drawn**, and it is worth Design's eye: the document draws one tag in the present tense with a count in it, and **D6 requires a *last seen* face with the reading's moment, and no count at all in two of the six states.** That is material for Design rather than a gap in the answer.

**Discharged on the Design side, 2026-08-29 — Design has carried the Rule 10 send-back.** The Design close re-sweep of 2026-08-25 found the sentence unchanged since 2026-08-20 — one present-tense tag with a count in it, and *and nothing else is said* closing the door on anything more — and it went back to Design gathered at `Audits/Architecture/The ninth sitting — what goes back to Design.md`. `Preparation/Design/Render Settings.md` now draws the second face beside the quoted sentence and cites D6 by name: a *last seen* face carrying the reading's moment and never the present tense, presence stated with no count in the two cannot-be-read states, and no tag at all where the reading says no cards or could not establish either way. **The drawing moved to the architecture's answer rather than the other way round**, so the answer does not move, the row does not reopen, and it does not change bucket. Kept in the past tense as the record of what the send-back was.


**Row 373 — the dependency band's two sentences.** *The dependency warning.*
> "**Three sentences, and which one depends on why the warning is there.** Nobody has checked this file on this server; or a check was run and the file has been replaced since; or a check was run with a Blender this job does not use. The [spec](../../Specs/Rendering.md) is firm that these must not read alike — the second and third are the ones that matter and would otherwise be the sentence the user has clicked past twenty times. So the replaced case says plainly that the new upload put the warning back."

Kind of promise: **a settled value** — the mockup draws the band with the replaced case in it: *This file's dependencies were checked on 12 Aug, and it has been uploaded over since.*

Earns its row by the third test **in the exact shape it exists to find**. Neither sentence can be drawn from anything on this page: the first needs a record of every check ever run against this file on this server, and the second needs the app to **notice that the file was replaced under a check that stands.**

Waits on: nothing to get.

**Answered already.** `Preparation/Architecture/Rendering.md` §5, **W1 to W3**, on `Dependency Upload & Remap` §11's V10 to V14. **W1** evaluates the warning *"at the moment of submitting, from the check standing alone, with no read of the file and no read of the machine"*. **V14** is the distinction the Design document draws: *"**Nobody has ever checked this file on this machine** and **a check was run and the file has been replaced since** are two different record states, not two absences."* **W3** fixes what counts as *replaced* — Smoozy's own recorded act, never a comparison of what sits on the machine, on the observed ground that a killed transfer leaves a piece of the file under the final name with no marker.

**Architecture carries three record states where the document draws two.** W2's third is *"a standing that is complete for this file on this machine but keyed to a Blender version other than the one the submission names"* — a check that stands and does not cover the Blender this job will render with. **It is a record state rather than an absence, and W2 states outright that the warning is never allowed to say nobody checked where one stands.** The Design document's *two sentences* has no place for it and neither has the mockup's band. Answered, and then some.

**That note is discharged, 2026-08-25 — spent rather than wrong.** The Design close re-sweep found the document now drawing exactly three sentences, the third being W2's third record state arriving in Design a week after Architecture wrote it, and the row's quote above is re-quoted to it. **The promise genuinely moved, and it moved onto the answer** — which is Rule 10 working and is why the note is marked discharged rather than deleted. The third case is answered whole and nothing about it is owed: `Preparation/Architecture/Rendering.md` W2 supplies the record state and W1 the key it is read part by part from; `Preparation/Architecture/Dependency Upload & Remap.md` V13 supplies the trigger — a submission naming a Blender other than the standing's is not covered by it — and V10 carries the version the check read with as a field of the standing. **W4a's one identity already covers the band in all four of its statements**, W2's three record states and W4's replacement, which is the same four the document now draws. **The row does not reopen and does not change bucket.**


**Row 374 — switching the warning off across the whole app.** *The dependency warning.*
> "**Switching it off is on the band, and it says what it does not cover.** It switches the warning off across the whole app, not for this file — and the [spec](../../Specs/Rendering.md) settles that restoring it later brings back the switched-off warning and not the case where Smoozy believes the file was checked. So one line: *Don't warn me about this again*, and beneath it, where it is restored."

Kind of promise: **a settled value** — the mockup carries the line: *Don't warn me about this again — restored in Settings, with the rest.*

Earns its row by the third test: **a switch on this band changes what a different page does, and it has to survive the app closing**, which nothing about drawing a band supplies.

Waits on: nothing to get.

**Partly answered, and the missing part is a declaration rather than a mechanism.**

**What is answered:** `Preparation/Architecture/Desktop app and delivery.md` §15, **G1 to G9**, on `State, identity and recovery` §10.3. **G1** keeps the rule structurally — no feature module has a durable store of its own, so *"a module that wanted to remember its own in its own way has nowhere to write it"*. **G4** makes a row *"the module and the warning identity, and nothing else"*. **G9** makes suppressions user-held, ending only when the user removes them. The restore control is Row 290's, one delete over that table, and L11 forbids a migration from removing a row or rewriting an identity. The Design document's *restoring it brings back the switched-off warning and not the case where Smoozy believes the file was checked* **holds by construction**: the suppression row is a different object from the check standing that `Rendering` W1 evaluates the warning from.

**What is not answered.** **G3** binds every later document — *"Every dismissible warning in every module, present and future, records its suppression here"* — and **A7** makes it a completeness test: *"A module document that does not declare its warnings' identities has not finished."* **`Preparation/Architecture/Rendering.md`, which owns this warning at its §5, declares no warning identity for it**; a search of that document for a suppression declaration finds nothing, and its §5 W4 hands wording and presentation to Design without naming the switch-off at all. **So the table exists and the row that goes in it is not named.** That clause is the finding.

**Answer, 2026-08-21: the identity is declared, at `Preparation/Architecture/Rendering.md` [W4a](../Rendering.md#5-the-missing-assets-warning) — `rendering.missing-assets`.** That is the declaration G3 requires and the completeness test A7 sets, and this document did not meet it before.

- **One row, carrying this document and that identity and nothing else** (G4), stable for the life of the application (G5), cleared by Row 290's one control and never rewritten by a migration (G6, L11).
- **One identity covers the whole band in all four of its statements** — W2's three record states and W4's unfound replacement. The spec makes all three states dismissible in one sentence, the `Dependency Upload & Remap` spec says the specific warning *"can be dismissed, exactly as the generic one can"*, and **the unfound case replaces rather than stands beside.** Two identities would let a user who switched it off meet it again under its fourth sentence, **which no spec grants** — so a second identity is refused on spec evidence rather than on taste.
- **The design's *not for this file* is kept structurally**: G4's row carries no per-subject qualifier and no path writes one, so the switch-off is across every file and every machine by construction.
- **Restoring brings back the warning and never a standing.** A suppression withholds the warning and touches no record, and the suppression row is a different object from the check standing `Rendering` W1 evaluates — which is the design sentence's own second half, kept for nothing.
- **The first-frame check is unaffected by a suppressed band.** It is settled at submitting in its own right (§4.4); the band only puts it in front of the user earlier.
- **No limit rides and no unresolved item is filed.**


**Row 375 — what the file will write, stated on this page.** *What it will write.*
> "Once the file has been read: each destination, how many files, and **which format each writes**. Then the total, split by format. A total that says only "720 files, EXR and PNG" answers nothing, since it does not say whether that is 720 of each."

Kind of promise: **a settled value** — the mockup draws the whole table, four destinations with per-destination counts and formats, then `720 files` split as `480 EXR`, `120 PNG`, `120 JPEG`, with the duplicate-picture line beneath it.

Earns its row by the third test: **every figure in the table is a fact about a `.blend` nobody has opened**, and the split by format is arithmetic over a set nothing on this page holds.

Waits on: **Fact 5** — whether Smoozy can drive a read of the file's destinations over the connection it holds. **Answered**, on all three payloads.

**Partly answered.**

**What is answered:** the destinations and the count shown before submitting are Row 122's, and `Preparation/Architecture/Job files and outputs.md` §4.3 **R7** answers it structurally rather than by keeping two computations in step — the destination map is computed **once per job, on whichever side ran the reading**, so the folders the user was shown and the folders the render writes into are the same object. §7.3 **I12 to I14** fix the count as the number of the expected-output inventory's entries, worked out frame by frame. The duplicate-picture line is §9.3 **H8**'s fourth of six, *"that the same picture is being written to more than one place"*, and H8 states that all six are fields of the recorded reading. §9.2 **H5** goes further and takes this page's showing for granted: the writes-nothing and colliding-pair conditions hold a job only where the map was computed on the machine, *"because where the file was read before submitting the user was already shown those two and answered by submitting."*

**What is not answered, and it is two things.**

- **Which format each destination writes, and the total split by format.** §7.3 **I14** itemises the inventory entry — *"the predicted path, the frame it belongs to, which route produced the name, and the destination that aims at it with its mapped folder"* — and **there is no format on it.** The only place a format is reached is `Preparation/Architecture/Download.md` §6 **N5**, which gets there through the destination in the recorded reading and states the mapping for **narrowing** alone: *"The match is against the inventory by predicted path, never against the file's name."* **No clause anywhere produces a per-format count, which is what this page draws.**
- **The route by which any of it reaches this page.** `Preparation/Architecture/Job files and outputs.md` §13 names what it supplies and to whom — `Rendering`, `Dependency Upload & Remap`, `Queue & History`, `Download`, `Telegram`, `Server Setup`, `Rented-server runtime`, `Security and licence`, `Desktop app and delivery` — and **there is no clause supplying `Render Settings` at all.** From the other side, `Preparation/Architecture/Render Settings.md` §15 disclaims it: *"The recorded reading's invocation, its destination payloads, the map, the identity, the staleness rule, the manifest, the inventory, the count, completeness — `Job files and outputs`"*, and its §12 tells `Download` only that *"The count and the folder come to it from `Job files and outputs`, not from here."* **So both documents have the material and neither routes it to the screen that draws it.**

**Answer, 2026-08-21: both halves, at `Preparation/Architecture/Job files and outputs.md` [I14a](../Job%20files%20and%20outputs.md#73-the-count), and it needed no new field and no new route.**

- **The format goes on the destination in the recorded reading** (§4.2 item 2), one per destination, taken by the same reading that already takes I7's single-file mode — the field `Download` N5 already matches chosen formats against, and the one I1 and I3's recipes take a name's ending from. **No format is copied onto an inventory entry**: an entry already names its destination, and a second copy would be a second mapping for N5's to disagree with.
- **The split by format is a count of I14's entries grouped by the format their destination carries.** Every entry falls in exactly one group and the groups sum to the count (I12, I13), **so a collision is counted once in the split exactly as in the total.**
- **The per-destination figure is not the total's working.** An entry naming two destinations is one file in both figures, **so the per-destination figures may sum above the count and the count is never derived by adding them** — which is I14's own *nothing keys the inventory per destination*. A video destination's entry counts one with its *may grow* mark, and §5.5's added write places are in no count and so in no figure.
- **The route the row found missing is that there is none to build.** Where the file was read before submitting, **the desktop composed the reading, the map and the inventory itself** (R7), so the page states figures over the object it made — nothing recomputed (R9), nothing asked of a machine. Where no reading stands the page states none of it, which is `Render Settings` Y7's case. §13 gains the supply paragraph nothing anywhere carried, and **`Render Settings` §15's disclaimer stands unchanged**: taking a supply is not owning one.
- **Material for Design rather than a park.** The mockup's writes table draws four per-destination counts summing to `720 files`. **Where a collision stands they will not sum**, by I14's own rule, while the split by format always sums to the total. How the page reconciles two figures that are different objects is Design's, and I14a says outright that they are.
- **One limit rides**, at §14 item 20: a colliding pair whose two destinations carry different formats — possible where the artist switched the ending off — is grouped deterministically under the first destination the entry records, and **which format survives is not decidable before the render** (I16). **What breaks is one file in the wrong group of a screen figure**, only for a pair the collision record already names and that already holds the job.


**Row 376 — the three download decisions, settled at this moment.** *Submitting.*
> "**Downloading as it renders is three decisions, not a checkbox.** On or off, where the files go, and which formats. All three are chosen here because this is where the user is and where the file has just been read — the format list can only be built from a file that was read, and this is the moment it has been."

Kind of promise: **Creator-stated** (2026-08-13) for the paragraph beneath it. The first is **a settled value** — the mockup draws the checkbox, the summary button reading *EXR and PNG — 600 of 720 files, to ~/Renders/kitchen*, and the modal behind it with per-format counts and a destination picker.

Earns its row by the third test: the format list is a fact about a file that has just been read, the destination has to be **carried away from this page and honoured for the life of a job nobody will be watching**, and *cannot be asked about later* is a claim about what happens with nobody there.

Waits on: nothing to get.

**Answered already.** `Preparation/Architecture/Download.md`:

- **Where the files go, settled here.** §2 puts on the arrangement *"the **destination**: the folder chosen or accepted when the arrangement was made, fixed for the life of the job (the spec's own rule — nothing can be asked of a user who is not there), and moved only by the user switching the arrangement off and on again, which resets it and asks once more"*.
- **The format list and the moment it exists.** §6.1 **N2** — *"Where the reading ran at pre-load, the material is already on the desktop side from that moment"*; where the job was submitted unread the machine-side reading lands at render start, and *"Until it lands, there is no list and nothing to narrow with."* §6.2 **N3** keeps an arrangement made with no list covering **everything**, *everything* being no filter rather than a frozen list, and offers the narrowing from the queue entry later.
- **The on-or-off switch turned later.** §2.4 R11's switch-off-and-on: off flips the desired state and removes nothing; on again resets destination and narrowing and keeps the per-file reckoning.

---

## Filed 2026-08-22 by the both-ways check over the Design documents

**The first both-ways ruling any Design document has ever had.** A fresh agent read this document against the rows above and ruled in both directions — what was missed and what does not belong. The rows below are what it found missing. Each carries the checker's own reasons and the constraints it named, because the runner filed them from the verdict and the verdict is not read again. The verdicts are in `Audits/Architecture/Both-ways check/`, consolidated at `Audits/Architecture/Both-ways check/Consolidated — the runner's worklist.md`.


**Row 435 — an untouched View Layer box is an evaluation at one frame, and the frame moves with the range.** *View Layers.*
> "- **A box the user has not touched is a reading of one frame** — the lowest the range covers — and its label is italic."

and, the clause the checker called the thing that stops a lie:
> "- **A View Layer whose renderability is animated in the file carries Blender's keyframe diamond**: solid where it is keyed on the frame being read, hollow where it is animated but not keyed there."

Kind of promise: **a drafted design sentence**, resting on the recorded reading and on a decision about where a per-frame evaluation is taken.

Earns its row by the third test, in Row 366's shape. **The box's untouched state is not a field of the file; it is the file's renderability evaluated at one frame**, and the frame is chosen from the frame field, which the user edits without a re-read. So either the recorded reading carries per-frame renderability across the whole range, or the page recomputes it from something the reading holds, or the box states the wrong frame the moment the range is edited. Row 366's answer establishes that a re-read is not available for it — Q5 prices a read at 3.2 to 3.8 seconds. *Animated but not keyed here* is a third state beyond on and off, and it is per frame.

**The clause that makes the decision non-obvious.** *The lowest the range covers* is a **minimum over the frame set**, and the frame set may be a written list rendered in the order it was written and never sorted (Row 134, from *Frames 10*, as `Preparation/Architecture/Coverage.md` records under Row 310) — so the lowest frame and the first frame are different values, and a builder taking the first would be wrong on exactly the case the frame field exists to allow.

**What Row 326 covers and what it does not.** `Rows/Design — Whole app.md` Row 326 holds the diamond, answered at `Preparation/Architecture/Render Settings.md` 1.1 Q2 item 2 and 2.2 Q3, with 4.1 V1 establishing that one read serves any frame later asked about — so **the per-frame material exists**. Row 326 does not reach **which frame the untouched box states**, nor the lowest-against-first hazard. *(The both-ways checker could not open that file and asked the runner to establish this; it was established on 2026-08-22 and the row is narrowed accordingly.)*

**Answer, 2026-08-23: both quoted clauses are answered at `Preparation/Architecture/Render Settings.md` [§4.1](../Render%20Settings.md#41-what-one-read-answers-per-frame) V11 and V12, out of material the recorded reading already carries and out of a tidy F1 already performs. Nothing needed a read and nothing needed a fact.**

- **Which frame the untouched box states — V11's first bullet.** The numerical minimum of the frame set the page has in hand, taken from the one parsed form F2 already names the boxes' reader of: the entry F5's composition would read at this moment — J3's group entry where more than one scene is ticked, else that scene's own, else the pre-read answer set — and, where no entry stands, the frame start of the reading in force for the scene in hand (Q2 item 1, F4).
- **The lowest-against-first hazard — V11's second bullet, and it is closed at the parse rather than at the box.** F1 stores a written list as the tidied **ascending** list, so its lowest frame is its first element and the two cannot differ; the written order never survives the parse, and no page ever reads this off the user's text. A triple's lowest is its first integer, the step counting forward from it.
- **V11's third and fourth bullets** carry the two cases a builder would otherwise assume: a first frame may be zero or negative, so the minimum is signed-integer arithmetic; and where no reading stands there are no boxes at all, so a lowest frame with nothing to evaluate at does not arise.
- **What the box reads at that frame — V12's first bullet.** Q3's answer: the stepped table inside the evaluated span, the extrapolation constant outside it on the side that has one, the curve data inside the key span but outside the table labelled computed, and ***not known without a read*** where Q3 has no answer — a driven or script-driven switch outside the scene's own frame range, which §13 item 7 already carries. **A box never states another frame's answer as this frame's.**
- **The diamond — V12's second bullet.** Computed from Q2 item 2's own material and nothing else: no animation, no diamond; **keyframed**, solid where the stated frame is exactly one of the curve's key frames the reading carries and hollow at every other frame, an equality and never a nearest-key or a tolerance; **driven or script-driven**, animated with no key frames at all, so the mark is hollow at every frame it is drawn at. Which mark a driven switch should carry is Design's; the material is this.
- **The row's own tension resolves the first way it names:** the recorded reading carries the per-frame material across the span, plus the curve and the constants, so the page recomputes for a new frame with **no read** — V12's preamble, on V1 and Q5's 3.2-to-3.8-second price, with a range edit not being one of J10's three drops. **Row 326's diamond answer is untouched and is cited rather than re-decided.**
- **Nothing was routed.** §14 records the escalation check for V11–V12, declined.

**Row 436 — the frame count, and what a repeated frame does to it.** *The frame field, and Submitting.*
> "**The frame count sits beside it**, which is what shows the user they got what they meant."

and, under *Submitting*:
> "and the Render control naming how many frames"

Kind of promise: **a drafted design sentence**, resting on a decision with money on the wrong answer.

Earns its row by the third test. The calibration file's **Row 14** settles that the frame set reaching the machine is an explicit list emitted once per frame in the asked order, that a malformed list renders nothing at all and still reports success, and that **a frame named twice is rendered twice — *money spent on a frame already had***. The field on this page accepts `1,5,9` and a mix, so a repeated frame is producible by typing.

**What is open.** Whether the count beside the field counts a repeated frame once or twice, and whether it is composed from the same object the submission is composed from. **The same figure is drawn twice on this page**, beside the field and on the Render control, and it is the figure the document says shows the user they got what they meant.

**What the answer owes:** a decision about what the count counts, and a dependency that the drawn figure and the submitted set are one object rather than two computations that can disagree.

**Answered already, 2026-08-23, by clauses that stand — `Preparation/Architecture/Render Settings.md` [§3](../Render%20Settings.md#3-the-frame-range-and-the-tidy) F1, F2 and F5. Nothing new was written, because both things the row says the answer owes are already decided sentences.**

- **What the count counts — F1.** *"A written list is held as the tidied list — repeats dropped, ascending order — of integers"*, parsed once at the moment the text is accepted. **So a repeated frame is counted once and rendered once**: the repeat never survives the parse, never becomes a store entry, never composes into a payload, and never reaches a machine. Calibration Row 14's money — *a frame named twice is rendered twice* — has nothing to bite on, which is the spec's own rule (*every frame the list covers is rendered once, however many times it is named*) and `Rendering` L18's *no frame appears in it twice*.
- **The one-object dependency — F2, in its own words.** *"The tidied list and the counted list are the same object. The page's frame count, the view-layer boxes' lowest frame, and the payload's range entry all read the one parsed form; there is no second parse and no second count anywhere."* The figure beside the field and the figure on the Render control are that one count, so the two drawings cannot disagree with each other or with the submitted set.
- **The group case — F2's second half with F5.** With more than one scene ticked the count is the group's — the group entry's count once for every ticked scene where a range was typed, each ticked scene's own range counted and added where none was — **and F5 composes each job's range entry from the same entry the count read**: J3's group entry where it stands, else that scene's own, else the pre-read answer set, else no entry at all, where F4 makes the job's frames the file's own.
- **Nothing was routed and nothing is unresolved for this row.**

**Row 437 — what Blender would otherwise have done silently.** *A number Smoozy will not take.*
> "- **A number Smoozy will not take** turns the field red and says the range, and says what Blender would otherwise have done silently."

Kind of promise: **a drafted design sentence**, resting on an outside fact.

Earns its row by the third test. *What Blender would otherwise have done silently* is **a per-setting statement of Blender's own out-of-range behaviour** — pulled back to a limit, refused, or taken and acted on — and it is stated for **every** number the page refuses, not one.

**The tension the answer must resolve, and it is the reason this is not a wording row.** Row 366's answer establishes exactly two of these for the resolution percentage alone, and states that Y1's box means they have nothing to bite on in a Smoozy job. **That is the opposite of what this sentence promises**: this sentence promises the page tells the user what would have happened had Smoozy not held the box, which requires the behaviour to be established for each refused number. Nothing in Rows 366 to 376 carries it, and the samples box, the frame field and the percentage box are three different refusals.

**What the answer owes:** a fact to get, per setting and possibly per version, with the honest possibility that some of it is already measured and only needs pointing at.

**Answer, 2026-08-23: answered in two of its three places and unresolved in the third, at `Preparation/Architecture/Render Settings.md` [§2.4](../Render%20Settings.md#24-what-the-page-refuses-and-what-a-refusal-states) J11 and J12, with §13 item 26 carrying what breaks. The row's honest possibility held: most of it was already measured and needed pointing at, and the one hole is a lookup rather than a gap.**

- **What the page refuses, as a closed set — J11.** Three places, every refusal in F1's shape (the value never becomes an entry, the entry that stood stands, nothing unaccepted is computed from or composed into a payload): the **resolution percentage box**, held to whole numbers from 1 to 32767 by Y1, where out-of-range, a fraction and text are three refusals and not one; the **frame field**, refusing the spec's own four — a last frame before the first, a step of zero, an empty list, a stray character; and the **samples box**, refusing what is not a whole number and imposing no further bound, because none is established. Simplify's three numbers are not answered on this page — they arrive from `Rendering`'s failed-job offer as store entries.
- **What each refusal states — J12, per setting, each on the question the map names for it and none composed from another setting's evidence.** **The percentage out of range:** corrected the instant it is assigned — 1 below, 32767 above — with no error, no warning, no line of any kind, exit status 0, and the render going ahead and being paid for at the corrected size, measured out of the written files in all four versions on both machines (*Settings 15*). **The percentage as a fraction or text:** refused with an error naming the setting — which, had the box taken it, would become an attempt ending at a failure status with nothing written under `Rendering` L8a, so the two halves of that box are not one sentence. **The frame field's list forms:** the text reaches Blender in no form (F1, F3), and what Blender does with a list it cannot read is render nothing at all and still exit 0, printing three named lines word for word in all four versions on both machines (*Frames 7*).
- **The tension the row named is resolved rather than smoothed.** Row 366's Q2a says the two measured percentage behaviours have nothing to bite on **in a Smoozy job**, which is true and is a different statement from this one: J12 states them as **what would have happened had the box not been held**, which is exactly what the sentence promises, and the box's range and the line's content turn out to be one fact rather than two.
- **Unresolved, with what breaks written down — §13 item 26.** The **samples box** has no established statement: `The map.md` poses no question for what the count setting accepts, *Settings 16* answers what displaces a count rather than what it takes, and *What can be set from outside*'s general sentence about a number outside a setting's range rests on the percentage alone, so carrying it across is the assembling Method forbids. **What breaks:** the design sentence's second half in one of its three places; and, because J11 imposes no invented range there, a typed count outside whatever Blender accepts may be silently corrected at assignment and render at a count nobody asked for at success — Row 150's own failure shape in that corner. **What holds:** the count pair and the whole imposition set for every count inside the range, and the whole-number gate. **The fact is gettable** — one question against the count setting's accepted values, answerable in the comparisons or on any box with a Blender on it, and one number closes both halves; it is reported to the runner as worth numbering.
- **J12 also states the two triple forms, and both have an established counterpart — the clause was corrected on 2026-08-23, having first read that they had none.** A step of zero is accepted, reads back as zero and writes the first frame once; a last frame before the first renders nothing at all and exits 0 — both in all four versions on both machines (*Frames 8*, which the joined answer restates in one sentence). **The bound travels with each statement**: both were posed as scene settings and as command-line arguments and never through an explicit frame list, which is the only route F3 composes, so each line states what Blender does with the triple rather than what Smoozy's own list would have produced from it.
- **Nothing was routed.** §14 records the escalation check for J11–J12, declined as a promise weaker in a named corner rather than a stop.

**BUCKET: ANSWERED. Moved out of *unresolved — promise standing* by the runner on 2026-08-31, and the marker is struck rather than deleted so the change is visible to a reader of this row alone.** **Fact 127 was answered on 2026-08-30** — registered blind as *Settings 18* — and the row's third place was written the next day.

**What the answer was, and it corrects this row's own closing sentence.** *One number closes both halves* is **false**: there is no single sample-count setting. Each count declares its own accepted range and each is corrected against its own limits, and the two engines' ceilings differ by nearly two orders of magnitude — Cycles' final-render count takes 1 to 16,777,216, EEVEE's 1 to 2,147,483,647. So the range the line states is **per engine**, not one line for the page.

**And the hazard turned out to be real rather than hypothetical.** A count outside the accepted range is **silently corrected to the nearest limit at assignment** — no error, no warning, no line of any kind — and **the corrected number is what the render works to and is paid for**. That is the *may be* in this row's own break sentence turning into *is*.

**Where it is answered.** `Render Settings` **J11 item 3** now imposes the running engine's own declared range on the samples box, exactly as Y1 already does for the percentage, on the ground that entry is the only place the correction can be caught — the setting read straight back returns the corrected figure, and the progress line is conditional twice over. **J12** carries the second half, *what Blender would otherwise have done silently*: the number would have been quietly changed to the nearest allowed one and the render charged at that figure. **§13 item 26 is closed** and **§13 item 31** carries the answer's own bounds — a driver or keyframe carrying an out-of-range count, a file *holding* one rather than being assigned one, a field typed into or dragged, and Windows, where nothing ran.

~~*(The struck marker read `BUCKET`-colon-`UNRESOLVED — PROMISE STANDING`, added by the runner 2026-08-23, with the samples box waiting on Fact 127 and the claim that one number closes both halves. **The marker's words are spelled out rather than reproduced, because the bucket is derived by grepping that exact string across the row files and a struck copy would still be counted.**)*~~

**Row 438 — the header states when the file was read.** *The header.*
> "So the header reads: the file's name, then the Blender version as a dropdown, then when it was read, then **Read again** and **Reset all to file values**."

Kind of promise: **a drafted design sentence**, resting on a decision and a dependency.

Earns its row by the third test. *When it was read* is **a moment shown to the user for an act performed by a Blender on the machine and presented on the desktop.**

**Two precedents, and neither is transferable by assumption — the checker said so in those words.** **Row 349** gave the server record a second, desktop-side moment because the machine-clock moment could not supply a *last seen* mark without the subtraction D5 forbids. **Row 381** fixed the submitted moment as the one figure on a queue entry taken from the desktop's clock, permitted because nothing rests on it.

**What is open.** Which clock this moment carries, and whether it rides the recorded reading or is stamped on receipt. **It bears directly on the header's own second claim** — that changing the version afterwards leaves the read describing a job that will not happen.

**Answer, 2026-08-23: `Preparation/Architecture/Render Settings.md` [§1.2](../Render%20Settings.md#12-when-the-read-runs-and-what-starts-it) Q18, which answers both open halves and the second claim, with neither precedent taken by resemblance — Q18's lead states each ground afresh.**

- **Which clock — Q18's first bullet: the desktop's**, on D5's own permission (*"the desktop's clock is used for presentation and for nothing a promise rests on"*), because nothing rests on it here: staleness is R13's four values and the version and never an age (Q13), Q14's noticing computes no interval (Q15), Q17's discard tests the version pick and the four values rather than a moment, and no clause anywhere reads this figure to decide anything. **The presentation needs that clock rather than merely tolerating it** — drawn as an age it is arithmetic against the desktop's now, drawn as a clock time it is read against the clock on the user's own screen, and both are the across-clocks subtraction D5 forbids unless the moment is already on that clock.
- **Rides or stamped — Q18's second bullet: both, in the one act.** It is stamped **when Q6 stores the reading**, not when Pre-load was asked for and not at the invocation's start, and it is a field **of** that reading. A read is in flight 3.2 to 3.8 seconds against a page that may move under it, and a reading that is no reading (Q7) or that arrives to be discarded (Q17) never reaches the stamp — so the header states a moment only for the read whose result is the one standing.
- **The machine's clock is not read for this and no duration crosses the two — Q18's third bullet** (D5, `Servers` H2). The render-start reading (R3) carries no desktop-side stamp and is never shown here; the header's moment is only ever a pre-load's. The desktop's own clock is undefended (`Desktop app and delivery` T19, Fact 13's negative half), and what that costs here is a moment drawn wrong, never a decision taken wrong.
- **The header's second claim — Q18's fourth bullet.** The moment dies with the reading it rides, so J10's three drops take it: after a version change the reading for that (server, file) is gone whole (Q12) and there is no read moment left to draw, so the header cannot state when the file was read beside a version the read was not taken with. **The pair goes together rather than the moment being corrected or left standing.**
- **Neither precedent was inherited.** `Servers` H2a's second moment exists for a *last seen* mark's comparison against now, and `Queue & History` N16's submitted moment exists because submission happens on the desktop and the machine was never told when; Q18 argues its own two grounds — the presentation is a comparison against the desktop's now, and nothing rests on the figure — and reaches the same clock for reasons of its own.
- **Nothing was routed and nothing is unresolved for this row.** §14 records the escalation check for Q18, declined.

---

## Filed 2026-08-25 by the Design close re-sweep

**The Design stage closed on 2026-08-23 and this document gained 190 lines and lost 26 — the largest of the eleven.** Nine sections are new. A fresh agent re-swept the whole document against `HEAD`, re-quoted the two promises that had moved, and found three promises this document did not make on 2026-08-23. **Each is silence rather than disagreement** — Rule 10's own last paragraph, and Rule 7 untouched. The rows below carry the sweep's own reasons and the clauses it named. The sweep is at `Audits/Architecture/Sweep/Design — Render Settings — Design close re-sweep.md`; the sitting's plan is at `Audits/Architecture/The ninth sitting — the consolidation plan.md`.

**Three of the four things the runner flagged turned out to be the document catching up with answers that already stood** — the pick line's third reading is B2's own *said as such*, the controls that read unavailable are B5's two honest cases, and the download panel's figures are Row 375's answer I14a — so they are not filed. What earns rows sits elsewhere.


**Row 491 — the read control on a server holding nothing but Blenders below 4.5, where no version is picked.** *When there is no version to pick.*

*(Re-quoted 2026-08-29 at the re-gate, and it is the **first** quotation below that moved. **Design has carried the Rule 10 send-back.** The row was filed against a sentence that withheld Render in both cases but the read only where there is no Blender at all — the enumeration `Audits/Architecture/The tenth sitting — what goes back to Design.md` item 2 sent back, because §7.1 **B5a** withholds the read in the below-floor case too. The sentence now withholds Render and the read alike in both cases, names `Render Settings` B5a as the reason, and states the difference the send-back asked for: the user's pick lifts the withholding there, and nothing on the page does where there is no Blender at all. **The design moved to the architecture's answer rather than the other way round**, so the answer does not move, the row does not reopen, and it does not change bucket. The as-filed wording is kept here in prose rather than in quotation marks, because quoting it would re-create the miss the check reports.)*

> "**Not offered means the control stays where it was and reads unavailable**, rather than disappearing — Render and the read alike, in both cases, which is the [architecture](../Render%20Settings.md) B5a: a read is something a Blender does, and on a server holding only Blenders below 4.5 no Blender is chosen until the user picks one. The difference between the cases is what brings the controls back — there, the user's pick lifts the withholding; with no Blender at all, nothing on this page does, and Server Setup is the way. Render reads *Render* there rather than naming a frame count. A control that vanished would make the page look different rather than closed, and the user would go looking for what they had broken."

and the case it enumerates against:
> "**A server holding nothing but Blenders below 4.5.** The list is not empty, so it stays a list; what is empty is the pick. The control shows no version, and the line beside it says Smoozy does not choose a Blender below 4.5 by itself and that the user can pick one. **The list is not disabled** — picking one is exactly what the user is being invited to do — and Render is not offered until they have, because there is nothing to render with."

Kind of promise: **a drafted design sentence**, new in the close and resting on a decision inside Architecture.

Earns its row by the third test — a dependency to satisfy, and the document's own enumeration is what exposed it. **As filed, the read control was deliberately left live on a machine where no version was picked**: the sentence withheld the read *only* where there was no Blender at all, and in the below-floor case it withheld Render alone and said so twice. But a pre-load needs a named build: `Preparation/Architecture/Job files and outputs.md` **R2** has the reading taken by the version that will render, **Q11** has every recorded reading carry the build's path and answered version, and **Q17** has the read carry the version pick it was started under and be discarded where that pick is not the page's current one — **none of which has a value in this state.** B2 forbids the obvious escape: *"Builds below the whole-app floor of 4.5 appear in the list, can be picked, and are never the automatic pick."*

**And the architecture had answered the same question for the other consumer and not for this one, which is what made it a row rather than an oversight to be assumed away.** **B2a** settles version resolution for a dependency check started with this page never opened, and reaches this exact state in its own words: *"**Where no version can be resolved** — B5's two honest cases, a machine holding only below-floor builds with no user pick, or no Blender at all — **the resolution answers with the case rather than a guess; no check runs**"*. **B5** itself covered submitting in both cases and pre-load in one — *"A machine with no Blender at all: the list is empty, **pre-load has nothing to run with**"* — and said nothing about pre-load on a below-floor-only machine. **So one clause decided *no check runs* for the check and no clause decided anything for the read the page then still offered.**

Waits on: nothing outside. This is a decision inside Architecture.

**What the answer owes:** whether the read control is offered at all before the user picks a below-floor build; and, if it is, what version it runs with, given that B2 will not supply one automatically and Q17 will discard a reading that lands with no pick standing behind it. **Both alternatives are stated decisions rather than drawing** — withholding the read is a fourth thing on B5's page-state, and offering it means a pre-load with a user's below-floor pick, which then anchors R2 for a job the same page will not let them submit until they make that pick anyway.

*(Everything in the four paragraphs above is the state at filing. **B5a** answered it on 2026-08-25 and Design carried the send-back on 2026-08-29; the quotations at the head of this row are the conformed sentences, and the answer below is what stands.)*

*(Superseded 2026-08-25 — the paragraph below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Render Settings.md` §7.1 **B5a**, with the declined third-stop check recorded at §14.

**The answer is B2a's applied to a consumer it never had.** Pre-load is a consumer of **B2a**'s version resolution exactly as `Preparation/Architecture/Dependency Upload & Remap.md` **G2**'s check is, so B2a's own answer holds unchanged: where no version resolves, the resolution answers with the case rather than a guess and **no read runs**. **B2**'s *never the automatic pick* is the escape that stays blocked.

**What is genuinely new is the page-side consequence B2a's consumer never had.** The read control **is not offered** — in Design's own sense of *not offered*, staying where it was and reading unavailable — for the same one reason Render is not offered on that machine, and **the withholding lasts exactly until the user's pick**. That is what separates this case from the no-Blender one, where nothing on the page restores either control. **R2**, **Q11** and **Q17** are the three clauses with no value in that state; **Q12** with Q17 already covers a pick cleared or changed while a read is in flight; **B5**'s reason line already stands on the page, so no second statement is composed.

**Nothing routed. Nothing left unresolved.**



**Row 492 — the pulled-in scene renders at its own engine and sample count.** *What it will write.*
> "**A compositor shared with another scene.** *Interior uses Exterior's render — rendering this scene renders Exterior too, at its own engine and sample count.* The [spec](../../Specs/Render%20Settings.md) has all three parts said before the job goes: which scene pulls in which, that a single-scene job can be much bigger than the scene it names, and that the pulled-in scene renders at the file's values rather than at this page's. **Where both scenes are ticked a second line says the doubling** — *Exterior is ticked as well, so it renders twice, once on its own and once inside this job* — because that is a different fact and only true of that case."

Kind of promise: **a drafted design sentence**, new in the close, resting on an outside fact and on a decision about the reach of this page's own impositions.

Earns its row by the third test twice over. The line states, **on screen and before the money is spent, what a second scene will render at.** Two halves of the sentence are already supplied and the third is not:

- ***Which scene pulls in which*** is `Preparation/Architecture/Render Settings.md` §1.1 **Q2 item 3** — *"Whether one scene pulls in another, by both routes — the compositor and the video sequencer — which is R4 item 3's reading, consumed here for the warning at the spec's *A scene that pulls in another scene*"* — and `Job files and outputs` §4.2 item 3 carries the reading. **Answered.**
- ***At its own engine and sample count*** is answered nowhere. §5 **C1**'s imposition set sets *"the chosen engine's own sample count"*, **C7** imposes the percentage *"against the scene's own percentage setting"* and **D3** assigns *"the scene's own device setting"* — **none of the three names which scene the pair is assigned against**, and the line is only true if it is the job's scene alone. A search of `Preparation/Architecture/` for the pulled-in scene returns Q2 item 3 and `Job files and outputs` §4.2 item 3 and nothing else; neither says anything about the settings such a scene renders at.

Waits on: **Fact 135** — at what engine and sample count a scene reached through a compositor's Render Layers node renders, and whether a value imposed at launch against the job's scene reaches it. **Blender's.**

**The Blender side does not settle it, and the sweep did not assemble an answer.** `Preparation/Blender Research/Across topics/The map.md` routes the compositor to *The compositor 1–7* and **poses no question about a reached scene's render settings**; its scene entry names *Settings 12* and *The compositor 3*. **The joined answers reach the edge of it and stop.** `What a file can be asked before it renders.md` records that *"A compositor node reading a second scene renders that scene fresh but does not run its compositor"* (*Settings 8*, *The compositor 4*) — so the second scene **is** rendered — and `What can be set from outside.md` records that *"Naming a scene picks that scene, and its own settings and its own compositor are what run"* (*Settings 12*, *The compositor 3*), **which is about the scene named at launch and not about one pulled in behind it.** Carrying the second sentence onto the first is exactly the assembling the Method forbids, so this row states the question and states no guess.

**What breaks while it is open:** the page states a cost in the artist's own terms before they pay for it, and **if a typed count or a picked engine does reach the pulled-in scene, the line is wrong in the direction that costs money** — a second scene rendering at a count the user typed for the first.

~~**Not answered.**~~

**ANSWERED 2026-08-29**, at `Preparation/Architecture/Render Settings.md` §5 **C8** — the owning clause — with §8.1 **D3** pointing at it, §5 **C3** taking the observed version split, `Preparation/Architecture/Rendering.md` §1.4 **L10** carrying the launch contract's own side, and the bounds at §13 **item 27**.

**The drawn line stands, true as drawn in all four supported versions.** Blender Fact 135 came back the same day, at question 9 of `Preparation/Blender Research/Comparison/The compositor.md`: a scene reached through another scene's compositor renders with **its own engine and its own sample count**, every figure got twice by run and repeat on the same machine the same day, and read on two separate instruments — Blender's own log lines and channel-exact comparison of the written pictures, with **4.5 printing no engine line at all**, so the picture rather than the log was the engine's instrument there; a view-layer sample override reaches through the node and the **pulled-in scene's own** policy governs it; and **the command-line engine option never reaches that scene at all**, writing onto the active scene alone and nothing to disk. **So the direction that costs money does not happen** — a second scene never renders at a count the user typed for the first.

**The half nobody had answered is now a rule rather than an absence.** C8 states which scene every imposition is assigned against: **the job's scene**, and no pair Smoozy composes ever names another scene of the file — C1's five pairs, C7's percentage, D3's device, the pin pairs and the Simplify pairs alike. That is what the row said was answered nowhere.

**The line's last clause is answered too, and C8's fourth bullet is where it is pointed at.** The Design sentence continues *Where both scenes are ticked a second line says the doubling*, and three standing clauses supply it: **§1.1 Q2 item 3**, which reads by both routes which scene pulls in which; **§2.1 J3**, whose per-(server, file) *scenes ticked* entry is what *both scenes are ticked* names; and **§3 F5** with **§9.3 Y11**, which make a ticked set one job per scene, each anchored. A scene both ticked and pulled in therefore renders once in its own job and once inside the other's, which is the doubling the second line states. Nothing new was decided for it; C8 supplies the pointer, added 2026-08-29 because the material was covered at section level and pointed at from nowhere.

**The one real version difference costs nothing here.** 4.5 does not honour a view layer's sample override under EEVEE where 5.0, 5.1 and 5.2 do, and it shows on the direct render path too. **Either count is the file's own and neither is the page's**, so the drawn line holds on both sides and nothing Smoozy composes, states or warns depends on which side a version falls.

**What is bounded is recorded rather than glossed**, at §13 item 27: the three established conducts rest on **one Mac on Apple silicon**, and the reach of every *other* imposition through the node — the percentage, the device pair, the technology argument, the disablers, Simplify, the pins — was never varied through it. What breaks in those corners is the spec's *at the file's values rather than at this page's*; what holds is C8's composition rule.

**No stop, nothing routed, no Rule 10 disagreement, and no existing clause found wrong.** The third-escalation check is recorded declined at §14 — C8 adds nothing the user meets.


**Row 493 — the colliding pair names where each destination comes from, by name.** *What it will write.*
> "**A colliding pair names both destinations, which is the whole of its value.** *Two of this job's outputs write to the same file. One will overwrite the other. Smoozy changes nothing in your file.* Then the two paths. **Each of the two names where it comes from as well as where it lands** — *the scene's output path*, *File Output node Beauty* — because the path alone is the same string twice and tells the artist nothing about which two things to go and change. **That naming is what makes it act on**: they have to find both in their own file, and *two of your outputs collide* sends them hunting. The Creator has been caught by this in their own work, which is why it is a warning at all."

Kind of promise: **a drafted design sentence**, new in the close, resting on a fact about a `.blend` the page has to hold in order to draw the line.

Earns its row by the third test, and the document argues in its own words that **the naming, not the path, is the whole value of the warning**. **The origin at the granularity the line draws is not stated to be on any record.**

- `Preparation/Architecture/Job files and outputs.md` §7.4 **I15** defines the collision record as *"two destinations **in different nodes** aimed at the same name in the same place"*, and **I16** states that *"where the pair is in the collision record both members are recorded, because the warning names two destinations"* — so both members exist as destinations.
- §7.3 **I14** itemises the inventory entry as *"the predicted path, the frame it belongs to, **which route produced the name**, and the destination that aims at it with its mapped folder"*. *Which route* separates the regular output path from a node recipe, which supplies the first of the document's two examples. **It does not supply the second.** §4.2 item 2 describes the destination set as *"the regular output path where it writes, and for each file output node that will run, one destination for each connected output"* — a per-node destination, with **no clause anywhere stating that the node's own name is carried on it** — and **R5** narrows what is kept per node in as many words: *"What is recorded per node is whether it **will run**, which is items 1 and 2 together."*
- §9.1 **H1** names the collision as one of the three hold conditions and reads it off the collision record; nothing in §9 says what the pair is presented as.

Waits on: nothing to get.

**This is Row 375's answer's own shape, one field further along.** I14a was written on 2026-08-21 because the format the page draws was on no record; the same is true here of the node's name, and the same fix may serve — the field either rides the destination or it does not exist to draw.

**What the answer owes:** whether the recorded reading carries, per destination, the name of the node it came from, **as the file's own bytes** (`Job files and outputs` R4a's discipline would govern them, a node name being as much the artist's text as a location is); and, where it does not, what the two lines say instead.

*(Superseded 2026-08-25 — the paragraph below is what the sweep found, and the answer follows it.)*

**Not answered.**

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Job files and outputs.md` §7.4 **I16a**. Clause by clause: *names where it comes from* is I16a's first and fourth bullets — the origin is a field of the destination in the recorded reading, and both members' origins are recorded where the pair is, in the collision record; *as well as where it lands* is **I14** as it stands, cited rather than re-decided; *the node named as the artist's text* is I16a's third bullet, under **R4a**'s bytes discipline.

**Nothing new was reached for and no fresh Blender claim is made.** §6.3 **P10** and §5.2 **D9** already key pairs per destination, and **I14a**'s shape — a field of the destination, taken in the same one reading — is the pattern this follows. **R5** is not edited; I16a states that its narrowing is about the will-run question rather than a ceiling on a destination's fields. §13's supplies to `Queue & History` and `Render Settings` each gained the field by name.

**Nothing routed. Nothing left unresolved.**

