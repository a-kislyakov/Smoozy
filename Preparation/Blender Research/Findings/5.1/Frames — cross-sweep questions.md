# Frames — cross-sweep questions (Blender 5.1)

Sources consulted: Blender 5.1 manual, Blender 5.1 Python API documentation, Blender 5.1 release notes, and Cycles' own files for 5.1 — all as supplied in `Blender Sources/`, snapshot taken 2026-07-30, which is the date read for everything below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered: 13 and 14 only.

---

## 13. When a render is stopped at the exact moment it is writing a frame's own image file, what is left where it writes?

**LABEL: not in the sources.**

The 5.1 manual, the 5.1 Python API documentation and the 5.1 release notes all say nothing about what is on disk if a render is cut off part-way through writing a frame's image file. Nothing states whether Blender writes the pixels straight into the final file or into a temporary file that is renamed at the end; nothing states whether a half-written file is left behind, whether it is removed, or whether it is left at zero length. The word for a broken or half-finished file does not appear in this context anywhere in those three. The Cycles files supplied were listed rather than searched, and what that does and does not settle is set out further down.

**The question was ambiguous, and I answer both readings the same way.** "Stopped" can mean the person cancels the render job from inside Blender, or the program is killed outright by the machine or by another program. The sources do not describe the on-disk result of either, so both readings get the same answer. The other reading would not change anything below.

What the sources *do* say nearby, none of which settles the question:

**Frames are written out after the frame is rendered, one file at a time.**

> "animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

Source: manual. https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

That places the write after the render of that frame, but says nothing about how the write itself is done or what an interrupted write leaves.

**There is a setting that creates an empty file at the frame's path while rendering.** In the API documentation, on `bpy.types.RenderSettings`:

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

Source: Python API documentation. https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

The manual's wording for the same setting:

> "Create empty placeholder frames while rendering."

Source: manual. https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

A caution about that manual quote: on that page the heading above this line reads "Image Sequence – Overwrite Image Multi-Layer EXR", the same heading as the Overwrite setting directly above it. That looks like a fault in the page.

What the picture on the same page shows is two checkboxes side by side, labelled "Overwrite" (ticked) and "Placeholders" (unticked). That much is **documented**, a picture in the manual being part of the manual. Picture: `_images/render_output_properties_output_panel.png`, on https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

Taking the quoted line as the description of the Placeholders checkbox rather than the Overwrite one is a step the reader supplies, from the wording matching the label. That step is **inferred**, not documented.

Neither wording says at what point in the frame's life the empty file appears, nor what the file contains between being created and being filled in, nor what is left if the render stops in between. Reading a stopped-mid-write state out of it would be my step, not the source's.

**The only statement in the sources about a render stopping and leaving something broken is about video, not about a frame's own image file:**

> "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Source: manual. https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

That sentence is about rendering to a video format. It is not about a single frame's image file, so it does not answer this question.

**What was done with Cycles' own files, and what it settles.** The files supplied in `Blender Sources/cycles_5.1` were listed by name: the add-on's Python files (`__init__.py`, `engine.py`, `properties.py`, `ui.py`, `operators.py`, `camera.py`, `osl.py`, `presets.py`, `version_update.py`) and the renderer's kernel and shader files under `source/` and `shader/`. No file among them is named for writing output images.

That is a list of names, not a reading of what is inside them. It does not establish that no line in those files bears on what an interrupted write leaves on disk. Their contents were not searched, so this source is unfinished here rather than silent.

### What running it settled — the hand tests of 2026-08-11

**LABEL: observed.** Twenty-one tests, built from all eight of this round's findings documents and the four audits and saved at `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, were run in Blender 4.5.12, 5.0.1, 5.1.2 and 5.2.0 on two machines. **Machine one, the Creator's Mac:** a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card. **Machine two, a rented Linux machine:** Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Every run had a settings folder of its own and was started with `--factory-startup`. Everything quoted is Blender 5.1.2's own output; the two machines are never merged.

**Blender writes the frame straight to its final name, and the file grows in place.** Three frames at 4096 by 4096, OpenEXR at 32 bits with no compression, the whole folder listed every two milliseconds. Every name that ever appeared was a final frame name — no `.tmp`, no `.part`, no dot-prefixed or hidden name:

> ```
> WATCH    0.000  <empty>
> WATCH    1.395  frame_0001.exr=86934
> WATCH    1.398  frame_0001.exr=2974614
> WATCH    1.402  frame_0001.exr=4596630
> ```

and 113 distinct sizes later it reached 268,501,910 bytes. — Creator's Mac. On the rented Linux machine it began `WATCH 1.717 frame_0001.exr=77824` and ended at 268,501,871 bytes, again with only the three final names ever seen. **The two machines agreed.** The write is announced as a step of its own: `00:01.478  render           | Saved: '…/frame_0001.exr'` followed by `Time: 00:01.16 (Saving: 00:00.33)`. Blender 4.5 prints those two lines bare, with no elapsed time and no `render` before them.

**How the stop is delivered decides what is left.** Three runs, each stopped at the instant frame 2's file first appeared:

- **An interrupt (`SIGINT`)** did **not** cut the write. Blender finished writing the frame and then stopped: `Saved: '…/frame_0002.exr'`, `Time: 00:01.01 (Saving: 00:00.33)`, then `Sent an internal break event. Press ^C again to kill Blender`, exit code 1. Frame 2 was 268,501,914 bytes and opened as a 4096 by 4096 image.
- **A terminate (`SIGTERM`)** left a file at frame 2's path of **0 bytes**, exit code -15, no `Saved:` line, unopenable: `OPEN loaded | size: (0, 0) | channels: 0 | depth: 0 | has_data: False`.
- **A kill (`SIGKILL`)** left **923 bytes**, exit code -9, again unopenable.

— Creator's Mac. On the rented Linux machine: the interrupt again finished the write and left a whole 268,501,875-byte frame; the terminate left **32,772 bytes**; the kill left **0 bytes**. **The two machines agreed on the pattern**; the exact leftover size is a race and differs run to run.

**The format changes what the leftover looks like, and one format is dangerous.** The same kill, once per format, at 4096 by 4096. PNG left 0 bytes on both machines; JPEG 0 bytes on both; OpenEXR 5,023 bytes on the Mac and 32,768 on Linux; TIFF **8 bytes on the Mac, which did not open, and 1,306 bytes on Linux, which did** — `OPEN loaded | size: (4096, 4096) | channels: 4 | depth: 32 | has_data: True` — against a complete TIFF of 5,399,372 bytes. So **a stopped-mid-write file is not always distinguishable from a finished one by asking whether it opens**; its size still tells them apart.

**Multi-layer OpenEXR needs a different setting in 5.1 than in 4.5.** Setting `file_format` to `OPEN_EXR_MULTILAYER` while Media Type is Image is refused — `TypeError: bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in (…)` — and with `media_type` set to `MULTI_LAYER_IMAGE` first it is accepted. One file per frame holds both view layers, 537,036,580 bytes complete and 59,172 bytes after a kill. Blender's own `bpy.data.images.load` reports `type: MULTILAYER | size: (0, 0) | has_data: False` for the **complete** file as well as the part-written one, so for that one format the reader used here cannot tell whole from part — a limit of the test, not a finding about the file.

**The operating system had already put everything on disk.** After a kill the folder was listed at once and again sixty seconds later; both listings were identical, on both machines and in all four versions.

**With Placeholders on, a file of zero bytes is put at the frame's path before that frame is rendered — one frame at a time.**

> ```
> WATCH    0.000  <empty>
> WATCH    0.355  frame_0001.exr=0
> WATCH    1.644  frame_0001.exr=8192
> ```

Frame 1's file appeared at **0 bytes** more than a second before any pixels reached it, and frame 2's file did not appear until frame 1 was finished — 2.137 seconds in on the rented Linux machine, 1.613 on the Creator's Mac. No file ever appeared for a frame the render had not reached. That settles the step this document marks as inferred. Stopping the run once frame 2's file had grown past zero left 33,687 bytes on the Creator's Mac and a short unopenable file on the rented Linux machine. **The two machines agreed.**

**The JPEG preview copy exists and is written after the frame.** With `use_preview` on and OpenEXR output, a `.jpg` sat beside every `.exr`, saved immediately after it — `Saved: '…/frame_0001.exr'` at 00:00.567 and `Saved: '…/frame_0001.jpg'` at 00:00.647. — Both machines, agreeing.

**A Compositor File Output node writes before the render's own frame file, and is set up differently from 4.5.** In 5.1 the compositor is a node group on `scene.compositing_node_group` and the node takes a `directory` and a `file_name`, so its files are called `nodeImage0002.exr`, where 4.5 uses `scene.node_tree`, a `base_path`, and `Image0002.exr`. Stopped the instant the node's own second file appeared, the folder held it at 33,498 bytes on the Creator's Mac and 16,387 on the rented Linux machine — unopenable in both — and **no Output-panel file at all for that frame**. *(The citation check of 2026-08-11 could not repeat this. It confirmed from Blender itself that the scene carries a `compositing_node_group` and that the node's properties are `directory` and `file_name` rather than `base_path`, but what is written here does not say how the node is wired into the new compositor, and the node wrote no file at all when it was set up from this description. The 4.5 result repeated exactly. Written out as item 24 on the hand-tests list.)*

**Cancelling from inside Blender's own window did not stop the render.** On the rented Linux machine only, a forty-frame render started from a timer in a real window, with the render window holding the keyboard (`the window that has the keyboard: Blender Render`), and Escape sent to every Blender window repeatedly for ten seconds with `xdotool`. Fifteen seconds after the first Escape the render was still going and went on writing whole frames. The same in all four versions. This shows the render did not stop; it cannot show Blender received the key, so it does not settle what an Escape pressed by a person would do.

**The citation check of 2026-08-11 repeated this and got the opposite, and both runs are recorded rather than one replacing the other.** Following the procedure written above on the rented Linux machine, the render **stopped**: sixteen frames stood in the folder when the first Escape was sent and sixteen stood there fifteen seconds after the last one, in Blender 4.5, run twice; in 5.2 it was fourteen and fourteen. A control identical in every respect except that no Escape was ever sent went from sixteen frames at twenty-five seconds to thirty-six at fifty seconds in 4.5, and from fourteen to thirty-three in 5.2 — so without the Escape the render kept going, and with it the render stopped. That is a question, not an answer. It is item 22 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, to be run again, and until it is run nothing downstream should treat either result as the fact.

**A machine going down loses even frames it had already reported saving.** On the rented Linux machine only, the same three-frame OpenEXR render was started and the machine was reset at the kernel — `echo b > /proc/sysrq-trigger`, with nothing flushed — the instant frame 2's file appeared. The machine restarted, and afterwards **the output folder was empty** and the run's own log file was 0 bytes. Frame 1 had been reported saved seconds earlier and was gone. A plain `reboot` had been tried first and the machine came back from it, so the loss is the reset's doing and not a failure of the step. The same reset in the other versions left the folder empty in 4.5, left no folder at all in 5.0, and in 5.2 left `frame_0001.exr` present at **0 bytes**; exactly which directory entries survive is what the file system happened to have written down, and no frame's contents survived in any version. This is the *unflushed write* case, and it comes out differently from every kill above. *(The citation check of 2026-08-11 did not repeat this: the kernel reset was refused by the permission system of the machine that check ran on. The result stands on its single run, and is item 25 on the hand-tests list.)*

**There is a setting in 5.2 that switches saving off, and it does not exist in 5.1.** Asked of Blender itself, `RenderSettings.save_output` is **absent** in 4.5, 5.0 and 5.1 and present in 5.2 as `type=BOOLEAN | default=True | description='Write frames to disk for animation renders'`. Both machines, agreeing. It is the only property in the render settings, in any of the four versions, whose name or description speaks of saving render output.

### What the second batch of hand tests settled — 2026-08-12

**LABEL: observed.** Four more tests — entries 22 to 25 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md` — were added by the citation check of 2026-08-11: three because a repeat came back differently from a run recorded above, and one because a procedure could not be followed as written. They were run on 2026-08-12 on the machines each entry names, on the same two machines described above, each with a settings folder of its own and `--factory-startup`. Everything quoted is Blender 5.1.2's own output, and the two machines are never merged. **Where a result differs from one written above, both stand and neither has been chosen.**

**Escape sent to Blender's own window stopped the render — entry 22.** On the rented Linux machine only, that being the one machine of the two with a screen this round could drive. A forty-frame render at 480 by 480 with eighty samples, PNG, into a fresh empty folder, started in a real window from a timer four seconds after launch. At twenty-five seconds the folder was counted and the machine was asked which window held the keyboard:

> ```
> COUNT at 25s, before any Escape: 37
> xdotool getwindowfocus getwindowname: Blender Render
> xdotool search --name Blender: 62914562 62914691
> ```

Escape was then sent to both of those windows and to the focused window, twenty-seven times over ten seconds, and the folder counted again:

> ```
> COUNT at 35s, right after the Escape window: 38
> COUNT at 50s, fifteen seconds after the last Escape: 38
> ```

So one more frame was finished after the first Escape, and then the count did not move again. A control in the same sitting, identical except that no Escape was ever sent, went `COUNT at 25s: 38`, then 40, then 40. The other three versions came out the same way: 4.5 stayed at 30, run twice; 5.0 stayed at 35; 5.2 went 32 to 33 and stopped. Every control reached all forty frames of the range, which is what ended its own climb; every test stopped short of forty and did not move again. **This agrees with the citation check's repeat and disagrees with the run recorded above under *Cancelling from inside Blender's own window*. Both stand.** It still cannot show that Blender received the key, so what a person pressing Esc with their own hand would get is not settled by it.

**A stop leaves a different number of bytes almost every time, for JPEG and for TIFF — entry 23.** Ten repeats per format per version per machine, each into a fresh empty folder: the three-frame render at 4096 by 4096 was started with no window and killed outright the instant frame 2's file first appeared, then that file's exact size was recorded and Blender's own image loader was asked to open it. In Blender 5.1 on the **Creator's Mac** the ten JPEG leftovers were **0 bytes every time**, and none of the ten opened, against a complete JPEG of 4,128,053 bytes. The ten TIFF leftovers were 8, 1,306, 1,306, 8, 1,306, 1,306, 1,286, 1,306, 8 and 0 bytes; five of the ten opened and reported 4096 by 4096, against a complete TIFF of 5,397,377 bytes. On the **rented Linux machine** the ten JPEG leftovers were 0, 0, 0, 0, 0, 4,096, 0, 0, 4,096 and 4,096 bytes, three opening, against a complete JPEG of 4,113,616 bytes; the ten TIFF leftovers were 238, 1,306, 1,306, 8, 1,306, 8, 1,306, 1,306, 8 and 8 bytes, five opening, against a complete TIFF of 5,399,372 bytes. **Every cell is a spread**, except this version's JPEG on the Mac, which came out at 0 bytes ten times out of ten. So the single numbers in the format table above — the 8 bytes on the Mac in particular — and the different single numbers the citation check got are both draws from a race rather than facts about a version or a machine. A file of 0 bytes never opened, in any of the 160 repeats.

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

Blender 5.0 and 5.2 gave exactly the same seven answers. So the set-up, written out: make a node group of kind `CompositorNodeTree` and put it on `scene.compositing_node_group`; put a Render Layers node in it, since nothing else feeds the File Output node — a group input does not; put a File Output node in it and add an item with `file_output_items.new("RGBA", "Image")`, because the node is born with no real input and that call is what makes one; link the Render Layers node's Image output to that input; switch `use_compositing` on; and set the node's `directory` and `file_name`. A group output socket is **not** needed. With that set-up, entry 7 ran as written. A complete three-frame run at 4096 by 4096, PNG from the Output panel and 32-bit uncompressed OpenEXR from the node, printed the node's save first every time:

> ```
> 00:01.474  render           | Saved: '…/node/nodeImage0001.exr'
> 00:02.091  render           | Saved: '…/out/frame_0001.png'
> 00:02.091  render           | Time: 00:01.76 (Saving: 00:00.61)
> ```

Killed the instant the node's own second file appeared, the folder held `nodeImage0002.exr` at **0 bytes**, which did not open, and **no Output-panel file at all for frame 2**; `nodeImage0001.exr` was whole at 268,501,713 bytes and opened. Exit code -9. In 5.0 the same stop left the node's second file at 721 bytes and in 5.2 at 13,013 bytes, neither opening, and neither had an Output-panel file for that frame. That repeats the shape of the result recorded above.

**The kernel reset, run a second time, left the folder present and empty in every version — entry 25.** On the rented Linux machine only, and run last. A plain `reboot` was tried first and the machine came back in about a minute and a half; a bare kernel reset with nothing running was also issued once and the machine came back from that, so the step itself works. Then, once per version, the three-frame OpenEXR render at 4096 by 4096 was started with no window and the machine was reset at the kernel — `echo b > /proc/sysrq-trigger` — the instant frame 2's file appeared. After the reset, in Blender 5.1:

> ```
> LIST /root/frames-cross-sweep-2/e25/5.1/out
>    <empty>
> ```

The run's own log file survived at 0 bytes, and the blend-file the run had been given survived whole at 95,504 bytes. Frames 1 and 2 were both absent, and frame 1 had been written and reported saved before the reset. The folder was present and empty in 4.5, 5.0 and 5.2 as well, and only 5.1's run log survived at all. **That agrees with the run recorded above for 5.1 and disagrees with it for 5.0 and 5.2**, where that run had no folder at all and a zero-byte `frame_0001.exr` respectively. Both runs stand and neither has been chosen. What came out the same way twice: **no frame's contents survived in any version, including a frame already reported saved.**

**Not covered.** This answer does not settle whether a partly written file is left behind, whether it is deleted, whether its size is zero or something in between, whether the answer differs by image format (PNG, OpenEXR, JPEG and so on), or whether the answer differs between a cancelled job and a killed program. It also does not settle whether the Placeholders setting changes any of this. All of that is simply absent from the sources. *(That paragraph describes what the **sources** do not settle, and it stands. Every item in it is answered above by a running Blender.)*

**Where I looked, so that "not in the sources" is a real answer.** I searched the whole 5.1 manual, the whole 5.1 API documentation and the whole 5.1 release notes for: partial, partially written, partial file, incomplete, incomplete file, truncated, truncated file, corrupt, corrupted file, corrupt image, damaged, unfinished, half-written, atomic, in-place, written directly, rename, .tmp, temp file, temporary file, flush, buffered, placeholder, unplayable. Within the render pages and the troubleshooting pages I also searched: abort, interrupt, cancel, Esc, stop the render, resume, crash, kill, SIGTERM, signal, power failure. The only hits that bore on the subject at all are the ones quoted above. I read in full: the Output panel page, the Rendering Animations page, the render Output introduction page, and the Render menu section of the Topbar page. I looked at the Output panel screenshot. I listed the Cycles 5.1 files and the kernel tree by name, and searched none of their contents. The release notes for 5.1 mention output files only for the compositor's File Output node and for an unrelated file-browser overwrite warning; neither concerns an interrupted write.

**Outside the interface.** **Brought up to date by the hand tests of 2026-08-11:** every result under *What running it settled* was got from a render driven from outside Blender's window, with no window at all, so the answer *holds* outside the interface — that is where it was measured. The one thing tried inside the window is the Escape key, on the rented Linux machine, and it did not stop the render; what a person pressing Esc or clicking the status bar's cancel button would get is the only part of this answer not established from outside. **Brought up to date again by the second batch of hand tests, 2026-08-12:** the Escape test was run again with a control and this time the render **stopped**, in all four versions — entry 22 above. The two runs disagree and neither has been chosen. Everything else here is still established from outside the window, and the hand half stays open either way, because no run so far can show that Blender received the key. What follows describes the sources and still stands.

The sources do not say. Nothing in the manual, the API documentation or the release notes describes on-disk results of an interrupted write at all, so there is nothing that separates a render driven by a person clicking from one started and controlled from outside Blender's window. One related thing is shown by what the command-line page contains: the list of command-line arguments for 5.1 includes no switch for Overwrite or Placeholders — the render options listed are `-b`, `-a`, `-S`, `-f`, `-s`, `-e`, `-j`, `-o`, `-E`, `-t`, the two Cycles options, and the format options `-F` and `-x`. That the list holds no such switch is **documented**, though listed rather than quoted, an absence having no words to quote. Going on from that to *those two settings can only come from the blend-file or from Python* is a step the reader supplies — the page does not say those are the only ways in — so that step is **inferred**. Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

**Operating system.** The sources do not say. Nothing here is stated per operating system. The only mention of any system is the phrase "similar to Unix 'touch'" in the API description of the placeholder setting, and that is an analogy explaining what an empty file is, not a statement that the behaviour differs elsewhere. **Brought up to date by the hand tests of 2026-08-11:** everything above was run on macOS 26.5.2 on an Apple M1 Pro **and** on Ubuntu 22.04.5 LTS on an AMD Threadripper, and the two agreed on every point of behaviour. Where they differ is in numbers that are races — the exact bytes a stop leaves, whether a part-written TIFF happens to open, and how often two renders collide on one frame — and none of those is read as an operating-system difference. **Windows is untested**, no Windows machine having run it, so this answer covers macOS and Linux and leaves Windows open. **Brought up to date again by the second batch of hand tests, 2026-08-12:** entry 23 ran the JPEG and TIFF stop ten times per version on each machine, and both machines gave a spread of sizes and a mixture of files that opened and files that did not — the race is now shown directly rather than reasoned about. One thing did divide the two machines in this version: on the Creator's Mac the JPEG leftover was 0 bytes in all ten repeats, where the rented Linux machine gave 4,096 bytes in three of ten. Entry 24 gave the same seven answers about the compositor set-up in 5.0, 5.1 and 5.2 on the Creator's Mac. Entries 22 and 25 were run on the rented Linux machine only, so what an Escape does inside a window and what a machine going down leaves are answered for Linux and left open for macOS.

---

## 14. When two renders write into one folder and are started at the same instant, what does each of them produce, and what does the folder hold at the end?

**LABEL: not in the sources**, for what the question actually asks. The manual does offer several machines rendering into one shared folder as a way of working, and names two settings to change for it, but the sources never describe what each render produces, how the two renders divide the frames between them, or what the folder holds when they are done — and nothing at all about two renders started at the same instant.

**What is documented.** The manual gives this as a hint on the Output panel page, under the heading "Primitive Render Farm":

> "An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Source: manual. https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

The two settings it names, quoted from the same page:

> "Overwrite existing files when rendering."

> "Create empty placeholder frames while rendering."

Source: manual. https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

(Both of those sit under a heading that reads "Image Sequence – Overwrite Image Multi-Layer EXR" on that page — the same heading twice, which looks like a fault in the page. The Output panel screenshot on the same page shows two checkboxes labelled "Overwrite" and "Placeholders"; that is what the picture shows, and it is **documented**. Picture: `_images/render_output_properties_output_panel.png`, on https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30. Matching each of the two quoted lines to one of those two checkboxes is a step the reader supplies, from the wording matching the label, and is **inferred**.)

The same two settings in the API documentation, with their kind and default:

> "use_overwrite
>
> Overwrite existing files while rendering (default True)
>
> Type:
>
> bool"

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)
>
> Type:
>
> bool"

Source: Python API documentation. https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

Note the defaults: out of the box Overwrite is on and Placeholders is off — the opposite of what the render-farm hint asks for on both counts.

Elsewhere the manual repeats that turning Overwrite off is how a render picks up where an earlier one left off:

> "Just disable the Overwrite option to start rendering where you left off."

Source: manual. https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

**What is not documented, and it is the substance of the question.** Reading the hint as "each machine renders a different frame, and the folder ends up with one file per frame and no duplicated work" requires me to supply the mechanism: that the empty placeholder file appears at a frame's path before that frame is rendered, that a render with Overwrite off decides to skip a frame by finding a file at that path, and that this happens quickly enough and in the right order to stop a second render from starting the same frame. **None of those three steps is stated anywhere in the sources.** The manual never says what test Overwrite performs, never says when in a frame's life the placeholder file is created, and never says the two settings interact at all — the hint simply lists them together. So the answer to "what does each of them produce, and what does the folder hold at the end" is: the sources do not say.

In particular the sources are silent on:

- What happens with the settings left at their defaults (Overwrite on, Placeholders off), which is what two renders started against the same folder would do unless changed.
- What happens when two renders reach the same frame at the same instant — whether one wins, whether both write, whether either notices the other. There is no mention of any lock, of any claim on a frame, or of any coordination between renders.
- Whether either render reports the frames it skipped, or how a render's own output differs from what it would have produced alone.
- What the folder holds if the two renders were given different frame ranges, the same range, or overlapping ranges.
- Anything about timing at all. The phrase "at the same instant" has no counterpart anywhere in the sources.

The one other place the manual discusses several machines rendering together is the Sample Subset feature, and it is a different arrangement — splitting the samples of a *single frame* across machines and merging the results afterwards with `bpy.ops.cycles.merge_images`, each machine writing its own file. It does not describe two renders writing into one folder over a frame range, so it does not answer this question either. That page is described here rather than quoted — no words from it are quoted, so what is said about it is a paraphrase. Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html — read 2026-07-30.

**What was done with Cycles' own files.** As set out under question 13, the supplied files were listed by name and their contents were not searched. No file among them is named for output writing, but a list of names cannot rule out a line inside those files bearing on two renders sharing a folder. This source is unfinished here too.

### What running it settled — the hand tests of 2026-08-11

**LABEL: observed.** The machines, the settings folders and the list are described under question 13 above. Everything quoted is Blender 5.1.2's own output; the two machines are never merged.

**Overwrite off skips a frame before rendering it, not after.** A five-frame render at 512 by 512 with 400 samples, Overwrite off, with a complete PNG placed by hand at frame 3's name beforehand. Each line carries the seconds elapsed since the run started:

> ```
>    4.041  00:03.806  render           | Saved: '…/frame_0001.png'
>    7.450  00:07.214  render           | Saved: '…/frame_0002.png'
>    7.450  00:07.214  render           | Skipping existing frame "…/frame_0003.png"
>   10.965  00:10.730  render           | Saved: '…/frame_0004.png'
> ```

Frame 3 took **no measurable time at all** — the skip is printed at the same thousandth of a second as frame 2's save — where its neighbours took three and a half seconds each. The file placed by hand kept its own size and last-changed time. — Creator's Mac; the rented Linux machine gave the same shape at 5.903 seconds. **The two machines agreed.** Note the wording: 5.0, 5.1 and 5.2 print `Skipping existing frame` with a capital S, where 4.5 prints `skipping existing frame`.

**"Already there" means a name exists, and nothing more.** Five things were put at frames 1 to 5's names before a six-frame render with Overwrite off: a complete PNG, a file of zero bytes, nineteen bytes of text that is not an image, a complete PNG of a different size, and a **folder**. All five were skipped and none touched:

> ```
> 00:00.320  render           | Skipping existing frame "…/frame_0001.png"
> 00:00.320  render           | Skipping existing frame "…/frame_0002.png"
> 00:00.320  render           | Skipping existing frame "…/frame_0003.png"
> 00:00.320  render           | Skipping existing frame "…/frame_0004.png"
> 00:00.321  render           | Skipping existing frame "…/frame_0005.png"
> 00:00.414  render           | Saved: '…/frame_0006.png'
> ```

Every one kept its original size and last-changed time. — Both machines, agreeing line for line. That answers what "already there" means to this setting: any name, whatever is behind it, including a folder.

**Two renders started together on the shipped defaults both render every frame.** Frames 1 to 8 at 400 by 400 and 200 samples, Overwrite on and Placeholders off, both started by one command. Each printed `Saved:` for all eight; the folder ended with eight files, all opening as 400 by 400 images. — Both machines, agreeing.

**Two renders started together on the manual's recipe divide the frames between them.** With Overwrite off and Placeholders on, one run on the rented Linux machine reported saving 2, 4, 5 and 8 and skipping 1, 3, 6 and 7, and the other reported the exact complement. Eight files, each written once, all readable. The Creator's Mac divided them the same way. — **Both machines agreed.**

**The placeholder is not a lock, and how often it is raced depends on the file system.** The recipe was run over a single frame, both Blenders started by one command, into a fresh empty folder, **thirty times per version per machine**. Counting how often both runs reported saving frame 1:

- Creator's Mac, local disk: 0 of 30 in 4.5, 0 of 30 in 5.0, **0 of 30 in 5.1**, 2 of 30 in 5.2.
- Rented Linux machine, local disk: **0 of 30 in 5.1**, and 0 of 30 in the other three.
- Rented Linux machine, a folder mounted over NFS from itself: 5 of 30 in 4.5, 6 of 30 in 5.0, **4 of 30 in 5.1**, 4 of 30 in 5.2.

In all 240 runs the resulting file was present and larger than zero bytes. What this shows is that the claim on a frame **can** be lost, and that it is lost far more often over a network file system than on a local disk — which is the arrangement the manual's own hint recommends. — 2026-08-11.

**Two renders over overlapping but different frame ranges.** One run over frames 1 to 10 and one over 6 to 15, started together. On the defaults both wrote the whole of their own range, so frames 6 to 10 were written twice and fifteen files were left. On the recipe one run saved 1 to 5 and skipped 6 to 10 while the other saved 6 to 15 — fifteen files, each written once. — Both machines, agreeing.

**A placeholder left behind by a killed render blocks its frame for good.** A five-frame render on the recipe was killed outright the instant frame 2's file appeared, leaving frame 1 whole and `frame_0002.png` at **0 bytes**. Started again and run to the end, it printed `Skipping existing frame "…/frame_0002.png"` and went on to save 3, 4 and 5; frame 2 stayed at 0 bytes and would not open. With two renders running and one killed part-way, the survivor finished the rest and left the killed one's claimed frame at 0 bytes in the same way. — Both machines, agreeing.

**With a video output the two settings do nothing.** Both renders wrote to one file that Blender named `vid0001-0008.mp4`, both printed `Video append frame 1` through `Video append frame 8`, and the folder ended with a single file that Blender's own movie reader read back as 8 frames — the same on the defaults and on the recipe. — Both machines, agreeing. *(Corrected by the citation check of 2026-08-11: the words quoted here were `Append frame 1` through `Append frame 8`, which is what Blender 4.5 prints. Blender 5.1 prints `Video append frame 1` and so on, behind the usual logging prefix. Checked in all four versions.)*

**Nothing in the output path identifies the run.** Setting the output path to `{name}` for twenty-one names and asking `frame_path` what came back: in Blender 5.1 nine were substituted — `blend_name`, `blend_dir`, `blend_name_lib`, `blend_dir_lib`, `fps`, `resolution_x`, `resolution_y`, `scene_name` and `camera_name` — and every other name, including `node_name`, `pid`, `process_id`, `hostname`, `machine`, `user`, `run`, `instance`, `uuid`, `random`, `time` and `date`, came back as `Error: Parse errors in path`. Two runs of the same file returned identical paths for every name. **Blender 4.5 substitutes only four of those nine** and errors on the other five. So the ten-name list this document quotes from the manual is not a list of ways to keep two runs apart: none of them varies between runs. — Both machines, agreeing.

**The two settings take when they are set from outside Blender's window.** A blend-file saved with Overwrite on and Placeholders off was rendered twice: once as saved, and once with `use_overwrite` set to False and `use_placeholder` to True through Blender's Python before the render, which printed `SET use_overwrite: False | use_placeholder: True`. Run again over their own folders, the first wrote over all four frames while the second reported `Info: No frames rendered, skipped to not overwrite` and `Skipping existing frame` for every one. — Both machines, all four versions, agreeing. That is the test this version's audit sent here as its test 9, and it confirms the inference the pass drew from the command-line page.

**Not covered.** This answer does not settle what actually happens on disk in this situation — only that the sources do not describe it. It does not settle whether the render-farm hint works reliably, whether it has known failure modes, what happens on a network file system as opposed to a local disk, what happens with a video output rather than an image sequence, or what happens when the two renders differ in their settings. *(That paragraph describes what the **sources** do not settle, and it stands. The hand tests above answer all of it but the last, from a running Blender.)*

**Where I looked, so that "not in the sources" is a real answer.** I searched the whole 5.1 manual, the whole 5.1 API documentation and the whole 5.1 release notes for: overwrite, placeholder, placeholders, use_overwrite, use_placeholder, empty placeholder, lock, file lock, lock file, concurrent, simultaneous, at the same time, same file at the same time, race condition, clash, conflict, multiple instances, second instance, two instances, another instance, multiple machines, several machines, render farm, shared directory, network file system, already exist, existing file, skip. I read in full: the Output panel page, the Rendering Animations page, the render Output introduction page, the command-line arguments page's render and format options, and the Sample Subset section of the Cycles Sampling page. I looked at the Output panel screenshot. The 5.1 release notes contain no entry about render output overwriting, placeholders, or several renders sharing a folder; their only output-file entries concern the compositor's File Output node reading and writing custom meta-data and allowing directories in item names, and a file-browser overwrite warning in the user-interface notes — none of which touches this.

**Outside the interface.** **Brought up to date by the hand tests of 2026-08-11:** the two settings do take when a render is driven from outside Blender's window, and every result under *What running it settled* was measured that way, with no window at all — see the last item there. So this answer holds outside the interface, and the inference the pass drew from the command-line page is confirmed. What follows describes the sources and still stands.

The sources do not say whether any of this differs when Blender is driven from outside its own window. The render-farm hint says "Start as many machines as you wish rendering to that directory" without saying how they are started, and it names the two settings by their positions in the Render Output panel — that is, by where a person clicks. One thing is shown by what the command-line page contains: the 5.1 command-line arguments include no switch for Overwrite or Placeholders. The render options listed are `-b`, `-a`, `-S`, `-f`, `-s`, `-e`, `-j`, `-o`, `-E`, `-t`, plus the Cycles options and the format options `-F` and `-x`. That absence is **documented**, listed rather than quoted. Going on to *a render started from outside must therefore carry these two settings in the blend-file, or set them through Python as `use_overwrite` and `use_placeholder` on the scene's render settings* is a step the reader supplies, since the page does not say those are the only ways in: **inferred**. Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, and Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — both read 2026-07-30. Whether the sharing arrangement itself behaves the same when driven from outside is not stated.

**Operating system.** The sources do not say. The render-farm hint speaks of "a shared directory over a network file system" without naming any operating system or any file system, and does not say the result differs between them. The only mention of a system anywhere near these settings is the phrase "similar to Unix 'touch'" used to explain what an empty placeholder file is. **Brought up to date by the hand tests of 2026-08-11:** every test above was run on macOS 26.5.2 on an Apple M1 Pro **and** on Ubuntu 22.04.5 LTS on an AMD Threadripper, and the two agreed on every point of behaviour. The one number that differed is how often two renders collide on one frame, which is a race, not an operating-system difference. **The file system does change it**: over a folder mounted by NFS the collision rate went from none in thirty repeats to four, five or six in thirty. **Windows is untested**, no Windows machine having run it.

---

## Things a person with Blender open could settle by hand in a few minutes

**Pruned on 2026-08-11, and pruned again on 2026-08-12** after the second batch of hand tests. All eight items below were carried out, in all four researched versions, on the Creator's Mac and on a rented Linux machine; what came back is written into questions 13 and 14 above as observed findings, and the list they were run from is `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`. Item H — what a second render treats as already there — was asked only by this version's documents and was run in all four, so no version's silence about it stands for a difference in Blender. What is left open is what a running Blender on either of those two machines could not reach:

1. **Two renders on two separate computers writing into one shared folder** — the arrangement the manual's own render-farm hint describes. The project's two machines have no file system shared between them. A network file system *was* tested, with both renders on the rented Linux machine writing to a folder mounted over NFS from itself, and it changed the answer — see question 14.
2. **Anything on Windows.** No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
3. **A real power cut, as against a kernel reset.** The reset was done twice, on 2026-08-11 and again on 2026-08-12, and the two runs do not agree about which directory entries survive. Both results are under question 13. Pulling a plug is still not reachable on either machine.
4. **Whether Blender receives an Escape key sent by a tool, and so what a person pressing Esc by hand would get.** Whether the render *stops* has been run three times on the rented Linux machine and has come out both ways; all three runs are under question 13 and none has been chosen. No run of this shape can show whether the key reached Blender at all.
5. **What a person clicking the status bar's cancel button leaves behind.** That control needs a hand on a mouse.

The eight items are kept below as written, since each names the exact set-up that was used.

These are for questions 13 and 14 only. Each says exactly what to do, what to look at, and what a result the other way would have looked like.

**A. What a stopped write leaves at the frame's path (question 13).**
1. Open a scene heavy enough that one frame takes about a minute. Set Output ‣ File Path to an empty folder, Media Type to Image, File Format to PNG. Leave Overwrite on and Placeholders off.
2. Set the frame range to a few frames. Press Ctrl-F12 to render the animation.
3. While the frame counter shows a frame being worked on, watch the folder in a file manager with file sizes shown. Note whether a file for the frame in progress appears before that frame finishes, and if so what size it is.
4. Repeat, and this time press Esc during the render. Look at the folder: note the name and size of every file, and whether a file exists for the frame that was in progress.
5. Try to open each file in Blender's Image Editor. Note which open and which do not.

What a result the other way would look like: no file at all at the in-progress frame's path while that frame is being worked on, and after the stop a folder holding only whole files that all open — nothing part-written, nothing of an odd size.

**B. The same, but killed rather than cancelled (question 13).**
Repeat A, but instead of pressing Esc, end the Blender process from the operating system's task manager or process monitor while a frame is being written. Then look at the folder as in step 4 and note the same things. Compare with A: the interesting difference is whether a partly written file survives a kill but not a cancel.

What a result the other way would look like: the folder after a kill holding exactly what it held after a cancel — no extra file, and no file of a different size.

**C. Whether a temporary name is used while writing (question 13).**
Repeat A, and while a frame is being written list the *whole* output folder including hidden files, and also the system temporary folder. Note whether any file appears under a name that is not the final frame name — for example with an extra extension or a leading dot — and whether it later becomes the final file.

What a result the other way would look like: no name anywhere but the final frame names — nothing hidden, nothing with an extra ending, and nothing in the system temporary folder that later turns into a frame.

**D. The same two tests with the Placeholders setting on (question 13).**
Repeat A and B with Placeholders enabled and Overwrite disabled. Note whether an empty file appears for each frame before it is rendered, how far ahead of the render it appears (one frame ahead, or all frames at once), and what size the file for the interrupted frame has after the stop.

What a result the other way would look like: no empty file appearing ahead of any frame with Placeholders on — that is, the folder filling up exactly as it did with the setting off.

**E. Two renders into one folder, at the defaults (question 14).**
1. Save a blend-file with a frame range of, say, 1 to 20, output set to an empty shared folder, image format PNG, Overwrite on and Placeholders off.
2. From a terminal, start two background renders of that same file at once, both writing to that folder.
3. When both finish, note: how many files are in the folder, whether each render's console output lists all 20 frames or fewer, how long each took compared with one render alone, and whether any file is unreadable or of an odd size.

What a result the other way would look like: twenty files in the folder, each render's console listing all twenty frames, each taking about as long as one render alone, and every file readable — that is, the two renders not touching each other at all.

**F. Two renders into one folder, set up as the manual's render farm (question 14).**
Repeat E with Overwrite disabled and Placeholders enabled in the blend-file. Note the same things, and additionally: whether the two renders between them rendered each frame once or some frames twice, whether any frame was left as a zero-length file at the end, and whether the two consoles ever report working on the same frame number.

What a result the other way would look like: the same as at the defaults — both renders listing all twenty frames, every frame rendered twice, and nothing in either console suggesting a frame was left to the other.

**G. Starting the two renders at the very same instant (question 14).**
Repeat F, but start both renders from a single command so they begin together rather than one after the other. Watch specifically the first frame of the range: note whether both renders work on frame 1, and what the file for frame 1 looks like at the end.

What a result the other way would look like: only one of the two renders ever reporting frame 1, and the file for frame 1 whole at the end — that is, no difference from starting the two one after the other.

**H. Which files a second render treats as already there (question 14).**
With Overwrite disabled, put files into the output folder by hand before rendering: one complete PNG at a correct frame name, one zero-length file at another correct frame name, and one small file of nonsense bytes at a third. Render the range and note which of the three the render leaves alone and which it renders over. That settles what "already there" means to this setting.

What a result the other way would look like: the render treating all three the same — leaving all three alone, or rendering over all three — so that what is inside a file makes no difference to the setting.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Frames, Blender 5.1

2026-08-12 — Claude Opus 5 (Claude Code) — the second batch of hand tests written in: entries 22, 23, 24 and 25 run and recorded under question 13, both standing lines brought up to date, and the researcher's own list pruned again.
