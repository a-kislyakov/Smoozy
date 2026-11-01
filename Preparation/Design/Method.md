# Method

*How a hole left in a Design document becomes a drawn answer: what this stage answers, where to start, what Rule 10 makes you read first, what a sitting does, where the Creator comes in, and when the stage is finished.*

*2026-10-27 — Removed the withdrawn paid-access text and its obsolete references under the Creator’s free-and-open-source decision of 2026-10-04 (wave 5); retained protections and evidence keep their force. No auditor has seen this removal.*

The clearest way to see this stage is to follow one hole all the way through it. Take a real one, closed on 2026-08-23. The [Whole app](Whole%20app.md) design document had said for two weeks that it did not settle what a failure looks like:

> a `PENDING` marker asking what a failure looks like wherever it attaches

Nine screens were waiting on that sentence. Each had been inventing its own or leaving it out. Watch what became of it.

*(This file names the marker form but never writes one, so the Conventions' project-wide sweep for unfinished work does not count the examples above as open work.)*

## Design answers the holes it left itself

Every screen was drawn before every question about it could be answered, so each Design document carries a list of what it did not settle, written as `PENDING` markers in square brackets. Those markers are the stage's remaining work, and the specs add a few of their own where a spec handed something to Design by name.

So this stage is not a fresh act of invention. It is a list of questions somebody already wrote down, mostly written by the same agents who drew the screens. Everything below follows from that.

**One marker is one question, and closing it means deleting it.** A marker that has been answered but left in place is worse than one never answered, because the next agent counts it and the board's numbers go wrong.

## Starting or resuming the stage

**"Let's do design", or anything close to it, makes you the design agent.** Your first act in any chat is to read the board — `Audits/Design/Progress.md` — which says where the stage stands, what the exact next action is, and what is settled that must not be re-opened. Read it, pick the screen, say your pick, and start. The Creator should never have to brief you.

**The board is the memory; this file is the method.** The board changes every sitting. This one does not. Where the board's counts disagree with the documents, the documents are right and the board is stale.

## Rule 10 comes before the drawing

**Read the architecture for that screen before you decide anything.** [Rule 10](../Groundwork/Rules.md) closed Specs and Architecture to this stage on 2026-08-23: where a design and an architecture answer disagree, the design changes or the thing is dropped. There is no third ending. A marker answered without reading `Preparation/Architecture/<screen>.md` is an answer that may have to be redrawn, and finding that out later costs more than reading it now.

Two repairs already came out of this and both are in the trail: a failed job's entry named a cause Smoozy is forbidden to work out, and the Server Setup panel claimed it could not read how far a setup got. Neither went back up a stage.

**So the reading order for one marker is: the marker, the spec section it serves, then the architecture document.** If the architecture never touches it, it is yours to decide freely.

## What a sitting does

Six things, in this order.

1. **Decide, and write the decision and its reasoning into the Design document** — beside the material it belongs with, not in a list of answers at the bottom. Then delete the marker. The document is the plan; this is where a decision lives.
2. **Draw it in the mockup** — `Mockup/Smoozy.html`. New classes take a prefix of their own; `About this mockup.md` says why and what joining a control up takes. For `Rendering` and `Telegram` there is no document, so the decision and its reasoning go into the mockup's own Design notes instead — that is the Creator's ruling of 2026-08-23 and it holds for those two only.
3. **Check Rule 9 on every word you just wrote onto a screen.** Standard software words, no metaphor, no idiom. This rule decays inside a single sitting, so check at the end of one rather than trusting yourself during it.
4. **Sign it** — a line in `Signatures/<screen>.signatures.md` saying what you settled and why, and in `Mockup/Signatures/Smoozy.signatures.md` where you drew. Signatures are what let a later agent see how a screen got its shape.
5. **Where the answer changes a spec, write the Rule 7 line** in `Audits/Specs/<module>/Decisions record.md`. Under Rule 10 this is now rare and never a repair going upward — it is only ever a genuinely new thing entering at the stage it touches.
6. **Update the board and commit.** The board is your last act; commit the drawing, the document and the trail together, with a plain-English message saying what was settled rather than what was edited.

The failure marker above went through all six on 2026-08-23. The document gained the shape — a red dot, one plain sentence, a second only where Smoozy knows what to do, then the controls — and the mockup gained the panel behind every control in the app that had promised raw text and led nowhere.

## Where the Creator comes in

**They read the drawing, and they rule.** The signature trail is full of *on the Creator's reading* and *the Creator's ruling*, and that is the shape of this stage: an agent drafts, the Creator looks at it in the browser, and what comes back is often a correction the agent could not have reached alone.

**So do not run a screen's whole list silently and present seventeen settled answers.** Take a batch that hangs together, draw it, and show it. What one ruling changes usually reaches the ones after it.

**And do not stop to ask whether to start.** Which markers to take, and how many, is yours. Only the decisions themselves come back.

**Where a choice is theirs and they hand it back** — *just pick something reasonable* — take the option that changes least, record it where it belongs, and do not explain the reasoning to them. That is standing guidance from 2026-08-23, not a one-off.

## The mockup seams, which are their own job

**Everything above is about closing a marker — turning a hole in a Design document into a drawn answer. The stage's last piece of work is a different job**, and it has its own brief: each Design document read against the part of `Preparation/Design/Mockup/Smoozy.html` that draws it. Eleven of them, plus a twelfth for `Rendering` and `Telegram`, which have no document and are read against their specs.

**Do not try to run these from this file.** [`Preparation/Groundwork/Templates/Design Mockup Seam Runner.md`](../Groundwork/Templates/Design%20Mockup%20Seam%20Runner.md) is the brief, it carries the order the rounds run in, and it replaces the standing Runner for this job. The board says which are closed.

Two things about them worth knowing here, because they sound like they contradict what is above. **They run without stopping for the Creator** — a ruling of 2026-08-24 recorded in [Auditing](../Groundwork/Auditing.md), which the brief carries in full — where every other round in the project stops for a gate. And **which side wins is settled before the round opens**: `Mockup/About this mockup.md` says the documents hold the decisions, so a fix lands in the drawing unless the document is plainly the stale one. That is the opposite of how a sitting above works, where the document is what you write into and the drawing follows.

## When the stage is finished

Three things, and the board tracks all of them.

**Every marker closed** — each screen's own list, and the few the specs still name. Done, 2026-08-23, the keyboard shortcuts with them: three rules in [Whole app](Whole%20app.md) binding every screen, and each screen's own set in its own document.

**The seams checked** — the Design documents against each other, and against the mockup. [Auditing](../Groundwork/Auditing.md) says how. A great many of these documents moved on 2026-08-23, so this is not a formality, and the first two rounds proved it: the documents-against-each-other round changed eleven documents, and the first mockup round turned up thirty-nine findings on one screen. **The mockup half is the section above** and runs on its own brief.

**And the stale prose caught with them.** The documents describe a stage that keeps moving, so a sentence written truly in one sitting goes wrong in the next. It has happened twice to `Mockup/About this mockup.md` alone — once saying the whole-app pieces were owed after they were drawn, once naming three narrowing steps after a fourth was added. **A document that describes the mockup goes stale every time the mockup moves**, so read it against the file rather than trusting it.
