# File output nodes — Architecture questions, second pass (Blender 4.5)

**Questions answered here: 16 and 17 only.** The rest of the sheet was read, but not answered, as instructed.

## Sources consulted, and when

- Blender 4.5 manual, plain-text copy in `Blender Sources/manual_4.5`, pages under `https://docs.blender.org/manual/en/4.5/` — read 2026-07-30 (snapshot date).
- Blender 4.5 Python API documentation, plain-text copy in `Blender Sources/api_4.5`, pages under `https://docs.blender.org/api/4.5/` — read 2026-07-30 (snapshot date).
- Blender 4.5 release notes, in `Blender Sources/release_notes_4.5`, pages under `https://developer.blender.org/docs/release_notes/4.5/` — read 2026-07-30 (snapshot date).
- Cycles' own files, in `Blender Sources/cycles_4.5` — read 2026-07-30 (snapshot date).
- One manual picture, `manual_4.5/_images/render_output_properties_post-processing_panel.png`, opened and looked at.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

(One thing I did not open but did see: my session's start-up context automatically displayed the project's recent git commit subject lines. I did not ask for them and did not act on them; they contain no statements about Blender's behaviour. Nothing else from the project reached me.)

---

## 16. When a scene is brought into another scene through its video sequence editor, what do the File Output nodes belonging to the brought-in scene do?

**LABEL: not in the sources.**

Nothing in the 4.5 manual, the 4.5 Python API documentation, the 4.5 release notes, or Cycles' own files says what a File Output node inside a scene does when that scene is pulled into another scene as a Scene strip. The two subjects never meet anywhere in the four sources: the compositing chapter never mentions strips or the sequencer, and the sequencer and Scene strip pages never mention the compositor's File Output node.

### What the sources do say, near the question

These are the closest documented statements. None of them is an answer to the question; together they are the whole of what is on the page.

**(a) A Scene strip is the other scene's render output.** — documented

> "Scene strips are a way to insert the render output of another scene into your sequence. Instead of rendering out a video, then inserting the video file, you can insert the scene directly."

Manual, https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/scene.html — read 2026-07-30.

The API says the same in one line:

> "Sequence strip using the rendered image of a scene"

API documentation, https://docs.blender.org/api/4.5/bpy.types.SceneStrip.html — read 2026-07-30.

**(b) Scene strips are rendered as part of the render pipeline, and the strip is the Compositor's output.** — documented

> "Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor."

Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html — read 2026-07-30.

This is the strongest thing on the page, and I want to be careful about what it does and does not say. It says the Scene strip's picture is the Compositor's output rather than the raw camera view. It says nothing about File Output nodes, and nothing about files being written.

It is also worded ambiguously about *whose* Compositing switch is meant — the containing scene's or the brought-in scene's. The sentence sits in the description of the containing scene's own Post Processing panel, so the plain reading is that both checkboxes on that panel are the containing scene's. But the phrase "the Scene strip will be the output of the Compositor" only makes sense of the brought-in scene's Compositor, because the strip's picture comes from the brought-in scene. The sources do not resolve this. I am flagging it rather than choosing.

**(c) Each scene has its own compositing node tree and its own Compositing and Sequencer switches.** — documented

> "node_tree — Compositing node tree"

> "use_nodes — Enable the compositing node tree"

API documentation, https://docs.blender.org/api/4.5/bpy.types.Scene.html — read 2026-07-30.

> "use_compositing — Process the render result through the compositing pipeline, if compositing nodes are enabled"

> "use_sequencer — Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist"

API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.

Both of those last two hang off render settings, and render settings belong to a scene. Cycles' own files show the same two switches being drawn from the current scene:

> `rd = context.scene.render`
> `col = layout.column(align=True, heading="Pipeline")`
> `col.prop(rd, "use_compositing")`
> `col.prop(rd, "use_sequencer")`

`Blender Sources/cycles_4.5/ui.py`, class `CYCLES_PT_post_processing`, `draw` method, lines 1112–1116 — read 2026-07-30. **LABEL for this line alone: observed** (this is what the file says; what a render does with it is a further step).

And the picture of the panel shows exactly those two checkboxes, both ticked, with Dither below them: `manual_4.5/_images/render_output_properties_post-processing_panel.png`, shown on https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html — looked at 2026-07-30. **LABEL for the picture: documented.**

**(d) A Scene strip can be fed by the other scene's sequencer instead of its camera.** — documented

> "Sequencer: Use the scene's Sequencer timeline as input, allowing one scene to reuse another scene's edit (instead of taking the render output from the scene)."

Manual, https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/scene.html — read 2026-07-30. The API names the same two choices, `CAMERA` and `SEQUENCER`, with `CAMERA` the default: https://docs.blender.org/api/4.5/bpy.types.SceneStrip.html — read 2026-07-30.

**(e) Only the Combined pass comes through a Scene strip.** — documented

> "Scene strips do not render individual Render Passes; only the Combined render pass will be used."

Manual, https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/scene.html — read 2026-07-30.

**(f) The sequencer's own render commands do not render Scene strips at all.** — documented

> "Sequence Render Image and Sequence Render Animation don't render the final video by default – specifically, they don't render Scene Strips, instead using the preview's shading mode (which is initially Solid).
> To output a video where the Scene Strips are rendered, use the Render menu in the top-bar, or change Sidebar ‣ View ‣ Scene Strip Display ‣ Shading to Rendered."

Manual, https://docs.blender.org/manual/en/4.5/editors/video_sequencer/sequencer/navigating.html — read 2026-07-30. The identical wording appears on https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/header.html — read 2026-07-30.

This matters because it means there are two different ways to render a sequence, and by the manual's own words they treat Scene strips differently. Which one is used decides whether the brought-in scene is rendered at all.

**(g) What a File Output node does on a render, stated generally.** — documented

> "This node writes out an image, for each frame range specified, to the filename entered, as part of a frameset sequence."

> "The image(s) will be saved on rendering, writing to the current frame. An entire sequence of images will be saved, when an animation is rendered."

Manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html — read 2026-07-30. The API's one-line description is "Write image file to disk": https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30.

The page says "on rendering" without ever saying which renders count.

### The step somebody would have to take, written out

**LABEL for this paragraph alone: inferred.** If the containing scene is rendered with Sequencer on, quote (b) says a Scene strip in it "will also be rendered as part of the pipeline", and quote (b) further says that with Compositing on the strip "will be the output of the Compositor" — so the brought-in scene's compositor node tree is evaluated during that render. Quote (g) says a File Output node "will be saved on rendering". Put together, the brought-in scene's File Output nodes would write. Conversely, quote (f) says the sequencer's own Sequence Render Image and Sequence Render Animation do not render Scene strips at the default shading, so on that route the brought-in scene would not be rendered and its File Output nodes would not write. And quote (d) says the `SEQUENCER` input type takes the other scene's edit "instead of taking the render output from the scene", which reads as no render of that scene and so no writing.

I am labelling all of that inferred and I would not lean on it. Every step needs the reader to supply the link between "the compositor runs" and "the File Output node writes", and none of the four sources supplies it. In particular the sources never say whether the Compositing switch consulted for a brought-in scene is that scene's own or the containing scene's, never say whether a File Output node writes when its scene is rendered indirectly, and never say what happens when the same brought-in scene appears in several strips at once.

### Where I looked, and what I ruled out

Searches were run over the whole of `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5` at once, case-insensitively.

Terms searched, and what came back:

- `file output` — in the manual, five files only: the File Output node page, the compositing index, the output-node index, the general index, and the path-templates page. None mentions strips or the sequencer.
- `scene strip`, `scene strips` — the Scene strip page, the sequencer navigating page, the preview header page, the preview sidebar page, the sequencer cache sidebar, the Post Processing page, the strips index and introduction, the montage editing page, the Alpha Over effect page, the manual's own to-do list, one SVG image, four API pages, and two release-note lines. None mentions the compositor's File Output node. Read every one of these that could plausibly carry it.
- `use_sequencer`, `use_compositing` — only the API's `RenderSettings` page, the API index pages, the API change log, the preferences page, and one line of Cycles' `ui.py`. No behavioural statement about File Output.
- `use_sequencer_override_scene_strip` — API `RenderSettings` only, described as "Use workbench render settings from the sequencer scene, instead of each individual scene used in the strip". About Workbench settings, not about writing files. Ruled out.
- `strip` and `sequenc` restricted to the whole `manual_4.5/compositing` folder — `strip` returns nothing at all; `sequenc` returns only "consequences", "image sequence", "frameset sequence" and similar. The compositing chapter never refers to the sequencer or to strips. This is the single strongest piece of evidence for silence.
- `compositor`, `compositing` restricted to `manual_4.5/video_editing` and `manual_4.5/editors/video_sequencer` — six hits, all unrelated: the introduction naming the Compositor as an alternative editing method, a modifier colour-space remark, and the VSE's own "Compositing" sidebar panel, which is blend mode and opacity, not the node compositor. Ruled out by reading.
- `another scene`, `other scene`, `second scene`, `secondary scene`, `different scene` — reached the Render Layers node page, whose hint is about loading multi-layer EXRs from another scene, not about strips; the Scene strip page; and a fixed-crash line in the release notes ("Crash when compositor references another scene"). Ruled out.
- `pipeline`, `render pipeline`, `rendering pipeline` — `render pipeline` and `rendering pipeline` return nothing in the manual. `pipeline` returns the Post Processing page's Pipeline heading (whose body is the manual's own "Todo: Add this information.") and otherwise unrelated pages about stereoscopy, PBR, FBX, glTF, Freestyle and colour management. There is no page in the 4.5 manual describing the render pipeline's order of operations. Ruled out.
- `output node`, `written twice`, `rendered twice`, `re-render`, `each strip`, `per strip` — nothing relevant. Ruled out.
- `sequencer`, `scene strip`, `file_output`, `OutputFile`, `use_sequencer` across all of `cycles_4.5` — exactly one hit, the `use_sequencer` checkbox in `ui.py` quoted above. Cycles' files carry nothing else on this subject, which is what I would expect: this is compositor and sequencer territory, not renderer territory. Ruled out.
- Release notes read in full for `compositor.md` and `sequencer.md`, plus `core.md` and `rendering.md`. The 4.5 compositor notes list new nodes, new options, changed and removed options; nothing about File Output and strips. The 4.5 sequencer notes contain one nearby line — "Scene strips are now invalidated when rendered scene changes" (https://developer.blender.org/docs/release_notes/4.5/sequencer/, read 2026-07-30) — which is about the preview refreshing, not about writing files. Ruled out.

Pages opened and read whole, not merely searched: the File Output node page; the Scene strip page; the Post Processing page; the sequencer navigating page; the preview sidebar page; the compositing introduction; the Compositor System page; the compositing limitations page; the Render Layers node page; the Render Output introduction and Rendering Animations pages; the Output properties page; the path templates page; the video editing introduction and directory structure pages; the command-line render and arguments pages; the API pages for `CompositorNodeOutputFile`, `NodeOutputFileSlotFile`, `NodeOutputFileSlotLayer`, `SceneStrip`, `RenderSettings` (the relevant parts), `Scene` (the relevant parts), and `bpy.ops.render`.

**Not covered.** This answer does not settle whether the files are in fact written, skipped, or written more than once. It does not settle whose Compositing switch governs a brought-in scene. It does not settle what happens with the `SEQUENCER` input type as opposed to the default `CAMERA` one. It does not settle what happens when several strips point at the same scene, or when strips are nested more than one level deep. It does not settle the timing — whether any writing happens before, during or after the containing scene's own output is written. And it says nothing about a scene pulled in by other means, such as the Render Layers node, which the question does not ask about.

**Outside the interface.** *(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)* The sources do not say. Nothing I found distinguishes a render started by a person clicking from a render started from outside Blender's window on this point, because nothing describes the point at all. What the sources do say about driving Blender from outside is narrow and general: the command-line arguments page gives "-S, --scene <name> — Set the active scene <name> for rendering" and "-a, --render-anim — Render frames from start to end (inclusive)" (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30), and the API's render operator takes a scene: "scene (string, (optional, never None)) – Scene, Scene to render, current scene if not specified" (https://docs.blender.org/api/4.5/bpy.ops.render.html, read 2026-07-30). Both let the containing scene be chosen from outside; neither says anything about the brought-in scene's File Output nodes. One caveat that does bear on it: quote (f) above says the sequencer's own Sequence Render commands skip Scene strips, and those are described only as menu items in the sequencer editor; the sources do not say whether or how they are reachable from outside the interface. Anything beyond that would be a guess.

**Operating system.** *(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)* The sources do not say. None of the pages consulted — the Scene strip page, the Post Processing page, the File Output node page, the sequencer navigating page, the path templates page, the Output properties page — contains the words Windows, macOS or Linux, checked by search. No operating-system difference is claimed anywhere in this subject area, and none is denied either.

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

**LABEL: not in the sources.**

The sources give the general naming rule for a File Output node's files, and they give the general rule for the frame number. They never say what those rules produce when the node's scene is rendered because another scene's sequencer pulled it in — in particular, they never say which frame number is used, nor which scene counts as "the current scene" for the parts of the name that depend on a scene.

Because question 16 could not establish that anything is written at all, this answer is about the naming rule that would apply if it is. That limit is part of the answer.

### The naming rule as documented, in general

**(a) The shape of the name.** — documented

> "Unlike the render output filepath, this node uses a base directory and an image name, by default the output path is composed of: {base path}/{file name}{frame number}.{extension}.
> Besides being split into two settings, in all other respects, this setting is treated the same as the render output path."

Manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html — read 2026-07-30.

**(b) The two pieces, in the API.** — documented

> "base_path — Base output path for the image
> Type: string, default "", (never None, blend relative // prefix supported, Supports template expressions)"

API documentation, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30.

> "path — Subpath used for this slot
> Type: string, default "", (never None, Supports template expressions)"

API documentation, https://docs.blender.org/api/4.5/bpy.types.NodeOutputFileSlotFile.html — read 2026-07-30.

**(c) The frame number and its padding, from the render output path rule that quote (a) points at.** — documented

> "When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of # anywhere in the file name (e.g. image_##_test.png translates to image_01_test.png).
> This setting expands Relative Paths where a // prefix represents the directory of the current blend-file."

Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30.

**(d) Template expressions, which 4.5 allows in exactly these paths.** — documented

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."

> "The following variables are currently available in template expressions:
> blend_name: The current blend file's name (without the .blend).
> fps: The frames per second of the current scene.
> resolution_x / resolution_y: The x and y resolution of the rendered image. This factors in the resolution scale as well, so if the scene resolution is 1000x600 and the scale is 50%, then resolution_x and resolution_y will be 500 and 300, respectively."

Manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html — read 2026-07-30.

The release notes state the same as a change made in 4.5:

> "Some file paths in Blender can now contain "template expressions" like `{blend_name}` and `{resolution_x}`, which get replaced by the values they reference when the path is used."
> "Currently this is only enabled on the Scene's render output path and the output paths of the compositor's File Output node."

Release notes, https://developer.blender.org/docs/release_notes/4.5/core/ — read 2026-07-30.

**(e) Substitution happens when the path is used, not when it is typed.** — documented

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

Manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html — read 2026-07-30.

**(f) There is no scene-name variable.** — documented, by the completeness of the list in quote (d). The list of available variables is `blend_name`, `fps`, `resolution_x` and `resolution_y`, and nothing else; the page adds "The set of available variables will expand over time." So in 4.5 a file's name cannot be made to carry the name of the scene that wrote it by this mechanism.

**(g) Multi-layer slots are named inside the file, not by it.** — documented

> "name — OpenEXR layer name used for this slot"

API documentation, https://docs.blender.org/api/4.5/bpy.types.NodeOutputFileSlotLayer.html — read 2026-07-30. Included only because "what the file is called" and "what the layers in it are called" are different things, and this is the only statement in the sources about the second.

### What is missing, precisely

Three parts of the name depend on something the sources leave undefined once a second scene is in the picture:

1. **The frame number.** Quote (c) says the frame number is appended. In a Scene strip the brought-in scene is played back through the strip, so there are two frame numbers in play — the containing scene's and the brought-in scene's — and the sources never say which one reaches the file name. The manual's Scene strip page describes strip timing only in general strip terms (Start, Duration, Strip Offset Start/End, Hold Offset Start/End, and "Current Frame — The Playhead's frame number relative to the start of the strip", https://docs.blender.org/manual/en/4.5/editors/video_sequencer/sequencer/sidebar/strip.html, read 2026-07-30), and never connects any of it to an output file name.

2. **`{fps}`.** Quote (d) defines it as "The frames per second of the current scene". Two scenes are involved and they may have different frame rates; the sources do not say which is "the current scene" at the moment the brought-in scene's path is expanded.

3. **`{resolution_x}` / `{resolution_y}`.** Quote (d) defines these as "The x and y resolution of the rendered image", with the example given in terms of "the scene resolution". Again, two scenes, two resolutions, and no statement of which one applies.

`{blend_name}` is the one that is safe: both scenes live in the same blend-file, so quote (d)'s "The current blend file's name" is unambiguous here. The same goes for the `//` prefix in quote (b) and (c), which resolves to "the directory of the current blend-file" — one blend-file, one directory.

### Where I looked, and what I ruled out

In addition to every search listed under question 16, which covered both questions equally, I ran these for the naming half:

- `template expression` — five files: the path templates manual page, the core release notes, and three API pages (`CompositorNodeOutputFile`, `NodeOutputFileSlotFile`, `RenderSettings`), plus one entry in the API's property-flag list ("This path supports the "{variable_name}" template syntax, which substitutes the value of the referenced variable in place of the template expression.", https://docs.blender.org/api/4.5/bpy_types_enum_items/property_flag_items.html, read 2026-07-30). Read the path-templates page in full, including its Syntax, Format Specifiers, Escape Sequences and Errors sections. No variable relates to a scene's identity, and no sentence mentions strips.
- `frame number` across the whole manual — 30 hits, read every one that touched output paths: the Output properties page, the command-line arguments page, the command-line render page, the File Output node page, the Ocean modifier page, and the strip sidebar's "Current Frame". None says which frame number is used when a scene is rendered indirectly.
- `file_slots`, `layer_slots`, `output_file`, `OutputFile` across the API — reached `CompositorNodeOutputFile`, `CompositorNodeOutputFileFileSlots`, `CompositorNodeOutputFileLayerSlots`, `NodeOutputFileSlotFile`, `NodeOutputFileSlotLayer`. Read all five. None mentions scenes, strips or the sequencer.
- `Relative Paths` — reached the Relative Paths section of the Save & Load page, read in full (https://docs.blender.org/manual/en/4.5/files/blend/open_save.html, read 2026-07-30). It defines `//` as relative to the referencing blend-file and says nothing scene-dependent. Ruled out.
- `rendered scene`, `strip's scene`, `Sequence Render` — the only new thing these found was the API note on `use_render_cache`: "Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)" (https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30). It confirms the phrase "indirectly rendered scenes" exists in Blender's own vocabulary, but says nothing about file names. Ruled out.
- Command-line arguments page, `-o` / `--render-output`: "Set the render path and file name. Use // at the start of the path to render relative to the blend-file. You can use path templating features such as {blend_name} in the path. See Blender's documentation on path templates for more details. The # characters are replaced by the frame number, and used to define zero padding." (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30.) Read in full because an override from outside could in principle change a name. It speaks only of the render output path; it does not say it touches a File Output node's base path or slot paths. Ruled out for this question, with that limit noted below.
- Cycles' own files searched for `path`, `base_path`, `filepath` in the vicinity of output settings — nothing about the File Output node exists there at all, as recorded under question 16. Ruled out.

**Not covered.** This answer does not settle which frame number lands in the name, nor which scene supplies `{fps}` and `{resolution_x}` / `{resolution_y}`. It does not settle whether anything is written at all — see question 16. It does not settle what happens if two strips of the same brought-in scene are on screen at once and the same name would be produced twice. It does not settle whether the containing scene's own output naming interacts with the brought-in scene's in any way. It does not cover names produced by a scene pulled in by any means other than a Scene strip.

**Outside the interface.** *(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)* The naming rule itself, as quoted, is written without reference to the interface: quotes (a), (c) and (d) describe how a path is composed and expanded, and quote (e) says expansion happens "at time of use (e.g. while rendering)". Nothing in them depends on somebody clicking. **LABEL for that sentence alone: inferred** — the step is that the sources describe the rule as a property of the path rather than of any window, and never state an exception for renders started from outside. One documented difference does exist at the edges: the path templates page says errors in a path are shown by the field turning red and by a tooltip listing the errors, both of which are things only visible inside the interface —

> "When there are template errors in a path, the path field will be highlighted red in the UI"

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

Manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html — read 2026-07-30. The red field and tooltip are interface-only by their nature; the failure with an error message is not described as interface-only. Beyond that, the sources do not say whether the naming outcome differs when Blender is driven from outside, and specifically do not say whether the command line's `-o` override reaches a File Output node's paths. I did not find, and would not assume, that it does.

**Operating system.** *(Answered by hand test on 2026-08-26 — see **What the hand tests settled** under this question. What follows is the researcher's record of the sources, and it stands.)* The sources do not say. The path templates page, the Output properties page, the File Output node page and the Relative Paths section contain no mention of Windows, macOS or Linux, checked by search. No operating-system-specific rule about file naming, path separators or drive letters appears in any of them.

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

## Things a person sitting at a computer with Blender 4.5 open could settle in a few minutes

Each of these is written so somebody who cannot see my reasoning can carry it out.

**A. Does a brought-in scene's File Output node write when the containing scene is rendered? (question 16)**

1. Open Blender 4.5. Make a second scene: top-bar scene selector, New Scene, type New. Call it `INNER`. Call the first one `OUTER`.
2. In `INNER`: put any object in front of the camera. Open the Compositing workspace, tick Use Nodes, add a File Output node (Add ‣ Output ‣ File Output), and connect the Render Layers node's Image socket to it. Set its Base Path to an empty folder you can watch, for example `//inner_out/`.
3. In `INNER`, set the frame range Start 1, End 5.
4. Switch to `OUTER`. Open the Video Sequencer. Add ‣ Scene ‣ INNER. Leave the strip's Input on Camera. Put the strip at frame 1.
5. In `OUTER`'s Output properties, set the frame range Start 1, End 5, set File Path to a different empty folder, and in Post Processing confirm both Compositing and Sequencer are ticked.
6. Empty the `inner_out` folder. Render `OUTER`'s animation from the top-bar Render menu ‣ Render Animation.
7. Look in `inner_out`. Record: are there files at all; how many; what exactly are they named, including the digits.

**B. Which frame number ends up in the name? (question 17)**

Repeat A, but in step 4 place the Scene strip so it starts at frame 20 of `OUTER` rather than frame 1, and extend `OUTER`'s range to cover it (Start 1, End 30). Render `OUTER`'s animation. Then look at the digits in the `inner_out` file names: do they run 0001–0005 (the brought-in scene's own frames) or 0020–0024 (the containing scene's frames), or something else. This is the single most valuable thing on this list.

**C. Whose Compositing switch decides?**

Run A once with `INNER`'s Post Processing ‣ Compositing unticked and `OUTER`'s ticked, then again with `INNER`'s ticked and `OUTER`'s unticked, then again with both ticked. Record for each of the three whether files appear in `inner_out`. That settles the ambiguity in the Post Processing sentence quoted under question 16.

**D. Does the Sequencer input type change it?**

Repeat A with the Scene strip's Input set to Sequencer instead of Camera (Sidebar ‣ Strip ‣ Scene ‣ Input). Record whether `inner_out` receives anything.

**E. Do the sequencer's own render commands write anything?**

Repeat A but at step 6 use the Video Sequencer's own Sequence Render Animation instead of the top-bar Render menu, with Scene Strip Display ‣ Shading left at its initial Solid. Record whether `inner_out` receives anything. Then set that Shading to Rendered and repeat. This tests the manual's statement that Sequence Render Animation does not render Scene strips.

**F. Two strips of the same scene at once.**

Repeat A but add the same `INNER` scene as two Scene strips on different channels, overlapping in time. Record whether `inner_out` gets one set of files or two, whether any file is written twice, and whether Blender reports anything.

**G. Which scene supplies `{fps}` and `{resolution_x}`?**

Repeat A, but first give `INNER` and `OUTER` deliberately different frame rates (say 24 and 30) and different resolutions (say 1920×1080 and 1280×720). Set `INNER`'s File Output node Base Path to `//inner_out/f{fps}_x{resolution_x}_`. Render `OUTER`'s animation from the top-bar Render menu. Read the numbers baked into the resulting file names and record which scene's values they are.

**H. Does the command line change the answer?**

Save the file from A as `test.blend`. Empty `inner_out`. From a terminal run `blender -b test.blend -S OUTER -a`. Record whether the same files appear as in A, with the same names. Then repeat with `-o` pointing somewhere else, added before `-a`, and record whether the `inner_out` files move or change name — this settles whether `-o` reaches a File Output node's base path.

---


## The researcher's own list of things to settle by hand — all of them now run

**Closed on 2026-08-26.** Every check proposed above was carried out, together with the tests the audit sent on and several more, in all four researched versions of Blender and on two computers. The results are written into questions 16 and 17 above as observed findings, and the whole list, with the exact scripts, is `Audits/Blender Research/File output nodes/Hand tests — Architecture questions.md`.

**Only these are still open, and neither can be reached from the equipment this project has:**

- **Windows.** No Windows machine has run it, so every answer above covers macOS on Apple silicon and Ubuntu Linux and leaves Windows untested.
- **The four tests that need Blender's own window, on macOS.** They were run on the rented Linux machine, which has a display server, and not on the Creator's Mac. So the windowed render, the played preview, the sequencer's own render command with a window, and the node's sidebar panel are settled on Linux alone. Each of the first three has a terminal counterpart that was run on both machines.
- **A folder reached over a network**, which has been on this topic's *cannot be run* list from the start: neither machine has a network store.

*(The checks the researcher proposed are left in place above this note rather than deleted, so a reader can see what was asked as well as what came back.)*


---

2026-08-26 — Claude Opus 5 (Claude Code subagent) — File output nodes, Blender 4.5.
