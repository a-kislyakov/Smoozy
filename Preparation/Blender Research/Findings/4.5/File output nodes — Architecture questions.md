# File output nodes — Architecture questions (Blender 4.5)

**Sources consulted, and when.** Blender's 4.5 manual, Blender's 4.5 Python API documentation, and Blender's 4.5 release notes, all as the plain-text copies in `Blender Sources/`, plus one picture from the manual's own image folder, plus Cycles' own files as supplied in `Blender Sources/cycles_4.5`. Every one of those was taken on **2026-07-30**, and that is the date read for everything quoted below.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

One thing said plainly so nobody has to wonder: to be sure the folder I was told to save into existed, I asked the computer whether that one folder was there, in a way that shows only the folder itself and never what is inside it. I did not see the name of any file in it, and I opened nothing in it. Nothing else outside `Blender Sources/` and the question sheet was touched.

Questions answered here: **16 and 17 only.**

---

## 16. When a scene is brought into another scene through its video sequence editor, what do the File Output nodes belonging to the brought-in scene do?

**LABEL: not in the sources.**

No page of the manual, no page of the API documentation, and no release note for 4.5 says what happens to the File Output nodes that sit in the compositor of a scene that is pulled into a second scene as a Scene strip. The sources describe Scene strips, and they describe File Output nodes, and they never put the two together.

### What the sources do say, and how close it gets

Three separate statements bear on the question. Each is quoted exactly. None of them answers it.

**(a) A Scene strip pulls in the other scene's render output.**

> "Scene strips are a way to insert the render output of another scene into your sequence. Instead of rendering out a video, then inserting the video file, you can insert the scene directly."

Source: the manual, https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/scene.html — read 2026-07-30.

The API documentation says the same in one line:

> "Sequence strip using the rendered image of a scene"

Source: the API documentation, https://docs.blender.org/api/4.5/bpy.types.SceneStrip.html — read 2026-07-30.

**(b) When a render runs the sequencer, the Scene strips inside it are rendered too, and the compositor is named as part of that.**

> "Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor."

Source: the manual, https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html — read 2026-07-30.

That is the closest the documentation comes. It says a compositor is involved in producing what the Scene strip shows. It does not say which scene's compositor is meant, and it says nothing at all about File Output nodes.

The same page describes the neighbouring switch:

> "Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Composite Output node."

Same page and date. Note that this sentence describes the compositor's job entirely in terms of the Composite node. It does not mention files being written on the side.

The API documentation gives the two switches in the same terms:

> "Process the render result through the compositing pipeline, if compositing nodes are enabled"

> "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist"

Source: the API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.

**Inferred, one step.** That each scene carries its own pair of these switches. The quoted words do not contain the word "scene". They sit on the page describing render settings, and a reader has to supply that a scene's render settings belong to that scene alone. The step is short but it is a step, and the same step is taken again below from Cycles' own files.

**(c) A File Output node writes when a render happens.**

> "This node writes out an image, for each frame range specified, to the filename entered, as part of a frameset sequence."

> "The image(s) will be saved on rendering, writing to the current frame. An entire sequence of images will be saved, when an animation is rendered."

Source: the manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html — read 2026-07-30.

"On rendering" and "the current frame" are the whole of it. Neither phrase is tied to a particular scene on any page I searched or opened; the record at the end of this answer says which those were.

### One thing I can add by examining a supplied file

**Observed.** The Cycles files supplied build the Post Processing panel by taking the render settings of whichever scene the panel is being shown for, and drawing the two switches from those:

```
class CYCLES_PT_post_processing(CyclesButtonsPanel, Panel):
    bl_label = "Post Processing"
```
```
        rd = context.scene.render

        col = layout.column(align=True, heading="Pipeline")
        col.prop(rd, "use_compositing")
        col.prop(rd, "use_sequencer")
```

File and place: `Blender Sources/cycles_4.5/ui.py`, in the class named `CYCLES_PT_post_processing`, around lines 1100 to 1116. Read 2026-07-30.

That is what the file says, and no more: this panel reaches those two switches through a scene.

**Inferred, one step.** That every scene therefore carries its own pair. The file shows one panel reading one scene's settings; a reader has to supply that the same holds for every scene. It is the same step already taken above from the API documentation's wording, and it is written out here rather than folded into the reading.

Neither the reading nor the step settles which scene's switches are consulted when one scene is rendered by way of another's strip. That would be a further step, and a much longer one.

### The picture I looked at

`_images/render_output_properties_post-processing_panel.png`, on the Post Processing page, https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html — read 2026-07-30. **Documented**: the panel holds two tick boxes side by side, labelled Compositing and Sequencer, both ticked, and a slider labelled Dither set to 1.000. Nothing in the picture concerns Scene strips or File Output nodes.

### The step somebody would have to take, which I am not taking as an answer

**Inferred, and offered only as a signposted guess, not as a finding.** If the sentence "If Compositing is also enabled, the Scene strip will be the output of the Compositor" means the brought-in scene's own compositor, then that scene's node tree runs while the outer scene renders; and since the manual says a File Output node's images "will be saved on rendering", the File Output nodes in that tree would write files. Two things have to be supplied to get there: first, that the compositor referred to is the brought-in scene's rather than the outer scene's, which the sentence does not say; and second, that "on rendering" covers a render that the scene is dragged into rather than one aimed at it. A reader has to supply both. That is why the label at the top of this answer is *not in the sources* and not *inferred* — the chain is guesswork resting on an ambiguous sentence, and a wrong answer here would be worse than none.

### An ambiguity in the question, flagged rather than silently resolved

"Brought into another scene through its video sequence editor" reads to me as a **Scene strip** in the second scene's sequencer, which is the only mechanism the sources describe for putting one scene inside another's edit. I answered that reading. There is a second, narrower reading worth naming: the Scene strip's own Input setting can be switched from Camera to Sequencer, described as —

> "Sequencer: Use the scene's Sequencer timeline as input, allowing one scene to reuse another scene's edit (instead of taking the render output from the scene)."

Source: the manual, https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/scene.html — read 2026-07-30.

Under that setting the strip explicitly does **not** take the render output of the brought-in scene, and the sources are equally silent about File Output nodes there. I mention it because it plainly changes what would be running, and because a reader might have either reading in mind.

The same setting is described in the API documentation as an either-or with two values:

> "Camera – Use the Scene's 3D camera as input."
> "Sequencer – Use the Scene's Sequencer timeline as input."

Source: the API documentation, https://docs.blender.org/api/4.5/bpy.types.SceneStrip.html — read 2026-07-30.

### Two documented facts nearby that limit any answer

**One.** The Scene strip page carries a stated limitation:

> "Scene strips do not render individual Render Passes; only the Combined render pass will be used."

Source: the manual, https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/scene.html — read 2026-07-30. This is about what the strip uses, not about what a compositor is fed, and it does not say passes are unavailable inside the brought-in scene's own compositing.

**Two.** The compositor will not run at all without an output target it recognises:

> "The compositor searches for the active Composite node, if none was found, it searches for the active Viewer node. If none was found, the compositor doesn't run altogether."

Source: the manual, https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html — read 2026-07-30. File Output nodes are not named in that list of things searched for. This is a general statement about the compositor and not about Scene strips.

**Inferred, one step, and the step does not hold.** *(Citation check, 2026-08-26: this paragraph read "it means any answer of the form 'the brought-in scene's compositor runs, therefore its File Output nodes write' also depends on that tree holding a Composite or Viewer node." The quoted sentence does not carry that — getting there needs the reader to supply that a File Output node cannot write in a tree the compositor declined to run. The hand tests further down this same document settle it the other way: with the brought-in tree left with no output node of its own, the File Output node writes, unchanged. The step has therefore been withdrawn rather than left standing.)* What the quoted words carry is only this: without an active Composite or Viewer node the compositor does not run. Whether a File Output node still writes in that case is not something the sentence says, and the test says it does.

**Three, as a contrast.** For the *other* way of reaching across scenes — the Render Layers node pointed at a second scene — the manual says plainly whether compositing is included, which is the thing the Post Processing page does not do for Scene strips:

> "To use the compositing output from another scene rather than its "raw" render output, first render that scene into a series of multi-layered images (using e.g. the OpenEXR format), then load those images into the Compositor of the current scene using the Image Node."

Source: the manual, https://docs.blender.org/manual/en/4.5/compositing/types/input/scene/render_layers.html — read 2026-07-30. That route gives raw render output and not compositing output, and the manual says so in so many words. The two are separate mechanisms and this one does not answer the question; it is here only to show that the manual is capable of saying plainly whether a cross-scene route includes compositing, and that for Scene strips it did not. It is not evidence about which way the Scene strip route goes.

**Not covered.** This answer does not settle whether the brought-in scene's compositor executes at all during the outer scene's render; whether, if it executes, its File Output nodes write; whether the outer scene's Compositing switch, the brought-in scene's, or both are what the Post Processing sentence refers to; what happens if the brought-in scene is used by more than one strip, or appears at more than one place on the timeline; what happens when the strip is muted; whether anything is written while merely looking at the sequencer preview as opposed to running a render; or what happens when the strip's Input is set to Sequencer instead of Camera.

**Outside the interface.** *(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)* The sources do not say. Nothing in the manual or the API documentation distinguishes File Output node behaviour, Scene strip rendering, or the Compositing and Sequencer switches between Blender driven by a person and Blender started by something else. The manual documents driving renders from a command line, including choosing which scene is rendered —

> "-S, --scene <name>"
> "Set the active scene <name> for rendering."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30 — and

> "Render in the background (without UI)."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — read 2026-07-30. Neither passage says anything about compositors in brought-in scenes.

There is one documented difference that runs the other way, and it is worth having. Two sequencer commands that only exist inside the editor's own menus deliberately skip Scene strips:

> "Sequence Render Image and Sequence Render Animation don't render the final video by default – specifically, they don't render Scene Strips, instead using the preview's shading mode (which is initially Solid)."
> "To output a video where the Scene Strips are rendered, use the Render menu in the top-bar, or change Sidebar ‣ View ‣ Scene Strip Display ‣ Shading to Rendered."

Source: the manual, https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/header.html — read 2026-07-30. What the quoted words say is that those two commands do not render Scene strips by default, drawing them in the preview's shading mode instead.

**Inferred, one step.** That a Scene strip drawn in a shading mode rather than rendered is therefore not running anybody's compositor. The quote does not say that; a reader has to supply it.

The sources do not say whether the same two commands, when triggered from outside the interface rather than from the menu, behave the same way.

**Operating system.** *(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)* The sources do not say. Nothing in any passage quoted above is qualified by operating system, and I found no page that ties Scene strips, File Output nodes, or the Post Processing switches to one system rather than another.

**But no search stands behind that line.** The sweep recorded below never looked for the name of any operating system, so this is a line about what was not met while looking for something else, not a silence established by searching. Read it as unlooked-for until that search has been run.

### Record of where I looked

Searched, across the whole of `manual_4.5`, `api_4.5` and `release_notes_4.5` at once, for: `file output`, `output node`, `scene strip`, `sequencer`, `composit` (to catch compositing, compositor, composited), `post processing`, `pipeline`, `render pipeline`, `another scene`, `other scene`, `nested scene`, `sub-scene`, `subscene`, `embedded scene`, `linked scene`, `scene reference`, `renders the scene`, `rendered scene`, `indirectly rendered`, `use_compositing`, `use_sequencer`, `frame number`, `saved on rendering`, `writes out`, `write out an image`.

Pages opened in full and read:
- File Output Node — https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html
- Scene Strip — https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/scene.html
- Post Processing — https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html
- Compositing Introduction — https://docs.blender.org/manual/en/4.5/compositing/introduction.html
- Compositor System — https://docs.blender.org/manual/en/4.5/compositing/compositor_system.html
- Compositing Limitations — https://docs.blender.org/manual/en/4.5/compositing/limitations.html
- Compositing Sidebar — https://docs.blender.org/manual/en/4.5/compositing/sidebar.html
- Render Layers Node — https://docs.blender.org/manual/en/4.5/compositing/types/input/scene/render_layers.html
- Video Editing Introduction — https://docs.blender.org/manual/en/4.5/video_editing/introduction.html
- Video Sequencer Preview Header — https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/header.html
- Video Sequencer Preview Sidebar — https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/sidebar.html
- Rendering Animations — https://docs.blender.org/manual/en/4.5/render/output/animation.html
- Output panel — https://docs.blender.org/manual/en/4.5/render/output/properties/output.html
- File Paths — https://docs.blender.org/manual/en/4.5/files/file_paths.html
- Rendering From The Command Line — https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html
- Command Line Arguments (frame and scene arguments) — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- SceneStrip — https://docs.blender.org/api/4.5/bpy.types.SceneStrip.html
- RenderSettings — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- CompositorNodeOutputFile — https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html
- NodeOutputFileSlotFile — https://docs.blender.org/api/4.5/bpy.types.NodeOutputFileSlotFile.html
- Render Operators — https://docs.blender.org/api/4.5/bpy.ops.render.html
- 4.5 release notes: Compositor — https://developer.blender.org/docs/release_notes/4.5/compositor/
- 4.5 release notes: Sequencer — https://developer.blender.org/docs/release_notes/4.5/sequencer/
- 4.5 release notes: Core — https://developer.blender.org/docs/release_notes/4.5/core/

Picture opened: `_images/render_output_properties_post-processing_panel.png`, from the Post Processing page.

Cycles files searched: everything under `Blender Sources/cycles_4.5`, for `sequenc`, `scene strip`, `file output`, `output_file`, `OutputFile`. The only relevant hit was the Post Processing panel in `ui.py` quoted above; the rest were unrelated (a temporary file in the shader-compiling helper, an animation-denoising command, and internal names about random-number sequences).

Ruled out and why:
- The whole Video Editing chapter apart from the Scene Strip page — every mention of a Scene strip in it concerns adding, colouring, timing or fading strips, never what the brought-in scene writes.
- The Video Sequencer editor chapter — its Scene strip mentions are about how strips are shown in the preview, about caching, and about the preview render commands. The caching page states only "prefetching is not currently supported for Scene strips" — https://docs.blender.org/manual/en/4.5/editors/video_sequencer/sequencer/sidebar/cache.html, read 2026-07-30. *(Citation check, 2026-08-26: this line previously said the address and date read were not recorded, so the quotation could not be traced. The address and date have now been supplied and the quotation checked against the local copy of that page, where it appears word for word.)*
- The Compositing chapter apart from the File Output page — it covers node data types, the compositing space, hardware limits, and the sidebar; it never mentions Scene strips or the sequencer's use of a scene.
- The 4.5 release notes — the Sequencer note lists changes to snapping, cache settings, keymaps, slipping and the removal of the Timeline Backdrop, plus "Scene strips are now invalidated when rendered scene changes"; the Compositor note lists new and removed nodes and options; neither touches this. The bug-fix list has one File Output entry, "File output node crash in eevee when in preview mode and without connections", which is unrelated.
- The Render Layers node page — a different cross-scene mechanism, quoted above only for contrast.

---


### What the hand tests settled — observed, 2026-08-26

**LABEL: observed.** Nothing in this section comes from reading. It was got by running Blender, by Claude Opus 5 (Claude Code subagent), on 2026-08-26, in Blender 4.5.12 LTS. The full list of tests, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`; what follows stands on its own and does not need it.

**The two computers, named in full.**

- **The Creator's own MacBook Pro** — Apple M1 Pro chip, macOS 26.5.2, Apple's own graphics, no second card. Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`.
- **A rented Linux machine, in Norway** — a Docker container holding half its host, Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 3060 with 12 GB and driver 595.84, about fifteen cores of an Intel Xeon E5-2697A v4, driven over SSH, with a display server of its own on `DISPLAY=:1` at 1920×1080. Blender 4.5.12 LTS at `/opt/blender/4.5.12/blender`.

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

`Outer`'s video sequencer holds one Scene strip pointing at `Inner`, placed at **outer frame 21** and five frames long, so outer frame 21 lines up with inner frame 1 and the two frame numbers can never coincide. `Inner`'s compositor holds a Render Layers node feeding a File Output node aimed at a folder called `watch`, plus the tree's own output node. Both scenes have Compositing and Sequencer switched on unless a test says otherwise. In this version the node carries one Base Path and a per-output name, set to the `watch` folder and `inner_`.

**The control that every other result rests on.** Rendering `Inner` on its own — `"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup <file>.blend -S Inner -a` — put `inner_0001.png`, `inner_0002.png`, `inner_0003.png`, `inner_0004.png` and `inner_0005.png` into `watch`, on both machines. So an empty folder in any test below can only mean that test's own change stopped the writing.

**The answer to the question.** **The brought-in scene's File Output node writes.** Rendering the whole of `Outer`'s animation from a terminal — `"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup <file>.blend -S Outer -a`, and `/opt/blender/4.5.12/blender -b --factory-startup <file>.blend -S Outer -a` on the rented machine — left five files in the `watch` folder — `inner_0001.png`, `inner_0002.png`, `inner_0003.png`, `inner_0004.png` and `inner_0005.png`, in Blender 4.5.12 LTS, **on both machines, with the same names**. In the same run the containing scene's own render was written as `outer_0021.png` through `outer_0025.png`.

Blender's own words on the Mac, all five frames, word for word:

```
Fra:1 Mem:9.35M (Peak 12.37M) | Time:00:00.00 | Compositing
Saved: '/private/tmp/fw/mac/t02/4.5/watch/inner_0001.png'
Saved: '/private/tmp/fw/mac/t02/4.5/watch_outer/outer_0021.png'
Fra:2 Mem:14.87M (Peak 17.90M) | Time:00:00.00 | Compositing
Saved: '/private/tmp/fw/mac/t02/4.5/watch/inner_0002.png'
Saved: '/private/tmp/fw/mac/t02/4.5/watch_outer/outer_0022.png'
Fra:3 Mem:14.87M (Peak 17.90M) | Time:00:00.00 | Compositing
Saved: '/private/tmp/fw/mac/t02/4.5/watch/inner_0003.png'
Saved: '/private/tmp/fw/mac/t02/4.5/watch_outer/outer_0023.png'
Fra:4 Mem:14.87M (Peak 17.90M) | Time:00:00.00 | Compositing
Saved: '/private/tmp/fw/mac/t02/4.5/watch/inner_0004.png'
Saved: '/private/tmp/fw/mac/t02/4.5/watch_outer/outer_0024.png'
Fra:5 Mem:14.87M (Peak 17.90M) | Time:00:00.00 | Compositing
Saved: '/private/tmp/fw/mac/t02/4.5/watch/inner_0005.png'
Saved: '/private/tmp/fw/mac/t02/4.5/watch_outer/outer_0025.png'
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

**A render pass is produced.** With the Mist pass switched on in the brought-in scene and fed to a second output, **both** files were written — `inner_0001.png` … `inner_0005.png` and `mist_0001.png` … `mist_0005.png` — so the documented limit that "Scene strips do not render individual Render Passes" does not stop the brought-in scene making a pass for its own File Output node.

**One compositing tree shared by both scenes could not be set up in this version.** Blender refused it outright: `AttributeError: bpy_struct: attribute "node_tree" from "Scene" is read-only`. With the containing scene left without a tree of its own, the run behaved exactly as the plain arrangement did.

**Moving the playhead with no render.** **In Blender 4.5 it writes.** With a real window open on the rented Linux machine, one area turned into the Video Sequencer's preview and **Scene Strip Display shading set to Rendered**, walking the playhead across every frame of the strip three times over, with no render started at all, left `inner_0001.png` through `inner_0005.png` in the node's folder. The containing scene's own output folder stayed empty. The control settles that it is the shading and nothing else: the same walk with Scene Strip Display shading left at Solid wrote nothing. This was run on the rented Linux machine only, so it is not established for macOS. A terminal counterpart — the current frame set to each of the strip's frames in turn, with no window and no render — wrote nothing on either machine.

**The sequencer's own render command.** From a terminal with no window at all, `bpy.ops.render.opengl(animation=True, sequencer=True)` and its single-frame form both **stopped with an error** and wrote nothing, in Blender 4.5.12 LTS, on both machines — `RuntimeError: Error: Cannot use OpenGL render in background mode (no opengl context)` — so those commands cannot be reached from outside Blender's window. *(Citation check, 2026-08-26: this sentence said the two commands "came back `{'CANCELLED'}`". Re-run on the Creator's Mac and on the rented Linux machine, both in Blender 4.5.12 LTS, they do not come back at all — they raise the error quoted above. `{'CANCELLED'}` is what the later versions return; 4.5 raises. The wrote-nothing half and the cannot-be-reached half both stand.)* With a real window on the rented Linux machine the same command **ran**, returning `{'FINISHED'}`: at the default Solid shading it wrote the containing scene's own five frames and **nothing** from the brought-in scene, and with Scene Strip Display shading set to Rendered it wrote both. That windowed run is on Linux only.

**Outside the interface — now answered by test.** The answer is the same whether Blender is driven from a terminal with no window or from a real window. The same file rendered with a real window open on the rented Linux machine — `DISPLAY=:1 XDG_RUNTIME_DIR=/run/user/0 /opt/blender/4.5.12/blender --factory-startup <file>.blend`, with the render started from inside it — left exactly the same files with exactly the same names as the terminal run. The one place the two routes differ is the sequencer's own render command, above, which does not exist outside the window at all.

**Operating system — now answered by test, for two systems.** Every test in the table above was run on both computers, and **the Mac and the rented Linux machine agreed on every one**: the same file names, the same counts, the same printed lines, the same refusals. The only differences anywhere in the two logs are line numbers, timestamps and the folders the work sat in. **Windows is untested** — no Windows machine has run it — so the answer covers macOS on Apple silicon and Ubuntu Linux and says nothing about Windows. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**What the hand tests still leave open for this question.** Windows, in every version. And the four tests that need Blender's own window — the windowed render, the played preview, the sequencer's own render command with a window, and the node's sidebar panel — were run on the rented Linux machine only, never on macOS, because the Creator was given no test to run.


---

## 17. What is a file written by a File Output node called, when that node belongs to a scene brought into another scene through its video sequence editor?

**LABEL: not in the sources** for the part of the question that makes it different from the general case — that is, for what a name comes out as when the node's scene is being rendered by way of another scene's Scene strip. The general naming rule *is* documented, and I give it below, because it is what any answer would have to be built on.

Also worth stating plainly: since question 16 could not establish that such a file is written at all, question 17 rests on something the sources never confirm. If nothing is written, there is no name.

### The general naming rule — documented

> "This node writes out an image, for each frame range specified, to the filename entered, as part of a frameset sequence."

> "Unlike the render output filepath, this node uses a base directory and an image name, by default the output path is composed of: {base path}/{file name}{frame number}.{extension}."

> "Besides being split into two settings, in all other respects, this setting is treated the same as the render output path."

Source: the manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html — read 2026-07-30.

That last sentence hands the rest of the rule to the render output path, which is documented separately:

> "When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of # anywhere in the file name (e.g. image_##_test.png translates to image_01_test.png)."

> "This setting expands Relative Paths where a // prefix represents the directory of the current blend-file."

Source: the manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30.

The API documentation confirms both the relative prefix and that the two path settings accept template expressions:

> "string, default "", (never None, blend relative // prefix supported, Supports template expressions)"

Source: the API documentation, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30, describing the node's base path. And for the per-output part of the name:

> "Subpath used for this slot"
> "string, default "", (never None, Supports template expressions)"

Source: the API documentation, https://docs.blender.org/api/4.5/bpy.types.NodeOutputFileSlotFile.html — read 2026-07-30.

### The template expressions, which are the one part of a name that can vary by scene

New in 4.5, and stated as such by the release notes:

> "Some file paths in Blender can now contain "template expressions" like `{blend_name}` and `{resolution_x}`, which get replaced by the values they reference when the path is used."

> "Currently this is only enabled on the Scene's render output path and the output paths of the compositor's File Output node."

Source: the 4.5 release notes, https://developer.blender.org/docs/release_notes/4.5/core/ — read 2026-07-30.

The manual lists what can be substituted. The entries below are the ones that bear on question 17; this answer does not show that they are all of it, and the list may hold more:

> "blend_name: The current blend file's name (without the .blend)."

> "fps: The frames per second of the current scene."

> "resolution_x / resolution_y: The x and y resolution of the rendered image. This factors in the resolution scale as well, so if the scene resolution is 1000x600 and the scale is 50%, then resolution_x and resolution_y will be 500 and 300, respectively."

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

Source: the manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html — read 2026-07-30.

Two of those three are written in terms of "the current scene" and "the rendered image". The manual never defines which scene is current when one scene is rendered by way of another's strip, and that is exactly the situation the question asks about. So the sources give the shape of the name but not the values that would fill it in this case.

### What is not settled, and I will not guess

The frame number is the part of the name that would matter most, and the sources do not tell us which frame number appears. A Scene strip has its own start frame and its own trimming —

> "Start Frame: The first frame to start the scene strip."

Source: the manual, https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/scene.html — read 2026-07-30 — and

> "animation_offset_start: Animation start offset (trim start)"
> "animation_offset_end: Animation end offset (trim end)"

Source: the API documentation, https://docs.blender.org/api/4.5/bpy.types.SceneStrip.html — read 2026-07-30

— so the brought-in scene is generally at a different frame from the outer scene at any given moment. The File Output page says only "writing to the current frame" (same page and date as above). Whose current frame that is, in this situation, the sources do not say. I am not going to pick one.

Likewise unstated: whether the same file name would be produced twice if the same scene appeared in two strips, and whether anything in a name distinguishes a file written this way from one written by rendering that scene on its own. Nothing in the manual, the API documentation, or the release notes addresses either.

**Not covered.** This answer does not settle which frame number lands in the name; which scene counts as "the current scene" for the frames-per-second substitution, or as "the rendered image" for the resolution substitutions, when the render was started on a different scene; whether the blend-file-relative prefix or the blend-file-name substitution could resolve differently in this situation (both refer to the one open file, so there is no obvious room for them to differ, but the sources do not address it); whether a file is written at all, which question 16 could not settle; what happens to names when the same brought-in scene is used more than once; or what any of it does when the Scene strip's Input is set to Sequencer rather than Camera, which question 16 names as a second reading of the same phrase and which is equally unaddressed here.

**Outside the interface.** *(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)* The sources do not say, for the specific case asked about. On the general rule they are neutral: the naming rule, the padding rule, the relative prefix and the template substitutions are all described as properties of the paths themselves, with no mention of whether Blender is being driven by a person. The manual notes that the command line can override the render output path and its padding —

> "The # characters are replaced by the frame number, and used to define zero padding."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30 — but says nothing about the File Output node's own paths being overridable that way, and I found no argument that touches them.

One documented consequence of the template feature is worth carrying into any unattended use, because it is about a render failing rather than about how something looks:

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

Source: the manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html — read 2026-07-30. The other half of how such errors are shown is described only as an interface behaviour — "the path field will be highlighted red in the UI" and a tooltip listing the errors, same page and date — which by its nature has no equivalent when nobody is looking at the window. The sources do not say what a File Output node path with a template error does, as opposed to the render output path.

**Operating system.** *(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)* The sources do not say. The naming rule, the padding, the relative prefix and the template substitutions are all stated without any operating-system qualification, and I found no page that gives one for these. In particular nothing says how the rule interacts with any system's own limits on what a name or a folder path may contain.

**But no search stands behind that line.** As with question 16, the sweep recorded below never looked for the name of any operating system, nor for the words a manual would use for the parts of a path that differ between systems. Read it as unlooked-for until that search has been run.

### Record of where I looked

Searched across the whole of `manual_4.5`, `api_4.5` and `release_notes_4.5` for: `file output`, `frame number`, `padded zeros`, `filename`, `file name`, `base path`, `subpath`, `template`, `path template`, `blend_name`, `current scene`, `scene strip`, `animation_offset`, `frame_start`, plus the same broad sweep listed under question 16.

Pages opened in full and read: the same list as question 16, plus two that the list under question 16 does not name and that this answer rules out below — the manual's Metadata page, https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html, read 2026-07-30, and the 4.5 release note for Pipeline, Assets & I/O, https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/, read 2026-07-30. *(Citation check, 2026-08-26: this line previously said neither page had its address or date read recorded, so the two short quotations could not be traced. Both addresses and dates have now been supplied, and both quotations checked against the local copies, where they appear word for word.)* The pages that carried the material for this answer were the File Output Node page, the Output panel page, the File Paths page, the Core release note, the CompositorNodeOutputFile and NodeOutputFileSlotFile API pages, and the SceneStrip API page.

Ruled out and why:
- The Metadata page — it covers what can be stamped *into* an image, including "Includes the frame number", not what a file is called.
- The command-line pages — they document overriding the *render* output path and its padding, and choosing the scene to render; nothing there reaches the File Output node's paths or the Scene strip case.
- The 4.5 release notes for the Compositor and the Sequencer — searched for any naming or path change; the Compositor note lists node additions, changes and removals only, and the Sequencer note lists interface and performance changes only.
- The Pipeline, Assets & I/O release note — its one File Output mention is "Image Format Settings in the Render Output, File Output, and Image Saving now has a Media Type", which concerns formats, not names.
- Cycles' own files — searched for path, naming and sequencer terms; nothing there concerns File Output node naming.

---


### What the hand tests settled — observed, 2026-08-26

**LABEL: observed.** Nothing in this section comes from reading. It was got by running Blender, by Claude Opus 5 (Claude Code subagent), on 2026-08-26, in Blender 4.5.12 LTS, on the two computers described under question 16 above — the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2) and a rented Linux machine in Norway (Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 3060, driver 595.84, driven over SSH). The same build on both, so a difference between them would be the operating system. **The two are never merged**; where both ran a test, both results are given. The set-up is the one described under question 16: a scene `Inner` at 24 frames per second and 320 × 180 with frames 1 to 5 and a camera `InnerCam`, brought into a scene `Outer` at 30 frames per second and 640 × 360 with frames 21 to 25 and a camera `OuterCam`, by a Scene strip placed at outer frame 21 — so no part of a name can be ambiguous about which scene it came from. The full list of tests, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`.

**What the file is called.** Rendering the whole of `Outer`'s animation from a terminal left five files in the node's folder — `inner_0001.png`, `inner_0002.png`, `inner_0003.png`, `inner_0004.png` and `inner_0005.png`, in Blender 4.5.12 LTS, on both machines.

**Which frame number.** **The brought-in scene's own.** With the node's name part set to `inner_####_`, which forces the number into a place of its own, the files were `inner_0001_.png`, `inner_0002_.png`, `inner_0003_.png`, `inner_0004_.png`, `inner_0005_.png` — while the containing scene's own render, in the same run, was `outer_0021.png` through `outer_0025.png`. In this version the frame number is put in even when it is not asked for, so the plain arrangement already gave five separately numbered files. Written into the name by hand as `####`, it appears on a single-frame render too: rendering one frame with `-f 21` gave `inner_0001_.png` while the containing scene's own file was `outer_0021.png`.

**What the naming variables resolve to.** This version has only four naming variables, and a name using any of the others **writes nothing at all**. A name set to `{scene_name}-{camera_name}-{fps}-{resolution_x}x{resolution_y}-{node_name}-{blend_name}_` produced no file and one line per frame:

```
Error: Invalid path template in File Output node. Skipping writing file.
```

The containing scene's own render was written normally in the same run, so the render itself carried on. A name using only the variables this version does have — `{fps}-{resolution_x}x{resolution_y}-{blend_name}_` — worked, and gave the **brought-in** scene's values in every position: `24.0-320x180-t_0001.png` through `24.0-320x180-t_0005.png`, where 24 frames per second and 320 × 180 are `Inner`'s and not `Outer`'s 30 and 640 × 360.

**The strip's own camera override does not reach the name.** This version has no `{camera_name}` variable at all, so the question does not arise: a name using it wrote nothing and printed `Error: Invalid path template in File Output node. Skipping writing file.` five times.

**`{blend_name}` and `//` name the open file, not the library.** The brought-in scene was put in a second blend file, in a folder of its own, and **linked** into the file holding the strip. Blender accepted it — it reported `{'FINISHED'}` for the link and built the strip against the linked scene — and the node wrote `mainfile_0001.png` through `mainfile_0005.png` into a folder beside the **open** file, not beside the library. The library's own folder gained nothing.

**The node's own format decides the extension.** With the node set to JPEG and both scenes' own output left at PNG, the node's files came out `inner_0001.jpg` through `inner_0005.jpg` while the containing scene's own render was still `outer_0021.png` through `outer_0025.png`.

**There is no Multi-Layer EXR setting on the node in this version**, which is why the same test in the later versions has no counterpart here; the node stayed on its own default and wrote `inner_0001.png` through `inner_0005.png`.

**The command line's `-o` does not reach the node.** Rendering with `-o <folder>/override_` moved the containing scene's own render to `override_0021.png` through `override_0025.png` and left the node's file exactly where and as it was.

**Nothing in the name tells two strips apart.** Two strips of the same scene, whether overlapping or at different places on the timeline, produced ten writes over the same names, so the second strip's files land on top of the first strip's.

**There is no panel that shows the finished name in this version.** The compositor sidebar's Node tab, photographed on the rented Linux machine, holds a *Node* panel with Name, Label and a Color row, then a *Properties* panel headed **Base Path:** with the path field, a **Save as Render** tick, **File Format** PNG, **Color** BW/RGB/RGBA, **Color Depth** 8/16, **Compression** 15%, **Color Manage… Follow Scene**, an **Add Input** button, the list of outputs holding `inner_`, a **File Subpath:** field holding `inner_`, and a **Format:** section with a ticked **Use Node Format** box. Nothing in it previews the finished path. The picture is at `Audits/Blender Research/File output nodes/Screenshots — Architecture questions/sidebar-4.5.png`. This was read on the rented Linux machine only.

**Outside the interface — now answered by test.** The names are the same whether Blender is driven from a terminal with no window or from a real window. The same file rendered with a real window open on the rented Linux machine gave exactly the same names as the terminal run, in Blender 4.5.12 LTS.

**Operating system — now answered by test, for two systems.** Every naming test was run on both computers and **every name came back character for character the same**. **Windows is untested**, no Windows machine having run it, so the answer covers macOS on Apple silicon and Ubuntu Linux only.

**What the hand tests still leave open for this question.** Windows, in every version; and the node's sidebar panel, which was read on the rented Linux machine only and never on macOS.


---

## Checks a person with Blender 4.5 open could run

These are the gaps above, turned into checks. Each is written so somebody who cannot see my reasoning can carry it out. None of them is a finding; they are the experiments that would produce findings. Several need a whole animation rendered, so this is a sitting's work rather than a few minutes'.

1. **Does a brought-in scene's File Output node write anything at all?**
   Make a new file with two scenes, call them *Inner* and *Outer*. In *Inner*, turn on Use Nodes in the Compositor, leave the Render Layers node connected to the Composite node, and add a File Output node fed from Render Layers, with its base path set to an empty folder you can watch. In *Outer*, open the Video Sequencer and add a Scene strip pointing at *Inner*, with its Input left on Camera. Make *Outer* the active scene, and in *Outer*'s Output properties tick both Compositing and Sequencer under Post Processing.

   **First, a control.** Make *Inner* the active scene and render one frame of *Inner* on its own. Check that a file appeared in the watched folder, write down its exact name, then empty the folder. Without this step an empty folder later tells nobody anything: it could mean this route writes nothing, or it could mean the node was never set up to write at all.

   **Then the check itself.** Make *Outer* the active scene again and render one frame of *Outer*. Then look in the folder. Note whether a file appeared, and its exact name.

2. **Which scene's Compositing switch decides it.**
   Repeat check 1 four times, once for each combination of *Inner*'s Compositing switch on or off and *Outer*'s Compositing switch on or off (Output properties ▸ Post Processing ▸ Compositing, in each scene separately). Record for each of the four whether a file appeared. This is what settles the ambiguity in the sentence "If Compositing is also enabled, the Scene strip will be the output of the Compositor."

3. **Which frame number ends up in the name.**
   Set up as in check 1, but place the Scene strip so the two scenes are deliberately out of step: set *Outer*'s frame range to 1–10, and drag the Scene strip so it starts at *Outer* frame 5. Set *Inner*'s frame range to 1–10 as well. Render *Outer*'s whole animation. List the files that appear and write down their names in full. Compare the numbers in them against *Outer*'s frames 1–10 and against the frames of *Inner* that the strip was actually showing.

4. **Whether a Composite node is needed.**
   Repeat check 1 with *Inner*'s Composite node deleted, so its compositor holds only Render Layers and the File Output node. Note whether a file still appears. This tests whether the statement "if none was found, the compositor doesn't run altogether" stops File Output nodes too.

5. **What the Sequencer Input setting changes.**
   Repeat check 1 with the Scene strip's Input switched from Camera to Sequencer (Sidebar ▸ Strip ▸ Scene ▸ Input). Note whether a file appears.

6. **What a muted strip does.**
   Repeat check 1 with the Scene strip muted (select the strip and press H, or untick it in the sidebar). Note whether a file still appears.

7. **The same scene in two strips.**
   Repeat check 1 with two Scene strips in *Outer*, both pointing at *Inner*, sitting at different places on the timeline and on different channels. Render the whole of *Outer*. Note how many files appear, what they are called, and whether any name occurs twice — and if so, what happened to the first one.

8. **Which scene "the current scene" means for the substitutions.**
   Set up as in check 1, but give *Inner* and *Outer* deliberately different frames-per-second and different render resolutions (for instance *Inner* at 24 frames per second and 1920×1080, *Outer* at 30 and 1280×720). Put `{fps}_{resolution_x}x{resolution_y}_` at the front of the File Output node's file name in *Inner*. Render *Outer*. Read the numbers that ended up in the file name and see which scene's settings they came from.

9. **Preview-render commands versus a real render.**
   With the setup from check 1, and *Outer* active, use the Video Sequencer's own Sequence Render Image command rather than the top-bar Render menu. Note whether any file appears. Then use the top-bar Render ▸ Render Image and compare. The documented statement is that those commands do not render Scene strips by default and draw them in the preview's shading mode instead; what this check adds is whether a File Output node in the brought-in scene writes anything either way.

10. **Whether it holds with nobody watching.**
    Save the file from check 1 and run Blender from a command line in the background, rendering one frame of *Outer* by name — that is, with the background switch, the scene chosen as *Outer*, and a single frame requested. Compare which files appear, and what they are called, against what check 1 produced with the window open. Repeat on a second operating system if one is to hand.

---


## The researcher's own list of things to settle by hand — all of them now run

**Closed on 2026-08-26.** Every check proposed above was carried out, together with the tests the audit sent on and several more, in all four researched versions of Blender and on two computers. The results are written into questions 16 and 17 above as observed findings, and the whole list, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`.

**Only these are still open, and neither can be reached from the equipment this project has:**

- **Windows.** No Windows machine has run it, so every answer above covers macOS on Apple silicon and Ubuntu Linux and leaves Windows untested.
- **The four tests that need Blender's own window, on macOS.** They were run on the rented Linux machine, which has a display server, and not on the Creator's Mac. So the windowed render, the played preview, the sequencer's own render command with a window, and the node's sidebar panel are settled on Linux alone. Each of the first three has a terminal counterpart that was run on both machines.
- **A folder reached over a network**, which has been on this topic's *cannot be run* list from the start: neither machine has a network store.

*(The checks the researcher proposed are left in place above this note rather than deleted, so a reader can see what was asked as well as what came back.)*


---

2026-08-25 — Claude Opus 5 (Claude Code subagent) — File output nodes, Blender 4.5.
