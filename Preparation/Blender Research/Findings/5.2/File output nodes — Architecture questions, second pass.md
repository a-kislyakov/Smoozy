# File output nodes — Architecture questions, second pass (Blender 5.2)

Questions answered: 16 and 17 only.

**Sources consulted, and when.** Blender 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` folder), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files (`Blender Sources/cycles_5.2`). All four were taken on 2026-07-30, and that is the date read for everything quoted below. I did the searching and reading on 2026-08-26.

`I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.`

(One clarification, so the line above is exact: I also read `Blender Sources/About this folder.md`, which is the note that sits at the top of the `Blender Sources/` folder and is inside it. It links onward to a document in `Preparation/`; I did not follow that link.)

---

## 16. When a scene is brought into another scene through its video sequence editor, what do the File Output nodes belonging to the brought-in scene do?

**How I read the question.** "Brought into another scene through its video sequence editor" I read as: scene B contains a Scene strip in its video sequencer, and that strip points at scene A. The File Output nodes in question are the ones in scene A's compositing node tree. The question is what those nodes do when scene B is rendered. The other reading — a Scene strip whose Input is set to Sequencer, so that scene B reuses scene A's *edit* rather than scene A's render — is a different case; the manual names both, and I have flagged below the one place where the distinction matters.

**LABEL: not in the sources.**

Nothing in the manual, the API documentation, the release notes or Cycles' files says whether a File Output node belonging to a scene pulled in as a Scene strip writes a file, is skipped, or writes something different. I searched for it directly and did not find it. What the sources *do* say is a ring of facts around the question, and I set those out because they are what somebody would otherwise mistake for an answer.

### What the sources do say, quoted

**(a) A Scene strip causes the other scene to be rendered.**

> "Scene strips are a way to insert the render output of another scene into your sequence. Instead of rendering out a video, then inserting the video file, you can insert the scene directly."

Source: manual. https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html — read 2026-07-30.

> "When used as strips, scenes are rendered dynamically during playback or rendering, and they share the same data-blocks unless they were created as full copies."

Source: manual. https://docs.blender.org/manual/en/5.2/scene_layout/scene/introduction.html — read 2026-07-30.

**(b) When a scene renders through its sequencer, Scene strips in it are rendered as part of that render, and a compositor can stand between the strip and the result.**

> "Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor."

Source: manual, Output properties → Post Processing panel, the "Pipeline – Sequencer" entry. https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html — read 2026-07-30.

That last sentence is ambiguous in a way that matters here, and I am not going to resolve it by guessing. "If Compositing is also enabled" does not say *whose* Compositing switch — the scene being rendered, or the scene the strip points at. Both scenes have one, because it is a per-scene setting (see (d)). Under the reading I think is meant — the strip's own scene — the sentence says the brought-in scene's compositor runs and its result is what the strip contributes. Under the other reading it is the containing scene's compositor. The page does not settle it.

The neighbouring entry, for completeness of what "Compositing" means:

> "Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node."

Same page, same address, read 2026-07-30.

**(c) A File Output node writes during rendering or compositing.**

> "The File Output node writes one or more images to disk during rendering or compositing."

Source: manual. https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30.

This is as close as the documentation comes, and it is not close enough. It says a File Output node writes when rendering or compositing happens. It does not say that a Scene strip's render of another scene counts as "rendering" for this purpose, and it does not say it does not.

**(d) The Compositing and Sequencer switches are per-scene render settings.**

> "use_compositing — Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)"

> "use_sequencer — Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist (default True)"

Source: Python API documentation, `RenderSettings`. https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30.

**Observed, from Cycles' own files.** Cycles draws both switches from `context.scene.render`, i.e. per scene, in the Output tab's Post Processing panel:

```
class CYCLES_PT_post_processing(CyclesButtonsPanel, Panel):
    bl_label = "Post Processing"
    bl_options = {'DEFAULT_CLOSED'}
    bl_context = "output"

    def draw(self, context):
        ...
        rd = context.scene.render

        col = layout.column(align=True, heading="Pipeline")
        col.prop(rd, "use_compositing")
        col.prop(rd, "use_sequencer")
```

File: `Blender Sources/cycles_5.2/ui.py`, lines 1169–1183 (class begins at line 1169; `use_compositing` at line 1182, `use_sequencer` at line 1183). Read 2026-07-30. This is observed: it is what the file says. That the two scenes therefore each have their own independent switch is one short step from the quoted `context.scene.render` and the API text in (d), and I label that step **inferred**.

**Documented, from a picture.** The Post Processing panel screenshot shows exactly two checkboxes under the heading "Pipeline", labelled "Compositing" and "Sequencer", both ticked, above a "Dither" slider reading 1.000. Image: `_images/render_output_properties_post-processing_panel.png`, on https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html — read 2026-07-30.

**(e) Rendering from inside the Video Sequencer's own menu does not render Scene strips by default.**

> "Sequence Render Image and Sequence Render Animation don't render the final video by default – specifically, they don't render Scene Strips, instead using the preview's shading mode (which is initially Solid)."

> "To output a video where the Scene Strips are rendered, use the Render menu in the top-bar, or change Sidebar ‣ View ‣ Scene Strip Display ‣ Shading to Rendered."

Source: manual. https://docs.blender.org/manual/en/5.2/editors/video_sequencer/sequencer/navigating.html — read 2026-07-30. The same two sentences appear on https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/header.html — read 2026-07-30.

This bears on the question because it names a route by which the brought-in scene is not rendered at all. What follows from that about its File Output nodes is a step the manual does not take, so: if the strip's scene is not rendered, there is no render for a File Output node in it to write during. That step is **inferred**, and I have written it out rather than folded it into the answer.

**(f) A limitation the manual does state about Scene strips.**

> "Scene strips do not render individual Render Passes; only the Combined render pass will be used."

Source: manual. https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html — read 2026-07-30. This is about what the strip *takes* from the scene, not about what the scene's File Output nodes write. I record it because it is the only stated restriction on the Scene strip page and somebody checking my work will find it and wonder whether I saw it.

**(g) A release note that mentions scene compositing reaching a Scene strip.** In the 5.2 fixed-bugs list:

> "Scene Strips doesn't preserves glare from scene compositing (Film->Transparency on) on alpha over"

Source: release notes, Bug Fixes. https://developer.blender.org/docs/release_notes/5.2/bugfixes/ — read 2026-07-30. It is the title of a fixed report, and it is about the *image* a Scene strip carries out of the brought-in scene's compositing, not about files written. I include it because it is the only line in any of the four sources that puts "Scene strip" and "scene compositing" in one sentence.

### Not covered

My answer does not settle: whether a File Output node in the brought-in scene writes, does not write, or writes once per containing-scene frame; whether the answer differs between the strip's Input being Camera and being Sequencer; whether it differs between rendering a still and rendering an animation; whether the brought-in scene's own Compositing switch, or the containing scene's, governs it; whether a File Output node in the *containing* scene behaves normally in this arrangement (I did not look for that, as it is not what was asked); and what happens when the same brought-in scene appears in several strips at once. It also does not settle the ambiguity in the quoted Post Processing sentence in (b).

### Outside the interface

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say. I found no statement anywhere in the manual, the API documentation, the release notes or Cycles' files that this behaviour differs between Blender driven by a person and Blender started and driven from outside. The command-line arguments page documents `-b, --background`, `-a, --render-anim`, `-f, --render-frame`, and `-S, --scene <name>` ("Set the active scene <name> for rendering"), and `bpy.ops.render.render` documents parameters `animation`, `write_still`, `scene` ("Scene, Scene to render, current scene if not specified") and `use_sequencer_scene` ("Use Sequencer Scene, Render the sequencer scene instead of the active scene") — but none of these pages mentions File Output nodes, Scene strips, or the two together. Sources: https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html and https://docs.blender.org/api/5.2/bpy.ops.render.html — both read 2026-07-30. One route named in (e), Sequence Render Image / Sequence Render Animation, exists only inside the interface, so that part of the record has no equivalent outside it.

### Operating system

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say. Nothing on any page I opened ties any of this to a particular operating system. The only operating-system-specific statement I met anywhere near this subject is unrelated to it: a Windows note about the GPU compositor and driver timeouts, on https://docs.blender.org/manual/en/5.2/compositing/limitations.html — read 2026-07-30.

### Where I looked (question 16)

Searched, case-insensitively, across every `.txt` file in `manual_5.2` and `api_5.2`, every `.md` file in `release_notes_5.2`, and every file in `cycles_5.2`, for these terms: `file output`, `OutputFile`, `file_output`, `scene strip`, `Scene Strip`, `Post Processing`, `Post-Processing`, `use_compositing`, `use_sequencer`, `use_sequencer_override_scene_strip`, `indirectly rendered`, `indirect render`, `referenced scene`, `another scene`, `other scene`, `render output of another scene`, `strip's scene`, `nested`, `sub-scene`, `composit` (restricted to `manual_5.2/video_editing` and `manual_5.2/editors/video_sequencer`), `sequencer` and `strip` (in `cycles_5.2`), `will not write`, `does not write`, `not written`, `no files are written`, `writes nothing`, `skipped`, `ignored`.

Pages opened in full or in the relevant part:
- https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html
- https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html
- https://docs.blender.org/manual/en/5.2/video_editing/sequencer_scene.html
- https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html
- https://docs.blender.org/manual/en/5.2/render/output/properties/output.html
- https://docs.blender.org/manual/en/5.2/render/output/animation.html
- https://docs.blender.org/manual/en/5.2/compositing/introduction.html
- https://docs.blender.org/manual/en/5.2/compositing/usage.html
- https://docs.blender.org/manual/en/5.2/compositing/limitations.html
- https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html (the Output and Considerations sections)
- https://docs.blender.org/manual/en/5.2/compositing/types/input/scene/render_layers.html
- https://docs.blender.org/manual/en/5.2/scene_layout/scene/introduction.html
- https://docs.blender.org/manual/en/5.2/editors/video_sequencer/sequencer/navigating.html
- https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/header.html
- https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/sidebar.html (Scene Strip Display and Metadata sections)
- https://docs.blender.org/manual/en/5.2/editors/video_sequencer/sequencer/sidebar/cache.html
- https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/editing.html (the Scene-strip parts)
- https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/strip_properties.html (searched, matching excerpts read)
- https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
- https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html
- https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html
- https://docs.blender.org/api/5.2/bpy.types.NodeCompositorFileOutputItem.html
- https://docs.blender.org/api/5.2/bpy.types.SceneStrip.html
- https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html
- https://docs.blender.org/api/5.2/bpy.ops.render.html
- https://developer.blender.org/docs/release_notes/5.2/compositor/
- https://developer.blender.org/docs/release_notes/5.2/sequencer/
- https://developer.blender.org/docs/release_notes/5.2/pipeline_io/
- https://developer.blender.org/docs/release_notes/5.2/bugfixes/ (searched, not read end to end)

Pictures opened: `_images/render_output_properties_post-processing_panel.png` and `_images/node-types_CompositorNodeOutputFile.webp`. They are the only two images in the 5.2 manual whose file names match `outputfile`, `file_output`, `file-output` or `post-processing`.

Ruled out and why:
- `bpy.types.SceneStrip` — the whole page. It lists what a Scene strip has (`scene`, `scene_camera`, `scene_input`, `view_layer`, and the usual image controls). Nothing on it mentions the compositor, File Output, or files written.
- `bpy.types.CompositorNodeOutputFile` and `bpy.types.NodeCompositorFileOutputItem` — the whole of both pages, including inherited members. Nothing mentions scenes, strips or the sequencer.
- The Render Layers node page. Its Hint is about pulling another scene's *compositing* output into the current scene's compositor, and says to do it by rendering to multi-layer images first. That is a different mechanism from a Scene strip, so it does not answer the question.
- The Compositor Strip and Compositor Modifier material (https://docs.blender.org/manual/en/5.2/compositing/usage.html and the Sequencer release notes). These are compositor node groups used *as* strip effects, not a scene brought in through the sequencer, so they are the wrong mechanism.
- Cycles' files. The only occurrence of "sequencer" in all 420 files is the one panel line quoted above; "file output" and "OutputFile" do not occur at all. Cycles has nothing further to say here.
- The command-line rendering page, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — read in full, 2026-07-30. It walks through `-b`, `-f`, `-a`, `-o`, `-F`, `-E`, `-s`, `-e`, `-t` with worked examples. It never mentions the compositor, File Output nodes, Scene strips or the sequencer.
- The `requirements/` folder in `Blender Sources/` — it is a page about system requirements, carries no version in its address, and could not bear on this.

---


### What the hand tests settled — observed, 2026-08-26

**LABEL: observed.** Nothing in this section comes from reading. It was got by running Blender, by Claude Opus 5 (Claude Code subagent), on 2026-08-26, in Blender 5.2.1 LTS. The full list of tests, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`; what follows stands on its own and does not need it.

**The two computers, named in full.**

- **The Creator's own MacBook Pro** — Apple M1 Pro chip, macOS 26.5.2, Apple's own graphics, no second card. Blender 5.2.1 LTS at `/Applications/Blender.app/Contents/MacOS/Blender`.
- **A rented Linux machine, in Norway** — a Docker container holding half its host, Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 3060 with 12 GB and driver 595.84, about fifteen cores of an Intel Xeon E5-2697A v4, driven over SSH, with a display server of its own on `DISPLAY=:1` at 1920×1080. Blender 5.2.1 LTS at `/opt/blender/5.2.1/blender`.

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

**The control that every other result rests on.** Rendering `Inner` on its own — `/Applications/Blender.app/Contents/MacOS/Blender -b --factory-startup <file>.blend -S Inner -a` — put `inner_Img0001.png`, `inner_Img0002.png`, `inner_Img0003.png`, `inner_Img0004.png` and `inner_Img0005.png` into `watch`, on both machines. So an empty folder in any test below can only mean that test's own change stopped the writing.

**The answer to the question.** **The brought-in scene's File Output node writes.** Rendering the whole of `Outer`'s animation from a terminal — `/Applications/Blender.app/Contents/MacOS/Blender -b --factory-startup <file>.blend -S Outer -a`, and `/opt/blender/5.2.1/blender -b --factory-startup <file>.blend -S Outer -a` on the rented machine — left one file in the `watch` folder, `inner_Img.png`, written over five times, with no frame number in the name, in Blender 5.2.1 LTS, **on both machines, with the same names**. In the same run the containing scene's own render was written as `outer_0021.png` through `outer_0025.png`.

Blender's own words on the Mac, the first two of the five frames, word for word — the last three read the same with the timestamps and the outer frame number moving on:

```
00:00.480  render           | Fra: 1 | Compositing
00:00.557  render           | Saved: '/private/tmp/fw/mac/t02/5.2/watch/inner_Img.png'
00:00.568  render           | Saved: '/private/tmp/fw/mac/t02/5.2/watch_outer/outer_0021.png'
00:00.574  render           | Fra: 2 | Compositing
00:00.581  render           | Saved: '/private/tmp/fw/mac/t02/5.2/watch/inner_Img.png'
00:00.590  render           | Saved: '/private/tmp/fw/mac/t02/5.2/watch_outer/outer_0022.png'
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

**The sequencer's own render command.** From a terminal with no window at all, `bpy.ops.render.opengl(animation=True, sequencer=True)` and its single-frame form both came back `{'CANCELLED'}` and wrote nothing, in Blender 5.2.1 LTS, on both machines — so those commands cannot be reached from outside Blender's window. With a real window on the rented Linux machine, and a Video Sequencer preview in front of it, the same command still came back `{'CANCELLED'}` and wrote nothing — at Solid shading and at Rendered shading alike. That windowed run is on Linux only.

**Outside the interface — now answered by test.** The answer is the same whether Blender is driven from a terminal with no window or from a real window. The same file rendered with a real window open on the rented Linux machine — `DISPLAY=:1 XDG_RUNTIME_DIR=/run/user/0 /opt/blender/5.2.1/blender --factory-startup <file>.blend`, with the render started from inside it — left exactly the same files with exactly the same names as the terminal run. The one place the two routes differ is the sequencer's own render command, above, which does not exist outside the window at all.

**Operating system — now answered by test, for two systems.** Every test in the table above was run on both computers, and **the Mac and the rented Linux machine agreed on every one**: the same file names, the same counts, the same printed lines, the same refusals. The only differences anywhere in the two logs are line numbers, timestamps and the folders the work sat in. **Windows is untested** — no Windows machine has run it — so the answer covers macOS on Apple silicon and Ubuntu Linux and says nothing about Windows. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**What the hand tests still leave open for this question.** Windows, in every version. And the four tests that need Blender's own window — the windowed render, the played preview, the sequencer's own render command with a window, and the node's sidebar panel — were run on the rented Linux machine only, never on macOS, because the Creator was given no test to run.


---

## 17. What is a file written by a File Output node called, when that node belongs to a scene brought into another scene through its video sequence editor?

**How I read the question.** Same arrangement as question 16: the File Output node sits in scene A's compositing node tree, and scene A is pulled into scene B by a Scene strip in scene B's sequencer. The question asks what the written file's name is in that situation.

**LABEL: not in the sources**, for the part of the question that is specific to the Scene strip. The general naming rule is **documented**, and I quote it below, but nothing states whether it changes — or what its scene-dependent parts resolve to — when the node's scene is being rendered as somebody else's Scene strip. Note also that question 16 could not establish that such a file is written at all; if it is not, this question has no subject.

### The general naming rule, documented

> "Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence. For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

> "Both the File Name and the Image names (when saving multiple images) can contain #### to force the frame number to appear in a specific place in the filename. This also has the effect that a single render will have the frame number embedded. The frame number will be padded with zeros to fill the number of # characters."

> "File Extensions — Add the file format extensions to the saved file name (eg: filename + .jpg)."

Source: manual. https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30.

And the release notes record a 5.2 change to that last switch:

> "File Output node now has a **File Extension** checkbox that is independent of the 'File Extension' checkbox in the scene render settings."

Source: release notes, Compositor, under "Added". https://developer.blender.org/docs/release_notes/5.2/compositor/ — read 2026-07-30.

The API documentation gives the same two name-carrying fields:

> "directory — The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)"

> "file_name — The base name of the file. Other information might be included in the final file name depending on the node options (default "", never None, Supports template expressions)"

Source: Python API documentation. https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30.

**Documented, from a picture.** The File Output node screenshot shows the node with two text fields, the upper reading `/tmp/` with a folder-browse button beside it and the lower reading `file_name`, and a single unconnected input socket on the left. Image: `_images/node-types_CompositorNodeOutputFile.webp`, on https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30. That is what is visible; I read nothing further into it.

### The part that would decide the Scene-strip case, and what the sources say about it

Two things in a File Output name depend on which scene and which frame Blender thinks it is rendering. Both are documented as features; neither is documented for this arrangement.

**Scene-dependent text in the path.** The File Output node's directory and file name accept path templates, and one of the variables is the scene's name:

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."

> "Available only in render output paths (including in the compositor File Output node):"

> "scene_name: Name of the current scene."

> "camera_name: Name of the current render camera."

> "Available only in path properties on a node:"

> "node_name: The name of the node that the path property is on."

Source: manual. https://docs.blender.org/manual/en/5.2/files/file_paths.html — read 2026-07-30.

"The current scene" is not defined anywhere on that page, and the page never mentions Scene strips or the sequencer. So when scene B is rendering and scene A is being rendered inside it as a strip, whether `{scene_name}` in scene A's File Output node yields scene A's name or scene B's is **not in the sources**. The same gap applies to `{camera_name}`: the Scene strip has its own camera override —

> "scene_camera — Override the scene's active camera"

Source: Python API documentation. https://docs.blender.org/api/5.2/bpy.types.SceneStrip.html — read 2026-07-30 — and the manual's Scene strip page says of the same control, "This can be used to override the scene's camera with any other object." (https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html, read 2026-07-30). Nothing says whether that override is what `{camera_name}` would report.

The other template variables named on the file-paths page are `blend_name`, `blend_dir`, `blend_name_lib`, `blend_dir_lib`, `fps`, `resolution_x` and `resolution_y`. Of these, `fps` and the two resolutions are also scene properties and so would face the same question; the page does not raise it.

**The frame number.** The name carries "the frame number", per the quote above, and the containing edit and the brought-in scene need not be at the same frame — the Scene strip has trimming and timing controls, and the manual documents an operator for keeping strip and scene ranges in step:

> "For Scene strips only – Updates the strip's Time properties to match the referenced scene's frame range. This operator should be used when the referenced scene's length is extended or shortened."

Source: manual. https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/editing.html — read 2026-07-30.

Which frame number lands in the file name — scene A's own frame, or the frame of scene B's timeline at that moment — is **not in the sources**. No page I opened states it.

### Not covered

My answer does not settle: what `{scene_name}`, `{camera_name}`, `{fps}`, `{resolution_x}` or `{resolution_y}` resolve to in a File Output node belonging to a scene rendered as a Scene strip; which frame number appears in the name; whether anything is inserted into or appended to the name that would not be there in an ordinary render of that scene; whether the same scene used in two strips at once produces two different names or collides on one; and whether a relative `//` directory resolves against the same blend-file in this arrangement. It also does not settle question 16's prior question of whether any file is written at all.

### Outside the interface

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say, for the Scene-strip-specific part. For the general rule, the command-line page documents that `-o, --render-output <path>` accepts path templates — "You can use path templating features such as {blend_name} in the path" — and that "The # characters are replaced by the frame number, and used to define zero padding", with the worked example "//render_ becomes //render_####, writing frames as //render_0001.png". Source: https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30. That option sets the *scene's* render output path, not a File Output node's; the page does not mention File Output nodes at all. I found no statement anywhere that File Output naming differs between Blender driven by a person and Blender driven from outside.

### Operating system

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say. Nothing on the File Output node page, the file-paths page or the Scene strip page ties naming to a particular operating system. The file-paths page does note that a slash in an input's name is read as a folder separator on the File Output node — "If an input name contains a slash character (/ or \\), the path is interpreted as a directory hierarchy" (manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html, read 2026-07-30) — which names both slash characters without saying that either is tied to a system. That is the closest the sources come, and it is not a statement about operating systems.

### Where I looked (question 17)

In addition to everything listed under question 16, which was one sweep covering both questions, I searched all four sources case-insensitively for: `scene_name`, `camera_name`, `blend_name`, `node_name`, `scene name`, `path template`, `template expression`, `frame number`, `filepath`, and `File Extension`.

Additional pages opened for this question:
- https://docs.blender.org/manual/en/5.2/files/file_paths.html (in full)
- https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html (the `filepath` entry: "Directory/name to save animations, # characters define the position and padding of frame numbers")

Ruled out and why:
- The Bug Fixes release-notes page contains a line naming a template variable in this node: "Compositor - frame slip bug using \{camera_name}\ Vairable in File Output Node when switching cameras" (https://developer.blender.org/docs/release_notes/5.2/bugfixes/, read 2026-07-30). It is about switching cameras within a render, not about Scene strips, so it does not answer the question. Two other File-Output lines on the same page — one about multi-layer EXR with anamorphic output, one about the Output Paths panel showing wrong paths for multi-views — are likewise about other situations.
- `bpy.types.NodeCompositorFileOutputItem` — the whole page. Its `name` field carries no description at all beyond its default, and nothing on the page mentions scenes or the sequencer.
- The Output Paths panel, which the manual says "Displays the final file path and extension for each input/layer, based on current settings" (manual, File Output Node page, read 2026-07-30). It is a display of the result, not a statement of the rule, and the manual does not show what it displays in this arrangement. No image of that panel exists in the 5.2 manual's `_images` folder.
- Cycles' files. `file output`, `file_output` and `OutputFile` do not occur anywhere in them. Nor do `node_name`, `path template`, `template expression`, `frame number`, `file extension`, `scene_name`, `camera_name` or `blend_name`. `filepath` occurs in four files — `maketx.py`, `osl.py`, `ui.py`, `operators.py` — all of it about baking textures and compiling OSL scripts, nothing to do with output naming.

---


### What the hand tests settled — observed, 2026-08-26

**LABEL: observed.** Nothing in this section comes from reading. It was got by running Blender, by Claude Opus 5 (Claude Code subagent), on 2026-08-26, in Blender 5.2.1 LTS, on the two computers described under question 16 above — the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2) and a rented Linux machine in Norway (Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 3060, driver 595.84, driven over SSH). The same build on both, so a difference between them would be the operating system. **The two are never merged**; where both ran a test, both results are given. The set-up is the one described under question 16: a scene `Inner` at 24 frames per second and 320 × 180 with frames 1 to 5 and a camera `InnerCam`, brought into a scene `Outer` at 30 frames per second and 640 × 360 with frames 21 to 25 and a camera `OuterCam`, by a Scene strip placed at outer frame 21 — so no part of a name can be ambiguous about which scene it came from. The full list of tests, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`.

**What the file is called.** Rendering the whole of `Outer`'s animation from a terminal left one file in the node's folder, `inner_Img.png`, written over five times, with no frame number in the name, in Blender 5.2.1 LTS, on both machines.

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

**What the node's Output Paths panel shows.** Photographed on the rented Linux machine, with the compositor's sidebar open on its Node tab and the File Output node selected, the panel holds one row per output, reading `/root/fonwork/win...ch/inner_Img.png` — the middle of the path elided by the panel's width. Blender 5.2 draws a **File Extensions** tick box inside the panel, above that row. It shows the same row whether the brought-in scene or the containing scene is the active scene, the editor pinned to the brought-in scene's tree. Set that name beside the two renders of the same file: through the strip the node wrote `inner_Img.png`, and rendered on its own it wrote `inner_Img0001.png` through `inner_Img0005.png`, so what the panel shows is the name without a frame number. The pictures are at `Audits/Blender Research/File output nodes/Screenshots — Architecture questions/`. This was read on the rented Linux machine only.

**Outside the interface — now answered by test.** The names are the same whether Blender is driven from a terminal with no window or from a real window. The same file rendered with a real window open on the rented Linux machine gave exactly the same names as the terminal run, in Blender 5.2.1 LTS.

**Operating system — now answered by test, for two systems.** Every naming test was run on both computers and **every name came back character for character the same**. **Windows is untested**, no Windows machine having run it, so the answer covers macOS on Apple silicon and Ubuntu Linux only.

**What the hand tests still leave open for this question.** Windows, in every version; and the node's sidebar panel, which was read on the rented Linux machine only and never on macOS.


---

## Things a person with Blender 5.2 open could settle by hand

These are the gaps above, each written as something to do rather than something to reason about. Each assumes Blender 5.2 and a fresh file saved to disk first, so that `//` paths and `{blend_name}` have something to resolve against.

1. **Does a brought-in scene's File Output node write at all?**
   Make two scenes, `Shot` and `Edit`. In `Shot`: add a compositing node tree (Compositor editor → New), and make sure Output properties → Post Processing → Compositing is ticked. Add a File Output node, set its Directory to `//out_shot/`, connect the Render Layers node's Image output to it. In `Edit`: open a Video Sequencer, Add ‣ Scene ‣ Shot, leave the strip's Input as Camera. In `Edit`'s Output properties → Post Processing, tick Sequencer. Set `Edit`'s frame range to 1–5 and its output path to `//out_edit/`. With `Edit` the active scene, render the animation (Ctrl-F12). Then look in `//out_shot/`. Report: is the folder there, how many files, and what each is called.

2. **Same test, but with `Shot`'s Compositing switch off.** Untick Output properties → Post Processing → Compositing in `Shot`, repeat step 1, and report whether `//out_shot/` gets anything. This settles whether the brought-in scene's own switch governs it.

3. **Same test, but with `Edit`'s Compositing switch off** (and `Shot`'s on). Repeat step 1 and report. Steps 2 and 3 together settle which scene's switch the Post Processing page's sentence "If Compositing is also enabled" refers to.

4. **Which scene does `{scene_name}` mean?** In `Shot`'s File Output node, set File Name to `{scene_name}_`. Repeat step 1. Report the exact file names produced — whether they begin `Shot_` or `Edit_`.

5. **Which frame number appears?** Place the Scene strip in `Edit` so that it does *not* start at frame 1 — e.g. drag it to start at frame 21, with `Shot`'s own frame range being 1–5. Set `Edit`'s render range to cover the strip (21–25). Render `Edit`'s animation. Report the frame numbers in the names of the files in `//out_shot/`: are they 0001–0005, or 0021–0025?

6. **Does the camera override reach the name?** In `Shot`'s File Output node set File Name to `{camera_name}_`. Add a second camera to `Shot`, and in `Edit`'s Scene strip set the Camera override to that second camera. Render `Edit`'s animation and report which camera's name appears in the file names.

7. **Does the Input mode change anything?** Repeat step 1 with the Scene strip's Input set to Sequencer instead of Camera (put a strip of some kind in `Shot`'s own sequencer first, so there is something to reuse). Report whether `//out_shot/` gets anything, and what.

8. **Does the Sequencer's own render menu differ?** Repeat step 1, but render using the Video Sequencer's own Sequence Render Animation rather than the top-bar Render menu, with Sidebar ‣ View ‣ Scene Strip Display ‣ Shading left at its initial Solid. Report whether `//out_shot/` gets anything. Then set that Shading to Rendered and repeat, and report again.

9. **The same scene in two strips at once.** In `Edit`, add two Scene strips both pointing at `Shot`, overlapping in time on different channels. Render `Edit`'s animation. Report how many files land in `//out_shot/`, their names, and whether any appear to have been written twice.

10. **From outside the interface.** Run the same file from a terminal with `blender -b <file>.blend -S Edit -a` and report whether `//out_shot/` gets the same files, with the same names, as the interactive run in step 1. Note which operating system it was run on, and if possible repeat on a second one.

11. **Does a still render behave like an animation render?** Repeat step 1 but render a single frame (F12) instead of the animation, and report whether `//out_shot/` gets anything.

---


## The researcher's own list of things to settle by hand — all of them now run

**Closed on 2026-08-26.** Every check proposed above was carried out, together with the tests the audit sent on and several more, in all four researched versions of Blender and on two computers. The results are written into questions 16 and 17 above as observed findings, and the whole list, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`.

**Only these are still open, and neither can be reached from the equipment this project has:**

- **Windows.** No Windows machine has run it, so every answer above covers macOS on Apple silicon and Ubuntu Linux and leaves Windows untested.
- **The four tests that need Blender's own window, on macOS.** They were run on the rented Linux machine, which has a display server, and not on the Creator's Mac. So the windowed render, the played preview, the sequencer's own render command with a window, and the node's sidebar panel are settled on Linux alone. Each of the first three has a terminal counterpart that was run on both machines.
- **A folder reached over a network**, which has been on this topic's *cannot be run* list from the start: neither machine has a network store.

*(The checks the researcher proposed are left in place above this note rather than deleted, so a reader can see what was asked as well as what came back.)*


---

2026-08-26 — Claude Opus 5 (Claude Code subagent) — File output nodes, Blender 5.2.

**Declaration.** I did the whole of this work myself and started no helper agents, so there is no helper whose reading I cannot vouch for. I opened the question sheet at `Preparation/Blender Research/Questions/File output nodes.md`, the files inside `Blender Sources/` listed above, and nothing else. I did not open the existing findings document for 5.2 or anything else in its folder, did not open `CLAUDE.md`, `Preparation/Groundwork/Start Here.md`, or any other part of `Preparation/`, `Audits/`, `App/` or `README.md`, and did not use the web. Nothing above comes from memory of how Blender works; every claim is either quoted from a file named here or is a step I have labelled inferred and written out.
