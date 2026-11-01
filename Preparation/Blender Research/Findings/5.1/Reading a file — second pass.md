# Reading a file — Blender 5.1 — second pass

Sources consulted, all read on **2026-07-30** (the snapshot date given in my inputs):
Blender 5.1 manual (including its `_images` folder), Blender 5.1 Python API documentation,
Blender 5.1 release notes, Cycles' own files for 5.1, and the undated requirements page
snapshot `requirements_2026-08-01.txt`.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

This pass answers questions **2 to 8** only, as instructed. For each answer I have written
down where I looked, what I searched for, and what I ruled out.

---

## 2. How many Blenders can be at work on one computer at the same time, and does one reading a file while another renders cause trouble?

**Short answer.** More than one Blender can run on one computer, and the documentation says
so outright — but it never states a maximum number, and it says nothing at all about what
happens when one Blender reads a file that another Blender is rendering.

### 2a. How many

**Label: documented** (that more than one is possible)
**Label: not in the sources** (for any maximum number)

The API documentation, on the page about using Blender as a Python module, states:

> "Only a single .blend file can be edited at a time."

and immediately after, under a Hint:

> "As with the application it's possible to start multiple instances, each with their own bpy and therefore Blender state. Python provides the multiprocessing module to make communicating with sub-processes more convenient."

Source: Python API documentation, https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html
Date read: 2026-07-30

The same recommendation appears on the threading page:

> "For running Python code independently of Blender, it is recommended to use the multiprocessing module."

Source: Python API documentation, https://docs.blender.org/api/5.1/info_gotchas_threading.html
Date read: 2026-07-30

The glossary defines what one of these is:

> "The timespan of a Blender instance. The session begins with starting an instance of Blender and ends with closing it."

Source: manual, https://docs.blender.org/manual/en/5.1/glossary/index.html
Date read: 2026-07-30

No page anywhere in the three documentation sets gives a number, a limit, a licence
restriction, or any advice on how many may sensibly run at once.

The nearest thing to a statement about many Blenders working together is a hint about many
*machines*, not many Blenders on one machine:

> "An easy way to get multiple machines to share the rendering workload is to:
> Set up a shared directory over a network file system.
> Disable Overwrite, enable Placeholders in the Render Output panel.
> Start as many machines as you wish rendering to that directory."

Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
Date read: 2026-07-30

### 2b. Does one reading while another renders cause trouble?

**Label: not in the sources**

Nothing in the manual, the API documentation, the release notes or Cycles' files says
whether two Blenders on one computer interfere with each other — no file locking, no
"file is in use", no warning about opening a file another Blender has open, no statement
about safety or lack of it.

Two things the sources *do* say that touch the edges of it, neither of which answers the
question:

- All instances share one temporary directory unless told otherwise. "The temporary
  directory is used to store various files at run-time (including render layers, physics
  cache, copy-paste buffer and crash logs)." and it is chosen by "User Preference … 
  Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms). The /tmp/
  directory."
  Source: manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html
  Date read: 2026-07-30
  The sources do not say whether two instances sharing that directory collide.

- Within one Blender, Python threads are unsupported: "In short: Python threads cause
  Blender to crash in hard to diagnose ways. For example, a crash can occur while rendering
  with Cycles, with Python drivers, while a background thread is used to download some file."
  Source: Python API documentation, https://docs.blender.org/api/5.1/info_gotchas_threading.html
  Date read: 2026-07-30
  This is about threads inside one Blender, not about two separate Blenders.

**Where I looked.** I searched the whole of `manual_5.1`, `api_5.1`, `release_notes_5.1`
and `cycles_5.1` for: *instance*, *instances of Blender*, *another Blender*, *multiple
instances*, *several instances*, *single instance*, *already running*, *at the same time*,
*same time*, *simultaneous*, *concurrent*, *parallel*, *multiprocessing*, *sub-process*,
*separate processes*, *another process*, *process ID*, *PID*, *file lock*, *locking*,
*lock file*, *flock*, *atomic*, *data race*, *read-only*, *in use by*, *opened by*, *same
file*, *open the same file*, *while another*, *twice*, *two copies*, *one copy*, *corrupt*,
*render farm*, *shared directory*, *network file system*, *NFS*, *file system*. I read in
full: the command-line pages (index, arguments, render, and the Linux, macOS and Windows
launch pages), the Blender directory layout page, the File Paths preferences page, the data
recovery page, the Cycles GPU rendering page's question-and-answer section, and the API
pages on threading, operators and Blender-as-a-module. I ruled out the Cycles files: a grep
of `engine.py`, `__init__.py` and `operators.py` for *lock*, *instance*, *process* and
*path* turned up only file-path handling in the denoise operator, nothing about
concurrency.

**Not covered.** Whether two Blenders reading the same `.blend` is safe; whether one
writing while another reads is safe; whether they contend for GPU memory, the temporary
directory, the autosave file, or the preferences file; any practical ceiling from RAM or
cores. None of this is in the sources.

**Outside the interface.** The "multiple instances" statement is made on a page about
running Blender without its window (as a Python module), and it says this is "As with the
application" — so it is claimed for both. Background running is documented separately:
`-b, --background` "Run in background (often used for UI-less rendering)."
(manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html).
One documented difference for the module form: "Functionality controlled by command line
arguments (shown by calling `blender --help`) isn't accessible", and "Blender's typical
signal handlers are not initialized, so there is no special handling for Control-C to
cancel a render and a crash log is not written in the event of a crash."
(https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html)

**Operating system.** Nothing in the sources makes the instance count OS-dependent. Two
OS-dependent details sit nearby: the temporary directory comes from `TEMP` on Windows and
`TMP`/`TMP_DIR` elsewhere, falling back to `/tmp/`; and the autosave location is
"`./autosave` — Autosave blend-file location. (Windows only, temp directory used for other
systems.)"
(manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html).
The requirements snapshot names the supported systems as Windows, macOS and Linux, and says
"Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0
and later require Apple Silicon running macOS 13 (Ventura) or later."
(Blender Sources/requirements/requirements_2026-08-01.txt; this page carries no
version-specific address.) That sentence names 4.5 and 5.0. It does not name 5.1, so
reading it as covering 5.1 is **inferred**, not documented — the step is that "5.0 and
later" includes 5.1.

> **Changed by the citation check, 2026-08-05.** This paragraph originally said the sentence
> "covers 5.1 by naming '5.0 and later'", which let an inferred step stand as documented. The
> requirements page has no version in its address and speaks for a version only where it names
> that version in so many words; it names 4.5 and 5.0. What 5.1's own requirements are has gone
> back for a fresh research pass.

---

## 3. Can a single read of a file tell, for every view layer, whether that layer renders at any frame later asked about?

**I read the question as:** having loaded the file once, can you determine for every view
layer in it whether that layer will be rendered at any frame you might later name? The
other reading — "does the file, read once, list the layers that render *at all*" — is
answered along the way and I mark where.

**Label: inferred**, with a documented core and a real gap at the end.

**Documented core.** Every view layer carries a stored on/off switch for rendering, and it
can be read for every layer, not just the active one.

> "Use for Rendering
> The active view layer will be used during rendering."

Source: manual, https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html
Date read: 2026-07-30

> "use
> Enable or disable rendering of this View Layer (default True)
> Type: bool"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.ViewLayer.html
Date read: 2026-07-30

All layers of all scenes are reachable: `Scene.view_layers` is a collection of `ViewLayer`
("(default None, readonly) Type: ViewLayers[ViewLayer]",
https://docs.blender.org/api/5.1/bpy.types.Scene.html), and every scene must have at least
one — "Each scene must contain at least one View Layer."
(https://docs.blender.org/manual/en/5.1/render/layers/introduction.html).

**The complication: the switch can be animated.** The View Layer panel screenshot in the
manual shows "Use for Rendering" as a ticked checkbox with a small dot immediately to its
right, and "Render Single Layer" as an unticked checkbox with the same dot.
Image: `_images/render_layers_view-layer_panel.png`, shown on
https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html (label: **documented**,
it is part of the manual). The manual explains what that dot is:

> "Decorators are small buttons that appear to the right of other buttons and show the state of the property. Decorators may appear next to number fields, menus, and checkboxes to indicate the property can be animated."

Source: manual, https://docs.blender.org/manual/en/5.1/interface/controls/buttons/decorators.html
Date read: 2026-07-30

*The step I took:* the decorator page says a decorator next to a checkbox means the
property can be animated; the View Layer screenshot shows a decorator next to "Use for
Rendering"; therefore "Use for Rendering" can be animated. A reader has to put those two
together, so this is **inferred**, not documented.

**What that means for a single read.** If the switch is animated by keyframes, the curve is
stored in the file and can be evaluated for any frame without re-reading the file:
`FCurve.data_path` is the "RNA Path to property affected by F-Curve", and
`FCurve.evaluate(frame)` will "Evaluate F-Curve at given frame … Returns: Value, Value of
F-Curve specific frame"
(https://docs.blender.org/api/5.1/bpy.types.FCurve.html). Alternatively `Scene.frame_set`
will "Set scene frame updating all objects and view layers immediately"
(https://docs.blender.org/api/5.1/bpy.types.Scene.html), after which the layer's `use` can
be read again. Either way, one load of the file is enough for the keyframed case.
Label for this paragraph: **documented** for what the two calls do; **inferred** for the
conclusion that one load therefore suffices.

**Where it stops being answerable.** Three things the sources do not settle:

- The sources nowhere state that Blender re-reads a view layer's `use` flag per frame while
  rendering an animation. So whether animating the flag actually changes what is rendered
  mid-sequence is **not in the sources**.
- The flag can also be driven rather than keyframed — "If a property is being driven by
  another, the decorator shows the driver icon."
  (https://docs.blender.org/manual/en/5.1/interface/controls/buttons/decorators.html) — and
  a driver may be a Python expression, whose result a plain read of stored values cannot
  predict. Automatic Python execution is off by default:
  "-Y, --disable-autoexec — Disable automatic Python script execution (Python-drivers &
  startup scripts), (default)."
  (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html)
- Scripts can change the flag at render time. The API documents handlers
  `bpy.app.handlers.frame_change_pre`, `frame_change_post`, `render_pre`, `render_init` and
  `render_write` (https://docs.blender.org/api/5.1/bpy.app.handlers.html). Nothing read
  before rendering can foresee what such a handler will do.

**A second switch that also decides this, and behaves differently outside the window.**

> "use_single_layer
> Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
Date read: 2026-07-30

The manual says the same in fewer words: "Render Single Layer — Only render the active view
layer." with "Note: This option is ignored when rendering from the command line."
(https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html)

**Not covered.** Whether the renderer honours a per-frame change to `use`; what happens with
a Python-expression driver on it; whether a layer with `use` off still consumes time or
produces a pass; anything about how a file's *other* scenes' layers behave when only one
scene is rendered.

**Outside the interface.** The reading side works the same: `Scene.view_layers`,
`ViewLayer.use`, `FCurve.evaluate` and `Scene.frame_set` are plain data and method access
with no window requirement stated. One documented difference matters here and points the
other way: `use_single_layer` is "ignored for rendering from command line", so a file that
looks like it renders one layer inside the window renders all enabled layers outside it.
A second, from the command-line arguments page:
"--disable-depsgraph-on-file-load — Background mode: Do not systematically build and
evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (-b or
-c options). … NOTE: this is a temporary option, in the future depsgraph will never be
automatically generated on file load in background mode."
(https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html) — so what is
evaluated on load in background mode is already different, and is stated to be changing.

**Operating system.** The sources say nothing that makes any of this OS-dependent.

**Where I looked.** Searched all of `manual_5.1` and `api_5.1` for *Use for Rendering*,
*use for rendering*, *Enable or disable rendering*, *Render Single Layer*,
*use_single_layer*, *view layer*, *animatable*, *can be animated*, *cannot be animated*,
*any property*, *most properties*, *decorator*, *animate property*. Read in full:
`render/layers/view_layer.txt`, `render/layers/introduction.txt`,
`interface/controls/buttons/decorators.txt`, `animation/keyframes/introduction.txt`,
`animation/drivers/index.txt`, `animation/drivers/introduction.txt`, the "View Layer
Attribute Lookup" section of `animation/drivers/workflow_examples.txt` (it turned out to be
about the material Attribute node, not about animating a layer's render switch), and the
API pages `bpy.types.ViewLayer`, `bpy.types.RenderSettings`, `bpy.types.Scene`,
`bpy.types.FCurve`, `bpy.types.bpy_struct` and `bpy.app.handlers`. Opened the images
`render_layers_view-layer_panel.png` and `render_layers_introduction_list.png`. Ruled out
`manual_5.1/render/layers/filter.txt`, `passes.txt` and `override.txt` — they cover what a
layer contains, not whether it renders. Ruled out the Freestyle view-layer pages entirely
(line-style settings). Ruled out the Cycles files: `CyclesRenderLayerSettings` in
`properties.py` holds pass toggles, not a render on/off switch.

---

## 4. Can the places a file is set up to write to be read out of it?

**Label: documented** — with one documented catch that stops a plain read from being the
final word.

**Yes.** The output destinations are ordinary readable properties.

The scene's render output path:

> "filepath
> Directory/name to save animations, # characters define the position and padding of frame numbers (default "", never None, blend relative // prefix supported, Supports template expressions)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
Date read: 2026-07-30

And the manual's description of the same field: "File Path — Choose the location to save
rendered frames." … "This setting expands Relative Paths where a // prefix represents the
directory of the current blend-file."
(https://docs.blender.org/manual/en/5.1/render/output/properties/output.html)

The compositor's File Output node:

> "directory
> The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)"

> "file_name
> The base name of the file. Other information might be included in the final file name depending on the node options (default "", never None, Supports template expressions)"

> "file_output_items
> (default None, readonly)
> Type: NodeCompositorFileOutputItems[NodeCompositorFileOutputItem]"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html
Date read: 2026-07-30

Each item carries the name that becomes part of the filename — `NodeCompositorFileOutputItem`
has `name` ("(default "", never None)") and `format`
(https://docs.blender.org/api/5.1/bpy.types.NodeCompositorFileOutputItem.html). The manual
describes how they combine:

> "Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence. For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

and

> "If an input name contains a slash character (/ or \), the path is interpreted as a directory hierarchy. Any missing directories are created automatically during output."

Source: manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
Date read: 2026-07-30

The release notes confirm that last behaviour is present in 5.1: "The **File Output** node
now allows specifying directories in item names. This was possible in older versions of
Blender and the functionality is now restored."
(release notes, https://developer.blender.org/docs/release_notes/5.1/compositor/)

Other write destinations recorded in the file and readable the same way (all quoted from
the Python API documentation, read 2026-07-30):

- `BakeSettings.filepath` — "Image filepath to use when saving externally"
  (https://docs.blender.org/api/5.1/bpy.types.BakeSettings.html)
- `PointCacheItem.filepath` — "Cache file path"
  (https://docs.blender.org/api/5.1/bpy.types.PointCacheItem.html)
- `CollectionExport.filepath` — "The file path used for exporting"
  (https://docs.blender.org/api/5.1/bpy.types.CollectionExport.html)
- `StripProxy.filepath` — "Location of custom proxy file", and its `directory`
  "Location to store the proxy files"
  (https://docs.blender.org/api/5.1/bpy.types.StripProxy.html)

**The catch: what you read may not be the path that gets written to.** Two documented
reasons.

First, templates. The output path may contain expressions that are only resolved at the
moment of use:

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

and

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."

Source: manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html
Date read: 2026-07-30

The variables include `blend_name`, `blend_dir`, `blend_name_lib`, `blend_dir_lib`, and —
for render output paths only — `fps`, `resolution_x`, `resolution_y`, `scene_name`,
`camera_name`, plus `node_name` for path properties on a node (same page). A template can
also be malformed: "Paths that support templates can have template errors, which prevent
the path from being processed." and "if there are errors in the render output path, then
rendering an animation will fail with an error message indicating the path errors."

Second, the command line can override the path after the file is read: `-o,
--render-output <path>` "Set the render path and file name."
(https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html), and the same
page warns that order matters — "blender --background --render-output /tmp test.blend
--render-frame 1 …will not render to /tmp because loading the blend-file overwrites the
render output that was set."

**One tool that resolves it for you.** `RenderSettings.frame_path(frame=…, preview=…,
view=…)` — "Return the absolute path to the filename to be written for a given frame …
Returns: File Path, The resulting filepath from the scenes render settings"
(https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html). Cycles' own files use it
exactly this way, which is a worked example of reading a file's output destination frame by
frame:

> ```
>             for frame in range(scene.frame_start, scene.frame_end + 1):
>                 scene.render.filepath = in_filepath
>                 filepath = scene.render.frame_path(frame=frame)
>                 in_filepaths.append(filepath)
> ```

Source: Cycles' own files, `cycles_5.1/operators.py`, lines 79–82, inside
`CYCLES_OT_denoise_animation.execute`. Label: **observed** (this is what the file says).
Date read: 2026-07-30
*The step beyond:* that this actually yields the path a render would write is what the API
text says `frame_path` does; that the code runs correctly is not something I checked.

**Not covered.** Whether `frame_path` expands template expressions (the API text for it
does not say). Whether the File Output node's final per-item paths — the manual says the
"Output Paths" panel "Displays the final file path and extension for each input/layer" —
are exposed to a reader anywhere other than that panel; I found no property for them.
Nothing here settles what happens when two outputs resolve to the same file.

**One place that is *not* in the file.** The render cache destination is a preference, not
file data: "The image is stored in the Render Cache folder as specified in the File Paths
Preferences."
(https://docs.blender.org/manual/en/5.1/render/output/properties/output.html), and
"Render Cache — The location where cached render images are stored."
(https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html).

**Outside the interface.** All of the above are data properties and one method; nothing in
their documentation requires a window. The command-line override `-o` is only available
outside the window, and it is documented to be overwritten by loading a blend-file if given
in the wrong order (both quotes above). The File Output node's "Output Paths" panel is a
sidebar panel, so that particular display is inside the interface only.

**Operating system.** Nothing in the sources makes any of this OS-dependent. Paths
themselves carry a general warning: "Blender's relative file paths are not compatible with
standard Python modules such as sys and os. Built-in Python functions don't understand
Blender's // prefix which denotes the blend-file path."
(https://docs.blender.org/api/5.1/info_gotchas_file_paths_and_encoding.html)

**Where I looked.** Searched all of `api_5.1` for properties named `filepath` and for
`base_path`, `file_slots`, `media_type`, `frame_path`, `directory`, `file_name`; searched
`manual_5.1` for *File Path*, *output path*, *path template*, *Output Paths*, *Render
Cache*, *Placeholders*, *Overwrite*. Read in full:
`render/output/properties/output.txt`, `files/file_paths.txt`,
`compositing/types/output/file_output.txt`, `advanced/command_line/arguments.txt`,
`advanced/command_line/render.txt`, `editors/preferences/file_paths.txt`, and the API pages
for `RenderSettings`, `CompositorNodeOutputFile`, `NodeCompositorFileOutputItem`,
`BakeSettings`, `PointCacheItem`, `CollectionExport`, `StripProxy`. Confirmed by grep that
`base_path` and `file_slots` appear nowhere in the 5.1 documentation, so the File Output
node's paths in 5.1 are `directory` + `file_name` + item names. Read
`cycles_5.1/operators.py` in the region that handles output paths.

---

## 5. Can the locations a file records for the things it points at be read out and written back, and by what?

**Label: documented**

**Yes, both ways.** The recorded location of each external thing is a plain property on the
data-block that points at it, and none of them is marked read-only. All quotes below are
from the Python API documentation, read 2026-07-30.

> "filepath
> Image/Movie file name (default "", never None, blend relative // prefix supported)"
> — and beside it —
> "filepath_raw
> Image/Movie file name (without data refreshing) (default "", never None, blend relative // prefix supported)"

https://docs.blender.org/api/5.1/bpy.types.Image.html

> "filepath
> Path to the library .blend file (default "", never None, blend relative // prefix supported)"

https://docs.blender.org/api/5.1/bpy.types.Library.html

> "filepath
> Sound sample file used by this Sound data-block (default "", never None, blend relative // prefix supported)"

https://docs.blender.org/api/5.1/bpy.types.Sound.html

> "filepath
> Filename of the movie or sequence file (default "", never None, blend relative // prefix supported)"

https://docs.blender.org/api/5.1/bpy.types.MovieClip.html

The same property, worded for the kind of data, appears on `CacheFile` ("Path to external
displacements file"), `VectorFont`, `Text`, `Volume`, `MultiresModifier`,
`MeshCacheModifier`, `ShaderNodeTexIES`, `ShaderNodeScript` and `CacheFileLayer`.

*The step for "written back":* the API documentation marks properties that cannot be
written, e.g. on the very same `Library` page — "is_archive … (default False, readonly)",
"packed_file (readonly)", "parent (readonly)", "version … readonly". `filepath` on those
pages carries no such mark. Concluding from the absence of the mark that it is writable is
a step the reader must take, so this half is **inferred**, not documented. The API also
offers a direct check at run time: `bpy_struct.is_property_readonly(property)` — "Check if
a property is readonly. … Returns: True when the property is readonly (not writable)."
(https://docs.blender.org/api/5.1/bpy.types.bpy_struct.html)

### By what

**Reading the whole list at once.**

> "bpy.utils.blend_paths(*, absolute=False, packed=False, local=False)
> Returns a list of paths to external files referenced by the loaded .blend file.
> Parameters:
> absolute (bool) – When true the paths returned are made absolute.
> packed (bool) – When true include file paths for packed data.
> local (bool) – When true skip linked library paths."

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.utils.html
Date read: 2026-07-30

Note the wording — "the loaded .blend file". The file has to be open in a Blender for this.

**Turning a recorded location into a usable one.** `bpy.path.abspath(path, *, start=None,
library=None)` and `bpy.path.relpath(path, *, start=None)`
(https://docs.blender.org/api/5.1/bpy.path.html), with this warning about linked data:

> "When using Blender data from linked libraries there is an unfortunate complication since the path will be relative to the library rather than the open blend-file. When the data block may be from an external blend-file pass the library argument from the bpy.types.ID.
> >>> bpy.path.abspath(image.filepath, library=image.library)"

Source: Python API documentation, https://docs.blender.org/api/5.1/info_gotchas_file_paths_and_encoding.html
Date read: 2026-07-30

**Rewriting them wholesale.** Four operators, quoted from
https://docs.blender.org/api/5.1/bpy.ops.file.html (read 2026-07-30):

- `bpy.ops.file.make_paths_absolute()` — "Make all paths to external files absolute"
- `bpy.ops.file.make_paths_relative()` — "Make all paths to external files relative to current .blend"
- `bpy.ops.file.find_missing_files(*, find_all=False, directory='', …)` — "Try to find missing external files"
- `bpy.ops.file.report_missing_files()` — "Report all missing external files"

and from the manual's File menu (https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html):
"Make Paths Relative — Make all paths to external files relative to the current
blend-file." / "Make Paths Absolute — Make all paths to external files absolute (= full
path from the system's root)."

**Rewriting one library's location.** `bpy.ops.wm.lib_relocate(*, library='', filepath='',
directory='', filename='', …)` and `bpy.ops.wm.lib_reload(...)`
(https://docs.blender.org/api/5.1/bpy.ops.wm.html); `bpy.ops.outliner.lib_relocate()`
(https://docs.blender.org/api/5.1/bpy.ops.outliner.html). The manual:

> "When the Outliner is in the Blender File Display Mode, a right-click on a library (a linked blend-file) will show the Relocate option. It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed), or to switch to a variation of the same data in a different file."

Source: manual, https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html
Date read: 2026-07-30

**Rewriting them as a side effect of saving.** `bpy.ops.wm.save_as_mainfile(…,
relative_remap=True, …)` — "relative_remap (bool) – Remap Relative, Remap relative paths
when saving to a different directory"
(https://docs.blender.org/api/5.1/bpy.ops.wm.html), described in the manual as "Remap
Relative — This option remaps Relative Paths (such as linked libraries and images) when
saving a file in a new location."
(https://docs.blender.org/manual/en/5.1/files/blend/open_save.html).
And when writing a library out: `bpy.data.libraries.write(filepath, datablocks, *,
path_remap='NONE', …)` with "NONE No path manipulation (default). RELATIVE Remap paths that
are already relative to the new location. RELATIVE_ALL Remap all paths to be relative to
the new location. ABSOLUTE Make all paths absolute on writing."
(https://docs.blender.org/api/5.1/bpy.types.BlendDataLibraries.html)

The 5.1 release notes record a fix in this area: "Bpy.data.libraries.write with
path_remap='RELATIVE_ALL' does not recalculate existing relative paths"
(release notes, https://developer.blender.org/docs/release_notes/5.1/bugfixes/).

**Making the change stick.** The manual says of paths generally: "Relative paths are the
default but this can be changed in the File tab of the Preferences." and "You cannot use
relative paths into a new untitled blend-file. Save it before linking to external files."
(https://docs.blender.org/manual/en/5.1/files/blend/open_save.html). Nothing in the sources
says a rewritten path reaches the file on disk without saving it — that step is implied by
the whole open/save model but never stated, so I do not claim it.

**Not covered.** Whether any of these can be done without loading the file into a Blender
(the API's own wording — "the loaded .blend file" — suggests not, but does not say it).
Whether every kind of external reference is covered by "all paths to external files".
Whether a written path is validated. Nothing here settles what happens to packed data,
beyond the `packed=True` option on `blend_paths`.

**Outside the interface.** The properties and `bpy.utils.blend_paths` are plain data access
with no window requirement stated. `make_paths_absolute()` and `make_paths_relative()` take
no arguments at all, and `find_missing_files` takes `directory` as a parameter, so all three
can in principle be called with arguments rather than through a file browser — but the
sources do not say whether they run without a window, and the API's own operator page warns
generally: "Certain operators in Blender are only intended for use in a specific context …
Operators' poll function can fail where an API function would raise an exception giving
details on exactly why."
(https://docs.blender.org/api/5.1/info_gotchas_operators.html). So: **not in the sources**
for whether these particular operators work in background mode.

**Operating system.** Nothing OS-specific about the properties or the operators. One
encoding caveat applies everywhere: "Paths are an exception to this rule since the existence
of non-UTF-8 paths on the user's file system cannot be ignored." and the advice to "Use
os.fsencode() or os.fsdecode() instead of built-in string decoding functions when operating
on paths."
(https://docs.blender.org/api/5.1/info_gotchas_file_paths_and_encoding.html)

**Where I looked.** Grepped all of `api_5.1` for properties beginning `filepath` and for
*blend_paths*, *abspath*, *relpath*, *path_remap*, *relative_remap*, *lib_relocate*,
*lib_reload*, *make_paths_absolute*, *make_paths_relative*. Grepped `manual_5.1` for
*Relative Paths*, *Remap Relative*, *Make Paths*, *External Data*, *Relocate*, *packed*.
Read in full: `files/file_paths.txt`, `files/blend/open_save.txt` (paths section),
`files/blend/packed_data.txt`, `files/linked_libraries/link_append.txt` (relocate and
broken-library sections), `interface/window_system/topbar.txt` (External Data menu), and
API pages `bpy.utils`, `bpy.path`, `info_gotchas_file_paths_and_encoding`,
`info_gotchas_operators`, `bpy.types.Library`, `Image`, `Sound`, `MovieClip`, `CacheFile`,
`VectorFont`, `BlendDataLibraries`, `bpy_struct`, `bpy.ops.file`, `bpy.ops.wm`,
`bpy.ops.outliner`.

---

## 6. Can anything be asked to report which of a file's references are missing, to search for them, and to rewrite the recorded locations?

**Label: documented** — all three, by name.

**Report.**

> "bpy.ops.file.report_missing_files()
> Report all missing external files"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.ops.file.html
Date read: 2026-07-30

> "Report Missing Files
> This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files."

Source: manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html
Date read: 2026-07-30

**Search.**

> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', hide_props_region=True, check_existing=False, filter_blender=False, … filemode=9, display_type='DEFAULT', sort_method='')
> Try to find missing external files
> Parameters:
> find_all (bool) – Find All, Find all files in the search path (not just missing) (optional)
> directory (str) – Directory, Directory of the file (optional, never None)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.ops.file.html
Date read: 2026-07-30

> "Find Missing Files
> In case you have broken links in a blend-file, this can help you to fix the problem. A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it, recursively in all contained directories. Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative."

with the note:

> "Recovered files might need to be reloaded. You can do that one by one, or you can save the blend-file and reload it again, so that all external files are reloaded at once."

Source: manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html
Date read: 2026-07-30

**Rewrite.** `bpy.ops.file.make_paths_absolute()` — "Make all paths to external files
absolute" — and `bpy.ops.file.make_paths_relative()` — "Make all paths to external files
relative to current .blend" (https://docs.blender.org/api/5.1/bpy.ops.file.html). For a
single library, `bpy.ops.wm.lib_relocate` / `bpy.ops.outliner.lib_relocate`, quoted under
question 5. For paths of things linked from a library, the whole-file remapping options of
`bpy.data.libraries.write(path_remap=…)` and `save_as_mainfile(relative_remap=…)`, also
quoted under question 5.

**A related reading of "which are missing", for linked data specifically.** Every data-block
carries a flag:

> "is_missing
> This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore) (default False, readonly)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.ID.html
Date read: 2026-07-30

So missing *linked data* can be found by reading a flag, without running any operator. The
sources do not offer an equivalent flag for a missing image or sound file; for those, the
report operator is what is documented.

The 5.1 release notes record one fix in this area: "External Multires .btx filepath not
cleared when re-packing, reports as missing even when empty"
(release notes, https://developer.blender.org/docs/release_notes/5.1/bugfixes/).

**Not covered.** How the report is delivered to something other than a person — the manual
says it appears "in the Info editor's header", and nothing in the sources describes a
machine-readable list of the missing ones. Whether `find_missing_files` can be given a
directory and run without its file browser appearing. Whether the search is safe on very
large directory trees. What "recovered" means when two files of the same name are found.

**Outside the interface.** `report_missing_files()` and the two `make_paths_*` operators
take no parameters, and `find_missing_files` takes `directory` as a parameter, which is
what a caller outside the window would need. But the manual describes both of the
interactive ones in terms of a File Browser appearing and a warning in the Info editor
header, and the API gives no statement about background use. The general warning applies:
"Certain operators in Blender are only intended for use in a specific context"
(https://docs.blender.org/api/5.1/info_gotchas_operators.html). So the operators exist and
are callable from Python — **documented** — but whether they work with no window is
**not in the sources**. `ID.is_missing` is a plain property and has no such doubt.

**Operating system.** Nothing in the sources makes any of this OS-dependent.

**Where I looked.** Grepped all of `api_5.1`, `manual_5.1` and `release_notes_5.1` for
*report_missing_files*, *find_missing_files*, *make_paths_absolute*, *make_paths_relative*,
*missing*, *is_missing*, *broken*, *Report Missing Files*, *Find Missing Files*,
*Relocate*. Read `bpy.ops.file.txt` around each operator, `bpy.types.ID.txt`,
`interface/window_system/topbar.txt` (the whole External Data and Clean Up sections),
`files/blend/packed_data.txt`, `files/linked_libraries/link_append.txt`. Ruled out
`troubleshooting/recover.txt` — it is about autosave and crashes, not missing references.

---

## 7. Can something a file points at be traced to the scene that uses it?

**Label: inferred** — the pieces to do it are documented; the trace itself is a walk you
have to perform, and only one leg of it is a single documented call.

**What is documented.** Every data-block knows how many things use it, and the whole
who-uses-what map can be asked for in one call:

> "user_map(*, subset=None, key_types=None, value_types=None)
> Returns a mapping of all ID data-blocks in current bpy.data to a set of all data-blocks using them.
> …
> Returns:
> dictionary that maps data-blocks ID's to their users.
> Return type:
> dict[bpy.types.ID, set[bpy.types.ID]]"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.BlendData.html
Date read: 2026-07-30

> "users
> Number of times this data-block is referenced (in [0, inf], default 0, readonly)"

and

> "user_of_id(id)
> Count the number of times that ID uses/references given one"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.ID.html
Date read: 2026-07-30

For objects specifically, the last leg of the walk is a single property:

> "users_scene
> The scenes this object is in.
> Type: tuple[Scene, …]
> Note
> Takes O(len(bpy.data.scenes) * len(bpy.data.objects)) time."

and beside it

> "users_collection
> The collections this object is in.
> Type: tuple[Collection, …]
> Note
> Takes O(len(bpy.data.collections) + len(bpy.data.scenes)) time."

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.Object.html
Date read: 2026-07-30

And the manual establishes that data-blocks reference one another in exactly the chain this
question needs:

> "They can reference each other, for reuse and instancing. (Child/parent, object/object-data, materials/images, in modifiers or constraints too…)"

Source: manual, https://docs.blender.org/manual/en/5.1/files/data_blocks.html
Date read: 2026-07-30

*The step I took:* an external file is held by a data-block (an `Image`, `Sound`,
`MovieClip`, `Library`, `CacheFile`, `VectorFont`…, see question 5). `user_map()` gives the
data-blocks that use that one; applied again to each of those it walks up the chain
(image → material → mesh → object); `Object.users_scene` then names the scenes. Each link is
documented; joining them into a trace is mine, so the answer is **inferred**.

**What is not a single answer.** There is no documented property or call that goes straight
from an image (or any non-object data-block) to the scenes using it. `users_scene` exists
only on `Object` in what I read.

**In the window.** The Outliner's Blender File mode shows a user *count*, not a scene:

> "Blender File:
> Lists all data in the current blend-file. On the right side of the list, a shield icon shows the number of users – clicking it adds or removes a fake user."

Source: manual, https://docs.blender.org/manual/en/5.1/editors/outliner/interface.html
Date read: 2026-07-30

The manual's screenshot of that mode confirms what it looks like: a single root row
"Current File" with collapsed rows beneath it named Brushes, Cameras, Collections, Images,
Lights, Materials, Meshes, Objects, Scenes, Screens, Texts, Window Managers, Workspaces and
Worlds — a grouping by kind of data, not by which scene uses what. The display-mode dropdown
in the same picture lists Scenes, View Layer, Sequence, Blender File, Data API and Orphan
Data.
Image: `_images/files_data-blocks_outliner-blender-file-view.png`, shown on
https://docs.blender.org/manual/en/5.1/files/data_blocks.html — label: **documented** for
what is visible in it.
And the manual's text on counts: "Every data-block has its usage counted (reference count),
when there is more than one, you can see the number of current users of a data-block to the
right of its name in the interface."
(https://docs.blender.org/manual/en/5.1/files/data_blocks.html)

**Not covered.** Whether the walk terminates cleanly in every case (cycles, node groups,
overrides, drivers referring to data-blocks). Whether `user_map()` counts a reference made
only by a driver or a custom property. What happens for data used by the Video Sequencer or
the compositor of one scene but not by any object. The cost of doing this on a large file —
the two `Object` properties carry cost notes, `user_map` does not.

**Outside the interface.** `user_map()`, `users`, `user_of_id()`, `users_scene` and
`users_collection` are data access with no window requirement stated, so the trace can be
performed without the interface. The Outliner route is inside the interface only, by
definition, and it gives a count rather than a scene in any case.

**Operating system.** The sources say nothing that makes any of this OS-dependent.

**Where I looked.** Grepped all of `api_5.1` for *user_map*, *users_scene*,
*users_collection*, *user_of_id*, *^users*; grepped `manual_5.1` for *Blender File*,
*users*, *user count*, *Remap Users*, *fake user*, *Orphan*, *Unused Data*. Read in full:
`editors/outliner/interface.txt` (display modes and filters), `editors/outliner/editing.txt`
(the ID-data operations), `files/data_blocks.txt`, and the API pages `bpy.types.BlendData`,
`bpy.types.ID`, `bpy.types.Object`, `bpy.ops.outliner`. Opened the image
`files_data-blocks_outliner-blender-file-view.png`. Ruled out
`editors/outliner/selecting.txt` and `usage.txt` — selection and navigation, no user
tracing.

---

## 8. Can a file's full chain of references be read while some of the referenced files are absent?

**I read the question as:** a file links to a library, which itself links to another, and so
on; if one of those files is not on disk, can the whole chain still be read out?

**Label: documented** for what happens to the missing link and for the part of the chain the
file itself holds. **Not in the sources** for whether the chain can be followed *past* a
missing file.

**A missing library does not lose the reference.**

> "Broken Libraries
> If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."

Source: manual, https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html
Date read: 2026-07-30

The placeholders are identifiable:

> "is_missing
> This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore) (default False, readonly)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.ID.html
Date read: 2026-07-30

The same behaviour is named as an option on import operations:

> "USE_PLACEHOLDERS
> Generate a placeholder (empty ID) if not found in any library files."

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.BlendImportContext.html
Date read: 2026-07-30

**What can be read of the chain.** Libraries are themselves data-blocks and they record
their position in the chain:

> "class bpy.types.Library(ID)
> External .blend file from which data is linked"
> "filepath — Path to the library .blend file"
> "parent — (readonly) Type: Library"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.Library.html
Date read: 2026-07-30

and data-blocks record which library they came from and whether they arrived indirectly:

> "library — Library file the data-block is linked from (readonly) Type: Library"
> "is_library_indirect — Is this ID block linked indirectly (default False, readonly)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.ID.html
Date read: 2026-07-30

And the list of everything external that the open file refers to:

> "bpy.utils.blend_paths(*, absolute=False, packed=False, local=False)
> Returns a list of paths to external files referenced by the loaded .blend file."

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.utils.html
Date read: 2026-07-30

**Where the answer runs out.** The onward references of a library live inside that library's
own file. The sources never say whether a file records the paths of libraries it reaches
only indirectly, nor whether those `Library` data-blocks with their `parent` links survive
when the intermediate file is absent. `Library.parent` is documented to exist; nothing
documents how it is populated when the parent could not be opened. So: whether the chain can
be followed past a missing file is **not in the sources**.

**One documented way to read a chain without disturbing the open file.**

> "static temp_data(*, filepath=None)
> A context manager that temporarily creates blender file data.
> Parameters:
> filepath (str | bytes | None) – The file path for the newly temporary data. When None, the path of the currently open file is used.
> Returns:
> Blend file data which is freed once the context exits."

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.BlendData.html
Date read: 2026-07-30

and, for listing what is inside another file without loading all of it:

> "load(filepath, *, link=False, …)
> Returns a context manager which exposes 2 library objects on entering. Each object has attributes matching bpy.data which are lists of strings to be linked."

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.BlendDataLibraries.html
Date read: 2026-07-30

The Blender-as-a-module page points at both as the alternative to starting more processes:
"In some cases the library API may be an alternative to starting separate processes,
although this API operates on reading and writing ID data-blocks and isn't a complete
substitute for loading .blend files"
(https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html). The sources do not say
what either does when the file named is not there.

**Not covered.** What error, if any, is raised when a library file is absent. Whether an
absent *image* (as opposed to library) leaves anything equivalent to a placeholder. Whether
`blend_paths()` includes libraries that could not be opened. Whether relocating a missing
library mid-chain restores everything below it — the manual says "the lost data can be
automatically restored" for the library that was relocated, and says nothing about what that
library in turn links to.

**Outside the interface.** All of `Library.filepath`, `Library.parent`, `ID.library`,
`ID.is_missing`, `ID.is_library_indirect`, `bpy.utils.blend_paths`,
`BlendData.temp_data` and `BlendDataLibraries.load` are data access and context managers
with no window requirement stated. The placeholder behaviour is described as happening
"while loading a blend-file", which is not tied to the interface. The repair route the
manual describes — right-click Relocate in the Outliner — is inside the interface, but its
operator equivalents (`bpy.ops.wm.lib_relocate`, `bpy.ops.outliner.lib_relocate`) exist; the
sources do not say whether they run without a window.

**Operating system.** Nothing in the sources makes any of this OS-dependent. The
non-UTF-8 path warning quoted under question 5 applies to reading these paths on any system.

**Where I looked.** Grepped all of `api_5.1`, `manual_5.1` and `release_notes_5.1` for
*missing*, *is_missing*, *broken*, *placeholder*, *indirect*, *is_library_indirect*,
*parent*, *relocate*, *blend_paths*, *temp_data*, *libraries.load*, *libraries.write*.
Read in full: `files/linked_libraries/link_append.txt` and
`files/data_blocks.txt`. Searched but did not read in full
`files/linked_libraries/library_overrides.txt` — its only hits for *missing* concern
overrides and resync ("if the later goes missing, the pose bones are definitively lost";
"some objects might go missing"), nothing about reference chains. Read the API pages
`bpy.types.Library`,
`bpy.types.BlendDataLibraries`, `bpy.types.BlendData`, `bpy.types.ID`,
`bpy.types.BlendImportContext`, `bpy.types.BlendImportContextLibrary`,
`bpy.types.BlendImportContextItem`, `bpy.app.handlers`, `bpy.utils`,
`info_advanced_blender_as_bpy`. Ruled out `files/asset_libraries/*` — asset libraries are
directories of files for browsing, not a reference chain.

---

## What the hand tests settled

**Observed (2026-08-04).** Eleven tests were run on the Creator's own computer — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — with Blender **5.1.2** (hash ec6e62d40fa9), started with no window (`-b`), with `--factory-startup`, and with `BLENDER_USER_RESOURCES` pointed at a throwaway settings folder so the Creator's own settings were untouched. The full record of each, with the exact commands and the raw output, is in `Preparation/Blender Research/Findings/5.1/Reading a file.md`, filed under the question it answers. What they settled, question by question:

**Question 2.** With a 60-frame background render of a file under way, a second Blender opened that same file with no window, printed its path and its three object names, and quit with an exit code of 0 while the render was still going. The render finished with an exit code of 0, wrote all 60 frames, and printed no error or warning of any kind. Repeated with a third Blender that opened the file, renamed an object and saved over it mid-render, the same held: the saver returned `{'FINISHED'}`, the render still finished with an exit code of 0 and all 60 frames and no error or warning, and reopening the file afterwards showed the saver's rename had taken. The other half of the question — how many Blenders can run at once — stays unresolved, and no test can settle it: whatever number this Mac reaches is a fact about this Mac.

**Observed (2026-08-05).** The whole of that test was run again, on the same Mac and with the same file, because the first run's record described the moment it started the second Blender wrongly. The second and third Blenders were started at the first look where two finished frames sat in the render's output folder — the folder was looked at five times a second and the count printed at every look, so what was waited for and what was seen are both on the page. The result was the same in every respect: the reader read the file and quit with an exit code of 0 mid-render, the saver returned `{'FINISHED'}`, both renders finished with an exit code of 0 and all 60 frames and no error or warning, and the rename was in the file when it was reopened. So the answer holds whether the other Blender starts after one finished frame or after two. The full record, with every poll line as it came back, is in `Preparation/Blender Research/Findings/5.1/Reading a file.md` under question 2, and the correction to the first run's description is beside it there.

**Question 3.** `ViewLayer.use` reads a default of `True` and `is_animatable` of `True`. It can be keyframed from Python, and the curve lands on the scene with the data path `view_layers["LayerB"].use`. Stepping the frame changes what the flag reads. An animation render honours the keyframed value: the file written for frame 1 held both view layers and the file written for frame 2 held only the one that stayed switched on. The terminal named each view layer as it started rendering. That answers the gap noted above — the renderer does re-read the flag per frame. Separately, on 2026-08-05 the Creator did the same thing in Blender's own window: hovering over the `Use for Rendering` checkbox and pressing **I** turned the checkbox yellow and the dot beside it into a diamond, while pressing **I** over `Render Single Layer` produced the message `"use_single_layer" property cannot be animated`. Their screenshot of the panel before the keyframe shows a round dot to the right of `Use for Rendering` and no dot beside `Render Single Layer`, which settles by sight the step this document had to infer from the manual's picture.

**Question 4.** `frame_path()` returns a path with any template expressions worked out, while the stored string keeps its braces; a malformed template makes it raise. The output-path marker `is_path_output` reads `True` for `RenderSettings.filepath` and `False` for `Image.filepath`, `Library.filepath` and `PointCache.filepath`. Neither `bpy.utils.blend_paths()`, nor `bpy.data.file_path_map()`, nor a callback given to `bpy.data.file_path_foreach()` returned the render output path or the File Output node's paths on a file that had both set. The File Output node's per-input names kept their slashes, and a render of one frame wrote a single file, `out/shot0001.exr`. On 2026-08-05 the Creator photographed the node's sidebar in Blender's own window: it does hold a section headed `Output Paths`, and that section holds **one** line for the whole node rather than one per input. On this version's screenshot the line is shortened to fit the panel and reads `/Users/artist/Documents/Sm...sts/Reading a file/13/out/shot.exr`. The same panel shows the two path fields reading `//out/` and `shot`, a two-button row with `Multi-Layer EXR` selected, and a `Layers` list holding `passes/diffuse` and `beauty`.

**Question 5.** An image's `filepath` was read, assigned a new value, saved and read back after reopening the file. It took the new value and kept it, which settles the step marked inferred above. All of it was done with no window.

**Question 6.** `report_missing_files()` returned `{'FINISHED'}` and printed a warning to the terminal naming the missing path and the data-block, with no window at all. `WindowManager` has no `reports` property in this version, so there is still no list a script could read. `find_missing_files(directory=…)` ran with no window, rewrote the recorded path, and the change survived saving and reopening; the rewritten path came back in the relative `//` form. On 2026-08-05 the Creator ran both from inside Blender's window. `File ‣ External Data ‣ Report Missing Files` put a message at the bottom of the window reading `Path '/Users/artist/Documents/Smoozy hand tests/Reading a file/14/tex_5.1/probe.png' not found, from local data-block 'IMBrokenProbe'`. Calling the search from the Python Console with a directory supplied returned `{'FINISHED'}` and opened no File Browser window. The image's recorded path afterwards read `//../15/tex_5.2/probe.png`, and the Creator reported that console as identical in all four versions; it was not established why the recovered path names `tex_5.2` in a version other than 5.2.

**Question 7.** On a file with two scenes, the repeated `user_map()` walk reached the right scene in four steps from an image, and reached nothing at all from an image nothing used. Asking the map directly for scene-typed users of the image returned an empty set.

**Question 8.** On a three-file chain, `Library.parent` came back `None` for every library in every case, while Blender's own log said `parent '<direct>'` for each. With the far end of the chain missing, both libraries were still listed and the missing one carried `is_missing` `True`; `blend_paths()` still held a path whose file does not exist. With the middle of the chain missing instead, the far file was named nowhere at all — so the readable chain stops at the first absent file. `bpy.data.libraries.load()` on a path that does not exist raised `OSError load: … failed to open blend file`; on a present file it exposed a `version` attribute but no `libraries` attribute, and listed only that file's own local data-blocks.

Everything above was run on macOS. The tests reachable from a terminal were run by this chat on 2026-08-04; the ones needing eyes on Blender's window were run by the Creator on 2026-08-05, and their full records, with the screenshots' locations, are in `Preparation/Blender Research/Findings/5.1/Reading a file.md`.

**Observed (2026-08-06).** New equipment arrived, and every one of those tests was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — with the same Blender build, no window, `--factory-startup`, and this topic's own settings folder. Blender's window was also opened on that machine's own display for three of them. **Every result agreed with the Mac's**, question by question: the concurrent reader and saver leaving a 60-frame render untouched; the view-layer switch keyframing and the animation render honouring it frame by frame, with `LayerA` and `LayerB` in the file for frame 1 and only `LayerA` in the file for frame 2; the stored braces, the raise on a malformed template, the four `is_path_output` answers, the three whole-file path lists returning neither output path, the node's per-input names keeping their slashes, and a render writing one file for the whole node; the recorded path read, written, saved and read back with the new value; the report naming the missing file with no window and no `reports` property on the window manager in this version; the search rewriting the path in the relative form and surviving a save and a reopening, with **no File Browser opening** when it was called with a folder inside Blender's window; the four-step walk to the scene and nothing at all from an unused image; and the chain reads, with the readable chain stopping at the first absent file and `Library.parent` empty throughout.

Four things about that second run are worth stating rather than glossing. **The recovered path differed:** on Linux it read `//search/deeper/probe.png`, the plain relative path to where the file was, rather than the Mac's `//../15/tex_5.2/probe.png`. Both stand and neither explains the other. **The trace script had to be repaired** before it would run at all on Linux — the unused image was not in the saved file there and the script raised `KeyError: 'bpy_prop_collection[key]: key "UnusedTex" not found'`; marking that image with a fake user fixed it, and both runs are recorded. **Two window checks could not be driven on the Linux machine**, because nothing there can move the mouse to a control or press a key at it: what Blender shows when the render switch is keyframed by hand, and what the Output Paths panel holds. That the panel exists in this version was confirmed there by screenshot; its contents rested on the Creator's Mac screenshot alone.

> **Note from the citation check of the Linux hand tests, 2026-08-06.** A tool for moving the pointer was put on that machine afterwards and the check tried both window items again. Neither is settled: a click on that machine could not be confirmed against any state Blender will report. Both still rest on the Creator's Mac. And **no claim is made about how long anything took** on the Linux machine, because other work was running on it.

So every operating-system line in this document is settled for **macOS and Linux** and stays open for **Windows**. No Windows machine has run it. The full Linux record, kept separate from the Mac's, is in `Preparation/Blender Research/Findings/5.1/Reading a file.md`, and the list it came from is `Audits/Blender Research/Reading a file/Hand tests — Linux.md`.

---

## Things a person with Blender 5.1 open could settle in a few minutes

The hand-test step ran on 2026-08-04 and settled every item that stood here except the ones below. What it found is written into `Preparation/Blender Research/Findings/5.1/Reading a file.md` as observed findings, and summarised in the section above. New equipment arrived on 2026-08-06 and the same tests were run again on a rented Linux machine; that is written into the same document, beside the Mac's results and never merged with them.

1. **How many Blenders will start at once.** Whether they disturb each other was run on both machines and is recorded under question 2 above. How many will start at once was not, and no test can settle it: one computer cannot establish a limit for every computer, and the rented machine is a container sharing someone else's hardware.
2. **Whether one Blender refuses a file another has open, in the interface.** The same pair of files was run with no window on both machines and neither refused the other. Doing it with two windows open needs eyes on both, so it stays open.
3. **Anything on Windows.** No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

2026-08-04 — Opus 5 (Claude Code) — Reading a file, Blender 5.1
