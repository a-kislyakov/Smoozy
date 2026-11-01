# Frames — cross-sweep questions (Blender 5.0)

Questions 13 and 14 from the Frames question sheet.

**Sources consulted, and when.** Blender's 5.0 manual, Blender's 5.0 Python API documentation, Blender's 5.0 release notes, and the Cycles files supplied for 5.0 — all as the plain-text and plain-file copies in `Blender Sources/`, taken on 2026-07-30. That snapshot date is the date read for everything cited below. I used no other source, and I did not go to the web.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

---

## 13. When a render is stopped at the exact moment it is writing a frame's own image file, what is left where it writes?

**LABEL: not in the sources.**

The sources never describe what sits on disk when a write to a frame's image file is cut off part-way. There is no statement anywhere in the 5.0 manual, the 5.0 API documentation, the 5.0 release notes, or the supplied Cycles files about a truncated file, a partly-written file, a zero-length file, a corrupt file, or a temporary file that is renamed once the write finishes. The question is about what Blender *does* at an instant, and the documentation describes what the features *are*.

**Note on the question.** It is ambiguous in two places, and I answer the reading I think is meant rather than rewriting it. First, "stopped" — I take it to cover both a render the user cancels and a render whose process is killed or whose machine goes down, because the sources are silent on both and I found nothing that separates them. Second, "a frame's own image file" — I take it to mean the per-frame image file an animation render writes to the output path, as opposed to a video container. If the intended reading were narrower (only a deliberate cancel from inside Blender), the answer is the same: not in the sources.

### What the sources do say, and where it stops short

These are the nearest documented statements. None of them answers the question; together they mark exactly how far the documentation goes.

**a. The documented name for a frame's file is the final frame path.** The quoted line says what this function returns. It does not say whether Blender writes straight to that name or to a temporary one first, and no page says either way — see the search record below.

> "Return the absolute path to the filename to be written for a given frame"

— Python API documentation, `RenderSettings.frame_path()`, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30. **documented**

**b. Writing a frame has a documented "after" point, and nothing else.**

> "on writing a render frame (directly after the frame is written)"

— Python API documentation, `bpy.app.handlers.render_write`, https://docs.blender.org/api/5.0/bpy.app.handlers.html — read 2026-07-30. **documented**

The same page lists `render_init`, `render_pre`, `render_post`, `render_complete`, `render_stats` and `render_cancel`. There is no before-the-write counterpart to `render_write`, and no handler that fires during a write.

**c. Cancelling a render is a documented event, described without reference to disk.**

> "on canceling a render job. Accepts one argument: the scene data-block being rendered"

— Python API documentation, `bpy.app.handlers.render_cancel`, https://docs.blender.org/api/5.0/bpy.app.handlers.html — read 2026-07-30. **documented**

**d. Cancelling and resuming is described only in terms of whole frames.**

> "when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

— Manual, Render ‣ Output ‣ Introduction, https://docs.blender.org/manual/en/5.0/render/output/introduction.html — read 2026-07-30. **documented**

> "since the frames you have already rendered will still be in the output directory."

— Manual, Rendering Animations, https://docs.blender.org/manual/en/5.0/render/output/animation.html — read 2026-07-30. **documented**

Both speak about frames that are already rendered. Neither says anything about the frame that was being written when the stop happened.

**e. Blender has a setting documented as creating empty placeholder files while frames are rendered, and it is off unless switched on.**

> "Create empty placeholder files while rendering frames (similar to Unix ‘touch’)"
>
> "Type:
>
> boolean, default False"

— Python API documentation, `RenderSettings.use_placeholder`, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30. **documented**

This is the only setting the sources describe as putting empty files down during a render, and it is a chosen setting rather than a consequence of an interruption. Two things the quoted words do not carry, and a reader has to supply: that those files sit at the frames' own output paths, and that "empty" means a file of zero length. **inferred** for both steps. The documentation does not say when in the sequence the placeholder is created, nor what becomes of it if the render stops before the frame is written.

**f. Searching the supplied Cycles files turned up nothing about writing frame files.** I searched the whole of `Blender Sources/cycles_5.0` for `placeholder` and `overwrite`. The matches are shading-kernel pixel functions in `source/kernel/film/write.h` and `source/kernel/film/data_passes.h` (named `film_overwrite_pass_rgbe`, `film_overwrite_pass_float`, `film_overwrite_pass_float3`) and in `source/kernel/film/volume_guiding_denoise.h` (`film_overwrite_pass_rgbe` only), which write pixel values into a render buffer in memory; one unrelated comment in `operators.py` line 76 ("Backup since we will overwrite the scene path temporarily"); and one more unrelated comment in `source/kernel/osl/closures_setup.h` line 40 ("set it to zero at the start and overwrite"). *(Corrected by the citation check of 2026-08-11: the sentence had said these were the only matches and had implied all three function names sit in `volume_guiding_denoise.h`. The sixth match and the true spread of the names are added; nothing among them writes a frame, so the conclusion is unchanged.)* The supplied Cycles files are the add-on's Python files and the render kernel headers. **observed** — searched the supplied files in `Blender Sources/cycles_5.0` on 2026-07-30 for the two terms named above. That is a record of looking. Two terms cannot show that no code anywhere in those files writes a frame's file, so this finding is that the search turned nothing up, not that nothing is there.

### Where I looked

Manual, API documentation, release notes and Cycles files for 5.0, searched whole rather than page by page. Terms searched, all case-insensitive: `truncat`, `corrupt`, `incomplete`, `partial`, `partially written`, `partially rendered`, `half-written`, `unfinished`, `temporary file`, `temp file`, `atomic`, `zero-byte`, `zero byte`, `empty file`, `complete file`, `valid image`, `leftover`, `left on disk`, `stale`, `flush`, `written to disk`, `write the file`, `saved directly`, `in progress`, `interrupt`, `cancel`, `abort`, `terminate`, `kill`, `crash`, `power fail`, `power loss`, `Ctrl-C`, `SIGINT`, `signal`, `unplayable`, `already exists`, `already rendered`, `existing file`, `if the file exists`, `as soon as`, `each frame is`, `after each frame`.

Pages ruled out after reading them in full or in the relevant part: Render ‣ Output ‣ Introduction; Rendering Animations; Render ‣ Output ‣ Properties ‣ Output; Render ‣ Output ‣ Properties ‣ Frame Range; the Status Bar page (its only relevant line is that a running task "can be aborted by clicking the cancel button", with nothing about files); the Command Line Arguments page; `bpy.ops.render`; `bpy.types.RenderSettings`; `bpy.app.handlers`. The only "unplayable" hit in the whole 5.0 manual is about video files, not image files, and belongs to a different question on the sheet.

### What running it settled — the hand tests of 2026-08-11

**LABEL: observed.** Twenty-one tests, built from all eight of this round's findings documents and the four audits and saved at `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, were run in Blender 4.5.12, 5.0.1, 5.1.2 and 5.2.0 on two machines. **Machine one, the Creator's Mac:** a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card. **Machine two, a rented Linux machine:** Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Every run had a settings folder of its own and was started with `--factory-startup`. Everything quoted is Blender 5.0.1's own output, and the two machines' results are given separately and never merged.

**Blender writes the frame straight to its final name, and the file grows in place.** Three frames at 4096 by 4096, OpenEXR at 32 bits with no compression, the whole folder listed every two milliseconds. Every name that ever appeared was a final frame name — no `.tmp`, no `.part`, no dot-prefixed or hidden name:

> ```
> WATCH    0.000  <empty>
> WATCH    1.297  frame_0001.exr=1082262
> WATCH    1.300  frame_0001.exr=3298198
> WATCH    1.304  frame_0001.exr=5239702
> ```

and 113 distinct sizes later it reached 268,501,910 bytes. — Creator's Mac. On the rented Linux machine it began `WATCH 1.304 frame_0001.exr=348160` and ended at 268,501,871 bytes, again with only the three final names ever seen. **The two machines agreed.**

**The write is announced as a step of its own, with the 5.0 logging prefix:**

> ```
> 00:01.443  render           | Saved: '…/frame_0001.exr'
> 00:01.443  render           | Time: 00:01.17 (Saving: 00:00.35)
> ```

— Creator's Mac; the rented Linux machine printed `00:01.432  render           | Time: 00:01.22 (Saving: 00:00.22)`. Blender 4.5 prints the same two lines bare, with no elapsed time and no `render` before them; 5.0, 5.1 and 5.2 all carry the prefix.

**How the stop is delivered decides what is left.** Three runs, each stopped at the instant frame 2's file first appeared:

- **An interrupt (`SIGINT`)** did **not** cut the write. Blender finished writing the frame and then stopped: `00:02.427  render           | Saved: '…/frame_0002.exr'`, `Time: 00:01.03 (Saving: 00:00.35)`, then `Sent an internal break event. Press ^C again to kill Blender`, exit code 1. Frame 2 was 268,501,914 bytes and opened as a 4096 by 4096 image.
- **A terminate (`SIGTERM`)** left a file at frame 2's path of **923 bytes**, exit code -15, no `Saved:` line, and Blender would not open it: `OPEN loaded | size: (0, 0) | channels: 0 | depth: 0 | has_data: False`.
- **A kill (`SIGKILL`)** left **21,403 bytes**, exit code -9, again unopenable.

— Creator's Mac. On the rented Linux machine: the interrupt again finished the write and left a whole 268,501,875-byte frame; the terminate left **32,772 bytes**; the kill left **8,196 bytes**. **The two machines agreed on the pattern.** The exact leftover size is a race and differs run to run; that a file exists at the frame's own path and does not open does not.

**The format changes what the leftover looks like, and one format is dangerous.** The same kill, once per format, at 4096 by 4096:

| format | leftover on the Creator's Mac | leftover on the rented Linux machine | does it open |
| --- | --- | --- | --- |
| PNG | 0 bytes | 0 bytes | no |
| JPEG | 0 bytes | 0 bytes | no |
| OpenEXR | 33,695 bytes | 32,768 bytes | no |
| OpenEXR multi-layer, two view layers | 50,980 bytes | — | the reader cannot say, see below |
| TIFF | 0 bytes | 1,306 bytes | **on Linux, yes — reported as 4096 × 4096, has_data True** |

A part-written TIFF opened in Blender and reported the full picture size, so **a stopped-mid-write file is not always distinguishable from a finished one by asking whether it opens**. Size still tells them apart: a complete TIFF was 5,399,366 bytes against the 1,306-byte leftover. — 2026-08-11.

**Multi-layer OpenEXR needs a different setting in 5.0 than in 4.5, and that is a version difference in the machinery.** Setting `file_format` to `OPEN_EXR_MULTILAYER` while Media Type is Image is refused in 5.0:

> ```
> TypeError: bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in ('JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')
> ```

With `media_type` set to `MULTI_LAYER_IMAGE` first it is accepted, and one file per frame is written holding both view layers, 537,036,580 bytes complete and 50,980 bytes after a kill. In Blender 4.5 there is no `media_type` and the format is reachable directly. Both the failure and the repair are recorded here. Blender's own `bpy.data.images.load` reports `type: MULTILAYER | size: (0, 0) | has_data: False` for the **complete** multi-layer file as well as the part-written one, so for that one format the reader used here cannot tell whole from part — a limit of the test, not a finding about the file.

**The operating system had already put everything on disk.** After a kill the folder was listed at once and again sixty seconds later; both listings were identical, on both machines and in all four versions.

**With Placeholders on, a file of zero bytes is put at the frame's path before that frame is rendered — one frame at a time.** A four-frame render with Overwrite off and Placeholders on:

> ```
> WATCH    0.000  <empty>
> WATCH    0.300  frame_0001.exr=0
> WATCH    1.254  frame_0001.exr=1417216
> ```

Frame 1's file appeared at **0 bytes** a second before any pixels reached it, and frame 2's file did not appear until frame 1 was finished — 1.683 seconds in on the rented Linux machine, 1.553 on the Creator's Mac. No file ever appeared for a frame the render had not reached. That settles the step this document marks as inferred under question 14. Stopping the run once frame 2's file had grown past zero left 33,687 bytes on the Creator's Mac and 36,864 on the rented Linux machine, neither of which opened. **The two machines agreed.**

**The JPEG preview copy exists and is written after the frame.** With `use_preview` on and OpenEXR output, a `.jpg` sat beside every `.exr`, and the run printed `Saved: '…/frame_0001.exr'` at 00:00.528 and `Saved: '…/frame_0001.jpg'` at 00:00.606. — Both machines, agreeing.

**A Compositor File Output node writes before the render's own frame file, and is set up differently from 4.5.** In 5.0 the compositor is a node group on `scene.compositing_node_group` and the node takes a `directory` and a `file_name`, where 4.5 uses `scene.node_tree` and a `base_path`; the node's files are accordingly called `nodeImage0002.exr` in 5.0 and `Image0002.exr` in 4.5. A complete frame printed the node's save first and the Output panel's second. Stopped the instant the node's own second file appeared, the folder held that file at 21,210 bytes on the Creator's Mac and 0 bytes on the rented Linux machine — unopenable in both — and **no Output-panel file at all for that frame**. *(The citation check of 2026-08-11 could not repeat this. It confirmed from Blender itself that the scene carries a `compositing_node_group` and that the node's properties are `directory` and `file_name` rather than `base_path`, but what is written here does not say how the node is wired into the new compositor, and the node wrote no file at all when it was set up from this description. The 4.5 result repeated exactly. Written out as item 24 on the hand-tests list.)*

**Cancelling from inside Blender's own window did not stop the render.** On the rented Linux machine only, a forty-frame render started from a timer in a real window, with the render window holding the keyboard (`the window that has the keyboard: Blender Render`), and Escape sent to every Blender window repeatedly for ten seconds with `xdotool`. Fifteen seconds after the first Escape the render was still going and went on writing whole frames. The same in all four versions. This shows the render did not stop; it cannot show Blender received the key, so it does not settle what an Escape pressed by a person would do.

**The citation check of 2026-08-11 repeated this and got the opposite, and both runs are recorded rather than one replacing the other.** Following the procedure written above on the rented Linux machine, the render **stopped**: sixteen frames stood in the folder when the first Escape was sent and sixteen stood there fifteen seconds after the last one, in Blender 4.5, run twice; in 5.2 it was fourteen and fourteen. A control identical in every respect except that no Escape was ever sent went from sixteen frames at twenty-five seconds to thirty-six at fifty seconds in 4.5, and from fourteen to thirty-three in 5.2 — so without the Escape the render kept going, and with it the render stopped. That is a question, not an answer. It is item 22 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, to be run again, and until it is run nothing downstream should treat either result as the fact.

**A machine going down loses even frames it had already reported saving.** On the rented Linux machine only, the same three-frame OpenEXR render was started and the machine was reset at the kernel — `echo b > /proc/sysrq-trigger`, with nothing flushed — the instant frame 2's file appeared. The machine restarted, and afterwards **the output folder the run had made did not exist at all**, and neither did the run's own log file. Frame 1 had been reported saved seconds earlier and was gone with the rest. A plain `reboot` had been tried first and the machine came back from it, so the loss is the reset's doing and not a failure of the step. The same reset in the other three versions left the folder in place but empty in 4.5 and 5.1, and in 5.2 left `frame_0001.exr` present at **0 bytes**; exactly which directory entries survive is what the file system happened to have written down, and no frame's contents survived in any version. This is the *unflushed write* case, and it comes out differently from every kill above. *(The citation check of 2026-08-11 did not repeat this: the kernel reset was refused by the permission system of the machine that check ran on. The result stands on its single run, and is item 25 on the hand-tests list.)*

**There is a setting in 5.2 that switches saving off, and it does not exist in 5.0.** Asked of Blender itself, `RenderSettings.save_output` is **absent** in 4.5, 5.0 and 5.1 and present in 5.2 as `type=BOOLEAN | default=True | description='Write frames to disk for animation renders'`. Both machines, agreeing. That is the only property in the render settings, anywhere in the four versions, whose name or description speaks of saving render output.

### What the second batch of hand tests settled — 2026-08-12

**LABEL: observed.** Four more tests — entries 22 to 25 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md` — were added by the citation check of 2026-08-11: three because a repeat came back differently from a run recorded above, and one because a procedure could not be followed as written. They were run on 2026-08-12 on the machines each entry names, on the same two machines described above, each run with a settings folder of its own and `--factory-startup`. Everything quoted is Blender 5.0.1's own output, and the two machines are never merged. **Where a result differs from one written above, both stand and neither has been chosen.**

**Escape sent to Blender's own window stopped the render — entry 22.** On the rented Linux machine only, that being the one machine of the two with a screen this round could drive. A forty-frame render at 480 by 480 with eighty samples, PNG, into a fresh empty folder, started in a real window from a timer four seconds after launch. At twenty-five seconds the folder was counted and the machine was asked which window held the keyboard:

> ```
> COUNT at 25s, before any Escape: 35
> xdotool getwindowfocus getwindowname: Blender Render
> xdotool search --name Blender: 62914562 62914691
> ```

Escape was then sent to both of those windows and to the focused window, twenty-seven times over ten seconds, and the folder counted again:

> ```
> COUNT at 35s, right after the Escape window: 35
> COUNT at 50s, fifteen seconds after the last Escape: 35
> ```

A control in the same sitting, identical except that no Escape was ever sent, went `COUNT at 25s: 36`, then 40, then 40. The other three versions came out the same way: 4.5 stayed at 30, run twice, where its control reached 40; 5.1 went 37 to 38 and stopped; 5.2 went 32 to 33 and stopped. Every control reached all forty frames of the range, which is what ended its own climb; every test stopped short of forty and did not move again. **This agrees with the citation check's repeat and disagrees with the run recorded above under *Cancelling from inside Blender's own window*. Both stand.** It still cannot show that Blender received the key, so what a person pressing Esc with their own hand would get is not settled by it.

**A stop leaves a different number of bytes almost every time, for JPEG and for TIFF — entry 23.** Ten repeats per format per version per machine, each into a fresh empty folder: the three-frame render at 4096 by 4096 was started with no window and killed outright the instant frame 2's file first appeared, then that file's exact size was recorded and Blender's own image loader was asked to open it. In Blender 5.0 on the **Creator's Mac** the ten JPEG leftovers were 0, 0, 0, 0, 0, 0, 0, 0, 4,096 and 0 bytes; one of the ten opened and reported 4096 by 4096, against a complete JPEG of 4,127,962 bytes. The ten TIFF leftovers were 8, 0, 1,306, 8, 1,306, 1,306, 0, 8, 1,306 and 1,306 bytes; four of the ten opened, against a complete TIFF of 5,397,370 bytes — and note that of the five files of exactly 1,306 bytes, four opened and one did not, so even the same size is not always the same file. On the **rented Linux machine** the ten JPEG leftovers were 0, 0, 4,096, 0, 4,096, 4,096, 0, 0, 4,096 and 0 bytes, four opening, against a complete JPEG of 4,113,616 bytes; the ten TIFF leftovers were 1,306, 8, 8, 1,306, 1,306, 1,306, 238, 1,306, 1,306 and 8 bytes, six opening, against a complete TIFF of 5,399,366 bytes. **Every cell is a spread.** So the single numbers in the format table above, and the different single numbers the citation check got, are both draws from a race rather than facts about a version or a machine. A file of 0 bytes never opened, in any of the 160 repeats.

**The Compositor File Output node set-up written out, and entry 7 run again from it — entry 24.** On the Creator's Mac. First the script the hand-test list itself carries was run again in 5.0, unchanged, and the node **did** write its files — so it is the description in this document, not the list's script, that could not be rebuilt from. Then the set-up was taken apart one piece at a time, one frame at 256 by 256, to see which pieces the node needs. Leaving each out in turn:

| what was left out | did the node write a file |
| --- | --- |
| nothing — the whole set-up | yes |
| the item added with `file_output_items.new`, linking into the node's own spare socket instead | no |
| the group's output socket and its Group Output node | **yes** |
| the link from the Render Layers node into the node's input | no |
| `use_compositing`, set to off | no |
| the Render Layers node, feeding the node from a group input instead | no |
| putting the group on `scene.compositing_node_group` at all | no |

Blender 5.1 and 5.2 gave exactly the same seven answers. So the set-up, written out: make a node group of kind `CompositorNodeTree` and put it on `scene.compositing_node_group`; put a Render Layers node in it, since nothing else feeds the File Output node — a group input does not; put a File Output node in it and add an item with `file_output_items.new("RGBA", "Image")`, because the node is born with no real input and that call is what makes one; link the Render Layers node's Image output to that input; switch `use_compositing` on; and set the node's `directory` and `file_name`. A group output socket is **not** needed. With that set-up, entry 7 ran as written. A complete three-frame run at 4096 by 4096, PNG from the Output panel and 32-bit uncompressed OpenEXR from the node, printed the node's save first every time:

> ```
> 00:01.623  render           | Saved: '…/node/nodeImage0001.exr'
> 00:02.264  render           | Saved: '…/out/frame_0001.png'
> 00:02.264  render           | Time: 00:01.99 (Saving: 00:00.64)
> ```

Killed the instant the node's own second file appeared, the folder held `nodeImage0002.exr` at **721 bytes**, which did not open, and **no Output-panel file at all for frame 2**; `nodeImage0001.exr` was whole at 268,501,713 bytes and opened. Exit code -9. In 5.1 the same stop left the node's second file at **0 bytes** and in 5.2 at **13,013 bytes**, neither opening, and neither had an Output-panel file for that frame. That repeats the shape of the result recorded above.

**The kernel reset, run a second time, left the folder present and empty in every version — entry 25.** On the rented Linux machine only, and run last. A plain `reboot` was tried first and the machine came back in about a minute and a half; a bare kernel reset with nothing running was also issued once and the machine came back from that, so the step itself works. Then, once per version, the three-frame OpenEXR render at 4096 by 4096 was started with no window and the machine was reset at the kernel — `echo b > /proc/sysrq-trigger` — the instant frame 2's file appeared. After the reset, in Blender 5.0:

> ```
> LIST /root/frames-cross-sweep-2/e25/5.0/out
>    <empty>
> ```

The run's own log file did not exist at all, and the blend-file the run had been given survived whole at 92,519 bytes. Frames 1 and 2 were both absent, and frame 1 had been written and reported saved before the reset. The folder was present and empty in 4.5, 5.1 and 5.2 as well. **That disagrees with the run recorded above**, which had **no folder at all** in 5.0. Both runs stand and neither has been chosen. What came out the same way twice: **no frame's contents survived in any version, including a frame already reported saved.**

### Not covered

This answer does not settle: whether the outcome differs by image format (PNG, OpenEXR, multi-layer EXR each write differently and the sources say nothing about any of them); whether an interrupted write leaves nothing, an empty file, or a file of partial length; whether the operating system's own write buffering plays a part; whether a deliberate cancel behaves differently from a killed process or a power cut; whether a file left behind can be told apart from a good one; and anything about video containers, which the question does not ask about. It also does not settle what happens to a placeholder file if Placeholders is switched on and the render stops before that frame is written.

**Outside the interface.** The sources do not say. The write path (`frame_path`) and the handlers above are the same Python API whether Blender is driven by a person or from outside, and the manual documents `-b, --background` ("Run in background (often used for UI-less rendering)") and `-a, --render-anim` ("Render frames from start to end (inclusive)") on the Command Line Arguments page, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30. But no page states what an interrupted write leaves in either case, so there is nothing to compare. Two things are worth noting rather than guessing from. First, the API documentation does flag interface-versus-outside differences where they exist — `RenderSettings.use_single_layer` carries "Only affects rendering from the interface, ignored for rendering from command line" — and no such note appears on anything to do with writing frames. Second, one documented route to stopping a render is interface-only: the status bar's cancel button. No command-line equivalent is documented. **documented** for the quoted lines; the question itself remains **not in the sources**. **Brought up to date by the hand tests of 2026-08-11:** every result above was got from a render driven from outside Blender's window, with no window at all, so the answer *holds* outside the interface — that is where it was measured. The one thing tried inside the window is the Escape key, on the rented Linux machine, and it did not stop the render; what a person pressing Esc or clicking the status bar's cancel button would get is the only part of this answer not established from outside. **Brought up to date again by the second batch of hand tests, 2026-08-12:** the Escape test was run again with a control, and this time the render **stopped**, in all four versions — entry 22 above. The two runs disagree and neither has been chosen. Everything else here is still established from outside the window, and the hand half stays open either way, because no run so far can show that Blender received the key.

**Operating system.** The sources do not say. No page I read states any per-operating-system difference in how a frame file is written or what an interruption leaves. The only operating system named anywhere near this material is in the phrase "similar to Unix ‘touch’", which is describing what the Placeholders setting does, not restricting it to one system. The record of looking above holds no operating-system word among its search terms, so this line rests on the pages read rather than on a search aimed at it. **Brought up to date by the hand tests of 2026-08-11:** everything above was run on macOS 26.5.2 on an Apple M1 Pro **and** on Ubuntu 22.04.5 LTS on an AMD Threadripper, and the two agreed on every point of behaviour. Where they differ is in numbers that are races — the exact bytes a stop leaves, whether a part-written TIFF happens to open, and how often two renders collide on one frame — and none of those is read as an operating-system difference. **Windows is untested**, no Windows machine having run it, so this answer covers macOS and Linux and leaves Windows open. **Brought up to date again by the second batch of hand tests, 2026-08-12:** entry 23 ran the JPEG and TIFF stop ten times per version on each machine, and both machines gave a spread of sizes and a mixture of files that opened and files that did not — the race is now shown directly rather than reasoned about. Entry 24 gave the same seven answers about the compositor set-up in 5.0, 5.1 and 5.2 on the Creator's Mac. Entries 22 and 25 were run on the rented Linux machine only, for the reasons given with them, so what an Escape does inside a window and what a machine going down leaves are answered for Linux and left open for macOS.

---

## 14. When two renders write into one folder and are started at the same instant, what does each of them produce, and what does the folder hold at the end?

**LABEL: not in the sources**, for the question as asked. Nothing states what each render produces, what happens when two are started at the same instant, or what the folder holds at the end. What *is* documented sits around the question rather than answering it: a supported way of pointing several renders at one folder, and what the two settings that make it work are called and do. Each of those carries its own label where it appears below.

**Note on the question.** It is ambiguous in two ways and I answer the reading I think is meant. First, "two renders" — I take it to mean two Blender renders writing frames to the same output folder, whether on two machines or one, since the sources' only treatment of this is the multiple-machine case. Second, the answer depends entirely on two settings the question does not mention, so I cover both the default settings and the configuration the manual recommends. If the intended reading were narrower — two renders of the same scene and frame range with default settings — the answer is the second case below, which is not in the sources.

### What is documented

**a. Blender documents pointing several renders at one shared folder, and calls it a way of sharing the work.** This is the only place in the 5.0 sources that addresses the situation at all.

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

— Manual, Render ‣ Output ‣ Properties ‣ Output, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30. **documented**

**b. What the two settings are.** The manual's own descriptions of them, quoted exactly as they appear, including a labelling oddity in the page: the heading line is repeated verbatim above both descriptions, so the page itself never says which description belongs to which setting. That the second one is Placeholders is my reading of its wording — **inferred** for that step.

> "Image Sequence – Overwrite Image Multi-Layer EXR
>
> Overwrite existing files when rendering.
>
> Image Sequence – Overwrite Image Multi-Layer EXR
>
> Create empty placeholder frames while rendering."

— Manual, Render ‣ Output ‣ Properties ‣ Output, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30. **documented**

**c. Their real names, kind and defaults.** The API documentation is unambiguous where the manual page above is not.

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

— Python API documentation, `bpy.types.RenderSettings`, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30. **documented**

So the shared-folder arrangement is not what happens by default. By default Overwrite is on and Placeholders is off — the opposite of both halves of the manual's recipe.

**d. The panel picture shows what the two settings are called, and how they were ticked when it was taken.** The Output panel screenshot on the same manual page, `_images/render_output_properties_output_panel.png`, shows a checkbox labelled **Overwrite** with a tick in it and a checkbox labelled **Placeholders** without one, side by side above **File Extensions** (ticked) and **Cache Result** (unticked). It also shows the labels the manual's text failed to render: the two settings are plainly called "Overwrite" and "Placeholders". **documented** — Manual, Render ‣ Output ‣ Properties ‣ Output, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30. I take from the picture only what is visible in it: the two labels, and which boxes are ticked in this screenshot. A picture shows a state, not a default — that these ticks are the shipped defaults comes from (c) above, not from here.

**e. Turning Overwrite off makes a render pick up where an earlier one stopped.**

> "Just disable the Overwrite option to start rendering where you left off."

> "Then disable Overwrite and re-render with Frame Step set to 1."

— Manual, Rendering Animations, https://docs.blender.org/manual/en/5.0/render/output/animation.html — read 2026-07-30. **documented**

### The mechanism, as far as it follows

**LABEL: inferred.** The manual states the recipe but never states why it works. Putting (a), (c) and (e) together: with Overwrite off a render does not redo a frame whose file is already there, and with Placeholders on a file appears at a frame's path as soon as that frame is taken up rather than when it is finished — so a second render arriving at that frame finds a file already present and moves on. **The steps a reader has to supply, none of which is written down:** that Overwrite off means the frame is passed over rather than rendered and then not written; that the placeholder is created *before* the frame is rendered rather than after; and that the two renders are therefore dividing the frames between them rather than duplicating them. Because all three have to be supplied, this is inferred, not documented, and I would not rely on it without a hand check.

### What is not in the sources

- **What each render produces.** No page says which frames a given machine ends up with, in what order, or how many.
- **What the folder holds at the end.** No page says whether every frame is present, whether placeholder files remain alongside finished ones, or whether anything needs cleaning up afterwards.
- **Two started at the same instant.** This is the heart of the question and the sources are wholly silent. Nothing says whether the check for an existing file and the creation of the placeholder happen as one indivisible step, so nothing rules out both renders seeing the same frame unclaimed and both rendering it. Nothing describes what happens when two renders write the same file path at once.
- **The default case.** With Overwrite on and Placeholders off — the shipped defaults — the sources say nothing whatever about two renders sharing a folder.
- **Failure.** Nothing about what the folder holds if one of the two renders stops part-way, or what happens to a placeholder it had already put down.

### Where I looked

Terms searched across the 5.0 manual, API documentation and release notes: `placeholder`, `overwrite`, `render farm`, `multiple machines`, `multiple instances`, `another instance`, `second instance`, `two computers`, `each machine`, `shared directory`, `network file system`, `distributed render`, `network render`, `file lock`, `locking`, `race condition`, `concurrent`, `simultaneous`, `at the same time`, `same file`, `same folder`, `same directory`, `conflict`, `already exists`, `already rendered`, `already being rendered`, `existing file`, `skip frames that`.

`shared directory` and `network file system` lead to the one page quoted above and nowhere else. `placeholder` appears in nine files of the 5.0 manual, the other eight being glTF import, empties, geometry-node string formatting, text editing, two contribution pages, linked libraries and sequencer image strips — none about render output. `multiple machines` appears in two: that page and the Cycles Sampling page discussed below. *(Corrected by the citation check of 2026-08-11: the sentence had said all four terms led to the one page and nowhere else.)* `file lock`, `race condition`, `another instance`, `second instance` and `same folder` return nothing at all in the 5.0 manual, API documentation or release notes. I also searched the whole of `Blender Sources/release_notes_5.0` for `placeholder`, `overwrite` and `render farm`: nothing in the 5.0 notes changes, adds or removes either setting, and the only render-farm mention is about logging, quoted below. That is a record of searching the sources on 2026-07-30, not an observation of Blender behaving.

One nearby thing I checked and set aside, so it is not mistaken for an answer: the Cycles Sampling page documents a different multiple-machine scheme, splitting the *samples* of a single frame across machines and merging the results with `bpy.ops.cycles.merge_images` (https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — read 2026-07-30). That scheme has each machine write its own separate file, so it does not describe two renders writing into one folder under the same names.

### What running it settled — the hand tests of 2026-08-11

**LABEL: observed.** The machines, the settings folders and the list are described under question 13 above. Everything quoted is Blender 5.0.1's own output; the two machines are never merged.

**Overwrite off skips a frame before rendering it, not after.** A five-frame render at 512 by 512 with 400 samples, Overwrite off, with a complete PNG placed by hand at frame 3's name beforehand. Each line carries the seconds elapsed since the run started:

> ```
>    3.919  00:03.693  render           | Saved: '…/frame_0001.png'
>    7.353  00:07.128  render           | Saved: '…/frame_0002.png'
>    7.353  00:07.128  render           | Skipping existing frame "…/frame_0003.png"
>   10.777  00:10.552  render           | Saved: '…/frame_0004.png'
> ```

Frame 3 took **no measurable time at all** — the skip is printed at the same thousandth of a second as frame 2's save — where its neighbours took three and a half seconds each. The file placed by hand kept its own size and last-changed time. — Creator's Mac; the rented Linux machine gave `5.956 … Saved: frame_0002.png`, `5.956 … Skipping existing frame "…frame_0003.png"`, `8.724 … Saved: frame_0004.png`. **The two machines agreed.** Note the wording: 5.0, 5.1 and 5.2 print `Skipping existing frame` with a capital S, where 4.5 prints `skipping existing frame`.

**"Already there" means a name exists, and nothing more.** Five things were put at frames 1 to 5's names before a six-frame render with Overwrite off: a complete PNG, a file of zero bytes, nineteen bytes of text that is not an image, a complete PNG of a different size, and a **folder**. All five were skipped and none touched:

> ```
> 00:00.274  render           | Skipping existing frame "…/frame_0001.png"
> 00:00.274  render           | Skipping existing frame "…/frame_0002.png"
> 00:00.274  render           | Skipping existing frame "…/frame_0003.png"
> 00:00.274  render           | Skipping existing frame "…/frame_0004.png"
> 00:00.274  render           | Skipping existing frame "…/frame_0005.png"
> 00:00.363  render           | Saved: '…/frame_0006.png'
> ```

Every one kept its original size and last-changed time. — Creator's Mac and the rented Linux machine, **agreeing line for line**.

**Two renders started together on the shipped defaults both render every frame.** Frames 1 to 8 at 400 by 400 and 200 samples, Overwrite on and Placeholders off, both started by one command. Each printed `Saved:` for all eight. The folder ended with eight files, all opening as 400 by 400 images. — Both machines, agreeing.

**Two renders started together on the manual's recipe divide the frames between them — but not perfectly.** The same file with Overwrite off and Placeholders on. On the rented Linux machine one run reported

> ```
> 00:01.947  render           | Saved: '…/frame_0001.png'
> 00:01.947  render           | Skipping existing frame "…/frame_0002.png"
> 00:03.659  render           | Saved: '…/frame_0003.png'
> 00:03.659  render           | Skipping existing frame "…/frame_0004.png"
> ```

and the other reported

> ```
> 00:01.905  render           | Saved: '…/frame_0001.png'
> 00:03.493  render           | Saved: '…/frame_0002.png'
> 00:03.493  render           | Skipping existing frame "…/frame_0003.png"
> ```

**Both runs saved frame 1.** Eight files were present at the end and all opened. On the Creator's Mac the same run divided the work cleanly with no frame taken twice. That is a race, not a difference between machines, and the next test counts it.

**The placeholder is not a lock, and how often it is raced depends on the file system.** The recipe was run over a single frame, both Blenders started by one command, into a fresh empty folder, **thirty times per version per machine**. Counting how often both runs reported saving frame 1:

- Creator's Mac, local disk: 0 of 30 in 4.5, **0 of 30 in 5.0**, 0 of 30 in 5.1, 2 of 30 in 5.2.
- Rented Linux machine, local disk: **0 of 30 in 5.0**, and 0 of 30 in the other three.
- Rented Linux machine, a folder mounted over NFS from itself: 5 of 30 in 4.5, **6 of 30 in 5.0**, 4 of 30 in 5.1, 4 of 30 in 5.2.

In all 240 runs the resulting file was present and larger than zero bytes. What this shows is that the claim on a frame **can** be lost, and that it is lost far more often over a network file system than on a local disk — which is the arrangement the manual's own hint recommends. — 2026-08-11.

**Two renders over overlapping but different frame ranges.** One run over frames 1 to 10 and one over 6 to 15, started together. On the defaults both wrote the whole of their own range, so frames 6 to 10 were written twice and fifteen files were left. On the recipe one run saved 1 to 5 and skipped 6 to 10 while the other saved 6 to 15 — fifteen files, each written once. — Both machines, agreeing.

**A placeholder left behind by a killed render blocks its frame for good.** A five-frame render on the recipe was killed outright the instant frame 2's file appeared, leaving frame 1 whole and `frame_0002.png` at **0 bytes**. Started again and run to the end, it printed `Skipping existing frame "…/frame_0002.png"` and went on to save frames 3, 4 and 5. Frame 2 stayed at 0 bytes and would not open. With two renders running and one killed part-way, the survivor finished the rest and left the killed one's claimed frame at 0 bytes in the same way. — Both machines, agreeing.

**With a video output the two settings do nothing.** Both renders wrote to one file that Blender named `vid0001-0008.mp4`, both printed `Video append frame 1` through `Video append frame 8`, and the folder ended with a single file that Blender's own movie reader read back as 8 frames — the same on the defaults and on the recipe. — Both machines, agreeing. *(Corrected by the citation check of 2026-08-11: the words quoted here were `Append frame 1` through `Append frame 8`, which is what Blender 4.5 prints. Blender 5.0 prints `00:00.512  render           | Video append frame 1` and so on; 5.1 and 5.2 do the same. Checked in all four versions.)*

**Nothing in the output path identifies the run.** Setting the output path to `{name}` for twenty-one names and asking `frame_path` what came back: in Blender 5.0 nine were substituted — `blend_name`, `blend_dir`, `blend_name_lib`, `blend_dir_lib`, `fps`, `resolution_x`, `resolution_y`, `scene_name` and `camera_name` — and every other name, including `node_name`, `pid`, `process_id`, `hostname`, `machine`, `user`, `run`, `instance`, `uuid`, `random`, `time` and `date`, came back as `Error: Parse errors in path`. Two runs of the same file returned identical paths for every name. **Blender 4.5 substitutes only four of those nine** — `blend_name`, `fps`, `resolution_x`, `resolution_y` — and errors on the other five. None of the nine varies between two runs, so there is no built-in way to keep two runs' file names apart. — Both machines, agreeing.

**The two settings take when they are set from outside Blender's window.** A blend-file saved with Overwrite on and Placeholders off was rendered twice: once as saved, and once with `use_overwrite` set to False and `use_placeholder` to True through Blender's Python before the render, which printed `SET use_overwrite: False | use_placeholder: True`. Run again over their own folders, the first wrote over all four frames while the second reported `Info: No frames rendered, skipped to not overwrite` and `Skipping existing frame` for every one. — Both machines, all four versions, agreeing.

### Not covered

This answer does not settle: the order or division of frames between the two renders; whether the placeholder claim can be raced; what a network file system does or does not guarantee here; whether the two renders are assumed to share a blend-file, scene and frame range; whether placeholder files are removed at the end; what happens when more than two renders are involved; and the case where the output is a video rather than image files, which this arrangement does not appear to be aimed at and which the question does not ask about.

**Outside the interface.** **Brought up to date by the hand tests of 2026-08-11:** the two settings do take when a render is driven from outside Blender's window, and every result under *What running it settled* was measured that way, with no window at all — see the last item there. So this answer holds outside the interface. What follows describes the sources and still stands.

Partly documented. The manual's recipe says "Start as many machines as you wish rendering to that directory" without saying how they are started, and it names the two settings by their panel locations ("in the Render Output panel"), which is an interface description. The Command Line Arguments page documents `-b, --background`, `-a, --render-anim`, `-f, --render-frame`, `-o, --render-output`, `-S, --scene` and a step argument ("Set number of frames to step forward after each rendered frame"), but I searched that whole page for `overwrite` and `placeholder` and that page documents **no command-line switch for either setting**. That they must therefore come from the blend-file or from Python is a step I supply — no page lists every way a setting can be set — so **inferred** for that step. The arguments themselves are **documented** — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30. The search is a record of looking, not an observation of Blender behaving.

One documented trap bears directly on two renders sharing a folder from outside, because it governs where they write at all:

> "blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set."

> "blender --background test.blend --render-output /tmp --render-frame 1
>
> …works as expected."

— Manual, Command Line Arguments, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30. **documented**

Also documented, and relevant to anything watching two background renders from outside: the 5.0 release notes state that the shape of what a background render prints has changed.

> "Most of Blender now uses the same logging system, including operators reports and background render progress. The output format has changed, and log levels are easier to control."

— Release notes, Core, https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30. **documented**

> "The format of logging output has changed, including background render progress. Add-ons or render farms parsing this output may need to be updated."

— Release notes, Python API, https://developer.blender.org/docs/release_notes/5.0/python_api/ — read 2026-07-30. **documented**

**Operating system.** The sources do not say. The recipe requires "a shared directory over a network file system" but names no operating system and states no per-system difference. The only operating system mentioned anywhere in this material is in the phrase "similar to Unix ‘touch’", which describes what Placeholders does rather than limiting it to one system. Whether the behaviour is the same on every system Blender runs on is not addressed. The record of looking above holds no operating-system word among its search terms, so this line rests on the pages read rather than on a search aimed at it. **Brought up to date by the hand tests of 2026-08-11:** everything above was run on macOS 26.5.2 on an Apple M1 Pro **and** on Ubuntu 22.04.5 LTS on an AMD Threadripper, and the two agreed on every point of behaviour. Where they differ is in numbers that are races — the exact bytes a stop leaves, whether a part-written TIFF happens to open, and how often two renders collide on one frame — and none of those is read as an operating-system difference. **Windows is untested**, no Windows machine having run it, so this answer covers macOS and Linux and leaves Windows open.

---

## Things a person with Blender open could settle by hand in a few minutes

**Pruned on 2026-08-11, and pruned again on 2026-08-12** after the second batch of hand tests. The ten items that stood here were all carried out, in all four researched versions, on the Creator's Mac and on a rented Linux machine; what came back is written into questions 13 and 14 above as observed findings, and the list they were run from is `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`. What is left is what a running Blender on either of those two machines could not reach.

1. **Two renders on two separate computers writing into one shared folder** — the arrangement the manual's own render-farm hint describes. The project's two machines have no file system shared between them, and sharing one would mean installing software on the Creator's working machine. A network file system *was* tested, with both renders on the rented Linux machine writing to a folder mounted over NFS from itself, and it changed the answer — see question 14 — so the two-computer case is worth taking up whenever a shared folder exists.
2. **Anything on Windows.** No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
3. **A real power cut, as against a kernel reset.** The reset was done twice, on 2026-08-11 and again on 2026-08-12, and the two runs do not agree about which directory entries survive. Both results are under question 13. Pulling a plug is still out of reach on either machine.
4. **Whether Blender receives an Escape key sent by a tool, and so what a person pressing Esc by hand would get.** Whether the render *stops* has been run three times on the rented Linux machine and has come out both ways — see question 13, where all three runs stand. No run of this shape can show whether the key reached Blender at all, so the hand half stays out of reach. Nothing in this round is held back for a person, so this is recorded as unreached rather than passed on.
5. **What a person clicking the status bar's cancel button leaves behind.** That control needs a hand on a mouse.

The ten items as the researcher wrote them are kept below, struck through by the results above rather than deleted, since each names the exact set-up that was used.

These are for question 13 and question 14. Each says exactly what to do and what to look at.

**For question 13**

1. **What an interrupted write leaves.** Open Blender 5.0. In Output Properties set the file path to an empty folder, Media Type to Image, File Format to PNG. Set the resolution high (say 7680 × 4320) so the write itself takes a visible moment. Set the frame range to 1–5. Press Ctrl-F12 to render the animation. Watch the status bar, and the moment the progress text shows it is saving a frame, click the cancel button in the status bar. Then look in the folder with a file manager and note, for each file present: its name, its size in bytes, and whether an image viewer will open it. The frame that was mid-write is the one to look at.

2. **The same with the process killed instead.** Repeat step 1, but instead of clicking cancel, kill the Blender process outright while it is saving (Force Quit on macOS, End Task on Windows, `kill -9` on Linux). Record the same three things for the frame that was in flight. Compare against step 1 — this is the difference the sources do not describe.

3. **The same for OpenEXR and for multi-layer EXR.** Repeat step 1 with File Format set to OpenEXR, then with Media Type set to Multi-Layer EXR. Record file sizes and whether each file opens. Formats write differently and the answer may not be the same for all three.

4. **Whether a temporary name is used.** Repeat step 1, but before cancelling, while a frame is being written, list the output folder and note every file name in it, including hidden files. If a name appears that is not the final frame name — anything with a suffix, a dot prefix, or a random-looking name — Blender is writing to a temporary file first. If only the final frame names ever appear, it is writing in place.

5. **From outside the interface.** Run `blender -b scene.blend -o /path/to/empty/folder/frame_ -a` from a terminal with the same high resolution, and press Ctrl-C while it is saving a frame. Record the same three things. Then repeat, killing the process instead. This is the case that matters most and the sources do not cover it at all.

**For question 14**

6. **Two renders, default settings, same instant.** Take one blend-file with a frame range of 1–20 and a short per-frame render time, output to a shared empty folder as PNG, leaving Overwrite ticked and Placeholders unticked. Start two Blender background renders of it at the same moment (in a terminal: `blender -b scene.blend -a & blender -b scene.blend -a &`). When both finish, record: how many frames each render reported writing, how many files are in the folder, and whether any file is unreadable or of an odd size. This is the exact default-settings case the sources are silent on.

7. **Two renders, the manual's recipe, same instant.** Same as step 6, but first untick Overwrite and tick Placeholders in Output Properties and save the blend-file. Start both at the same moment. Then record: which frame numbers each render reported working on; whether any frame number appears in both lists (that would show the claim can be raced); how many files the folder holds at the end; whether any file is zero bytes (a placeholder never filled in); and whether all 20 frames are present and readable.

8. **When the placeholder appears.** With Placeholders ticked and Overwrite unticked, start a single render of a slow frame and, while it is still rendering that frame, list the output folder. If a zero-byte file for that frame is already there, the placeholder is created before the frame is rendered — which is the step the mechanism depends on and the documentation does not state.

9. **What a stopped render leaves in the shared folder.** Run step 7, then kill one of the two renders part-way. Let the other run to the end. Record whether the folder ends up with every frame, and whether any zero-byte placeholder is left behind for a frame the killed render had claimed but never wrote.

10. **Across two machines and a network folder.** Repeat step 7 with the two renders on two different computers writing to the same network folder, ideally two different operating systems. Record the same things. A network file system can behave differently from a local disk, and the manual's recipe is written for exactly this case.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Frames, Blender 5.0

2026-08-12 — Claude Opus 5 (Claude Code) — the second batch of hand tests written in: entries 22, 23, 24 and 25 run and recorded under question 13, both standing lines brought up to date, and the researcher's own list pruned again.
