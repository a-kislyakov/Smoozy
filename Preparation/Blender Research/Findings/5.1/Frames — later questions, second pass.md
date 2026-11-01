# Frames — later questions, second pass (Blender 5.1)

Sources consulted, all inside `Blender Sources/`, all read on **2026-07-30** (the snapshot date given to me):

- Blender 5.1 manual (`manual_5.1`, including its `_images` folder)
- Blender 5.1 Python API documentation (`api_5.1`)
- Blender 5.1 release notes (`release_notes_5.1`)
- Cycles' own files (`cycles_5.1`)

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 12 was assigned to me. The rest of the sheet is untouched.

---

## 12. When one render writes one video and separate image files over the same frame range, what does an interruption leave if all the images have been written but the video has not finished closing?

### The reading I took, and the other one

The question does not say how a single render comes to write both a video and separate image files, so I had to choose a reading.

**The reading I answer:** the scene's own output is set to a video format, and in the same render one or more compositor **File Output** nodes write image files for the same frames.

I chose it because the manual describes the scene's own output as a single choice between media types, so one render cannot make the scene's own output be both. **That narrowing is inferred, not documented** — see the note under the quote:

> "Media Type
>
> The type of media to save.
>
> Image:
>
> Saves each input as a separate image file. …
>
> Multi-Layer EXR:
>
> Saves all inputs together in a single multi-layer OpenEXR file.
>
> Video:
>
> Encodes each frame into a video container. Encoding options can be found in the Encoding panel."

— Blender manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30.

*(Citation check, 2026-08-09 — Opus 5.* The quote is at that address word for word. What it carries is one setting called Media Type with three named values. Reading "The type of media to save" as *one at a time*, and reading out from that that the scene's own output has no other route to disk, are steps a reader has to supply. The first-pass document for this same version says exactly that and labels the narrowing **inferred**; this page said it flatly. That was a disagreement between the two passes inside one version, and it is now settled the first pass's way: **inferred**, step named. The hand test at **(i)** below is what settles it outright. Changed by the citation check.*)*

**The other reading**, which I do not answer: that the File Output node itself writes the video. The manual lists only two media types for that node ("Image", "Multi-Layer EXR" — same page as the node quote below), but the node's format block is the same kind of settings block used by the scene, and that block does carry a video option: `media_type`, "The type of media to save (default 'IMAGE')", `Literal['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']` (Blender Python API, https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html, read 2026-07-30; the node's `format` property is of that type, https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html, read 2026-07-30). Whether a File Output node can be set to write a video in 5.1 is **not settled** by the sources I have: the manual says two types, the API type allows three, and neither says which wins. I list this as a hand-check at the end.

### Answer

*(Hand tests, 2026-08-09 — Opus 5. The question was put to two running computers on 2026-08-09, and most of what this page calls "not in the sources" is now settled by observation. The observed answer is in its own section below, **Hand tests, 2026-08-09 — observed**. The label below still stands, because it is a statement about the documentation and the documentation has not changed. One thing this page leaves open is now closed: whether a File Output node can be pointed at a video. It cannot, in any of the four versions — see (j).)*

**LABEL: not in the sources** — for the scenario exactly as asked. No page in the 5.1 manual, API documentation, release notes or Cycles files describes a render that writes a video and image files at once, and none describes what a part-finished video file contains or what state it is left in. Below are the four things the sources *do* say that touch the scenario, each with its own label, and then the gap.

**(a) The image files are written one frame at a time, as the render goes — documented.**

> "The File Output node writes one or more images to disk during rendering or compositing. Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence. For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

— Blender manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html, read 2026-07-30.

The same per-frame writing is stated for animation output generally:

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

— Blender manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html, read 2026-07-30.

And the API confirms a per-frame write moment:

> "bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

— Blender Python API, https://docs.blender.org/api/5.1/bpy.app.handlers.html, read 2026-07-30.

**(b) Image files already written stay on disk when a render ends early — inferred.**

The manual says this for the scene's own image-sequence output, not for File Output nodes:

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

— Blender manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html, read 2026-07-30.

The step I took, and a reader has to accept it for the claim to stand: the File Output node is documented as writing its images to disk "during rendering" (quote (a)); the manual says frames written during a render are still in the output directory afterwards (quote above); I carried that from the scene's own image output across to the node's image output. The sources never say it about the node. So this is **inferred**, not documented.

**(c) A video that hits a problem mid-render may be left unusable — documented.**

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

— Blender manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30.

Note the words: **"might"** become unplayable, and **"a problem … while rendering"**. The manual does not say it always happens, does not say it is caused by the file not being closed, and does not say whether an unplayable file still holds the frames that were encoded before the stop. The same page's neighbouring advice repeats the point at the level of the whole workflow:

> "Restart the rendering from the place (the frame) where any problem occurred."

— same page, given as an advantage of rendering stills first.

And:

> "Your computer accidentally turns off in the middle of rendering your movie!
>
> Unless your animation renders in a few minutes, it is best to render the animation as separate image files. Instead of rendering directly to a compressed movie file, use a lossless format (e.g. PNG)."

— Blender manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html, read 2026-07-30.

**(d) There may be more than one video file to account for — documented.**

> "Autosplit Output
>
> If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

— Blender manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30.

So with Autosplit on, an interruption can leave several video files, of which at most the last one is the part-finished one. The sources say nothing about the state of the earlier ones.

**The gap.** Nothing in the 5.1 sources describes a video file being "closed", "finalised" or "completed" at the end of a render, nothing describes what such a file holds if the render stops first, and nothing describes the two kinds of output existing side by side from one render. The manual's only word on the subject is the hedged "might become unplayable" in (c). I will not turn that into a statement about what the file contains, because the sources do not support it.

### Not covered

- Whether the part-finished video file exists at all on disk, whether it is empty, and whether it holds any of the frames encoded before the stop. The sources are silent; "might become unplayable" does not settle it.
- Whether the video is unusable *because* it was not closed, or for some other reason. The manual never mentions a closing step.
- Whether the image files and the video cover the same frames at the moment of the stop, or whether the image for the frame in progress can be written while that frame never reaches the video (or the reverse). The sources describe no ordering between the two writes within a frame.
- Whether the last image file is itself whole. Quote (a) says the node writes during rendering; it does not say a write is all-or-nothing.
- Whether an interruption caused deliberately (a person or a program stopping the render) leaves the same thing as one caused by the machine going down. The manual's wording covers "a problem … while rendering" and "your computer accidentally turns off"; it does not separate the two cases.
- Whether the video and the images agree in content — I only address what is left on disk, not what is in it.
- The File Output node reading only. If the intended scenario was some other way of getting both outputs from one render, this answer may not apply.
- Blender 5.1 only, as it stands in the snapshot taken 2026-07-30.

### Outside the interface

*(Hand tests, 2026-08-09 — Opus 5. This line is now answered by observation. Every run recorded in **Hand tests, 2026-08-09 — observed** below started Blender with `-b --factory-startup` and no window, on the Creator's Mac and on the rented Linux machine, which has no display server and cannot open one. So what is recorded there is the driven-from-outside behaviour for Blender 5.1. The `render_cancel` handler this section declines to read anything into was watched directly: it fires on `SIGINT` and does not fire on `SIGTERM`, and by the time it fires the video file has already reached its full size. What is still not settled is the other direction, whether a stop from inside the window leaves the same thing; that needs eyes on a window and was dropped this round.)*

**The sources do not say.** Nothing in the pages quoted above ties any of it to Blender being driven by a person at the window rather than started from outside. The command-line arguments page documents `-a, --render-anim` ("Render frames from start to end (inclusive).") and `-o, --render-output <path>` without a word about what a stopped run leaves behind (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30). I searched that page for signal handling, `Ctrl-C` and abort behaviour and found only `--disable-abort-handler` ("Disable the abort handler.") and `--disable-crash-handler` ("Disable the crash handler."), neither of which says anything about output files. The only hint that the encoding path is the same either way is a tip on the Output page — "When you view the System Console, you can see some of the output of the encoding process. You will see even more output if you execute Blender as `blender -d`." (https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30) — which tells you where to watch, not what is left. The cancel handler `bpy.app.handlers.render_cancel` ("on canceling a render job") is documented with no mention of the interface, but I will not read a claim about driven-from-outside behaviour out of that.

Because the answer above is mostly "not in the sources", the honest statement is that the sources give me no basis to say the behaviour differs, and no basis to say it is the same.

### Operating system

*(Hand tests, 2026-08-09 — Opus 5. Two of the three are now answered by observation, and separately. Every test below that was run on both machines was run twice — once on the Creator's Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, and once on the rented Linux machine, Ubuntu 22.04.5 LTS on Vast.ai with an NVIDIA GeForce RTX 5060 and no display server. For Blender 5.1 the two agreed on every result. **Windows is untested**, not answered: no Windows machine has run it.)*

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. The 248 runs recorded at item (e) below put one qualification on "the two agreed on every result". **They agree on the shape and differ on the detail.** On both machines a `SIGINT` left the last image whole every time and a `SIGTERM` or `SIGKILL` left it short every time. What differed is what "short" was: on the rented Linux machine every one of the eighty short files, across all four versions, was exactly **0 bytes**, while on the Creator's Mac sixty-nine were 0 bytes and eleven were a whole number of 4096-byte blocks — 4096, 16384, 24576, 40960 or 73728 — all far short of a complete image. **[Recount, 2026-08-09 — this sentence used to say seventy-two and eight, and that was a miscount. Counting the run-by-run figures themselves — this version's ten runs per signal as printed on this page, and the other three versions' in the hand-test record these findings were written from — gives sixty-nine empty and eleven block-sized. A recount of figures already on the record: nothing was re-run, and no new fact is added.]** In Blender 5.1 itself, the Mac's one non-empty short file was 24576 bytes, in the first `SIGKILL` run. Windows is still untested.*)*

**The sources do not say.** I found no operating-system qualifier on any of the pages quoted. The video formats page says "Blender uses FFmpeg to handle video encoding and decoding" with no per-system exceptions, and its only per-system note is about playback rather than writing — "Because codecs are required for both encoding and decoding, they must be available on both the system creating the file and the device playing it." (https://docs.blender.org/manual/en/5.1/files/media/video_formats.html, read 2026-07-30). Nothing there or elsewhere says whether a stopped render leaves the same thing on Windows, macOS and Linux.

### Hand tests, 2026-08-09 — observed

Eleven tests were written for this question and run by hand on **2026-08-09**, the same tests in all four researched versions. The whole list, with the exact commands and the exact Python, is `Audits/Blender Research/Frames/Hand tests — later questions.md`. What follows is what came back for Blender 5.1. It is the same record as in `Preparation/Blender Research/Findings/5.1/Frames — later questions.md`, written here too so that neither page points a reader the wrong way.

**The two machines, in full.** *The rented Linux machine* — rented on Vast.ai and reached over SSH, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, and no display server at all; Blender 5.1.2 at `/opt/blender/5.1/blender`. It was shared with other work while these ran. *The Creator's Mac* — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics; Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`. Neither machine speaks for the other. Every run used `--factory-startup` and a fresh throwaway `BLENDER_USER_RESOURCES` folder.

**A word about the reader.** Neither machine has `ffprobe` or `ffmpeg`, so every video file was read back by loading it into Blender as a movie clip. That reader prints `loaded ok` even for a file holding nothing — a 48-byte file comes back as `loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. So each result records three things separately: whether the file exists, its size in bytes, and the frame count the reader gave.

**What was built, to what.** A blend-file made by script: the scene's own output with `media_type` set to `VIDEO` and then `file_format` set to `FFMPEG`, container MPEG-4, codec H.264, into one empty folder; the compositor given a Render Layers node feeding both a File Output node and the Group Output, the node writing PNGs into a second empty folder; Cycles on the processor. In 5.1 the compositor is reached at `scene.compositing_node_group`, the node's folder at `directory` and `file_name`, and the node writes nothing until an item is added to its `file_output_items` — its format's media type having defaulted to `MULTI_LAYER_IMAGE`. Rendered with `blender -b --factory-startup job.blend -a`, from outside Blender's own window.

*(Citation check, 2026-08-09 — Opus 5. **What this set-up does not say, and a repeater has to supply.** It does not say what is in the scene, how many samples, what resolution, or what frame rate. The citation check repeated these tests using the factory-startup scene as it stands, 128 samples, 640 by 360 for the eight-frame runs and 480 by 270 for the twenty-frame runs, and 24 frames a second — the last three read off this page's own readback lines, the first two guessed. Frames then rendered in about half a second where this run's took three or four, and that difference is what item (e) turns on. Anybody repeating this should write down the sample count. Changed by the citation check.)*

**(a) A complete run: the images arrive one per frame, the video arrives at the very end.** Frame range 1–8. On the rented Linux machine the PNGs appeared one at a time, `vid0001-0008.mp4` appeared at 0.608 s holding **48 bytes** and held 48 bytes throughout, the eighth PNG landed at 25.031 s with the video still at 48 bytes and still held open, and at 25.132 s the log reads `vids=[('vid0001-0008.mp4', 4831)] open=[]`. Readback: `loaded ok | frame_duration: 8 | size: (640, 360) | fps: 24.0`. On the Creator's Mac the same shape, ending at 4624 bytes and `frame_duration: 8`. **The two machines agreed.** This is quote (a)'s claim on this page put to a machine, and it holds: the node writes as the render goes.

**(b) The state the question describes.** A handler on `render_write` printed the state at every frame and killed its own process the moment it fired for the last frame. On the rented Linux machine, `SIGKILL`:

> `KILLATLAST render_write frame: 8 | images: ['ImageImage0001.png', … , 'ImageImage0008.png'] | video: [('vid0001-0008.mp4', 48)]`
> `KILLATLAST sending SIGKILL to self 58185`
> `exit status of blender: 137`

`KILLATLAST render call returned` was not printed, so the stop landed where it was aimed. Left on disk: eight PNGs of 369050 bytes each and a **48-byte** video reading back `frame_duration: 0`. `SIGTERM` gave exit status 143 and the same. On the Creator's Mac, both signals gave the identical result. **The two machines agreed.**

**(c) The last image was whole.** Pixels compared rather than bytes: `PIX RESULT: largest difference on any channel = 0.0` on both machines, the two files differing in size by nine bytes on Linux and eight on the Mac.

**(d) Interrupted partway, three ways.** Frame range 1–20, the signal sent once ten PNGs were on disk. On the rented Linux machine, `SIGINT`: exit code 1, ten images, `[('vid0001-0020.mp4', 3850)]`, Blender printing `Sent an internal break event. Press ^C again to kill Blender`, readback `frame_duration: 10`. `SIGTERM` (exit `-15`) and `SIGKILL` (exit `-9`): ten images and a 48-byte video reading back `frame_duration: 0`. On the Creator's Mac the same three, the `SIGINT` file being 3831 bytes and reading back `frame_duration: 10`. **The two machines agreed.** A second `SIGINT` run on the Linux machine, the one at (g), left a file reading back **9** frames with ten images on disk, so the reader's frame count for a `SIGINT`-stopped file was not the same twice.

**(e) Whether the last image is whole.** In 5.1 on the rented Linux machine all ten images were at the full size after each of the three signals. In Blender 4.5 the same `SIGKILL` run left its tenth image at 0 bytes, and in 5.0 at 155648 bytes against a full 254253 — so a cut-short image file is a thing that happens, and 5.1 did not show it in these runs.

*(Citation check, 2026-08-09 — Opus 5.* The 5.1 half reproduced exactly, on both machines. **The cross-version half is disputed** and is on the hand-test list: the citation check's own 4.5 and 5.0 runs left every image at the full size after every signal on the rented Linux machine, and the one cut-short image it saw was in 4.5 on the Creator's Mac after `SIGTERM`. Changed by the citation check.*)*

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. The test was run, as entry 12 of `Audits/Blender Research/Frames/Hand tests — later questions.md`: **ten runs for each of `SIGINT`, `SIGTERM` and `SIGKILL`, in all four versions, on both machines, 248 runs in all.** All three sets of runs now stand side by side and none is deleted.*

*The set-up numbers the earlier record left out, now written down: frames 1 to 20, 480 by 270, 24 frames a second, Cycles on the processor, four versions run side by side with a fixed thread count each — **200 samples at four threads on the rented Linux machine**, **150 samples at two threads on the Creator's Mac** — giving 4.2 to 4.9 seconds a frame on Linux and 4.9 to 5.5 on the Mac. The watcher sent the signal between 0.000013 and 0.001426 seconds after the tenth image file appeared.*

***For Blender 5.1.*** *On the **rented Linux machine**, a complete image being 254122 bytes: the tenth image was 254122 bytes in all ten `SIGINT` runs and **0 bytes in all ten `SIGTERM` runs and all ten `SIGKILL` runs**. On the **Creator's Mac**, a complete image being 256712 bytes: 256712 in all ten `SIGINT` runs; **0 bytes in all ten `SIGTERM` runs**; and 24576, 0, 0, 0, 0, 0, 0, 0, 0, 0 bytes across the ten `SIGKILL` runs in order.*

***The controls.*** *Ten images on disk and the first nine at the full size in every run, so the stop really bit; exit statuses 1, -15 and -9 throughout; the full size taken from a complete uninterrupted run of the same file on the same machine. The other outcome is real and it happened: `SIGINT` gave ten whole images every time.*

***Beside the two runs above, without picking.*** *On `SIGINT` all three agree. On `SIGTERM` and `SIGKILL` this batch does not match the 5.1 half of the finding above, nor the citation check's repeat of it — both record all ten full in 5.1, and this batch found the tenth image short in every one of its forty `SIGTERM` and `SIGKILL` runs in 5.1 across the two machines. It does support the cross-version half, that a cut-short image file is a thing that happens, and it found it in 5.1 too. Added by the second batch of hand tests.)*

**(f) A second run over the same range**, on the rented Linux machine, from what the `SIGKILL` run left, allowed to finish: the video went from 48 bytes to **6898 bytes** with a new modification time, reading back `frame_duration: 20`; all twenty PNGs written again with new modification times, including the ten already complete.

**(g) Whether a cancel handler fires when the stop comes from outside.** On the rented Linux machine, `SIGINT`:

> `CANCELWATCH render_cancel fired at frame 1`
> `CANCELWATCH copied /root/ht-frames/e10_5.1_SIGINT/vid/vid0001-0020.mp4 3833 -> /root/ht-frames/e10_5.1_SIGINT/copy/vid0001-0020.mp4`
> `RUNSIGNAL copy and final file identical byte for byte: True`

The file was already at its full size when the cancel fired and nothing changed it afterwards. With `SIGTERM` no `render_cancel` line appeared at all and the file left behind was 48 bytes. `render_complete` never appeared in either run.

**(h) The container changes the number of bytes left, not the frame count.** The ten containers 5.1 offers are `['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME']`. The same stop-at-the-last-frame run in each left: MPEG4 48 bytes, MKV 0, WEBM 262, AVI 0, DV no file at all, FLASH 0, MPEG1 0, MPEG2 0, OGG 0, QUICKTIME 36 — every one reading back `frame_duration: 0`. WEBM and DV never reached the interruption, printing `Could not initialize streams, probably unsupported codec combination` and `Render width has to be 720 pixels for DV!`. The control, the same containers run to the end, produced readable files for the other eight.

**(i) The Media Type premise.** On the rented Linux machine: `Q6 media_type present: True`, `is_enum_flag: False`, items `['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`; with it set to `VIDEO`, `Q6 set PNG FAILED: TypeError bpy_struct: item.attr = val: enum "PNG" not found in ('FFMPEG')`, and `is_movie_format: True` against `False` under the image setting.

**(j) Whether a File Output node can be pointed at a video — it cannot.** This is the thing this page called "not settled". On the rented Linux machine:

> `Q7 node format media_type present: True`
> `Q7 node media_type items: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`
> `Q7 node media_type now: MULTI_LAYER_IMAGE`
> `Q7 set node media_type VIDEO FAILED: TypeError bpy_struct: item.attr = val: enum "VIDEO" not found in ('IMAGE', 'MULTI_LAYER_IMAGE')`
> `Q7 set node file_format FFMPEG FAILED: TypeError bpy_struct: item.attr = val: enum "FFMPEG" not found in ('OPEN_EXR_MULTILAYER')`

The underlying type lists three values, and the node will take two of them. 5.0 and 5.2 printed the same; Blender 4.5 has no such property on the node at all. So the second reading of the question — one File Output node writing the video — cannot be built in any of the four.

**(k) Overwrite and Placeholders can be changed with a video output in force**, in all four versions: `Q9 VIDEO file_format: FFMPEG | is_movie_format: True`, `Q9 VIDEO use_overwrite was True now False`, `Q9 VIDEO use_placeholder was False now True`. This is about the settings, not about how the boxes are drawn in a panel.

**(l) Autosplit: the file already split away survives, the one being written does not.** On the rented Linux machine only. Two set-ups failed first and are recorded rather than quietly replaced: container `AVI` with codec `NONE` gave `Error: Could not initialize streams, probably unsupported codec combination` and a 0-byte file, and container `AVI` with codec `HUFFYUV` worked but was far too slow to reach 2 GiB. What was used: container `QUICKTIME`, codec `QTRLE`, 3840 by 2160, `use_autosplit = True`, frame range 1–4000, and the **Workbench** engine rather than Cycles so that a couple of thousand frames could be reached at all — that last change is a departure from the set-up used everywhere else here and is recorded because the engine is not what this test is asking about. The render was watched and killed with `SIGKILL` the moment a second video file appeared. What came back:

> `RUNSPLIT 3793.4 s: a second video file has appeared: ['vid_0000001-4000.mov', 'vid_0010001-4000.mov'] -- sending SIGKILL to pid 139630`
> `RUNSPLIT exit code: -9`
> `RUNSPLIT files left: [('vid_0000001-4000.mov', 2012531971), ('vid_0010001-4000.mov', 36)]`

Readback: the first file `loaded ok | frame_duration: 2197 | size: (3840, 2160) | fps: 24.0`; the second file `loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. The other three versions gave the same two files with the same two names and the same shape of answer. The control that the split really happened is the file count: one file would have meant the condition never bit, and there were two. **The Creator's Mac was not used for this test**, because each run writes over two gigabytes and takes between thirty and sixty-seven minutes; macOS is untested for autosplit and that is a gap rather than an answer.

**What these tests do not settle.** Nothing was run inside Blender's own window, so nothing here says what pressing Esc or clicking the cancel button leaves. Nothing was run on Windows. The reader used was Blender itself. And every run used one scene, one codec and one frame rate.

### Where I looked

All searches were run over the whole of each folder, not over single files.

**Manual (`manual_5.1`, every `.txt`).** Terms searched, case-insensitive: `File Output`, `interrupt`, `cancel`, `abort`, `unfinished`, `incomplete`, `corrupt`, `partial`, `partially written`, `half-written`, `written so far`, `truncate`, `truncated`, `playable`, `unplayable`, `container`, `finalize`, `finalise`, `trailer`, `muxer`, `flush`, `close the file`, `moov`, `damaged`, `broken file`, `power failure`, `crash`, `turns off`, `recover`, `resume`, `Overwrite`, `Esc`, `re-render all frames`, `become unplayable`, `render directly to a video`, `directly to a movie`, `not recommended to render directly`, `stop the render`, `stop rendering`, `signal`, `SIGTERM`, `Ctrl-C`, `render-anim`.

Pages opened in full and what they gave:

- `render/output/properties/output.html` — the Output panel and the Encoding panel. Carries the "might become unplayable" tip, the Autosplit note, the Media Type list, and the `Mux Rate` / "Multiplexing" wording. This is the only page in the whole manual matching `unplayable`, and the only one matching `muxer`. It never mentions closing or finishing a video file.
- `render/output/animation.html` — Direct Approach vs Frame Sequence, and the "your computer accidentally turns off" hint. Speaks about resuming an image-sequence render; says nothing about what a movie file holds after a stop.
- `render/output/introduction.html` — "when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range". About images only.
- `compositing/types/output/file_output.html` — the File Output node. Documents that it writes during rendering; says nothing about renders that stop.
- `render/output/properties/post_processing.html` — Compositing and Sequencer pipeline switches. Nothing on interruption.
- `render/introduction.html`, `files/media/video_formats.html`, `compositing/limitations.html`, `interface/window_system/topbar.html` (Render menu), `advanced/command_line/arguments.html`, `files/file_paths.html` — all checked, none touches the question.

Ruled out and why: `troubleshooting/` (the `corrupt` matches are all graphics-driver and file-import pages, not render output); `editors/video_sequencer/` and `editors/clip/` (the `incomplete`/`damaged` matches are about proxy timecodes on footage being read in, not files being written); `physics/` (`interrupt` and `resume` matches are about simulation caches); `modeling/`, `animation/`, `sculpt_paint/`, `grease_pencil/` (matches on `cancel`, `partial`, `truncate` are all about editing tools and maths nodes).

**Pictures. LABEL: documented**, for what is visible in them. I opened `_images/render_output_properties_output_panel.png` (the Output panel with File Format set to PNG) and `_images/render_output_properties_output_video-encoding-panel.png` (the same panel with File Format set to FFmpeg video and Container set to Matroska, plus Encoding, Video and Audio sub-panels), both from https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30. Neither shows anything about a render that stops part-way. Also visible: in the video picture the Overwrite and Placeholders labels appear dimmed while File Extensions and Cache Result do not — I mention it only because I looked; it does not bear on what an interruption leaves on disk, and I have not built anything on it. I also opened `_images/node-types_CompositorNodeOutputFile.webp` via the node page reference; visible in it is a File Output node with a directory field and a file-name field and one input socket, and nothing about interruption.

*(Citation check, 2026-08-09 — Opus 5.* The Method treats a picture in the manual as a document, to be named with its page, described by what is visible, and labelled. This record carried no label. Added, and the Container row named. Changed by the citation check.*)*

**Python API (`api_5.1`, every `.txt`).** Terms searched: `unplayable`, `interrupt`, `abort`, `incomplete`, `partial file`, `canceling a render`, `movie file`, `video file`, `media_type`, `use_overwrite`, `use_placeholder`, `is_movie_format`, `use_autosplit`, `filepath`. Files read in full or in part: `bpy.app.handlers.txt` (the whole render handler block), `bpy.types.CompositorNodeOutputFile.txt`, `bpy.types.ImageFormatSettings.txt`, `bpy.types.FFmpegSettings.txt`, `bpy.types.RenderSettings.txt`, `bpy.ops.render.txt`. `bpy.ops.render.render` is marked "Undocumented, consider contributing." and its parameters say nothing about stopping. `RenderSettings.use_overwrite` ("Overwrite existing files while rendering") and `use_placeholder` ("Create empty placeholder files while rendering frames (similar to Unix 'touch')") exist but carry no statement about video output or about interruption. Nothing in the API describes a part-finished file of any kind. Ruled out: the `movie file` / `video file` matches are file-browser filter flags and the Image data-block, unrelated.

**Release notes (`release_notes_5.1`, every file).** Searched all files for `file output`, `ffmpeg`, `video`, `encod`, `cancel`, `abort`, `interrupt`. Read `pipeline_io.md` (its "Video & Audio output" section: only a custom CRF quality level and a higher audio bitrate ceiling) and `compositor.md` (File Output node changes: custom meta-data, and directories allowed in item names). Also checked `rendering.md`, `core.md` and `bugfixes.md`. Nothing in the 5.1 notes changes or describes what an interrupted render leaves. Address: https://developer.blender.org/docs/release_notes/5.1/pipeline_io/ and https://developer.blender.org/docs/release_notes/5.1/compositor/, read 2026-07-30.

**Cycles' own files (`cycles_5.1`).** Searched the whole folder for `ffmpeg`, `file_output`, `movie`, `container`, `mux` — no matches at all. Searched the Python files for `cancel`, `abort`, `interrupt`, `write` — the only hits are in `osl.py` (writing compiled shader files) and `operators.py`, at lines 88 and 97, both inside the Denoise Animation operator. The supplied Cycles files contain no video encoding and nothing about the render's output files, so they cannot settle this question. Ruled out.

*(Citation check, 2026-08-09 — Opus 5.* Two corrections against the files. The `operators.py` hits were called "a preset operator"; they are in `CYCLES_OT_denoise_animation`, and the preset operators live in a different file, `presets.py`, which was not among the hits. And "no image-file writing" is too strong — the Denoise Animation operator does build image file paths and write files; what these files hold nothing of is the render's own output writing. Both corrected to the files. Changed by the citation check.*)*

---

## Things a person with Blender 5.1 open could settle by hand

*(Hand tests, 2026-08-09 — Opus 5. Six of the eight items on this list were run on 2026-08-09 and their answers are in **Hand tests, 2026-08-09 — observed** above, so they are struck here and only what is genuinely still open is left. Item 1, does an interrupted video render leave a file and does it hold anything — see (d). Item 2, the same killed rather than cancelled — see (d), where `SIGINT`, `SIGTERM` and `SIGKILL` were each run and did not all leave the same thing. Item 3, the same from the command line — see (d) again; every run in this whole set was from the command line, so there is no in-window run to compare it with. Item 4, do the images survive when the video does not — see (b) and (d): they do. Item 5, is the last image file whole — see (c) and (e), and read the whole of (e), which now carries three sets of runs rather than one. Item 6, can a File Output node be set to write a video at all — see (j): no, in any of the four versions. Items 7 and 8 are set out below.)*

**Answered on Linux, untested on the Mac — Autosplit.** What an interruption leaves when the file has already split into more than one was run on the rented Linux machine and is at (l) above: the file already split away held every frame that went into it, and the one being written was left at 36 bytes holding none. It was not run on the Creator's Mac, because each run writes over two gigabytes and takes between thirty and sixty-seven minutes, so macOS is untested for this one.

**Half answered, half dropped — item 8.** Whether Overwrite and Placeholders can be held and changed with a video output in force was answered from a terminal, at (k) above: in all four versions both can. Whether the boxes are *clickable in the panel* was not answered and is dropped this round, because the rented Linux machine has no display server and cannot open a window at all, and the Creator ran nothing. Reading a property is a different test from looking at a panel.

**Not run — Windows.** The closing line of this list asks for all three operating systems. macOS and Linux were each run and are answered separately above. Windows was not tried: no Windows machine has run it, so it is untested rather than answered. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

The researcher's original list follows, kept as the record of what was thought open at the time.

1. **Does an interrupted video render leave a file, and does it hold anything?**
   Open Blender. In Properties ▸ Output, set the file path to an empty folder, set Media Type to Video, set Container to Matroska and Video Codec to H.264. Set the frame range to 1–200 on a scene slow enough that each frame takes a few seconds. Press Ctrl-F12 to render the animation. After roughly 30 frames have rendered, press Esc. Then look in the output folder: note whether a file is there, its size in bytes, and whether it opens in a video player and, if it does, how many frames it plays.

2. **The same, but killed rather than cancelled.**
   Repeat step 1, but instead of pressing Esc, end the Blender process from the operating system's task manager or activity monitor. Compare the file's presence, size and playability with what step 1 gave.

3. **The same, run from the command line.**
   Repeat steps 1 and 2 with Blender started as `blender --background <file>.blend --render-anim`, interrupting once with Ctrl-C in the terminal and once by killing the process. Compare against the results from inside the window. This is the case the documents are most silent about.

4. **Do the images survive when the video does not?**
   Take the same file. In the compositor, add a File Output node fed from the Render Layers node, and point it at a second empty folder. Leave the scene output as the video from step 1. Render the animation and interrupt it as in step 1. Then count the image files in the second folder, note the highest frame number reached, and compare it with how many frames the video plays. This is the exact scenario of question 12.

5. **Is the last image file whole?**
   After step 4, open the highest-numbered image file in an image viewer and check whether it displays fully, and compare its size in bytes against the file before it.

6. **Can a File Output node be set to write a video at all in 5.1?**
   Add a File Output node in the compositor, open the sidebar at Node ▸ Properties ▸ Node Format, and look at what the Media Type list offers. Record whether "Video" appears there alongside "Image" and "Multi-Layer EXR". The manual lists only two; the underlying setting allows three.

7. **What does Autosplit leave?**
   Repeat step 1 with Autosplit Output enabled and a render long enough to pass 2 GiB, then interrupt after the split. Record how many files exist and which of them play.

8. **Are Overwrite and Placeholders available for a video output?**
   In Properties ▸ Output, switch Media Type between Image and Video and note whether the Overwrite and Placeholders checkboxes can be clicked in each case. This settles what a person can do about the leftovers, which the manual's picture only hints at.

Run each of these on all three operating systems if the answer is going to be relied on, since nothing in the documents says the behaviour is the same across them.

---

2026-08-09 — Opus 5 (Claude Code) — Frames, Blender 5.1
