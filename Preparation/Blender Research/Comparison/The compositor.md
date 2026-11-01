# The compositor — Blender 4.5, 5.0, 5.1, 5.2 and 5.3 side by side

What makes a file's compositor run, what it belongs to, what a render becomes once it reaches it, how far it can be followed through it, what decides the size of what gets written, and what a scene pulled in through it is rendered with.

## The machines behind the tests

Three computers were used, and their results are three results, never merged. The equipment section of `Preparation/Blender Research/Method.md` holds every machine in full.

- **The Creator's own Mac** — a MacBook Pro with an M1 Pro chip, Apple's own graphics (Metal) and no second card; the only one of the three with a screen of its own.
- **A rented Linux machine with an NVIDIA GeForce RTX 4090** — the **first** rented Linux machine in the equipment section. It has a display, but a shared one, so a look that needs the pointer held still or a button pressed does not reliably reach Blender there.
- **A second rented Linux machine with an NVIDIA GeForce RTX 4080 SUPER** — the **fourth** in that section, with no display server at all.

For Blender 4.5 to 5.2, questions 1 to 7 were run on the Mac and on the 4090 machine; question 8 on the Mac and on the 4080 SUPER machine; **question 9 on the Mac alone**. **Blender 5.3 was run on the Mac alone, for every question**, on macOS 26.6.2 with 16 GB of memory. Its window looks were made by a script driving Blender's own window with a simulated pointer and simulated keys, read off Blender's own screenshots; nobody touched the Mac. Where 4.5 to 5.2's window looks on the Mac were taken by a person — hovering among them — 5.3's were not, and the body says where that difference in method matters.

For questions 1 to 8 all three machines carried the same four Blenders — 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.0 LTS. The two rented machines have different cards and different processors, so **no figure from one may be set beside a figure from the other**. Both were shared with other topics' tests throughout, so nothing here says how long anything took or how much memory anything used. The four Blenders report the same four build fingerprints on the Mac and on the 4080 SUPER machine, so those are the same four builds.

**The Mac's 5.2 is not certainly the same build behind every question.** Question 9 was run there against **5.2.1 LTS**; the earlier questions were run against 5.2.0. So a 5.2 figure from question 9 and a 5.2 figure from an earlier question may not be from the same build of Blender, and a difference between them would have to allow for that before it could be read as anything else.

**Every 5.3 answer is from Blender 5.3.0 Beta, built 2026-10-16.** A beta can change before it is released, and nothing here says the released 5.3 behaves the same.

A test result settles only the machines it ran on. **Windows, other makes of graphics card and machines holding more than one card were not tested.** Uniform results on the tested machines do not settle those cases. **Every 5.3 test rests on the Mac alone**, as do this topic's question 9 tests. Where a 5.3 result from the Mac sits beside an earlier version's result from a rented machine, agreement is agreement across machines; disagreement does not establish a version difference. Documented changes are labelled separately. Question 9 names one earlier result also established in another topic on Linux.

---

## The answer in short

### Where Blender itself differs between versions

- **1** — the Use Nodes switch is a real condition in 4.5 and inert from 5.0.
- **1** — the final output node is called Composite in 4.5 and Group Output from 5.0.
- **1** — muting that node stops the compositor's result reaching the file in 4.5 and does not from 5.0.
- **1** — the Enable Output node exists from 5.0 and not in 4.5.
- **1** — the Compositor editor's header holds a Use Nodes checkbox in 4.5 and a tree chooser from 5.0.
- **1 and 5** — a second output node is marked by a red header in 4.5 and an "Unused Output" warning from 5.0.
- **1** — the compositor device starts at CPU in 4.5, 5.0 and 5.1 and at GPU in 5.2 and 5.3.
- **1** — at 9000 pixels wide, with the compositor running on the graphics card, 5.1 alone wrote a black frame.
- **1 and 3** — 5.3 replaces the single compositor group with an effects stack (documented change; its render behaviour observed on the Mac).
- **1 and 5** — 5.3 changes the default tree to Group Input → Group Output (documented and observed on the Mac).
- **1** — 5.3 enables the viewport compositor by default in Layout (documented; not tested).
- **2** — a Save Output switch exists in 5.2 and 5.3 and in none of 4.5, 5.0 and 5.1, and it defeats even a path handed in on the command line.
- **2** — an emptied File Path writes into the working folder in 4.5 and fails outright in 5.0, 5.1, 5.2 and 5.3.
- **2** — the Output panel gained a Media Type row from 5.0.
- **3** — in 4.5 a compositing tree cannot be shared between two scenes and the attempt is refused; from 5.0 it can be, and a shared tree follows the scene named inside it rather than the scene being rendered.
- **5** — on a fresh scene, Render Layers offers the full pass list in 4.5 and 5.0, and only Image and Alpha in 5.1 to 5.3; enabled passes are qualified under question 5.
- **5** — Render Layers uses longer pass-socket names from 5.0; question 5 gives examples.
- **5** — the live output node can be recognised from the file from 5.0 and not in 4.5.
- **5** — hovering a link shows the socket's name with a trailing full stop in 4.5 and without one in 5.0 and 5.1, and a fuller display on a Render Layers node in 5.2. Where 5.3 stands is not established.
- **5** — the Split node's inputs changed at 5.0.
- **6** — a File Output node fed a picture scaled to 120 pixels wide writes 120 in 4.5 and 5.0 and 122 in 5.1, 5.2 and 5.3.
- **7** — the Sequencer Scene, its Render menu entries and its render-command flag exist from 5.0 and not in 4.5.
- **7** — a multi-layer OpenEXR carries an extra pixel-aspect channel per layer in 5.0, 5.1 and 5.2, and not in 4.5. Whether 5.3's does was not read.
- **8** — the Split node lands its first input on the right of the frame in 4.5 and on the left from 5.0.
- **8** — muting a node marks its unused sockets as having no effect in 4.5 and marks nothing from 5.0.
- **8** — a fresh File Output node starts on PNG in 4.5 and Multi-Layer OpenEXR from 5.0.
- **8** — its file-name setting starts at `file_name` in 5.0 and 5.1, and at `{blend_name}` in 5.2 and 5.3.
- **8** — 5.1 adds custom metadata reading and writing to Image and File Output nodes (release note; metadata transfer untested).
- **8** — a compositor tree accepts the general-purpose Switch type from 5.2; 4.5 to 5.1 refuse it. The Add menu is a separate open point.
- **8** — with no window an empty Viewer image exists in 4.5, 5.1, 5.2 and 5.3 and does not exist at all in 5.0.
- **8** — a render made with a window fills the Viewer node's image in 4.5 to 5.2 and does not in 5.3. Blender 5.3's release notes state the change.
- **8** — where 4.5, 5.0 and 5.1 hand one input back exactly at a Mix's full factor and out of Depth Combine, 5.2 comes back one step of a floating-point number short of exact. Where 5.3 stands is not established.
- **8** — a Mix node mixes the two pictures' transparency as well as their colours in 5.1, 5.2 and 5.3 and only their colours in 4.5 and 5.0.
- **8** — a freshly added Mix node starts with its factor at 1.0 in 5.2 and 5.3 and at 0.5 in the other three.
- **8** — a compositor tree accepts 187 node types in 5.3 and 168 in 5.2.
- **9** — EEVEE honours a view layer's sample override from 5.0 and ignores it in 4.5, through the node and directly.
- **9** — a render's log names the engine it is using from 5.0, and names it nowhere in 4.5.
- **9** — the name EEVEE answers to on the command line is `BLENDER_EEVEE_NEXT` in 4.5 and `BLENDER_EEVEE` in the other four.
- **9** — a refused engine name, and the engine listing itself, end 4.5 in an abort where they end the other four cleanly; how 4.5 fails there may be the operating system's doing rather than Blender's, and the body says so.

Except where labelled documented, these differences are **observed**. Questions 1 to 7's tests among 4.5 to 5.2 ran on the Mac and the 4090 machine; hovering rests on the Mac alone. Question 8's tests among those four ran on the 4080 SUPER machine, with Mac counterparts for the Viewer results. Question 9's tests ran on the Mac alone. **5.3's observations all rest on the Mac.** Its new differences rest on release notes, on matching tests on the Mac in both versions, or on both. Agreement with an earlier Linux result does not extend a 5.3 observation to Linux.

### What is not established

These are open points or limits on the answers. Each question gives the full result and its limits.

- **1** — whether a Warning node exists in every version.
- **1 and 2** — whether the different files written without a final output node mark a version change.
- **1 and 6** — the GPU compositor's full frame above 8k in 4.5, 5.0 and 5.2; the first-pixel tests do not settle it.
- **1 and 8** — whether 5.3's GPU setting actually used the Mac's graphics.
- **1** — File Output writing during a still render not asked to save, in 4.5 to 5.2.
- **1 and 2** — the effects of the render engine's compositing and image-saving flags, in all five.
- **2** — 5.2's Save Output tooltip, and its off-switch behaviour for a single frame or with no File Output node.
- **3** — different Compositing switches on scenes sharing a group, in 5.0 to 5.2.
- **4** — Scene-strip carry-over within one Blender session across every version.
- **4** — the scope of 5.3's documented change to scene references inside nested groups.
- **5** — Group Input as the render's entry in 4.5 to 5.2, and enabled-pass sockets in 5.3.
- **5** — 5.3's precision; its manual and API disagree.
- **5 and 8** — whether the picture data itself carries an origin marker; none was found in the inspected properties.
- **5 and 8** — 5.3's fuller socket display and its comparison with the earlier hand-hover results; hovering on Linux.
- **1, 5 and 6** — header controls and Crop to Render Region's availability beyond the machines named under those questions.
- **6** — the extra Scale and supplied-picture sizes, Crop node, pixel aspect, Burn Into Image, odd H.264 dimensions and crop placement in 4.5 to 5.2.
- **1 and 8** — exact agreement between compositor devices beyond the tested trees and machines.
- **7** — conditional Sequencer Scene menu entries in 5.0 to 5.2, and selection without a window there in this topic's runs; Settings 12 holds that test.
- **7** — whether the scene's own compositor runs under a strip in 4.5 to 5.2.
- **7** — the pixel-aspect entry in 5.3's multi-layer OpenEXR.
- **8** — the Mac half of thirteen terminal tests in 4.5 to 5.2; every 5.3 Linux counterpart.
- **8** — whether 5.3 changes 5.2's one-step rounding result; the two were tested on different machines.
- **8** — which Switch the Add menu adds in 4.5 to 5.2, and which node their manual page describes.
- **8** — a file-name token for the socket's name, metadata passing through the compositor, and a metadata property on 5.3's image data block.
- **9** — the re-render button's samples in 5.3, and its engine and samples in 4.5 to 5.2.
- **9** — results beyond macOS, apart from the earlier EEVEE override result qualified there.
- **9** — EEVEE without a display on Windows, third-party engines, the GPU compositor, Workbench samples, adaptive sampling, time limits and sample subsets.
- **All questions** — Windows, untested makes of graphics card, multiple cards, and every 5.3 test result beyond macOS on Apple silicon.

### Where only the documentation differs

- **1 and 2** — only 5.1 names the two render-engine flags as conditions; all five expose them.
- **1 and 5** — 5.1 and 5.2 describe a header control for choosing the live output; the observed headers have none.
- **1** — 5.3's API defaults for an effect's switches disagree with their observed starting values.
- **2** — 5.3's Output-panel picture omits the checkbox its text and program have.
- **5** — 4.5's Split-node page says it has no outputs; the node has one.
- **6** — only 5.2 and 5.3 quote the 8k limit; every tested version wrote the full size. Content is qualified under question 1.
- **8** — only 5.2 and 5.3 name socket inspection; all five lack a script property for it.
- **8** — only 5.2 quotes which end of a Mix factor gives which input; the tests agree.
- **8** — 5.0 to 5.2's Alpha Over pictures repeat one input name where their text and program use two.

Question 4's basic answer agrees across versions; the nested-group change remains open there. Question 9's documented EEVEE override difference accompanies a real behaviour difference and belongs in the first list.

---

## 1. What decides whether a file's compositor runs at all?

**How it is known.** Each version's documents name their conditions as documented, and every one of them marks the summary — that the conditions form one complete list — as inferred. Everything decided below was settled by hand test, some from a terminal and some in a Blender window. 4.5 to 5.2 were run on the Creator's Mac and then again on the rented Linux machine; every figure agreed on the two machines, and the two window looks the Mac left half-answered came back answered on the rented machine. 5.3 was run on the Mac alone.

**The same in all five.** Three things govern what lands in the scene's own file.

- The **Compositing** switch in Output ‣ Post Processing. On, the compositor's result is what lands in the file; off, the raw render is.
- **A compositing tree must be present.** With none attached, the raw render is written. In 5.3, where a scene holds a stack of effects, an empty stack, an effect with no node group, and every effect switched off for render each leave the raw render.
- **That tree must hold a live final output node** for the compositor's result to reach the scene's own file. Without one, the compositor's result does not reach it — **but the compositor is not skipped.** With no final output node in the tree at all, a File Output node in the same tree goes on writing, in 4.5 to 5.2 on both machines (**File output nodes 13**) and in 5.3 on the Mac. What the missing output node stops is the compositor's result reaching the scene's own file, and nothing else. What lands there instead is not the same in every run: 4.5 to 5.2 wrote the raw render; 5.3 wrote a transparent black frame, over a black raw render and over a green one alike. Whether that is a change in Blender is not established — a Blender 5.2.2 control on the same Mac, on 5.3's scene, wrote the same transparent frame as 5.3, so the split may lie in the scene the earlier runs used.

Three further results came back the same in all five. A tree with no Render Layers node in it still runs, and whatever it makes is what lands — a plain red colour fed to the output wrote a red frame. The Post Processing panel holds exactly two checkboxes, Compositing and Sequencer, both on by default, plus a Dither slider, and nothing else; in 5.3 the two checkboxes sit in a row labelled Pipeline. And the two flags a render engine declares — one for applying compositing, one for saving images during an animation — are present on the render-engine type in all five versions. Only 5.1 documents them as conditions. Their presence was observed; neither flag's effect was tested.

**Where it differs, version by version.**

**Blender 4.5.**
- The old **Use Nodes** switch is a real condition here. It starts off in a new file, and turning it off with everything else in place leaves the raw render in the file.
- The final output node is called **Composite**.
- **Muting that node stops the compositor's result reaching the file** — the raw render is written instead.
- There is **no Enable Output node**; the node type does not exist.
- The Compositor editor's header carries a **Use Nodes checkbox**, off by default, with the Add menu greyed out until it is ticked. Ticking it builds the standard tree. Under the header sits a fixed path reading "Scene > Compositing Node Tree".
- With two output nodes in one tree, the live one is marked by a **dark red header** and the other is grey. The last node clicked is the live one.
- The compositor device starts at **CPU**.

**Blender 5.0.**
- Use Nodes decides nothing. Setting it off does not even take — it reads back on — and the compositor runs anyway. Blender 5.0's release notes say the switch was removed from the node editor.
- The final output node is called **Group Output**. The Composite node type does not exist and cannot be added.
- **Muting the final output node makes no difference** — the compositor's result is still written.
- The **Enable Output node exists**. With one in the tree and its Enable input turned off, the written frame came out black: the compositor ran and delivered nothing, rather than being skipped.
- The Compositor editor's header has no checkbox. Instead it offers a node-tree sub-type dropdown with two entries, Scene and Sequencer, plus an ordinary data-block selector and a New button. Nothing exists until New is clicked.
- With two output nodes, the one that is not live carries a warning above its header reading **"Unused Output"**.
- The compositor device starts at **CPU**.

**Blender 5.1.** The same as 5.0 on every point above. One result is 5.1's alone: at 9000 pixels wide, with the compositor device set to GPU, **5.1 wrote a black frame** where 4.5, 5.0, 5.2 and 5.3 all wrote the compositor's result at the first pixel. On the CPU device all five versions wrote the compositor's result.

**This one came back on both machines,** and that matters. On the Mac the run was repeated three times on Apple's own graphics with the same outcome. On the rented machine it was repeated twice on an NVIDIA card, with the processor's frame correct in the same runs. Two operating systems and two different makes of graphics card give the same answer, so this is a difference between **versions of Blender**, not between machines. 5.3, run the same way on the same Mac, did not write a black frame.

One thing about how the rented machine's run went: the script that sets both devices inside one run of Blender **crashed Blender 5.1 outright**, after writing the processor's frame and before finishing the card's. It did not do that on every run, and **how often was deliberately not established** — nobody counted, and nothing here should be read as saying how likely it is. The black frame itself is not in doubt; it was reached again by giving each device a run of Blender to itself. No other version crashed.

**Blender 5.2.** The same as 5.0 on every point above, except that the compositor device starts at **GPU** rather than CPU, and the 9000-pixel frame came back with the compositor's result at its first pixel on both devices, on both machines.

**Blender 5.3.** The same as 5.2 on every point above, on the Mac: Use Nodes reads back on whatever it is set to and decides nothing; the output node is Group Output and the Composite type does not exist; muting it makes no difference; the Enable Output node exists and, with Enable off, the frame came out black; the editor's header has no Use Nodes checkbox; the node that is not live carries "Unused Output"; the device starts at GPU; and the 9000-pixel frame's first pixel came back composited on both devices. What 5.3 adds:

- **A scene's compositor is a stack of effects rather than one node group.** Blender 5.3's release notes state the change. Each effect points at a node group of its own and has its own render switch and preview switch. A fresh file holds no effect and no node group.
- **The older one-group pointer is still there**, with Blender warning that it is to be removed in 6.0. Giving a scene a group through it shows up as an effect called Scene Effect; giving an effect a group shows up through it.
- **A newly made effect starts with both switches on**, whether made from a script or by the New button. The API page says they start off; the program and that page disagree.
- **An effect's render switch decides whether it takes part.** Off, the raw render was written to the last figure; its preview switch changed nothing in a render.
- **Several effects run in list order, each taking the one before's result.** Two effects matched, to the last figure or within one floating-point step, the same two steps chained inside one group, in each order. Which effect was marked active did not change the order. An effect switched off for render is left out of the chain, and an effect with no node group passes the picture through without stopping the next one.
- **A group's "can be used as a scene compositor effect" flag, left off, does not stop it running** as an effect.
- **The render reaches a group through a Group Input node** as well as through a Render Layers node: Group Input → Invert → Group Output, with no Render Layers node in the group, wrote the render inverted.
- **The Compositor editor's New button** gave the scene one effect, both switches on, holding a new group of a Group Input wired to a Group Output and no Render Layers node; the path above the canvas then read Scene › Scene Effect › Scene Compositor. Blender 5.3's release notes state that the default compositor tree changed to this. The header otherwise matches 5.0's family: a dropdown reading Scene, the menus, a node-group chooser with New and a pin. Along the bottom of the empty editor sits a shelf of ready-made effects — Bloom, Vignette, Sepia and others. In a fresh file nothing exists until New.
- **The live output follows a click on a node's header**, not the node a script has made the editor's active one, and the Unused Output warning moves with it.
- **A File Output node with no connection to Group Output still writes** — with a Group Output elsewhere in the group, with none, and fed from a Group Input — and it wrote during a still render that was not asked to save anything.
- **Two other times the compositor runs**, both tied to an editor being shown, are named in 5.3's sources and were not tested as such: the viewport compositor, which Blender 5.3's release notes say is now on by default, set to Always, in the startup file's Layout workspace; and the interactive compositor, which the same notes say runs after rendering if a Viewer is visible. Neither is the compositing of a final render.

**The 9000-pixel frame, read in full in 5.3.** 4.5 to 5.2's runs read the first pixel only. 5.3's read every pixel, at 9000 by 400, 400 by 9000 and 10000 by 5000. On CPU the whole frame was composited every time. On GPU, on Apple's graphics, only 8,192 times the shorter side was — 3,276,800 pixels at both of the first two sizes, 40,960,000 at the third — and the top of the frame held the raw render. The file was always written at full size, and the limit applies to the height as well as the width. What 4.5, 5.0 and 5.2 hold beyond the first pixel was never read, so this is not established as a difference between versions; a Blender 5.2.2 control on the same Mac composited exactly the same pixels as 5.3 at 9000 by 400. No other make of card was tried for 5.3.

**One thing about the device that only the rented machine could ask.** Four compositor trees — an invert; a blur; a rescale to fixed pixel dimensions; and a chain of an invert into a blur blended back over the original at half strength — were each rendered twice at 320 by 240, once with the compositor on the processor and once on the card, and the two written files compared value by value. **Fifteen of the sixteen comparisons were identical to the last figure.** The sixteenth, the blur in 4.5, differed by at most one step of 255, with nothing differing by more than that anywhere. So at ordinary sizes the choice of device makes no visible difference to what the compositor produces, in any of the four versions. This rests on the rented Linux machine and its NVIDIA card alone, and it is **not** the same claim as the 9000-pixel result above. Nor does it settle whether the two devices give the same values exactly: a finer comparison under question 8, on the other rented machine, at another size and in another file format, found them never identical. **5.3 was asked the same on the Mac's own graphics**, with the same four trees at 320 by 240 written as 32-bit OpenEXR: the invert came back identical to the last figure and the other three within about five millionths, with no value differing by more than one step of 255. The three machines' figures may not be set beside each other, and none settles the others.

**Verdict: it differs.**

**Two things rest on one machine in each version**, and both came back the same in all five:

- **The Post Processing panel's own header carries no checkbox.** The header shows the fold arrow and the words, and nothing else. The Stereoscopy header in the same picture does carry a checkbox, so one would have shown here if it existed. For 4.5 to 5.2 this rests on the rented Linux machine, the Mac's looks having come back without an answer; for 5.3 on the Mac.
- **Neither output node's own header carries a control for making it the live one.** The header holds the fold arrow and the node's name, and nothing else — at three times magnification in 4.5 to 5.2, on the rented machine; in 5.3, on the Mac. Blender 5.1's and 5.2's documents quote the manual saying the live output can be changed in the node's header, and 4.5's, 5.0's and 5.3's say nothing about such a control — that shape is not a change in Blender, and the pictures show the same headers in all five.

**Not established.** The effect of the render engine's compositing flag in every version. Whether a **Warning node** exists in each version: it appears in Blender 5.2's documented list of compositor output nodes and in no other version's findings, and no test of it was ever written down. What a tree with no output node writes to the scene's own file, and what the GPU compositor writes above 8k beyond the first pixel, as set out above. Whether 5.3's GPU run really used the card: the Mac has no tool that names the processes on its graphics. And whether a File Output node writes during a still render not asked to save in the other four versions: only 5.3 was asked.

---

## 2. Can the place where a scene's rendered frames are written be switched off entirely?

**How it is known.** Documented outright in 5.2, backed by that version's release notes, and documented outright in 5.3; recorded as not in the sources in 4.5, 5.0 and 5.1. Settled by hand test. 4.5 to 5.2 were run from a terminal and by eye on the Creator's Mac and then again on the rented Linux machine, which agreed line for line — including the whole of the longest quoted run in the topic. 5.3 was run from a terminal and in its window on the Mac alone.

**The same in all five.** An animation render writes its frames automatically. A single frame rendered through the render command writes only when it is told to. Muting a File Output node stops that node writing — the folder stays empty. And with the compositor's final output node deleted, a file is still written: the raw render in 4.5 to 5.2, a transparent black frame in 5.3 — a split that is not established as a difference, as set out under question 1.

**Where it differs, version by version.**

**Blender 4.5, 5.0 and 5.1.** There is no switch for it. Asked for by name, the setting is simply absent in these three, and the render goes ahead and writes as usual. Nothing in the Output panel's header turns writing off — the window look found no checkbox there in any of these three.

**Blender 5.2.** Yes — a **Save Output** switch, in the Output panel's own header, immediately to the left of the word "Output", carrying no words of its own and ticked by default. Blender 5.2's release notes state the change: saving render output can now be disabled. With it turned off, a two-frame animation run from a terminal wrote **nothing** from the Output panel — not to the path stored in the file, and not to the path handed to it on the command line — while a **File Output node in the same tree wrote both frames**. The same run in 4.5, 5.0 and 5.1 wrote both the command-line frames and the File Output frames.

**Blender 5.3.** The same switch, in the same place, ticked by default and carrying no words, and the same result on the Mac: with it off, the two-frame animation wrote nothing from the Output panel, with the command line's path or without, while the File Output node wrote both frames. 5.3 was asked three things 5.2 was not:

- **A single frame told to save, with the switch off,** wrote only the File Output node's file, whether or not the command line named a path.
- **With the switch off and no File Output node at all,** Blender printed "Render output disabled in Output properties and no active compositing File Output nodes" as an error, rendered no frame, wrote nothing — and **ended with status 0**, for an animation and a single frame alike. So the exit status does not report this failure.
- **The checkbox's tooltip** reads "Save Output" over "Write frames to disk for animation renders." It was caught by the simulated pointer in two photographs.

5.3's own manual picture of the Output panel shows no checkbox in the header, where its text and the program both have one: a stale picture inside one version's documents.

**A second difference, on the same question.** Emptying the File Path field does different things:

- **Blender 4.5** writes the frames anyway, into the folder the command was run from, named by their frame numbers.
- **Blender 5.0, 5.1, 5.2 and 5.3** fail. Each reports that it could not create the directory for the file, writes nothing, and stops after the first frame. 5.3 ended with status 1.

So an empty path is not a way of switching writing off in any version — but it is harmless in 4.5 and fatal in the other four.

**A third, smaller one.** The Output panel gained a **Media Type** row from 5.0: 4.5's panel has none, and 5.0, 5.1, 5.2 and 5.3 all offer a dropdown with three entries — Image, Multi-Layer EXR, Video.

**Verdict: it differs.** The switch that stops a render's output being saved exists in **5.2 and 5.3**; 4.5, 5.0 and 5.1 cannot switch it off, and the absence was checked by asking each of them for the setting by name rather than inferred from a quiet manual. For 4.5 to 5.2 this rests on both machines, which gave the same result in all four versions: on Linux too, the Save Output run wrote nothing from the Output panel and only the File Output node's two frames. 5.3's rests on the Mac.

**Not established.** The wording of the tooltip on 5.2's Save Output checkbox — the Creator gave its data path instead, which settles what the control is but not what it says. It could not be reached on the rented machine either, because a tooltip needs the pointer held still and the display there is shared, so this gap stands on both machines; 5.3's tooltip, above, says nothing about 5.2's. What 5.2 does with the switch off for a single frame, or with no File Output node: asked of 5.3 alone. And what the render engine's image-saving flag actually does: Blender 5.1's documents name it as the one documented thing that can stop files being written during an animation render, and the flag is present in all five versions, but it was never exercised in any of them, on any machine.

---

## 3. Does a compositor belong to one scene, or to the whole file it is part of?

**How it is known.** Documented in every version. 4.5 marks its key step inferred, 5.0 to 5.2 document the sharing outright, and 5.3 documents each piece and marks the combined answer inferred. Settled by hand test: 4.5 to 5.2 from a terminal and by eye on the Creator's Mac and then again on the rented Linux machine, which agreed on every point — including the exact words of 4.5's refusal; 5.3 from a terminal and in its window on the Mac alone.

**The same in all five.** Which compositor is in play is decided by which scene is rendered. Naming a scene on the command line picks that scene, and that scene's own compositor is the one that runs — a file holding two scenes with different compositors gave each scene's own result when each was named in turn. A file can hold several scenes, each with its own compositor.

**Where it differs, version by version.**

**Blender 4.5.** The compositing tree is a read-only property of the scene. Pointing a second scene at an existing tree was **refused outright**, with Blender saying the property cannot be written to. Each scene keeps its own tree, and the second scene went on rendering through its own arrangement. In the Compositor editor's header there is no way to name or choose a tree; what shows instead is a fixed path reading "Scene > Compositing Node Tree".

**Blender 5.0, 5.1 and 5.2.** The tree is a thing in its own right that a scene points at. Two scenes were pointed at the same tree and the assignment was accepted. Both then wrote the **same** picture — the picture of the scene named on the Render Layers node inside the shared tree, not the picture of the scene being rendered. The Compositor editor's header shows a data-block selector with a New button, the ordinary way a shared thing is picked in Blender, and clicking New makes one with a name of its own. Beside it is a sub-type dropdown offering **Scene** and **Sequencer** — so from 5.0 a compositing tree is used by the video sequencer as well as by a scene, which 4.5's header offers no sign of.

**Blender 5.3.** The node groups are file-level things, as in 5.0 to 5.2, and one group served two scenes the same way: both wrote the picture of the scene named on the Render Layers node inside it. What a scene holds is its own **stack of effects** (question 1), each pointing at a group, and sharing one group gave each scene an effect of its own pointing at it. **Each scene's own Compositing switch then decided for it**: with the second scene's switch off, rendering it wrote its raw render while the first, switch on, still wrote the shared group's result. The header carries a dropdown reading Scene and a node-group chooser with New and a pin; New gave the scene an effect called Scene Effect holding a new group called Scene Compositor.

**Verdict: it differs.** In 4.5 a compositing tree belongs to one scene and cannot be shared. From 5.0 it is a shareable thing the file holds, and a scene points at one; the same tree can serve two scenes, and when it does, a Render Layers node inside it follows the scene named on the node rather than the scene being rendered. In 5.3 a scene points at its groups through its own stack of effects rather than through one pointer. 4.5 to 5.2 rest on both machines, which gave the same answer in all four versions; 5.3 on the Mac.

One thing checked and found the same everywhere: the Render menu in a fresh file holds Render Image, Render Animation, Render Audio, View Render, View Animation and a Lock Interface checkbox — and no entry naming a Sequencer Scene. That came back identical in 4.5 to 5.2 on both machines and in 5.3 on the Mac. What the menu holds once a Sequencer Scene differs from the active one is under question 7.

**Not established.** Two scenes sharing one tree with different Compositing switches: settled for 5.3 as above, not asked of 5.0, 5.1 or 5.2.

---

## 4. If a scene takes in another scene's rendered result, does running the first also run the second scene's compositor?

**How it is known.** The sources partly document what each route supplies. Whether the second scene's compositor runs, and whose switch controls the Scene strip, were left inferred or not in the sources. It was settled by hand test: in 4.5 to 5.2 on the Creator's Mac and then again on the rented Linux machine, every version returning the same results on both; in 5.3 on the Mac alone.

**Through a Render Layers node pointing at the other scene — no.** What arrives is the other scene's raw render. Its compositor does no work at all: the other scene's own File Output node, wired so that any work would leave a file behind, wrote nothing in any run, in any version — while rendering that scene directly filled the folder. The other scene's own Compositing switch made no difference to either signal, on or off, in any version.

**Through a Scene strip in the video sequencer — yes.** The strip carries the other scene's composited picture, and that scene's compositor genuinely runs: its File Output node wrote a file exactly when that scene's own Compositing switch was on. This also settles whose switch the manual's sentence "If Compositing is also enabled" means. It is the **strip's own scene's** switch. The rendering scene's switch made no difference to the picture, in any version. In 5.3 there is one more switch in play: with the strip's scene's Compositing on but its one effect switched off for render, the strip carried that scene's raw render and its File Output node wrote nothing.

**Verdict: the same everywhere.** 4.5 to 5.2 rest on two machines — macOS and Linux, Apple's own graphics and an NVIDIA card — and 5.3 on the Mac; the same figures throughout.

**One caution from 5.3, which is not a difference between versions.** The strip result above holds when each render is made in a fresh Blender. Rendered one after another in one Blender session, the picture carried over: once the first scene had been rendered with the strip's scene's compositor running, every later render in that session carried the same composited picture whatever the switches, and the strip's scene's File Output node never wrote again. A Blender 5.2.2 control on the same Mac did the same. 4.5, 5.0 and 5.1 were not asked, and the earlier 5.2 runs followed the switches. Recorded, not explained.

Blender 5.3's release notes add that the compositor now renders scenes referenced inside node groups. That speaks to whether a referenced scene is rendered at all, not to whether its compositor runs, and no version tested a reference inside a nested group, so what it changes is not established.

One thing did differ between the versions in these runs, and it is answered under question 8: the name the File Output node gave the file it wrote. It changes nothing here.

---

## 5. When a render enters the compositor, what does it arrive as, can it be told apart from anything else there, and can it be followed forward through nodes that only pass it along unchanged or split it into more than one?

**How it is known.** The first part is documented in every version. The middle part is recorded as not in the sources in every version, at the level of the data; 5.3 also marks as inferred that the entry nodes can be told apart by type. The last part is inferred in every version. Settled by hand test. In 4.5 to 5.2 it was run from a terminal and by eye on the Creator's Mac; the terminal tests were run again on the rented Linux machine, in all four versions, and agreed line for line. 5.3 was run from a terminal and in its window on the Mac alone. **What hovering a link shows rests on the Mac alone in every version.** It was attempted on the rented machine four different ways and could not be settled: no tooltip appeared, but neither did one on a control that certainly has one, and something else moved the pointer during the run. That is a fault in the run, not a finding about Linux, and no answer was written in.

**The same in all five.**

- **What it arrives as.** A Render Layers node, offering an image and an alpha, plus one socket for each enabled render pass. Once inside, it is one of only three kinds of data, and it carries a position and a size in the compositing space rather than being only a grid of pixels. In 5.3 the render also arrives through a Group Input node (question 1). 5.3's two readings on precision disagree — its manual says all three kinds are stored at half precision, its API page that final renders use full precision — and neither was tested.
- **Telling the entry point apart — yes.** The Render Layers node is its own type, plainly distinguishable from the Image node, and it names the scene and the view layer it reads. In every version the view-layer reading came back as the real name rather than the placeholder the reference page prints. In 5.3 the Group Input node carries a third type of its own; its documents add that a Group Input inside a nested group carries whatever the outer tree hands it, so on its own it is not proof that the render is there.
- **Telling the data apart — not established.** No version's sources describe a marker carried by the picture that identifies it as a render. None was found in the inspected node and socket properties or in the displays seen. That does not establish what the picture data itself carries; 5.3's fuller inspection display was not reached.
- **Following it forward — yes.** In every version the walk from the Render Layers node ran to completion, through a reroute to both of its destinations, with the muted link and the muted node each flagged along the way, and the muted node reporting the route it passes signals through.

**Where it differs, version by version.**

**What the Render Layers node offers on a fresh scene.** Blender 4.5 and 5.0 present the full list of about thirty pass sockets whether or not the passes are switched on, and 5.0 uses longer names where 4.5 uses short ones — Diffuse Color rather than DiffCol, Material Index rather than IndexMA, Depth rather than Z. Blender 5.1, 5.2 and 5.3 present **only Image and Alpha**. On a fresh scene is the whole of that: switching a pass on puts its socket back in 5.1 and 5.2, which is settled under **File output nodes 16** — with the Mist pass on, the Mist socket was there to be linked and wrote its file in every version that topic tested. One pass was tried, so what the sockets do in general is not settled, and 5.3 was not asked in this topic.

**Whether the live output node can be recognised from the file.** In Blender 4.5 no node in a compositing tree carries that mark and the probe printed nothing. In 5.0, 5.1, 5.2 and 5.3 the Group Output node reports itself as the live output, so which output is live is a property of the node in the file rather than something derived from an open window.

**What hovering a link shows — and this one is the Mac's answer only.** Blender 4.5 shows the socket's name with a trailing full stop. Blender 5.0 and 5.1 show the same name with no full stop. Blender 5.2 shows that same single word for a link out of an Image node, but for a link out of a Render Layers node it shows a fuller display — and only once the eye icon in that node's upper-left corner has been clicked. After a render that display gives the resolution, the transformation, the sampling and the precision, plus a line naming the type. Before a render it gives something different again: a value, a colour swatch and its numbers. Those four answers were taken by hand. **5.3's was taken by a simulated pointer**, after a frame had been rendered from a script: over the middle of a link, nothing at all; on the Image node's output socket, a box reading only "Image", gone within three seconds; on the Render Layers node's output socket, after about three seconds, the one-value kind of display — "Image", a value of 0 0 0 0, "Type: Float Color", a colour swatch and its numbers. The fuller display never appeared; both nodes carried an open-eye icon at the right end of their headers. The same simulated pointer brought up nothing at all in a Blender 5.2.2 control on the same Mac, so the method, not the version, may be what decides what 5.3 showed: **where 5.3 stands here is not established.** **No line of any of it, in any version, names where the image came from.**

**The Split node.** Blender 4.5's has a Factor input and two image inputs. From 5.0 it has Position and Rotation and two image inputs. All five have exactly one output, so something can be followed forward past it in every version. Which physical side each input lands on is answered under question 8, and it too changes at 5.0.

**Verdict: it differs.** The three headline answers are the same everywhere; the machinery underneath is not. Everything on this question except the hovering rests, for 4.5 to 5.2, on both machines, which agreed in all four versions — including the Render Layers node offering only Image and Alpha on a fresh scene from 5.1, and the view-layer reading coming back as a real name. 5.3's answers rest on the Mac.

One thing that looks like a version difference and is not: Blender 4.5's manual says the Split node has no output sockets, and Blender 4.5 itself gives it one. That is a contradiction inside a single version's own documents.

How a second output node is marked — a dark red header in 4.5, an "Unused Output" warning from 5.0 — bears on this question too, and is recorded under question 1, along with the answer that neither node's own header carries a control for making it the live one.

**Not established.** An origin marker in the picture data itself, and 5.3's precision and hovering, as qualified above. The render arriving through a Group Input node: seen in 5.3, never tried in 4.5 to 5.2. Whether switching a pass on puts its socket back in 5.3.

---

## 6. Besides a scene's pixel dimensions and its resolution percentage, can anything else inside a file decide the size of what gets written?

**How it is known.** Every version documents the render region and marks the stereo sizes inferred; every version leaves the File Output node's size unsettled. Settled by hand test. 4.5 to 5.2 were run from a terminal and by eye on the Creator's Mac; the terminal tests were run again on the rented Linux machine, in all four versions, and every size figure agreed. The window look was run there in two halves: **the first half agreed on both machines; the second half — ticking Render Region and looking again — rests on the Mac alone**, because it needs a button pressed and a press does not reliably reach Blender on the rented machine. 5.3 was run from a terminal and in its window on the Mac alone.

**The same in all five.** Yes: at least three other things decide it. These shared tests gave the same results apart from the measurement below.

- **The render region with Crop to Render Region on.** A region covering the middle half of a 400 by 200 frame wrote a file of exactly 200 by 100. With the region set but the crop left off, the written size was untouched at 400 by 200.
- **The stereo layout, when both views go into one file.** Unsqueezed side-by-side doubled the width to 800 by 200. Unsqueezed top-bottom doubled the height to 400 by 400. Both squeezed forms kept the scene's own 400 by 200.
- **A File Output node writes at the size of the picture plugged into it**, not the scene's. Fed a picture scaled down while the scene's own output ran unchanged in the same render, it wrote the small size and the scene's output wrote 400 by 200.

Four more results came back the same everywhere. The output-path template reports the scene's own resolution numbers and not the size actually written — a file named for 400 by 200 measured 200 by 100. The sequencer route does not change the written size: all six of its runs wrote the scene's own size. At 9000 pixels wide the written file was the full 9000 by 400 on both devices. And in the window, Crop to Render Region starts greyed out with both boxes unticked, and nothing in the Output panel offers a second way of deciding the written size. All of those held on both machines for 4.5 to 5.2 and on the Mac for 5.3 — except that Crop to Render Region **becoming** available once Render Region is ticked was only seen on the Mac, in every version.

**Where it differs.** One number, and only one. Fed a picture scaled to 120 by 90, the File Output node wrote **120 by 90 in Blender 4.5 and 5.0**, and **122 by 90 in Blender 5.1, 5.2 and 5.3**. The two extra pixels were not asked for and the test does not explain them. They came back on the Mac in all three of those versions, and on the rented Linux machine in 5.1 and 5.2.

**5.3 was asked more, and none of it is a difference between versions**, because the earlier versions were not asked:

- A Scale node set to half size gave the File Output node a file of 200 by 100, and an 800 by 600 picture fed straight in gave 800 by 600, with the scene's own output at 400 by 200 throughout.
- **A Crop node** with its transparency option off, placed before the output node, did not change the main output's size: it stayed 400 by 200, its corner transparent.
- **Pixel aspect** set to 2 and **Burn Into Image** each left the written size at 400 by 200.
- At **400 by 9000 and 10000 by 5000**, as at 9000 by 400, the file was written at full size on both devices.
- **Under a cropped render region, the crop cuts the compositor's full-frame picture.** A group whose only picture is a full-frame left-to-right gradient wrote 200 by 100 holding the middle half of the gradient — about 0.25 at its left edge and 0.75 at its right — not the whole gradient squeezed in. A group fed through a Group Input wrote 200 by 100 as well.
- **An odd frame size in H.264 is refused, not adjusted.** At 1921 by 1081 Blender printed "width not divisible by 2 (1921x1081)" as an error, rendered no frame, wrote no file — and ended with status 0. The 1920 by 1080 control wrote a two-frame video at 1920 by 1080.

**Verdict: it differs** — on that one measurement. The other shared tests agreed; the extra 5.3 results are not established for every version. For 4.5 to 5.2 the whole of it rests on two machines, apart from the second half of the Crop to Render Region look; 5.3's rests on the Mac.

**One further thing, from the rented machine only.** Whether the compositor's own device setting changes what gets written was asked directly there, at 320 by 240, across four compositor trees and all four versions: it does not — fifteen of sixteen comparisons identical to the last figure, the sixteenth differing by one step of 255. That is set out in full under question 1, with 5.3's own reading on the Mac, the one size at which the two devices do disagree, and the limit of what each settles.

**Only the documentation differs here, on one point.** A limit saying pictures larger than 8k are not supported and are cropped or scaled is quoted only from Blender 5.2's and 5.3's own Limitations pages; 4.5's has no such section and neither 5.0's nor 5.1's documents quote one. The same test in all five, on every machine it ran on, wrote a full 9000 by 400 file, so nothing about the **written size** follows from that difference in what is written down. The picture's **content** did differ at that size — 5.1 alone came back black on the graphics card, on both machines and both makes of card — and that is recorded under question 1 as a real difference. So is 5.3's finding that, on Apple's graphics, only part of the frame was composited, which is not established for the earlier versions.

**Operating system.** 5.3's documents carry the one operating-system note in this area: the Limitations page names the graphics driver timing out on Windows during heavy compositing on the card. That bears on long GPU compositing rather than on size, and no Windows machine ran it.

**Not established.** Everything in the list above that only 5.3 was asked.

---

## 7. Can a scene be set to bypass the compositor and produce its output from its video sequence editor instead, and if so, what does that change about what gets written?

**How it is known.** Documented in every version that the two switches exist, with 5.1 and 5.2 marking as inferred the step that either can be off while the other is on; 5.3 documents the setting and several of its effects and marks the rest inferred. Settled by hand test: 4.5 to 5.2 from a terminal and by eye on the Creator's Mac and then again on the rented Linux machine, which agreed on every figure, every channel name and every menu entry; 5.3 from a terminal and in its window on the Mac alone.

**The same in all five.** Yes. Compositing and Sequencer are two separate checkboxes in one panel, both on by default. Turn Compositing off and leave Sequencer on, and the written frame comes from the video sequence editor. Six further results came back identically in every version:

- With Sequencer on and **no strips**, a file is still written, and it holds the scene's ordinary picture — the camera view with Compositing off, the compositor's result with it on.
- With **a strip present**, the strip's picture is written whether the rendering scene's own Compositing switch is on or off. The scene's own compositor result never reaches the file.
- With both switches off, the camera view is written.
- The written size does not change on any of these routes.
- Scene strips carry only the combined picture; individual passes do not come across.
- Rendered to a multi-layer OpenEXR through the sequencer, the file holds one layer called Composite with only the combined picture — the Depth pass is gone. Rendered without the sequencer, the same scene's file holds its view layer with both its Combined and its Depth passes. A multi-layer file is written either way. (In 5.3's run the Mist pass was switched on as well, on a Workbench scene, and no Mist channel appeared in either file.)

**Where it differs, version by version.**

**The Sequencer Scene, from 5.0 onward.** Blender 5.0, 5.1, 5.2 and 5.3 give the sequencer a scene of its own rather than following the window's active scene, add conditional Render menu entries for rendering that scene instead of the active one, and add a flag to the render command for choosing it. Blender 5.0's release notes state the change outright: the sequencer no longer uses the active scene in the window for its context and now uses the Sequencer Scene. Blender 4.5 has none of this. This matters for what gets written because it decides whose output settings apply.

**What a multi-layer OpenEXR carries alongside the picture.** Blender 4.5's file held the colour channels alone. Blender 5.0, 5.1 and 5.2 each carried an extra pixel-aspect channel per layer as well, both with and without the sequencer. 5.3's reading listed the file's colour and depth channels and was not built to show a pixel-aspect entry, so it says nothing either way.

**5.3 answers three things the earlier versions left open or were never asked**, all on the Mac:

- **The Render menu grows its Sequencer Scene entries.** With the workspace's Sequencer Scene set to a scene other than the active one, the menu adds Render Sequencer Image (⌥F12) and Render Sequencer Animation (⌥⌘F12) between Render Animation and Render Audio.
- **A run with no window renders the active scene.** A plain single-frame render from the command line rendered the file's active scene and ignored every workspace's Sequencer Scene. Naming the Sequencer Scene on the command line rendered its edit; so did the render command's Sequencer Scene flag, called from a script with no window, which wrote to that scene's own output path.
- **With a strip present and Compositing on, the scene's own compositor does not run at all.** Its File Output node wrote nothing then; it wrote only when the group's result was what reached the file.

**Verdict: it differs.** For 4.5 to 5.2 the whole of this question rests on both machines, which gave the same answers in all four versions; 5.3's rests on the Mac.

**Not established.** Whether the Render menu grows its Sequencer Scene entries in 5.0, 5.1 and 5.2: the look was taken in a fresh file where the two had not been made different, in all four versions and on both machines, so this is untried there rather than answered differently. How a run started with no window picks a Sequencer Scene in 5.0, 5.1 and 5.2: not tried in this topic's runs, on either machine; **Settings 12** holds that test. Whether the scene's own compositor runs under a strip in 4.5 to 5.2: not asked. And whether 5.3's multi-layer file carries the pixel-aspect entry.

---

## 8. Can a picture be followed forward through a node that changes its colours, and through a node that takes two pictures in and gives one out — and can it be told which of the two came out the far side?

**How it is known.** The readings from the sources gave three kinds of answer, and they were much the same in all five versions: **documented** that a colour-changing node takes one picture in and gives one picture out, that the Switch node passes one named input straight through, that Depth Combine chooses pixel by pixel on depth, and that one designated input decides the size and placement of what comes out; **not in the sources** that a picture carries any name, tag or record of where it came from; and **inferred**, in every version, that the links and sockets described one page at a time add up to a chain that can be walked.

Almost everything decided below is **observed** rather than read. For 4.5 to 5.2, the terminal tests were run on the second rented Linux machine, in all four versions; the four tests that need a screen were run on the Creator's Mac, in all four versions. Four of the Linux tests were also run on the Mac and agreed with it to the last figure: the four Alpha Over and Switch renders, the Viewer node with no window, the reading of a saved graph, and the list of property names on a socket. **The Mac half of the other thirteen of the seventeen terminal tests has simply not been run** for those four versions. Nothing blocks it; until it is run, those thirteen answers settle Linux alone. **For 5.3 every test was run on the Mac, and none on Linux.** So on those thirteen, 4.5 to 5.2 and 5.3 answered on different machines: where they agree, they agree across two machines, and where they part, nothing is established. That is a gap in what was attempted, not a difference between operating systems.

**The same in all five.**

**Yes, a picture can be followed forward through the wiring. No origin marker was found in the inspected properties.**

- **Through a colour-changing node.** One picture in, one picture out, in every version. The link leaving it names the node and the socket it reaches.
- **Through a node that takes two pictures in.** Every such node has one picture output, in every version, so the chain does not break there either.
- **The whole chain reads back out of a saved file with no window at all.** One Blender built a tree — picture node, colour node, two-picture node, output — and saved it; a second, separate Blender opened that file and printed every link with the node and the socket at both ends, the Switch node's stored choice, and each node's internal routing. Complete, in all five versions — 4.5 to 5.2 on both machines, 5.3 on the Mac.
- **And it reads exactly the same with a window open.** On the Mac, the same file read with no window and with the window open gave the same nodes, the same links, the same socket identifiers and the same stored choice, character for character, in all five versions.

**Which of the two came out the far side — the running Blender settles it, node by node, and the same way in every version.**

- **The Switch node hands the chosen picture through exactly.** Off gives the first input with no difference at all, On gives the second, in all five.
- **The choice can be fed from a link, and it still decides the whole frame at once.** A value node and then a whole picture were both linked into the Switch input; both links were accepted in every version, and with a picture running from 0 on the left of the frame to 1 on the right on that input, the whole frame still came out as the Off input. So the choice never varies from pixel to pixel.
- **Mix and Alpha Over hand one input back at the ends of their factor.** With the two opaque test pictures on the inputs, a Mix at factor 0 gives back the first and a Mix at factor 1 the second; Alpha Over at factor 0 gives back the background and at factor 1 the opaque foreground. Every one of those is exact in 4.5, 5.0, 5.1 and 5.3, and a hair off in 5.2 on one of them, as set out under "where it differs" below. Between the ends, both inputs contribute and nothing says which contributed what.
- **The mark that says an input has no effect follows the factor, not the picture.** In all five versions the first input is marked as having no effect at a factor of 1.0, the second at 0.0, and neither in between. **That mark reports the factor's arithmetic, not which picture reached the far side**, so it cannot be used to tell inputs apart on its own; a node's stated internal route and the rendered frame are what do that.
- **A muted node states which input it hands over, and what it states is what the render produces.** Every two-picture node's internal routing named one picture input — always the first — and the rendered frame matched that input exactly, in all five versions, for Alpha Over, Mix, Switch, Split and Z or Depth Combine alike.
- **A muted colour node passes the picture through untouched.** Hue/Saturation/Value with a large shift and RGB Curves with a strong curve both changed the picture when live and gave it back with no difference at all when muted, in every version.
- **Depth Combine chooses pixel by pixel.** With the first picture's depth running across the frame against a flat depth for the second, the left of the result was the first input and the right the second, and the whole frame matched neither.
- **Split puts both inputs out, each on its own side of the line** — which side is where the versions part company, below.

**No record of where a picture came from was found in the inspected node and socket properties, in any version. Whether the picture data itself carries one is not established.**

- **Nothing on a socket reports the picture travelling through it.** The list of property names on a socket is the same in 4.5 to 5.2 bar one addition, and in none of the five versions does any name an inspection, a resolution, a transform, a precision or a sampling. The one property that sounds promising holds a socket's own fixed value, not a picture. The list is identical with a window open and with none — on both machines for 4.5 to 5.2, on the Mac for 5.3.
- **No metadata property exists on the compositor's Image, File Output or Image Info nodes**, in any version, nor on the image data block in 4.5 to 5.2, where it was looked for. The Image node has no input socket at all, so nothing can be fed into it.
- **No property on a socket or on a node names a priority or a domain.** The rule the manual describes for which input decides the frame is not reachable from a script anywhere.
- **A link cannot be walked across a node group boundary.** Nothing in the outer tree names a node inside the group, in any version; the walk has to step into the group's own tree.

**Four more results came back the same in all five, on different machines.** For 4.5 to 5.2 each was run on the rented Linux machine only; for 5.3 on the Mac only.

- **Alpha Over takes the frame of its first input.** A 32-pixel picture and a 128-pixel picture were fed in, once each way round, and rendered at 128; the area the result covered followed whichever picture was on the background input, in every version. In 4.5 to 5.2 it came back that way on all three of the runs that asked it.
- **And so does a Mix node.** The same two pictures into a Mix node's two colour inputs, with the factor set and stated at 0.0, 0.5 and 1.0, and the frame read straight off the node rather than guessed at from where the picture was solid: the frame is the first colour input's — 32 when the small picture is first, 128 when the large one is — in all five versions, at every factor tried. It does not follow the second input, or the larger picture, or the render size, and it does not move with the factor. A control with no Mix node in the tree at all reported each picture's own size, which is what makes the readings readable. **The frame must be read from the node and not from the picture:** how much of a frame is solid is a reading of the picture's transparency, not of its frame, and from 5.1 a Mix node mixes transparency as well as colour, so at a factor of 1.0 the solid area shrinks to the second input's while the frame stays on the first.
- **A faint mark survives a colour change.** A block raised by 0.02 at the centre of a picture, put through a hue and saturation shift, was still there at its full strength when the result was compared against the same picture put through the same shift, with nothing else moved. Compared instead against the unmarked original, which is what the documented technique describes, the difference outside the mark came out larger than the difference inside it.
- **The card's picture and the processor's picture are close but not the same.** On the 4080 SUPER machine they were about four parts in ten million apart on a heavy tree and about six on a small one, the same figure to the last digit in all four versions; on the Mac's own graphics in 5.3, about one part in a million on the heavy tree and three in ten million on the small one. So a picture cannot be identified by its exact values across the two device settings, and any test coarser than a millionth still holds. The two machines' figures may not be set beside each other. This is **not** the same comparison as the one at 320 by 240 under question 1.

**One more, from the rented Linux machine only.** **The compositor set to the graphics card really composites on the card, with no window at all.** On the Linux NVIDIA card, in 4.5 to 5.2, the Blender set to the processor was never once found on the card across many askings, and the same Blender set to the card was found there holding memory while it worked, with its own share of processor work falling away. Whether 5.3's run used the card was not checked: the Mac has no tool that names the processes on its graphics.

**And a result from the Mac alone, the same in all five versions.** A picture rendered with a window is the same picture as one rendered with none, with no difference at all across four renders.

**Where it differs, version by version.** Unless an item says otherwise, 4.5 to 5.2's results come from the same hand test run in all four versions on the rented Linux machine, and 5.3's from the same test on the Mac; where 5.3 joins one side, it joins it on another machine.

**The Split node's two sides.** Feed a picture into the first input and a different one into the second, and read the two ends of the frame. **Blender 4.5 puts the first input on the right and the second on the left. Blender 5.0, 5.1, 5.2 and 5.3 put the first on the left and the second on the right.** So a tree carried from 4.5 to 5.0 has its two halves swapped over.

**Whether muting marks a socket as having no effect.** **Blender 4.5 sets that mark, and setting it is what muting does**: on a muted Alpha Over the second picture input, the factor and the checkbox all turn true, and unmuted every one reads false. **Blender 5.0, 5.1, 5.2 and 5.3 leave it false throughout**, muted or not, on Alpha Over, Switch, Split and Depth Combine alike. So a reader who wants to know which input a muted node ignores can ask for that mark in 4.5 and must read the internal routing instead from 5.0 onward.

**Where a Mix node's factor starts.** A freshly added Mix node holds a factor of **1.0 in Blender 5.2 and 5.3** and **0.5 in 4.5, 5.0 and 5.1**. So a file opened with nothing changed mixes differently in 5.2 and 5.3 from the way it mixes in the other three.

**Whether a Mix node mixes transparency as well as colour.** **Blender 5.1 and 5.2 mix the two pictures' transparency along with their colours; 4.5 and 5.0 mix only the colours.** With a large picture on the first input, a small one on the second and the factor at 0.5, a pixel outside the small picture comes back fully solid in 4.5 and 5.0 and half solid in 5.1 and 5.2, with identical colours either way. Blender 5.1's release notes state the change. **5.3 sides with 5.1 and 5.2** on the one reading it took: at a factor of 1.0, with the large picture first, the solid area shrank to the small picture's, as it does in 5.2.

**What a fresh File Output node writes, and what it calls the file.** What the node calls its file follows the node's mode. In multi-layer mode no socket name reaches the file; in image mode it does. A freshly added node starts on PNG in 4.5 and on Multi-Layer OpenEXR from 5.0 — **that starting format is itself the version change**, and it is what made this test look like the node naming its file differently. In 5.3 a fresh node would not even take PNG until its media type had been set to Image. With the node's file-name setting and its format both left exactly as each version starts them, and its input socket renamed:

- **Blender 4.5** wrote a file named after the socket, with the frame number after it.
- **Blender 5.0 and 5.1** wrote one `file_name.exr` for the whole node, `file_name` being what their file-name setting starts at.
- **Blender 5.2 and 5.3** wrote the blend file's own name with `.exr` on the end, because their file-name setting starts at a token standing for that name; in 5.3, for a file never saved, that came out as `Unsaved.exr`.

The socket's name reached the file in none of the four later versions because in multi-layer mode the file takes the node's own file name alone. Put the same node into Image mode and the socket's name does reach the file, joined to the node's file name. The mode rule is settled under **File output nodes 6** and the starting format under **File output nodes 1**.

**Whether an empty Viewer image exists when there is no window.** **Blender 4.5, 5.1, 5.2 and 5.3** hold an image called `Viewer Node` as soon as a Viewer node is added, sitting at 256 by 256 with every value zero and staying that way across a render. **Blender 5.0 holds no such image at all**, before or after. For 4.5 to 5.2 this came back the same on both machines: the rented Linux machine gave it in all four versions, and the Mac's no-window runs matched it line for line. 5.3's is from the Mac. With a window open, 4.5 to 5.2 fill the image at the size of the render — so in 5.0 the image does not come into being until a render happens with a window there.

**Whether a render with a window fills the Viewer node's image.** This one was run on the Mac in all five versions. **In 4.5, 5.0, 5.1 and 5.2** a render made with the window open fills the image at the size of the render, with real values. **In 5.3 it does not**: in the Layout workspace the image stayed at 256 by 256 with every value zero after the render, exactly as with no window. In the Compositing workspace, which shows an image editor beside the compositor, it was already at the render's size with real values before the render was started, and every value was zero after it; that pattern is recorded rather than explained. Blender 5.3's release notes state the change: rendering no longer writes to viewer images, though the interactive compositor runs after rendering if the viewer is visible. A Blender 5.2.2 control on the same Mac filled the image in both workspaces.

**And one of a size that matters only to an exact test.** Where 4.5, 5.0 and 5.1 hand an input back with no difference at all, **5.2 comes back one step of a floating-point number short of exact** — on a Mix at full factor, and on Depth Combine with flat depths, both ways round. It is about six parts in a hundred million. It changes nothing anybody would see; it breaks any test that asks two pictures to be identical to the last digit. That is the 4080 SUPER machine's answer. **5.3 came back exact** on all of them — but on the Mac, and the Mac half of this test was never run in 5.2, so whether 5.3 differs from 5.2 here is not established.

**Custom metadata at the file nodes.** Blender 5.1's release notes say the Image and File Output nodes now read and write custom metadata. This is a **documented addition in 5.1**. The property checks found no property naming metadata on either node in any version; that does not test metadata reading or writing and does not overturn the release note. Whether metadata already in a file passes through the compositor remains untested in all five versions.

**Which node types a compositor tree accepts.** Offered every node type Blender lists, a compositor tree accepted **168 in 5.2** — on the rented Linux machine, and again in a Blender 5.2.2 control on the same Mac — and **187 in 5.3**, on the Mac. Which types were added was not listed.

**Verdict: it differs.** The three headline answers — a picture can be followed forward through both kinds of node, which of the two came out can be told for each of the nodes named above, and no origin marker was found in the inspected properties — are the same in all five versions. What differs is the machinery underneath, as set out above. The exact-rounding comparison with 5.3 remains open because its machine differs from 5.2's.

**Which Switch the Add menu adds — not established.** Drawn by its own code, 5.3's Add menu entry labelled "Switch" adds the **general-purpose Switch node** — the type geometry nodes use — not the compositor's own Switch. The general-purpose one has the inputs Switch, False and True and the output Output, and it gave back the chosen picture exactly, as the compositor's own does. The compositor's own Switch, with Switch, Off, On and Image, is still accepted by a compositor tree, but no menu entry adds it. The general-purpose type is accepted by a compositor tree from 5.2 and refused in 4.5, 5.0 and 5.1. What 5.2.0's menu adds was not tested, though a Blender 5.2.2 control on the same Mac drew the same entry as 5.3; 4.5 to 5.1 were never asked. So whether the menu's Switch changed node, and at which version, is not established, and nor is which node the manual's Switch page describes in 4.5 to 5.2.

**Not established.** Eight things, none of them a difference between versions:

- **What the socket inspection panel draws in its fuller form.** A script can drive Blender but cannot hold a real pointer still over a socket. In 4.5 to 5.2 it was tried as far as that goes, with a window and without; in 5.3 a simulated pointer brought up only the one-value display, as set out under question 5.
- **Whether the File Output node's file-name setting can carry a token standing for the socket's name.** Untried in all five.
- **Whether metadata already sitting inside a picture file on disk comes out anywhere in the compositor.** Untried in all five; there is no property on any of the three relevant nodes to look at.
- **The Mac half of thirteen of the seventeen terminal tests in 4.5 to 5.2, and the Linux half of every 5.3 test.** Not blocked by anything; simply not run.
- **Whether 5.3 hands one input back exactly where 5.2 does not**, as above.
- **Which Switch the Add menu adds in 4.5 to 5.2, and so which node the manual's Switch page describes.** 5.3's manual names the Switch node's sockets False, True and Output — the names of the general-purpose Switch its menu adds — and its picture for that page is named after that node. 5.2's manual names the same three, and 5.2's own compositor Switch offers Off, On and Image; whether 5.2's manual was describing the other node is not settled.
- **Whether 5.3's compositor, set to the graphics card, ran on it.**
- **Whether 5.3's image data block carries a metadata property.** Only the three nodes were looked at.

**Only the documentation differs on three further points**, and every one of them is a version's own documents disagreeing with that same version's running program, or with the other versions' documents. **In each case Blender does the same thing everywhere the test could reach, so none is a difference in behaviour.**

- **Only Blender 5.2's and 5.3's manuals name socket inspection at all.** Nothing behind it is reachable from a script in any of the five versions, with a window or without, so nothing about the running program follows from three manuals being silent. What the panel draws in its fuller form stays untested everywhere.
- **Only Blender 5.2's documents were found to quote a sentence saying which end of a Mix factor gives which input.** The test came back the same in all five versions anyway. One version's reading wrote it down and four did not.
- **The manual pictures of the Alpha Over node in 5.0, 5.1 and 5.2 draw both picture inputs with the same name**, where those versions' own text, and the running program, name them Background and Foreground. That is a stale picture, not a change in Blender. 5.3's picture of that node was not described.

**Two things about running Blender on macOS**, both facts about the operating system rather than about any version of Blender:

- **A Blender started with no window on macOS still reports a screen with four areas on it**, and prints the same four as a Blender started with a window, in 4.5 to 5.2; in 5.3 it still reports a window and a workspace to a script. So counting areas cannot tell a windowed run from a windowless one. In 4.5 to 5.2 the Viewer node's image tells them apart, filling only when there is a window. In 5.3 it does not, since a windowed render no longer fills it; there Blender's own background flag does.
- **A Blender given a window did not end itself when the script asked it to.** The same script with an ordinary exit added after that request ended in about a second and printed exactly the same thing. This was tried in 4.5, which is enough to show the two endings differ.

Neither of these says anything about how the versions differ.

---

## 9. When one scene takes in another scene's rendered result, what decides the render engine and the sample count that the second scene is rendered with, and what effect on that does a value for either, given to the render from outside, have?

**How it is known.** Two kinds of claim sit side by side here and they are kept apart.

From the documents, every version says the same thing and none of them answers the question. All five state where the two settings live — the engine on a scene, the sample count on a scene under Cycles and under EEVEE, with an override on a view layer inside it and, under Cycles, a scene-level policy saying how that override is used; the Cycles sample count is per scene, **observed in Cycles' own files**. Getting from there to the scene reached through the node is a step a reader has to supply: 4.5 to 5.2 mark their answers **inferred** for it, and 5.3 marks it **not in the sources**. What a value handed in from outside does to that scene is recorded as **not in the sources** in all five, and that stays true whatever a run showed: no version's manual, API documentation or release notes say which scene the command line's engine option reaches, and none names an option that sets a sample count.

Everything decided below is **observed**: one set of hand tests, run in all five versions on the Creator's Mac, driven from a terminal. In 4.5 to 5.2 every figure was got twice — once by the run and once by a repeat of it on the same machine; in 5.3 each was got once, the windowed render twice. Each test file was built by the same Blender that rendered it, so nothing crossed a version boundary. Two instruments and no weaker ones: the lines Blender prints about its own render, and exact comparison of the written pictures channel by channel, where a difference of exactly zero means the same picture to the last bit. **Sample count here means the count that was asked for**: denoising, adaptive sampling and any time limit were switched off in every file, and Cycles ran on the processor with a fixed seed, so a render repeats exactly.

**The same in all five.**

- **The engine is the pulled-in scene's own.** Workbench inside a Cycles render, Cycles inside a Workbench render, EEVEE inside a Cycles render and Cycles inside an EEVEE render all came out as the pulled-in scene's engine, matching a render of that scene made on its own to the last bit and differing plainly from one made with the compositing scene's engine. From 5.0 Blender names the engine in its own log as well, and it names the pulled-in scene's every time.
- **The sample count is the pulled-in scene's own**, under Cycles and under EEVEE alike, with the counts swapped between the two scenes and tried both ways round.
- **The compositing scene's count does not leak into it.** With the pulled-in scene held at one count and only the compositing scene's number changed — by a factor of four thousand — not one pixel of the written picture moved.
- **A view layer's sample override reaches through the node**, and under Cycles it is the **pulled-in scene's own** policy that decides what happens to it. Told to use the override, Blender used it whatever its size; told to ignore it, it rendered at the scene's count; told to bound it, it used the override below the scene's count and clamped it to the scene's count above. Giving the two scenes opposite policies settled whose is read: the count followed the pulled-in scene's in both directions and the compositing scene's in neither.
- **Nothing about the render changes because it arrived through the node.** In every version, ten files covering all three engines, four Cycles counts, two EEVEE counts and an override under each engine were rendered twice over — once through the node, once with the pulled-in scene named as the scene to render and no compositor involved at all. All fifty pairs came out identical to the last bit.
- **The compositing scene's own view layer is not rendered at all** when nothing in its compositor asks for it. One scene renders, and then the compositor runs.
- **An engine given on the command line never reaches the scene behind the node.** With both scenes carrying the same engine, so that any change could only come from the argument, the picture was the stored engine's every time. What the argument does instead is write the engine onto the **active scene and onto that scene alone**, at the point in the command line where it appears — read before it, both scenes still hold what the file holds; read after it, only the active scene has changed. It writes nothing to disk: reopening the file afterwards shows both scenes unchanged. Naming the pulled-in scene as the scene to render does make the argument apply to it, but that is a different render — the compositing scene's compositor is then not used at all.
- **No command-line option sets a sample count**, by each version's own account of itself: neither section of its help where such an option would sit holds one. Offered one anyway, Blender neither takes it nor refuses it — it renders at the stored count, says nothing about the argument, and ends normally. **A script handed in on the command line does reach it**: setting the pulled-in scene's count before the render made Blender render at that count.
- **An engine name that does not exist is refused rather than ignored.** All five print the same one-line error and render nothing.
- **The node carries two settings and no more** — a scene and a view layer. A view layer carries no engine setting anywhere, and one sample setting. The engine lives on a scene's render settings and nowhere the node or a view layer can reach.
- **A render started inside a Blender with a window is the same picture, to the last bit, as one started from a terminal.** The limit on that is worth stating: it was started by a script running inside that window, not by a person pressing a key, so what it shows is that having a window and a full interface loaded changes nothing about the render path.
- **The node's own re-render operator, called from a script, cancels without rendering**, in all five, with the node editor in the right state.

**Where it differs, version by version.**

**Whether a view layer's sample override is honoured under EEVEE.** With the pulled-in scene set to 64 samples and its view layer overridden to 8, **Blender 4.5 rendered at 64 — the override had no effect — while 5.0, 5.1, 5.2 and 5.3 all rendered at 8**, each matching an 8-sample render exactly. In 4.5 to 5.2 it was re-run in every version, so no answer there rests on a single run; 5.3's rests on one. **It is not a difference in what the compositor does, and that was established rather than assumed:** in 4.5 to 5.2 the same file rendered directly, with no compositor involved at all, split the same way, and in 5.3 the through-node render matched the direct one to the last bit. What differs is whether EEVEE honours a view layer's sample override at all; the path through the node reports faithfully whichever way that goes. The documents line up with it — the four later versions carry a manual page saying the override is always used under EEVEE, and 4.5's manual has no such page — but the documents are not what settles it; the runs are. **The same split is settled again outside this topic, on a second operating system and a second make of card**, by **Settings 16**, in 4.5 to 5.2 on the Creator's Mac and on a rented Linux machine with an NVIDIA RTX 4060 Ti.

**Whether a render's log names the engine.** Blender 5.0, 5.1, 5.2 and 5.3 print a line naming the engine in use. Blender 4.5 prints none: its progress lines name the scene and the view layer, and Cycles' and EEVEE's sample counts, and never the engine. So in 4.5 the engine a render actually used cannot be read out of what Blender prints, and the only way to tell is the picture.

**What EEVEE is called from outside.** Blender 4.5 names it `BLENDER_EEVEE_NEXT` and Blender 5.0, 5.1, 5.2 and 5.3 name it `BLENDER_EEVEE` — both in the listing each version prints when asked what engines it has, and in each version's own API documentation. Blender 5.0's release notes state the rename outright. Workbench and Cycles are named the same in all five. So a command line that names EEVEE has to know which Blender it is talking to.

**How a refused command ends.** Asked to list its engines, Blender 4.5 prints the listing and then aborts, ending with status 134 three times out of three; the other four print the listing and end normally. Given an engine name that does not exist, all five print the same error and render nothing, but 4.5 again ends at 134 where the other four end at 1. Both of these rest on the Mac alone, and an abort is the kind of thing that can be an operating system's doing as much as Blender's.

**Verdict: it differs** — on those four points, and 5.3 sides with 5.0, 5.1 and 5.2 on all four, on the same Mac. The answer proper is the same in all five: the scene behind the node is rendered with its own engine and its own sample count, its own view layer's override reaches through under its own policy, an engine given on the command line never touches it, and no command-line option exists that would set its sample count. The whole of this question rests on the Creator's Mac and settles macOS on Apple silicon alone.

**What the re-render button does, from 5.3 alone.** In 5.3 a simulated click on the button beside the Render Layers node's view-layer setting started a render of the scene the node names: Blender's log named that scene, and the status bar showed it rendering. Tried on two files whose scenes carry different engines, both ways round, **the render used the engine of the scene behind the node** each time. Its sample count could not be read: the windowed log prints no sample lines, the render result held no picture afterwards, and a Viewer fed by the node stayed empty. 4.5 to 5.2 were never pressed.

**Not established.** Six things, none of them a difference between versions:

- **What the node's own re-render button renders with.** In 5.3 it renders with the engine of the scene behind the node, as above, and its sample count is not known. In 4.5 to 5.2 the operator behind it cancelled without rendering, and a press is a mouse click none of the machines used for those versions could make. This project has since had a rented machine on which a synthetic click reached Blender's window, so it is untried there rather than unreachable.
- **Everything above, on any operating system but macOS.** Nothing was rented for this question, in any version. **The one exception is the EEVEE override difference among 4.5 to 5.2, which has already been repeated elsewhere and needs no run**: **Settings 16** found the same split — a view layer's own sample count changing an EEVEE render in 5.0, 5.1 and 5.2 and doing nothing in 4.5 — in all four versions on the Creator's Mac and on a rented Linux machine with an NVIDIA RTX 4060 Ti, both machines agreeing, with Blender 5.0's own release note beside it. So that difference is settled on a second operating system and a second make of card for those four, and this question's earlier request for such a run is met. 5.3's side of it rests on the Mac.
- **Whether EEVEE can render with no display on Windows.** Blender 5.1's own documents state that it cannot. No Windows machine ran it, so it is untested in all five.
- **What a third-party render engine does with any of this.** Every run started Blender at factory settings, so no add-on engine existed in any of them, and this project has none.
- **Whether running the compositor on the graphics card changes any of it.** Blender 5.2's release notes record a fix for a crash when a second scene was rendered through the compositor on the card; no run in any version was aimed at that, and none crashed.
- **What a sample count is for Workbench**, and whether adaptive sampling, a time limit or a sample subset change any of the above. Workbench was tested for the engine only. Whether Blender's own documents place a Workbench sample count on the scene at all is not settled: the manual's one sentence here, saying that a Workbench setting is saved per scene, can be read as naming a sample count or as naming only an anti-aliasing choice, and which of the two it names has not been settled either way.
