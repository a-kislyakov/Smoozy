# Frames — later questions, third pass (Blender 4.5)

**Sources consulted, and when.** Blender 4.5 manual, Blender 4.5 Python API documentation, Blender 4.5 release notes, and the Cycles 4.5 files, all as supplied in this project's `Blender Sources/` folder. The snapshot was taken on 2026-07-30, and that is the date read I give for everything below. I did the reading on 2026-08-09.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 12 was assigned to me. The rest of the sheet is untouched.

---

## 12. When one render writes one video and separate image files over the same frame range, what does an interruption leave if all the images have been written but the video has not finished closing?

### A note on the question before the answer

The question is ambiguous about *how* one render comes to write both a video and separate image files. I answered the reading I believe is meant: the scene's own render output is set to a video format, and one or more **File Output** nodes in the Compositor write image files during the same render, over the same frame range. That is the only arrangement the sources describe in which a single render produces both kinds of file.

The other reading is the mirror image of it — the scene's render output set to an image format while a File Output node writes the video. The sources point away from that reading but do not close it. The API describes the File Output node as "Write image file to disk" (Python API, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html, read 2026-07-30), and the manual's picture of the node shows a single image input labelled with an image format. But the 4.5 release notes say a Media Type setting that can be Video was added to the File Output node's format settings, so I cannot rule the second reading out from what I read. Whichever reading is taken, my answer below is the same, because the sources do not describe what an interruption leaves in either case.

### LABEL: not in the sources

The sources do not say what an interruption leaves behind when one render is writing a video and separate image files at the same time. They never discuss the two outputs together, and they never describe the state of a half-written video file in enough detail to answer the question — not whether the file exists, not whether it holds the frames written before the interruption, not whether it can be opened.

What the sources *do* say, which is less than the question asks:

**One.** A video file that a render did not finish may be unusable, and the remedy given is to render everything again from the start.

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30.

Note the hedge in the source's own words — "might become unplayable". The manual does not commit to what the file *is* after an interruption, only that it may not play.

**Two.** Image files that a render has already written survive an interruption, and a later render can carry on from where it stopped. The manual says this only about renders whose output is an image format.

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

> "Just disable the Overwrite option to start rendering where you left off."

Manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html, read 2026-07-30.

> "For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

Manual, https://docs.blender.org/manual/en/4.5/render/output/introduction.html, read 2026-07-30.

**Three.** The File Output node writes one image per frame as the render works through the frames.

> "This node writes out an image, for each frame range specified, to the filename entered, as part of a frameset sequence."

> "The image(s) will be saved on rendering, writing to the current frame. An entire sequence of images will be saved, when an animation is rendered."

Manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html, read 2026-07-30.

**Four.** Both outputs can exist in one node tree at once: the Composite node feeds the render's own output, and the File Output node writes files of its own from anywhere in the tree.

> "The Composite node defines the final output of the Compositor. It is the node where the image result is sent to the renderer or image output after rendering."

Manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/composite.html, read 2026-07-30.

> "This node can be used as a way to automatically save the image after a render; In addition, since this node can be hooked in anywhere in the node tree, it can also save intermediate images automatically."

Manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html, read 2026-07-30.

What none of these do is put the two together. No page I found says the File Output node still writes when the render's own output is a video, and no page says what the video file contains when the render stops after the last frame is composited but before the container is finished. The phrase in the question — "not finished closing" — has no counterpart anywhere in the manual, the API documentation, the release notes, or the Cycles files. The word "close", applied to a video or output file, does not appear.

### Hand tests, second batch, 2026-08-09 — observed

*(Added by the second batch of hand tests, 2026-08-09 — Opus 5. This document was written after the first batch of hand tests had been run, and it does not carry them. **The eleven earlier tests and their results are not in this document; they are in `Audits/Blender Research/Frames/Hand tests — later questions.md`, entries 1 to 11, and in the other findings documents for this round.** What is below is only the one test the citation check sent back for a second batch, and only what that test showed. The labels above still stand, because they are statements about the documentation and the documentation has not changed.)*

**The question this test answers.** Whether the last image file on disk is whole when a render is stopped. It was run because three earlier sets of runs disagreed about it. The test is entry 12 of the hand-test list, and it was run **ten times for each of `SIGINT`, `SIGTERM` and `SIGKILL`, in Blender 4.5, 5.0, 5.1 and 5.2, on both machines — 248 runs in all**, on **2026-08-09**.

**The two machines, in full.** *The rented Linux machine* — rented on Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, an NVIDIA GeForce RTX 5060, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, no display server at all; Blender 4.5.12 at `/opt/blender/4.5/blender`. *The Creator's Mac* — a MacBook Pro with an M1 Pro chip, macOS 26.5.2; Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`. Neither speaks for the other, so the two are recorded separately. Every run used `-b --factory-startup` and a fresh throwaway `BLENDER_USER_RESOURCES` folder, so nothing of the Creator's own setup reached a result, and no window was open on either machine.

**What was built, and to what.** A blend-file made by script: the scene's own output set to `FFMPEG`, container MPEG-4, codec H.264, writing into an empty folder; the compositor given a Render Layers node feeding both a Composite node and a File Output node, the node writing PNGs into a second empty folder; Cycles on the processor with adaptive sampling off; frames 1 to 20 at 480 by 270 and 24 frames a second. The four versions were run side by side on each machine, so each Blender was given a fixed thread count: **200 samples at four threads on the rented Linux machine**, **150 samples at two threads on the Creator's Mac**. That gave 4.2 to 4.9 seconds a frame on Linux and 4.9 to 5.5 seconds a frame on the Mac. The image folder, the video folder and the settings folder were deleted and made again before every single run.

**What was done.** `blender -b --factory-startup job.blend -a` was started, the image folder was watched in a loop polling every millisecond, and the signal was sent the instant the tenth PNG file appeared — measured across the runs, between 0.000013 and 0.001426 seconds after the file appeared. Blender's own printing shows where it was at that moment: `Fra:10 … | Finished` and then `Fra:10 … | Compositing`, which is when the File Output node writes.

**What came back, for Blender 4.5.** On the **rented Linux machine**, where a complete image in the control run was 93602 bytes: the tenth image, `Image0010.png`, was 93602 bytes in all ten `SIGINT` runs, and **0 bytes in all ten `SIGTERM` runs and all ten `SIGKILL` runs**. One line word for word, the first `SIGTERM` run, with the captured Blender log left out of the quote:

> `RUN {"workdir": "w_4.5", "sig": "SIGTERM", "tenth_seen_at": 56.57370328903198, "signal_sent_at": 56.57418894767761, "exit": -15, "n_images": 10, "images": [["Image0001.png", 93602], … ["Image0009.png", 93602], ["Image0010.png", 0]], "videos": [["vid0001-0020.mp4", 48]]}`

On the **Creator's Mac**, where a complete image was 93588 bytes: 93588 bytes in all ten `SIGINT` runs; short in all ten `SIGTERM` runs, at 0, 16384, 0, 16384, 0, 0, 0, 0, 40960 and 0 bytes in run order; and short in all ten `SIGKILL` runs, at 0, 0, 16384, 0, 0, 16384, 4096, 0, 0 and 0 bytes.

**The controls.** In every one of the sixty 4.5 runs the first nine images were at the full size and exactly ten images were on disk, so the render was stopped rather than allowed to finish; the exit statuses were `1` for `SIGINT`, `-15` for `SIGTERM` and `-9` for `SIGKILL` throughout. The full size itself comes from a complete uninterrupted run of the same file on the same machine, which left all twenty images at one size — without it, "short" would mean nothing. And the test can come out the other way, because it did: `SIGINT` gave ten runs with every image whole, in this version and in the other three, on both machines.

**This settles a disagreement rather than closing the point.** Three sets of runs now exist and all three stay on the record. The first hand-test batch recorded, for 4.5, a 0-byte tenth image after `SIGKILL` and full images after `SIGTERM` and `SIGINT`. The citation check, repeating it, recorded full images after every signal on Linux and a 0-byte tenth image after `SIGTERM` on the Mac. This batch matches the first on `SIGKILL`, matches the citation check's Mac run on `SIGTERM`, and matches neither where they say a signal left the image whole. None of them is being struck and none is being picked as the answer.

**What this test does not settle.** Nothing was run inside Blender's own window, so nothing here says what pressing Esc or clicking the status bar's cancel button leaves. Nothing was run on Windows, because no Windows machine has run it. And this test looked only at the image files — what the video file held is a different test, and it is in the hand-test list rather than in this document. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Not covered

- Whether the half-written video file exists on disk at all after an interruption, and if it does, whether it holds the frames rendered up to that point.
- Whether the video file can be repaired, or read by anything, or is simply discarded.
- Whether the images written by a File Output node are affected in any way by the video output failing to close — my answer assumes they are separate writes because the sources describe them as separate writes, but the sources never test that assumption against an interruption.
- Whether Blender writes anything different on an orderly cancel (a person or a script stopping the render) than on a hard stop (the process being killed, or the machine going down). The sources make no distinction between the two anywhere I looked.
- What Autosplit Output does to the picture. The manual says a video over 2 GiB is split into more than one file, so an interruption could leave several video files rather than one, but it says nothing about their state. Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30: "This will automatically split the output into multiple files after the first file is 2GiB in size."

### Outside the interface

The same answer holds: the sources say nothing about this either way, whether Blender is driven by a person clicking or started and controlled from outside its own window. Nothing I read distinguishes the two for output files. The manual's page on rendering from the command line lists the arguments for rendering a whole animation in the background and says nothing about stopping one part-way, nor about what is left behind if you do. Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html, read 2026-07-30. The only stopping mechanism the manual describes at all is a button in the window: "The task can be aborted by clicking the cancel button ()." Manual, https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html, read 2026-07-30. That sentence is about the interface, and I found no matching statement for a render driven from outside it. The Python API offers handlers that fire when a render job is cancelled or completes — "on canceling a render job" and "on completion of render job" (Python API, https://docs.blender.org/api/4.5/bpy.app.handlers.html, read 2026-07-30) — which tells you the two endings are distinguished internally, but the documentation does not say what either ending does to a video file that is open.

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. Part of this line is now answered by observation, for the image files only. Every one of the 248 runs recorded above started Blender with `-b --factory-startup` and no window, on the Creator's Mac and on the rented Linux machine, which has no display server and cannot open one. So what those runs show **is** the driven-from-outside behaviour for Blender 4.5, at least for the File Output node's images: a `SIGINT` — which is what Ctrl-C in a terminal sends — left the last image whole every time, and a `SIGTERM` or `SIGKILL` left it short every time. What is still not settled is the other direction, whether a stop from inside the window leaves the same thing; that needs eyes on a window, the rented machine cannot open one, and the Creator ran nothing this round. The sources' silence, recorded above, stands as a statement about the documentation.*)*

### Operating system

The sources do not say. I found nothing that makes this behaviour differ by operating system, and nothing that says it is the same on all of them. The nearest thing to an operating-system statement in this area is about which video formats play where, not about interrupted writes: "Codecs are used to encode and decode the movie, and so must be present on both the encoding machine (Blender) and the target machine." Manual, https://docs.blender.org/manual/en/4.5/files/media/video_formats.html, read 2026-07-30. That is about playback compatibility and does not bear on the question.

*(Hand tests, second batch, 2026-08-09 — Opus 5 — **observed**. Two of the three operating systems are now answered, in part and separately, by the runs recorded above. The test was run on the Creator's Mac, a MacBook Pro with an M1 Pro chip running macOS 26.5.2, and on the rented Linux machine, Ubuntu 22.04.5 LTS on Vast.ai with no display server. **The two agree on the shape and differ on the detail.** On both, a `SIGINT` left the last image whole every time and a `SIGTERM` or `SIGKILL` left it short every time. Where they part is what "short" was: on the rented Linux machine every one of the eighty short files, across all four versions, was exactly **0 bytes**, while on the Creator's Mac sixty-nine were 0 bytes and eleven were a whole number of 4096-byte blocks — 4096, 16384, 24576, 40960 or 73728 — all far short of a complete image. **[Recount, 2026-08-09 — this sentence used to say seventy-two and eight, and that was a miscount. Counting the run-by-run figures themselves — this version's ten runs per signal as printed on this page, and the other three versions' in the hand-test record these findings were written from — gives sixty-nine empty and eleven block-sized. A recount of figures already on the record: nothing was re-run, and no new fact is added.]** That is the whole of what these runs say about operating systems, and it covers only the image files. **Windows is untested**, not answered: no Windows machine has run it. The sources' silence, recorded above, stands as a statement about the documentation.*)*

---

## Where I looked

I searched the whole manual, the whole Python API documentation and the whole set of release notes at once, rather than opening likely pages, because the answer I expected — and got — was silence, and silence has to be shown.

**Terms searched across the manual, the API documentation and the release notes:** file output, movie, video, ffmpeg, encoder, encode, container, muxer, stream, media type, close the file, closing the file, finalize, finalise, flush, trailer, moov, header, cancel, abort, interrupt, terminate, kill, stopped, stop rendering, stop the render, Esc, crash, power outage, shut down, resume, restart, re-render, recover, corrupt, truncated, partial file, half-written, unfinished, incomplete, not finished, prematurely, playable, unplayable, written to disk, overwrite, placeholder, is_movie_format.

**Pages I opened and read in full:**

- https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — the Output and Encoding panels. This is where the one relevant sentence lives ("the file might become unplayable"). Also the source for Overwrite, Placeholders and Autosplit Output.
- https://docs.blender.org/manual/en/4.5/render/output/animation.html — Rendering Animations. Covers the Direct Approach (straight to a movie file) against the Frame Sequence approach, and what survives when the computer goes down mid-render. It never mentions doing both in one render.
- https://docs.blender.org/manual/en/4.5/render/output/introduction.html — cancelling and resuming, stated only for image file formats.
- https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html — the File Output node.
- https://docs.blender.org/manual/en/4.5/compositing/types/output/composite.html — the Composite node.
- https://docs.blender.org/manual/en/4.5/compositing/types/output/index.html — the list of output nodes, to be sure there is no fourth one that writes files.
- https://docs.blender.org/manual/en/4.5/files/media/video_formats.html — every container and codec Blender supports. No mention of what a half-written file of any of them looks like.
- https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — rendering from the command line.
- https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — the Render menu.
- https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html — the File Output node's settings.
- https://docs.blender.org/api/4.5/bpy.app.handlers.html — the render handlers, including render_cancel, render_complete and render_write.
- https://docs.blender.org/api/4.5/bpy.ops.render.html — the render operators.
- https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — is_movie_format, use_overwrite, use_placeholder, frame_path.
- https://docs.blender.org/api/4.5/bpy.types.ImageFormatSettings.html — the format settings shared by the render output and the File Output node.
- https://docs.blender.org/api/4.5/bpy.types.FFmpegSettings.html — searched for close, cancel, abort. Nothing.
- https://developer.blender.org/docs/release_notes/4.5/core/, https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/, https://developer.blender.org/docs/release_notes/4.5/rendering/, https://developer.blender.org/docs/release_notes/4.5/cycles/, and the bug-fix list — searched for ffmpeg, movie, file output, encode, cancel, abort, interrupt. The only hits touching this area are that ProRes writing was added, that the File Output node's paths now accept template expressions, and that a Media Type setting was added to the Render Output, File Output and Image Saving format settings. None says anything about interruption.

**Pictures I opened:**

- `_images/render_output_properties_output_panel.png`, from https://docs.blender.org/manual/en/4.5/render/output/properties/output.html. It shows the Output panel with the file format set to PNG. Overwrite and Placeholders are both visible and Overwrite is ticked. The picture does not show the panel with a video format chosen, so it settles nothing about how those two settings behave for a video.
- `_images/node-types_CompositorNodeOutputFile.webp`, from https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html. It shows a File Output node with a Base Path and a single Image input marked WebP — an image format.

**Cycles' own files.** I searched all of `Blender Sources/cycles_4.5` for ffmpeg, movie, file_output, encode_video and cancel. The only hits for "cancel" are in `operators.py` — a `{'CANCELLED'}` return and a comment reading `# TODO: support cancel and progress reports.` — neither of which is about writing output files. The copy of Cycles supplied here is its kernel, its utility code, its shaders and its Python interface; it contains nothing that writes a video file, which is consistent with the writing of the video being Blender's own work rather than the render engine's. I have ruled Cycles out as a source for this question.

**What I ruled out and why.**

- `https://docs.blender.org/manual/en/4.5/troubleshooting/recover.html` — the only manual page about recovering after a crash or power cut. It is entirely about blend-files, auto-save and undo. It says nothing about render output. Ruled out.
- The Video Sequencer pages — they describe rendering a video *from* an image sequence as a second, separate step. That is a different render from the one the question asks about. Ruled out.
- The fluid and physics cache pages, which came up repeatedly in searches for "resume" and "abort" — these are about simulation baking, not rendering frames to files. Ruled out.
- `--disable-abort-handler` in the command-line arguments — I checked it. It is one line, "Disable the abort handler", with no further explanation and nothing tying it to output files. Ruled out as unable to support any claim.

---

## Things a person with Blender open could settle in a few minutes

*(Hand tests, second batch, 2026-08-09 — Opus 5. This list is pruned only of what has actually been run, and the numbering is kept so nothing above points at a gap. **All seven items were in fact run on 2026-08-09**, but by the first batch of hand tests, whose results are in `Audits/Blender Research/Frames/Hand tests — later questions.md` and in the other findings documents for this round, not in this one — so they are left standing here rather than struck on the strength of work this document does not carry. What the second batch settled, and what is therefore struck:*

- ***Item 3, the image half.** "Compare what the two leave behind" for a Ctrl-C against an outright kill is answered above for the image files, on both machines and in all four versions, by 248 runs: a `SIGINT` left the last image whole every time, a `SIGTERM` or `SIGKILL` left it short every time. The video half of item 3 is not answered in this document.*
- ***Item 7, in part.** macOS and Linux are compared above for the image half, and they agree on the shape and differ on the detail. **Windows was not tried**: no Windows machine has run it, so Windows is untested rather than answered, for this and for everything else on the list.*
- ***Item 5 needs eyes on a window and was dropped this round.** Whether the Overwrite and Placeholders boxes are *drawn* greyed with a video format chosen is a fact about a panel on a screen. The rented machine has no display server and the Creator ran nothing, so it could not be looked at. The neighbouring question — whether those two settings can be held and changed with a video output in force — was answered from a terminal by the first batch and is recorded in the other documents for this round; that is a different test, not this one.*
- ***Item 2 needs eyes on a window for the part that says "press the cancel button in the status bar",** and that part was dropped for the same reason. Stopping the render from outside instead is what the runs above and the first batch did.*

*Nothing else on the list is struck.)*

Each of these is a check I could not make from the documents.

1. **Does a File Output node write images when the render's own output is a video?** Open Blender. In Output Properties, set the file format to FFmpeg Video and pick a container and codec. Set the frame range to 1–20. Turn on Use Nodes in the Compositor, and add a File Output node connected to the Render Layers image socket, with its base path set to an empty folder. Render the animation to the end. Then look in that folder. Twenty image files means yes; an empty folder means no.

2. **What is left when the render is stopped part-way?** Repeat the setup above with a frame range long enough to give you time — say 1–200, with a scene that takes a few seconds a frame. Start the animation render, let it get to about frame 50, then press the cancel button in the status bar. Then look at three things: how many image files the File Output folder holds; whether the video file exists at the render output path; and its size in bytes. Try to open the video file in a player, and note whether it opens, and if it does, how many frames it holds.

3. **The same, but with the process killed rather than cancelled.** Run the same render from a terminal with `blender -b yourfile.blend -a`, let it reach about frame 50, and stop the process — first with Ctrl-C, then, in a second run, by killing it outright. Compare what the two leave behind, on the same three points as above. This is the difference the documents never draw.

4. **Does the video file exist from the start, or only appear at the end?** Start a long animation render to a video format and watch the output folder while it runs. Note whether a file appears immediately and grows, or only appears when the render finishes. Note its size at intervals. This tells you whether an interruption can leave a file at all.

5. **What Overwrite and Placeholders do when the format is a video.** Open Output Properties with the file format set to FFmpeg Video and look at whether the Overwrite and Placeholders checkboxes are shown, greyed out, or absent. The manual's picture only shows them with PNG selected.

6. **Whether Autosplit changes the picture.** With Autosplit Output enabled and a render long enough to pass 2 GiB, interrupt part-way and note how many video files exist and which of them, if any, will play.

7. **Whether the answer differs by operating system.** Run check 2 and check 3 unchanged on Windows, macOS and Linux, and compare the three points recorded each time.

---

2026-08-09 — Opus 5 (Claude Code) — Frames, Blender 4.5
