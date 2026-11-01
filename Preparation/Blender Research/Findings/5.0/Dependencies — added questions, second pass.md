# Dependencies — added questions, second pass (Blender 5.0)

**Sources consulted, and when.** Blender's 5.0 manual (plain text plus its `_images` pictures), Blender's 5.0 Python API documentation, and Blender's 5.0 release notes — all read on **2026-07-30** (the date the copies in `Blender Sources/` were taken). Cycles' own files for 5.0 were also searched, on the same date; they turned out to hold nothing bearing on either question, and I say where I looked below. Blender's requirements page was not needed and is not cited.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only questions 17 and 18 were in scope. The rest of the sheet was not touched.

---

## 17. What use does a `.blend` file make of the places it records for writing, besides writing to them?

**Label: documented**, with three points inside it marked **inferred** where I had to take a step. One point is marked as falling outside the question.

### The reading I took, and the other one

I read "the places it records for writing" as the destinations a file stores for output it will produce: the render output path, the compositor File Output node's directory, and the various cache, bake and proxy directories. The question is ambiguous — it could also be read as "every path in the file", since almost any path can be written to at some point (unpacking, for one). I answered the first reading. Under the second reading the answer would also have to cover ordinary texture and library paths, which the rest of this sheet already deals with.

### A. The render output location is read back to decide which frames to render

> "Overwrite existing files when rendering."

> "Create empty placeholder frames while rendering."

Manual, Output panel, `https://docs.blender.org/manual/en/5.0/render/output/properties/output.html`, read 2026-07-30.

The same page's hint spells out what that is for:

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Same page, same date.

And on the animation page:

> "Just disable the Overwrite option to start rendering where you left off."

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

Manual, `https://docs.blender.org/manual/en/5.0/render/output/animation.html`, read 2026-07-30.

The same page again, for partial renders:

> "Using an image format for output, you can use the Frame Step option to render every N'th frame. Then disable Overwrite and re-render with Frame Step set to 1."

The Python side gives the two switches and their defaults:

> "use_overwrite
>
> Overwrite existing files while rendering
>
> Type:
>
> boolean, default True"

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch')
>
> Type:
>
> boolean, default False"

API documentation, `https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html`, read 2026-07-30.

I also opened the picture of the panel, `_images/render_output_properties_output_panel.png`, named on the Output page above. It shows the path field (holding `/tmp/`) with four checkboxes beside it: Overwrite (ticked), Placeholders (unticked), File Extensions (ticked), Cache Result (unticked). **Documented**, from a picture that is part of the manual. I looked at it because the text of that page prints the same heading twice ("Image Sequence – Overwrite Image Multi-Layer EXR") for what are plainly two different settings; the picture settles that the second one is called Placeholders.

**Inferred (one step):** to skip frames that are already there, and to resume where it left off, Blender must look at the output location before writing to it. The manual states the outcome, not the mechanism. It does not say whether Blender lists the directory or asks after each expected file name by name.

### B. The render output location is read back to play the result

> "Launching the animation player opens a new window, playing back images or a video located at the render output of the current scene."

Manual, `https://docs.blender.org/manual/en/5.0/render/output/animation_player.html`, read 2026-07-30.

> "Rendered image sequences can be played back in the Animation Player."

Manual, `https://docs.blender.org/manual/en/5.0/render/output/introduction.html`, read 2026-07-30.

> "bpy.ops.render.play_rendered_anim()
>
> Play back rendered frames/movies using an external player"

API documentation, `https://docs.blender.org/api/5.0/bpy.ops.render.html`, read 2026-07-30.

### C. The render output location can be asked where a frame will land, without rendering

**Label: documented** that a path is handed back; **inferred** that nothing is rendered or written when it is asked for. *(Added by the citation check, 2026-08-06: this section carried the document's overall "documented" label. The quoted words say a path is returned; they do not say that nothing is written. This version's first-pass document marks the same step inferred.)*

> "frame_path(*, frame=-2147483648, preview=False, view='')
>
> Return the absolute path to the filename to be written for a given frame"

and, for its result:

> "File Path, The resulting filepath from the scenes render settings"

API documentation, `https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html`, read 2026-07-30.

This turns the recorded location into a question that can be answered ahead of time.

### D. The render output location is parsed and checked, and a bad one stops the render

The recorded path may hold template expressions, which are worked out when the path is used rather than when it is typed:

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

> "When there are template errors in a path, the path field will be highlighted red in the UI"

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

Manual, `https://docs.blender.org/manual/en/5.0/files/file_paths.html`, read 2026-07-30. The same page notes which paths this covers:

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."

So the recorded write location is also a thing the file checks and complains about.

### E. The render output location is borrowed by the viewport render

> "Additionally, some output settings are used too:
>
> Resolution
>
> Aspect
>
> Output path
>
> File format"

Manual, `https://docs.blender.org/manual/en/5.0/editors/3dview/viewport_render.html`, read 2026-07-30.

### F. Cache and bake locations are read back

This is the largest use. The same recorded place that a simulation is written to is where it is loaded from again.

Physics caches:

> "External
>
> Allows you to read the cache from a drive using a user-specified file path."

> "Path
>
> Select the directory path to the cache files."

Manual, `https://docs.blender.org/manual/en/5.0/physics/baking.html`, read 2026-07-30. In Python the same switch reads:

> "use_external
>
> Read cache from an external location"

API documentation, `https://docs.blender.org/api/5.0/bpy.types.PointCache.html`, read 2026-07-30.

Fluid caches:

> "cache_directory
>
> Directory that contains fluid cache files"

API documentation, `https://docs.blender.org/api/5.0/bpy.types.FluidDomainSettings.html`, read 2026-07-30.

> "Offset
>
> Frame offset that is used when loading the simulation from the cache. It is not considered when baking the simulation, only when loading it."

> "Resumable
>
> Extra data will be saved so that you can resumed baking after pausing."

Manual, `https://docs.blender.org/manual/en/5.0/physics/fluid/type/domain/cache.html`, read 2026-07-30. The second one is a further use: what is already at the recorded place is read in order to carry on a bake that was stopped.

The Ocean modifier is the plainest statement of the whole point:

> "Rather than simulating the ocean data live, it can be baked to files in a given directory. When a simulation is baked, the simulator engine is completely bypassed, and all information for the modifier or texture is retrieved from the baked files."

> "Upon loading the data from these baked files, when a frame of the bake sequence is read, it is cached in memory. This means that accessing loaded frames subsequent times is fast, not incurring the overhead of drive access."

> "Since these baked files are plain OpenEXR's, they can also be opened and rendered in any other application or renderer that supports them."

> "If the cache path folder does not exist, it will be created."

Manual, `https://docs.blender.org/manual/en/5.0/modeling/modifiers/physics/ocean.html`, read 2026-07-30. Three separate uses beyond writing: the place is read from, its contents are held in memory once read, and it doubles as a handover point to other programs. The last quoted line is a fourth: the recorded place is made to exist.

Geometry node bakes:

> "Pack/Unpack
>
> Packs or unpacks already baked data."

Manual, `https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/geometry/operations/bake.html`, read 2026-07-30. And in Python:

> "bpy.ops.object.geometry_node_bake_pack_single(*, session_uid=0, modifier_name='', bake_id=0)
>
> Pack baked data from disk into the .blend file"

API documentation, `https://docs.blender.org/api/5.0/bpy.ops.object.html`, read 2026-07-30. So the recorded bake directory is also where the file pulls data back in from, to stop needing that directory at all.

### G. Cache and bake locations are inspected to report state

> "When data is baked the number of baked frames is displayed above the simulation zone or bake nodes."

Manual, `https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/baking.html`, read 2026-07-30.

> "Baked simulations will be shown as fully opaque, cached simulations will be slightly transparent, and invalid caches will be slightly transparent with dark diagonal stripes."

Manual, `https://docs.blender.org/manual/en/5.0/editors/dope_sheet/navigating.html`, read 2026-07-30.

The Python side exposes the same state:

> "info
>
> Info on current cache status"

> "is_baked
>
> The cache is baked"

> "is_frame_skip
>
> Some frames were skipped while baking/saving that cache"

> "is_outdated"

API documentation, `https://docs.blender.org/api/5.0/bpy.types.PointCache.html`, read 2026-07-30.

**Inferred (one step):** for a cache kept on disk rather than in memory, reporting this state means reading the recorded location. The sources report the state; they do not say where the state is read from.

### H. The recorded cache place doubles as an identity that two files can share

> "Disk Cache
>
> The cache of a baked simulation will be stored inside the blend-file when you save it. When Disk Cache is checked, Blender will save the cache separately to the drive in a folder named blendcache_[filename] alongside the blend-file. (The blend-file must be saved first.)"

> "Use Library Path
>
> Share the disk cache when the physics object is linked into another blend-file. When this option is enabled, linked versions of the object will reference the same disk cache. Otherwise linked versions of the object will use independent caches."

Manual, `https://docs.blender.org/manual/en/5.0/physics/baking.html`, read 2026-07-30. In Python:

> "use_library_path
>
> Use this file's path for the disk cache when library linked into another file (for local bakes per scene file, disable this option)"

API documentation, `https://docs.blender.org/api/5.0/bpy.types.PointCache.html`, read 2026-07-30.

So the recorded write place is also the thing that decides whether two files see one simulation or two. The same page also makes the file names inside it matter in their own right:

> "Each cache can have a name. Double-click the cache entry to give it a name. If this name is given, any disk cache will be stored in files starting with that name. For example, a cache named MyCache will be stored in MyCache_xxxxxx_yy.bphys."

> "The cache name in Caches List and the Index has to exactly match the external cache files name in order to work. The cache files name format is name_frame_index.bphys."

### I. Proxy locations are read back and checked for what is already there

> "File
>
> Allows you to use preexisting proxies."

> "Overwrite
>
> Whether to overwrite existing proxy files or keep them."

> "Directory
>
> By default, all generated proxy videos are stored to the folder <path of original footage>/BL_proxy/<clip name>, but this can be changed to a custom directory using this option."

Manual, `https://docs.blender.org/manual/en/5.0/editors/video_sequencer/sequencer/sidebar/proxy.html`, read 2026-07-30.

### J. One thing that looks like an answer but is not the file's own record

The Cache Result option makes a render readable again later, but the place it goes is a preference, not something the `.blend` records:

> "Saving – Cache Result
>
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image. The Compositor can then use this file to improve performance, especially for heavy compositing.
>
> The image is stored in the Render Cache folder as specified in the File Paths Preferences. You can also load it back into the Image Editor's Render Result, even after closing and reopening Blender; see Open Cached Render."

Manual, `https://docs.blender.org/manual/en/5.0/render/output/properties/output.html`, read 2026-07-30.

> "Find the render cache file for the current scene and load it into the Render Result. This way, you can restore the last render from a previous Blender session and continue working in the Compositor without having to render the scene again."

Manual, `https://docs.blender.org/manual/en/5.0/editors/image/editing.html`, read 2026-07-30.

The file records the switch; the location comes from Preferences ("Render Cache — The location where cached render images are stored", manual, `https://docs.blender.org/manual/en/5.0/editors/preferences/file_paths.html`, read 2026-07-30). I include it because the pattern — write it, then read it back into the same session or a later one — is exactly what the question asks about, but the place itself is not the file's.

### Not covered

- How Blender checks the output location with Overwrite off — one directory listing or one question per expected file name — is not stated anywhere I found. That matters if several machines share a directory over a network.
- Whether the placeholder is created before or after the frame starts rendering, and what happens if a machine dies between the two, is not stated.
- Nothing was found saying the compositor's File Output node checks what is already at its directory. Its page describes writing only.
- Nothing was found about how any of these behave on a network file system beyond the bare hint quoted in A.
- Whether the render output location is used for anything at all when the format is a video rather than an image sequence (the Overwrite and Placeholders settings are labelled "Image Sequence" in the manual's headings) is not settled.

### Outside the interface

**Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** 

Partly holds, and one part plainly does not.

The recorded output location can be set and overridden from outside:

> "-o, --render-output <path>
>
> Set the render path and file name. Use // at the start of the path to render relative to the blend-file.
>
> You can use path templating features such as {blend_name} in the path."

Manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`, read 2026-07-30. The same page warns that the order matters, because opening the file replaces what was set:

> "…will not render to /tmp because loading the blend-file overwrites the render output that was set."

The Overwrite and Placeholders settings have **no command-line switch** in the list of Render Options and Format Options on that page (which give `-b`, `-a`, `-S`, `-f`, `-s`, `-e`, `-j`, `-o`, `-E`, `-t`, `-F`, `-x`). So outside the interface they can only come from what the file already holds, or from a script. **The sources do not say** whether they are honoured in background mode; the "Primitive Render Farm" hint describes several machines sharing a directory but never says those machines are running without a window.

The playback use (B) is the part that differs. Playing back the recorded output location is an operator and a menu item ("Topbar ‣ Render ‣ View Animation", manual, `https://docs.blender.org/manual/en/5.0/render/output/animation_player.html`). The command line has a player too, but it takes files given to it rather than the scene's recorded location, and it is turned off in background mode:

> "-a <options> <file(s)>
>
> Instead of showing Blender's user interface, this runs Blender as an animation player, to view movies and image sequences rendered in Blender (ignored if -b is set)."

Manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`, read 2026-07-30.

The bake-related uses (F, G, H) carry one requirement that holds either way:

> "In order to bake the simulation, the blend-file must be saved to your computer. The location the file is saved determines where the baked data is also saved."

Manual, `https://docs.blender.org/manual/en/5.0/physics/simulation_nodes.html`, read 2026-07-30.

### Operating system

**Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** 

The sources give one operating-system-dependent limit, and it is about writing to a cache place:

> "If you are not allowed to write to the required subdirectory caching will not take place, e.g. if your blend-file path is very long and your operating system has a limit on the path length that is supported."

Manual, `https://docs.blender.org/manual/en/5.0/physics/particles/emitter/cache.html`, read 2026-07-30.

Beyond that, **the sources do not say**. The one other mention of a platform is descriptive rather than limiting — the placeholder switch is explained as "similar to Unix 'touch'" (API documentation, `https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html`) — and describes what it does, not where it works.

### Where I looked

Searched the whole 5.0 manual and the whole 5.0 API documentation for: `placeholder`, `overwrite`, `output path`, `output directory`, `output folder`, `render output`, `resume`, `already rendered`, `existing files`, `skip`, `touch`, `cache`, `bake`, `blendcache`, `bake_directory`, `cache_directory`, `proxy_dir`, `base_path`, `Play Rendered`, `play_rendered_anim`, `Render Cache`, `Open Cached Render`, `template expression`, `will be created`, `writable`, `not allowed to write`, `permission`.

Pages opened in full: Output panel; Rendering Animations; Render Output introduction; Animation Player; File Paths (path templates); Preferences File Paths; Command Line Arguments; Rendering From The Command Line; Baking Physics Simulations; Particle Cache; Cloth Cache; Soft Body Cache; Fluid Domain Cache; Simulation Nodes; Simulation Zone; Geometry Nodes Baking; Bake Node; Geometry Nodes Modifier; Ocean Modifier; Sequencer Proxy; Sequencer Cache; Packed Data; Topbar (Report/Find Missing Files); Metadata; Viewport Render; Image Editor editing. API pages opened: `RenderSettings`, `PointCache`, `NodesModifier`, `NodesModifierBake`, `OceanModifier`, `FluidDomainSettings`, `SequenceEditor`, `bpy.ops.render`, `bpy.ops.object`.

Ruled out and why: the Metadata panel stamps "the filename of the blend-file", not the output location, so it is not a use of the recorded write place. The Sequencer's Cache panel is memory only and records no location. Cycles' own files (`properties.py`, `ui.py`, `engine.py`, `operators.py`, `__init__.py`, `camera.py`, `osl.py`, `presets.py`, `version_update.py`, searched for `cache`, `filepath`, `directory`, `dirpath`) hold no output or cache location of their own; the only two matches were tile memory ("Tiles are cached to disk while rendering to save memory") and the kernel cache, neither of which is a place the `.blend` records. The 5.0 release notes were searched for `cache`, `bake` and `template`; nothing there bears on this question except the path template variables added in 5.0 (`https://developer.blender.org/docs/release_notes/5.0/core/`), which are covered in D.

---

**Observed — the hand tests for this round, run 2026-08-07 on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and 118 GB of free disk, and no display server at all.** Blender 5.0 was started from a terminal with no window, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and the Blender answered `RESOURCES /root/dependencies/res-5.0`. Every render used Cycles on the processor, so nothing here depends on the graphics card. The machine was shared with other topics' work throughout, so nothing here is a timing or memory measurement of a clean machine except where it says what it did about that. The full list of tests, with every set-up written out, is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

Pictures were compared as pictures, never as files. Before that was leaned on, two renders of the same frame of the same file were compared and gave `largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE`, so any difference reported below is a real difference.

**A missing output folder is made.** A file whose render output path pointed into a folder that did not exist was rendered with `--background --factory-startup <file> --render-anim`, and again with the path two folders deep into folders that did not exist. Both worked, exit status 0:

```
EXIT STATUS 0
00:00.264  blend            | Read blend: "/root/dependencies/work/q17/5.0/render.blend"
00:00.504  render           | Saved: '/root/dependencies/work/q17/5.0/deep3/deep4/frame_0001.png'
00:00.504  render           | Time: 00:00.22 (Saving: 00:00.10)
FILES:
/root/dependencies/work/q17/5.0/deep3
/root/dependencies/work/q17/5.0/deep3/deep4
/root/dependencies/work/q17/5.0/deep3/deep4/frame_0001.png
```

**The output place is looked at, and Blender says so by name.** Five frames were rendered, frames 2 and 4 deleted, `use_overwrite` set to False, and the animation rendered again with no window:

```
00:00.281  render           | Skipping existing frame "/root/dependencies/work/q17/5.0/ow/frame_0001.png"
00:00.543  render           | Saved: '/root/dependencies/work/q17/5.0/ow/frame_0002.png'
00:00.543  render           | Skipping existing frame "/root/dependencies/work/q17/5.0/ow/frame_0003.png"
00:00.683  render           | Saved: '/root/dependencies/work/q17/5.0/ow/frame_0004.png'
00:00.683  render           | Skipping existing frame "/root/dependencies/work/q17/5.0/ow/frame_0005.png"
```

Only the two deleted files carried a new modification time afterwards; the other three kept theirs.

**An empty file counts as a frame already done.** With `use_overwrite` off and `use_placeholder` on, five zero-length files were made at the five names `frame_path()` gave, and the animation rendered:

```
EXIT 0
00:00.274  render           | Skipping existing frame "/root/dependencies/work/q17/5.0/ph/frame_0001.png"
00:00.274  render           | Skipping existing frame "/root/dependencies/work/q17/5.0/ph/frame_0002.png"
00:00.274  render           | Skipping existing frame "/root/dependencies/work/q17/5.0/ph/frame_0003.png"
00:00.274  render           | Skipping existing frame "/root/dependencies/work/q17/5.0/ph/frame_0004.png"
00:00.274  render           | Skipping existing frame "/root/dependencies/work/q17/5.0/ph/frame_0005.png"
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
PATH SET TO '/root/dependencies/work/q17/5.0/tmpl/{{blend_name}}_'
FRAMEPATH OK '/root/dependencies/work/q17/5.0/tmpl/tmplok_0001.png'
EXIT 0
00:00.593  render           | Saved: '/root/dependencies/work/q17/5.0/tmpl/tmplok_0001.png'
=== the BAD template ===
PATH SET TO '/root/dependencies/work/q17/5.0/tmpl/{{blend_name.png'
FRAMEPATH RAISED RuntimeError Error: Parse errors in path '/root/dependencies/work/q17/5.0/tmpl/{{blend_name.png':
--- single frame ---
EXIT 1
00:00.273  reports          | ERROR Parse errors in path '/root/dependencies/work/q17/5.0/tmpl/{{blend_name.png':
                            | - Invalid or incomplete template expression '{{blend_name.png'.
--- animation ---
EXIT 1
00:00.279  reports          | ERROR Parse errors in path '/root/dependencies/work/q17/5.0/tmpl/{{blend_name.png':
                            | - Invalid or incomplete template expression '{{blend_name.png'.
```

**The recorded path beats one given before the file on the command line.**

```
--- output given BEFORE the file ---
00:00.551  render           | Saved: '/root/dependencies/work/q17/5.0/out/frame_0001.png'
--- output given AFTER the file ---
00:00.520  render           | Saved: '/root/dependencies/work/q17/5.0/ord/after/x_0001.png'
```

**`frame_path()` names the file that appears, for a frame render and for an animation — and not for a still render.** With the output path `shot_{{blend_name}}_##_`:

```
PREDICTED frame 7: /root/dependencies/work/q17/5.0/fp2/shot_fp2_07_.png
00:00.518  render           | Saved: '/root/dependencies/work/q17/5.0/fp2/shot_fp2_07_.png'
```

but through `bpy.ops.render.render(write_still=True)` at frame 7, on the same path, the template was substituted and the `##` padding was not:

```
PREDICTED /root/dependencies/work/q17/5.0/fp/shot_fp_07_.png
ON DISK ['shot_fp_##_.png']
MATCH False
```

**The file-wide path commands reach an ocean cache path and nothing else that is recorded for writing.** One file was built holding six absolute locations — an image loaded from disk, the render output path, a compositor File Output node's directory, a point cache with External on, a geometry-nodes bake directory, and an ocean modifier's cache path:

```
--- as saved ---
  image ReadTex   : /root/dependencies/work/q17paths/5.0/assets/tex.png
  render output   : /root/dependencies/work/q17paths/5.0/renders/frame_
  file output node: /root/dependencies/work/q17paths/5.0/fileout
  point cache     : /root/dependencies/work/q17paths/5.0/ptcache
  gn bake dir     : /root/dependencies/work/q17paths/5.0/gnbake
  ocean cache     : /root/dependencies/work/q17paths/5.0/oceanbake
MAKE PATHS RELATIVE -> {{'FINISHED'}}
--- after Make Paths Relative ---
  image ReadTex   : //assets/tex.png
  render output   : /root/dependencies/work/q17paths/5.0/renders/frame_
  file output node: /root/dependencies/work/q17paths/5.0/fileout
  point cache     : /root/dependencies/work/q17paths/5.0/ptcache
  gn bake dir     : /root/dependencies/work/q17paths/5.0/gnbake
  ocean cache     : //oceanbake
```

Make Paths Absolute put both back. With all six pointed at a folder that does not exist, `bpy.ops.file.report_missing_files()` named two of them:

```
Warning: Path '/root/dependencies/work/q17paths/5.0/gone' not found, from local data-block 'OBGrid'
Warning: Path '/root/dependencies/work/q17paths/5.0/gone/tex.png' not found, from local data-block 'IMReadTex'
BLEND PATHS: ['/root/dependencies/work/q17paths/5.0/gone', '/root/dependencies/work/q17paths/5.0/gone/tex.png', '/root/dependencies/work/q17paths/5.0/assets/tex.png']
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
    Grid '/root/dependencies/work/q17paths/5.0/oceanbake'
    ReadTex '/root/dependencies/work/q17paths/5.0/assets/tex.png'
    Tex '/root/dependencies/work/q17paths/5.0/assets/tex.png'
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
Error: Frame '/root/dependencies/work/denoise/5.0/f_0003.exr' not found, animation must be complete
```

In this version `OPEN_EXR_MULTILAYER` is not in the file-format list until `image_settings.media_type = 'MULTI_LAYER_IMAGE'` is set first; in 4.5 the format can be chosen straight away.

**Observed — the Mac half of this round's hand tests, run 2026-08-07 on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card.** Blender 5.0.1 was started from a terminal, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and this Blender answered `RESOURCES …/depwork/res-5.0` and `VERSION 5.0.1`. The application used was `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`. Every render used Cycles on the processor. **Other topics were running Blender on this same Mac throughout, so nothing was timed and no memory was measured; there is no timing or memory reading anywhere below.** **This is an addition to a document written earlier in this round; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

In the machine's own words quoted below, the working folder's long path is written `…/depwork`; nothing else in any quotation is altered.

Pictures were compared as pictures, never as files. Before that was leaned on, two renders of the same frame of the same file were compared on this Mac and gave `largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE`, so any difference reported below is a real difference.

**Everything the Linux half found about the output place holds on macOS too.** Taken in order:

*A missing output folder is made.* The same file rendered with `--background --factory-startup <file> --render-anim`, and again with `--render-output <workdir>/deep3/deep4/frame_ --render-frame 1` into two folders that did not exist. Both worked, exit status 0, nothing said about the folder:

```
EXIT STATUS 0
00:00.289  blend            | Read blend: "…/depwork/q17/5.0/render.blend"
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
00:00.458  render           | Saved: '…/depwork/q17/5.0/deep3/deep4/frame_0001.png'
00:00.458  render           | Time: 00:00.16 (Saving: 00:00.05)
Blender quit
FILES:
…/depwork/q17/5.0/deep3
…/depwork/q17/5.0/deep3/deep4
…/depwork/q17/5.0/deep3/deep4/frame_0001.png
```

*The output place is looked at before writing.* Five frames rendered, frames 2 and 4 deleted, `use_overwrite` set to False, the animation rendered again with no window:

```
00:00.724  render           | Skipping existing frame "…/depwork/q17/5.0/ow/frame_0001.png"
00:01.041  render           | Saved: '…/depwork/q17/5.0/ow/frame_0002.png'
00:01.041  render           | Skipping existing frame "…/depwork/q17/5.0/ow/frame_0003.png"
00:01.535  render           | Saved: '…/depwork/q17/5.0/ow/frame_0004.png'
00:01.535  render           | Skipping existing frame "…/depwork/q17/5.0/ow/frame_0005.png"
```

and only the two deleted files carried a new modification time afterwards:

```
before   frame_0001.png 1786065792 8525     after   frame_0001.png 1786065792 8525
         frame_0003.png 1786065792 8525             frame_0002.png 1786065796 8525
         frame_0005.png 1786065792 8525             frame_0003.png 1786065792 8525
                                                    frame_0004.png 1786065796 8525
                                                    frame_0005.png 1786065792 8525
```

*An empty file counts as a frame already done.* With `use_overwrite` off and `use_placeholder` on, five zero-length files were put at the five names `frame_path()` gave:

```
--- five zero-length files placed ---
frame_0001.png 0   frame_0002.png 0   frame_0003.png 0   frame_0004.png 0   frame_0005.png 0
--- render the animation, no window ---
EXIT 0
00:00.320  render           | Skipping existing frame "…/depwork/q17/5.0/ph/frame_0001.png"
00:00.320  render           | Skipping existing frame "…/depwork/q17/5.0/ph/frame_0002.png"
00:00.320  render           | Skipping existing frame "…/depwork/q17/5.0/ph/frame_0003.png"
00:00.320  render           | Skipping existing frame "…/depwork/q17/5.0/ph/frame_0004.png"
00:00.320  render           | Skipping existing frame "…/depwork/q17/5.0/ph/frame_0005.png"
00:00.321  reports          | No frames rendered, skipped to not overwrite
--- sizes after ---
frame_0001.png 0   frame_0002.png 0   frame_0003.png 0   frame_0004.png 0   frame_0005.png 0
```

Nothing was rendered, all five were still zero length, and the run still reported success.

*A bad path template stops a render started from outside, for a single frame as well as for an animation*, with a sound template as the control:

```
=== a GOOD template, as a control ===
PATH SET TO '…/depwork/q17/5.0/tmpl/{blend_name}_'
FRAMEPATH OK '…/depwork/q17/5.0/tmpl/render_0001.png'
EXIT 0
00:00.472  render           | Saved: '…/depwork/q17/5.0/tmpl/tmplok_0001.png'
=== the BAD template ===
PATH SET TO '…/depwork/q17/5.0/tmpl/{blend_name.png'
FRAMEPATH RAISED RuntimeError Error: Parse errors in path '…/depwork/q17/5.0/tmpl/{blend_name.png':
--- single frame ---
EXIT 1
00:00.275  reports          | ERROR Parse errors in path '…/depwork/q17/5.0/tmpl/{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
--- animation ---
EXIT 1
00:00.275  reports          | ERROR Parse errors in path '…/depwork/q17/5.0/tmpl/{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
--- what is in the folder ---
tmplok_0001.png
```

One thing about that control worth recording, because it is a difference in how the test was set up and not a difference in Blender: on this Mac `frame_path` was asked **before** the file had been saved under its new name, so `{blend_name}` still stood for the old name and it answered `render_0001.png`. The file that appeared on disk when the render ran was `tmplok_0001.png`, which is what the template says.

*The recorded path beats one given before the file on the command line.*

```
--- output given BEFORE the file ---
00:00.434  render           | Saved: '…/depwork/q17/5.0/tmplsrc/out/frame_0001.png'
--- output given AFTER the file ---
00:00.433  render           | Saved: '…/depwork/q17/5.0/ord/after/x_0001.png'
--- what landed where ---
…/depwork/q17/5.0/ord/after/x_0001.png
```

*`frame_path()` names the file that appears, for a frame render and for an animation — and not for a still render.* With the output path `shot_{blend_name}_##_`:

```
PATH '…/depwork/q17/5.0/fp2/shot_{blend_name}_##_'
PREDICTED frame 1: …/depwork/q17/5.0/fp2/shot_fp2_01_.png
PREDICTED frame 7: …/depwork/q17/5.0/fp2/shot_fp2_07_.png
--- render frame 7 from the command line ---
Saved: '…/depwork/q17/5.0/fp2/shot_fp2_07_.png'
--- render the animation ---
Saved: '…/depwork/q17/5.0/fp2/shot_fp2_01_.png' … '…/depwork/q17/5.0/fp2/shot_fp2_05_.png'
ON DISK: shot_fp2_01_.png shot_fp2_02_.png shot_fp2_03_.png shot_fp2_04_.png shot_fp2_05_.png shot_fp2_07_.png
```

but through `bpy.ops.render.render(write_still=True)` at frame 7, on the same path:

```
PATH '…/depwork/q17/5.0/fp/shot_{blend_name}_##_'
PREDICTED …/depwork/q17/5.0/fp/shot_render_07_.png
ON DISK ['shot_render_##_.png']
MATCH False
```

*The file-wide path commands reach an ocean cache path and nothing else recorded for writing.* The same six recorded locations were built into one file — an image loaded from disk, the render output path, a compositor File Output node's directory, a point cache with External on, a geometry-nodes bake directory, and an ocean modifier's cache path:

```
TREE ATTR scene.compositing_node_group | FILE OUTPUT NODE PROPERTY directory
--- as saved ---
  image ReadTex   : …/depwork/q17paths/5.0/assets/tex.png
  render output   : …/depwork/q17paths/5.0/renders/frame_
  file output node: …/depwork/q17paths/5.0/fileout
  point cache     : …/depwork/q17paths/5.0/ptcache
  gn bake dir     : …/depwork/q17paths/5.0/gnbake
  ocean cache     : …/depwork/q17paths/5.0/oceanbake
MAKE PATHS RELATIVE -> {'FINISHED'}
--- after Make Paths Relative ---
  image ReadTex   : //assets/tex.png
  render output   : …/depwork/q17paths/5.0/renders/frame_
  file output node: …/depwork/q17paths/5.0/fileout
  point cache     : …/depwork/q17paths/5.0/ptcache
  gn bake dir     : …/depwork/q17paths/5.0/gnbake
  ocean cache     : //oceanbake
MAKE PATHS ABSOLUTE -> {'FINISHED'}
--- after Make Paths Absolute ---
  image ReadTex   : …/depwork/q17paths/5.0/assets/tex.png
  … the other four unchanged …
  ocean cache     : …/depwork/q17paths/5.0/oceanbake
```

With all six pointed at a folder that does not exist, `bpy.ops.file.report_missing_files()` named two of them:

```
Warning: Path '…/depwork/q17paths/5.0/gone' not found, from local data-block 'OBGrid'
Warning: Path '…/depwork/q17paths/5.0/gone/tex.png' not found, from local data-block 'IMReadTex'
REPORT MISSING FILES -> {'FINISHED'}
BLEND PATHS: ['…/depwork/q17paths/5.0/gone', '…/depwork/q17paths/5.0/gone/tex.png', '…/depwork/q17paths/5.0/assets/tex.png']
```

`OBGrid` carries the ocean modifier and `IMReadTex` is the image. The render output path, the File Output node, the point cache and the geometry-nodes bake directory were named by nothing.

One thing about the set-up, recorded because it had to be worked around: on this Mac, **saving the file turned the image path and the ocean cache path into relative ones by itself**, which would have left Make Paths Relative with nothing to do and half the test blind. All six were therefore set back to absolute after the save, and only then were the two commands run. What is quoted above is that second run.

*Cycles' animation denoiser reads the recorded output place and stops on a frame that is not there.* Five frames rendered to multi-layer OpenEXR with denoising data passes, frame 3 deleted, and `bpy.ops.cycles.denoise_animation()` called:

```
MEDIA TYPE SET TO MULTI_LAYER_IMAGE
FILE FORMAT OPEN_EXR_MULTILAYER
RENDERED ['dn.blend', 'f_0001.exr', 'f_0002.exr', 'f_0003.exr', 'f_0004.exr', 'f_0005.exr']
AFTER DELETING FRAME 3 ['dn.blend', 'f_0001.exr', 'f_0002.exr', 'f_0004.exr', 'f_0005.exr']
Error: Frame '…/depwork/denoise/5.0/f_0003.exr' not found, animation must be complete
DENOISE ANIMATION RAISED RuntimeError Error: Frame '…/depwork/denoise/5.0/f_0003.exr' not found, animation must be complete
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

**The path walk visits the same three things it visited on Linux.** The function's own description was printed before it was called, rather than guessed at, and it names three arguments:

```
HAS file_path_foreach: True
HAS file_path_map    : True
   :arg visit_path_fn: function that takes three parameters: the data-block, a file path, and a placeholder for future use.
file_path_foreach RAISED TypeError <lambda>() takes 1 positional argument but 3 were given
FOREACH flags=None visited 3:
    Grid '//oceanbake'
    ReadTex '//assets/tex.png'
    Tex '//assets/tex.png'
FOREACH flags={'EXPAND_CACHES'} RAISED ValueError: flags: 'EXPAND_CACHES' not found in ('SKIP_LINKED', 'SKIP_PACKED', 'RESOLVE_TOKEN', 'SKIP_WEAK_REFERENCES', 'SKIP_MULTIFILE', 'RELOAD_EDITED')
```

(The one-argument call is kept because it is what was typed first, before the description had been read.) The walk reached the image and the ocean cache path and none of the other four recorded write places. The paths read as relative here because the file on disk holds them that way, which is the same three data-blocks by another spelling.

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
00:00.705  render           | Saved: '…/depwork/q17/5.0/wow/frame_0002.png'
00:00.800  render           | Saved: '…/depwork/q17/5.0/wow/frame_0004.png'
```

Only the two missing frames carry a new modification time. **And this version says nothing about it.** With no window it prints `Skipping existing frame` for each of the three; in the window run those lines are not printed at all, though the frames are plainly skipped. The console output was flushed before the process ended, so this is not a lost buffer.

**The Animation Player does play what is at the recorded output place.** With five frames sitting at the recorded place, a script called `bpy.ops.render.play_rendered_anim()` and then listed the running processes:

```
play_rendered_anim() -> {'FINISHED'}
    51664 /Applications/Blender 5.0.1.app/Contents/MacOS/Blender --gpu-backend metal -a -f 24 1.0 -s 1 -e 5 -j 1 -c 4096 …/depwork/q17/5.0/wow/frame_0001.png
```

So the player is a second Blender started with `-a`, and the last thing it is handed is the first frame at the file's own recorded output place. As a control the same was done with the recorded place pointing at an **empty** folder. That control was run once, in Blender 4.5 on this same Mac, and its own words are quoted here; the player was still started at that place and said so:

```
wm_main_playanim_intern: '…/depwork/q17/4.5/emptyout/frame_0001.png' not an image file
```

so the operator reads the recorded place rather than doing something generic.

**The viewport render uses the recorded output place.** With the recorded place an empty folder, a script called `bpy.ops.render.opengl(animation=True)` with the 3D viewport as its context:

```
render output path: '…/depwork/q17/5.0/glout/gl_'
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

**Label: mixed.** Two cases are **documented**. The main case — a physics or geometry-nodes simulation whose cache or bake is not there — is **inferred**, and the sources are close to silent on what a *render* specifically does, as opposed to what playback does. What happens to the render's messages, log or exit is **not in the sources**.

### A. An unbaked light probe volume in EEVEE: the world's light is used instead — inferred

*(Changed by the citation check, 2026-08-06: this heading read "documented". The quoted sentence says "Irradiance Volume" while the page is headed "Light Probe Volume", and nothing quoted says the two names mean the same thing, so a reader has to supply that step. The document's own summary line above, which counts this among "two cases documented", should be read with that in mind.)*

> "If an object is not inside any Irradiance Volume, or if the indirect lighting has not been baked, the world's diffuse lighting will be used to shade it."

Manual, `https://docs.blender.org/manual/en/5.0/render/eevee/light_probes/volume.html`, read 2026-07-30.

This is the clearest answer on the sheet: the render does not stop and does not warn; it falls back to a different source of light, and the picture is simply different.

One qualification, and it matters for the question as worded. This bake is not kept at a place the file records:

> "Light probe volume light data is static and needs to be manually baked. Once baked, the data is stored inside the object data-block and can be moved, animated and linked between blender files."

Same page, same date. So "absent" here means never baked, not a file gone missing.

### B. Unbaked physics and motion blur in Cycles: the render may be wrong — documented

> "If there are particles or other physics system in a scene, be sure to bake them before rendering, otherwise you might not get correct or consistent motion blur."

Manual, `https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/motion_blur.html`, read 2026-07-30.

This is the only statement I found that says outright what a render does differently without a bake. It says the result may be wrong, not that the render fails.

### C. Physics simulations with no bake: the simulation is worked out again — inferred

The sources say the calculation happens whether or not there is a bake:

> "The simulation is only calculated for positive frames in between the Start and End frames of the Cache panel, whether you bake or not."

Manual, `https://docs.blender.org/manual/en/5.0/physics/particles/emitter/cache.html`, read 2026-07-30.

> "The result of a simulation is automatically cached to memory when the animation is played, so that the next time it runs, it can be replayed more quickly by reading the results from the memory."

> "It is generally recommended to bake your physics simulations before rendering. Aside from no longer needing to go through the time-consuming process of simulating again, baking can help prevent potential glitches and ensure that the outcome of the simulation remains exactly the same every time."

> "Note that playing the animation will try to simulate any visible physics simulations."

Manual, `https://docs.blender.org/manual/en/5.0/physics/baking.html`, read 2026-07-30.

And the cache only fills if the frames are visited in order:

> "When an animation is played, each physics system writes each frame to the cache. Note that for the cache to fill up, one has to start the playback before or on the frame that the simulation starts."

Manual, `https://docs.blender.org/manual/en/5.0/physics/particles/emitter/cache.html`, read 2026-07-30.

**The step I took:** the manual says the simulation is calculated whether or not you bake, that baking saves "going through the time-consuming process of simulating again", and that it "can help prevent potential glitches and ensure that the outcome of the simulation remains exactly the same every time". From that I take it that with the bake absent, a render simulates rather than reads, and that the result is not guaranteed to be the same as the baked one. The sources say this about playback and about the general recommendation; **they do not say it about rendering in so many words**, apart from the motion blur note in B. If a reader will not grant that step, the honest answer here is "not in the sources".

The Ocean modifier states the same either/or plainly, though only as a state and not as a consequence:

> "is_cached
>
> Whether the ocean is using cached data or simulating"

API documentation, `https://docs.blender.org/api/5.0/bpy.types.OceanModifier.html`, read 2026-07-30.

### D. Geometry-nodes simulation zones with no bake: frames must be visited in order — inferred

> "The simulation is tied to the animation system, with support for sub-steps. It will only be evaluated while the animation frame changes, and is cached like the existing physics simulations in Blender."

Manual, `https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/simulation/simulation_zone.html`, read 2026-07-30.

> "When the result is ready to be sent to a render-farm, it can be baked to disk. This allows for the simulation to be rendered in a non-sequential order."

Same page, and word for word again in the manual at `https://docs.blender.org/manual/en/5.0/physics/simulation_nodes.html`, both read 2026-07-30.

**The step I took:** if baking to disk is what "allows for the simulation to be rendered in a non-sequential order", then without the bake a render that jumps to a frame out of order does not give the simulated result. The sources state what baking enables; they do not state what happens without it, and they do not say what the render shows instead — an empty result, the first frame, or a wrong one.

### E. A cache that is present but cannot be read: nothing happens, or the playback is wrong — documented

This is not absence, but it is the nearest thing the sources describe outright, and it is new in 5.0:

> "Support for LZO (lite) and LZMA (heavy) compressed disk caches has been dropped in favor of ZSTD compression."

> "Blender will warn users when loading older .blend files using these removed compression methods. End users will need to delete these old caches and rebake them. Loading unsupported point caches and trying to play them back will simply do nothing or produce glitched playback."

Release notes, `https://developer.blender.org/docs/release_notes/5.0/physics/`, read 2026-07-30. The Python side records the matching removal:

> "The `PointCache.compression` property was removed; caches are always compressed now."

Release notes, `https://developer.blender.org/docs/release_notes/5.0/python_api/`, read 2026-07-30.

Two things worth holding on to. There is a warning, but it is on **loading the file**, not on rendering. And the described outcome — "simply do nothing or produce glitched playback" — is silent failure, not a stop.

### F. Asking for a frame outside a bake's range: the ends are repeated — documented

> "Frame Start, End
>
> Frames of the simulation to bake (inclusive). The start and end frames of the bake are repeated when accessing frames outside of the baked range."

Manual, Ocean modifier, `https://docs.blender.org/manual/en/5.0/modeling/modifiers/physics/ocean.html`, read 2026-07-30.

This answers a near neighbour of the question — a bake that exists but does not reach the frame being rendered — and not the question itself.

### G. What the sources do not say

I found **no statement anywhere** about what a render does when:

- a disk point cache folder (`blendcache_[filename]`) has been deleted or is empty;
- an External cache path points at files that are not there;
- a fluid domain's Cache Directory is empty or gone;
- a geometry-nodes bake directory is empty or gone;
- an Ocean modifier's Cache Path holds no baked EXR files at all.

Nothing in the manual, the API documentation or the 5.0 release notes says whether the render stops, warns, writes anything to its log, changes its exit, or carries on quietly. **Not in the sources**, and I would treat that as the working answer for the render-farm case until somebody checks it by hand (see the list at the end).

Two nearby mechanisms exist but are not stated to cover caches or bakes, so I ruled them out rather than borrowing them:

> "Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files."

Manual, `https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html`, read 2026-07-30. It speaks of "links to unpacked files"; nothing says a cache or bake directory is one of them, and it is something a person triggers from a menu.

> "Show Missing Media
>
> Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent."

Manual, `https://docs.blender.org/manual/en/5.0/editors/video_sequencer/preview/sidebar.html`, read 2026-07-30. This is a real documented render behaviour for absent files — magenta or transparent — but it is about video sequencer strip media, not about a cache or a bake.

For completeness, the same shape of question for a volume file sequence has a documented answer, again for out-of-range rather than absent, and again for a read reference rather than a bake: "Mode — How the volume should behave before the sequence's first frame (Start) and after its last (Start + Frames). Clip: Show nothing." (manual, `https://docs.blender.org/manual/en/5.0/modeling/volumes/properties.html`, read 2026-07-30).

### Not covered

- Whether a render aborts, warns, or continues silently when a recorded cache or bake location is empty. This is the centre of the question and it is unanswered.
- Whether anything appears in the render's log or on its output stream naming the missing location.
- Whether a frame rendered without its bake comes out the same every time, or differs run to run. The manual only says baking "can help prevent potential glitches and ensure that the outcome of the simulation remains exactly the same every time", which implies but does not state the opposite.
- Whether the answer differs by simulation type. I found separate statements for physics point caches, geometry-nodes simulations, ocean and EEVEE light probes, and they do not agree with one another closely enough to generalise.
- Whether the 5.0 warning about unsupported point cache compression appears when the file is opened without a window.

### Outside the interface

**Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** 

This is where the gap hurts most, and the sources will not close it.

Everything the manual says about caches filling and simulations being worked out is phrased around **playing the animation**: "when the animation is played", "playing the animation will try to simulate", "one has to start the playback before or on the frame that the simulation starts". None of it is phrased around rendering, and none of it is phrased around Blender being driven from outside its window.

The one statement that points at driving from outside is the render-farm one: baking to disk "allows for the simulation to be rendered in a non-sequential order" (manual, `https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/simulation/simulation_zone.html`, read 2026-07-30). Taken at face value it says a job split across frames needs the bake. It does not say what a frame rendered without it looks like.

The 5.0 warning about unsupported point caches is described as appearing "when loading older .blend files" (release notes, `https://developer.blender.org/docs/release_notes/5.0/physics/`). The sources do not say where that warning goes when nobody is looking at a window.

**The sources do not say** whether any of A to F holds, differs, or is reported differently when Blender is started and driven from outside. For A (the light probe fallback) I would expect no difference, since it is a shading decision and not a message, but the sources do not state it and I am not going to guess.

### Operating system

**Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** 

**The sources do not say.** Nothing in the 5.0 manual, API documentation or release notes ties any of this to a particular operating system. The only operating-system-dependent statement I found anywhere near caches is about writing rather than reading, and about a path being too long: "If you are not allowed to write to the required subdirectory caching will not take place, e.g. if your blend-file path is very long and your operating system has a limit on the path length that is supported." (manual, `https://docs.blender.org/manual/en/5.0/physics/particles/emitter/cache.html`, read 2026-07-30). That describes a cache never being written, which would then be absent later — a route into this question, but not an answer to it.

### Where I looked

Searched the whole 5.0 manual and the whole 5.0 API documentation for: `not baked`, `missing cache`, `cache is missing`, `no cache`, `cache does not`, `if the cache`, `when the cache`, `invalid cache`, `resimulate`, `re-simulate`, `recalculate`, `outdated`, `missing`, `not found`, `does not exist`, `must be baked`, `before rendering`, `not been baked`, `without baking`, `unbaked`, `render farm`, `render-farm`, `non-sequential`, `out of order`, `sequential`, `independently`, `pass through`, `pass-through`, `no bake`, `is_cached`, `has_cache`, `is_outdated`, `blendcache`.

Pages opened in full: Baking Physics Simulations; Particle Cache; Cloth Cache; Soft Body Cache; Fluid Domain Cache; Simulation Nodes; Simulation Zone; Geometry Nodes Baking; Bake Node; Geometry Nodes Modifier; Ocean Modifier; Light Probes index; Light Probe Volume; Cycles Motion Blur; Volume Properties; Dope Sheet navigating (cache display); Topbar; Video Sequencer preview sidebar; Command Line Arguments; Rendering From The Command Line. API pages opened: `PointCache`, `PointCacheItem` (through search), `OceanModifier`, `FluidDomainSettings`, `NodesModifier`, `NodesModifierBake`, `bpy.ops.object` (bake and cache operators). Release notes read in full: `physics.md`; searched across all 5.0 release note subjects for `cache` and `bake`, and read the relevant parts of `eevee.md`, `python_api.md`, `rendering.md` and `core.md`.

Ruled out and why: the Sequencer's Cache panel is memory-only and cannot be absent from disk. `Report Missing Files` and `Show Missing Media` are discussed above and are not stated to cover caches or bakes. Cycles' own files were searched (`properties.py`, `ui.py`, `engine.py`, `operators.py`, `osl.py`, `presets.py`, `camera.py`, `version_update.py`, `__init__.py`) for `cache` and `bake`; the only cache mentions are Cycles' own tile-to-disk memory saving and its kernel cache — "If optimized kernels are not available, rendering will proceed using generic kernels until the optimized set is available in the cache" (`properties.py`, in the `kernel_optimization_level` property description, **observed** in the file, read 2026-07-30). That is a genuine "cache absent, render carries on regardless" statement, but the kernel cache is not something a `.blend` records, so it does not answer this question. I mention it only so the search is not repeated.

---

**Observed — the hand tests for this round, run 2026-08-07 on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and 118 GB of free disk, and no display server at all.** Blender 5.0 was started from a terminal with no window, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and the Blender answered `RESOURCES /root/dependencies/res-5.0`. Every render used Cycles on the processor, so nothing here depends on the graphics card. The machine was shared with other topics' work throughout, so nothing here is a timing or memory measurement of a clean machine except where it says what it did about that. The full list of tests, with every set-up written out, is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

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
use_external True | filepath /root/dependencies/work/q18/5.0/emptycache | name 'SmoozyCache'
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
BEFORE BAKE | cache_directory: /root/dependencies/work/q18fluid/5.0/fluidcache | has_cache_baked_any: False | has_cache_baked_data: False
BAKE ALL -> {'FINISHED'}
AFTER BAKE  | cache_directory: /root/dependencies/work/q18fluid/5.0/fluidcache | has_cache_baked_any: True | has_cache_baked_data: True
cache folder: ['config', 'data']
cache folder removed; exists now: False
SAME SESSION, FOLDER GONE | has_cache_baked_any: True | has_cache_baked_data: True
REOPENED with the cache folder gone (folder exists: True )
  has_cache_baked_any: True | has_cache_baked_data: True | has_cache_baked_noise: True | has_cache_baked_mesh: True | has_cache_baked_particles: True
```

and what the reopening had put back on disk:

```
/root/dependencies/work/q18fluid/5.0/fluidcache/config/config_0001.uni
/root/dependencies/work/q18fluid/5.0/fluidcache/data/fluid_data_0001.vdb
```

**An EEVEE light probe volume with no bake does render differently.** A closed room with its faces turned inward so the world cannot light it directly, a sphere inside, a light probe volume covering it, and a strong green world. Rendered unbaked, then with `bpy.ops.object.lightprobe_cache_bake(subset='ALL')`, then after `bpy.ops.object.lightprobe_cache_free(subset='ALL')`:

```
ENGINE BLENDER_EEVEE
  unbaked against baked: largest channel difference=0.796078 channels differing=24796 of 57600 — DIFFERENT PICTURE
  baked against freed  : largest channel difference=0.796078 channels differing=24796 of 57600 — DIFFERENT PICTURE
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

**This version has no texture cache to test.** `scene.render.use_texture_cache` does not exist in it, nor `use_auto_generate_texture_cache`, nor the `texture_cache_directory` preference, and `generate_texture_cache` is not in `dir(bpy.ops.render)`. The same is true of 4.5 and 5.1; 5.2 has all four. So the texture cache is a real difference between the versions and not a gap in one version's documentation.

**A disk point cache written by 4.5 with a compression this version dropped is announced.** The cloth file was baked in Blender 4.5 with `point_cache.compression = 'LIGHT'` and Disk Cache on, and the same saved file was then opened and rendered here:

```
EXIT 0
is_baked True | is_outdated False | is_frame_skip False | use_disk_cache True | info '60 frames on disk.'
Warning: Cloth Cache in object Grid can not be read because it uses an outdated compression method. You need to delete the caches and re-bake.
render EXIT 0
00:00.270  reports          | WARNING Cloth Cache in object Grid can not be read because it uses an outdated compression method. You need to delete the caches and re-bake.
00:00.465  render           | Saved: '/root/dependencies/work/oldcache/from5.0_0060.png'
PICTURES size=(160, 160) largest channel difference=1.000000 channels differing=4899 of 102400
DIFFERENT PICTURE
```

This is the only case in all of these tests where a cache that cannot be used is named on the console of a run with no window. The exit status is still 0, `is_baked` still reads `True`, and the picture differs from the one 4.5 renders with the same cache. 5.1 and 5.2 printed the same words. `PointCache.compression` exists in 4.5 with the choices `NO`, `LIGHT` and `HEAVY`, and does not exist in this version at all.

**Observed — the Mac half of this round's hand tests, run 2026-08-07 on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card.** Blender 5.0.1 was started from a terminal, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and this Blender answered `RESOURCES …/depwork/res-5.0` and `VERSION 5.0.1`. The application used was `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`. Every render used Cycles on the processor. **Other topics were running Blender on this same Mac throughout, so nothing was timed and no memory was measured; there is no timing or memory reading anywhere below.** **This is an addition to a document written earlier in this round; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

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
use_external True | filepath …/depwork/q18/5.0/emptycache | name 'SmoozyCache'
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
…/depwork/q18fluid/5.0/fluidcache/config/config_0001.uni
…/depwork/q18fluid/5.0/fluidcache/data/fluid_data_0001.vdb
```

*An EEVEE light probe volume with no bake does render differently, and freeing the bake puts the render back exactly.*

```
ENGINE BLENDER_EEVEE | EEVEE names offered: ['BLENDER_EEVEE']
  unbaked against baked: largest channel difference=0.815686 channels differing=37040 of 57600 — DIFFERENT PICTURE
  baked against freed  : largest channel difference=0.815686 channels differing=37040 of 57600 — DIFFERENT PICTURE
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
with the cache present  average colour r=0.143 g=0.152 b=0.241
with the folder deleted average colour r=0.138 g=0.146 b=0.167
the two strips compared as pictures: largest channel difference=0.349020 channels differing=24193 of 112000 — DIFFERENT PICTURE
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

2026-08-06 — Opus 5 (Claude Code) — Dependencies, Blender 5.0
