# Sources — Render Settings (Architecture)

*Where the outside-world facts [Render Settings](../Render%20Settings.md) labels in place came from. The document carries each claim's label beside the claim; this file carries the citation, the exact words where an outside publication or a comparison is quoted, the date read where one exists, and what each source does **not** cover. **A decision Smoozy made gets no entry here.** Where the citation is owned by one of this stage's frozen input documents, or by a peer answer document or its companion, the entry is a pointer to that document and not a second citation.*

**Blender facts are not re-derived here.** Each Blender entry names the joined answer or the comparison question the document itself already names at *Where a Blender fact is looked up is fixed*, and nothing further. No Blender answer in this file is assembled out of several topics. The joined answers are [What a file can be asked before it renders](../../Blender%20Research/Across%20topics/What%20a%20file%20can%20be%20asked%20before%20it%20renders.md) and [What can be set from outside](../../Blender%20Research/Across%20topics/What%20can%20be%20set%20from%20outside.md), in `Preparation/Blender Research/Across topics`; the comparison questions live in [Preparation/Blender Research/Comparison](../../Blender%20Research/Comparison).

**Nothing here was looked up outside this project.** Where `Preparation/` holds no source for a claim, the entry says so, says what breaks, and names the folders searched. No address is supplied that this project does not already record.

**The four parts.** Part 1, Blender facts as pointers. Part 2, this document's own controlled test of 2026-08-19 — the only place in this file where an observation is recorded rather than pointed at. Part 3, the stage's frozen input documents, `Facts.md`, and peer answer documents. Part 4, outside publications quoted directly, each with the address and date this project records. Part 5, assumptions and claims with no source here.

---

# Part 1 — Blender facts: pointers to the joined answers and comparison questions

Every entry in this part is **shape 1**.

## B1. The one window, both edges silent, and the argument name taken for a file (Y3, Y4, V4, C1, C7)

**Owned by:** [What can be set from outside](../../Blender%20Research/Across%20topics/What%20can%20be%20set%20from%20outside.md), whole; behind it *What a render demands 3*, *Writing 1*, *Dependencies 17*, *Settings 16*, *Settings 1*. Label: **documented** for the arguments, **observed in a test** for the two edges.

**In the joined answer's own words:** anything imposed on the command line *"has to sit in one window: **after the file, and before the render instruction**. Outside that window it is silently ignored — no message, no failure, the render goes ahead on the file's own value."* The left edge is *Settings 1*'s tested ordering rule — an expression placed before the file *"still printed its values, and the image still came out at the file's own size"*; the right edge is *Settings 16*'s — a count placed after the render argument gave `Sample 4096/4096`, the file's own number. And *"An argument name the running version does not carry is taken for a file to open, which can end a run at a failure status **after** its frame is already on disk"* (*Settings 9*, *Settings 1*).

**A third position is now settled, and the window quoted above is short of an edge. Added 2026-08-31, out of the repaired Blender research.** [What can be set from outside](../../Blender%20Research/Across%20topics/What%20can%20be%20set%20from%20outside.md) now sets out **three** positions on the launch line rather than two: **after a bare double dash the line is Cycles' own, and an argument Cycles does not know is swallowed without a word while the render goes ahead and ends at 0.** Label: **observed in a test**. **What breaks:** nothing this document composes — Y3, Y4, V4, C1 and C7 all place their material inside the window, and the imposition set carries no device argument — **but the window as quoted admits a position at which a mistyped argument is silently ignored at a successful ending**, and the *silently ignored* the quotation describes is the outside-the-window case rather than this one. `Rendering` L8 item 9 is where the device argument sits after the double dash, and `Rendering`'s own sources file carries what that costs.

**Not covered:** the thread-count exemption Y3 does not use is *What a render demands 3*'s. Windows, everywhere in these topics.

## B2. The three settings with no argument of their own (Q2a, C1, C7, V4, Section 4's constraint)

**Owned by:** *Settings 1*, *What a render demands 3*. Label: **documented** for the absence, **observed in a test** for the script lever.

**What *Settings 1* carries:** *"No argument of its own — in all four versions, on both machines. A sample count, a resolution percentage, and whether a view layer renders."* Every version searched its manual and found none, and the help text of each program was then searched on both machines for `sampl`, `resolution`, `percent` and `view layer`. The lever that reaches all three is a Python expression handed in at launch, after the file: *"the expression printed `SET 8 25 False` and the render came out 50 by 25 pixels instead of 200 by 100."*

**Not covered:** the exact per-version property spelling of each of the three. *Settings 1* establishes that the settings have no argument, not how each is spelled — which is the whole of [Section 13](../Render%20Settings.md#13-unresolved) item 24 for the resolution percentage, entered at [U18](#u18-the-resolution-percentage-pairs-spelling-c7-y4-section-13-item-24).

## B3. The launch-script lever's demonstrated size (Y6; Section 13 item 6)

**Owned by:** *What a render demands 3*. Label: **observed in a test**, all four versions, both machines.

**What it carries:** *"A start-up script can set a setting that has no argument of its own, and it reaches the render: samples set to seven printed seven and produced a picture that is not the 64-sample picture."* And, for the percentage: *"a 160 by 120 file at 37 per cent wrote a 59 by 44 image in every version on both machines."*

**Not covered, and it is Y6's whole point:** nothing at a size a real job reaches. Both demonstrations are at 160×120. This document's own test adds 64×64 at 16 samples and nothing larger — [Part 2](#part-2--the-controlled-test-of-2026-08-19), *What this test does not cover*.

## B4. Nothing imposed at launch is written into the file (Y3, V6, D2)

**Owned by:** *Settings 1*, *Writing 1*, *Reading a file 9*, *What a render demands 3*, and for the device argument *The machine 5* and *Settings 9*. Label: **observed in a test**.

**What *Settings 1* carries:** *"after a render with arguments contradicting everything the file held, the file's size and modification time were unchanged and reading it back gave its own saved values."* *The machine 5* for the device argument: *"The option writes nothing back: the saved preferences were unchanged afterwards."*

**Not covered:** the exception the joined answer states — a run that saves the file in the same session writes the imposed value in. Nothing in this document's launch material saves; that is V6's own clause and is a decision, not a source.

## B5. The four things that move across the version boundary (Section 1.4's constraint, Q11, Q12)

**Owned by:** [What a file can be asked before it renders](../../Blender%20Research/Across%20topics/What%20a%20file%20can%20be%20asked%20before%20it%20renders.md), *Read the file with the version that will render it*, which names four and no more; behind it *What a render demands 5*, *Reading a file 3*, *Settings 13*, *Add-ons 1*. Label: **observed in a test** for all four.

**The four, in the joined answer's own words:** a value can come back different from the value saved — *"A file saved by 4.5 with its tiling switched off and a size set reads back, in the newer three, as switched on at a size nobody chose"* (*What a render demands 5*); *"the switch saying whether a view layer renders is declared off in 4.5 and on from 5.0"* (*Reading a file 3*); *"The real-time engine's identifier changes at 5.0"* (*Settings 13*); and *"With the Cycles add-on switched off, a Cycles file reads back under the other engine's name and its Cycles settings cannot be read at all"* (*Settings 13*, *What a render demands 5*, *Add-ons 1*).

**Not covered:** the joined answer states four; it does not claim the four are exhaustive of everything that moves. What a fifth would cost is Q11's version binding, which holds whatever the count.

## B6. A read beside a running render, and how many Blenders one machine carries (W4; Section 13 items 1 and 2)

**Owned by:** *Reading a file 2*. Label: **observed in a test** for the second half; **not in the sources in all four versions** for the first.

**What it carries:** a second Blender opened the same file with no window mid-render, *"quit with an exit code of 0 while the render was still going"*, and the render *"finished with an exit code of 0, wrote all 60 frames, and printed no error and no warning at all"* — all four versions, Mac and rented Linux. And *"The count could not be tested on either machine, and no test could settle it… Whatever number of Blenders a computer would start is a fact about that computer."*

**Not covered, and it is exactly W4's *magnitude untested*:** *"Neither run says anything about how long any of it took"*, and *"Whether a second or third Blender slows the render down. Neither machine can say."* The machine-side measurement is the frozen input document's and is entered at [M3](#m3-one-box-with-blender-on-it-16--five-reads-beside-a-21-frame-render-w4-section-13-item-1).

## B7. The per-frame view-layer answer (Q3, V1, Section 4's constraint)

**Owned by:** *Reading a file 3*; also *Settings 1* for the keyframed render. Label: **observed in a test**, all four versions, both machines.

**What *Reading a file 3* carries:** *"The switch can be keyframed from a script, and the curve lands on the scene with a data path naming the layer"*; *"Stepping the frame changes what the switch reads"*; *"An animation render honours the keyframed value frame by frame. With one layer switched on at frame 1 and off at frame 2, the file written for frame 1 held both layers and the file written for frame 2 held only the one still switched on."* Blender marks the render switch animatable and the render-single-layer switch not, read directly on the rented machine in all four versions.

**Not covered:** all four versions *"record as **not in the sources** whether Blender re-reads the switch at each frame of an animation render."* What is established is that an animation render honours the keyframed value frame by frame, by test on two machines — not a statement about re-reading.

**Repaired 2026-08-23.** This entry used to record the document's wording as sitting ahead of the comparison's and left it there: [Section 4](../Render%20Settings.md#4-view-layers)'s constraint read *the switch is read again at every frame*, which is the joined answer's phrasing (*"the switch is read again at every frame, so a reading is a reading of that frame"*, and again *"a Cycles render started from outside reads the value again at every frame; the view-layer switch behaves the same way"*) and not the comparison's. **The constraint now states the tested sentence and carries the *not in the sources* record beside it.** The joined answer's own phrasing still sits ahead of its comparison; that is Blender Research's and is reported to the runner rather than touched here.

## B8. The route to the animation curve, per version (Q2 item 2, V6, V9)

**Owned by:** *Reading a file 3*. Label: **observed in a test**, confirmed in all four versions on both machines.

**In its own words:** *"In **4.5** an action's curves hang directly off the action, and reading them that way worked. In **5.0, 5.1 and 5.2** that route does not exist — asking for it raised the same error, worded the same way — and the curve has to be reached by walking the action's layers, then its strips, then its channel bags, then its curves."*

**Not covered:** nothing there establishes that the layered route reaches curves in 4.5. The 4.5 aside is this document's test's and is at [Part 2](#part-2--the-controlled-test-of-2026-08-19) item 8. Removing a curve, as against reading one, is posed by no comparison — it is this document's test's, item 4.

## B9. The switch's declared starting value (V10)

**Owned by:** *Reading a file 3*, with *Settings 1*'s note beside it. Label: **documented** and **observed in a test**, on one machine.

**What they carry:** *Reading a file 3* — *"In **4.5** it reads off. In **5.0, 5.1 and 5.2** it reads on. Each version's own API text says the same as its probe."* *Settings 1* records the same split and settles which of the two it is: *"each version's running program declares exactly what its own documentation says, read out of all four under question 17, on the Creator's Mac and on no other machine"*, so it is *"a difference in Blender rather than in the documentation"*. It adds the thing V10 actually needs: *"it is the property's declared value and not what a view layer carries: in all four versions the default scene's own layer reads on, and a layer made fresh reads on — so 4.5's declared `False` is not what a real layer starts at"*, and *"In every version's tests here, the file's own view layers rendered without anyone turning them on."*

**Repaired 2026-08-30.** This entry used to record the split as **not reconciled** and quoted *Settings 1* as saying no test read the default back from a fresh view layer and that whether it is a change in Blender or a correction to the documentation *"is not established here"*. **Neither sentence is in *Settings 1***, which says the opposite of both: a layer made fresh was read in all four versions, and the split is a difference in Blender. The label and the *not covered* below are corrected to what the comparison carries.

**Not covered:** the reading behind it was made on **the Creator's Mac and on no other machine** — *Settings 17*'s bound, which is macOS on Apple silicon and nothing else. V10's use — recorded so nothing cold-reads the default — needs neither the answer nor the bound.

## B10. The driver case and the stored-script case, as the research left them (V2, V3; Section 13 item 7)

**Owned by:** *Reading a file 3*'s *Not established* list. Label: **not established** there; the document's own answers are observed and are [Part 2](#part-2--the-controlled-test-of-2026-08-19) items 1 and 2.

**In its own words:** *"**Whether a driver, as opposed to a keyframe, on that switch can be resolved from one read.** Flagged by every version, tested by neither machine. Blender 5.0's, 5.1's and 5.2's documents add that automatic running of Python drivers is off by default, so a Python-driven switch would read differently again. Blender 4.5's do not mention it."* And *"**Whether a script inside the file could change the switch as each frame comes round.** Named as a possibility by 5.1 and 5.2, tested by nobody."*

**This is the address for item 7's *5.x documentation says needs the automatic-running permission*.** The claim is the comparison's, not a lookup owed.

**Not covered:** the comparison poses no driver shape at all, so the shape this document's test posed — a scripted expression — is the test's own ground and is bounded there.

## B11. Render Single Layer never reaches a render started from outside (Section 4's constraint)

**Owned by:** *Settings 1*, *What a render demands 3*, *Reading a file 3*. Label: **documented** and **observed in a test**.

**In *Settings 1*'s words:** *"**"Render Single Layer" is ignored from the command line, in all four versions, on both machines.** Every version's manual says so, and the test watched it happen on both: with that setting turned on at launch, both view layers were still written."* Blender refuses to animate it — *"`use_single_layer` property cannot be animated"*.

**Not covered:** nothing about the per-layer switch follows from this; the two are separate settings and the document delivers unchecking only through the per-layer one.

## B12. The stored-scripts preference, and the two launch switches (V3; Section 13 item 9)

**Owned by:** *The machine 7* for the preference and the printed decline; *Settings 11* for the two overriding arguments, as [What can be set from outside](../../Blender%20Research/Across%20topics/What%20can%20be%20set%20from%20outside.md) routes it. Label: **documented** for the starting value and the switches, **observed in a test** for the printed line.

**What *The machine 7* carries:** *"Blender does not run a blend-file's stored Python unless told to"*; *"Automatic execution starts switched off in all four — checked in all four versions on both machines rather than assumed"*; what it catches is *"registered text data-blocks, and driver expressions written as Python"*; the way to settle it in advance is *"`-y` / `--enable-autoexec` and `-Y` / `--disable-autoexec`, with the same worked example in every version"*; and the refusal prints *"`scripts disabled for "<the file>", skipping 'Text.py'`, and the render carries straight on."*

**The manual's exact wording for the two switches, with its address**, is at [P2](#p2-blenders-command-line-arguments-page--the-two-automatic-script-execution-switches-v3-section-13-item-9).

**Not covered:** what any of the dialogs do with no window is *"missing from all four"* — *The machine 7*'s own words — which is why the printed line and not a dialog is what this document leans on.

## B13. A sample count is a ceiling, and the five that stand between (C1, C2, C5; Section 13 item 10)

**Owned by:** *Settings 16*, the establishing comparison; the joined answer [What a file can be asked before it renders](../../Blender%20Research/Across%20topics/What%20a%20file%20can%20be%20asked%20before%20it%20renders.md) states the same five as a list. Label: **documented** for the settings themselves, **observed in a test** for what each does, in all four versions on both machines.

**In *Settings 16*'s own words:** *"A sample count is a **ceiling, not a quantity**. In Cycles, five things stand between the number a run is given and the number of samples a pixel actually gets."* Its measured answers, each identical in all four versions on both machines: adaptive sampling *"turns the count into a maximum, and it is on unless somebody turns it off, at a threshold of 0.01"*; *"The Cycles Time Limit is spent on each frame, not across the whole run, and a frame cut short is still written"*; *"a view layer's own number stands in place of a count supplied from outside under `Use`, is capped by it under `Bounded`, and is thrown away under `Ignore`"*; *"Sample Subset decides how many samples are rendered, instead of the maximum"*.

**The joined answer names the five as a list**, and this is the one place in the project where they are enumerated: *"sampling that stops early, on unless switched off; a time limit; a subset; a view layer's own count; and the rule deciding what to do with that count."*

**That list closes item 10's unplaced-fifth gap, 2026-08-23.** [Section 13](../Render%20Settings.md#13-unresolved) item 10 used to say one of the five was unplaced and owed a lookup against *Settings 16*. The lookup was not owed: the enumeration above is the document's own joined answer, quoted in this entry already, and C1 covers all five — item 2 the early stop, item 3 the time limit, item 5 the subset, and **item 4 the last two in one act**, since setting the layer-samples rule to `IGNORE` is the rule deciding what to do with the count and is what throws every view layer's own count away.

**Not covered:** *Settings 16* records *"the rule by which the several limits combine when more than one is in force"* as **not in the sources in all four**, *"because the part of Cycles that runs a render session is missing from every version's file snapshot, which holds the render kernel and utilities only"* — which is C2's ground, word for word. Whether any displacer behaves differently on a card is in that comparison's own not-established list — *"whether the Time Limit is counted the same way on a graphics card"* — and is entered at [U10](#u10-the-displacer-set-the-combining-rule-the-card-side-and-eevee-c2-c3-c5-section-13-item-10). **And the comparison names the device of every displacer run, in its own machine conditions**: *"Every Cycles render behind question 16 was on the processor, not on the graphics card"*, in every version on both of its two machines — the Creator's Mac and *"the rented Linux computer with an RTX 4060 Ti and no display server"* — so that *"nothing under question 16 says anything about a render on a graphics card"*. *(Corrected 2026-08-30 by the downstream citation audit. This read **names the device of no displacer run at all**, which is what C5 was struck and rebuilt on, 2026-08-23. The comparison does record the device, and it records the processor. The card side stays open either way, so nothing that stood on this moves.)*

## B14. The layer-samples rule — its three answers and its starting value (Q2 item 1, C1 item 4, C6)

**Owned by:** *Settings 16*. Label: **documented** for the choice, **observed in a test** for the table.

**The table it establishes**, with the scene's count supplied at launch and identical in all four versions on both machines: under `Use` the second layer took its own 64 against a scene count of 1024; under `Bounded` the same; under `Ignore` it took 1024. *"The default is `Use` in all four versions."* And: *"A count handed to a render can be silently displaced by a number stored inside the file."*

**Not covered:** *Settings 16* names the choice by the manual's own words — Use, Bounded and Ignore. **The identifiers the reading returns are not in it**; those are this document's test's, [Part 2](#part-2--the-controlled-test-of-2026-08-19) item 9, and the document labels them so in place.

## B15. EEVEE — the two features that raise the count, and the per-version override split (C3)

**Owned by:** *Settings 16*. Label: **observed in a test**, both halves; the per-version half additionally carried by 5.0's release notes.

**What it carries:** *"EEVEE takes exactly the number it is given, except for two features that raise it"* — Render Samples 64 with motion blur at five steps reported 65 of 65; Render Samples 16 with depth of field and Jitter Camera on reported 19 of 19. *"The two features are documented: Jitter Camera adds depth-of-field samples by a stated formula, and with motion blur "the render sample count is rounded up to the next multiple of steps"."* And the split: *"**A view layer's own sample count changes an EEVEE render in 5.0, 5.1 and 5.2, and does nothing in 4.5.**"* — second layer 256 of 256 in 4.5, 8 of 8 in the newer three. *"Cycles obeys a view-layer override in all four versions… It is EEVEE alone that changed."*

**Not covered, and it is item 10's second half:** what the **scene-level** layer-samples rule does to an EEVEE render. *Settings 16*'s own not-established list carries *"whether the Layer Samples control is shown only when an override exists, and whether the setting still bites when it is hidden"*, and the table above it is Cycles'. Entered at [U10](#u10-the-displacer-set-the-combining-rule-the-card-side-and-eevee-c2-c3-c5-section-13-item-10).

## B16. Nothing announces a displacement while it happens (Section 5's constraint, C2)

**Owned by:** *Reports 11*, as the joined answer routes it. Label: **observed in a test**, all four versions on both machines.

**In its own words:** *"**Nothing else is printed.** No notice, no warning, nothing naming the time limit or the noise threshold, in any version on either machine. Every one of the twenty-four runs exited 0 and wrote its picture, whether the frame finished or not."* The only printed sign is positional — *"the only reliable printed sign that a frame stopped short is a line reading `Sample 0/<maximum>` at the end of that frame's sampling. It is not a message, it is not an error, and it is not distinguishable from a normal line by anything but its position."* And the one sample figure written into a saved image *"is the maximum asked for, not the number reached"*.

**Not covered:** *Reports 11* is Cycles' answer for the sample line; EEVEE *"counts differently and says nothing about it"* and Workbench prints no sample line at all. Nothing this document does reads the printed line — Q8's rule from `Rented-server runtime` §11.4 forbids it — so the comparison is cited for the silence and not for a route.

## B17. What decides the engine list, and each version's own identifier (Q2 item 4, E1, E4, Section 6.1's constraint)

**Owned by:** *Settings 13*, with *Images 9* for the script route. Label: **observed in a test**.

**What *Settings 13* carries:** *"**What decides the list is what is registered in the running Blender.**"* *"**Blender's own data cannot be read for the list.** Asked for the choices on the engine property, a factory-startup Blender prints exactly one entry, its own EEVEE, in every version on both machines."* *"`-E help` prints identifiers, not the human labels."* The identifiers: *"`-E help` prints `BLENDER_EEVEE_NEXT` in 4.5, and `BLENDER_EEVEE` in 5.0, 5.1 and 5.2."* And the route split, which is where 6.1's constraint comes from: on the command line 4.5 takes its own name only and *"`BLENDER_EEVEE` errors, writes nothing, then hangs"*, while 5.0, 5.1 and 5.2 take both; set from a script every version takes only its own (*Images 9*). *"**The name that works everywhere, by every route, is the version's own.**"*

**The no-single-symptom half:** *"On the Mac, 4.5 ended with `libc++abi: terminating` and exit code 134. On the one-card Linux machine, 4.5 did not end at all, had to be killed at sixty seconds for exit code 124, and printed nothing at all before hanging. On the two-card Linux machine, 4.5 also had to be killed… but it printed its full engine listing first."* Four machines agree 4.5 ends badly; *"none of the symptoms is chosen between."*

**Not covered:** *Settings 13*'s route to the real list is `-E help`, which **4.5 cannot be relied on to answer** — it printed its full listing on two of the comparison's four machines and nothing at all on the other two, and on every one of them the run then failed to end, as the paragraph above sets out. The two-route reading Q2 item 4 uses — the property enumeration plus the engine-class walk — is posed by no comparison and is this document's test's, [Part 2](#part-2--the-controlled-test-of-2026-08-19) item 7. Also not covered anywhere: *"whether a 5.x command-line run under the old name really renders in EEVEE, and whether it warns"*, and anything at all about a third-party engine.

## B18. What a render does when the engine's add-on is not there (E2, E3, E5; Section 13 item 5)

**Owned by:** *Settings 13* and *Add-ons 7*. Label: **observed in a test**, all four versions on both machines.

**In *Settings 13*'s words:** with the Cycles add-on off, a file saved with Cycles *"printed `Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)`, the scene's engine read back as EEVEE's identifier, the printed list dropped to two, and the render ran on EEVEE at EEVEE's own 64 samples rather than the file's Cycles setting of 4… **Anything that trusts a file's engine name to survive must not.**"* *Add-ons 7* adds the exit status — *"the process exits 0. Nothing in the exit status says the wrong engine was used"* — and the two message shapes, `Error: …` in 4.5 against a timestamped `reports | ERROR …` line from 5.0.

**Not covered — and the sentence this entry rested on is gone. Corrected 2026-08-31, out of the repaired Blender research.** This entry carried, as *Settings 13*'s own words, that *"anything involving a third-party render engine"* is **untested in all four versions everywhere**. **The repaired research no longer says that**: *Settings 13* now settles the case through ***Add-ons 7***, which is the run. **Where the engine is named at launch and its add-on is absent, no image is written — 5.0, 5.1 and 5.2 end at 1, and 4.5 does not end at all**, killed on the Mac at status 134 and never exiting on its own on a rented Linux machine. Label: **observed in a test**. **Only the launch-timing half is left unasked.** **What breaks:** the *Cycles switched off* case above is a **fallback** — the render ran on EEVEE and exited 0 — and this entry's *untested everywhere* sentence was what stopped that fallback being read as the general shape. It is not the general shape: with the engine named at launch there is **no fallback and no image**, and on 4.5 **a rented machine can be left running a job that never ends**. `Server Setup`'s A9 entry rests on the same repair and is corrected the same day. **What is still not covered:** the case remains *Cycles switched off* for [B18](#b18-what-a-render-does-when-the-engines-add-on-is-not-there-e2-e3-e5-section-13-item-5)'s own claim, none having been installed and installing one not allowed.

## B19. The one add-on case a file announces, and the 4.5-only trace (E5, Row 154)

**Owned by:** *Add-ons 1* and *Add-ons 7*, which the Row 154 file names for this claim; *Settings 13* carries the same announcement from the other side. Label: **observed in a test** — the printed announcement and the fallback in **all four versions on both machines**; the window half in **all four versions on the one-card Linux machine and in 5.2 only on the Mac**, the two agreeing for 5.2, which is *Add-ons 7*'s own bound and is narrower than the rest.

**What *Add-ons 1* carries:** *"Nothing in any version's manual, API documentation or release notes describes a record inside a `.blend` file of the add-ons that file needs, and nothing describes a way to read such a record out of one."* And the trace: **4.5** opening a Cycles file with Cycles off *"reports the engine as `BLENDER_EEVEE_NEXT`… and — this is the part that matters — still lists `cycles` among the scene's keys"*, while **5.0, 5.1 and 5.2** *"list **no keys at all**"*, 5.0's release notes stating the change. *Add-ons 7* carries the moment the announcement arrives: *"The same sentence appears as a red warning along the bottom of the screen, and it appears as soon as the file is opened rather than when a render is started."* It also carries the exit status — *"the process exits 0. Nothing in the exit status says the wrong engine was used"* — and the two message shapes, `Error: …` in 4.5 against a timestamped `reports | ERROR …` line from 5.0. *Settings 13* has the same case from the engine-list side, in all four versions on both machines: with the Cycles add-on off a Cycles file *"printed `Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)`, the scene's engine read back as EEVEE's identifier, the printed list dropped to two, and the render ran on EEVEE at EEVEE's own 64 samples rather than the file's Cycles setting of 4."*

**E5's label was corrected on 2026-08-23, and the comparisons are stronger than the label was.** The announcement carried **Creator-stated** (2026-08-13) in the document's body, faithful to the spec. Both comparisons above were opened that day — *Add-ons 7* by the map's own naming, *Settings 13* already cited by the document — and both **observe** it, including the one thing the label made no claim about: that it arrives as soon as the file is opened rather than when a render is started. The body now carries the observed label with each half's own bound. **What is not touched is what is separately unestablished**: what form the announcement takes to a run with no window, which is [U5](#u5-what-form-the-one-add-on-announcement-arrives-in-e5-section-13-item-5) and stands exactly as it did.

**Not covered, and it is item 5 entire:** what form the announcement takes to a run with no window. *Add-ons 7*'s window half is the Creator's and the agent's at a screen; the windowless half of the same case is a **printed line** — the shape Q8's rule from `Rented-server runtime` §11.4 forbids reading an ending from. Nothing in either comparison gives a readable field. Entered at [U5](#u5-what-form-the-one-add-on-announcement-arrives-in-e5-section-13-item-5).

## B20. A scene name no scene bears, and a render told nothing (J7's constraint, J9, Y10)

**Owned by:** *Settings 12*. Label: **documented** for the naming argument, **observed in a test** for both behaviours, all four versions on both machines.

**In its own words:** *"**A name no scene bears is reported and then ignored.** The terminal printed `Can't find scene: 'NoSuchScene' in file: '…'` and then rendered the same scene it would have rendered anyway, saved the image and exited cleanly… **Anything that relies on a bad scene name failing will not get a failure.**"* And: *"**Told nothing, it renders the scene that was showing when the file was saved.**"*

**Not covered, and it is the fallback the document builds nothing on:** *"The test files were built and saved by a Blender with no window. Whether a file saved from the interface records the active scene the same way was not tried anywhere."* Y10 names that gap as `Rendering`'s and `Queue & History`'s and does not re-carry it.

## B21. Frame lists — order, mixing, repeats, and the malformed list (Section 3's constraint, F1, F3, J12)

**Owned by:** *Frames 7*. Label: **observed in a test**, all four versions on both machines.

**In its own words:** *"A comma-separated list of single frames works, and is rendered in the order written rather than sorted"*; *"The two forms can be mixed in one argument"*; *"**A repeated frame is rendered twice**, over the same file… Duplicates are neither rejected nor collapsed"*; and *"**A malformed list is refused, nothing is rendered, and Blender still exits with the code 0** — in all four versions on both machines, with the same three messages, word for word: `Error: incorrect comma use`, `Warning: negative range ignored`, `Error: not a number`."*

**Not covered:** the list length limit is the operating system's rather than Blender's — *"macOS said `argument list too long` and returned 127, and Linux said `Argument list too long` and returned 126"*. Nothing in this document bounds the composed list, and no clause reads that limit.

## B22. What a frame range is, the step rule, and the two malformed triples (F2; J12's triple bullet)

**Owned by:** *Frames 8*; the joined answer [What a file can be asked before it renders](../../Blender%20Research/Across%20topics/What%20a%20file%20can%20be%20asked%20before%20it%20renders.md) restates both malformed cases in one sentence. Label: **documented** for the three numbers, **observed in a test** for the behaviour, all four versions on both machines.

**What *Frames 8* carries, in its own words:** *"**A step that lands on the end frame renders it.** Start 1, end 10, step 3 gave frames 1, 4, 7 and 10."* *"**A step that overshoots does not reach back for the end frame.** Start 3, end 12, step 4 gave 3, 7 and 11."* — which is F2's arithmetic. It also carries the two cases J12 calls unposed: *"**A step of 0 is accepted, reads back as 0, and writes the first frame once.** No version hung, and all four ended on their own, leaving one file, on both machines"*; and *"**An end below the start renders nothing at all** and exits with the code 0, in all four on both machines."* The joined answer's one-line form: *"A step of nought writes the first frame once; an end below the start writes nothing at all. *Frames 8*"*.

**The document was repaired against this entry on 2026-08-23, and the publication was right.** [J12](../Render%20Settings.md#24-what-the-page-refuses-and-what-a-refusal-states)'s triple bullet used to state that a step of zero and a last frame before the first *"have no established counterpart to state"* and that *"what Blender does with those two was posed by nothing."* **Both were posed, in all four versions on both machines.** *Frames 8* was opened on 2026-08-23 — the map names it as the one question answering what a frame range consists of — and carries both cases in the words quoted above; the joined answer restates them in one sentence. The bullet now states both, with the exit-code bound *Frames 8* itself records (0 for 5.0, 5.1 and 5.2 on the step of 0; not recorded for 4.5 on the Mac) and with the route bound below.

**Not covered by *Frames 8*:** both cases are posed as scene settings and command-line arguments, never as anything reaching Blender through an explicit frame list — which is the only route F3 composes.

## B23. The resolution percentage — its range, where it lives, and what it does out of range (Q2a, J11, J12, C7, Y1; Section 13 items 24, 25 and 26)

**Owned by:** *Settings 14* for the values and the setting's home; *Settings 15* for what a render does with a value outside them. Label: **documented** for the range, **observed in a test** for everything else, all four versions on both machines.

**What *Settings 14* carries:** *"**Whole numbers from 1 to 32767 inclusive, and the setting starts at 100.** There is no zero, no negative number, and no fraction"*; the outright limits and the dragging limits are two different pairs, and *"the property reports itself as not animatable"*; and *"**Where the setting lives is the same in all four versions** — on the scene's render settings, not on any render engine."*

**What *Settings 15* carries:** *"**The value is corrected the instant it is assigned, and nothing whatever is said about it.** Below the range stores 1; above it stores 32767. No error, no warning, no line of any kind. A control in the same run — assigning text, which is refused with a complaint on that same channel — proves the silence is real and not a channel that never speaks."* *"**A fraction is refused, not rounded.** 50.7 and 0.4 are both refused with an error naming the setting. So is text, and so is a number too large for a machine whole number."* *"**Anything that judges a render by its exit status will never learn that the percentage it asked for was not the one used**"* — the exit-status control fired in every run at 42, and a corrected run came back 0. The four measured pairs Q2a's arithmetic is read off: *"1920 by 1080 at 200 per cent gives 3840 by 2160; at 33 per cent, 633 by 356; at a request of 0 per cent — stored as 1 — 19 by 10; and 4 by 4 at 32767 per cent gives 1310 by 1310."* And *"**A request that works out to less than one pixel is refused, and no file appears.** 4 by 4 at 1 per cent stopped with `Image too small` in all four versions on both machines."*

**Not covered, and all three gaps are the document's own unresolved items:** neither comparison states the truncation rule — *Settings 15* measures four pairs and no version's documentation states a rule at all ([U19](#u19-q2as-truncation-rule-section-13-item-25)); neither carries the **per-version property spelling** the pair at C7 needs ([U18](#u18-the-resolution-percentage-pairs-spelling-c7-y4-section-13-item-24)); and both are the **percentage** setting's evidence alone. *(That third gap read *which is why J12 declines to carry the silent-correction sentence to the samples box*, and **it is spent — corrected 2026-08-31**: the sample counts have their own evidence now, at [B23a](#b23a-the-sample-counts--what-each-accepts-and-what-blender-does-with-a-number-outside-it-j11-item-3-j12s-two-samples-bullets-c1-c9-section-13-items-26-and-31), so J12 states the silent correction for that box on the counts' own showing and carries nothing across. The bound itself stands: nothing here may be carried from one setting to another, which is what made the second lookup necessary.)* *Settings 14* also records as not established what the `%` field does when an out-of-range number is typed into it — a window question, and Design's.

## B23a. The sample counts — what each accepts, and what Blender does with a number outside it (J11 item 3, J12's two samples bullets, C1, C9; Section 13 items 26 and 31)

**Added 2026-08-31. This entry was owed and missing.** The document states at *Where a Blender fact is looked up is fixed* that it is owed — *"What is owed is the sources file's own entry for the question, which is the runner's and is routed rather than written"* — and until now every Blender claim in J11 item 3 and J12's two samples bullets traced through `Facts.md` alone, with no citation in this file at all.

**Owned by:** ***Settings 18***, **read here through [Facts](../Facts.md) Fact 127's register entry rather than out of the comparison**, which is what the document itself does and says it does. That register entry is the citation; the comparison's own text is not quoted here and was not opened. Label: **observed in a test** — all four versions on two machines, the Creator's Mac (4.5.12, 5.0.1, 5.1.2 and 5.2.1, macOS 26.6.2) and a rented RTX 3060 Linux machine whose four builds were checked build for build against the Mac's before anything ran.

**The route the question came by, because it is not the map's.** Fact 127 was named by the Rows 435–438 implementer on **2026-08-23** against J11's closed set of three refusals, numbered by the runner the same day, **registered blind as *Settings 18* on 2026-08-29**, and **answered on 2026-08-30** — version-isolated passes, hand-tested and comparison-checked, and repaired the same day by the cross-sweep and the mechanism sweep. It is the fourth of the four lookup routes the document names, and the one place in the document where a Blender fact arrives through the register.

**What the register carries, in the terms the clauses use.**

- **There is no one sample-count setting.** Each count declares its own range and each is corrected against its own limits, so the same number is cut down for one engine and taken whole by another. **Cycles' final-render count takes 1 to 16,777,216; EEVEE's takes 1 to 2,147,483,647** — read out of each running Blender with no file open, identical in all four versions on both machines. A view layer's own override takes 0 to 2,147,483,647, where 0 means no override. **Workbench's is not a number at all** — a fixed enumeration, matched case-sensitively, a choice outside the list refused with every real choice named, which is the correction C9's Workbench row carries.
- **Out of range is corrected in silence and paid for.** **A count outside the accepted range is corrected to the nearest limit at assignment, nothing whatever is reported — no error, no warning, no line of any kind — and the corrected number is what the render works to and is charged for.** Cycles asked for 99,999,999 stores 16,777,216 and settles in to render that many, the remaining-time estimate running roughly nine to fourteen hours across the four versions; at the other end a count of 0 stores 1, renders its frame, saves it and exits 0. **A control makes the silence a result rather than a quiet channel:** the same runs made one deliberately wrong assignment and it was refused loudly.
- **Where the corrected figure can be read, and it is not everywhere.** Two homes carry it and neither announces it — the setting read straight back, and the ordinary progress line's number after the slash. **The progress line is conditional twice over** — a sample subset or a view layer's own count makes that number count against itself instead (*Settings 16*), and neither was put on the same file as an out-of-range count — **and on the render-operator route no progress line is printed at all in 5.0, 5.1 and 5.2** (*Reports 3*), leaving the read-back the only reading. That is the ground J11 item 3 gives for imposing the range **at the box**, nothing below it being able to catch the correction.
- **What is refused rather than corrected, and the line is exact.** Above the setting's own ceiling but **inside a machine whole number**, silently corrected. **Past what a whole number can hold, refused outright with an error naming the setting, and the value already there stays.** A decimal, a word, text that looks like a number, or nothing at all is likewise refused with an error naming the setting and the kind of value it expected — nothing rounded, nothing truncated. **A true-or-false value is the one exception and stores 1.** **The whole-number refusal is one behaviour written two ways** — ordinary figures on macOS, a shorter number notation on Linux.
- **The route is the one Smoozy composes.** Every result came off a Blender started with no window and driven by a script handed in at launch — the shape `Rendering` builds and C1 assigns through.

**Not covered, and these four travel with the answer** — they are *Settings 18*'s own bounds, and the document carries them at [Section 13](../Render%20Settings.md#13-unresolved) item 31: **a driver or a keyframe carrying a count out of range**, unposed; **a file *holding* such a count, loaded rather than assigned**, not established, every observed correction being a correction at assignment; **a field typed into or dragged**, everything having been set from a launch instruction, and the two dragging limits the answer records are limits on dragging alone; and **Windows**, where no machine ran any part of it — which matters for the whole-number refusal's two written shapes, a third on Windows being possible and untried.

**What breaks if it is wrong:** J11 item 3's range and J12's two samples bullets are the whole of what the page tells the user about this box, and they are one fact — the range is imposed because the correction is silent. If the correction were loud, or the ranges different, the box would be refusing values Blender takes and the line would be stating a range that is not the setting's. **What holds regardless:** J11 item 3's imposition is a bound on what the page accepts, and it stands whatever any of the four uncovered cases turns out to be; and no field of Blender's is ever typed into or dragged by Smoozy, so the third bound reaches nothing this document composes.

## B24. Which card renders, and where a device request comes from (Section 8.1's constraint, D2, D3, D4)

**Owned by:** *The machine 5* and *Settings 9*, with *What a render demands 3* and *What actually ran 4*; routed by [What can be set from outside](../../Blender%20Research/Across%20topics/What%20can%20be%20set%20from%20outside.md). Label: **documented** for the argument, **observed in a test** for everything else.

**What they carry:** *The machine 5* — *"Three settings, and all three must line up"*, and the argument *"names a technology and has no way of naming one card out of several"*, and where there is a choice *"it takes every card of that technology, whatever the preferences said"*. *Settings 9* — *"**A launch argument beats both the file and the machine's preferences**… The argument only works after a bare double dash"*; and the quiet case D4 rests on: *"**A file asking for the graphics card when the machine has none to give renders on the processor, writes the image, and exits 0**… Anything that needs to know whether a render actually used the graphics card cannot learn it this way."* The loud case, which is `Rendering`'s and is cited there: a technology named at launch with no hardware behind it *"stopped the render with `Found no Cycles device of the specified type`, exit code 1, no image"*.

**The three routes, and the fourth that is not among them — D3's ground and item 21's:** *Settings 9* establishes the launch argument, the machine's saved preferences, and the value saved in the file. **Assigning the scene's own device setting from a script handed in at launch is not one of them**, and the same comparison records that the 5.x callback attached to that setting *"when the value changes"* was *"recorded nowhere and no test touched it on any machine."*

**Not covered, and it is item 20's:** *"what a file's "GPU Compute" means on an AMD or an Intel graphics card. No machine the project has had carried one — Apple's own graphics on the Mac, NVIDIA cards on every rented machine."*

**And one condition on every card reading, carried 2026-08-31 out of the repaired Blender research.** **Every reading of a machine's cards on record was taken after a refresh was asked for; asked cold, a four-card machine and a cardless one give the same answer** — the repaired *The machine 5* records the windowless runs coming back **empty** in all four versions with an untouched settings folder, and empty again under factory settings, filling only once a refresh was asked for. Label: **observed in a test**. **Nothing here breaks:** D2 to D4 read no card list, `Servers` H16 declines that route outright and the repair strengthens its third reason, and D4's quiet case is a render's own outcome rather than a listing. **It is recorded because a later clause reading a card list must not treat an empty answer as a cardless machine.** **A conflict sits beside it, stated and not resolved:** the multi-provider batch's record says every individual device arrives switched on, and that batch's own record does not say whether its script called a refresh — `Rendering`'s sources file M13 carries it, and nothing here reconciles them.

**Where the rule now lives, and this document was checked for the same reliance and has none. Added 2026-09-02.** The safe reading is `Server Setup` **R1a** — a Blender device list is read only after a refresh has been asked for, and an unrefreshed empty list is never read as *this machine has no cards* — with the conflict written as permanently unresolved at that document's §13 item 25 and at `Servers` §16 item 24, the Creator having ruled on 2026-08-31 that no further Blender research and no further rented machine will settle it. **This document reads no card list at all**: [D5](../Render%20Settings.md#82-the-processor-on-a-machine-with-cards-warning) takes the hardware reading from `Servers`' remembered one and says in terms that there is no Blender in it, and D6's table keys on `Servers` H11's card states, which come from the bus scan and the driver's own query. **So nothing here moves**, and the entry stands as a condition on any later clause rather than on a present one.

## B25. Each version's own recorded breaks (B7, B8; Section 13 item 16)

**Owned by:** *Settings 10*, which the Row 156 file names as the material's source and which the map names for reading a file across versions. Label: **documented**, from each version's own release notes.

**The body was repaired on 2026-08-23 to name it.** B7 and B8 stated the break-note material and cited nothing: *Settings 10* was not in the document's own *Where a Blender fact is looked up is fixed* paragraph, so the trail ran from this entry to the comparison and never back the other way. The comparison was opened that day — its question and no more of the file — and its words are quoted below unchanged; B7 now carries its statement and its label, B8 carries 5.0's two render-side breaks in its words, and the paragraph lists it among the comparisons opened.

**In its own words:** *"Each version's own release notes record its own breaks, and these are statements about opening files across versions, so they are answers to this question."* **The two silent breaks B8 names are in 5.0's entry:** *"It also names two render-side breaks: a material property whose meaning had been inverted since 4.2 is fixed, so files saved from 5.0 will not bake the same lighting in older Blenders, and view layer overrides are newly supported so older files using them will render differently."* The general statement all four share is the same page word for word, and the tested behaviour is the same on both machines: a 5.2 file opened with no window by the older three printed *"`File written by newer Blender binary (502.44), expect loss of data!`"* and then rendered and saved, exiting cleanly.

**Not covered, and it is B9's whole reason:** the notes run 4.5 to 5.2 and no further. *Settings 10* is a comparison of four versions; it says nothing about a version that does not exist yet, which is item 16.

**2026-10-02: B7 and B9 are struck and B8 composes no notes**, on the Creator's ruling that the warning is one line that opens nothing. Nothing in the document now uses this entry's material; it stays as the record of what B7 and B8 cited, and the spec still cites *Settings 10* as what the line rests on.

## B26. A value read out of an older file is not always the value saved (Section 1.4's constraint, Q11)

**Owned by:** *What a render demands 5*. Label: **observed in a test**, both machines agreeing line for line.

**In its own words:** *"A file saved by Blender 4.5 with Use Tiling off and Tile Size 256 reads back **as saved** in 4.5, and as Use Tiling **on** with Tile Size **8192** in 5.0, 5.1 and 5.2… **So a tile size read out of an older file through a newer Blender is not necessarily the tile size that was saved.**"* The same comparison carries the add-on half: *"**Cycles' values cannot be read with the add-on off**, in any version, with identical wording."*

**Not covered:** *"No version documents a way to read a file without Blender"*, so nothing in this project reads a `.blend` except by opening it — which is why Q1's reading is an invocation and not a parse.

## B27. A first frame may be zero or negative (V11)

**Owned by:** the [Render Settings spec](../../Specs/Render%20Settings.md) and its [companion](../../Specs/Sources/Render%20Settings.sources.md), not by any comparison. Label: **Creator-stated** (2026-07-27), which is the label the document carries in place.

**In the spec's own words:** *"**A file's range can start at any frame number.** Not necessarily 1 — it may be 10, or 0, or a negative number, all of which Blender allows."* Its companion records why there is nothing further: *"The Creator holds that this one needs no confirming, and it was deliberately not researched."*

**Not covered, and it is the whole of the citation:** **no comparison question poses it.** *Frames 8* establishes what a frame range consists of and the step rule ([B22](#b22-what-a-frame-range-is-the-step-rule-and-the-two-malformed-triples-f2-j12s-triple-bullet)) and poses no negative or zero first frame anywhere. **Breaks if wrong:** V11's minimum is arithmetic over signed integers; a one-based or positional assumption anywhere below it would draw the wrong frame in a box. **What holds:** Rule 2 admits a Creator-stated fact, and nothing here rests on more than the sign.

## B28. The names the launch-time pairs use, the near-misses, and the group that swallows a wrong name (C7, C9, V4, Y4; `Rendering` L8, L10a, L11; Section 13 items 24 and 28)

**Owned by:** *Settings 17*, registered on 2026-08-29 for [Section 13](../Render%20Settings.md#13-unresolved) item 24 and answered by all four versions the same day. Label: **documented** from each version's own API page for every name but Cycles', which has no page for its render settings in any of the four and is **observed** out of Cycles' own shipped files; **observed** for everything each name does, and for the swallow; writing a name out whole from a scene is **inferred** in every version except where one API page spells `bpy.data.scenes[0].render.resolution_percentage` out. **The names come from the API documentation and from Cycles' files, never from the manual** — every version's manual was searched and names these settings by their interface labels alone.

**The names, in the comparison's own table and the same in all four versions:** `resolution_percentage` on `scene.render`; Cycles' `samples` on `scene.cycles`; EEVEE's `taa_render_samples` on `scene.eevee`; Workbench's `render_aa` on `scene.display`, which *"takes the value as text and not as a number"*; a view layer's own `samples`; and a view layer's `use`. **The near-misses exist in all four versions too, under the same name, label, description and starting value**: `use_single_layer`, Cycles' `preview_samples`, EEVEE's `taa_samples`, Cycles' `use_layer_samples` and Grease Pencil's `aa_samples`.

**The three sentences the document is built on, in the comparison's own words.** *"A wrong name does not stop the render and does not show in the exit status"* — three names deliberately misspelled, the frame rendered and saved, exit 0, the refusal on the terminal and nowhere else. *"In Blender 4.5 the complaint is missing as well, on Cycles' settings"* — two invented names both accepted with no error of any kind, the value *"gone the instant it is read back"*, while 5.0, 5.1 and 5.2 refuse and every other settings group refuses in all four. And *"A correct name handed in before the file is loaded prints the right value back and has no effect"* — both orders printing the value set, only the order after the file changing the picture, and *"The readback does not even warn"*.

**One thing runs the opposite way and the document keeps the two apart** (`Rendering` L8): an unknown *switch* on the launch line is not swallowed — `--render-percentage 50` was taken for a file name in all four versions, *"wrote no picture at all, and exited 1"*.

**How much was run:** fifteen tests, each with a control saying what a negative would have looked like, in all four versions on 2026-08-29, every one re-run by a second reading — sixty runs and sixty repeats, no repeat disagreeing — every run from the factory settings and Blender's own default scene.

**Not covered, and it is [Section 13](../Render%20Settings.md#13-unresolved) item 28 and `Rendering` §9 item 25:** **one machine ran every part of it — the Creator's Mac, macOS on Apple silicon, nothing rented** — so whether any of these names holds on Linux, where Smoozy renders, is the comparison's own largest gap, and no Windows machine ran any of it. Nor whether these are the only names; nor a count belonging to a whole scene rather than to one engine; nor a name for any engine beyond the three; nor whether Workbench's anti-aliasing is a sample count at all; nor what an add-on does to any of them. **And the comparison poses `Rendering` L10a's check itself nowhere** — the object-does-not-carry-the-name answer is taken after the swallowed assignment, never before one and never as a check.

## B29. The five names C1 items 2 to 5 and D3 assign against, read off four running Blenders (C1, C10, D3, Y4; Section 13 items 28 and 29)

**Owned by nothing in the Blender research, and it is not a comparison.** *Settings 17* was posed on the count and pin names alone and does not carry these; *Settings 16* answers the imposition set in the manual's own interface words and carries no property name; *Settings 9* names no path on a scene. **The source is a lookup run at Build on 2026-09-20**, which is the work `What Build inherits` item 8(a) named as Build's own — no rental, no research round, no fixture, no render. Label: **observed (2026-09-20 lookup)**.

**Where it is recorded:** `Audits/Build/Step 10c/The property-name lookup — what was read.md`. That file holds, per version and per name, the full path written out from the scene, the kind, the starting value, the label and the description as Blender gave them, the commands that produced them, and what differed between versions.

**What was run.** Each of `/Applications/Blender 4.5.12.app`, `/Applications/Blender 5.0.1.app`, `/Applications/Blender 5.1.2.app` and the unnumbered `/Applications/Blender.app` — **which was asked its own version and answered 5.2.2 LTS** — was started with no window and from factory settings, ran a script that printed the whole property list of the objects in question with each property's kind, starting value, interface label and description, and exited. **Nothing of the Creator's was opened, installed, written or saved.** The lists were printed whole and then matched against what was wanted, never searched for a wanted name.

**The names, the same in all four versions, all on the job's scene's Cycles settings:** `use_adaptive_sampling`, `adaptive_threshold`, `adaptive_min_samples`, `time_limit` (its off value zero, in Blender's own words *"Zero disables the limit"*), `use_layer_samples` (choices `USE`, `BOUNDED`, `IGNORE`, starting at `USE`), `use_sample_subset`, and `device` (choices `CPU`, `GPU`, the processor value `CPU`).

**How C9's refusal was lifted without assembling anything.** C9 declined `use_layer_samples` because nothing in *Settings 17* says what it governs, and joining two questions to decide it is the assembling [Method](../Method.md) forbids. **Nothing was joined.** The lookup asked, in each version, which property anywhere reachable offers exactly the three choices `USE`, `BOUNDED` and `IGNORE`; the answer was one property and only one, in every version, with `USE` as its starting value.

**The near misses the enumeration turned up, recorded because matching a wanted thing to a near neighbour is this project's own named fault:** `scene.render.compositor_device`, which takes the same two values as D3's pair on the same scene and decides how compositing is carried out; `scene.render.compositor_denoise_device`; and viewport twins of all three of C1 item 2's names — `use_preview_adaptive_sampling`, `preview_adaptive_threshold`, `preview_adaptive_min_samples` — **carrying identical labels and identical descriptions**, so those three are told apart by name alone.

**Not covered, and it is [Section 13](../Render%20Settings.md#13-unresolved) item 28's bound unchanged:** every reading was taken on the Creator's Mac, macOS on Apple silicon; **no name was set on Linux, where Smoozy renders**, and no name was *set* at all in this lookup — it read what each program says it carries, and did not render with any of them. Nor does it say whether these are the only names, nor what an add-on does to any of them. `Rendering` L10a's name check is the backstop, and [U24](#u24-every-name-in-b28-was-set-on-one-mac-and-the-check-written-from-it-was-run-nowhere-c7-c9-rendering-l10a-section-13-item-28) covers it for these five exactly as for B28's.


## B30. A reader that crashes Blender after the file opens, and the files that never reach the reader (Q8)

**Owned by nothing in the Blender research.** Two desk tests run at Build on 2026-10-02, out of the Creator's tests of 0.1.7. Label: **observed (2026-10-02, at the desk)**.

**Where it is recorded:** `Audits/Build/Step 10c/The Creator's tests of 0.1.7/Why Read the file failed — what was found.md` and `Audits/Build/Step 10c/The Local View crash — what landed.md`.

**The crash.** The Creator's own file, saved with 5.1 and left with a 3D viewport in Local View, opens and ends normally in every Blender at the desk with nothing handed in. Smoozy's reader, launched as the machine launches it, ended every one of them by a crash before it wrote anything — 4.5.12, 5.0.1, 5.1.2 and 5.2.2 — at the walk's read of `SpaceView3D.local_view`. With that one property stepped over, the reading finished in all four. A factory scene put into Local View and saved by each of the four crashed the reader the same way.

**The files that never reach the reader.** A corrupt file, a file cut short after its first four thousand bytes, and an absent path, each launched with `Job files and outputs` R18's script before the file and a reader whose first act prints a mark: all four Blenders ended at status 1 with Blender's own one-line refusal, and **none printed the mark**.

**Not covered:** every run was on the Creator's Mac, macOS on Apple silicon, never on Linux, where Smoozy reads. Nor does it say what other file shapes abort between the file opening and the reader starting.

## B31. A scene with nothing on it that changes from frame to frame reads the same at every frame without being stepped (Q3; Section 13 item 22)

**Owned by nothing in the Blender research.** A controlled test run at Build on 2026-10-02 and 2026-10-03, out of the Creator's notes on 0.1.8 (note 4). Label: **observed in a test (2026-10-02 and 2026-10-03, at the desk)**.

**Where it is recorded:** `Audits/Build/Step 10c/The Creator's notes on 0.1.8, piece 2 — what landed.md`, sections 1, 3, 4 and 7. The results files it names sit in scratch, not in the repository.

**What it carries.** Smoozy's reading script, composed by the code before the change (every scene stepped) and after it (a scene stepped only where it carries animation data, its compositing tree or a group tree inside it carries animation data, or a timeline marker names a camera), run over **214 files** — 160 real files from the Local View sweep, 4 earlier uploads and 50 made files of thirteen kinds (switch keyed, driven, or held only through an action strip; compositing tree keyed; a keyed value inside a nested group; markers with and without cameras; an object animated in a still scene; a render setting keyed over 250 frames; a sequencer strip pointing to a still scene; two scenes with one animated; ten still scenes; and in 4.5 alone, Use Nodes keyed). **Each file read by 4.5.12, 5.0.1, 5.1.2 and 5.2.2, with no frames and with frames 1, 2, 3, 10, 24 and 100: every reading byte-identical before and after**, apart from one file whose reading names a temporary folder Blender makes fresh on each run, which differs the same way when the old script reads it twice. Taking out the scene's-animation limb changed 21 of the made files, across all four versions; taking out the markers limb changed the files whose markers name cameras in 5.0, 5.1 and 5.2; **taking out the compositing-tree limb changed nothing in the corpus**, so that limb is kept on the build runner's ruling rather than on a file that needed it. The cost: the Creator's file, ten scenes of one frame, read in about 22 to 23 seconds before and about 2 after, in all four versions; a stand-in made to match it, about 29 seconds before and under 1 after.

**Not covered:** every run was on the Creator's Mac, never on Linux or a rented machine. A per-frame value changed by a script run on a frame change was not posed; the expectation that with scripts off the render agrees with the reading is not tested. Nothing was read at a long animated range, which is Section 13 item 22's remaining case.

---

# Part 2 — The controlled test of 2026-08-19

**This is the one part of this file that records an observation rather than pointing at one.** It was run for this document, on a machine rented for it and destroyed in the same sitting. Every finding is labelled **observed (2026-08-19 test)** where the document uses it, and each is used with the bound stated below. **The item numbers are stable and are cited from outside this file** — `Rendering` L8a and [Rendering sources](Rendering.sources.md) M18 both cite *item 5*.

**The machine.** Vast.ai instance **48153932**, on physical machine **136679**, from offer 48100888, in **South Korea**. A **container**, not a virtual machine, from a bare `ubuntu:22.04` image: Ubuntu 22.04.5 LTS, kernel `6.8.0-138-generic`, overlayfs, 4 effective cores of an Intel Core i3-8300 @ 3.70 GHz, 31.3 GB memory, a 30 GB allocation, one GTX 1080 (unused — every render ran on the processor). Reached at `ssh5.vast.ai` port 33932 through the provider's proxy, as root. About **$0.0489 per hour**; rented on 2026-08-19 and destroyed inside the same sitting — a little under an hour, roughly five cents, the destroy landing in the first minutes of 2026-08-20 on the Creator's clock; the account was checked afterwards and this instance was gone. One other instance stood on the account before this rental and after it, not this work's, and was not touched.

**The Blenders.** **4.5.12 LTS** (hash `84afd5f785f7`, built 2026-07-21) and **5.2.0 LTS** (hash `fbe6228777e7`, built 2026-07-14), fetched onto the box from Blender's own downloads (`download.blender.org/release/Blender4.5/blender-4.5.12-linux-x64.tar.xz` and `…/Blender5.2/blender-5.2.0-linux-x64.tar.xz`, read 2026-08-19), unpacked into plain folders, each asked its version by the line beginning `Blender `. **5.0 and 5.1 were not installed and nothing here speaks for them.**

**The fixture.** Built **natively by each version** from one branching script (`--factory-startup`), saved as that version's own file. Scene `Main`, frames 1–10 step 1, Cycles at 16 samples with adaptive sampling off, 64×64 at 100%, PNG, device left at the processor. Five view layers: `VL_Always` (untouched); `VL_Key` keyframed on the render switch with **constant** interpolation — on at 1, off at 4, on at 8; `VL_KeyLin` keyframed with **linear** interpolation — 1.0 at 1, 0.0 at 10; `VL_Drv` carrying a **scripted-expression driver** on the switch, expression `frame % 2`, which the reading reported valid; `VL_Script` untouched but with a stored text registered as a module, appending a frame-change handler that sets the switch to on where the frame is divisible by 3. Every render was started on the command-line route, no-window mode, animation form, frames 1–10, per the launch contract's shape. Which layers rendered at which frames was read from the run's own per-layer progress lines (`Main, VL_…` in 4.5's `Fra:` lines; `Start rendering: Main, VL_…` under 5.2's frame lines), per frame.

**The reader.** One invocation per version — `-b <fixture> --python <reader>` — emitting one JSON document between `SMOOZY_READ_START`/`SMOOZY_READ_END` sentinels: the per-frame switch values by stepping frames 1–12, the curves by the per-version route, the drivers with validity, and the engine enumerations.

### What was found, per finding

1. **One read answers per frame for a driver-driven switch.** The read's frame-stepping gave `VL_Drv` on at odd frames and off at even, in both versions, under default preferences; the control render rendered exactly those frames. Read and render identical, frame for frame. *A negative would have looked like*: the read reporting the driver's stored value at every frame while the render alternated.
2. **The scripted switch, both sides of the one preference.** Under default preferences both versions printed `scripts disabled for "<file>", skipping 'smoozy_handler.py'` and neither the read nor the render ran the handler — `VL_Script` read and rendered everywhere, identically. With the autoexec launch switch on, both versions printed `SMOOZY_HANDLER_REGISTERED`, and the read and the render both gave `VL_Script` on exactly frames 3, 6, 9. Read and render agreed under both states.
3. **A launch assignment loses to the file's animation, totally and silently.** `--python` scripts assigning the switch inside the window: on `VL_Always` (no animation) the assignment held — pinned off, it rendered on no frame, both versions. On `VL_Key` (keyframed) the same assignment — tried on and off — was ignored on every frame: the render followed the curve exactly, exit 0, both versions. On `VL_Drv` the driver redrove it the same way.
4. **Severing then assigning holds.** Removing the switch's curve — in 4.5 through the action's own curves, in 5.2 by walking the action's layers, strips and channel bags, removal by the channel bag's own remove — then assigning: pinned **on**, `VL_Key` rendered on all ten frames against a curve saying otherwise; pinned **off**, on none. Both versions. Removing the driver by `scene.driver_remove('view_layers["VL_Drv"].use')` (returned True, both versions) then assigning off: `VL_Drv` rendered nowhere. The pinned-on direction was not posed on the driver. Nothing saved the file in any run.
5. **A raising launch script does not end the run.** A script raising a deliberate error after the file: both versions printed the traceback, **rendered anyway** (three frames written), and **exited 0**. With `--python-exit-code 63` on the line, both versions **ended at exit 63 with no frame written**. *A negative for the first half would have looked like*: the plain run stopping before the render instruction.
6. **A file that cannot be opened never reaches the handed-in script.** Against a 2 KB random-bytes file: exit 1, Blender's own line `Error: File format is not supported in file "…"`, no sentinel, the reader never ran — both versions. Against an absent path: exit 1, `Error: Cannot read file "…": No such file or directory`, same shape.
7. **Engine identifiers enumerate by two routes together, and neither alone answers.** The engine property's own enumeration returned exactly one identifier — `BLENDER_EEVEE_NEXT` in 4.5.12, `BLENDER_EEVEE` in 5.2.0 — and did not list Cycles even with its add-on on and the scene reading `CYCLES`. The engine-class walk (`bpy.types.RenderEngine.__subclasses__()`, reading each class's `bl_idname`) returned `CYCLES`; one class (`HydraRenderEngine`) carried no `bl_idname` at all and raises if read without a guard. Workbench appeared in neither route.
8. **The curve routes, re-confirmed in the direction the research records.** 5.2.0 refused `action.fcurves` with `AttributeError: 'Action' object has no attribute 'fcurves'`; the layers → strips → channelbags → fcurves walk answered. 4.5.12 answered by `action.fcurves` directly. One aside: 4.5.12 also walked an (empty-looking) `action.layers` without raising; nothing establishes that route reaches curves there and nothing uses it.
9. **Extrapolation and the boolean threshold.** Frames past the last key read and rendered at the last key's value (frames 11–12 read on for `VL_Key`; render posed only to 10). The linearly-interpolated switch read and rendered **on wherever its curve value was nonzero** — frames 2–9, including values well below one-half — and off at exactly zero. Before-first-key extrapolation was not posed. The layer-samples rule's enumeration read `USE`, `BOUNDED`, `IGNORE`, standing at `USE`, both versions.

### Which clause each finding carries

| Item | Clauses in [Render Settings](../Render%20Settings.md) |
|---|---|
| 1 | Q3, [V1–V2](../Render%20Settings.md#41-what-one-read-answers-per-frame); Section 13 item 7 |
| 2 | [V3](../Render%20Settings.md#41-what-one-read-answers-per-frame), V8; Section 13 items 7 and 9 |
| 3 | [V5](../Render%20Settings.md#42-the-pin-and-the-mechanism-that-holds-it) — Row 147's contention |
| 4 | [V4, V6](../Render%20Settings.md#42-the-pin-and-the-mechanism-that-holds-it), Y5; Section 13 item 4's pinned-on residue |
| 5 | [V7](../Render%20Settings.md#42-the-pin-and-the-mechanism-that-holds-it), [Section 12](../Render%20Settings.md#12-what-this-document-supplies-and-to-which-document)'s first requirement; Section 13 item 13. **Cited from outside this file** by `Rendering` L8a and §9 item 19 |
| 6 | [Q8](../Render%20Settings.md#13-all-or-nothing-and-the-three-endings) ending 2 |
| 7 | [Q2 item 4](../Render%20Settings.md#11-the-settings-half-of-the-one-recorded-reading), E1 |
| 8 | [Q2 item 2](../Render%20Settings.md#11-the-settings-half-of-the-one-recorded-reading), [V9](../Render%20Settings.md#43-the-route-to-the-curve-per-version), V6's severing |
| 9 | [Q3](../Render%20Settings.md#11-the-settings-half-of-the-one-recorded-reading), Q2 item 1's layer-samples identifiers, C1 item 4; Section 13 item 8 |

### What this test does not cover

One box, one day, one provider, one fixture family. Renders at 64×64, 16 samples, on the processor only — nothing at a real size, nothing on a card, nothing under measured contention. Blender 5.0 and 5.1 untried. One driver shape (a simple scripted expression; no Python-function driver, which 5.x documentation says needs the automatic-running permission — *Reading a file 3*, [B10](#b10-the-driver-case-and-the-stored-script-case-as-the-research-left-them-v2-v3-section-13-item-7)). One handler shape. The pinned-on direction untried on a driver. Before-first-key extrapolation untried. Interpolation kinds other than constant and linear untried. No file naming an engine the reading Blender lacks could be posed — Cycles was enabled on both builds. **No resolution percentage was posed at all**, in either version, which is why C7's pair has no spelling from here. **No device other than the processor was posed**, which is why D3's route is unobserved. **Where on the launch line `--python-exit-code` sat was not varied.** Windows nowhere.

---

# Part 3 — The stage's frozen input documents, `Facts.md`, and peer answer documents

Every entry in this part is **shape 1**. The document's own rule governs how each machine finding is read: *the frozen machine evidence of the batch of 2026-08-17 is read as **this is what that box did on that day**, never as a general statement.*

## M1. One box with Blender on it §1.3 — Fact 5's second payload, in one read (Q1, Q2)

**Owned by:** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md), §1.3, on 2026-08-17, in 4.5.9, 5.0.1 and 5.2.0. Label: **observed in a test**, one box.

**What it establishes for Q1:** *"**All three payloads come out of one read.** The reader's error map came back `{}` in every final run in every version."* The settings values came back in the same sentinel-framed document as the destinations and the recorded locations — *"one JSON document on standard output between `SMOOZY_READ_START` and `SMOOZY_READ_END` sentinels"*.

**What it establishes for Q2, itemised in that document's own reply:** `frame_start`, `frame_end`, `frame_step`, `frame_count`; `resolution_x`, `resolution_y`, `resolution_percentage` and a worked `pixels [640, 360]`; `engine_as_read CYCLES`; `samples {'cycles': 128, 'eevee': 64}`; `cycles_device GPU`; the early-stop trio; `time_limit 0.0`; `use_layer_samples USE`; `saved_with_blender`; per view layer the name, `use_now`, `animated`, the curve's keys, and `samples_override` with `overrides_samples`; and the scene-pulls-in-scene reading by both routes — *"`{"via_render_layers_node": [], "via_sequencer_strip": [{"strip":"to_second","scene":"Second","use_sequencer":true}]}`"*. **The version difference inside the reading is recorded there too:** the second scene's engine read back `BLENDER_EEVEE_NEXT` in 4.5.9 and `BLENDER_EEVEE` in the other two.

**Not covered:** one box, one day, three versions, **5.1 not installed**. The reading's per-frame answers there were read by stepping and by the curve alike — *"it works either by stepping frames or by reading the curve"* — with no driver and no stored script anywhere in the fixture. Q2 item 4's two-route engine enumeration is not in it and is this document's test's, [Part 2](#part-2--the-controlled-test-of-2026-08-19) item 7.

## M2. One box with Blender on it §1.5 — what the read costs (Q5, Q17, Q18; Section 13 item 22)

**Owned by:** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) §1.5, reached here through `Job files and outputs` R1, which the document cites. Label: **observed in a test**.

**In its own words:** *"**About 3.2 to 3.8 seconds end to end including a fresh connection; the reading itself 0.09 to 0.12 seconds; 15 to 26 KB back.**"* Three runs per version at load average 4.14, with no Blender of the tester's own running.

**Not covered, and it is item 22 entire:** the fixture behind those figures is `Main` at *"frames 10–50 step 2"* with a `frame_count` of 21 — a stepping span of forty-one frames and three view layers. **Nothing has been read at a real film's frame count, on any box.** The figure is also taken on a container shared by three testers on four cores, which the document treats as an upper bound rather than a measurement.

## M3. One box with Blender on it §1.6 — five reads beside a 21-frame render (W4; Section 13 item 1)

**Owned by:** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) §1.6, headed **NOT ANSWERED**; carried into the stage by `Job files and outputs` R17 and `Rented-server runtime` A12 / §16 item 7, both of which the document cites. Label: **observed in a test**, magnitude untested.

**In its own words:** *"Five reads were taken while a 21-frame render ran. **All five succeeded with an empty error map, and the render finished all 21 frames and all 105 files.** In-script time rose from about 0.11 s to 0.24–0.50 s."* And the reason nothing more is claimed: *"On a four-core container shared with two other testers the two cannot be separated. **What stands: a read against a running render completed correctly every time on this container. The magnitude is untested.**"*

**Not covered:** the magnitude, on any box. W1's *shared, below render* is declared against that, and W4 states the one-edit tightening path.

## M4. One box with Blender on it §1.4 — the keyframed layer confirmed by rendering (Q3, V1)

**Owned by:** the same document, §1.4. Label: **observed in a test**, one box, one fixture.

**What it establishes:** the read's per-frame prediction was checked against a real render inside one run — *"Frames 10 to 20 take about twice as long and the break falls exactly where the keyframe says"* — and the destination prediction held exactly: *"It predicted five files per frame and 105 over 21 frames… **105 exactly, with no folder at all for the muted node.**"*

**Not covered:** keyframes only, constant interpolation, one fixture. The driver and stored-script cases were not posed there at all and are this document's test's.

## M5. One box with Blender on it §1.3 — the engine case that could not be posed (E5; Section 13 item 5)

**Owned by:** the same document, §1.3. Label: **not posed**, and it is stated there as a gap.

**In its own words:** *"**Not posed, and a real gap:** a file naming an engine the chosen Blender lacks. Cycles is enabled on all three builds here, including under factory settings, and no Blender lacking it could be produced without installing or removing software on a shared box."*

**This document's own test stands in the same place** — Cycles was enabled on both builds — so the case is unposed across every box this project has driven.

## M6. The rented machine — a killed transfer under the final name (Q13)

**Owned by:** [The rented machine](../The%20rented%20machine.md), through `Facts.md` Fact 55, 2026-08-13. Label: **observed in a test**, one box, one day.

**What it establishes, and it is the ground Q13 refuses size and modification time on:** an 8 GB write in flight at the moment of a restart *"died and did not come back, leaving 4.7 GB **under the file's final name with nothing marking it incomplete** and an empty error stream."* Fact 55's later half adds, against a real render, that *"a partial sits under the frame's real final name with nothing to mark it"* — *"which settles that Blender does not write to a temporary name and rename."*

**Not covered:** one box, one provider, one day.

## M7. Facts.md Fact 21 — the four-value file identity, and what the transfer surface cannot supply (Q13, Q14 item 2)

**Owned by:** [Facts.md](../Facts.md) Fact 21, answered 2026-08-15/16 by the six-machine batch. Label: **observed in a test**.

**In its own words:** *"**Yes, from size, inode, modification time and change time together** — a save-by-rename changes the inode, and a hand-restored modification time still moves the change time, which cannot be set back. Posed live, a byte changed mid-fetch and **the copy returned 0 and delivered a different file**, with only the timestamps revealing it."*

**And the route, which is Q14 item 2's whole constraint:** *"**the file-transfer surface cannot supply two of them.** Revision 3 of the file-transfer protocol… carries flags, size, owner and group, permissions, access and modification times, and extensions. **No inode, no change time, and the modification time is a 32-bit count of seconds.** So the change time… is not on that surface, and the reading is a **command-channel** one — unestablished ground on an arbitrary rental."* The publication behind that revision claim, with its address and date, is at [P3](#p3-opensshs-protocol-file--revision-3-of-the-file-transfer-protocol-q14-item-2).

**Not covered:** whether the command channel answers on an arbitrary rental. `File Browser` §13 item 9 carries it and Q14 item 2 declines to claim where it cannot answer.

## M8. Facts.md Fact 46 and Servers §15 — the card states, the count, and the container (D5, D6, D7; Section 13 item 12)

**Owned by:** [Facts.md](../Facts.md) Fact 46, answered in part 2026-08-15/16 on a two-card box with no Blender, no screen and no display server, and widened 2026-08-22 by [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md); carried into the stage by `Servers` §15 H1, H2, H5, H11, H12 and H16, which the document cites. Label: **observed in a test**; the no-card half **not answered**.

**What Fact 46 establishes:** the count trap — the driver's own count query *"returns the value once per card, so a caller reading the first line is right by luck and one parsing the whole output as an integer gets `22`"*, and *"It passes every test ever written on a one-card machine"*; identity needs the UUID and bus location; *"**Per-card memory has exactly one route and it is `nvidia-smi`**"*. The discrimination: *"A card present with no driver makes `nvidia-smi` simply absent, **byte-identical to what a cardless machine gives**; a driver with no card gives `No devices were found`, exit 6. **So the discrimination is possible only by reading PCI**"*, and *"Anything concluding "no GPU" from `nvidia-smi` failing will misclassify a perfectly good rented A6000."*

**The container half:** `Servers` H12 — the bus scan *"listed the host's own storage controller, its audio device and its network card, none of which belong to the container"*, and on the containers of the multi-provider batch the scan *"does not exist: `lspci` is absent from the image."* So on that kind *"the reading never claims *no card*; it says the cards could not be established."*

**Not covered, and it is item 12 entire:** *"**The no-card half is not answered**"*, and it is **unposable on Vast** — see [P4](#p4-vastais-search-offers-reference--no-machine-kind-filter-d7-section-13-item-12). The multi-card container reading is explicitly still open. **And `Servers` H11 was corrected on 2026-08-22 from three card states to four, with a fifth table row for a machine on which the scan has no route at all**, and again on 2026-08-23 to key that row on the driver's answer as well; H12b names the uncovered case as *"a machine that is not a container and has no `lspci` either"*.

**The document was repaired against that on 2026-08-23.** Its preamble said *three card states rather than two* and now says four over five rows, naming both dates of `Servers`' own correction. **D6's table was short one row** — it carried H12's container case and nothing for H11's fifth row on a machine that is not a container — and a sixth row was added, answering as the container row does and for the same reason: the reading claims neither way, so the page claims neither way. `Servers` is not edited from here; H11 and H12b are read as they stand.

**One place outside this document still carries the stale count and is routed, not touched:** Row 158's own file records the answer as *"three card states rather than two, and a container reading that never claims *no card*"*.

## M9. Facts.md Fact 3 — the device answer is NVIDIA's alone (D2; Section 13 item 20)

**Owned by:** [Facts.md](../Facts.md) Fact 3, answered for the NVIDIA make 2026-08-15/16. Label: **observed in a test**.

**In its own words:** the Cycles device setting *"takes `NONE`, `CUDA`, `OPTIX`, `HIP`, `ONEAPI` and **refuses `METAL` outright**"*; two distinguishable refusals at launch, *"both exit 1 with nothing written"*; and *"**What is not distinguishable is a broken driver from absent hardware**"*. **Still owed:** *"every make other than NVIDIA… AMD, Intel and Apple hardware are unposed and want another rental."*

**Not covered:** anything the document says about the cards side is `Rendering`'s Rows 8 and 9, cited rather than decided here.

## M10. Facts.md Fact 45 — the Blender search's ceiling and its untested placements (B1, B11; Section 13 item 19)

**Owned by:** [Facts.md](../Facts.md) Fact 45, answered in part 2026-08-15/16; carried into the stage by `Server Setup` §12, which the document takes whole. Label: **observed in a test**.

**What it establishes, and it is B1's honesty marks:** *"**A plain unpacked folder is invisible to every route but a whole-disk file search**… and **a renamed binary defeats the file search too**, found only by running each candidate and reading its version output."* The parsing rule B1 inherits: the apt build *"prints two lines of colour-management noise before its version, so a reader taking the first line gets none — every build must be parsed by finding the line beginning `Blender `, never by position."* And the below-floor case, found by accident: *"the apt build is **3.0.1, below the floor**."*

**Not covered, and it is item 19 entire:** *"flatpak was not installed on either box; neither tested a non-root login against a renamed binary; a Blender inside a container, on a network mount, or in a user's home directory is untested."* Plus the ceiling: *"a non-root login found three of four on the other machine and was **refused the fourth outright**."*

## M11. Facts.md Fact 54 — the sort trap (B2)

**Owned by:** [Facts.md](../Facts.md) Fact 54, answered 2026-08-15/16. Label: **observed in a test**.

**In its own words:** *"**a plain alphabetical sort of the release listing picks 4.2.9 as the newest when 4.2.23 exists**: anything reading that index must sort by version, not by string."*

**Not covered:** the fact is about Blender's own release index rather than about a machine's installed builds. B2 applies the same discipline to answered versions parsed as components, which is `Server Setup` B2's, cited.

## M12. Facts.md Fact 13 — the desktop's clock is undefended (Q18)

**Owned by:** [Facts.md](../Facts.md) Fact 13, answered in part 2026-08-16, and carried into the stage by `Desktop app and delivery` T19, which the document cites. Label: **documented**, from each vendor's own documentation; the observed half deliberately unanswered.

**In its own words:** *"**Nothing on any of the three systems was shown to survive both a reboot and an administrator.** Every monotonic reading each vendor documents as immune to the clock resets at boot… **Every cross-reboot record is wall-clock-derived**, and each vendor documents the administrator's power over it."*

**Not covered:** *"the brief forbade changing the Creator's clock, so **no claim about what a wound-back clock does to any reading is observed on any system, this Mac included**."* Q18 leans on the negative alone — that the desktop's clock is undefended — and puts nothing on it that a decision rests on.

## M13. Facts.md Fact 19 — answered, and unused by this document (Q14; Section 13 item 18)

**Owned by:** [Facts.md](../Facts.md) Fact 19, answered for both payloads 2026-08-15/16. Label: **observed in a test**.

**In its own words:** *"**ANSWERED for both payloads… and the watching is free.** A render took 43.64 seconds watched against controls of 43.47 and 43.79, and the watcher cost **0.08 seconds of processor over 55**. A file arriving by transfer is **indistinguishable in event shape** from a frame being written."* And the trap: *"**a watch on a file follows the inode**, so a save-by-rename kills it silently, and a replaced folder gives one ignored event and then nothing."* **Still owed:** *"the render was deliberately run on the processor, so the card case is unposed."*

**Why it is here and not behind a clause:** Q14's noticing carries no watch — the clause was rewritten 2026-08-22 — so this fact stands answered and this document leans on none of it. Section 13 item 18 records the same.

## M14. Servers §15 — the hardware reading's shape, moment and clock (D5, D6)

**Owned by:** `Servers` §15 H1, H2, H5, H16 — a peer answer document, cited by the document as *Row 158's whole supply*. Label: as `Servers` records each, all resting on Fact 46 and the multi-provider batch.

**What D5 and D6 take:** each card's make as a keyable value, its unique identifier and bus location, its total memory, the count read as a count (H5 — *"The count is the number of rows of a per-card query, and nothing else is read as a count"*); the reading's three states and the rule that *"the reading carries the moment it was taken **and the machine's own clock it was taken on**, and no duration is computed by subtracting it from the desktop's clock"* (H2); and that no Blender is in it (H16).

**Not covered by this document at all:** how the reading is taken, when, and on which connection. H1a's re-key — the state turning on the connection rather than the period — is `Servers`', and D6 was corrected to it on 2026-08-22.

## M15. Rendering's companion, P4 — the two autoexec switches, with an address (V3; Section 13 item 9)

**Owned by:** [Rendering sources](Rendering.sources.md) P4, which carries the manual's exact words, the address and the date read. This is a pointer to a peer answer document's companion; the citation is owned there and is not re-taken here. The words and address are repeated at [P2](#p2-blenders-command-line-arguments-page--the-two-automatic-script-execution-switches-v3-section-13-item-9) because this document's item 9 quotes the switch by name.

**Not covered:** the pages read are the **5.1** manual's, and neither document makes a per-version claim on the two switches.

---

# Part 4 — Outside publications quoted directly

Every entry in this part is **shape 2**: the exact words, the address, and the date read, taken from this document or from elsewhere in this project. **No address here was looked up for this file.**

## P1. Blender's own downloads — the two archives this document's test drove (Part 2)

**Source:** `download.blender.org/release/Blender4.5/blender-4.5.12-linux-x64.tar.xz` and `download.blender.org/release/Blender5.2/blender-5.2.0-linux-x64.tar.xz`. **Read 2026-08-19**, onto the rented container. Label: **observed in a test**.

**What is recorded of them:** 4.5.12 LTS, hash `84afd5f785f7`, built 2026-07-21; 5.2.0 LTS, hash `fbe6228777e7`, built 2026-07-14 — each read from the line beginning `Blender ` on the unpacked build itself, not from the page.

**Not covered:** nothing on those pages was read as a claim. **5.0 and 5.1 were not fetched.** That a rental can reach Blender's own downloads at all is `Facts.md` Fact 54's and is `Server Setup`'s, not this document's.

## P2. Blender's command-line arguments page — the two automatic-script-execution switches (V3; Section 13 item 9)

**Source:** the Blender manual's command-line arguments page and its scripting-security page. **Read 2026-07-30.** Label: **documented**.

**Exact words, as this project records them:** *"`-y, --enable-autoexec` — Enable automatic Python script execution."* and *"`-Y, --disable-autoexec` — Disable automatic Python script execution (Python-drivers & startup scripts), (default)."*

**Where the fact lives, and it is a comparison question:** ***Settings 11***, which carries the pair in all four versions — *"a preference called Auto Run Python Scripts … which two launch arguments override — one to enable, one to disable"* — and *"Every version's manual says automatic execution is off by default"*; *The machine 7* carries the same pair. The pages quoted are `https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html` and `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html`, both read **2026-07-30**. The entry is held by [Rendering sources](Rendering.sources.md) P4. *(This entry pointed only at two 5.1 findings documents until 2026-08-30. `Findings/` sits outside every downstream reading at every stage, per the [Blender Research method](../../Blender%20Research/Method.md).)*

**What it carries here:** V3's *"both switches are documented"* — the sentence L9's 2026-08-21 correction rests on — and item 9's *"Blender documents `-Y, --disable-autoexec`"*, the lever the document names and declines to pull. **Narrower than it reads, and *Settings 11* says so in place:** the `(default)` printed beside the disabling switch *"describes the factory state of that preference, not an unconditional rule"* — with neither switch given, the preference decides. The lever is still there and is still not pulled; nothing this document decides moves.

**Not covered:** the pages are the **5.1** manual's. Nothing here is a version comparison.

## P3. OpenSSH's `PROTOCOL` file — revision 3 of the file-transfer protocol (Q14 item 2)

**Source:** OpenSSH's own `PROTOCOL` file — *"Note that OpenSSH's sftp and sftp-server implement revision 3 of the SSH filexfer protocol described in:"*. Address `https://raw.githubusercontent.com/openssh/openssh-portable/master/PROTOCOL`, **read 2026-08-23**. Label: **documented**.

**Where in this project the quotation sits, with its address:** [Facts.md](../Facts.md) Fact 21's addendum of 2026-08-21, the address and reading date supplied there on 2026-08-23; and `Dependency Upload & Remap` Q5 route 1 and `File Browser` §13 item 9, which Q14 item 2 cites.

**What it carries here:** the attribute structure named in Q14 item 2 — no inode and no change time — which is why the page's comparison is a command-channel reading and why Q14 declines to claim where that channel cannot answer.

**Not covered, and Fact 21 records the correction:** the words *"and the one this project's sessions negotiate"* were struck on 2026-08-23. What stands in their place is protocol version 3 announced by every rental posed — **four boxes across three sellers** — which is an observation and not a claim about every rental.

## P4. Vast.ai's search-offers reference — no machine-kind filter (D7; Section 13 item 12)

**Source:** Vast.ai's own CLI reference for searching offers — `https://docs.vast.ai/cli/reference/search-offers.md`, **read 2026-08-20**. Label: **documented**.

**What this project records of it:** it *"lists `cpu_arch`, `cpu_cores`, `cpu_ghz`, `cpu_cores_effective`, `cpu_ram` and `has_avx` — the processor on a GPU machine — and no field naming a machine kind."*

**Where in this project it sits:** [Facts.md](../Facts.md) Fact 46, which uses it to establish that the no-card half *"cannot be posed on Vast.ai at all"* — a conclusion drawn from the missing filter rather than from two refused rentals.

**Not covered, and Fact 46 states it as the correction it is:** this establishes what can be **asked for**, not what the provider sells. One unrun search — `num_gpus=0` with `cpu_ram` above zero — would settle whether the marketplace lists such an offer at all.

---

# Part 5 — Assumptions, and claims with no source in this project

Every entry in this part is **shape 3**. Each says what breaks and what was searched. **Where an entry says no source was found, the search covered `Preparation/Blender Research/Across topics`, `Preparation/Blender Research/Comparison`, this stage's frozen input documents, `Preparation/Architecture/Facts.md`, the peer answer documents' own companions in this folder, and the Specs' companions in `Preparation/Specs/Sources`.**

## U1. Which file shapes emit no start sentinel (Q8 ending 2)

**Label: bounded observation, generalised no further, and the document says so in place.** [Part 2](#part-2--the-controlled-test-of-2026-08-19) item 6 poses **two** file shapes — a corrupt file and an absent path. **Whether any other shape ends the run before the handed-in reader** — a file Blender can begin to open that aborts first — **is posed by nothing in this project.** No comparison asks it; *Settings 10* poses a newer-version file, which opens and renders.

**Breaks:** the ending's **label alone**. Nothing is stored either way (Q7), submitting stays offered (Q9), and the version named is Smoozy's own record rather than a diagnosis of the file.

**Narrowed 2026-10-02 ([B30](#b30-a-reader-that-crashes-blender-after-the-file-opens-and-the-files-that-never-reach-the-reader-q8)).** A file cut short is a third shape that never reaches the reader. A file Blender opens whose reading crashed Blender was a shape that emitted no start sentinel while the sentinel was written at the reader's end; Q8 now has the reader write it first, so that shape is ending 3. What is still posed by nothing is a file that aborts between opening and the reader.

## U2. Whether the command channel answers the four values on an arbitrary rental (Q14 item 2)

**Label: unresolved, and the document carries it.** [M7](#m7-factsmd-fact-21--the-four-value-file-identity-and-what-the-transfer-surface-cannot-supply-q13-q14-item-2) establishes that the change time is not on the transfer surface. That the command channel supplies it on any given rental is **unestablished ground** — `File Browser` §13 item 9's own words.

**Breaks:** on a rental where it cannot answer, Q14 item 2's comparison does not run and the page's display goes stale until item 1 or item 3 fires. **What holds:** Q14 claims nothing where the channel cannot answer, and R12's attempt-start check guards every render.

## U3. Blender 5.0 and 5.1 (Section 13 item 3)

**Label: assumption, inferred from the research's own version boundary.** No source in this project observes 5.0 or 5.1 for anything [Part 2](#part-2--the-controlled-test-of-2026-08-19) established: the severing route, the driver evaluation, the raise-does-not-stop behaviour. 5.1 was not installed in the 2026-08-17 batch either ([M1](#m1-one-box-with-blender-on-it-13--fact-5s-second-payload-in-one-read-q1-q2)).

**Breaks:** a version-specific difference inside the from-5.0 branch nobody has looked for. **What holds:** the boundary itself is the research's, confirmed in all four versions for the curve route ([B8](#b8-the-route-to-the-animation-curve-per-version-q2-item-2-v6-v9)), the imposition identifiers ([B13](#b13-a-sample-count-is-a-ceiling-and-the-five-that-stand-between-c1-c2-c5-section-13-item-10), [B14](#b14-the-layer-samples-rule--its-three-answers-and-its-starting-value-q2-item-1-c1-item-4-c6)) and the window ([B1](#b1-the-one-window-both-edges-silent-and-the-argument-name-taken-for-a-file-y3-y4-v4-c1-c7)).

## U4. The test's own bounds (Section 13 items 4 and 17)

**Label: bounded observation.** One rented container, one day, one provider, one fixture family, 64×64 at 16 samples on the processor, one driver expression, one handler shape. [Part 2](#part-2--the-controlled-test-of-2026-08-19)'s *What this test does not cover* is the whole list.

**Breaks:** any clause resting on the test, in the shape its bound names. **What holds:** each finding is used with its bound stated where it is used, and the document's own reading rule — *this is what that box did on that day*.

## U5. What form the one add-on announcement arrives in (E5; Section 13 item 5)

**Label: unresolved. No source found.** [B19](#b19-the-one-add-on-case-a-file-announces-and-the-45-only-trace-e5-row-154) gives the case at a window and the printed line to a terminal; **no source in this project gives a readable field for it**, and the one observation of the case points away from a raise at the open — the read completed and reported EEVEE as the file's engine (Specs' [Render Settings sources](../../Specs/Sources/Render%20Settings.sources.md), *A file's engine does not necessarily survive the render*). The printed-line shape is the one Q8's rule from `Rented-server runtime` §11.4 forbids. **The case has been posed on no box**: Cycles was enabled on every build of the 2026-08-17 batch ([M5](#m5-one-box-with-blender-on-it-13--the-engine-case-that-could-not-be-posed-e5-section-13-item-5)) and of this document's test.

**Breaks:** the announcement arriving in a shape the reader does not record, so `Server Setup`'s offer never fires and the job renders the fallback at success. **What holds:** `engine_as_read`, observed; the fallback's shape, observed by the research ([B18](#b18-what-a-render-does-when-the-engines-add-on-is-not-there-e2-e3-e5-section-13-item-5)); and E2's anchor refusal for the user-picked half.

## U6. Behaviour of the launch-script lever at a real job's size (Y6; Section 13 item 6)

**Label: unresolved. No source found.** Every demonstration in this project is small — [B3](#b3-the-launch-script-levers-demonstrated-size-y6-section-13-item-6)'s 160×120, this document's test's 64×64.

**Breaks:** an imposed value behaving differently at a real size — unobserved in either direction. **What holds:** the same lever the redirect and the remap already ride, whose failure would be theirs too.

## U7. Driver and stored-script shapes beyond the two posed (V2, V3, V8; Section 13 items 7 and 9)

**Label: unresolved, on the shapes named.** A **Python-function driver** — which *Reading a file 3* records the 5.x documents as saying needs the automatic-running permission — was posed by nobody; only one handler shape was; and both answers rest on the read and the render sharing one evaluation machinery under one preference, which is structural but was watched on one box. **A pinned layer whose switch a stored script redrives, on a machine whose preferences allow stored scripts, is posed nowhere at all** — the severing removes curves and drivers and does not reach a handler.

**Breaks:** a driver or script shape where the read's stepping and the render diverge — the boxes and the plan total wrong for that file, silently; and, for the pin, a job that renders as the script says at success. **What holds:** the preference starts off in every version ([B12](#b12-the-stored-scripts-preference-and-the-two-launch-switches-v3-section-13-item-9)), the launch enables it in neither direction, and the case needs the user to have altered their own machine and shipped such a file.

## U8. The curve outside the read's stepped table (Q3, V12; Section 13 item 8)

**Label: partly observed, partly unposed.** After-last-key extrapolation was observed ([Part 2](#part-2--the-controlled-test-of-2026-08-19) item 9). **Before-first-key extrapolation was not posed**, and **no interpolation kind other than constant and linear was posed**, by this test or by any comparison — *Reading a file 3* poses keyframes and does not enumerate interpolation kinds.

**Breaks:** a box drawn from client-side curve arithmetic for an exotic-interpolation switch at a frame outside the stepped span — a display fault, never a render, since the render evaluates its own frames. **What holds:** Q3's span covers the file's own range and every key; the constant-interpolation case is exact; the label distinguishes computed from evaluated.

## U9. A driven switch outside the scene's own frame range (Q3, V12; Section 13 item 7)

**Label: structural, not observed.** A driven switch has no keys, so Q3's span for it is the scene's own frame range alone. **No source in this project poses a driver evaluated outside a read's stepped span**, and the document answers such a frame *not known without a read* rather than by arithmetic.

**Breaks:** a box that says nothing is known until the user reads the file, for a typed range outside the scene's own — a range the spec permits without restriction. **What holds:** the honesty of the label, and that no box ever states another frame's answer as this frame's.

## U10. The displacer set, the combining rule, the card side, and EEVEE (C2, C3, C5; Section 13 item 10)

**Label: three separate gaps, all unresolved.**

- **The list is open by the spec's own statement**, and the combining rule is established nowhere — [B13](#b13-a-sample-count-is-a-ceiling-and-the-five-that-stand-between-c1-c2-c5-section-13-item-10) carries *Settings 16*'s reason: the part of Cycles that runs a render session is absent from every version's file snapshot. **Breaks:** a displacer not in C1's set costs a render at a count other than typed, silently. **What holds:** C2's extension point — one more pair, nothing else moving.
- **Whether the scene-level layer-samples rule governs EEVEE from 5.0 is answered nowhere.** *Settings 16*'s table is Cycles'; its own not-established list carries *"whether the Layer Samples control is shown only when an override exists, and whether the setting still bites when it is hidden"*. **Breaks:** Row 150's promise under EEVEE on 5.0 and later — a second layer at its own count, silently, at exit 0. **What holds:** the count pair, which every version takes, and the whole set under Cycles.
- **C5's *every Cycles run behind the displacer evidence was on the processor* was struck on 2026-08-23, and the clause was rebuilt on what the comparison records.** *Settings 16* was re-read that day: it names its two machines — the Creator's Mac and *"the rented Linux computer with an RTX 4060 Ti and no display server"* — and the reading concluded it **named the device of no displacer run**. **That reading was wrong, and it is corrected here, 2026-08-30, by the downstream citation audit.** The comparison's own machine conditions state the device outright: *"Every Cycles render behind question 16 was on the processor, not on the graphics card"*, in every version on both machines, so that *"nothing under question 16 says anything about a render on a graphics card"* — which is what the struck sentence had said. **The conclusion never moved and does not move now**: nothing speaks for the imposition set on a graphics card, and that negative is what C5 is used for and what [Section 13](../Render%20Settings.md#13-unresolved) item 10 carries, beside the comparison's own not-established entry, *"whether the Time Limit is counted the same way on a graphics card"*. What changes is that the negative rests on a stated device again rather than on an absent one. **Breaks:** nothing. C5 and item 10 in [Render Settings](../Render%20Settings.md) were repaired in the same sitting; `Rows/Render Settings.md` had carried the original sentence correctly throughout.

## U11. Whether saved preferences pull a processor-set scene onto the cards (D4; Section 13 item 11)

**Label: unresolved. No source found.** Every observed silent redirection in this project ran the other way — a cards-asking scene rendering on the processor on a machine left as it comes ([B24](#b24-which-card-renders-and-where-a-device-request-comes-from-section-81s-constraint-d2-d3-d4)). **No source poses the reverse.**

**Breaks:** a user's processor choice costing a cards render — mitigated for the override case by D3's pair, unmitigated for the file's-own case, where Smoozy imposes nothing by design. **What holds:** the warning promises no outcome.

## U12. Fact 46's no-card and multi-card halves (D7; Section 13 item 12)

**Label: not answered, and unposable on the provider tested.** [M8](#m8-factsmd-fact-46-and-servers-15--the-card-states-the-count-and-the-container-d5-d6-d7-section-13-item-12) and [P4](#p4-vastais-search-offers-reference--no-machine-kind-filter-d7-section-13-item-12). The genuinely-cardless reading is argued from the bus scan and has never been observed on a real cardless box; the multi-card container reading is explicitly still open.

**Breaks:** Row 158's warning on exactly those boxes — shown from a reading that lies, or withheld where cards stand. **What holds:** the per-state table, which never claims what the reading cannot, and the container's could-not-establish row.

## U13. Break notes for lines above the shipped ceiling (B9; Section 13 item 16)

**Label: assumption about the future, and it is not a fact to be sourced.** [B25](#b25-each-versions-own-recorded-breaks-b7-b8-section-13-item-16) is a four-version comparison; a build newer than anything the shipped application knows has no notes anywhere, by construction.

**Breaks:** nothing — B9 composes a true statement. Recorded so nobody reads the warning as complete for all time. **Closed 2026-10-02 with item 16: no notes are shown.**

## U14. Fact 45's untested placements and its non-root ceiling (B1; Section 13 item 19)

**Label: unresolved, carried from `Server Setup` and not re-posed.** [M10](#m10-factsmd-fact-45--the-blender-searchs-ceiling-and-its-untested-placements-b1-b11-section-13-item-19).

**Breaks:** a build the search cannot see is a build the pick cannot offer — a version match made over an incomplete list. **What holds:** ~~B11's honesty marks travel with the list.~~ **From 2026-10-02 they are kept in the answer and the diary and not drawn (`Server Setup` B11, Creator-stated)**, so on a non-root login the page may offer an incomplete list without saying so — the cost the Creator's ruling accepts.

## U15. Every card make other than NVIDIA (D2; Section 13 item 20)

**Label: unposed.** [M9](#m9-factsmd-fact-3--the-device-answer-is-nvidias-alone-d2-section-13-item-20) and [B24](#b24-which-card-renders-and-where-a-device-request-comes-from-section-81s-constraint-d2-d3-d4)'s *not covered*. AMD, Intel and Apple hardware are unposed in every version.

**Breaks:** the cards side of D2 on another make — a failed attempt naming the make, by `Rendering`'s own rule, never a wrong render. **What holds:** the processor side (D3), which names no technology.

## U16. D3's route to the device (Section 13 item 21)

**Label: unobserved, and the clause is labelled rather than changed.** [B24](#b24-which-card-renders-and-where-a-device-request-comes-from-section-81s-constraint-d2-d3-d4) establishes three routes to the device setting and **assigning it from a script handed in at launch is not among them**. This document's test poses no device at all — its whole device record is the processor.

**Breaks:** Row 157's first half, silently — the user's processor override ignored, the scene rendering on the cards, exit 0 with nothing said. **What holds:** nothing observed. **The posed shape, for whoever runs it:** assign the scene device from a launch script inside the window, render, and read back which device ran.

## U17. Q3's stepping at a real frame count (Section 13 item 22)

**Label: unresolved. No source found.** [M2](#m2-one-box-with-blender-on-it-15--what-the-read-costs-q5-q17-q18-section-13-item-22) prices the read on fixtures of ten and forty-one frames. **Nothing in this project reads a file at a real film's frame count, on any box.**

**Breaks:** Q5's *not long enough to need a second mechanism* and R16's *not a reason to skip it*, both priced against a figure not shown to scale. What gets slower is a read, never a render's correctness. **What holds:** the curve route Q3 already carries as the fallback.

**Narrowed 2026-10-03.** A scene with nothing on it that changes from frame to frame is no longer stepped, so its range costs nothing — observed in a test, [B31](#b31-a-scene-with-nothing-on-it-that-changes-from-frame-to-frame-reads-the-same-at-every-frame-without-being-stepped-q3-section-13-item-22). **Still unresolved: a scene that really is animated over a long range**, which is stepped at every frame and has not been read at a real frame count on any box.

## U18. The resolution percentage pair's spelling (C7, Y4; Section 13 item 24)

**CLOSED 2026-08-29. The lookup was registered blind as *Settings 17* and answered by all four versions the same day; the citation is [B28](#b28-the-names-the-launch-time-pairs-use-the-near-misses-and-the-group-that-swallows-a-wrong-name-c7-c9-v4-y4-rendering-l8-l10a-l11-section-13-items-24-and-28) and the name is `resolution_percentage` on the scene's render settings. What replaces this entry is a bound rather than a gap — [U24](#u24-every-name-in-b28-was-set-on-one-mac-and-the-check-written-from-it-was-run-nowhere-c7-c9-rendering-l10a-section-13-item-28) — and this entry's *Breaks* sentence is corrected at [Section 13](../Render%20Settings.md#13-unresolved) item 24's closure, being right for three versions and one settings group short in the fourth. The original entry follows, unchanged.**

**Label: unresolved. No source found.** *Settings 1* establishes that the setting has no argument in any version ([B2](#b2-the-three-settings-with-no-argument-of-their-own-q2a-c1-c7-v4-section-4s-constraint)); *Settings 14* establishes where it lives and the range it takes ([B23](#b23-the-resolution-percentage--its-range-where-it-lives-and-what-it-does-out-of-range-q2a-j11-j12-c7-y1-section-13-items-24-25-and-26)). **No comparison this document cites carries the exact per-version spelling the pair needs, and this document's test posed no percentage at all**, so [Part 2](#part-2--the-controlled-test-of-2026-08-19) does not carry it either. Y4's sentence — that the pairs' spellings live in the cited comparisons and in this file — does not cover this one entry, and the document says so in place.

**Breaks:** a pair composed from a spelling nobody read raises inside the launch material, and by Section 13 item 13 the render then runs anyway at exit 0, at the file's own percentage, for a user who typed 50. **What holds:** Y4's delivery rule and C7's imposition, both of which stand whatever the spelling turns out to be. **What is owed is one lookup** — in the comparisons, or on any box with a Blender on it.

## U19. Q2a's truncation rule (Section 13 item 25)

**Label: read off four measured pairs, and stated in no documentation anywhere.** [B23](#b23-the-resolution-percentage--its-range-where-it-lives-and-what-it-does-out-of-range-q2a-j11-j12-c7-y1-section-13-items-24-25-and-26) carries the four pairs *Settings 15* measured out of written files, identically in all four versions on both machines. **No version's documentation states the rule at all** — that is *Settings 15*'s own record. Truncation is what those four pairs support and rounding is what they rule out, the discriminating pair being 633.6 → 633 and 10.8 → 10.

**What no pair poses:** a product landing exactly on a whole number, an odd full size, and every percentage other than those four.

**Breaks:** the figure differing by one pixel on an axis from what the render writes — **a display fault and never a wrong render**, because what is imposed is the user's own percentage and Blender does its own arithmetic at render time. **What holds:** the four pairs, and the reading's own carried figure ([M1](#m1-one-box-with-blender-on-it-13--fact-5s-second-payload-in-one-read-q1-q2)'s `pixels`), which is Blender's own answer at the file's own percentage.

## U20. ~~What the sample count setting accepts~~ — **ANSWERED, and moved to [B23a](#b23a-the-sample-counts--what-each-accepts-and-what-blender-does-with-a-number-outside-it-j11-item-3-j12s-two-samples-bullets-c1-c9-section-13-items-26-and-31)** (J11 item 3, J12; Section 13 items 26 and 31)

**Closed 2026-08-31 in this file; the answer itself landed 2026-08-30.** Fact 127 was registered blind as ***Settings 18*** on 2026-08-29 and answered on 2026-08-30, and [Section 13](../Render%20Settings.md#13-unresolved) item 26 closed with it. **This entry went on calling the question unresolved and unsourced after it was answered**, which is the gap this correction closes; B23a is the one home now. **The residue is not nothing** — it is item 31's four bounds, recorded at B23a, and it is narrower than what stood here. **Nothing in this entry's own reasoning was wrong when it was written**, and it is kept below so a reader looking for the old gap finds where it went.

**How this entry read.** *Label: unresolved. No source found, and it is a lookup rather than a gap.* `The map.md` poses no question for it: *Settings 16* answers what displaces a typed count ([B13](#b13-a-sample-count-is-a-ceiling-and-the-five-that-stand-between-c1-c2-c5-section-13-item-10)); *Settings 15* measured the **resolution percentage's** own range and correction ([B23](#b23-the-resolution-percentage--its-range-where-it-lives-and-what-it-does-out-of-range-q2a-j11-j12-c7-y1-section-13-items-24-25-and-26)); and [What can be set from outside](../../Blender%20Research/Across%20topics/What%20can%20be%20set%20from%20outside.md)'s general sentence — *"a number outside a setting's range is corrected without a word and the render goes ahead at the corrected size"* — cites *Settings 15* and rests on that one setting's evidence. Carrying it to a second setting is the assembling [Method](../Method.md) forbids.

*Breaks: two things. The design sentence's second half for that box — the line states the range and claims nothing about Blender. And, because J11 imposes no range there rather than inventing one, a typed count outside whatever Blender's setting accepts is composed into C1's count pair and may be silently corrected at assignment, rendering at a count nobody asked for and reporting success. What holds: the count pair and the whole imposition set for every count inside the range, and the box's whole-number gate. One number would close both halves.*

**Both halves are closed, by different clauses, and the break it named was exactly right:** the silent correction is what *Settings 18* observed, and J11 item 3 now imposes the picked engine's own range at the box while J12 states what Blender would otherwise have done. And it was not one number — **there is no one sample-count setting**, each declares its own range, and the two engines' ceilings differ by nearly two orders of magnitude.

## U21. Where on the launch line `--python-exit-code` is honoured from (V7; `Rendering` §9 item 19)

**Label: assumption. No source found.** [Part 2](#part-2--the-controlled-test-of-2026-08-19) item 5's observation is *"With `--python-exit-code 63` on the line"* and says nothing about position; the position was not varied. Nothing in `Preparation/` states where the switch is honoured from. The seam was routed and closed in `Rendering` L8a, which places it at the head of the line; that placement's own residue is [Rendering sources](Rendering.sources.md) U2 and is not this document's.

**Breaks:** if the switch is honoured only from a position the contract does not use, every fail-loud conduct this document specifies is silent again — a wrong render at exit 0. **What holds:** the observed positive itself.

## U22. How many Blenders one machine will carry at once (W1, W4; Section 13 item 2)

**Label: not established, and no test can reach it.** [B6](#b6-a-read-beside-a-running-render-and-how-many-blenders-one-machine-carries-w4-section-13-items-1-and-2) — *Reading a file 2* records it as not in the sources in all four versions and as a test that cannot be run, *"Whatever number of Blenders a computer would start is a fact about that computer."*

**Breaks:** a machine slowed by parallel readings nobody bounded. **What holds:** the single admission point, W1's shared class, and W4's one-edit path to a stated bound.

## U23. A file naming an engine the reading Blender lacks, posed on no box (Section 13 item 5)

**Label: no longer unposed. Rewritten 2026-08-31, out of the repaired Blender research.** **The *unposed, twice* label and the *Breaks* line below both rested on a sentence that is gone** — *Settings 13*'s *anything involving a third-party engine is untested in all four versions everywhere*. ***Add-ons 7* is the run.** **Where the engine is named at launch and its add-on is absent, no image is written; 5.0, 5.1 and 5.2 end at 1, and 4.5 does not end at all** — killed on the Mac at status 134, and never exiting on its own on a rented Linux machine. Label: **observed in a test**. See [B18](#b18-what-a-render-does-when-the-engines-add-on-is-not-there-e2-e3-e5-section-13-item-5), where the same repair is recorded against the clause that carried the old sentence.

**What this entry used to say, kept so the old gap can be traced.** *Unposed, twice. Distinct from [U5](#u5-what-form-the-one-add-on-announcement-arrives-in-e5-section-13-item-5), which is about the announcement's shape: this is the run itself. [M5](#m5-one-box-with-blender-on-it-13--the-engine-case-that-could-not-be-posed-e5-section-13-item-5) records that no such Blender could be produced on a shared box, and [Part 2](#part-2--the-controlled-test-of-2026-08-19) stands in the same place. Breaks: E5's mechanism has documented ground and no run behind it.*

**Breaks now:** **E5's fallback shape**. E5 rests on the *Cycles switched off* observation, where the render fell back to EEVEE and exited 0; the engine-named-at-launch case has **no fallback and no image**, and on 4.5 no ending at all — which is what E5 and [Section 13](../Render%20Settings.md#13-unresolved) item 5 must now be written against, and this file decides nothing about how. **What holds:** `engine_as_read`, observed on three versions ([M1](#m1-one-box-with-blender-on-it-13--fact-5s-second-payload-in-one-read-q1-q2)); E2's anchor refusal; and [U5](#u5-what-form-the-one-add-on-announcement-arrives-in-e5-section-13-item-5), the announcement's shape to a run with no window, which is untouched and still open.

## U24. Every name in B28 was set on one Mac, and the check written from it was run nowhere (C7, C9; `Rendering` L10a; Section 13 item 28)

**Label: bounded observation, and one inference on top of it.** [B28](#b28-the-names-the-launch-time-pairs-use-the-near-misses-and-the-group-that-swallows-a-wrong-name-c7-c9-v4-y4-rendering-l8-l10a-l11-section-13-items-24-and-28) is sixty runs and sixty repeats in four versions on **the Creator's Mac, macOS on Apple silicon, nothing rented** — and **no Linux machine tried any of these names in any version**, while every machine Smoozy renders on is rented Linux. The comparison names that as its own largest gap. **The inference is `Rendering` L10a's**: the check reads a pair's name back against the object it was assigned on, and what the comparison observed is that answer *after* a swallowed assignment, never the check itself and never before an assignment.

**Breaks:** a name spelled differently on Linux is a pair that applies to nothing — loud in every group and version but 4.5's Cycles, and in that one corner the name check is the only thing standing, itself untried anywhere. **What holds:** the names are read out of each version's own running program and off each version's own documentation page for all but Cycles' two; nothing in the document is composed from a label, a panel or a manual wording; and one run at Build settles the check, on a material the launch contract already composes.

## B32. EEVEE's time limit, from 5.3 alone (C1 item 3, C3, C10) — added 2026-11-02

**Label: documented for the setting (*Settings 2*, *Settings 3*, *Settings 16*), observed for its name and its off value.** The name, its kind and its off value were read at the desk on 5.3.0 Beta `5d8151335b6b` and 5.4.0 Alpha `3f4216e1bad1`, and are absent on 5.2.3 `e13b728c6880`: `Audits/Build/Blender 5.3/Piece 1, the desk readings — what landed.md`, reading 1.

**What Smoozy's own composition does with it, observed the same day** (`Audits/Build/Blender 5.3/Piece 4, EEVEE's time limit and templates in a bake folder — what landed.md`; the test is `a_typed_count_under_eevee_outlasts_the_time_limit_the_file_was_saved_with`). An EEVEE file at 400 by 400, saved with Render Samples 4,096 and, where the version has one, a one-second limit; 600 typed; frame 1; the processor's Mac, no window.

| | The count alone, in plain Blender | Through Smoozy's job |
| --- | --- | --- |
| 5.2.3 (no limit there) | 600 of 600 | 600 of 600, ended at 0 |
| 5.3.0 Beta | 125 of 600 printed, about 2 seconds | 600 of 600, ended at 0 |
| 5.4.0 Alpha | 1 of 600 printed, about 2 seconds | 600 of 600, ended at 0 |

**Not covered:** Linux, a graphics card, more than one view layer. C5 stands.
