# Writing — Blender 5.1 — second pass

**Sources consulted, and when.** Blender 5.1 manual (`Blender Sources/manual_5.1`, including the picture files in its `_images` folder), Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`). All were read on **2026-07-30** (the snapshot date given to me), and that is the date read on every finding below.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

One thing I must declare rather than let stand silently, because the line above has to be exact. Before saving this document I ran a directory listing on the folder it was to be saved into, `Preparation/Blender Research/Findings/5.1/`, to check the folder existed. That listing showed me the file names of other findings documents sitting there. I opened none of them, and I read nothing but their names, sizes and dates. In particular I did not open the existing `Writing.md` for this version, which is the one that would have anchored this pass. I record it so the declaration above can be taken at face value: no file outside `Blender Sources/` and the question sheet was opened, but that one folder was listed.

This is a second pass. It answers questions **2, 3, 4, 5, 6, 7, 8 and 9** only. I did not open the existing findings document for this version. I started no helper agents, so there is nobody else to vouch for. Every search and every reading below is my own.

A note on how I worked, because this pass exists for the record of looking: for each question I searched the whole manual and the whole API documentation at once, by word, before opening anything. The searches are written out under each answer, in the words I actually searched for. Where I say the sources are silent, the searches that found nothing are listed too.

---

## 2. How is a place to write recorded in a file — what parts does the recorded thing have?

**Label: documented.**

A place to write is recorded as **one piece of text**, not as a structure with separate fields. Everything about where the file goes — the folder, the base name, the padding marks, any template expressions — lives inside that single string. The things that finish the name off (the format, and whether an extension is added) are recorded separately, as their own settings beside it.

There are two such strings for a render, in two different places.

**a) The scene's own output path.** It is a text setting on the scene's render settings:

> "filepath
>
> Directory/name to save animations, # characters define the position and padding of frame numbers (default "", never None, blend relative // prefix supported, Supports template expressions)
>
> Type:
>
> str"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html, read 2026-07-30.

That one sentence names four of the parts the string can hold: a directory part, a name part, `#` characters, and a `//` prefix. The render settings themselves hang off the scene:

> "render
>
> (readonly, never None)
>
> Type:
>
> RenderSettings"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.Scene.html, read 2026-07-30.

And the render settings are described as belonging to a scene:

> "Rendering settings for a Scene data-block"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html, read 2026-07-30.

The manual describes the same single field from the interface side:

> "File Path
>
> Choose the location to save rendered frames."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30.

**b) The File Output node's path.** This one is recorded as **two** strings plus one string per output item:

> "directory
>
> The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)"

> "file_name
>
> The base name of the file. Other information might be included in the final file name depending on the node options (default "", never None, Supports template expressions)"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html, read 2026-07-30.

Each of the node's outputs carries a name of its own, recorded on the item:

> "name
>
> (default "", never None)
>
> Type:
>
> str"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html, read 2026-07-30.

The manual says that item name is part of the recorded place, not decoration:

> "Lists all input sockets of the node.
> The name of each input socket is used to construct the final file name."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html, read 2026-07-30.

**c) What is recorded beside the string, not inside it.** The extension is not part of the recorded path. Whether one is added is its own switch:

> "use_file_extension
>
> Add the file format extensions to the rendered file name (eg: filename + .jpg) (default True)"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html, read 2026-07-30.

And what extension it would be comes from the format setting, recorded separately again:

> "file_format
>
> File format to save the rendered images as (default 'PNG')"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html, read 2026-07-30.

**Documented by picture.** The manual's screenshot of the Output panel (`_images/render_output_properties_output_panel.png`, shown on https://docs.blender.org/manual/en/5.1/render/output/properties/output.html) shows the recorded place as one wide text box, holding `/tmp/`, with a folder-browse button at its right end. Below it, as separate controls, are the checkboxes **Overwrite** (ticked), **Placeholders** (unticked), **File Extensions** (ticked), **Cache Result** (unticked), then **File Format: PNG**, **Color: BW / RGB / RGBA** with RGBA chosen, **Color Depth: 8 / 16** with 8 chosen, and **Compression 15%**. Read 2026-07-30. This confirms visually that the path is one field and the format and extension settings are neighbours of it, not parts of it.

The File Output node's picture (`_images/node-types_CompositorNodeOutputFile.webp`, shown on https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html) shows two stacked text boxes on the node, the upper holding `/tmp/` with a folder-browse button, the lower holding `file_name`. Read 2026-07-30.

**Not covered.** I did not find anything in the sources describing how the string is laid out inside the blend file itself — its length limit, its text encoding, or whether the folder and name parts are stored apart from each other on disk. The sources describe the setting as a program sees it, not as bytes in a file. I also found no statement of what separator character is stored when a path is typed on one operating system and the file opened on another.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The same. The path is a property of the scene's render settings and of the node, and the Python API documentation quoted above is the description of exactly those properties as reached from a script, with no window involved. Nothing in the API pages says any of these properties behave differently in background mode.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say. None of the pages quoted mention an operating system in connection with how the path is recorded.

**Where I looked.** Manual pages opened: `render/output/properties/output`, `compositing/types/output/file_output`, `files/file_paths`, `files/blend/open_save`, `editors/preferences/file_paths`, `render/output/introduction`, `render/output/animation`. API pages opened: `bpy.types.RenderSettings`, `bpy.types.CompositorNodeOutputFile`, `bpy.types.NodeCompositorFileOutputItem`, `bpy.types.ImageFormatSettings`, `bpy.types.Scene`, `bpy.path`. Searches run across the whole manual and whole API: `filepath`, `file_slots`, `layer_slots`, `NodeOutputFileSlot`, `output path`, `File Path`, `Directory`. The search for `file_slots`, `layer_slots` and `NodeOutputFileSlot` returned **no hits anywhere in the 5.1 API documentation**, which is why I describe the node's outputs as items with names rather than slots with paths.

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

Everything in the Linux sections of this document was run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. It is a rented container sharing somebody else's hardware, so it is given fewer cores than the chip really has and its system settings are not entirely its own; other work may have been running on it at the same time, so nothing here says how long anything took. Blender was started with no window unless an entry says otherwise, with its settings folder pointed at one of this topic's own and with the display variable unset so no window could be reached. The build used for this version was **Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)**. Date: 2026-08-06. In the output below, `…` stands for the working folder on that machine, which was `/root/smoozy/Writing/work/<entry>/5.1`. The full list these entries come from is `Audits/Blender Research/Writing/Hand tests — Linux.md`.

**One note on how the output is shown.** From 5.0 every line Blender prints while rendering carries a timestamp and a channel name in front of it, like `00:00.617  render           | Saved: '…'`. Where a quoted line below carries no such prefix, the prefix has been dropped because the timestamp differs from one run to the next; nothing after the bar is changed.

**These results settle Linux and say nothing about macOS.** Where the same test was also run on the Creator's Mac, both results are given and neither is folded into the other.

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

**Label: documented**, with one part **inferred** and flagged as such.

I found **five** kinds of mark. They are listed in the order the sources introduce them.

### 3.1 `//` at the start — "the folder this blend file is in"

> "This setting expands Relative Paths
> where a // prefix represents the directory of the current blend-file."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30.

The same mark, described where relative paths themselves are explained:

> "When relative paths are supported, the File Browser provides a Relative Path checkbox,
> when entering the path into a text field, use a double slash prefix (//) to make it so."

and

> "You cannot use relative paths into a new untitled blend-file.
> Save it before linking to external files."

— Manual, https://docs.blender.org/manual/en/5.1/files/blend/open_save.html, read 2026-07-30.

The command line documentation says the same of the output path given at start-up:

> "Use // at the start of the path to render relative to the blend-file."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30.

The API confirms it for both the scene path and the node's directory: both are described as "blend relative // prefix supported" (quoted in full under question 2).

### 3.2 `#` — the frame number, and how many digits it gets

> "When rendering an animation,
> the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png).
> You can set a custom padding size by adding the appropriate number of # anywhere in the file name
> (e.g. image_##_test.png translates to image_01_test.png)."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30.

Said again, with more examples, for the path given on the command line:

> "The # characters are replaced by the frame number, and used to define zero padding.
>
> animation_##_test.png becomes animation_01_test.png
>
> test-######.png becomes test-000001.png
>
> When the filename does not contain #, the suffix #### is added to the filename."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30.

And for the File Output node:

> "Both the File Name and the Image names (when saving multiple images) can contain #### to force the
> frame number to appear in a specific place in the filename. This also has the effect that a single render will
> have the frame number embedded. The frame number will be padded with zeros to fill the number of #
> characters."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html, read 2026-07-30.

### 3.3 `{` … `}` — template expressions, replaced when the path is used

This is the largest group. The manual gives it a page of its own.

> "Path templates substitute template expressions (written {abcd}, where
> "abcd" is a variable name) in a filepath when the path is used."

> "For example, if the currently open blend file is dance.blend and the render
> output path is set to this:
>
> //my_render_dir/{blend_name}.png
>
> Then when rendering, Blender will treat the path as:
>
> //my_render_dir/dance.png"

> "This substitution happens internally at time of use (e.g. while rendering), so
> the template syntax will remain as-is in the path field."

— Manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html, read 2026-07-30.

Where they work at all:

> "Currently path templates are only supported for the render output path in
> Scene properties and the output paths in the compositor's File Output
> node. More filepaths will support templates in future releases."

— Manual, same page, read 2026-07-30.

The names that may go inside the braces, quoted exactly:

> "Available in all paths that support template expressions:
>
> blend_name:
>
> The currently open blend file's name (without the .blend).
>
> blend_dir:
>
> Path up to (but not including) the currently open blend file.
>
> blend_name_lib:
>
> Like blend_name, except that for path properties on
> library-linked data-blocks it will be the name of the library blend file
> that the data-block comes from.
>
> blend_dir_lib:
>
> Like blend_dir, except that for path properties on
> library-linked data-blocks it will be the path to the library blend file
> that the data-block comes from.
>
> Available only in render output paths (including in the compositor File Output node):
>
> fps:
>
> The frames per second of the current scene.
>
> resolution_x / resolution_y:
>
> The x and y resolution of the rendered image.
> This factors in the resolution scale as well, so if the scene resolution is
> 1000x600 and the scale is 50%, then resolution_x and resolution_y
> will be 500 and 300, respectively.
>
> scene_name:
>
> Name of the current scene.
>
> camera_name:
>
> Name of the current render camera.
>
> Available only in path properties on a node:
>
> node_name:
>
> The name of the node that the path property is on."

— Manual, same page, read 2026-07-30.

Inside a template expression, a colon and further `#` marks have their own meaning — and note that this is a **different** meaning of `#` from the one in 3.2:

> "Format specifiers currently can only be used with variables that represent
> numerical values, not string values. The available format specifiers are:
>
> dance_{fps:###}.png: format as an integer with at least 3 digits.
>
> dance_{fps:.###}.png: format as a floating point number with exactly 3
> digits after the decimal point.
>
> dance_{fps:###.##}.png: format as a floating point number with at least 3
> digits for the integer part and exactly 2 digits for the fractional part.
>
> In all cases, the number of hash symbols (#) indicates the desired number of
> digits."

> "For example, if the fps is 29.97, then:
>
> dance_{fps:###}.png -> dance_030.png
>
> dance_{fps:.###}.png -> dance_29.970.png
>
> dance_{fps:###.##}.png -> -> dance_029.97.png
>
> Note that the values are properly rounded for the given number of digits."

— Manual, same page, read 2026-07-30.

### 3.4 `{{` and `}}` — how to mean a literal brace

> "Because { and } are used for template expressions in paths that support
> them, when a literal { or } is desired in such a path they must be
> escaped by writing them double:
>
> {{ translates to a single { in the final path.
>
> }} translates to a single } in the final path."

> "For example:
>
> my_weird}}_{{path.png -> my_weird}_{path.png
>
> //my_render_{{dir}}/{blend_name}.png -> //my_render_{dir}/dance.png
>
> //my_render_dir/{{{blend_name}}}.png -> //my_render_dir/{dance}.png"

— Manual, same page, read 2026-07-30.

### 3.5 `%` — the view name, in multi-view and stereo renders

This one appears only in the API documentation, in the description of the call that turns a recorded path into a real filename:

> "view (str) – View, The name of the view to use to replace the "%" chars (optional, never None)"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html (the `frame_path` function), read 2026-07-30.

**Inferred, and here is the step.** The quoted line says `%` characters in the path are replaced by a view name. It does not say anywhere in the sources that a person may type `%` into the Output panel's File Path field and get the same effect during an ordinary render; the only place `%` is described is as an argument to that one Python function. To claim it works in the field itself, a reader has to supply the assumption that `frame_path` reads the same string the renderer does. So: **that `%` marks a spot for the view name is documented for `frame_path`; that it does so for a render started any other way is inferred.**

### 3.6 `/` and `\` inside a File Output node's item name — folder separators

> "When the Media Type is set to Image, input names are also used to construct
> subdirectories:
>
> If an input name contains a slash character (/ or \),
> the path is interpreted as a directory hierarchy.
>
> Any missing directories are created automatically during output.
>
> This allows organizing outputs into folders directly from the node setup,
> for example: passes/diffuse or masks/object_01."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html, read 2026-07-30.

The release notes state this as a change made in this version:

> "The **File Output** node now allows specifying directories in item names. This was possible in
> older versions of Blender and the functionality is now restored."

— Release notes, https://developer.blender.org/docs/release_notes/5.1/compositor/, read 2026-07-30.

### 3.7 What a badly formed mark does

> "Paths that support templates can have template errors, which prevent the
> path from being processed."

> "For example, in the following path:
>
> //my_render_dir/{blend_name.png
>
> The expression {blend_name isn't properly closed, which will result in an
> error."

> "When there are template errors in a path, the path field will be highlighted red
> in the UI"

> "Depending on the path, template errors may prevent certain actions. For
> example, if there are errors in the render output path, then rendering an
> animation will fail with an error message indicating the path errors."

— Manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html, read 2026-07-30.

**Documented by picture.** The manual's tooltip screenshot on that page (`_images/files_file_paths-invalid_path_template_tooltip.png`) shows the Output panel's path field filled with `//my_render_dir/{blend_name.png`, the whole field drawn in red, and a tooltip beneath it reading:

> "Output Path
> Directory/name to save animations, # characters define the position and padding of frame numbers.
> Value: //my_render_dir/{blend_name.png
> Syntax error(s):
>  - Invalid or incomplete template expression '{blend_name.png'."

Read 2026-07-30. What is visible: the field turns red, the tooltip is headed "Output Path", it repeats the value, and it lists the errors under the heading "Syntax error(s)". I do not read anything further into it.

**Not covered.** I found no statement about `~` for a home folder, no statement that environment variables are expanded inside an output path, and no statement about `..` inside a path. See the record of looking below — those searches came back empty, so I cannot say whether such marks are taken literally or not; the sources simply do not address them. I also cannot say from the sources what happens if `#` marks appear in more than one place in the same name, nor what happens when a template expression produces text containing a slash.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* `//`, `#` and `{}` all hold. The command line page is itself an outside-the-window source and documents `//`, `#` and `{}` for the path given with `-o`: "You can use path templating features such as {blend_name} in the path." (Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30). The red-field highlighting in 3.7 is explicitly an interface thing — the manual says "the path field will be highlighted red in the UI" — so that part of the behaviour has no equivalent outside the window; the failure it warns about is described separately and is not tied to the interface.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say, for any of these marks. The one place an operating system is mentioned near this subject is the File Output node's item names, where both `/` and `\` are named as separators without tying either to a system.

**Where I looked.** Searched the whole manual for `####` and for `# ` in path context; opened every hit (`advanced/command_line/render`, `advanced/command_line/arguments`, `modeling/modifiers/physics/ocean`, `compositing/types/output/file_output`, `contribute/manual/guides/markup_guide`). Searched the whole manual and API for: `template`, `{blend_name}`, `path template`, `escape`, `relative path`, `// prefix`, `%`, `% chars`, `suffix`, `environment variable`, `~/`, `home director`, `${`. The search for `%` as a path mark returned exactly one hit in the whole of the API documentation and none in the manual. The searches for `~` as a home folder and for environment variables inside output paths returned only unrelated hits (`--python-use-system-env`, `--factory-startup`, the file browser's "home directory" bookmark, and an extensions-listing template that uses `${body}`), none of them about render output paths. Opened `files/file_paths` in full, `files/blend/open_save` (Relative Paths section), `editors/preferences/file_paths`, `editors/file_browser`, `bpy.path` in full. Opened the two template-error pictures and the two panel pictures.

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

**Label: documented for one case; not in the sources for the other. This is a split answer and the split matters.**

**The one case that is documented.** For the File Output node, when an output item's name contains a slash, the folders that name implies are made:

> "If an input name contains a slash character (/ or \),
> the path is interpreted as a directory hierarchy.
>
> Any missing directories are created automatically during output."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html, read 2026-07-30.

Note precisely what that covers: the folders implied by the **item name**, on a node whose Media Type is Image. It is written under the node's "Inputs" heading, immediately after "When the Media Type is set to Image, input names are also used to construct subdirectories".

**The case that is not in the sources.** Whether Blender makes the folder named in the **scene's own output path** — the File Path field in the Output panel, or the path given with `-o` — when that folder does not exist. Nothing in the manual, the API documentation, the release notes or Cycles' files says either way. Nor does anything say whether the node's own **Directory** field is created when missing; the sentence above is about item names, not about the Directory field.

The nearest thing I found is about a different part of Blender entirely — the file browser, where a person types a path by hand:

> "If you type a nonexistent path, you will be prompted to create it."

— Manual, https://docs.blender.org/manual/en/5.1/editors/file_browser.html, read 2026-07-30.

That is the file browser prompting a person. It is not a render writing frames, and I do not carry it across.

**Not covered.** Everything above about the scene output path. Also not covered: whether folder-making, where it does happen, makes a whole chain of missing folders or only the last one — the wording "Any missing directories" suggests more than one but does not say a chain is made from an arbitrary depth; and whether folder-making happens before the render starts or at the moment the first file is written.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sentence about the File Output node is written without reference to the window, and the node writes "during rendering or compositing" (Manual, same page). Nothing says it differs outside. For the scene output path, since the sources do not say what happens at all, they equally do not say whether it differs outside.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say.

**Where I looked.** Searched the whole manual, the whole API documentation, the whole release notes and all of Cycles' files, one pattern at a time, for: `mkdir`, `makedirs`, `create the folder`, `folder is created`, `directory is created`, `directories are created`, `creates directories`, `missing directories`, `parent director`, `path will be created`, `ensure the director`, `will be created`, `are created`, `is created`, `does not exist`, `doesn't exist`, `non-existent`, `nonexistent`, `not exist`, `automatically`, `auto-create`, and `folder` within the render output and compositor output sections.

Of all of that, exactly **two** lines bore on the question. One is the File Output node line quoted above (found by `missing directories`, `directories are created` and `automatically`). The other is the file browser line quoted above (found by `nonexistent`). Every other hit was about something else entirely — vertex groups, bone collections, node groups, asset catalogs, extension repositories, bundle paths in geometry nodes, grease pencil strokes. I read the surrounding text of each hit inside `render/`, `files/` and `compositing/` to be sure. `mkdir` and `makedirs` returned no hits anywhere, including in Cycles' Python files.

I also read the whole of `render/output/properties/output`, `render/output/animation`, `render/output/introduction`, `advanced/command_line/arguments`, `advanced/command_line/render` and `files/file_paths` line by line rather than by search, because those are where such a statement would most naturally sit. None of them mentions folder creation.

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

**Label: not in the sources.**

None of the four sources says what happens on a failed write. Not for a missing folder, not for a full disk, not for a folder that cannot be written to. The manual describes what the output settings *are*; it does not describe what the writing *does* when the writing does not work.

The single nearby thing the sources do say is about a different kind of failure — a badly formed template expression, which is a fault in the path text rather than in the disk:

> "Depending on the path, template errors may prevent certain actions. For
> example, if there are errors in the render output path, then rendering an
> animation will fail with an error message indicating the path errors."

— Manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html, read 2026-07-30.

That is the only documented statement I found in the whole of 5.1's manual of a render failing because of its output path, and it is about the shape of the text, not about the disk.

One more thing exists in the sources that is adjacent but is not an answer. Cycles' own files contain a case of a write that could not happen and what the code does about it — but it is about compiling a shader script, not about writing a rendered frame:

> `                report({'ERROR'}, rpt_("Failed to write .oso file next to external .osl file at {:s}").format(dst_path))`

— Cycles' own files, `Blender Sources/cycles_5.1/osl.py`, line 216. Read 2026-07-30. **Observed** in the file. I record it only to show the search was done and to be explicit that it is not about rendered output. Reading anything about frame writing from it would be unsupported.

**Not covered.** Everything the question asks. I cannot say whether a render stops, skips the frame, carries on, leaves a part-written file, prints a message, or ends with a particular exit status. Any of those would be invention.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say. I note only that the one adjacent statement above is worded without reference to the window ("rendering an animation will fail with an error message"), and that the command line documentation offers `--log-level` with an `error` setting — "error: Errors only" (Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30) — which shows errors can be surfaced outside the window in general. It says nothing about write failures specifically, and I draw no conclusion from it.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say.

**Where I looked.** Searched the whole manual, API documentation, release notes and Cycles' files, pattern by pattern, for: `cannot write`, `unable to write`, `failed to write`, `write error`, `save error`, `could not open`, `could not save`, `no space`, `disk full`, `out of space`, `out of disk`, `permission denied`, `write permission`, `writable`, `not writable`, `read-only`, `skips the frame`, `partial file`, `incomplete file`, `corrupt`, `will fail`, `fails with an error`, `error message`, `aborts`, `cancels`.

Results: `cannot write`, `unable to write`, `write error`, `save error`, `could not open`, `disk full`, `out of space`, `skips the frame`, `partial file` and `incomplete file` returned **no hits at all** in any of the four sources. `failed to write` returned the single Cycles shader line quoted above. `read-only` and `writable` returned hits only about extension repositories, add-on directories, user preferences, linked data-blocks and read-only RNA properties. `error message` returned hits about GPU drivers, EEVEE shadow memory, bone parenting, Python, and startup troubleshooting. `will fail` returned the template-error line quoted above plus hits about NLA strips, surface duplication, motion tracking, and opening newer blend files in older Blender.

I also opened, in full or in the relevant part: the whole `troubleshooting/` section listing (`crash`, `recover`, `startup`, `python`, `network`, `report_bug`, `3d_view`, and the GPU pages) — none of them concerns output writing; `render/output/properties/output`; `render/output/animation`; `advanced/command_line/arguments`; `advanced/command_line/render`; `files/file_paths`; and Cycles' `operators.py`, `osl.py`, `engine.py`, `properties.py`, `__init__.py`, `ui.py`, `camera.py`, `presets.py`, `version_update.py` by search for `filepath`, `directory`, `os.path`, `makedirs`, `output`.

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

**Label: documented,** with the writing of two of them **inferred** and flagged.

The sources name the following ways. I give what each one writes, and where the source says so.

### 6.1 Render Image, from the Render menu

> "Render Image F12
>
> Render the active scene at the current frame."

— Manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html, read 2026-07-30.

**What it writes: nothing, by itself.**

> "While animation renders are automatically saved, still renders are not.
> These have to be saved manually."

— Manual, https://docs.blender.org/manual/en/5.1/editors/image/editing.html, read 2026-07-30.

### 6.2 Render Animation, from the Render menu

> "Render Animation Ctrl-F12
>
> Render the animation of the active scene."

— Manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html, read 2026-07-30.

**What it writes: a file per frame, or a video, at the output path, without being asked.**

> "While rendering stills will allow you to view and save the image from the render buffer when
> it is complete, animations are a series of images, or frames,
> and are automatically saved directly out to a drive after being rendered."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html, read 2026-07-30.

Which of the two shapes it takes is the output setting:

> "Frame Sequence
>
> The Frame Sequence is a much more stable approach,
> where you set your output format to a still format (such as JPG, PNG or a multi-layer format).
> Click Animation to render your scene out to a set of images,
> where each image is a frame in the sequence."

> "Blender creates a file for each frame of the animation."

and

> "The Direct Approach, which is highly not recommended and not a standard practice,
> is where you set your output format to an AVI or MOV format,
> and click Animation to render your scene directly out to a movie file.
> Blender creates one file that holds all the frames of your animation."

— Manual, same page, read 2026-07-30.

### 6.3 Render Sequencer Image and Render Sequencer Animation, from the Render menu

> "Render Sequencer Image Alt-F12
>
> If a Sequencer Scene exists that differs from the active scene, render that
> scene instead.
>
> Render Sequencer Animation Ctrl-Alt-F12
>
> Render the animation of the Sequencer Scene."

— Manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html, read 2026-07-30.

**What they write:** the sources do not describe their writing separately from 6.1 and 6.2. **Inferred**, and here is the step: they are described as rendering "that scene instead" and "the animation of the Sequencer Scene", so a reader has to supply the assumption that the still one behaves like Render Image (writes nothing) and the animation one like Render Animation (writes automatically). The sources do not state it. I flag it rather than assert it.

### 6.4 Sequence Render Image and Sequence Render Animation, from the Video Sequencer

> "Sequence Render Image
>
> Show the current frame preview as a Render Result where you can save it as an image file.
>
> Sequence Render Animation
>
> Save previews of the frames in the scene range (or the preview range, if active) to a video file
> or a series of image files. See the Output panel for details."

— Manual, https://docs.blender.org/manual/en/5.1/editors/video_sequencer/sequencer/navigating.html, read 2026-07-30. (The identical wording also appears at https://docs.blender.org/manual/en/5.1/editors/video_sequencer/preview/header.html.)

**What they write:** the first writes nothing until saved by hand ("where you can save it"); the second writes to the output path, either a video or a series of images. Both are documented in the words above. What they render is not the same thing as a full render:

> "Sequence Render Image and Sequence Render Animation don't render the final video by default –
> specifically, they don't render Scene Strips, instead using the preview's
> shading mode (which is initially Solid)."

— Manual, same page, read 2026-07-30.

### 6.5 Viewport render, from the 3D Viewport's View menu

> "Render a Still Image
>
> To render a still image, use 3D Viewport ‣ View ‣ Render Viewport Preview.
>
> Render an Animation
>
> To render an animation, use 3D Viewport ‣ View ‣ Render Playblast.
>
> Render Keyframes
>
> To render an animation, but only those frames that have a keyframe, use
> 3D Viewport ‣ View ‣ Render Playblast on Keyframes. This only renders those frames for which
> the selected objects have an animation key. The other frames are still written to the output, but will simply
> repeat the last-rendered frame."

— Manual, https://docs.blender.org/manual/en/5.1/editors/3dview/viewport_render.html, read 2026-07-30.

**What it writes:** it uses the same output settings —

> "Additionally, some output settings are used too:
>
> Resolution
>
> Aspect
>
> Output path
>
> File format"

— Manual, same page, read 2026-07-30.

The keyframes-only variant is explicit that every frame in the range still gets written: "The other frames are still written to the output, but will simply repeat the last-rendered frame." A limit worth recording:

> "Viewport rendering only works for the Workbench and EEVEE render engines.
> It's not supported for Cycles."

— Manual, same page, read 2026-07-30.

### 6.6 From the command line, one frame

> "-f, --render-frame <frame>
>
> Render frame <frame> and save it.
>
> +<frame> start frame relative, -<frame> end frame relative.
>
> A comma separated list of frames can also be used (no spaces).
>
> A range of frames can be expressed using .. separator between the first and last frames (inclusive)."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30.

**What it writes:** the frame, saved. Note the difference from 6.1 — the words are "Render frame `<frame>` **and save it**". This is the same still render that is not saved from the window, but from the command line the saving is part of what the argument does.

An example the manual gives:

> "blender -b file.blend -o /project/renders/frame_##### -F OPEN_EXR -f -2"
>
> "-o /project/renders/frame_#####
>
> Path of where to save the rendered image, using five padded zeros for the frame number."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html, read 2026-07-30.

### 6.7 From the command line, an animation

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30.

> "blender -b file.blend -a
>
> -a
>
> Render the whole animation using all the settings saved in the blend-file."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html, read 2026-07-30.

**What it writes:** the whole frame range, to the output path, by the rule in 6.2.

A caution the manual gives twice, which decides *where* it writes:

> "Arguments are executed in the order they are given!
>
> The following command will not work, since the output and extension are set after Blender is told to render:
>
> blender -b file.blend -a -x 1 -o //render
>
> The following command will behave as expected:
>
> blender -b file.blend -x 1 -o //render -a
>
> Always position -f or -a as the last arguments."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html, read 2026-07-30.

### 6.8 From Python

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)
>
> Undocumented, consider contributing."

> "animation (bool) – Animation, Render files from the animation range of this scene (optional)
>
> write_still (bool) – Write Image, Save the rendered image to the output path (used only when animation is disabled) (optional)"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.ops.render.html, read 2026-07-30.

**What it writes:** with `animation=True`, "files from the animation range"; with `animation=False`, nothing unless `write_still=True`, which saves "to the output path". Note the operator's own description is literally the word "Undocumented" in 5.1 — I quote it rather than fill the gap.

There is also the viewport one:

> "bpy.ops.render.opengl(*, animation=False, render_keyed_only=False, sequencer=False, write_still=False, view_context=True)
>
> Take a snapshot of the active viewport"

> "render_keyed_only (bool) – Render Keyframes Only, Render only those frames where selected objects have a key in their animation data. Only used when rendering animation (optional)
>
> write_still (bool) – Write Image, Save the rendered image to the output path (used only when animation is disabled) (optional)
>
> view_context (bool) – View Context, Use the current 3D view for rendering, else use scene settings (optional)"

— Python API documentation, same page, read 2026-07-30.

### 6.9 What writes alongside any of these

Two things write whether or not the render itself does.

The File Output node:

> "The File Output node writes one or more images to disk during rendering or compositing.
> Each connected input is saved to the specified directory and filename,
> with the frame number appended to create a sequence."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html, read 2026-07-30.

And the render cache, when switched on:

> "Saving – Cache Result
>
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image."
>
> "The image is stored in the Render Cache folder as specified in the
> File Paths Preferences."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30.

### 6.10 One thing that is a render but writes sound

> "Render Audio
>
> Mix the scene's audio to a sound file."

— Manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html, read 2026-07-30.

> "Audio can be rendered from the Render Menu."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/audio/introduction.html, read 2026-07-30.

Its path is asked for at the time, not taken from the output path:

> "filepath (str) – File Path, Path to file (optional, never None)"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.ops.sound.html (the `mixdown` operator), read 2026-07-30.

**Not covered.** Whether 6.3 writes automatically (flagged as inferred above). Whether the `-c, --command` mechanism can start a render — the sources document only extension management under it, so I cannot say it can, and I did not find any statement that it cannot. What the writing does when it fails (see question 5). Whether these ways differ in *what* they write beyond image versus video — for instance whether a viewport render writes the same passes as a full render.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* 6.6, 6.7 and 6.8 are the outside-the-window ways and are documented as such — the command line page and the Python operators exist for exactly that. The manual states the case for it plainly:

> "One advantage of using the command line is that we do not need a graphical display
> (no need for X server on Linux for example)
> and consequently we can render via a remote shell (typically SSH)."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html, read 2026-07-30.

The menu items in 6.1–6.5 have no meaning outside a window. The important difference for anything driven from outside: a still started with `-f` **is** saved, whereas the same still started with F12 is not.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say for any of these. The one operating-system remark I found near the command line concerns launching, not rendering — the manual has separate launching pages for Linux, macOS and Windows (`advanced/command_line/launch/`), and `-con, --start-console` is marked "(Windows only)" (Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30).

**Where I looked.** Searched the whole manual for `render image`, `render animation`, `viewport render`, `Render Result`, `write_still`, `playblast`. Searched the whole API for `bpy.ops.render`, `mixdown`. Opened in full: `interface/window_system/topbar` (Render Menu section), `render/output/animation`, `render/output/introduction`, `editors/3dview/viewport_render`, `editors/video_sequencer/sequencer/navigating`, `editors/video_sequencer/preview/header`, `editors/image/editing`, `advanced/command_line/arguments`, `advanced/command_line/render`, `advanced/command_line/extension_arguments`, `render/output/audio/introduction`, `bpy.ops.render`, `bpy.ops.sound`. Searched the release notes for `playblast` and `viewport render` — hits were bug-fix entries only, nothing that changes the list above.

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

**Reading of the question.** I read it as: a render that is not set up to write anything to disk — a still render started from the window, where the manual says nothing is saved. **The question is ambiguous**: it could also be read as asking what a render leaves behind on a machine when every writing feature is switched off, including temporary files. I answer the first reading, and add what the sources say about the second at the end so nothing is lost.

**Label: documented.**

Such a render can be set up. The manual says so:

> "While animation renders are automatically saved, still renders are not.
> These have to be saved manually."

— Manual, https://docs.blender.org/manual/en/5.1/editors/image/editing.html, read 2026-07-30.

**What it leaves behind is an image held inside Blender, called the Render Result.** It is a data-block, and it can be looked at in the Image editor:

> "In addition to images, the data-block selector includes the following items:
>
> Render Result: displays renders. When this item is selected, the Slot,
> View Layer, and Render Pass selectors become available (see below)."

— Manual, https://docs.blender.org/manual/en/5.1/editors/image/introduction.html, read 2026-07-30.

That it is a data-block, in so many words:

> "These images are the "Render Result" and "Viewer" image data-blocks,
> and the files saved directly to disk with the Render Animation operator."

— Manual, https://docs.blender.org/manual/en/5.1/render/color_management/displays_views.html, read 2026-07-30.

It is held in one of several **slots**, and more than one render can be left behind at once:

> "Slot
>
> The render slot to view (and render to). You can create new renders without
> losing previous ones by selecting an empty slot before rendering. Afterwards,
> you compare them by pressing J and Alt-J to cycle forwards and backwards.
> Alternatively, you can use the number keys 1, 2, 3 etc.
> to select the slot with the corresponding number."

— Manual, https://docs.blender.org/manual/en/5.1/editors/image/introduction.html, read 2026-07-30.

> "Render Slot Cycle Next/Previous J/Alt-J
>
> Switch to the next/previous render slot (that contains a render)."

— Manual, https://docs.blender.org/manual/en/5.1/editors/image/navigating.html, read 2026-07-30.

The manual's own word for where a still sits before it is saved is "buffer":

> "While rendering stills will allow you to view and save the image from the render buffer when
> it is complete"

— Manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html, read 2026-07-30.

The Render Result also carries metadata, which can be looked at:

> "Show Metadata
>
> Displays metadata about the selected Render Result. See the Output tab's
> Metadata panel to change what metadata to include."

— Manual, https://docs.blender.org/manual/en/5.1/editors/image/navigating.html, read 2026-07-30.

**What it does not leave behind, unless asked.** The cache file is off by default, and only exists if switched on:

> "Note that Blender doesn't create these cache files by default. You
> have to enable Cache Result
> in the scene's Output options and then render it at least once."

— Manual, https://docs.blender.org/manual/en/5.1/editors/image/editing.html, read 2026-07-30.

When it is switched on, the Render Result survives shutting Blender down, because a file was written after all:

> "Find the render cache file for the current scene and load it into the
> Render Result. This way, you can restore the last render from a previous
> Blender session and continue working in the Compositor without having to
> render the scene again."

— Manual, same page, read 2026-07-30.

**On the second reading — temporary files.** The manual says the temporary directory holds render layers among other things:

> "The temporary directory is used to store various files at run-time
> (including render layers, physics cache, copy-paste buffer and crash logs)."

> "The temporary directory is selected based on the following priority:
>
> User Preference (see File Paths).
>
> Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms).
>
> The /tmp/ directory."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html, read 2026-07-30.

I record that as the sources' only statement bearing on the second reading. It does not say a render leaves files there afterwards, only that the directory is used at run-time; claiming that files remain when the render ends would be a step the sources do not support.

**Not covered.** How long the Render Result lasts — the sources do not say whether it survives loading another file, and, more to the point here, whether anything at all is left behind when Blender is run without a window and then exits. Nor do the sources say how many slots there are, or what happens to slots when Blender closes.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* This is the weak point of the answer, and I will not paper over it. Everything quoted above is about the Image editor — a window. A Render Result is a data-block, and the data-block exists whether or not there is a window to show it in; that much follows from it being called a data-block. But the sources nowhere describe what a still render leaves behind when Blender is run in background mode and then exits, and I will not guess. What the sources do make clear is the practical consequence for anything driven from outside: a still started with `-f` on the command line is documented as "Render frame `<frame>` **and save it**" (Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30), so the situation this question describes — a render that writes nothing — is the window's situation, not the command line's.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say, except for the temporary directory, where the manual does distinguish: "TEMP on Windows, TMP & TMP_DIR on other platforms", and names `/tmp/` as the last resort.

**Where I looked.** Searched the whole manual for `Render Result`, `render slot`, `render buffer`, `not saved`, `saved manually`, `save the image`, `Save As`, `placeholder`, `Cache Result`, `in memory`. Opened `editors/image/introduction`, `editors/image/editing`, `editors/image/navigating`, `render/color_management/displays_views`, `render/output/animation`, `render/output/properties/output`, `advanced/blender_directory_layout`, `editors/preferences/file_paths`. Searched the API for `use_render_cache`, `render_cache_directory`, `temporary_directory` and read each. Searched the whole manual and API for `in memory` and `buffer` in this context — the only relevant hit was the "render buffer" line quoted above.

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

**Label: documented.**

Four things happen to the recorded string, in this order as the sources describe them.

**Step one — template expressions are replaced.**

> "This substitution happens internally at time of use (e.g. while rendering), so
> the template syntax will remain as-is in the path field."

— Manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html, read 2026-07-30.

**Step two — a leading `//` becomes the blend file's folder.**

> "This setting expands Relative Paths
> where a // prefix represents the directory of the current blend-file."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30.

**Step three — the frame number goes in. This is the part the question asks about, and the answer has two cases.**

If the recorded name contains `#`, the number goes **where the `#` marks are**, padded to as many digits as there are marks:

> "You can set a custom padding size by adding the appropriate number of # anywhere in the file name
> (e.g. image_##_test.png translates to image_01_test.png)."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30.

> "The # characters are replaced by the frame number, and used to define zero padding.
>
> animation_##_test.png becomes animation_01_test.png
>
> test-######.png becomes test-000001.png"

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30.

If it contains no `#`, the number goes **at the end of the name**, four digits wide:

> "When rendering an animation,
> the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png)."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30.

The command line page says the same, and says it as a rule about the marks rather than about the number:

> "When the filename does not contain #, the suffix #### is added to the filename.
>
> The frame number will be added at the end of the filename, eg:
>
> blender -b animation.blend -o //render_ -F PNG -x 1 -a
>
> //render_ becomes //render_####, writing frames as //render_0001.png"

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30.

That last example is the clearest single statement of the whole chain: `//render_` becomes `//render_####`, and then a frame of it is written as `//render_0001.png`.

**Step four — what decides the end of the name.** The extension. It is added, or not, by a switch:

> "Saving – File Extensions
>
> Adds the correct file extensions per file type to the output files."

— Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30.

> "use_file_extension
>
> Add the file format extensions to the rendered file name (eg: filename + .jpg) (default True)"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html, read 2026-07-30.

From the command line the same switch is:

> "-x, --use-extension <bool>
>
> Set option to add the file extension to the end of the file."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30.

*Which* extension is decided by the chosen format:

> "file_format
>
> File format to save the rendered images as (default 'PNG')"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html, read 2026-07-30.

And the manual lists the extension each format carries — for example "JPEG … .jpg .jpeg", "OpenEXR … .exr", "PNG … .png", "WebP … .webp", "AVIF … .avif" (Manual, https://docs.blender.org/manual/en/5.1/files/media/image_formats.html, read 2026-07-30), and for video containers "MPEG-4 … File Extensions: .mp4, .mpg, .mpeg", "Matroska … File Extension: .mkv", "WebM … File Extension: .webm", "AVI … File Extension: .avi" (Manual, https://docs.blender.org/manual/en/5.1/files/media/video_formats.html, read 2026-07-30).

**The finished name can be asked for directly.** There is a call whose entire job is to perform this whole conversion and hand back the answer:

> "frame_path(*, frame=-2147483648, preview=False, view='')
>
> Return the absolute path to the filename to be written for a given frame"
>
> "frame (int) – Frame number to use, if unset the current frame will be used (in [-inf, inf], optional)
>
> preview (bool) – Preview, Use preview range (optional)
>
> view (str) – View, The name of the view to use to replace the "%" chars (optional, never None)"
>
> "Returns:
>
> File Path, The resulting filepath from the scenes render settings (never None)"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html, read 2026-07-30.

**Observed** in Cycles' own files: Cycles uses exactly that call to work out the name a render would have written, by putting a path into the setting and reading the finished name back out. The comment on it says what it is doing:

> ```
>             # Denoise animation sequence with expanded frames matching
>             # Blender render output file naming.
>             in_filepath = scene.render.filepath
> ```
> ```
>             for frame in range(scene.frame_start, scene.frame_end + 1):
>                 scene.render.filepath = in_filepath
>                 filepath = scene.render.frame_path(frame=frame)
>                 in_filepaths.append(filepath)
>
>                 if not os.path.isfile(filepath):
>                     scene.render.filepath = original_filepath
>                     err_msg = tip_("Frame '%s' not found, animation must be complete") % filepath
> ```

— Cycles' own files, `Blender Sources/cycles_5.1/operators.py`, lines 70–86, inside `CYCLES_OT_denoise_animation.execute`. Read 2026-07-30.

**Label for the Cycles finding: observed.** What is observed is what the file says: Cycles sets `scene.render.filepath`, calls `scene.render.frame_path(frame=frame)` for each frame in the scene range, and treats the result as the name of the file that a render would have produced. Going from that to any claim about what a render actually writes to disk would be a step beyond the file, and I do not take it.

**For the File Output node the shape of the name is given explicitly:**

> "Each connected input is saved to the specified directory and filename,
> with the frame number appended to create a sequence.
> For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

— Manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html, read 2026-07-30.

So on that node the frame number sits after the socket name and before the extension — unless `#` marks say otherwise:

> "Both the File Name and the Image names (when saving multiple images) can contain #### to force the
> frame number to appear in a specific place in the filename."

— Manual, same page, read 2026-07-30.

The node also shows the finished result before anything is written:

> "Output Paths"
>
> "Displays the final file path and extension for each input/layer, based on current settings.
> This helps verify that outputs are named and organized correctly."

— Manual, same page, read 2026-07-30.

**Not covered.** Several edges.

- **Video output.** For a single video file there is no per-frame name, and the sources do not say what name it gets — in particular they do not say whether a frame range is put into the name. I searched for this specifically and found nothing (see the record below). The one thing the sources do say about video naming is that one file can become several: "If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size." (Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30). How the split files are told apart in their names is not stated.
- **Views.** When a render has several views, the API says each view carries a suffix: "file_suffix — Suffix added to the render images for this view (default "", never None)" and "camera_suffix — Suffix to identify the cameras to use, and added to the render images for this view (default "", never None)" (Python API documentation, https://docs.blender.org/api/5.1/bpy.types.SceneRenderView.html, read 2026-07-30), and the manual says "By default the system saves each view as an individual file, thus generating as many files as views to be rendered" (Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/stereoscopy/usage.html, read 2026-07-30). Where in the name that suffix lands relative to the frame number and the extension is **not stated anywhere I could find**.
- **Order.** I have given the four steps in the order the sources introduce them, not in an order the sources state. No source says template replacement happens before `#` replacement, or the reverse.
- **Collisions.** What happens when two frames would be given the same name — for instance when a `#` run is too short for the frame number — is not stated.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* It holds. Everything in step three is stated on the command line page as well as in the panel page, in the same terms, and the worked example `//render_ becomes //render_####, writing frames as //render_0001.png` is itself a command line example. The extension switch has a command line form, `-x`. The `frame_path` call is a Python call and needs no window.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say. Nothing on any of these pages ties the naming to a system. I note only, without carrying it across, that a neighbouring utility exists for a system difference in paths generally: "bpy.path.resolve_ncase(path) — Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path." (Python API documentation, https://docs.blender.org/api/5.1/bpy.path.html, read 2026-07-30). That is about finding an existing path, not about naming one to write, and I draw nothing from it here.

**Where I looked.** Searched the whole manual and API for: `appended`, `is appended`, `added to the file name`, `suffix`, `frame number`, `padding`, `padded`, `extension`, `use_file_extension`, `frame_path`, `####`, and for four-digit-range patterns such as `0001-0250` that would show a frame range inside a video name — that last returned no relevant hits anywhere. Opened in full: `render/output/properties/output`, `advanced/command_line/arguments`, `advanced/command_line/render`, `files/file_paths`, `files/media/image_formats`, `files/media/video_formats`, `compositing/types/output/file_output`, `render/output/properties/stereoscopy/usage`, `render/output/properties/stereoscopy/introduction`, `bpy.types.RenderSettings`, `bpy.types.ImageFormatSettings`, `bpy.types.SceneRenderView`, `bpy.path`. Searched Cycles' files for `filepath`, `frame_path`, `os.path` and read `operators.py` around every hit.

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

**Reading of the question.** I read the two halves as: (a) can the node's output place be decided at the moment a render is kicked off, without having changed the blend file beforehand; and (b) can it be decided by something other than a person working in Blender's window, writing the value into the file. **This is ambiguous** — "writing it into the file from outside Blender" could mean editing the blend file with some other program entirely. I answer the reading above, and say plainly what the other reading would need at the end.

**Label: split. Part is documented, part is not in the sources.**

### (a) At the moment the render is started

**There is no command line argument for it.** `-o, --render-output <path>` sets one thing, and the manual says which:

> "-o, --render-output <path>
>
> Set the render path and file name.
> Use // at the start of the path to render relative to the blend-file."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30.

"The render path" is the scene's output path — the same thing the API calls `filepath` and describes as "Directory/name to save animations". Nothing on the command line arguments page mentions the compositor, a node, or a File Output node. I read the whole page and searched it; there is no such argument. **Label for that: not in the sources**, in the strong sense — the page that would carry such an argument is complete, is titled "Command Line Arguments", and does not carry one.

**There is a Python way, and it is documented as writable.** The node's two path properties are plain text properties with no readonly marking:

> "directory
>
> The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)
>
> Type:
>
> str"

> "file_name
>
> The base name of the file. Other information might be included in the final file name depending on the node options (default "", never None, Supports template expressions)
>
> Type:
>
> str"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html, read 2026-07-30.

Compare with the same page's properties that *are* fixed, which say so: "file_output_items — (default None, readonly)" and "format — (readonly)". The two path properties carry no such marking.

Python can be handed to Blender at the moment of starting, in three forms:

> "-P, --python <filepath>
>
> Run the given Python script file.
>
> --python-text <name>
>
> Run the given Python script text block.
>
> --python-expr <expression>
>
> Run the given expression as a Python script.
>
> The expression may be a complete multi-line script;
> you are limited only by the platform's maximum argument length."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30.

And the order in which such things take effect is spelled out:

> "Arguments are executed in the order they are given. eg:
>
> blender --background test.blend --render-frame 1 --render-output "/tmp"
>
> …will not render to /tmp because --render-frame 1 renders before the output path is set.
>
> blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set.
>
> blender --background test.blend --render-output /tmp --render-frame 1
>
> …works as expected."

— Manual, same page, read 2026-07-30.

**Label for the Python route: inferred, and here is the step.** The sources document (i) that the node's `directory` and `file_name` are writable text properties, and (ii) that a Python script or expression can be run at start-up in a stated order relative to loading the file and rendering. Putting those two together to say "therefore the node's place can be set at the moment a render is started" is a step the reader has to take; no page says it in one breath. It is a short step, but it is a step, so the label is inferred and not documented.

**One thing that genuinely settles part (a) without any inference.** Some of the place *is* decided at the moment of the render rather than beforehand, no matter who set it, because template expressions are resolved then:

> "Currently path templates are only supported for the render output path in
> Scene properties and the output paths in the compositor's File Output
> node."

> "This substitution happens internally at time of use (e.g. while rendering), so
> the template syntax will remain as-is in the path field."

— Manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html, read 2026-07-30.

So a node whose directory reads `//{scene_name}/` writes somewhere that depends on the scene at the moment of the render, without anything being changed for that render. The variables available in a node's path include one that exists nowhere else:

> "Available only in path properties on a node:
>
> node_name:
>
> The name of the node that the path property is on."

— Manual, same page, read 2026-07-30.

**Label for that: documented.**

### (b) By writing it into the file from outside Blender

**Not in the sources.** Nothing in the manual, the API documentation, the release notes or Cycles' files describes reading or changing a node's output path in a blend file by any means other than Blender itself. There is no description anywhere of the blend file's internal layout for these properties, and no tool named for editing one from outside.

What the sources do offer instead is the round trip through Blender without a window: load the file, change the property, save it. The pieces are all documented separately —`-b, --background` ("Run in background (often used for UI-less rendering)"), `-P, --python <filepath>`, and the writable properties above, all on pages already quoted — but no page puts them together for this purpose, and I will not assemble a claim the sources do not make.

**Not covered.** Whether a node's path can be driven or animated (the node inherits `driver_add` and `keyframe_insert` from `bpy_struct` per https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html, but nothing says those work on a text property, and I do not claim they do). Whether the per-item names — which also shape the place, per question 3.6 — can be set the same way; the item's `name` property is documented as writable at https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html, but again no page says it can be set at render time. Whether any of this holds when the node lives in a linked library rather than the file being rendered.

**Outside the interface.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* This whole question is about outside the interface, and the answer is: there is no dedicated switch, and the general mechanism (Python at start-up) is documented to exist but is nowhere documented as being used for this. The asymmetry with the scene's own output path is the thing worth carrying away: the scene path has `-o`, the node path has nothing of the kind.

**Operating system.** *(Partly settled by a hand test on 2026-08-05 — the up-to-date line is at the end of this answer.)* The sources do not say. The only nearby operating-system remark is about how much Python can be passed on one line — "you are limited only by the platform's maximum argument length" (Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30) — which acknowledges a platform difference in size without naming any platform.

**Where I looked.** Read `advanced/command_line/arguments` in full, all 692 lines, looking for any argument touching the compositor or a node; there is none. Read `advanced/command_line/render` in full. Read `advanced/command_line/extension_arguments` to check whether the `-c, --command` mechanism reaches rendering — every subcommand documented there is extension management (`list`, `sync`, `update`, `install`, `install-file`, `remove`, `repo-list`, `repo-add`, `repo-remove`). Read `bpy.types.CompositorNodeOutputFile` and `bpy.types.NodeCompositorFileOutputItem` in full. Read `files/file_paths` in full. Searched the whole manual and API for `file output`, `output path`, `render-output`, `--python`, `background`. Searched the whole release notes for `File Output`, `path template`, `output path`, `filepath`, `render output` — the only 5.1 entries touching the node are the two quoted in question 3.6 and this one: "The **Image** and **File Output** nodes now read and write custom meta-data." (Release notes, https://developer.blender.org/docs/release_notes/5.1/compositor/, read 2026-07-30), which is about metadata, not about where files go.

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

2026-08-04 — Opus 5 (Claude Code) — Writing, Blender 5.1
