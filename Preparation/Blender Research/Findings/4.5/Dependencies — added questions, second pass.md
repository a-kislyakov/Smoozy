# Dependencies — added questions, second pass (Blender 4.5)

Sources consulted, and on what date:

- Blender's manual for 4.5 (`https://docs.blender.org/manual/en/4.5/`), including its images — read 2026-07-30.
- Blender's Python API documentation for 4.5 (`https://docs.blender.org/api/4.5/`) — read 2026-07-30.
- Blender's 4.5 release notes (`https://developer.blender.org/docs/release_notes/4.5/`) — read 2026-07-30.
- Cycles' own files as supplied for 4.5 — read 2026-07-30.
- Blender's requirements page (`https://www.blender.org/download/requirements/`) — read 2026-08-01.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One thing I must declare, because it happened before I could prevent it. The project's `CLAUDE.md` was placed into my context automatically by the tooling, before my first action. I did not open it, and I did not follow it — it tells agents to read `Preparation/Groundwork/Start Here.md`, and I did not open that file or anything it points to. What I saw of `CLAUDE.md` was a signpost with no claims about Blender in it, so nothing in it could have coloured a finding below. I started no helper agents, so there is no one else to vouch for.

Only questions 17 and 18 were in scope. Every other question on the sheet was left alone.

---

## 17. What use does a `.blend` file make of the places it records for writing, besides writing to them?

**Ambiguity flagged.** "The places it records for writing" I read as: the output and cache folders stored inside the `.blend` itself — the render output path, the compositor File Output node's base path, physics cache paths, the fluid cache directory, the geometry-nodes bake path, the ocean modifier's cache path, and proxy directories. The other reading would be the places Blender writes to that are *not* recorded in the file (the temporary directory, the render-cache folder), which are set in Preferences rather than in the file; I have kept those out except where noted, and said so where the line is thin.

**LABEL: documented**, with two parts marked **inferred** below where a reader has to supply a step.

The sources give seven further uses. Each is quoted.

### a. They are read back from

The most common second use is simply reading. The ocean modifier states it plainly:

> "Rather than simulating the ocean data live, it can be baked to files in a given directory. When a simulation is baked, the simulator engine is completely bypassed, and all information for the modifier or texture is retrieved from the baked files."

> "Upon loading the data from these baked files, when a frame of the bake sequence is read, it is cached in memory. This means that accessing loaded frames subsequent times is fast, not incurring the overhead of drive access."

Source: manual, Ocean Modifier — `https://docs.blender.org/manual/en/4.5/modeling/modifiers/physics/ocean.html`. Read 2026-07-30.

The fluid cache directory has a setting that only takes effect on reading:

> "Offset — Frame offset that is used when loading the simulation from the cache. It is not considered when baking the simulation, only when loading it."

Source: manual, Fluid Domain Cache — `https://docs.blender.org/manual/en/4.5/physics/fluid/type/domain/cache.html`. Read 2026-07-30.

The same wording appears in the API, on `cache_frame_offset`:

> "Frame offset that is used when loading the simulation from the cache. It is not considered when baking the simulation, only when loading it."

Source: Python API, `FluidDomainSettings` — `https://docs.blender.org/api/4.5/bpy.types.FluidDomainSettings.html`. Read 2026-07-30.

A physics point cache can be pointed at a folder purely in order to read:

> "External — Allows you to read the cache from a drive using a user-specified file path."

> "Path — Select the directory path to the cache files."

Source: manual, Baking Physics Simulations — `https://docs.blender.org/manual/en/4.5/physics/baking.html`. Read 2026-07-30.

In the API the same property reads:

> "use_external — Read cache from an external location"

Source: Python API, `PointCache` — `https://docs.blender.org/api/4.5/bpy.types.PointCache.html`. Read 2026-07-30.

And the geometry-nodes Bake node is described as a two-way thing from the start:

> "The Bake node allows saving and loading intermediate geometries."

Source: manual, Bake Node — `https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/geometry/operations/bake.html`. Read 2026-07-30.

### b. They are examined to decide whether a frame still needs rendering

The render output path is checked against what is already there:

> "Overwrite — Overwrite existing files when rendering."

Source: manual, Output — `https://docs.blender.org/manual/en/4.5/render/output/properties/output.html`. Read 2026-07-30.

> "use_overwrite — Overwrite existing files while rendering. Type: boolean, default True"

Source: Python API, `RenderSettings` — `https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html`. Read 2026-07-30.

What that check is for is stated in the animation page:

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

> "Just disable the Overwrite option to start rendering where you left off."

> "Using an image format for output, you can use the Frame Step option to render every N'th frame. Then disable Overwrite and re-render with Frame Step set to 1."

Source: manual, Rendering Animations — `https://docs.blender.org/manual/en/4.5/render/output/animation.html`. Read 2026-07-30.

I also opened the picture the Output page names, `_images/render_output_properties_output_panel.png`. It shows the Output panel with a path field reading `/tmp/`, an Overwrite checkbox (ticked), a Placeholders checkbox (unticked), File Extensions (ticked) and Cache Result (unticked). Label for that: **documented**, it is part of the manual. It confirms the two options sit beside the path field; it shows nothing about behaviour.

### c. Files already at that location are treated as claims by other machines

> "Placeholders — Create empty placeholder frames while rendering."

Source: manual, Output — `https://docs.blender.org/manual/en/4.5/render/output/properties/output.html`. Read 2026-07-30.

> "use_placeholder — Create empty placeholder files while rendering frames (similar to Unix 'touch')"

Source: Python API, `RenderSettings` — `https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html`. Read 2026-07-30.

And the hint on the same manual page:

> "Primitive Render Farm — An easy way to get multiple machines to share the rendering workload is to: Set up a shared directory over a network file system. Disable Overwrite, enable Placeholders in the Render Output panel. Start as many machines as you wish rendering to that directory."

Source: manual, Output — `https://docs.blender.org/manual/en/4.5/render/output/properties/output.html`. Read 2026-07-30.

**LABEL for this part: inferred.** The step I took: the manual says several machines rendering into one shared folder will share the workload if Overwrite is off and Placeholders on. For that to divide the work rather than duplicate it, each machine must be looking at the folder and treating another machine's placeholder as a frame already spoken for. The manual never says that in so many words. A reader has to supply it, so this is inferred, not documented.

### d. They are played back from

> "Launching the animation player opens a new window, playing back images or a video located at the render output of the current scene."

Source: manual, Animation Player — `https://docs.blender.org/manual/en/4.5/render/output/animation_player.html`. Read 2026-07-30.

### e. They are turned into filenames, and can be asked for that name without writing anything

**LABEL: documented** that the name is handed back; **inferred** that nothing is written when it is asked for. *(Added by the citation check, 2026-08-06: this section carried the document's overall "documented" label. The quoted words say a path is returned; they do not say that nothing is written.)*

> "frame_path(\*, frame=-2147483648, preview=False, view='') — Return the absolute path to the filename to be written for a given frame"

> "Returns: File Path, The resulting filepath from the scenes render settings"

Source: Python API, `RenderSettings` — `https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html`. Read 2026-07-30.

The recorded path is not taken literally. Substitution happens when it is used:

> "Path templates substitute template expressions (written {abcd}, where “abcd” is a variable name) in a filepath when the path is used."

*(Corrected by the citation check, 2026-08-06: the manual writes “abcd” with its own quotation marks where this had `'abcd'`. Nothing in the claim turns on it.)*

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node."

Source: manual, File Paths — `https://docs.blender.org/manual/en/4.5/files/file_paths.html`. Read 2026-07-30.

The release notes state this is new in 4.5:

> "Some file paths in Blender can now contain \"template expressions\" like `{blend_name}` and `{resolution_x}`, which get replaced by the values they reference when the path is used."

> "Currently this is only enabled on the Scene's render output path and the output paths of the compositor's File Output node."

Source: release notes — `https://developer.blender.org/docs/release_notes/4.5/core/`. Read 2026-07-30.

### f. They are checked for validity, and a bad one stops the render

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

Source: manual, File Paths — `https://docs.blender.org/manual/en/4.5/files/file_paths.html`. Read 2026-07-30.

> "When there are template errors in a path, the path field will be highlighted red in the UI"

> "Hovering over the path field will pop up a tooltip that contains a list of the template errors encountered in that path"

Source: same page. Read 2026-07-30. (I did not open the two named pictures, `files_file_paths-invalid_path_template.png` and `files_file_paths-invalid_path_template_tooltip.png`, because the surrounding text already carries the claim.)

The release notes record the consequence for anything driving Blender from outside:

> "`scene.render.frame_path()` used to be infallible but can now fail if there are errors processing Path Templates in the render output path."

Source: release notes — `https://developer.blender.org/docs/release_notes/4.5/python_api/`. Read 2026-07-30.

### g. A recorded cache path is used as a name, so two files can be made to share one cache

> "Use Library Path — Share the disk cache when the physics object is linked into another blend-file. When this option is enabled, linked versions of the object will reference the same disk cache. Otherwise linked versions of the object will use independent caches."

Source: manual, Baking Physics Simulations — `https://docs.blender.org/manual/en/4.5/physics/baking.html`. Read 2026-07-30.

> "use_library_path — Use this file's path for the disk cache when library linked into another file (for local bakes per scene file, disable this option)"

Source: Python API, `PointCache` — `https://docs.blender.org/api/4.5/bpy.types.PointCache.html`. Read 2026-07-30.

Relatedly, one recorded folder can serve two separate features:

> "Bake Path — Location on disk where the baked data is stored. Note, this path is also used for simulation zones."

Source: manual, Bake Node — `https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/geometry/operations/bake.html`. Read 2026-07-30.

### h. The recorded path overrides one handed to Blender before the file is loaded

> "blender --background --render-output /tmp test.blend --render-frame 1 — …will not render to /tmp because loading the blend-file overwrites the render output that was set."

Source: manual, Command Line Arguments — `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`. Read 2026-07-30.

**LABEL for this part: documented.** The recorded path is read out of the file at load and used to replace whatever was set beforehand — a use of the recorded value that has nothing to do with writing.

### i. Two smaller ones

Existing proxy files at a recorded proxy directory are looked at and can be reused:

> "File — Allows you to use preexisting proxies."

> "Overwrite — Whether to overwrite existing proxy files or keep them."

Source: manual, Strip Proxy & Timecode — `https://docs.blender.org/manual/en/4.5/editors/video_sequencer/sequencer/sidebar/proxy.html`. Read 2026-07-30.

And a recorded cache folder is created if it is not there:

> "If the cache path folder does not exist, it will be created."

Source: manual, Ocean Modifier — `https://docs.blender.org/manual/en/4.5/modeling/modifiers/physics/ocean.html`. Read 2026-07-30.

### Where I looked

Searched the whole 4.5 manual and the whole 4.5 API documentation as text, for: `placeholder`, `overwrite`, `cache director`, `cache folder`, `cache path`, `output path`, `output director`, `disk cache`, `point cache`, `blendcache`, `Use Library Path`, `bake path`, `bake target`, `custom directory`, `custom path`, `temporary director`, `temp director`, `temporary files`, `render farm`, `render-farm`, `non-sequential`, `sequential order`, `distributed render`, `use_overwrite`, `use_placeholder`, `bake_directory`, `frame_path`, `Report Missing Files`, `Find Missing Files`.

Pages opened in full or in part: Output; Rendering Animations; Animation Player; Output index; File Paths; File Output Node; Baking Physics Simulations; Fluid Domain Cache; Particles Emitter Cache; Rigid Body World; Dynamic Paint Canvas; Simulation Nodes; Simulation Zone; Geometry Nodes Baking; Bake Node; Geometry Nodes Modifier; Ocean Modifier; Mesh Sequence Cache Modifier; Timeline editor; Image editor editing; Node editing (Read Render Layers); Topbar (External Data); Command Line Arguments; Blender Directory Layout; Strip Proxy & Timecode; Clip editor sidebar; Preferences File Paths. API pages opened: `RenderSettings`, `PointCache`, `PointCacheItem`, `FluidDomainSettings`, `OceanModifier`, `NodesModifier`, `NodesModifierBake`, `NodesModifierWarning`, `bpy.ops.ptcache`, `bpy.ops.fluid`. Release notes: `core`, `python_api`, `physics`, `pipeline_assets_io`, `bugfixes` (searched for `bake` and `cache`).

Cycles' own files: searched `properties.py` for `cache`, `bake`, `directory`, `filepath`, `StringProperty`. Cycles records no output or cache folder of its own in the file. The only two mentions of writing to disk are a tile cache and a kernel cache, neither of which is a path the `.blend` records:

> `description="Render high resolution images in tiles to reduce memory usage, using the specified tile size. Tiles are cached to disk while rendering to save memory",`

File `properties.py`, in the `use_auto_tile` property, line 1053. **LABEL: observed** (this is what the file says).

Ruled out and why: the Preferences "Render Cache" folder and the Temporary Directory are read back from — "Find the render cache file for the current scene and load it into the Render Result" (manual, Image editor, `https://docs.blender.org/manual/en/4.5/editors/image/editing.html`) and "Reads all the current scene's render layers from cache, as needed" (manual, Node editing, `https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html`) — but both folders are set in Preferences, not recorded in the `.blend`, so they fall outside the question as I read it. The `.blend` records only the switch: "use_render_cache — Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)" (API, `RenderSettings`).

**Not covered.** The sources do not say *how* Blender checks the output location — whether it lists the folder, asks about one file at a time, or something else; nor how often. They do not say whether a zero-length placeholder file counts as "existing" for the Overwrite check, which is the thing the shared-folder hint actually turns on. They do not say what happens when two machines create a placeholder for the same frame at the same instant, nor whether any locking is involved. They do not say whether the Overwrite check looks at the file's size or age, so a truncated or half-written frame left behind by a crash is not covered. Nothing here settles what happens when the recorded folder does not exist at render time, except for the ocean cache path, which is stated to be created.

**Outside the interface.** **Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** Mostly yes, and the sources say so directly for some of it. The recorded output path is read at file load and replaces one set on the command line ("loading the blend-file overwrites the render output that was set", Command Line Arguments), which is a `--background` example. `filepath`, `use_overwrite`, `use_placeholder` and `frame_path()` are ordinary Python API members, and `frame_path()` is explicitly described in the release notes as something that can now fail on a bad template — a failure mode that exists for whatever is driving Blender. Template errors are stated to make "rendering an animation… fail with an error message", with no mention of a window. Two of the uses above are interface-only: the red path field and its tooltip are described as "in the UI", and the Animation Player is reached from "Topbar ‣ Render ‣ View Animation" or Ctrl-F11. For the Overwrite and Placeholders checks specifically, the sources do not say one way or the other whether they behave identically in background mode — the "Primitive Render Farm" hint never mentions how the machines are started.

**Operating system.** **Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** The sources do not distinguish, with one exception, which is about writing failing rather than about these other uses:

> "If you are not allowed to write to the required subdirectory caching will not take place, e.g. if your blend-file path is very long and your operating system has a limit on the path length that is supported."

Source: manual, Particles Emitter Cache — `https://docs.blender.org/manual/en/4.5/physics/particles/emitter/cache.html`. Read 2026-07-30.

The requirements page names Windows, macOS and Linux for 4.5 — "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur)" — source `https://www.blender.org/download/requirements/`, read 2026-08-01. It says nothing about paths or caches. Beyond the path-length note, the sources are silent on operating-system differences here. In particular, nothing says whether the shared-folder scheme behaves the same on a network file system on each operating system.

---

**Observed — the hand tests for this round, run 2026-08-07 on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and 118 GB of free disk, and no display server at all.** Blender 4.5 was started from a terminal with no window, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and the Blender answered `RESOURCES /root/dependencies/res-4.5`. Every render used Cycles on the processor, so nothing here depends on the graphics card. The machine was shared with other topics' work throughout, so nothing here is a timing or memory measurement of a clean machine except where it says what it did about that. The full list of tests, with every set-up written out, is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

Pictures were compared as pictures, never as files. Before that was leaned on, two renders of the same frame of the same file were compared and gave `largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE`, so any difference reported below is a real difference.

**A missing output folder is made.** A file whose render output path pointed into a folder that did not exist was rendered with `--background --factory-startup <file> --render-anim`, and again with the path two folders deep into folders that did not exist. Both worked, exit status 0:

```
EXIT STATUS 0
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/dependencies/work/q17/4.5/render.blend"
Saved: '/root/dependencies/work/q17/4.5/deep3/deep4/frame_0001.png'
Time: 00:00.25 (Saving: 00:00.10)
Blender quit
FILES:
/root/dependencies/work/q17/4.5/deep3
/root/dependencies/work/q17/4.5/deep3/deep4
/root/dependencies/work/q17/4.5/deep3/deep4/frame_0001.png
```

**The output place is looked at, and Blender says so by name.** Five frames were rendered, frames 2 and 4 deleted, `use_overwrite` set to False, and the animation rendered again with no window:

```
skipping existing frame "/root/dependencies/work/q17/4.5/ow/frame_0001.png"
Saved: '/root/dependencies/work/q17/4.5/ow/frame_0002.png'
skipping existing frame "/root/dependencies/work/q17/4.5/ow/frame_0003.png"
Saved: '/root/dependencies/work/q17/4.5/ow/frame_0004.png'
skipping existing frame "/root/dependencies/work/q17/4.5/ow/frame_0005.png"
```

Only the two deleted files carried a new modification time afterwards; the other three kept theirs.

**An empty file counts as a frame already done.** With `use_overwrite` off and `use_placeholder` on, five zero-length files were made at the five names `frame_path()` gave, and the animation rendered:

```
--- five zero-length files placed ---
frame_0001.png 0   frame_0002.png 0   frame_0003.png 0   frame_0004.png 0   frame_0005.png 0
EXIT 0
skipping existing frame "/root/dependencies/work/q17/4.5/ph/frame_0001.png"
skipping existing frame "/root/dependencies/work/q17/4.5/ph/frame_0002.png"
skipping existing frame "/root/dependencies/work/q17/4.5/ph/frame_0003.png"
skipping existing frame "/root/dependencies/work/q17/4.5/ph/frame_0004.png"
skipping existing frame "/root/dependencies/work/q17/4.5/ph/frame_0005.png"
Info: No frames rendered, skipped to not overwrite
--- sizes after ---
frame_0001.png 0   frame_0002.png 0   frame_0003.png 0   frame_0004.png 0   frame_0005.png 0
```

Note the exit status: nothing was rendered and the run still reported success.

**Two Blenders writing into one folder divide the frames between them.** A slower scene (400 samples, 200 by 200, two threads, eight frames) was rendered by two Blenders started five seconds apart into one empty folder, Overwrite off and Placeholders on. Four seconds into the first run the folder held one file, of zero length, for the frame being rendered. Then:

```
--- what the first Blender reported ---
Saved: 'frame_0001.png'
skipping existing frame "frame_0002.png"
Saved: 'frame_0003.png'
skipping existing frame "frame_0004.png"
Saved: 'frame_0005.png'
skipping existing frame "frame_0006.png"
Saved: 'frame_0007.png'
skipping existing frame "frame_0008.png"
--- what the second Blender reported ---
skipping existing frame "frame_0001.png"
Saved: 'frame_0002.png'
skipping existing frame "frame_0003.png"
Saved: 'frame_0004.png'
skipping existing frame "frame_0005.png"
Saved: 'frame_0006.png'
skipping existing frame "frame_0007.png"
Saved: 'frame_0008.png'
```

with all eight files holding a real image afterwards and none claimed twice. This was two Blenders on one machine writing into an ordinary folder; a folder shared between two machines over a network could not be reached — see the *cannot be run* note at the end.

**A bad path template stops a render started from outside, for a single frame as well as for an animation.** A control path with a sound template rendered normally. The faulty one, `{{blend_name.png` with the closing brace missing, was first put to the Blender itself with no window:

```
=== a GOOD template, as a control ===
PATH SET TO '/root/dependencies/work/q17/4.5/tmpl/{{blend_name}}_'
FRAMEPATH OK '/root/dependencies/work/q17/4.5/tmpl/tmplok_0001.png'
EXIT 0
Saved: '/root/dependencies/work/q17/4.5/tmpl/tmplok_0001.png'
=== the BAD template ===
PATH SET TO '/root/dependencies/work/q17/4.5/tmpl/{{blend_name.png'
FRAMEPATH RAISED RuntimeError Error: Parse errors in path '/root/dependencies/work/q17/4.5/tmpl/{{blend_name.png':
--- single frame ---
EXIT 1
Error: Parse errors in path '/root/dependencies/work/q17/4.5/tmpl/{{blend_name.png':
- Invalid or incomplete template expression '{{blend_name.png'.
--- animation ---
EXIT 1
Error: Parse errors in path '/root/dependencies/work/q17/4.5/tmpl/{{blend_name.png':
- Invalid or incomplete template expression '{{blend_name.png'.
```

**The recorded path beats one given before the file on the command line.**

```
--- output given BEFORE the file ---
Saved: '/root/dependencies/work/q17/4.5/out/frame_0001.png'
--- output given AFTER the file ---
Saved: '/root/dependencies/work/q17/4.5/ord/after/x_0001.png'
```

**`frame_path()` names the file that appears, for a frame render and for an animation — and not for a still render.** With the output path `shot_{{blend_name}}_##_`:

```
PREDICTED frame 1: /root/dependencies/work/q17/4.5/fp2/shot_fp2_01_.png
PREDICTED frame 7: /root/dependencies/work/q17/4.5/fp2/shot_fp2_07_.png
--- render frame 7 from the command line ---
Saved: '/root/dependencies/work/q17/4.5/fp2/shot_fp2_07_.png'
ON DISK: shot_fp2_01_.png … shot_fp2_05_.png shot_fp2_07_.png
```

but through `bpy.ops.render.render(write_still=True)` at frame 7, on the same path:

```
PREDICTED /root/dependencies/work/q17/4.5/fp/shot_fp_07_.png
ON DISK ['shot_fp_##_.png']
MATCH False
```

**The file-wide path commands reach an ocean cache path and nothing else that is recorded for writing.** One file was built holding six absolute locations — an image loaded from disk, the render output path, a compositor File Output node's directory, a point cache with External on, a geometry-nodes bake directory, and an ocean modifier's cache path — and the two commands run on it:

```
--- as saved ---
  image ReadTex   : /root/dependencies/work/q17paths/4.5/assets/tex.png
  render output   : /root/dependencies/work/q17paths/4.5/renders/frame_
  file output node: /root/dependencies/work/q17paths/4.5/fileout
  point cache     : /root/dependencies/work/q17paths/4.5/ptcache
  gn bake dir     : /root/dependencies/work/q17paths/4.5/gnbake
  ocean cache     : /root/dependencies/work/q17paths/4.5/oceanbake
MAKE PATHS RELATIVE -> {{'FINISHED'}}
--- after Make Paths Relative ---
  image ReadTex   : //assets/tex.png
  render output   : /root/dependencies/work/q17paths/4.5/renders/frame_
  file output node: /root/dependencies/work/q17paths/4.5/fileout
  point cache     : /root/dependencies/work/q17paths/4.5/ptcache
  gn bake dir     : /root/dependencies/work/q17paths/4.5/gnbake
  ocean cache     : //oceanbake
```

Make Paths Absolute put both back. With all six pointed at a folder that does not exist, `bpy.ops.file.report_missing_files()` named two of them:

```
Warning: Path '/root/dependencies/work/q17paths/4.5/gone' not found, from local data-block 'OBGrid'
Warning: Path '/root/dependencies/work/q17paths/4.5/gone/tex.png' not found, from local data-block 'IMReadTex'
BLEND PATHS: ['/root/dependencies/work/q17paths/4.5/gone', '/root/dependencies/work/q17paths/4.5/gone/tex.png', '/root/dependencies/work/q17paths/4.5/assets/tex.png']
```

`OBGrid` carries the ocean modifier and `IMReadTex` is the image. The render output path, the File Output node, the point cache and the geometry-nodes bake directory were named by nothing.

**This version has no `bpy.data.file_path_foreach` at all**, though it has `file_path_map`:

```
HAS file_path_foreach: False
HAS file_path_map    : True
```

**Two version-dependent shapes had to be asked of the running Blender rather than assumed**, and are recorded because they bear on anything written against this: the compositor tree is reached through `scene.node_tree` in this version, and the File Output node's property is named `base_path` here. (In 5.0, 5.1 and 5.2 they are `scene.compositing_node_group` and `directory`.)

**Which of these settings this version has at all**, asked directly:

```
RenderSettings.use_texture_cache: (no such property)
RenderSettings.use_auto_generate_texture_cache: (no such property)
RenderSettings.use_render_cache: False
RenderSettings.use_overwrite: True
RenderSettings.use_placeholder: False
Preferences render_cache_directory:
Preferences texture_cache_directory: (no such property)
generate_texture_cache in dir(bpy.ops.render): False
denoise_animation in dir(bpy.ops.cycles): True
```

One thing recorded because it was got wrong first: `hasattr(bpy.ops.render, "smoozy_no_such_operator")` returns **True**, so `hasattr` settles nothing about an operator. The lines above use `dir()`.

**Cycles' animation denoiser reads the recorded output place and stops on a frame that is not there.** Five frames were rendered to multi-layer OpenEXR with denoising data passes, frame 3 deleted, and `bpy.ops.cycles.denoise_animation()` called:

```
RENDERED ['dn.blend', 'f_0001.exr', 'f_0002.exr', 'f_0003.exr', 'f_0004.exr', 'f_0005.exr']
AFTER DELETING FRAME 3 ['dn.blend', 'f_0001.exr', 'f_0002.exr', 'f_0004.exr', 'f_0005.exr']
Error: Frame '/root/dependencies/work/denoise/4.5/f_0003.exr' not found, animation must be complete
```

In this version `image_settings.file_format = 'OPEN_EXR_MULTILAYER'` is enough to choose that format; in 5.0, 5.1 and 5.2 that name is not in the list until `image_settings.media_type = 'MULTI_LAYER_IMAGE'` is set first.

**Observed — the Mac half of this round's hand tests, run 2026-08-07 on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card.** Blender 4.5.12 LTS was started from a terminal, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and this Blender answered `RESOURCES …/depwork/res-4.5` and `VERSION 4.5.12 LTS`. The application used was `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`. Every render used Cycles on the processor. **Other topics were running Blender on this same Mac throughout, so nothing was timed and no memory was measured; there is no timing or memory reading anywhere below.** **This is an addition to a document written earlier in this round; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

In the machine's own words quoted below, the working folder's long path is written `…/depwork`; nothing else in any quotation is altered.

Pictures were compared as pictures, never as files. Before that was leaned on, two renders of the same frame of the same file were compared on this Mac and gave `largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE`, so any difference reported below is a real difference.

**Everything the Linux half found about the output place holds on macOS too.** Taken in order:

*A missing output folder is made.* The same file rendered with `--background --factory-startup <file> --render-anim`, and again with `--render-output <workdir>/deep3/deep4/frame_ --render-frame 1` into two folders that did not exist. Both worked, exit status 0, nothing said about the folder:

```
EXIT STATUS 0
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "…/depwork/q17/4.5/render.blend"
Saved: '…/depwork/q17/4.5/deep3/deep4/frame_0001.png'
Time: 00:00.16 (Saving: 00:00.05)
Blender quit
FILES:
…/depwork/q17/4.5/deep3
…/depwork/q17/4.5/deep3/deep4
…/depwork/q17/4.5/deep3/deep4/frame_0001.png
```

*The output place is looked at before writing.* Five frames rendered, frames 2 and 4 deleted, `use_overwrite` set to False, the animation rendered again with no window:

```
skipping existing frame "…/depwork/q17/4.5/ow/frame_0001.png"
Saved: '…/depwork/q17/4.5/ow/frame_0002.png'
skipping existing frame "…/depwork/q17/4.5/ow/frame_0003.png"
Saved: '…/depwork/q17/4.5/ow/frame_0004.png'
skipping existing frame "…/depwork/q17/4.5/ow/frame_0005.png"
```

and only the two deleted files carried a new modification time afterwards:

```
before   frame_0001.png 1786065788 8525     after   frame_0001.png 1786065788 8525
         frame_0003.png 1786065788 8525             frame_0002.png 1786065792 8525
         frame_0005.png 1786065788 8525             frame_0003.png 1786065788 8525
                                                    frame_0004.png 1786065792 8525
                                                    frame_0005.png 1786065788 8525
```

*An empty file counts as a frame already done.* With `use_overwrite` off and `use_placeholder` on, five zero-length files were put at the five names `frame_path()` gave:

```
--- five zero-length files placed ---
frame_0001.png 0   frame_0002.png 0   frame_0003.png 0   frame_0004.png 0   frame_0005.png 0
--- render the animation, no window ---
EXIT 0
skipping existing frame "…/depwork/q17/4.5/ph/frame_0001.png"
skipping existing frame "…/depwork/q17/4.5/ph/frame_0002.png"
skipping existing frame "…/depwork/q17/4.5/ph/frame_0003.png"
skipping existing frame "…/depwork/q17/4.5/ph/frame_0004.png"
skipping existing frame "…/depwork/q17/4.5/ph/frame_0005.png"
Info: No frames rendered, skipped to not overwrite
--- sizes after ---
frame_0001.png 0   frame_0002.png 0   frame_0003.png 0   frame_0004.png 0   frame_0005.png 0
```

Nothing was rendered, all five were still zero length, and the run still reported success.

*A bad path template stops a render started from outside, for a single frame as well as for an animation*, with a sound template as the control:

```
=== a GOOD template, as a control ===
PATH SET TO '…/depwork/q17/4.5/tmpl/{blend_name}_'
FRAMEPATH OK '…/depwork/q17/4.5/tmpl/render_0001.png'
EXIT 0
Saved: '…/depwork/q17/4.5/tmpl/tmplok_0001.png'
=== the BAD template ===
PATH SET TO '…/depwork/q17/4.5/tmpl/{blend_name.png'
FRAMEPATH RAISED RuntimeError Error: Parse errors in path '…/depwork/q17/4.5/tmpl/{blend_name.png':
--- single frame ---
EXIT 1
Error: Parse errors in path '…/depwork/q17/4.5/tmpl/{blend_name.png':
- Invalid or incomplete template expression '{blend_name.png'.
--- animation ---
EXIT 1
Error: Parse errors in path '…/depwork/q17/4.5/tmpl/{blend_name.png':
- Invalid or incomplete template expression '{blend_name.png'.
--- what is in the folder ---
tmplok_0001.png
```

One thing about that control worth recording, because it is a difference in how the test was set up and not a difference in Blender: on this Mac `frame_path` was asked **before** the file had been saved under its new name, so `{blend_name}` still stood for the old name and it answered `render_0001.png`. The file that appeared on disk when the render ran was `tmplok_0001.png`, which is what the template says.

*The recorded path beats one given before the file on the command line.*

```
--- output given BEFORE the file ---
Saved: '…/depwork/q17/4.5/tmplsrc/out/frame_0001.png'
--- output given AFTER the file ---
Saved: '…/depwork/q17/4.5/ord/after/x_0001.png'
--- what landed where ---
…/depwork/q17/4.5/ord/after/x_0001.png
```

*`frame_path()` names the file that appears, for a frame render and for an animation — and not for a still render.* With the output path `shot_{blend_name}_##_`:

```
PATH '…/depwork/q17/4.5/fp2/shot_{blend_name}_##_'
PREDICTED frame 1: …/depwork/q17/4.5/fp2/shot_fp2_01_.png
PREDICTED frame 7: …/depwork/q17/4.5/fp2/shot_fp2_07_.png
--- render frame 7 from the command line ---
Saved: '…/depwork/q17/4.5/fp2/shot_fp2_07_.png'
--- render the animation ---
Saved: '…/depwork/q17/4.5/fp2/shot_fp2_01_.png' … '…/depwork/q17/4.5/fp2/shot_fp2_05_.png'
ON DISK: shot_fp2_01_.png shot_fp2_02_.png shot_fp2_03_.png shot_fp2_04_.png shot_fp2_05_.png shot_fp2_07_.png
```

but through `bpy.ops.render.render(write_still=True)` at frame 7, on the same path:

```
PATH '…/depwork/q17/4.5/fp/shot_{blend_name}_##_'
PREDICTED …/depwork/q17/4.5/fp/shot_render_07_.png
ON DISK ['shot_render_##_.png']
MATCH False
```

*The file-wide path commands reach an ocean cache path and nothing else recorded for writing.* The same six recorded locations were built into one file — an image loaded from disk, the render output path, a compositor File Output node's directory, a point cache with External on, a geometry-nodes bake directory, and an ocean modifier's cache path:

```
TREE ATTR scene.node_tree | FILE OUTPUT NODE PROPERTY base_path
--- as saved ---
  image ReadTex   : …/depwork/q17paths/4.5/assets/tex.png
  render output   : …/depwork/q17paths/4.5/renders/frame_
  file output node: …/depwork/q17paths/4.5/fileout
  point cache     : …/depwork/q17paths/4.5/ptcache
  gn bake dir     : …/depwork/q17paths/4.5/gnbake
  ocean cache     : …/depwork/q17paths/4.5/oceanbake
MAKE PATHS RELATIVE -> {'FINISHED'}
--- after Make Paths Relative ---
  image ReadTex   : //assets/tex.png
  render output   : …/depwork/q17paths/4.5/renders/frame_
  file output node: …/depwork/q17paths/4.5/fileout
  point cache     : …/depwork/q17paths/4.5/ptcache
  gn bake dir     : …/depwork/q17paths/4.5/gnbake
  ocean cache     : //oceanbake
MAKE PATHS ABSOLUTE -> {'FINISHED'}
--- after Make Paths Absolute ---
  image ReadTex   : …/depwork/q17paths/4.5/assets/tex.png
  … the other four unchanged …
  ocean cache     : …/depwork/q17paths/4.5/oceanbake
```

With all six pointed at a folder that does not exist, `bpy.ops.file.report_missing_files()` named two of them:

```
Warning: Path '…/depwork/q17paths/4.5/gone' not found, from local data-block 'OBGrid'
Warning: Path '…/depwork/q17paths/4.5/gone/tex.png' not found, from local data-block 'IMReadTex'
REPORT MISSING FILES -> {'FINISHED'}
BLEND PATHS: ['…/depwork/q17paths/4.5/gone', '…/depwork/q17paths/4.5/gone/tex.png', '…/depwork/q17paths/4.5/assets/tex.png']
```

`OBGrid` carries the ocean modifier and `IMReadTex` is the image. The render output path, the File Output node, the point cache and the geometry-nodes bake directory were named by nothing.

One thing about the set-up, recorded because it had to be worked around: on this Mac, **saving the file turned the image path and the ocean cache path into relative ones by itself**, which would have left Make Paths Relative with nothing to do and half the test blind. All six were therefore set back to absolute after the save, and only then were the two commands run. What is quoted above is that second run.

*Cycles' animation denoiser reads the recorded output place and stops on a frame that is not there.* Five frames rendered to multi-layer OpenEXR with denoising data passes, frame 3 deleted, and `bpy.ops.cycles.denoise_animation()` called:

```
FILE FORMAT OPEN_EXR_MULTILAYER
RENDERED ['dn.blend', 'f_0001.exr', 'f_0002.exr', 'f_0003.exr', 'f_0004.exr', 'f_0005.exr']
AFTER DELETING FRAME 3 ['dn.blend', 'f_0001.exr', 'f_0002.exr', 'f_0004.exr', 'f_0005.exr']
Error: Frame '…/depwork/denoise/4.5/f_0003.exr' not found, animation must be complete
DENOISE ANIMATION RAISED RuntimeError Error: Frame '…/depwork/denoise/4.5/f_0003.exr' not found, animation must be complete
```

*Which of these settings this version has at all*, asked directly on this machine:

```
RenderSettings.use_texture_cache                none
RenderSettings.use_auto_generate_texture_cache  none
RenderSettings.use_render_cache                 False
RenderSettings.use_overwrite                    True
RenderSettings.use_placeholder                  False
PointCache.compression                          'NO'
   its choices                                 NO, LIGHT, HEAVY
Preferences render_cache_directory              ''
Preferences texture_cache_directory             'none'
generate_texture_cache in dir(bpy.ops.render)   False
denoise_animation in dir(bpy.ops.cycles)        True
merge_images in dir(bpy.ops.cycles)             True
hasattr(bpy.ops.render, 'smoozy_no_such_operator') -> True
```

**This version has no `bpy.data.file_path_foreach` at all on macOS either**, though it has `file_path_map`:

```
HAS file_path_foreach: False
HAS file_path_map    : True
```

**This version has no texture cache to test on macOS either.** `scene.render.use_texture_cache` does not exist in it: `this version has a texture cache setting: False`.

### What the window could be asked, and what it could not

The Creator's Mac has a screen, so this is the machine where the drawing questions can be reached. Blender was started there with a window and a `--python` script, and the script did its work as it ran. **One limit shaped all of it and is recorded because it decides what could and could not be asked**: when Blender is started this way from a terminal on this machine, the window is created and drawn — the screenshots below are of a fully drawn Blender — but **its event loop never runs**. A timer registered with `bpy.app.timers.register` never fired once in any attempt, `bpy.ops.wm.redraw_timer` never returned, and `bpy.ops.wm.quit_blender()` never quit, so each script ended the process itself after flushing what Blender had printed. So anything needing an event — hovering over a control, or a redraw after something is changed — could not be reached, and anything the window already shows when the file opens could.

**The path field is drawn red for a bad template.** A file whose render output path reads `//out/{blend_name.png` was saved with its Properties editor already on the Output tab, opened with a window, and photographed with Blender's own `bpy.ops.screen.screenshot`. The same was done with `//out/{blend_name}.png`, a sound template, as the control. Reading the average colour of the strip of the picture holding the path field:

```
the bad template   average colour r=0.294 g=0.186 b=0.187
the good template  average colour r=0.199 g=0.200 b=0.201
```

In the bad one the red is about twice the green and the blue; in the control all three are equal, which is Blender's ordinary grey. The field itself reads `//out/{blend_name.png`.

**The tooltip could not be reached at all**, because reading it means hovering, and hovering is an event. That half of the question stays open, and it is open for the reason above rather than for want of a screen.

**Overwrite-off skips frames inside Blender's own window as well.** Entry 2's folder was rebuilt with frames 2 and 4 deleted and Overwrite off, and the file opened with a window and rendered by a script calling `bpy.ops.render.render(animation=True)`:

```
before
   frame_0001.png 1786068024 8525
   frame_0003.png 1786068024 8525
   frame_0005.png 1786068024 8525
render(animation=True) -> {'FINISHED'}
after
   frame_0001.png 1786068024 8525
   frame_0002.png 1786068025 8533
   frame_0003.png 1786068024 8525
   frame_0004.png 1786068025 8533
   frame_0005.png 1786068024 8525
--- what the window Blender printed ---
skipping existing frame "…/depwork/q17/4.5/wow/frame_0001.png"
Saved: '…/depwork/q17/4.5/wow/frame_0002.png'
skipping existing frame "…/depwork/q17/4.5/wow/frame_0003.png"
Saved: '…/depwork/q17/4.5/wow/frame_0004.png'
skipping existing frame "…/depwork/q17/4.5/wow/frame_0005.png"
```

Only the two missing frames carry a new modification time. This version names the skipped frames on the console in a window run exactly as it does with no window.

**The Animation Player does play what is at the recorded output place.** With five frames sitting at the recorded place, a script called `bpy.ops.render.play_rendered_anim()` and then listed the running processes:

```
play_rendered_anim() -> {'FINISHED'}
    51545 /Applications/Blender 4.5.12.app/Contents/MacOS/Blender --factory-startup …/depwork/q17/4.5/tmplsrc/render.blend --python …
    51549 /Applications/Blender 4.5.12.app/Contents/MacOS/Blender --gpu-backend metal -a -f 24 1.0 -s 1 -e 5 -j 1 -c 4096 …/depwork/q17/4.5/wow/frame_0001.png
```

So the player is a second Blender started with `-a`, and the last thing it is handed is the first frame at the file's own recorded output place. As a control the same was done with the recorded place pointing at an **empty** folder. That control was run once, in Blender 4.5 on this same Mac, and its own words are quoted here; the player was still started at that place and said so:

```
wm_main_playanim_intern: '…/depwork/q17/4.5/emptyout/frame_0001.png' not an image file
```

so the operator reads the recorded place rather than doing something generic.

**The viewport render uses the recorded output place.** With the recorded place an empty folder, a script called `bpy.ops.render.opengl(animation=True)` with the 3D viewport as its context:

```
render output path: '…/depwork/q17/4.5/glout/gl_'
that folder before: []
3D viewport found: True
render.opengl(animation=True) -> {'FINISHED'}
that folder after: ['gl_0001.png', 'gl_0002.png', 'gl_0003.png', 'gl_0004.png', 'gl_0005.png']
```


**Both standing lines, brought up to date by these tests.**

*Driven from outside.* **Settled, and mostly yes.** Every reading in the Linux block was taken with Blender running with no window at all, on a machine that has no display server. Looking at the output place before writing, treating an empty file as a frame already done, two Blenders sharing a folder, refusing a bad template with a non-zero exit, making a missing output folder, handing back a computed path, the file-wide path commands, the path walk and the animation denoiser all hold there. **The part that draws has since been reached on the Creator's Mac and is settled too**: the path field is drawn red for a bad template and grey for a sound one; Overwrite-off skips frames inside the window as well as outside it; the animation player is a second Blender started with `-a` and handed the first frame at the file's own recorded output place; and the viewport render writes into that same recorded place. **One thing is still not reached: the tooltip on the path field**, because reading it means hovering, and no window driven this way could be made to answer an event.

*Operating system.* **Now settled on two operating systems, Linux and macOS, and each machine is its own result.** Every test above was run on the rented Linux machine on 2026-08-07 and again on the Creator's Mac on 2026-08-07, and the two agree everywhere except where the Mac block below says plainly that they do not. Nothing here is folded into one sentence across the two machines. **Windows is untried: no Windows machine has run it.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

*Cannot be run at all with this project's equipment:* a folder shared between two machines over a network file system, which is the arrangement the sources actually describe, and therefore also what happens when two machines write a placeholder for the same frame at the same instant. There is one Linux machine and one Mac with no shared network folder between them.


## 18. What does a render do when a cache or a bake that a file records is absent?

**Ambiguity flagged.** "A cache or a bake that a file records" can be read two ways. The reading I answer is: the file records a cache or bake — either as data inside itself or as a path pointing at one on disk — and at render time that cache or bake is not there. The other reading is narrower: only a *path* recorded in the file, whose files have gone missing. I cover both below, and say which case each finding belongs to, because the sources handle them differently.

**LABEL: not in the sources**, for the question as asked. With one documented exception and several documented near-misses, all set out below.

I could not find any page in the 4.5 manual, the 4.5 API documentation, the 4.5 release notes, or Cycles' own files that states what a render does when it meets an absent cache or bake. There is no general statement, no error message quoted, and no fallback described for the physics caches, the fluid cache, the geometry-nodes bakes, or the ocean bake.

### The one documented case

EEVEE's light probe volumes have a stated fallback:

> "If an object is not inside any Irradiance Volume, or if the indirect lighting has not been baked, the world's diffuse lighting will be used to shade it."

Source: manual, Light Probe Volume — `https://docs.blender.org/manual/en/4.5/render/eevee/light_probes/volume.html`. Read 2026-07-30. **LABEL: inferred.** *(Changed by the citation check, 2026-08-06: the label read "documented". The quoted sentence says "Irradiance Volume" while the page is headed "Light Probe Volume", and nothing quoted says the two names mean the same thing.)*

Note what this does and does not settle. It is a bake with no recorded path — the same page says "Once baked, the data is stored inside the object data-block and can be moved, animated and linked" — so it belongs to the first reading of the question, not the second. It says what happens when the bake was never made. It does not say what happens if a bake exists but its data cannot be read.

### The near-misses, in the second reading

The closest thing to a documented answer for a *recorded path* is the ocean modifier, and it is about frames outside the baked range rather than about the files being gone:

> "Frames of the simulation to bake (inclusive). The start and end frames of the bake are repeated when accessing frames outside of the baked range."

Source: manual, Ocean Modifier — `https://docs.blender.org/manual/en/4.5/modeling/modifiers/physics/ocean.html`. Read 2026-07-30. **LABEL: documented**, for that narrow case only.

The API records that the ocean modifier is in one of two states:

> "is_cached — Whether the ocean is using cached data or simulating"

Source: Python API, `OceanModifier` — `https://docs.blender.org/api/4.5/bpy.types.OceanModifier.html`. Read 2026-07-30.

This tells you the two states exist. It does not say which one an absent bake puts it in, and the manual's statement that "the simulator engine is completely bypassed" when baked does not tell you what happens when the baked files are missing. I will not guess.

For geometry-nodes simulations, what the manual says is why you bake, not what happens if you have not:

> "When the result is ready to be sent to a render-farm, it can be baked to disk. This allows for the simulation to be rendered in a non-sequential order."

Source: manual, Simulation Nodes — `https://docs.blender.org/manual/en/4.5/physics/simulation_nodes.html`, and the same two sentences again at Simulation Zone — `https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/simulation/simulation_zone.html`. Read 2026-07-30.

**LABEL: inferred**, and only this much: if baking to disk is what *allows* non-sequential rendering, then without that bake the simulation is not safe to render out of order. The step a reader must supply is that "allows" means "is required for". The sources do not say what the wrong result looks like — an empty frame, a frame at the wrong simulation state, or something else. I could not find that anywhere.

For physics point caches, the manual describes filling the cache, not rendering without one:

> "The result of a simulation is automatically cached to memory when the animation is played, so that the next time it runs, it can be replayed more quickly by reading the results from the memory."

Source: manual, Baking Physics Simulations — `https://docs.blender.org/manual/en/4.5/physics/baking.html`. Read 2026-07-30.

> "When an animation is played, each physics system writes each frame to the cache. Note that for the cache to fill up, one has to start the playback before or on the frame that the simulation starts."

Source: manual, Particles Emitter Cache — `https://docs.blender.org/manual/en/4.5/physics/particles/emitter/cache.html`. Read 2026-07-30.

Both sentences are about playback. Neither mentions rendering. It would be easy to carry them across to a render and say what a render does with a missing cache; that carry-across is exactly what I am not entitled to make, so I have not.

The same page also records a case in which a cache silently never gets written:

> "If you are not allowed to write to the required subdirectory caching will not take place, e.g. if your blend-file path is very long and your operating system has a limit on the path length that is supported."

Source: manual, Particles Emitter Cache — same address. Read 2026-07-30. This is a route to an absent cache, but it does not say what a render then does.

And for the external point cache, the file records a path and a naming rule, with a caution that a mismatch stops it working:

> "The cache name in Caches List and the Index has to exactly match the external cache files name in order to work. The cache files name format is name_frame_index.bphys."

Source: manual, Baking Physics Simulations — `https://docs.blender.org/manual/en/4.5/physics/baking.html`. Read 2026-07-30. "In order to work" is as far as it goes; what "not working" looks like in a render is not stated.

### Whether anything reports it

The sources describe places where the *state* of a cache is shown, all of them inside Blender's window.

> "Baked simulations will be shown as fully opaque, cached simulations will be slightly transparent, and invalid caches will be slightly transparent with dark diagonal stripes."

Source: manual, Timeline — `https://docs.blender.org/manual/en/4.5/editors/timeline.html`. Read 2026-07-30. I opened the picture that page names, `_images/editors_timeline_cache.png`. It shows a short timeline strip with a thin solid bar beneath it and a playhead at frame 33. It is too small to distinguish the three states the text describes, so I take nothing from it beyond confirming that a cache bar is drawn under the timeline. **LABEL: documented** (it is part of the manual), and nothing more read into it.

There is a status string in the API:

> "info — Info on current cache status. Type: string, default \"\", (readonly, never None)"

Source: Python API, `PointCache` — `https://docs.blender.org/api/4.5/bpy.types.PointCache.html`. Read 2026-07-30.

The documentation does not say what that string contains, or whether it ever names a missing cache.

Geometry-nodes modifiers can carry warnings:

> "Warning created during evaluation of a geometry nodes modifier"

Source: Python API, `NodesModifierWarning` — `https://docs.blender.org/api/4.5/bpy.types.NodesModifierWarning.html`. Read 2026-07-30.

> "Nodes that show a warning message in the node editor will also show that message here."

Source: manual, Geometry Nodes Modifier — `https://docs.blender.org/manual/en/4.5/modeling/modifiers/generate/geometry_nodes.html`. Read 2026-07-30.

Neither says a missing bake produces one.

Blender's general missing-file check is described only for linked files, not caches:

> "Report Missing Files — This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files."

Source: manual, Topbar — `https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html`. Read 2026-07-30. Whether recorded cache and bake folders are among the "external files" it checks is not stated anywhere I could find.

### Where I looked

Searched the whole 4.5 manual as text for: `not baked`, `unbaked`, `no bake`, `without baking`, `has not been baked`, `not been baked`, `missing cache`, `cache is missing`, `cache missing`, `if the cache`, `when the cache`, `if no cache`, `if the bake`, `without a cache`, `cache is not`, `bake is not`, `no longer valid`, `invalid cache`, `outdated`, `recalculat`, `simulating live`, `live simulation`, `simulate live`, `falls back`, `fall back`, `fallback`, `will not be rendered`, `nothing will`, `will be empty`, `appear empty`, `disappear`, `no effect on the render`, `render will`, `renders as`, `not found`, `cannot be found`, `could not be found`, `missing file`, `report missing`, `free cache`, `Free All`, `free bake`, `blendcache`, `render farm`, `non-sequential`, `from the first frame`, `from the start frame`, `out of order`.

Searched the whole 4.5 API documentation for: `Cannot read cache`, `cache file`, `bake data`, `Bake failed`, `no bake`, `not baked`, `bake_directory`.

Searched the 4.5 release notes for `bake` and `cache`. What is there is fixes and one import note, none of which describes an absent cache at render time — for example "Fix for hollow interior when importing OpenVDB cache" (`https://developer.blender.org/docs/release_notes/4.5/physics/`) and, in the bug-fix list, "Regression Geometry Nodes: Bake gets discarded with Booleans sometimes" (`https://developer.blender.org/docs/release_notes/4.5/bugfixes/`). Read 2026-07-30.

Pages opened in full or in part while hunting: Baking Physics Simulations; Fluid Domain Cache; Fluid Introduction; Particles Emitter Cache; Rigid Body World; Rigid Body Tips; Dynamic Paint Canvas; Physics Introduction; Simulation Nodes; Simulation Zone; Geometry Nodes Baking; Bake Node; Geometry Nodes Modifier; Ocean Modifier; Mesh Sequence Cache Modifier; Data-Blocks; Light Probe Volume; Timeline; Topbar; Rendering Animations; Output; Command Line Arguments. API pages opened: `PointCache`, `PointCacheItem`, `FluidDomainSettings`, `OceanModifier`, `NodesModifier`, `NodesModifierBake`, `NodesModifierWarning`, `RenderSettings`, `bpy.ops.ptcache`, `bpy.ops.fluid`.

Cycles' own files: searched `properties.py` for `cache` and `bake`. Nothing there concerns an absent cache or bake at render time; the two matches are the tile cache and the kernel optimization cache, quoted under question 17. Cycles' operator file and engine file were searched by the same terms with no relevant match.

Ruled out and why: the pages on caching all describe how to make a cache and what the settings mean; none carries the opposite case. The pages that come closest — the ocean bake, the simulation zone, the EEVEE probe volume — are quoted above, and only the last of the three is a real answer.

**Not covered.** This answer settles very little. It does not settle what a render does when a fluid cache directory is empty or gone, what it does when a geometry-nodes bake folder is missing, what it does when an external point cache path points nowhere, or whether any of these produce an error, a warning, a silent wrong frame, or a silent empty frame. It does not settle whether a render stops or continues. It does not settle whether a missing cache is reported anywhere a machine could read. It does not settle whether the answer differs between a cache that was never made and one whose files were deleted after the fact. For the one documented case, the EEVEE light probe volume, it settles only the never-baked case.

**Outside the interface.** **Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** The sources do not say. Every place that reports cache state which I could find — the Timeline's cache bars, the node editor's warnings, the Info editor's header — is part of Blender's window. The one documented behaviour, the light probe volume falling back to the world's diffuse lighting, is described as a property of the shading and mentions no interface, so on its face it should hold anywhere; but the page does not address the question, and I will not turn silence into a yes. Whether an absent cache or bake produces anything a caller outside Blender could detect — a message on the console, a non-zero exit, an entry in an operator's report — is not stated anywhere in these sources. Given the question, that gap is the important one.

**Operating system.** **Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** The sources do not say. Nothing I found distinguishes operating systems on this point. The only operating-system-specific statement anywhere nearby is the path-length note quoted under question 17, which is about a cache failing to be written rather than about what a render then does. The requirements page names Windows, macOS and Linux for 4.5 and says nothing about caches — `https://www.blender.org/download/requirements/`, read 2026-08-01.

---

**Observed — the hand tests for this round, run 2026-08-07 on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and 118 GB of free disk, and no display server at all.** Blender 4.5 was started from a terminal with no window, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and the Blender answered `RESOURCES /root/dependencies/res-4.5`. Every render used Cycles on the processor, so nothing here depends on the graphics card. The machine was shared with other topics' work throughout, so nothing here is a timing or memory measurement of a clean machine except where it says what it did about that. The full list of tests, with every set-up written out, is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

Pictures were compared as pictures, never as files. Before that was leaned on, two renders of the same frame of the same file were compared and gave `largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE`, so any difference reported below is a real difference.

**An unbaked physics simulation is worked out either way, but not to the same state.** A grid with a cloth modifier and no pinning, glowing white against a black world under an orthographic camera, frames 1 to 60, nothing baked and the animation never played. The file was opened twice: once going straight to frame 60, once stepping through every frame to it. Each time the evaluated grid was read and that frame rendered.

```
MODE jump FRAME 60  lowest z = -12.2144  highest z = -8.2118  lowest x = -2.0007  highest x = 2.0017
MODE step FRAME 60  lowest z = -11.7781  highest z = -7.7781  lowest x = -2.0000  highest x = 2.0000
PICTURES size=(160, 160) largest channel difference=1.000000 channels differing=741 of 102400
DIFFERENT PICTURE
```

So the cloth does fall whichever way the frame is reached — it is not left at its start — but the two states are not the same. Nothing was printed about a cache in either run.

**A physics disk cache that has been deleted is used silently until it is not there, and then the render simulates instead and says nothing.** The same file with Disk Cache ticked, baked with `bpy.ops.ptcache.bake_all(bake=True)` — which made a folder `blendcache_baked` beside it holding 60 `.bphys` files — and saved. Frame 60 was rendered; then the whole folder was deleted and frame 60 rendered again.

```
--- with the cache present ---
is_baked True | is_outdated False | is_frame_skip False | use_disk_cache True | info '60 frames on disk.'
EXIT 0
--- delete the cache folder ---
is_baked True | is_outdated False | is_frame_skip False | use_disk_cache True | info '60 frames on disk.'
EXIT 0
lines naming cache or bake: 1
--- the two pictures ---
PICTURES size=(160, 160) largest channel difference=1.000000 channels differing=4899 of 102400
DIFFERENT PICTURE
--- and against the no-bake jump render kept earlier ---
PICTURES size=(160, 160) largest channel difference=1.000000 channels differing=4899 of 102400
DIFFERENT PICTURE
```

Three things in that. The picture changes. Nothing is said — the one line naming "cache or bake" is the `Read blend` line, because the file is called `baked.blend` — and the exit status is 0. And the picture with the cache gone is **exactly** the picture the same file gives with no bake at all, so the render silently simulated instead. Note also that `is_baked` still reads `True` and `info` still reads `60 frames on disk.` with the folder deleted, so the file's own flags are no evidence that a cache exists on the machine doing the rendering.

**An external point cache pointing at an empty folder does say so, in a place a machine can read.** The same file with the point cache's External ticked, its path pointed at an empty folder and its name set:

```
EXIT 0
use_external True | filepath /root/dependencies/work/q18/4.5/emptycache | name 'SmoozyCache'
is_baked False | is_outdated False | info 'No valid data to read!'
after going to frame 60: is_baked False | info 'No valid data to read!'
GRID at frame 60: lowest z = -12.2144  highest z = -8.2118
--- the picture against the no-bake jump render ---
PICTURES size=(160, 160) largest channel difference=0.000000 channels differing=0 of 102400
SAME PICTURE
```

So `PointCache.info` reads `No valid data to read!`, nothing appears on the console, the exit status is 0, and the render simulated.

**An unbaked geometry-nodes simulation zone behaves nothing like the cloth: asked for one frame out of order, it barely advances at all.** A cube carrying a simulation zone whose one float state item grows by 0.005 each frame and drives a Transform Geometry node, so the movement piles up; glowing white against a black world under an orthographic camera; frames 1 to 60; nothing baked. The same two runs as above:

```
MODE jump FRAME 60  lowest z = -1.0000  highest z = 1.0000  lowest x = -9.9850  highest x = -7.9850
MODE step FRAME 60  lowest z = -1.0000  highest z = 1.0000  lowest x = -0.8500  highest x = 1.1500
PICTURES size=(160, 160) largest channel difference=1.000000 channels differing=921 of 102400
DIFFERENT PICTURE
```

The cube starts spanning −10 to −8. Stepping through moved it about 9.1 units; jumping moved it 0.015. Nothing was printed about a bake in either run.

**With the bake on disk, that same out-of-order frame comes out right; with the bake deleted it does not, and nothing is said.** The zone was baked to a folder of its own with `bpy.ops.object.geometry_node_bake_single(...)` after setting the modifier's `bake_target` to `DISK`, the zone's own bake to `DISK` and `ANIMATION`, and a custom frame range of 1 to 60 — all three were needed, since with them left alone the bake reported `{'FINISHED'}` and wrote nothing. Frame 60 was rendered, the bake folder deleted, and frame 60 rendered again:

```
--- frame 60 with the bake present ---
EXIT 0
--- delete the bake folder ---
EXIT 0
(lines naming bake or cache, other than the file name:) 1
--- the two pictures ---
PICTURES size=(160, 160) largest channel difference=1.000000 channels differing=921 of 102400
DIFFERENT PICTURE
--- the baked one against the stepped-through no-bake render ---
PICTURES size=(160, 160) largest channel difference=0.000000 channels differing=0 of 102400
SAME PICTURE
--- the unbaked one against the jumped no-bake render ---
PICTURES size=(160, 160) largest channel difference=0.184314 channels differing=75 of 102400
DIFFERENT PICTURE
```

So the bake is what makes a single out-of-order frame match the stepped-through result exactly, and its loss is silent, with exit status 0.

**A fluid domain's flags are held in the file and do not notice the cache going — and opening the file makes the folder again and writes into it.** A small gas simulation, domain resolution 16, cache directory of its own, frames 1 to 12, baked with `bpy.ops.fluid.bake_all()` and saved:

```
BEFORE BAKE | cache_directory: /root/dependencies/work/q18fluid/4.5/fluidcache | has_cache_baked_any: False | has_cache_baked_data: False
BAKE ALL -> {'FINISHED'}
AFTER BAKE  | cache_directory: /root/dependencies/work/q18fluid/4.5/fluidcache | has_cache_baked_any: True | has_cache_baked_data: True
cache folder: ['config', 'data']
cache folder removed; exists now: False
SAME SESSION, FOLDER GONE | has_cache_baked_any: True | has_cache_baked_data: True
REOPENED with the cache folder gone (folder exists: True )
  has_cache_baked_any: True | has_cache_baked_data: True | has_cache_baked_noise: True | has_cache_baked_mesh: True | has_cache_baked_particles: True
```

and what the reopening had put back on disk:

```
/root/dependencies/work/q18fluid/4.5/fluidcache/config/config_0001.uni
/root/dependencies/work/q18fluid/4.5/fluidcache/data/fluid_data_0001.vdb
```

**An EEVEE light probe volume with no bake does render differently.** A closed room with its faces turned inward so the world cannot light it directly, a sphere inside, a light probe volume covering it, and a strong green world. Rendered unbaked, then with `bpy.ops.object.lightprobe_cache_bake(subset='ALL')`, then after `bpy.ops.object.lightprobe_cache_free(subset='ALL')`:

```
ENGINE BLENDER_EEVEE_NEXT
  unbaked against baked: largest channel difference=0.858824 channels differing=24806 of 57600 — DIFFERENT PICTURE
  baked against freed  : largest channel difference=0.858824 channels differing=24806 of 57600 — DIFFERENT PICTURE
```

Nothing was printed about the missing bake. Two things worth carrying with that. EEVEE **does** run on a machine with no display server at all, after printing `libEGL warning: egl: failed to create dri2 screen` and three `EGL Error (0x3009): EGL_BAD_MATCH` lines. And the engine's own name is not the same in every version: 4.5 offers only `BLENDER_EEVEE_NEXT`, and 5.0, 5.1 and 5.2 offer only `BLENDER_EEVEE`.

**An ocean modifier's bake could not be made on this machine, so nothing about its absence was settled here.** `bpy.ops.object.ocean_bake(modifier="Ocean")` answered `{'FINISHED'}` and wrote nothing, leaving `is_cached` at `False` and the cache folder empty:

```
ocean_bake signature: ['rna_type', 'modifier', 'free']
is_cached False use_foam False
BAKE -> {'FINISHED'} is_cached False
folder: False
```

Since the bake never took, the pictures before and after deleting the folder were bound to match, and they did — a result that carries nothing, and is therefore not reported as a finding. It was tried again on the Creator's Mac, where the bake files **can** be written; the question still could not be answered, and the Mac block below says why.

**This version has no texture cache to test.** `scene.render.use_texture_cache` does not exist in it, nor `use_auto_generate_texture_cache`, nor the `texture_cache_directory` preference, and `generate_texture_cache` is not in `dir(bpy.ops.render)`. The same is true of 5.0 and 5.1; 5.2 has all four. So the texture cache is a real difference between the versions and not a gap in one version's documentation.


**This version is the one that wrote a cache the later ones refuse.** A disk point cache baked here with `point_cache.compression = 'LIGHT'` was carried, in its saved file, to 5.0, 5.1 and 5.2. All three printed, once on opening and once while rendering, `Warning: Cloth Cache in object Grid can not be read because it uses an outdated compression method. You need to delete the caches and re-bake.`, still exited 0, still read `is_baked True` and `info '60 frames on disk.'`, and rendered a different picture from the one this version renders with that same cache. `PointCache.compression` exists here with the choices `NO`, `LIGHT` and `HEAVY`, and does not exist in 5.0, 5.1 or 5.2 at all.

**Observed — the Mac half of this round's hand tests, run 2026-08-07 on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card.** Blender 4.5.12 LTS was started from a terminal, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and this Blender answered `RESOURCES …/depwork/res-4.5` and `VERSION 4.5.12 LTS`. The application used was `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`. Every render used Cycles on the processor. **Other topics were running Blender on this same Mac throughout, so nothing was timed and no memory was measured; there is no timing or memory reading anywhere below.** **This is an addition to a document written earlier in this round; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

In the machine's own words quoted below, the working folder's long path is written `…/depwork`; nothing else in any quotation is altered.

Pictures were compared as pictures, never as files. Before that was leaned on, two renders of the same frame of the same file were compared on this Mac and gave `largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE`, so any difference reported below is a real difference.

**Everything the Linux half found about an absent cache or bake holds on macOS too.** Taken in order:

*An unbaked physics simulation is worked out either way, but not to the same state.* A grid standing up with a cloth modifier and no pinning, glowing white against a black world under an orthographic camera, frames 1 to 60, nothing baked and the animation never played. The file was opened twice: once going straight to frame 60, once stepping through every frame to it.

```
MODE jump FRAME 60  lowest z = -12.2137  highest z = -8.2122  lowest x = -2.0005  highest x = 2.0010
MODE step FRAME 60  lowest z = -11.7781  highest z = -7.7781  lowest x = -2.0000  highest x = 2.0000
PICTURES size=(160, 160) largest channel difference=0.772549 channels differing=861 of 102400
DIFFERENT PICTURE
```

Nothing was printed about a cache in either run. Note how close these numbers are to the Linux ones for the same test — the stepped-through figures are the same to four decimal places, and the jumped ones differ in the fourth.

*A physics disk cache that has been deleted is used silently until it is not there, and then the render simulates instead and says nothing.* The same file with Disk Cache ticked, its cache range 1 to 60, baked with `bpy.ops.ptcache.bake_all(bake=True)` — which made a folder `blendcache_baked` beside it holding 60 files — and saved. Frame 60 was rendered; the whole folder was then deleted and frame 60 rendered again.

```
CACHE FOLDER blendcache_baked holds 60 files
--- with the cache present ---
is_baked True | is_outdated False | is_frame_skip False | use_disk_cache True | info '60 frames on disk.'
EXIT 0
--- delete the cache folder ---
is_baked True | is_outdated False | is_frame_skip False | use_disk_cache True | info '0 frames on disk.'
is_baked True | is_outdated False | is_frame_skip False | use_disk_cache True | info '0 frames on disk.'
EXIT 0
lines naming cache or bake: 2   (both of them the file's own name, baked.blend and nocache_0060.png)
--- the two pictures ---
PICTURES size=(160, 160) largest channel difference=0.772549 channels differing=4899 of 102400
DIFFERENT PICTURE
--- and against the no-bake jump render kept earlier ---
PICTURES size=(160, 160) largest channel difference=0.772549 channels differing=4896 of 102400
DIFFERENT PICTURE
--- the baked one against the stepped-through no-bake render ---
PICTURES size=(160, 160) largest channel difference=0.000000 channels differing=0 of 102400
SAME PICTURE
```

Three things in that, and one of them is a **disagreement with the Linux result**. The picture changes; nothing is said, the only lines naming a cache or a bake being the file's own name; and the exit status is 0. That much both machines agree on. **But `PointCache.info` does not read the same on the two machines once the folder is gone.** On this Mac it reads `'0 frames on disk.'`; on the rented Linux machine, with the same 60-frame cache and the same steps, it read `'60 frames on disk.'`. `is_baked` still reads `True` on both. So on macOS there is one reading a caller outside Blender can use to notice the loss, and on Linux there is not — and a program that trusted `info` would be right on one machine and wrong on the other.

Two further comparisons were taken here that the Linux run did not report, and they are what shows the cache was really being used: **with the cache present the render is pixel for pixel the render the same file gives when every frame is stepped through** (`channels differing=0 of 102400 — SAME PICTURE`), and with the cache gone it is not. The cache-gone render is also **not** identical to the jumped-to no-bake render — 4896 of 102400 channels differ — so it is close to it without being it.

*An external point cache pointing at an empty folder does say so, in a place a machine can read.* The same file with the point cache's External ticked, its path pointed at an empty folder and its name set:

```
EXIT 0
use_external True | filepath …/depwork/q18/4.5/emptycache | name 'SmoozyCache'
is_baked False | is_outdated False | info 'No valid data to read!'
after going to frame 60: is_baked False | info 'No valid data to read!'
GRID at frame 60: lowest z = -12.2137  highest z = -8.2122
--- the picture against the no-bake jump render ---
PICTURES size=(160, 160) largest channel difference=0.000000 channels differing=0 of 102400
SAME PICTURE
```

`PointCache.info` reads `No valid data to read!`, nothing appears on the console, the exit status is 0, and the render simulated — its picture is pixel for pixel the jumped-to no-bake render.

*An unbaked geometry-nodes simulation zone behaves nothing like the cloth: asked for one frame out of order, it barely advances at all.*

```
MODE jump FRAME 60  lowest z = -1.0000  highest z = 1.0000  lowest x = -9.9850  highest x = -7.9850
MODE step FRAME 60  lowest z = -1.0000  highest z = 1.0000  lowest x = -0.8500  highest x = 1.1500
PICTURES size=(160, 160) largest channel difference=0.772549 channels differing=921 of 102400
DIFFERENT PICTURE
```

The cube starts spanning −10 to −8. Stepping through moved it about 9.1 units; jumping moved it 0.015. These are the Linux numbers exactly, to four decimal places. Nothing was printed about a bake in either run.

*With the bake on disk, that same out-of-order frame comes out right; with the bake deleted it does not, and nothing is said.*

```
BAKE id <the zone's own> target DISK mode ANIMATION range 1 60
BAKE SINGLE -> {'FINISHED'}
BAKE FOLDER <D>/gnbake/<bake id>/meta 60 files
BAKE FOLDER <D>/gnbake/<bake id>/blobs 60 files
--- frame 60 with the bake present ---
EXIT 0
--- delete the bake folder ---
EXIT 0
(lines naming bake or cache, other than the file name:) 2
--- the two pictures ---
PICTURES size=(160, 160) largest channel difference=0.772549 channels differing=921 of 102400
DIFFERENT PICTURE
--- the baked one against the stepped-through no-bake render ---
PICTURES size=(160, 160) largest channel difference=0.000000 channels differing=0 of 102400
SAME PICTURE
--- the unbaked one against the jumped no-bake render ---
PICTURES size=(160, 160) largest channel difference=0.113725 channels differing=96 of 102400
DIFFERENT PICTURE
```

*A fluid domain's flags are held in the file and do not notice the cache going — and opening the file makes the folder again and writes into it.*

```
BEFORE BAKE  | cache_directory: //fluidcache | has_cache_baked_any: False | has_cache_baked_data: False
BAKE ALL -> {'FINISHED'}
AFTER BAKE   | cache_directory: //fluidcache | has_cache_baked_any: True | has_cache_baked_data: True
cache folder: ['config', 'data']
cache folder removed; exists now: False
SAME SESSION, FOLDER GONE | cache_directory: //fluidcache | has_cache_baked_any: True | has_cache_baked_data: True
REOPENED with the cache folder gone (folder exists: True )
  has_cache_baked_any: True | has_cache_baked_data: True | has_cache_baked_noise: True | has_cache_baked_mesh: True | has_cache_baked_particles: True
```

and what the reopening had put back on disk:

```
…/depwork/q18fluid/4.5/fluidcache/config/config_0001.uni
…/depwork/q18fluid/4.5/fluidcache/data/fluid_data_0001.vdb
```

*An EEVEE light probe volume with no bake does render differently, and freeing the bake puts the render back exactly.*

```
ENGINE BLENDER_EEVEE_NEXT | EEVEE names offered: ['BLENDER_EEVEE_NEXT']
  unbaked against baked: largest channel difference=0.866667 channels differing=37041 of 57600 — DIFFERENT PICTURE
  baked against freed  : largest channel difference=0.866667 channels differing=37041 of 57600 — DIFFERENT PICTURE
  unbaked against freed: largest channel difference=0.000000 channels differing=0 of 57600 — SAME PICTURE
```

Nothing was printed about the missing bake. **On this Mac EEVEE printed no warning of any kind before working** — the `libEGL` and `EGL_BAD_MATCH` lines the Linux machine printed are a thing of a machine with no display server, not of EEVEE.

*This version is the one that wrote a cache the later ones refuse.* A disk point cache baked here with `point_cache.compression = 'LIGHT'` was carried, in its saved file, to 5.0, 5.1 and 5.2 on this same Mac. All three printed the same sentence, once on opening and once while rendering, still exited 0, still read `is_baked True` and `info '60 frames on disk.'`, and rendered a different picture from the one this version renders with that same cache:

```
Warning: Cloth Cache in object Grid can not be read because it uses an outdated compression method. You need to delete the caches and re-bake.
PICTURES size=(160, 160) largest channel difference=0.772549 channels differing=4899 of 102400
DIFFERENT PICTURE
```

That is word for word what the Linux machine printed. `PointCache.compression` exists here with the choices `NO`, `LIGHT` and `HEAVY`, and does not exist in 5.0, 5.1 or 5.2 at all.


### What the window could be asked, and what it could not

The Creator's Mac has a screen, so this is the machine where the drawing questions can be reached. The same limit described under question 17 applies: the window is created and drawn, but its event loop never runs when Blender is started this way from a terminal, so anything needing an event could not be reached.

**The Timeline does show the loss, by drawing nothing.** Entry 13's file was rebuilt with the object carrying the cloth cache selected and active, the Timeline's cache-drawing options turned on, and the current frame set to 30; it was baked to disk, opened with a window and photographed with Blender's own screenshot command; then the `blendcache_` folder was deleted and it was opened and photographed again. Reading the strip of the picture where the cache bar is drawn:

```
with the cache present  average colour r=0.157 g=0.164 b=0.252
with the folder deleted average colour r=0.149 g=0.156 b=0.175
the two strips compared as pictures: largest channel difference=0.498039 channels differing=25024 of 112000 — DIFFERENT PICTURE
```

With the cache there, a solid blue bar runs under the frame numbers from frame 1 to frame 60. With the folder gone **the bar is not drawn at all** — not faint, not striped, absent, leaving only the blue playhead marker. So the interface does show the loss, and shows it by omission.

**An ocean modifier's bake can now be made — and it still does not settle what a render does when it is gone.** This is the test that could not be set up on Linux at all, and it was carried further here without reaching an answer. Called the ordinary way, the operator does on this Mac exactly what it did on Linux — it reports success and writes nothing:

```
ocean_bake(modifier='Ocean') -> {'FINISHED'}
AFTER exec BAKE   is_cached: False | files: 0
```

Seven set-ups were tried this way and all seven did the same: the plain one, with the cache folder made first, with a trailing slash on the path, with foam on, at resolution 16 over five frames, with the file saved first, and with a relative path. Called the other way, through the path the interface itself would use, **it does write the bake**:

```
ocean_bake('INVOKE_DEFAULT', ...) -> {'FINISHED'}
AFTER BAKE       is_cached: False | folder: ['disp_0001.exr', 'disp_0002.exr', 'disp_0003.exr', 'disp_0004.exr', '…'] | files: 20
```

Twenty files, one per frame of the baked range. **But `is_cached` never turns True** — not straight after the bake, and not after the file is saved and opened again:

```
straight after that bake: is_cached False | files 20
after saving and opening again: is_cached False | files 20
opened again with the folder gone: is_cached False
```

and the render does not change:

```
  as opened against reopened-with-files: channels differing=0 of 78400 — SAME PICTURE
  reopened-with-files against reopened-no-files: channels differing=0 of 78400 — SAME PICTURE
```

The two pictures being the same is exactly what would happen if the bake were never being read, so **this still says nothing about what a render does when an ocean bake is gone**, and it must not be read as saying the loss makes no difference. What it does settle is the reason: it is not the want of a window that stops the bake being used, because the files can be written with a window there and the modifier still reports itself as simulating.


**Both standing lines, brought up to date by these tests.**

*Driven from outside.* **Settled.** Every reading in the Linux block was taken with Blender running with no window at all, on a machine with no display server, and every behaviour described holds there — including EEVEE's, which was expected not to run at all and does. The one thing that is **not** available from outside is any announcement: a missing physics cache, a missing geometry-nodes bake and an unbaked light probe are all silent, with exit status 0, and the only things a caller outside Blender can read are `PointCache.info` and the warning about an outdated cache compression. **What the window adds has since been reached on the Creator's Mac**: the Timeline does show a lost cache, by drawing no cache bar at all where a solid one stood. So the loss is visible to a person and invisible to a program on Linux; on macOS a program can also read it off `PointCache.info`, which there drops to `0 frames on disk.`

*Operating system.* **Now settled on two operating systems, Linux and macOS, and each machine is its own result.** Every test above was run on the rented Linux machine on 2026-08-07 and again on the Creator's Mac on 2026-08-07, and the two agree everywhere except where the Mac block below says plainly that they do not. Nothing here is folded into one sentence across the two machines. **Windows is untried: no Windows machine has run it.**

*Still not settled, on either machine:* what a render does when an **ocean modifier's** baked files are gone. On the Creator's Mac the bake **can** be made — twenty `disp_####.exr` files, when the operator is called the way the interface calls it — but `is_cached` never turns True, even after the file is saved and opened again, and the render is the same picture with the files there and with them gone. Since the modifier never reports itself as using the bake, those two pictures matching says nothing about the loss. The detail is in the Mac block below.



## What a running Blender still has to settle

This list has been pruned twice. Everything that could be reached from a terminal was run on 2026-08-07 on the rented Linux machine, and everything that needed a screen was run the same day on the Creator's Mac; all of it is written into the two answers above. What is left is only what is still open.

- **What the tooltip on a red path field lists.** The field itself was reached on the Creator's Mac and is drawn red; the tooltip was not, because reading it means hovering, and a Blender window started from a terminal on that machine never runs its event loop, so it can be drawn and photographed but never given an event.
- **What a render does when an ocean modifier's baked files are gone.** Still open on both machines. On Linux the bake could not be written at all. On the Mac it can be written — twenty `disp_####.exr` files, through the operator's invoke path — but `is_cached` stays False even after the file is saved and reopened, and the render is the same picture with the files there and gone, so nothing about the loss can be read from it.
- **Whether the placeholder arrangement holds over a network file system, and what two machines do when they claim the same frame at the same instant.** This project has one Linux machine and one Mac with no shared network folder between them, so it cannot be reached at all.
- **Whether two Blenders rendering into one folder divide the frames on macOS as well.** That test needs two Blenders running at once, and only one may run at a time on the Creator's Mac; the Linux result stands alone.
- **Anything at all about Windows.** No Windows machine has run it.

---

2026-08-06 — Opus 5 (Claude Code) — Dependencies, Blender 4.5
