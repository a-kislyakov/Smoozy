# Frames — later questions (Blender 5.2)

**Sources consulted, and when.** Blender 5.2 manual, Blender 5.2 Python API documentation, Blender 5.2 release notes, and the Cycles 5.2 add-on files — all as supplied in `Blender Sources/`, snapshot taken 2026-07-30. That snapshot date is my date read for everything cited below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One thing to declare honestly against that line: to find out whether my output file already existed, I listed the file names in the folder I was told to write to (`Preparation/Blender Research/Findings/5.2/`). I saw file names only and opened none of them. No content from this project reached me.

I did not use the web. I started no helper agents, so there is nobody else to vouch for.

---

## 12. When one render writes one video and separate image files over the same frame range, what does an interruption leave if all the images have been written but the video has not finished closing?

**Reading taken, because the question is ambiguous.** I answer it as: one render job whose scene output is set to write a video, and whose compositor also has File Output nodes writing images, both covering the same frames. That is the only arrangement I found described in the 5.2 manual for one render producing a video and separate image files at once. The File Output node's own Media Type lists two choices, and neither of them is video:

> "Media Type
>
> Determines how multiple inputs are stored:
>
> Image:
>
> Saves each input as a separate image file.
> Each input can use its own image format settings.
>
> Multi-Layer EXR:
>
> Saves all inputs together in a single multi-layer OpenEXR file."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html, read 2026-07-30

whereas the scene's own Output panel does offer video:

> "Video:
>
> Encodes each frame into a video container.
> Encoding options can be found in the Encoding panel."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html, read 2026-07-30

The other readings I did not take: two separate renders over the same range, or the video coming from the Video Sequencer in a second pass. If either of those is what was meant, this answer does not address it.

*(Hand tests, 2026-08-09 — Opus 5. The question was put to two running computers on 2026-08-09, and most of what this page calls "not in the sources" is now settled by observation. The observed answer is in its own section below, **Hand tests, 2026-08-09 — observed**. The label below still stands, because it is a statement about the documentation and the documentation has not changed.)*

**LABEL: not in the sources.**

The sources nowhere describe what an interrupted render leaves behind when a video and separate image files were being written together, and nowhere describe closing or finishing a video file as a step at all. There is no statement about the state of the video file at the moment a render stops, and no statement about the image files that a File Output node wrote during that same render. So the exact case the question asks about is unanswered by the sources.

What the sources do say, which touches the edges of it:

**On the video.** The manual warns that a video written directly by a render can end up unusable if the render hits trouble, and that the remedy is to re-render everything:

> "It's not recommended to render directly to a video format in the first instance.
> If a problem occurs while rendering, the file might become unplayable and you will
> have to re-render all frames from the beginning."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html, read 2026-07-30

This is the closest the sources come. Note what it does and does not say. It says the file "might" become unplayable — not that it always does, not what "a problem" covers, and not whether the file is left on disk at all. It is written as a recommendation, not as a description of what happens on interruption.

The same warning appears again in the animation chapter, in the same shape — a hint about the computer going down mid-render, with re-rendering as the remedy:

> "Your computer accidentally turns off in the middle of rendering your movie!
>
> Unless your animation renders in a few minutes,
> it is best to render the animation as separate image files.
> Instead of rendering directly to a compressed movie file, use a lossless format (e.g. PNG)."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html, read 2026-07-30

**On the images.** The File Output node is documented as writing while the render runs, one file per frame:

> "The File Output node writes one or more images to disk during rendering or compositing.
> Each connected input is saved to the specified directory and filename,
> with the frame number appended to create a sequence.
> For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html, read 2026-07-30

and as writing alongside the main render output rather than instead of it:

> "Use the File Output node to save raw passes or intermediate steps without overwriting the main render result."

— Manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html, read 2026-07-30

The manual does say that frames already written to disk survive a render that stops — but it says it about frames written by the Output panel in the frame-sequence approach, not about files written by a File Output node:

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering,
> since the frames you have already rendered will still be in the output directory."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html, read 2026-07-30

> "For example, when rendering animations to image file formats the render job can be canceled
> and resumed at the last rendered frame by changing the frame range."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/introduction.html, read 2026-07-30

Taking those two together with the File Output node writing "during rendering" would give the claim that the images stay on disk — but that is a step the sources do not take, so it would be **inferred**, and I am not asserting it as the answer. I flag the step rather than hide it: the sources state that image files already written by the Output panel survive an interruption, and state separately that the File Output node writes images to disk during rendering; they never state that File Output images survive an interruption.

**On "finished closing".** The sources contain no notion of a video file being closed, finalised, or muxed. I searched the whole manual and the whole Python API documentation for "finaliz", "mux", "trailer", "corrupt", "unplayable", "truncated", "incomplete", "partial", "close" in this sense — the only hit that bears on video at all is the "might become unplayable" line quoted above. The API documents the render handlers one line each. The two that bear on a render stopping and on a frame being written say nothing about any output file:

> "bpy.app.handlers.render_cancel
>
> on canceling a render job. Accepts one argument: the scene data-block being rendered"

> "bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

— Python API, https://docs.blender.org/api/5.2/bpy.app.handlers.html, read 2026-07-30

**NOT COVERED.** This answer does not settle: whether the partial video file exists on disk after the interruption; whether it plays at all, plays up to the last frame encoded, or is unreadable; whether Blender removes or keeps it; whether the images written by the File Output node are intact and complete; whether the last image written before the interruption is whole; whether the setting the manual calls Autosplit Output changes any of this; whether it differs between containers or codecs; and whether the way the render was interrupted — a key press, the process being killed, the machine losing power — makes any difference. The sources address none of these.

**Outside the interface.** *(Hand tests, 2026-08-09 — Opus 5. This line is now answered by observation. Every run recorded in **Hand tests, 2026-08-09 — observed** below started Blender with `-b --factory-startup` and no window, on the Creator's Mac and on the rented Linux machine, which has no display server and cannot open one. So what is recorded there is the driven-from-outside behaviour for Blender 5.2: images arriving one per frame, a video file that holds 48 bytes until the end, a `SIGINT` that closes the file and fires `render_cancel`, and a `SIGTERM` or `SIGKILL` that does neither. What is still not settled is the other direction — whether a stop from inside the window, the `Esc` this section quotes, leaves the same thing. That needs eyes on a window and was dropped this round.)*

The sources do not say. The two warnings quoted above sit in the Output and Rendering Animations chapters and are not qualified by how the render was started, and the command-line page documents rendering an animation from outside the window without adding anything about interruption:

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30

Nothing in the sources describes what stopping such a run does to a video file, and nothing describes an interruption arriving from outside Blender at all. Every way of cancelling the manual does describe is a person at the window. One is a key press — "As with a normal render, you can abort it with Esc." (Manual, https://docs.blender.org/manual/en/5.2/editors/3dview/viewport_render.html, read 2026-07-30), and that sentence is about a viewport render. Another is a click — "The task can be aborted by clicking the cancel button ()", said of the status bar's progress display for "the currently running task (such as rendering or baking)" (Manual, https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html, read 2026-07-30). Whether an interruption from outside behaves the same as either is not stated.

*(Citation check, 2026-08-09 — Opus 5.* This read "The only cancelling the manual describes is a person pressing a key", resting on one sentence from the Viewport Render page. Two things were wrong with it. That sentence is about a viewport render, not an animation render, and the 5.2 manual carries a second in-window way to stop a running render — the status bar's cancel button — which this page did not record and the 4.5 document did. "The only" is not carried; both are now quoted. Changed by the citation check.*)*

**Operating system.** *(Hand tests, 2026-08-09 — Opus 5. Two of the three are now answered by observation, and separately. Every test below that was run on both machines was run twice — once on the Creator's Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, and once on the rented Linux machine, Ubuntu 22.04.5 LTS on Vast.ai with an NVIDIA GeForce RTX 5060 and no display server. For Blender 5.2 the two agreed on every result. **Windows is untested**, not answered: no Windows machine has run it.)*

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. The 248 runs recorded at item (e) put one qualification on "the two agreed on every result". **They agree on the shape and differ on the detail.** On both machines a `SIGINT` left the last image whole every time and a `SIGTERM` or `SIGKILL` left it short every time. What differed is what "short" was: on the rented Linux machine every one of the eighty short files, across all four versions, was exactly **0 bytes**, while on the Creator's Mac sixty-nine were 0 bytes and eleven were a whole number of 4096-byte blocks — 4096, 16384, 24576, 40960 or 73728 — all far short of a complete image. **[Recount, 2026-08-09 — this sentence used to say seventy-two and eight, and that was a miscount. Counting the run-by-run figures themselves — this version's ten runs per signal as printed on this page, and the other three versions' in the hand-test record these findings were written from — gives sixty-nine empty and eleven block-sized. A recount of figures already on the record: nothing was re-run, and no new fact is added.]** In Blender 5.2 itself, the Mac's three non-empty short files were 73728, 24576 and 24576 bytes, all in `SIGTERM` runs. Windows is still untested.*)*

The sources do not say the behaviour differs by operating system, and say nothing about the behaviour at all, so there is nothing to differ. The one operating-system-flavoured fact I found nearby is that the video output format is not guaranteed to be present in every build:

> "Formats that can be compiled into Blender, not available on all systems:
> OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30

and that codecs must be present on the machine doing the encoding:

> "Because codecs are required for both encoding and decoding,
> they must be available on both the system creating the file and the device playing it."

— Manual, https://docs.blender.org/manual/en/5.2/files/media/video_formats.html, read 2026-07-30

Neither says anything about interruption.

### Hand tests, 2026-08-09 — observed

Eleven tests were written for this question and run by hand on **2026-08-09**, the same tests in all four researched versions. The whole list, with the exact commands and the exact Python, is `Audits/Blender Research/Frames/Hand tests — later questions.md`. What follows is what came back for Blender 5.2.

**The two machines, in full.** *The rented Linux machine* — rented on Vast.ai and reached over SSH, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, and no display server at all; Blender 5.2.0 at `/opt/blender/5.2/blender`. It was shared with other work while these ran. *The Creator's Mac* — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics; Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`. Neither machine speaks for the other, so each result says which one it came from. Every run used `--factory-startup` and a fresh throwaway `BLENDER_USER_RESOURCES` folder.

**A word about the reader.** Neither machine has `ffprobe` or `ffmpeg`, so every video file was read back by loading it into Blender as a movie clip. That reader prints `loaded ok` even for a file holding nothing — a 48-byte file comes back as `loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. So each result records three things separately: whether the file exists, its size in bytes, and the frame count the reader gave.

**What was built, to what.** A blend-file made by script: the scene's own output with `media_type` set to `VIDEO` and then `file_format` set to `FFMPEG`, container MPEG-4, codec H.264, writing into one empty folder; the compositor given a Render Layers node feeding both a File Output node and the Group Output, the File Output node writing PNGs into a second empty folder; Cycles on the processor. Three things about driving 5.2 came out of building it, and they are facts about the machinery rather than answers to this question: the scene's compositor is reached at `scene.compositing_node_group` rather than `scene.node_tree`; the File Output node has `directory` and `file_name` rather than `base_path`; and the node writes nothing at all until an item is added to its `file_output_items`, its format's media type having defaulted to `MULTI_LAYER_IMAGE`. Rendered with `blender -b --factory-startup job.blend -a`, from outside Blender's own window.

*(Citation check, 2026-08-09 — Opus 5. **What this set-up does not say, and a repeater has to supply.** It does not say what is in the scene, how many samples, what resolution, or what frame rate. The citation check repeated these tests using the factory-startup scene as it stands, 128 samples, 640 by 360 for the eight-frame runs and 480 by 270 for the twenty-frame runs, and 24 frames a second — the last three read off this page's own readback lines, the first two guessed. Frames then rendered in about half a second where this run's took three or four, and that difference is what item (e) turns on. Anybody repeating this should write down the sample count. Changed by the citation check.)*

**(a) A complete run: the images arrive one per frame, the video arrives at the very end.** Frame range 1–8, watched ten times a second on Linux and four times a second on the Mac. On the rented Linux machine the PNG files appeared one at a time, `vid0001-0008.mp4` appeared at 0.707 s holding **48 bytes** and held 48 bytes throughout, the eighth PNG landed at 27.965 s with the video still at 48 bytes and still held open, and at 28.066 s the log reads:

> ` 28.066  imgs=[eight files] vids=[('vid0001-0008.mp4', 4583)] open=[]`

Readback of the finished file: `READVID loaded ok | frame_duration: 8 | size: (640, 360) | fps: 24.0`. On the Creator's Mac the same shape, ending at 4608 bytes and `frame_duration: 8`. **The two machines agreed.**

**(b) The state the question describes.** This is the researcher's own test 6, the one the audit added because the rest of the list interrupted halfway instead. A handler on `bpy.app.handlers.render_write` printed the state at every frame and killed its own process the moment it fired for the last frame. On the rented Linux machine, with `SIGKILL`:

> `KILLATLAST render_write frame: 8 | images: ['ImageImage0001.png', 'ImageImage0002.png', 'ImageImage0003.png', 'ImageImage0004.png', 'ImageImage0005.png', 'ImageImage0006.png', 'ImageImage0007.png', 'ImageImage0008.png'] | video: [('vid0001-0008.mp4', 48)]`
> `KILLATLAST sending SIGKILL to self 62206`
> `exit status of blender: 137`

The line `KILLATLAST render call returned` was not printed, so the stop landed where it was aimed. Left on disk: eight PNG files of 367544 bytes each, and `vid0001-0008.mp4` of **48 bytes**, reading back `frame_duration: 0 | size: (256, 256) | fps: 0.0`. `SIGTERM` gave exit status 143 and the identical result. On the Creator's Mac, both signals gave the identical result, the PNGs there being 368005 bytes each. **The two machines agreed.** So the state the question describes can be reached, and what it leaves is a full set of images beside a 48-byte video that holds no frames.

**(c) The last image was whole.** Pixels compared rather than bytes, between the eighth PNG of the stopped run and the eighth PNG of the complete run: `PIX RESULT: largest difference on any channel = 0.0` on both machines, the files differing in size by nine bytes on Linux (367544 against 367535) and eight on the Mac.

**(d) Interrupted partway, three ways.** Frame range 1–20, the signal sent once ten PNGs were on disk. On the rented Linux machine, `SIGINT` gave `RUNSIGNAL exit code: 1`, ten images and `[('vid0001-0020.mp4', 3967)]`, with Blender printing `Sent an internal break event. Press ^C again to kill Blender` and then `00:14.104  render           | Fra: 11 | Remaining: 00:03.07 | Mem: 8M | Cancelled`; readback `frame_duration: 10`. `SIGTERM` gave exit code `-15` and `SIGKILL` gave `-9`, both leaving ten images and a 48-byte video reading back `frame_duration: 0`. On the Creator's Mac the same three, the `SIGINT` file being 3862 bytes and reading back `frame_duration: 10`; that run's last lines were `Saved: '…/ImageImage0010.png'`, `Video append frame 10`, `Time: 00:01.83 (Saving: 00:00.00)` and then `Sent an internal break event`. **The two machines agreed.** A second `SIGINT` run on the Linux machine, the one at (g), left a file reading back **9** frames with ten images on disk, so the reader's frame count for a `SIGINT`-stopped file was not the same twice.

**(e) A file being on disk is not the same as the frame having finished.** In Blender 5.2 on the rented Linux machine all ten images were at the full size after each of the three signals. In Blender 4.5 the same `SIGKILL` run left its tenth image at 0 bytes, and in Blender 5.0 at 155648 bytes against a full 254253 — so a cut-short image file is a thing that happens, and 5.2 did not show it in these runs.

*(Citation check, 2026-08-09 — Opus 5.* The 5.2 half reproduced exactly: all ten at 95128 bytes on the rented Linux machine after each signal, and 95252 bytes on the Creator's Mac. **The cross-version half is disputed** and is on the hand-test list: the citation check's own 4.5 and 5.0 runs on the rented Linux machine left every image at the full size after every signal, and the one cut-short image it saw was in 4.5 on the Creator's Mac after `SIGTERM`. Changed by the citation check.*)*

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. The test was run, as entry 12 of `Audits/Blender Research/Frames/Hand tests — later questions.md`: **ten runs for each of `SIGINT`, `SIGTERM` and `SIGKILL`, in all four versions, on both machines, 248 runs in all.** All three sets of runs now stand side by side and none is deleted.*

*The set-up numbers the earlier record left out, now written down: frames 1 to 20, 480 by 270, 24 frames a second, Cycles on the processor, four versions run side by side with a fixed thread count each — **200 samples at four threads on the rented Linux machine**, **150 samples at two threads on the Creator's Mac** — giving 4.2 to 4.9 seconds a frame on Linux and 4.9 to 5.5 on the Mac. The watcher polled every millisecond and sent the signal between 0.000013 and 0.001426 seconds after the tenth image file appeared.*

***What came back for Blender 5.2.*** *On the **rented Linux machine**, where a complete image in the control run was 253330 bytes: the tenth image, `ImageImage0010.png`, was 253330 bytes in all ten `SIGINT` runs and **0 bytes in all ten `SIGTERM` runs and all ten `SIGKILL` runs**. On the **Creator's Mac**, where a complete image was 254935 bytes: 254935 bytes in all ten `SIGINT` runs; short in all ten `SIGTERM` runs, at 73728, 0, 0, 0, 24576, 24576, 0, 0, 0 and 0 bytes in run order; and **0 bytes in all ten `SIGKILL` runs**.*

***The controls.*** *In every one of the sixty runs the first nine images were at the full size and exactly ten images were on disk, so the render was stopped rather than finished; exit statuses were 1, -15 and -9 for the three signals throughout. The full size comes from a complete uninterrupted run of the same file on the same machine. And the other outcome is a real one, because it happened: `SIGINT` gave ten runs with every image whole.*

***How this sits beside the two runs above, without picking between them.*** *On `SIGINT` all three agree. On `SIGTERM` and `SIGKILL` this batch does **not** match the 5.2 half of the finding above, nor the citation check's repeat of it: both record all ten images at the full size after every signal in 5.2, and this batch found the tenth image short in every one of its forty `SIGTERM` and `SIGKILL` runs in 5.2 across the two machines. It does support the finding's cross-version half — that a cut-short image file is a thing that happens — and it found it in 5.2 as well, which the earlier runs did not. Nothing here says which earlier run was right; the earlier runs were one run per signal, this one was ten, and this one states a condition theirs do not — how quickly the signal followed the file appearing. Added by the second batch of hand tests.)*

**(f) A second run over the same range**, on the rented Linux machine, starting from what the `SIGKILL` run left and allowed to finish: the video went from 48 bytes to **6799 bytes** with a new modification time, reading back `frame_duration: 20`, and all twenty PNGs were written again at 253339 bytes with new modification times, including the ten the first run had already completed.

**(g) Whether a cancel handler fires when the stop comes from outside.** On the rented Linux machine, with handlers on `render_cancel`, `render_complete` and `render_write`, and `SIGINT` sent once ten images were written:

> `CANCELWATCH render_write frame 9 | video sizes: [('vid0001-0020.mp4', 48)]`
> `CANCELWATCH render_cancel fired at frame 1`
> `CANCELWATCH copied /root/ht-frames/e10_5.2_SIGINT/vid/vid0001-0020.mp4 3788 -> /root/ht-frames/e10_5.2_SIGINT/copy/vid0001-0020.mp4`
> `CANCELWATCH render call returned`
> `RUNSIGNAL copy and final file identical byte for byte: True`

The file was already at its full 3788 bytes when the cancel handler fired, and nothing changed it afterwards; readback `frame_duration: 9`. With `SIGTERM`, no `render_cancel` line appeared at all, no copy was taken, and the file left behind was 48 bytes reading back `frame_duration: 0`. `render_complete` never appeared in either run.

**(h) The container changes the number of bytes left, not the frame count.** On the rented Linux machine the containers 5.2 offers are `['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME']` — the same ten as in the other three versions, and the codec list is the same seventeen. The same stop-at-the-last-frame run in each left: MPEG4 48 bytes, MKV 0, WEBM 264, AVI 0, DV no file at all, FLASH 0, MPEG1 0, MPEG2 0, OGG 0, QUICKTIME 36. Every one read back `frame_duration: 0`. WEBM and DV never reached the interruption and said so, printing `RuntimeError: Error: Could not initialize streams, probably unsupported codec combination` and `RuntimeError: Error: Render width has to be 720 pixels for DV!`. The control — the same containers rendered to the end — produced readable files for the other eight (MPEG4 2270 bytes / 4 frames, MKV 2064 / 4, AVI 7258 / 4, FLASH 3630 / 4, MPEG1 4096 / 2, MPEG2 6144 / 2, OGG 5439 / 4, QUICKTIME 2204 / 4). That is the researcher's own test 5.

**(i) The premise about one output at a time.** On the rented Linux machine:

> `Q6 media_type present: True`
> `Q6 media_type is_enum_flag: False | items: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`
> `Q6 file_format is_enum_flag: False`
> `Q6 set FFMPEG ok, file_format now: FFMPEG`
> `Q6 set PNG FAILED: TypeError bpy_struct: item.attr = val: enum "PNG" not found in ('FFMPEG')`
> `Q6 is_movie_format: True`

So the media type holds one value at a time, and with it set to `VIDEO` the only file format the render output will accept is `FFMPEG`. One caution about this record: the two format lists the script printed under `IMAGE` and under `VIDEO` came back identical, because that listing is the property's whole declared list rather than what is on offer at the moment; what shows the live state is the failed assignment above and `is_movie_format` reading `False` under the image setting and `True` under the video setting in the same run.

**(j) A File Output node cannot be set to write a video.** This is the researcher's own test 6 in the second-pass document, which asked for `node.format.media_type = 'VIDEO'` to be tried. On the rented Linux machine:

> `Q7 node format media_type present: True`
> `Q7 node media_type items: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`
> `Q7 node media_type now: MULTI_LAYER_IMAGE`
> `Q7 set node media_type VIDEO FAILED: TypeError bpy_struct: item.attr = val: enum "VIDEO" not found in ('IMAGE', 'MULTI_LAYER_IMAGE')`
> `Q7 set node file_format FFMPEG FAILED: TypeError bpy_struct: item.attr = val: enum "FFMPEG" not found in ('OPEN_EXR_MULTILAYER')`

5.0 and 5.1 printed the same; Blender 4.5 has no such property on the node at all. So it is not accepted, in any of the four versions.

**(k) Overwrite and Placeholders can be changed with a video output in force.** On the rented Linux machine, all four versions printed the same six lines; for 5.2:

> `Q9 IMAGE file_format: PNG | is_movie_format: False`
> `Q9 IMAGE use_overwrite was True now False`
> `Q9 IMAGE use_placeholder was False now True`
> `Q9 VIDEO file_format: FFMPEG | is_movie_format: True`
> `Q9 VIDEO use_overwrite was True now False`
> `Q9 VIDEO use_placeholder was False now True`

This is about the settings, not about how the boxes are drawn in a panel.

**(l) Autosplit: the file already split away survives, the one being written does not.** On the rented Linux machine only. Two set-ups failed first and are recorded rather than quietly replaced: container `AVI` with codec `NONE` gave `Error: Could not initialize streams, probably unsupported codec combination` and a 0-byte file, and container `AVI` with codec `HUFFYUV` worked but was far too slow to reach 2 GiB. What was used: container `QUICKTIME`, codec `QTRLE`, 3840 by 2160, `use_autosplit = True`, frame range 1–4000, and the **Workbench** engine rather than Cycles so that a couple of thousand frames could be reached at all — that last change is a departure from the set-up used everywhere else here and is recorded because the engine is not what this test is asking about. The render was watched and killed with `SIGKILL` the moment a second video file appeared. What came back:

> `RUNSPLIT 4038.7 s: a second video file has appeared: ['vid_0000001-4000.mov', 'vid_0010001-4000.mov'] -- sending SIGKILL to pid 139780`
> `RUNSPLIT exit code: -9`
> `RUNSPLIT files left: [('vid_0000001-4000.mov', 2012531972), ('vid_0010001-4000.mov', 36)]`

Readback: the first file `loaded ok | frame_duration: 2197 | size: (3840, 2160) | fps: 24.0`; the second file `loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. The other three versions gave the same two files with the same two names and the same shape of answer. The control that the split really happened is the file count: one file would have meant the condition never bit, and there were two. **The Creator's Mac was not used for this test**, because each run writes over two gigabytes and takes between thirty and sixty-seven minutes; macOS is untested for autosplit and that is a gap rather than an answer.

**What these tests do not settle.** Nothing was run inside Blender's own window, so nothing here says what pressing Esc or clicking the status bar's cancel button leaves. Nothing was run on Windows, because no Windows machine has run it. The video reader used was Blender itself, so a frame count is what Blender's movie-clip reader made of the file rather than what every player would make of it. And every run used one scene, one codec and one frame rate. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Where I looked, so that "not in the sources" can be checked

- **Manual, whole-tree searches** (case-insensitive) for: `ffmpeg`, `mux`, `finaliz`, `trailer`, `corrupt`, `unplayable`, `not playable`, `truncated`, `partial file`, `incomplete file`, `interrupt`, `cancel`, `abort`, `stopped`, `crash`, `resume`, `power outage`, `power supply`, `power failure`, `turns off`, `re-render`, `restart the render`, `container`, `movie file`, `video file`. The only pages that came back with anything bearing on the question were the four cited above: the Output panel properties page, Rendering Animations, the Output introduction, and the File Output node page. Supported Video & Audio Formats was read in full and describes containers and codecs only — it never mentions writing being cut short.
- **Manual pages read in full**: `render/output/animation`, `render/output/introduction`, `render/output/properties/output`, `compositing/types/output/file_output`, `files/media/video_formats`, and the render-related parts of `advanced/command_line/arguments`. I also listed `compositing/types/output/` in full to be sure no other output node exists that writes video.
- **Python API, whole-tree searches** for `unplayable`, `finaliz`, `interrupt`, `render_cancel`, `render_complete`, `render_write`, `media_type`, `is_movie_format`, `movie`. `bpy.app.handlers` documents the render handlers, one line each, with no mention of output files. `bpy.types.ImageFormatSettings.media_type` lists the three media types and nothing more. `bpy.types.RenderSettings.is_movie_format` is a read-only flag with a one-line description. Nothing in the API documentation describes what an interrupted render leaves on disk.
- **Release notes for 5.2**, all subject files searched for `ffmpeg`, `video`, `movie`, `encod`, `mux`, `cancel`, `interrupt`, `abort`, `stop render`, `render job`, `file output`, `media type`, `output`. The Rendering notes record only "Saving render output can now be disabled." and light/shadow linking; the Compositor notes record a File Extension checkbox on the File Output node. Nothing about interruption or about finishing a video file.
- **Cycles' own files** searched for `ffmpeg`, `mux`, `movie`, `video file`, `container` — no matches anywhere in the add-on. Searched for `cancel`, `abort`, `interrupt` — the only relevant hit is a comment in `operators.py` line 97, `# TODO: support cancel and progress reports.`, which is about an operator in the add-on and not about writing output. Cycles does not touch video output in these files. Label for that: **observed**, from `Blender Sources/cycles_5.2`.
- **Pictures**: I did not need any. The Output panel screenshot (`render_output_properties_output_panel.png`) and the encoding panel screenshot (`render_output_properties_output_video-encoding-panel.png`) are referenced on the Output page, but they show settings, and the question is about behaviour when a render stops, which a screenshot cannot show.

### Things a person at a computer with Blender open could settle in a few minutes

*(Hand tests, 2026-08-09 — Opus 5. All six items on this list were run on 2026-08-09 and their answers are in **Hand tests, 2026-08-09 — observed** above, so they are struck here and only what is genuinely still open is left. Item 1, does the partial video survive and does it play — see (d). Item 2, the same stopped from outside with Ctrl-C and with `kill -9` — see (d), where `SIGINT`, `SIGTERM` and `SIGKILL` were each run and did not all leave the same thing. Item 3, is the last image whole — see (c) and (e), and read the whole of (e), which now carries three sets of runs rather than one. Item 4, does the frame count match — see (d): ten images beside a file the reader called ten frames in one run and nine in another. Item 5, does the container matter — see (h), where all ten containers were run. Item 6, can the state the question describes be reached at all — see (b): it can, and it was reached by a handler stopping the process at the moment the last frame was written rather than by anyone pressing a key.)*

**Answered on Linux, untested on the Mac — Autosplit.** What an interruption leaves when the file has already split into more than one was run on the rented Linux machine and is at (l) above: the file already split away held every frame that went into it, and the one being written was left at 36 bytes holding none. It was not run on the Creator's Mac, because each run writes over two gigabytes and takes between thirty and sixty-seven minutes, so macOS is untested for this one.

**Still open — anything needing eyes on a window.** Items 1 and 6 were written as somebody pressing Esc. What was run instead was a signal sent from outside, because the rented Linux machine has no display server and cannot open a window at all, and the Creator ran nothing this round. So it is not known whether a stop from inside the window leaves the same thing as a signal from outside. The same reason covers how the Overwrite and Placeholders boxes are drawn with a video output; whether those settings can be held and changed was answered from a terminal instead, at (k).

**Still open — Windows.** Untested rather than answered: no Windows machine has run it. macOS and Linux are each answered on their own above.

The researcher's original list follows, kept as the record of what was thought open at the time.

1. **Does the partial video survive, and does it play?** Open a simple scene. In Output properties set Media Type to Video, pick any container and codec the encoding panel offers and write down which, and set a file path you can find. Set the frame range to 1–100. In the compositor, enable Use Nodes, add a File Output node, connect the Render Layers image to it, and point it at a second folder. Start the animation render. Around frame 50, press Esc. Then look at both folders. Record: how many image files the File Output node left, whether the video file exists, its size, and whether a video player opens it and how many frames it plays. *A result the other way:* the File Output folder is empty, or there is no video file at all, or the video opens and plays every frame up to the interruption. Each of those is a different answer from a part-written video sitting beside a full set of images.
2. **Same test, but stopped from outside.** Run the same file with `blender -b file.blend -a`. At around frame 50, press Ctrl-C in that terminal. Then run it a second time and, from another terminal, end it outright with `kill -9 <process id>`. Compare what each run leaves — image count, video file present or not, whether it plays and how far. This is the case the manual never covers and the one that matters if renders are driven from outside the window. *A result the other way:* the two ways of stopping leave exactly the same thing, or both leave a video that plays right through, or neither leaves any image files.
3. **Is the last image whole?** In test 1, check the highest-numbered image the File Output node wrote: does it open, and is it fully rendered or partly blank? *A result the other way:* it opens and the picture is complete, which would say that a file being there does mean the frame finished.
4. **Does the frame count match?** In test 1, compare the highest frame number among the File Output images with the number of frames the partial video actually plays. *A result the other way:* the two numbers are the same.
5. **Does the container matter?** Repeat test 1 with a different container from the ones the encoding panel offers, interrupting at the same frame, and see whether one leaves a playable file where another does not. *A result the other way:* every container tried leaves the same thing.
6. **Can the state the question describes be reached at all?** The question asks about all the images being written while the video has not finished. Set the frame range to 1–10 and run the same file twice: once straight through with no interruption, and once pressing Esc while the frame counter is on the last frame. Look at whether the File Output folder holds all ten images in the interrupted run, and how the video file compares with the uninterrupted one — size, and whether it plays to the end. *A result the other way:* the last image is missing, so that state was never reached; or the interrupted video is the same as the uninterrupted one.

---

2026-08-09 — Opus 5 (Claude Code) — Frames, Blender 5.2
