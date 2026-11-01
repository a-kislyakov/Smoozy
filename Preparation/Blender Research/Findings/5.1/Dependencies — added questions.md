# Dependencies — added questions (Blender 5.1)

**Sources consulted, and when.** Blender's 5.1 manual (including its pictures), Blender's 5.1 Python API documentation, Blender's 5.1 release notes and Cycles' own 5.1 files, all as supplied in `Blender Sources/` and all taken on **2026-07-30** — that is my date read for every one of them. I also searched Blender's requirements page as supplied, taken on **2026-08-01**; it does not name 5.1 anywhere, so it cannot speak for this version, and nothing in it touched these two questions.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: questions 17 and 18 only.

---

## 17. What use does a `.blend` file make of the places it records for writing, besides writing to them?

**Reading of the question.** I take "the places it records for writing" to mean the file paths and directories stored inside a `.blend` whose purpose is to receive files Blender produces: the render output path, the compositor File Output node's directories, physics cache paths, geometry-nodes bake directories, the ocean modifier's cache path, proxy directories, and the dynamic paint image output directory. The question was not ambiguous to me in any serious way, but a narrower reading is possible — "the output path" alone, meaning only the render output — and if that narrower reading is meant, only sections A6, B7, B8, C10 and D12–D13 below apply.

**LABEL: documented** for each individual use listed below (each carries its own quote); **inferred** where marked.

The short answer: quite a lot. Across 5.1 the same recorded write location is also read back from, checked before writing, created if missing, used to claim work between machines, resolved and reported without anything being written at all, and used as a shared identity when one file links another.

### A. The place is read back from

**A1. Ocean modifier cache path.** Once baked, the recorded folder replaces the simulator entirely.

> "Rather than simulating the ocean data live, it can be baked to files in a given directory. When a simulation is baked, the simulator engine is completely bypassed, and all information for the modifier or texture is retrieved from the baked files."

> "Upon loading the data from these baked files, when a frame of the bake sequence is read, it is cached in memory. This means that accessing loaded frames subsequent times is fast, not incurring the overhead of drive access."

Manual, Ocean Modifier — https://docs.blender.org/manual/en/5.1/modeling/modifiers/physics/ocean.html — read 2026-07-30.

**A2. Physics point cache path.** The recorded path is explicitly a read path as well.

> "External
>
> Allows you to read the cache from a drive using a user-specified file path."

Manual, Baking Physics Simulations — https://docs.blender.org/manual/en/5.1/physics/baking.html — read 2026-07-30.

The API says the same of the flag and the path:

> "filepath
>
> Cache file path (default "", never None, blend relative // prefix supported)"

> "use_external
>
> Read cache from an external location (default False)"

Python API, `bpy.types.PointCache` — https://docs.blender.org/api/5.1/bpy.types.PointCache.html — read 2026-07-30.

**A3. Fluid cache directory.** There is a setting whose only effect is on reading, not writing.

> "Offset
>
> Frame offset that is used when loading the simulation from the cache. It is not considered when baking the simulation, only when loading it."

Manual, Cache (fluid domain) — https://docs.blender.org/manual/en/5.1/physics/fluid/type/domain/cache.html — read 2026-07-30. The same words appear on `cache_frame_offset` in the Python API, `bpy.types.FluidDomainSettings` — https://docs.blender.org/api/5.1/bpy.types.FluidDomainSettings.html — read 2026-07-30.

Resuming is tied to what was written into the place:

> "Resumable
>
> Extra data will be saved so that you can resumed baking after pausing."

> "It is not possible to pause or resume a Bake All process as only the most essential cache files are stored on drive."

Same page and date. (The wording "you can resumed" is the manual's own.) **inferred**, step written out: the quotes say extra data is saved so that baking can resume, and that resuming is impossible where only the most essential files are on the drive. That resuming therefore reads those saved files back is a step the reader has to supply; the manual does not say it.

**A4. Geometry-nodes bake directory.** Loading is named alongside saving.

> "The Bake node allows saving and loading intermediate geometries."

Manual, Bake Node — https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/geometry/operations/bake.html — read 2026-07-30.

> "Baking allows saving and loading intermediate geometries. Baking parts of the node tree can be used for better performance."

Manual, Baking (geometry nodes) — https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/baking.html — read 2026-07-30.

One recorded directory serves two different producers:

> "Bake Path
>
> Location on disk where the baked data is stored. Note, this path is also used for simulation zones."

Manual, Bake Node, same address and date.

**A5. Proxy directories.** A recorded proxy directory can be pointed at proxies that already exist, rather than at ones made now.

> "File
>
> Allows you to use preexisting proxies."

Manual, Proxy (Video Sequencer) — https://docs.blender.org/manual/en/5.1/editors/video_sequencer/sequencer/sidebar/proxy.html — read 2026-07-30.

And the written proxies are then read in place of the original footage:

> "you can generate one or more proxies, which are copies of the original footage stored at a lower resolution and/or quality. These proxies can then be used as a less resource-heavy stand-in while working on the scene."

> "Proxy Custom Directory
>
> By default, proxies are stored to a BL_proxy subfolder next to the original file. Use this option to specify a different location."

Manual, Sidebar Region (Movie Clip editor) — https://docs.blender.org/manual/en/5.1/editors/clip/sidebar.html — read 2026-07-30.

**A6. The render output path is read back by the animation player.**

> "Launching the animation player opens a new window, playing back images or a video located at the render output of the current scene."

Manual, Animation Player — https://docs.blender.org/manual/en/5.1/render/output/animation_player.html — read 2026-07-30.

> "Rendered image sequences can be played back in the Animation Player."

Manual, Introduction (Render Output) — https://docs.blender.org/manual/en/5.1/render/output/introduction.html — read 2026-07-30.

The operator that does this carries no path argument of its own:

> "bpy.ops.render.play_rendered_anim()
>
> Play back rendered frames/movies using an external player"

Python API, `bpy.ops.render` — https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-07-30.

### B. The place is inspected before writing, and what is found there changes what happens

**B7. Existing frames in the output directory are treated as already done.** This is the clearest non-write use of a recorded write location.

> "Just disable the Overwrite option to start rendering where you left off."

and, immediately above it:

> "since the frames you have already rendered will still be in the output directory."

Manual, Rendering Animations — https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

The same page uses it a second way, to fill in gaps:

> "Using an image format for output, you can use the Frame Step option to render every N'th frame. Then disable Overwrite and re-render with Frame Step set to 1."

Same page and date.

The setting itself:

> "Image Sequence – Overwrite Image Multi-Layer EXR
>
> Overwrite existing files when rendering."

Manual, Output — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30. (That heading is repeated verbatim over the next entry on the page, which is the Placeholders setting; the manual's own labelling there is faulty. The picture on the same page settles the names — see below.)

> "use_overwrite
>
> Overwrite existing files while rendering (default True)"

Python API, `bpy.types.RenderSettings` — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

**B8. Empty files are written into the place, and the place is then shared between machines.** The placeholder file is documented as empty, and the same page documents an arrangement in which several machines render into one directory.

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

Python API, `bpy.types.RenderSettings`, same address and date.

> "Hint
>
> Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Manual, Output — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

**inferred**, step written out: the hint puts placeholders and Overwrite-off together and says the machines then share the work, so a placeholder left by one machine must be what stops another machine taking the same frame. The manual never says that, and never says how an empty placeholder is told apart from a finished frame. See *Not covered* below, which is the same gap.

**Picture, same page, same date** — `_images/render_output_properties_output_panel.png`, captioned "Output panel." I opened it. It shows a path field reading `/tmp/`, and below it four checkboxes in two columns: **Overwrite** (ticked) and **Placeholders** (unticked) on the top row, **File Extensions** (ticked) and **Cache Result** (unticked) below. **documented** — this confirms the two setting names that the text of the page mislabels. What the picture does not show is any behaviour; only the names and default-looking states. No words carry this one — it rests on my reading of the image, so anyone checking it has to open the picture rather than look for a sentence.

**B9. The proxy directory has an overwrite switch of its own.**

> "Overwrite
>
> Whether to overwrite existing proxy files or keep them."

Manual, Proxy (Video Sequencer) — https://docs.blender.org/manual/en/5.1/editors/video_sequencer/sequencer/sidebar/proxy.html — read 2026-07-30.

### C. The place is created if it is not there

**C10. Compositor File Output node.**

> "Any missing directories are created automatically during output."

Manual, File Output Node — https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — read 2026-07-30. On the same page, the recorded name is also read as a folder structure: "If an input name contains a slash character (/ or \\), the path is interpreted as a directory hierarchy."

**C11. Ocean modifier.**

> "If the cache path folder does not exist, it will be created."

Manual, Ocean Modifier — https://docs.blender.org/manual/en/5.1/modeling/modifiers/physics/ocean.html — read 2026-07-30.

### D. The place is resolved, checked and reported without anything being written

**D12. The final filename for a frame can be asked for as a value in its own right.**

> "frame_path(*, frame=-2147483648, preview=False, view='')
>
> Return the absolute path to the filename to be written for a given frame"
>
> "Returns:
>
> File Path, The resulting filepath from the scenes render settings (never None)"

Python API, `bpy.types.RenderSettings` — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30. The quoted words describe a call that hands back a path. They do not say whether anything is written when it is called, so that part is not established here.

**D13. The recorded path is a small template language that is evaluated at the moment of use, and can be wrong on its own terms.**

> "Path templates substitute template expressions (written {abcd}, where "abcd" is a variable name) in a filepath when the path is used."

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."

> "Paths that support templates can have template errors, which prevent the path from being processed."

> "When there are template errors in a path, the path field will be highlighted red in the UI"

> "Hovering over the path field will pop up a tooltip that contains a list of the template errors encountered in that path"

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

Manual, File Paths — https://docs.blender.org/manual/en/5.1/files/file_paths.html — read 2026-07-30.

So a recorded write location is also a thing that is parsed, validated, drawn in red, explained in a tooltip, and made into a reason to refuse to render. Variables named on that page include `blend_name`, `blend_dir`, `blend_name_lib`, `blend_dir_lib`, `fps`, `resolution_x` / `resolution_y`, `scene_name`, `camera_name` and `node_name` — that is a list off the page, not a quote. **inferred**, step written out: variables carrying those names would have to be filled from the scene and the camera, so the recorded path draws on more than the disk. The page names the variables; it does not say where their values come from. The API marks the two properties accordingly: `bpy.types.RenderSettings.filepath` is "Directory/name to save animations, # characters define the position and padding of frame numbers (default "", never None, blend relative // prefix supported, Supports template expressions)", and on `bpy.types.CompositorNodeOutputFile` the directory is "The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)" (https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html, read 2026-07-30).

**D14. Cache and bake locations take part in Blender's general path bookkeeping.** Two 5.1.1 fixes say so by name:

> "GeoNodes simulation cache files not reported via foreach_path callback. (blender/blender#155953)"

> "Correct path for foreach_path callback of object's particle systems. (blender/blender@6a713d7797)"

Release notes, Blender 5.1: Corrective Releases — https://developer.blender.org/docs/release_notes/5.1/corrective_releases/ — read 2026-07-30. **documented** that these locations are meant to be enumerated by that path callback in 5.1; **inferred**, and I flag it as a step the reader must supply, that the callback is what feeds the file-wide path operations the manual describes ("Make Paths Relative", "Make Paths Absolute", "Report Missing Files", "Find Missing Files" — Manual, Topbar, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html, read 2026-07-30). The release note does not say what `foreach_path` is used for, and I did not find `foreach_path` anywhere in the 5.1 API documentation.

### E. The place is an identity, shared or matched

**E15. A cache's recorded name decides the filenames, and the object's name stands in when there is none.**

> "Each cache can have a name. Double-click the cache entry to give it a name. If this name is given, any disk cache will be stored in files starting with that name. For example, a cache named MyCache will be stored in MyCache_xxxxxx_yy.bphys."

> "If the cache does not have a name (which is the default), the filename of the cache will depend on the object it is attached to, although this is not immediately obvious. For example, a cache on an object Cube will be stored in 43756265_xxxxxx_yy.bphys, where 43756265 is determined by the object name."

> "Warning
>
> When there are multiple caches on one object, always specify a Cache Name. As described above, the filename of an unnamed cache is determined by the name of the object it is attached to. As a result, an object with multiple physics systems that all have an unnamed cache will cause conflict and can result in losing cache files."

Manual, Baking Physics Simulations — https://docs.blender.org/manual/en/5.1/physics/baking.html — read 2026-07-30.

**E16. Reading an external cache is a name-matching exercise.**

> "The cache name in Caches List and the Index has to exactly match the external cache files name in order to work. The cache files name format is name_frame_index.bphys."

Same page and date.

**E17. The recorded location can be made a shared location when the file is linked into another file.**

> "Disk Cache
>
> The cache of a baked simulation will be stored inside the blend-file when you save it. When Disk Cache is checked, Blender will save the cache separately to the drive in a folder named blendcache_[filename] alongside the blend-file. (The blend-file must be saved first.)"

> "Use Library Path
>
> Share the disk cache when the physics object is linked into another blend-file. When this option is enabled, linked versions of the object will reference the same disk cache. Otherwise linked versions of the object will use independent caches."

Same page and date. The API puts it as a choice about which file's path wins:

> "use_library_path
>
> Use this file's path for the disk cache when library linked into another file (for local bakes per scene file, disable this option) (default True)"

Python API, `bpy.types.PointCache` — https://docs.blender.org/api/5.1/bpy.types.PointCache.html — read 2026-07-30.

Also documented on the same manual page: "When using Library Overrides, data-blocks only support Disk Cache storage."

### One near miss, stated so it is not mistaken for a finding

The render cache — "Saving – Cache Result … Saves the rendered view layers and their passes to a multi-layer OpenEXR image. The Compositor can then use this file to improve performance, especially for heavy compositing." and "You can also load it back into the Image Editor's Render Result, even after closing and reopening Blender" (Manual, Output, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30) — is a genuine read-back use, and "Open Cached Render" reads it (Manual, Editing, https://docs.blender.org/manual/en/5.1/editors/image/editing.html, read 2026-07-30: "Find the render cache file for the current scene and load it into the Render Result. This way, you can restore the last render from a previous Blender session and continue working in the Compositor without having to render the scene again."). But the *location* is not recorded in the `.blend`: "The image is stored in the Render Cache folder as specified in the File Paths Preferences." Only the switch (`use_render_cache`) is in the file. Same for the temporary directory, which the manual says holds "render layers, physics cache, copy-paste buffer and crash logs" and comes from Preferences or environment variables, not the file (Manual, Blender's Directory Layout, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html, read 2026-07-30).

### Not covered

- **How** the existence check behind Overwrite works. Nothing in the sources says whether Blender lists the output directory or tests each expected filename, whether it checks once before the job or per frame, whether it looks at anything beyond the name (size, contents, age), or how an empty placeholder file is distinguished from a finished frame. The Primitive Render Farm hint depends on that distinction and never explains it.
- Whether Overwrite applies to video output as well as image sequences, or only to image sequences. The manual's headings say "Image Sequence – Overwrite" but the same headings are visibly mislabelled on that page, so I will not lean on them.
- Whether the render output path is read anywhere else — for instance to seed a file browser's starting folder, or by the Video Sequencer. I found no statement either way.
- What happens when two machines race on the same placeholder file; the sources describe the arrangement, not its guarantees.
- The dynamic paint image-sequence output directory ("you can define used UV maps and output file saving directory, filenames and image format", Manual, Canvas, https://docs.blender.org/manual/en/5.1/physics/dynamic_paint/canvas.html, read 2026-07-30) is a recorded write place, but I found no documented use of it other than writing.
- Whether the geometry-nodes bake directory is read back automatically at render time, or only when a bake is present and current. The manual says loading happens; it does not say when.

### Outside the interface

**Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** 

Partly documented, partly not.

Documented: path templates work when the path is given on the command line —

> "-o, --render-output <path>
>
> Set the render path and file name. Use // at the start of the path to render relative to the blend-file.
>
> You can use path templating features such as {blend_name} in the path. See Blender's documentation on path templates for more details."

Manual, Command Line Arguments (the page's own header reads "Blender 5.1") — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. The same page warns that order matters, which is a real difference from clicking: "blender --background --render-output /tmp test.blend --render-frame 1 … will not render to /tmp because loading the blend-file overwrites the render output that was set."

Not documented: whether Overwrite and Placeholders behave identically when Blender is started and driven from outside. They are scene properties saved in the file (`bpy.types.RenderSettings`), and nothing in the sources says they are ignored in background mode — but nothing says they are honoured either, and the Primitive Render Farm hint never mentions how the machines are started.

One place it clearly differs: the animation player. On the command line the player is given files explicitly, not the scene's recorded output path —

> "-a <options> <file(s)>
>
> Instead of showing Blender's user interface, this runs Blender as an animation player, to view movies and image sequences rendered in Blender (ignored if -b is set)."

Same page and date. So use A6 above — reading the recorded output path in order to play it — is an interface-side behaviour; **inferred** from the two quotes together (the operator takes no path and opens a window; the command-line player takes files and is ignored under `-b`).

Uses D12 (`frame_path`) and D13 (template errors preventing a render) are the two that plainly do not need a person at the screen: the first is a Python call, and the second is described as making "rendering an animation … fail with an error message". The red field and the tooltip are interface-only by their own description.

### Operating system

**Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** 

The sources say almost nothing. The one operating-system-specific statement I found bearing on a recorded write place is about a cache directory failing to be usable:

> "If you are not allowed to write to the required subdirectory caching will not take place, e.g. if your blend-file path is very long and your operating system has a limit on the path length that is supported."

Manual, Cache (particles) — https://docs.blender.org/manual/en/5.1/physics/particles/emitter/cache.html — read 2026-07-30.

Everything else above is written without reference to any operating system. The requirements page, taken 2026-08-01 (https://www.blender.org/download/requirements/), does not name 5.1 and says nothing about output or cache paths, so it cannot be used here.

### Where I looked, and what I ruled out

Searched the whole of `manual_5.1` and `api_5.1` for: `Placeholder`, `overwrit`, `Overwrite`, `render output`, `output path`, `output directory`, `Play Rendered`, `play_rendered_anim`, `Animation Player`, `external player`, `View Animation`, `frame_path`, `use_placeholder`, `use_overwrite`, `Custom Directory`, `Proxy Custom`, `BL_proxy`, `template expression`, `Open Cached Render`, `Render Cache`, `blendcache`, `bphys`, `Cache Path`, `Bake Path`, `bake_directory`, `cache_directory`, `foreach_path`, `Missing`, `Report Missing Files`. Read in full: the Output, Rendering Animations, Animation Player, Render Output Introduction, File Output Node, File Paths, Baking Physics Simulations, fluid domain Cache, particle Cache, cloth Cache, soft body Cache, dynamic paint Canvas, Ocean Modifier, geometry-nodes Baking, Bake Node, Simulation Nodes, Simulation Zone, Geometry Nodes Modifier, VSE Proxy, Movie Clip Sidebar, Preferences File Paths, Blender's Directory Layout, Topbar and Command Line Arguments pages, plus the `bpy.types.RenderSettings`, `bpy.types.PointCache`, `bpy.types.FluidDomainSettings`, `bpy.types.NodesModifier`, `bpy.types.NodesModifierBake`, `bpy.types.CompositorNodeOutputFile` and `bpy.ops.render` API pages. Opened two pictures: `render_output_properties_output_panel.png` and `physics_baking_multi-cache-interface.png` (the latter shows the Cache panel with File Format "Point Cache", Compression "Lite", a named cache list, "Use Library Path" ticked and "External" unticked — it added nothing beyond the text, so I have not leaned on it). Grepped all of `release_notes_5.1` for `cache`, `bake`, `template`, `filepath`, `file path`, `output`. Grepped all of `cycles_5.1` for `cache`, `filepath`, `directory`, `StringProperty`: Cycles records no write location of its own in 5.1 — the only related setting is `tile_size`, "Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory" (`cycles_5.1/properties.py`, `tile_size` property, around line 1056; **observed**), and that scratch location is not a path the file records. Searched `requirements/requirements_2026-08-01.txt` for `5.1`, `cache`, `bake`, `output` — no matches.

---

**Observed — the hand tests for this round, run 2026-08-07 on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and 118 GB of free disk, and no display server at all.** Blender 5.1 was started from a terminal with no window, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and the Blender answered `RESOURCES /root/dependencies/res-5.1`. Every render used Cycles on the processor, so nothing here depends on the graphics card. The machine was shared with other topics' work throughout, so nothing here is a timing or memory measurement of a clean machine except where it says what it did about that. The full list of tests, with every set-up written out, is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

Pictures were compared as pictures, never as files. Before that was leaned on, two renders of the same frame of the same file were compared and gave `largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE`, so any difference reported below is a real difference.

**A missing output folder is made.** A file whose render output path pointed into a folder that did not exist was rendered with `--background --factory-startup <file> --render-anim`, and again with the path two folders deep into folders that did not exist. Both worked, exit status 0:

```
EXIT STATUS 0
00:00.347  blend            | Read blend: "/root/dependencies/work/q17/5.1/render.blend"
00:00.691  render           | Saved: '/root/dependencies/work/q17/5.1/deep3/deep4/frame_0001.png'
00:00.692  render           | Time: 00:00.33 (Saving: 00:00.13)
FILES:
/root/dependencies/work/q17/5.1/deep3
/root/dependencies/work/q17/5.1/deep3/deep4
/root/dependencies/work/q17/5.1/deep3/deep4/frame_0001.png
```

**The output place is looked at, and Blender says so by name.** Five frames were rendered, frames 2 and 4 deleted, `use_overwrite` set to False, and the animation rendered again with no window:

```
00:00.281  render           | Skipping existing frame "/root/dependencies/work/q17/5.1/ow/frame_0001.png"
00:00.543  render           | Saved: '/root/dependencies/work/q17/5.1/ow/frame_0002.png'
00:00.543  render           | Skipping existing frame "/root/dependencies/work/q17/5.1/ow/frame_0003.png"
00:00.683  render           | Saved: '/root/dependencies/work/q17/5.1/ow/frame_0004.png'
00:00.683  render           | Skipping existing frame "/root/dependencies/work/q17/5.1/ow/frame_0005.png"
```

Only the two deleted files carried a new modification time afterwards; the other three kept theirs.

**An empty file counts as a frame already done.** With `use_overwrite` off and `use_placeholder` on, five zero-length files were made at the five names `frame_path()` gave, and the animation rendered:

```
EXIT 0
00:00.274  render           | Skipping existing frame "/root/dependencies/work/q17/5.1/ph/frame_0001.png"
00:00.274  render           | Skipping existing frame "/root/dependencies/work/q17/5.1/ph/frame_0002.png"
00:00.274  render           | Skipping existing frame "/root/dependencies/work/q17/5.1/ph/frame_0003.png"
00:00.274  render           | Skipping existing frame "/root/dependencies/work/q17/5.1/ph/frame_0004.png"
00:00.274  render           | Skipping existing frame "/root/dependencies/work/q17/5.1/ph/frame_0005.png"
00:00.274  reports          | No frames rendered, skipped to not overwrite
```

All five were still zero length afterwards. Note the exit status: nothing was rendered and the run still reported success.

**Two Blenders writing into one folder divide the frames between them.** A slower scene (400 samples, 200 by 200, two threads, eight frames) was rendered by two Blenders started five seconds apart into one empty folder, Overwrite off and Placeholders on. Four seconds into the first run the folder held one file, of zero length, for the frame being rendered. Then:

```
--- what the first Blender reported ---
00:08.010  render           | Saved: 'frame_0001.png'
00:08.010  render           | Skipping existing frame "frame_0002.png"
00:14.219  render           | Saved: 'frame_0003.png'
00:14.219  render           | Skipping existing frame "frame_0004.png"
00:18.627  render           | Saved: 'frame_0005.png'
00:18.627  render           | Skipping existing frame "frame_0006.png"
00:23.020  render           | Saved: 'frame_0007.png'
00:23.020  render           | Skipping existing frame "frame_0008.png"
--- what the second Blender reported ---
00:00.307  render           | Skipping existing frame "frame_0001.png"
00:08.079  render           | Saved: 'frame_0002.png'
00:08.079  render           | Skipping existing frame "frame_0003.png"
00:13.022  render           | Saved: 'frame_0004.png'
00:13.022  render           | Skipping existing frame "frame_0005.png"
00:17.448  render           | Saved: 'frame_0006.png'
00:17.448  render           | Skipping existing frame "frame_0007.png"
00:21.810  render           | Saved: 'frame_0008.png'
```

with all eight files holding a real image afterwards and none claimed twice. This was two Blenders on one machine writing into an ordinary folder; a folder shared between two machines over a network could not be reached — see the *cannot be run* note at the end.

**A bad path template stops a render started from outside, for a single frame as well as for an animation.** A control path with a sound template rendered normally. The faulty one, `{{blend_name.png` with the closing brace missing, was first put to the Blender itself with no window:

```
=== a GOOD template, as a control ===
PATH SET TO '/root/dependencies/work/q17/5.1/tmpl/{{blend_name}}_'
FRAMEPATH OK '/root/dependencies/work/q17/5.1/tmpl/tmplok_0001.png'
EXIT 0
00:00.593  render           | Saved: '/root/dependencies/work/q17/5.1/tmpl/tmplok_0001.png'
=== the BAD template ===
PATH SET TO '/root/dependencies/work/q17/5.1/tmpl/{{blend_name.png'
FRAMEPATH RAISED RuntimeError Error: Parse errors in path '/root/dependencies/work/q17/5.1/tmpl/{{blend_name.png':
--- single frame ---
EXIT 1
00:00.273  reports          | ERROR Parse errors in path '/root/dependencies/work/q17/5.1/tmpl/{{blend_name.png':
                            | - Invalid or incomplete template expression '{{blend_name.png'.
--- animation ---
EXIT 1
00:00.279  reports          | ERROR Parse errors in path '/root/dependencies/work/q17/5.1/tmpl/{{blend_name.png':
                            | - Invalid or incomplete template expression '{{blend_name.png'.
```

**The recorded path beats one given before the file on the command line.**

```
--- output given BEFORE the file ---
00:00.551  render           | Saved: '/root/dependencies/work/q17/5.1/out/frame_0001.png'
--- output given AFTER the file ---
00:00.520  render           | Saved: '/root/dependencies/work/q17/5.1/ord/after/x_0001.png'
```

**`frame_path()` names the file that appears, for a frame render and for an animation — and not for a still render.** With the output path `shot_{{blend_name}}_##_`:

```
PREDICTED frame 7: /root/dependencies/work/q17/5.1/fp2/shot_fp2_07_.png
00:00.518  render           | Saved: '/root/dependencies/work/q17/5.1/fp2/shot_fp2_07_.png'
```

but through `bpy.ops.render.render(write_still=True)` at frame 7, on the same path, the template was substituted and the `##` padding was not:

```
PREDICTED /root/dependencies/work/q17/5.1/fp/shot_fp_07_.png
ON DISK ['shot_fp_##_.png']
MATCH False
```

**The file-wide path commands reach an ocean cache path and nothing else that is recorded for writing.** One file was built holding six absolute locations — an image loaded from disk, the render output path, a compositor File Output node's directory, a point cache with External on, a geometry-nodes bake directory, and an ocean modifier's cache path:

```
--- as saved ---
  image ReadTex   : /root/dependencies/work/q17paths/5.1/assets/tex.png
  render output   : /root/dependencies/work/q17paths/5.1/renders/frame_
  file output node: /root/dependencies/work/q17paths/5.1/fileout
  point cache     : /root/dependencies/work/q17paths/5.1/ptcache
  gn bake dir     : /root/dependencies/work/q17paths/5.1/gnbake
  ocean cache     : /root/dependencies/work/q17paths/5.1/oceanbake
MAKE PATHS RELATIVE -> {{'FINISHED'}}
--- after Make Paths Relative ---
  image ReadTex   : //assets/tex.png
  render output   : /root/dependencies/work/q17paths/5.1/renders/frame_
  file output node: /root/dependencies/work/q17paths/5.1/fileout
  point cache     : /root/dependencies/work/q17paths/5.1/ptcache
  gn bake dir     : /root/dependencies/work/q17paths/5.1/gnbake
  ocean cache     : //oceanbake
```

Make Paths Absolute put both back. With all six pointed at a folder that does not exist, `bpy.ops.file.report_missing_files()` named two of them:

```
Warning: Path '/root/dependencies/work/q17paths/5.1/gone' not found, from local data-block 'OBGrid'
Warning: Path '/root/dependencies/work/q17paths/5.1/gone/tex.png' not found, from local data-block 'IMReadTex'
BLEND PATHS: ['/root/dependencies/work/q17paths/5.1/gone', '/root/dependencies/work/q17paths/5.1/gone/tex.png', '/root/dependencies/work/q17paths/5.1/assets/tex.png']
```

`OBGrid` carries the ocean modifier and `IMReadTex` is the image. The render output path, the File Output node, the point cache and the geometry-nodes bake directory were named by nothing.

**What `bpy.data.file_path_foreach` visits.** The call's own description was printed first rather than its arguments guessed; it takes a function of three parameters. What was typed before that was read, and what came back, is recorded too:

```
file_path_foreach RAISED TypeError <lambda>() takes 1 positional argument but 3 were given
```

and then, with the right shape:

```
HAS file_path_foreach: True
HAS file_path_map    : True
FOREACH flags=None visited 3:
    Grid '/root/dependencies/work/q17paths/5.1/oceanbake'
    ReadTex '/root/dependencies/work/q17paths/5.1/assets/tex.png'
    Tex '/root/dependencies/work/q17paths/5.1/assets/tex.png'
FOREACH flags={{'EXPAND_CACHES'}} RAISED ValueError: flags: 'EXPAND_CACHES' not found in ('SKIP_LINKED', 'SKIP_PACKED', 'RESOLVE_TOKEN', 'SKIP_WEAK_REFERENCES', 'SKIP_MULTIFILE', 'RELOAD_EDITED')
```

So the walk visits the image and the ocean cache path and none of the other four recorded write places, and this version's flag list holds no `EXPAND_CACHES`. (4.5 has no `file_path_foreach` at all; 5.2 accepts `EXPAND_CACHES` and visits the same three.)

**Two version-dependent shapes had to be asked of the running Blender rather than assumed**, and are recorded because they bear on anything written against this: the compositor tree is reached through `scene.compositing_node_group` in this version, and the File Output node's property is named `directory` here. (In 4.5 they are `scene.node_tree` and `base_path`.)

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
AFTER DELETING FRAME 3 ['dn.blend', 'f_0001.exr', 'f_0002.exr', 'f_0004.exr', 'f_0005.exr']
Error: Frame '/root/dependencies/work/denoise/5.1/f_0003.exr' not found, animation must be complete
```

In this version `OPEN_EXR_MULTILAYER` is not in the file-format list until `image_settings.media_type = 'MULTI_LAYER_IMAGE'` is set first; in 4.5 the format can be chosen straight away.

**Observed — the Mac half of this round's hand tests, run 2026-08-07 on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card.** Blender 5.1.2 was started from a terminal, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and this Blender answered `RESOURCES …/depwork/res-5.1` and `VERSION 5.1.2`. The application used was `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`. Every render used Cycles on the processor. **Other topics were running Blender on this same Mac throughout, so nothing was timed and no memory was measured; there is no timing or memory reading anywhere below.** **This is an addition to a document written earlier in this round; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

In the machine's own words quoted below, the working folder's long path is written `…/depwork`; nothing else in any quotation is altered.

Pictures were compared as pictures, never as files. Before that was leaned on, two renders of the same frame of the same file were compared on this Mac and gave `largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE`, so any difference reported below is a real difference.

**Everything the Linux half found about the output place holds on macOS too.** Taken in order:

*A missing output folder is made.* The same file rendered with `--background --factory-startup <file> --render-anim`, and again with `--render-output <workdir>/deep3/deep4/frame_ --render-frame 1` into two folders that did not exist. Both worked, exit status 0, nothing said about the folder:

```
EXIT STATUS 0
00:00.289  blend            | Read blend: "…/depwork/q17/5.1/render.blend"
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
00:00.458  render           | Saved: '…/depwork/q17/5.1/deep3/deep4/frame_0001.png'
00:00.458  render           | Time: 00:00.16 (Saving: 00:00.05)
Blender quit
FILES:
…/depwork/q17/5.1/deep3
…/depwork/q17/5.1/deep3/deep4
…/depwork/q17/5.1/deep3/deep4/frame_0001.png
```

*The output place is looked at before writing.* Five frames rendered, frames 2 and 4 deleted, `use_overwrite` set to False, the animation rendered again with no window:

```
00:00.724  render           | Skipping existing frame "…/depwork/q17/5.1/ow/frame_0001.png"
00:01.041  render           | Saved: '…/depwork/q17/5.1/ow/frame_0002.png'
00:01.041  render           | Skipping existing frame "…/depwork/q17/5.1/ow/frame_0003.png"
00:01.535  render           | Saved: '…/depwork/q17/5.1/ow/frame_0004.png'
00:01.535  render           | Skipping existing frame "…/depwork/q17/5.1/ow/frame_0005.png"
```

and only the two deleted files carried a new modification time afterwards:

```
before   frame_0001.png 1786065797 8598     after   frame_0001.png 1786065797 8598
         frame_0003.png 1786065797 8598             frame_0002.png 1786065800 8598
         frame_0005.png 1786065797 8598             frame_0003.png 1786065797 8598
                                                    frame_0004.png 1786065801 8598
                                                    frame_0005.png 1786065797 8598
```

*An empty file counts as a frame already done.* With `use_overwrite` off and `use_placeholder` on, five zero-length files were put at the five names `frame_path()` gave:

```
--- five zero-length files placed ---
frame_0001.png 0   frame_0002.png 0   frame_0003.png 0   frame_0004.png 0   frame_0005.png 0
--- render the animation, no window ---
EXIT 0
00:00.320  render           | Skipping existing frame "…/depwork/q17/5.1/ph/frame_0001.png"
00:00.320  render           | Skipping existing frame "…/depwork/q17/5.1/ph/frame_0002.png"
00:00.320  render           | Skipping existing frame "…/depwork/q17/5.1/ph/frame_0003.png"
00:00.320  render           | Skipping existing frame "…/depwork/q17/5.1/ph/frame_0004.png"
00:00.320  render           | Skipping existing frame "…/depwork/q17/5.1/ph/frame_0005.png"
00:00.321  reports          | No frames rendered, skipped to not overwrite
--- sizes after ---
frame_0001.png 0   frame_0002.png 0   frame_0003.png 0   frame_0004.png 0   frame_0005.png 0
```

Nothing was rendered, all five were still zero length, and the run still reported success.

*A bad path template stops a render started from outside, for a single frame as well as for an animation*, with a sound template as the control:

```
=== a GOOD template, as a control ===
PATH SET TO '…/depwork/q17/5.1/tmpl/{blend_name}_'
FRAMEPATH OK '…/depwork/q17/5.1/tmpl/render_0001.png'
EXIT 0
00:00.472  render           | Saved: '…/depwork/q17/5.1/tmpl/tmplok_0001.png'
=== the BAD template ===
PATH SET TO '…/depwork/q17/5.1/tmpl/{blend_name.png'
FRAMEPATH RAISED RuntimeError Error: Parse errors in path '…/depwork/q17/5.1/tmpl/{blend_name.png':
--- single frame ---
EXIT 1
00:00.275  reports          | ERROR Parse errors in path '…/depwork/q17/5.1/tmpl/{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
--- animation ---
EXIT 1
00:00.275  reports          | ERROR Parse errors in path '…/depwork/q17/5.1/tmpl/{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
--- what is in the folder ---
tmplok_0001.png
```

One thing about that control worth recording, because it is a difference in how the test was set up and not a difference in Blender: on this Mac `frame_path` was asked **before** the file had been saved under its new name, so `{blend_name}` still stood for the old name and it answered `render_0001.png`. The file that appeared on disk when the render ran was `tmplok_0001.png`, which is what the template says.

*The recorded path beats one given before the file on the command line.*

```
--- output given BEFORE the file ---
00:00.434  render           | Saved: '…/depwork/q17/5.1/tmplsrc/out/frame_0001.png'
--- output given AFTER the file ---
00:00.433  render           | Saved: '…/depwork/q17/5.1/ord/after/x_0001.png'
--- what landed where ---
…/depwork/q17/5.1/ord/after/x_0001.png
```

*`frame_path()` names the file that appears, for a frame render and for an animation — and not for a still render.* With the output path `shot_{blend_name}_##_`:

```
PATH '…/depwork/q17/5.1/fp2/shot_{blend_name}_##_'
PREDICTED frame 1: …/depwork/q17/5.1/fp2/shot_fp2_01_.png
PREDICTED frame 7: …/depwork/q17/5.1/fp2/shot_fp2_07_.png
--- render frame 7 from the command line ---
Saved: '…/depwork/q17/5.1/fp2/shot_fp2_07_.png'
--- render the animation ---
Saved: '…/depwork/q17/5.1/fp2/shot_fp2_01_.png' … '…/depwork/q17/5.1/fp2/shot_fp2_05_.png'
ON DISK: shot_fp2_01_.png shot_fp2_02_.png shot_fp2_03_.png shot_fp2_04_.png shot_fp2_05_.png shot_fp2_07_.png
```

but through `bpy.ops.render.render(write_still=True)` at frame 7, on the same path:

```
PATH '…/depwork/q17/5.1/fp/shot_{blend_name}_##_'
PREDICTED …/depwork/q17/5.1/fp/shot_render_07_.png
ON DISK ['shot_render_##_.png']
MATCH False
```

*The file-wide path commands reach an ocean cache path and nothing else recorded for writing.* The same six recorded locations were built into one file — an image loaded from disk, the render output path, a compositor File Output node's directory, a point cache with External on, a geometry-nodes bake directory, and an ocean modifier's cache path:

```
TREE ATTR scene.compositing_node_group | FILE OUTPUT NODE PROPERTY directory
--- as saved ---
  image ReadTex   : …/depwork/q17paths/5.1/assets/tex.png
  render output   : …/depwork/q17paths/5.1/renders/frame_
  file output node: …/depwork/q17paths/5.1/fileout
  point cache     : …/depwork/q17paths/5.1/ptcache
  gn bake dir     : …/depwork/q17paths/5.1/gnbake
  ocean cache     : …/depwork/q17paths/5.1/oceanbake
MAKE PATHS RELATIVE -> {'FINISHED'}
--- after Make Paths Relative ---
  image ReadTex   : //assets/tex.png
  render output   : …/depwork/q17paths/5.1/renders/frame_
  file output node: …/depwork/q17paths/5.1/fileout
  point cache     : …/depwork/q17paths/5.1/ptcache
  gn bake dir     : …/depwork/q17paths/5.1/gnbake
  ocean cache     : //oceanbake
MAKE PATHS ABSOLUTE -> {'FINISHED'}
--- after Make Paths Absolute ---
  image ReadTex   : …/depwork/q17paths/5.1/assets/tex.png
  … the other four unchanged …
  ocean cache     : …/depwork/q17paths/5.1/oceanbake
```

With all six pointed at a folder that does not exist, `bpy.ops.file.report_missing_files()` named two of them:

```
Warning: Path '…/depwork/q17paths/5.1/gone' not found, from local data-block 'OBGrid'
Warning: Path '…/depwork/q17paths/5.1/gone/tex.png' not found, from local data-block 'IMReadTex'
REPORT MISSING FILES -> {'FINISHED'}
BLEND PATHS: ['…/depwork/q17paths/5.1/gone', '…/depwork/q17paths/5.1/gone/tex.png', '…/depwork/q17paths/5.1/assets/tex.png']
```

`OBGrid` carries the ocean modifier and `IMReadTex` is the image. The render output path, the File Output node, the point cache and the geometry-nodes bake directory were named by nothing.

One thing about the set-up, recorded because it had to be worked around: on this Mac, **saving the file turned the image path and the ocean cache path into relative ones by itself**, which would have left Make Paths Relative with nothing to do and half the test blind. All six were therefore set back to absolute after the save, and only then were the two commands run. What is quoted above is that second run.

*Cycles' animation denoiser reads the recorded output place and stops on a frame that is not there.* Five frames rendered to multi-layer OpenEXR with denoising data passes, frame 3 deleted, and `bpy.ops.cycles.denoise_animation()` called:

```
MEDIA TYPE SET TO MULTI_LAYER_IMAGE
FILE FORMAT OPEN_EXR_MULTILAYER
RENDERED ['dn.blend', 'f_0001.exr', 'f_0002.exr', 'f_0003.exr', 'f_0004.exr', 'f_0005.exr']
AFTER DELETING FRAME 3 ['dn.blend', 'f_0001.exr', 'f_0002.exr', 'f_0004.exr', 'f_0005.exr']
Error: Frame '…/depwork/denoise/5.1/f_0003.exr' not found, animation must be complete
DENOISE ANIMATION RAISED RuntimeError Error: Frame '…/depwork/denoise/5.1/f_0003.exr' not found, animation must be complete
```

*Which of these settings this version has at all*, asked directly on this machine:

```
RenderSettings.use_texture_cache                none
RenderSettings.use_auto_generate_texture_cache  none
RenderSettings.use_render_cache                 False
RenderSettings.use_overwrite                    True
RenderSettings.use_placeholder                  False
PointCache.compression                          'none'
Preferences render_cache_directory              ''
Preferences texture_cache_directory             'none'
generate_texture_cache in dir(bpy.ops.render)   False
denoise_animation in dir(bpy.ops.cycles)        True
merge_images in dir(bpy.ops.cycles)             True
hasattr(bpy.ops.render, 'smoozy_no_such_operator') -> True
```

**The path walk visits the same three things it visited on Linux.** The function's own description was printed before it was called, rather than guessed at, and it names three arguments — and in this version the description says `:param` where 5.0's says `:arg`:

```
HAS file_path_foreach: True
HAS file_path_map    : True
   :param visit_path_fn: function that takes three parameters: the data-block, a file path, and a placeholder for future use.
file_path_foreach RAISED TypeError <lambda>() takes 1 positional argument but 3 were given
FOREACH flags=None visited 3:
    Grid '//oceanbake'
    ReadTex '//assets/tex.png'
    Tex '//assets/tex.png'
FOREACH flags={'EXPAND_CACHES'} RAISED ValueError: flags: 'EXPAND_CACHES' not found in ('SKIP_LINKED', 'SKIP_PACKED', 'RESOLVE_TOKEN', 'SKIP_WEAK_REFERENCES', 'SKIP_MULTIFILE', 'RELOAD_EDITED')
```

(The one-argument call is kept because it is what was typed first, before the description had been read.) The walk reached the image and the ocean cache path and none of the other four recorded write places.

**This version has no texture cache to test on macOS either.** `scene.render.use_texture_cache` does not exist in it: `this version has a texture cache setting: False`.

### What the window could be asked, and what it could not

The Creator's Mac has a screen, so this is the machine where the drawing questions can be reached. Blender was started there with a window and a `--python` script, and the script did its work as it ran. **One limit shaped all of it and is recorded because it decides what could and could not be asked**: when Blender is started this way from a terminal on this machine, the window is created and drawn — the screenshots below are of a fully drawn Blender — but **its event loop never runs**. A timer registered with `bpy.app.timers.register` never fired once in any attempt, `bpy.ops.wm.redraw_timer` never returned, and `bpy.ops.wm.quit_blender()` never quit, so each script ended the process itself after flushing what Blender had printed. So anything needing an event — hovering over a control, or a redraw after something is changed — could not be reached, and anything the window already shows when the file opens could.

**The path field is drawn red for a bad template.** A file whose render output path reads `//out/{blend_name.png` was saved with its Properties editor already on the Output tab, opened with a window, and photographed with Blender's own `bpy.ops.screen.screenshot`. The same was done with `//out/{blend_name}.png`, a sound template, as the control. Reading the average colour of the strip of the picture holding the path field:

```
the bad template   average colour r=0.306 g=0.167 b=0.167
the good template  average colour r=0.184 g=0.184 b=0.184
```

In the bad one the red is about twice the green and the blue; in the control all three are equal, which is Blender's ordinary grey. The field itself reads `//out/{blend_name.png`.

**The tooltip could not be reached at all**, because reading it means hovering, and hovering is an event. That half of the question stays open, and it is open for the reason above rather than for want of a screen.

**Overwrite-off skips frames inside Blender's own window as well.** Entry 2's folder was rebuilt with frames 2 and 4 deleted and Overwrite off, and the file opened with a window and rendered by a script calling `bpy.ops.render.render(animation=True)`:

```
before
   frame_0001.png 1786068025 8525
   frame_0003.png 1786068025 8525
   frame_0005.png 1786068025 8525
render(animation=True) -> {'FINISHED'}
after
   frame_0001.png 1786068025 8525
   frame_0002.png 1786068026 8533
   frame_0003.png 1786068025 8525
   frame_0004.png 1786068026 8533
   frame_0005.png 1786068025 8525
--- what the window Blender printed ---
00:00.705  render           | Saved: '…/depwork/q17/5.1/wow/frame_0002.png'
00:00.800  render           | Saved: '…/depwork/q17/5.1/wow/frame_0004.png'
```

Only the two missing frames carry a new modification time. **And this version says nothing about it.** With no window it prints `Skipping existing frame` for each of the three; in the window run those lines are not printed at all, though the frames are plainly skipped. The console output was flushed before the process ended, so this is not a lost buffer.

**The Animation Player does play what is at the recorded output place.** With five frames sitting at the recorded place, a script called `bpy.ops.render.play_rendered_anim()` and then listed the running processes:

```
play_rendered_anim() -> {'FINISHED'}
    51778 /Applications/Blender 5.1.2.app/Contents/MacOS/Blender --gpu-backend metal -a -f 24 1.0 -s 1 -e 5 -j 1 -c 4096 …/depwork/q17/5.1/wow/frame_0001.png
```

So the player is a second Blender started with `-a`, and the last thing it is handed is the first frame at the file's own recorded output place. As a control the same was done with the recorded place pointing at an **empty** folder. That control was run once, in Blender 4.5 on this same Mac, and its own words are quoted here; the player was still started at that place and said so:

```
wm_main_playanim_intern: '…/depwork/q17/4.5/emptyout/frame_0001.png' not an image file
```

so the operator reads the recorded place rather than doing something generic.

**The viewport render uses the recorded output place.** With the recorded place an empty folder, a script called `bpy.ops.render.opengl(animation=True)` with the 3D viewport as its context:

```
render output path: '…/depwork/q17/5.1/glout/gl_'
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

**LABEL: not in the sources**, for the question as asked. Nothing in the 5.1 manual, the 5.1 API documentation, the 5.1 release notes or Cycles' 5.1 files states what a render does when a cache directory or bake that the file records is missing — not whether it stops, warns, renders the un-simulated state, silently renders something wrong, or falls back to simulating. I searched hard for this and found silence; the record of that search is at the end of this answer.

What the sources do say is narrower, and I set it out below so the gap is visible rather than papered over.

### Fragment 1 — asking for a frame outside a bake's range is defined, for the ocean modifier only

> "Frame Start, End
>
> Frames of the simulation to bake (inclusive). The start and end frames of the bake are repeated when accessing frames outside of the baked range."

**LABEL: documented.** Manual, Ocean Modifier — https://docs.blender.org/manual/en/5.1/modeling/modifiers/physics/ocean.html — read 2026-07-30.

So for this one modifier, a frame the bake does not cover gets the nearest baked end frame instead. This is about a frame outside the range of an existing bake. It says nothing about the cache folder being gone altogether, and the manual on that page gives no equivalent rule for any other cache or bake.

### Fragment 2 — without a bake, physics is computed while frames are played, and only forwards

> "Baking refers to the act of storing or caching the results of a calculation. The result of a simulation is automatically cached to memory when the animation is played, so that the next time it runs, it can be replayed more quickly by reading the results from the memory."

> "Note that playing the animation will try to simulate any visible physics simulations."

Manual, Baking Physics Simulations — https://docs.blender.org/manual/en/5.1/physics/baking.html — read 2026-07-30.

> "When an animation is played, each physics system writes each frame to the cache. Note that for the cache to fill up, one has to start the playback before or on the frame that the simulation starts."

> "The simulation is only calculated for positive frames in between the Start and End frames of the Cache panel, whether you bake or not. So if you want a simulation that is longer than the default frame range, you have to change the End frame."

Manual, Cache (particles) — https://docs.blender.org/manual/en/5.1/physics/particles/emitter/cache.html — read 2026-07-30.

**LABEL: documented** for the quoted words. **inferred**, and here is the step: these describe playback, not rendering. If rendering advances frames the same way playback does, then with no cache present a render that begins part-way through the simulation's range has not passed through the earlier frames, and the physics state at the first rendered frame is not the state it would have had. The sources do not say that rendering advances frames the same way playback does, and I did not find any page that says it. A reader has to supply that link, so this is inferred and nothing stronger. It is the closest the 5.1 documentation comes to answering question 18, and it still does not say what the render *produces*.

Two adjacent documented facts from the same pages, which bound how much a stale or absent cache can be trusted:

> "The cache is cleared automatically on changes. But not on all changes, so it may be necessary to delete it manually, e.g. if you change a force field."

> "The system is protected against changes after baking. If for example the mesh changes the simulation is not calculated anew."

Manual, Cache (particles), same address and date.

### Fragment 3 — baking to disk is what makes out-of-order rendering possible

> "When the result is ready to be sent to a render-farm, it can be baked to disk. This allows for the simulation to be rendered in a non-sequential order."

**LABEL: documented.** The same sentence appears twice: Manual, Simulation Nodes — https://docs.blender.org/manual/en/5.1/physics/simulation_nodes.html — and Manual, Simulation Zone — https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/simulation/simulation_zone.html — both read 2026-07-30.

**inferred**, step written out: if baking to disk is what "allows" non-sequential rendering, then without that bake non-sequential rendering is not allowed — that is, frames rendered out of order will not carry the right simulation state. The manual states the positive and leaves the negative to the reader, so this is inferred. It does not say what an out-of-order render without a bake actually produces, nor whether anything is reported.

Related, documented, from the same area:

> "Data is not stored to disk until the Bake operation is ran."

> "Important
>
> Blend-files must be saved to a disk before data can be baked."

> "It's not guaranteed that data written with one Blender version can be read by another Blender version."

Manual, Baking (geometry nodes) — https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/baking.html — read 2026-07-30. That last line matters here: a bake written by one version may be unreadable by another, which is a second way for a recorded bake to be effectively absent. The manual does not say what happens when it cannot be read.

### Fragment 4 — a cache can simply fail to be written in the first place

> "If you are not allowed to write to the required subdirectory caching will not take place, e.g. if your blend-file path is very long and your operating system has a limit on the path length that is supported."

**LABEL: documented.** Manual, Cache (particles) — https://docs.blender.org/manual/en/5.1/physics/particles/emitter/cache.html — read 2026-07-30. This describes a cache that is absent because it was never written. It does not say what a render then does.

### Fragment 5 — a mismatched external cache is described only as not working

> "Note
>
> The cache name in Caches List and the Index has to exactly match the external cache files name in order to work. The cache files name format is name_frame_index.bphys."

**LABEL: documented.** Manual, Baking Physics Simulations — https://docs.blender.org/manual/en/5.1/physics/baking.html — read 2026-07-30. "In order to work" is as far as the manual goes; what "not work" looks like in a render is not stated.

### Fragment 6 — the state flags exist, but their meaning when files are gone is not documented

**LABEL: documented** that these readable state values exist, with the words quoted below. **LABEL: not in the sources** for what any of them says once the files are gone.

The API exposes readable state, which is the machinery any answer to question 18 would run on, but describes none of it in terms of missing files:

> "info
>
> Info on current cache status (default "", readonly, never None)"

> "is_baked
>
> The cache is baked (default False, readonly)"

> "is_outdated
>
> (default False, readonly)"

> "is_frame_skip
>
> Some frames were skipped while baking/saving that cache (default False, readonly)"

Python API, `bpy.types.PointCache` — https://docs.blender.org/api/5.1/bpy.types.PointCache.html — read 2026-07-30. Note that `is_outdated` is documented with no description at all.

The fluid domain has a parallel set — `has_cache_baked_any`, `has_cache_baked_data`, `has_cache_baked_guide`, `has_cache_baked_mesh`, `has_cache_baked_noise`, `has_cache_baked_particles`, and matching `is_cache_baking_*` flags — in `bpy.types.FluidDomainSettings` (https://docs.blender.org/api/5.1/bpy.types.FluidDomainSettings.html, read 2026-07-30). The documentation does not say whether these are recomputed by looking at the disk or read from what the file stored, which is exactly the difference that decides what happens when the cache folder has been deleted or was never copied to the rendering machine.

Geometry-nodes modifiers can carry warnings — `bpy.types.NodesModifierWarning`, "Warning created during evaluation of a geometry nodes modifier", with a `message` and a `type` (https://docs.blender.org/api/5.1/bpy.types.NodesModifierWarning.html, read 2026-07-30) — but nothing says a missing bake produces one.

### Not covered

- The question as asked: what the render actually does. Not stated for any cache or bake type except the ocean modifier's out-of-range rule.
- Whether anything at all is reported — a message, a warning, a non-zero exit — when a cache or bake is absent at render time. Nothing found. (I did not chase the general "missing external file" machinery, since that is a different question on the sheet.)
- Whether the answer differs between the kinds of cache a file can record: point caches, fluid caches, geometry-nodes simulation zone bakes, Bake node bakes, ocean bakes, proxies, and Alembic/USD files behind a Mesh Sequence Cache modifier. The sources treat these on separate pages and none of them answers the question, so I cannot say whether they behave alike.
- Whether a partly-present cache — some frames there, some not — is treated differently from a wholly absent one.
- Whether an absent bake causes a fall-back to live simulation, or to no simulation at all.
- The Mesh Sequence Cache modifier and the Cache File data-block point at Alembic or USD files, which are caches the file records (Manual, Mesh Sequence Cache Modifier, https://docs.blender.org/manual/en/5.1/modeling/modifiers/modify/mesh_sequence_cache.html, read 2026-07-30). The page says nothing about the file being absent.
- Cycles' kernel cache is deliberately excluded: "If optimized kernels are not available, rendering will proceed using generic kernels until the optimized set is available in the cache" (`cycles_5.1/properties.py`, `kernel_optimization_level` description, around line 1641; **observed**). **observed** covers only that those words sit in that file. That Cycles keeps this cache for itself is my reading of them, not something the file states; what stands without that reading is that the grep found no path setting for it anywhere in `cycles_5.1`, so the `.blend` records no location for it. Either way it is not an answer to this question. I note it only so nobody later mistakes it for one.

### Outside the interface

**Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** 

Not in the sources. Nothing states whether a render driven from outside Blender's window behaves differently when a cache or bake is absent — which is precisely the case that matters and precisely the one nothing addresses.

Two documented facts frame the gap, both from Manual, Command Line Arguments (page header "Blender 5.1") — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30:

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

> "-f, --render-frame <frame>
>
> Render frame <frame> and save it.
>
> +<frame> start frame relative, -<frame> end frame relative.
>
> A comma separated list of frames can also be used (no spaces).
>
> A range of frames can be expressed using .. separator between the first and last frames (inclusive)."

> "-j, --frame-jump <frames>
>
> Set number of frames to step forward after each rendered frame."

So the command line offers both a start-to-end run and a single arbitrary frame. Whether the second one leaves an uncached simulation in the wrong state is exactly what fragment 2 above would predict, and exactly what the sources never say.

### Operating system

**Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** 

Not in the sources for this question. The only operating-system-specific statement I found anywhere near it is fragment 4's path-length note, which is about a cache failing to be written, not about a render finding one absent.

### Where I looked, and what I ruled out

Searched the whole of `manual_5.1` for, case-insensitively: `cache`, `bake`, `baked`, `not baked`, `unbaked`, `without baking`, `isn't baked`, `is not cached`, `no cache`, `cache is empty`, `cache file`, `bake file`, `blendcache`, `bphys`, `missing`, `absent`, `not found`, `cannot find`, `recalculat`, `re-calculat`, `resimulat`, `re-simulat`, `simulated again`, `outdated`, `out of date`, `out-of-date`, `render farm`, `render-farm`, `non-sequential`, `out of order`, `sequentially`. Read in full: Baking Physics Simulations, particle Cache, cloth Cache, soft body Cache, fluid domain Cache, dynamic paint Canvas, Simulation Nodes, Simulation Zone, geometry-nodes Baking, Bake Node, Geometry Nodes Modifier, Ocean Modifier, Mesh Sequence Cache Modifier, Rendering Animations, Render Output Introduction, Command Line Arguments, Topbar (for "Report Missing Files"), and Blender's Directory Layout. Ruled out as not bearing on the question: the Video Sequencer's "Show Missing Media" ("Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent", Manual, Sidebar Region (Preview), https://docs.blender.org/manual/en/5.1/editors/video_sequencer/preview/sidebar.html) — that is about missing strip media, not a cache or bake; the Freestyle View Map Cache and the animation player's frame cache — both memory caches held during a session, not things a file records; the asset-library index cache in the Local Cache Directory — not recorded by the file. Searched all of `api_5.1` for `PointCache`, `is_outdated`, `is_baked`, `has_cache_baked`, `cache status`, `bake_directory`, `bake_target`, `geometry_node_bake`, `simulation_nodes_cache`, `ptcache`, and read the `bpy.types.PointCache`, `bpy.types.PointCacheItem`, `bpy.types.FluidDomainSettings`, `bpy.types.NodesModifier`, `bpy.types.NodesModifierBake`, `bpy.types.NodesModifierWarning` and `bpy.ops.ptcache` pages in full — the operator descriptions are one line each ("Bake physics", "Bake from cache", "Delete physics bake") and say nothing about absence. Grepped all of `release_notes_5.1` for `cache` and `bake`: the only hits are bug-fix entries (a wrong baked frame range with packed bakes, a Bake node not keeping textures between sessions, the `foreach_path` fix cited under question 17, a memory leak in cached image code, and VSE cache invalidation), none of which describes what a render does when a cache is absent. Grepped all of `cycles_5.1` for `cache`: only the tile-to-disk and kernel-cache descriptions noted above, plus internal source comments about processor cache, which are not about this. Searched `requirements/requirements_2026-08-01.txt` for `5.1`, `cache`, `bake`, `output` — no matches, and the page does not name 5.1, so it cannot speak for this version.

---

**Observed — the hand tests for this round, run 2026-08-07 on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and 118 GB of free disk, and no display server at all.** Blender 5.1 was started from a terminal with no window, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and the Blender answered `RESOURCES /root/dependencies/res-5.1`. Every render used Cycles on the processor, so nothing here depends on the graphics card. The machine was shared with other topics' work throughout, so nothing here is a timing or memory measurement of a clean machine except where it says what it did about that. The full list of tests, with every set-up written out, is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

Pictures were compared as pictures, never as files. Before that was leaned on, two renders of the same frame of the same file were compared and gave `largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE`, so any difference reported below is a real difference.

**An unbaked physics simulation is worked out either way, but not to the same state.** A grid with a cloth modifier and no pinning, glowing white against a black world under an orthographic camera, frames 1 to 60, nothing baked and the animation never played. The file was opened twice: once going straight to frame 60, once stepping through every frame to it. Each time the evaluated grid was read and that frame rendered.

```
MODE jump FRAME 60  lowest z = -12.2144  highest z = -8.2118  lowest x = -2.0007  highest x = 2.0017
MODE step FRAME 60  lowest z = -11.7781  highest z = -7.7781  lowest x = -2.0000  highest x = 2.0000
PICTURES size=(160, 160) largest channel difference=1.000000 channels differing=744 of 102400
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
use_external True | filepath /root/dependencies/work/q18/5.1/emptycache | name 'SmoozyCache'
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
PICTURES size=(160, 160) largest channel difference=1.000000 channels differing=909 of 102400
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
PICTURES size=(160, 160) largest channel difference=1.000000 channels differing=909 of 102400
DIFFERENT PICTURE
--- the baked one against the stepped-through no-bake render ---
PICTURES size=(160, 160) largest channel difference=0.000000 channels differing=0 of 102400
SAME PICTURE
--- the unbaked one against the jumped no-bake render ---
PICTURES size=(160, 160) largest channel difference=0.701961 channels differing=90 of 102400
DIFFERENT PICTURE
```

So the bake is what makes a single out-of-order frame match the stepped-through result exactly, and its loss is silent, with exit status 0.

**A fluid domain's flags are held in the file and do not notice the cache going — and opening the file makes the folder again and writes into it.** A small gas simulation, domain resolution 16, cache directory of its own, frames 1 to 12, baked with `bpy.ops.fluid.bake_all()` and saved:

```
BEFORE BAKE | cache_directory: /root/dependencies/work/q18fluid/5.1/fluidcache | has_cache_baked_any: False | has_cache_baked_data: False
BAKE ALL -> {'FINISHED'}
AFTER BAKE  | cache_directory: /root/dependencies/work/q18fluid/5.1/fluidcache | has_cache_baked_any: True | has_cache_baked_data: True
cache folder: ['config', 'data']
cache folder removed; exists now: False
SAME SESSION, FOLDER GONE | has_cache_baked_any: True | has_cache_baked_data: True
REOPENED with the cache folder gone (folder exists: True )
  has_cache_baked_any: True | has_cache_baked_data: True | has_cache_baked_noise: True | has_cache_baked_mesh: True | has_cache_baked_particles: True
```

and what the reopening had put back on disk:

```
/root/dependencies/work/q18fluid/5.1/fluidcache/config/config_0001.uni
/root/dependencies/work/q18fluid/5.1/fluidcache/data/fluid_data_0001.vdb
```

**An EEVEE light probe volume with no bake does render differently.** A closed room with its faces turned inward so the world cannot light it directly, a sphere inside, a light probe volume covering it, and a strong green world. Rendered unbaked, then with `bpy.ops.object.lightprobe_cache_bake(subset='ALL')`, then after `bpy.ops.object.lightprobe_cache_free(subset='ALL')`:

```
ENGINE BLENDER_EEVEE
  unbaked against baked: largest channel difference=0.768628 channels differing=24812 of 57600 — DIFFERENT PICTURE
  baked against freed  : largest channel difference=0.768628 channels differing=24812 of 57600 — DIFFERENT PICTURE
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

**This version has no texture cache to test.** `scene.render.use_texture_cache` does not exist in it, nor `use_auto_generate_texture_cache`, nor the `texture_cache_directory` preference, and `generate_texture_cache` is not in `dir(bpy.ops.render)`. The same is true of 4.5 and 5.0; 5.2 has all four. So the texture cache is a real difference between the versions and not a gap in one version's documentation.

**A disk point cache written by 4.5 with a compression this version dropped is announced.** The cloth file was baked in Blender 4.5 with `point_cache.compression = 'LIGHT'` and Disk Cache on, and the same saved file was then opened and rendered here:

```
EXIT 0
is_baked True | is_outdated False | is_frame_skip False | use_disk_cache True | info '60 frames on disk.'
Warning: Cloth Cache in object Grid can not be read because it uses an outdated compression method. You need to delete the caches and re-bake.
render EXIT 0
00:00.346  reports          | WARNING Cloth Cache in object Grid can not be read because it uses an outdated compression method. You need to delete the caches and re-bake.
00:00.560  render           | Saved: '/root/dependencies/work/oldcache/from5.1_0060.png'
PICTURES size=(160, 160) largest channel difference=1.000000 channels differing=4899 of 102400
DIFFERENT PICTURE
```

This is the only case in all of these tests where a cache that cannot be used is named on the console of a run with no window. The exit status is still 0, `is_baked` still reads `True`, and the picture differs from the one 4.5 renders with the same cache. 5.0 and 5.2 printed the same words. `PointCache.compression` exists in 4.5 with the choices `NO`, `LIGHT` and `HEAVY`, and does not exist in this version at all.

**Observed — the Mac half of this round's hand tests, run 2026-08-07 on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card.** Blender 5.1.2 was started from a terminal, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and this Blender answered `RESOURCES …/depwork/res-5.1` and `VERSION 5.1.2`. The application used was `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`. Every render used Cycles on the processor. **Other topics were running Blender on this same Mac throughout, so nothing was timed and no memory was measured; there is no timing or memory reading anywhere below.** **This is an addition to a document written earlier in this round; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

In the machine's own words quoted below, the working folder's long path is written `…/depwork`; nothing else in any quotation is altered.

Pictures were compared as pictures, never as files. Before that was leaned on, two renders of the same frame of the same file were compared on this Mac and gave `largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE`, so any difference reported below is a real difference.

**Everything the Linux half found about an absent cache or bake holds on macOS too.** Taken in order:

*An unbaked physics simulation is worked out either way, but not to the same state.* A grid standing up with a cloth modifier and no pinning, glowing white against a black world under an orthographic camera, frames 1 to 60, nothing baked and the animation never played. The file was opened twice: once going straight to frame 60, once stepping through every frame to it.

```
MODE jump FRAME 60  lowest z = -12.2137  highest z = -8.2122  lowest x = -2.0005  highest x = 2.0010
MODE step FRAME 60  lowest z = -11.7781  highest z = -7.7781  lowest x = -2.0000  highest x = 2.0000
PICTURES size=(160, 160) largest channel difference=0.772549 channels differing=870 of 102400
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
PICTURES size=(160, 160) largest channel difference=0.772549 channels differing=4893 of 102400
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
use_external True | filepath …/depwork/q18/5.1/emptycache | name 'SmoozyCache'
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
PICTURES size=(160, 160) largest channel difference=0.772549 channels differing=912 of 102400
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
PICTURES size=(160, 160) largest channel difference=0.772549 channels differing=909 of 102400
DIFFERENT PICTURE
--- the baked one against the stepped-through no-bake render ---
PICTURES size=(160, 160) largest channel difference=0.000000 channels differing=0 of 102400
SAME PICTURE
--- the unbaked one against the jumped no-bake render ---
PICTURES size=(160, 160) largest channel difference=0.113725 channels differing=120 of 102400
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
…/depwork/q18fluid/5.1/fluidcache/config/config_0001.uni
…/depwork/q18fluid/5.1/fluidcache/data/fluid_data_0001.vdb
```

*An EEVEE light probe volume with no bake does render differently, and freeing the bake puts the render back exactly.*

```
ENGINE BLENDER_EEVEE | EEVEE names offered: ['BLENDER_EEVEE']
  unbaked against baked: largest channel difference=0.815686 channels differing=36905 of 57600 — DIFFERENT PICTURE
  baked against freed  : largest channel difference=0.815686 channels differing=36905 of 57600 — DIFFERENT PICTURE
  unbaked against freed: largest channel difference=0.000000 channels differing=0 of 57600 — SAME PICTURE
```

Nothing was printed about the missing bake. **On this Mac EEVEE printed no warning of any kind before working** — the `libEGL` and `EGL_BAD_MATCH` lines the Linux machine printed are a thing of a machine with no display server, not of EEVEE.

*A disk point cache written by 4.5 with a compression this version dropped is announced.* The cache was baked in Blender 4.5 on this same Mac with `point_cache.compression = 'LIGHT'` and its file carried here. This version printed the same sentence twice, once on opening and once while rendering, and still exited 0:

```
EXIT 0
Warning: Cloth Cache in object Grid can not be read because it uses an outdated compression method. You need to delete the caches and re-bake.
is_baked True | is_outdated False | is_frame_skip False | use_disk_cache True | info '60 frames on disk.'
render EXIT 0
00:00.700  reports          | WARNING Cloth Cache in object Grid can not be read because it uses an outdated compression method. You need to delete the caches and re-bake.
PICTURES size=(160, 160) largest channel difference=0.772549 channels differing=4899 of 102400
DIFFERENT PICTURE
```

That is word for word what the Linux machine printed. This remains the one case in this whole round where a cache that cannot be used is announced. `is_baked` still reads `True` and `info` still reads `60 frames on disk.`


### What the window could be asked, and what it could not

The Creator's Mac has a screen, so this is the machine where the drawing questions can be reached. The same limit described under question 17 applies: the window is created and drawn, but its event loop never runs when Blender is started this way from a terminal, so anything needing an event could not be reached.

**The Timeline does show the loss, by drawing nothing.** Entry 13's file was rebuilt with the object carrying the cloth cache selected and active, the Timeline's cache-drawing options turned on, and the current frame set to 30; it was baked to disk, opened with a window and photographed with Blender's own screenshot command; then the `blendcache_` folder was deleted and it was opened and photographed again. Reading the strip of the picture where the cache bar is drawn:

```
with the cache present  average colour r=0.160 g=0.167 b=0.253
with the folder deleted average colour r=0.155 g=0.162 b=0.180
the two strips compared as pictures: largest channel difference=0.349020 channels differing=24190 of 112000 — DIFFERENT PICTURE
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

2026-08-06 — Opus 5 (Claude Code) — Dependencies, Blender 5.1
