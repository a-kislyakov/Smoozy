# Frames — cross-sweep questions, second pass (Blender 5.1)

Questions 13 and 14 from the Frames question sheet. Answered cold.

**Sources consulted, and on what date.** Blender's 5.1 manual as plain text (`Blender Sources/manual_5.1`, including its `_images` folder); Blender's 5.1 Python API documentation as plain text (`Blender Sources/api_5.1`); Blender's 5.1 release notes (`Blender Sources/release_notes_5.1`); Cycles' own files for 5.1 (`Blender Sources/cycles_5.1`). All four were taken on 2026-07-30, and that is the date read for everything quoted below. I did the searching and writing on 2026-08-11.

`I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.`

(One clarification, offered so the line above cannot be read as more than it is: I also read `Blender Sources/About this folder.md`, which sits inside `Blender Sources/`. It is a note about how that folder was made. It contains a link to a planning document elsewhere in the project; I did not follow it, and I opened nothing it points to.)

---

## A note that applies to both answers

Neither question is about a feature. Both are about what Blender *does* to files on disk in a moment of trouble. Blender's documentation describes features. I searched hard for behaviour and found very little, and I have said so plainly rather than reasoning my way to something that sounds right. The record of where I looked is written out under each answer, and a combined list of search terms is at the end.

---

## 13. When a render is stopped at the exact moment it is writing a frame's own image file, what is left where it writes?

**LABEL: not in the sources.**

Nothing in the 5.1 manual, the 5.1 Python API documentation, the 5.1 release notes, or Cycles' own 5.1 files says what the output file looks like if Blender is stopped part-way through writing it. The sources never mention a partly written image file, a truncated image file, a temporary or scratch name that is renamed once the write finishes, or any guarantee that a frame's file appears all at once. They also never say the opposite. The question is simply not addressed.

What I *can* report is the surrounding ground the sources do cover, because it bounds the answer.

**Documented — the write happens after the frame is rendered, not during it.**

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

Source: manual. https://docs.blender.org/manual/en/5.1/render/output/animation.html — date read 2026-07-30.

The Python API describes a matching event:

> "on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

(the description of `bpy.app.handlers.render_write`). Source: Python API documentation. https://docs.blender.org/api/5.1/bpy.app.handlers.html — date read 2026-07-30.

Together these say there is a distinct moment at which a frame is written, and it comes after that frame is rendered. They do not say how long that moment lasts, nor what the file looks like inside it. Reading "directly after the frame is written" as a promise that the file is complete and correct at that instant would be me supplying the promise, not the source.

**Documented — cancelling a render is a thing the sources acknowledge, but only at frame granularity.**

> "For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

Source: manual. https://docs.blender.org/manual/en/5.1/render/output/introduction.html — date read 2026-07-30.

The unit of talk is "the last rendered frame". The sources never descend below a whole frame when discussing cancellation. The Python API's cancellation hook for render engines is likewise about the render, not the write:

> "Test if the render operation should been canceled, this is a fast call that should be used regularly for responsiveness"

(the description of `RenderEngine.test_break()`). Source: Python API documentation. https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — date read 2026-07-30. (The wording "should been" is the documentation's own.)

**Documented — the one place the sources do warn about a spoiled file is video, not a frame's own image file.**

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Source: manual. https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — date read 2026-07-30.

This is the closest the whole 5.1 documentation comes to describing damage from an interrupted render, and it is explicitly about a video container. The same page recommends still images precisely because "the frames you have already rendered will still be in the output directory" (https://docs.blender.org/manual/en/5.1/render/output/animation.html), which speaks to finished frames, not the one being written.

**Documented — other things that may sit in the output folder during a render, which change what "what is left" means.**

Two settings put extra files in the same folder, and both are worth knowing before anyone inspects a folder after an interruption.

Placeholders:

> "Create empty placeholder frames while rendering."

Source: manual, Output panel. https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — date read 2026-07-30.

and the same setting in the Python API (`RenderSettings.use_placeholder`):

> "Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

Source: Python API documentation. https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — date read 2026-07-30.

EXR preview copies (`ImageFormatSettings.use_preview`):

> "When rendering animations, save JPG preview images in same directory (default False)"

Source: Python API documentation. https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html — date read 2026-07-30.

The manual's wording of the same option, under OpenEXR output options:

> "When rendering animations (or single frames via command line), save a JPEG copy of the image, for a quick preview."

Source: manual. https://docs.blender.org/manual/en/5.1/files/media/image_formats.html — date read 2026-07-30.

**Inferred — one small step, written out.** From "Create empty placeholder files while rendering frames (similar to Unix 'touch')" it follows that, with Placeholders switched on, there is a stretch of time during which a file exists at the frame's path and is empty. The step I am taking: "create an empty file while rendering the frame" means the empty file exists before the rendered image is put there, so an interruption in that stretch leaves an empty file. The sources do not say this in so many words, and they do not say whether the placeholder is later replaced, filled in place, or removed and rewritten. Label: **inferred**, and no further than that.

**Observed — Cycles' own files cannot answer this.** The Cycles files supplied for 5.1 (`Blender Sources/cycles_5.1`) contain the add-on's Python (`__init__.py`, `camera.py`, `engine.py`, `operators.py`, `osl.py`, `presets.py`, `properties.py`, `ui.py`, `version_update.py`), the shader node definitions under `shader/`, and the render kernel headers under `source/kernel/` and `source/util/`. I searched all of it for file-writing (`fopen`, `ofstream`, `ImageOutput`, `write_image`, `rename(`, `.tmp`, `temp`). The only matches are unrelated: `osl.py` line 14–16 compiles a shader to a temporary file —

> `"""compile .osl file with given filepath to temporary .oso file"""`

(file `Blender Sources/cycles_5.1/osl.py`, line 14) — `operators.py` handles the standalone denoise and merge operators, and `source/kernel/device/hiprt/bvh.h` line 82 carries a comment reading "`ofstream` to map prim id in hiprtHit to the one compatible to what next stage expects". *(The `bvh.h` match was added by the citation check of 2026-08-11, which found it while checking "the only matches" claim. It is as unrelated as the others, so the conclusion below is unchanged.)* No part of the supplied Cycles code writes rendered frames to disk. Whatever writes a frame's image file is not in the files I was given, so I cannot examine it. Date read 2026-07-30, on macOS (Darwin 25.5.0, Apple silicon).

**Ambiguity flagged.** "Stopped" could mean stopped from inside Blender (the render job cancelled) or the whole program stopped from outside (killed, or the machine going down). I have answered both, because the sources are silent on both, and the record of looking below covers both. If only the first was meant, the answer is unchanged.

### What running it settled — the hand tests of 2026-08-11

**LABEL: observed.** Twenty-one tests, built from all eight of this round's findings documents and the four audits and saved at `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, were run in Blender 4.5.12, 5.0.1, 5.1.2 and 5.2.0 on two machines. **Machine one, the Creator's Mac:** a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card. **Machine two, a rented Linux machine:** Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Every run had a settings folder of its own and `--factory-startup`. Everything quoted is Blender 5.1.2's own output; the two machines are never merged. The same results are in this version's first-pass document, in fuller form.

**Blender writes straight to the final name from the start.** Three frames at 4096 by 4096 as OpenEXR with no compression, the whole folder listed every two milliseconds. The only names that ever appeared were the three final frame names — nothing hidden, nothing ending `.tmp` or `.part`, and nothing in the system temporary folder:

> ```
> WATCH    0.000  <empty>
> WATCH    1.395  frame_0001.exr=86934
> WATCH    1.398  frame_0001.exr=2974614
> ```

and 113 distinct sizes later the file reached 268,501,910 bytes. — Creator's Mac; the rented Linux machine did the same, ending at 268,501,871 bytes. **The two machines agreed.**

**How the stop is delivered decides what is left.** Each run was stopped the instant frame 2's file first appeared. An **interrupt** did not cut the write: Blender finished the frame — `Saved: '…/frame_0002.exr'`, `Time: 00:01.01 (Saving: 00:00.33)`, then `Sent an internal break event. Press ^C again to kill Blender`, exit code 1 — and the file was whole and opened. A **terminate** left 0 bytes on the Creator's Mac and 32,772 on the rented Linux machine, exit code -15, no `Saved:` line, unopenable. A **kill** left 923 and 0 bytes, exit code -9, unopenable. So a file **does** exist at the frame's own path after a stop that is not an interrupt.

**The format matters, and a part-written file is not always distinguishable from a whole one.** The same kill per format at 4096 by 4096: PNG left 0 bytes on both machines; JPEG 0 on both; OpenEXR 5,023 on the Mac and 32,768 on Linux; TIFF 8 bytes on the Mac, which did not open, and **1,306 on Linux, which did — reported as 4096 × 4096 with `has_data: True`** — against a complete TIFF of 5,399,372 bytes. Multi-layer OpenEXR needed `media_type` set to `MULTI_LAYER_IMAGE` first, and then wrote one file per frame, 537,036,580 bytes complete and 59,172 after a kill.

**Nothing was still being held back by the operating system.** After a kill the folder was listed at once and again sixty seconds later; the two listings were identical, on both machines and in all four versions. That answers the buffering item this document names under *Not covered*.

**The placeholder is created before the frame is rendered, one frame at a time.** With Overwrite off and Placeholders on: `WATCH 0.355 frame_0001.exr=0`, with the first real bytes only at 1.644 seconds, and frame 2's file not appearing until frame 1 was finished. That is exactly the inference this document marks as inferred, now observed. Stopped once frame 2's file had grown past zero, the leftover was 33,687 bytes on the Creator's Mac and a short unopenable file on the rented Linux machine. **The two machines agreed.**

**The JPEG preview copy exists and is written after each frame.** With `use_preview` on and OpenEXR output the folder held a `.jpg` beside every `.exr` — `Saved: '…/frame_0001.exr'` at 00:00.567 and `Saved: '…/frame_0001.jpg'` at 00:00.647. That is the extra file this document raises, and it appears for an animation render driven from outside the window.

**A Compositor File Output node writes before the render's own frame file.** Stopped the instant the node's own second file appeared, the folder held it at 33,498 bytes on the Creator's Mac and 16,387 on the rented Linux machine, unopenable, and **no Output-panel file at all for that frame**. In 5.1 the compositor is a node group on `scene.compositing_node_group` and the node takes a `directory` and a `file_name`, where 4.5 uses `scene.node_tree` and a `base_path`. *(The citation check of 2026-08-11 could not repeat this. It confirmed from Blender itself that the scene carries a `compositing_node_group` and that the node's properties are `directory` and `file_name` rather than `base_path`, but what is written here does not say how the node is wired into the new compositor, and the node wrote no file at all when it was set up from this description. The 4.5 result repeated exactly. Written out as item 24 on the hand-tests list.)*

**Cancelling from inside Blender's own window did not stop the render.** On the rented Linux machine only: a forty-frame render in a real window with the render window holding the keyboard, Escape sent to every Blender window repeatedly for ten seconds with `xdotool`. Fifteen seconds later the render was still going and went on writing whole frames, in all four versions. This shows the render did not stop; it cannot show Blender received the key.

**The citation check of 2026-08-11 repeated this and got the opposite, and both runs are recorded rather than one replacing the other.** Following the procedure written above on the rented Linux machine, the render **stopped**: sixteen frames stood in the folder when the first Escape was sent and sixteen stood there fifteen seconds after the last one, in Blender 4.5, run twice; in 5.2 it was fourteen and fourteen. A control identical in every respect except that no Escape was ever sent went from sixteen frames at twenty-five seconds to thirty-six at fifty seconds in 4.5, and from fourteen to thirty-three in 5.2 — so without the Escape the render kept going, and with it the render stopped. That is a question, not an answer. It is item 22 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, to be run again, and until it is run nothing downstream should treat either result as the fact.

**A machine going down loses even frames it had already reported saving.** On the rented Linux machine only, the machine was reset at the kernel — `echo b > /proc/sysrq-trigger`, nothing flushed — the instant frame 2's file appeared. Afterwards **the output folder was empty** and the run's log file was 0 bytes; frame 1 had been reported saved seconds earlier and was gone. A plain `reboot` had been tried first and the machine came back from it. This is the *unflushed write* case, and it comes out differently from every kill above. *(The citation check of 2026-08-11 did not repeat this: the kernel reset was refused by the permission system of the machine that check ran on. The result stands on its single run, and is item 25 on the hand-tests list.)*

### What the second batch of hand tests settled — 2026-08-12

**LABEL: observed.** Four more tests — entries 22 to 25 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md` — were added by the citation check of 2026-08-11: three because a repeat came back differently from a run recorded above, and one because a procedure could not be followed as written. They were run on 2026-08-12 on the machines each entry names, on the same two machines described above, each with a settings folder of its own and `--factory-startup`. Everything quoted is Blender 5.1.2's own output, and the two machines are never merged. The same results are in this version's first-pass document, in fuller form. **Where a result differs from one written above, both stand and neither has been chosen.**

**Escape sent to Blender's own window stopped the render — entry 22.** On the rented Linux machine only, the one machine with a screen this round could drive. A forty-frame render at 480 by 480 with eighty samples, PNG, started in a real window from a timer. At twenty-five seconds the folder held 37 files, `xdotool getwindowfocus getwindowname` answered `Blender Render`, and `xdotool search --name Blender` answered `62914562 62914691`. Escape was sent to both windows and to the focused window twenty-seven times over ten seconds; the folder held 38 files ten seconds later and 38 fifteen seconds after that — one more frame finished, then nothing. A control in the same sitting with no Escape sent went 38, then 40, then 40. The other three versions came out the same way — 4.5 froze at 30, run twice, 5.0 at 35, 5.2 at 33, while every control reached all 40 frames of the range. **This agrees with the citation check's repeat and disagrees with the run recorded above. Both stand.** It still cannot show that Blender received the key.

**A stop leaves a different number of bytes almost every time, for JPEG and for TIFF — entry 23.** Ten repeats per format per version per machine: the three-frame render at 4096 by 4096 was killed the instant frame 2's file appeared, and its size and whether it opened were recorded. In Blender 5.1 on the **Creator's Mac** the ten JPEG leftovers were **0 bytes every time** and none opened; the ten TIFF leftovers were 8, 1,306, 1,306, 8, 1,306, 1,306, 1,286, 1,306, 8 and 0 bytes, five opening. On the **rented Linux machine** the ten JPEG leftovers were 0, 0, 0, 0, 0, 4,096, 0, 0, 4,096 and 4,096 bytes, three opening; the ten TIFF leftovers were 238, 1,306, 1,306, 8, 1,306, 8, 1,306, 1,306, 8 and 8 bytes, five opening. Complete frames were 4,128,053 and 5,397,377 bytes on the Mac and 4,113,616 and 5,399,372 on the rented machine. **Every cell is a spread**, except this version's JPEG on the Mac, which came out at 0 bytes ten times of ten. So the single figures written above, and the different single figures the citation check got, are draws from a race rather than facts about a version or a machine.

**The Compositor File Output node set-up written out, and entry 7 run again from it — entry 24.** On the Creator's Mac. The script the hand-test list carries was run again unchanged and the node **did** write its files, so it is the description in this document, not the list's script, that could not be rebuilt from. Taking the set-up apart one piece at a time showed which pieces the node needs: without the item added by `file_output_items.new`, without the link into that input, without a Render Layers node feeding it, without `use_compositing`, or without the group being put on `scene.compositing_node_group`, the node wrote nothing; without the group's output socket and Group Output node it still wrote. Blender 5.0 and 5.2 gave the same seven answers. So the set-up is: a node group of kind `CompositorNodeTree` on `scene.compositing_node_group`; a Render Layers node inside it; a File Output node with an item added by `file_output_items.new("RGBA", "Image")`, which is what creates the input; a link from the Render Layers node's Image output into that input; `use_compositing` switched on; and the node's `directory` and `file_name` set. With that, entry 7 ran as written: a complete run printed the node's save first every time — `00:01.474  render           | Saved: '…/node/nodeImage0001.exr'` before `00:02.091  render           | Saved: '…/out/frame_0001.png'` — and a run killed the instant the node's own second file appeared left `nodeImage0002.exr` at **0 bytes**, which did not open, and **no Output-panel file at all for frame 2**. In 5.0 the same stop left 721 bytes and in 5.2 13,013 bytes, neither opening. That repeats the shape of the result recorded above.

**The kernel reset, run a second time, left the folder present and empty in every version — entry 25.** On the rented Linux machine only, run last. A plain `reboot` was tried first and the machine came back. Once per version, the three-frame OpenEXR render was started with no window and the machine reset at the kernel the instant frame 2's file appeared. In Blender 5.1 the output folder came back present and `<empty>`, the run's own log file survived at 0 bytes, and the blend-file survived whole at 95,504 bytes. Frames 1 and 2 were both absent, and frame 1 had been reported saved before the reset. The folder was present and empty in 4.5, 5.0 and 5.2 as well. **That agrees with the run recorded above for 5.1 and disagrees with it for 5.0 and 5.2**, where that run had no folder at all and a zero-byte `frame_0001.exr` respectively. Both stand. What came out the same way twice: no frame's contents survived in any version, including a frame already reported saved.

**Not covered.** This answer does not settle: whether Blender writes a frame to its final name from the start or to a temporary name first; whether an interrupted write leaves a short file, a full-length file with garbage in it, or no file at all; whether that differs between formats (PNG, JPEG, OpenEXR, and the rest are written by different code and the sources say nothing about any of them); whether the operating system's own buffering can leave a file that looks complete but is not; and whether a placeholder file is overwritten in place or replaced. It also does not settle how long the write window is — which is what decides how likely it is that a stop lands inside it.

**Outside the interface.** The sources say nothing about interrupted writes either inside or outside Blender's window, so there is no documented difference to report. Two nearby facts are documented and do differ. First, the EXR preview copy: the manual says it is saved "When rendering animations (or single frames via command line)" (https://docs.blender.org/manual/en/5.1/files/media/image_formats.html), so the single-frame case is called out as a command-line one — meaning a command-line render of one EXR frame can leave two files in the folder where a click-driven single-frame render would leave one. That is a documented inside/outside difference in what sits in the output folder. Second, when driving Blender from a command line the order of arguments decides where output goes at all — the quote for that is below. **Brought up to date by the hand tests of 2026-08-11:** every result under *What running it settled* was got from a render driven from outside Blender's window, with no window at all, so the answer holds outside the interface — that is where it was measured. Sending Escape to Blender's own window on the rented Linux machine did not stop the render, so what a person pressing Esc or clicking the status bar's cancel button would get is the only part not established from outside — but see the note under *Cancelling from inside Blender's own window* above, where a repeat came back the other way. **Brought up to date again by the second batch of hand tests, 2026-08-12:** the Escape test was run a third time, with a control, and the render **stopped** in all four versions — entry 22 above. The runs disagree and none has been chosen. The hand half stays open either way, because no run so far can show that Blender received the key. The EXR preview copy noted here does appear for an animation render driven from outside, one JPEG beside each EXR. *(The 2026-08-11 update block had been pasted into the middle of the sentence introducing the argument-order quote, leaving that quote stranded after the paragraph. The citation check of 2026-08-11 closed the sentence and pointed it at its quote; no words were added to or removed from either claim.)*

> "blender --background --render-output /tmp test.blend --render-frame 1 …will not render to /tmp because loading the blend-file overwrites the render output that was set."

Source: manual. https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — date read 2026-07-30. Beyond that, the sources do not say.

**Operating system.** The sources do not say. Nothing in the 5.1 documentation ties image-file writing behaviour to an operating system. Two documented OS facts sit nearby but do not answer the question: the list of render formats is not the same everywhere, and Blender's temporary directory is set by different environment variables per system — the quotes for both are below. **Brought up to date by the hand tests of 2026-08-11:** everything above was run on macOS 26.5.2 on an Apple M1 Pro **and** on Ubuntu 22.04.5 LTS on an AMD Threadripper, and the two agreed on every point of behaviour; where they differ is in numbers that are races. **Windows is untested**, no Windows machine having run it, so this answer covers macOS and Linux and leaves Windows open. *(The 2026-08-11 update block had been pasted into the middle of the sentence introducing those two quotes, leaving them stranded after the paragraph. The citation check of 2026-08-11 closed the sentence and pointed it at its quotes; no words were added to or removed from either claim.)* **Brought up to date again by the second batch of hand tests, 2026-08-12:** entry 23 ran the JPEG and TIFF stop ten times per version on each machine, and both machines gave a spread of sizes and a mixture of files that opened and files that did not — the race is now shown directly. One thing did divide the two machines in this version: on the Creator's Mac the JPEG leftover was 0 bytes in all ten repeats, where the rented Linux machine gave 4,096 bytes in three of ten. Entries 22 and 25 were run on the rented Linux machine only, so what an Escape does inside a window and what a machine going down leaves are answered for Linux and left open for macOS.

> "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP."

(manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, date read 2026-07-30) — so which format's write behaviour is even in play can differ by system; and Blender's temporary directory is set by different environment variables per system, `TEMP` "Store temporary files here (MS-Windows)" and `TMPDIR` "Store temporary files here (UNIX Systems)" (same page). That temporary directory is documented as being for "temporary save files" (https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html), not for render output, so I am not treating it as evidence that frames are written through a temporary file.

### Where I looked for question 13

Searched, in every case across the whole of `manual_5.1`, `api_5.1` and `release_notes_5.1` at once, not file by file:

- Words for a spoiled file: `partial`, `partially`, `partially written`, `partial file`, `incomplete`, `incomplete file`, `truncated`, `corrupt`, `damaged`, `broken`, `unusable`, `unplayable`, `zero-byte`, `0 bytes`, `empty file`, `zero length`. Every hit was unrelated — `truncated` is a maths node, `corrupt` is mesh import checks and GPU driver troubleshooting, `incomplete` is proxies and GPU rendering, `broken` is linked libraries. The single relevant hit, `unplayable`, is the video warning quoted above.
- Words for a careful write: `atomic` (zero hits anywhere), `written atomically` (zero), `safe write` (zero), `temporary file`, `temp file` (zero), `.tmp` (zero), `rename`, `in-place`, `fsync` (zero), `flush`, `journal` (zero). The `temporary file` hits are Blender's temp directory, the pose-library clipboard, and a USD hook — none about render output. `rename` hits are data-block renaming and file-browser renaming.
- Words for the stop itself: `canceled`, `cancelled`, `aborted`, `abort`, `interrupted`, `stopped`, `stop rendering`, `stop the render`, `mid-render`, `Ctrl-C` (only copy/paste shortcuts), `SIGINT` (zero), `signal` (only video signals and noise signals), `terminate` (only light-path termination), `crash`, `power fail`, `shut down`.
- Words for the write itself: `while rendering`, `during rendering`, `written to disk`, `saved to disk`, `write to disk`, `written directly`, `saved directly`, `disk full` (zero), `out of space` (zero).

Pages opened and read in full: the Output panel page (https://docs.blender.org/manual/en/5.1/render/output/properties/output.html); Rendering Animations (https://docs.blender.org/manual/en/5.1/render/output/animation.html); the output Introduction (https://docs.blender.org/manual/en/5.1/render/output/introduction.html); Supported Graphics Formats, which is where saving-images options live (https://docs.blender.org/manual/en/5.1/files/media/image_formats.html); Rendering From The Command Line (https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html); Command Line Arguments, render and format and environment sections (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html); the File Output node (https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html); Cycles Performance settings (https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html); the File Paths preferences (https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html). In the API: `bpy.app.handlers`, `bpy.types.RenderSettings`, `bpy.types.RenderEngine`, `bpy.types.Image`, `bpy.types.ImageFormatSettings`, `bpy.ops.render`.

Ruled out, with the reason:

- **Troubleshooting → Recovering Data** (https://docs.blender.org/manual/en/5.1/troubleshooting/recover.html). It opens "Computer crashes, power outages, or simply forgetting to save can result in the loss or corruption of your work", which looks exactly on point, but the whole page is about blend-files and autosave. Nothing about render output.
- **Cycles tile caching.** Cycles Performance says "Tiles are cached to disk while rendering to save memory". I checked whether this puts anything in the output folder. The page does not say where the cache goes, and the only documented cache location is the separate "Render Cache" preference (https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html). Ruled out as not being the output folder.
- **Cache Result / `use_render_cache`.** "Saves the rendered view layers and their passes to a multi-layer OpenEXR image… stored in the Render Cache folder as specified in the File Paths Preferences." Again a different folder. Ruled out.
- **Save Buffers.** I searched for `Save Buffers` and `use_save_buffers` on the chance an older mechanism for spilling render buffers to disk still existed. Zero hits in 5.1. Ruled out as absent.
- **`bpy.ops.render.render()`.** This is the operator that does the work, and it would be the natural place for a statement about output files. Its entire description in 5.1 is "Undocumented, consider contributing." (https://docs.blender.org/api/5.1/bpy.ops.render.html). Ruled out because it says nothing.
- **The 5.1 release notes**, all twenty-one files. I read `rendering.md`, `core.md`, `cycles.md` and `pipeline_io.md` in full and searched all of them for `overwrite`, `placeholder`, `partial`, `interrupt`, `cancel`, `output path`, `render output`, `file output`. The image-related 5.1 changes are AVIF support, an OpenEXR HTJ2K encoding option, and multi-threaded JPEG 2000 saving; none concerns interruption. Ruled out.
- **Cycles' own files**, searched as described above. Ruled out because they contain no frame-writing code.

---

## 14. When two renders write into one folder and are started at the same instant, what does each of them produce, and what does the folder hold at the end?

**LABEL: documented in part; not in the sources for the part the question turns on.**

The documented part first, because it is unusually direct: the manual explicitly tells you to point several renders at one folder, and it tells you which two settings make that work.

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

Source: manual, Output panel page. https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — date read 2026-07-30.

The two settings it names are described on the same page. Note that the manual's own headings for them are damaged — the heading is printed twice with the same text, and the second one should say Placeholders:

> "Image Sequence – Overwrite Image Multi-Layer EXR
>
> Overwrite existing files when rendering.
>
> Image Sequence – Overwrite Image Multi-Layer EXR
>
> Create empty placeholder frames while rendering."

Source: manual. https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — date read 2026-07-30. (The trailing words in each heading are the manual's way of marking where the setting lives and which media types show it: the Image Sequence sub-panel, for Media Type "Image" and "Multi-Layer EXR". Read that way, the first setting is called Overwrite and the second is the one whose name has been lost.)

**Documented from a picture — the second setting is called Placeholders.** The same page carries `[image: _images/render_output_properties_output_panel.png]`, captioned "Output panel." I opened that picture. It shows the Output panel with a file path field reading `/tmp/`, and below it two columns of checkboxes: left column "Overwrite" (ticked) and "File Extensions" (ticked); right column "Placeholders" (unticked) and "Cache Result" (unticked). Below those, File Format "PNG", Color BW/RGB/RGBA with RGBA selected, Color Depth 8/16 with 8 selected, and Compression at 15%. So the panel does offer a checkbox named Placeholders, sitting beside Overwrite, and in that picture Overwrite is on and Placeholders is off. Source: the picture on https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — date read 2026-07-30. Label for what I take from it: **documented**. What I do not take from it: the picture cannot tell you what either checkbox does.

The Python API gives both settings' defaults, which matters because the question says "started at the same instant" without saying anything was configured:

> "use_overwrite
>
> Overwrite existing files while rendering (default True)"

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

Source: Python API documentation. https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — date read 2026-07-30.

And the manual documents what turning Overwrite off buys you:

> "Just disable the Overwrite option to start rendering where you left off."

and

> "Using an image format for output, you can use the Frame Step option to render every N'th frame. Then disable Overwrite and re-render with Frame Step set to 1."

Source: manual. https://docs.blender.org/manual/en/5.1/render/output/animation.html — date read 2026-07-30.

Where each render writes is fixed by the output path, and the Python API names the call that resolves it:

> "Return the absolute path to the filename to be written for a given frame"

(the description of `RenderSettings.frame_path()`). Source: Python API documentation. https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — date read 2026-07-30. Two renders with the same output settings therefore aim at exactly the same filenames.

**Inferred — the coordinated case, with the step written out.** Take the two documented statements "Overwrite existing files while rendering (default True)" and "Just disable the Overwrite option to start rendering where you left off." Together they say that with Overwrite off, a frame whose file is already present is not rendered again. Now add "Create empty placeholder files while rendering frames". A render that is about to work on a frame puts an empty file at that frame's path first. The step a reader has to supply — and it is a real step, which is why this is inferred and not documented — is that the *other* render then sees that empty file as "already present" and moves on to the next frame. Granting that step, the two renders divide the frames between them without being told who takes what, and the folder ends up holding one file per frame in the range, some written by one render and some by the other. This is plainly what the "Primitive Render Farm" hint intends, but the manual never spells the mechanism out; it only lists the steps and says it works.

**Not in the sources — everything the question actually turns on.** The sheet asks what each render produces and what the folder holds at the end, for two renders "started at the same instant". On that, the 5.1 sources say nothing:

- **The default case is not covered at all.** With the shipped defaults (Overwrite on, Placeholders off), two renders both work through every frame and both write to the same filenames. The sources never describe what happens when two Blender processes write the same file, nor what the folder holds afterwards. There is no statement anywhere that this is safe, unsafe, or even that it is possible.
- **Simultaneity is not covered.** The Primitive Render Farm hint says "Start as many machines as you wish", and says nothing about timing. Two renders reaching the same frame in the same instant — both finding no placeholder, both creating one, both rendering it, both writing it — is exactly the case the question names, and the manual does not address it. There is no documented lock, claim, or first-writer-wins rule.
- **No documented way to keep two renders apart automatically.** I checked the path-template variables, which would be where a per-machine or per-process token would live. The full documented list is `blend_name`, `blend_dir`, `blend_name_lib`, `blend_dir_lib`, `fps`, `resolution_x`, `resolution_y`, `scene_name`, `camera_name`, `node_name` (https://docs.blender.org/manual/en/5.1/files/file_paths.html). None identifies the machine, the process, or the run. Two renders of the same blend-file with the same settings therefore produce identical paths, and only Overwrite and Placeholders stand between them.
- **What "empty" means for a placeholder is not covered.** "similar to Unix 'touch'" is as far as the sources go. Whether a later render distinguishes an empty placeholder from a finished frame is not stated. (Cycles' own denoise operator, in `Blender Sources/cycles_5.1/operators.py` line 84–86, tests only `if not os.path.isfile(filepath):` and reports `"Frame '%s' not found, animation must be complete"` — an **observed** example, in the files I was given, of Blender-side code treating mere existence as completeness. It is the denoise operator, not the render loop, so it does not answer the question; I note it only because it is the one place in the supplied files where existence is used as a test at all. Read on macOS, Darwin 25.5.0, date read 2026-07-30.)

**Documented background — two Blenders at once is a supported idea.** The Python API says, of running Blender as a module:

> "As with the application it's possible to start multiple instances, each with their own bpy and therefore Blender state."

Source: Python API documentation. https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html — date read 2026-07-30. This confirms that running two Blenders is expected; it says nothing about their output folders.

**Ambiguity flagged.** "Two renders" could mean two separate Blender processes, or one process rendering twice. I have answered the first, because "started at the same instant" only makes sense for two processes and because the manual's own multi-writer material (the Primitive Render Farm hint) is about separate machines. Under the other reading — one Blender, two renders one after the other — the second render's behaviour is the Overwrite setting and nothing more, and the "same instant" clause has no meaning.

### What running it settled — the hand tests of 2026-08-11

**LABEL: observed.** The machines and the list are described under question 13 above. Everything quoted is Blender 5.1.2's own output; the two machines are never merged. The inference this document writes out — that the other render sees the empty placeholder as "already present" and moves on — is confirmed, and the case the document says the manual does not address is now counted.

**Overwrite off skips a frame before rendering it.** A five-frame render, Overwrite off, with a complete PNG placed by hand at frame 3's name. The seconds elapsed run down the left:

> ```
>    7.450  00:07.214  render           | Saved: '…/frame_0002.png'
>    7.450  00:07.214  render           | Skipping existing frame "…/frame_0003.png"
>   10.965  00:10.730  render           | Saved: '…/frame_0004.png'
> ```

The skip is printed at the same thousandth of a second as the previous save, where each real frame cost three and a half seconds, and the file placed by hand was untouched. — Both machines, agreeing.

**"Already there" means a name exists, whatever is behind it.** A complete PNG, a zero-byte file, nineteen bytes of text, a PNG of the wrong size and a **folder** were all put at frames 1 to 5's names; all five were skipped and none touched, and only frame 6 was rendered. — Both machines, agreeing. So the empty placeholder question this document raises — whether a later render can tell a placeholder from a finished frame — is answered plainly: it cannot, and it does not try.

**On the shipped defaults both renders render every frame.** Two renders of frames 1 to 8 started by one command: each printed `Saved:` for all eight, and the folder ended with eight readable files. — Both machines, agreeing. That is the default case this document says there is no statement about anywhere.

**On the manual's recipe the two renders divide the frames between them.** One reported saving 2, 4, 5 and 8 and skipping the rest; the other reported the exact complement. Eight files, each written once, all readable. — Both machines, agreeing.

**The placeholder is not a lock, and the file system decides how often it is raced.** The recipe was run over a single frame, both Blenders started by one command, into a fresh empty folder, **thirty times per version per machine**. Both runs saved frame 1: 0 of 30 in 4.5, 5.0 and 5.1 on the Creator's Mac and 2 of 30 in 5.2; 0 of 30 in all four on the rented Linux machine's own disk; and **5 of 30 in 4.5, 6 of 30 in 5.0, 4 of 30 in 5.1 and 4 of 30 in 5.2 over a folder mounted from that machine over NFS**. In all 240 runs the file was present and larger than zero bytes. So the claim on a frame can be lost, and it is lost far more often over a network file system than on a local disk — the very arrangement the manual's hint recommends, and the thing this document names as untested.

**Two renders over overlapping but different ranges.** One over frames 1 to 10 and one over 6 to 15, started together. On the defaults each wrote its whole range, so the overlapping five were written twice; on the recipe one saved 1 to 5 and skipped 6 to 10 while the other saved 6 to 15, fifteen files each written once. — Both machines, agreeing.

**A placeholder left by a killed render blocks its frame for good.** Killed the instant frame 2's file appeared, the folder held frame 1 whole and frame 2 at 0 bytes. Run again to the end, the render printed `Skipping existing frame "…/frame_0002.png"` and left frame 2 at 0 bytes, unopenable. With two renders and one killed part-way, the survivor finished the rest and left the killed one's claimed frame at 0 bytes. — Both machines, agreeing.

**With a video output the two settings do nothing.** Both renders wrote to one file named `vid0001-0008.mp4`, both printed `Video append frame 1` through `Video append frame 8`, and one file was left that read back as 8 frames — the same on the defaults and on the recipe. — Both machines, agreeing. *(Corrected by the citation check of 2026-08-11: the words quoted here were `Append frame 1` through `Append frame 8`, which is what Blender 4.5 prints. Blender 5.1 prints `Video append frame 1` and so on, behind the usual logging prefix. Checked in all four versions.)*

**No path template identifies the run.** This document lists ten template variables and says none identifies the machine, the process or the run. Asked of Blender itself, nine of the ten were substituted in 5.1 — all but `node_name` — and twelve invented names, including `pid`, `process_id`, `hostname`, `machine`, `user`, `run`, `instance`, `uuid`, `random`, `time` and `date`, all came back as `Error: Parse errors in path`. Two runs of the same file returned identical paths for every name. Blender 4.5 substitutes only four of the ten. — Both machines, agreeing.

**The two settings take from outside the window.** Set through Blender's Python before the render — `SET use_overwrite: False | use_placeholder: True` — the second of two runs later reported `Info: No frames rendered, skipped to not overwrite` where the first wrote over everything. — Both machines, all four versions, agreeing. That confirms the inside/outside difference this document draws from the command-line page: the settings must come from the blend-file or from Python, and from Python they work.

**Not covered.** This answer does not settle: what the folder holds when two renders collide on one frame; whether a half-written frame from one render can be seen as finished by the other; whether the placeholder mechanism is safe over a network file system, which is the very thing the hint recommends; whether renders started at different instants behave differently from renders started together; how any of this behaves when the two renders have *different* settings (different formats, different frame steps) but the same folder; and whether the folder ends with one file per frame or with some frames written twice.

**Outside the interface.** The two settings that make this work, Overwrite and Placeholders, have no command-line arguments. I read the full Render Options and Format Options sections of https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html and searched that whole page for `overwrite` and `placeholder`: the only hit for either word is an unrelated sentence about argument order. The documented command-line render controls are `-b`, `-a`, `-S`, `-f`, `-s`, `-e`, `-j`, `-o`, `-E`, `-t`, `-F`, `-x`. So a render farm driven from outside must carry Overwrite and Placeholders in the blend-file itself, or set them through Python. That is a real inside/outside difference and it is documented by absence from an exhaustive list. Two further documented points bear on driving this from outside. The output folder, which is the whole subject of the question, is set with `-o`: "Set the render path and file name." And argument order decides whether that even takes effect:

> "blender --background --render-output /tmp test.blend --render-frame 1 …will not render to /tmp because loading the blend-file overwrites the render output that was set."

and

> "blender --background test.blend --render-output /tmp --render-frame 1 …works as expected."

Source: manual. https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — date read 2026-07-30. Beyond that, the sources do not say whether the folder behaviour differs inside and outside Blender's window. **Brought up to date by the hand tests of 2026-08-11:** the two settings do take when a render is driven from outside Blender's window — set through Blender's Python before the render, they printed `SET use_overwrite: False | use_placeholder: True` and the run then skipped every existing frame where an unchanged run wrote over them. Every result under *What running it settled* was measured with no window at all, so this answer holds outside the interface.

**Operating system.** The sources do not say. The Primitive Render Farm hint names "a shared directory over a network file system" without naming any operating system or any file system, and gives no caveat for any of them. The API's phrase "similar to Unix 'touch'" is a comparison meant to explain what a placeholder is; it is not a statement that the feature is Unix-only, and I am not reading it as one. The one documented OS-dependent thing nearby is which output formats exist at all: "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP." (manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, date read 2026-07-30). **Brought up to date by the hand tests of 2026-08-11:** every test above was run on macOS 26.5.2 on an Apple M1 Pro **and** on Ubuntu 22.04.5 LTS on an AMD Threadripper, and the two agreed on every point of behaviour. The one number that differed is how often two renders collide on one frame, which is a race, not an operating-system difference. **The file system does change it**: over a folder mounted by NFS the collision rate went from none in thirty repeats to four, five or six in thirty. **Windows is untested**, no Windows machine having run it.

### Where I looked for question 14

Searched across the whole of `manual_5.1`, `api_5.1` and `release_notes_5.1` at once:

- The feature words: `placeholder`, `Placeholders`, `placeholder frames`, `use_placeholder`, `overwrite`, `use_overwrite`, `touch`.
- The many-machines words: `render farm`, `multiple machines`, `shared directory`, `network file system`, `workload`, `distribute`, `multiple instances`, `several instances`, `concurrent`, `simultaneous`, `at the same time`, `queue`, `render job`, `batch render`.
- The collision words: `file lock`, `locking`, `race condition`, `conflict`, `clobber` (zero), `same file`, `same path`, `same output`, `already exists`, `existing file`, `skip`, `resume`, `resumed`.

Pages opened and read: the Output panel page in full, including its picture `_images/render_output_properties_output_panel.png` (https://docs.blender.org/manual/en/5.1/render/output/properties/output.html); Rendering Animations in full (https://docs.blender.org/manual/en/5.1/render/output/animation.html); the output Introduction (https://docs.blender.org/manual/en/5.1/render/output/introduction.html); File Paths, for the path-template variable list (https://docs.blender.org/manual/en/5.1/files/file_paths.html); Command Line Arguments (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html); Rendering From The Command Line (https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html); the File Output node (https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html); the Cycles Sampling page, Sample Subset section (https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html). In the API: `bpy.types.RenderSettings`, `bpy.types.ImageFormatSettings`, `bpy.app.handlers`, `info_advanced_blender_as_bpy`, `bpy.ops.render`.

Ruled out, with the reason:

- **Cycles Sample Subset** (https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html). This is the other place in 5.1 where the manual describes several machines rendering at once: "A typical use case is to distribute the rendering of a single frame over multiple machines." I read the whole section. It splits the *samples* of one frame across machines and then merges the results with `bpy.ops.cycles.merge_images`, and its own example gives the machines different filenames (`1.exr`, `2.exr`). It is not about two renders sharing an output folder, and it says nothing about what a folder holds. Ruled out.
- **The File Output node** (https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html). Checked because it is a second way a render writes into a folder, and because 5.1's release notes change it. It documents that "Any missing directories are created automatically during output", which is the only folder-level behaviour stated anywhere, and nothing about two writers. Ruled out.
- **Metadata page.** It mentions render farms — "It can be useful to use the Note field if you are setting up a render farm" (https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html) — but only as advice about labelling. Ruled out.
- **Path templates.** Checked specifically for a per-process or per-machine variable that would keep two renders apart. The documented list has none. Ruled out as absent, and reported as such above.
- **Auto-save's use of a process ID.** `bpy.types.PreferencesFilePaths` says "Automatic saving of temporary files in temp directory, uses process ID", which is the one place in 5.1 where Blender is documented to distinguish its own processes on disk. It concerns blend-file auto-save in the temporary directory, not render output. Ruled out.
- **The 5.1 release notes**, searched for `overwrite`, `placeholder`, `output path`, `render output`, `file output`, `cancel`. Nothing about output-folder sharing or concurrency. Ruled out.
- **Cycles' own files.** Searched all Python and all of `source/` for `overwrite`, `placeholder`, `filepath`. Overwrite and Placeholders are Blender-level render settings and appear nowhere in Cycles; the `filepath` hits are the standalone denoise and merge operators in `operators.py`. Ruled out.

---

## Things a person with Blender open could settle by hand in a few minutes

**Pruned on 2026-08-11, and pruned again on 2026-08-12** after the second batch of hand tests. Every item below was carried out, in all four researched versions, on the Creator's Mac and on a rented Linux machine; what came back is written into the answers above as observed findings, and the list they were run from is `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`. What is left open is what a running Blender on either of those two machines could not reach:

1. **Two renders on two separate computers writing into one shared folder** — the arrangement the manual's own render-farm hint describes. The project's two machines have no file system shared between them, and sharing one would mean installing software on the Creator's working machine. A network file system *was* tested, with both renders on the rented Linux machine writing to a folder mounted over NFS from itself, and it changed the answer.
2. **Anything on Windows.** No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
3. **A real power cut, as against a kernel reset.** The reset was done twice, on 2026-08-11 and again on 2026-08-12, and the two runs do not agree about which directory entries survive. Both results are written in above. Pulling a plug is still not reachable on either machine.
4. **Whether Blender receives an Escape key sent by a tool, and so what a person pressing Esc by hand would get.** Whether the render *stops* has been run three times on the rented Linux machine and has come out both ways; all three runs are written in above and none has been chosen. No run of this shape can show whether the key reached Blender at all.
5. **What a person clicking the status bar's cancel button leaves behind.** That control needs a hand on a mouse.

The items are kept below as written, since each names the exact set-up that was used.


Both answers are limited by the same silence, and most of it is cheap to break by experiment. Each item below says exactly what to do.

**For question 13**

1. **Does a stopped write leave a file, and how big is it?** Open Blender. In Output Properties set the file path to an empty folder, File Format to PNG, resolution to something large (say 7680 × 4320) so the write takes a visible moment. Set the frame range to 1–20. Start Render Animation. While it runs, watch the folder in a file manager sorted by size, and press Esc at a random moment. Then look: is there a file for the frame that was in progress, and if so, what size is it and does it open? Repeat five or six times so you catch the write itself and not just the render.
2. **The same, but killed from outside.** Run `blender -b scene.blend -o /path/to/empty/folder/frame_#### -F PNG -a` from a terminal, and kill the process (Ctrl-C, then separately with a hard kill) at random moments. Compare what the folder holds in each case with what step 1 produced. This is the case that matters for a machine going down.
3. **Does the file appear under its final name from the start?** Repeat step 1 or 2 with the folder open, and watch whether any file with a different or extra name (anything ending `.tmp`, `.part`, or with a leading dot) ever appears and then vanishes. A single run with a very large EXR gives the longest window to see it.
4. **Does format change the answer?** Repeat step 2 three times, once with PNG, once with OpenEXR (Codec: None, so the file is large and slow to write), once with JPEG. Note for each whether an interrupted write leaves nothing, a short file, or a full-size unreadable file.
5. **What does a placeholder actually look like mid-render?** Enable Placeholders in Output Properties, start Render Animation, and while frame 3 is rendering check the size of frame 3's file on disk. Then let it finish and check the size again. This settles the inference I flagged above.
6. **Does the EXR preview copy appear?** Set File Format to OpenEXR, enable the Preview option, render an animation of three frames, and list the folder. Confirm whether a JPG sits beside each EXR, and whether it appears before or after the EXR.

**For question 14**

7. **Two renders, defaults, same folder.** Save one blend-file. From two terminals, start `blender -b scene.blend -o /shared/folder/frame_#### -F PNG -s 1 -e 20 -a` at the same moment (paste both commands, then hit Return in each). Leave Overwrite on and Placeholders off, which is what a fresh file has. When both finish, list the folder: how many files, what sizes, and does every frame open correctly? Look especially for a file that is smaller than its neighbours.
8. **Two renders, the manual's recipe.** In the blend-file, disable Overwrite and enable Placeholders, save, then repeat step 7. Afterwards, check that every frame in 1–20 exists, that none is zero bytes, and that none is unreadable. Then check each render's own console output to see which frames each one claims to have rendered — this shows whether the work actually divided, and whether any frame was rendered twice.
9. **Force the collision.** Repeat step 8 with a very short frame range (1–3) and a very fast scene, so both renders are certain to reach frame 1 together. Run it ten times, and count how often a frame ends up rendered by both.
10. **Over a network share.** Repeat step 8 with the output folder on a network file system rather than a local disk, since that is what the manual's hint actually recommends. Compare the result with step 8's.
11. **What does the placeholder-skipping render report?** With Overwrite off, put a hand-made empty file at frame 5's exact output path (`touch` it, or create an empty file with that name), then render frames 1–10. Does frame 5 get rendered, skipped, or overwritten? This settles whether "already present" means "any file" or "a valid image".
12. **Is there a per-run token for the path?** In the Output path field, try `{blend_name}` to confirm templating works, then try a few plausible names for a per-process variable and see whether Blender substitutes anything or leaves the text alone. This confirms from the program itself that there is no built-in way to keep two runs' filenames apart.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Frames, Blender 5.1

2026-08-12 — Claude Opus 5 (Claude Code) — the second batch of hand tests written in: entries 22, 23, 24 and 25 run and recorded under question 13, both standing lines brought up to date, and the researcher's own list pruned again.
