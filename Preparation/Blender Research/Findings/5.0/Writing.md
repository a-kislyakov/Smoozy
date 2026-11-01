# Writing — Blender 5.0

Sources consulted, all on **2026-07-30** (the snapshot date given to me): Blender 5.0 manual (`Blender Sources/manual_5.0`, including its `_images` folder), Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own files (`Blender Sources/cycles_5.0`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

No helper agents were used. Everything below I read myself.

---

## 1. Can where a render writes be settled at the moment the render is started, without changing the file?

**Answer.** Yes, at launch, by a command-line argument. Blender takes `-o` / `--render-output`, which sets the render path for that run.

The manual's list of arguments says:

> `-o, --render-output <path>`
>
> Set the render path and file name.
> Use `//` at the start of the path to render relative to the blend-file.

There is a strict ordering rule attached to it. The manual states:

> `blender --background --render-output /tmp test.blend --render-frame 1`
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set.

and

> `blender --background test.blend --render-output /tmp --render-frame 1`
>
> …works as expected.

The same page also states the general rule: "Arguments are executed in the order they are given." The dedicated command-line rendering page repeats it: "Always position `-f` or `-a` as the last arguments."

The sources also document ingredients for a Python route at launch. The manual lists `-P, --python <filepath>` ("Run the given Python script file") and `--python-expr <expression>` ("Run the given expression as a Python script"). The API exposes the path as `RenderSettings.filepath`, without marking that property read-only, reached through `Scene.render`.

**LABEL: documented** — for the fact that `-o` sets the render path at launch, and for the ordering rule.

**LABEL: inferred** — for the "without changing the file" half. The sources nowhere say that `-o` writes anything back into the blend-file. The step I took: saving a blend-file is a separate, explicit action in the sources (`bpy.ops.wm.save_mainfile`, "Save the current Blender file"), and nothing on the command-line or rendering pages mentions saving; so setting `-o` alters only the loaded, in-memory settings. That is a reader-supplied step, so it is inference, not documentation.

**LABEL: inferred** — that Python provides a second route for setting the path at launch. The step is to combine a path property that is not marked read-only with the arguments that run Python at launch. The sources do not state that combined conclusion.

**Not covered.** The sources do not state what happens to the blend-file on disk during a render, so "without changing the file" is not settled outright. They also do not say whether `-o` affects anything other than the scene render path (see Q9 for the File Output node). Nothing says what `-o` does when the blend-file also has a value — beyond the ordering note above.

**Quoted from:** the manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html and https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html ; the API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html and https://docs.blender.org/api/5.0/bpy.ops.wm.html and https://docs.blender.org/api/5.0/bpy.types.Scene.html
**Date read:** 2026-07-30

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* This is the outside-the-interface case: `-o` is a command-line argument, and the manual's own examples all use `-b` (background). The manual notes for `-b`: "Run in background (often used for UI-less rendering)." The sources do not describe an equivalent of `-o` from inside the window other than typing into the Output panel field.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say the arguments differ by operating system. Two related notes exist: the manual warns "Arguments are case sensitive! `-F` and `-f` are not the same", and separate launch pages exist for Linux, macOS and Windows. Whether `-o` behaves differently per system is not stated.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

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
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup …/test.blend -o …/elsewhere2/e_ -F PNG -x 1 -f 1
```

*What came back.*

```
--- before ---
a6c73ba5da52d697efea68d920079903b4e6520e1de03dcfc928bf4dc63384d4  …/test.blend
a6c73ba5da52d697efea68d920079903b4e6520e1de03dcfc928bf4dc63384d4  …/keep.blend
…/test.blend 92541 1785882843
Saved: '…/elsewhere2/e_0001.png'
render exit code: 0
--- after ---
a6c73ba5da52d697efea68d920079903b4e6520e1de03dcfc928bf4dc63384d4  …/test.blend
a6c73ba5da52d697efea68d920079903b4e6520e1de03dcfc928bf4dc63384d4  …/keep.blend
…/test.blend 92541 1785882843
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
Saved: '…/node_out/nd_Image0001.png'
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

Everything in the Linux sections of this document was run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. It is a rented container sharing somebody else's hardware, so it is given fewer cores than the chip really has and its system settings are not entirely its own; other work may have been running on it at the same time, so nothing here says how long anything took. Blender was started with no window unless an entry says otherwise, with its settings folder pointed at one of this topic's own and with the display variable unset so no window could be reached. The build used for this version was **Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)**. Date: 2026-08-06. In the output below, `…` stands for the working folder on that machine, which was `/root/smoozy/Writing/work/<entry>/5.0`. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests — Linux.md`.

**One note on how the output is shown.** From 5.0 every line Blender prints while rendering carries a timestamp and a channel name in front of it, like `00:00.617  render           | Saved: '…'`. Where a quoted line below carries no such prefix, the prefix has been dropped because the timestamp differs from one run to the next; nothing after the bar is changed.

**These results settle Linux and say nothing about macOS.** Where the same test was also run on the Creator's Mac, both results are given and neither is folded into the other.

**Entry 17 — does the output argument change the blend-file on disk?**

*What was done, to what.* A blend-file was saved with its output path recorded as
`…/stored/stored_`, a byte-for-byte copy kept beside it, and the file then rendered
with the output argument pointing at `…/elsewhere/e_`.

*The command.*

```
/opt/blender/blender-5.0.1-linux-x64/blender -b --factory-startup …/test.blend -o …/elsewhere/e_ -F PNG -x 1 -f 1
```

*What came back.* The two checksums and the size and modified time were identical
before and after:

```
--- checksums before
3d9ef2e8dee3d7c9b5366ca681a712a529657b085373ab4ceeb36995e5b42db7  …/test.blend
3d9ef2e8dee3d7c9b5366ca681a712a529657b085373ab4ceeb36995e5b42db7  …/keep.blend
…/test.blend 92433 1785990951
--- the render, with the output argument pointing somewhere else
Saved: '…/elsewhere/e_0001.png'
exit code: 0
--- checksums after
3d9ef2e8dee3d7c9b5366ca681a712a529657b085373ab4ceeb36995e5b42db7  …/test.blend
3d9ef2e8dee3d7c9b5366ca681a712a529657b085373ab4ceeb36995e5b42db7  …/keep.blend
…/test.blend 92433 1785990951
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

**Answer.** There are two different recorded things in Blender 5.0, and they are shaped differently.

**(a) The scene's render output.** One single text string, stored on the scene's render settings. The API says:

> `filepath`
>
> Directory/name to save animations, `#` characters define the position and padding of frame numbers
>
> Type:
>
> string, default "", (never None, blend relative `//` prefix supported, Supports template expressions)

So the one string carries, inside itself: an optional `//` prefix, a directory part, a name part, `#` runs, and template expressions. The file extension is **not** part of it — it is a separate switch plus a read-only derived value:

> `use_file_extension`
>
> Add the file format extensions to the rendered file name (eg: filename + .jpg)
>
> Type: boolean, default True

> `file_extension`
>
> The file extension used for saving renders
>
> Type: string, default "", (readonly, never None)

The manual's Output panel page describes the same field as "File Path — Choose the location to save rendered frames."

I opened the manual's picture of that panel, `_images/render_output_properties_output_panel.png` (shown on the Output page). Visible in it: a single wide text field containing `/tmp/` with a folder-browse button at its right, then checkboxes Overwrite, Placeholders, File Extensions, Cache Result, then File Format (PNG), Color (BW/RGB/RGBA), Color Depth (8/16), Compression (15%). So the recorded place really is one field, not several.

**LABEL: documented** for the API text; **LABEL: documented** for what the picture shows (it is part of the manual).

**(b) The File Output node's output.** Two separate strings plus per-socket names. The 5.0 release notes state the change plainly:

> - The **File Output** node was redesigned for a better user experience. (blender/blender@68dc278fe5)
>   - It now supports dragging links to create a new input.
>   - The final paths of the images are displayed in the UI to remove guess work.
>   - The base path is now split into a Directory and a File Name.

and, in the Python API notes:

> - File Output node now takes `directory` and `file_name` as two different inputs
>   - Removed:
>     - `bpy.types.CompositorNodeOutputFile.file_slots`
>     - `bpy.types.CompositorNodeOutputFile.layer_slots`
>     - `bpy.types.CompositorNodeOutputFile.base_path`
>   - Added:
>     - `bpy.types.CompositorNodeOutputFile.directory`
>     - `bpy.types.CompositorNodeOutputFile.file_name`
>     - `bpy.types.CompositorNodeOutputFile.file_output_items`

The API describes the two parts:

> `directory`
>
> The directory where the image will be written
>
> Type: string, default "", (never None, blend relative `//` prefix supported, Supports template expressions)

> `file_name`
>
> The base name of the file. Other information might be included in the final file name depending on the node options
>
> Type: string, default "", (never None, Supports template expressions)

Note the asymmetry, which is in the source itself: `directory` says "blend relative `//` prefix supported", `file_name` does not.

The third part is the socket name. The manual says of the Images/Layers panel: "Lists all input sockets of the node. The name of each input socket is used to construct the final file name."

I opened the manual's picture of the node, `_images/node-types_CompositorNodeOutputFile.webp`. Visible in it: a node headed "File Output", a first text field containing `/tmp/` with a folder-browse button, a second text field containing `file_name`, and one unconnected input socket. That matches the two-part description.

**LABEL: documented.**

**Not covered.** Nothing here says how these strings are laid out inside the `.blend` file on disk (byte format, block names, etc.) — only what the properties are. Nothing says whether the scene render path is one per scene or shared; it is reached through `Scene.render`, which implies per scene, but the sources do not say it in words. The 767 / 255 / 1023 character limits in Q10 apply, but the sources do not connect them to these particular fields by name.

**Quoted from:** the API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html and https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html ; the manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html and https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html (and the two images named above); the release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/ and https://developer.blender.org/docs/release_notes/5.0/python_api/
**Date read:** 2026-07-30

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The Python API exposes the same properties, and `bpy.app.background` is documented as "True when blender is running without a user interface (started with -b)". **LABEL: inferred** that those ingredients make the properties readable and writable without the window. The sources do not say the stored shape differs.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say the recorded shape differs by operating system.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 1 — what a fresh Blender records, and which switches sit beside it.**

*What was done, to what.* Blender was started with no window and factory settings,
and a short script printed the render output path, the built-in default behind it,
each of the output switches, and the preference that seeds a new scene.

*The command.*

```
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup --python q1.py
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
media_type IMAGE
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
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup --python q2.py
```

*What came back.*

```
scene has node_tree: False
scene has compositing_node_group: True
node type CompositorNodeOutputFile
  PROP active_item_index INT writable 0
  PROP directory STRING writable ''
  PROP file_name STRING writable ''
  PROP file_output_items COLLECTION readonly ''
  PROP format POINTER readonly ''
  PROP save_as_render BOOLEAN writable False
inputs ['']
```

(Only the properties bearing on where the node writes are quoted.) There is no
`base_path`, no `file_slots` and no `layer_slots`. A new node starts with no items
at all and one unnamed input.

**Entry 25 — how much text the path field actually holds.** Asking for a path
1343 characters long and reading it back gave a stored length of exactly 1023.
A path 443 characters long was stored whole. So the published 1023 limit is the
one enforced on the stored text.

*Standing lines, brought up to date.* **Driven from outside** — everything above
was read and set with no window open, so the recorded shape is the same thing a
driven Blender sees. **Operating system** — settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.0.1**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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

*What is on the node's face.* Two stacked text fields with no labels, the upper
holding `//out/` and the lower holding `shot`, then one row per input:

```
File Output
  [ //out/ ]
  [ shot   ]
  ○ passes_diffuse
  ○ beauty
  ○                        (empty)
```

The input named `passes/diffuse` when it was typed appears here as
`passes_diffuse` — the slash has become an underscore.

*What is in the sidebar.* A **Node** panel holding Name (`File Output`), Label
(empty) and Color. Then a **Properties** panel holding, in this order:

```
[ //out/ ]
[ shot   ]
Image | Multi-Layer EXR                    (Multi-Layer EXR chosen)
Node Format
            [x] Save as Render
Color Depth         Float (Half) | Float (Full)   (Full chosen)
Codec               None
            [ ] Interleave
Color Management    Follow Scene | Override       (Follow Scene chosen)
Layers
  ● passes_diffuse
  ● beauty                                  (selected)
Socket Type         Color
Output Paths
  /Users/artist/Documents/Smoozy hand tests/Reading a file/13/out/shot.exr
Custom Properties
  [ + New ]
```

So the node records two path fields plus a name per input, and it carries a panel
called **Output Paths** that shows the finished name before anything is rendered.

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
VERSION 5.0.1
startup filepath '/tmp/'
rna default     ''
use_file_extension True True
file_extension True '.png'
use_overwrite True True
use_placeholder True False
use_render_cache True False
save_output False 'ABSENT'
file_format PNG
media_type IMAGE
render_output_directory '//'
```

**Entry 16 — what a File Output node records.** The node's own settings came back as
`active_item_index`, `directory`, `file_name`, `file_output_items`, `format`, with its inputs reading `['']`.

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

**Answer.** The sources describe the following marks. `%` has the narrow limit stated below.

**`#` — frame number and its padding.** The manual's Output page:

> When rendering an animation,
> the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png).
> You can set a custom padding size by adding the appropriate number of `#` anywhere in the file name
> (e.g. `image_##_test.png` translates to `image_01_test.png`).

The command-line page repeats it with more examples:

> The `#` characters are replaced by the frame number, and used to define zero padding.
>
> `animation_##_test.png` becomes `animation_01_test.png`
>
> `test-######.png` becomes `test-000001.png`

The API says the same on the property itself: "`#` characters define the position and padding of frame numbers".

**`//` — the blend-file's own directory, when it starts the path.** The Output page:

> This setting expands Relative Paths
> where a `//` prefix represents the directory of the current blend-file.

The command-line page: "Use `//` at the start of the path to render relative to the blend-file." The manual's relative-paths section: "when entering the path into a text field, use a double slash prefix (`//`) to make it so", and "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."

**`{` … `}` — template expressions.** The manual's File Paths page:

> Path templates substitute template expressions (written `{abcd}`, where
> "abcd" is a variable name) in a filepath when the path is used.

and

> This substitution happens internally at time of use (e.g. while rendering), so
> the template syntax will remain as-is in the path field.

The scope is limited, and the manual says so:

> Currently path templates are only supported for the render output path in
> Scene properties and the output paths in the compositor's File Output
> node. More filepaths will support templates in future releases.

The variables, quoted from the same page — available in all paths that support templates: `blend_name` ("The currently open blend file's name (without the .blend)"), `blend_dir` ("Path up to (but not including) the currently open blend file"), `blend_name_lib`, `blend_dir_lib` (the library blend-file's name/path for library-linked data-blocks). Available only in render output paths, including the File Output node: `fps`, `resolution_x` / `resolution_y` ("This factors in the resolution scale as well"), `scene_name` ("Name of the current scene"), `camera_name` ("Name of the current render camera"). Available only in path properties on a node: `node_name` ("The name of the node that the path property is on").

The 5.0 release notes record which of these are new:

> ## More Path Template Variables
>
> The following new path template variables have been introduced
> …
> - `scene_name`
> - `camera_name`
> - `node_name`
> - `blend_name_lib`
> - `blend_dir`
> - `blend_dir_lib`

**`:` inside a template expression, followed by `#` marks — a format specifier.** The File Paths page:

> Format specifiers … are written after a separating colon, like this:
>
> `dance_{fps:FORMAT}.png`

> `dance_{fps:###}.png`: format as an integer with at least 3 digits.
>
> `dance_{fps:.###}.png`: format as a floating point number with exactly 3
> digits after the decimal point.
>
> `dance_{fps:###.##}.png`: format as a floating point number with at least 3
> digits for the integer part and exactly 2 digits for the fractional part.

with worked results: "if the fps is 29.97, then: `dance_{fps:###}.png -> dance_030.png`", "`dance_{fps:.###}.png -> dance_29.970.png`", "`dance_{fps:###.##}.png -> -> dance_029.97.png`", and "Note that the values are properly rounded for the given number of digits." Also: "Format specifiers currently can only be used with variables that represent numerical values, not string values."

**`{{` and `}}` — escapes, standing for a single literal brace.** Same page:

> `{{` translates to a single `{` in the final path.
>
> `}}` translates to a single `}` in the final path.

with examples: "`my_weird}}_{{path.png -> my_weird}_{path.png`", "`//my_render_{{dir}}/{blend_name}.png -> //my_render_{dir}/dance.png`", "`//my_render_dir/{{{blend_name}}}.png -> //my_render_dir/{dance}.png`".

**`%` — the view name, in multi-view / stereo rendering.** This appears once, in the API, on the method that expands a render path:

> `view` (string, (optional, never None)) – View, The name of the view to use to replace the "%" chars

**LABEL: documented** for `#`, `//`, `{...}`, `:` specifiers, `{{`/`}}`.
**LABEL: documented, but thinly** for `%`. It is stated outright, but only in that one parameter description; the manual never mentions `%` in an output path, and nothing says what happens to `%` in a normal render as opposed to a `frame_path()` call.

What a bad template does is also documented:

> Paths that support templates can have template errors, which prevent the
> path from being processed.

> When there are template errors in a path, the path field will be highlighted red
> in the UI

> Depending on the path, template errors may prevent certain actions. For
> example, if there are errors in the render output path, then rendering an
> animation will fail with an error message indicating the path errors.

I opened the manual's picture `_images/files_file_paths-invalid_path_template_tooltip.png`. Visible in it: an Output panel whose path field, shown in red, contains `//my_render_dir/{blend_name.png`, and a tooltip reading "Output Path / Directory/name to save animations, # characters define the position and padding of frame numbers. / Value: //my_render_dir/{blend_name.png / Syntax error(s): / - Invalid or incomplete template expression '{blend_name.png'." **LABEL: documented** (a manual image).

**Not covered.** Whether `#` marks are honoured in the *directory* part of the path, not just the file name, is not stated — the manual says "anywhere in the file name". Whether `%` is expanded during an ordinary render (rather than only by `frame_path()`) is not stated. Whether the File Output node's `file_name` honours `//` is not stated (its `directory` explicitly does; `file_name` is not marked so). Whether `~`, environment variables, or wildcards mean anything in these paths is not stated anywhere I searched.

**Quoted from:** the manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html , https://docs.blender.org/manual/en/5.0/render/output/properties/output.html , https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html , https://docs.blender.org/manual/en/5.0/files/blend/open_save.html , and the image `_images/files_file_paths-invalid_path_template_tooltip.png` on the File Paths page; the API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html ; the release notes, https://developer.blender.org/docs/release_notes/5.0/core/
**Date read:** 2026-07-30

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The `#` and `//` marks and the templates are described on the command-line page as well as the UI page, so they hold when Blender is driven from outside. The manual's `-o` entry says so directly: "You can use path templating features such as `{blend_name}` in the path." One part is interface-only: the red highlight and the tooltip are described as UI behaviour ("the path field will be highlighted red in the UI"). What a template error looks like without a window is not stated, beyond "rendering an animation will fail with an error message indicating the path errors".

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say any of these marks differ by operating system. `//` as a prefix is Blender's own convention, not the system's; the API notes a related helper, `bpy.path.basename`, "Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility." Nothing further is said.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

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
describe. `{blend_name}`, `{fps}` and `{scene_name}` were all replaced. A `#` sitting in the folder part is **not**
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
ext=False '/tmp/{scene_name}_'         -> '/tmp/Scene_0007'
ext=False '/tmp/{fps}_'                -> '/tmp/24.0_0007'
ext=False '/tmp/out_%_'                -> '/tmp/out_%_0007'
ext=False '/tmp/out_%_##'              -> '/tmp/out_%_07'
ext=False '/tmp/dir_##/out_'           -> '/tmp/dir_##/out_0007'
ext=False '/tmp/out_.png'              -> '/tmp/out_.png0007'
ext=False '/tmp/'                      -> '/tmp/0007'
ext=False ''                           -> '0007'
ext=False '/tmp/my_weird}}_{{path_'    -> '/tmp/my_weird}_{path_0007'
```

The switch-on lines of this run matched entry 9's, line for line. Entry 9's table
leaves out `/tmp/{scene_name}_`, which its own paragraph reports on instead; with the
switch on this run gave `'/tmp/{scene_name}_' -> '/tmp/Scene_0007.png'`. The three
calls made with a view name, which the script makes with the switch on, came back the
same as entry 9's too:

```
view=''     -> '/tmp/out_%_0007.png'
view='left' -> '/tmp/out_%_0007_L.png'
view='right' -> '/tmp/out_%_0007_R.png'
```

Two things fall out. The switch changes one thing and nothing else: with it off, no
`.png` is put on the end. Every mark gave the same result either way — `//`, `#`,
`{}`, `{{`/`}}` and `%` are untouched by it. And only one of the fifteen came back
identical under both switch positions: `/tmp/out_##_x.png`, whose recorded name
already ends in `.png`, to which nothing further was added with the switch on. Its
near miss is `/tmp/out_.png`, where the `.png` sits in the middle of the stem — with
the switch on that came back as `/tmp/out_.png0007.png`, and with it off as
`/tmp/out_.png0007`.

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
asked for names: ['passes/diffuse', 'masks/object_01']
item names now : ['passes_diffuse', 'masks_object_01']
Saved: '…/nodebase/base_passes_diffuse0001.png'
Saved: '…/nodebase/base_masks_object_010001.png'
```

The slashes were replaced by underscores as the names were set, and no folders were
made.

**Entry 14 — a template whose value carries a `#`.** The scene was renamed `sc##ne` and the path set to `{scene_name}_`:

```
scene name is now: 'sc##ne'
frame_path: '…/tmpl/sc07ne_.png'
Saved: '…/tmpl/sc01ne_.png'
```

The same thing through `{blend_name}`, with the file saved as `da##ce.blend`:

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

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.0.1**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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
ext=True  '/tmp/{scene_name}_'         -> '/tmp/Scene_0007.png'
```

**Entry 2 — a backslash.** Setting the output path to a name ending `back\slash_`
made a **folder** called `back` on this Linux machine and wrote `slash_0001.png`
inside it, exactly as on the Mac. The Linux machine itself has no trouble with a
backslash in a name — an ordinary file called `back\slash` was made without
complaint — so turning it into a folder separator is Blender's own doing and not the
computer's.

**Entry 6 — a slash inside a File Output node's input name.** In this version the slash was replaced by an underscore the moment the name was set, and **no
folder was made**:

```
item names: ['passes_diffuse', 'masks_object_01']
Saved: '…/nodebase/passes_diffuse0001.png'
```

**Entry 14 — a template whose value carries a `#`.** the scene was renamed `sc##ne` and the path set to `{scene_name}_`:

```
frame_path: '…/tmpl/sc07ne_.png'
Saved: '…/tmpl/sc01ne_.png'
```

So the template goes in first and the hash marks it brings with it are then read as
the place for the frame number.

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

**LABEL: not in the sources.**

I could not find any statement, anywhere in the 5.0 manual, the 5.0 API documentation, the 5.0 release notes or the Cycles files, about whether rendering creates missing directories on the way to the output path.

**Where I looked, and with what words.** I searched the whole of `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0` for: `mkdir`, `make director*`, `create director*`, `director* creat*`, `will be created`, `created automatically`, `automatically created`, `create the directory`, `creates the directory`, `nonexistent`, `does not exist`, `output directory`, `output folder`. I read in full the Output properties page, the Rendering Animations page, the render output introduction, the File Output node page, the File Paths page, the command-line arguments page, the command-line rendering page and the Preferences File Paths page. None of them says anything about creating folders during a render.

The near-misses I did find are all about *other* features, and I am naming them so it is clear they are not the answer:

- The Ocean modifier: "If the cache path folder does not exist, it will be created." (manual, modifiers) — a cache path, not a render output path.
- The File Browser: "If you type a nonexistent path, you will be prompted to create it." (manual, File Browser) — a person typing in a browser, not a render writing.
- The Temporary Files preference, which does the opposite: "The path must reference an existing directory or it will be ignored and the systems temporary directory will be used instead."
- `bpy.utils` has parameters described as "create (bool) – Treat the path as a directory and create it if its not existing", but these are on resource-path helpers, not on rendering.

None of these speaks for the render output path, so I will not stretch them into an answer.

**Not covered.** Everything: the still-image case, the animation case, the File Output node case, and whether one missing level behaves differently from several. Also not covered: whether the answer differs between the scene output path and the File Output node's Directory.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not in the sources either way.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not in the sources.

*(Listed at the end as something a person could settle by hand in minutes.)*

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 3 — the scene's own output path.**

*What was done, to what.* At factory settings, with no window, the render output
path was set to `…/one_missing/img_` (one folder that did not exist) and one frame
rendered; then to `…/two_a/two_b/img_` (two folders that did not exist) and one
frame rendered.

*The command.*

```
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup --python q3.py -- …
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
Saved: '…/node_a/node_b/nd_0001.exr'
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
Saved: '…/node_a/node_b/nd_0001.exr'
exit code: 0
```

**Entry 6 — folders implied by an input's name.** In this version the slash in an input's name was replaced by an underscore and **no folder was made** for it — see question 3.

*The two machines.* All three were run on both. **They agreed**: a missing output
folder, and a whole chain of missing folders, is made on the Creator's Mac and on the
rented Linux machine alike.

*Standing lines, brought up to date.* **Driven from outside** — every run had no
window at all, so folder making is not something the interface does on Blender's
behalf, on either machine. **Operating system** — now settled on two systems, macOS
and Linux, which agreed. Windows is still untried: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**


## 5. What does a write do when its folder is not there, when there is no room left, or when it cannot be written to?

**LABEL: not in the sources** — for all three conditions.

**Where I looked, and with what words.** I searched all four source sets for: `disk full`, `full disk`, `no space`, `no space left`, `out of disk`, `ENOSPC`, `quota`, `permission`, `denied`, `read-only`, `read only`, `not writable`, `writable`, `write permission`, `read-only file system`, `cannot save`, `unable to write`, `failed to write`, `fails`, `failure`, `abort`, `error message`, `does not exist`, `nonexistent`. I read the Output properties page, Rendering Animations, the render output introduction, the File Output node page, the File Paths page, both command-line pages and the Troubleshooting section. Nothing describes what a render does when the destination folder is missing, when the drive is full, or when the location cannot be written to.

The only write-failure behaviour the sources do document is a *different* failure — a bad path template, not a bad destination:

> Depending on the path, template errors may prevent certain actions. For
> example, if there are errors in the render output path, then rendering an
> animation will fail with an error message indicating the path errors.

Two adjacent, documented facts that are about size but **not** about running out of room, and which I am flagging so they are not mistaken for the answer:

> If your video is huge and exceeds 2GiB, enable Autosplit Output.
> This will automatically split the output into multiple files after the first file is 2GiB in size.

and, from Working Limits, the string-length ceilings: "directory: 767", "file-name: 255", "file-path: 1023". The sources do not say what happens when those are exceeded either.

Also documented, and relevant to recovering from an interrupted write but not to the failure itself:

> Just disable the Overwrite option to start rendering where you left off.

**Not covered.** Whether the render stops or continues; whether other frames still get written; what the exit code is; whether a partial file is left behind; whether anything is reported to the console; whether a File Output node failing stops the main render. None of it is stated.

**Quoted from:** the manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html , https://docs.blender.org/manual/en/5.0/render/output/properties/output.html , https://docs.blender.org/manual/en/5.0/render/output/animation.html , https://docs.blender.org/manual/en/5.0/advanced/limits.html
**Date read:** 2026-07-30

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not in the sources. In particular, nothing says what a background render's exit code or console output is on a write failure.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not in the sources.

*(Listed at the end as something a person could settle by hand.)*

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

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
00:00.533  image.write      | ERROR OpenImageIO write failed: Could not open file "…/ro/img_0001.png"
Error: Render error (Undefined error: 0) cannot save: '…/ro/img_0001.png'
raised: RuntimeError Error: Render error (Undefined error: 0) cannot save: '…/ro/img_0001.png'
python-route exit code: 1
```

and from the command line alone, with `-o` pointed into the same folder:

```
00:01.191  image.write      | ERROR OpenImageIO write failed: Could not open file "…/ro/cli_0001.png"
00:01.191  reports          | ERROR Render error (Undefined error: 0) cannot save: '…/ro/cli_0001.png'
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
00:00.520  image.openexr    | ERROR IMB_exr_write_channels: Failed to write pixel data to image file "/Volumes/SMOOZYTINY/img_0001.exr". No space left on device.
00:00.520  render           | Saved: '/Volumes/SMOOZYTINY/img_0001.exr'
00:00.560  image.openexr    | ERROR IMB_exr_begin_write: No space left on device.
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

**Entry 23 — switching automatic saving off.** There is no such switch in 5.0:

```
has save_output: False
could not set save_output: AttributeError 'RenderSettings' object has no attribute 'save_output'
animation returned: {'FINISHED'}
listing: ['img_0001.png']
```

**Entry 25 — a name the computer will not take.** A file name of 300 characters was
stored whole and then refused by the computer: `ERROR OpenImageIO write failed: Could not open file "…"` followed by `Error: Render error (Undefined error: 0) cannot save: '…'`,
exit code 1, and nothing on disk.

*Standing lines, brought up to date.* **Driven from outside** — settled. Every
failure above is announced on the terminal in the words quoted, and a Blender
started with `-b` and told to render from the command line ends with an exit code
of 1. The one qualification is that a failure raised inside a Python script that
catches it — as entry 23 does — leaves the process ending with 0.
**Operating system** — settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.0.1**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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
00:00.905  image.write      | ERROR OpenImageIO write failed: Could not open file "…/ro/img_0001.png"
Error: Render error (Success) cannot save: '…/ro/img_0001.png'
raised: RuntimeError Error: Render error (Success) cannot save: '…/ro/img_0001.png'
exit code: 1
```

and straight from the command line, with no Python at all:

```
00:01.070  image.write      | ERROR OpenImageIO write failed: Could not open file "…/ro/cli_0001.png"
00:01.070  reports          | ERROR Render error (Success) cannot save: '…/ro/cli_0001.png'
exit code: 1
```

**This is where the two machines differ, and it is in the words only.** On the
Creator's Mac the bracket read `(Undefined error: 0)`; on Linux the same empty reason
is printed as `(Success)`. Neither says why the write failed.

In both runs with the capability dropped, the folder was left **completely empty** —
not even a part-written file — and the render stopped at the very first frame, with
frames two and three never attempted. In the plain root run, where the machine let the
write through, all three frames were written and the process ended with 0.

**No room left on the volume — run by the runner on 2026-08-06, once the rented machine was idle.** This had been left as *needs the machine to itself* while other work was on the box. It is now run, and it is the sharpest result this question has.

*What was done, to what.* On the rented Linux machine, with nothing else running on it, the disk was filled with a ballast file until two megabytes remained. A one-line write was tried first to show the disk really was full. Then Blender was asked to render one frame of a default scene at 1920 by 1080 on the graphics card and save it as a PNG into that full disk. Each version was pointed at its own settings folder, and the ballast was removed afterwards.

*What came back.* Blender printed its ordinary saving line and exited **0**:

```
Saved: '/root/smoozy/Machine notes/full.png'
blender exit: 0
```

*But the file it saved is not whole.* Read back straight afterwards it does not end as a PNG must. **This was checked twice, and the first check was faulty** — it compared a file's last eight bytes against a twelve-byte marker, which can never match, so it would have called any file incomplete. That fault was found later and the whole test was run again in all four versions with a correct check. This is the second, sound run:

```
exit 0, said saved once, no error line about the disk -> 864256 bytes, ends properly: False
```

Every line this version printed about errors, warnings or saving was then read directly, setting aside an unrelated warning about AMD graphics, and there was nothing but the saving line.

So on a full disk this version **writes a part of a picture, announces it as saved, and exits as though nothing were wrong**. There is no error, no warning, and nothing in the exit status to tell the two apart. The same came back in all four versions on this machine, with only the byte count differing.

For contrast, on the same full disk an ordinary one-line write from the shell fails plainly and says so: `No space left on device`. The failure is visible to everything except Blender's own report of itself.

*On what kind of computer.* The rented Linux machine, run over SSH with no window and nothing else running. **This settles Linux only.** It was not run on the Creator's Mac, so macOS is still untried for this question, and Windows is untried everywhere. The test is written out in full under *Needs the machine to itself* in `Audits/Blender Research/Writing/Hand tests — Linux.md`.

**Observed — the disk filling in the *middle* of an animation, 2026-08-06.** The disk-full answer above was got with the disk already full before the render began. That is not how it happens in use: a long animation slowly eats the disk and hits the wall part way. This is that case, and it behaves differently.

*What was done, to what.* On a rented Linux machine with four NVIDIA GeForce RTX 4060 Ti cards, driver 580.95.05, sixty-one cores, 197 GB of memory and no display server, over SSH, with nothing else running on it. A 400-frame animation at 4000 by 4000, PNG with compression off so each frame is about 64 MB, rendered on the cards. The disk was filled with a ballast file first, leaving about 1.5 GB — enough for roughly twenty-four frames.

*What came back.*

```
blender exit: 1
lines mentioning an error, a failure or the disk: 0
frames it announced as saved: 25
last line before it stopped: 02:38.094  render  | Fra: 26 | Mem
```

*And what was on disk.*

```
files: 25   whole: 24   part-written: 1   empty: 0
first file that is not whole: f_0025.png
f_0024.png   64105848 bytes  whole: True
f_0025.png   33988608 bytes  whole: False
```

*So three things are true at once.* Blender **exited 1**, which the disk-being-full-at-the-start case did not — there it exited 0. It said **nothing at all** about why: not one line mentioning the disk, an error or a failure. And **the last frame it announced as saved is the broken one** — it counted 25 saved, and the twenty-fifth is half a picture.

Anything trusting the last `Saved:` line takes a corrupt frame for a finished one. Anything counting files gets 25 where 24 are usable. The exit status is the only honest signal, and it says only that something went wrong.

*On what kind of computer.* The four-card rented Linux machine described above. **This was run in Blender 5.2 alone and settles that version on Linux only.** The other three versions were not run for this, and Windows is untried everywhere.


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
Linux. Windows is untried. *(The last sentence but one was overtaken on
2026-08-07: running out of room is now settled on Linux too. The section below holds
that run.)*


### Observed — the full-disk test on a rented Linux machine, 2026-08-07

*(Added 2026-08-07, in the round that ran this one test. It came from this round and no
auditor has seen it.)*

This is entry 28 of `Audits/Blender Research/Writing/Hand tests — Linux.md`, which had
been written out in full and left unrun because the machine of the day was shared. It
is the same procedure as entry 7 of the Mac list, so the two machines can be set beside
each other.

*On what kind of computer.* A rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA
GeForce RTX 4080 SUPER with 16 GB on driver 580.173.02, an AMD EPYC 7B12 with
twenty-four cores, 39 GB of memory, 118 GB of free disk, and **no display server at
all** — reached over SSH and driven with no window. It is not the machine the rest of
that Linux list was run on; that one had an RTX 4090 on driver 580.105.08 and has since
been destroyed. Each version was given a settings folder of its own, and the display
variables were unset. The build was **Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)**. Date: 2026-08-07.

*What was done, to what.* The machine's real disk was never filled. A disposable file
system of two megabytes was made inside a file, formatted and mounted, giving 1,804 KB
free — close to the space the Mac's two-megabyte disk image gave. A three-frame
animation of 2000 by 2000 pixel uncompressed 32-bit OpenEXR files was then rendered
onto it with no window, by the same script the Mac used with one path changed. A whole
frame of that render is 64,032,536 bytes in 4.5 and about the same in the others, so a
single frame cannot fit and the disk fills part way through the first one.

*The control that makes this a test.* The very same script was run first into a folder
with room to spare, in every version. It wrote all three frames and ended with 0:

```
00:02.375  render           | Saved: '/root/writing-full-disk/roomy/5.0/img_0001.exr'
00:02.499  render           | Saved: '/root/writing-full-disk/roomy/5.0/img_0002.exr'
00:02.623  render           | Saved: '/root/writing-full-disk/roomy/5.0/img_0003.exr'
operator returned: {'FINISHED'}
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)

Blender quit
exit code: 0
-rw-r--r-- 1 root root 64032656 Aug  7 01:24 img_0001.exr
-rw-r--r-- 1 root root 64032656 Aug  7 01:24 img_0002.exr
-rw-r--r-- 1 root root 64032656 Aug  7 01:24 img_0003.exr
```

So a run that writes nothing because the set-up was wrong is tellable from a run that
writes a broken file. **A negative result** would have been the full-disk run doing what
the control did — three whole frames, exit code 0 — or writing nothing at all and
saying so cleanly, or carrying on past frame 1. None of those happened.

*What came back on the full disk.*

```
00:01.959  image.openexr    | ERROR IMB_exr_write_channels: Failed to write pixel data to image file "/root/writing-full-disk/mnt/img_0001.exr". No space left on device.
00:01.959  render           | Saved: '/root/writing-full-disk/mnt/img_0001.exr'
00:02.005  image.openexr    | ERROR IMB_exr_begin_write: Cannot open image stream "/root/writing-full-disk/mnt/img_0002.exr". No space left on device.
Error: Error writing render result, No space left on device (see console)
Error: Render error (No space left on device) cannot save: '/root/writing-full-disk/mnt/img_0002.exr'
raised: RuntimeError Error: Error writing render result, No space left on device (see console)
Error: Render error (No space left on device) cannot save: '/root/writing-full-disk/mnt/img_0002.exr'

Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)

Blender quit
exit code: 1
```

On the Mac this version named the cause in both lines too, but named no file in the second: `IMB_exr_begin_write: No space left on device.` On Linux the same line also names the stream it could not open.

*What was left on the little volume.*

```
-rw-r--r-- 1 root root 1945600 Aug  7 01:23 img_0001.exr
-rw-r--r-- 1 root root       0 Aug  7 01:23 img_0002.exr
Filesystem     1K-blocks  Used Available Use% Mounted on
/dev/loop5          1968  1928         0 100% /root/writing-full-disk/mnt
```

One part-written first frame of 1,945,600 bytes, which Blender had announced as
`Saved`, and an **empty second file** of no bytes at all. The render stopped at frame 2;
frame 3 was never attempted. The volume was completely full afterwards.

*Is the part-written frame a picture?* No. It was compared as a picture and not byte for
byte: the kept file and the control's whole first frame were both handed to Blender's
own image reader in the same version that wrote them. The whole one opened as a
2000 by 2000 picture with four channels and sixteen million numbers of colour, none of
them zero. The part-written one would not open at all — Blender refused it as an unknown
kind of file, reported it as 0 by 0 with no channels, and gave back no colour:

```
00:01.767  image.openexr    | ERROR imb_load_openexr: Unable to query scanline information
00:01.775  image.read       | ERROR IMB_load_image_from_memory: unknown file-format (/root/writing-full-disk/kept/5.0/img_0001.exr)
---- file: /root/writing-full-disk/roomy/5.0/img_0001.exr
     bytes on disk: 64032656
     Blender says size: (2000, 2000)  channels: 4
     numbers of colour read: 16000000
     expected for (2000, 2000) : 16000000
     numbers that are not zero: 16000000
---- file: /root/writing-full-disk/kept/5.0/img_0001.exr
     bytes on disk: 1945600
     Blender says size: (0, 0)  channels: 0
     numbers of colour read: 0
     expected for (0, 0) : 0
     numbers that are not zero: 0
==== the two pictures are not even the same shape: 16000000 against 0
```

*The plain contrast.* On the same full volume, an ordinary write from the shell failed
and said why. Run once, after the 4.5 run, while the volume was still full:

```
bash: line 4: echo: write error: No space left on device
redirect exit code: 1
dd: error writing '/root/writing-full-disk/mnt/shell_dd.bin': No space left on device
1+0 records in
0+0 records out
0 bytes copied, 0.000154225 s, 0.0 kB/s
dd exit code: 1
```

*Did Linux agree with the Mac?* **Yes, on everything that matters, and the two are never
folded into one answer.** On both machines the first frame is part-written and announced
as `Saved`, the render stops at frame 2, frame 3 is never attempted, and a windowless
Blender ends with exit code 1. Two things differ. Linux leaves an **empty second file**
behind where the Mac's volume held only the first. And the wording is not the same, as
set out above. The byte counts differ too — 1,945,600 on Linux against 1,908,736 on the
Mac — but the two volumes had different amounts of room, so that is the disk and not
Blender.

*And how this sits beside the other Linux disk-full run above.* The run recorded higher
up under 2026-08-06 filled the machine's real disk with ballast first and then rendered
**one** frame of a default scene as a **PNG**, and got exit code 0 with no error line at
all. This run rendered a **three-frame animation** of **OpenEXR** files onto a small
volume that filled part way through, and got exit code 1 with the errors quoted. Both
were on Linux. What differed between them is the file format, the number of frames, and
whether the disk was already full before Blender started. Nothing here says which of
those made the difference; that is untested.

*Standing lines, brought up to date.* *(This round, 2026-08-07.)* **Driven from
outside** — settled. Every run above had no window and was started from the command
line; the failure is announced on the terminal in the words quoted, and the process ends
with exit code 1. **Operating system** — running out of room is now settled on **two**
systems, macOS and Linux, and they agree on what the render does. Windows is untried: no
Windows machine has run it.


## 6. What are the ways a render can be started, and what does each of them write?

**Answer.** The sources name these ways. I give what each is documented to write; where writing is not stated, I say so.

**From the Render menu, in the window** (manual, Topbar):

> Render Image F12
>
> Render the active scene at the current frame.

> Render Animation Ctrl-F12
>
> Render the animation of the active scene.

> Render Sequencer Image Alt-F12
>
> If a Sequencer Scene exists that differs from the active scene, render that scene instead.

> Render Sequencer Animation Ctrl-Alt-F12
>
> Render the animation of the Sequencer Scene.

> Render Audio
>
> Mix the scene's audio to a sound file.

What the still and the animation write is stated on the Rendering Animations page, and it is the key difference:

> While rendering stills will allow you to view and save the image from the render buffer when
> it is complete, animations are a series of images, or frames,
> and are automatically saved directly out to a drive after being rendered.

The contrast supports an inference: **Render Image does not save the image automatically** (the result sits in the render buffer to be viewed and saved by hand), while **Render Animation writes the frames automatically** to the output path. Render Audio writes a sound file. The sequencer variants are documented as rendering a different scene, not as writing differently; the manual adds only: "Both of the sequencer render options automatically set the Show Sequencer Scene toggle in the image editor Render Result after rendering."

**Viewport render, in the window** (manual, Viewport Render):

> To render a still image, use 3D Viewport ‣ View ‣ Render Viewport Preview.

> To render an animation, use 3D Viewport ‣ View ‣ Render Playblast.

> To render an animation, but only those frames that have a keyframe, use 3D Viewport ‣ View ‣ Render Playblast on Keyframes. This only renders those frames for which the selected objects have an animation key. The other frames are still written to the output, but will simply repeat the last-rendered frame.

The page says viewport render uses "some output settings … Output path, File format", and that it "only works for the Workbench and EEVEE render engines. It's not supported for Cycles." It states that Render Playblast on Keyframes writes frames, as quoted above. It does not state whether Render Viewport Preview saves its still automatically, or describe the writing behaviour of ordinary Render Playblast in the quoted words.

**From the command line** (manual, Command Line Arguments):

> `-a, --render-anim`
>
> Render frames from start to end (inclusive).

> `-f, --render-frame <frame>`
>
> Render frame `<frame>` and save it.

Note the wording difference in the source itself: `-f` says "and save it", while the in-window Render Image does not save. `-f` also accepts more than one frame: "`+<frame>` start frame relative, `-<frame>` end frame relative. A comma separated list of frames can also be used (no spaces). A range of frames can be expressed using `..` separator between the first and last frames (inclusive)."

**From Python** (API, Render Operators):

> `bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)`
>
> Undocumented, consider contributing.

with

> `write_still` (boolean, (optional)) – Write Image, Save the rendered image to the output path (used only when animation is disabled)

and the viewport equivalent:

> `bpy.ops.render.opengl(*, animation=False, render_keyed_only=False, sequencer=False, write_still=False, view_context=True)`
>
> Take a snapshot of the active viewport

The `write_still` description documents that setting it to true saves a non-animation render to the output path. **LABEL: inferred** that leaving it false means the output path is not written, and that an animation run writes as the manual says animations do. The operator's own description is literally "Undocumented, consider contributing".

**What is written alongside, in any of these.** The Output panel switches, quoted from the manual and the API:

> Overwrite existing files when rendering. (`use_overwrite`, "Type: boolean, default True")

> Create empty placeholder files while rendering frames (similar to Unix 'touch') (`use_placeholder`, "Type: boolean, default False")

> Saving – Cache Result
>
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image. … The image is stored in the Render Cache folder as specified in the File Paths Preferences.

And File Output nodes write independently of the above — "The File Output node writes one or more images to disk during rendering or compositing."

**LABEL: documented** for the named entry points and for the words quoted about what they render or write.

**LABEL: inferred** that the in-window Render Image does not save automatically. The step is the manual's contrast between a still that can be saved from the render buffer and an animation that is automatically saved.

**LABEL: inferred** that `-a` always writes and that its writing differs from the in-window Render Image. The sources say `-f` "Render frame <frame> and save it" and the 5.0 compositor release notes say "Rendering through the command line always does an animation render"; putting those statements together with the manual's animation description is a reader-supplied step.

**Not covered.** This research does not establish that the list of render entry points is complete. It also does not establish what Render Viewport Preview or ordinary Render Playblast writes. Baking is not treated here — the question asks about renders. Nothing says what the sequencer render variants write differently. Nothing states whether Render Audio uses the same output path field. Nothing says what an aborted render (Esc) leaves.

**Quoted from:** the manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html , https://docs.blender.org/manual/en/5.0/render/output/animation.html , https://docs.blender.org/manual/en/5.0/editors/3dview/viewport_render.html , https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html , https://docs.blender.org/manual/en/5.0/render/output/properties/output.html , https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html ; the API documentation, https://docs.blender.org/api/5.0/bpy.ops.render.html and https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html ; the release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/
**Date read:** 2026-07-30

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The command-line and Python routes are the outside-the-interface ones, and they are documented as such. One documented behaviour difference is recorded in the 5.0 release notes for the File Output node:

> The **File Output** node now only appends the frame number to the image name when doing an
> animation render. Rendering through the command line always does an animation render, so no change
> of behavior is expected. Rendering using the `render.render()` operator when `animation=False`
> will see the change of behavior…

The menu items, shortcuts and viewport-render entries are inside-the-window only by their nature; the sources do not give command-line equivalents for viewport render.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say the ways of starting a render differ by operating system. Two small, related notes: `-con, --start-console` is marked "(Windows only)", and `--debug-ghost` "(Linux only)". Neither is about starting a render.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 22 — what `-f`, `-a` and the Python render call each write.**

*What was done, to what.* One saved file holding both a scene output path and a
compositor File Output node was copied three times, and each copy rendered by one
route, each with its own `-o` folder so the routes could not see each other's work.

*The commands.*

```
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup …/r_f.blend  -o …/f_scene/s_ -F PNG -x 1 -f 1
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup …/r_a.blend  -o …/a_scene/s_ -F PNG -x 1 -a
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup …/r_py.blend -o …/py_scene/s_ -F PNG -x 1 --python q22.py
```

where `q22.py` calls `bpy.ops.render.render` three times: `animation=False,
write_still=False`, then `animation=False, write_still=True`, then `animation=True`.

*What came back.* From `-f 1`:

```
Executing compositor
Saved: '…/node_out/nd_Image0001.png'
Saved: '…/f_scene/s_0001.png'
exit code: 0
```

From `-a`:

```
Executing compositor
Saved: '…/node_out/nd_Image0001.png'
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
00:00.395  render           | Saved: '…/node_out/nd_Image.png'      (animation=False, write_still=False)
00:00.401  render           | Saved: '…/node_out/nd_Image.png'      (animation=False, write_still=True)
00:00.402  render           | Saved: '…/py_scene/s_.png'            (animation=False, write_still=True)
00:00.408  render           | Saved: '…/node_out/nd_Image0001.png'  (animation=True)
00:00.409  render           | Saved: '…/py_scene/s_0001.png'        (animation=True)
```

So the File Output node writes on all three, but its name carries the frame number
only on the animation call.

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

**Entry 23 — switching automatic saving off.** There is no such switch in 5.0 — see question 5 above for the exact words.

*Standing lines, brought up to date.* **Driven from outside** — the command-line and
Python routes are settled by the runs above. The viewport route is settled too, in
the other direction: it refuses to run with no window. What the menu entries do is
still open and sits with the Creator (entries 26 to 28 of the hand-test list).
**Operating system** — settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.0.1**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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
Render Viewport Preview
Render Playblast
Render Playblast on Keyframes
```

and their tooltips:

```
Render Viewport Preview        Take a snapshot of the active viewport.
                               Python: bpy.ops.render.opengl()

Render Playblast               Render the viewport for the animation range of this scene.
                               Python: bpy.ops.render.opengl(animation=True)

Render Playblast on Keyframes  Render the viewport for the animation range of this scene,
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
--- animation=False, write_still=False   → Saved: '…/node_out/nd_Image.png'
--- animation=False, write_still=True    → Saved: '…/node_out/nd_Image.png'
                                           Saved: '…/py_scene/s_.png'
--- animation=True                       → Saved: '…/node_out/nd_Image0001.png'
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
engine: BLENDER_EEVEE
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

**Answer.** For an in-window still render, the manual documents a render result held in Blender. It does not establish that the same result remains available after a background render. The manual states the split:

> While rendering stills will allow you to view and save the image from the render buffer when
> it is complete, animations are a series of images, or frames,
> and are automatically saved directly out to a drive after being rendered.

The thing left behind is addressable in the Image Editor. The manual's Image Editor page says the data-block selector includes:

> Render Result: displays renders. When this item is selected, the Slot,
> View Layer, and Render Pass selectors become available (see below).

and about slots:

> The render slot to view (and render to). You can create new renders without
> losing previous ones by selecting an empty slot before rendering.

The Python switch that produces this state is documented:

> `write_still` (boolean, (optional)) – Write Image, Save the rendered image to the output path (used only when animation is disabled)

The description documents what `write_still=True` does. **LABEL: inferred** that `animation=False` and `write_still=False` leave the output path unwritten.

The manual gives adjacent information about temporary storage:

> The temporary directory is used to store various files at run-time
> (including render layers, physics cache, copy-paste buffer and crash logs).

and its location is picked as:

> User Preference (see File Paths).
>
> Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms).
>
> The /tmp/ directory.

This does not establish that every no-output render leaves a temporary file. Separately, if Cache Result is on, the render is not set up to write no files: a cache file is written even though the output path is not used.

> Saves the rendered view layers and their passes to a multi-layer OpenEXR image. … The image is stored in the Render Cache folder as specified in the File Paths Preferences.
> You can also load it back into the Image Editor's Render Result, even after closing
> and reopening Blender; see Open Cached Render.

with the matching operator described as: "Find the render cache file for the current scene and load it into the Render Result. … Note that Blender doesn't create these cache files by default."

**LABEL: documented** for the in-window Render Result and for what Cache Result writes. **LABEL: not in the sources** for whether a no-file render leaves temporary files behind.

**Not covered.** How long the Render Result survives, whether it survives loading another file, and how many slots there are, are not stated. Whether a still render in background mode (`-b`) leaves a Render Result at all, given there is no Image Editor to show it, is not stated anywhere. Whether the temporary "render layers" files are removed when Blender exits is not stated. Whether the Render Result is saved into the blend-file is not stated.

**Quoted from:** the manual, https://docs.blender.org/manual/en/5.0/render/output/animation.html , https://docs.blender.org/manual/en/5.0/editors/image/introduction.html , https://docs.blender.org/manual/en/5.0/editors/image/editing.html , https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html , https://docs.blender.org/manual/en/5.0/render/output/properties/output.html ; the API documentation, https://docs.blender.org/api/5.0/bpy.ops.render.html
**Date read:** 2026-07-30

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* This is the weak point. Everything the sources say about what is left behind is framed around the Image Editor and the render buffer, which need a window. The sources do not say what a background render with `write_still=False` leaves behind, or whether anything can be retrieved from it. The temporary-directory statement and the Cache Result statement are not marked as UI-only, but nothing says they hold in background mode either.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The temporary directory differs by system, and the manual says so: "Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms)" and "The /tmp/ directory". The 5.0 release notes also record a change here: "Removed use of the undocumented `TMP` environment variable on UNIX, use `TMPDIR` instead." (Note that this contradicts the manual page's "TMP & TMP_DIR on other platforms" wording; the command-line arguments page lists "TMPDIR: Store temporary files here (UNIX Systems). The path must reference an existing directory or it will be ignored." I am reporting all three as they stand rather than picking one.) Nothing else in this answer is stated to differ by system.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 20 — a windowless render told not to save.**

*What was done, to what.* At factory settings, with no window, the render cache and
placeholders switched off, the output path pointed at an empty folder, and
`bpy.ops.render.render(animation=False, write_still=False)` called. The image
data-blocks were listed before and after, and the temporary folder and the output
folder were listed afterwards.

*The command.*

```
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup --python q20.py -- …
```

*What came back.*

```
images before: [('Render Result', 'RENDER_RESULT')]
returned: {'FINISHED'}
images after : [('Render Result', 'RENDER_RESULT', (0, 0))]
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

**Entry 23 — switching automatic saving off.** There is no switch for turning automatic saving off in 5.0 — see question 5.

*Standing lines, brought up to date.* **Driven from outside** — settled. The Render
Result exists in a windowless Blender and can be reached from a script, and a render
told not to save leaves nothing at all on disk. What becomes of it after the process
ends was not tested and cannot be: the process is gone. **Operating system** —
settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.0.1**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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
images after : [('Render Result', 'RENDER_RESULT', (0, 0))]
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

**Answer, in the order the sources describe it.**

**Step 1 — templates are expanded.** "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

**Step 2 — a leading `//` is expanded to the blend-file's directory.** "This setting expands Relative Paths where a `//` prefix represents the directory of the current blend-file."

**Step 3 — the frame number goes in.** Where it lands depends on whether the recorded name contains `#`:

> When rendering an animation,
> the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png).
> You can set a custom padding size by adding the appropriate number of `#` anywhere in the file name
> (e.g. `image_##_test.png` translates to `image_01_test.png`).

The command-line page is more explicit about the no-`#` case:

> When the filename does not contain `#`, the suffix `####` is added to the filename.
>
> The frame number will be added at the end of the filename, eg:
>
> `blender -b animation.blend -o //render_ -F PNG -x 1 -a`
>
> `//render_` becomes `//render_####`, writing frames as `//render_0001.png`

So: **with `#` marks, the frame number sits exactly where the `#` run is, padded to the length of that run; with no `#` marks, it is appended at the end of the name, padded to four digits.**

**Step 4 — the end of the name.** Two things decide it.

First, the file extension, which is a switch, not part of the recorded path:

> Saving – File Extensions
>
> Adds the correct file extensions per file type to the output files.

> `use_file_extension` — Add the file format extensions to the rendered file name (eg: filename + .jpg). Type: boolean, default True

> `file_extension` — The file extension used for saving renders. Type: string, default "", (readonly, never None)

and from the command line:

> `-x, --use-extension <bool>`
>
> Set option to add the file extension to the end of the file.

Which extension is chosen follows from the file format; the manual's table of supported image formats lists them (for example JPEG "`.jpg .jpeg`", OpenEXR "`.exr`", PNG "`.png`").

Second, in multi-view renders, a per-view suffix. The API says of a scene render view:

> `file_suffix` — Suffix added to the render images for this view

> `camera_suffix` — Suffix to identify the cameras to use, and added to the render images for this view

and the enum item for the Individual views format says: "Individual files for each view with the prefix as defined by the scene views."

**The one call that does the whole expansion.** The API documents a method that returns the finished name:

> `frame_path(*, frame=-2147483648, preview=False, view='')`
>
> Return the absolute path to the filename to be written for a given frame

with parameters "frame (int …) – Frame number to use, if unset the current frame will be used", "preview (boolean …) – Preview, Use preview range", "view (string …) – View, The name of the view to use to replace the "%" chars", returning "File Path, The resulting filepath from the scenes render settings".

**LABEL: observed** — that Blender's shipped Cycles add-on uses this method to build the paths it checks for a completed denoise-animation sequence. Cycles' own files say:

```
            # Denoise animation sequence with expanded frames matching
            # Blender render output file naming.
            in_filepath = scene.render.filepath
            …
            for frame in range(scene.frame_start, scene.frame_end + 1):
                scene.render.filepath = in_filepath
                filepath = scene.render.frame_path(frame=frame)
                in_filepaths.append(filepath)

                if not os.path.isfile(filepath):
```

**File:** `Blender Sources/cycles_5.0/operators.py`, inside `CYCLES_OT_denoise_animation.execute`, lines 70–84. *(Citation check, 2026-08-05: line range corrected from 70–83; the last quoted line, `if not os.path.isfile(filepath):`, is line 84.)*

What is observed is what the file *says*: the add-on sets `scene.render.filepath`, calls `frame_path(frame=…)`, and then checks that exact path with `os.path.isfile`. That a render actually writes to that same name is a step beyond the file — **LABEL: inferred** for that step, and the step is: the add-on treats `frame_path()` output as the name a completed render left on disk (it errors with "Frame '%s' not found, animation must be complete" otherwise), so `frame_path()` must match what the renderer writes.

**The File Output node has its own shape.** The manual states the assembly directly:

> Each connected input is saved to the specified directory and filename,
> with the frame number appended to create a sequence.
> For example: `{Directory}/{File Name}{Socket Name}{frame number}.{extension}`.

Two 5.0 changes bear on this, from the release notes:

> The **File Output** node now only appends the frame number to the image name when doing an
> animation render. … Finally, adding `####` manually in the name should
> substitute the frame number even in single renders.

and the node now shows the answer in the UI: "Output Paths … Displays the final file path and extension for each input/layer, based on current settings."

**LABEL: documented** for steps 1–4 and for the File Output node shape; **observed / inferred** as marked for the Cycles finding.

**Not covered.** The exact ordering when several of these apply at once — for example whether a multi-view suffix goes before or after the frame number, or before or after the extension — is not stated. Whether `#` runs are honoured outside the file-name portion is not stated. What happens when `use_file_extension` is off and the recorded name already ends in an extension is not stated. Whether the `-x` argument and the panel switch are the same setting is not said in so many words. Whether `%` is expanded during a real render, as opposed to inside a `frame_path()` call, is not stated.

**Quoted from:** the manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html , https://docs.blender.org/manual/en/5.0/render/output/properties/output.html , https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html , https://docs.blender.org/manual/en/5.0/files/media/image_formats.html , https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html ; the API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html , https://docs.blender.org/api/5.0/bpy.types.SceneRenderView.html , https://docs.blender.org/api/5.0/bpy_types_enum_items/views_format_multiview_items.html ; the release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/ ; and Cycles' own file `Blender Sources/cycles_5.0/operators.py`.
**Date read:** 2026-07-30

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The `-o` and `-x` arguments and the `####` behaviour are documented on the command-line page, so the naming holds outside the window. One documented difference exists for the File Output node: "Rendering through the command line always does an animation render, so no change of behavior is expected", while `render.render()` with `animation=False` behaves differently. `frame_path()` is a Python method and needs no window.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not stated to differ. The only path-shape note is `bpy.path.native_pathsep`, "Replace the path separator with the systems native os.sep", and `bpy.path.resolve_ncase`, "Resolve a case insensitive path on a case sensitive system" — both helpers, neither described as part of render naming.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

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

That is the case this answer records as not covered by the sources: the switch off
and the recorded name already ending in an extension.

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
carries a `#` — `{scene_name}`, with the scene renamed `sc##ne`:

```
scene name is now: 'sc##ne'
frame_path: '…/tmpl/sc07ne_.png'
Saved: '…/tmpl/sc01ne_.png'
```

So the template goes in first, and the hashes it brings are then read as a place for
the frame number.

**Entry 16 — how a File Output node's name is built.** With the node's folder
`…/node_out/`, its file name `nd_` and one input named `Image`:

```
Saved: '…/node_out/nd_Image0001.png'
```

so the shape is the node's folder, then its file name, then the input's name, then
the frame number, then the extension.

*Standing lines, brought up to date.* **Driven from outside** — every name above was
produced with no window at all. **Operating system** — settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.0.1**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 30 — what a File Output node shows on its face.**

*What was done, to what.* A file holding a File Output node wired to a Render Layers
node was opened with the matching Blender, the Compositing workspace shown, and the
node looked at; then the node selected and the sidebar opened. Two screenshots per
version, kept at `/Users/artist/Desktop/Screenshots/010/`, one folder per version.

*One thing to record plainly.* The file used was not the one this entry named. It was
a file prepared for another topic — its node's own Output Paths panel points into that
topic's folder — and it carries three inputs in 4.5 and two in the other three rather
than the one the entry described. What is written below is what the screenshots show.

*What is on the node's face.* Two stacked text fields with no labels, the upper
holding `//out/` and the lower holding `shot`, then one row per input:

```
File Output
  [ //out/ ]
  [ shot   ]
  ○ passes_diffuse
  ○ beauty
  ○                        (empty)
```

The input named `passes/diffuse` when it was typed appears here as
`passes_diffuse` — the slash has become an underscore.

*What is in the sidebar.* A **Node** panel holding Name (`File Output`), Label
(empty) and Color. Then a **Properties** panel holding, in this order:

```
[ //out/ ]
[ shot   ]
Image | Multi-Layer EXR                    (Multi-Layer EXR chosen)
Node Format
            [x] Save as Render
Color Depth         Float (Half) | Float (Full)   (Full chosen)
Codec               None
            [ ] Interleave
Color Management    Follow Scene | Override       (Follow Scene chosen)
Layers
  ● passes_diffuse
  ● beauty                                  (selected)
Socket Type         Color
Output Paths
  /Users/artist/Documents/Smoozy hand tests/Reading a file/13/out/shot.exr
Custom Properties
  [ + New ]
```

So the node records two path fields plus a name per input, and it carries a panel
called **Output Paths** that shows the finished name before anything is rendered.

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
`nd_Image0001.png`.

*The two machines.* All of these were run on both. **They agreed** in this version,
name for name.

*Standing lines, brought up to date.* **Driven from outside** — every run had no
window. **Operating system** — now settled on two systems, macOS and Linux, which
agreed. Windows is untried.


## 9. Can the place a file output node writes to be settled for a render — at the moment the render is started, and by writing it into the file from outside Blender?

**This question is ambiguous, and I say so plainly.** "Writing it into the file from outside Blender" can mean (a) using Blender itself, driven from a command line or script, to put the path into the blend-file and save it, or (b) editing the `.blend` on disk with some tool that is not Blender. I answer reading (a), which I think is meant. Reading (b) is addressed at the end.

**Answer, part one — at the moment the render is started.**

I found **no dedicated command-line argument for the File Output node's path.** The command-line arguments page lists one output-path argument, `-o, --render-output <path>`, described as "Set the render path and file name", and the File Output node is nowhere on that page. I read the whole page and searched all four source sets for `file output`, `output node`, `directory`, `file_name` in a command-line context; nothing.

**LABEL: not in the sources** for a dedicated argument, with the record above.

What *is* documented is that the node's two path parts are ordinary writable Python properties:

> `directory` — The directory where the image will be written
>
> Type: string, default "", (never None, blend relative `//` prefix supported, Supports template expressions)

> `file_name` — The base name of the file. Other information might be included in the final file name depending on the node options
>
> Type: string, default "", (never None, Supports template expressions)

Neither is marked `(readonly)`, unlike, say, `file_output_items` which is marked "(readonly)". And Blender can be given Python at launch:

> `-P, --python <filepath>` — Run the given Python script file.

> `--python-expr <expression>` — Run the given expression as a Python script.

with the same ordering rule as `-o`: "Arguments are executed in the order they are given", and "loading the blend-file overwrites the render output that was set."

**LABEL: inferred** for the conclusion that the node's path can therefore be set at the moment the render is started. The step a reader has to supply: the properties are writable, Blender runs a supplied script at launch, and arguments run in order — so a script placed after the blend-file and before `-f`/`-a` can set them before the render happens. The sources never write that sentence.

There is also a route that needs no path-setting at all, because 5.0 added template variables usable in node paths. The manual: "Available only in path properties on a node: `node_name`: The name of the node that the path property is on", and the render-output variables `scene_name`, `camera_name`, `blend_name`, `blend_dir` are usable in "the output paths in the compositor's File Output node". **LABEL: documented.**

**Answer, part two — writing it into the file from outside Blender (reading (a)).**

Documented ingredients: Blender can be run with no window ("`-b, --background` — Run in background (often used for UI-less rendering)"), can be given a script, and can save:

> `bpy.ops.wm.save_mainfile(...)`
>
> Save the current Blender file
>
> `filepath` (string, (optional, never None)) – File Path, Path to file

**LABEL: inferred** that these compose into "set the node's directory and file_name from a script, then save the blend-file, without a window". The step: each piece is documented on its own; the sources never describe the combination.

**Reading (b) — editing the `.blend` with something that is not Blender.** **LABEL: not in the sources.** I searched the manual, API documentation and release notes for the blend-file's on-disk layout in terms that would allow outside editing (`blend-file format`, `file format`, `DNA`, `compatibility`, `compression`) and found nothing that describes reading or writing node path strings from outside. The one adjacent 5.0 note is about compression, not structure: "Blendfile Compression on saving is now enabled by default … Users who still want to write uncompressed blendfiles by default will have to edit and save again their user preferences."

**Not covered.** Whether `-o` has any effect at all on File Output nodes (I found no statement either way — the sources only tie `-o` to "the render path and file name"). Whether the node path is per-node-tree, and what happens with linked or overridden node groups. Whether a script that sets these values before `-a` reliably survives the render. Whether saving a blend-file in background mode has any caveats.

**Quoted from:** the API documentation, https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html and https://docs.blender.org/api/5.0/bpy.ops.wm.html ; the manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html and https://docs.blender.org/manual/en/5.0/files/file_paths.html ; the release notes, https://developer.blender.org/docs/release_notes/5.0/core/ and https://developer.blender.org/docs/release_notes/5.0/python_api/
**Date read:** 2026-07-30

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The whole answer is about outside the interface. The one documented behaviour difference for this node between the two worlds is the 5.0 release-note change already quoted in Q6 and Q8: "Rendering through the command line always does an animation render, so no change of behavior is expected. Rendering using the `render.render()` operator when `animation=False` will see the change of behavior, but setting that to True and providing `start_frame` and `end_frame` should restore the old behavior." (Note the release note names `start_frame`/`end_frame`; the API for the operator names them `frame_start` and `frame_end`. I report both as written.)

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say any of this differs by operating system.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 16 — does `-o` reach a File Output node?**

*What was done, to what.* A file was saved holding a scene output path pointing at
one folder and a File Output node named `FileOut` pointing at another. Blender was
started with no window, given that file, then `-o` pointing at a third folder, then
`-f 1`. Afterwards the file was reopened and the node's stored values printed.

*The command.*

```
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup …/node.blend -o …/cli_out/cli_ -F PNG -x 1 -f 1
```

*What came back.*

```
Executing compositor
Saved: '…/node_out/nd_Image0001.png'
Saved: '…/cli_out/cli_0001.png'
exit code: 0
```

and reopening the file gave the node's stored values unchanged. So `-o` moves the
scene's own output and leaves the node where it was.

**Entry 17 — setting the node's place at the moment the render starts.**

*The command.*

```
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup …/node.blend --python-expr "import bpy; sc=bpy.context.scene; t=sc.compositing_node_group; n=t.nodes['FileOut']; n.directory='…/from_cli/'; n.file_name='zz_'; print('set to', n.directory, n.file_name)" -f 1
```

*What came back.*

```
set to …/from_cli/ zz_
Saved: '…/from_cli/zz_Image0001.png'
exit code: 0
```

So yes: the node's `directory` and `file_name` are ordinary writable values, a script
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
node directory: '/tmp/written_in/' file_name: 'ww_'
```

**Entry 19 — whether `bpy.data.file_path_foreach` reaches the node's place.**

```
has file_path_foreach: True
loaded control image: '…/cli_out/cli_0001.png'
  VISIT ("bpy.data.images['cli_0001.png']", "'…/cli_out/cli_0001.png'", 'None')
count: 1
```

The call exists and works — it found the ordinary image that was loaded as a control
— and it did **not** visit the File Output node's folder, nor the scene's own render
output path.

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
Saved: '…/node_out/nd_Image0001.png'
Saved: '…/cli_out/cli_0001.png'
```

and the node's stored values read back unchanged afterwards.

**Entry 19, part one — setting the node's place at the moment the render starts.** A
one-line script handed to Blender at launch set the node's folder and name, and the
render that followed used them:

```
set to …/from_cli/ zz_
Saved: '…/from_cli/zz_Image0001.png'
```

**Entry 19, part two — writing it into the file from outside.** A windowless Blender
set the same two values and saved the file; reopened, the values were still there:

```
Info: Saved "edit.blend"
node directory: '…/written_in/' file_name: 'ww_'
```

**Entry 19, part three — the call that walks every file path a blend-file holds.**

```
has file_path_foreach: True
count: 0
```

It exists and runs, and it visited **nothing** — neither the File Output node's
folder nor the scene's own output path.

*The two machines.* All of these were run on both. **They agreed** in this version.

*Standing lines, brought up to date.* **Driven from outside** — this is the whole of
what was tested: every run was driven from outside with no window. **Operating
system** — now settled on two systems, macOS and Linux, which agreed. Windows is
untried.


## 10. Can the place a render records for writing carry characters that the computer it lands on will not accept, and what does a render do when it does?

**Answer, first half — can it carry them?**

**LABEL: not in the sources.** The quoted sources do not establish whether the render output path can retain a character that the destination operating system refuses. The API describes the field as a string and does not state a character rule. The manual separately lists common text-field length limits:

> Text Fields
>
> Fixed strings are used internally, and while it is not useful to list all limits, here are some common limits.
> Used for data-block names, modifiers, vertex groups, UV layers…
>
> directory:
>
> 767
>
> file-name:
>
> 255
>
> file-path:
>
> 1023

with the note:

> Multi-byte encoding means some Unicode characters use more than a single ASCII character.

Two further pieces point the same way. The API offers a helper whose existence implies the path is not cleaned automatically:

> `bpy.path.clean_name(name, *, replace='_')`
>
> Returns a name with characters replaced that
> may cause problems under various circumstances,
> such as writing to a file.
>
> All characters besides A-Z/a-z, 0-9 are replaced with "_"
> or the replace argument if defined.

And the 5.0 release notes list, among fixed bugs:

> * Assert browsing a non UTF8 render output path [a229d0d08f6] - Backported to 4.5.2

which documents that a non-UTF-8 render output path reached the file browser. It does not establish that a render accepts an operating-system-forbidden character.

**LABEL: documented** — for the field's stated type, the general text-field limits, the `clean_name` helper, and the non-UTF-8 bug-fix entry.

The existence of `clean_name` does not establish whether rendering applies equivalent cleaning of its own, so no claim about automatic cleaning follows from it.

**Answer, second half — what does a render do when it does?**

**LABEL: not in the sources.**

**Where I looked, and with what words.** I searched all of `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0` for: `invalid character`, `illegal character`, `forbidden character`, `reserved name`, `sanitiz*`, `special character`, `non-ASCII`, `UTF-8`, `UTF8`, `unicode`, `colon`, `backslash`, `path separator`, `trailing space`, `case sensitive`, `case insensitive`, `denied`, `read-only file system`, `fails`, `failure`, `error message`. The only hit describing forbidden characters anywhere in the manual is about asset catalogs, not render paths: "Invalid characters: `:`, `\`." The only documented render-path failure mode is a template syntax error ("rendering an animation will fail with an error message indicating the path errors" — see Q3 and Q5), which is Blender's own syntax, not the computer's rules.

I read in full: Working Limits, File Paths, Output properties, the File Output node page, Relative Paths in Open & Save, `bpy.path`, and the command-line pages. None says what a render does when the destination name is one the system will refuse.

**Not covered.** Whether the render stops, skips the frame, substitutes characters, or reports anything. What happens when a path exceeds 767 / 255 / 1023 characters. Whether the answer differs between the scene output path and a File Output node. Whether a path that is fine on one system and refused on another is detected at all before the write is attempted.

**Quoted from:** the manual, https://docs.blender.org/manual/en/5.0/advanced/limits.html , https://docs.blender.org/manual/en/5.0/files/file_paths.html , https://docs.blender.org/manual/en/5.0/files/asset_libraries/catalogs.html ; the API documentation, https://docs.blender.org/api/5.0/bpy.path.html and https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html ; the release notes, https://developer.blender.org/docs/release_notes/5.0/bugfixes/
**Date read:** 2026-07-30

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not in the sources. The bug-fix entry names "browsing" a non-UTF-8 render output path, which is a file-browser act and so inside the window; nothing says what a background render does with the same path.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* This is exactly where the sources go quiet, and the question is precisely an operating-system question. The sources never say which characters any operating system refuses, nor that Blender's handling differs between them. The only system-shaped notes I found near paths are `bpy.path.native_pathsep` ("Replace the path separator with the systems native os.sep"), `bpy.path.basename` ("Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility"), and `bpy.path.resolve_ncase` ("Resolve a case insensitive path on a case sensitive system"). None of them is described as part of rendering.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

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
ERROR OpenImageIO write failed: Could not open file "…nnnn0001.png"
Error: Render error (Undefined error: 0) cannot save: '…nnnn0001.png'
raised: RuntimeError Error: Render error (Undefined error: 0) cannot save: '…nnnn0001.png'
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

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.0.1**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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
00:00.553  image.write      | ERROR OpenImageIO write failed: Could not open file "…/long/nnn…nnn0001.png"
Error: Render error (Success) cannot save: '…/long/nnn…nnn0001.png'
raised: RuntimeError Error: Render error (Success) cannot save: '…/long/nnn…nnn0001.png'
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

The one thing on that list left unrun for want of a machine to itself — what a render
does on Linux when the volume runs out of room — was run on 2026-08-07 on a rented
Linux machine with an NVIDIA GeForce RTX 4080 SUPER and no display server. Its result
is under question 5 above. Nothing on either list is now unrun.

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

2026-08-04 — Opus 5 (Claude Code) — Writing, Blender 5.0
