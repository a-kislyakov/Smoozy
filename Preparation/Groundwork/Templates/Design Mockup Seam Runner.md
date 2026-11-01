# Design Mockup Seam Runner

The brief for running the Design stage's **mockup seams** — each Design document read against the part of `Preparation/Design/Mockup/Smoozy.html` that draws it. It replaces the standing [Runner](Runner.md) for this one job and for no other.

It exists because these rounds differ from every other round in the project in three ways at once: one side of the seam is a drawing rather than a document, the roster is fixed at two named models, and **the round does not stop for the Creator**. Written 2026-08-24 on the Creator's instruction, after the first of the eleven ran the ordinary way and proved the shape.

Copy the block below into a fresh chat. Fill in nothing — it works out its own next Subject from the board.

```
You are the runner for the Design stage's mockup seams. You drive every round from start to finish and you do not stop to ask anything. Read this whole brief before you begin.

## What you are doing, and why it is eleven rounds

The Design stage cannot be ticked until its seams are checked. There are two halves. The documents against each other ran on 2026-08-24 as the thread *what a failure looks like*. This is the other half: **each Design document read against the part of the mockup that draws it.**

It cannot be one round. `Preparation/Design/Mockup/Smoozy.html` is around 470,000 characters and there are eleven Design documents; handing an auditor all of that and the word "consistency" produces the thin, broad findings that oversized Subjects always produce. So it is eleven small pair seams, one per document, plus one final round for the three modules that have no document at all.

**Which side wins is already settled and is not yours to decide.** `Preparation/Design/Mockup/About this mockup.md` says it: *"It is not the plan. The Design documents beside it are… Where the two disagree, the documents win."* So a fix lands in the drawing, and lands in the document only where the document is plainly the stale one. Every finding must say which, and the Verdict must carry it, because the implementer cannot work it out.

## Read these first, in full

- `Preparation/Groundwork/Auditing.md` — the section *Who runs the round: the runner* is your runbook, and the section on **seams** binds this Subject. Read the ruling recorded there for these rounds, which is what allows you not to stop.
- `Preparation/Groundwork/Rules.md`, `Conventions.md`, `Glossary.md`.
- `Preparation/Design/Method.md` — the stage's own method.
- `Audits/Design/Progress.md` — the board. **It says which rounds are done and which is next.** Read it first in any chat and update it last.
- `Preparation/Design/Mockup/About this mockup.md` — the drawing's own conventions.

## The order, and how to know where you are

The board is the memory. It names the rounds already closed. Take the next one from this order and do not re-open a closed one:

1. **Queue & History** — closed 2026-08-24, the ordinary way, with the Creator ruling on two decisions. It is the worked example; read its round folder if you want to see the shape.
2. **Design values** — against the token block at the top of the drawing. Small and mechanical, and every screen leans on it, so it goes early.
3. **Whole app** — it owns the conventions every screen document points at, so settling it early means the rounds after it inherit a correct convention document. Its drawing half is spread across the file rather than sitting in one block: the raw-text panel, the notifications, the menu bar, the password prompt, the update box, the lock band, and the resizable seams.
4. **Servers**
5. **Server Setup**
6. **Render Settings**
7. **File Browser**
8. **Dependency Upload & Remap**
9. **Transfers**
10. **Keys**
11. **Settings**
12. **Access, Rendering and Telegram together** — the three the Creator ruled on 2026-08-23 are *drawn rather than written*. They have no Design document, so this is not a pair seam of the same kind: it is the drawing and its Design notes read against those three **specs**. Say so in that round's boundary, and give the auditors the specs rather than a Design document. It goes last because it needs everything else settled.

## One round, step by step

**Do not stop between steps and do not report progress between rounds.** Run each round to its close, commit, update the board, and start the next.

### 1. Open the round

The tree must be clean and committed. Commit anything outstanding first.

Create `Audits/Design/<Document> ↔ the drawing/Round 1/` and write `Round info.md` in it. Copy the shape from `Audits/Design/Queue & History ↔ the drawing/Round 1/Round info.md` — it carries every section these rounds need. It must hold:

- **The commit** the round judges, in full, taken with `git log --format=%H -1`. Not the Creator's to fetch.
- **The Subject**, and why the drawing counts as a side of a seam at all.
- **The audit boundary** — what this round owns, and what it leaves to the other ten. Always leave out: every other screen; whether either side is good on its own, which is that document's own round; and **the mockup's own machinery**, which `About this mockup.md` says outright is not part of Smoozy — the state bar, the window-width bar, the *Mockup state* list, the *Design notes* button, the line about where a Mac puts its menu bar, and the *Nothing drawn behind this yet* fallback.
- **Why the round is allowed** — first audit of this Subject.
- **Who is doing the work** — the two below.
- **Anything from `Preparation/Groundwork/Open Threads.md`** triggered by an audit event, and anything parked there that names this screen. Check every time; several items are parked against particular screens.

### 2. Write the two briefs

Both come from the same body. Copy `Audits/Design/Queue & History ↔ the drawing/Round 1/Brief — Claude Opus 5.md` and change the Subject, the file list, the drawing's line ranges, the round folder and the model name. **Do not rewrite the body** — it was tuned on the first round and it works.

The two things to get right each time:

**The drawing's slices.** Name the exact parts of `Smoozy.html` the auditor should read, and say the rest is other rounds' work. Every screen sits between two banner comments of the form `══════════════ NAME ══════════════`, and its stylesheet sits under a comment naming it. **Find them fresh each round — line numbers drift every time a fix lands** — with something like:

    grep -n '══════════════ [A-Z]' "Preparation/Design/Mockup/Smoozy.html"

Give the auditor line numbers *and* the banner text, so it can confirm it is in the right place if the numbers have moved. Include, for most screens: the screen's markup, its stylesheet block, the token block at the top (roughly the first 130 lines), and the **Design notes** near the end of the file, naming the headings that bear on this screen. Add whatever else the screen opens — the raw-text panel, the notifications, a dialog it raises.

**One line the auditor needs and will not work out.** Tell it that every state is drawn in the markup and hidden with `hidden` or a `data-` attribute, that it should read the hidden ones because a hidden state is a drawn state, and that it cannot run the file and does not need to.

Also give it, as reference rather than as the Subject: `Preparation/Design/Whole app.md`, which owns the conventions the screen obeys, and `Preparation/Specs/Whole app.md`, without which it cannot apply Rule 8.

### 3. Run both auditors

**Two seats, fixed. Do not vary them and do not ask.**

**Claude Opus 5** — launch as a subagent with the Agent tool, pointed at its brief file. Tell it three things the brief cannot: that it is running in Claude Code and should say so in its sign-off, that its last act is to save its file into the round folder and confirm it exists, and that its final message to you is one line rather than its findings.

**Gemini 3.7 Flash High** — run it yourself through the `agy` CLI. The model id already carries the reasoning level; do not pass a different one.

    agy --model gemini-3.7-flash-high --effort high \
        --dangerously-skip-permissions --print-timeout 25m -p "$(cat PROMPT_FILE)"

Build `PROMPT_FILE` by pulling the fenced block out of the brief and adding the same three lines you gave Opus:

    awk '/^```$/{f=!f; next} f' "Audits/Design/<Document> ↔ the drawing/Round 1/Brief — Gemini 3.7 Flash High.md" > /tmp/p.txt

Run it in the background; it takes a few minutes and Opus takes longer. `--dangerously-skip-permissions` is needed because print mode would otherwise block on the first file read — it is reading this project and writing one file into the round folder, and the Creator has approved this.

**Confirm both files landed before you go on.** Check each one opens with its "what I consulted" line and confirms it read no audit trail, and that it signs off with the assigned model name.

### 4. Consolidate — and take the Creator's pile yourself

Launch a fresh Claude Opus 5 subagent on `Preparation/Groundwork/Templates/Consolidator.md`. Give it the Subject, the round folder, and "none, this is round 1".

**This is where these rounds depart from every other round in the project, and the departure is the Creator's own instruction of 2026-08-24: the round does not stop.** Tell the consolidator so in its brief. It still does everything else the template asks — cluster, check against the Rules and Conventions, sort into three piles, write the Verdict. What changes is the middle pile:

- **It puts nothing to the Creator and waits for nothing.** Every finding that would have gone to them, it decides itself.
- **It decides them the way the project already decides a handed-back choice**: take the option that changes least, and the option the plan's nearest already-answered question points at. That is the standing guidance of 2026-08-23 and this is the same rule at a larger scale.
- **Every such call is written into the Verdict as its own item, marked plainly as the consolidator's, with the reasoning it would have given the Creator and the alternative it turned down.** These go at the top of the must-do list, not buried among the small fixes. The Creator reads them afterwards and may overturn any of them; the trail is what makes that possible, so it is not optional.
- **The cap is gone with the gate.** The two-item ceiling existed to protect the Creator from ruling on a dozen things a round. Nothing is being put to them, so nothing needs capping — but the reasoning behind each call still gets written down in full, however many there are.

Everything else in the Consolidator template stands, the plain-English rule hardest of all.

### 5. Implement

Launch a fresh Claude Opus 5 subagent on `Preparation/Groundwork/Templates/Implementer.md`. Give it the Subject, the Verdict, and the decisions records — the seam's own, which does not exist on a round 1 and which it creates, plus the record of any already-audited document it edits.

Six things to tell it every time:

- It is running in Claude Code, for its signatures.
- It is editing a drawing as well as documents, and `About this mockup.md` holds that file's conventions. **Two of them bite:** a new screen's classes take a prefix of their own, and it has gone wrong twice by reusing a name that already existed — so grep for a class name before using it. And joining a control up takes no code, only `data-go`, `data-panel`, `data-sec` or `data-modal`.
- **Every new state it draws needs an entry in the *Mockup state* list** in the title bar. The bar along the bottom is built from that list, so adding to the list is all it takes.
- **Rule 9 binds every word it puts on a screen**, and that rule's own text says it decays inside a sitting — so check the strings at the end rather than trusting yourself during.
- **Verify the drawing before finishing**: load the file, step the state select through every option, watch for JavaScript errors and check the new states show what they should. Report the state count.
- **Do not touch any document under `Preparation/Architecture/`.** Rule 10 makes this binding. Where a round needs the architecture to know something, the Verdict files an Open Threads item for the architecture runner instead — see below.

### 6. Close the round

Check the implementer's work against the Verdict: every must-do done, nothing added that the Verdict did not ask for, the decisions records matching what happened. Report anything off and correct it; a close check never opens a new findings list.

Then the paperwork the Verdict put on other documents:

- **Open Threads gets an item for every piece of work this round put on a document outside the Subject**, with the Due line that file requires. Only someone already working on a Subject ever opens its Verdict, so anything owed elsewhere has to be written where it will be found.
- **Anything the round wants from Architecture goes there too, as a note for the architecture runner, due at the mockup sweep that stage already owes.** Never as an edit. Say plainly whether it is a note or a hole — a construction the architecture already supports but does not state is a note, and nothing waits on it.
- **A new Blender question means two entries, not one**: the register, and the *questions added after a version was finished* section of `Preparation/Blender Research/Coverage.md`. The second is the one that gets skipped.

Then commit — one commit per round, the Subject's edits and the whole trail together, with a message saying what was settled rather than what was edited, naming both models.

**Then update `Audits/Design/Progress.md` and start the next round.** The board is your memory: it must say which rounds are closed, which is next, and anything settled that a fresh agent must not re-litigate. Keeping it current is what lets this chat be closed at any point without losing the thread.

## When all twelve are done

Say so, once, and stop. The Design stage's seams are then checked and the board should say what the stage still owes before it can be ticked — read its own finish test in `Preparation/Design/Method.md` rather than assuming it is nothing.

## Things learned the hard way, which will cost you an hour each if you skip them

- **The mockup is full of non-breaking spaces and real em dashes.** A string you copy out of a browser or retype by hand will not match the file. Every exact-match edit that failed on the first round failed for this reason. Read the line with its escapes shown before you match on it, or match on a short unambiguous fragment.
- **Line numbers in `Smoozy.html` drift with every fix.** Re-derive them for each round and each brief; never carry them from a previous round's notes.
- **A `data-` attribute can carry more than one variant name**, space-separated, where several states share almost everything and differ in one block. The queue and history entries do this. It beats a fifth copy of every group. If you add the pattern anywhere new, guard for the attribute being absent — that broke eight states on the first round.
- **Checking for layout overflow needs a baseline.** The stage scrolls at the wide widths whatever you do, because a window wider than the pane is drawn scaled down. Comparing raw counts against nothing tells you nothing. Write the previous commit's file out with `git show <commit>:path`, load it, run the same sweep, and compare per width.
- **Verify by running every state, not by reading.** The first round's repair broke eight states with a one-line change that read perfectly. Stepping the state select through all of them catches it in seconds.
```

---
