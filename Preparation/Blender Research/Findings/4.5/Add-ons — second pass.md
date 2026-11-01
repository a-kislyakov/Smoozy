# Add-ons — Blender 4.5 — second pass (questions 2, 3, 4, 11)

**Sources consulted, and when.** Blender 4.5 manual (`manual_4.5`, including two of its images), Blender 4.5 Python API documentation (`api_4.5`), Blender 4.5 release notes (`release_notes_4.5`), Cycles' own files as shipped with Blender 4.5 (`cycles_4.5`), and the undated Blender requirements page in `requirements/requirements_2026-08-01.txt`. All read on **2026-07-30** (the snapshot date given for the documentation folders). The requirements page carries no version in its address, so I have used it only where it names 4.5 in so many words.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

A note on one source. The requirements page is not published per version. Where I quote it below I say which sentence names 4.5 and which does not.

---

## Question 2

### Settled by hand test — which engines are registered, and whether an add-on's part in a render can be seen

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 4.5.12. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**The list of engines a Blender will accept is asked for from the command line, and it shrinks when an add-on goes.** With everything as installed:

`blender -E help`

printed:

```
Blender Engine Listing:
	BLENDER_EEVEE_NEXT
	BLENDER_WORKBENCH
	CYCLES
```

That command then ended with `libc++abi: terminating` and an `Abort trap: 6`, exit status 134. Cycles was then switched off:

`blender --background --python-expr "import bpy; print('DISABLE', bpy.ops.preferences.addon_disable(module='cycles')); bpy.ops.wm.save_userpref()"`

which printed `DISABLE {'FINISHED'}` and `Info: Preferences saved`. Running `blender -E help` again printed:

```
Blender Engine Listing:
	BLENDER_EEVEE_NEXT
	BLENDER_WORKBENCH
```

A test add-on written for this work, whose whole content is a `RenderEngine` class with `bl_idname = "SMOOZY_TEST"`, was built and installed with `blender --command extension install-file -r user_default -e smoozy_engine-1.0.0.zip`. After that, `blender -E help` printed:

```
Blender Engine Listing:
	BLENDER_EEVEE_NEXT
	BLENDER_WORKBENCH
	CYCLES
	SMOOZY_TEST
```

**The engine setting's own list of values is not where add-on engines appear.** With Cycles registered:

`blender --background --python-expr "import bpy; r=bpy.context.scene.render; print('ENGINE', r.engine); print('ITEMS', [i.identifier for i in bpy.types.RenderSettings.bl_rna.properties['engine'].enum_items]); print('MULTI', getattr(r,'has_multiple_engines','NO SUCH PROPERTY')); print('SUBCLASSES', [(c.__name__, getattr(c,'bl_idname',None)) for c in bpy.types.RenderEngine.__subclasses__()])"`

printed:

```
ENGINE BLENDER_EEVEE_NEXT
ITEMS ['BLENDER_EEVEE_NEXT']
MULTI True
SUBCLASSES [('HydraRenderEngine', None), ('CyclesRender', 'CYCLES')]
```

and after Cycles was switched off, the same command printed the same first three lines with:

```
SUBCLASSES [('HydraRenderEngine', None)]
```

So there is a Python route to which engines are registered and by what class — the subclasses of `bpy.types.RenderEngine` — and it changes with what is switched on. The engine property's own value list does not.

**An add-on's declared category can be read from Python.** `blender --background --python-expr "import addon_utils; print('PATHS', addon_utils.paths()); m=list(addon_utils.modules()); print('COUNT', len(m)); [print('MODULE', x.__name__, getattr(x,'bl_info',None)) for x in m]"` returned each add-on's whole metadata block, for example:

```
MODULE cycles {'name': 'Cycles Render Engine', 'author': '', 'blender': (2, 80, 0), 'description': 'Cycles renderer integration', 'warning': '', 'doc_url': 'https://docs.blender.org/manual/en/latest/render/cycles/', 'tracker_url': '', 'support': 'OFFICIAL', 'category': 'Render'}
MODULE node_wrangler {'name': 'Node Wrangler', 'author': 'Bartek Skorupa, Greg Zaal, Sebastian Koenig, Christian Brinkmann, Florian Meyer', 'version': (3, 55), 'blender': (4, 2, 0), 'location': 'Node Editor Toolbar or Shift-W', 'description': 'Various tools to enhance and speed up node-based workflow', 'warning': '', 'doc_url': '{BLENDER_MANUAL_URL}/addons/node/node_wrangler.html', 'support': 'OFFICIAL', 'category': 'Node'}
```

Extensions installed from a package appear in the same list, by their full module name — `bl_ext.user_default.smoozy_test` — so the category and the rest of that block are readable for both kinds of add-on.

**An add-on that is not an engine can take part in a render, and its absence is silent.** A second test add-on was written whose whole content adds a function to `bpy.app.handlers.render_pre` that prints a line. Rendering one frame of a plain file with it installed printed, among the render's own lines:

```
SMOOZY HANDLER RAN
```

Rendering the same file in a settings folder where that add-on had never been installed printed no such line, no warning, and nothing naming the add-on. Both renders finished, both wrote an image, and both exited 0. The two images were the same size (128 by 128); the handler's change to the resolution did not affect the frame. (*Citation check, 2026-08-05: this used to say "because the handler's change came too late", which is a reading of why rather than something the run showed.*)

**Outside the interface.** Settled: the engine list, the registered engine classes and every add-on's metadata block are all reachable with Blender driven from outside its window.

**Operating system.** Settled for macOS only, on the one machine described above. The same tests were run again on a rented Linux machine, and that result has its own block under this question; the two are never merged. Windows stays untested.

### Settled by hand test on the rented Linux machine — which engines are registered, and whether an add-on's part in a render can be seen

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 4.5.12 LTS from `/opt/blender/blender-4.5.12-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

**The engine list, asked for from the command line.** Two things came out that the Mac could not show.

`blender -E help`, with `DISPLAY` and `XAUTHORITY` unset, does not print an engine list at all. In Blender 4.5 it printed:

```
GHOST: failed to initialize display for back-end(s): ['WAYLAND', 'X11']
  'WAYLAND': unable to connect to display!
  'X11': unable to open a display!
GHOST: unable to initialize, exiting!
```

and then did not exit; wrapped in `timeout 30`, exit status 124. In 5.0, 5.1 and 5.2 the same run printed a timestamped version of those lines, ending `ERROR Unable to initialize GHOST, exiting!`, followed by `Error: Not freed memory blocks: ...`, and exited 1. So on Linux this command needs a display even though it prints nothing but text. The Mac was never asked this question: no run there hid the display from Blender, so the macOS record says nothing about it. This is therefore a Linux finding, not a difference between the two systems. *(Citation check, 2026-08-06: this read "The Mac never showed this, because macOS gave the list without one", which the macOS record does not carry.)*

With the machine's own display available, `blender -E help` printed:

```
Blender Engine Listing:
	BLENDER_EEVEE_NEXT
	BLENDER_WORKBENCH
	CYCLES
```

**Blender 4.5 never exits from `-E help` on this machine.** The engine list is printed and the program keeps running until it is killed; every such run here was wrapped in `timeout 30` and came back with exit status 124. Cycles was then switched off with `blender --background --python-expr "import bpy; print('DISABLE', bpy.ops.preferences.addon_disable(module='cycles')); bpy.ops.wm.save_userpref()"`, and the same command printed:

```
Blender Engine Listing:
	BLENDER_EEVEE_NEXT
	BLENDER_WORKBENCH
```

The engine names are the same as the macOS run recorded for this version.

**The engine setting's own list, and the registered engine classes.**

`blender --background --python-expr "import bpy; r=bpy.context.scene.render; print('ENGINE', r.engine); print('ITEMS', [i.identifier for i in bpy.types.RenderSettings.bl_rna.properties['engine'].enum_items]); print('MULTI', r.has_multiple_engines); print('SUBCLASSES', [(c.__name__, getattr(c,'bl_idname',None)) for c in bpy.types.RenderEngine.__subclasses__()])"`

printed:

```
ENGINE BLENDER_EEVEE_NEXT
ITEMS ['BLENDER_EEVEE_NEXT']
MULTI True
SUBCLASSES [('HydraRenderEngine', None), ('CyclesRender', 'CYCLES')]
```

and after Cycles was switched off, the same command printed the same first three lines with `SUBCLASSES [('HydraRenderEngine', None)]`. Exit status 0 throughout. Identical to the macOS run.

**An add-on's declared metadata block.** `blender --background --python-expr "import addon_utils; print('PATHS', addon_utils.paths()); m=list(addon_utils.modules()); print('COUNT', len(m)); [print('MODULE', x.__name__, getattr(x,'bl_info',None)) for x in m]"` returned a whole metadata block for every bundled add-on, for example:

```
MODULE cycles {'name': 'Cycles Render Engine', 'author': '', 'blender': (2, 80, 0), 'description': 'Cycles renderer integration', 'warning': '', 'doc_url': 'https://docs.blender.org/manual/en/latest/render/cycles/', 'tracker_url': '', 'support': 'OFFICIAL', 'category': 'Render'}
```

That block is character for character the one the macOS run recorded.

**An add-on that is not an engine takes part in a render, and its absence is silent.** The same handler test add-on was built and installed. Rendering one frame with it installed printed, among the render's own lines:

```
SMOOZY HANDLER RAN
```

Rendering the same file in a settings folder where it had never been installed printed no such line, no warning, and nothing naming the add-on. Both renders wrote an image, both exited 0, and both images measured 128 by 128:

```
IMG with_0001.png (128, 128)
IMG without_0001.png (128, 128)
```

The macOS run and this one agreed throughout.

**Outside the interface.** Settled again on a second operating system, with one Linux-only limit: `-E help` needs a display on this machine, so the engine list is **not** reachable from a Blender that has no display to open. Everything else here was reached with Blender driven from outside its own window.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. The engine names, the registered classes and the metadata block agreed on both. One thing here is settled for Linux alone: `-E help` needs a display on this machine. The matching case was never posed on the Mac, so it is not recorded as a difference between the two systems. *(Citation check, 2026-08-06: narrowed from "needs a display on Linux and did not on macOS".)* Windows stays untested.

### Settled by hand test on the rented Linux machine — what an add-on's opened row shows, and the engine field with the add-on switched off

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 4.5.12 LTS, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`. Fresh settings folders, preferences saved once beforehand so the Quick Setup panel is not in the picture.

**An add-on's opened row.** A script set `show_expanded` in the metadata blocks of `cycles` and `node_wrangler`, opened the Add-ons preferences and photographed them. The picture shows, for **Cycles Render Engine**: the line "Cycles renderer integration.", the word "Built-in" at the right of it, a **Website** field reading `docs.blender.org`, a **Type** field reading `Core`, and then a section headed **Preferences** holding one row of buttons — `None`, `CUDA`, `OptiX`, `HIP`, `oneAPI` — with `None` selected. For **Node Wrangler**: "Various tools to enhance and speed up node-based workflow.", "Built-in", the same **Website** field, **Type** `Core`, a **Maintainer** field naming five people, and a **Version** field reading `3.55`.

**Nothing in either row says whether the add-on takes part in a render.** Neither row carries a tag, a category, a licence, a repository name or a declared permission. Cycles shows no maintainer and no version; Node Wrangler shows both. Both read `Type  Core`. That is the same as the Creator's macOS run in Blender 5.2, except for the row of compute-device buttons, which names this machine's own kinds of graphics hardware rather than the Mac's — see question 4.

**The Render Engine field when the engine's add-on is switched off.** A file saved with Cycles was opened with a window, in a settings folder where Cycles had been switched off. The script printed:

```
ENGINE BLENDER_EEVEE_NEXT
HAS_CYCLES False
```

and the picture shows the **Render Engine** field reading **`EEVEE`**, with EEVEE's own settings below it — Sampling, Viewport Samples 16, Render Samples 64, Shadows, Advanced, Light Paths, Raytracing. A red warning sits along the bottom of the window: **`Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be inst`** — cut off by the width of the window. It was already there when the picture was taken, before any render was asked for.

Note that the field reads `EEVEE` in all four versions, although the identifier behind it differs. The version difference recorded elsewhere in this document is not visible at the window.

**The engine list, opened without clicking.** A click on the Render Engine field could not be shown to do anything. The list was opened instead with Blender's own operator:

`bpy.ops.wm.context_menu_enum('INVOKE_DEFAULT', data_path='scene.render.engine')`

which returned `{'FINISHED'}` in all four versions. The picture shows a menu headed **Engine** holding exactly two entries — **EEVEE**, highlighted as the current one, and **Workbench** — with a tooltip beside it reading `Engine to use for rendering: EEVEE`. **Cycles is not among them.** The same in all four versions, and it agrees with the Creator's macOS run in Blender 5.2, which reported that only EEVEE and Workbench were offered.

**A note on the pointer, and what it is worth.** The machine has `xdotool`, so the pointer can be moved and clicked. Two things were established before any of the above was written down. Blender **does** receive button and key events sent to its own window: its own modal handler printed `EVENT LEFTMOUSE PRESS`, `EVENT LEFTMOUSE RELEASE`, `EVENT LEFTMOUSE CLICK` with the coordinates that were sent, and an `A` key press. But it receives **no pointer-motion events at all** — a modal handler watching for them saw none — and **a click could not be shown to work any control**: after clicking a Properties tab icon the editor's `context` still read `TOOL`; after clicking a preferences section `active_section` still read `INTERFACE`; after clicking the "Filter Add-ons" tick box `use_filter_by_owner` still read `False`. **So nothing above was got by clicking.** Every panel and menu was opened by one of Blender's own operators, with the identifier read out of `bpy.types` first rather than guessed, and each operator's own return value is quoted beside it.

**Outside the interface.** This is a window test. The matching no-window results are under question 2 above and question 7 below.

**Operating system.** Settled for Linux in this version, at the window. The Creator's macOS run covered Blender 5.2 only, so for the other versions there is no macOS result to compare with. Windows stays untested.


> What makes a `.blend` file need an add-on at the moment it renders, and can an add-on that takes part in producing the render be told apart from one that only rearranges Blender's existing tools while the artist works?

**This question has two halves and I answer them separately.**

### 2a — What makes a file need an add-on at render time

**LABEL: inferred** (built on documented and observed pieces; the step is written out below).

The sources never state, in one place, "a file needs an add-on when X". What they do give is a set of things an add-on can install into Blender, plus proof that a file can store references to those things. I take the step from those two to "the file then needs the add-on". That step is mine, so the label is inferred.

**The pieces, quoted.**

*An add-on installs things into Blender by registering classes, and the list of what may be registered is documented.* From the API documentation for `bpy.utils.register_class`, the accepted class types are:

> "cls (type[bpy.types.Panel | bpy.types.UIList | bpy.types.Menu | bpy.types.Header | bpy.types.Operator | bpy.types.KeyingSetInfo | bpy.types.RenderEngine | bpy.types.AssetShelf | bpy.types.FileHandler | bpy.types.PropertyGroup | bpy.types.AddonPreferences | bpy.types.NodeTree | bpy.types.Node | bpy.types.NodeSocket]) – Registerable Blender class type."

(this is the `cls` parameter line under `bpy.utils.register_class`; the same list appears again under `unregister_class`)
Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.utils.html — read 2026-07-30

*An add-on can register a render engine, and that engine carries an identifier.* The API documentation gives a worked example:

> "class CustomRenderEngine(bpy.types.RenderEngine):
> # These three members are used by Blender to set up the
> # RenderEngine; define its internal name, visible name and capabilities.
> bl_idname = "CUSTOM""

and further down:

> "def register():
> # Register the RenderEngine.
> bpy.utils.register_class(CustomRenderEngine)"

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html — read 2026-07-30

*The scene's engine setting is an enumeration, and in the generated documentation it holds only one entry.*

> "engine
>
> Engine to use for rendering
>
> Type:
>
> enum in ['BLENDER_EEVEE_NEXT'], default 'BLENDER_EEVEE_NEXT'"

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30

*Cycles — which ships with Blender — is itself an add-on with a registered engine, and it plants settings on the scene and other data.* **Observed**, from Cycles' own files:

> "bl_info = {
>     "name": "Cycles Render Engine",
>     "author": "",
>     "blender": (2, 80, 0),
>     "description": "Cycles renderer integration",
>     "warning": "",
>     "doc_url": "https://docs.blender.org/manual/en/latest/render/cycles/",
>     "tracker_url": "",
>     "support": 'OFFICIAL',
>     "category": "Render"}"

File: `cycles_4.5/__init__.py`, lines 7–16 — read 2026-07-30

> "class CyclesRender(bpy.types.RenderEngine):
>     bl_idname = 'CYCLES'
>     bl_label = "Cycles""

File: `cycles_4.5/__init__.py`, lines 42–44 — read 2026-07-30

> "    @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )
>
>     @classmethod
>     def unregister(cls):
>         del bpy.types.Scene.cycles"

File: `cycles_4.5/properties.py`, lines 1100–1110 — read 2026-07-30
The same pattern appears for `bpy.types.Camera.cycles_custom` (lines 1116–1125), `bpy.types.Material.cycles` (lines 1172–1181) and `bpy.types.Light.cycles` (lines 1212–1221), among others.

*A file can also carry scripts that run at render time, independently of any add-on.* The manual lists these:

> "Registered Text-Blocks
>
> A text data-block can have its Register option enabled which means it will load on start.
>
> Animation Drivers
>
> Python expressions can be used to Drive values and are often used in more advanced rigs and animations."

and

> "Rendering with Freestyle, because Freestyle uses scripts to control line styles."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html — read 2026-07-30

**The step I took.** The scene stores which engine to render with, and an engine identifier such as `'CYCLES'` only exists while the add-on that registers it is enabled. Likewise, a scene stores `scene.cycles` settings, which only exist while the add-on that plants them is enabled. So: a file needs an add-on at render time when the file names an engine that the add-on registers, or holds data under a property the add-on plants, or holds a node type the add-on registers. That "so" is the inference; no source states it.

**Not covered.** Nothing in the sources tells me what actually happens on render when any of these are absent — whether the render fails, substitutes something, or ignores the setting. Nothing tells me whether Blender writes a list of contributing add-ons into the file. And nothing distinguishes an add-on merely *touched* while authoring (an importer, say, that made the geometry and is then no longer needed) from one still needed at render — the pieces above cover only what is referenced by stored data.

**Outside the interface.** The pieces quoted are all about registration and stored data, not about the window. Nothing in what I read suggests they differ when Blender is driven from outside. But the sources do not say so in so many words, so this is not settled.

**Operating system.** The sources say nothing about any of this differing by operating system.

### 2b — Can the two kinds of add-on be told apart?

**LABEL: not in the sources** — for any reliable, Blender-enforced distinction. What exists is self-declared labelling, quoted below.

**What is there.** Only descriptive metadata the add-on's own author chooses.

*Extension manifests may carry tags, from a fixed list, and the list includes both "Render" and "User Interface".*

> "tags:
>
> List of tags. See the list of available tags."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html — read 2026-07-30

The tag list for add-ons runs: "3D View, Add Curve, Add Mesh, Animation, Bake, Camera, Compositing, Development, Game Engine, Geometry Nodes, Grease Pencil, Import-Export, Lighting, Material, Modeling, Mesh, Node, Object, Paint, Pipeline, Physics, Render, Rigging, Scene, Sculpt, Sequencer, System, Text Editor, Tracking, User Interface, UV" — each on its own line.
Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/tags.html — read 2026-07-30

The manual describes tags as coming from the platform, not from Blender's inspection: the manifest comment reads

> "# # Optional: tag list defined by Blender and server, see:"

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html — read 2026-07-30

*Legacy add-ons carry a category in `bl_info`, again author-chosen.*

> "bl_info
>
> is a dictionary containing add-on metadata such as the title,
> version and author to be displayed in the Preferences add-on list."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/scripting/addon_tutorial.html — read 2026-07-30

> "The only difference between add-ons and built-in Python modules is that add-ons must contain a bl_info variable
> which Blender uses to read metadata such as name, author, category and project link.
> The User Preferences add-on listing uses bl_info to display information about each add-on."

Source: Python API documentation — https://docs.blender.org/api/4.5/info_overview.html — read 2026-07-30

Cycles declares `"category": "Render"` — **observed**, `cycles_4.5/__init__.py`, line 16. (*Citation check, 2026-08-05: this said line 15, which is `"support": 'OFFICIAL'`.*)

*The manifest's `permissions` field is the only field that declares what an add-on will actually do, and its options say nothing about rendering.*

> "permissions:
>
> Add-ons can list which resources they require. The available options are
> files, network, clipboard, camera, microphone."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html — read 2026-07-30

*And the manual states outright that an add-on is not structurally different from any other script.*

> "An add-on will typically register operators, panels, menu items, etc,
> but it's worth noting that any script can do this,
> when executed from the Text editor or even the interactive console –
> there is nothing inherently different about an add-on that allows it to integrate with Blender,
> such functionality is just provided by the blender_api:bpy module for any script to access."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/scripting/addon_tutorial.html — read 2026-07-30

**Reading the question.** I read "told apart" as *by Blender, or from what an add-on records* — not "can a person reading the description guess". If the other reading was meant, the answer is trivially yes, from the tagline and tags.

**Not covered.** The tags and category are free choices by the author and nothing in the sources says Blender checks them against what the add-on registers. There may be a way to enumerate registered render engines from Python, but I found no page in the API documentation describing one. Whether the extensions platform enforces tag accuracy is outside what I read.

**Outside the interface.** Tags and category are text in a manifest or in `bl_info`; nothing suggests they differ when Blender runs without its window. The `bpy.types.Addon` type — the per-add-on record Blender exposes — has only two properties, neither of them a category:

> "module
>
> Module name"
> "preferences
>
> Type:
>
> AddonPreferences, (readonly)"

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.Addon.html — read 2026-07-30
So the distinction is not available from Python through that type, inside the interface or outside it. That much is documented.

**Operating system.** The sources say nothing about this differing by operating system.

### Where I looked, for question 2

Searched the whole of `manual_4.5` and `api_4.5` (and `release_notes_4.5`) for, case-insensitive: `RenderEngine`, `render engine`, `external render engine`, `third-party render`, `bl_info`, `custom node`, `register a node`, `NodeCustomGroup`, `Undefined`, `undefined node`, `unknown node`, `missing node`, `node type is not`, `requires the add-on`, `add-on must be enabled`, `enable the add-on`, `add-on is enabled`, `if the add-on`, `engine is not found`, `engine not available`, `unknown engine`, `falls back to`, `fall back`, `missing add-on`, `add-on missing`, `custom propert`, `owner_ids`, `use_filter_by_owner`, `Filter Add-ons`, `permission`, `wheel`.

Pages opened in full or in part: manual — `advanced/scripting/addon_tutorial.txt`, `advanced/scripting/security.txt`, `advanced/extensions/addons.txt`, `advanced/extensions/getting_started.txt`, `advanced/extensions/tags.txt`, `advanced/extensions/index.txt`, `addons/index.txt`, `editors/preferences/addons.txt`, `files/custom_properties.txt` (scanned), `advanced/command_line/render.txt`, `advanced/command_line/arguments.txt`. API — `bpy.types.RenderEngine.txt`, `bpy.types.RenderSettings.txt`, `bpy.types.Addon.txt`, `bpy.types.Addons.txt`, `bpy.types.NodeTree.txt` (the "Undefined" line), `bpy.utils.txt`, `info_overview.txt`, `bpy.types.WorkSpace.txt`. Cycles — `__init__.py` (whole), `properties.py` (register functions and the property-group `register` classmethods), `engine.py`, `version_update.py`.

Ruled out and why: the only "Undefined" in the API documentation is `bpy.types.NodeTree`'s node-type enumeration —

> "Undefined – Undefined type of nodes (can happen e.g. when a linked node tree goes missing)."

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.NodeTree.html — read 2026-07-30
— which is about a *linked node tree*, not a missing add-on, so it does not answer this question. The manual's `addons/` folder documents individual bundled add-ons and is grouped by folder (3D View, Animation, Import-Export, Node, Rigging, System), which is a manual-authoring choice and not a property of the add-ons; I did not treat it as evidence. Release notes searched for `extension`, `add-on`, `addon`, `platform` — the four hits touch key-maps, smooth groups, a bug fix list and shader variants, none of them this question.

---

## Question 3

### Settled by hand test — installing, switching on, and setting up an add-on with no window

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 4.5.12. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

**Installing from a file works with no window, and the switch-on survives into later runs.** A small test add-on, written for this work, was built and installed:

`blender --command extension build --source-dir <source folder> --output-dir <output folder>`

printed:

```
building: smoozy_test-1.0.0.zip
complete
created: "<output folder>/smoozy_test-1.0.0.zip", 459
```

then

`blender --command extension install-file -r user_default -e <output folder>/smoozy_test-1.0.0.zip`

printed:

```
STATUS Installed "smoozy_test"
SMOOZY TEST ADDON REGISTERED
SMOOZY TEST ADDON UNREGISTERED
```

(the two capitalised lines are the add-on's own print statements). Exit status 0. A separate, later run then printed the switched-on add-ons:

`blender --background --python-expr "import bpy; print('MODULES', sorted(a.module for a in bpy.context.preferences.addons))"`

```
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

No preferences save was asked for; the install wrote it itself.

**With `--no-prefs`, the package is installed but is not switched on.** In a fresh settings folder, `blender --command extension install-file -r user_default -e --no-prefs <the same zip>` printed `STATUS Installed "smoozy_test"` and nothing else, and a later run printed the eight built-in modules with no `bl_ext.user_default.smoozy_test` among them. `blender --command extension list` still showed `smoozy_test [installed]`.

**The switch-on and switch-off operators work with no window.**

`blender --background --python-expr "import bpy; print('ENABLE', bpy.ops.preferences.addon_enable(module='bl_ext.user_default.smoozy_test')); print('AFTER', sorted(a.module for a in bpy.context.preferences.addons)); print('DISABLE', bpy.ops.preferences.addon_disable(module='bl_ext.user_default.smoozy_test')); print('REFRESH', bpy.ops.preferences.addon_refresh())"`

printed:

```
ENABLE {'FINISHED'}
AFTER ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
DISABLE {'FINISHED'}
REFRESH {'FINISHED'}
```

**`--addons` loads the add-on for that run but does not put it in the switched-on list.** `blender --background --addons bl_ext.user_default.smoozy_test --python-expr "import bpy; print('MODULES', sorted(a.module for a in bpy.context.preferences.addons))"` printed the add-on's own `SMOOZY TEST ADDON REGISTERED` line, but the printed module list held only the eight built-in ones. Given a name that does not exist, the same argument printed:

```
Add-on not loaded: "smoozy_test", cause: No module named 'smoozy_test'
```

and carried on, exit status 0.

**An add-on's own settings can be read, changed and made to stick, with no window.** Cycles' key in the preferences was found by asking rather than guessing:

`blender --background --python-expr "import bpy; ks=[a.module for a in bpy.context.preferences.addons if 'cycles' in a.module]; print('KEYS', ks); p=bpy.context.preferences.addons[ks[0]].preferences; print('PROPS', list(p.bl_rna.properties.keys())); print('DEVICE', p.compute_device_type); print('OFFERED', [i.identifier for i in p.bl_rna.properties['compute_device_type'].enum_items])"`

printed:

```
KEYS ['cycles']
PROPS ['rna_type', 'bl_idname', 'compute_device_type', 'devices', 'peer_memory', 'metalrt', 'use_hiprt', 'use_oneapirt', 'kernel_optimization_level']
DEVICE METAL
OFFERED []
```

Note the last line: the list of compute devices on offer came back **empty** with no window, even though the current value is `METAL`. Because no offered value could be read, the test used a plain yes-or-no setting instead, and set it to the opposite of whatever it already was:

`blender --background --python-expr "import bpy; p=bpy.context.preferences.addons['cycles'].preferences; print('BEFORE', p.peer_memory); p.peer_memory = not p.peer_memory; print('SET', p.peer_memory); print('SAVE', bpy.ops.wm.save_userpref())"`

```
BEFORE False
SET True
SAVE {'FINISHED'}
```

and a separate later run:

`blender --background --python-expr "import bpy; p=bpy.context.preferences.addons['cycles'].preferences; print('AFTER', p.peer_memory)"`

```
AFTER True
```

**Fetching from the internet is refused unless it is allowed.** In a fresh settings folder, `blender --command extension sync` printed a Python traceback ending:

```
RuntimeError: Operator bpy.ops.extensions.repo_sync_all.poll() Online access required to check for updates. Enable online access in System preferences
Error synchronizing
```

exit status 1. With `--offline-mode` the same command ended `... Launch Blender without --offline-mode`. With `--online-mode` it succeeded.

**Outside the interface.** Settled: installing, switching on, switching off, and setting and saving an add-on's own settings all work with Blender driven entirely from outside its window. Two limits were seen: `--addons` does not add the add-on to the switched-on list, and Cycles' list of compute devices comes back empty in this mode.

**Operating system.** Settled for macOS only, on the one machine described above. Linux was tested afterwards on a rented Linux machine, in its own block under this question, and again on a third machine with no display server at all, in a block of its own below. **A computer with no graphics hardware at all is still not tested**; the screenless case itself now is.

### Settled by hand test on the rented Linux machine — installing, switching on and setting up an add-on with no window, and with no display to reach

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 4.5.12 LTS from `/opt/blender/blender-4.5.12-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

**Installing from a file, and whether the switch-on survives.** The same small test add-on was built and installed:

`blender --command extension build --source-dir <source folder> --output-dir <output folder>`

```
building: smoozy_test-1.0.0.zip
complete
created: "<output folder>/smoozy_test-1.0.0.zip", 459
```

`blender --command extension install-file -r user_default -e <output folder>/smoozy_test-1.0.0.zip`

```
STATUS Installed "smoozy_test"
SMOOZY TEST ADDON REGISTERED
SMOOZY TEST ADDON UNREGISTERED
```

Exit status 0. A separate later run printed:

```
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

No preferences save was asked for. The built file is 459 bytes on both machines.

**With `--no-prefs`** the package was installed — `STATUS Installed "smoozy_test"` — and a later run printed the eight built-in modules with no `bl_ext.user_default.smoozy_test` among them, while `blender --command extension list` still showed `smoozy_test [installed]`.

**The switch-on, switch-off and refresh operators** all reported finished:

```
ENABLE {'FINISHED'}
AFTER ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
DISABLE {'FINISHED'}
REFRESH {'FINISHED'}
```

**`--addons` loads the add-on for that run only.** It printed the add-on's own `SMOOZY TEST ADDON REGISTERED` line, but the module list held only the eight built-in ones. Given a name that does not exist it printed `Add-on not loaded: "smoozy_test", cause: No module named 'smoozy_test'` and carried on, exit status 0.

**An add-on's own settings can be read, changed and made to stick.** Cycles' key was found by asking rather than guessing:

```
KEYS ['cycles']
PROPS ['rna_type', 'bl_idname', 'compute_device_type', 'devices', 'peer_memory', 'metalrt', 'use_hiprt', 'use_oneapirt', 'kernel_optimization_level']
DEVICE NONE
OFFERED []
```

The property list is identical to the Mac's. Two things are not. The current compute-device value reads **`NONE`** here, where the Mac read `METAL` — see question 4. And the offered list came back **empty on this machine too**, so that emptiness is not a fact about Apple's graphics.

Because no offered value could be read, a plain yes-or-no setting was used, set to the opposite of what it held:

```
BEFORE False
SET True
SAVE {'FINISHED'}
```

and a separate later run printed `AFTER True`. The same as the macOS run.

**Fetching from the internet is refused unless it is allowed**, in the same words: `RuntimeError: Operator bpy.ops.extensions.repo_sync_all.poll() Online access required to check for updates. Enable online access in System preferences`, then `Error synchronizing`, exit status 1.

**The new part: a Blender with no display to reach at all.** Every step above was run again with `DISPLAY` and `XAUTHORITY` unset, in a settings folder of its own. Installing, switching on, setting an add-on's own setting and saving it all worked, printing the same lines and exiting 0. Then Blender was started **without `--background`** and with no display:

`blender --python-expr "import bpy; print('RAN WITH A WINDOW')"`

It printed the display failure and did not print `RAN WITH A WINDOW`; wrapped in `timeout 180`, exit status 124 in Blender 4.5 and exit status 1 in 5.0, 5.1 and 5.2. **So the whole add-on cycle works with no display, as long as Blender is not asked to open a window; asked to open one it cannot, it stops.**

**Outside the interface.** Settled again on a second operating system: installing, switching on, switching off, and setting and saving an add-on's own settings all work with Blender driven entirely from outside its window, and also with no display for it to reach. The same two limits were seen: `--addons` does not add the add-on to the switched-on list, and Cycles' list of compute devices comes back empty.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed, except that the current compute-device value differs with the machine's graphics. **A computer with no display server at all is now tested too** — see the block below, run on a third machine that has none. Windows stays untested, and the manual's line about headless Windows systems was not reached at all.

### Settled by hand test on a machine with no display server at all

**Label: observed.** Run on **2026-08-06** on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, reached over SSH** — with Blender at `/opt/blender/blender-4.5.12-linux-x64/blender`, `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`. Every run pointed Blender at a fresh settings folder of its own through `BLENDER_USER_RESOURCES`, exported in the shell that ran Blender. **Nothing else was running on the machine.** This is a third machine; nothing here is merged with the Creator's Mac or with the one-card rented Linux machine. The full list is `Audits/Blender Research/Add-ons/Hand tests — two cards and no screen.md`.

**This is the claim item B of the Linux list could not make.** That machine had a display server running and Blender was merely cut off from it. This one has none, and nothing was installed or started to change that:

```
DISPLAY=[] WAYLAND_DISPLAY=[] XDG_SESSION_TYPE=[tty]
--- /tmp/.X11-unix:
total 8
drwxrwxrwt  2 root root 4096 Aug  6 10:11 .
drwxrwxrwt 11 root root 4096 Aug  6 11:38 ..
--- X server processes:
(none)
--- /dev/dri:
crw-rw----  1 root video  226,   0 Aug  6 10:11 card0
crw-rw----  1 root video  226,   1 Aug  6 10:11 card1
crw-rw----  1 root render 226, 128 Aug  6 10:11 renderD128
crw-rw----  1 root render 226, 129 Aug  6 10:11 renderD129
```

The X socket directory is empty, no X or Wayland process is running, and the session type is `tty`.

**The whole cycle worked.** A small extension of two files — a `blender_manifest.toml` and an `__init__.py` whose `register` prints a line — was built, installed, listed, switched on and off, refreshed, and had an add-on's own setting changed and saved. It is **not** the same package as the one the one-card machine used: it was written afresh for this machine and builds to 474 bytes where that one built to 459, so the two are never compared by size. Word for word:

```
building: smoozy_test-1.0.0.zip
complete
created: "/root/smoozy/no screen/work/addon_out/smoozy_test-1.0.0.zip", 474
build exit=0
STATUS Installed "smoozy_test"
SMOOZY TEST ADDON REGISTERED
SMOOZY TEST ADDON UNREGISTERED
install exit=0
  smoozy_test [installed]: "Smoozy Test", A tiny add-on used only for testing
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
ENABLE {'FINISHED'}
AFTER ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
DISABLE {'FINISHED'}
REFRESH {'FINISHED'}
```

The listing command also printed, before the package line:

```
[Errno 2] No such file or directory: '/root/smoozy/no screen/res-4.5/extensions/blender_org'
Repository data: /root/smoozy/no screen/res-4.5/extensions/blender_org not found, sync required!
```

**An add-on's own settings can be read, changed and made to stick, with no display server.** Cycles' key was found by asking rather than guessing:

```
KEYS ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
PROPS ['rna_type', 'bl_idname', 'compute_device_type', 'devices', 'peer_memory', 'metalrt', 'use_hiprt', 'use_oneapirt', 'kernel_optimization_level']
DEVICE NONE
BEFORE False
SET True
SAVE {'FINISHED'}
```

and a separate later run printed `AFTER True`. The property list is the same as on the other two machines. The setting used here is `use_hiprt`, a plain yes-or-no one, set to the opposite of what it held.

**Fetching from the internet is refused unless it is allowed**, in the same words as on the other machines:

```
RuntimeError: Operator bpy.ops.extensions.repo_sync_all.poll() Online access required to check for updates. Enable online access in System preferences
Error synchronizing
```

**`--addons` printed the add-on's own `SMOOZY TEST ADDON REGISTERED` line, and the module list held the same nine names.** *(Two-card citation check, 2026-08-06: this was headed "`--addons` loads the add-on for that run only", which its own record does not carry, and which **contradicts the one-card block earlier in this same document**. There, under the same heading, `--addons` printed the register line and "the module list held only the eight built-in ones" — the limit that block states in as many words. Here the list holds nine. The two results are recorded side by side and neither is chosen. What the record does show is that by this point the settings folder had already had the add-on installed and its preferences saved with it in the list, so nine names is what an ordinary run would print with no `--addons` at all, and nothing here separates the two. Nor was any later run made without `--addons` to show the add-on had gone. The claim has been reduced to what was printed, and the question sent back as a test.)*

**Observed on the four-card machine — 2026-08-06. Eight built-in add-ons or nine, and what decides the number.**

Run over SSH on a **third rented Linux machine — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, 197 GB of memory, no display server** — using Blender at `/opt/blender/blender-4.5.12-linux-x64/blender`, `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/four cards/addons-4.5`, a folder deleted and remade at the start of the run. Label: **observed**. This is a **fourth** machine; nothing here is merged with the Creator's Mac, the one-card rented Linux machine or the two-card rented Linux machine. The full list, with every command written out, is `Audits/Blender Research/Add-ons/Hand tests — four cards.md`.

**Nothing about this machine's cards bears on the answer.** Four cards, a smaller card and more memory are what it was rented for, and none of them touches add-ons: every run here is a windowless Blender printing a list of module names. What does bear on it is the settings folder, which is the point of the test.

**The test add-on is not either earlier machine's package.** It was written afresh with a different name and builds to 487 bytes, where the one-card machine's built to 459 and the two-card machine's to 474, so the three are never compared by size.

**The number of switched-on add-ons was read at nine points in one run**, from an emptied folder through installing, loading with `--addons`, switching on and saving. Word for word, and **identical in all four versions, name for name and count for count**:

```
--- 1. factory startup, nothing installed
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
COUNT 8
--- 2. a fresh settings folder, nothing installed
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
COUNT 8
--- 4. installing it with --no-prefs, so the saved preferences do not hold it
STATUS Installed "smoozy_four"
install exit=0
  smoozy_four [installed]: "Smoozy Four", A tiny add-on used only for testing
--- 5. a run with --addons, on a folder whose preferences do not hold the add-on
SMOOZY FOUR ADDON REGISTERED
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
COUNT 8
SMOOZY FOUR ADDON UNREGISTERED
--- 6. the very next run, without --addons
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
COUNT 8
--- 7. now switching it on and saving the preferences
SMOOZY FOUR ADDON REGISTERED
{'FINISHED'}
{'FINISHED'}
SMOOZY FOUR ADDON UNREGISTERED
--- 8. a run with --addons, on a folder whose preferences do hold the add-on
SMOOZY FOUR ADDON REGISTERED
MODULES ['bl_ext.user_default.smoozy_four', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
COUNT 9
SMOOZY FOUR ADDON UNREGISTERED
--- 9. the very next run, without --addons
SMOOZY FOUR ADDON REGISTERED
MODULES ['bl_ext.user_default.smoozy_four', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
COUNT 9
SMOOZY FOUR ADDON UNREGISTERED
```

**Four things follow, each shown by two steps of the run.**

**The built-in count is eight**, and the eight names are the same in all four versions. A fresh settings folder and `--factory-startup` agree.

**`--addons` loads the add-on for that run and does not put it in the switched-on list.** Step 5 printed the add-on's own register line — so it really was loaded — while the list still held only the eight built-in names.

**It really is for that run only.** Step 6, the very next run with no `--addons`, printed the same eight and no register line. That is the run neither earlier machine made, and it is the one that separates the two recorded results.

**Nine names means the saved preferences hold the add-on, and `--addons` has nothing to do with it.** After step 7 switched it on and saved, step 8 printed nine **with** the flag and step 9 printed nine **without** it.

**So the disagreement above is closed, and neither earlier result was about the machine.** The one-card machine printed eight because its folder's preferences did not hold the add-on; the two-card machine printed nine because by that point in its own run the add-on had been installed, switched on and saved. **The number is decided by the settings folder alone.** It is not a version difference either: all four versions printed the same numbers at every step. *(Four-card citation check, 2026-08-06: this whole run was made again on the same machine, which is still alive, with a freshly emptied settings folder per version and a differently named test add-on. Every one of the nine steps printed the same names and the same counts, in all four versions — eight through step 6, nine at steps 8 and 9, the add-on's register line at step 5 with the list still holding only the eight. The check added one step the run above did not make: the same used folder, whose saved preferences do hold the add-on, run with `--factory-startup`. That printed **eight** and no register line, in all four versions, and an ordinary run on the same folder printed nine three times over. So the sentence is true of ordinary runs; `--factory-startup` sets the folder aside and always gives eight.)*

**Driven from outside — this block.** Every run above was made from outside Blender's window, and on this machine there is no other way: with no display server there is nothing for a window to open on.

**Operating system — this block.** Linux, on the machine named at the top of it. macOS and Windows are untested for this particular run.



The build and the install each reported exit 0. *(Two-card citation check, 2026-08-06: this read "Every step exited 0." Two of the steps recorded above show an exit status — `build exit=0` and `install exit=0` — and the rest do not; the fetch from the internet raised a `RuntimeError` and printed `Error synchronizing`, and no status was written down for it. Narrowed to what the record shows.)*

**And then Blender was asked for a window on that machine.** `blender --python-expr "import bpy; print('RAN WITH A WINDOW')"` under `timeout 120` never printed `RAN WITH A WINDOW`. It printed:

```
GHOST: failed to initialize display for back-end(s): ['WAYLAND', 'X11']
  'WAYLAND': unable to connect to display!
  'X11': unable to open a display!
GHOST: unable to initialize, exiting!
```

on **standard error**, and then **did not end** — killed by a 120-second cut-off, exit status **124**. That is the same message and the same exit status the one-card machine gave when cut off from its display.

**What this settles.** **The whole add-on cycle — build, install, list, switch on, switch off, refresh, set an add-on's own setting, save it and read it back — works on a computer with no display server at all**, in this version, as long as Blender is not asked to open a window. Asked to open one it cannot, it says so and stops. That answers item B of the Linux list **for a computer with no display server running**, on Linux.

*(Two-card citation check, 2026-08-06: the last sentence read "That is item B of the Linux list answered for Linux." Item B is worded "a computer with no screen attached at all", and that is not what was observed. What the evidence above shows is that **no display server was running** — `DISPLAY` empty, no socket, no X or Wayland process, session type `tty`. Whether a monitor is physically plugged into either card was never looked at and cannot be from here. The two are not the same claim, and the difference is not a quibble in one direction: a machine can have a display server and no monitor, in which case Blender's window opens perfectly well. **The condition that decides what Blender does is the one that was tested** — with no display server there is nothing for a window to open on, whether or not a screen is attached — so the practical half of the question is settled and the words have been corrected to the tested ones.)*

**What it does not settle.** A computer with **no graphics hardware at all**: this machine has two graphics cards, it simply has no display server on top of them. And the manual's line about headless *Windows* systems, which needs a Windows computer.

**Outside the interface.** Every run in this block was made from outside Blender's window, and on this machine there is no other way: with no display server there is no window to be inside. So this block speaks for the outside case alone, and none of the window tests the one-card machine ran could be repeated here. *(Two-card citation check, 2026-08-06: added by this check — the block carried neither standing line of its own, where the two blocks above it each carry both.)*

**Operating system.** Linux, on the machine named at the top of this block. It says nothing about macOS, where the project's only machine has a display server, and nothing about Windows, which no Windows machine has run. *(Two-card citation check, 2026-08-06: added by this check.)*


> Can an add-on be installed, switched on, and given whatever else it needs before it will work, on a computer with no screen attached?

**LABEL: documented**, for install and switch-on. **Partly not in the sources**, for "whatever else it needs" — see below.

### Running with no screen

> "One advantage of using the command line is that we do not need a graphical display
> (no need for X server on Linux for example)
> and consequently we can render via a remote shell (typically SSH)."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — read 2026-07-30

> "-b, --background
>
> Run in background (often used for UI-less rendering)."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30

### Installing

There is a dedicated command-line facility for it, and the manual says so plainly:

> "Extensions can be built, validated & installed via command-line."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html — read 2026-07-30

The subcommands are documented:

> "install:
>
> Install packages.
>
> install-file:
>
> Install package from file."

and their usage:

> "blender --command extension install [-h] [-s] [-e] [--no-prefs]
> PACKAGES"

> "blender --command extension install-file [-h] -r REPO [-e] [--no-prefs]
> FILE"

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html — read 2026-07-30

And `--command` does not need a window at all:

> "-c, --command <command>
>
> Run a command which consumes all remaining arguments.
> Use -c help to list all other commands.
> Pass --help after the command to see its help text.
>
> This implies --background mode."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30

### Switching on

Three documented routes, none of them requiring a person at a screen.

*As part of installing:*

> "-e, --enable
>
> Enable the extension after installation."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html — read 2026-07-30

*At launch, per run:*

> "--addons <addon(s)>
>
> Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30

*From a script, including a script placed in a system directory so it applies to everyone:*

> "For example, a script can enable add-ons for every user.
>
> $BLENDER_SYSTEM_SCRIPTS/startup/enable_addons.py
>
> def register():
> import addon_utils
> addon_utils.enable("my-addon")"

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30

There is also an operator, `bpy.ops.preferences.addon_enable(*, module='')` — "Turn on this add-on" — Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.ops.preferences.html — read 2026-07-30. The sources do not say whether that operator works in background mode; see "Not covered".

Installation can also be sidestepped entirely by placing files in a directory Blender reads:

> "Extensions packages should be extracted in this system directory,
> with a resulting path like this:
>
> $BLENDER_SYSTEM_EXTENSIONS/system/my-addon/blender_manifest.toml"

and

> "Add-ons that have not been converted to become an extension yet need
> to be placed in the addons script directory.
>
> For example, an add-on could be located at:
>
> $BLENDER_SYSTEM_SCRIPTS/addons/simple_addon.py
> $BLENDER_SYSTEM_SCRIPTS/addons/complex_addon/__init__.py"

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30

### "Whatever else it needs"

**Partly documented, partly not in the sources.**

*What is documented.* Preferences are reachable and writable from outside the interface: the install subcommands carry

> "--no-prefs
>
> Treat the user-preferences as read-only,
> preventing updates for operations that would otherwise modify them.
> This means removing extensions or repositories for example, wont update the user-preferences."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html — read 2026-07-30

Written the other way round: without that flag, these operations *do* update the preferences. That is my reading of the sentence, and it is a short step, so treat it as inferred.

Preferences can also be written from a script: `bpy.ops.wm.save_userpref()` — "Make the current preferences default" — Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.ops.wm.html — read 2026-07-30. And the manual says they save on their own by default:

> "By default changes to preferences are saved on exit,
> this allows changes to the keymap and Quick Favorites menu to be stored and used between
> Blender Sessions."

Source: Manual — https://docs.blender.org/manual/en/4.5/editors/preferences/introduction.html — read 2026-07-30

The manual is explicit that the Python API can set preferences up for a deployment:

> "The Blender Python API can be used to customize Blender. This includes
> changing preferences, changing the startup file and adding UI elements."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30

An add-on can also take settings from the command line itself. **Observed**, in Cycles:

> "    parser.add_argument("--cycles-device",
>                         help="Set the device to use for Cycles, overriding user preferences and the scene setting."
>                              "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
>                              "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
>                         default=None)"

File: `cycles_4.5/engine.py`, lines 16–20 — read 2026-07-30

and Cycles is handed the fact that Blender is running in the background:

> "    _cycles.init(path, user_path, bpy.app.background)"

File: `cycles_4.5/engine.py`, line 52 — read 2026-07-30. (*Citation check, 2026-08-05: this said line 51, which is blank. What the line shows is that `bpy.app.background` is passed in; what Cycles then does with it is not shown here.*)

*Where it breaks.* Two documented things an add-on may want are not available with no interface.

> "# Note that in background mode (no GUI available), keyconfigs are not available either,
> # so we have to check this to avoid nasty errors in background case.
> kc = wm.keyconfigs.addon
> if kc:"

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/scripting/addon_tutorial.html — read 2026-07-30

> "# Lazily import GPU module, so that the render engine works in
> # background mode where the GPU module can't be imported by default."

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html — read 2026-07-30

**Not covered.** The sources never say whether `bpy.ops.preferences.addon_install`, `bpy.ops.preferences.addon_enable` or the `bpy.ops.extensions.*` operators succeed in background mode — the API documentation lists them without a note either way, and the manual's operator gotchas page discusses context failures in general terms only. The sources also do not describe what an add-on's *own* preferences look like when set from a script, nor whether every add-on's setup can be completed without an interface — Cycles' device list, for one, is built by querying the machine, which the sources do not describe as scriptable. And nothing states that "no screen attached" is identical to "background mode"; I have treated the manual's "we do not need a graphical display" as covering it.

**Outside the interface.** This question is *about* outside the interface, and the answer above is the outside-the-interface answer: yes for install and enable, with the two documented gaps (key-maps, GPU module). Inside the interface the same operations exist as buttons — "Install from Disk", the enable checkbox — Source: Manual — https://docs.blender.org/manual/en/4.5/editors/preferences/extensions.html and https://docs.blender.org/manual/en/4.5/editors/preferences/addons.html — read 2026-07-30.

**Operating system.** One documented difference, and it is about rendering rather than installing:

> "Headless Rendering
>
> Headless rendering is not supported on headless Windows systems."

Source: Manual — https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html — read 2026-07-30
That sentence sits on the EEVEE limitations page, so on its face it is about EEVEE. The "no need for X server" sentence names Linux specifically. Beyond those two, the sources say nothing about operating-system differences here.

### Where I looked, for question 3

Searched the whole of `manual_4.5` and `api_4.5` for, case-insensitive: `headless`, `no display`, `without a display`, `X server`, `Xorg`, `EGL`, `virtual framebuffer`, `screen attached`, `background mode`, `background-mode`, `in background`, `GUI`, `OpenGL`, `GPU driver`, `addon_utils`, `addon_enable`, `addon_install`, `--addons`, `--command`, `save_userpref`, `read_userpref`, `Auto-Save Preferences`, `Save Preferences`, `--no-prefs`, `offline-mode`, `online-mode`, `factory-startup`.

Pages opened: manual — `advanced/command_line/render.txt`, `advanced/command_line/arguments.txt` (whole file scanned by section), `advanced/command_line/extension_arguments.txt` (whole), `advanced/deploying_blender.txt` (whole), `advanced/blender_directory_layout.txt` (whole), `advanced/extensions/getting_started.txt` (whole), `advanced/extensions/addons.txt` (whole), `advanced/scripting/addon_tutorial.txt` (sections), `editors/preferences/extensions.txt` (whole), `editors/preferences/addons.txt` (whole), `editors/preferences/introduction.txt` (the defaults menu), `render/eevee/limitations/limitations.txt` (the headless section). API — `bpy.ops.preferences.txt` (whole), `bpy.ops.extensions.txt` (whole), `bpy.ops.wm.txt` (searched), `bpy.types.RenderEngine.txt`, `info_tips_and_tricks.txt` (the "Use Blender without it's User Interface" section), `info_gotchas_operators.txt` (whole), `bpy.utils.txt`. Cycles — `engine.py` (lines 1–60), `properties.py` (the `CyclesPreferences` class). Image opened: `_images/advanced_deploying-blender_system-extensions.png`, which shows a Repositories dropdown listing `extensions.blender.org`, `System` and `User Default`, each with a tick box — **documented**, since it is part of the manual.

Ruled out: `manual_4.5/advanced/command_line/launch/{linux,windows,macos}.txt` — these describe how to start Blender from a terminal on each system, not what a screenless machine can do with add-ons. `troubleshooting/3d_view.txt` and `troubleshooting/gpu/` — about display drivers for the viewport, not about add-on setup.

---

## Question 4

### Settled by hand test — what happens on a kind of computer an add-on does not name, and which bundled modules land on disk

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 4.5.12. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

**A package naming only another kind of computer is refused, and nothing is installed.** Two test add-ons were built, identical but for their manifests: one with `platforms = ["windows-x64"]`, one with `platforms = ["macos-x64"]`. Both built without complaint. Installing the first:

`blender --command extension install-file -r user_default -e smoozy_win_only-1.0.0.zip`

printed:

```
ERROR smoozy_win_only: This platform (macos-arm64) isn't one of (windows-x64)
Package should have been installed but not found: smoozy_win_only
```

and the second:

```
ERROR smoozy_macx64_only: This platform (macos-arm64) isn't one of (macos-x64)
Package should have been installed but not found: smoozy_macx64_only
```

**Both runs exited 0** despite the error. `blender --command extension list` afterwards showed the User Default repository with nothing in it, and the switched-on list held only the eight built-in add-ons. So `macos-x64` is refused on an Apple Silicon Mac exactly as `windows-x64` is.

**The same package, placed straight into the extensions folder, is listed but cannot be switched on.** The Windows-only add-on's folder — its `__init__.py` and its manifest — was copied into `<settings folder>/extensions/user_default/smoozy_win_only/`. Then:

`blender --command extension list`

```
Repository: "extensions.blender.org" (id=blender_org)
Repository: "User Default" (id=user_default)
  smoozy_win_only [installed]: "Smoozy Windows Only", A test add-on naming one platform
Repository: "System" (id=system)
```

and switching it on:

`blender --background --python-expr "import bpy; print('ENABLE', bpy.ops.preferences.addon_enable(module='bl_ext.user_default.smoozy_win_only')); print('MODULES', sorted(a.module for a in bpy.context.preferences.addons))"`

printed:

```
RuntimeError: Extension bl_ext.user_default.smoozy_win_only is incompatible (This platform (macos-arm64) isn't one of (windows-x64))
```

twice — once raised inside `addon_utils.py` and once as the operator's own error — followed by:

```
Error: Extension bl_ext.user_default.smoozy_win_only is incompatible (This platform (macos-arm64) isn't one of (windows-x64))
```

Exit status 0. So the check happens when Blender scans the folder as well as when it installs.

**A repository built from a folder is filtered by Blender itself.** A listing was generated from a folder holding both the plain test package and the Windows-only one:

`blender --command extension server-generate --repo-dir=<packages folder>`

printed `found 2 packages.` and wrote an `index.json` naming both, the Windows-only one carrying `"platforms": ["windows-x64"]`. That folder was added as a repository with a `file://` address and synced. `blender --command extension list` then showed:

```
Repository: "Smoozy Local" (id=smoozy_local)
  smoozy_test: "Smoozy Test", A tiny test add-on
```

— only the compatible one. `blender --online-mode --command extension install smoozy_win_only -e` printed:

```
Package "smoozy_win_only" not found in remote repositories!
```

exit status 1, while `blender --online-mode --command extension install smoozy_test -e` printed `STATUS Installed "smoozy_test"`, exit status 0.

**Only the matching bundled module is unpacked.** A test add-on was built carrying three bundled Python modules of the same library, one each for macOS on Apple Silicon, Linux and Windows, named in its manifest as:

```
wheels = [
  "./wheels/pillow-12.3.0-cp311-cp311-macosx_11_0_arm64.whl",
  "./wheels/pillow-12.3.0-cp311-cp311-manylinux_2_27_x86_64.manylinux_2_28_x86_64.whl",
  "./wheels/pillow-12.3.0-cp311-cp311-win_amd64.whl",
]
```

After installing it, the add-on's own folder held **all three** files, unopened, in a `wheels` folder. But the unpacked code sat somewhere else and was for this machine only — under `<settings folder>/extensions/.local/lib/python3.11/site-packages/`, whose compiled parts were named:

```
_imagingmath.cp311-darwin.so
_imaging.cp311-darwin.so
_webp.cp311-darwin.so
_imagingcms.cp311-darwin.so
_imagingft.cp311-darwin.so
_imagingmorph.cp311-darwin.so
_imagingtk.cp311-darwin.so
_avif.cp311-darwin.so
```

with a `.dylibs` folder beside them. No Windows or Linux compiled file was unpacked anywhere. The module could then be used: `blender --background --python-expr "import PIL; print('PIL', PIL.__file__, PIL.__version__)"` printed the path above and `12.3.0`.

**Building one package per kind of computer needs the manifest to name them.** `blender --command extension build --split-platforms` on the manifest above, which named no platforms, printed:

```
FATAL_ERROR: Error in arguments "--split-platforms" with a manifest that does not declare "platforms"
```

exit status 1. With `platforms = ["windows-x64", "macos-arm64", "linux-x64"]` added, the same command printed:

```
building: smoozy_wheels-1.0.0-windows_x64.zip
complete
building: smoozy_wheels-1.0.0-macos_arm64.zip
complete
building: smoozy_wheels-1.0.0-linux_x64.zip
complete
```

**Outside the interface.** Settled: the platform check, the message it gives and the folder filtering all happen with Blender driven from outside its window, and the message goes to the terminal rather than to a window. The exit status does not report the refusal — it stays 0.

**Operating system.** Settled from the macOS side only. What a **Linux** machine does with a package naming another kind, and what carrying an installed add-on across to a machine of a different kind does, were both run afterwards on a rented Linux machine and have their own block under this question; the two machines' results are never merged. Windows stays untried: no Windows machine has run it.

### Settled by hand test on the rented Linux machine — what happens on a kind of computer an add-on does not name, which bundled modules land on disk, and what a compiled part for the wrong machine does

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 4.5.12 LTS from `/opt/blender/blender-4.5.12-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

**A package naming only another kind of computer is refused, and this machine can pose the case the Mac could not.** Four test add-ons were built, identical but for their manifests: `platforms = ["windows-x64"]`, `["macos-x64"]`, `["macos-arm64"]` and `["linux-x64"]`. All four built without complaint. Installing each with `blender --command extension install-file -r user_default -e <zip>` printed:

```
ERROR smoozy_win_only: This platform (linux-x64) isn't one of (windows-x64)
Package should have been installed but not found: smoozy_win_only
```

```
ERROR smoozy_macx64_only: This platform (linux-x64) isn't one of (macos-x64)
Package should have been installed but not found: smoozy_macx64_only
```

```
ERROR smoozy_macarm_only: This platform (linux-x64) isn't one of (macos-arm64)
Package should have been installed but not found: smoozy_macarm_only
```

```
STATUS Installed "smoozy_linux_only"
SMOOZY TEST ADDON REGISTERED
SMOOZY TEST ADDON UNREGISTERED
```

**All four runs exited 0**, the refusals included. Afterwards `blender --command extension list` showed only `smoozy_linux_only [installed]` under User Default, and the switched-on list held that module and the eight built-in ones.

So the sentence is the same on both machines and names the machine it is running on: `macos-arm64` on the Mac, `linux-x64` here. **An add-on built for a Mac is refused on Linux exactly as an add-on built for Windows was refused on the Mac**, and an add-on built for Linux installs here without complaint. The second line of the refusal is the same in this version on both machines.

**The same package's folder placed straight into the extensions directory** is listed but cannot be switched on:

```
Repository: "User Default" (id=user_default)
  smoozy_win_only [installed]: "Smoozy Windows Only", A test add-on naming one platform
```

and switching it on printed `RuntimeError: Extension bl_ext.user_default.smoozy_win_only is incompatible (This platform (linux-x64) isn't one of (windows-x64))` twice — once raised inside `addon_utils.py` and once as the operator's own error — followed by the same sentence as `Error:`. Exit status 0. The same as the macOS run.

**A repository built from a plain folder is filtered by Blender itself.** `server-generate` on a folder holding both the plain package and the Windows-only one printed `found 2 packages.` and wrote an `index.json` naming both, the Windows-only one carrying `"platforms": ["windows-x64"]`. After adding that folder as a repository by `file://` address and syncing, `blender --command extension list` showed only:

```
Repository: "Smoozy Local" (id=smoozy_local)
  smoozy_test: "Smoozy Test", A tiny test add-on
```

and `blender --online-mode --command extension install smoozy_win_only -e` printed `Package "smoozy_win_only" not found in remote repositories!`, exit status 1. Installing `smoozy_test` printed `STATUS Installed "smoozy_test"`, exit status 0. The same as the macOS run, with this machine's own platform doing the filtering.

**Only the matching bundled module is unpacked, and here that means the Linux one.** A test add-on was built carrying three bundled Python modules of the same library, one each for macOS on Apple Silicon, Linux and Windows, named in its manifest as `cp311` files. After installing it, the add-on's own folder held **all three** files, unopened, in a `wheels` folder:

```
pillow-12.3.0-cp311-cp311-macosx_11_0_arm64.whl
pillow-12.3.0-cp311-cp311-manylinux_2_27_x86_64.manylinux_2_28_x86_64.whl
pillow-12.3.0-cp311-cp311-win_amd64.whl
```

The unpacked code sat somewhere else and was for this machine only — under `<settings folder>/extensions/.local/lib/python3.11/site-packages/`, whose compiled parts were named:

```
_avif.cpython-311-x86_64-linux-gnu.so
_imaging.cpython-311-x86_64-linux-gnu.so
_imagingcms.cpython-311-x86_64-linux-gnu.so
_imagingft.cpython-311-x86_64-linux-gnu.so
_imagingmath.cpython-311-x86_64-linux-gnu.so
_imagingmorph.cpython-311-x86_64-linux-gnu.so
_imagingtk.cpython-311-x86_64-linux-gnu.so
_webp.cpython-311-x86_64-linux-gnu.so
```

with a `pillow.libs` folder beside them. No macOS or Windows compiled file appeared anywhere. The module could then be used with no window: `import PIL` printed the path above and `12.3.0`, and `from PIL import Image; print('IMAGE OK', Image.new('RGB',(2,2)).size)` printed `IMAGE OK (2, 2)`. This is the exact mirror of the macOS run, which unpacked only the macOS files.

**Building one package per kind of computer** still refuses a manifest that names no platforms: `FATAL_ERROR: Error in arguments "--split-platforms" with a manifest that does not declare "platforms"`, exit status 1.

**The new part: an add-on whose only compiled part is built for another kind of computer.** Two more packages were built, one whose bundled-module list names **only** the macOS file, one whose list names **only** the Windows file. Neither manifest names any `platforms`, so nothing stops either being installed. On this Linux machine:

```
STATUS Installed "smoozy_macwheel"
SMOOZY TEST ADDON REGISTERED
SMOOZY TEST ADDON UNREGISTERED
```

Exit status 0, **no warning of any kind**. The package is listed as installed, and its module is in the switched-on list:

```
MODULES ['bl_ext.user_default.smoozy_macwheel', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

Listing the extensions folder shows the wheel still sitting unopened in the add-on's own folder, and **no `.local` folder at all** — nothing was unpacked:

```
<settings folder>/extensions/user_default/smoozy_macwheel/__init__.py
<settings folder>/extensions/user_default/smoozy_macwheel/blender_manifest.toml
<settings folder>/extensions/user_default/smoozy_macwheel/wheels
<settings folder>/extensions/user_default/smoozy_macwheel/wheels/pillow-12.3.0-cp311-cp311-macosx_11_0_arm64.whl
```

And using it fails:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
ModuleNotFoundError: No module named 'PIL'
```

**Exit status 0.** The Windows-only package behaved the same way in every respect. So an add-on carrying a compiled part built for another kind of computer **installs, switches on, and is simply missing what it needs**, with nothing said at install time and nothing in the exit status.

**The new part: carrying an installed, set-up add-on to a computer of a different kind.** The whole settings folder holding the installed three-module add-on was packed on this Linux machine, copied to the Creator's MacBook Pro (M1 Pro chip, macOS 26.5.2), and that Mac's Blender 4.5.12 LTS was pointed at it. The Mac listed it as installed:

```
Repository: "User Default" (id=user_default)
  smoozy_wheels [installed]: "Smoozy Wheels", A test add-on carrying bundled Python modules
```

and had it switched on:

```
VERSION 4.5.12 LTS
MODULES ['bl_ext.user_default.smoozy_wheels', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

`import PIL` succeeded and printed the version, `12.3.0`. But using it failed:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
  File "<settings folder>/extensions/.local/lib/python3.11/site-packages/PIL/Image.py", line 95, in <module>
    from . import _imaging as core
ImportError: cannot import name '_imaging' from 'PIL' (<settings folder>/extensions/.local/lib/python3.11/site-packages/PIL/__init__.py)
```

**Exit status 0.** Looking on disk, the unpacked compiled files were still the Linux ones, `_imaging.cpython-311-x86_64-linux-gnu.so` and the rest; nothing had gone back to the add-on's own folder to unpack the macOS file that is sitting there. So the install, the switch-on and the plain Python travel; the compiled part does not, and nothing announces it until the add-on tries to use it.

This one test was run on both machines by design — packed on Linux, opened on macOS — and is written here as one crossing rather than as two results.

**Outside the interface.** Settled: the platform check, the message it gives, the folder filtering, which bundled module is unpacked, and everything about the wrong-machine compiled part all happen with Blender driven from outside its window, and the message goes to the terminal. The exit status never reports any of it — it stays 0.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed on the shape of the answer, each naming its own kind: `macos-arm64` there, `linux-x64` here. A package built for a Mac is refused on Linux, which the Mac could not show. Carrying an installed add-on from Linux to macOS was run once, across the two machines, and is recorded above. Windows stays untried: no Windows machine has run it.

### Settled by hand test on the rented Linux machine — installing a package for another kind of computer at the window, and what Cycles' own panel offers

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 4.5.12 LTS, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`. Fresh settings folders, preferences saved once beforehand.

**Installing a Windows-only package through the window.** A script opened the Preferences editor at its Get Extensions section, read the install operator's own list of arguments so that none was guessed —

```
OPERATOR PROPS ['directory', 'enable_on_install', 'filepath', 'files', 'filter_glob', 'overwrite', 'repo', 'rna_type', 'target', 'url']
```

— and then called it with the package's path and the user repository. It printed:

```
Package should have been installed but not found: smoozy_win_only
INSTALL RESULT {'FINISHED'}
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

So the operator reported **finished** while refusing the package. Nothing was installed and nothing was switched on. The picture, taken with the Add-ons section then showing, holds the fourteen add-ons that ship with this Blender and **no "Smoozy Windows Only" anywhere in the list**. That settles a gap the Creator's macOS run left open: the refused package does not appear in the Add-ons list afterwards.

**What Cycles' own panel offers on a machine with an NVIDIA card.** The picture of the opened Cycles row in the Add-ons preferences shows a section headed **Preferences** holding one row of buttons: **`None`**, **`CUDA`**, **`OptiX`**, **`HIP`**, **`oneAPI`**, with `None` selected. The same row appears in the System preferences under a heading **Cycles Render Devices**. On the Creator's Mac the same setting reads `METAL`.

**And the buttons are not the property's own list of values.** Asked from Python on this machine, with a window open and without, the offered list came back empty:

```
DEVICE NONE
OFFERED []
```

while `refresh_devices()` then filled the devices collection with this machine's own hardware:

```
DEVICES [('NVIDIA GeForce RTX 4090', 'CUDA', True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', False), ('NVIDIA GeForce RTX 4090', 'OPTIX', True)]
```

So the row of buttons is drawn by the add-on's own interface code, not from the setting's list of values, and what a script can read about it is not what a person sees.

**Outside the interface.** The install refusal at the window matches the one from outside the window under question 4 above, word for word in its first line. The compute-device buttons are only visible at the window; from outside it the offered list is empty.

**Operating system.** Settled for Linux in this version, at the window. The Creator's macOS run covered Blender 5.2 only. The two machines offer different compute devices, which is the machine's graphics rather than the operating system. Windows stays untested.


> What does an add-on record about the kinds of computer it is meant for, what happens on one it does not name, and does an add-on set up on one computer still work when copied to a computer of a different kind?

### What it records

**LABEL: documented.**

An extension's manifest has an optional `platforms` field.

> "platforms:
>
> List of supported platforms. If omitted, the extension will be available in all operating systems.
> The available options are
> ["windows-x64", "windows-arm64", "macos-x64", "macos-arm64", "linux-x64"]"

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html — read 2026-07-30

The same file's example manifest shows it commented out, with the same five names split across two lines:

> "# # Optional: list of supported platforms. If omitted, the extension will be available in all operating systems.
> # platforms = ["windows-x64", "macos-arm64", "linux-x64"]
> # # Other supported platforms: "windows-arm64", "macos-x64""

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html — read 2026-07-30

Note what these names hold: an operating system *and* a processor kind. Each of the five is one pair.

Alongside `platforms`, a manifest records the Blender versions it is for — a different axis, but part of "the kinds of computer" only in the loosest sense, so I note it rather than dwell on it:

> "blender_version_min:
>
> Minimum supported Blender version - use at least 4.2.0."
> "blender_version_max:
>
> Blender version that the extension does not support, earlier versions are supported."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html — read 2026-07-30

A separate record exists for bundled Python modules, which carry their own platform names inside their filenames:

> "wheels = [
> "./wheels/pillow-10.3.0-cp311-cp311-macosx_11_0_arm64.whl",
> "./wheels/pillow-10.3.0-cp311-cp311-manylinux_2_28_x86_64.whl",
> "./wheels/pillow-10.3.0-cp311-cp311-win_amd64.whl",
> ]"

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/python_wheels.html — read 2026-07-30

A legacy add-on's `bl_info` records no platform at all — see the tutorial's simplest add-on, which has only `name`, `blender` and `category` — Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/scripting/addon_tutorial.html — read 2026-07-30. Cycles' own `bl_info` likewise names no platform (**observed**, `cycles_4.5/__init__.py`, lines 7–16).

### What happens on a kind it does not name

**LABEL: inferred**, and weakly. This is the part the sources are thinnest on.

The strongest statement is the negative one, quoted twice above: "If omitted, the extension will be available in all operating systems." Read the other way, when `platforms` *is* present the extension is available only in the operating systems it names. That reversal is my step, not the manual's words, so: inferred.

One further piece points the same way. When Blender asks a repository for its list, it says which machine it is:

> "When Blender fetches the extensions listing it passes the following arguments to make sure only
> compatible extensions are listed:
>
> platform
>
> blender_version"

with the example query

> "https://extensions.blender.org/api/v1/extensions/?blender_version=4.2.0&platform=linux-x64"

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/creating_repository/dynamic_repository.html — read 2026-07-30

So for a *remote* listing, an extension for another platform is filtered out before it is ever shown. That is documented for dynamic repositories.

**Not covered, and this is the sharp edge of this answer.** The sources never say what happens if such a package is installed anyway — from a file, from a static repository, or by being dropped into a system directory. No error message, no refusal, no warning is described. I searched hard for this (terms listed below) and found nothing.

For the one case where the sources *do* describe a failure, the cause is the bundled binary rather than the manifest:

> "While not common practice, Python add-ons can be distributed with their own precompiled libraries. Unlike regular
> Python scripts, these are not portable between different platforms.
>
> It is possible the library is incompatible with your Blender installation (attempting to load a library built for a
> different version of Python, or loading a 32-bit library on a 64-bit system).
>
> If the add-on contains .pyd or .so files, check that the distribution is compatible with your operating
> system."

Source: Manual — https://docs.blender.org/manual/en/4.5/troubleshooting/python.html — read 2026-07-30

And a documented symptom, on Windows:

> "If Python is raising errors or you have an add-on that just fails when enabled with an error – e.g: … is not a
> valid Win32 application."

Source: Manual — https://docs.blender.org/manual/en/4.5/troubleshooting/python.html — read 2026-07-30

### Copied to a computer of a different kind

**LABEL: inferred** for the answer; **documented** for the pieces.

Two things travel badly, and one thing does not travel at all.

*Compiled parts do not travel.* The quotation above — "Unlike regular Python scripts, these are not portable between different platforms" — is as direct as the sources get. A pure-Python add-on carries no such restriction in anything I read.

*Where an add-on lives differs by operating system.* The user directory is:

> "Linux
>
> $HOME/.config/blender/4.5/"
> "macOS
>
> /Users/$USER/Library/Application Support/Blender/4.5/"
> "Windows
>
> %USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\4.5\"

and within it:

> "./extensions
>
> Extension repositories.
>
> Located in both user and system directories. Repositories are loaded from
> both directories."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html — read 2026-07-30

So a straight file copy from one kind of computer to another lands in the wrong place unless the copier knows the layout. That is my inference from the three different paths.

*Some of the setup is about the machine, not the add-on.* **Observed**, Cycles chooses its default compute device by looking at the machine it is on:

> "    @staticmethod
>     def default_device():
>         import platform
>         # Default to selecting the Metal compute device on Apple Silicon GPUs
>         # (drivers are tightly integrated with macOS so pose no stability risk)
>         if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):
>             return 5
>         return 0"

File: `cycles_4.5/properties.py`, lines 1558–1565 — read 2026-07-30

and it builds the list of offered device types from what the build supports:

> "    def get_device_types(self, context):
>         import _cycles
>         has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()
>
>         list = [('NONE', "None", n_("Do not use compute device"), 0)]
>         if has_cuda:"

File: `cycles_4.5/properties.py`, lines 1567–1572 — read 2026-07-30. (*Citation check, 2026-08-05: the quote stopped at line 1569, which shows only the query. The list being built from the result starts at line 1571, so the quote has been extended to cover the claim.*)

**Inferred from that:** an add-on's stored settings can name things that do not exist on another kind of computer (a Metal device on a machine with no Metal), so copying settings across is not the same as copying the add-on. The step from "the list is built from the machine" to "settings may not carry over" is mine.

*One thing that is portable by design.* Blender itself:

> "Whether it's on a USB stick, sitting on a folder on your desktop, or fully installed, Blender runs out of the box.
>
> No installation needed, no Internet connection required. Truly portable, take it with you wherever you go!"

Source: Blender requirements page, `requirements/requirements_2026-08-01.txt`, lines 7–9 — read 2026-07-30. This page names no version at that point, so I record it without claiming it for 4.5 specifically. The manual makes the version-neutral equivalent point about a `portable` folder holding "preferences, startup file, installed extensions and presets" — Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html — read 2026-07-30.

**Not covered.** The sources do not say whether an add-on installed on Windows and copied wholesale to Linux is *detected* as wrong, only that compiled parts will not load. They do not say what a `--split-platforms` build does if the wrong one is installed. They do not describe migrating add-on preferences between machines. And `windows-arm64` appears in the platform list while the requirements page's system tables do not mention Windows on ARM at all — I cannot reconcile those from the sources, and I do not try to.

**Outside the interface.** The `platforms` field is manifest text and applies wherever the manifest is read; nothing suggests otherwise. The remote-listing filter is described for Blender fetching a listing, which the command line also does (`blender --command extension sync` — "Download package information for remote repositories" — Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html — read 2026-07-30). But the sources do not state that the platform filter applies identically to the command-line path. Unsettled.

**Operating system.** This whole answer is about operating systems. The named ones are Windows, macOS and Linux, on the five platform identifiers quoted. Separately, the requirements page names 4.5 in so many words and states its macOS requirements:

> "Blender 4.5 LTS supports Intel chips, macOS 11.2 (Big Sur), requires GPU with Metal 2.2 (AMD, Apple Silicon, or Intel Skylake and newer)"

Source: `requirements/requirements_2026-08-01.txt`, line 92 — read 2026-07-30. (*Citation check, 2026-08-05: this used to say the page "confirms" the operating systems named by the five platform identifiers. It does not — it is a macOS support statement. Narrowed to what the sentence carries.*)

### Where I looked, for question 4

Searched the whole of `manual_4.5`, `api_4.5` and `release_notes_4.5` for, case-insensitive: `windows-x64`, `windows-arm64`, `macos-x64`, `macos-arm64`, `linux-x64`, `windows_x64`, `macos_x64`, `platform`, `platforms`, `operating system`, `architecture`, `arm64`, `x86`, `64-bit`, `incompatible`, `unsupported`, `not supported`, `not compatible`, `not portable`, `portable between`, `cross-platform`, `copied to another`, `another computer`, `other computers`, `filtered out`, `not be listed`, `will not be shown`, `not available in`, `blender_version_min`, `blender_version_max`, `.pyd`, `.so`, `wheel`.

Every hit for the five platform identifiers across the whole of `manual_4.5` is accounted for: `advanced/extensions/getting_started.txt` (lines 89, 190), `advanced/extensions/python_wheels.txt` (lines 89, 98, 100), `advanced/extensions/creating_repository/static_repository.txt` (lines 87, 91), `advanced/extensions/creating_repository/dynamic_repository.txt` (line 32). There are none anywhere in `api_4.5` — the Python API documentation never mentions extension platforms. `release_notes_4.5` mentions "platform" only for EEVEE shader compilation, big-endian deprecation, pen tilt and a build section — none of them add-on platform declarations.

Pages opened in full: `advanced/extensions/getting_started.txt`, `advanced/extensions/python_wheels.txt`, `advanced/extensions/creating_repository/static_repository.txt`, `advanced/extensions/creating_repository/dynamic_repository.txt`, `advanced/blender_directory_layout.txt`, `troubleshooting/python.txt`, `advanced/extensions/version_number_guidelines.txt` (start), `requirements/requirements_2026-08-01.txt` (whole). Cycles — `properties.py`, the `CyclesPreferences` class in full.

Ruled out: `editors/preferences/system.txt` and `editors/preferences/viewport.txt` carry platform-specific notes ("This option is not available on macOS…", "not supported on Qualcomm GPUs on Windows") but these are about Blender's own settings, not about add-ons. `troubleshooting/gpu/` covers graphics drivers, not add-on portability.

---

## Question 11

### Settled by hand test — what can be read about what an add-on needs

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 4.5.12. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

**The two spellings of the online-access property.** The manual spells one `bpy.app.online_access_overriden` and the API documentation spells it `online_access_override`. Asked directly:

`blender --background --python-expr "import bpy; print('ACCESS', bpy.app.online_access); print('HAS_OVERRIDE', hasattr(bpy.app,'online_access_override')); print('HAS_OVERRIDEN', hasattr(bpy.app,'online_access_overriden')); print('PREF', bpy.context.preferences.system.use_online_access)"`

printed:

```
ACCESS False
HAS_OVERRIDE True
HAS_OVERRIDEN False
PREF False
```

So only `online_access_override` exists. Started with `--online-mode` the same reads printed `ACCESS True`, `OVERRIDE True`, `PREF False`; started with `--offline-mode`, `ACCESS False`, `OVERRIDE True`, `PREF False`. The preference itself is never changed by either switch.

**`extension validate` reports nothing about what an add-on needs.** Run on the source folder and again on the built package:

`blender --command extension validate <source folder>`
`blender --command extension validate <package>.zip`

each printed exactly one line:

```
Success parsing TOML in "<the path given>"
```

exit status 0 in both cases. No field of the manifest is printed back, so this command cannot be used to read an add-on's declared needs.

**A bundled module is unpacked into a shared place, not into the add-on's own folder.** An add-on carrying three bundled modules of one library — one each for macOS on Apple Silicon, Linux and Windows — was installed. All three files stayed unopened in the add-on's own `wheels` folder, while the unpacked code went to `<settings folder>/extensions/.local/lib/python3.11/site-packages/`, and only the one for this machine was unpacked. It could then be used from a plain background run: `import PIL` printed that path and `12.3.0`.

**The manifest can be read straight off disk.** The installed add-on's `blender_manifest.toml` sat at `<settings folder>/extensions/user_default/smoozy_test/blender_manifest.toml` and its content was the same text that was packed, unchanged. A generated repository listing carries the same fields plus the package's size and a hash.

**Outside the interface.** Settled: the online-access properties, the validate command and the manifest on disk are all reachable with Blender driven from outside its window.

**Operating system.** Settled for macOS only, on the one machine described above. Which bundled module is needed does differ by kind of computer; the macOS-on-Apple-Silicon case was tested here and the Linux case afterwards on a rented Linux machine, in its own block under this question. Windows stays untested.

### Settled by hand test on the rented Linux machine — what can be read about what an add-on needs, and what happens when what it needs is built for another machine

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 4.5.12 LTS from `/opt/blender/blender-4.5.12-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

**Only one spelling of the online-access override property exists.**

```
ACCESS False
HAS_OVERRIDE True
HAS_OVERRIDEN False
PREF False
```

— the spelling the API documentation uses, not the one the manual uses. Under `--online-mode` the same reads printed `ACCESS True`, `OVERRIDE True`; under `--offline-mode`, `ACCESS False`, `OVERRIDE True`. The preference itself was never touched by either. Exit status 0 throughout, and identical to the macOS run.

**Validating a package prints exactly one line and no manifest field at all.** On the source folder and on the built zip alike:

```
Success parsing TOML in "<source folder>"
```

```
Success parsing TOML in "<the zip>"
```

Exit status 0 for both. So that command cannot be used to read an add-on's declared needs. The same as the macOS run.

**The manifest on disk is the same text that was packed**, unchanged, and a generated repository listing carries the same fields plus the package's size and a hash — see question 8.

**A bundled module is unpacked into a shared place, not into the add-on's own folder**, and can then be used from a plain run with no window. Here that place is `<settings folder>/extensions/.local/lib/python3.11/site-packages/`, and the compiled parts unpacked there were the Linux ones — `_imaging.cpython-311-x86_64-linux-gnu.so` and seven more. `import PIL` printed that path and `12.3.0`; `from PIL import Image` built a 2 by 2 image. Full record under question 4.

**The new part: what happens when the bundled module for the running machine is absent.** A package was built whose bundled-module list names only the macOS file, and another naming only the Windows file. Both installed on this Linux machine with **no warning of any kind**, exit status 0, and both were switched on. Nothing was unpacked — there was no `.local` folder at all — and the wheel sat unopened in the add-on's own folder. Using it printed:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
ModuleNotFoundError: No module named 'PIL'
```

**Exit status 0.** So an add-on may declare a bundled module, be installed, be switched on, and simply not have it, with nothing said until something tries to use it.

**Cycles' own settings list** is the same nine properties on both machines: `rna_type`, `bl_idname`, `compute_device_type`, `devices`, `peer_memory`, `metalrt`, `use_hiprt`, `use_oneapirt`, `kernel_optimization_level`. Their current values on this machine read:

```
PROP compute_device_type ENUM NONE
PROP peer_memory BOOLEAN False
PROP metalrt ENUM AUTO
PROP kernel_optimization_level ENUM FULL
PROP use_hiprt BOOLEAN False
PROP use_oneapirt BOOLEAN True
```

Note that `metalrt`, which names Apple's own graphics, is present on this Linux machine too. The macOS run recorded the property list but not the values of `use_hiprt` and `use_oneapirt`, so those two are established here only.

**Outside the interface.** Settled again on a second operating system: everything above was read, and every package built and installed, with Blender driven from outside its own window.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed on the property spellings, the validate output and where a bundled module lands; each unpacked its own machine's compiled files. What happens when the bundled module for the running machine is absent was run on Linux only. Windows stays untested, and the 4.5 release note about a Python library dependency failing from subprocesses, whose wording points at Windows, was not tried.

### Settled by hand test on the rented Linux machine — what an opened row shows about what an add-on needs

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 4.5.12 LTS, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`.

**No declared permission, no bundled module and no dependency of any kind appears in an add-on's opened row.** The two rows opened carry a website, a type, and for one of them a maintainer and a version. The only field naming anything outside Blender is the website address. Neither add-on opened declares a permission in the first place, so this settles what those two rows show and nothing more — the same limit the Creator's macOS run had.

**The one add-on setting on screen that names something outside Blender is Cycles' own device row**, `None | CUDA | OptiX | HIP | oneAPI`, which names kinds of graphics hardware rather than anything the add-on requires. The System preferences also carry a **Network** section with an **Allow Online Access** tick box, unticked in a fresh settings folder, which is the preference the online-access property reads.

**Outside the interface.** This is a window test. The matching no-window results are under question 11 above.

**Operating system.** Settled for Linux in this version, at the window. The Creator's macOS run covered Blender 5.2 only. Windows stays untested.


> Can an add-on depend on something else being on the computer, or on a computer elsewhere, before it will work, and can what it needs be read anywhere?

### Something else on the computer

**LABEL: documented.** Yes, and the manual states a rule about it.

> "Bundle Dependencies
>
> For add-ons to be bundled as extensions, they must be self-contained.
> That means they must come with all its dependencies. In particular 3rd party Python modules.
>
> There are a few options for this:
>
> Bundle with Python Wheels.
>
> This is the recommended way to bundle dependencies.
>
> Bundle other add-ons together.
>
> This is recommended if an add-on depends on another add-on."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html — read 2026-07-30

So the intended answer for extensions is: an add-on should *not* depend on something else already being there; it should carry it. The manifest field for carrying it:

> "wheels:
>
> List of relative file-paths Python Wheels."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html — read 2026-07-30

> "Now installing the package will extract the wheel into the extensions own site-packages directory."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/python_wheels.html — read 2026-07-30

The rule is a should, not a can't. The manual documents a way to reach outside it:

> "--python-use-system-env
>
> Allow Python to use system environment variables such as PYTHONPATH and the user site-packages directory."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30

> "Blender bundles a complete Python installation and does not interact with the
> system Python by default. This can be changed with the --python-use-system-env
> command line argument, if care is
> taken to set up a compatible Python version."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30

An add-on can also need a *writable place* on the machine, and there is a documented function for it:

> "bpy.utils.extension_path_user(package, *, path='', create=False)
>
> Return a user writable directory associated with an extension."

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.utils.html — read 2026-07-30

with the manual adding what it does not survive:

> "This directory will be kept between upgrades but will be removed if the extension is uninstalled."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html — read 2026-07-30

**Observed**, a dependency of exactly this kind in practice: Cycles imports a compiled module `_cycles` that is not part of its own Python files — `import _cycles` appears at the top of `cycles_4.5/osl.py` (line 8) and inside functions in `cycles_4.5/engine.py` (e.g. line 35, `import _cycles`) and `cycles_4.5/properties.py` (line 1568). It also depends on how Blender was built, and reports when that dependency is missing:

> "    def update_script_node(self, node):
>         if engine.with_osl():
>             from . import osl
>             osl.update_script_node(node, self.report)
>         else:
>             self.report({'ERROR'}, "OSL support disabled in this build")"

File: `cycles_4.5/__init__.py`, lines 107–112 — read 2026-07-30. (*Citation check, 2026-08-05: this said lines 108–113; the passage runs 107 to 112.*)

### A computer elsewhere

**LABEL: documented.** Yes, and there is a declared field for it plus a switch that governs it.

> "# # Optional: add-ons can list which resources they will require:
> # # * files (for access of any filesystem operations)
> # # * network (for internet access)
> # # * clipboard (to read and/or write the system clipboard)
> # # * camera (to capture photos and videos)
> # # * microphone (to capture audio)"

with the worked example

> "# [permissions]
> # network = "Need to sync motion-capture data to server""

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html — read 2026-07-30

> "Internet Access
>
> If the add-on needs to use internet, it must check for the (read-only) property bpy.app.online_access.
> This option is controlled by Preferences, which can be overriding via command-line
> (--offline-mode / --online-mode)."

and the caveat, which the manual repeats in three places:

> "Add-ons that follow this setting will only connect to the internet if enabled.
> However, Blender cannot prevent third-party add-ons from violating this rule."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html — read 2026-07-30 (the same caveat appears at https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html)

> "bpy.app.online_access
>
> Boolean, true when internet access is allowed by Blender & 3rd party scripts (read-only)"

> "bpy.app.online_access_override
>
> Boolean, true when internet access preference is overridden by the command line (read-only)"

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.app.html — read 2026-07-30

(The manual spells this second name `bpy.app.online_access_overriden`; the API documentation spells it `online_access_override`. I record both spellings as I found them and take no view.)

The preference itself:

> "use_online_access
>
> Allow Blender to access the internet. Add-ons that follow this setting will only connect to the internet if enabled. However, Blender cannot prevent third-party add-ons from violating this rule.
>
> Type:
>
> boolean, default False"

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.PreferencesSystem.html — read 2026-07-30

Note the default is False.

A second, different kind of "computer elsewhere" is where the add-on itself comes from — a remote repository, which may need credentials:

> "access_token
>
> Personal access token, may be required by some repositories"
> "remote_url
>
> Remote URL to the extension repository, the file-system may be referenced using the file URI scheme: "file://""

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.UserExtensionRepo.html — read 2026-07-30

And the documented failure mode when that elsewhere-computer is unreachable:

> "403 Error Accessing Extensions
>
> On systems or networks that use an additional layer of SSL management:
> it's possible Blender's connection to https://extensions.blender.org is blocked.
>
> In this case the problem can't be resolved in Blender's side,
> it's up the system administrators to allow a connection to extensions.blender.org."

Source: Manual — https://docs.blender.org/manual/en/4.5/troubleshooting/network.html — read 2026-07-30

### Can what it needs be read anywhere?

**LABEL: documented** that the declarations exist in a file at a known path. **Not in the sources** that Blender offers any way to read them out.

*What can be read, and where it sits.* The manifest is a plain file inside the package, and its installed location is documented:

> "$BLENDER_SYSTEM_EXTENSIONS/system/my-addon/blender_manifest.toml"

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30

It holds `wheels` and `[permissions]` as quoted above. So an add-on's declared dependencies are readable by anything that can read a file at that path. A package can also be checked without installing it:

> "You may also validate a package without having to extract it first.
>
> blender --command extension validate add-on-package.zip"

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html — read 2026-07-30

And there is a listing command:

> "list:
>
> List all packages."
> "blender --command extension list [-h] [-s]
>
> List packages from all enabled repositories."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html — read 2026-07-30

A repository publishes a machine-readable index:

> "This creates an index.json listing from all the .zip extensions found in the –repo-dir location.
>
> For more details, read the generated JSON
> API."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/creating_repository/static_repository.html — read 2026-07-30

*What cannot be read, as far as the sources go.* The Python API documentation contains **no** mention of the word "wheel" in the extension sense and exactly **one** of "permission" — and that one is about filesystem write permissions, not the manifest field:

> "because it is cleared each upgrade and the users may not have write permissions
> to the repository (typically "System" repositories)."

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.utils.html — read 2026-07-30

There is no `manifest` anywhere in `api_4.5`. The type Blender exposes for an installed add-on carries only a module name and a preferences pointer (quoted under question 2b). So: no documented Python route to an add-on's declared dependencies.

**Not covered.** The output format of `blender --command extension list` is not shown anywhere in the sources, so I cannot say whether it reports dependencies, platforms, or only names. The "generated JSON API" the static-repository page points to is a link out of the pages I was given; its contents are not in my sources. Nothing states whether an unmet dependency is detected at install time, at enable time, or only when the add-on runs. Nothing describes what a *legacy* add-on may declare about dependencies — `bl_info` as documented has no field for it.

**Outside the interface.** Better outside than in, on this one. The `--offline-mode` and `--online-mode` switches, `--python-use-system-env`, `--command extension validate` and `--command extension list` are all command-line things and cannot be reached by clicking. The manual presents `--offline-mode` explicitly as the way to enforce it:

> "Working offline can be enforced by running with the --offline-mode
> command line argument. Users
> will then be unable to enable online access in the preferences."

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30
Inside the interface the equivalent is the Online Access preference. I found no capability here that exists inside but not outside.

**Operating system.** The wheel example shows dependencies being gathered per operating system:

> "pip download pillow --dest ./wheels --only-binary=:all: --python-version=3.11 --platform=macosx_11_0_arm64
> pip download pillow --dest ./wheels --only-binary=:all: --python-version=3.11 --platform=manylinux_2_28_x86_64
> pip download pillow --dest ./wheels --only-binary=:all: --python-version=3.11 --platform=win_amd64"

Source: Manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/python_wheels.html — read 2026-07-30
So *which* dependency is needed varies by operating system, while the mechanism does not. The one documented mechanism difference I found is the 4.5 bug-fix note that a Python DLL dependency in extensions "cant be loaded from subprocesses" — Source: https://developer.blender.org/docs/release_notes/4.5/bugfixes/ — read 2026-07-30 — which by its wording is Windows-specific, but the note is a one-line bug-fix entry and I do not read more into it. Otherwise the sources say nothing about operating-system differences here.

### Where I looked, for question 11

Searched the whole of `manual_4.5`, `api_4.5` and `release_notes_4.5` for, case-insensitive: `dependenc`, `wheel`, `whl`, `site-packages`, `site_packages`, `pip`, `manifest`, `blender_manifest`, `permission`, `network`, `online_access`, `use_online_access`, `offline`, `access_token`, `remote_url`, `PYTHONPATH`, `python-use-system-env`, `extension_path_user`, `require`, `requires`, `bundle`, `vendorize`, `self-contained`. Searched `api_4.5` specifically for `wheel` and `permission` and `manifest` to establish the absence recorded above — `wheel` returns only mouse-wheel and colour-wheel hits, `permission` returns one filesystem-permissions hit, `manifest` returns nothing.

Pages opened in full: manual — `advanced/extensions/addons.txt`, `advanced/extensions/getting_started.txt`, `advanced/extensions/python_wheels.txt`, `advanced/extensions/index.txt`, `advanced/extensions/creating_repository/static_repository.txt`, `advanced/extensions/creating_repository/dynamic_repository.txt`, `advanced/deploying_blender.txt`, `advanced/command_line/extension_arguments.txt`, `troubleshooting/network.txt`, `editors/preferences/extensions.txt`. API — `bpy.utils.txt` (the resource and extension-path functions), `bpy.app.txt` (online access), `bpy.types.PreferencesSystem.txt` (searched for online), `bpy.types.PreferencesExtensions.txt`, `bpy.types.UserExtensionRepo.txt`, `bpy.types.Preferences.txt`, `bpy.types.Addon.txt`. Cycles — `osl.py` (start), `engine.py`, `properties.py`, `__init__.py`. Release notes — `python_api.md` (whole), `bugfixes.md` (searched).

Ruled out: `editors/asset_browser.txt` and `files/asset_libraries/` use "dependencies" for what an asset refers to, which is a different sense of the word. `animation/drivers/drivers_panel.txt` uses it for the dependency graph. `contribute/manual/…/install/windows.txt` uses it for building the manual itself.

---

## What is still open

Every test on both lists has now been run, and the results are written into the answers above as observed findings. The Mac list, with each test's set-up and steps, is at `Audits/Blender Research/Add-ons/Hand tests.md`; the Linux list is at `Audits/Blender Research/Add-ons/Hand tests — Linux.md`.

**Two machines, never merged.** Everything was run on the Creator's MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics), and then again on a rented Linux machine (Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH). Where a test was run on both, each machine's result is written up in its own block and the finding says whether they agreed.

**The five window tests are now answered for this version.** The Creator ran them on macOS in Blender 5.2 only, so they were not answers here; they were run again at Blender's own window on the Linux machine, in all four versions, on 2026-08-06, and those results are in the answers above. They settle this version on Linux. What the same panels show on macOS in this version is still untested, because the Creator's run covered 5.2 alone.

**Three things the Linux machine settled that the Mac could not**: what Blender does with an add-on carrying a compiled part built for another kind of computer; what happens when an installed, set-up add-on is carried across to a machine of a different kind; and what Cycles' own device setting reads and offers on a machine whose graphics are not Apple's. All three are in the answers above.

**Driving a window over SSH: what works and what does not.** The machine has `xdotool`. Blender receives button and key events sent to its own window — its own modal handler printed them back with the coordinates that were sent — but it receives **no pointer-motion events at all**, and **a click could not be shown to work any control**: three separate clicks aimed at a Properties tab, a preferences section and a tick box each left the matching property unchanged. So nothing in this document was got by clicking. The workspace "Filter Add-ons" panel, the Get Extensions repositories drop-down and the render-engine list were each opened instead by one of Blender's own operators, with the identifier read out of `bpy.types` first, and each finding quotes the operator and what it returned. One thing is still unread: in Blender 4.5, 5.0 and 5.1 a tooltip covered the repositories menu in every picture taken, so what those three versions list there was not seen.

**Cannot be settled with the equipment the project has** — one MacBook Pro with Apple's own graphics; one rented Linux machine with one NVIDIA card, **which was destroyed on 2026-08-06 and can never be run again**; and a second rented Linux machine with two NVIDIA cards and no display server at all. *(Two-card citation check, 2026-08-06: this named two machines and called the rented one a container. There are three, one of them gone, and the list below was written when there were two.)*

- Anything on Windows. That leaves untested the "is not a valid Win32 application" failure the manual describes, the `file://HOST/share/...` repository address the sources give for Windows only, and whether the manual's line about headless Windows systems touches add-ons. (Questions 3, 4, 5, 8, 11.)
- A computer with **no graphics hardware at all**. The screenless case itself is now answered: a third machine, with two NVIDIA cards and **no display server at all**, ran the whole add-on cycle on 2026-08-06, and that result is written up under question 3. What is left is a machine with no graphics device, which no machine here is. (Question 3.)
- A graphics card of a make that is neither Apple's nor NVIDIA's. **Two graphics cards in one machine can now be reached** — the third machine has two — but nothing in this topic's questions turns on them, so it is left alone rather than half-answered; what two cards do to Cycles' own device list belongs to *The machine*. (Questions 3, 4.) *(Two-card citation check, 2026-08-06: the second half of this line was no longer true.)*
- How add-ons are sold, and anything about licence keys, activation or entitlement. No test on a computer can answer it. (Question 8.)
- A third-party extensions site. No address for one appears in the sources, so none can be tried without guessing at one. A repository made from a folder was tested instead, on both machines. (Questions 5, 8.)

---

2026-08-04 — Opus 5 (Claude Code) — Add-ons, Blender 4.5
