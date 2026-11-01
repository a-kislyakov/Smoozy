# Frames — later questions (Blender 5.0)

**Sources consulted, and when.** Blender 5.0's manual, Blender 5.0's Python API documentation, the Blender 5.0 release notes, and Cycles' own files, all as supplied in `Blender Sources/` and all taken on **2026-07-30**. That snapshot date is the date read for every finding below.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project — with one exception I want on the record: I listed the folder `Preparation/Blender Research/Findings/5.0/` to find out whether the file I was told to write already existed. I saw the names of the files in it and opened none of them. No content from anywhere in this project other than the question sheet entered this work.

I answered question 12 only. No helper agents were used; everything below is my own reading.

---

## 12. When one render writes one video and separate image files over the same frame range, what does an interruption leave if all the images have been written but the video has not finished closing?

**The reading I answered.** The question was ambiguous in two places, and I say so plainly rather than choosing silently.

*First ambiguity — how one render writes both.* I took it that the scene's own output cannot produce a video and separate image files at the same time, because its **Media Type** is a single choice. Here are the words that claim rests on:

> "media_type"
>
> "The type of media to save (default 'IMAGE')"
>
> Type: Literal['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']

Python API, Image Format Settings — https://docs.blender.org/api/5.0/bpy.types.ImageFormatSettings.html — read 2026-07-30.

Note what that does and does not carry. It gives one property with three possible values. Reading "the type of media to save" as *one at a time*, and reading out from that that the scene's own output has no other route to disk, are both steps a reader has to supply. So the narrowing is **inferred**, and the step is mine. What settles it outright is the hand test at **(i)** below, which put it to a running Blender 5.0.

*(Citation check, 2026-08-09 — Opus 5.* This passage first said the claim "rests on the media type property … which I read but did not quote here — so as it stands in this document the claim carries no words of its own and is not traceable." The words were where the researcher said they were; they are now quoted, the address is given, and the step past them is named and labelled inferred. Changed by the citation check.*)* On that reading, one render produces both by setting the scene output to Video and putting a **File Output** node in the compositor to write the images, and that is the reading I answered. The other reading would be two separate renders (or a second scene), which is not one interruption and not what the question describes.

*Second ambiguity — "has not finished closing".* I read this as: the video file's container was never completed, because the render ended before the step that finishes it. The other reading would be that the file is merely still open with its data already fully written. The sources do not describe either step, so as it happens the answer is the same for both, but the distinction matters and I flag it.

*(Hand tests, 2026-08-09 — Opus 5. The question was put to two running computers on 2026-08-09, and most of what this page calls "not in the sources" is now settled by observation. The observed answer is in its own section below, **Hand tests, 2026-08-09 — observed**. The labels below still stand, because they are statements about the documentation and the documentation has not changed.)*

**Short answer.** The sources do not say. Blender's documentation never describes the moment a video file is closed, never describes what an interruption leaves on disk for a video, and never describes what an interruption does to files a File Output node has already written. What it does say, twice, is that rendering straight to video is fragile and that a problem during the render can leave you with a file you cannot play and nothing to salvage.

### 12a. That one render can write a video and separate image files at once — **inferred**, from three documented pieces

The scene output's Video setting:

> "Video:
>
> Encodes each frame into a video container.
> Encoding options can be found in the Encoding panel."

Manual, Output panel — https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30.

The File Output node, writing the images:

> "The File Output node writes one or more images to disk during rendering or compositing.
> Each connected input is saved to the specified directory and filename,
> with the frame number appended to create a sequence."

Manual, File Output Node — https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html — read 2026-07-30.

And what makes the compositor run during a render at all:

> "Pipeline – Compositing
>
> Renders the output from the compositing node setup,
> and then applies the Composite node tree on all images,
> displaying the image inputted in the Group Output node."

Manual, Post Processing — https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html — read 2026-07-30.

**The step I took:** each quote above carries one piece on its own — that the scene output's Video setting encodes each frame into a container, that the File Output node writes images to disk during rendering, and that the compositing pipeline runs as part of a render. Those three pieces are documented. None of them says the two writes can both be in force in one render at the same time. A reader has to put the three together to get there, so the combination is inferred, not documented.

**Not covered.** None of this says in what order within a frame the image write and the video encode happen, nor that the two are guaranteed to stay in step frame for frame.

### 12b. What the interruption leaves for the video — **not in the sources**, with one documented warning close by

There is nothing in the manual, the API documentation, the release notes or Cycles' files about closing, finishing, or completing a video file, and nothing about what a video file contains if the render stopped. The nearest thing the manual says is a warning, and it is a warning about outcome, not a description of file state:

> "It's not recommended to render directly to a video format in the first instance.
> If a problem occurs while rendering, the file might become unplayable and you will
> have to re-render all frames from the beginning."

Manual, Output panel (Tips) — https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30.

Note the word "might". The manual does not say a stopped render always leaves an unplayable file, does not say a file is left at all, and does not distinguish an interruption during the frames from an interruption at the closing step. It also gives no way to repair or complete such a file — its remedy is to re-render everything.

A second page gives the same advice:

> "The Direct Approach, which is highly not recommended and not a standard practice,
> is where you set your output format to an AVI or MOV format,
> and click Animation to render your scene directly out to a movie file."

and, under Hints:

> "Your computer accidentally turns off in the middle of rendering your movie!
>
> Unless your animation renders in a few minutes,
> it is best to render the animation as separate image files.
> Instead of rendering directly to a compressed movie file, use a lossless format (e.g. PNG)."

Manual, Rendering Animations — https://docs.blender.org/manual/en/5.0/render/output/animation.html — read 2026-07-30.

**Not covered.** Whether a file exists on disk at all after such an interruption; whether it holds the frames encoded so far; whether any player or repair tool could read it; whether the answer differs between containers (the manual's Supported Video & Audio Formats page, which I read in full but do not quote here, names containers including MP4, Matroska, AVI and QuickTime, and says nothing about how any of them behaves when a write stops).

### 12c. That the video is being written as the render goes, not all at the end — **inferred**

The manual documents Autosplit:

> "If your video is huge and exceeds 2GiB, enable Autosplit Output.
> This will automatically split the output into multiple files after the first file is 2GiB in size."

Manual, Output panel (Encoding) — https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30.

**The step I took:** for a file to reach 2GiB and then cause a split to a second file, the file has to be growing while there are still frames left to render. So video data reaches disk during the render rather than being held back until the end. A reader has to supply that step, so this is inferred, not documented.

**Not covered.** This says nothing about whether what has reached disk is readable on its own, and nothing about whatever closing step the sources never mention. Data on disk and a usable file are not the same thing, and the manual's own "might become unplayable" warning is the evidence that they can come apart.

### 12d. What the interruption leaves for the separate image files — **not in the sources**

The manual says nothing about what happens to files a File Output node has already written when the render is interrupted. It does not say they are kept, and it does not say they are removed or rolled back. I found no statement either way.

The manual does make a survival statement, but it is about the *scene's own* output written as an image sequence, which is not the File Output node and not the setup this question describes:

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering,
> since the frames you have already rendered will still be in the output directory."

Manual, Rendering Animations (Hints) — https://docs.blender.org/manual/en/5.0/render/output/animation.html — read 2026-07-30.

And:

> "For example, when rendering animations to image file formats the render job can be canceled
> and resumed at the last rendered frame by changing the frame range."

Manual, Render Output Introduction — https://docs.blender.org/manual/en/5.0/render/output/introduction.html — read 2026-07-30.

I am deliberately not carrying either statement across to the File Output node. Both are written about the scene output in image mode, and this question's scene output is in video mode.

**Not covered.** Whether the File Output node's images survive; whether the last image written is complete or half-written; whether the image count and the video's frame count can be relied on to match.

### 12e. That the Python API lists separate handlers for a cancelled render and a completed one — **documented**, but it settles nothing about files

The Python API lists separate handlers for the two endings:

> "bpy.app.handlers.render_cancel
>
> on canceling a render job. Accepts one argument: the scene data-block being rendered"

> "bpy.app.handlers.render_complete
>
> on completion of render job. Accepts one argument: the scene data-block being rendered"

Python API, Application Handlers — https://docs.blender.org/api/5.0/bpy.app.handlers.html — read 2026-07-30.

**Not covered.** This shows only that Blender knows the difference between a cancelled job and a completed one. It says nothing about what either does to a video file or to images already on disk. I include it because it is the closest the sources come to the question, not because it answers it.

### Hand tests, 2026-08-09 — observed

Eleven tests were written for this question and run by hand on **2026-08-09**, the same tests in all four researched versions. The whole list, with the exact commands and the exact Python, is `Audits/Blender Research/Frames/Hand tests — later questions.md`. What follows is what came back for Blender 5.0.

**The two machines, in full.** *The rented Linux machine* — rented on Vast.ai and reached over SSH, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, and no display server at all; Blender 5.0.1 at `/opt/blender/5.0/blender`. It was shared with other work while these ran. *The Creator's Mac* — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics; Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`. Neither machine speaks for the other, so each result says which one it came from. Every run used `--factory-startup` and a fresh throwaway `BLENDER_USER_RESOURCES` folder.

**A word about the reader.** Neither machine has `ffprobe` or `ffmpeg`, so every video file was read back by loading it into Blender as a movie clip. That reader prints `loaded ok` even for a file holding nothing — a 48-byte file comes back as `loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. So each result records three things separately: whether the file exists, its size in bytes, and the frame count the reader gave.

**What was built, to what.** A blend-file made by script: the scene's own output with `media_type` set to `VIDEO` and then `file_format` set to `FFMPEG`, container MPEG-4, codec H.264, writing into one empty folder; the compositor given a Render Layers node feeding both a File Output node and the Group Output, the File Output node writing PNGs into a second empty folder; Cycles on the processor. Three things about driving 5.0 came out of building it, and they are facts about the machinery rather than answers to this question: the scene's compositor is reached at `scene.compositing_node_group` rather than `scene.node_tree`; the File Output node has `directory` and `file_name` rather than `base_path`; and the node writes nothing at all until an item is added to its `file_output_items`, its format's media type having defaulted to `MULTI_LAYER_IMAGE`. Rendered with `blender -b --factory-startup job.blend -a`, from outside Blender's own window.

*(Citation check, 2026-08-09 — Opus 5. **What this set-up does not say, and a repeater has to supply.** It does not say what is in the scene, how many samples, what resolution, or what frame rate. The citation check repeated these tests using the factory-startup scene as it stands, 128 samples, 640 by 360 for the eight-frame runs and 480 by 270 for the twenty-frame runs, and 24 frames a second — the last three read off this page's own readback lines, the first two guessed. Frames then rendered in about half a second where this run's took three or four, and that difference is what item (e) turns on. Anybody repeating this should write down the sample count. Changed by the citation check.)*

**(a) A complete run: the images arrive one per frame, the video arrives at the very end.** Frame range 1–8, watched ten times a second on Linux and four times a second on the Mac. On the rented Linux machine the PNG files appeared one at a time, `vid0001-0008.mp4` appeared at 1.133 s holding **48 bytes** and held 48 bytes throughout, the eighth PNG landed at 25.245 s with the video still at 48 bytes and still held open, and at 25.346 s the log reads:

> ` 25.346  imgs=[eight files] vids=[('vid0001-0008.mp4', 4832)] open=[]`

Readback of the finished file: `READVID loaded ok | frame_duration: 8 | size: (640, 360) | fps: 24.0`. On the Creator's Mac the same shape, ending at 4624 bytes and `frame_duration: 8`. **The two machines agreed.**

**(b) The state the question describes.** A handler on `bpy.app.handlers.render_write` printed the state at every frame and killed its own process the moment it fired for the last frame. On the rented Linux machine, with `SIGKILL`:

> `KILLATLAST render_write frame: 8 | images: ['ImageImage0001.png', 'ImageImage0002.png', 'ImageImage0003.png', 'ImageImage0004.png', 'ImageImage0005.png', 'ImageImage0006.png', 'ImageImage0007.png', 'ImageImage0008.png'] | video: [('vid0001-0008.mp4', 48)]`
> `KILLATLAST sending SIGKILL to self 53565`
> `exit status of blender: 137`

The line `KILLATLAST render call returned` was not printed, so the stop landed where it was aimed. Left on disk: eight PNG files of 368626 bytes each, and `vid0001-0008.mp4` of **48 bytes**, reading back `READVID exists: True | size: 48` and `READVID loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. `SIGTERM` gave exit status 143 and the identical result. On the Creator's Mac, both signals gave the identical result. **The two machines agreed.**

**(c) The last image was whole.** Pixels compared rather than bytes, between the eighth PNG of the stopped run and the eighth PNG of the complete run: `PIX RESULT: largest difference on any channel = 0.0` on both machines, the files differing in size by nine bytes on Linux (368626 against 368617) and eight on the Mac.

**(d) Interrupted partway, three ways.** Frame range 1–20, the signal sent once ten PNGs were on disk. On the rented Linux machine, `SIGINT` gave `RUNSIGNAL exit code: 1`, ten images and `[('vid0001-0020.mp4', 3845)]`, with Blender printing `Sent an internal break event. Press ^C again to kill Blender`; readback `frame_duration: 10`. `SIGTERM` gave exit code `-15` and `SIGKILL` gave `-9`, both leaving ten images and a 48-byte video reading back `frame_duration: 0`. On the Creator's Mac the same three, the `SIGINT` file being 3831 bytes and reading back `frame_duration: 10`. **The two machines agreed.**

**(e) A file being on disk is not the same as the frame having finished.** On the rented Linux machine, both the `SIGKILL` and the `SIGTERM` runs left their tenth image, `ImageImage0010.png`, at **155648 bytes** against 254253 for a complete one, with the other nine at the full size. The `SIGINT` runs left all ten at the full size.

*(Citation check, 2026-08-09 — Opus 5. **Disputed, going back to be run again. Do not lean on it.*** The citation check repeated all three runs on both machines in 5.0. On the rented Linux machine the tenth image came back at the full size after every signal — `('ImageImage0010.png', 95190)` after `SIGKILL` and the same after `SIGTERM` — and on the Creator's Mac the same, all ten full after all three. The citation check did see a cut-short image, but in Blender **4.5** on the Mac after `SIGTERM`, not here. Neither run overwrites the other and neither is picked. Written out as a test on the hand-test list, to be run ten times per signal per version per machine with the sample count recorded, because on this evidence it is a race between the signal and the image write and one run per signal cannot settle it. Changed by the citation check.)*

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. The test was run, as entry 12 of `Audits/Blender Research/Frames/Hand tests — later questions.md`: **ten runs for each of `SIGINT`, `SIGTERM` and `SIGKILL`, in all four versions, on both machines, 248 runs in all.** All three sets of runs now stand side by side and none is deleted.*

*The set-up numbers the earlier record left out, now written down: frames 1 to 20, 480 by 270, 24 frames a second, Cycles on the processor, four versions run side by side with a fixed thread count each — **200 samples at four threads on the rented Linux machine**, **150 samples at two threads on the Creator's Mac** — giving 4.2 to 4.9 seconds a frame on Linux and 4.9 to 5.5 on the Mac. The watcher polled every millisecond and sent the signal between 0.000013 and 0.001426 seconds after the tenth image file appeared.*

***What came back for Blender 5.0.*** *On the **rented Linux machine**, where a complete image in the control run was 254244 bytes: the tenth image, `ImageImage0010.png`, was 254244 bytes in all ten `SIGINT` runs and **0 bytes in all ten `SIGTERM` runs and all ten `SIGKILL` runs**. On the **Creator's Mac**, where a complete image was 256081 bytes: 256081 bytes in all ten `SIGINT` runs; **0 bytes in all ten `SIGTERM` runs**; and in the ten `SIGKILL` runs, 0, 0, 0, 24576, 0, 0, 0, 0, 0 and 0 bytes in run order.*

***The controls.*** *In every one of the sixty runs the first nine images were at the full size and exactly ten images were on disk, so the render was stopped rather than finished; exit statuses were 1, -15 and -9 for the three signals throughout. The full size comes from a complete uninterrupted run of the same file on the same machine, which left all twenty images at one size. And the other outcome is a real one, because it happened: `SIGINT` gave ten runs with every image whole.*

***How this sits beside the two runs above, without picking between them.*** *On `SIGINT` all three agree. On `SIGTERM` and `SIGKILL` this batch **matches the shape of the finding above exactly** — short after both, whole after `SIGINT` — although the short files here were empty where the finding above records 155648 bytes, and this batch found the same shape in all four versions rather than in 5.0 alone. It does **not** match the citation check, which found every image at the full size after every signal in 5.0 on both machines. Nothing here says which earlier run was right; the earlier runs were one run per signal, this one was ten, and this one states a condition theirs do not — how quickly the signal followed the file appearing. Added by the second batch of hand tests.)*

**(f) A second run over the same range**, on the rented Linux machine, starting from what the `SIGKILL` run left — a 48-byte video and ten PNGs, the tenth of them cut short — and allowed to finish: the video went from 48 bytes to **6897 bytes** with a new modification time, reading back `frame_duration: 20`, and all twenty PNGs were written again at 254253 bytes with new modification times, including the nine the first run had already completed.

**(g) Whether a cancel handler fires when the stop comes from outside.** On the rented Linux machine, with handlers on `render_cancel`, `render_complete` and `render_write`, and `SIGINT` sent once ten images were written:

> `CANCELWATCH render_write frame 10 | video sizes: [('vid0001-0020.mp4', 48)]`
> `CANCELWATCH render_cancel fired at frame 1`
> `CANCELWATCH copied /root/ht-frames/e10_5.0_SIGINT/vid/vid0001-0020.mp4 3845 -> /root/ht-frames/e10_5.0_SIGINT/copy/vid0001-0020.mp4`
> `CANCELWATCH render call returned`
> `RUNSIGNAL copy and final file identical byte for byte: True`

The file was already at its full 3845 bytes when the cancel handler fired, and nothing changed it afterwards. With `SIGTERM`, no `render_cancel` line appeared at all, no copy was taken, and the file left behind was 48 bytes reading back `frame_duration: 0`. `render_complete` never appeared in either run. The control that the handlers were working is the `CANCELWATCH render_write frame N` line, once per frame in both runs.

**(h) The container changes the number of bytes left, not the frame count.** On the rented Linux machine the containers 5.0 offers are `['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME']` — the same ten as in the other three versions. The same stop-at-the-last-frame run in each left: MPEG4 48 bytes, MKV 0, WEBM 262, AVI 0, DV no file at all, FLASH 0, MPEG1 0, MPEG2 0, OGG 0, QUICKTIME 36. Every one read back `frame_duration: 0`. WEBM and DV never reached the interruption and said so, printing `RuntimeError: Error: Could not initialize streams, probably unsupported codec combination` and `RuntimeError: Error: Render width has to be 720 pixels for DV!` and exiting 0. The control — the same containers rendered to the end — produced readable files for the other eight (MPEG4 2243 bytes / 4 frames, MKV 2035 / 4, AVI 7232 / 4, FLASH 3592 / 4, MPEG1 4096 / 2, MPEG2 6144 / 2, OGG 5402 / 1, QUICKTIME 2177 / 4). This is the answer to the researcher's own item 2 below, and it also names the containers the version really offers, which the *Not covered* note in 12b said was not quoted from anywhere.

**(i) The Media Type premise, which 12a rests on.** On the rented Linux machine:

> `Q6 media_type present: True`
> `Q6 media_type is_enum_flag: False | items: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`
> `Q6 file_format is_enum_flag: False`
> `Q6 file_format now: FFMPEG`
> `Q6 set FFMPEG ok, file_format now: FFMPEG`
> `Q6 set PNG FAILED: TypeError bpy_struct: item.attr = val: enum "PNG" not found in ('FFMPEG')`
> `Q6 is_movie_format: True`

So the media type holds one value at a time (`is_enum_flag: False`), and with it set to `VIDEO` the only file format the render output will accept is `FFMPEG`. That is the premise the researcher stated without a quote, tested. One caution about this record: the two format lists the script printed under `IMAGE` and under `VIDEO` came back identical, because that listing is the property's whole declared list rather than what is on offer at the moment; what shows the live state is the failed assignment above and `is_movie_format` reading `False` under the image setting and `True` under the video setting in the same run. The same script printed `Q6 media_type present: False` in Blender 4.5, where there is no such property.

**(j) A File Output node cannot be set to write a video.** On the rented Linux machine:

> `Q7 node format media_type present: True`
> `Q7 node media_type items: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`
> `Q7 node media_type now: MULTI_LAYER_IMAGE`
> `Q7 set node media_type VIDEO FAILED: TypeError bpy_struct: item.attr = val: enum "VIDEO" not found in ('IMAGE', 'MULTI_LAYER_IMAGE')`
> `Q7 set node file_format FFMPEG FAILED: TypeError bpy_struct: item.attr = val: enum "FFMPEG" not found in ('OPEN_EXR_MULTILAYER')`

5.1 and 5.2 printed the same. Blender 4.5 has no such setting on the node at all. So in none of the four versions would the node take a video setting, and the second reading of the question — one File Output node writing the video — cannot be built.

**(k) Overwrite and Placeholders can be changed with a video output in force.** On the rented Linux machine, all four versions printed the same six lines; for 5.0:

> `Q9 IMAGE file_format: PNG | is_movie_format: False`
> `Q9 IMAGE use_overwrite was True now False`
> `Q9 IMAGE use_placeholder was False now True`
> `Q9 VIDEO file_format: FFMPEG | is_movie_format: True`
> `Q9 VIDEO use_overwrite was True now False`
> `Q9 VIDEO use_placeholder was False now True`

This is about the settings, not about how the boxes are drawn in a panel.

**(l) Autosplit: the file already split away survives, the one being written does not.** On the rented Linux machine only. Two set-ups failed first and are recorded rather than quietly replaced: container `AVI` with codec `NONE` gave `Error: Could not initialize streams, probably unsupported codec combination` and a 0-byte file, and container `AVI` with codec `HUFFYUV` worked but was far too slow to reach 2 GiB. What was used: container `QUICKTIME`, codec `QTRLE`, 3840 by 2160, `use_autosplit = True`, frame range 1–4000, and the **Workbench** engine rather than Cycles so that a couple of thousand frames could be reached at all — that last change is a departure from the set-up used everywhere else here and is recorded because the engine is not what this test is asking about. The render was watched and killed with `SIGKILL` the moment a second video file appeared. What came back:

> `RUNSPLIT 2295.3 s: a second video file has appeared: ['vid_0000001-4000.mov', 'vid_0010001-4000.mov'] -- sending SIGKILL to pid 139549`
> `RUNSPLIT exit code: -9`
> `RUNSPLIT files left: [('vid_0000001-4000.mov', 2012531479), ('vid_0010001-4000.mov', 36)]`

Readback: the first file `loaded ok | frame_duration: 2197 | size: (3840, 2160) | fps: 24.0`; the second file `loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. The other three versions gave the same two files with the same two names and the same shape of answer. The control that the split really happened is the file count: one file would have meant the condition never bit, and there were two. **The Creator's Mac was not used for this test**, because each run writes over two gigabytes and takes between thirty and sixty-seven minutes; macOS is untested for autosplit and that is a gap rather than an answer.

**What these tests do not settle.** Nothing was run inside Blender's own window, so nothing here says what pressing Esc or clicking the status bar's cancel button leaves. Nothing was run on Windows, because no Windows machine has run it. The video reader used was Blender itself, so a frame count is what Blender's movie-clip reader made of the file rather than what every player would make of it. And every run used one scene, one codec and one frame rate. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Where I looked, so that "not in the sources" carries weight

Searched across the whole of the 5.0 manual, the whole of the 5.0 Python API documentation, all 23 release-note files for 5.0, and all of Cycles' supplied files, for: *close / closing the file / closed properly, finalize, trailer, write trailer, mux, muxer, moov, footer, header, flush, index, truncated, partial, partially written, half-written, partial file, incomplete, unfinished, unplayable, not playable, corrupt, left behind, already written, interrupt, cancel, cancelled, abort, stop rendering, canceled render, resume, power outage, crash while rendering, on cancel, end of the render.*

Of all of those, the only hits bearing on this question were the ones quoted above. "Unplayable" appears exactly once in the entire manual, in the Output panel tip quoted in 12b. "Trailer", "muxer" and "moov" do not appear in any technical sense anywhere. There is no page in the manual about a render being interrupted.

Pages I read in full and ruled out: Render Output Introduction; Output Properties → Output (the whole page, including the Encoding panel and both Tips); Post Processing; Rendering Animations; Supported Video & Audio Formats (every container and codec entry, and the "Known Limitations" section — it covers only output size, nothing about interrupted writes); File Output Node; Rendering From The Command Line; Command Line Arguments (format and output sections); Render Introduction. In the API: `bpy.types.RenderSettings` (including `is_movie_format`, `use_overwrite`, `use_placeholder`, `use_file_extension`), `bpy.types.FFmpegSettings` (every property, including `use_autosplit`), `bpy.types.ImageFormatSettings` (`media_type`), `bpy.types.CompositorNodeOutputFile`, and `bpy.app.handlers`. In the release notes: the compositor, pipeline & I/O, rendering, core, python API and corrective-releases files; the only 5.0 changes touching this ground are the File Output node redesign and its frame-numbering change, and HDR video writing — none of them mention interruption. In Cycles' files: searched every file for *ffmpeg, movie, video, container, mux, file_output, render_file* and found nothing at all, which is consistent with Cycles being the engine that produces pixels rather than the part of Blender that writes output files.

I also opened the manual's picture of the Output panel, `_images/render_output_properties_output_panel.png` (linked from the Output panel page above). It shows a panel with Overwrite, Placeholders, File Extensions, Cache Result, File Format PNG, Color, Color Depth and Compression. **Documented, from the picture:** those controls sit together on the Output panel. The picture shows an image format, not a video one, so it says nothing about the video case and I take nothing further from it.

I did not need the web and did not go there.

### Outside the interface

*(Hand tests, 2026-08-09 — Opus 5. This line is now answered by observation. Every run recorded in **Hand tests, 2026-08-09 — observed** above started Blender with `-b --factory-startup` and no window, on the Creator's Mac and on the rented Linux machine, which has no display server and cannot open one. So what is recorded there is the driven-from-outside behaviour for Blender 5.0: images arriving one per frame, a video file that holds 48 bytes until the end, a `SIGINT` that closes the file and fires `render_cancel`, and a `SIGTERM` or `SIGKILL` that does neither. It also answers the *Not covered* line just below — whether an interruption arriving as a signal to a background Blender behaves like a cancel: with `SIGINT` it did, with `SIGTERM` it did not. What is still not settled is the other direction, whether a stop from inside the window leaves the same thing; that needs eyes on a window and was dropped this round.)*

The sources do not say. Nothing in the manual, the API documentation or the release notes describes what an interruption leaves for a video file whether Blender is driven by a person or by a command line, so there is no documented difference and no documented sameness to report. The warning in 12b is written without reference to how the render was started.

Three related things the sources *do* say, which bear on this setup when Blender is driven from outside:

> "-a
>
> Render the whole animation using all the settings saved in the blend-file."

Manual, Rendering From The Command Line — https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — read 2026-07-30.

> "The **File Output** node now only appends the frame number to the image name when doing an
> animation render. Rendering through the command line always does an animation render, so no change
> of behavior is expected."

Release notes, Compositor — https://developer.blender.org/docs/release_notes/5.0/compositor/ — read 2026-07-30.

**The step I took — inferred:** the quote says the node appends the frame number when doing an animation render, and that a command-line render always does an animation render. From those two halves together I take it that a command-line render numbers the node's images per frame, which is what makes "separate image files over the same frame range" the normal outcome there. A reader has to put the two halves side by side to get there, so this is inferred, not documented.

And the manual's note that command-line output arguments are order-sensitive:

> "Always position -f or -a as the last arguments."

Same page. This affects whether the output settings you meant are the ones in force at all, which is upstream of anything about interruption.

**Not covered.** Whether an interruption arriving as a signal to a background Blender behaves like a cancel from the interface. The sources say nothing about signals, Ctrl-C, or process termination.

### Operating system

*(Hand tests, 2026-08-09 — Opus 5. Two of the three are now answered by observation, and separately. Every test above that was run on both machines was run twice — once on the Creator's Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, and once on the rented Linux machine, Ubuntu 22.04.5 LTS on Vast.ai with an NVIDIA GeForce RTX 5060 and no display server. For Blender 5.0 the two agreed on every result: the same 48 bytes held until the end, the same complete set of images at the moment of the stop, the same 48-byte file after a `SIGTERM` or a `SIGKILL`, the same ten frames read back after a `SIGINT`, and the same whole last picture. **Windows is untested**, not answered: no Windows machine has run it.)*

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. The 248 runs recorded at item (e) above put one qualification on "the two agreed on every result". **They agree on the shape and differ on the detail.** On both machines a `SIGINT` left the last image whole every time and a `SIGTERM` or `SIGKILL` left it short every time. What differed is what "short" was: on the rented Linux machine every one of the eighty short files, across all four versions, was exactly **0 bytes**, while on the Creator's Mac sixty-nine were 0 bytes and eleven were a whole number of 4096-byte blocks — 4096, 16384, 24576, 40960 or 73728 — all far short of a complete image. **[Recount, 2026-08-09 — this sentence used to say seventy-two and eight, and that was a miscount. Counting the run-by-run figures themselves — this version's ten runs per signal as printed on this page, and the other three versions' in the hand-test record these findings were written from — gives sixty-nine empty and eleven block-sized. A recount of figures already on the record: nothing was re-run, and no new fact is added.]** In Blender 5.0 itself, the Mac's one non-empty short file was 24576 bytes, in the fourth `SIGKILL` run. Windows is still untested.*)*

The sources do not say that the interruption behaviour differs by operating system — they do not describe the behaviour at all, so there is nothing to vary.

Two documented facts do vary by system and sit underneath this question:

> "Formats that can be compiled into Blender, not available on all systems:
> OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

Manual, Command Line Arguments — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30. What these words carry is that FFMPEG is one of the things named as not present in every build. That FFMPEG is what video output goes through is not carried by anything I quoted, so I do not claim it here — it is worth checking before anyone leans on this line.

> "Codecs are used to encode and decode the movie,
> and so must be present on both the encoding machine (Blender) and the target machine."

and

> "Each has advantages and disadvantages, and compatibility with different players on
> different operating systems."

Manual, Supported Video & Audio Formats — https://docs.blender.org/manual/en/5.0/files/media/video_formats.html — read 2026-07-30.

**Not covered.** Whether a half-written file from one system reads the same way on another. The sources are silent.

---

## Things a person with Blender open could settle in a few minutes

*(Hand tests, 2026-08-09 — Opus 5. Six of the seven items on this list were run on 2026-08-09 and their answers are in **Hand tests, 2026-08-09 — observed** above, so they are struck here and only what is genuinely still open is left. Item 1, does the interrupted video exist and what is in it — see (d). Item 2, the same per container — see (h), where all ten containers 5.0 offers were run, not the four named here. Item 3, do the File Output node's images survive and do they match — see (d) and (e), and read the whole of (e), which now carries three sets of runs rather than one. Item 4, the exact case the question asks about — see (b); it was reached not by hand but by a handler that stopped the process at the moment the last frame was written, which is why the caution added to it no longer applies. Item 5, does Blender hold the video file open — see (a), where the handle was watched for the whole run and released in the same tenth of a second in which the file reached its full size. Item 7, interruption from outside with three different endings — see (d) and (g); the Windows part of it was not run and is noted below. Item 6, Autosplit, is still open and is noted below.)*

**Answered on Linux, untested on the Mac — Autosplit.** What an interruption leaves when the file has already split into more than one was run on the rented Linux machine and is at (l) above: the file already split away held every frame that went into it, and the one being written was left at 36 bytes holding none. It was not run on the Creator's Mac, because each run writes over two gigabytes and takes between thirty and sixty-seven minutes, so macOS is untested for this one.

**Still open — anything needing eyes on a window.** Items 1 to 4 were written as somebody clicking cancel. What was run instead was a signal sent from outside, because the rented Linux machine has no display server and the Creator ran nothing this round. So it is not known whether a stop from inside the window leaves the same thing as a signal from outside.

**Not run — Windows.** Item 5's Windows half ("on Windows, use Resource Monitor's associated-handles search") and item 7's ("Repeat on Windows, macOS and Linux") were not tried: no Windows machine has run them. macOS and Linux are each answered above, on their own; Windows is untested rather than answered.

The researcher's original list follows, kept as the record of what was thought open at the time.

1. **Does the interrupted video file exist, and what is in it?**
   Open a scene with a short animation, say frames 1–20, that takes a few seconds per frame. In Properties → Output → Output, set Media Type to Video, choose Matroska container with H.264, and set the file path to an empty folder. Start Render Animation. When the status bar reaches roughly frame 15, cancel the render. Then look in that folder. Record: is there a file; what size is it; will VLC, QuickTime and `ffprobe` open it; if any of them opens it, how many frames does it contain and does it reach frame 15 or stop earlier.

2. **Same test, per container.** Repeat check 1 four times, changing only the container: MPEG-4, Matroska, AVI, QuickTime. Record for each whether a file is left and whether anything will play it. This settles whether "might become unplayable" depends on the container, which the manual does not say.

3. **Do the File Output node's images survive, and do they match the video?**
   To the same scene, in the Compositing workspace with Use Nodes on and Properties → Output → Post Processing → Compositing enabled, add a File Output node fed from the render's image, set its Directory to a second empty folder, its File Name to `img_`, and its Media Type to Image with PNG format. Render the animation and cancel it partway as before. Record: how many PNG files are in the second folder and which frame numbers they carry; whether the highest-numbered PNG opens as a complete image or is truncated; and how the highest PNG frame number compares to the last frame the video actually contains.

4. **The exact case the question asks about — interrupt after the last frame but before the file is finished.**
   Set the frame range to something very short, say frames 1–3, so the frames finish quickly. Render to video with a File Output node as in check 3. Watch for the moment the last frame's image appears in the File Output folder, and cancel the render immediately at that instant. Then record whether the video file left behind is playable, whether it contains all three frames, and whether all three PNGs are present and complete. Repeat several times, since the timing is tight and the result may not be the same every attempt. **Caution on this one:** hitting that instant by hand may not be possible at all, and a run that misses it looks the same as a run that hit it. Do check 5 first, which is what would show whether there is a gap to aim at.

5. **Does Blender leave the video file open?**
   During a video render on macOS or Linux, run `lsof` against the output file (on Windows, use Resource Monitor's associated-handles search). Record whether Blender holds the file open for the whole render, and at what point in the render the handle is released. This is what would settle whether there is a distinct closing step at all — the sources never mention one.

6. **Autosplit at the 2GiB boundary.** Render to video with Autosplit Output enabled and settings crude enough to pass 2GiB quickly (large resolution, a high bitrate, an intra-only codec such as FFV1 or PNG). Cancel after the second file appears. Record whether the first file — the one that was completed and split away before the interruption — plays, and whether the second, unfinished one does. This would show whether a file Blender finished mid-render is left in a better state than the one it was still writing.

7. **Interruption from outside the interface.** Run `blender -b file.blend -a` with the same video-plus-File-Output setup, and end it three ways in three separate runs: Ctrl-C in the terminal; a plain `kill` (SIGTERM); and a `kill -9` (SIGKILL). Record for each what is left on disk — video file present or not, playable or not, and how many File Output images survived. Repeat on Windows, macOS and Linux, since the sources say nothing about whether this varies.

---

2026-08-09 — Opus 5 (Claude Code) — Frames, Blender 5.0
