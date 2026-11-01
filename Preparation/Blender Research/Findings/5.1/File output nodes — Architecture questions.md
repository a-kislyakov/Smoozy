# File output nodes — Architecture questions (Blender 5.1)

**Sources consulted, and when.** Blender's manual for 5.1 (`Blender Sources/manual_5.1`, including its `_images` folder), Blender's Python API documentation for 5.1 (`Blender Sources/api_5.1`), the Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files for 5.1 (`Blender Sources/cycles_5.1`). All were taken on 2026-07-30, and that is the date read for everything quoted below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered here: **16 and 17** only.

---

## 16. When a scene is brought into another scene through its video sequence editor, what do the File Output nodes belonging to the brought-in scene do?

**LABEL: not in the sources** — for the direct question. Nothing in the 5.1 manual, the 5.1 API documentation, the 5.1 release notes or the Cycles files says what a File Output node in a scene does when that scene is used as a Scene strip in another scene's sequencer. Below I give the pieces that do exist, and one inference clearly marked as such.

### The direct answer

There is no sentence anywhere in the sources that puts "File Output node" and "Scene strip" together. I checked this two ways. First, the words "File Output" appear in only five files of the whole 5.1 manual, and three of those are index lists:

- `compositing/types/output/file_output.txt` (the node's own page)
- `compositing/index.txt` (table of contents)
- `compositing/types/output/index.txt` (table of contents)
- `files/file_paths.txt` (path templates)
- `genindex.txt` (index)

None of them mentions Scene strips, the sequencer, or a scene being used inside another scene. Second, of the fifteen manual files that mention "scene strip", none mentions the File Output node.

### What the sources do say, that bears on it

**(a) A Scene strip is the render of another scene.**

> "Scene strips are a way to insert the render output of another scene into your sequence. Instead of rendering out a video, then inserting the video file, you can insert the scene directly."

Source: manual — https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/scene.html — read 2026-07-30. **LABEL: documented.**

The API says the same in one line:

> "Sequence strip using the rendered image of a scene"

Source: API documentation — https://docs.blender.org/api/5.1/bpy.types.SceneStrip.html — read 2026-07-30. **LABEL: documented.**

**(b) When the sequencer pipeline runs, Scene strips are rendered as part of it, and the compositor is involved.**

> "Renders the output of the Video Sequence editor, instead of the view from the 3D scene’s active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html — read 2026-07-30. **LABEL: documented.**

This is the closest the sources come to the question. Note carefully what it does *not* say: it does not say whose "Compositing" switch is meant — the sequencer scene's, or the brought-in scene's. See "Ambiguity" below.

**(c) The compositor only runs at all if the Compositing switch is on.**

> "To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output’s Post Processing properties."

Source: manual — https://docs.blender.org/manual/en/5.1/compositing/introduction.html — read 2026-07-30. **LABEL: documented.**

And the switch itself:

> "Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)"

Source: API documentation, `RenderSettings.use_compositing` — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30. **LABEL: documented.**

The companion switch:

> "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist (default True)"

Source: API documentation, `RenderSettings.use_sequencer` — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30. **LABEL: documented.**

**(d) Cycles draws both switches from a scene's own render settings.** The panel's own code reads them off the scene in front of it:

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
```

Source: Cycles' own files, `Blender Sources/cycles_5.1/ui.py`, class `CYCLES_PT_post_processing`, beginning at line 1112 (the three quoted lines that matter are 1122, 1125 and 1126). Read 2026-07-30. **LABEL: observed** — this is what the file says, and no more: it shows the panel drawing two named settings off the scene it is given. That each scene therefore carries its own pair of switches is a one-step reading of `context.scene.render`, so treat that as **inferred**. What a render then does with either switch is not in this file at all, and nothing here should be read as saying it.

The manual's picture of the panel shows the two switches, both ticked:

Source: manual picture `_images/render_output_properties_post-processing_panel.png`, shown on https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html — read 2026-07-30. **LABEL: documented.** What is visible: a panel headed "Post Processing" with two ticked checkboxes labelled "Compositing" and "Sequencer", and a "Dither" slider reading 1.000.

**(e) A scene can have its own compositor node tree.**

> "Chooses which node group (compositor node tree) to display and edit. Each Scene can have its own compositor node tree."

Source: manual — https://docs.blender.org/manual/en/5.1/editors/compositor.html — read 2026-07-30. **LABEL: documented.**

**(f) A File Output node writes during rendering or compositing.**

> "The File Output node writes one or more images to disk during rendering or compositing."

Source: manual — https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — read 2026-07-30. **LABEL: documented.**

**(g) One setting, off by default, makes Scene strips take their Workbench render settings from the sequencer scene instead of their own.** It is about Workbench only:

> "Use workbench render settings from the sequencer scene, instead of each individual scene used in the strip (default False)"

Source: API documentation, `RenderSettings.use_sequencer_override_scene_strip` — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30. **LABEL: documented.** It speaks only about Workbench render settings. It does not say anything about the Compositing switch.

Reading this as a general rule — that a Scene strip uses its own scene's settings unless something overrides them — is a step past the quote, and the quote covers only one kind of setting. **LABEL: inferred**, and weakly: the step is that a setting existing to override one thing implies that thing is otherwise taken per scene, and that what holds for Workbench holds for the rest. No source states either half.

**(h) Scene strips are restricted to the Combined pass.**

> "Scene strips do not render individual Render Passes; only the Combined render pass will be used."

Source: manual — https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/scene.html — read 2026-07-30. **LABEL: documented.**

**(i) Scene strips are rendered during playback too, not only during a render.**

> "When used as strips, scenes are rendered dynamically during playback or rendering, and they share the same data-blocks unless they were created as full copies."

Source: manual — https://docs.blender.org/manual/en/5.1/scene_layout/scene/introduction.html — read 2026-07-30. **LABEL: documented.**

**(j) One route through the interface does not render Scene strips, by default.**

> "Sequence Render Image and Sequence Render Animation don’t render the final video by default – specifically, they don’t render Scene Strips, instead using the preview’s shading mode (which is initially Solid)."
>
> "To output a video where the Scene Strips are rendered, use the Render menu in the top-bar, or change Sidebar ‣ View ‣ Scene Strip Display ‣ Shading to Rendered."

Source: manual — https://docs.blender.org/manual/en/5.1/editors/video_sequencer/sequencer/navigating.html — read 2026-07-30. The same two sentences also appear on https://docs.blender.org/manual/en/5.1/editors/video_sequencer/preview/header.html. **LABEL: documented.**

### The one inference I am prepared to make

**LABEL: inferred.** If the brought-in scene's compositor is executed when that scene is rendered as a Scene strip, then the File Output nodes in it write their files, just as they would in an ordinary render.

The step, written out: (b) says the Scene strip "will be the output of the Compositor" when Compositing is enabled — so the compositor is executed for the strip. (f) says the File Output node "writes one or more images to disk during rendering or compositing". Put those together and the nodes write. The reader has to supply the assumption that nothing suppresses File Output nodes in this particular situation — and no source states either that something does or that nothing does. That assumption is the whole gap, and it is why this is inferred and not documented.

Two further inferences, both weaker, both flagged:

- **LABEL: inferred.** Via the sequencer's own "Sequence Render Image" / "Sequence Render Animation" commands with default settings, the brought-in scene is not rendered at all — (j) says these "don’t render Scene Strips" — so its compositor does not run and its File Output nodes write nothing. Step supplied by the reader: that "not rendered" means the scene's compositing pipeline is not executed. The sources do not say that in so many words.
- **LABEL: inferred.** A File Output node in the brought-in scene that is fed from individual render passes may not receive them, because (h) says Scene strips "do not render individual Render Passes". Step supplied by the reader: that "do not render" means the passes are never produced, rather than merely that the strip displays only Combined. The sentence supports both readings.

### Ambiguity in the question and in the source

The question is clear enough. The **source** is the ambiguous part, in one place that matters: in quote (b), "If Compositing is also enabled" does not say whose Compositing switch. Two readings:

1. The **brought-in scene's** switch. Under this reading, each Scene strip is rendered through its own scene's compositing pipeline, and File Output nodes in that scene's tree write. It sits comfortably beside (d) and (e), which show a compositing tree and a pair of switches attached to a scene rather than to the file. It gets no support from (g), which is about Workbench settings and says nothing about compositing.
2. The **sequencer scene's** switch — the scene the panel being documented belongs to. Under this reading the sentence would be describing the sequencer scene's own compositor, and it would say nothing at all about the brought-in scene's File Output nodes.

I have not chosen between them, because nothing in the sources settles it. Reading 1 is what the sentence's subject ("the Scene strip will be the output of the Compositor") points at, but that is a judgement about English, not a documented fact.

### Record of where I looked (question 16)

All searches were run over the whole of `Blender Sources/manual_5.1`, `Blender Sources/api_5.1`, `Blender Sources/release_notes_5.1` and `Blender Sources/cycles_5.1`, case-insensitively, unless noted.

Terms searched, and what came back:

- **"file output"** — manual: five files only (`compositing/types/output/file_output.txt`, `compositing/index.txt`, `compositing/types/output/index.txt`, `files/file_paths.txt`, `genindex.txt`). API: `bpy.types.CompositorNodeOutputFile.txt` plus index pages. Release notes: two lines in `compositor.md`, two in `bugfixes.md`. Cycles files: nothing.
- **"OutputFile" / "FileOutput"** — API only: `bpy.types.CompositorNodeOutputFile.txt`, `bpy.types.NodeCompositorFileOutputItem.txt`, `bpy.types.CompositorNode.txt`, `bpy.types.NodeInternalSocketTemplate.txt`, `bpy.types.txt` and the generated indexes. Opened the first two; the rest are lists of names.
- **"scene strip"** — manual: fifteen files. Opened all the ones that could plausibly carry behaviour: `video_editing/edit/montage/strips/scene.txt`, `video_editing/sequencer_scene.txt`, `video_editing/edit/montage/editing.txt`, `video_editing/edit/montage/strips/introduction.txt`, `editors/video_sequencer/sequencer/navigating.txt`, `editors/video_sequencer/preview/header.txt`, `editors/video_sequencer/preview/sidebar.txt`, `editors/video_sequencer/sequencer/sidebar/cache.txt`, `scene_layout/scene/introduction.txt`, `interface/window_system/workspaces.txt`, `render/output/properties/post_processing.txt`. Ruled out without opening in full: `editors/dope_sheet/display/overlays.txt`, `video_editing/edit/montage/strips/effects/alpha_over_under.txt`, `video_editing/edit/montage/strips/index.txt`, `video_editing/storyboarding/index.txt` — their matching lines are about overlay colours, blending and tables of contents. API: seven files, of which only `bpy.types.SceneStrip.txt` and `bpy.types.RenderSettings.txt` carry anything; the others are theme colours, operator listings and a dope-sheet overlay.
- **Co-occurrence test.** Of the manual files containing "File Output", the ones that also contain "strip" are `genindex.txt` and `compositing/index.txt` — both tables of contents. The ones that also contain "sequence" are those two plus `file_output.txt` and `file_paths.txt`, where "sequence" means a numbered image sequence, not the video sequencer. So no page discusses the two together.
- **"post processing" / "post-processing"** — led to `render/output/properties/post_processing.txt`, which I opened in full, along with its picture.
- **"referenced scene"** — three hits, all in `video_editing/`, all about frame ranges and workspaces. Ruled out.
- **"nested scene", "sub-scene", "subscene"** — no hits anywhere.
- **"written to disk" / "writes … to disk"** — manual: only `compositing/types/output/file_output.txt`.
- **"sequencer" inside the manual's `render/` folder** — eight files. Opened `render/output/introduction.txt`, `render/output/animation.txt`, `render/output/properties/post_processing.txt`, `render/output/properties/output.txt`. Checked the matching lines of `render/output/properties/metadata.txt` (metadata source: scene versus sequencer strips), `render/cameras.txt` (safe areas), `render/color_management/color_spaces.txt` and `render/color_management/opencolorio.txt` (the Sequencer's colour space) and ruled all four out as unrelated to File Output.
- **"sequencer" inside `cycles_5.1`** — one hit, `ui.py`, quoted above. Nothing else in Cycles mentions the sequencer, scene strips or File Output.
- **Release notes for 5.1** — read `compositor.md` and `sequencer.md` in full, and `pipeline_io.md`. Searched `index.md` and the rest for "scene strip" and "file output"; the only hits are the four listed above, two of which are bug-fix one-liners about a socket dimension and a missing link-drag-search entry.

Pages opened in full for this question: the File Output node page, the Scene Strip page, the Sequencer Scene page, the Post Processing page, the compositing introduction, the Compositor editor page, the Scenes introduction, the VSE preview header and sidebar, the VSE sequencer navigating page, the render output introduction and animation pages, the command-line arguments and command-line render pages, `bpy.types.SceneStrip`, `bpy.types.CompositorNodeOutputFile`, `bpy.types.NodeCompositorFileOutputItem`, the relevant parts of `bpy.types.RenderSettings` and `bpy.ops.render`, and `cycles_5.1/ui.py` around the Post Processing panel.

### Not covered

- Whether Blender actually executes the brought-in scene's File Output nodes. Everything above is assembled from two separate pages; no page states the behaviour.
- Whether they write once per frame of the master timeline, once per frame of the brought-in scene, or not at all.
- What happens during **playback** or in the sequencer **preview**, where (i) says scenes are still "rendered dynamically". Nothing says whether files are written then. This is the sharpest gap: if they are, scrubbing a timeline would write files.
- Whether the Scene strip's `scene_input` setting (`CAMERA` versus `SEQUENCER`, documented at https://docs.blender.org/api/5.1/bpy.types.SceneStrip.html) changes any of this. Nothing says.
- Whether the brought-in scene being reached more than once (several strips referencing the same scene, or nested Scene strips) changes anything. Nothing says.
- What happens when the brought-in scene's Compositing switch is off but the sequencer scene's is on, or the reverse.
- The Compositor **strip modifier** — a separate feature, mentioned at https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strip_modifiers.html — is a different route by which a compositing tree runs inside the sequencer. It is not a File Output node "belonging to the brought-in scene", so it is outside this question, and I did not pursue it.

### Outside the interface

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say. No page distinguishes what a File Output node does when Blender is driven from outside its window from what it does inside.

What is documented, and is as close as the sources get:

- The render operator can be aimed at a named scene and at the sequencer scene from a script, with no window involved:
  > "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"
  > "use_sequencer_scene (bool) – Use Sequencer Scene, Render the sequencer scene instead of the active scene (optional)"
  > "scene (str) – Scene, Scene to render, current scene if not specified (optional, never None)"

  Source: API documentation — https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-07-30. **LABEL: documented.**

- From the command line, the scene to render can be chosen and Blender can be run with no window:
  > "-S, --scene <name>" / "Set the active scene <name> for rendering."

  Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. **LABEL: documented.**

  > "-b" / "Render in the background (without UI)."

  Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read 2026-07-30. **LABEL: documented.**

- The one in-window route documented to skip Scene strips, quote (j), has a scriptable counterpart:
  > "sequencer (bool) – Sequencer, Render using the sequencer’s OpenGL display (optional)"

  Source: API documentation, `bpy.ops.render.opengl` — https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-07-30. **LABEL: documented.** That this operator is the same thing as the "Sequence Render Image / Animation" menu items is **inferred** from the matching description, not stated.

So the distinction the sources actually draw is between two *render routes*, not between inside and outside the window. Whether the answer to question 16 differs inside and outside: the sources do not say.

### Operating system

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say. Nothing on any page consulted attaches an operating system to Scene strips, to the Post Processing switches, or to the File Output node. Blender's per-version requirements page is not in the sources for 5.1; the only requirements file present is dated and not version-specific, so I did not use it.

---


### What the hand tests settled — observed, 2026-08-26

**LABEL: observed.** Nothing in this section comes from reading. It was got by running Blender, by Claude Opus 5 (Claude Code subagent), on 2026-08-26, in Blender 5.1.2. The full list of tests, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`; what follows stands on its own and does not need it.

**The two computers, named in full.**

- **The Creator's own MacBook Pro** — Apple M1 Pro chip, macOS 26.5.2, Apple's own graphics, no second card. Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`.
- **A rented Linux machine, in Norway** — a Docker container holding half its host, Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 3060 with 12 GB and driver 595.84, about fifteen cores of an Intel Xeon E5-2697A v4, driven over SSH, with a display server of its own on `DISPLAY=:1` at 1920×1080. Blender 5.1.2 at `/opt/blender/5.1.2/blender`.

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

**The control that every other result rests on.** Rendering `Inner` on its own — `"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup <file>.blend -S Inner -a` — put `inner_Img0001.png`, `inner_Img0002.png`, `inner_Img0003.png`, `inner_Img0004.png` and `inner_Img0005.png` into `watch`, on both machines. So an empty folder in any test below can only mean that test's own change stopped the writing.

**The answer to the question.** **The brought-in scene's File Output node writes.** Rendering the whole of `Outer`'s animation from a terminal — `"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup <file>.blend -S Outer -a`, and `/opt/blender/5.1.2/blender -b --factory-startup <file>.blend -S Outer -a` on the rented machine — left one file in the `watch` folder, `inner_Img.png`, written over five times, with no frame number in the name, in Blender 5.1.2, **on both machines, with the same names**. In the same run the containing scene's own render was written as `outer_0021.png` through `outer_0025.png`.

Blender's own words on the Mac, the first two of the five frames, word for word — the last three read the same with the timestamps and the outer frame number moving on:

```
00:00.347  render           | Fra: 1 | Compositing
00:00.416  render           | Saved: '/private/tmp/fw/mac/t02/5.1/watch/inner_Img.png'
00:00.430  render           | Saved: '/private/tmp/fw/mac/t02/5.1/watch_outer/outer_0021.png'
00:00.438  render           | Fra: 2 | Compositing
00:00.444  render           | Saved: '/private/tmp/fw/mac/t02/5.1/watch/inner_Img.png'
00:00.457  render           | Saved: '/private/tmp/fw/mac/t02/5.1/watch_outer/outer_0022.png'
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

**The sequencer's own render command.** From a terminal with no window at all, `bpy.ops.render.opengl(animation=True, sequencer=True)` and its single-frame form both came back `{'CANCELLED'}` and wrote nothing, in Blender 5.1.2, on both machines — so those commands cannot be reached from outside Blender's window. With a real window on the rented Linux machine, and a Video Sequencer preview in front of it, the same command still came back `{'CANCELLED'}` and wrote nothing — at Solid shading and at Rendered shading alike. That windowed run is on Linux only.

**Outside the interface — now answered by test.** The answer is the same whether Blender is driven from a terminal with no window or from a real window. The same file rendered with a real window open on the rented Linux machine — `DISPLAY=:1 XDG_RUNTIME_DIR=/run/user/0 /opt/blender/5.1.2/blender --factory-startup <file>.blend`, with the render started from inside it — left exactly the same files with exactly the same names as the terminal run. The one place the two routes differ is the sequencer's own render command, above, which does not exist outside the window at all.

**Operating system — now answered by test, for two systems.** Every test in the table above was run on both computers, and **the Mac and the rented Linux machine agreed on every one**: the same file names, the same counts, the same printed lines, the same refusals. The only differences anywhere in the two logs are line numbers, timestamps and the folders the work sat in. **Windows is untested** — no Windows machine has run it — so the answer covers macOS on Apple silicon and Ubuntu Linux and says nothing about Windows. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**What the hand tests still leave open for this question.** Windows, in every version. And the four tests that need Blender's own window — the windowed render, the played preview, the sequencer's own render command with a window, and the node's sidebar panel — were run on the rented Linux machine only, never on macOS, because the Creator was given no test to run.


---

## 17. What is a file written by a File Output node called, when that node belongs to a scene brought into another scene through its video sequence editor?

**LABEL: not in the sources** — for the scene-strip part. No source states any naming rule specific to this situation, or says that the ordinary rule is or is not changed by it. What follows is the ordinary rule, documented, plus the exact points at which the ordinary rule stops being decidable in this situation.

### The ordinary naming rule (documented)

> "The File Output node writes one or more images to disk during rendering or compositing. Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence. For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

> "Both the File Name and the Image names (when saving multiple images) can contain #### to force the frame number to appear in a specific place in the filename. This also has the effect that a single render will have the frame number embedded. The frame number will be padded with zeros to fill the number of # characters."

> "When the Media Type is set to Image, input names are also used to construct subdirectories: If an input name contains a slash character (/ or \\), the path is interpreted as a directory hierarchy. Any missing directories are created automatically during output."

> "Lists all input sockets of the node. The name of each input socket is used to construct the final file name."

Source: manual — https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — read 2026-07-30. **LABEL: documented.**

The node also shows the result up front:

> "Displays the final file path and extension for each input/layer, based on current settings. This helps verify that outputs are named and organized correctly."

Same page, same date. **LABEL: documented.**

The 5.1 release notes record one change to the naming machinery:

> "The **File Output** node now allows specifying directories in item names. This was possible in older versions of Blender and the functionality is now restored."

Source: release notes — https://developer.blender.org/docs/release_notes/5.1/compositor/ — read 2026-07-30. **LABEL: documented.**

### Path templates, and why they are the crux here

The node's directory and file name accept template expressions:

For `directory` — "The directory where the image will be written (default “”, never None, blend relative // prefix supported, Supports template expressions)"

For `file_name` — "The base name of the file. Other information might be included in the final file name depending on the node options (default “”, never None, Supports template expressions)"

Source: API documentation — https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30. **LABEL: documented.**

The templates, and the variables available:

> "Path templates substitute template expressions (written {abcd}, where “abcd” is a variable name) in a filepath when the path is used."

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor’s File Output node. More filepaths will support templates in future releases."

> "Available in all paths that support template expressions: blend_name: The currently open blend file’s name (without the .blend). blend_dir: Path up to (but not including) the currently open blend file. blend_name_lib: Like blend_name, except that for path properties on library-linked data-blocks it will be the name of the library blend file that the data-block comes from. blend_dir_lib: Like blend_dir, except that for path properties on library-linked data-blocks it will be the path to the library blend file that the data-block comes from."

> "Available only in render output paths (including in the compositor File Output node): fps: The frames per second of the current scene. resolution_x / resolution_y: The x and y resolution of the rendered image. This factors in the resolution scale as well, so if the scene resolution is 1000x600 and the scale is 50%, then resolution_x and resolution_y will be 500 and 300, respectively. scene_name: Name of the current scene. camera_name: Name of the current render camera."

> "Available only in path properties on a node: node_name: The name of the node that the path property is on."

Source: manual — https://docs.blender.org/manual/en/5.1/files/file_paths.html — read 2026-07-30. **LABEL: documented.** (The source lays these out as a definition list; the words are exact but I have run the lines together to quote them here.)

Three of these are defined in terms of "the current scene" or "the current render camera". The sources never define which scene is "current" while a brought-in scene's compositor is executed as part of another scene's sequencer render. That is the heart of question 17, and it is unanswered. The candidates are the sequencer scene (the one being rendered) and the brought-in scene (the one the node belongs to). Nothing in the manual, the API documentation, the release notes or the Cycles files chooses between them.

Note also that the templates apply to the node's **directory** and **file name** only. The per-input item name carries no such note:

For the item's `name`, the whole entry reads "(default “”, never None)" followed by "Type:" and "str" — with no mention of template expressions.

Source: API documentation, `NodeCompositorFileOutputItem` — https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html — read 2026-07-30. **LABEL: documented** for the words of that entry, quoted above. Whether the item name accepts templates is **not in the sources** — no page says either way — and reading the missing note as a refusal is **inferred**, the step being that the page would have carried the note if it applied.

### What is undecided, specifically

1. **Which scene `{scene_name}` names.** Not in the sources, as above.
2. **Which camera `{camera_name}` names.** Not in the sources. Complicated further by the Scene strip's own camera override, which is documented but never connected to the template: "Override the scene’s active camera" (`SceneStrip.scene_camera`, https://docs.blender.org/api/5.1/bpy.types.SceneStrip.html, read 2026-07-30) and "This can be used to override the scene’s camera with any other object." (https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/scene.html, read 2026-07-30). **LABEL for both quotes: documented**; the connection to `{camera_name}` is not made anywhere and I do not make it.
3. **Which frame number is appended.** The node page says "the frame number" without saying which timeline it belongs to. A Scene strip sits at some position on the master timeline and maps to the brought-in scene's own frames — the strip carries start, trim and duration properties (`content_start`, `content_trim_start`, `left_handle`, and so on, https://docs.blender.org/api/5.1/bpy.types.SceneStrip.html, read 2026-07-30) — but no source says which of the two numbers reaches the file name.
4. **What `{fps}`, `{resolution_x}` and `{resolution_y}` resolve to**, since these too are defined against "the current scene" / "the rendered image".
5. **Whether any name is produced at all**, which depends entirely on the answer to question 16.

`{blend_name}`, `{blend_dir}` and `{node_name}` are the three that do not depend on which scene is current: the blend file is the same either way, and the node is the node. **LABEL: inferred** — the step is that a Scene strip references a scene inside the same blend file, which the manual implies ("Existing scenes can be added from the Add ‣ Scene ‣ “Scene Name”.", same scene.html page) but does not state as a rule; scenes appended from an Asset Library are documented on that page too, and once appended they are in the current file. This holds only for a scene that is in the current file. Whether a Scene strip can point at a scene in another blend file is not in the sources, and if it can, none of the three is settled — see "Not covered" below.

### Record of where I looked (question 17)

The same whole-tree searches as for question 16 apply, plus these, all run over `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1`:

- **"template expression"** — four files: `manual_5.1/files/file_paths.txt` (opened in full), `api_5.1/bpy.types.CompositorNodeOutputFile.txt` (opened in full), `api_5.1/bpy.types.RenderSettings.txt` (read the `filepath` entry), and `api_5.1/bpy_types_enum_items/property_flag_items.txt` (the generic flag list, ruled out as it defines the flag rather than its use).
- **"frame number"** — read `render/output/properties/output.txt` for the general padding rule and `compositing/types/output/file_output.txt` for the node's. Neither says which timeline the number comes from when a scene is used as a strip.
- **"scene_name" / "camera_name"** — only `files/file_paths.txt`. No other page defines them or qualifies "the current scene".
- **"Output Paths"** — only the node's own page.
- **`NodeCompositorFileOutputItem`** — read in full to check whether the per-input name carries a template note. It does not.
- **Release notes** — the only naming-related entry for 5.1 is the restored ability to put directories in item names, quoted above. Nothing about scene strips and naming.

Ruled out and why: the render output path pages (`render/output/properties/output.txt`, `render/output/properties/metadata.txt`) describe the scene's own output path and burn-in metadata, not the File Output node's naming, and neither mentions scene strips. The Multi-Layer EXR and Saving Images pages describe formats and extensions, not which scene supplies a name.

### Not covered

- Everything listed under "What is undecided" above.
- Collisions: whether two Scene strips referencing the same scene, or the same scene reached at two points on the master timeline, would produce the same name twice. Not in the sources.
- Whether the "Output Paths" panel on the node, which "Displays the final file path and extension for each input/layer, based on current settings", shows the name that would be used in this situation. The panel is documented; its behaviour for a scene used as a strip is not.
- The Multi-Layer EXR case, where "Saves all inputs together in a single multi-layer OpenEXR file" (same node page) — the naming pieces contributed by input names change, and nothing says whether the scene-strip situation affects that differently.
- A brought-in scene that lives in **another** blend file rather than this one. The template list quoted above carries two variables for exactly that case — `blend_name_lib` and `blend_dir_lib`, "for path properties on library-linked data-blocks" — so the sources plainly contemplate a data-block coming from elsewhere. Nothing consulted says whether a scene used as a Scene strip can be one of those, and the paragraph above about `{blend_name}`, `{blend_dir}` and `{node_name}` only covers a scene added or appended into the current file. So what any of the blend-file variables resolve to in that case is not established.

### Outside the interface

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say. The naming rules quoted are stated without reference to how Blender was started, and no page gives a different rule for a scripted or command-line render. Two documented facts that are adjacent:

- Path templates are substituted at the moment the path is used, not stored: "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field." (https://docs.blender.org/manual/en/5.1/files/file_paths.html, read 2026-07-30). **LABEL: documented.**
- A broken template stops the render: "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors." (same page and date). **LABEL: documented.** The page also shows the error being surfaced by a red field and a hover tooltip — pictures `_images/files_file_paths-invalid_path_template.png` and `_images/files_file_paths-invalid_path_template_tooltip.png` on that page. Those two signals are visibly parts of the window, so they cannot be seen when Blender is driven from outside it; that last sentence is **inferred**, from the pictures showing a field and a tooltip.

### Operating system

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say, with one exception that is genuinely relevant here. The File Output node's input names accept either slash as a directory separator:

> "If an input name contains a slash character (/ or \\), the path is interpreted as a directory hierarchy."

Source: manual — https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — read 2026-07-30. **LABEL: documented.** Both separators are accepted; the page does not tie either to an operating system, and does not say what the written path looks like on each.

Nothing else consulted attaches an operating system to File Output naming or to path templates.

---


### What the hand tests settled — observed, 2026-08-26

**LABEL: observed.** Nothing in this section comes from reading. It was got by running Blender, by Claude Opus 5 (Claude Code subagent), on 2026-08-26, in Blender 5.1.2, on the two computers described under question 16 above — the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2) and a rented Linux machine in Norway (Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 3060, driver 595.84, driven over SSH). The same build on both, so a difference between them would be the operating system. **The two are never merged**; where both ran a test, both results are given. The set-up is the one described under question 16: a scene `Inner` at 24 frames per second and 320 × 180 with frames 1 to 5 and a camera `InnerCam`, brought into a scene `Outer` at 30 frames per second and 640 × 360 with frames 21 to 25 and a camera `OuterCam`, by a Scene strip placed at outer frame 21 — so no part of a name can be ambiguous about which scene it came from. The full list of tests, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`.

**What the file is called.** Rendering the whole of `Outer`'s animation from a terminal left one file in the node's folder, `inner_Img.png`, written over five times, with no frame number in the name, in Blender 5.1.2, on both machines.

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

**Outside the interface — now answered by test.** The names are the same whether Blender is driven from a terminal with no window or from a real window. The same file rendered with a real window open on the rented Linux machine gave exactly the same names as the terminal run, in Blender 5.1.2.

**Operating system — now answered by test, for two systems.** Every naming test was run on both computers and **every name came back character for character the same**. **Windows is untested**, no Windows machine having run it, so the answer covers macOS on Apple silicon and Ubuntu Linux only.

**What the hand tests still leave open for this question.** Windows, in every version; and the node's sidebar panel, which was read on the rented Linux machine only and never on macOS.


---

## Things a person with Blender open could settle in a few minutes

These are for someone sitting at a computer running Blender 5.1. Each is written so it can be carried out without reading anything above.

1. **Does a Scene strip make the brought-in scene's File Output node write?**
   Make a new blend file with two scenes, "Shot" and "Edit". In "Shot": put any object in front of the camera, set frames 1 to 5, open the Compositor, tick New to make a node tree, add a File Output node, connect the Render Layers node's Image socket to it, set its Directory to an empty folder you can watch, and in Properties ‣ Output ‣ Post Processing make sure Compositing is ticked. In "Edit": open a Video Sequencer, Add ‣ Scene ‣ Shot, set frames 1 to 5, set an output path, and make sure Post Processing has both Compositing and Sequencer ticked. With "Edit" active, press Render Animation from the top-bar Render menu. Then look in the watched folder. Report: were any files written, how many, and what exactly are they called.

2. **Which frame number lands in the name.**
   Repeat test 1, but drag the Scene strip so it starts at frame 20 on the "Edit" timeline instead of frame 1, and set "Edit"'s frame range to 20–24. Render Animation from the top-bar. Report the exact file names. If they read 0001–0005 the brought-in scene's own frames are used; if they read 0020–0024 the master timeline's are.

3. **Which scene `{scene_name}` means.**
   Repeat test 1, but set the File Output node's File Name to `{scene_name}_` and its Directory to the watched folder. Render Animation from "Edit" with the top-bar Render menu. Report the exact file names — whether they contain "Shot" or "Edit". Do the same with `{camera_name}`, and again with `{camera_name}` after setting the Scene strip's Camera override (Sidebar ‣ Strip ‣ Scene ‣ Camera) to a second camera with a distinctive name.

4. **Whose Compositing switch decides.**
   Run test 1 four times, with the Compositing switch in Properties ‣ Output ‣ Post Processing set on/on, on/off, off/on and off/off across the two scenes ("Shot" first, "Edit" second). Report for each of the four whether files appeared in the watched folder.

5. **Does mere playback write files?**
   Set up test 1. Do not render. With "Edit" active, open the Video Sequencer in Preview mode, set Sidebar ‣ View ‣ Scene Strip Display ‣ Shading to Rendered, and play the timeline through once, then scrub back and forth a few times. Report whether anything appeared in the watched folder, and how many files.

6. **Does the sequencer's own render command skip it?**
   Set up test 1. With "Edit" active, use the Video Sequencer's View menu ‣ Sequence Render Animation (rather than the top-bar Render menu), leaving the preview shading at its default. Report whether anything appeared in the watched folder.

7. **Does it behave the same with no window?**
   Save the file from test 1, empty the watched folder, and from a terminal run `blender -b thefile.blend -S Edit -a`. Report which files appear in the watched folder and their exact names, and whether they match what test 1 produced.

8. **Passes.**
   Repeat test 1 but connect a pass other than Image — for example Mist or Z, enabled in Properties ‣ View Layer ‣ Passes — to a second input on the File Output node. Report whether that second file is written, and whether it contains the pass or something blank.

9. **Two strips, one scene.**
   Repeat test 1 with two Scene strips in "Edit", both referencing "Shot", placed one after the other on the timeline. Render Animation from the top-bar. Report how many files were written, their names, and whether any file was written twice.

---


## The researcher's own list of things to settle by hand — all of them now run

**Closed on 2026-08-26.** Every check proposed above was carried out, together with the tests the audit sent on and several more, in all four researched versions of Blender and on two computers. The results are written into questions 16 and 17 above as observed findings, and the whole list, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`.

**Only these are still open, and neither can be reached from the equipment this project has:**

- **Windows.** No Windows machine has run it, so every answer above covers macOS on Apple silicon and Ubuntu Linux and leaves Windows untested.
- **The four tests that need Blender's own window, on macOS.** They were run on the rented Linux machine, which has a display server, and not on the Creator's Mac. So the windowed render, the played preview, the sequencer's own render command with a window, and the node's sidebar panel are settled on Linux alone. Each of the first three has a terminal counterpart that was run on both machines.
- **A folder reached over a network**, which has been on this topic's *cannot be run* list from the start: neither machine has a network store.

*(The checks the researcher proposed are left in place above this note rather than deleted, so a reader can see what was asked as well as what came back.)*


---

2026-08-25 — Claude Opus 5 (Claude Code subagent) — File output nodes, Blender 5.1.
