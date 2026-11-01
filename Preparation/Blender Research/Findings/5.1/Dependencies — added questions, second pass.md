# Dependencies — added questions, second pass — Blender 5.1

Sources consulted, and when:

- Blender 5.1 manual (plain-text copy), read 2026-07-30.
- Blender 5.1 Python API documentation (plain-text copy), read 2026-07-30.
- Blender 5.1 release notes, read 2026-07-30.
- Cycles' own files for 5.1, read 2026-07-30.
- Blender's requirements page, taken 2026-08-01, read 2026-08-01. It names "Blender 5.0 and later", which is the only place it speaks for 5.1.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

Scope: questions 17 and 18 only. The rest of the sheet was not touched.

---

## 17. What use does a `.blend` file make of the places it records for writing, besides writing to them?

**Reading of the question.** I read "the places it records for writing" as the output, cache and bake locations stored inside the file — the render output path, the compositor File Output node's directory, physics cache paths, the geometry-node bake path, the ocean modifier's cache path, the dynamic paint image output directory, and proxy directories. The other possible reading is the file's own save location, which I have not answered. Flagging that, as instructed, rather than silently choosing.

The answer has several parts. Each is labelled on its own, because they do not all rest on the same footing.

### 17a. The render output place is looked at to see which frames are already there

**Label: documented** (that rendering resumes from what is already in the output directory).
**Label: inferred** (that Blender does this by testing each frame's file for existence).

Exact words, Blender 5.1 manual, "Rendering Animations":

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

> "Just disable the Overwrite option to start rendering where you left off."

And, a few lines further down, in "Animation Preview":

> "Using an image format for output, you can use the Frame Step option to render every N'th frame. Then disable Overwrite and re-render with Frame Step set to 1."

Address: https://docs.blender.org/manual/en/5.1/render/output/animation.html
Date read: 2026-07-30

The setting itself, Blender 5.1 manual, "Output":

> "Image Sequence – Overwrite Image Multi-Layer EXR
>
> Overwrite existing files when rendering."

Address: https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
Date read: 2026-07-30

(The heading above that paragraph is printed twice in the manual with the same text, once for Overwrite and once for Placeholders. The two descriptions underneath are different and are the ones I quote.)

The same setting in the Python API, `RenderSettings.use_overwrite`:

> "Overwrite existing files while rendering (default True)"

Address: https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
Date read: 2026-07-30

The picture of the Output panel shows the setting in place. Overwrite is ticked, Placeholders is not, and the path field reads `/tmp/`.

Image: `_images/render_output_properties_output_panel.png`, on https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
Date read: 2026-07-30

The inference step, written out: the manual says that with Overwrite off, rendering starts "where you left off", and that the already-rendered frames "will still be in the output directory". For that to work Blender must look at the output place and find out, frame by frame, what is already there. The sources never say so in those words, and never say when the check happens or exactly what it tests. So the mechanism is my inference, not the manual's statement.

**Not covered.** Whether the check is per-frame or per-directory. Whether it tests only for the file's presence or also its contents or size. What happens when the file that is present is empty or half-written. Whether a zero-length placeholder from another machine counts as "already there" — the render-farm hint below implies it must, but nothing says it.

### 17b. The render output place is used as a shared noticeboard between machines

**Label: documented.**

Exact words, Blender 5.1 manual, "Output", under the heading "Hint":

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

And the setting:

> "Create empty placeholder frames while rendering."

Address: https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
Date read: 2026-07-30

The Python API puts it slightly differently, `RenderSettings.use_placeholder`:

> "Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

Address: https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
Date read: 2026-07-30

So the output place is not only written to. It is the only thing the several machines in that hint share. Each one writes an empty file to claim a frame, and each one reads the directory to find out which frames are unclaimed.

**Not covered.** The manual does not say what happens if two machines claim the same frame at the same instant, nor whether a network file system gives any guarantee about that. It does not say what happens if a machine dies after writing its placeholder and before writing the frame — the placeholder would remain, and nothing states whether a later run treats that frame as done. It calls the arrangement "primitive", and offers no further detail.

### 17c. The render output place is read back for playback

**Label: documented.**

Exact words, Blender 5.1 manual, "Animation Player":

> "Launching the animation player opens a new window, playing back images or a video located at the render output of the current scene."

Address: https://docs.blender.org/manual/en/5.1/render/output/animation_player.html
Date read: 2026-07-30

And, Blender 5.1 manual, "Introduction" (Output):

> "Rendered image sequences can be played back in the Animation Player."

Address: https://docs.blender.org/manual/en/5.1/render/output/introduction.html
Date read: 2026-07-30

The matching operator, Python API, `bpy.ops.render.play_rendered_anim()`:

> "Play back rendered frames/movies using an external player"

Address: https://docs.blender.org/api/5.1/bpy.ops.render.html
Date read: 2026-07-30

**Not covered.** What the player does when the render output place holds nothing, or holds only part of a sequence.

### 17d. The render output place is read back as the input to Cycles' animation denoiser

**Label: observed** (this is what Cycles' own file says).

Cycles ships an operator that takes the scene's recorded render output path and uses it as the place to *read* frames from.

File: `Blender Sources/cycles_5.1/operators.py`, class `CYCLES_OT_denoise_animation`, lines 30-88.

Its description, lines 31-33:

> `"Denoise rendered animation sequence using current scene and view " \`
> `    "layer settings. Requires denoising data passes and output to " \`
> `    "OpenEXR multilayer files"`

Its `input_filepath` property, lines 37-41:

> `    input_filepath: StringProperty(`
> `        name='Input Filepath',`
> `        description='File path for image to denoise. If not specified, uses the render file path and frame range from the scene',`
> `        default='',`
> `        subtype='FILE_PATH')`

And what it does when no input path is given, lines 69-88:

> `        else:`
> `            # Denoise animation sequence with expanded frames matching`
> `            # Blender render output file naming.`
> `            in_filepath = scene.render.filepath`
> `            if out_filepath == '':`
> `                out_filepath = in_filepath`
> `` 
> `            # Backup since we will overwrite the scene path temporarily`
> `            original_filepath = scene.render.filepath`
> `` 
> `            for frame in range(scene.frame_start, scene.frame_end + 1):`
> `                scene.render.filepath = in_filepath`
> `                filepath = scene.render.frame_path(frame=frame)`
> `                in_filepaths.append(filepath)`
> `` 
> `                if not os.path.isfile(filepath):`
> `                    scene.render.filepath = original_filepath`
> `                    err_msg = tip_("Frame '%s' not found, animation must be complete") % filepath`
> `                    self.report({'ERROR'}, err_msg)`
> `                    return {'CANCELLED'}`

Date read: 2026-07-30

Three things this file says outright. The recorded render output path is used as a read location. Every frame in the scene's frame range is tested for existence with `os.path.isfile`. If any is missing the operator stops with the message "Frame '%s' not found, animation must be complete".

This is the only place in the sources I was given where the existence check on the output place is spelled out in so many words — and note that it is Cycles' own animation-denoise operator doing it, not the render itself.

The operator is registered, `Blender Sources/cycles_5.1/operators.py`, lines 151-155:

> `classes = (`
> `    CYCLES_OT_use_shading_nodes,`
> `    CYCLES_OT_denoise_animation,`
> `    CYCLES_OT_merge_images`
> `)`

Date read: 2026-07-30

**Not covered.** What happens when the operator is actually run is a step past what the file says, and would be **inferred**, not observed. The step: reading `if not os.path.isfile(filepath)` and concluding "a real run stops on the first missing frame" assumes the code runs as written, which I did not verify. I did not run Blender. I also searched Cycles' `ui.py` for `denoise_animation` and found no menu entry, and searched the whole 5.1 manual for "denoise animation" and "merge images" and found nothing — so the sources do not show where a person would find this from the interface.

### 17e. The recorded output place is queried to work out a file name, without anything being written

**Label: documented** that a file name is handed back; **inferred** that nothing is written when it is asked for. *(Changed by the citation check, 2026-08-06: the label read "documented" for the whole of this. The quoted words say a path is returned; they do not say that nothing is written. This version's first-pass document, D12, says so plainly.)*

Python API, `RenderSettings.frame_path()`:

> "Return the absolute path to the filename to be written for a given frame"

and its return value:

> "File Path, The resulting filepath from the scenes render settings (never None)"

Address: https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
Date read: 2026-07-30

So the recorded place is also a source of answers — you can ask what a given frame's file would be called without rendering it. The Cycles operator quoted above uses exactly this.

Related, and worth separating out: the recorded output path is not taken literally. It is expanded before use. Blender 5.1 manual, "File Paths":

> "Path templates substitute template expressions (written {abcd}, where "abcd" is a variable name) in a filepath when the path is used."

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."

Address: https://docs.blender.org/manual/en/5.1/files/file_paths.html
Date read: 2026-07-30

And the same page on what happens when the recorded text is malformed:

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

Address: https://docs.blender.org/manual/en/5.1/files/file_paths.html
Date read: 2026-07-30

That last one is a use besides writing in a plain sense: the recorded place is examined and can be judged faulty before anything is written.

**Not covered.** The list of variables is given for 5.1 only. The manual says more paths will support templates later, so this list is version-specific.

### 17f. Cache and bake places are read back — that is their whole point

**Label: documented.**

Several kinds of place recorded for writing exist beyond the render output. In every case the sources say the same thing: what is written there is later read back.

Ocean modifier. Blender 5.1 manual, "Ocean Modifier":

> "Rather than simulating the ocean data live, it can be baked to files in a given directory. When a simulation is baked, the simulator engine is completely bypassed, and all information for the modifier or texture is retrieved from the baked files."

> "Cache Path
>
> Folder to store the baked EXR files in. The sequences will be in the form disp_####.exr, normal_####.exr, and foam_####.exr, where #### is the four digit frame number. If the cache path folder does not exist, it will be created."

> "Upon loading the data from these baked files, when a frame of the bake sequence is read, it is cached in memory. This means that accessing loaded frames subsequent times is fast, not incurring the overhead of drive access."

> "The start and end frames of the bake are repeated when accessing frames outside of the baked range."

Address: https://docs.blender.org/manual/en/5.1/modeling/modifiers/physics/ocean.html
Date read: 2026-07-30

Fluid domain. Blender 5.1 manual, "Cache":

> "Cache Directory
>
> Directory to store baked simulation files in. Inside this directory each simulation type (i.e. mesh, particles, noise) will have its own directory containing the simulation data."

> "Offset
>
> Frame offset that is used when loading the simulation from the cache. It is not considered when baking the simulation, only when loading it."

Address: https://docs.blender.org/manual/en/5.1/physics/fluid/type/domain/cache.html
Date read: 2026-07-30

Physics point caches. Blender 5.1 manual, "Baking Physics Simulations":

> "External
>
> Allows you to read the cache from a drive using a user-specified file path."

> "Path
>
> Select the directory path to the cache files."

> "Disk Cache
>
> The cache of a baked simulation will be stored inside the blend-file when you save it. When Disk Cache is checked, Blender will save the cache separately to the drive in a folder named blendcache_[filename] alongside the blend-file. (The blend-file must be saved first.)"

Address: https://docs.blender.org/manual/en/5.1/physics/baking.html
Date read: 2026-07-30

Geometry node bakes. Blender 5.1 manual, "Bake Node":

> "The Bake node allows saving and loading intermediate geometries."

> "Bake Path
>
> Location on disk where the baked data is stored. Note, this path is also used for simulation zones."

Address: https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/geometry/operations/bake.html
Date read: 2026-07-30

Proxies in the video sequencer. Blender 5.1 manual, "Proxy":

> "Directory
>
> By default, all generated proxy videos are stored to the folder <path of original footage>/BL_proxy/<clip name>, but this can be changed to a custom directory using this option."

> "File
>
> Allows you to use preexisting proxies."

Address: https://docs.blender.org/manual/en/5.1/editors/video_sequencer/sequencer/sidebar/proxy.html
Date read: 2026-07-30

There are two more such places in the API whose descriptions say only "save"/"store", with no read use stated. `DynamicPaintSurface.image_output_path`:

> "Directory to save the textures (default "", never None, blend relative // prefix supported)"

Address: https://docs.blender.org/api/5.1/bpy.types.DynamicPaintSurface.html

`CompositorNodeOutputFile.directory`:

> "The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)"

Address: https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html
Date read for both: 2026-07-30

For those two the sources record no use besides writing.

**Not covered.** How a bake place is searched — whether Blender expects exact file names, or scans the directory. The point cache page gives one hint and no more: "The cache name in Caches List and the Index has to exactly match the external cache files name in order to work. The cache files name format is name_frame_index.bphys." Nothing similar is given for the geometry node bake path or the fluid cache directory.

### 17g. The write places are counted among the file's paths

**Label: documented** that this happened; **not in the sources** what it means.

Blender 5.1 release notes, "Corrective Releases", in the list of fixes for Blender 5.1.1:

> "GeoNodes simulation cache files not reported via foreach_path callback. (blender/blender#155953)"

Address: https://developer.blender.org/docs/release_notes/5.1/corrective_releases/
Date read: 2026-07-30

That is a statement that in 5.1.1 the geometry node simulation cache files are reported through a path-enumeration callback — that is, they are treated as paths belonging to the file and not only as somewhere to write. The sources I have do not explain what `foreach_path` is or what consumes it, and I did not go outside them to find out.

The nearest user-facing things are the External Data operators. Blender 5.1 manual, "Topbar":

> "Make Paths Relative
>
> Make all paths to external files relative to the current blend-file."

> "Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist."

Address: https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html
Date read: 2026-07-30

**Not covered — and this is a real gap.** Neither the manual nor the API says whether "all paths to external files" includes write places such as the render output path or a bake directory. The API descriptions are just as unspecific: `bpy.ops.file.make_paths_relative()` is "Make all paths to external files relative to current .blend", and `bpy.ops.file.report_missing_files()` is "Report all missing external files" (https://docs.blender.org/api/5.1/bpy.ops.file.html, read 2026-07-30). I looked for a list of which properties count and found none. So I cannot say whether making paths relative rewrites your render output path, nor whether a missing output directory is reported as a missing file. It matters, and the sources do not answer it.

### Where I looked for 17

Searched the whole 5.1 manual (case-insensitive) for: `Placeholder`, `overwrite existing`, `already exist`, `existing frames`, `already rendered`, `resume`, `rendered animation`, `Play Rendered Animation`, `render farm` / `render-farm` / `renderfarm`, `Render Cache` / `Cached Render` / `Cache Result`, `cache directory`, `blendcache`, `path template`, `Make Paths Relative` / `Make Paths Absolute` / `Report Missing Files` / `Find Missing Files`, `writable` / `write permission` / `not allowed to write` / `read-only`, `Saved:`.

Searched the whole 5.1 API documentation for: `use_overwrite`, `use_placeholder`, `filepath` on `RenderSettings`, `frame_path`, `bake_directory`, `bake_target`, `foreach_path`, `Directory to`, `Location on disk`, `Path to save`, `Directory to store`, `output path`, `play_rendered_anim`.

Searched Cycles' 5.1 files for: `cache` / `Cache`, `filepath`, `frame_path`, `render.filepath`, `denoise_animation`, `merge_images`.

Searched the 5.1 release notes for: `bake`, `cache`.

Opened and read in full: the Output properties page, the Rendering Animations page, the Animation Player page, the Output introduction page, the File Paths (path templates) page, the File Output node page, the Baking Physics Simulations page, the fluid Cache page, the Ocean modifier bake section, the Bake node page, the Geometry Nodes modifier page, the sequencer Proxy page, the Command Line Arguments render and playback sections, the Rendering From The Command Line page, `cycles_5.1/operators.py`.

Opened the picture `_images/render_output_properties_output_panel.png`.

Ruled out, and why. The Preferences file paths page (https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html) carries "Render Output — Where rendered images/videos are saved" and "Render Cache — The location where cached render images are stored". Both are preferences, not places recorded in a `.blend`, so they are outside this question. I note them only because the "Cache Result" option on the Output panel writes to the preference's Render Cache folder, not to any place the file records — the manual says "The image is stored in the Render Cache folder as specified in the File Paths Preferences", and it is read back by "Open Cached Render" and by the compositor's "Read View Layers". If the question was meant to reach that, the honest answer is that the place involved is not one the file records.

### Outside the interface (17)

**Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** 

Mostly yes, with one clear exception, and one gap.

The settings behind 17a and 17b are stored in the file, and the command line uses the file's settings. Blender 5.1 manual, "Rendering From The Command Line":

> "-a
>
> Render the whole animation using all the settings saved in the blend-file."

Address: https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html
Date read: 2026-07-30

The write place itself can also be set from outside. Blender 5.1 manual, "Command Line Arguments":

> "-o, --render-output <path>
>
> Set the render path and file name. Use // at the start of the path to render relative to the blend-file."

> "-b, --background
>
> Run in background (often used for UI-less rendering)."

Address: https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
Date read: 2026-07-30

There is no command line switch for Overwrite or Placeholders in the argument list. They come from the file. That is consistent with the render-farm hint, which describes several machines all rendering the same file to one directory.

The exception is 17c, playback. The animation player needs a window: "Launching the animation player opens a new window". The command line has a separate player mode, and it takes files as arguments rather than reading the scene's recorded output place:

> "-a <options> <file(s)>
>
> Instead of showing Blender's user interface, this runs Blender as an animation player, to view movies and image sequences rendered in Blender (ignored if -b is set)."

Address: https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
Date read: 2026-07-30

So the "play what is at the recorded output place" behaviour is an interface behaviour. Driven from outside, you name the files yourself, and in background mode the player is ignored altogether.

17d and 17e are Python. Nothing in the sources says either is unavailable without a window.

The gap: the sources nowhere state that the existing-file check behind 17a happens the same way in background mode. I am relying on the sentence that `-a` uses "all the settings saved in the blend-file". That is the strongest thing the sources offer and it is not a direct statement about the check.

### Operating system (17)

**Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** 

The sources do not say any of this differs by operating system, and I found no statement that it does. Two things are worth carrying anyway.

Path length is an operating system matter, and the manual says so for caches. Blender 5.1 manual, "Cache" (particles):

> "If you are not allowed to write to the required subdirectory caching will not take place, e.g. if your blend-file path is very long and your operating system has a limit on the path length that is supported."

Address: https://docs.blender.org/manual/en/5.1/physics/particles/emitter/cache.html
Date read: 2026-07-30

The placeholder description names a Unix tool as an analogy — "similar to Unix 'touch'" — but the property is described without any platform restriction, and the manual's render-farm hint puts no platform condition on it.

Blender runs on Windows, macOS and Linux. Blender's requirements page:

> "Blender is cross-platform, it runs on every major operating system."

and, for the only sentence on that page that names the version I am researching:

> "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later."

Address: https://www.blender.org/download/requirements/
Date read: 2026-08-01

I treat "5.0 and later" as naming 5.1. Everywhere else that page describes whichever release was current on 2026-08-01 and does not speak for 5.1.

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

**Reading of the question.** "A cache or a bake that a file records" can mean either a cache the file points at on disk, or a bake the file itself holds. I answer both, marking which is which. I have taken "absent" to include both "never made" and "made and then gone".

There is no single answer. The sources describe five different behaviours and are silent on several more. I take them one at a time, and then say plainly where the silence is.

### 18a. Physics simulations: the render does not stop, it simulates

**Label: documented** for the quoted sentences. **Label: inferred** for what this means for a render of one frame.

Blender 5.1 manual, "Baking Physics Simulations":

> "Baking refers to the act of storing or caching the results of a calculation. The result of a simulation is automatically cached to memory when the animation is played, so that the next time it runs, it can be replayed more quickly by reading the results from the memory."

> "It is generally recommended to bake your physics simulations before rendering. Aside from no longer needing to go through the time-consuming process of simulating again, baking can help prevent potential glitches and ensure that the outcome of the simulation remains exactly the same every time."

> "Note that playing the animation will try to simulate any visible physics simulations."

Address: https://docs.blender.org/manual/en/5.1/physics/baking.html
Date read: 2026-07-30

Blender 5.1 manual, "Cache" (particles), under "Hints":

> "When an animation is played, each physics system writes each frame to the cache. Note that for the cache to fill up, one has to start the playback before or on the frame that the simulation starts."

> "The simulation is only calculated for positive frames in between the Start and End frames of the Cache panel, whether you bake or not."

Address: https://docs.blender.org/manual/en/5.1/physics/particles/emitter/cache.html
Date read: 2026-07-30

And what Cycles says about the consequence. Blender 5.1 manual, "Motion Blur":

> "If there are particles or other physics system in a scene, be sure to bake them before rendering, otherwise you might not get correct or consistent motion blur."

Address: https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/motion_blur.html
Date read: 2026-07-30

What is documented: absence of a bake is not an error. The simulation is computed instead of read, which takes time; the outcome is not guaranteed to be identical run to run; and motion blur specifically may come out wrong.

The inference step, written out: the sentences about filling the cache are about *playing* the animation, not about rendering. To get from "for the cache to fill up, one has to start the playback before or on the frame that the simulation starts" to "rendering a single frame in the middle of an unbaked simulation gives the wrong result", a reader has to supply the assumption that a render evaluates frames the same way playback does. The sources do not say that. So that conclusion is **inferred**, and I flag it as the weakest link in this answer.

### 18b. Geometry node simulations: baking is what makes out-of-order rendering possible

**Label: documented** for the quotes. **Label: inferred** for what happens without the bake.

Blender 5.1 manual, "Simulation Nodes":

> "The simulation is automatically cached during playback. The valid cache can be seen as a strong yellow line in the timeline editor."

> "When the result is ready to be sent to a render-farm, it can be baked to disk. This allows for the simulation to be rendered in a non-sequential order."

> "Calculate to Frame
>
> Calculate simulations in geometry nodes modifiers from the start to current frame."

Address: https://docs.blender.org/manual/en/5.1/physics/simulation_nodes.html
Date read: 2026-07-30

The same two sentences appear on the Simulation Zone page (https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/simulation/simulation_zone.html, read 2026-07-30), where the words "render-farm" and "non-sequential order" are used identically.

What is documented: baking to disk is the thing that "allows for the simulation to be rendered in a non-sequential order", and the operator that fills the gap without a bake works "from the start to current frame".

The inference step, written out: if the bake is what allows non-sequential rendering, then without it rendering is sequential — a render of frame 300 has to come after frames 1 to 299 were evaluated. The manual never states the negative case. It only states what baking allows. Reading the negative out of it is my step.

### 18c. EEVEE light probe volumes: the render falls back to the world

**Label: inferred.** *(Changed by the citation check, 2026-08-06: this read "documented". The quoted sentence says "Irradiance Volume" while the page is headed "Light Probe Volume", and nothing quoted says the two names mean the same thing, so a reader has to supply that step.)*

This is a bake the file itself holds, not one it points at.

Blender 5.1 manual, "Light Probe Volume":

> "If an object is not inside any Irradiance Volume, or if the indirect lighting has not been baked, the world's diffuse lighting will be used to shade it."

> "Light probe volume light data is static and needs to be manually baked. Once baked, the data is stored inside the object data-block and can be moved, animated and linked between blender files."

Address: https://docs.blender.org/manual/en/5.1/render/eevee/light_probes/volume.html
Date read: 2026-07-30

This is the clearest answer on the sheet. The render proceeds, and something different is drawn — the world's diffuse lighting stands in for the missing bake. Nothing in the manual says a warning is raised.

**Not covered.** Whether anything is reported. Whether an image rendered without the bake is distinguishable from one rendered with it other than by looking at it.

### 18d. The ocean modifier: out-of-range frames repeat the ends

**Label: documented** for the range behaviour. **Label: not in the sources** for missing files.

Blender 5.1 manual, "Ocean Modifier":

> "The start and end frames of the bake are repeated when accessing frames outside of the baked range."

Address: https://docs.blender.org/manual/en/5.1/modeling/modifiers/physics/ocean.html
Date read: 2026-07-30

Python API, `OceanModifier.is_cached`:

> "Whether the ocean is using cached data or simulating (default False, readonly)"

Address: https://docs.blender.org/api/5.1/bpy.types.OceanModifier.html
Date read: 2026-07-30

So there are two states, cached and simulating, and asking for a frame past the ends of a bake gets you the end frame's data rather than nothing. What happens when the EXR files at the recorded Cache Path are gone while the modifier still thinks it is baked is not stated anywhere I could find.

### 18e. What is reported

**Label: documented** that a cache reports its own state. **Label: not in the sources** that a render reports anything.

Python API, `PointCache.info`:

> "Info on current cache status (default "", readonly, never None)"

and alongside it:

> "is_baked
>
> The cache is baked (default False, readonly)"

> "is_outdated
>
> (default False, readonly)"

> "is_frame_skip
>
> Some frames were skipped while baking/saving that cache (default False, readonly)"

Address: https://docs.blender.org/api/5.1/bpy.types.PointCache.html
Date read: 2026-07-30

What that status text looks like when nothing is cached is visible in the manual's own picture of the Cache panel: the line reads "0 frames in memory (0 B)."

Image: `_images/physics_particles_emitter_cache_settings.png`, on https://docs.blender.org/manual/en/5.1/physics/particles/emitter/cache.html
Date read: 2026-07-30

That is what the picture shows and no more. It is a panel in the interface, at a moment when nothing was cached. It does not show anything about rendering, and I read nothing further into it.

I found no statement anywhere in the 5.1 manual, API documentation or release notes that a render prints, logs or otherwise announces a missing cache or bake.

### 18f. Where the sources say nothing at all

**Label: not in the sources.**

For each of these I searched and found nothing:

- **Fluid caches.** What a render does when the Cache Directory is empty, missing, or holds only some frames. The fluid Cache page describes baking, the three cache types, formats and compression, and says nothing about absence.
- **Point caches marked External.** What happens when the recorded Path holds nothing, or holds files whose names do not match. The manual gives only the requirement — "The cache name in Caches List and the Index has to exactly match the external cache files name in order to work" — and never says what "not work" looks like.
- **Disk caches in `blendcache_[filename]`.** What happens when the blend-file has been moved or copied without that folder.
- **Geometry node bakes on disk.** What a render does when the Bake Path holds nothing. The Bake node page and the Geometry Nodes modifier page describe where data is stored and never what happens when it is not there.
- **Alembic and USD cache files.** The Mesh Sequence Cache modifier page describes what is read and how frames are looked up, and says nothing about a missing file.
- **Whether a render ever stops.** No source I was given says a missing cache or bake aborts a render.

### One adjacent finding, which is not an answer to this question

Cycles' own file states outright what happens when one particular cache is missing — but it is a cache Cycles keeps for itself, not one the `.blend` records, so it does not answer the question as asked. I include it only so it is not mistaken for something I missed.

File: `Blender Sources/cycles_5.1/properties.py`, `kernel_optimization_level`, description:

> `        description="Kernels can be optimized based on scene content. Optimized kernels are requested at the start of a render. "`
> `                    "If optimized kernels are not available, rendering will proceed using generic kernels until the optimized set "`
> `                    "is available in the cache. This can result in additional CPU usage for a brief time (tens of seconds)",`

Date read: 2026-07-30. **Label: observed.** The render proceeds with a slower fallback. Again, this is not a place the file records.

### Where I looked for 18

Searched the whole 5.1 manual (case-insensitive) for: `not baked`, `isn't baked`, `is not baked`, `unbaked`, `without baking`, `if you do not bake`, `if you don't bake`, `cannot be found`, `could not be found`, `is missing`, `are missing`, `not found`, `recalculat`, `re-simulat`, `resimulat`, `recomput`, `non-sequential`, `out of order`, `sequential order`, `render farm` / `render-farm`, `from the first frame`, `from frame 1`, `must be played`, `play the animation from`, `skipping frames`, `bake` restricted to the EEVEE and geometry-nodes folders, and `cache`/`bake` crossed with `before rendering`, `when rendering`, `at render time`, `during rendering`.

Searched the whole 5.1 API documentation for: `has not been baked`, `not baked`, `no bake`, `bake is missing`, `missing bake`, `without a bake`, plus the full text of `bpy.types.PointCache`, `bpy.types.OceanModifier`, `bpy.types.NodesModifier`, `bpy.types.NodesModifierBake`, `bpy.ops.ptcache`, and the light probe operators in `bpy.ops.object`.

Searched the 5.1 release notes for `bake` and `cache` across every file, and read the Core, Pipeline & I/O, Rendering, Geometry Nodes, Python API and index pages.

Searched Cycles' 5.1 files for `cache` and `Cache`.

Opened and read in full: the Baking Physics Simulations page, the particle Cache page, the cloth Cache page, the soft body Cache page, the fluid domain Cache page, the Simulation Nodes page, the Simulation Zone page, the Bake node page, the Geometry Nodes modifier Bake section, the geometry nodes Baking page, the Light Probe Volume page, the Light Probe Sphere page, the light probes index, the Ocean modifier bake section, the Mesh Sequence Cache modifier page, the dynamic paint Canvas page, the Cycles Motion Blur page, the Timeline editor Cache section.

Opened the pictures `_images/physics_particles_emitter_cache_settings.png`, `_images/physics_baking_multi-cache-interface.png` and `_images/modeling_geometry-nodes_simulation_baking.png`.

Ruled out, and why. The troubleshooting section of the 5.1 manual contains no mention of `cache` or `bake` at all, so it holds nothing on this. The command line arguments page has no bake or cache-building switch — the only match for either word is `-c <cache_memory>`, which is the animation player's image buffer. The 5.1 release notes mention caches only in bug-fix lists.

### Outside the interface (18)

**Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** 

**Not settled by the sources, and this is the most important gap in this answer.**

Every documented statement about an unbaked physics simulation filling its cache is phrased in terms of *playback* — "when the animation is played", "one has to start the playback before or on the frame that the simulation starts", "playing the animation will try to simulate any visible physics simulations". Playback is an interface activity. Nothing in the sources says what the equivalent is when Blender is started with `-b` and told to render.

The one thing the sources do give is that the file's settings are used: "-a — Render the whole animation using all the settings saved in the blend-file" (https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html, read 2026-07-30). And the command line offers no way to bake — I searched the whole argument list for `bake` and `cache` and the only hit is the player's memory option.

The manual does say that baking is what lets a simulation be sent out and rendered out of order: "When the result is ready to be sent to a render-farm, it can be baked to disk. This allows for the simulation to be rendered in a non-sequential order." That is the closest the sources come to addressing rendering outside the interface, and it addresses the baked case, not the absent one.

For 18c, the EEVEE fallback, there is a separate and quite specific limitation on driving EEVEE from outside. Blender 5.1 manual, "Limitations":

> "Headless Rendering
>
> Headless rendering is not supported on headless Windows systems."

Address: https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html
Date read: 2026-07-30

### Operating system (18)

**Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** 

The sources do not say that any of the behaviours in 18a to 18e differ by operating system, and I found no statement that they do.

Two operating-system-specific statements did turn up nearby and are worth carrying:

> "If you are not allowed to write to the required subdirectory caching will not take place, e.g. if your blend-file path is very long and your operating system has a limit on the path length that is supported."

Address: https://docs.blender.org/manual/en/5.1/physics/particles/emitter/cache.html

> "Headless rendering is not supported on headless Windows systems."

Address: https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html
Date read for both: 2026-07-30

The first is about a cache failing to be *made*, which is a different thing from one being absent at render time, but it is the only place the sources tie cache behaviour to an operating system.

Blender itself runs on Windows, macOS and Linux — "Blender is cross-platform, it runs on every major operating system", with "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later" being the only sentence on that page that names 5.1. https://www.blender.org/download/requirements/, read 2026-08-01.

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
