# Images — Blender 5.1 — second pass (question 7 only)

**Sources consulted, and when.** Blender 5.1 manual (`Blender Sources/manual_5.1`, including its `_images` folder), Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`). All four were taken on 2026-07-30, and that is the date I read them.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

This pass answers question 7 alone. I did not open the existing findings document for this version.

---

## 7. Can Blender turn a set of finished frames into a short moving picture?

**Answer: yes.** The manual describes this as a normal, recommended way of working: render the animation out as numbered still image files, load those files back in as an image sequence, and have Blender write a video file from them.

### Label

**documented** — for the central claim (Blender can take already-rendered numbered still frames and write them out as a video file), and for the named parts that do it.

Two supporting points below are separately labelled: one **observed** (from Cycles' own files), and the "outside the interface" point is **inferred**, with the step written out.

### Reading of the question

I read "a set of finished frames" as a set of already-rendered still image files sitting on disk, and "a short moving picture" as a video file — a movie. That is the reading the manual's own workflow matches.

The question is ambiguous in one way, and I am flagging it rather than choosing silently. "Turn into a short moving picture" could also be read as *play them back as an animation without writing anything new* — showing motion on screen rather than producing a file. That reading is also answered yes, and by a different feature: the Animation Player. I cover it briefly at the end of this answer so the other reading is not left hanging.

### What the sources say

**The overview says it plainly.**

> "Images can then be encoded to a video by adding the rendered image sequence into
> the Video Sequencer and choosing an appropriate
> Video Output."

- Source: manual — https://docs.blender.org/manual/en/5.1/render/output/introduction.html
- Date read: 2026-07-30

**The rendering-animations page describes it as one of two named approaches, and gives the steps.**

> "Finally you can render out from the Sequencer and compress the frames into a video file."

> "The Frame Sequence is a much more stable approach,
> where you set your output format to a still format (such as JPG, PNG or a multi-layer format).
> Click Animation to render your scene out to a set of images,
> where each image is a frame in the sequence."

> "Blender creates a file for each frame of the animation.
> You can then use Blender’s Compositor to perform any frame manipulation (post-processing).
> You can then use Blender’s Video Sequencer to load that final image sequence,
> add an audio track to the animation, and render out to an MPEG format to complete your movie."

The same page then gives the step-by-step, of which the last three steps are the ones that matter here:

> "Choose Add Image from the add menu. Select all the frames from your output folder that you want to include
> in your animation. They will be added as a strip in the Sequence editor."

> "In the Output panel, choose the container and codec you want (e.g. MPEG H.264) and configure them.
> The video codecs are described in Output Options."

> "Click the Animation render button and Blender will render out the Sequence editor output into a movie."

And, under Hints:

> "You can then make a movie out of the separate frames with Blender’s Sequence editor
> or use 3rd party encoding software."

- Source: manual — https://docs.blender.org/manual/en/5.1/render/output/animation.html
- Date read: 2026-07-30

**The video-formats page states what does the encoding, and says outright that this is what these formats are for.**

> "Blender uses FFmpeg to handle video encoding and decoding.
> These formats are primarily used to compress rendered image sequences into playable movies."

The same page lists the containers Blender can write (MPEG-4, Matroska, WebM, AVI, DV, Flash, MPEG-1, MPEG-2, Ogg, QuickTime) and the video codecs (AV1, H.264, H.265/HEVC, WEBM/VP9, DNxHD, DV, "FFmpeg video codec #1", Flash Video, HuffYUV, MPEG-1, MPEG-2, MPEG-4(DivX), ProRes, PNG, QuickTime Animation, Theora), plus "No Video: For audio-only encoding."

- Source: manual — https://docs.blender.org/manual/en/5.1/files/media/video_formats.html
- Date read: 2026-07-30

**The strip that holds the frames is documented as its own thing.**

> "Image strips are used to add still images or numbered image sequences
> to the Video Sequencer. They are commonly used for slides, graphic overlays,
> background plates, rendered image sequences, or frame-by-frame animation."

> "In the case of (numbered) image sequences
> (e.g. *-0001.jpg, *-0002.jpg, *-0003.jpg, etc, of any image format), you have a choice:"

Frames may be a mix of formats:

> "Shift-LMB click selected non-related stills for batch processing; each image will be one frame,
> in sort order, and can be a mix of file types (jpg, png, exr, etc.)."

Missing frames are handled explicitly:

> "This allows you to render an image sequence with a few frames missing, while ensuring that the image sequence strip
> maintains the correct length. Placeholder frames are rendered in pink in the VSE preview."

- Source: manual — https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/image.html
- Date read: 2026-07-30

**The setting that makes a render come from the Sequencer rather than from the 3D scene.**

> "Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene’s active camera.
> If the sequence contains Scene strips, these will also be rendered as part of the pipeline.
> If Compositing is also enabled, the Scene strip will be the output of the Compositor."

- Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html
- Date read: 2026-07-30

I also opened the picture on that page, `_images/render_output_properties_post-processing_panel.png` (shown on the page above). It shows a "Post Processing" panel with a "Pipeline" row containing two ticked checkboxes labelled "Compositing" and "Sequencer", and below them a "Dither" slider reading 1.000. Label for what I take from the picture: **documented** — the two checkboxes exist, side by side, both ticked in that screenshot.

**The setting that makes the written file a video rather than stills.**

> "Media Type
>
> The type of media to save."

> "Video:
>
> Encodes each frame into a video container.
> Encoding options can be found in the Encoding panel."

And the Encoding panel:

> "Here you choose which video container, codec, and compression settings you want to use.
> With all of these compression choices, there is a trade-off between file size,
> compatibility across platforms, and playback quality."

The same page recommends this route over rendering straight to video:

> "It’s not recommended to render directly to a video format in the first instance.
> If a problem occurs while rendering, the file might become unplayable and you will
> have to re-render all frames from the beginning. If you first render out a set
> of static images such as the default PNG format or the higher-quality OpenEXR
> (which can retain HDR pixel data), you can combine them as
> an Image Strip
> in the Video Sequencer."

and lists as one of the benefits:

> "Try out different video encoding options in seconds,
> rather than minutes or hours as encoding is usually much faster than rendering the 3D scene."

- Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
- Date read: 2026-07-30

I opened two pictures from that page. `_images/render_output_properties_output_panel.png` shows an "Output" panel with a path field reading `/tmp/`, checkboxes Overwrite, Placeholders, File Extensions, Cache Result, and "File Format" set to PNG with Color BW/RGB/RGBA, Color Depth 8/16, Compression 15%. `_images/render_output_properties_output_video-encoding-panel.png` shows the same panel with "File Format" set to "FFmpeg video", and below it an "Encoding" section (Container: Matroska; Autosplit Output), a "Video" section (Video Codec: H.264; Output quality: Medium quality; Encoding speed: Good; Keyframe interval: 18; Max B-frames, unticked, 0) and an "Audio" section (Audio Codec: Vorbis; Bitrate 192; Volume 1.000). Label: **documented**. One thing worth saying plainly: the picture labels the control "File Format" and its video entry "FFmpeg video", while the text on the same page calls the control "Media Type" with an entry "Video". I am reporting both as they stand and not reconciling them.

**A documented limit on this route, and the documented way round it.**

> "The Video Sequencer does not support multi-layer EXR files.
> To render to a video format you will have to skip the next three steps and
> instead use an Image Input node
> in the Compositor."

- Source: manual — https://docs.blender.org/manual/en/5.1/render/output/animation.html
- Date read: 2026-07-30

The Compositor's Image node is documented as accepting anything Blender reads:

> "The Image node injects any image format that is supported by Blender."

- Source: manual — https://docs.blender.org/manual/en/5.1/compositing/types/input/image.html
- Date read: 2026-07-30

**Two ways to set it going from inside Blender's window.** The top bar's Render menu:

> "Render Animation Ctrl-F12
>
> Render the animation of the active scene."

- Source: manual — https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html
- Date read: 2026-07-30

and the Video Sequencer's own menu:

> "Sequence Render Animation
>
> Save previews of the frames in the scene range (or the preview range, if active) to a video file
> or a series of image files. See the Output panel for details."

with a caution attached:

> "Sequence Render Image and Sequence Render Animation don’t render the final video by default –
> specifically, they don’t render Scene Strips, instead using the preview’s
> shading mode (which is initially Solid).
>
> To output a video where the Scene Strips are rendered, use the Render menu in the top-bar,
> or change Sidebar ‣ View ‣ Scene Strip Display ‣ Shading to Rendered."

- Source: manual — https://docs.blender.org/manual/en/5.1/editors/video_sequencer/sequencer/navigating.html
- Date read: 2026-07-30

(That caution is about Scene strips — strips that are a 3D scene. It does not say anything against image strips, which are what a set of finished frames becomes.)

**What the same thing looks like from Python.**

> "use_sequencer
>
> Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist (default True)"

- Source: Python API — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
- Date read: 2026-07-30

> "media_type
>
> The type of media to save (default 'IMAGE')
>
> Type:
>
> Literal[‘IMAGE’, ‘MULTI_LAYER_IMAGE’, ‘VIDEO’]"

- Source: Python API — https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html
- Date read: 2026-07-30

> "bpy.ops.sequencer.image_strip_add(*, directory='', files=None, … image_import_type='DETECT', use_sequence_detection=True, use_placeholders=False)
>
> Add an image or image sequence to the sequencer"

- Source: Python API — https://docs.blender.org/api/5.1/bpy.ops.sequencer.html
- Date read: 2026-07-30

> "new_image(name, filepath, channel, frame_start, *, fit_method='ORIGINAL')
>
> Add a new image strip"

- Source: Python API — https://docs.blender.org/api/5.1/bpy.types.StripsTopLevel.html
- Date read: 2026-07-30

> "append(filename)
>
> Push an image from ImageStrip.directory"

- Source: Python API — https://docs.blender.org/api/5.1/bpy.types.StripElements.html
- Date read: 2026-07-30

> "sequence_editor_create()
>
> Ensure sequence editor is valid in this scene"

- Source: Python API — https://docs.blender.org/api/5.1/bpy.types.Scene.html
- Date read: 2026-07-30

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"
>
> "animation (bool) – Animation, Render files from the animation range of this scene (optional)"

- Source: Python API — https://docs.blender.org/api/5.1/bpy.ops.render.html
- Date read: 2026-07-30

**From Cycles' own files — observed.** Cycles draws the Sequencer pipeline toggle in its own Output properties panel:

```
1112:class CYCLES_PT_post_processing(CyclesButtonsPanel, Panel):
1113:    bl_label = "Post Processing"
1114:    bl_options = {'DEFAULT_CLOSED'}
1115:    bl_context = "output"
…
1124:        col = layout.column(align=True, heading="Pipeline")
1125:        col.prop(rd, "use_compositing")
1126:        col.prop(rd, "use_sequencer")
```

- Source: `Blender Sources/cycles_5.1/ui.py`, lines 1112–1126
- Date read: 2026-07-30
- Label: **observed**. What this shows is that Cycles presents the `use_sequencer` setting to the user in a panel headed "Pipeline" under Output. It does not by itself show what a render does when the setting is on — that would be a step past what the file says.

**Release notes for 5.1.** The release notes do not announce this ability (it is not new), but they do record changes to the video encoding around it:

> "## Video & Audio output
>
> - Video encoding now allows "Custom" quality level by manually specified ffmpeg CRF value. (blender/blender!151264)
> - Audio bitrate can be set to higher values than 384kb/s now (up to 2048kb/s, depending on audio codec limits). (blender/blender!151126)"

- Source: release notes — https://developer.blender.org/docs/release_notes/5.1/pipeline_io/
- Date read: 2026-07-30

> "- Render settings audio bitrate can be set to higher values than 384kb/s now (up to 2048kb/s, depending on audio codec limits). (blender/blender!151126)"

- Source: release notes — https://developer.blender.org/docs/release_notes/5.1/sequencer/
- Date read: 2026-07-30

The 5.1 sequencer release notes also list a set of Python property renames on strips (`frame_final_duration` → `duration`, `frame_final_start` → `left_handle`, `frame_final_end` → `right_handle`, `frame_offset_start` → `left_handle_offset`, `frame_offset_end` → `right_handle_offset`, `frame_duration` → `content_duration`, `frame_start` → `content_start`, `animation_offset_start` → `content_trim_start`, `animation_offset_end` → `content_trim_end`), with:

> "Old properties will still be available, but are deprecated (to be removed in 6.0, see blender/blender!153012)."

- Source: release notes — https://developer.blender.org/docs/release_notes/5.1/sequencer/
- Date read: 2026-07-30

I found nothing in any of the three documentation sources for 5.1 saying this ability was removed, restricted or changed in kind.

### Observed — hand test, 2026-08-03

Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 5.1.2, started from a terminal with no window throughout.

**This settles the disagreement between this document and `Preparation/Blender Research/Findings/5.1/Images.md`, which leaves the assembly half unresolved. The observed result below is what now stands, and the same result is written into that document too.**

*The whole job with no window at all.* Set up, in a windowless Blender: six PNG frames at 64 by 48 rendered from the ordinary scene into a folder; then the 3D scene changed so a fresh render could not be mistaken for those frames — the cube deleted and the world background set to pure blue; then an image strip built from the six saved files with `scene.sequence_editor_create()` and `strips.new_image(name="Frames", filepath=<first file>, channel=1, frame_start=1)` followed by `strip.elements.append(<each remaining file name>)`. Blender printed `STRIP: Frames type: IMAGE elements: 6` and `use_sequencer: True`. Four blend-files were saved from that state: video output with the Sequencer pipeline on, video output with it off, PNG output with it on, PNG output with it off.

Done, in a terminal:

```
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b -noaudio --factory-startup /tmp/images-hand-tests/out/t16/5.1.2/sequence-test.blend -a
```

Blender printed, for each frame, `Rendering frame N`, `Executing sequencer`, `Video append frame N`. The exit status was `0`, and one file appeared at the output path: `out_0001-0006.mp4`, which `file` reported as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`. The same command on the file with the Sequencer pipeline switched off wrote `nosec_0001-0006.mp4`.

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
s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3254
s_0002.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3254
s_0003.png   size=(64, 48) centre=(0.0, 0.0, 0.0, 0.0) corner=(0.0, 0.0, 0.0, 0.0) bytes=1243
s_0004.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3254
s_0005.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3254
```

The frame whose file was gone came out fully transparent black.

*Multi-layer OpenEXR frames offered to the Video Sequencer.* Set up: three multi-layer OpenEXR frames at 64 by 48 with the depth and normal passes on; a strip built from them; PNG output. Done: the same background animation render. Three files were written, with no message about the format:

```
s_0001.png   size=(64, 48) centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=3002
s_0002.png   size=(64, 48) centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=3002
s_0003.png   size=(64, 48) centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=3002
```

Compared with the same scene's PNG frames put through the same route, which gave `centre=(0.5373, ...)` and `corner=(0.2, ...)`, the picture is there but darker.

*How long the result is.* Twenty-four frames, a strip built from all of them, MPEG-4 container and H.264 codec, encoded once at `render.fps = 24` and once at `render.fps = 12`. Each movie's own header records:

```
at24_0001-0024.mp4   bytes=2314   timescale=1000 duration=1000 -> 1.000 s
at12_0001-0024.mp4   bytes=2592   timescale=1000 duration=2000 -> 2.000 s
```

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Blender 5.1.2 open.

*Which control is it.* **Both.** The Output panel has two rows, one above the other: the upper is labelled `Media Type` and offers `Image`, `Multi-Layer EXR` and `Video`; the lower is labelled `File Format` and holds the still-image formats, each with its extension in the label, such as `PNG (.png)` and `OpenEXR (.exr)`. So the manual's text and the manual's own screenshot were describing two different rows of the same panel, not disagreeing about one. With `Media Type` set to `Multi-Layer EXR` the `File Format` row disappears altogether.

*Which containers and codecs are offered.* With `Media Type` set to `Video`, the `Encoding` section's `Container` menu offers `MPEG-4`, `Matroska`, `WebM`, then a dividing line, then `AVI`, `DV`, `Flash`, `MPEG-1`, `MPEG-2`, `Ogg`, `QuickTime`. The `Video Codec` menu offers `No Video`, `AV1`, `H.264`, `H.265 / HEVC`, `WebM / VP9`, then a dividing line, then `DNxHD`, `DV`, `FFmpeg video codec #1`, `Flash Video`, `HuffYUV`, `MPEG-1`, `MPEG-2`, `MPEG-4 (divx)`, `PNG`, `ProRes`, `QuickTime Animation`, `Theora`. That settles the "not available on all systems" caveat for this machine: video output is really there, and the container and codec used above are both on those menus.

The whole list of tests, with the exact commands, is at `Audits/Blender Research/Images/Hand tests.md`.

### Observed — the same tests on a second machine, 2026-08-06

Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, with no window throughout, driven by the same scripts the Mac run used, with only the folder path changed. The whole list is at `Audits/Blender Research/Images/Hand tests — Linux.md`.

*The whole job with no window at all.* `blender -b -noaudio --factory-startup .../sequence-test.blend -a` printed `Executing sequencer` and `Video append frame 1` through `Video append frame 6` on timestamped lines, the exit status was `0`, and `out_0001-0006.mp4` appeared, reported by `file` as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`. Rendered again to PNG, the frames read:

```
frames       f_0001.png   size=(64, 48) centre=(0.5451, 0.5451, 0.5451, 1.0) corner=(0.2275, 0.2275, 0.2275, 1.0)
png_seq_on   s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0)
png_seq_off  s_0001.png   size=(64, 48) centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

*A folder of frames with nothing prepared.* The same command wrote `n_0001-0003.mkv`, reported as `Matroska data` — a movie of the 3D scene.

*A frame missing from the set.* All five output frames were written, nothing was printed about the missing one, and the third came out fully transparent black: `s_0003.png size=(64, 48) centre=(0.0, 0.0, 0.0, 0.0) corner=(0.0, 0.0, 0.0, 0.0) bytes=1809`.

*Multi-layer OpenEXR frames offered to the Video Sequencer.* Three files written with no message about the format, `centre=(0.2599, 0.2599, 0.2599, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0)`.

*How long the result is.* The two movies' own headers record:

```
at24_0001-0024.mp4 bytes=2314   timescale=1000 duration=1000 -> 1.000 s
at12_0001-0024.mp4 bytes=2597   timescale=1000 duration=2000 -> 2.000 s
```

Both machines were run and both agreed on every one of these, with only the byte sizes of the written files differing by a few bytes.

*Which container and codec will actually take it.* A new test, run on both machines and recorded in full under question 4 in `Preparation/Blender Research/Findings/5.1/Images.md`, encoded three frames for every container with H.264 and for the MPEG-4 container with every codec. Ten of those pairings are refused with `Error: Could not initialize streams, probably unsupported codec combination`, and three more with a complaint about the picture size. So the container and codec menus offer more pairings than this version will write.

### The other reading of the question — playing frames back without writing a file

> "The animation player is a utility typically used for previewing rendered animations,
> supporting all image and video formats also supported by Blender.
> This is a convenient way to play back image sequences at the correct frame rate."

> "Launching the animation player opens a new window,
> playing back images or a video located at the render output of the current scene."

- Source: manual — https://docs.blender.org/manual/en/5.1/render/output/animation_player.html
- Date read: 2026-07-30
- Label: **documented**

### Outside the interface

**Observed — yes, the whole job.** The hand test above did both halves with no window open: the image strip was built from the finished frames by a script in a windowless Blender, and the movie was encoded by `blender -b <file> -a` from a terminal, with Blender printing `Executing sequencer` for each frame. The written frames prove the pictures came from the strip and not from a fresh 3D render. So the step named below as inferred — that the Video Sequencer encodes to video when Blender is run in background mode — is now observed to hold on that machine. (Previously, before the test: partly documented, with the last step inferred.)

What was documented, from the command-line page:

> "-b, --background
>
> Run in background (often used for UI-less rendering)."

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

> "-F, --render-format <format>
>
> Set the render format.
> Valid options are:
> TGA RAWTGA JPEG IRIS PNG BMP HDR TIFF.
>
> Formats that can be compiled into Blender, not available on all systems:
> OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

- Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
- Date read: 2026-07-30

And a worked example of a background animation render:

> "blender --background --enable-autoexec my_movie.blend --render-anim"

- Source: manual — https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html
- Date read: 2026-07-30

**The step I took, so it can be checked.** This paragraph is kept as a record of the reasoning before the hand test above confirmed it; the last sentence's call for a hand check has since been answered, not left open. The sources never say in so many words "the Video Sequencer encodes to video when Blender is run in background mode". What they do say, separately, is (a) `use_sequencer` is a scene render setting whose description is "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist", with default True — a scene setting saved in the file, not a window state; (b) `-a/--render-anim` renders the animation, and `-b` runs with no interface; (c) `FFMPEG` is a valid `-F` render format. Putting those three together gives: a blend-file that already holds an image-sequence strip, run with `-b … -F FFMPEG -a`, should write a video. That was a step past the quoted words, so it was labelled **inferred**, not documented — and the hand test above confirmed it.

**Where it does differ, documented.** Two of the routes above are interface-only by their own description:

- The Sequencer's own "Sequence Render Animation" is a menu entry in an editor, and its behaviour depends on preview shading and on a Sidebar setting — quoted above from https://docs.blender.org/manual/en/5.1/editors/video_sequencer/sequencer/navigating.html
- The Animation Player is explicitly disabled without an interface: "Instead of showing Blender’s user interface, this runs Blender as an animation player, to view movies and image sequences rendered in Blender (ignored if -b is set)." (command-line page, address above.) So the *playback* reading of the question does not hold outside the interface.

One documented note about running without the window, on where the encoder's own messages go:

> "When you view the System Console,
> you can see some of the output of the encoding process.
> You will see even more output if you execute Blender as blender -d."

- Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
- Date read: 2026-07-30

### Operating system

**Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The whole job — building the strip with no window and encoding it from a terminal — works on both, and the frames, the file types and the recorded durations match. That closes the gap this section used to name: a render happens on a rented Linux machine, and the job has now been done there. Both builds have FFmpeg compiled in, and the MPEG-4 container with the H.264 codec wrote a playable file on both. **Windows is still untried** — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

What the sources say, which still stands, and it matters: the video format is not guaranteed to be present in every build.

> "Formats that can be compiled into Blender, not available on all systems:
> OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

- Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
- Date read: 2026-07-30

Documented, about the codecs themselves:

> "Because codecs are required for both encoding and decoding,
> they must be available on both the system creating the file and the device playing it."

> "Not all codecs are available in all containers."

- Source: manual — https://docs.blender.org/manual/en/5.1/files/media/video_formats.html
- Date read: 2026-07-30

The manual does contain one operating-system-specific statement about rendering with no interface, but it is about the EEVEE render engine, not about the Sequencer or about encoding:

> "Headless Rendering
>
> Headless rendering is not supported on headless Windows systems."

- Source: manual — https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html
- Date read: 2026-07-30

Turning a set of finished frames into a video uses image strips, which are files being read from disk, and no 3D render engine is named anywhere in that workflow. So I do not carry the EEVEE statement across to it. Beyond these, **the sources do not say** whether the image-sequence-to-video route behaves differently on Windows, macOS or Linux, and I am not going to guess.

### Not covered

What this answer does not settle:

- It does not settle what a render actually *does* when a sequencer strip is present. Every claim above is about what settings and operators exist and what the manual instructs; none of it is me watching a file appear.
- It does not settle whether the Sequencer pipeline runs when Blender has no interface. That is the inferred step, flagged above.
- It does not settle how long "short" may be. No maximum length is stated anywhere I searched. The one size limit found is on the output file: "If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size." (Output page, address above.) There is also "Some codecs impose output size restrictions. For example, H.264 requires both width and height to be divisible by 2." (video formats page) — that is picture size, not length.
- It does not settle what happens if frames in the set are missing at the moment of encoding. The Use Placeholders option is documented for *adding* a strip and for keeping its length right, but the sources do not say what a render writes for a placeholder frame that is still missing.
- It does not settle whether the frame rate of the resulting video comes from the scene or from anywhere else. The manual's workflow says to set "the frame rate, which should already be set" in the Format panel, and I did not chase the Format panel further, as that is a different question from the one asked.
- It does not cover animated GIF. I searched for it (see record below) and found no GIF output format in 5.1's sources; the manual's own contributor guide says "Avoid .gif; use embedded videos for animations instead", which is advice about writing the manual, not about Blender's output formats. So: no evidence either way beyond GIF's absence from every format list I read.
- It does not settle whether audio must be present or absent. Audio appears throughout the workflow as an option; I did not test the claim.

### Record of where I looked

The brief asks for this in full, so here it is: files searched, terms searched, pages opened, and what I ruled out.

**Searched across the whole of each source** (`grep -ri`, case-insensitive, whole folder at once), in `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1`:

`image sequence`, `ffmpeg`, `encode` / `encod`, `codec`, `container`, `sequencer`, `Video Sequencer`, `sequence editor`, `movie file`, `video file`, `render out`, `compile`, `assemble`, `stitch`, `flipbook`, `playblast`, `timelapse`, `slideshow`, `contact sheet`, `frames into`, `gif`, `use_sequencer`, `new_image`, `image_strip_add`, `background mode`, `headless`, `--background`, `Render Animation`, `post processing`, `Opening an Image Sequence`.

Terms that returned nothing at all, in any of the three documentation sources: `flipbook`, `timelapse`, `slideshow`, `contact sheet`. `playblast` returned one manual hit and one release-note hit, both about stereo 3D bug fixes, not about building a video from stills — ruled out. `gif` returned only image *filenames* used as illustrations in the manual, a line of manual-writing advice, and one aside in `bpy_extras.image_utils` about a conversion callback taking "the path for a GIF (for example)" — no GIF output format anywhere, ruled out.

**Manual pages opened in full or in part:**

- `render/output/animation.txt` (whole) — the core of the answer
- `render/output/introduction.txt` (whole)
- `render/output/animation_player.txt` (whole)
- `render/output/properties/output.txt` (lines 1–175 and 320–373)
- `render/output/properties/post_processing.txt` (whole)
- `files/media/video_formats.txt` (whole)
- `files/media/image_formats.txt` (the "Opening an Image Sequence" section)
- `video_editing/edit/montage/strips/image.txt` (whole)
- `video_editing/introduction.txt` (whole)
- `editors/video_sequencer/sequencer/navigating.txt` (the Range / Sequence Render section)
- `editors/image/image_settings.txt` (the Source section)
- `editors/image/editing.txt` (the Open / UDIM section)
- `compositing/types/input/image.txt` (Image node)
- `interface/window_system/topbar.txt` (Render menu)
- `advanced/command_line/arguments.txt` (Render Options, Format Options, Animation Playback Options)
- `advanced/scripting/security.txt` (the background-render example)
- `render/eevee/limitations/limitations.txt` (Headless Rendering)

**Manual pictures opened** (from `manual_5.1/_images`): `render_output_properties_output_panel.png`, `render_output_properties_output_video-encoding-panel.png`, `render_output_properties_post-processing_panel.png`. I also noted but did not open `video-editing_introduction_screen-layout.png`, as the words on that page already answered what I needed.

**API pages opened:** `bpy.types.RenderSettings`, `bpy.types.ImageFormatSettings`, `bpy.types.FFmpegSettings`, `bpy.types.ImageStrip`, `bpy.types.StripsTopLevel`, `bpy.types.StripElements`, `bpy.types.SequenceEditor`, `bpy.types.Scene` (the sequence-editor methods), `bpy.ops.sequencer` (the `image_strip_add` and `movie_strip_add` entries), `bpy.ops.render` (the whole operator list, plus `render`, `opengl`, `play_rendered_anim`).

**Release-note files read:** `sequencer.md` (whole), `pipeline_io.md` (whole), and a folder-wide search of all 21 files in `release_notes_5.1` for the video and encoding terms above, which surfaced only `pipeline_io.md`, `sequencer.md`, `compositor.md`, `python_api.md`, `core.md` and `bugfixes.md`. Of these I ruled out `compositor.md` (its video hits are embedded demonstration clips and a Movie Distortion node fix), `core.md` (its "encoding" is run-length encoding of memory, nothing to do with video), and `bugfixes.md` (a list of fixed bugs; its video-sequencer entries are about scaling glitches, an FPS counter, scrubbing and a metadata crash, none of which speak to whether frames can be turned into a video).

**Cycles files searched:** a folder-wide search of `cycles_5.1` for `ffmpeg`, `video`, `sequencer`, `movie`, `codec` returned exactly two hits: the `use_sequencer` line in `ui.py` quoted above, and a comment containing a URL in `source/kernel/camera/projection.h` about VR video, which is unrelated and ruled out. Cycles' files otherwise have nothing to say about this question, which is what one would expect — this is not a render-engine matter.

**What I did not do:** I did not go to the web, and did not need to.

---

## Things a person with Blender open could settle in a few minutes

Items 1, 2, 4 and 7 on this list have been run and their results are in the "Observed" section above. Item 4 was answered for frames written as pictures rather than as a movie: the frame whose file was gone came out fully transparent black, with nothing printed.

Items 3 and 6 — which control it is, and which containers and codecs the menus offer — were run by the Creator on 2026-08-04 and are written in above.

What is still open:

1. **How long the video can be.** Not tried; nothing in the sources caps it, and a very long run was out of proportion to the question.

4. **Windows.** Everything observed above was got on macOS on Apple silicon and on Ubuntu on an NVIDIA card, and on both — every test was run again on a rented Linux machine on 2026-08-06, and the second list is at `Audits/Blender Research/Images/Hand tests — Linux.md`. No Windows machine has run it, so that system stays untried.

---

2026-08-03 — Opus 5 (Claude Code) — Images, Blender 5.1
