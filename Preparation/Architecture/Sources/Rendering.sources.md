# Sources — Rendering (Architecture)

*Where the outside-world facts [Rendering](../Rendering.md) labels in place came from. The document carries each claim's label beside the claim; this file carries the citation, the exact words where an outside publication is quoted, the date read, and what each source does **not** cover. **A decision Smoozy made gets no entry here.** Where the citation is owned by one of this stage's frozen input documents, or by a peer answer document, the entry is a pointer to that document and not a second citation.*

***Three publications were read on 2026-08-23 and their addresses recorded here for the first time*** — the two Open Group entries at [P1](#p1-the-open-group-base-specifications-issue-8--standard-stream-buffering-l7s-named-property) and [P2](#p2-the-open-groups-specification-for-test--the-three-exit-statuses-a7-a7a) and NVIDIA's component list at [P3](#p3-nvidias-installed-components-list-for-the-linux-display-driver--libnvoptixso1-l23). Each carried a quotation and a date in the document with no address under `Preparation/`. **In all three the publication's words match what the document carried**, with one difference of form recorded at P1. Everything else here is taken from the document itself or from elsewhere in this project, and where the project holds no source the entry says so and names the folders searched.

**Blender facts are not re-derived here.** Each Blender entry names the joined answer or the comparison question the document itself already names, and nothing further. No Blender answer in this file is assembled out of several topics.

---

# Part 1 — Blender facts: pointers to the joined answers and comparison questions

Every entry in this part is **shape 1**. The document fixes the lookup at *What binds this before anything is decided*: the joined answers `Running Blender with nobody there`, `What can be set from outside`, `What a render really did`, `When a render ends`, `While a render runs`, `Where a render's files go` and `What has to travel with a file` in [Preparation/Blender Research/Across topics](../../Blender%20Research/Across%20topics), the comparison questions those name, and the row files' quotations of them. The comparison questions live in [Preparation/Blender Research/Comparison](../../Blender%20Research/Comparison).

## B1. The command-line route, and the no-window mode (L1, L2, L3 — Row 11)

**Owned by:** *Reports 9*, *The machine 7*, *The machine 1*, *Settings 6*; the joined answers `Running Blender with nobody there` and `While a render runs`. Label: **documented and observed in a test**, as those comparisons record it.

**Quoted in the document from the joined answer:** *"Setting a value from a script is not the same as starting the render from one"* ([What can be set from outside](../../Blender%20Research/Across%20topics/What%20can%20be%20set%20from%20outside.md), from *Reports 9*). L2 carries no further claim.

**Not covered by the comparisons themselves:** *Reports 9* records that **no version's sources set out, way by way, what is reported** — the five-ways answer is the research's own test on both machines rather than a documented statement, and 5.2's page carries a different sentence from the other three. The observed launch form on a real box, `blender -b file.blend …`, is the frozen input document's and is entered at [M1](#m1-the-rented-machine--the-detached-render-and-the-undetached-one-l6-k12-row-19) rather than here.

## B2. The two streams, and the line that names what did the work (L7, L26, L27, L28; Section 9 items 5 and 6)

**Owned by:** *What actually ran 1*, *What actually ran 5*, *What actually ran 6*, *Reports 2*; the joined answer `What a render really did`. Label: **observed in a test** for the stream split and the closing-report timing; **documented** for the absence of any device record in Blender's own metadata.

**What the comparison carries, in its own words:** the device line is reached in 4.5 only through `--debug-cycles`, landing on the **error** stream, and in 5.0, 5.1 and 5.2 through `--log-level info` on **standard output** (*What actually ran 1*); it *"is a closing report, printed after the last sample and before the frame is written. A render stopped part-way never reaches it."* (*What actually ran 6*).

**That second quotation is dead. Corrected 2026-08-31, out of the repaired Blender research.** The comparisons and joined answers were repaired on 2026-08-30, after the citation check that had ruled every downstream citation sound, and *What actually ran 6* **no longer carries the sentence quoted above**. It now records that **after a stop, 5.0 and 5.1 print the whole closing block, 5.2 prints a cancelled line and no block, and 4.5 prints nothing** — and that **5.2's own published fix list calls that block on exit fake, with wrong numbers**. Label: **observed in a test**, with that version's own publication behind the last part. **So a device line observed on a run that did not finish is not evidence of anything.** [L26](../Rendering.md) rests on the withdrawn sentence and is where the repair is owed; `Queue & History` E12 and `Server Setup` W4 and W3a were repaired on this evidence on 2026-08-31 and gate the reading on the job's **recorded outcome** rather than on the line's presence. **What breaks:** any conduct that treats the line's presence as proof the run finished — in `Server Setup` that could fire a setup fault and restart a machine. **The first quotation, the per-version route, is untouched.**

**Not covered:** *What actually ran 1* is an **NVIDIA-on-Linux** answer — Cycles only. Nothing there establishes that an EEVEE or Workbench job names any device at all, which is the document's [Section 9](../Rendering.md#9-unresolved) item 6 and is entered at [U9](#u9-whether-an-eevee-or-workbench-job-names-any-device-section-9-item-6). What else 4.5's debugging switch puts on the complaints stream is not in the comparison either — [U8](#u8-what-else-45s-debugging-switch-prints-l27-section-9-item-5).

## B3. The one window, and the order of the line (L8)

**Owned by:** *Writing 1*, *What a render demands 3*, *Settings 16*, *Dependencies 17*, *Settings 1*; and for the device argument after the bare double dash, *Settings 9* and *The machine 5*. Label: **documented** for the argument set and the double-dash rule, **observed in a test** for the silent-ignore outside the window.

**The thread-count exemption** L8 names and declines to use is *What a render demands 3*'s.

**Not covered:** the exact spelling of each argument is the comparison questions' and is not restated in the document or here. *Settings 9* records the negative that fixes the double-dash rule — without it Blender *"rendered and saved the frame it was asked for, then reached the stray argument, printed `unknown argument, loading as file: --cycles-device`"* and ended at exit 1 — observed on the one-card Linux machine in all four versions and nowhere else.

**A third position on the line is now settled, and it is the one [L8](../Rendering.md) item 9 puts the device argument in. Added 2026-08-31, out of the repaired Blender research.** [What can be set from outside](../../Blender%20Research/Across%20topics/What%20can%20be%20set%20from%20outside.md) now sets out **three** positions rather than two: **after a bare double dash the line is Cycles' own, and an argument Cycles does not know is swallowed without a word while the render goes ahead and ends at 0.** Label: **observed in a test**. **What breaks:** L8 item 9 places `--cycles-device` exactly there, and this document carries **no name check covering that position** — [L10a](../Rendering.md)'s check covers Python settings pairs only. **A mistyped device argument therefore renders the whole job on the processor and reports success**, which is precisely what Row 9 exists to prevent, and the *Settings 9* negative above does not reach it because that argument sat outside the double dash and was loud. `Job files and outputs` D11's stated window needs the same third edge — after the file, before the render instruction, **and before any bare double dash**. **Nothing is decided here:** where the argument goes, and whether a check is added, is the document's.

**A second argument was sitting past that dash and nobody had noticed. Added 2026-09-04, raised by the builder writing the document's rules half at Build step 7.** L8's numbered list ran to ten items, and the tenth was the device-line ask — the switch L26 asks every render to name what did the work with. Numbered after item 9's double dash it sat in exactly the position this entry describes, so **the ask would have been swallowed on every attempt and L26's line would never have been printed anywhere**, silently, with `Queue & History` E12 reading *not known* for every finished job and `Server Setup` W4 concluding nothing about any machine for ever. L8 is corrected and the item is struck out of the list. **What this file has to say about it, and it is the reason the fault survived a stage close:** the ask's own sentence said *placed where that version's evidence shows it taking effect*, and **no comparison and no joined answer records a position on the line for either version's switch.** *What actually ran 1* settles which switch and which stream, and nothing more. So there was no evidence at this stage's bar to check the placement against, in either direction. **The head is chosen on L8a's own footing rather than on an observation**, and the residue is [Section 9](../Rendering.md#9-unresolved) item 28, which one run at Build settles together with item 19's. **Nothing is decided here.**

## B4. The machine's saved preferences, the factory-startup switch, and the stored-scripts default (L9)

**Owned by:** *What a render demands 3*, *The machine 5*, *The machine 7*. Label: **documented** for the preference route and the default, **observed in a test** for the printed decline.

**The two autoexec arguments L9 names are a direct manual quotation** and are entered separately at [P4](#p4-blenders-command-line-arguments-page--the-two-automatic-script-execution-switches-l9).

**Not covered:** which add-ons a job needs, and how they reach the preferences, is `Server Setup`'s and is cited by the document rather than sourced here.

**One thing *The machine 7* now carries that nothing downstream uses. Recorded 2026-08-31, out of the repaired Blender research.** **A run can learn that its own stored scripts were blocked.** Label: **observed in a test**. **It is carried nowhere in this project** — this entry and the Specs' `What a job writes` sources file are the two places it would sit — and **no clause of [L9](../Rendering.md) or anywhere else is written on it**, so nothing breaks and nothing moves. It is recorded here so the next reader knows the research holds it, rather than rediscovering it as new. **Nothing is decided from it here**, and what L9 should do with it, if anything, is the document's.

## B5. The values that have no command-line argument, and Simplify (L10)

**Owned by:** *Settings 1*, *What a render demands 3* for which values have an argument in which version; *What a render demands 2 and 4* for Simplify, reached through the [Rendering spec's sources](../../Specs/Sources/Rendering.sources.md) as the document cites it. Label: **documented** for the argument set.

**The media type is no longer among the values, and this entry claimed it until 2026-08-23** — see [C8](#c8-the-media-type-l10-named-and-no-payload-carries--repaired-by-removing-it). *Images 3* is still cited at L10, for the one thing it is now cited for: that from 5.0 the media type is set before the format and the format setting then takes only the still-image names, which is why a contract that sets no format never meets the ordering. *Reports 11* is no longer cited by that bullet at all.

**Not covered:** Texture Limit and Child Particles are the inferred half — [U11](#u11-simplifys-inferred-pair-section-9-item-11). The per-version redirect property names are the frozen input document's, at [M7](#m7-one-box-with-blender-on-it-1-and-2--fact-5s-payloads-and-the-per-version-property-names-l10-p1-step-4).

## B6. The scene argument, and what a render does when it is not told which scene (L15, L16, S6 — Rows 1 and 5's submission half)

**Owned by:** *Settings 1* and *What a render demands 3* for the argument's presence in all four versions; *Settings 12* for both halves of the behaviour. Label: **documented** for the naming argument; **observed in a test**, identically in all four versions on both machines, for what a wrong name does.

**What the comparison carries:** *Settings 12* is *"Can a render started from outside Blender be pointed at one named scene of a file, and what does it render when it is not told which?"*, and it records the scene-name-that-names-no-scene case as settled by test rather than by documentation — two versions' sources marked the not-told case **inferred** and two marked it **not in the sources**.

**Not covered, and it is the fallback the document builds nothing on:** what a file saved from Blender's own interface records as its showing scene. Every file the research tried was saved by a Blender with no window. [U12](#u12-what-an-interface-saved-file-records-as-its-showing-scene-l16-section-9-item-12).

## B7. The animation form, the naming split, and the frame list (L17, L18)

**Owned by:** *Writing 6* for the naming split — the frame number left out of a single-frame render's name and put into an animation's, and the file-output node's own split by version; *Frames 7* for the explicit list, the malformed list rendering nothing at success, and a frame named twice being rendered twice. Label: **observed in a test**.

**Not covered:** Blender's own skip-existing test is `Job files and outputs` C2's and is cited by the document, not sourced here. Windows is untried for the frame-list forms — [U13](#u13-windows-section-9-item-13).

## B8. The engine argument, and what an unknown engine name does (L19, L20, K15)

**Owned by:** *Settings 13* and *Images 9*. Label: **observed in a test**.

**What the comparison carries, and the document states its thinness in place:** from 5.0 the command line also accepts 4.5's engine name while a script takes only the version's own — resting on **one run, on one machine, on a 64 by 64 scene, with no record of which engine actually rendered**. The 4.5 run that printed its message and never ended is *Settings 13*'s and *What actually ran 2*'s — one run, nine minutes, Linux.

**Not covered:** which engine actually rendered under the cross-version acceptance. The document's answer is to refuse to send anything but the chosen version's own name, which is a decision and earns no entry.

## B9. The device argument (L21, L21a, L22, L24 — Rows 8 and 9)

**Owned by:** *The machine 5* for a named technology taking every card of its kind whatever the preferences said, and for EEVEE rendering on one card with no version supporting more; *Settings 9* for the launch argument beating both the file and the preferences, for the double-dash requirement, and for the quiet processor render when a file asks for a card the machine cannot give — image written, **exit 0**; *The machine 4* for the same quiet case. Label: **observed in a test**, in all four versions on both machines.

**Not covered, and the document records it as honesty:** no per-card form of the argument is established either way — [U4](#u4-a-per-card-form-of-the-device-argument-l21). *Settings 9* also records that what a file's *GPU Compute* means on AMD or Intel hardware is untried in every version, no machine this project has had carrying one.

## B10. What each way of ending a render leaves at the frame's own file (K4 — Rows 16 and 17)

**Owned by:** *Frames 13*, cited through `Job files and outputs` 10.7, which the document takes whole rather than re-deciding. Label: **observed in a test**; *Frames 13* records that **all four versions answer *not in the sources*** and that hand tests settled it.

**Not covered:** the comparison's verdict is that the machinery around the write differs by version — a 5.2-only setting, the multi-layer route, the File Output node's build, and what Blender prints — while what is left where it writes is the same in all four versions on both machines. The document leans only on the second half.

## B11. Anything waiting on a run to finish needs a limit of its own (K9)

**Owned by:** *The machine 7*. Label: **observed in a test**, with the rule itself inferred from it. **Corrected 2026-08-30 at the citation audit; this read *documented*, and nothing in that question documents a rule about waiting.** What the comparison establishes is that a wait on an unattended machine can be unbounded: a script inside Blender reading standard input *"waits for ever"* — still sitting there after sixty seconds in all four versions on both machines, stopped only by a time limit — and it is that question's only thing that waits indefinitely with nobody at the machine. The step from that to *anything waiting on a run to finish needs a limit of its own* is Smoozy's, and the spec states the rule in its own words at Row 16. This is the only Blender ground under K9; the 60-second value is not Blender's and is entered as an assumption at [U1](#u1-k9s-60-second-deadline).

## B12. A successful exit status is not a statement about the outputs (K14)

**Owned by:** the joined answer [When a render ends](../../Blender%20Research/Across%20topics/When%20a%20render%20ends.md). Label: as that answer records it. **Corrected 2026-08-31: this entry quoted that answer's *"Twelve separate endings report success"*, and that sentence no longer exists there.** The comparisons and joined answers were repaired on 2026-08-30 and the figure is **twenty, six of them ending at 0 with an empty output folder** — which is the sharper fact than the count for anything deciding completeness. **K14's decision is unharmed and strengthened:** it reads a successful exit status as no statement about the outputs, and there are now more endings supporting it, not fewer. **The four sources files the runner corrected on 2026-08-31 did not include this one**; it is corrected here.

**Not covered:** the two fixed rules bounding the classification are `Queue & History` E2's and are cited, not sourced.

## B13. The launch-time names, the order rule, and the one settings group that swallows a wrong name (L8, L10, L10a, L11)

**Owned by:** *Settings 17*, registered 2026-08-29 for `Render Settings` §13 item 24 and answered by all four versions the same day; the full citation is that document's companion, [Render Settings sources](Render%20Settings.sources.md) B28, and is not repeated here. Label: as that entry records it — **documented** for the names but Cycles', **observed** for every conduct, on one machine.

**What this document takes from it, and only this.** The per-version spellings its settings pairs are composed from are `Render Settings` C7's and C9's, and they are the same in all four versions. **The window's left edge is silent and plausible**: a pair before the file prints the value asked for back and changes nothing, so L8 item 7's placement carries the whole weight and no read-back can stand in for it. **A wrong name inside the material is refused on the terminal and nowhere else, at exit 0** — which L11 with L8a turns into a failed attempt with nothing written — **except on Cycles' settings in 4.5, where nothing is printed at all**, which is what L10a's name check exists for. **And an unknown launch *switch* runs the other way entirely**, taken for a file name, no picture written, exit 1.

**Not covered:** the check L10a specifies was posed by no run, and every name behind this entry was set on the Creator's Mac alone — [U17](#u17-l10as-name-check-and-every-name-behind-it-section-9-item-25).

## B14. What a saved scene records for Simplify, and which half of each pair a job sets (A1d, L10's Simplify bullet; Section 9 items 11 and 23)

**Added 2026-08-29 at the re-gate, which found this file carrying no entry for A1d at all** — the commit that wrote A1d did not touch this companion, so a reader arriving here to check where the group's file values come from found Simplify only under [B5](#b5-the-values-that-have-no-command-line-argument-and-simplify-l10), which is about the delivery route and settles nothing about what the file records. **No claim in the document was unsupported:** the citations are in A1d itself, in [Facts](../Facts.md) Fact 134 and in `Render Settings` Q2 item 6. What was missing is this file's record of them.

**Owned by:** *What a render demands 6* — the one question of that comparison Row 472 opened, named by the row's file and by **Fact 134** rather than by the map, which poses no Simplify question at all. **Fact 134 is the register's own entry** and names the answering round: four version-isolated passes, audited, hand-tested twice, citation-checked, two Linux machines rented and destroyed. The document's fixed-lookup paragraph names the question as one of the two opened outside the map's set.

**Labels, exactly as the document carries them.** **Documented** for the names, meanings, ranges and spellings of the switch and the three settings, the same in all four versions; **observed** for the texture pair, taken from Cycles' own files; **observed** for the conduct — the round trip through saving, the per-scene rather than per-file holding, the read-back with no window open hand-run in all four versions on two machines, and the crossing between operating systems; **observed** for the hand test in which the render-half limits materially changed the picture in every version, which is what puts A1d's render-half targets under the spec's own *"Three settings are exposed, all from Simplify's render half"*; **documented and observed** for the version claim, 5.2's texture resolution scale being new in that version's own release notes against three versions each answering that the render-side property does not exist; and **observed, tried in 5.2 only** for the defaults trap — an untouched scene does not read back at the documented defaults.

**What this document takes from it, and only this.** That a saved scene records a file value behind all three offered settings, so the group is the page's ordinary two-object composition and no constant of Smoozy's exists anywhere (A1d); which member of each pair a job sets and which is never read; that a documented default is never substituted for a read-back value; and that 5.2's extra pair is left as saved.

**Not covered:** what Blender does with an **out-of-range** assignment to the maximum-subdivision or child-particle settings — the question poses none, and carrying the one silent-correction finding in this stage across from another setting is the assembling [Method](../Method.md) forbids: [U18](#u18-an-out-of-range-assignment-to-simplifys-two-typed-settings-section-9-item-23). The **launch route** for Texture Limit and Child Particles — a value assigned by the launch material rather than saved in the file — stays inferred: [U11](#u11-simplifys-inferred-pair-section-9-item-11). And the round's own stated silences: Windows on either kind of processor, an exact Linux distribution package, and reading these values without Blender at all.

## B15. Which of a card's two rendering methods to name, read against the size of the scene (L23; Section 9 item 3, Section 10's twelfth check) — **added 2026-09-02**

**Why this entry exists.** L23 carried two speed readings that disagreed on direction and said outright that neither settled it. *What a render demands 7* resolves them, and nobody had read it against this clause. **It is inside the document's fixed lookup and no exception is opened for it**: it is a comparison question named by two of the joined answers the document's *What binds this* paragraph fixes — [Running Blender with nobody there](../../Blender%20Research/Across%20topics/Running%20Blender%20with%20nobody%20there.md), whose own list carries *"**What a render demands 7** — a technology named at launch with no hardware behind it stops the run"*, and [What has to travel with a file](../../Blender%20Research/Across%20topics/What%20has%20to%20travel%20with%20a%20file.md), which cites it for the technology living outside the file. It is also already cited in this file at [C12](#c12-the-third-position-on-the-launch-line-and-the-device-argument-that-sits-in-it--added-2026-09-01-out-of-the-repaired-blender-research-l8-l24a-job-files-and-outputs-d11) for the double-dash position.

**Owned by:** *What a render demands 7*, in [Preparation/Blender Research/Comparison/What a render demands.md](../../Blender%20Research/Comparison/What%20a%20render%20demands.md), question 7. Read 2026-09-02.

**Labels, exactly as the comparison carries them.** **Observed in a test** for every figure below — *"on the Linux machine, in all four versions, three repeats of every measurement on a machine checked to be quiet"*. **Not in the sources** for the numeric form of the question: *"No version gives a threshold, a count or a figure of any kind."* The documents establish only that the ranking depends on the scene tested, never how.

**What the comparison carries, in its own words.** *"At 322 triangles the two NVIDIA methods are within a tenth of a second of each other, with CUDA marginally ahead."* *"At 10,485,762 triangles OptiX is about four times faster than CUDA … On the curve-heavy scene the gap is three to five times, the same way round."* *"At 125,829,122 triangles the choice decides whether there is a render at all. Every OptiX run finished and wrote its frame, in all four versions. **Every CUDA run was killed by the system**, in all four versions, part-way through building the structure, with no frame written."* And the control on the finding: *"A negative result was available and did not happen."*

**Not covered, in the comparison's own words, and both bounds are carried in the document.** *"The condition reached was the **host machine's** memory ceiling, not the **card's**. OptiX never used more than about eight of the card's ten gigabytes and CUDA died long before it got near, so nothing here touches what happens when a card's own memory fills"* — so where the threshold sits on another machine is not established, and [Section 9](../Rendering.md#9-unresolved) item 3 marks that half permanently unresolved under the Creator's ruling of 2026-08-31. And the noise floor the pixel comparison is read against: *"this long measurement is macOS and Apple's own graphics only"* — a hundred and twenty pairs across four versions, three scenes, five repeats each; the Linux machine's own same-method control was one pair per row, between 4.17 × 10⁻⁷ and 9.54 × 10⁻⁷. **One card, one make, one machine**: this project has never had an AMD or an Intel card, so L23's other two rows are untouched by any of it.

**What this document takes from it, and only this.** That the direction between the two NVIDIA methods turns on the size of the scene and changes sign, which replaces L23's standing pair-of-readings paragraph; that above some size the choice decides whether there is a render at all; and the two bounds above. **What it does not take:** the decision itself, which is unmoved and rests on the omitted OptiX library at [M10](#m10-six-machines--the-ready-action-and-the-omitted-optix-library-l23) and on the three pictures at [M17](#m17-factsmd--fact-107-the-earlier-cuda-against-optix-pair-and-the-three-pictures-l23-section-9-item-3) — neither a speed claim, and neither disturbed by this reading. *(From 2026-10-03 the NVIDIA value is `OPTIX`, on the Creator's ruling, which is a decision and has no entry here. This reading and M10 and M17 are now what that value gains and costs, and no longer the ground for `CUDA`.)*

---

# Part 2 — The machine evidence: pointers to this stage's frozen input documents

Every entry in this part is **shape 1**. The document's own rule governs how each is read: *the frozen machine evidence is read as **this is what that box did on that day**, never as a general statement.*

## M1. The rented machine — the detached render and the undetached one (L6, K12, Row 19)

**Owned by:** [The rented machine](../The%20rented%20machine.md). A render started detached survived the connection dropping; a render **not** detached, sent the hang-up signal a dropped connection sends, *"the process ended at once"* — `Hangup` — with two finished frames on disk and the third, in flight, **lost entirely**.

**Not covered:** one box, one day. Nothing there measures how long a detached render survives beyond the observed run, and nothing establishes the behaviour on a machine kind other than that one.

## M2. The rented machine — the provider-console reboot (A4)

**Owned by:** [The rented machine](../The%20rented%20machine.md). The Creator clicked reboot in the provider's own interface while a 500-frame render ran detached. Blender had announced **eleven** frames as saved; all eleven were on disk whole. **Six further files sat beside them at zero bytes** — `f_0012.png` through `f_0017.png`, the placeholders Blender writes ahead of itself — so the folder said seventeen and the truth was eleven.

**Not covered:** one reboot, one provider, one day. The same document records that the first disk-full check applied to these frames was **faulty** and wrongly reported all eleven as damaged; the eleven-whole reading is the corrected one.

## M3. The rented machine — the purged driver, and the state that was not posed (L24, L25; Section 9 item 4)

**Owned by:** [The rented machine](../The%20rented%20machine.md), fifteen packages purged, `nvidia-smi` and `libcuda` gone. With the technology named at launch, Blender 4.5.3 ended at **exit 1**, wrote no file, and printed the found-no-device line.

**Not covered, and this is the larger half:** the other three versions for the purged case, and **the driver mismatched rather than removed** — the state an unattended upgrade produces — which was only ever observed producing a quiet processor render **when the device came from a preferences script**, the wrong side of the line the decision turns on. [U7](#u7-the-mismatched-driver-at-launch-section-9-item-4).

## M4. The rented machine — a killed transfer leaves a piece of the file at the final name (W3)

**Owned by:** [The rented machine](../The%20rented%20machine.md), cited by the document through `Dependency Upload & Remap` V13's refusal of the obvious tests. **Not covered:** what any other transfer tool does, and anything about the transfer contract's own staged route, under which part-arrived bytes never sit at a final name.

## M5. The last two boxes §3 — Fact 2, the execution group and the three signals (L5, K3, K4, K8)

**Owned by:** [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §3. On the virtual machine the execution group is the worker (`comm=python3`), its child the recorded target (`comm=blender`, **44 threads**), and *"the launch material `exec`s itself away so nothing sits between them"*; the container gave the same shape. Under all three signals the signal reached the recorded render process every time. The orphaned-worker half is yes with the boot registration's one-line correction — a render finishing under a supervisor it never met, **byte-for-byte identical** to the uninterrupted run, no frame re-rendered.

**The stop timings K8 quotes are that table's:** 1 to 3 seconds to a recorded outcome on the virtual machine, **1.9 to 14 seconds on the container**, on a small scene on a contended container.

**Not covered:** two boxes, one sitting. **No heavy frame was posed**, so nothing there bounds the latency to honour an interrupt on a frame that takes real time — [U1](#u1-k9s-60-second-deadline).

## M6. The last two boxes §6 — Fact 8 in full (S3, S9)

**Owned by:** [The last two boxes](../The%20last%20two%20boxes%20—%20the%20hook%20that%20cannot%20be%20read%20back,%20the%20full%20disk,%20and%20the%20unit%20that%20kills%20a%20render.md) §6. **312 trials run, no disagreement between the machine's answer and the queue**, the window about three milliseconds wide, *still in hand* never once returned.

**The tester's own disclaimer is now carried by the document, corrected 2026-08-23.** The same section records that **the first 82 trials were worthless and the tester says so** — round one, cold connections cut at up to one second against a handshake costing 1.6–1.9 seconds, so those cuts landed before the request was sent at all. **The usable count is 230**: round two, 120 warm trials at cuts of 0.02–0.40 s, and round three, 110 warm trials at 0.000–0.018 s. The document quoted 312 in four places — the *What binds this* summary, [Section 4](../Rendering.md#4-submitting)'s constraint line, S3 and S9 — and now quotes 230 usable of 312 run in all four. **The conclusion is unaffected:** the three-millisecond transition and the never-returned *still in hand* are both round three's, which is entirely inside the usable set.

## M7. One box with Blender on it §1 and §2 — Fact 5's payloads and the per-version property names (L10, P1 step 4)

**Owned by:** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) §1 and §2, including the rewrite taking effect in a real render and the unrewritten fixture building the foreign tree silently. The property name for a file output node's folder: **`base_path` in 4.5.9, `directory` in 5.0.1 and 5.2.0**.

**Not covered:** **Blender 5.1 was not exercised** — [U5](#u5-blender-51-section-9-item-2). Whether the reading's payloads carry the running Blender's own version identifier is **Fact 105**, not established — [U3b](#u3b-whether-the-machine-side-reading-names-its-own-blender-version-l14-section-9-item-16).

## M8. One box with Blender on it §5 — Fact 7's two halves (A7, A7a, A8)

**Owned by:** [One box with Blender on it](../One%20box%20with%20Blender%20on%20it%20—%20the%20reading,%20the%20listing,%20and%20the%20names%20that%20do%20not%20come%20back.md) §5. A truncated listing and a genuinely short one were both four lines, separable only by exit status; on the command channel a plain listing exits 2 for both the refused folder and the absent one, separated only by the English message.

**The two readings this entry once disagreed with the document about are now the document's own, corrected 2026-08-23.** The same section, corrected 2026-08-22 — **after** A7's own correction of 2026-08-20 — records, on the **job output folder itself** on the rented Linux box, with root and non-root controls either side: *"The stat of this folder separates them by exit status — 0 against 1 — and the directory test does not: `test -d` exits 0 for both."* And it bounds that in as many words: *"The stat half holds for the folder itself and not in general: a macOS run of 2026-08-21 put a stat and a file test on a path under an unreadable parent and both exited 1 in either case … a case not posed on this box."* **A7 now carries the first, A7a the second, and A7b says which reading was wrong and why.** The observed exit statuses behind it: `stat` 0 on the refused folder against 1 on the gone one; `ls -la` 2 on both; `test -d` recorded as 0 on both.

**Not covered:** one box, one day, one unprivileged login shape — the same bound the document's [Section 9](../Rendering.md#9-unresolved) item 15 puts on every machine claim. The English-message separation is stable on that box **only** because the image carries `C`, `C.utf8` and `POSIX` and the language variable is unset — and mining the machine's wording is what A7's own last sentence forbids in any case. The under-a-parent case has only the macOS run behind it and none on Linux.

## M9. Six machines — Fact 3's accepted names and the two distinguishable refusals (L8 item 9, L22, L24)

**Owned by:** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), Fact 3. `CPU`, `CUDA`, `OPTIX` and `OPTIX+CPU` all rendered at **exit 0**; `HIP`, `ONEAPI` and `METAL` gave **exit 1, nothing written**, `Error: Found no Cycles device of the specified type`; `BOGUS` gave **exit 1, nothing written**, `Error: BOGUS is not a valid Cycles device.` A broken driver is **not** distinguishable from absent hardware — both give the second line. The observed spelling of the argument is `--cycles-device`.

**Not covered:** **NVIDIA only.** AMD, Intel and Apple hardware are unposed and want another rental. The same entry records that `--help` prints one list for every platform and **that list is wrong about this build** — the setting itself refuses `METAL` outright with `enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`, so three routes give three answers on one build. That bears on L23, which now states the asymmetry — see [C2](#c2-l23s-asymmetry-between-two-published-lists--repaired-by-stating-it).

## M10. Six machines — the ready action, and the omitted OptiX library (L23)

**Owned by:** [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md), the ready-action findings: `ubuntu-drivers install --gpgpu` **does not install `nvidia-smi`** and **omits `libnvoptix`**, so OptiX fails with `Error: Found no Cycles device of the specified type` **while CUDA works**. On a fat provider image `libnvoptix.so.1` and `libcuda.so.1` came with the driver package and no CUDA toolkit was ever needed on any of the machines.

**What that library is** is a direct publication quotation and is entered at [P3](#p3-nvidias-installed-components-list-for-the-linux-display-driver--libnvoptixso1-l23).

**Not covered:** four images on three providers. The mechanism is settled; the list is per image and cannot be settled. The same document records that one of the four broke the fact's own precondition — it requires an untouched machine and was asked sixth on that provider.

## M11. Two more boxes — Fact 6's choice half (Section 9 item 8)

**Owned by:** [Two more boxes](../Two%20more%20boxes%20—%20the%20volume,%20the%20full%20disk,%20and%20a%20day%20left%20running.md), Fact 6. A rental does offer a choice of disk; on that box **it is not a second disk** — a 20 GiB quota on a slice of the same 1.8 T physical disk. Everything on both mounts survived a stop and a start, both markers byte-identical.

**Not covered, and it is exactly what the document's item 8 still owes:** **the machine's own unattended housekeeping over days was not posed**, and neither was the recycle the previous batch found on a container. The *more than one physical disk* half is answered elsewhere and the document routes it to `Job files and outputs` L4 rather than here.

## M12. The multi-provider batch — what the every-card promise is worth (L21)

**Owned by:** [The multi-provider batch](../The%20multi-provider%20batch%20—%20the%20surface%20that%20did%20not%20vary,%20the%20Debian%20route%20that%20exists,%20and%20the%20cards%20a%20slice%20cannot%20see.md), two rented multi-card boxes, both Blender versions: GTX 1660 SUPER at 960×540 and 3000 samples, 27.19 / 27.37 s on one card against 15.97 / 16.05 s on two — **1.70×**; RTX 5090 at 1920×1080 and 3000 samples, about **1.8×** on both CUDA and OptiX.

**And the negative that had to be got past, which the same document records:** a first attempt at 480×270 and 400 samples gave **3.01 s on one card and 2.76 s on two** — *"the shape of a false negative"*, since at that size the answer would have read as *two cards take as long as one*.

**Not covered:** how any of it behaves on a scene that does not fit in one card's memory. The GTX 1660 SUPER row is a single pair of runs with **no version split** and carries no version claim.

## M13. The multi-provider batch — the backend selector arrives off (L21a, L24)

**Owned by:** the same document. From `--factory-startup`, in **both** Blender versions on **both** boxes, `bpy.context.preferences.addons["cycles"].preferences.compute_device_type` arrives as **`NONE`** and a Cycles render therefore runs **on the processor** — while every individual device in the list arrives with `use = True`, both CUDA entries and both OptiX entries. On machine E a 480×270, 400-sample render took **12.52 s** with the settings as they arrive and **3.01 s** once the technology was set and one card selected.

**Not covered, and the document is now narrowed to it, corrected 2026-08-23:** the reading was taken **from `--factory-startup`**, which is precisely the launch mode L9 refuses to use and the one L9 itself says *"discards"* the machine's saved preferences a plain run reads. So it is a statement about a **factory-default preference set** and not about the preference set a plain run would find on that box, which was posed on neither. L21a and L24 both said more than that and both now say exactly that; the residue is the document's [Section 9](../Rendering.md#9-unresolved) item 20, entered at [U16](#u16-what-a-fresh-rentals-own-saved-blender-preferences-hold-section-9-item-20).

**A conflict, stated rather than resolved. Added 2026-08-31, out of the repaired Blender research.** [L21a](../Rendering.md), [L24](../Rendering.md) and `Servers` H16 rest on this batch record's *"every individual device in the list arrives with `use = True`"*. **The repaired *The machine 5* says the opposite of the list itself: the windowless runs came back *empty* in all four versions with an untouched settings folder, and empty again under factory settings, filling only once a refresh was asked for.** **This batch's own record does not say whether its script called a refresh**, so the two cannot be reconciled from anything the project holds. Per the Architecture method's rule for a disagreement between a reading and a run, and per the front door's rule for this stage, **it is stated here and not resolved**, and no clause is rewritten on it by this file.

**Where it is now written up as permanently unresolved, added 2026-09-02.** `Server Setup` [Section 13](../Server%20Setup.md#13-unresolved) item 25 and `Servers` [Section 16](../Servers.md#16-unresolved) item 24, both naming the Creator's ruling of 2026-08-31 — no further Blender research and no further rented machine — so **no observation will settle it**. **Nothing in this document moves either way:** [L21a](../Rendering.md) is a prohibition on inferring anything from a device list and holds whatever the list holds, and [L24](../Rendering.md) rests on the backend selector arriving `NONE` rather than on the list's fullness. The safe reading for anything that does take such a list is `Server Setup` **R1a**'s — the refresh is asked for first, and an unrefreshed empty list is never read as *this machine has no cards*.

**And the same repair narrows the cold reading. Added 2026-08-31.** **Every card reading on record was taken after a refresh; asked cold, a four-card machine and a cardless one give the same answer.** Label: **observed in a test**. `Servers` H16 declines that route already, so nothing breaks there and the repair strengthens its third reason; `Facts.md` Fact 46 is where the pointer should gain **The machine 5**, which is that register's and is not edited here.

## M14. The multi-provider batch — the build's device-type list is not a capability list (L23)

**Owned by:** the same document. `preferences.get_device_types()` returned `['NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI']` on machines with NVIDIA cards and **no AMD hardware**, and Blender 5.2 printed `WARNING HIPEW initialization failed: Error opening HIP dynamic library` on the same box. The list is what the build was compiled with.

**The asymmetry between this list and the manual's option list is now stated in the document, added 2026-08-23.** L23 rejects this one as a capability list and fills its AMD and Intel rows from the other; [M9](#m9-six-machines--fact-3s-accepted-names-and-the-two-distinguishable-refusals-l8-item-9-l22-l24) and [P5](#p5-blenders-command-line-arguments-page--the-accepted-device-names-l23) record that the other is no better — printed identically for every platform and wrong about that build. **Neither is read as a capability list; the option list is taken for the spelling alone**, which is all the table's third column claims for those two rows.

**Not covered:** two boxes, one seller family, two versions.

## M15. The multi-provider batch — OptiX against CUDA on an RTX 5090 (L23)

**Owned by:** the same document. OptiX faster in both versions at both card counts — 16.70 s and 16.51 s against 22.03 s and 21.82 s on one card, 9.23 s and 9.44 s against 12.30 s and 12.31 s on two. **About 1.3× in OptiX's favour.**

**Not covered:** one card model, one scene, one day. It goes the opposite way to the earlier reading at [M17](#m17-factsmd--fact-107-the-earlier-cuda-against-optix-pair-and-the-three-pictures-l23-section-9-item-3). **The document stated outright that neither settled the direction until 2026-09-02, and it no longer does:** [B15](#b15-which-of-a-cards-two-rendering-methods-to-name-read-against-the-size-of-the-scene-l23-section-9-item-3-section-10s-twelfth-check--added-2026-09-02) resolves both by scene size — this reading's scene is 1920×1080 at 3000 samples on an RTX 5090, and the earlier one's is 640×480 at 1024 samples, which is the split B15 measures directly. **Neither of these two is superseded as a measurement**; what is superseded is the claim that no reading settles the direction.

## M16. Facts.md — Facts 105 and 106, both **not answered** (L14, K9; Section 9 items 16 and 7)

**Owned by:** [Facts](../Facts.md), the stage's live fact register, Facts 105 and 106, both numbered from this document on 2026-08-19 and both carrying an explicit *not answered* marker. **Not covered:** neither has a machine behind it. What each costs is entered at [U1](#u1-k9s-60-second-deadline) and [U3b](#u3b-whether-the-machine-side-reading-names-its-own-blender-version-l14-section-9-item-16).

## M17. Facts.md — Fact 107, the earlier CUDA-against-OptiX pair, and the three pictures (L23; Section 9 item 3)

**Owned by:** [Facts](../Facts.md), Fact 107, which names the file the reading sits in: `Preparation/Blender Research/Findings/5.1/What a render demands — second pass.md`, hand test 28 — a rented Linux machine with one NVIDIA GeForce RTX 4080 SUPER, driver 580.173.02, Ubuntu 22.04.5, Blender 5.1.2, the same 640×480 file at 1024 samples, run back to back: **OptiX 4.14 s** on Blender's own `Time:` line and 5.82 s wall, **CUDA 3.62 s and 4.45 s wall — the faster of the two**. The same block compared the pictures pixel by pixel: CUDA matched the card reference to 2.9e-06 while **OptiX matched neither** — 0.0618 against the processor and 0.0532 against the CUDA card. *"Three devices, three pictures."*

**Not covered, in the findings file's own words:** the machine was *"shared with another topic's tests"* while these ran — one busy machine, one card, one small scene, one day, and a scene small enough that kernel-build time is a large share of it.

**Fact 107's re-posed half is ANSWERED, 2026-09-02, and this entry said it was not.** It stood *"re-posed and not answered on a scene big enough that kernel-build time is not a large share of it, on an uncontended machine"* — which names the conditions *What a render demands 7* was run under, on the one rented machine this project ever **checked** to be uncontended. See [B15](#b15-which-of-a-cards-two-rendering-methods-to-name-read-against-the-size-of-the-scene-l23-section-9-item-3-section-10s-twelfth-check--added-2026-09-02). **The pixel half of this entry is untouched and is still the ground L23's decision rests on**, together with the omitted OptiX library at [M10](#m10-six-machines--the-ready-action-and-the-omitted-optix-library-l23). *(Until 2026-10-03. From that day the NVIDIA value is `OPTIX` on the Creator's ruling, and this pixel half and M10 are its costs rather than its grounds.)* **What is left unanswered is narrower than what this entry claimed and is marked permanently unresolved at [Section 9](../Rendering.md#9-unresolved) item 3:** where the threshold sits on any machine other than the one measured, the one measurement having met the **host's** memory ceiling rather than the card's. *(`Facts.md`'s own Fact 107 entry still reads as unanswered on both routes; that is another document's and is not corrected from here.)*

## M18. Render Settings' companion sources, item 5 — the failure-to-exit-status switch (L8a; Section 9 item 19)

**Owned by:** [Render Settings sources](Render%20Settings.sources.md), the controlled test of 2026-08-19, item 5. A launch script raising a deliberate error after the file: both versions *"printed the traceback, **rendered anyway** (three frames written), and **exited 0**"*; *"With `--python-exit-code 63` on the line, both versions **ended at exit 63 with no frame written**."*

This is a pointer to a peer answer document's companion rather than to a frozen input document; the citation is owned there and is not re-taken here.

**Not covered:** one container, one day, one fixture family; Blender 4.5.12 and 5.2.0 only, **5.0 and 5.1 untried**; every render at 64×64 on the processor. **Where on the line the switch sat was not varied** — [U2](#u2-where-on-the-launch-line-the-failure-to-exit-status-switch-must-sit-l8a-section-9-item-19).

---

# Part 3 — Outside publications quoted directly

Every entry in this part is **shape 2**: the exact words the document carries, the address, and the date read, taken from the document itself or from elsewhere in this project. Where the project records no address, the entry says so and no address is supplied.

## P1. The Open Group Base Specifications Issue 8 — standard-stream buffering (L7's named property)

**Source:** The Open Group Base Specifications **Issue 8**, IEEE Std 1003.1-2024, page titled `stdin, stderr, stdout — standard I/O streams`. **Address: `https://pubs.opengroup.org/onlinepubs/9799919799/functions/stdin.html`. Read 2026-08-23** — the page itself, not a mirror or a summary. Label: **documented**.

**Exact words, as the publication carries them:** *"When opened, **stderr** shall not be fully buffered; **stdin** and **stdout** shall be fully buffered if and only if the file descriptor associated with the stream is determined not to be associated with an interactive device."*

**The publication was read against what the document carried, and one difference of form was found and corrected.** The words are the same. **The document carried them as two reordered fragments** — the *stdin* and *stdout* half first, quoted as though it began the sentence, and the *stderr* half second — with a date read of 2026-08-20 and **no address anywhere under `Preparation/`**. The publication's own single sentence now stands in the document, with this address and today's date (L7's named paragraph, corrected 2026-08-23). Nothing the clause concludes changes: a pipe is not an interactive device, so the ordinary output stream may hold what a program printed until it exits and the complaints stream does not.

**On the address's other home in this project.** It is also recorded at `Audits/Architecture/Sanity sweep/Verified — raw-queue-rendering.md`, which is outside this stage's reading and which this document may not lean on. That is why the page was read rather than copied. [Queue & History](../Queue%20&%20History.md) R4 carries the same quotation for its own liveness case and is being repaired separately; **this entry records only the reading made for this document, and duplicates nothing.**

**Beside it, a demonstration and not an observation — corrected 2026-08-23.** The two-line pipe test posed on the Creator's Mac — two lines printed two seconds apart, arriving two seconds apart on a terminal and both at once, at exit, on a pipe — **has no observation record anywhere under `Preparation/`**: no machine description, no commands, no captured output, no date. It does not meet this stage's evidence bar, it is **not** carried here as observed, and **no observation record was invented for it**. This entry rests on the publication alone, which carries the claim on its own authority. **This entry carried it as *observed in a test* until today**, against `Sources/Queue & History.sources.md`, which recorded the same finding and routed this clause here; the routing arrived with the stage-close pass.

**What this carries:** the ordinary output stream on a pipe may hold what a program has printed until the program exits, while the complaints stream does not.

**Not covered, and the document says so in place:** ~~**whether Blender itself flushes** — a Blender question, parked in [Open Threads](../../Groundwork/Open%20Threads.md) as the second of three questions owed to a fresh blind researcher, and assumed in neither direction here.~~ **Corrected 2026-09-02, twice over, and the struck sentence is stale on both halves.**

**First, it is not parked.** L7 was corrected on 2026-09-02 to say so: whether Blender itself flushes is a question about what Blender does, and the Creator ruled on 2026-08-31 that there will be no further Blender research — **so it is permanently unresolved rather than pending, and no reader may leave a space in the code or the record for the answer.** This entry went on calling it parked after the clause had stopped doing so.

**Second, the behaviour it was standing in for is answered by test, and the two must not be confused.** The *mechanism* — whether Blender calls a flush — is the permanently unresolved half above. The *behaviour* — when a printed line reaches a reader on the far end of a pipe — is settled: ***Reports 14***, in `While a render runs`, tested in all four versions on the Creator's Mac and on the rented Linux machine, with the reader proved able to show both a held and a prompt pattern before anything was concluded. Label: **observed in a test**. **A render's own printed lines reach a pipe exactly as they are produced** — dozens of separate arrivals across a run, in bursts at each frame boundary — while **a Python script's own printed lines reach neither a pipe nor a file until the run ends**. `Queue & History` R4 was narrowed to the script half on 2026-09-02 and its companion carries the answer in full; **it is recorded here because L7 is where the buffering property is stated and a reader of L7 alone would still think the render half open.**

**And what the timing costs is wider than L7's closing sentence says. Added 2026-09-02.** L7 ends *"what is at risk is liveness, which is `Queue & History` R4's"*, which reads as the live commentary alone. **The same streams carry a second reader's trigger**: `Queue & History` P1a takes the **frame's save announcement** observed in passing at the worker's copy loop — one of the five observations [Section 8](../Rendering.md#8-what-this-document-supplies-and-to-which-document) hands to the worker contract and K13 carries — and the newest-saved figure it reports is what queues preview work per frame. **So lines held in a pipe until exit would delay the previews as well as the words**, and every preview would arrive at the ending rather than during the render. **It costs nothing in fact and is recorded so nobody re-derives the bound as commentary alone:** the save announcement is a render's own printed line, which is the half *Reports 14* answers. **What holds either way:** `Queue & History` P1a already states the no-print case in full — no report, no per-frame trigger, P1b producing the set at the ending — so a held stream costs a delay and never a hole, and nothing in the capture is at risk, everything printed being captured whichever way it goes.

**Nothing in this entry changes a clause.** L7's conclusion — a pipe is not an interactive device, so the ordinary stream may hold what was printed until exit and the complaints stream does not — rests on the publication alone and is untouched. ~~**L7's own closing sentence still names the commentary alone and is not edited here**; the widening is carried on both companions and routed to the runner.~~ **The routing landed the same day: L7's closing sentence carries the widening as of 2026-09-02** — both readers named, R4's live readings and P1a's frame-save announcement, with the delay-never-a-hole conclusion above — **and it names the two observations that assert no stream as costing nothing here**, the device line and the colour-substitution line, both carried with the outcome rather than read live. What they rest on is the clause's *both streams are captured, always*, which this entry's buffering question does not touch.

## P2. The Open Group's specification for `test` — the three exit statuses (A7, A7a)

**Source:** The Open Group Base Specifications **Issue 8**, IEEE Std 1003.1-2024, page titled `test — evaluate expression`, EXIT STATUS. **Address: `https://pubs.opengroup.org/onlinepubs/9799919799/utilities/test.html`. Read 2026-08-23** — the page itself. Label: **documented**.

**Exact words, as the publication carries them:** *"0 — expression evaluated to true. 1 — expression evaluated to false or expression was missing. >1 — An error occurred."*

**The publication matches the document word for word.** No address was recorded anywhere in this project before this entry — the quotation and a date read of 2026-08-20 sat in the document itself and at `Audits/Architecture/Sanity sweep/Verified — shared — OpenSSH.md`, and neither carried one. The address above is supplied by reading the page, and the document now carries it with today's date.

**What this carries, and it is narrower than the document once made it:** the file test has **no third value** for *I was not allowed to look*, so it cannot separate a refusal from an absence even in principle. **That is a statement about the file test and about nothing else** — the specification says nothing about a stat, and the stat is what does separate the two for the folder itself ([M8](#m8-one-box-with-blender-on-it-5--fact-7s-two-halves-a7-a7a-a8)). A7 now cites it for exactly that and no more.

**Not covered:** the runs beside the original quotation were made **on macOS with the system's own programs**; the rentals are Linux with a different set. The specification is universal; the two program behaviours are not.

## P3. NVIDIA's installed-components list for the Linux display driver — `libnvoptix.so.1` (L23)

**Source:** NVIDIA's own installed-components list for the Linux display driver, **driver 580.105.08**, page titled `Chapter 5. Listing of Installed Components`. **Address: `https://download.nvidia.com/XFree86/Linux-x86_64/580.105.08/README/installedcomponents.html`. Read 2026-08-23** — the page itself. Label: **documented**.

**Exact words, as the publication carries them:** *"The OptiX library (`/usr/lib/libnvoptix.so.1`); This library implements the OptiX ray tracing engine. It is loaded by the `liboptix.so.*` library bundled with applications that use the OptiX API."*

**The publication matches the document, and carries one sentence more than the document quotes.** The document's quotation stops after *"…implements the OptiX ray tracing engine."*; the entry above restores the third sentence, which says the library is loaded by a bundled `liboptix.so.*` and strengthens rather than qualifies what L23 takes from it. The same page also lists a data file at `/usr/share/nvidia/nvoptix.bin`, which nothing here relies on. **No address was recorded under `Preparation/`** before this entry — it existed only at `Audits/Architecture/Sanity sweep/Verified — raw-queue-rendering.md`, which is outside this stage's reading, which is why the page was read.

**What this carries:** the library the one provider's driver route omitted is a **driver-installed file**, so its absence is a packaging matter on that provider's image rather than a property of the hardware.

**Not covered:** one driver version's component list. It does not say which driver packages a distribution's own installer selects, which is the behaviour [M10](#m10-six-machines--the-ready-action-and-the-omitted-optix-library-l23) observed.

## P4. Blender's command-line arguments page — the two automatic-script-execution switches (L9)

**Source:** the Blender manual's command-line arguments page and its scripting-security page. **Read 2026-07-30.** Label: **documented**.

**Exact words, as the document carries them:** *"`-y, --enable-autoexec` — Enable automatic Python script execution."* and *"`-Y, --disable-autoexec` — Disable automatic Python script execution (Python-drivers & startup scripts), (default)."*, with the security page's worked example of this launch shape: `blender --background --enable-autoexec my_movie.blend --render-anim`.

**Where the fact lives, and it is a comparison question:** ***Settings 11***, which carries the pair in all four versions — *"a preference called Auto Run Python Scripts … which two launch arguments override — one to enable, one to disable"* — and *"Every version's manual says automatic execution is off by default"*. *The machine 7* carries the same pair and the same worked shape, *"with the same worked example in every version — a background animation render"*. The pages quoted are `https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html` and `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html`, both read **2026-07-30**. *(This entry pointed only at two 5.1 findings documents until 2026-08-30. `Findings/` sits outside every downstream reading at every stage, per the [Blender Research method](../../Blender%20Research/Method.md); the comparison question is where the fact lives.)*

**What this carries:** L9's correction of itself — an argument for the stored-scripts preference **does** exist in both directions, and the contract's not using either is a choice rather than an absence. The default being off is documented. **Narrower than it reads, and *Settings 11* says so in place:** the `(default)` the arguments page prints beside the disabling switch *"describes the factory state of that preference, not an unconditional rule"* — with neither switch given, the preference decides. Nothing L9 turns on moves: it leaves the preference alone deliberately, rather than resting on the default.

**Not covered:** the pages read are the **5.1** manual's. Nothing here is a version comparison, and the document makes no per-version claim on these two switches. *Settings 11* records the security page's own gap: whether Freestyle's line-style scripts run without the enabling switch is settled in no version, *"and more machines do not help that."*

## P5. Blender's command-line arguments page — the accepted device names (L23)

**Source:** the Blender manual's command-line arguments page. **Read 2026-07-30.** Label: **documented**.

**Exact words, as the document carries them:** *"--cycles-device <device> … Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL. Append +CPU to a GPU device to render on both CPU and GPU."*

**Where the fact lives, and it is a comparison question:** ***The machine 5*** — *"One command-line option, `--cycles-device`, taking exactly the same six values in every version — CPU, CUDA, OPTIX, HIP, ONEAPI, METAL — with `+CPU` available to add the processor, and only read when a bare `--` comes first. Its help text is worded identically in all four."* *Settings 9* carries the double-dash rule from the other side, by test, in all four versions on both machines. The page quoted is `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html`, read **2026-07-30**. *(This entry pointed only at a 5.1 findings document until 2026-08-30. `Findings/` sits outside every downstream reading at every stage, per the [Blender Research method](../../Blender%20Research/Method.md); the comparison question is where the fact lives.)*

**What this carries:** the **words** `HIP` and `ONEAPI` that fill L23's AMD and Intel rows are published, so a second make costs a value rather than a mechanism.

**Not covered, and it is the sharper limit, now stated in the document (2026-08-23):** this list is **not a capability list**. [Six machines](../Six%20machines%20—%20the%20container,%20the%20webhook%20and%20the%20ready%20action.md) Fact 3 records that `blender --help` prints the identical list *"identically in 4.5.12 and 5.2.0"*, that **`--help` prints one list for every platform**, and that **it is wrong about that build** — the setting refuses `METAL` outright with `enum "METAL" not found in ('NONE', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI')`, three routes giving three answers on one build. So the published list establishes the **spelling** and nothing about whether hardware of that make then renders, which is what L23's two rows take from it and the whole content of their *hardware unposed* mark.

## P6. Blender's GPU-rendering page — which system each technology is supported on (L23's struck Apple row)

**Source:** the Blender manual's Cycles GPU-rendering page. **Read 2026-07-30.** Label: **documented**. **The document now cites this entry by name for the claim**, having stated it bare before 2026-08-23.

**Exact words, as the project records them:** CUDA *"is supported on Windows and Linux"*, OptiX *"is supported on Windows and Linux"*, HIP *"is supported on Windows and Linux"*, oneAPI *"is supported on Windows and Linux"*, and *"Metal for GPU acceleration is supported on Apple computers with Apple Silicon"*.

**Where the fact lives, and it is a comparison question:** ***The machine 2*** — *"the five technologies are CUDA, OptiX, HIP, oneAPI and Metal. CUDA, OptiX, HIP and oneAPI are stated as Windows and Linux only; Metal as macOS and Apple Silicon only"*, the same in all four versions. *What a render demands 7* carries the same documented split beside it. The page quoted is `https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`, read **2026-07-30**. *(This entry pointed only at a 5.1 findings document until 2026-08-30. `Findings/` sits outside every downstream reading at every stage, per the [Blender Research method](../../Blender%20Research/Method.md); the comparison question is where the fact lives. A second quotation stood beside that pointer — a comment out of Cycles' own shipped file — and is removed rather than repointed: no comparison carries it, and nothing in the document rests on it.)*

**What this carries:** the struck Apple row is documented rather than assumed, and the same page is the only project source that puts HIP and oneAPI on Linux at all, which is what makes L23's two documentation-filled rows relevant to a Linux rental.

**Not covered:** the **5.1** manual, one date. The page states support by system and says nothing about a particular card of a make being served — which is exactly the *hardware unposed* half L23 marks.

**One phrase L23 used to quote here has been removed from the document, corrected 2026-08-23.** *"every ordinary Linux rental"* is [The shape](../The%20shape.md)'s, where the sentence is a **condition on Fact 63's webhook route** — *"If Fact 63 shows that the webhook route cannot coexist with the receiving-slot fallback while every ordinary Linux rental remains usable, the agent stops…"* — and not a statement of which systems this project rents. It is a Smoozy document, it earns no citation, and it was no evidence for the struck Apple row. **What the row now rests on** is this entry for the Apple-Silicon-only half and the frozen input documents' own record of every machine this project has rented being Linux, carried at the document's [Section 9](../Rendering.md#9-unresolved) item 13.

---

# Part 4 — Assumptions, and claims with no source in this project

Every entry in this part is **shape 3**. Each says what breaks if it is wrong, and says plainly that no source was found. **The folders searched for all of them:** `Preparation/Blender Research/` (joined answers, comparison questions, findings, question register), `Preparation/Architecture/` (the frozen input documents, `Facts.md`, `Coverage.md`, the answer documents and their `Sources/` companions), `Preparation/Specs/` and its `Sources/`, `Preparation/Groundwork/` including `Open Threads.md`.

## U1. K9's ~~60-second~~ **120-second** deadline

**REWRITTEN 2026-08-31. The deadline was raised from 60 seconds to 120 on the runner's ruling, and this entry had gone on calling Fact 106 unanswered nine days after it was answered.** The original follows, struck.

**Label: assumption**, for the value. **Two bodies of measurement now bound it, and they disagree.**

**Fact 106 — answered 2026-08-25**, on the Pile 3 rental. A frame Blender itself estimated at ten and a half hours was interrupted four times: **2.666, 8.142, 13.437 and 29.946 seconds**, against a sample batch of **37.3 seconds**. The mechanism it establishes is that **the latency is the time left in the current sample batch and nothing else** — Blender announces the break at once, finishes the batch in flight, then cancels — and that how deep the frame is does not matter. Label: **observed in a test**.

***Frames 15*** — sixteen tests~~ on one scene at **320 by 180**~~, whose **longest recorded stop waits are sixty-five to seventy-three seconds, in all four supported versions.** Label: **observed in a test**. This is the body no architecture document carried until 2026-08-31; it lived only under `Audits/`, which a builder is told not to read. **Corrected 2026-10-01: those waits were not on a 320 by 180 scene, which is the Cycles scene most of *Frames 15* used.** They are its entry 13: an EEVEE render at 1920 by 1080, frames 1 to 500, on the rented Linux machine, whose viewport is drawn by its processor and where each frame took about ninety seconds — 72.84 s in 4.5, 66.37 in 5.0, 68.04 in 5.1, 65.04 in 5.2, against 0.54 to 2.35 s for the same render on the Mac, and the research says the two machines' figures are not a comparison of anything (`Findings/4.5/Frames — Architecture questions, second pass.md` §13, and the same entry for 5.0, 5.1 and 5.2). Label: **observed in a test**. `Rendering` K9 and `Facts.md` Fact 106 were corrected the same way the same day. Where the paragraphs below call *Frames 15*'s long waits a small scene's, the words are struck; why those waits were long is still not established.

**They are not reconciled, and this stage does not reconcile them.** Fact 106's mechanism predicts a *short* wait on a small scene~~, and *Frames 15*'s long waits are on a small scene~~ *(corrected 2026-10-01: they were not; see above)*. Nothing in this project explains the difference, and per the Architecture front door this is stated rather than resolved. **Neither result is doubted; what is unknown is what separates them.**

**The two scenes are named here, because one reading of the disagreement was routed in and is false. Added 2026-09-02.** The claim was that **both** measurement sets are small scenes. **Half of it is right and half is the opposite of right.** ~~*Frames 15*' scene is small — **320 by 180**, sixteen tests, as this entry already states.~~ *(Corrected 2026-10-01: most of *Frames 15* used that scene, but its long waits were entry 13, at 1920 by 1080; see above.)* **Fact 106's is a deliberately expensive scene**, and [The Pile 3 batch](../The%20Pile%203%20batch%20%E2%80%94%20a%20full%20mount%20under%20a%20running%20supervisor,%20an%20interrupted%20heavy%20frame,%20and%20an%20hour%20of%20silence.md) §2 says so in its own words: *"A deliberately expensive scene: Cycles on the card, 16384 samples, adaptive sampling off, denoising off, 24 bounces, 1920×1080 at 100%, thirty-six subdivision-5 icospheres with transmissive materials"*, with Blender's own estimate for the one frame at **ten and a half hours** and a sample batch of sixteen every 37.3 seconds. Label: **observed in a test**. **Nothing was changed on the claim and no value moved.**

~~**What is true, and it is sharper than the claim was: the two sets differ in scene size in the direction opposite to their waits.**~~ The 1920-by-1080 scene at 16384 samples gave the **short** waits — 2.666 to 29.946 seconds~~ — and the 320-by-180 scene gave the **long** ones, sixty-five to seventy-three~~. **So scene size is not the separator**~~**, and a reader reaching for it has the sign backwards**~~ *(corrected 2026-10-01: the long waits were also at 1920 by 1080, an EEVEE render on a machine whose viewport is drawn by its processor, so the two sets do not differ in scene size at all; see above)*, which is the same conclusion the paragraph below draws from the mechanism's end (neither *a heavy frame*, depth not mattering, nor only *a scene whose batch runs long*). Recorded because *small scene* appears in this entry describing *Frames 15*' alone, and a later reader could carry the word across to Fact 106's, where it is false. **Nothing rests on it either way:** K9's 120 clears the highest observation of both sets, and the asymmetry below is why the value is safe in the direction it errs.

**And they will not be reconciled. PERMANENTLY UNRESOLVED, 2026-09-02.** Both bodies are in hand and neither explains the other, so separating them needs a fresh run on a rented machine — and the Creator ruled on 2026-08-31 that there will be no further rental to answer a question and no further Blender research. **This is not a pending item and no reader may leave a space in the code or the record for a reconciliation that is coming.** **What breaks:** nothing establishes a ceiling on the wait at all, so K9's 120 is bounded from above only by the higher of two disagreeing measurements. **What holds:** the asymmetry above, which is why the value is safe in the direction it errs; and the case a builder watches for, which is neither *a heavy frame* — Fact 106 settles that depth does not matter — nor only *a scene whose batch runs long*, the long waits being~~ on a small scene and~~ unaccounted for by batch length *(corrected 2026-10-01: not a small scene; see above)*. **No silence to remove:** expiry writes nothing durable and the entry shows *stopped* when the outcome lands, so a wait past the deadline corrects itself in front of the user. [Section 9](../Rendering.md#9-unresolved) item 7 carries the same line at the document's own end.

**Why 120 and not 60.** Sixty is **below an observed value**, so on the shape *Frames 15* tested the waiter would expire routinely on renders that were stopping normally, and K10's premature sentence would be the ordinary case rather than the corner. One hundred and twenty clears the highest observation with room. **The asymmetry is the argument**: expiry changes no durable state (K10, K11; C16–C17), so a long deadline costs only a slower honest message, while a short one costs a false *not stopped* on an ordinary render.

**Breaks:** a render slower than the deadline to honour the stop is reported as not stopped and then stops — unchanged in kind, and now less likely. **What holds:** the sentence's own honesty (K10, K11), the entry correcting itself when the outcome lands, and the value being one named constant changed by one edit.

~~**Label: assumption.** **No source.** The value is bounded below by [M5](#m5-the-last-two-boxes-3--fact-2-the-execution-group-and-the-three-signals-l5-k3-k4-k8)'s worst observed time to a recorded outcome — 14 seconds on the container — by a factor of four, and by nothing above. **Fact 106**, *a heavy frame's latency to honour an interrupt*, is numbered and **not answered**; no evidence in this project covers it.~~

## U2. Where on the launch line the failure-to-exit-status switch must sit (L8a; Section 9 item 19)

**Label: assumption.** **No source.** [M18](#m18-render-settings-companion-sources-item-5--the-failure-to-exit-status-switch-l8a-section-9-item-19)'s observation is *"with `--python-exit-code 63` on the line"* and says nothing about position; nothing in `Preparation/` states where the switch is honoured from. **Breaks:** if it is honoured only from a position this contract does not use, every fail-loud conduct in the launch material is silent again — a wrong render at exit 0. **What holds:** the observed positive itself, and the head being the position taken by Blender's own no-window mode.

## U3a. Whether the regular output path's argument is in force when the launch material runs (Section 9 item 1)

**Label: ANSWERED, 2026-08-31, and this entry was five days behind it. Corrected 2026-09-01 at the paperwork sweep.** The Build stage's opening found the answer already inside the project — *Writing 13*, five arrangements of the file, the setting and the script, all four versions, two machines — and **the arrangement that works is the file, then the setting, then the script, which is L8's own composition order.** So the read-back sees the argument's value and every job renders. `Job files and outputs` §14 item 19 and `Rented-server runtime` §16 item 11 are both closed; **the residue is item 19a, which scene an imposed path lands on, and it fails safe.** *(The superseded entry follows, kept because its *Breaks* line is the reason the read-back exists at all.)* ~~**Label: unresolved.** **No source.** The question is owed to Blender Research and is recorded as open in `Job files and outputs` §14 item 19 and `Rented-server runtime` §16 item 11;~~ no comparison question and no joined answer addresses ordering between an argument and a handed-in script. **Breaks:** the read-back mismatches and **every attempt fails before the render instruction** — no job renders anywhere. **What holds:** the failure is safe and total rather than silent, and the marker names what was read.

## U3b. Whether the machine-side reading names its own Blender version (L14; Section 9 item 16)

**Label: unresolved — Fact 105, not answered.** **No source.** [M7](#m7-one-box-with-blender-on-it-1-and-2--fact-5s-payloads-and-the-per-version-property-names-l10-p1-step-4) answers Fact 5 on three payloads and **none of them is the version**; what was observed is that the values returned were version-correct, which is a different claim. **Breaks:** L14's free ride only — the confirmation becomes one short headless invocation on every attempt. **What holds:** the confirmation entire.

## U4. A per-card form of the device argument (L21)

**Label: answered — the argument has none, in all four versions. Corrected 2026-08-30 at the citation audit, with L21.** *The machine 5* states it outright: the option *"names a technology and has no way of naming one card out of several"*, and *"individual cards are reachable only through the saved preferences file or through Python"* — in all four versions, both as documented and as tested on machines with one, two and four cards. [P5](#p5-blenders-command-line-arguments-page--the-accepted-device-names-l23)'s published option list carries no per-card form either. **This entry read *unresolved, and recorded as honesty*, on a 2026-08-21 search of `Preparation/Blender Research/` and `Audits/Blender Research/` reported as finding no index form *and no statement that one does not exist*. The first half stands and the second was wrong**, the statement sitting in the very comparison [B9](#b9-the-device-argument-l21-l21a-l22-l24--rows-8-and-9) already names; the Open Threads entry that search fed is the runner's to close. **Nothing rests on it:** L21's promise is every card, the exclusive-render rule gives one render the machine, and no row asks for per-card selection. It stands in [Open Threads](../../Groundwork/Open%20Threads.md) as a fourth question deliberately **not** sent to a researcher, which was the right call for a different reason from the one recorded there: the answer was already in hand.

## U5. Blender 5.1 (Section 9 item 2)

**Label: inferred.** **No source.** 5.1 was not exercised for the per-version property names or for the reading ([M7](#m7-one-box-with-blender-on-it-1-and-2--fact-5s-payloads-and-the-per-version-property-names-l10-p1-step-4)) and 5.0 and 5.1 were untried in [M18](#m18-render-settings-companion-sources-item-5--the-failure-to-exit-status-switch-l8a-section-9-item-19)'s controlled test. **Breaks:** a version-specific difference nobody has looked for. **What holds:** the version boundary is the research's own — one name in 4.5, another from 5.0 — so 5.1 sits inside the from-5.0 branch.

## U6. AMD and Intel hardware (Section 9 item 3)

**Label: documented for the word, hardware unposed.** The **words** have a source — [P5](#p5-blenders-command-line-arguments-page--the-accepted-device-names-l23) and [P6](#p6-blenders-gpu-rendering-page--which-system-each-technology-is-supported-on-l23s-struck-apple-row). **No source at all for the hardware:** no machine this project has had carried an AMD or Intel card, which *Settings 9* also records. **Breaks:** a cards job on such a machine costs a failed attempt on the machine, ending before the render instruction with the make it could not serve recorded. **What holds:** the mechanism entire; a further make costs one table value.

## U7. The mismatched driver at launch (Section 9 item 4)

**Label: unresolved.** **No source.** The mismatched state — what an unattended upgrade produces — was only ever observed through a preferences script ([M3](#m3-the-rented-machine--the-purged-driver-and-the-state-that-was-not-posed-l24-l25-section-9-item-4)), which is the wrong side of the line the decision turns on, and the purged case is one version. **Breaks:** a machine in that state could render on the processor at success despite L24. **What holds:** the purged case's refusal and the two distinguishable refusal lines.

## U8. What else 4.5's debugging switch prints (L27; Section 9 item 5)

**Label: unresolved.** **No source.** *What actually ran 1* records that 4.5 *"printed hundreds of lines"* under every log switch it has without naming a device, but nothing enumerates what the Cycles debugging switch itself puts on the complaints stream. **Breaks:** the raw record behind a 4.5 failure may carry debug lines beside the failure text the user opens. **What holds:** the streams are kept apart and whole, and from 5.0 the case does not arise.

## U9. Whether an EEVEE or Workbench job names any device (Section 9 item 6)

**Label: unresolved.** **No source.** Every result on record in *What actually ran 1* is Cycles' own. **Breaks:** the entry says the job's devices are not known for such jobs even when they finish — the spec's own fallback sentence, so the promise is kept in its weaker stated form. **What holds:** the Cycles case, which is what the every-card promise exists for.

## U10. A job's folder across the machine's own unattended housekeeping (Section 9 item 8)

**Label: unresolved.** **No source.** [M11](#m11-two-more-boxes--fact-6s-choice-half-section-9-item-8) posed a stop and a start and nothing longer. **Breaks:** a folder vanishing between connections reads as absent paths in the completeness statement — visible, not silent. **What holds:** nothing reads presence as state.

## U11. Simplify's inferred pair (Section 9 item 11)

**Label: inferred, and narrowed — the inference is now the launch route alone.** **Narrowed 2026-08-29 at the re-gate, which found this entry still carrying the pre-narrowing text while the document's [Section 9](../Rendering.md#9-unresolved) item 11 had been narrowed on 2026-08-29 with A1d, so the two said different things about the same label.** **What is answered**, by *What a render demands 6* ([B14](#b14-what-a-saved-scene-records-for-simplify-and-which-half-of-each-pair-a-job-sets-a1d-l10s-simplify-bullet-section-9-items-11-and-23)): every Simplify value was read back in all four versions on two machines, so *never read back* is answered for Child Particles; and the hand test rendered with all three render-half limits in force and changed the picture materially in every version, so *reaches a render* is answered for all three **where the values sit in the saved file**. **What is still inferred, and it is the whole of what this entry now claims:** the **launch route** for Texture Limit and Child Particles — a value assigned by the launch material rather than saved in the file — plus the original graphics-device and Windows bounds. **No source for the launch route.** The Creator ruled **2026-08-12** that the promise stands as it is, and that ruling is untouched. **Breaks:** a Simplify job whose Texture Limit or Child Particles did not take renders bigger than asked, at success. **What holds:** the switch and Max Subdivision, observed changing the picture in all four versions on both machines. *(This entry read, until today: no source for the two; Texture Limit written into a file and read back but never imposed as a render started; Child Particles read in documentation and neither imposed nor read back.)*

## U12. What an interface-saved file records as its showing scene (L16; Section 9 item 12)

**Label: unresolved.** **No source.** Every file the research tried was saved by a Blender with no window, and an artist's file is saved from the interface. **Breaks:** the unnamed-scene fallback only. **What holds:** every named-scene job, which is every job whose file was read.

## U13. Windows (Section 9 item 13)

**Label: unresolved, and off this document's path.** **No source** for the redirect, Simplify or the frame-list forms on Windows; the research's own limit lists say so, and one findings file records outright that *"Windows was not tried"*. **Breaks nothing here:** every rented machine is Linux, so the untried half is not on the machine-side path, and the desktop side composes rather than renders. Recorded so no later reader takes the contract as demonstrated anywhere but Linux.

## U14. The five unposed remap kinds (Section 9 item 10)

**Label: unresolved, narrowed.** Facts 100 and 101 are answered and Fact 99 is answered for the **image** kind; **cache file, sound, font, movie clip and volume are not posed and are not claimed** — routed to `Dependency Upload & Remap` M9b, whose document owns them. **No source** for the five. **Breaks:** a kind for which assignment alone is not enough renders silently missing that kind's assets at success. **What holds:** the mechanism accommodates a per-kind reload step, and the marker records every pair's outcome. **One trap travels with it:** `image.has_data` read `False` throughout while the render was reading the file correctly, so nothing gates on it.

## U15. Every machine claim in this document (Section 9 item 15)

**Label: observed, and bounded.** One to six boxes on one to three days. The stop timings, the refusal lines, the naming forms and the survival observations are frozen snapshots. Recorded so no later document reads a figure in this document as a description of rented machines.

## U16. What a fresh rental's own saved Blender preferences hold (Section 9 item 20)

**Label: unresolved.** **No source.** Added 2026-08-23. Both readings of the backend selector arriving as `NONE` were taken from `--factory-startup` ([M13](#m13-the-multi-provider-batch--the-backend-selector-arrives-off-l21a-l24)); nothing in `Preparation/` records what a plain run finds in a fresh rental's saved preferences. **Breaks:** nothing in the mechanism — the technology named at launch overrides both the file and the machine's saved preferences (*Settings 9*, *The machine 5*) — only any wider claim built on the reading, and L21a and L24 now make none. **What holds:** that a factory-default preference set alone reaches the quiet processor render, which is L24's force; and L9's own ground, which is *What a render demands 3*'s and *The machine 5*'s.

## U17. L10a's name check, and every name behind it (Section 9 item 25)

**Label: bounded observation, and one inference on top of it.** Added 2026-08-29 with L10a. What *Settings 17* observed is that in 4.5 a name Cycles' settings do not carry is accepted silently and *"the value is gone the instant it is read back"* — an answer taken **after** the swallowed assignment. L10a reads the name back at that same place and fails the attempt on it; **the check itself was run in no version on no machine**, and the inference is that nothing is written onto the object by the swallow, which is what the vanished value says and nothing contradicts. **The whole comparison is one Mac on Apple silicon and no Linux machine tried any of these names** — `Render Settings` §13 item 28's bound, which is the wider gap.

**Breaks:** a version that does carry the name after swallowing it leaves the 4.5 Cycles hole open — a paid render at the file's own count at exit 0, the one shape L8a cannot reach because nothing raises. **What holds:** every other settings group in every version, refused loudly; the names themselves, identical in all four; and one run at Build settling the check on a material this contract already composes.

## U18. An out-of-range assignment to Simplify's two typed settings (Section 9 item 23)

**Label: unresolved.** **No source.** Added 2026-08-29 at the re-gate, with A1d's two typed boxes. *What a render demands 6* documents the **ranges** — a whole number from 0 to 32767 for maximum subdivision, a fraction from 0 to 1 for child particles, the same in all four versions ([B14](#b14-what-a-saved-scene-records-for-simplify-and-which-half-of-each-pair-a-job-sets-a1d-l10s-simplify-bullet-section-9-items-11-and-23)) — and **poses no out-of-range assignment**. The one silent-correction finding this stage holds is the resolution percentage setting's own (*Settings 15*; `Render Settings` J12), and carrying it to a second setting is the assembling [Method](../Method.md) forbids, so it is not carried. **Breaks:** the offer's two refusal lines state the range alone rather than what Blender would otherwise have done silently — Row 437's whole-app sentence weaker in the same shape `Render Settings` §13 item 26 records for the samples box. **What holds:** the ranges themselves, and the boxes holding them, so **nothing out of range ever reaches a pair** (A1d); and every in-range value's imposition.

---

# What this file found, what was repaired, and what is left

**Five findings were put to this document on 2026-08-23 and all five are repaired in place; three more arrived with that day's stage-close pass and are repaired below as C8 to C10; one more arrived with the re-gate of 2026-08-29 and is C11.** This section used to be headed *what the document does not carry* and to record C1 to C5 unfixed; each is replaced below by what changed. Nothing here is a new finding.

## C1. A7's command-channel negative — **repaired**, and it was a correction of a correction

A7 said, corrected 2026-08-20, that *"on the command channel nothing separates them by exit status at all"* and that a file test and a stat **on the path itself** both exit 1 in either case, routing the separation to a walk of the parent components. The evidence beside that sentence was a **macOS** run on a path **under an unreadable parent**. The stage's own frozen input document, corrected 2026-08-22 — after A7's correction was written — records the opposite for the case A7 and A10 actually read, the job output **folder itself** on the rented Linux box: the stat separates them 0 against 1 and the directory test does not ([M8](#m8-one-box-with-blender-on-it-5--fact-7s-two-halves-a7-a7a-a8)).

**The conclusion survives and is reached more cheaply than the wrong ground claimed.** Row 21's three-way distinction stands: the stat's exit status separates refusal from absence for the path itself, and the walk is needed only where the stat is itself refused, which is what an unreadable parent produces and which the Linux box never posed. A7 now carries the stat, A7a the walk and its bound, and A7b the correction with what did not survive — the claim that no test on the path separates them, which collapsed the directory test's negative into the stat's positive. A10 is reworded to cite both clauses. **Nothing is routed:** `File Browser` P3 is cited for the walk's shape and is unchanged, its own route being the subsystem's.

## C2. L23's asymmetry between two published lists — **repaired by stating it**

L23 rejects `preferences.get_device_types()` as a capability list and fills its AMD and Intel rows from the manual's `--cycles-device` option list. [M9](#m9-six-machines--fact-3s-accepted-names-and-the-two-distinguishable-refusals-l8-item-9-l22-l24) and [P5](#p5-blenders-command-line-arguments-page--the-accepted-device-names-l23) record that the second is no better a capability list than the first — printed identically for every platform, printed identically in both versions, and wrong about that build. **The clause now says so**: neither list is read as a capability list, the option list is taken for the **spelling** alone, and *hardware unposed* is the whole of what the two rows withhold. The table and the two values do not move.

## C3. L24's *default state of a machine nobody has set up* — **repaired by narrowing the ground**

The reading was taken from `--factory-startup`, the launch mode L9 refuses and the one that discards the saved preferences a plain run reads ([M13](#m13-the-multi-provider-batch--the-backend-selector-arrives-off-l21a-l24)). **L24 now claims the factory-default preference set and no more**, and states that Row 9's force does not rest on which set is in force, since the technology named at launch overrides both. **L21a carried the identical over-reach in its closing sentence** — *the observation of what they actually hold on a machine nobody has set up* — and is narrowed with it, so the document does not contradict itself. The residue is the document's [Section 9](../Rendering.md#9-unresolved) item 20 and [U16](#u16-what-a-fresh-rentals-own-saved-blender-preferences-hold-section-9-item-20).

## C4. Fact 8's 312 trials — **repaired**, the figure is 230 usable of 312 run

The tester discards the first 82 in as many words. All four places the document quoted 312 now quote both numbers ([M6](#m6-the-last-two-boxes-6--fact-8-in-full-s3-s9)). The conclusion is untouched: the three-millisecond transition and the never-returned *still in hand* are round three's, wholly inside the usable set.

## C5. The two quotations with no address — **repaired by reading the publications**

Both were read on 2026-08-23 and both match the document's words: the Open Group Issue 8 stdio page ([P1](#p1-the-open-group-base-specifications-issue-8--standard-stream-buffering-l7s-named-property)) and the Open Group's `test` page ([P2](#p2-the-open-groups-specification-for-test--the-three-exit-statuses-a7-a7a)). NVIDIA's installed-components list ([P3](#p3-nvidias-installed-components-list-for-the-linux-display-driver--libnvoptixso1-l23)) was read with them, its address having lived only under `Audits/` as well. **One difference of form:** P1's quotation was carried as two reordered fragments and is replaced by the publication's own single sentence. **One quotation was short rather than wrong:** P3's stops a sentence early, and the third sentence is restored here.

## C6. The phrase quoted from a Smoozy document — **repaired by removing it**

L23's struck Apple row quoted `The shape`'s *"every ordinary Linux rental"*, which is a condition on Fact 63's webhook route and not a statement of what this project rents. It is removed from the clause; the row now rests on [P6](#p6-blenders-gpu-rendering-page--which-system-each-technology-is-supported-on-l23s-struck-apple-row) for the Apple-Silicon-only half and on the document's own [Section 9](../Rendering.md#9-unresolved) item 13 for the Linux half.

## C7. The job identity's 128 bits — **routed, not repaired here**

S3 carries *"128 bits from the operating system's cryptographic random source, opaque, never reused"*. **`Queue & History` Y1–Y2 owns that claim, its source and its label**, and is being repaired separately. S3 states it by citation rather than independently, so nothing is decided or labelled in this document; the only change made was to name the owning document beside the markers, which a bare `Y1–Y2` left unrecoverable. **No second citation is taken here** and this file carries none.

## C8. The media type L10 named and no payload carries — **repaired by removing it**

L10's settings-pairs bullet listed *the media type where multi-layer output is asked for* among the values of `Render Settings`' payload that have no command-line argument. **`Render Settings` Y1's payload has nine entry kinds and neither a format nor a media type**, and its *what the payload never carries* list names neither, so a builder composing from this document would have looked for a payload entry that does not exist. **The [Render Settings spec](../../Specs/Render%20Settings.md) settles it rather than leaving it a gap**: *"Everything Smoozy does not put on this page comes from the file's own settings and is rendered as the artist saved it — the output format, denoising, and anything else the file carries that this page does not offer."* **So this contract sets no format at all**, and the ordering *Images 3* records — from 5.0 the media type is set before the format, and the format setting then takes only the thirteen still-image names, `OPEN_EXR_MULTILAYER` and `FFMPEG` not among them — bites a composition that sets a format and never this one. A file asking for multi-layer output keeps its own saved value, which is the mode `Job files and outputs` I7 already reads the inventory off. **L8's item 6 carried the same phantom** — *the format and ending arguments, where the settings payload requires them* — and is corrected with it, so the document does not name a payload value in one clause that another says does not exist. **Nothing is routed and no promise moves**; the entry above is [B5](#b5-the-values-that-have-no-command-line-argument-and-simplify-l10). **Raised by the `Render Settings` checker against this document, 2026-08-23.**

**One place is deliberately left alone:** the document's [Section 9](../Rendering.md#9-unresolved) item 18 names *a multi-layer media type* inside a passage the item preserves verbatim as the entry it carried before it was closed. It is a dated record of what was said, kept unchanged by that item's own rule.

## C9. L7's Creator's-Mac pipe run — **repaired by labelling it a demonstration**

L7 stated the run flat, as observed, and this file's [P1](#p1-the-open-group-base-specifications-issue-8--standard-stream-buffering-l7s-named-property) re-labelled it *observed in a test* rather than catching it. **No observation record for the run exists anywhere under `Preparation/`** — the finding is `Queue & History`'s, made on 2026-08-23, which named this clause and routed it here; the routing had not been carried. **L7 and P1 now both say what it is:** a demonstration with no record, not evidence at this stage's bar, with nothing resting on it. **The clause's conclusion is unchanged** — a pipe is not an interactive device, so the ordinary stream may hold what was printed until exit and the complaints stream does not — because the publication carries it alone. **No observation record was invented**, and the parked Blender-flushing question is untouched.

## C10. Three quotations the document had drifted on — **repaired from the sources**

Each differed from its source by a lowered capital or by stripped emphasis at a mid-sentence splice, and in two of the three **this file already carried the source's own form**, so the document was the copy that drifted. `--python-exit-code 63` at L8a and at [Section 9](../Rendering.md#9-unresolved) item 19 is restored to *"With"*, per [M18](#m18-render-settings-companion-sources-item-5--the-failure-to-exit-status-switch-l8a-section-9-item-19). A7b's bound is restored to *"The stat half holds for the folder itself and not in general"* and the splice moved outside the quotation marks, per [M8](#m8-one-box-with-blender-on-it-5--fact-7s-two-halves-a7-a7a-a8). S4a's line-order promise is restored to `Preparation/Specs/Queue & History.md`'s own emphasis — *"**A new job joins the end of its machine's line.** It goes behind everything already waiting there."* **No source has moved and no promise reads differently**; all three are character-exact against their sources today.

## C11. The companion did not carry A1d at all — **repaired by adding [B14](#b14-what-a-saved-scene-records-for-simplify-and-which-half-of-each-pair-a-job-sets-a1d-l10s-simplify-bullet-section-9-items-11-and-23) and narrowing [U11](#u11-simplifys-inferred-pair-section-9-item-11)**

**Raised by the re-gate checker against this file, 2026-08-29.** The commit that wrote A1d on Fact 134's answer did not touch this companion, with two consequences. **There was no Part 1 entry for *What a render demands 6* or Fact 134** — every other Blender-fact clause in the document has one — so A1d's observed claims had no record here and Simplify appeared only under [B5](#b5-the-values-that-have-no-command-line-argument-and-simplify-l10), which is about the delivery route. **And U11 still read the pre-narrowing text**, while the document's [Section 9](../Rendering.md#9-unresolved) item 11 had been narrowed the same day, so this file and the document said different things about the same label — the disagreement the gate exists to find. **B14 now carries the claims and their labels, U11 is narrowed to the launch route alone, and [U18](#u18-an-out-of-range-assignment-to-simplifys-two-typed-settings-section-9-item-23) carries the out-of-range residue.** **No claim in the document was unsupported and nothing in the document moved**; the citations were in A1d, in `Facts.md` Fact 134 and in `Render Settings` Q2 item 6 throughout. **No comparison question was opened for this entry** — it is a shape-1 pointer, taken from what the document and the fact register already carry.

## B25 — the launch line's order, and what the read-back returns (L8, L10, Section 9 item 1)

**The claim:** a setting given on the launch line is in force when a script handed in at the same launch runs, provided the setting comes after the file and before the script — and what the script reads back is the raw text handed in rather than a finished file name. Label: **observed in a test**.

**The joined answer [The map](../../Blender%20Research/Across%20topics/The%20map.md) names for this question is [What can be set from outside](../../Blender%20Research/Across%20topics/What%20can%20be%20set%20from%20outside.md), and the comparison question behind it is *Writing 13*. **No answer is assembled here.** **Three quotations, and each is attributed to the file it is actually in — corrected 2026-08-31 the same day, after a checker found two of the three named to the wrong document.** All three joined answers are on this stage's permitted reading list and every quotation is verbatim.

- *"The arrangement that works is the file, then the setting, then the script"* — [What a render really did](../../Blender%20Research/Across%20topics/What%20a%20render%20really%20did.md).
- *"A script handed in at the same launch sees whichever of the two was written before it, and the last of them on the line is the one the render uses"* — [Where a render's files go](../../Blender%20Research/Across%20topics/Where%20a%20render's%20files%20go.md).
- *"The value it reads back is the raw text handed in, template marks and all, so the comparison is against that text and not against a finished file name"* — [What can be set from outside](../../Blender%20Research/Across%20topics/What%20can%20be%20set%20from%20outside.md).

**Covers:** five arrangements, all four versions, two machines — the Creator's Mac and one rented Linux machine. **Does not cover:** Windows; and which scene an imposed output path lands on, which that document's own limit note states rests on one file holding two scenes.

**The full entry, with what breaks, is `Job files and outputs.sources.md` entry 73**, because D10 owns the read-back. This entry is a pointer, per the file's own rule above. *(Added 2026-08-31 by the Build stage's first sitting, when [Section 9](../Rendering.md#9-unresolved) item 1 closed.)*

---

*Written by Claude Opus 5, 2026-08-23. Three publications were read for this pass — the two Open Group pages and NVIDIA's component list, each named with its address and today's date above. Everything else is taken from `Preparation/Architecture/Rendering.md` itself or from elsewhere in this project, and where the project holds no source the entry says so.*

## C12. The third position on the launch line, and the device argument that sits in it — **added 2026-09-01, out of the repaired Blender research (L8, L24a, `Job files and outputs` D11)**

**The fact.** [What can be set from outside](../../Blender%20Research/Across%20topics/What%20can%20be%20set%20from%20outside.md): *"After a bare double dash the line is Cycles', and every Cycles argument — the render device among them — is read only there and nowhere else"* (*Settings 9*, *The machine 5*, *What a render demands 7*, *Settings 17*, *Reports 12*). And, on what happens to a name that position does not know: *"Written after the double dash it is not a failure at all: the line there is Cycles', an argument Cycles does not know is swallowed without a word, and the render goes ahead and ends at success — with a control proving arguments in that position genuinely are read"* (*Settings 17*, *Reports 12*).

**Label: observed in a test, with a control.** The control is what makes the negative usable: without it, a swallowed argument and an argument in a position nothing reads look identical.

**What it changed here.** L8's window had two edges — after the file, before the render instruction — and the line has **three**: the double dash closes it as surely as the render instruction does. And L24a: **L24's two loud refusals are refusals of the device *value*, and the argument's own *name* is checked by nothing in that position**, so a misspelling is the quiet processor render Row 9 exists to prevent, arriving by the one route L24 does not cover. **Nothing inside the launch material can catch it** — no read-back exists for a line Blender never parsed — so the check is at composition, and the argument's spelling is a per-version constant of the contract at one declaration point.

**What breaks if that constant is ever composed per job:** `Server Setup` W4 reads the same evidence and would blame the machine for Smoozy's own line, offering to re-run the ready action and restart it. That dependency is named at both ends.

## C13. Whether a stopped render reaches the line naming what did the work — **added 2026-09-01; the answer reverses what L26 said**

**The fact.** *What actually ran 6*, repaired: *"Whether a stopped render ever reaches that line was reasoned from the timings of a run that finished; no stopped render was ever run for it."* ***Frames 15*** ran one. **After a stop, 5.0 and 5.1 give the whole closing block with the device line in it, 5.2 prints `Rendering was canceled.` and no block, and 4.5 prints no report at any level** — and in the runs that printed, the frame's own sampling had completed inside the wait, so **what a frame genuinely cut short prints is recorded nowhere.** **And 5.2's own published fix list calls that block on exit fake, with wrong numbers** — the 5.0 run bears it out, its block reporting 600 samples for a frame the line above it calls 599.

**Label: observed in a test**, with that version's own publication behind the last part.

**What it changed here.** L26 said *a stopped render never reaches it*, and that sentence was the ground two other documents rested a machine-restarting act on. It is struck. **The observation is unchanged — the line is still watched for as the bytes pass — and the *reading* of it is now gated on the job's recorded outcome being *finished***, which is `Queue & History` E12's rule, repaired on 2026-08-31 on this same evidence. `Server Setup` W4 is the one act that turns on it. **This document states no second gate.**


## C14. A handler registered before the file, and the two places only it can set — **added 2026-10-02 (L8, L8b)**

**The fact.** A geometry-nodes bake folder set by the launch material after the file has loaded is ignored, and the folder the file recorded is read — even with the modifier switched off and on or the object retagged; set by a `load_post` handler registered from a script placed before the file, it is read, identical to the plain render (4.5.12 and 5.2.2). A Replay fluid cache writes frame 1's files beside the `.blend` the moment the file opens, before any script runs; with its folder set after load, frame 1 was still written there; set by that handler, nothing was written beside the `.blend` (4.5.12 only).

**Label: observed in a test**, at the desk, 2026-10-02, during the Build stage, on one Mac, from the command line with no window, on copies. **Not the Blender research.** **Sources:** `Audits/Build/Step 10c/The simulation fixes — what each piece is, which files it owns, and in what order.md`, *Five things worth carrying past the table*, item 1, with the two probe scripts it names, `after.py` and `before.py`; and `Audits/Build/Step 10c/Does the redirect cut the render off from bakes — what was seen.md`, where a geometry-nodes copy placed where the material pointed was not read.

**Not covered.** ~~What a raise inside the handler does to the run — L8b writes it as an assumption and reads the handler's running from the material instead.~~ *(Covered since 2026-10-02 — see below.)* ~~Blender 5.0 and 5.1.~~ *(5.0 and 5.1 covered for the raise alone.)* A rented machine. Whether the handler moves L8's window is not something this test varied; L8b's reason that it does not is that it sets nothing before the load.

**Added 2026-10-02 by simulation fix piece 4 — what a raise inside the handler does.** A script placed before the file registered a kept `load_post` handler that marked a flag in Python's own list of loaded modules and then raised; the line carried `--python-exit-code 63`, the file was a factory cube saved by 4.5.12, and one frame was asked for. **In 4.5.12, 5.0.1, 5.1.2 and 5.2.2 alike**: Blender printed *"Error in bpy.app.handlers.load_post[2]:"* and the traceback, loaded the file, ran the script after the file — which read the flag the handler had set before raising — wrote the frame and **ended at 0**. With the script after the file raising where the handler had not marked itself finished, **the run ended at 63 with no frame written**, in all four. And through Smoozy's own composed line, in 5.2.2: the script before the file left off, the material's check raised and the run ended at 63 with no frame; a handler whose table did not hold the domain's recorded folder raised *"no job folder for the recorded location"*, the run went on, and the material's check ended it at 63 with no frame. **Label: observed in a test**, at the desk, from the command line with no window, on copies. **Not the Blender research.** **Source:** `Audits/Build/Step 10c/Simulation fix piece 4 — what landed.md`, section 1, and the probe's own record in the builder's scratch, `sim-fix-4-5/probes/raise/RESULT.txt`.

## C15. Stepping a simulation never baked from the scene's start, and what it was seen to give — **added 2026-10-02 (L10, L10b, S13)**

**The fact.** In one Blender run, a script called `frame_set` for each frame in turn from the scene's start and rendered each frame of the range by a render call as it was reached; references were made one run per frame, stepping 1 to N and rendering N. **`control_sim_zone`**, an unbaked geometry-nodes simulation zone: 24 of 24 frames identical for 1–24, and 13 of 13 for 12–24 with 1–11 stepped and not rendered; stepping took 0.00 s; the whole run 2.0–2.1 s against 2.1 s for a plain `--render-frame` of the same list. Rendered as one animation (`-s 1 -e 24 -a`, `-s 12 -e 24 -a`) and through Smoozy's own list, every frame was frame 1's picture (`control_sim_zone` from part-way: the stepped frame 1's picture throughout). **`made-kind-4-smoke-replay`**, an unbaked Replay smoke: stepped and rendered as reached, 1–2 identical to the references and drifting to 4.9% of pixels off by frame 24; with 1–11 stepped first, 5.3–9% off; three stepped ways, 5–13% apart; stepping 0.8–0.9 s for 24 frames; 48 cache files written beside the `.blend` with no folder set. **Smoozy's own frame 12 of that smoke, before simulation fix piece 4, was identical to the 1–11-then-12–24 run's frame 12**, the render having read the cache the reading at the render's start had stepped beside the `.blend`.

**Label: observed in a test**, at the desk, 2026-10-02, during the Build stage, on one Mac, from the command line with no window, on fresh copies, in 4.5.12 and 5.2.2. **Not the Blender research.** **Sources:** `Audits/Build/Step 10c/Simulation fix piece 5 — what landed.md`, sections 7 and 8, with the probe scripts `tools/step_and_render.py` and `tools/step_probe.sh` in the runner's scratch `sim-fix-4-5/`; and `Audits/Build/Step 10c/Simulation files at the desk — what was seen.md` for the frozen command-line render in 4.5.12, 5.0.1, 5.1.2 and 5.2.2.

**Not covered.** **L10b's own shape — every frame stepped in the material, then the command-line render of the list — was never run**; the probe rendered from a script. Unbaked physics (rigid body, cloth, particles) stepped at all. 5.0 and 5.1 for the stepping. A real scene's stepping time or memory. A rented machine.

**Added 2026-10-02 by the machine side of decisions A and B — L10b's own shape, run and then built.** Source: `Audits/Build/Step 10c/Decision A — what landed.md`, sections 1, 4 and 5. Label: **observed in a test**, at the desk, from the command line with no window, on fresh copies, in 4.5.12, 5.0.1, 5.1.2 and 5.2.2.
- **Before any code.** Smoozy's own composed line had a script placed after the launch material and before `--render-frame`. The script called `frame_set` from the scene's start to the list's last frame, and rendered nothing.
  - `control_sim_zone`, 1–24 and 12–24: 24 of 24 and 13 of 13 frames identical to the stepped references, in all four. Unstepped, only frame 1 matched.
  - `made-kind-4-smoke-replay`, 1–24: it moved, 48 cache files went to the job folder and none went beside the `.blend`.
  - `made-kind-4-cloth-unbaked`, 12–24: identical to its own 1–24.
  - `compound_collision_chain` (an unbaked rigid-body chain, with motion blur), 12–24: identical in 4.5.12, and within 1/255 in 5.2.2, to a reference that steps 1–24 and then renders each frame. Unstepped, 20–33% of pixels were off. **A plain `-s 1 -e 24 -a` render of the chain does not move it as stepping does**, so it is no reference.
- **As built.** The material's own `Step` instruction gave the same results through Smoozy's line, in all four.
  - The chain's 5.0.1 and 5.1.2 frames came within 7/255 of the probe's, on at most 0.14% of pixels.
  - A re-run list of 5, 9 and 17 stepped 1–17, and its three frames were identical to the references.
  - A factory cube and `bake_load` were not stepped.
- **What it cost on a larger scene**: `mesh_fracturing`, a geometry-nodes simulation over 250 frames, Cycles, with a camera added.
  - **Stepping 1 to 250 took 9.25 s in 5.2.2 and 9.55 s in 4.5.12**, about 0.04 s a frame. Through Smoozy's line, the whole run up to the render instruction peaked at 572 MB.
  - Stepped and then rendered (248–250), the run peaked at 3,837 MB in 5.2.2 and 2,402 MB in 4.5.12. Rendered plainly it peaked at 3,258 MB and 2,114 MB.
  - **The plain render is not like for like.** Unstepped, it draws the unsimulated object, 38% of pixels away from the stepped frame 250. The two render times (100 s and 334 s in 5.2.2) were taken under different load and are not compared.

**Not covered.** A rented machine. Soft body, dynamic paint and particle simulations. A Replay liquid. A scene whose `frame_start` is later than the list's lowest frame.

## C16. The 5.3.0 Alpha, read at the desk beside 5.2.2 — **added 2026-10-02 (L8, L8a, L10, L24a, L26)**

**The fact.** Every launch argument, switch and property name the composition and the launch material use reads the same in the 5.3.0 Alpha as in 5.2.2. So 5.3 joins the versions the contract is composed for, with its own cited arm for the device argument and the device-line ask.

**What was run.** The Creator's own install, `/Applications/Blender 5.3.0a.app`, answered ***5.3.0 Alpha***, build hash **`68609be8e23b`**, built 2026-10-01. It was run beside `/Applications/Blender.app`, which answered *5.2.2 LTS*, hash `d13f752e3b9c`. Both were run with no window, one at a time, and nothing of the Creator's was opened, written or saved. Each ran four things:
- **Its help text.** The two are word for word the same, apart from the version line and one argument added in 5.3 (`--debug-gpu-backend-no-fallback`), which this contract does not use. The text includes `--background`, `--python-exit-code`, `-S`, `--engine`, `--render-output`, `--python`, `--render-frame`, `--log-level`, and `--cycles-device` with the same six values, read only after a bare `--`.
- **Its whole property lists** for every structure the composition and the launch material reach: the scene, its render, Cycles and EEVEE settings, the view layer, objects and the modifiers the material walks, point caches, the sequencer, actions down to their curves, and the file output node. **Nothing the composition uses was removed or changed in 5.3.** The nine lines 5.2.2 has and 5.3 does not are two axis defaults, a modifier's panel list, and node icons and link kinds. 5.3 only adds names, among them an EEVEE time limit this contract does not set.
- **The composed line's shape** on the factory scene: `--background --python-exit-code 63 --log-level info -S Scene --engine CYCLES --render-output … --python … --render-frame 1,3 -- --cycles-device CPU`. Both exited 0 and wrote both frames. The device line (`Path tracing on: … [CPU]`), the frame announcements and the save announcements came out on standard output in the same shape in both.
- **A script that raises**, with `--python-exit-code 63`. Both exited 63 and wrote no frame.

**Label: observed in a test**, at the desk, 2026-10-02, during the Build stage, on one Mac (macOS on Apple silicon), from the command line with no window. **Not the Blender research.** **Source:** `Audits/Build/Step 10c/The fixes after the Vast pass before 0.1.9 — what landed.md`, item 3.

**Not covered.** Linux, where Smoozy renders. A card. A real scene. Whether the alpha behaves the same once it is rebuilt: the build service replaces an alpha under the same version, and this reading is of build `68609be8e23b` alone.

## C17. Blender's stated OptiX driver minimum per version — **added 2026-10-11 (L23a, Section 9 item 3)**

**The fact.** Blender states an OptiX driver minimum per version: 535 for 4.5 to 5.1, 570 for 5.2.0, 575 for 5.2.1 and later. **Label: documented.** Every page below was read on 2026-10-11, by the check recorded at `Audits/Build/Step 10c/The OptiX driver claims, checked against Blender's own pages.md`.

- **4.5 to 5.1, 535.** `https://developer.blender.org/docs/release_notes/4.5/cycles/`: *"The minimum driver version required for OptiX has increased to 535."* The manual for 4.5, 5.0 and 5.1 (`https://docs.blender.org/manual/en/<version>/render/cycles/gpu_rendering.html`): *"at least 535"*.
- **5.2.0, 570.** The manual's fix of 2026-08-17, commit `ec379305f1` in `projects.blender.org/blender/blender-manual`: *"It was already wrong, previously was 570."* The build settings, `build_files/config/pipeline_config.yaml` at tag v5.2.0 (2026-07-13), name CUDA 12.8.0.
- **5.2.1 and later, 575.** `https://developer.blender.org/docs/release_notes/5.2/cycles/`: *"The minimum driver version for OptiX is now 575."* `https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html`, and the `/latest/` and `/dev/` pages: *"a driver version of at least 575"*. Commit `b13ba36f4649497f` (2026-08-16), *"Use CUDA 12.9.1 instead of 12.8.0"*: *"The minimum driver version requirement (mainly for OptiX OSL) is raised from 570 to 575."*; on the 5.2 branch it is `4d16b97d19`, and the build settings name CUDA 12.9.1 at tags v5.2.1 (2026-08-24) and v5.2.2 (2026-09-14).

**The project's copy is older than the live page.** `Blender Sources/manual_5.2/render/cycles/gpu_rendering.txt` line 33 reads *"a driver version of at least 535"*; the manual changed it on 2026-08-17.

**Observed beside it, and not the source of the bar.** On Blender 5.2.2, 550.163.01 failed every OptiX render and 570.124.06, 570.195.03, 570.211.01 and 595.91.07 rendered, on Hyperstack RTX A6000 machines and RunPod RTX 4090 pods. **Label: observed in a test**, 2026-10-11, at Build. **Source:** `Audits/Build/Step 10c/Making OptiX work — what we found.md`, finding 1.

## C18. The 5.4.0 Alpha, read at the desk beside the 5.3.0 Beta — **added 2026-10-18 (L8, L8a, L10, L24a, L26)**

**The fact.** Every launch argument, switch and property name the composition, the launch material and `Job files and outputs`' reading script use reads the same in the 5.4.0 Alpha as in the 5.3.0 Beta. So 5.4 joins the versions the contract is composed for (`THE_5_4_LOOKUP` in `App/Rendering — machine/src/spellings.rs`), with its own cited arm for the device argument and the device-line ask.

**What was run.** One 5.4.0 Alpha for macOS on Apple silicon, from Blender's build service (`https://builder.blender.org/download/daily/`): `blender-5.4.0-alpha+main.3f4216e1bad1-darwin.arm64-release.dmg`, 358,447,738 bytes, SHA-256 `cd81ff76447f057f799670156a3d445452072448fe1fedfe66b1ff782234200a` (matching the service's own checksum file). Copied to `/Applications/Blender 5.4a.app`, it answered ***5.4.0 Alpha***, build hash **`3f4216e1bad1`**, built 2026-10-18 (commit date 2026-10-17). It was run beside `/Applications/Blender 5.3b.app`, which answered *5.3.0 Beta*, hash `5d8151335b6b`. Both were run with no window, from factory settings, one at a time, and nothing of the Creator's was opened, written or saved. Five things were run:
- **The help text.** Word for word the same in both, apart from the version line. `--cycles-device` takes the same six values after a bare `--`.
- **The whole property lists of every structure each Blender registers**, Cycles' own settings included (4,121 in 5.3, 4,123 in 5.4). **Nothing was removed or changed.** 5.4 adds two menus, one mesh property (`use_radial_symmetry`) and two icon names. Every one of the 23 scene paths the composition assigns, reads back or names, and every Blender name the launch material, the script before the file, the remap and the reading script use, read the same.
- **A real render through the render worker's own driver** (`a_real_blender_renders_two_view_layers_on_the_processor_through_the_real_line`, `App/Rendering — machine/src/acting/pipeline.rs`): the reading taken by the alpha, the material composed from it, placed and read back, and the line's own words. A stand-in saved by the alpha with two rendering view layers, Cycles on the processor, frames 1 and 2, as a multi-layer OpenEXR. It exited 0 and wrote both frames, each holding both layers. The device line (`Path tracing on: … [CPU]`), the frame announcements and the save announcements came out on standard output in the same shape as 5.3's on the same file, and the two readings were the same apart from the version.
- **The device argument after a bare `--`.** On the same file, saved for the processor, `--cycles-device METAL` moved the render to the Mac's graphics (`[METAL_Apple M1 Pro (GPU - 14 cores)]`) in both, and `CPU` kept it on the processor. So the argument is read, not dropped.
- **A script that raises**, with `--python-exit-code 63`. Both exited 63 and wrote no frame.

**Label: observed in a test**, at the desk, 2026-10-18, during the Build stage, on one Mac (macOS on Apple silicon), from the command line with no window. **Not the Blender research.** **Source:** `Audits/Build/Step 10c/The thirteenth notes, piece F2 — what landed.md`.

**Not covered.** Linux, where Smoozy renders. A card other than the Mac's own graphics. A real scene. Whether the alpha behaves the same once rebuilt: the build service replaces an alpha under the same version, and this reading is of build `3f4216e1bad1` alone.

## C19. A scene set to write a video, switched to a still picture for one render — **added 2026-10-27 (L10, L10c, S13)**

A factory scene set to Cycles on the processor, 64 by 48 at 100 per cent, MPEG-4 and H.264, range 1 to 10, saved; opened again; its output switched to PNG (from 5.0 the media type set to picture first) and its range narrowed to frame 1; rendered as an animation over that one frame. It wrote `v_0001.png` and nothing else, 64 by 48. The same with frame 7 rendered alone wrote `v_0007.png`. The control, the same file left as a video and narrowed to frame 1, wrote `v_0001-0001.mp4`. The file on disk still read as a video afterwards. A second scene at 50 per cent set to carry transparency (QuickTime, PNG codec, RGBA) wrote a 32 by 24 picture with four channels. **From 5.0 the format follows the media type**: the picture format set alone was refused (`enum "PNG" not found in ('FFMPEG')`), and the media type set to `IMAGE` made the format PNG at once; 4.5 has no media type and took `PNG` straight. `is_movie_format` read false after the switch in both.

**Label: observed in a test**, 2026-10-27, during the Build stage, in a cloud container on Linux with no window — **not the Blender research.** Blender 4.5.15 LTS (`5afeabba6097`) and 5.2.3 LTS (`e13b728c6880`), the Blender Foundation's own builds of Blender as a Python module, from the Python package index, because the session could not reach `download.blender.org`. **Source:** `Audits/Build/Step 10c/The video wants — what landed.md`, *The Blender test*, which carries both scripts and their output.

**Not covered.** 5.0, 5.1, 5.3 and 5.4. The command-line render Smoozy uses: the test rendered from Python, so that a value the material sets reaches a command-line render rests on *What can be set from outside*. One video container and codec, and one other for transparency.
