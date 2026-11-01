# Add-ons — Blender 5.2 — second pass (questions 2, 4, 10, 11)

**Sources consulted, and when.** Blender 5.2 manual (`Blender Sources/manual_5.2`, including two pictures from its `_images` folder), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files (`Blender Sources/cycles_5.2`). All read on **2026-07-30** (the snapshot date given in my inputs).

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

Two disclosures, so nothing is hidden:

- I also read `Blender Sources/About this folder.md` and `Blender Sources/requirements/requirements_2026-08-01.txt`. Both are inside `Blender Sources/`, so the line above still holds. Neither is in the list of sources my inputs named, so **no answer below rests on either of them**. (The requirements page does say "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later", which names my version in so many words, but I have not used it to carry any claim.)
- The `About this folder.md` note contains a link to a project document about research method. I did not follow it.

I worked alone. No helper agents were started, so there is nobody else's work to vouch for.

A note on how I searched: I searched the whole manual and the whole API documentation at once with text search before opening anything, for every question. The search terms and the pages ruled out are recorded under each answer, under **Where I looked**.

---

## 2. What makes a `.blend` file need an add-on at the moment it renders, and can an add-on that takes part in producing the render be told apart from one that only rearranges Blender's existing tools while the artist works?

### Settled by hand test — which engines are registered, and whether an add-on's part in a render can be seen

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.2.0. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**The list of engines a Blender will accept is asked for from the command line, and it shrinks when an add-on goes.** With everything as installed:

`blender -E help`

printed:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

Exit status 0. Cycles was then switched off:

`blender --background --python-expr "import bpy; print('DISABLE', bpy.ops.preferences.addon_disable(module='cycles')); bpy.ops.wm.save_userpref()"`

which printed `DISABLE {'FINISHED'}` and `Info: Preferences saved`. Running `blender -E help` again printed:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
```

A test add-on written for this work, whose whole content is a `RenderEngine` class with `bl_idname = "SMOOZY_TEST"`, was built and installed with `blender --command extension install-file -r user_default -e smoozy_engine-1.0.0.zip`. After that, `blender -E help` printed:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
	SMOOZY_TEST
```

**The engine setting's own list of values is not where add-on engines appear.** With Cycles registered:

`blender --background --python-expr "import bpy; r=bpy.context.scene.render; print('ENGINE', r.engine); print('ITEMS', [i.identifier for i in bpy.types.RenderSettings.bl_rna.properties['engine'].enum_items]); print('MULTI', getattr(r,'has_multiple_engines','NO SUCH PROPERTY')); print('SUBCLASSES', [(c.__name__, getattr(c,'bl_idname',None)) for c in bpy.types.RenderEngine.__subclasses__()])"`

printed:

```
ENGINE BLENDER_EEVEE
ITEMS ['BLENDER_EEVEE']
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

### Settled at the window — what an add-on's row shows, and what the engine list offers when the add-on is off (Blender 5.2 only)

**Label: observed.** Run by the Creator on 2026-08-05, at Blender's own window, on their own MacBook Pro (Apple M1 Pro chip, macOS 26.5.2), in Blender 5.2. Blender was started from a terminal pointed at a throwaway settings folder of its own, so their own Blender settings were not touched. **This was run in Blender 5.2 only**, so it says nothing about 4.5, 5.0 or 5.1. The Creator works on a Mac, so this settles macOS and leaves Windows and Linux open.

**The Add-ons preferences, two rows opened.** The Creator opened the "Cycles Render Engine" row and the "Node Wrangler" row in Edit ▸ Preferences ▸ Add-ons and returned a screenshot. The Cycles row shows:

```
Cycles Render Engine                                    [ticked]
Cycles renderer integration.                            Built-in
Website        docs.blender.org
Type           Core
Preferences
  [ None | Metal ]                (Metal is the one selected)
  [x] Apple M1 Pro (GPU - 14 cores)
  [ ] Apple M1 Pro
  Kernel Optimization    Full
  MetalRT                Auto
```

and the Node Wrangler row shows:

```
Node Wrangler                                           [unticked]
Various tools to enhance and speed up node-based workflow.    Built-in
Website        docs.blender.org
Type           Core
Maintainer     Blender Foundation
Version        4.1.0
```

**Nothing in either row says whether the add-on takes part in producing a render.** The two opened rows do not carry the same fields: Cycles shows no Maintainer and no Version, while Node Wrangler shows both. Neither shows a tag, a category, a licence, a repository or a declared permission of any kind. Both are marked "Built-in" at the right, and both read `Type  Core`. Also visible in the same screenshot, as closed rows: "glTF 2.0 format" (ticked), "Hydra Storm render engine" (unticked), "Manage UI translations" (unticked).

**What the window offers when the engine's add-on is off.** A file saved with Cycles chosen was opened in a settings folder where Cycles had been switched off. The Creator's own words:

> "Immediately upon opening it gave me a warning at the bottom that said "Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)". Only EEVEE and workbench are present as a choice."

**Not covered by this result.** Both add-ons opened are ones that ship with Blender. What an extension installed from a repository shows in its own row was not reported, so whether a licence, a repository name or a declared permission appears for *that* kind of add-on is still unsettled. And what the Render Engine field itself read was not reported — only what its list offered.

**Outside the interface.** Settled both ways for the engine. Driven from outside its window, Blender prints `Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)` to the terminal and carries on (2026-08-04, recorded above). At the window, the same words appear as a warning at the bottom of the screen, and the engine is gone from the list of choices. For the second half of the question — telling a render add-on from an interface one — the window adds nothing: the same fields are shown for both.

**Operating system.** macOS only, and Blender 5.2 only.

### Settled by hand test on the rented Linux machine — which engines are registered, and whether an add-on's part in a render can be seen

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.2.0 LTS from `/opt/blender/blender-5.2.0-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

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
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

`-E help` printed the list and exited 0. Cycles was then switched off with `blender --background --python-expr "import bpy; print('DISABLE', bpy.ops.preferences.addon_disable(module='cycles')); bpy.ops.wm.save_userpref()"`, and the same command printed:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
```

The engine names are the same as the macOS run recorded for this version.

**The engine setting's own list, and the registered engine classes.**

`blender --background --python-expr "import bpy; r=bpy.context.scene.render; print('ENGINE', r.engine); print('ITEMS', [i.identifier for i in bpy.types.RenderSettings.bl_rna.properties['engine'].enum_items]); print('MULTI', r.has_multiple_engines); print('SUBCLASSES', [(c.__name__, getattr(c,'bl_idname',None)) for c in bpy.types.RenderEngine.__subclasses__()])"`

printed:

```
ENGINE BLENDER_EEVEE
ITEMS ['BLENDER_EEVEE']
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

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 5.2.0 LTS, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`. Fresh settings folders, preferences saved once beforehand so the Quick Setup panel is not in the picture.

**An add-on's opened row.** A script set `show_expanded` in the metadata blocks of `cycles` and `node_wrangler`, opened the Add-ons preferences and photographed them. The picture shows, for **Cycles Render Engine**: the line "Cycles renderer integration.", the word "Built-in" at the right of it, a **Website** field reading `docs.blender.org`, a **Type** field reading `Core`, and then a section headed **Preferences** holding one row of buttons — `None`, `CUDA`, `OptiX`, `HIP`, `oneAPI` — with `None` selected. For **Node Wrangler**: "Various tools to enhance and speed up node-based workflow.", "Built-in", the same **Website** field, **Type** `Core`, a **Maintainer** field naming five people, and a **Version** field reading `3.55`.

**Nothing in either row says whether the add-on takes part in a render.** Neither row carries a tag, a category, a licence, a repository name or a declared permission. Cycles shows no maintainer and no version; Node Wrangler shows both. Both read `Type  Core`. That is the same as the Creator's macOS run in Blender 5.2, except for the row of compute-device buttons, which names this machine's own kinds of graphics hardware rather than the Mac's — see question 4.

**The Render Engine field when the engine's add-on is switched off.** A file saved with Cycles was opened with a window, in a settings folder where Cycles had been switched off. The script printed:

```
ENGINE BLENDER_EEVEE
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


### Short answer

Two things in the sources tie a saved file to an add-on at render time. First, **the render engine**: a scene records which engine it renders with, and an engine can be supplied by an add-on. Second, **settings the add-on hangs on the scene and its objects**: an add-on can attach its own blocks of settings to scenes, objects, materials, lights, worlds, meshes and view layers, and Cycles — an add-on itself — does exactly this.

On the second half: **no**. The sources give no way to tell a render-participating add-on from a working-comfort one. There is a list of self-declared tags an author may attach (one of which is "Render" and another "User Interface"), and there is a documented list of the things Python add-ons are allowed to plug into (only one of which, the render engine, is about rendering) — but nothing in the sources says either can be relied on, and the same documentation shows that any add-on at all can attach itself to the render through handlers.

### Part one — what ties a file to an add-on at render time

**(a) The render engine.**

> "More renderers from third-party developers are available as
> add-ons.
> Each renderer has its own render settings to control render quality and performance."
>
> "The active render engine can be selected in the Render Properties."

Source: manual, https://docs.blender.org/manual/en/5.2/render/render_engines.html — read 2026-07-30. (The same sentence about third-party renderers appears at https://docs.blender.org/manual/en/5.2/render/introduction.html.)

> "Render the whole animation using all the settings saved in the blend-file."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — read 2026-07-30.

> "-E, --engine <engine>
>
> Specify the render engine.
> Use -E help to list available engines."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

That an engine is a thing an add-on registers, under a name, is visible in Cycles' own files:

> `class CyclesRender(bpy.types.RenderEngine):`
> `    bl_idname = 'CYCLES'`
> `    bl_label = "Cycles"`

Source: `Blender Sources/cycles_5.2/__init__.py`, lines 44–46 — read 2026-07-30. The same file registers that class only inside its `register()` function (`for cls in classes: register_class(cls)`, lines 158–159, where the `classes` tuple holding `CyclesRender` begins at line 130), and removes it again in `unregister()` (lines 184–185).

The API documentation says the same in its own words, in its worked example of a render engine:

> "These three members are used by Blender to set up the
> RenderEngine; define its internal name, visible name and capabilities."

Source: Python API, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html — read 2026-07-30.

And a picture in the manual shows a render engine sitting in the add-on list with a tick-box beside it, alongside a second one that is switched off:

Rows visible in the Add-ons preferences screenshot include "Cycles Render Engine" (ticked) and "Hydra Storm render engine" (unticked), among "Node Wrangler", "Rigify", "Pose Library" and several file formats.

Source: manual picture `_images/editors_preferences_section_addons.webp`, shown on https://docs.blender.org/manual/en/5.2/editors/preferences/addons.html — read 2026-07-30. **documented** (it is part of the manual). What the picture shows is that a render engine can be an add-on you tick or untick; it does not show what happens to a render if you untick it.

**Label for (a): inferred.** The step a reader has to take: the sources say the engine choice is a setting, that settings are saved in the blend-file, that an engine can come from an add-on, and that an engine's name is registered by the add-on when it is switched on. Putting those together gives "a file whose engine is an add-on's engine needs that add-on present and switched on at render time". No source states that sentence.

**(b) Settings the add-on attaches to the scene and its data.**

Cycles' own files show an add-on hanging its settings on Blender's data:

> `    def register(cls):`
> `        bpy.types.Scene.cycles = PointerProperty(`

Source: `Blender Sources/cycles_5.2/properties.py`, lines 1141–1142 — read 2026-07-30. The same file does this for `bpy.types.Camera.cycles_custom` (line 1158), `bpy.types.Material.cycles` (1209), `bpy.types.Light.cycles` (1250), `bpy.types.World.cycles` (1316), `bpy.types.World.cycles_visibility` (1369), `bpy.types.Mesh.cycles` / `Curve.cycles` / `MetaBall.cycles` (1385–1395), `bpy.types.Object.cycles` (1483), `bpy.types.Scene.cycles_curves` (1512) and `bpy.types.ViewLayer.cycles` (1603). Each is removed again in the matching `unregister` (for example `del bpy.types.Scene.cycles`, line 1150).

The API documentation describes the same technique, and describes it as storage:

> "For example, if you want to store material settings for a custom engine:"

Source: Python API, https://docs.blender.org/api/5.2/info_overview.html — read 2026-07-30.

That these settings arrive from saved files, and are old enough to need converting when an old file is opened, is visible in Cycles too. Cycles registers a file-version handler:

> `    bpy.app.handlers.version_update.append(version_update.do_versions)`

Source: `Blender Sources/cycles_5.2/__init__.py`, line 161 — read 2026-07-30. And the handler's own code reasons about the file it is fixing:

> `    # File doesn't need versioning`
> `    if bake_type_idx < diffuse_direct_idx:`
> `        return`
>
> `    # File needs versioning`

Source: `Blender Sources/cycles_5.2/version_update.py`, lines 45–49 — read 2026-07-30.

The 5.2 release notes confirm that settings defined this way are the same kind of thing as a user's own custom properties:

> "The maximum level of nesting for IDProperties is now set to 1024 levels (previously there was no limits, leading to recursion crashes in the code when reaching OF stack memory limits). This affects both user properties (groups inside of groups inside of groups etc.), as well as py-defined data (RNA structs inside of RNA structs inside of RNA structs etc.)."

Source: release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/ — read 2026-07-30. (*Citation check, 2026-08-05: this was cited to the `core` release notes, where the passage does not appear. It is in the Python API release notes.*)

**Label for (b): observed** for the fact that Cycles attaches its settings to Scene, Object, Material, Light, World, Mesh, Curve, MetaBall, Camera and ViewLayer, and removes them when switched off; and for the fact that its file-version handler reasons about values read from files. **Inferred** for the step from there to "a file's Cycles settings cannot be read back without the Cycles add-on": the sources do not say what happens to those values when the add-on is absent.

**(c) Adjacent, and deliberately kept separate.** The manual documents ways a *file* runs its own scripts at render time — but these are scripts carried inside the file, not add-ons, and I am not claiming otherwise:

> "Registered Text-Blocks
>
> A text data-block can have its Register option enabled which means it will load on start.
>
> Animation Drivers
>
> Python expressions can be used to Drive values and are often used in more advanced rigs and animations."
>
> "Rendering with Freestyle, because Freestyle uses scripts to control line styles."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/scripting/security.html — read 2026-07-30. **documented**, but about scripts inside the file, not about add-ons.

### Part two — telling the two kinds of add-on apart

What the sources do offer:

> "Add-ons are assigned categories by what areas of Blender they affect."

Source: manual, https://docs.blender.org/manual/en/5.2/editors/preferences/addons.html — read 2026-07-30.

The tags themselves are a fixed list, self-declared in the add-on's manifest:

> "# # Optional: tag list defined by Blender and server, see:
> # # https://docs.blender.org/manual/en/latest/advanced/extensions/tags.html
> # tags = ["Animation", "Sequencer"]"

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/getting_started.html — read 2026-07-30. The full list for add-ons runs "3D View, Add Curve, Add Mesh, Animation, Bake, Camera, Compositing, Development, Game Engine, Geometry Nodes, Grease Pencil, Import-Export, Lighting, Material, Modeling, Mesh, Node, Object, Paint, Pipeline, Physics, Render, Rigging, Scene, Sculpt, Sequencer, System, Text Editor, Tracking, User Interface, UV" — source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/tags.html — read 2026-07-30.

A second, sharper signal is the list of things a Python add-on is allowed to plug into at all:

> "The Blender Python API allows integration for:
>
> bpy.types.Panel
>
> bpy.types.Menu
>
> bpy.types.Operator
>
> bpy.types.PropertyGroup
>
> bpy.types.KeyingSet
>
> bpy.types.RenderEngine
>
> This is intentionally limited. Currently, for more advanced features such as mesh modifiers,
> object types, or shader nodes, C/C++ must be used."

Source: Python API, https://docs.blender.org/api/5.2/info_overview.html — read 2026-07-30.

And Blender does have a documented notion of "this add-on's contribution is interface":

> "Filter Add-ons
>
> Determines which add-ons are enabled in the active workspace.
> When unchecked, the global add-ons will be used.
> When checked, you can enable individual add-ons in the list below."

Source: manual, https://docs.blender.org/manual/en/5.2/interface/window_system/workspaces.html — read 2026-07-30. The matching API property spells out that this is about the interface only:

> "use_filter_by_owner
>
> Filter the UI by tags (default False)"

Source: Python API, https://docs.blender.org/api/5.2/bpy.types.WorkSpace.html — read 2026-07-30.

Against all of that stands this: the API documents a set of hooks that any add-on may attach to, several of which fire during a render.

> "bpy.app.handlers.render_pre
>
> on render (before). Accepts one argument: the scene data-block being rendered"

Source: Python API, https://docs.blender.org/api/5.2/bpy.app.handlers.html — read 2026-07-30. The same page lists `render_init`, `render_post`, `render_complete`, `render_cancel`, `render_stats`, `render_write`, `composite_pre`, `composite_post`, `composite_cancel`, `frame_change_pre`, `frame_change_post`, `depsgraph_update_pre` and `depsgraph_update_post`. Of `frame_change_pre` it says:

> "Called when a frame change is triggered for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations (for example swap an object to another mesh) for the new frame."

Same page, same date.

**Label for part two: not in the sources.** Nothing in the manual, the API documentation or the 5.2 release notes states any way — a flag, a field, a query — to tell whether a given add-on takes part in producing a render. The tags are a documented, author-declared label with no stated meaning beyond describing "what areas of Blender they affect", and the handler list shows an add-on tagged anything at all can change the scene while it renders.

### Not covered

- What actually happens to a render when the engine's add-on is absent — that is question 7's ground, and I found nothing on it while searching for this one.
- Whether an add-on's saved settings survive a round trip through a Blender where the add-on is switched off.
- Whether the tags list is enforced or checked by anything other than the extensions website's own review.
- Whether anything reads the tags at render time. Nothing suggests it does.
- Node groups, geometry-nodes assets, and imported/linked data as possible ties to an add-on: I searched and found nothing tying these to add-ons (see below).

### Outside the interface

Everything above is stated without reference to a person clicking. The command-line page's `-E, --engine` and "all the settings saved in the blend-file" are specifically about running Blender from a command line. The one item that is interface-shaped is the workspace add-on filter, which the API calls "Filter the UI by tags" — by its own words it is about what the interface shows, so it has no bearing outside the interface. The sources do not say anything else about how any of this differs when Blender is driven from outside its own window.

### Operating system

The sources say nothing about any of this differing by operating system, and nothing here mentions one. **The sources do not say.**

### Where I looked

Searched the whole of `manual_5.2`, `api_5.2` and `release_notes_5.2` for: `requires an add-on`, `requires the add-on`, `needs an add-on`, `add-on is required`, `required add-on` — **no matches anywhere**. Searched for lines mentioning an add-on and a blend-file together (`add-on|addon` cross-checked against `blend-file|blend file|.blend`) — six matches, all incidental (a web address containing "add-ons", a link to the extensions site, a sentence about add-ons writing to other blend-files in the asset-libraries page, a translators' guide link). Searched for `add-on is not enabled`, `addon is not enabled`, `add-on is missing`, `missing add-on`, `without the add-on`, `add-on is disabled` — the only hits were the add-on tutorial explaining that `unregister` runs when an add-on is disabled.

Searched for `Undefined` — one hit, `bpy.types.NodeTree` ("Undefined type of nodes (can happen e.g. when a linked node tree goes missing)"), which is about linked node trees, not add-ons; ruled out. Searched `custom node` — hits in the compositor and shader-node pages about node groups and Open Shading Language, none about add-ons registering node types; ruled out, and the API's integration list above says shader nodes need C/C++ anyway. Searched `render engine` narrowed to lines also mentioning add-ons — only the glTF add-on page and the two render pages quoted above.

Pages opened in full: manual `render/introduction`, `render/render_engines`, `advanced/scripting/security`, `advanced/extensions/addons`, `advanced/extensions/getting_started`, `advanced/extensions/tags`, `editors/preferences/addons`, `files/custom_properties`, `files/blend/compatibility`; API `info_overview`, `bpy.types.RenderEngine`, `bpy.types.RenderSettings` (the `engine` entry), `bpy.types.WorkSpace`, `bpy.types.wmOwnerID`, `bpy.app.handlers`; release notes `index`, `core`, `python_api`, `rendering`. Cycles files opened: `__init__.py` in full, `properties.py` (registration lines), `version_update.py` (opening section).

Ruled out and why: `files/blend/compatibility.html` covers version-to-version file compatibility and never mentions add-ons. `files/custom_properties.html` covers the user's own custom properties and never mentions add-ons. The `bpy.types.RenderSettings` `engine` entry lists only `Literal['BLENDER_EEVEE']`, which tells me nothing usable about add-on engines and which I have therefore not quoted as evidence of anything.

---

## 4. What does an add-on record about the kinds of computer it is meant for, what happens on one it does not name, and does an add-on set up on one computer still work when copied to a computer of a different kind?

### Settled by hand test — what happens on a kind of computer an add-on does not name, and which bundled modules land on disk

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.2.0. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

**A package naming only another kind of computer is refused, and nothing is installed.** Two test add-ons were built, identical but for their manifests: one with `platforms = ["windows-x64"]`, one with `platforms = ["macos-x64"]`. Both built without complaint. Installing the first:

`blender --command extension install-file -r user_default -e smoozy_win_only-1.0.0.zip`

printed:

```
ERROR smoozy_win_only: This platform (macos-arm64) isn't one of (windows-x64)
Package was expected to be installed but not found: smoozy_win_only
```

and the second:

```
ERROR smoozy_macx64_only: This platform (macos-arm64) isn't one of (macos-x64)
Package was expected to be installed but not found: smoozy_macx64_only
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
  "./wheels/pillow-12.3.0-cp313-cp313-macosx_11_0_arm64.whl",
  "./wheels/pillow-12.3.0-cp313-cp313-manylinux_2_27_x86_64.manylinux_2_28_x86_64.whl",
  "./wheels/pillow-12.3.0-cp313-cp313-win_amd64.whl",
]
```

After installing it, the add-on's own folder held **all three** files, unopened, in a `wheels` folder. But the unpacked code sat somewhere else and was for this machine only — under `<settings folder>/extensions/.local/lib/python3.13/site-packages/`, whose compiled parts were named:

```
_imagingmath.cp313-darwin.so
_imaging.cp313-darwin.so
_webp.cp313-darwin.so
_imagingcms.cp313-darwin.so
_imagingft.cp313-darwin.so
_imagingmorph.cp313-darwin.so
_imagingtk.cp313-darwin.so
_avif.cp313-darwin.so
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

### Settled at the window — installing a package for another kind of computer (Blender 5.2 only)

**Label: observed.** Run by the Creator on 2026-08-05, at Blender's own window, on their own MacBook Pro (Apple M1 Pro chip, macOS 26.5.2), in Blender 5.2. Blender was started from a terminal pointed at a throwaway settings folder of its own, so their own Blender settings were not touched. **This was run in Blender 5.2 only**, so it says nothing about 4.5, 5.0 or 5.1. The Creator works on a Mac, so this settles macOS and leaves Windows and Linux open.

**What was done.** The Creator went to Edit ▸ Preferences ▸ Get Extensions, chose "Install from Disk…" from the drop-down at the top right, and picked `smoozy_win_only-1.0.0.zip` — a small test add-on built for this work whose manifest names `platforms = ["windows-x64"]` and nothing else.

**What came back**, in the Creator's own words:

> "I got a red warning in the console that said "smoozy_win_only: This platform (macos-arm64) isn't one of (windows-x64)" and in the Get Extensions panel nothing happened."

So the window route refuses the package with the same sentence the command-line route gives, the message goes to the console rather than into the panel, and the panel itself shows nothing.

**Not covered by this result.** Whether the add-on then appeared anywhere in the Add-ons list was not reported.

**Outside the interface.** Settled both ways. From outside the window the refusal prints `ERROR smoozy_win_only: This platform (macos-arm64) isn't one of (windows-x64)` and the command still exits 0 (2026-08-04, recorded above). At the window the same sentence appears as a red warning in the console, and the panel does nothing.

**Operating system.** macOS only, and Blender 5.2 only. What a Windows or Linux machine does with a package naming another kind is still untested, and was not tried with the Creator's own equipment.

### Settled by hand test on the rented Linux machine — what happens on a kind of computer an add-on does not name, which bundled modules land on disk, and what a compiled part for the wrong machine does

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.2.0 LTS from `/opt/blender/blender-5.2.0-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

**A package naming only another kind of computer is refused, and this machine can pose the case the Mac could not.** Four test add-ons were built, identical but for their manifests: `platforms = ["windows-x64"]`, `["macos-x64"]`, `["macos-arm64"]` and `["linux-x64"]`. All four built without complaint. Installing each with `blender --command extension install-file -r user_default -e <zip>` printed:

```
ERROR smoozy_win_only: This platform (linux-x64) isn't one of (windows-x64)
Package was expected to be installed but not found: smoozy_win_only
```

```
ERROR smoozy_macx64_only: This platform (linux-x64) isn't one of (macos-x64)
Package was expected to be installed but not found: smoozy_macx64_only
```

```
ERROR smoozy_macarm_only: This platform (linux-x64) isn't one of (macos-arm64)
Package was expected to be installed but not found: smoozy_macarm_only
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

**Only the matching bundled module is unpacked, and here that means the Linux one.** A test add-on was built carrying three bundled Python modules of the same library, one each for macOS on Apple Silicon, Linux and Windows, named in its manifest as `cp313` files. After installing it, the add-on's own folder held **all three** files, unopened, in a `wheels` folder:

```
pillow-12.3.0-cp313-cp313-macosx_11_0_arm64.whl
pillow-12.3.0-cp313-cp313-manylinux_2_27_x86_64.manylinux_2_28_x86_64.whl
pillow-12.3.0-cp313-cp313-win_amd64.whl
```

The unpacked code sat somewhere else and was for this machine only — under `<settings folder>/extensions/.local/lib/python3.13/site-packages/`, whose compiled parts were named:

```
_avif.cpython-313-x86_64-linux-gnu.so
_imaging.cpython-313-x86_64-linux-gnu.so
_imagingcms.cpython-313-x86_64-linux-gnu.so
_imagingft.cpython-313-x86_64-linux-gnu.so
_imagingmath.cpython-313-x86_64-linux-gnu.so
_imagingmorph.cpython-313-x86_64-linux-gnu.so
_imagingtk.cpython-313-x86_64-linux-gnu.so
_webp.cpython-313-x86_64-linux-gnu.so
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
<settings folder>/extensions/user_default/smoozy_macwheel/wheels/pillow-12.3.0-cp313-cp313-macosx_11_0_arm64.whl
```

And using it fails:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
ModuleNotFoundError: No module named 'PIL'
```

**Exit status 0.** The Windows-only package behaved the same way in every respect. So an add-on carrying a compiled part built for another kind of computer **installs, switches on, and is simply missing what it needs**, with nothing said at install time and nothing in the exit status.

**The new part: carrying an installed, set-up add-on to a computer of a different kind.** The whole settings folder holding the installed three-module add-on was packed on this Linux machine, copied to the Creator's MacBook Pro (M1 Pro chip, macOS 26.5.2), and that Mac's Blender 5.2.0 LTS was pointed at it. The Mac listed it as installed:

```
Repository: "User Default" (id=user_default)
  smoozy_wheels [installed]: "Smoozy Wheels", A test add-on carrying bundled Python modules
```

and had it switched on:

```
VERSION 5.2.0 LTS
MODULES ['bl_ext.user_default.smoozy_wheels', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

`import PIL` succeeded and printed the version, `12.3.0`. But using it failed:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
  File "<settings folder>/extensions/.local/lib/python3.13/site-packages/PIL/Image.py", line 95, in <module>
    from . import _imaging as core
ImportError: cannot import name '_imaging' from 'PIL' (<settings folder>/extensions/.local/lib/python3.13/site-packages/PIL/__init__.py)
```

**Exit status 0.** Looking on disk, the unpacked compiled files were still the Linux ones, `_imaging.cpython-313-x86_64-linux-gnu.so` and the rest; nothing had gone back to the add-on's own folder to unpack the macOS file that is sitting there. So the install, the switch-on and the plain Python travel; the compiled part does not, and nothing announces it until the add-on tries to use it.

This one test was run on both machines by design — packed on Linux, opened on macOS — and is written here as one crossing rather than as two results.

**Outside the interface.** Settled: the platform check, the message it gives, the folder filtering, which bundled module is unpacked, and everything about the wrong-machine compiled part all happen with Blender driven from outside its window, and the message goes to the terminal. The exit status never reports any of it — it stays 0.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed on the shape of the answer, each naming its own kind: `macos-arm64` there, `linux-x64` here. A package built for a Mac is refused on Linux, which the Mac could not show. Carrying an installed add-on from Linux to macOS was run once, across the two machines, and is recorded above. Windows stays untried: no Windows machine has run it.

### Settled by hand test on the rented Linux machine — installing a package for another kind of computer at the window, and what Cycles' own panel offers

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 5.2.0 LTS, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`. Fresh settings folders, preferences saved once beforehand.

**Installing a Windows-only package through the window.** A script opened the Preferences editor at its Get Extensions section, read the install operator's own list of arguments so that none was guessed —

```
OPERATOR PROPS ['directory', 'enable_on_install', 'filepath', 'files', 'filter_glob', 'overwrite', 'repo', 'rna_type', 'target', 'url']
```

— and then called it with the package's path and the user repository. It printed:

```
Package was expected to be installed but not found: smoozy_win_only
INSTALL RESULT {'FINISHED'}
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

So the operator reported **finished** while refusing the package. Nothing was installed and nothing was switched on. The picture, taken with the Add-ons section then showing, holds the thirteen add-ons that ship with this Blender and **no "Smoozy Windows Only" anywhere in the list**. That settles a gap the Creator's macOS run left open: the refused package does not appear in the Add-ons list afterwards.

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


### Short answer

An add-on packaged as an extension may list the kinds of computer it is for, in a field called `platforms`, choosing from five fixed names that pair an operating system with a chip family. Leaving the field out means "all of them". On a kind it does not name, two things are documented: the download service is told which kind of computer is asking and only lists what fits, and Blender's own package builder will not even produce a package for a kind that is not on the list. What happens if such a package is installed anyway is **not in the sources**.

On copying: if the add-on carries compiled parts, the manual says outright that these do not travel between kinds of computer.

### What is recorded

> "platforms:
>
> List of supported platforms. If omitted, the extension will be available in all operating systems.
> The available options are
> [“windows-x64”, “windows-arm64”, “macos-x64”, “macos-arm64”, “linux-x64”]"

And, in the sample manifest on the same page:

> "# # Optional: list of supported platforms. If omitted, the extension will be available in all operating systems.
> # platforms = ["windows-x64", "macos-arm64", "linux-x64"]
> # # Other supported platforms: "windows-arm64", "macos-x64""

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/getting_started.html — read 2026-07-30. **documented.**

The field is optional. The same page's list of required values is `blender_version_min`, `id`, `license`, `maintainer`, `name`, `schema_version`, `tagline`, `type`, `version` — `platforms` is in the optional list. So an add-on may record nothing at all about the kind of computer it is for. **documented.**

The same five names appear again as something a download link may carry:

> "platforms:
>
> Comma-separated list of supported platforms.
> If omitted, the extension will be available in all operating systems."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/creating_repository/static_repository.html — read 2026-07-30. **documented.**

For a legacy add-on — the older kind, not packaged as an extension — nothing equivalent exists. The manual's description of the old metadata block mentions name, version, author and a minimum Blender version, and nothing about kinds of computer:

> "bl_info
>
> is a dictionary containing add-on metadata such as the title,
> version and author to be displayed in the Preferences add-on list.
> It also specifies the minimum Blender version required to run the script;
> older versions won’t display the add-on in the list."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/scripting/addon_tutorial.html — read 2026-07-30. A real example of such a block, from Cycles' own files, carries `name`, `author`, `blender`, `description`, `warning`, `doc_url`, `tracker_url`, `support`, `category` and nothing about computers:

> `bl_info = {`
> `    "name": "Cycles Render Engine",`
> `    "author": "",`
> `    "blender": (2, 80, 0),`

Source: `Blender Sources/cycles_5.2/__init__.py`, lines 7–10 (the block runs to line 16) — read 2026-07-30. **observed.**

### What happens on a kind it does not name

Two documented effects, both before the add-on ever runs.

**The listing.** When Blender asks a repository what it has, it says what kind of computer it is:

> "When Blender fetches the extensions listing it passes the following arguments to make sure only
> compatible extensions are listed:
>
> platform
>
> blender_version"

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/creating_repository/dynamic_repository.html — read 2026-07-30. **documented.**

**The build.** Blender's package builder will not make a package for a kind the manifest does not name, even when the ingredients for it are present:

> "Note
>
> Even though there is a Linux-only wheel present, no Linux zip file is generated.
> This happens because the platforms field only has Mac and Windows."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/python_wheels.html — read 2026-07-30. **documented.**

What happens if the package reaches the wrong kind of computer anyway — by "Install from Disk", by a copied folder, by a system repository placed on a shared drive — is **not in the sources**. See **Where I looked**.

### Copying a working set-up to a different kind of computer

The manual answers this for compiled parts, plainly:

> "While not common practice, Python add-ons can be distributed with their own precompiled libraries. Unlike regular
> Python scripts, these are not portable between different platforms.
>
> It is possible the library is incompatible with your Blender installation (attempting to load a library built for a
> different version of Python, or loading a 32-bit library on a 64-bit system).
>
> If the add-on contains .pyd or .so files, check that the distribution is compatible with your operating
> system."

Source: manual, https://docs.blender.org/manual/en/5.2/troubleshooting/python.html — read 2026-07-30. **documented.**

Bundled third-party Python modules are the common case of exactly that. The manual tells authors to fetch a separate file per kind of computer:

> "For wheels that contain binary compiled files, wheels for all supported platforms should be included:"
>
> `pip download pillow --dest ./wheels --only-binary=:all: --python-version=3.13 --platform=macosx_11_0_arm64`
> `pip download pillow --dest ./wheels --only-binary=:all: --python-version=3.13 --platform=manylinux_2_28_x86_64`
> `pip download pillow --dest ./wheels --only-binary=:all: --python-version=3.13 --platform=win_amd64`

and says what installing does with them:

> "Now installing the package will extract the wheel into the extensions own site-packages directory."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/python_wheels.html — read 2026-07-30. **documented.**

**Inferred**, with the step written out: since installing extracts the matching wheel into the installed add-on's own folder, and since compiled libraries "are not portable between different platforms", copying an *installed* add-on that used a compiled wheel to a different kind of computer carries the wrong compiled library with it. The sources do not say this in those words; they say each half separately.

An add-on with no compiled parts is a different matter. The manual calls plain Python scripts portable by contrast ("Unlike regular Python scripts, these are not portable"), and the one platform-independent wheel example is fetched once with no platform named at all (`pip wheel jsmin -w ./wheels`, same page). **Inferred**, same step: the contrast implies pure-Python add-ons do travel; no source says so directly.

Two further things stand in the way of a straight folder copy, both documented and neither about the kind of computer as such:

> "The value of __package__ will vary between systems so it should never be replaced with a literal string."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/addons.html — read 2026-07-30. This sentence is ambiguous: "systems" could mean different computers or different installations (the surrounding text is about the repository name becoming part of the add-on's name, as in `bl_ext.{repository_module_name}.kitsu`). I read it as "different installations", because that is what the paragraph around it is about. On the other reading it would be a direct statement that a copied add-on's own name changes on the new machine.

And the place add-ons live differs by operating system: `$HOME/.config/blender/5.2/` on Linux, `/Users/$USER/Library/Application Support/Blender/5.2/` on macOS, `%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.2\` on Windows, with `./extensions` and `./scripts/addons` inside. Source: manual, https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html — read 2026-07-30. **documented.**

### Not covered

- What Blender does when an extension whose `platforms` list excludes this computer is installed anyway. Nothing found.
- Whether the `platforms` list is checked at install time, at start-up, at both, or only by the download service.
- Whether the five names are the complete set for 5.2 in the software, or only the set the extensions website accepts. The manual presents them as "the available options" without saying who enforces them.
- Anything about chip families beyond the five names — no source describes what `x64` or `arm64` mean here.
- Whether an add-on can itself find out which kind of computer it is on. (`bpy.app.build_platform`, "The platform this blender instance was built for", exists — Python API, https://docs.blender.org/api/5.2/bpy.app.html — but the sources do not connect it to the manifest's five names, and I am not claiming they match.)

### Outside the interface

The whole of this holds outside the interface, and the building side of it is only available there: `blender --command extension build --split-platforms` and `blender --command extension validate` are command-line tools, documented at https://docs.blender.org/manual/en/5.2/advanced/command_line/extension_arguments.html. The listing behaviour is a property of the download service and Blender's request to it, not of any window. The sources do not describe any difference between the two settings for any of this.

### Operating system

This answer *is* the operating-system answer, in part: the five names are three operating systems crossed with chip families, and the folder an add-on lives in differs per operating system. Beyond that, the sources do not say that the rules themselves differ by operating system — the same manifest field, the same builder behaviour, the same listing query on all of them. One asymmetry is visible in the sources: `linux-x64` has no Arm partner in the list of five, while Windows and macOS each have both. The sources do not explain that.

### Where I looked

Searched all of `manual_5.2` and `api_5.2` for `platform` — 40-odd hits, every one of them read; the ones that matter are quoted above, the rest are about the VFX reference platform, temporary-folder variables, and shader compilation testing. Searched for `arm64` and `x64` — hits only in the four extension pages quoted. Searched `manual_5.2/advanced`, `manual_5.2/editors/preferences` and `api_5.2/bpy.ops.extensions.txt` for `incompatible`, `not compatible`, `unsupported platform`, `not supported on` — **the only hit in any of them is a viewport page about Qualcomm GPUs**, nothing about extensions. Searched the API for `manifest` — one hit only (`info_overview`), so the API documentation does not describe the manifest's fields at all. Searched the API and manual for `wheel` — the only substantive hits are the two extension pages; everything else is mouse wheels and colour wheels.

Pages opened in full: manual `advanced/extensions/getting_started`, `advanced/extensions/python_wheels`, `advanced/extensions/creating_repository/static_repository`, `advanced/extensions/creating_repository/dynamic_repository`, `advanced/command_line/extension_arguments`, `advanced/blender_directory_layout`, `advanced/deploying_blender`, `troubleshooting/network`, `troubleshooting/python`, `editors/preferences/extensions`. API pages opened: `bpy.ops.extensions` (every operator listed and its parameters), `bpy.types.UserExtensionRepo`, `bpy.types.PreferencesExtensions`, `bpy.app` (attribute list in full). Release notes: all of `5.2` searched for `extension`, `add-on`, `addon` — the only hits are a VR manual link, a glTF import/export note, a compositor file-extension checkbox, a fill-tool note, and four bug-fix lines; none touch platforms.

Ruled out: `bpy.ops.extensions.package_install` and `package_install_files` take repository, package id, url and an overwrite flag and say nothing about platforms; I read every parameter and there is no platform among them.

---

## 10. Can a computer be asked which add-ons it holds, and for which Blender?

### Settled by hand test — what can be asked of a computer about its add-ons

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.2.0. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

**Three different lists exist, and none of them is everything.**

*What the repositories hold.* `blender --command extension list` prints one line per repository and, under it, one indented line per package, marked `[installed]` where it is on this machine:

```
Repository: "User Default" (id=user_default)
  smoozy_test [installed]: "Smoozy Test", A tiny test add-on
```

Each line carries the identifier, the name and the tagline — no version number and no repository name of its own. **It shows an installed package whether or not the add-on is switched on**: after `bpy.ops.preferences.addon_disable` and a preferences save, the same `[installed]` line was still printed. It never shows a built-in add-on, because those are not packages in a repository.

*What is switched on.* `blender --background --python-expr "import bpy; a=bpy.context.preferences.addons; print('COUNT', len(a)); print('MODULES', sorted(x.module for x in a))"` in a fresh settings folder printed:

```
COUNT 8
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

This list **drops an add-on the moment it is switched off** — the test add-on appeared in it after installing with `-e`, and was gone after `addon_disable`. So it is the switched-on set, not an inventory.

*What is on the machine at all.* `blender --background --python-expr "import addon_utils; print('PATHS', addon_utils.paths()); m=list(addon_utils.modules()); print('COUNT', len(m)); [print('MODULE', x.__name__, getattr(x,'bl_info',None)) for x in m]"` printed:

```
PATHS ['/Applications/Blender.app/Contents/Resources/5.2/scripts/addons_core']
COUNT 13
```

followed by one line per add-on with its whole metadata block. The 13 are `io_anim_bvh`, `bl_pkg`, `cycles`, `io_scene_fbx`, `io_scene_gltf2`, `hydra_storm`, `ui_translate`, `node_wrangler`, `pose_library`, `rigify`, `io_curve_svg`, `io_mesh_uv_layout` and `viewport_vr_preview` — every add-on that ships with Blender, switched on or not. With the test extension installed, `bl_ext.user_default.smoozy_test` appeared in that list too, so this route covers both kinds. `addon_utils` is the closest thing to an inventory that was found.

**The system-information report can be written with no window, and it names versions and paths.** `blender --background --python-expr "import bpy; print('SYSINFO', bpy.ops.wm.sysinfo(filepath='system-info.txt'))"` printed `SYSINFO {'FINISHED'}`, exit status 0, and wrote the file with no prompt of any kind. Its add-on section reads:

```
Enabled add-ons:
====================================

io_anim_bvh (version: (1, 0, 1), path: '/Applications/Blender.app/Contents/Resources/5.2/scripts/addons_core/io_anim_bvh/__init__.py')
io_curve_svg (version: UNKNOWN, path: '.../io_curve_svg/__init__.py')
io_mesh_uv_layout (version: (1, 2, 0), path: '.../io_mesh_uv_layout/__init__.py')
io_scene_fbx (version: ..., path: '.../io_scene_fbx/__init__.py')
io_scene_gltf2 (version: ..., path: '.../io_scene_gltf2/__init__.py')
cycles (version: UNKNOWN, path: '.../cycles/__init__.py')
pose_library (version: (2, 0), path: '.../pose_library/__init__.py')
bl_pkg (version: (0, 0, 1), path: '.../bl_pkg/__init__.py')
```

(paths shortened after the first; each is the full path in the file). Note that the version reads `UNKNOWN` for add-ons whose metadata block carries no version, Cycles among them.

**Which Blender the answer belongs to is a matter of the folder.** One settings folder was shared between all four installed Blenders. An add-on installed once, by 5.2, was listed and switched on by 5.1, 5.0 and 4.5 as well, from the same folder, and the folder grew no version-named subfolder. So each Blender reports what is in the folder it was pointed at; nothing in Blender refuses another version's add-on.

**Outside the interface.** Settled: all three lists and the report are reachable with Blender driven from outside its window, and the report writes without prompting.

**Operating system.** Settled for macOS only, on the one machine described above. The same tests were run again on a rented Linux machine, and that result has its own block under this question; the two are never merged. Windows stays untested.

### Settled at the window — what an add-on's row reports about itself (Blender 5.2 only)

**Label: observed.** Run by the Creator on 2026-08-05, at Blender's own window, on their own MacBook Pro (Apple M1 Pro chip, macOS 26.5.2), in Blender 5.2. Blender was started from a terminal pointed at a throwaway settings folder of its own, so their own Blender settings were not touched. **This was run in Blender 5.2 only**, so it says nothing about 4.5, 5.0 or 5.1. The Creator works on a Mac, so this settles macOS and leaves Windows and Linux open.

**What was done.** In Edit ▸ Preferences ▸ Add-ons, the "Cycles Render Engine" row and the "Node Wrangler" row were opened with the arrow at the left. The Creator returned a screenshot.

```
Cycles Render Engine                                    [ticked]
Cycles renderer integration.                            Built-in
Website        docs.blender.org
Type           Core
Preferences
  [ None | Metal ]                (Metal is the one selected)
  [x] Apple M1 Pro (GPU - 14 cores)
  [ ] Apple M1 Pro
  Kernel Optimization    Full
  MetalRT                Auto
```

```
Node Wrangler                                           [unticked]
Various tools to enhance and speed up node-based workflow.    Built-in
Website        docs.blender.org
Type           Core
Maintainer     Blender Foundation
Version        4.1.0
```

**So the window does report a version — but not for every add-on.** Node Wrangler's row shows `Version  4.1.0` and `Maintainer  Blender Foundation`; Cycles' row shows neither. That matches what the system-information report gives from outside the window, where Cycles' version reads `UNKNOWN` (recorded above, 2026-08-04). No repository name appears in either row, and no location on disk.

**Not covered by this result.** Both add-ons opened are ones that ship with Blender. What an extension installed from a repository shows in its own row was not reported, so whether a licence, a repository name or a declared permission appears for *that* kind of add-on is still unsettled.

**Outside the interface.** Settled both ways. The three lists reachable from outside the window are recorded above; the window's own row adds a version and a maintainer for add-ons that declare them, and adds no repository name.

**Operating system.** macOS only, and Blender 5.2 only.

### Settled by hand test on the rented Linux machine — what can be asked of a computer about its add-ons

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.2.0 LTS from `/opt/blender/blender-5.2.0-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

**What is switched on.** `blender --background --python-expr "import bpy; a=bpy.context.preferences.addons; print('COUNT', len(a)); print('MODULES', sorted(x.module for x in a))"` printed:

```
COUNT 8
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

The same eight as on macOS. That collection drops an add-on the moment it is switched off: after switching the test add-on off and saving the preferences, its module was gone from this list while `blender --command extension list` still printed `smoozy_test [installed]`. Both machines agreed.

**What is on the machine at all.** `blender --background --python-expr "import addon_utils; print('PATHS', addon_utils.paths()); m=list(addon_utils.modules()); print('COUNT', len(m)); ..."` printed:

```
PATHS ['/opt/blender/blender-5.2.0-linux-x64/5.2/scripts/addons_core']
COUNT 13
```

and then one line per add-on with its whole metadata block. The add-ons named were `io_anim_bvh`, `bl_pkg`, `cycles`, `io_scene_fbx`, `io_scene_gltf2`, `hydra_storm`, `ui_translate`, `node_wrangler`, `pose_library`, `rigify`, `io_curve_svg`, `io_mesh_uv_layout`, `viewport_vr_preview` — the same set, in the same order, that the macOS run recorded for this version.

**The system-information report can be written with no window and with no prompt.** `blender --background --python-expr "import bpy; print(bpy.ops.wm.sysinfo(filepath='system-info.txt'))"` printed `{'FINISHED'}`, exit status 0, and the file's "Enabled add-ons" section read:

```
Enabled add-ons:
========================================

io_anim_bvh (version: (1, 0, 1), path: '.../addons_core/io_anim_bvh/__init__.py')
io_curve_svg (version: UNKNOWN, path: '.../addons_core/io_curve_svg/__init__.py')
io_mesh_uv_layout (version: (1, 2, 0), path: '.../addons_core/io_mesh_uv_layout/__init__.py')
io_scene_fbx (version: (5, 15, 0), path: '.../addons_core/io_scene_fbx/__init__.py')
io_scene_gltf2 (version: (5, 2, 39), path: '.../addons_core/io_scene_gltf2/__init__.py')
cycles (version: UNKNOWN, path: '.../addons_core/cycles/__init__.py')
pose_library (version: (2, 0), path: '.../addons_core/pose_library/__init__.py')
bl_pkg (version: (0, 0, 1), path: '.../addons_core/bl_pkg/__init__.py')
```

The paths above are cut to their last two parts; each is the full path inside this machine's Blender folder. It names each add-on's version and path, with `UNKNOWN` where the metadata block carries none — Cycles among them. The same as the macOS run.

**Which Blender the answer belongs to is a matter of the folder alone.** One settings folder handed to all four Blenders in turn gave each of them the same installed add-on — see question 6.

**Outside the interface.** Settled again on a second operating system: all three routes, and the system-information report, are reachable with Blender driven from outside its own window and with no prompt of any kind.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed on the routes, the switched-on set and the bundled set. Windows stays untested.

### Settled by hand test on the rented Linux machine — what the Add-ons list and an opened row show

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 5.2.0 LTS, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`.

**The Add-ons list holds the add-ons that ship with Blender, and nothing else.** The picture of the list in a fresh settings folder shows, in order: BioVision Motion Capture (BVH) format, Cycles Render Engine, FBX format, glTF 2.0 format, Hydra Storm render engine, Manage UI translations, Node Wrangler, Pose Library, Rigify, Scalable Vector Graphics (SVG) 1.1 format, UV Layout, VR Scene Inspection. Each has a tick box, and the ones ticked are the ones the no-window run lists as switched on. The extensions system itself, `bl_pkg`, is in that switched-on list but has no row of its own.

**An opened row shows a version and a maintainer for one add-on and neither for Cycles.** Node Wrangler's row carries `Maintainer` with five names and `Version 3.55`; Cycles' row carries neither. That matches the `UNKNOWN` the system-information report gives for Cycles from outside the window. Neither row shows a repository name or a location on disk. The same as the Creator's macOS run in Blender 5.2.

**Outside the interface.** This is a window test. The matching no-window results are under question 10 above, and the two agree on which add-ons are there and which are switched on.

**Operating system.** Settled for Linux in this version, at the window. The Creator's macOS run covered Blender 5.2 only. Windows stays untested.


### Short answer

**Yes**, and without a person at a screen. Blender has a command for it: `blender --command extension list`. There is also a Python route to the enabled add-ons and to the folders add-ons live in. As for "for which Blender": add-ons are held in a folder named after the Blender version, so the answer is per-version by construction, and the version can be asked for directly.

I read the question two ways and answer both, because it is ambiguous. Reading one: can you ask a machine what it has installed, and tell which Blender install each thing belongs to. Reading two: can you ask what it has, and tell which Blender versions each add-on says it supports. Both are answerable.

### Asking the machine

> "Subcommand: list
>
> usage:
>
> blender --command extension list [-h] [-s]
>
> List packages from all enabled repositories.
>
> options:
>
> -h, --help
>
> show this help message and exit
>
> -s, --sync
>
> Sync the remote directory before performing the action."

And, alongside it:

> "Subcommand: repo-list
>
> usage:
>
> blender --command extension repo-list [-h]
>
> List all repositories stored in Blender’s preferences."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/extension_arguments.html — read 2026-07-30. **documented.**

Note the wording: "from all enabled repositories". A repository can be switched off (`enabled`, "Enable the repository (default False)" — Python API, https://docs.blender.org/api/5.2/bpy.types.UserExtensionRepo.html, read 2026-07-30), and the sources do not say what `list` does about a switched-off one beyond that phrase.

Through Python, the enabled add-ons are a collection on the preferences:

> "addons
>
> (default None, readonly)
>
> Type:
>
> Addons[Addon]"

Source: Python API, https://docs.blender.org/api/5.2/bpy.types.Preferences.html — read 2026-07-30. Each entry carries its name:

> "class bpy.types.Addon(bpy_struct)
>
> Python add-ons to be loaded automatically"
>
> "module
>
> Module name (default “”, never None)"

Source: Python API, https://docs.blender.org/api/5.2/bpy.types.Addon.html — read 2026-07-30. **documented** that this collection exists and what an entry holds. **Inferred**, and worth flagging as the weak point: the phrase "to be loaded automatically" reads as "the ones switched on", which would mean this collection is not a list of everything installed. The sources do not say so outright.

For the folders themselves, the manual points at a Python call:

> "You can also find add-on path locations by running this in the Python Console:
>
> import addon_utils
> print(addon_utils.paths())"

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/scripting/addon_tutorial.html — read 2026-07-30. **documented.**

And the API gives the paths directly:

> "bpy.utils.user_resource(resource_type, *, path='', create=False)
>
> Return a user resource path (normally from the users home directory).
>
> Parameters:
>
> resource_type (Literal['DATAFILES', 'CONFIG', 'SCRIPTS', 'EXTENSIONS']) – The resource type."

and

> "bpy.utils.script_paths(*, subdir=None, user_pref=True, check_all=False, use_user=True, use_system_environment=True)
>
> Returns a list of valid script paths."

Source: Python API, https://docs.blender.org/api/5.2/bpy.utils.html — read 2026-07-30. **documented.**

The folders and what sits in them are set out plainly:

> "./extensions
>
> Extension repositories.
>
> Located in both user and system directories. Repositories are loaded from
> both directories."
>
> "./scripts/addons/*.py
>
> Python add-ons which may be enabled in the Preferences include import/export format support,
> render engine integration and many handy utilities."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html — read 2026-07-30. **documented.** So the question can also be answered by reading the disk, with no Blender running at all — **inferred**, the step being that a folder listing is a list of what is held.

### For which Blender

**Reading one — which Blender install.** The folders are named after the version:

> "Linux
>
> $HOME/.config/blender/5.2/
> …
> macOS
>
> /Users/$USER/Library/Application Support/Blender/5.2/
>
> Windows
>
> %USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.2\"

and, for the bundled side:

> "Linux
>
> Archive downloaded from blender.org:
>
> ./5.2/"

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html — read 2026-07-30. **documented.** The same page notes several installs can sit side by side; the deployment page says so outright:

> "Multiple Blender versions can
> co-exist on the same system, and deployment can be automated using
> standard file management tools."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html — read 2026-07-30. **documented.**

Two ways round that are documented and would break the version-named-folder rule: a folder called `portable` next to the program, and the `BLENDER_USER_RESOURCES` variable, "can be set to a custom directory to replace the default user directory" (same directory-layout page). **documented.**

Blender will also tell you which one it is:

> "bpy.app.version
>
> The Blender version as a tuple of 3 numbers (major, minor, micro). eg. (4, 3, 1)"

and the path lookup is itself version-aware:

> "bpy.utils.resource_path(type, *, major=None, minor=None)
>
> Return the base path for storing system files.
> …
> major (int | None) – Major version. None (the default) uses bpy.app.version[0].
>
> minor (int | None) – Minor version. None (the default) uses bpy.app.version[1]."

Sources: Python API, https://docs.blender.org/api/5.2/bpy.app.html and https://docs.blender.org/api/5.2/bpy.utils.html — read 2026-07-30. **documented.** That second one is the sharpest thing here: a running Blender can be asked for the resource folder of a *different* version than itself.

**Reading two — which Blender versions an add-on says it supports.** Recorded in the manifest, and required:

> "blender_version_min:
>
> Minimum supported Blender version - use at least 4.2.0."
>
> "blender_version_max:
>
> Blender version that the extension does not support, earlier versions are supported."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/getting_started.html — read 2026-07-30 (`blender_version_min` under required values, `blender_version_max` under optional). **documented.**

For a legacy add-on the same information is in the old metadata block, and the manual states the consequence:

> "It also specifies the minimum Blender version required to run the script;
> older versions won’t display the add-on in the list."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/scripting/addon_tutorial.html — read 2026-07-30. **documented.** An example: `"blender": (2, 80, 0)` — `Blender Sources/cycles_5.2/__init__.py`, line 10 — **observed**. (*Citation check, 2026-08-05: this said line 9, which is `"author": "",`.*)

The version is also part of what a download service is told, alongside the kind of computer: "platform / blender_version" (dynamic repository page, cited under question 4).

### Not covered

- **What `blender --command extension list` actually prints.** The manual gives the command and one sentence about it. There is no example output anywhere in the sources, so I cannot say whether it names versions, repositories, enabled state, or anything else. This is the single largest gap in this answer.
- Whether `list` includes legacy add-ons (which are not packages in a repository) or only extensions.
- Whether `bpy.context.preferences.addons` holds installed-but-switched-off add-ons.
- Whether any of these routes report an add-on's own version number.
- What `addon_utils.paths()` returns, beyond "add-on path locations" — `addon_utils` is not documented in the Python API at all (see below).
- Whether one Blender can be asked about another Blender's add-ons. `bpy.utils.resource_path` takes a version and returns a path, which is suggestive, but the sources do not say the add-ons found there could be listed or read by the running one.

### Outside the interface

This holds outside the interface, and the best of it only exists there. `blender --command extension list` and `repo-list` are command-line subcommands; the manual's whole extension-arguments page is written for a Blender with no window. The Python routes work through `--python` / `--python-expr`, documented on https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html. The folder-reading route needs no Blender at all. One caution the sources do state: `--factory-startup` "Skip reading the startup.blend in the users home directory" (same page) — what that does to the add-on list is not said, and I am not guessing.

### Operating system

Holds on all three, with the folder paths differing as quoted above. The command and the Python calls are described without any operating-system qualification. **Documented** for the paths; **the sources do not say** whether anything else differs.

### Where I looked

Searched all of `manual_5.2` and `api_5.2` (excluding the generated index files, which are just word lists) for `installed extensions`, `list installed`, `which add-ons`, `list of add-ons`, `list all packages` — seven hits total, all quoted or accounted for above (the directory-layout page twice, the extension-arguments page, the workspaces page, and three properties on `bpy.ops.extensions` / `bpy.types.WindowManager`).

Searched everything for `addon_utils` — **four hits, all in the manual, none in the API documentation**: two in the deployment page (`addon_utils.enable("my-addon")`) and two in the add-on tutorial (`addon_utils.paths()`). So `addon_utils` is a module the manual uses but the API documentation does not describe. That is why I claim nothing about what `paths()` returns.

Listed every operator under `bpy.ops.preferences` beginning `addon` or `extension`: `addon_disable`, `addon_enable`, `addon_expand`, `addon_install`, `addon_refresh`, `addon_remove`, `addon_show`, `extension_repo_add`, `extension_repo_remove`, `extension_url_drop`, plus `script_directory_add` / `script_directory_remove`. Listed every operator under `bpy.ops.extensions`: 30 of them, from `package_disable` to `userpref_tags_set`. **There is no listing or query operator among either set** — every one of them acts, none reports. That is why the answer runs through the command line, the preferences collection and the folders instead.

Listed every function in `bpy.utils` (39 of them) and read `resource_path`, `user_resource`, `script_paths`, `script_path_user`, `extension_path_user`, `modules_from_path` and `load_scripts` in full. Read the whole attribute list of `bpy.app` (60-odd entries) and the version-related ones in full. Read `bpy.types.Addon`, `bpy.types.Addons`, `bpy.types.Preferences` (the `addons` entry), `bpy.types.PreferencesExtensions`, `bpy.types.UserExtensionRepo` and the extension-related entries of `bpy.types.WindowManager` in full. Opened the manual picture `_images/editors_preferences_section_addons.webp` (https://docs.blender.org/manual/en/5.2/editors/preferences/addons.html): it shows a plain list of add-on names with tick-boxes and an expand arrow each — **no version numbers and no repository names are visible in it**, which is why I do not claim the interface shows them.

---

## 11. Can an add-on depend on something else being on the computer, or on a computer elsewhere, before it will work, and can what it needs be read anywhere?

### Settled by hand test — what can be read about what an add-on needs

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.2.0. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

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

**A bundled module is unpacked into a shared place, not into the add-on's own folder.** An add-on carrying three bundled modules of one library — one each for macOS on Apple Silicon, Linux and Windows — was installed. All three files stayed unopened in the add-on's own `wheels` folder, while the unpacked code went to `<settings folder>/extensions/.local/lib/python3.13/site-packages/`, and only the one for this machine was unpacked. It could then be used from a plain background run: `import PIL` printed that path and `12.3.0`.

**The manifest can be read straight off disk.** The installed add-on's `blender_manifest.toml` sat at `<settings folder>/extensions/user_default/smoozy_test/blender_manifest.toml` and its content was the same text that was packed, unchanged. A generated repository listing carries the same fields plus the package's size and a hash.

**Outside the interface.** Settled: the online-access properties, the validate command and the manifest on disk are all reachable with Blender driven from outside its window.

**Operating system.** Settled for macOS only, on the one machine described above. Which bundled module is needed does differ by kind of computer; the macOS-on-Apple-Silicon case was tested here and the Linux case afterwards on a rented Linux machine, in its own block under this question. Windows stays untested.

### Settled at the window — whether a declared permission is shown anywhere (Blender 5.2 only)

**Label: observed.** Run by the Creator on 2026-08-05, at Blender's own window, on their own MacBook Pro (Apple M1 Pro chip, macOS 26.5.2), in Blender 5.2. Blender was started from a terminal pointed at a throwaway settings folder of its own, so their own Blender settings were not touched. **This was run in Blender 5.2 only**, so it says nothing about 4.5, 5.0 or 5.1. The Creator works on a Mac, so this settles macOS and leaves Windows and Linux open.

**What was done.** In Edit ▸ Preferences ▸ Add-ons, the "Cycles Render Engine" row and the "Node Wrangler" row were opened with the arrow at the left. The Creator returned a screenshot. The fields shown were:

```
Cycles Render Engine                                    [ticked]
Cycles renderer integration.                            Built-in
Website        docs.blender.org
Type           Core
Preferences
  [ None | Metal ]                (Metal is the one selected)
  [x] Apple M1 Pro (GPU - 14 cores)
  [ ] Apple M1 Pro
  Kernel Optimization    Full
  MetalRT                Auto
```

```
Node Wrangler                                           [unticked]
Various tools to enhance and speed up node-based workflow.    Built-in
Website        docs.blender.org
Type           Core
Maintainer     Blender Foundation
Version        4.1.0
```

**No declared permission appears in either row**, and no bundled module and no dependency of any kind. The only field in either row that names something outside Blender is `Website`, which reads `docs.blender.org` for both.

**Not covered by this result.** Both add-ons opened are ones that ship with Blender. What an extension installed from a repository shows in its own row was not reported, so whether a licence, a repository name or a declared permission appears for *that* kind of add-on is still unsettled. Neither of the two add-ons opened declares any permission in the first place, so this settles what the row shows for these two and not what a row looks like for an add-on that does declare one.

**Outside the interface.** Unchanged: `blender --command extension validate` reports nothing but `Success parsing TOML` (2026-08-04, recorded above), and the window shows nothing either. Reading the manifest as a file remains the only route found.

**Operating system.** macOS only, and Blender 5.2 only.

### Settled by hand test on the rented Linux machine — what can be read about what an add-on needs, and what happens when what it needs is built for another machine

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.2.0 LTS from `/opt/blender/blender-5.2.0-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

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

**A bundled module is unpacked into a shared place, not into the add-on's own folder**, and can then be used from a plain run with no window. Here that place is `<settings folder>/extensions/.local/lib/python3.13/site-packages/`, and the compiled parts unpacked there were the Linux ones — `_imaging.cpython-313-x86_64-linux-gnu.so` and seven more. `import PIL` printed that path and `12.3.0`; `from PIL import Image` built a 2 by 2 image. Full record under question 4.

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
PROP use_hiprt BOOLEAN True
PROP use_oneapirt BOOLEAN True
```

Note that `metalrt`, which names Apple's own graphics, is present on this Linux machine too. The macOS run recorded the property list but not the values of `use_hiprt` and `use_oneapirt`, so those two are established here only.

**Outside the interface.** Settled again on a second operating system: everything above was read, and every package built and installed, with Blender driven from outside its own window.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed on the property spellings, the validate output and where a bundled module lands; each unpacked its own machine's compiled files. What happens when the bundled module for the running machine is absent was run on Linux only. Windows stays untested, and the 4.5 release note about a Python library dependency failing from subprocesses, whose wording points at Windows, was not tried.

### Settled by hand test on the rented Linux machine — what an opened row shows about what an add-on needs

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 5.2.0 LTS, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`.

**No declared permission, no bundled module and no dependency of any kind appears in an add-on's opened row.** The two rows opened carry a website, a type, and for one of them a maintainer and a version. The only field naming anything outside Blender is the website address. Neither add-on opened declares a permission in the first place, so this settles what those two rows show and nothing more — the same limit the Creator's macOS run had.

**The one add-on setting on screen that names something outside Blender is Cycles' own device row**, `None | CUDA | OptiX | HIP | oneAPI`, which names kinds of graphics hardware rather than anything the add-on requires. The System preferences also carry a **Network** section with an **Allow Online Access** tick box, unticked in a fresh settings folder, which is the preference the online-access property reads.

**Outside the interface.** This is a window test. The matching no-window results are under question 11 above.

**Operating system.** Settled for Linux in this version, at the window. The Creator's macOS run covered Blender 5.2 only. Windows stays untested.


### Short answer

The documented position is the opposite of a dependency system: an add-on packaged as an extension **must not** depend on anything else being on the computer — it has to bring what it needs with it. The manual says so in as many words and gives three ways to comply. There is no field in the manifest for "this needs that", and the manifest's fields are listed exhaustively.

Depending on a computer elsewhere is a different matter and is documented in two shapes: internet access, which an add-on must declare and must check for at run time; and the repository it came from, which is a machine elsewhere but is only involved in installing and updating, not in working.

What an add-on needs **can** be read, in one place: its manifest, a plain text file that ships in the package and sits beside the installed add-on. It lists the bundled modules and the declared permissions. There is no way in the sources to read it through Blender's Python interface.

### Depending on something else on the computer — the rule

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
> This is recommended if an add-on depends on another add-on.
>
> Make sure that both the individual and the combined add-on
> check for already registered types (Operators, Panels, …).
> This avoids duplication of operators and panels on the interface
> if the add-ons are installed as a bundle and individually.
>
> Bundle with Vendorize
>
> This can be used as a way to bundle a pure Python dependencies as a sub-module.
>
> This has the advantage of avoiding version conflicts although it requires some work to setup each package."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/addons.html — read 2026-07-30. **documented.**

Note what the middle option concedes: an add-on *can* depend on another add-on ("This is recommended if an add-on depends on another add-on") — and the documented remedy is to ship both together in one package, not to declare the dependency.

The manifest has no field for declaring a dependency. The manual's list of manifest values is exhaustive — required: `blender_version_min`, `id`, `license`, `maintainer`, `name`, `schema_version`, `tagline`, `type`, `version`; optional: `blender_version_max`, `website`, `copyright`, `tags`, `platforms`, `wheels`, `permissions`; plus a `build` table with `paths` and `paths_exclude_pattern`, and a reserved `[build.generated]`. Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/getting_started.html — read 2026-07-30. **documented**, and documented as complete, since the page ends the list with:

> "Reserved:
>
> These values must not be declared in a TOML and are reserved for internal use."

There is one more documented "must not assume", about the computer's own disk:

> "Add-ons must not assume their own directory is user writable since this may not be the case for “System” repositories.
> Writing files into the add-on’s directory also has the down side that upgrading the extension will remove all files.
>
> Add-ons which need their own user directory should use a utility function provided for this purpose:
>
> extension_directory = bpy.utils.extension_path_user(__package__, path="", create=True)"

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/addons.html — read 2026-07-30. **documented.**

And one documented way an add-on could reach outside its bundle to the computer's own Python, though it is a thing the person launching Blender turns on, not a thing the add-on declares:

> "--python-use-system-env
>
> Allow Python to use system environment variables such as PYTHONPATH and the user site-packages directory."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30. **documented.** The deployment page adds the caution: "Blender bundles a complete Python installation and does not interact with the system Python by default. This can be changed with the --python-use-system-env command line argument, if care is taken to set up a compatible Python version." (https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html, read 2026-07-30.)

### Depending on a computer elsewhere

**While running — internet access.** This is declared, and must also be checked at run time:

> "# # Optional: add-ons can list which resources they will require:
> # # * files (for access of any filesystem operations)
> # # * network (for internet access)
> # # * clipboard (to read and/or write the system clipboard)
> # # * camera (to capture photos and videos)
> # # * microphone (to capture audio)
> …
> # [permissions]
> # network = "Need to sync motion-capture data to server"
> # files = "Import/export FBX from/to disk"
> # clipboard = "Copy and paste bone transforms""

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/getting_started.html — read 2026-07-30. **documented.**

> "Internet Access
>
> If the add-on needs to use internet, it must check for the (read-only) property bpy.app.online_access.
> This option is controlled by Preferences, which can be overriding via command-line
> (--offline-mode / --online-mode)."
>
> "Note
>
> Add-ons that follow this setting will only connect to the internet if enabled.
> However, Blender cannot prevent third-party add-ons from violating this rule."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/addons.html — read 2026-07-30. **documented**, and the second half of it matters: the declaration is a statement of intent, not a fence.

The matching properties:

> "bpy.app.online_access
>
> Boolean, true when internet access is allowed by Blender & 3rd party scripts (read-only)."
>
> "bpy.app.online_access_override
>
> Boolean, true when internet access preference is overridden by the command line (read-only)."

Source: Python API, https://docs.blender.org/api/5.2/bpy.app.html — read 2026-07-30. **documented.**

The deployment page states the default and how to force it:

> "By default Blender does not access the internet, however this can be
> enabled in the System preferences with the
> Online Access option.
>
> Working offline can be enforced by running with the --offline-mode
> command line argument. Users
> will then be unable to enable online access in the preferences."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html — read 2026-07-30. **documented.**

**While installing and updating — the repository.** An add-on comes from somewhere, and that somewhere can be a machine elsewhere, optionally behind a token:

> "remote_url
>
> Remote URL to the extension repository, the file-system may be referenced using the file URI scheme: “file://” (default “”, never None)"
>
> "access_token
>
> Personal access token, may be required by some repositories (default “”, never None)"

Source: Python API, https://docs.blender.org/api/5.2/bpy.types.UserExtensionRepo.html — read 2026-07-30. **documented.**

> "Some repositories may require authentication. The user can specify an access token for a repository,
> which is passed along with the API request from Blender.
>
> This is passed to the servers via an Authorization header"

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/creating_repository/dynamic_repository.html — read 2026-07-30. **documented.**

A picture in the manual states the consequence outright. The Get Extensions preferences screenshot shows the text:

> "Internet access is required to install and update online extensions.
>
> You can adjust this later from "System" preferences.
>
> While offline, use "Install from Disk" instead."

with two buttons, "Continue Offline" and "Allow Online Access".

Source: manual picture `_images/editors_preferences_section_extensions.webp`, shown on https://docs.blender.org/manual/en/5.2/editors/preferences/extensions.html — read 2026-07-30. **documented** (it is part of the manual, and what I have taken from it is the text visible in it).

The same page's text confirms an installed add-on does not keep needing that machine, at least for updates:

> "Install from Disk
>
> Install an extension from a .zip package.
> This is installed to a Local Repository and no updates will be available."

Same page, same date. **documented.**

And a repository need not be elsewhere at all — the manual documents a read-only System repository that "could be located on a read-only network drive or in a system directory", set by `$BLENDER_SYSTEM_EXTENSIONS` (https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html, read 2026-07-30). A network drive is a computer elsewhere that an installation can sit on. **documented.**

### Can what it needs be read anywhere

Yes — from the manifest, as a file.

The manifest travels in the package ("An extension is shared as a .zip archive containing a manifest file and other files") and, once installed, sits beside the add-on. The deployment page gives the exact shape of the installed path:

> "Extensions packages should be extracted in this system directory,
> with a resulting path like this:
>
> $BLENDER_SYSTEM_EXTENSIONS/system/my-addon/blender_manifest.toml"

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html — read 2026-07-30. **documented.**

What it holds, for this question, is the bundled-module list and the declared permissions:

> "wheels = [
> "./wheels/pillow-12.1.0-cp313-cp313-macosx_11_0_arm64.whl",
> "./wheels/pillow-12.1.0-cp313-cp313-manylinux_2_27_x86_64.manylinux_2_28_x86_64.whl",
> "./wheels/pillow-12.1.0-cp313-cp313-win_amd64.whl",
> ]"

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/python_wheels.html — read 2026-07-30. **documented.**

A package can also be checked without unpacking it:

> "You may also validate a package without having to extract it first.
>
> blender --command extension validate add-on-package.zip"

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/getting_started.html — read 2026-07-30. **documented.** (What `validate` reports is not described beyond "Validate the package meta-data".)

Blender itself reads this file for display:

> "The only difference between add-on extensions and built-in Python modules is that add-ons must include a
> blender_manifest.toml which Blender uses to read metadata such as name, author, tags and project link.
> The User Preferences add-on listing uses this to display information about each add-on."

Source: Python API, https://docs.blender.org/api/5.2/info_overview.html — read 2026-07-30. **documented.** Note the four things named — name, author, tags, project link — do not include permissions or wheels.

**Inferred**, with the step: since the manifest is a plain TOML file at a known place beside the installed add-on, anything that can read a file can read an add-on's declared permissions and bundled modules without running Blender. The sources describe the file and its location; they do not say "you may read it yourself".

For a **legacy** add-on there is no manifest, and the old metadata block has no field for what it needs either. Its fields, as shipped in Cycles: `name`, `author`, `blender`, `description`, `warning`, `doc_url`, `tracker_url`, `support`, `category` — `Blender Sources/cycles_5.2/__init__.py`, lines 7–16 — **observed**. The manual's own description of `bl_info` names "the title, version and author" and the minimum Blender version, nothing more (https://docs.blender.org/manual/en/5.2/advanced/scripting/addon_tutorial.html, read 2026-07-30).

### Not covered

- **There is no route in the sources to read a manifest through Blender's Python interface.** I searched the whole API documentation for `manifest` and found exactly one mention, in prose, on `info_overview`. So an answer of "read it as a file" is the only one available.
- What Blender does with a declared permission. The sources say authors should declare them and give a reason for each; nothing says Blender checks, enforces or even shows them. The one enforcement-shaped sentence in the sources says the opposite: "Blender cannot prevent third-party add-ons from violating this rule."
- Whether an add-on that fails to find something it needs reports it in any particular way. The nearest is "If the Add-on does not activate when enabled, check the Console window for any errors that may have occurred" (https://docs.blender.org/manual/en/5.2/editors/preferences/addons.html), which is generic.
- Whether an installed extension records where it came from in a way that can be read back. `UserExtensionRepo` holds the repository's URL, but that is the repository's setting, not the add-on's.
- Bundling one add-on inside another is documented as a practice, but nothing describes how a reader would tell, from the outside, that a package contains a second add-on.
- Whether the "network" permission has any effect on what `bpy.app.online_access` reports. Nothing connects the two beyond the manifest comment telling authors to check it.

### Outside the interface

Holds outside the interface, and much of it is written for that setting: `--offline-mode` / `--online-mode` override the preference from the command line, `--python-use-system-env` is a launch argument, `blender --command extension validate` is a command-line tool, and `$BLENDER_SYSTEM_EXTENSIONS` is an environment variable. `bpy.app.online_access` is a property a script reads, with no window involved. The one item that is interface-only is the "Continue Offline / Allow Online Access" prompt in the picture above; the deployment page's `--offline-mode` covers the same ground without it. The sources do not otherwise describe a difference.

### Operating system

The sources do not say that any of this differs by operating system, and none of it is stated per operating system. The one operating-system-shaped detail is that the bundled modules an add-on carries are per kind of computer (question 4). **The sources do not say** anything more.

### Where I looked

Searched all of `manual_5.2`, `api_5.2` and `release_notes_5.2` for `blender_manifest` — five files: three manual pages, one manual command-line page, one API page. All five opened and read. Searched the API alone for `manifest` — one hit. Searched everything for `online_access` — the manual hits are the two extension pages, the API hits are `bpy.app`, `bpy.types.PreferencesSystem`, `bpy.types.PreferencesExtensions` and `bpy.types.PreferencesFilePaths`; all read. Searched for `site-packages` and `sys.path` — the hits are the directory-layout page, the wheels page, the `--python-use-system-env` argument, the Python troubleshooting page and three `bpy.utils` entries; all read.

Opened in full: manual `advanced/extensions/addons`, `advanced/extensions/getting_started`, `advanced/extensions/python_wheels`, `advanced/extensions/creating_repository/dynamic_repository`, `advanced/extensions/creating_repository/static_repository`, `advanced/command_line/extension_arguments`, `advanced/deploying_blender`, `editors/preferences/extensions`, `editors/preferences/addons`, `troubleshooting/network`, `troubleshooting/python`; API `bpy.app`, `bpy.utils`, `bpy.types.UserExtensionRepo`, `bpy.types.PreferencesExtensions`, `bpy.ops.extensions`, `info_overview`.

Ruled out: `troubleshooting/network.html` turned out to be a single note about a 403 error reaching extensions.blender.org through SSL-managing networks, which I read in full and which adds nothing to the question. `bpy.types.PreferencesFilePaths.use_extension_online_access_handled` and `bpy.types.PreferencesExtensions.use_online_access_handled` are both about whether the user has been shown the online-access prompt, not about what an add-on needs.

---

## What is still open

Every test on both lists has now been run, and the results are written into the answers above as observed findings. The Mac list, with each test's set-up and steps, is at `Audits/Blender Research/Add-ons/Hand tests.md`; the Linux list is at `Audits/Blender Research/Add-ons/Hand tests — Linux.md`.

**Two machines, never merged.** Everything was run on the Creator's MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics), and then again on a rented Linux machine (Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH). Where a test was run on both, each machine's result is written up in its own block and the finding says whether they agreed.

**The five window tests were run on both machines.** The Creator ran them on macOS in Blender 5.2 on 2026-08-05; they were run again at Blender's own window on the Linux machine, in all four versions, on 2026-08-06. Three gaps the Creator's answers left were closed by the Linux run: the Render Engine field reads `EEVEE` before its list is opened; the refused package does not appear anywhere in the Add-ons list afterwards; and the Add-ons list itself was photographed in full. One gap stands in both runs — both add-ons opened were add-ons that ship with Blender, so what an extension installed from a repository shows in its own row is still unsettled. (Questions 2, 4, 8, 10, 11.)

**Three things the Linux machine settled that the Mac could not**: what Blender does with an add-on carrying a compiled part built for another kind of computer; what happens when an installed, set-up add-on is carried across to a machine of a different kind; and what Cycles' own device setting reads and offers on a machine whose graphics are not Apple's. All three are in the answers above.

**Driving a window over SSH: what works and what does not.** The machine has `xdotool`. Blender receives button and key events sent to its own window — its own modal handler printed them back with the coordinates that were sent — but it receives **no pointer-motion events at all**, and **a click could not be shown to work any control**: three separate clicks aimed at a Properties tab, a preferences section and a tick box each left the matching property unchanged. So nothing in this document was got by clicking. The workspace "Filter Add-ons" panel, the Get Extensions repositories drop-down and the render-engine list were each opened instead by one of Blender's own operators, with the identifier read out of `bpy.types` first, and each finding quotes the operator and what it returned. One thing is still unread: in Blender 4.5, 5.0 and 5.1 a tooltip covered the repositories menu in every picture taken, so what those three versions list there was not seen.

**Cannot be settled with the equipment the project has** — one MacBook Pro with Apple's own graphics; one rented Linux machine with one NVIDIA card, **which was destroyed on 2026-08-06 and can never be run again**; and a second rented Linux machine with two NVIDIA cards and no display server at all. *(Two-card citation check, 2026-08-06: this named two machines and called the rented one a container. There are three, one of them gone, and the list below was written when there were two.)*

- Anything on Windows. That leaves untested the "is not a valid Win32 application" failure the manual describes, the `file://HOST/share/...` repository address the sources give for Windows only, and whether the manual's line about headless Windows systems touches add-ons. (Questions 3, 4, 5, 8, 11.)
- A computer with **no graphics hardware at all**. *(Two-card citation check, 2026-08-06: this line read "A computer with no screen attached at all. Both machines have a screen…" and is out of date. A third machine, with two NVIDIA cards and **no display server at all**, ran the whole add-on cycle on 2026-08-06; that result is written up under question 3 of this version's main findings document, not here, this document not covering question 3. What is left is a machine with no graphics device, which no machine the project has ever had.)* (Question 3.)
- A graphics card of a make that is neither Apple's nor NVIDIA's. **Two graphics cards in one machine can now be reached** — the third machine has two — but nothing in this topic's questions turns on them, so it is left alone rather than half-answered; what two cards do to Cycles' own device list belongs to *The machine*. (Questions 3, 4.) *(Two-card citation check, 2026-08-06: the second half of this line was no longer true.)*
- How add-ons are sold, and anything about licence keys, activation or entitlement. No test on a computer can answer it. (Question 8.)
- A third-party extensions site. No address for one appears in the sources, so none can be tried without guessing at one. A repository made from a folder was tested instead, on both machines. (Questions 5, 8.)

---

2026-08-04 — Opus 5 (Claude Code) — Add-ons, Blender 5.2
