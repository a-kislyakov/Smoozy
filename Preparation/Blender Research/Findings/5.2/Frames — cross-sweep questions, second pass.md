# Frames — cross-sweep questions, second pass (Blender 5.2)

Sources consulted: the Blender 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` folder), the Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), the Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files (`Blender Sources/cycles_5.2`). All were taken on 2026-07-30, and that is the date read for everything quoted below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Question answered: 13 only.

---

## 13. When a render is stopped at the exact moment it is writing a frame's own image file, what is left where it writes?

**LABEL: not in the sources.**

The 5.2 manual, the 5.2 Python API documentation, the 5.2 release notes and Cycles' own 5.2 files nowhere describe what sits at the output path when a render is halted part-way through writing a frame's image file. They do not say whether the file is left partly written, left at zero length, left absent, written somewhere else first and moved into place, or removed on the way out. There is no statement anywhere in these sources about partial, truncated, or half-written image files.

**Reading of the question.** I answered the reading I think is meant: the image file for one frame at the path set in Properties ▸ Output ▸ Output ▸ File Path, being written by an animation render. The question is ambiguous in one way worth naming — "a frame's own image file" could instead mean a file written by a Compositor File Output node for that frame. The sources are silent for that reading too; the searches below covered both, and nothing in the File Output material speaks to interruption during a write either.

### What the sources do say that touches on this

These are the nearest things I found. None of them answers the question; I set them out so the edge of the silence is visible.

**(a) A file can already exist at that path before the frame is written, if Placeholders is on.** Documented.

> "Create empty placeholder files while rendering frames (similar to Unix ‘touch’)"

Source: Python API documentation, `bpy.types.RenderSettings.use_placeholder`, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30. The same page gives its default: `(default False)`.

The manual carries the same option in shorter words:

> "Create empty placeholder frames while rendering."

Source: manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. (Note for whoever checks this: on that page the heading above this sentence is a duplicate of the Overwrite heading above it — the page prints "Image Sequence – Overwrite Image Multi-Layer EXR" twice. The sentence quoted is the Placeholders one.)

I also opened the panel screenshot on that page, `_images/render_output_properties_output_panel.png`, part of https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. Visible in it: a checkbox labelled **Overwrite** (ticked) and, beside it, a checkbox labelled **Placeholders** (unticked), plus **File Extensions** (ticked), **Cache Result** (unticked), File Format PNG, Color BW/RGB/RGBA with RGBA active, Color Depth 8/16 with 8 active, Compression 15%. That confirms both options exist in the Output panel under those names. **Label for what the picture shows: documented.**

**(b) Placeholders are created ahead of the frame, not after it.** Inferred — here is the step. The manual's render-farm hint says:

> "Disable Overwrite, enable Placeholders in the Render Output panel."
> "Start as many machines as you wish rendering to that directory."

Source: manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. For that arrangement to divide work between machines, each machine must be able to see that a frame is already claimed before it is finished; that only works if the empty file appears at the start of the frame rather than at the end. The source does not say this outright. The reader has to supply the step, so this is inferred, not documented. What it would mean for question 13: with Placeholders on, a file of some kind is at the path from before the write begins, so an interruption during the write cannot leave the path empty. What that file contains at the instant of interruption — nothing, or part of an image — is still not in the sources.

**(c) Writing a frame is a distinct step with its own duration.** Documented, and it bounds the question rather than answering it.

> "on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

Source: Python API documentation, `bpy.app.handlers.render_write`, https://docs.blender.org/api/5.2/bpy.app.handlers.html — read 2026-07-30.

> "on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."

Source: Python API documentation, `bpy.app.handlers.render_stats`, same page and date. Saving is reported as its own measured time, separate from render time. **Inferred** from that, and stated as inference: because saving is measured as its own span of time, there is a window during which a frame is being saved and is not yet saved, and a stop can land inside it. The step a reader has to supply is that a measured "saving time" implies a period of writing. Neither quoted line says what is on disk inside that window. *(Label corrected by the citation check of 2026-08-11: the window clause had sat under the **documented** label, though it needs a reader's step. This version's first-pass document makes the same move and labels it inferred; the two now agree.)*

**(d) A render job can be stopped.** Documented, but with no word on what it leaves.

> "The task can be aborted by clicking the cancel button ()."

Source: manual, https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html — read 2026-07-30.

> "when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

Source: manual, https://docs.blender.org/manual/en/5.2/render/output/introduction.html — read 2026-07-30. This speaks about frames already finished, not about the one being written when the stop arrives.

**(e) The manual does warn about damage — but only for video, not for image frames.** Documented, and quoted here because it shows the manual raises the subject in one place and not the other.

> "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Source: manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. That sentence is about rendering directly to a video format. Nothing equivalent is written about a single frame's image file.

### What running it settled — the hand tests of 2026-08-11

**LABEL: observed.** Twenty-one tests, built from all eight of this round's findings documents and the four audits and saved at `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, were run in Blender 4.5.12, 5.0.1, 5.1.2 and 5.2.0 on two machines. **Machine one, the Creator's Mac:** a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card. **Machine two, a rented Linux machine:** Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Every run had a settings folder of its own and `--factory-startup`. Everything quoted is Blender 5.2.0's own output; the two machines are never merged. The same results are in this version's first-pass document, in fuller form, along with everything the tests settled about question 14, which this document does not answer.

**Blender writes straight to the final name, and no temporary name is used.** Three frames at 4096 by 4096 as OpenEXR with no compression, the whole folder listed every two milliseconds. The only names that ever appeared were the three final frame names:

> ```
> WATCH    0.000  <empty>
> WATCH    1.588  frame_0001.exr=33690
> WATCH    1.591  frame_0001.exr=2904986
> ```

and 109 distinct sizes later the file reached 268,501,914 bytes. — Creator's Mac; the rented Linux machine did the same, ending at 268,501,875 bytes. **The two machines agreed.** So Blender does not write somewhere else first and move the finished file into place.

**A stopped write does leave a file at the path with Placeholders off, and how the stop is delivered decides what.** Each run was stopped the instant frame 2's file first appeared, with Overwrite on and Placeholders off. An **interrupt** did not cut the write: Blender finished the frame — `00:02.703  render           | Saved: '…/frame_0002.exr'`, `Time: 00:01.04 (Saving: 00:00.32)`, then `Sent an internal break event. Press ^C again to kill Blender`, exit code 1 — and the file was whole and opened. A **terminate** left 33,695 bytes on the Creator's Mac and 16,384 on the rented Linux machine, exit code -15, no `Saved:` line, unopenable. A **kill** left 0 bytes and 32,768 bytes, exit code -9, unopenable. That answers the first two items in *Not covered* below: a file is left, and some of the bytes written before the stop remain.

**The format matters, and a part-written file is not always distinguishable from a whole one.** The same kill per format at 4096 by 4096: PNG left 0 bytes on both machines; JPEG 0 on both; OpenEXR 5,027 on the Mac and 32,772 on Linux; TIFF **1,306 bytes on the Mac, which opened and reported 4096 × 4096 with `has_data: True`**, and 1,278 on Linux, which did not — against a complete TIFF of 5,398,529 bytes. Multi-layer OpenEXR needed `media_type` set to `MULTI_LAYER_IMAGE` first, and then wrote one file per frame, 537,036,607 bytes complete and 55,103 after a kill.

**Nothing was still being held back by the operating system.** After a kill the folder was listed at once and again sixty seconds later; the two listings were identical, on both machines and in all four versions.

**Placeholders are created before the frame is rendered, one frame at a time.** With Overwrite off and Placeholders on: `WATCH 0.753 frame_0001.exr=0`, with the first real bytes about a second later, and frame 2's file not appearing until frame 1 was finished. That is exactly the step this document marks as inferred at (b), now observed. Stopped once frame 2's file had grown past zero, the leftover was 33,691 bytes on the Creator's Mac and a short unopenable file on the rented Linux machine. **The two machines agreed.**

**A Compositor File Output node writes before the render's own frame file.** This is the other reading of the question named above. With a File Output node writing OpenEXR and the Output panel writing PNG, a complete frame printed the node's save first. Stopped the instant the node's own second file appeared, the folder held that file at 13,022 bytes on the Creator's Mac, unopenable, and **no Output-panel file at all for that frame**. So the two writers do not behave the same way at a stop: the node's file is left short and the render's own file is not there at all. *(The citation check of 2026-08-11 could not repeat this. It confirmed from Blender itself that the scene carries a `compositing_node_group` and that the node's properties are `directory` and `file_name` rather than `base_path`, but what is written here does not say how the node is wired into the new compositor, and the node wrote no file at all when it was set up from this description. The 4.5 result repeated exactly. Written out as item 24 on the hand-tests list.)*

**Cancelling from inside Blender's own window did not stop the render.** On the rented Linux machine only: a forty-frame render in a real window with the render window holding the keyboard, Escape sent to every Blender window repeatedly for ten seconds with `xdotool`. Fifteen seconds later the render was still going and went on writing whole frames, in all four versions. This shows the render did not stop; it cannot show Blender received the key, so it does not settle what an Escape pressed by a person would do.

**The citation check of 2026-08-11 repeated this and got the opposite, and both runs are recorded rather than one replacing the other.** Following the procedure written above on the rented Linux machine, the render **stopped**: sixteen frames stood in the folder when the first Escape was sent and sixteen stood there fifteen seconds after the last one, in Blender 4.5, run twice; in 5.2 it was fourteen and fourteen. A control identical in every respect except that no Escape was ever sent went from sixteen frames at twenty-five seconds to thirty-six at fifty seconds in 4.5, and from fourteen to thirty-three in 5.2 — so without the Escape the render kept going, and with it the render stopped. That is a question, not an answer. It is item 22 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, to be run again, and until it is run nothing downstream should treat either result as the fact.

**A machine going down loses even frames it had already reported saving.** On the rented Linux machine only, the machine was reset at the kernel — `echo b > /proc/sysrq-trigger`, nothing flushed — the instant frame 2's file appeared. Afterwards the output folder held **`frame_0001.exr` at 0 bytes and nothing else**, and the run's log file was 0 bytes; frame 1 had been reported saved seconds earlier, its name survived and its contents did not. A plain `reboot` had been tried first and the machine came back from it. This is the hard-stop case the *Not covered* line names, and it comes out differently from every kill above. *(The citation check of 2026-08-11 did not repeat this: the kernel reset was refused by the permission system of the machine that check ran on. The result stands on its single run, and is item 25 on the hand-tests list.)*

### What the second batch of hand tests settled — 2026-08-12

**LABEL: observed.** Four more tests — entries 22 to 25 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md` — were added by the citation check of 2026-08-11: three because a repeat came back differently from a run recorded above, and one because a procedure could not be followed as written. They were run on 2026-08-12 on the machines each entry names, on the same two machines described above, each with a settings folder of its own and `--factory-startup`. Everything quoted is Blender 5.2.0's own output, and the two machines are never merged. The same results are in this version's first-pass document, in fuller form. **Where a result differs from one written above, both stand and neither has been chosen.**

**Escape sent to Blender's own window stopped the render — entry 22.** On the rented Linux machine only, the one machine with a screen this round could drive. A forty-frame render at 480 by 480 with eighty samples, PNG, started in a real window from a timer. At twenty-five seconds the folder held 32 files, `xdotool getwindowfocus getwindowname` answered `Blender Render`, and `xdotool search --name Blender` answered `62914562 62914691`. Escape was sent to both windows and to the focused window twenty-six times over ten seconds; the folder held 33 files ten seconds later and 33 fifteen seconds after that — one more frame finished, then nothing. A control in the same sitting with no Escape sent went 32, then 40, then 40. The other three versions came out the same way — 4.5 froze at 30, run twice, 5.0 at 35, 5.1 at 38, while every control reached all 40 frames of the range. **This agrees with the citation check's repeat and disagrees with the run recorded above. Both stand.** It still cannot show that Blender received the key.

**A stop leaves a different number of bytes almost every time, for JPEG and for TIFF — entry 23.** Ten repeats per format per version per machine: the three-frame render at 4096 by 4096 was killed the instant frame 2's file appeared, and its size and whether it opened were recorded. In Blender 5.2 on the **Creator's Mac** the ten JPEG leftovers were 8,192, 0, 0, 0, 0, 4,096, 0, 0, 4,096 and 0 bytes, three opening; the ten TIFF leftovers were 238, 8, 254, 1,306, 1,306, 0, 8, 1,306, 8 and 8 bytes, three opening. On the **rented Linux machine** the ten JPEG leftovers were 0, 0, 4,096, 0, 0, 0, 0, 4,096, 0 and 0 bytes, two opening; the ten TIFF leftovers were 8, 1,306, 1,286, 1,306, 1,306, 1,306, 1,306, 1,306, 1,306 and 1,306 bytes, seven opening — and of the eight files of exactly 1,306 bytes, seven opened and one did not. Complete frames were 4,137,461 and 5,398,529 bytes on the Mac and 4,123,208 and 5,400,506 on the rented machine. **Every cell is a spread**, so the single figures written above, and the different single figures the citation check got, are draws from a race rather than facts about a version or a machine.

**The Compositor File Output node set-up written out, and entry 7 run again from it — entry 24.** On the Creator's Mac. The script the hand-test list carries was run again unchanged and the node **did** write its files, so it is the description in this document, not the list's script, that could not be rebuilt from. Taking the set-up apart one piece at a time showed which pieces the node needs: without the item added by `file_output_items.new`, without the link into that input, without a Render Layers node feeding it, without `use_compositing`, or without the group being put on `scene.compositing_node_group`, the node wrote nothing; without the group's output socket and Group Output node it still wrote. Blender 5.0 and 5.1 gave the same seven answers. So the set-up is: a node group of kind `CompositorNodeTree` on `scene.compositing_node_group`; a Render Layers node inside it; a File Output node with an item added by `file_output_items.new("RGBA", "Image")`, which is what creates the input; a link from the Render Layers node's Image output into that input; `use_compositing` switched on; and the node's `directory` and `file_name` set. With that, entry 7 ran as written: a complete run printed the node's save first every time — `00:02.533  render           | Saved: '…/node/nodeImage0001.exr'` before `00:03.231  render           | Saved: '…/out/frame_0001.png'` — and a run killed the instant the node's own second file appeared left `nodeImage0002.exr` at **13,013 bytes**, which did not open, and **no Output-panel file at all for frame 2**. In 5.0 the same stop left 721 bytes and in 5.1 0 bytes, neither opening. That repeats the shape of the result recorded above, and with it the point that the two writers do not behave the same way at a stop.

**The kernel reset, run a second time, left the folder present and empty in every version — entry 25.** On the rented Linux machine only, run last. A plain `reboot` was tried first and the machine came back. Once per version, the three-frame OpenEXR render was started with no window and the machine reset at the kernel the instant frame 2's file appeared. In Blender 5.2 the output folder came back present and `<empty>`, the run's own log file did not exist at all, and the blend-file survived whole at 96,018 bytes. Frames 1 and 2 were both absent, and frame 1 had been reported saved before the reset. The folder was present and empty in 4.5, 5.0 and 5.1 as well. **That disagrees with the run recorded above**, which had `frame_0001.exr` present at 0 bytes in 5.2. Both stand. What came out the same way twice: no frame's contents survived in any version, including a frame already reported saved.

**Not covered.** This answer does not settle: whether a stopped write leaves a file at the path at all when Placeholders is off; whether any bytes written before the stop remain; whether Blender writes to a temporary name and moves the finished file into place; whether behaviour differs between image formats (PNG, JPEG, OpenEXR, multi-layer OpenEXR) or between one-shot and progressive writing; whether it differs between a polite cancel (Esc, the status-bar cancel button, an interrupt signal) and a hard stop (power loss, a killed process); whether Blender removes an unfinished file as it shuts down; and what the file's contents are if it does survive. It also does not settle the same questions for files written by a Compositor File Output node. None of that is in these sources.

**Outside the interface.** The sources do not say. I found no statement that this behaviour — or any behaviour around writing a frame's file — is either the same or different when Blender is started and driven from outside its own window (`blender -b … -a`, `--render-frame`). The command-line pages I read describe which frames get rendered and where output goes, not what a halted write leaves. Two things that are stated hold regardless of how the render was started, because they are scene render settings rather than interface features: `use_overwrite` and `use_placeholder` are properties of `bpy.types.RenderSettings` (https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, read 2026-07-30), so they are part of the file being rendered. That much is documented. Everything else about the interrupted write is not. Note also that the one documented way of stopping named in these sources — the status-bar cancel button — is an interface control and has no described counterpart outside the window. **Brought up to date by the hand tests of 2026-08-11:** every result under *What running it settled* was got from a render driven from outside Blender's window, with no window at all, so the answer holds outside the interface — that is where it was measured. Sending Escape to Blender's own window on the rented Linux machine did not stop the render, so what a person pressing Esc or clicking the status bar's cancel button would get is the only part not established from outside. **Brought up to date again by the second batch of hand tests, 2026-08-12:** the Escape test was run again with a control and this time the render **stopped**, in all four versions — entry 22 above. The two runs disagree and neither has been chosen. The hand half stays open either way, because no run so far can show that Blender received the key.

**Operating system.** The sources do not say. Nothing I found states any per-operating-system difference in what a halted write leaves, and nothing states that it is the same everywhere either. The only OS-flavoured wording anywhere near this is the API's comparison of placeholders to "Unix ‘touch’", which is a description of the effect, not a statement that the option behaves differently on Windows or macOS. **Brought up to date by the hand tests of 2026-08-11:** everything above was run on macOS 26.5.2 on an Apple M1 Pro **and** on Ubuntu 22.04.5 LTS on an AMD Threadripper, and the two agreed on every point of behaviour; where they differ is in numbers that are races. **Windows is untested**, no Windows machine having run it, so this answer covers macOS and Linux and leaves Windows open. **Brought up to date again by the second batch of hand tests, 2026-08-12:** entry 23 ran the JPEG and TIFF stop ten times per format per version on each machine, and both machines gave a spread of sizes and a mixture of files that opened and files that did not, which shows the race directly. Entries 22 and 25 were run on the rented Linux machine only, so what an Escape does inside a window and what a machine going down leaves are answered for Linux and left open for macOS.

---

## Record of where I looked

Everything below was run against `Blender Sources/` only.

**Whole-tree text searches (case-insensitive) across `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2`.** Terms searched, with the outcome:

- Words for a half-finished file: `partial`, `partially`, `partial file`, `partial image`, `partial frame`, `incomplete`, `truncat`, `unfinished`, `half-written`, `corrupt`, `damaged`, `unreadable`, `cannot be read`, `invalid image`, `black frame`, `garbage`. Every hit was about something else — box-select selecting partially enclosed items, EEVEE feature limitations, text truncation in fields, importer warnings about corrupt mesh data, GPU troubleshooting. Nothing about an output file.
- Words for how a file is written: `atomic`, `temporary file`, `.tmp`, `rename`, `flush`, `fsync`, `buffered`, `streaming`, `in-place`, `close the file`, `file handle`, `checksum`, `scanline`, `incremental`, `zero byte`, `0 bytes`, `empty file`. Hits were the temporary-directory preference, node and text-editor features, the audio module's caching, and an image-buffer crop/resize; none about writing rendered frames.
- Words for a stop: `interrupt`, `interrupted`, `cancel`, `canceled`, `abort`, `aborted`, `stopped`, `Ctrl-C`, `SIGTERM`, `signal`, `terminate`, `kill`, `quit`, `exit code`, `crash`, `power fail`, `power supply`, `while writing`, `being written`, `during the write`, `in the middle of`. The only render-relevant hits are the ones quoted in (d) above, plus `render/output/animation.txt` line 101 ("May need to interrupt rendering to use the computer, and want to be able to resume rendering where you left off") and its "Your computer accidentally turns off in the middle of rendering your movie!" hint — both of which are about restarting from frames already complete, not about the frame in hand.
- Words for a failed write: `failed to write`, `unable to save`, `cannot save`, `save failed`, `write error`, `disk full`, `out of disk`, `leftover`, `stale file`. One hit inside Cycles (`osl.py` line 216, an error message about writing a compiled shader file next to a script — nothing to do with frames), one bug-fix line about a File Output node and multi-layer EXR, nothing else.
- `placeholder` and `save_buffers` across manual and API, to find every place the option is described. `save_buffers` does not appear at all in 5.2. `placeholder` hits are listed in (a); the rest are glTF export, geometry-nodes string formatting, UI tooltips, sequencer strip placeholders, and library-link placeholder data-blocks.

**Pages opened in full and read:**

- https://docs.blender.org/manual/en/5.2/render/output/introduction.html
- https://docs.blender.org/manual/en/5.2/render/output/animation.html
- https://docs.blender.org/manual/en/5.2/render/output/properties/output.html
- https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html (Status Messages section)
- https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html (Render Menu section)
- https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html
- https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html (Temporary Directory section)
- https://docs.blender.org/manual/en/5.2/files/media/image_formats.html (scanned for every mention of write/written/save/disk/buffer)
- https://docs.blender.org/api/5.2/bpy.app.handlers.html
- https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html (the `use_overwrite` and `use_placeholder` entries)
- https://docs.blender.org/api/5.2/bpy.ops.render.html (the full list of render operators, and the `render`, `opengl`, `view_cancel` entries)
- https://docs.blender.org/api/5.2/bpy.types.Image.html (the `save`, `save_render` entries)
- https://developer.blender.org/docs/release_notes/5.2/rendering/ (read in full; the Render Output section lists two changes, "Saving render output can now be disabled." and "Light/shadow linking can be easily copied to the selection" — *corrected by the citation check of 2026-08-11, which had been written as though the first were the only one*)

All 21 files of the 5.2 release notes were searched for `output`, `write`, `cancel` and `interrupt`; nothing in them bears on an interrupted write.

**Picture opened:** `_images/render_output_properties_output_panel.png`, belonging to https://docs.blender.org/manual/en/5.2/render/output/properties/output.html. What I took from it is in (a).

**Cycles' own files, and why they were ruled out.** I listed every file in `Blender Sources/cycles_5.2` and searched all of its Python files for `write`, `save`, `filepath` and `cancel`. What is shipped there is the add-on's Python layer (`__init__.py`, `properties.py`, `ui.py`, `engine.py`, `operators.py`, `osl.py`, `camera.py`, `presets.py`, `version_update.py`, `maketx.py`), the compiled OSL shaders, and a set of kernel and utility header files under `source/`. None of it writes rendered frames to the output path — that is done by Blender, not by Cycles, and Blender's own code for it is not in this folder. So Cycles' files cannot answer this question, and I claim nothing observed from them about it.

Two adjacent Cycles findings I deliberately did **not** treat as an answer, since they are about other files in other places:

- `cycles_5.2/properties.py`, the `tile_size` property (lines 1084–1090), reads: `description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory"`. The manual repeats it at https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html — "Tiles are cached to disk while rendering to save memory." **Label: observed** (from the file) **and documented** (from the manual), read 2026-07-30. This is a cache written while the frame is still rendering, not the frame's own image file at the output path, and neither source says where that cache lives or what becomes of it if the render is stopped.
- `bpy.ops.cycles` carries a denoise operator whose `output_filepath` parameter says renders "will be denoised in-place" if no path is given (https://docs.blender.org/api/5.2/bpy.ops.cycles.html, read 2026-07-30). That is a separate operator run over already-rendered files, not the render's own write, so it is out of scope for this question.

**I did not use the web at any point, and did not need to.**

---

## Things a person with Blender open could settle by hand in a few minutes

**Pruned on 2026-08-11, and pruned again on 2026-08-12** after the second batch of hand tests. Every item below was carried out, in all four researched versions, on the Creator's Mac and on a rented Linux machine; what came back is written into the answers above as observed findings, and the list they were run from is `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`. What is left open is what a running Blender on either of those two machines could not reach:

1. **Two renders on two separate computers writing into one shared folder** — the arrangement the manual's own render-farm hint describes. The project's two machines have no file system shared between them, and sharing one would mean installing software on the Creator's working machine. A network file system *was* tested, with both renders on the rented Linux machine writing to a folder mounted over NFS from itself, and it changed the answer.
2. **Anything on Windows.** No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
3. **A real power cut, as against a kernel reset.** The reset was done twice, on 2026-08-11 and again on 2026-08-12, and the two runs do not agree about which directory entries survive. Both results are written in above. Pulling a plug is still not reachable on either machine.
4. **Whether Blender receives an Escape key sent by a tool, and so what a person pressing Esc by hand would get.** Whether the render *stops* has been run three times on the rented Linux machine and has come out both ways; all three runs are written in above and none has been chosen. No run of this shape can show whether the key reached Blender at all.
5. **What a person clicking the status bar's cancel button leaves behind.** That control needs a hand on a mouse.

The items are kept below as written, since each names the exact set-up that was used.


Each of these would answer something the sources do not.

1. **Does an interrupted write leave a file, and what size is it?** Open Blender 5.2. In Properties ▸ Output ▸ Output, set File Path to an empty folder, set File Format to PNG, leave Overwrite ticked and Placeholders unticked. Set the resolution high (say 7680 × 4320) so writing takes a noticeable moment, and the frame range to a single frame. Render the animation with Ctrl-F12, and click the cancel button in the status bar at the instant the status line changes from rendering to saving. Then look in the folder with a file manager and note: is there a file for that frame, what is its size in bytes, and will an image viewer open it?
2. **The same test with Placeholders on.** Repeat step 1 with Placeholders ticked and Overwrite unticked. Note whether the file appears at the start of the frame rather than the end (watch the folder while the frame renders), and what its size is if the render is cancelled before the frame finishes.
3. **The same test from outside the window.** Run `blender -b file.blend -o /some/empty/folder/frame_##### -F PNG -a` from a terminal, and press Ctrl-C while a frame is saving. Note what is in the folder afterwards, and how the last file's size compares to a complete one.
4. **A hard stop rather than a polite one.** Repeat step 3 but end the process outright (Task Manager on Windows, `kill -9` on macOS or Linux) during a save. Compare what is left with what step 3 left.
5. **Format by format.** Repeat step 1 for OpenEXR, multi-layer OpenEXR, JPEG and TIFF, since each writes differently, and note whether the remains differ.
6. **Is a temporary name used?** While a large frame is saving, watch the output folder for any file whose name is not the final frame name (a dot-prefixed or `.tmp`-suffixed name, for instance). If one appears and is then renamed, that settles the question outright.
7. **Compositor File Output node.** Repeat step 1 with a File Output node writing the frame instead of the Output panel, and note whether what is left behind differs.
8. **Run all of the above on each operating system available** — Windows, macOS and Linux — since nothing in the sources says the answer is the same on all three.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Frames, Blender 5.2

2026-08-12 — Claude Opus 5 (Claude Code) — the second batch of hand tests written in: entries 22, 23, 24 and 25 run and recorded under question 13, both standing lines brought up to date, and the researcher's own list pruned again.
