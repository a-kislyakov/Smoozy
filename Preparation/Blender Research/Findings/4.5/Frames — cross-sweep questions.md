# Frames — cross-sweep questions (Blender 4.5)

Sources consulted, all read on **2026-07-30** (the snapshot date): Blender 4.5 manual, Blender 4.5 Python API documentation, Blender 4.5 release notes, and the Cycles 4.5 files supplied in this folder.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

Questions answered: 13 and 14.

---

## 13. When a render is stopped at the exact moment it is writing a frame's own image file, what is left where it writes?

**LABEL: not in the sources.**

The sources never describe what a frame's output file looks like if the render stops while that file is being written. There is no statement anywhere about a part-written file, a truncated file, a zero-length file, a temporary file that is renamed once writing finishes, or a file that is deleted again on failure. So I cannot say what is left where it writes.

**The question was ambiguous, and I answered both readings the same way.** "Stopped" could mean stopped on purpose (a person cancelling) or stopped by the machine going down. The sources are silent on both, so the answer does not change. "Where it writes" I read as the path that frame's image file is written to; the other reading, the output folder as a whole, is also unanswered.

### What the sources do say near this, and why none of it settles the question

The manual states that frames written *before* the stop survive. It does not extend that to the frame in progress:

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

— Manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30

That sentence sits under the heading "Your computer accidentally turns off in the middle of rendering your movie!" on the same page. It is the closest the manual comes to the question. It speaks only of frames "already rendered", and says nothing about the one being written when the power went.

The manual also says a cancelled render can be picked up again, again without saying anything about the frame that was in flight:

> "For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

— Manual, https://docs.blender.org/manual/en/4.5/render/output/introduction.html — read 2026-07-30

The Python API describes a handler that runs after a frame has been written, and says nothing about a write being interrupted part-way:

> "on writing a render frame (directly after the frame is written)"

— Python API, `bpy.app.handlers.render_write`, https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30

The nearest thing in the sources to a file that exists without holding a finished image is the Placeholders option, and it is only in play when that option is switched on (it is off by default):

> "Create empty placeholder files while rendering frames (similar to Unix ‘touch’)"
>
> "Type:
>
> boolean, default False"

— Python API, `bpy.types.RenderSettings.use_placeholder`, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30

The quoted words say Blender deliberately creates empty placeholder files while rendering frames, in that one mode. That such a file sits at the frame's own output path is a step past those words — the same step I mark as inferred under question 14. And none of it tells us what an interrupted write leaves, with Placeholders on or off.

### Where I looked

- Read in full: manual pages `render/output/introduction`, `render/output/animation`, `render/output/properties/output`, `advanced/command_line/render`, and the Render Options part of `advanced/command_line/arguments`.
- Searched the whole 4.5 manual (all `.txt`) for: `corrupt`, `truncat`, `incomplete`, `partially written`, `partial file`, `zero bytes`, `empty file`, `unreadable`, `damaged`, `half-written`, `unfinished`, `broken file`, `invalid image`, `flush`, `finalize`, `close the file`, `atomic`, `rename`, `temporary file`, `.part`, `in-flight`, `mid-write`, `interrupted`, `written to disk`, `saved to disk`, `writing the file`, `file is written`, `is written to`, `output directory`. The only hits were unrelated (geometry-node maths, text alignment, GPU driver troubleshooting, video proxy timecodes) or the two manual sentences quoted above.
- Searched `cancel`, `abort`, `interrupt`, `stop the render`, `Esc` across the manual's `render/` section: one hit, the `render/output/introduction` sentence quoted above.
- Searched the whole 4.5 Python API documentation for: `zero bytes`, `empty file`, `unreadable`, `damaged`, `half-written`, `unfinished`, `partially`, `use_overwrite`, `use_placeholder`. Read `bpy.app.handlers` in full and the `use_overwrite` / `use_placeholder` / `frame_path` entries of `bpy.types.RenderSettings`.
- Searched the 4.5 release notes (all files) for `overwrite`, `placeholder`, `output path`, `file output`, `write`, `render output`, `EXR`, `partial`. Nothing about interrupted writes. The only near-miss is a listed bug fix, "Placeholder output files wrong name" (https://developer.blender.org/docs/release_notes/4.5/bugfixes/, read 2026-07-30), which concerns naming, not interruption.
- Searched the Cycles 4.5 files for `placeholder`, `overwrite`, `partial file`, `atomic`, `temp file`, `.tmp`, `output_driver`, `OutputDriver`, `write_render_tile`, `save…image`, `image_output`. The only hits were the unrelated `source/util/atomic.h` (thread-safe arithmetic on the GPU) and a comment in `operators.py`. The supplied Cycles tree holds `source/kernel` and `source/util`, the add-on's Python files, a `shader` folder of 121 files (104 of them compiled OSL shaders) and a `license` folder — none of it is file-writing or session code, so it cannot answer this. *(Corrected by the citation check of 2026-08-11: the sentence had said the tree contains only the kernel, the utilities and the Python files, leaving out the `shader` and `license` folders.)*

### What running it settled — the hand tests of 2026-08-11

**LABEL: observed.** The machines, the settings folders and the list are described under question 14 below. Everything quoted here is Blender 4.5.12's own output. Results from the two machines are given separately and are never merged.

**Blender writes the frame straight to its final name, and the file grows in place.** An animation of three frames at 4096 by 4096, OpenEXR at 32 bits with no compression, watched with a listing of the whole folder every two milliseconds. Every name that ever appeared was a final frame name — no `.tmp`, no `.part`, no dot-prefixed name, nothing hidden:

> ```
> WATCH    0.000  <empty>
> WATCH    1.285  frame_0001.exr=1635102
> WATCH    1.288  frame_0001.exr=3941150
> WATCH    1.291  frame_0001.exr=6664990
> WATCH    1.294  frame_0001.exr=8856350
> ```

and 112 distinct sizes later it reached 268,501,790 bytes. — Creator's Mac. On the rented Linux machine the same run began `WATCH 1.270 frame_0001.exr=1183744` and ended at 268,501,751 bytes, again with only the three final names ever seen. **The two machines agreed.**

**The write is announced as a step of its own.** Blender 4.5 prints, at the start of a line and with no prefix:

> ```
> Saved: '…/frame_0001.exr'
> Time: 00:01.20 (Saving: 00:00.34)
> ```

— Creator's Mac; the rented Linux machine printed the same shape, `Time: 00:01.22 (Saving: 00:00.21)`. In Blender 5.0, 5.1 and 5.2 the same two lines carry a leading elapsed time and the word `render`, which is the only difference in what is printed.

**How the stop is delivered decides what is left.** Three runs, each stopped at the instant frame 2's file first appeared:

- **An interrupt (`SIGINT`)** did **not** cut the write. Blender finished writing the frame, then stopped: `Sent an internal break event. Press ^C again to kill Blender`, `Saved: '…/frame_0002.exr'`, `Time: 00:01.12 (Saving: 00:00.33)`, exit code 1. Frame 2 was 268,501,794 bytes and opened as a 4096 by 4096 image.
- **A terminate (`SIGTERM`)** left a file at frame 2's path of **0 bytes**, exit code -15, with no `Saved:` line. Blender would not open it: `OPEN loaded | size: (0, 0) | channels: 0 | depth: 0 | has_data: False`.
- **A kill (`SIGKILL`)** left a file of **25,379 bytes**, exit code -9, again unopenable.

— Creator's Mac. On the rented Linux machine: the interrupt again finished the write and left a whole 268,501,755-byte frame; the terminate left **0 bytes**; the kill left **32,772 bytes**. **The two machines agreed on the pattern.** The exact leftover size is a race and differs run to run; what does not differ is that a file exists at the frame's own path and does not open.

**The format changes what the leftover looks like, and one format is dangerous.** The same kill, once per format, at 4096 by 4096:

| format | leftover on the Creator's Mac | leftover on the rented Linux machine | does it open |
| --- | --- | --- | --- |
| PNG | 0 bytes | 0 bytes | no |
| JPEG | 0 bytes | 4,096 bytes | **on Linux, yes — reported as 4096 × 4096** |
| OpenEXR | 0 bytes | 16,384 bytes | no |
| OpenEXR multi-layer, two view layers | 17,669 bytes | 0 bytes | the reader cannot say — see below |
| TIFF | 1,306 bytes | 1,306 bytes | **yes, on both — reported as 4096 × 4096, has_data True** |

A part-written TIFF, and on Linux a part-written JPEG, opened in Blender and reported the full picture size. So **a stopped-mid-write file is not always distinguishable from a finished one by asking whether it opens.** Complete frame 1 was 5,534,881 bytes (TIFF) against the 1,306-byte leftover, so size still tells them apart. — 2026-08-11.

**The multi-layer EXR case, and a limit of the test.** With two view layers, one file per frame is written, 536,937,733 bytes on the Creator's Mac and 536,937,712 on the rented Linux machine, and a kill left 17,669 bytes and 0 bytes. Blender's own `bpy.data.images.load` reports `type: MULTILAYER | size: (0, 0) | has_data: False` for the **complete** file as well as the part-written one, so for that one format the reader used here cannot tell whole from part. That is a limit of the test, not a finding about the file.

**The operating system had already put everything on disk.** After a kill, the folder was listed at once and again sixty seconds later. Both listings were identical, byte for byte and time for time, on both machines and in all four versions. Nothing was still being held back.

**With Placeholders on, a file of zero bytes is put at the frame's path before that frame is rendered — one frame at a time.** A four-frame render with Overwrite off and Placeholders on:

> ```
> WATCH    0.000  <empty>
> WATCH    0.288  frame_0001.exr=0
> WATCH    1.278  frame_0001.exr=692224
> ```

Frame 1's file appeared at **0 bytes** a second before any pixels were written to it, and frame 2's file did not appear until frame 1 was finished — 1.635 seconds in on the rented Linux machine, and 1.567 seconds in on the Creator's Mac. No file ever appeared for a frame the render had not reached. That settles the step this document marks as inferred under question 14. Stopping the run once frame 2's file had grown past zero left 78,623 bytes on the Creator's Mac and 36,864 on the rented Linux machine, neither of which opened. **The two machines agreed.**

**The JPEG preview copy exists and is written after the frame.** With `use_preview` on and OpenEXR output, the folder held a `.jpg` beside every `.exr`, and the run printed `Saved: '…/frame_0001.exr'` immediately followed by `Saved: '…/frame_0001.jpg'`. — Creator's Mac and the rented Linux machine, agreeing.

**A Compositor File Output node writes before the render's own frame file.** With a File Output node writing OpenEXR into a second folder and the Output panel writing PNG, a complete frame printed the node's save first and the Output panel's second. Stopped the instant the node's own second file appeared, the folder held the node's `Image0002.exr` at 610 bytes on the Creator's Mac and 0 bytes on the rented Linux machine — unopenable in both — and **no Output-panel file at all for that frame**. — 2026-08-11.

**Cancelling from inside Blender's own window did not stop the render.** On the rented Linux machine only, a forty-frame render started from a timer in a real window, with the render window holding the keyboard (`the window that has the keyboard: Blender Render`), and the Escape key sent to every Blender window repeatedly for ten seconds with `xdotool`. Fifteen seconds after the first Escape the render was still going, and it went on writing whole frames — frames 18 through 25 were written after it. The same happened in all four versions. What this shows is that the render did not stop; it cannot show that Blender received the key, so it does not settle what an Escape pressed by a person would do.

**The citation check of 2026-08-11 repeated this and got the opposite, and both runs are recorded rather than one replacing the other.** Following the procedure written above on the rented Linux machine, the render **stopped**: sixteen frames stood in the folder when the first Escape was sent and sixteen stood there fifteen seconds after the last one, in Blender 4.5, run twice; in 5.2 it was fourteen and fourteen. A control identical in every respect except that no Escape was ever sent went from sixteen frames at twenty-five seconds to thirty-six at fifty seconds in 4.5, and from fourteen to thirty-three in 5.2 — so without the Escape the render kept going, and with it the render stopped. That is a question, not an answer. It is item 22 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, to be run again, and until it is run nothing downstream should treat either result as the fact.

**A machine going down loses even frames it had already reported saving.** On the rented Linux machine only, the same three-frame OpenEXR render was started and the machine was reset at the kernel — `echo b > /proc/sysrq-trigger`, with nothing flushed — the instant frame 2's file appeared. The machine restarted, and afterwards the output folder was **empty**: no frame 1, though the render had reported it saved seconds earlier, and no frame 2. The run's own log file was 0 bytes and the blend-file the run had built was 0 bytes. A plain `reboot` had been tried first and the machine came back from it, so the loss is the reset's doing and not a failure of the step. This is the *unflushed write* case, and it comes out differently from every kill above. *(The citation check of 2026-08-11 did not repeat this: the kernel reset was refused by the permission system of the machine that check ran on. The result stands on its single run, and is item 25 on the hand-tests list.)*

### What the second batch of hand tests settled — 2026-08-12

**LABEL: observed.** Four more tests — entries 22 to 25 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md` — were added by the citation check of 2026-08-11: three because a repeat came back differently from a run recorded above, and one because a procedure could not be followed as written. They were run on 2026-08-12 on the machines each entry names. **Machine one, the Creator's Mac:** a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card. **Machine two, a rented Linux machine:** Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Every run had a settings folder of its own and was started with `--factory-startup`. The two machines' results are given separately and never merged. **Where a result differs from one written above, both stand and neither has been chosen.**

**Escape sent to Blender's own window stopped the render — entry 22.** On the rented Linux machine only, that being the one machine of the two with a screen this round could drive. A forty-frame render at 480 by 480 with eighty samples, PNG, into a fresh empty folder, started in a real window from a timer four seconds after launch. At twenty-five seconds the folder was counted and the machine was asked which window held the keyboard:

> ```
> COUNT at 25s, before any Escape: 30
> xdotool getwindowfocus getwindowname: Blender Render
> xdotool search --name Blender: 62914562 62914691
> ```

Escape was then sent to both of those windows and to the focused window, twenty-six times over ten seconds, and the folder counted again:

> ```
> COUNT at 35s, right after the Escape window: 30
> COUNT at 50s, fifteen seconds after the last Escape: 30
> ```

A control in the same sitting, identical except that no Escape was ever sent, went `COUNT at 25s: 31`, then 40, then 40. In Blender 4.5 the test was run twice and gave 30, 30 and 30 both times. The other three versions came out the same way: 5.0 stayed at 35 where its control reached 40; 5.1 went 37 to 38 and stopped, where its control reached 40; 5.2 went 32 to 33 and stopped, where its control reached 40. Every control reached all forty frames of the range, which is what ended its own climb; every test stopped short of forty and did not move again. **This agrees with the citation check's repeat and disagrees with the run recorded above under *Cancelling from inside Blender's own window*. Both stand.** It still cannot show that Blender received the key, so what a person pressing Esc with their own hand would get is not settled by it. One line was printed once during the 4.5 test and made no difference to the counts: `X Error of failed request:  BadWindow (invalid Window parameter)`, from the tool sending the key.

**A stop leaves a different number of bytes almost every time, for JPEG and for TIFF — entry 23.** Ten repeats per format per version per machine, each into a fresh empty folder: the three-frame render at 4096 by 4096 was started with no window and killed outright the instant frame 2's file first appeared, then that file's exact size was recorded and Blender's own image loader was asked to open it. In Blender 4.5 on the **Creator's Mac** the ten JPEG leftovers were 8,192, 0, 0, 0, 4,096, 0, 4,096, 4,096, 4,096 and 0 bytes; five of the ten opened and reported 4096 by 4096, against a complete JPEG of 4,128,006 bytes. The ten TIFF leftovers were 1,286, 1,306, 1,306, 0, 1,306, 8, 0, 1,278, 254 and 8 bytes; three of the ten opened, against a complete TIFF of 5,534,881 bytes. On the **rented Linux machine** the ten JPEG leftovers were 0, 0, 0, 4,096, 0, 4,096, 0, 4,096, 0 and 4,096 bytes, four opening, against a complete JPEG of 4,113,528 bytes; the ten TIFF leftovers were 1,306, 8, 8, 8, 1,306, 8, 1,306, 1,306, 1,306 and 1,306 bytes, six opening, against a complete TIFF of 5,534,204 bytes. **Every cell is a spread.** So the single numbers in the format table above, and the different single numbers the citation check got, are both draws from a race rather than facts about a version or a machine. Two things the repeats also show: a file of 0 bytes never opened, and 1,306 bytes was the commonest TIFF leftover and usually opened but not always — in one repeat in 5.0 on the Creator's Mac, and one in 5.2 on the rented Linux machine, a file of exactly 1,306 bytes did not open.

**Entry 24 was not run in Blender 4.5.** It covers the Compositor File Output node in 5.0, 5.1 and 5.2 only, because the citation check repeated the 4.5 result exactly and the set-up it could not rebuild was the one those three versions use. The 4.5 result recorded above stands as it is.

**The kernel reset, run a second time, left the folder present and empty in every version — entry 25.** On the rented Linux machine only, and run last. A plain `reboot` was tried first and the machine came back in about a minute and a half; a bare kernel reset with nothing running was also issued once and the machine came back from that, so the step itself works. Then, once per version, the three-frame OpenEXR render at 4096 by 4096 was started with no window and the machine was reset at the kernel — `echo b > /proc/sysrq-trigger` — the instant frame 2's file appeared. After the reset, in Blender 4.5:

> ```
> LIST /root/frames-cross-sweep-2/e25/4.5/out
>    <empty>
> ```

The run's own log file did not exist at all, and the blend-file the run had been given survived whole at 435,463 bytes. Frames 1 and 2 were both absent, and frame 1 had been written and reported saved before the reset. The same reset left the folder present and empty in 5.0, 5.1 and 5.2 too; 5.1's run log survived at 0 bytes and the other three had none, and all four blend-files survived whole. **That disagrees with the run recorded above**, which had the folder gone in 5.0, empty in 4.5 and 5.1, holding a zero-byte `frame_0001.exr` in 5.2, and the blend-file at 0 bytes. Both runs stand and neither has been chosen. What came out the same way twice: **no frame's contents survived in any version, including a frame already reported saved.**

**Not covered, after the tests.** What a person pressing Escape or clicking the status bar's cancel button leaves — entry 22 shows the render stopping when the key is sent by a tool, and entry 8 shows it not stopping, and neither run can show that Blender received the key; anything on Windows; and a real power cut as against a kernel reset.

**Not covered.** This answer does not settle whether Blender writes frames straight to their final path or via a temporary file; whether an interrupted write leaves nothing, an empty file, or a part-written one; whether the answer differs by image format (PNG, OpenEXR, multi-layer OpenEXR); or whether operating-system file buffering changes what is on disk at the moment of the stop. It also does not settle whether a "stop" delivered as a cancel differs from one delivered by the machine dying. *(That paragraph describes what the **sources** do not settle, and it stands. Every item in it is answered above by a running Blender.)*

**Outside the interface.** The sources do not say. They say nothing about what an interrupted write leaves inside the interface either, so there is nothing to compare. Nothing in the command-line documentation mentions output-file integrity, and there is no command-line argument relating to overwriting or placeholders. **Brought up to date by the hand tests of 2026-08-11:** every result above was got from a render driven from outside Blender's window, with no window at all, so the answer *holds* outside the interface — that is where it was measured. The one thing tried inside the window is the Escape key, on the rented Linux machine, and it did not stop the render; that leaves what a person clicking or pressing Escape would get unsettled, and it is the only part of this answer that is not established from outside. **Brought up to date again by the second batch of hand tests, 2026-08-12:** the Escape test was run again with a control, on the rented Linux machine, and this time the render **stopped** in all four versions — see entry 22 above. The two runs disagree and neither has been chosen. Everything else in this answer is still established from outside the window, and the window half stays open either way, because no run so far can show that Blender received the key.

**Operating system.** The sources do not say. No page distinguishes operating systems on this point. The only platform word anywhere near it is descriptive, not restrictive: the API describes placeholder creation as "similar to Unix ‘touch’". **Brought up to date by the hand tests of 2026-08-11:** everything above was run on macOS 26.5.2 on an Apple M1 Pro **and** on Ubuntu 22.04.5 LTS on an AMD Threadripper, and the two agreed on every point except the exact number of bytes a stop leaves, which is a race, and one case where a part-written JPEG opened on Linux and not on the Mac. **Windows is untested**, no Windows machine having run it, so this answer covers macOS and Linux and leaves Windows open. **Brought up to date again by the second batch of hand tests, 2026-08-12:** entry 23 ran the JPEG and TIFF stop ten times per version on each machine, and both machines produced a spread of sizes and a mixture of files that opened and files that did not. So the one case where a part-written JPEG opened on Linux and not on the Mac is a draw from that spread and not a difference between the two operating systems. Entries 22 and 25 were run on the rented Linux machine only, for the reasons given with them, so what an Escape does inside a window and what a machine going down leaves are answered for Linux and left open for macOS. **Windows is still untested.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 14. When two renders write into one folder and are started at the same instant, what does each of them produce, and what does the folder hold at the end?

This question has three parts, and they do not share one label. The arrangement itself is documented. What each render produces inside that arrangement is inferred, and I write the step out. The "at the same instant" part is not in the sources at all.

### Part one — the arrangement is documented, and the manual calls it an easy way to work

**LABEL: documented.**

The manual explicitly endorses several renders writing into one folder, and names the two settings that make them share the work rather than duplicate it:

> "Hint
>
> Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

— Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30

The two settings are described on the same page, under the heading "Image Sequence":

> "Overwrite
>
> Overwrite existing files when rendering.
>
> Placeholders
>
> Create empty placeholder frames while rendering."

— Manual, same page and address — read 2026-07-30

And in the Python API, with their defaults:

> "use_overwrite
>
> Overwrite existing files while rendering
>
> Type:
>
> boolean, default True"

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix ‘touch’)
>
> Type:
>
> boolean, default False"

— Python API, `bpy.types.RenderSettings`, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30

The manual's picture of the Output panel shows both as checkboxes, with Overwrite ticked and Placeholders unticked — matching those defaults.

— Manual image `_images/render_output_properties_output_panel.png`, on https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30. What is visible: an "Output" panel with path `/tmp/`, a ticked "Overwrite" checkbox and an unticked "Placeholders" checkbox side by side, plus File Extensions, Cache Result, File Format PNG, Color, Color Depth and Compression.

### Part two — what each render produces, in the arrangement the manual recommends

**LABEL: inferred.**

With Overwrite off and Placeholders on, each render works through the frame range, and each frame ends up rendered by whichever render reached it first; the folder ends up holding one file per frame, not two, and no frame is rendered twice. The renders divide the frames between them by which files are already present.

That covers the arrangement in general. It does **not** cover two renders arriving at the same frame in the same instant, which is the part the question actually asks about — part three below records that as not in the sources, and nothing above should be read as settling it.

**The step I took, spelled out.** The manual never states outright that switching Overwrite off makes a render skip frames whose files already exist. What it states is the consequence:

> "Just disable the Overwrite option to start rendering where you left off."

> "Then disable Overwrite and re-render with Frame Step set to 1."

— Manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30 (the second sentence is under "Animation Preview", about filling in the frames a Frame Step render skipped)

A reader has to join those to "Create empty placeholder frames while rendering" and to the Primitive Render Farm hint to reach the claim above: if a present file makes a render skip a frame, and a placeholder file appears at a frame's path as soon as a render starts that frame, then a second render meeting that placeholder skips that frame and moves to the next. That joining is mine, not the manual's, which is why this is inferred and not documented. The manual asserts the arrangement works; it never explains the mechanism.

Both of those "if"s are mine as well, and neither is small. The manual never says that a present file makes a render skip a frame — only that disabling Overwrite lets you carry on where you left off. And "Create empty placeholder frames while rendering" says nothing about *when* in a frame's work the placeholder appears, or *where* it is put.

Supporting, and also short of proof: the API documents the method that turns a frame number into its output path — "Return the absolute path to the filename to be written for a given frame" (`bpy.types.RenderSettings.frame_path`, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30). Two renders sharing the same output path setting and frame numbering therefore aim at the same file names. That the render pipeline itself uses this same rule is my inference; the API describes it as a Python method, not as the renderer's internal behaviour.

### Part three — "started at the same instant"

**LABEL: not in the sources.**

Nothing in the manual, the API documentation, the release notes, or the supplied Cycles files says what happens when two renders reach the same frame at the same moment. There is no statement about file locking, about whether creating a placeholder is a single indivisible act, about what a network file system does to that, or about which render wins if both create or write the same file at once. The manual's hint says only "Start as many machines as you wish rendering to that directory" and stops there.

Equally, the sources never say what the folder holds if the two renders are left on the **default** settings (Overwrite on, Placeholders off). On those settings both renders would work through the same frames and aim at the same file names, but the documentation does not describe the outcome, so I will not guess it.

**The question was ambiguous.** "Two renders write into one folder" could mean (a) two Blender processes with the same output path and the same frame range — the reading I answered, and the one the manual's render-farm hint matches; (b) two renders using the same folder but different file names, in which case they never touch the same file and there is no interaction to describe — the sources say nothing about this either, but nothing about it is in question; or (c) two renders of different scenes, or of different frame ranges, writing into the same folder. I answered reading (a). **Reading (c) I did not answer**, and nothing in the searches recorded below settles it; it is carried into the Not covered line as an open hole rather than left as a passing remark.

### Where I looked

- Read in full: manual pages `render/output/properties/output` (including its Output panel picture), `render/output/animation`, `render/output/introduction`, `advanced/command_line/render`, the Render Options part of `advanced/command_line/arguments`, and the render-farm hint on `render/output/properties/metadata`.
- Searched the whole 4.5 manual for: `placeholder`, `overwrite`, `render farm`, `network render`, `shared directory`, `network file system`, `multiple machines`, `simultaneous`, `concurrent`, `two instances`, `multiple instances`, `another instance`, `file lock`, `locking`, `race condition`, `same time`, `clash`, `collide`, `conflict`, `duplicate frame`, `same frame`, `two computers`, `other machines`, `each machine`, `existing file`, `already rendered`, `skip…frame`, `touch`. Every hit outside the pages named above was unrelated (sculpting, geometry nodes, keymaps, add-on namespaces, video proxies).
- Searched the whole 4.5 Python API documentation for `use_overwrite`, `use_placeholder`, `render farm`, `placeholder`, `empty placeholder`, `Overwrite existing files`. The render-relevant hits are the two `bpy.types.RenderSettings` entries quoted above; everything else belongs to glTF export, library linking, sequencer image strips, or UI layout.
- Searched the 4.5 release notes for `overwrite`, `placeholder`, `output path`, `render output`, `write`, `partial`. The only relevant entry is the bug fix "Placeholder output files wrong name" (https://developer.blender.org/docs/release_notes/4.5/bugfixes/, read 2026-07-30) — a change to placeholder naming, with no description of concurrent behaviour.
- Searched the Cycles 4.5 files for `placeholder`, `overwrite`, `atomic`, `temp file`, `.tmp`. As noted under question 13, the supplied Cycles tree holds only the GPU kernel, utilities, and the add-on's Python files, and contains no output-writing code. Overwrite and Placeholders are Blender scene render settings, not Cycles settings, and do not appear in `properties.py` or `ui.py`.

### What running it settled — the hand tests of 2026-08-11

**LABEL: observed.** Twenty-one tests, built from all eight of this round's findings documents and the four audits and saved at `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, were run in Blender 4.5.12, 5.0.1, 5.1.2 and 5.2.0 on two machines. **Machine one, the Creator's Mac:** a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card. **Machine two, a rented Linux machine:** Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Every run had a settings folder of its own and was started with `--factory-startup`. The two machines' results are given separately below and are never merged. Everything quoted is Blender 4.5.12's own output.

**Overwrite off skips a frame before rendering it, not after.** A five-frame render at 512 by 512 with 400 samples, Overwrite off, with a complete PNG placed by hand at frame 3's name beforehand. Each line carries the seconds elapsed since the run started:

> ```
>    5.062  Saved: '…/frame_0001.png'
>    5.062  Time: 00:04.61 (Saving: 00:00.05)
>    9.619  Saved: '…/frame_0002.png'
>    9.619  Time: 00:04.55 (Saving: 00:00.00)
>    9.620  skipping existing frame "…/frame_0003.png"
>   14.130  Saved: '…/frame_0004.png'
> ```

Frame 3 took **one thousandth of a second**; its neighbours took four and a half seconds each. The file placed by hand kept its own size and its own last-changed time. — Creator's Mac, 2026-08-11. On the rented Linux machine the same run gave `7.100 Saved: frame_0002.png`, `7.102 skipping existing frame "…frame_0003.png"`, `10.912 Saved: frame_0004.png`. **The two machines agreed.** Note the wording: Blender 4.5 prints `skipping existing frame` in lower case, where 5.0, 5.1 and 5.2 print `Skipping existing frame`.

**"Already there" means a name exists, and nothing more.** Five things were put at frames 1 to 5's names before a six-frame render with Overwrite off: a complete PNG, a file of zero bytes, nineteen bytes of text that is not an image, a complete PNG of a different size, and a **folder**. All five were skipped and none was touched:

> ```
> skipping existing frame "…/frame_0001.png"
> skipping existing frame "…/frame_0002.png"
> skipping existing frame "…/frame_0003.png"
> skipping existing frame "…/frame_0004.png"
> skipping existing frame "…/frame_0005.png"
> Saved: '…/frame_0006.png'
> ```

Every one of the five kept its original size and last-changed time. — Creator's Mac and the rented Linux machine, 2026-08-11; **the two machines agreed**, byte for byte and line for line.

**Two renders started together on the shipped defaults both render every frame.** One blend-file, frames 1 to 8, 400 by 400, 200 samples, Overwrite on and Placeholders off, both started by one command. Each of the two printed `Saved:` for all eight frames. The folder ended with eight files, all of them opening as 400 by 400 images. — Creator's Mac and the rented Linux machine, 2026-08-11; **the two machines agreed**.

**Two renders started together on the manual's recipe divide the frames between them.** The same file with Overwrite off and Placeholders on. One run reported:

> ```
> skipping existing frame "…/frame_0001.png"
> Saved: '…/frame_0002.png'
> skipping existing frame "…/frame_0003.png"
> Saved: '…/frame_0004.png'
> skipping existing frame "…/frame_0005.png"
> Saved: '…/frame_0006.png'
> skipping existing frame "…/frame_0007.png"
> Saved: '…/frame_0008.png'
> ```

and the other reported the exact complement — saved 1, 3, 5, 7 and skipped 2, 4, 6, 8. Eight files, each written once, all readable. — Creator's Mac, 2026-08-11. The rented Linux machine gave the same split. **The two machines agreed.**

**The placeholder is not a lock, and how often it is raced depends on the file system.** The recipe was run over a single frame, both Blenders started by one command, into a fresh empty folder, **thirty times per version per machine**. Counting how often both runs reported saving frame 1:

- Creator's Mac, local disk: **0 of 30** in 4.5, 0 of 30 in 5.0, 0 of 30 in 5.1, **2 of 30 in 5.2**.
- Rented Linux machine, local disk: **0 of 30** in 4.5, and 0 of 30 in 5.0, 5.1 and 5.2.
- Rented Linux machine, a folder mounted over NFS from itself: **5 of 30** in 4.5, 6 of 30 in 5.0, 4 of 30 in 5.1, 4 of 30 in 5.2.

In all 240 runs the resulting file was present and larger than zero bytes. Two collisions on one machine and none on the other is not a version difference and is not read as one; what it shows is that the claim on a frame **can** be lost, and that it is lost far more often over a network file system than on a local disk — which is the arrangement the manual's own hint recommends. — 2026-08-11.

**Two renders over overlapping but different frame ranges.** One run over frames 1 to 10 and one over 6 to 15, started together. On the defaults both wrote the whole of their own range, so frames 6 to 10 were written twice and the folder ended with fifteen files. On the recipe, one run saved 1 to 5 and skipped 6 to 10 — `skipping existing frame "…/frame_0006.png"` and so on — while the other saved 6 to 15; fifteen files, each written once. — Creator's Mac and the rented Linux machine, 2026-08-11; **the two machines agreed**. This is reading (c) of the question, which this answer left open above.

**A placeholder left behind by a killed render blocks its frame for good.** A five-frame render on the recipe was killed outright the instant frame 2's file appeared. The folder then held `frame_0001.png` at 642,065 bytes and `frame_0002.png` at **0 bytes**. The same render was started again and run to the end:

> ```
> skipping existing frame "…/frame_0001.png"
> skipping existing frame "…/frame_0002.png"
> Saved: '…/frame_0003.png'
> Saved: '…/frame_0004.png'
> Saved: '…/frame_0005.png'
> ```

Frame 2 stayed at 0 bytes and Blender would not open it. With two renders running and one killed part-way, the survivor finished the rest and left the killed one's claimed frame at 0 bytes in the same way. — Creator's Mac and the rented Linux machine, 2026-08-11; **the two machines agreed**.

**With a video output the two settings do nothing.** Both renders wrote to one file whose name Blender built as `vid0001-0008.mp4`, both printed `Append frame 1` through `Append frame 8`, and the folder ended with a single file that Blender's own movie reader read back as 8 frames — the same on the defaults and on the recipe. — Creator's Mac and the rented Linux machine, 2026-08-11; **the two machines agreed**.

**Nothing in the output path identifies the run.** Setting the output path to `{name}` for twenty-one names and asking `frame_path` what it returned: in Blender 4.5 only `blend_name`, `fps`, `resolution_x` and `resolution_y` were substituted, and every other name — including `pid`, `process_id`, `hostname`, `machine`, `user`, `run`, `instance`, `uuid`, `random`, `time` and `date` — came back as `Error: Parse errors in path`. Two runs of the same file returned identical paths for every name. — Creator's Mac and the rented Linux machine, 2026-08-11; **the two machines agreed**. (In 5.0, 5.1 and 5.2 five further names are substituted — `blend_dir`, `blend_name_lib`, `blend_dir_lib`, `scene_name` and `camera_name` — and none of them identifies the run either.)

**Not covered, after the tests.** This answer still does not settle: what two renders on **two separate computers** sharing one folder do, since the project has no file system shared between its two machines; what happens on Windows; or whether a network file system other than NFS behaves the same way. What the tests did settle is written above. The earlier list — how finely the work divides, what the defaults do, what a collision leaves, whether an abandoned placeholder is ever cleaned up, and whether a network file system changes the outcome — is now answered by observation rather than left open.

**Not covered.** This answer does not settle: how finely the work divides (per frame is what the placeholder scheme implies, but the manual never says so); what happens on the default settings; what happens when two renders collide on the same frame within the same instant; whether a placeholder left behind by a render that dies is ever cleaned up, or whether it permanently blocks that frame from being rendered; whether the finished folder can be told apart from one where some frames are still empty placeholders; and how a network file system's own behaviour affects any of it. It also leaves reading (c) of the question entirely open — what each render produces, and what the folder holds, when the two renders cover different frame ranges or different scenes. *(That paragraph describes what the **sources** do not settle, and it stands. The hand tests above answer most of it from a running Blender.)*

**Outside the interface.** **Brought up to date by the hand tests of 2026-08-11:** the two settings do take when a render is driven from outside Blender's window. A blend-file saved with Overwrite on and Placeholders off was rendered twice: once as saved, and once with `use_overwrite` set to False and `use_placeholder` to True through Blender's Python before the render. The second run printed `SET use_overwrite: False | use_placeholder: True`, and when each was run again over its own folder the first wrote over all four frames while the second reported `Info: No frames rendered, skipped to not overwrite` and `skipping existing frame` for every one. Both machines, all four versions, agreeing. So this answer holds outside the interface, and that is where every result in it was measured. What was written below about the sources still stands as a description of the sources.

The sources do not state any difference, and one line points towards there being none, though reaching it takes a step. The command line has no argument for Overwrite or Placeholders — the full Render Options list on https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html (read 2026-07-30) covers background mode, animation, scene, frame, frame start/end, frame jump, output path, engine, threads and two Cycles options (`--cycles-device` and `--cycles-print-stats`), and nothing else. *(Corrected by the citation check of 2026-08-11: the list had ended at threads and said "nothing else", omitting the two Cycles options that the same section carries.)* But both settings live in the blend-file as `bpy.types.RenderSettings` properties, and the manual describes the animation argument as:

> "-a
>
> Render the whole animation using all the settings saved in the blend-file."

— Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — read 2026-07-30

**The step:** Overwrite and Placeholders are settings saved in the blend-file, and `-a` is documented as using all the settings saved in the blend-file, so a render started from the command line carries them. That is inferred, not stated. Separately, both appear in the Python API as properties of `bpy.types.RenderSettings`, quoted above with their type and their default; nothing I have quoted says whether a script may set them rather than only read them, so I do not lean on that. What the sources never say is whether the *outcome* of two renders sharing a folder differs when they are driven from outside the window. Note also that the manual's own render-farm hint is written in terms of the panel ("in the Render Output panel"), and it does not say how the machines are started.

**Operating system.** The sources do not say. No page distinguishes operating systems here. The hint says "a shared directory over a network file system" without naming a system or a platform, and the API's "similar to Unix ‘touch’" is describing what a placeholder is, not limiting it to one platform. **Brought up to date by the hand tests of 2026-08-11:** every test above was run on macOS 26.5.2 on an Apple M1 Pro **and** on Ubuntu 22.04.5 LTS on an AMD Threadripper. The two machines agreed on everything except how often two renders collide on one frame: on the local disk the Creator's Mac saw two collisions in thirty repeats in Blender 5.2 and none in the other three versions, while the rented Linux machine saw none in any version. That is a race and is not read as a difference between operating systems. **Windows is untested**, no Windows machine having run it.

---

## Things a person with Blender open could settle by hand in a few minutes

**Pruned on 2026-08-11, and pruned again on 2026-08-12** after the second batch of hand tests. The eight items that stood here were all carried out, in all four researched versions, on the Creator's Mac and on a rented Linux machine; what came back is written into questions 13 and 14 above as observed findings, and the list they were run from is `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`. What is left is what a running Blender on either of those two machines could not reach.

1. **Two renders on two separate computers writing into one shared folder.** This is the arrangement the manual's own render-farm hint describes, and it is the one thing about question 14 that neither machine can answer alone. The project's two machines are the Creator's Mac and a rented Linux machine on the public internet, with no file system shared between them, and sharing one would mean installing software on the Creator's working machine. A network file system **was** tested, with both renders on the rented Linux machine writing to a folder mounted over NFS from itself, and it changed the answer — see question 14 — so the two-computer case is worth taking up whenever a shared folder exists.

2. **Anything on Windows.** No Windows machine has run it. Every answer above covers macOS and Linux and leaves Windows open.

3. **A real power cut, as against a kernel reset.** The rented Linux machine was reset at the kernel with nothing flushed, which is as close as a machine reached over a network can come, and the result is under question 13. The reset was done twice, on 2026-08-11 and again on 2026-08-12, and the two runs do not agree about which directory entries survive. Pulling a plug is still not reachable on either machine.

4. **Whether Blender receives an Escape key sent by a tool, and so what a person pressing Esc with their own hand would get.** Whether the render *stops* has now been run three times on the rented Linux machine and has come out both ways — it did not stop on 2026-08-11, and it did stop in the citation check's repeat and again in entry 22 on 2026-08-12, that last one with a control. All three runs are recorded under question 13 and none has been chosen. What no run of this shape can show is whether the key reached Blender at all, so the hand half of the question stays out of reach. Nothing in this round is held back for a person, so this is recorded as unreached rather than passed on.

5. **What a person clicking the status bar's cancel button leaves behind.** That control needs a hand on a mouse, and nothing in this round is held back for a person.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Frames, Blender 4.5

2026-08-12 — Claude Opus 5 (Claude Code) — the second batch of hand tests written in: entries 22, 23 and 25 run and recorded under question 13, both standing lines brought up to date, and the researcher's own list pruned again.
