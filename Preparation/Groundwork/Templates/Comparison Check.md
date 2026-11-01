# Comparison Check

The prompt to hand a model when a topic's comparison has been written and needs checking against the findings it rests on. The last step of a topic.

Like the citation check, this is a check and not a round: the question is whether a verdict is carried by the findings, which has one right answer. One chat, every verdict, a ruling each — and it fixes what it can.

The comparison owns every Blender fact this stage produces, and everything a later stage reads is built on it — including the layer at `Preparation/Blender Research/Across topics/`, which may only carry what a comparison carries. So a single overreaching line here can undo a whole topic's work. The one error this exists to catch above all others is a **silence written down as a difference**.

```
You are checking a comparison against the findings it was built from. For every question, one thing: does the verdict follow from what the versions actually said?

You are not researching, and you are not judging whether Blender really behaves this way. You know a great deal about Blender and none of it counts here — a verdict you happen to believe still fails if the findings do not carry it, and one you happen to doubt still passes if they do.

A VERSION MAY HAVE MORE THAN ONE FINDINGS DOCUMENT

Where the inputs name two documents for a version, open both for every question. A verdict has to be carried by both taken together. Where the two disagree with each other, the comparison should say so rather than pick one — a disagreement inside a version is not a difference between versions.

HOW TO RULE ON EACH QUESTION

Open the question in the comparison. Open the same question in every version's findings. Then choose one:

- **Follows** — the verdict is what those findings add up to.
- **Overreaches** — the verdict says more than the findings support.
- **Gap read as a difference** — the worst one, and the reason this check exists. A version answered "documented" sitting beside a version answered "not in the sources" is NOT evidence that Blender changed. Far more often one version's documentation is simply thinner. Where the comparison has turned a silence into "it differs", rule this.
- **Understates** — the findings agree plainly and the comparison hedged anyway.
- **Missing** — a question on the sheet that the comparison does not cover.
- **Label dropped** — a version's answer was inferred or observed rather than documented, or a hand test settled it on one operating system only, and the comparison presents it as though it were plainly documented. That difference is what a reader decides how much to build on.
- **Reaches past the machines it was run on** — a result that came back the same in all versions still settles only the machines it was actually run on. Where the comparison lets a uniform result read as established generally — for an operating system nobody tested, another make of graphics card, a machine with more than one — rule this.
- **Two machines read as two versions** — tests have been run on more than one machine, and this is the fault that arrives with them. A result from one machine set beside a result from another says nothing about the versions. Where the comparison compares 4.5 on one computer against 5.2 on another and calls the difference a version difference, rule this; the honest verdict is "not established for every version". Watch too for a difference between operating systems written into the list of differences in Blender itself — that list is about versions, and an operating-system difference in it will have somebody building version-specific handling for something every version does the same.

Two things are **not** faults, so do not rule against them. A difference resting on a release note saying a thing was added, removed or changed is real, even where another version is silent. And a difference resting on the same test actually run in every version and coming back differently is real — a test run everywhere is not a silence.

Check the comparison's three lists too, each in its own entry: *Where Blender itself differs between versions*, *What is not established*, and *Where only the documentation differs*. They sit at the top, before the first question. The two difference lists must stay rigidly apart — a documentation difference in the behaviour list makes somebody build version-specific handling for something Blender does identically, and a behaviour difference in the documentation list hides a real one.

Two more things about the lists. **A list line is the short version and its question holds the full answer**, so rule against a fact stated twice in full, and against one whose only home is a list — a version difference nobody can check under a question is a difference nobody can check. And rule against a fact **missing from a list** that its own question establishes.

**Rule against trail that has crept back in.** A comparison holds answers; how the work was done lives beside it in `Audits/Blender Research/<topic>/Comparison trail.md`. So a sentence saying what the comparer read, which findings document covers which question, which version had a second pass, what an earlier check corrected, or when any of it happened does not belong in the comparison, and neither does a round label saying a question was added or compared later. Two things that look like trail and are not: which machine a result was run on, which qualifies the fact; and a disagreement that leaves an answer open, which a reader has to know about — though that one belongs in the comparison only if it is written as a fact about Blender rather than about documents.

FIX WHAT YOU CAN

Where the fix is to bring a verdict back to what the findings carry — change "it differs" to "not established for every version", restore a dropped label, add a missing question — make it in the comparison and say you did.

Where a fix would need a fact nobody has established, do not supply one. Say what is missing and leave it.

WHAT TO WRITE

One file at the path below. In it:

1. One line at the top: what you consulted, the commit you read it at (`git rev-parse HEAD`), and confirmation that you used nothing but the findings and the comparison.
2. One entry per question — the number, the ruling, and where it is anything but "follows", what the findings actually say and what the comparison claims instead.
3. What you changed.
4. What you left, and why.
5. A summary line: how many of each ruling.

Then add one line to the topic's decisions record saying what the check found and what you changed.

Write plainly. Short sentences, no jargon. Somebody who does not write software reads this and makes decisions from it.

Name documents by their path from the project root; your file lives deep inside `Audits/` and a relative link from there leads nowhere.

Sign off at the end with: date — [assigned model name] (the tool you're running in) — comparison check, [topic]. Use the assigned name below verbatim.

Do not commit. The runner does that.

Inputs:
- The topic is: [e.g. File output nodes]
- The comparison is: [path, e.g. Preparation/Blender Research/Comparison/File output nodes.md]
- The findings are: [one path per version]
- The question sheet is: [path]
- The comparison's trail, for anything you need to look up about how it was built: [path, e.g. Audits/Blender Research/File output nodes/Comparison trail.md]
- Save your check as: [path, e.g. Audits/Blender Research/File output nodes/Comparison check/Opus 5.md]
- The decisions record is: [path]
- Sign off as (exact model name): [the model name the Creator assigned — use it verbatim]
```

---
