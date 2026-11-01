# File output nodes — Architecture questions, second pass (Blender 5.0)

Questions answered here: **16 and 17 only.** The whole sheet was read; the other questions were not answered.

**Sources consulted, and when.** Blender 5.0 manual (`Blender Sources/manual_5.0`, including its `_images` folder), Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own files (`Blender Sources/cycles_5.0`). All four are a snapshot taken **2026-07-30**, and that is the date read for every quote below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## Question 16

**When a scene is brought into another scene through its video sequence editor, what do the File Output nodes belonging to the brought-in scene do?**

### LABEL: not in the sources

No page in the manual, no page in the Python API documentation, no release note, and no Cycles file mentions the File Output node together with Scene strips, or says anything about what a File Output node in a Scene strip's scene does. The searches that establish this are listed under "Where I looked" below.

### What the sources do say near this question

These are the closest documented statements. None of them names the File Output node in the Scene strip case; they are the raw material an inference would have to be built from, and I quote them so a reader can judge that for themselves.

**(a) Scene strips are rendered as part of the enclosing scene's render pipeline — documented.**

> "Renders the output of the Video Sequence editor, instead of the view from the 3D scene’s active camera.
> If the sequence contains Scene strips, these will also be rendered as part of the pipeline.
> If Compositing is also enabled, the Scene strip will be the output of the Compositor."

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html — the "Pipeline – Sequencer" entry. Read 2026-07-30.

I also opened the picture on that page, `_images/render_output_properties_post-processing_panel.png`, cited from the same address. It shows a "Post Processing" panel with two tick boxes side by side, "Compositing" and "Sequencer", both ticked, and a "Dither" slider reading 1.000. That is all it shows. **Label for the picture: documented.**

**Ambiguity I am flagging rather than deciding.** The sentence "If Compositing is also enabled" does not say *whose* Compositing setting is meant. The panel it appears in belongs to whichever scene's Output properties you are looking at, so the plain reading is that both tick boxes belong to the scene you are rendering (the enclosing scene). The other reading is that it means the brought-in scene's own Compositing setting. Likewise "the Scene strip will be the output of the Compositor" does not say which scene's compositor. The words do not settle it, and I am not going to settle it by guessing.

**(b) A File Output node writes during rendering or compositing — documented.**

> "The File Output node writes one or more images to disk during rendering or compositing."

Source: manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html. Read 2026-07-30.

**(c) Whether a compositing node tree is used at all is a per-scene setting — documented.**

> "To make the render engine actually use the compositing node tree (or disable compositing),
> the Compositing Pipeline option must be enabled in the output’s Post Processing properties."

Source: manual, https://docs.blender.org/manual/en/5.0/compositing/introduction.html. Read 2026-07-30.

> "Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene"
> — description of `use_compositing`, "boolean, default True"

> "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist"
> — description of `use_sequencer`, "boolean, default True"

Source: API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html. Read 2026-07-30. These sit on `RenderSettings`, which is a scene's render settings, so each scene carries its own pair.

The API page for a scene backs this up:

> "Unused but kept for compatibility reasons. Setting the property has no effect, and getting it always returns True. Use #scene.render.use_compositing to turn compositing to enable or disable compositing."
> — note on `use_nodes`, marked "Deprecated since version 5.0: removal planned in version 6.0"

Source: API documentation, https://docs.blender.org/api/5.0/bpy.types.Scene.html. Read 2026-07-30.

**(d) The two tick boxes are read off the scene — observed, from Cycles' own files.**

File: `Blender Sources/cycles_5.0/ui.py`, class `CYCLES_PT_post_processing`, lines 1112–1128. Exact lines:

```
class CYCLES_PT_post_processing(CyclesButtonsPanel, Panel):
    bl_label = "Post Processing"
    bl_options = {'DEFAULT_CLOSED'}
    bl_context = "output"

    def draw(self, context):
        layout = self.layout
        layout.use_property_split = True
        layout.use_property_decorate = False

        rd = context.scene.render

        col = layout.column(align=True, heading="Pipeline")
        col.prop(rd, "use_compositing")
        col.prop(rd, "use_sequencer")

        layout.prop(rd, "dither_intensity", text="Dither", slider=True)
```

What this shows, and only this: the Compositing and Sequencer settings drawn in that panel are taken from a scene's own render settings (`context.scene.render`), which makes them per-scene settings. Anything about what a render then *does* with them is a step beyond the file and is not observed.

**(e) A Scene strip does not carry render passes — documented.**

> "Scene strips do not render individual Render Passes;
> only the Combined render pass will be used."

Source: manual, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/scene.html — the "Limitations" section. Read 2026-07-30. This is about what the strip hands on to the sequencer. It says nothing about File Output nodes, which write their own files rather than handing anything on.

**(f) Two of the sequencer's own render commands do not render Scene strips at all — documented.**

> "Sequence Render Image and Sequence Render Animation don’t render the final video by default –
> specifically, they don’t render Scene Strips, instead using the preview’s
> shading mode (which is initially Solid).
>
> To output a video where the Scene Strips are rendered, use the Render menu in the top-bar,
> or change Sidebar ‣ View ‣ Scene Strip Display ‣ Shading to Rendered."

Source: manual, https://docs.blender.org/manual/en/5.0/editors/video_sequencer/preview/header.html. Read 2026-07-30. The same words appear at https://docs.blender.org/manual/en/5.0/editors/video_sequencer/sequencer/navigating.html.

### The inference, written out, and its weight

**LABEL for this paragraph only: inferred.** If you accept (a) that a Scene strip is rendered as part of the enclosing scene's render pipeline, and (c) that a scene's own Compositing setting is what makes its compositing node tree run, and (b) that a File Output node writes during rendering or compositing, then a File Output node in the brought-in scene would write its files while that strip is rendered. The steps a reader has to supply and that no source supplies: that "rendered as part of the pipeline" means the brought-in scene's full render including its own compositing pass, rather than only the part that produces the picture the strip needs; that the brought-in scene's own Compositing setting is what governs it; and that File Output nodes are not skipped in this context the way (e) shows render passes are dropped and (f) shows Scene strips are skipped entirely by the sequencer's own two render commands. Three supplied steps is a long chain, and (e) and (f) are both direct evidence that the Scene strip path does drop things that a plain render would do. I would not bet anything on this inference. The sources do not answer the question.

### Not covered

- Whether any file is written at all by the brought-in scene's File Output nodes during a Scene strip render. That is the question, and it is unanswered.
- Whether the brought-in scene's own Compositing tick box, or the enclosing scene's, or both, govern it.
- What happens when the same scene is used by more than one Scene strip in the same sequence, or when a strip is trimmed so only part of the scene's frame range is used, or when the strip is muted, or when the strip's scene input is set to "Sequencer" rather than "Camera" (see `scene_input` on https://docs.blender.org/api/5.0/bpy.types.SceneStrip.html, "enum in ['CAMERA', 'SEQUENCER'], default 'CAMERA'").
- Whether files would be written once per enclosing-scene frame, or once per frame of the brought-in scene, or both, and what happens where two strips overlap.
- Nesting more than one level deep — a brought-in scene that itself contains Scene strips.
- One thing worth knowing that the sources do state, and that widens the question: a compositing node tree is a data-block that "can then be reused across different scenes or linked/appended from other blend-files" (manual, https://docs.blender.org/manual/en/5.0/compositing/introduction.html, read 2026-07-30). So "the File Output nodes belonging to the brought-in scene" may not be nodes belonging to that scene alone. Nothing in the sources says what that means for writing.

### Outside the interface

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say. Nothing I found distinguishes what happens when Blender is driven from outside its own window from what happens when a person clicks. What the sources do give is the machinery for driving it from outside, none of which mentions Scene strips or File Output nodes:

> "-b, --background
> Run in background (often used for UI-less rendering)."

> "-a, --render-anim
> Render frames from start to end (inclusive)."

> "-S, --scene <name>
> Set the active scene <name> for rendering."

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html. Read 2026-07-30.

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"
> "Undocumented, consider contributing."
> "use_sequencer_scene (boolean, (optional)) – Use Sequencer Scene, Render the sequencer scene instead of the active scene"
> "scene (string, (optional, never None)) – Scene, Scene to render, current scene if not specified"

Source: API documentation, https://docs.blender.org/api/5.0/bpy.ops.render.html. Read 2026-07-30. Note that the operator itself is marked undocumented.

One caution for the case in (f): those two commands that skip Scene strips are commands of the sequencer editor, which means they are reached by a person clicking in that editor. I found nothing saying whether an equivalent skip applies to anything driven from outside.

### Operating system

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say. I searched the pages this answer rests on — the Post Processing page, the Scene Strip page, the File Output node page, the File Paths page, the Sequencer Scene page, and the API pages for the File Output node and for Scene strips — for "Windows", "macOS", "Linux", "platform" and "operating system". No hits on any of them. Nothing in what I read is stated to hold on one system and not another, and nothing states that it holds on all of them.

### Where I looked (question 16)

Searched across the whole of `manual_5.0`, `api_5.0` and `release_notes_5.0` at once, case-insensitively, for: "file output", "output node", "OutputFile", "file_output", "Output File", "scene strip", "SceneStrip", "nested scene", "nested", "sub-scene", "subscene", "another scene", "referenced scene", "render pipeline", "written to disk", "write to disk", "writes", "rendered twice", "recursive", "template expression", "post processing", "post-processing", "current frame", "frame_current", "subframe", "sub-frame". Also searched `cycles_5.0` for "sequencer", "scene strip", "SceneStrip", "file output", "OutputFile", "compositor", "composit", "video sequence".

Also ran a cross-search listing every manual file that mentions the sequencer *and* mentions compositing, and opened the ones that could plausibly bear on the question.

Pages opened and read in full or in the relevant part:
- Manual: the File Output node page; the Enable Output node page; the Compositor System page; the Compositing introduction; the Render Layers node page; the Scene Strip page; the Sequencer Scene page; the Scene introduction page (Scene Layout); the Post Processing page and its panel picture; the Output properties page; the Rendering Animations page; the video sequencer preview header page; the video sequencer preview sidebar page; the sequencer navigating page; the sequencer cache sidebar page; the strip editing page; the strip modifiers page; the strips introduction and index pages; the workspaces page; the dope sheet overlays page; the command line arguments page; the File Paths page.
- API: `CompositorNodeOutputFile`, `NodeCompositorFileOutputItem`, `SceneStrip`, `RenderSettings`, `Scene`, `bpy.ops.render`.
- Release notes: the compositor page, the sequencer page, the Python API page, the user interface page, the colour management page, and the bug-fix page — every File Output mention in each of them.
- Cycles: `ui.py` (the Post Processing panel and every "compositor"/"sequencer" mention in the file); confirmed no Cycles file mentions Scene strips, the sequencer's scene strips, or file output at all.

Ruled out, and why:
- **Compositor Modifier on a strip** (manual, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strip_modifiers.html): this is a compositing node tree attached to a strip in the sequencer, a different thing from a node tree belonging to the brought-in scene. The page never mentions the File Output node. Ruled out as not the thing asked about.
- **The Render Layers node's hint** (manual, https://docs.blender.org/manual/en/5.0/compositing/types/input/scene/render_layers.html): "To use the compositing output from another scene rather than its “raw” render output, first render that scene into a series of multi-layered images (using e.g. the OpenEXR format), then load those images into the Compositor of the current scene using the Image Node." This is about pulling another scene into a compositor through the Render Layers node, not about Scene strips in a video sequence editor. Ruled out as a different route between scenes.
- **`use_sequencer_override_scene_strip`** (API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html): "Use workbench render settings from the sequencer scene, instead of each individual scene used in the strip". This is the one setting I found that speaks about whose settings win for a Scene strip, and it is limited to Workbench render settings; it says nothing about compositing or file writing. The manual states the same in different words at https://docs.blender.org/manual/en/5.0/editors/video_sequencer/preview/sidebar.html: "Use the Workbench render settings from the current scene rather than the scenes referenced by the strips." Ruled out as too narrow to answer the question, but recorded because it shows the documentation does think about whose settings apply and still never extends that to compositing.

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

## Question 17

**What is a file written by a File Output node called, when that node belongs to a scene brought into another scene through its video sequence editor?**

### LABEL: not in the sources

Nothing in the manual, the API documentation, the release notes or the Cycles files says how a File Output node's file is named when the node's scene is rendered as a Scene strip inside another scene. The general naming rules are documented and are quoted below, but the one thing that would make them specific to this case — which scene supplies the frame number, and which scene counts as "the current scene" — is never stated.

### The general naming rules, which are documented

**The shape of the name — documented.**

> "The File Output node writes one or more images to disk during rendering or compositing.
> Each connected input is saved to the specified directory and filename,
> with the frame number appended to create a sequence.
> For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

and

> "Lists all input sockets of the node.
> The name of each input socket is used to construct the final file name."

and

> "Displays the final file path and extension for each input/layer, based on current settings.
> This helps verify that outputs are named and organized correctly."

Source: manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html. Read 2026-07-30. The last quote describes the Output Paths panel, which shows the final path in the interface.

**The two halves of the path — documented.**

> "directory
> The directory where the image will be written
> Type:
> string, default “”, (never None, blend relative // prefix supported, Supports template expressions)"

> "file_name
> The base name of the file. Other information might be included in the final file name depending on the node options
> Type:
> string, default “”, (never None, Supports template expressions)"

Source: API documentation, https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html. Read 2026-07-30.

**When the frame number is appended — documented, and this changed in 5.0.**

> "The **File Output** node now only appends the frame number to the image name when doing an
> animation render. Rendering through the command line always does an animation render, so no change
> of behavior is expected. Rendering using the `render.render()` operator when `animation=False`
> will see the change of behavior, but setting that to True and providing `start_frame` and
> `end_frame` should restore the old behavior. Finally, adding `####` manually in the name should
> substitute the frame number even in single renders."

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/ — the "Changed" section. Read 2026-07-30. The same change is listed on the bug-fix page as "Compositor File Output: Optional frame numbering" (https://developer.blender.org/docs/release_notes/5.0/bugfixes/, read 2026-07-30).

**What can be put in the name by template — documented, and this is where the question bites.**

> "Currently path templates are only supported for the render output path in
> Scene properties and the output paths in the compositor’s File Output
> node. More filepaths will support templates in future releases."

> "Available in all paths that support template expressions:
> blend_name:
> The currently open blend file’s name (without the .blend).
> blend_dir:
> Path up to (but not including) the currently open blend file."

> "Available only in render output paths (including in the compositor File Output node):
> fps:
> The frames per second of the current scene.
> resolution_x / resolution_y:
> The x and y resolution of the rendered image.
> This factors in the resolution scale as well […]
> scene_name:
> Name of the current scene.
> camera_name:
> Name of the current render camera."

> "Available only in path properties on a node:
> node_name:
> The name of the node that the path property is on."

Source: manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html. Read 2026-07-30. The same page gives the format specifiers (`{fps:###}` and so on), the escape sequences (`{{` and `}}`), and states: "Paths that support templates can have template errors, which prevent the path from being processed."

### What is missing, precisely

Three things decide the name in the case asked about, and the sources settle none of them:

1. **Which frame number is appended.** During a Scene strip render there are two frame counts running: the enclosing scene's timeline position and the frame of the brought-in scene that the strip maps it to. The manual says only "the frame number", with no statement of which scene's frame it means when the two differ. The Scene Strip page confirms the two can differ — the strip has trim and offset settings, and there is an operator "For Scene strips only – Updates the strip’s Time properties to match the referenced scene’s frame range" (manual, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/editing.html, read 2026-07-30) — but nothing connects that to file naming.
2. **Whether it counts as an animation render.** By the release note above, the frame number is only appended "when doing an animation render". Whether a Scene strip rendered inside an enclosing scene's animation render counts as an animation render for the nodes inside the brought-in scene is not stated anywhere I could find.
3. **What "the current scene" means for the templates.** `{scene_name}` is "Name of the current scene" and `{camera_name}` is "Name of the current render camera", and `{fps}` is "The frames per second of the current scene". In a Scene strip render there are two scenes and possibly two cameras — the Scene strip can override the camera ("This can be used to override the scene’s camera with any other object", manual, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/scene.html, read 2026-07-30; and `scene_camera`, "Override the scene’s active camera", API, https://docs.blender.org/api/5.0/bpy.types.SceneStrip.html, read 2026-07-30). Nothing says which one these variables resolve to. Note also that `{fps}` and `{resolution_x}` / `{resolution_y}` have the same problem, since the two scenes can have different frame rates and resolutions.

One part of the name *is* unambiguous even in this case, and I state it as inferred rather than documented. **LABEL for this paragraph only: inferred.** The `//` prefix and `{blend_name}` / `{blend_dir}` are defined against the open blend file, not against a scene ("The currently open blend file’s name"; and for the render output path, "This setting expands Relative Paths where a // prefix represents the directory of the current blend-file", manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html, read 2026-07-30). Both scenes live in one blend file in the case asked about, so these parts cannot differ between them. The step a reader has to supply: that the brought-in scene is in the same blend file. If it is linked from another blend file instead, the sources give `blend_name_lib` and `blend_dir_lib`, "except that for path properties on library-linked data-blocks it will be the name of the library blend file that the data-block comes from" — but they do not say how a Scene strip render interacts with that.

### Not covered

- The whole scene-strip-specific part of the question: which frame number, whether frame numbering happens at all, and what "current scene" and "current render camera" resolve to. Unanswered.
- Whether the Output Paths panel — the one place the interface shows the final name — shows what would actually be written during a Scene strip render, or only what would be written when that scene is rendered directly.
- What happens if two Scene strips using the same scene are on screen at the same enclosing-scene frame, and whether the names would collide.
- Whether the enclosing scene's own output path settings (its file extension setting, its overwrite setting, a `-o` override on the command line) reach the File Output node at all. The command line `-o` is documented as "Set the render path and file name" (manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30) and nothing says it touches File Output nodes.
- Everything in question 16 that this answer depends on. If the brought-in scene's File Output nodes do not write at all, this question does not arise.

### Outside the interface

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

Partly documented, and it matters here. The release note quoted above states: "Rendering through the command line always does an animation render, so no change of behavior is expected." So from the command line the frame number is appended, where an interface single-frame render via the operator with `animation=False` would not append it. Source: release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/, read 2026-07-30. That statement is about File Output nodes generally; the sources do not say whether it still holds for nodes reached through a Scene strip. The rest of the naming — directory, file name, socket name, extension, templates — is described in terms that do not mention the interface at all, and no source says it differs.

### Operating system

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say. Same search as for question 16: "Windows", "macOS", "Linux", "platform", "operating system" across the File Output node page, the File Paths page, the Output properties page, the Scene Strip page, the Post Processing page, the Sequencer Scene page, and the API pages for the File Output node and Scene strips — no hits. Nothing about path separators, path length limits, or characters allowed in names is stated on any of them.

### Where I looked (question 17)

Same whole-source searches as for question 16, plus these aimed at naming: "template expression" (four files across all sources — the File Output node API page, the RenderSettings API page, a property-flags list, and the File Paths manual page), "frame number", "####", "current scene", "frame_current", "frame_subframe", "subframe", "sub-frame", "scene_name", "camera_name", "blend_name", "relative path", "-o, --render-output".

Pages opened for this answer: the File Output node page; the File Paths page in full; the Output properties page; the Rendering Animations page; the command line arguments page (the `-o`, `-a`, `-b`, `-S`, `-f` entries); the Scene Strip page; the strip editing page ("Update Scene Frame Range"); the API pages for `CompositorNodeOutputFile`, `NodeCompositorFileOutputItem`, `SceneStrip`, `RenderSettings`, `Scene` and `bpy.ops.render`; the compositor, Python API and bug-fix release notes.

Ruled out, and why:
- **The scene's own render output path** (`filepath`, "Directory/name to save animations, # characters define the position and padding of frame numbers", API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html, read 2026-07-30, and the Output properties manual page): this names the file the *render* writes, not the file a File Output node writes. Ruled out as a different file, though it shares the `#` padding convention and the template support.
- **The Media Type setting on the File Output node** (Image versus Multi-Layer EXR, manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html): it changes how many files there are and whether socket names become layers inside one file rather than parts of file names. Documented there, but it is the same in or out of a Scene strip as far as any source says, so it does not bear on what the Scene strip case changes.
- **`bpy.ops.node.file_output_*` operators** (API, https://docs.blender.org/api/5.0/bpy.ops.node.html): these add and remove the node's inputs. Nothing about naming during a render.

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

These are written so somebody who cannot see my reasoning can carry them out. Each one settles something the sources leave open.

**Setup used by all of them.** One blend file, saved to disk, with two scenes. Call them `Shot` and `Edit`. In `Shot`: any visible object and a camera, frame range 1 to 10. In `Shot`'s compositor, add a File Output node, connect the Render Layers node's Image output to it, set its Directory to `//out/` and its File Name to `shot_`. Leave Media Type on Image. In `Edit`: open the Video Sequencer, Add ‣ Scene ‣ `Shot`, so there is one Scene strip. Set `Edit`'s frame range to 1 to 10 as well. In `Edit`'s Output properties, under Post Processing, make sure both Compositing and Sequencer are ticked. Set `Edit`'s own output path to somewhere separate, e.g. `//edit/`. Empty the `out` folder before each test.

1. **Does the brought-in scene's File Output node write at all?** With `Edit` as the active scene, use Render ‣ Render Animation from the top bar (not the sequencer's own Sequence Render Animation). When it finishes, look in the `out` folder. Files present or absent is the answer to question 16.

2. **If files appear, what are they called?** Note the exact names, in particular whether a frame number is present, how many digits it has, and whether the socket name appears between the file name and the number. That is question 17.

3. **Which scene's frame number?** Move the Scene strip in `Edit` so it starts at frame 5 instead of frame 1 (drag it right by four frames), leaving `Shot`'s own range at 1 to 10 and extending `Edit`'s range to cover the strip. Empty `out`, render `Edit`'s animation again, and look at the numbers on the files. If they run 1…10 the number comes from the brought-in scene; if they run 5…14 it comes from the enclosing scene.

4. **Which scene counts as "the current scene" for templates?** Change the File Output node's File Name in `Shot` to `{scene_name}_{camera_name}_`. Empty `out`, render `Edit`'s animation, and read the resulting file names. They will contain either `Shot` or `Edit`, and either `Shot`'s camera name or whatever camera the strip overrides with. Repeat with a camera override set on the strip (Scene strip properties ‣ Camera) to see whether `camera_name` follows the override.

5. **Whose Compositing tick box governs it?** Untick Compositing in `Shot`'s Output ‣ Post Processing, leaving `Edit`'s ticked. Empty `out`, render `Edit`'s animation, check whether files appear. Then swap: re-tick `Shot`'s, untick `Edit`'s, and repeat. Four combinations in total; the pattern of which ones write is the answer.

6. **Does it count as an animation render?** With everything back to the starting setup, render a *single* frame of `Edit` (Render ‣ Render Image) rather than an animation, and see whether a file appears in `out` and whether it carries a frame number. Then change `Shot`'s File Name to `shot_####_` and repeat, since the release note says `####` in the name substitutes the frame number even in single renders.

7. **Does the sequencer's own render command skip it?** With `Edit` active and the Video Sequencer open, use the sequencer's own Sequence Render Animation (View menu of the preview) instead of the top-bar Render menu. Empty `out` first. The manual says that command does not render Scene strips; check whether `out` stays empty, which would confirm nothing in the brought-in scene's compositor ran.

8. **Two strips of the same scene, and overlaps.** Duplicate the Scene strip so two strips reference `Shot` at different times on different channels, overlapping for part of their length. Empty `out`, render `Edit`'s animation, and count the files and check for gaps or overwrites. This settles whether the same name is written twice.

9. **Does it work with no interface?** Repeat test 1 from a terminal, rendering the same file in background mode with the animation flag and the scene flag set to `Edit`. Compare the contents of `out` with what test 1 produced. This settles whether the answer differs when Blender is driven from outside its own window.

10. **Same thing on another operating system.** Repeat test 1 and test 9 on a second operating system with the same blend file and compare, since no source states whether any of this is the same everywhere.

---


## The researcher's own list of things to settle by hand — all of them now run

**Closed on 2026-08-26.** Every check proposed above was carried out, together with the tests the audit sent on and several more, in all four researched versions of Blender and on two computers. The results are written into questions 16 and 17 above as observed findings, and the whole list, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`.

**Only these are still open, and neither can be reached from the equipment this project has:**

- **Windows.** No Windows machine has run it, so every answer above covers macOS on Apple silicon and Ubuntu Linux and leaves Windows untested.
- **The four tests that need Blender's own window, on macOS.** They were run on the rented Linux machine, which has a display server, and not on the Creator's Mac. So the windowed render, the played preview, the sequencer's own render command with a window, and the node's sidebar panel are settled on Linux alone. Each of the first three has a terminal counterpart that was run on both machines.
- **A folder reached over a network**, which has been on this topic's *cannot be run* list from the start: neither machine has a network store.

*(The checks the researcher proposed are left in place above this note rather than deleted, so a reader can see what was asked as well as what came back.)*


---

2026-08-26 — Claude Opus 5 (Claude Code subagent) — File output nodes, Blender 5.0.
