# Method

*How the plans become the app: who does the work, what is written where, how it is checked, what a builder does when it meets something the plans left open, and the moments that reach the Creator.*

Everything about **what** to build, and in what order, is the [Plan](Plan.md). This document is about how the stage runs. The two are separate because the plan will change as the app is made and this will not.

*2026-10-27 — Removed the obsolete Access routing example with its paid plan.*

## Starting or resuming the stage

**"Let's build", or anything close to it, makes you the build runner.** Your first act in any chat is to read the board — `Audits/Build/Progress.md` — which says where the work is, what the exact next action is, and what is waiting on the Creator. Your last act is to update it. That is what lets a chat be closed, or the work move to another tool, without anything being lost.

Then read the [Plan](Plan.md), and start where the board says.

**Before you write any code at all, read three things once.** They are short, they are written for you, and every one of them exists because somebody would otherwise discover its contents the hard way.

- `Preparation/Architecture/About this folder.md` — the front door to the plans. It says which of the forty-five files are plans and which are frozen evidence, and it has a table that turns *I have been told to build X* into *start at Y*. An evidence document is what somebody watched a real machine do. It is never a plan and must never be built from.
- `Preparation/Architecture/What Build inherits.md` — everything Architecture left open on purpose, with what breaks if it turns out badly.
- `Audits/Blender Research/Cross-sweep/Mechanism sweep/Downstream after the repairs — Claude Opus 5.md` — **the one file under `Audits/` a builder may read.** The Blender comparisons were repaired after the check that had ruled every sentence resting on them sound, so about forty sentences in the plans no longer say what the research says. That file is the list. Nothing under `Preparation/` holds it. Read that one file and no other under `Audits/`, however useful its name looks.

Everything else under `Audits/` is the checking record and is not a builder's.

## What Build makes, and what it does not

Build makes the app, in `App/`, organised by module the way `Preparation/` is. It makes nothing else.

**It does not re-decide what the plans decided.** The Architecture stage closed on 2026-09-02 with 507 live promises, 499 of them answered and the other eight unresolved with what breaks written down, and its answers are the instructions. Where a plan says how something is done, that is how it is done.

**It does not go looking for Blender facts, and it does not rent a machine to answer a question.** The Creator ruled on 2026-08-31 that there will be no further Blender research and no rental for answering a question — and, the same day, that Build may rent a machine to **run the app on**. The two are different acts and the line between them is what the rental is for. So a thing that wanted either is **permanently unresolved rather than pending**, and what a builder does with one is the section below. Blender facts that are already answered are looked up the way the plans look them up: start at `Preparation/Blender Research/Across topics/The map.md`, read the joined answer it names, and read `Preparation/Blender Research/Across topics/What the research cannot say.md` before leaning on any of it. Nobody here assembles a Blender answer out of several topics themselves.

**Facts about everything Blender is not** — a provider, an operating system, a shell, a library, a standard — are got the way Architecture got them: from the thing's own publication, quoted, with the address and the date read, and never from an agent's memory or from whatever is lying around the open web. A thing's publication is not only its manual: release notes, protocol files, changelogs, package manifests and headers shipped on the disk count the same. Rule 2 binds here exactly as it binds everywhere else.

## The rule that matters most, because a builder meets it hundreds of times

**Twenty plans end with a numbered section of what that plan leaves open, and there are a few hundred numbered entries between them.** A figure is derivable — it was 462 on the morning of 2026-08-31 and 463 by that evening, when one was filed — and **that is the point: it moves every time this stage or the one above it writes an entry, so a number written here goes stale by the ordinary work rather than by neglect.** **How many of them are open is not derivable, because closure is written into the entry in prose rather than marked** — an entry may open with *CLOSED*, or carry its closure three paragraphs down, or narrow rather than close. **Two figures are in circulation — 339 and 328 — from two hand-passes on different days, and neither has been re-derived.** Do not repeat either as though it were counted. **The number is not what matters and no work turns on it.** What matters is that a builder meets whatever it meets, and the shape below is how it is met.

This is not a backlog and it is not a set of holes. The Architecture stage's own rule was that a promise may close unresolved as long as what breaks is written down, and most of these are exactly that, working as designed.

**When you meet one:**

1. **Read the entry.** Most say three things: what is not established, what breaks if it goes badly, and what holds without it.
2. **Build the *what holds*.** That is the answer. It is what the stage decided the app does in the absence of the missing thing.
3. **Do not invent the missing thing, and do not build a rescue for it.** Rule 8 governs: judge Smoozy by its promises. A safety net nobody asked for is a want, and Rule 7 says what happens to a want.
4. **Record that you met it**, in the module's decisions record, with what you built.

**A large minority of the entries do not have that shape, and the two exceptions matter more than the rule.** Added 2026-08-31, out of the audit of this document's first draft, which measured them: on one matching, about a quarter of the open entries carry no *what holds* limb in any wording, and some carry neither limb. `Security and licence` §14 item 9 is the whole entry — *"Every Linux claim in §3.3, never observed."*

- **An entry with no *what holds* does not license you to invent one.** Step 2 has no referent, so the answer comes from the clauses the entry points at, read directly: **the document's own answer stands as written and you build that, with the entry's caution recorded beside the code.** Where the entry names no clause and the document genuinely does not say what to do, that is the *plan is the bug* case below and it goes up — it is not a gap for a builder to fill quietly.
- **Some entries are not about the app at all.** `Telegram` §21 item 16 and `The shape`'s single entry are routing notes addressed to the architecture runner — something owed in another document, or a record to be tidied. **A builder builds nothing for one of these.** Note it and move on; it belongs to that stage's work list, not to this one.

**Where the entry turns out to be wrong** — the app cannot be built the way the plan says — that is the *plan is the bug* case and it goes **up**. `Preparation/Groundwork/Auditing.md` owns it and the rule is: never patch the code to paper over a bad plan. Fix the plan document, record why, re-check whatever depended on it, then bring the code into line.

## Where the app is built, and it is not all in one place

**Creator-stated, 2026-09-02 and 2026-09-03.** This is a standing rule of the stage, not a note about one sitting, and it is here rather than on the board because a live tracker is the wrong home for something that does not change.

**The Mac is built at the desk.** The Creator's own machine carries the whole toolchain and builds, tests and bundles the application in well under a minute. **Nothing about a macOS build ever goes to a hosted runner.** A macOS runner minute is published at ten times a Linux one, and it buys a result the desk already produces for nothing.

**Windows and Linux are built on hosted runners, because this project has neither machine and no rental will answer for one.** That is the whole of what the runners are for.

**ONE PART OF WINDOWS CAN BE CHECKED AT THE DESK AFTER ALL, AND NOBODY HAD USED IT. Added 2026-09-20, out of a hosted run that failed for a missing `#[cfg(unix)]`.** The machine's program — `smoozy-server` — **cross-checks for Windows on this Mac in seconds**, the `x86_64-pc-windows-msvc` target being installed: `cargo check --target x86_64-pc-windows-msvc -p smoozy-server`. **The program never runs on Windows, but the hosted Windows job builds the whole workspace, so it must still compile there**, and a missing gate stops that job before it reaches anything else. **The whole workspace cannot be checked this way** — the desktop crate's cryptography wants a C cross toolchain — **and that difference is exactly the freedom the machine program's split bought.** So: **anything that touches the machine program is cross-checked here before a runner is started.** A hosted run is not the place to find a missing gate.

**A runner is started by hand and never on a push.** The workflow carries no push trigger and none is to be added. Start one when there is a question worth asking — a build step closing, or a change that could behave differently on a system nobody here can run — and not otherwise. ~~A run costs about fifteen minutes across the two systems.~~ **A run costs about fifty runner minutes, not fifteen. Corrected 2026-09-07 from a measured run**: Linux 22 minutes and Windows 32, and the two are metered separately rather than sharing one wall clock. The fifteen was written before the workflow kept its artifacts and before the workspace was this size, **and a figure like this goes stale by the ordinary work** — so treat it as an order of magnitude and read the run's own timing rather than this sentence.

**WHEN A RUN IS WORTH STARTING, NARROWED 2026-09-27, Creator-stated:** *"it should be safe to run GitHub builds a little less frequently… I think we can speed things up."* **A hosted run goes before a real release and at a step's end, and not before each test pass on a rental.** A test pass uses only the Mac app and the Linux machine program, and since B10a the machine program for a pass is built at this desk. **The desk's Windows cross-check of the machine program above still goes before every push that touches it**, because that is the one Windows fault the desk can see.

**A RUN IS WATCHED ON A CLOCK, NEVER ONLY BY WAITING FOR IT TO END. Creator-stated, 2026-09-21.** Two Linux jobs hung that day for two and a half and three and a half hours, and nobody noticed until the Creator asked: the runner had left a wait that reports when a run ends, and a hung run never ends. **So every wait on a hosted run carries its own deadline** — about fifty minutes, a normal run being about forty-five — **and when it fires, the runner reads each job's own state and timing.** A job past about an hour is treated as stuck: its log is read for tests *running for over 60 seconds*, and it is cancelled. **The workflow also carries a per-job ceiling** so a hang ends itself; the ceiling is the backstop, the clock is the habit. **One trap, met the same day: the `timeout` command does not exist on this Mac**, so a wait wrapped in it fails at once and silently, and looks exactly like a wait that finished. Use a loop that asks the run's own status every couple of minutes until it is done or the deadline passes.

**A TRANSFER'S PACE ON A RENTAL IS NEVER BLAMED ON SMOOZY WITHOUT THE LINE MEASURED BESIDE IT. Creator-stated, 2026-09-28:** *"Every time there's an upload or download speed discrepancy, make sure you test my internet connection AT THAT EXACT MOMENT before jumping to a conclusion that Smoozy is at fault here."* The Creator's Mac moves between a room near the router and one where the signal is poor, so the line can be bad and then healthy ten seconds later. **So every timed upload or download on a pass is paired, at the same moment, with the Mac's own line measured (macOS's network quality test) and a plain copy of a file to the same machine over the same route**, and all three are written down together. **A pace finding without that pair is not a finding.**

**Money.** Runner minutes are metered on a private repository, against an allowance the account holds. That allowance is the Creator's to see and an agent has no route to it that does not change a permission on their account, which [Rule 4](../Groundwork/Rules.md)'s fourth escalation forbids. So the discipline is not to spend carefully against a number nobody can read — it is to run the thing only when the answer matters.

**What the desk needs, named rather than numbered.** Rust through `rustup`, so a toolchain update is one command and never a reinstall; Node, only because the shell's own command-line tool is fetched with it; and Apple's command line tools. **No version is written here on purpose** — a number in a document goes stale by the ordinary work, and `rustup show` and the workflow say what is actually in use. **The Creator asked on 2026-09-03 that the desk stay on the current stable of everything**, so an update is ordinary maintenance rather than a decision, and the check that matters afterwards is that the application still builds, tests and runs.

**Dependencies are pinned to an exact version and bumped deliberately.** The shell and its build helper are pinned in `App/Cargo.toml` with `=`, the shell's command-line tool is pinned in the workflow, and `App/Cargo.lock` is committed. **This is not an accident to be tidied away**: two builds of the same commit have to produce the same application, and the size comparison the shell was chosen on is only reproducible because the versions are fixed. **A builder that loosens a pin has broken that, whatever else it gains.** Bumping one is its own act, with the application rebuilt, tested and run afterwards — and it is a Rule 7 change to the module that carries it.

**And the builder runs the commands, not the Creator.** Creator-stated, 2026-09-03: they do not work from a terminal, and handing them a command to paste is handing them the agent's own job. Whatever needs running, the agent runs.

## Writing the code

**The code is held to four things at once** — the Specs (does it do what we said?), the Design (does it look like what was drawn?), the Architecture (is it built the way the plan says, and does a deviation cost maintainability?), and plain correctness. That is `Preparation/Groundwork/Auditing.md`'s list and Build does not add to it.

**A clause marker is the unit.** Every plan labels its clauses — `L8`, `H5b`, `D10`. Where a piece of code keeps a clause, the code says which clause, by document and marker, because a marker alone is unrecoverable — `M1` is defined in eight different documents. That one habit is what makes the gate below possible.

**Rule 9 binds every word the user reads.** Standard software words, never a metaphor, and vague is the real failure rather than ugly. Rule 1 binds everything else an agent writes.

**Rule 1's technical-word exception does not reach this stage.** It covers the Architecture documents and their trail and says *nothing else*. So Build's own documents are plain English, and where one must name an exact thing — a shell, a file, a marker — it names it exactly, because a name is not jargon. Code and its comments are not documents and carry no such duty.

**Code is signed through its commits**, never a signatures file, and every commit message names the model that did the work. A commit names the files it is committing, never the whole folder.

## Checking: one gate that is counted, then the ordinary loop

**The gate: traced, both ways.** Every clause the plans state has code that keeps it, and every piece of code names the clause it keeps. A clause with no code is a promise nobody built. Code that keeps a clause and does not name it is a promise nobody can find again.

**The unit is the clause marker, and it is genuinely countable** — a little over two and a half thousand across the twenty plans, unique inside each document — **so the forward leg is mechanical rather than a slogan.** The exact figure is not written here for the same reason the entry count is not: it moves whenever a clause is added, and a gate that reports against a stale total is reporting against nothing. **The gate derives it when it runs.** The gate is copied from Architecture deliberately, because that stage measured what counting was worth: it found what reading did not, over and over.

**What the gate does not cover, stated here so nobody reports a pass it did not earn.** Added 2026-08-31, out of the audit of this document's first draft.

- **A plan whose sections carry no clause markers cannot be counted this way, and it is read section by section instead.** The gate's report says which sections were read rather than counted, and **a gate report that does not say this has not run.** **The gate finds them rather than trusting a list here**, because which plans they are changes: `The shape` has never had markers and is not expected to, being a shape rather than a set of clauses; `State, identity and recovery` had them in three of its twelve sections when this was written, and adding the rest is Architecture's own outstanding work, so a builder may meet that document either way.
- **The reverse leg is narrow on purpose.** It asks whether a piece of code names *a clause*, not whether it names *something*. Code that keeps no clause is allowed — plumbing, a loop, a test — and is not a finding. What the leg catches is code that implements a promise while pointing at nothing, and behaviour the plans never asked for that has been given a plausible-sounding justification instead of a marker. **It does not catch a wrong marker**, only a missing one; the audit catches wrong.

**What the gate does not ask is whether the code is any good.** That is the audit's, below, and a gate claiming to do both is two agents guessing differently at the one check meant to be certain.

**A fresh agent runs it over a module as soon as that module is finished and before it is audited**, and again at the stage close. The build runner does not run it, because it would be marking its own work.

**Then the ordinary loop.** Same four roles, same rounds, same Creator gates as every other stage, with the three differences `Preparation/Groundwork/Auditing.md` already names for code: what it is checked against, whether the auditors run it as well as read it (a per-round choice, because the parts that rent and drive machines cost money to run), and signing through commits.

~~**Build owes the code-audit briefs.**~~ **WRITTEN 2026-09-03, when the first module was ready to audit, which is when this said they should be written.** They are [Code Auditor](../Groundwork/Templates/Code%20Auditor.md), [Code Consolidator](../Groundwork/Templates/Code%20Consolidator.md) and [Code Implementer](../Groundwork/Templates/Code%20Implementer.md), and the gate above has one too — [Build Gate](../Groundwork/Templates/Build%20Gate.md). The pending marker in `Preparation/Groundwork/Auditing.md` is cleared, which is what says this is no longer owed.

**Scenarios stop being audits and become tests.** `Preparation/Scenarios/` holds eight. At every other stage they were played through documents; here the finished app has to pass them. **How much of that eight has ever been replayed, and what each replay left open, is `Audits/Architecture/`'s record and the board's to carry** — it changes as the work goes on, and a count written here would go false inside this stage. What binds is that all eight are tests before the stage closes.

## The moments that reach the Creator

**Rule 4's waiver was written for the Architecture stage and does not carry here.** That waiver existed because the Creator does not read that stage. They will use this one. So the checking half of Rule 4 is back: finish the task in front of you, then check before starting something significant you were not clearly asked to do.

**What does carry, unchanged:** commit and push at the end of every sitting, whether or not anything closed — work that exists on one machine is work nothing else the Creator owns can see. And do not stop to ask about pacing: the plan and the board decide what comes next, not the Creator.

**Five things stop a builder dead** (four until 2026-09-18).

1. **A promise cannot be kept in code.** The plan is faithful and the thing cannot be built. That is a hole, it goes up, and only the Creator rules on a change to what the user gets.
2. **The answer costs the user something no spec says they get** — a place to look, something to type, a thing that can be lost. Same route.
3. **Money at a different scale.** Renting machines, paid services and small change are covered by the Creator's ruling of 2026-08-17 and its five bounds, which the [Architecture method](../Architecture/Method.md) holds and this stage does not restate. A subscription, a plan upgrade, a payment method, or a spend beyond small change is named and put to them. **Build may rent a machine** — ruled 2026-08-31, and the earlier ruling that there would be no further rental stands for what it was about, which was renting to answer a question. Renting to run the app on is a different act. **A rental runs until somebody destroys it by hand and nothing caps it**, so destroying it and checking the account empty is part of the work rather than tidying afterwards.
4. **Anything that touches what is already the Creator's.** Their accounts hold things this project did not make. Everything pre-existing is read-only.
5. **An audit round that cannot seat Gemini.** Added 2026-09-18, Creator-stated. The seats are one Claude Opus 5 and one Gemini 3.8 Flash High; if Gemini fails twice, the round stops before consolidation and the Creator is told what failed and what fixing it takes. No other model and no second Opus fills the seat without their word for that round. The full rule is in `Preparation/Groundwork/Auditing.md`, beside the Build roster.

**EVERY PASS THAT DOES SOMETHING HARD ON A MACHINE RUNS ON EVERY PROVIDER WE HOLD A KEY FOR. Creator-stated, 2026-09-21.** Installing or updating a driver, setting a machine up, rendering — anything past a simple reading — is tried on **Vast, RunPod, Sesterce and Hyperstack**, each its own rental, each destroyed and its account checked empty. *(The keys are in `~/.smoozy-keys/`; RunPod's was added by the Creator the same day.)* **The reason is that providers genuinely differ and the Creator's Mac cannot see any of it**: the first pass on one provider found faults every desk test had walked past. **A pass on one provider proves one provider.**

**What each is expected to show is not the same, and a builder must know it before calling a result a fault.** Vast and RunPod rent a **container**, not a whole machine: the graphics driver is the host's and Smoozy is right not to install one, and on a provider whose start-up registration is a stored field rather than a file at a path, **Smoozy does not come back by itself after the provider restarts it** — `Rented-server runtime` §16 item 16, permanently unresolved, and step 6's qualifier. **Watching that fail there is watching the known hole, not a new fault.** **But a container cannot be placed at all with release 0.1.1. Added 2026-09-21, from the second pass on Vast:** the container's start-up entry is written empty, so the placement's own read-back fails before anything else runs. **Until a release carries the fix, a container pass stops at the placement**, and renting one buys nothing past that. **Corrected 2026-09-22, from the third pass on Vast: 0.1.2 carries the fix.** Smoozy places itself on a Vast container, **and Vast re-runs its start-up file on a restart, so Smoozy comes back by itself there too.** The restart hole is a RunPod expectation, not a Vast one. **And a shell command holding `rm`, `sudo`, `chmod` or `chown` is refused by this project's own settings**, with no prompt shown. A pass that needs a script runs it with `bash`. Hyperstack rents a virtual machine and shows the whole path. What Sesterce rents has not been read by any agent, and the first pass there says.

Everything reaching them is put as a moment: what the user would get, and what they would get instead. Never as a technical choice.

**And nothing else reaches them at all. Creator-stated, 2026-09-03.** They do not read code and have no use for an account of it, so a sitting's findings, decisions, proofs and counts go in the paperwork — the board, the decisions record, the plans — and **not into the chat**. The paperwork is written for the next agent, who is the only one who needs it.

**What the chat says at the end of a sitting is two things: that the work is done, and that a fresh chat should be started.** Nothing about what was built, nothing about what was proved, no numbers, no summary of the work. **A moment that genuinely reaches them under the four above is the one exception**, and it is put in their terms rather than in the work's.

**This is a rule about the chat and not about the record.** Everything a builder would have said is still written down — more of it, if anything, because the board is now the only place it exists. A sitting that leaves the paperwork thin and tells the Creator instead has broken this rule twice over.

## Who drives it

One role carries the stage: the **build runner**. It picks what comes next from the plan, hands out briefs, files what comes back, keeps the plan and the board true, and commits. Everything in this document is its memory, not the Creator's.

**The runner orchestrates and does not build. Creator-stated 2026-09-03, and it is a standing rule of this stage rather than a note about one sitting.**

**Everything that is real work goes to a fresh agent** — building, gating, auditing, consolidating, implementing, checking a round's quotations, renting and destroying a machine. **What the runner keeps for itself is the paperwork and the judgement**: the board, the records, the briefs, the commits, and the calls that are its own to make.

**The reason is not tidiness, it is stamina.** The Creator is away from the computer for hours at a time and will not come back to start the next chat. A runner that builds things itself fills its own context and stops being able to run the night; a runner that only briefs, files and commits can keep going through a whole stage. **So delegation is the first priority, not the last resort** — if a job could go to a sub-agent, it goes to one.

~~**And the runner does not stop.** Not at the end of a build step, not at the end of an audit round, not for a decision. **It stops when the Creator says stop, and at no other point.**~~

**REPLACED 2026-09-07, Creator-stated. The runner works in chunks and checks in at the end of each one.**

**A chunk is one piece of work that ends in something on disk** — a reading written, a piece built and tested, a gate run, a round closed. Not a whole step, and not a single agent. **At the end of one: file the paperwork, commit, say in a sentence or two what landed, and check in.** Then wait.

**The runner must be interruptible between chunks.**

**What does not change: the runner still rules the decisions that would have gone to the Creator**, under the standing delegation, and does not ask about pacing within a chunk. **The five moments that reach the Creator are unchanged.** The board and the plan still decide what to build next.

**The check-in says what the chunk completed.**

- **Decisions the work would have put to the Creator are ruled by whoever is holding them**, under the standing delegation of 2026-08-31 — *"If there are decisions I have to make — you make them by choosing something reasonable, I don't care"* — and written down with the full reasoning that would have gone to them. **A round's consolidator rules its own Creator pile and writes the Verdict straight out rather than waiting.** `Keys` Round 1 and all three of step 4's rounds ran that way.
- **The five moments in *The moments that reach the Creator* still stop that one thing and nothing else.** Park it on the board with what is owed, finish everything else, and carry on. Scaling the work down is the Creator's call and stopping the whole stage for one parked item is not.
- **Renting is allowed while they are away** as long as the brief that rents also destroys and checks the account empty, and the runner holds the instance's own details so it can destroy the machine itself if the agent it started stops.

**What a runner learns the hard way, written here so nobody learns it twice.** On 2026-09-03 the computer rebooted with three implementers running and killed all three mid-edit. **Nothing was lost, and what made that true was the paperwork**: each implementer was working from a Verdict, which is a numbered list of must-dos, so a fresh agent could check the list against the code, find what had landed and what was half-applied, and finish. **An agent that is given a checkable work list is recoverable and one that is given a description of a job is not.** Brief accordingly.

**Keep reports short. Creator-stated, 2026-09-07.**

**The largest avoidable cost is not the work. It is the runner reading long reports and copying them onto the board.** So: **an agent writes its findings to disk and reports back in one or two sentences — what landed, and the sharpest thing it found.** Nothing else. The runner reads the file only when it must act on it, and points the board at the file otherwise. **A finding on disk is worth the same to the next agent whether or not the runner ever read it; a finding pasted into a report is paid for twice.**

**Briefs say whether an agent works alone or may delegate.** Send one or two at a time.

**Plan the sitting’s agents in advance and leave room for the commit and the paperwork.** Stop at a chosen boundary.

**Stop cleanly, Creator-stated 2026-10-30.** Stop at a work boundary rather than mid-way, and file the handover before the sitting ends. Stop delegated work too and keep its partial work, what was learned, unfinished checks and exact next action on disk. Commit and push the saved state where authorised; leave no process or rental without an owner.

**It says when a fresh chat is wanted**, plainly and with the reason, before the chat is in trouble rather than at the point of failure. That is a statement, not a request. What the fresh chat needs to be told is normally nothing: the board is written so the answer is *read the board*.

**Five rules about working beside other chats**, all of which this project has already been bitten by. Only the runner writes anything shared — the plan, the board, a decisions record. Only the runner commits, and **never while an agent it started is running**: a checkpoint commit on 2026-08-31 swept four of another agent's in-flight edits under an unrelated message. And a commit names its files.

**The fourth was added 2026-09-07 and it is the sharpest, because it can destroy work that is not yours and leave no trace. Nobody runs `git stash`, `git reset`, `git checkout` or anything else that moves the working tree while another agent is in it.** An agent measuring dependencies stashed another agent's in-flight file to see whether a failure predated its own change, and popped it back immediately — the file was verified identical, 1,165 uncommitted lines before and after, and nothing was lost. **It reported itself, which is the only reason this rule exists rather than the loss it nearly caused.** The question it was answering is a fair one; the answer is to ask it in a copy outside the repository, which is how every measurement in this stage has been taken. **And that copy is built into its own build folder, never a shared one. Added 2026-10-03.** A builder built its deliberately broken copy of a crate into the shared `App/target-a/`; the build tool took the copy for the same package, judged the real tree's older files up to date, and **kept serving the broken build to the real tree for over an hour** (`Audits/Build/Step 10c/The Creator's notes on 0.1.8, piece 2 — what landed.md`, section 6).

**The fifth was added 2026-09-07, on the second time it happened, and it is about what an agent leaves behind rather than what it does. Nobody starts a wait that has no condition it can reach, and an agent kills every background thing it started before it reports.** Twice now a sub-agent has started a loop to sit while something else finished, watching for a line that was never going to be written, and left it spinning when it finished its own work. The first sat for two hours; the second for an hour and a half, **through the close of a step and past two commits.**

**The loop itself is harmless and is not the point.** The point is that **the agent that started it had already reported and gone, so it was invisible to the runner and showed only on the Creator's own screen** — both times, and both times the Creator is the one who found it. **Nothing in this stage looks for a process an agent left running.**

**So two things bind.** A wait is written against something that will certainly happen, and **an agent that cannot say what would end its wait does not start one.** And whatever an agent starts in the background, it stops before it reports — **because after it reports there is nobody left who knows the thing exists.**

**A sixth was added 2026-09-07 and it is the sharpest of the family, because it defeats the fix for the other two. Stopping an agent does not stop the agents it started.** The runner killed four sweeps, said so, and committed. **Four sub-agents of those sweeps went on running and went on spending** — one of them started forty seconds after its parent was killed. **The Creator found them on their own screen, as with the two orphaned wait loops, and asked what the hell was going on.**

**So: after killing anything, list what is running and kill what is left.** A parent's death is not a family's. **And a brief that hands out sub-agents should say so**, because the runner cannot stop what it does not know exists — three of the four had names the runner had never seen.

**And a killed agent that rents leaves its rental to nobody. Added 2026-09-26.** On 2026-09-23 an account limit killed four agents in one minute, one of them 41 seconds after a RunPod create call answered. **Nobody destroyed that pod; RunPod reclaimed it after nearly three hours**, and three days passed before anyone looked. **So whenever an agent that rents stops without reporting — killed, limited, crashed — the runner's next act is a sweep of every provider's account**, read from each provider's own API, before anything else. Luck is not a safeguard.

**And one thing for the runner: waiting is what the harness already does.** A background command reports back on its own, so a loop that polls for it is a second mechanism doing the first one's job, and it is the second one that gets orphaned.

## When the stage is finished

- Every module in the plan is built, gated and audited.
- Every clause in the twenty plans has code or is recorded as deliberately unbuilt with the reason — and, for whichever plans the gate cannot count over, the reading that stands in its place is recorded section by section. *(This said "the two plans" until 2026-09-02, which was a hand-kept count of the sort the gate's own rule above forbids: only `The shape` carries no clause markers now. The gate finds them itself, so this line names none.)*
- ~~The eight scenarios pass as tests.~~ **Seven of the eight scenarios pass as tests; the eighth, *The user queues multiple jobs overnight*, is recorded as not playable**, because it was retired from replay at Specs on 2026-08-08 (`Audits/Specs/The user queues multiple jobs overnight/Not for replay.md`) and a scenario is never edited. *(Ruled 2026-10-24 by the build runner under the standing delegation, from the step 12 map; if the Creator wants an unattended night tested, they write its replacement, with an agent's help.)*
- **Every word the user reads has been swept against Rule 9 at step 13, the plan's last step** — industry words, no poetry, the shortest sentence that gets the point across. Added 2026-09-29, Creator-stated.
- A search of the whole project for `[PENDING` returns nothing open.
- The app installs, runs and updates itself on each system the release matrix names — and where there is no hardware to prove one, that is written down as unproved rather than passed over.
