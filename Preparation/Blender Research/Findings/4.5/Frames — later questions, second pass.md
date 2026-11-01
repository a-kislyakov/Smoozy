# Frames — later questions, second pass (Blender 4.5)

**Sources consulted, and when.** Blender's 4.5 manual, Blender's 4.5 Python API documentation, the 4.5 release notes, and Cycles' own files for 4.5, all as the plain-text copies held in this project's `Blender Sources/` folder, taken on 2026-07-30. **That snapshot date, 2026-07-30, is the date read for everything quoted below, Cycles' files included.**

*(Citation check, 2026-08-09 — Opus 5.* Cycles' files were given a date read of 2026-08-09 here, which is the day the researcher worked rather than the day the snapshot was taken. Cycles' files come out of the same snapshot as the manual, the API documentation and the release notes, so all four now carry 2026-07-30. The citation check levelled this date across every version it covered. Changed by the citation check.*)*

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 12. When one render writes one video and separate image files over the same frame range, what does an interruption leave if all the images have been written but the video has not finished closing?

**Reading of the question.** The question is ambiguous and I have answered one reading. The reading I took: a single render job whose main output is set to a video format, running alongside the Compositor's File Output node, which writes separate image files for the same frames — so one render produces both a video and an image sequence. That is the only arrangement in which one render job writes both, going by the sources: the main output takes one file format, and the File Output node is documented as an extra writer of images that can sit anywhere in the node tree.

*(Citation check, 2026-08-09 — Opus 5.* "The main output takes one file format" is stated here with no quote and no label, and nothing on this page's documentation side carries it. What carries it is the hand test at **(i)** below, which printed `Q6 file_format is_enum_flag: False`, `Q6 set FFMPEG ok`, then `Q6 set PNG ok` on a running Blender 4.5. So the premise is **observed**, not documented, and (i) is its evidence. The citation check re-ran (i) and got the same lines. Changed by the citation check.*)* The other reading is that "one video and separate image files" means two separate renders of the same frame range, one to video and one to images; I have not answered that reading, because the question says "one render".

*(Hand tests, 2026-08-09 — Opus 5. The question was put to two running computers on 2026-08-09, and most of what this page calls "not in the sources" is now settled by observation. The observed answer is in its own section below, **Hand tests, 2026-08-09 — observed**. The label below still stands, because it is a statement about the documentation and the documentation has not changed.)*

**LABEL: not in the sources**, for what the question actually asks. The sources do not describe the state a video file is left in when a render is interrupted, and they do not describe a closing step for a video file at all. What is documented are the surrounding pieces, quoted below; the specific outcome asked about is not.

### What the sources do say

**The video may be unusable if anything goes wrong during the render, and the remedy is to re-render every frame.** This is the closest the sources come, and it is a general warning, not a description of the interrupted-and-not-closed state:

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — date read 2026-07-30.

Note the wording: "might become unplayable". It does not say the file is deleted, does not say the file is kept, does not say how much of it is there, and does not tie the outcome to any particular cause of interruption.

**Image files written during an animation render are written to disk as the render goes, and the ones already written stay there.**

> "animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html — date read 2026-07-30.

> "For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/introduction.html — date read 2026-07-30.

**The File Output node saves an image writing to the current frame, and a whole sequence when an animation is rendered.**

> "This node writes out an image, for each frame range specified, to the filename entered, as part of a frameset sequence."

> "The image(s) will be saved on rendering, writing to the current frame. An entire sequence of images will be saved, when an animation is rendered."

Source: manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html — date read 2026-07-30.

*(Citation check, 2026-08-09 — Opus 5.* This heading first read "The File Output node writes one image per frame while the render runs." Two steps past the quotes. "Per frame" needs the reader to add that an animation makes each frame the current frame in turn; "while the render runs" needs the reader to add that "will be saved on rendering" means as the frames go rather than in one batch at the end. Neither is on the page. The heading now says what the words carry; both further steps are **inferred**, and observed item (a) below is what settles them by watching. Changed by the citation check.*)*

**The main render output is fed by the Composite node, separately from the File Output node.** This is what makes the two outputs of one render distinct:

> "The Composite node defines the final output of the Compositor. It is the node where the image result is sent to the renderer or image output after rendering."

Source: manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/composite.html — date read 2026-07-30.

**A single-file video is a container holding an encoded stream.**

> "Video formats are composed of a container, a codec, and sometimes audio which is stored using its own codec. The roll of the container to encapsulate video and audio data that is compressed using a codec."

Source: manual, https://docs.blender.org/manual/en/4.5/files/media/video_formats.html — date read 2026-07-30. (Spelling as printed.)

### What I could not find, and so am not claiming

Nothing in the manual, the Python API documentation, the release notes or Cycles' files says:

- that a video file has a closing or finalising step at the end of a render;
- what is on disk if a render to video is stopped — whether a file exists, whether it holds the frames rendered so far, whether it can be opened, or whether Blender removes it;
- whether the outcome differs between a render stopped on purpose and one ended by the machine or the program going down;
- whether the images written by a File Output node are affected in any way by the main output being a video, or by the render being stopped;
- anything at all about the combined case in the question — a video and an image sequence from the same render, interrupted after the last image is written.

**Not covered.** This answer does not settle what is actually left on disk. It does not settle whether the images from the File Output node are complete and usable at the moment of interruption — only that the sources say already-rendered frames of an image-sequence render remain in the output directory, which is a statement about the main output written as images, not about the File Output node specifically. It does not settle whether "all the images have been written" implies the video's last frame was also handed to the encoder. It does not settle whether a partly written video can be repaired. And because the sources never mention a closing step, the answer cannot confirm or deny the premise that such a step exists.

**Outside the interface.** *(Hand tests, 2026-08-09 — Opus 5. This line is now answered by observation. Every run recorded in **Hand tests, 2026-08-09 — observed** below started Blender with `-b --factory-startup` and no window, on the Creator's Mac and on the rented Linux machine, which has no display server and cannot open one. So what is recorded there is the driven-from-outside behaviour for Blender 4.5. What is still not settled is the other direction — whether a stop from inside the window leaves the same thing; that needs eyes on a window and was dropped this round. The sentence below stands as a statement about the documentation.)* The sources do not say. The manual documents rendering an animation from the command line with `-a, --render-anim` ("Render frames from start to end (inclusive).", https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html) and gives a page of command-line rendering examples (https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html), both read 2026-07-30, but neither page mentions interruption, cancelling, or what a stopped render leaves behind. The arguments page does carry two entries with "abort" and "crash" in their names, and here is the whole of what they say: "--disable-abort-handler" / "Disable the abort handler." and "--disable-crash-handler" / "Disable the crash handler." Neither mentions a render and neither mentions output files. *(Citation check, 2026-08-09 — Opus 5. Those two entries were on a page recorded as read and were neither quoted nor set aside. Added from the page. Changed by the citation check.)* The warning about a video becoming unplayable is on the Output properties page and is not qualified by how Blender was started, but the sources never state that it applies equally in background mode, so I am not claiming it.

**Operating system.** *(Hand tests, 2026-08-09 — Opus 5. Two of the three are now answered by observation, and separately. Every test below that was run on both machines was run twice — once on the Creator's Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, and once on the rented Linux machine, Ubuntu 22.04.5 LTS on Vast.ai with an NVIDIA GeForce RTX 5060 and no display server. The two agreed on everything except one number: the file left by a `SIGINT` partway through read back as 9 frames on Linux and 10 on the Mac, with ten images on disk in both. **Windows is untested**, not answered: no Windows machine has run it.)* The sources do not say. The nearest statements are about codec and player compatibility, not about interruption: "Each has advantages and disadvantages, and compatibility with different players on different operating systems" (manual, https://docs.blender.org/manual/en/4.5/files/media/video_formats.html, read 2026-07-30). Nothing anywhere ties the behaviour of a stopped render, for either the video or the images, to a particular operating system.

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. The 248 runs recorded at item (e) above add one more comparison of the two machines. **They agree on the shape and differ on the detail.** On both, a `SIGINT` left the last image whole every time and a `SIGTERM` or `SIGKILL` left it short every time. What differed is what "short" was: on the rented Linux machine every one of the eighty short files was exactly **0 bytes**, while on the Creator's Mac sixty-nine were 0 bytes and eleven were a whole number of 4096-byte blocks — 4096, 16384, 24576, 40960 or 73728 — all far short of a complete image. **[Recount, 2026-08-09 — this sentence used to say seventy-two and eight, and that was a miscount. Counting the run-by-run figures themselves — this version's ten runs per signal as printed on this page, and the other three versions' in the hand-test record these findings were written from — gives sixty-nine empty and eleven block-sized. A recount of figures already on the record: nothing was re-run, and no new fact is added.]** Windows is still untested.*)*

### Hand tests, 2026-08-09 — observed

Eleven tests were written for this question and run by hand on **2026-08-09**, the same tests in all four researched versions. The whole list, with the exact commands and the exact Python, is `Audits/Blender Research/Frames/Hand tests — later questions.md`. What follows is what came back for Blender 4.5. It is the same record as in `Preparation/Blender Research/Findings/4.5/Frames — later questions.md`, written here too so that neither page points a reader the wrong way.

**The two machines, in full.** *The rented Linux machine* — rented on Vast.ai and reached over SSH, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, and no display server at all; Blender 4.5.12 at `/opt/blender/4.5/blender`. It was shared with other work while these ran. *The Creator's Mac* — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics; Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`. Neither machine speaks for the other, so each result says which one it came from. Every run used `--factory-startup` and a fresh throwaway `BLENDER_USER_RESOURCES` folder.

**A word about the reader.** Neither machine has `ffprobe` or `ffmpeg`, so every video file was read back by loading it into Blender as a movie clip. That reader prints `loaded ok` even for a file holding nothing — a 48-byte file comes back as `loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. So each result records three things separately: whether the file exists, its size in bytes, and the frame count the reader gave.

**What was built, to what.** A blend-file made by script: the scene's own output set to `FFMPEG`, container MPEG-4, codec H.264, writing into one empty folder; the compositor given a Render Layers node feeding both a Composite node and a File Output node, the File Output node writing PNGs into a second empty folder; Cycles on the processor. In 4.5 the compositor is reached at `scene.node_tree` and the node's folder at its `base_path`, and the render output's image settings have no `media_type`, so `file_format` was set to `FFMPEG` directly. Rendered with `blender -b --factory-startup job.blend -a`, from outside Blender's own window.

*(Citation check, 2026-08-09 — Opus 5. **What this set-up does not say, and a repeater has to supply.** It does not say what is in the scene, how many samples, what resolution, or what frame rate. The citation check repeated these tests using the factory-startup scene as it stands, 128 samples, 640 by 360 for the eight-frame runs and 480 by 270 for the twenty-frame runs, and 24 frames a second — the last three read off this page's own readback lines, the first two guessed. Frames then rendered in about half a second where this run's took three or four, and that difference is what item (e) turns on. Anybody repeating this should write down the sample count. Changed by the citation check.)*

**(a) A complete run: the images arrive one per frame, the video arrives at the very end.** Frame range 1–8, watched ten times a second on Linux and four times a second on the Mac. On the rented Linux machine, at its turning points:

> `  0.726  imgs=[] vids=[('vid0001-0008.mp4', 48)] open=['vid0001-0008.mp4']`
> `  4.555  imgs=['Image0001.png'] vids=[('vid0001-0008.mp4', 48)] open=['vid0001-0008.mp4']`
> … one further PNG at each of 8.400, 11.729, 15.780, 19.816, 23.160, 26.989 …
> ` 31.010  imgs=[eight files, Image0001.png … Image0008.png] vids=[('vid0001-0008.mp4', 48)] open=['vid0001-0008.mp4']`
> ` 31.111  imgs=[eight files] vids=[('vid0001-0008.mp4', 4655)] open=[]`

Readback: `READVID loaded ok | frame_duration: 8 | size: (640, 360) | fps: 24.0`. On the Creator's Mac the same shape, ending at 4789 bytes and `frame_duration: 8`. **The two machines agreed.** So the video file is created at once at 48 bytes, holds 48 bytes for the whole render, and reaches its full size in the same tenth of a second in which the process lets go of it — which answers the researcher's own item 4 below.

**(b) The state the question describes.** A handler on `bpy.app.handlers.render_write` printed the state at every frame and killed its own process the moment it fired for the last frame. On the rented Linux machine, with `SIGKILL`:

> `KILLATLAST render_write frame: 8 | images: ['Image0001.png', 'Image0002.png', 'Image0003.png', 'Image0004.png', 'Image0005.png', 'Image0006.png', 'Image0007.png', 'Image0008.png'] | video: [('vid0001-0008.mp4', 48)]`
> `KILLATLAST sending SIGKILL to self 49538`
> `exit status of blender: 137`

Left on disk: eight PNGs of 162489 bytes each and a **48-byte** video reading back `frame_duration: 0 | size: (256, 256) | fps: 0.0`. `SIGTERM` gave exit status 143 and the same. On the Creator's Mac, both signals gave the identical result. **The two machines agreed.**

**(c) The last image was whole.** Pixels compared rather than bytes, between the eighth PNG of the stopped run and the eighth PNG of the complete run: `PIX RESULT: largest difference on any channel = 0.0` on both machines, with the files differing in size by nine bytes on Linux and eight on the Mac.

**(d) Interrupted partway, three ways.** Frame range 1–20, the signal sent once ten PNGs were on disk. On the rented Linux machine, `SIGINT` gave `RUNSIGNAL exit code: 1`, ten images, `[('vid0001-0020.mp4', 3960)]`, Blender printing `Sent an internal break event. Press ^C again to kill Blender`, and readback `frame_duration: 9`. `SIGTERM` gave exit code `-15` and `SIGKILL` gave `-9`, both leaving ten images and a 48-byte video reading back `frame_duration: 0`. On the Creator's Mac the same three, except that the `SIGINT` file was 3963 bytes and read back `frame_duration: 10`. **The two machines agreed on the pattern and differed on that one number.** The number also moved between runs on the same machine: the `SIGINT` run at (g), same signal and same set-up with handlers added, left a file the reader called 10 frames on the rented Linux machine. What did not move is the contrast itself — `SIGINT` leaves a file the reader finds frames in, `SIGTERM` and `SIGKILL` leave one it finds none in.

**(e) A file being on disk is not the same as the frame having finished.** On the rented Linux machine the `SIGKILL` run left its tenth image, `Image0010.png`, at **0 bytes**, the other nine at 93611 bytes each. The `SIGTERM` and `SIGINT` runs left all ten at the full size.

*(Citation check, 2026-08-09 — Opus 5. **Disputed, going back to be run again. Do not lean on it.*** The citation check repeated all three runs on both machines. On the rented Linux machine every signal left all ten images at the full size — `('Image0010.png', 70768)` after `SIGKILL`, and the same after `SIGTERM` and `SIGINT`. On the Creator's Mac a **0-byte** tenth image appeared after `SIGTERM` — `[… ('Image0009.png', 70768), ('Image0010.png', 0)]` — and not after the other two. So the thing this finding points at does happen; which signal shows it did not hold. Neither run overwrites the other and neither is picked. Written out as a test on the hand-test list, to be run ten times per signal per version per machine with the sample count recorded. Changed by the citation check.)*

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. The test was run, as entry 12 of `Audits/Blender Research/Frames/Hand tests — later questions.md`: **ten runs for each of `SIGINT`, `SIGTERM` and `SIGKILL`, in all four versions, on both machines, 248 runs in all.** All three sets of runs now stand side by side and none is deleted.*

*The set-up numbers the earlier record left out, now written down: frames 1 to 20, 480 by 270, 24 frames a second, Cycles on the processor, four versions run side by side with a fixed thread count each — **200 samples at four threads on the rented Linux machine**, **150 samples at two threads on the Creator's Mac** — giving 4.2 to 4.9 seconds a frame on Linux and 4.9 to 5.5 on the Mac. The watcher sent the signal between 0.000013 and 0.001426 seconds after the tenth image file appeared.*

***For Blender 4.5.*** *On the **rented Linux machine**, a complete image being 93602 bytes: the tenth image was 93602 bytes in all ten `SIGINT` runs and **0 bytes in all ten `SIGTERM` runs and all ten `SIGKILL` runs**. On the **Creator's Mac**, a complete image being 93588 bytes: 93588 in all ten `SIGINT` runs; short in all ten `SIGTERM` runs, at 0, 16384, 0, 16384, 0, 0, 0, 0, 40960, 0 bytes in run order; short in all ten `SIGKILL` runs, at 0, 0, 16384, 0, 0, 16384, 4096, 0, 0, 0.*

***The controls.*** *Ten images on disk and the first nine at the full size in every run, so the stop really bit; exit statuses 1, -15 and -9 throughout; the full size taken from a complete uninterrupted run of the same file on the same machine. The other outcome is a real one and it happened: `SIGINT` gave ten whole images every time.*

***Beside the two runs above, without picking.*** *On `SIGINT` all three agree. On `SIGKILL` this batch matches the finding above — a 0-byte tenth image. On `SIGTERM` it does not match the finding above, which records all ten full, and does match the citation check's Mac run. It does not match the citation check's Linux runs. Added by the second batch of hand tests.)*

**(f) A second run over the same range**, on the rented Linux machine, starting from what the `SIGKILL` run left and allowed to finish: the video went from 48 bytes to **6657 bytes** with a new modification time, reading back `frame_duration: 20`, and all twenty PNGs were written again with new modification times, including the nine already complete.

**(g) Whether a cancel handler fires when the stop comes from outside.** On the rented Linux machine, with `SIGINT`:

> `CANCELWATCH render_write frame 10 | video sizes: [('vid0001-0020.mp4', 48)]`
> `CANCELWATCH render_cancel fired at frame 1`
> `CANCELWATCH copied /root/ht-frames/e10_4.5_SIGINT/vid/vid0001-0020.mp4 4032 -> /root/ht-frames/e10_4.5_SIGINT/copy/vid0001-0020.mp4`
> `RUNSIGNAL copy and final file identical byte for byte: True`

The file was already at its full size when the cancel handler fired, and nothing changed it afterwards. With `SIGTERM` no `render_cancel` line appeared at all and the file left behind was 48 bytes. `render_complete` never appeared in either run.

**(h) The container changes the number of bytes left, not the frame count.** The same stop-at-the-last-frame run in each of the ten containers 4.5 offers left: MPEG4 48 bytes, MKV 0, WEBM 262, AVI 0, DV no file at all, FLASH 0, MPEG1 0, MPEG2 0, OGG 0, QUICKTIME 36 — every one reading back `frame_duration: 0`. WEBM and DV never reached the interruption, printing `Could not initialize streams, probably unsupported codec combination` and `Render width has to be 720 pixels for DV!`. The control, the same containers run to the end, produced readable files for the other eight.

**(i) The premise about one output at a time.** On the rented Linux machine: `Q6 media_type present: False`; the file format list is the fifteen names ending in `FFMPEG`; `Q6 file_format is_enum_flag: False`; `Q6 set FFMPEG ok` then `Q6 set PNG ok`. So the render output's file format holds one value at a time and setting one replaces the other. `media_type` is absent in 4.5 and present in 5.0, 5.1 and 5.2 with the items `['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']`.

**(j) A File Output node cannot be set to write a video, in any of the four versions.** In 4.5: `Q7 node format media_type present: False` and `Q7 set node file_format FFMPEG FAILED: TypeError bpy_struct: item.attr = val: enum "FFMPEG" not found in ('BMP', 'IRIS', 'PNG', …, 'WEBP')`. In 5.0, 5.1 and 5.2 the node does have the setting, listing three values, and `Q7 set node media_type VIDEO FAILED: TypeError bpy_struct: item.attr = val: enum "VIDEO" not found in ('IMAGE', 'MULTI_LAYER_IMAGE')`.

**(k) Overwrite and Placeholders can be changed with a video output in force**, in all four versions: `Q9 VIDEO file_format: FFMPEG | is_movie_format: True`, `Q9 VIDEO use_overwrite was True now False`, `Q9 VIDEO use_placeholder was False now True`. This is about the settings, not about how the boxes are drawn in a panel.

**(l) Autosplit: the file already split away survives, the one being written does not.** On the rented Linux machine only. Two set-ups failed first and are recorded rather than quietly replaced: container `AVI` with codec `NONE` gave `Error: Could not initialize streams, probably unsupported codec combination` and a 0-byte file, and container `AVI` with codec `HUFFYUV` worked but was far too slow to reach 2 GiB. What was used: container `QUICKTIME`, codec `QTRLE`, 3840 by 2160, `use_autosplit = True`, frame range 1–4000, and the **Workbench** engine rather than Cycles so that a couple of thousand frames could be reached at all — that last change is a departure from the set-up used everywhere else here and is recorded because the engine is not what this test is asking about. The render was watched and killed with `SIGKILL` the moment a second video file appeared. What came back:

> `RUNSPLIT 1912.7 s: a second video file has appeared: ['vid_0000001-4000.mov', 'vid_0010001-4000.mov'] -- sending SIGKILL to pid 139544`
> `RUNSPLIT exit code: -9`
> `RUNSPLIT files left: [('vid_0000001-4000.mov', 2006829677), ('vid_0010001-4000.mov', 36)]`

Readback: the first file `loaded ok | frame_duration: 2179 | size: (3840, 2160) | fps: 24.0`; the second file `loaded ok | frame_duration: 0 | size: (256, 256) | fps: 0.0`. The other three versions gave the same two files with the same two names and the same shape of answer. The control that the split really happened is the file count: one file would have meant the condition never bit, and there were two. **The Creator's Mac was not used for this test**, because each run writes over two gigabytes and takes between thirty and sixty-seven minutes; macOS is untested for autosplit and that is a gap rather than an answer.

**What these tests do not settle.** Nothing was run inside Blender's own window, so nothing here says what pressing Esc or clicking the cancel button leaves. Nothing was run on Windows. The reader used was Blender itself, so a frame count is what Blender's movie-clip reader made of the file rather than what every player would. And every run used one scene, one codec and one frame rate.

### Record of where I looked

Searched across the whole of the 4.5 manual and the whole of the 4.5 Python API documentation as plain text, plus all nineteen 4.5 release-note files and all of Cycles' 4.5 files.

Terms searched, whole-corpus, case-insensitive: `interrupt`, `abort`, `cancel` / `cancelled` / `canceling`, `stop the render`, `stop rendering`, `stops rendering`, `prematurely`, `midway`, `part-way`, `partway`, `terminate`, `killed`, `shut down`, `shutdown`, `crash`, `power outage`, `recover`, `resume`, `re-render`, `corrupt`, `truncate`, `unplayable`, `playable`, `incomplete`, `unfinished`, `partially written`, `half-written`, `partial`, `closing`, `close the file`, `not be closed`, `finalize` / `finalise`, `flush`, `muxer`, `container`, `moov`, `trailer`, `write the header`, `file header`, `stream header`, `encoder`, `written incrementally`, `appended`, `placeholder`, `overwrite`, `ffmpeg`, `movie`, `video`, `file output`.

Pages opened and read in full or in the relevant part:

- https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — the Output and Encoding panels, including the "Tips" block that carries the unplayable-file warning. This is the single most relevant page found.
- https://docs.blender.org/manual/en/4.5/render/output/animation.html — Rendering Animations, including the "Hints" block about the computer turning off mid-render.
- https://docs.blender.org/manual/en/4.5/render/output/introduction.html — cancel-and-resume for image formats.
- https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html — Compositing and Sequencer toggles, to check whether the two-output case is described there. It is not; the "Pipeline" section is an unfilled "Todo".
- https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html and .../composite.html and .../index.html — the Compositor's output nodes.
- https://docs.blender.org/manual/en/4.5/files/media/video_formats.html — containers, codecs, and the "Known Limitations" section.
- https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html and .../render.html — every render-related argument and every command-line rendering example.
- https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — the Render menu entries.
- API: https://docs.blender.org/api/4.5/bpy.app.handlers.html (the render handlers, including `render_cancel`, "on canceling a render job", and `render_write`, "on writing a render frame (directly after the frame is written)" — neither says anything about the output file's state); https://docs.blender.org/api/4.5/bpy.ops.render.html (`render(animation=…)`); https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html (`is_movie_format`, `use_overwrite`, `use_placeholder`, `frame_path`); https://docs.blender.org/api/4.5/bpy.types.FFmpegSettings.html (every setting, including `use_autosplit`, "Autosplit output at 2GB boundary"); https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html ("Write image file to disk", and its properties). All read 2026-07-30. None of these mention what an interrupted write leaves.

Pictures opened: `_images/render_output_properties_output_panel.png` and `_images/render_output_properties_output_video-encoding-panel.png`, both from https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30. **LABEL: documented**, for what is visible in them — in the first, a File Format row reading "PNG"; in the second, a File Format row reading "FFmpeg video" and a Container row reading "Matroska". Neither shows anything about interruption, and nothing further is taken from either here.

*(Citation check, 2026-08-09 — Opus 5.* The Method treats a picture in the manual as a document, to be named with its page, described by what is visible, and labelled. This record carried no label. Added, and the visible file-format rows named. Changed by the citation check.*)*

Release notes checked: all of https://developer.blender.org/docs/release_notes/4.5/ , by searching every file for the terms above. The only video-related entries are additions and fixes unrelated to interruption — a ProRes writing feature and a Media Type setting under Images & Movies (https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/), FFV1 gaining higher bit depths (https://developer.blender.org/docs/release_notes/4.5/rendering/), and path templates being enabled on the render output path and the File Output node's paths (https://developer.blender.org/docs/release_notes/4.5/core/). Two bug-fix lines mention videos — "Blender corrupts videos saved in the image editor" and "Ffmpeg settings get lost when switching the output file format" (https://developer.blender.org/docs/release_notes/4.5/bugfixes/) — but the first is about saving from the Image Editor and the second about a settings panel, so neither answers this question. All read 2026-07-30.

Ruled out and why: Cycles' own files (`Blender Sources/cycles_4.5`) contain nothing about video, movie files, containers, muxing or the File Output node. I searched every Python file there for `ffmpeg`, `movie`, `video`, `mux`, `container`, `file_output` and `output_file`; the only hits are an unrelated temporary `.oso` file in `osl.py` and `output_filepath` properties on two operators in `operators.py` — Denoise Animation at line 48 and Merge Images at line 136. Neither concerns video output. Nothing in these files touches video containers or the writing of the render's output file, so this source cannot answer the question.

*(Citation check, 2026-08-09 — Opus 5.* Two corrections. The two operators were called "OSL-related"; they are Denoise Animation and Merge Images, and the OSL code is in a different file, `osl.py`. And the closing clause read "Writing the render's output file is not Cycles' work", which says who does the work and is carried by nothing in these files — the honest finding is the absence. Both corrected to the files. Changed by the citation check.*)*

---

## Things a person with Blender open could settle in a few minutes

*(Hand tests, 2026-08-09 — Opus 5. All five items on this list were run on 2026-08-09 and their answers are in **Hand tests, 2026-08-09 — observed** above, so the list is pruned to what is genuinely still open. The five were: item 1, what the video file looks like when a render is stopped part-way — see (d); item 2, the same stopped at the very end — see (b); item 3, the same from outside the interface, cancelled and killed — see (d), where all three of `SIGINT`, `SIGTERM` and `SIGKILL` were run; item 4, whether Blender writes the video as it goes — see (a), where the file held 48 bytes throughout and reached its full size only at the end; item 5, whether the File Output images are complete when the video is not — see (c) and (e).)*

*(Hand tests, second batch, 2026-08-09 — Opus 5. Item 5 stays struck, but the answer at (e) has since been run again, ten times per signal on both machines, and (e) now carries three sets of runs rather than one. Anybody following item 5 should read all of (e), not just its first paragraph.)*

**Still open — anything needing eyes on a window.** Items 1 and 2 above were written as somebody pressing Esc. What was run instead was a signal sent from outside, because the rented Linux machine has no display server and the Creator ran nothing. So it is not known whether a stop from inside the window leaves the same thing as a signal from outside. The same reason covers how the Overwrite and Placeholders boxes are drawn when the output is a video; whether those settings can be held and changed with a video output was answered from a terminal instead, at (k).

**Answered on Linux, untested on the Mac — Autosplit.** What an interruption leaves when the file has already split into more than one was run on the rented Linux machine and is at (l) above: the file already split away held every frame that went into it, and the one being written was left at 36 bytes holding none. It was not run on the Creator's Mac, because each run writes over two gigabytes and takes between thirty and sixty-seven minutes, so macOS is untested for this one.

**Still open — Windows.** Untested rather than answered: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

2026-08-09 — Opus 5 (Claude Code) — Frames, Blender 4.5
