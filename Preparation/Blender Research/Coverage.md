# Coverage

Where the Blender Research stage has got to: which topics have been answered for which Blender versions, and which questions are owed an answer from a version already finished.

This is a **live tracker**, like the checklists in [Start Here](../Groundwork/Start%20Here.md): keeping it current is its whole job, so it describes changing state where every other document stays timeless.

It exists for two directions, and the second is the one that gets forgotten. A **new version** arrives and owes an answer to every question. A **new question** arrives and owes an answer from every version already researched, including ones finished long ago. Both show up here as gaps.

How many questions each topic holds is not repeated here — the [Question register](Question%20register.md) owns that, and a second copy would drift from it. A topic counts as **answered** for a version only when every one of its questions has a finding, including the ones answered *not in the sources*, which are answers like any other.

## The 2026-08-31 ruling governs every blank cell below

**The Creator ruled on 2026-08-31 that there will be no further Blender research and no further rented machine for answering a question.** *(Build may rent a machine to run the app on, ruled the same day. That is a different act and it revives nothing here.)*

**So a blank in this file is permanently unanswered, never pending.** No pass is coming, no machine is coming, and nobody should wait on one. The blanks stay, because they are the truthful record of what this project does not know; what each one costs is written beside it or in that question's entry in the [Question register](Question%20register.md). Read a blank as a hole to work around.

*Recorded 2026-09-02. This file had carried no mention of the ruling at all.*

**Except for Blender 5.3, from 2026-10-16 — Creator-stated.** The Creator lifted the ruling for 5.3 alone, once its beta documentation was in `Blender Sources/`. Every question sheet runs again for 5.3 as it stands, so **a blank in the 5.3 column below is pending until that pass lands, not permanent.** Every other blank keeps the meaning above. How the 5.3 work runs is in the [method](Method.md), under the same date.

## Answered

**What a tick means, said outright because the table and the rule above disagree.** The rule is that a topic is answered only when every one of its questions has a finding. **A tick below means every question the topic held when its research closed has a finding in that version.** Eleven topics have since taken a question that has no finding in any version and never will — The machine, Add-ons, Reading a file, Dependencies, File output nodes, Writing, Frames, Settings, Reports, Images, and What a render demands. **The ticks stand and none is changed**: the work behind each was done, and clearing one would deny it. **The twenty-one blank rows further down are where those eleven topics are unfinished**, and they are the part of this file to read. *(Written 2026-09-02, when the two were found to disagree.)*

| Topic | 4.5 | 5.0 | 5.1 | 5.2 | 5.3 |
|---|---|---|---|---|---|
| Builds | x | x | x | x | x |
| The machine | x | x | x | x | x |
| Add-ons | x | x | x | x | x |
| Reading a file | x | x | x | x | x |
| Dependencies | x | x | x | x | x |
| The compositor | x | x | x | x | x |
| File output nodes | x | x | x | x | x |
| Writing | x | x | x | x | x |
| Frames | x | x | x | x | x |
| Settings | x | x | x | x | x |
| Reports | x | x | x | x | x |
| Images | x | x | x | x | x |
| What actually ran | x | x | x | x | x |
| What a render demands | x | x | x | x | x |
| Node setups | x | x | x | x | x |

## Everything after answering is tracked elsewhere, on purpose

Whether a topic has been audited, hand-tested, checked against its sources or compared is **not** here. All of that is audit-trail information, and auditors read this folder while having to stay blind to that trail. It lives on the board, `Audits/Blender Research/Progress.md`, which is also where the runner keeps its place in the journey.

## Questions added after a version was finished

Any question taking a number after a version's pass has closed goes here until every version has answered it, so it cannot quietly stay unanswered for the versions nobody is looking at any more.

**Sixty-five. Forty-four are answered by all four versions; the twenty-one added on 2026-08-30 are answered by none.** **Settings 17 was added on 2026-08-29** for Architecture's `Render Settings` §13 item 24, **and was answered by all four the same day** — all seven steps, closed with a checked comparison. The six added on 2026-08-18 were answered by all four on 2026-08-25. What a render demands 6 was added and answered by all four on 2026-08-28 for Architecture Fact 134. **The compositor 9 was added on 2026-08-29 for Architecture Fact 135 and Row 492, and was answered by all four the same day** — all seven steps, closed with a checked comparison.

| Question | 4.5 | 5.0 | 5.1 | 5.2 | 5.3 |
|---|---|---|---|---|---|
| The compositor 8 — how far a picture can be followed through colour work and a merging node | x | x | x | x |  |
| The compositor 9 — what a scene reached through another scene's compositor is rendered with | x | x | x | x |  |
| Settings 17 — what name a launch-time instruction uses for three settings that have no argument | x | x | x | x |  |
| File output nodes 14 — whether what decides a node writes can change from frame to frame | x | x | x | x |  |
| The machine 7 — whether anything a render does can wait for a person | x | x | x | x |  |
| Add-ons 12 — whether installing an add-on disturbs a render already running | x | x | x | x |  |
| Add-ons 13 — what more can be learned about an add-on working, once it is switched on | x | x | x | x |  |
| Add-ons 14 — whether an add-on set up for one Blender works in another | x | x | x | x |  |
| Reading a file 9 — what each means of listing and rewriting recorded locations reaches | x | x | x | x |  |
| Dependencies 17 — what use a file makes of the places it records for writing | x | x | x | x |  |
| Dependencies 18 — what a render does when a cache or a bake is absent | x | x | x | x |  |
| Reading a file 10 — how a missing-file search chooses among possible matches | x | x | x | x |  |
| Reading a file 11 — what a later render uses after a file already read is replaced | x | x | x | x |  |
| Dependencies 19 — what makes two library locations one library or two | x | x | x | x |  |
| File output nodes 15 — what remains when one of a frame's several writes cannot finish | x | x | x | x |  |
| Frames 12 — what remains when images finish but a video has not closed | x | x | x | x |  |
| Settings 14 — what values a render's resolution percentage can be set to | x | x | x | x |  |
| Settings 15 — what a render does when asked for a value that setting will not take | x | x | x | x |  |
| Images 9 — what a render produces where a missing image feeds something other than colour | x | x | x | x |  |
| Settings 16 — what decides how many samples a render given a count from outside actually takes | x | x | x | x |  |
| Reports 7 — what a render reports about its progress across the whole range of picture sizes | x | x | x | x |  |
| Reports 8 — what a render prints over the whole of a run, first line to last | x | x | x | x |  |
| Reports 9 — what a render reports about its progress under each way of starting one | x | x | x | x |  |
| Reports 10 — what a render does when the machine's own memory runs out while sampling | x | x | x | x |  |
| Reports 11 — what a render prints that shows whether a frame took the samples it was asked for | x | x | x | x |  |
| Frames 13 — what is left where a render writes when it is stopped mid-write of a frame's own image | x | x | x | x |  |
| Frames 14 — what two renders started at the same instant into one folder each produce | x | x | x | x |  |
| Writing 11 — what is present where a render writes while it is still working | x | x | x | x |  |
| Writing 12 — what a render does when it must make a folder and cannot | x | x | x | x |  |
| Images 10 — what a finished picture holds about what produced it | x | x | x | x |  |
| File output nodes 16 — whether a scene reached through a sequencer strip runs its own file output nodes at all | x | x | x | x |  |
| File output nodes 17 — whether a node in a sequencer-reached scene stamps the frame number, and whether that differs by version | x | x | x | x |  |
| Writing 13 — whether a value handed in on the command line is in force by the time a script handed in at the same launch runs | x | x | x | x |  |
| Frames 15 — what a render interrupted part-way through an animation does between being asked and quitting | x | x | x | x |  |
| Reports 12 — what ending status a run gives back, across each of the ways a run can end | x | x | x | x |  |
| Reports 13 — what a render puts on each of its output streams when a file the scene points at is not there | x | x | x | x |  |
| What a render demands 6 — what a saved scene records and exposes about limits on subdivision, child particles and texture size, and their switch | x | x | x | x |  |
| Builds 10 — what a computer needs before a published build can be unpacked | x | x | x | x |  |
| Dependencies 20 — what a save records for a recorded outside place it never touched | x | x | x | x |  |
| Writing 14 — how one frame's writes stand in time against the next frame's | x | x | x | x |  |
| Settings 18 — what values a sample count takes, and what happens outside them | x | x | x | x |  |
| Settings 19 — what denoising settings a scene or a view layer carries | x | x | x | x |  |
| Reports 14 — when a printed line reaches a reader that is not a terminal | x | x | x | x |  |
| What a render demands 7 — what choosing between a card's rendering methods changes | x | x | x | x |  |
| The machine 8 — what a render does on a computer whose screen is drawn by its processor rather than by its card | | | | |  |
| Writing 15 — what is in a single video file at each point of a run | | | | |  |
| Writing 16 — when a render reports a file written, and what is on disk at that moment | | | | |  |
| Frames 16 — what is in the place a render writes to for the frame it had not finished when it was stopped | | | | |  |
| Frames 17 — what each way of stopping leaves of a video file | | | | |  |
| The machine 9 — what a render asked for the graphics card uses on a computer where Blender has just been put on | | | | |  |
| Add-ons 15 — where an add-on can be installed to, and whether each place disturbs a running render | | | | |  |
| Reading a file 12 — what form a repaired location is written back in | | | | |  |
| Reading a file 13 — what changes when a recorded library is pointed at a different file | | | | |  |
| Dependencies 21 — what a windowless render leaves for a program to read when something is missing | | | | |  |
| File output nodes 18 — what a file output node holds when first added, and what a render writes from one left as it comes | | | | |  |
| Frames 18 — the name and the already-there check on a scene producing more than one view | | | | |  |
| Frames 19 — what decides how long a render takes to finish stopping, across every picture size | | | | |  |
| Frames 20 — what two renders started at the same instant into one folder produce, across every picture size | | | | |  |
| Settings 20 — what number is printed and what number a picture records when several things name a sample count | | | | |  |
| Settings 21 — what a render does with something handed to it at launch that it does not recognise | | | | |  |
| Reports 15 — what the numbers count in each progress line of a frame worked in more than one piece | | | | |  |
| Reports 16 — what a render does when a graphics card's memory runs out while sampling, and what it reports throughout | | | | |  |
| Images 11 — whether the colour treatment reaches the pixels of a format holding more light than a screen shows | | | | |  |
| Images 12 — what happens when a picture is opened on a computer without the colour setup it was written under | | | | |  |
| What a render demands 8 — what decides whether a render works a frame whole or in several pieces | | | | |  |
The first two were added on 2026-08-03 by the round-2 audit of *What a job writes*; the next seven on 2026-08-06, when the Creator opened the stage again. **All nine were answered by all four versions on 2026-08-06.** The next five were added on 2026-08-07 after the stress test split its gaps by responsibility, and the two after that on 2026-08-08 by the round-1 audit of the scenario *The user renders the same project on a new machine*. The last was drafted on 2026-08-08 by the round-1 audit of the scenario *The user adds a texture and renders again* and took its number on 2026-08-09 — it had been parked in [Open Threads](../Groundwork/Open%20Threads.md) with no number and no row here, which is the second time this section has caught exactly that. All eight were answered by all four versions on 2026-08-09, so the six topics carrying them read **x** above again. The eighteenth was added on 2026-08-10 at the Creator's own asking and answered by all four versions the same day. **The last ten were added on 2026-08-11**, drafted blind by a cross-sweep that read the fifteen checked comparisons against each other; several exist because an earlier answer was true of its own test conditions and false as a general claim. They fall in four topics, and all four versions answered them on 2026-08-11, so those topics read **x** above again. Each later answer belongs in a document of its own beside the ones already there — never inside the audited document.

Rows stay here once they are ticked, rather than being deleted. They are the record that a question added to a closed topic did get picked up, and this file exists because one of them did not for three days.

**The last three were added on 2026-08-18**, by Round 1 of `Preparation/Architecture/Job files and outputs.md`. **Two came from one box on one day:** with a sequencer strip active the scene's own file output nodes wrote nothing, and in a sequencer-reached scene one version stamped a frame number where another did not. **Neither is a Blender behaviour yet** — they are machine observations, and the Architecture stage does not make claims about Blender, so both were fenced there and sent here instead. **No version has answered either**, and *The compositor* and *File output nodes* both read finished, which is exactly the shape this section exists to stop going quiet. **The third came from that round's close gate rather than from the writing**: the redirect leaves one destination to the command line and has its launch script read that value back, and nothing establishes whether the value is in force by the time the script runs. If it is not, every job fails before it renders. *Writing* also reads finished.

**Three more were added on 2026-08-18, by Batch E of the Architecture stage**, and they arrive by the same route as the three above: they are **machine observations made while posing something else**, and the Architecture stage does not make claims about Blender, so each was fenced there and sent here. **One box, one day, two versions.** A render interrupted the ordinary way went on printing every frame number it had not reached, doing no work at each, before quitting — which if it holds makes time-to-stop grow with the frames remaining and skews any measurement of what a stopped job prints. A run handed an unusable frame argument printed an error, wrote nothing, and **ended at a status meaning success**, while an interrupted run ended at a status other than the one usually associated with that signal — so a job that rendered nothing cannot be told from one that succeeded by status alone. And a scene pointing at a texture that is not there put **nothing at all** on the second output stream in either version. **No version has answered any of the three**, and *Frames* and *Reports* both read finished — the same shape this section exists to stop going quiet, now for the second time in a week.

**This section stayed empty for three days while the first two sat in the register**, which is the failure it was built to catch. Five questions were added that day; three were picked up by later passes and answered, and these two were not, because both topics already showed every step done and nobody goes back to a topic that reads finished. Recorded here rather than quietly fixed, since the same thing will happen again to the next question added to a closed topic.

**All six of the 2026-08-18 questions were answered by all four versions on 2026-08-25**, so *File output nodes*, *Writing*, *Frames* and *Reports* read **x** above again. **Every one of the six came back the same way, and that is the finding rather than a disappointment:** in all four versions the sources say nothing about a brought-in scene's file output nodes, nothing about what a run hands back to whatever started it, nothing about which output stream carries a complaint, and nothing about what a render does between being asked to stop and stopping. What each version *does* carry — the naming rule, the ordering of what is given at a launch, the one exit code Blender lets a script set, and the one place Cycles names a stream — is written into each finding with its quotation. **So all six turn on what a running Blender does rather than on the reading**, and each of them was settled that way. Where this round stands step by step is the board's, not this file's.

**Seven questions were added on 2026-08-29 and none of them has been answered by any version yet.** Every one had been parked in [Open Threads](../Groundwork/Open%20Threads.md) with a **Due** line saying *when the Blender Research stage is next opened*, or its near equivalent, and all seven fired together when it was. They fall in six topics — Builds, Dependencies, Writing, Settings twice, Reports and What a render demands — and **every one of those topics reads answered above**, which is the exact shape this section exists to stop going quiet. They stay here, blank, until all four versions have answered each.

**All seven were answered by all four versions on 2026-08-29**, so Builds, Dependencies, Writing, Settings, Reports and What a render demands read **x** above again.

**One limit in the sources themselves came out of that round and is worth recording here rather than only in a finding.** The Cycles files supplied with this research hold the kernel and the utilities and **no device-implementation folder at all**, so part of what What a render demands 7 asks about is in no file any researcher was given. Both versions that met it said so and neither filled the gap. It is the same shape of limit The compositor 9 met, where the place Cycles reads a scene's sample count is in no supplied file either.

  ~~Only a running Blender closes it.~~ **Rewritten 2026-09-02: both gaps are permanently unresolved.** Only a running Blender could have closed them, and the 2026-08-31 ruling means none is coming. **What breaks:** the reading route is shut for good, so whatever those two questions hold rests on the hand tests and on nothing in the sources — and whatever the hand tests did not reach stays unknown. No later reading pass can add to it.

**A second gap in the supplied sources, found on 2026-08-30 and recorded here so nobody meets it as a puzzle.** Blender 5.0's release notes point at a compositor migration page that **is not in the snapshot** — that folder holds the top-level subject files and not the pages they link onward to. A researcher needed it, said so, and left that part unanswered rather than guessing or going to the web. **Anything the release notes hand off to a linked page is outside what this research can reach**, in every version.

  ~~Until the snapshot is taken again more deeply.~~ **Rewritten 2026-09-02: this gap is permanently unresolved.** The snapshot will not be taken again — the 2026-08-31 ruling ends this stage's research. **What breaks:** a page the release notes only point at can never be read here, the 5.0 compositor migration page among them, so a version difference recorded only on such a page is one this project will not have.

**Five questions were added on 2026-08-30 and none of them has been answered by any version yet.** They come from the cross-sweep of that date, which read the fifteen checked comparisons against each other and then spent a third pass trying to kill everything the first two raised. **Every one of the five is a place the comparisons themselves cannot reach**: two topics reporting opposite contents of one folder after the same act, one topic using a single word for two opposite ways of stopping, a boundary nobody has ever measured, a summary sentence whose runs never met the one condition that breaks it, and a machine property four topics lean on and none has ever varied. They fall in three topics — The machine, Writing and Frames — and **all three read answered above**, which is the shape this section exists to stop going quiet.

~~They stay here, blank, until all four versions have answered each.~~ **Rewritten 2026-09-02: all five are permanently unanswered under the 2026-08-31 ruling.** No version is going to answer them. The five blanks stay, and so does what each one is — a place the comparisons cannot reach — because that is what the next reader has to work around. What each costs is in its entry in the [Question register](Question%20register.md).

**A third gap in the supplied Cycles files, found on 2026-08-30 by three researchers independently.** The folder holds the add-on's Python files, the kernel and the utility headers — and **no session, progress or logging code at all**. That is exactly where a render's printed output is produced, so **Reports 14 cannot be closed by reading in any version**: the place the answer lives is in no file any researcher was given. It is the third limit of this shape, after the missing device folder and the release-notes pages that are linked but not held.

  ~~All three mean the same thing — only a running Blender closes them.~~ **Rewritten 2026-09-02: all three are permanently unresolved.** Only a running Blender could have closed them, and the 2026-08-31 ruling means none is coming. **What breaks:** the sources can never be made to say what a render prints or when it prints it, so whatever the hand tests did not reach on that stays unknown for good.

**Sixteen more questions were added on 2026-08-30, later the same day, and none of them has been answered by any version either.** They come from the mechanism sweep of that date — the pass that read the fifteen comparisons and the ten joined answers for claims that outran the runs behind them, and then repaired every one it found. **What it could not repair by writing is what these are.** About forty-one runs were named at the end of `Audits/Blender Research/Cross-sweep/Mechanism sweep/Repairs — the list.md` and a few more by the agents doing the repairs; four pairs of them turned out to be one run each, and rather more than half were declined, either because an existing number already asks the thing or because they are questions about this project's own records rather than about Blender. **The sixteen that took a number all have the same shape**: an answer that is true of the conditions it was measured under and is written down as a general claim, where the untested condition is the one Smoozy actually meets — a machine nobody has ever opened Blender on, a render with no window, a picture bigger than a thumbnail, a node nobody touched, a scene with two views. They fall in ten topics — The machine, Add-ons, Reading a file twice, Dependencies, File output nodes, Frames three times, Settings twice, Reports twice, Images twice and What a render demands — and **every one of those topics reads answered in the table above**, which is the shape this section exists to stop going quiet.

~~They stay here, blank, until all four versions have answered each.~~ **Rewritten 2026-09-02: all sixteen are permanently unanswered under the 2026-08-31 ruling.** No version is going to answer them. **Sixteen is also the number that never reached a question sheet** — they took a number on 2026-08-30 and no sheet line was ever written for any of them, and now none will be; the [Question register](Question%20register.md) records that and holds what each one breaks. The blanks stay, and so does what they have in common — an answer true of the conditions it was measured under, written down as a general claim, where the untested condition is the one Smoozy meets.
