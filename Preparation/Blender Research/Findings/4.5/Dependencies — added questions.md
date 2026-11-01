# Dependencies — added questions (17 and 18), Blender 4.5

**Sources consulted, and when.** Blender 4.5 manual, Blender 4.5 Python API documentation, Blender 4.5 release notes, and Cycles' own files as shipped with 4.5 — all read on 2026-07-30. Blender's requirements page as taken on 2026-08-01 (it names no version relevant to these two questions, so nothing here rests on it).

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

I answered only questions 17 and 18. I did not run Blender; nothing below is labelled from something I ran.

---

## 17. What use does a `.blend` file make of the places it records for writing, besides writing to them?

**Ambiguity, flagged.** I read the question as: a `.blend` records a number of file paths that Blender writes files to — what else does Blender do with those recorded paths? A second reading is possible: what use the file's own data makes of those recorded paths internally. I answered the first reading. For the second, I found nothing in the sources describing anything of that kind; the searches and pages recorded at the end of this answer are the whole of what I looked at for it. **LABEL for the second reading: not in the sources.**

### First, which places a `.blend` records for writing

These are the write locations I found recorded in scene, node and modifier data. Whether Preferences also holds write locations, and whether any of those travel with the file, is not something I established from the sources, and nothing below rests on it.

- The scene's render output path. "Directory/name to save animations, # characters define the position and padding of frame numbers" — `RenderSettings.filepath`, Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30.
- The compositor File Output node's base path, and its per-slot names. "Base output path for the image" — `CompositorNodeOutputFile.base_path`, Python API, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html, read 2026-07-30.
- Physics point-cache locations. "When Disk Cache is checked, Blender will save the cache separately to the drive in a folder named blendcache_[filename] alongside the blend-file." and "Path — Select the directory path to the cache files." — manual, https://docs.blender.org/manual/en/4.5/physics/baking.html, read 2026-07-30.
- The fluid domain's cache directory. "Directory that contains fluid cache files" — `FluidDomainSettings.cache_directory`, Python API, https://docs.blender.org/api/4.5/bpy.types.FluidDomainSettings.html, read 2026-07-30.
- The Geometry Nodes modifier's bake directory. "Location on disk where the bake data is stored" — `NodesModifier.bake_directory`, Python API, https://docs.blender.org/api/4.5/bpy.types.NodesModifier.html, read 2026-07-30.
- The Ocean modifier's bake folder. "Path to a folder to store external baked images" — `OceanModifier.filepath`, Python API, https://docs.blender.org/api/4.5/bpy.types.OceanModifier.html, read 2026-07-30.
- Dynamic Paint's image-sequence output folder. "Directory to save the textures" — `DynamicPaintSurface.image_output_path`, Python API, https://docs.blender.org/api/4.5/bpy.types.DynamicPaintSurface.html, read 2026-07-30.
- The scene's texture-bake external image path. "Image filepath to use when saving externally" — `BakeSettings.filepath`, Python API, https://docs.blender.org/api/4.5/bpy.types.BakeSettings.html, read 2026-07-30.

### Use A — the same place is read back from

**LABEL: documented for the first three below. The fourth is inferred, and says so where it sits.**

Ocean modifier:

> "Rather than simulating the ocean data live, it can be baked to files in a given directory. When a simulation is baked, the simulator engine is completely bypassed, and all information for the modifier or texture is retrieved from the baked files."

and

> "Upon loading the data from these baked files, when a frame of the bake sequence is read, it is cached in memory. This means that accessing loaded frames subsequent times is fast, not incurring the overhead of drive access."

Source: manual, Ocean Modifier, https://docs.blender.org/manual/en/4.5/modeling/modifiers/physics/ocean.html — read 2026-07-30.

Fluid domain cache directory:

> "Offset — Frame offset that is used when loading the simulation from the cache. It is not considered when baking the simulation, only when loading it."

Source: manual, Fluid Domain Cache, https://docs.blender.org/manual/en/4.5/physics/fluid/type/domain/cache.html — read 2026-07-30.

Point cache:

> "External — Allows you to read the cache from a drive using a user-specified file path."

Source: manual, Baking Physics Simulations, https://docs.blender.org/manual/en/4.5/physics/baking.html — read 2026-07-30.

Geometry Nodes bake path — **LABEL for this one: inferred**, unlike the three above.

> "The Bake node allows saving and loading intermediate geometries."

Source: manual, Bake Node, https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/geometry/operations/bake.html — read 2026-07-30.

The step a reader has to supply: that quote says the Bake node saves and loads, and the modifier property quoted at the top of this answer says the bake directory is "Location on disk where the bake data is stored". Neither says that the loading reads from that recorded directory. Joining the two is my step, not the sources'.

### Use B — the place is inspected for what is already there, and that decides whether a frame is written

**LABEL: documented that already-written frames are honoured; inferred that this means the recorded place is examined.**

The manual's Output panel:

> "Overwrite — Overwrite existing files when rendering."

> "Placeholders — Create empty placeholder frames while rendering."

and immediately after, under the heading "Primitive Render Farm":

> "Set up a shared directory over a network file system."
> "Disable Overwrite, enable Placeholders in the Render Output panel."
> "Start as many machines as you wish rendering to that directory."

Source: manual, Output, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30.

The picture on that page, `_images/render_output_properties_output_panel.png`, shows the panel with the path field reading `/tmp/`, Overwrite ticked, Placeholders unticked, File Extensions ticked and Cache Result unticked. LABEL for that: **documented** (it is part of the manual). Nothing beyond what is visible is claimed from it.

The Python API restates both, and adds what a placeholder is:

> "use_overwrite — Overwrite existing files while rendering" (default True)

> "use_placeholder — Create empty placeholder files while rendering frames (similar to Unix 'touch')" (default False)

Source: Python API, RenderSettings, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.

The manual states the resulting behaviour plainly:

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."
> "Just disable the Overwrite option to start rendering where you left off."

and, for rendering every Nth frame first:

> "Then disable Overwrite and re-render with Frame Step set to 1."

Source: manual, Rendering Animations, https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30.

The inferred step: for Blender to "start rendering where you left off", and for several machines to divide work by writing placeholders into one shared directory, Blender must look at the recorded output location and test what is already there before writing each frame. The sources never say this in so many words; they only state the outcome.

### Use C — the place is played back from

**LABEL: documented.**

> "Launching the animation player opens a new window, playing back images or a video located at the render output of the current scene."

Source: manual, Animation Player, https://docs.blender.org/manual/en/4.5/render/output/animation_player.html — read 2026-07-30.

Restated on the same subject:

> "Rendered image sequences can be played back in the Animation Player."

Source: manual, Render Output Introduction, https://docs.blender.org/manual/en/4.5/render/output/introduction.html — read 2026-07-30.

The matching operator: "bpy.ops.render.play_rendered_anim() — Play back rendered frames/movies using an external player" — Python API, https://docs.blender.org/api/4.5/bpy.ops.render.html, read 2026-07-30.

### Use D — the place is computed and handed back as a string, without anything being written

**LABEL: documented** that a path is computed and handed back; **inferred** that nothing is written when it is. *(Changed by the citation check, 2026-08-06: the label read "documented" for the whole of this. The quoted words say a path is returned; they do not say that nothing is written, so a reader has to supply that step.)*

> "frame_path(*, frame=-2147483648, preview=False, view='') — Return the absolute path to the filename to be written for a given frame"

with "Returns: File Path, The resulting filepath from the scenes render settings".

Source: Python API, RenderSettings, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.

### Use E — the place is parsed and validated, and its errors are reported

**LABEL: documented.**

Two of these recorded write locations carry a small template language, which Blender parses:

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

Validation and reporting:

> "When there are template errors in a path, the path field will be highlighted red in the UI"

> "Hovering over the path field will pop up a tooltip that contains a list of the template errors encountered in that path"

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

Source: manual, File Paths, https://docs.blender.org/manual/en/4.5/files/file_paths.html — read 2026-07-30.

The release notes state this as a change in this version:

> "Some file paths in Blender can now contain "template expressions" like `{blend_name}` and `{resolution_x}`, which get replaced by the values they reference when the path is used."
> "Currently this is only enabled on the Scene's render output path and the output paths of the compositor's File Output node."

Source: release notes, Core, https://developer.blender.org/docs/release_notes/4.5/core/ — read 2026-07-30.

The API marks the two properties accordingly: `RenderSettings.filepath` is "string, default "", (never None, blend relative // prefix supported, Supports template expressions)" and `CompositorNodeOutputFile.base_path` carries the same note. Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html and https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30.

### Use F — the place is expanded against the blend-file's own folder

**LABEL: documented.**

> "This setting expands Relative Paths where a // prefix represents the directory of the current blend-file."

Source: manual, Output, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30. The quoted sentence begins "This setting", and on that page it sits under the heading **File Path** — "Choose the location to save rendered frames." — so the setting it belongs to is the recorded render output path. *(Changed by the citation check, 2026-08-06: the finding said it had not carried across which control the sentence belongs to, which left the claim resting on nothing. Reading the page settles it; nothing beyond the page has been supplied.)*

The API carries the note "blend relative // prefix supported" on the recorded write paths listed at the top of this answer. I quoted it only for `RenderSettings.filepath` and `CompositorNodeOutputFile.base_path` (addresses above), so the wider claim is unquoted.

This is arguably part of writing rather than besides it; I include it because the expansion is a use of the recorded text that happens before any file is touched.

### Use G — the recorded value is restored over anything set before the file was loaded

**LABEL: documented.**

> "blender --background --render-output /tmp test.blend --render-frame 1"
> "…will not render to /tmp because loading the blend-file overwrites the render output that was set."

Source: manual, Command Line Arguments, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

### What I looked for and did not find

**LABEL for the following: not in the sources.**

- Whether the render output path (or any other recorded write path) is touched by File ‣ External Data ‣ Make Paths Relative / Make Paths Absolute, or is scanned by Report Missing Files / Find Missing Files. The manual describes those four only in terms of "external files" and "links to unpacked files that no longer exist" — manual, Topbar, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html, read 2026-07-30. It never names output or cache paths there, and never excludes them either.
- Whether Blender creates a missing output directory when rendering. The only "create the folder" statement I found anywhere is the Ocean modifier's: "If the cache path folder does not exist, it will be created." (manual, Ocean Modifier, address above). Nothing equivalent is stated for the render output path or the File Output node.
- Whether the existence check behind Overwrite is a per-file test or a directory listing, and what happens when two machines test the same frame at the same moment.

**Where I looked, and with what terms.** I searched the whole of the 4.5 manual and the whole of the 4.5 API text for: `placeholder`, `overwrite`, `frame_path`, `output path`, `render path`, `render output`, `render cache`, `cached render`, `path template`, `Path Templates`, `blendcache`, `cache directory`, `Cache Path`, `bake directory`, `Bake Path`, `image_output_path`, `cache_directory`, `bake_directory`, `is_cached`, `filepath`, `Make Paths Relative`, `Make Paths Absolute`, `Report Missing Files`, `Find Missing Files`, `does not exist`, `not found`, `will be created`, `play_rendered_anim`, `Animation Player`. I read in full: the Output properties page, Rendering Animations, Render Output Introduction, Animation Player, File Paths (path templates), Command Line Arguments (render and playback sections), Topbar (External Data), Preferences ‣ File Paths, the File Output node page, Baking Physics Simulations, the particle/cloth/soft-body cache pages, the Fluid Domain Cache page, the Ocean modifier page, the Bake node page, the Geometry Nodes modifier page. In the API I read `bpy.types.RenderSettings`, `bpy.types.CompositorNodeOutputFile`, `bpy.types.PointCache`, `bpy.types.OceanModifier`, `bpy.types.NodesModifier`, `bpy.types.BakeSettings`, `bpy.ops.render`. I searched all of Cycles' own 4.5 files for `cache`, `bake`, `filepath`, `directory`, `path`; Cycles exposes no output or cache path of its own (see the note under question 18 about its kernel cache), so it contributed nothing to this answer. I searched all 4.5 release notes for `cache`, `bake`, `path template`.

**Not covered.** This answer lists the uses the sources describe; it is not a proof that no other use exists. It does not settle what Blender does with a recorded write path that is unreachable or unwritable at the moment it is used, nor the timing of any existence check relative to the render of the frame, nor whether any of these paths appear in the file's dependency-management tools.

**Outside the interface.** **Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** Mostly yes, and one part no.
- The render output path can be set from the command line: "-o, --render-output <path> — Set the render path and file name." and "You can use path templating features such as {blend_name} in the path." — manual, Command Line Arguments, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30. So use E and use F hold there. **documented.**
- Loading the blend-file overwrites a render output set earlier on the command line, quoted under use G. **documented.**
- Overwrite and Placeholders are scene properties, and the manual's own render-farm recipe is several machines rendering into a shared directory. The sources do not say that this requires the interface. Whether background renders honour them is not stated in so many words — **inferred** from them being ordinary scene properties plus the render-farm hint.
- The template error reporting in use E is split: the red field and the hover tooltip are interface things by their own description; the sentence "rendering an animation will fail with an error message indicating the path errors" names no interface. **documented** as written; where that error message is delivered in a background run is not stated.
- The Animation Player has a command-line form: "-a <options> <file(s)> — Instead of showing Blender's user interface, this runs Blender as an animation player, to view movies and image sequences rendered in Blender (ignored if -b is set)." — same page. Note the parenthesis: it is ignored when Blender is in background mode. **documented.**
- `frame_path()` is a Python call and carries no interface requirement. **documented** to that extent.

**Operating system.** **Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** The sources do not say. Nothing on any of the pages above is qualified by operating system. The only mention of one is descriptive rather than limiting — the API's "similar to Unix 'touch'" for placeholders — and the particle cache page's warning that writing may not happen at all on some systems: "If you are not allowed to write to the required subdirectory caching will not take place, e.g. if your blend-file path is very long and your operating system has a limit on the path length that is supported." (manual, Particle Cache, https://docs.blender.org/manual/en/4.5/physics/particles/emitter/cache.html, read 2026-07-30). That is about writing, not about the other uses. Beyond it, guessing.

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

**Ambiguity, flagged.** "Absent" could mean never made, or made and then the files removed. The sources do not distinguish the two anywhere I found, so I answer both together and say where that matters.

**LABEL, overall: mixed — documented for two specific cases, inferred for two more, and not in the sources for the general case.** Taken case by case:

### Case 1 — EEVEE light probe volume with no bake: documented

> "If an object is not inside any Irradiance Volume, or if the indirect lighting has not been baked, the world's diffuse lighting will be used to shade it."

Source: manual, Light Probe Volume, https://docs.blender.org/manual/en/4.5/render/eevee/light_probes/volume.html — read 2026-07-30. **LABEL: inferred.** *(Changed by the citation check, 2026-08-06: the label read "documented". The quoted sentence says "Irradiance Volume" while the page is headed "Light Probe Volume", and nothing quoted says the two names mean the same thing, so a reader has to supply that step.)*

The render therefore proceeds and substitutes the world's diffuse lighting. The page states no error, no warning and no refusal to render. Note also, from the same page, that this particular bake is not an external file at all:

> "Light probe volume light data is static and needs to be manually baked. Once baked, the data is stored inside the object data-block and can be moved, animated and linked between blender files."

### Case 2 — Ocean modifier, frames outside the baked range: documented

> "The start and end frames of the bake are repeated when accessing frames outside of the baked range."

Source: manual, Ocean Modifier, https://docs.blender.org/manual/en/4.5/modeling/modifiers/physics/ocean.html — read 2026-07-30. **LABEL: documented.**

So a partial bake does not fail; the nearest baked end frame is used again. The page does not say what happens when the whole bake is gone.

### Case 3 — Ocean modifier with no bake at all: inferred

The manual sets bake and live simulation against each other:

> "Rather than simulating the ocean data live, it can be baked to files in a given directory. When a simulation is baked, the simulator engine is completely bypassed, and all information for the modifier or texture is retrieved from the baked files."

(same page, same date), and the API exposes the state as one or the other:

> "is_cached — Whether the ocean is using cached data or simulating"

Source: Python API, OceanModifier, https://docs.blender.org/api/4.5/bpy.types.OceanModifier.html — read 2026-07-30.

**LABEL: inferred.** The step a reader has to supply: the sources describe exactly two states, "using cached data" and "simulating", and say the solver is bypassed only when baked; from that I conclude that with no bake present the solver runs and the render gets a live-simulated ocean. Neither source says this for the case where the bake files were recorded and then removed.

### Case 4 — Geometry Nodes simulation zones with no bake: inferred

> "The simulation is automatically cached during playback."

> "When the result is ready to be sent to a render-farm, it can be baked to disk. This allows for the simulation to be rendered in a non-sequential order."

Source: manual, Simulation Zone, https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/simulation/simulation_zone.html — read 2026-07-30; the same two sentences appear in manual, Simulation Nodes, https://docs.blender.org/manual/en/4.5/physics/simulation_nodes.html — read 2026-07-30.

**LABEL: inferred.** The step: if baking to disk is what *allows* non-sequential rendering, then without the bake the render is not free to take frames out of order and still get the simulated result. The sources do not say what actually comes out of a single out-of-order frame rendered with no bake — whether it is the start state, an empty result, or something else. They say only that baking is what makes the non-sequential case work.

### Case 5 — the general case, and the physics point caches: not in the sources

For particle systems, cloth, soft bodies, rigid bodies, Dynamic Paint and fluid domains, I could not find any statement of what a render does when the recorded cache or bake is absent. The nearest the sources get is advice, which states the consequence of not baking rather than the behaviour of a missing bake:

> "It is generally recommended to bake your physics simulations before rendering. Aside from no longer needing to go through the time-consuming process of simulating again, baking can help prevent potential glitches and ensure that the outcome of the simulation remains exactly the same every time."

Source: manual, Baking Physics Simulations, https://docs.blender.org/manual/en/4.5/physics/baking.html — read 2026-07-30.

and, about filling a cache at all:

> "When an animation is played, each physics system writes each frame to the cache. Note that for the cache to fill up, one has to start the playback before or on the frame that the simulation starts."

> "The simulation is only calculated for positive frames in between the Start and End frames of the Cache panel, whether you bake or not."

Source: manual, Particle Cache, https://docs.blender.org/manual/en/4.5/physics/particles/emitter/cache.html — read 2026-07-30. Both sentences are about playback, not about rendering; carrying them across to a render is a step the sources do not take.

**LABEL: not in the sources**, with the record of looking below.

Two further things I can state and cannot:

- There is a status string. "info — Info on current cache status", type "string, default "", (readonly, never None)" — Python API, PointCache, https://docs.blender.org/api/4.5/bpy.types.PointCache.html, read 2026-07-30. Also `is_baked`, `is_baking`, `is_frame_skip` ("Some frames were skipped while baking/saving that cache") and `is_outdated` (no description given). What any of these read when the recorded cache files are missing is not documented. **LABEL: documented** that the properties exist; **not in the sources** for what they say in this situation.
- Cache state is shown in the Timeline: "Baked simulations will be shown as fully opaque, cached simulations will be slightly transparent, and invalid caches will be slightly transparent with dark diagonal stripes." — manual, Timeline, https://docs.blender.org/manual/en/4.5/editors/timeline.html, read 2026-07-30. That is a drawing in the interface, and says nothing about what a render does. **LABEL: documented**, but it does not answer the question.

I found no documented error message, warning, log line or non-zero exit for an absent cache or bake anywhere in the 4.5 manual, API documentation or release notes.

### One thing that is nearby and is not this

Cycles keeps a cache of its own that a `.blend` does not record, and it does describe what happens when it is absent:

> "Kernels can be optimized based on scene content. Optimized kernels are requested at the start of a render. If optimized kernels are not available, rendering will proceed using generic kernels until the optimized set is available in the cache. This can result in additional CPU usage for a brief time (tens of seconds)"

Source: Cycles' own files, `properties.py`, in the `kernel_optimization_level` property definition of the Cycles preferences class (the `EnumProperty` whose items are `OFF`, `INTERSECT`, `FULL`) — read 2026-07-30. **LABEL: observed** (this is what the file says; I did not run it). I record it only to mark the boundary: this cache is not one the file records, so it is not an answer to the question as asked.

### Where I looked, and with what terms

Across the whole 4.5 manual and the whole 4.5 API text I searched for: `not baked`, `is not baked`, `isn't baked`, `unbaked`, `has not been baked`, `not been baked`, `without baking`, `no cache`, `missing cache`, `cache is missing`, `missing`, `does not exist`, `doesn't exist`, `not found`, `cannot be found`, `could not be found`, `invalid cache`, `cache invalid`, `outdated`, `stale`, `fall back`, `fallback`, `falls back`, `recalculat`, `recomput`, `re-simulat`, `resimulat`, `non-sequential`, `sequentially`, `magenta`, `pink`, `blendcache`, `render farm`, `before rendering`, `when rendering`, `warning`, `error`. In the 4.5 release notes I searched for `bake` and `cache` — the only hits are a fixed OpenVDB import bug, a list of fixed Bake-node and Geometry-Nodes-bake bugs, one EEVEE light probe bake fix, "Cache sampler output", and "Simplified Cache settings" in the sequencer; none states what a render does with an absent cache. In Cycles' own 4.5 files I searched for `cache`, `bake`, `filepath`, `directory`, `path`; the only cache described is the kernel cache quoted above, and the tile cache ("Tiles are cached to disk while rendering to save memory", `properties.py`, `use_auto_tile` description), neither of which is recorded in a `.blend`.

I read in full: Baking Physics Simulations; the Particle, Cloth and Soft Body cache pages; Fluid Domain Cache; the Ocean modifier page; Simulation Nodes; Simulation Zone; the Bake node page; the Geometry Nodes modifier page (Manage ‣ Bake); EEVEE Light Probe Volume; the Timeline cache section; Dynamic Paint Canvas (Cache and Output panels); Mesh Cache and Mesh Sequence Cache modifier pages. In the API I read `bpy.types.PointCache`, `bpy.types.OceanModifier`, `bpy.types.NodesModifier`, `bpy.types.FluidDomainSettings`, `bpy.types.DynamicPaintSurface`, `bpy.types.BakeSettings`, and the bake operators in `bpy.ops.object` (`bake`, `geometry_node_bake_single`, `geometry_node_bake_delete_single`, `geometry_node_bake_pack_single`, `geometry_node_bake_unpack_single`, `lightprobe_cache_bake`, `ocean_bake`). I looked at three manual pictures: `_images/physics_baking_multi-cache-interface.png` (shows File Format, Compression, a two-entry cache list, Use Library Path ticked, External unticked, Simulation Start 1, End 250 — no status line visible), `_images/modeling_geometry-nodes_simulation_baking.png` (shows Calculate to Frame, Bake with a trash icon, Cache ticked — no status line visible), and `_images/render_output_properties_output_panel.png` (used for question 17). None of the three shows any indication of a missing cache.

**Not covered.** This answer does not settle: what a render does when a point cache, a fluid cache, a Dynamic Paint image sequence, or a Geometry Nodes disk bake that the file records has been deleted; whether such a render fails, warns, or silently substitutes something; whether anything is written to the console or to the process exit status; whether the answer differs between "never baked" and "baked then deleted"; and whether a partially present cache (some frames only) behaves like the Ocean modifier's documented repeat-the-end-frame rule or differently. It also does not cover caches read through the Mesh Cache or Mesh Sequence Cache modifiers, for which the sources describe the file path but never the missing-file case.

**Outside the interface.** **Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** The sources do not say, for any of the five cases. Two remarks I can make from what is written:
- The Timeline's opaque / transparent / striped cache drawing is an editor feature by its own description, so it cannot be a source of this information in a run with no window. **documented** that it is an editor feature; **not in the sources** for what replaces it.
- The Geometry Nodes statement "When the result is ready to be sent to a render-farm, it can be baked to disk. This allows for the simulation to be rendered in a non-sequential order." is the only sentence in the sources that connects baking to rendering away from a person's own session, and it names no interface. Whether an unbaked simulation zone behaves differently in a background render than in the interface is not stated. **not in the sources.**

**Operating system.** **Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** The sources do not say. None of the pages above qualifies any of this by operating system. The requirements page (https://www.blender.org/download/requirements/, read 2026-08-01) speaks only to what hardware and systems a release supports and says nothing about caches or bakes.

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

2026-08-06 — Opus 5 (Claude Code) — Dependencies (added questions 17 and 18), Blender 4.5
