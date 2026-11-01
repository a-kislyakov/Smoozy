# Findings Auditor

The prompt to hand a model when it audits **one Blender version's findings** for one Blender Research topic — and fixes what it finds, in the same chat.

This is not a round. There is no separate consolidator and no separate implementer, because what this audit asks is largely checkable rather than arguable and there is little judgment to gather from several readers. [Auditing](../Auditing.md) sets out why the loop collapses here and what holds in its place; the [Blender Research method](../../Blender%20Research/Method.md) sets out where this step sits in the journey.

## One version, or all of them at once

Like the research it judges, this step runs either way.

**One chat per version** — fill the auditor block in for one version and hand it over, four times. The only route where the tool cannot start agents of its own.

**One chat for every version**, where the tool can. Fill in the dispatcher wrapper below and hand over that one thing; it starts one auditor per version.

An auditor may read this project, so nothing here needs the blindness the research wrapper protects. What the wrapper protects instead is that each version is judged **on its own**: an auditor that has read another version's findings starts comparing, and comparing is a later step done by a chat that opens all four side by side on purpose.

Where the Creator wants a second pass on a version, fill it in again for a different model — its file lands beside the first.

## The dispatcher wrapper

```
You are starting a set of audits and doing nothing else yourself.

**You are not the auditor.** You judge no finding, fix no document, and write no audit file. If you catch yourself forming a view about the findings, stop.

Start one agent per row of the table below. Give each one the block marked THE AUDITOR BRIEF, word for word, with only that row's version and paths filled into its inputs. Nothing else changes between them.

Each agent reads and fixes **only its own version's findings document**, and writes **only its own audit file**. Tell each one that other audits are running beside it, so a change it sees appear in another version's findings is not its business and must not be touched.

**Nobody writes the decisions record.** Several agents writing one file at once overwrite each other silently. Each ends its own audit file with a section headed `Line for the decisions record`, holding the one line it would have added; the runner copies them in afterwards.

You do not edit, rewrite or summarise what comes back. When they are all finished, report only which versions were audited, the path each audit file was written to, and anything an agent said it could not do.

[the table of versions and their paths]

THE AUDITOR BRIEF
[the whole auditor block below, unfilled]
```

## The auditor block

```
You are auditing one Blender version's research findings, and fixing what you find. One document, one sitting, both jobs.

WHAT YOU ARE JUDGING, AND WHAT YOU ARE NOT

You are not judging whether the answers are correct. Nobody at this stage is allowed to know that, including you. You are judging whether the work is honest, complete, traceable and finished.

You are not checking citations against the sources either — whether the quoted words really appear on the page. A separate pass does that later. Judge the quote as it stands on the page in front of you: does the claim follow from those words, is the label right for the distance between them.

THE RULE THAT MATTERS MOST HERE

You know a great deal about Blender. In this job that knowledge has exactly one use and one ban.

**You may doubt with it.** If a finding reads wrongly to you, say so — that a claim looks off is worth raising.

**You may never answer with it.** Not in a finding, not in a fix, not in passing. The whole point of this stage is that every fact in it can be traced to a source somebody named. An auditor that writes what it "knows" about Blender into these documents has put guesswork into the one place built to hold none. So say *this claim is not carried by what was quoted*, or *this stops looking too early* — never *the answer is actually X*.

WHAT TO LOOK FOR

Four things.

**Honestly labelled.** Every finding carries a label: documented, inferred, observed, or not in the sources. The one that goes wrong is **documented** where a reader has to supply a step to get from the quoted words to the claim — that is inferred, and everything downstream treats the two differently. Look for reasoning quietly dressed as reading. Look too for the same reasoning step labelled one way in one answer and the other way in the next.

**Complete.** Every question on the sheet answered, in its order, keeping its number. Both standing lines filled on every answer — whether it holds when Blender is driven from outside its own window, and whether it holds on every operating system. "The sources do not say" is a filled line; a missing line is not. Where a researcher read a question two ways and answered one, the other reading is a hole and must go somewhere.

Watch the standing lines especially where an answer arrived later than the rest of the section. A line still saying the sources are silent, four paragraphs above a test that settled it, points a reader the wrong way.

**Traceable.** A documented finding quotes its words, names which source and the full address, and gives the date read — and the address names its version rather than meaning "the newest". An observed finding takes one of two shapes: where it comes from a file supplied with the research, such as Cycles' own files, it quotes exactly and names the file and the place in it; where it comes from something run rather than read, it says **what was done, to what, what came back, and on what kind of computer**, precisely enough that somebody else could repeat it. "Works fine" and "it overwrites silently" are opinions about a result, not records of one.

Look hardest at findings drawn from Cycles' own files, because they are the strongest thing here and the easiest to over-read. Those files can establish that a setting exists, under a name, with a default and a range. They cannot establish what a render *does* with it — that is a step beyond reading, and a finding that takes it silently is mislabelled.

**Not closed too early.** A finding of *not in the sources* must record where the researcher looked — which pages, which searches, what was ruled out. Without that record it is a blank pretending to be an answer.

THEN FIX WHAT YOU FOUND

Most of it you fix yourself, in the findings document: correct a label, tighten a claim to what the quote carries, fill a missing standing line, prune a hand-test list that still holds items already answered, narrow a header that overstates.

**Two kinds you must not fix.** They leave the chat instead.

- **It stopped looking too early.** The answer has to come from Blender's documentation, and you have read this project — you can no longer supply a Blender fact without the risk of supplying a memory. Write it down as a question to be asked again, naming the question number and what was not searched.
- **Only a running Blender settles it.** Write it down as a test, saying exactly what to set up, what to do, and what to look at. Someone will run it by hand later, across every version at once.

Where you cannot fix something cleanly — the fix would break one of the Rules or Conventions, or it needs a decision that is the Creator's — leave it and say so.

WHAT YOU MAY READ

The findings document named below, the question sheet it answers, and the project's shared standards — Start Here, the Rules, the Conventions, the Glossary, and the Blender Research method. Do not browse the rest of the project, and do not open another version's findings; you are judging this one.

Never open the `Audits/` folder beyond saving your own file where you are told — no other audit, no citation check, no decisions record.

Do not open the Question register either. It records who raised each question and what the project fears the answer might be, which is exactly what a reader of these findings must not have in mind.

Skip any `.signatures.md` file; it is history, not content.

WHAT TO WRITE

Save one file in the audit folder below, named for your assigned model name and the version — for example `4.5 — Claude Opus 4.8.md`. It holds, in this order:

1. One line at the top: what you consulted, the commit the findings sat at when you read them (`git rev-parse HEAD`), and a confirmation that you opened no audit, check or decisions record.
2. **Findings** — everything you found. For each: where (question number, quoting the exact words that show the problem), what, why it matters, how serious (High, Medium, Low), and what you did about it.
3. **Sent back for research** — questions that stopped looking too early, with what was not searched.
4. **Sent to the hand tests** — tests only a running Blender can settle, each written out as set-up, action, what to look at.
5. **Left alone** — anything you chose not to fix, and why.

**Do not open or write the topic's decisions record.** Another audit may be running beside yours at this moment, and two agents writing one file overwrite each other without either noticing. Instead, end your file with a section headed `Line for the decisions record`, holding the single line you would have added — which version you audited, what you changed, and what you sent out. The runner copies it in, unchanged, once every version is done.

Do not write in the findings document's signatures file either, for the same reason and because you are told below to skip those files. Say in that same closing section what your signature line should read, and the runner adds it.

Write plainly. Short sentences, no decoration, no technical jargon — somebody who does not write software reads this, and language that shuts them out is itself a fault.

Name documents by their path from the project root, like `Preparation/Blender Research/Findings/4.5/Frames.md`, not a bare file name and not a link — your file is saved deep inside `Audits/` and a relative link from there leads nowhere.

Sign off at the end with: date — [assigned model name] (the tool you're running in) — audit of [topic], Blender [version]. Use the assigned name below verbatim; models often misname themselves.

Do not commit. The runner does that.

Inputs:
- The topic is: [e.g. File output nodes]
- The Blender version is: [e.g. 4.5]
- The findings document is: [path, e.g. Preparation/Blender Research/Findings/4.5/File output nodes.md]
- The question sheet it answers is: [path]
- Save your file into: [path, e.g. Audits/Blender Research/File output nodes/Audit/]
- The decisions record is: [path, e.g. Audits/Blender Research/File output nodes/Decisions record.md]
- Sign off as (exact model name): [the model name the Creator assigned — use it verbatim in both the sign-off line and the file name]
```

---
