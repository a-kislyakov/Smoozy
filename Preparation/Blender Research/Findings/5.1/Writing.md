# Writing — Blender 5.1

**Sources consulted, and when.** The Blender 5.1 manual, the Blender 5.1 Python API documentation, the Blender 5.1 release notes, and the Cycles 5.1 files, all as supplied in `Blender Sources/` and all taken on **2026-07-30** (my date read for every citation below). I did no web access.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

One precise disclosure, so the line above is exactly true: to confirm the folder I was told to save into existed, I ran a directory listing of `Preparation/Blender Research/Findings/5.1/` and saw the names of the files already there. I opened none of them, and I read no content from that folder or anywhere else outside `Blender Sources/`.

---

## 1. Can where a render writes be settled at the moment the render is started, without changing the file?

**Answer.** Yes. Blender's command line takes an argument that sets the render path and file name for that run, and the manual's own worked examples use it exactly this way. It also gives an ordering rule that must be obeyed: the path must be set *after* the blend-file is loaded and *before* the render is asked for.

Quoted, on the argument itself:

> "-o, --render-output <path>
> Set the render path and file name.
> Use // at the start of the path to render relative to the blend-file."

Quoted, on ordering (this is the part that decides whether it works at all):

> "blender --background test.blend --render-frame 1 --render-output "/tmp"
> …will not render to /tmp because --render-frame 1 renders before the output path is set.
> blender --background --render-output /tmp test.blend --render-frame 1
> …will not render to /tmp because loading the blend-file overwrites the render output that was set.
> blender --background test.blend --render-output /tmp --render-frame 1
> …works as expected."

And in the rendering-from-the-command-line page:

> "Always position -f or -a as the last arguments."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html and https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html
Date read: 2026-07-30

There is a second route at start time, also from outside the file: Blender runs a Python script or expression given on the command line, and the render output path is a writable property.

> "-P, --python <filepath>
> Run the given Python script file."

> "--python-expr <expression>
> Run the given expression as a Python script."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html

> "filepath
> Directory/name to save animations, # characters define the position and padding of frame numbers (default "", never None, blend relative // prefix supported, Supports template expressions)
> Type:
> str"

Source: API — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
Date read: 2026-07-30

**Label: documented** for setting the path at launch with `-o`. **Inferred** for setting it with a launch-time Python script: the sources separately show that a script can run at launch and that the path is a writable property, but do not join those facts. **Inferred** for the "without changing the file" half: the sources never say in so many words that `-o` leaves the blend-file untouched. The step I took is that the manual describes loading the blend-file as something that *overwrites* the value set by `-o`, which places the value in the loaded scene data rather than on disk, and saving a blend-file is a separate operator (`bpy.ops.wm.save_mainfile`, "Save the current Blender file", https://docs.blender.org/api/5.1/bpy.ops.wm.html) that nothing in the render arguments invokes. A reader has to supply that step, so it is inferred, not documented.

**Not covered.** The sources do not say what `-o` does with a path that ends in a separator versus one that does not, nor whether `-o` has any effect on paths other than the scene render output path (see Q9). They do not state an exit status or error text for a bad path given to `-o`.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* This is *only* about outside the interface — `-o` is a command-line argument, and the manual's whole page on it is about running Blender with no graphical display: "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)." (https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html). **Documented** for `-o` working outside the interface. **Inferred** for the comparison with the interface: the Output panel records the same scene property, and the separate save operator saves the file, but no quoted sentence joins those facts.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The manual gives launch instructions for Linux, macOS and Windows and does not restrict `-o` to any of them; arguments that *are* platform-specific are marked so in the same list (for example "-con, --start-console … (Windows only)" and "--no-window-frame Disable all window decorations (Linux only)"). That marking convention is documented; reading its absence on `-o` as "all platforms" is **inferred**. One documented caveat applies everywhere: "Arguments are case sensitive! -F and -f are not the same." (https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html)

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

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
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup …/test.blend -o …/elsewhere2/e_ -F PNG -x 1 -f 1
```

*What came back.*

```
--- before ---
907e0b544ddd91cf6e4517d76b5e228d9b791624709b937fb0fdec782821a23e  …/test.blend
907e0b544ddd91cf6e4517d76b5e228d9b791624709b937fb0fdec782821a23e  …/keep.blend
…/test.blend 95505 1785882847
Saved: '…/elsewhere2/e_0001.png'
render exit code: 0
--- after ---
907e0b544ddd91cf6e4517d76b5e228d9b791624709b937fb0fdec782821a23e  …/test.blend
907e0b544ddd91cf6e4517d76b5e228d9b791624709b937fb0fdec782821a23e  …/keep.blend
…/test.blend 95505 1785882847
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

Everything in the Linux sections of this document was run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. It is a rented container sharing somebody else's hardware, so it is given fewer cores than the chip really has and its system settings are not entirely its own; other work may have been running on it at the same time, so nothing here says how long anything took. Blender was started with no window unless an entry says otherwise, with its settings folder pointed at one of this topic's own and with the display variable unset so no window could be reached. The build used for this version was **Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)**. Date: 2026-08-06. In the output below, `…` stands for the working folder on that machine, which was `/root/smoozy/Writing/work/<entry>/5.1`. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests — Linux.md`.

**One note on how the output is shown.** From 5.0 every line Blender prints while rendering carries a timestamp and a channel name in front of it, like `00:00.617  render           | Saved: '…'`. Where a quoted line below carries no such prefix, the prefix has been dropped because the timestamp differs from one run to the next; nothing after the bar is changed.

**These results settle Linux and say nothing about macOS.** Where the same test was also run on the Creator's Mac, both results are given and neither is folded into the other.

**Entry 17 — does the output argument change the blend-file on disk?**

*What was done, to what.* A blend-file was saved with its output path recorded as
`…/stored/stored_`, a byte-for-byte copy kept beside it, and the file then rendered
with the output argument pointing at `…/elsewhere/e_`.

*The command.*

```
/opt/blender/blender-5.1.2-linux-x64/blender -b --factory-startup …/test.blend -o …/elsewhere/e_ -F PNG -x 1 -f 1
```

*What came back.* The two checksums and the size and modified time were identical
before and after:

```
--- checksums before
506da9847fe377eea6388fc0444eaae4327dba05e1a597225cd3e6be8eb84d83  …/test.blend
506da9847fe377eea6388fc0444eaae4327dba05e1a597225cd3e6be8eb84d83  …/keep.blend
…/test.blend 95401 1785990952
--- the render, with the output argument pointing somewhere else
Saved: '…/elsewhere/e_0001.png'
exit code: 0
--- checksums after
506da9847fe377eea6388fc0444eaae4327dba05e1a597225cd3e6be8eb84d83  …/test.blend
506da9847fe377eea6388fc0444eaae4327dba05e1a597225cd3e6be8eb84d83  …/keep.blend
…/test.blend 95401 1785990952
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

**Answer.** There are two different recorded things, with different shapes.

**(a) The scene's render output path.** One single text field that holds directory and name together:

> "filepath
> Directory/name to save animations, # characters define the position and padding of frame numbers (default "", never None, blend relative // prefix supported, Supports template expressions)"

Source: API — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html

The manual calls the same field "File Path" and says what surrounds it:

> "File Path
> Choose the location to save rendered frames."

> "Saving – File Extensions
> Adds the correct file extensions per file type to the output files."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html

So the recorded thing for a scene render is: one path string (directory + base name, possibly with `#` marks and `{…}` template expressions, possibly with a `//` prefix), plus separate settings that finish the name — a File Extensions switch, a Media Type, and a File Format. The extension itself is derived, not typed:

> "file_extension
> The file extension used for saving renders (default "", readonly, never None)"

Source: API — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html

The Output panel screenshot in the manual shows the field holding `/tmp/`, with Overwrite and File Extensions ticked, Placeholders and Cache Result unticked, and File Format PNG. (Image `_images/render_output_properties_output_panel.png`, on https://docs.blender.org/manual/en/5.1/render/output/properties/output.html. Label for what the picture shows: **documented**, for what is visible in it only.)

**(b) The File Output node's path.** Two separate text fields, plus a per-input name:

> "directory
> The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)"

> "file_name
> The base name of the file. Other information might be included in the final file name depending on the node options (default "", never None, Supports template expressions)"

Source: API — https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html

> "Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence. For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

Source: manual — https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html

The node picture in the manual shows exactly two stacked text fields on the node body, the upper holding `/tmp/` and the lower holding `file_name`. (Image `_images/node-types_CompositorNodeOutputFile.webp`, same page.)

Each input carries its own name and, optionally, its own format:

> "name
> (default "", never None)"
> "override_node_format
> Use a different format instead of the node format for this file (default False)"

Source: API — https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html

**(c) How long the recorded text may be.** The manual publishes the fixed string limits:

> "directory:
> 767
> file-name:
> 255
> file-path:
> 1023"
> "Note
> Multi-byte encoding means some Unicode characters use more than a single ASCII character."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/limits.html
Date read: 2026-07-30

**Label: documented** for the recorded fields, the per-input name and format override, and the published text limits. **Inferred** for the summary that the scene's separate File Extensions, Media Type and File Format settings finish the name: the quoted words document the extension switch and the derived extension, but do not state that whole relationship in one place.

**Not covered.** How any of this is laid out inside the `.blend` file on disk (byte layout, block names) is not described anywhere in these sources. Whether the limits above are counted in bytes or in characters is only hinted at by the multi-byte note, not stated.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The same properties are the ones the Python API exposes, and the API documentation is the same documentation whether Blender has a window or not; nothing on these pages restricts them to interactive use. Label: **inferred** (the sources do not say it outright; I am reading the absence of any UI-only marking). Caveat: the "path field will be highlighted red in the UI" behaviour described under path templates (Q3) is by its nature interface-only.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say the recorded shape differs by operating system. The one path-shaped thing they do mark as platform-dependent is the separator, via a helper that exists to fix it: "bpy.path.native_pathsep(path) — Replace the path separator with the system's native os.sep." (https://docs.blender.org/api/5.1/bpy.path.html). "The sources do not say" for anything beyond that.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 1 — what a fresh Blender records, and which switches sit beside it.**

*What was done, to what.* Blender was started with no window and factory settings,
and a short script printed the render output path, the built-in default behind it,
each of the output switches, and the preference that seeds a new scene.

*The command.*

```
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup --python q1.py
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
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup --python q2.py
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

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.1.2**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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

*What is on the node's face.* Two stacked text fields with no labels, each with its
own folder-browse button, the upper holding `//out/` and the lower holding `shot`,
then one row per input:

```
File Output
  [ //out/ ]  📁
  [ shot   ]  📁
  ○ passes/diffuse
  ○ beauty
  ○                        (empty)
```

The input named `passes/diffuse` keeps its slash here.

*What is in the sidebar.* A **Node** panel holding Name (`File Output`), Label
(placeholder text `Custom Label`), Color, `[x] Show Options` and `[ ] Mute`. Then a
**Properties** panel holding, in this order:

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
  /Users/artist/Documents/Sm…sts/Reading a file/13/out/shot.exr
Custom Properties
  [ + New ]
```

(The path in the Output Paths panel is shown shortened in the middle by Blender
itself.) So the node records two path fields plus a name per input, and it carries a
panel called **Output Paths** that shows the finished name before anything is
rendered.

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
VERSION 5.1.2
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

**Answer.** Seven kinds, all documented.

**`//` at the start — the blend-file's own folder.**

> "This setting expands Relative Paths where a // prefix represents the directory of the current blend-file."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html

> "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."
> "Note
> You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."

Source: manual — https://docs.blender.org/manual/en/5.1/files/blend/open_save.html

**`#` — where the frame number goes, and how many digits.**

> "The # characters are replaced by the frame number, and used to define zero padding.
> animation_##_test.png becomes animation_01_test.png
> test-######.png becomes test-000001.png
> When the filename does not contain #, the suffix #### is added to the filename."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html

> "You can set a custom padding size by adding the appropriate number of # anywhere in the file name (e.g. image_##_test.png translates to image_01_test.png)."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html

**`{name}` — a template expression, replaced when the path is used.**

> "Path templates substitute template expressions (written {abcd}, where "abcd" is a variable name) in a filepath when the path is used."
> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

The named variables, quoted as the manual lists them:

> "blend_name: The currently open blend file's name (without the .blend).
> blend_dir: Path up to (but not including) the currently open blend file.
> blend_name_lib: Like blend_name, except that for path properties on library-linked data-blocks it will be the name of the library blend file that the data-block comes from.
> blend_dir_lib: Like blend_dir, except that for path properties on library-linked data-blocks it will be the path to the library blend file that the data-block comes from."

> "Available only in render output paths (including in the compositor File Output node):
> fps: The frames per second of the current scene.
> resolution_x / resolution_y: The x and y resolution of the rendered image. This factors in the resolution scale as well…
> scene_name: Name of the current scene.
> camera_name: Name of the current render camera."

> "Available only in path properties on a node:
> node_name: The name of the node that the path property is on."

And where they work at all:

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."

**`:` inside a template expression — a format specifier follows.**

> "Format specifiers … are written after a separating colon, like this: dance_{fps:FORMAT}.png"
> "dance_{fps:###}.png: format as an integer with at least 3 digits.
> dance_{fps:.###}.png: format as a floating point number with exactly 3 digits after the decimal point.
> dance_{fps:###.##}.png: format as a floating point number with at least 3 digits for the integer part and exactly 2 digits for the fractional part."
> "For example, if the fps is 29.97, then:
> dance_{fps:###}.png -> dance_030.png
> dance_{fps:.###}.png -> dance_29.970.png
> dance_{fps:###.##}.png -> -> dance_029.97.png"

Note that inside a template expression `#` means digit count, not frame number.

**`{{` and `}}` — a literal brace.**

> "{{ translates to a single { in the final path.
> }} translates to a single } in the final path."
> "my_weird}}_{{path.png -> my_weird}_{path.png
> //my_render_{{dir}}/{blend_name}.png -> //my_render_{dir}/dance.png
> //my_render_dir/{{{blend_name}}}.png -> //my_render_dir/{dance}.png"

Source for all the template quotes: manual — https://docs.blender.org/manual/en/5.1/files/file_paths.html
Date read: 2026-07-30

**`%` — the view name, in multi-view/stereo renders.** This appears only in the API, on the call that turns the recorded path into a real one:

> "view (str) – View, The name of the view to use to replace the "%" chars (optional, never None)"

Source: API — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html (`frame_path`)

**`/` or `\` inside a File Output node's input name — a folder split.** This is a mark inside the *name*, not the path field, but it stands for something else in the same way:

> "If an input name contains a slash character (/ or \), the path is interpreted as a directory hierarchy."
> "Any missing directories are created automatically during output."
> "This allows organizing outputs into folders directly from the node setup, for example: passes/diffuse or masks/object_01."

Source: manual — https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html

The release notes state this plainly as a change:

> "The **File Output** node now allows specifying directories in item names. This was possible in older versions of Blender and the functionality is now restored."

Source: release notes — https://developer.blender.org/docs/release_notes/5.1/compositor/
Date read: 2026-07-30

**What a broken mark does.**

> "Paths that support templates can have template errors, which prevent the path from being processed."
> "When there are template errors in a path, the path field will be highlighted red in the UI"
> "Note
> Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

Source: manual — https://docs.blender.org/manual/en/5.1/files/file_paths.html

I opened both pictures on that page. `_images/files_file_paths-invalid_path_template.png` shows the Output panel's path field drawn with a red background, holding `//my_render_dir/{blend_name.png`. `_images/files_file_paths-invalid_path_template_tooltip.png` shows the tooltip over that field, reading: "Output Path — Directory/name to save animations, # characters define the position and padding of frame numbers. Value: //my_render_dir/{blend_name.png — Syntax error(s): - Invalid or incomplete template expression '{blend_name.png'." Label for what those pictures show: **documented**.

**Label: documented** throughout, with one exception. For `%`, what is documented is only that `frame_path`'s `view` argument replaces "%" chars with a view name; that a recorded render output path may carry `%` as a mark at all is **inferred** from that, and the sources do not say it. *(Citation check, 2026-08-05: label narrowed for `%`; it had been covered by the blanket "documented".)*

**Not covered.** Whether `..` walks up a folder, whether `~` means a home folder, whether environment variables like `$HOME` or `%TEMP%` are expanded inside an output path — none of this is stated anywhere I looked. I searched the manual and API for "environment variable", "expand", "wildcard", "$HOME" and "~", and the only hits were unrelated (a debug argument's shader-name wildcard, and the `BLENDER_*` variables that set resource directories, not output paths). Also not covered: what `%` does in a path when the scene is not multi-view, and whether `#` inside a `{…}` expression and `#` outside one can be mixed in one path.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The documented `-o` help repeats `//` and `#` for the command line specifically, so those two demonstrably work with no window. Template expressions are also documented for the command line: "You can use path templating features such as {blend_name} in the path." (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html) — **documented**. The sources do not state whether `%` replacement or folder splitting in a File Output node input name behaves the same outside the interface. The red-field and tooltip reporting of template errors is interface-only by its own wording ("highlighted red in the UI"); the sources do say the failure itself still happens outside it ("rendering an animation will fail with an error message indicating the path errors"), but do not say where that message goes when there is no window.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say any of these marks differ by operating system. The File Output node's folder-splitting explicitly accepts both slash shapes ("a slash character (/ or \)"), which is documented. The quote does not say that this holds on every operating system.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

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

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.1.2**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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

**Answer.** Split. For the **File Output node**, yes, and it is stated outright:

> "Any missing directories are created automatically during output."

Source: manual — https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
Date read: 2026-07-30
**Label: documented.** Note the sentence sits under the node's Inputs section, in the passage about input names containing slashes; read strictly it is about the folders implied by input names.

For the **scene's render output path**, the sources are silent. **Label: not in the sources.**

**Where I looked, for that silence.** I searched the whole manual, the whole API documentation and the whole 5.1 release notes for: "missing director", "create.*director", "director.*created", "directory does not exist", "folder does not exist", "non-existing director", "mkdir", "makedirs", "folder is created", "creates the folder", "creates the directory", "will be created", "are created", "is created automatically", "automatically created", "parent directory", "nonexistent", "non-existent" and "does not exist". The only folder-creation statements anywhere in the manual are the File Output node line above, the Ocean modifier's cache — "If the cache path folder does not exist, it will be created." (https://docs.blender.org/manual/en/5.1/modeling/modifiers/physics/ocean.html) — and the Store Bundle Item node, which is about data paths not disk paths. I also read the whole Output panel page (https://docs.blender.org/manual/en/5.1/render/output/properties/output.html), the whole Rendering Animations page (https://docs.blender.org/manual/en/5.1/render/output/animation.html), the whole command-line arguments page and the whole command-line render page, and the `RenderSettings` API page in full. None of them mentions folder creation for the render output path. That the Ocean modifier and the File Output node both say it explicitly, and the render output path does not, is why I will not guess either way.

**Not covered.** Whether the scene render output path creates folders at all; whether it creates more than one missing level; whether it creates them before rendering starts or at the moment the first frame is written; what happens to Placeholders when the folder is missing.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The File Output node sentence is about rendering, and the manual's own description of the node is "The File Output node writes one or more images to disk during rendering or compositing" — nothing ties it to a window. **Inferred** (the step: the sentence describes output behaviour, not a UI action). For the scene render output path, the sources do not say.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 3 — the scene's own output path.**

*What was done, to what.* At factory settings, with no window, the render output
path was set to `…/one_missing/img_` (one folder that did not exist) and one frame
rendered; then to `…/two_a/two_b/img_` (two folders that did not exist) and one
frame rendered.

*The command.*

```
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup --python q3.py -- …
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

**Answer. Not in the sources**, for all three conditions, with one adjacent exception.

The one adjacent thing the sources do state is what happens when the *path itself* is malformed rather than the destination unusable:

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

Source: manual — https://docs.blender.org/manual/en/5.1/files/file_paths.html
Date read: 2026-07-30
**Label: documented**, but for a different failure than the one asked about.

**Where I looked.** Across the whole manual, the whole API documentation and the whole 5.1 release notes I searched for: "no space", "disk full", "out of disk", "storage full", "insufficient space", "file system full", "quota", "ENOSPC", "write permission", "permission denied", "cannot write", "unable to write", "failed to write", "could not write", "unable to save", "cannot save", "unwritable", "not writable", "write-protected", "read-only file", "OSError", "IOError", "errno", "render fails", "rendering will fail", "aborts", "skips the frame" and "continues rendering". Every one of these returned either nothing or hits that have nothing to do with writing render output (for example "no spaces" in an argument-list description, or "Shadow buffer full" in EEVEE's light settings). I additionally read in full: the Output panel page, the Rendering Animations page, the Render output introduction page, the File Output node page, the File Paths page, the command-line arguments page, the command-line render page, the whole troubleshooting section's file list (`3d_view`, `crash`, `network`, `python`, `recover`, `report_bug`, `startup`, plus the GPU sub-pages), and the `RenderSettings` and `CompositorNodeOutputFile` API pages. There is no page in these sources describing what a failed write does.

**What the sources do cover nearby, which is not an answer but is worth separating out so nobody mistakes it for one.** Two settings govern whether a write is attempted at all, and they are documented:

> "Image Sequence – Overwrite Image Multi-Layer EXR
> Overwrite existing files when rendering."
> "Image Sequence – Overwrite Image Multi-Layer EXR
> Create empty placeholder frames while rendering."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
(Both entries carry the same heading text in the manual; the second is Placeholders. I am quoting the page as it reads.)

> "use_overwrite
> Overwrite existing files while rendering (default True)"
> "use_placeholder
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

Source: API — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html

**Not covered.** Everything the question asks: whether Blender stops or continues, what it prints, what exit status it returns, whether a partially written file is left on disk, and whether the three conditions behave alike.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

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
00:00.634  image.write      | ERROR OpenImageIO write failed: Could not open file "…/ro/img_0001.png"
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
00:00.455  image.openexr    | ERROR Unknown error in IMB_exr_write_channels
00:00.455  render           | Saved: '/Volumes/SMOOZYTINY/img_0001.exr'
00:00.484  image.openexr    | ERROR Unknown error in IMB_exr_begin_write
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

**Entry 23 — switching automatic saving off.** There is no such switch in 5.1:

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

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.1.2**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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
00:00.627  image.write      | ERROR OpenImageIO write failed: Could not open file "…/ro/img_0001.png"
Error: Render error (Success) cannot save: '…/ro/img_0001.png'
raised: RuntimeError Error: Render error (Success) cannot save: '…/ro/img_0001.png'
exit code: 1
```

and straight from the command line, with no Python at all:

```
00:01.016  image.write      | ERROR OpenImageIO write failed: Could not open file "…/ro/cli_0001.png"
00:01.016  reports          | ERROR Render error (Success) cannot save: '…/ro/cli_0001.png'
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
exit 0, said saved once, no error line about the disk -> 835584 bytes, ends properly: False
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
variables were unset. The build was **Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)**. Date: 2026-08-07.

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
00:02.553  render           | Saved: '/root/writing-full-disk/roomy/5.1/img_0001.exr'
00:02.710  render           | Saved: '/root/writing-full-disk/roomy/5.1/img_0002.exr'
00:02.833  render           | Saved: '/root/writing-full-disk/roomy/5.1/img_0003.exr'
operator returned: {'FINISHED'}
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)

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
00:02.127  image.openexr    | ERROR IMB_exr_write_channels: Failed to write pixel data to image file "/root/writing-full-disk/mnt/img_0001.exr". No space left on device.
00:02.127  render           | Saved: '/root/writing-full-disk/mnt/img_0001.exr'
00:02.201  image.openexr    | ERROR IMB_exr_begin_write: Cannot open image stream "/root/writing-full-disk/mnt/img_0002.exr". No space left on device.
Error: Error writing render result, No space left on device (see console)
Error: Render error (No space left on device) cannot save: '/root/writing-full-disk/mnt/img_0002.exr'
raised: RuntimeError Error: Error writing render result, No space left on device (see console)
Error: Render error (No space left on device) cannot save: '/root/writing-full-disk/mnt/img_0002.exr'

Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)

Blender quit
exit code: 1
```

**This is where the two machines differ.** On the Mac this version lost the reason entirely in the two OpenEXR lines — they read `Unknown error in IMB_exr_write_channels` and `Unknown error in IMB_exr_begin_write`. On Linux both lines name the real cause and the file. The two `Error:` lines below them are the same on both machines.

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
00:02.172  image.openexr    | ERROR imb_load_openexr: Unable to query scanline information
00:02.178  image.read       | ERROR IMB_load_image_from_memory: unknown file-format (/root/writing-full-disk/kept/5.1/img_0001.exr)
---- file: /root/writing-full-disk/roomy/5.1/img_0001.exr
     bytes on disk: 64032656
     Blender says size: (2000, 2000)  channels: 4
     numbers of colour read: 16000000
     expected for (2000, 2000) : 16000000
     numbers that are not zero: 16000000
---- file: /root/writing-full-disk/kept/5.1/img_0001.exr
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

**Answer.** The sources name these.

**Render Image (F12).** Its main Render Result is not saved automatically.

> "Render Image F12
> Render the active scene at the current frame."

Source: manual — https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html

> "Important
> While animation renders are automatically saved, still renders are not. These have to be saved manually."

Source: manual — https://docs.blender.org/manual/en/5.1/editors/image/editing.html

**Render Animation (Ctrl-F12).** Writes every frame.

> "Render Animation Ctrl-F12
> Render the animation of the active scene."

Source: manual — https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/animation.html

What it writes depends on the format: "Blender creates one file that holds all the frames of your animation" for the video route, and "Blender creates a file for each frame of the animation" for the image-sequence route (same page).

**Render Sequencer Image / Render Sequencer Animation.**

> "Render Sequencer Image Alt-F12
> If a Sequencer Scene exists that differs from the active scene, render that scene instead."
> "Render Sequencer Animation Ctrl-Alt-F12
> Render the animation of the Sequencer Scene."

Source: manual — https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html

**Render Audio.** Writes a sound file.

> "Render Audio
> Mix the scene's audio to a sound file."

Source: manual — https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html

> "bpy.ops.sound.mixdown(*, filepath='', check_existing=True, …)
> Mix the scene's audio to a sound file"

Source: API — https://docs.blender.org/api/5.1/bpy.ops.sound.html
Note this one takes its own `filepath`, not the render output path.

**Viewport render, still and animation.**

> "Render a Still Image
> To render a still image, use 3D Viewport ‣ View ‣ Render Viewport Preview."
> "Render an Animation
> To render an animation, use 3D Viewport ‣ View ‣ Render Playblast."
> "Render Keyframes
> To render an animation, but only those frames that have a keyframe, use 3D Viewport ‣ View ‣ Render Playblast on Keyframes. … The other frames are still written to the output, but will simply repeat the last-rendered frame."

It uses the scene's output settings: "Additionally, some output settings are used too: Resolution / Aspect / Output path / File format".

> "Note
> Viewport rendering only works for the Workbench and EEVEE render engines. It's not supported for Cycles."

Source: manual — https://docs.blender.org/manual/en/5.1/editors/3dview/viewport_render.html

**The command line.**

> "-a, --render-anim
> Render frames from start to end (inclusive)."
> "-f, --render-frame <frame>
> Render frame <frame> and save it."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
The words "and save it" on `-f` are the load-bearing ones: a single frame started this way is written, unlike F12.

**Python operators.**

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"
> "animation (bool) – Animation, Render files from the animation range of this scene (optional)"
> "write_still (bool) – Write Image, Save the rendered image to the output path (used only when animation is disabled) (optional)"

> "bpy.ops.render.opengl(*, animation=False, render_keyed_only=False, sequencer=False, write_still=False, view_context=True)
> Take a snapshot of the active viewport"

Source: API — https://docs.blender.org/api/5.1/bpy.ops.render.html
Date read: 2026-07-30

**The File Output node may also write during rendering or compositing.**

> "The File Output node writes one or more images to disk during rendering or compositing."

Source: manual — https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html

**Label: documented** for the quoted descriptions of each start and for the statements about animation, command-line and File Output node writing. **Inferred** for the claim that Render Image does not save its main result: the reader must join the Render Image description to the separate statement that still renders are not saved automatically. **Inferred** for the claim that a still started with `bpy.ops.render.render()` at its defaults does not save its main result, since `write_still` defaults to `False` and its own description says it is what saves the image.

**Not covered.** The answer does not establish what Render Sequencer Image writes, what Render Sequencer Animation writes, or what `bpy.ops.render.opengl` writes in each mode. Whether the menu items "Render Viewport Preview" and "Render Playblast" pass `write_still` as true or false is not stated; the manual says viewport render uses the output path but never says a viewport still is saved to it. Baking is not addressed here. Nothing states what any of these return as an exit status.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* `-a` and `-f` are outside-only. `bpy.ops.render.render` and `bpy.ops.render.opengl` can be driven from a script, and the API notes that using Blender as a Python module "is equivalent to running a script in background-mode (passing the command-line arguments --background or -b)" (https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html) — **documented** that background operation is a supported mode. The menu items and shortcuts are interface-only by definition. The sources do not say whether viewport render works in background mode.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say whether these ways of starting a render or their writes differ by operating system. One documented background-mode note is that "The audio device is disabled in background-mode by default and can be re-enabled by passing in -setaudio Default afterwards." (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html), which bears on Render Audio started in background.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 22 — what `-f`, `-a` and the Python render call each write.**

*What was done, to what.* One saved file holding both a scene output path and a
compositor File Output node was copied three times, and each copy rendered by one
route, each with its own `-o` folder so the routes could not see each other's work.

*The commands.*

```
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup …/r_f.blend  -o …/f_scene/s_ -F PNG -x 1 -f 1
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup …/r_a.blend  -o …/a_scene/s_ -F PNG -x 1 -a
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup …/r_py.blend -o …/py_scene/s_ -F PNG -x 1 --python q22.py
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
00:00.461  render           | Saved: '…/node_out/nd_Image.png'      (animation=False, write_still=False)
00:00.467  render           | Saved: '…/node_out/nd_Image.png'      (animation=False, write_still=True)
00:00.468  render           | Saved: '…/py_scene/s_.png'            (animation=False, write_still=True)
00:00.473  render           | Saved: '…/node_out/nd_Image0001.png'  (animation=True)
00:00.474  render           | Saved: '…/py_scene/s_0001.png'        (animation=True)
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

**Entry 23 — switching automatic saving off.** There is no such switch in 5.1 — see question 5 above for the exact words.

*Standing lines, brought up to date.* **Driven from outside** — the command-line and
Python routes are settled by the runs above. The viewport route is settled too, in
the other direction: it refuses to run with no window. What the menu entries do is
still open and sits with the Creator (entries 26 to 28 of the hand-test list).
**Operating system** — settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.1.2**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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

**Answer.** The sources establish that a still render does not automatically save its main Render Result. They do not establish that this means no files at all are written, or what every such render leaves behind.

**One: the Render Result, held in Blender, in a slot.**

> "Important
> While animation renders are automatically saved, still renders are not. These have to be saved manually."

> "Render Result: displays renders. When this item is selected, the Slot, View Layer, and Render Pass selectors become available (see below)."
> "Slot
> The render slot to view (and render to). You can create new renders without losing previous ones by selecting an empty slot before rendering."

Source: manual — https://docs.blender.org/manual/en/5.1/editors/image/editing.html and https://docs.blender.org/manual/en/5.1/editors/image/introduction.html

**Two: the temporary directory can hold run-time files, including render layers.** The quoted words do not say that every render creates one, or that a render asked to write no output still does so.

> "The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs)."
> "The temporary directory is selected based on the following priority:
> User Preference (see File Paths).
> Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms).
> The /tmp/ directory."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html
Date read: 2026-07-30

**Three: the render cache, if Cache Result is on.** This is a file on disk, so Cache Result must be off for a set-up intended to write no files.

> "Saving – Cache Result
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image. The Compositor can then use this file to improve performance, especially for heavy compositing.
> The image is stored in the Render Cache folder as specified in the File Paths Preferences. You can also load it back into the Image Editor's Render Result, even after closing and reopening Blender; see Open Cached Render."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html

> "Find the render cache file for the current scene and load it into the Render Result. This way, you can restore the last render from a previous Blender session and continue working in the Compositor without having to render the scene again."

Source: manual — https://docs.blender.org/manual/en/5.1/editors/image/editing.html

> "use_render_cache
> Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes) (default False)"

Source: API — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html

The manual's Output panel screenshot shows Cache Result unticked by default (`_images/render_output_properties_output_panel.png`).

**A fourth, adjacent case.** Placeholders are files, even when empty, so they must also be off for a set-up intended to write no files:

> "use_placeholder
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

Source: API — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html

> "Primitive Render Farm
> An easy way to get multiple machines to share the rendering workload is to:
> Set up a shared directory over a network file system.
> Disable Overwrite, enable Placeholders in the Render Output panel.
> Start as many machines as you wish rendering to that directory."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html

**Label: documented** for the Render Result and slot, the stated uses of the temporary directory, the optional render cache, and placeholders. **Not in the sources** for whether a render can be made to write no files at all and, if so, what it leaves behind beyond the in-memory Render Result. The earlier claim that "do not save it and nothing is written" did not follow from the quotes because the same answer names several other possible writes.

**Not covered.** Whether temporary render-layer files exist for every render or only some; whether they survive Blender exiting; how large they are. Whether anything at all is left behind by a render started in background mode with the main output, Cache Result, Placeholders and File Output nodes all disabled — the Render Result is described as something you look at in the Image Editor, and the sources never say what becomes of it with no window.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* This is the weak spot in the answer. Render slots and the Image Editor are interface things. The temporary directory and the render cache are not. The sources do not say what a background render leaves behind when it writes no output files.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The location of any temporary files is partly answered, and documented: the temporary directory differs by system — "Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms)" and, failing those, "The /tmp/ directory". The command-line page states the same variables: "TEMP: Store temporary files here (MS-Windows)." and "TMPDIR: Store temporary files here (UNIX Systems). The path must reference an existing directory or it will be ignored." (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html). The sources do not establish that the no-output set-up leaves such a file on any system.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 20 — a windowless render told not to save.**

*What was done, to what.* At factory settings, with no window, the render cache and
placeholders switched off, the output path pointed at an empty folder, and
`bpy.ops.render.render(animation=False, write_still=False)` called. The image
data-blocks were listed before and after, and the temporary folder and the output
folder were listed afterwards.

*The command.*

```
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup --python q20.py -- …
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

**Entry 23 — switching automatic saving off.** There is no switch for turning automatic saving off in 5.1 — see question 5.

*Standing lines, brought up to date.* **Driven from outside** — settled. The Render
Result exists in a windowless Blender and can be reached from a script, and a render
told not to save leaves nothing at all on disk. What becomes of it after the process
ends was not tested and cannot be: the process is gone. **Operating system** —
settled for macOS only.

### Observed — the Creator's hand tests, 2026-08-05

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.1.2**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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

**Answer.** Four parts are documented. The sources do not establish the order in which the first three are applied.

**Step 1 — `//` is expanded.**

> "This setting expands Relative Paths where a // prefix represents the directory of the current blend-file."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html

**Step 2 — `{…}` template expressions are substituted, at the moment of use.**

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

> "For example, if the currently open blend file is dance.blend and the render output path is set to this:
> //my_render_dir/{blend_name}.png
> Then when rendering, Blender will treat the path as:
> //my_render_dir/dance.png"

Source: manual — https://docs.blender.org/manual/en/5.1/files/file_paths.html

**Step 3 — the frame number goes where the `#` marks are; if there are none, it goes on the end.** This is the direct answer to "where a frame's number appears".

> "The # characters are replaced by the frame number, and used to define zero padding.
> animation_##_test.png becomes animation_01_test.png
> test-######.png becomes test-000001.png
> When the filename does not contain #, the suffix #### is added to the filename.
> The frame number will be added at the end of the filename, eg:
> blender -b animation.blend -o //render_ -F PNG -x 1 -a
> //render_ becomes //render_####, writing frames as //render_0001.png"

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html

> "When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png)."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html

**Step 4 — the end of the name is decided by the File Extensions switch and the chosen file format.**

> "Saving – File Extensions
> Adds the correct file extensions per file type to the output files."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html

> "use_file_extension
> Add the file format extensions to the rendered file name (eg: filename + .jpg) (default True)"
> "file_extension
> The file extension used for saving renders (default "", readonly, never None)"

Source: API — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html

From the command line the same switch is:

> "-x, --use-extension <bool>
> Set option to add the file extension to the end of the file."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html

**The whole conversion has a name, and you can ask for its result.**

> "frame_path(*, frame=-2147483648, preview=False, view='')
> Return the absolute path to the filename to be written for a given frame"
> "Returns:
> File Path, The resulting filepath from the scenes render settings (never None)"

Source: API — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html

**Observed, from Cycles' own files.** Cycles itself relies on that call to reproduce Blender's naming rather than rebuilding them. In `Blender Sources/cycles_5.1/operators.py`, in `CYCLES_OT_denoise_animation.execute`, lines 70–92, under the comment on lines 70–71: *(Citation check, 2026-08-05: comment line numbers corrected from 69–70.)*

```
            # Denoise animation sequence with expanded frames matching
            # Blender render output file naming.
            in_filepath = scene.render.filepath
```

and then, per frame:

```
            for frame in range(scene.frame_start, scene.frame_end + 1):
                scene.render.filepath = in_filepath
                filepath = scene.render.frame_path(frame=frame)
                in_filepaths.append(filepath)

                if not os.path.isfile(filepath):
```

File: `Blender Sources/cycles_5.1/operators.py`, lines 70–84.
Date read: 2026-07-30
**Label: observed** — this is what the file says. What it shows is that `frame_path` is treated by Blender's own bundled add-on as the authority on the per-frame name. Anything further about what a render then does with that name is a step beyond the file, and I have not taken it.

**For the File Output node, the shape of the built name is stated directly.**

> "For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."
> "Both the File Name and the Image names (when saving multiple images) can contain #### to force the frame number to appear in a specific place in the filename. This also has the effect that a single render will have the frame number embedded. The frame number will be padded with zeros to fill the number of # characters."
> "Output Paths
> Displays the final file path and extension for each input/layer, based on current settings. This helps verify that outputs are named and organized correctly."

Source: manual — https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html

**Label: documented** for steps 1–4 and the node's shape; **observed** for the Cycles quotation.

**Not covered.** The order of steps 1–3 relative to each other is my reading, not a statement in the sources — the pages describe each mark separately and never say which is applied first. Also not covered: what a *video* file is named when an animation is rendered to a movie container (whether a frame range is appended, what happens to `#` marks); what happens when there is more than one run of `#` in one path; whether the frame number is inserted before or after the extension when File Extensions is off; and how a negative frame number is written.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The `#`, `//`, `-x` and template rules are all documented on the command-line page for use with no window, so the naming rules demonstrably hold there. **Documented.** The File Output node's "Output Paths" preview panel is interface-only by its nature.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say the naming differs by operating system. Two path-shaped platform helpers exist in the API and are worth naming rather than guessing from: "bpy.path.native_pathsep(path) — Replace the path separator with the system's native os.sep." and "bpy.path.resolve_ncase(path) — Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path." (https://docs.blender.org/api/5.1/bpy.path.html). Neither is documented as being applied to render output paths, so I draw no conclusion from them.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

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

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.1.2**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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
then one row per input:

```
File Output
  [ //out/ ]  📁
  [ shot   ]  📁
  ○ passes/diffuse
  ○ beauty
  ○                        (empty)
```

The input named `passes/diffuse` keeps its slash here.

*What is in the sidebar.* A **Node** panel holding Name (`File Output`), Label
(placeholder text `Custom Label`), Color, `[x] Show Options` and `[ ] Mute`. Then a
**Properties** panel holding, in this order:

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
  /Users/artist/Documents/Sm…sts/Reading a file/13/out/shot.exr
Custom Properties
  [ + New ]
```

(The path in the Output Paths panel is shown shortened in the middle by Blender
itself.) So the node records two path fields plus a name per input, and it carries a
panel called **Output Paths** that shows the finished name before anything is
rendered.

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

I read this as two questions about the File Output node's directory and file name: (a) can they be set at the moment a render is launched, and (b) can they be set by changing the blend-file from outside Blender. If the intended reading of (b) was instead "by editing the `.blend` bytes directly with something that is not Blender", the sources say nothing at all about that, and my answer to (b) would be "not in the sources".

**(a) At the moment the render is started.**

There is **no command-line argument** for it. The one output argument is scoped to the scene render path:

> "-o, --render-output <path>
> Set the render path and file name."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
I read that whole page. It is the complete list of arguments ("Usage: blender [args ...] [file] [args ...]" heading a sectioned list of Render, Cycles, Format, Playback, Window, Python, Network, Logging, Debug, GPU, Misc and Other options). Nothing on it mentions the compositor or the File Output node. **Label for "no dedicated argument exists": inferred** — the step is reading a complete argument list and finding the thing absent.

There **is** a route, via Python at launch, because the node's paths are ordinary writable properties:

> "directory
> The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)"
> "file_name
> The base name of the file. Other information might be included in the final file name depending on the node options (default "", never None, Supports template expressions)"

Source: API — https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html

They are reachable from the scene:

> "compositing_node_group
> Compositor Nodes
> Type:
> NodeTree"

Source: API — https://docs.blender.org/api/5.1/bpy.types.Scene.html

and a script can be run at launch:

> "-P, --python <filepath>
> Run the given Python script file."
> "--python-expr <expression>
> Run the given expression as a Python script."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
Date read: 2026-07-30

**Label: inferred** for "yes, via a launch-time script". The step a reader must supply: that a writable property, reachable from `Scene`, changed by a script run before the render argument, is in force when the render happens. The sources state each link but never join them. The same ordering rule as Q1 applies — the manual's rule is "Arguments are executed in the order they are given" — so the script must come after the blend-file and before `-f`/`-a`.

There is a documented alternative that changes the destination **without any launch-time argument at all**: templates. Because the node's paths support template expressions, the destination can be made to vary by itself.

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node."
> "Available only in path properties on a node:
> node_name: The name of the node that the path property is on."

Source: manual — https://docs.blender.org/manual/en/5.1/files/file_paths.html
**Label: documented.** This settles the destination per blend-file, scene, camera, resolution or frame rate — but not per launch, since none of the listed variables is supplied from outside.

**(b) By writing it into the file from outside Blender.**

The properties are writable and the file can be saved without a window. The pieces, quoted:

> "-b, --background
> Run in background (often used for UI-less rendering)."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html

> "bpy.ops.wm.save_mainfile(*, filepath='', …)
> Save the current Blender file"

Source: API — https://docs.blender.org/api/5.1/bpy.ops.wm.html

And, if Blender is not to be launched as a program at all:

> "Blender supports being built as a Python module, allowing import bpy to be added to any Python script, providing access to Blender's features."
> "Note
> Blender as a Python Module isn't provided on Blender's official download page. A pre-compiled bpy module is available via PIP."
> "For the most part using Blender as a Python module is equivalent to running a script in background-mode (passing the command-line arguments --background or -b), however there are some differences to be aware of."

Source: API — https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html
Date read: 2026-07-30

**Label: inferred.** The step: writable node property + background mode + a save operator = the path can be written into the blend-file with no window. No page says this in one place.

**Not covered.** Nothing in the sources confirms that a File Output node path changed by a launch-time script is actually honoured by the render that follows — I am reasoning from the property being writable, not from a statement. Nothing states whether `-o` has any effect on File Output nodes (I believe not, but the sources do not say so; they only scope templates, not `-o`). Nothing describes any command-line way to address a specific node by name. Nothing describes editing a `.blend` with a non-Blender tool.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* This whole answer is about outside the interface, and background mode is documented ("Run in background (often used for UI-less rendering)"). The gap named above — no statement that a script-set node path takes effect — is exactly the gap that matters most for outside-the-interface use, and I will not paper over it.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say whether either route differs by operating system.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

**Entry 16 — does `-o` reach a File Output node?**

*What was done, to what.* A file was saved holding a scene output path pointing at
one folder and a File Output node named `FileOut` pointing at another. Blender was
started with no window, given that file, then `-o` pointing at a third folder, then
`-f 1`. Afterwards the file was reopened and the node's stored values printed.

*The command.*

```
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup …/node.blend -o …/cli_out/cli_ -F PNG -x 1 -f 1
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
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup …/node.blend --python-expr "import bpy; sc=bpy.context.scene; t=sc.compositing_node_group; n=t.nodes['FileOut']; n.directory='…/from_cli/'; n.file_name='zz_'; print('set to', n.directory, n.file_name)" -f 1
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

**Answer, first half: not in the sources.** The sources describe text fields, but do not state that they can store a character rejected by the computer that later receives the file. The only checking they describe is of template syntax, not of characters the filesystem would reject:

> "Paths that support templates can have template errors, which prevent the path from being processed."
> "When there are template errors in a path, the path field will be highlighted red in the UI"

Source: manual — https://docs.blender.org/manual/en/5.1/files/file_paths.html
Date read: 2026-07-30

That Blender knows some characters are troublesome is documented, but only as a helper you must call yourself — it is not described as being applied to render output paths:

> "bpy.path.clean_name(name, *, replace='_')
> Returns a name with characters replaced that may cause problems under various circumstances, such as writing to a file.
> All characters besides A-Z/a-z, 0-9 are replaced with "_" or the replace argument if defined."

> "bpy.path.display_name_to_filepath(name)
> Performs the reverse of display_name using literal versions of characters which aren't supported in a filepath."

> "bpy.path.native_pathsep(path)
> Replace the path separator with the system's native os.sep."

> "bpy.path.resolve_ncase(path)
> Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path."

Source: API — https://docs.blender.org/api/5.1/bpy.path.html

The one hard constraint the sources do publish is length, not character set:

> "directory:
> 767
> file-name:
> 255
> file-path:
> 1023"
> "Note
> Multi-byte encoding means some Unicode characters use more than a single ASCII character."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/limits.html

**Answer, second half — what a render does when the path holds such a character: not in the sources.**

**Where I looked.** Across the whole manual, the whole API documentation and the whole 5.1 release notes I searched for: "invalid character", "illegal character", "reserved character", "forbidden character", "sanitiz", "valid filename", "invalid filename", "colon", "asterisk", "question mark", "trailing space", "reserved name", "CON, PRN", "backslash", "path separator", "case sensitive", "case-sensitive", "UTF-8", "unicode", "NTFS", "APFS", "ext4", "SMB", "NFS", "MAX_PATH", "260 char", "path length" and "long path". The only character-restriction statement anywhere in the manual is about asset catalogs, not file paths — "Invalid characters: :, \." (https://docs.blender.org/manual/en/5.1/files/asset_libraries/catalogs.html) — and the only encoding statements are about USD prim names and about asset catalog files being UTF-8. I also read in full: the File Paths page, the Working Limits page, the Output panel page, the File Output node page, the command-line arguments page, the `bpy.path` page and the `RenderSettings` page. Nothing describes rejection, substitution, truncation or failure of a render output path on character grounds.

**Label: documented** for the length limits and for the existence of the `bpy.path` helpers; **not in the sources** for whether a recorded render path can carry a character unacceptable to the target computer and for what a render does when it does.

**Not covered.** Whether the path is validated at all before rendering; whether an over-long path is truncated or refused; whether a character that is fine on one system and not another produces any warning; what happens when a `{blend_name}` or `{scene_name}` substitution *introduces* such a character (this last is the sharp one — the manual documents that scene and camera names go into paths, and Blender allows characters in names that a filesystem may not allow in a filename, but the sources never join those two facts).

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The one reporting mechanism the sources describe for a bad path — the red field and its tooltip — is explicitly of the interface ("highlighted red in the UI"). What is reported outside it is not stated, except that a template error makes an animation render "fail with an error message". For character problems specifically, the sources do not say.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* This is the heart of the question and the sources do not answer it. What they do document is that Blender ships helpers acknowledging the differences — a native path separator, and case-insensitive resolution on a case-sensitive system — which is evidence that the differences exist, not evidence of what a render does about them. I will not go further than that.

---

### Observed — hand tests run 2026-08-05

All of the below was run on the Creator's own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only — from a terminal, with Blender started with no window and with `BLENDER_USER_RESOURCES` pointed at a fresh empty settings folder so the Creator's own Blender settings were never read or written. Date: 2026-08-05. The build used for this version was **Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)**. In the output below, `…` stands for the working folder, which was outside this project. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests.md`.

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

These were run by the Creator, at Blender's own window, on their own computer — one MacBook Pro, Apple M1 Pro, macOS 26.5.2, Apple graphics only. That settles macOS and leaves Windows and Linux open. The version used here was **Blender 5.1.2**. Date: 2026-08-05. The entries come from `Audits/Blender Research/Writing/Hand tests.md`.

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
00:00.637  image.write      | ERROR OpenImageIO write failed: Could not open file "…/long/nnn…nnn0001.png"
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

2026-08-04 — Opus 5 (Claude Code) — Writing, Blender 5.1
