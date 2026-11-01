# Diagnostics — what reaches the Creator when a user hits a bug

*A parked want under [Rule 7](Rules.md), with the investigation behind it. Written 2026-08-24 from a conversation with the Creator, checked against the project's documents, then audited six times — five rounds by Claude Opus 5 and one by Gemini 3.7 Flash High — and corrected after each. Every claim about what the project says is quoted with a file and line; line numbers were exact when written and the repository has moved since. Everything under "What I would do" is the runner's opinion and is marked as such, except where the Creator has ruled.*

**Status.** **It entered Specs on 2026-08-25, on the Creator's word.** The document that now binds is [Reporting a problem](../Specs/Reporting%20a%20problem.md), with four changes to the [Whole app spec](../Specs/Whole%20app.md) beside it. **This note is the investigation behind that spec and is not maintained against it** — where the two ever differ, the spec is right. Three further questions were settled on 2026-08-25 and are recorded under *What the Creator settled*. Nothing is built, and what Design and Architecture now owe is in [Open Threads](Open%20Threads.md).

**What this document is.** The answer to one question the Creator asked. It is not a spec, not a design, and not a document any stage owns yet.

---

## The question

Users will hit bugs and want to tell me about them. What do I get to act on?

---

## The short answer

**Nobody has ever asked this question in this project.** Six stages, the project's whole commit history, the whole audit trail — no telemetry, no crash reporting, no bug reporting, no support route, in any vocabulary. (`analytics` does appear four times, always meaning the licence host's own request-timing figures, never Smoozy collecting anything.)

And a crash on the user's computer leaves nothing behind. `State, identity and recovery` **D6** (`:222`): *"No durable state depends on the desktop application shutting down cleanly. Every durable write happens at the moment the fact becomes true, and nothing is written on exit. A crash and a clean close therefore leave the same durable state, and there is no recovery path for one that is not the ordinary path for the other."*

That is deliberate and good. It also means a crash is unrecognisable afterwards.

**The nearest thing to an exception** is `Desktop app and delivery` **Z21a** (`:1143`), which writes a durable record before an update is published *"because the actor watching it can die… the user may quit it, the machine may lose power, it may crash."* That record names the release identity being published. It never says why anything failed. (`State, identity and recovery:353` writes a provisional job record before a submission for a related reason, so Z21a is not unique in shape, only in naming the crash.)

---

## The three things that change the shape of this question

### 1. The Creator has already ruled on this, in the form closest to a report bundle

`Architecture/Telegram.md` **E3** (`:330`), Creator-stated 2026-08-22 — and it *replaced* a rule that uploaded raw machine output automatically:

> **The machine's own words stay on the machine until the user asks for them.**

The reasoning (`:334`):

> "The complaints stream is the user's own text in every way that matters: it carries their folder names, their file paths, their scene and project names, and whatever their own add-ons and scripts printed. Telegram is a third party's servers, and sending that stream automatically publishes all of it on the user's behalf **without the user having said it may**. Nothing in any spec grants that."

One tap sends it. Nothing is trimmed, prefixed or redacted. The Creator added a second ground himself: raw output is hard to read on a phone, so pushing it unasked costs a wall of text as well as the privacy.

**E3a** (`:340`) generalises it: *"No stream, no captured output, no tool's own words and no add-on's or script's printing reaches Telegram except on a tap that asked for it."*

So "the user presses a button, nothing goes automatically" is the ruled position of this project, decided two days ago. **And E3 ranks its own grounds**: `:334` calls the Telegram spec's wording *"the second reason and it is the stronger one"*, above the third-party point. An earlier draft of this note leaned on the demoted one and built an argument on it that has since been deleted.

### 2. Two channels already carry something outward, and both end at me

**The licence check.** Every application start and every six hours (`Access` §7 C1), carrying the licence key or claimed trial start, the local computer identity and the computer name (`Security and licence` §1.2). At the far end my own service keeps a durable slot record — computer identity, computer name, taken at, *"last checked"* (`Licence service:165–168`) — plus a refusal reason code against the licence record, which rule 3 (`:610`) calls *"the record's own state and not a log."* Platform logging was turned off deliberately (`:608`). **Nothing in that document reaches the live stream, and whether it can be prevented at all is recorded unresolved rather than settled** (§15 item 12) — it *"carries every header and the caller's IP to whoever holds a token that can attach to this service — which is the Creator's own account"* (`:612`).

**The update check.** `Desktop app and delivery` **E11** (`:1101`): *"The request says which version is running and which system it is running on, because a manifest cannot be answered without them, and it says nothing else."* To Smoozy's own hosting over HTTPS (**E10**), at application start and on a repeating timer (**Z23**, `:1155`).

E11 is written as a privacy guarantee about what does *not* go up. **Read the other way it is a per-copy heartbeat reaching my own server on every start, carrying no identity at all — and it is the only route that ends at **Smoozy's own hosting** carrying no identity. (Two routes already end at me; the licence check is the other, and it carries the computer identity and name.)**

**One correction to an earlier claim of mine.** I said the project had ruled on what the Creator gets to see. Lighter than it sounded: a consolidating agent made that call and wrote down that it was choosing *not* to escalate it, on the test that the exposure is *"not a place to look, something to type, or a thing that can be lost"* (`Audits/Architecture/Licence service ↔ Security and licence/Round 1/Verdict.md:256`). The Creator has never personally ruled on it.

### 3. Two of the three possible homes are barred, and the third is open

**The failure panel is barred.** `Design/Whole app.md:282` closes the shape — *"Nothing else, in that order"* — and `:286` permits exactly one addition anywhere, already spent: *"That is the only addition anywhere, so nothing else, in that order holds everywhere else."*

**Settings has no drawer.** `Design/Settings.md:17`: *"There is no General section, because a section holding whatever did not fit is an admission that the other four were named badly."*

**The menu bar is the one place left, and something is already sitting in it.** `Design/Whole app.md:44` says the menu bar *"holds only what has no other home"*, and `:51` that *"Window and Help — the operating system's own."* The drawing disagrees: the mockup's Help menu holds one Smoozy-authored item, **"Smoozy help"**, wired to nothing and described in no prose document anywhere. Two auditors read that menu with the menu bar as their explicit subject and neither mentioned it.

That is a small Design defect worth raising on its own, and **it does no work in the argument below.** The menu bar does not stand or fall with it: the **Smoozy** menu already carries Smoozy-authored items in the prose — About, Settings, Check for updates, Hide, Quit — and is not the operating system's own. So if the *Smoozy help* defect resolves toward the prose, the menu bar is still open; only that one item closes.

*(Mockup line numbers are deliberately omitted here: an agent is editing `Smoozy.html` in the background and they drift under the reader.)*

---

## What the project already has

### The rules that would bind a report bundle already exist

A report bundle is an **exported record** in this project's own vocabulary, and that is already a bound category. `Security and licence` §13 rule 3 (`:548`) binds *"any log, message or exported record"*; §3.0 rule 6 (`:180`) binds *"no screen, notification, message, exported record or log"*. Three existing rules cover a bundle without a word being added.

§13's rules reach further than I first said. **Rule 6** already covers ordinary file paths — a failure message *"carries no passphrase, no fingerprint and no path beyond what the user already sees"* (`:551`). Rule 5 refuses a higher log level partly because *"those levels print key paths and identity material."* §3.0 rule 1 (`:175`) is the exact list a bundle rule would want: *"Never a host, a port, a username, a file path, a key comment or a fingerprint."*

`Security and licence` §9 (`:429`) and `Desktop app and delivery` L10 (`:373`) both forbid secrets in *"a log table, a migration backup or a crash record"* — a crash record is named as a thing that could exist. Nothing creates one.

### A capped, retained record of machine output already exists

The raw-text panel tells the user when it filled: *"Only part of this was recorded. 2.1 MB was kept and 41.8 MB was dropped, when the record reached the size limit Smoozy keeps for it."*

That is the shape I was going to propose — a capped record with an honest statement when it overflows — built once already, for the machine's words rather than Smoozy's own.

### The material on the machine

`State, identity and recovery` §1.2 (`:80–97`) lists fifteen machine stores. Several are records of what happened rather than current state: the event journal, the per-job note (*ending, times, re-run marks, the reason a render failed*), the captured streams, the machine's history copy, the operation records, the job record directory with its attempt records.

**Their retention differs and the difference matters.** `Rented-server runtime` **R1** (`:833–842`) gives the captured streams — and the records that inherit its list — an absolute guarantee: they end only when the job's machine-side record ends, and are *"Never ended by an age, a cap, a sweep, a schedule, or a free-space reading."* **The event journal is not in that guarantee.** Its row's cell is empty and it points at E6, which caps it at 32,768 entries and drops the oldest acknowledged entries first (`State…:240`).

### The user's own computer holds more than "state"

Beside outcome records, failure records and cursors, §1.1 (`:46`) names **history records and their tombstones** — an accumulating, per-job, never-swept account. (`Queue & History` N3's ending-and-exit-status wording describes the **machine-side** per-job note, not this record; the desktop's history takes the ending at C11 and the machine's words at E9.). Plus the suppression table, one row per warning ever switched off, which G9 ends only when the user removes them — *"No age, no cap, no sweep"*.

None of it is a log and nothing reconstructs a sequence from it.

### What a user can send me today

A failure's raw text sits one control away, the control naming whose words they are (`Design/Whole app.md:290`), the panel offering Close and nothing else, the text selectable. `:296`: *"It offers no way to save or copy the text, and that is deliberate rather than missed: no spec grants either, the text is selectable, and the menu bar already carries copy."*

**That copy route is deliberate and audited.** An auditor raised it and it was ruled in, on the reasoning that *"pasting a wall of Blender output into a search is the one thing a user actually wants to do with it"* (`Audits/Design/What a failure looks like/Round 1/Verdict.md:243`).

**But copy has no destination.** `Specs/Whole app.md:164`: *"The user downloads a binary from Smoozy's website. No email, no account, no registration. From then on Smoozy keeps itself up to date, so this is the only trip to the website."* There is no support address anywhere in the project. A user who copies is holding text and must find a recipient unaided.

The app names its running version in exactly one place — About, holding the version, the date it was installed, when it was last checked, and nothing else.

**And one thing lands on the rented machine that nobody collects.** Blender writes its own crash file where it dies — `blender.crash.txt` in `/tmp` on Linux (`Blender Research/Findings/5.1/Reports.md:83`). Whether it appears in every case is still an open Blender question.

---

## The failures the project already knows nobody can see

- `Desktop app and delivery` **G2** (`:852`) — a module remembering its own suppressed warnings *"satisfies its own spec and breaks this one, and nobody finds out until a user presses the control and one warning does not come back."*
- **A1** (`:327`) — a module bypassing the adapter boundary: *"the failure is invisible from inside that module."*
- **T23** (`:796`) — *"A post is therefore attempted and forgotten."*
- §22 **item 26** (`:1284`) — a job ending inside any reconnection window produces no notification at all, and the window is routine.
- §22 **item 41** (`:1299`) — losing the offline update recovery key can permanently stop automatic updating — silently, on every copy — once the last authorised online key is also lost or compromised.
- `Download` item 3 with **F14** (`:435`) — a download onto a surviving mount point *"succeeds with no error anywhere… the user plugs the drive back in, finds nothing, and every number on the entry says the files came home."*
- `Open Threads:297` — a disk-full machine kills its own supervisor and *"the arrangement's own death is recorded nowhere Smoozy writes… from outside it is indistinguishable from a machine that is briefly busy."*
- `What Build inherits:69` — an unreadable launch script *rendered anyway, the wrong scene at the wrong size, wrote a file, quit at exit status 0.*

**And the one where the user is left with nothing.** §22 **item 46** (`:1305`): *"On macOS and Linux, a new build that cannot start at all is not rolled back by anything."* Its own Breaks line: *"the recovery half of Row 280 on two of three systems."* Filed as unresolved rather than as a broken promise, with the previous version retained on disk.

**The pattern matters more than any single item.** Every one of these was resolved by making Smoozy *say* something to the user, or by writing down that it cannot tell. **Not one was resolved by recording anything for later.** A black box would be the first thing of its kind here.

### No answer for the architecture's own assumptions failing at runtime

Twelve clauses say some version of *a builder that reaches here has found a fault* (`State…:59`, `Desktop app and delivery` P2, P3, P10, S8, X9; `Servers` D7; `Server protocol and transfers` C6, G1; `Job files and outputs` N2; `Keys`). Every one is addressed to a builder at design time. **None says what happens if the assumption turns out false while the app runs.**

The single exception is **H36** (`:266`) — the web-content boundary fails closed and *"The condition is recorded through the ordinary failure plumbing"*. That records a condition to the user, not a cause to anyone else.

### Build is told nothing

`What Build inherits.md` is 157 lines written to be read cold at the start of Build. **Observability, logging, diagnostics and crash handling appear in it nowhere.**

---

## Where this sits in the process

Rule 8's three promises (`Specs/Whole app.md:61`) are all about the user; none is about the Creator learning anything. **So this is a want under Rule 7, not a hole.**

**It enters at Specs, and all of it does.** Rule 7 (`Rules.md:83`) routes a want to *"Specs if it changes what the user gets, Design if it changes only how the app looks or feels, Architecture if the user never sees it."* I earlier split this — the on-disk record to Architecture as invisible, the control to Specs — and that split does not hold. The user reads the record before sending it, so its contents are not invisible; and what it holds is precisely the privacy decision the Creator personally took in E3. Settling that at Architecture would put it where he could not see it. One decision, entering at Specs.

**There is a precedent.** *Check for updates…* was cut for having no spec behind it and the Creator put it back the same day, the cost recorded as *"one sentence of a spec's"* (`Design/Whole app.md:298`).

**The neighbouring question is already parked, in the Creator's own words.** `Open Threads:903` (`:874` when this note was written; commits have landed since), under **Parked** — *"What the user is shown when the machine misbehaves"*, raised 2026-08-06, due at Design: *"how it will be displayed in Smoozy to the user is also very important… In each case the honest thing to show the user is not obvious, and in some of them the truthful answer is that Smoozy cannot yet tell."* **Its trigger has fired** — it is due *at Design*, and Design is the stage running now (`Start Here.md`, Stage 4 unticked, its mockup seams begun 2026-08-24). It has simply not been lifted into *Due now*, which is bookkeeping rather than a trigger still waiting. It is the near half of this territory — what the *user* sees. A black box is its far half.

---

## What I would do — opinion, not a finding

**The Creator's requirement binds the shape.** Creator-stated, 2026-08-24: *"I don't want users to have to find a file somewhere on their hard drive by themselves and copy certain parts of it. It has to be easy for them."* So any answer where the user hunts, decides which part matters, or reproduces the bug to report it is refused before it is designed. Contents were left to me: *"I think you will know it better than me."*

### The route out — the Creator's shape, and it needs no exception to the boundary

**Creator-stated, 2026-08-24:** *"Can't we just have Smoozy save a file somewhere the user chooses and then gently nudge the user to attach it to an email to hello@smoozy.app?"*

**Yes, and it is better than what this note proposed before it.** An auditor reached the same shape independently, which is worth recording: the answer was found twice, from opposite ends.

**Why it is better is not convenience — it is that the boundary is never crossed, so the whole pile of grants the previous version needed goes.** (Not that nothing at all needs granting: one clause still does, and it is named under *Two honest costs* below.) Every price the previous version carried existed because *Smoozy* did the sending: a second exception to Row 278, a carve-out from Row 239's one always-on service, a fourth domain in a document naming three, an entry on a crossing list calling itself complete, and a whole second service to own. **All of it goes.** On `Security and licence` §1.1's own test (`:27`) — *"Smoozy putting anything of the user's anywhere the user did not open and does not hold"* — Smoozy puts nothing anywhere. The user attaches the file and sends it from their own mail account, which they opened and hold. The same reasoning §1.1 already applies to a rented server, to Telegram, and to the caller's network address.

**And it fixes what was actually wrong with copy.** The objection was never the copying — it was that copy led nowhere. An address on the screen is the missing half.

**It also gives the Creator a way back to the user, which nothing else did.** A mail message carries the sender's own reply address, given by their own act, with no account and no registration anywhere in Smoozy. Under the endpoint version a follow-up question was impossible — E10's *"No account, no registration, no email"* makes such an endpoint anonymous by construction, and anonymous by construction is unreachable by construction. A bug needing one question would have died.

**One correction to something this note asserted twice.** It proposed handing the file to *"the platform's own share act"*. **No share act exists anywhere in the project** — searched, and there is nothing of that kind. **But opening a local folder does exist, and it is Creator-stated.** `Specs/Queue & History.md:120`, 2026-08-07: *"Show a job's downloaded files on the user's own computer. This opens the local folder that job's outputs landed in… It needs no connection and no machine."* It is carried down both stages — `Download` H7 (`:487`), *"One place, and it opens"* — and Design draws it as one of a pair. **So the two pieces the Creator's route needs are both already in the project**: the system file picker, one shared service every module reaches through the same adapter (`Desktop app and delivery` §18.4 K11), and an open-the-folder act with a Creator ruling behind it.

**One honest limit, and it is the project's gap rather than this note's.** No adapter service that opens a local folder is named in `Desktop app and delivery` §21's service list, while A1 (`:327`) requires that a module *"that opens a file, posts a notification, reads a name, draws a menu or opens a picker for itself has broken this boundary."* So H7's *it opens* has a promise and an owner and no named service behind it.

**The refinement worth taking: offer a default place rather than requiring a choice.** The requirement is that the user does not hunt. They will not hunt for a file they either saved to the offered place or put somewhere themselves a moment earlier — but a decision they do not care about is still a decision, and a default removes it while the picker stays for anyone who wants it.

**A share route would be better still where it exists**, since it hands the file to the mail program already attached. It is new work rather than inherited, and it fails on a computer with no mail program configured — likelier on Linux, and on a Mac where the artist lives in webmail. So it is an improvement to consider, not the floor. **The floor is: file saved, address named, nudge given** — and, since the project already opens a local folder for a finished job, the folder opened afterwards so the file is in front of them rather than somewhere they must go and find.

**Two things this floor still owes.** `Design/Whole app.md` gives the reason there is no File menu as *"Smoozy opens nothing and saves nothing of its own"* — this would be the first file Smoozy authors and writes. It forces no File menu and Rule 10 keeps Design out of it, but the clause's stated reason lapses and someone should say so. And the picker that exists is a **chooser**: every use in the project picks existing files or a destination folder. A dialog that saves a new file under a name appears nowhere. **If Smoozy names the file and the user only picks the folder, the existing piece really is enough** — and that is what makes the claim true, so it should be said that way.

**Two honest costs.**

**Settled by the Creator, 2026-08-24:** *"My promise about no email was about registering to use Smoozy, which the user doesn't need to do. So it's all good."* **An address to write to is not a registration, and the promise is untouched.** What follows is the reading that was owed before the ruling, kept because a spec sentence will have to say it.

 `Specs/Whole app.md:164` — *"No email, no account, no registration"* — is about **getting the app**, and an address to write to is neither. That one clears easily. **The other does not clear by itself.** `:115`, in the Security concern: *"Smoozy has no account, no registration and **no email anywhere in it**, so there is nowhere else for anything of theirs to be."* *Anywhere in it* is not about getting the app, and `Desktop app and delivery` §19 cites this same sentence as its own constraint.

**The argument, now confirmed rather than proposed:** that sentence's own stated reason is *so there is nowhere else for anything of theirs to be*, and it is enumerating what Smoozy holds **for the user** — an account, a registration, a mailbox. A support address is none of those; the user writes from their own mail account, which they opened and hold, which is §1.1's test again. **The Creator has since said exactly this**, so the sentence's reach is settled and the address may be drawn.

And an address is a standing commitment to read and answer mail. That is a real ongoing cost rather than a one-off, and it is the Creator's to accept.

### What the record holds

**1. What Smoozy did, in order, with times.** Its own acts, not its internals: connected, read a file, started a job, sent a batch, published a download. The unit is the act the user could name; a record of internal calls is unreadable by me and unshowable to them. Nothing on the desktop keeps this today.

**2. What Smoozy was told** — the endings and readings that arrived from a machine, **and the machine's own Blender version and Linux family**, which nothing else here holds. My acts and the machine's answers together are the thing neither side holds alone. **But not on one clock.** `State, identity and recovery` D5 (`:218`) forbids subtracting readings taken on different clocks and keeps the desktop's clock for presentation only, because the user's computer clock is undefended. So every moment names the clock it came from, and the machine's own order comes from the journal's per-machine sequence number rather than from time.

**Why the Blender version earns its place**: `Specs/Whole app.md:108` records that *"the machine's own memory running out kills Blender 4.5 and 5.0 outright, leaving both streams empty and writing no crash file, while 5.1 and 5.2 say so and stop."* A report of a render that ended printing nothing is a known Blender behaviour on two versions and a possible Smoozy bug on the other two. Without the version the Creator cannot tell which, on a record built to let him act on Smoozy's bugs.

**3. What failed**, as Smoozy already composes it. The failure records exist at `State, identity and recovery` §8 — but §8.2 holds a condition identity and the observed values it was raised over, not prose, and §8.3 makes the identity opaque. **The person-facing wording is Design's and the spec's**, so a bundle either carries the record and its values, or the composed sentence from wherever it was composed. Not no authoring, as this note first said.

**4. Smoozy's own version and the user's operating system.** E11 already sends both to my hosting on every update check, so nothing new is learned about the user. *(An earlier draft said "and the shape of the install", which is defined nowhere and which E11's own list — **no path** — does not cover. Dropped.)*

**5. What Smoozy assumed and found wrong.** The valuable one. Twelve clauses expect a builder to catch these at design time and nothing answers for them at runtime. These are Smoozy's own bugs rather than the user's or the machine's, and today they are silent everywhere.

**6. What Smoozy's own process reported on its way down, where it got the chance.** The note is about a black box and until now its contents held nothing about the crash itself. Item 3's failure records exist for a connection dropping or a write refused; **by construction they do not exist for the case where Smoozy itself died.** Without this the Creator learns only that a run ended during an upload — not whether it was the transfer, the machine running out of memory, or the user force-quitting, which leaves the identical absence of a mark. **It is written when it happens rather than on exit**, so it is D6-shaped and needs no exemption of its own. **Its limit has to be stated rather than hedged.** A process that is killed — by a force-quit, or by the machine running out of memory — runs no code of its own, so nothing writes anything. **Those two stay indistinguishable, and both look like any other death, on the mark alone.** What item 6 can catch is the narrower case of a fault Smoozy sees coming and survives long enough to write. That is *do what can be done, and say what can't*, and the note claims no more than it. **What an operating system actually permits a dying process to write is an outside fact and needs a source before it becomes a promise** (Rule 2).

**7. One sentence from the user, in their own words, written before they send.** Every other item is machine-generated, and for a whole class of bug the user's sentence *is* the bug. Walk this note's own flagship case: a download onto a surviving mount point *"succeeds with no error anywhere"* and *"every number on the entry says the files came home."* The artist finds the empty drive four days later and sends the record. It reads: connected, browsed, published a download, 240 files, all written. **Nothing failed, nothing was assumed wrong, nothing crashed — the record is indistinguishable from a successful download.** Only *"the drive was unplugged and the files aren't there"* makes it a bug at all. The same holds for every case where Smoozy is slow or wrong rather than broken. This note already refuses to cost the Creator a round trip through the user, then gave the user no way to speak.

**What it must not hold** is already ruled: nothing from the secret store, no host key or fingerprint, no licence key, no computer identity — but only three of the four are already covered for an **exported record**. §3.0 rule 6 (`:180`) and §13 rule 3 (`:548`) both name exported records; **§13 rule 4 (`:549`) binds *"any log"* and nothing wider**, so the computer identity would have to be said in a bundle rule or rule 4 widened. One clause, not a hole — but the ground is not already covered, as I claimed. Never the contents of the user's files.

**What it does hold, which is the user's:** file paths, folder names, scene names, hostnames, usernames. That is what makes a report useful and what E3 was rewritten to protect. E3 answered the principle — the user is shown it and taps, nothing trimmed behind their back — and the same answer applies here.

**And it says what it could not include.** `Specs/Whole app.md`: *"Do what can be done, and say what can't."* A bundle assembled with no server reachable holds no machine words, and must say whether they were absent, unreachable, or never collected — `Telegram` E2 keeps those apart one layer down (*"Collected and empty and never established are different things"*) and a bundle that collapses them costs the Creator a round trip through the user.

### Retention — settled by the Creator, 2026-08-24

**Creator-stated:** *"I think we can cap it by size actually."* Asked how big, and having said they had no idea, the number below is the runner's and is theirs to overrule.

**One rolling diary on the user's own computer, capped at 20 MB, oldest lines dropped as it fills.**

**Why a size cap is the right shape and not a retreat.** Three earlier attempts here tied retention to jobs, to history, or to a count of runs, and every one of them failed the same way: it lost whatever happened before a job existed. The artist who crashes three times while getting a fresh machine render-ready has submitted nothing, so a job-shaped rule keeps none of it. **A single rolling diary has no such holes** — there is nothing to reason about, nothing to key on, and no case that falls outside the rule.

**Why 20 MB.** It is negligible beside what this product already handles: single scene files run to gigabytes and a finished sequence is tens of them. It holds weeks of ordinary use in plain text. And it is fixed rather than read, so no free-space reading decides anything — which is what [11.4](../Architecture/State,%20identity%20and%20recovery.md) already requires of every other bounded store on this side.

**It does not disturb the earlier ruling.** §11.1's three endings were put about **fetched preview pictures** and answered with age. This is a different object — plain text, far smaller, and useless once old — and the Creator has now answered this one on its own terms. Neither ruling reaches the other.

### The diary and the report are two different things

**Creator-asked:** *"when a problem occurs and the user wants to report, will only the latest and therefore relevant part of the log be preserved for the email?"* **Yes, and the distinction is the answer to how the email stays small.**

**The diary is what Smoozy keeps. The report is a slice of it.**

- Reporting a **failed job** takes that job's own acts and the session around them.
- Reporting a **crash** takes the session that died.
- Reporting **nothing in particular** takes the current session and the one before it.

**Three things keep the slice small enough to attach.** It is a slice rather than the whole diary. It is plain text, which compresses to a small fraction of its size. And where a slice is still too large, **Smoozy drops from the middle — where the least is happening — and says how much it dropped**, which is the behaviour the raw-text panel already has for a machine's output that would not fit: *"Only part of this was recorded. 2.1 MB was kept and 41.8 MB was dropped."*

**What must not happen is a silent trim.** A report that quietly leaves things out is a report the Creator reads as complete. The statement travels with the file.

### The clean-exit mark

**Write a mark on a clean exit, so its absence is a crash.** Without it a crash is unrecognisable even to Smoozy, and the user must remember days later that something went wrong.

**It is an exception to D6, not something outside it.** My earlier defence — *nothing reads it to decide anything* — was wrong, and my own next sentence contradicted it: something reads it at launch to decide what to put in front of the user, which is the launch-time difference D6's third sentence forbids. The mark still survives on the merits: no durable state depends on it and losing it costs one missed prompt. But it enters as a **named exemption**, and D6's document has been audited, so Rule 7 (`Rules.md:87`) requires a decisions-record line saying so.

**One narrowing is worth taking.** Record *the user asked to quit* at the moment they ask — a fact becoming true, which is the same shape item 6 rests on — rather than writing anything on the way out. The exemption then reaches only D6's third sentence, about a launch-time path that differs, and leaves its second, *nothing is written on exit*, untouched.

### Raising it after a crash, which needs a granted exception

**A user who has to remember to go looking will not**, so Smoozy should raise it at the next launch. **But there is nowhere the project currently lets it appear.** `Specs/Whole app.md:94`, Creator-stated 2026-07-27:

> "It appears on the thing that failed… Smoozy does not interrupt with a box in the middle of the screen, with one exception: where the user is standing there waiting on the answer… **There is no separate place failures are collected instead**; the thing that failed is where its failure lives."

A previous run that crashed is not a thing on any screen. A launch-time prompt is both a box the user is not waiting on and a separate place a failure is collected. **Both halves are refused**, and I tested this only against Design's failure shape, never against the spec clause that decides where a failure may appear at all.

**Granted by the Creator, 2026-08-24:** *"Smoozy may speak up, yes."* **So it enters as a named exception with its cost written down**, the way the Linux update box did — not as something assumed. **Without the grant the only route left would have been a menu the user must remember to open days later — which is the thing that will not happen.** That fallback does not depend on the contested *Smoozy help* item: the **Smoozy** menu already carries Smoozy-authored items — About, Settings, Check for updates — and is not the operating system's own. The precedent this note leans on, *Check for updates…* restored at a cost of *"one sentence of a spec's"*, lives in that menu.

### The rest

**Where a server is reachable, offer that machine's material too** — the journal tail and the per-job note. **Blender's own crash file — and an earlier draft of this note was wrong about it, in the way this note warns against.** It said the project *"says nothing whatever about its contents"*, which is a universal negative about a trail the same note admits it had not read. The trail does say. `Blender Research/Findings/5.1/Reports.md:289` records a crash file *"holding a version line, a section headed `# backtrace`, and a section headed `# Python backtrace`"*, the 5.0 findings say the same, and `Findings/5.0/The machine.md:1065` reads a Python backtrace to identify a third-party add-on's thread. **So the shape is established and it is useful** — a backtrace is exactly what would tell a Blender fault from a Smoozy one.

**What is left is narrower and still real.** `Security and licence` §13 rule 2 forbids the machine's environment being *"captured into any record Smoozy keeps or shows"*, and nothing read establishes whether a backtrace carries environment values. That is one question rather than the open-ended one this note first claimed.

**Item 46 is worth putting to the Creator as more than a want.** A build that will not start, on Mac or Linux, rolled back by nothing, with its own Breaks line naming the recovery half of Row 280. It is also the one case the clean-exit mark cannot reach, because a build that never starts never writes anything.

---

## What the Creator settled

Seven rulings, all of them now written into `Preparation/Specs/Reporting a problem.md`, which is where they bind.

**On 2026-08-25, when the want was taken to Specs.**

- **Reporting is offered in both places** — the Smoozy menu, and on a failure the user is looking at. Asked whether the menu alone was enough, the Creator answered *"Both."* It costs a second addition to the failure shape, which Design must now make.
- **The user is not shown the report before sending it.** *"I don't think sending something they didn't see is a problem in this case, because they don't need it to be readable to them."*
- **The address is real** and the standing commitment behind it is accepted.

**On 2026-08-24, taken after the audit rounds closed**, and each written into the section it belongs to above.

- **Retention: a size cap.** One rolling diary, 20 MB, oldest lines dropped. The number is the runner's and is theirs to overrule; the shape is theirs.
- **The report is a slice, not the diary.** Asked by the Creator, and it is what keeps an attachment small.
- **The no-email promise is about registration**, so an address to write to does not touch it.
- **Smoozy may speak up at the next launch after a crash**, as a named exception to where a failure may appear.

## Open questions I have not settled

Three of the four below were closed on 2026-08-25 and are struck through with what closed them. Two remain.

- ~~Whether the user is shown the record whole before sending, or a summary with the whole behind a control.~~ **Closed.** The Creator ruled the showing unnecessary — *"they don't need it to be readable to them"* — and the reason it holds is stronger than the one this note was weighing. **Smoozy is not the sender.** The consent that showing would have carried is already carried by the user attaching the file to their own mail, so the E3 argument does not reach this case at all. The file is plain text in a folder they chose and Smoozy opens, so looking is available without a control being drawn.
- ~~Whether a share route is worth building on top of the floor~~ — **closed, and refused rather than deferred.** The Creator uses no mail program and reads mail in a browser, and is not unusual in that; a route resting on one being set up would do nothing at all for those users. **Pointing at the file works for everybody**, so it is the ceiling as well as the floor and the question is not open. Label: **Creator-stated** (2026-08-25).
- Whether the Help menu can carry an item at all on Linux and Windows, where the whole menu bar sits behind one title-bar button. **Still open, and now smaller**: the spec puts reporting in the **Smoozy** menu, which already carries Smoozy-authored items, so nothing rests on Help. What is left is Design's ordinary question of where a menu item lives on each system.
- Whether what the machine tells Smoozy, plus the exit status, is enough to tell a Smoozy fault from a Blender fault without the machine's raw stream. **Still open.** The spec's answer for now is to take the machine's own material where a server is reachable, and to leave Blender's crash file unresolved.

---

## Corrections made to earlier drafts

Window and Help are the operating system's own in the *document*; the drawing contradicts it. §13 rule 6 does cover file paths. The question has been asked twice, not once, and `Telegram` E3 is the closer one. The update check also leaves the machine. An agent ruled the network-address question and declined to escalate; the Creator has not ruled it. Only actions whose parts are smaller than the whole write outcome records. R1's never-swept guarantee covers the captured streams and their inheritors — **not the event journal**, which is capped and drops oldest. The Open Threads item is **parked**, not due now. §22 holds a 46-entry list, six closed, about 40 open. `analytics` does appear, meaning the host's own figures. The entry-screen links are `Access.md:99`. `Design/Whole app.md:44` for the menu-bar rule, `Design/Settings.md:17` for the General section. Item 46 does name a Breaks line, so *"breaks no promise"* was overstated. **And the substantive reversals, in order of weight.** The copy route has no destination. The Rule 7 split does not hold — it all enters at Specs. **And the third-party argument for sending to my own hosting is wrong on the project's own test**: §1.1 asks whether the user opened the place and holds it, which puts Telegram inside the boundary and makes *being Smoozy's own* the very property that marks an exception. What I called a route needing no new principle is a request for a second exception to a Creator-stated promise. Also: E3 ranks its grounds and I leaned on the one it demotes; two routes already end at me, not one; §13 rule 4 binds logs and not exported records, so the computer identity is not already covered; and the parked Open Threads item's trigger *has* fired, since Design is the stage now running. **Round five was internal consistency**: the heading claiming the Creator's route needs no grant at all, when one clause still does; the menu bar described as contested when only one item in it is; E3's demoted ground still framed as *the* ground; R1 cited against a cap and then declared unborrowable; and — the worst of them — a flat claim that the project says nothing about Blender's crash file's contents, asserted about a trail this note admits it had not read, and false. **Round three added two gaps rather than errors** — the record held nothing about the crash itself, and nothing in it was the user's own sentence — plus citation repairs: §1.1's test is at `:27` and its Telegram reading at `:31`; Row 239 *forbids* an always-on service and the licence service is carved out of it, rather than Row 239 permitting one; §8 holds a condition identity and observed values, not prose, so the person-facing wording is owned elsewhere; and whether the licence platform's live stream can be prevented is recorded unresolved rather than settled as impossible.

---

## What was checked, and what was not

Five passes. Read in full or near it: `Specs/Whole app.md`, `Design/Whole app.md`, `Design/Settings.md`, `Groundwork/Rules.md`, the mockup's menu bar, About and raw-text panel; `State, identity and recovery.md`; `Security and licence.md`; `Desktop app and delivery.md`; `Telegram.md`; `Licence service.md`; `Access.md`; `What Build inherits.md`; `Rented-server runtime.md` R1; the per-module architecture documents; `Open Threads.md`'s relevant items; `Scenarios/` and `Build/`; `Coverage.md` and `Rows/` by search. `Audits/` holds about 1,334 documents; searches touched several hundred and about seventeen were read in full. Git history searched across the whole repository.

Not covered: the mockup's stylesheet, most of the decisions records end to end, the frozen input documents, and the Blender Research trail beyond the crash-file entries in the 5.0 and 5.1 *Reports* and *The machine* findings. **No universal negative about that trail is claimed anywhere in this note** — an earlier draft made one and it was false. The "twelve clauses" count is confirmed at eight of twelve sites.

**The honest finding: nobody has ever asked this. The absence is real. What earlier drafts overstated was their own coverage of the ground next to it, and their confidence that copying text was a route to anywhere.**
