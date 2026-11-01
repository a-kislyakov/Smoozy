# Writing — Blender 5.0 — second pass (question 6 only)

**Sources consulted, and when.** Blender 5.0 manual (`Blender Sources/manual_5.0`, including images in its `_images` folder), Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own files (`Blender Sources/cycles_5.0`). All read on 2026-07-30 (the snapshot date given to me).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 6. What are the ways a render can be started, and what does each of them write?

**Reading of the question.** I read "a render" as a final render of a scene — the thing that produces an image, a frame sequence, a video or a sound file — and "started" as the ways a person or an outside program can set one going. I have listed every starting point the sources describe, and for each one, what the sources say lands on disk. I have **not** counted baking (`bpy.ops.object.bake`, fluid, dynamic paint), the animation player, or interactive viewport shading as "a render being started"; I say below why, so you can disagree with that line.

The answer below is in parts. Each part carries its own label, because the sources are much more explicit about some starting points than others.

---

### 6a. Render Image (F12) — from the interface

**Label: documented** (that the way exists) / **inferred** (that it writes nothing on its own).

The Render menu in the top bar lists:

> "Render Image F12"
>
> "Render the active scene at the current frame."

— manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html, read 2026-07-30.

The default keymap page repeats it:

> "F12"
>
> "Render the current frame."

— manual, https://docs.blender.org/manual/en/5.0/interface/keymap/blender_default.html, read 2026-07-30.

Neither says a file is written. The nearest the sources come to saying it is not is on the animations page:

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

— manual, https://docs.blender.org/manual/en/5.0/render/output/animation.html, read 2026-07-30.

And the Output panel is described as being for animations:

> "This panel provides options for setting the location of rendered frames for animations, and the quality of the saved images."

— manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html, read 2026-07-30.

The API says the same of the output path property:

> "filepath"
>
> "Directory/name to save animations, # characters define the position and padding of frame numbers"

— Python API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html, read 2026-07-30.

The result of a still render lands in a render slot held in memory:

> "The render slot to view (and render to). You can create new renders without losing previous ones by selecting an empty slot before rendering."

— manual, https://docs.blender.org/manual/en/5.0/editors/image/introduction.html, read 2026-07-30.

**The step I took:** the sources say animations are saved automatically and that the output path is "to save animations"; they say stills are held in a buffer/slot that you then save. From that I conclude Render Image writes no image file of its own. No source says this outright, so the label is inferred, not documented.

---

### 6b. Render Animation (Ctrl-F12) — from the interface

**Label: documented.**

> "Render Animation Ctrl-F12"
>
> "Render the animation of the active scene."

— manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html, read 2026-07-30.

What it writes depends on the output format, and the sources state both cases plainly:

> "Blender creates one file that holds all the frames of your animation."

(the "Direct Approach", output set to a movie format)

> "Blender creates a file for each frame of the animation."

(the "Frame Sequence" approach, output set to a still format)

— both manual, https://docs.blender.org/manual/en/5.0/render/output/animation.html, read 2026-07-30.

And, as quoted in 6a, frames "are automatically saved directly out to a drive after being rendered."

The Media Type setting on the Output panel names the three shapes the writing takes:

> "Image:"
> "Saves each input as a separate image file. Each input can use its own image format settings."
>
> "Multi-Layer EXR:"
> "Saves all inputs together in a single multi-layer OpenEXR file."
>
> "Video:"
> "Encodes each frame into a video container. Encoding options can be found in the Encoding panel."

— manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html, read 2026-07-30.

When the output is a video, sound can go into the same file:

> "These settings change how sound is exported while rendering."

— manual, same page, read 2026-07-30.

---

### 6c. Render Sequencer Image (Alt-F12) and Render Sequencer Animation (Ctrl-Alt-F12) — from the interface

**Label: documented** (that they exist and when they appear) / **inferred** (that they write the same as 6a and 6b).

> "Render Sequencer Image Alt-F12"
>
> "If a Sequencer Scene exists that differs from the active scene, render that scene instead."
>
> "Render Sequencer Animation Ctrl-Alt-F12"
>
> "Render the animation of the Sequencer Scene."
>
> "Both of the sequencer render options automatically set the Show Sequencer Scene toggle in the image editor Render Result after rendering."

— manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html, read 2026-07-30.

The release notes state this is new and when the entries appear:

> "If the sequencer scene differs from the active scene, the Render menu will show new options for "Render Sequencer Image" /  "Render Sequencer Animation" on top of the normal scene render operators, allowing the user to specify which scene they'd like to render. Additionally, a toggle will appear in the Image Editor header for Render Results that allows swapping between scene and sequencer scene renders."

— release notes, https://developer.blender.org/docs/release_notes/5.0/sequencer/, read 2026-07-30.

**The step I took:** these two are described only as rendering a different scene, not as writing differently. So I take them to write what 6a and 6b write, using the sequencer scene's own output settings. Nothing in the sources says so; that is my inference.

---

### 6d. Render Audio — from the interface

**Label: documented.**

> "Render Audio"
>
> "Mix the scene's audio to a sound file."

— manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html, read 2026-07-30.

> "Audio can be rendered from the Render Menu."

— manual, https://docs.blender.org/manual/en/5.0/render/output/audio/introduction.html, read 2026-07-30.

The matching operator takes its own file path, not the scene output path:

> "bpy.ops.sound.mixdown(*, filepath='', check_existing=True, … container='FLAC', codec='FLAC', channels='STEREO', format='S16', mixrate=48000, bitrate=192, split_channels=False)"
>
> "Mix the scene's audio to a sound file"
>
> "filepath (string, (optional, never None)) – File Path, Path to file"

— Python API, https://docs.blender.org/api/5.0/bpy.ops.sound.html, read 2026-07-30.

It can write more than one file:

> "Split Channels"
>
> "Each audio channel will be rendered into a separate file."

— manual, https://docs.blender.org/manual/en/5.0/render/output/audio/introduction.html, read 2026-07-30.

---

### 6e. Viewport render — from the 3D Viewport's View menu

**Label: documented** (the three entries, and that the animation ones write) / **not in the sources** (whether the still one writes).

> "Viewport rendering lets you create quick preview renders from the current viewpoint (rather than from the active camera, as would be the case with a regular render)."
>
> "Viewport rendering only works for the Workbench and EEVEE render engines. It's not supported for Cycles."
>
> "To render a still image, use 3D Viewport ‣ View ‣ Render Viewport Preview."
>
> "To render an animation, use 3D Viewport ‣ View ‣ Render Playblast."
>
> "To render an animation, but only those frames that have a keyframe, use 3D Viewport ‣ View ‣ Render Playblast on Keyframes. This only renders those frames for which the selected objects have an animation key. The other frames are still written to the output, but will simply repeat the last-rendered frame."

— manual, https://docs.blender.org/manual/en/5.0/editors/3dview/viewport_render.html, read 2026-07-30.

The same page says which output settings a viewport render obeys:

> "Additionally, some output settings are used too:"
>
> "Resolution"
> "Aspect"
> "Output path"
> "File format"

— manual, same page, read 2026-07-30.

So the two animation entries write frames (or a video) to the scene's output path, in the scene's file format — the keyframes one writing *every* frame in the range, repeating the last rendered one where there is no key. Whether "Render Viewport Preview" (the still) writes a file is **not stated**; the operator behind it has a switch for it that is off by default:

> "bpy.ops.render.opengl(*, animation=False, render_keyed_only=False, sequencer=False, write_still=False, view_context=True)"
>
> "Take a snapshot of the active viewport"
>
> "write_still (boolean, (optional)) – Write Image, Save the rendered image to the output path (used only when animation is disabled)"

— Python API, https://docs.blender.org/api/5.0/bpy.ops.render.html, read 2026-07-30.

The sources do not say which arguments the menu entry passes, so I will not guess.

A release note confirms these entries were still present and were changed in 5.0:

> "Viewport Render Image and Animation now support rendering HDR output without clamping."

— release notes, https://developer.blender.org/docs/release_notes/5.0/eevee/, read 2026-07-30. (Note the release note uses the older names; the 5.0 manual uses "Render Viewport Preview" / "Render Playblast".)

---

### 6f. Sequence Render Image / Sequence Render Animation — from the Video Sequencer

**Label: documented.**

> "Sequence Render Image"
>
> "Show the current frame preview as a Render Result where you can save it as an image file."
>
> "Sequence Render Animation"
>
> "Save previews of the frames in the scene range (or the preview range, if active) to a video file or a series of image files. See the Output panel for details."
>
> "Sequence Render Image and Sequence Render Animation don't render the final video by default – specifically, they don't render Scene Strips, instead using the preview's shading mode (which is initially Solid)."
>
> "To output a video where the Scene Strips are rendered, use the Render menu in the top-bar, or change Sidebar ‣ View ‣ Scene Strip Display ‣ Shading to Rendered."

— manual, https://docs.blender.org/manual/en/5.0/editors/video_sequencer/preview/header.html, read 2026-07-30. The same two entries and wording also appear at https://docs.blender.org/manual/en/5.0/editors/video_sequencer/sequencer/navigating.html, read 2026-07-30.

So: the image one writes nothing by itself (it hands you a Render Result to save by hand); the animation one writes a video file or an image sequence to the Output panel's path.

---

### 6g. From the command line

**Label: documented.**

The manual has a page for exactly this:

> "In some situations we want to increase the render speed, access Blender remotely to render something or build scripts that use the command line."
>
> "blender -b file.blend -f 10"
>
> "-b"
> "Render in the background (without UI)."
>
> "-f 10"
> "Render only the 10th frame."
>
> "blender -b file.blend -a"
>
> "-a"
> "Render the whole animation using all the settings saved in the blend-file."
>
> "Always position -f or -a as the last arguments."

— manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html, read 2026-07-30.

The argument list is explicit that the single-frame form **saves**, which is the difference from F12:

> "-f, --render-frame <frame>"
>
> "Render frame <frame> and save it."
>
> "+<frame> start frame relative, -<frame> end frame relative."
>
> "A comma separated list of frames can also be used (no spaces)."
>
> "A range of frames can be expressed using .. separator between the first and last frames (inclusive)."

and

> "-a, --render-anim"
>
> "Render frames from start to end (inclusive)."

and, for what the animation form puts on disk:

> "blender -b animation.blend -o //render_ -F PNG -x 1 -a"
>
> "//render_ becomes //render_####, writing frames as //render_0001.png"

— all manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30.

Two traps the same page states outright, because they change *where* the writing goes:

> "Arguments are executed in the order they are given. eg:"
>
> "blender --background test.blend --render-frame 1 --render-output "/tmp""
>
> "…will not render to /tmp because --render-frame 1 renders before the output path is set."
>
> "blender --background --render-output /tmp test.blend --render-frame 1"
>
> "…will not render to /tmp because loading the blend-file overwrites the render output that was set."
>
> "blender --background test.blend --render-output /tmp --render-frame 1"
>
> "…works as expected."

— manual, same page, read 2026-07-30.

A background render also writes progress and the saved path to the log. The release notes give the shape of it:

> "00:00.244  render           | Rendering single frame (frame 1)"
> "00:00.245  render           | Start rendering: Scene, ViewLayer"
> "00:00.245  render           | Engine: Cycles"
> "00:26.799  render           | Mem: 2983M | Sample 1/16 "
> "00:26.799  render           | Mem: 2983M | Sample 16/16"
> "00:28.631  render           | Saved: '/tmp/0001.png'"
> "00:28.631  render           | Time: 00:28.38 (Saving: 00:00.15)"

— release notes, https://developer.blender.org/docs/release_notes/5.0/core/, read 2026-07-30.

That log can itself be written to a file:

> "--log-file <filepath>"
>
> "Set a file to output the log to."

— manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30.

And the release notes flag the log format as changed in 5.0:

> "The format of logging output has changed, including background render progress. Add-ons or render farms parsing this output may need to be updated."

— release notes, https://developer.blender.org/docs/release_notes/5.0/python_api/, read 2026-07-30.

One caution: `-a` means two different things depending on `-b`.

> "-a <options> <file(s)>"
>
> "Instead of showing Blender's user interface, this runs Blender as an animation player, to view movies and image sequences rendered in Blender (ignored if -b is set)."

— manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30. (This is a player, not a render, and writes nothing.)

Cycles adds its own command-line switches, which must follow a bare `--`:

> "Cycles add-on options must be specified following a double dash."
>
> "--cycles-device <device>"
> "Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL."
>
> "--cycles-print-stats"
> "Log statistics about render memory and time usage."

— manual, same page, read 2026-07-30.

I confirmed these are parsed by Cycles' own file rather than by Blender's core argument list. **Label for this next bit: observed.**

> ```
> parser.add_argument("--cycles-print-stats",
>                     help="Print rendering statistics to stderr",
>                     action='store_true')
> parser.add_argument("--cycles-device",
>                     help="Set the device to use for Cycles, overriding user preferences and the scene setting."
>                          "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
>                          "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
>                     default=None)
> ```

— Cycles' own files, `Blender Sources/cycles_5.0/engine.py`, in `_configure_argument_parser()`, lines 13–20. Read 2026-07-30. Note `--cycles-print-stats` goes to stderr, per the file's own help text — that is a write to the error stream, not to a file.

---

### 6h. From Python, inside a running Blender

**Label: documented.**

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"
>
> "Undocumented, consider contributing."
>
> "animation (boolean, (optional)) – Animation, Render files from the animation range of this scene"
>
> "write_still (boolean, (optional)) – Write Image, Save the rendered image to the output path (used only when animation is disabled)"
>
> "use_viewport (boolean, (optional)) – Use 3D Viewport, When inside a 3D viewport, use layers and camera of the viewport"
>
> "use_sequencer_scene (boolean, (optional)) – Use Sequencer Scene, Render the sequencer scene instead of the active scene"
>
> "layer (string, (optional, never None)) – Render Layer, Single render layer to re-render (used only when animation is disabled)"
>
> "scene (string, (optional, never None)) – Scene, Scene to render, current scene if not specified"
>
> "frame_start (int in [-inf, inf], (optional)) – Start Frame, Frame to start rendering animation at. If not specified, the scene start frame will be assumed. This should only be specified if doing an animation render"
>
> "frame_end (int in [-inf, inf], (optional)) – End Frame, Frame to end rendering animation at. If not specified, the scene end frame will be assumed. This should only be specified if doing an animation render"

— Python API, https://docs.blender.org/api/5.0/bpy.ops.render.html, read 2026-07-30. Note the operator itself carries no description ("Undocumented, consider contributing.") — everything above comes from its arguments.

So from Python there are three shapes: `animation=True` (writes files across the frame range), `write_still=True` with `animation=False` (writes one image to the output path), and neither (writes nothing; result stays in the Render Result). The frame range can now be given on the call itself — the release notes state this is new:

> "The `render.render()` operator now supports `frame_start` and `frame_end` optional arguments."

— release notes, https://developer.blender.org/docs/release_notes/5.0/python_api/, read 2026-07-30.

`bpy.ops.render.opengl` (quoted in full in 6e) is the Python route to a viewport render, with the same `animation` / `write_still` pair plus `sequencer` and `view_context`.

The API confirms buttons and Python are the same thing:

> "Most hotkeys and buttons in Blender call an operator which is also exposed to Python via bpy.ops."
>
> "To see the Python equivalent hover your mouse over the button and see the tooltip, e.g Python: bpy.ops.render.render(),"

— Python API, https://docs.blender.org/api/5.0/info_api_reference.html, read 2026-07-30.

---

### 6i. From a script run by a Blender started from outside

**Label: documented.**

> "blender --background --python myscript.py"
>
> "You might want to run this with a blend-file so the script has some data to operate on."
>
> "blender myscene.blend --background --python myscript.py"

— Python API, https://docs.blender.org/api/5.0/info_tips_and_tricks.html, read 2026-07-30.

The manual gives a rendering example of the same:

> "To render an animation in background mode, allowing drivers and other scripts to run:"
>
> "blender --background --enable-autoexec my_movie.blend --render-anim"

— manual, https://docs.blender.org/manual/en/5.0/advanced/scripting/security.html, read 2026-07-30.

What it writes is whatever the script's operator calls write (6h).

---

### 6j. From Blender built as a Python module

**Label: documented** (that this route exists and is equivalent to background mode) / **not in the sources** (what exactly it writes, beyond that).

> "Blender supports being built as a Python module, allowing import bpy to be added to any Python script, providing access to Blender's features."
>
> "Blender as a Python Module isn't provided on Blender's official download page."
>
> "Visualizing data by rendering images and animations."
>
> "For the most part using Blender as a Python module is equivalent to running a script in background-mode (passing the command-line arguments --background or -b), however there are some differences to be aware of."
>
> "Functionality controlled by command line arguments (shown by calling blender --help aren't accessible)."
>
> "Blender's typical signal handlers are not initialized, so there is no special handling for Control-C to cancel a render and a crash log is not written in the event of a crash."

— Python API, https://docs.blender.org/api/5.0/info_advanced_blender_as_bpy.html, read 2026-07-30.

The last quote is a difference in what gets written: no crash log on a crash.

---

### 6k. What any of these renders may write *in addition* to the main output

**Label: documented**, except the last item.

These are not separate ways to start a render; they are extra files a render writes whichever way it was started.

**File Output nodes in the compositor.**

> "The File Output node writes one or more images to disk during rendering or compositing."
>
> "Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence. For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."
>
> "Automatically save the final image after a render."

— manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html, read 2026-07-30.

Whether the compositor runs at all during a render is a switch:

> "Pipeline – Compositing"
>
> "Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node."

— manual, https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html, read 2026-07-30.

**Placeholder files.**

> "Image Sequence – Overwrite Image Multi-Layer EXR"
>
> "Create empty placeholder frames while rendering."

— manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html, read 2026-07-30. (The heading is repeated verbatim in the source for both Overwrite and Placeholders; the second one is Placeholders.)

> "use_placeholder"
>
> "Create empty placeholder files while rendering frames (similar to Unix 'touch')"
>
> "Type: boolean, default False"

— Python API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html, read 2026-07-30.

**A render cache file.**

> "Saving – Cache Result"
>
> "Saves the rendered view layers and their passes to a multi-layer OpenEXR image."
>
> "The image is stored in the Render Cache folder as specified in the File Paths Preferences."

— manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html, read 2026-07-30.

> "use_render_cache"
>
> "Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)"
>
> "Type: boolean, default False"

— Python API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html, read 2026-07-30.

> "Note that Blender doesn't create these cache files by default. You have to enable Cache Result in the scene's Output options and then render it at least once."

— manual, https://docs.blender.org/manual/en/5.0/editors/image/editing.html, read 2026-07-30.

I also opened the Output panel screenshot, `_images/render_output_properties_output_panel.png` (part of the manual page https://docs.blender.org/manual/en/5.0/render/output/properties/output.html, read 2026-07-30). **Label: documented.** Visible in it: a path field showing `/tmp/`, and checkboxes labelled Overwrite, Placeholders, File Extensions, Cache Result, with Overwrite and File Extensions ticked and Placeholders and Cache Result unticked. That matches the defaults quoted above.

**Cycles tile cache. Label: observed for the words, inferred for the effect.**

> ```
>     tile_size: IntProperty(
>         name="Tile Size",
>         default=2048,
>         description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory",
>         min=8,
>         max=8192,
>     )
> ```

— Cycles' own files, `Blender Sources/cycles_5.0/properties.py`, lines 1056–1062. Read 2026-07-30.

The file *says* tiles are cached to disk while rendering. That a Cycles render therefore puts temporary data on disk during the render is the step I took from those words — inferred, not observed. Where those tiles are written, and whether they survive the render, is not in the sources.

---

### 6l. Things I decided are not "a render being started", and why

- **Interactive viewport shading.** The manual says "Blender supports interactive 3D viewport rendering for all render engines, for quick iteration on lighting and shading. Once this is done, the final quality image or animation can be rendered and output." (manual, https://docs.blender.org/manual/en/5.0/render/introduction.html, read 2026-07-30). It is set against the final render on the same page, and no source describes it writing anything. **Not in the sources** that it writes.
- **Baking** (`bpy.ops.object.bake`, `bpy.ops.fluid.bake_*`, `bpy.ops.dpaint.bake`, and the animation "bake" operators). These exist in the API, and Cycles has its own `bake` entry point — observed, `Blender Sources/cycles_5.0/__init__.py` line 80: `def bake(self, depsgraph, obj, pass_type, pass_filter, width, height):`, and `Blender Sources/cycles_5.0/engine.py` line 115, same name. They do not go through the scene output path and the sources do not call them renders, so I left them out.
- **The animation player** (`-a` without `-b`, View Animation Ctrl-F11). Playback, not rendering; writes nothing.

---

### Not covered

- Whether Render Image (F12) writes nothing is **inferred**, not stated. The sources never say "F12 does not save a file".
- Whether "Render Viewport Preview" (the still viewport render menu entry) writes a file is not stated at all.
- Which operator and which arguments each menu entry maps to is not stated anywhere I looked. I matched them by name and by argument names only, and I have flagged where I did.
- What "Render Sequencer Image" and "Render Sequencer Animation" write is not stated; I inferred it from the ordinary two.
- Nothing in the sources describes an outcome — what appears on disk when a render is interrupted, cancelled, or fails partway. That is behaviour, and the documentation describes features.
- The sources do not say whether several of these can run at once, nor what happens when two renders target the same path.
- Where Cycles' disk tile cache is written, and whether it is cleaned up.
- I did not find a stated list anywhere of "the ways a render can be started". The list above is assembled from separate pages; there may be a starting point documented under a word I did not think to search. My search terms are recorded below so you can judge that.

### Outside the interface

*(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)*

Partly, and the split matters.

Documented as working with no interface: `-b` / `--background` with `-f` or `-a` ("Run in background (often used for UI-less rendering).", manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html); `--background --python`; and the bpy module ("equivalent to running a script in background-mode", https://docs.blender.org/api/5.0/info_advanced_blender_as_bpy.html). The command-line page adds:

> "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."

— manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html, read 2026-07-30.

Two differences the sources state outright for background use:

> "The audio device is disabled in background-mode by default and can be re-enabled by passing in -setaudio Default afterwards."

— manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30. (Bears on 6d, Render Audio.)

> "Blender's typical signal handlers are not initialized, so there is no special handling for Control-C to cancel a render and a crash log is not written in the event of a crash."

— Python API, https://docs.blender.org/api/5.0/info_advanced_blender_as_bpy.html, read 2026-07-30. (Bears on the bpy module route only.)

And the ordering trap in 6g is a difference that exists only outside the interface: where a command-line render writes depends on the order of the arguments, and on the fact that loading the blend-file overwrites an output path set before it.

The menu entries (6a–6d, 6f) and the viewport render entries (6e) are interface things by definition. Their Python operators can be called from a script, but whether `bpy.ops.render.opengl` works with no window is **not in the sources**. The nearest thing I found is a comment in an example render engine — "Lazily import GPU module, so that the render engine works in background mode where the GPU module can't be imported by default." (Python API, https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html, read 2026-07-30) — which is about importing a module, not about viewport rendering, and I will not stretch it. Note also that viewport render is documented as Workbench and EEVEE only, never Cycles.

The bpy module route loses the command-line switches entirely: "Functionality controlled by command line arguments (shown by calling blender --help aren't accessible)." — so `--threads`, `--log` and the Cycles `--cycles-device` switch are not available there.

### Operating system

*(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)*

The sources state no difference in *which* ways exist, or in *what* they write, between operating systems. What differs is documented only in how you reach the executable:

- macOS: "Open the terminal application, and run the executable within the app bundle, with commands like this: cd /Applications/Blender.app/Contents/MacOS ./Blender" — manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/macos.html, read 2026-07-30.
- Windows: "When Blender is started on a Microsoft Windows operating system, the Console Window (called the Command Prompt) is first created as a separate window on the desktop. The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console." — manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/windows.html, read 2026-07-30. This affects where you see the log output quoted in 6g.
- Linux: "If you have Blender installed in your PATH (usually when Blender is installed through a distribution package), you can simply run: blender" — manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/linux.html, read 2026-07-30. Plus the "no need for X server on Linux" note quoted above.

One format caveat, which is about output rather than about starting: "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP." — manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30. So which formats a render *can* write is not guaranteed identical on every build.

Cycles' device list differs by platform in effect — "Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL" — but the sources do not tie those names to operating systems, so I say nothing further.

---

### Record of where I looked (question 6)

**Folders searched in full, by text search across every `.txt`/`.md` in them:** `Blender Sources/manual_5.0`, `Blender Sources/api_5.0`, `Blender Sources/release_notes_5.0`, `Blender Sources/cycles_5.0`.

**Terms I searched for** (case-insensitive unless noted): `render animation`, `Render Menu`, `Render Image`, `F12`, `Viewport Render`, `playblast`, `Render Result`, `write_still`, `render anim`, `render_anim`, `render-frame`, `render_frame`, `sequencer scene`, `Sequencer Scene`, `placeholder`, `render farm`, `render queue`, `network render`, `batch render`, `start a render`, `starting a render`, `begin rendering`, `trigger a render`, `Quick Render`, `Render Region`, `render job`, `render pipeline`, `background mode`, `Python Module`, `bpy.ops.render.render`, `mixdown`, `bake`, `opengl`, `render preview`, `audio codec`, `encoding`, `cache`, `temp`, `disk`, `command`, `arguments`, `Encoding`, `Rendering`. In the API I also listed every `bpy.ops.*` operator signature across all `bpy.ops.*.txt` files and filtered for `render|opengl|mixdown|bake`, to be sure no render-starting operator lived outside `bpy.ops.render`.

**Pages and files I opened and read:**

Manual — `render/output/introduction.txt`, `render/output/animation.txt`, `render/output/properties/output.txt`, `render/output/properties/post_processing.txt`, `render/output/audio/introduction.txt`, `render/index.txt`, `render/introduction.txt`, `interface/window_system/topbar.txt`, `interface/keymap/blender_default.txt`, `advanced/command_line/render.txt`, `advanced/command_line/arguments.txt` (all of it), `advanced/command_line/launch/index.txt`, `.../linux.txt`, `.../macos.txt`, `.../windows.txt`, `advanced/scripting/security.txt`, `editors/3dview/viewport_render.txt`, `editors/3dview/index.txt`, `editors/image/introduction.txt`, `editors/image/editing.txt`, `editors/video_sequencer/preview/header.txt`, `editors/video_sequencer/sequencer/navigating.txt`, `compositing/types/output/file_output.txt`, `render/output/properties/stereoscopy/usage.txt` (skimmed for what a render writes per view).

Manual images — `_images/render_output_properties_output_panel.png` (read; contents described in 6k), `_images/interface_window-system_topbar_menus.png` (read; shows only the top-bar menu names File / Edit / Render / Window / Help, no menu contents — ruled out as unhelpful). I checked whether an image of the Render menu itself exists: the topbar page names only three images, and none is the open Render menu.

Python API — `bpy.ops.render.txt` (all of it), `bpy.ops.sound.txt`, `bpy.types.RenderSettings.txt`, `bpy.app.handlers.txt`, `info_advanced_blender_as_bpy.txt` (all of it), `info_tips_and_tricks.txt`, `info_api_reference.txt`, `bpy.types.RenderEngine.txt` (searched, not read whole).

Release notes — `rendering.md` (all of it), `pipeline_io.md` (all of it), `core.md` (logging section), `python_api.md` (Rendering, Logging sections), `sequencer.md`, `eevee.md`, `user_interface.md`, `bugfixes.md`, `corrective_releases.md` (all searched for render-starting terms).

Cycles — `engine.py`, `__init__.py`, `operators.py`, `properties.py` (searched for `render`, `bake`, `write`, `save`, `cache`, `temp`, `disk`; read the argument parser, the render/bake entry points, the denoise operator, and the tile-size property).

**What I ruled out and why:**

- `bpy.ops.sequencer.*` — searched every operator signature; the only file-writing one is `export_subtitles` (writes a `.srt`), which is not a render.
- Bake operators — reasoning given in 6l.
- `bpy.ops.image.save_as` — a manual save of an existing image, not a way to start a render.
- `cycles.denoise_animation` (Cycles' own `operators.py`) — it reads already-rendered frames and writes denoised ones; it does not start a render. Observed: its own description reads "Denoise rendered animation sequence using current scene and view layer settings. Requires denoising data passes and output to OpenEXR multilayer files" (`Blender Sources/cycles_5.0/operators.py`, lines 31–33). *(Citation check, 2026-08-05: line range corrected from 30–32; line 30 is the `class` line and the quoted description runs 31–33.)*
- The Freestyle, Grease Pencil, colour-management and view-layer pages — searched, nothing about starting a render or about writing.
- I found no page anywhere in the manual matching `start a render`, `starting a render`, `begin rendering`, `trigger a render`, or `Quick Render`. There is no single page that enumerates the ways.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 22 — what `-f`, `-a` and the Python render call each write.**

*What was done, to what.* One saved file holding both a scene output path and a
compositor File Output node was copied three times, and each copy rendered by one
route, each with its own `-o` folder so the routes could not see each other's work.

*The commands.*

```
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup …/r_f.blend  -o …/f_scene/s_ -F PNG -x 1 -f 1
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup …/r_a.blend  -o …/a_scene/s_ -F PNG -x 1 -a
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup …/r_py.blend -o …/py_scene/s_ -F PNG -x 1 --python q22.py
```

where `q22.py` calls `bpy.ops.render.render` three times: `animation=False,
write_still=False`, then `animation=False, write_still=True`, then `animation=True`.

*What came back.* From `-f 1`:

```
Executing compositor
Saved: '…/node_out/nd_Image0001.png'
Saved: '…/f_scene/s_0001.png'
exit code: 0
```

From `-a`:

```
Executing compositor
Saved: '…/node_out/nd_Image0001.png'
Saved: '…/a_scene/s_0001.png'
exit code: 0
```

From the three Python calls:

```
--- animation=False, write_still=False
    {'FINISHED'}
--- animation=False, write_still=True
    {'FINISHED'}
--- animation=True
    {'FINISHED'}
00:00.395  render           | Saved: '…/node_out/nd_Image.png'      (animation=False, write_still=False)
00:00.401  render           | Saved: '…/node_out/nd_Image.png'      (animation=False, write_still=True)
00:00.402  render           | Saved: '…/py_scene/s_.png'            (animation=False, write_still=True)
00:00.408  render           | Saved: '…/node_out/nd_Image0001.png'  (animation=True)
00:00.409  render           | Saved: '…/py_scene/s_0001.png'        (animation=True)
```

So the File Output node writes on all three, but its name carries the frame number
only on the animation call.

Two things in that are worth stating plainly. A single frame started with `-f` on
the command line writes the same names an animation does, frame number and all. And
a still saved by `write_still=True` gets **no** frame number — the file was
`s_.png`, not `s_0001.png`.

**Entry 21 — the viewport route with no window.** All three shapes of
`bpy.ops.render.opengl` were tried:

```
--- calling bpy.ops.render.opengl(**{'write_still': True, 'view_context': True})
Error: Cannot use OpenGL render in background mode (no opengl context)
   raised: RuntimeError Error: Cannot use OpenGL render in background mode (no opengl context)
--- calling bpy.ops.render.opengl(**{'write_still': True, 'view_context': False})
Error: Cannot use OpenGL render in background mode (no opengl context)
--- calling bpy.ops.render.opengl(**{'animation': True, 'view_context': False})
Error: Cannot use OpenGL render in background mode (no opengl context)
listing: no folder
```

Nothing was written, on any of the three.

**Entry 12 — an animation rendered to a video.** With the format set to an FFmpeg
Matroska video and frames 1 to 5, one file was written and it was called
`mov_0001-0005.mkv`. With `##` typed into the recorded name the hashes were left
alone and the file was called `mov_##.mkv`.

**Entry 23 — switching automatic saving off.** There is no such switch in 5.0 — see question 5 above for the exact words.

*Standing lines, brought up to date.* **Driven from outside** — the command-line and
Python routes are settled by the runs above. The viewport route is settled too, in
the other direction: it refuses to run with no window. What the menu entries do is
still open and sits with the Creator (entries 26 to 28 of the hand-test list).
**Operating system** — settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.0.1**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 26 — what the Render menu holds, and what each entry calls.**

*What was done, to what.* Blender was opened at its normal startup file, the Render
menu at the top of the window opened, and the pointer rested on each entry in turn
so its tooltip appeared. Six screenshots per version, kept at
`/Users/artist/Desktop/Screenshots/023/`, one folder per version.

*What came back.* The menu holds the same six entries in all four versions, in this
order:

```
Render Image        F12
Render Animation    ⌘ F12
------------------------
Render Audio...
------------------------
View Render         F11
View Animation      ⌘ F11
------------------------
[ ] Lock Interface
```

The tooltips, read from the screenshots:

```
Render Image      Render active scene.
                  Python: bpy.ops.render.render(use_viewport=True)

Render Animation  Render active scene.
                  Python: bpy.ops.render.render(animation=True, use_viewport=True)

Render Audio...   Mix the scene's audio to a sound file.
                  Python: bpy.ops.sound.mixdown()

View Render       Toggle show render view.
                  Python: bpy.ops.render.view_show()

View Animation    Play back rendered frames/movies using an external player.
                  Python: bpy.ops.render.play_rendered_anim()

Lock Interface    Lock interface during rendering in favor of giving more memory
                  to the renderer.
                  Python: RenderSettings.use_lock_interface
                          bpy.data.scenes["Scene"].render.use_lock_interface
```

Two things to be exact about. **Render Image passes no `write_still` at all**, and
neither does Render Animation — the only arguments either menu entry passes are the
ones shown. And the menu carried **no** Sequencer render entries in any of the four.
The entry did not ask for a sequencer scene to be set up, so this run says nothing
about whether such entries appear when one exists.

All six tooltips were read from the 4.5 screenshots. Four of the six were read from
the 5.0 set and two from each of the 5.1 and 5.2 sets; every one of those matched
the 4.5 wording. The rest of the screenshots are in the folder named above and were
not opened.

**Entry 27 — does pressing F12 write a file?**

*What was done, to what.* A prepared file per version, with its output path pointing
at an empty folder beside it and a one-frame range, was opened with the matching
Blender. F12 was pressed, the render window closed, and the folder looked at; then
Ctrl-F12 was pressed and the folder looked at again.

*What came back*, in the Creator's own words:

> "F12 saves nothing. Ctrl+F12 saves one file one file called img_0001.png.
> Consistent across all versions."

**Entry 28 — the 3D Viewport's own render entries.**

*What was done, to what.* The same prepared file, with its output folder emptied
first. The View menu in the 3D window was opened, the pointer rested on each
render entry, then the still one was clicked and the folder looked at, then the
animation one and the folder looked at again. Three screenshots per version, kept
at `/Users/artist/Desktop/Screenshots/019/`, one folder per version.

*What came back.* The three entries are named, in this version:

```
Render Viewport Preview
Render Playblast
Render Playblast on Keyframes
```

and their tooltips:

```
Render Viewport Preview        Take a snapshot of the active viewport.
                               Python: bpy.ops.render.opengl()

Render Playblast               Render the viewport for the animation range of this scene.
                               Python: bpy.ops.render.opengl(animation=True)

Render Playblast on Keyframes  Render the viewport for the animation range of this scene,
                               but only render keyframes of selected objects.
                               Python: bpy.ops.render.opengl(animation=True,
                                                             render_keyed_only=True)
```

All three tooltips were read from the screenshots for this version.

and, in the Creator's own words, for what they wrote:

> "Same as test 18 — still writes nothing, an animation (or playblast or whatever
> it's called in some versions) writes one file called img_0001.png"

"Test 18" is the Creator's own number for entry 27 above. So the still entry wrote
nothing and the animation entry wrote one file. The entry also asked what the
keyframes-only variant does; that was not reported, and is not filled in here.

Note that the menu entries are named differently in 4.5 from the other three, and
that neither of the two operator calls passes `write_still`.

**Entry 29 — the switch in the Output panel's header.**

*What was done, to what.* Blender opened at its normal startup file, Properties ‣
Output tab, the panel headed Output, and the pointer rested on the header strip of
that panel.

*What came back*, in the Creator's own words:

> "This checkbox only exists in 5.2. Its tooltip says "Save Output / Write frames to
> disk for animation renders.""

So there is no such checkbox in this version.

The entry also asked for the labels of every checkbox inside the panel, and for the
`Python:` line at the end of the tooltip. Neither was reported, and neither is
filled in here.

*Standing lines, brought up to date.* **Driven from outside** — nothing here bears on it: every one of these was done at Blender's own window, which is the half the terminal cannot reach. **Operating system** — the Creator works on a Mac, so this settles macOS and leaves Windows and Linux open.


### Observed — hand tests run on a rented Linux machine, 2026-08-06

Everything in the Linux sections of this document was run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. It is a rented container sharing somebody else's hardware, so it is given fewer cores than the chip really has and its system settings are not entirely its own; other work may have been running on it at the same time, so nothing here says how long anything took. Blender was started with no window unless an entry says otherwise, with its settings folder pointed at one of this topic's own and with the display variable unset so no window could be reached. The build used for this version was **Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)**. Date: 2026-08-06. In the output below, `…` stands for the working folder on that machine, which was `/root/smoozy/Writing/work/<entry>/5.0`. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests — Linux.md`.

**One note on how the output is shown.** From 5.0 every line Blender prints while rendering carries a timestamp and a channel name in front of it, like `00:00.617  render           | Saved: '…'`. Where a quoted line below carries no such prefix, the prefix has been dropped because the timestamp differs from one run to the next; nothing after the bar is changed.

**These results settle Linux and say nothing about macOS.** Where the same test was also run on the Creator's Mac, both results are given and neither is folded into the other.

**Entry 22 — what the command line and the Python call each write.** A blend-file
holding both a scene output and a File Output node was rendered three ways. A single
frame from the command line wrote one scene image and one node image; an animation
wrote the same; and the three shapes of the Python call gave:

```
--- animation=False, write_still=False   → Saved: '…/node_out/nd_Image.png'
--- animation=False, write_still=True    → Saved: '…/node_out/nd_Image.png'
                                           Saved: '…/py_scene/s_.png'
--- animation=True                       → Saved: '…/node_out/nd_Image0001.png'
                                           Saved: '…/py_scene/s_0001.png'
```

**Entry 21 — the viewport route with no window.** Refused, in all three shapes, with
the same words as on the Mac and nothing written:

```
Error: Cannot use OpenGL render in background mode (no opengl context)
```

**Entry 27 — the viewport route with a real window.** This is the half the Mac could
never reach from a terminal. The rented machine has a display of its own, so Blender
was started **with a window**, driven by a timer, and asked for the same three calls.
All three succeeded:

```
window count: 1
engine: BLENDER_EEVEE
--- bpy.ops.render.opengl(write_still=True, view_context=False)
   returned: {'FINISHED'}
--- bpy.ops.render.opengl(animation=True, view_context=False)
   returned: {'FINISHED'}
--- bpy.ops.render.opengl(write_still=False, view_context=False)
   returned: {'FINISHED'}
listing: ['vp_.png', 'vp_0001.png']
```

and on the terminal:

```
Saved: '…/vp/vp_.png'
OpenGL Render written to '…/vp/vp_.png'
Saved: '…/vp/vp_0001.png'
```

So with a window the viewport route writes: told to save a still it writes one file
with **no frame number**, an animation writes one file **with** the frame number, and
told not to save it writes nothing. The refusal recorded above is about having no
window, not about the route itself.

**Entry 12 — a video.** One file holding the whole frame range in its name,
`mov_0001-0005.mkv`, the same as on the Mac.

*The two machines.* Entries 22, 21 and 12 were run on both and **agreed**. Entry 27
was run on Linux only; nothing like it was run on the Mac.

*Standing lines, brought up to date.* **Driven from outside** — the command-line and
Python routes were all driven from outside with no window; the viewport route is the
one that needs a window, and it works when there is one. **Operating system** — now
settled on two systems, macOS and Linux, which agreed on everything run on both.
Windows is untried.


## Still open for a running Blender

The list this section began as has been worked through, both halves of it. What was
run, when, on what and with what result is written into each question above as an
observed finding, and the whole list with its exact commands is
`Audits/Blender Research/Writing/Hand tests.md`.

One thing is left that a running Blender could settle, and nobody has run it:

- **Whether an animation rendered to a video carries the scene's sound.** It was
  never on the list.

Three smaller gaps were left by answers that did not cover everything their entry
asked for, and are written in where they belong above rather than filled in:
what the keyframes-only viewport render writes; the labels of the checkboxes inside
the Output panel and the `Python:` line of its header switch; and whether any
message appeared when over-long text was entered in the File Path field.

A second list was run on 2026-08-06, when a second machine arrived: a rented Linux
machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve
cores, reached over SSH. That list is
`Audits/Blender Research/Writing/Hand tests — Linux.md`, and its results are written
into each question above **beside** the Mac's rather than over them. It settled on
Linux what the Mac could only settle on macOS: which characters Linux refuses in a
name, whether a missing output folder is made, what a render does when the folder
cannot be written to, what the folder separator does, and whether a shorter limit on
length bites.

One thing on that list was not run, because it needs the machine to itself and the
machine is shared with other work:

- **What a render does on Linux when the volume runs out of room.** It is written out
  in full, with its exact commands, under *Needs the machine to itself* in
  `Audits/Blender Research/Writing/Hand tests — Linux.md`.

And these were not run:

- Which characters an output path may hold on Windows, and what a render does when it
  meets one it may not.
- Whether a missing output folder is made on Windows.
- What a render does on Windows when the folder cannot be written to, or the disk is
  full.
- Whether what is recorded differs on Windows, where the path separator differs.
- Whether an over-long name behaves differently on Windows, where the whole-path
  limit is shorter.
- Anything needing two graphics cards, a card that is neither Apple's nor NVIDIA's, a
  second machine of the same kind, or hardware below Blender's stated floor.

The equipment is one MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, and the
rented Linux machine described above. No Windows machine was used. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

2026-08-04 — Opus 5 (Claude Code) — Writing, Blender 5.0
