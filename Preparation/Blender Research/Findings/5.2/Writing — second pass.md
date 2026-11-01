# Writing — Blender 5.2 — second pass

**Sources consulted, and when.** Blender 5.2 manual (`manual_5.2`), Blender 5.2 Python API documentation (`api_5.2`), Blender 5.2 release notes (`release_notes_5.2`), and Cycles' own files (`cycles_5.2`) — all inside `Blender Sources/`, all taken on **2026-07-30**, which is the date read for every finding below. Manual pictures in `manual_5.2/_images` were opened where noted.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

Two helper agents were used, one for a sweep of `api_5.2` and one for a sweep of `cycles_5.2`. Each was given the full restriction warning word for word, was told to read only its one folder, was told not to read the question sheet, and was required to hand back exact quoted text with file and line rather than summaries. Both signed a declaration that they read only their assigned folder, started no helpers of their own, and quoted verbatim. I vouch for both. Every quotation of theirs that carries a claim below I re-checked in the file myself.

Questions answered this pass: **1 to 9**. Question 10 was not assigned and is not answered.

---

## 1. Can where a render writes be settled at the moment the render is started, without changing the file?

**Answer.** Yes, from the command line. Blender takes an argument that sets the render path and file name for that run.

**Label: documented.**

Exact words:

> `-o, --render-output <path>`
>
> Set the render path and file name.
> Use `//` at the start of the path to render relative to the blend-file.

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

The same page states that order matters, and that loading the blend-file undoes an output path set before it:

> `blender --background test.blend --render-frame 1 --render-output "/tmp"`
>
> …will not render to /tmp because `--render-frame 1` renders before the output path is set.
>
> `blender --background --render-output /tmp test.blend --render-frame 1`
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set.
>
> `blender --background test.blend --render-output /tmp --render-frame 1`
>
> …works as expected.

Source: same page, same date.

The format and the extension can be settled the same way:

> `-F, --render-format <format>`
>
> Set the render format.

> `-x, --use-extension <bool>`
>
> Set option to add the file extension to the end of the file.

Source: same page, same date.

A second route exists. Blender will run a script or an expression given at launch:

> `-P, --python <filepath>`
>
> Run the given Python script file.

> `--python-expr <expression>`
>
> Run the given expression as a Python script.

Source: same page, same date. And the render output path is a writable string property:

> `filepath`
>
> Directory/name to save animations, `#` characters define the position and padding of frame numbers (default “//”, never None, blend relative // prefix supported, Supports template expressions)
>
> Type:
>
> `str`

Source: Python API — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30.

**Label for the Python route: inferred.** The step I took: the sources say Blender runs a Python expression given on the command line, and separately say `filepath` is a settable string property (it is not marked readonly, unlike `file_extension` which is marked "readonly" on the same page). Putting those two together gives "a launch-time expression can set the output path". The sources nowhere show that combination in one place.

**Not covered.** The sources do not say whether either route leaves the blend-file on disk unchanged. Nothing on the command-line page says the file is written back, and the argument-order note treats the loaded scene as something in memory that a later argument overwrites, but neither statement is a claim about the file on disk. The sources also do not say what happens if `-o` is given a path that turns out to be unusable — see question 5.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* This *is* the outside-the-interface case. `-o` is a command-line argument, and the page's own worked example uses background mode (`blender -b animation.blend -o //render_ -F PNG -x 1 -a`). The sources do not describe any equivalent single control inside Blender's own window that sets an output path only for the current render without altering the stored setting.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say this differs by operating system. The command-line page marks the arguments that are platform-specific in their own text — `-con, --start-console` carries "(Windows only)", `--no-window-frame` carries "(Linux only)", `--qos <level>` carries "(Windows only)". `-o`, `-F`, `-x`, `-P` and `--python-expr` carry no such note. Reading the absence of a note as "works everywhere" is an inference, not something the page states.

**Where I looked.** `advanced/command_line/arguments.txt` read in full. Searched the whole manual for "output path", "render-output", "command line". Searched the whole API documentation for `filepath`, `render.filepath`, `frame_path`. The API's own change log for 5.1 to 5.2 was checked for changes to `RenderSettings` (it lists `anisotropic_filter`, `save_output`, `use_auto_generate_texture_cache`, `use_texture_cache` as added — no change to `filepath`).

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

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
"/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup …/test.blend -o …/elsewhere2/e_ -F PNG -x 1 -f 1
```

*What came back.*

```
--- before ---
c8c0bc78b57c86a268ee83705a18433074511fd7afff6da1d116fd0833fa41cb  …/test.blend
c8c0bc78b57c86a268ee83705a18433074511fd7afff6da1d116fd0833fa41cb  …/keep.blend
…/test.blend 96080 1785882851
Saved: '…/elsewhere2/e_0001.png'
render exit code: 0
--- after ---
c8c0bc78b57c86a268ee83705a18433074511fd7afff6da1d116fd0833fa41cb  …/test.blend
c8c0bc78b57c86a268ee83705a18433074511fd7afff6da1d116fd0833fa41cb  …/keep.blend
…/test.blend 96080 1785882851
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

Everything in the Linux sections of this document was run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. It is a rented container sharing somebody else's hardware, so it is given fewer cores than the chip really has and its system settings are not entirely its own; other work may have been running on it at the same time, so nothing here says how long anything took. Blender was started with no window unless an entry says otherwise, with its settings folder pointed at one of this topic's own and with the display variable unset so no window could be reached. The build used for this version was **Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)**. Date: 2026-08-06. In the output below, `…` stands for the working folder on that machine, which was `/root/smoozy/Writing/work/<entry>/5.2`. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests — Linux.md`.

**One note on how the output is shown.** From 5.0 every line Blender prints while rendering carries a timestamp and a channel name in front of it, like `00:00.617  render           | Saved: '…'`. Where a quoted line below carries no such prefix, the prefix has been dropped because the timestamp differs from one run to the next; nothing after the bar is changed.

**These results settle Linux and say nothing about macOS.** Where the same test was also run on the Creator's Mac, both results are given and neither is folded into the other.

**Entry 17 — does the output argument change the blend-file on disk?**

*What was done, to what.* A blend-file was saved with its output path recorded as
`…/stored/stored_`, a byte-for-byte copy kept beside it, and the file then rendered
with the output argument pointing at `…/elsewhere/e_`.

*The command.*

```
/opt/blender/blender-5.2.0-linux-x64/blender -b --factory-startup …/test.blend -o …/elsewhere/e_ -F PNG -x 1 -f 1
```

*What came back.* The two checksums and the size and modified time were identical
before and after:

```
--- checksums before
b92ab0fe224ae37fb060746f0eda17ad638cf72862f2f8fab3dc714af3f9d17f  …/test.blend
b92ab0fe224ae37fb060746f0eda17ad638cf72862f2f8fab3dc714af3f9d17f  …/keep.blend
…/test.blend 95980 1785990954
--- the render, with the output argument pointing somewhere else
Saved: '…/elsewhere/e_0001.png'
exit code: 0
--- checksums after
b92ab0fe224ae37fb060746f0eda17ad638cf72862f2f8fab3dc714af3f9d17f  …/test.blend
b92ab0fe224ae37fb060746f0eda17ad638cf72862f2f8fab3dc714af3f9d17f  …/keep.blend
…/test.blend 95980 1785990954
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

**Answer.** There are two separate recorded things, and they have different parts.

**The scene's render output.** It is one string, holding directory and base name together, plus a set of separate switches beside it.

**Label: documented.**

> `filepath`
>
> Directory/name to save animations, `#` characters define the position and padding of frame numbers (default “//”, never None, blend relative // prefix supported, Supports template expressions)

Source: Python API — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30.

The switches recorded beside it, all on the same page, quoted exactly:

> `save_output`
>
> Write frames to disk for animation renders (default True)

> `use_file_extension`
>
> Add the file format extensions to the rendered file name (eg: filename + .jpg) (default True)

> `use_overwrite`
>
> Overwrite existing files while rendering (default True)

> `use_placeholder`
>
> Create empty placeholder files while rendering frames (similar to Unix ‘touch’) (default False)

> `use_render_cache`
>
> Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes) (default False)

> `file_extension`
>
> The file extension used for saving renders (default “”, readonly, never None)

Source: same page, same date. The format itself is a separate block:

> `image_settings`
>
> (readonly, never None)
>
> Type:
>
> `ImageFormatSettings`

Source: same page, same date. And inside that block:

> `file_format`
>
> File format to save the rendered images as (default 'PNG')

> `media_type`
>
> The type of media to save (default 'IMAGE')
>
> Type:
>
> `Literal[‘IMAGE’, ‘MULTI_LAYER_IMAGE’, ‘VIDEO’]`

Source: Python API — https://docs.blender.org/api/5.2/bpy.types.ImageFormatSettings.html — read 2026-07-30.

The manual shows the same set as one panel. I opened the picture `_images/render_output_properties_output_panel.png`, named in the manual at https://docs.blender.org/manual/en/5.2/render/output/properties/output.html. Visible in it: a single wide text field containing `/tmp/` with a folder-browse button at its right, and four checkboxes below labelled **Overwrite**, **Placeholders**, **File Extensions**, **Cache Result**, then **File Format** (PNG), **Color** (BW / RGB / RGBA), **Color Depth** (8 / 16) and **Compression**. **Label for that: documented** (a manual picture is part of the manual). I take from it only what is visible: one field for the place, several separate switches, and the format chosen separately.

**The recorded string also carries flags describing what it accepts.** The API documents these as properties of the property itself:

> `is_path_output`
>
> Property is a filename, filepath or directory output (default False, readonly)

> `is_path_supports_blend_relative`
>
> Property is a path which supports the “//” prefix, signifying the location as relative to the “.blend” file’s directory (default False, readonly)

> `is_path_supports_templates`
>
> Property is a path which supports the “{variable_name}” variable expression syntax, which substitutes the value of the referenced variable in place of the expression (default False, readonly)

Source: Python API — https://docs.blender.org/api/5.2/bpy.types.Property.html — read 2026-07-30. The matching flag names are `OUTPUT_PATH`, `PATH_SUPPORTS_BLEND_RELATIVE` and `SUPPORTS_TEMPLATES` at https://docs.blender.org/api/5.2/bpy_types_enum_items/property_flag_items.html, same date.

**The File Output node's place.** It is recorded in three parts, not one.

> `directory`
>
> The directory where the image will be written (default “”, never None, blend relative // prefix supported, Supports template expressions)

> `file_name`
>
> The base name of the file. Other information might be included in the final file name depending on the node options (default “”, never None, Supports template expressions)

> `use_file_extension`
>
> Add the file format extensions to the saved file name (eg: filename + .jpg) (default False)

> `save_as_render`
>
> Apply render part of display transform when saving byte image (default False)

Source: Python API — https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30.

The third part is the per-input name:

> `name`
>
> (default “”, never None)

> `override_node_format`
>
> Use a different format instead of the node format for this file (default False)

Source: Python API — https://docs.blender.org/api/5.2/bpy.types.NodeCompositorFileOutputItem.html — read 2026-07-30. The manual says the same:

> The name of each input socket is used to construct the final file name.

Source: manual — https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30.

**How long each part may be.** The manual gives fixed internal limits:

> Fixed strings are used internally, and while it is not useful to list all limits, here are some common limits.
>
> `directory`:
>
> 767
>
> `file-name`:
>
> 255
>
> `file-path`:
>
> 1023

and immediately after:

> Multi-byte encoding means some Unicode characters use more than a single ASCII character.

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/limits.html — read 2026-07-30.

**Not covered.** How any of this is laid out as bytes inside the `.blend` file. The sources describe the properties and their limits, never the on-disk format of the file. Also not covered: whether the numbers on the limits page are counted in characters or in bytes — the note about multi-byte encoding is placed after the table but does not say which of the two the table means.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The same properties are what the Python API exposes, and Python is the interface used when Blender is driven from outside. Nothing in the sources says the recorded parts differ between a session with a window and one without.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say the recorded parts differ by operating system. The limits page gives one set of numbers with no per-system qualification. Note separately that `bpy.path.native_pathsep` exists — "Replace the path separator with the system’s native `os.sep`" (https://docs.blender.org/api/5.2/bpy.path.html, read 2026-07-30) — which shows Blender distinguishes its own separator from the system's, but the page does not say what the recorded string itself uses.

**Where I looked.** Read in full: `bpy.types.RenderSettings.txt`, `bpy.types.CompositorNodeOutputFile.txt`, `bpy.types.NodeCompositorFileOutputItem.txt`, `bpy.types.NodeCompositorFileOutputItems.txt`, `bpy.types.ImageFormatSettings.txt`, `bpy.types.Property.txt`, `bpy_types_enum_items/property_flag_items.txt`, `bpy_types_enum_items/property_subtype_string_items.txt`, `advanced/limits.txt`, `render/output/properties/output.txt`, `compositing/types/output/file_output.txt`. Searched the API for `base_path`, `file_slots`, `layer_slots`, `NodeOutputFileSlotFile`, `NodeOutputFileSlotLayer` — all returned nothing, so the older slot-based names are not what 5.2 documents.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 1 — what a fresh Blender records, and which switches sit beside it.**

*What was done, to what.* Blender was started with no window and factory settings,
and a short script printed the render output path, the built-in default behind it,
each of the output switches, and the preference that seeds a new scene.

*The command.*

```
"/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup --python q1.py
```

*What came back.*

```
startup filepath '/tmp/'
rna default     '//'
use_file_extension True True
file_extension True '.png'
use_overwrite True True
use_placeholder True False
use_render_cache True False
save_output True True
file_format PNG
media_type IMAGE
render_output_directory '//'
```

So the value a fresh Blender starts with is `/tmp/`, which comes from the startup
file rather than from the property's own built-in default; the built-in default is
'//'. The preference that seeds a new scene reads `//`.

**Entry 2 — what a File Output node records.**

*What was done, to what.* At factory settings, a compositor node tree was made and
one File Output node added to it; every property of the node was printed with its
kind and whether it can be written.

*The command.*

```
"/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup --python q2.py
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
  PROP panel_states COLLECTION readonly ''
  PROP save_as_render BOOLEAN writable False
  PROP use_file_extension BOOLEAN writable False
inputs ['']
```

(Only the properties bearing on where the node writes are quoted.) There is no
`base_path`, no `file_slots` and no `layer_slots`. `use_file_extension` is on this
node in 5.2. A new node starts with no items at all and one unnamed input.

**Entry 25 — how much text the path field actually holds.** Asking for a path
1343 characters long and reading it back gave a stored length of exactly 1023.
A path 443 characters long was stored whole. So the published 1023 limit is the
one enforced on the stored text.

*Standing lines, brought up to date.* **Driven from outside** — everything above
was read and set with no window open, so the recorded shape is the same thing a
driven Blender sees. **Operating system** — settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.2.0**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 29 — the switch in the Output panel's header.**

*What was done, to what.* Blender opened at its normal startup file, Properties ‣
Output tab, the panel headed Output, and the pointer rested on the header strip of
that panel.

*What came back*, in the Creator's own words:

> "This checkbox only exists in 5.2. Its tooltip says "Save Output / Write frames to
> disk for animation renders.""

The same checkbox is visible, ticked, in the header of the Output panel in the
screenshots taken for entry 31 for this version.

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

*What is on the node's face.* Two stacked text fields with no labels, each with its
own folder-browse button, the upper holding `//out/` and the lower holding `shot`,
then one row per input and a `+` for adding another:

```
File Output
  [ //out/ ]  📁
  [ shot   ]  📁
  ○ passes/diffuse
  ○ beauty
  ○                        (empty)
  +
```

The input named `passes/diffuse` keeps its slash here.

*What is in the sidebar.* A **Node** panel holding Name (`File Output`), Label
(placeholder text `Custom Label`), Color, `[x] Show Options`, `[ ] Mute` and
`Propagate  All Messages`. Then a **Properties** panel holding, in this order:

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
  ● passes/diffuse
  ● beauty                                  (selected)
Socket Type         Color
Output Paths
  [x] File Extensions
  /Users/artist/Documents/Smoozy hand tests/Reading a file/13/out/shot.exr
  +
Custom Properties
  [ + New ]
```

So the node records two path fields plus a name per input, and it carries a panel
called **Output Paths** holding a **File Extensions** checkbox of its own and the
finished name shown before anything is rendered.

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
VERSION 5.2.0 LTS
startup filepath '/tmp/'
rna default     '//'
use_file_extension True True
file_extension True '.png'
use_overwrite True True
use_placeholder True False
use_render_cache True False
save_output True True
file_format PNG
media_type IMAGE
render_output_directory '//'
```

**Entry 16 — what a File Output node records.** The node's own settings came back as
`active_item_index`, `directory`, `file_name`, `file_output_items`, `format`, `panel_states`, `use_file_extension`, with its inputs reading `['']`.

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

**Answer.** Six, and one of them has an escape form.

**Label: documented.**

**`//` at the start — the blend-file's own folder.**

> This setting expands Relative Paths
> where a `//` prefix represents the directory of the current blend-file.

Source: manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

> Use `//` at the start of the path to render relative to the blend-file.

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

> When relative paths are supported, the File Browser provides a Relative Path checkbox,
> when entering the path into a text field, use a double slash prefix (`//`) to make it so.

and:

> You cannot use relative paths into a new untitled blend-file.
> Save it before linking to external files.

Source: manual — https://docs.blender.org/manual/en/5.2/files/blend/open_save.html — read 2026-07-30.

**`#` — the frame number, and how many digits it gets.**

> When rendering an animation,
> the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png).
> You can set a custom padding size by adding the appropriate number of `#` anywhere in the file name
> (e.g. `image_##_test.png` translates to `image_01_test.png`).

Source: manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

> The `#` characters are replaced by the frame number, and used to define zero padding.
>
> `animation_##_test.png` becomes `animation_01_test.png`
>
> `test-######.png` becomes `test-000001.png`

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

The File Output node uses the same mark:

> Both the File Name and the Image names (when saving multiple images) can contain `####` to force the
> frame number to appear in a specific place in the filename. This also has the effect that a single render will
> have the frame number embedded. The frame number will be padded with zeros to fill the number of `#`
> characters.

Source: manual — https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30.

**`{` and `}` — a named value substituted when the path is used.**

> Path templates substitute template expressions (written `{abcd}`, where
> “abcd” is a variable name) in a filepath when the path is used.

and:

> This substitution happens internally at time of use (e.g. while rendering), so
> the template syntax will remain as-is in the path field.

and:

> Currently path templates are only supported for the render output path in
> Scene properties and the output paths in the compositor’s File Output
> node. More filepaths will support templates in future releases.

The names that may go inside, quoted exactly:

> Available in all paths that support template expressions:
>
> `blend_name`:
>
> The currently open blend file’s name (without the .blend).
>
> `blend_dir`:
>
> Path up to (but not including) the currently open blend file.
>
> `blend_name_lib`:
>
> Like `blend_name`, except that for path properties on
> library-linked data-blocks it will be the name of the library blend file
> that the data-block comes from.
>
> `blend_dir_lib`:
>
> Like `blend_dir`, except that for path properties on
> library-linked data-blocks it will be the path to the library blend file
> that the data-block comes from.
>
> Available only in render output paths (including in the compositor File Output node):
>
> `fps`:
>
> The frames per second of the current scene.
>
> `resolution_x` / `resolution_y`:
>
> The x and y resolution of the rendered image.
> This factors in the resolution scale as well, so if the scene resolution is
> 1000x600 and the scale is 50%, then `resolution_x` and `resolution_y`
> will be 500 and 300, respectively.
>
> `scene_name`:
>
> Name of the current scene.
>
> `camera_name`:
>
> Name of the current render camera.
>
> Available only in path properties on a node:
>
> `node_name`:
>
> The name of the node that the path property is on.

Source for all of the above: manual — https://docs.blender.org/manual/en/5.2/files/file_paths.html — read 2026-07-30.

**`:` inside braces — separates the name from a format specifier.**

> Format specifiers instruct Blender how to format the substituted value.
> They are written after a separating colon, like this:
>
> `dance_{fps:FORMAT}.png`

and:

> `dance_{fps:###}.png`: format as an integer with at least 3 digits.
>
> `dance_{fps:.###}.png`: format as a floating point number with exactly 3
> digits after the decimal point.
>
> `dance_{fps:###.##}.png`: format as a floating point number with at least 3
> digits for the integer part and exactly 2 digits for the fractional part.
>
> In all cases, the number of hash symbols (`#`) indicates the desired number of
> digits.

and:

> Format specifiers currently can only be used with variables that represent
> numerical values, not string values.

and:

> If no format specification is given, default formatting for the named variable
> is used (e.g. floating point for fps, integer for resolution).

Source: same page, same date. Note that inside a format specifier, `#` counts digits rather than standing for the frame number.

**`{{` and `}}` — an escape, giving one literal brace.**

> Because `{` and `}` are used for template expressions in paths that support
> them, when a literal `{` or `}` is desired in such a path they must be
> escaped by writing them double:
>
> `{{` translates to a single `{` in the final path.
>
> `}}` translates to a single `}` in the final path.

with worked examples:

> `my_weird}}_{{path.png -> my_weird}_{path.png`
>
> `//my_render_{{dir}}/{blend_name}.png -> //my_render_{dir}/dance.png`
>
> `//my_render_dir/{{{blend_name}}}.png -> //my_render_dir/{dance}.png`

Source: same page, same date.

**`%` — the name of a view, in multi-view rendering.**

> `view` (str) – View, The name of the view to use to replace the “%” chars (optional, never None)

Source: Python API — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30, in the description of `frame_path()`.

**`/` or `\` inside a File Output node input's name — a folder hierarchy.**

> When the Media Type is set to Image, input names are also used to construct
> subdirectories:
>
> If an input name contains a slash character (`/` or `\`),
> the path is interpreted as a directory hierarchy.
>
> Any missing directories are created automatically during output.
>
> This allows organizing outputs into folders directly from the node setup,
> for example: `passes/diffuse` or `masks/object_01`.

Source: manual — https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30.

**Not covered.** Whether `%` is honoured in the render output path field itself. The only place `%` is described is the `view` argument of `frame_path()`, which is a function that computes a path — the manual never mentions `%` in the Output panel or on the multi-view pages. Also not covered: whether `#` inside a `{...}` format specifier and `#` meaning the frame number can appear in the same path without confusion; the manual describes each separately and never shows both. Also not covered: whether `//` anywhere other than the start is special.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The `//` prefix and the `#` marks are documented directly on the command-line argument, which is the outside-the-interface route:

> You can use path templating features such as `{blend_name}` in the path.
> See Blender’s documentation on path templates for more details.

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30. So all three of `//`, `#` and `{}` hold when the path is supplied from outside. One difference is documented for `//` and it is not about the interface at all: it cannot be used before the blend-file has been saved. A second point worth flagging: `//` is Blender's own convention and is not understood by ordinary tools —

> Blender’s relative file paths are not compatible with standard Python modules such as `sys` and `os`.
> Built-in Python functions don’t understand Blender’s `//` prefix which denotes the blend-file path.

Source: Python API — https://docs.blender.org/api/5.2/info_gotchas_file_paths_and_encoding.html — read 2026-07-30.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say the meaning of any of these marks differs by operating system. The one place a system difference is documented near this subject is `bpy.path.basename` — "Equivalent to `os.path.basename`, but skips a “//” prefix. Use for Windows compatibility." (https://docs.blender.org/api/5.2/bpy.path.html, read 2026-07-30) — which flags a compatibility concern without saying the marks themselves behave differently.

**Where I looked.** `files/file_paths.txt` read in full — this is the page that carries the template rules. Also read in full: `advanced/command_line/arguments.txt`, `render/output/properties/output.txt`, `compositing/types/output/file_output.txt`, `files/blend/open_save.txt` (Relative Paths section), `bpy.path.txt`. Searched the whole manual and API for `%`, `%s`, "percent", "% chars", "substitution token", "token", `$`. The only `$` expansions found anywhere are for an external text editor's arguments (`$filepath`, `$line`, `$column`, `$line0`, `$column0` at https://docs.blender.org/manual/en/5.2/editors/preferences/file_paths.html) and are not output paths. "substitution tokens" occurs twice in the whole API, both as the description of `allow_path_tokens` on the image-save operators, not on render output. Searched the whole manual for "template" — the hits are `files/file_paths.txt` plus the two cross-references in `render/output/properties/output.txt` and `advanced/command_line/arguments.txt`.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

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
item names now : ['passes/diffuse', 'masks/object_01']
Saved: '…/nodebase/base_passes/diffuse0001.png'
Saved: '…/nodebase/base_masks/object_010001.png'
```

The slashes were kept, and the folders `base_passes` and `base_masks` were made.

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

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.2.0**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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

A box headed `Report: Error` also appeared inside the render window, carrying the same two lines. Nothing was written.

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

**Entry 6 — a slash inside a File Output node's input name.** In this version the slash was kept and the folder it implies was made:

```
item names: ['passes/diffuse', 'masks/object_01']
Saved: '…/nodebase/passes/diffuse0001.png'
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

**Answer.** Split. For one specific case it is stated outright. For the scene's render output path it is not stated anywhere.

**The File Output node's input-name subfolders — label: documented.**

> If an input name contains a slash character (`/` or `\`),
> the path is interpreted as a directory hierarchy.
>
> **Any missing directories are created automatically during output.**

Source: manual — https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30. (Emphasis mine; the words are exact.)

That sentence sits inside the node's **Inputs** section, in the list of bullets about what an input's name does. It is a statement about the folders implied by an input name. Whether it also covers the node's own **Directory** field when that folder is missing is not said on that page or anywhere else.

**The scene's render output path — label: not in the sources.**

Nothing in the manual, the API documentation, the release notes or Cycles' files says whether Blender creates a missing folder when writing an animation frame or a still to the render output path, and nothing says it does not.

**What the sources do say nearby, which is not the same thing.** Two other parts of Blender have this stated explicitly, which shows the manual does document such behaviour when it is there:

> If the cache path folder does not exist, it will be created.

Source: manual — https://docs.blender.org/manual/en/5.2/modeling/modifiers/physics/ocean.html — read 2026-07-30.

And in the File Browser, which is an interactive path field, creation is a prompt rather than automatic:

> Directory Ctrl-L
>
> The current folder path.
> Tab will auto-complete an existing path.
> If you type a nonexistent path, you will be prompted to create it.

Source: manual — https://docs.blender.org/manual/en/5.2/editors/file_browser.html — read 2026-07-30.

**Not covered.** Whether the File Output node's own **Directory** field is created when missing. Whether the scene render output folder is created when missing. Whether creation, where it happens, makes one level or a whole chain (the node wording says "directories", plural, and the example `passes/diffuse` is two levels deep — but that shows two levels below an existing base, not a missing base).

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Nothing in the sources ties folder creation to whether Blender has a window. The File Output node sentence says "during output", which is a render-time event, not an interface event. The File Browser prompt is by its nature interactive and could not apply to a background run, but that is a statement about the File Browser and not about rendering.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say. No page on this subject names an operating system.

**Where I looked.** Searched the whole manual and the whole release notes, case-insensitively, for: "directories are created", "created automatically", "missing directories", "create the directory", "does not exist", "will be created", "will create", "creates the folder", "creates a folder", "create folders", "nonexistent", "non-existent", "if the folder", "if the directory". The complete set of hits relevant to writing output is the two quoted above plus the File Browser prompt; every other hit was about vertex groups, bone collections, geometry-node bundles, smoke simulation, shape keys or brush catalogs. Searched the whole API documentation for `makedirs`, `mkdir`, `exist_ok`, "created automatically" — all returned nothing. Searched all of Cycles' files for `makedirs`, `mkdir`, `os.mkdir` — all returned nothing; Cycles' own files contain no Python `open(` call at all and write only temporary OSL files and a copied `.oso` beside its source. Read in full: `render/output/properties/output.txt`, `render/output/animation.txt`, `render/output/introduction.txt`, `compositing/types/output/file_output.txt`, `files/file_paths.txt`, `advanced/command_line/arguments.txt` — none of them mentions folder creation for the render output path.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 3 — the scene's own output path.**

*What was done, to what.* At factory settings, with no window, the render output
path was set to `…/one_missing/img_` (one folder that did not exist) and one frame
rendered; then to `…/two_a/two_b/img_` (two folders that did not exist) and one
frame rendered.

*The command.*

```
"/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup --python q3.py -- …
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

**Entry 6 — folders implied by an input's name.** In this version the folders implied by a slash in an input's name were made.

*The two machines.* All three were run on both. **They agreed**: a missing output
folder, and a whole chain of missing folders, is made on the Creator's Mac and on the
rented Linux machine alike.

*Standing lines, brought up to date.* **Driven from outside** — every run had no
window at all, so folder making is not something the interface does on Blender's
behalf, on either machine. **Operating system** — now settled on two systems, macOS
and Linux, which agreed. Windows is still untried: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**


## 5. What does a write do when its folder is not there, when there is no room left, or when it cannot be written to?

**Answer. Label: not in the sources.**

None of the three cases is described. The sources do not say what happens when the folder is missing, they do not say what happens when the disk is full, and they do not say what happens when the location cannot be written to.

**What the sources do describe, which is adjacent and must not be mistaken for it.** Three kinds of failure around output *are* documented, and all three are failures of the settings rather than of the disk.

A path that is malformed as a template fails before anything is written:

> Paths that support templates can have template errors, which prevent the
> path from being processed.

and:

> Depending on the path, template errors may prevent certain actions. For
> example, if there are errors in the render output path, then rendering an
> animation will fail with an error message indicating the path errors.

and how it shows:

> When there are template errors in a path, the path field will be highlighted red
> in the UI:

Source: manual — https://docs.blender.org/manual/en/5.2/files/file_paths.html — read 2026-07-30.

I opened the picture the manual names there, `_images/files_file_paths-invalid_path_template_tooltip.png`. Visible in it: an Output panel whose path field, holding `//my_render_dir/{blend_name.png`, is drawn with a red background, and a tooltip headed **Output Path** reading "Directory/name to save animations, # characters define the position and padding of frame numbers.", then "Value: //my_render_dir/{blend_name.png", then in red "Syntax error(s):" and "- Invalid or incomplete template expression '{blend_name.png'." **Label for that: documented.** I take from it only what is visible: the field turns red, and the tooltip names the offending expression. It shows nothing about disk conditions.

Having no output route at all is an error:

> When disabled, Blender does not write render output from this panel and instead
> relies on other systems. For animation renders, if the
> Video Sequencer is active and no alternative output is
> available, an error is raised. Otherwise, if compositing is enabled, active
> File Output Node nodes are used to save the result.
> If no such nodes exist, an error is reported.

and:

> Playblast operations also require an active output method. Both Render
> Playblast and Render Playblast on Keyframes will raise an error when render
> output is disabled in this panel.

Source: manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

And how any operator error reaches a script:

> If operator was cancelled but there wasn’t any reports from it with `{'ERROR'}` type,
> it will just return `{'CANCELLED'}` without raising any exceptions.
> However, if there are error reports, a `RuntimeError` will be raised
> after the operator finishes execution, including all error report messages,
> regardless of the return status (even if it was `{'FINISHED'}`).

Source: Python API — https://docs.blender.org/api/5.2/bpy.ops.html — read 2026-07-30. **Label: documented**, but note carefully what it is documenting: it is a general rule about operators, and it says nothing about whether a failed disk write produces an error report in the first place. Concluding from it that a failed write raises `RuntimeError` in a script would be an inference resting on an assumption the sources never supply.

**Not covered.** Everything the question actually asks. In particular: whether a failed write stops the render or lets it continue to the next frame; whether a part-written file is left behind; whether the failure is reported to the console, to the status bar, as an operator error report, or as a process exit code; whether the three cases behave alike.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say. The one failure that *is* described in interface terms — the red field and its tooltip — is by its nature something only a person looking at the window would see, and the sources do not say what a background run shows instead. The exit code that a background run could act on is documented only for Python exceptions: `--python-exit-code <code>`, "Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line), zero disables" (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30) — which is about scripts, not about render writes.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say. The single occurrence of the word "permission" anywhere in the API documentation is unrelated to rendering, and the manual's one heading on the subject, "Permissions on Windows" at https://docs.blender.org/manual/en/5.2/editors/preferences/file_paths.html, is about launching an external text editor under User Account Control, not about writing renders.

**Where I looked.** Searched the whole manual, the whole API documentation and the whole release notes, case-insensitively, for: "disk space", "disk full", "out of disk", "no space", "storage space", "quota", "errno", "ENOSPC", "device full", "insufficient", "run out", "ran out", "free space", "writable", "write access", "permission", "read-only", "cannot be written", "could not", "unable to", "failed to write", "unable to open", "fails with an error", "error message", "aborts", "abort the render", "skips the frame", "skip the frame", "continues rendering", "stops rendering". Every hit was in an unrelated subject — shadow buffers, Cycles time limits, cloth solvers, add-on repositories, texture-cache disk usage, GPU troubleshooting, the bug-fix list. Searched all of Cycles' files for `OSError`, `IOError`, `PermissionError`, `FileNotFoundError`, `open(` — all returned nothing; the only file-write error text Cycles carries at all is about OSL shader compilation, "Failed to write .oso file next to external .osl file at {:s}" in `cycles_5.2/osl.py` line 216, which is not a render write. Read in full: `render/output/properties/output.txt`, `render/output/animation.txt`, `compositing/types/output/file_output.txt`, `files/file_paths.txt`, `troubleshooting/crash.txt`, `troubleshooting/startup.txt`.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

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
00:00.744  image.write      | ERROR OpenImageIO write failed: Could not open file "…/ro/img_0001.png"
Error: Render error (Undefined error: 0) cannot save: '…/ro/img_0001.png'
raised: RuntimeError Error: Render error (Undefined error: 0) cannot save: '…/ro/img_0001.png'
python-route exit code: 1
```

and from the command line alone, with `-o` pointed into the same folder:

```
00:01.273  image.write      | ERROR OpenImageIO write failed: Could not open file "…/ro/cli_0001.png"
00:01.273  reports          | ERROR Render error (Undefined error: 0) cannot save: '…/ro/cli_0001.png'
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
00:00.540  image.openexr    | ERROR Unknown error in IMB_exr_write_channels
00:00.540  render           | Saved: '/Volumes/SMOOZYTINY/img_0001.exr'
00:00.574  image.openexr    | ERROR Unknown error in IMB_exr_begin_write
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

**Entry 23 — switching automatic saving off.** The switch is there in 5.2, and switching it off with no File Output node present
gives an error rather than a silent render:

```
has save_output: True
save_output set to False
Error: Render output disabled in Output properties and no active compositing File Output nodes
animation raised: RuntimeError Error: Render output disabled in Output properties and no active compositing File Output nodes
listing: no folder
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

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.2.0**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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

A box headed `Report: Error` also appeared inside the render window, carrying the same two lines. Nothing was written.

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
00:00.869  image.write      | ERROR OpenImageIO write failed: Could not open file "…/ro/img_0001.png"
Error: Render error (Success) cannot save: '…/ro/img_0001.png'
raised: RuntimeError Error: Render error (Success) cannot save: '…/ro/img_0001.png'
exit code: 1
```

and straight from the command line, with no Python at all:

```
00:01.471  image.write      | ERROR OpenImageIO write failed: Could not open file "…/ro/cli_0001.png"
00:01.471  reports          | ERROR Render error (Success) cannot save: '…/ro/cli_0001.png'
exit code: 1
```

**This is where the two machines differ, and it is in the words only.** On the
Creator's Mac the bracket read `(Undefined error: 0)`; on Linux the same empty reason
is printed as `(Success)`. Neither says why the write failed.

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

**Answer.** The sources name nine distinct ways. What each writes is stated for some and not for others.

**Label: documented**, for the list and for the writing behaviour where quoted; the gaps are marked.

**From the Render menu at the top of the window** — exact words:

> Render Image F12
>
> Render the active scene at the current frame.
>
> Render Animation Ctrl-F12
>
> Render the animation of the active scene.
>
> Render Sequencer Image Alt-F12
>
> If a Sequencer Scene exists that differs from the active scene, render that
> scene instead.
>
> Render Sequencer Animation Ctrl-Alt-F12
>
> Render the animation of the Sequencer Scene.
>
> Render Audio
>
> Mix the scene’s audio to a sound file.

Source: manual — https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html — read 2026-07-30.

What the first two write:

> By default, animation renders are saved automatically, while still renders are
> not. Still images must be saved manually from the Image Editor
> or written using a File Output Node.

and:

> When enabled, Blender behaves in the standard way: still renders (F12)
> are not saved automatically, while animation renders (Ctrl-F12) are
> written to the configured output path.

Source: manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

Render Audio writes a sound file, per the topbar wording above, and:

> Audio can be rendered from the Render Menu.

Source: manual — https://docs.blender.org/manual/en/5.2/render/output/audio/introduction.html — read 2026-07-30. That page lists the audio output's own options, including "Relative Path — Select the file path relative to the blend-file" and "Split Channels", so the audio write has its own path chosen at the time it is invoked. What the two Sequencer render entries write is not separately stated; the manual only notes that both "automatically set the Show Sequencer Scene toggle in the image editor Render Result after rendering".

**From the 3D Viewport's View menu** — three more:

> To render a still image, use 3D Viewport ‣ View ‣ Render Viewport Preview.

> To render an animation, use 3D Viewport ‣ View ‣ Render Playblast.

> To render an animation, but only those frames that have a keyframe, use
> 3D Viewport ‣ View ‣ Render Playblast on Keyframes. This only renders those frames for which
> the selected objects have an animation key. The other frames are still written to the output, but will simply
> repeat the last-rendered frame.

Source: manual — https://docs.blender.org/manual/en/5.2/editors/3dview/viewport_render.html — read 2026-07-30. That last sentence is the only direct statement in the sources about *what* a playblast writes: every frame in the range is written, and frames without a key repeat the previous rendered image. The same page says these use the scene's output settings:

> Additionally, some output settings are used too:
>
> Resolution
>
> Aspect
>
> Output path
>
> File format

and limits which engines they work with:

> Viewport rendering only works for the Workbench and EEVEE render engines.
> It’s not supported for Cycles.

Source: same page, same date. And these three need an output route to exist:

> Playblast operations also require an active output method. Both Render
> Playblast and Render Playblast on Keyframes will raise an error when render
> output is disabled in this panel.

Source: manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

**From the command line** — two:

> `-a, --render-anim`
>
> Render frames from start to end (inclusive).

> `-f, --render-frame <frame>`
>
> Render frame `<frame>` and save it.
>
> `+<frame>` start frame relative, `-<frame>` end frame relative.
>
> A comma separated list of frames can also be used (no spaces).
>
> A range of frames can be expressed using `..` separator between the first and last frames (inclusive).

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30. Note the difference in the words: `-f` says "Render frame `<frame>` **and save it**", where the in-window still render is documented as not saved automatically. **Label for that contrast: documented** — both halves are quoted from the sources, and I am reporting the difference in wording, not concluding anything beyond it.

**From Python** — two operators:

> `bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)`
>
> Undocumented, consider contributing.
>
> …
>
> `animation` (bool) – Animation, Render files from the animation range of this scene (optional)
>
> `write_still` (bool) – Write Image, Save the rendered image to the output path (used only when animation is disabled) (optional)
>
> …
>
> `frame_start` (int) – Start Frame, Frame to start rendering animation at. If not specified, the scene start frame will be assumed. This should only be specified if doing an animation render (in [-inf, inf], optional)
>
> `frame_end` (int) – End Frame, Frame to end rendering animation at. If not specified, the scene end frame will be assumed. This should only be specified if doing an animation render (in [-inf, inf], optional)

> `bpy.ops.render.opengl(*, animation=False, render_keyed_only=False, sequencer=False, write_still=False, view_context=True)`
>
> Take a snapshot of the active viewport
>
> …
>
> `render_keyed_only` (bool) – Render Keyframes Only, Render only those frames where selected objects have a key in their animation data. Only used when rendering animation (optional)
>
> `write_still` (bool) – Write Image, Save the rendered image to the output path (used only when animation is disabled) (optional)

Source: Python API — https://docs.blender.org/api/5.2/bpy.ops.render.html — read 2026-07-30. The `write_still` wording is the clearest statement in the sources of what a still render writes: nothing, unless that switch is turned on.

**A note on what governs all of the above.** In 5.2 the automatic writing can be switched off wholesale:

> Saving render output can now be disabled. (blender/blender@342f37566f)

Source: release notes — https://developer.blender.org/docs/release_notes/5.2/rendering/ — read 2026-07-30. In the API that is `save_output`, "Write frames to disk for animation renders (default True)" (https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, read 2026-07-30), listed as newly added in the 5.1-to-5.2 change log at https://docs.blender.org/api/5.2/change_log.html.

**Not covered.** What the two Sequencer render entries write, as distinct from the ordinary two. Whether "Render Viewport Preview" saves anything (the page does not say, and its wording parallels the still render, which does not). Whether the Video Sequencer's own render is a tenth route or the same Ctrl-F12 — the manual's animation page says "Click the Animation render button and Blender will render out the Sequence editor output into a movie", which reads as the same button. What exit code, if any, any of these produce.

**This question was ambiguous.** "What does each of them write" could mean which files land on disk, or which parts of the render result are produced. I answered the first reading, since the sheet's opening sentence is about "where the files a Blender render produces end up". Under the second reading the answer would be about view layers, passes and render slots, which the sources cover on different pages.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Of the nine, two are command-line arguments and two are Python operators, so four hold outside the window. The five menu entries are by definition inside it. The command-line page is explicit that `-b, --background` is the mode intended for this — "Run in background (often used for UI-less rendering)" — and its worked example combines `-b` with `-o` and `-a`. One documented difference: viewport rendering and playblasts depend on the current viewport — `bpy.ops.render.opengl` has `view_context`, "Use the current 3D view for rendering, else use scene settings", and the manual says "Activating Viewport Render will render from the current active view". The sources do not say what these do when there is no viewport.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say the routes differ by operating system. The keyboard shortcuts quoted are the manual's own, and the manual does not qualify them per system on this page. The command-line page marks its platform-specific arguments individually and marks none of `-a`, `-f`, `-b`, `-o`.

**Where I looked.** Read in full: `interface/window_system/topbar.txt` (Render Menu section), `editors/3dview/viewport_render.txt`, `render/output/animation.txt`, `render/output/properties/output.txt`, `render/output/introduction.txt`, `render/output/index.txt`, `render/output/audio/introduction.txt`, `advanced/command_line/arguments.txt`, `bpy.ops.render.txt`. Searched the manual for "playblast" (four hits, all quoted or cited above) and for "Render Image". Confirmed from the API that the complete list of `bpy.ops.render` operators is: `clear_texture_cache`, `color_management_white_balance_preset_add`, `cycles_integrator_preset_add`, `cycles_performance_preset_add`, `cycles_sampling_preset_add`, `cycles_viewport_sampling_preset_add`, `eevee_raytracing_preset_add`, `generate_texture_cache`, `opengl`, `play_rendered_anim`, `preset_add`, `render`, `shutter_curve_preset`, `swap_dimensions`, `view_cancel`, `view_show` — so there is no separate "render animation" operator; animation is an argument to `render`. Searched the API for "playblast" — no hits at all, so the playblast operations are named only in the manual.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 22 — what `-f`, `-a` and the Python render call each write.**

*What was done, to what.* One saved file holding both a scene output path and a
compositor File Output node was copied three times, and each copy rendered by one
route, each with its own `-o` folder so the routes could not see each other's work.

*The commands.*

```
"/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup …/r_f.blend  -o …/f_scene/s_ -F PNG -x 1 -f 1
"/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup …/r_a.blend  -o …/a_scene/s_ -F PNG -x 1 -a
"/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup …/r_py.blend -o …/py_scene/s_ -F PNG -x 1 --python q22.py
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
00:00.561  render           | Saved: '…/node_out/nd_Image.png'      (animation=False, write_still=False)
00:00.567  render           | Saved: '…/node_out/nd_Image.png'      (animation=False, write_still=True)
00:00.567  render           | Saved: '…/py_scene/s_.png'            (animation=False, write_still=True)
00:00.573  render           | Saved: '…/node_out/nd_Image0001.png'  (animation=True)
00:00.574  render           | Saved: '…/py_scene/s_0001.png'        (animation=True)
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

**Entry 23 — switching automatic saving off.** 5.2 has it, and switching it off with no File Output node present makes an animation render fail with `Error: Render output disabled in Output properties and no active compositing File Output nodes` — see question 5 above.

*Standing lines, brought up to date.* **Driven from outside** — the command-line and
Python routes are settled by the runs above. The viewport route is settled too, in
the other direction: it refuses to run with no window. What the menu entries do is
still open and sits with the Creator (entries 26 to 28 of the hand-test list).
**Operating system** — settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.2.0**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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
```

Only the first of the three tooltips was read from the screenshots for this version;
it matches the 4.5 and 5.0 wording. The other two screenshots are in the folder named
above and were not opened.

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

The same checkbox is visible, ticked, in the header of the Output panel in the
screenshots taken for entry 31 for this version.

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

**Answer.** It can be set up that way, in two different ways, and what is left behind is an image held in memory called the Render Result — which is temporary, and is lost unless saved by hand or unless a cache file was separately switched on.

**That it can be set up that way — label: documented.**

> The automatic saving of renders can be toggled using the checkbox in the panel header.

and:

> When disabled, Blender does not write render output from this panel and instead
> relies on other systems.

Source: manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. And separately, a still render simply is not saved:

> By default, animation renders are saved automatically, while still renders are
> not.

Source: same page, same date.

**What is left behind — label: documented.**

> Render Result images and Compositor Viewer node results are temporary and
> are not automatically saved to disk when saving the blend-file or closing Blender.
>
> These results must be manually saved if they should be preserved.
>
> Failing to save these images before rendering again, loading a new file,
> or closing Blender will cause the image data to be lost.

Source: manual — https://docs.blender.org/manual/en/5.2/files/media/image_formats.html — read 2026-07-30.

Said again in the preferences page:

> Render Result images and Compositor Viewer node results are temporary
> and will be lost when rendering again, loading a new blend-file,
> or closing Blender unless manually saved.

Source: manual — https://docs.blender.org/manual/en/5.2/editors/preferences/save_load.html — read 2026-07-30. That page also documents that saving the blend-file *can* be made to sweep them up, via a preference:

> Save Modified Images
>
> Controls how modified images are handled when saving the blend-file.
>
> This affects images edited within Blender, such as painted textures,
> generated images, Render Results, and Compositor Viewer node results
> that have unsaved changes.
>
> Ask Every Time:
>
> Displays a confirmation dialog when saving the blend-file, allowing
> modified images to be saved individually.
>
> Always Save:
>
> Automatically saves all modified images when saving the blend-file.
>
> Never Save:
>
> Never saves modified images automatically when saving the blend-file.

Source: same page, same date. (`save_modified_images` is listed as newly added to `PreferencesFilePaths` in the 5.1-to-5.2 change log, https://docs.blender.org/api/5.2/change_log.html, read 2026-07-30.)

**Where the leftover lives, and that there can be several.**

> Render Result: displays renders. When this item is selected, the Slot,
> View Layer, and Render Pass selectors become available (see below).

and:

> Slot
>
> The render slot to view (and render to). You can create new renders without
> losing previous ones by selecting an empty slot before rendering. Afterwards,
> you compare them by pressing J and Alt-J to cycle forwards and backwards.

Source: manual — https://docs.blender.org/manual/en/5.2/editors/image/introduction.html — read 2026-07-30.

**A file can still be left behind, if the cache is switched on.**

> Saving – Cache Result
>
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image.
> The Compositor can then use this file to improve performance, especially for heavy compositing.
>
> The image is stored in the Render Cache folder as specified in the
> File Paths Preferences.

Source: manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. And it survives a restart:

> Find the render cache file for the current scene and load it into the
> Render Result. This way, you can restore the last render from a previous
> Blender session and continue working in the Compositor without having to
> render the scene again.
>
> Note that Blender doesn’t create these cache files by default. You
> have to enable Cache Result
> in the scene’s Output options and then render it at least once.

Source: manual — https://docs.blender.org/manual/en/5.2/editors/image/editing.html — read 2026-07-30, under "Open Cached Render". So a render with automatic saving off is not guaranteed to write nothing — if Cache Result is on, an EXR is written to the Render Cache folder, which the preferences describe as "The location where cached render images are stored" (https://docs.blender.org/manual/en/5.2/editors/preferences/file_paths.html, read 2026-07-30).

**And in some configurations it is not allowed to write nothing at all.**

> For animation renders, if the
> Video Sequencer is active and no alternative output is
> available, an error is raised. Otherwise, if compositing is enabled, active
> File Output Node nodes are used to save the result.
> If no such nodes exist, an error is reported.

Source: manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

**Not covered.** Whether the Render Result exists at all after a *background* run finishes and the process exits — the sources say it is lost "when … closing Blender", which would cover process exit, but they do not address the background case in so many words. Also not covered: how much memory it holds, how many slots there are, and whether the render-time handlers (`render_complete`, `render_write`) can reach it.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* This is the weakest point of the whole answer. The Render Result is documented entirely in terms of the Image Editor, the render slots and manual saving — all of which need a window. The sources never say what a background run leaves behind when it writes nothing. The one mechanism that does not need a window is the render cache file, which is written to a folder and read back later. There is also a documented hook that fires at the right moment: "`bpy.app.handlers.render_write` — on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered" (https://docs.blender.org/api/5.2/bpy.app.handlers.html, read 2026-07-30), and `bpy.types.Image.save_render(filepath, *, scene=None, quality=0)`, "Save image to a specific path using a scenes render settings" (https://docs.blender.org/api/5.2/bpy.types.Image.html, same date). I am not claiming these combine into a working background workflow; that would be an inference the sources do not support.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say. Nothing on any of these pages names an operating system.

**Where I looked.** Searched the whole manual for "Render Result" (sixteen hits; the relevant ones are all quoted or cited above). Read in full: `files/media/image_formats.txt` (the warning block), `editors/preferences/save_load.txt`, `editors/image/introduction.txt`, `editors/image/editing.txt`, `editors/preferences/file_paths.txt`, `render/output/properties/output.txt`, `render/output/animation.txt`. Read the 5.2 rendering release note in full, which is where "Saving render output can now be disabled" appears. Checked the API for `save_output` and `use_render_cache`.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 20 — a windowless render told not to save.**

*What was done, to what.* At factory settings, with no window, the render cache and
placeholders switched off, the output path pointed at an empty folder, and
`bpy.ops.render.render(animation=False, write_still=False)` called. The image
data-blocks were listed before and after, and the temporary folder and the output
folder were listed afterwards.

*The command.*

```
"/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup --python q20.py -- …
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

**Entry 23 — switching automatic saving off.** 5.2 has one, and with it off and no File Output node present an animation render is refused outright rather than running silently — see question 5.

*Standing lines, brought up to date.* **Driven from outside** — settled. The Render
Result exists in a windowless Blender and can be reached from a script, and a render
told not to save leaves nothing at all on disk. What becomes of it after the process
ends was not tested and cannot be: the process is gone. **Operating system** —
settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.2.0**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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

The same checkbox is visible, ticked, in the header of the Output panel in the
screenshots taken for entry 31 for this version.

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
has save_output: True
save_output set to False
Error: Render output disabled in Output properties and no active compositing File Output nodes
animation raised: RuntimeError Error: Render output disabled in Output properties and no active compositing File Output nodes
listing: no folder
```

So the switch is there, it can be turned off from outside, and turning it off with no
File Output node present makes the render refuse rather than run silently.

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

**Answer.** Four things happen to the recorded string, in this order as the sources describe them: any `{…}` expressions are substituted; `//` is expanded to the blend-file's folder; the frame number is put in — at the position of the `#` marks, or appended at the end if there are none; and the extension is appended, decided by the chosen file format, if the File Extensions switch is on.

**Label: documented**, for each of the four. The order in which I list them is my arrangement of separately documented facts; the sources do not print an ordered list.

**Substitution happens at the moment the path is used.**

> This substitution happens internally at time of use (e.g. while rendering), so
> the template syntax will remain as-is in the path field.

Source: manual — https://docs.blender.org/manual/en/5.2/files/file_paths.html — read 2026-07-30. With the worked example on the same page:

> For example, if the currently open blend file is `dance.blend` and the render
> output path is set to this:
>
> `//my_render_dir/{blend_name}.png`
>
> Then when rendering, Blender will treat the path as:
>
> `//my_render_dir/dance.png`

**`//` becomes the blend-file's folder.**

> This setting expands Relative Paths
> where a `//` prefix represents the directory of the current blend-file.

Source: manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

**Where the frame number goes.**

> When rendering an animation,
> the frame number is appended at the end of the file name with four padded zeros (e.g. `image0001.png`).
> You can set a custom padding size by adding the appropriate number of `#` anywhere in the file name
> (e.g. `image_##_test.png` translates to `image_01_test.png`).

Source: same page, same date. The command-line page says the same thing and adds what happens when there are no `#` at all:

> The `#` characters are replaced by the frame number, and used to define zero padding.
>
> `animation_##_test.png` becomes `animation_01_test.png`
>
> `test-######.png` becomes `test-000001.png`
>
> **When the filename does not contain `#`, the suffix `####` is added to the filename.**
>
> The frame number will be added at the end of the filename, eg:
>
> `blender -b animation.blend -o //render_ -F PNG -x 1 -a`
>
> `//render_` becomes `//render_####`, writing frames as `//render_0001.png`

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30. (Emphasis mine; the words are exact.) So: the number appears wherever the `#` run appears, and only at the very end of the base name if there is no `#` run. Note that the two pages agree on four digits as the default and on `#` overriding both position and width.

Read literally, `image_##_test.png` becoming `image_01_test.png` shows the run of `#` replaced in place, in the middle of the name, before the extension.

**What decides the end of the name.** The chosen file format supplies it, and a switch decides whether it is added at all.

> Saving – File Extensions
>
> Adds the correct file extensions per file type to the output files.

Source: manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

> `use_file_extension`
>
> Add the file format extensions to the rendered file name (eg: filename + .jpg) (default True)

and the extension itself is a value Blender computes and exposes read-only:

> `file_extension`
>
> The file extension used for saving renders (default “”, readonly, never None)

Source: Python API — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30. Which format is chosen:

> `file_format`
>
> File format to save the rendered images as (default 'PNG')

Source: https://docs.blender.org/api/5.2/bpy.types.ImageFormatSettings.html — read 2026-07-30. From the command line the same switch is `-x, --use-extension <bool>`, "Set option to add the file extension to the end of the file" (command-line page, same date).

**The whole thing can be asked for directly.**

> `frame_path(*, frame=-2147483648, preview=False, view='')`
>
> Return the absolute path to the filename to be written for a given frame
>
> Parameters:
>
> `frame` (int) – Frame number to use, if unset the current frame will be used (in [-inf, inf], optional)
>
> `preview` (bool) – Preview, Use preview range (optional)
>
> `view` (str) – View, The name of the view to use to replace the “%” chars (optional, never None)
>
> Returns:
>
> File Path, The resulting filepath from the scenes render settings (never None)

Source: Python API — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30. This is the sources' own answer to the question: one call turns the recorded place plus a frame number into the actual absolute filename.

Cycles' own files use it exactly that way, which is an **observed** finding — these are Cycles' files, not documentation:

```
            for frame in range(scene.frame_start, scene.frame_end + 1):
                scene.render.filepath = in_filepath
                filepath = scene.render.frame_path(frame=frame)
                in_filepaths.append(filepath)
```

Source: `Blender Sources/cycles_5.2/operators.py`, lines 79–82, inside `CYCLES_OT_denoise_animation.execute`. *(Citation check, 2026-08-05: line range corrected from 78–81.)* The comment above it reads:

```
            # Denoise animation sequence with expanded frames matching
            # Blender render output file naming.
```

Source: same file, lines 70–71. *(Citation check, 2026-08-05: corrected from 72–73.)* **Label: observed** — I am reporting what the file says, that Cycles sets `scene.render.filepath` and then calls `frame_path(frame=…)` once per frame to get each frame's name. What Blender then produces on disk from that call is a step beyond the file, and I have not taken it.

**A fifth thing, in multi-view renders.** Each view adds its own ending:

> `file_suffix`
>
> Suffix added to the render images for this view (default “”, never None)

> `camera_suffix`
>
> Suffix to identify the cameras to use, and added to the render images for this view (default “”, never None)

Source: Python API — https://docs.blender.org/api/5.2/bpy.types.SceneRenderView.html — read 2026-07-30. And the manual:

> By default the system saves each view as an individual file,
> thus generating as many files as views to be rendered.

Source: manual — https://docs.blender.org/manual/en/5.2/render/output/properties/stereoscopy/usage.html — read 2026-07-30.

**The File Output node builds its name differently**, and its own formula is printed:

> Each connected input is saved to the specified directory and filename,
> with the frame number appended to create a sequence.
> For example: `{Directory}/{File Name}{Socket Name}{frame number}.{extension}`.

Source: manual — https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30. So there the socket name sits between the base name and the frame number. The node also shows the result before you render:

> Output Paths
>
> …
>
> Displays the final file path and extension for each input/layer, based on current settings.
> This helps verify that outputs are named and organized correctly.

Source: same page, same date.

**Not covered.** The order I gave is assembled from separate pages; the sources never state which of substitution, `//` expansion and frame-number insertion happens first, and a path where one could affect another — a template variable whose value contains a `#`, say — is not addressed anywhere. Also not covered: what happens when the recorded string already ends in the extension the format would add (the sources describe `bpy.path.ensure_ext` as a utility that avoids doubling, but never say the render path uses it). Also not covered: whether the four-digit default can produce a name for a frame number needing more than four digits, and what it does then.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Everything here holds outside, and the command-line page is the source for the clearest part of it — the `####` fallback and the `blender -b … -o //render_ -F PNG -x 1 -a` example are printed on the command-line page itself. `frame_path()` is a Python call and needs no window. The one part that is interface-only is the node's "Output Paths" panel, which is a preview shown in a sidebar.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say the naming differs by operating system. Two related things are documented that a reader should not confuse with it: `bpy.path.native_pathsep` — "Replace the path separator with the system’s native `os.sep`" — and `bpy.path.resolve_ncase` — "Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path" (both https://docs.blender.org/api/5.2/bpy.path.html, read 2026-07-30). Those show Blender has tools for per-system path differences; neither says the render output name is put through them.

**Where I looked.** Read in full: `render/output/properties/output.txt`, `advanced/command_line/arguments.txt`, `files/file_paths.txt`, `compositing/types/output/file_output.txt`, `render/output/properties/stereoscopy/usage.txt`, `bpy.types.RenderSettings.txt`, `bpy.types.ImageFormatSettings.txt`, `bpy.types.SceneRenderView.txt`, `bpy.path.txt`, `cycles_5.2/operators.py`. Searched the manual for "padding", "padded", "zeros", "extension", "frame number", "suffix". Searched the API for `frame_path`, `file_extension`, `use_file_extension`, `ensure_ext`.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

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

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.2.0**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 30 — what a File Output node shows on its face.**

*What was done, to what.* A file holding a File Output node wired to a Render Layers
node was opened with the matching Blender, the Compositing workspace shown, and the
node looked at; then the node selected and the sidebar opened. Two screenshots per
version, kept at `/Users/artist/Desktop/Screenshots/010/`, one folder per version.

*One thing to record plainly.* The file used was not the one this entry named. It was
a file prepared for another topic — its node's own Output Paths panel points into that
topic's folder — and it carries three inputs in 4.5 and two in the other three rather
than the one the entry described. What is written below is what the screenshots show.

*What is on the node's face.* Two stacked text fields with no labels, each with its
own folder-browse button, the upper holding `//out/` and the lower holding `shot`,
then one row per input and a `+` for adding another:

```
File Output
  [ //out/ ]  📁
  [ shot   ]  📁
  ○ passes/diffuse
  ○ beauty
  ○                        (empty)
  +
```

The input named `passes/diffuse` keeps its slash here.

*What is in the sidebar.* A **Node** panel holding Name (`File Output`), Label
(placeholder text `Custom Label`), Color, `[x] Show Options`, `[ ] Mute` and
`Propagate  All Messages`. Then a **Properties** panel holding, in this order:

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
  ● passes/diffuse
  ● beauty                                  (selected)
Socket Type         Color
Output Paths
  [x] File Extensions
  /Users/artist/Documents/Smoozy hand tests/Reading a file/13/out/shot.exr
  +
Custom Properties
  [ + New ]
```

So the node records two path fields plus a name per input, and it carries a panel
called **Output Paths** holding a **File Extensions** checkbox of its own and the
finished name shown before anything is rendered.

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

**This question is ambiguous and I flag it before answering.** "By writing it into the file from outside Blender" can be read two ways. Reading A: editing the `.blend` file's stored contents with some tool that is not Blender. Reading B: having something outside Blender cause the value to be put into the loaded file — that is, driving Blender from a script or a command line so that the node's path is set. I answer **Reading B**, because the sheet's other questions are about how renders are started and controlled from outside, and because Reading A concerns the `.blend` file format, which the sources never describe. What Reading A would need is stated at the end.

**At the moment the render is started — partly documented, partly inferred, partly absent.**

**There is no command-line argument for it. Label: not in the sources.** `-o, --render-output <path>` is documented as "Set the render path and file name" and says nothing about the compositor. I read the whole command-line page and there is no argument naming the File Output node, the compositor, or a node's directory.

**Python at launch can do it. Label: inferred.** The command line runs a script or an expression —

> `-P, --python <filepath>`
>
> Run the given Python script file.

> `--python-expr <expression>`
>
> Run the given expression as a Python script.

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30 — and the node's two path parts are ordinary writable string properties:

> `directory`
>
> The directory where the image will be written (default “”, never None, blend relative // prefix supported, Supports template expressions)

> `file_name`
>
> The base name of the file. Other information might be included in the final file name depending on the node options (default “”, never None, Supports template expressions)

Source: Python API — https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30. Neither is marked readonly, unlike `format` and `file_output_items` on the same page, which are. **The step I took:** the sources say Blender runs Python given on the command line, and separately that these are writable properties; joining them gives "a launch-time script can set the node's directory and file name". The sources nowhere state that combination, and nowhere show an example of setting a File Output node's path from a script.

**There is a documented way to make the node's path resolve differently per render without touching the field at all.** Path templates are explicitly supported on it:

> Currently path templates are only supported for the render output path in
> Scene properties and the output paths in the compositor’s File Output
> node. More filepaths will support templates in future releases.

Source: manual — https://docs.blender.org/manual/en/5.2/files/file_paths.html — read 2026-07-30. **Label: documented.** The variables available include ones that vary with what is being rendered — `scene_name`, "Name of the current scene"; `camera_name`, "Name of the current render camera"; `resolution_x` / `resolution_y`; `fps` — and one available only on nodes, `node_name`, "The name of the node that the path property is on" (all quoted in question 3, same page and date). And substitution "happens internally at time of use (e.g. while rendering)". So the recorded value is fixed while the resolved place changes with the render. That this works on the File Output node in practice is corroborated by a release note:

> Compositor - frame slip bug using \{camera_name}\ Vairable in File Output Node when switching cameras. [[cd28c908f0a](https://projects.blender.org/blender/blender/commit/cd28c908f0a1466d1885d985345ee95d501cab12)]

Source: release notes — https://developer.blender.org/docs/release_notes/5.2/bugfixes/ — read 2026-07-30. (Quoting a release note is reporting a documented change, per the rules of this work. The typo is in the source.)

**Whether the node is used at all can also be settled per render.** With automatic saving off, the node becomes the writer:

> Otherwise, if compositing is enabled, active
> File Output Node nodes are used to save the result.
> If no such nodes exist, an error is reported.

Source: manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. The switch behind that is `save_output`, "Write frames to disk for animation renders (default True)" (https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, read 2026-07-30), new in 5.2 per https://developer.blender.org/docs/release_notes/5.2/rendering/ — "Saving render output can now be disabled."

**Writing it into the file from outside Blender (Reading A) — label: not in the sources.** Nothing in the manual, the API documentation, the release notes or Cycles' files describes editing a `.blend` file's stored contents with anything other than Blender, and nothing describes the on-disk layout that would be needed to do so.

**The nearest thing the sources offer, and why it is not the same.** There is a documented mechanism for walking and rewriting the file paths a blend-file holds:

> `file_path_foreach(visit_path_fn, *, subset=None, visit_types=None, flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES'})`
>
> Call `visit_path_fn` for the file paths used by all ID data-blocks in current `bpy.data`.
>
> …
>
> `visit_path_fn` (…) – function that takes three parameters: the data-block, a file path, and a `bpy.types.BlendDataPathMeta` metadata object. The function should return either `None` or a `str`. In the latter case, the visited file path will be replaced with the returned string.

Source: Python API — https://docs.blender.org/api/5.2/bpy.types.BlendData.html — read 2026-07-30. And 5.2 extended it with render farms named as the reason:

> ## Path Iterator
>
> Three new options for `bpy.data.file_path_foreach` so render farms and other scripts can find all relevant image files for rendering, including the new Cycles texture cache. (blender/blender!159159)
>
> - `EXPAND_TOKENS`: visit each UDIM tile path.
> - `EXPAND_SEQUENCES`: visit each image and volume sequence file path.
> - `EXPAND_CACHES`: visit cache files, like texture cache files associated with images.

Source: release notes — https://developer.blender.org/docs/release_notes/5.2/python_api/ — read 2026-07-30. *(Citation check, 2026-08-05: the third bullet was missing. The quote said "Three new options" and showed two; `EXPAND_CACHES` has been restored from the source.)* But this runs inside Blender, on `bpy.data`, so it is Reading B and not Reading A. And crucially the sources **do not say whether a File Output node's `directory` and `file_name` are among the paths it visits** — it is described as visiting the paths of "ID data-blocks", and the two new options are described as being about UDIM tiles and image sequences. Concluding that it reaches the node's output paths would be an inference the sources do not support, so I do not draw it.

**Not covered.** Whether any command-line argument added by an add-on could do it (only Cycles' two, `--cycles-device` and `--cycles-print-stats`, are in scope here, and neither concerns output paths — confirmed in `Blender Sources/cycles_5.2/engine.py`, lines 8–22, where `_configure_argument_parser` registers exactly those two). Whether the node's `directory` and `file_name` are visited by `file_path_foreach`. Whether setting the node's path from a launch script and then rendering in the same run works — no source shows it. Whether the `.blend` file must be saved afterwards for the change to hold, and what that would mean for a farm.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The whole question is about outside the interface, and the answer is uneven: the templates route is documented and needs no window; the Python route is inferred; the direct command-line route does not exist. One point in favour of the templates route for outside use is that it needs nothing to be set at launch at all — the recorded value stays as typed and resolves per render. One point against relying on it is the failure mode from question 5: "if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors", and the way that error is shown in the sources is a red field and a hover tooltip, which nobody sees in a background run.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say. No page on this subject names an operating system.

**Where I looked.** Read in full: `advanced/command_line/arguments.txt` (every argument, looking for anything naming the compositor or a node — none), `compositing/types/output/file_output.txt`, `files/file_paths.txt`, `bpy.types.CompositorNodeOutputFile.txt`, `bpy.types.NodeCompositorFileOutputItem.txt`, `bpy.types.NodeCompositorFileOutputItems.txt`, `bpy.types.BlendData.txt`, `bpy_types_enum_items/file_path_foreach_flag_items.txt`, `bpy.types.BlendDataPathMeta.txt`, `release_notes_5.2/python_api.md`, `release_notes_5.2/compositor.md`, `release_notes_5.2/rendering.md`, `cycles_5.2/engine.py`, `cycles_5.2/maketx.py`. Searched the release notes for "template" (three hits, all about `UILayout.template_palette` and `template_popup_confirm`, none about path templates — so path templates are not a 5.2 change and the manual is the only source that describes them for this version). Searched the whole API for `base_path` and `file_slots`, the older names for these fields — no hits, confirming 5.2 documents `directory` and `file_name`.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 16 — does `-o` reach a File Output node?**

*What was done, to what.* A file was saved holding a scene output path pointing at
one folder and a File Output node named `FileOut` pointing at another. Blender was
started with no window, given that file, then `-o` pointing at a third folder, then
`-f 1`. Afterwards the file was reopened and the node's stored values printed.

*The command.*

```
"/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup …/node.blend -o …/cli_out/cli_ -F PNG -x 1 -f 1
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
"/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup …/node.blend --python-expr "import bpy; sc=bpy.context.scene; t=sc.compositing_node_group; n=t.nodes['FileOut']; n.directory='…/from_cli/'; n.file_name='zz_'; print('set to', n.directory, n.file_name)" -f 1
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
  VISIT ("bpy.data.images['cli_0001.png']", "'…/cli_out/cli_0001.png'", '<BlendDataPathMeta object at 0x123992550>')
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

2026-08-04 — Opus 5 (Claude Code) — Writing, Blender 5.2
