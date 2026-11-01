# Writing — Blender 4.5

**Sources consulted:** Blender 4.5 manual (plain text plus its `_images` pictures), Blender 4.5 Python API documentation, Blender 4.5 release notes, and Cycles' own files as shipped with 4.5 — all as supplied in `Blender Sources/`, snapshot taken 2026-07-30. Date read for every finding below: 2026-07-30.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 1. Can where a render writes be settled at the moment the render is started, without changing the file?

**Answer.** **Inferred yes.** Blender's command line takes an argument that sets the render path and file name after the blend-file has been loaded. The documented part is that the later command-line setting works for that run. The step supplied by the reader is that this is a run-time setting rather than a change to the file on disk; the sources do not state that part.

> "-o, --render-output <path>
>
> Set the render path and file name.
> Use // at the start of the path to render relative to the blend-file."

The same page is explicit that the order of arguments decides whether this works, because loading a blend-file resets the value:

> "blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set.
>
> blender --background test.blend --render-output /tmp --render-frame 1
>
> …works as expected."

Two neighbouring arguments settle the rest of the name in the same way: `-F, --render-format <format>` ("Set the render format.") and `-x, --use-extension <bool>` ("Set option to add the file extension to the end of the file."). A worked example on the same page:

> "blender -b animation.blend -o //render_ -F PNG -x 1 -a"

**Label:** inferred for the full answer; documented for the command-line arguments and their order.

**Not covered.** The sources do not say whether `-o` alters the blend-file on disk. They do not say what `-o` does to the compositor's File Output nodes — see question 9. They do not give a limit on how many times it may be given, nor what happens if it is given twice.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* This *is* the outside-the-interface route. The same page documents `-b, --background` — "Run in background (often used for UI-less rendering)" — and the worked example combines `-b`, `-o` and `-a`. Inside Blender's own window the equivalent is typing into the Output panel's File Path field, which does change the file's stored value. Label for that contrast: documented for the command line, inferred for "inside the window it changes the stored value" (the step: the manual calls File Path a setting of the scene, and `RenderSettings.filepath` is a stored property, so editing it edits the scene).

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The command-line arguments page marks operating-system-specific arguments in so many words — `-con, --start-console` is "(Windows only)", `--debug-ghost` is "(Linux only)", `-r, --register` is "(Windows & Linux only)". `-o, --render-output` carries no such mark. Label: inferred (step: the page's convention is to name the restriction, and it names none here). The sources do not state this outright.

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

**Answer.** There are two different recordings, with different shapes.

**(a) The scene's render output.** One single string.

> "filepath
>
> Directory/name to save animations, # characters define the position and padding of frame numbers
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported, Supports template expressions)"

So the one string carries the folder and the leading part of the name together. The manual's panel calls it File Path — "Choose the location to save rendered frames."

The rest of the final name is recorded separately, not inside that string:

> "use_file_extension
>
> Add the file format extensions to the rendered file name (eg: filename + .jpg)
>
> Type:
>
> boolean, default True"

> "file_extension
>
> The file extension used for saving renders
>
> Type:
>
> string, default "", (readonly, never None)"

and the format itself:

> "file_format
>
> File format to save the rendered images as
>
> Type:
>
> enum in Image Type Items, default 'PNG'"

**(b) The compositor's File Output node.** Two strings, one on the node and one per slot.

> "Unlike the render output filepath, this node uses a base directory and an image name,
> by default the output path is composed of:
> {base path}/{file name}{frame number}.{extension}."

> "base_path
>
> Base output path for the image
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported, Supports template expressions)"

> "path
>
> Subpath used for this slot
>
> Type:
>
> string, default "", (never None, Supports template expressions)"

For multilayer output the slot records a layer name instead of a path piece:

> "name
>
> OpenEXR layer name used for this slot"

**Lengths.** The manual gives fixed internal limits:

> "directory:
>
> 767
>
> file-name:
>
> 255
>
> file-path:
>
> 1023"

with the caveat "Multi-byte encoding means some Unicode characters use more than a single ASCII character."

*(Citation check, 2026-08-05: narrowed. The page introduces these numbers generically
— "Fixed strings are used internally, and while it is not useful to list all limits,
here are some common limits. Used for data-block names, modifiers, vertex groups, UV
layers…" — and nowhere says they are the limits enforced on the render output path.
That they are is a step the reader supplies. Entry 25 below is what shows 1023 is the
figure actually enforced on the stored text.)*

**Where a new scene's value comes from.**

> "render_output_directory
>
> The default directory for rendering output, for new scenes"

**Picture.** The manual's Output panel screenshot (`_images/render_output_properties_output_panel.png`, shown on the Output page) shows the File Path field holding `/tmp/`, with Overwrite ticked, Placeholders unticked, File Extensions ticked, Cache Result unticked, File Format PNG, Color RGBA, Color Depth 8, Compression 15%. The File Output node screenshot (`_images/node-types_CompositorNodeOutputFile.webp`, on the File Output Node page) shows a node with a "Base Path:" field holding `/tmp/` and a single "Image" input. Label for both: documented (what is visible in the picture only — the pictures do not state that these are factory defaults).

**Label:** documented.

**Not covered.** How these strings are laid out inside the .blend file itself (byte format) is not described anywhere in the sources. Whether the recorded string is stored already expanded is answered only for templates: "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field." Nothing is said about whether `//` or `#` are likewise stored unexpanded, though the same page's wording implies they are read at use time.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The recording is a property of the scene and of the node, reachable through the Python API (`bpy.types.RenderSettings.filepath`, `bpy.types.CompositorNodeOutputFile.base_path`, `bpy.types.NodeOutputFileSlotFile.path`), which is what a driven Blender uses. The API documentation makes no distinction between window and background use for these properties. Label: inferred (step: the properties are documented without any note limiting them to interactive use).

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say. The length limits page names no operating system. The `//` prefix and the template syntax are described without any platform note.

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


## 3. What characters or marks can a recorded place to write carry that stand for something else, and what does each do?

**Answer.** Five were found. Four are documented directly. The fifth, `%`, is inferred from the `frame_path` API entry.

**`//` at the start — the folder of the blend-file.**

> "This setting expands Relative Paths
> where a // prefix represents the directory of the current blend-file."

and

> "When relative paths are supported, the File Browser provides a Relative Path checkbox,
> when entering the path into a text field, use a double slash prefix (//) to make it so."

and, in the preferences:

> "The default path // refers to the folder of the currently open blend-file"

with a stated limit:

> "You cannot use relative paths into a new untitled blend-file.
> Save it before linking to external files."

**`#` — the frame number, and how many digits it gets.**

> "You can set a custom padding size by adding the appropriate number of # anywhere in the file name
> (e.g. image_##_test.png translates to image_01_test.png)."

and

> "The # characters are replaced by the frame number, and used to define zero padding.
>
> animation_##_test.png becomes animation_01_test.png
>
> test-######.png becomes test-000001.png
>
> When the filename does not contain #, the suffix #### is added to the filename."

**`{name}` — a template expression, replaced by a value.**

> "Path templates substitute template expressions (written {abcd}, where
> "abcd" is a variable name) in a filepath when the path is used."

The variables available are listed:

> "blend_name:
>
> The current blend file's name (without the .blend).
>
> fps:
>
> The frames per second of the current scene.
>
> resolution_x / resolution_y:
>
> The x and y resolution of the rendered
> image. This factors in the resolution scale as well…"

with "The set of available variables will expand over time." A colon inside the braces introduces a format specifier, and inside that specifier `#` means a digit, not a frame number:

> "dance_{fps:###}.png: format as an integer with at least 3 digits.
>
> dance_{fps:.###}.png: format as a floating point number with exactly 3
> digits after the decimal point.
>
> dance_{fps:###.##}.png: format as a floating point number with at least 3
> digits for the integer part and exactly 2 digits for the fractional part."

Where templates are allowed is stated:

> "Currently path templates are only supported for the render output path in
> Scene properties and the output paths in the compositor's File Output
> node. More filepaths will support templates in future releases."

The release notes say the same:

> "Some file paths in Blender can now contain "template expressions" like
> `{blend_name}` and `{resolution_x}`, which get replaced by the values they
> reference when the path is used."

**`{{` and `}}` — a literal brace.**

> "{{ translates to a single { in the final path.
>
> }} translates to a single } in the final path."

> "my_weird}}_{{path.png -> my_weird}_{path.png"

**`%` — the name of a view (multi-view / stereo).** From the Python API for `frame_path`:

> "view (string, (optional, never None)) – View, The name of the view to use to replace the "%" chars"

This implies that `%` characters can occur in the path used by `frame_path`, but the quoted words do not directly say where `%` may be recorded in a render output path. Label: inferred.

**Label:** documented for `//`, `#`, `{name}`, `{{` and `}}`; inferred for `%` as stated above.

**Not covered.** For `%` the sources describe it only as a parameter of `frame_path`; no manual page explains where a `%` may be put in an output path, what happens when a path has no `%` and several views are rendered, or how many `%` characters are allowed. The multi-view manual page says only "By default the system saves each view as an individual file, thus generating as many files as views to be rendered" without naming the mark. The sources also do not say whether `#`, `//`, `{}` and `%` interact — for instance whether a template expression may itself produce a `#` that is then treated as a frame number.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Nothing in the sources ties any of these five to the window. `//` and `{blend_name}` are both documented on the command line: "-o, --render-output <path> … Use // at the start of the path to render relative to the blend-file. You can use path templating features such as {blend_name} in the path." `#` likewise. Label: documented for `//`, `#` and `{}`; the sources do not say for `%` and `{{`/`}}`.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say. None of the five is given a platform note. Note in passing that the API offers `bpy.path.native_pathsep(path)` — "Replace the path separator with the systems native os.sep" — which shows Blender knows systems differ on separators, but the sources do not connect that to render output paths.

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

**Answer.** **Not in the sources.** No page in the 4.5 manual, the 4.5 Python API documentation, the 4.5 release notes or Cycles' files states whether rendering creates a missing output folder.

**Where I looked.** I searched the whole of the manual, the whole of the API documentation and the whole of the release notes for: `makedirs`, `mkdir`, "create the directory", "creates the director", "directory … created", "created if … exist", "does not exist", "automatically create", "create … folder", "folder … create", "output directory", "output folder", "missing directory", "nonexistent", "non-existent". I read in full: the Output properties page, the Rendering Animations page, the File Paths (Path Templates) page, the File Output Node page, the Command Line Arguments page, the File Paths preferences page, and the Relative Paths section of Opening & Saving.

Two nearby statements exist, and neither answers the question:

- The Ocean modifier page says "If the cache path folder does not exist, it will be created." That is the Ocean modifier's cache, not the render output.
- The File Browser page says, of its Directory field, "Tab will auto-complete an existing path. If you type a nonexistent path, you will be prompted to create it." That is a prompt in the File Browser, and a File Browser cannot appear in a background render.

Neither is about writing a rendered frame, so I am not carrying either across.

**Label:** not in the sources.

**Not covered.** Everything: the still-image case, the animation case, the File Output node case, and whether a missing *intermediate* folder differs from a missing last folder.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not in the sources. The File Browser prompt above cannot apply when Blender is driven from outside its window, but that only removes one route; it says nothing about what a render then does.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not in the sources.

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

**Answer.** **Not in the sources** for all three cases.

**Where I looked.** I searched the manual, the API documentation, the release notes and Cycles' files for: "disk full", "out of space", "no space", "write error", "cannot write", "unable to write", "failed to write", "write permission", "writable", "read-only", "permission", "does not exist", "missing directory". I read in full the Output properties page, the Rendering Animations page, the Command Line Arguments page (including its Argument Order and Environment Variables sections) and the Troubleshooting pages on startup and crashes. The only "failed to write" anywhere in Cycles' files is in `osl.py`, about compiling a shader script, not about rendering a frame.

**What the sources *do* say about a render failing to produce its file** — one cause only, and it is Blender's own path syntax rather than the disk:

> "Depending on the path, template errors may prevent certain actions. For
> example, if there are errors in the render output path, then rendering an
> animation will fail with an error message indicating the path errors."

The release notes record the same change on the Python side:

> "`scene.render.frame_path()` used to be infallible but can now fail if there
> are errors processing [Path
> Templates](https://docs.blender.org/manual/en/4.5/files/file_paths.html#path-templates)
> in the render output path."

The manual also shows what that error looks like. In the picture `_images/files_file_paths-invalid_path_template_tooltip.png` (on the File Paths page) the Output field is drawn in red holding `//my_render_dir/{blend_name.png`, and the tooltip reads: "Output Path — Directory/name to save animations, # characters define the position and padding of frame numbers. Value: //my_render_dir/{blend_name.png — Syntax error(s): - Invalid or incomplete template expression '{blend_name.png'." Label for the picture: documented.

Two settings govern a different kind of clash — a file that is already there, not a place that cannot be written:

> "Overwrite
>
> Overwrite existing files when rendering.
>
> Placeholders
>
> Create empty placeholder frames while rendering."

**Label:** not in the sources (for the three cases asked); documented for the path-template failure quoted above.

**Not covered.** Whether a failed write stops an animation or lets it continue; whether Blender retries; what exit code a background Blender returns; whether anything is left half-written.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The manual's only stated symptom is a red field and a tooltip — both of which need a window. What a background run reports instead is not in the sources. The command-line page documents `--python-exit-code <code>` for Python exceptions only, and `-q, --quiet` ("Suppress status printing (warnings & errors are still printed)"), which tells us errors are printed but not which errors.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not in the sources.

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

**No room left on the volume — run by the runner on 2026-08-06, once the rented machine was idle.** This had been left as *needs the machine to itself* while other work was on the box. It is now run, and it is the sharpest result this question has.

*What was done, to what.* On the rented Linux machine, with nothing else running on it, the disk was filled with a ballast file until two megabytes remained. A one-line write was tried first to show the disk really was full. Then Blender was asked to render one frame of a default scene at 1920 by 1080 on the graphics card and save it as a PNG into that full disk. Each version was pointed at its own settings folder, and the ballast was removed afterwards.

*What came back.* Blender printed its ordinary saving line and exited **0**:

```
Saved: '/root/smoozy/Machine notes/full.png'
blender exit: 0
```

*But the file it saved is not whole.* Read back straight afterwards it does not end as a PNG must. **This was checked twice, and the first check was faulty** — it compared a file's last eight bytes against a twelve-byte marker, which can never match, so it would have called any file incomplete. That fault was found later and the whole test was run again in all four versions with a correct check. This is the second, sound run:

```
exit 0, said saved once, no error line about the disk -> 831488 bytes, ends properly: False
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
variables were unset. The build was **Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)**. Date: 2026-08-07.

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
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Saved: '/root/writing-full-disk/roomy/4.5/img_0001.exr'
Time: 00:02.32 (Saving: 00:00.08)

Saved: '/root/writing-full-disk/roomy/4.5/img_0002.exr'
Time: 00:00.12 (Saving: 00:00.08)

Saved: '/root/writing-full-disk/roomy/4.5/img_0003.exr'
Time: 00:00.12 (Saving: 00:00.08)

operator returned: {'FINISHED'}

Blender quit
exit code: 0
-rw-r--r-- 1 root root 64032536 Aug  7 01:23 img_0001.exr
-rw-r--r-- 1 root root 64032536 Aug  7 01:23 img_0002.exr
-rw-r--r-- 1 root root 64032536 Aug  7 01:23 img_0003.exr
```

So a run that writes nothing because the set-up was wrong is tellable from a run that
writes a broken file. **A negative result** would have been the full-disk run doing what
the control did — three whole frames, exit code 0 — or writing nothing at all and
saying so cleanly, or carrying on past frame 1. None of those happened.

*What came back on the full disk.*

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
OpenEXR-writePixels: ERROR: Failed to write pixel data to image file "/root/writing-full-disk/mnt/img_0001.exr". No space left on device.
Saved: '/root/writing-full-disk/mnt/img_0001.exr'
Time: 00:01.67 (Saving: 00:00.01)

IMB_exr_begin_write: ERROR: Cannot open image file "/root/writing-full-disk/mnt/img_0002.exr". No space left on device.
Time: 00:00.04

Error: Error writing render result, No space left on device (see console)
Error: Render error (No space left on device) cannot save: '/root/writing-full-disk/mnt/img_0002.exr'
raised: RuntimeError Error: Error writing render result, No space left on device (see console)
Error: Render error (No space left on device) cannot save: '/root/writing-full-disk/mnt/img_0002.exr'


Blender quit
exit code: 1
```

On the Mac this version named the cause in both lines too, but named no file in the second: `IMB_exr_begin_write: ERROR: No space left on device.` On Linux the same line also names the file it could not open.

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
Unable to query scanline information
IMB_load_image_from_memory: unknown file-format (/root/writing-full-disk/kept/4.5/img_0001.exr)
---- file: /root/writing-full-disk/roomy/4.5/img_0001.exr
     bytes on disk: 64032536
     Blender says size: (2000, 2000)  channels: 4
     numbers of colour read: 16000000
     expected for (2000, 2000) : 16000000
     numbers that are not zero: 16000000
---- file: /root/writing-full-disk/kept/4.5/img_0001.exr
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

**Answer.** The sources name these routes.

**From the Render menu (manual, Topbar):**

> "Render Image F12
>
> Render the active scene at the current frame.
>
> Render Animation Ctrl-F12
>
> Render the animation of the active scene.
>
> …
>
> Render Audio
>
> Mix the scene's audio to a sound file."

What Render Animation writes is stated plainly:

> "While rendering stills will allow you to view and save the image from the render buffer when
> it is complete, animations are a series of images, or frames,
> and are automatically saved directly out to a drive after being rendered."

What Render Image writes is **not** stated on that page. A render operator carries this switch:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, layer='', scene='')
>
> Render active scene
>
> …
>
> write_still (boolean, (optional)) – Write Image, Save the rendered image to the output path (used only when animation is disabled)"

The sources do not state what the F12 menu item passes to this operator. The operator's default therefore does not establish what F12 writes. Label for what F12 writes: **not in the sources**. The pages quoted here were the Topbar Render menu, Rendering Animations and the `bpy.ops.render.render` entry; none states the menu item's `write_still` value.

**Viewport Render (manual, 3D Viewport):**

> "To render a still image, use 3D Viewport ‣ View ‣ Viewport Render Image."
>
> "To render an animation, use 3D Viewport ‣ View ‣ Viewport Render Animation."
>
> "To render an animation, but only those frames that have a keyframe,
> use 3D Viewport ‣ View ‣ Viewport Render Keyframes."

It uses the same recorded place:

> "Additionally, some output settings are used too:
>
> Resolution
>
> Aspect
>
> Output path
>
> File format"

and its Keyframes mode writes a file for every frame, not only the keyed ones:

> "The other frames are still written to the output, but will simply repeat the last-rendered frame."

Its operator has the same Write Image switch:

> "bpy.ops.render.opengl(*, animation=False, render_keyed_only=False, sequencer=False, write_still=False, view_context=True)
>
> Take a snapshot of the active viewport"

with the note "Viewport rendering only works for the Workbench and EEVEE render engines. It's not supported for Cycles."

**From the command line:**

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

> "-f, --render-frame <frame>
>
> Render frame <frame> and save it.
>
> +<frame> start frame relative, -<frame> end frame relative.
>
> A comma separated list of frames can also be used (no spaces).
>
> A range of frames can be expressed using .. separator between the first and last frames (inclusive)."

Note that `-f` says "and save it" outright, where the menu's Render Image does not.

**From Python:** `bpy.ops.render.render(...)` and `bpy.ops.render.opengl(...)` above, driven by `-P, --python <filepath>` ("Run the given Python script file"), `--python-text <name>`, or `--python-expr <expression>`.

**Alongside any of these, other things may write during the same render:**

- The compositor's File Output node: "This node writes out an image, for each frame range specified, to the filename entered, as part of a frameset sequence." and "The image(s) will be saved on rendering, writing to the current frame. An entire sequence of images will be saved, when an animation is rendered."
- The render cache, if switched on: "Cache Result — Saves the rendered view layers and their passes to a multi-layer OpenEXR image. … The image is stored in the Render Cache folder as specified in the File Paths Preferences." Its default is off (`use_render_cache`, "boolean, default False").
- Placeholders, if switched on: "Create empty placeholder files while rendering frames (similar to Unix 'touch')."

**A hook that marks each write:**

> "bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written)"

alongside `render_init`, `render_pre`, `render_post`, `render_complete`, `render_cancel`.

**Label:** documented for every quoted line; not in the sources for what Render Image via the menu writes.

**Not covered.** The sources do not say what the *menu* items pass to their operators, so what F12 writes remains open. They do not say what happens when `-a` and `-f` are both given. They do not say whether Render Audio uses the render output path or asks for its own (the Audio page lists a Relative Path option and a Container, which suggests its own file browser, but that is not stated). They say nothing about rendering started from the Video Sequencer beyond "Click the Animation render button and Blender will render out the Sequence editor output into a movie."

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* `-a`, `-f`, `-b` and `-P` are the outside routes and are fully documented as such. The menu items and the Viewport Render menu items need a window. Viewport Render is a special case worth flagging: it is described as rendering "from the current active view … if you are not in an active camera view, a virtual camera is used to match the current perspective", which is a window notion; the sources do not say what `bpy.ops.render.opengl` does with `view_context=True` when there is no window. Label: documented for the command-line routes, not in the sources for the viewport route driven from outside.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say, with one aside: `-a` is overloaded — "Instead of showing Blender's user interface, this runs Blender as an animation player … (ignored if -b is set)". That overload is documented without any platform restriction.

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

**Answer.** **Inferred.** A still render with its file-writing routes switched off leaves a data-block called Render Result in the Blender session, held in a render slot and viewable in the Image Editor. The sources document the Render Result and the separate act of saving a still, but they do not state the whole no-files conclusion in one place.

> "Render Result: displays renders. When this item is selected, the Slot,
> View Layer, and Render Pass selectors become available (see below)."

> "Slot
>
> The render slot to view (and render to). You can create new renders without
> losing previous ones by selecting an empty slot before rendering. Afterwards,
> you compare them by pressing J and Alt-J to cycle forwards and backwards."

The API confirms this is a real data-block kind:

> "enum in ['IMAGE', 'MULTILAYER', 'UV_TEST', 'RENDER_RESULT', 'COMPOSITING'], default 'IMAGE', (readonly)"

(that is `bpy.types.Image.type`), and `bpy.types.Image.render_slots` is "RenderSlots bpy_prop_collection of RenderSlot, (readonly)".

A second image data-block exists if the compositor has a Viewer node:

> "Viewer Node: displays the image that's fed into the
> Viewer Node in the Compositor."

The colour-management page names both together: "These images are the "Render Result" and "Viewer" image data-blocks".

The manual contrasts stills, which it says you can view and save from the render buffer, with animations, which it says are automatically saved to a drive. The step from that contrast to "a still is not saved unless asked" is inferred. The Sequencer preview likewise offers "Show the current frame preview as a Render Result where you can save it as an image file."

**One thing that does survive Blender closing, if switched on:** the render cache.

> "Find the render cache file for the current scene and load it into the
> Render Result. This way, you can restore the last render from a previous
> Blender session and continue working in the Compositor without having to
> render the scene again.
>
> Note that Blender doesn't create these cache files by default. You
> have to enable Cache Result in the scene's Output options and then render it at least once."

That is a file, so a render with Cache Result on is not a render that writes nothing.

**Label:** inferred for the whole answer. The existence of Render Result, render slots and the separate save action are documented; the step is that switching off every file-writing route still leaves that data-block.

**Not covered.** The sources do not say how many slots there are, nor whether the Render Result survives loading another file, nor whether it is saved into the blend-file. They do not say what a *background* run leaves behind when nothing is written — see below. They do not say whether a render with the output path pointing nowhere still fills the Render Result.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* This is where the answer is weakest, and it matters. Everything quoted above is about the Image Editor and about a Blender session that stays open. When Blender is started with `-b` and told to render one frame, `-f` is documented as "Render frame <frame> and save it" — it saves. Whether a background run that does *not* save leaves anything reachable is **not in the sources**; the process ends and the Image Editor never existed. I searched the API documentation for "Render Result" (no match outside the index) and for `render_slots` / `RenderSlot` / `RENDER_RESULT`, which is how I found the data-block entries above; none of them carries a note about background mode.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say. Nothing in the Image Editor or render slot documentation names a platform.

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


## 8. How does the place a file records for writing become the name of a file on disk? Where does a frame's number appear, and what decides the end of the name?

**Answer.** Three pieces, in order: the recorded string, the frame number, and the extension.

**The recorded string.** For the scene's render output it is one string holding folder and name-stem together — "Directory/name to save animations". A `//` at the front is replaced by the blend-file's folder; `{…}` expressions are replaced by their values at the moment of use.

**Where the frame number goes.** Wherever `#` characters sit, and if there are none, on the end:

> "The # characters are replaced by the frame number, and used to define zero padding.
>
> animation_##_test.png becomes animation_01_test.png
>
> test-######.png becomes test-000001.png
>
> When the filename does not contain #, the suffix #### is added to the filename.
>
> The frame number will be added at the end of the filename, eg:
>
> blender -b animation.blend -o //render_ -F PNG -x 1 -a
>
> //render_ becomes //render_####, writing frames as //render_0001.png"

The Output panel page says the same in its own words:

> "When rendering an animation,
> the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png).
> You can set a custom padding size by adding the appropriate number of # anywhere in the file name
> (e.g. image_##_test.png translates to image_01_test.png)."

For the File Output node the composition is spelled out:

> "by default the output path is composed of:
> {base path}/{file name}{frame number}.{extension}."

and "Besides being split into two settings, in all other respects, this setting is treated the same as the render output path."

**What decides the end of the name.** A switch plus the chosen format:

> "File Extensions
>
> Adds the correct file extensions per file type to the output files."

> "use_file_extension
>
> Add the file format extensions to the rendered file name (eg: filename + .jpg)
>
> Type:
>
> boolean, default True"

> "file_extension
>
> The file extension used for saving renders
>
> Type:
>
> string, default "", (readonly, never None)"

The extension per format is tabulated in the manual — for example "PNG … .png", "BMP … .bmp", "Iris … .sgi .rgb .bw" on the image formats page, and "File Extension: .mkv", "File Extensions: .mp4, .mpg, .mpeg" and so on for containers on the video formats page. On the command line the same switch is `-x, --use-extension <bool>` — "Set option to add the file extension to the end of the file."

**The whole result, as one call.** The API exposes exactly this computation:

> "frame_path(*, frame=-2147483648, preview=False, view='')
>
> Return the absolute path to the filename to be written for a given frame
>
> Parameters:
>
> frame (int in [-inf, inf], (optional)) – Frame number to use, if unset the current frame will be used
>
> preview (boolean, (optional)) – Preview, Use preview range
>
> view (string, (optional, never None)) – View, The name of the view to use to replace the "%" chars
>
> Returns:
>
> File Path, The resulting filepath from the scenes render settings"

**Observed, from Cycles' own files.** Cycles relies on that call to reproduce Blender's naming rather than rebuilding it, which shows the call is the intended way to turn the recorded string into a real name. In `cycles_4.5/operators.py`, class `CYCLES_OT_denoise_animation`, lines 74–99:

```
        else:
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

Note the comment: "Denoise animation sequence with expanded frames matching Blender render output file naming." Source: `Blender Sources/cycles_4.5/operators.py`, lines 75–89. **Label: observed** (this is what the file says). The step beyond it — that `frame_path` therefore produces byte-for-byte the name a render writes — is **inferred**, and Cycles' own use of it plus `os.path.isfile` is the evidence.

**Label:** documented for the naming rules; observed for the Cycles file; inferred where marked.

**Not covered.** What name Blender gives a *movie* file — the manual never states how the frame range becomes part of a video's file name. I searched the manual and API for "0001-", "start-end", "frame range" near "file name", and found nothing. Nor does it say how Autosplit names the second and later pieces, only "This will automatically split the output into multiple files after the first file is 2GiB in size." It does not say what happens when the recorded string ends in a path separator (the panel screenshot shows `/tmp/`, and the `//render_` example shows a stem, but the rule joining them is not stated). It does not say what happens if `use_file_extension` is off and the name already ends in an extension.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The `#` rules and the extension switch are documented on the Command Line Arguments page itself, so they hold for a driven Blender. `frame_path` is a Python call whose entry names no window requirement. Label: documented for the command-line rules; inferred for use of `frame_path` with no window.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say. `frame_path` is described as returning "the absolute path", with no note on separators or drive letters. The related helper `bpy.path.native_pathsep(path)` — "Replace the path separator with the systems native os.sep" — exists but is not stated to be applied to render output.

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


## 9. Can the place a File Output node writes to be settled for a render — at the moment the render is started, and by writing it into the file from outside Blender?

**This question is ambiguous** and I am answering one reading while naming the other. I read "by writing it into the file from outside Blender" as: setting the value from outside Blender's window rather than by typing it into the node — that is, from a script or the command line. The other reading is: editing the .blend file itself with some tool that is not Blender. On that second reading the sources say nothing at all; there is no page about editing a blend-file's contents without Blender.

**(a) At the moment the render is started, from the command line: no dedicated argument.** I read the whole Command Line Arguments page. It carries exactly one output-path argument, `-o, --render-output <path>`, whose description is "Set the render path and file name" — the render path, not a node's. No argument on that page mentions nodes, the compositor, or File Output. Label: **not in the sources** that `-o` reaches a File Output node; I searched the page and the release notes for "node", "compositor" and "File Output" in connection with command-line arguments and found no such link.

**(b) From a script, which a render can be started with.** The node's path is an ordinary writable string property:

> "base_path
>
> Base output path for the image
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported, Supports template expressions)"

> "path
>
> Subpath used for this slot
>
> Type:
>
> string, default "", (never None, Supports template expressions)"

and Blender can be given a script before rendering: "-P, --python <filepath> — Run the given Python script file", "--python-expr <expression> — Run the given expression as a Python script", with "Arguments are executed in the order they are given."

Label: **inferred**. Two steps, both mine. First, the API marks read-only properties in so many words — `file_extension` is "(readonly)", `file_slots` is "(readonly)" — and `base_path` and `path` carry no such mark, so they can be set. Second, `-P` runs before a later `-f`/`-a` because arguments run in order. Neither step is stated for this node specifically.

**(c) Without any script, by what the path itself says.** The File Output node's paths support template expressions, so one saved node can land in different places depending on the blend-file it is in:

> "Currently this is only enabled on the Scene's render output path and the output
> paths of the compositor's File Output node."

and "//my_render_dir/{blend_name}.png" resolves with "{blend_name} getting replaced by the dance from dance.blend". Label: documented. Note the limit — the only variables are `blend_name`, `fps`, `resolution_x`, `resolution_y`, so what can be varied this way is fixed by that list.

**Not covered.** Whether `-o` leaves File Output nodes untouched or affects them in some way is not stated either way — I am reporting the absence of any statement, not an absence of effect. Nothing says how a File Output node's path combines with the scene's output path, if at all. Nothing says whether a node inside a linked or overridden node tree can have its path changed.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* That is the whole of this question. Python was the only outside route found, and that conclusion is inferred rather than stated. The effect of `-o` on File Output nodes and the reading that means editing the `.blend` with a tool other than Blender remain unestablished.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not in the sources. The command-line arguments page, the File Output node path entries and the path-template page quoted above give no operating-system condition for these routes.

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

**Answer.** **Not in the sources.** No page states which characters an operating system will refuse in a render output path, whether Blender checks for them, whether it replaces them, or what a render does when it meets one.

**Where I looked.** I searched the manual, the API documentation, the release notes and Cycles' files for: "invalid character", "illegal character", "forbidden character", "sanitize", "not allowed in file", "special character", "character … not supported", "non-ascii", "ascii", "utf-8", "utf8", "reserved". I read the Working Limits page, the File Paths (Path Templates) page in full, the Output properties page, the File Output Node page, the File Browser page and the whole of `bpy.path`.

**What the sources do carry, none of which answers the question:**

A helper exists that cleans a name, but it is a Python utility for a script to call, not something documented as applied to render paths:

> "bpy.path.clean_name(name, *, replace='_')
>
> Returns a name with characters replaced that
> may cause problems under various circumstances,
> such as writing to a file.
>
> All characters besides A-Z/a-z, 0-9 are replaced with "_"
> or the replace argument if defined."

Length is limited, and the limit is in bytes rather than letters:

> "directory: 767 … file-name: 255 … file-path: 1023"
>
> "Multi-byte encoding means some Unicode characters use more than a single ASCII character."

The one path failure Blender is documented to catch is its own syntax, not the computer's rules: "Paths that support templates can have template errors, which prevent the path from being processed", shown as a red field and the tooltip "Syntax error(s): - Invalid or incomplete template expression '{blend_name.png'" in `_images/files_file_paths-invalid_path_template_tooltip.png`. A `{` in a path is refused because Blender reserves it, not because a disk would.

Two API helpers acknowledge that computers differ over paths, without saying anything about refusal:

> "bpy.path.native_pathsep(path) — Replace the path separator with the systems native os.sep"

> "bpy.path.resolve_ncase(path) — Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path"

**Label:** not in the sources.

**Not covered.** Everything asked: whether the field accepts such characters, whether the check happens on typing or on writing, whether the frame is skipped or the whole render stops, and whether a template expression could introduce such a character after the field was checked.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not in the sources. The one error display documented is a red field and a tooltip, which need a window; what a background run does is not stated.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* Not in the sources — which is precisely the gap, since this is the question where the operating system is the whole point. The existence of `native_pathsep` and `resolve_ncase` shows Blender treats separators and case as differing between systems, but the sources do not extend that to which characters a name may hold.

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

2026-08-04 — Opus 5 (Claude Code) — Writing, Blender 4.5
