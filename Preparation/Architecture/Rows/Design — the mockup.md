# Rows — Design: the mockup

Rows **424 to 426** and **470 to 475**, from `Preparation/Design/Mockup/Smoozy.html`. Part of `Preparation/Architecture/Coverage.md`; how to read a row is there, and the facts they wait on are in `Preparation/Architecture/Facts.md`.

Every number here is permanent, never reused and never renumbered.

**Filed at the stage close, 2026-08-20**, from the first sweep the Design documents have ever had, on the Creator's ruling of the same day.

**The mockup is neither a spec nor a design document, and `Method` says outright that it is swept like the rest** — it is the single home of every design value, and it may well owe no rows, which would be an answer. It owed three at that sweep, and six more at the ninth sitting.

**The labels do not discriminate for this document, and that is worth saying rather than leaving to be noticed.** The brief's middle label is *a settled value — it is in the mockup*. For the eleven other Design documents that label picks out the sentences that point here; **for this one it selects everything in the file.** What is worth knowing instead: **none of the three filed in 2026-08-20 is Creator-stated, none carries a date, and all three are drawings rather than sentences.** `About this mockup.md` puts drawings below documents — *"Where the two disagree, the documents win"* — so if a row's behaviour is one a Design document later contradicts, **it is the drawing that moves.** These three numbers are therefore the softest ground in the whole consolidation, and they were spent knowingly.

The last of those three, **Row 426**, was filed by the runner at the consolidation rather than by the sweep of this document, and the reason is recorded on the row: two agents reached the same figure from opposite directions, one of them declined to file it, and the one that searched hardest found nothing producing it.

**Rows 470 to 475 were filed at the ninth sitting, 2026-08-25**, from two sweeps run against the drawing after the Design close, which took it from 4,422 lines and 148 drawn states to 9,519 lines and 196. Rows 470 to 473 come from `Audits/Architecture/Sweep/Design — the mockup — the three drawn modules.md`; rows 474 and 475 from `Audits/Architecture/Sweep/Design — the mockup — Design close re-sweep.md`. **All six arrive unanswered.** None of the six is Creator-stated and none carries a date; four are drawings and two rest on a sentence in the drawing's own Design notes panel, which is prose inside the file rather than a document beside it.

**Rows 470 to 473 are the first rows this project has ever filed for the design of `Access`, `Rendering` or `Telegram`.** Until the Creator's ruling of 2026-08-23 those three modules had no design at all, and `Preparation/Architecture/Coverage.md` records the consequence in its own words: *"Whatever those three hand over is in no row above and in no sweep"*. That ruling made this file their plan — `Preparation/Design/Mockup/About this mockup.md` states *"Access, Rendering and Telegram live here and nowhere else"* and *"That makes this file the plan for those three, and only those three"* — so for those three the subordination stated above does not apply. There is no document beside the drawing for it to lose to, and a drawn decision of theirs moves only when the architecture contradicts it.

**The coverage gate's quote check covers this file from 2026-08-29, and until that day it did not.** ~~A mockup-quoting row is outside the automatic check and has to be read.~~ The old check found a promise by looking for forty or more characters between two double quote marks, and **every row in this file quotes a line of HTML, which is full of double quote marks** — so it split each quoted promise into fragments too short to test and reported nothing at all. A quote that had gone missing and a quote that still stood looked identical to it. That was stated here on 2026-08-25, after the Design close moved one of these quotes and withdrew another with the scan seeing neither.

**The check was widened at the stage close and now reads a drawn value as a quotation in its own right**, pairing backticks by position rather than by pattern. **Thirty-five drawn values are checked where none was before, and it found two breaks in this file's rows on its first pass** — Row 425's stale second copy in an answer document's head table, and Row 424's promise, gone from the drawing since 2026-08-25 with nothing but this file's own prose recording it. **Row 424 will be reported as a miss on every pass from now on and nothing is owed for it**; the row says why. `Audits/Architecture/Gates/The quote check widened — what it can now see.md` is the record.

**Corrected 2026-08-29 at the re-gate.** This paragraph read that Row 424 is *the one break this stage carries knowingly*, and that stopped being true the moment Rows 471, 472 and 473 were answered: each refused a drawn value under Rule 10, Design carried all three, and every one of their quotations then missed. **Those were not breaks and were never owed anything** — they are the send-backs working — but the sentence counted them as though they could not happen. The three rows are now re-quoted against the drawing as it stands, so the count is right again by repair rather than by wording. **Row 424 is different in kind and is why the sentence is kept at all:** its promise was withdrawn from the drawing with nothing put in its place, so there is no current wording to re-quote and the miss is permanent. `Audits/Architecture/Gates/Re-gate 2026-08-29/The sixteen quote-check misses, read one by one — Claude Opus 5.md` reads all sixteen one by one.

**All three of the 2026-08-20 rows were re-read against the drawing on 2026-08-25 and each says on itself what came of it.** Row 424's promise is gone from the file, which is Rule 10 working rather than a fault; Row 425's second quote was replaced and its first left alone; Row 426's quote still finds, unchanged.

---

**Row 424 — the queue's server heading says when a server will next be idle.** *The Queue screen — the `qmach` heading of the `Vast 5060` group.*
> `                  <span class="qm2 open">idle in 12 min</span>`

Kind of promise: **a settled value** — it is in the mockup. Not Creator-stated, no date.

**Earns its row by the third test in its sharpest form.** Every other reading on that heading is a statement about **now** — *1 rendering · 2 waiting*, *not updating · last seen 11:04*, *62%*. **This one is a statement about the future**: how long until the machine's line runs out and it falls quiet. It cannot be drawn at Build from what the app has been arranged to know, **because the arrangement written for this stage deliberately produces no such figure.**

Waits on: nothing to get. **No fact, Blender or otherwise, is missing** — the difficulty is not that nobody has measured something. It is that the figure is a **forecast over a whole server's line**, and the closest thing this stage has written declines to offer one.

**Not answered anywhere, and the nearest architecture declines the smaller version of the same figure on stated grounds.**

- `Preparation/Architecture/Queue & History.md` **Section 12, W1 to W6** is the whole of *idle* in this stage. **W1** defines it as a **present condition** — *"A machine is idle when it is connected, no render operation is running, and its line holds nothing that can start"*. **W4** schedules a re-check *"while connected and short of the half hour"* and re-verifies against the machine before speaking. **W5** speaks once per idle stretch. **Every clause is about a machine that *has* gone quiet and for how long. Nothing in the section produces a moment at which one *will*.**
- **B9 is the finding that makes this sharp rather than merely absent.** It records that **no countdown to the end of a job is offered**, and gives the reason: *"no running figure covers a whole animation in any version, the whole-job total is a plan adaptive sampling legitimately undershoots, and Blender's own estimate covers the frame in hand and comes and goes"*, so *"A countdown built from elapsed time and counted frames would be a confident figure Smoozy cannot stand behind on a machine billed by the hour"*. B9 closes by naming itself *"the row where nothing is offered is the answer"*. **A countdown to when a server's whole line runs out is strictly wider than the one B9 declines** — it is that countdown summed over every job on the machine, plus the one running.
- **R6**, the closed list of readings the render reader produces, contains no whole-job or whole-line time figure; the only estimate in it is *"Cycles' time estimate for the frame in hand, where a line carries one, marked as Blender's and as rough"*.
- `Preparation/Architecture/Servers.md` hands idle timing to `Queue & History` and holds none of its own. `Facts.md` neither asks for it nor answers it. No existing row carries it.

**The mockup shows a number on a screen that the architecture, having considered the smaller version of the same number, wrote down that it would not supply.** And the same drawing labels the one time figure the architecture **does** offer exactly as R6 requires — `~1m 40s left · Blender's estimate` — two screens away.

**Answer, 2026-08-21: refused, at `Preparation/Architecture/Queue & History.md` [W1a](../Queue%20&%20History.md#12-the-idle-half-hour) — and the *strictly worse* test was worked rather than asserted.**

- **The figure would be three terms**: the running render's remaining time, plus the whole duration of every startable job in the line, plus whatever a person does with the held ones. **B9's declined figure is the first term alone**, and B9's three grounds carry to it unchanged.
- **The second term has no operand at all.** An unstarted job has no elapsed time, no counted frames and no estimate of Blender's. **The only quantity held for it is B1's plan total**, which B5 already bounds to *progress against the plan* with *"a job can correctly end short of it or pass it"* — and turning plan units into a duration needs **a units-per-second rate nothing in this stage measures, records or supplies.**
- **And the line is not a fixed quantity.** Held jobs cannot start until a person answers (W1, U5), the user may reorder or remove work at any moment, and **untracked work counts as work** (W1, A4): a machine rendering a declined job is busy, Smoozy holds no manifest, no plan total and no reading for it, **so any forecast is short by an amount it can neither bound nor state.**
- **The harm B9 names is larger here, not smaller** — one job's error becomes that error summed over the line, on the one heading that stands for the machine.
- **What the heading carries instead** is the present readings it already draws plus W1 to W5's idle statement. **The mockup's own shut-state sibling was *Nothing rendering · 4 waiting* when this row was answered, and it was exactly the supported shape; the Design close of 2026-08-25 redrew those headings, and what stands in their place — a running job's name with its bar and its waiting count, and *Not updating · last seen 11:04* — is the same shape and is likewise a statement about now.** The drawn element is handed back to be parked and **no Design document is repaired here.**

**The promise was then withdrawn by the design, 2026-08-25 — and the row's number, its quote and its reasoning all stay exactly where they are.** The Design close carried W1a's refusal into the drawing: the string quoted at the head of this row is **no longer anywhere in `Preparation/Design/Mockup/Smoozy.html`**, checked in the file on 2026-08-25. The three `qm2 open` readings the file now carries are *1 rendering · 2 waiting* at line 4500, *1 rendering · 3 waiting* at line 4544 and *not updating · last seen 11:04* at line 4580 — which is W1a's *"the present readings it already draws"*, drawn.

**This is not a broken row and not a fault. It is Rule 10 working as intended**, and it is the clearest instance this stage has: the architecture refused a figure on stated grounds, and the design changed rather than the answer. **The row is closed twice over** — by the refusal of 2026-08-21, and by the withdrawal of the promise it was filed against.

**Nothing is struck and nothing is deleted.** W1a's argument above is the record of **why no idle forecast is offered anywhere in Smoozy**, and a reader who arrives at this file asking *why does no screen say when a server will next be free* has to be able to find it here. **The quote will not be found in the source document, and that is expected rather than a repair owed** — see the note on the quote check at the head of this file.


**Row 425 — the window's title bar is Smoozy's own, and it carries a live reading.** *The title bar of the drawn application window.*
> `    <div class="lights"><i></i><i></i><i></i></div>`

and
> `    <span class="rate mono" id="axrate">↑ 12.4 MB/s</span>`

Both lines are inside the `<div class="titlebar">` that sits directly below the menu-bar row and above `<div class="body">`, **and which is therefore on every screen in the file.** Its height is fixed by the token `--title-h: 38px;`, which `Preparation/Design/Design values.md` lists in its table *The shape of the window* as one of *"the load-bearing sizes of the window itself"*, against the entry *The title bar*.

**The second quote was replaced on 2026-08-25 and the first was not.** The Design close gave the rate span an `id` attribute, `axrate`, when the Access work needed the reading hidden on the entry screen; the string above is the line as it now stands in `Preparation/Design/Mockup/Smoozy.html` at line 2574, taken from the file and checked character for character. **The reading is unchanged, the drawn bar is unchanged, and the answer at H6a below stands untouched — only the quoted string moved.** The first quote, the three window buttons, still stands word for word at line 2566 and was left alone. *(Both line numbers corrected 2026-08-29 at the second stage-close re-sweep — they read 2580 and 2572, six high, the title bar having moved up when the Help menu was removed from the block above it. **The head table of `Preparation/Architecture/Desktop app and delivery.md` was carrying the pre-`axrate` string until the same day**, and the widened quote check is what found it; the row file had been correct since 2026-08-25 and the second copy had not.)*

Kind of promise: **a settled value** — it is in the mockup, and the height token is carried into `Design values.md`. Not Creator-stated, no date.

**Earns its row by the third test.** Three window buttons drawn as part of the application's own layout, **at a height the application fixes**, in a bar that also holds a live figure the operating system's own title bar cannot hold — **that is a title bar the application draws, not one the system draws.** It cannot be reached from what the app already knows; it requires the shell to be able to hand the frame over, **which is a property of the shell exactly like the two properties the architecture already holds it to.**

Waits on: **an outside fact about the three operating systems and about the chosen shell — whether the application can give up the operating system's own window frame and draw the title bar itself (the window buttons, the drag region and the bar's height) on macOS, Windows and Linux, and whether the shell chosen in `Preparation/Architecture/Desktop app and delivery.md` exposes that.** **It is not in `Preparation/Architecture/Facts.md` under any number** — that file was searched for window-frame material and holds only Facts 12, 13, 22, 23, 24, 73 and 98, which are about menu bars, download folders and update replacement, none of which reach the frame.

**Partly answered, and the missing half is the surface rather than the value.**

- **The reading is arranged.** `Preparation/Architecture/Desktop app and delivery.md` §11.4 **R15**: *"The reading presented to the user is `Server protocol and transfers`' R12 — Smoozy's own count of bytes moved over a window, taken at the same accounting point as the enforcement, per channel and summed. It is never read out of a tool's output and never derived from a file's size and a start time."* **R16** carries its two limits. So the figure in that span exists and has a defined provenance, and the mockup draws it consistently in three places — here, in the Transfers heading, and in Settings. **That reading's own unanswered clause — the length of the window it is averaged over — is Row 329's and not this row's.**
- **The surface is nowhere.** §2.2 **H6** is the list of required properties of the shell, *"and a candidate failing any one of them is not this route"*. **There are eight** — seven when this row was filed, an eighth added 2026-08-21 with [2.6](../Desktop%20app%20and%20delivery.md#26-the-colour-scheme-read-and-declared), *"A window frame that follows the operating system's light-or-dark setting, or nothing set at all"*, **which is about the frame's appearance and not about whether the frame can be given up at all**. Property 6 is *"The platform's native menu bar and the platform's native file picker exposed to the application"*. **No property asks whether the application can draw the title bar itself**, which is this row's surface. §18.3, K9 and K10, works the menu-bar property through in detail — including what a shell that cannot expose it costs — and says nothing about the frame. §2.4 (H11 to H15) and §2.5 (H16) enumerate what the small shell costs and which promises it was checked against; **the frame is in neither.** Every file in `Preparation/Architecture/` was searched for *title bar*, *titlebar*, *traffic light*, *window control*, *frameless*, *window frame*, *minimum window* and *window size*: the only hits are Row 305's own quote and the Microsoft menu-bar citation that mentions a title bar in passing as the thing a menu bar sits below.

**Why the gap is worth naming rather than waving through.** **H16** states that *"the smaller route keeps every promise on every configuration this project can establish, no promise is offered up for size"*, and it reaches that by walking the promises Wails touches one at a time. **This is a promise Wails touches that was not on that walk — because no row carried it, which is exactly the condition this sweep was run to find.** If the chosen shell cannot give the frame up on one of the three systems, H7's fallback order is the remedy and the cost is small; **but nobody has checked, and R15's reading currently has an established value and no established place to sit.**

**Corrected 2026-09-02: the shell is Tauri v2 and not Wails.** Build filled `Desktop app and delivery` H5b from measured artifacts and Tauri came out materially smaller, so H16's walk was over the candidate that did not win. **The walk's answer does not move and the drawn title bar survives**: H6b checked all eight of H6's properties and H6a's own frameless facility against Tauri's own documentation and none fails, and H6a's *whether Tauri publishes the same facility* — deliberately unread until then — is now read and holds. **Two build details differ and neither is a promise**: the drag attribute does not reach child elements on this shell, and the macOS custom title bar publishes a cost Wails left unstated.

**Answer, 2026-08-21: answered on all three systems, at `Preparation/Architecture/Desktop app and delivery.md` [H6a](../Desktop%20app%20and%20delivery.md#22-the-route-chosen-and-the-properties-it-is-held-to), from the shell's own documentation and its own release notes. `Facts.md` [Fact 123](../Facts.md) is numbered from it.**

- **The frame is given up by one published option**, and with it the title bar becomes the application's own. **The drag region is a CSS property on any element**, with nested elements marked no-drag; **the window buttons are drawn by Smoozy and wired to the shell's own published runtime acts**; and **the bar's height is not the shell's at all** — with the frame gone the web view is the whole window, so `--title-h: 38px` is one row of the page's own grid and **no platform figure is read or reconciled.**
- **All three systems are named in the shell's own release notes**, which is what turns one cross-platform flag into per-system evidence rather than an assumption from a flag's absence of a platform table.
- **Windows keeps its shadow and its rounded corners**, the decorations option left at its default. **macOS has a second route** which keeps the system's own window buttons in the system's own place; **both are supplied and the frameless one is taken**, because that is what the mockup draws, and which the Mac uses is Design's.
- **It is deliberately not made a ninth entry on H6's required-property list.** H6's properties **disqualify a candidate**, and what rests on this one is a **drawing** — `About this mockup.md` already rules that a drawing moves when it disagrees with the documents — so a fallback shell that could not do it would cost the drawn bar and **no promise**. It is nonetheless checked exactly as a property is checked.
- **H23 is narrowed rather than contradicted**: with no system title bar on any of the three, the Windows and macOS appearance defaults govern the border alone, and the bar's colour is H17's reading with H19's declaration. **Rows 321 and 327 do not move.** **H16's walk is extended and its answer does not move.**
- **Two limits ride**, at §22 items 39 and 40. **What a frameless window costs per system is published only in part and none of it is observed** — the shell states what frameless removes only as *no borders or title bar*, names the standard window buttons nowhere, and publishes nothing about drag-to-edge arrangement, double-click-to-zoom or the system window menu on any system, with **Linux documented thinnest of the three.** **Nothing promised breaks**, because Smoozy draws its own three buttons on every system either way. And the shell's documented fullscreen carve-out disables the drag functionality, **which is a documented interaction between two Design decisions taken apart from each other** — Row 305's Window menu offers a full-screen item — and costs nothing, a full-screen window being undraggable on any system.


**Row 426 — a request's remaining time.** *The Transfers screen — the upload request's summary line.*
> `To RunPod 4090 · 1.4 of 4.2 GB · about 6 minutes left`

**Re-read against the drawing on 2026-08-25, after the Design close: the quote still finds and nothing is owed on this row.** The text occurs inside line 3517 of `Preparation/Design/Mockup/Smoozy.html`, unchanged. It is recorded because the two rows above it both moved at that close, and a reader should not be left wondering why the third did not.

Kind of promise: **a settled value** — it is in the mockup, which is the single home of every design value. Not Creator-stated, no date. **`Preparation/Design/Transfers.md` does not mention it**: that document's own sentence for the line is Row 411, which names a count, a bar and a destination and stops.

**Filed by the runner at the consolidation, 2026-08-20, and the reason is worth recording.** Two agents reached this figure from opposite directions. The `Transfers` sweep searched all forty-odd architecture documents for it, found nothing, and **could not file a row because its own document carries no sentence to quote** — it wrote the finding down and asked the runner to decide where it belonged. The sweep of this document had the quotable drawing and **declined to file it**, on the ground that it is *derivable at Build* from bytes remaining and the achieved rate. **The runner rules that the dismissal misapplies the floor.** The question is not whether an arithmetic exists but whether the app has been arranged to know its inputs against the right denominator over a defined window — and R12 forbids the naive route in as many words. **So it is a row, and it is filed against the mockup because that is where the promise is drawn.**

**Earns its row by the third test.** It is a value on a screen that nothing in this stage produces.

Waits on: nothing to get.

**Not answered anywhere.** **Nothing in `Preparation/Architecture/` produces a time estimate.** All forty-odd documents were searched for an estimate, a remaining time, a time left and a finish time; the only hits are Fact 25's *"the slowest set the finish time"* about three machines sharing one line, and waiter deadlines, and neither is this.

**The ingredients exist and nothing pairs them.** `Preparation/Architecture/Server protocol and transfers.md` **R12** gives an achieved rate per channel, summed; `Preparation/Architecture/Download.md` **M16** gives a byte total that exists before the first transfer starts. **Nothing says whether the estimate is taken against the request's own channels or against the shared budget the request is competing inside, and nothing says over what window.** R12's own rule is a caution here rather than a help: the rate is *"never derived from a file's size and a start time"*, **which is the naive way an estimate would be built.**

**And the neighbouring case shows the standard the architecture holds itself to.** `Preparation/Architecture/Queue & History.md` **B9** declines a countdown over a job on the stated ground that a confident figure Smoozy cannot stand behind is worse than none, on a machine billed by the hour. A transfer estimate is not a render estimate and the reasoning does not carry across untouched — **but nobody has done the work of saying so.**

**Answer, 2026-08-21: supplied and decided, at `Preparation/Architecture/Desktop app and delivery.md` [R15b](../Desktop%20app%20and%20delivery.md#114-the-achieved-rate-reading).**

- **The rule is bytes remaining divided by the request's own achieved rate.** Bytes remaining is the request's byte whole — `Download` M16 downwards, `File Browser` U11 upwards, **both existing before the first transfer starts** — minus bytes staged summed over its units (G4), with units whose disposition is not to send absent from both terms.
- **The rate is R12a's, over R12a's window** — the trailing three seconds at the same accounting point, with the elapsed-time divisor before it fills — **summed over this request's channels and no others.** The only difference from R15's reading is the set summed over. **A per-direction rate is forbidden here** because it moves when another request is admitted or ends, which the line it sits on does not name.
- **It is never built the naive way.** R12's ban on a rate derived from a file's size and a start time **also rules out the average-since-the-request-began route**, and the estimate is recomputed from the trailing window at every accounting interval **with no memory of an earlier estimate.**
- **Three states carry no estimate and say so rather than showing a number**: no channel of the request moving bytes, the whole not yet known (M17's running enumeration), and the request ended.
- **Why B9 does not carry, worked rather than asserted.** B9's three grounds are all that **nobody measures the quantity**. Here **neither operand is modelled**: the whole is an enumeration of exact byte counts, durable before the first byte, **and it is not a plan a job may legitimately finish short of**; the rate is Smoozy's own count at the point that enforces the caps. **The figure converges as the request runs instead of being one claim from a model**, which is the opposite of B9's *a confident figure Smoozy cannot stand behind*. **Nothing here offers a remaining time for a render and nothing composes one from this estimate.**
- **One assumption and two inherited limits are declared.** The estimate assumes the request keeps its present share of the budget — **so a cap change, an admission, a completion, a higher-class competitor or a start-over all move it, and it is wrong for up to three seconds afterwards, exactly as long as the rate beside it is wrong.** The byte whole is drawn from **apparent** sizes, so a sparse file's share is its claimed size; and downwards the count is bytes Smoozy accepted rather than bytes the line carried.


**Row 470 — the two names Smoozy offers for the bot, and the shape of the one it offers again on a tap.** *The `Make a Telegram bot` dialog — step 2 of the walkthrough, under the heading* It asks for two names. Copy each of these.
> `                <div class="tg-nm"><span class="tg-nl">Name</span><span class="mono">Smoozy</span><button type="button" class="mini">Copy</button></div>`

and
> `                <div class="tg-nm"><span class="tg-nl">Username</span><span class="mono">smoozy_a7k2_bot</span><button type="button" class="mini">Copy</button><button type="button" class="mini">Another</button></div>`

Lines 4325 and 4326 of `Preparation/Design/Mockup/Smoozy.html`, inside the `#stgbot` dialog's `bsteps` list. The Design notes state the same promise at line 8024 — *"the two names ready to copy, another username on a tap"*.

Kind of promise: **a settled value** — it is in the mockup. Not Creator-stated, no date.

**Earns its row by the third test, on all three of its limbs.**

- **A decision that is not obvious.** The drawing fixes the display name as the literal string *Smoozy* for every user's bot, and fixes the username's composition as a stem, four characters, and a `_bot` suffix. Nothing in `Preparation/Specs/Telegram.md` and nothing in `Preparation/Architecture/Telegram.md` states either. `Telegram` §6 S1 goes exactly this far and no further — *"Smoozy offers both names ready to copy; offering another unique name on a tap needs nothing from the bot-making chat, because Smoozy only generates candidates — the user carries them over"* — which says who generates and says nothing about what is generated.
- **An outside fact.** A candidate that Telegram will not accept turns the spec's *"minute of work"* into a loop the walkthrough cannot detect, because Smoozy never speaks to the bot-making chat and only learns of a refusal when the user comes back without a token.
- **A dependency.** The regenerated candidate has to be different from the last one in a way that changes the collision, which is a property of the composition rule rather than of the tap.

Waits on: **Fact 133** — what Telegram requires of a bot's display name and of a bot's username, the permitted characters, the length bounds and the terminal `bot` requirement, and whether anything other than the bot-making chat can tell a candidate username from a taken one. **It is an outside fact about Telegram, not a Blender fact, and must not be answered from memory.** It is owed against Telegram's own published bot documentation, which is the same publication `Telegram` §3 B13, §11 I3 and §12 M8 are already read against. No rented machine is needed.

**Not covered by anything already filed.** Row 256 waits on Fact 69, and `Telegram` §21 item 3 states Fact 69's remainder exactly: *"Fact 69 — what the bot-making chat does when a unique name is already taken. **Breaks:** one walkthrough sentence's wording — how the collision is described to the user; the mechanism (Smoozy generates another candidate on a tap) needs nothing from the chat."* That is the wording of the collision message. **The composition of the candidate itself is named nowhere.**

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.** Filed unanswered at the ninth sitting, 2026-08-25.

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Telegram.md` §6 S1a–S1c.

The offered name is the literal `Smoozy` held to Telegram's 64-character ceiling; the username is a fixed stem plus four characters from lowercase letters and digits plus the required ending, **satisfying Telegram's published username rule by construction — letters, digits and underscores only, 5 to 32 characters, ending in `bot`, lowercase because usernames are not case-sensitive.** **The tap changes the four-character block and nothing else** — **S1b**: the stem and the ending never move, and a draw matching any block already offered in this walkthrough is drawn again, so every tap yields a candidate different from every one before it and re-rolls the collision across that block's 1,679,616 values. **One residue is declared and is this row's own: [Section 21](../Telegram.md#21-unresolved) item 25** — Telegram leaves the rule's edge cases unenumerated behind one catch-all error, and the one every candidate of this composition carries is the underscore immediately before the suffix. **If it goes the wrong way every candidate is refused identically and no tap cures it**; the user falls back to typing a name of their own (S1c), so the bot is still made and what is lost is the offer's value rather than the walkthrough. **The hard negative is written in: nothing available to Smoozy answers *may this be used for a new bot*, so no candidate is ever offered as free and no surface may word it so.** Every refusal is met inside Telegram's own chat, another tap cures a collision, and the user may always type a name of their own — the offer is a convenience and never a gate. **The per-account bot ceiling was weighed against the third stop and declined**: Smoozy cannot detect it, both remedies are Telegram's own, and no composition escapes it.



**~~Row 471~~ — the form the licence key is shown in, in the field and in the mask.** *The entry screen's field, the lock band's field, and Settings'* Licensed *reading.*

**STRUCK 2026-10-04; marked 2026-10-26.** The Creator withdrew the paid layer. This promise is no longer owed; the former quotation, answer and limits below remain as the record. The number is permanent and is never reused.
**Re-quoted 2026-08-29 at the re-gate. Design has carried the Rule 10 send-back and no `SMZ-` form is drawn anywhere in the file any more.** The two shaped placeholders were replaced; the masked reading was dropped whole rather than replaced, so there is nothing to re-quote in its place. The as-filed wording is kept below in prose, because quoting it here would re-create the miss the check reports.

**What the drawing carries now** — the entry screen's *Enter a licence key* card (`#axentry`, `.ax-card`) at line 3151, and the lock band (`#axband`) at line 3108:
> `            <input class="num mono" id="axkey" placeholder="Paste your licence key" aria-label="Licence key">`

and
> `      <input class="num mono" id="axlockkey" placeholder="Paste your licence key" aria-label="Licence key">`

The same unshaped placeholder is drawn a third time at line 4376, in Settings' trial block.

**The masked reading is gone and nothing stands in its place.** The Licence section's `data-lv="ok"` block no longer draws the key at all; it reads the computer's name and when it was last checked. The `data-lv="none"` block's second copy of the mask went with it.

**What the row was filed against, kept in prose.** Two key fields carrying the shaped placeholder `SMZ-XXXX-XXXX-XXXX`, at what were then lines 3130 and 3091, and a masked reading `SMZ-••••-••••-••••` in the Licence section at line 4220 with a second copy at 4255 and the shaped placeholder again at 4278. **A9 refused all five as a seller-specific form the seller's own documentation does not promise**, and Design carried it.

Kind of promise: **a settled value** — it is in the mockup. Not Creator-stated, no date.

**Earns its row by the third test, and this one is sharper than it looks because an answer already bears on it from two sides.**

- **`Preparation/Architecture/Security and licence.md` §3 rule 6 permits the mask only on one condition and says outright that nothing supplies what the condition needs.** Rule 6 was added 2026-08-21 per Row 334: *"No screen, notification, message, exported record or log carries a stored value or any part of one, and **no get is ever performed in order to compose something to show**."* Its second consequence is the one this drawing sits on: *"**A mask drawn to a fixed shape is a drawing of *remembered* and is permitted; a mask that preserves any part of the value, its length included, is the value.** **Nothing anywhere supplies a display fragment of a stored secret** — not the local record, not §1.2's check response, not the store — so a screen that wants one is asking for a record no document holds."* The drawn mask is a fixed shape **only if every key really has that shape**. If keys vary in length or grouping, the same mask stops being a fixed shape and becomes the value's length, which rule 6 refuses; and `Preparation/Architecture/Access.md`'s binding section says this module *"holds a handle to the licence key and never the plaintext"*, so there is nothing on that side to compose a truthful mask from either.
- **`Preparation/Architecture/Licence service.md` §8 E4 forbids the desktop binding to any seller.** *"Replacing the seller changes the adapter and the address, and what it must not change is the desktop, the request, and the opacity of the reference."* A key form compiled into a placeholder on the entry screen and the lock band is a seller's format inside the desktop application, and it survives the seller being replaced whether or not the next seller's keys look like that.

So there is a fact to get and a decision to take on top of it: what the two fields prompt with, and what the Settings reading shows, given that no document holds a display fragment and no document holds the key's form. **A wrong answer fails quietly** — a paying user pastes a legitimate key that does not match the shape the field advertises, and either doubts the key or is silently corrected.

Waits on: **Fact 10**, which already exists and is not renumbered here — the seller, and what that seller can do. What this row needs of it is the part no reading has reached: **what form a licence key issued by the seller takes, whether that form is the same for every key it issues, and whether Smoozy has any way to know it.** `Licence service` §8 E1 records the seller as Lemon Squeezy with its documentation read 2026-08-16 and citation-checked, and records that *"a license key will be generated and emailed to the customer"* — and it records nothing about the key's shape. **No reading of the key's form exists anywhere in `Preparation/Architecture/`.**

*(Superseded 2026-08-27 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.** Filed unanswered at the ninth sitting, 2026-08-25.

**Answered 2026-08-27 at `Preparation/Architecture/Access.md` §1 A9, from Fact 10's focused official reading.** The seller's current documentation shows UUID-shaped examples but promises no invariant key form and exposes no format-discovery field; its `key_short` is a real secret fragment, not a safe or seller-neutral mask. Every entry field therefore carries no shaped example or seller prefix and performs no client-side format validation, while Settings receives only the existing fact that a key is remembered or entered and may represent it only with one fixed seller-neutral indicator that is not derived from the key. **All three `SMZ-XXXX-XXXX-XXXX` placeholders and both `SMZ-••••-••••-••••` readings are refused and sent back to Design under Rule 10. Nothing is unresolved; Design owns the replacement's exact words and drawing.**


**Row 472 — Simplify's three values, drawn with no file value behind them.** *The render settings panel's Simplify group, shown on a re-submission.*
**Re-quoted 2026-08-29 at the re-gate. Design has carried the Rule 10 send-back, so what this row was filed against no longer exists and the drawing now says the opposite.** The note the row quoted is gone whole and its replacement refuses the premise in terms; two of the three drawn values are replaced by the file's own read-back values. The as-filed wording is kept below in prose, because quoting it here would re-create the miss the check reports.

**What the drawing carries now**, at line 8196 of `Preparation/Design/Mockup/Smoozy.html`, in the Design notes under the heading *Starting a render, and starting it again*:
> `    <p><b>The switch and the three take the page's own composition</b>&nbsp;— the file's read-back value in the file's italic, the user's answer in bold with the cross that hands the file's value back. A saved scene records Simplify's switch and a pair of values for each of the three, all readable back like anything else, so there is a file value behind every row. <b>No starting value is invented</b>: the drawn 2, 0.50 and 1024 this note once defended came from no document and are gone. Without pre-filled reducing values the offer starts at the file's own limits, which on an untouched scene may cut little&nbsp;— a recorded cost, not an&nbsp;oversight.</p>`

and the drawn values as they now stand, in the Simplify group (`.simp`, `data-ag="again"`, from line 7323):
> `                      <span class="fld w-num mono">6</span>`

> `                      <span class="fld w-num mono">1.00</span>`

> `                      <span class="pick" style="width:104px"><select aria-label="Texture Limit"><option>No Limit</option><option>128</option><option>256</option><option>512</option><option selected>1024</option><option>2048</option><option>4096</option><option>8192</option></select></span>`

**The Texture Limit list is unchanged and always matched**, because A1d refused the preselected constant and not the option list, which is the file's own documented enumeration. What moved is that the row is now the user's answer with a cross naming the file's value, rather than a value from nowhere.

**What the row was filed against, kept in prose.** A Design note reading that the three are neither the file's italic nor the user's bold, that Simplify is the one thing Smoozy adds to a job's settings rather than takes out of the file, and that there is therefore no file value behind them and nothing for a cross to hand back — with the drawn constants `2` and `0.50` beside it. That note sat at line 8037 and the values at lines 7201 and 7205. **Fact 134 disproved the premise and A1d refused the constants**; the send-back is `Audits/Architecture/Row 472 — what goes back to Design.md` items 1 and 2, and Design carried both.

Kind of promise: **a settled value** — it is in the mockup. Not Creator-stated, no date. **The Creator's ruling of 2026-08-12 that Simplify is promised as it stands sits on Row 22 and does not reach this.**

**Earns its row by the third test, on a decision and a fact together.**

- **A decision nobody has taken.** When the user switches Simplify on for a re-submission, Smoozy sends three values. `Preparation/Architecture/Rendering.md` 1.4 L10 delivers them — *"Simplify's switch and its three settings — Max Subdivision, Child Particles, Texture Limit — as settings pairs"*, *"exactly where the payload carries them"* — and says nothing about what the payload carries. The drawing puts three concrete values on the screen (2, 0.50, 1024) and declares in the Design notes that they come from nowhere in the file. **Somebody has to say where they do come from.**
- **The fact decides whether the drawn premise stands at all.** If the file records Simplify values, then Simplify is *not* the one thing Smoozy adds rather than takes out of the file, the italic-and-bold convention the whole panel is built on applies to these three rows like every other, and the drawing's own reason for drawing them plainly is gone. If it does not, the three values are Smoozy's own defaults and need a home.
- **A second Blender claim rides on the same fact.** The Design notes at line 8036 exclude one setting on a version claim: *"Texture Resolution, which only one of the four supported Blenders has and would appear and disappear with the Blender chosen"*. Nothing in the map or the joined answers carries that, and Row 22's own constraint line records what the research does and does not cover for this group — *"Not covered by the research: whether Texture Limit or Child Particles imposed this way actually reach a render, anything on a graphics device, and Windows."*

Waits on: **Fact 134**, and it is **Blender's**, looked up in the existing research rather than asked again. `Preparation/Blender Research/Across topics/The map.md` names no question for Simplify, and the one joined answer that reaches it is `Preparation/Blender Research/Across topics/What can be set from outside.md`, which lists it among the settings with no launch argument: *"No argument in any version: the sample count, the resolution percentage, whether a view layer renders, tile size, persistent data, render region, simplify, the time limit, denoising, bounces, and every setting belonging to the other two engines"*. **That settles how a value is imposed and settles nothing about what the file already records or what a read returns.** So the fact, stated as the row and not as an answer: **whether a `.blend`'s scene records values for Simplify's switch, its maximum subdivision, its child-particle proportion and its texture limit; whether the reading `Render Settings` performs returns them; and which of the four supported versions carry which of those settings.**

**Not covered by anything already filed.** Row 22 quotes the spec's promise that the three are offered and is answered at `Rendering` L10 with the inferred half carried at that document's §9 item 11. **Neither the row nor the answer reaches what the values are or where they start from.**

~~**Not answered.** Filed unanswered at the ninth sitting, 2026-08-25.~~

**ANSWERED 2026-08-29**, at `Preparation/Architecture/Rendering.md` §6.1 **A1d**, with the page's half at `Preparation/Architecture/Render Settings.md` §1.1 **Q2 item 6**, §2.1 **J3**, §2.4 **J11**, §3 **F5** and §9.1 **Y1**. `Rendering` L10's Simplify bullet, its §6 constraint line, its §8 supply paragraph and `Render Settings` §12's supply line point at it; the narrowed inferred half is `Rendering` §9 item 11 and the new residue is that document's item 23.

**The drawn premise falls, which is what the row was filed on.** Fact 134 came back answered at *What a render demands 6*: a saved scene records Simplify's switch and a **pair** of values for each of the three settings — one for the viewport, one for the final render — readable back with no window open, per scene, surviving saving and an operating-system crossing. **So Simplify is not the one thing Smoozy adds rather than takes out of the file**, and the italic-and-bold convention `Render Settings` J1 builds the page on applies to these three rows like every other. The three drawn constants — `2`, `0.50`, `1024` — come from no document and are refused: no spec names a starting value, and Rule 7 forbids inventing one.

**Three things the answer settles beyond the row's two.** A Smoozy job sets the **render half** of each pair and leaves the viewport half exactly as saved, which is the spec's own *"Three settings are exposed, all from Simplify's render half"*. The **documented defaults are never substituted for a reading** — an untouched scene does not read back at them, observed in 5.2 — so nothing infers from a value whether a file was edited. And the store's Simplify entries are **per setting** rather than one block written by switching on, which the overturned premise had presumed.

**The version claim holds.** 5.2 alone records a texture resolution scale — its own release notes state it as new, and the older three, each asked, answered that the render-side property does not exist — so the Design notes' ground for excluding Texture Resolution is exactly right and the spec's exclusion stands.

**`Rendering` A1c stands under the fact**, checked rather than assumed: a file saved with Simplify already on renders as the artist saved it, because *"It is off unless the user turns it on"* binds Smoozy's own act — the spec's next sentence says so in terms — and forcing a saved switch off would be the opposite fault.

**Three Rule 10 send-backs go to Design**, at `Audits/Architecture/Row 472 — what goes back to Design.md`. **No stop, nothing routed outside the two documents, and no existing clause found wrong.**


**Row 473 — the moment a machine writes into its own message, read by a user in another time zone.** *The phone view — the status message's* Updated *line.*
> `          <p class="tg-at">Updated 12 Aug 2026, 14:22:07 UTC</p>`

**Re-quoted 2026-08-29 at the re-gate. Design has carried the Rule 10 send-back**, so the bare time the row was filed against no longer exists: C2a required a complete date and time with seconds and a visible `UTC` label, and the drawing now carries exactly that, at line 3260 with its companions at 3277 and 3312. **The row was filed on the bare `Updated 14:22:07`**, kept here in prose rather than in quotation marks so the record survives without re-creating the miss. The answer does not move — this is the send-back working.

and the Design note that makes it load-bearing:
> `    <p><b>One message per server, kept current rather than sent again</b>, and it carries the moment it was last written. That moment is the whole mechanism behind a server going quiet: nothing is left to send that news, so a message that has not moved for twenty minutes is what the user reads&nbsp;instead.</p>`

Line 3239 of `Preparation/Design/Mockup/Smoozy.html`, the first status message's `tg-at` line inside `#tgview`; the same stamp is drawn at line 3256 (`Updated 14:22:05`), and bare moments of the same kind at 3289, 3304, 3338, 3356, 3375, 3389, 3400 and 3422. The Design note is line 8011, under *The phone*.

Kind of promise: **a settled value** — it is in the mockup. Not Creator-stated, no date.

**Earns its row by the third test: an outcome that cannot be reached without something else holding first.**

- **Smoozy is forbidden to do the subtraction, so the user does it.** `Preparation/Architecture/Telegram.md` C2 is explicit: *"D5 holds: the moment is one machine's clock, stated as such, and Smoozy subtracts nothing across clocks — the reader of the silence is the user."* The twenty-minute reading is the user comparing that stamp against their own clock. **A stamp written in UTC and read by a user five or nine hours away is wrong by that offset, in the one mechanism this module has for a machine that has died.**
- **The failure is quiet and it runs both ways.** A machine that stopped an hour ago reads as current to a user behind UTC; a live machine reads as long dead to a user ahead of it. **Nothing on the screen tells them which.**
- **C2's own clause requires something the drawing does not carry.** *"stated as such"* is the phrase, and the drawn line states a bare time with no clock and no zone. **If that clause binds the message's wording rather than only that document's prose, then under Rule 10 the drawing is what moves and this is a send-back rather than a row.** The sweep filed it as a row because the substantive half — whether a machine can be given the user's zone at all, and what is shown if it cannot — is reached by no document either way, and that half is this stage's. **The runner decides which half it is answering.**

Waits on: **an arrangement question rather than an outside fact, and there is nothing to get** — whether anything supplies a rented machine with the user's own time zone, and if not, what a moment written on the machine's clock reads as on the user's phone. The machine's clock is UTC and the reading is already recorded, quoted inside `Preparation/Architecture/State, identity and recovery.md` D5: *"The machine is on UTC … It has no idea what time zone anyone using it is in"*. The role configuration a machine is given is enumerated at `Preparation/Architecture/Server Setup.md` G1 — *"the proved conversation identity, the machine's own identity and display label, and the machine-level reporting settings"* — and **carries no zone**.

**Not covered by anything already filed.** Row 265 asks *"where *when it was last updated* is actually written"* and is answered at C2 — into the message's own text, on the machine's clock — precisely so that nothing rests on what a phone shows for the time of an edit (`Telegram` §21 item 6). **Neither the row nor the answer reaches which clock the user is reading it against.**

*(Superseded 2026-08-28 — the line below is what the sweep found, and the answer follows it.)*

**Not answered.** Filed unanswered at the ninth sitting, 2026-08-25.

**Answered 2026-08-28 at `Preparation/Architecture/Telegram.md` §10 C2a.** The machine receives no user time zone and no new time-zone value is added to the role configuration, told-record, board or worker store. Its status message writes a complete UTC date and 24-hour time including seconds, with `UTC` visible beside it; Smoozy computes no age across the machine, phone or desktop clocks. **The bare `Updated 14:22:07` form is refused and sent back to Design under Rule 10: every such line needs the complete date and the `UTC` label.** Nothing is unresolved.


**Row 474 — the menu bar's keyboard shortcuts, drawn in one platform's notation and nowhere else in the project.** *The `sysbar` menu bar at the top of the drawn window — three of the fourteen combinations.*
> `        <button type="button" data-go="set">Settings…<span class="k mono">⌘,</span></button>`

and
> `        <button type="button" data-go="sv">All servers<span class="k mono">⌘1</span></button>`

and
> `        <button type="button">Full screen<span class="k mono">⌃⌘F</span></button>`

Lines 2492, 2524 and 2541 of `Preparation/Design/Mockup/Smoozy.html`, inside the four menus at lines 2485 to 2547. **Fourteen combinations are drawn in all**, at lines 2492, 2501, 2502, 2511, 2512, 2513, 2515, 2524, 2525, 2526, 2527, 2528, 2539 and 2541: `⌘,` `⌘H` `⌘Q` `⌘X` `⌘C` `⌘V` `⌘A` `⌘1` `⌘2` `⌘3` `⌘4` `⌘5` `⌘M` `⌃⌘F`.

*(Two corrections, 2026-08-29, at the second stage-close re-sweep. **The line numbers moved** when the Help menu was removed on 2026-08-25 and `Report a problem…` was inserted into the Smoozy menu — five menus became four, and the block lost a net six lines. **The three quoted lines are unchanged and all three still find**; the menu removal moved none of the fourteen combinations, the deleted Help item having carried no key. **And the count said thirteen over a list of fourteen**, which is a slip this change did not cause — the drawing held fourteen on the day the row was filed, and the head table of `Preparation/Architecture/Desktop app and delivery.md` repeated it. K15's answer works through all fourteen acts, so the answer holds and only the number was wrong.)*

Kind of promise: **a settled value** — it is in the mockup. Not Creator-stated, no date.

**Earns its row by the third test.** **Fourteen key combinations** on a drawn interface, all in macOS notation, on an application `Preparation/Architecture/Desktop app and delivery.md` builds for three systems. `Preparation/Design/Whole app.md` settles which **acts** get a key — *"the places in the rail, Settings, quit, and the clipboard three"* — and three rules binding every screen's own set, and it **names no combination anywhere**. So the combinations are promised here and in no other document in the project, and **what a Windows or a Linux user presses is written down nowhere at all.**

Waits on: **Fact 137** — each platform's own keyboard-shortcut conventions for the menu items Smoozy carries, including which combinations each system reserves, and whether the shell chosen in `Preparation/Architecture/Desktop app and delivery.md` exposes an accelerator beside a menu item, on a platform's native menu bar and on the application-drawn one-button arrangement Windows and Linux use. **Rule 2 forbids answering the first half from memory**, and the same question about the menu **bar** was already treated this way: `Preparation/Design/Whole app.md` labels its menu-bar sentence *an assumption* and hands it to Architecture, which answered it at `Desktop app and delivery` §18.1 K1 to K3 from each platform's own documentation. **The accelerators went with nothing.** §18.3 K9 holds *"Wails must expose the platform's native menu bar"* as required property 6 of H6 and works through what a shell that cannot costs; **it says nothing about the key beside an item.**

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered anywhere, and the search was exhaustive.** `Desktop app and delivery` §18 fixes the menus (K5, *"This document builds the integration and adds no item"*), classifies Window (K6), routes cut, copy and paste to whatever has focus (K7), and sends menu item labels through the typography point (K8) — **nothing about an accelerator, a modifier or a per-system mapping.** §21's service table, the closed list of what that document supplies, reads for menus *"the four menus' integration per platform"* with one picker service, and A1 forbids a module reaching the operating system by any other route. *(Three corrections, 2026-08-29 — this paragraph said **five** menus, said K6 classifies **Help**, and quoted §21 as **"the five menus' integration per platform"**. The Creator ruled the Help menu out on 2026-08-25, K5 and K6 were narrowed to four the same day, and §21's table was rewritten with them. **None of it touches this row's finding**, which is that nothing anywhere fixes an accelerator, a modifier or a per-system mapping — that is as true of four menus as of five.)* Every file in `Preparation/Architecture/` was searched for *keyboard shortcut*, *accelerator*, *mnemonic*, *modifier key*, *key combination*, *Command key*, *Control key* and the `⌘` character itself: **no hits of any kind.** `Preparation/Specs/Whole app.md`: no hits — the specs promise no shortcut anywhere. `Preparation/Design/Design values.md`, the single home of this file's values: it holds no key.

**One thing that narrows what is at stake.** K10's shape applies here too — a shell that could not carry an accelerator would cost the key and not a promise, because every act with a key drawn on it has a control of its own elsewhere in the app. **So this is a property of the route rather than an escalation, exactly as the menu bar itself is. What it is not is answered.**

**Not answered.** Filed unanswered at the ninth sitting, 2026-08-25.

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Desktop app and delivery.md` §18.5 K14–K19.

Answered against **Fact 137**, and the load-bearing half is that **the shell's accelerator works only with a platform-native menu bar** — so on Windows and Linux, where the menus are drawn in the web view, the keystroke is caught in the page. The modifier map is stated, and **no combination is invented where a platform publishes none**: Quit and Settings have no published Windows convention, and Hide, Minimise and Full screen have none off the Mac. Settings on Linux takes GNOME's published combination with KDE's variant recorded beside it; the mnemonic, F10 and Shift+F10 conventions are dropped knowingly at K17 rather than by oversight. **One assumption is filed as §22 item 52** — whether the clipboard keys reach the web view on Windows and Linux without a native menu.



**Row 475 — where a problem report lands, and what it is called.** *The report box `#rpbox` — the* Save in *row of the asking reading, and the saved reading.*
> `        <span class="rp-wp mono">/Users/artist/Downloads</span>`

and
> `      <p class="msub" data-rp="saved" hidden><span class="mono">smoozy-report-25-aug-14-32.txt</span> is in your Downloads folder, which Smoozy has just&nbsp;opened.</p>`

Lines 2952 and 2962 of `Preparation/Design/Mockup/Smoozy.html`, inside `#rpbox` at lines 2933 to 2969 — the first beside the label `Save in` and a `Choose…` control at line 2953, the second in the saved reading. *(Line numbers corrected 2026-08-29 at the second stage-close re-sweep: the menu-bar block above lost a net six lines when the Help menu went, and every reference here was six high. Both quotes are unchanged and still find.)* The box is drawn for `Preparation/Specs/Reporting a problem.md` and is drawn **without a Design document under it**: that spec's own *Where the user is offered this* labels the appearance **settled at Design** (2026-08-25), and the drawing is the whole of that settling.

Kind of promise: **a settled value** — it is in the mockup. Not Creator-stated, no date.

**Earns its row by the third test.** *Where the report lands* is named as this drawing's to decide, and it decides two things. **A file Smoozy composes and writes into a folder the user chose is a local write like any other**, and every other local write in this project is arranged — where it goes, what happens when the name is taken, and which clock stamps it. The spec settles the act, the address and the folder being opened; **it settles neither the destination nor the name.** And the name carries a moment, `25-aug-14-32`, **with no clock named**, on the one document in the project that states outright that *"The user's computer clock is undefended"* and requires every moment in the diary to say which clock it came from.

Waits on: **nothing outside to get.** What is missing is a decision nobody has taken, resting on two things that already exist and are not paired.

- **`Preparation/Architecture/Desktop app and delivery.md` §8, the downloads folder** — *"the three per-system routes, proposed and never written into unasked"*, in §21's own words. The drawing proposes that folder as the report's default place. **Whether the report is a thing that folder is for, and what the default is on a system whose route §8 resolves differently, is not stated.**
- **§7, the receiving-filesystem collision judgement, and §6, the platform file adapter.** A4 requires every local write to go through the adapter, and §7's *taken* check is what stands between a composed name and an overwrite — but §21 hands that outcome **to `Download`**, whose Replace and Keep rules are the user-facing response built on it. **Nothing owns the report's.** A second report saved into the same folder inside the same minute composes the same name, and **what happens then is written down in no document.**

*(Superseded 2026-08-25 — the line below is what the sweep found, and the answer follows it.)*

**Not answered anywhere.** `Preparation/Specs/Reporting a problem.md` in full: it says Smoozy *"writes a single file, offers a place to put it, opens the folder it landed in"*, and *"Three things keep it small enough to attach"* — **it names no folder and no filename**; its *What Smoozy keeps* section fixes the diary at 20 MB rolling and nothing there reaches the report file on disk. `Desktop app and delivery` §6, §7, §8 and §21: **not one of the three names a report, and no clause pairs them for a file this application composes for itself.** `Preparation/Architecture/Download.md`, which owns Replace and Keep both: its subject is a downloaded unit and its landing place, and nothing in it reaches a file the desktop writes of its own accord. `Preparation/Architecture/State, identity and recovery.md` §11.1's classes, searched for anything holding a report or a diary: **the document predates the spec and holds neither.** `Preparation/Architecture/Coverage.md` and `Facts.md`: no row and no fact carries it.

**Not answered.** Filed unanswered at the ninth sitting, 2026-08-25.

**Answered 2026-08-25 at the tenth sitting.** `Preparation/Architecture/Reporting a problem.md` §7 **V2–V5 and V7–V8**, one clause per half of this row's promise.

- **Where it lands.** Smoozy names the file and **the user picks only the folder** (V2): the box's *Save in* row with its *Choose…* control is a folder choice, served by `Desktop app and delivery` K11's existing picker in its choose-a-destination-folder use, with no save-a-new-file dialog, and K13 judging the picker's path like any other name before anything is written. **The default place is §8's downloads-folder proposal per system, with no special case here** (V4) — and **where §8's route resolves to nothing, nothing is proposed**: the report is then written only into a folder the user chose, and what the *Save in* row reads in that state is Design's to draw ([12](../Reporting%20a%20problem.md#12-unresolved) item 8).
- **What it is called.** A fixed stem, a moment and the plain-text extension (V3); **the moment is the desktop clock's at the save, named as such under D5's presentation use**, because nothing rests on the name — every moment a promise touches is inside the file with its clock stated. **A taken name is never overwritten and never prompted over** (V5): the write goes through `Desktop app and delivery` §7's collision judgement, the taken check is the filesystem's own exclusive create, and on *taken* the name is recomposed with a non-colliding suffix and judged again, so a second report in the same minute lands beside the first under its own name.
- **The saved reading is conditional, which is this row's second quoted line.** The folder is opened with the file shown in it through one named reveal service — `Desktop app and delivery` §18.6 **K20–K24**, written 2026-08-25 *(V7's pointer corrected 2026-08-29; it had gone on asserting the service was routed and unwritten)*. **Every route returns an outcome and it is learned** (V8). **Where the reveal succeeded the drawn *which Smoozy has just opened* sentence may be shown; where it did not, that sentence is not shown** — the box still names the file and the folder it is in, which is the part that stays true, and that wording is Design's ([12](../Reporting%20a%20problem.md#12-unresolved) item 8). Nothing retries and no failure record is raised: the file is written and named, and the reveal is the pointing rather than the act.

---

*Rows 510 and 511 were filed by the re-sweep of 2026-08-31, against the nine drawn states the commit of 2026-08-29 added. **The mockup was swept earlier that same morning, on the older baseline**, so no check in this stage had seen these nine — the sweep had gone stale by eleven Design files within five hours of running. Six existing rows were filed off drawn states exactly like these.*

**Row 510 — the invitation link names the bot the token answers for, and nothing reads that name back.** *The bot's own address, shown to the user after their token is accepted.*
> "read back from the check the previous box ran"

Where it came from: `Preparation/Design/Mockup/Smoozy.html`, the `#stgbotok` dialog, in the drawing's own note beside the `t.me/…` link.

Kind of promise: **a settled value** — it is in the mockup, and the note states the rule outright. Not Creator-stated, no date.

**Earns its row by the third test.** The drawing fixes where the shown name must come from — **the identity check, never the username Smoozy offered** — and the architecture does not supply it. `Preparation/Architecture/Telegram.md` §6 **S3** defines that check and classifies its outcome into three — refused, unreachable, answered — and **reads nothing at all out of the answering case**. **S6** names the link only as a shape and never says which name fills it. So the drawing's rule has no mechanism under it.

Waits on: **an outside fact about Telegram** — whether the identity call returns a username at all, and under what conditions it may be absent. **That is owed against Telegram's own published documentation and must not be answered from memory** (Rule 2). No machine and no rental: this project already holds a bot token, and the same call was exercised for Fact 102.

**Why it matters more than a link.** The pre-existing Settings *Bot* reading rests on the same unarranged read-back, so one answer serves both. And if the name can be absent, the drawing shows a link that goes nowhere with nothing said.

**Answered 2026-09-01 at `Preparation/Architecture/Telegram.md` §6 S3a and S6.** The successful `getMe` response is the authority: Smoozy reads its `id` and `username`, keeps both through the conversation proof, and writes them to the local record with the proved conversation identity. The setup link is `https://t.me/` plus that returned username, and Settings reads the same stored value with `@` in front. **The offered candidate is never a fallback.** Telegram's Bot API reference documents `getMe` as returning a `User`, but marks `User.username` optional; its bot guide separately says every bot has a `t.me/<bot_username>` link without guaranteeing the field in this response. So a missing or malformed username is a stated failed check: the paste stays for retry, nothing is stored or delivered, and no unusable link is drawn. `Preparation/Architecture/Sources/Telegram.sources.md` entry 81 carries the two published statements and their limit.

**Row 511 — the byte total drawn beside a leftover request, which no clause composes.** *What the part-sent files are using.*
> "part-sent files are using **210 MB**"

Where it came from: `Preparation/Design/Mockup/Smoozy.html`, the leftover-request state.

Kind of promise: **a settled value** — a figure drawn on a screen.

**Earns its row by the third test.** **`File Browser` U8 supplies everything else that state draws** — its presentation, its composition from durable records, and the remove act — **and no byte total is among what it names.** `Server protocol and transfers` **S9**'s field list is exhaustive and holds an expected total and a checkpointed high-water offset; **what it does not hold is the staged size on the machine**, which **S12 step 1** reads off the filesystem. **For an upload that filesystem is the server's, and this state is drawn with the server not connected.** So there are two candidate numbers, they are not the same number, and neither is composed anywhere.

Waits on: **nothing outside to get** — a decision. Which figure the line states, or that it states none.

**A Rule 10 matter rides with it and is already sent back.** The same drawn line says those bytes are *"on this computer"*, and **S1 and S2 put an upload's staging on the server** while **S28** says it is a *download's* staging that sits on the user's disk. The row's only control is *Remove*, so a user told it is their own disk expects their own disk freed. That is at `Audits/Architecture/The fifteenth sitting — what goes back to Design.md`, item 1, and it does not come back up as a hole.

**Answered 2026-09-01 at `Preparation/Architecture/File Browser.md` §3.4 U8.** The offline leftover line carries **no byte total**. `Server protocol and transfers` S9's expected total is the size of the finished files, not the material staged on the server, and its checkpointed high-water offset is only what the desktop accounted for; S10a observes that it can be either ahead of or behind the staged object's real size. The real size is a filesystem reading the disconnected server cannot supply. U8 therefore composes the leftover's existence, server and remove act from the durable records and does not relabel either candidate as disk use. The already-routed Design correction still stands: an upload's staging is on the server, not *on this computer*, and the line loses the figure rather than replacing it with another guess.
