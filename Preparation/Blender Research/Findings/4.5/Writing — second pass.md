# Writing — Blender 4.5 — second pass

**Sources consulted:** Blender 4.5 manual (`Blender Sources/manual_4.5`, text and `_images`), Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`). All read on **2026-07-30** (the snapshot date given in my inputs).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## How I searched

Everything below rests on whole-corpus text searches across `manual_4.5`, `api_4.5` and `release_notes_4.5` at once, plus targeted reads. The searches I ran (each across all three, unless noted) were, grouped by intent:

- **Output path itself:** `output`, `output directory`, `output folder`, `filepath`, `render-output`, `base_path`, `file_slots`, `OutputFile`, `File Output`, `path template`, `Path Template`, `path_template`, `path token`, `path_tokens`, `substitution token`.
- **Marks inside a path:** `# characters`, `####`, `padded zeros`, `padding`, `//`, `Relative Path`, `%`, `% chars`, `percent`, `$`, `backslash`, `path separator`, `forward slash`.
- **Folder creation:** `mkdir`, `makedirs`, `create the folder`, `create the directory`, `creating directories`, `create missing`, `creates the director`, `create.*director`, `nonexistent`, `non-existent`, `does not exist`, `doesn't exist`, `will be created`, `is created automatically`, `automatically created`, `Blender will create`, `Blender creates`, `missing folder`, `missing directory`, `folder does not`, `directory does not`.
- **Write failure:** `permission denied`, `disk full`, `out of disk`, `no space`, `read-only`, `read only file`, `write permission`, `not writable`, `writable`, `cannot write`, `unable to write`, `failed to write`, `Cannot save`, `write error`, `I/O error`, `errno`, `storage is full`, `disk space`, `running out of space`, `insufficient`, `no such file`, `error message`, `reports an error`, `fail with an error`, `abort`, `exit code`, `return code`, `status code`, `quit`, `stops rendering`, `skip the frame`.
- **Characters a computer may reject:** `invalid character`, `illegal character`, `special character`, `forbidden character`, `unsupported character`, `sanitiz`, `reserved name`, `valid filename`, `invalid path`, `path is invalid`, `malformed`, `colon`, `asterisk`, `question mark`, `wildcard character`, `replaced with an underscore`, `rename the file`, `case sensitive`, `utf-8`, `utf8`.

Pages read in full or in large part: `render/output/properties/output.txt`, `render/output/introduction.txt`, `render/output/animation.txt`, `render/output/audio/introduction.txt`, `render/output/properties/index.txt`, `render/output/properties/stereoscopy/introduction.txt` and `usage.txt`, `files/file_paths.txt`, `files/index.txt`, `files/introduction.txt`, `files/blend/open_save.txt`, `files/media/image_formats.txt`, `advanced/command_line/render.txt`, `advanced/command_line/arguments.txt` (whole file), `advanced/limits.txt`, `advanced/blender_directory_layout.txt` (temporary directory section), `editors/3dview/viewport_render.txt`, `editors/file_browser.txt`, `editors/image/introduction.txt`, `editors/preferences/file_paths.txt`, `interface/window_system/topbar.txt` (Render menu), `compositing/types/output/file_output.txt`, `render/color_management.txt` (render result section); API pages `bpy.types.RenderSettings`, `bpy.types.Scene`, `bpy.types.CompositorNodeOutputFile`, `bpy.types.NodeOutputFileSlotFile`, `bpy.types.NodeOutputFileSlotLayer`, `bpy.types.SceneRenderView`, `bpy.types.PreferencesFilePaths`, `bpy.path`, `bpy.ops.render`, `bpy.ops.sound` (mixdown), `bpy.ops.image` (open / save_as); release notes `core.md`, `rendering.md`, `python_api.md`, `compositor.md`, `pipeline_assets_io.md`, `bugfixes.md`, `index.md`; Cycles files `engine.py`, `operators.py`, `properties.py`, `ui.py`, `__init__.py`.

Images opened: `_images/render_output_properties_output_panel.png`, `_images/files_file_paths-invalid_path_template.png`, `_images/files_file_paths-invalid_path_template_tooltip.png`, `_images/node-types_CompositorNodeOutputFile.webp`, `_images/editors_preferences_section_file-paths.png`.

I did not use the web, and did not need it.

---

## 1. Can where a render writes be settled at the moment the render is started, without changing the file?

**Answer.** Yes. Blender's command line takes an output path for that run, and the order of the arguments decides whether it takes effect.

> `-o, --render-output <path>`
>
> Set the render path and file name.
> Use `//` at the start of the path to render relative to the blend-file.

— manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html (Render Options), read 2026-07-30.

The order rules are spelled out twice.

> Arguments are executed in the order they are given!
>
> The following command will not work, since the output and extension are set after Blender is told to render:
>
> `blender -b file.blend -a -x 1 -o //render`
>
> The following command will behave as expected:
>
> `blender -b file.blend -x 1 -o //render -a`
>
> Always position `-f` or `-a` as the last arguments.

— manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html, read 2026-07-30.

> `blender --background --render-output /tmp test.blend --render-frame 1`
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set.
>
> `blender --background test.blend --render-output /tmp --render-frame 1`
>
> …works as expected.

— manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html (Argument Parsing ‣ Argument Order), read 2026-07-30.

A second route exists at start-up: a Python script or expression run before the render.

> `-P, --python <filepath>` — Run the given Python script file.
>
> `--python-expr <expression>` — Run the given expression as a Python script.

— manual, same page (Python Options), read 2026-07-30. The value it would set is a writable property: `RenderSettings.filepath`, "Directory/name to save animations, # characters define the position and padding of frame numbers", `string, default "", (never None, blend relative // prefix supported, Supports template expressions)` — API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30.

**Label.**
- That the path can be set at the moment the render starts: **documented**.
- That the blend-file on disk is left unchanged: **inferred**. Step taken: the sources say the loaded blend-file "overwrites the render output that was set", which places the setting in the loaded scene data, and nowhere in the command-line documentation is `-o` said to write anything back to the blend-file; saving is a separate, explicit action in Blender. The sources never state outright that `-o` leaves the file untouched, so a reader has to supply that last step.

**Not covered.** Whether `-o` applies to every scene in the file or only the one being rendered; whether it also affects a File Output node (see question 9); whether the value is written back to the blend-file on disk under any circumstance; what `-o` does when combined with path templates that fail.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* This *is* the outside-the-interface mechanism. The manual frames it that way: "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)" — https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html, read 2026-07-30. The sources do not describe an equivalent "set the path just for this run" control inside Blender's own window.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say `-o` behaves differently per operating system. The manual carries separate command-line launching pages for Linux, macOS and Windows (`advanced/command_line/launch/`), which cover how to start Blender, not how `-o` behaves. One argument is marked Windows-only (`-con, --start-console`); `-o` is not.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 15 — does `-o` change the blend-file on disk?**

*What was done, to what.* A scene was built at factory settings (Workbench engine,
64 by 64 pixels, frames 1 to 1, PNG) with its render output path stored as
`…/stored/stored_`, and saved as `test.blend`. A byte-for-byte copy was taken as
`keep.blend`. Blender was then started again with no window, given `test.blend`,
then `-o` pointing at a different folder, then `-f 1`. The checksum, size and
modified time of `test.blend` were read before and after; the file was then
reopened and its stored output path printed.

*The command.*

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup …/test.blend -o …/elsewhere2/e_ -F PNG -x 1 -f 1
```

*What came back.*

```
--- before ---
ebc3bcdc58ab4f97f50fddb95e9f9c2eb865c0e08ca761b3b7fb5161009917a5  …/test.blend
ebc3bcdc58ab4f97f50fddb95e9f9c2eb865c0e08ca761b3b7fb5161009917a5  …/keep.blend
…/test.blend 434663 1785882839
Saved: '…/elsewhere2/e_0001.png'
render exit code: 0
--- after ---
ebc3bcdc58ab4f97f50fddb95e9f9c2eb865c0e08ca761b3b7fb5161009917a5  …/test.blend
ebc3bcdc58ab4f97f50fddb95e9f9c2eb865c0e08ca761b3b7fb5161009917a5  …/keep.blend
…/test.blend 434663 1785882839
filepath after reopening: '…/stored/stored_'
```

An earlier run of this same test was made without `--factory-startup`, so the
Creator's own installed add-ons were loaded. That run is recorded here because it
happened: in 5.2 it ended with `Writing: /var/folders/…/T/blender.crash.txt` and
no image, the crash log naming a Python thread setting a property. The command was
then repeated with `--factory-startup` added, which is the run quoted above.

**Entry 16 — what `-o` reaches.** With a file holding both a scene output path and
a compositor File Output node, `-o` was given a third folder. The node's own files
still landed in the node's own folder and the node's stored value was unchanged
afterwards:

```
Saved: '…/node_out/nd_0001.png'
Saved: '…/cli_out/cli_0001.png'
```

*Standing lines, brought up to date.* **Driven from outside** — this is settled:
`-o` sets the path for that run only, the blend-file on disk is untouched (same
checksum, same size, same modified time), the stored path is still there when the
file is reopened, and the image goes where `-o` said. **Operating system** — settled
for macOS only. No Windows machine was used here; Windows sits in the *cannot be run*
section of the hand-test list. *(Citation check, 2026-08-06: this line also said Linux had
no machine here. That is no longer true — a rented Linux machine has since been
used, and its result is in the section immediately below.)*


### Observed — hand tests run on a rented Linux machine, 2026-08-06

Everything in the Linux sections of this document was run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. It is a rented container sharing somebody else's hardware, so it is given fewer cores than the chip really has and its system settings are not entirely its own; other work may have been running on it at the same time, so nothing here says how long anything took. Blender was started with no window unless an entry says otherwise, with its settings folder pointed at one of this topic's own and with the display variable unset so no window could be reached. The build used for this version was **Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)**. Date: 2026-08-06. In the output below, `…` stands for the working folder on that machine, which was `/root/smoozy/Writing/work/<entry>/4.5`. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests — Linux.md`.

**One note on how the output is shown.** In 4.5 the lines Blender prints while rendering are plain, with no timestamp in front, so they are quoted here exactly as they came.

**These results settle Linux and say nothing about macOS.** Where the same test was also run on the Creator's Mac, both results are given and neither is folded into the other.

**Entry 17 — does the output argument change the blend-file on disk?**

*What was done, to what.* A blend-file was saved with its output path recorded as
`…/stored/stored_`, a byte-for-byte copy kept beside it, and the file then rendered
with the output argument pointing at `…/elsewhere/e_`.

*The command.*

```
/opt/blender/blender-4.5.12-linux-x64/blender -b --factory-startup …/test.blend -o …/elsewhere/e_ -F PNG -x 1 -f 1
```

*What came back.* The two checksums and the size and modified time were identical
before and after:

```
--- checksums before
6b2e3f4e29ea09ef74aca64c33c4f28cd9da4826c6a6f843b4065d7ae6c3f359  …/test.blend
6b2e3f4e29ea09ef74aca64c33c4f28cd9da4826c6a6f843b4065d7ae6c3f359  …/keep.blend
…/test.blend 434663 1785990949
--- the render, with the output argument pointing somewhere else
Saved: '…/elsewhere/e_0001.png'
exit code: 0
--- checksums after
6b2e3f4e29ea09ef74aca64c33c4f28cd9da4826c6a6f843b4065d7ae6c3f359  …/test.blend
6b2e3f4e29ea09ef74aca64c33c4f28cd9da4826c6a6f843b4065d7ae6c3f359  …/keep.blend
…/test.blend 434663 1785990949
--- reopening the file and reading the stored path
filepath after reopening: '…/stored/stored_'
```

So the image landed where the argument said, and the file on disk was not touched.

**Entry 18 — what the argument reaches.** Pointing it at a third folder moved the
scene's own output and left a File Output node exactly where it was; the node's
stored values read back unchanged afterwards.

*The two machines.* Both were run. **They agreed**: the Creator's Mac and the rented
Linux machine gave the same result in this version.

*Standing lines, brought up to date.* **Driven from outside** — every run had no
window at all. **Operating system** — now settled on two systems, macOS and Linux,
which agreed. Windows is still untried.


## 2. How is a place to write recorded in a file — what parts does the recorded thing have?

**Answer.** Two different things are recorded, in two different shapes.

**(a) The scene's render output path — one string.** It hangs off the scene: `Scene.render` is of type `RenderSettings, (readonly, never None)` (API, https://docs.blender.org/api/4.5/bpy.types.Scene.html). On that:

> `filepath`
>
> Directory/name to save animations, # characters define the position and padding of frame numbers
>
> Type: string, default "", (never None, blend relative // prefix supported, Supports template expressions)

— API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30.

So the one string carries a directory part and a name part, and may carry `#` marks. In the interface it is one field:

> File Path
>
> Choose the location to save rendered frames.

— manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30.

The screenshot on that page (`_images/render_output_properties_output_panel.png`, part of the manual, read 2026-07-30) shows the Output panel with a single path field holding `/tmp/`, and below it the checkboxes Overwrite, Placeholders, File Extensions, Cache Result, then File Format, Color, Color Depth, Compression. Label for what that picture shows: **documented**.

Two things that end up in the written file name are *not* in the string:

> `use_file_extension` — Add the file format extensions to the rendered file name (eg: filename + .jpg) — Type: boolean, default True
>
> `file_extension` — The file extension used for saving renders — Type: string, default "", (readonly, never None)

— API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30.

**(b) A File Output node's path — two parts.**

> Base Path
>
> Unlike the render output filepath, this node uses a base directory and an image name, by default the output path is composed of: `{base path}/{file name}{frame number}.{extension}`.
>
> Besides being split into two settings, in all other respects, this setting is treated the same as the render output path.

— manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html, read 2026-07-30.

In the API those two settings are:

> `base_path` — Base output path for the image — Type: string, default "", (never None, blend relative // prefix supported, Supports template expressions)

— API, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html, read 2026-07-30.

> `path` — Subpath used for this slot — Type: string, default "", (never None, Supports template expressions)

— API, https://docs.blender.org/api/4.5/bpy.types.NodeOutputFileSlotFile.html, read 2026-07-30. (The multilayer variant records a layer name instead: `name` — "OpenEXR layer name used for this slot" — API, https://docs.blender.org/api/4.5/bpy.types.NodeOutputFileSlotLayer.html.)

The node's picture in the manual (`_images/node-types_CompositorNodeOutputFile.webp`, read 2026-07-30) shows a node headed "File Output", a "Base Path:" label above a single path field holding `/tmp/`, and one input socket named "Image" marked WebP. **Documented**, from the picture.

**(c) Lengths.** The manual gives fixed internal string limits:

> directory: 767
>
> file-name: 255
>
> file-path: 1023
>
> identifier: 63
>
> Note: Multi-byte encoding means some Unicode characters use more than a single ASCII character.

— manual, https://docs.blender.org/manual/en/4.5/advanced/limits.html (Text Fields), read 2026-07-30.

**(d) A default for new scenes** is held in preferences, not in the scene: `render_output_directory` — "The default directory for rendering output, for new scenes" — API, https://docs.blender.org/api/4.5/bpy.types.PreferencesFilePaths.html, read 2026-07-30; and in the manual, "Render Output — Where rendered images/videos are saved" — https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html, read 2026-07-30.

**Label.** documented.

**Not covered.** How any of this is laid out inside the blend-file as bytes — the sources describe properties, not file format. Whether the limits quoted above are the ones enforced on the render output path specifically; the limits page says only "Fixed strings are used internally, and while it is not useful to list all limits, here are some common limits." Whether a trailing slash is what makes Blender treat the string as a directory rather than a name-stem — the sources never say.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The same properties are the ones the Python API exposes, so the recorded thing is identical whether reached by clicking or by script; the API pages quoted above are the outside-the-interface view of it. The sources do not describe any separate recording used only in background mode.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say the recorded shape differs by operating system. The note about multi-byte encoding under the length limits is the only hint that the same text can consume different amounts of the limit, and it is not tied to any one system.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 1 — what a fresh Blender records, and which switches sit beside it.**

*What was done, to what.* Blender was started with no window and factory settings,
and a short script printed the render output path, the built-in default behind it,
each of the output switches, and the preference that seeds a new scene.

*The command.*

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup --python q1.py
```

*What came back.*

```
startup filepath '/tmp/'
rna default     ''
use_file_extension True True
file_extension True '.png'
use_overwrite True True
use_placeholder True False
use_render_cache True False
save_output False 'ABSENT'
file_format PNG
media_type ABSENT
render_output_directory '//'
```

So the value a fresh Blender starts with is `/tmp/`, which comes from the startup
file rather than from the property's own built-in default; the built-in default is
''. The preference that seeds a new scene reads `//`.

**Entry 2 — what a File Output node records.**

*What was done, to what.* At factory settings, a compositor node tree was made and
one File Output node added to it; every property of the node was printed with its
kind and whether it can be written.

*The command.*

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup --python q2.py
```

*What came back.*

```
scene has node_tree: True
scene has compositing_node_group: False
node type CompositorNodeOutputFile
  PROP active_input_index INT writable 0
  PROP base_path STRING writable ''
  PROP file_slots COLLECTION readonly ''
  PROP format POINTER readonly ''
  PROP layer_slots COLLECTION readonly ''
  PROP save_as_render BOOLEAN writable False
inputs ['Image']
```

(Only the properties bearing on where the node writes are quoted; the full print
also listed the ordinary node properties — colour, label, location and so on.)
A slot's own piece of path is `file_slots[0].path`, which started as `'Image'`.

**Entry 25 — how much text the path field actually holds.** Asking for a path
1343 characters long and reading it back gave a stored length of exactly 1023.
A path 443 characters long was stored whole. So the published 1023 limit is the
one enforced on the stored text.

*Standing lines, brought up to date.* **Driven from outside** — everything above
was read and set with no window open, so the recorded shape is the same thing a
driven Blender sees. **Operating system** — settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 4.5.12**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 29 — the switch in the Output panel's header.**

*What was done, to what.* Blender opened at its normal startup file, Properties ‣
Output tab, the panel headed Output, and the pointer rested on the header strip of
that panel.

*What came back*, in the Creator's own words:

> "This checkbox only exists in 5.2. Its tooltip says "Save Output / Write frames to
> disk for animation renders.""

So there is no such checkbox in this version.

The entry also asked for the labels of every checkbox inside the panel, and for the
`Python:` line at the end of the tooltip. Neither was reported, and neither is
filled in here.

**Entry 30 — what a File Output node shows on its face.**

*What was done, to what.* A file holding a File Output node wired to a Render Layers
node was opened with the matching Blender, the Compositing workspace shown, and the
node looked at; then the node selected and the sidebar opened. Two screenshots per
version, kept at `/Users/artist/Desktop/Screenshots/010/`, one folder per version.

*One thing to record plainly.* The file used was not the one this entry named. It was
a file prepared for another topic — its node's own Output Paths panel points into that
topic's folder — and it carries three inputs in 4.5 and two in the other three rather
than the one the entry described. What is written below is what the screenshots show.

*What is on the node's face.* One label, `Base Path:`, above one text field
holding `//out/` with a folder-browse button. Below it, one row per input, each row
showing the input's name on the left and its format on the right:

```
File Output
  Base Path:
  [ //out/                    ]
  ○ Image           PNG
  ○ passes/di…      PNG
  ○ beauty          PNG
```

*What is in the sidebar.* A **Node** panel holding Name (`File Output`), Label
(empty) and Color. Then a **Properties** panel holding, in this order:

```
Base Path:  [ //out/ ]
            [x] Save as Render
File Format         PNG
Color               BW | RGB | RGBA        (RGBA chosen)
Color Depth         8 | 16                 (8 chosen)
Compression         15%
Color Management    Follow Scene
[ + Add Input ]
  Image
  passes/diffuse
  beauty                                   (selected)
File Subpath:  [ beauty ]
Format:  [x] Use Node Format
```

So in this version the node records one path field, and each input carries its own
**File Subpath** and its own choice of whether to use the node's format. There is no
panel showing finished file paths.

**Entry 32 — very long text in the File Path field.**

*What was done, to what.* A prepared line of 1400 characters
(`abcdefghij` repeated 140 times) was pasted over the File Path field in Properties ‣
Output, Return pressed, and whatever remained in the field copied back out and saved
to a file. The four saved files are at
`/Users/artist/Documents/Smoozy hand tests/Writing/32/`, named `came back 4.5.txt`,
`came back 5.0.txt`, `came back 5.1.txt` and `came back 5.2.txt`, beside the
`long line.txt` that went in.

*What came back.* Read from the saved files:

```
went in                 : 1400 characters
came back, this version : 1023 characters
```

and the 1023 characters that came back are exactly the first 1023 of the 1400 that
went in — the same in all four versions. So the field itself cuts the text down to
1023 characters as it is entered.

The entry also asked for any message Blender showed on pressing Return. None was
reported, so whether one appeared is not recorded here.

*Standing lines, brought up to date.* **Driven from outside** — nothing here bears on it: every one of these was done at Blender's own window, which is the half the terminal cannot reach. **Operating system** — the Creator works on a Mac, so this settles macOS and leaves Windows and Linux open.


### Observed — hand tests run on a rented Linux machine, 2026-08-06

**Entry 15 — what a fresh Blender records, and which output switches it has.**

*What was done, to what.* Blender was started at factory settings with no window and
asked to print the recorded output path, the property's own built-in default, the
output switches, the file format and the preference that seeds a new scene.

*What came back.*

```
VERSION 4.5.12 LTS
startup filepath '/tmp/'
rna default     ''
use_file_extension True True
file_extension True '.png'
use_overwrite True True
use_placeholder True False
use_render_cache True False
save_output False 'ABSENT'
file_format PNG
media_type ABSENT
render_output_directory '//'
```

**Entry 16 — what a File Output node records.** The node's own settings came back as
`active_input_index`, `base_path`, `file_slots`, `layer_slots`, `format`, with its inputs reading `['Image']`.

**Entry 3 — how long the recorded text may be.** Asked to hold a path of 1239
characters, the property came back holding 1023: `stored length: 1023`. So 1023 is
the limit actually enforced, and the text is cut without a word being said.

**Entry 25 — the folder separator.** On this machine `os.sep` is `'/'`. A path typed
with backslashes is **stored exactly as typed** and turned into forward slashes only
when it is used:

```
--- backslash
    asked for: '…\\sep_back\\img_'
    stored as: '…\\sep_back\\img_'
    frame_path: '…/sep_back/img_0001.png'
```

A doubled forward slash in the middle of a path is collapsed to one. Blender's own
helper `bpy.path.native_pathsep('a\\b\\c.png')` came back `'a/b/c.png'`.

*The two machines.* Entries 15, 16 and 3 were run on both. **They agreed** in this
version. Entry 25 was run on Linux only.

*Standing lines, brought up to date.* **Driven from outside** — every run had no
window. **Operating system** — now settled on two systems for what is recorded and
for the 1023-character limit, and they agreed. What the separator does is settled
for Linux, where a backslash in a stored path is turned into a forward slash when the
path is used. Windows is still untried.


## 3. What characters or marks can a recorded place to write carry that stand for something else rather than for themselves, and what does each of them do?

**Answer.** Four, of which three are documented for the render output path itself.

**`//` at the start — the blend-file's own folder.**

> This setting expands Relative Paths where a `//` prefix represents the directory of the current blend-file.

— manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30.

> When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (`//`) to make it so.

and

> You cannot use relative paths into a new untitled blend-file. Save it before linking to external files.

— manual, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html (Relative Paths), read 2026-07-30.

> The default path `//` refers to the folder of the currently open blend-file

— manual, https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html, read 2026-07-30.

**`#` — the frame number, and how many digits it gets.**

> When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of `#` anywhere in the file name (e.g. image_##_test.png translates to image_01_test.png).

— manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30.

> The `#` characters are replaced by the frame number, and used to define zero padding.
>
> animation_##_test.png becomes animation_01_test.png
>
> test-######.png becomes test-000001.png
>
> When the filename does not contain `#`, the suffix `####` is added to the filename.

— manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30.

**`{` … `}` — template expressions.** New in this version:

> Some file paths in Blender can now contain "template expressions" like `{blend_name}` and `{resolution_x}`, which get replaced by the values they reference when the path is used. […] Currently this is only enabled on the Scene's render output path and the output paths of the compositor's File Output node.

— release notes, https://developer.blender.org/docs/release_notes/4.5/core/, read 2026-07-30.

The full behaviour, from the manual (https://docs.blender.org/manual/en/4.5/files/file_paths.html, read 2026-07-30):

> Path templates substitute template expressions (written `{abcd}`, where "abcd" is a variable name) in a filepath when the path is used.

> This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field.

The variables it lists are exactly four:

> blend_name: The current blend file's name (without the .blend).
>
> fps: The frames per second of the current scene.
>
> resolution_x / resolution_y: The x and y resolution of the rendered image. This factors in the resolution scale as well, so if the scene resolution is 1000x600 and the scale is 50%, then resolution_x and resolution_y will be 500 and 300, respectively.
>
> The set of available variables will expand over time.

Inside an expression, a colon introduces a format, and `#` changes meaning again — it counts digits:

> `dance_{fps:###}.png`: format as an integer with at least 3 digits.
>
> `dance_{fps:.###}.png`: format as a floating point number with exactly 3 digits after the decimal point.
>
> `dance_{fps:###.##}.png`: format as a floating point number with at least 3 digits for the integer part and exactly 2 digits for the fractional part.
>
> In all cases, the number of hash symbols (#) indicates the desired number of digits.

with worked examples for fps 29.97 giving `dance_030.png`, `dance_29.970.png` and `dance_029.97.png`, and "Note that the values are properly rounded for the given number of digits."

And a doubled brace stands for a plain brace:

> `{{` translates to a single `{` in the final path.
>
> `}}` translates to a single `}` in the final path.
>
> `my_weird}}_{{path.png -> my_weird}_{path.png`
>
> `//my_render_{{dir}}/{blend_name}.png -> //my_render_{dir}/dance.png`
>
> `//my_render_dir/{{{blend_name}}}.png -> //my_render_dir/{dance}.png`

**`%` — a view name (multi-view / stereo).** Only stated in the API, as a parameter of the path resolver:

> `view (string, (optional, never None))` – View, The name of the view to use to replace the "%" chars

— API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html (`frame_path`), read 2026-07-30.

**What I looked for and did not find.** No environment-variable expansion (`$NAME`, `%NAME%`), no `~` for a home folder, no wildcard `*`, and no other token is documented for render output paths. Searched all three sources for `$`, `environment variable`, `~`, `wildcard`, `path token`, `substitution token`, `%`, `percent`. Two near-misses I ruled out:

- The Text Editor preference has its own expansions — "`$filepath`: The absolute path of the file. `$line`: The line to open at (Optional). `$column`…" (manual, https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html). These are arguments for launching an external text editor, not path marks.
- Two image operators take `allow_path_tokens (boolean, (optional)) – Allow the path to contain substitution tokens` (API, https://docs.blender.org/api/4.5/bpy.ops.image.html, `bpy.ops.image.open` and `bpy.ops.image.save_as`). The tokens themselves are not listed anywhere I could find, and these are image operators, not the render output path.

**Label.** documented for `//`, `#` and `{}`/`{{`/`}}`; for `%`, documented only that `frame_path`'s `view` argument replaces "%" chars, and **inferred** for the wider claim that a recorded render output path may carry `%` as a mark at all — the API entry says what `frame_path` does with such characters, not that the stored path may hold them. *(Citation check, 2026-08-05: label corrected from documented to inferred for that wider claim.)*

**Not covered.** Whether `//` and `#` also work in the File Output node's two fields. The manual says of the node's path, "Besides being split into two settings, in all other respects, this setting is treated the same as the render output path" — reading that as "so `//` and `#` work there too" is a step the reader supplies, so it is **inferred**, not documented. Also not covered: whether `%` is honoured in the stored path field itself or only when `frame_path` is called with a view; what `//` resolves to when the blend-file has never been saved and a render is started anyway (the sources only say relative paths cannot be *used* into an untitled file); whether `//../` climbs to a parent folder.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* `//` is explicitly supported on the command line — "Use `//` at the start of the path to render relative to the blend-file" (manual, arguments page) — and `#` likewise, with worked examples on the same page for `-o`. Templates are described as substituting "at time of use (e.g. while rendering)", which does not depend on a window being open. The one part of the template documentation that is interface-only is how errors are shown (a red field and a tooltip); see question 5.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say any of these marks differ by operating system.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 9 — what each mark does, asked of the path builder.**

*What was done, to what.* A file was saved as `dance.blend`, and
`scene.render.frame_path(frame=7)` was called after setting the render output path
to each of fifteen strings in turn, with the File Extensions switch on and then off.

*(Citation check, 2026-08-05: only the switch-on half is reported below. The
switch-off run is described as having been made and its results were never written
down, so that half cannot be repeated against anything. It has gone onto the
hand-test list to be run again.)*

*(Hand tests, 2026-08-05: the switch-off half was run again as entry 34.
It is written out below, beside this entry rather than over it.)*

*What came back*, with the extension switch on:

```
'/tmp/out_'                  -> '/tmp/out_0007.png'
'/tmp/out_##_x.png'          -> '/tmp/out_07_x.png'
'/tmp/out_######'            -> '/tmp/out_000007.png'
'//out_'                     -> '…/out_0007.png'
'//sub/out_'                 -> '…/sub/out_0007.png'
'/tmp/{blend_name}_'         -> '/tmp/dance_0007.png'
'/tmp/{fps}_'                -> '/tmp/24.0_0007.png'
'/tmp/out_%_'                -> '/tmp/out_%_0007.png'
'/tmp/out_%_##'              -> '/tmp/out_%_07.png'
'/tmp/dir_##/out_'           -> '/tmp/dir_##/out_0007.png'
'/tmp/out_.png'              -> '/tmp/out_.png0007.png'
'/tmp/'                      -> '/tmp/0007.png'
''                           -> '0007.png'
'/tmp/my_weird}}_{{path_'    -> '/tmp/my_weird}_{path_0007.png'
```

and with a view name supplied:

```
view=''     -> '/tmp/out_%_0007.png'
view='left' -> '/tmp/out_%_0007_L.png'
view='right' -> '/tmp/out_%_0007_R.png'
```

Three things fall out of that. `//` and `#` and `{{`/`}}` all behave as the sources
describe. `{blend_name}` and `{fps}` were replaced; `{scene_name}` was refused with `Unknown variable referenced in template expression '{scene_name}'`. A `#` sitting in the folder part is **not**
replaced — `/tmp/dir_##/out_` came back with the two hashes still in it. And `%` was
**never replaced**: supplying a view left the `%` where it was and added `_L` or `_R`
at the end of the name instead.

**Entry 34 — the same fifteen strings with the File Extensions switch off.**

*What was done, to what.* This is the half of entry 9 that was never written down,
run again on 2026-08-05 so that both halves are on the record. Entry 9's own script
was used word for word: a file saved as `dance.blend`, the render output path set to
each of the same fifteen strings in turn, and `scene.render.frame_path(frame=7)`
called with the File Extensions switch on and then off. Same computer, same build,
no window, and this run's settings folder was made before it was used, because
Blender stops if it is pointed at one that is not there.

*What came back*, with the extension switch off — the `ext=False` lines as the
terminal printed them:

```
ext=False '/tmp/out_'                  -> '/tmp/out_0007'
ext=False '/tmp/out_##_x.png'          -> '/tmp/out_07_x.png'
ext=False '/tmp/out_######'            -> '/tmp/out_000007'
ext=False '//out_'                     -> '…/out_0007'
ext=False '//sub/out_'                 -> '…/sub/out_0007'
ext=False '/tmp/{blend_name}_'         -> '/tmp/dance_0007'
ext=False '/tmp/{scene_name}_'         -> "RAISED RuntimeError: Error: Parse errors in path '/tmp/{scene_name}_':\n- Unknown variable referenced in template expression '{scene_name}'.\n"
ext=False '/tmp/{fps}_'                -> '/tmp/24.0_0007'
ext=False '/tmp/out_%_'                -> '/tmp/out_%_0007'
ext=False '/tmp/out_%_##'              -> '/tmp/out_%_07'
ext=False '/tmp/dir_##/out_'           -> '/tmp/dir_##/out_0007'
ext=False '/tmp/out_.png'              -> '/tmp/out_.png0007'
ext=False '/tmp/'                      -> '/tmp/0007'
ext=False ''                           -> '0007'
ext=False '/tmp/my_weird}}_{{path_'    -> '/tmp/my_weird}_{path_0007'
```

Blender also printed the refusal on its own, once for each switch position, before
the lines above:

```
Error: Parse errors in path '/tmp/{scene_name}_':
- Unknown variable referenced in template expression '{scene_name}'.
```

The switch-on lines of this run matched entry 9's, line for line. Entry 9's table
leaves out `/tmp/{scene_name}_`, which its own paragraph reports on instead; with the
switch on this run met the same refusal as with it off. The three calls made with a
view name, which the script makes with the switch on, came back the same as entry
9's too:

```
view=''     -> '/tmp/out_%_0007.png'
view='left' -> '/tmp/out_%_0007_L.png'
view='right' -> '/tmp/out_%_0007_R.png'
```

Two things fall out. The switch changes one thing and nothing else: with it off, no
`.png` is put on the end. Every mark gave the same result either way — `//`, `#`,
`{}`, `{{`/`}}` and `%` are untouched by it, and `{scene_name}` was refused in the
same words under both switch positions. And of the fourteen strings that gave back a
path, only one came back identical under both switch positions: `/tmp/out_##_x.png`,
whose recorded name already ends in `.png`, to which nothing further was added with
the switch on. Its near miss is `/tmp/out_.png`, where the `.png` sits in the middle
of the stem — with the switch on that came back as `/tmp/out_.png0007.png`, and with
it off as `/tmp/out_.png0007`.

**Entry 11 — `%` and views in a real render.** With two views switched on and the
Individual views format, the files written were:

```
'…/with_percent/img_%_0001_L.png'
'…/with_percent/img_%_0001_R.png'
'…/no_percent/img_0001_L.png'
'…/no_percent/img_0001_R.png'
'…/hash_percent/img_%_001_L.png'
'…/hash_percent/img_%_001_R.png'
```

with `views: [('left', '_L', '_L', True), ('right', '_R', '_R', True)]`. So the `%`
stays in the name as a plain character, and the view's own ending goes on after the
frame number and before the extension whether a `%` is there or not.

**Entry 24 — a backslash.** Setting the path to `…/chars/back\slash_` gave:

```
    stored as: '…/chars/back\slash_'
    frame_path: '…/chars/back/slash_0001.png'
    Saved: '…/chars/back/slash_0001.png'
```

and a folder called `back` appeared. So a backslash in the recorded path stands for
a folder separator.

**Entry 5 — a slash inside a File Output input's name.** The node's own format was
set to a still-image format and two inputs were named `passes/diffuse` and
`masks/object_01`:

```
slot paths now : ['passes/diffuse', 'masks/object_01']
Saved: '…/nodebase/passes/diffuse0001.png'
Saved: '…/nodebase/masks/object_010001.png'
```

The slashes were kept, and the folders `passes` and `masks` were made.

**Entry 14 — a template whose value carries a `#`.** In 4.5 `{scene_name}` is not a name this version knows:

```
Error: Parse errors in path '…/tmpl/{scene_name}_':
- Unknown variable referenced in template expression '{scene_name}'.
```

so the same test was run again through `{blend_name}`, with the file saved as
`da##ce.blend`:

```
blend name: da##ce.blend
frame_path: '…/tmpl/da07ce_.png'
Saved: '…/tmpl/da01ce_.png'
```

So the template is put in first and the hashes it brings with it are then read as a
place for the frame number.

*Standing lines, brought up to date.* **Driven from outside** — every mark above was
exercised with no window open, so all of it holds for a driven Blender.
**Operating system** — settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 4.5.12**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 31 — a broken template expression, seen in the window.**

*What was done, to what.* A file whose render output path was already the unclosed
expression `//out_{blend_name.png` was opened with the matching Blender, the pointer
rested on the File Path field in Properties ‣ Output, and then Ctrl-F12 pressed. Two
screenshots per version, kept at `/Users/artist/Desktop/Screenshots/024/`, one
folder per version.

*What came back.* The File Path field is drawn **filled dark red**, with the text in
white. Its tooltip reads, word for word:

```
Output Path
Directory/name to save animations, # characters define the position and padding of frame numbers.
Value: //out_{blend_name.png
Path template error(s):
  - Invalid or incomplete template expression '{blend_name.png'.
Python: RenderSettings.filepath
bpy.data.scenes["Scene"].render.filepath
```

After Ctrl-F12 a render window opened holding no image, and the message appeared in
two places at once — in the status bar along the bottom of the main window, and as a
red-marked line in the Info list:

```
Parse errors in path '//out_{blend_name.png': - Invalid or incomplete template expression '{blend_name.png'.
```

Nothing was written.

Note that the tooltip's own heading for the fault is `Path template error(s):`.

*Standing lines, brought up to date.* **Driven from outside** — nothing here bears on it: every one of these was done at Blender's own window, which is the half the terminal cannot reach. **Operating system** — the Creator works on a Mac, so this settles macOS and leaves Windows and Linux open.


### Observed — hand tests run on a rented Linux machine, 2026-08-06

**Entry 9 — the same fifteen recorded strings through the path builder, with the
File Extensions switch on and then off.** A blend-file was saved as `dance.blend`,
the output path set to each string in turn, and `scene.render.frame_path(frame=7)`
called under both switch positions. Every line came back the same as on the
Creator's Mac. The lines that carry the marks:

```
ext=True  '/tmp/out_'                  -> '/tmp/out_0007.png'
ext=False '/tmp/out_'                  -> '/tmp/out_0007'
ext=True  '/tmp/out_##_x.png'          -> '/tmp/out_07_x.png'
ext=True  '/tmp/out_######'            -> '/tmp/out_000007.png'
ext=True  '//out_'                     -> '…/out_0007.png'
ext=True  '/tmp/{blend_name}_'         -> '/tmp/dance_0007.png'
ext=True  '/tmp/{fps}_'                -> '/tmp/24.0_0007.png'
ext=True  '/tmp/out_%_'                -> '/tmp/out_%_0007.png'
ext=True  '/tmp/dir_##/out_'           -> '/tmp/dir_##/out_0007.png'
ext=True  '/tmp/my_weird}}_{{path_'    -> '/tmp/my_weird}_{path_0007.png'
view='left' -> '/tmp/out_%_0007_L.png'
```

and for the scene's own name:

```
ext=True  '/tmp/{scene_name}_'         -> "RAISED RuntimeError: Error: Parse errors in path '/tmp/{scene_name}_':\n- Unknown variable referenced in template expression '{scene_name}'.\n"
```

**Entry 2 — a backslash.** Setting the output path to a name ending `back\slash_`
made a **folder** called `back` on this Linux machine and wrote `slash_0001.png`
inside it, exactly as on the Mac. The Linux machine itself has no trouble with a
backslash in a name — an ordinary file called `back\slash` was made without
complaint — so turning it into a folder separator is Blender's own doing and not the
computer's.

**Entry 6 — a slash inside a File Output node's input name.** In this version the slash was kept and the folder it implies was made:

```
slot paths: ['passes/diffuse', 'masks/object_01']
Saved: '…/nodebase/passes/diffuse0001.png'
```

**Entry 14 — a template whose value carries a `#`.** 4.5 does not know that variable, so the path was refused outright and nothing was
written:

```
frame_path raised: RuntimeError Error: Parse errors in path '…/tmpl/{scene_name}_':
- Unknown variable referenced in template expression '{scene_name}'.
exit code: 1
```

**Entry 8 — a broken template expression, with no window.** The path was set to
`…/{blend_name.png`:

```
Error: Parse errors in path '…/{blend_name.png':
- Invalid or incomplete template expression '{blend_name.png'.
frame_path raised: RuntimeError Error: Parse errors in path '…/{blend_name.png':
- Invalid or incomplete template expression '{blend_name.png'.
render raised: RuntimeError Error: Parse errors in path '…/{blend_name.png':
- Invalid or incomplete template expression '{blend_name.png'.
exit code: 1
```

Nothing was written and the folder was left empty.

**Entry 25 — the backslash again, as a separator.** A path typed with backslashes is
stored as typed and turned into forward slashes when used; see question 2.

*The two machines.* Entries 9, 2, 6, 14 and 8 were run on both. **They agreed** in
this version, line for line. Entry 25 was run on Linux only.

*Standing lines, brought up to date.* **Driven from outside** — every run had no
window. **Operating system** — the marks are now settled on two systems, macOS and
Linux, which agreed. Windows is still untried.


## 4. Does Blender make folders that are not already there when it writes?

**Answer. Not in the sources.** I could not find any statement, in any of the three documentation sets or in Cycles' own files, about whether a render creates a missing output folder.

**Where I looked.** Whole-corpus searches of `manual_4.5`, `api_4.5` and `release_notes_4.5` for: `mkdir`, `makedirs`, `create the folder`, `create the directory`, `creating directories`, `create missing`, `creates the director`, `create.*director`, `nonexistent`, `non-existent`, `does not exist`, `doesn't exist`, `will be created`, `is created automatically`, `automatically created`, `Blender will create`, `Blender creates`, `missing folder`, `missing directory`, `folder does not`, `directory does not`, `output directory`, `output folder`. I read in full the pages where such a statement would belong if it existed: the Output panel page, the Rendering Animations page, the Rendering From The Command Line page, the whole Command Line Arguments page, the File Paths (path templates) page, the File Output node page, and the `RenderSettings` API page. None of them mentions folder creation.

**What the searches did turn up, and why I ruled each out:**

- "If the cache path folder does not exist, it will be created." — manual, https://docs.blender.org/manual/en/4.5/modeling/modifiers/physics/ocean.html, read 2026-07-30. This is the Ocean modifier's bake cache, a different feature with its own path field. It says nothing about renders.
- "If you type a nonexistent path, you will be prompted to create it." — manual, https://docs.blender.org/manual/en/4.5/editors/file_browser.html (Directory field), read 2026-07-30. This is the File Browser's own address bar, and it *asks* rather than acting, which is the opposite of a silent write.
- `bpy.ops.file.directory_new` — "Create a new directory" — API, https://docs.blender.org/api/4.5/bpy.ops.file.html. A File Browser button.
- `bpy.utils.user_resource(..., create=...)` — "Treat the path as a directory and create it if its not existing" — API, https://docs.blender.org/api/4.5/bpy.utils.html. Blender's own configuration and script folders, not render output.

I want to be plain about one thing: the fact that the Ocean modifier's page says outright that it creates its folder, and the render output pages say nothing, is *not* evidence either way. It is only the absence I am reporting.

**Label.** not in the sources.

**Not covered.** Everything about the actual behaviour: whether one missing level is created, whether several are, whether it differs between the scene render path and a File Output node path, and whether Placeholders (which write empty files ahead of time) behave differently.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not addressed by the sources, in either direction.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not addressed by the sources.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 3 — the scene's own output path.**

*What was done, to what.* At factory settings, with no window, the render output
path was set to `…/one_missing/img_` (one folder that did not exist) and one frame
rendered; then to `…/two_a/two_b/img_` (two folders that did not exist) and one
frame rendered.

*The command.*

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup --python q3.py -- …
```

*What came back.*

```
Saved: '…/one_missing/img_0001.png'
Saved: '…/two_a/two_b/img_0001.png'
exit code: 0
```

and on disk afterwards:

```
…/one_missing/img_0001.png
…/two_a/two_b/img_0001.png
```

So the folders are made, and a whole chain of missing folders is made, not just the
last one.

**Entry 4 — a File Output node's own folder field.** With the node's folder set to
`…/node_a/node_b/`, neither of which existed:

```
Saved: '…/node_a/node_b/nd_0001.png'
exit code: 0
```

So the node makes its missing folders too, and again more than one level.

**Entry 5 — folders implied by an input's name.** Covered under question 3 above:
the folders implied by a slash in an input's name were made where the slash
survived being set.

*Standing lines, brought up to date.* **Driven from outside** — every run above had
no window at all, so folder making is not something the interface does on Blender's
behalf. **Operating system** — settled for macOS only.


### Observed — hand tests run on a rented Linux machine, 2026-08-06

**Entry 4 — the scene's own output path.**

*What was done, to what.* At factory settings, with no window, the output path was
set to `…/one_missing/img_` (one folder that did not exist) and one frame rendered;
then to `…/two_a/two_b/img_` (two folders that did not exist) and one frame rendered.

*What came back.*

```
Saved: '…/one_missing/img_0001.png'
Saved: '…/two_a/two_b/img_0001.png'
exit code: 0
```

and on disk afterwards:

```
…/one_missing/img_0001.png
…/two_a/two_b/img_0001.png
```

So the folders are made on Linux too, and a whole chain of them rather than only the
last one.

**Entry 5 — a File Output node's own folder field.** With the node's folder set to
`…/node_a/node_b/`, neither of which existed:

```
Saved: '…/node_a/node_b/nd_0001.png'
exit code: 0
```

**Entry 6 — folders implied by an input's name.** In this version the folders implied by a slash in an input's name were made.

*The two machines.* All three were run on both. **They agreed**: a missing output
folder, and a whole chain of missing folders, is made on the Creator's Mac and on the
rented Linux machine alike.

*Standing lines, brought up to date.* **Driven from outside** — every run had no
window at all, so folder making is not something the interface does on Blender's
behalf, on either machine. **Operating system** — now settled on two systems, macOS
and Linux, which agreed. Windows is still untried: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**


## 5. What does a write do when its folder is not there, when there is no room left, or when it cannot be written to?

**Answer. Not in the sources**, for all three cases. The documentation describes what the output settings *are*, not what happens when the drive refuses.

**Where I looked.** Whole-corpus searches for: `permission denied`, `disk full`, `out of disk`, `no space`, `read-only`, `read only file`, `write permission`, `not writable`, `writable`, `cannot write`, `unable to write`, `failed to write`, `Cannot save`, `write error`, `I/O error`, `errno`, `storage is full`, `disk space`, `running out of space`, `insufficient`, `no such file`, `does not exist`, `error message`, `reports an error`, `fail with an error`, `abort`, `exit code`, `return code`, `status code`, `stops rendering`, `skip the frame`. I listed and checked the troubleshooting section of the manual (`troubleshooting/`: report_bug, index, python, recover, network, startup, crash, 3d_view, and the GPU pages) — it covers crashes, start-up problems, graphics drivers and file recovery, and has no page about writing output. I read the Output panel page, the Rendering Animations page, both command-line pages, and the `RenderSettings` API page.

**The one adjacent thing that is documented** is a different failure — a broken *template*, not a broken destination:

> Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors.

and

> When there are template errors in a path, the path field will be highlighted red in the UI […] Hovering over the path field will pop up a tooltip that contains a list of the template errors encountered in that path

— manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html (Errors), read 2026-07-30.

The two pictures on that page show it. In `_images/files_file_paths-invalid_path_template.png` the Output panel's path field, holding `//my_render_dir/{blend_name.png`, is drawn filled red. In `_images/files_file_paths-invalid_path_template_tooltip.png` the tooltip reads, in so many words: "Output Path / Directory/name to save animations, # characters define the position and padding of frame numbers. / Value: //my_render_dir/{blend_name.png / Syntax error(s): / - Invalid or incomplete template expression '{blend_name.png'." Both read 2026-07-30. Label for what those pictures show: **documented**.

The release notes record the matching change on the scripting side:

> `scene.render.frame_path()` used to be infallible but can now fail if there are errors processing Path Templates in the render output path.

— release notes, https://developer.blender.org/docs/release_notes/4.5/python_api/, read 2026-07-30.

The only exit-code control documented anywhere concerns Python, not writing: "`--python-exit-code <code>` — Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line), zero disables." (manual, arguments page, read 2026-07-30).

**Label.** not in the sources (for the three cases asked about); documented for the template-error case, which is not one of them.

**Not covered.** Everything the question asks: whether a missing folder, a full drive or an unwritable folder stops the render, skips the frame, or is only reported; whether anything is printed to the console; whether the process exit status reflects it.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The one documented failure route is described in interface terms — a red field and a hover tooltip — neither of which exists when Blender is driven from outside. The same page does say the render "will fail with an error message", which is not tied to a window, but the sources do not say where that message goes in background mode.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not addressed by the sources.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 3 — a folder that is not there.** Answered under question 4 above: the
folder is made and the frame is written. There is no failure to report.

**Entry 6 — a folder that cannot be written to.**

*What was done, to what.* A folder was made and its write permission removed with
`chmod 500`. The refusal was proved first by trying to make an ordinary empty file
in it. A three-frame animation was then rendered into it, with no window.

*What came back.*

```
touch [.../ro/canary.txt]: Permission denied
touch exit code: 1
…/ro/img_0001.png: Permission denied
Error: Render error (Permission denied) cannot save: '…/ro/img_0001.png'
raised: RuntimeError Error: Render error (Permission denied) cannot save: '…/ro/img_0001.png'
python-route exit code: 1
```

and from the command line alone, with `-o` pointed into the same folder:

```
…/ro/cli_0001.png: Permission denied
Error: Render error (Permission denied) cannot save: '…/ro/cli_0001.png'
cli-route exit code: 1
```

Afterwards the folder was empty — no file at all, not even a part-written one.
The render stopped at the first frame: frames 2 and 3 were never attempted.

**Entry 7 — no room left.**

*What was done, to what.* A disposable disk image of two megabytes was made and
mounted, and a three-frame animation of 2000 by 2000 pixel uncompressed 32-bit
OpenEXR files was rendered onto it, with no window.

*What came back.*

```
OpenEXR-writePixels: ERROR: Failed to write pixel data to image file "/Volumes/SMOOZYTINY/img_0001.exr". No space left on device.
Saved: '/Volumes/SMOOZYTINY/img_0001.exr'
IMB_exr_begin_write: ERROR: No space left on device.
Error: Error writing render result, No space left on device (see console)
Error: Render error (No space left on device) cannot save: '/Volumes/SMOOZYTINY/img_0002.exr'
raised: RuntimeError Error: Error writing render result, No space left on device (see console)
exit code: 1
```

and the volume held one file afterwards, `img_0001.exr`, 1,908,736 bytes — a
part-written file that Blender had reported as `Saved` on the same line as the
failure. The render stopped at frame 2; frame 3 was never attempted.

**Entry 8 — a broken template expression, with no window.**

```
path set to: '…/{blend_name.png'
frame_path raised: RuntimeError Error: Parse errors in path '…/{blend_name.png':
- Invalid or incomplete template expression '{blend_name.png'.

Error: Parse errors in path '…/{blend_name.png':
- Invalid or incomplete template expression '{blend_name.png'.
render raised: RuntimeError Error: Parse errors in path '…/{blend_name.png':
- Invalid or incomplete template expression '{blend_name.png'.
exit code: 1
```

Nothing was written, and the wording of the message is the same one the manual
shows in a tooltip.

**Entry 23 — switching automatic saving off.** There is no such switch in 4.5:

```
has save_output: False
could not set save_output: AttributeError 'RenderSettings' object has no attribute 'save_output'
animation returned: {'FINISHED'}
listing: ['img_0001.png']
```

**Entry 25 — a name the computer will not take.** A file name of 300 characters was
stored whole and then refused by the computer: `Error: Render error (File name too long) cannot save: '…'`,
exit code 1, and nothing on disk.

*Standing lines, brought up to date.* **Driven from outside** — settled. Every
failure above is announced on the terminal in the words quoted, and a Blender
started with `-b` and told to render from the command line ends with an exit code
of 1. The one qualification is that a failure raised inside a Python script that
catches it — as entry 23 does — leaves the process ending with 0.
**Operating system** — settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 4.5.12**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 31 — a broken template expression, seen in the window.**

*What was done, to what.* A file whose render output path was already the unclosed
expression `//out_{blend_name.png` was opened with the matching Blender, the pointer
rested on the File Path field in Properties ‣ Output, and then Ctrl-F12 pressed. Two
screenshots per version, kept at `/Users/artist/Desktop/Screenshots/024/`, one
folder per version.

*What came back.* The File Path field is drawn **filled dark red**, with the text in
white. Its tooltip reads, word for word:

```
Output Path
Directory/name to save animations, # characters define the position and padding of frame numbers.
Value: //out_{blend_name.png
Path template error(s):
  - Invalid or incomplete template expression '{blend_name.png'.
Python: RenderSettings.filepath
bpy.data.scenes["Scene"].render.filepath
```

After Ctrl-F12 a render window opened holding no image, and the message appeared in
two places at once — in the status bar along the bottom of the main window, and as a
red-marked line in the Info list:

```
Parse errors in path '//out_{blend_name.png': - Invalid or incomplete template expression '{blend_name.png'.
```

Nothing was written.

Note that the tooltip's own heading for the fault is `Path template error(s):`.

*Standing lines, brought up to date.* **Driven from outside** — nothing here bears on it: every one of these was done at Blender's own window, which is the half the terminal cannot reach. **Operating system** — the Creator works on a Mac, so this settles macOS and leaves Windows and Linux open.


### Observed — hand tests run on a rented Linux machine, 2026-08-06

**Entry 4 — a folder that is not there.** No failure at all: the folder is made and
the frame is written. Same as on the Mac. See question 4.

**Entry 7 — a folder that cannot be written to.** This needed care on this machine,
and the care is part of the result. The rented machine is reached as the root user,
and **the root user is not stopped by an unwritable folder**: with the folder set to
allow no writing at all, an ordinary `touch` succeeded and Blender wrote all three
frames without complaint, ending with exit code 0. So the test was run again with the
one capability that lets root ignore permissions dropped for the run, which is what
made the folder genuinely unwritable — proved first with an ordinary file:

```
touch: cannot touch '…/ro/canary2.txt': Permission denied
touch with the capability dropped, exit code: 1
```

With that in place, a render aimed at the folder gave:

```
…/ro/img_0001.png: Permission denied
Time: 00:00.34

Error: Render error (Invalid argument) cannot save: '…/ro/img_0001.png'
raised: RuntimeError Error: Render error (Invalid argument) cannot save: '…/ro/img_0001.png'
exit code: 1
```

and straight from the command line, with no Python at all:

```
…/ro/cli_0001.png: Permission denied
Error: Render error (Invalid argument) cannot save: '…/ro/cli_0001.png'
exit code: 1
```

**This is where the two machines differ, and it is in the words only.** On the
Creator's Mac 4.5 named the real cause in the bracket — `Error: Render error
(Permission denied) cannot save:` — where on Linux the bracket reads `(Invalid
argument)`. On both machines the operating system's own reason, `Permission denied`,
is printed on the line above; only the bracket differs. *(Citation check,
2026-08-06: narrowed — "instead" read as though only Linux printed that line, but
the Mac record above this section shows the same line.)*

In both runs with the capability dropped, the folder was left **completely empty** —
not even a part-written file — and the render stopped at the very first frame, with
frames two and three never attempted. In the plain root run, where the machine let the
write through, all three frames were written and the process ended with 0.

**No room left on the volume** was **not run**. It needs the machine to itself, and
the machine is shared. It is written out in full under *Needs the machine to itself*
in `Audits/Blender Research/Writing/Hand tests — Linux.md`.

**Entry 8 — a broken template expression.** The same as on the Mac, word for word:
the path-building call and the render both refuse with `Invalid or incomplete
template expression`, nothing is written, and a Blender with no window ends with exit
code 1. See question 3.

*The two machines.* Entries 4 and 8 were run on both and **agreed**. Entry 7 was run
on both: **what the render did agreed** — it stops at the first frame, leaves nothing
behind, and ends with exit code 1 — and **what it said differed**, as set out above.

*Standing lines, brought up to date.* **Driven from outside** — every run had no
window; the failure is announced on the terminal and ends the process with 1.
**Operating system** — the missing-folder case and the unwritable-folder case are now
settled on two systems, macOS and Linux; what a render does is the same on both and
what it prints is not. Running out of room is settled on macOS only and still open on
Linux. Windows is untried.


## 6. What are the ways a render can be started, and what does each of them write?

**Answer.** The sources document these ways.

**(a) Render ‣ Render Image (F12).**

> Render Image F12 — Render the active scene at the current frame.

— manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html (Render Menu), read 2026-07-30.

That it writes nothing on its own is **inferred**, not stated — the nearest the sources come is:

> While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered.

which says a still is held in a buffer to be viewed and saved, and says nothing about what the menu item writes. *(Citation check, 2026-08-05: the flat claim "it writes nothing on its own" was narrowed and marked inferred here, matching the Label paragraph below, which already called this joining-up inferred.)*

— manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html, read 2026-07-30.

**(b) Render ‣ Render Animation (Ctrl-F12).** "Render the animation of the active scene." (topbar page). What it writes depends on the format:

> The Direct Approach […] is where you set your output format to an AVI or MOV format, and click Animation to render your scene directly out to a movie file. Blender creates one file that holds all the frames of your animation.

> The Frame Sequence is a much more stable approach, where you set your output format to a still format (such as JPG, PNG or a multi-layer format). […] Blender creates a file for each frame of the animation.

— manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html, read 2026-07-30.

The colour-management page names the same operator as the one that writes: "the files saved directly to a drive with the Render Animation operator" — https://docs.blender.org/manual/en/4.5/render/color_management.html, read 2026-07-30.

Two settings change what an animation write does to files that are already there:

> Overwrite — Overwrite existing files when rendering.
>
> Placeholders — Create empty placeholder frames while rendering.

— manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30. In the API, `use_placeholder` is "Create empty placeholder files while rendering frames (similar to Unix 'touch')" and `use_overwrite` is "Overwrite existing files while rendering", default True (https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html). The manual's "Primitive Render Farm" hint on the Output page turns the pair into a method: "Set up a shared directory over a network file system. Disable Overwrite, enable Placeholders in the Render Output panel. Start as many machines as you wish rendering to that directory."

The release notes for this version record a fix in this area: "Placeholder output files wrong name" — https://developer.blender.org/docs/release_notes/4.5/bugfixes/ (Render & Cycles section list), read 2026-07-30.

**(c) Render ‣ Render Audio.** "Mix the scene's audio to a sound file." (topbar page). Its operator is `bpy.ops.sound.mixdown(*, filepath='', check_existing=True, …)` — "Mix the scene's audio to a sound file", with `check_existing` = "Check and warn on overwriting existing files" (API, https://docs.blender.org/api/4.5/bpy.ops.sound.html, read 2026-07-30). The manual adds "Split Channels — Each audio channel will be rendered into a separate file." (https://docs.blender.org/manual/en/4.5/render/output/audio/introduction.html).

**(d) 3D Viewport ‣ View ‣ Viewport Render Image / Viewport Render Animation / Viewport Render Keyframes.** From https://docs.blender.org/manual/en/4.5/editors/3dview/viewport_render.html, read 2026-07-30:

> Additionally, some output settings are used too: Resolution / Aspect / Output path / File format

> To render a still image, use 3D Viewport ‣ View ‣ Viewport Render Image.
>
> To render an animation, use 3D Viewport ‣ View ‣ Viewport Render Animation.
>
> To render an animation, but only those frames that have a keyframe, use 3D Viewport ‣ View ‣ Viewport Render Keyframes. This only renders those frames for which the selected objects have an animation key. The other frames are still written to the output, but will simply repeat the last-rendered frame.

and "Viewport rendering only works for the Workbench and EEVEE render engines. It's not supported for Cycles."

**(e) From the command line.** Two arguments start a render, and they are described differently:

> `-a, --render-anim` — Render frames from start to end (inclusive).
>
> `-f, --render-frame <frame>` — Render frame <frame> and save it. `+<frame>` start frame relative, `-<frame>` end frame relative. A comma separated list of frames can also be used (no spaces). A range of frames can be expressed using `..` separator between the first and last frames (inclusive).

— manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30. Note that `-f` says "and save it" outright.

**(f) From Python.** Two operators:

> `bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, layer='', scene='')` — Render active scene
>
> `write_still (boolean, (optional))` – Write Image, Save the rendered image to the output path (used only when animation is disabled)

> `bpy.ops.render.opengl(*, animation=False, render_keyed_only=False, sequencer=False, write_still=False, view_context=True)` — Take a snapshot of the active viewport
>
> `write_still (boolean, (optional))` – Write Image, Save the rendered image to the output path (used only when animation is disabled)

— API, https://docs.blender.org/api/4.5/bpy.ops.render.html, read 2026-07-30.

**Label.** documented, for each of the above. The joining-up — that (a) and (f)-with-`write_still=False` are the same "renders but does not write" case — is **inferred**; step taken: the still-render operator's own parameter says the image is saved only when `write_still` is set, and the manual says a still is held in a render buffer to "view and save", so the two describe the same thing from two sides.

**Not covered.** Whether a File Output node in the compositor writes on every one of these routes (the node page says "The image(s) will be saved on rendering" without naming which renders). Baking is not covered here — the question asks about renders. Whether the Video Sequencer's own render is a distinct route or just Render Animation with sequencer processing on (`use_sequencer` — "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist", default True — API RenderSettings). What Autosplit does to the file count for long videos beyond "This will automatically split the output into multiple files after the first file is 2GiB in size" (Output page).

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Routes (a)–(d) are menu items and do not exist when Blender is driven from outside its window. Route (e) is command-line only. Route (f) works either way. The manual's command-line render page presents `-b` (background, "Run in background (often used for UI-less rendering)") as the normal way to do this without a display.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say the routes differ by operating system. The one engine-related restriction stated is about engines, not systems (Viewport Render: Workbench and EEVEE only).

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 22 — what `-f`, `-a` and the Python render call each write.**

*What was done, to what.* One saved file holding both a scene output path and a
compositor File Output node was copied three times, and each copy rendered by one
route, each with its own `-o` folder so the routes could not see each other's work.

*The commands.*

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup …/r_f.blend  -o …/f_scene/s_ -F PNG -x 1 -f 1
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup …/r_a.blend  -o …/a_scene/s_ -F PNG -x 1 -a
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup …/r_py.blend -o …/py_scene/s_ -F PNG -x 1 --python q22.py
```

where `q22.py` calls `bpy.ops.render.render` three times: `animation=False,
write_still=False`, then `animation=False, write_still=True`, then `animation=True`.

*What came back.* From `-f 1`:

```
Executing compositor
Saved: '…/node_out/nd_0001.png'
Saved: '…/f_scene/s_0001.png'
exit code: 0
```

From `-a`:

```
Executing compositor
Saved: '…/node_out/nd_0001.png'
Saved: '…/a_scene/s_0001.png'
exit code: 0
```

From the three Python calls:

```
--- animation=False, write_still=False
    {'FINISHED'}
--- animation=False, write_still=True
    {'FINISHED'}
--- animation=True
    {'FINISHED'}
Saved: '…/node_out/nd_0001.png'      (from animation=False, write_still=False)
Saved: '…/node_out/nd_0001.png'      (from animation=False, write_still=True)
Saved: '…/py_scene/s_.png'           (from animation=False, write_still=True)
Saved: '…/node_out/nd_0001.png'      (from animation=True)
Saved: '…/py_scene/s_0001.png'       (from animation=True)
```

So in 4.5 the File Output node writes on all three, and its name carries the frame
number every time.

Two things in that are worth stating plainly. A single frame started with `-f` on
the command line writes the same names an animation does, frame number and all. And
a still saved by `write_still=True` gets **no** frame number — the file was
`s_.png`, not `s_0001.png`.

**Entry 21 — the viewport route with no window.** All three shapes of
`bpy.ops.render.opengl` were tried:

```
--- calling bpy.ops.render.opengl(**{'write_still': True, 'view_context': True})
Error: Cannot use OpenGL render in background mode (no opengl context)
   raised: RuntimeError Error: Cannot use OpenGL render in background mode (no opengl context)
--- calling bpy.ops.render.opengl(**{'write_still': True, 'view_context': False})
Error: Cannot use OpenGL render in background mode (no opengl context)
--- calling bpy.ops.render.opengl(**{'animation': True, 'view_context': False})
Error: Cannot use OpenGL render in background mode (no opengl context)
listing: no folder
```

Nothing was written, on any of the three.

**Entry 12 — an animation rendered to a video.** With the format set to an FFmpeg
Matroska video and frames 1 to 5, one file was written and it was called
`mov_0001-0005.mkv`. With `##` typed into the recorded name the hashes were left
alone and the file was called `mov_##.mkv`.

**Entry 23 — switching automatic saving off.** There is no such switch in 4.5 — see question 5 above for the exact words.

*Standing lines, brought up to date.* **Driven from outside** — the command-line and
Python routes are settled by the runs above. The viewport route is settled too, in
the other direction: it refuses to run with no window. What the menu entries do is
still open and sits with the Creator (entries 26 to 28 of the hand-test list).
**Operating system** — settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 4.5.12**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 26 — what the Render menu holds, and what each entry calls.**

*What was done, to what.* Blender was opened at its normal startup file, the Render
menu at the top of the window opened, and the pointer rested on each entry in turn
so its tooltip appeared. Six screenshots per version, kept at
`/Users/artist/Desktop/Screenshots/023/`, one folder per version.

*What came back.* The menu holds the same six entries in all four versions, in this
order:

```
Render Image        F12
Render Animation    ⌘ F12
------------------------
Render Audio...
------------------------
View Render         F11
View Animation      ⌘ F11
------------------------
[ ] Lock Interface
```

The tooltips, read from the screenshots:

```
Render Image      Render active scene.
                  Python: bpy.ops.render.render(use_viewport=True)

Render Animation  Render active scene.
                  Python: bpy.ops.render.render(animation=True, use_viewport=True)

Render Audio...   Mix the scene's audio to a sound file.
                  Python: bpy.ops.sound.mixdown()

View Render       Toggle show render view.
                  Python: bpy.ops.render.view_show()

View Animation    Play back rendered frames/movies using an external player.
                  Python: bpy.ops.render.play_rendered_anim()

Lock Interface    Lock interface during rendering in favor of giving more memory
                  to the renderer.
                  Python: RenderSettings.use_lock_interface
                          bpy.data.scenes["Scene"].render.use_lock_interface
```

Two things to be exact about. **Render Image passes no `write_still` at all**, and
neither does Render Animation — the only arguments either menu entry passes are the
ones shown. And the menu carried **no** Sequencer render entries in any of the four.
The entry did not ask for a sequencer scene to be set up, so this run says nothing
about whether such entries appear when one exists.

All six tooltips were read from the 4.5 screenshots. Four of the six were read from
the 5.0 set and two from each of the 5.1 and 5.2 sets; every one of those matched
the 4.5 wording. The rest of the screenshots are in the folder named above and were
not opened.

**Entry 27 — does pressing F12 write a file?**

*What was done, to what.* A prepared file per version, with its output path pointing
at an empty folder beside it and a one-frame range, was opened with the matching
Blender. F12 was pressed, the render window closed, and the folder looked at; then
Ctrl-F12 was pressed and the folder looked at again.

*What came back*, in the Creator's own words:

> "F12 saves nothing. Ctrl+F12 saves one file one file called img_0001.png.
> Consistent across all versions."

**Entry 28 — the 3D Viewport's own render entries.**

*What was done, to what.* The same prepared file, with its output folder emptied
first. The View menu in the 3D window was opened, the pointer rested on each
render entry, then the still one was clicked and the folder looked at, then the
animation one and the folder looked at again. Three screenshots per version, kept
at `/Users/artist/Desktop/Screenshots/019/`, one folder per version.

*What came back.* The three entries are named, in this version:

```
Viewport Render Image
Viewport Render Animation
Viewport Render Keyframes
```

and their tooltips:

```
Viewport Render Image       Take a snapshot of the active viewport.
                            Python: bpy.ops.render.opengl()

Viewport Render Animation   Render the viewport for the animation range of this scene.
                            Python: bpy.ops.render.opengl(animation=True)

Viewport Render Keyframes   Render the viewport for the animation range of this scene,
                            but only render keyframes of selected objects.
                            Python: bpy.ops.render.opengl(animation=True,
                                                          render_keyed_only=True)
```

All three tooltips were read from the screenshots for this version.

and, in the Creator's own words, for what they wrote:

> "Same as test 18 — still writes nothing, an animation (or playblast or whatever
> it's called in some versions) writes one file called img_0001.png"

"Test 18" is the Creator's own number for entry 27 above. So the still entry wrote
nothing and the animation entry wrote one file. The entry also asked what the
keyframes-only variant does; that was not reported, and is not filled in here.

Note that the menu entries are named differently in 4.5 from the other three, and
that neither of the two operator calls passes `write_still`.

**Entry 29 — the switch in the Output panel's header.**

*What was done, to what.* Blender opened at its normal startup file, Properties ‣
Output tab, the panel headed Output, and the pointer rested on the header strip of
that panel.

*What came back*, in the Creator's own words:

> "This checkbox only exists in 5.2. Its tooltip says "Save Output / Write frames to
> disk for animation renders.""

So there is no such checkbox in this version.

The entry also asked for the labels of every checkbox inside the panel, and for the
`Python:` line at the end of the tooltip. Neither was reported, and neither is
filled in here.

*Standing lines, brought up to date.* **Driven from outside** — nothing here bears on it: every one of these was done at Blender's own window, which is the half the terminal cannot reach. **Operating system** — the Creator works on a Mac, so this settles macOS and leaves Windows and Linux open.


### Observed — hand tests run on a rented Linux machine, 2026-08-06

**Entry 22 — what the command line and the Python call each write.** A blend-file
holding both a scene output and a File Output node was rendered three ways. A single
frame from the command line wrote one scene image and one node image; an animation
wrote the same; and the three shapes of the Python call gave:

```
--- animation=False, write_still=False   → Saved: '…/node_out/nd_0001.png'
--- animation=False, write_still=True    → Saved: '…/node_out/nd_0001.png'
                                           Saved: '…/py_scene/s_.png'
--- animation=True                       → Saved: '…/node_out/nd_0001.png'
                                           Saved: '…/py_scene/s_0001.png'
```

**Entry 21 — the viewport route with no window.** Refused, in all three shapes, with
the same words as on the Mac and nothing written:

```
Error: Cannot use OpenGL render in background mode (no opengl context)
```

**Entry 27 — the viewport route with a real window.** This is the half the Mac could
never reach from a terminal. The rented machine has a display of its own, so Blender
was started **with a window**, driven by a timer, and asked for the same three calls.
All three succeeded:

```
window count: 1
engine: BLENDER_EEVEE_NEXT
--- bpy.ops.render.opengl(write_still=True, view_context=False)
   returned: {'FINISHED'}
--- bpy.ops.render.opengl(animation=True, view_context=False)
   returned: {'FINISHED'}
--- bpy.ops.render.opengl(write_still=False, view_context=False)
   returned: {'FINISHED'}
listing: ['vp_.png', 'vp_0001.png']
```

and on the terminal:

```
Saved: '…/vp/vp_.png'
OpenGL Render written to '…/vp/vp_.png'
Saved: '…/vp/vp_0001.png'
```

So with a window the viewport route writes: told to save a still it writes one file
with **no frame number**, an animation writes one file **with** the frame number, and
told not to save it writes nothing. The refusal recorded above is about having no
window, not about the route itself.

**Entry 12 — a video.** One file holding the whole frame range in its name,
`mov_0001-0005.mkv`, the same as on the Mac.

*The two machines.* Entries 22, 21 and 12 were run on both and **agreed**. Entry 27
was run on Linux only; nothing like it was run on the Mac.

*Standing lines, brought up to date.* **Driven from outside** — the command-line and
Python routes were all driven from outside with no window; the viewport route is the
one that needs a window, and it works when there is one. **Operating system** — now
settled on two systems, macOS and Linux, which agreed on everything run on both.
Windows is untried.


## 7. If a render can be set up so that it writes no files, what does it leave behind?

**Answer.** Yes, it can — a still render that is not told to save writes nothing. That much is **inferred** rather than stated (see question 6: `write_still` "Save the rendered image to the output path"; and "rendering stills will allow you to view and save the image from the render buffer when it is complete" — neither says a still render writes no file). What it leaves behind is an image held inside Blender, called the Render Result. *(Citation check, 2026-08-05: marked inferred; it had been stated flatly.)*

> These images are the "Render Result" and "Viewer" image data-blocks

— manual, https://docs.blender.org/manual/en/4.5/render/color_management.html, read 2026-07-30.

It is reachable in the Image Editor, with a set of selectors alongside:

> Render Result: displays renders. When this item is selected, the Slot, View Layer, and Render Pass selectors become available (see below).

> Slot — The render slot to view (and render to). You can create new renders without losing previous ones by selecting an empty slot before rendering. Afterwards, you compare them by pressing J and Alt-J to cycle forwards and backwards. Alternatively, you can use the number keys 1, 2, 3 etc. to select the slot with the corresponding number.

> View Layer — The View Layer to display.
>
> Render Pass — The Render Pass to display.

— manual, https://docs.blender.org/manual/en/4.5/editors/image/introduction.html, read 2026-07-30.

And in its own window: "View Render F11 — Show the Render window. (Press again to switch back to the main Blender window.)" — manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html, read 2026-07-30.

Two things can turn "writes no files" back into files, and both are opt-in:

> Cache Result — Saves the rendered view layers and their passes to a multi-layer OpenEXR image. The Compositor can then use this file to improve performance, especially for heavy compositing. The image is stored in the Render Cache folder as specified in the File Paths Preferences. You can also load it back into the Image Editor's Render Result, even after closing and reopening Blender

— manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30. (`use_render_cache` — "Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)", default False — API RenderSettings.)

And a File Output node in the compositor writes regardless of the render output path: "This node writes out an image, for each frame range specified, to the filename entered […] The image(s) will be saved on rendering" — manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html, read 2026-07-30.

**Something else may be left on the drive.** The manual says of the temporary directory:

> The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs).

— manual, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html, read 2026-07-30. That page also gives the order in which the temporary directory is chosen: "User Preference (see File Paths). / Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms). / The /tmp/ directory."

**Label.** documented, for the Render Result, the slots, the render window, and Cache Result. **Inferred** for the temporary directory: step taken — the page lists "render layers" among the things stored there at run time, from which I take it that a render can leave files there even when it writes no output files; the source does not say when they appear, how big they are, or whether they are removed afterwards.

**Not covered.** Whether the Render Result survives quitting Blender (only the Cache Result is described as reloadable "even after closing and reopening Blender"). Whether the Render Result exists at all when Blender runs in background mode, where there is no Image Editor and no render window — the sources do not say. How many render slots there are.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Everything left behind that the sources describe — the Image Editor's Render Result, the slots, the render window — is described in interface terms. The sources do not say what a still render leaves behind when Blender is driven from outside its window. The temporary-directory sentence is the only part that is not interface-bound.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The temporary directory differs by system, and the sources say so: the environment variables consulted are "TEMP on Windows, TMP & TMP_DIR on other platforms", falling back to "/tmp/". The preferences page adds "The path must reference an existing directory or it will be ignored and the systems temporary directory will be used instead. When left blank, the systems temporary directory will be used." Nothing else in this answer is said to vary by system.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 20 — a windowless render told not to save.**

*What was done, to what.* At factory settings, with no window, the render cache and
placeholders switched off, the output path pointed at an empty folder, and
`bpy.ops.render.render(animation=False, write_still=False)` called. The image
data-blocks were listed before and after, and the temporary folder and the output
folder were listed afterwards.

*The command.*

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup --python q20.py -- …
```

*What came back.*

```
images before: [('Render Result', 'RENDER_RESULT'), ('Viewer Node', 'COMPOSITING')]
returned: {'FINISHED'}
images after : [('Render Result', 'RENDER_RESULT', (0, 0)), ('Viewer Node', 'COMPOSITING', (256, 256))]
tempdir listing: []
output folder listing: []
```

So a Render Result data-block **is** there with no window — it is there before the
render as well as after — and it reports a size of zero. Nothing was written to the
output folder, and nothing was left in Blender's own temporary folder, which was
empty both before and after.

**Entry 33 — whether a Cycles render leaves tiles behind.** A 4000 by 4000 pixel
Cycles render with the tile size set to 512 was run, with the temporary folder
listed before, during (from the per-frame handler) and after:

```
tile_size: 2048
use_auto_tile: True
tile_size set to 512
tempdir before: []
   during render, tempdir: []
returned: {'FINISHED'}
tempdir after : []
output listing: ['img_0001.png']
```

Nothing appeared in the temporary folder at any point.

**Entry 23 — switching automatic saving off.** There is no switch for turning automatic saving off in 4.5 — see question 5.

*Standing lines, brought up to date.* **Driven from outside** — settled. The Render
Result exists in a windowless Blender and can be reached from a script, and a render
told not to save leaves nothing at all on disk. What becomes of it after the process
ends was not tested and cannot be: the process is gone. **Operating system** —
settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 4.5.12**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 27 — does pressing F12 write a file?**

*What was done, to what.* A prepared file per version, with its output path pointing
at an empty folder beside it and a one-frame range, was opened with the matching
Blender. F12 was pressed, the render window closed, and the folder looked at; then
Ctrl-F12 was pressed and the folder looked at again.

*What came back*, in the Creator's own words:

> "F12 saves nothing. Ctrl+F12 saves one file one file called img_0001.png.
> Consistent across all versions."

**Entry 29 — the switch in the Output panel's header.**

*What was done, to what.* Blender opened at its normal startup file, Properties ‣
Output tab, the panel headed Output, and the pointer rested on the header strip of
that panel.

*What came back*, in the Creator's own words:

> "This checkbox only exists in 5.2. Its tooltip says "Save Output / Write frames to
> disk for animation renders.""

So there is no such checkbox in this version.

The entry also asked for the labels of every checkbox inside the panel, and for the
`Python:` line at the end of the tooltip. Neither was reported, and neither is
filled in here.

*Standing lines, brought up to date.* **Driven from outside** — nothing here bears on it: every one of these was done at Blender's own window, which is the half the terminal cannot reach. **Operating system** — the Creator works on a Mac, so this settles macOS and leaves Windows and Linux open.


### Observed — hand tests run on a rented Linux machine, 2026-08-06

**Entry 20 — a windowless render told not to save.**

```
temp dir: /tmp/blender_XXXXXX/
returned: {'FINISHED'}
images after : [('Render Result', 'RENDER_RESULT', (0, 0)), ('Viewer Node', 'COMPOSITING', (256, 256))]
tempdir listing: []
output folder listing: []
```

Nothing on disk, nothing in Blender's temporary folder, and the Render Result held
inside Blender reporting a size of zero.

**Entry 23 — switching automatic saving off.** ```
has save_output: False
could not set save_output: AttributeError 'RenderSettings' object has no attribute 'save_output'
animation returned: {'FINISHED'}
listing: ['img_0001.png']
```

So there is no such switch in this version, and the animation wrote its file as
usual.

**Entry 24 — a large Cycles render on the processor.** Tile size forced to 512 on a
4000 by 4000 frame:

```
tile_size: 2048
use_auto_tile: True
tile_size set to 512
tempdir before: []
   during render, tempdir: []
tempdir after : []
output listing: ['img_0001.png']
```

Nothing in the temporary folder before, during or after.

**Entry 26 — the same on the graphics card, which only this machine can do.** Cycles
was pointed at the NVIDIA card and the processor switched off:

```
device: NVIDIA GeForce RTX 4090 CUDA use = True
device: AMD Ryzen Threadripper PRO 3975WX 32-Cores CPU use = False
device: NVIDIA GeForce RTX 4090 OPTIX use = True
cycles device: GPU
tempdir before: []
tempdir after : []
output listing: ['img_0001.png']
```

So a Cycles render on an NVIDIA card leaves nothing in the temporary folder either.

**Entry 27 — one thing a windowed Blender does leave behind.** When the windowed run
was told to quit, it printed:

```
Saved session recovery to "/tmp/quit.blend"
```

So a Blender started **with** a window writes a recovery copy of the open file on
quitting. No such line appeared in any windowless run.

*The two machines.* Entries 20, 23 and 24 were run on both and **agreed** in this
version, including the list of images held in memory. Entries 26 and 27 were run on
Linux only.

*Standing lines, brought up to date.* **Driven from outside** — entries 20, 23, 24
and 26 all had no window. **Operating system** — settled on two systems, macOS and
Linux, which agreed on everything run on both. The recovery file on quitting is
settled for a windowed Blender on Linux only. Windows is untried.


## 8. How does the place a file records for writing become the name of a file on disk? Say where a frame's number appears in it, and what decides the end of the name.

**Answer.**

**The frame number.** By default it goes on the end; with `#` marks it goes wherever they are.

> When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of `#` anywhere in the file name (e.g. image_##_test.png translates to image_01_test.png).

— manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30.

> The `#` characters are replaced by the frame number, and used to define zero padding.
>
> animation_##_test.png becomes animation_01_test.png
>
> test-######.png becomes test-000001.png
>
> When the filename does not contain `#`, the suffix `####` is added to the filename.
>
> The frame number will be added at the end of the filename, eg:
>
> `blender -b animation.blend -o //render_ -F PNG -x 1 -a`
>
> `//render_` becomes `//render_####`, writing frames as `//render_0001.png`

— manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30. That last worked example is the clearest single statement of the whole chain: a path with no name and no marks becomes name + four-digit number + extension.

**What decides the end of the name.** The file extension, added or not according to one switch:

> File Extensions — Adds the correct file extensions per file type to the output files.

— manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30.

> `use_file_extension` — Add the file format extensions to the rendered file name (eg: filename + .jpg) — Type: boolean, default True
>
> `file_extension` — The file extension used for saving renders — Type: string, default "", (readonly, never None)

— API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30.

From the command line the same switch is `-x, --use-extension <bool>` — "Set option to add the file extension to the end of the file." (manual, arguments page). Which extension each format gets is tabulated in the manual's Supported Graphics Formats page (https://docs.blender.org/manual/en/4.5/files/media/image_formats.html, read 2026-07-30) — BMP `.bmp`, Iris `.sgi .rgb .bw`, PNG `.png`, JPEG `.jpg .jpeg`, and so on down the table.

**Templates are substituted on the way.** "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field." — manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html, read 2026-07-30.

**A view name may also be inserted.** For multi-view renders each view carries `file_suffix` — "Suffix added to the render images for this view" — and `camera_suffix` — "Suffix to identify the cameras to use, and added to the render images for this view" (API, https://docs.blender.org/api/4.5/bpy.types.SceneRenderView.html, read 2026-07-30). The path resolver takes a view for the same purpose (see the `%` quote in question 3).

**The whole expansion has a name.** The API exposes the resolver:

> `frame_path(*, frame=-2147483648, preview=False, view='')`
>
> Return the absolute path to the filename to be written for a given frame
>
> `frame (int in [-inf, inf], (optional))` – Frame number to use, if unset the current frame will be used
>
> `preview (boolean, (optional))` – Preview, Use preview range
>
> `view (string, (optional, never None))` – View, The name of the view to use to replace the "%" chars
>
> Returns: File Path, The resulting filepath from the scenes render settings

— API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30.

**Observed, from Cycles' own files.** Cycles reproduces render file naming by calling that resolver rather than assembling names itself. In `Blender Sources/cycles_4.5/operators.py`, inside `CYCLES_OT_denoise_animation.execute`, lines 74–97:

```
        else:
            # Denoise animation sequence with expanded frames matching
            # Blender render output file naming.
            in_filepath = scene.render.filepath
            if out_filepath == '':
                out_filepath = in_filepath

            # Backup since we will overwrite the scene path temporarily
            original_filepath = scene.render.filepath

            for frame in range(scene.frame_start, scene.frame_end + 1):
                scene.render.filepath = in_filepath
                filepath = scene.render.frame_path(frame=frame)
                in_filepaths.append(filepath)
```

Label for this: **observed** — that is what the file says. Taking from it that `frame_path` is the authoritative way to learn the name a render will write is a step beyond the text, so that part is **inferred**; the step is that a component shipped with Blender describes its own use of `frame_path` as "matching Blender render output file naming", which only makes sense if `frame_path` produces the same names the renderer does.

**For the File Output node**, the manual gives the assembly directly: "by default the output path is composed of: `{base path}/{file name}{frame number}.{extension}`" — https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html, read 2026-07-30.

**Label.** documented, except where marked inferred or observed above.

**Not covered.** The exact order when several of these apply at once — for instance whether a multi-view suffix lands before or after the frame number, and whether it lands before or after the extension. Whether the trailing part of the recorded string is treated as a folder or a name-stem, and what makes the difference. What happens to the extension when the recorded name already ends in one and File Extensions is on. What name a video (rather than a frame sequence) file gets, and what Autosplit names the second and later parts.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The `-o` documentation, which is the outside-the-interface route, carries the same `#` rules and the same worked example, and `-x` is the same File Extensions switch, so the naming appears to be identical. `frame_path` is a Python call and works wherever Python does. Nothing here is described as interface-only.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say the naming differs by operating system. Two adjacent facts are worth having: the length limits (file-name 255, file-path 1023, with multi-byte characters counting for more than one) on https://docs.blender.org/manual/en/4.5/advanced/limits.html, and `bpy.path.resolve_ncase` — "Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path" (API, https://docs.blender.org/api/4.5/bpy.path.html) — which shows the documentation is aware that systems differ on letter case. Neither is stated to apply to render output naming.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 10 — the names a render actually writes.**

*What was done, to what.* At factory settings, with no window, frames 1 and 2 were
rendered to PNG for each of seven recorded strings in turn.

*What came back on disk.*

```
…/a_plain/img_0001.png            from  a_plain/img_
…/a_plain/img_0002.png
…/b_mid_hash/img_01_v2.png        from  b_mid_hash/img_##_v2
…/b_mid_hash/img_02_v2.png
…/c_dir_hash/seq_##/img_0001.png  from  c_dir_hash/seq_##/img_
…/c_dir_hash/seq_##/img_0002.png
…/d_typed_ext/img_0001.png        from  d_typed_ext/img_####.png
…/d_typed_ext/img_0002.png
…/e_no_ext_sw/img_0001            from  e_no_ext_sw/img_ with File Extensions off
…/e_no_ext_sw/img_0002
…/f_percent/img_%_0001.png        from  f_percent/img_%_
…/f_percent/img_%_0002.png
…/g_trailing/0001.png             from  g_trailing/
…/g_trailing/0002.png
```

Four things fall out. A run of `#` in the *name* is replaced in place, padded to its
own width. A run of `#` in a *folder* name is left alone, and a folder literally
called `seq_##` was made. An extension already typed at the end is not doubled — a
name ending `.png` with File Extensions on came out with one `.png`. And a path
ending in a separator gives a file whose whole name is the frame number.

**Entry 9 — the same asked of the path builder.** Covered in full under question 3
above. Worth repeating here: `/tmp/out_.png` came back as `/tmp/out_.png0007.png`,
so an extension typed in the middle of a stem *is* doubled where a matching one at
the very end is not; and an empty recorded path came back as the bare `0007.png`.

**Entry 34 — the same strings with the extension switch off.** Covered in full under
question 3 above. What matters here is what the switch does to the end of the name:
with it off nothing is added at all, so `/tmp/out_` came back as `/tmp/out_0007` and
an empty recorded path as the bare `0007`. A name that already ends in `.png` is the
one case where both switch positions agree — `/tmp/out_##_x.png` came back as
`/tmp/out_07_x.png` either way — while `/tmp/out_.png`, whose `.png` sits in the
middle of the stem, came back as `/tmp/out_.png0007.png` with the switch on and
`/tmp/out_.png0007` with it off.

**Entry 13 — frame numbers past four digits, and below zero.**

```
frame_path(99999): '…/wide_none/img_99999.png'
frame_path(100001): '…/wide_none/img_100001.png'
Saved: '…/wide_none/img_99999.png'
Saved: '…/wide_none/img_100000.png'
Saved: '…/wide_none/img_100001.png'

frame_path(12344): '…/wide_two/img_12344.png'      from  wide_two/img_##
frame_path(12345): '…/wide_two/img_12345.png'
Saved: '…/wide_two/img_12344.png'
Saved: '…/wide_two/img_12345.png'

frame_path(-2): '…/negative/img_-0002.png'
frame_path(-1): '…/negative/img_-0001.png'
Saved: '…/negative/img_0000.png'
```

So the padding is a minimum, not a maximum: a number too wide for the run of `#`
simply takes the room it needs. The negative case is worth recording exactly as it
came: the path builder gave `img_-0002.png` and `img_-0001.png`, and the render
wrote one file called `img_0000.png`.

**Entry 12 — a video.** With the format an FFmpeg Matroska video and frames 1 to 5,
`frame_path(frame=1)` gave `…/plain/mov_0001-0005.mkv` and that is the file that
appeared — one file, with the whole frame range in its name. With `##` typed into
the name, `frame_path` gave `…/hashes/mov_##.mkv` and that is the name that
appeared, hashes and all.

**Entry 11 — where a view's ending lands.** After the frame number and before the
extension: `img_0001_L.png` and `img_0001_R.png`. See question 3 above for the
whole run.

**Entry 14 — the order in which the marks are applied.** With a template whose value
carries a `#` — `{blend_name}`, with the file saved as `da##ce.blend`:

```
blend name: da##ce.blend
frame_path: '…/tmpl/da07ce_.png'
Saved: '…/tmpl/da01ce_.png'
```

So the template goes in first, and the hashes it brings are then read as a place for
the frame number.

**Entry 16 — how a File Output node's name is built.** With the node's folder
`…/node_out/`, its file name `nd_` and one input named `Image`:

```
Saved: '…/node_out/nd_0001.png'
```

so in 4.5 the shape is the node's base path, then the slot's own piece of path,
then the frame number, then the extension. The input socket's name does not appear
separately.

*Standing lines, brought up to date.* **Driven from outside** — every name above was
produced with no window at all. **Operating system** — settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 4.5.12**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 30 — what a File Output node shows on its face.**

*What was done, to what.* A file holding a File Output node wired to a Render Layers
node was opened with the matching Blender, the Compositing workspace shown, and the
node looked at; then the node selected and the sidebar opened. Two screenshots per
version, kept at `/Users/artist/Desktop/Screenshots/010/`, one folder per version.

*One thing to record plainly.* The file used was not the one this entry named. It was
a file prepared for another topic — its node's own Output Paths panel points into that
topic's folder — and it carries three inputs in 4.5 and two in the other three rather
than the one the entry described. What is written below is what the screenshots show.

*What is on the node's face.* One label, `Base Path:`, above one text field
holding `//out/` with a folder-browse button. Below it, one row per input, each row
showing the input's name on the left and its format on the right:

```
File Output
  Base Path:
  [ //out/                    ]
  ○ Image           PNG
  ○ passes/di…      PNG
  ○ beauty          PNG
```

*What is in the sidebar.* A **Node** panel holding Name (`File Output`), Label
(empty) and Color. Then a **Properties** panel holding, in this order:

```
Base Path:  [ //out/ ]
            [x] Save as Render
File Format         PNG
Color               BW | RGB | RGBA        (RGBA chosen)
Color Depth         8 | 16                 (8 chosen)
Compression         15%
Color Management    Follow Scene
[ + Add Input ]
  Image
  passes/diffuse
  beauty                                   (selected)
File Subpath:  [ beauty ]
Format:  [x] Use Node Format
```

So in this version the node records one path field, and each input carries its own
**File Subpath** and its own choice of whether to use the node's format. There is no
panel showing finished file paths.

*Standing lines, brought up to date.* **Driven from outside** — nothing here bears on it: every one of these was done at Blender's own window, which is the half the terminal cannot reach. **Operating system** — the Creator works on a Mac, so this settles macOS and leaves Windows and Linux open.


### Observed — hand tests run on a rented Linux machine, 2026-08-06

**Entry 10 — what names a render actually writes.** Seven recorded strings, two
frames each:

```
…/a_plain/img_0001.png          …/a_plain/img_0002.png
…/b_mid_hash/img_01_v2.png      …/b_mid_hash/img_02_v2.png
…/c_dir_hash/seq_##/img_0001.png
…/d_typed_ext/img_0001.png
…/e_no_ext_sw/img_0001          (File Extensions off — nothing added)
…/f_percent/img_%_0001.png
…/g_trailing/0001.png
```

A run of hashes in a **folder** name is left alone; a folder literally called
`seq_##` was made.

**Entry 13 — frame numbers wider than the padding.** `img_99999.png`,
`img_100000.png` and `img_100001.png` with no hash marks at all, and `img_12344.png`
and `img_12345.png` against two hash marks. So the padding is a minimum, not a
maximum. Asked for frame minus two the path-building call gave `img_-0002.png` while
the render wrote `img_0000.png`.

**Entry 11 — where a view's own ending lands.** After the frame number and before the
extension, whether a `%` is in the name or not: `img_%_0001_L.png`,
`img_0001_L.png`, `img_%_001_R.png`.

**Entry 12 — a video.** `mov_0001-0005.mkv`; with hash marks typed in, the marks are
left in the name untouched.

**Entry 9 — what decides the end of the name.** The File Extensions switch plus the
chosen format, and an extension already typed at the very end is not doubled while
one in the middle of a stem is: `/tmp/out_.png` became `/tmp/out_.png0007.png`.

**Entry 18 — the File Output node's own name.** In this version the node wrote
`nd_0001.png`.

*The two machines.* All of these were run on both. **They agreed** in this version,
name for name.

*Standing lines, brought up to date.* **Driven from outside** — every run had no
window. **Operating system** — now settled on two systems, macOS and Linux, which
agreed. Windows is untried.


## 9. Can the place a file output node writes to be settled for a render — at the moment the render is started, and by writing it into the file from outside Blender?

**A note on the question.** I read the second half — "by writing it into the file from outside Blender" — as: can the value be put into the blend-file by something other than a person clicking in Blender's window. The other reading would be: can the blend-file be edited by a tool that is not Blender at all. I answer the first and say what I found for the second.

**Answer, first half — at the moment the render is started.**

There is **no command-line argument for it**. I read the whole of the Command Line Arguments page — Render Options, Cycles Render Options, Format Options, Animation Playback, Window, Python, Network, Logging, Debug, GPU, Misc, Other, Argument Parsing, Environment Variables — and the only output-path argument is `-o, --render-output <path>`, documented as "Set the render path and file name" (the scene's path). I also searched all three sources for `base_path`, `File Output`, `OutputFile`, `compositor` within the command-line pages: nothing. Cycles adds no path arguments of its own either; its own file `Blender Sources/cycles_4.5/engine.py`, lines 8–21, defines exactly two, `--cycles-print-stats` and `--cycles-device` (**observed**).

There is, however, a documented way for the node's path to come out different at render time without anyone editing it — path templates, which this version enables on precisely this node:

> Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases.

— manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html, read 2026-07-30.

> Currently this is only enabled on the Scene's render output path and the output paths of the compositor's File Output node.

— release notes, https://developer.blender.org/docs/release_notes/4.5/core/, read 2026-07-30.

Both the node's fields are marked for it in the API: `base_path` … "(never None, blend relative // prefix supported, Supports template expressions)" and the slot's `path` … "(never None, Supports template expressions)" (https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html and https://docs.blender.org/api/4.5/bpy.types.NodeOutputFileSlotFile.html, read 2026-07-30). And the substitution is stated to happen "at time of use (e.g. while rendering)".

The remaining route at start-up is a script run before the render: `-P, --python <filepath>` and `--python-expr <expression>` (manual, arguments page), with "Arguments are executed in the order they are given!" (manual, command-line render page), acting on the writable properties `base_path` and `file_slots[…].path`.

**Answer, second half — from outside Blender.** The two properties are ordinary writable strings in the Python API (quoted above; neither is marked readonly), and Blender can be run with no window at all — "`-b, --background` — Run in background (often used for UI-less rendering)". The sources describe no way to change a blend-file's contents without running Blender.

**Label.**
- That there is no command-line argument for the node's path: **not in the sources**, in the strict sense that no such argument is documented — the record of where I looked is above. I am not claiming the sources say one does not exist; I am claiming none is documented.
- That path templates apply to this node and are substituted while rendering: **documented**.
- That a script run at start-up can set the node's path for that render, and that a script can set it and save the file: **inferred**. Step taken: the sources document (i) that these two properties are writable strings, (ii) that `-P` / `--python-expr` run Python at launch, and (iii) that arguments run in the order given, so a script placed before `-a` runs before the render. Putting those three together is my step; no page says it.

**Not covered.** Whether `-o` has any effect on a File Output node (I found no statement either way). Whether the node writes on every kind of render. What happens to a node whose base path has a template error — the Errors section speaks of "the render output path" specifically, and says only that "Depending on the path, template errors may prevent certain actions." Whether the node's slot subpaths can contain `#`.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The whole answer is about running Blender from outside its window, and the mechanisms named — background mode, Python at start-up, templates substituted at time of use — are all documented without reference to a window. The one part that is interface-only is how template errors are shown (red field, tooltip).

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say any of this differs by operating system.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 16 — does `-o` reach a File Output node?**

*What was done, to what.* A file was saved holding a scene output path pointing at
one folder and a File Output node named `FileOut` pointing at another. Blender was
started with no window, given that file, then `-o` pointing at a third folder, then
`-f 1`. Afterwards the file was reopened and the node's stored values printed.

*The command.*

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup …/node.blend -o …/cli_out/cli_ -F PNG -x 1 -f 1
```

*What came back.*

```
Executing compositor
Saved: '…/node_out/nd_0001.png'
Saved: '…/cli_out/cli_0001.png'
exit code: 0
```

and reopening the file gave the node's stored values unchanged. So `-o` moves the
scene's own output and leaves the node where it was.

**Entry 17 — setting the node's place at the moment the render starts.**

*The command.*

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup …/node.blend --python-expr "import bpy; sc=bpy.context.scene; t=sc.node_tree; n=t.nodes['FileOut']; n.base_path='…/from_cli/'; n.file_slots[0].path='zz_'; print('set to', n.base_path, n.file_slots[0].path)" -f 1
```

*What came back.*

```
set to …/from_cli/ zz_
Saved: '…/from_cli/zz_0001.png'
exit code: 0
```

So yes: the node's `base_path` and `file_slots[0].path` are ordinary writable values, a script
given at launch reaches them, and the render that follows uses the new ones.

**Entry 18 — writing the node's place into the file, with no window.** The same
expression with `bpy.ops.wm.save_mainfile()` added and no render:

```
Info: Saved "edit.blend"
saved
exit code: 0
```

and reopening the file afterwards gave:

```
node base_path: '/tmp/written_in/' slot paths: ['ww_']
```

**Entry 19 — whether `bpy.data.file_path_foreach` reaches the node's place.**

```
has file_path_foreach: False
loaded control image: '…/cli_out/cli_0001.png'
raised: AttributeError 'BlendData' object has no attribute 'file_path_foreach'
count: 0
```

There is no such call in 4.5.

**Entry 4 — the node's missing folder.** Answered under question 4 above: the node
makes its own missing folders, more than one level deep.

*Standing lines, brought up to date.* **Driven from outside** — settled, and settled
in the affirmative on both halves of the question: the place can be set for a
single run by a launch-time script, and it can be written into the file by a
windowless Blender and saved. There is still no command-line argument that names the
node. Whether a `.blend` can be changed by a tool that is not Blender remains
untested; nothing here bears on it. **Operating system** — settled for macOS only.


### Observed — hand tests run on a rented Linux machine, 2026-08-06

**Entry 18 — does the output argument reach the node?** No. Pointing it at a third
folder moved the scene's own output there and left the node writing into its own
stored folder:

```
Saved: '…/node_out/nd_0001.png'
Saved: '…/cli_out/cli_0001.png'
```

and the node's stored values read back unchanged afterwards.

**Entry 19, part one — setting the node's place at the moment the render starts.** A
one-line script handed to Blender at launch set the node's folder and name, and the
render that followed used them:

```
set to …/from_cli/ zz_
Saved: '…/from_cli/zz_0001.png'
```

**Entry 19, part two — writing it into the file from outside.** A windowless Blender
set the same two values and saved the file; reopened, the values were still there:

```
Info: Saved "edit.blend"
node base_path: '…/written_in/' slot paths: ['ww_']
```

**Entry 19, part three — the call that walks every file path a blend-file holds.**

```
has file_path_foreach: False
raised: AttributeError 'BlendData' object has no attribute 'file_path_foreach'
count: 0
```

*The two machines.* All of these were run on both. **They agreed** in this version.

*Standing lines, brought up to date.* **Driven from outside** — this is the whole of
what was tested: every run was driven from outside with no window. **Operating
system** — now settled on two systems, macOS and Linux, which agreed. Windows is
untried.


## 10. Can the place a render records for writing carry characters that the computer it lands on will not accept, and what does a render do when it does?

**Answer, first half.** The field is an unrestricted string as far as the documentation goes. `RenderSettings.filepath` is "string, default "", (never None, blend relative // prefix supported, Supports template expressions)" (API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30) — no character set, no rejected characters, no validation described. The only limits given anywhere are lengths: "directory: 767 / file-name: 255 / file-path: 1023", with "Multi-byte encoding means some Unicode characters use more than a single ASCII character" (manual, https://docs.blender.org/manual/en/4.5/advanced/limits.html, read 2026-07-30). The only characters given special treatment are the ones in question 3, and of those only the braces are ever described as invalid — an unclosed one, which is Blender's own rule, not the computer's.

**Answer, second half. Not in the sources.** Nothing in the manual, the API documentation, the release notes or Cycles' files says what a render does when the path holds a character the operating system will not take.

**Where I looked.** Whole-corpus searches for: `invalid character`, `illegal character`, `special character`, `forbidden character`, `unsupported character`, `sanitiz`, `reserved name`, `valid filename`, `file name.*character`, `invalid path`, `path is invalid`, `malformed`, `replaced with an underscore`, `rename the file`, `colon`, `asterisk`, `question mark`, `wildcard character`, `backslash`, `path separator`, `forward slash`, `case sensitive`, `utf-8`, `utf8`. Pages read: the Output panel page, the File Paths / path templates page (including its Errors section and both pictures), the Relative Paths section of Opening & Saving, the Working Limits page, the command-line arguments page, the `bpy.path` module page, and the `RenderSettings` API page.

**What the searches turned up, and what each is worth:**

- `bpy.path.clean_name(name, *, replace='_')` — "Returns a name with characters replaced that may cause problems under various circumstances, such as writing to a file. All characters besides A-Z/a-z, 0-9 are replaced with "_" or the replace argument if defined." — API, https://docs.blender.org/api/4.5/bpy.path.html, read 2026-07-30. This shows the documentation acknowledges that some characters "may cause problems […] when writing to a file". But it is a helper a script must call for itself. Nothing says the render output path is passed through it. Taking this as evidence about what a render does would be an over-read.
- `bpy.path.resolve_ncase(path)` — "Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path." Same page. Again a helper, and about letter case, not rejected characters.
- `bpy.path.native_pathsep(path)` — "Replace the path separator with the systems native os.sep." Same page. Shows the separator is known to differ by system; says nothing about render output.
- Asset catalogs, a different feature, are the one place a forbidden set is written down: "Invalid characters: `:`, `\`." — manual, https://docs.blender.org/manual/en/4.5/files/asset_libraries/catalogs.html, read 2026-07-30. Not render output; I record it only because it shows what such a statement looks like when the manual makes one, and there is no counterpart for render paths.
- One release-note bugfix line touches characters and systems: "Mac version blender crashes when Asset Libraries path contains BACK SLASH "\"" — https://developer.blender.org/docs/release_notes/4.5/bugfixes/, read 2026-07-30. Documented, but about asset library paths, not render output.

**Label.** documented that the recorded value is an unrestricted string with only length limits; **not in the sources** for what a render does when the characters are unacceptable to the computer.

**Not covered.** Whether Blender checks the path before starting; whether it substitutes, truncates, refuses the frame, or refuses the whole render; whether a per-frame failure stops an animation; whether anything is reported. Also not covered: what happens when the path is longer than the limits quoted.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not addressed by the sources. Worth flagging that the one form of bad-path feedback that *is* documented — the red field and tooltip for template errors — has no counterpart described for background running.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say. The three `bpy.path` helpers above show that the documentation is aware of per-system differences in separators and letter case, and the asset-library crash fix shows a backslash behaving differently on macOS; none of these is stated to apply to render output paths.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 24 — characters this computer will and will not take.**

*What was done, to what.* First the computer itself was asked, with ordinary empty
files, which of nine awkward names it would refuse:

```
touch [co:lon] exit code: 0
touch [aster*isk] exit code: 0
touch [quest?ion] exit code: 0
touch [pi|pe] exit code: 0
touch [back\slash] exit code: 0
touch [кириллица] exit code: 0
touch [em😀ji] exit code: 0
touch [trailing_space ] exit code: 0
```

macOS refused none of them. Then each was given to Blender as the end of the render
output path and one frame rendered, with no window.

*What came back.*

```
    stored as: '…/chars/co:lon_'         frame_path: '…/chars/co:lon_0001.png'
    stored as: '…/chars/aster*isk_'      frame_path: '…/chars/aster*isk_0001.png'
    stored as: '…/chars/quest?ion_'      frame_path: '…/chars/quest?ion_0001.png'
    stored as: '…/chars/pi|pe_'          frame_path: '…/chars/pi|pe_0001.png'
    stored as: '…/chars/back\\slash_'     frame_path: '…/chars/back/slash_0001.png'
    stored as: '…/chars/кириллица_'      frame_path: '…/chars/кириллица_0001.png'
    stored as: '…/chars/em😀ji_'          frame_path: '…/chars/em😀ji_0001.png'
    stored as: '…/chars/trailing_space_ ' frame_path: '…/chars/trailing_space_ 0001.png'
    stored as: '…/chars/dot._'           frame_path: '…/chars/dot._0001.png'
```

and every one of them returned `{'FINISHED'}` and left a file:

```
['aster*isk_0001.png', 'back', 'co:lon_0001.png', 'dot._0001.png', 'em😀ji_0001.png',
 'pi|pe_0001.png', 'quest?ion_0001.png', 'trailing_space_ 0001.png',
 'кириллица_0001.png']
```

Note `back`: the backslash was turned into a folder separator, so a folder called
`back` was made and the file inside it called `slash_0001.png`. Nothing was
sanitised, nothing was refused, and the name on disk is the name that was asked for.

**Entry 25 — names and paths longer than the published limits.**

*What was done, to what.* The path was set to a folder plus a stem of 300 `n`
characters, and then to a stem of 1200. The stored length was read back, then a
frame rendered.

*What came back.*

```
--- 300 chars asked length: 443
    stored length: 443
    frame_path length: 451
```

```
Error: Render error (File name too long) cannot save: '…nnnn0001.png'
raised: RuntimeError Error: Render error (File name too long) cannot save: '…nnnn0001.png'
```

```
--- 1200 chars asked length: 1343
    stored length: 1023
    frame_path length: 1021
```

with the same refusal, exit code 1, and nothing on disk. So the field itself cuts a
path down to 1023 characters without saying anything, and a single name longer than
the computer allows is refused at the moment of writing with the message above.

*Standing lines, brought up to date.* **Driven from outside** — settled: with no
window, a name the computer refuses stops the render, is announced on the terminal
in the words above, and ends the process with 1. **Operating system** — this is
where the answer is thinnest, and it is thin for a reason. macOS refuses almost
nothing: of the awkward characters tried, only a backslash changed anything, and
that was Blender's own doing rather than the computer's. Windows and Linux were not run
here; they sit in the *cannot be run* section of
`Audits/Blender Research/Writing/Hand tests.md` and stay unanswered.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 4.5.12**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 32 — very long text in the File Path field.**

*What was done, to what.* A prepared line of 1400 characters
(`abcdefghij` repeated 140 times) was pasted over the File Path field in Properties ‣
Output, Return pressed, and whatever remained in the field copied back out and saved
to a file. The four saved files are at
`/Users/artist/Documents/Smoozy hand tests/Writing/32/`, named `came back 4.5.txt`,
`came back 5.0.txt`, `came back 5.1.txt` and `came back 5.2.txt`, beside the
`long line.txt` that went in.

*What came back.* Read from the saved files:

```
went in                 : 1400 characters
came back, this version : 1023 characters
```

and the 1023 characters that came back are exactly the first 1023 of the 1400 that
went in — the same in all four versions. So the field itself cuts the text down to
1023 characters as it is entered.

The entry also asked for any message Blender showed on pressing Return. None was
reported, so whether one appeared is not recorded here.

*Standing lines, brought up to date.* **Driven from outside** — nothing here bears on it: every one of these was done at Blender's own window, which is the half the terminal cannot reach. **Operating system** — the Creator works on a Mac, so this settles macOS and leaves Windows and Linux open.


### Observed — hand tests run on a rented Linux machine, 2026-08-06

**Entry 1 — what this Linux machine itself refuses in a name.** Before Blender was
involved at all, ordinary empty files were made on the working file system (reported
by `stat` as ext2/ext3, with `NAME_MAX` 255 and `PATH_MAX` 4096):

```
touch [co:lon] exit code: 0
touch [aster*isk] exit code: 0
touch [quest?ion] exit code: 0
touch [pi|pe] exit code: 0
touch [back\slash] exit code: 0
touch [кириллица] exit code: 0
touch [em😀ji] exit code: 0
touch [tra ilingspace ] exit code: 0
touch [dot.] exit code: 0
touch [semi;colon] exit code: 0
touch [dollar$sign] exit code: 0
touch [new
line] exit code: 0
```

Every one succeeded, including a name holding a line break. Length is the one thing
it refuses:

```
a name of 254 characters: exit code 0
a name of 255 characters: exit code 0
a name of 256 characters: exit code 1   message: touch: cannot touch 'LLL…LLL': File name too long
a name of 300 characters: exit code 1   message: touch: cannot touch 'LLL…LLL': File name too long
```

and a whole path built past the published maximum:

```
makedirs failed at asked length 4255 -> OSError errno 36 File name too long
```

A forward slash cannot be part of a name at all — it is the separator: `touch "a/b"`
made a file called `b` inside a folder called `a`, once that folder existed.
*(Citation check, 2026-08-06: narrowed — the test's own script makes the folder
first, and the line as written read as though the folder were made for it.)*

**So Linux, like macOS, refuses almost nothing but an over-long name.** The two
systems agree on this, and neither of the machines this project has can show what a
render does when the computer refuses an ordinary character.

**Entry 2 — the same characters through a render.** Blender stored every one exactly
as asked and wrote it to disk under that name, in every case:

```
--- asked for: '…/chars/co:lon_'
    stored as: '…/chars/co:lon_'
    frame_path: '…/chars/co:lon_0001.png'
    returned: {'FINISHED'}
```

and the folder afterwards held

```
['aster*isk_0001.png', 'back', 'co:lon_0001.png', 'dot._0001.png', 'em😀ji_0001.png',
 'pi|pe_0001.png', 'quest?ion_0001.png', 'trailing_space_ 0001.png', 'кириллица_0001.png']
```

The one exception is the backslash, which Blender itself turns into a folder
separator — hence the folder called `back` rather than a file with a backslash in its
name. Nothing was cleaned up and nothing was refused.

**Entry 3 — over-long names and paths.** A stem of 300 characters, giving a whole
name past the 255 the system allows:

```
--- 300 chars asked length: 339
    stored length: 339
    frame_path length: 347
```

```
…/long/nnn…nnn0001.png: File name too long
Error: Render error (Invalid argument) cannot save: '…/long/nnn…nnn0001.png'
raised: RuntimeError Error: Render error (Invalid argument) cannot save: '…/long/nnn…nnn0001.png'
```

and a stem of 1200 characters:

```
--- 1200 chars asked length: 1239
    stored length: 1023
    frame_path length: 1021
```

with the same refusal, exit code 1, and nothing on disk (`listing: []`). So the
property itself cuts a path down to 1023 characters without saying anything, and a
name longer than the system will take is refused at the moment of writing.

*(Citation check, 2026-08-06 — a disagreement, recorded and not resolved. Repeating
this entry on the same machine, the 1200-character case in 4.5 came back with a
different bracket from the 300-character one, not "the same refusal":
`Error: Render error (File name too long) cannot save: '…0001.png'` rather than
`(Invalid argument)`. The 300-character case agreed with the record. Both runs stand;
the entry has been sent back to be run again.)*

*The two machines.* All three parts were run on both. **They agreed on what happens**
— nothing is cleaned up, nothing is refused for its characters, the text is cut at
1023, and an over-long name stops the render with exit code 1 and nothing on disk.
**They differed in what the refusal says**: on the Mac 4.5 said `Error: Render error
(File name too long) cannot save:` and the later three said `(Undefined error: 0)`;
on Linux 4.5 says `(Invalid argument)` for the over-long name with `File name too
long` printed by the system on the line above, and the later three say `(Success)`.

*Standing lines, brought up to date.* **Driven from outside** — every run had no
window; a name the computer refuses stops the render, is announced on the terminal in
the words above, and ends the process with 1. **Operating system** — now settled on
two systems. macOS and Linux both refuse almost nothing, so **neither machine this
project has can show what a render does when the computer rejects an ordinary
character**; the only refusal either machine produced is an over-long name. Windows,
where the rules are different and the whole-path limit is shorter, is still untried: no
Windows machine has run it.


## Still open for a running Blender

The list this section began as has been worked through, both halves of it. What was
run, when, on what and with what result is written into each question above as an
observed finding, and the whole list with its exact commands is
`Audits/Blender Research/Writing/Hand tests.md`.

One thing is left that a running Blender could settle, and nobody has run it:

- **Whether an animation rendered to a video carries the scene's sound.** It was
  never on the list.

Three smaller gaps were left by answers that did not cover everything their entry
asked for, and are written in where they belong above rather than filled in:
what the keyframes-only viewport render writes; the labels of the checkboxes inside
the Output panel and the `Python:` line of its header switch; and whether any
message appeared when over-long text was entered in the File Path field.

A second list was run on 2026-08-06, when a second machine arrived: a rented Linux
machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve
cores, reached over SSH. That list is
`Audits/Blender Research/Writing/Hand tests — Linux.md`, and its results are written
into each question above **beside** the Mac's rather than over them. It settled on
Linux what the Mac could only settle on macOS: which characters Linux refuses in a
name, whether a missing output folder is made, what a render does when the folder
cannot be written to, what the folder separator does, and whether a shorter limit on
length bites.

One thing on that list was not run, because it needs the machine to itself and the
machine is shared with other work:

- **What a render does on Linux when the volume runs out of room.** It is written out
  in full, with its exact commands, under *Needs the machine to itself* in
  `Audits/Blender Research/Writing/Hand tests — Linux.md`.

And these were not run:

- Which characters an output path may hold on Windows, and what a render does when it
  meets one it may not.
- Whether a missing output folder is made on Windows.
- What a render does on Windows when the folder cannot be written to, or the disk is
  full.
- Whether what is recorded differs on Windows, where the path separator differs.
- Whether an over-long name behaves differently on Windows, where the whole-path
  limit is shorter.
- Anything needing two graphics cards, a card that is neither Apple's nor NVIDIA's, a
  second machine of the same kind, or hardware below Blender's stated floor.

The equipment is one MacBook Pro with an Apple M1 Pro chip running macOS 26.5.2, and the
rented Linux machine described above. No Windows machine was used.

---

2026-08-04 — Opus 5 (Claude Code) — Writing, Blender 4.5
