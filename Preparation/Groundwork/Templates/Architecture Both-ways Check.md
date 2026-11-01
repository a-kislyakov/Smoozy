# Architecture Both-ways Check

The prompt to hand a model when it checks **one document** against the rows the Architecture stage filed for it. The [Architecture method](../../Architecture/Method.md) requires this once every document has been swept, and requires it rule **both ways** — *a check that can only add is a check that only ever grows the stage.* One agent per document, and never the agent that swept it. Copy the block, fill in the inputs, and run it.

```
You are checking one document against the rows the Architecture stage filed for it. You did not sweep this document and you are not sweeping it now. You produce a verdict, not a list of rows.

## Rule both ways, and the second way is the one that gets skipped

**What was missed.** A promise in the document that earns a row and has none.

**What does not belong.** A row that is not this stage's work — ordinary construction that is Build's, a decision that is Design's, a product question that is the Creator's, a promise the document does not actually make, or a row that duplicates another.

Report both, and report them at the same bar. A check that only ever adds makes the stage bigger every time it runs, which is the failure this instruction exists to prevent.

## The bar a row is held to

**Three things earn a row.** The document says outright that something is Architecture's. Or it carries a `[PENDING: …]` marker naming Architecture. Or it promises an outcome that cannot be reached without a choice that is not obvious, or that rests on an outside fact, or that needs something else to hold before it can be kept.

**The third test has a floor.** Every spec opens by saying not one word about how it is built, so *this document does not say how* is true of nearly every sentence in it and earns nothing. **A row is owed where there is a decision to take, a fact to get, or a dependency to satisfy.** Ordinary construction is Build's.

**Where the floor sits, exactly.** Read the calibration file named in the inputs, and read the rows it names as kept deliberately at the floor. **Corrected 2026-08-22** — this said the calibration file *kept three rows deliberately at the floor and says in each why*, and it named a file carrying no such labels at all. Five checkers hit that in one sitting and each inferred a different floor. The rows that actually say it in their own words are `Preparation/Architecture/Rows/Servers.md` Rows 25 and 46, `Preparation/Architecture/Rows/Whole app.md` Rows 290 and 291, and `Preparation/Architecture/Rows/Keys.md` Row 245. **A row at or above that line stays. A row below it does not belong.** Judge against those rather than against your own sense of what is important.

## What you must not do

- **Do not write rows.** Where something was missed, quote the promise and say what kind of row it would be. The runner files it.
- **Do not answer anything**, and do not say what you think an outside fact's answer is.
- **Do not judge the document.** A promise you think is wrong is still a promise. That is the audit loop's work, not yours.
- **Do not re-quote or repair anything.** If a quote looks wrong, say so; do not fix it.
- **Never assemble a Blender answer out of several topics yourself.** Where you need one, start at `Preparation/Blender Research/Across topics/The map.md` and go where it sends you.

## Four things that are already known, so you do not report them as findings

1. **Some rows carry a deliberately stale promise quote**, because the Creator corrected the spec after the sweep. Such a row keeps its number, is marked stale, and quotes the corrected text beside the original. **That is correct and is not a finding.** Check instead that the corrected text is quoted faithfully and that the row still describes work this stage owes.
2. **A quote carries the source document's own markup and its links are dead where they sit.** Correct. Not a finding.
3. **A row may cite a fact by number.** The facts are in `Preparation/Architecture/Facts.md`. A row citing a fact that does not exist there **is** a finding; a row citing one that does is not.
4. **The inputs may name questions already queued for you.** Rule on each and say which way you went. Those are the reason this check exists; do not merely repeat them back.

## What you may read

The document, its companion `.sources.md` file, the rows file, `Preparation/Architecture/Facts.md`, `Preparation/Architecture/Coverage.md`, `Preparation/Architecture/Method.md`, the calibration file, the project's shared standards, and the Blender Research map with whatever it sends you to.

**Everything else under `Preparation/Blender Research/` is closed, and so is everything under `Audits/` except the one file the inputs name.**

## How to write it

Use the exact technical word — Rule 1's plain-English rule is suspended for this stage's own documents. Name every document by its path from the project root; do not write a link.

Report, in this order: **rows that do not belong**, each with the row number and why; **promises missed**, each with the promise quoted exactly from the document and one line on which test it meets; **rulings** on the questions the inputs queued for you; and **anything else wrong** that is not either of the first two.

End with two counts: how many rows you would strike, and how many you would add. **If both are zero, say so plainly** — that is a real result and the commonest correct one.

Sign off: date — [assigned model name] (the tool you are running in) — both-ways check of [document name].

Save your verdict as a single file at the path given below. Do not open anything else already in that folder.

Inputs:
- The document is: [path from the project root]
- Its companion sources file is: [path, or "none"]
- The rows filed for it are: [path to the rows file, and the number range]
- The calibration file, which is where the floor sits: `Preparation/Architecture/Rows/Servers.md` (Rows 25 and 46), with `Preparation/Architecture/Rows/Whole app.md` (Rows 290 and 291) beside it
- Queued for your ruling: [the questions already recorded against this document, or "none"]
- Save your verdict into: [path]
- Sign off as (exact model name): [the model name assigned — use it verbatim in the sign-off and the file name]
```

---
