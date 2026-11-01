# File output nodes — Blender 5.2 — questions 16 and 17

Sources consulted: Blender's 5.2 manual, Blender's 5.2 Python API documentation, and the Blender 5.2 release notes, all as the plain-text copies in `Blender Sources/`, plus Cycles' own 5.2 files in the same folder. All were taken on 2026-07-30, and that is my date read for everything below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 16. When a scene is brought into another scene through its video sequence editor, what do the File Output nodes belonging to the brought-in scene do?

**LABEL: not in the sources.**

Nothing in the 5.2 manual, the 5.2 API documentation, the 5.2 release notes or Cycles' files says what happens to a File Output node that sits in a scene which is being pulled into another scene as a Scene strip. The two subjects — File Output nodes, and Scene strips — are each documented, and never together. No page discusses them in the same sentence, the same section, or the same page. One passage does put File Output nodes and the Video Sequencer together, and it is unscoped about whose nodes it means — item 5 below. It never names scene strips, so it settles nothing here, but it is the one place a reader should look before accepting this answer.

### How I read the question

I read "brought into another scene through its video sequence editor" as: the outer scene's sequencer holds a **Scene strip** that points at the inner scene, and the outer scene is the one being rendered. That is the only mechanism the 5.2 manual describes for putting one scene inside another's sequencer.

One documented detail splits that reading in two, and the sheet's wording does not choose between them. A Scene strip has an **Input** setting:

> "Camera:
>
> Use the Scene’s 3D camera as input.
>
> Sequencer:
>
> Use the scene’s Sequencer timeline as input, allowing one scene to reuse
> another scene’s edit (instead of taking the render output from the scene)."

— Manual, Scene Strip, https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html — read 2026-07-30

The same choice appears in the API as `SceneStrip.scene_input`, "Input type to use for the Scene strip (default 'CAMERA')", with members `CAMERA` — "Use the Scene’s 3D camera as input" — and `SEQUENCER` — "Use the Scene’s Sequencer timeline as input" (https://docs.blender.org/api/5.2/bpy.types.SceneStrip.html — read 2026-07-30).

So the inner scene can be brought in either as a camera render or as its own edit. The sources say nothing about File Output nodes in either case, so the split does not change the answer here — but it is the obvious place where an answer would differ, and it is worth naming.

### The closest the sources come

None of the following answers the question. I list them because they are the material a reader would otherwise assume settles it, and because they are the record of what I found rather than what I looked for.

1. What the node itself is documented to do, and when:

> "The File Output node writes one or more images to disk during rendering or compositing."

— Manual, File Output Node, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30

That sentence names two occasions, "rendering or compositing", and does not say whose render, or whether a render driven from another scene's sequencer counts.

2. What the Post Processing panel says about a render whose sequence holds scene strips:

> "Renders the output of the Video Sequence editor, instead of the view from the 3D scene’s active camera.
> If the sequence contains Scene strips, these will also be rendered as part of the pipeline.
> If Compositing is also enabled, the Scene strip will be the output of the Compositor."

— Manual, Post Processing, https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html — read 2026-07-30

That last sentence is itself ambiguous in the source: it does not say whose "Compositing" checkbox is meant (the rendered scene's, or the strip's scene's), and it does not say whose Compositor produces the image. I did not resolve it, and the page offers nothing further.

The panel it describes is shown in the manual's picture `_images/render_output_properties_post-processing_panel.png`, on that same page (https://docs.blender.org/manual/en/5.2/render/output/properties/post_processing.html — read 2026-07-30), which I opened. It shows a "Post Processing" panel with two checkboxes, "Compositing" and "Sequencer", both ticked, and a "Dither" slider at 1.000. **documented**, from that picture. That the panel belongs to one scene's render settings comes from the page the picture sits on, not from the picture. The picture shows nothing about scene strips.

3. What a Scene strip carries:

> "Scene strips are a way to insert the render output of another scene into your sequence.
> Instead of rendering out a video, then inserting the video file, you can insert the scene directly."

and, from the same page's Limitations section:

> "Scene strips do not render individual Render Passes;
> only the Combined render pass will be used."

— Manual, Scene Strip, https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html — read 2026-07-30

This constrains what the strip *takes* from the inner scene. It says nothing about what the inner scene's nodes *do* on the way there.

4. The two pipeline switches, as the API describes them:

> "Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)"

— `RenderSettings.use_compositing`, API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30

> "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist (default True)"

— `RenderSettings.use_sequencer`, same page, read 2026-07-30

Both are properties of a scene's render settings, so each scene has its own pair. The documentation does not say which scene's pair governs the inner scene's compositor when that scene is reached through a strip.

5. The one passage that puts File Output nodes and the sequencer in the same paragraph:

> "When disabled, Blender does not write render output from this panel and instead
> relies on other systems. For animation renders, if the
> Video Sequencer is active and no alternative output is
> available, an error is raised. Otherwise, if compositing is enabled, active
> File Output Node nodes are used to save the result.
> If no such nodes exist, an error is reported."

— Manual, Output panel, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30

The passage sits on the page for a scene's own Output panel, and "this panel" is that scene's. But the quoted words stop there. They do not say whose Video Sequencer is meant, and they do not say whose File Output nodes — "active File Output Node nodes" is unscoped in the sentence itself. So ruling this passage out takes a step the words do not carry: that the nodes meant belong to the panel's own scene and to no other. **LABEL for that step: inferred.** I therefore record the passage as unscoped rather than as ruled out, and it is the closest anything in the sources comes to the question.

### What would follow only by inference, and why I am not asserting it

**LABEL for this sub-claim: inferred — and weakly.** If one takes the Post Processing sentence to mean that the strip's image is the *inner* scene's compositor output, then the inner scene's compositor must be evaluated while the outer render runs; and since a File Output node is documented to write "during rendering or compositing", its File Output nodes would write files. The steps a reader has to supply, none of which the sources state:

- that "the Compositor" in "the Scene strip will be the output of the Compositor" is the inner scene's, not the outer scene's;
- that evaluating that node tree includes running its File Output nodes, rather than only the chain feeding the tree's output image;
- that the phrase "during rendering or compositing" covers a compositor run reached through another scene's sequencer.

Three supplied steps is too many for anything but a guess. I am recording it as the shape the answer might take, not as a finding. The answer to question 16 remains **not in the sources**.

### Where I looked

Files searched, whole-tree, case-insensitive, in `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2`:

- `file output` — in the manual it appears in exactly seven files: `genindex`, `render/output/animation`, `render/output/properties/output`, `compositing/types/index`, `compositing/types/output/index`, `compositing/types/output/file_output`, `files/file_paths`. I opened and read every one of those mentions. In the API it appears in the two File Output pages plus index and listing pages. In the release notes it appears twice (compositor, bugfixes). In Cycles' files it does not appear at all.
- `outputfile` — API only; the two pages already read.
- `scene strip` — in the manual: `interface/window_system/workspaces`, `editors/video_sequencer/sequencer/sidebar/cache`, `editors/video_sequencer/preview/header`, `editors/dope_sheet/display/overlays`, `editors/video_sequencer/preview/sidebar`, `editors/video_sequencer/sequencer/navigating`, `render/output/properties/post_processing`, `scene_layout/scene/introduction`, `video_editing/sequencer_scene`, `video_editing/edit/montage/strips/index`, `video_editing/edit/montage/strips/introduction`, `video_editing/storyboarding/index`, `video_editing/edit/montage/editing`, `video_editing/edit/montage/strips/scene`, `video_editing/edit/montage/strips/effects/alpha_over_under`. In the API: `StripsTopLevel`, `StripsMeta`, `SpaceDopeSheetOverlay`, `ThemeCommonAnim`, `SceneStrip`, `bpy.ops.sequencer`, `WorkSpace`. None of them mentions output nodes or writing files.
- `sequenc` restricted to the whole `compositing/` folder of the manual — thirty-odd hits, all read down the line. The only ones about the sequencer at all are the Sequencer Strip Info node, the compositor-strip and compositor-modifier sections of `compositing/usage`, and a passing note in `convert_to_display`. None mentions File Output.
- `nested scene`, `sub-scene`, `subscene`, `scene within a scene`, `scene inside`, `brought in`, `referenced scene`, `other scene`, `secondary scene` — no hit connects File Output to a scene reached through a sequencer.
- `writes one or more images`, `written to disk`, `save the result`, `as part of the pipeline`, `sequencer pipeline`, `compositing pipeline`, `Sync Scene Time`, `recursi` — all hits read; nothing further.
- `VSE` — no hits in the compositing pages, `files/file_paths` or `render/output`.

Pages opened in full and ruled out: `video_editing/edit/montage/strips/scene` (Scene strip — no mention of compositing or output nodes), `video_editing/sequencer_scene` (Sequencer Scene — no mention), `video_editing/edit/montage/strips/compositor` (Compositor strip — a separate feature; its Usage section says "Compositor strips support all compositor nodes that operate on image data" and never names File Output), `video_editing/edit/montage/modifiers/compositor`, `compositing/usage`, `compositing/limitations` (hardware only), `compositing/compositor_system` (data types and operation domains; its Output Nodes section is about image size, and names viewport and final render compositing only), `compositing/introduction`, `compositing/types/input/scene/render_layers` (its Hint covers using another scene's compositing output, but by rendering that scene to multi-layer files and loading them — not through the sequencer), `render/output/introduction`, `render/output/animation`, `advanced/command_line/render`, `advanced/command_line/arguments`, `release_notes_5.2/compositor.md`, `release_notes_5.2/sequencer.md` (its "Scene Strips" section lists a view-layer selector, a workbench-world override, and an Add-menu wording change — nothing about output nodes). Cycles' files were ruled out by search: no occurrence of `file output`, `OutputFile` or `scene strip` anywhere in `cycles_5.2`.

Pictures opened: `_images/render_output_properties_post-processing_panel.png` (described above) and `_images/node-types_CompositorNodeOutputFile.webp`, which shows a File Output node with a directory field reading `/tmp/`, a file-name field reading `file_name`, and one unnamed input socket. **documented**, from that picture, which sits on the File Output Node page, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30. *(Citation check, 2026-08-26: this line carried a note that the second picture's page address was missing. The address has been supplied — the 5.2 File Output Node page embeds `_images/node-types_CompositorNodeOutputFile.webp` at its head — and the picture opened and found to show what is described.)* Neither picture shows anything about scene strips.

**One term this record does not cover.** The manual and the release notes were not swept for `output file` with the words in that order — `outputfile` as one word was searched in the API only. Anywhere the node is written the other way round would not have been caught by the sweeps above.

### Not covered

This answer does not settle: whether the inner scene's File Output nodes write at all; if they do, how often (once per outer-timeline frame, once per inner-scene frame, or once per strip instance); what happens when two strips point at the same scene; whether a muted or trimmed strip suppresses the writes; whether retiming, `strobe`, `use_reverse_frames` or content trimming change how many files appear; whether the inner scene's own Compositing and Sequencer switches, or the outer scene's, decide; whether the behaviour differs between the strip's Camera input and its Sequencer input; and whether sequencer playback or prefetch in the interface causes writes as a render does. The sources address none of these.

### Outside the interface

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say. I searched the command-line pages (`advanced/command_line/arguments`, `advanced/command_line/render`) and found nothing about File Output nodes or scene strips. `-b, --background` is documented as "Run in background (often used for UI-less rendering)" and `-S, --scene <name>` as "Set the active scene <name> for rendering" (Manual, Command Line Arguments, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30), but neither page states any difference in how compositor output nodes behave when Blender is driven from outside its window. Since the sources do not settle what happens inside the interface either, they cannot settle whether it differs outside it.

### Operating system

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say. No page I read attaches an operating-system condition to File Output nodes, to Scene strips, or to the Post Processing switches. The only operating-system-specific note anywhere in the compositing documentation is about GPU driver timeouts — "TDR on Windows", Manual, Limitations, https://docs.blender.org/manual/en/5.2/compositing/limitations.html — read 2026-07-30 — which is about the GPU compositor freezing, not about writing files.

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

**LABEL: not in the sources.**

The sources document how a File Output node's file name is built in the ordinary case: the pattern, where the frame number goes, and the template variables a name may carry. They say nothing about how that construction behaves when the node's scene is being rendered as a Scene strip inside another scene.

Two parts of the name would plausibly differ, and the sources leave them open in different ways. The `{scene_name}` and `{camera_name}` variables are defined in so many words as "the current scene" and "the current render camera", and no page I read says which scene or camera is current in the nested case. The frame number is not tied to any scene by any wording I found — the pages that place it in the name simply call it "the frame number" — so there the sources are not ambiguous but silent.

### How I read the question

I read "what is a file called" as: the full name and path of the file that lands on disk. I did not treat it as asking only for the last part of the name.

### What is documented about the name, in the ordinary case

The pattern:

> "Each connected input is saved to the specified directory and filename,
> with the frame number appended to create a sequence.
> For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

The frame-number placement:

> "Both the File Name and the Image names (when saving multiple images) can contain #### to force the
> frame number to appear in a specific place in the filename. This also has the effect that a single render will
> have the frame number embedded. The frame number will be padded with zeros to fill the number of #
> characters."

Folders coming out of socket names:

> "If an input name contains a slash character (/ or \),
> the path is interpreted as a directory hierarchy."

> "Any missing directories are created automatically during output."

Where the socket name goes:

> "Lists all input sockets of the node.
> The name of each input socket is used to construct the final file name."

The extension:

> "Add the file format extensions to the saved file name (eg: filename + .jpg)."

All five — Manual, File Output Node, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30

The node also carries a panel that displays the answer for you:

> "Displays the final file path and extension for each input/layer, based on current settings.
> This helps verify that outputs are named and organized correctly."

— same page, Output Paths panel, read 2026-07-30

The API states the two fields and their support for the blend-relative prefix and for templates:

> "directory
>
> The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)"

> "file_name
>
> The base name of the file. Other information might be included in the final file name depending on the node options (default "", never None, Supports template expressions)"

> "use_file_extension
>
> Add the file format extensions to the saved file name (eg: filename + .jpg) (default False)"

— API, `CompositorNodeOutputFile`, https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30

And 5.2's release notes record that this last switch became the node's own:

> "File Output node now has a **File Extension** checkbox that is independent of the 'File Extension'
> checkbox in the scene render settings."

— Release notes, Compositor, https://developer.blender.org/docs/release_notes/5.2/compositor/ — read 2026-07-30

### The template variables, and what the nested case leaves open about them

> "Currently path templates are only supported for the render output path in
> Scene properties and the output paths in the compositor’s File Output
> node."

> "Available only in render output paths (including in the compositor File Output node):"

> "scene_name:
>
> Name of the current scene."

> "camera_name:
>
> Name of the current render camera."

> "Available only in path properties on a node:
>
> node_name:
>
> The name of the node that the path property is on."

> "blend_name:
>
> The currently open blend file’s name (without the .blend)."

> "blend_dir:
>
> Path up to (but not including) the currently open blend file."

All — Manual, File Paths, https://docs.blender.org/manual/en/5.2/files/file_paths.html — read 2026-07-30

The same page also documents `fps` ("The frames per second of the current scene"), `resolution_x` / `resolution_y`, `blend_name_lib` and `blend_dir_lib`, format specifiers written after a colon, `{{` and `}}` escaping, and that a malformed expression turns the field red and can stop the render.

That these variables apply to File Output node paths is said outright by the first quote above. That the camera variable turns on which camera is current is a further step, and the only thing pointing at it is a 5.2 bug-fix entry. **LABEL for that step: inferred.** The step a reader has to supply: a bug-fix line names a fault, not the behaviour that was intended, so reading "when switching cameras" as a rule about which camera counts is my reading rather than the source's.

> "Compositor - frame slip bug using \{camera_name}\ Vairable in File Output Node when switching cameras."

— Release notes, Bug Fixes, https://developer.blender.org/docs/release_notes/5.2/bugfixes/ — read 2026-07-30 (spelling as in the source)

No page I read defines "current scene" or "current render camera" for a scene being rendered through another scene's Scene strip. Whether some page does is not settled by the record of looking below, which does not sweep those two phrases.

### One thing that is only inferred, and I am flagging it as such

**LABEL for this sub-claim: inferred.** `{blend_name}`, `{blend_dir}` and the `//` blend-relative prefix should give the same result whichever scene is regarded as current, wherever both scenes live in the same blend file. The step I took: the manual says a Scene strip cannot point at the sequencer's own scene —

> "Scene strips cannot be used to reference the sequence’s own scene; a secondary scene must be used instead."

— and that scenes offered in the Add menu are scenes in the current file, with asset scenes brought in bodily:

> "Existing scenes can be added from the Add ‣ Scene ‣ “Scene Name”.
> Additionally, any scene asset in the current file or in an Asset Library
> will also show up in the Add ‣ Scene. ... When adding a scene asset as a
> strip, the scene is always appended/duplicated."

— both, Manual, Scene Strip, https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html — read 2026-07-30

The step: "always appended/duplicated" is said of a scene **asset** being added, and I read it as covering every scene a strip can point at, so that "the currently open blend file" is the same file for both scenes. The manual does not state that as a rule, and it does discuss library-linked data-blocks elsewhere (`blend_name_lib`, `blend_dir_lib`), so a linked scene is a case the sources leave open and this step does not reach. Treat it as inferred, not documented, and as covering only scenes that sit in the open file.

### Where I looked

All the searches listed under question 16 apply here too; they were run once and cover both questions. In addition, aimed at naming specifically:

- Read `files/file_paths` end to end — the whole path-template feature, its variables, its syntax, its format specifiers, its escaping and its error handling.
- Read `render/output/properties/output` — its File Path section documents the frame number being "appended at the end of the file name with four padded zeros" and the `#` padding, for the scene's own output path, not the node's.
- Read `advanced/command_line/arguments` — `-o, --render-output <path>` sets "the render path and file name" and explicitly supports "path templating features such as {blend_name}". It is documented as the render output path; nothing there says it touches File Output node paths, so I ruled it out rather than treating it as a naming input for this question.
- Searched for `scene_name`, `camera_name`, `node_name`, `blend_name` across the manual and API — the definitions above are the only place they are defined.
- Read the API pages `CompositorNodeOutputFile` and `NodeCompositorFileOutputItem` in full. The item page carries `name`, `override_node_format`, `save_as_render`, `socket_type` and a format block; it says nothing about scenes, strips or sequencers.
- Read `bpy.types.SceneStrip` in full, and `RenderSettings.use_compositing` / `use_sequencer`. Nothing about file naming.
- Opened `_images/node-types_CompositorNodeOutputFile.webp`, which shows the node's two name-related fields (`/tmp/` and `file_name`) and nothing about nesting.
- Searched `release_notes_5.2` for `composit` across `pipeline_io.md`, `rendering.md`, `core.md`, `index.md` and `python_api.md` — the only hit is the index's link to the compositor page. Read `compositor.md` and `sequencer.md` in full.

**What this record does not cover.** The searches carried over from question 16 were screened for one thing: whether a page mentions output nodes or writing files. That screen cannot establish silence about which frame number counts, or which camera counts, when a scene is reached through a strip — a page could settle either without ever naming an output node. And the two phrases this answer turns on, "current scene" and "current render camera", were not swept themselves, nor were the words around which camera is in force during a render. So for the frame number and for the two variables, the silence recorded here is not established to the bar this project sets.

### Not covered

This answer does not settle: which frame number lands in the name when the node's scene is reached through a strip — the inner scene's own frame, or the position on the outer timeline; what `{scene_name}` and `{camera_name}` expand to in that case; whether the outer scene's frame offset, trimming, retiming or `strobe` shift the number; whether files from two strips pointing at the same scene collide on the same name; and whether the node's Output Paths panel, which previews the final path "based on current settings", previews the nested case correctly at all. It also does not settle the linked-library case for `{blend_name}` and `//`.

### Outside the interface

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say, for the nested case. What they do say about naming — the pattern, the `#` padding, the template variables and their definitions — is written without reference to the interface, and the command-line page documents templates working in `-o` render output paths, which are given from outside the interface. It does not tie that to running with `-b`, and I am not claiming it does. But no page states whether a File Output node's name resolves identically when Blender is driven from outside its window, and since the nested case is undocumented inside the interface as well, the sources cannot settle the comparison.

### Operating system

*(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)*

The sources do not say. No naming rule I found is qualified by operating system. The File Output page treats both slash characters alike — "If an input name contains a slash character (/ or \), the path is interpreted as a directory hierarchy" (Manual, File Output Node, same address as above, read 2026-07-30) — without naming an operating system for either.

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

## Things a person with Blender 5.2 open could settle by hand in a few minutes

Each of these is written so somebody who cannot see my reasoning can carry it out.

**A. Does a brought-in scene's File Output node write anything at all?**
1. Open Blender, new file. Rename the default scene `Inner`.
2. In `Inner`, open the Compositing workspace and tick Use Nodes. Add a File Output node. Set its Directory to a new empty folder you can find again, and its File Name to `inner_`. Connect the Render Layers node's Image output to the File Output node's input.
3. Create a second scene, `Outer` (Topbar scene menu, New).
4. In `Outer`, open the Video Editing workspace and add a Scene strip: Add ‣ Scene ‣ Inner. Leave its Input on Camera. Set `Outer`'s frame range to 1–5.
5. In `Outer`'s Output properties, confirm Post Processing has both Compositing and Sequencer ticked.
6. Render the animation from `Outer`.
7. Look in the folder. Report: are there files, how many, and what exactly are they named.

**B. Same test with the strip's Input set to Sequencer.**
Repeat A, but in step 4 set the Scene strip's Input to Sequencer, and put at least one strip (a Color strip is enough) in `Inner`'s own sequencer so it has an edit to reuse. Report whether files appear and what they are called. Compare with A.

**C. Which frame number lands in the name.**
Repeat A, but in step 4 place the Scene strip so it starts at frame 20 of `Outer`'s timeline, and set `Outer`'s frame range to 20–24 while `Inner`'s own frame range stays 1–5. Render animation from `Outer`. Report the exact file names — do they read 0001–0005, 0020–0024, or something else.

**D. What `{scene_name}` and `{camera_name}` expand to.**
Repeat A, but set the File Output node's File Name to `{scene_name}_{camera_name}_`. Give `Inner` and `Outer` clearly different names and clearly differently named cameras. Render animation from `Outer`. Report the exact file names.

**E. Which Compositing switch governs it.**
Repeat A three more times: once with `Inner`'s Post Processing ‣ Compositing unticked, once with `Outer`'s unticked, once with both unticked. Report in each case whether files appear.

**F. Does muting the strip stop the writes?**
Repeat A, then mute the Scene strip in `Outer`'s sequencer — using whichever mute control the sequencer offers for a strip — and render again into an emptied folder. Report whether files appear, and say which control was used.

**G. Two strips, one scene.**
Repeat A but place two Scene strips in `Outer`, both pointing at `Inner`, on different channels and overlapping in time. Render animation. Report how many files appear and whether any name appears twice.

**H. Does the Output Paths panel preview the nested case?**
With setup A in place, open `Inner`'s compositor sidebar ‣ Node ‣ Properties ‣ Output Paths and note the path it displays. Then compare it with the file that actually lands on disk after rendering from `Outer`. Report whether they match.

**I. Does it differ outside the interface?**
Save the file from setup A. From a terminal, run Blender with no window on that file, with `Outer` set as the scene, rendering the animation — `blender -b <file>.blend -S Outer -a`, into an emptied output folder. `-b` and `-S` are quoted from the manual above; the animation flag is not, so check it on the Command Line Arguments page before running. Report whether the files, their count and their names match what step 7 produced, and copy out any messages Blender prints about the File Output node.

---


## The researcher's own list of things to settle by hand — all of them now run

**Closed on 2026-08-26.** Every check proposed above was carried out, together with the tests the audit sent on and several more, in all four researched versions of Blender and on two computers. The results are written into questions 16 and 17 above as observed findings, and the whole list, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`.

**Only these are still open, and neither can be reached from the equipment this project has:**

- **Windows.** No Windows machine has run it, so every answer above covers macOS on Apple silicon and Ubuntu Linux and leaves Windows untested.
- **The four tests that need Blender's own window, on macOS.** They were run on the rented Linux machine, which has a display server, and not on the Creator's Mac. So the windowed render, the played preview, the sequencer's own render command with a window, and the node's sidebar panel are settled on Linux alone. Each of the first three has a terminal counterpart that was run on both machines.
- **A folder reached over a network**, which has been on this topic's *cannot be run* list from the start: neither machine has a network store.

*(The checks the researcher proposed are left in place above this note rather than deleted, so a reader can see what was asked as well as what came back.)*


---

2026-08-25 — Claude Opus 5 (Claude Code subagent) — File output nodes, Blender 5.2.

I ran no helper agents. Every search and every page above was opened by me, and the declaration at the top covers all of it.
