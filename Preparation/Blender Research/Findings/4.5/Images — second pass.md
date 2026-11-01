# Images — Blender 4.5 — second pass (question 7 only)

**Sources consulted, and when.** Blender 4.5 manual (`Blender Sources/manual_4.5`, including its `_images` folder), Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`). All four were taken on 2026-07-30, and that is the date read for everything quoted below. I did the searching and reading on 2026-08-03.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I answered question 7 alone, as instructed. I did not open the existing findings document for this version.

---

## 7. Can Blender turn a set of finished frames into a short moving picture?

### How I read the question

I took "a set of finished frames" to mean a series of already-rendered still image files sitting in a folder, and "a short moving picture" to mean a single video file that plays those frames in order.

The question is mildly ambiguous. A second reading is "can Blender show them moving", meaning play them back as motion without writing anything new to disk. I answer the first reading as the main answer, and cover the second reading briefly at the end, because the sources treat it as a separate feature with its own page.

### Answer

**Yes.** The manual describes this as a normal, named workflow: render an animation out as separate image files, load those files into the Video Sequencer as one strip, set the output format to a video format, and render again — the second render writes a movie file instead of stills.

**LABEL: documented.**

#### The workflow, in the manual's own words

The manual's "Rendering Animations" page describes two ways of making a movie and calls the image-files-first way the "Frame Sequence" approach. Its summary of that approach:

> "The Frame Sequence is a much more stable approach, where you set your output format to a still format (such as JPG, PNG or a multi-layer format). Click Animation to render your scene out to a set of images, where each image is a frame in the sequence."

> "Blender creates a file for each frame of the animation. You can then use Blender's Compositor to perform any frame manipulation (post-processing). You can then use Blender's Video Sequencer to load that final image sequence, add an audio track to the animation, and render out to an MPEG format to complete your movie."

Earlier on the same page:

> "Finally you can render out from the Sequencer and compress the frames into a playable movie clip."

The same page then gives the step-by-step. The steps that matter here:

> "In Blender, open the Video Sequencer."

> "Choose Add Image from the add menu. Select all the frames from your output folder that you want to include in your animation. They will be added as a strip in the Sequence editor."

> "In the Output panel, choose the container and codec you want (e.g. MPEG H.264) and configure them. The video codecs are described in Output Options."

> "Click the Animation render button and Blender will render out the Sequence editor output into a movie."

And, under "Hints", the same thing stated plainly as a recovery route:

> "You can then make a movie out of the separate frames with Blender's Sequence editor or use 3rd party encoding software."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30.

#### One stated limitation on the input files

The same page carries a note in the middle of those steps:

> "The Video Sequencer does not support multi-layer EXR files. To render to a video format you will have to skip the next three steps and instead use an Image Input node in the Compositor."

So multi-layer EXR frames cannot go through the Video Sequencer route; the manual sends them through the Compositor instead. Source: Blender manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30.

**LABEL: documented.**

#### What the Video Sequencer accepts as "a set of frames"

The Image/Sequence strip page states what a numbered set of files becomes:

> "In the case of (numbered) image sequences (e.g. *-0001.jpg, *-0002.jpg, *-0003.jpg, etc, of any image format), you have a choice:"

and, for the "Batch" choice:

> "Shift-LMB click selected non-related stills for batch processing; each image will be one frame, in sort order, and can be a mix of file types (jpg, png, exr, etc.)."

The same page covers gaps in the set:

> "Image sequences can use placeholder files. This works by enabling Use Placeholders checkbox when adding an image strip. The option detects the frame range of opened images using Blender's frame naming scheme (filename + frame number + .extension) and makes an image sequence with all files in between even if they are missing. This allows you to render an image sequence with a few frames missing and still the image strip will have the correct range to account for the missing frames displayed as black."

> "When the missing frames are rendered or placed in the same folder, you can refresh the Sequencer and get the missing frames in the strip."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/image.html — read 2026-07-30.

And the editor's own introduction states its purpose:

> "The Video Sequencer allows you to place images, videos, sounds, and scenes on a timeline and combine them into a new video."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/editors/video_sequencer/introduction.html — read 2026-07-30.

**LABEL: documented.**

#### The switch that makes the second render produce the strip rather than the 3D scene

The Output properties have a "Post Processing" panel with a Sequencer option:

> "Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html — read 2026-07-30.

The Python API describes the same switch and gives its default:

> "use_sequencer — Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist"
> "Type: boolean, default True"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.

**LABEL: documented.**

#### That the output can be a video file at all

The output format page:

> "Choose the file format to save to. Based on which format is used, other options such as channels, bit depth and compression level are available."
> "For rendering out to images see: saving images, for rendering to videos see the Encoding panel."

and the Encoding panel:

> "Here you choose which video container, codec, and compression settings you want to use. With all of these compression choices, there is a trade-off between file size, compatibility across platforms, and playback quality."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30.

The supported-formats page states what does the encoding:

> "Blender used FFmpeg to handle video encoding/decoding various video formats. These formats are primarily used for compressing rendered sequences into a playable movie."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/files/media/video_formats.html — read 2026-07-30.

The Python API's list of output file formats splits into an "Image" group and a "Movie" group, and the Movie group has one entry:

> "Movie"
> "FFMPEG:"
> "FFmpeg Video."

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy_types_enum_items/image_type_items.html — read 2026-07-30.

The containers available for it:

> "format — Output file container"
> "Type: enum in ['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME'], default 'MKV'"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.FFmpegSettings.html — read 2026-07-30.

**LABEL: documented.**

#### Two pictures from the manual

I opened `_images/video-editing_sequencer_strips_introduction_add-menu.png` (the picture named on the manual's strips introduction page). It is a screenshot of the Video Sequencer's Add menu. Visible entries, top to bottom: Add / Scene / Clip / Mask / Movie / Sound / Image/Sequence / Color / Text / Adjustment Layer / Effect Strip / Transitions (greyed out). This confirms by sight that "Image/Sequence" is an entry in that menu.

I opened `_images/render_output_properties_output_video-encoding-panel.png` (the picture named on the Output properties page). It is a screenshot of the Output panel with a video format chosen. Visible: File Path `/tmp/`; checkboxes Overwrite (ticked, greyed), Placeholders (greyed), File Extensions (ticked), Cache Result; File Format set to "FFmpeg video"; Color BW / RGB with RGB selected; an Encoding section with Container "Matroska" and an Autosplit Output checkbox; a Video section with Video Codec "H.264", Output quality "Medium quality", Encoding speed "Good", Keyframe interval 18, Max B-frames 0; an Audio section with Audio Codec "Vorbis", Bitrate 192, Volume 1.000.

**LABEL: documented** (these are pages of the manual). What I take from them is only what is visible in them: the menu entry exists, and the output panel offers a video file format with container and codec choices underneath.

Date read: 2026-07-30.

#### The same thing from the Python side

The operator that adds the strip:

> "bpy.ops.sequencer.image_strip_add(*, directory='', files=None, ... frame_start=0, frame_end=0, channel=1, replace_sel=True, ... fit_method='FIT', set_view_transform=True, use_placeholders=False)"
> "Add an image or image sequence to the sequencer"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.ops.sequencer.html — read 2026-07-30.

The strip type itself:

> "class bpy.types.ImageStrip(Strip)"
> "Sequence strip to load one or more images"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.ImageStrip.html — read 2026-07-30.

Creating one directly, and adding frames to it one at a time:

> "new_image(name, filepath, channel, frame_start, *, fit_method='ORIGINAL')"
> "Add a new image strip"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.StripsTopLevel.html — read 2026-07-30.

> "append(filename)"
> "Push an image from ImageStrip.directory"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.types.StripElements.html — read 2026-07-30.

And the render call that writes the result out:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, layer='', scene='')"
> "Render active scene"
> "animation (boolean, (optional)) – Animation, Render files from the animation range of this scene"

Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.ops.render.html — read 2026-07-30.

**LABEL: documented.**

#### From Cycles' own files

Cycles draws the Post Processing panel itself, including the Sequencer switch. From `Blender Sources/cycles_4.5/ui.py`, lines 1101–1118:

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

**LABEL: observed.** What this shows is that the setting is presented in the Output properties when Cycles is the render engine, and that it reads from the scene's render settings (`context.scene.render`). It does not by itself show what a render does with that setting — that step would be inferred, and I do not take it here; the manual and API quotes above are what carry the behaviour claim.

File and place: `Blender Sources/cycles_4.5/ui.py`, lines 1101–1118. Read 2026-07-30.

#### What the release notes say about this in 4.5

Nothing in the 4.5 release notes removes or replaces the route above. Three notes touch what it can write:

> "Video rendering to FFV1 codec now supports 10, 12, and 16 bits output."

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/rendering/ — read 2026-07-30.

> "Add support for writing ProRes codec videos"

> "Image Format Settings in the Render Output, File Output, and Image Saving now has a Media Type enum that specifies the general type of the file format, be it Image, Multi-Layer EXR, or Video."

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/ — read 2026-07-30.

The Video Sequencer's own release-notes page lists changes and three removals, none of which is the image strip or the ability to render out:

> "Timeline Backdrop option has been removed"
> "\"Change Effect Input\" operator was removed, since it was identical to \"Swap Effect Inputs.\""
> "Over Drop effect and blend mode was removed (it was doing exactly the same thing as regular Alpha Over since 2006). Usage of this effect in existing files is now changed to Alpha Over."

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/sequencer/ — read 2026-07-30.

**LABEL: documented.**

#### The second reading: showing frames moving without writing a file

There is a separate feature for this, and it makes no file:

> "The animation player is a utility typically used for previewing rendered animations, supporting all image and video formats also supported by Blender. This is a convenient way to play back image sequences at the correct frame rate."

> "Launching the animation player opens a new window, playing back images or a video located at the render output of the current scene. You can also drop images or movie files in a running animation player. It will then restart the player with the new data."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/render/output/animation_player.html — read 2026-07-30.

**LABEL: documented.** Note the wording is about playing back, not about producing anything. Nothing on that page says it can write a file.

### Observed — hand test, 2026-08-03

Run on the Creator's own computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, Blender 4.5.12, started from a terminal with no window throughout.

**This settles the disagreement between this document and `Preparation/Blender Research/Findings/4.5/Images.md`, which leaves the outside-the-window half for a fresh search rather than answering it. The observed result below is what now stands, and the same result is written into that document too.**

*The whole job with no window at all.* Set up, in a windowless Blender: six PNG frames at 64 by 48 rendered from the ordinary scene into a folder; then the 3D scene changed so a fresh render could not be mistaken for those frames — the cube deleted and the world background set to pure blue; then an image strip built from the six saved files with `scene.sequence_editor_create()` and `strips.new_image(name="Frames", filepath=<first file>, channel=1, frame_start=1)` followed by `strip.elements.append(<each remaining file name>)`. Blender printed `STRIP: Frames type: IMAGE elements: 6` and `use_sequencer: True`. Four blend-files were saved from that state: video output with the Sequencer pipeline on, video output with it off, PNG output with it on, PNG output with it off.

Done, in a terminal:

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b -noaudio --factory-startup /tmp/images-hand-tests/out/t16/4.5.12_LTS/sequence-test.blend -a
```

Blender printed `Append frame 1` through `Append frame 6`, the exit status was `0`, and one file appeared at the output path: `out_0001-0006.mp4`, which `file` reported as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`. The same command on the file with the Sequencer pipeline switched off wrote `nosec_0001-0006.mp4`.

To see which pictures went into it, the same two files were rendered again with PNG output instead of video, and the written frames' pixels read:

```
frames       f_0001.png   size=(64, 48) centre=(0.5451, 0.5451, 0.5451, 1.0) corner=(0.2275, 0.2275, 0.2275, 1.0)
png_seq_on   s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0)
png_seq_off  s_0001.png   size=(64, 48) centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

With the Sequencer pipeline on, what came out is the saved frames — grey cube on dark grey. With it off, what came out is the changed 3D scene — blue, no cube. The six Sequencer frames took 0.3 seconds in total; the six 3D renders took about that each.

*A folder of frames with nothing prepared.* `blender -b --factory-startup -F FFMPEG -o <folder>/n_ -a` against the plain startup file printed `Append frame 1`, `Append frame 2`, `Append frame 3` and wrote `n_0001-0003.mkv` — a movie of the 3D scene, not of any folder of pictures. There is no command-line argument that turns a folder of frames into a movie by itself.

*A frame missing from the set at the moment of encoding.* Set up: five frames written, a strip built naming all five, the file saved, and then the third frame's file deleted from disk. Done: the same background animation render, to PNG. All five output frames were written, nothing was printed about the missing one, and their pixels read:

```
s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3215
s_0002.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3215
s_0003.png   size=(64, 48) centre=(0.0, 0.0, 0.0, 0.0) corner=(0.0, 0.0, 0.0, 0.0) bytes=1175
s_0004.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3215
s_0005.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0) bytes=3215
```

The frame whose file was gone came out fully transparent black.

*Multi-layer OpenEXR frames offered to the Video Sequencer.* Set up: three multi-layer OpenEXR frames at 64 by 48 with the depth and normal passes on; a strip built from them; PNG output. Done: the same background animation render. Three files were written, with no message about the format:

```
s_0001.png   size=(64, 48) centre=(0.2609, 0.2609, 0.2609, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=3296
s_0002.png   size=(64, 48) centre=(0.2609, 0.2609, 0.2609, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=3296
s_0003.png   size=(64, 48) centre=(0.2609, 0.2609, 0.2609, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0) bytes=3296
```

Compared with the same scene's PNG frames put through the same route, which gave `centre=(0.5373, ...)` and `corner=(0.2, ...)`, the picture is there but darker.

*How long the result is.* Twenty-four frames, a strip built from all of them, MPEG-4 container and H.264 codec, encoded once at `render.fps = 24` and once at `render.fps = 12`. Each movie's own header records:

```
at24_0001-0024.mp4   bytes=2313   timescale=1000 duration=1000 -> 1.000 s
at12_0001-0024.mp4   bytes=2580   timescale=1000 duration=2000 -> 2.000 s
```

**Observed — hand test, 2026-08-04, from the Creator's own screenshots.** Taken by the Creator on the same computer: a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Blender 4.5.12 open and the output set to `FFmpeg Video`. Seen: the `Encoding` section's `Container` menu offers `MPEG-4`, `Matroska`, `WebM`, then a dividing line, then `AVI`, `DV`, `Flash`, `MPEG-1`, `MPEG-2`, `Ogg`, `QuickTime`. The `Video` section's `Video Codec` menu offers `No Video`, `AV1`, `H.264`, `H.265 / HEVC`, `WebM / VP9`, then a dividing line, then `DNxHD`, `DV`, `FFmpeg video codec #1`, `Flash Video`, `HuffYUV`, `MPEG-1`, `MPEG-2`, `MPEG-4 (divx)`, `PNG`, `ProRes`, `QuickTime Animation`, `Theora`. That settles the "not available on all systems" caveat for this machine: video output is really there, and the container and codec used above are both on those menus. The `File Format` menu in the same panel has one entry under a heading reading `Movie` — `FFmpeg Video` — and nothing animated of the single-picture kind.

The whole list of tests, with the exact commands, is at `Audits/Blender Research/Images/Hand tests.md`.

### Observed — the same tests on a second machine, 2026-08-06

Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Blender 4.5.12 LTS at `/opt/blender/blender-4.5.12-linux-x64/blender`, with no window throughout, driven by the same scripts the Mac run used, with only the folder path changed. The whole list is at `Audits/Blender Research/Images/Hand tests — Linux.md`.

*The whole job with no window at all.* The same six frames, the same change to the 3D scene, the same strip built by a script, and then `blender -b -noaudio --factory-startup .../sequence-test.blend -a`. Blender printed `Append frame 1` through `Append frame 6`, the exit status was `0`, and `out_0001-0006.mp4` appeared, reported by `file` as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`. Rendered again to PNG, the frames read:

```
frames       f_0001.png   size=(64, 48) centre=(0.5451, 0.5451, 0.5451, 1.0) corner=(0.2275, 0.2275, 0.2275, 1.0)
png_seq_on   s_0001.png   size=(64, 48) centre=(0.5373, 0.5373, 0.5373, 1.0) corner=(0.2, 0.2, 0.2, 1.0)
png_seq_off  s_0001.png   size=(64, 48) centre=(0.0784, 0.3373, 0.8196, 1.0) corner=(0.0745, 0.3333, 0.8196, 1.0)
```

*A folder of frames with nothing prepared.* The same command wrote `n_0001-0003.mkv`, reported as `Matroska data` — a movie of the 3D scene.

*A frame missing from the set.* All five output frames were written, nothing was printed about the missing one, and the third came out fully transparent black: `s_0003.png size=(64, 48) centre=(0.0, 0.0, 0.0, 0.0) corner=(0.0, 0.0, 0.0, 0.0) bytes=1169`.

*Multi-layer OpenEXR frames offered to the Video Sequencer.* Three files written with no message about the format, `centre=(0.2609, 0.2609, 0.2609, 1.0) corner=(0.0431, 0.0431, 0.0431, 1.0)`.

*How long the result is.* The two movies' own headers record:

```
at24_0001-0024.mp4 bytes=2312   timescale=1000 duration=1000 -> 1.000 s
at12_0001-0024.mp4 bytes=2580   timescale=1000 duration=2000 -> 2.000 s
```

Both machines were run and both agreed on every one of these, to the fourth decimal place, with only the byte sizes of the written files differing by a few bytes.

*Which container and codec will actually take it.* A new test, run on both machines and recorded in full under question 4 in `Preparation/Blender Research/Findings/4.5/Images.md`, encoded three frames for every container with H.264 and for the MPEG-4 container with every codec. Ten of those pairings are refused with `Error: Could not initialize streams, probably unsupported codec combination`, and three more with a complaint about the picture size. So the container and codec menus offer more pairings than this version will write.

### Not covered

- The sources do not say what the resulting movie's playback speed is derived from in this specific case. The manual's Frame Rate description is general — "The number of frames that are displayed per second, relevant for Animation" (https://docs.blender.org/manual/en/4.5/render/output/properties/format.html) — and nothing I found ties it explicitly to an image strip made from existing files.
- The sources do not say whether the 3D scene is still rendered during the second pass when the only thing on the timeline is an image strip. The Sequencer option is described as rendering "the output of the Video Sequence editor, instead of the view from the 3D scene's active camera", and `use_sequencer` is described as processing "the render (and composited) result through the video sequence editor pipeline". Those two wordings do not settle whether any 3D rendering work happens. I did not find a page that says.
- The sources do not say what happens to frames missing from the set when Use Placeholders is *not* enabled. Only the placeholders case is described ("displayed as black").
- The word "short" in the question is not addressed anywhere. I found no stated limit on how long such a movie may be. The only size-related statement I found is about splitting large output files: "If your video is huge and exceeds 2GiB, enable Autosplit Output" (https://docs.blender.org/manual/en/4.5/render/output/properties/output.html).
- I did not check whether Blender can write an animated still-image format (an animated GIF or animated WebP). I searched for it and found nothing — see the record below — but "not found by these searches" is weaker than a source saying so.
- Audio is out of scope of the question and I did not pursue it beyond the one background-mode note below.

### Outside the interface

**Observed — yes, the whole job.** The hand test above did both halves with no window open: the image strip was built from the finished frames by a script in a windowless Blender, and the movie was encoded by `blender -b <file> -a` from a terminal. The written frames prove the pictures came from the strip and not from a fresh 3D render. So this holds when Blender is driven from outside its own window, on that machine. The one thing that does not is the animation player, which the command-line page says is ignored when `-b` is given. (Previously, before the test: partly documented, partly inferred, with the Sequencer pipeline running headless as the named gap. It runs.)

What was documented, from the command line arguments page:

> "-b, --background — Run in background (often used for UI-less rendering)."
> "-a, --render-anim — Render frames from start to end (inclusive)."
> "-F, --render-format <format> — Set the render format. Valid options are: TGA RAWTGA JPEG IRIS AVIRAW AVIJPEG PNG BMP HDR TIFF."
> "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

So a video format can be selected from the command line, and an animation render can be triggered from the command line, without a window. The manual also gives a worked background-mode animation render:

> "blender --background --enable-autoexec my_movie.blend --render-anim"

Source: Blender manual, https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html — read 2026-07-30.

**This paragraph records the reasoning as it stood before the hand test above; the test has since confirmed it, so read it as history, not as an open question.** The step I supplied at the time: the Sequencer stage is a setting on the *scene's render settings* (`use_sequencer`, default on), not a setting on an editor window, and the command line can start an animation render and set the format; therefore the same pipeline stage should run when the render is started with no window. No source I found stated this outright, which is why the label was inferred rather than documented. The hand test above ran exactly this and it worked, so the assumption held.

Documented differences between inside and outside:

- The animation *player* is not available without a window: "-a <options> <file(s)> — Instead of showing Blender's user interface, this runs Blender as an animation player, to view movies and image sequences rendered in Blender (ignored if -b is set)." (same command-line arguments page). So the second reading of the question — showing frames moving — has no equivalent outside the interface.
- "The audio device is disabled in background-mode by default and can be re-enabled by passing in -setaudio Default afterwards." (same page).

**This gap is closed by the hand test above, and is kept here only as a record of what the documentation itself does not cover.** Every step in the manual's Frame Sequence workflow for *building* the strip is written as clicks — "In Blender, open the Video Sequencer", "Choose Add Image from the add menu", "Click the Animation render button". The Python calls that would do the same thing without clicking exist and are documented (`bpy.ops.sequencer.image_strip_add`, `StripsTopLevel.new_image`, `StripElements.append`, `bpy.ops.render.render(animation=True)`), but no source states that they work with no window open, and no source gives a start-to-finish example of assembling an image strip and encoding it from outside the interface. The command line arguments page has no argument that assembles a movie from a folder of images. The hand test above used exactly those Python calls, with no window open, and they worked — so this remains a hole in the documentation, not a hole in what Blender can do.

### Operating system

**Observed on two machines, and the same on both: macOS 26.5.2 on Apple silicon (M1 Pro), and Ubuntu 22.04.5 LTS on an NVIDIA GeForce RTX 4090.** The whole job — building the strip with no window and encoding it from a terminal — works on both, and the frames, the file types and the recorded durations match. That closes the gap this section used to name: a render happens on a rented Linux machine, and the job has now been done there. Both builds have FFmpeg compiled in, and the MPEG-4 container with the H.264 codec wrote a playable file on both. **Windows is still untried** — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

What the sources say, which still stands: video output is a build-time option, not guaranteed everywhere. From the command line arguments page, listing render formats: "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP." (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html). FFMPEG is in that second list.

Documented: which codecs play back where varies. From the supported formats page (https://docs.blender.org/manual/en/4.5/files/media/video_formats.html):

> "Codecs are used to encode and decode the movie, and so must be present on both the encoding machine (Blender) and the target machine."

> "There are dozens, if not hundreds, of codecs, including Xvid, H.264, DivX, Microsoft, and so on. Each has advantages and disadvantages, and compatibility with different players on different operating systems."

Not stated: I found no page that names Windows, macOS or Linux in connection with making a movie out of an image sequence, and no per-operating-system list of which containers or codecs are present. I searched the video formats page, the image formats page, the Output properties page and the Rendering Animations page for "macOS", "Windows" and "Linux" and got no matches in any of them. The manual does say the version of FFmpeg in use can be seen inside Blender — "FFmpeg — The version of the installed FFmpeg components and codecs." (https://docs.blender.org/manual/en/4.5/getting_started/help.html) — which implies it can vary between installations, but that is my reading, not a statement in the source.

### Record of where I looked

Searched, not browsed. All searches were run across the whole of each source folder.

**Manual (`manual_4.5`).**

- Searched all files for: "image sequence", "into a movie", "make a movie", "encode", "FFmpeg", "video file", "movie file", "sequence editor", "Video Sequencer", "playable movie", "compress the frames", "render out", "export video", "GIF", "flipbook", "stop motion", "time-lapse", "playblast", "animated image", "background mode", "macOS", "Windows", "Linux".
- Read in full or in part: `render/output/animation.txt` (the decisive page), `render/output/animation_player.txt`, `render/output/properties/output.txt`, `render/output/properties/post_processing.txt`, `render/output/properties/format.txt`, `render/output/properties/frame_range.txt`, `files/media/video_formats.txt`, `files/media/image_formats.txt`, `video_editing/introduction.txt`, `video_editing/setup/introduction.txt`, `video_editing/edit/montage/strips/image.txt`, `editors/video_sequencer/introduction.txt`, `editors/image/image_settings.txt`, `editors/image/editing.txt`, `compositing/types/input/image.txt`, `advanced/command_line/arguments.txt`, `advanced/scripting/security.txt`, `getting_started/help.txt`.
- Listed the whole of `video_editing/` to check there is no separate "exporting" or "rendering" page for the Video Sequencer. There is not; the rendering instructions live on the Rendering Animations page in the render section.
- Pictures opened: `_images/video-editing_sequencer_strips_introduction_add-menu.png`, `_images/render_output_properties_output_video-encoding-panel.png`. I also listed all `_images` filenames matching "sequencer", "video-editing" and "output" to see what pictures existed before choosing those two.
- Ruled out: "GIF" matched only pages about Rigify (the letters g-i-f sit inside the word "rigify") — false matches, not a format. "flipbook", "stop motion", "playblast" and "animated image" matched nothing anywhere in the manual. "time-lapse" matched only the two motion blur pages, unrelated.

**Python API (`api_4.5`, 1851 files).**

- Searched all files for: "image_strip_add", "new_image", "SequenceEditor", "FFMPEG_VIDEO", "AVI_JPEG", "media_type", "GIF", "flipbook", "stop motion", "time-lapse", "playblast", "background".
- Read: `bpy.ops.sequencer.txt` (the image strip operator), `bpy.ops.render.txt` (all render operators listed; read `render`, `opengl`, `play_rendered_anim`), `bpy.types.ImageStrip.txt`, `bpy.types.StripsTopLevel.txt`, `bpy.types.StripElements.txt`, `bpy.types.RenderSettings.txt` (the `use_sequencer` entry), `bpy.types.ImageFormatSettings.txt`, `bpy.types.FFmpegSettings.txt`, `bpy_types_enum_items/image_type_items.txt`.
- Listed every API page whose name contains "Strip" to make sure I had not missed a strip type that assembles frames some other way. The list is entirely strip kinds (Image, Movie, Sound, Scene, Colour, Text, effects and transitions) plus the animation-editor `NlaStrip` family, which is unrelated.
- Ruled out: "FFMPEG_VIDEO" and "AVI_JPEG" match nothing in the 4.5 API documentation — in 4.5 the movie entry in the output format list is plain `FFMPEG`. "media_type" matches nothing in the 4.5 API documentation either, even though the 4.5 release notes announce it; I note the discrepancy and did not build anything on it.

**Release notes (`release_notes_4.5`, 19 files).**

- Searched all 19 files for: "sequencer", "ffmpeg", "video", "movie", "sequence", "playable movie", "compress the frames", "render out", "export video".
- Read: `sequencer.md` in full, `rendering.md` in full, the "Images & Movies" section of `pipeline_assets_io.md`, the "Video Sequencer" and "Image & Movies" sections of `python_api.md`, and `index.md`.
- Ruled out: most "video" matches across the release notes are links to demonstration video files embedded in the notes themselves (geometry nodes, user interface, animation), not statements about Blender writing video. `bugfixes.md` mentions video-related fixes but none about assembling image sequences into a movie.

**Cycles' own files (`cycles_4.5`).**

- Searched everything for: "ffmpeg", "movie", "video", "sequencer".
- One relevant match: `ui.py` line 1116, quoted above. "video" matched once, in a comment containing a web link inside a camera projection header file — unrelated. "ffmpeg" and "movie" matched nothing.

---

## Things a person with Blender 4.5 open could settle by hand in a few minutes

Items 1, 2, 3, 4, 5, 8 and 9 on this list have been run and their results are in the "Observed" section above. Item 6 is answered there too: this build reports FFmpeg compiled in, and `-F FFMPEG` was accepted. Item 9's answer came from the complete list of output formats this version's setting will take — `BMP, IRIS, PNG, JPEG, JPEG2000, TARGA, TARGA_RAW, CINEON, DPX, OPEN_EXR_MULTILAYER, OPEN_EXR, HDR, TIFF, WEBP, FFMPEG` — which holds no animated single-image format.

Item 7, which containers and codecs the menus actually offer, was run by the Creator on 2026-08-04 and is written in above.

Every one of them was then run again on a second machine — a rented Linux machine, Ubuntu 22.04.5 LTS with an NVIDIA GeForce RTX 4090 — on 2026-08-06, and the results are written in above beside the Mac's. That second list is at `Audits/Blender Research/Images/Hand tests — Linux.md`.

What is still open:

1. **Windows.** Everything observed above was got on macOS on Apple silicon and on Ubuntu on an NVIDIA card, and on both. No Windows machine has run it, so that system stays untried.

---

2026-08-03 — Opus 5 (Claude Code) — Images, Blender 4.5
