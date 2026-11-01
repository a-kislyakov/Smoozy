# Add-ons — Blender 4.5

**Sources consulted, and when.** Blender 4.5 manual (text and pictures), Blender 4.5 Python API documentation, Blender 4.5 release notes, and Cycles' own files as shipped with Blender 4.5 — all read on 2026-07-30 (the snapshot date).

`I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.`

A note on how I searched: the manual and API documentation were searched whole, by word, before any file was opened. The search terms I used are listed under each answer that reports silence.

---

## 1. Can a `.blend` file be read for the add-ons it needs?

### Settled by hand test — the workspace add-on list, and what a file's engine reads as

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 4.5.12. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**A workspace's add-on filter is real, it is saved in the `.blend`, and it can be read with no window.** Blender was asked which of three workspace properties exist, told to switch the filter on and add one add-on to it, and told to save the file:

`blender --background --python-expr "import bpy; w=bpy.data.workspaces[0]; print('NAME', w.name); print('PROPS', [p for p in ('owner_ids','use_filter_by_owner','active_addon') if hasattr(w,p)]); w.use_filter_by_owner=True; print('ENABLE', bpy.ops.wm.owner_enable(owner_id='cycles')); print('OWNERS', [o.name for o in w.owner_ids]); bpy.ops.wm.save_as_mainfile(filepath='ws.blend')"`

It printed, word for word:

```
NAME Animation
PROPS ['owner_ids', 'use_filter_by_owner', 'active_addon']
ENABLE {'FINISHED'}
OWNERS []
```

Exit status 0. Note that the operator did not write into the workspace the script had in hand. The saved file was then opened again, with no window:

`blender --background ws.blend --python-expr "import bpy; [print('WS', w.name, w.use_filter_by_owner, [o.name for o in w.owner_ids]) for w in bpy.data.workspaces]"`

which printed:

```
WS Animation True []
WS Compositing False []
WS Geometry Nodes False []
WS Layout False ['cycles']
WS Modeling False []
WS Rendering False []
WS Scripting False []
WS Sculpting False []
WS Shading False []
WS Texture Paint False []
WS UV Editing False []
```

Exit status 0. So the add-on name `cycles` was written into the file, against the workspace that was active, and read back out of it with no window.

**A file's stored engine does not read back as itself when the add-on is gone.** A file was saved with the engine set to Cycles, Cycles was then switched off in that settings folder, and the file was opened again with no window:

`blender --background cycles.blend --python-expr "import bpy; s=bpy.context.scene; print('ENGINE', s.render.engine); print('HAS_CYCLES', hasattr(s,'cycles')); print('ITEMS', [i.identifier for i in bpy.types.RenderSettings.bl_rna.properties['engine'].enum_items]); print('KEYS', [k for k in s.keys()])"`

It printed:

```
ENGINE BLENDER_EEVEE_NEXT
HAS_CYCLES False
ITEMS ['BLENDER_EEVEE_NEXT']
KEYS ['cycles']
```

Cycles was switched back on and the same file opened again, printing `ENGINE CYCLES`, `HAS_CYCLES True`, `SAMPLES 1` — the sample count the file was saved with. (*Citation check, 2026-08-05: the command quoted above cannot print a `SAMPLES` line, and the command that did was not recorded, so this one step could not be repeated from this description. It was run again on 2026-08-05 with every command written down, and came back the same; that run is the next block.*) So the name is intact inside the file; it is only what a Blender without the add-on reports that changes. On the `KEYS` line, the add-on's own settings block survives in the file as a bare key that can still be listed.

**Outside the interface.** Settled: the workspace add-on list can be written, saved and read with Blender driven entirely from outside its window.

**Operating system.** Settled for macOS only, on the one machine described above. Linux was tested afterwards on a rented Linux machine and has its own block under this question; the two are never merged. Windows stays untried: no Windows machine has run it.

### Settled by hand test — the saved sample count read back, with the command written down

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-05, with Blender 4.5.12. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable — a folder made before it was used — so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

**Why it was run again.** The block above reports a `SAMPLES` line printed when the file was opened with Cycles switched back on. The command written down there prints no such line, and the command that did was never recorded, so that one step could not be repeated. The test was run again with every command written down. It came back the same, so this completes the record rather than changing what was found. The commands below are as they were typed, except that the program name is shortened to `blender` and the paths to their file names, as in the block above.

Cycles was switched on and the preferences saved:

`blender --background --python-expr "import bpy; print('ENABLE', bpy.ops.preferences.addon_enable(module='cycles')); bpy.ops.wm.save_userpref(); print('ADDONS', sorted(a.module for a in bpy.context.preferences.addons))"`

which printed:

```
ENABLE {'FINISHED'}
Info: Preferences saved
ADDONS ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
```

Exit status 0. A file was then saved with the engine set to Cycles and the sample count set to 1:

`blender --background --python-expr "import bpy; s=bpy.context.scene; s.render.engine='CYCLES'; s.cycles.samples=1; print('ENGINE', s.render.engine); print('SAMPLES', s.cycles.samples); bpy.ops.wm.save_as_mainfile(filepath='cycles.blend')"`

which printed:

```
ENGINE CYCLES
SAMPLES 1
Info: Saved as "cycles.blend"
```

Exit status 0.

**This is the command that prints a `SAMPLES` line.** The saved file was opened again, with no window:

`blender --background cycles.blend --python-expr "import bpy; s=bpy.context.scene; print('ENGINE', s.render.engine); print('HAS_CYCLES', hasattr(s,'cycles')); print('SAMPLES', getattr(getattr(s,'cycles',None),'samples',None))"`

It printed, word for word:

```
ENGINE CYCLES
HAS_CYCLES True
SAMPLES 1
```

Exit status 0.

Cycles was then switched off in the same settings folder with `blender --background --python-expr "import bpy; print('DISABLE', bpy.ops.preferences.addon_disable(module='cycles')); bpy.ops.wm.save_userpref()"`, which printed `DISABLE {'FINISHED'}` and `Info: Preferences saved`, exit status 0. The same reading command, run again on the same untouched file, printed:

```
Error: Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
ENGINE BLENDER_EEVEE_NEXT
HAS_CYCLES False
SAMPLES None
```

Exit status 0. Cycles was switched back on with `blender --background --python-expr "import bpy; print('ENABLE', bpy.ops.preferences.addon_enable(module='cycles')); bpy.ops.wm.save_userpref()"` — `ENABLE {'FINISHED'}`, `Info: Preferences saved`, exit status 0 — and the same reading command, run a third time, printed the same three lines as before:

```
ENGINE CYCLES
HAS_CYCLES True
SAMPLES 1
```

Exit status 0. So the sample count saved inside the file comes back as it was saved whenever the add-on is switched on, and reads as nothing at all while it is switched off, with the file never touched in between.

**Outside the interface.** Settled: every step here was done with Blender driven from outside its own window.

**Operating system.** Settled for macOS only, on the one machine described above. Linux was tested afterwards on a rented Linux machine and has its own block under this question; the two are never merged. Windows stays untried: no Windows machine has run it.

### Settled by hand test on the rented Linux machine — the workspace add-on list, what a file's engine reads as, and the saved sample count

**Label: observed.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH, on 2026-08-06, with Blender 4.5.12 LTS from `/opt/blender/blender-4.5.12-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

**The workspace add-on list behaves exactly as it did on the Mac.** The same two commands were run:

`blender --background --python-expr "import bpy; w=bpy.data.workspaces[0]; print('NAME', w.name); print('PROPS', [p for p in ('owner_ids','use_filter_by_owner','active_addon') if hasattr(w,p)]); w.use_filter_by_owner=True; print('ENABLE', bpy.ops.wm.owner_enable(owner_id='cycles')); print('OWNERS', [o.name for o in w.owner_ids]); bpy.ops.wm.save_as_mainfile(filepath='ws.blend')"`

printed, word for word:

```
NAME Animation
PROPS ['owner_ids', 'use_filter_by_owner', 'active_addon']
ENABLE {'FINISHED'}
OWNERS []
```

Exit status 0. Reading the saved file back:

`blender --background ws.blend --python-expr "import bpy; [print('WS', w.name, w.use_filter_by_owner, [o.name for o in w.owner_ids]) for w in bpy.data.workspaces]"`

```
WS Animation True []
WS Compositing False []
WS Geometry Nodes False []
WS Layout False ['cycles']
WS Modeling False []
WS Rendering False []
WS Scripting False []
WS Sculpting False []
WS Shading False []
WS Texture Paint False []
WS UV Editing False []
```

Exit status 0. Identical to the macOS run, down to the operator writing `cycles` against the active workspace rather than the one the script had in hand.

**The saved sample count reads back the same way.** A file was saved with the engine set to Cycles and the sample count set to 1, then opened again with no window:

`blender --background cycles.blend --python-expr "import bpy; s=bpy.context.scene; print('ENGINE', s.render.engine); print('HAS_CYCLES', hasattr(s,'cycles')); print('SAMPLES', getattr(getattr(s,'cycles',None),'samples',None))"`

```
ENGINE CYCLES
HAS_CYCLES True
SAMPLES 1
```

Cycles was switched off with `blender --background --python-expr "import bpy; print('DISABLE', bpy.ops.preferences.addon_disable(module='cycles')); bpy.ops.wm.save_userpref()"`, and the same reading command on the same untouched file printed:

```
Error: Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
ENGINE BLENDER_EEVEE_NEXT
HAS_CYCLES False
SAMPLES None
```

Cycles was switched back on and the third run printed `ENGINE CYCLES`, `HAS_CYCLES True`, `SAMPLES 1` again. Every run exited 0. The macOS run and this one agreed on every line.

**What a Blender without the add-on reports about the file.** Opening the same file in a settings folder where Cycles is switched off and asking for the scene's own keys:

`blender --background cycles.blend --python-expr "import bpy; s=bpy.context.scene; print('ENGINE', s.render.engine); print('HAS_CYCLES', hasattr(s,'cycles')); print('ITEMS', [i.identifier for i in bpy.types.RenderSettings.bl_rna.properties['engine'].enum_items]); print('KEYS', [k for k in s.keys()])"`

```
ENGINE BLENDER_EEVEE_NEXT
HAS_CYCLES False
ITEMS ['BLENDER_EEVEE_NEXT']
KEYS ['cycles']
```

Exit status 0. The add-on's own settings block survives in the file as a bare key that can still be listed. This is the same as the macOS run in this version.

**Outside the interface.** Settled again on a second operating system: the workspace add-on list can be written, saved and read, and a file's engine and sample count read back, with Blender driven entirely from outside its window.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed on every line above. Windows stays untried: no Windows machine has run it.

### Settled by hand test on the rented Linux machine — the workspace "Filter Add-ons" control at the window

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 4.5.12 LTS. Blender was opened **with a window** on the machine's own display, driven from a script on a timer, and photographed with Blender's own `bpy.ops.screen.screenshot`. A fresh settings folder was used, with the preferences saved once beforehand so that Blender's own Quick Setup panel is not in the picture.

The script turned the largest editor into the Properties editor, made it fill the window, set it to its Tool tab, and set the workspace's filter property. It printed:

```
PROPS PROPERTIES TOOL 682 404
WORKSPACE Layout
BEFORE use_filter_by_owner False
AFTER use_filter_by_owner True
OWNERS []
```

**The picture shows a section headed "Workspace" in the Tool tab, below a section headed "Options".** So the control the question is about is there in this version.

**Opened, on a second attempt, without clicking.** Blender offers no way for a script to open a panel that is drawn closed, and a click could not be relied on — see the note on the pointer below. The panel was opened instead by Blender's own operator:

`bpy.ops.wm.call_panel('INVOKE_DEFAULT', name='WORKSPACE_PT_addons', keep_open=True)`

whose identifier was read out of `bpy.types` rather than guessed. It returned `{'INTERFACE'}` in all four versions, and the panel is drawn as a small window over the editor.

**What it holds.** A list of add-ons, each row carrying the add-on's category before its name, and each with a tick box of its own at the right. Every box was empty, which matches `owner_ids` reading `[]`. **In this version it holds eight rows**: `Import-Exp… (BVH) format`, `Import-Exp…) 1.1 format`, `Import-Export: UV Layout`, `Import-Export: FBX format`, `Import-Export: glTF 2.0 f…`, `Render: Cycles Render En…`, `Animation: Pose Library`, `System: Blender Extensions` — the names are cut off by the width of the panel, and are written here as they appear. The same panel in Blender 5.2 holds three rows, not eight. The set of add-ons that is switched on is the same eight in all four versions, as recorded under question 10, so the shorter list in 5.2 is not a shorter set of add-ons.

**A note on the pointer, and what it is worth.** The machine has `xdotool`, so the pointer can be moved and clicked. Two things were established before any of the above was written down. Blender **does** receive button and key events sent to its own window: its own modal handler printed `EVENT LEFTMOUSE PRESS`, `EVENT LEFTMOUSE RELEASE`, `EVENT LEFTMOUSE CLICK` with the coordinates that were sent, and an `A` key press. But it receives **no pointer-motion events at all** — a modal handler watching for them saw none — and **a click could not be shown to work any control**: after clicking a Properties tab icon the editor's `context` still read `TOOL`; after clicking a preferences section `active_section` still read `INTERFACE`; after clicking the "Filter Add-ons" tick box `use_filter_by_owner` still read `False`. **So nothing above was got by clicking.** Every panel and menu was opened by one of Blender's own operators, with the identifier read out of `bpy.types` first rather than guessed, and each operator's own return value is quoted beside it.

**Outside the interface.** This is a window test and says nothing about being driven from outside. The property behind the control was set from a script with no window under question 1 above, and that half is settled.

**Operating system.** Settled for Linux in this version: the panel is there, and what it holds when opened is recorded above. On macOS this control was run in Blender 5.2 only, so there is no macOS result for this version to set beside it. Windows stays untested. *(Citation check, 2026-08-06: this line read "and is untested on Linux", which the block above it contradicts — the Linux run recorded exactly what the panel holds.)*


**Label: not in the sources.**

Nothing in the manual, the API documentation or the release notes describes a record inside a `.blend` file of the add-ons that file needs, and nothing describes a way to read such a record out of a file.

Two things came close enough that they should be reported, because both are real and both are easy to mistake for an answer.

**(a) A workspace can carry a list of add-on names, but it is an interface filter.** The API documentation gives a workspace two relevant members:

> `owner_ids`
>
> Type:
>
> `wmOwnerIDs bpy_prop_collection of wmOwnerID, (readonly)`

and

> `use_filter_by_owner`
>
> Filter the UI by tags

- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.WorkSpace.html
- Date read: 2026-07-30

The two operators that fill that list are described as add-on operations:

> `bpy.ops.wm.owner_disable(*, owner_id='')`
>
> Disable add-on for workspace

> `bpy.ops.wm.owner_enable(*, owner_id='')`
>
> Enable add-on for workspace

- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.wm.html
- Date read: 2026-07-30

And the manual, describing workspace options:

> Filter Add-ons
>
> Determines which add-ons are enabled in the active workspace.
> When unchecked, the global add-ons will be used.
> When checked, you can enable individual add-ons in the list below.

- Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/workspaces.html
- Date read: 2026-07-30

This is a list the artist may or may not have filled in, it is per workspace, and the API calls it a UI filter. The quoted words do not establish whether the list is stored in the `.blend`, and it is not a statement of what the file needs. **Label for this part: documented only for the workspace properties and their interface purpose.**

**(b) Blender can report the add-ons in use, but that describes the running program, not the file.** See question 10.

**Where I looked.** Whole-tree searches of the 4.5 manual, API documentation and release notes for: `add-on`, `addon`, `addon_utils`, `bl_info`, `blender_manifest`, `dependenc`, `missing`, `unregistered`, `undefined node`, `required add-on`, `add-on is required`, `requires the add-on`, `add-ons used`, `which add-ons`, `list of add-ons`, `enabled add-ons`. I read in full: the manual's Extensions pages, the Preferences Add-ons and Get Extensions pages, the blend-file Compatibility page, the Data-Blocks and Files sections, the Scripting & Security page, and the API pages for `bpy.types.Addon`, `bpy.types.Addons`, `bpy.types.Preferences` and `bpy.types.WorkSpace`. The only hit that ties add-on names to something saved in a `.blend` is the workspace filter above.

**Not covered.** Whether Blender stores anything internally that is not exposed or documented; whether a `.blend` can be parsed by anything outside Blender for this purpose; whether a linked library file changes the picture.

**Outside the interface.** The sources do not say. `WorkSpace.owner_ids` is an ordinary data property and nothing marks it as interface-only, but the sources never state that it is readable when Blender is driven from outside its window, so I will not claim it.

**Operating system.** The sources say nothing that makes any of this differ by operating system.

---

## 2. What makes a `.blend` need an add-on at the moment it renders, and can an add-on that takes part in producing the render be told apart from one that only rearranges Blender's existing tools while the artist works?

### Settled by hand test — which engines are registered, and whether an add-on's part in a render can be seen

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 4.5.12. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

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


**Label: documented in parts, inferred where joined, and not in the sources for the second half.**

I read the question's first half as: what can be saved in a file that makes an add-on necessary for the render to come out right. The other reading — what makes Blender demand an add-on at the instant rendering starts — is not answered anywhere I could find.

### What can make a file need an add-on

**The render engine.** The manual:

> Blender includes three render engines with different strengths:
>
> EEVEE is a physically based realtime renderer.
>
> Cycles is a physically based path tracer.
>
> Workbench is designed for layout, modeling and previews.
>
> More renderers from third-party developers are available as
> add-ons.

- Source: manual, https://docs.blender.org/manual/en/4.5/render/introduction.html
- Date read: 2026-07-30

Cycles itself is one of these add-ons. The command-line page says so plainly:

> Cycles add-on options must be specified following a double dash.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

And Cycles' own files carry an add-on's metadata block and define a render-engine class:

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

> ```
> class CyclesRender(bpy.types.RenderEngine):
>     bl_idname = 'CYCLES'
>     bl_label = "Cycles"
> ```

- Source: Cycles' own files, `cycles_4.5/__init__.py`, lines 7–16 and lines 42–44
- Date read: 2026-07-30
- **Label: observed.**
- *Citation check, 2026-08-05:* this said "register a render engine". The quoted lines define the class; registering happens in `register()` at line 134, which is not quoted. Narrowed to "define a render-engine class".

The API documentation lists `bpy.types.RenderEngine` as one of the classes Python may integrate with (https://docs.blender.org/api/4.5/info_overview.html, read 2026-07-30). The API exposes this render setting:

> `engine`
>
> Engine to use for rendering

- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Date read: 2026-07-30

**Inferred step:** if the engine saved in a file is one supplied by an add-on, then that add-on must be present and enabled for the file to render with that engine. The sources state that engines come from add-ons and that the file stores an engine; they never join those two sentences. The join is mine.

**Scripts saved inside the file.** These are not add-ons, but they are the documented way a file itself pulls Python into a render:

> Here are the different ways blend-files may automatically run scripts.
>
> Registered Text-Blocks
>
> A text data-block can have its Register option enabled which means it will load on start.
>
> Animation Drivers
>
> Python expressions can be used to Drive values and are often used in more advanced rigs and animations.

and

> Rendering with Freestyle, because Freestyle uses scripts to control line styles.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html
- Date read: 2026-07-30

### Telling a render-participating add-on from a working-comfort one

**Label: not in the sources.** Nothing in the sources marks an add-on as taking part in producing an image. What exists is metadata the author writes about themselves:

- `type` in the manifest — the manual lists its only values as "add-on", "theme" (https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html, read 2026-07-30). It separates add-ons from themes, nothing finer.
- `tags` — the manual's list of supported add-on tags includes `Render`, `Compositing`, `Bake`, `Lighting`, `Material`, `Node` alongside `User Interface`, `3D View`, `Text Editor` (https://docs.blender.org/manual/en/4.5/advanced/extensions/tags.html, read 2026-07-30). The manifest calls this a "tag list defined by Blender and server" and it is optional.
- `permissions` — `files`, `network`, `clipboard`, `camera`, `microphone`, each with a reason. Nothing about rendering.
- The legacy `bl_info` dictionary has a `category`; Cycles sets `"category": "Render"` (observed, `cycles_4.5/__init__.py` line 16 — *citation check, 2026-08-05: this said line 15, which is `"support": 'OFFICIAL'`*).

All of these are declarations by the author, chosen for browsing and searching. None of them is a statement that the add-on is required for the render, and none of them is checked against behaviour.

**Not covered.** Whether Blender behaves differently at render time for engine add-ons versus other add-ons; whether any add-on that only adds menus can still change a render (for example by writing settings on load); what happens with add-ons that register geometry-node or compositor node groups. I found no source for any of these.

**Outside the interface.** The render-engine points hold when Blender is driven from outside — `-E`/`--engine` is a command-line argument and `-E CYCLES` is the manual's own example (https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html, read 2026-07-30). The script-execution points explicitly differ: automatic execution is off by default and must be turned on for a background render, with the manual's own example `blender --background --enable-autoexec my_movie.blend --render-anim`. The telling-apart half is silent either way.

**Operating system.** The sources say nothing that makes any of this differ by operating system.

---

## 3. Can an add-on be installed, switched on, and given whatever else it needs before it will work, on a computer with no screen attached?

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


**Label: documented for installing and switching on; inferred for the settings an add-on needs.**

**No screen is needed to drive Blender.** The manual:

> One advantage of using the command line is that we do not need a graphical display
> (no need for X server on Linux for example)
> and consequently we can render via a remote shell (typically SSH).

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html
- Date read: 2026-07-30

**Installing.** There is a command-line tool for it, and it runs without a window by definition:

> `-c, --command <command>`
>
> Run a command which consumes all remaining arguments.
> Use `-c help` to list all other commands.
> Pass `--help` after the command to see its help text.
>
> This implies `--background` mode.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

> `install`:
>
> Install packages.
>
> `install-file`:
>
> Install package from file.

with

> `blender --command extension install [-h] [-s] [-e] [--no-prefs] PACKAGES`

and

> `blender --command extension install-file [-h] -r REPO [-e] [--no-prefs] FILE`
>
> Install a package file into a user repository.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html
- Date read: 2026-07-30

**Switching on.** Three documented routes.

`-e, --enable` on either install command:

> `-e, --enable`
>
> Enable the extension after installation.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html
- Date read: 2026-07-30

A start-up argument:

> `--addons <addon(s)>`
>
> Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

A start-up script, which the manual itself offers for exactly this:

> For example, a script can enable add-ons for every user.
>
> `$BLENDER_SYSTEM_SCRIPTS/startup/enable_addons.py`
>
> ```
> def register():
>     import addon_utils
>     addon_utils.enable("my-addon")
> ```

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html
- Date read: 2026-07-30

There is also an operator, `bpy.ops.preferences.addon_enable(*, module='')` — "Turn on this add-on" (Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.preferences.html, read 2026-07-30).

**Whatever else it needs before it will work.** An add-on's own settings are ordinary properties on an `AddonPreferences` class, reached like this in the API's own example:

> ```
> addon_prefs = preferences.addons[__name__].preferences
> ```

- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.AddonPreferences.html
- Date read: 2026-07-30

and preferences can be written out with `bpy.ops.wm.save_userpref()` — "Make the current preferences default" (https://docs.blender.org/api/4.5/bpy.ops.wm.html, read 2026-07-30).

**Inferred step:** because those settings are normal properties reachable from Python, and Python scripts can be run with `--python` in background mode, an add-on's settings can be set and saved without a screen. The sources never say this in one place; I joined the pieces. Where an add-on needs something that is not a property — a licence entered in a web browser, an interactive sign-in — the sources say nothing at all.

**Not covered.** Whether every add-on can be fully set up this way; whether an add-on that fails to register in background mode reports anything usable; whether the preferences file must already exist.

**Outside the interface.** This whole answer is about being outside the interface. The one difference the sources name is that `--command` forces background mode, so it cannot be mixed with an interactive session.

**Operating system.** The install and enable arguments are documented without any operating-system qualification. One nearby caution, about rendering rather than about add-ons: "Headless rendering is not supported on headless Windows systems" (manual, https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html, read 2026-07-30). That statement is in the EEVEE limitations page and I do not extend it to add-on installation.

---

## 4. What does an add-on record about the kinds of computer it is meant for, what happens on one it does not name, and does an add-on set up on one computer still work when copied to a computer of a different kind?

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


**Label: documented for what is recorded and for portability; not in the sources for what happens on an unnamed kind.**

### What it records

The manifest has an optional field:

> `platforms`:
>
> List of supported platforms. If omitted, the extension will be available in all operating systems.
> The available options are
> [“windows-x64”, “windows-arm64”, “macos-x64”, “macos-arm64”, “linux-x64”]

and in the example manifest:

> ```
> # # Optional: list of supported platforms. If omitted, the extension will be available in all operating systems.
> # platforms = ["windows-x64", "macos-arm64", "linux-x64"]
> # # Other supported platforms: "windows-arm64", "macos-x64"
> ```

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html
- Date read: 2026-07-30

Note that these names carry both the operating system and the processor kind.

It also records the Blender versions it is meant for — `blender_version_min` (required, "use at least 4.2.0") and `blender_version_max` (optional), from the same page.

The bundled Python modules are recorded per platform too:

> ```
> wheels = [
> "./wheels/pillow-10.3.0-cp311-cp311-macosx_11_0_arm64.whl",
> "./wheels/pillow-10.3.0-cp311-cp311-manylinux_2_28_x86_64.whl",
> "./wheels/pillow-10.3.0-cp311-cp311-win_amd64.whl",
> ]
> ```

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/python_wheels.html
- Date read: 2026-07-30

### What happens on a kind it does not name

**Label: not in the sources.** What the sources describe is filtering *before* it reaches the machine, not behaviour on the machine.

A repository that answers queries is told the platform, so it can leave incompatible packages out of the listing:

> When Blender fetches the extensions listing it passes the following arguments to make sure only
> compatible extensions are listed:
>
> `platform`
>
> `blender_version`

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/creating_repository/dynamic_repository.html
- Date read: 2026-07-30

A drop-in download link can carry the same information:

> `platforms`:
>
> Comma-separated list of supported platforms.
> If omitted, the extension will be available in all operating systems.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/creating_repository/static_repository.html
- Date read: 2026-07-30

And building can split a package by platform, skipping platforms the manifest does not name:

> Note
>
> Even though there is a Linux-only wheel present, no Linux zip file is generated.
> This happens because the platforms field only has Mac and Windows.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/python_wheels.html
- Date read: 2026-07-30

What none of these say is what Blender does if such a package is installed anyway — from disk, from a system repository, or by hand. I found no statement of that.

### Copied to a computer of a different kind

The manual is explicit that compiled parts do not travel:

> While not common practice, Python add-ons can be distributed with their own precompiled libraries. Unlike regular
> Python scripts, these are not portable between different platforms.
>
> It is possible the library is incompatible with your Blender installation (attempting to load a library built for a
> different version of Python, or loading a 32-bit library on a 64-bit system).
>
> If the add-on contains `.pyd` or `.so` files, check that the distribution is compatible with your operating
> system.

- Source: manual, https://docs.blender.org/manual/en/4.5/troubleshooting/python.html
- Date read: 2026-07-30

The settings side is also warned about — the identifier an add-on stores its preferences under is not the same string everywhere:

> The value of `__package__` will vary between systems so it should never be replaced with a literal string.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html
- Date read: 2026-07-30

**Not covered.** What Blender shows or does when an installed extension names no matching platform; whether a pure-Python add-on with no wheels is unaffected by all of this (the sources never say so outright); whether preferences copy cleanly between machines of the same kind.

**Outside the interface.** The command-line build tool takes `--split-platforms`, so the platform side of packaging is fully available outside the interface. Whether a platform mismatch is reported differently in background mode is not stated.

**Operating system.** This answer is about operating systems throughout. The five platform names cover Windows (x64, arm64), macOS (x64, arm64) and Linux (x64) — no other operating system has a name in the list.

---

## 5. How can an add-on be found, fetched and installed without a person at a screen, and what decides the places it can be fetched from?

### Settled by hand test — what the extension commands actually offer and print

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 4.5.12. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

**What a fresh settings folder holds.** `blender --command extension repo-list` printed three repositories:

```
blender_org:
    name: "extensions.blender.org"
    directory: "<settings folder>/extensions/blender_org"
    url: "https://extensions.blender.org/api/v1/extensions/"
    access_token: None
user_default:
    name: "User Default"
    directory: "<settings folder>/extensions/user_default"
    source: "USER"
system:
    name: "System"
    directory: "/Applications/Blender 4.5.12.app/Contents/Resources/4.5/extensions/system"
    source: "SYSTEM"
```

Exit status 0. So the two local repositories the manual speaks of are `User Default`, which lives in the user's own folder, and `System`, which lives inside the application.

**What `extension list` prints when nothing has been fetched.** `blender --command extension list` printed:

```
[Errno 2] No such file or directory: '<settings folder>/extensions/blender_org'
Repository data: <settings folder>/extensions/blender_org not found, sync required!
Repository: "extensions.blender.org" (id=blender_org)
[Errno 2] No such file or directory: '<settings folder>/extensions/user_default'
Repository: "User Default" (id=user_default)
Repository: "System" (id=system)
```

Exit status 0. Once a package is installed, its line appears indented under its repository, marked `[installed]`:

```
Repository: "User Default" (id=user_default)
  smoozy_test [installed]: "Smoozy Test", A tiny test add-on
```

Each line carries the package identifier, the name and the tagline. No version number, no switched-on state, and no built-in add-on ever appears — only packages in repositories.

**Fetching is gated by the online-access preference, not only by the network.** `blender --command extension list -s` in a fresh settings folder printed a traceback ending:

```
RuntimeError: Operator bpy.ops.extensions.repo_sync_all.poll() Online access required to check for updates. Enable online access in System preferences
Error synchronizing
```

exit status 1. With `--offline-mode` the last sentence became `Launch Blender without --offline-mode`. With `--online-mode` the fetch went ahead:

```
STATUS Checking repository "extensions.blender.org" for updates...
STATUS Refreshing extensions list for "extensions.blender.org"...
STATUS Extensions list for "extensions.blender.org" updated
```

exit status 0, and `extension list` then printed the whole catalogue of that repository, one line per package. The same gate applies to a repository that is a plain folder on this machine: syncing a `file://` repository was refused in exactly the same words until `--online-mode` was given.

**What the commands are.** `blender --command extension --help` printed the whole set, the same twelve in every version:

```
    server-generate     Create a listing from all packages.
    build               Build a package.
    validate            Validate a package.
    list                List all packages.
    sync                Synchronize with remote repositories.
    update              Upgrade any outdated packages.
    install             Install packages.
    install-file        Install package from file.
    remove              Remove packages.
    repo-list           List repositories.
    repo-add            Add repository.
    repo-remove         Remove repository.
```

`blender --command extension install --help` shows one positional argument, `PACKAGES` — "The packages to operate on (separated by ``,`` without spaces)" — and no way to give a web address. Installing by address is not offered.

**Outside the interface.** This whole answer is the outside-the-interface answer, and it is now settled by running it rather than by reading it.

**Operating system.** Settled for macOS only, on the one machine described above. The same tests were run again on a rented Linux machine, and that result has its own block under this question; the two are never merged. Windows stays untested.

### Settled by hand test on the rented Linux machine — what the extension commands offer and print, and what gates them

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 4.5.12 LTS from `/opt/blender/blender-4.5.12-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

**What a fresh settings folder holds.** `blender --command extension repo-list` printed three repositories:

```
blender_org:
    name: "extensions.blender.org"
    directory: "<settings folder>/extensions/blender_org"
    url: "https://extensions.blender.org/api/v1/extensions/"
    access_token: None
user_default:
    name: "User Default"
    directory: "<settings folder>/extensions/user_default"
    source: "USER"
system:
    name: "System"
    directory: "/opt/blender/blender-4.5.12-linux-x64/4.5/extensions/system"
    source: "SYSTEM"
```

Exit status 0. The same three as on macOS, with the System repository inside this machine's own Blender folder rather than inside the application bundle. Nothing appeared inside the settings folder — `find` returned the folder alone.

**What `extension list` prints when nothing has been fetched:**

```
[Errno 2] No such file or directory: '<settings folder>/extensions/blender_org'
Repository data: <settings folder>/extensions/blender_org not found, sync required!
Repository: "extensions.blender.org" (id=blender_org)
[Errno 2] No such file or directory: '<settings folder>/extensions/user_default'
Repository: "User Default" (id=user_default)
Repository: "System" (id=system)
```

Exit status 0. Once a package is installed its line appears indented beneath its repository, marked `[installed]`, carrying the identifier, the name and the tagline, with no version number and no switched-on state — the same shape as on macOS.

**The set of subcommands is the same twelve**, word for word, in every version on this machine, and the same twelve the macOS run recorded:

```
    server-generate     Create a listing from all packages.
    build               Build a package.
    validate            Validate a package.
    list                List all packages.
    sync                Synchronize with remote repositories.
    update              Upgrade any outdated packages.
    install             Install packages.
    install-file        Install package from file.
    remove              Remove packages.
    repo-list           List repositories.
    repo-add            Add repository.
    repo-remove         Remove repository.
```

`blender --command extension install --help` shows one positional argument, `PACKAGES`, and no way to give a web address.

**Fetching is gated by the online-access preference, not only by the network**, in identical words:

```
RuntimeError: Operator bpy.ops.extensions.repo_sync_all.poll() Online access required to check for updates. Enable online access in System preferences
Error synchronizing
```

exit status 1. With `--offline-mode` the last sentence became `Launch Blender without --offline-mode`, exit status 1. With `--online-mode` the fetch went ahead:

```
STATUS Checking repository "extensions.blender.org" for updates...
STATUS Refreshing extensions list for "extensions.blender.org"...
STATUS Extensions list for "extensions.blender.org" updated
```

exit status 0, and `extension list` then printed that repository's whole catalogue, over a thousand indented lines. The catalogue is somebody else's and changes; it is not copied here. The same gate applied to a repository that is a plain folder on this same machine, until `--online-mode` was given.

**Asking for updates for a package installed from a file** printed nothing and exited 1; with `--online-mode` it printed only `Repository data: <settings folder>/extensions/blender_org not found, sync required!` and exited 0. Nothing was ever offered for the package installed from a file.

Every one of these agreed with the macOS run.

**Outside the interface.** Settled again on a second operating system: the whole cycle — listing repositories, listing packages, syncing, updating, installing, validating — runs with Blender driven from outside its own window, and `--command` forces that mode.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed, the only difference being where the System repository sits on disk. Windows stays untested, and the `file://HOST/share/...` address form the sources give for Windows was not tried.

### Settled by hand test on the rented Linux machine — the Get Extensions panel at the window

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 4.5.12 LTS, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`. A fresh settings folder, preferences saved once beforehand.

**In a fresh settings folder the panel does not show a repository list at all.** The picture shows a section headed **Online Extensions** carrying three lines —

- "Internet access is required to install and update online extensions."
- "You can adjust this later from \"System\" preferences."
- "While offline, use \"Install from Disk\" instead."

— and two buttons beneath them, **Continue Offline** and **Allow Online Access**. Above that sit a search field, a filter drop-down reading **Add-ons**, and a drop-down reading **Repositories**.

**What the repositories hold, read from the preferences rather than from the drop-down.** A script cannot open a drop-down, so the same information was read from the properties behind it. It printed each repository as its name, its module, whether it is enabled, whether it has a remote address, that address, and its four tick-box settings:

```
REPOS [('extensions.blender.org', 'blender_org', True, True, 'https://extensions.blender.org/api/v1/extensions/', False, False, True, False), ('User Default', 'user_default', True, False, '', False, False, False, False), ('System', 'system', True, False, '', False, False, False, False)]
```

and the whole list of settings a repository carries:

```
REPO PROPS ['access_token', 'custom_directory', 'directory', 'enabled', 'module', 'name', 'remote_url', 'rna_type', 'source', 'use_access_token', 'use_cache', 'use_custom_directory', 'use_remote_url', 'use_sync_on_startup']
```

with the extensions preferences themselves carrying only `['active_repo', 'repos', 'rna_type', 'use_online_access_handled']`.

So the three repositories are the same three the command line lists, each is enabled, only `extensions.blender.org` has an address, none demands an access token, none uses a custom directory, and only `extensions.blender.org` is set to check on startup.

**The repositories drop-down, opened without clicking.** A click on it could not be shown to do anything. It was opened instead with Blender's own operator:

`bpy.ops.wm.call_menu('INVOKE_DEFAULT', name='USERPREF_MT_extensions_active_repo')`

the identifier read out of `bpy.types` rather than guessed. It returned `{'INTERFACE'}` in all four versions. In **Blender 5.2** that menu draws the three repositories with their tick boxes, their address, a **Check for Updates on Startup** tick and a closed **Advanced** section. In this version the menu opened — the operator returned `{'INTERFACE'}` — but in every picture taken a tooltip for the menu's `Refresh Remote` item covered the list, so **what this version's menu lists was not read**.

**The settings menu beside it.** `bpy.ops.wm.call_menu('INVOKE_DEFAULT', name='USERPREF_MT_extensions_settings')` returned `{'INTERFACE'}` and opens a menu headed **Extension Settings** holding **Visit Extensions Platform** (its tooltip: "Open a preset website in the web browser. Online directory of free and open source extensions"), **Install Available Updates**, greyed out, and **Install from Disk…**. The same three in all four versions.

**A note on the pointer, and what it is worth.** The machine has `xdotool`, so the pointer can be moved and clicked. Two things were established before any of the above was written down. Blender **does** receive button and key events sent to its own window: its own modal handler printed `EVENT LEFTMOUSE PRESS`, `EVENT LEFTMOUSE RELEASE`, `EVENT LEFTMOUSE CLICK` with the coordinates that were sent, and an `A` key press. But it receives **no pointer-motion events at all** — a modal handler watching for them saw none — and **a click could not be shown to work any control**: after clicking a Properties tab icon the editor's `context` still read `TOOL`; after clicking a preferences section `active_section` still read `INTERFACE`; after clicking the "Filter Add-ons" tick box `use_filter_by_owner` still read `False`. **So nothing above was got by clicking.** Every panel and menu was opened by one of Blender's own operators, with the identifier read out of `bpy.types` first rather than guessed, and each operator's own return value is quoted beside it.

**Outside the interface.** This is a window test. The matching no-window results are under question 5 above; the two agree on the three repositories and their addresses.

**Operating system.** Settled for Linux in this version, at the window. The Creator's macOS run covered Blender 5.2 only. Windows stays untested.


**Label: documented.**

### Finding, fetching, installing

All from the extensions command-line tool, which implies background mode:

> `list`:
>
> List all packages.
>
> `sync`:
>
> Synchronize with remote repositories.
>
> `update`:
>
> Upgrade any outdated packages.
>
> `install`:
>
> Install packages.
>
> `install-file`:
>
> Install package from file.
>
> `remove`:
>
> Remove packages.

with, for `list`:

> `blender --command extension list [-h] [-s]`
>
> List packages from all enabled repositories.

for `sync`:

> `blender --command extension sync [-h]`
>
> Download package information for remote repositories.

and for `install`:

> `PACKAGES`:
>
> The packages to operate on (separated by `,` without spaces).
>
> ...
>
> `-s, --sync`
>
> Sync the remote directory before performing the action.
>
> `-e, --enable`
>
> Enable the extension after installation.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html
- Date read: 2026-07-30

There is also an operator, for scripted use: `bpy.ops.extensions.package_install(*, repo_directory='', repo_index=-1, pkg_id='', enable_on_install=True, url='', do_legacy_replace=False)` — "Download and install the extension" (Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.extensions.html, read 2026-07-30).

### What decides the places it can be fetched from

**Repositories held in the preferences.** Repositories are added, listed and removed from the command line too:

> `blender --command extension repo-add [-h] [--name NAME] [--directory DIRECTORY] [--url URL] [--access-token ACCESS_TOKEN] [--source SOURCE] [--cache BOOLEAN] [--clear-all] [--no-prefs] ID`
>
> Add a new local or remote repository.

> `--url URL`
>
> The URL, for remote repositories (optional).
> When omitted the repository is considered “local”
> as it is not connected to an external repository,
> where packages may be installed by file or managed manually.

> `--access-token ACCESS_TOKEN`
>
> The access token to use for remote repositories which require a token.

> `--source SOURCE`
>
> The type of source in (‘USER’, ‘SYSTEM’).
> System repositories are managed outside of Blender and are considered read-only.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html
- Date read: 2026-07-30

**What ships by default.**

> By default Blender has a Remote Repository pointing towards the
> Official Blender Extensions Platform and two Local Repositories.
>
> In the cases where more repositories are needed (e.g., to access third party extension platforms),
> new repositories can be added.

- Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/extensions.html
- Date read: 2026-07-30

The picture on that page's Repositories section shows exactly three entries, all ticked: `extensions.blender.org` with a globe icon and the address `https://extensions.blender.org/api/v1/extensions/` beneath it, then `User Default` and `System`.

- Source: manual, picture `_images/editor_preferences_section_extensions_repositories.png`, on the page https://docs.blender.org/manual/en/4.5/editors/preferences/extensions.html
- Date read: 2026-07-30
- **Label: documented** (what is visible in the picture; the reading of the icons as remote-versus-local is mine and would be inferred).

**A URL may be a local path.** The repository URL field is described as:

> Remote URL to the extension repository, the file-system may be referenced using the file URI scheme: “file://”

- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.UserExtensionRepo.html
- Date read: 2026-07-30

and the manual's own testing instructions use `file:///path/to/packages/index.json`, including `file://HOST/share/path/to/packages/index.json for network shares on Windows` (https://docs.blender.org/manual/en/4.5/advanced/extensions/creating_repository/static_repository.html, read 2026-07-30).

**Who may host.**

> The official extensions platform for the Blender project is extensions.blender.org.
> Other third party sites can also be supported, as long as they follow the Extensions Platform specification.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/index.html
- Date read: 2026-07-30

**Whether the machine may reach the internet at all.** The preference is off by default:

> `use_online_access`
>
> Allow Blender to access the internet. Add-ons that follow this setting will only connect to the internet if enabled. However, Blender cannot prevent third-party add-ons from violating this rule.
>
> Type:
>
> boolean, default False

- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.PreferencesSystem.html
- Date read: 2026-07-30

and it can be forced from the command line:

> `--online-mode`
>
> Allow internet access, overriding the preference.
>
> `--offline-mode`
>
> Disallow internet access, overriding the preference.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

with the deployment page adding:

> Working offline can be enforced by running with the `--offline-mode`
> command line argument. Users
> will then be unable to enable online access in the preferences.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html
- Date read: 2026-07-30

Two further limits on fetching are in the preferences: `network_timeout` ("The time in seconds to wait for online operations before a connection may fail with a time-out error. Zero uses the systems default.", default 0) and `network_connection_limit` ("Limit the number of simultaneous internet connections online operations may make at once. Zero disables the limit.", default 0) — same API page as above.

**Not covered.** Whether `--command extension install` can take a bare URL rather than a package identifier; how a package identifier is chosen when two repositories both hold one; whether the access token can be supplied per-invocation rather than stored.

**Outside the interface.** This entire answer is the outside-the-interface path; the only inside-only routes the sources name are dragging a URL or a `.zip` into the window and the Install button.

**Operating system.** Documented without operating-system qualification, except that the `file://` form for network shares is given specifically for Windows. One network note is environment-specific rather than system-specific: a 403 error reaching extensions.blender.org on networks with extra SSL management, which "can't be resolved in Blender's side" (manual, https://docs.blender.org/manual/en/4.5/troubleshooting/network.html, read 2026-07-30).

---

## 6. What does an add-on belong to, once it is installed on a computer?

### Settled by hand test — where an installed add-on sits, and whether it belongs to one Blender

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 4.5.12. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

**Where it lands.** After installing a small test add-on into the `user_default` repository of a fresh settings folder, the whole folder held:

```
<settings folder>/config/userpref.blend
<settings folder>/extensions/.cache/compat.dat
<settings folder>/extensions/user_default/.blender_ext
<settings folder>/extensions/user_default/smoozy_test/__init__.py
<settings folder>/extensions/user_default/smoozy_test/__pycache__/__init__.cpython-311.pyc
<settings folder>/extensions/user_default/smoozy_test/blender_manifest.toml
```

The manifest on disk was the same text that was packed, unchanged. Blender's own account of those places:

`blender --background --python-expr "import bpy; print('EXT', bpy.utils.user_resource('EXTENSIONS')); print('CONFIG', bpy.utils.user_resource('CONFIG')); print('SCRIPTS', bpy.utils.user_resource('SCRIPTS')); print('USER', bpy.utils.resource_path('USER')); print('LOCAL', bpy.utils.resource_path('LOCAL')); print('SYSTEM', bpy.utils.resource_path('SYSTEM')); print('EXTPATH', bpy.utils.extension_path_user('bl_ext.user_default.smoozy_test', path='', create=False))"`

printed:

```
EXT <settings folder>/extensions
CONFIG <settings folder>/config
SCRIPTS <settings folder>/scripts
USER <settings folder>
LOCAL /Applications/Blender 4.5.12.app/Contents/Resources/4.5
SYSTEM /Library/Application Support/Blender/4.5
EXTPATH <settings folder>/extensions/.user/user_default/smoozy_test
```

So the writable folder Blender hands an add-on is not inside the add-on's own folder but under `extensions/.user/<repository>/<id>`.

**Its name carries its repository.** The switched-on list printed it as `bl_ext.user_default.smoozy_test`.

**Belonging to one Blender version is a matter of the folder, not of any check.** One settings folder was made and given to all four Blenders in turn. The add-on was installed once, with Blender 5.2. Then each Blender in turn was pointed at the same folder:

```
VERSION 5.2.0 LTS
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', ...]
VERSION 5.1.2
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', ...]
VERSION 5.0.1
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', ...]
VERSION 4.5.12 LTS
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', ...]
```

and `blender --command extension list` showed `smoozy_test [installed]` for every one of them. The shared folder grew no version-named subfolder: it held only `config` and `extensions`. So an add-on belongs to whichever settings folder it was put in, and it is the default folder's version-numbered name that keeps versions apart — not anything in Blender that refuses another version's add-ons.

**Outside the interface.** Settled: the folder layout, the naming and the sharing all behave the same with Blender driven from outside its window, and `BLENDER_USER_RESOURCES` moves all of it.

**Operating system.** Settled for macOS only, on the one machine described above. The same tests were run again on a rented Linux machine, and that result has its own block under this question; the two are never merged. Windows stays untested.

### Settled by hand test on the rented Linux machine — where an installed add-on sits, and whether it belongs to one Blender

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 4.5.12 LTS from `/opt/blender/blender-4.5.12-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

**Where the files land.** After installing the test add-on, `find <settings folder> -maxdepth 5 | sort` printed:

```
<settings folder>
<settings folder>/config
<settings folder>/config/userpref.blend
<settings folder>/extensions
<settings folder>/extensions/.cache
<settings folder>/extensions/.cache/compat.dat
<settings folder>/extensions/user_default
<settings folder>/extensions/user_default/.blender_ext
<settings folder>/extensions/user_default/smoozy_test
<settings folder>/extensions/user_default/smoozy_test/__init__.py
<settings folder>/extensions/user_default/smoozy_test/__pycache__
<settings folder>/extensions/user_default/smoozy_test/__pycache__/__init__.cpython-311.pyc
<settings folder>/extensions/user_default/smoozy_test/blender_manifest.toml
```

and the manifest on disk was the same nine lines that were packed, unchanged. The same as the macOS run, but for the Python version in the cached file's name.

**The folders Blender hands out:**

```
EXT <settings folder>/extensions
CONFIG <settings folder>/config
SCRIPTS <settings folder>/scripts
USER <settings folder>
LOCAL /opt/blender/blender-4.5.12-linux-x64/4.5
SYSTEM
EXTPATH <settings folder>/extensions/.user/user_default/smoozy_test
```

Note the last line: the writable folder Blender hands the add-on is **not** inside the add-on's own folder. `SYSTEM` came back empty, as it did on macOS.

**Belonging to one Blender version is a matter of the folder and nothing else.** One settings folder was handed to all four Blenders in turn. An add-on installed once by 5.2 was listed by 5.1, 5.0 and 4.5 from that same folder —

```
Repository: "User Default" (id=user_default)
  smoozy_test [installed]: "Smoozy Test", A tiny test add-on
```

— and each of them had it switched on:

```
VERSION 4.5.12 LTS
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

The shared folder grew no version-named subfolder. Nothing in Blender refused another version's add-on. The same as the macOS run.

**A mismatched package's folder** dropped straight into the extensions directory is listed but cannot be switched on — see question 4.

**Outside the interface.** Settled again on a second operating system: every path above was read, and the shared-folder test run, with Blender driven from outside its own window.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed; only the paths themselves differ, which is the rule working rather than a difference. Windows stays untested.


**Label: documented.**

The question can be read two ways — what holds it, or who owns it. I answer the first; the ownership reading is question 8's licence material.

**It belongs to a repository.** Repositories are either user-managed or system-provided:

> `source`
>
> Select if the repository is in a user managed or system provided directory
>
> USER
> User – Repository managed by the user, stored in user directories.
>
> SYSTEM
> System – Read-only repository provided by the system.

- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.UserExtensionRepo.html
- Date read: 2026-07-30

Installing from a file puts it in a local repository specifically:

> Install from Disk
>
> Install an extension from a .zip package.
> This is installed to a Local Repository and no updates will be available.

- Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/extensions.html
- Date read: 2026-07-30

**It belongs to a directory, under a version-numbered Blender folder.** The user directory is per version — `$HOME/.config/blender/4.5/` on Linux, `/Users/$USER/Library/Application Support/Blender/4.5/` on macOS, `%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\4.5\` on Windows — and within it:

> `./extensions`
>
> Extension repositories.
>
> Located in both user and system directories. Repositories are loaded from
> both directories.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html
- Date read: 2026-07-30

A bundled system repository has a stated shape:

> `$BLENDER_SYSTEM_EXTENSIONS/system/my-addon/blender_manifest.toml`

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html
- Date read: 2026-07-30

Legacy add-ons instead belong to a scripts directory: `./scripts/addons/*.py` — "Python add-ons which may be enabled in the Preferences include import/export format support, render engine integration and many handy utilities." (directory layout page, as above).

**It belongs to a Python name that includes its repository.**

> The legacy add-ons would use their module name to access the preferences. This could lead to a name clash when
> extensions with the same name (from different repositories) would be installed.
> To prevent this conflict, the repository name is now part of the namespace.
>
> For example, now instead of `kitsu` the module name would be `bl_ext.{repository_module_name}.kitsu` instead.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html
- Date read: 2026-07-30

**It may own a small directory of its own, which Blender ties to its life.**

> This directory will be kept between upgrades but will be removed if the extension is uninstalled.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html
- Date read: 2026-07-30

reached with `bpy.utils.extension_path_user(package, *, path='', create=False)` — "Return a user writable directory associated with an extension." (https://docs.blender.org/api/4.5/bpy.utils.html, read 2026-07-30). The manual warns the add-on's own folder is not a safe place to write:

> Add-ons must not assume their own directory is user writable since this may not be the case for “System” repositories.
> Writing files into the add-on’s directory also has the down side that upgrading the extension will remove all files.

(same page).

**Not covered.** Whether an add-on can be moved between repositories after installation; what happens to an add-on's stored preferences when its repository is removed (the interface offers "Remove Repository & Files" and says "These changes are permanent and cannot be reversed", but says nothing about preferences); whether two repositories may hold the same add-on at once.

**Outside the interface.** The directories and repositories are the same either way; `BLENDER_USER_RESOURCES`, `BLENDER_SYSTEM_EXTENSIONS` and `BLENDER_SYSTEM_SCRIPTS` are environment variables, and `--env-system-extensions` and `--env-system-scripts` are command-line arguments, so all of this is reachable from outside.

**Operating system.** The user and system directory paths differ per operating system, and the manual gives all three sets, quoted above. The portable-installation folder differs too: next to the executable on Windows and Linux, but "Inside the application bundle at Blender.app/Contents/Resources" on macOS.

---

## 7. What does a render do when an add-on it needs is not there?

### Settled by hand test — what a render does when the add-on behind its engine is not there

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 4.5.12. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

This is the question all four versions' documents record as *not in the sources*. It is now answered, three ways.

**A file whose engine is Cycles, rendered with Cycles switched off.** A file was saved with the engine set to Cycles, one sample, 64 by 64, and rendered once with Cycles on to prove it worked — it wrote `ok_0001.png`, exit status 0. Cycles was then switched off and its preferences saved. The same file was rendered again:

`blender --background cycles.blend --render-output <folder>/off_ --render-frame 1`

The render printed:

```
Error: Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
```

and then **carried on and rendered anyway**, with EEVEE, sixty-four samples:

```
Fra:1 | Rendering 1 / 64 samples
Fra:1 | Rendering 25 / 64 samples
Fra:1 | Rendering 50 / 64 samples
Fra:1 | Rendering 64 / 64 samples
Saved: '<folder>/off_0001.png'
```

An image file appeared, 3.5 kB against the 4.8 kB of the Cycles one, and **the process exited 0**. Nothing in the exit status says the wrong engine was used.

**The same file with the engine forced on the command line.** `blender --background cycles.blend -E CYCLES --render-output <folder>/forced_ --render-frame 1` printed the same first message and then:

```
Error: engine not found 'CYCLES'
```

The process was killed: `libc++abi: terminating` and `Abort trap: 6`, exit status 134. No image was written.

**A file whose engine belongs to an add-on that was never installed.** A test add-on supplying an engine called `SMOOZY_TEST` was installed, a file saved with that engine, and one frame rendered — it wrote an image, exit status 0. The same file was then rendered in a settings folder where that add-on had never existed:

```
Error: Engine 'SMOOZY_TEST' not available for scene 'Scene' (an add-on may need to be installed or enabled)
```

followed by the same EEVEE render and `Saved: '<folder>/without_0001.png'`, exit status 0. Opening that file in the same bare Blender and asking for its engine printed the same message and then `ENGINE BLENDER_EEVEE_NEXT`.

**An add-on that changes a render without being an engine.** A test add-on that adds a function to `bpy.app.handlers.render_pre` was installed, and one frame rendered: the render printed the add-on's own line `SMOOZY HANDLER RAN`. Rendering the same file where the add-on had never been installed printed **no message at all** about it, finished normally and exited 0.

So: a missing engine add-on is reported on the terminal, the render falls back to EEVEE, an image is written and the exit status is 0. A missing add-on of any other kind is not reported at all. Only forcing the engine with `-E` stops the render, and in this version that ends by killing the process rather than exiting cleanly.

**Outside the interface.** Settled: the message goes to the terminal, and there is no dialog and no prompt. What a window shows instead was not tested here.

**Operating system.** Settled for macOS only, on the one machine described above. The same tests were run again on a rented Linux machine, and that result has its own block under this question; the two are never merged. Windows stays untested.

### Settled by hand test on the rented Linux machine — what a render does when the add-on behind its engine is not there

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 4.5.12 LTS from `/opt/blender/blender-4.5.12-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

**A file whose engine is Cycles, rendered with Cycles switched off, renders anyway.** A 64 by 64, one-sample Cycles file was saved and rendered once with Cycles on, writing `ok_0001.png`. Cycles was then switched off and the same file rendered again:

```
Error: Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
Fra:1 Mem:39.08M (Peak 39.11M) | Time:00:00.48 | Rendering 1 / 64 samples
Fra:1 Mem:39.08M (Peak 39.11M) | Time:00:00.58 | Rendering 64 / 64 samples
Saved: '<output folder>/off_0001.png'
```

**Exit status 0.** The middle of the render's progress lines is cut above; the first and last are kept. An image was written — `off_0001.png` against `ok_0001.png`, a few hundred bytes apart, both around 3 to 5 kB — and nothing in the exit status says the wrong engine was used. The same as the macOS run.

**Forcing the engine on the command line writes no image.** `blender --background cycles.blend -E CYCLES --render-output ... --render-frame 1` printed the same "not available" message and then:

```
Error: engine not found 'CYCLES'
```

with exit status 124 — the run was wrapped in `timeout 300` and 4.5 never exited on its own. No `forced_0001.png` appeared in the output folder. On macOS this run ended in an abort in 4.5 and exit 1 in the others; here 4.5 hangs instead of aborting, and the others exit 1 as they did there.

**A file whose engine belongs to an add-on that was never installed behaves the same way.** A test add-on supplying an engine called `SMOOZY_TEST` was built and installed; `blender -E help` then listed it beside the built-in engines. A file was saved with that engine and rendered, writing `with_0001.png`. Rendering the same file in a settings folder where the add-on had never been installed printed:

```
Error: Engine 'SMOOZY_TEST' not available for scene 'Scene' (an add-on may need to be installed or enabled)
```

and then rendered and wrote `without_0001.png` anyway, exit status 0. Asking that bare Blender for the file's engine gave the same message and then `ENGINE BLENDER_EEVEE_NEXT`.

**An add-on that changes a render without being an engine is completely silent when absent.** The render printed no message at all about it, finished, and exited 0. Both images measured 128 by 128.

Every one of these agreed with the macOS run, except the way Blender 4.5 ends after `-E CYCLES` fails.

**Outside the interface.** Settled again on a second operating system: every render above was started from outside Blender's window, and the messages went to the terminal.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed on what a render does; they differ only in how Blender 4.5 ends after a forced engine is not found — an abort on macOS, a hang on Linux. Windows stays untested.


**Label: not in the sources.**

I could not find any statement of what happens to a render when an add-on the file depends on is absent or disabled — not an error, not a substitution, not a fallback, not an exit code.

**Where I looked.** Whole-tree searches of the 4.5 manual, API documentation and release notes for: `missing`, `not found`, `not installed`, `not enabled`, `fails`, `failed`, `fallback`, `falls back`, `reverts to`, `incompatible`, `not compatible`, `unregistered`, `undefined node`, `unknown node`, `engine is not`, `required add-on`, `add-on is required`. I read in full: the command-line arguments page and the command-line rendering page, the Render introduction, the Preferences Add-ons and Get Extensions pages, the Extensions Add-ons page, the Scripting & Security page, the Troubleshooting section (Python errors, network, crash, startup, report a bug), and the API pages for `bpy.ops.extensions`, `bpy.ops.preferences`, `bpy.types.RenderEngine` and `bpy.types.RenderSettings`. The 4.5 release notes were searched for `add-on`, `addon` and `extension` in full; nothing bears on this.

The closest the sources come are three things that are not answers:

- "If the Add-on does not activate when enabled, check the Console window for any errors that may have occurred." (manual, https://docs.blender.org/manual/en/4.5/editors/preferences/addons.html, read 2026-07-30). This is about enabling, not rendering.
- The engine list can be asked for: "Use `-E help` to list available engines." (manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30). That tells you what is present, not what happens when something is absent.
- For scripts blocked by the security setting: "When a blend-file tries to execute a script and is not allowed, a dialog will appear. In it, you can choose to Allow Execution or to Ignore the scripts." (manual, https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html, read 2026-07-30). That is a different situation — the script is present, permission is not — and it describes a dialog, which is an interface thing.

**Not covered.** Everything the question asks. In particular: whether a render aborts or proceeds, whether anything is written to the output, what the process exit code is, and whether any of that differs between an engine add-on and any other kind.

**Outside the interface.** Not answered either way. Note that the one nearby documented behaviour — the auto-run dialog — is a dialog, and the sources do not say what stands in its place with no window.

**Operating system.** The sources say nothing.

---

## 8. Where do add-ons come from besides Blender's own, how are they sold, and what does that mean for taking a copy of one from one computer to another?

### Settled by hand test — where packages come from, and what a copy installed from a file can do

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 4.5.12. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

**What is set up to begin with.** `blender --command extension repo-list` in a fresh settings folder printed three entries — `extensions.blender.org` with the address `https://extensions.blender.org/api/v1/extensions/` and no access token, `User Default` marked `source: "USER"`, and `System` marked `source: "SYSTEM"` and pointing inside the application at `/Applications/Blender 4.5.12.app/Contents/Resources/4.5/extensions/system`.

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

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 4.5.12 LTS from `/opt/blender/blender-4.5.12-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

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

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 4.5.12 LTS, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`.

**An add-on's opened row shows no licence and no repository name.** The two rows opened — Cycles Render Engine and Node Wrangler — carry a website, a type reading `Core`, and for one of them a maintainer and a version. Neither carries a licence, a repository, a tag or a declared permission. So a person cannot tell from a row what they are allowed to do with a copy. Both add-ons ship with Blender, so what a row shows for an add-on installed from a repository is still unsettled here, exactly as it was on the Mac.

**The only setting on the repositories side bearing on getting at an add-on is an access token.** Read from the preferences rather than from the drop-down, a repository carries `access_token` and `use_access_token` among its settings, and all three repositories in a fresh settings folder have `use_access_token` false. Full list under question 5.

**Outside the interface.** This is a window test. Nothing here changes what is reachable from outside the window.

**Operating system.** Settled for Linux in this version, at the window. The Creator's macOS run covered Blender 5.2 only. Nothing here bears on how add-ons are sold.


**Label: documented for where they come from and for ways to install or share a package; inferred for the permission to copy an official-platform add-on; not in the sources for how add-ons are sold or for the terms of add-ons from elsewhere.**

### Where they come from

> The official extensions platform for the Blender project is extensions.blender.org.
> Other third party sites can also be supported, as long as they follow the Extensions Platform specification.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/index.html
- Date read: 2026-07-30

Anyone can host their own, and the sources give both shapes: a static repository, which needs only a JSON file generated by `blender --command extension server-generate` and plain static-file hosting, and a dynamic repository, which answers queries with a filtered listing (manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/creating_repository/static_repository.html and .../dynamic_repository.html, read 2026-07-30).

A file on disk is a source in its own right:

> Install from Disk
>
> Use the drop-down menu in the top right,
> or drag-and-drop an extension .zip package into Blender.

- Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/extensions.html
- Date read: 2026-07-30

And the loose end:

> Blender comes with some preinstalled Add-ons already, ready to be enabled.
> But you can also add your own, or any interesting ones you find on the web.

- Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/addons.html
- Date read: 2026-07-30

Render engines specifically: "More renderers from third-party developers are available as add-ons." (manual, https://docs.blender.org/manual/en/4.5/render/introduction.html, read 2026-07-30).

### How they are sold

**Label: not in the sources.** Nothing in the manual, API documentation or release notes describes selling, pricing, payment, purchase, licence keys, or a marketplace for add-ons.

**Where I looked.** Whole-tree searches for: `sell`, `selling`, `commercial`, `paid`, `purchase`, `marketplace`, `price`, `payment`, `proprietary`, `license`, `licence`. The only hits that are about add-ons at all are licence statements, not sales.

What the sources do say is about licences, and only for the official platform:

> The Blender Extensions Platform
> only supports free and open source extensions compliant with the
> Blender’s license:
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
> Public Domain (CC0).

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/licenses.html
- Date read: 2026-07-30

The manifest requires the licence to be declared, using SPDX identifiers, with `"SPDX:GPL-3.0-or-later"` as the example (manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html, read 2026-07-30).

The manual's own summary of what the GPL grants includes:

> You have the right to copy and distribute the program.

with the responsibilities that accompany it, including "You must include the source code or make the source code freely available."

- Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/about/license.html
- Date read: 2026-07-30

Note carefully: that page is about Blender itself, and ends "The GPL only applies to the Blender application and not the artwork you create with it". The requirement that add-ons be GPL-3.0-or-later is stated as a rule of the official platform, not as a rule of Blender.

### What that means for taking a copy from one computer to another

The sources document several ways to install the same package on another machine or make a repository available to several machines:

- A `.zip` can be installed on the other machine from disk; that lands it in a local repository and "no updates will be available" (Preferences page, quoted above).
- For many machines at once, a shared read-only system repository: "This repository could be located on a read-only network drive or in a system directory." (manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html, read 2026-07-30).
- But compiled parts do not travel between kinds of computer — see question 4's quote from the Python troubleshooting page.
- And the add-on's own settings identifier is not the same string everywhere: "The value of `__package__` will vary between systems so it should never be replaced with a literal string." (manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html, read 2026-07-30).

**Inferred step:** because the platform requires add-ons on it to be GPL-3.0-or-later, and the GPL grants the right to copy and distribute, an add-on obtained from the official platform may be copied between your own machines. The sources state both halves separately; the join is mine, and it does not extend to add-ons from anywhere else, whose terms the sources say nothing about.

**Not covered.** Whether third-party sites may sell; whether an add-on may require activation or a key; whether a bought add-on's terms can forbid copying; anything at all about the extensions platform's own account or moderation rules beyond "The extension will be held for review, and published once the moderation team approves it."

**Outside the interface.** Copying, hosting and installing all have command-line equivalents (see question 5). The only routes named as interface-only are drag-and-drop and the Install button.

**Operating system.** Where an add-on comes from does not differ by operating system in the sources. Whether the copy works on arrival does — see question 4.

---

## 10. Can a computer be asked which add-ons it holds, and for which Blender?

### Settled by hand test — what can be asked of a computer about its add-ons

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 4.5.12. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

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
PATHS ['/Applications/Blender 4.5.12.app/Contents/Resources/4.5/scripts/addons_core']
COUNT 14
```

followed by one line per add-on with its whole metadata block. The 14 are `io_anim_bvh`, `bl_pkg` and `copy_global_transform`, `cycles`, `io_scene_fbx`, `io_scene_gltf2`, `hydra_storm`, `ui_translate`, `node_wrangler`, `pose_library`, `rigify`, `io_curve_svg`, `io_mesh_uv_layout` and `viewport_vr_preview` — every add-on that ships with Blender, switched on or not. With the test extension installed, `bl_ext.user_default.smoozy_test` appeared in that list too, so this route covers both kinds. `addon_utils` is the closest thing to an inventory that was found.

**The system-information report can be written with no window, and it names versions and paths.** `blender --background --python-expr "import bpy; print('SYSINFO', bpy.ops.wm.sysinfo(filepath='system-info.txt'))"` printed `SYSINFO {'FINISHED'}`, exit status 0, and wrote the file with no prompt of any kind. Its add-on section reads:

```
Enabled add-ons:
====================================

io_anim_bvh (version: (1, 0, 1), path: '/Applications/Blender 4.5.12.app/Contents/Resources/4.5/scripts/addons_core/io_anim_bvh/__init__.py')
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

### Settled by hand test on the rented Linux machine — what can be asked of a computer about its add-ons

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 4.5.12 LTS from `/opt/blender/blender-4.5.12-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

**What is switched on.** `blender --background --python-expr "import bpy; a=bpy.context.preferences.addons; print('COUNT', len(a)); print('MODULES', sorted(x.module for x in a))"` printed:

```
COUNT 8
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

The same eight as on macOS. That collection drops an add-on the moment it is switched off: after switching the test add-on off and saving the preferences, its module was gone from this list while `blender --command extension list` still printed `smoozy_test [installed]`. Both machines agreed.

**What is on the machine at all.** `blender --background --python-expr "import addon_utils; print('PATHS', addon_utils.paths()); m=list(addon_utils.modules()); print('COUNT', len(m)); ..."` printed:

```
PATHS ['/opt/blender/blender-4.5.12-linux-x64/4.5/scripts/addons_core']
COUNT 14
```

and then one line per add-on with its whole metadata block. The add-ons named were `io_anim_bvh`, `bl_pkg`, `copy_global_transform`, `cycles`, `io_scene_fbx`, `io_scene_gltf2`, `hydra_storm`, `ui_translate`, `node_wrangler`, `pose_library`, `rigify`, `io_curve_svg`, `io_mesh_uv_layout`, `viewport_vr_preview` — the same set, in the same order, that the macOS run recorded for this version.

**The system-information report can be written with no window and with no prompt.** `blender --background --python-expr "import bpy; print(bpy.ops.wm.sysinfo(filepath='system-info.txt'))"` printed `{'FINISHED'}`, exit status 0, and the file's "Enabled add-ons" section read:

```
Enabled add-ons:
=========================================

io_anim_bvh (version: (1, 0, 1), path: '.../addons_core/io_anim_bvh/__init__.py')
io_curve_svg (version: UNKNOWN, path: '.../addons_core/io_curve_svg/__init__.py')
io_mesh_uv_layout (version: (1, 2, 0), path: '.../addons_core/io_mesh_uv_layout/__init__.py')
io_scene_fbx (version: (5, 13, 1), path: '.../addons_core/io_scene_fbx/__init__.py')
io_scene_gltf2 (version: (4, 5, 51), path: '.../addons_core/io_scene_gltf2/__init__.py')
cycles (version: UNKNOWN, path: '.../addons_core/cycles/__init__.py')
pose_library (version: (2, 0), path: '.../addons_core/pose_library/__init__.py')
bl_pkg (version: (0, 0, 1), path: '.../addons_core/bl_pkg/__init__.py')
```

The paths above are cut to their last two parts; each is the full path inside this machine's Blender folder. It names each add-on's version and path, with `UNKNOWN` where the metadata block carries none — Cycles among them. The same as the macOS run.

**Which Blender the answer belongs to is a matter of the folder alone.** One settings folder handed to all four Blenders in turn gave each of them the same installed add-on — see question 6.

**Outside the interface.** Settled again on a second operating system: all three routes, and the system-information report, are reachable with Blender driven from outside its own window and with no prompt of any kind.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed on the routes, the switched-on set and the bundled set. Windows stays untested.

### Settled by hand test on the rented Linux machine — what the Add-ons list and an opened row show

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 4.5.12 LTS, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`.

**The Add-ons list holds the add-ons that ship with Blender, and nothing else.** The picture of the list in a fresh settings folder shows, in order: BioVision Motion Capture (BVH) format, Copy Global Transform, Cycles Render Engine, FBX format, glTF 2.0 format, Hydra Storm render engine, Manage UI translations, Node Wrangler, Pose Library, Rigify, Scalable Vector Graphics (SVG) 1.1 format, UV Layout, VR Scene Inspection. Each has a tick box, and the ones ticked are the ones the no-window run lists as switched on. The extensions system itself, `bl_pkg`, is in that switched-on list but has no row of its own.

**An opened row shows a version and a maintainer for one add-on and neither for Cycles.** Node Wrangler's row carries `Maintainer` with five names and `Version 3.55`; Cycles' row carries neither. That matches the `UNKNOWN` the system-information report gives for Cycles from outside the window. Neither row shows a repository name or a location on disk. The same as the Creator's macOS run in Blender 5.2.

**Outside the interface.** This is a window test. The matching no-window results are under question 10 above, and the two agree on which add-ons are there and which are switched on.

**Operating system.** Settled for Linux in this version, at the window. The Creator's macOS run covered Blender 5.2 only. Windows stays untested.


**Label: documented for the available commands, data and report; inferred for the separation between Blender versions; not in the sources for the noted gaps.**

### Asking which add-ons

From the command line:

> `blender --command extension list [-h] [-s]`
>
> List packages from all enabled repositories.

and, for the repositories themselves:

> `blender --command extension repo-list [-h]`
>
> List all repositories stored in Blender’s preferences.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html
- Date read: 2026-07-30

The subcommand summary at the top of the same page words it differently — "`list`: List all packages." — so it is not stated whether the output separates what is installed from what is merely available in a repository. I flag that as unsettled rather than guess.

From Python, there is an add-ons collection:

> `addons`
>
> Type:
>
> `Addons bpy_prop_collection of Addon, (readonly)`

- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.Preferences.html
- Date read: 2026-07-30

where an `Addon` is described as:

> class `bpy.types.Addon(bpy_struct)`
>
> Python add-ons to be loaded automatically
>
> `module`
>
> Module name

- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.Addon.html
- Date read: 2026-07-30

As a report file:

> Save System Info
>
> This extracts system information which can be useful for including in bug reports, inspecting the configuration, or
> diagnosing problems.
>
> You will be prompted to save a text file called `system-info.txt`.

whose sections include:

> Enabled Add-Ons
>
> Lists add-ons currently in use along with their versions and paths.

- Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/help.html
- Date read: 2026-07-30

The operator behind it is `bpy.ops.wm.sysinfo(*, filepath='')` — "Generate system information, saved into a text file" (https://docs.blender.org/api/4.5/bpy.ops.wm.html, read 2026-07-30).

The manual also offers, for locations:

> ```
> import addon_utils
> print(addon_utils.paths())
> ```

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/scripting/addon_tutorial.html
- Date read: 2026-07-30

(`addon_utils` is used in two manual pages but has no page of its own in the 4.5 API documentation; I searched the whole API tree for `addon_utils` and found nothing.)

### For which Blender

The question is answered by the fact that every route above is run *by* a particular Blender executable, plus these:

- The user directory is version-numbered — `.../blender/4.5/`, `.../Blender/4.5/`, `...\Blender\4.5\` — and `./extensions` lives inside it (manual, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html, read 2026-07-30).
- Python can ask for a version's base path explicitly: `bpy.utils.resource_path(type, major=bpy.app.version[0], minor=bpy.app.version[1])` — "Return the base path for storing system files.", with `type` "string in [‘USER’, ‘LOCAL’, ‘SYSTEM’]" and major/minor defaulting to the current version. And `bpy.utils.user_resource(resource_type, ...)` takes `'EXTENSIONS'` among its resource types (https://docs.blender.org/api/4.5/bpy.utils.html, read 2026-07-30).
- Each add-on declares its own Blender range: `blender_version_min` (required) and `blender_version_max` (optional) (https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html, read 2026-07-30).
- A repository is told which Blender is asking: the query parameter `blender_version` (https://docs.blender.org/manual/en/4.5/advanced/extensions/creating_repository/dynamic_repository.html, read 2026-07-30).

**Inferred step:** since the extensions directory sits inside a folder named for the Blender version, add-ons installed for one version are not automatically the same set as another version's. The sources show the layout; they do not spell out the consequence. The existence of `bpy.ops.preferences.copy_prev()` — "Copy settings from previous version" (https://docs.blender.org/api/4.5/bpy.ops.preferences.html, read 2026-07-30) — points the same way but is about settings, not add-ons.

**Not covered.** Whether `extension list` shows installed and available separately, or shows enabled state; whether `preferences.addons` includes installed-but-disabled add-ons (the wording "to be loaded automatically" suggests not, but I will not claim it); whether anything short of running Blender can be asked.

**Outside the interface.** `--command extension list` and `--command extension repo-list` are outside-only by definition, and `bpy.ops.wm.sysinfo` takes a `filepath`, so it need not prompt. Whether it prompts when no window exists is not stated.

**Operating system.** The commands and API are documented without operating-system qualification; only the directory paths differ, and the manual gives all three.

---

## 11. Can an add-on depend on something else being on the computer, or on a computer elsewhere, before it will work, and can what it needs be read anywhere?

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


**Label: documented for self-contained extensions, their manifest fields and the online-access setting; inferred for the absence of a declared extension-to-extension dependency mechanism; not in the sources for legacy add-ons and the other gaps noted below.**

### On something else on the computer

For an add-on to be bundled as an extension, the documented rule is that it must bring its dependencies:

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

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html
- Date read: 2026-07-30

The bundled modules are named in the manifest:

> `wheels`:
>
> List of relative file-paths Python Wheels.

with requirements: "Wheels must be bundled unmodified from Python’s package index." and "Wheels must include their dependencies." (manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/python_wheels.html, read 2026-07-30). Installing puts them somewhere Blender's Python can see: "Now installing the package will extract the wheel into the extensions own site-packages directory." (same page).

**Inferred step:** the manifest reference on the getting-started page lists every required value (`blender_version_min`, `id`, `license`, `maintainer`, `name`, `schema_version`, `tagline`, `type`, `version`) and every optional one (`blender_version_max`, `website`, `copyright`, `tags`, `platforms`, `wheels`, `permissions`), and no field among them names another extension as a prerequisite. Together with the instruction to bundle another add-on when you depend on it, I read this as: there is no way to declare "this add-on needs that add-on". The sources never say so outright, so this is inferred, not documented.

One further thing an add-on depends on is Blender's own Python:

> Blender and the bpy module are only compatible
> with a single Python version. This makes it possible for add-ons and VFX software
> in general to only have to target a single Python version.
>
> Blender bundles a complete Python installation and does not interact with the
> system Python by default. This can be changed with the `--python-use-system-env`
> command line argument, if care is
> taken to set up a compatible Python version.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html
- Date read: 2026-07-30

A related fix in this version, quoted as a documented change:

> Python3.dll dependency in extensions cant be loaded from subprocesses..? leads to import issues in extension

- Source: release notes, https://developer.blender.org/docs/release_notes/4.5/bugfixes/
- Date read: 2026-07-30

### On a computer elsewhere

An extension manifest can declare that the add-on needs network access:

> ```
> # # Optional: add-ons can list which resources they will require:
> # # * files (for access of any filesystem operations)
> # # * network (for internet access)
> # # * clipboard (to read and/or write the system clipboard)
> # # * camera (to capture photos and videos)
> # # * microphone (to capture audio)
> # #
> # # If using network, remember to also check `bpy.app.online_access`
> ```
>
> ```
> # [permissions]
> # network = "Need to sync motion-capture data to server"
> ```

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html
- Date read: 2026-07-30

> If the add-on needs to use internet, it must check for the (read-only) property `bpy.app.online_access`.
> This option is controlled by Preferences, which can be overriding via command-line
> (`--offline-mode` / `--online-mode`).

and, importantly:

> Add-ons that follow this setting will only connect to the internet if enabled.
> However, Blender cannot prevent third-party add-ons from violating this rule.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/addons.html
- Date read: 2026-07-30

The property itself: `bpy.app.online_access` — "Boolean, true when internet access is allowed by Blender & 3rd party scripts (read-only)", with `bpy.app.online_access_override` — "Boolean, true when internet access preference is overridden by the command line (read-only)" (https://docs.blender.org/api/4.5/bpy.app.html, read 2026-07-30).

### Can what it needs be read anywhere

For an extension, the manifest records its bundled wheels and any declared permissions. The sources also document validation of a manifest without first extracting a package:

> To validate the manifest without building the package:
>
> `blender --command extension validate`
>
> You may also validate a package without having to extract it first.
>
> `blender --command extension validate add-on-package.zip`

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/extensions/getting_started.html
- Date read: 2026-07-30

with the subcommand described as taking:

> `SOURCE_PATH`:
>
> The package source path (either directory containing package files or the package archive).
> This path must containing a `blender_manifest.toml` manifest.

- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html
- Date read: 2026-07-30

The deployment page gives an installed-extension example at `$BLENDER_SYSTEM_EXTENSIONS/system/my-addon/blender_manifest.toml`. A repository's listing carries manifest information: the static repository page says of the drop-URL parameters, "With the exception of the repository, all the other parameters can be extracted from the extensions manifest." The sources do not say that the validation command prints the fields, so they do not establish a command that reports every recorded need.

**Gap.** There is no documented Python API for reading an installed extension's manifest. I searched the whole 4.5 API documentation for `manifest` and found no occurrence at all.

**Not covered.** Whether wheel version conflicts between two add-ons are resolved, and how — the manual's own Python Wheels page carries an open note: "Finalize a policy for how conflicting versions of a wheel are handled." Also not covered: whether the declared permissions are enforced in any way (the sources say only that Blender cannot prevent violations of the online-access rule); whether an add-on may depend on a licence server or a specific GPU; whether legacy (non-extension) add-ons declare anything at all beyond `bl_info`.

**Outside the interface.** `--command extension validate` is outside-only by definition, and `bpy.app.online_access` is a plain read-only property. The online-access preference is explicitly overridable from the command line in both directions, which is the difference that matters here: a machine driven from outside can be forced offline regardless of what its preferences say.

**Operating system.** Nothing in this answer is stated to differ by operating system, except that which wheels apply depends on the platform (question 4).

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
