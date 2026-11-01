# Frames — later questions — Blender 5.1

**Sources consulted, and when.** Blender 5.1's manual, Blender 5.1's Python API documentation, Blender 5.1's release notes, and Cycles' own files for 5.1 — all from the copies in `Blender Sources/`, taken on 2026-07-30. I read those copies on 2026-08-09.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 12 was assigned to me. The rest of the sheet is untouched.

---

## 12. When one render writes one video and separate image files over the same frame range, what does an interruption leave if all the images have been written but the video has not finished closing?

*(Hand tests, 2026-08-09 — Opus 5. The question was put to two running computers on 2026-08-09, and most of what this page calls "not in the sources" is now settled by observation. The observed answer is in its own section below, **Hand tests, 2026-08-09 — observed**. The label below still stands, because it is a statement about the documentation and the documentation has not changed.)*

**Label: not in the sources** (for the thing actually asked — what the interrupted video file is left as). Parts of the surrounding picture are documented, and I give those below, clearly separated from the gap.

### The question was ambiguous, and here is the reading I answered

A render job's own output is **one** kind of media, not two. The manual's Output panel lists Media Type as a single choice:

> "Media Type
>
> The type of media to save.
>
> Image:
>
> Saves each input as a separate image file.
> Each input can use its own image format settings.
>
> Multi-Layer EXR:
>
> Saves all inputs together in a single multi-layer OpenEXR file.
>
> Video:
>
> Encodes each frame into a video container.
> Encoding options can be found in the Encoding panel."

- Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
- Date read: 2026-07-30 (snapshot); consulted 2026-08-09

The Python API documentation says the same, as one property with three possible values:

> "media_type
>
> The type of media to save (default 'IMAGE')"

and its type is given as:

> "Literal[‘IMAGE’, ‘MULTI_LAYER_IMAGE’, ‘VIDEO’]"

- Source: Python API documentation — https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html
- Date read: 2026-07-30 (snapshot)

Getting from those two quotes to a reading of the question takes a step the sources do not take. They say Media Type is one choice with three values. They do not say that a render's own output has no other route to disk. A reader has to supply that missing part to reach "a render cannot write both a video and separate image files from its own output". So the narrowing below is **inferred**, and the step is mine.

On that inference, the only way in these sources for **one** render to write a video **and** separate image files over the same frames is: the scene's own output is set to Video, and the compositor's File Output node writes the image files alongside it. That is the reading I answered.

The other reading — that a render's own output produces both a video and an image sequence at once — I set aside on the strength of that inference, not on anything the sources say outright. If the inference is wrong, that reading is unanswered here.

### What is documented

**1. The File Output node writes its images to disk while the render is running, with the frame number in the name.**

> "The File Output node writes one or more images to disk during rendering or compositing.
> Each connected input is saved to the specified directory and filename,
> with the frame number appended to create a sequence."

- Source: manual — https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
- Date read: 2026-07-30 (snapshot)

**2. Animation frames are saved to a drive after being rendered, and something fires directly after each frame is written.**

> "animations are a series of images, or frames,
> and are automatically saved directly out to a drive after being rendered."

- Source: manual — https://docs.blender.org/manual/en/5.1/render/output/animation.html
- Date read: 2026-07-30 (snapshot)

The Python API documentation says the same about the point at which a frame is on disk, describing the handler that fires then:

> "on writing a render frame (directly after the frame is written)."

- Source: Python API documentation — https://docs.blender.org/api/5.1/bpy.app.handlers.html
- Date read: 2026-07-30 (snapshot)

**3. Image frames already written survive the render being cut short.** The manual says this in two places, both about an animation whose own output is an image sequence:

> "Your computer accidentally turns off in the middle of rendering your movie!
>
> Unless your animation renders in a few minutes,
> it is best to render the animation as separate image files.
> Instead of rendering directly to a compressed movie file, use a lossless format (e.g. PNG).
>
> This allows you an easy recovery if there is a problem and you have to re-start the rendering,
> since the frames you have already rendered will still be in the output directory."

- Source: manual — https://docs.blender.org/manual/en/5.1/render/output/animation.html
- Date read: 2026-07-30 (snapshot)

> "when rendering animations to image file formats the render job can be canceled
> and resumed at the last rendered frame by changing the frame range."

- Source: manual — https://docs.blender.org/manual/en/5.1/render/output/introduction.html
- Date read: 2026-07-30 (snapshot)

**4. A video written directly by a render that hits trouble may be unusable, and the manual's advice is to start over.**

> "It’s not recommended to render directly to a video format in the first instance.
> If a problem occurs while rendering, the file might become unplayable and you will
> have to re-render all frames from the beginning."

- Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
- Date read: 2026-07-30 (snapshot)

This is the closest the sources come to the question. Note what it does and does not say. It says the file *might* become unplayable — not that it will be, not what it contains, not whether it is still on disk. And it says "a problem occurs while rendering", which is broader and vaguer than "the render was interrupted with the video not yet closed".

**5. One picture, and what is visible in it.** The manual's Encoding panel screenshot, `_images/render_output_properties_output_video-encoding-panel.png`, shown on https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, has the output set to a video (File Format reads "FFmpeg video", Container "Matroska"). In it, the **Overwrite** and **Placeholders** checkboxes are greyed out. In the companion Output panel screenshot on the same page, `_images/render_output_properties_output_panel.png`, where the format is PNG, Overwrite is on and both boxes are live.

- Label for this: **documented** (the pictures are part of the manual) — that in these two pictures the controls are greyed out in the one with a video output and live in the one with a PNG output.
- This finding has no words to quote, because it rests on reading a picture. What stands in for the quote is the two named picture files and the page they sit on, so anyone checking it has to open the pictures rather than search the text.
- Caution: both screenshots show a "File Format" row rather than the "Media Type" row the 5.1 text describes, so the pictures look older than the text around them. Anything beyond "these two boxes are greyed out in this picture" is my assumption, not the picture's content.

**6. A video output need not be a single file.**

> "If your video is huge and exceeds 2GiB, enable Autosplit Output.
> This will automatically split the output into multiple files after the first file is 2GiB in size."

- Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
- Date read: 2026-07-30 (snapshot)

I include this only because it bears on "what is left on disk". That an interruption with this option on would leave more than one video file, of which only the last was being written, is **inferred** — the quote is about splitting at a size while the render runs, and says nothing at all about an interruption.

### Applying point 3 to the image files in this question — **inferred**

The question's premise is that all the image files have been written. The manual's statement that already-rendered frames "will still be in the output directory" is written about an animation whose **own** output is an image sequence, not about images written by a File Output node. To reach "the File Output node's images are still there after the interruption", a reader has to take a step the manual does not take: the node "writes … to disk during rendering", the sources nowhere say anything removes or rolls back what it wrote, therefore what it wrote stays. That step is mine, so the label is **inferred**, not documented.

### The gap — what is not in the sources

Nothing in the 5.1 manual, the 5.1 Python API documentation, the 5.1 release notes, or Cycles' 5.1 files says any of the following:

- Whether a video file is left on disk at all when a render is interrupted before the container is finished, or whether Blender removes it.
- What such a file contains — how many of the encoded frames are in it, its size, or whether any player can open it.
- Whether the finished image files and the unfinished video are consistent with each other (whether the video holds every frame the images hold, or fewer).
- Whether Blender ever finishes closing a video after an interruption — for instance on the way out of a cancelled job — or leaves it as it was.
- Whether an interruption during the closing step specifically differs from an interruption mid-render.

More basically: the sources never describe a "closing" or "finalising" step for a video container at all. The word does not appear in that sense anywhere I searched. The manual describes the container, the codecs, the bit rate, the muxing settings and the 2GiB autosplit, and stops there. The behaviour of a video output that was cut short is simply not documented — the single sentence at point 4 above is the whole of it, and it is a warning, not a description.

**Not covered.** This answer does not settle what the partial video file actually is after an interruption — present or absent, playable or not, how many frames it holds. It does not settle whether the answer differs by container (MP4, Matroska, AVI and the rest are documented as different formats, and nothing says they behave alike when cut short). It does not settle whether "interrupted" means the same thing for a deliberate stop, a crash, and the machine losing power — the sources draw no such distinction and I have not inferred one. It does not settle whether the File Output node can itself be pointed at a video (the manual documents only Image and Multi-Layer EXR for that node, while the underlying format settings type lists a Video value; I did not chase that down, as the question does not turn on it). And it does not settle what happens on the *next* run — whether an interrupted video is picked up, overwritten, or left alone.

**Outside the interface.** *(Hand tests, 2026-08-09 — Opus 5. This line is now answered by observation. Every run recorded in **Hand tests, 2026-08-09 — observed** below started Blender with `-b --factory-startup` and no window, on the Creator's Mac and on the rented Linux machine, which has no display server and cannot open one. So what is recorded there is the driven-from-outside behaviour for Blender 5.1: images arriving one per frame, a video file that holds 48 bytes until the end, a `SIGINT` that closes the file and fires `render_cancel`, and a `SIGTERM` or `SIGKILL` that does neither. What is still not settled is the other direction, whether a stop from inside the window leaves the same thing; that needs eyes on a window and was dropped this round.)*

The sources do not say. They do document that an animation can be rendered without the interface at all — "blender -b file.blend -a", with "-b" given as "Render in the background (without UI)" and "-a" as "Render the whole animation using all the settings saved in the blend-file" (manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html, read 2026-07-30). And the 5.1 release notes confirm such a render can be cancelled, in a fixed-bug line: "Warnings about freeing memory after the leak detecter has run when cancelling a CLI render" (release notes, https://developer.blender.org/docs/release_notes/5.1/bugfixes/, read 2026-07-30). So a render started outside the interface can be cancelled. That the whole situation the question describes — a video output and a File Output node together, cut short — can arise outside the interface is **inferred** from that line, not stated: no source puts those pieces together. And no source says what any of it leaves on disk, or says whether that is the same as or different from a render driven by a person clicking. Since the behaviour is undocumented inside the interface too, there is nothing to compare.

**Operating system.** *(Hand tests, 2026-08-09 — Opus 5. Two of the three are now answered by observation, and separately. Every test below that was run on both machines was run twice — once on the Creator's Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, and once on the rented Linux machine, Ubuntu 22.04.5 LTS on Vast.ai with an NVIDIA GeForce RTX 5060 and no display server. For Blender 5.1 the two agreed on every result. **Windows is untested**, not answered: no Windows machine has run it.)*

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. The 248 runs recorded at item (e) above put one qualification on "the two agreed on every result". **They agree on the shape and differ on the detail.** On both machines a `SIGINT` left the last image whole every time and a `SIGTERM` or `SIGKILL` left it short every time. What differed is what "short" was: on the rented Linux machine every one of the eighty short files, across all four versions, was exactly **0 bytes**, while on the Creator's Mac sixty-nine were 0 bytes and eleven were a whole number of 4096-byte blocks — 4096, 16384, 24576, 40960 or 73728 — all far short of a complete image. **[Recount, 2026-08-09 — this sentence used to say seventy-two and eight, and that was a miscount. Counting the run-by-run figures themselves — this version's ten runs per signal as printed on this page, and the other three versions' in the hand-test record these findings were written from — gives sixty-nine empty and eleven block-sized. A recount of figures already on the record: nothing was re-run, and no new fact is added.]** In Blender 5.1 itself, the Mac's one non-empty short file was 24576 bytes, in the first `SIGKILL` run. Windows is still untested.*)*

The sources do not say. I found no statement anywhere in the 5.1 manual, API documentation or release notes that ties render output, video encoding, cancelling a render, or what is left on disk to any particular operating system. The video and audio format page states only that "Blender uses FFmpeg to handle video encoding and decoding" (manual, https://docs.blender.org/manual/en/5.1/files/media/video_formats.html, read 2026-07-30), with no per-system caveat, and its "Known Limitations" section names only a codec limitation ("H.264 requires both width and height to be divisible by 2"), not an operating-system one. The absence of an OS caveat is not the same as an assurance that behaviour is identical everywhere, so I am not claiming that.

### Hand tests, 2026-08-09 — observed

Eleven tests were written for this question and run by hand on **2026-08-09**, the same tests in all four researched versions. The whole list, with the exact commands and the exact Python, is `Audits/Blender Research/Frames/Hand tests — later questions.md`. What follows is what came back for Blender 5.1.

**The two machines, in full.** *The rented Linux machine* — rented on Vast.ai and reached over SSH, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, and no display server at all; Blender 5.1.2 at `/opt/blender/5.1/blender`. It was shared with other work while these ran. *The Creator's Mac* — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics; Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`. Neither machine speaks for the other, so each result says which one it came from. Every run used `--factory-startup` and a fresh throwaway `BLENDER_USER_RESOURCES` folder.

**A word about the reader.** Neither machine has `ffprobe` or `ffmpeg`, so every video file was read back by loading it into Blender as a movie clip. That reader prints `loaded ok` even for a file holding nothing — a 48-byte file comes back as `loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. So each result records three things separately: whether the file exists, its size in bytes, and the frame count the reader gave.

**What was built, to what.** A blend-file made by script: the scene's own output with `media_type` set to `VIDEO` and then `file_format` set to `FFMPEG`, container MPEG-4, codec H.264, writing into one empty folder; the compositor given a Render Layers node feeding both a File Output node and the Group Output, the File Output node writing PNGs into a second empty folder; Cycles on the processor. Three things about driving 5.1 came out of building it, and they are facts about the machinery rather than answers to this question: the scene's compositor is reached at `scene.compositing_node_group` rather than `scene.node_tree`; the File Output node has `directory` and `file_name` rather than `base_path`; and the node writes nothing at all until an item is added to its `file_output_items`, its format's media type having defaulted to `MULTI_LAYER_IMAGE`. Rendered with `blender -b --factory-startup job.blend -a`, from outside Blender's own window.

*(Citation check, 2026-08-09 — Opus 5. **What this set-up does not say, and a repeater has to supply.** It does not say what is in the scene, how many samples, what resolution, or what frame rate. The citation check repeated these tests using the factory-startup scene as it stands, 128 samples, 640 by 360 for the eight-frame runs and 480 by 270 for the twenty-frame runs, and 24 frames a second — the last three read off this page's own readback lines, the first two guessed. Frames then rendered in about half a second where this run's took three or four, and that difference is what item (e) turns on. Anybody repeating this should write down the sample count. Changed by the citation check.)*

**(a) A complete run: the images arrive one per frame, the video arrives at the very end.** Frame range 1–8, watched ten times a second on Linux and four times a second on the Mac. On the rented Linux machine the PNG files appeared one at a time, `vid0001-0008.mp4` appeared at 0.608 s holding **48 bytes** and held 48 bytes throughout, the eighth PNG landed at 25.031 s with the video still at 48 bytes and still held open, and at 25.132 s the log reads:

> ` 25.132  imgs=[eight files] vids=[('vid0001-0008.mp4', 4831)] open=[]`

Readback of the finished file: `READVID loaded ok | frame_duration: 8 | size: (640, 360) | fps: 24.0`. On the Creator's Mac the same shape, ending at 4624 bytes and `frame_duration: 8`. **The two machines agreed.** This is the answer to the researcher's own point 1 and point 2 above, put to a machine: the node's images do reach the disk one per frame while the render runs.

**(b) The state the question describes.** A handler on `bpy.app.handlers.render_write` printed the state at every frame and killed its own process the moment it fired for the last frame. On the rented Linux machine, with `SIGKILL`:

> `KILLATLAST render_write frame: 8 | images: ['ImageImage0001.png', 'ImageImage0002.png', 'ImageImage0003.png', 'ImageImage0004.png', 'ImageImage0005.png', 'ImageImage0006.png', 'ImageImage0007.png', 'ImageImage0008.png'] | video: [('vid0001-0008.mp4', 48)]`
> `KILLATLAST sending SIGKILL to self 58185`
> `exit status of blender: 137`

The line `KILLATLAST render call returned` was not printed, so the stop landed where it was aimed. Left on disk: eight PNG files of 369050 bytes each, and `vid0001-0008.mp4` of **48 bytes**, reading back `frame_duration: 0 | size: (256, 256) | fps: 0.0`. `SIGTERM` gave exit status 143 and the identical result. On the Creator's Mac, both signals gave the identical result. **The two machines agreed.**

**(c) The last image was whole.** Pixels compared rather than bytes, between the eighth PNG of the stopped run and the eighth PNG of the complete run: `PIX RESULT: largest difference on any channel = 0.0` on both machines, the files differing in size by nine bytes on Linux (369050 against 369041) and eight on the Mac.

**(d) Interrupted partway, three ways.** Frame range 1–20, the signal sent once ten PNGs were on disk. On the rented Linux machine, `SIGINT` gave `RUNSIGNAL exit code: 1`, ten images and `[('vid0001-0020.mp4', 3850)]`, with Blender printing `Sent an internal break event. Press ^C again to kill Blender`; readback `frame_duration: 10`. `SIGTERM` gave exit code `-15` and `SIGKILL` gave `-9`, both leaving ten images and a 48-byte video reading back `frame_duration: 0`. On the Creator's Mac the same three, the `SIGINT` file being 3831 bytes and reading back `frame_duration: 10`. **The two machines agreed.** One thing worth recording about the frame count: a second `SIGINT` run of the same set-up on the rented Linux machine, the one in (g) below, left a file that read back as **9** frames rather than 10, with ten images on disk. So the number the reader gives for a `SIGINT`-stopped file was not the same twice on the same machine.

**(e) A file being on disk is not the same as the frame having finished.** In Blender 5.1 on the rented Linux machine all ten images were at the full size after each of the three signals. In Blender 4.5 the same `SIGKILL` run left its tenth image at 0 bytes, and in Blender 5.0 it left it at 155648 bytes against a full 254253 — so a cut-short image file is a thing that happens, and 5.1 simply did not show it in these runs.

*(Citation check, 2026-08-09 — Opus 5.* The 5.1 half of this reproduced exactly: all ten images at 95295 bytes after each of the three signals on the rented Linux machine, and the full size on the Creator's Mac too. **The cross-version half is disputed.** The citation check's own 4.5 and 5.0 runs on the rented Linux machine left every image at the full size after every signal; the one cut-short image it saw was in 4.5 on the Creator's Mac after `SIGTERM`, not after `SIGKILL`. So "a cut-short image file is a thing that happens" still stands, and the citation check saw it — but not in the runs named here. That half is on the hand-test list to be run again. Changed by the citation check.*)*

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. The test was run, as entry 12 of `Audits/Blender Research/Frames/Hand tests — later questions.md`: **ten runs for each of `SIGINT`, `SIGTERM` and `SIGKILL`, in all four versions, on both machines, 248 runs in all.** All three sets of runs now stand side by side and none is deleted.*

*The set-up numbers the earlier record left out, now written down: frames 1 to 20, 480 by 270, 24 frames a second, Cycles on the processor, four versions run side by side with a fixed thread count each — **200 samples at four threads on the rented Linux machine**, **150 samples at two threads on the Creator's Mac** — giving 4.2 to 4.9 seconds a frame on Linux and 4.9 to 5.5 on the Mac. The watcher polled every millisecond and sent the signal between 0.000013 and 0.001426 seconds after the tenth image file appeared.*

***What came back for Blender 5.1.*** *On the **rented Linux machine**, where a complete image in the control run was 254122 bytes: the tenth image, `ImageImage0010.png`, was 254122 bytes in all ten `SIGINT` runs and **0 bytes in all ten `SIGTERM` runs and all ten `SIGKILL` runs**. On the **Creator's Mac**, where a complete image was 256712 bytes: 256712 bytes in all ten `SIGINT` runs; **0 bytes in all ten `SIGTERM` runs**; and in the ten `SIGKILL` runs, 24576, 0, 0, 0, 0, 0, 0, 0, 0 and 0 bytes in run order.*

***The controls.*** *In every one of the sixty runs the first nine images were at the full size and exactly ten images were on disk, so the render was stopped rather than finished; exit statuses were 1, -15 and -9 for the three signals throughout. The full size comes from a complete uninterrupted run of the same file on the same machine. And the other outcome is a real one, because it happened: `SIGINT` gave ten runs with every image whole.*

***How this sits beside the two runs above, without picking between them.*** *On `SIGINT` all three agree. On `SIGTERM` and `SIGKILL` this batch does **not** match the 5.1 half of the finding above, nor the citation check's repeat of it: both record all ten images at the full size after every signal in 5.1, and this batch found the tenth image short in every one of its forty `SIGTERM` and `SIGKILL` runs in 5.1 across the two machines. It does support the finding's cross-version half — that a cut-short image file is a thing that happens — and it found it in 5.1 as well, which the earlier runs did not. Nothing here says which earlier run was right; the earlier runs were one run per signal, this one was ten, and this one states a condition theirs do not — how quickly the signal followed the file appearing. Added by the second batch of hand tests.)*

**(f) A second run over the same range**, on the rented Linux machine, starting from what the `SIGKILL` run left and allowed to finish: the video went from 48 bytes to **6898 bytes** with a new modification time, reading back `frame_duration: 20`, and all twenty PNGs were written again at 254131 bytes with new modification times, including the ten the first run had already completed. So the leftover video was replaced rather than added to or left alone, and nothing was skipped. That is the answer to the researcher's own hand test 6.

**(g) Whether a cancel handler fires when the stop comes from outside.** On the rented Linux machine, with handlers on `render_cancel`, `render_complete` and `render_write`, and `SIGINT` sent once ten images were written:

> `CANCELWATCH render_write frame 9 | video sizes: [('vid0001-0020.mp4', 48)]`
> `CANCELWATCH render_cancel fired at frame 1`
> `CANCELWATCH copied /root/ht-frames/e10_5.1_SIGINT/vid/vid0001-0020.mp4 3833 -> /root/ht-frames/e10_5.1_SIGINT/copy/vid0001-0020.mp4`
> `CANCELWATCH render call returned`
> `RUNSIGNAL copy and final file identical byte for byte: True`

The file was already at its full 3833 bytes when the cancel handler fired, and nothing changed it afterwards; readback `frame_duration: 9`. With `SIGTERM`, no `render_cancel` line appeared at all, no copy was taken, and the file left behind was 48 bytes reading back `frame_duration: 0`. `render_complete` never appeared in either run.

**(h) The container changes the number of bytes left, not the frame count.** On the rented Linux machine the containers 5.1 offers are `['MPEG4', 'MKV', 'WEBM', 'AVI', 'DV', 'FLASH', 'MPEG1', 'MPEG2', 'OGG', 'QUICKTIME']` — the same ten as in the other three versions. The same stop-at-the-last-frame run in each left: MPEG4 48 bytes, MKV 0, WEBM 262, AVI 0, DV no file at all, FLASH 0, MPEG1 0, MPEG2 0, OGG 0, QUICKTIME 36. Every one read back `frame_duration: 0`. WEBM and DV never reached the interruption and said so, printing `RuntimeError: Error: Could not initialize streams, probably unsupported codec combination` and `RuntimeError: Error: Render width has to be 720 pixels for DV!`. The control — the same containers rendered to the end — produced readable files for the other eight (MPEG4 2243 bytes / 4 frames, MKV 2035 / 4, AVI 7232 / 4, FLASH 3592 / 4, MPEG1 4096 / 2, MPEG2 6144 / 2, OGG 5403 / 1, QUICKTIME 2177 / 4). That is the answer to the researcher's own hand test 3.

**(i) The Media Type premise, and the inference the researcher drew from it.** On the rented Linux machine:

> `Q6 media_type present: True`
> `Q6 media_type is_enum_flag: False | items: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`
> `Q6 file_format is_enum_flag: False`
> `Q6 set FFMPEG ok, file_format now: FFMPEG`
> `Q6 set PNG FAILED: TypeError bpy_struct: item.attr = val: enum "PNG" not found in ('FFMPEG')`
> `Q6 is_movie_format: True`

So the media type holds one value at a time, and with it set to `VIDEO` the only file format the render output will accept is `FFMPEG`. One caution about this record: the two format lists the script printed under `IMAGE` and under `VIDEO` came back identical, because that listing is the property's whole declared list rather than what is on offer at the moment; what shows the live state is the failed assignment above and `is_movie_format` reading `False` under the image setting and `True` under the video setting in the same run.

**(j) Whether a File Output node can be pointed at a video — it cannot.** This is the loose end the audit sent back and the researcher's own hand test 6 in the second-pass document. On the rented Linux machine:

> `Q7 node format media_type present: True`
> `Q7 node media_type items: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`
> `Q7 node media_type now: MULTI_LAYER_IMAGE`
> `Q7 set node media_type VIDEO FAILED: TypeError bpy_struct: item.attr = val: enum "VIDEO" not found in ('IMAGE', 'MULTI_LAYER_IMAGE')`
> `Q7 set node file_format FFMPEG FAILED: TypeError bpy_struct: item.attr = val: enum "FFMPEG" not found in ('OPEN_EXR_MULTILAYER')`

The property lists three values and will take two of them. 5.0 and 5.2 printed the same; Blender 4.5 has no such property on the node at all.

**(k) Overwrite and Placeholders can be changed with a video output in force.** This is the researcher's own hand test 5, answered from a terminal rather than from a panel. On the rented Linux machine, all four versions printed the same six lines; for 5.1:

> `Q9 IMAGE file_format: PNG | is_movie_format: False`
> `Q9 IMAGE use_overwrite was True now False`
> `Q9 IMAGE use_placeholder was False now True`
> `Q9 VIDEO file_format: FFMPEG | is_movie_format: True`
> `Q9 VIDEO use_overwrite was True now False`
> `Q9 VIDEO use_placeholder was False now True`

Both settings can be held and changed with a video output in force. Whether the boxes are *drawn* greyed out in the panel — which is what the manual's screenshot was being read for at point 5 above — is a different question and was not tested; see the note under the hand-test list.

**(l) Autosplit: the file already split away survives, the one being written does not.** On the rented Linux machine only. Two set-ups failed first and are recorded rather than quietly replaced: container `AVI` with codec `NONE` gave `Error: Could not initialize streams, probably unsupported codec combination` and a 0-byte file, and container `AVI` with codec `HUFFYUV` worked but was far too slow to reach 2 GiB. What was used: container `QUICKTIME`, codec `QTRLE`, 3840 by 2160, `use_autosplit = True`, frame range 1–4000, and the **Workbench** engine rather than Cycles so that a couple of thousand frames could be reached at all — that last change is a departure from the set-up used everywhere else here and is recorded because the engine is not what this test is asking about. The render was watched and killed with `SIGKILL` the moment a second video file appeared. What came back:

> `RUNSPLIT 3793.4 s: a second video file has appeared: ['vid_0000001-4000.mov', 'vid_0010001-4000.mov'] -- sending SIGKILL to pid 139630`
> `RUNSPLIT exit code: -9`
> `RUNSPLIT files left: [('vid_0000001-4000.mov', 2012531971), ('vid_0010001-4000.mov', 36)]`

Readback: the first file `loaded ok | frame_duration: 2197 | size: (3840, 2160) | fps: 24.0`; the second file `loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. The other three versions gave the same two files with the same two names and the same shape of answer. The control that the split really happened is the file count: one file would have meant the condition never bit, and there were two. **The Creator's Mac was not used for this test**, because each run writes over two gigabytes and takes between thirty and sixty-seven minutes; macOS is untested for autosplit and that is a gap rather than an answer.

**What these tests do not settle.** Nothing was run inside Blender's own window, so nothing here says what pressing Esc or clicking the status bar's cancel button leaves. Nothing was run on Windows, because no Windows machine has run it. The video reader used was Blender itself, so a frame count is what Blender's movie-clip reader made of the file rather than what every player would make of it. And every run used one scene, one codec and one frame rate. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Where I looked

Everything below is inside `Blender Sources/`, on the 5.1 copies.

Pages read in full or in the relevant part:

- Output properties — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html (whole page: Media Type, Overwrite, Placeholders, Encoding, Container, Autosplit, Mux Rate, the Tips at the end)
- Rendering Animations — https://docs.blender.org/manual/en/5.1/render/output/animation.html (whole page, including the Hints section on the computer turning off mid-render)
- Output Introduction — https://docs.blender.org/manual/en/5.1/render/output/introduction.html (whole page)
- Supported Video & Audio Formats — https://docs.blender.org/manual/en/5.1/files/media/video_formats.html (whole page, including Known Limitations)
- File Output Node — https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html (whole page)
- Rendering From The Command Line — https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html (whole page)
- Render menu in the Topbar — https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html (Render Menu section)
- Status Bar — https://docs.blender.org/manual/en/5.1/interface/window_system/status_bar.html (the one line about aborting a job)
- Node editing, Read View Layers — https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html (reached by searching for "recover"; it is about reading cached view layers, not about files on disk)
- https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html (media_type and its allowed values)
- https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html (whole page)
- https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html (filepath, use_overwrite, use_placeholder, use_file_extension)
- https://docs.blender.org/api/5.1/bpy.app.handlers.html (render_init, render_pre, render_post, render_write, render_complete, render_cancel, render_stats)
- Release notes: https://developer.blender.org/docs/release_notes/5.1/compositor/, https://developer.blender.org/docs/release_notes/5.1/pipeline_io/ (its "Video & Audio output" section), https://developer.blender.org/docs/release_notes/5.1/rendering/, https://developer.blender.org/docs/release_notes/5.1/core/, https://developer.blender.org/docs/release_notes/5.1/bugfixes/

Pictures opened:

- `_images/render_output_properties_output_video-encoding-panel.png` and `_images/render_output_properties_output_panel.png`, both on https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
- `_images/node-types_CompositorNodeOutputFile.webp`, on https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — it shows only the node's directory and file-name fields, and settles nothing here

Searches run across the whole of the 5.1 manual, the whole of the 5.1 API documentation, the whole of the 5.1 release notes, and the whole of Cycles' 5.1 files, each case-insensitive:

- Words for a render being cut short: interrupt, interrupted, cancel, cancelled, canceled, abort, aborted, stop, stop rendering, killed, quit blender, quitting, power outage, power failure, crash during render, SIGTERM, kill, resume, when the render
- Words for a damaged or part-written file: corrupt, unplayable, not playable, damaged, incomplete, unfinished, partial, partially written, partial file, truncated, half-written, zero-byte, empty file, 0 bytes, leftover
- Words for a video being wound up: finalize, finaliz, close the file, close the movie, closing, flush, trailer, moov, mux, muxer, container, movie file
- Words for what is on disk afterwards: left on disk, remains on disk, already written, written so far, up to that point, so far rendered

The only hits with any bearing are the ones quoted above. The searches for `finaliz`, `flush`, `trailer`, `moov`, `partially written`, `partial file`, `half-written`, `zero-byte`, `0 bytes`, `left on disk`, `remains on disk`, `already written`, `written so far`, `stop rendering` and `power failure` returned nothing relevant anywhere in the four sources — `moov` and `trailer` (in the video sense) returned nothing at all.

Cycles' own 5.1 files were searched for `ffmpeg`, `movie`, `video`, `file_output` and `cancel`. The only matches are two lines in `cycles_5.1/operators.py`, at the top level of that folder — a `return {'CANCELLED'}` at line 88 and a comment at line 97 reading `# TODO: support cancel and progress reports.` — both inside the Denoise Animation operator, which does read and write image files of its own but has nothing to do with the render's video output. Cycles' files say nothing about video output or about what an interruption leaves behind.

*(Citation check, 2026-08-09 — Opus 5.* This read "both inside an operator unrelated to writing output files". The operator is `CYCLES_OT_denoise_animation`, and it writes image files of its own — it sets `scene.render.filepath` and builds input and output paths. What it is unrelated to is the render's *video* output, which is what this question is about. Corrected to the file, and the file's place in the folder added, which the record itself flagged as missing. Changed by the citation check.*)*

---

## Things a person with Blender open could settle in a few minutes

*(Hand tests, 2026-08-09 — Opus 5. Five of the six items on this list were run on 2026-08-09 and their answers are in **Hand tests, 2026-08-09 — observed** above, so they are struck here and only what is genuinely still open is left. Item 1, what the interrupted video file actually is — see (b) and (d). Item 2, the same killed rather than cancelled — see (d), where `SIGINT`, `SIGTERM` and `SIGKILL` were each run. Item 3, whether the container makes a difference — see (h), where all ten containers 5.1 offers were run, not the three named here. Item 4, whether the images and the video agree — see (b) and (d). Item 6, what a second run does to the leftover — see (f). Item 5, whether Overwrite and Placeholders are really unavailable for video, is half answered and half dropped, and is set out below.)*

**Half answered, half dropped — item 5.** Whether the two settings can be held and changed with a video output in force was answered from a terminal, at (k) above: in all four versions both can. Whether the boxes are *drawn* greyed out in the panel was not answered and is dropped this round, because the rented Linux machine has no display server and cannot open a window at all, and the Creator ran nothing. Reading a property is a different test from looking at a panel, and this document should not let the first stand in for the second.

**Answered on Linux, untested on the Mac — Autosplit.** What an interruption leaves when the file has already split into more than one was run on the rented Linux machine and is at (l) above: the file already split away held every frame that went into it, and the one being written was left at 36 bytes holding none. It was not run on the Creator's Mac, because each run writes over two gigabytes and takes between thirty and sixty-seven minutes, so macOS is untested for this one.

**Still open — anything else needing eyes on a window.** Items 1 and 4 were written as somebody pressing Esc. What was run instead was a signal sent from outside, for the reason just given. So it is not known whether a stop from inside the window leaves the same thing.

**Not run — Windows.** The closing line of this list asks for all three operating systems. macOS and Linux were each run and are answered separately above. Windows was not tried: no Windows machine has run it, so it is untested rather than answered.

The researcher's original list follows, kept as the record of what was thought open at the time.

1. **What the interrupted video file actually is.** Open a scene that renders quickly. In Properties ‣ Output, set Media Type to Video, Container to Matroska, Video Codec to H.264, and File Path to an empty folder. In the Compositor, add a File Output node fed from the Render Layers image socket, pointed at a second empty folder, format PNG. Set the frame range to 1–50. Start Render Animation. When the status bar shows it has begun frame 50, press Esc. Then look at both folders and write down: how many PNG files exist; whether a `.mkv` file exists; its size in bytes; and whether a video player opens it and, if so, how many frames it plays. That run settles it for one container, one way of being interrupted, and one computer — not for the others below.

2. **The same run, but killed rather than cancelled.** Repeat step 1 from the command line — `blender -b file.blend -a` — and instead of pressing Esc, end the Blender process from the operating system partway through. Record the same three things. This tells you whether a clean cancel and a hard stop leave different remains, which the sources never distinguish.

3. **Whether the container matters.** Repeat step 1 three times, changing only Container: MPEG-4 (`.mp4`), Matroska (`.mkv`), AVI (`.avi`). Record for each whether the leftover file opens in a player. Different containers are written differently and the manual says nothing about whether they survive a cut-short render alike.

4. **Whether the images and the video agree.** In the run from step 1, count the PNG files written and count the frames the leftover video plays. Note the difference. This is the "all the images have been written but the video has not finished closing" case stated in the question, checked directly.

5. **Whether Overwrite and Placeholders are really unavailable for video.** In Properties ‣ Output, switch Media Type between Image and Video and watch the Overwrite and Placeholders checkboxes. The manual's screenshot shows them greyed out for video, but that screenshot looks older than the page text. Confirm it in 5.1 itself. If they are greyed out, it means a second run cannot skip what the interrupted video already covered.

6. **What a second run does to the leftover video.** After the interrupted run from step 1, start Render Animation again over the same range without deleting anything. Note whether the leftover video file is replaced, appended to, or left untouched, and whether the PNG files are re-rendered or skipped.

---

2026-08-09 — Opus 5 (Claude Code) — Frames, Blender 5.1
