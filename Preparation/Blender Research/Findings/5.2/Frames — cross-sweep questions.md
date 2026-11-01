# Frames — cross-sweep questions (Blender 5.2)

Questions answered: 13 and 14.

**Sources consulted:** Blender 5.2 manual (text and one image), Blender 5.2 Python API documentation, Blender 5.2 release notes, and the Cycles 5.2 files supplied — all from `Blender Sources/`, snapshot taken 2026-07-30. That snapshot date is the date read for every quotation below. I ran nothing and rendered nothing.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 13. When a render is stopped at the exact moment it is writing a frame's own image file, what is left where it writes?

**Reading taken.** The question is ambiguous about which file is meant. I answer it as: an animation render writing an image sequence through the Output panel, stopped during the moment that one frame's image file is being saved to the output folder. The other reading is a file being written by a Compositor File Output node rather than by the Output panel; I checked that page too and it is equally silent (see the record of looking).

**LABEL: not in the sources.**

The 5.2 manual, the 5.2 Python API documentation and the 5.2 release notes say nothing about the state of an image file that was part-way written when the render was stopped. They do not say whether a part-written file is left behind, whether it is empty, whether it is deleted, or whether it can be told apart from a finished one.

What the sources *do* say, all of it short of the question:

**A frame render can be stopped, and this is a supported way of working.**

> "For example, when rendering animations to image file formats the render job can be canceled
> and resumed at the last rendered frame by changing the frame range."

Source: manual, https://docs.blender.org/manual/en/5.2/render/output/introduction.html — read 2026-07-30.

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering,
> since the frames you have already rendered will still be in the output directory."

Source: manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html — read 2026-07-30. This is about frames already rendered. It says nothing about the one that was in flight.

**The render statistics name a saving time beside a render time.** The Python API documents the statistics handler as receiving:

> "on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."

Source: API, https://docs.blender.org/api/5.2/bpy.app.handlers.html — read 2026-07-30. *Inferred* from this, and stated as inference: because saving is measured as its own span of time, there is a window during which a frame is being saved and is not yet saved. The step a reader must supply is that a measured "saving time" implies a period of writing. What is on disk during that window is not stated anywhere.

**There is a documented moment "after the frame is written".**

> "on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

Source: API, `bpy.app.handlers.render_write`, https://docs.blender.org/api/5.2/bpy.app.handlers.html — read 2026-07-30. There is also a cancel handler:

> "on canceling a render job. Accepts one argument: the scene data-block being rendered"

Source: API, `bpy.app.handlers.render_cancel`, same page and date. Neither says anything about the file on disk at the moment of cancelling.

**Empty placeholder files are documented as being created while frames are rendered**, if the Placeholders option is on:

> "Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

Source: API, `bpy.types.RenderSettings.use_placeholder`, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30. The manual's wording for the same option:

> "Create empty placeholder frames while rendering."

Source: manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. This tells you empty files can be created while frames are rendered. The quoted words do not say where such a file sits, whether one exists for the frame that is in flight, whether it is later filled in place or replaced, or what remains if the render stops between the two. The option is off by default, so it does not describe the ordinary case at all.

### What running it settled — the hand tests of 2026-08-11

**LABEL: observed.** Twenty-one tests, built from all eight of this round's findings documents and the four audits and saved at `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, were run in Blender 4.5.12, 5.0.1, 5.1.2 and 5.2.0 on two machines. **Machine one, the Creator's Mac:** a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card. **Machine two, a rented Linux machine:** Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Every run had a settings folder of its own and was started with `--factory-startup`. Everything quoted is Blender 5.2.0's own output; the two machines are never merged.

**Blender writes the frame straight to its final name, and the file grows in place.** Three frames at 4096 by 4096, OpenEXR at 32 bits with no compression, the whole folder listed every two milliseconds. Every name that ever appeared was a final frame name — no `.tmp`, no `.part`, no dot-prefixed or hidden name:

> ```
> WATCH    0.000  <empty>
> WATCH    1.588  frame_0001.exr=33690
> WATCH    1.591  frame_0001.exr=2904986
> WATCH    1.594  frame_0001.exr=5518234
> ```

and 109 distinct sizes later it reached 268,501,914 bytes. — Creator's Mac. On the rented Linux machine it began `WATCH 1.859 frame_0001.exr=745472` and ended at 268,501,875 bytes, again with only the three final names ever seen. **The two machines agreed.** The write is announced as a step of its own: `00:01.625  render           | Saved: '…/frame_0001.exr'` followed by `Time: 00:01.22 (Saving: 00:00.32)`. Blender 4.5 prints those two lines bare, with no elapsed time and no `render` before them.

**How the stop is delivered decides what is left.** Three runs, each stopped at the instant frame 2's file first appeared:

- **An interrupt (`SIGINT`)** did **not** cut the write. Blender finished writing the frame and then stopped: `00:02.703  render           | Saved: '…/frame_0002.exr'`, `Time: 00:01.04 (Saving: 00:00.32)`, then `Sent an internal break event. Press ^C again to kill Blender`, exit code 1. Frame 2 was 268,501,918 bytes and opened as a 4096 by 4096 image.
- **A terminate (`SIGTERM`)** left **33,695 bytes** at frame 2's path, exit code -15, no `Saved:` line, unopenable: `OPEN loaded | size: (0, 0) | channels: 0 | depth: 0 | has_data: False`.
- **A kill (`SIGKILL`)** left **0 bytes**, exit code -9, again unopenable.

— Creator's Mac. On the rented Linux machine: the interrupt again finished the write and left a whole 268,501,879-byte frame; the terminate left **16,384 bytes**; the kill left **32,768 bytes**. **The two machines agreed on the pattern**; the exact leftover size is a race and differs run to run.

**The format changes what the leftover looks like, and one format is dangerous.** The same kill, once per format, at 4096 by 4096. PNG left 0 bytes on both machines; JPEG 0 bytes on both; OpenEXR 5,027 bytes on the Mac and 32,772 on Linux; TIFF **1,306 bytes on the Mac, which opened** — `OPEN loaded | size: (4096, 4096) | channels: 4 | depth: 32 | has_data: True` — and 1,278 bytes on Linux, which did not. A complete TIFF was 5,398,529 bytes. So **a stopped-mid-write file is not always distinguishable from a finished one by asking whether it opens**; its size still tells them apart. That the same format opened on one machine and not the other is the byte count it happened to reach, not a difference between the machines.

**Multi-layer OpenEXR needs a different setting in 5.2 than in 4.5.** Setting `file_format` to `OPEN_EXR_MULTILAYER` while Media Type is Image is refused — the enum does not hold it — and with `media_type` set to `MULTI_LAYER_IMAGE` first it is accepted. One file per frame holds both view layers, 537,036,607 bytes complete and 55,103 bytes after a kill. Blender's own `bpy.data.images.load` reports `type: MULTILAYER | size: (0, 0) | has_data: False` for the **complete** file as well as the part-written one, so for that one format the reader used here cannot tell whole from part — a limit of the test, not a finding about the file.

**The operating system had already put everything on disk.** After a kill the folder was listed at once and again sixty seconds later; both listings were identical, on both machines and in all four versions.

**With Placeholders on, a file of zero bytes is put at the frame's path before that frame is rendered — one frame at a time.**

> ```
> WATCH    0.000  <empty>
> WATCH    0.753  frame_0001.exr=0
> ```

Frame 1's file appeared at **0 bytes** about a second before any pixels reached it, and frame 2's file did not appear until frame 1 was finished — 1.876 seconds in on the Creator's Mac. No file ever appeared for a frame the render had not reached. That settles the step this document marks as inferred. Stopping the run once frame 2's file had grown past zero left 33,691 bytes on the Creator's Mac and a short unopenable file on the rented Linux machine. **The two machines agreed.**

**The JPEG preview copy exists and is written after the frame.** With `use_preview` on and OpenEXR output, a `.jpg` sat beside every `.exr`, saved immediately after it — `Saved: '…/frame_0001.exr'` at 00:00.663 and `Saved: '…/frame_0001.jpg'` at 00:00.737. — Both machines, agreeing. Neither this document nor 5.2's second pass mentions that option; it exists in all four versions and behaves the same way in all four.

**A Compositor File Output node writes before the render's own frame file, and is set up differently from 4.5.** In 5.2 the compositor is a node group on `scene.compositing_node_group` and the node takes a `directory` and a `file_name`, so its files are called `nodeImage0002.exr`, where 4.5 uses `scene.node_tree`, a `base_path`, and `Image0002.exr`. Stopped the instant the node's own second file appeared, the folder held it at 13,022 bytes on the Creator's Mac — unopenable — and **no Output-panel file at all for that frame**. That is the reading of question 13 this document names as its other reading, answered by observation. *(The citation check of 2026-08-11 could not repeat this. It confirmed from Blender itself that the scene carries a `compositing_node_group` and that the node's properties are `directory` and `file_name` rather than `base_path`, but what is written here does not say how the node is wired into the new compositor, and the node wrote no file at all when it was set up from this description. The 4.5 result repeated exactly. Written out as item 24 on the hand-tests list.)*

**Cancelling from inside Blender's own window did not stop the render.** On the rented Linux machine only, a forty-frame render started from a timer in a real window, with the render window holding the keyboard (`the window that has the keyboard: Blender Render`), and Escape sent to every Blender window repeatedly for ten seconds with `xdotool`. Fifteen seconds after the first Escape the render was still going and went on writing whole frames. The same in all four versions. This shows the render did not stop; it cannot show Blender received the key, so it does not settle what an Escape pressed by a person would do.

**The citation check of 2026-08-11 repeated this and got the opposite, and both runs are recorded rather than one replacing the other.** Following the procedure written above on the rented Linux machine, the render **stopped**: sixteen frames stood in the folder when the first Escape was sent and sixteen stood there fifteen seconds after the last one, in Blender 4.5, run twice; in 5.2 it was fourteen and fourteen. A control identical in every respect except that no Escape was ever sent went from sixteen frames at twenty-five seconds to thirty-six at fifty seconds in 4.5, and from fourteen to thirty-three in 5.2 — so without the Escape the render kept going, and with it the render stopped. That is a question, not an answer. It is item 22 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, to be run again, and until it is run nothing downstream should treat either result as the fact.

**A machine going down loses even frames it had already reported saving.** On the rented Linux machine only, the same three-frame OpenEXR render was started and the machine was reset at the kernel — `echo b > /proc/sysrq-trigger`, with nothing flushed — the instant frame 2's file appeared. The machine restarted, and afterwards the output folder held **`frame_0001.exr` at 0 bytes and nothing else**, and the run's own log file was 0 bytes. Frame 1 had been reported saved seconds earlier; its name survived and its contents did not. A plain `reboot` had been tried first and the machine came back from it, so the loss is the reset's doing and not a failure of the step. The same reset left the folder empty in 4.5 and 5.1 and left no folder at all in 5.0; exactly which directory entries survive is what the file system happened to have written down, and no frame's contents survived in any version. This is the *unflushed write* case, and it comes out differently from every kill above. *(The citation check of 2026-08-11 did not repeat this: the kernel reset was refused by the permission system of the machine that check ran on. The result stands on its single run, and is item 25 on the hand-tests list.)*

**The 5.2 release note about switching saving off names a real setting, and it exists only here.** Asked of Blender itself, `RenderSettings.save_output` is present in 5.2 as `type=BOOLEAN | default=True | value=True | read_only=False | description='Write frames to disk for animation renders'`, and is **absent in 4.5, 5.0 and 5.1**. Both machines, agreeing. It is the only property in the render settings, in any of the four versions, whose name or description speaks of saving render output. Two other things in 5.2 are absent from 4.5 and are worth recording beside it: `ImageFormatSettings.media_type`, whose values are `IMAGE`, `MULTI_LAYER_IMAGE` and `VIDEO`, and `ImageFormatSettings.use_exr_interleave`; and 5.1 and 5.2 offer `AVIF` in the file-format list where 4.5 and 5.0 do not.

### What the second batch of hand tests settled — 2026-08-12

**LABEL: observed.** Four more tests — entries 22 to 25 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md` — were added by the citation check of 2026-08-11: three because a repeat came back differently from a run recorded above, and one because a procedure could not be followed as written. They were run on 2026-08-12 on the machines each entry names, on the same two machines described above, each with a settings folder of its own and `--factory-startup`. Everything quoted is Blender 5.2.0's own output, and the two machines are never merged. **Where a result differs from one written above, both stand and neither has been chosen.**

**Escape sent to Blender's own window stopped the render — entry 22.** On the rented Linux machine only, that being the one machine of the two with a screen this round could drive. A forty-frame render at 480 by 480 with eighty samples, PNG, into a fresh empty folder, started in a real window from a timer four seconds after launch. At twenty-five seconds the folder was counted and the machine was asked which window held the keyboard:

> ```
> COUNT at 25s, before any Escape: 32
> xdotool getwindowfocus getwindowname: Blender Render
> xdotool search --name Blender: 62914562 62914691
> ```

Escape was then sent to both of those windows and to the focused window, twenty-six times over ten seconds, and the folder counted again:

> ```
> COUNT at 35s, right after the Escape window: 33
> COUNT at 50s, fifteen seconds after the last Escape: 33
> ```

So one more frame was finished after the first Escape, and then the count did not move again. A control in the same sitting, identical except that no Escape was ever sent, went `COUNT at 25s: 32`, then 40, then 40. The other three versions came out the same way: 4.5 stayed at 30, run twice; 5.0 stayed at 35; 5.1 went 37 to 38 and stopped. Every control reached all forty frames of the range, which is what ended its own climb; every test stopped short of forty and did not move again. **This agrees with the citation check's repeat and disagrees with the run recorded above under *Cancelling from inside Blender's own window*. Both stand.** It still cannot show that Blender received the key, so what a person pressing Esc with their own hand would get is not settled by it.

**A stop leaves a different number of bytes almost every time, for JPEG and for TIFF — entry 23.** Ten repeats per format per version per machine, each into a fresh empty folder: the three-frame render at 4096 by 4096 was started with no window and killed outright the instant frame 2's file first appeared, then that file's exact size was recorded and Blender's own image loader was asked to open it. In Blender 5.2 on the **Creator's Mac** the ten JPEG leftovers were 8,192, 0, 0, 0, 0, 4,096, 0, 0, 4,096 and 0 bytes; three of the ten opened and reported 4096 by 4096, against a complete JPEG of 4,137,461 bytes. The ten TIFF leftovers were 238, 8, 254, 1,306, 1,306, 0, 8, 1,306, 8 and 8 bytes; three of the ten opened, against a complete TIFF of 5,398,529 bytes. On the **rented Linux machine** the ten JPEG leftovers were 0, 0, 4,096, 0, 0, 0, 0, 4,096, 0 and 0 bytes, two opening, against a complete JPEG of 4,123,208 bytes; the ten TIFF leftovers were 8, 1,306, 1,286, 1,306, 1,306, 1,306, 1,306, 1,306, 1,306 and 1,306 bytes, seven opening, against a complete TIFF of 5,400,506 bytes — and note that of the eight files of exactly 1,306 bytes, seven opened and one did not, so even the same size is not always the same file. **Every cell is a spread.** So the single numbers in the format table above — the 1,306 that opened on the Mac and the 1,278 that did not on Linux — and the different single numbers the citation check got are both draws from a race rather than facts about a version or a machine. A file of 0 bytes never opened, in any of the 160 repeats.

**The Compositor File Output node set-up written out, and entry 7 run again from it — entry 24.** On the Creator's Mac. First the script the hand-test list itself carries was run again, unchanged, and the node **did** write its files — so it is the description in this document, not the list's script, that could not be rebuilt from. Then the set-up was taken apart one piece at a time, one frame at 256 by 256, to see which pieces the node needs. Leaving each out in turn:

| what was left out | did the node write a file |
| --- | --- |
| nothing — the whole set-up | yes |
| the item added with `file_output_items.new`, linking into the node's own spare socket instead | no |
| the group's output socket and its Group Output node | **yes** |
| the link from the Render Layers node into the node's input | no |
| `use_compositing`, set to off | no |
| the Render Layers node, feeding the node from a group input instead | no |
| putting the group on `scene.compositing_node_group` at all | no |

Blender 5.0 and 5.1 gave exactly the same seven answers. So the set-up, written out: make a node group of kind `CompositorNodeTree` and put it on `scene.compositing_node_group`; put a Render Layers node in it, since nothing else feeds the File Output node — a group input does not; put a File Output node in it and add an item with `file_output_items.new("RGBA", "Image")`, because the node is born with no real input and that call is what makes one; link the Render Layers node's Image output to that input; switch `use_compositing` on; and set the node's `directory` and `file_name`. A group output socket is **not** needed. With that set-up, entry 7 ran as written. A complete three-frame run at 4096 by 4096, PNG from the Output panel and 32-bit uncompressed OpenEXR from the node, printed the node's save first every time:

> ```
> 00:02.533  render           | Saved: '…/node/nodeImage0001.exr'
> 00:03.231  render           | Saved: '…/out/frame_0001.png'
> 00:03.231  render           | Time: 00:02.80 (Saving: 00:00.69)
> ```

Killed the instant the node's own second file appeared, the folder held `nodeImage0002.exr` at **13,013 bytes**, which did not open, and **no Output-panel file at all for frame 2**; `nodeImage0001.exr` was whole at 268,501,717 bytes and opened. Exit code -9. In 5.0 the same stop left the node's second file at 721 bytes and in 5.1 at 0 bytes, neither opening, and neither had an Output-panel file for that frame. That repeats the shape of the result recorded above.

**The kernel reset, run a second time, left the folder present and empty in every version — entry 25.** On the rented Linux machine only, and run last. A plain `reboot` was tried first and the machine came back in about a minute and a half; a bare kernel reset with nothing running was also issued once and the machine came back from that, so the step itself works. Then, once per version, the three-frame OpenEXR render at 4096 by 4096 was started with no window and the machine was reset at the kernel — `echo b > /proc/sysrq-trigger` — the instant frame 2's file appeared. After the reset, in Blender 5.2:

> ```
> LIST /root/frames-cross-sweep-2/e25/5.2/out
>    <empty>
> ```

The run's own log file did not exist at all, and the blend-file the run had been given survived whole at 96,018 bytes. Frames 1 and 2 were both absent, and frame 1 had been written and reported saved before the reset. The folder was present and empty in 4.5, 5.0 and 5.1 as well. **That disagrees with the run recorded above**, which had `frame_0001.exr` present at 0 bytes in 5.2. Both runs stand and neither has been chosen. What came out the same way twice: **no frame's contents survived in any version, including a frame already reported saved.**

**Not covered.** Everything the question actually asks. Whether a part-written file survives a stop; how large it is; whether Blender removes it; whether a stopped-mid-write file is readable, or distinguishable from a complete one by size, by extension, or by anything else; whether the behaviour differs between image formats (PNG, OpenEXR, JPEG, multi-layer EXR); whether it differs between a deliberate cancel, a process being killed, and the machine losing power. None of this is in the 5.2 sources.

**Where I looked.** Manual and API searched whole, case-insensitively, for: partial, incomplete, truncat, corrupt, unfinished, half-written, atomic, interrupted, interrupt, abort, cancel, zero-byte, empty file, "0 bytes", flush, "disk write", "written to disk", "partially written", "delete the file", "remove the file", touch, terminate, kill, signal, SIGINT, Ctrl-C, crash, power. Pages read in full: render output introduction, rendering animations, the Output panel properties page, the status bar page, the command line arguments page. Release notes for 5.2 searched for: overwrite, placeholder, output path, file output, writing, "render output", "save output" — the Render Output section holds two entries, "Saving render output can now be disabled." and "Light/shadow linking can be easily copied to the selection" (https://developer.blender.org/docs/release_notes/5.2/rendering/, read 2026-07-30); the first is about switching automatic saving off, not about interruption, and the second is about something else entirely. *(Corrected by the citation check of 2026-08-11: the sentence had named the first entry as the only one in that section.)* The Compositor File Output page was checked for the same silence. The supplied Cycles files were searched for overwrite, placeholder, `.tmp`, "temporary file", cancel, interrupt, and for any image-output code (`OutputDriver`, `write_render_tile`, `ImageOutput`): nothing writes rendered frames to disk there — what is supplied is the add-on's Python plus kernel headers (`source/kernel`, `source/util` only), so Cycles cannot speak to this at all.

**Outside the interface.** **Brought up to date by the hand tests of 2026-08-11:** every result under *What running it settled* was got from a render driven from outside Blender's window, with no window at all, so the answer *holds* outside the interface — that is where it was measured. The one thing tried inside the window is the Escape key, on the rented Linux machine, and it did not stop the render; what a person pressing Esc or clicking the status bar's cancel button would get is the only part of this answer not established from outside. **Brought up to date again by the second batch of hand tests, 2026-08-12:** the Escape test was run again with a control and this time the render **stopped**, in all four versions — entry 22 above. The two runs disagree and neither has been chosen. Everything else here is still established from outside the window, and the hand half stays open either way, because no run so far can show that Blender received the key. What follows describes the sources and still stands.

The sources do not say. No 5.2 page describes what an interrupted render leaves behind at all, inside the interface or outside it, so there is no basis for saying whether the two differ. Worth recording as documented silence: the 5.2 command line arguments page (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30) lists the render options and never mentions stopping a render, keyboard interrupts, or signals; the words Ctrl-C, SIGINT and interrupt do not appear anywhere in the 5.2 manual in a render sense. The one cancelling mechanism the manual does describe is an interface one — "The task can be aborted by clicking the cancel button ()." (manual, https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html, read 2026-07-30).

**Operating system.** **Brought up to date by the hand tests of 2026-08-11:** everything above was run on macOS 26.5.2 on an Apple M1 Pro **and** on Ubuntu 22.04.5 LTS on an AMD Threadripper, and the two agreed on every point of behaviour. Where they differ is in numbers that are races — the exact bytes a stop leaves, and whether a part-written TIFF happens to reach a size that opens. **Windows is untested**, no Windows machine having run it, so this answer covers macOS and Linux and leaves Windows open. **Brought up to date again by the second batch of hand tests, 2026-08-12:** entry 23 ran the JPEG and TIFF stop ten times per version on each machine, and both machines gave a spread of sizes and a mixture of files that opened and files that did not — the race is now shown directly rather than reasoned about, and the one-machine-opened-one-did-not line in the format table above is a draw from it. Entry 24 gave the same seven answers about the compositor set-up in 5.0, 5.1 and 5.2 on the Creator's Mac. Entries 22 and 25 were run on the rented Linux machine only, so what an Escape does inside a window and what a machine going down leaves are answered for Linux and left open for macOS. What follows describes the sources and still stands.

The sources do not say. Nothing in the 5.2 pages ties image-file writing, or its interruption, to any operating system. The only operating-system distinction found anywhere near file writing is for temporary files in general — `TEMP` on MS-Windows, `TMPDIR` on UNIX systems (manual, command line arguments page, read 2026-07-30) — and nothing connects those to writing rendered frames.

---

## 14. When two renders write into one folder and are started at the same instant, what does each of them produce, and what does the folder hold at the end?

**Reading taken.** I answer it as: two separate renders of the same frame range, both configured to write to the same output folder with the same file naming, so both are aiming at the same file names. The other reading is two renders writing different file names into one shared folder; on that reading the sources describe no interaction between them at all, and say nothing about it either.

**LABEL: not in the sources** — for the question as asked. Some of the ground around it is documented, and each of those items carries its own label below.

**What is documented: the manual describes this arrangement, and names two settings to change for it.** The manual gives it as a hint on the Output panel page:

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

Source: manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

That is the whole of it. It states the recipe and the result in the loosest terms ("share the rendering workload"). It does not say what each machine produces, how the work divides, what happens when two machines reach for the same frame, or what the folder holds when they finish.

**The two settings it names, quoted exactly.** From the manual, same page and date — note the two option headings on this page are garbled in the text (both read "Image Sequence – Overwrite Image Multi-Layer EXR"), so the descriptions are what carry the meaning:

> "Overwrite existing files when rendering."

> "Create empty placeholder frames while rendering."

From the API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30:

> "use_overwrite
>
> Overwrite existing files while rendering (default True)"

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

**The panel image shows the two options and how the boxes stood in the screenshot.** LABEL for this item: **documented** (a picture in the manual). The Output panel screenshot on the Output panel properties page, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — `[image: _images/render_output_properties_output_panel.png]`, viewed 2026-07-30 — shows a checkbox labelled **Overwrite**, ticked, and beside it a checkbox labelled **Placeholders**, unticked. What the picture carries is how those two boxes stood in that screenshot. That those are the defaults comes from the API quotes above (default True, default False), not from the picture.

LABEL: **inferred.** Each of the two settings the render-farm hint asks you to change is the opposite of its documented default, so a Blender with nothing changed is not in the state the hint describes. The step a reader has to supply is that the two boxes named on the panel are the two properties the API names.

**Where the sources run out.** LABEL for the question itself: **not in the sources.** Nothing in the 5.2 manual, API documentation, release notes or supplied Cycles files says:

- how two renders started at the same instant divide the frames between them;
- what happens when both reach for the same frame at the same moment — there is no mention anywhere of locking, of a file being reserved, of atomic creation, or of one render detecting another;
- what each render produces individually;
- what the folder contains once both have stopped.

Searches for the concepts that would carry such a statement — file lock, locking, race condition, concurrent, simultaneous, "same time", "multiple instances", "another instance", "in parallel", "two machines", "same frame", "same file", clash, collide, "network file system", "shared directory", "render farm" — turned up only the render-farm hint quoted above and unrelated matters (Cycles sample-splitting across machines, a metadata note about render farms, hair curve settings). There is no 5.2 page about network or distributed rendering.

**One inference, marked as such.** LABEL: **inferred.** With settings left at their defaults — Overwrite on, Placeholders off — two renders of the same frame range with the same output path are both writing to the same file names, because the file name for a frame is derived from the output path and the frame number:

> "When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png)."

Source: manual, Output panel page, read 2026-07-30. And the API exposes exactly that derivation:

> "frame_path(*, frame=-2147483648, preview=False, view='')
>
> Return the absolute path to the filename to be written for a given frame"

Source: API, `bpy.types.RenderSettings.frame_path`, read 2026-07-30. The steps a reader has to supply: that two renders with identical output settings therefore compute identical file names; that "Overwrite existing files while rendering" being on means neither one steps aside for a file the other has already written; and so each render renders the whole range and writes over the other's work. What the sources do **not** support, and I do not claim, is what any one file ends up containing when both wrote it, or in what order.

A second inference, marked as such and weaker. LABEL: **inferred.** For the render-farm recipe to divide work at all, the empty placeholder file for a frame must be created *before* that frame is rendered, so that the other machines — with Overwrite off — see a file already present and move on. The sources say the placeholders are created "while rendering frames" and never state the ordering. The step supplied is the whole mechanism. Treat this as the only reading under which the manual's hint makes sense, not as something the manual says.

### What running it settled — the hand tests of 2026-08-11

**LABEL: observed.** The machines, the settings folders and the list are described under question 13 above. Everything quoted is Blender 5.2.0's own output; the two machines are never merged.

**Overwrite off skips a frame before rendering it, not after.** A five-frame render at 512 by 512 with 400 samples, Overwrite off, with a complete PNG placed by hand at frame 3's name beforehand. Each line carries the seconds elapsed since the run started:

> ```
>    5.126  00:04.809  render           | Saved: '…/frame_0001.png'
>    9.404  00:09.088  render           | Saved: '…/frame_0002.png'
>    9.405  00:09.088  render           | Skipping existing frame "…/frame_0003.png"
>   13.763  00:13.446  render           | Saved: '…/frame_0004.png'
> ```

Frame 3 took **one thousandth of a second**, where its neighbours took more than four seconds each. The file placed by hand kept its own size and last-changed time. — Creator's Mac; the rented Linux machine gave the same shape at 6.791 seconds. **The two machines agreed.** 5.0, 5.1 and 5.2 print `Skipping existing frame` with a capital S; 4.5 prints `skipping existing frame`.

**"Already there" means a name exists, and nothing more.** Five things were put at frames 1 to 5's names before a six-frame render with Overwrite off: a complete PNG, a file of zero bytes, nineteen bytes of text that is not an image, a complete PNG of a different size, and a **folder**. All five were skipped and none touched:

> ```
> 00:00.402  render           | Skipping existing frame "…/frame_0001.png"
> 00:00.402  render           | Skipping existing frame "…/frame_0002.png"
> 00:00.402  render           | Skipping existing frame "…/frame_0003.png"
> 00:00.402  render           | Skipping existing frame "…/frame_0004.png"
> 00:00.402  render           | Skipping existing frame "…/frame_0005.png"
> 00:00.523  render           | Saved: '…/frame_0006.png'
> ```

Every one kept its original size and last-changed time. — Both machines, agreeing line for line. (On the rented Linux machine that run also printed `xcb_connection_has_error() returned true` twice before the render began, in 5.2 alone; it made no difference to what was written and is recorded because it was printed.)

**Two renders started together on the shipped defaults both render every frame.** Frames 1 to 8 at 400 by 400 and 200 samples, Overwrite on and Placeholders off, both started by one command. Each printed `Saved:` for all eight; the folder ended with eight files, all opening as 400 by 400 images. — Both machines, agreeing.

**Two renders started together on the manual's recipe divide the frames between them.** With Overwrite off and Placeholders on, one run on the rented Linux machine reported saving 2, 4, 6 and 7 and skipping 1, 3, 5 and 8, and the other reported the exact complement. Eight files, each written once, all readable. The Creator's Mac divided them the same way. — **Both machines agreed.**

**The placeholder is not a lock, and in 5.2 on the Creator's Mac it was raced on a plain local disk.** The recipe was run over a single frame, both Blenders started by one command, into a fresh empty folder, **thirty times per version per machine**. Counting how often both runs reported saving frame 1:

- Creator's Mac, local disk: 0 of 30 in 4.5, 0 of 30 in 5.0, 0 of 30 in 5.1, **2 of 30 in 5.2**.
- Rented Linux machine, local disk: **0 of 30 in 5.2**, and 0 of 30 in the other three.
- Rented Linux machine, a folder mounted over NFS from itself: 5 of 30 in 4.5, 6 of 30 in 5.0, 4 of 30 in 5.1, **4 of 30 in 5.2**.

In all 240 runs the resulting file was present and larger than zero bytes. Two collisions on one machine and none on the other is **not** read as a version difference — it is a race, and the same race showed in 5.0 in the eight-frame run on the rented Linux machine, where both copies saved frame 1. What the counts do show is that the claim on a frame **can** be lost, and that it is lost far more often over a network file system than on a local disk — which is the arrangement the manual's own hint recommends. — 2026-08-11.

**Two renders over overlapping but different frame ranges.** One run over frames 1 to 10 and one over 6 to 15, started together. On the defaults both wrote the whole of their own range, so frames 6 to 10 were written twice and fifteen files were left. On the recipe one run saved 1 to 5 and skipped 6 to 10 while the other saved 6 to 15 — fifteen files, each written once. — Both machines, agreeing.

**A placeholder left behind by a killed render blocks its frame for good.** A five-frame render on the recipe was killed outright the instant frame 2's file appeared, leaving frame 1 whole and `frame_0002.png` at **0 bytes**. Started again and run to the end, it printed `Skipping existing frame "…/frame_0002.png"` and went on to save 3, 4 and 5; frame 2 stayed at 0 bytes and would not open. With two renders running and one killed part-way, the survivor finished the rest and left the killed one's claimed frame at 0 bytes in the same way. — Both machines, agreeing. So a stopped render leaves a hole no later render fills.

**With a video output the two settings do nothing.** Both renders wrote to one file that Blender named `vid0001-0008.mp4`, both printed `Video append frame 1` through `Video append frame 8`, and the folder ended with a single file that Blender's own movie reader read back as 8 frames — the same on the defaults and on the recipe. — Both machines, agreeing. *(Corrected by the citation check of 2026-08-11: the words quoted here were `Append frame 1` through `Append frame 8`, which is what Blender 4.5 prints. Blender 5.2 prints `Video append frame 1` and so on, behind the usual logging prefix. Checked in all four versions.)*

**Nothing in the output path identifies the run.** Setting the output path to `{name}` for twenty-one names and asking `frame_path` what came back: in Blender 5.2 nine were substituted — `blend_name`, `blend_dir`, `blend_name_lib`, `blend_dir_lib`, `fps`, `resolution_x`, `resolution_y`, `scene_name` and `camera_name` — and every other name, including `node_name`, `pid`, `process_id`, `hostname`, `machine`, `user`, `run`, `instance`, `uuid`, `random`, `time` and `date`, came back as `Error: Parse errors in path`. Two runs of the same file returned identical paths for every name. **Blender 4.5 substitutes only four of those nine.** None of them varies between runs, so there is no built-in way to keep two runs' file names apart. — Both machines, agreeing.

**The two settings take when they are set from outside Blender's window.** A blend-file saved with Overwrite on and Placeholders off was rendered twice: once as saved, and once with `use_overwrite` set to False and `use_placeholder` to True through Blender's Python before the render, which printed `SET use_overwrite: False | use_placeholder: True`. Run again over their own folders, the first wrote over all four frames while the second reported `Info: No frames rendered, skipped to not overwrite` and `Skipping existing frame` for every one. — Both machines, all four versions, agreeing.

**Not covered.** The question's core: what each render produces and what the folder holds at the end. Also not covered: whether the answer differs between two renders on one machine and two machines over a network file system; whether a tie on the same frame is resolved at all; whether a file written by two renders at once can be damaged; whether the placeholder mechanism has any guarantee behind it or is merely a convention that usually works; whether output that is a single video file, rather than an image sequence, behaves differently when two renders share a folder — Overwrite and Placeholders are shown on the Output panel for Image and Multi-Layer EXR media types, and the sources do not say what they do for video.

**Where I looked.** The whole of the 5.2 manual and API text, searched case-insensitively for the terms listed above plus overwrite, placeholder, touch, skip, "network render", "frame server", farm. Pages read in full: the Output panel properties page, rendering animations, render output introduction, the command line arguments page. The Output panel image opened and read. Release notes for 5.2 searched for overwrite, placeholder, output, writing — nothing on concurrent renders. Supplied Cycles files searched for overwrite and placeholder — the only matches are unrelated kernel functions (`film_overwrite_pass_*`) and a comment in the denoise operator about temporarily changing the scene's output path; no image-writing code is present in what was supplied.

**Outside the interface.** **Brought up to date by the hand tests of 2026-08-11:** the two settings do take when a render is driven from outside Blender's window, and every result under *What running it settled* was measured that way, with no window at all — see the last item there. So this answer holds outside the interface. What follows describes the sources and still stands.

The sources do not say. The render-farm hint is written in interface terms ("in the Render Output panel") and never mentions the command line, background mode, or scripting. Documented silence worth recording: the 5.2 command line arguments page lists, under Render Options, `-b`, `-a`, `-S`, `-f`, `-s`, `-e`, `-j`, `-o`, `-E`, `-t`, `--cycles-device` and `--cycles-print-stats`, and under Format Options `-F` and `-x`, and has no flag for Overwrite or Placeholders *(corrected by the citation check of 2026-08-11: the list had put `-F` and `-x` among the render options and had left out the two Cycles options; the only hit for "overwrite" anywhere on that page is the sentence about argument order, and "placeholder" does not appear at all, so the point the list is making holds)*, so the command line offers no documented way to set them; both are properties of `bpy.types.RenderSettings` (API page, read 2026-07-30), which is a documented way to set them from Python. Whether the behaviour of two simultaneous renders is the same when they are driven from outside is not addressed anywhere, because the behaviour is not addressed anywhere.

**Operating system.** **Brought up to date by the hand tests of 2026-08-11:** every test above was run on macOS 26.5.2 on an Apple M1 Pro **and** on Ubuntu 22.04.5 LTS on an AMD Threadripper. The two machines agreed on everything except how often two renders collide on one frame — two collisions in thirty repeats on the Mac in this version, none on the rented machine — which is a race and is not read as a difference between operating systems. **Windows is untested**, no Windows machine having run it. What follows describes the sources and still stands.

The sources do not say. The render-farm hint says "over a network file system" without naming any operating system or any file system, and nothing else in the 5.2 sources distinguishes operating systems on this point. The one operating-system-flavoured phrase in the relevant text is a comparison, not a restriction: the placeholder option is described as "similar to Unix 'touch'" in the API documentation for all platforms.

---

## Things a person with Blender open could settle

**Pruned on 2026-08-11, and pruned again on 2026-08-12** after the second batch of hand tests. All eight items below were carried out, in all four researched versions, on the Creator's Mac and on a rented Linux machine; what came back is written into questions 13 and 14 above as observed findings, and the list they were run from is `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`. What is left open is what a running Blender on either of those two machines could not reach:

1. **Two renders on two separate computers writing into one shared folder** — item 8 below, and the arrangement the manual's own render-farm hint describes. The project's two machines have no file system shared between them, and sharing one would mean installing software on the Creator's working machine. A network file system *was* tested, with both renders on the rented Linux machine writing to a folder mounted over NFS from itself, and it changed the answer — see question 14 — so the two-computer case is worth taking up whenever a shared folder exists.
2. **Anything on Windows.** No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
3. **A real power cut, as against a kernel reset.** The reset was done twice, on 2026-08-11 and again on 2026-08-12, and the two runs do not agree about which directory entries survive. Both results are under question 13. Pulling a plug is still not reachable on either machine.
4. **Whether Blender receives an Escape key sent by a tool, and so what a person pressing Esc by hand would get.** Whether the render *stops* has been run three times on the rented Linux machine and has come out both ways; all three runs are under question 13 and none has been chosen. No run of this shape can show whether the key reached Blender at all. Nothing in this round is held back for a person, so this is recorded as unreached rather than passed on.
5. **What a person clicking the status bar's cancel button leaves behind.** That control needs a hand on a mouse.

The eight items are kept below as written, since each names the exact set-up that was used.

These are for question 13 and 14 only. Each says what to do, what to look at, and what a negative result would look like.

**For question 13**

1. **Watch a file grow, then stop mid-write.** Open Blender 5.2. In Properties → Output, set the file path to an empty folder, set File Format to OpenEXR with no compression and set the render resolution high (e.g. 4096×4096) so that saving takes a visible amount of time. Set the frame range to 1–3. Start an animation render (Ctrl-F12). With a file manager or a terminal open on that folder, watch the sizes of the files appearing. Press Esc during the moment the status bar shows the saving step for frame 2. Then look at the folder: note whether a file for frame 2 exists, its exact size in bytes, and whether an image viewer can open it. Repeat three times — the timing is what makes this worth repeating. What a negative result looks like: no file for frame 2 in the folder in any of the three repeats, so nothing part-written is left behind at all.

2. **Same again, but kill the process instead of cancelling.** Run the same render from a terminal with `blender -b <file> -a`. During the saving of a frame, end the process outright (close the terminal window, or end the task in the operating system's process manager). Look at the folder again and record whether a file for that frame exists and its size. This separates "Blender was asked to stop" from "Blender was stopped". What a negative result looks like: exactly what step 1 left, so the two ways of stopping cannot be told apart by what is on disk.

3. **Same again with Placeholders on.** Tick Placeholders in the Output panel and repeat step 1. Note, at the exact moment a frame starts rendering, whether a zero-byte file for that frame appears immediately; and after cancelling mid-save, whether what is left is zero bytes, part of an image, or a whole image. What a negative result looks like: no file appears when the frame starts, and what is left after cancelling is the same as it was with Placeholders off.

4. **Compare formats.** Repeat step 1 once each with PNG, JPEG, OpenEXR and Multi-Layer OpenEXR. Record for each whether an interrupted file is left and whether it opens. What a negative result looks like: all four formats behave alike — either every one leaves a file or none does — so the format makes no difference.

**For question 14**

5. **Two renders, one folder, defaults untouched.** Take one blend-file with a frame range of, say, 1–20 and each frame slow enough to see (a few seconds). Leave Overwrite ticked and Placeholders unticked. Start it from two terminals at once: `blender -b <file> -o //out/ -a` in both, launched within the same second. Let both finish. Record: how many frames each terminal reports rendering, how many files are in the folder, the file sizes, and whether any file is unreadable. What a negative result looks like: both terminals report every frame, the folder holds twenty readable files of ordinary size, and nothing shows the two renders met at all.

6. **Two renders, one folder, the render-farm settings.** Same blend-file, but untick Overwrite and tick Placeholders first. Start two renders at once as above. Record which frames each one reports rendering, whether the two lists overlap at all, whether any frame is left as a zero-byte file at the end, and how many frames the folder holds when both have finished. What a negative result looks like: both renders report the whole range, so the two settings divided no work between them.

7. **Force the tie.** Same as step 6, but start both processes from a single command so they begin as close to the same instant as possible, with the frame range set to a single frame. Repeat ten times, into a fresh empty folder each time. Record how often the resulting single file is a valid image and how often it is zero bytes or unreadable. This is the case the documentation does not cover, and it is the one that matters. What a negative result looks like: a valid image in all ten repeats, so a tie on one frame never shows in what is left.

8. **Same test across a network folder.** Repeat step 6 with the output folder on a network share rather than a local disk, with the two renders on two different machines. Record the same things. A difference between this and step 6 would be the strongest finding available here. What a negative result looks like: the same frame lists, the same file count and the same file sizes as step 6, so a network folder behaves no differently from a local disk.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Frames, Blender 5.2

2026-08-12 — Claude Opus 5 (Claude Code) — the second batch of hand tests written in: entries 22, 23, 24 and 25 run and recorded under question 13, both standing lines brought up to date, and the researcher's own list pruned again.
