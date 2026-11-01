# File output nodes — Architecture questions (Blender 5.0)

Sources consulted: Blender's 5.0 manual, Blender's 5.0 Python API documentation, and Blender's 5.0 release notes, all as the plain-text snapshot taken on 2026-07-30; and Cycles' own files as supplied for 5.0. All reading done on 2026-08-25 against that snapshot; the date read for every quotation below is 2026-07-30.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered here: 16 and 17 only.

---

## 16. When a scene is brought into another scene through its video sequence editor, what do the File Output nodes belonging to the brought-in scene do?

**LABEL: not in the sources.**

No page in the 5.0 manual, the 5.0 API documentation, or the 5.0 release notes says what happens to a File Output node that sits in the compositor of a scene which is being rendered because another scene's sequencer holds a Scene strip pointing at it. The sources never bring the two subjects into contact. Nothing says the node writes; nothing says it is skipped, silenced, or ignored; nothing says the files land anywhere in particular.

What the sources *do* say, and which is the closest anything comes, is below. Each of these is documented except sub-point d, which is observed and carries its own label; none of them answers the question.

**a. What a Scene strip is.**

> "Scene strips are a way to insert the render output of another scene into your sequence."

Source: manual. Address: https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/scene.html — Date read: 2026-07-30

> "Scene strips cannot be used to reference the sequence’s own scene; a secondary scene must be used instead."

Same page and date. This matters only because it confirms the arrangement the question describes is always two distinct scenes.

**b. That the brought-in scene really is rendered, and that a compositor is involved.**

> "Renders the output of the Video Sequence editor, instead of the view from the 3D scene’s active camera.
> If the sequence contains Scene strips, these will also be rendered as part of the pipeline.
> If Compositing is also enabled, the Scene strip will be the output of the Compositor."

Source: manual, the Post Processing panel, under "Pipeline – Sequencer". Address: https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html — Date read: 2026-07-30

This is the only sentence in any of the three sources that puts a Scene strip and the compositor in the same statement. It says a compositor's output becomes the strip's picture. It does not say which scene's compositor, and it does not mention File Output nodes at all.

**c. That the sequencer stage receives an already-composited result.**

> "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist"

Source: API documentation, `RenderSettings.use_sequencer`. Address: https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — Date read: 2026-07-30

And, alongside it:

> "Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene"

Source: API documentation, `RenderSettings.use_compositing`. Same address and date.

**d. That these two switches belong to a scene, one set per scene.**

The Post Processing panel is built from `context.scene.render`, so each scene carries its own pair of switches:

> `        rd = context.scene.render`
> `        col = layout.column(align=True, heading="Pipeline")`
> `        col.prop(rd, "use_compositing")`
> `        col.prop(rd, "use_sequencer")`

**LABEL for the reading of the file: observed.** What the four lines show, and no more, is that this panel takes those two settings from whatever scene it is being drawn for. **LABEL for "one set per scene": inferred, one step** — the file shows one panel reading one scene's settings, and a reader has to supply that the same holds for every scene. File: `Blender Sources/cycles_5.0/ui.py`, class `CYCLES_PT_post_processing`, in its `draw` method (lines 1122–1126 of that file). Date read: 2026-07-30. *(Citation check, 2026-08-26: the whole sub-point was labelled observed, including the heading's "one set per scene". The quoted lines carry the reading but not the conclusion, so the label has been split. The line numbers are right — `rd = context.scene.render` is line 1122 and `col.prop(rd, "use_sequencer")` is line 1126.)*

**e. What a File Output node is said to do, in general.**

> "The File Output node writes one or more images to disk during rendering or compositing."

Source: manual. Address: https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html — Date read: 2026-07-30

**f. One limitation the manual does state about Scene strips.**

> "Scene strips do not render individual Render Passes;
> only the Combined render pass will be used."

Source: manual. Address: https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/scene.html — Date read: 2026-07-30

This is a restriction on what the strip *takes* from the render. It says nothing about what the brought-in scene's compositor does or writes while producing it.

**g. That the brought-in scene is rendered at the moment it is needed.**

> "When used as strips, scenes are rendered dynamically during playback or rendering,
> and they share the same data-blocks unless they were created as full copies."

Source: manual. Address: https://docs.blender.org/manual/en/5.0/scene_layout/scene/introduction.html — Date read: 2026-07-30

Note the phrase "during playback or rendering". The sources do not say whether a File Output node behaves the same in both, or at all in the first.

### An inference, offered separately and marked as such

**LABEL: inferred.** If the brought-in scene's own compositor is what produces the strip's picture (quote b), and a File Output node "writes one or more images to disk during rendering or compositing" (quote e), then a File Output node in the brought-in scene would write its files while that strip is being produced.

The steps a reader has to supply to get there, and which the sources do not supply, are:

1. That "the Compositor" in quote b means the brought-in scene's compositor rather than the master scene's. Quote b does not say, and quote d shows both scenes have their own Compositing switch, so both readings are grammatically open.
2. That the whole node tree is executed in this situation, including branches that only end in a File Output node and feed nothing back to the strip.
3. That "during rendering or compositing" in quote e covers a compositor run performed to serve a strip in a different scene, and not only a compositor run performed to produce that scene's own final render.

Any of those three could be false without contradicting a word in the sources. Treat this inference as weak. It is not a substitute for the "not in the sources" answer above.

### The ambiguity in quote b, stated plainly

Quote b's sentence "If Compositing is also enabled, the Scene strip will be the output of the Compositor" has two readings and the manual does not settle which is meant:

- Reading one: the brought-in scene's own Compositing switch, so the strip carries that scene's composited picture rather than its raw render.
- Reading two: the master scene's Compositing switch.

**LABEL: inferred.** I lean to reading one, on the single ground that the sentence is describing what the strip carries. That is a step a reader supplies, not something the quoted words say: nothing quoted anywhere in this document says whose compositor "the Compositor" is, and nothing quoted rules reading two out. The reading is left open rather than settled.

### Not covered

- Whether any file at all is written. That is the question, and it is unanswered.
- Whether the answer differs between a full render of the master scene and mere playback or preview in the sequencer.
- Whether the answer differs between the strip's two Input types, Camera and Sequencer. They are named in the API's `SceneStrip.scene_input` entry, https://docs.blender.org/api/5.0/bpy.types.SceneStrip.html, read 2026-07-30; that entry's own words are not quoted here.
- Whether a scene reached through two levels of nesting behaves like one reached through a single level.
- What happens when several strips point at the same scene, or when the same compositing node group is shared by both scenes. In 5.0 the compositing node tree is a data-block in its own right — "The compositing node tree is now its own data block, making it reusable across blend files." (release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/, read 2026-07-30) — so whether "the File Output nodes belonging to the brought-in scene" could be the very same nodes as the master scene's is a step from that sentence rather than anything the sources state (**inferred**). The sources say nothing about what that would do.

### Outside the interface

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say. Nothing on any page I read distinguishes what a File Output node in a brought-in scene does when Blender is driven from outside its own window from what it does when a person clicks Render.

Two related things are documented, neither of which answers it:

> "-S, --scene <name>
> Set the active scene <name> for rendering."

Source: manual. Address: https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — Date read: 2026-07-30. So which scene is rendered can be chosen from outside; the sources do not go on to say what its strips' scenes' File Output nodes then do.

And the release-note sentence quoted under question 17 below states that a command-line render is always an animation render. That bears on naming, not on whether anything is written.

Also relevant as a gap, the 5.0 release notes say:

> "If the sequencer scene differs from the active scene, the Render menu will show new options for "Render Sequencer Image" /  "Render Sequencer Animation" on top of the normal scene render operators, allowing the user to specify which scene they'd like to render."

Source: release notes, 5.0 Video Sequencer. Address: https://developer.blender.org/docs/release_notes/5.0/sequencer/ — Date read: 2026-07-30. That is a menu inside the interface, and the sources do not say what the equivalent is from outside it.

### Operating system

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say. No page I read attaches any operating-system condition to Scene strips, to the compositor, or to the File Output node.

---


### What the hand tests settled — observed, 2026-08-26

**LABEL: observed.** Nothing in this section comes from reading. It was got by running Blender, by Claude Opus 5 (Claude Code subagent), on 2026-08-26, in Blender 5.0.1. The full list of tests, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`; what follows stands on its own and does not need it.

**The two computers, named in full.**

- **The Creator's own MacBook Pro** — Apple M1 Pro chip, macOS 26.5.2, Apple's own graphics, no second card. Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`.
- **A rented Linux machine, in Norway** — a Docker container holding half its host, Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 3060 with 12 GB and driver 595.84, about fifteen cores of an Intel Xeon E5-2697A v4, driven over SSH, with a display server of its own on `DISPLAY=:1` at 1920×1080. Blender 5.0.1 at `/opt/blender/5.0.1/blender`.

Both are the same build to the exact commit, so a difference between the two machines would be the operating system and not the build. **The two are never merged into one answer**; where both ran a test, both results are given. Nothing here was timed and nothing rendered on a graphics card — every render used Cycles on the processor at one sample, which is the only way the two machines could be set beside each other at all. The Linux machine's screen is drawn by its processor rather than by its card.

**The file every test was built from.** One blend file with two scenes, built by a short script so that it comes out identical in every version on both machines, and built so that no answer can be ambiguous:

| | the brought-in scene | the containing scene |
|---|---|---|
| name | `Inner` | `Outer` |
| camera | `InnerCam`, plus a second camera `OverrideCam` | `OuterCam` |
| frames per second | 24 | 30 |
| resolution | 320 × 180 | 640 × 360 |
| frame range | 1 to 5 | 21 to 25 |
| its own render output | a folder called `watch_innerown` | a folder called `watch_outer` |

`Outer`'s video sequencer holds one Scene strip pointing at `Inner`, placed at **outer frame 21** and five frames long, so outer frame 21 lines up with inner frame 1 and the two frame numbers can never coincide. `Inner`'s compositor holds a Render Layers node feeding a File Output node aimed at a folder called `watch`, plus the tree's own output node. Both scenes have Compositing and Sequencer switched on unless a test says otherwise. In this version the node carries a Directory and a File Name, set to the `watch` folder and `inner_`, with one output item named `Img`.

**The control that every other result rests on.** Rendering `Inner` on its own — `"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup <file>.blend -S Inner -a` — put `inner_Img0001.png`, `inner_Img0002.png`, `inner_Img0003.png`, `inner_Img0004.png` and `inner_Img0005.png` into `watch`, on both machines. So an empty folder in any test below can only mean that test's own change stopped the writing.

**The answer to the question.** **The brought-in scene's File Output node writes.** Rendering the whole of `Outer`'s animation from a terminal — `"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup <file>.blend -S Outer -a`, and `/opt/blender/5.0.1/blender -b --factory-startup <file>.blend -S Outer -a` on the rented machine — left one file in the `watch` folder, `inner_Img.png`, written over five times, with no frame number in the name, in Blender 5.0.1, **on both machines, with the same names**. In the same run the containing scene's own render was written as `outer_0021.png` through `outer_0025.png`.

Blender's own words on the Mac, the first two of the five frames, word for word — the last three read the same with the timestamps and the outer frame number moving on:

```
00:00.306  render           | Fra: 1 | Compositing
00:00.364  render           | Saved: '/private/tmp/fw/mac/t02/5.0/watch/inner_Img.png'
00:00.379  render           | Saved: '/private/tmp/fw/mac/t02/5.0/watch_outer/outer_0021.png'
00:00.386  render           | Fra: 2 | Compositing
00:00.392  render           | Saved: '/private/tmp/fw/mac/t02/5.0/watch/inner_Img.png'
00:00.403  render           | Saved: '/private/tmp/fw/mac/t02/5.0/watch_outer/outer_0022.png'
```

**What stops it, and what does not.**

| what was changed | what landed in the brought-in scene's folder |
|---|---|
| nothing (the plain arrangement) | it writes |
| the **brought-in** scene's Compositing switch turned off | **nothing** |
| the **containing** scene's Compositing switch turned off | it writes, unchanged |
| both Compositing switches turned off | **nothing** |
| the containing scene's **Sequencer** switch turned off | **nothing** |
| the strip muted | **nothing** |
| the strip's **Input** set to Sequencer instead of Camera | **nothing** |
| the brought-in scene put two levels down, behind a middle scene | **nothing** |
| the brought-in tree left with no output node of its own | it writes, unchanged |
| the sequencer's Override Scene Strip switch turned on | it writes, unchanged |
| a File Output node added to the **containing** scene as well | the brought-in scene's writes; **the containing scene's own node writes nothing at all** |

All four combinations of the two Compositing switches were run, so the manual sentence "If Compositing is also enabled, the Scene strip will be the output of the Compositor" is settled by test: **it is the brought-in scene's own switch that governs, and the containing scene's has no effect.** In every row that wrote nothing, the containing scene's own five frames were still written, so the render itself ran.

**Two levels down was run with a control.** A third scene, `Middle`, with its own camera and its own sequencer holding the `Inner` strip, was put between the two. Rendering `Outer` printed only `Fra: 11` through `Fra: 15` — the middle scene's own frames — and wrote nothing from `Inner`. Rendering the very same file with `-S Middle` instead did write from `Inner`. So a scene reached through a strip is rendered through its **camera**, and its own sequencer is not used, which is the same thing the Sequencer input row above shows from the other side.

**How often it writes.** Once for every frame of the strip that is rendered, and the count follows the strip rather than the scene:

- trimming the strip to three frames gave **three** writes, not five;
- setting the containing scene's frame step to 2 gave **three** writes, at the brought-in scene's frames 1, 3 and 5;
- stretching the strip to ten frames over a five-frame scene gave **ten** writes over five names;
- two strips of the same scene at the same time gave **two** writes per frame — ten in all — and one set of files;
- two strips at different places on the timeline also gave ten writes and one set of files, with nothing in the names telling the two strips apart.

**A single frame writes too.** Rendering one frame of `Outer` with `-f 21`, and rendering one frame from a script with `bpy.ops.render.render(animation=False, write_still=True)`, both wrote. So did `bpy.ops.render.render(animation=False)` **with `write_still` left off**, where the containing scene's own picture was not saved at all and the brought-in scene's node still wrote its file; the operator returned `{'FINISHED'}` and raised nothing. *(Citation check, 2026-08-26: repeating the two operator runs needs one step this record did not carry — the containing scene's current frame has to be inside the strip before the operator is called. Run on a freshly built file with the current frame left where Blender puts it, frame 1, both operator calls write nothing at all, because the playhead is nowhere near the strip. With the current frame set to 21 first, both reproduce exactly as described, on the Creator's Mac, in all four versions. The command line's `-f 21` needs no such step, because the frame is named in the command.)* Starting the animation with `bpy.ops.render.render(animation=True)` instead of the command line's `-a` gave exactly the same files as `-a`.

**A render pass is produced.** With the Mist pass switched on in the brought-in scene and fed to a second output, **both** files were written — `inner_Img.png` and `inner_Mist.png` — so the documented limit that "Scene strips do not render individual Render Passes" does not stop the brought-in scene making a pass for its own File Output node.

**One compositing tree shared by both scenes changes nothing.** The containing scene was pointed at the very same compositing tree the brought-in scene uses, with its own Compositing switch on. The node still wrote **once** per frame — five writes, one file — not twice.

**Moving the playhead with no render.** **It writes nothing.** With a real window open on the rented Linux machine, one area turned into the Video Sequencer's preview and **Scene Strip Display shading set to Rendered**, walking the playhead across every frame of the strip three times over, with no render started, left the folder empty. The same walk at Solid shading also left it empty. This was run on the rented Linux machine only, so it is not established for macOS. A terminal counterpart — the current frame set to each of the strip's frames in turn, with no window and no render — wrote nothing on either machine.

**The sequencer's own render command.** From a terminal with no window at all, `bpy.ops.render.opengl(animation=True, sequencer=True)` and its single-frame form both came back `{'CANCELLED'}` and wrote nothing, in Blender 5.0.1, on both machines — so those commands cannot be reached from outside Blender's window. With a real window on the rented Linux machine, and a Video Sequencer preview in front of it, the same command still came back `{'CANCELLED'}` and wrote nothing — at Solid shading and at Rendered shading alike. That windowed run is on Linux only.

**Outside the interface — now answered by test.** The answer is the same whether Blender is driven from a terminal with no window or from a real window. The same file rendered with a real window open on the rented Linux machine — `DISPLAY=:1 XDG_RUNTIME_DIR=/run/user/0 /opt/blender/5.0.1/blender --factory-startup <file>.blend`, with the render started from inside it — left exactly the same files with exactly the same names as the terminal run. The one place the two routes differ is the sequencer's own render command, above, which does not exist outside the window at all.

**Operating system — now answered by test, for two systems.** Every test in the table above was run on both computers, and **the Mac and the rented Linux machine agreed on every one**: the same file names, the same counts, the same printed lines, the same refusals. The only differences anywhere in the two logs are line numbers, timestamps and the folders the work sat in. **Windows is untested** — no Windows machine has run it — so the answer covers macOS on Apple silicon and Ubuntu Linux and says nothing about Windows. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**What the hand tests still leave open for this question.** Windows, in every version. And the four tests that need Blender's own window — the windowed render, the played preview, the sequencer's own render command with a window, and the node's sidebar panel — were run on the rented Linux machine only, never on macOS, because the Creator was given no test to run.


---

## 17. What is a file written by a File Output node called, when that node belongs to a scene brought into another scene through its video sequence editor?

**LABEL: not in the sources**, for what the question actually asks. The general naming rule for a File Output node is documented; nothing in any of the three sources says what that rule resolves to when the node's scene is being rendered as a strip inside another scene's sequencer, and several parts of the rule depend on things that become ambiguous in exactly that situation.

### The general rule, which is documented

> "Each connected input is saved to the specified directory and filename,
> with the frame number appended to create a sequence.
> For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

Source: manual. Address: https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html — Date read: 2026-07-30

From the same page, on the two path fields and on where the socket name enters:

> "The base name of the file.
> The final filename may also include frame numbers or layer names depending on the settings."

> "The name of each input socket is used to construct the final file name."

And on the two Media Types, which change whether there are many files or one:

> "Image:
> Saves each input as a separate image file."

> "Multi-Layer EXR:
> Saves all inputs together in a single multi-layer OpenEXR file."

Same page and date. The API states the same two fields:

> "directory — The directory where the image will be written"

> "file_name — The base name of the file. Other information might be included in the final file name depending on the node options"

Source: API documentation. Address: https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html — Date read: 2026-07-30. That page also records that `directory` is "never None, blend relative // prefix supported, Supports template expressions" and `file_name` is "never None, Supports template expressions".

### The frame number part, which 5.0 changed

> "The **File Output** node now only appends the frame number to the image name when doing an
> animation render. Rendering through the command line always does an animation render, so no change
> of behavior is expected. Rendering using the `render.render()` operator when `animation=False`
> will see the change of behavior, but setting that to True and providing `start_frame` and
> `end_frame` should restore the old behavior. Finally, adding `####` manually in the name should
> substitute the frame number even in single renders."

Source: release notes, 5.0 Compositor. Address: https://developer.blender.org/docs/release_notes/5.0/compositor/ — Date read: 2026-07-30

This is a documented statement about 5.0's behaviour. It says the frame number appears only on an animation render. It does not say whether rendering a master scene counts as "an animation render" for a File Output node in a scene reached through a strip, and it does not say *which* frame number is used.

### The parts of a name that can be written as template expressions

The manual lists what may be substituted into a File Output node's paths:

> "Currently path templates are only supported for the render output path in
> Scene properties and the output paths in the compositor’s File Output
> node."

and, among the variables:

> "scene_name:
> Name of the current scene."

> "camera_name:
> Name of the current render camera."

> "fps:
> The frames per second of the current scene."

> "node_name:
> The name of the node that the path property is on."

Source: manual. Address: https://docs.blender.org/manual/en/5.0/files/file_paths.html — Date read: 2026-07-30. The page also lists `blend_name`, `blend_dir`, `blend_name_lib`, `blend_dir_lib`, `resolution_x` and `resolution_y`, and states that substitution "happens internally at time of use (e.g. while rendering)".

### What is therefore not settled

Every one of these hangs on a word the sources leave undefined for the nested case:

1. **The frame number.** The manual says the frame number is appended. It never says whether that is the brought-in scene's own current frame or the frame of the master scene whose sequencer is being rendered. The manual states "The strip length will be determined based on the animation settings in that scene" (https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/scene.html, read 2026-07-30), so the brought-in scene's frame numbering and the master scene's are set separately. Nothing in the sources says which one lands in the file name.
2. **"the current scene".** `scene_name` is "Name of the current scene" and `fps` is "of the current scene". When the node's own scene is being rendered inside another scene's render, the sources never say which of the two is "current".
3. **"the current render camera"**, for `camera_name`. A Scene strip carries its own camera override — "This can be used to override the scene’s camera with any other object" (same Scene Strip page, read 2026-07-30). Whether that override reaches `camera_name` is not stated anywhere.
4. **`resolution_x` / `resolution_y`.** The File Paths page gives them as "The x and y resolution of the rendered image" (https://docs.blender.org/manual/en/5.0/files/file_paths.html, read 2026-07-30). Which scene's resolution applies when a strip is being rendered is not stated.
5. **Whether an animation render of the master counts as an animation render** for the nested node, which by the release note quoted above decides whether a frame number appears in the name at all.

The one thing the sources offer for checking a name is a panel that shows it:

> "Displays the final file path and extension for each input/layer, based on current settings.
> This helps verify that outputs are named and organized correctly."

Source: manual, the Output Paths panel. Address: https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html — Date read: 2026-07-30. That is a display inside the interface, and the manual does not say whether what it shows is what a nested render would produce.

I also looked at the picture the manual carries for this node, `_images/node-types_CompositorNodeOutputFile.webp`, shown on the File Output Node page at https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html — Date read: 2026-07-30. **LABEL for what follows: documented**, since the picture is part of the manual. What is visible in it: the node carries two text fields, the upper holding `/tmp/` with a small button beside it and the lower holding `file_name`, and one unlabelled socket on the left-hand side. What that button does, and what may be plugged into that socket, are not visible in the picture. It shows nothing about scenes, strips or frame numbers.

### Not covered

- The literal answer: no example name, and no rule stated for the nested case, appears in any of the three sources.
- Whether the answer differs between the strip's Camera and Sequencer input types.
- What happens when the same scene is used by more than one strip at once, or by strips at different places on the master timeline, and two writes would therefore compete for one name.
- Whether the file extension is decided by the node's own format settings in this case; the general rule that it is comes from the same page, but the nested case is not addressed.
- Whether `blend_name` / `blend_dir` differ, for instance if the brought-in scene is linked from another file. The manual documents `blend_name_lib` and `blend_dir_lib` as the library-aware variants for "path properties on library-linked data-blocks" (File Paths page, https://docs.blender.org/manual/en/5.0/files/file_paths.html, read 2026-07-30), but says nothing about the sequencer case.

### Outside the interface

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

Partly documented and partly inferred, and only for one part of the name. The release note quoted above states outright: "Rendering through the command line always does an animation render, so no change of behavior is expected." **LABEL for what follows: inferred.** Joining that sentence to the one before it in the same note — that the node "now only appends the frame number to the image name when doing an animation render" — gives that a File Output node driven from the command line always gets the frame number appended, whereas the same node driven by `render.render()` with `animation=False` does not. That joining is a step a reader supplies; neither sentence says it on its own. If it holds, it is a real difference between two ways of driving Blender from outside its window, and it changes the file name.

Beyond that the sources are silent. They do not say whether the nested-scene questions listed above resolve differently outside the interface. Note also that the Output Paths panel, the only documented way to see the final name, exists only inside the interface.

### Operating system

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say. No page I read attaches an operating-system condition to File Output naming or to path templates.

One nearby documented statement, which is about builds rather than about naming and which I am recording only so it is not mistaken for a gap: the command-line arguments page lists render formats and then says "Formats that can be compiled into Blender, not available on all systems:" / "OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP." (https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30). Since the extension is part of the file name, availability of a format is a way an operating system could matter, but the sources do not draw that connection and I am not drawing it for them.

---

## Record of where I looked

This record covers both questions together; the searching was the same sweep.

**Whole-source searches** (run over `manual_5.0` excluding `_images`, `api_5.0`, `release_notes_5.0`, and `cycles_5.0`, case-insensitive):

- `scene strip` — 40 hits in the manual, 7 in the API, 10 in the release notes. Every one was opened or read in context. Not one mentions the File Output node or writing files.
- `file output` — in the manual, only the File Output Node page, the two index pages, and two lines on the File Paths page. In the API, only `ImageFormatSettings` (unrelated, about compression speed), `aud` (unrelated, audio backends) and `NodeCompositorFileOutputItems`. No hit anywhere ties the node to a strip or a nested scene.
- `composit` restricted to `manual_5.0/editors/video_sequencer` and `manual_5.0/video_editing` — hits are the strip Compositing sidebar section, the cache page, the Compositor strip modifier, and the Alpha Over effect. None concerns File Output nodes or the compositor of a strip's scene.
- `sequenc` restricted to `manual_5.0/compositing` — 17 hits, all of them the ordinary English word "sequence" (image sequences, frame sequences) except one line on the Convert to Display page. The compositing chapter never discusses the video sequencer's Scene strips.
- `nested`, `sub-scene`, `subscene`, `referenced scene`, `another scene`, `other scene`, `written to disk`, `writes to disk`, `write to disk` — all run across manual, API and release notes. The only hits touching this subject are the Scene Strip page and the Render Layers node's hint about using another scene's compositing output, which concerns the Render Layers node and not the sequencer.
- `frame number` restricted to `manual_5.0/render`, `manual_5.0/compositing`, `manual_5.0/files`, `manual_5.0/video_editing`, and `####` across the whole manual — this found the naming rule, the Output panel's own four-zero padding rule, and the command-line `-o` padding rules.
- `file output`, `scene strip`, `sequencer` across `cycles_5.0` — one hit only, `col.prop(rd, "use_sequencer")` in `ui.py`, quoted above. Cycles' files carry no File Output node settings and no sequencer settings of their own; the panel simply draws the scene's two pipeline switches. Ruled out as a source for either question beyond that one observation.

**Pages opened and read in full:**

- Manual: File Output Node; Scene Strip; Post Processing; File Paths; Compositing Introduction; Compositing Limitations; Compositor System (first section); Render Layers Node; Enable Output Node; Sequencer Scene; Video Editing Introduction; Strip Modifiers (Compositor Modifier section); Scenes Introduction; Rendering Animations; Render Output Introduction; Command Line Arguments (render, format and scene options).
- API: `CompositorNodeOutputFile`; `NodeCompositorFileOutputItems`; `NodeCompositorFileOutputItem`; `SceneStrip`; `RenderSettings` (the `use_compositing`, `use_sequencer` and `use_sequencer_override_scene_strip` entries); `Scene` (the `compositing_node_group` entry).
- Release notes: the whole of `compositor.md` and the whole of `sequencer.md`; the Scene-strip and compositor lines in `bugfixes.md` and `corrective_releases.md`; `pipeline_io.md`, `core.md`, `rendering.md`, `python_api.md` and `index.md` searched for the four relevant terms.
- Pictures: `_images/node-types_CompositorNodeOutputFile.webp`, the File Output Node page's illustration, opened and described under question 17.

**Ruled out, and why:**

- The whole `compositing/types/` tree apart from the output pages: these are per-node reference pages for colour, filter, mask, transform and utility nodes. The full-text search for `scene strip` and `sequenc` over the compositing chapter returned nothing relevant, so opening them one by one would add nothing.
- `bugfixes.md` and `corrective_releases.md`: searched. They list one Scene-strip render fix ("Scene strip rendering flushes user edits") and one File Output fix ("Compositor File Output: Optional frame numbering", the same change as the release-note entry quoted above). Neither says anything about File Output nodes inside a strip's scene.
- The Compositor strip modifier, new in 5.0: it is a compositing node tree attached to a strip, not a File Output node inside a brought-in scene, so it is a different subject from both questions. Noted and set aside.
- Cycles' files generally: they contain no File Output node code and no sequencer code. Only the Post Processing panel touched either question, and it is quoted above.

**I did not use the web, and did not need to.**

---


### What the hand tests settled — observed, 2026-08-26

**LABEL: observed.** Nothing in this section comes from reading. It was got by running Blender, by Claude Opus 5 (Claude Code subagent), on 2026-08-26, in Blender 5.0.1, on the two computers described under question 16 above — the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2) and a rented Linux machine in Norway (Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 3060, driver 595.84, driven over SSH). The same build on both, so a difference between them would be the operating system. **The two are never merged**; where both ran a test, both results are given. The set-up is the one described under question 16: a scene `Inner` at 24 frames per second and 320 × 180 with frames 1 to 5 and a camera `InnerCam`, brought into a scene `Outer` at 30 frames per second and 640 × 360 with frames 21 to 25 and a camera `OuterCam`, by a Scene strip placed at outer frame 21 — so no part of a name can be ambiguous about which scene it came from. The full list of tests, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`.

**What the file is called.** Rendering the whole of `Outer`'s animation from a terminal left one file in the node's folder, `inner_Img.png`, written over five times, with no frame number in the name, in Blender 5.0.1, on both machines.

**Which frame number.** **The brought-in scene's own.** With the node's name part set to `inner_####_`, which forces the number into a place of its own, the files were `inner_0001_Img.png`, `inner_0002_Img.png`, `inner_0003_Img.png`, `inner_0004_Img.png`, `inner_0005_Img.png` — while the containing scene's own render, in the same run, was `outer_0021.png` through `outer_0025.png`. Without `####` the plain arrangement puts **no** frame number in the name at all, and one file is written over five times. Written into the name by hand as `####`, the number appears on a single-frame render too: rendering one frame with `-f 21` gave `inner_0001_Img.png` while the containing scene's own file was `outer_0021.png`.

**What the naming variables resolve to.** **Every one of them takes its value from the brought-in scene.** A name set to `{scene_name}-{camera_name}-{fps}-{resolution_x}x{resolution_y}-{node_name}-{blend_name}_` produced:

```
Inner-InnerCam-24.0-320x180-FO_inner-t_Img.png
```

`{scene_name}` is `Inner` and not `Outer`; `{camera_name}` is `InnerCam` and not `OuterCam`; `{fps}` is `24.0` and not `30`; the resolution is `320x180` and not `640x360`; `{node_name}` is the node's own name; and `{blend_name}` is the open blend file's name. Not one position took a value from the containing scene.

**The strip's own camera override does not reach the name.** With the strip's own Camera field set to a second camera in the brought-in scene, called `OverrideCam`, and the node's name set to `{camera_name}_`, the file came out `InnerCam_Img.png`. The override did **not** reach the name; the brought-in scene's own active camera did.

**`{blend_name}` and `//` name the open file, not the library.** The brought-in scene was put in a second blend file, in a folder of its own, and **linked** into the file holding the strip. Blender accepted it — it reported `{'FINISHED'}` for the link and built the strip against the linked scene — and the node wrote `mainfile_Img.png` into a folder beside the **open** file, not beside the library. The library's own folder gained nothing.

**The node's own format decides the extension.** With the node set to JPEG and both scenes' own output left at PNG, the node's files came out `inner_Img.jpg` while the containing scene's own render was still `outer_0021.png` through `outer_0025.png`.

**Multi-Layer EXR through a strip.** With the node's Media Type set to Multi-Layer EXR the run left one file, `inner_.exr`. The output's own name, `Img`, is not in the file name — it is a layer inside the file — and there is no frame number.

**The command line's `-o` does not reach the node.** Rendering with `-o <folder>/override_` moved the containing scene's own render to `override_0021.png` through `override_0025.png` and left the node's file exactly where and as it was.

**Nothing in the name tells two strips apart.** Two strips of the same scene, whether overlapping or at different places on the timeline, produced ten writes over the same names, so the second strip's files land on top of the first strip's.

**What the node's Output Paths panel shows.** Photographed on the rented Linux machine, with the compositor's sidebar open on its Node tab and the File Output node selected, the panel holds one row per output, reading `/root/fonwork/win...ch/inner_Img.png` — the middle of the path elided by the panel's width. It shows the same row whether the brought-in scene or the containing scene is the active scene, the editor pinned to the brought-in scene's tree. Set that name beside the two renders of the same file: through the strip the node wrote `inner_Img.png`, and rendered on its own it wrote `inner_Img0001.png` through `inner_Img0005.png`, so what the panel shows is the name without a frame number. The pictures are at `Audits/Blender Research/File output nodes/Screenshots — Architecture questions/`. This was read on the rented Linux machine only.

**Outside the interface — now answered by test.** The names are the same whether Blender is driven from a terminal with no window or from a real window. The same file rendered with a real window open on the rented Linux machine gave exactly the same names as the terminal run, in Blender 5.0.1.

**Operating system — now answered by test, for two systems.** Every naming test was run on both computers and **every name came back character for character the same**. **Windows is untested**, no Windows machine having run it, so the answer covers macOS on Apple silicon and Ubuntu Linux only.

**What the hand tests still leave open for this question.** Windows, in every version; and the node's sidebar panel, which was read on the rented Linux machine only and never on macOS.


---

## Things a person with Blender 5.0 open could settle by hand in a few minutes

These are written so somebody who cannot see my reasoning can carry them out.

1. **Does a brought-in scene's File Output node write at all?**
   Make a new file with two scenes, "Shot" and "Edit". In "Shot": set the frame range to 1–5, add a Compositor node tree, and in it connect the Render Layers node's Image output to a File Output node. Set the File Output node's Directory to an empty folder you can watch and its File Name to `shot_`. In Properties ‣ Output ‣ Post Processing for "Shot", make sure Compositing is on. Switch to "Edit", set its frame range to 1–5, open the Video Sequencer, and add a Scene strip pointing at "Shot", starting at frame 1. In Properties ‣ Output ‣ Post Processing for "Edit", make sure Sequencer is on. Now render the animation of "Edit" (Render ‣ Render Animation, with "Edit" active). If the Render menu offers more than one animation entry, say which one you used. Look in the watched folder. Report: are there files, how many, and what are they called exactly?

2. **Which frame number ends up in the name.**
   Repeat test 1, but place the Scene strip so it starts at frame 20 of "Edit" rather than frame 1, and set "Edit"'s frame range to 20–24. Render the animation of "Edit". Report the exact file names. If they are `shot_0001` … `shot_0005`, the brought-in scene's own frames are used; if they are `shot_0020` … `shot_0024`, the master timeline's frames are used.

3. **Does the frame number appear at all.**
   Repeat test 1, but render a single frame of "Edit" (Render ‣ Render Image) instead of the animation. Report whether the written file has a frame number in its name, and whether a file is written at all.

4. **Whether it also happens on the command line.**
   Save the file from test 2 and run `blender -b <file>.blend -S Edit -a`, then report the exact contents of the watched folder. Report that list beside the one test 2 produced from inside the interface; no comparison is being asked for, only both lists as they stand.

5. **What `{scene_name}` resolves to.**
   Repeat test 1, but set the File Output node's File Name to `{scene_name}_{node_name}_`. Render the animation of "Edit". Report the exact file names. They will show whether "the current scene" is "Shot" or "Edit".

6. **Whether the strip's camera override reaches `{camera_name}`.**
   Repeat test 5 with File Name `{camera_name}_`, and set the Scene strip's Camera field in "Edit" to a second camera object that is not "Shot"'s active camera. Render the animation of "Edit". Report the exact file names.

7. **Whether the "Shot" scene's own Compositing switch decides it.**
   Repeat test 1 twice: once with "Shot"'s Properties ‣ Output ‣ Post Processing ‣ Compositing switched off, and once with "Edit"'s Compositing switched off (leaving "Shot"'s on). Report in each case whether files appear. This settles which scene's switch governs, and so which reading of the Post Processing sentence quoted under question 16 is correct.

8. **Does mere playback write files?**
   With the file from test 1 open on "Edit", do not render. Just scrub the sequencer timeline across the strip a few times, and play it back once. Report whether any file appears in the watched folder.

9. **Does the strip's Input type matter?**
   Repeat test 1 with the Scene strip's Input set to Sequencer instead of Camera (give "Shot" a sequencer strip of its own so it has something to show). Report whether files still appear and what they are called.

10. **What the Output Paths panel predicts, versus what is written.**
    Before running test 1, open the File Output node's Sidebar ‣ Node ‣ Properties ‣ Output Paths panel while "Shot" is the active scene, and write down exactly what it shows. Then run test 1 and report the names actually written, beside what the panel showed. Report both; no comparison is being asked for.

---


## The researcher's own list of things to settle by hand — all of them now run

**Closed on 2026-08-26.** Every check proposed above was carried out, together with the tests the audit sent on and several more, in all four researched versions of Blender and on two computers. The results are written into questions 16 and 17 above as observed findings, and the whole list, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`.

**Only these are still open, and neither can be reached from the equipment this project has:**

- **Windows.** No Windows machine has run it, so every answer above covers macOS on Apple silicon and Ubuntu Linux and leaves Windows untested.
- **The four tests that need Blender's own window, on macOS.** They were run on the rented Linux machine, which has a display server, and not on the Creator's Mac. So the windowed render, the played preview, the sequencer's own render command with a window, and the node's sidebar panel are settled on Linux alone. Each of the first three has a terminal counterpart that was run on both machines.
- **A folder reached over a network**, which has been on this topic's *cannot be run* list from the start: neither machine has a network store.

*(The checks the researcher proposed are left in place above this note rather than deleted, so a reader can see what was asked as well as what came back.)*


---

2026-08-25 — Claude Opus 5 (Claude Code subagent) — File output nodes, Blender 5.0.
