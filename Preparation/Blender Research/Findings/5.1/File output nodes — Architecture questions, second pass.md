# File output nodes — Architecture questions, second pass (Blender 5.1)

Questions answered here: **16 and 17 only.** Questions 1–15 were read but are not mine to answer.

**Sources consulted, and when.** Blender 5.1 manual (`Blender Sources/manual_5.1`, including its `_images` folder), Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`). All four were taken on **2026-07-30**, and that is the date read for everything quoted below. I did the searching and reading on 2026-08-26.

`I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.`

I did all the work myself. No helper agents were used, so there is nobody else's reading to vouch for.

---

## 16. When a scene is brought into another scene through its video sequence editor, what do the File Output nodes belonging to the brought-in scene do?

**LABEL: not in the sources** — for the question as asked. No page in the manual, the API documentation, the release notes or the Cycles files says what a File Output node inside a scene does when that scene is pulled into another scene as a Scene strip. Below I first give what the sources *do* say (all documented), then one inference, clearly marked and with its step written out.

### What the sources do say

**(a) A Scene strip is the render output of the other scene.** Documented.

> "Scene strips are a way to insert the render output of another scene into your sequence. Instead of rendering out a video, then inserting the video file, you can insert the scene directly."

Source: manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/scene.html — read 2026-07-30.

The same page also limits what comes across:

> "Scene strips do not render individual Render Passes; only the Combined render pass will be used."

Same page and date.

**(b) Scene strips are rendered as part of the sequencer pipeline, and the compositor can be the thing that produces them.** Documented.

> "Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor."

Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html — read 2026-07-30.

This is the closest any source comes to the question. It says the compositor is involved in producing a Scene strip. It says nothing about File Output nodes, and nothing about files being written.

**(c) The compositor only runs at all if that switch is on.** Documented.

> "To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output's Post Processing properties."

Source: manual, https://docs.blender.org/manual/en/5.1/compositing/introduction.html — read 2026-07-30.

The two switches themselves, in the Python API:

> "use_compositing … Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)"

> "use_sequencer … Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist (default True)"

Source: API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

**(d) Those two switches belong to a scene, one set per scene.** Observed, from Cycles' own files.

`Blender Sources/cycles_5.1/ui.py`, class `CYCLES_PT_post_processing`, lines 1122–1126:

```
        rd = context.scene.render

        col = layout.column(align=True, heading="Pipeline")
        col.prop(rd, "use_compositing")
        col.prop(rd, "use_sequencer")
```

Read 2026-07-30. This is a panel drawing two properties of `context.scene.render`, so each scene carries its own pair. What that means for a render is a step beyond the file and is not claimed here.

**(e) What a File Output node is for.** Documented.

> "The File Output node writes one or more images to disk during rendering or compositing."

Source: manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — read 2026-07-30.

**(f) One case where Scene strips are explicitly *not* rendered.** Documented.

> "Sequence Render Image and Sequence Render Animation don't render the final video by default – specifically, they don't render Scene Strips, instead using the preview's shading mode (which is initially Solid). To output a video where the Scene Strips are rendered, use the Render menu in the top-bar, or change Sidebar ‣ View ‣ Scene Strip Display ‣ Shading to Rendered."

Source: manual, https://docs.blender.org/manual/en/5.1/editors/video_sequencer/preview/header.html — read 2026-07-30. The same words appear at https://docs.blender.org/manual/en/5.1/editors/video_sequencer/sequencer/navigating.html.

**(g) There is a setting about whose render settings a Scene strip uses, but it is narrow.** Documented.

> "use_sequencer_override_scene_strip … Use workbench render settings from the sequencer scene, instead of each individual scene used in the strip (default False)"

Source: API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30. The manual's matching control is described only for preview display:

> "Override Scene Settings — Use the Workbench render settings from the current scene rather than the scenes referenced by the strips. Only available for the Wireframe and Solid shading modes."

Source: manual, https://docs.blender.org/manual/en/5.1/editors/video_sequencer/preview/sidebar.html — read 2026-07-30. It concerns Workbench settings, not compositing and not file writing.

### The one inference

**LABEL: inferred.** If a Scene strip's picture is produced by the brought-in scene's compositor, then the File Output nodes in that compositor write their files, the same as in any other render.

The step a reader has to supply: the manual says "If Compositing is also enabled, the Scene strip will be the output of the Compositor" (quote (b)), and separately says the File Output node "writes one or more images to disk during rendering or compositing" (quote (e)). Joining the two requires assuming that "the output of the Compositor" means the whole node tree was evaluated in the ordinary way, File Output nodes included, rather than only the branch feeding the picture. No source says that. So this is my step, not the manual's, and it could be wrong in either direction — the nodes could be skipped, or they could write once for every frame of the outer timeline on which the strip appears.

**A genuine ambiguity in quote (b), flagged rather than silently resolved.** "If Compositing is also enabled" does not say whose Compositing switch — the sequencer scene's, or the brought-in scene's. Both scenes have one (see (c) and (d)). I read it as the brought-in scene's, because that is the scene whose compositor could produce that strip's picture and because the sentence is about what a Scene strip is made of. The other reading is that it refers to the same scene as the Sequencer switch beside it, that is, the scene being rendered. The sources do not settle it.

**Also flagged: the question itself has two readings.** "Brought into another scene through its video sequence editor" is a Scene strip, and that is the reading I answered. There is a second way a scene is used inside a Scene strip that I did *not* treat as the question: `scene_input` set to `SEQUENCER`, documented as "Sequencer — Use the Scene's Sequencer timeline as input, allowing one scene to reuse another scene's edit (instead of taking the render output from the scene)" (manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/scene.html, read 2026-07-30). On that setting the strip explicitly does not take the scene's render output, so the compositor question does not arise in the same way. The sources say nothing about File Output nodes there either.

**NOT COVERED.** This answer does not settle: whether the brought-in scene's File Output nodes write at all during a Scene strip render; if they do, whether they write once per outer-timeline frame, once per strip, or once per scene frame; what happens when the same scene appears in two strips at once; what happens when the outer render is an animation rather than a single frame; whether the Scene Strip Display "Shading: Rendered" preview path writes files the way a final render would; and whether the muting of a strip or of a node changes any of it.

**Where I looked, and what I ruled out.**

Searched, case-insensitively, across all of `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1`: `file output`, `output node`, `outputfile`, `CompositorNodeOutputFile`, `scene strip`, `strip scene`, `sequenc`, `VSE`, `video sequence editor`, `sequence editor`, `composit`, `scene's compositor`, `compositor of the scene`, `master scene`, `nested scene`, `sub-scene`, `subscene`, `referenced scene`, `another scene`, `other scene`, `secondary scene`, `use_compositing`, `use_sequencer`, `override scene strip`, `sequencer override`, `written to disk`, `writes to disk`, `saved to disk`, `overwrit`, `side effect`, `twice`, `pipeline`.

I also listed every file in the manual that mentions "file output" (five: the general index, `compositing/index`, `compositing/types/output/index`, `compositing/types/output/file_output`, `files/file_paths`) and then checked which of those also mention a strip or the sequencer. Only the general index and, incidentally, the compositing index (which merely lists the new Sequencer Strip Info node) do. On the File Output page itself, the only match for "sequenc" is the word "sequence" in "to create a sequence".

Pages opened in full and ruled out, with the reason:
- File Output node page (https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html) — describes the node completely and never mentions scenes, strips or the sequencer.
- Scene Strip page (https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/scene.html) — describes the strip completely; its only stated limitation is about render passes, not about node trees or written files.
- Sequencer Scene page (https://docs.blender.org/manual/en/5.1/video_editing/sequencer_scene.html) — about which scene holds the edit and how rendering it is offered in the Render menu; nothing about compositing or file writing.
- Post Processing page (https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html) — quoted above; the nearest thing to an answer, and it stops short.
- Compositing introduction, Compositor System and Limitations pages (https://docs.blender.org/manual/en/5.1/compositing/introduction.html and the two beside it) — cover when the compositor is used and how its data works; no sequencer or Scene strip case.
- Render Layers node page (https://docs.blender.org/manual/en/5.1/compositing/types/input/scene/render_layers.html) — this is the *other* way one scene reaches another (a node pointing at a scene), not the sequencer, so it is not this question; noted here only so it is clear I saw it and set it aside.
- Sequencer Strip Info node page (https://docs.blender.org/manual/en/5.1/compositing/types/input/sequencer_strip_info.html) — about a compositor node used by a strip's Compositor *modifier*, which is the reverse direction (compositing applied to a strip), not a Scene strip's own scene.
- Strip Properties, Strip Modifiers, and the whole `video_editing` folder — searched for "composit", "render output" and "file output"; the only hits are the Compositor modifier and unrelated uses of the word.
- Command line pages (https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html and .../arguments.html) — searched for "sequencer", "composit" and "scene strip"; no matches at all.
- Release notes for 5.1 — searched every file for "scene strip", "file output" and "output node". The compositor page has two File Output changes (custom meta-data; directories in item names) and the bug-fix list has three unrelated File Output and Scene strip fixes. None touches this question.
- Cycles' own files — searched for "sequencer", "scene strip", "file_output", "use_sequencer", "use_compositing". The only user-facing hit is the Post Processing panel quoted at (d); the two kernel hits are an unrelated internal use of the word.
- Picture checked: `_images/node-types_CompositorNodeOutputFile.webp`, the only File Output image in the manual. It shows the node with a directory field reading `/tmp/`, a file name field reading `file_name`, and one empty input socket. Nothing about scenes or strips. Documented, from the manual, read 2026-07-30.

**1. Outside the interface.** *(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)* The sources do not say. Nothing I found distinguishes a Scene strip render started by a person from one started from the command line or from Python. What is documented is only that both routes exist and that they choose a scene: the command line has "-S, --scene <name> … Set the active scene <name> for rendering" (manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30), and Python has `bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)` with "use_sequencer_scene (bool) – Use Sequencer Scene, Render the sequencer scene instead of the active scene (optional)" and "scene (str) – Scene, Scene to render, current scene if not specified" (API documentation, https://docs.blender.org/api/5.1/bpy.ops.render.html, read 2026-07-30). One difference is documented but it belongs to the preview, not to the interface as such: the Sequence Render Image and Sequence Render Animation commands do not render Scene strips by default (quote (f)), and those are commands in the Video Sequencer's own menu.

**2. Operating system.** *(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)* The sources do not say. I found no statement anywhere that any of this differs by operating system. The only operating-system-shaped remark near the subject is on the File Output page and concerns path spelling rather than behaviour: "If an input name contains a slash character (/ or \), the path is interpreted as a directory hierarchy" (manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html, read 2026-07-30) — both separators are accepted.

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

**LABEL: not in the sources** — for the "when brought in through the sequencer" part, which is the part the question turns on. The general naming rule is documented and is given below; no source says whether or how it changes for a scene rendered as a Scene strip.

### The naming rule the sources do give (documented)

> "Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence. For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

> "Both the File Name and the Image names (when saving multiple images) can contain #### to force the frame number to appear in a specific place in the filename. This also has the effect that a single render will have the frame number embedded. The frame number will be padded with zeros to fill the number of # characters."

> "When the Media Type is set to Image, input names are also used to construct subdirectories: If an input name contains a slash character (/ or \), the path is interpreted as a directory hierarchy. Any missing directories are created automatically during output."

Source: manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — read 2026-07-30.

The node's two fields, in the API:

> "directory — The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)"

> "file_name — The base name of the file. Other information might be included in the final file name depending on the node options (default "", never None, Supports template expressions)"

Source: API documentation, https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30.

The manual also documents a panel that shows the result:

> "Output Paths … Displays the final file path and extension for each input/layer, based on current settings. This helps verify that outputs are named and organized correctly."

Same manual page and date.

### The part that matters here, and where it stops

The name can contain template expressions, and two of the available ones are written in terms of "the current scene" and "the current render camera":

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node."

> "Available only in render output paths (including in the compositor File Output node): fps: The frames per second of the current scene. resolution_x / resolution_y: The x and y resolution of the rendered image… scene_name: Name of the current scene. camera_name: Name of the current render camera."

> "Available only in path properties on a node: node_name: The name of the node that the path property is on."

Source: manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html — read 2026-07-30.

**LABEL for the scene-strip case: not in the sources.** Two things decide the final name in this situation and neither is settled anywhere:

1. **Which frame number is appended.** The rule says "the frame number", and in a Scene strip render there are two candidates — the frame of the outer timeline being rendered, and the frame of the brought-in scene that the strip maps to. The manual documents that a strip does map between the two, in general terms — "Current Frame — The Playhead's frame number relative to the start of the strip" (manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/strip_properties.html, read 2026-07-30) — but it never says which number a File Output node uses, and it never even says that a Scene strip renders its scene at a remapped frame in so many words.
2. **What "the current scene" resolves to** for `{scene_name}`, `{camera_name}`, `{fps}`, `{resolution_x}` and `{resolution_y}` while a Scene strip is being rendered — the brought-in scene or the scene holding the edit. The template page defines the variables and stops. The Scene strip page documents that the strip can override the camera — "Camera — This can be used to override the scene's camera with any other object" (manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/scene.html, read 2026-07-30) — which makes `{camera_name}` a real question, and one nothing answers.

Everything else about the name is unaffected on its face: the directory, the base file name, the socket or item name, the `####` padding and the extension all come from the node and its scene, and none of them mentions the sequencer.

**Ambiguity in the question, flagged.** I read "what is a file called" as asking for the composed file name, and I have answered that. It could instead be asking only "does the name differ from an ordinary render of that scene" — my answer to that narrower reading is the same: the sources do not say, and the two open points above are exactly where a difference could hide.

**NOT COVERED.** This answer does not settle which frame number lands in the name, what `{scene_name}` and `{camera_name}` expand to during a Scene strip render, whether the strip's own name or channel ever enters a name (nothing suggests it does, but nothing rules it out either), or what happens to the name when the same scene is used by two strips in the same edit.

**Where I looked, and what I ruled out.** The same whole-corpus searches listed under question 16, plus these aimed at naming: `file name`, `filename`, `frame number`, `####`, `template expression`, `scene_name`, `camera_name`, `node_name`, `blend_name`, `Output Paths`, `padded`, `directory`. Pages opened: the File Output node page and the File Paths page (both quoted above, both read in full); the strip Time panel section of https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/strip_properties.html; the Speed Control page (https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/effects/speed_control.html), which has a "Frame Number" mode but is about which frame of a strip's source is shown, not about what a written file is called, and so is ruled out; https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html and https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html, whose per-item properties are `name`, `format`, `override_node_format`, `save_as_render`, `socket_type`, `vector_socket_dimensions` and `color` — none of them scene- or strip-aware. Ruled out: https://docs.blender.org/manual/en/5.1/render/output/properties/output.html and https://docs.blender.org/manual/en/5.1/render/output/animation.html, which name the *scene's own* output path and its Overwrite option, not the node's; and https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html, which has a "Sequencer Strips — Use metadata from the strips in the Sequencer" option but is about metadata stamped inside a file, not about the file's name. The one File Output picture in the manual, `_images/node-types_CompositorNodeOutputFile.webp`, shows only the directory and file-name fields (documented, manual, read 2026-07-30). Release notes searched for "template", "file output" and "scene strip": the only relevant 5.1 change is "The **File Output** node now allows specifying directories in item names. This was possible in older versions of Blender and the functionality is now restored." (release notes, https://developer.blender.org/docs/release_notes/5.1/compositor/, read 2026-07-30), which affects names but says nothing about scenes or strips. Cycles' files searched for `file_output`, `directory`, `file_name`: nothing about this node.

**1. Outside the interface.** *(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)* The sources do not say. Nothing in the naming rule, the template variable list or the node's API entry mentions the interface, a window, or background running, and I found no separate statement about names differing when Blender is driven from outside. One documented remark shows that template errors can stop a render outright, and it is written in terms of rendering rather than of clicking: "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors." (manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html, read 2026-07-30). The other half of that same section is interface-only by its own words — the red highlight and the hover tooltip that lists the errors — so that feedback, but not the failure, is something only a person at the screen gets.

**2. Operating system.** *(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)* The sources do not say that the name differs by operating system. The only related documented point is that both slash characters are accepted inside an item name: "If an input name contains a slash character (/ or \), the path is interpreted as a directory hierarchy" (manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html, read 2026-07-30), and that `directory` supports the blend-relative `//` prefix (API documentation, https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html, read 2026-07-30).

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

## Things a person sitting at a computer with Blender 5.1 open could settle in a few minutes

Each of these answers something the sources leave open. They are written so somebody who cannot see my reasoning can carry them out.

1. **Does a Scene strip render make the brought-in scene's File Output nodes write?** In one blend-file make two scenes, `Shot` and `Edit`. In `Shot`: set frame range 1–10, put any object in front of the camera, open the Compositor, tick Use Nodes, add a File Output node, connect the Render Layers Image output to it, set its Directory to an empty folder and its File Name to `shot_`. In Properties ‣ Output ‣ Post Processing for `Shot`, make sure Compositing is on. In `Edit`: open the Video Sequencer, Add ‣ Scene ‣ Shot, place the strip starting at frame 1; in Properties ‣ Output ‣ Post Processing for `Edit`, make sure Sequencer is on; set `Edit`'s output path to a second empty folder. With `Edit` active, Render ‣ Render Animation. Then look in the first folder. Whether it is empty, or holds ten files, or holds some other number, is the answer to question 16.
2. **Which frame number the files carry.** In the same setup, move the strip in `Edit` so it starts at frame 51 instead of frame 1, and set `Edit`'s frame range to 51–60. Render the animation again into a fresh empty folder for the node. If the files are numbered 0001–0010 the node used the brought-in scene's own frames; if they are numbered 0051–0060 it used the outer timeline's. That is the first half of question 17.
3. **What `{scene_name}` and `{camera_name}` expand to.** Change the File Output node's File Name in `Shot` to `{scene_name}_{camera_name}_` and repeat the render from `Edit`. Read the names of the files produced. Then set the Scene strip's Camera override in `Edit` to a second camera object in `Shot` and render once more, into a fresh folder, and see whether `{camera_name}` follows the override. That is the second half of question 17.
4. **Whether the strip's scene's Compositing switch is the one that matters.** Repeat test 1 twice more: once with Compositing turned off in `Shot` and on in `Edit`, once with it on in `Shot` and off in `Edit`. Which combination produces files settles the ambiguity I flagged in quote (b).
5. **Whether the preview path writes files too.** With the setup from test 1, in the Video Sequencer's Preview set Sidebar ‣ View ‣ Scene Strip Display ‣ Shading to Rendered, then use the Video Sequencer's own Sequence Render Animation command (not the top-bar Render menu) into a fresh folder, and check whether the node's folder gains files.
6. **What happens when the same scene is in two strips at once.** Add a second Scene strip in `Edit`, also pointing at `Shot`, on another channel and overlapping the first. Render the animation into fresh folders and count the files the node produced. This shows whether the same names are written twice for one frame.
7. **Whether muting matters.** Mute the Scene strip in `Edit` (H), render, and check the node's folder; then unmute, mute the File Output node in `Shot` instead (M on the node), render, and check again.

---


## The researcher's own list of things to settle by hand — all of them now run

**Closed on 2026-08-26.** Every check proposed above was carried out, together with the tests the audit sent on and several more, in all four researched versions of Blender and on two computers. The results are written into questions 16 and 17 above as observed findings, and the whole list, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`.

**Only these are still open, and neither can be reached from the equipment this project has:**

- **Windows.** No Windows machine has run it, so every answer above covers macOS on Apple silicon and Ubuntu Linux and leaves Windows untested.
- **The four tests that need Blender's own window, on macOS.** They were run on the rented Linux machine, which has a display server, and not on the Creator's Mac. So the windowed render, the played preview, the sequencer's own render command with a window, and the node's sidebar panel are settled on Linux alone. Each of the first three has a terminal counterpart that was run on both machines.
- **A folder reached over a network**, which has been on this topic's *cannot be run* list from the start: neither machine has a network store.

*(The checks the researcher proposed are left in place above this note rather than deleted, so a reader can see what was asked as well as what came back.)*


---

2026-08-26 — Claude Opus 5 (Claude Code subagent) — File output nodes, Blender 5.1.
