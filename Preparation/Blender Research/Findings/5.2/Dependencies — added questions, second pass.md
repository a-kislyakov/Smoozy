# Dependencies — added questions, second pass (Blender 5.2)

**Sources consulted, and when.** Blender's 5.2 manual, Blender's 5.2 Python API documentation, Blender's 5.2 release notes and Cycles' own 5.2 files — all read on **2026-07-30** (the date the snapshot in `Blender Sources/` was taken). I did not need Blender's requirements page for either question, and do not cite it.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

Scope: questions 17 and 18 only. Every other question on the sheet was left alone. I started no other agents.

---

## 17. What use does a `.blend` file make of the places it records for writing, besides writing to them?

**LABEL: documented**, for each individual use listed below. One general claim inside it — that Blender inspects the output directory to work out which frames are already there — is **inferred**, and marked as such where it appears.

**Short answer.** A recorded write location is not only written to. Blender also (a) reads data back out of it, (b) looks at what is already sitting there and decides not to write, or deletes it, and (c) resolves, checks and displays the path itself without touching the disk at all. Below, each use with the words that carry it.

### (a) Reading back out of places recorded for writing

**Physics caches.** The cache path recorded for a physics system can be pointed at a cache somebody else wrote:

> "External
>
> Allows you to read the cache from a drive using a user-specified file path."
>
> "Path
>
> Select the directory path to the cache files."

Source: manual, *Baking Physics Simulations*. Address: https://docs.blender.org/manual/en/5.2/physics/baking.html — read 2026-07-30.

The API says the same of the recorded property:

> "use_external
>
> Read cache from an external location (default False)"

and

> "filepath
>
> Cache file path (default "", never None, blend relative // prefix supported)"

Source: Python API, `PointCache`. Address: https://docs.blender.org/api/5.2/bpy.types.PointCache.html — read 2026-07-30.

**Fluid cache directory.** The recorded directory is read on the way in, and one setting applies only to reading:

> "Cache Directory
>
> Directory to store baked simulation files in."

> "Offset
>
> Frame offset that is used when loading the simulation from the cache.
> It is not considered when baking the simulation, only when loading it."

Source: manual, *Cache* (fluid domain). Address: https://docs.blender.org/manual/en/5.2/physics/fluid/type/domain/cache.html — read 2026-07-30.

**Ocean modifier cache path.** Here the manual states outright that once the bake exists, the recorded folder is the only thing consulted:

> "Rather than simulating the ocean data live, it can be baked to files in a given directory.
> When a simulation is baked, the simulator engine is completely bypassed,
> and all information for the modifier or texture is retrieved from the baked files."

> "Upon loading the data from these baked files, when a frame of the bake sequence is read,
> it is cached in memory."

> "Cache Path
>
> Folder to store the baked EXR files in."

Source: manual, *Ocean Modifier*. Address: https://docs.blender.org/manual/en/5.2/modeling/modifiers/physics/ocean.html — read 2026-07-30.

**Geometry-nodes bake path.** The recorded bake location is described as a two-way place:

> "Baking allows saving and loading intermediate geometries."

Source: manual, *Baking* (geometry nodes). Address: https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/baking.html — read 2026-07-30.

> "Bake Path
>
> Location on disk where the baked data for
> Simulation Zones
> and Bake Nodes are stored."

Source: manual, *Geometry Nodes Modifier*. Address: https://docs.blender.org/manual/en/5.2/modeling/modifiers/geometry_nodes.html — read 2026-07-30.

**Video-sequencer proxy directory.** A place recorded for writing proxies, which can also be pointed at proxies already written:

> "Proxy Directory
>
> The location to store the proxies for the project."

> "File
>
> Allows you to use preexisting proxies."

Source: manual, *Proxy*. Address: https://docs.blender.org/manual/en/5.2/editors/video_sequencer/sequencer/sidebar/proxy.html — read 2026-07-30.

### (b) Looking at what is already there, and deciding not to write it — or deleting it

**The render output path.** Two settings exist whose only purpose is to make Blender take account of what is already at the output location:

> "Image Sequence – Overwrite Image Multi-Layer EXR
>
> Overwrite existing files when rendering.
>
> Image Sequence – Overwrite Image Multi-Layer EXR
>
> Create empty placeholder frames while rendering."

Source: manual, *Output*. Address: https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. (The heading line is repeated in the source page; the second block is the Placeholders setting. The panel picture confirms the two names.)

Picture: `_images/render_output_properties_output_panel.png`, on that same page. It shows the Output panel with a path field reading `/tmp/` and four checkboxes: **Overwrite** (ticked), **Placeholders** (unticked), **File Extensions** (ticked), **Cache Result** (unticked). **documented** — this settles that the two options are named Overwrite and Placeholders and that Overwrite is on by default.

The same page gives the use these two are for:

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

And on the rendering-animations page:

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering,
> since the frames you have already rendered will still be in the output directory.
>
> Just disable the Overwrite option to start rendering where you left off."

> "Using an image format for output,
> you can use the Frame Step option to render every N'th frame.
> Then disable Overwrite and re-render with Frame Step set to 1."

Source: manual, *Rendering Animations*. Address: https://docs.blender.org/manual/en/5.2/render/output/animation.html — read 2026-07-30.

And:

> "when rendering animations to image file formats the render job can be canceled
> and resumed at the last rendered frame by changing the frame range."

Source: manual, *Introduction* (Render Output). Address: https://docs.blender.org/manual/en/5.2/render/output/introduction.html — read 2026-07-30.

The API descriptions of the two recorded settings:

> "use_overwrite
>
> Overwrite existing files while rendering (default True)"

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

Source: Python API, `RenderSettings`. Address: https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30.

**inferred** — from "Overwrite existing files while rendering" plus "disable the Overwrite option to start rendering where you left off", it follows that before writing a frame Blender looks at the output location to see whether that frame's file is already present, and skips it when Overwrite is off. The step a reader must supply: the sources never say "Blender checks whether the file exists". They say the option controls whether existing files are overwritten and that turning it off resumes where you left off; skipping can only be decided by inspecting what is there. Nothing in the sources describes the check itself — not its timing, not whether an empty placeholder counts as present, not what happens if two machines look at the same instant.

**Deleting at a recorded write place.** The recorded cache locations are also where deletion operators act:

> "Once the simulation has been baked, the cache can be deleted by pressing Free All."

Source: manual, *Cache* (fluid domain), address as above — read 2026-07-30.

> "Delete All Bakes
>
> Delete bakes of all physics systems in the scene, even those of different types."

Source: manual, *Baking Physics Simulations*, address as above — read 2026-07-30.

> "Overwrite
>
> Whether to overwrite existing proxy files or keep them."

Source: manual, *Proxy*, address as above — read 2026-07-30.

### (c) Resolving, checking and showing the path, without touching the disk

**Relative-path expansion.** The recorded output place is a path that gets expanded like any other:

> "This setting expands Relative Paths
> where a // prefix represents the directory of the current blend-file."

Source: manual, *Output*, address as above — read 2026-07-30. The API records the same of the property itself:

> "filepath
>
> Directory/name to save animations, # characters define the position and padding of frame numbers (default "//", never None, blend relative // prefix supported, Supports template expressions)"

Source: Python API, `RenderSettings`, address as above — read 2026-07-30.

**Template substitution and validation.** In 5.2 the recorded output place can carry template expressions, which Blender parses, substitutes, and reports errors for:

> "Path templates substitute template expressions (written {abcd}, where
> "abcd" is a variable name) in a filepath when the path is used."

> "This substitution happens internally at time of use (e.g. while rendering), so
> the template syntax will remain as-is in the path field."

> "Currently path templates are only supported for the render output path in
> Scene properties and the output paths in the compositor's File Output
> node. More filepaths will support templates in future releases."

> "When there are template errors in a path, the path field will be highlighted red
> in the UI"

> "Hovering over the path field will pop up a tooltip that contains a list of the
> template errors encountered in that path"

> "Depending on the path, template errors may prevent certain actions. For
> example, if there are errors in the render output path, then rendering an
> animation will fail with an error message indicating the path errors."

Source: manual, *File Paths*. Address: https://docs.blender.org/manual/en/5.2/files/file_paths.html — read 2026-07-30.

Picture: `_images/files_file_paths-invalid_path_template_tooltip.png`, on that same page. It shows the Output field in Scene properties outlined and filled red, containing `//my_render_dir/{blend_name.png`, with a tooltip reading "Output Path — Directory/name to save animations, # characters define the position and padding of frame numbers.", then "Value: //my_render_dir/{blend_name.png", then in red "Syntax error(s): - Invalid or incomplete template expression '{blend_name.png'." **documented** — the recorded output place is parsed and reported on in the interface before anything is written.

**Working out the path and handing it back.** The API offers a call that computes the write path and hands it back. **documented** for the handing back; **inferred** that nothing is written when it is called. *(Changed by the citation check, 2026-08-06: this read "a call whose entire purpose is to compute the write path without writing", under the document's overall "documented" label. The quoted words say a path is returned; they do not say that nothing is written.)*

> "frame_path(*, frame=-2147483648, preview=False, view='')
>
> Return the absolute path to the filename to be written for a given frame"

Source: Python API, `RenderSettings`, address as above — read 2026-07-30.

**Showing the path.** The File Output node's recorded directory is used to display, not write:

> "Output Paths"
>
> "Displays the final file path and extension for each input/layer, based on current settings.
> This helps verify that outputs are named and organized correctly."

Source: manual, *File Output Node*. Address: https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30.

**Playing back from it.** The recorded output place is where the animation player looks:

> "Launching the animation player opens a new window,
> playing back images or a video located at the render output of the current scene."

Source: manual, *Animation Player*. Address: https://docs.blender.org/manual/en/5.2/render/output/animation_player.html — read 2026-07-30.

> "Rendered image sequences can be played back in the Animation Player."

Source: manual, *Introduction* (Render Output), address as above — read 2026-07-30.

> "bpy.ops.render.play_rendered_anim()
>
> Play back rendered frames/movies using an external player"

Source: Python API, *Render Operators*. Address: https://docs.blender.org/api/5.2/bpy.ops.render.html — read 2026-07-30.

**Being enumerated as a file path.** At least one kind of recorded write place — the point cache — is among the paths Blender's path iterator walks:

> "SKIP_MULTIFILE:
>
> Skip Multi-file.
>
> Skip paths where a single dir is used with an array of files, eg. sequence strip images or point-caches. In this case only the first file path is processed. This is needed for directory manipulation callbacks which might otherwise modify the same directory multiple times."

Source: Python API, *File Path Foreach Flag Items*. Address: https://docs.blender.org/api/5.2/bpy_types_enum_items/file_path_foreach_flag_items.html — read 2026-07-30.

The iterator itself:

> "Call visit_path_fn for the file paths used by all ID data-blocks in current bpy.data."

Source: Python API, `BlendData`. Address: https://docs.blender.org/api/5.2/bpy.types.BlendData.html — read 2026-07-30.

**Not covered.**
- I could not settle whether the **render output path** is among the paths that `file_path_foreach` / `file_path_map` visit, or whether "Make Paths Relative", "Make Paths Absolute" and "Report Missing Files" touch it. The topbar page says only "Make all paths to external files relative to the current blend-file" and "This option is useful to check if there are links to unpacked files that no longer exist" (manual, *Topbar*, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html, read 2026-07-30), which does not say either way for a place written to rather than read from. Point caches are named explicitly; output paths are not named anywhere I found.
- I found nothing saying what Blender does with the output place at the moment a render *starts* — whether the directory is created, tested for writability, or locked. The only "create the directory" statement I found is for the File Output node ("Any missing directories are created automatically during output") and the Ocean cache path ("If the cache path folder does not exist, it will be created"), both of which are part of writing.
- Nothing says whether reading back a cache from a recorded write place is done by the render or only by the viewport.
- The mechanism behind the Placeholders render-farm hint — how two machines avoid rendering the same frame — is not described anywhere I found.

**Outside the interface.** **Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** Partly holds, and one part clearly does not.
- Path resolution and template substitution hold when Blender is driven from outside: "Use // at the start of the path to render relative to the blend-file. You can use path templating features such as {blend_name} in the path." (manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30). **documented.**
- The file's own recorded output place takes priority over one supplied on the command line, if the file is loaded afterwards: "…will not render to /tmp because loading the blend-file overwrites the render output that was set." Same page, same date. **documented.**
- The red field and the hover tooltip are interface-only by their nature; but the consequence is not — "rendering an animation will fail with an error message indicating the path errors" (manual, *File Paths*, address above), which does not mention the interface.
- The animation player, driven from outside, is given files directly rather than reading the scene's output place: "-a <options> <file(s)> — Instead of showing Blender's user interface, this runs Blender as an animation player, to view movies and image sequences rendered in Blender (ignored if -b is set)." (manual, *Command Line Arguments*, address above). So the "play back what is at the recorded output place" use is one I only saw documented for the in-interface operator.
- For Overwrite and Placeholders the sources do not say one way or the other. The render-farm hint describes many machines rendering to a shared directory, which is unattended work, but that is as far as it goes.

**Operating system.** **Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** The sources do not distinguish, with one exception that touches writing rather than reading: "If you are not allowed to write to the required subdirectory caching will not take place, e.g. if your blend-file path is very long and your operating system has a limit on the path length that is supported." (manual, *Cache* (particles), https://docs.blender.org/manual/en/5.2/physics/particles/emitter/cache.html, read 2026-07-30). The Placeholders description mentions Unix by way of comparison only — "similar to Unix 'touch'" — and does not limit the feature to it. Nothing I found says any of the uses above behaves differently per operating system.

---

**Observed — the hand tests for this round, run 2026-08-07 on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and 118 GB of free disk, and no display server at all.** Blender 5.2 was started from a terminal with no window, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and the Blender answered `RESOURCES /root/dependencies/res-5.2`. Every render used Cycles on the processor, so nothing here depends on the graphics card. The machine was shared with other topics' work throughout, so nothing here is a timing or memory measurement of a clean machine except where it says what it did about that. The full list of tests, with every set-up written out, is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

Pictures were compared as pictures, never as files. Before that was leaned on, two renders of the same frame of the same file were compared and gave `largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE`, so any difference reported below is a real difference.

**A missing output folder is made.** A file whose render output path pointed into a folder that did not exist was rendered with `--background --factory-startup <file> --render-anim`, and again with the path two folders deep into folders that did not exist. Both worked, exit status 0:

```
EXIT STATUS 0
00:00.500  blend            | Read blend: "/root/dependencies/work/q17/5.2/render.blend"
00:00.830  render           | Saved: '/root/dependencies/work/q17/5.2/deep3/deep4/frame_0001.png'
00:00.830  render           | Time: 00:00.32 (Saving: 00:00.16)
FILES:
/root/dependencies/work/q17/5.2/deep3
/root/dependencies/work/q17/5.2/deep3/deep4
/root/dependencies/work/q17/5.2/deep3/deep4/frame_0001.png
```

**The output place is looked at, and Blender says so by name.** Five frames were rendered, frames 2 and 4 deleted, `use_overwrite` set to False, and the animation rendered again with no window:

```
00:00.487  render           | Skipping existing frame "/root/dependencies/work/q17/5.2/ow/frame_0001.png"
00:00.815  render           | Saved: '/root/dependencies/work/q17/5.2/ow/frame_0002.png'
00:00.815  render           | Skipping existing frame "/root/dependencies/work/q17/5.2/ow/frame_0003.png"
00:00.955  render           | Saved: '/root/dependencies/work/q17/5.2/ow/frame_0004.png'
00:00.955  render           | Skipping existing frame "/root/dependencies/work/q17/5.2/ow/frame_0005.png"
```

Only the two deleted files carried a new modification time afterwards; the other three kept theirs.

**An empty file counts as a frame already done.** With `use_overwrite` off and `use_placeholder` on, five zero-length files were made at the five names `frame_path()` gave, and the animation rendered:

```
EXIT 0
00:00.486  render           | Skipping existing frame "/root/dependencies/work/q17/5.2/ph/frame_0001.png"
00:00.486  render           | Skipping existing frame "/root/dependencies/work/q17/5.2/ph/frame_0002.png"
00:00.486  render           | Skipping existing frame "/root/dependencies/work/q17/5.2/ph/frame_0003.png"
00:00.486  render           | Skipping existing frame "/root/dependencies/work/q17/5.2/ph/frame_0004.png"
00:00.486  render           | Skipping existing frame "/root/dependencies/work/q17/5.2/ph/frame_0005.png"
00:00.486  reports          | No frames rendered, skipped to not overwrite
```

All five were still zero length afterwards. Note the exit status: nothing was rendered and the run still reported success.

**Two Blenders writing into one folder divide the frames between them.** A slower scene (400 samples, 200 by 200, two threads, eight frames) was rendered by two Blenders started five seconds apart into one empty folder, Overwrite off and Placeholders on. Four seconds into the first run the folder held one file, of zero length, for the frame being rendered. Then:

```
--- what the first Blender reported ---
00:10.390  render           | Saved: 'frame_0001.png'
00:10.390  render           | Skipping existing frame "frame_0002.png"
00:18.278  render           | Saved: 'frame_0003.png'
00:18.278  render           | Skipping existing frame "frame_0004.png"
00:24.057  render           | Saved: 'frame_0005.png'
00:24.058  render           | Skipping existing frame "frame_0006.png"
00:30.720  render           | Saved: 'frame_0007.png'
00:30.720  render           | Skipping existing frame "frame_0008.png"
--- what the second Blender reported ---
00:00.496  render           | Skipping existing frame "frame_0001.png"
00:10.277  render           | Saved: 'frame_0002.png'
00:10.277  render           | Skipping existing frame "frame_0003.png"
00:16.487  render           | Saved: 'frame_0004.png'
00:16.487  render           | Skipping existing frame "frame_0005.png"
00:22.544  render           | Saved: 'frame_0006.png'
00:22.544  render           | Skipping existing frame "frame_0007.png"
00:28.768  render           | Saved: 'frame_0008.png'
```

with all eight files holding a real image afterwards and none claimed twice. This was two Blenders on one machine writing into an ordinary folder; a folder shared between two machines over a network could not be reached — see the *cannot be run* note at the end.

**A bad path template stops a render started from outside, for a single frame as well as for an animation.** A control path with a sound template rendered normally. The faulty one, `{{blend_name.png` with the closing brace missing, was first put to the Blender itself with no window:

```
=== a GOOD template, as a control ===
PATH SET TO '/root/dependencies/work/q17/5.2/tmpl/{{blend_name}}_'
FRAMEPATH OK '/root/dependencies/work/q17/5.2/tmpl/tmplok_0001.png'
EXIT 0
00:01.530  render           | Saved: '/root/dependencies/work/q17/5.2/tmpl/tmplok_0001.png'
=== the BAD template ===
PATH SET TO '/root/dependencies/work/q17/5.2/tmpl/{{blend_name.png'
FRAMEPATH RAISED RuntimeError Error: Parse errors in path '/root/dependencies/work/q17/5.2/tmpl/{{blend_name.png':
--- single frame ---
EXIT 1
00:00.979  reports          | ERROR Parse errors in path '/root/dependencies/work/q17/5.2/tmpl/{{blend_name.png':
                            | - Invalid or incomplete template expression '{{blend_name.png'.
--- animation ---
EXIT 1
00:01.093  reports          | ERROR Parse errors in path '/root/dependencies/work/q17/5.2/tmpl/{{blend_name.png':
                            | - Invalid or incomplete template expression '{{blend_name.png'.
```

**The recorded path beats one given before the file on the command line.**

```
--- output given BEFORE the file ---
00:00.867  render           | Saved: '/root/dependencies/work/q17/5.2/out/frame_0001.png'
--- output given AFTER the file ---
00:00.809  render           | Saved: '/root/dependencies/work/q17/5.2/ord/after/x_0001.png'
```

**`frame_path()` names the file that appears, for a frame render and for an animation — and not for a still render.** With the output path `shot_{{blend_name}}_##_`:

```
PREDICTED frame 7: /root/dependencies/work/q17/5.2/fp2/shot_fp2_07_.png
00:01.899  render           | Saved: '/root/dependencies/work/q17/5.2/fp2/shot_fp2_07_.png'
```

but through `bpy.ops.render.render(write_still=True)` at frame 7, on the same path, the template was substituted and the `##` padding was not:

```
PREDICTED /root/dependencies/work/q17/5.2/fp/shot_fp_07_.png
ON DISK ['shot_fp_##_.png']
MATCH False
```

**The file-wide path commands reach an ocean cache path and nothing else that is recorded for writing.** One file was built holding six absolute locations — an image loaded from disk, the render output path, a compositor File Output node's directory, a point cache with External on, a geometry-nodes bake directory, and an ocean modifier's cache path:

```
--- as saved ---
  image ReadTex   : /root/dependencies/work/q17paths/5.2/assets/tex.png
  render output   : /root/dependencies/work/q17paths/5.2/renders/frame_
  file output node: /root/dependencies/work/q17paths/5.2/fileout
  point cache     : /root/dependencies/work/q17paths/5.2/ptcache
  gn bake dir     : /root/dependencies/work/q17paths/5.2/gnbake
  ocean cache     : /root/dependencies/work/q17paths/5.2/oceanbake
MAKE PATHS RELATIVE -> {{'FINISHED'}}
--- after Make Paths Relative ---
  image ReadTex   : //assets/tex.png
  render output   : /root/dependencies/work/q17paths/5.2/renders/frame_
  file output node: /root/dependencies/work/q17paths/5.2/fileout
  point cache     : /root/dependencies/work/q17paths/5.2/ptcache
  gn bake dir     : /root/dependencies/work/q17paths/5.2/gnbake
  ocean cache     : //oceanbake
```

Make Paths Absolute put both back. With all six pointed at a folder that does not exist, `bpy.ops.file.report_missing_files()` named two of them:

```
Warning: Path '/root/dependencies/work/q17paths/5.2/gone' not found, from local data-block 'OBGrid'
Warning: Path '/root/dependencies/work/q17paths/5.2/gone/tex.png' not found, from local data-block 'IMReadTex'
BLEND PATHS: ['/root/dependencies/work/q17paths/5.2/gone', '/root/dependencies/work/q17paths/5.2/gone/tex.png', '/root/dependencies/work/q17paths/5.2/assets/tex.png']
```

`OBGrid` carries the ocean modifier and `IMReadTex` is the image. The render output path, the File Output node, the point cache and the geometry-nodes bake directory were named by nothing.

**What `bpy.data.file_path_foreach` visits, and what `EXPAND_CACHES` adds.** The call's own description was printed first rather than its arguments guessed; it takes a function of three parameters. What was typed before that was read, and what came back, is recorded too:

```
file_path_foreach RAISED TypeError <lambda>() takes 1 positional argument but 3 were given
```

and then, with the right shape:

```
HAS file_path_foreach: True
HAS file_path_map    : True
FOREACH flags=None visited 3:
    Grid '/root/dependencies/work/q17paths/5.2/oceanbake'
    ReadTex '/root/dependencies/work/q17paths/5.2/assets/tex.png'
    Tex '/root/dependencies/work/q17paths/5.2/assets/tex.png'
FOREACH flags={{'EXPAND_CACHES'}} visited 3:
    Grid '/root/dependencies/work/q17paths/5.2/oceanbake'
    ReadTex '/root/dependencies/work/q17paths/5.2/assets/tex.png'
    Tex '/root/dependencies/work/q17paths/5.2/assets/tex.png'
```

So the walk visits the image and the ocean cache path and none of the other four recorded write places, with or without that flag. `EXPAND_CACHES` is a choice this version accepts and 5.0 and 5.1 do not: in those two the same call answers `ValueError: flags: 'EXPAND_CACHES' not found in ('SKIP_LINKED', 'SKIP_PACKED', 'RESOLVE_TOKEN', 'SKIP_WEAK_REFERENCES', 'SKIP_MULTIFILE', 'RELOAD_EDITED')`. 4.5 has no `file_path_foreach` at all.

**Two version-dependent shapes had to be asked of the running Blender rather than assumed**, and are recorded because they bear on anything written against this: the compositor tree is reached through `scene.compositing_node_group` in this version, and the File Output node's property is named `directory` here. (In 4.5 they are `scene.node_tree` and `base_path`.)

**Which of these settings this version has at all**, asked directly:

```
RenderSettings.use_texture_cache: True
RenderSettings.use_auto_generate_texture_cache: False
RenderSettings.use_render_cache: False
RenderSettings.use_overwrite: True
RenderSettings.use_placeholder: False
Preferences render_cache_directory:
Preferences texture_cache_directory:
generate_texture_cache in dir(bpy.ops.render): True
denoise_animation in dir(bpy.ops.cycles): True
```

The first two and the last two are this version's alone: in 4.5, 5.0 and 5.1 `use_texture_cache`, `use_auto_generate_texture_cache` and `texture_cache_directory` do not exist and `generate_texture_cache` is not in `dir(bpy.ops.render)`. One thing recorded because it was got wrong first: `hasattr(bpy.ops.render, "smoozy_no_such_operator")` returns **True**, so `hasattr` settles nothing about an operator. The lines above use `dir()`.

**Cycles' animation denoiser reads the recorded output place and stops on a frame that is not there.** Five frames were rendered to multi-layer OpenEXR with denoising data passes, frame 3 deleted, and `bpy.ops.cycles.denoise_animation()` called:

```
AFTER DELETING FRAME 3 ['dn.blend', 'f_0001.exr', 'f_0002.exr', 'f_0004.exr', 'f_0005.exr']
Error: Frame '/root/dependencies/work/denoise/5.2/f_0003.exr' not found, animation must be complete
```

In this version `OPEN_EXR_MULTILAYER` is not in the file-format list until `image_settings.media_type = 'MULTI_LAYER_IMAGE'` is set first; in 4.5 the format can be chosen straight away.

**Observed — the Mac half of this round's hand tests, run 2026-08-07 on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card.** Blender 5.2.0 LTS was started from a terminal, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and this Blender answered `RESOURCES …/depwork/res-5.2` and `VERSION 5.2.0 LTS`. The application used was `/Applications/Blender.app/Contents/MacOS/Blender`. Every render used Cycles on the processor. **Other topics were running Blender on this same Mac throughout, so nothing was timed and no memory was measured; there is no timing or memory reading anywhere below.** **This is an addition to a document written earlier in this round; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

In the machine's own words quoted below, the working folder's long path is written `…/depwork`; nothing else in any quotation is altered.

Pictures were compared as pictures, never as files. Before that was leaned on, two renders of the same frame of the same file were compared on this Mac and gave `largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE`, so any difference reported below is a real difference.

**Everything the Linux half found about the output place holds on macOS too.** Taken in order:

*A missing output folder is made.* The same file rendered with `--background --factory-startup <file> --render-anim`, and again with `--render-output <workdir>/deep3/deep4/frame_ --render-frame 1` into two folders that did not exist. Both worked, exit status 0, nothing said about the folder:

```
EXIT STATUS 0
00:00.289  blend            | Read blend: "…/depwork/q17/5.2/render.blend"
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
00:00.458  render           | Saved: '…/depwork/q17/5.2/deep3/deep4/frame_0001.png'
00:00.458  render           | Time: 00:00.16 (Saving: 00:00.05)
Blender quit
FILES:
…/depwork/q17/5.2/deep3
…/depwork/q17/5.2/deep3/deep4
…/depwork/q17/5.2/deep3/deep4/frame_0001.png
```

*The output place is looked at before writing.* Five frames rendered, frames 2 and 4 deleted, `use_overwrite` set to False, the animation rendered again with no window:

```
00:00.724  render           | Skipping existing frame "…/depwork/q17/5.2/ow/frame_0001.png"
00:01.041  render           | Saved: '…/depwork/q17/5.2/ow/frame_0002.png'
00:01.041  render           | Skipping existing frame "…/depwork/q17/5.2/ow/frame_0003.png"
00:01.535  render           | Saved: '…/depwork/q17/5.2/ow/frame_0004.png'
00:01.535  render           | Skipping existing frame "…/depwork/q17/5.2/ow/frame_0005.png"
```

and only the two deleted files carried a new modification time afterwards:

```
before   frame_0001.png 1786065801 8598     after   frame_0001.png 1786065801 8598
         frame_0003.png 1786065801 8598             frame_0002.png 1786065806 8598
         frame_0005.png 1786065801 8598             frame_0003.png 1786065801 8598
                                                    frame_0004.png 1786065806 8598
                                                    frame_0005.png 1786065801 8598
```

*An empty file counts as a frame already done.* With `use_overwrite` off and `use_placeholder` on, five zero-length files were put at the five names `frame_path()` gave:

```
--- five zero-length files placed ---
frame_0001.png 0   frame_0002.png 0   frame_0003.png 0   frame_0004.png 0   frame_0005.png 0
--- render the animation, no window ---
EXIT 0
00:00.320  render           | Skipping existing frame "…/depwork/q17/5.2/ph/frame_0001.png"
00:00.320  render           | Skipping existing frame "…/depwork/q17/5.2/ph/frame_0002.png"
00:00.320  render           | Skipping existing frame "…/depwork/q17/5.2/ph/frame_0003.png"
00:00.320  render           | Skipping existing frame "…/depwork/q17/5.2/ph/frame_0004.png"
00:00.320  render           | Skipping existing frame "…/depwork/q17/5.2/ph/frame_0005.png"
00:00.321  reports          | No frames rendered, skipped to not overwrite
--- sizes after ---
frame_0001.png 0   frame_0002.png 0   frame_0003.png 0   frame_0004.png 0   frame_0005.png 0
```

Nothing was rendered, all five were still zero length, and the run still reported success.

*A bad path template stops a render started from outside, for a single frame as well as for an animation*, with a sound template as the control:

```
=== a GOOD template, as a control ===
PATH SET TO '…/depwork/q17/5.2/tmpl/{blend_name}_'
FRAMEPATH OK '…/depwork/q17/5.2/tmpl/render_0001.png'
EXIT 0
00:00.472  render           | Saved: '…/depwork/q17/5.2/tmpl/tmplok_0001.png'
=== the BAD template ===
PATH SET TO '…/depwork/q17/5.2/tmpl/{blend_name.png'
FRAMEPATH RAISED RuntimeError Error: Parse errors in path '…/depwork/q17/5.2/tmpl/{blend_name.png':
--- single frame ---
EXIT 1
00:00.275  reports          | ERROR Parse errors in path '…/depwork/q17/5.2/tmpl/{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
--- animation ---
EXIT 1
00:00.275  reports          | ERROR Parse errors in path '…/depwork/q17/5.2/tmpl/{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
--- what is in the folder ---
tmplok_0001.png
```

One thing about that control worth recording, because it is a difference in how the test was set up and not a difference in Blender: on this Mac `frame_path` was asked **before** the file had been saved under its new name, so `{blend_name}` still stood for the old name and it answered `render_0001.png`. The file that appeared on disk when the render ran was `tmplok_0001.png`, which is what the template says.

*The recorded path beats one given before the file on the command line.*

```
--- output given BEFORE the file ---
00:00.434  render           | Saved: '…/depwork/q17/5.2/tmplsrc/out/frame_0001.png'
--- output given AFTER the file ---
00:00.433  render           | Saved: '…/depwork/q17/5.2/ord/after/x_0001.png'
--- what landed where ---
…/depwork/q17/5.2/ord/after/x_0001.png
```

*`frame_path()` names the file that appears, for a frame render and for an animation — and not for a still render.* With the output path `shot_{blend_name}_##_`:

```
PATH '…/depwork/q17/5.2/fp2/shot_{blend_name}_##_'
PREDICTED frame 1: …/depwork/q17/5.2/fp2/shot_fp2_01_.png
PREDICTED frame 7: …/depwork/q17/5.2/fp2/shot_fp2_07_.png
--- render frame 7 from the command line ---
Saved: '…/depwork/q17/5.2/fp2/shot_fp2_07_.png'
--- render the animation ---
Saved: '…/depwork/q17/5.2/fp2/shot_fp2_01_.png' … '…/depwork/q17/5.2/fp2/shot_fp2_05_.png'
ON DISK: shot_fp2_01_.png shot_fp2_02_.png shot_fp2_03_.png shot_fp2_04_.png shot_fp2_05_.png shot_fp2_07_.png
```

but through `bpy.ops.render.render(write_still=True)` at frame 7, on the same path:

```
PATH '…/depwork/q17/5.2/fp/shot_{blend_name}_##_'
PREDICTED …/depwork/q17/5.2/fp/shot_render_07_.png
ON DISK ['shot_render_##_.png']
MATCH False
```

*The file-wide path commands reach an ocean cache path and nothing else recorded for writing.* The same six recorded locations were built into one file — an image loaded from disk, the render output path, a compositor File Output node's directory, a point cache with External on, a geometry-nodes bake directory, and an ocean modifier's cache path:

```
TREE ATTR scene.compositing_node_group | FILE OUTPUT NODE PROPERTY directory
--- as saved ---
  image ReadTex   : …/depwork/q17paths/5.2/assets/tex.png
  render output   : …/depwork/q17paths/5.2/renders/frame_
  file output node: …/depwork/q17paths/5.2/fileout
  point cache     : …/depwork/q17paths/5.2/ptcache
  gn bake dir     : …/depwork/q17paths/5.2/gnbake
  ocean cache     : …/depwork/q17paths/5.2/oceanbake
MAKE PATHS RELATIVE -> {'FINISHED'}
--- after Make Paths Relative ---
  image ReadTex   : //assets/tex.png
  render output   : …/depwork/q17paths/5.2/renders/frame_
  file output node: …/depwork/q17paths/5.2/fileout
  point cache     : …/depwork/q17paths/5.2/ptcache
  gn bake dir     : …/depwork/q17paths/5.2/gnbake
  ocean cache     : //oceanbake
MAKE PATHS ABSOLUTE -> {'FINISHED'}
--- after Make Paths Absolute ---
  image ReadTex   : …/depwork/q17paths/5.2/assets/tex.png
  … the other four unchanged …
  ocean cache     : …/depwork/q17paths/5.2/oceanbake
```

With all six pointed at a folder that does not exist, `bpy.ops.file.report_missing_files()` named two of them:

```
Warning: Path '…/depwork/q17paths/5.2/gone' not found, from local data-block 'OBGrid'
Warning: Path '…/depwork/q17paths/5.2/gone/tex.png' not found, from local data-block 'IMReadTex'
REPORT MISSING FILES -> {'FINISHED'}
BLEND PATHS: ['…/depwork/q17paths/5.2/gone', '…/depwork/q17paths/5.2/gone/tex.png', '…/depwork/q17paths/5.2/assets/tex.png']
```

`OBGrid` carries the ocean modifier and `IMReadTex` is the image. The render output path, the File Output node, the point cache and the geometry-nodes bake directory were named by nothing.

One thing about the set-up, recorded because it had to be worked around: on this Mac, **saving the file turned the image path and the ocean cache path into relative ones by itself**, which would have left Make Paths Relative with nothing to do and half the test blind. All six were therefore set back to absolute after the save, and only then were the two commands run. What is quoted above is that second run.

*Cycles' animation denoiser reads the recorded output place and stops on a frame that is not there.* Five frames rendered to multi-layer OpenEXR with denoising data passes, frame 3 deleted, and `bpy.ops.cycles.denoise_animation()` called:

```
MEDIA TYPE SET TO MULTI_LAYER_IMAGE
FILE FORMAT OPEN_EXR_MULTILAYER
RENDERED ['dn.blend', 'f_0001.exr', 'f_0002.exr', 'f_0003.exr', 'f_0004.exr', 'f_0005.exr']
AFTER DELETING FRAME 3 ['dn.blend', 'f_0001.exr', 'f_0002.exr', 'f_0004.exr', 'f_0005.exr']
Error: Frame '…/depwork/denoise/5.2/f_0003.exr' not found, animation must be complete
DENOISE ANIMATION RAISED RuntimeError Error: Frame '…/depwork/denoise/5.2/f_0003.exr' not found, animation must be complete
```

*Which of these settings this version has at all*, asked directly on this machine:

```
RenderSettings.use_texture_cache                True
RenderSettings.use_auto_generate_texture_cache  False
RenderSettings.use_render_cache                 False
RenderSettings.use_overwrite                    True
RenderSettings.use_placeholder                  False
PointCache.compression                          'none'
Preferences render_cache_directory              ''
Preferences texture_cache_directory             ''
generate_texture_cache in dir(bpy.ops.render)   True
denoise_animation in dir(bpy.ops.cycles)        True
merge_images in dir(bpy.ops.cycles)             True
hasattr(bpy.ops.render, 'smoozy_no_such_operator') -> True
```

**The path walk visits the same three things it visited on Linux, and this is the version that takes `EXPAND_CACHES`.** The function's own description was printed before it was called, and in this version it names the third argument as a real object rather than a placeholder:

```
HAS file_path_foreach: True
HAS file_path_map    : True
   :param visit_path_fn: function that takes three parameters: the data-block, a file path, and a :class:`bpy.types.BlendDataPathMeta` metadata object.
file_path_foreach RAISED TypeError <lambda>() takes 1 positional argument but 3 were given
FOREACH flags=None visited 3:
    Grid '//oceanbake'
    ReadTex '//assets/tex.png'
    Tex '//assets/tex.png'
FOREACH flags={'EXPAND_CACHES'} visited 3:
    Grid '//oceanbake'
    ReadTex '//assets/tex.png'
    Tex '//assets/tex.png'
```

(The one-argument call is kept because it is what was typed first, before the description had been read.) The walk reached the image and the ocean cache path and none of the other four recorded write places, and `EXPAND_CACHES` made no difference to what it reached.

**The texture cache behaves differently on this Mac from the way it behaved on Linux, and this is a disagreement between the two machines rather than between two versions.** The same scene — one image used as an emission colour on a plane, Cycles on the processor, 16 samples, denoising off, 120 by 120 — was rendered three times:

```
this version has a texture cache setting: True
cacheoff:      use_texture_cache=False auto_generate=False -> blender_tx folder: (none)
cacheon_nogen: use_texture_cache=True  auto_generate=False -> blender_tx folder: (none)
cacheon_gen:   use_texture_cache=True  auto_generate=True  -> blender_tx folder: ['checker.png.0-926BDA8569F1F2504902F06ABA51E46D.tx']
  cache off against cache on with no file made:  channels differing=0 of 57600 — SAME PICTURE
  cache off against cache on with the file made: largest channel difference=0.090196 channels differing=10854 of 57600 — DIFFERENT PICTURE
```

The name of the cache file is character for character the one Linux made. **On Linux the third comparison was `SAME PICTURE`; on this Mac it is a different picture.** Because that is a disagreement, it was checked again with the cache file already sitting there before any render:

```
blender_tx before this run: ['checker.png.0-926BDA8569F1F2504902F06ABA51E46D.tx']
  cache off against cache off again:                 channels differing=0 of 57600 — SAME PICTURE
  cache off against cache on, no generating:         largest channel difference=0.090196 channels differing=10854 of 57600 — DIFFERENT PICTURE
  cache on with the file made, against the same again: channels differing=0 of 57600 — SAME PICTURE
```

So on this Mac the difference is steady, it is reversible — turning the cache off gives the first picture back exactly — and it appears whenever a cache file exists to be read, whether or not generating is on. Nothing failed in any of the runs. The size of the difference is small: the largest single channel moves by 0.09 out of 1.

### What the window could be asked, and what it could not

The Creator's Mac has a screen, so this is the machine where the drawing questions can be reached. Blender was started there with a window and a `--python` script, and the script did its work as it ran. **One limit shaped all of it and is recorded because it decides what could and could not be asked**: when Blender is started this way from a terminal on this machine, the window is created and drawn — the screenshots below are of a fully drawn Blender — but **its event loop never runs**. A timer registered with `bpy.app.timers.register` never fired once in any attempt, `bpy.ops.wm.redraw_timer` never returned, and `bpy.ops.wm.quit_blender()` never quit, so each script ended the process itself after flushing what Blender had printed. So anything needing an event — hovering over a control, or a redraw after something is changed — could not be reached, and anything the window already shows when the file opens could.

**The path field is drawn red for a bad template.** A file whose render output path reads `//out/{blend_name.png` was saved with its Properties editor already on the Output tab, opened with a window, and photographed with Blender's own `bpy.ops.screen.screenshot`. The same was done with `//out/{blend_name}.png`, a sound template, as the control. Reading the average colour of the strip of the picture holding the path field:

```
the bad template   average colour r=0.352 g=0.174 b=0.174
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
00:00.705  render           | Saved: '…/depwork/q17/5.2/wow/frame_0002.png'
00:00.800  render           | Saved: '…/depwork/q17/5.2/wow/frame_0004.png'
```

Only the two missing frames carry a new modification time. **And this version says nothing about it.** With no window it prints `Skipping existing frame` for each of the three; in the window run those lines are not printed at all, though the frames are plainly skipped. The console output was flushed before the process ended, so this is not a lost buffer.

**The Animation Player does play what is at the recorded output place.** With five frames sitting at the recorded place, a script called `bpy.ops.render.play_rendered_anim()` and then listed the running processes:

```
play_rendered_anim() -> {'FINISHED'}
    51823 /Applications/Blender.app/Contents/MacOS/Blender --gpu-backend metal -a -f 24 1.0 -s 1 -e 5 -j 1 -c 4096 …/depwork/q17/5.2/wow/frame_0001.png
```

So the player is a second Blender started with `-a`, and the last thing it is handed is the first frame at the file's own recorded output place. As a control the same was done with the recorded place pointing at an **empty** folder. That control was run once, in Blender 4.5 on this same Mac, and its own words are quoted here; the player was still started at that place and said so:

```
wm_main_playanim_intern: '…/depwork/q17/4.5/emptyout/frame_0001.png' not an image file
```

so the operator reads the recorded place rather than doing something generic.

**The viewport render uses the recorded output place.** With the recorded place an empty folder, a script called `bpy.ops.render.opengl(animation=True)` with the 3D viewport as its context:

```
render output path: '…/depwork/q17/5.2/glout/gl_'
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

**LABEL: not in the sources**, for the general case. Two particular cases are **documented**, and one more is **documented** only for frames outside a bake's range. Everything else I looked for is silence, and the record of the looking is below.

**Reading chosen.** I read the question as: the file records that a cache or a bake exists (or records a location where one should be), and at render time that data is not there. Ambiguous — it could also be read narrowly as "the recorded cache *file* is missing from disk", which is close to question 12's ground. My answer covers the wider reading and says where the narrow one is unanswered.

### What is documented

**EEVEE light probe volumes.** This is the one clean, direct answer I found:

> "If an object is not inside any Irradiance Volume, or if the indirect lighting has not been baked,
> the world's diffuse lighting will be used to shade it."

Source: manual, *Light Probe Volume*. Address: https://docs.blender.org/manual/en/5.2/render/eevee/light_probes/volume.html — read 2026-07-30. **inferred.** *(Changed by the citation check, 2026-08-06: this read "documented". The quoted sentence says "Irradiance Volume" while the page is headed "Light Probe Volume", and nothing quoted says the two names mean the same thing, so a reader has to supply that step.)* The render does not fail and does not stop; it shades with the world's diffuse lighting instead.

That bake lives inside the file rather than on disk, which the same page states:

> "Light probe volume light data is static and needs to be manually baked.
> Once baked, the data is stored inside the object data-block and can be moved, animated and linked
> between blender files."

By contrast, sphere probes are not a stored bake at all at render time:

> "In the viewport, capture only happens if an update is detected on the light probe data or position.
> For renders, the capture happens at the start of each frame."

Source: manual, *Light Probe Sphere*. Address: https://docs.blender.org/manual/en/5.2/render/eevee/light_probes/sphere.html — read 2026-07-30. **documented.**

**Cycles texture cache (new in 5.2).** When the cache files are missing, the render can make them:

> "use_auto_generate_texture_cache
>
> Automatically create tx files from image files when rendering, if the files do not exist or are outdated. The path to store the texture cache files is configured in the preferences (default False)"

Source: Python API, `RenderSettings`. Address: https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30. **documented.**

The manual says the same:

> "Auto Generate
>
> Automatically generate and update .tx files for images when rendering.
> Files are regenerated when the source image is modified."

Source: manual, *Performance* (Cycles). Address: https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html — read 2026-07-30.

And the release note, which is an explicit statement of what changed in this version:

> "On scenes with many image textures, the texture cache significantly reduces memory usage and startup time. This comes at the cost of a small rendering performance impact, and increase disk space usage."
>
> "To get started, enable Performance > Texture Cache and Auto Generate in the render properties. For every image, a corresponding tx file will be generated in the `blender_tx/` folder next to the image file."

Source: release notes, *Cycles*. Address: https://developer.blender.org/docs/release_notes/5.2/cycles/ — read 2026-07-30.

Caveat worth naming: the *location* of this cache is a preference, not something the `.blend` file records — "The directory for storing tx files generated from image files… Leave blank to store tx files in the same directory as image files" (Python API, `PreferencesFilePaths`, https://docs.blender.org/api/5.2/bpy.types.PreferencesFilePaths.html, read 2026-07-30). What the file records is whether the cache is used (`use_texture_cache`, "default True") and whether missing files are generated (`use_auto_generate_texture_cache`, "default False"). What happens when the cache is on, auto-generate is off, and the `.tx` file is absent — **not in the sources**.

**Ocean modifier, frames outside the baked range.** A partial answer:

> "The start and end frames of the bake are repeated when accessing frames outside of the baked range."

Source: manual, *Ocean Modifier*. Address: https://docs.blender.org/manual/en/5.2/modeling/modifiers/physics/ocean.html — read 2026-07-30. **documented**, but only for frames outside the range of a bake that *does* exist. What happens when the baked EXRs are gone altogether is not stated; the API only exposes the state, not the fallback: "is_cached — Whether the ocean is using cached data or simulating (default False, readonly)" (Python API, `OceanModifier`, https://docs.blender.org/api/5.2/bpy.types.OceanModifier.html, read 2026-07-30).

### What is not in the sources

**Physics point caches — particles, cloth, soft body, rigid body.** No page says what a render does when there is no bake. What the sources do say, and it is only adjacent:

> "The result of a simulation is automatically cached to memory when the animation is played,
> so that the next time it runs, it can be replayed more quickly by reading the results from the memory."

> "It is generally recommended to bake your physics simulations before rendering.
> Aside from no longer needing to go through the time-consuming process of simulating again,
> baking can help prevent potential glitches and ensure that the outcome of the simulation
> remains exactly the same every time."

Source: manual, *Baking Physics Simulations*. Address: https://docs.blender.org/manual/en/5.2/physics/baking.html — read 2026-07-30.

> "When an animation is played, each physics system writes each frame to the cache.
> Note that for the cache to fill up,
> one has to start the playback before or on the frame that the simulation starts."

> "The simulation is only calculated for positive frames
> in between the Start and End frames of the Cache panel, whether you bake or not."

Source: manual, *Cache* (particles). Address: https://docs.blender.org/manual/en/5.2/physics/particles/emitter/cache.html — read 2026-07-30.

> "Note
>
> If there are particles or other physics system in a scene,
> be sure to bake them before rendering,
> otherwise you might not get correct or consistent motion blur."

Source: manual, *Motion Blur* (Cycles). Address: https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/motion_blur.html — read 2026-07-30.

**inferred**, and I flag it as weaker than it looks: from "avoid the time-consuming process of simulating again" and "the simulation is only calculated for positive frames between Start and End, whether you bake or not", it follows that with no bake the simulation is worked out again as frames are evaluated. The step a reader has to supply is that a render evaluates frames the way playback does — the sources describe the filling of the cache for *playback*, never for a render. The warnings about "potential glitches", results not being "exactly the same every time", and motion blur that is not "correct or consistent" say plainly that the outcome may be wrong, but not what it will be, and not whether anything is reported.

**Geometry-nodes simulation zones and bake nodes.** No statement of what happens when the bake is missing. The nearest:

> "The simulation is automatically cached during playback."

> "When the result is ready to be sent to a render-farm, it can be baked to disk.
> This allows for the simulation to be rendered in a non-sequential order."

Source: manual, *Simulation Nodes*. Address: https://docs.blender.org/manual/en/5.2/physics/simulation_nodes.html — read 2026-07-30. The same two sentences also appear in the manual's *Simulation Zone* page, https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/simulation/simulation_zone.html.

**inferred**: since baking to disk is what "allows for the simulation to be rendered in a non-sequential order", without it the frames must be reached in order. The step supplied: the sources do not say what a render out of order produces without a bake — only that the bake is what makes it possible.

**Fluid (Mantaflow) caches.** Nothing on a missing cache at render time. The only related statement is about one cache type rebuilding itself during viewport playback: "Replay: The cache will be baked as the simulation is being played in the viewport." (manual, *Cache* (fluid domain), https://docs.blender.org/manual/en/5.2/physics/fluid/type/domain/cache.html, read 2026-07-30). That is the viewport, not a render.

**Alembic / USD / VDB sequences** — caches a file records by path. I found nothing about a render meeting a missing one. The volume page describes only what happens outside a sequence's frame range ("Clip: Show nothing", manual, *Volume Properties*, https://docs.blender.org/manual/en/5.2/modeling/volumes/properties.html, read 2026-07-30), which is a different situation.

**Reporting.** Nothing says a render reports a missing cache or bake. The only place I found where the state of a cache is shown at all is the timeline, which is inside the interface: "Baked simulations will be shown as fully opaque, cached simulations will be slightly transparent, and invalid caches will be slightly transparent with dark diagonal stripes." (manual, *Navigating* (Dope Sheet), https://docs.blender.org/manual/en/5.2/editors/dope_sheet/navigating.html, read 2026-07-30).

**Not covered.**
- Whether a render errors, warns, or silently continues in any of the silent cases above. I found no statement either way.
- Whether the *file* being gone from disk differs from the bake never having been made. No source distinguishes these.
- Whether the answer differs between Cycles and EEVEE for physics caches. Only the EEVEE light probe case is engine-specific and documented.
- Whether a render pointed at a shared or networked cache location behaves differently when the location is unreachable rather than empty.
- What a render does when only *some* frames of a bake are present.

**Outside the interface.** **Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** The sources do not say, for the physics and geometry-nodes cases — everything they describe is phrased around playback and the viewport. Two things do carry over:
- The EEVEE light probe volume fallback is stated as a shading rule, not an interface behaviour, so nothing in it depends on a window being open. **inferred**, from the absence of any interface condition in the sentence.
- The texture cache can be built ahead of a render from outside entirely: "blender scene.blend --command maketx" and "blender --command maketx image.png --colorspace sRGB" (release notes, *Cycles*, address above, read 2026-07-30; the same two commands appear in the manual's Cycles *Performance* page). Cycles' own file confirms the command exists and what it does when no image path is given:

> "Generate Cycles texture cache (.tx) files.\n"
> "\n"
> "When a file path is given, generate a tx file for that image.\n"
> "When no file path is given, a blend file must be loaded and tx\n"
> "files are generated for all images used by shader node trees."

**observed** — file `Blender Sources/cycles_5.2/maketx.py`, lines 15–19, inside `argparse_create()`. Read 2026-07-30. The same file at lines 90–105 shows `maketx_blend()` refusing when no blend file is loaded (`parser.error("No image file path given and no blend file loaded.")`) and then calling `bpy.ops.render.generate_texture_cache()`. What a render *does* with the result is a step beyond what the file says.

**Operating system.** **Since settled by a hand test — see the observed finding at the end of this answer. What follows is what the sources alone said.** The sources do not say for any of the cases above. The one operating-system difference I found anywhere near this ground is about a cache failing to be written in the first place, not about a render meeting an absent one: "If you are not allowed to write to the required subdirectory caching will not take place, e.g. if your blend-file path is very long and your operating system has a limit on the path length that is supported." (manual, *Cache* (particles), address above, read 2026-07-30).

---

## Where I looked

Everything below is inside `Blender Sources/`. I searched the whole of `manual_5.2` and `api_5.2` at once before opening any page, as instructed, and searched `release_notes_5.2` file by file (there are only twenty).

**Search terms tried across the whole manual and the whole API** (case-insensitive): `placeholder`, `overwrite`, `bake`, `cache`, `not baked`, `has not been baked`, `no cache`, `cache is missing`, `missing cache`, `without baking`, `must be baked`, `need(s) to be baked`, `before rendering`, `unbaked`, `stale`, `outdated`, `invalid cache`, `cache file not`, `empty cache`, `cache empty`, `not found`, `does not exist`, `absent`, `missing`, `falls back`, `fall back`, `recalculat`, `resimulat`, `simulated again`, `from the start`, `first frame`, `non-sequential`, `out of order`, `render farm`, `resume`, `reuse`, `skip frames`, `already rendered`, `existing frames`, `output path`, `output directory`, `output folder`, `render output`, `proxy`, `Proxy Directory`, `Custom Directory`, `texture cache`, `Make Paths Absolute`, `Make Paths Relative`, `Report Missing Files`, `Find Missing Files`, `path template`, `frame_path`, `play_rendered_anim`, `file_path_foreach`, `use_external`, `is_cached`, `is_outdated`.

**Pages opened in full or in part (manual, 5.2):** `render/output/properties/output`, `render/output/animation`, `render/output/animation_player`, `render/output/introduction`, `render/output/index` listing, `render/cycles/render_settings/performance`, `render/cycles/render_settings/motion_blur`, `render/eevee/light_probes/volume`, `render/eevee/light_probes/sphere`, `render/eevee/light_probes/index`, `physics/baking`, `physics/introduction`, `physics/simulation_nodes`, `physics/particles/emitter/cache`, `physics/cloth/settings/cache`, `physics/fluid/type/domain/cache`, `physics/fluid/introduction`, `physics/rigid_body/world`, `physics/rigid_body/tips`, `physics/soft_body/introduction`, `modeling/modifiers/physics/ocean`, `modeling/modifiers/geometry_nodes`, `modeling/modifiers/modify/mesh_sequence_cache`, `modeling/geometry_nodes/baking`, `modeling/geometry_nodes/geometry/operations/bake`, `modeling/geometry_nodes/simulation/simulation_zone`, `modeling/volumes/properties`, `compositing/types/output/file_output`, `editors/video_sequencer/sequencer/sidebar/proxy`, `editors/preferences/file_paths`, `editors/dope_sheet/navigating`, `editors/3dview/viewport_render`, `interface/window_system/topbar`, `files/file_paths`, `advanced/command_line/arguments`, `advanced/command_line/render`, `troubleshooting/` (whole folder listed and searched).

**Pictures opened:** `_images/render_output_properties_output_panel.png` (Output panel — settles the names Overwrite / Placeholders / File Extensions / Cache Result and that Overwrite is ticked by default) and `_images/files_file_paths-invalid_path_template_tooltip.png` (a bad template in the Output field, the field red, the tooltip listing the syntax error).

**Pages opened (API, 5.2):** `bpy.types.RenderSettings`, `bpy.types.PointCache`, `bpy.types.PointCacheItem`, `bpy.types.OceanModifier`, `bpy.types.BlendData`, `bpy.types.BlendDataPathMeta`, `bpy.types.PreferencesFilePaths`, `bpy.ops.render`, `bpy.ops.object` (ocean_bake), `bpy_types_enum_items/file_path_foreach_flag_items`.

**Release notes read in full:** `cycles.md`, `rendering.md`, `pipeline_io.md`, `physics.md`, plus grep of all twenty files for `cache`, `bake`, `output path`, `overwrite`, `placeholder`. `python_api.md` read around the Path Iterator section.

**Cycles' own files:** listed the whole of `cycles_5.2`; read `maketx.py` in full; grepped `properties.py`, `ui.py`, `operators.py`, `__init__.py` and the whole `source/` tree for `texture_cache`, `maketx`, `.tx`. Ruled out `source/` as a place answers could come from for these two questions: it holds only `kernel/` and `util/`, which contain the rendering kernel, not image loading, cache handling or output writing.

**Ruled out, and why:**
- Blender's requirements page — it speaks about hardware and operating systems, and names nothing about output locations or caches. Not consulted.
- `addons/` pages in the manual — the hits for "placeholder" and "bake" there are the glTF exporter and Rigify, neither about recorded write locations or render-time caches.
- `contribute/` pages — hits are about writing the manual itself.
- Everything the searches turned up under `grease_pencil/`, `sculpt_paint/`, `animation/` for "bake" is baking of *animation channels* (motion paths, keyframes, NLA), not a cache on disk, so outside both questions.
- Question 12's ground (what a render does about a missing file it points at, and whether it says so) I deliberately did not go hunting into, beyond the cache-specific material above.

---

**Observed — the hand tests for this round, run 2026-08-07 on a rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and 118 GB of free disk, and no display server at all.** Blender 5.2 was started from a terminal with no window, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and the Blender answered `RESOURCES /root/dependencies/res-5.2`. Every render used Cycles on the processor, so nothing here depends on the graphics card. The machine was shared with other topics' work throughout, so nothing here is a timing or memory measurement of a clean machine except where it says what it did about that. The full list of tests, with every set-up written out, is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

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
use_external True | filepath /root/dependencies/work/q18/5.2/emptycache | name 'SmoozyCache'
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
BEFORE BAKE | cache_directory: /root/dependencies/work/q18fluid/5.2/fluidcache | has_cache_baked_any: False | has_cache_baked_data: False
BAKE ALL -> {'FINISHED'}
AFTER BAKE  | cache_directory: /root/dependencies/work/q18fluid/5.2/fluidcache | has_cache_baked_any: True | has_cache_baked_data: True
cache folder: ['config', 'data']
cache folder removed; exists now: False
SAME SESSION, FOLDER GONE | has_cache_baked_any: True | has_cache_baked_data: True
REOPENED with the cache folder gone (folder exists: True )
  has_cache_baked_any: True | has_cache_baked_data: True | has_cache_baked_noise: True | has_cache_baked_mesh: True | has_cache_baked_particles: True
```

and what the reopening had put back on disk:

```
/root/dependencies/work/q18fluid/5.2/fluidcache/config/config_0001.uni
/root/dependencies/work/q18fluid/5.2/fluidcache/data/fluid_data_0001.vdb
```

**An EEVEE light probe volume with no bake does render differently.** A closed room with its faces turned inward so the world cannot light it directly, a sphere inside, a light probe volume covering it, and a strong green world. Rendered unbaked, then with `bpy.ops.object.lightprobe_cache_bake(subset='ALL')`, then after `bpy.ops.object.lightprobe_cache_free(subset='ALL')`:

```
ENGINE BLENDER_EEVEE
  unbaked against baked: largest channel difference=0.768628 channels differing=24818 of 57600 — DIFFERENT PICTURE
  baked against freed  : largest channel difference=0.768628 channels differing=24818 of 57600 — DIFFERENT PICTURE
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

**Cycles' texture cache, which is this version's alone, does not need its files.** `scene.render.use_texture_cache`, `use_auto_generate_texture_cache`, the `texture_cache_directory` preference and `bpy.ops.render.generate_texture_cache` all exist here and in none of 4.5, 5.0 or 5.1, so this is a real difference between the versions and not a gap in the others' documentation. A plane lit by a checked image was rendered three times:

```
this version has a texture cache setting: True
cacheoff:      use_texture_cache=False auto_generate=False -> blender_tx folder: (none)
cacheon_nogen: use_texture_cache=True  auto_generate=False -> blender_tx folder: (none)
cacheon_gen:   use_texture_cache=True  auto_generate=True  -> blender_tx folder: ['checker.png.0-926BDA8569F1F2504902F06ABA51E46D.tx']
  cache off against cache on with no file made:  channels differing=0 of 57600 — SAME PICTURE
  cache off against cache on with the file made: channels differing=0 of 57600 — SAME PICTURE
```

So with the cache on, generating off and no `.tx` file anywhere, the render finished and gave exactly the picture it gives with the cache off; with generating on, a `.tx` file appeared in a `blender_tx` folder beside the image and the picture was still the same.

**A disk point cache written by 4.5 with a compression 5.0 dropped is announced.** The cloth file was baked in Blender 4.5 with `point_cache.compression = 'LIGHT'` and Disk Cache on, and the same saved file was then opened and rendered here:

```
EXIT 0
is_baked True | is_outdated False | is_frame_skip False | use_disk_cache True | info '60 frames on disk.'
Warning: Cloth Cache in object Grid can not be read because it uses an outdated compression method. You need to delete the caches and re-bake.
render EXIT 0
00:00.482  reports          | WARNING Cloth Cache in object Grid can not be read because it uses an outdated compression method. You need to delete the caches and re-bake.
00:00.689  render           | Saved: '/root/dependencies/work/oldcache/from5.2_0060.png'
PICTURES size=(160, 160) largest channel difference=1.000000 channels differing=4899 of 102400
DIFFERENT PICTURE
```

This is the only case in all of these tests where a cache that cannot be used is named on the console of a run with no window. The exit status is still 0, `is_baked` still reads `True`, and the picture differs from the one 4.5 renders with the same cache. 5.0 and 5.1 printed the same words. `PointCache.compression` exists in 4.5 with the choices `NO`, `LIGHT` and `HEAVY`, and does not exist in this version at all.

**Observed — the Mac half of this round's hand tests, run 2026-08-07 on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card.** Blender 5.2.0 LTS was started from a terminal, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and this Blender answered `RESOURCES …/depwork/res-5.2` and `VERSION 5.2.0 LTS`. The application used was `/Applications/Blender.app/Contents/MacOS/Blender`. Every render used Cycles on the processor. **Other topics were running Blender on this same Mac throughout, so nothing was timed and no memory was measured; there is no timing or memory reading anywhere below.** **This is an addition to a document written earlier in this round; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

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
use_external True | filepath …/depwork/q18/5.2/emptycache | name 'SmoozyCache'
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
…/depwork/q18fluid/5.2/fluidcache/config/config_0001.uni
…/depwork/q18fluid/5.2/fluidcache/data/fluid_data_0001.vdb
```

*An EEVEE light probe volume with no bake does render differently, and freeing the bake puts the render back exactly.*

```
ENGINE BLENDER_EEVEE | EEVEE names offered: ['BLENDER_EEVEE']
  unbaked against baked: largest channel difference=0.815686 channels differing=36947 of 57600 — DIFFERENT PICTURE
  baked against freed  : largest channel difference=0.815686 channels differing=36947 of 57600 — DIFFERENT PICTURE
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

2026-08-06 — Opus 5 (Claude Code) — Dependencies, Blender 5.2
