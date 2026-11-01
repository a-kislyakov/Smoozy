# Images — Blender 5.0 — second pass (question 7 only)

**Sources consulted, and when.** Blender's manual (`Blender Sources/manual_5.0`, including its `_images` folder), Blender's Python API documentation (`Blender Sources/api_5.0`), Blender's release notes (`Blender Sources/release_notes_5.0`), Cycles' own files (`Blender Sources/cycles_5.0`), and the requirements page (`Blender Sources/requirements/requirements_2026-08-01.txt`). All of these were taken on **2026-07-30** (the requirements page on 2026-08-01), and that is the date read for every finding below. I searched them on 2026-08-03.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

One disclosure, so that line can be trusted. I did not open `CLAUDE.md`, but the tool I run in placed its text into my context automatically before I did anything, so I saw it. Its whole content is a signpost telling Blender researchers to stop reading and pointing everyone else at `Preparation/Groundwork/Start Here.md`. I did not follow that pointer, and I opened nothing under `Preparation/` except the question sheet named in my brief. That file makes no claim about how Blender behaves, so nothing in it could have coloured a finding below. I opened no other project file, ran no helper agents, and used no web access.

This pass answers question 7 only.

---

## 7. Can Blender turn a set of finished frames into a short moving picture?

**LABEL: documented.**

Yes. The manual describes this as a normal, expected step, and describes it twice in two different places, plus a third route for one awkward case.

### The plain statement

> "Images can then be encoded to a video by adding the rendered image sequence into the Video Sequencer and choosing an appropriate Video Output."

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/introduction.html — Date read: 2026-07-30.

And, on the page about rendering animations:

> "Finally you can render out from the Sequencer and compress the frames into a playable movie clip."

> "You can then make a movie out of the separate frames with Blender's Sequence editor or use 3rd party encoding software."

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/animation.html — Date read: 2026-07-30.

The same idea appears in the page that lists video formats:

> "These formats are primarily used for compressing rendered sequences into a playable movie."

Source: manual, https://docs.blender.org/manual/en/5.0/files/media/video_formats.html — Date read: 2026-07-30.

### The steps the manual gives

The manual sets out the whole job step by step under the heading "Frame Sequence Workflow". The relevant steps, quoted exactly:

> "In Blender, open the Video Sequencer."

> "Choose Add Image from the add menu. Select all the frames from your output folder that you want to include in your animation. They will be added as a strip in the Sequence editor."

> "In the Output panel, choose the container and codec you want (e.g. MPEG H.264) and configure them."

> "Click the Animation render button and Blender will render out the Sequence editor output into a movie."

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/animation.html — Date read: 2026-07-30.

### That the strip really is a set of separate frames

The page describing the strip type says so:

> "In the case of (numbered) image sequences (e.g. \*-0001.jpg, \*-0002.jpg, \*-0003.jpg, etc, of any image format), you have a choice:"

and, describing the "Import As" choice on the add operation:

> "Image Sequence:
> All images are imported as a single image sequence, no matter their extension or filename. Note that since these files do not have to be numbered, placeholders are not supported for this mode."

and, on missing frames:

> "This allows you to render an image sequence with a few frames missing, while ensuring that the image sequence strip maintains the correct length. Placeholder frames are rendered in pink in the VSE preview."

Source: manual, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/image.html — Date read: 2026-07-30.

### The switch that makes the render come from the Sequencer rather than the camera

> "Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline."

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html — Date read: 2026-07-30.

I also opened the picture on that page, `_images/render_output_properties_post-processing_panel.png`. **LABEL: documented (from a picture).** It shows a panel headed "Post Processing" with two ticked checkboxes side by side, "Compositing" and "Sequencer", and a "Dither" slider reading 1.000 below them. That settles that the Sequencer pipeline is an ordinary on/off box in the Output properties, and that it is ticked in the state the manual chose to show.

I opened one more picture, `_images/render_output_properties_output_video-encoding-panel.png`, referenced from the Encoding panel section of the Output properties page. **LABEL: documented (from a picture).** It shows an "Output" panel with a path of `/tmp/`, a "File Format" menu reading "FFmpeg video", and beneath it an "Encoding" section with "Container" set to "Matroska", a "Video" section with "Video Codec" set to "H.264", "Output quality" set to "Medium quality", "Encoding speed" set to "Good", "Keyframe interval" 18, "Max B-frames" unticked and 0, and an "Audio" section with "Audio Codec" set to "Vorbis", "Bitrate" 192, "Volume" 1.000. That settles that a movie container and codec are chosen in the same Output panel that a still-image format would be chosen in.

Source of both pictures: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html and https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — Date read: 2026-07-30.

### The one documented exception, and its documented workaround

> "Note
>
> The Video Sequencer does not support multi-layer EXR files. To render to a video format you will have to skip the next three steps and instead use an Image Input node in the Compositor."

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/animation.html — Date read: 2026-07-30.

So if the finished frames are multi-layer EXR files, the manual sends you through the Compositor instead of the Sequencer. That the Compositor's Image node can take a whole sequence rather than one picture follows from the API's description of where an image comes from:

> "SEQUENCE
> Image Sequence – Multiple image files, as a sequence."

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.Image.html — Date read: 2026-07-30.

### The same thing stated in the menus

> "Render Sequencer Animation Ctrl-Alt-F12
>
> Render the animation of the Sequencer Scene."

Source: manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html — Date read: 2026-07-30.

And in the description of the built-in video editing template:

> "Use Render ‣ Render Animation to export your final video."

Source: manual, https://docs.blender.org/manual/en/5.0/video_editing/setup/app_template.html — Date read: 2026-07-30.

### What the Python API side offers

Every step above has a named handle in the API, which matters for the "outside the interface" line further down.

Adding the frames as one strip:

> "bpy.ops.sequencer.image_strip_add(...)
>
> Add an image or image sequence to the sequencer"

and, among its choices:

> "Image Sequence – Import all selected images as a single image sequence. The sequence of images does not have to match Blender's numbered sequence pattern, so placeholders cannot be inferred."

Source: Python API, https://docs.blender.org/api/5.0/bpy.ops.sequencer.html — Date read: 2026-07-30.

Building the same strip without any file browser:

> "new_image(name, filepath, channel, frame_start, \*, fit_method='ORIGINAL')
>
> Add a new image strip"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.StripsTopLevel.html — Date read: 2026-07-30.

Adding further frames to that strip one at a time:

> "append(filename)
>
> Push an image from ImageStrip.directory"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.StripElements.html — Date read: 2026-07-30.

That the strip holds many pictures, not one:

> "class bpy.types.ImageStrip(Strip)
>
> Sequence strip to load one or more images"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.ImageStrip.html — Date read: 2026-07-30.

Sending the render through the Sequencer:

> "use_sequencer
>
> Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist
>
> Type:
>
> boolean, default True"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — Date read: 2026-07-30.

Choosing a movie as the thing written out:

> "media_type
>
> The type of media to save
>
> Type:
>
> enum in ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO'], default 'IMAGE'"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.ImageFormatSettings.html — Date read: 2026-07-30.

The corresponding file format entry:

> "FFMPEG:
>
> FFmpeg Video."

Source: Python API, https://docs.blender.org/api/5.0/bpy_types_enum_items/image_type_all_items.html — Date read: 2026-07-30.

And the containers a movie can be written into:

> "format
>
> Output file container
>
> Type:
>
> enum in ['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME'], default 'MKV'"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.FFmpegSettings.html — Date read: 2026-07-30.

Starting the job:

> "bpy.ops.render.render(\*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"

> "use_sequencer_scene (boolean, (optional)) – Use Sequencer Scene, Render the sequencer scene instead of the active scene"

Source: Python API, https://docs.blender.org/api/5.0/bpy.ops.render.html — Date read: 2026-07-30.

### What Cycles' own files show

**LABEL: observed.** Cycles' add-on files draw the Sequencer pipeline switch into their own Output properties panel, so this switch is present when Cycles is the engine, not only under some other engine.

Exact lines, from `Blender Sources/cycles_5.0/ui.py`, in the class `CYCLES_PT_post_processing` (its `draw` method; the `use_sequencer` line is line 1126 of that file):

```
class CYCLES_PT_post_processing(CyclesButtonsPanel, Panel):
    bl_label = "Post Processing"
    bl_options = {'DEFAULT_CLOSED'}
    bl_context = "output"

    def draw(self, context):
        layout = self.layout
        layout.use_property_split = True
        layout.use_property_decorate = False

        rd = context.scene.render

        col = layout.column(align=True, heading="Pipeline")
        col.prop(rd, "use_compositing")
        col.prop(rd, "use_sequencer")

        layout.prop(rd, "dither_intensity", text="Dither", slider=True)
```

What this file says is that the panel exists, sits in the Output tab, is closed by default, and offers those three controls. What a render actually does when `use_sequencer` is ticked is a step beyond the file — for that, the claim rests on the manual quote above, not on this file.

Source: `Blender Sources/cycles_5.0/ui.py`, class `CYCLES_PT_post_processing` — Date read: 2026-07-30.

### What the release notes say about this in 5.0

The release notes do not add or remove the ability. They describe changes around it.

> "If the sequencer scene differs from the active scene, the Render menu will show new options for \"Render Sequencer Image\" / \"Render Sequencer Animation\" on top of the normal scene render operators, allowing the user to specify which scene they'd like to render."

> "Images can now be added in bulk from drag and drop or the file browser. (blender/blender!143974)"

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/sequencer/ — Date read: 2026-07-30.

> "The `end_frame` property on newly added image and effect strips (which indirectly controlled their duration relative to the `start_frame`) has been replaced with `length`, to support multiple image strips being added at a time. (blender/blender!143974)"

> "`ImageFormatSettings` now has a `media_type` member that needs to be set to an appropriate type before setting the actual `file_format` member. (blender/blender@92d5c2078e)"

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/python_api/ — Date read: 2026-07-30.

That second one matters to anyone driving this from a script: as of 5.0 the kind of media is set first, and only then the format.

> "HDR and wide gamut videos can now be read and written. (blender/blender!120033, blender/blender!145373)"

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/pipeline_io/ — Date read: 2026-07-30.

> "Some command line flags were replaced.
>
> |`--debug-ffmpeg`|`--log video`|"

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/core/ — Date read: 2026-07-30.

### Ambiguity in the question, flagged

I read "short moving picture" as a video file — a movie. That is the reading everything above answers.

The other reading is an animated picture file rather than a video file, for example an animated GIF. I searched for that and found nothing: the complete list of formats Blender can write, quoted from the API's own enumeration, is JPEG, OPEN_EXR, PNG, WEBP, BMP, CINEON, DPX, IRIS, JPEG2000, HDR, TARGA, TARGA_RAW, TIFF, OPEN_EXR_MULTILAYER and FFMPEG (https://docs.blender.org/api/5.0/bpy_types_enum_items/image_type_all_items.html, read 2026-07-30). No animated-image format appears there, and searching the manual's output pages and that API page for "gif" returned nothing at all. On that second reading the answer would be **not in the sources**, tending to no.

Nothing in the sources treats "short" as a distinct case. The length of the result is simply the frame range.

### Observed — hand test, 2026-08-03

Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.0.1, started from a terminal with no window throughout.

**This settles the disagreement between this document and `Preparation/Blender Research/Findings/5.0/Images.md`, which does not establish the outside-the-window half. The observed result below is what now stands, and the same result is written into that document too.**

*The whole job with no window at all.* Set up, in a windowless Blender: six PNG frames at 64 by 48 rendered from the ordinary scene into a folder; then the 3D scene changed so a fresh render could not be mistaken for those frames — the cube deleted and the world background set to pure blue; then an image strip built from the six saved files with `scene.sequence_editor_create()` and `strips.new_image(name="Frames", filepath=<first file>, channel=1, frame_start=1)` followed by `strip.elements.append(<each remaining file name>)`. Blender printed `STRIP: Frames type: IMAGE elements: 6` and `use_sequencer: True`. Four blend-files were saved from that state: video output with the Sequencer pipeline on, video output with it off, PNG output with it on, PNG output with it off.

Done, in a terminal:

```
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b -noaudio --factory-startup /tmp/images-hand-tests/out/t16/5.0.1/sequence-test.blend -a
```

Blender printed, for each frame:

```
00:00.371  render           | Rendering animation (frames 1..6)
00:00.381  render           | Rendering frame 1
00:00.381  render           | Executing sequencer
00:00.434  render           | Video append frame 1
00:00.434  render           | Time: 00:00.05 (Saving: 00:00.03)
```

The exit status was `0`, and one file appeared at the output path: `out_0001-0006.mp4`, which `file` reported as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`. The same command on the file with the Sequencer pipeline switched off wrote `nosec_0001-0006.mp4`.

To see which pictures went into it, the same two files were rendered again with PNG output instead of video, and the written frames' pixels read:

```
frames       f_0001.png   size=(64, 48) centre=(0.5451, 0.5451, 0.5451, 1.0) corner=(0.2275, 0.2275, 0.2275, 1.0)
png_seq_on   s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0)
png_seq_off  s_0001.png   size=(64, 48) centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

With the Sequencer pipeline on, what came out is the saved frames — grey cube on dark grey. With it off, what came out is the changed 3D scene — blue, no cube. The six Sequencer frames took 0.3 seconds in total; the six 3D renders took about that each.

*A folder of frames with nothing prepared.* `blender -b --factory-startup -F FFMPEG -o <folder>/n_ -a` against the plain startup file wrote `n_0001-0003.mkv` — a movie of the 3D scene, not of any folder of pictures. There is no command-line argument that turns a folder of frames into a movie by itself.

*A frame missing from the set at the moment of encoding.* Set up: five frames written, a strip built naming all five, the file saved, and then the third frame's file deleted from disk. Done: the same background animation render, to PNG. All five output frames were written, nothing was printed about the missing one, and their pixels read:

```
s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3181
s_0002.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3181
s_0003.png   size=(64, 48) centre=(0.0, 0.0, 0.0, 0.0) corner=(0.0, 0.0, 0.0, 0.0) bytes=1170
s_0004.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3181
s_0005.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3181
```

The frame whose file was gone came out fully transparent black.

*Multi-layer OpenEXR frames offered to the Video Sequencer.* Set up: three multi-layer OpenEXR frames at 64 by 48 with the depth and normal passes on; a strip built from them; PNG output. Done: the same background animation render. Three files were written, with no message about the format:

```
s_0001.png   size=(64, 48) centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=2929
s_0002.png   size=(64, 48) centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=2929
s_0003.png   size=(64, 48) centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=2929
```

Compared with the same scene's PNG frames put through the same route, which gave `centre=(0.5373, ...)` and `corner=(0.2, ...)`, the picture is there but darker.

*How long the result is.* Twenty-four frames, a strip built from all of them, MPEG-4 container and H.264 codec, encoded once at `render.fps = 24` and once at `render.fps = 12`. Each movie's own header records:

```
at24_0001-0024.mp4   bytes=2314   timescale=1000 duration=1000 -> 1.000 s
at12_0001-0024.mp4   bytes=2592   timescale=1000 duration=2000 -> 2.000 s
```

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Blender 5.0.1 open. Seen: the Output panel's top control is labelled `Media Type` and offers `Image`, `Multi-Layer EXR` and `Video`; below it sits a second control labelled `File Format`. With `Video` chosen, the `Encoding` section's `Container` menu offers `MPEG-4`, `Matroska`, `WebM`, then a dividing line, then `AVI`, `DV`, `Flash`, `MPEG-1`, `MPEG-2`, `Ogg`, `QuickTime`. The `Video Codec` menu offers `No Video`, `AV1`, `H.264`, `H.265 / HEVC`, `WebM / VP9`, then a dividing line, then `DNxHD`, `DV`, `FFmpeg video codec #1`, `Flash Video`, `HuffYUV`, `MPEG-1`, `MPEG-2`, `MPEG-4 (divx)`, `PNG`, `ProRes`, `QuickTime Animation`, `Theora`. That settles the "not available on all systems" caveat for this machine: video output is really there, and the container and codec used above are both on those menus. With `Media Type` on `Image`, the `File Format` menu holds thirteen still-image entries and nothing animated of the single-picture kind.

The whole list of tests, with the exact commands, is at `Audits/Blender Research/Images/Hand tests.md`.

### Observed — the same tests on a second machine, 2026-08-06

Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, with no window throughout, driven by the same scripts the Mac run used, with only the folder path changed. The whole list is at `Audits/Blender Research/Images/Hand tests — Linux.md`.

*The whole job with no window at all.* `blender -b -noaudio --factory-startup .../sequence-test.blend -a` printed `Executing sequencer` and `Video append frame 1` through `Video append frame 6` on timestamped lines, the exit status was `0`, and `out_0001-0006.mp4` appeared, reported by `file` as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`. Rendered again to PNG, the frames read:

```
frames       f_0001.png   size=(64, 48) centre=(0.5451, 0.5451, 0.5451, 1.0) corner=(0.2275, 0.2275, 0.2275, 1.0)
png_seq_on   s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0)
png_seq_off  s_0001.png   size=(64, 48) centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

*A folder of frames with nothing prepared.* The same command wrote `n_0001-0003.mkv`, reported as `Matroska data` — a movie of the 3D scene.

*A frame missing from the set.* All five output frames were written, nothing was printed about the missing one, and the third came out fully transparent black: `s_0003.png size=(64, 48) centre=(0.0, 0.0, 0.0, 0.0) corner=(0.0, 0.0, 0.0, 0.0) bytes=1164`.

*Multi-layer OpenEXR frames offered to the Video Sequencer.* Three files written with no message about the format, `centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0)`.

*How long the result is.* The two movies' own headers record:

```
at24_0001-0024.mp4 bytes=2314   timescale=1000 duration=1000 -> 1.000 s
at12_0001-0024.mp4 bytes=2597   timescale=1000 duration=2000 -> 2.000 s
```

Both machines were run and both agreed on every one of these, with only the byte sizes of the written files differing by a few bytes.

*Which container and codec will actually take it.* A new test, run on both machines and recorded in full under question 4 in `Preparation/Blender Research/Findings/5.0/Images.md`, encoded three frames for every container with H.264 and for the MPEG-4 container with every codec. Ten of those pairings are refused with `Error: Could not initialize streams, probably unsupported codec combination`, and three more with a complaint about the picture size. So the container and codec menus offer more pairings than this version will write.

### Not covered

- The sources do not put any limit, upper or lower, on how long the result may be. Nothing here settles anything about "short" as opposed to long.
- The sources do not say what happens if the set of frames is inconsistent — different sizes are addressed (the strip page describes auto-scaling and Fit Method), but different bit depths, different colour spaces or a mix of formats within one sequence are not spelled out beyond the "Batch" note that a batch "can be a mix of file types (jpg, png, exr, etc.)".
- The sources do not state what happens if a frame file is corrupt or half-written. Missing frames are covered by the placeholder feature; damaged ones are not mentioned.
- The sources do not say how the frames' colour is interpreted on the way in beyond the "Set View Transform" option on the add operation, which the manual describes as automatic based on the colour space of the imported media.
- Nothing here settles how long the encoding takes, or whether it can be resumed if interrupted. The manual's resume advice ("Just disable the Overwrite option to start rendering where you left off") is about rendering frames, not about encoding them into a movie.
- I did not exercise Blender. Everything above is read, not run.

### Outside the interface

**Observed — yes, the whole job.** The hand test above did both halves with no window open: the image strip was built from the finished frames by a script in a windowless Blender, and the movie was encoded by `blender -b <file> -a` from a terminal, with Blender printing `Executing sequencer` for each frame. The written frames prove the pictures came from the strip and not from a fresh 3D render. So the step named below as inferred — that the Sequencer pipeline is honoured in background mode — is now observed to hold on that machine. (Previously, before the test: inferred, with a documented base and a named gap.)

The documented base. Blender can be run without its window, and can be told to render the whole animation using the settings already saved in the file:

> "-b, --background
>
> Run in background (often used for UI-less rendering)."

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

> "-F, --render-format \<format\>
>
> Set the render format."

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — Date read: 2026-07-30.

> "blender -b file.blend -a
>
> -a
>
> Render the whole animation using all the settings saved in the blend-file."

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — Date read: 2026-07-30.

Every step of the job also has a named Python handle that does not need a window: building the image strip (`StripsTopLevel.new_image`, `StripElements.append`), turning on the Sequencer pipeline (`RenderSettings.use_sequencer`, which is already on by default), choosing a movie as the output (`ImageFormatSettings.media_type` set to `'VIDEO'`, then `file_format` to `'FFMPEG'`, then `FFmpegSettings.format` for the container), and starting the render (`bpy.ops.render.render` with `animation=True`). All are quoted above.

The inference, written out. Putting those two together — a saved file whose Sequencer holds the image strip and whose output is set to a movie, plus `blender -b file.blend -a`, which renders "using all the settings saved in the blend-file", with `use_sequencer` on by default — the job can be set up and run with nobody at the window. The step a reader has to supply is that the Sequencer pipeline is honoured in background mode. **No source I found states that outright.** I searched the manual's command-line pages and scripting pages for "sequencer" and found no mention at all, and searched the whole manual for "background mode", "background-mode" and "headless", which turned up only the audio note, the dependency-graph notes, an add-on tutorial aside, and the EEVEE headless note. So treat "it works headless" as inferred, not documented.

Three documented differences between inside and outside the window:

1. **The same letter means two different things.** `-a` is the render-animation flag, but on its own, without `-b`, it makes Blender a player instead: "Instead of showing Blender's user interface, this runs Blender as an animation player, to view movies and image sequences rendered in Blender (ignored if -b is set)." (Command Line Arguments page, read 2026-07-30.) Anyone driving Blender from outside needs `-b` present for `-a` to mean rendering.

2. **Order of arguments matters.** "Arguments are executed in the order they are given!" and "Always position -f or -a as the last arguments." (Rendering From The Command Line, read 2026-07-30.) So the output path and format must be set before `-a`, or they are ignored.

3. **Sound is off by default outside the window.** "The audio device is disabled in background-mode by default and can be re-enabled by passing in -setaudio Default afterwards." (Command Line Arguments page, read 2026-07-30.) This does not affect turning frames into a picture-only movie, but it does affect any audio track added alongside.

One route that looks like it does need the window. The Video Sequencer's own render commands are described as preview-based:

> "Sequence Render Image and Sequence Render Animation don't render the final video by default – specifically, they don't render Scene Strips, instead using the preview's shading mode (which is initially Solid)."

Source: manual, https://docs.blender.org/manual/en/5.0/editors/video_sequencer/sequencer/navigating.html — Date read: 2026-07-30.

The matching API entry is `bpy.ops.render.opengl`, described as "Take a snapshot of the active viewport", with an option "sequencer (boolean, (optional)) – Sequencer, Render using the sequencer's OpenGL display" (https://docs.blender.org/api/5.0/bpy.ops.render.html, read 2026-07-30). **LABEL: inferred** — a command described as snapshotting the active viewport reads as needing a viewport, so this is the route to avoid when driving Blender from outside. The sources do not say so in as many words. For frames-into-a-movie, the ordinary Render Animation route quoted earlier is the one the manual points at.

### Operating system

**Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The whole job — building the strip with no window and encoding it from a terminal — works on both, and the frames, the file types and the recorded durations match. That closes the gap this section used to name: a render happens on a rented Linux machine, and the job has now been done there. Both builds have FFmpeg compiled in, and the MPEG-4 container with the H.264 codec wrote a playable file on both. **Windows is still untried** — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

What the sources say, which still stands. The one documented catch is that the movie format itself is not guaranteed to be present in every build:

> "-F, --render-format \<format\>
>
> Set the render format. Valid options are: TGA RAWTGA JPEG IRIS PNG BMP HDR TIFF.
>
> Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — Date read: 2026-07-30.

FFMPEG is the movie format, and it is in the second list. So the sources say plainly that the ability to write a movie at all depends on how that copy of Blender was built, and is not promised on every system.

The manual also warns about the receiving end rather than the writing end:

> "There are dozens, if not hundreds, of codecs, including Xvid, H.264, DivX, Microsoft, and so on. Each has advantages and disadvantages, and compatibility with different players on different operating systems."

Source: manual, https://docs.blender.org/manual/en/5.0/files/media/video_formats.html — Date read: 2026-07-30.

Beyond those two points, the sources do not draw any operating-system distinction for turning frames into a movie. I found exactly one operating-system-specific rendering limitation anywhere in the 5.0 manual, and it is about a render engine, not about encoding:

> "Headless Rendering
>
> Headless rendering is not supported on headless Windows systems."

Source: manual, https://docs.blender.org/manual/en/5.0/render/eevee/limitations/limitations.html — Date read: 2026-07-30.

**LABEL: inferred** — this would only touch the job in hand if the Sequencer timeline contains Scene strips that EEVEE has to render, since a timeline holding only image strips renders no 3D scene. The step a reader must supply is that a Sequencer render of image strips alone does not invoke a render engine; the sources do not say that outright, so do not treat it as settled.

The requirements page names 5.0 in one place — "Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." (`Blender Sources/requirements/requirements_2026-08-01.txt`, read 2026-08-01). That is about which machines run 5.0 at all, not about this feature.

---

## Where I looked, for question 7

This is the full record, including the searches that found nothing.

**Searched across the whole manual, the whole API documentation, the whole release notes and all Cycles files at once**, for these terms (case-insensitive): `image sequence`, `stitch`, `assemble`, `concatenate`, `flipbook`, `timelapse`, `time lapse`, `montage`, `frames into`, `into a movie`, `playable movie`, `compress the frames`, `make a movie`, `render out to a movie`, `movie out of`, `encode`, `ffmpeg`, `movie`, `video`, `sequencer`, `background mode`, `background-mode`, `headless`, `render animation`, `image strip`, `prores`, `h.264`, `container`, `av1`, `webm`, `matroska`, `gif`, `new_image`, `image_strip_add`, `use_sequencer`, `media_type`.

**Manual pages opened in full or in part:**

- `render/output/animation.txt` → https://docs.blender.org/manual/en/5.0/render/output/animation.html — read in full. This is the page that carries the workflow.
- `render/output/introduction.txt` → https://docs.blender.org/manual/en/5.0/render/output/introduction.html — read in full.
- `render/output/properties/post_processing.txt` → https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html — read in full.
- `render/output/properties/output.txt` → https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read the Media Type, Encoding, Container, Video, Colour Depth and ProRes sections.
- `render/output/animation_player.txt` → https://docs.blender.org/manual/en/5.0/render/output/animation_player.html — read in full, and **ruled out**: it plays frames back on screen, it does not write a file. "The animation player is a utility typically used for previewing rendered animations".
- `video_editing/edit/montage/strips/image.txt` → https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/image.html — read in full.
- `video_editing/introduction.txt` → https://docs.blender.org/manual/en/5.0/video_editing/introduction.html — read in full.
- `video_editing/setup/app_template.txt` → https://docs.blender.org/manual/en/5.0/video_editing/setup/app_template.html — read the relevant part.
- `files/media/video_formats.txt` → https://docs.blender.org/manual/en/5.0/files/media/video_formats.html — read the opening and the container list.
- `interface/window_system/topbar.txt` → https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html — read the Render menu section.
- `editors/video_sequencer/sequencer/navigating.txt` → https://docs.blender.org/manual/en/5.0/editors/video_sequencer/sequencer/navigating.html — read the Range and render sections.
- `advanced/command_line/arguments.txt` → https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read the render, format, playback and logging sections.
- `advanced/command_line/render.txt` → https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — read in full.
- `compositing/types/input/image.txt` → https://docs.blender.org/manual/en/5.0/compositing/types/input/image.html — read in full, to check the Compositor route the manual sends multi-layer EXR down. It confirms the node takes "any image format that is supported by Blender" but does not itself mention sequences; that came from the API instead.
- `editors/image/image_settings.txt` and `editors/image/editing.txt` → https://docs.blender.org/manual/en/5.0/editors/image/image_settings.html and https://docs.blender.org/manual/en/5.0/editors/image/editing.html — checked for the "Opening an Image Sequence" reference. Relevant only to loading, not to writing a movie; **ruled out** as an answer source.
- `render/eevee/limitations/limitations.txt` → https://docs.blender.org/manual/en/5.0/render/eevee/limitations/limitations.html — checked for the only OS-specific rendering limitation in the manual.

**Manual pictures opened:** `_images/render_output_properties_post-processing_panel.png` and `_images/render_output_properties_output_video-encoding-panel.png`. Both described above.

**API pages opened:**

- `bpy.ops.sequencer.txt` → https://docs.blender.org/api/5.0/bpy.ops.sequencer.html — read the `image_strip_add` entry and its options.
- `bpy.types.StripsTopLevel.txt` → https://docs.blender.org/api/5.0/bpy.types.StripsTopLevel.html — read `new_image`, `new_movie`, `new_sound`.
- `bpy.types.StripElements.txt` → https://docs.blender.org/api/5.0/bpy.types.StripElements.html — read in full.
- `bpy.types.ImageStrip.txt` → https://docs.blender.org/api/5.0/bpy.types.ImageStrip.html — read the opening and properties.
- `bpy.types.RenderSettings.txt` → https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read `use_sequencer` and `use_sequencer_override_scene_strip`.
- `bpy.ops.render.txt` → https://docs.blender.org/api/5.0/bpy.ops.render.html — read `render`, `opengl`, `play_rendered_anim`. `play_rendered_anim` ("Play back rendered frames/movies using an external player") **ruled out**: playback, not writing.
- `bpy.types.ImageFormatSettings.txt` → https://docs.blender.org/api/5.0/bpy.types.ImageFormatSettings.html — read `media_type`, `file_format`, and neighbours.
- `bpy_types_enum_items/image_type_all_items.txt` → https://docs.blender.org/api/5.0/bpy_types_enum_items/image_type_all_items.html — read in full.
- `bpy.types.FFmpegSettings.txt` → https://docs.blender.org/api/5.0/bpy.types.FFmpegSettings.html — read `format` and the neighbouring encoding settings.
- `bpy.types.Image.txt` → https://docs.blender.org/api/5.0/bpy.types.Image.html — read the `source` enumeration.
- `bpy.types.SequenceEditor.txt` → https://docs.blender.org/api/5.0/bpy.types.SequenceEditor.html — searched for "background"; the only hit is `use_prefetch`, which is about playback speed, not about running without a window. **Ruled out.**

**Release notes pages opened:**

- `sequencer.md` → https://developer.blender.org/docs/release_notes/5.0/sequencer/ — read in full.
- `python_api.md` → https://developer.blender.org/docs/release_notes/5.0/python_api/ — read the "Image & Movies" and "VSE" sections.
- `pipeline_io.md` → https://developer.blender.org/docs/release_notes/5.0/pipeline_io/ — read the "Images & Movies" section.
- `core.md` → https://developer.blender.org/docs/release_notes/5.0/core/ — read the command-line flag table.
- `compositor.md` → https://developer.blender.org/docs/release_notes/5.0/compositor/ — read the File Output node changes, since the Compositor is the manual's fallback route.
- `color_management.md`, `rendering.md`, `bugfixes.md`, `corrective_releases.md`, `index.md` — searched, not read in full. Nothing in them adds to or contradicts the answer. `corrective_releases.md` contains two Sequencer entries, neither about writing a movie from frames.

**Cycles files:** searched all of `cycles_5.0` for `sequencer`, `ffmpeg` and `image_sequence`. Exactly one hit, `ui.py` line 1126, quoted above. Cycles has nothing else to say on this; it is a render engine, and the frames-into-a-movie job is not its work.

**Requirements page:** searched `requirements_2026-08-01.txt` for "5.0" and the operating system names, for the operating-system line only.

**Things I looked for and did not find anywhere:** any built-in single command that converts a folder of frames into a movie without going through the Sequencer or the Compositor; any animated-image output format such as GIF; any statement that a Sequencer render does or does not work with `-b`; any statement that this differs by operating system beyond the "not available on all systems" format list.

---

## Things a person with Blender 5.0 open could settle by hand in a few minutes

Items 1, 2, 3, 4, 5 and 7 on this list have been run and their results are in the "Observed" section above. Item 6 is answered too: the complete list of formats this version's output setting will take is `JPEG, OPEN_EXR, PNG, WEBP, BMP, CINEON, DPX, IRIS, JPEG2000, HDR, TARGA, TARGA_RAW, TIFF, OPEN_EXR_MULTILAYER, FFMPEG`, which holds no animated single-image format.

Item 6, which containers and codecs the menus actually offer, was run by the Creator on 2026-08-04 and is written in above.

Every one of them was then run again on a second machine — a rented Linux machine, Ubuntu 22.04.5 LTS with an NVIDIA GeForce RTX 4090 — on 2026-08-06, and the results are written in above beside the Mac's. That second list is at `Audits/Blender Research/Images/Hand tests — Linux.md`.

What is still open:

1. **Windows.** Everything observed above was got on macOS on Apple silicon and on Ubuntu on an NVIDIA card, and on both. No Windows machine has run it, so that system stays untried.

---

2026-08-03 — Opus 5 (Claude Code) — Images, Blender 5.0
