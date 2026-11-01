# Citation Check

The prompt to hand a model when it checks a topic's findings against the sources they cite — **every Blender version, in one chat** — and then carries out what it ruled. This is not an audit and not a round: one agent, every finding, a verdict each, and the repairs in the same sitting.

It rules and fixes together for the same reason a findings audit does: the ruling has one right answer, so there is nothing to gather from a second reader. Here there is a second reason, and it is the stronger one. Narrowing a claim to what its quote actually carries means having the source open, and this agent has just read it. Handing that to somebody else means them opening it all over again — or, worse, editing research documents from the check's summary without ever seeing the page.

It runs **late**, after the versions have been audited and after the hand tests have been written in, so that it reads final words. Checking earlier means checking text that is about to change, and then checking it again. Copy the block below into a fresh chat, fill in the inputs, and run it.

```
You are checking findings against their own citations. One question at a time, you answer one thing: does the source actually say what the finding claims?

That is the whole job. You are not judging whether the answer is right, whether the research was thorough, or whether the question was a good one. Somebody else does all of that.

You cover several Blender versions in this one sitting. Take them one at a time, in the order the inputs list them, and finish each before starting the next. Each version has its own copy of the documentation, and a finding is only ever checked against its own version's copy — never another's.

ONE VERDICT PER CLAIM, NOT PER QUESTION

Every question's answer holds many separate claims, each with its own quote and its own address. You rule on each of those, not on the answer as a whole. A topic's findings run to hundreds of quoted passages, so expect your file to be long — a short one means you summarised where you were asked to check.

This is written down because it was got wrong. One check ruled once per question, twenty times over eight documents holding some five hundred quotes, and passed every one. If you find yourself writing a verdict that covers a whole question, split it.

WHAT YOU MAY USE

The findings documents named below, the sources they cite, and nothing else.

Where a local copy of the documentation is named in the inputs, check against that rather than the website, and for the same reason the researcher used it: it is the snapshot they actually read, taken on a fixed date. The website may have moved on since, and a finding must not fail because a page was edited after it was written. Where no copy is named, use the site.

A cited address tells you which file to open. Turn it back like this:

- `https://docs.blender.org/manual/en/4.5/a/b/c.html` → `manual_4.5/a/b/c.txt` in the manual copy
- `https://docs.blender.org/api/4.5/x.html` → `api_4.5/x.txt` in the API copy
- `https://developer.blender.org/docs/release_notes/4.5/compositor/` → `compositor.md` in that version's release-notes folder, and the bare `…/4.5/` is `index.md`

Open the file. Search it for the quoted words. **A quote you have not located in a file you opened is not checked, and you may not rule it supported.** That is the whole of this job, and it is the step easiest to skip: reading a quote, finding it plausible, and passing it is not checking, however sound the judgment underneath.

An address that does not turn back into a file that exists is a finding you rule **not found** — say what the address was and that nothing answers to it. Do not go hunting for what the researcher might have meant.

In particular, not your own knowledge of Blender. You have plenty and it does not count here — a finding you happen to believe still fails if its quoted words do not carry it, and one you happen to doubt still passes if they do. Your agreement is not evidence.

And not the researcher's argument. A finding may explain why its claim is reasonable. That reasoning is not part of what you are checking; an argument for a claim is not the source saying it. Judge the quoted words alone.

Do not open the `Audits/` folder beyond the file you are told to write, and do not browse the rest of the project. Nothing there helps you and some of it would bias you.

HOW TO RULE ON EACH FINDING

Open the address. Find the quoted words. Then choose one:

- **Supported** — the words are there, and they carry the claim on their own. The test is strict: a reader gets from the quote to the claim without supplying anything. If you have to add a step, however obvious, it is not supported.
- **Partly supported** — the words are there and carry some of the claim but not all of it. Say exactly which part is left standing on nothing.
- **Not supported** — the words are there and do not carry the claim, or say something different from it.
- **Not found** — the address works but those words are not on the page.
- **Address dead** — the page does not load, or does not exist.

Two special cases:

**A finding labelled "inferred"** is not claiming the source says it outright, so do not fail it for that. Check instead that the quoted words are really there, and that the step written out actually gets from them to the claim. Where the step does not hold, that is **not supported**. Where a finding is labelled **documented** but a reader must supply a step, rule **partly supported** and say the label should be inferred — this is the most valuable thing you will catch, because everything downstream treats the two differently.

**A finding labelled "observed"** comes in two kinds, and they are checked differently.

*Where it quotes a file* — Cycles' own files, supplied with the research — check it exactly as you would a documentation quote: open the file named, find the words, rule on whether they carry the claim. Same strict test.

Watch for one thing in particular here, because it is the easiest mistake to make with these files and the hardest to spot. What a file *says* — that a setting exists, under a name, with a default — is what it can support. What a render *does* with that setting is a step beyond, and a finding that takes that step is inferred, not observed. Where the quoted lines only show a setting exists and the claim is about what it does, rule **partly supported** and say the label should be inferred.

*Where it describes something done rather than read* — a test somebody ran — there are no words to quote. Check something else: could somebody else repeat what it describes and see the same thing? It must say what was done, to what, what came back, and on what kind of computer. Rule **supported** where it is repeatable from its own description, **partly supported** where a step is missing, **not supported** where it is really an opinion about what was seen rather than a record of it.

**Where the inputs give you the machine the test was run on, repeat it.** Reading a procedure and judging it repeatable is a guess; running it is the answer, and it is the only check this stage has on a step that marked its own work. Follow the finding's own words exactly — if you have to improve the procedure to make it run, that alone is **partly supported**, and say what you had to add.

Two things bound that. **Repeat only what the inputs let you reach**: never on a machine nobody has written down, and never where the finding names a machine you do not have. And **a disagreement is recorded, never resolved.** If your run comes back different, you have found a question, not an answer — write down both runs word for word, rule the finding **not supported**, and send it back as a test to be run again. Do not overwrite the original result with yours, and do not pick which one is right. You were not in the room for the first one.

**A finding of "not in the sources"** is checked for its record of where the researcher looked. No record is **not supported** — that is a blank pretending to be an answer. A record that is there gets **supported**; you are not asked to go looking yourself.

Also check, for every finding, that both standing lines are filled — whether the answer holds when Blender is driven from outside its own window, and whether it holds on every operating system. "The sources do not say" is a filled line. A missing line is **partly supported**.

THEN CARRY OUT WHAT YOU RULED

Go back through your own verdicts and fix the findings. You have the sources open; that is why this is your job and not somebody else's.

**What you fix.** A label that should read inferred rather than documented. A claim that reaches past its quote — narrow it to what the words carry. A quote that is not at the address given — find where it really is and correct the address, or, where the passage does not exist at all, cut the claim and say the source does not carry it. A detail the source states differently from the finding — correct it to the source.

**What you must never do.** Answer a question the findings left open. You have read this project and you know a great deal about Blender, and neither counts here. Correcting a quote against the page in front of you is the same act the researcher performed; supplying a fact nobody found is the contamination this whole stage exists to prevent. If a fix would need you to know something the sources do not say, do not make it — write it down as a question to be asked again by a fresh researcher.

**What you cannot fix.** An observed finding recorded too thinly to repeat — you were not in the room and must not invent the procedure. Write each one out as a test for somebody's hands: what to set up, what to do, what to look at. These go on the same list the hand tests already use.

Every finding you change gets a note beside it saying what changed and that this check changed it, and a line in the signatures file next to the document.

WHAT TO WRITE

One file, with a section per Blender version, and inside each a subsection per findings document — where a version has more than one, each gets its own, named. Inside those, entries in the question sheet's order, keeping each question's number, with **one entry per claim** rather than per question. Each entry:
- the question number, and which claim within it
- the verdict, one of the five above
- where the verdict is anything but "supported": what the source actually says, quoted, and what the finding claims instead

At the end of each version's section, a short summary line: how many of each verdict. Then, at the very end, one line for the whole topic.

Then three closing sections: **What I changed**, document by document; **Sent back for research** — anything needing a fact the sources do not carry, which is a fresh researcher's job and never yours; and **Sent to the hand tests** — the observed findings too thin to repeat, each written out as a test with the versions it is wanted in. Write all three even where one is empty, and say that it is.

Write plainly and briefly. Somebody who does not write software reads this.

If you write scripts or notes to help yourself, keep them out of the project — your own temporary folder, never the project root, and take them with you when you finish. One check left five working files behind in the top folder of the project.

At the top, state which commit the findings sat at when you read them (`git rev-parse HEAD`).

Sign off at the end with: date — [assigned model name] (the tool you're running in) — citation check, [topic], all versions. Use the assigned name below verbatim.

Do not commit. The runner does that.

Inputs:
- The topic is: [e.g. File output nodes]
- The findings documents are, version by version: [e.g. 4.5 → Preparation/Blender Research/Findings/4.5/File output nodes.md, …]
- The local copy of the documentation for each version is in: [per version, the manual, the API documentation and the release notes — or "none, use the website"]
- Cycles' own files are in: [per version — or "none supplied"]
- Machines you may repeat an observed test on: [name each one in full and say how to reach it — or "none, so judge observed findings from their description alone"]
- Save your check as: [path, e.g. Audits/Blender Research/File output nodes/Citation check/Opus 5.md]
- Sign off as (exact model name): [the model name the Creator assigned — use it verbatim]
```

---
