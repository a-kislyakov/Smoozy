# Frames — cross-sweep questions, second pass (Blender 4.5)

Questions answered: 13 and 14 only.

**Sources consulted, and when.** Blender's 4.5 manual (`Blender Sources/manual_4.5`, including its `_images` folder), Blender's 4.5 Python API documentation (`Blender Sources/api_4.5`), the Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`) — all read as of the snapshot date **2026-07-30**. I also read `Blender Sources/requirements/requirements_2026-08-01.txt`, the requirements page, which names Blender 4.5 in so many words; date read **2026-08-01**.

`I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.`

I ran no helper agents. Everything below was searched and read by me.

---

## How I searched (applies to both answers)

I searched the whole of `manual_4.5`, `api_4.5` and `release_notes_4.5` at once, case-insensitively, before opening anything.

Terms searched across all three, with the number of hits found:

- `placeholder` — 19 hits in the manual, of which only two are about render output; `use_placeholder` / `use_overwrite` in the API
- `overwrite` — many hits; only three are about render output
- `partial`, `incomplete`, `truncat`, `corrupt`, `unfinished`, `half-written` — no hit about a render output file
- `atomic` — 0 hits anywhere
- `fsync` — 0. `journal` — 0. `race condition` — 0. `mutex` — 0. `semaphore` — 0
- `partial file` — 0. `incomplete file` — 0. `unreadable` — 0. `invalid file` — 0. `zero-byte` — 0. `0 bytes` — 0
- `temporary file`, `temp file`, `.tmp` — hits only about Blender's own temp directory and auto-save, never about render output
- `lock file`, `file lock` — 0 relevant hits
- `simultaneous`, `concurrent`, `at the same time`, `multiple instances`, `same machine`, `same frame` — no hit about two renders sharing an output folder
- `render farm`, `farm`, `network file system`, `shared directory`, `distributed` — the only relevant hit is the "Primitive Render Farm" hint quoted under question 14
- `cancel`, `abort`, `interrupt`, `Esc`, `kill`, `terminate`, `quit`, `SIGTERM`, `Ctrl-C` — nothing describing what an interrupted render leaves on disk
- `rename`, `in-place`, `safe write`, `clobber`, `collision`, `conflict`, `duplicate work`, `stale`, `resume`, `already rendered`, `existing file`, `empty file`, `touch`, `while rendering`, `written to disk`, `saved to disk`, `flush` — the only relevant hits are the ones quoted below

Pages I opened in full and read:

- `render/output/properties/output.txt` → https://docs.blender.org/manual/en/4.5/render/output/properties/output.html
- `render/output/animation.txt` → https://docs.blender.org/manual/en/4.5/render/output/animation.html
- `render/output/introduction.txt` → https://docs.blender.org/manual/en/4.5/render/output/introduction.html
- `render/introduction.txt` → https://docs.blender.org/manual/en/4.5/render/introduction.html
- `files/file_paths.txt` (path templates) → https://docs.blender.org/manual/en/4.5/files/file_paths.html
- `advanced/command_line/arguments.txt`, the Render Options section → https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- `interface/window_system/topbar.txt`, the Render Menu section → https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html
- `compositing/types/output/file_output.txt` → https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html
- `files/media/image_formats.txt`, the Saving Images section → https://docs.blender.org/manual/en/4.5/files/media/image_formats.html
- `render/cycles/render_settings/performance.txt`, the Memory section → https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html
- API: `bpy.types.RenderSettings` → https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- API: `bpy.app.handlers` → https://docs.blender.org/api/4.5/bpy.app.handlers.html
- API: `bpy.ops.render` → https://docs.blender.org/api/4.5/bpy.ops.render.html
- Release notes: `rendering.md`, `pipeline_assets_io.md`, and the Render & Cycles section of `bugfixes.md` → https://developer.blender.org/docs/release_notes/4.5/rendering/ , https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/ , https://developer.blender.org/docs/release_notes/4.5/bugfixes/

Picture I opened: `manual_4.5/_images/render_output_properties_output_panel.png`, the Output panel screenshot on https://docs.blender.org/manual/en/4.5/render/output/properties/output.html

Cycles' own files: I listed the whole of `cycles_4.5` (399 files) and searched it. What is there is the Python add-on layer (`__init__.py`, `engine.py`, `properties.py`, `ui.py`, `operators.py`, `camera.py`, `osl.py`, `presets.py`, `version_update.py`), 104 compiled OSL shaders, and 262 files under `source/kernel` and `source/util`, 253 of them headers. *(Corrected by the citation check of 2026-08-11: the count 262 is of files, not of headers.)* I searched those files for `overwrite`, `placeholder`, `filepath`, `fopen`, `ofstream`, `write_file`, `output_driver`, `image_write`, `cancel` and `interrupt`. **Ruled out:** none of the supplied Cycles files writes a rendered frame to disk — the only file-writing terms found are a comment in `source/kernel/device/hiprt/common.h` about mapping primitive ids, and `operators.py`'s denoise-an-animation operator, which reads and writes files a user names. Frame output is done by Blender itself, and Blender's own code is not in what I was given.

I did not use the web. I did not need it.

---

## 13. When a render is stopped at the exact moment it is writing a frame's own image file, what is left where it writes?

**LABEL: not in the sources.**

Nothing in the 4.5 manual, the 4.5 Python API documentation, the 4.5 release notes, or Cycles' own files says what sits at the output path if a render is stopped part-way through writing a frame's image file. The sources do not say whether the file is left truncated, left empty, left absent, or left whole. They do not say whether Blender writes under a temporary name and renames on success, or writes straight to the final name.

The question was slightly ambiguous and I answered one reading. **The reading I took:** "stopped" means the render run ends at that instant by any means (a person cancelling it, or the process ending), and "where it writes" means the output path from the Output panel — what the manual calls the File Path. **The other reading** would be that "stopped" means only a deliberate cancel from inside Blender, leaving a machine crash out of scope. Neither reading is covered, so the answer does not change.

What the sources *do* say, and what they stop short of:

Writing a frame is a step that happens after the frame is rendered:

> "animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."
> — manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30

There is a distinct moment at which a frame counts as written, and Blender exposes it:

> "on writing a render frame (directly after the frame is written)"
> — API, `bpy.app.handlers.render_write`, https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30

Cancelling is a separate, named event from completing:

> "on canceling a render job. Accepts one argument: the scene datablock being rendered"
> — API, `bpy.app.handlers.render_cancel`, https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30

> "on completion of render job. Accepts one argument: the scene datablock being rendered"
> — API, `bpy.app.handlers.render_complete`, https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30

Saving is timed separately from rendering:

> "on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."
> — API, `bpy.app.handlers.render_stats`, https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30

Taken together these say only that saving a frame is its own step with a start and an end, and that a cancel can arrive. None of them says what the file looks like if the cancel lands inside that step.

One thing the sources *do* settle about what can sit at the output path during a render, though it is a different thing from a half-written image. Blender can be told to put an empty file at the frame's path:

> "Placeholders
>
> Create empty placeholder frames while rendering."
> — manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30

> "Create empty placeholder files while rendering frames (similar to Unix 'touch')"
> — API, `bpy.types.RenderSettings.use_placeholder`, type "boolean, default False", https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30

So with Placeholders switched on there is a documented case in which an empty file bearing the frame's name exists at the output path while the frame is being worked on. The sources do not say at what instant the placeholder is created relative to rendering and saving, and do not say what becomes of it if the render stops.

I also checked the picture. The Output panel screenshot on the same manual page shows the Overwrite and Placeholders checkboxes side by side under the panel, with Overwrite ticked and Placeholders unticked. **documented** (from the picture: `_images/render_output_properties_output_panel.png`, on https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30). Nothing about interruption is visible in it.

One Cycles finding, and I flag it because it is adjacent rather than an answer. Cycles can put intermediate render data on disk somewhere while a frame is still being rendered — this is not the frame's own image file, and the sources do not say where it goes:

> `description="Render high resolution images in tiles to reduce memory usage, using the specified tile size. Tiles are cached to disk while rendering to save memory",`
> — **observed**, Cycles' own files, `cycles_4.5/properties.py`, the `use_auto_tile` property, lines 1051–1054; `default=True`. Read 2026-07-30.

The manual repeats this without saying where the cache lives:

> "Tiles are cached to disk while rendering to save memory."
> — manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html — read 2026-07-30

I ruled out the following as places the answer might have been, having searched or read them: the Saving Images section of the image formats page (describes format and colour options only, nothing about how a file is written); the File Output node page (a separate writer, and it says nothing about interruption); the Recovering Data and Crashes troubleshooting pages (about `.blend` files and auto-save, not render output); the Render Menu section of the topbar page (lists Render Image and Render Animation, says nothing about stopping); the command line arguments page (documents `-b`, `-a`, `-f`, `-o`, `-s`, `-e`, `-j`, `-F`, `-x`, and nothing about interruption or overwriting); and the 4.5 release notes, whose Render Output section covers only pixel-density metadata, BMP alpha, and FFV1 bit depths.

### What running it settled — the hand tests of 2026-08-11

**LABEL: observed.** Twenty-one tests, built from all eight of this round's findings documents and the four audits and saved at `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, were run in Blender 4.5.12, 5.0.1, 5.1.2 and 5.2.0 on two machines. **Machine one, the Creator's Mac:** a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card. **Machine two, a rented Linux machine:** Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Every run had a settings folder of its own and was started with `--factory-startup`. Everything quoted is Blender 4.5.12's own output; the two machines are never merged. The same results are written into this version's first-pass document, in fuller form.

**Blender writes straight to the final name, and no temporary name is ever used.** Three frames at 4096 by 4096 as OpenEXR with no compression, the whole folder listed every two milliseconds. The only names that ever appeared were the three final frame names — nothing hidden, nothing ending `.tmp` or `.part`:

> ```
> WATCH    0.000  <empty>
> WATCH    1.285  frame_0001.exr=1635102
> WATCH    1.288  frame_0001.exr=3941150
> ```

and 112 distinct sizes later the file reached 268,501,790 bytes. — Creator's Mac; the rented Linux machine did the same and ended at 268,501,751 bytes. **The two machines agreed.**

**How the stop is delivered decides what is left, and the three ways differ.** Each run was stopped the instant frame 2's file first appeared. An **interrupt** did not cut the write: Blender finished the frame — `Sent an internal break event. Press ^C again to kill Blender`, `Saved: '…/frame_0002.exr'`, `Time: 00:01.12 (Saving: 00:00.33)`, exit code 1 — and the file was whole and opened. A **terminate** left 0 bytes on both machines, exit code -15, no `Saved:` line, unopenable. A **kill** left 25,379 bytes on the Creator's Mac and 32,772 on the rented Linux machine, exit code -9, unopenable. So a file **does** exist at the frame's own path after a stop that is not an interrupt, and it is short.

**The format matters, and a part-written file is not always distinguishable from a whole one.** The same kill per format at 4096 by 4096: PNG left 0 bytes on both machines; JPEG 0 on the Mac and 4,096 on Linux — and **the Linux one opened, reported as 4096 × 4096**; OpenEXR 0 and 16,384; TIFF 1,306 on both, **and it opened on both**, against a complete TIFF of 5,534,881 bytes. Multi-layer OpenEXR with two view layers wrote one file per frame, 536,937,733 bytes complete, 17,669 after a kill; for that one format Blender's own image loader reports no data for the **complete** file too, so the reader used here cannot tell whole from part — a limit of the test.

**Nothing was still being held back by the operating system.** After a kill the folder was listed at once and again sixty seconds later; the two listings were identical, on both machines and in all four versions.

**The placeholder is created before the frame is rendered, one frame at a time.** With Overwrite off and Placeholders on: `WATCH 0.288 frame_0001.exr=0`, and the first real bytes only at 1.278 seconds; frame 2's file did not appear until frame 1 was finished. That settles the inference this document draws. Stopped once frame 2's file had grown past zero, the leftover was 78,623 bytes on the Creator's Mac and 36,864 on the rented Linux machine, neither opening. **The two machines agreed.**

**The JPEG preview copy exists and is written after each frame.** With `use_preview` on and OpenEXR output the folder held a `.jpg` beside every `.exr`, and the run printed the EXR's save immediately followed by the JPEG's.

**A Compositor File Output node writes before the render's own frame file.** Stopped the instant the node's own second file appeared, the folder held that file at 610 bytes on the Creator's Mac and 0 bytes on the rented Linux machine, unopenable, and **no Output-panel file at all for that frame**.

**Cancelling from inside Blender's own window did not stop the render.** On the rented Linux machine only: a forty-frame render in a real window with the render window holding the keyboard, Escape sent to every Blender window repeatedly for ten seconds with `xdotool`. Fifteen seconds later the render was still going and went on writing whole frames, in all four versions. This shows the render did not stop; it cannot show Blender received the key.

**The citation check of 2026-08-11 repeated this and got the opposite, and both runs are recorded rather than one replacing the other.** Following the procedure written above on the rented Linux machine, the render **stopped**: sixteen frames stood in the folder when the first Escape was sent and sixteen stood there fifteen seconds after the last one, in Blender 4.5, run twice; in 5.2 it was fourteen and fourteen. A control identical in every respect except that no Escape was ever sent went from sixteen frames at twenty-five seconds to thirty-six at fifty seconds in 4.5, and from fourteen to thirty-three in 5.2 — so without the Escape the render kept going, and with it the render stopped. That is a question, not an answer. It is item 22 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`, to be run again, and until it is run nothing downstream should treat either result as the fact.

**A machine going down loses even frames it had already reported saving.** On the rented Linux machine only, the machine was reset at the kernel — `echo b > /proc/sysrq-trigger`, nothing flushed — the instant frame 2's file appeared. Afterwards the output folder was **empty**: no frame 1, though it had been reported saved seconds earlier, and no frame 2; the run's log file and the blend-file it had built were both 0 bytes. A plain `reboot` had been tried first and the machine came back from it. This is the *unflushed write* case, and it comes out differently from every kill above. *(The citation check of 2026-08-11 did not repeat this: the kernel reset was refused by the permission system of the machine that check ran on. The result stands on its single run, and is item 25 on the hand-tests list.)*

### What the second batch of hand tests settled — 2026-08-12

**LABEL: observed.** Four more tests — entries 22 to 25 on `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md` — were added by the citation check of 2026-08-11: three because a repeat came back differently from a run recorded above, and one because a procedure could not be followed as written. They were run on 2026-08-12 on the machines each entry names, on the same two machines described above, each run with a settings folder of its own and `--factory-startup`. The two machines are never merged, and the same results are in this version's first-pass document, in fuller form. **Where a result differs from one written above, both stand and neither has been chosen.**

**Escape sent to Blender's own window stopped the render — entry 22.** On the rented Linux machine only, that being the one machine with a screen this round could drive. A forty-frame render at 480 by 480 with eighty samples, PNG, started in a real window from a timer. At twenty-five seconds the folder held 30 files, `xdotool getwindowfocus getwindowname` answered `Blender Render`, and `xdotool search --name Blender` answered `62914562 62914691`. Escape was sent to both windows and to the focused window twenty-six times over ten seconds; the folder held 30 files ten seconds later and 30 fifteen seconds after that. A control in the same sitting with no Escape sent went 31, then 40, then 40. Blender 4.5 was tested twice, 30, 30 and 30 both times. The other three versions came out the same way — 5.0 froze at 35, 5.1 at 38, 5.2 at 33, while each control reached all 40 frames of the range. **This agrees with the citation check's repeat and disagrees with the run recorded above. Both stand.** It still cannot show that Blender received the key.

**A stop leaves a different number of bytes almost every time, for JPEG and for TIFF — entry 23.** Ten repeats per format per version per machine: the three-frame render at 4096 by 4096 was killed the instant frame 2's file appeared, and its size and whether it opened were recorded. In Blender 4.5 on the **Creator's Mac**, the ten JPEG leftovers were 8,192, 0, 0, 0, 4,096, 0, 4,096, 4,096, 4,096 and 0 bytes, five of the ten opening; the ten TIFF leftovers were 1,286, 1,306, 1,306, 0, 1,306, 8, 0, 1,278, 254 and 8 bytes, three opening. On the **rented Linux machine** the ten JPEG leftovers were 0, 0, 0, 4,096, 0, 4,096, 0, 4,096, 0 and 4,096 bytes, four opening; the ten TIFF leftovers were 1,306, 8, 8, 8, 1,306, 8, 1,306, 1,306, 1,306 and 1,306 bytes, six opening. Complete frames were 4,128,006 and 5,534,881 bytes on the Mac and 4,113,528 and 5,534,204 on the rented machine. **Every cell is a spread**, so the single figures written above, and the different single figures the citation check got, are draws from a race rather than facts about a version or a machine.

**Entry 24 was not run in Blender 4.5.** It covers the Compositor File Output node in 5.0, 5.1 and 5.2 only, because the citation check repeated the 4.5 result exactly.

**The kernel reset, run a second time, left the folder present and empty in every version — entry 25.** On the rented Linux machine only, run last. A plain `reboot` was tried first and the machine came back. Once per version, the three-frame OpenEXR render was started with no window and the machine reset at the kernel the instant frame 2's file appeared. In Blender 4.5 the output folder came back present and `<empty>`, the run's own log file did not exist at all, and the blend-file survived whole at 435,463 bytes. The folder was present and empty in 5.0, 5.1 and 5.2 as well. **That disagrees with the run recorded above**, which had the folder gone in 5.0, empty in 4.5 and 5.1, holding a zero-byte `frame_0001.exr` in 5.2, and the blend-file at 0 bytes. Both stand. What came out the same way twice: no frame's contents survived in any version, including a frame already reported saved.

**Not covered.** This answer does not settle: whether a stopped render leaves a truncated file, an empty file, or no file at all; whether the file format matters (a single-layer PNG and a multi-layer OpenEXR are written very differently and the sources distinguish neither); whether a placeholder left behind is later distinguishable from a finished frame; whether Blender writes via a temporary name; whether the answer differs between a cancel from inside Blender and the process being killed outright; and where Cycles' tile cache is written.

**Outside the interface.** The sources do not say. The command line arguments page documents running Blender with `-b, --background` — "Run in background (often used for UI-less rendering)" (manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30) — and documents `-a, --render-anim` and `-f, --render-frame`, but says nothing about interrupting a render or about what is left on disk. Nothing I found distinguishes the two cases. **Brought up to date by the hand tests of 2026-08-11:** every result under *What running it settled* was got from a render driven from outside Blender's window, with no window at all, so the answer holds outside the interface — that is where it was measured. Sending Escape to Blender's own window on the rented Linux machine did not stop the render, so what a person pressing Esc or clicking the status bar's cancel button would get is the only part of this answer not established from outside. **Brought up to date again by the second batch of hand tests, 2026-08-12:** the Escape test was run again with a control and this time the render **stopped**, in all four versions — entry 22 above. The two runs disagree and neither has been chosen. Everything else here is still established from outside the window, and the hand half stays open either way, because no run so far can show that Blender received the key.

**Operating system.** The sources do not say. The requirements page states Blender runs on Windows, macOS and Linux, and names 4.5 explicitly: "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur)." (requirements page, `Blender Sources/requirements/requirements_2026-08-01.txt`, read 2026-08-01). No source ties the behaviour of a half-written render file to any operating system or file system. The API's phrase "similar to Unix 'touch'" is an explanation of what a placeholder is, not a claim that the feature is limited to Unix. **Brought up to date by the hand tests of 2026-08-11:** everything above was run on macOS 26.5.2 on an Apple M1 Pro **and** on Ubuntu 22.04.5 LTS on an AMD Threadripper, and the two agreed on every point of behaviour; where they differ is in numbers that are races — the exact bytes a stop leaves, and whether a part-written JPEG or TIFF happens to reach a size that opens. **Windows is untested**, no Windows machine having run it, so this answer covers macOS and Linux and leaves Windows open. **Brought up to date again by the second batch of hand tests, 2026-08-12:** entry 23 ran that stop ten times per format per version on each machine, and both machines gave a spread of sizes and a mixture of files that opened and files that did not, which shows the race directly rather than by inference. Entries 22 and 25 were run on the rented Linux machine only, so what an Escape does inside a window and what a machine going down leaves are answered for Linux and left open for macOS.

---

## 14. When two renders write into one folder and are started at the same instant, what does each of them produce, and what does the folder hold at the end?

**LABEL: not in the sources**, for what the question actually asks. The sources document that Blender is *meant* to be used this way and which two settings make it work, but they never state what each render produces, what the folder holds at the end, or what happens when two renders start at the same instant.

The question was ambiguous and I answered one reading. **The reading I took:** two separate render runs — two Blender processes — with the same output folder, started simultaneously. **Other readings** would be two render runs one after another in the same Blender, or one render writing through two different writers at once. The sources are silent on all of them.

What is documented:

Blender's manual gives a recipe for exactly this arrangement, in a Hint box on the Output panel page:

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."
> — manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30

The two settings it names, in full:

> "Overwrite
>
> Overwrite existing files when rendering.
>
> Placeholders
>
> Create empty placeholder frames while rendering."
> — manual, same page — read 2026-07-30

> "use_overwrite — Overwrite existing files while rendering — Type: boolean, default True"
> — API, `bpy.types.RenderSettings`, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30

> "use_placeholder — Create empty placeholder files while rendering frames (similar to Unix 'touch') — Type: boolean, default False"
> — API, same page — read 2026-07-30

The Output panel screenshot confirms these two are checkboxes shown together, Overwrite ticked and Placeholders unticked. **documented** (picture `_images/render_output_properties_output_panel.png`, on https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30). In the manual's text they sit under a heading "Image Sequence", which is the manual's own grouping for these two options.

Turning Overwrite off is what makes a second run skip work already on disk:

> "Just disable the Overwrite option to start rendering where you left off."
> — manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30

The 4.5 release notes contain one entry touching placeholders, in the Render & Cycles list of bugs fixed in 4.5:

> "Placeholder output files wrong name"
> — release notes, https://developer.blender.org/docs/release_notes/4.5/bugfixes/ — read 2026-07-30

The page introducing that list says: "There are a total of 494 commits in 4.5 that fixed issues that existed in Blender 4.4 or older." (same address, read 2026-07-30). The entry gives no detail beyond its title.

**One inference, with the step written out.** Both renders compute their output file names the same way, so with the same settings they aim at the same names. The step: the manual says the output file name is built from the File Path with the frame number appended, or substituted where `#` characters appear — "When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of # anywhere in the file name" (manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30) — and the API exposes that same calculation as "Return the absolute path to the filename to be written for a given frame" (API, `bpy.types.RenderSettings.frame_path`, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30). Nothing in that calculation depends on which process or which machine is running. So two renders with the same output path and the same frame range target identical file names. **LABEL for this piece: inferred.** What each file ends up containing is not stated anywhere, and I will not guess.

Related and worth recording, because it bears on whether Blender offers any built-in way for two concurrent renders to write to *different* names: 4.5's path templates let the output path contain substituted values, and the manual lists every variable available:

> "The following variables are currently available in template expressions:
>
> blend_name: The current blend file's name (without the .blend).
>
> fps: The frames per second of the current scene.
>
> resolution_x / resolution_y: The x and y resolution of the rendered image."
> — manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html — read 2026-07-30

None of those varies between two processes rendering the same file, and the same page says "The set of available variables will expand over time." **documented.**

What the sources never say, and I searched hard for it:

- What each of the two renders produces. Not stated.
- What the folder holds when both have finished. Not stated.
- What happens when both start at the same instant — whether two renders can both decide to take the same frame before either has claimed it, and whether creating a placeholder is an exclusive act that one process wins and the other loses. Not stated. The words `atomic`, `race condition`, `lock file`, `file lock`, `mutex` and `semaphore` return zero hits across the manual, the API documentation and the release notes.
- Whether the recipe works on a local folder as well as "a shared directory over a network file system" — the Hint names only the network case.
- What happens with the shipped defaults, Overwrite on and Placeholders off, which is the case a person hits if they simply start two renders. Not stated.
- Whether the placeholder is removed, replaced, or overwritten when the frame it stands for is finished.

I ruled these places out after reading them: the command line arguments page has no `--overwrite` or `--placeholder` switch and says nothing about two Blenders sharing a folder; the metadata page mentions render farms only in passing ("It can be useful to use the Note field if you are setting up a render farm.", manual, https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html, read 2026-07-30) and settles nothing here; the File Output node page describes a second, separate writer and says nothing about collisions; the 4.5 release notes' Rendering and Pipeline & I/O pages contain nothing on this; and Cycles' own files contain no output-path or file-writing code at all.

### What running it settled — the hand tests of 2026-08-11

**LABEL: observed.** The machines and the list are described under question 13 above. Everything quoted is Blender 4.5.12's own output; the two machines are never merged. This is where this document's refusal to state a mechanism is settled by observation rather than reasoning, and the mechanism turns out to be the one the first-pass document guessed — with one important qualification, in the fifth item.

**Overwrite off skips a frame before rendering it.** A five-frame render, Overwrite off, with a complete PNG placed by hand at frame 3's name. The seconds elapsed run down the left:

> ```
>    9.619  Saved: '…/frame_0002.png'
>    9.620  skipping existing frame "…/frame_0003.png"
>   14.130  Saved: '…/frame_0004.png'
> ```

Frame 3 cost one thousandth of a second where its neighbours cost four and a half seconds each, and the file placed by hand was untouched. — Creator's Mac and the rented Linux machine, agreeing. Blender 4.5 writes `skipping existing frame` in lower case; 5.0, 5.1 and 5.2 write `Skipping existing frame`.

**"Already there" means a name exists, whatever is behind it.** A complete PNG, a zero-byte file, nineteen bytes of text, a PNG of the wrong size and a **folder** were all put at frames 1 to 5's names; all five were skipped and none touched, and only frame 6 was rendered. — Both machines, agreeing.

**On the shipped defaults both renders render every frame.** Two renders of frames 1 to 8 started by one command: each printed `Saved:` for all eight, and the folder ended with eight readable files. — Both machines, agreeing.

**On the manual's recipe the two renders divide the frames between them.** One reported saving 2, 4, 6, 8 and skipping 1, 3, 5, 7; the other reported the exact complement. Eight files, each written once, all readable. — Both machines, agreeing.

**The placeholder is not a lock.** The recipe was run over a single frame, both Blenders started by one command, into a fresh empty folder, **thirty times per version per machine**. Both runs saved frame 1: 0 of 30 in 4.5, 5.0 and 5.1 on the Creator's Mac and 2 of 30 in 5.2; 0 of 30 in all four on the rented Linux machine's own disk; and **5 of 30 in 4.5, 6 of 30 in 5.0, 4 of 30 in 5.1 and 4 of 30 in 5.2 over a folder mounted from that machine over NFS**. In all 240 runs the file was present and larger than zero bytes. So the claim on a frame can be lost, and it is lost far more often over a network file system than on a local disk — which is the arrangement the manual's hint recommends.

**Two renders over overlapping but different ranges.** One over frames 1 to 10 and one over 6 to 15, started together. On the defaults each wrote its whole range, so the overlapping five were written twice; on the recipe one saved 1 to 5 and skipped 6 to 10 while the other saved 6 to 15, fifteen files each written once. — Both machines, agreeing.

**A placeholder left by a killed render blocks its frame for good.** Killed the instant frame 2's file appeared, the folder held frame 1 whole and frame 2 at 0 bytes. Run again to the end, the render printed `skipping existing frame "…/frame_0002.png"` and left frame 2 at 0 bytes, unopenable. With two renders and one killed part-way, the survivor finished the rest and left the killed one's claimed frame at 0 bytes. — Both machines, agreeing.

**With a video output the two settings do nothing.** Both renders wrote to one file named `vid0001-0008.mp4`, both printed `Append frame 1` through `Append frame 8`, and one file was left that read back as 8 frames — the same on the defaults and on the recipe. — Both machines, agreeing.

**No path template identifies the run.** Of twenty-one names tried in the output path, Blender 4.5 substituted only `blend_name`, `fps`, `resolution_x` and `resolution_y`; every other name, including `pid`, `hostname`, `uuid` and `random`, came back as `Error: Parse errors in path`, and two runs of the same file returned identical paths for every name. The four variables this document quotes from the manual are the whole of it in 4.5; 5.0, 5.1 and 5.2 add five more, none of which varies between runs either. — Both machines, agreeing.

**The two settings take from outside the window.** Set through Blender's Python before the render — `SET use_overwrite: False | use_placeholder: True` — the second of two runs later reported `Info: No frames rendered, skipped to not overwrite` where the first wrote over everything. — Both machines, all four versions, agreeing.

**Not covered.** This answer does not settle: what either render produces; what the folder holds at the end; whether the placeholder mechanism is safe against two processes claiming the same frame at the same moment; whether the answer differs on a local disk versus a network share; what the defaults do; whether the two renders are on one machine or two; or whether an interrupted one of the two leaves anything behind — that last is question 13's territory and is equally uncovered.

**Outside the interface.** The sources do not say. Both settings are properties of the scene's render settings, `bpy.types.RenderSettings.use_overwrite` and `use_placeholder` (API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30), rather than interface state, and the manual's own instruction is to set them "in the Render Output panel" — that is, from inside Blender's window, before the machines are started. The manual's Hint says only "Start as many machines as you wish rendering to that directory" and does not say whether those machines run Blender with a window or in background mode. The command line arguments page offers no switch for either setting. **inferred, and the step is this:** because the two settings live in the scene's render settings and are therefore saved in the blend-file, a render started from the command line uses whatever the blend-file carries. What that inference does *not* reach is whether the behaviour itself is the same outside the window, which no source states. **Brought up to date by the hand tests of 2026-08-11:** the inference is confirmed and the gap is closed. Both settings were changed through Blender's Python from outside the window, before the render, and took effect — see the last item under *What running it settled*. Every result there was measured with no window at all, so this answer holds outside the interface.

**Operating system.** The sources do not say. The requirements page confirms Blender runs on Windows, macOS and Linux, and names 4.5 in so many words: "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur)." (requirements page, `Blender Sources/requirements/requirements_2026-08-01.txt`, read 2026-08-01). Nothing ties the Overwrite or Placeholders behaviour, or the outcome of two renders sharing a folder, to any operating system or file system. The API's "similar to Unix 'touch'" is a description of what an empty placeholder is, not a statement that the feature is Unix-only. **Brought up to date by the hand tests of 2026-08-11:** every test above was run on macOS 26.5.2 on an Apple M1 Pro **and** on Ubuntu 22.04.5 LTS on an AMD Threadripper, and the two agreed on every point of behaviour. The one number that differed is how often two renders collide on one frame, which is a race and not an operating-system difference. **The file system does change it**: over a folder mounted by NFS the collision rate went from none in thirty repeats to four, five or six in thirty. **Windows is untested**, no Windows machine having run it.

---

## Things a person at a computer with Blender 4.5 open could settle in a few minutes

**Pruned on 2026-08-11, and pruned again on 2026-08-12** after the second batch of hand tests. All ten items below were carried out, in all four researched versions, on the Creator's Mac and on a rented Linux machine; what came back is written into questions 13 and 14 above as observed findings, and the list they were run from is `Audits/Blender Research/Frames/Hand tests — cross-sweep questions.md`. What is left open is what a running Blender on either of those two machines could not reach:

1. **Two renders on two separate computers writing into one shared folder** — item 9 below. The project's two machines have no file system shared between them. A network file system *was* tested, with both renders on the rented Linux machine writing to a folder mounted over NFS from itself, and it changed the answer — see question 14.
2. **Anything on Windows.** No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
3. **A real power cut, as against a kernel reset.** The reset was done twice, on 2026-08-11 and again on 2026-08-12, and the two runs do not agree about which directory entries survive. Both results are under question 13. Pulling a plug is still out of reach.
4. **Whether Blender receives an Escape key sent by a tool, and so what a person pressing Esc by hand would get.** Whether the render *stops* has been run three times on the rented Linux machine and has come out both ways — see question 13. No run of this shape can show whether the key reached Blender at all, so the hand half stays out of reach.
5. **What a person clicking the status bar's cancel button leaves behind.** That control needs a hand on a mouse.

The ten items are kept below as written, since each names the exact set-up that was used.

Each of these is written so somebody who cannot see my reasoning can carry it out.

**For question 13**

1. **What a stopped render leaves at the frame's path.** Open Blender. In Properties ‣ Output, set File Path to an empty folder, File Format to PNG, and set the frame range to 1–10 in Properties ‣ Output ‣ Format ‣ Frame Range. Make the scene heavy enough that one frame takes at least thirty seconds (raise the resolution, or raise Cycles' sample count). Press Ctrl-F12 to Render Animation. Watch the output folder in a file manager with file sizes showing. When frame 3 has finished and frame 4 is under way, wait until frame 4's render is complete and its file has just appeared, then press Esc during that first second. Report: does a file for frame 4 exist, what is its size in bytes, and does an image viewer open it?

2. **The same, but with the process killed rather than cancelled.** Repeat step 1, but instead of pressing Esc, end the Blender process outright (Task Manager on Windows, `kill` on macOS or Linux) at the moment the frame-4 file appears. Report the same three things.

3. **Whether a temporary name is used.** Repeat step 1 and, while a frame is being written, list the output folder and report every file name in it, including hidden files and any name that is not the expected `####.png`. This settles whether Blender writes under a different name first and renames.

4. **Whether the format changes the answer.** Repeat steps 1 and 2 with File Format set to OpenEXR MultiLayer instead of PNG, with two view layers active. Report file existence, size, and whether the file opens.

5. **What a placeholder looks like next to a finished frame.** In Properties ‣ Output, untick Overwrite and tick Placeholders. Render the same animation, and while frame 4 is rendering, list the folder with sizes. Report the size of frame 4's file while it is rendering and again after it has finished.

**For question 14**

6. **Two renders, defaults, same folder.** Save a blend-file with File Path pointing at one empty folder, PNG output, frame range 1–20, and roughly ten seconds per frame. Leave Overwrite ticked and Placeholders unticked, the shipped defaults. From a terminal, start two background renders at the same moment on the same machine, both writing to that folder: run `blender -b thefile.blend -a` twice, launched together. When both finish, report: how many files are in the folder, their sizes, their modification times, and whether any two files differ from each other in content when the same frame is compared before and after.

7. **Two renders, the manual's render-farm recipe.** Same blend-file, but untick Overwrite and tick Placeholders before saving. Start the two background renders together again. When both finish, report: how many files are in the folder, whether any file is zero bytes, whether every frame in the range has a non-empty file, and — from each render's console output — which frame numbers each one claims to have rendered. Whether the two lists overlap is the whole question.

8. **The same-instant case specifically.** Repeat step 7 several times in a row, launching the two renders as close together as the machine allows. Report whether any run ends with a frame missing, a frame left at zero bytes, or the same frame rendered by both. Repeating matters: a race may show up only sometimes.

9. **Local folder versus network share.** Repeat step 7 twice — once with the output folder on the machine's own disk, once on a folder mounted over the network from another machine — and report whether the outcomes differ. The manual's recipe names a network file system; this checks whether that wording matters.

10. **What a placeholder becomes.** During the run in step 7, watch one specific frame's file in a file manager with sizes showing, and report the sequence of sizes it goes through from first appearing to the render finishing.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Frames, Blender 4.5

2026-08-12 — Claude Opus 5 (Claude Code) — the second batch of hand tests written in: entries 22, 23 and 25 run and recorded under question 13, both standing lines brought up to date, and the researcher's own list pruned again.
