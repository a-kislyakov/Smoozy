# Frames — cross-sweep questions, second pass (Blender 5.0)

Questions answered: 13 and 14 only.

**Sources consulted, and when.** Blender 5.0's manual, Blender 5.0's Python API documentation, Blender 5.0's release notes, and Cycles' own files as supplied — all four as the copies held in `Blender Sources/`, taken on 2026-07-30. That snapshot date is the date read for everything cited below. One picture from the manual was opened and is cited where it is used. Nothing outside `Blender Sources/` was used, and the web was not consulted.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I started no helper agents; all the searching and reading behind this document was done by me, so the line above covers the whole of it.

---

## 13. When a render is stopped at the exact moment it is writing a frame's own image file, what is left where it writes?

**LABEL: not in the sources.**

**A note on the question.** I read "a frame's own image file" as the per-frame image file that an animation render writes for that frame — the individual file, as opposed to a video file that holds all frames. The other reading would be a frame's file written by something else in the render, such as the compositor's File Output node. My searching covered both, and the sources are silent in the same way for both; nothing below changes under either reading.

**The answer.** The Blender 5.0 sources never say what sits at the output path if a render is stopped part-way through writing one frame's image file. They do not say whether Blender writes a frame under a temporary name and renames it at the end, whether it writes straight into the final file, whether an interrupted write leaves a part-sized file, an empty file, or no file at all. I searched for this in the plain words and the trade words and found nothing at all (the record is below). This is a real silence, not an unsearched corner.

What the sources *do* say, near this question but short of answering it:

**(a) A frame's file is written after that frame has rendered, and writing takes its own measurable time.** The release notes' example of Blender's console output shows the saving of a frame as a step of its own, with its own duration, and shows the path being reported only once saving is done:

> ```
> 00:26.799  render           | Mem: 2983M | Sample 16/16
> 00:28.631  render           | Saved: '/tmp/0001.png'
> 00:28.631  render           | Time: 00:28.38 (Saving: 00:00.15)
> ```

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30.

The step from that quote to "there is therefore a window of time during which a frame's file is part-written" is one a reader has to take; the source does not take it. So: **inferred**, and the step is exactly that — saving is shown as an interval with a start and an end, so it can be interrupted in the middle. What is on disk during that interval is still not stated anywhere.

**(b) There is a moment defined as "directly after the frame is written".** The Python API defines a hook that fires then:

> "on writing a render frame (directly after the frame is written)"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.app.handlers.html (entry `bpy.app.handlers.render_write`) — read 2026-07-30.

The same page defines a separate hook for a cancelled render:

> "on canceling a render job. Accepts one argument: the scene data-block being rendered"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.app.handlers.html (entry `bpy.app.handlers.render_cancel`) — read 2026-07-30.

Documented: the two moments are distinct and separately named. Not documented: anything about the state of the file itself when the second happens instead of the first.

**(c) The sources name only the final path, and never mention any temporary or intermediate file.** The command line page describes writing as going straight to the numbered name:

> "//render_ becomes //render_####, writing frames as //render_0001.png"

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

And the manual says frames go to disk as they are made:

> "animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/animation.html — read 2026-07-30.

I searched specifically for a temporary-file or safe-write mechanism and found none named anywhere. But an absence of mention is not proof that there is none — documentation routinely omits internals. So I do **not** claim Blender writes in place. The honest statement is: the sources describe only the final file name, and describe no other file.

**(d) One case where a file at that path definitely exists before the pixels do.** There is an optional setting that creates the frame's file empty:

> "Create empty placeholder files while rendering frames (similar to Unix 'touch')"
> Type: boolean, default False

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html (property `use_placeholder`) — read 2026-07-30.

The manual's wording for the same setting:

> "Create empty placeholder frames while rendering."

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30.

It is off unless switched on. The manual's picture of the Output panel shows the checkbox labelled **Placeholders**, unticked, beside **Overwrite**, ticked — `_images/render_output_properties_output_panel.png`, on https://docs.blender.org/manual/en/5.0/render/output/properties/output.html. **Documented** (from the picture): those are the two labels and those are their states in the panel as pictured.

So with that setting on, a file of zero length is created for the frame at some point during rendering. Whether it is created before the frame renders or at the moment of writing is **not** stated — the words are only "while rendering frames". Even here, the sources do not say what the file holds if the write is cut off half-way.

### What running it settled — the hand tests of 2026-08-11

**LABEL: observed.** Twenty-one tests, built from all eight of this round's findings documents and the four audits and saved at `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, were run in Blender 4.5.12, 5.0.1, 5.1.2 and 5.2.0 on two machines. **Machine one, the Creator's Mac:** a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card. **Machine two, a rented Linux machine:** Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Every run had a settings folder of its own and `--factory-startup`. Everything quoted is Blender 5.0.1's own output; the two machines are never merged. The same results are in this version's first-pass document, in fuller form.

**Blender writes straight to the final name, and no temporary name is ever used.** Three frames at 4096 by 4096 as OpenEXR with no compression, the whole folder listed every two milliseconds. The only names that ever appeared were the three final frame names:

> ```
> WATCH    0.000  <empty>
> WATCH    1.297  frame_0001.exr=1082262
> WATCH    1.300  frame_0001.exr=3298198
> ```

and 113 distinct sizes later the file reached 268,501,910 bytes. — Creator's Mac; the rented Linux machine did the same, ending at 268,501,871 bytes. **The two machines agreed.** This answers the question this document deliberately left open: the sources describe only the final file name because that is the only name there is.

**How the stop is delivered decides what is left.** Each run was stopped the instant frame 2's file first appeared. An **interrupt** did not cut the write: Blender finished the frame — `00:02.427  render           | Saved: '…/frame_0002.exr'`, `Time: 00:01.03 (Saving: 00:00.35)`, then `Sent an internal break event. Press ^C again to kill Blender`, exit code 1 — and the file was whole and opened. A **terminate** left 923 bytes on the Creator's Mac and 32,772 on the rented Linux machine, exit code -15, no `Saved:` line, unopenable. A **kill** left 21,403 and 8,196 bytes, exit code -9, unopenable. So a file **does** exist at the frame's own path after a stop that is not an interrupt, and it is short.

**The format matters, and a part-written file is not always distinguishable from a whole one.** The same kill per format at 4096 by 4096: PNG left 0 bytes on both machines; JPEG 0 on both; OpenEXR 33,695 on the Mac and 32,768 on Linux; TIFF 0 on the Mac and **1,306 on Linux, which opened and reported 4096 × 4096**, against a complete TIFF of 5,399,366 bytes. Multi-layer OpenEXR needed `media_type` set to `MULTI_LAYER_IMAGE` first — with Media Type at Image the format is not in the enum at all, which is a difference from 4.5, where there is no `media_type` — and then wrote one file per frame, 537,036,580 bytes complete and 50,980 after a kill.

**Nothing was still being held back by the operating system.** After a kill the folder was listed at once and again sixty seconds later; the two listings were identical, on both machines and in all four versions.

**The placeholder is created before the frame is rendered, one frame at a time.** With Overwrite off and Placeholders on: `WATCH 0.300 frame_0001.exr=0`, with the first real bytes only at 1.254 seconds, and frame 2's file not appearing until frame 1 was finished. That settles what this document says is not stated. Stopped once frame 2's file had grown past zero, the leftover was 33,687 bytes on the Creator's Mac and 36,864 on the rented Linux machine, neither opening. **The two machines agreed.**

**The JPEG preview copy exists and is written after each frame.** With `use_preview` on and OpenEXR output the folder held a `.jpg` beside every `.exr` — `Saved: '…/frame_0001.exr'` at 00:00.528 and `Saved: '…/frame_0001.jpg'` at 00:00.606. That is the extra file this document names under *Not covered*.

**A Compositor File Output node writes before the render's own frame file.** Stopped the instant the node's own second file appeared, the folder held it at 21,210 bytes on the Creator's Mac and 0 bytes on the rented Linux machine, unopenable, and **no Output-panel file at all for that frame**. In 5.0 the compositor is a node group on `scene.compositing_node_group` and the node takes a `directory` and a `file_name`, where 4.5 uses `scene.node_tree` and a `base_path`. *(The citation check of 2026-08-11 could not repeat this. It confirmed from Blender itself that the scene carries a `compositing_node_group` and that the node's properties are `directory` and `file_name` rather than `base_path`, but what is written here does not say how the node is wired into the new compositor, and the node wrote no file at all when it was set up from this description. The 4.5 result repeated exactly. Written out as item 24 on the hand-tests list.)*

**Cancelling from inside Blender's own window did not stop the render.** On the rented Linux machine only: a forty-frame render in a real window with the render window holding the keyboard, Escape sent to every Blender window repeatedly for ten seconds with `xdotool`. Fifteen seconds later the render was still going and went on writing whole frames, in all four versions. This shows the render did not stop; it cannot show Blender received the key.

**The citation check of 2026-08-11 repeated this and got the opposite, and both runs are recorded rather than one replacing the other.** Following the procedure written above on the rented Linux machine, the render **stopped**: sixteen frames stood in the folder when the first Escape was sent and sixteen stood there fifteen seconds after the last one, in Blender 4.5, run twice; in 5.2 it was fourteen and fourteen. A control identical in every respect except that no Escape was ever sent went from sixteen frames at twenty-five seconds to thirty-six at fifty seconds in 4.5, and from fourteen to thirty-three in 5.2 — so without the Escape the render kept going, and with it the render stopped. That is a question, not an answer. It is item 22 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, to be run again, and until it is run nothing downstream should treat either result as the fact.

**A machine going down loses even frames it had already reported saving.** On the rented Linux machine only, the machine was reset at the kernel — `echo b > /proc/sysrq-trigger`, nothing flushed — the instant frame 2's file appeared. Afterwards **the output folder the run had made did not exist at all**, and neither did the run's log file. Frame 1 had been reported saved seconds earlier. A plain `reboot` had been tried first and the machine came back from it. This is the *unflushed write* case, and it comes out differently from every kill above. *(The citation check of 2026-08-11 did not repeat this: the kernel reset was refused by the permission system of the machine that check ran on. The result stands on its single run, and is item 25 on the hand-tests list.)*

### What the second batch of hand tests settled — 2026-08-12

**LABEL: observed.** Four more tests — entries 22 to 25 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md` — were added by the citation check of 2026-08-11: three because a repeat came back differently from a run recorded above, and one because a procedure could not be followed as written. They were run on 2026-08-12 on the machines each entry names, on the same two machines described above, each with a settings folder of its own and `--factory-startup`. Everything quoted is Blender 5.0.1's own output, and the two machines are never merged. The same results are in this version's first-pass document, in fuller form. **Where a result differs from one written above, both stand and neither has been chosen.**

**Escape sent to Blender's own window stopped the render — entry 22.** On the rented Linux machine only, the one machine with a screen this round could drive. A forty-frame render at 480 by 480 with eighty samples, PNG, started in a real window from a timer. At twenty-five seconds the folder held 35 files, `xdotool getwindowfocus getwindowname` answered `Blender Render`, and `xdotool search --name Blender` answered `62914562 62914691`. Escape was sent to both windows and to the focused window twenty-seven times over ten seconds; the folder held 35 files ten seconds later and 35 fifteen seconds after that. A control in the same sitting with no Escape sent went 36, then 40, then 40. The other three versions came out the same way — 4.5 froze at 30, 5.1 at 38, 5.2 at 33, while every control reached all 40 frames of the range. **This agrees with the citation check's repeat and disagrees with the run recorded above. Both stand.** It still cannot show that Blender received the key.

**A stop leaves a different number of bytes almost every time, for JPEG and for TIFF — entry 23.** Ten repeats per format per version per machine: the three-frame render at 4096 by 4096 was killed the instant frame 2's file appeared, and its size and whether it opened were recorded. In Blender 5.0 on the **Creator's Mac** the ten JPEG leftovers were 0, 0, 0, 0, 0, 0, 0, 0, 4,096 and 0 bytes, one opening; the ten TIFF leftovers were 8, 0, 1,306, 8, 1,306, 1,306, 0, 8, 1,306 and 1,306 bytes, four opening — and of the five files of exactly 1,306 bytes, four opened and one did not. On the **rented Linux machine** the ten JPEG leftovers were 0, 0, 4,096, 0, 4,096, 4,096, 0, 0, 4,096 and 0 bytes, four opening; the ten TIFF leftovers were 1,306, 8, 8, 1,306, 1,306, 1,306, 238, 1,306, 1,306 and 8 bytes, six opening. Complete frames were 4,127,962 and 5,397,370 bytes on the Mac and 4,113,616 and 5,399,366 on the rented machine. **Every cell is a spread**, so the single figures written above, and the different single figures the citation check got, are draws from a race rather than facts about a version or a machine.

**The Compositor File Output node set-up written out, and entry 7 run again from it — entry 24.** On the Creator's Mac. The script the hand-test list carries was run again in 5.0 unchanged and the node **did** write its files, so it is the description in this document, not the list's script, that could not be rebuilt from. Taking the set-up apart one piece at a time showed which pieces the node needs: without the item added by `file_output_items.new`, without the link into that input, without a Render Layers node feeding it, without `use_compositing`, or without the group being put on `scene.compositing_node_group`, the node wrote nothing; without the group's output socket and Group Output node it still wrote. Blender 5.1 and 5.2 gave the same seven answers. So the set-up is: a node group of kind `CompositorNodeTree` on `scene.compositing_node_group`; a Render Layers node inside it; a File Output node with an item added by `file_output_items.new("RGBA", "Image")`, which is what creates the input; a link from the Render Layers node's Image output into that input; `use_compositing` switched on; and the node's `directory` and `file_name` set. With that, entry 7 ran as written: a complete run printed the node's save first every time — `00:01.623  render           | Saved: '…/node/nodeImage0001.exr'` before `00:02.264  render           | Saved: '…/out/frame_0001.png'` — and a run killed the instant the node's own second file appeared left `nodeImage0002.exr` at **721 bytes**, which did not open, and **no Output-panel file at all for frame 2**. In 5.1 the same stop left 0 bytes and in 5.2 13,013 bytes, neither opening. That repeats the shape of the result recorded above.

**The kernel reset, run a second time, left the folder present and empty in every version — entry 25.** On the rented Linux machine only, run last. A plain `reboot` was tried first and the machine came back. Once per version, the three-frame OpenEXR render was started with no window and the machine reset at the kernel the instant frame 2's file appeared. In Blender 5.0 the output folder came back present and `<empty>`, the run's own log file did not exist at all, and the blend-file survived whole at 92,519 bytes. Frames 1 and 2 were both absent, and frame 1 had been reported saved before the reset. The folder was present and empty in 4.5, 5.1 and 5.2 as well. **That disagrees with the run recorded above**, which had no folder at all in 5.0. Both stand. What came out the same way twice: no frame's contents survived in any version, including a frame already reported saved.

**Not covered.** This answer does not settle: whether an interrupted write leaves a part-sized file, an empty file, or nothing; whether the answer differs by image format (PNG, OpenEXR, JPEG, multi-layer OpenEXR all write differently in the general world, and the sources say nothing about interruption for any of them); whether it differs between a polite cancel (pressing the cancel control) and the process being killed outright or the machine losing power; whether anything is cleaned up afterwards; and what happens to the extra files a frame can produce (for example the optional JPEG preview — "When rendering animations, save JPG preview images in same directory", `use_preview`, default False, Python API documentation, https://docs.blender.org/api/5.0/bpy.types.ImageFormatSettings.html, read 2026-07-30).

**Outside the interface.** No difference is documented, because nothing is documented. The one thing worth saying: the console output quoted in (a) is from the release notes' logging example and is explicitly about running without the window — the release notes say the new logging covers "operators reports and background render progress" (https://developer.blender.org/docs/release_notes/5.0/core/, read 2026-07-30). So the saving step and its timing are visible when Blender is driven from outside its window. What is left on disk mid-write is unstated in both cases alike. **Brought up to date by the hand tests of 2026-08-11:** every result under *What running it settled* was got from a render driven from outside Blender's window, with no window at all, so the answer holds outside the interface — that is where it was measured. Sending Escape to Blender's own window on the rented Linux machine did not stop the render, so what a person pressing Esc or clicking the status bar's cancel button would get is the only part not established from outside. **Brought up to date again by the second batch of hand tests, 2026-08-12:** the Escape test was run again with a control and this time the render **stopped**, in all four versions — entry 22 above. The two runs disagree and neither has been chosen. The hand half stays open either way, because no run so far can show that Blender received the key.

**Operating system.** The sources do not say. Nothing in any page I read makes the behaviour of writing a frame's file conditional on the operating system. The phrase "similar to Unix 'touch'" in the placeholder tooltip is a comparison used to explain what an empty file is; it does not state that the setting is limited to one family of systems, and I did not treat it as such. **Brought up to date by the hand tests of 2026-08-11:** everything above was run on macOS 26.5.2 on an Apple M1 Pro **and** on Ubuntu 22.04.5 LTS on an AMD Threadripper, and the two agreed on every point of behaviour; where they differ is in numbers that are races. **Windows is untested**, no Windows machine having run it, so this answer covers macOS and Linux and leaves Windows open. **Brought up to date again by the second batch of hand tests, 2026-08-12:** entry 23 ran that stop ten times per format per version on each machine, and both machines gave a spread of sizes and a mixture of files that opened and files that did not, which shows the race directly. Entries 22 and 25 were run on the rented Linux machine only, so what an Escape does inside a window and what a machine going down leaves are answered for Linux and left open for macOS.

**Where I looked (question 13).**

Searched the whole of the 5.0 manual, the whole of the 5.0 Python API documentation, and the whole of the 5.0 release notes for, case-insensitively: partial, partially written, partial file, incomplete, truncat*, corrupt, broken, damaged, unreadable, invalid image, half-written, unfinished, cut short, torn, atomic, temporary file, temp file, lock file, file locking, flush, written to disk, file is written, empty file, zero byte, 0 bytes, while saving, during saving, mid-write, cancel, interrupt, abort, terminate, kill, Ctrl-C, power failure, power outage, crash, resume, re-render, Saved:, Saving:.

Pages opened in full and ruled out: manual — Rendering Animations (https://docs.blender.org/manual/en/5.0/render/output/animation.html; it covers resuming at frame level, never file level), Output introduction (https://docs.blender.org/manual/en/5.0/render/output/introduction.html; says a job "can be canceled and resumed at the last rendered frame by changing the frame range", which is about which frame to restart from, not about the file that was being written), Output panel (https://docs.blender.org/manual/en/5.0/render/output/properties/output.html), Command Line Arguments (https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html), Status Bar (https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html; documents that "The task can be aborted by clicking the cancel button" and nothing about what the abort leaves), Image Formats (https://docs.blender.org/manual/en/5.0/files/media/image_formats.html; format options only, nothing about how a file is written), Crashes and Recovering Data under Troubleshooting (both are about blend-files and crash logs, not render output — ruled out on that basis). Python API — `bpy.types.RenderSettings`, `bpy.app.handlers`, `bpy.ops.render` (the render operator itself is marked "Undocumented, consider contributing"), `bpy.types.ImageFormatSettings`. Release notes — all twenty-three 5.0 subject files scanned for the terms above; only the Core page's logging section was relevant. *(Corrected by the citation check of 2026-08-11: the count was written as twenty-four; `release_notes_5.0` holds twenty-three files.)*

Cycles' own files: checked and ruled out. What is supplied is Cycles' add-on layer (`__init__.py`, `properties.py`, `engine.py`, `ui.py`, `operators.py`, `camera.py`, `osl.py`, `presets.py`, `version_update.py`) and its kernel and utility headers under `source/kernel` and `source/util`. Frame files are written by Blender itself, not by Cycles, and nothing in the supplied files touches saving a rendered frame to disk. Searched those files for: overwrite, placeholder, filepath, write.

---

## 14. When two renders write into one folder and are started at the same instant, what does each of them produce, and what does the folder hold at the end?

**LABEL: not in the sources** — with one documented arrangement that partly speaks to it, given below.

**A note on the question.** "Two renders" I read as two separate rendering runs — two copies of Blender, on one machine or two — whose output path points at the same folder. The question does not say whether their frame ranges overlap or whether their output settings match; I answer the general case and say where the answer would turn on that.

**The answer, in two halves.**

**Half one — what the sources do say.** There is exactly one place in the 5.0 documentation that describes more than one render writing into one folder, and it is a recipe rather than a description of behaviour:

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

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30.

The two settings it names:

> "Overwrite existing files when rendering."

and

> "Create empty placeholder frames while rendering."

Same page, same date.

Their defaults, from the Python API documentation (https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html, read 2026-07-30):

> `use_overwrite` — "Overwrite existing files while rendering" — Type: boolean, default True
>
> `use_placeholder` — "Create empty placeholder files while rendering frames (similar to Unix 'touch')" — Type: boolean, default False

The manual's picture of the Output panel agrees: **Overwrite** ticked, **Placeholders** unticked (`_images/render_output_properties_output_panel.png`, on https://docs.blender.org/manual/en/5.0/render/output/properties/output.html, read 2026-07-30). **Documented** from the picture.

So the documented arrangement is: *the recipe only works with both settings changed from their defaults.* Started as they come out of the box, two renders sharing a folder are not in the arrangement the manual describes.

Also documented, and it matters here: two renders with the same output path and the same frames aim at exactly the same file names. The naming rule is fixed and leaves no room for a per-run difference:

> "When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of # anywhere in the file name"

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30.

> "The # characters are replaced by the frame number, and used to define zero padding. […] When the filename does not contain #, the suffix #### is added to the filename."

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

**Inferred**, and here is the step: the name of a frame's file is built from the output path and the frame number alone, by the rules quoted; nothing in either rule varies with which copy of Blender is running; therefore two runs pointed at one folder with overlapping frame numbers and the same file name pattern compute the identical file name for a shared frame. The sources state the naming rule; they do not state this consequence, so it is not documented.

**Half two — what the sources do not say, which is most of the question.** Nothing in the 5.0 manual, Python API documentation, release notes, or the supplied Cycles files says:

- what happens when the two runs are started at the same instant, in the Primitive Render Farm arrangement or any other — the recipe says "Start as many machines as you wish", and says nothing about timing, ordering, or two machines reaching for the same frame together;
- whether a placeholder, once created, is created in a way that stops a second render creating it too, or whether two renders can both decide a frame is theirs;
- whether there is any locking, claiming, or reservation of a frame or a file between renders — I found no such word anywhere;
- what each run produces on its own in this situation: which frames it renders, which it skips, what it reports;
- what the folder holds at the end: whether every frame is present exactly once, whether some frames are written twice, whether any file ends up written by two runs at once;
- anything at all about the default arrangement, where both runs are set to overwrite and neither creates placeholders.

I want to be plain about the strongest temptation here: the Primitive Render Farm hint reads as if it explains a skip-what-is-already-there scheme, and one can easily supply the rest. The manual does not supply it. It states a recipe and a result at the level of "share the rendering workload". Everything past that would be me writing, not the source.

### What running it settled — the hand tests of 2026-08-11

**LABEL: observed.** The machines and the list are described under question 13 above. Everything quoted is Blender 5.0.1's own output; the two machines are never merged. This is where this document's refusal to supply the mechanism is settled by observation instead.

**Overwrite off skips a frame before rendering it.** A five-frame render, Overwrite off, with a complete PNG placed by hand at frame 3's name. The seconds elapsed run down the left:

> ```
>    7.353  00:07.128  render           | Saved: '…/frame_0002.png'
>    7.353  00:07.128  render           | Skipping existing frame "…/frame_0003.png"
>   10.777  00:10.552  render           | Saved: '…/frame_0004.png'
> ```

The skip is printed at the same thousandth of a second as the previous save, where each real frame cost three and a half seconds, and the file placed by hand was untouched. — Both machines, agreeing. 5.0, 5.1 and 5.2 write `Skipping existing frame`; 4.5 writes it in lower case.

**"Already there" means a name exists, whatever is behind it.** A complete PNG, a zero-byte file, nineteen bytes of text, a PNG of the wrong size and a **folder** were all put at frames 1 to 5's names; all five were skipped and none touched, and only frame 6 was rendered. — Both machines, agreeing.

**On the shipped defaults both renders render every frame.** Two renders of frames 1 to 8 started by one command: each printed `Saved:` for all eight, and the folder ended with eight readable files. — Both machines, agreeing. That is the case this document says is not covered anywhere.

**On the manual's recipe the two renders divide the frames between them — and can collide.** On the rented Linux machine one run reported `Saved: frame_0001.png`, `Skipping existing frame frame_0002.png`, `Saved: frame_0003.png`… and the other reported `Saved: frame_0001.png`, `Saved: frame_0002.png`, `Skipping existing frame frame_0003.png`… — **both saved frame 1**. Eight files were present at the end and all opened. On the Creator's Mac the same run divided cleanly with no frame taken twice.

**The placeholder is not a lock, and the file system decides how often it is raced.** The recipe was run over a single frame, both Blenders started by one command, into a fresh empty folder, **thirty times per version per machine**. Both runs saved frame 1: 0 of 30 in 4.5, 5.0 and 5.1 on the Creator's Mac and 2 of 30 in 5.2; 0 of 30 in all four on the rented Linux machine's own disk; and **5 of 30 in 4.5, 6 of 30 in 5.0, 4 of 30 in 5.1 and 4 of 30 in 5.2 over a folder mounted from that machine over NFS**. In all 240 runs the file was present and larger than zero bytes. So the claim on a frame can be lost, and it is lost far more often over a network file system than on a local disk — the arrangement the manual's hint recommends.

**Two renders over overlapping but different ranges.** One over frames 1 to 10 and one over 6 to 15, started together. On the defaults each wrote its whole range, so the overlapping five were written twice; on the recipe one saved 1 to 5 and skipped 6 to 10 while the other saved 6 to 15, fifteen files each written once. — Both machines, agreeing.

**A placeholder left by a killed render blocks its frame for good.** Killed the instant frame 2's file appeared, the folder held frame 1 whole and frame 2 at 0 bytes. Run again to the end, the render printed `Skipping existing frame "…/frame_0002.png"` and left frame 2 at 0 bytes, unopenable. With two renders and one killed part-way, the survivor finished the rest and left the killed one's claimed frame at 0 bytes. — Both machines, agreeing.

**With a video output the two settings do nothing.** Both renders wrote to one file named `vid0001-0008.mp4`, both printed `Video append frame 1` through `Video append frame 8`, and one file was left that read back as 8 frames — the same on the defaults and on the recipe. — Both machines, agreeing. That is the tenth of this document's own hand tests. *(Corrected by the citation check of 2026-08-11: the words quoted here were `Append frame 1` through `Append frame 8`, which is what Blender 4.5 prints. Blender 5.0 prints `Video append frame 1` and so on, behind the usual logging prefix. Checked in all four versions.)*

**No path template identifies the run.** Of twenty-one names tried in the output path, Blender 5.0 substituted nine — `blend_name`, `blend_dir`, `blend_name_lib`, `blend_dir_lib`, `fps`, `resolution_x`, `resolution_y`, `scene_name`, `camera_name` — and every other, including `pid`, `hostname`, `uuid` and `random`, came back as `Error: Parse errors in path`. Two runs of the same file returned identical paths for every name. Blender 4.5 substitutes only four of the nine. — Both machines, agreeing.

**The two settings take from outside the window.** Set through Blender's Python before the render — `SET use_overwrite: False | use_placeholder: True` — the second of two runs later reported `Info: No frames rendered, skipped to not overwrite` where the first wrote over everything. — Both machines, all four versions, agreeing.

**Not covered.** This answer does not settle: the timing question the question actually turns on (two renders starting at the same instant); what happens with the default settings; whether two runs can collide on one file; what the folder holds when they finish; whether the answer changes for a video output rather than an image sequence, or for a multi-layer OpenEXR; whether it changes on a network file system as against a local disk; and whether two runs on one machine differ from two on two machines.

**Outside the interface.** The two settings involved are ordinary scene render settings — they appear in the Output panel and as properties on `bpy.types.RenderSettings` (https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html, read 2026-07-30). **Inferred**, step written out: because they are scene properties saved with the file and reachable from Python rather than window commands, a render started from outside Blender's window uses whatever they are set to in the file or in script, exactly as a render started by clicking would. The sources do not say this in so many words and do not describe the Primitive Render Farm recipe as an inside-the-window-only or outside-the-window-only technique. One thing is documented for the outside case and is worth carrying: the 5.0 release notes warn that the console output changed and that "Add-ons or render farms parsing this output may need to be updated" (https://developer.blender.org/docs/release_notes/5.0/python_api/, read 2026-07-30), with the new format shown at https://developer.blender.org/docs/release_notes/5.0/core/. So anything reading a run's progress from its printed output is reading a format that changed in 5.0. **Brought up to date by the hand tests of 2026-08-11:** the two settings do take when a render is driven from outside Blender's window — set through Blender's Python before the render, they printed `SET use_overwrite: False | use_placeholder: True` and the run then skipped every existing frame where an unchanged run wrote over them. Every result under *What running it settled* was measured with no window at all, so this answer holds outside the interface.

**Operating system.** The sources do not say. The only environment the recipe names is "a shared directory over a network file system", which is not an operating system. No page I read makes Overwrite, Placeholders, or shared-folder rendering conditional on the operating system, and none discusses how different systems behave when two programs write one file. As with question 13, the "similar to Unix 'touch'" phrase is an explanation of what an empty file is, not a statement that the feature is limited to one family of systems. **Brought up to date by the hand tests of 2026-08-11:** every test above was run on macOS 26.5.2 on an Apple M1 Pro **and** on Ubuntu 22.04.5 LTS on an AMD Threadripper, and the two agreed on every point of behaviour. The one number that differed is how often two renders collide on one frame, which is a race, not an operating-system difference. **The file system does change it**: over a folder mounted by NFS the collision rate went from none in thirty repeats to four, five or six in thirty. **Windows is untested**, no Windows machine having run it.

**Where I looked (question 14).**

Searched the whole of the 5.0 manual, the whole of the 5.0 Python API documentation, and the whole of the 5.0 release notes for, case-insensitively: render farm, network file system, network, shared directory, shared drive, remote drive, NFS, SMB, multiple machines, several machines, other machines, multiple instances, another instance, distributed render, same directory, same folder, same output, in parallel, at the same time, simultaneous, concurrent, race condition, conflict, clash, lock file, file locking, exclusive access, atomic, already exists, existing file, overwrite, use_overwrite, placeholder, use_placeholder.

What those searches turned up and what I did with it. "Placeholder" in a rendering sense exists in exactly two places in 5.0 — the Output panel page of the manual and the `use_placeholder` property in the Python API — both quoted above. Every other hit for "placeholder" is a different feature (the Video Sequencer's "Use Placeholders" for missing frames of an image strip, in `bpy.ops.sequencer`; text and font placeholders; add-on import options) and was ruled out as unrelated to render output. "Render farm" appears four times in all of 5.0: the Primitive Render Farm hint quoted above; a Metadata page note that the Note field "can be useful […] if you are setting up a render farm" (https://docs.blender.org/manual/en/5.0/render/output/properties/metadata.html) — ruled out, it is about stamping text into images; a Cycles Hair page remark about "multi device rendering or render farms with mixed hardware" — ruled out, it is about hair shape settings; and the logging warning quoted above.

Pages opened in full and ruled out: manual — Output panel, Rendering Animations, Output introduction, Command Line Arguments (its Render Options list has no option for sharing a folder, claiming frames, or coordinating runs), Status Bar, Cycles Sampling. On that last one: it does describe two machines rendering at once, but of one frame, each writing its own separate file, and combining them afterwards — "A typical use case is to distribute the rendering of a single frame over multiple machines", with the example combining `1.exr` and `2.exr` into `combined.exr` (https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html, read 2026-07-30). That is two renders writing different names, not two renders sharing a folder's worth of frame names, so I ruled it out as an answer to this question. Python API — `bpy.types.RenderSettings` (read in full for anything about sharing, claiming, or existing files; `frame_path()` is documented only as "Return the absolute path to the filename to be written for a given frame"), `bpy.app.handlers`, `bpy.ops.render`, and `info_advanced_blender_as_bpy` (which says of running Blender as a library that "it's possible to start multiple instances, each with their own bpy and therefor Blender state" — ruled out, it is about Python state in separate processes, and says nothing about them writing into one folder). Release notes — all twenty-three 5.0 subject files scanned; nothing about overwrite, placeholders, output-folder sharing, or concurrent renders. *(The file count was corrected from twenty-four to twenty-three by the citation check of 2026-08-11.)* Cycles' own files — same set as for question 13; the only multi-render machinery there is the image-merging operator in `operators.py`, which combines separately named files a user names by hand, and which is used after both renders are finished, so it does not speak to two renders writing at once.

---

## Things a person with Blender 5.0 open could settle by hand in a few minutes

**Pruned on 2026-08-11, and pruned again on 2026-08-12** after the second batch of hand tests. Every item below was carried out, in all four researched versions, on the Creator's Mac and on a rented Linux machine; what came back is written into the answers above as observed findings, and the list they were run from is `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`. What is left open is what a running Blender on either of those two machines could not reach:

1. **Two renders on two separate computers writing into one shared folder** — the arrangement the manual's own render-farm hint describes. The project's two machines have no file system shared between them, and sharing one would mean installing software on the Creator's working machine. A network file system *was* tested, with both renders on the rented Linux machine writing to a folder mounted over NFS from itself, and it changed the answer.
2. **Anything on Windows.** No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
3. **A real power cut, as against a kernel reset.** The reset was done twice, on 2026-08-11 and again on 2026-08-12, and the two runs do not agree about which directory entries survive. Both results are written in above. Pulling a plug is still not reachable on either machine.
4. **Whether Blender receives an Escape key sent by a tool, and so what a person pressing Esc by hand would get.** Whether the render *stops* has been run three times on the rented Linux machine and has come out both ways; all three runs are written in above and none has been chosen. No run of this shape can show whether the key reached Blender at all.
5. **What a person clicking the status bar's cancel button leaves behind.** That control needs a hand on a mouse.

The items are kept below as written, since each names the exact set-up that was used.


These are the gaps above, written so somebody can carry them out without needing my reasoning.

**For question 13 — what an interrupted write leaves.**

1. Open Blender. In Output Properties, set the file path to an empty folder, set the format to PNG, and set the frame range to something long (say 1 to 200) with a scene heavy enough that each frame takes several seconds. Start Render Animation. Watch the status bar; the instant the progress bar shows a frame finishing and the console prints its sample count complete, click the cancel button on the status bar. Then look in the folder with a file manager and check the size of the highest-numbered file. Repeat several times to catch the moment. Record: is there ever a file whose size is smaller than a finished frame's, and can it be opened as an image?

2. The same test with the format set to OpenEXR, then to multi-layer OpenEXR, then to JPEG. Formats differ in how much they buffer before writing, so the answer may differ. Record file sizes and whether each opens.

3. The same test run from outside the window: `blender -b <file> -o <folder>/frame_#### -F PNG -a`, and stop it part-way — once with Ctrl-C in the terminal, once by killing the process outright, and if possible once by cutting the machine's power. Compare what the folder holds in each case. Ctrl-C and a hard kill may well differ, and that difference is exactly what is missing from the documentation.

4. During a long single frame's *saving* step (use a very large resolution and OpenEXR so saving takes a noticeable time), check with a file listing, repeatedly and quickly, whether a file appears at the final name immediately at zero length and grows, or appears only at full size at the end, or appears first under some other name. That settles the temporary-file question the sources never address.

5. Turn on Placeholders in the Output panel and repeat test 1. Check whether the placeholder for the frame being rendered appears at the start of the frame or only at the end, and what is left at that path after cancelling.

**For question 14 — two renders into one folder.**

6. Point two copies of Blender at one empty folder with the same output path, same format, and the same frame range, with the settings as they come (Overwrite on, Placeholders off). Start both as close to the same instant as you can — on one machine, two terminals with `blender -b … -a` launched together; better, a single command that starts both. Let them run to the end. Then list the folder: how many files, are any missing, are any of an odd size, and does every file open as an image? Also keep each run's printed output and compare which frames each one says it saved.

7. Repeat test 6 with the manual's recipe: Overwrite off, Placeholders on. Same questions. Additionally record, from the two runs' printed output, whether the two runs divided the frames between them and whether any frame was rendered by both.

8. Repeat test 7 with the frames deliberately started at the same instant on a shared network folder rather than a local disk, since the manual's recipe names a network file system specifically and network folders behave differently when two machines create the same file.

9. In test 7, watch the very first frame closely: do both runs create a placeholder for frame 1, or does exactly one? That single observation is what the whole recipe rests on and what the documentation never states.

10. Repeat test 6 with the output set to a video format rather than an image sequence, two runs into one folder with the same file name. Record what the folder holds and whether either file plays.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Frames, Blender 5.0

2026-08-12 — Claude Opus 5 (Claude Code) — the second batch of hand tests written in: entries 22, 23, 24 and 25 run and recorded under question 13, both standing lines brought up to date, and the researcher's own list pruned again.
