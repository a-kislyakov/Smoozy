# Frames — later questions, second pass (Blender 5.2)

Sources consulted, all read on **2026-07-30** (the snapshot date given to me):

- Blender 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` folder)
- Blender 5.2 Python API documentation (`Blender Sources/api_5.2`)
- Blender 5.2 release notes (`Blender Sources/release_notes_5.2`)
- Cycles' own files for 5.2 (`Blender Sources/cycles_5.2`)
- The requirements page (`Blender Sources/requirements/requirements_2026-08-01.txt`)

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I ran no helper agents. Everything below is my own reading.

---

## 12. When one render writes one video and separate image files over the same frame range, what does an interruption leave if all the images have been written but the video has not finished closing?

### How I read the question

The question was ambiguous about how one render comes to write both kinds of file, so I state my reading plainly.

I answer the reading where **one animation render** has two output paths active at once: the scene's own Output panel set to save a video, and one or more Compositor **File Output** nodes saving separate image files, both covering the same frame range. In 5.2 that is the only combination the manual describes, because the File Output node's media types are only image ones — **documented**, below — and because a scene has a single output media type — **inferred**, see the note after the quotes:

> "Media Type
>
> The type of media to save.
>
> Image:
>
> Saves each input as a separate image file. …
>
> Multi-Layer EXR: …
>
> Video:
>
> Encodes each frame into a video container."

— manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html (read 2026-07-30)

> "Media Type
>
> Determines how multiple inputs are stored:
>
> Image:
>
> Saves each input as a separate image file. Each input can use its own image format settings.
>
> Multi-Layer EXR:
>
> Saves all inputs together in a single multi-layer OpenEXR file."

— manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html (read 2026-07-30)

*(Citation check, 2026-08-09 — Opus 5.* Both quotes are at their addresses word for word. They carry the node half of the reading outright: the node's list has two entries and neither is video. They do not carry the scene half. "The type of media to save" with three named values becomes "a scene has a **single** output media type" only if the reader supplies that one value is held at a time — a step the page does not take. 5.1's first pass says exactly this about its own identical wording and labels the narrowing inferred; this page said it flatly, and the two versions are now brought into line. The hand test at **(i)** below is what settles it outright. Changed by the citation check.*)*

The other reading, which I do not answer, is that "one render" means two separate render runs over the same frames. Nothing on the sheet or in the sources tells me which was meant.

I also take the question's premise as given: all the image files are already on disk, and the video file has not been finished off.

### Answer

*(Hand tests, 2026-08-09 — Opus 5. The question was put to two running computers on 2026-08-09, and most of what this page leaves open is now settled by observation. The observed answer is in its own section below, **Hand tests, 2026-08-09 — observed**. The labels below still stand, because they are statements about the documentation and the documentation has not changed.)*

**Label: mixed — see each part.**

**Part one: the image files. Documented (that they are written to disk as the render runs), inferred (that an interruption leaves them alone).**

The File Output node writes its images while the render is running, one file per frame, not in one batch at the end:

> "The File Output node writes one or more images to disk during rendering or compositing. Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence."

— manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html (read 2026-07-30)

The Python API says the same about the moment of writing, in its list of things a script can be told about:

> "bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

— Python API, https://docs.blender.org/api/5.2/bpy.app.handlers.html (read 2026-07-30)

That already-written frames stay on disk after an interruption is stated by the manual, but for frames written by the **Output panel** to an image format, not for File Output node files:

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

— manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html (read 2026-07-30)

**The step I took (this is why it is inferred, not documented):** the manual says image files that have been written survive a stopped render, and separately says the File Output node writes image files to disk during the render. I joined those two to conclude that the node's image files also survive. No page in the sources says that about the node's files, and no page says Blender deletes or rewrites them either — I searched for such a statement and found none (terms listed in the record below).

**Part two: the video file. Documented as far as "a file is left and it may be unplayable"; the exact state of that file is not in the sources.**

The one statement in the whole 5.2 manual about a video file left behind by a render that hit trouble is this:

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

— manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html (read 2026-07-30)

One thing follows from those words without me adding anything: the frames already encoded into the file cannot be picked up, because the recovery the manual names is re-rendering every frame from the start.

A second thing does **not** follow without a step, and I have separated it out. That a file exists on disk after the interruption is not in the quote. "The file might become unplayable" says what might happen to a file; getting from there to "a file is there to be found afterwards" needs the reader to supply that the thing which becomes unplayable is still on disk. That step is **inferred**, not documented — and what settles it outright is the hand test at **(b)** below, where a 48-byte file was on disk on both machines.

*(Citation check, 2026-08-09 — Opus 5.* This passage first said "Two things follow from those words without me adding anything: a file exists (something must exist to 'become unplayable'), and the frames already encoded into it cannot be picked up." The second holds on the quoted words; the first needs a step, and the parenthesis is that step written out rather than a reading of the quote. Split, with the step named and pointed at the observation that really carries it. Changed by the citation check.*)*

The manual makes the matching positive claim only for image formats, never for video:

> "Images can also be used for rendering animations which has a couple advantages. For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

— manual, https://docs.blender.org/manual/en/5.2/render/output/introduction.html (read 2026-07-30)

and

> "Restart the rendering from the place (the frame) where any problem occurred."

— manual, listed as an advantage of rendering to still images first, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html (read 2026-07-30)

**What is left, then, on the sources' own words:** the image files, and — if a video file is there at all, which the sources do not say and observation (b) does — one video file that the manual says might be unplayable and whose content cannot be used as a starting point for a resumed render.

**Part three: how many video files. Inferred.**

If Autosplit was on and the render had already passed 2 GiB, more than one video file would be on disk at the moment of the interruption:

> "Autosplit Output
>
> If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

— manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html (read 2026-07-30)

**The step I took:** the manual says splitting happens during the render, once a size is reached, so files earlier in the sequence must already be on disk before the render ends. It does not say what state those earlier files are in after an interruption, and I do not claim they are playable.

**Part four: what is not in the sources at all.**

None of these is answered anywhere in the 5.2 manual, Python API documentation, release notes, or the Cycles files:

- Whether Blender writes a proper container ending for the video when a render is stopped, and whether stopping it on purpose differs from the machine going down. The manual's single sentence covers both under "if a problem occurs" and hedges with "might".
- Whether the partial video file has a size at all, whether it holds the frames rendered so far, and whether any player or tool can read them out.
- Whether Blender deletes, renames, or leaves the partial video file.
- Whether the last frame in the video matches the last image file on disk — that is, whether the two outputs stop at the same frame.
- Whether the audio track being muxed in changes the outcome.

### Not covered

This answer does not settle the on-disk state of the unfinished video: its size, whether a player can open it, whether the frames inside it are recoverable by anything, or whether a purposeful stop leaves it in a better state than a crash. It does not settle whether the images and the video stop at the same frame. It does not settle what happens to the File Output node's images written for the frame that was in progress when the interruption came — the question's premise takes all images as written, so I did not chase the in-progress case. It says nothing about repairing such a file, because the sources say nothing about that.

### Outside the interface

*(Hand tests, 2026-08-09 — Opus 5. This line is now answered by observation. Every run recorded in **Hand tests, 2026-08-09 — observed** below started Blender with `-b --factory-startup` and no window, on the Creator's Mac and on the rented Linux machine, which has no display server and cannot open one. So what is recorded there is the driven-from-outside behaviour for Blender 5.2, and it also answers the question this section leaves open — how a background render is interrupted and what the video file looks like afterwards: with `SIGINT` Blender prints `Sent an internal break event`, fires `render_cancel`, and leaves a file the reader can read; with `SIGTERM` or `SIGKILL` it leaves 48 bytes the reader finds nothing in. What is still not settled is the other direction, whether a stop from inside the window leaves the same thing; that needs eyes on a window and was dropped this round.)*

The sources describe running renders with no window — `blender -b file.blend -a` for a whole animation, with `-F` able to force the format, listing `FFMPEG` among the values (manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html and https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30). Nothing in them says how a background render is interrupted, or what the video file looks like afterwards.

The sentence I lean on — "If a problem occurs while rendering, the file might become unplayable" — is written with no reference to how the render was started, so nothing in the sources limits it to someone clicking. Equally, nothing in the sources extends it to a background run in so many words. The Python API documents that a cancelled render is a distinct event a script can be told about:

> "bpy.app.handlers.render_cancel
>
> on canceling a render job. Accepts one argument: the scene data-block being rendered"

— Python API, https://docs.blender.org/api/5.2/bpy.app.handlers.html (read 2026-07-30)

but it does not say what that does to any file. The only interface-specific note I found about video rendering is about messages, not files:

> "When you view the System Console, you can see some of the output of the encoding process. You will see even more output if you execute Blender as blender -d."

— manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html (read 2026-07-30)

**So: the sources do not say whether this behaves the same when Blender is driven from outside its own window.**

### Operating system

*(Hand tests, 2026-08-09 — Opus 5. Two of the three are now answered by observation, and separately. Every test below that was run on both machines was run twice — once on the Creator's Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, and once on the rented Linux machine, Ubuntu 22.04.5 LTS on Vast.ai with an NVIDIA GeForce RTX 5060 and no display server. For Blender 5.2 the two agreed on every result. **Windows is untested**, not answered: no Windows machine has run it.)*

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. The 248 runs recorded at item (e) below put one qualification on "the two agreed on every result". **They agree on the shape and differ on the detail.** On both machines a `SIGINT` left the last image whole every time and a `SIGTERM` or `SIGKILL` left it short every time. What differed is what "short" was: on the rented Linux machine every one of the eighty short files, across all four versions, was exactly **0 bytes**, while on the Creator's Mac sixty-nine were 0 bytes and eleven were a whole number of 4096-byte blocks — 4096, 16384, 24576, 40960 or 73728 — all far short of a complete image. **[Recount, 2026-08-09 — this sentence used to say seventy-two and eight, and that was a miscount. Counting the run-by-run figures themselves — this version's ten runs per signal as printed on this page, and the other three versions' in the hand-test record these findings were written from — gives sixty-nine empty and eleven block-sized. A recount of figures already on the record: nothing was re-run, and no new fact is added.]** In Blender 5.2 itself, the Mac's three non-empty short files were 73728, 24576 and 24576 bytes, all in `SIGTERM` runs. Windows is still untested.*)*

The sources do not distinguish operating systems on this point. Two nearby statements are worth recording, because they bear on whether video output exists at all on a given machine:

> "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

— manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html (read 2026-07-30)

> "Because codecs are required for both encoding and decoding, they must be available on both the system creating the file and the device playing it."

— manual, https://docs.blender.org/manual/en/5.2/files/media/video_formats.html (read 2026-07-30)

Beyond that, **the sources do not say** whether an interrupted video render leaves the same thing behind on Windows, macOS and Linux.

---

## Hand tests, 2026-08-09 — observed

Eleven tests were written for this question and run by hand on **2026-08-09**, the same tests in all four researched versions. The whole list, with the exact commands and the exact Python, is `Audits/Blender Research/Frames/Hand tests — later questions.md`. What follows is what came back for Blender 5.2. It is the same record as in `Preparation/Blender Research/Findings/5.2/Frames — later questions.md`, written here too so that neither page points a reader the wrong way.

**The two machines, in full.** *The rented Linux machine* — rented on Vast.ai and reached over SSH, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, and no display server at all; Blender 5.2.0 at `/opt/blender/5.2/blender`. It was shared with other work while these ran. *The Creator's Mac* — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics; Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`. Neither machine speaks for the other. Every run used `--factory-startup` and a fresh throwaway `BLENDER_USER_RESOURCES` folder.

**A word about the reader.** Neither machine has `ffprobe` or `ffmpeg`, so every video file was read back by loading it into Blender as a movie clip. That reader prints `loaded ok` even for a file holding nothing — a 48-byte file comes back as `loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. So each result records three things separately: whether the file exists, its size in bytes, and the frame count the reader gave.

**What was built, to what.** A blend-file made by script: the scene's own output with `media_type` set to `VIDEO` and then `file_format` set to `FFMPEG`, container MPEG-4, codec H.264, into one empty folder; the compositor given a Render Layers node feeding both a File Output node and the Group Output, the node writing PNGs into a second empty folder; Cycles on the processor. In 5.2 the compositor is reached at `scene.compositing_node_group`, the node's folder at `directory` and `file_name`, and the node writes nothing until an item is added to its `file_output_items` — its format's media type having defaulted to `MULTI_LAYER_IMAGE`. Rendered with `blender -b --factory-startup job.blend -a`, from outside Blender's own window.

*(Citation check, 2026-08-09 — Opus 5. **What this set-up does not say, and a repeater has to supply.** It does not say what is in the scene, how many samples, what resolution, or what frame rate. The citation check repeated these tests using the factory-startup scene as it stands, 128 samples, 640 by 360 for the eight-frame runs and 480 by 270 for the twenty-frame runs, and 24 frames a second — the last three read off this page's own readback lines, the first two guessed. Frames then rendered in about half a second where this run's took three or four, and that difference is what item (e) turns on. Anybody repeating this should write down the sample count. Changed by the citation check.)*

**(a) A complete run: the images arrive one per frame, the video arrives at the very end.** Frame range 1–8. On the rented Linux machine the PNGs appeared one at a time, `vid0001-0008.mp4` appeared at 0.707 s holding **48 bytes** and held 48 bytes throughout, the eighth PNG landed at 27.965 s with the video still at 48 bytes and still held open, and at 28.066 s the log reads `vids=[('vid0001-0008.mp4', 4583)] open=[]`. Readback: `loaded ok | frame_duration: 8 | size: (640, 360) | fps: 24.0`. On the Creator's Mac the same shape, ending at 4608 bytes and `frame_duration: 8`. **The two machines agreed.** This is Part one of the answer above put to a machine: the node's images do reach the disk one per frame while the render runs.

**(b) The state the question describes.** A handler on `render_write` printed the state at every frame and killed its own process the moment it fired for the last frame. On the rented Linux machine, `SIGKILL`:

> `KILLATLAST render_write frame: 8 | images: ['ImageImage0001.png', … , 'ImageImage0008.png'] | video: [('vid0001-0008.mp4', 48)]`
> `KILLATLAST sending SIGKILL to self 62206`
> `exit status of blender: 137`

`KILLATLAST render call returned` was not printed, so the stop landed where it was aimed. Left on disk: eight PNGs of 367544 bytes each and a **48-byte** video reading back `frame_duration: 0`. `SIGTERM` gave exit status 143 and the same. On the Creator's Mac, both signals gave the identical result, its PNGs being 368005 bytes. **The two machines agreed.** So Part two's reading of the manual's warning — that "a file exists" — holds here in the narrow sense that a file is on disk, and what is in it is nothing the reader can read.

**(c) The last image was whole.** Pixels compared rather than bytes: `PIX RESULT: largest difference on any channel = 0.0` on both machines, the two files differing in size by nine bytes on Linux (367544 against 367535) and eight on the Mac.

**(d) Interrupted partway, three ways.** Frame range 1–20, the signal sent once ten PNGs were on disk. On the rented Linux machine, `SIGINT`: exit code 1, ten images, `[('vid0001-0020.mp4', 3967)]`, Blender printing `Sent an internal break event. Press ^C again to kill Blender` and `Fra: 11 | Remaining: 00:03.07 | Mem: 8M | Cancelled`, readback `frame_duration: 10`. `SIGTERM` (exit `-15`) and `SIGKILL` (exit `-9`): ten images and a 48-byte video reading back `frame_duration: 0`. On the Creator's Mac the same three, the `SIGINT` file being 3862 bytes and reading back `frame_duration: 10`. **The two machines agreed.** So a render stopped on purpose and one ended by the process being killed do **not** leave the same thing, which is the distinction the manual never draws.

**(e) Whether the last image is whole.** In 5.2 on the rented Linux machine all ten images were at the full size after each of the three signals. In Blender 4.5 the same `SIGKILL` run left its tenth image at 0 bytes, and in 5.0 at 155648 bytes against a full 254253.

*(Citation check, 2026-08-09 — Opus 5.* The 5.2 half reproduced exactly, on both machines. **The cross-version half is disputed** and is on the hand-test list: the citation check's own 4.5 and 5.0 runs on the rented Linux machine left every image at the full size after every signal, and the one cut-short image it saw was in 4.5 on the Creator's Mac after `SIGTERM`. Changed by the citation check.*)*

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. The test was run, as entry 12 of `Audits/Blender Research/Frames/Hand tests — later questions.md`: **ten runs for each of `SIGINT`, `SIGTERM` and `SIGKILL`, in all four versions, on both machines, 248 runs in all.** All three sets of runs now stand side by side and none is deleted.*

*The set-up numbers the earlier record left out, now written down: frames 1 to 20, 480 by 270, 24 frames a second, Cycles on the processor, four versions run side by side with a fixed thread count each — **200 samples at four threads on the rented Linux machine**, **150 samples at two threads on the Creator's Mac** — giving 4.2 to 4.9 seconds a frame on Linux and 4.9 to 5.5 on the Mac. The watcher sent the signal between 0.000013 and 0.001426 seconds after the tenth image file appeared.*

***For Blender 5.2.*** *On the **rented Linux machine**, a complete image being 253330 bytes: the tenth image was 253330 bytes in all ten `SIGINT` runs and **0 bytes in all ten `SIGTERM` runs and all ten `SIGKILL` runs**. On the **Creator's Mac**, a complete image being 254935 bytes: 254935 in all ten `SIGINT` runs; short in all ten `SIGTERM` runs, at 73728, 0, 0, 0, 24576, 24576, 0, 0, 0, 0 bytes in order; and **0 bytes in all ten `SIGKILL` runs**.*

***The controls.*** *Ten images on disk and the first nine at the full size in every run, so the stop really bit; exit statuses 1, -15 and -9 throughout; the full size taken from a complete uninterrupted run of the same file on the same machine. The other outcome is real and it happened: `SIGINT` gave ten whole images every time.*

***Beside the two runs above, without picking.*** *On `SIGINT` all three agree. On `SIGTERM` and `SIGKILL` this batch does not match the 5.2 half of the finding above, nor the citation check's repeat of it — both record all ten full in 5.2, and this batch found the tenth image short in every one of its forty `SIGTERM` and `SIGKILL` runs in 5.2 across the two machines. It does support the cross-version half, that a cut-short image file is a thing that happens, and it found it in 5.2 too. Added by the second batch of hand tests.)*

**(f) A second run over the same range**, on the rented Linux machine, from what the `SIGKILL` run left, allowed to finish: the video went from 48 bytes to **6799 bytes** with a new modification time, reading back `frame_duration: 20`; all twenty PNGs written again with new modification times, including the ten already complete.

**(g) Whether a cancel handler fires when the stop comes from outside.** On the rented Linux machine, `SIGINT`:

> `CANCELWATCH render_cancel fired at frame 1`
> `CANCELWATCH copied /root/ht-frames/e10_5.2_SIGINT/vid/vid0001-0020.mp4 3788 -> /root/ht-frames/e10_5.2_SIGINT/copy/vid0001-0020.mp4`
> `RUNSIGNAL copy and final file identical byte for byte: True`

The file was already at its full size when the cancel fired and nothing changed it afterwards. With `SIGTERM` no `render_cancel` line appeared at all and the file left behind was 48 bytes. `render_complete` never appeared in either run. This is the `render_cancel` handler the answer above quotes, watched rather than reasoned about.

**(h) The container changes the number of bytes left, not the frame count.** The ten containers 5.2 offers are `['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME']`. The same stop-at-the-last-frame run in each left: MPEG4 48 bytes, MKV 0, WEBM 264, AVI 0, DV no file at all, FLASH 0, MPEG1 0, MPEG2 0, OGG 0, QUICKTIME 36 — every one reading back `frame_duration: 0`. WEBM and DV never reached the interruption, printing `Could not initialize streams, probably unsupported codec combination` and `Render width has to be 720 pixels for DV!`. The control, the same containers run to the end, produced readable files for the other eight.

**(i) The premise about one output at a time.** On the rented Linux machine: `Q6 media_type present: True`, `is_enum_flag: False`, items `['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`; with it set to `VIDEO`, `Q6 set PNG FAILED: TypeError bpy_struct: item.attr = val: enum "PNG" not found in ('FFMPEG')`, and `is_movie_format: True` against `False` under the image setting.

**(j) A File Output node cannot be set to write a video.** This is the researcher's own test 6 below, which asked for `node.format.media_type = 'VIDEO'` to be tried. On the rented Linux machine:

> `Q7 node media_type items: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`
> `Q7 node media_type now: MULTI_LAYER_IMAGE`
> `Q7 set node media_type VIDEO FAILED: TypeError bpy_struct: item.attr = val: enum "VIDEO" not found in ('IMAGE', 'MULTI_LAYER_IMAGE')`
> `Q7 set node file_format FFMPEG FAILED: TypeError bpy_struct: item.attr = val: enum "FFMPEG" not found in ('OPEN_EXR_MULTILAYER')`

5.0 and 5.1 printed the same; Blender 4.5 has no such property on the node at all. So it is not accepted, in any of the four versions.

**(k) Overwrite and Placeholders can be changed with a video output in force**, in all four versions: `Q9 VIDEO file_format: FFMPEG | is_movie_format: True`, `Q9 VIDEO use_overwrite was True now False`, `Q9 VIDEO use_placeholder was False now True`. This is about the settings, not about how the boxes are drawn in a panel.

**(l) Autosplit: the file already split away survives, the one being written does not.** On the rented Linux machine only. Two set-ups failed first and are recorded rather than quietly replaced: container `AVI` with codec `NONE` gave `Error: Could not initialize streams, probably unsupported codec combination` and a 0-byte file, and container `AVI` with codec `HUFFYUV` worked but was far too slow to reach 2 GiB. What was used: container `QUICKTIME`, codec `QTRLE`, 3840 by 2160, `use_autosplit = True`, frame range 1–4000, and the **Workbench** engine rather than Cycles so that a couple of thousand frames could be reached at all — that last change is a departure from the set-up used everywhere else here and is recorded because the engine is not what this test is asking about. The render was watched and killed with `SIGKILL` the moment a second video file appeared. What came back:

> `RUNSPLIT 4038.7 s: a second video file has appeared: ['vid_0000001-4000.mov', 'vid_0010001-4000.mov'] -- sending SIGKILL to pid 139780`
> `RUNSPLIT exit code: -9`
> `RUNSPLIT files left: [('vid_0000001-4000.mov', 2012531972), ('vid_0010001-4000.mov', 36)]`

Readback: the first file `loaded ok | frame_duration: 2197 | size: (3840, 2160) | fps: 24.0`; the second file `loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. The other three versions gave the same two files with the same two names and the same shape of answer. The control that the split really happened is the file count: one file would have meant the condition never bit, and there were two. **The Creator's Mac was not used for this test**, because each run writes over two gigabytes and takes between thirty and sixty-seven minutes; macOS is untested for autosplit and that is a gap rather than an answer.

**What these tests do not settle.** Nothing was run inside Blender's own window, so nothing here says what pressing Esc or clicking the cancel button leaves. Nothing was run on Windows. The reader used was Blender itself. Audio was never muxed in, so the *Part four* question about an audio track is untouched. And every run used one scene, one codec and one frame rate.

## Record of where I looked

Searches were run over the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` unless a narrower scope is named.

**Words I searched for across all four sources at once:** `unplayable`, `playable`, `partially written`, `partial file`, `half-written`, `leftover`, `resume`, `resumed`, `re-render all frames`, `crash`, `power outage`, `closing the file`, `close the file`, `flush`.

**Words I searched for across the whole 5.2 manual:** `incomplete`, `corrupt`, `truncat`, `unfinished`, `finaliz`, `finalis`, `trailer`, `moov`, `container is`, `closed properly`, `write the index`, `mux`, `file output`, `Ctrl-C`, `SIGINT`, `terminate`, `kill the process`, `render job`, `stop the render`, `stop rendering`, `render progress`, `render directly to`, `directly to a video`, `directly to a movie`, `render out to a movie`.

**Words I searched for across the render, compositing, files and video-editing parts of the manual:** `cancel`, `abort`, `interrupt`, `encoder`, `encoding process`, `at the end of the render`, `when the render finishes`, `written to disk`, `stream`, `deletes the file`, `remove the file`, `discard … file`, `clean up`, `movie`, `video`, `ffmpeg`.

**Words I searched for across the Python API documentation:** `ffmpeg`, `is_movie_format`, `use_autosplit`, `autosplit`, `media_type`, `use_file_extension`, `use_overwrite`, `use_placeholder`, `movie`, `render_cancel`, `render_write`, `render_complete`, `render_init`, `render_post`.

**Words I searched for across the 5.2 release notes:** `video`, `ffmpeg`, `movie`, `mp4`, `encod`, `file output`, `render output`, `render anim`, `interrupt`, `cancel`.

**Words I searched for across the Cycles files:** `ffmpeg`, `movie`, `file_output`, `.mp4`, `mux`.

**What those searches turned up, and what they did not:**

- `unplayable` has exactly one hit in the entire 5.2 manual, on the Output properties page. That single sentence is the whole documented basis for what an interrupted video render leaves. There is no second statement anywhere.
- `finaliz` / `finalis` / `trailer` / `moov` / `close the file` / `closing the file` have **no** hits anywhere in the manual, API documentation, release notes or Cycles files that relate to writing a video. The manual never describes the moment a video file is closed off, so the question's own phrase "has not finished closing" has no counterpart in the sources.
- `partially written`, `partial file`, `half-written` — no hits anywhere.
- `Ctrl-C`, `SIGINT`, `terminate`, `kill the process` — no hits about stopping a render. Every `Ctrl-C` hit in the manual is the copy shortcut in some editor.
- `deletes the file`, `remove the file`, `clean up` — no hits in the render, files or compositing parts of the manual. Nothing says Blender tidies up a partial output.

**Pages I opened in full and what I ruled out:**

- https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — the Output panel, Encoding panel and the tips at the end. Carries the "unplayable" sentence, the media types, and Autosplit. This is the main source for the answer.
- https://docs.blender.org/manual/en/5.2/render/output/animation.html — Rendering Animations, including "Direct Approach", "Frame Sequence" and the hint about the computer turning off mid-render. Carries the "frames you have already rendered will still be in the output directory" sentence, which is about the Output panel's own image files.
- https://docs.blender.org/manual/en/5.2/render/output/introduction.html — carries the "canceled and resumed at the last rendered frame" sentence, stated for image file formats only.
- https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — the File Output node in full. Says it writes during rendering; says nothing about interruptions.
- https://docs.blender.org/manual/en/5.2/files/media/video_formats.html — containers, codecs, HDR, known limitations. Ruled out: it describes what the formats are, never what a half-written one contains.
- https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html and .../arguments.html — command-line rendering. Ruled out: no mention of interrupting a render or of the resulting files.
- https://docs.blender.org/api/5.2/bpy.app.handlers.html — the render handlers, including `render_cancel`, `render_write`, `render_complete`. Useful for when a frame is written; silent on files after a cancel.
- https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — checked `is_movie_format`, `use_overwrite`, `use_placeholder`. These describe settings, not interruption behaviour.
- https://docs.blender.org/api/5.2/bpy.types.FFmpegSettings.html — every encoding setting, including `use_autosplit` ("Autosplit output at 2GB boundary (default False)"). Nothing about interruption.
- https://docs.blender.org/api/5.2/bpy.types.ImageFormatSettings.html — `media_type` is one of `IMAGE`, `MULTI_LAYER_IMAGE`, `VIDEO`. Recorded because the File Output node's `format` is of this type, which is why I flagged a hand-check below.
- https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html — the node's properties. Nothing about interruption.
- https://docs.blender.org/api/5.2/bpy.ops.render.html — `bpy.ops.render.render` is marked "Undocumented, consider contributing." Nothing to take from it.
- https://docs.blender.org/manual/en/5.2/troubleshooting/recover.html and .../crash.html — searched for the word `render` in both; **no hits**. These pages are about recovering blend-files, not render output. Ruled out.

**Picture I opened:** `_images/render_output_properties_output_panel.png`, the Output panel screenshot on the Output properties page (manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html, read 2026-07-30). It shows the output path, the Overwrite, Placeholders, File Extensions and Cache Result checkboxes, File Format set to PNG, Color, Color Depth and Compression. **LABEL: documented**, for exactly that — those controls are visible in this picture, with **PNG** chosen. It shows nothing about interruptions, and I take nothing further from it.

*(Citation check, 2026-08-09 — Opus 5.* This read "those controls are what the panel offers with an image format chosen". The picture shows the panel with PNG chosen; whether the same controls appear with any other image format is not visible in it. Narrowed to PNG. Changed by the citation check.*)*

**Release notes for 5.2:** I read `rendering.md` (its whole Render Output section is two lines: "Saving render output can now be disabled." and a light-linking note), and searched `compositor.md`, `pipeline_io.md`, `core.md`, `sequencer.md` and `bugfixes.md` for `ffmpeg`, `movie`, `mp4`, `encod`, `render output`, `render anim`, `file output`, `interrupt` and `cancel`. Nothing there touches what an interrupted video render leaves. The only nearby entries are a new independent File Extension checkbox on the File Output node (https://developer.blender.org/docs/release_notes/5.2/compositor/) and stereoscopic and panoramic metadata now being exported to movies (https://developer.blender.org/docs/release_notes/5.2/pipeline_io/). Both read 2026-07-30.

**Cycles' own files:** I searched `cycles_5.2` for `ffmpeg`, `movie`, `file_output`, `.mp4` and `mux` — **no hits at all**. The folder holds `__init__.py`, `camera.py`, `engine.py`, `operators.py`, `osl.py`, `presets.py`, `properties.py`, `ui.py`, `version_update.py`, `maketx.py`, a `shader` folder, a `license` folder, and a `source` folder containing only `kernel` and `util`. Ruled out: nothing in these files concerns writing video or image output files, so they cannot answer this question. **Label for that: observed** — I searched the supplied files and found the terms absent.

**Requirements page:** read, and ruled out. It describes operating system and hardware requirements and names Blender 4.5 LTS and 5.0 in its notes, not 5.2, and says nothing about render output.

---

## Things a person with Blender 5.2 open could settle by hand

*(Hand tests, 2026-08-09 — Opus 5. Five of the six items on this list were run on 2026-08-09 and their answers are in **Hand tests, 2026-08-09 — observed** above, so they are struck here and only what is genuinely still open is left. Item 1, the core case — see (b) and (d). Item 2, on purpose versus by accident — see (d): `SIGINT`, `SIGTERM` and `SIGKILL` do not all leave the same thing. Item 3, from outside the window — see (d) again; every run in this whole set was from outside the window. Item 4, do the two outputs stop at the same frame — see (d): ten images beside a file the reader called ten frames in one run and nine in another. Item 6, settling the ambiguity about `node.format.media_type = 'VIDEO'` — see (j): it is refused, in all four versions.)*

**Answered on Linux, untested on the Mac — Autosplit.** What an interruption leaves when the file has already split into more than one was run on the rented Linux machine and is at (l) above: the file already split away held every frame that went into it, and the one being written was left at 36 bytes holding none. It was not run on the Creator's Mac, because each run writes over two gigabytes and takes between thirty and sixty-seven minutes, so macOS is untested for this one.

**Still open — anything needing eyes on a window.** Items 1 and 2 were written as somebody pressing Esc or force-quitting from Activity Monitor. What was run instead was a signal sent from outside, because the rented Linux machine has no display server and cannot open a window at all, and the Creator ran nothing this round. So it is not known whether a stop from inside the window leaves the same thing as a signal from outside.

**Still open — the audio track.** *Part four* above asks whether muxing in an audio track changes the outcome. No run in this set had audio, so it is untested.

**Still open — Windows.** Untested rather than answered: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

The researcher's original list follows, kept as the record of what was thought open at the time.

1. **The core case.** In a new file, set Properties ‣ Output ‣ Output panel: Media Type to Video, container MPEG-4, codec H.264, file path `/tmp/vid/test.mp4`. Turn on Use Nodes in the Compositor and add a File Output node fed by the Render Layers image, Media Type Image, format PNG, directory `/tmp/imgs/`. Set the frame range to 1–40 and make each frame take a few seconds. Render Animation (Ctrl-F12), and at about frame 25 press Esc. Then look at both folders and write down: how many PNG files exist and their sizes; whether `/tmp/vid/test.mp4` exists, its size in bytes, and whether a video player opens it and how many frames it shows.

2. **On purpose versus by accident.** Repeat step 1 exactly, but instead of pressing Esc, force-quit the Blender process at about frame 25 (Task Manager, Activity Monitor, or `kill`). Compare the video file's size and playability with what step 1 left. This is the difference the manual never draws.

3. **From outside the window.** Save that same file and run `blender -b file.blend -a` in a terminal, then press Ctrl-C at about frame 25. Compare the two files left behind with what steps 1 and 2 left. This settles the "outside the interface" line.

4. **Do the two outputs stop at the same frame?** In whichever of the runs above leaves a readable video, count the frames in the video and count the PNG files, and say whether the video's last frame is the same frame as the last PNG.

5. **Autosplit.** Repeat step 1 with Autosplit Output enabled and a resolution and length that pass 2 GiB, then interrupt after the first split has happened. Note how many video files exist, and whether the earlier, already-split file plays.

6. **Settling the ambiguity I flagged.** In the Python console, on a File Output node, try `node.format.media_type = 'VIDEO'` and see whether it is accepted and what the node's interface then offers. This tells us whether a single File Output node can itself write a video, which would give a second way to read question 12.

---

2026-08-09 — Opus 5 (Claude Code) — Frames, Blender 5.2
