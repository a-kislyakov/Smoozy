# Comparer

The prompt to hand a model when one topic has been researched for every Blender version, audited, hand-tested and checked — and the answers are ready to be laid side by side. Copy the block below into a fresh chat, fill in the inputs, and run it. What you write then gets a **Comparison Check** of its own, by a different model.

This is deliberately **not** the researcher-runner. That agent wrote the question sheet, handed out the handouts and filed what came back; asking it to write the conclusion would be asking it to mark its own work. Same reason a runner never closes its own audit round.

```
You are writing the comparison for one Blender Research topic. Every version has been answered, audited, tested by hand where nothing else could settle it, and checked against its sources. Your job is to lay those answers side by side, question by question, and say plainly where Blender differs and where it does not.

You are not researching. You never open Blender's documentation, you never open Cycles' files, and you never use your own knowledge of Blender. Everything you need is in the findings. If the findings do not answer something, that is itself the answer and you record it as such.

READ THE HAND-TEST LIST FIRST

The inputs name it. Read its closing notes before you write a word.

Earlier steps in this topic read all the versions side by side and wrote down what they saw — a question two versions answer in opposite ways, a test that could not be run anywhere, a source sentence one version leaned on and another refused. Those notes exist precisely because the person who found them could not settle them, and they are aimed at you. Without them you will meet the same things cold and read at least one of them as Blender changing.

The list also says which tests could not be run at all — no network drive, the wrong operating system, a question nobody could type. Those are silences in what could be tried, never differences between versions, and they must not appear as any kind of change.

A VERSION MAY HAVE MORE THAN ONE FINDINGS DOCUMENT

Where the inputs name two documents for a version, read both. The second is a later pass over some of the same questions, made because the first had not recorded where it looked — not a replacement. Take them together, and where the two disagree with each other, say so plainly rather than picking one. **A disagreement inside one version is not a difference between versions.**

HOW TO BUILD IT

Work through the question sheet in order, keeping every number. For each question, set the versions' answers next to each other, then give one verdict from exactly these three:

- **The same everywhere** — every version answered, and the answers agree. Say what the answer is, once.
- **It differs** — every version answered, and they do not agree. Say what each version does, plainly, version by version.
- **Not established for every version** — at least one version was not answered, or was answered "not in the sources".

THE THIRD ONE IS WHERE THIS GOES WRONG, SO BE STRICT

A version answered "documented" sitting beside a version answered "not in the sources" is **NOT** evidence that Blender changed. It is far more often evidence that one version's documentation is thinner than another's — newer documentation lags, older documentation gets rewritten, and neither tells you anything about the software.

Writing a gap in the documentation down as a difference in Blender is the single most likely error in this whole stage, and once it is written down everything downstream believes it. So:

- Never turn a silence into a difference.
- Where the answers differ **and** one of them is a silence, the verdict is "not established for every version", not "it differs" — however tempting the pattern looks.
- **Exception to the silence rule:** If one version is silent, but another version explicitly quotes a Release Note proving the feature was "added", "removed", or "changed", the release note wins. A documented change in a release note proves a real difference in Blender, and supersedes the silence in the other version's documentation. The verdict becomes "It differs".
- **Second exception, for a test rather than a document:** where the same hand test was actually run in every version and came back differently, that is a real difference and the verdict is "it differs". A test run everywhere is not a silence. Say which test settled it and on what kind of computer.
- Where you can see that a real difference is likely but not established (and no release note proves it), say so in plain words as a note under the verdict, marked clearly as not established. A note is useful. A verdict that overreaches is worse than no comparison at all.

WHAT ELSE TO RECORD

Under each verdict, where it applies:
- **What rests on it** — nothing. Leave that out. The register holds it and this document does not repeat it.
- **Where a version's answer was labelled inferred or observed** rather than documented, say so beside that version's answer. A reader deciding what to build on should see the difference between "the manual says this" and "somebody worked it out".
- **Where a hand-run test settled it**, say which machine it was run on. A test on one kind of computer does not settle the others.

That last point carries more weight than it reads. A result that came back **the same in all four versions still settles only the machines it was run on**. Whether it holds elsewhere is untested everywhere — and untested everywhere is not a difference between versions either. Do not let a result uniform across the versions read as though it were established generally.

**THE MACHINES ARE A SECOND AXIS, AND MIXING IT WITH THE VERSIONS RUINS THE DOCUMENT**

Tests have been run on more than one machine. The inputs name them. This gives you two ways things can differ, and they must never be confused:

- **Between versions** — 4.5 does one thing, 5.2 another. That is what this document exists to report, and it is what the three lists at the top are about.
- **Between machines** — macOS does one thing, Linux another, in the same version. That is an operating-system fact, and it belongs inside the answer, never in the behaviour-differs list.

Two rules follow, and both are strict.

**Never compare a result from one machine against a result from another and call it a version difference.** If 4.5 was tested on the Mac and 5.2 on Linux and they came back differently, you have learned nothing about the versions. That is "not established for every version", and say why in so many words.

**Where the same test was run on both machines in the same version, report both.** If they agree, say the answer held on both and name them. If they differ, say what each did — that is a real and valuable finding about operating systems, and Smoozy renders on the machine that is not the Mac.

At the top of the document, name the machines the tests came from — one line each, what it is, its graphics, whether it had a screen — and point at the equipment section of `Preparation/Blender Research/Method.md`, which holds every machine in full. Do not copy those descriptions across; there is one home for them and fifteen copies drift. Then let each answer name its own machine.

Three short lists go **at the top of the document** — after the machines and before the first question. Use these three names exactly:

- **Where Blender itself differs between versions.** That list is the reason this stage exists — it is what tells the app where it must behave differently, and everything downstream reads it as exactly that.
- **What is not established.** Everything no version settled, and everything settled for some versions or some machines and not others.
- **Where only the documentation differs.** Blender does the same thing everywhere, and one version's manual says so while another's does not.

**A list line is the short version and points at its question; the question holds the full answer.** No fact is stated twice in full, and no fact lives only in a list — a version difference whose only home is the summary is a difference nobody can check.

They go at the top rather than at the end because of who reads this document. Somebody arrives with a design problem, not a question number, and these three lists tell them in a minute whether the rest of the comparison touches their work at all. The full answers still follow, in question order.

**Keep the behaviour list and the documentation list rigidly apart.** They feel like one list and they are not. Putting a documentation difference in the behaviour list makes somebody build version-specific handling for something Blender does identically, which costs real work and adds a bug for every version that drifts. Putting a behaviour difference in the documentation list hides a real one. When you cannot tell which a question is, it belongs in neither — say so under its verdict and leave both lists alone.

HOW TO WRITE IT

Plain English. Short sentences, no jargon, no decoration. Somebody who does not write software reads this and makes decisions from it.

State each answer once. Do not restate a finding's citation — the finding holds it, and a second copy would drift from it. Where somebody is checking the evidence, they open the finding for that version.

Write it as the document that answers on its own. The comparison owns every Blender fact in this stage — the findings are the paper trail behind it, not a second document anybody reads, and the layer at `Preparation/Blender Research/Across topics/` that joins the topics for later stages may only carry what a comparison carries. So an answer left implicit here, on the grounds that the finding spells it out, is an answer nobody downstream will ever see.

**The comparison holds the answers and nothing else.** Everything about how the work was done goes into a second file, `Audits/Blender Research/<topic>/Comparison trail.md`, named in the inputs below. The test for which is which: **would a reader build something different if this line were not here?** What you read and what you did not read, which findings document covers which question, which version had a second pass and what it covered, what a citation check or a comparison check corrected, and when any of it happened — all of that goes to the trail. What stays in the comparison is the answer, what it does not cover, the label it carries, and which machine it was run on.

Two things follow that are easy to get wrong in opposite directions. A **disagreement that leaves an answer open stays in the comparison**, because a reader has to know what they cannot lean on — but write it as a fact about Blender rather than about paperwork: not *two of this version's documents disagree*, but *whether this holds is not settled, and the two readings disagree*. And **no round labels**: never say a question was added later or compared later. Rule 6 bans describing the moment of writing, and a reader meeting the questions in number order should not have to work out which sitting produced which.

Where you are extending a comparison that already exists, **fold your work into it** rather than appending a section of your own. Your questions take their places in the number order, your entries go into the three lists where they belong, and where something you have found overturns a line already there, **the later reading is what the document says** and the earlier one goes to the trail with the reason. This stage spent several rounds adding by pure insertion so that no earlier words moved, and what it produced was documents stratified by date, where the oldest reading of a question sat hundreds of lines above the newest. The record is the findings and the trail; those are untouched either way.

Save the comparison as a new file at the path given below, and create its companion signatures file in the `Signatures/` subfolder beside it — `<topic>.signatures.md` — with your line in it, as every document in this project carries one. The trail file carries its sign-off inside itself and has no companion of its own, like a hand-test list.

Sign off at the end of each with: date — [assigned model name] (the tool you're running in) — comparison of [topic]. Use the assigned name below verbatim; models often misname themselves. Open the trail file by saying which findings and which hand-test lists you read, and that you consulted nothing else — no documentation, no Cycles files, nothing of your own.

Inputs:
- The topic is: [e.g. File output nodes]
- The findings are: [one path per version]
- The question sheet is: [path]
- The hand-test lists, whose closing notes are written for you: [one path per list — there is one per machine the topic was tested on, and you read all of them]
- The machines the tests were run on: [name each one in full]
- Save the comparison as: [path, e.g. Preparation/Blender Research/Comparison/File output nodes.md]
- Save the trail as: [path, e.g. Audits/Blender Research/File output nodes/Comparison trail.md — where one already exists, add to it rather than writing over it]
- Sign off as (exact model name): [the model name the Creator assigned — use it verbatim]
```

---
