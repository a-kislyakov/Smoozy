# Frames — later questions (Blender 4.5)

**Sources consulted, and when.** Blender 4.5 manual, Blender 4.5 Python API reference, Blender 4.5 release notes, and the Cycles add-on files, all from the snapshot in `Blender Sources/` taken on 2026-07-30. That snapshot date is the date read for every citation below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One precise qualification to that line, so it is not overstated. Before writing, I listed the contents of the folder this file is saved in, `Preparation/Blender Research/Findings/4.5/`, to check whether this file already existed. I saw the file names in it and nothing else — I opened none of them and read none of their contents. No other part of this project was listed, searched or opened. I started no helper agents, so this declaration covers all the work behind this file.

Only question 12 was assigned. The rest of the sheet is untouched.

---

## 12. When one render writes one video and separate image files over the same frame range, what does an interruption leave if all the images have been written but the video has not finished closing?

### The question is ambiguous, and here is the reading I answered

The question does not say how one render comes to write both a video and separate image files. In Blender 4.5 the render output has a single File Format at a time, so the only arrangement I can support from the sources is: the scene's own output is set to a video format, and one or more File Output nodes in the Compositor write image files on the same frames. **That is the reading I answered.**

*(Citation check, 2026-08-09 — Opus 5.* The sentence "the render output has a single File Format at a time" is stated here with no quote behind it and no label, and it is the premise that rules two of the three readings out. Nothing on this page's **documentation** side carries it. What does carry it is the hand test at **(i)** below, which put the premise to a running Blender and printed `Q6 file_format is_enum_flag: False`, `Q6 set FFMPEG ok`, then `Q6 set PNG ok`. So the premise is **observed**, not documented, and its evidence is (i). The earlier audit note here sent it back to a fresh researcher; that is now stale, because (i) settles it and the citation check re-ran (i) and got the same lines. Changed by the citation check.*)*

The other readings I can see, and did not answer:

- Two separate render jobs over the same frame range — but the question says "one render".
- A File Output node itself set to write a video. The 4.5 release notes say the File Output node's format settings gained a Media Type that can be Video (quoted below), which makes this conceivable, but the manual and the API reference for 4.5 describe nothing about it, so I cannot answer that reading.

### Answer

*(Hand tests, 2026-08-09 — Opus 5. The question was put to two running computers on 2026-08-09 and much of what is called "not in the sources" below is now settled by observation. The observed answer is in its own section, **Hand tests, 2026-08-09 — observed**, further down this page. The "not in the sources" label below still stands, because it is a statement about the documentation and the documentation has not changed.)*

**LABEL: not in the sources** — for the question as asked. The sources never describe the state of the files left behind when a render that was writing a video is interrupted, and they never describe a render that writes a video and image files at the same time. Three neighbouring facts *are* documented, and they are the most that can be said:

**(a) A File Output node saves an image writing to the current frame, and saves a whole sequence when an animation is rendered.**

LABEL: **documented**

> "This node writes out an image, for each frame range specified, to the filename entered, as part of a frameset sequence."

> "The image(s) will be saved on rendering, writing to the current frame. An entire sequence of images will be saved, when an animation is rendered."

Source: manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html — read 2026-07-30.

The API reference gives the node the same one-line description:

> "Write image file to disk"

Source: API reference, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30.

What this does **not** say: it does not say that a written image file is complete or valid, and it does not say what happens to the image being written at the instant the render is interrupted. It establishes that a save happens per frame, and that an animation produces a sequence of saves.

*(Audit note, 2026-08-09 — Opus 5.* This finding first read "written frame by frame, as the render goes … not once at the end". That is a further step: the quoted words say what each save is aimed at — "the current frame" — not when the writing happens on the clock, and they do not rule out saves being held and flushed at the end. The step is **inferred**, not documented, and is written out here rather than left inside the claim. Hand test 4 below is what would settle it.*)*

*(Citation check, 2026-08-09 — Opus 5.* The heading of this finding first read "saves an image per frame". That is one step further again: "an image per frame" needs the reader to add that an animation makes each frame the current frame in turn, and neither quote says so. The heading now reads what the quotes carry — an image saved writing to the current frame, and a whole sequence when an animation is rendered. The per-frame reading is **inferred** and it is the observed item (a) below that settles it by watching. Changed by the citation check.*)*

**(b) A video that a render was writing may be left unplayable, and the manual's remedy is to re-render everything.**

LABEL: **documented**

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30.

The same page's neighbour puts it as a hint:

> "Your computer accidentally turns off in the middle of rendering your movie!"

> "Unless your animation renders in a few minutes, it is best to render the animation as separate image files. Instead of rendering directly to a compressed movie file, use a lossless format (e.g. PNG)."

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30.

And the introduction to render output states the resumability contrast plainly, for image formats only:

> "when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/introduction.html — read 2026-07-30.

Note the exact wording in the first quote: "**might** become unplayable". The manual does not say the file *is* unplayable, does not say under what conditions it is or is not, and does not distinguish a video that got all its frames from one that got half of them. It gives a risk, not an outcome.

**(c) Blender does distinguish a cancelled render job from a completed one, at least well enough to fire a different notification.**

LABEL: **documented**

> "bpy.app.handlers.render_cancel"
> "on canceling a render job. Accepts one argument: the scene datablock being rendered"

> "bpy.app.handlers.render_complete"
> "on completion of render job. Accepts one argument: the scene datablock being rendered"

> "bpy.app.handlers.render_write"
> "on writing a render frame (directly after the frame is written)"

Source: API reference, https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30.

What this does **not** say: it says nothing about what Blender does to an open video file when a job is cancelled — whether it closes it, finalises it, discards it, or leaves it as it stands. It only shows that a cancel is a distinct event from a completion.

**One picture, because it bears on the video side of the same panel.**

LABEL: **documented** for what the two pictures show; **inferred** for the general rule drawn from them (the step is written out below).

I opened `_images/render_output_properties_output_video-encoding-panel.png`, the "Encoding panel" picture on https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, and the "Output panel" picture `_images/render_output_properties_output_panel.png` on the same page. In the first, File Format is "FFmpeg video" and the two Image Sequence checkboxes — Overwrite and Placeholders — are drawn greyed out. In the second, File Format is "PNG" and those same two checkboxes are drawn active. Read 2026-07-30.

That, and only that, is what the two pictures show: two panels, each at one setting, with those boxes drawn one way in one and the other way in the other.

*(Audit note, 2026-08-09 — Opus 5.* This finding first read, as documented: "the Overwrite and Placeholders controls are unavailable when the render output is a video and available when it is a still image format", followed by "I am not reading anything further into it." It is reading two things further, and both are steps the reader has to supply. One: that a control drawn greyed in a screenshot is a control that cannot be used. Two: that what two pictures show at two particular settings holds for every video format and every still image format. Neither step is on the page. The general claim is therefore **inferred**. The picture-level description above stands as documented.*)*

**What the sources never say, and what I therefore cannot answer.**

- Whether a video file that received every frame but was never closed contains those frames in a usable form.
- Whether such a file is left on disk at all, or removed.
- Whether Blender writes anything at the end of a video render that a stopped render would skip.
- Whether the images written by File Output nodes are affected in any way by the video output failing — the sources never place the two outputs in the same render.
- Which of the two — the video or the images — is written first on a given frame.

I searched for all of this and found nothing; the record of the search is below.

### Hand tests, 2026-08-09 — observed

Eleven tests were written for this question and run by hand on **2026-08-09**, the same tests in all four researched versions. The whole list, with the exact commands and the exact Python, is `Audits/Blender Research/Frames/Hand tests — later questions.md`. What follows is what came back for Blender 4.5.

**The two machines, in full.** *The rented Linux machine* — rented on Vast.ai and reached over SSH, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, and no display server at all; Blender 4.5.12 at `/opt/blender/4.5/blender`. It was shared with other work while these ran. *The Creator's Mac* — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics; Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`. Neither machine speaks for the other, so each result below says which one it came from. Every run used `--factory-startup` and a fresh throwaway `BLENDER_USER_RESOURCES` folder.

**A word about the reader.** Neither machine has `ffprobe` or `ffmpeg`, so every video file was read back by loading it into Blender as a movie clip and printing what it reported. That reader prints `loaded ok` even for a file holding nothing — a 48-byte file comes back as `loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. So each result records three things separately: whether the file exists, its size in bytes, and the frame count the reader gave.

**What was built, to what.** A blend-file made by script: the scene's own output set to `FFMPEG`, `ffmpeg.format = 'MPEG4'`, `ffmpeg.codec = 'H264'`, writing into one empty folder; the compositor given a Render Layers node feeding both a Composite node and a File Output node, the File Output node writing PNGs into a second empty folder; Cycles on the processor. In 4.5 the compositor is reached at `scene.node_tree` and the File Output node's folder at its `base_path`, and `scene.render.image_settings` has no `media_type` property, so `file_format` was set to `FFMPEG` directly. The file was then rendered with `blender -b --factory-startup job.blend -a` — from outside Blender's own window, on machines where in one case no window is possible at all.

*(Citation check, 2026-08-09 — Opus 5. **What this set-up does not say, and a repeater has to supply.** It does not say what is in the scene, how many samples, what resolution, or what frame rate. The citation check repeated these tests using the factory-startup scene as it stands, 128 samples, 640 by 360 for the eight-frame runs and 480 by 270 for the twenty-frame runs, and 24 frames a second — the last three read off this page's own readback lines, the first two guessed. Frames then rendered in about half a second where this run's took three or four, and that difference is what item (e) turns on. Anybody repeating this should write down the sample count. Changed by the citation check.)*

**(a) A complete run: the images arrive one per frame, the video arrives at the very end.** Frame range 1–8. A watcher recorded the state ten times a second on Linux and four times a second on the Mac. On the rented Linux machine the log reads, at its turning points:

> `  0.726  imgs=[] vids=[('vid0001-0008.mp4', 48)] open=['vid0001-0008.mp4']`
> `  4.555  imgs=['Image0001.png'] vids=[('vid0001-0008.mp4', 48)] open=['vid0001-0008.mp4']`
> … one further PNG at each of 8.400, 11.729, 15.780, 19.816, 23.160, 26.989 …
> ` 31.010  imgs=[eight files, Image0001.png … Image0008.png] vids=[('vid0001-0008.mp4', 48)] open=['vid0001-0008.mp4']`
> ` 31.111  imgs=[eight files] vids=[('vid0001-0008.mp4', 4655)] open=[]`
> `exit code: 0`

Readback of the finished file: `READVID exists: True | size: 4655` and `READVID loaded ok | frame_duration: 8 | size: (640, 360) | fps: 24.0`.

On the Creator's Mac the same shape: the video file at 48 bytes from the first second until the process was gone, then 4789 bytes, reading back `frame_duration: 8 | size: (640, 360) | fps: 24.0`; the PNGs appearing one at a time.

**The two machines agreed.** The File Output node's images reach the disk one per frame while the render runs. The video file is created at once at 48 bytes, stays at 48 bytes for the whole render, and reaches its full size in the same tenth of a second in which the process lets go of it.

**(b) The state the question describes.** A handler on `bpy.app.handlers.render_write` printed the state at every frame and killed its own process the moment it fired for the last frame. On the rented Linux machine, with `SIGKILL`:

> `KILLATLAST render_write frame: 8 | images: ['Image0001.png', 'Image0002.png', 'Image0003.png', 'Image0004.png', 'Image0005.png', 'Image0006.png', 'Image0007.png', 'Image0008.png'] | video: [('vid0001-0008.mp4', 48)]`
> `KILLATLAST sending SIGKILL to self 49538`
> `exit status of blender: 137`

The line `KILLATLAST render call returned` was not printed, so the stop landed where it was aimed. Left on disk: eight PNG files of 162489 bytes each, and `vid0001-0008.mp4` of **48 bytes**, reading back `READVID exists: True | size: 48` and `READVID loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. The same run ended with `SIGTERM` instead gave exit status 143 and the identical result.

On the Creator's Mac, both signals gave the identical result: all eight images present at the moment of the stop, a 48-byte video file, `frame_duration: 0`.

**The two machines agreed.**

**(c) The last image was whole.** Comparing pixels rather than bytes, between the eighth PNG of the stopped run and the eighth PNG of the complete run: on the rented Linux machine `PIX RESULT: largest difference on any channel = 0.0`, with the two files differing in size by nine bytes (162489 against 162480). On the Creator's Mac the same, `0.0`, files differing by eight bytes.

**(d) Interrupted partway, three ways.** Frame range 1–20; the signal sent once ten PNGs were on disk. On the rented Linux machine:

> `RUNSIGNAL 11.998 images on disk: 10 -- sending SIGINT to pid 86676`
> `RUNSIGNAL exit code: 1`
> `RUNSIGNAL video on disk after: [('vid0001-0020.mp4', 3960)]`

with Blender printing `Sent an internal break event. Press ^C again to kill Blender` and then `Blender quit`. Readback: `size: 3960`, `loaded ok | frame_duration: 9 | size: (480, 270) | fps: 24.0`. The same run with `SIGTERM` gave exit code `-15`, ten images and `[('vid0001-0020.mp4', 48)]`, reading back `frame_duration: 0`; with `SIGKILL`, exit code `-9` and the same 48 bytes and `frame_duration: 0`.

On the Creator's Mac, `SIGINT` left 3963 bytes reading back `frame_duration: 10`; `SIGTERM` and `SIGKILL` left 48 bytes and `frame_duration: 0`. Ten images in every run on both machines.

**The two machines agreed on the pattern and differed on one number**: the file left by `SIGINT` read back as **9** frames on the rented Linux machine and **10** on the Creator's Mac, with ten images on disk in both. The number also moved between runs on the same machine — the `SIGINT` run at (g) below, same signal and same set-up with handlers added, left a file the reader called **10** frames on the rented Linux machine. So the frame count the reader gives for a `SIGINT`-stopped file is not reliably the same as the image count and was not the same twice. What did not move is the contrast itself: `SIGINT` leaves a file the reader finds frames in, `SIGTERM` and `SIGKILL` leave one it finds none in.

**(e) A file being on disk is not the same as the frame having finished.** On the rented Linux machine, the `SIGKILL` run above left its tenth image, `Image0010.png`, at **0 bytes**, with the other nine at 93611 bytes each. The `SIGTERM` and `SIGINT` runs left all ten at the full size.

*(Citation check, 2026-08-09 — Opus 5. **This one is disputed and is going back to be run again. Do not lean on it.*** The citation check repeated all three runs on both machines and got something different. On the rented Linux machine, all ten images came back at the full size after **every** one of the three signals — `D SIGKILL images after: [('Image0001.png', 70768), … ('Image0010.png', 70768)]`. On the Creator's Mac, in this same version, a **0-byte** tenth image appeared — but after `SIGTERM`, not `SIGKILL`: `D SIGTERM images after: [… ('Image0009.png', 70768), ('Image0010.png', 0)]`, with `SIGINT` and `SIGKILL` both leaving all ten full. So the thing this finding points at does happen and the citation check saw it happen; which signal shows it did not hold from one run to the next. Neither run is being overwritten and neither is being picked. The likeliest reading is that this is a race between the signal and the image write, and that a single run per signal cannot settle it — the citation check's frames rendered in about half a second where this run's took three or four, which changes how often the signal lands mid-write. It is written out as a test on the hand-test list, to be run ten times per signal per version per machine with the sample count recorded. Changed by the citation check.)*

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. The test above was run, as entry 12 of `Audits/Blender Research/Frames/Hand tests — later questions.md`. **Ten runs for each of `SIGINT`, `SIGTERM` and `SIGKILL`, in all four versions, on both machines — 248 runs in all.** All three sets of runs now stand side by side and none is deleted.*

*What was set, since that is what the earlier record left out: frames 1 to 20, 480 by 270, 24 frames a second, Cycles on the processor, and — because four versions were run side by side — a fixed thread count each: **200 samples at four threads on the rented Linux machine**, **150 samples at two threads on the Creator's Mac**. That gave 4.2 to 4.9 seconds a frame on Linux and 4.9 to 5.5 on the Mac. The watcher polled every millisecond and sent the signal between 0.000013 and 0.001426 seconds after the tenth image file appeared.*

***What came back for Blender 4.5.*** *On the **rented Linux machine**, where a complete image in the control run was 93602 bytes: the tenth image was 93602 bytes in all ten `SIGINT` runs, and **0 bytes in all ten `SIGTERM` runs and all ten `SIGKILL` runs**. One line word for word, the first `SIGTERM` run, with the Blender log left out of the quote: `RUN {"workdir": "w_4.5", "sig": "SIGTERM", "tenth_seen_at": 56.57370328903198, "signal_sent_at": 56.57418894767761, "exit": -15, "n_images": 10, "images": [["Image0001.png", 93602], … ["Image0009.png", 93602], ["Image0010.png", 0]], "videos": [["vid0001-0020.mp4", 48]]}`. On the **Creator's Mac**, where a complete image was 93588 bytes: 93588 bytes in all ten `SIGINT` runs; short in all ten `SIGTERM` runs, at 0, 16384, 0, 16384, 0, 0, 0, 0, 40960 and 0 bytes in run order; and short in all ten `SIGKILL` runs, at 0, 0, 16384, 0, 0, 16384, 4096, 0, 0 and 0 bytes.*

***The controls.*** *In every one of the sixty runs the first nine images were at the full size and exactly ten images were on disk, so the render was stopped rather than finished; exit statuses were 1, -15 and -9 for the three signals throughout. The full size itself comes from a complete uninterrupted run of the same file on the same machine, which left all twenty images at one size. And the test could come out the other way, because it did: `SIGINT` gave ten runs with every image whole, in this version and in the other three, on both machines.*

***How this sits beside the two runs above, without picking between them.*** *On `SIGINT` all three agree — nobody has seen a short image after a `SIGINT`, and this batch did not either in eighty tries across the four versions. On `SIGKILL` this batch **matches the finding above**: a 0-byte tenth image. On `SIGTERM` it does **not** match the finding above, which records all ten full, and does match the citation check's Mac run, which records a 0-byte tenth image after `SIGTERM`. It does not match the citation check's Linux runs, which found every image full after every signal. Nothing here says which earlier run was right. What is on the record is that the earlier runs were one run per signal and this one was ten, and that this batch states a condition theirs do not — how quickly the signal followed the file appearing. Added by the second batch of hand tests.)*

**(f) A second run over the same range.** Starting from what the `SIGKILL` run left on the rented Linux machine — a 48-byte video and ten PNGs, the tenth of them empty — the same file was rendered again over the same range and allowed to finish. Afterwards the video was **6657 bytes** with a new modification time, reading back `frame_duration: 20`, and all twenty PNGs were 93611 bytes with new modification times, including the nine the first run had already completed (`Image0001.png` moved from `14:47:49` to `14:54:52`).

**(g) Whether a cancel handler fires when the stop comes from outside.** With handlers registered on `render_cancel`, `render_complete` and `render_write`, on the rented Linux machine, `SIGINT` sent once ten images were written:

> `CANCELWATCH render_write frame 10 | video sizes: [('vid0001-0020.mp4', 48)]`
> `CANCELWATCH render_cancel fired at frame 1`
> `CANCELWATCH copied /root/ht-frames/e10_4.5_SIGINT/vid/vid0001-0020.mp4 4032 -> /root/ht-frames/e10_4.5_SIGINT/copy/vid0001-0020.mp4`
> `CANCELWATCH render call returned`
> `RUNSIGNAL copy and final file identical byte for byte: True`

The file was already at its full 4032 bytes at the moment the cancel handler fired, and the copy taken then was the same file, byte for byte, as the one left behind. With `SIGTERM`, no `render_cancel` line appeared at all, no copy was taken, and the file left behind was 48 bytes reading back `frame_duration: 0`. `render_complete` never appeared in either run. The control that the handlers were working is the `CANCELWATCH render_write frame N` line, which appeared once per frame in both runs.

**(h) The container changes the number of bytes left, not the frame count.** On the rented Linux machine, the same stop-at-the-last-frame run in each of the ten containers 4.5 offers — `['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME']` — left: MPEG4 48 bytes, MKV 0, WEBM 262, AVI 0, DV no file at all, FLASH 0, MPEG1 0, MPEG2 0, OGG 0, QUICKTIME 36. Every one of them read back `frame_duration: 0`. Two of the ten never reached the interruption and said so: WEBM printed `RuntimeError: Error: Could not initialize streams, probably unsupported codec combination` and DV printed `RuntimeError: Error: Render width has to be 720 pixels for DV!`, both exiting 0. The control — the same eight containers rendered to the end — produced files the reader could read (MPEG4 2243 bytes / 4 frames, MKV 2035 / 4, AVI 7232 / 4, FLASH 3632 / 4, MPEG1 4096 / 2, MPEG2 6144 / 2, OGG 5407 / 1, QUICKTIME 2177 / 4), while WEBM and DV failed the same way whether interrupted or not.

**(i) The premise about one output at a time.** On the rented Linux machine:

> `Q6 media_type present: False`
> `Q6 file_format items: ['BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX', 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP', 'FFMPEG']`
> `Q6 file_format is_enum_flag: False`
> `Q6 set FFMPEG ok, file_format now: FFMPEG`
> `Q6 set PNG ok, file_format now: PNG`
> `Q6 is_movie_format: False`

So in 4.5 the render output's file format is a setting that holds one value at a time (`is_enum_flag: False`), `FFMPEG` is one of its values, and setting it to one value replaces the other. There is no `media_type` property on the render output's image settings in 4.5 — the same script printed `Q6 media_type present: True` with the items `['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']` in 5.0, 5.1 and 5.2.

**(j) Whether the File Output node can itself be set to write a video — it cannot, in 4.5.** On the rented Linux machine:

> `Q7 node format media_type present: False`
> `Q7 node file_format now: PNG`
> `Q7 set node file_format FFMPEG FAILED: TypeError bpy_struct: item.attr = val: enum "FFMPEG" not found in ('BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX', 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP')`

The same script in 5.0, 5.1 and 5.2 printed that the node does have a `media_type` listing three values including `VIDEO`, and that setting it to `VIDEO` came back `enum "VIDEO" not found in ('IMAGE', 'MULTI_LAYER_IMAGE')`. So in none of the four versions would the node take a video setting.

**(k) Overwrite and Placeholders can be changed with a video output in force.** On the rented Linux machine, all four versions printed the same six lines; for 4.5:

> `Q9 IMAGE file_format: PNG | is_movie_format: False`
> `Q9 IMAGE use_overwrite was True now False`
> `Q9 IMAGE use_placeholder was False now True`
> `Q9 VIDEO file_format: FFMPEG | is_movie_format: True`
> `Q9 VIDEO use_overwrite was True now False`
> `Q9 VIDEO use_placeholder was False now True`

This is about the settings, not about how the boxes are drawn in a panel. What the panel draws was not tested — see the note under the hand-test list below.

**(l) Autosplit: the file already split away survives, the one being written does not.** On the rented Linux machine only. Two set-ups failed first and are recorded rather than quietly replaced: container `AVI` with codec `NONE` gave `Error: Could not initialize streams, probably unsupported codec combination` and a 0-byte file, and container `AVI` with codec `HUFFYUV` worked but was far too slow to reach 2 GiB. What was used: container `QUICKTIME`, codec `QTRLE`, 3840 by 2160, `use_autosplit = True`, frame range 1–4000, and the **Workbench** engine rather than Cycles so that a couple of thousand frames could be reached at all — that last change is a departure from the set-up used everywhere else here and is recorded because the engine is not what this test is asking about. The render was watched and killed with `SIGKILL` the moment a second video file appeared. What came back:

> `RUNSPLIT 1912.7 s: a second video file has appeared: ['vid_0000001-4000.mov', 'vid_0010001-4000.mov'] -- sending SIGKILL to pid 139544`
> `RUNSPLIT exit code: -9`
> `RUNSPLIT files left: [('vid_0000001-4000.mov', 2006829677), ('vid_0010001-4000.mov', 36)]`

Readback: the first file `loaded ok | frame_duration: 2179 | size: (3840, 2160) | fps: 24.0`; the second file `loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. The other three versions gave the same two files with the same two names and the same shape of answer. The control that the split really happened is the file count: one file would have meant the condition never bit, and there were two. **The Creator's Mac was not used for this test**, because each run writes over two gigabytes and takes between thirty and sixty-seven minutes; macOS is untested for autosplit and that is a gap rather than an answer.

**What these tests do not settle.** Nothing was run inside Blender's own window, so nothing here says what pressing Esc or clicking the status bar's cancel button leaves. Nothing was run on Windows, because no Windows machine has run it. The video reader used was Blender itself, so "the reader gave 9 frames" is what Blender's movie-clip reader made of the file and not what every player would make of it. And every run used one scene, one codec and one frame rate.

### Related 4.5 release notes

These are the only 4.5 release-note entries that touch this ground. None of them describe interruption.

> "Image Format Settings in the Render Output, File Output, and Image Saving now has a Media Type enum that specifies the general type of the file format, be it Image, Multi-Layer EXR, or Video."

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/ — read 2026-07-30.

A second, separate finding sits inside that note, and it gets its own label. **LABEL: not in the sources** — the `media_type` property is nowhere in the 4.5 API reference. The record of looking: I searched the whole `api_4.5` folder and its general index for "media_type" and for "media", and found neither. So the release note announces it and the API reference does not document it.

> "Video rendering to FFV1 codec now supports 10, 12, and 16 bits output."

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/rendering/ — read 2026-07-30.

> "Blender corrupts videos saved in the image editor"

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/bugfixes/ — read 2026-07-30. This is a fixed bug about saving a video from the Image Editor, not about a render being interrupted. I include it only so the reader knows I found it and set it aside.

### Not covered

- The whole of the question's premise — a single render writing a video and images together — is undocumented. Everything above is assembled from statements about each output on its own.
- Nothing here settles whether an interrupted video file is readable, partly readable, or empty.
- Nothing here settles what "all the images have been written" means in terms of file validity; the sources never say an image file on disk is a finished image.
- Autosplit Output means a video render can produce more than one file — but only on the two conditions the page states. The page reads: "If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size" (same output.html page). So it takes the setting being enabled *and* the first file passing 2GiB. What an interruption leaves when autosplit has already produced earlier files is not described anywhere, and I did not chase it. *(Citation check, 2026-08-09 — Opus 5. This first read "a long video render can produce more than one file", which drops both conditions and swaps length for size. Corrected to the page. Changed by the citation check.)*
- Nothing here covers the Video Sequencer as a source of the video, only the render output.

### Outside the interface

The sources do not say. I found no statement anywhere in the manual, the API reference or the release notes that distinguishes what an interrupted render leaves on disk when Blender is driven from the command line or in background mode from what it leaves when a person clicks. Two of the command-line switches I read say nothing about interruption:

> "-b, --background"
> "Run in background (often used for UI-less rendering)."

> "-a, --render-anim"
> "Render frames from start to end (inclusive)."

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

The API reference for the render operator is equally silent:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, layer='', scene='')"
> "Render active scene"
> "animation (boolean, (optional)) – Animation, Render files from the animation range of this scene"

Source: API reference, https://docs.blender.org/api/4.5/bpy.ops.render.html — read 2026-07-30.

One point worth flagging as a difference in *how the interruption happens* rather than in *what it leaves*: the manual documents an in-window way to stop a running render —

> "Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate. The task can be aborted by clicking the cancel button ()."

Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html — read 2026-07-30.

— and documents no equivalent from outside the window. I searched for signal handling and for Ctrl-C in the command-line documentation and found nothing on either.

Two entries on the command-line arguments page do use the word abort, and here is the whole of what they say:

> "--disable-abort-handler"
> "Disable the abort handler."

> "--disable-crash-handler"
> "Disable the crash handler."

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

Neither says what an abort handler is, neither mentions a render, and neither says anything about output files. They are recorded here because the page was read and they were on it, not because they answer anything.

*(Citation check, 2026-08-09 — Opus 5.* The two entries above were missing. The "Pages read" list at the foot of this document names the command-line page as read for "background, render-anim, **abort-handler**, examples", and then this paragraph reported finding nothing, so a reader could not tell whether the abort entry said something relevant that went unrecorded. It does not; it says one sentence and that sentence is now quoted. The earlier audit note here sent this back to a fresh researcher; that is no longer needed, because the answer was on the page. Changed by the citation check.*)*

*(Hand tests, 2026-08-09 — Opus 5.* This line is now answered by observation, and the observation is that everything in the section above was done from outside the interface. Every run recorded in **Hand tests, 2026-08-09 — observed** started Blender with `-b --factory-startup` and no window, on the Creator's Mac and on the rented Linux machine, which has no display server and cannot open one. So the behaviour recorded there **is** the driven-from-outside behaviour for Blender 4.5: images arriving one per frame, a video file that holds 48 bytes until the end, a `SIGINT` that closes the file and fires `render_cancel`, and a `SIGTERM` or `SIGKILL` that does not. What is still not settled is the other direction — whether a stop from **inside** the window leaves the same thing. That needs eyes on a window and was dropped this round; the reason is under the hand-test list below. The sources' silence, recorded below, stands as a statement about the documentation.*)*

*(Audit note, 2026-08-09 — Opus 5.* The original wording called `-b` and `-a` "the two relevant switches", which cannot be right with an abort switch read on the same page; that phrase is corrected above.*)*

*(Citation check, 2026-08-09 — Opus 5.* An earlier audit note here called this the largest hole in the document and sent the standing line "Outside the interface" back to a fresh researcher, because the abort-handler entry had been read and not recorded. That is now done, above, from the page itself: the entry is one sentence and it carries nothing about a render or its output. The line is no longer open on that account. What an abort handler *is* remains undescribed in the 4.5 sources, and that — not this standing line — is what goes back to a fresh researcher. Changed by the citation check.*)*

### Operating system

The sources do not say. I found no statement that any of this behaves differently on one operating system than another. The nearest the manual comes to the topic is about playback and codec availability, not about interruption:

> "Codecs are used to encode and decode the movie, and so must be present on both the encoding machine (Blender) and the target machine."

> "Each has advantages and disadvantages, and compatibility with different players on different operating systems."

Source: manual, https://docs.blender.org/manual/en/4.5/files/media/video_formats.html — read 2026-07-30.

Read those words closely. The "Each" in the second one is codecs, not files: what the page says is that codecs differ in their compatibility with different players on different operating systems, and that a codec has to be present on both the encoding and the target machine. To get from there to "a finished file may play differently on different systems" a reader has to join a file to the codec it was written with — a short step, but a step. Either way the page says nothing about what an unfinished file contains.

*(Citation check, 2026-08-09 — Opus 5.* This first read, flatly, "That says a finished file may play differently on different systems." The quoted sentence is about codecs. Narrowed to what the words carry, with the step named. Changed by the citation check.*)*

*(Hand tests, 2026-08-09 — Opus 5.* Two of the three operating systems are now answered by observation, and they are answered separately. Every test in **Hand tests, 2026-08-09 — observed** was run twice where it was run on both machines — once on the Creator's Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, and once on the rented Linux machine, an Ubuntu 22.04.5 LTS machine on Vast.ai with an NVIDIA GeForce RTX 5060 and no display server. The two agreed on everything except one number: the video file left by a `SIGINT` partway through read back as **9** frames on Linux and **10** on the Mac, with ten images on disk in both. Everything else — the 48 bytes held until the end, the complete set of images, the 48-byte file after a `SIGTERM` or `SIGKILL`, the whole last picture — was the same on both. **Windows is untested**, not answered: no Windows machine has run it.*)* **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. One more thing the two machines now say, from the 248 runs recorded at item (e) above. **They agree on the shape and differ on the detail.** On both, a `SIGINT` left the last image whole every time and a `SIGTERM` or `SIGKILL` left it short every time. Where they part is what "short" was: on the rented Linux machine every one of the eighty short files was exactly **0 bytes**, while on the Creator's Mac sixty-nine were 0 bytes and eleven were a whole number of 4096-byte blocks — 4096, 16384, 24576, 40960 or 73728 — all far short of a complete image. **[Recount, 2026-08-09 — this sentence used to say seventy-two and eight, and that was a miscount. Counting the run-by-run figures themselves — this version's ten runs per signal as printed on this page, and the other three versions' in the hand-test record these findings were written from — gives sixty-nine empty and eleven block-sized. A recount of figures already on the record: nothing was re-run, and no new fact is added.]** So the two operating systems gave the same answer to the question and different-looking leftovers. Windows is still untested.*)*

---

## Where I looked

**Whole-source searches** (case-insensitive, across the 4.5 manual, the 4.5 API reference and the 4.5 release notes, plus the Cycles add-on files where noted):

- Interruption words: `interrupt`, `cancel`, `cancelling`, `canceling a render`, `cancel the render`, `stop rendering`, `stop the render`, `abort`, `kill`, `terminate`, `SIGINT`, `Ctrl-C`, `crash`, `power fail`, `power loss`, `power supply`.
- File-integrity words: `corrupt`, `unplayable`, `playable`, `truncat`, `damaged`, `broken file`, `incomplete`, `unfinish`, `partial`, `zero-byte`, `empty file`, `0 bytes`, `not finished`, `does not finish`, `stopped part`, `half-written`.
- Closing-and-finalising words: `finalize`, `finalise`, `closing the file`, `closed`, `flush`, `trailer`, `moov`, `muxer`.
- Recovery words: `recover`, `resume`, `left off`, `already rendered`, `salvag`.
- Output words: `File Output`, `Overwrite`, `Placeholders`, `is_movie_format`, `use_overwrite`, `use_placeholder`, `autosplit`, `media_type`, `media`, `ffmpeg`, `video`, `encod`, `movie`.
- Handler names: `render_cancel`, `render_complete`, `render_write`, `render_init`, `render_pre`, `render_post`, `render_stats`.

**Pages read in full or in the relevant part:**

- https://docs.blender.org/manual/en/4.5/render/output/introduction.html
- https://docs.blender.org/manual/en/4.5/render/output/animation.html (whole page)
- https://docs.blender.org/manual/en/4.5/render/output/properties/output.html (Output, Encoding, Audio and Tips sections)
- https://docs.blender.org/manual/en/4.5/render/output/index.html
- https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html (whole page)
- https://docs.blender.org/manual/en/4.5/compositing/types/output/composite.html (whole page)
- https://docs.blender.org/manual/en/4.5/compositing/types/output/index.html
- https://docs.blender.org/manual/en/4.5/files/media/video_formats.html
- https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html (background, render-anim, abort-handler, examples)
- https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html
- https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html (the "Read Render Layers" entry, which mentions "recovering some information from a failed render" — it is about reading render layers from cache, not about output files, so I set it aside)
- https://docs.blender.org/manual/en/4.5/troubleshooting/recover.html and https://docs.blender.org/manual/en/4.5/troubleshooting/crash.html (both are about recovering the blend-file after a crash, not about render output)
- https://docs.blender.org/api/4.5/bpy.app.handlers.html
- https://docs.blender.org/api/4.5/bpy.ops.render.html
- https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html (`is_movie_format`, `use_overwrite`, `use_placeholder`, `use_file_extension`)
- https://docs.blender.org/api/4.5/bpy.types.FFmpegSettings.html (all properties, including `use_autosplit`: "Autosplit output at 2GB boundary")
- https://docs.blender.org/api/4.5/bpy.types.ImageFormatSettings.html (full property list)
- https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html
- https://developer.blender.org/docs/release_notes/4.5/rendering/
- https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/
- https://developer.blender.org/docs/release_notes/4.5/compositor/
- https://developer.blender.org/docs/release_notes/4.5/bugfixes/ (searched for corrupt, cancel, abort, stop, interrupt)

**Pictures opened:** `_images/render_output_properties_output_video-encoding-panel.png` and `_images/render_output_properties_output_panel.png`, both on the Output properties page. Reported above.

**Cycles' own files:** I searched `cycles_4.5` for `ffmpeg`, `movie`, `video`, `file_format`, `filepath`, `output`, `cancel`, `abort`, `break` and `write`. The only hits are in `operators.py` and `engine.py`. `operators.py` holds three operators — Use Nodes, Denoise Animation and Merge Images — and two of them, Denoise Animation at line 48 and Merge Images at line 136, carry an `output_filepath` and read and write image files of their own. `engine.py` holds a function named `render_frame_finish`, which returns early if there is no session and otherwise hands `engine.session` to a Cycles function of the same name; the file does not say what that does. Nothing in the Cycles files touches video containers or the writing of the render output. I have nothing to report from them.

*(Citation check, 2026-08-09 — Opus 5.* Two corrections against the files. The record named one operator where two carry an `output_filepath`, and it said `render_frame_finish` "ends a Cycles render session", which the file does not say — it shows only the function and what it passes on. Both corrected to the files. Changed by the citation check.*)*

*(Audit note, 2026-08-09 — Opus 5.* The closing clause here first read "…the writing of the render output, which Blender itself does." That last clause says who does the writing, which is a claim about how the work is divided between Cycles and Blender, and nothing quoted anywhere on this page carries it. Removed. The honest finding is the absence: these files say nothing on the subject.*)*

**Ruled out and why:** the Video Sequencer pages, because the question is about a render's own output rather than about encoding an existing sequence; the Image Editor save pages, because the question is about an animation render rather than a manual save; the fluid and physics cache pages, which matched "resume" and "pause" but concern baking simulations.

**I did not use the web.** Everything above came from the snapshot in `Blender Sources/`.

---

## Things a person with Blender 4.5 open could settle in a few minutes

*(Hand tests, 2026-08-09 — Opus 5. This list was seven items. Six of them were run on 2026-08-09 and their answers are in **Hand tests, 2026-08-09 — observed** above; they are struck from the list here so nobody runs them twice. The researcher's numbering is kept so the audit notes still point at something. What is left below is what is genuinely still open.)*

**Answered on 2026-08-09, and removed from this list:** the researcher's items 1 and 3 (what an interruption leaves, mid-render and from outside the window) — see (d) above; item 2 (the exact case the question asks about) — see (b); item 4 (whether the images reach the disk as the render goes) — see (a); item 5 (whether Blender writes to the video after a cancel fires) — see (g); item 6 (whether a File Output node accepts a video format in 4.5) — see (j).

**Answered on Linux, untested on the Mac — Autosplit.** What an interruption leaves when the file has already split into more than one was run on the rented Linux machine and is at (l) above: the file already split away held every frame that went into it, and the one being written was left at 36 bytes holding none. It was not run on the Creator's Mac, because each run writes over two gigabytes and takes between thirty and sixty-seven minutes, so macOS is untested for this one.

**Still open — anything needing eyes on a window.** Three things could not be settled this round and the reason is the same for all three: the rented Linux machine has no display server, so Blender cannot open a window there at all, and the Creator ran nothing. So it is not known what pressing Esc or clicking the status bar's cancel button leaves behind, as against sending a signal from outside; nor how the Overwrite and Placeholders boxes are *drawn* when the output is a video, which is what the two screenshots above were being read for. The neighbouring question — whether those two settings can be held and changed with a video output in force — was answered from a terminal instead, at (k) above, and it is a different question from how the panel draws them.

**Still open — Windows.** Nothing here was run on Windows, because no Windows machine has run it. macOS and Linux are each answered on their own above; Windows is untested rather than answered.

---

2026-08-09 — Opus 5 (Claude Code) — Frames, Blender 4.5
