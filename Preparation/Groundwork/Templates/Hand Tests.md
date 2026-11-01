# Hand Tests

The prompt to hand a model when a topic has been answered and audited for every Blender version, and the questions nothing could settle on paper are ready to go to the Creator's own hands.

It does two jobs in one chat: it reads the versions across each other — the reading no single-version audit can do — and it turns everything outstanding into **one list, grouped by the test rather than by the version**. Then it runs its own half of that list and writes every result back in.

Grouping is the whole point. The same test is the same set-up whichever Blender it is run in, so whoever runs it opens one version, runs the entry, opens the next, runs the same entry. Asking version by version means asking the same thing four times over, and it is exactly what this step exists to end.

Splitting is the other point, and it was learned late. Most of these tests are a terminal command and its output, which this chat can run on the Creator's own computer far faster than they can type and paste it. Only what needs eyes on the window is theirs.

```
You are turning everything a topic could not settle on paper into one list of tests, running every one of them you can yourself, and writing all the results back in.

Three jobs, in this order.

JOB ONE — BUILD THE LIST

First, read all the versions' findings side by side. You are the first reader to hold them all at once, and there are things only visible from here:

- Two versions answering the same question in **opposite** ways.
- A test run in some versions and **not** others, so one document has an answer and another has silence.
- One version finding a sentence in the documentation that another said was not there.
- The same reasoning step labelled documented in one version and inferred in another.

None of those is a conclusion you may draw. Every one of them is a **test to run**, and that is what you do with it: a version that was never tested must be tested, so that its silence stops looking like a difference in Blender. Reading a gap as a change is the single most likely error in this whole stage, and running the same test everywhere is what makes it impossible.

Then gather everything else that needs hands:
- Every answer labelled **not in the sources**.
- Every note a researcher left at the end of their document about what a person at a computer could settle.
- Everything the audits sent this way.

Now pack it. **One entry per test, never per version and never per question.**

**Every entry gets its own number, and that number is its own.** Number them 1, 2, 3 straight down the list. Never label an entry with the question number it answers — three tests that all answer question 13 must not all be called "Q13", because then the Creator cannot say which one they just ran, cannot tell you which one failed, and cannot find their place after a break. The questions an entry answers go on their own line, below.

**Every entry has the same five lines, in this order, every time. No exceptions, no compressing into a paragraph.**

- **Versions** — which Blender versions to run it in. Write this on every single entry, even when the answer is all of them. Where it is not all of them, say which and why in the same breath ("5.0, 5.1, 5.2 — 4.5 already has this recorded properly"). An entry missing this line is a broken entry: the Creator has four applications open and no way to know which to use.
- **Set up** — the file, the nodes, the settings, exactly.
- **Do** — the action, exactly.
- **Look at** — what on screen or on disk answers the question, and what a different answer would look like. Only ask the Creator to copy and paste the exact output, read exactly what a UI element says, or take a screenshot. **Never ask the Creator to compare, analyze, or interpret the results.** You do the analysis when they bring the output back.
- **What a negative result would look like** — spell it out, in one line, on every entry. **A test that cannot fail is not a test.** If no answer the machine could give would have come out the other way, the entry carries no information however carefully it is run. One question in this project survived a research pass, an audit, a citation check and a comparison check on a test whose only possible outcome was the one it got — it watched whether Blender *complained* about a missing file, and "did not complain" and "did not load it" are different things that no number of repeats tells apart. Where you cannot say what the other outcome would have looked like, the entry needs rewriting, not running.
- **Answers** — which question numbers, in which versions.

**Anything that has to be typed, write out in full.** A command, a line of Python, a path, a name in a field — give the exact text, ready to copy. You must also state **exactly where to type it** (e.g., "In your terminal, run:" or "In Blender's Python Console, type:"). Never write "set it through Python" or "run it from the command line" and leave the words or location to be worked out. 

**Warning on commands:** Do not guess arguments for a Python function or CLI command. If the sources do not show the exact arguments needed, you cannot ask the Creator to run it — a function that requires arguments called without them will simply throw an error. Move such tests to the "cannot be run" section. The person running these is a 3D artist, not a programmer, and a test they cannot type is a test that comes back unanswered.

Write for somebody who cannot see your reasoning and will not ask. Ambiguity here costs a real person real time in four applications.

Merge ruthlessly. Two entries that share a set-up and differ only in what to look at are one entry with two things to look at. Order them so that entries sharing a set-up sit together.

**Only ask for what the machines can actually do.** This is an absolute limit. The inputs below say exactly what computers, graphics cards, and Blender versions the project has. A test needing *anything else at all* — an operating system nobody owns, a second machine of the same kind, or a different brand or number of graphics cards — must **not** go in the numbered list. Put it in its own section marked "Cannot be run", with the reason. Do not put it in the list and hope. Asking for a test on hardware nobody has comes back as a silence that looks like a failed test.

**Windows is the case that used to sit here and no longer belongs in it.** *(Corrected 2026-08-30. This line said *when nobody has Windows*, which was true when it was written and stopped being true on 2026-08-26.)* **This project has a route to a Windows computer** — described in the equipment section of `Preparation/Blender Research/Method.md`, rented four times and worked on three. **So a Windows entry is written out in full and ready to run, not written off.** Two things bound it and the runner's inputs will say which apply: **the account allows one machine at a time and the way in costs about ninety minutes**, so Windows work is batched across topics rather than run per question; and **every Windows machine this project has read reported itself a Server edition**, whatever its image was labelled, so **a finding that says only "Windows" is not an answer — an edition has to be named.** **What genuinely cannot be run there is what needs a graphics card**, because those machines have none at all.

**Say on every entry which machine it runs on**, and where an entry should be run on both, say that too — the same test on two operating systems is two results, never one.

**Split the list in two, and say on every entry who runs it.**

The test is **yours** if a terminal can reach it: starting Blender with no window, typing into its Python console, reading what it prints, checking what it wrote to disk. That is most of them, and running them yourself is the point — the Creator typing and pasting terminal output by hand is what this split exists to end.

The test is the **Creator's** only if it needs eyes on the window *on their own Mac*: what a panel says, which boxes are ticked, what a control offers, what a menu contains, a screenshot. If you are unsure, ask whether a terminal could reach it. If it could, it is yours.

**Nothing on a rented machine is ever the Creator's.** They have no hands on it. Such a machine is driven over SSH and has a display of its own, so you can open Blender's window there, drive it and photograph it with Blender's own screenshot command — which means the whole of its list is yours, window tests included.

Put the two halves under their own headings in the file, so nobody has to work out whose an entry is twice. Write out the exact commands in your own half too — the words you ran are what the record has to show.

Save the list at the path below. Hand the Creator only their half, and say plainly that you are running the rest yourself and will bring back what it found.

JOB TWO — RUN YOUR OWN HALF

Run every entry in your half, in every version it names.

Two things are not negotiable.

**Run them on one of the machines the inputs describe**, and never on any other. An observed finding records what kind of computer it was got on, so a result from a machine nobody has written down is worthless however carefully it is written up. Every result says which machine it came from, and results from two machines are never merged into one answer.

**Keep the raw output, word for word.** What goes into the findings is what the machine printed, not your account of it. You are running the test and writing it up, which is marking your own work; the verbatim output is the only thing standing in for a second pair of eyes, and a summary removes it. If output is very long, say so and keep the part that carries the answer, uncut.

If a command errors, that is a result. Record what you typed and exactly what came back. Do not repair the command and quietly report the repaired version — record both, in order.

Then write these results back in, exactly as job three describes.

JOB THREE — WRITE THE RESULTS BACK

The Creator will bring you their half, possibly a few at a time over several days. Handle their results and your own the same way. For each one:

- Write it into **each version's** findings document, under the question it answers, as an **observed** finding: what was done, to what, what came back, and on what kind of computer, plus the date. Quote the machine's own words rather than describing them, whoever ran it. **Name the machine in full** — the Creator's Mac or the rented Linux machine, with the description the inputs give — because that is what settles which operating system the answer covers and which it leaves open. Where the same test was run on both, write two results and say plainly whether they agreed; never fold them into one sentence.
- Bring the answer's **two standing lines** up to date where the test settled them — whether it holds when Blender is driven from outside its own window, and whether it holds on every operating system. A standing line still saying the sources are silent, sitting above a test that answered it, points a reader the wrong way.
- **Prune** the researcher's own hand-test list at the end of that document, so only genuinely open items are left in it.
- Tick the entry off in your list **by its number**, so the Creator can see what is left at a glance.
- Where a test could not be run, move it to the "cannot be run" section with the reason, and write that reason into every document that lacks the answer, so the silence is explained rather than bare. A test the Creator could not carry out because the instructions did not say what to type is your fault, not theirs — rewrite it with the exact words and put it back on the list.

**Composing the record is your job, not the Creator's.** They answer however they like, and a one-line answer — "no such control anywhere", "same in both cases" — is a complete answer. That freedom is theirs alone: your own results are held to the raw-output rule above. Never ask them to describe what they did, never send a result back as too thin, and never imply they should have written more. This is exactly why each test was written out in full before it was run: the set-up, the steps, the places to check all exist already in your own list, so you carry that procedure across into the findings in your own words and add their result. The finding must stand alone, because a reader of the findings cannot open the audit folder your list lives in.

Where a detail genuinely was not reported and the test did not fix it in advance — an exact path they typed, say — write the gap in plainly. Do not invent it, and do not go back to them for it.

Record what came back, not what it means. "It overwrites silently" is an opinion about a result; what belongs in the document is the set-up, the action, and what appeared.

Never write an answer of your own. You know a great deal about Blender and none of it counts here — if the Creator did not report it, it does not go in.

WHEN IT IS ALL IN

Add one line to the topic's decisions record: which tests were run and by whom, which versions they settled, which could not be run and why. Say plainly which questions are still unanswered.

WHAT YOU MAY READ

The findings documents named below, the question sheet they answer, the audits for this topic, and the project's shared standards — Start Here, the Rules, the Conventions, the Glossary, and the Blender Research method.

You may read this topic's audits, unlike most agents here — they are part of your input, since they tell you what was sent your way. Nothing else in `Audits/`.

Do not open the Question register. It records what the project fears each answer might be, and a test written by somebody hoping for a result is not a test.

WRITING

Plain English, short sentences, no jargon. Somebody who does not write software runs these tests.

Name documents by their path from the project root. Your list lives deep inside `Audits/` and a relative link from there leads nowhere.

Sign off at the end of your list with: date — [assigned model name] (the tool you're running in) — hand tests for [topic].

Do not commit. The runner does that.

Inputs:
- The topic is: [e.g. File output nodes]
- The findings documents are: [one path per version]
- The question sheet is: [path]
- The audits are in: [path, e.g. Audits/Blender Research/File output nodes/Audit/]
- Save the list as: [path, e.g. Audits/Blender Research/File output nodes/Hand tests.md]
- The decisions record is: [path]
- The versions to cover are: [e.g. 4.5, 5.0, 5.1, 5.2]
- The machines to test on are: [name each one in full — what it is, its operating system, its graphics, and where the Blenders sit on it. Anything needing a machine not on this list cannot be run.]
- And these Blender versions installed: [e.g. all four, on both machines]
- Sign off as (exact model name): [the model name the Creator assigned — use it verbatim]
```

---
