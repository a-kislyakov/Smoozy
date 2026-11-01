# Add-ons — Blender 5.1 — second pass (questions 2, 3, 4, 8)

**Sources consulted, and when.** Blender 5.1 manual (`Blender Sources/manual_5.1`, including its `_images` pictures), Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`). One page outside the per-version sets was also read: `Blender Sources/requirements/requirements_2026-08-01.txt`, Blender's requirements page. All read on **2026-07-30** (the snapshot date given for the documentation), except the requirements page, whose own snapshot date is 2026-08-01.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

I did this work alone. No helper agents were started, so there is no helper whose reading I would have to vouch for.

A note on how to read this. Each answer carries a label, a "Not covered" line, the exact words the claim rests on, and the two standing lines (outside the interface / operating system). Because this is a second pass, each answer also carries a **Where I looked** section listing the files searched, the words searched for, and what was ruled out. Where an answer has several parts I have labelled each part separately, because they do not all stand on the same ground.

---

## 2. What makes a `.blend` file need an add-on at the moment it renders, and can an add-on that takes part in producing the render be told apart from one that only rearranges Blender's existing tools while the artist works?

### Settled by hand test — which engines are registered, and whether an add-on's part in a render can be seen

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.1.2. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

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

### Settled by hand test on the rented Linux machine — which engines are registered, and whether an add-on's part in a render can be seen

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.1.2 from `/opt/blender/blender-5.1.2-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

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

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 5.1.2, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`. Fresh settings folders, preferences saved once beforehand so the Quick Setup panel is not in the picture.

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


This question has two halves and they got different answers. I answer them in turn.

### 2a. What makes a `.blend` file need an add-on at the moment it renders

**Label: mixed — parts documented, parts observed, and the direct question is not in the sources.**

The sources never state, in one place or in any place, "a blend-file needs an add-on when it contains X". What they do give is four separate mechanisms by which a file can come to depend on code that an add-on supplies. I list them with what each one actually rests on.

**(i) The render engine named in the file. — documented, plus one inferred step.**

A scene records which engine it will render with:

> "engine
>
> Engine to use for rendering (default 'BLENDER_EEVEE')"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

Engines can be supplied by add-ons:

> "More renderers from third-party developers are available as
> add-ons."

— Manual, https://docs.blender.org/manual/en/5.1/render/introduction.html — read 2026-07-30.

And the manual's directory listing says the same in passing, describing what lives in the add-ons folder:

> "Python add-ons which may be enabled in the Preferences include import/export format support,
> render engine integration and many handy utilities."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — read 2026-07-30.

The list of engines Blender will accept is not fixed in the documentation; the command-line page tells you to ask the program:

> "-E, --engine <engine>
>
> Specify the render engine.
> Use -E help to list available engines."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

*The inferred step:* the sources say the scene stores an engine, and that engines can come from add-ons, and that the list of available engines is discovered at run time rather than fixed. From those three together it follows that a file whose stored engine is one an add-on supplies depends on that add-on at render time. The sources do not say this sentence themselves, and they do not say what happens when the engine is absent, so the label here is **inferred**, not documented.

**(ii) Settings blocks that an add-on attaches to the scene and to objects. — observed, plus one inferred step.**

Cycles is itself an add-on. Its own files declare it as one:

> ```
> bl_info = {
>     "name": "Cycles Render Engine",
>     "author": "",
>     "blender": (2, 80, 0),
>     "description": "Cycles renderer integration",
>     "warning": "",
>     "doc_url": "https://docs.blender.org/manual/en/latest/render/cycles/",
>     "tracker_url": "",
>     "support": 'OFFICIAL',
>     "category": "Render"}
> ```

— Cycles' own files, `cycles_5.1/__init__.py`, lines 7–16 — read 2026-07-30.

It registers a render engine under a fixed identifier:

> ```
> class CyclesRender(bpy.types.RenderEngine):
>     bl_idname = 'CYCLES'
>     bl_label = "Cycles"
> ```

— Cycles' own files, `cycles_5.1/__init__.py`, lines 42–44 — read 2026-07-30.

And when it registers, it hangs its own settings blocks onto Blender's existing data types, removing them again when it unregisters:

> ```
>     @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )
>
>     @classmethod
>     def unregister(cls):
>         del bpy.types.Scene.cycles
> ```

— Cycles' own files, `cycles_5.1/properties.py`, lines 1101–1112 — read 2026-07-30.

The same file does this for eleven further attachment points. The lines are, in `cycles_5.1/properties.py`:

> ```
> 1103:        bpy.types.Scene.cycles = PointerProperty(
> 1119:        bpy.types.Camera.cycles_custom = PointerProperty(
> 1170:        bpy.types.Material.cycles = PointerProperty(
> 1211:        bpy.types.Light.cycles = PointerProperty(
> 1272:        bpy.types.World.cycles = PointerProperty(
> 1325:        bpy.types.World.cycles_visibility = PointerProperty(
> 1341:        bpy.types.Mesh.cycles = PointerProperty(
> 1346:        bpy.types.Curve.cycles = PointerProperty(
> 1351:        bpy.types.MetaBall.cycles = PointerProperty(
> 1439:        bpy.types.Object.cycles = PointerProperty(
> 1468:        bpy.types.Scene.cycles_curves = PointerProperty(
> 1547:        bpy.types.ViewLayer.cycles = PointerProperty(
> ```

— Cycles' own files, `cycles_5.1/properties.py` — read 2026-07-30. (I have quoted the line numbers and the opening of each statement; the description text that follows each one is not needed for the claim.)

*The inferred step:* the file shows these settings exist only while the add-on is registered — they are created in `register` and deleted in `unregister`. A scene saved with values in `scene.cycles` therefore carries data whose home does not exist unless that add-on is enabled. The sources do not say what Blender does with such data when the add-on is absent, so the consequence is **inferred**, and only that far.

The API documentation does state that properties Python adds to Blender's types behave like Blender's own:

> "These properties can be animated, accessed by the user interface and Python
> like Blender's existing properties."

— Python API documentation, https://docs.blender.org/api/5.1/bpy.props.html — read 2026-07-30.

**(iii) Free-form custom properties on data-blocks. — documented.**

> "This data is saved with the blend-file and copied with objects, for example:"

— Python API documentation, https://docs.blender.org/api/5.1/info_quickstart.html — read 2026-07-30. (The sentence is about properties assigned by name onto any data-block that has an ID.)

The same page adds:

> "These properties are valid outside of Python. They can be animated by curves or used in driver paths."

This documents that data an add-on writes in that form travels inside the file. It does **not** document that the add-on is needed to read it back — the wording says the opposite for the values themselves ("valid outside of Python").

**(iv) Python inside the file that is evaluated when the file is used. — documented.**

The manual lists what in a blend-file can run by itself:

> "Registered Text-Blocks
>
> A text data-block can have its Register option enabled which means it will load on start.
>
> Animation Drivers
>
> Python expressions can be used to Drive values and are often used in more advanced rigs and animations."

and, in the same place, under things that need no auto-execution setting to happen:

> "Rendering with Freestyle, because Freestyle uses scripts to control line styles."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html — read 2026-07-30.

Driver expressions can call functions that are not built in, and the API documentation is explicit that those functions must be put in place before the drivers are evaluated, and that doing it from outside the file is unreliable:

> "Since drivers may be evaluated immediately after loading a blend-file it is necessary
> to ensure the driver name-space is initialized beforehand.
>
> This can be done by registering text data-blocks to execute on startup,
> which executes the scripts before drivers are evaluated."

and

> "Using The --python command line argument to populate name-space often fails to achieve the desired goal
> because the initial evaluation will lookup a function that doesn't exist yet,
> marking the driver as invalid - preventing further evaluation.
>
> Populating the driver name-space before the blend-file loads also doesn't work
> since opening a file clears the name-space."

— Python API documentation, https://docs.blender.org/api/5.1/bpy.app.html — read 2026-07-30.

This documents a real dependency of a file on Python code — but note what it says about *where* that code should live: inside the file, as a registered text-block. It is a documented argument against depending on an add-on for this particular thing.

**What is not in the sources for this half.** No page says what a render does when a needed add-on is missing, no page gives a list of what creates such a need, and no page names a file feature as "requires an add-on".

**Not covered.** Whether a file records the add-on it needs; what a render actually does when the add-on is gone; whether missing settings blocks are dropped, kept, or reported; whether a third-party engine identifier survives a save when its add-on is not enabled; and whether anything in EEVEE, Workbench or the compositor creates an add-on dependency (I found nothing saying so).

### 2b. Can a render-participating add-on be told apart from a working-time-only one?

**Label: not in the sources** (for any marker Blender itself records or acts on), with two documented near-misses set out below.

There is no field in an add-on's metadata that says whether it takes part in producing a render. The complete list of what an extension's manifest may declare is given in the manual, and the only field about *kind* is:

> "type:
>
> "add-on", "theme"."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/getting_started.html — read 2026-07-30.

The nearest things to a subject marker are author-chosen labels, not guarantees:

> "tags:
>
> List of tags. See the list of available tags."

— same page, same date. The list of tags an add-on may carry includes both `Render` and `User Interface`, along with `Pipeline`, `Compositing`, `Material`, `Lighting` and others:

— Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/tags.html — read 2026-07-30.

And the Preferences page describes those labels as a description of area, chosen for the add-on:

> "Add-ons are assigned categories by what areas of Blender they affect."

— Manual, https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html — read 2026-07-30.

The manifest's one machine-meaningful declaration of what an add-on will do concerns resources, not rendering:

> "permissions:
>
> Add-ons can list which resources they require. The available options are
> files, network, clipboard, camera, microphone."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/getting_started.html — read 2026-07-30.

The manual goes further and says outright that add-ons have no special standing (*citation check, 2026-08-05: this used to say "the API documentation"; the quote is from the manual, as the source line beneath it already said*):

> "there is nothing inherently different about an add-on that allows it to integrate with Blender,
> such functionality is just provided by the blender_api:bpy module for any script to access."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/scripting/addon_tutorial.html — read 2026-07-30. (The odd `blender_api:` fragment is in the source text as downloaded; it is a link marker that did not render into the plain-text copy.)

**The two things that do mark render participation, once an add-on is running.** Both are visible only after the add-on is enabled, not from the package:

- Registering a render engine. The base class exists for exactly that, and the documented example shows the method Blender calls: "This is the method called by Blender for both final renders (F12) and small preview for materials, world and lights." — Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30.
- Attaching to the render callbacks. Seven of them exist and are documented by name: `render_init`, `render_pre`, `render_post`, `render_write`, `render_complete`, `render_cancel`, `render_stats`. For example: "render_pre — on render (before). Accepts one argument: the scene data-block being rendered" — Python API documentation, https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30.

An observed illustration that this distinction is not recorded anywhere in the package: Cycles, whose whole purpose is producing renders, is marked in its own metadata only by `"category": "Render"` (`cycles_5.1/__init__.py`, line 16), the same kind of free-text label any add-on may choose.

**Not covered.** Whether Blender internally tracks which add-on registered which engine or handler; whether anything reports that at run time; whether the extensions platform enforces tags; and whether an add-on that only changes menus can nonetheless alter a render (nothing in the sources rules it out).

### Where I looked (question 2)

- Read whole: `manual_5.1/render/introduction.txt`, `manual_5.1/advanced/scripting/security.txt`, `manual_5.1/advanced/extensions/tags.txt`, `manual_5.1/advanced/extensions/getting_started.txt`, `manual_5.1/editors/preferences/addons.txt`, `manual_5.1/advanced/blender_directory_layout.txt` (first 200 lines), `api_5.1/bpy.types.Addon.txt`, `api_5.1/bpy.types.Addons.txt`, `api_5.1/bpy.types.AddonPreferences.txt`, `cycles_5.1/__init__.py`, and the registration parts of `cycles_5.1/properties.py`.
- Read in part: `api_5.1/bpy.types.RenderEngine.txt`, `api_5.1/bpy.app.handlers.txt`, `api_5.1/info_overview.txt` (Registration section), `api_5.1/bpy.props.txt`, `api_5.1/info_quickstart.txt` (Custom Properties section), `api_5.1/bpy.app.txt` (driver namespace section), `manual_5.1/animation/drivers/drivers_panel.txt`, `manual_5.1/advanced/scripting/addon_tutorial.txt`.
- Picture opened: `_images/editors_preferences_section_addons.webp`, the screenshot on the Add-ons preferences page. It shows the add-on list with tick-boxes and includes entries named "Cycles Render Engine" and "Hydra Storm render engine" alongside "Node Wrangler" and "Manage UI translations", with no column, badge or marking of any kind separating them.
- Searched the whole manual and the whole API documentation for: `Undefined`, `NodeUndefined`, `custom node`, `not registered`, `unregistered`, `missing add-on`, `add-on is missing`, `add-on must be enabled`, `requires the add-on`, `if the add-on`, `add-on is not enabled`, `add-on is disabled`, `without the add-on`, `enabled add-on`, `RenderEngine`, `external render engine`, `third-party render`, `at render time`, `needed to render`, `required add-on`, `depends on`, `dependency`, `dependencies`, `declares`, `capabilit`.
- Ruled out: `manual_5.1/compositing/` in its entirety contains no occurrence of "add-on" at all; `manual_5.1/render/` contains exactly one, the sentence quoted above. `manual_5.1/files/` mentions add-ons only in the asset-library and import/export pages, neither about rendering. Searching for `Undefined` across the whole manual returns nothing, so there is no documented "unknown node type" placeholder to lean on. The 5.1 release notes contain no entry about add-on dependency or about what a render does without an add-on (I searched all 21 note files for `add-on`, `addon`, `extension`, `manifest`, `wheel`, `platform`).

### Outside the interface

For 2a, everything cited holds without the interface: the engine identifier is stored in the file and can be overridden with `-E` on the command line; the Cycles registrations happen in `register()`, which runs whenever the add-on is enabled, with or without a window; and drivers are documented as evaluating on file load, which happens in background mode too. One difference is documented for drivers and startup scripts: automatic execution is off by default and must be turned on explicitly when driving Blender from outside — "To render an animation in background mode, allowing drivers and other scripts to run: `blender --background --enable-autoexec my_movie.blend --render-anim`" (Manual, https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html, read 2026-07-30). For 2b, the absence of a marker is an absence in the package metadata, so it is the same from outside as from inside.

### Operating system

Nothing in either half is stated to differ by operating system. The manifest's `type`, `tags` and `permissions` fields, the engine identifier, and the render callbacks are all described without any operating-system qualification. One 5.1 change touches two of the permission names on one system only: "On macOS, scripts and extensions can now access the device camera, microphone and system audio. A system permission dialog will be prompted to the user." — Release notes, https://developer.blender.org/docs/release_notes/5.1/python_api/ — read 2026-07-30. That concerns `camera` and `microphone`, not rendering.

---

## 3. Can an add-on be installed, switched on, and given whatever else it needs before it will work, on a computer with no screen attached?

### Settled by hand test — installing, switching on, and setting up an add-on with no window

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.1.2. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

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

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.1.2 from `/opt/blender/blender-5.1.2-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

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

**Label: observed.** Run on **2026-08-06** on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, reached over SSH** — with Blender at `/opt/blender/blender-5.1.2-linux-x64/blender`, `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`. Every run pointed Blender at a fresh settings folder of its own through `BLENDER_USER_RESOURCES`, exported in the shell that ran Blender. **Nothing else was running on the machine.** This is a third machine; nothing here is merged with the Creator's Mac or with the one-card rented Linux machine. The full list is `Audits/Blender Research/Add-ons/Hand tests — two cards and no screen.md`.

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
[Errno 2] No such file or directory: '/root/smoozy/no screen/res-5.1/extensions/blender_org'
Repository data: /root/smoozy/no screen/res-5.1/extensions/blender_org not found, sync required!
```

**An add-on's own settings can be read, changed and made to stick, with no display server.** Cycles' key was found by asking rather than guessing:

```
KEYS ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
PROPS ['rna_type', 'bl_idname', 'compute_device_type', 'devices', 'peer_memory', 'metalrt', 'use_hiprt', 'use_oneapirt', 'kernel_optimization_level']
DEVICE NONE
BEFORE True
SET False
SAVE {'FINISHED'}
```

and a separate later run printed `AFTER False`. The property list is the same as on the other two machines. The setting used here is `use_hiprt`, a plain yes-or-no one, set to the opposite of what it held.

**Fetching from the internet is refused unless it is allowed**, in the same words as on the other machines:

```
RuntimeError: Operator bpy.ops.extensions.repo_sync_all.poll() Online access required to check for updates. Enable online access in System preferences
Error synchronizing
```

**`--addons` printed the add-on's own `SMOOZY TEST ADDON REGISTERED` line, and the module list held the same nine names.** *(Two-card citation check, 2026-08-06: this was headed "`--addons` loads the add-on for that run only", which its own record does not carry, and which **contradicts the one-card block earlier in this same document**. There, under the same heading, `--addons` printed the register line and "the module list held only the eight built-in ones" — the limit that block states in as many words. Here the list holds nine. The two results are recorded side by side and neither is chosen. What the record does show is that by this point the settings folder had already had the add-on installed and its preferences saved with it in the list, so nine names is what an ordinary run would print with no `--addons` at all, and nothing here separates the two. Nor was any later run made without `--addons` to show the add-on had gone. The claim has been reduced to what was printed, and the question sent back as a test.)*

**Observed on the four-card machine — 2026-08-06. Eight built-in add-ons or nine, and what decides the number.**

Run over SSH on a **third rented Linux machine — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, 197 GB of memory, no display server** — using Blender at `/opt/blender/blender-5.1.2-linux-x64/blender`, `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/four cards/addons-5.1`, a folder deleted and remade at the start of the run. Label: **observed**. This is a **fourth** machine; nothing here is merged with the Creator's Mac, the one-card rented Linux machine or the two-card rented Linux machine. The full list, with every command written out, is `Audits/Blender Research/Add-ons/Hand tests — four cards.md`.

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
00:00.058  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.058  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4645, total unfreed memory 1.212318 MB
```

on **standard output**, and exited **1**. Nothing was written to standard error. That is the same message and the same exit status the one-card machine gave when cut off from its display, plus the memory-leak line that machine never showed.

**What this settles.** **The whole add-on cycle — build, install, list, switch on, switch off, refresh, set an add-on's own setting, save it and read it back — works on a computer with no display server at all**, in this version, as long as Blender is not asked to open a window. Asked to open one it cannot, it says so and stops. That answers item B of the Linux list **for a computer with no display server running**, on Linux.

*(Two-card citation check, 2026-08-06: the last sentence read "That is item B of the Linux list answered for Linux." Item B is worded "a computer with no screen attached at all", and that is not what was observed. What the evidence above shows is that **no display server was running** — `DISPLAY` empty, no socket, no X or Wayland process, session type `tty`. Whether a monitor is physically plugged into either card was never looked at and cannot be from here. The two are not the same claim, and the difference is not a quibble in one direction: a machine can have a display server and no monitor, in which case Blender's window opens perfectly well. **The condition that decides what Blender does is the one that was tested** — with no display server there is nothing for a window to open on, whether or not a screen is attached — so the practical half of the question is settled and the words have been corrected to the tested ones.)*

**What it does not settle.** A computer with **no graphics hardware at all**: this machine has two graphics cards, it simply has no display server on top of them. And the manual's line about headless *Windows* systems, which needs a Windows computer.

**Outside the interface.** Every run in this block was made from outside Blender's window, and on this machine there is no other way: with no display server there is no window to be inside. So this block speaks for the outside case alone, and none of the window tests the one-card machine ran could be repeated here. *(Two-card citation check, 2026-08-06: added by this check — the block carried neither standing line of its own, where the two blocks above it each carry both.)*

**Operating system.** Linux, on the machine named at the top of this block. It says nothing about macOS, where the project's only machine has a display server, and nothing about Windows, which no Windows machine has run. *(Two-card citation check, 2026-08-06: added by this check.)*


**Label: documented, for all three steps.** The sources give a command-line route for each, and state that the command-line route runs without a graphical display.

**No display is needed.** The manual says so directly, about running Blender from the command line:

> "One advantage of using the command line is that we do not need a graphical display
> (no need for X server on Linux for example)
> and consequently we can render via a remote shell (typically SSH)."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read 2026-07-30.

And the mode itself:

> "-b, --background
>
> Run in background (often used for UI-less rendering)."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

**Installing.** There is a dedicated command set for managing extensions, and using it forces background mode:

> "-c, --command <command>
>
> Run a command which consumes all remaining arguments.
> Use -c help to list all other commands.
> Pass --help after the command to see its help text.
>
> This implies --background mode."

— same page, same date.

The commands themselves cover fetching from a repository and installing from a file on disk:

> "install:
>
> Install packages.
>
> install-file:
>
> Install package from file."

and, in full form:

> "blender --command extension install [-h] [-s] [-e] [--no-prefs]
> PACKAGES"

> "blender --command extension install-file [-h] -r REPO [-e] [--no-prefs]
> FILE"

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/extension_arguments.html — read 2026-07-30.

**Switching on.** Both install commands take a switch that enables in the same step:

> "-e, --enable
>
> Enable the extension after installation."

— same page, same date.

Enabling can also be done from Python, without any window:

> "bpy.ops.preferences.addon_enable(*, module='')
>
> Turn on this add-on
>
> Parameters:
>
> module (str) – Module, Module name of the add-on to enable (optional, never None)"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.ops.preferences.html — read 2026-07-30. The same page carries `addon_install` ("Install an add-on"), with a parameter "enable_on_install (bool) – Enable on Install, Enable after installing (optional)", and `addon_disable` ("Turn off this add-on").

There is a third route documented for exactly this situation — a whole studio's worth of machines, no person clicking:

> "For example, a script can enable add-ons for every user.
>
> $BLENDER_SYSTEM_SCRIPTS/startup/enable_addons.py
>
> ```
> def register():
>     import addon_utils
>     addon_utils.enable("my-addon")
> ```"

— Manual, https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30.

And a fourth, per-run rather than persistent:

> "--addons <addon(s)>
>
> Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

**Making the choice stick.** Enabling is a preferences change, and the manual says preferences are saved on exit by default:

> "Auto-Save Preferences
>
> By default changes to preferences are saved on exit,
> this allows changes to the keymap and Quick Favorites menu to be stored and used between
> Blender Sessions."

— Manual, https://docs.blender.org/manual/en/5.1/editors/preferences/introduction.html — read 2026-07-30.

It can also be done deliberately, from Python:

> "bpy.ops.wm.save_userpref()
>
> Make the current preferences default"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.ops.wm.html — read 2026-07-30.

The extension commands take an explicit switch for *not* writing preferences, which is the same fact seen from the other side:

> "--no-prefs
>
> Treat the user-preferences as read-only,
> preventing updates for operations that would otherwise modify them.
> This means removing extensions or repositories for example, wont update the user-preferences."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/extension_arguments.html — read 2026-07-30. (The missing apostrophe in "wont" is in the source.)

**"Whatever else it needs".** Four kinds of further setup are documented, all reachable without a screen:

1. *Where it may be fetched from.* Repositories can be added by command, with all their settings as arguments, including an access token: `blender --command extension repo-add [-h] [--name NAME] [--directory DIRECTORY] [--url URL] [--access-token ACCESS_TOKEN] [--source SOURCE] [--cache BOOLEAN] [--clear-all] [--no-prefs] ID` — same page, same date.
2. *The add-on's own settings.* Add-ons may carry their own preferences, reached by name from Python: "Add-ons can define their own preferences which use the package name as an identifier. This can be accessed using `__package__`." and "`addon_prefs = bpy.context.preferences.addons[__package__]`" — Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/addons.html — read 2026-07-30.
3. *Internet permission.* This is a preference, and it can be forced either way on the command line: "--online-mode — Allow internet access, overriding the preference." and "--offline-mode — Disallow internet access, overriding the preference." — Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. It matters here because the Get Extensions panel states plainly: "Internet access is required to install and update online extensions." and "While offline, use "Install from Disk" instead." — Manual, picture `_images/editors_preferences_section_extensions.webp` on https://docs.blender.org/manual/en/5.1/editors/preferences/extensions.html — read 2026-07-30, label **documented** (it is part of the manual).
4. *A writable place of its own.* "Add-ons which need their own user directory should use a utility function provided for this purpose: `extension_directory = bpy.utils.extension_path_user(__package__, path="", create=True)`" — Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/addons.html — read 2026-07-30.

**One documented difference in background mode, and it is about add-ons.** Keyboard shortcuts cannot be set up:

> "# Note that in background mode (no GUI available), keyconfigs are not available either,
> # so we have to check this to avoid nasty errors in background case.
> kc = wm.keyconfigs.addon
> if kc:"

— Manual, https://docs.blender.org/manual/en/5.1/advanced/scripting/addon_tutorial.html — read 2026-07-30. The API documentation carries the same warning in its keymap page: "so skipping keymap registration in background mode doesn't interfere with other registration logic" and "return # Can be None in background mode." — https://docs.blender.org/api/5.1/bpy.types.KeyMaps.html — read 2026-07-30.

**Not covered.** Whether every one of these commands succeeds on a machine with no display attached at all (the sources say a graphical display is not *needed*, which is not the same as saying every path is exercised without one); whether `bpy.ops.extensions.package_install` works without a window, as opposed to the `--command extension` route which is documented to imply background mode; what an install failure looks like or how it is reported; whether a system permission dialog can be answered without a screen on macOS (see the operating-system line below); and whether wheels install correctly in this mode.

### Where I looked (question 3)

- Read whole: `manual_5.1/advanced/command_line/extension_arguments.txt`, `manual_5.1/advanced/command_line/render.txt`, `manual_5.1/advanced/deploying_blender.txt`, `manual_5.1/editors/preferences/introduction.txt`, `manual_5.1/editors/preferences/extensions.txt`, `manual_5.1/editors/preferences/addons.txt`, `manual_5.1/advanced/extensions/addons.txt`, `manual_5.1/advanced/scripting/security.txt`, `manual_5.1/troubleshooting/network.txt`.
- Read in part: `manual_5.1/advanced/command_line/arguments.txt` (lines 1–120, 185–305, 480–640), `api_5.1/bpy.ops.preferences.txt`, `api_5.1/bpy.ops.extensions.txt`, `api_5.1/bpy.ops.wm.txt` (search), `api_5.1/bpy.utils.txt` (`extension_path_user`, `load_scripts`, `script_paths`), `api_5.1/bpy.types.PreferencesExtensions.txt`, `api_5.1/bpy.types.UserExtensionRepo.txt`, `api_5.1/bpy.app.txt` (`online_access`), `api_5.1/info_advanced_blender_as_bpy.txt`, `manual_5.1/editors/preferences/system.txt` (Network section).
- Pictures opened: `_images/editors_preferences_section_extensions.webp` (the Get Extensions panel, shown in its offline state with "Continue Offline" and "Allow Online Access" buttons) and `_images/advanced_deploying-blender_system-extensions.png` (the repository list, showing three entries — `extensions.blender.org`, `System`, `User Default` — each with a tick-box, plus `+`, `-`, a refresh control, and an "Advanced" section).
- Searched the whole manual and API documentation for: `headless`, `X server`, `Xorg`, `display server`, `without a display`, `no display`, `EGL`, `background mode`, `--background`, `in background`, `addon_utils`, `--addons`, `save_userpref`, `Auto-Save Preferences`, `factory-startup`, `online_access`, `offline`, `access token`.
- Ruled out: `headless` appears only twice in the whole manual, both on the EEVEE limitations page and both about rendering rather than installing (see the operating-system line). There is no page about installing add-ons without a display; the answer is assembled from the command-line pages and the deployment page, which is why each step is cited separately.

### Outside the interface

This entire answer is about being outside the interface, so the interesting direction is the reverse: what differs when Blender is *not* a person's window. Three documented differences. First, keyboard shortcuts cannot be registered (quoted above). Second, if Blender is being driven as a Python module rather than as a program, the preferences — which is where enabled add-ons are recorded — are not loaded at all: "The user's startup and preferences are ignored to prevent your local configuration from impacting script behavior. The Python module behaves as if `--factory-startup` was passed as a command line argument." and "The user's preferences and startup can be loaded using operators: `bpy.ops.wm.read_userpref()`" — Python API documentation, https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html — read 2026-07-30. Third, on that same route, "Functionality controlled by command line arguments (shown by calling `blender --help`) isn't accessible" — same page — which removes `--addons`, `--online-mode` and the whole `--command extension` route.

### Operating system

The install and enable routes are documented without any operating-system qualification. Two operating-system-specific facts touch the edges of this question and I record them as found. The manual states that a graphical display is not needed and names Linux in its example ("no need for X server on Linux for example"). The EEVEE limitations page states: "Headless rendering is not supported on headless Windows systems." — Manual, https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html — read 2026-07-30; that is about rendering, not about installing or enabling an add-on. And the 5.1 release notes state that on macOS an add-on reaching for camera, microphone or system audio triggers a dialog: "A system permission dialog will be prompted to the user." — https://developer.blender.org/docs/release_notes/5.1/python_api/ — read 2026-07-30. Whether that dialog can be dealt with on a machine with no screen is not stated.

---

## 4. What does an add-on record about the kinds of computer it is meant for, what happens on one it does not name, and does an add-on set up on one computer still work when copied to a computer of a different kind?

### Settled by hand test — what happens on a kind of computer an add-on does not name, and which bundled modules land on disk

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.1.2. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

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

### Settled by hand test on the rented Linux machine — what happens on a kind of computer an add-on does not name, which bundled modules land on disk, and what a compiled part for the wrong machine does

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.1.2 from `/opt/blender/blender-5.1.2-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

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

**The new part: carrying an installed, set-up add-on to a computer of a different kind.** The whole settings folder holding the installed three-module add-on was packed on this Linux machine, copied to the Creator's MacBook Pro (M1 Pro chip, macOS 26.5.2), and that Mac's Blender 5.1.2 was pointed at it. The Mac listed it as installed:

```
Repository: "User Default" (id=user_default)
  smoozy_wheels [installed]: "Smoozy Wheels", A test add-on carrying bundled Python modules
```

and had it switched on:

```
VERSION 5.1.2
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

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 5.1.2, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`. Fresh settings folders, preferences saved once beforehand.

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


Three parts. The first is well documented, the second only partly, the third is documented for one specific case and absent for the general case.

### 4a. What it records

**Label: documented.**

An extension's manifest may name the systems it is for. The field is optional:

> "platforms:
>
> List of supported platforms. If omitted, the extension will be available in all operating systems.
> The available options are
> ["windows-x64", "windows-arm64", "macos-x64", "macos-arm64", "linux-x64"]"

— Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/getting_started.html — read 2026-07-30. The manifest example on the same page says the same thing in its comments: "# # Optional: list of supported platforms. If omitted, the extension will be available in all operating systems." and "# platforms = ["windows-x64", "macos-arm64", "linux-x64"]" and "# # Other supported platforms: "windows-arm64", "macos-x64"".

So the recorded granularity is operating system *and* processor kind together — five named combinations, no more.

Alongside that, the manifest records the Blender versions it is for, which is a different axis but is recorded in the same place:

> "blender_version_min:
>
> Minimum supported Blender version - use at least 4.2.0."

> "blender_version_max:
>
> Blender version that the extension does not support, earlier versions are supported."

— same page, same date.

An add-on's bundled Python modules are recorded per platform separately, by their own file names:

> "For wheels that contain binary compiled files, wheels for all supported platforms should be included:"

and the example lists three wheels, one each for macOS-arm64, manylinux-x86_64 and Windows-amd64:

> ```
> wheels = [
> "./wheels/pillow-12.1.0-cp313-cp313-macosx_11_0_arm64.whl",
> "./wheels/pillow-12.1.0-cp313-cp313-manylinux_2_27_x86_64.manylinux_2_28_x86_64.whl",
> "./wheels/pillow-12.1.0-cp313-cp313-win_amd64.whl",
> ]
> ```

— Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/python_wheels.html — read 2026-07-30.

**An add-on need not record any of this. — observed.** The older style of add-on, which Blender still supports, has no platform field at all. Cycles, shipped with Blender, is one of these: its whole metadata block is the ten lines quoted under question 2a, and it contains no platform entry. There is no `blender_manifest.toml` anywhere in `cycles_5.1/`; the folder holds `__init__.py`, `camera.py`, `engine.py`, `license/`, `operators.py`, `osl.py`, `presets.py`, `properties.py`, `shader/`, `source/`, `ui.py`, `version_update.py` and nothing else. — Cycles' own files, `cycles_5.1/` — read 2026-07-30.

That the older style is still supported is documented:

> "In order to allow a smooth transition process, the so-called legacy add-ons will continue to be supported by Blender.
> They can be installed via Install legacy Add-on button in the User
> Preferences."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/addons.html — read 2026-07-30.

And what the older metadata block does hold is described as: "a dictionary containing add-on metadata such as the title, version and author to be displayed in the Preferences add-on list. It also specifies the minimum Blender version required to run the script; older versions won't display the add-on in the list." — Manual, https://docs.blender.org/manual/en/5.1/advanced/scripting/addon_tutorial.html — read 2026-07-30. Blender version, yes. Machine kind, no.

### 4b. What happens on a computer it does not name

**Label: documented for one step of the journey; inferred for the rest; not in the sources for what the artist actually sees.**

What is documented is that the machine's kind is used to decide what a repository offers:

> "When Blender fetches the extensions listing it passes the following arguments to make sure only
> compatible extensions are listed:
>
> platform
>
> blender_version"

> "query URL:
>
> https://extensions.blender.org/api/v1/extensions/?blender_version=4.2.0&platform=linux-x64"

— Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/creating_repository/dynamic_repository.html — read 2026-07-30.

The same field is carried on download links:

> "platforms:
>
> Comma-separated list of supported platforms.
> If omitted, the extension will be available in all operating systems."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/creating_repository/static_repository.html — read 2026-07-30.

*The inferred step:* both pages say that omitting the field makes the extension available on all operating systems. Read the other way, naming the field restricts availability to the named ones — that is what "only compatible extensions are listed" is doing. So on a machine whose kind is not named, the add-on is not offered. **The step a reader has to supply is the negative**, which neither page states, so this is **inferred**, not documented. And the inference only reaches the listing. Nothing says what happens if the package is installed from disk anyway, whether a warning is shown, whether it is refused, or whether it installs and then fails.

A related documented fact about the build side: when an add-on is built once per platform, only the named platforms get a package, and a wheel for an unnamed one is simply dropped:

> "Even though there is a Linux-only wheel present, no Linux zip file is generated.
> This happens because the platforms field only has Mac and Windows."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/python_wheels.html — read 2026-07-30.

What the artist would see on the unnamed machine is **not in the sources**. I searched for it; see the record below.

### 4c. Does it still work when copied to a computer of a different kind?

**Label: documented, for add-ons that carry compiled parts — and the answer there is no.**

> "While not common practice, Python add-ons can be distributed with their own precompiled libraries. Unlike regular
> Python scripts, these are not portable between different platforms.
>
> It is possible the library is incompatible with your Blender installation (attempting to load a library built for a
> different version of Python, or loading a 32-bit library on a 64-bit system).
>
> If the add-on contains .pyd or .so files, check that the distribution is compatible with your operating
> system."

— Manual, https://docs.blender.org/manual/en/5.1/troubleshooting/python.html — read 2026-07-30.

Two things follow, and the source says both parts itself: plain Python add-ons are portable ("Unlike regular Python scripts"), and add-ons with compiled parts are not. Since the recommended way to bundle dependencies is wheels, and wheels are the thing that carries compiled parts, this is not an edge case:

> "Bundle with Python Wheels.
>
> This is the recommended way to bundle dependencies."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/addons.html — read 2026-07-30.

**A second obstacle to copying, this one about where things live. — documented.** The place an installed add-on sits is different on each operating system and carries the Blender version in its path:

> "Linux
>
> $HOME/.config/blender/5.1/"
>
> "macOS
>
> /Users/$USER/Library/Application Support/Blender/5.1/"
>
> "Windows
>
> %USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.1\"

and, of what is stored there:

> "User directories store preferences, startup file, installed extensions,
> presets and more."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — read 2026-07-30.

So a copy is not a matter of moving one folder to the same path; the path itself differs by system.

**A third fact, observed, about add-ons behaving differently by machine kind even when they run everywhere.** Cycles — one add-on, no platform declaration — chooses different defaults by inspecting the machine at run time:

> ```
>     @staticmethod
>     def default_device():
>         import platform
>         # Default to selecting the Metal compute device on Apple Silicon GPUs
>         # (drivers are tightly integrated with macOS so pose no stability risk)
>         if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):
>             return 5
>         return 0
> ```

— Cycles' own files, `cycles_5.1/properties.py`, lines 1572–1579 — read 2026-07-30. The same file branches on `sys.platform` at lines 1825, 1836, 1850 and 1855, and shows a macOS-only message at line 1869: `col.label(text=rpt_("Requires Apple Silicon with macOS %s or newer") % mac_version,`.

*The inferred step for this one:* the file shows the code reads the machine and branches. That an add-on's *behaviour* therefore differs between machines even when the same files are copied across is one step beyond what the file says, so it is **inferred**.

**Not covered.** Whether an already-installed add-on's folder can be copied between machines of the *same* kind and simply work (the sources describe installing, not copying an install); what Blender does with a copied folder that has no matching entry in the destination's preferences; whether the preferences file itself is portable; whether an add-on's own stored settings travel with it; what happens to already-extracted wheels when the folder is copied; and what an artist sees when an add-on is present but wrong for the machine.

### Where I looked (question 4)

- Read whole: `manual_5.1/advanced/extensions/getting_started.txt`, `manual_5.1/advanced/extensions/python_wheels.txt`, `manual_5.1/advanced/extensions/creating_repository/static_repository.txt`, `manual_5.1/advanced/extensions/creating_repository/dynamic_repository.txt`, `manual_5.1/advanced/extensions/creating_repository/index.txt`, `manual_5.1/troubleshooting/python.txt`, `manual_5.1/advanced/extensions/addons.txt`, `manual_5.1/advanced/blender_directory_layout.txt` (lines 1–200).
- Read in part: `cycles_5.1/properties.py` (the preferences class and its platform branches), `cycles_5.1/__init__.py` (whole), the file listing of `cycles_5.1/`, `manual_5.1/advanced/scripting/addon_tutorial.txt` (metadata section), `Blender Sources/requirements/requirements_2026-08-01.txt`.
- Searched the whole manual, API documentation and release notes for: `platforms`, `platform`, `windows-x64`, `macos-arm64`, `linux-x64`, `arm64`, `incompatible`, `not compatible`, `unsupported platform`, `not available`, `filtered`, `hidden`, `will not be shown`, `not shown`, `greyed`, `grayed`, `portable`, `.pyd`, `.so`, `wheel`, `site-packages`, `build_platform`.
- Ruled out: `bpy.app.build_platform` ("The platform this blender instance was built for", https://docs.blender.org/api/5.1/bpy.app.html) describes Blender, not an add-on, so it does not answer what an *add-on* records. `gpu.platform` is about graphics hardware, not operating systems. The API documentation has no page for the manifest at all — the manifest is documented only in the manual. Searching the preferences, extensions and troubleshooting sections for words describing a rejected or hidden add-on (`incompatible`, `not compatible`, `unsupported platform`, `not shown`, `greyed`, `grayed`, `filtered`) returned nothing about platforms, which is the basis for calling 4b's visible outcome absent.
- One source outside the per-version sets, recorded because I used it and it names versions in so many words. Blender's requirements page states: "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." — `Blender Sources/requirements/requirements_2026-08-01.txt` — read 2026-08-01 (that page's own snapshot date). It names "5.0 and later", which covers 5.1 by its own words but not by naming 5.1. I raise it only because it sits oddly beside the manifest's `macos-x64` option, and I flag that as an open question rather than resolving it: the manual lists `macos-x64` as an available platform value, and the requirements page says 5.0 and later require Apple Silicon. Nothing in the 5.1 manual, API documentation or release notes reconciles the two. This is listed below as something a person could check by hand.

### Outside the interface

The platform field is part of the package and is used when a repository listing is fetched, which the command-line `extension sync` / `extension install` route does as well — "Synchronize with remote repositories." and "-s, --sync — Sync the remote directory before performing the action." (Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/extension_arguments.html, read 2026-07-30). So the same filtering applies. The portability facts about compiled parts are properties of the files, not of the interface. What the sources do not say is whether the command-line install refuses a platform-mismatched package the same way, or at all — the pages that describe the filtering describe a *listing* being fetched, not an install being refused.

### Operating system

This question is itself about operating systems, and the answer differs by one: the user directory paths differ per system (quoted above), and the five platform values pair each operating system with a processor kind, so "kind of computer" here means both. The compiled-library warning names Windows specifically for one failure mode — "If Python is raising errors or you have an add-on that just fails when enabled with an error – e.g: `... is not a valid Win32 application.`" (Manual, https://docs.blender.org/manual/en/5.1/troubleshooting/python.html, read 2026-07-30) — but the portability statement above it is general.

---

## 8. Where do add-ons come from besides Blender's own, how are they sold, and what does that mean for taking a copy of one from one computer to another?

### Settled by hand test — where packages come from, and what a copy installed from a file can do

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.1.2. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

**What is set up to begin with.** `blender --command extension repo-list` in a fresh settings folder printed three entries — `extensions.blender.org` with the address `https://extensions.blender.org/api/v1/extensions/` and no access token, `User Default` marked `source: "USER"`, and `System` marked `source: "SYSTEM"` and pointing inside the application at `/Applications/Blender 5.1.2.app/Contents/Resources/5.1/extensions/system`.

**Anyone can stand up a repository from a plain folder.** `blender --command extension server-generate --repo-dir=<folder>` printed `found 2 packages.` and wrote an `index.json` carrying each package's whole manifest, its size and a hash — for example:

```
      "id": "smoozy_win_only",
      "name": "Smoozy Windows Only",
      "license": [
        "SPDX:GPL-3.0-or-later"
      ],
      "blender_version_min": "4.2.0",
      "platforms": [
        "windows-x64"
      ],
      "archive_url": "./smoozy_win_only-1.0.0.zip",
      "archive_size": 498,
      "archive_hash": "sha256:f15d404cc12d97469a0feee6fcaf72774cbe425cd7fd8b4972b553b67e9df96d"
```

That folder was added as a repository with `blender --command extension repo-add --name "Smoozy Local" --url "file://<folder>/index.json" smoozy_local`, which printed nothing and exited 0, and after a sync its packages could be installed by name.

**A copy installed from a file gets no updates, and says so by saying nothing.** With a package installed from a file, `blender --command extension update` printed **nothing at all** and exited 1. With `--online-mode` it printed only:

```
Repository data: <settings folder>/extensions/blender_org not found, sync required!
```

and exited 0. Nothing was offered for the package installed from a file.

**How add-ons are sold could not be tested.** Nothing on this computer can answer it; it is a fact about the world outside Blender, and no add-on that is sold was to hand.

**Outside the interface.** Settled: hosting from a folder, adding a repository, installing from a file and asking for updates all work with Blender driven from outside its window.

**Operating system.** Settled for macOS only, on the one machine described above. The same tests were run again on a rented Linux machine, and that result has its own block under this question; the two are never merged. Windows stays untested.

### Settled by hand test on the rented Linux machine — where packages come from, and what a copy installed from a file can do

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.1.2 from `/opt/blender/blender-5.1.2-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

**The repositories a fresh settings folder holds** are the same three as on macOS — `extensions.blender.org` at `https://extensions.blender.org/api/v1/extensions/` with no access token, `User Default` in the settings folder, and `System` inside the Blender folder. Full output under question 5.

**A listing generated from a plain folder carries each package's whole manifest, its size and a hash.** `blender --command extension server-generate --repo-dir=<packages folder>` printed `found 2 packages.` and wrote an `index.json` beginning:

```
{
  "version": "v1",
  "blocklist": [],
  "data": [
    {
      "schema_version": "1.0.0",
      "id": "smoozy_test",
      "name": "Smoozy Test",
      "tagline": "A tiny test add-on",
      "version": "1.0.0",
      "type": "add-on",
      "maintainer": "Smoozy Test",
      "license": [
        "SPDX:GPL-3.0-or-later"
      ],
      "blender_version_min": "4.2.0",
      "archive_url": "./smoozy_test-1.0.0.zip",
      "archive_size": 459,
      "archive_hash": "sha256:c1f75f4578b4d704ec04e7df4d403771684cefdc48647b548943ac9f06d4ec55"
    },
```

The second entry is the same shape with `"platforms": ["windows-x64"]` added. Note the empty `"blocklist"` at the top — the mechanism the manual describes is there in the generated file in this version, and nothing was put in it.

That folder was added as a repository by `file://` address, synced and installed from, all from the command line. A package installed from a file never got an update offered: `blender --command extension update` printed nothing and exited 1, and with `--online-mode` printed only the "sync required" line and exited 0.

**What copying a set-up add-on to a computer of a different kind actually does** was run for the first time, across the project's two machines. The whole settings folder holding an installed add-on with a bundled compiled module was packed on this Linux machine and opened by the Creator's MacBook Pro. The Mac listed it as installed and had it switched on, `import PIL` worked, and using it failed with `ImportError: cannot import name '_imaging' from 'PIL'`. The full record is under question 4. So the technical limit the manual states is now observed: the install and the plain Python travel, the compiled part does not, and nothing announces it until the add-on tries to use it.

**On how add-ons are sold, this machine says nothing**, as no computer can. Nothing was attempted.

**Outside the interface.** Settled again on a second operating system: generating a listing, adding a repository, syncing, installing and updating all run with Blender driven from outside its own window.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described, and the copying half was run across both. Windows stays untested. Nothing here bears on how add-ons are sold, which no test on any computer can settle.

### Settled by hand test on the rented Linux machine — what the window shows about where an add-on came from

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 5.1.2, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`.

**An add-on's opened row shows no licence and no repository name.** The two rows opened — Cycles Render Engine and Node Wrangler — carry a website, a type reading `Core`, and for one of them a maintainer and a version. Neither carries a licence, a repository, a tag or a declared permission. So a person cannot tell from a row what they are allowed to do with a copy. Both add-ons ship with Blender, so what a row shows for an add-on installed from a repository is still unsettled here, exactly as it was on the Mac.

**The only setting on the repositories side bearing on getting at an add-on is an access token.** Read from the preferences rather than from the drop-down, a repository carries `access_token` and `use_access_token` among its settings, and all three repositories in a fresh settings folder have `use_access_token` false. Full list under question 5.

**Outside the interface.** This is a window test. Nothing here changes what is reachable from outside the window.

**Operating system.** Settled for Linux in this version, at the window. The Creator's macOS run covered Blender 5.2 only. Nothing here bears on how add-ons are sold.


Three parts again, and they land very differently: the first is well documented, the second is almost entirely absent, the third is documented mechanically and only partly on the question of permission.

### 8a. Where they come from

**Label: documented.**

There is one official place:

> "The official extensions platform for the Blender project is extensions.blender.org.
> Other third party sites can also be supported, as long as they follow the Extensions Platform specification."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/index.html — read 2026-07-30.

Other places are provided for, in three grades of effort:

> "Third party sites that wish to support extensions in Blender can do so in several ways, in order of complexity:
>
> Static:
> Host a static JSON file listing all the packages of your repository.
>
> Dynamic:
> Serve the JSON file on-demand based on the Blender version and platform.
>
> Platform:
> Fork the entire Extensions Website to create your own."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/creating_repository/index.html — read 2026-07-30.

Blender is set up to reach more than one of them:

> "By default Blender has a Remote Repository pointing towards the
> Official Blender Extensions Platform and two Local Repositories.
>
> In the cases where more repositories are needed (e.g., to access third party extension platforms),
> new repositories can be added."

— Manual, https://docs.blender.org/manual/en/5.1/editors/preferences/extensions.html — read 2026-07-30. The picture on the deployment page confirms the three by name — a globe-marked `extensions.blender.org`, `System`, and `User Default`, each with a tick-box — Manual, picture `_images/advanced_deploying-blender_system-extensions.png` on https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30, label **documented**.

A repository can also be a plain folder, including one on a network drive, with no site at all:

> "When working offline or in a more controlled environment, it may be useful
> to provide a set of extensions to all users. For this there is a default
> read-only System repository. This repository could be located on a
> read-only network drive or in a system directory."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30.

And a repository's address may be a local path rather than a web address:

> "remote_url
>
> Remote URL to the extension repository, the file-system may be referenced using the file URI scheme: "file://""

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.UserExtensionRepo.html — read 2026-07-30.

Finally, the loosest source of all, stated in the manual without qualification:

> "Blender comes with some preinstalled Add-ons already, ready to be enabled.
> But you can also add your own, or any interesting ones you find on the web."

— Manual, https://docs.blender.org/manual/en/5.1/editors/preferences/addons.html — read 2026-07-30.

### 8b. How they are sold

**Label: not in the sources.** Nothing in the 5.1 manual, the 5.1 API documentation or the 5.1 release notes describes buying, selling, pricing, subscriptions, licence keys, activation, or any commercial arrangement for add-ons. The record of the search is below.

What the sources do say bears on the question sideways, and I give it as what it is rather than as an answer:

**The official platform will not carry anything but free and open source work.**

> "The Blender Extensions Platform
> only supports free and open source extensions compliant with the
> Blender's license:
>
> For add-ons, the required license is
> GNU General Public License v3.0 or later.
>
> For themes, the recommended license is
> GNU General Public License v3.0 or later,
> but any GPL-compatible license
> is supported.
>
> For assets used in add-ons, the required license is
> Public Domain (CC0)."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/licenses.html — read 2026-07-30.

**Every extension must declare a licence, and it is one of the required fields.**

> "license:
>
> List of licenses,
> use SPDX license identifier."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/getting_started.html — read 2026-07-30. The example manifest shows the form: `license = [ "SPDX:GPL-3.0-or-later", ]`.

**"Free" in the manual explicitly does not mean free of charge.** The manual takes pains over this:

> "While this is often true,
> the term "free software" as used by the Free Software Foundation (originators of the GNU Project and creators of the
> GNU General Public License) is intended to mean "free as in freedom" rather than in the sense of "no cost""

— Manual, https://docs.blender.org/manual/en/5.1/getting_started/about/license.html — read 2026-07-30. That page is about Blender itself, not about add-ons; I quote it because it is the only place in the sources that addresses cost at all.

**There is one mechanism in Blender that could gate access to a repository.**

> "access_token
>
> Personal access token, may be required by some repositories (default "", never None)"

> "use_access_token
>
> Repository requires an access token (default False)"

— Python API documentation, https://docs.blender.org/api/5.1/bpy.types.UserExtensionRepo.html — read 2026-07-30. The manual describes the same field: "Access Token — Personal access token, may be required by some repositories." (https://docs.blender.org/manual/en/5.1/editors/preferences/extensions.html) and shows how it reaches the server: "This is passed to the servers via an Authorization header: `curl -i https://extensions.blender.org/api/v1/extensions/ -H "Accept: application/json" -H "Authorization: Bearer abc29832befb92983423abcaef93001"`" (https://docs.blender.org/manual/en/5.1/advanced/extensions/creating_repository/dynamic_repository.html) — both read 2026-07-30. **The sources never connect this to payment.** They say only that some repositories may require authentication. Reading it as a way of selling would be my invention, not theirs.

**And an observed reminder that not every add-on Blender ships is under Blender's own licence.** Cycles, which appears in the Add-ons list as "Cycles Render Engine", carries:

> "The default license for Cycles source code is Apache 2.0."

— Cycles' own files, `cycles_5.1/license/readme.txt`, line 1 — read 2026-07-30. Its source files carry the matching marker: `# SPDX-License-Identifier: Apache-2.0` — `cycles_5.1/__init__.py`, line 3 — read 2026-07-30. The `license/` folder holds `Apache2-license.txt`, `BSD-3-Clause-license.txt`, `MIT-license.txt`, `SPDX-license-identifiers.txt`, `Zlib-license.txt` and `readme.txt`.

### 8c. What that means for taking a copy from one computer to another

Two strands: what the sources say you *can do*, and what they say about whether you *may*.

**What you can do. — documented.**

An add-on can be installed from a file, with no site involved:

> "Install from Disk
>
> Install an extension from a .zip package.
> This is installed to a Local Repository and no updates will be available."

— Manual, https://docs.blender.org/manual/en/5.1/editors/preferences/extensions.html — read 2026-07-30. Note the sentence's own warning: a copy taken this way is cut off from updates.

The same thing without a person:

> "blender --command extension install-file [-h] -r REPO [-e] [--no-prefs]
> FILE
>
> Install a package file into a user repository."

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/extension_arguments.html — read 2026-07-30.

A whole set of add-ons can be laid out for many machines by unpacking them into a folder:

> "Extensions packages should be extracted in this system directory,
> with a resulting path like this:
>
> $BLENDER_SYSTEM_EXTENSIONS/system/my-addon/blender_manifest.toml"

— Manual, https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30. The same page frames why: "For security or other reasons, workstation may not have internet access." and Blender's own deployment is described as file copying — "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

A copy can also ride along with Blender itself:

> "These repositories are intended to bundle extensions
> with Blender, to make it portable."

— Manual, https://docs.blender.org/manual/en/5.1/editors/preferences/extensions.html — read 2026-07-30. And the portable-installation section says what a `portable` folder next to the program then holds: "This folder will then store preferences, startup file, installed extensions and presets." — Manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — read 2026-07-30.

You can even run your own listing so copies keep updating, from a folder rather than a site:

> "To generate a valid JSON file use the server generate
> Blender command-line tool:
>
> blender --command extension server-generate --repo-dir=/path/to/packages"

and the address such a repository would be given: "file:///path/to/packages/index.json on Linux and macOS." — Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/creating_repository/static_repository.html — read 2026-07-30.

**Whether you may. — documented for the official platform only; not in the sources otherwise.**

For anything from extensions.blender.org, the licence page quoted above settles it: add-ons there are GPL v3.0 or later, and the manual states what that licence grants:

> "You have the right to use the program for any purpose."
>
> "You have the right to copy and distribute the program."

— Manual, https://docs.blender.org/manual/en/5.1/getting_started/about/license.html — read 2026-07-30. That page is written about Blender, and says so ("The GPL only applies to the Blender application and not the artwork you create with it"), so applying it to an add-on carrying the same licence is a step the reader makes. I therefore mark the conclusion — *that an add-on from the official platform may be copied from machine to machine* — as **inferred**, resting on two documented facts: that platform requires GPL v3.0-or-later for add-ons, and the manual states that licence carries the right to copy and distribute.

For an add-on from anywhere else, the sources say nothing at all about what its licence permits. The manifest requires a licence to be declared, but the manual does not constrain what it may be outside the official platform, and there is no page about what a third-party licence might allow or forbid.

**A blocking mechanism exists, and it is worth noting here.** A repository's own configuration can name extensions that are not to be served:

> ```
> schema_version = "1.0.0"
>
> [[blocklist]]
> id = "my_example_package"
> reason = "Explanation for why this extension was blocked"
> ```

— Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/extension_arguments.html — read 2026-07-30, in the description of `--repo-config`: "This can be used to defined blocked extensions, for example". The sources do not say what Blender does with a blocked entry, only that a repository listing can carry one.

**Not covered.** How add-ons are sold — nothing found, so nothing said. Whether any named marketplace exists. Whether a paid add-on can be tied to one machine, and by what means. Whether an access token is ever used for payment. What licence a third-party add-on may carry. Whether copying an *installed* add-on folder (rather than its `.zip`) between machines works. Whether the extensions platform's own terms of use say anything further — that document is not in the sources.

### Where I looked (question 8)

- Read whole: `manual_5.1/advanced/extensions/index.txt`, `manual_5.1/advanced/extensions/licenses.txt`, `manual_5.1/advanced/extensions/getting_started.txt`, `manual_5.1/advanced/extensions/creating_repository/index.txt`, `manual_5.1/advanced/extensions/creating_repository/static_repository.txt`, `manual_5.1/advanced/extensions/creating_repository/dynamic_repository.txt`, `manual_5.1/editors/preferences/extensions.txt`, `manual_5.1/editors/preferences/addons.txt`, `manual_5.1/advanced/deploying_blender.txt`, `manual_5.1/getting_started/about/license.txt`, `manual_5.1/copyright.txt`, `manual_5.1/troubleshooting/network.txt`, `manual_5.1/advanced/command_line/extension_arguments.txt`.
- Read in part: `api_5.1/bpy.types.UserExtensionRepo.txt`, `api_5.1/bpy.types.PreferencesExtensions.txt`, `api_5.1/bpy.ops.extensions.txt`, `manual_5.1/advanced/blender_directory_layout.txt`, `cycles_5.1/license/` (all six files, first lines), `cycles_5.1/__init__.py`.
- Picture opened: `_images/advanced_deploying-blender_system-extensions.png` (the repository list, described above).
- Searched the whole manual, API documentation and release notes for: `sell`, `selling`, `sold`, `buy`, `purchase`, `paid`, `price`, `payment`, `subscription`, `marketplace`, `monet`, `commercial`, `for sale`, `license key`, `activation`, `DRM`, `proprietary`, `closed source`, `royalt`, `donat`, `Blender ID`, `third party`, `third-party`, `extensions.blender.org`, `access token`, `blocklist`.
- Ruled out, one by one. `sold`, `payment`, `license key`, `royalt`, `marketplace` (in an add-on sense), `DRM`: no occurrences anywhere. `buy` occurs once, in Blender's own history page, about buying rights from a defunct company. `subscription` occurs once outside the Python messaging module, in the history page, about Blender Cloud. `commercial` occurs in the licence page (contrasting with commercial software), the history page, the "about" page and a keymap description — never about add-ons. `proprietary` occurs once, inside the GPL explanation. `paid` occurs once, in the history page. `Blender ID` occurs four times: uploading to the platform requires one ("Upload the .zip file (this step requires Blender ID)."), plus the chat service, the translation guide, and a file-browser feature of the same name — none about money. The extensions platform's terms, guidelines or pricing are not in the sources at all; the manual points outward to a Developer Handbook I did not and could not follow ("For extension add-on guidelines (requirements for extensions.blender.org), refer to the Developer Handbook."). I did not go to the web.

### Outside the interface

All three parts hold outside the interface. Repositories can be listed, added and removed by command (`repo-list`, `repo-add`, `repo-remove`), packages installed from a remote repository or from a file by command, and a repository listing generated by command — all on the Extensions Command Line Arguments page cited above, and all under `--command`, which "implies --background mode". The licensing facts are properties of the packages, not of the interface. One difference: the manual's account of getting an add-on "from the Website" is interface-only — "Drag the installation URL into Blender." and "Use the drop-down menu in the top right, or drag-and-drop an extension .zip package into Blender." (https://docs.blender.org/manual/en/5.1/editors/preferences/extensions.html) — dragging has no counterpart outside a window, and the command-line `install` / `install-file` subcommands stand in for it.

### Operating system

Nothing in this answer is stated to differ by operating system, with two exceptions of detail. The address form for a folder-based repository differs: "file:///path/to/packages/index.json on Linux and macOS.", "file:///C:/path/to/packages/index.json on Windows.", "file://HOST/share/path/to/packages/index.json for network shares on Windows." — Manual, https://docs.blender.org/manual/en/5.1/advanced/extensions/creating_repository/static_repository.html — read 2026-07-30. And the place a portable copy must be put differs: "Windows: Next to the Blender executable, in the unzipped folder / Linux: Next to the Blender executable, in the unzipped folder / macOS: Inside the application bundle at Blender.app/Contents/Resources" — Manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — read 2026-07-30. The licensing facts carry no operating-system qualification.

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

2026-08-04 — Opus 5 (Claude Code) — Add-ons, Blender 5.1
