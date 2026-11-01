# Add-ons — Blender 5.0

This document answers the Add-ons question sheet for Blender 5.0 from the permitted sources.

**Sources consulted:** the Blender 5.0 manual (plain-text snapshot and its `_images` folder), the Blender 5.0 Python API documentation, the Blender 5.0 release notes, and the Cycles add-on's own Python files — all inside `Blender Sources/`. All were read on **2026-07-30** (the snapshot date), and this report was written on 2026-08-04.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

---

## How I searched

Before answering I searched the whole manual and the whole API documentation at once, rather than opening likely files. The terms I used, across `manual_5.0`, `api_5.0` and `release_notes_5.0`:

`add-on`, `addon`, `add_on`, `extension`, `bl_ext`, `bl_info`, `blender_manifest`, `manifest`, `repository`, `repo`, `wheel`, `platform`, `depend`, `dependency`, `missing`, `not installed`, `not found`, `not registered`, `undefined`, `requires add-on`, `required add-on`, `add-on is missing`, `missing extension`, `unknown engine`, `engine not`, `fallback`, `sell`, `sold`, `purchase`, `price`, `paid`, `commercial`, `market`, `store`, `donat`, `headless`, `X server`, `display server`, `no display`, `autoexec`, `trusted source`, `addon_utils`, `online_access`, `factory-startup`.

The pages that carry nearly all the add-on material in 5.0 are:

- https://docs.blender.org/manual/en/5.0/advanced/extensions/index.html and its sub-pages (`getting_started`, `addons`, `licenses`, `tags`, `version_number_guidelines`, `python_wheels`, `creating_repository/*`)
- https://docs.blender.org/manual/en/5.0/editors/preferences/extensions.html
- https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html
- https://docs.blender.org/manual/en/5.0/advanced/command_line/extension_arguments.html
- https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html
- https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html
- https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html
- https://docs.blender.org/manual/en/5.0/advanced/scripting/addon_tutorial.html
- https://docs.blender.org/manual/en/5.0/troubleshooting/python.html and https://docs.blender.org/manual/en/5.0/troubleshooting/network.html
- https://docs.blender.org/api/5.0/bpy.ops.extensions.html, https://docs.blender.org/api/5.0/bpy.ops.preferences.html, https://docs.blender.org/api/5.0/bpy.types.Addon.html, https://docs.blender.org/api/5.0/bpy.types.Addons.html, https://docs.blender.org/api/5.0/bpy.types.AddonPreferences.html, https://docs.blender.org/api/5.0/bpy.types.UserExtensionRepo.html, https://docs.blender.org/api/5.0/bpy.types.PreferencesExtensions.html, https://docs.blender.org/api/5.0/bpy.utils.html, https://docs.blender.org/api/5.0/bpy.app.html, https://docs.blender.org/api/5.0/info_overview.html

---

## 1. Can a `.blend` file be read for the add-ons it needs?

### Settled by hand test — the workspace add-on list, and what a file's engine reads as

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.0.1. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

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
ENGINE BLENDER_EEVEE
HAS_CYCLES False
ITEMS ['BLENDER_EEVEE']
KEYS []
```

Cycles was switched back on and the same file opened again, printing `ENGINE CYCLES`, `HAS_CYCLES True`, `SAMPLES 1` — the sample count the file was saved with. (*Citation check, 2026-08-05: the command quoted above cannot print a `SAMPLES` line, and the command that did was not recorded, so this one step could not be repeated from this description. It was run again on 2026-08-05 with every command written down, and came back the same; that run is the next block.*) So the name is intact inside the file; it is only what a Blender without the add-on reports that changes. On the `KEYS` line, nothing at all is listed, so the add-on's own settings block cannot be seen this way.

**Outside the interface.** Settled: the workspace add-on list can be written, saved and read with Blender driven entirely from outside its window.

**Operating system.** Settled for macOS only, on the one machine described above. Linux was tested afterwards on a rented Linux machine and has its own block under this question; the two are never merged. Windows stays untried: no Windows machine has run it.

### Settled by hand test — the saved sample count read back, with the command written down

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-05, with Blender 5.0.1. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable — a folder made before it was used — so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

**Why it was run again.** The block above reports a `SAMPLES` line printed when the file was opened with Cycles switched back on. The command written down there prints no such line, and the command that did was never recorded, so that one step could not be repeated. The test was run again with every command written down. It came back the same, so this completes the record rather than changing what was found. The commands below are as they were typed, except that the program name is shortened to `blender` and the paths to their file names, as in the block above.

Cycles was switched on and the preferences saved:

`blender --background --python-expr "import bpy; print('ENABLE', bpy.ops.preferences.addon_enable(module='cycles')); bpy.ops.wm.save_userpref(); print('ADDONS', sorted(a.module for a in bpy.context.preferences.addons))"`

which printed:

```
ENABLE {'FINISHED'}
Info: Preferences saved
ADDONS ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
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
00:00.282  reports          | ERROR Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
ENGINE BLENDER_EEVEE
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

**Label: observed.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH, on 2026-08-06, with Blender 5.0.1 from `/opt/blender/blender-5.0.1-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

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
00:00.217  reports          | ERROR Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
ENGINE BLENDER_EEVEE
HAS_CYCLES False
SAMPLES None
```

Cycles was switched back on and the third run printed `ENGINE CYCLES`, `HAS_CYCLES True`, `SAMPLES 1` again. Every run exited 0. The macOS run and this one agreed on every line.

**What a Blender without the add-on reports about the file.** Opening the same file in a settings folder where Cycles is switched off and asking for the scene's own keys:

`blender --background cycles.blend --python-expr "import bpy; s=bpy.context.scene; print('ENGINE', s.render.engine); print('HAS_CYCLES', hasattr(s,'cycles')); print('ITEMS', [i.identifier for i in bpy.types.RenderSettings.bl_rna.properties['engine'].enum_items]); print('KEYS', [k for k in s.keys()])"`

```
ENGINE BLENDER_EEVEE
HAS_CYCLES False
ITEMS ['BLENDER_EEVEE']
KEYS []
```

Exit status 0. No key at all is left on the scene. This is the same as the macOS run in this version.

**Outside the interface.** Settled again on a second operating system: the workspace add-on list can be written, saved and read, and a file's engine and sample count read back, with Blender driven entirely from outside its window.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed on every line above. Windows stays untried: no Windows machine has run it.

### Settled by hand test on the rented Linux machine — the workspace "Filter Add-ons" control at the window

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 5.0.1. Blender was opened **with a window** on the machine's own display, driven from a script on a timer, and photographed with Blender's own `bpy.ops.screen.screenshot`. A fresh settings folder was used, with the preferences saved once beforehand so that Blender's own Quick Setup panel is not in the picture.

The script turned the largest editor into the Properties editor, made it fill the window, set it to its Tool tab, and set the workspace's filter property. It printed:

```
PROPS PROPERTIES TOOL 682 407
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


**Answer.** No such thing is described. Nothing in the manual, the API documentation or the release notes says a `.blend` file records the add-ons it needs, and nothing offers a way to read such a list out of a file.

Two nearby things do exist, and neither is the thing asked for:

**(a) A workspace inside the file can carry a list of add-on names.** This is a user-interface filter, not a record of need.

> Filter Add-ons
>
> Determines which add-ons are enabled in the active workspace.
> When unchecked, the global add-ons will be used.
> When checked, you can enable individual add-ons in the list below.

Source: manual, https://docs.blender.org/manual/en/5.0/interface/window_system/workspaces.html — read 2026-07-30.

> `active_addon`
>
> Active Add-on in the Workspace Add-ons filter
>
> `owner_ids`
>
> Type: `wmOwnerIDs` `bpy_prop_collection` of `wmOwnerID`, (readonly)
>
> `use_filter_by_owner`
>
> Filter the UI by tags

Source: API documentation, https://docs.blender.org/api/5.0/bpy.types.WorkSpace.html — read 2026-07-30. The same page opens with "class bpy.types.WorkSpace(ID)" and "Workspace data-block, defining the working environment for the user".

The operators that write that list are named for add-ons:

> `bpy.ops.wm.owner_enable(*, owner_id='')`
>
> Enable add-on for workspace

Source: API documentation, https://docs.blender.org/api/5.0/bpy.ops.wm.html — read 2026-07-30.

That this list travels inside the `.blend` is **inferred**, in two steps: the manual says workspaces are data-blocks, and that data-blocks are what a blend-file contains.

> The base unit for any Blender project is the data-block. Examples of data-blocks include: meshes, objects, materials, textures, node trees, scenes, texts, brushes, and even Workspaces.

> They are the primary contents of the blend-file.

Source: manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html — read 2026-07-30.

**(b) The 5.0 release notes mention Cycles scene settings**, but say that one script route does not give access to them:

> `bpy.context.scene['cycles']` will not give access to Cycles' scene settings.

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/python_api/ — read 2026-07-30.

**LABEL:** not in the sources (for the question as asked). The workspace filter and the release-note quote are **documented**; the claim that the workspace list is stored in the file is **inferred**.

**Not covered.** Whether any tool outside Blender can list add-on-owned data inside a `.blend`; whether the workspace filter list survives a file being opened where those add-ons are absent; whether a file records the add-on that created any particular data-block.

**Where I looked (for the silence).** Searched the whole manual, API and release notes for `requires add-on`, `required add-on`, `add-on is missing`, `missing extension`, `depend`/`dependency` (the only add-on hits are about bundling Python modules), and read the whole of `files/` in the manual (`introduction`, `data_blocks`, `custom_properties`, `blend/open_save`, `blend/compatibility`, `blend/packed_data`, `linked_libraries/*`). None of them mentions add-ons being recorded in or read from a file.

**Outside the interface.** The workspace filter is described only as a user-interface feature. The API documents the related properties and operators. That a script driving Blender could read `WorkSpace.owner_ids` after opening a file is **inferred** from those API entries. The sources do not say more.

**Operating system.** The sources draw no operating-system distinction here.

---

## 2. What makes a `.blend` file need an add-on at the moment it renders, and can an add-on that takes part in producing the render be told apart from one that only rearranges Blender's existing tools while the artist works?

### Settled by hand test — which engines are registered, and whether an add-on's part in a render can be seen

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.0.1. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

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

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.0.1 from `/opt/blender/blender-5.0.1-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

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

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 5.0.1, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`. Fresh settings folders, preferences saved once beforehand so the Quick Setup panel is not in the picture.

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


**Answer, first part.** The sources describe ingredients by which an add-on can take part in rendering: enabling an add-on runs its `register()` function, which loads its classes into Blender; an add-on can provide a render engine, and Cycles registers a property group on Blender's scene data. That a file naming such an engine or holding such properties depends on that add-on at render time is **inferred**. The quoted words establish registration and the stored engine choice, but do not state what dependency follows.

> When the add-on is enabled, Blender executes the code and runs the `register()` function. When the add-on is disabled, Blender runs the `unregister()` function.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/scripting/addon_tutorial.html — read 2026-07-30.

> Registering a class with Blender results in the class definition being loaded into Blender, where it becomes available alongside existing functionality. Once this class is loaded you can access it from `bpy.types`, using the `bl_idname` rather than the classes original name.

Source: API documentation, https://docs.blender.org/api/5.0/info_overview.html — read 2026-07-30.

> More renderers from third-party developers are available as add-ons.

Source: manual, https://docs.blender.org/manual/en/5.0/render/introduction.html — read 2026-07-30.

Cycles itself is such an add-on, and its own files show the two hooks exactly. **Observed:**

```
class CyclesRender(bpy.types.RenderEngine):
    bl_idname = 'CYCLES'
    bl_label = "Cycles"
```
File: `Blender Sources/cycles_5.0/__init__.py`, lines 42–44.

```
    def register(cls):
        bpy.types.Scene.cycles = PointerProperty(
```
File: `Blender Sources/cycles_5.0/properties.py`, lines 1102–1103.

```
def register():
    from bpy.utils import register_class
    ...
    properties.register()
    ui.register()
    operators.register()
    presets.register()

    for cls in classes:
        register_class(cls)
```
File: `Blender Sources/cycles_5.0/__init__.py`, lines 133–153.

The scene stores which engine to use:

> `engine`
>
> Engine to use for rendering

Source: API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30. (The documented enum on that page lists only `'BLENDER_EEVEE'`, because the list is filled in at run time by whatever engines are registered — that last clause is **inferred** from the `RenderEngine` registration mechanism above, not stated.)

And the Add-ons list in Preferences shows a render engine sitting there as an ordinary add-on with a checkbox: the screenshot `_images/editors_preferences_section_addons.webp` (on https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html) lists "Cycles Render Engine" among "BioVision Motion Capture (BVH) format", "Node Wrangler", "Rigify" and others, each with a tick-box. **documented** — read 2026-07-30.

**Answer, second part.** The sources give no machine-readable mark that separates an add-on which produces the render from one which only rearranges the interface. The nearest things are all descriptive:

- The manifest's `type` only distinguishes add-on from theme: `type: "add-on", "theme".` (https://docs.blender.org/manual/en/5.0/advanced/extensions/getting_started.html)
- Tags/categories are described as areas affected, not roles: "Add-ons are assigned categories by what areas of Blender they affect." (https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html)
- The legacy `bl_info` dictionary carries a free-text category. **Observed:** `"category": "Render"` in `Blender Sources/cycles_5.0/__init__.py`, line 16.
- The manifest's `[permissions]` table names resources (`files`, `network`, `clipboard`, `camera`, `microphone`), not whether the add-on takes part in a render.

**LABEL:** **documented** for registration and for render engines being available as add-ons; **observed** for the Cycles code; **inferred** for the conclusion that the stored engine or add-on properties make a render depend on that add-on; **not in the sources** for a reliable way to tell a render-participating add-on from an interface-only one.

**Not covered.** Whether a file can be inspected to discover *which* add-on owns a given piece of stored data; whether an add-on that only adds menus can still change render output (for example through drivers or handlers); anything about geometry-node or compositor node types supplied by add-ons.

**Outside the interface.** The manual documents enabling an add-on from a startup script, and the tags and categories are read from files. That these parts need no person clicking is **inferred** from those routes. The Preferences screenshot is only meaningful inside the interface.

**Operating system.** The sources draw no operating-system distinction for registration or for engine selection.

---

## 3. Can an add-on be installed, switched on, and given whatever else it needs before it will work, on a computer with no screen attached?

### Settled by hand test — installing, switching on, and setting up an add-on with no window

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.0.1. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

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

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.0.1 from `/opt/blender/blender-5.0.1-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

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

**Label: observed.** Run on **2026-08-06** on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all, reached over SSH** — with Blender at `/opt/blender/blender-5.0.1-linux-x64/blender`, `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`. Every run pointed Blender at a fresh settings folder of its own through `BLENDER_USER_RESOURCES`, exported in the shell that ran Blender. **Nothing else was running on the machine.** This is a third machine; nothing here is merged with the Creator's Mac or with the one-card rented Linux machine. The full list is `Audits/Blender Research/Add-ons/Hand tests — two cards and no screen.md`.

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
[Errno 2] No such file or directory: '/root/smoozy/no screen/res-5.0/extensions/blender_org'
Repository data: /root/smoozy/no screen/res-5.0/extensions/blender_org not found, sync required!
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

Run over SSH on a **third rented Linux machine — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, 197 GB of memory, no display server** — using Blender at `/opt/blender/blender-5.0.1-linux-x64/blender`, `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/four cards/addons-5.0`, a folder deleted and remade at the start of the run. Label: **observed**. This is a **fourth** machine; nothing here is merged with the Creator's Mac, the one-card rented Linux machine or the two-card rented Linux machine. The full list, with every command written out, is `Audits/Blender Research/Add-ons/Hand tests — four cards.md`.

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
00:00.036  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.036  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4556, total unfreed memory 1.272339 MB
```

on **standard output**, and exited **1**. That run also wrote to standard error, fourteen times over:

```
Freeing memory after the leak detector has run. This can happen when using static variables in C++ that are defined outside of functions. To fix this error, use the 'construct on first use' idiom.
```

That is the same message and the same exit status the one-card machine gave when cut off from its display, plus the memory-leak line that machine never showed.

**What this settles.** **The whole add-on cycle — build, install, list, switch on, switch off, refresh, set an add-on's own setting, save it and read it back — works on a computer with no display server at all**, in this version, as long as Blender is not asked to open a window. Asked to open one it cannot, it says so and stops. That answers item B of the Linux list **for a computer with no display server running**, on Linux.

*(Two-card citation check, 2026-08-06: the last sentence read "That is item B of the Linux list answered for Linux." Item B is worded "a computer with no screen attached at all", and that is not what was observed. What the evidence above shows is that **no display server was running** — `DISPLAY` empty, no socket, no X or Wayland process, session type `tty`. Whether a monitor is physically plugged into either card was never looked at and cannot be from here. The two are not the same claim, and the difference is not a quibble in one direction: a machine can have a display server and no monitor, in which case Blender's window opens perfectly well. **The condition that decides what Blender does is the one that was tested** — with no display server there is nothing for a window to open on, whether or not a screen is attached — so the practical half of the question is settled and the words have been corrected to the tested ones.)*

**What it does not settle.** A computer with **no graphics hardware at all**: this machine has two graphics cards, it simply has no display server on top of them. And the manual's line about headless *Windows* systems, which needs a Windows computer.

**Outside the interface.** Every run in this block was made from outside Blender's window, and on this machine there is no other way: with no display server there is no window to be inside. So this block speaks for the outside case alone, and none of the window tests the one-card machine ran could be repeated here. *(Two-card citation check, 2026-08-06: added by this check — the block carried neither standing line of its own, where the two blocks above it each carry both.)*

**Operating system.** Linux, on the machine named at the top of this block. It says nothing about macOS, where the project's only machine has a display server, and nothing about Windows, which no Windows machine has run. *(Two-card citation check, 2026-08-06: added by this check.)*


**Answer.** Yes for installing and switching on — Blender 5.0 ships a command-line interface for exactly this, and it forces the no-window mode itself. Giving an add-on its own settings is possible through Python, but the sources only document reading those settings, not a supported way to set them from the command line.

**Installing and enabling.**

> `-c, --command <command>`
>
> Run a command which consumes all remaining arguments. Use `-c help` to list all other commands. Pass `--help` after the command to see its help text.
>
> This implies `--background` mode.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

> `blender --command extension install [-h] [-s] [-e] [--no-prefs] PACKAGES`
>
> `-e, --enable` — Enable the extension after installation.

> `blender --command extension install-file [-h] -r REPO [-e] [--no-prefs] FILE`
>
> Install a package file into a user repository.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/extension_arguments.html — read 2026-07-30.

Enabling can also happen at launch, or from a script:

> `--addons <addon(s)>`
>
> Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

> For example, a script can enable add-ons for every user.
>
> `$BLENDER_SYSTEM_SCRIPTS/startup/enable_addons.py`
>
> ```
> def register():
>     import addon_utils
>     addon_utils.enable("my-addon")
> ```

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html — read 2026-07-30.

> `bpy.ops.preferences.addon_enable(*, module='')` — Turn on this add-on

Source: API documentation, https://docs.blender.org/api/5.0/bpy.ops.preferences.html — read 2026-07-30.

**No screen needed for the run itself.**

> One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH).

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — read 2026-07-30.

**"Whatever else it needs".** Add-ons may have their own settings:

> Some add-ons may have their own preferences which can be found in the Preferences section of the add-on information box.
>
> Some add-ons use this section for example to enable/disable certain functions of the add-on. Sometimes these might even all default to off. So it is important to check if the enabled add-on has any particular preferences.

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html — read 2026-07-30.

The documented way to reach them from Python is read access:

> ```
> # Access with:
> addon_prefs = bpy.context.preferences.addons[__package__]
> ```

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/addons.html — read 2026-07-30.

That the same handle can be written to and then made permanent with `bpy.ops.wm.save_userpref()` ("Make the current preferences default", https://docs.blender.org/api/5.0/bpy.ops.wm.html) is **inferred** — the step a reader has to supply is that an `AddonPreferences` property reached this way is writable and that saving preferences persists it. The sources do not say that outright.

One add-on does take its own command-line arguments, which is a supported way to configure it with no screen. Documented for Cycles:

> Cycles add-on options must be specified following a double dash.
>
> `--cycles-device <device>` — Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> `blender -b file.blend -f 20 -- --cycles-device OPTIX`

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

And **observed**, in the add-on's own file — it parses those arguments itself, from `init()`, which `register()` calls:

```
def _parse_command_line():
    import sys

    argv = sys.argv
    if "--" not in argv:
        return

    parser = _configure_argument_parser()
    args, _ = parser.parse_known_args(argv[argv.index("--") + 1:])
```
File: `Blender Sources/cycles_5.0/engine.py`, lines 24–32; `init()` calls `_parse_command_line()` at line 53 of the same file.

Also **observed**, and relevant to unattended setup — Cycles does not look for compute devices by itself at start-up:

```
    # Refresh device list. This does not happen automatically on Blender
    # startup due to unstable drivers that can cause crashes.
    def refresh_devices(self):
```
File: `Blender Sources/cycles_5.0/properties.py`, lines 1707–1710.

**LABEL:** **documented** for install and enable; **documented** for Cycles' own command-line options; **observed** for the Cycles code; **inferred** for setting an arbitrary add-on's preferences from a script.

**Not covered.** Whether an installed-and-enabled state persists into later runs without an explicit preferences save; whether `--command extension install` needs a writable user directory; whether the extension commands report failure through the process exit code; whether an add-on that needs a graphics device works with no screen.

**Outside the interface.** This whole answer is about outside the interface. The one caveat the sources give is that `--command` forces background mode, so these commands cannot be mixed with a normal windowed session in the same run.

**Operating system.** The extension commands and `--addons` are documented without operating-system restriction. Two nearby statements do differ by system: "no need for X server on Linux" (above), and, for a different subject, "Headless rendering is not supported on headless Windows systems." (EEVEE limitations, https://docs.blender.org/manual/en/5.0/render/eevee/limitations/limitations.html, read 2026-07-30). The second is about EEVEE rendering, not about installing add-ons.

---

## 4. What does an add-on record about the kinds of computer it is meant for, what happens on one it does not name, and does an add-on set up on one computer still work when copied to a computer of a different kind?

### Settled by hand test — what happens on a kind of computer an add-on does not name, and which bundled modules land on disk

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.0.1. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

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

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.0.1 from `/opt/blender/blender-5.0.1-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

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

**The new part: carrying an installed, set-up add-on to a computer of a different kind.** The whole settings folder holding the installed three-module add-on was packed on this Linux machine, copied to the Creator's MacBook Pro (M1 Pro chip, macOS 26.5.2), and that Mac's Blender 5.0.1 was pointed at it. The Mac listed it as installed:

```
Repository: "User Default" (id=user_default)
  smoozy_wheels [installed]: "Smoozy Wheels", A test add-on carrying bundled Python modules
```

and had it switched on:

```
VERSION 5.0.1
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

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 5.0.1, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`. Fresh settings folders, preferences saved once beforehand.

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


**Answer, what it records.** The manifest has an optional `platforms` list.

> `# # Optional: list of supported platforms. If omitted, the extension will be available in all operating systems.`
> `# platforms = ["windows-x64", "macos-arm64", "linux-x64"]`
> `# # Other supported platforms: "windows-arm64", "macos-x64"`

> `platforms:`
>
> List of supported platforms. If omitted, the extension will be available in all operating systems. The available options are `["windows-x64", "windows-arm64", "macos-x64", "macos-arm64", "linux-x64"]`

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/getting_started.html — read 2026-07-30.

It also records Blender versions (`blender_version_min`, optional `blender_version_max`) and may carry per-platform Python wheels.

**Answer, what happens elsewhere.** For remote repositories, the listing is filtered before the add-on is ever offered:

> When Blender fetches the extensions listing it passes the following arguments to make sure only compatible extensions are listed:
>
> `platform`
>
> `blender_version`

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/creating_repository/dynamic_repository.html — read 2026-07-30.

A package can also be built so that each platform gets its own file:

> `--split-platforms` — Build a separate package for each platform. Adding the platform as a file name suffix (before the extension).

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/extension_arguments.html — read 2026-07-30.

> Even though there is a Linux-only wheel present, no Linux zip file is generated. This happens because the `platforms` field only has Mac and Windows.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/python_wheels.html — read 2026-07-30.

What Blender does if a package naming other platforms is nonetheless installed from disk on this one is **not in the sources**. I searched the extensions pages, the preferences pages and the extension command-line page for `platform` and found only the statements above; none describes a refusal, a warning, or a silent acceptance.

**Answer, copying to a different kind of computer.** The general case is not established. The sources do say plainly that compiled parts are not portable:

> While not common practice, Python add-ons can be distributed with their own precompiled libraries. Unlike regular Python scripts, these are not portable between different platforms.
>
> It is possible the library is incompatible with your Blender installation (attempting to load a library built for a different version of Python, or loading a 32-bit library on a 64-bit system).
>
> If the add-on contains `.pyd` or `.so` files, check that the distribution is compatible with your operating system.

Source: manual, https://docs.blender.org/manual/en/5.0/troubleshooting/python.html — read 2026-07-30.

And an add-on's own identity is not the same string everywhere:

> The value of `__package__` will vary between systems so it should never be replaced with a literal string.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/addons.html — read 2026-07-30.

An add-on's default can also be platform-dependent. **Observed**, in Cycles:

```
    @staticmethod
    def default_device():
        import platform
        # Default to selecting the Metal compute device on Apple Silicon GPUs
        # (drivers are tightly integrated with macOS so pose no stability risk)
        if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):
            return 5
        return 0
```
File: `Blender Sources/cycles_5.0/properties.py`, lines 1572–1579.

**LABEL:** **documented** for `platforms`, for listing filtered by platform, for split builds, and for compiled libraries not being portable; **observed** for the Cycles default; **not in the sources** for what Blender does when a platform-restricted package is installed on a platform it does not name. The general result of copying a configured add-on to a different kind of computer was not established by the recorded search.

**Not covered.** Whether "kinds of computer" beyond the five named identifiers (for example a specific graphics card, or amount of memory) can be recorded at all — I found nothing; whether Blender checks `platforms` at enable time as well as at install time; whether an already-installed extension is disabled when the machine changes.

**Outside the interface.** The manifest and platform filtering are described in files and repository fetching, and `--command extension sync` and `list` are outside-interface routes. That the same filtering applies through those commands is **inferred**; the sources do not compare inside and outside use directly.

**Operating system.** This whole answer is the operating-system answer. The identifiers name Windows, macOS and Linux, with two processor families for Windows and macOS and one (`linux-x64`) for Linux.

---

## 5. How can an add-on be found, fetched and installed without a person at a screen, and what decides the places it can be fetched from?

### Settled by hand test — what the extension commands actually offer and print

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.0.1. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

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
    directory: "/Applications/Blender 5.0.1.app/Contents/Resources/5.0/extensions/system"
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

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.0.1 from `/opt/blender/blender-5.0.1-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

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
    directory: "/opt/blender/blender-5.0.1-linux-x64/5.0/extensions/system"
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

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 5.0.1, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`. A fresh settings folder, preferences saved once beforehand.

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


**Answer, finding and fetching.** The extension command line covers the whole cycle, and implies background mode (see question 3).

> `list:` List all packages.
> `sync:` Synchronize with remote repositories.
> `update:` Upgrade any outdated packages.
> `install:` Install packages.
> `install-file:` Install package from file.
> `remove:` Remove packages.

> `blender --command extension list [-h] [-s]`
>
> List packages from all enabled repositories.
>
> `-s, --sync` — Sync the remote directory before performing the action.

> `blender --command extension sync [-h]`
>
> Download package information for remote repositories.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/extension_arguments.html — read 2026-07-30.

**Answer, what decides the places.** Repositories, held in the preferences, and added or removed from the command line:

> `blender --command extension repo-add [-h] [--name NAME] [--directory DIRECTORY] [--url URL] [--access-token ACCESS_TOKEN] [--source SOURCE] [--cache BOOLEAN] [--clear-all] [--no-prefs] ID`
>
> `--url URL` — The URL, for remote repositories (optional). When omitted the repository is considered "local" as it is not connected to an external repository, where packages may be installed by file or managed manually.
>
> `--access-token ACCESS_TOKEN` — The access token to use for remote repositories which require a token.
>
> `--source SOURCE` — The type of source in ('USER', 'SYSTEM'). System repositories are managed outside of Blender and are considered read-only.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/extension_arguments.html — read 2026-07-30.

What is there to begin with:

> By default Blender has a Remote Repository pointing towards the Official Blender Extensions Platform and two Local Repositories.

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/extensions.html — read 2026-07-30. The screenshot on that page, `_images/editor_preferences_section_extensions_repositories.png`, shows three rows — "extensions.blender.org" (globe icon, ticked and selected), "User Default" and "System" — with the URL field beneath reading `https://extensions.blender.org/api/v1/extensions/` and a ticked "Check for Updates on Startup". **documented**, read 2026-07-30.

A repository can also point at a plain file path rather than a web address:

> `remote_url` — Remote URL to the extension repository, the file-system may be referenced using the file URI scheme: "file://"

Source: API documentation, https://docs.blender.org/api/5.0/bpy.types.UserExtensionRepo.html — read 2026-07-30. The manual gives the shapes: `file:///path/to/packages/index.json` on Linux and macOS, `file:///C:/path/to/packages/index.json` on Windows, `file://HOST/share/path/to/packages/index.json` for network shares on Windows (https://docs.blender.org/manual/en/5.0/advanced/extensions/creating_repository/static_repository.html).

Two further gates:

> By default Blender does not access the internet, however this can be enabled in the System preferences with the Online Access option.
>
> Working offline can be enforced by running with the `--offline-mode` command line argument.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html — read 2026-07-30. The matching switches are `--online-mode` ("Allow internet access, overriding the preference") and `--offline-mode` ("Disallow internet access, overriding the preference") on https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html.

And, at the serving end, a repository can refuse to offer particular packages:

> This can be used to defined blocked extensions, for example
>
> ```
> schema_version = "1.0.0"
>
> [[blocklist]]
> id = "my_example_package"
> reason = "Explanation for why this extension was blocked"
> ```

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/extension_arguments.html — read 2026-07-30.

**LABEL:** **documented** throughout.

**Not covered.** What `extension list` actually prints (columns, whether it marks enabled state); whether `install` accepts a bare URL as well as a package name; how a failure is signalled to the calling script; whether the online-access preference gates the extension commands themselves as well as add-on code.

**Outside the interface.** This is the outside-the-interface answer. The interface routes described alongside it — dragging an installation URL into Blender, drag-and-drop of a `.zip` — have no command-line equivalent stated, but `install-file` covers installing from a file.

**Operating system.** The only operating-system differences stated are in the `file://` URL forms above. Everything else is written without distinction.

---

## 6. What does an add-on belong to, once it is installed on a computer?

### Settled by hand test — where an installed add-on sits, and whether it belongs to one Blender

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.0.1. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

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
LOCAL /Applications/Blender 5.0.1.app/Contents/Resources/5.0
SYSTEM /Library/Application Support/Blender/5.0
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

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.0.1 from `/opt/blender/blender-5.0.1-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

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
LOCAL /opt/blender/blender-5.0.1-linux-x64/5.0
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
VERSION 5.0.1
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

The shared folder grew no version-named subfolder. Nothing in Blender refused another version's add-on. The same as the macOS run.

**A mismatched package's folder** dropped straight into the extensions directory is listed but cannot be switched on — see question 4.

**Outside the interface.** Settled again on a second operating system: every path above was read, and the shared-folder test run, with Blender driven from outside its own window.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed; only the paths themselves differ, which is the rule working rather than a difference. Windows stays untested.


**Answer.** A repository, and through it a directory; and its name becomes part of a namespace that includes the repository.

> To prevent this conflict, the repository name is now part of the namespace.
>
> For example, now instead of `kitsu` the module name would be `bl_ext.{repository_module_name}.kitsu` instead.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/addons.html — read 2026-07-30.

Repositories are of two sources:

> `source` — Select if the repository is in a user managed or system provided directory
>
> `USER` — User – Repository managed by the user, stored in user directories.
>
> `SYSTEM` — System – Read-only repository provided by the system.

Source: API documentation, https://docs.blender.org/api/5.0/bpy.types.UserExtensionRepo.html — read 2026-07-30.

The directories are per Blender version, and split user from system:

> `./extensions`
>
> Extension repositories.
>
> Located in both user and system directories. Repositories are loaded from both directories.

with user directories being `$HOME/.config/blender/5.0/` (Linux), `/Users/$USER/Library/Application Support/Blender/5.0/` (macOS), `%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.0\` (Windows), and system directories `./5.0/`, `./Blender.app/Contents/Resources/5.0/`, `%ProgramFiles%\Blender Foundation\Blender\5.0\` and so on.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html — read 2026-07-30.

Installing from a file puts the add-on somewhere specific:

> Install from Disk
>
> Install an extension from a `.zip` package. This is installed to a Local Repository and no updates will be available.

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/extensions.html — read 2026-07-30.

The source also describes a directory tied to an extension's lifetime. The conclusion that this is a place of the add-on's own follows from the cited function name and is **inferred**; the quoted words call it only "this directory":

> This directory will be kept between upgrades but will be removed if the extension is uninstalled.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/addons.html — read 2026-07-30, describing `bpy.utils.extension_path_user(__package__, path="", create=True)`.

> Add-ons must not assume their own directory is user writable since this may not be the case for "System" repositories. Writing files into the add-on's directory also has the down side that upgrading the extension will remove all files.

Same page, same date.

An older kind of add-on belongs somewhere else again:

> Add-ons that have not been converted to become an extension yet need to be placed in the `addons` script directory.
>
> For example, an add-on could be located at:
>
> `$BLENDER_SYSTEM_SCRIPTS/addons/simple_addon.py`

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html — read 2026-07-30.

**LABEL:** **documented** for repositories, namespaces, versioned directories and the old add-on directory; **inferred** for the description of the extension-owned directory from the function name and the quoted lifetime statement.

**Not covered.** Whether an add-on belongs to a particular user account in any stronger sense than living in that account's directory; whether the same add-on can be present in two repositories at once and what then happens; whether "belongs to" in the sense of ownership or licence entitlement is recorded anywhere (it is not, in these sources).

**Outside the interface.** `BLENDER_USER_RESOURCES`, `BLENDER_USER_EXTENSIONS`, `BLENDER_SYSTEM_EXTENSIONS` and `BLENDER_SYSTEM_SCRIPTS` are documented as environment variables. That these let placement be steered without a person clicking is **inferred** from their documented purpose.

**Operating system.** The user and system directory paths differ by operating system, quoted above. The portable-installation folder differs too: next to the executable on Windows and Linux, "Inside the application bundle at Blender.app/Contents/Resources" on macOS.

---

## 7. What does a render do when an add-on it needs is not there?

### Settled by hand test — what a render does when the add-on behind its engine is not there

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.0.1. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

This is the question all four versions' documents record as *not in the sources*. It is now answered, three ways.

**A file whose engine is Cycles, rendered with Cycles switched off.** A file was saved with the engine set to Cycles, one sample, 64 by 64, and rendered once with Cycles on to prove it worked — it wrote `ok_0001.png`, exit status 0. Cycles was then switched off and its preferences saved. The same file was rendered again:

`blender --background cycles.blend --render-output <folder>/off_ --render-frame 1`

The render printed:

```
ERROR Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
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

Exit status 1. No image was written.

**A file whose engine belongs to an add-on that was never installed.** A test add-on supplying an engine called `SMOOZY_TEST` was installed, a file saved with that engine, and one frame rendered — it wrote an image, exit status 0. The same file was then rendered in a settings folder where that add-on had never existed:

```
ERROR Engine 'SMOOZY_TEST' not available for scene 'Scene' (an add-on may need to be installed or enabled)
```

followed by the same EEVEE render and `Saved: '<folder>/without_0001.png'`, exit status 0. Opening that file in the same bare Blender and asking for its engine printed the same message and then `ENGINE BLENDER_EEVEE`.

**An add-on that changes a render without being an engine.** A test add-on that adds a function to `bpy.app.handlers.render_pre` was installed, and one frame rendered: the render printed the add-on's own line `SMOOZY HANDLER RAN`. Rendering the same file where the add-on had never been installed printed **no message at all** about it, finished normally and exited 0.

So: a missing engine add-on is reported on the terminal, the render falls back to EEVEE, an image is written and the exit status is 0. A missing add-on of any other kind is not reported at all. Only forcing the engine with `-E` stops the render, and that exits 1.

**Outside the interface.** Settled: the message goes to the terminal, and there is no dialog and no prompt. What a window shows instead was not tested here.

**Operating system.** Settled for macOS only, on the one machine described above. The same tests were run again on a rented Linux machine, and that result has its own block under this question; the two are never merged. Windows stays untested.

### Settled by hand test on the rented Linux machine — what a render does when the add-on behind its engine is not there

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.0.1 from `/opt/blender/blender-5.0.1-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

**A file whose engine is Cycles, rendered with Cycles switched off, renders anyway.** A 64 by 64, one-sample Cycles file was saved and rendered once with Cycles on, writing `ok_0001.png`. Cycles was then switched off and the same file rendered again:

```
00:00.223  reports          | ERROR Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
00:01.081  render           | Fra: 1 | Rendering 1 / 64 samples
00:01.250  render           | Fra: 1 | Rendering 64 / 64 samples
Saved: '<output folder>/off_0001.png'
```

**Exit status 0.** The middle of the render's progress lines is cut above; the first and last are kept. An image was written — `off_0001.png` against `ok_0001.png`, a few hundred bytes apart, both around 3 to 5 kB — and nothing in the exit status says the wrong engine was used. The same as the macOS run.

**Forcing the engine on the command line writes no image.** `blender --background cycles.blend -E CYCLES --render-output ... --render-frame 1` printed the same "not available" message and then:

```
Error: engine not found 'CYCLES'
```

with exit status 1. No `forced_0001.png` appeared in the output folder. On macOS this run ended in an abort in 4.5 and exit 1 in the others; here 4.5 hangs instead of aborting, and the others exit 1 as they did there.

**A file whose engine belongs to an add-on that was never installed behaves the same way.** A test add-on supplying an engine called `SMOOZY_TEST` was built and installed; `blender -E help` then listed it beside the built-in engines. A file was saved with that engine and rendered, writing `with_0001.png`. Rendering the same file in a settings folder where the add-on had never been installed printed:

```
Error: Engine 'SMOOZY_TEST' not available for scene 'Scene' (an add-on may need to be installed or enabled)
```

and then rendered and wrote `without_0001.png` anyway, exit status 0. Asking that bare Blender for the file's engine gave the same message and then `ENGINE BLENDER_EEVEE`.

**An add-on that changes a render without being an engine is completely silent when absent.** The render printed no message at all about it, finished, and exited 0. Both images measured 128 by 128.

Every one of these agreed with the macOS run, except the way Blender 4.5 ends after `-E CYCLES` fails.

**Outside the interface.** Settled again on a second operating system: every render above was started from outside Blender's window, and the messages went to the terminal.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed on what a render does; they differ only in how Blender 4.5 ends after a forced engine is not found — an abort on macOS, a hang on Linux. Windows stays untested.


**Answer.** **Not in the sources.** No page in the 5.0 manual, API documentation or release notes describes what a render does when a needed add-on is absent — not for a missing render engine, not for missing settings, not for missing node or modifier types.

The nearest documented statements, none of which answers the question:

> If the Add-on does not activate when enabled, check the Console window for any errors that may have occurred.

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html and the same tip on .../editors/preferences/extensions.html and .../advanced/extensions/addons.html — read 2026-07-30. This is about enabling failing, not about a render meeting an absence.

> Any installed extension can be removed. This is a permanent change, though. To stop an extension temporarily, it is better to Disable it instead.

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/extensions.html — read 2026-07-30. It says nothing about files that were using it.

> Removing these properties from the deprecated 'dict-like' user properties in IDs etc. will fully break forward compatibility. In other words, these blendfiles, if opened again in older versions of Blender, will only have default values for all the add-on's defined properties.

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/python_api/ — read 2026-07-30. This is about older Blender versions and a deprecated storage route, not about a missing add-on.

**Where I looked.** Searched the whole manual, API and release notes for `missing`, `not installed`, `not found`, `not registered`, `undefined`, `unknown engine`, `engine not`, `fallback`, `add-on is missing`, `missing extension`. I read `troubleshooting/python.html`, `troubleshooting/crash.html`'s neighbours in `troubleshooting/`, `render/introduction.html`, `editors/preferences/addons.html`, `editors/preferences/extensions.html`, `advanced/extensions/addons.html`, `advanced/command_line/render.html`, `advanced/command_line/arguments.html`, and the `RenderEngine` and `RenderSettings` API pages. None of them states a behaviour for this case.

**LABEL:** not in the sources.

**Not covered.** Everything about the actual behaviour: whether the render stops, substitutes another engine, renders without the add-on's contribution, prints a message, or returns a particular exit code.

**Outside the interface.** The sources do not say. In particular, nothing says whether a background render signals this condition through the process exit code.

**Operating system.** The sources do not say.

---

## 8. Where do add-ons come from besides Blender's own, how are they sold, and what does that mean for taking a copy of one from one computer to another?

### Settled by hand test — where packages come from, and what a copy installed from a file can do

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.0.1. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

**What is set up to begin with.** `blender --command extension repo-list` in a fresh settings folder printed three entries — `extensions.blender.org` with the address `https://extensions.blender.org/api/v1/extensions/` and no access token, `User Default` marked `source: "USER"`, and `System` marked `source: "SYSTEM"` and pointing inside the application at `/Applications/Blender 5.0.1.app/Contents/Resources/5.0/extensions/system`.

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

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.0.1 from `/opt/blender/blender-5.0.1-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

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

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 5.0.1, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`.

**An add-on's opened row shows no licence and no repository name.** The two rows opened — Cycles Render Engine and Node Wrangler — carry a website, a type reading `Core`, and for one of them a maintainer and a version. Neither carries a licence, a repository, a tag or a declared permission. So a person cannot tell from a row what they are allowed to do with a copy. Both add-ons ship with Blender, so what a row shows for an add-on installed from a repository is still unsettled here, exactly as it was on the Mac.

**The only setting on the repositories side bearing on getting at an add-on is an access token.** Read from the preferences rather than from the drop-down, a repository carries `access_token` and `use_access_token` among its settings, and all three repositories in a fresh settings folder have `use_access_token` false. Full list under question 5.

**Outside the interface.** This is a window test. Nothing here changes what is reachable from outside the window.

**Operating system.** Settled for Linux in this version, at the window. The Creator's macOS run covered Blender 5.2 only. Nothing here bears on how add-ons are sold.


**Answer, where from.**

> The official extensions platform for the Blender project is extensions.blender.org. Other third party sites can also be supported, as long as they follow the Extensions Platform specification.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/index.html — read 2026-07-30.

> Third party sites that wish to support extensions in Blender can do so in several ways, in order of complexity:
>
> Static: Host a static JSON file listing all the packages of your repository.
>
> Dynamic: Serve the JSON file on-demand based on the Blender version and platform.
>
> Platform: Fork the entire Extensions Website to create your own.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/creating_repository/index.html — read 2026-07-30.

> To host your own extensions and leverage Blender update system all that is required is to host a static JSON file that is generated by Blender.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/creating_repository/static_repository.html — read 2026-07-30.

There is also the route with no site at all — `Install from Disk` from a `.zip`, and the bundled-with-Blender route:

> These repositories are intended to bundle extensions with Blender, to make it portable.

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/extensions.html — read 2026-07-30.

**Answer, how they are sold.** **Not in the sources.** Nothing in the 5.0 manual, API documentation or release notes describes selling, pricing, purchase, licensing keys, entitlements or subscriptions for add-ons. Searched for `sell`, `sold`, `purchase`, `price`, `paid`, `commercial`, `market`, `store`, `donat` across the manual's `advanced/` and `editors/preferences/` trees and the release notes; the only hits were unrelated words.

What the sources do state about terms is the licence rule of the official platform:

> The Blender Extensions Platform only supports free and open source extensions compliant with the Blender's license:
>
> For add-ons, the required license is GNU General Public License v3.0 or later.
>
> For themes, the recommended license is GNU General Public License v3.0 or later, but any GPL-compatible license is supported.
>
> For assets used in add-ons, the required license is Public Domain (CC0).

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/licenses.html — read 2026-07-30.

The manifest carries a licence field per package: `license = ["SPDX:GPL-3.0-or-later",]`, described as "List of licenses, use SPDX license identifier" (https://docs.blender.org/manual/en/5.0/advanced/extensions/getting_started.html).

Access to a repository can be gated by a token, which is the only access-control mechanism described:

> Some repositories may require authentication. The user can specify an access token for a repository, which is passed along with the API request from Blender.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/creating_repository/dynamic_repository.html — read 2026-07-30.

**Answer, moving a package between computers.** The sources describe installing a package file and using a shared system repository. They do not establish that copying an already installed add-on carries its setup, licence entitlement or right to use it. The documented routes warn about updates and compiled parts.

> Install from Disk — Install an extension from a `.zip` package. This is installed to a Local Repository and no updates will be available.

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/extensions.html — read 2026-07-30.

For the many-computers case the manual points at a shared system repository:

> When working offline or in a more controlled environment, it may be useful to provide a set of extensions to all users. For this there is a default read-only System repository. This repository could be located on a read-only network drive or in a system directory.
>
> Extensions packages should be extracted in this system directory, with a resulting path like this:
>
> `$BLENDER_SYSTEM_EXTENSIONS/system/my-addon/blender_manifest.toml`

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html — read 2026-07-30.

And the caution from question 4 applies: precompiled libraries "are not portable between different platforms".

**LABEL:** **documented** for where add-ons come from, installing a package file, and using a shared system repository; **not in the sources** for how add-ons are sold or what sale and entitlement mean for moving a copy between computers.

**Not covered.** Whether a paid add-on's entitlement can be carried between computers; whether any licence-checking mechanism is supported by Blender (nothing describes one); whether copying an installed add-on directory, as opposed to installing its `.zip`, is supported.

**Outside the interface.** Every route here has a command-line form except drag-and-drop: `--command extension install`, `install-file`, `repo-add --url`, and the `$BLENDER_SYSTEM_EXTENSIONS` directory that needs no Blender interaction at all.

**Operating system.** The `file://` URL forms differ by system (question 5). The system-extensions directory is described with one shape for all systems. Compiled parts inside an add-on are platform-specific.

---

## 10. Can a computer be asked which add-ons it holds, and for which Blender?

### Settled by hand test — what can be asked of a computer about its add-ons

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.0.1. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

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
PATHS ['/Applications/Blender 5.0.1.app/Contents/Resources/5.0/scripts/addons_core']
COUNT 13
```

followed by one line per add-on with its whole metadata block. The 13 are `io_anim_bvh`, `bl_pkg`, `cycles`, `io_scene_fbx`, `io_scene_gltf2`, `hydra_storm`, `ui_translate`, `node_wrangler`, `pose_library`, `rigify`, `io_curve_svg`, `io_mesh_uv_layout` and `viewport_vr_preview` — every add-on that ships with Blender, switched on or not. With the test extension installed, `bl_ext.user_default.smoozy_test` appeared in that list too, so this route covers both kinds. `addon_utils` is the closest thing to an inventory that was found.

**The system-information report can be written with no window, and it names versions and paths.** `blender --background --python-expr "import bpy; print('SYSINFO', bpy.ops.wm.sysinfo(filepath='system-info.txt'))"` printed `SYSINFO {'FINISHED'}`, exit status 0, and wrote the file with no prompt of any kind. Its add-on section reads:

```
Enabled add-ons:
====================================

io_anim_bvh (version: (1, 0, 1), path: '/Applications/Blender 5.0.1.app/Contents/Resources/5.0/scripts/addons_core/io_anim_bvh/__init__.py')
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

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.0.1 from `/opt/blender/blender-5.0.1-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

**What is switched on.** `blender --background --python-expr "import bpy; a=bpy.context.preferences.addons; print('COUNT', len(a)); print('MODULES', sorted(x.module for x in a))"` printed:

```
COUNT 8
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

The same eight as on macOS. That collection drops an add-on the moment it is switched off: after switching the test add-on off and saving the preferences, its module was gone from this list while `blender --command extension list` still printed `smoozy_test [installed]`. Both machines agreed.

**What is on the machine at all.** `blender --background --python-expr "import addon_utils; print('PATHS', addon_utils.paths()); m=list(addon_utils.modules()); print('COUNT', len(m)); ..."` printed:

```
PATHS ['/opt/blender/blender-5.0.1-linux-x64/5.0/scripts/addons_core']
COUNT 13
```

and then one line per add-on with its whole metadata block. The add-ons named were `io_anim_bvh`, `bl_pkg`, `cycles`, `io_scene_fbx`, `io_scene_gltf2`, `hydra_storm`, `ui_translate`, `node_wrangler`, `pose_library`, `rigify`, `io_curve_svg`, `io_mesh_uv_layout`, `viewport_vr_preview` — the same set, in the same order, that the macOS run recorded for this version.

**The system-information report can be written with no window and with no prompt.** `blender --background --python-expr "import bpy; print(bpy.ops.wm.sysinfo(filepath='system-info.txt'))"` printed `{'FINISHED'}`, exit status 0, and the file's "Enabled add-ons" section read:

```
Enabled add-ons:
====================================

io_anim_bvh (version: (1, 0, 1), path: '.../addons_core/io_anim_bvh/__init__.py')
io_curve_svg (version: UNKNOWN, path: '.../addons_core/io_curve_svg/__init__.py')
io_mesh_uv_layout (version: (1, 2, 0), path: '.../addons_core/io_mesh_uv_layout/__init__.py')
io_scene_fbx (version: (5, 14, 0), path: '.../addons_core/io_scene_fbx/__init__.py')
io_scene_gltf2 (version: (5, 0, 21), path: '.../addons_core/io_scene_gltf2/__init__.py')
cycles (version: UNKNOWN, path: '.../addons_core/cycles/__init__.py')
pose_library (version: (2, 0), path: '.../addons_core/pose_library/__init__.py')
bl_pkg (version: (0, 0, 1), path: '.../addons_core/bl_pkg/__init__.py')
```

The paths above are cut to their last two parts; each is the full path inside this machine's Blender folder. It names each add-on's version and path, with `UNKNOWN` where the metadata block carries none — Cycles among them. The same as the macOS run.

**Which Blender the answer belongs to is a matter of the folder alone.** One settings folder handed to all four Blenders in turn gave each of them the same installed add-on — see question 6.

**Outside the interface.** Settled again on a second operating system: all three routes, and the system-information report, are reachable with Blender driven from outside its own window and with no prompt of any kind.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed on the routes, the switched-on set and the bundled set. Windows stays untested.

### Settled by hand test on the rented Linux machine — what the Add-ons list and an opened row show

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 5.0.1, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`.

**The Add-ons list holds the add-ons that ship with Blender, and nothing else.** The picture of the list in a fresh settings folder shows, in order: BioVision Motion Capture (BVH) format, Cycles Render Engine, FBX format, glTF 2.0 format, Hydra Storm render engine, Manage UI translations, Node Wrangler, Pose Library, Rigify, Scalable Vector Graphics (SVG) 1.1 format, UV Layout, VR Scene Inspection. Each has a tick box, and the ones ticked are the ones the no-window run lists as switched on. The extensions system itself, `bl_pkg`, is in that switched-on list but has no row of its own.

**An opened row shows a version and a maintainer for one add-on and neither for Cycles.** Node Wrangler's row carries `Maintainer` with five names and `Version 3.55`; Cycles' row carries neither. That matches the `UNKNOWN` the system-information report gives for Cycles from outside the window. Neither row shows a repository name or a location on disk. The same as the Creator's macOS run in Blender 5.2.

**Outside the interface.** This is a window test. The matching no-window results are under question 10 above, and the two agree on which add-ons are there and which are switched on.

**Operating system.** Settled for Linux in this version, at the window. The Creator's macOS run covered Blender 5.2 only. Windows stays untested.


**Answer, which add-ons.** The sources document two partial lists, but not a complete inventory of every add-on held on the computer. The command line lists packages from enabled repositories; Python lists enabled add-ons.

> `blender --command extension list [-h] [-s]`
>
> List packages from all enabled repositories.

> `blender --command extension repo-list [-h]`
>
> List all repositories stored in Blender's preferences.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/extension_arguments.html — read 2026-07-30.

From Python, the enabled add-ons are a collection on the preferences:

> `addons`
>
> Type: `Addons` `bpy_prop_collection` of `Addon`, (readonly)

Source: API documentation, https://docs.blender.org/api/5.0/bpy.types.Preferences.html — read 2026-07-30. And `bpy.types.Addon` is described as "Python add-ons to be loaded automatically", with a `module` ("Module name") and a `preferences` member (https://docs.blender.org/api/5.0/bpy.types.Addon.html).

Where they live on disk can also be asked:

> You can also find add-on path locations by running this in the Python Console:
>
> ```
> import addon_utils
> print(addon_utils.paths())
> ```

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/scripting/addon_tutorial.html — read 2026-07-30.

`bpy.utils.user_resource(resource_type, ...)` takes `'EXTENSIONS'` among its resource types, and `bpy.utils.script_paths(...)` returns "a list of valid script paths" (https://docs.blender.org/api/5.0/bpy.utils.html).

**Answer, for which Blender.** Three separate things carry the version:

1. The directories are per version — `.../blender/5.0/`, `.../Blender/5.0/`, `./5.0/` — so what is installed is already sorted by Blender version (https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html).
2. Each package declares its range: `blender_version_min` ("Minimum supported Blender version - use at least 4.2.0") and `blender_version_max` ("Blender version that the extension does not support, earlier versions are supported") — https://docs.blender.org/manual/en/5.0/advanced/extensions/getting_started.html.
3. Repositories are asked with the running version as a parameter: `blender_version` (https://docs.blender.org/manual/en/5.0/advanced/extensions/creating_repository/dynamic_repository.html).

That an installation therefore belongs to exactly one Blender version's directory is **inferred** from point 1 — the step being that a version-numbered folder holds that version's extensions.

Neither quote says that disabled legacy add-ons, disabled extensions, and packages merely offered by a remote repository can be separated into one complete installed list. The pages and whole-document searches recorded above produced no complete inventory route.

**LABEL:** **documented** for the two partial listing routes and for the version fields; **not in the sources** for a complete list of add-ons installed on the computer; **inferred** for the conclusion that the answer is naturally per Blender version.

**Not covered.** The exact output of `extension list` — whether it names the repository, the version, or the enabled state; whether it lists legacy add-ons as well as extensions; whether it works with no repositories enabled; whether a computer can be asked about a Blender version other than the one being run.

**Outside the interface.** Both routes are outside-the-interface routes: `--command` forces background mode, and the Python collection is readable from a script.

**Operating system.** Only the directory paths differ, as quoted in question 6.

---

## 11. Can an add-on depend on something else being on the computer, or on a computer elsewhere, before it will work, and can what it needs be read anywhere?

### Settled by hand test — what can be read about what an add-on needs

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.0.1. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it.

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

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.0.1 from `/opt/blender/blender-5.0.1-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

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

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 5.0.1, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`.

**No declared permission, no bundled module and no dependency of any kind appears in an add-on's opened row.** The two rows opened carry a website, a type, and for one of them a maintainer and a version. The only field naming anything outside Blender is the website address. Neither add-on opened declares a permission in the first place, so this settles what those two rows show and nothing more — the same limit the Creator's macOS run had.

**The one add-on setting on screen that names something outside Blender is Cycles' own device row**, `None | CUDA | OptiX | HIP | oneAPI`, which names kinds of graphics hardware rather than anything the add-on requires. The System preferences also carry a **Network** section with an **Allow Online Access** tick box, unticked in a fresh settings folder, which is the preference the online-access property reads.

**Outside the interface.** This is a window test. The matching no-window results are under question 11 above.

**Operating system.** Settled for Linux in this version, at the window. The Creator's macOS run covered Blender 5.2 only. Windows stays untested.


**Answer, something else on the computer.** For add-ons bundled as extensions, the stated rule is that the package must carry its dependencies. The sources do not establish whether every kind of add-on can require a separately installed program or library.

> For add-ons to be bundled as extensions, they must be self-contained. That means they must come with all its dependencies. In particular 3rd party Python modules.
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
> Bundle with Vendorize
>
> This can be used as a way to bundle a pure Python dependencies as a sub-module.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/addons.html — read 2026-07-30.

> Wheels must include their dependencies.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/python_wheels.html — read 2026-07-30.

Note the second option: an add-on may depend on another add-on, and the answer given is to ship both together, with a caution:

> Make sure that both the individual and the combined add-on check for already registered types (Operators, Panels, …). This avoids duplication of operators and panels on the interface if the add-ons are installed as a bundle and individually.

Same page, same date.

There is no field in the manifest that names another extension as a requirement. The full list of manifest values on https://docs.blender.org/manual/en/5.0/advanced/extensions/getting_started.html is `schema_version`, `id`, `version`, `name`, `tagline`, `maintainer`, `type`, `blender_version_min`, `blender_version_max`, `website`, `license`, `copyright`, `tags`, `platforms`, `wheels`, `permissions`, and a `[build]` table — none of which names another extension.

**Answer, a computer elsewhere.** The sources document that an add-on may request network access and give a remote server as an example. They require an add-on using the internet to check whether online access is allowed. They do not state that this records a required remote computer, or what happens when that computer is unavailable.

> `# # Optional: add-ons can list which resources they will require:`
> `# # * files (for access of any filesystem operations)`
> `# # * network (for internet access)`
> `# # * clipboard (to read and/or write the system clipboard)`
> `# # * camera (to capture photos and videos)`
> `# # * microphone (to capture audio)`
>
> `# [permissions]`
> `# network = "Need to sync motion-capture data to server"`

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/getting_started.html — read 2026-07-30.

> If the add-on needs to use internet, it must check for the (read-only) property `bpy.app.online_access`. This option is controlled by Preferences, which can be overriding via command-line (`--offline-mode` / `--online-mode`).

> Add-ons that follow this setting will only connect to the internet if enabled. However, Blender cannot prevent third-party add-ons from violating this rule.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/addons.html — read 2026-07-30.

> `bpy.app.online_access`
>
> Boolean, true when internet access is allowed by Blender & 3rd party scripts (read-only).
>
> `bpy.app.online_access_override`
>
> Boolean, true when internet access preference is overridden by the command line (read-only).

Source: API documentation, https://docs.blender.org/api/5.0/bpy.app.html — read 2026-07-30. (The manual on the same subject spells the second one `bpy.app.online_access_overriden`; the API page spells it `online_access_override`. I report both as read; the sources do not reconcile them.)

**Answer, can what it needs be read anywhere.** Partly. The manifest travels with the package and records declared wheels, permissions, platforms and Blender-version range. The sources do not present it as a complete list of everything the add-on needs.

> A manifest is a file with all the meta-data required for an extension to be processed.

> An extension is shared as a `.zip` archive containing a manifest file and other files.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/getting_started.html — read 2026-07-30.

The package can be validated without unpacking:

> You may also validate a package without having to extract it first.
>
> `blender --command extension validate add-on-package.zip`

Same page, same date.

And it sits beside the installed add-on on disk — the deployment page's own example path is `$BLENDER_SYSTEM_EXTENSIONS/system/my-addon/blender_manifest.toml` (https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html).

Third, a repository listing is generated from the package files: `server-generate` "creates an `index.json` listing from all the `.zip` extensions found in the –repo-dir location", and the download-link parameters "can be extracted from the extensions manifest" (https://docs.blender.org/manual/en/5.0/advanced/extensions/creating_repository/static_repository.html). The quoted words do not say that the listing carries every manifest fact.

Reading the manifest exposes its declared *wheels, permissions, platforms and Blender-version range*. The quoted sources do not say that these are most or all of what the add-on needs. The listed fields include no field naming another extension.

**LABEL:** **documented** for extension self-containment, wheels, bundling, permissions and online access, for the manifest being inside the package and beside an installed system extension, and for package validation without extraction; **not in the sources** for any manifest field that names another extension as a prerequisite, for a complete readable requirements list, or for whether a separately installed program or remote computer can be required.

**Not covered.** Whether Blender enforces the `[permissions]` declarations at run time (the manual says only that it "cannot prevent third-party add-ons from violating this rule" for network); whether an add-on can require a system-installed program or library outside Python; what happens at enable time if a wheel for the running platform is absent; whether the manifest of an *installed* extension can be read through the Python API rather than off disk.

**Outside the interface.** The documented package checks and online-access controls hold outside the interface: `--command extension validate` runs in background mode, the manifest is a file, and `--offline-mode`/`--online-mode` are command-line switches that override the preference. The unresolved dependency cases remain unresolved outside the interface too.

**Operating system.** Wheels are chosen per platform and per Python version, so what an add-on needs on disk differs by operating system (see the `pillow` example on the Python Wheels page). The manifest, the permissions table and the online-access rule are stated without operating-system distinction.

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

2026-08-04 — Opus 5 (Claude Code) — Add-ons, Blender 5.0
