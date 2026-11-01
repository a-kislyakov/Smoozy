# Architecture Sweep

The prompt to hand a model when it sweeps **one document** for the promises Architecture owes it. Building that list is the first work of the Architecture stage, one agent per document, and every agent has to read to the same bar — so the wording below is never rewritten by hand for each one. Copy the block, fill in the inputs at the bottom, and run it.

The [Architecture method](../../Architecture/Method.md) owns what this is for and where the rows end up. This brief is only how one agent is told to do its share.

```
You are sweeping one document for the promises the Architecture stage owes it. You are not designing anything, not answering anything, and not auditing the document. You produce a list of rows and hand it back.

## What a row is

A row is a promise the document makes that Architecture has to answer. It holds:

- **The promise, quoted.** The exact words from the document, not a description of them. Quoted rather than paraphrased is the whole point: everything downstream checks an answer against these words, and an answer to a promise somebody misread is invisible unless the promise sits on the page.
- **Where it came from** — the document's path from the project root, and the heading it sits under.
- **What it waits on** — any outside fact the promise cannot be kept without. Name the fact; do not go and answer it.
- **Why it earns a row** — one line, naming which of the three tests below it met.

Do not number the rows. Numbers are permanent and only the runner assigns them.

## Three things earn a row

1. The document says outright that something is Architecture's.
2. It carries a `[PENDING: …]` marker naming Architecture.
3. It promises an outcome that cannot be reached without a choice that is not obvious, or that rests on an outside fact, or that needs something else to hold before it can be kept.

**The third test has a floor, and without it the list swallows the whole app.** Every spec opens by saying not one word about how it is built, so *this document does not say how* is true of nearly every sentence in it. That alone is not a row. Ordinary construction is **Build's** — exactly as most of how a screen is drawn is. A row is owed where there is **a decision to take, a fact to get, or a dependency to satisfy**. Nothing else. If you find yourself filing a row for every promise in the document, you have dropped the floor.

**Most markers name no stage, and the ones that do are the minority.** Of the two hundred-odd unfinished spots across the specs and the design, only about seventeen name Architecture. So the third test does most of the work here, and a sweep that only collected the named markers has swept nothing.

**Design documents owe far fewer rows than specs, and that is expected.** Design says what the user sees, and most of how a screen is drawn is Build's. What Design does hand over is the handful of things it says outright. A short list from a design document is an answer, not a thin sweep. The same goes for the mockup: it may well owe no rows at all.

## An *unresolved* is checked before it becomes a row

Several *unresolved* labels in the specs are stale — the label sits in the document while the owning sources file and the Blender Research comparison say the question was answered months ago. One of them nearly sent the Creator to rent a server to answer a question twice.

So where the document labels something **unresolved** and you are about to file a row for it, open two things first: the document's companion `.sources.md` file, and whatever that file points at. Where the document is out of date, **say so in the row** — that the label is stale, and what the research actually settled — rather than filing a question nobody needs to answer again.

## A Blender fact is looked up, never asked again

A row that appears to want a Blender question answered is a row nobody has looked up. Where the promise leans on Blender, go and read the answer.

**Where you look is fixed and you do not get to choose.** Start at `Preparation/Blender Research/Across topics/The map.md`. It lists the questions a builder arrives with and names, for each, the joined answer to read first. Where an answer spans topics it has already been joined and checked — read that. Open a file under `Preparation/Blender Research/Comparison/` **only** where the map, a joined answer, or the document's own sources file names it, and read that question alone.

**Never assemble an answer out of several topics yourself.** The one time this project did, a claim that was true of its own test conditions and false as a general rule reached a spec and survived four separate checks. If the map and the joined answers do not settle it, the row says so; it does not say what you think the answer is.

## Facts that are not Blender's

What genuinely falls to this stage is everything Blender is not: the rented machine, the connection, the providers, the operating systems Smoozy itself runs on, the services it talks to. Where a promise rests on one of those, name the fact as something the row waits on. Do not answer it, and do not answer it from your own memory — Rule 2 forbids that, and your recollection of how a provider or an operating system behaves is exactly the kind of confidently-wrong knowledge that rule exists to keep out.

Two things worth flagging where you can see them, because they decide what the stage costs:

- **A fact that needs a rented machine to answer.** Say what the machine would have to be — a card of a particular size, more than one card, no screen — because that decides whether the question is answerable on the machine that gets rented.
- **A fact that decides the shape** rather than a detail: whether something can survive with nobody connected, whether a running thing can be reached and told to stop, and anything else that would make a whole arrangement wrong if it came out the other way.

## What you may read

The document itself, its companion `.sources.md` file, anything the document links to or names in plain words, the project's shared standards — `Preparation/Groundwork/Start Here.md`, `Rules.md`, `Conventions.md`, `Glossary.md` — and `Preparation/Architecture/Method.md`, which says what this stage is for.

Two places are closed. **Everything under `Audits/`**, which is audit trail and none of this stage's business. And everything under `Preparation/Blender Research/` except the map, the joined answers beside it, and the individual comparison questions those name.

## How to write it

The Architecture stage's own documents, this list included, use **the exact technical word**. That is Rule 1's one exception and it is deliberate: these rows are read by the agent that will build the app, and a plain-English paraphrase is ambiguous to it. Quote promises exactly as the document words them, and where you add anything of your own, say it precisely rather than gently. Nothing here is written to be pleasant to read.

**A quote is verbatim, including the source document's own markup.** Copy the sentence exactly as it sits, bold markers and links and all. The coverage gate finds a row's promise by searching the source document for that string, so a quote tidied even slightly stops being findable, and the gate turns from a search into a judgment — which is the one thing it exists not to be.

**A link inside a quote will be dead where you save it, and you leave it dead.** Those links were written from `Preparation/Specs/`; your file sits three levels away. Do not repair them, do not strip them, do not follow them. They are part of the quoted string.

**Everything you write of your own names a document by its path from the project root, as plain text, and never as a link.** That is the Conventions' rule for anything written inside `Audits/`, and it is there because a relative link written from a folder you are not in looks right and leads nowhere. Write `Preparation/Architecture/The rented machine.md`, not a link to it.

## What you do not do

- You do not write into `Preparation/Architecture/Coverage.md`, the board, or any other shared file. Only the runner writes those. You save your own file and nothing else.
- You do not answer a row.
- You do not judge the document. A promise you think is wrong is still a promise, and it gets a row.
- You do not decide what the app should do. Nothing here is a product decision.

## Save it

Save your list as a single file at the path given below, named after the document you swept. Do not open anything else already in that folder.

Sign off at the end: date — [assigned model name] (the tool you are running in) — architecture sweep of [document name].

Inputs:
- The document to sweep is: [path from the project root]
- Its companion sources file is: [path to the `.sources.md` file, or "none"]
- A worked example of this sweep done to the right bar: [path to the calibration document's swept list, or "none — you are the calibration sweep"]
- Save your list into: [path, e.g. Audits/Architecture/Sweep/]
- Sign off as (exact model name): [the model name assigned — use it verbatim in the sign-off and in the file name]
```

---
