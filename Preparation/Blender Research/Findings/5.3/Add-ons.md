- **Sources consulted, and dates.** The Blender 5.3 manual (with its pictures), the Blender 5.3 Python API documentation and the Blender 5.3 release notes, all from copies taken 2026-10-11, which is the date read for everything quoted from them. Cycles' own files for 5.3, from a copy taken 2026-10-16, which is the date read for everything quoted from them. All of these are inside `Blender Sources/`. The work was done on 2026-10-16.
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

# Add-ons — Blender 5.3 findings

**Notes on that declaration, for full honesty.**
- Inside `Blender Sources/` I also opened the folder's own note, `About this folder.md`. I did not follow the link in it. I did not open `requirements/` or any folder for another Blender version.
- Before I started, the tool I run in put some text into my view without my opening anything: the text of the project's `CLAUDE.md` (a signpost telling agents to read a planning file — I did not open that file), a list of titles of saved memory notes, and the repository's recent commit messages and changed file names. None of that text says anything about how Blender handles add-ons, and nothing below comes from it.
- One search produced a long result that the tool saved in its own results file. I did not open that file. I ran the search again into my notes folder (`/private/tmp/claude-501/smoozy-research-5.3/Add-ons/`) and read it there.
- I ran nothing. No Blender was started. The only things I "examined" rather than read were Cycles' own files, and those findings are labelled **observed**.
- The release notes front page says: "Blender 5.3 is currently in **Beta** until November 4, 2026." — https://developer.blender.org/docs/release_notes/5.3/, read 2026-10-11. Everything below describes the documentation in that state.

**How quotes are written.** The text copies break sentences across lines; I join those lines with a space. Where a quote joins a heading or a setting's name to the text under it (for example "Refresh Local — Scan extension …"), the dash between them is mine. "…" marks words left out. Every other character is the source's.

**How I searched, for every question.** I searched the whole of the 5.3 manual, API documentation and release notes at once for "add-on", "addon" and "add-ons" (373 matching lines in the manual, 218 in the API documentation, all reviewed page by page), and for "extension", "manifest", "repository", "wheel" and "platform". Each answer adds the extra terms I searched for that question.

## How the hand tests were run (2026-10-17)

*Added by the hand-test step, not by the researcher.* Every section below headed **Observed — …** records a test run on **2026-10-17** by the agent that owns the hand-test list, on **the Creator's Mac: a MacBook Pro with an Apple M1 Pro chip, its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2**, in **Blender 5.3.0 Beta**, hash `5d8151335b6b`, built 2026-10-16, at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`. **Each result is a macOS result on Apple silicon and nothing more: no test was run on Linux or Windows.** The full list, with every command and every script written out, is `Audits/Blender Research/Add-ons/Hand tests — 5.3.md`; each section names its entry.

What holds for every run unless a section says otherwise:

- Blender was started from a terminal, **with no window**, as `--background` or `--command`.
- Each entry pointed Blender at **a new, empty settings folder of its own** with the `BLENDER_USER_RESOURCES` environment variable, and before anything else Blender was asked where its user, configuration, extensions and scripts folders were; all four printed back inside the test's work folder every time. **Once that was not enough**: the first attempt at entry 14 part D pointed at two settings folders that had not been created, the check passed on the printed paths alone, and Blender saved a preferences file into the default 5.3 user folder on this Mac — the Creator's own — which was then moved out. That is recorded as an observed result under question 6. From then on the check also required the folder to exist. Apart from that, the Creator's own Blender settings were never read or changed.
- Blender's temporary folder was pointed at the work folder with `TMPDIR`.
- In the commands, `blender` stands for the program above. In commands **and in outputs**, `<work>` stands for the work folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Add-ons`; that is the only substitution made in any quoted output. Output is otherwise word for word, except where a section says lines were left out, with what Blender printed on its ordinary output and its error output together unless a section says they were kept apart. Taken together, the two can interleave out of the order they were written in: a timestamped line from Blender's own log can appear above a script's line printed before it. Where an output is cut for length, the section says what was left out.
- Exit statuses were read straight after each command.
- Nothing was fetched from the internet. Commands that could have gone online were run inside a macOS network block (`sandbox-exec` with a profile denying outgoing connections), and the section says so where it matters.
- Every run started with `--background` that ended normally ended with the line `Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)`, a blank line and `Blender quit`. Those three lines are left out of the quoted outputs below; runs started with `--command` printed none of them. Where a run ended otherwise, its output is quoted whole, closing lines included.

---

## 1. Can a `.blend` file be read for the add-ons it needs?

**The question is ambiguous.** Reading A: can a `.blend` file be examined, with or without Blender, to get a list of the add-ons it needs? Reading B: once the file is open in Blender, can it be inspected for signs of add-ons? I answer both.

**Label:** Reading A — **not in the sources**. Reading B — **inferred**, and only partial.

**Reading A.** No page describes a list of needed add-ons stored inside a `.blend` file. No page describes a command or function that produces such a list. The sources do not describe the `.blend` file format at all, so reading a file without Blender is not covered.

**Reading B.** Once a file is open, some traces of add-ons can be read. None of them names an add-on.

1. Settings an add-on defines are saved in the file, but the add-on's definition of them is not.
   > "The values assigned to Blender data are saved to disk but the class definitions are not, this means whenever you load Blender the class needs to be registered too."
   > "This is best done by creating an add-on which loads on startup and registers your properties."

   — API documentation, https://docs.blender.org/api/5.3/bpy.types.PropertyGroup.html, read 2026-10-11.

2. Each scene stores the render engine it uses, and render engines can come from add-ons.
   > "Engine to use for rendering (default 'BLENDER_EEVEE')"

   — API documentation, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html (the `engine` setting), read 2026-10-11.
   > "More renderers from third-party developers are available as add-ons."

   — manual, https://docs.blender.org/manual/en/5.3/render/introduction.html, read 2026-10-11.

3. The engine name registered by an add-on differs from its module name.
   **Observed:** in Cycles' own files (`__init__.py`, lines 44–45, read 2026-10-16), the engine class registers with `bl_idname = 'CYCLES'`:
   > `class CyclesRender(bpy.types.RenderEngine):`
   > `    bl_idname = 'CYCLES'`

   Cycles' files sit in a folder called `cycles` (the API documentation names its files as `addons_core/cycles/operators.py:49` — https://docs.blender.org/api/5.3/bpy.ops.cycles.html, read 2026-10-11).
   **Inferred:** the name a scene stores for its engine (item 2) is this engine identifier (`'CYCLES'`), not the add-on's module name (`'cycles'`). Nothing in the sources maps an engine name back to the add-on that provides it.

4. Node trees have an "Undefined" and a "Custom" type.
   > "UNDEFINED Undefined – Undefined type of nodes (can happen e.g. when a linked node tree goes missing)."
   > "CUSTOM Custom – Custom nodes."

   — API documentation, https://docs.blender.org/api/5.3/bpy.types.NodeTree.html, read 2026-10-11.

5. A workspace can carry its own list of add-ons.
   > "Filter Add-ons — Determines which add-ons are enabled in the active workspace. When unchecked, the global add-ons will be used. When checked, you can enable individual add-ons in the list below."

   — manual, https://docs.blender.org/manual/en/5.3/interface/window_system/workspaces.html, read 2026-10-11.
   > "Workspace data-block, defining the working environment for the user"

   — API documentation, https://docs.blender.org/api/5.3/bpy.types.WorkSpace.html, read 2026-10-11.
   > "They are the primary contents of the blend-file."

   — manual, https://docs.blender.org/manual/en/5.3/files/data_blocks.html (about data-blocks, which it says include "even Workspaces"), read 2026-10-11.

**The steps I took (inferred).** From 2 and 3: reading a scene's engine setting tells you an engine name. It does not tell you which add-on supplies that engine; that needs knowledge from outside the file. From 1: a file can hold values that only make sense with an add-on present, but the sources do not say how those values look when the add-on is absent, or how to tell which add-on they belong to. From 4: a node tree of kind "Custom" was made with nodes some script registered; which script is not recorded anywhere the sources describe. From 5: a file can list add-ons per workspace, but this is a filter on what the interface shows, used only when the filter is switched on. It is not a list of what the file needs to render.

**Not covered.** Whether Blender keeps any record of which add-on created a saved setting. Whether saved add-on values stay readable when the add-on is absent. Any way to read a `.blend` file without Blender. Any way to map an engine name or a setting name to an add-on.

**Where I looked.** Beyond the general searches: "missing add-on", "required", "depend", "need" near "add-on", "undefined", "unregistered", "not registered", "file format", "DNA", "SDNA", "blendfile", "blend-file" near "add-on". Pages read: the API list of what a loaded file holds (`bpy.types.BlendData` — no add-on entry), the manual's pages on blend-file compatibility, opening and saving, data-blocks, and custom properties. All 5.3 release notes pages: nothing on this.

**Outside the interface.** **Settled by test for Reading B — observed, on the Creator's Mac, 2026-10-17; see the two Observed sections below.** With no window, a script read a file's stored engine, its add-on's saved sample count and its workspace add-on list, and wrote and saved the workspace list. *What the sources say:* Reading B needs the file open in Blender. Blender can open a file with no window (`-b, --background`) and run a Python script (`-P, --python <filepath>`); both are on https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11. So a script can read the traces in B without the interface — **inferred** from those two documented options. The sources say nothing about reading the file without Blender.

**Operating system.** **Tested on macOS on Apple silicon only (2026-10-17, below);** Linux and Windows were not tested in this sitting, which had no such machine. *What the sources say:* nothing about this differing between operating systems.

### Observed — what a file's stored engine and an add-on's saved settings read back as, with the add-on on, off and on again

**Label: observed.** Entry 10 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new settings folder — see *How the hand tests were run* at the top of this document.

A file `cycles.blend` was saved from the start-up scene with the engine set to Cycles and Cycles' sample count set to 1. `blender --background cycles.blend --python read_cycles_file.py` — printing the scene's `render.engine`, whether the scene has a `cycles` attribute, the sample count through it, the identifiers of the engine setting's list of values, and the scene's own keys — was run three times on the untouched file.

With Cycles switched on, exit status 0:

```
00:00.326  blend            | Read blend: "<work>/out/10/cycles.blend"
ENGINE CYCLES
HAS_CYCLES True
SAMPLES 1
ITEMS ['BLENDER_EEVEE']
KEYS []
```

With Cycles switched off and the preferences saved, exit status 0:

```
00:00.323  blend            | Read blend: "<work>/out/10/cycles.blend"
00:00.328  reports          | ERROR Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
ENGINE BLENDER_EEVEE
HAS_CYCLES False
SAMPLES None
ITEMS ['BLENDER_EEVEE']
KEYS []
```

With Cycles switched back on and the preferences saved, exit status 0:

```
00:00.332  blend            | Read blend: "<work>/out/10/cycles.blend"
ENGINE CYCLES
HAS_CYCLES True
SAMPLES 1
ITEMS ['BLENDER_EEVEE']
KEYS []
```

A second file, `engine.blend`, saved with the engine `SMOOZY_TEST` from a test add-on, read back in a folder where that add-on had never been installed as `ERROR Engine 'SMOOZY_TEST' not available for scene 'Scene' (an add-on may need to be installed or enabled)` and `ENGINE BLENDER_EEVEE` (question 7).

### Observed — the workspace add-on list, saved in the file and read back with no window

**Label: observed.** Entry 11 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new, empty settings folder — see *How the hand tests were run* at the top of this document.

`blender --background --python ws_write.py` — a script taking the first workspace, printing its name and which of `owner_ids`, `use_filter_by_owner` and `active_addon` it has, switching its filter on, calling `bpy.ops.wm.owner_enable(owner_id='cycles')`, printing the workspace's `owner_ids`, and saving the file as `ws.blend` — printed, exit status 0:

```
NAME Animation
PROPS ['owner_ids', 'use_filter_by_owner', 'active_addon']
ENABLE {'FINISHED'}
OWNERS []
Info: Saved as "ws.blend"
```

The saved file was opened again, with no window, and every workspace printed with its filter setting and its add-on list — `blender --background ws.blend --python ws_read.py` — exit status 0:

```
00:00.323  blend            | Read blend: "<work>/out/11/ws.blend"
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

So the name `cycles` was saved in the file against the Layout workspace — not the Animation workspace the script had in hand and had switched the filter on for — and read back out of it with no window.

### Observed — the workspace "Filter Add-ons" control at the window

**Label: observed.** Entry 18 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, **with Blender's window open on the Mac's screen**, started from a terminal, driven by a script and photographed with Blender's own `bpy.ops.screen.screenshot`; a new settings folder, its preferences saved once with the splash switched off. Each picture was taken after a forced redraw, twice.

The script turned the largest editor into the Properties editor at its Tool tab and printed `SMOOZY_PROPS PROPERTIES TOOL 2477 1503` and `SMOOZY_WORKSPACE Layout use_filter_by_owner False`. **The picture shows**, in the Tool tab: `Select Box` with its row of mode buttons; an open section `Options` holding `Affect Only` with three unticked boxes, `Origins`, `Locations`, `Parents`; and a **closed section headed `Workspace`**. The script then set the workspace's filter on — `SMOOZY_BEFORE use_filter_by_owner False`, `SMOOZY_AFTER use_filter_by_owner True`, `SMOOZY_OWNERS []` — and the next picture was the same as the first, the `Workspace` section being closed. A script cannot open a section drawn closed, so the "Filter Add-ons" tick box itself, inside that section, was not seen.

The add-on list that belongs to that control was opened instead with Blender's own operator, `bpy.ops.wm.call_panel('INVOKE_DEFAULT', name='WORKSPACE_PT_addons', keep_open=True)` — the identifier read out of `bpy.types` first — which printed `SMOOZY_CALL_PANEL {'INTERFACE'}`. The picture shows a small panel over the editor holding **three rows**, each with an empty tick box at its right:

```
Import-Export: glTF 2.0 for...      [ ]
Render: Cycles Render Engi...       [ ]
Animation: Pose Library             [ ]
```

(names cut off by the panel's width), with empty space below. Each row carries the add-on's category before its name.

**Outside the interface.** The property behind the control was set and saved from a script with no window — see the section above.

**Operating system.** macOS on Apple silicon only.

---

## 2. What makes a `.blend` file need an add-on at the moment it renders, and can an add-on that takes part in producing the render be told apart from one that only rearranges Blender's existing tools while the artist works?

### Part 1 — what makes a file need an add-on at render time

**Label:** **documented** for the ways add-on code can take part in a render. **Inferred** for the step to "the file needs the add-on". The sources never use that phrase.

*Those labels answer from the sources only. Parts have since been tested — **observed** on the Creator's Mac, 2026-10-17, entries 10 and 17 of the hand tests; see the Observed section at the end of this answer and question 7.*

The documented ways add-on code takes part in a render:

1. **The render engine itself.**
   > "More renderers from third-party developers are available as add-ons."

   — manual, https://docs.blender.org/manual/en/5.3/render/render_engines.html, read 2026-10-11.
   > "This is the method called by Blender for both final renders (F12) and small preview for materials, world and lights."

   — API documentation, https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html (the example add-on render engine), read 2026-10-11.
   Cycles is itself an add-on. The picture `editors_preferences_section_addons.webp` on https://docs.blender.org/manual/en/5.3/editors/preferences/addons.html (read 2026-10-11) shows "Cycles Render Engine" ticked in the Add-ons list, beside "Hydra Storm render engine" (not ticked). **Documented** (picture).
   **Observed:** Cycles' `register()` function (`__init__.py`, lines 133–158) registers the engine class (`register_class(cls)`, line 154, for `classes = (CyclesRender,)`). It also attaches its own groups of settings to scenes, materials, lights, worlds, objects and more — for example `bpy.types.Scene.cycles = PointerProperty(` at `properties.py`, line 1191. Cycles' own files, read 2026-10-16.

2. **Saved settings the add-on defines.** The PropertyGroup quote under question 1: values are saved, definitions are not, "whenever you load Blender the class needs to be registered too."

3. **Functions Blender calls during a render.**
   > "Handlers are functions that Blender calls when certain events occur, such as loading a file, changing frame or rendering."
   > "By default handlers are freed when loading new files, in some cases you may want the handler stay running across multiple files (when the handler is part of an add-on for example)."

   The list includes `render_init`, `render_pre`, `render_post`, `render_write`, `frame_change_pre` and `frame_change_post`.
   — API documentation, https://docs.blender.org/api/5.3/bpy.app.handlers.html, read 2026-10-11.

4. **Custom node types — with a limit.** The things a script can register include `bpy.types.NodeTree`, `bpy.types.Node` and `bpy.types.NodeSocket` (https://docs.blender.org/api/5.3/bpy.utils.html, `register_class`, read 2026-10-11). But:
   > "This is intentionally limited. Currently, for more advanced features such as mesh modifiers, object types, or shader nodes, C/C++ must be used."

   — API documentation, https://docs.blender.org/api/5.3/info_overview.html, read 2026-10-11.

5. **Functions used by drivers.** Scripts can add functions that driver expressions call:
   > "Dictionary for drivers namespace, editable in-place, reset on file load (read-only)."

   — API documentation, https://docs.blender.org/api/5.3/bpy.app.html, read 2026-10-11. The manual shows a script adding a function this way (https://docs.blender.org/manual/en/5.3/animation/drivers/workflow_examples.html, read 2026-10-11). Neither page names add-ons as the thing that adds them.

**The step (inferred).** A file needs an add-on at render time when what it saved points at something only that add-on supplies: the engine named in the scene (1), settings stored under the add-on's groups (2), or functions its drivers call (5). It also needs it when the add-on changes the scene through a handler while the render runs (3). Point 4 limits what a pure-Python add-on can contribute: by that quote, it cannot add a new shader node.

### Part 2 — telling the two kinds apart

**Label:** **inferred** from what the manifest does and does not hold; no source addresses the question directly.

- The extension manifest has no field saying whether an add-on takes part in rendering. The required fields are `blender_version_min`, `id`, `license`, `maintainer`, `name`, `schema_version`, `tagline`, `type` and `version`. The optional ones are `blender_version_max`, `website`, `copyright`, `tags`, `platforms`, `wheels` and `permissions` (https://docs.blender.org/manual/en/5.3/advanced/extensions/getting_started.html, read 2026-10-11).
- Tags exist and are optional. One available tag is "Render", among about thirty others (https://docs.blender.org/manual/en/5.3/advanced/extensions/tags.html, read 2026-10-11).
  > "Add-ons are assigned categories by what areas of Blender they affect."

  — manual, https://docs.blender.org/manual/en/5.3/editors/preferences/addons.html, read 2026-10-11.
- **Observed:** Cycles describes itself with the older style of description, not a manifest: `"category": "Render"` (`__init__.py`, line 16, read 2026-10-16).
- The manual gives two named examples of add-ons whose results do not need them later:
  > "Once you generate your rig you won’t need Rigify or any other add-on to use it."

  — https://docs.blender.org/manual/en/5.3/addons/rigify/introduction.html, read 2026-10-11.
  > "The add-on only contains the user interface and the logic that determines what is stored in a pose asset."

  — https://docs.blender.org/manual/en/5.3/animation/armatures/posing/editing/pose_library.html, read 2026-10-11.

**The step (inferred).** Tags and categories are labels the author chooses, and they are optional. They are not a statement about rendering. Part 1 points at what does decide it: what the add-on registers (an engine, settings groups, handlers, driver functions). Finding that out means reading or running the add-on's code. The sources describe no way to ask Blender this.

**Not covered.** Part 1: the sources never say when a file "needs" an add-on; the list above shows how add-on code can take part, and may not be complete. Part 2: no documented way to ask Blender whether an add-on affects render output. Whether a render-time handler changes the picture or only, for example, writes a log — the sources cannot say.

**Where I looked.** Beyond the general searches: "render engine", "RenderEngine", "handler", "persistent", "driver namespace", "PropertyGroup", "register_class", "category", "tags", "support". Pages read: the API overview, the handlers page, the RenderEngine page, the bpy.utils page, the extension manifest page, the tags page, and the Rigify and pose-library pages. Also all of Cycles' Python files.

**Outside the interface.** **Settled by test for what a terminal shows — observed, on the Creator's Mac, 2026-10-17; see the Observed section below and question 7.** With no window: `-E help` listed the engines registered, Cycles' and a test add-on's among them, and dropped Cycles once it was switched off; the subclasses of `bpy.types.RenderEngine` changed the same way, while the engine setting's own list of values held only `BLENDER_EEVEE`; a test add-on that is not an engine printed its own line during a render through a `render_pre` handler, and its absence printed nothing. At the window, no add-on's row showed anything saying whether it takes part in a render (question 13). *What the sources say:* Part 1: the RenderEngine example expects an add-on engine to work without a window:
> "Lazily import GPU module, so that the render engine works in background mode where the GPU module can't be imported by default."

— API documentation, https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html, read 2026-10-11. **Documented** (a comment in the example). The handlers page does not say whether render handlers behave differently in background mode.
Part 2: inspecting extension manifests, tags, and registered classes works the same without a window. The sources describe no command or API function, inside or outside the interface, to tell the two kinds apart.

**Operating system.** **Tested on macOS on Apple silicon only (2026-10-17, below);** Linux and Windows, and `-E help` on a machine with no display server, were not tested in this sitting. *What the sources say:* nothing about this differing between operating systems.

### Observed — which engines are registered, and an add-on's part in a render seen from outside

**Label: observed.** Entry 10 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, in a new settings folder — see *How the hand tests were run* at the top of this document. `-E help` was run without `--background`, on this Mac with its display there; it printed its list and ended without a window being asked for. Everything else was run with no window.

**The engine list from the command line.** `blender -E help` printed, exit status 0, with Cycles on:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

with Cycles switched off and the preferences saved:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
```

and with Cycles back on and the test add-on `smoozy_engine` installed — an extension whose whole content is a render engine with `bl_idname = "SMOOZY_TEST"`:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
	SMOOZY_TEST
```

**The engine setting's own list, and the registered engine classes.** `blender --background --python engines.py` — printing the scene's engine, the identifiers of `RenderSettings.engine`'s list of values, `has_multiple_engines`, and each subclass of `bpy.types.RenderEngine` with its `bl_idname` — printed, exit status 0, with Cycles on:

```
ENGINE BLENDER_EEVEE
ITEMS ['BLENDER_EEVEE']
MULTI True
SUBCLASSES [('HydraRenderEngine', None), ('CyclesRender', 'CYCLES')]
```

and with Cycles off, the same first three lines and `SUBCLASSES [('HydraRenderEngine', None)]`.

**An add-on that is not an engine, taking part in a render.** The test add-on `smoozy_handler`, whose whole content is a function added to `bpy.app.handlers.render_pre` that prints `SMOOZY HANDLER RAN` and sets the scene's width to 64, printed that line during a render of a 128-by-128 Workbench file in the settings folder where it was installed, and nothing in a render of the same file in a folder where it had never been; both renders exited 0 and both images measured 128 by 128. The full record is under question 7.

**An add-on's declared category, from Python.** `addon_utils.modules()` gives each add-on's metadata block with its `category` — `'category': 'Render'` for Cycles, `'category': 'Node'` for Node Wrangler, and, for an extension, `'category': 'Development'` with `'support': 'COMMUNITY'` (question 10 and question 6).

**What an add-on's row shows at the window** (entry 17, with Blender's window open; full record under question 13). Cycles' opened row shows its description, `Built-in`, `Website`, `Type  Core` and its own device settings; Node Wrangler's shows its description, `Built-in`, `Website`, `Type  Core`, `Maintainer` and `Version`; an installed extension declaring the tag `Render` shows its description, `Website`, `Type  Extension`, `Maintainer`, `Version` and `File`, and no tag. No row shows a category or anything saying whether the add-on takes part in a render.

**The engine field at the window with Cycles switched off** (entry 21; full record under question 7): a file saved with Cycles opened with its `Render Engine` field reading `EEVEE`, a red bar along the bottom reading `Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)`, and an engine list holding only `EEVEE` and `Workbench`.

---

## 3. Can an add-on be installed, switched on, and given whatever else it needs before it will work, on a computer with no screen attached?

**Label:** **documented** that installing and switching on can be done by commands, without Blender's window. **Inferred** that this works with no display at all, and that the change is kept. **Not in the sources:** whether switching on from a script without a window is saved automatically.

*Those labels answer from the sources only. Since tested — **observed** on the Creator's Mac, 2026-10-17, entries 1 to 6 of the hand tests; see the Observed sections at the end of this answer. Among them: a switch-on made by a script with no window and no save was not there in the next run, and was there after a run that also saved.*

**Installing and switching on by command.**
> "blender --command extension install-file [-h] -r REPO [-e] [--no-prefs] FILE" … "Install a package file into a user repository." … "-e, --enable Enable the extension after installation."
> "blender --command extension install [-h] [-s] [-e] [--no-prefs] PACKAGES" … "-s, --sync Sync the remote directory before performing the action."

— manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/extension_arguments.html, read 2026-10-11.
> "-c, --command <command> Run a command which consumes all remaining arguments. … This implies --background mode."
> "-b, --background Run in background (often used for UI-less rendering)."
> "--addons <addon(s)> Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons."

— manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11.

**By placing files and a start-up script.**
> "Extensions packages should be extracted in this system directory, with a resulting path like this: $BLENDER_SYSTEM_EXTENSIONS/system/my-addon/blender_manifest.toml"
> "For example, a script can enable add-ons for every user. $BLENDER_SYSTEM_SCRIPTS/startup/enable_addons.py" (the example script calls `addon_utils.enable("my-addon")`)
> "$BLENDER_SYSTEM_SCRIPTS/addons/simple_addon.py" (for older-style add-ons)

— manual, https://docs.blender.org/manual/en/5.3/advanced/deploying_blender.html, read 2026-10-11.

**From Python.** `bpy.ops.preferences.addon_install(… enable_on_install=False …)` "Install an add-on"; `bpy.ops.preferences.addon_enable(module='')` "Turn on this add-on" (https://docs.blender.org/api/5.3/bpy.ops.preferences.html). `bpy.ops.extensions.package_install(…)` "Download and install the extension"; `bpy.ops.extensions.package_install_files(…)` "Install extensions from files into a locally managed repository" (https://docs.blender.org/api/5.3/bpy.ops.extensions.html). Both read 2026-10-11.

**No display needed (for rendering).**
> "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."

— manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/render.html, read 2026-10-11. This is said about rendering. Since `--command` "implies --background mode", I **infer** the same holds for the extension commands. The sources do not say it in so many words.

**"Whatever else it needs."**
- Bundled Python modules are unpacked by the install itself:
  > "Now installing the package will extract the wheel into the extensions own site-packages directory."

  — https://docs.blender.org/manual/en/5.3/advanced/extensions/python_wheels.html, read 2026-10-11.
- Settings inside the add-on:
  > "Some add-ons may have their own preferences … Sometimes these might even all default to off. So it is important to check if the enabled add-on has any particular preferences."

  — https://docs.blender.org/manual/en/5.3/editors/preferences/addons.html, read 2026-10-11.
  They can be reached from Python as `bpy.context.preferences.addons[__package__]` (https://docs.blender.org/manual/en/5.3/advanced/extensions/addons.html, read 2026-10-11).
  **Observed** example: Cycles keeps its GPU device choice in its own add-on settings — `class CyclesPreferences(bpy.types.AddonPreferences):` with `compute_device_type: EnumProperty(` (`properties.py`, lines 1680 and 1710, read 2026-10-16).
- Internet access:
  > "By default Blender does not access the internet"

  — https://docs.blender.org/manual/en/5.3/advanced/deploying_blender.html, read 2026-10-11.
  > "--online-mode Allow internet access, overriding the preference."

  — https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11.
  The picture `editors_preferences_section_extensions.webp` on https://docs.blender.org/manual/en/5.3/editors/preferences/extensions.html (read 2026-10-11) shows the message: "Internet access is required to install and update online extensions." **Documented** (picture).
- The GPU drawing module, without a window:
  > "Initializes the GPU module for background use. If the initialization fails, a SystemError will be raised."

  — https://docs.blender.org/api/5.3/gpu.html (`gpu.init()`), read 2026-10-11. With the RenderEngine comment quoted under question 2, this suggests an add-on that loads that module as it starts may fail without a window — **inferred**.

**Is the change kept?**
> "--no-prefs Treat the user-preferences as read-only, preventing updates for operations that would otherwise modify them. This means removing extensions or repositories for example, won’t update the user-preferences."

— https://docs.blender.org/manual/en/5.3/advanced/command_line/extension_arguments.html, read 2026-10-11. **Inferred:** without `--no-prefs`, the extension commands do write to the stored preferences, so an extension installed with `-e` stays switched on.
For changes made from a Python script: "Save preferences on exit when modified (unless factory settings have been loaded) (default True)" (https://docs.blender.org/api/5.3/bpy.types.Preferences.html), and `bpy.ops.wm.save_userpref()` "Make the current preferences default" (https://docs.blender.org/api/5.3/bpy.ops.wm.html). Both read 2026-10-11. Whether the save-on-exit happens in background mode is **not in the sources**.

**Not covered.** Whether `--addons` accepts the long names extensions get (see question 6). The identifier of the default user repository, which `install-file -r` needs: the picture shows only the display name "User Default". Whether `extension install` needs `--online-mode`. Because `-c` "consumes all remaining arguments", `--online-mode` would have to come before `-c` — **inferred**. Whether a given add-on's own start-up code works without a display.

**Where I looked.** Beyond the general searches: "background", "headless", "display", "X server", "--addons", "enable", "install", "no-prefs", "save_userpref", "auto-save", "online". Pages read: the command-line arguments page, the extension command-line page, the deploying page, the preferences introduction and system pages, the Add-ons and Get Extensions preference pages, the wheels page, the API pages for preferences operators, extensions operators, `bpy.app`, `gpu`, the RenderEngine example, and Blender as a Python module.

**Outside the interface.** **Settled by test for everything a terminal reaches — observed, on the Creator's Mac, 2026-10-17; see the Observed sections below.** With no window: `install-file` installed a package and, with `-e`, saved the switch-on itself; switching on and off and rescanning returned `{'FINISHED'}`; a script's switch-on with no save was not there in the next run, and was after a run that also saved; `--addons` loaded an extension by its long name for that run without adding it to the switched-on list; Cycles' own settings were read, changed, saved and read back changed; a sync was refused without `--online-mode` or the online-access preference. *What the sources say:* everything above except: "Drag the installation URL into Blender" and "Search for the extension name and click on Install" (https://docs.blender.org/manual/en/5.3/editors/preferences/extensions.html, read 2026-10-11), which are interface-only. Also, on rendering without a window:
> "In this case, the Preferences are still used but you may want to override them"

— https://docs.blender.org/manual/en/5.3/advanced/scripting/security.html, read 2026-10-11.
For Blender used as a Python module:
> "The user’s startup and preferences are ignored to prevent your local configuration from impacting script behavior."

— https://docs.blender.org/api/5.3/info_advanced_blender_as_bpy.html, read 2026-10-11. **Inferred:** add-ons switched on in the preferences are therefore not loaded there by default.

**Operating system.** **Tested on macOS on Apple silicon only (2026-10-17, below).** A computer with no display server, Linux and Windows were not tested in this sitting, which had no such machine. *What the sources say:* the no-display remark names Linux ("no need for X server on Linux"). On macOS, a 5.3 release note says: "The Blender application icon no longer shows up in the Dock when running in background mode via the command line." (https://developer.blender.org/docs/release_notes/5.3/user_interface/, read 2026-10-11). Where preferences and extensions are stored differs by system (see question 6). Nothing else differs by system in the sources.

### Observed — the identifier `install-file -r` needs for the default user repository

**Label: observed.** Entry 1 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new, empty settings folder — see *How the hand tests were run* at the top of this document.

`blender --command extension repo-list` printed the repository shown as "User Default" under the identifier `user_default`:

```
user_default:
    name: "User Default"
    directory: "<work>/res/01/extensions/user_default"
    source: "USER"
```

exit status 0, and `blender --command extension install-file --help` printed `-r, --repo REPO  The repository identifier.` The full listing is under question 5. Every install in the sections below used `-r user_default`.

### Observed — installing from a file and switching on with no window, and whether it lasts

**Label: observed.** Entry 2 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new, empty settings folder — see *How the hand tests were run* at the top of this document.

**The test add-on.** `smoozy_test`, made for this test: a `blender_manifest.toml` (identifier `smoozy_test`, version 1.0.0, `blender_version_min = "4.2.0"`, licence `SPDX:GPL-3.0-or-later`, no `platforms`) and an `__init__.py` whose `register()` prints `SMOOZY TEST ADDON REGISTERED` and whose `unregister()` prints `SMOOZY TEST ADDON UNREGISTERED`. `blender --command extension build --source-dir <work>/src/smoozy_test --output-dir <work>/pkg` printed, exit status 0:

```
building: smoozy_test-1.0.0.zip
complete
created: "<work>/pkg/smoozy_test-1.0.0.zip", 459
```

**Installing and switching on.** `blender --command extension install-file -r user_default -e <work>/pkg/smoozy_test-1.0.0.zip` printed, exit status 0:

```
STATUS Installed "smoozy_test"
SMOOZY TEST ADDON REGISTERED
SMOOZY TEST ADDON UNREGISTERED
```

**The switch-on lasts into a separate run, with no save asked for.** The next run, `blender --background --python modules.py` (printing the count and sorted modules of `bpy.context.preferences.addons`), printed, exit status 0:

```
SMOOZY TEST ADDON REGISTERED
COUNT 9
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
SMOOZY TEST ADDON UNREGISTERED
```

and the settings folder now held `config/userpref.blend`, which nothing but the install had written.

**Switching on, off and rescanning from a script.** `blender --background --python enable_cycle.py`, a script calling `bpy.ops.preferences.addon_enable(module='bl_ext.user_default.smoozy_test')`, printing the switched-on modules, calling `addon_disable` on the same module and `addon_refresh()`, and printing the modules again, printed, exit status 0:

```
SMOOZY TEST ADDON REGISTERED
SMOOZY TEST ADDON UNREGISTERED
SMOOZY TEST ADDON REGISTERED
ENABLE {'FINISHED'}
AFTER ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
SMOOZY TEST ADDON UNREGISTERED
DISABLE {'FINISHED'}
REFRESH {'FINISHED'}
END ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

**With `--no-prefs`**, in a second new settings folder, `blender --command extension install-file -r user_default -e --no-prefs <work>/pkg/smoozy_test-1.0.0.zip` printed the same three lines, exit status 0. The next run's `MODULES` held only the eight that a new folder starts with, `COUNT 8`, and printed no register line. `blender --command extension list` still showed `smoozy_test [installed]: "Smoozy Test", A tiny test add-on` under User Default. The settings folder held the add-on's folder and `extensions/.cache/compat.dat`, and no `config` folder at all.

### Observed — a script's switch-on with no window, read back with and without a save

**Label: observed.** Entry 3 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new, empty settings folder — see *How the hand tests were run* at the top of this document. The test add-on `smoozy_test` is described above.

The package was installed **without** `-e`: `blender --command extension install-file -r user_default <work>/pkg/smoozy_test-1.0.0.zip` printed `STATUS Installed "smoozy_test"`, exit status 0, and nothing else. A separate run printed `COUNT 8` and the eight names a new folder starts with.

**Switched on, no save asked for.** `blender --background --python enable_nosave.py`, a script printing `bpy.context.preferences.use_preferences_save`, calling `bpy.ops.preferences.addon_enable(module='bl_ext.user_default.smoozy_test')`, then printing `bpy.context.preferences.is_dirty` and the switched-on modules, printed, exit status 0:

```
USE_PREFERENCES_SAVE True
SMOOZY TEST ADDON REGISTERED
ENABLE {'FINISHED'}
IS_DIRTY True
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
SMOOZY TEST ADDON UNREGISTERED
```

The very next run, `blender --background --python modules.py`, printed, exit status 0:

```
COUNT 8
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

with no register line.

**Switched on and saved.** `blender --background --python enable_save.py`, the same switch-on followed by `bpy.ops.wm.save_userpref()`, printed, exit status 0:

```
00:00.324  blend            | Writing user preferences: "<work>/res/03/config/userpref.blend" 
SMOOZY TEST ADDON REGISTERED
ENABLE {'FINISHED'}
Info: Preferences saved
SAVE {'FINISHED'}
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
SMOOZY TEST ADDON UNREGISTERED
```

and the very next run printed `SMOOZY TEST ADDON REGISTERED`, `COUNT 9` and the nine names with `bl_ext.user_default.smoozy_test` first. The settings folder's `config` folder and its `userpref.blend` both carry the time of that saving run; neither the install without `-e` nor the run without a save had made them.

### Observed — what `--addons` does with an extension's long name

**Label: observed.** Entry 4 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new, empty settings folder — see *How the hand tests were run* at the top of this document. The test add-on `smoozy_test` is described above. `modules.py` prints the count and sorted modules of `bpy.context.preferences.addons`.

Ten runs in one settings folder, in this order. Every run exited 0.

```
--- 1. blender --background --factory-startup --python modules.py   (nothing installed)
COUNT 8
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
--- 2. blender --background --python modules.py   (the new settings folder, nothing installed)
COUNT 8
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
--- 3. blender --command extension install-file -r user_default --no-prefs <work>/pkg/smoozy_test-1.0.0.zip
STATUS Installed "smoozy_test"
    then blender --command extension list showed, under User Default:
  smoozy_test [installed]: "Smoozy Test", A tiny test add-on
--- 4. blender --background --addons bl_ext.user_default.smoozy_test --python modules.py
SMOOZY TEST ADDON REGISTERED
COUNT 8
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
SMOOZY TEST ADDON UNREGISTERED
--- 5. blender --background --python modules.py   (the very next run, no --addons)
COUNT 8
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
--- 6. blender --background --python enable_save.py   (switch on, then save the preferences)
00:00.330  blend            | Writing user preferences: "<work>/res/04/config/userpref.blend" 
SMOOZY TEST ADDON REGISTERED
ENABLE {'FINISHED'}
Info: Preferences saved
SAVE {'FINISHED'}
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
SMOOZY TEST ADDON UNREGISTERED
--- 7. blender --background --addons bl_ext.user_default.smoozy_test --python modules.py
SMOOZY TEST ADDON REGISTERED
COUNT 9
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
SMOOZY TEST ADDON UNREGISTERED
--- 8. blender --background --python modules.py   (the very next run, no --addons)
SMOOZY TEST ADDON REGISTERED
COUNT 9
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
SMOOZY TEST ADDON UNREGISTERED
--- 9. blender --background --factory-startup --python modules.py   (the same folder)
COUNT 8
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
--- 10. blender --background --addons smoozy_test --python modules.py   (the short name)
SMOOZY TEST ADDON REGISTERED
Add-on not loaded: "smoozy_test", cause: No module named 'smoozy_test'
COUNT 9
MODULES ['bl_ext.user_default.smoozy_test', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
SMOOZY TEST ADDON UNREGISTERED
```

At step 4 the add-on's register line printed and the switched-on list held 8 names; at step 5 neither the register line nor the name appeared. The count reached 9 only after step 6's save, and then read 9 with `--addons` (step 7) and without it (step 8), and 8 again with `--factory-startup` (step 9), with no register line. At step 10 the short name was refused with the line above, and the run went on and exited 0.

### Observed — an add-on's own settings read, changed and kept, with no window (Cycles)

**Label: observed.** Entry 5 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new, empty settings folder — see *How the hand tests were run* at the top of this document.

**Read.** `blender --background --python cycles_prefs.py` — a script finding Cycles' key among the switched-on add-ons rather than assuming it, printing its settings' names, the current compute device, the values `compute_device_type` offers, and each setting's kind and value — printed, exit status 0:

```
KEYS ['cycles']
PROPS ['rna_type', 'bl_idname', 'compute_device_type', 'devices', 'peer_memory', 'metalrt', 'use_hiprt', 'use_oneapirt', 'kernel_optimization_level']
DEVICE METAL
OFFERED []
PROP bl_idname STRING 
PROP compute_device_type ENUM METAL
PROP peer_memory BOOLEAN False
PROP metalrt ENUM AUTO
PROP use_hiprt BOOLEAN True
PROP use_oneapirt BOOLEAN True
PROP kernel_optimization_level ENUM FULL
```

**Changed and saved.** `blender --background --python cycles_toggle.py`, setting `peer_memory` to the opposite of what it held and calling `bpy.ops.wm.save_userpref()`, printed, exit status 0:

```
00:00.322  blend            | Writing user preferences: "<work>/res/05/config/userpref.blend" 
BEFORE False
SET True
Info: Preferences saved
SAVE {'FINISHED'}
```

**Kept.** A separate later run, `blender --background --python cycles_after.py`, printed `AFTER True`, exit status 0.

**The devices.** `blender --background --python cycles_devices.py`, printing the `devices` collection, calling Cycles' own `refresh_devices()`, and printing the collection again, printed, exit status 0:

```
DEVICES_BEFORE []
REFRESH None
DEVICES_AFTER [('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]
DEVICE METAL
OFFERED []
```

So with no window the list of values `compute_device_type` offers reads empty while its value reads `METAL`, and the devices collection is empty until `refresh_devices()` is called.

---

## 4. What does an add-on record about the kinds of computer it is meant for, what happens on one it does not name, and does an add-on set up on one computer still work when copied to a computer of a different kind?

**Label:** what is recorded — **documented**. What happens on an unnamed kind — **documented** for listing, **not in the sources** for installing. Copying — **documented** that binaries do not move between systems; the rest **inferred**.

*Those labels answer from the sources only. Installing a package that names another kind of computer, and one whose only bundled module is named for another kind, have since been tested — **observed** on the Creator's Mac, 2026-10-17, entries 7 to 9 of the hand tests; see the Observed sections at the end of this answer. Copying an installed add-on to another kind of computer was not tested in this sitting.*

**What is recorded.**
> "platforms: List of supported platforms. If omitted, the extension will be available in all operating systems. The available options are [“windows-x64”, “windows-arm64”, “macos-x64”, “macos-arm64”, “linux-x64”]"

— manual, https://docs.blender.org/manual/en/5.3/advanced/extensions/getting_started.html, read 2026-10-11.
A manifest can also list bundled Python "wheels" per system. The example has one each for `macosx_11_0_arm64`, `manylinux_2_28_x86_64` and `win_amd64` (https://docs.blender.org/manual/en/5.3/advanced/extensions/python_wheels.html, read 2026-10-11).
Older-style add-ons describe themselves with `bl_info`. The examples in the sources show keys for name, author, version, minimum Blender version, location, description, warning, links and category — no key for a kind of computer (https://docs.blender.org/manual/en/5.3/advanced/scripting/addon_tutorial.html; https://docs.blender.org/api/5.3/bpy.types.AddonPreferences.html; read 2026-10-11). **Observed:** Cycles' `bl_info` (`__init__.py`, lines 7–16) has no such key either. **Inferred:** older-style add-ons record nothing about the kind of computer.

For reference, the manual also says which computers Blender 5.3 runs on:
> "Blender 5.0 and later only supports Macs with Apple silicon."

— https://docs.blender.org/manual/en/5.3/getting_started/installing/macos.html, read 2026-10-11.
> "Blender supports both x64 and arm64 architectures on Windows."

— https://docs.blender.org/manual/en/5.3/getting_started/installing/windows.html, read 2026-10-11.

**What happens on a kind it does not name.**
> "When Blender fetches the extensions listing it passes the following arguments to make sure only compatible extensions are listed: platform blender_version"

— https://docs.blender.org/manual/en/5.3/advanced/extensions/creating_repository/dynamic_repository.html, read 2026-10-11.
> "Even though there is a Linux-only wheel present, no Linux zip file is generated. This happens because the platforms field only has Mac and Windows."

— https://docs.blender.org/manual/en/5.3/advanced/extensions/python_wheels.html, read 2026-10-11 (about building one package per system with `--split-platforms`).
What happens if a package that does not name the current system is installed from a file is **not in the sources**.

**Copying to a different kind of computer.**
> "While not common practice, Python add-ons can be distributed with their own precompiled libraries. Unlike regular Python scripts, these are not portable between different platforms."
> "If the add-on contains .pyd or .so files, check that the distribution is compatible with your operating system."

— manual, https://docs.blender.org/manual/en/5.3/troubleshooting/python.html, read 2026-10-11.
> "For wheels that contain binary compiled files, wheels for all supported platforms should be included"

— https://docs.blender.org/manual/en/5.3/advanced/extensions/python_wheels.html, read 2026-10-11.
> "The value of __package__ will vary between systems so it should never be replaced with a literal string."

— https://docs.blender.org/manual/en/5.3/advanced/extensions/addons.html, read 2026-10-11.
**Observed**, Cycles' own settings: the default GPU choice is picked by system and chip.
> `# Default to selecting the Metal compute device on Apple Silicon GPUs`
> `if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):`

The offered device types come from what the machine reports: `has_cuda, has_optix, has_hip, has_metal, has_oneapi, has_hiprt = _cycles.get_device_types()` (`properties.py`, lines 1684–1694, read 2026-10-16).

**The steps (inferred).** An add-on whose files are plain Python can move between systems. One carrying compiled libraries cannot, per the troubleshooting quote. Installing unpacks wheels into the extension's own folder (question 3), so an installed copy may hold only what was unpacked on its first system; the sources do not say whether all systems' wheels or only the current one are unpacked. Settings saved for one machine may name things the other machine lacks — Cycles' device type is an example.

**Not covered.** Whether Blender refuses, warns, or silently installs a package that does not name the current system. Whether an installed extension folder copied by hand to another system is recognised. Whether settings copied between machines are checked.

**Where I looked.** Beyond the general searches: "platforms", "windows-x64", "macos-arm64", "linux-x64", "architecture", "arm64", "x64", "compatible", "incompatible", "portable", ".pyd", ".so", "precompiled". Pages read: the manifest page, the wheels page, the static and dynamic repository pages, the extension command-line page, the Python troubleshooting page, the installing pages for Windows, macOS and Linux, and Cycles' `properties.py`.

**Outside the interface.** **Settled by test for installing with no window — observed, 2026-10-17, below.** A package naming only another kind of computer printed `ERROR <id>: This platform (macos-arm64) isn't one of (…)` at `install-file` and was not installed, exit status 0; placed by hand it was listed and refused at switch-on; a repository made from a folder left it out of the listing and `install` by name did not find it. A package whose only bundled module is named for another kind of computer installed and switched on with nothing said, and failed at import. *What the sources say:* the `platforms` field and wheels are part of the package, so they apply however it is installed — **inferred**. The listing filter is described for fetching listings; whether the command-line `install` uses the same filter is not stated.

**Operating system.** **Tested on macOS on Apple silicon only (2026-10-17, below).** What a Linux or Windows machine does with a package naming another kind of computer, and carrying an installed add-on to a computer of another kind, were not tested in this sitting, which had no such machine. *What the sources say:* this whole question is about systems. The five names above are the only ones the manifest accepts. File-name differences for wheels are shown above.

### Observed — what Cycles' own device setting reads on this machine

**Label: observed.** Entry 5 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new, empty settings folder — see *How the hand tests were run* at the top of this document. Full record under question 3.

In a new settings folder Cycles' `compute_device_type` read `METAL`. After Cycles' own `refresh_devices()`, its `devices` collection held `[('Apple M1 Pro', 'CPU', False), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]` — name, kind, and whether it is used. This is one machine with Apple's own graphics; what the same setting reads on a machine with another kind of graphics was not tested in this sitting (no such machine).

### Observed — a package naming another kind of computer, installed from a file and placed by hand

**Label: observed.** Entry 7 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new, empty settings folder — see *How the hand tests were run* at the top of this document.

**The packages.** Four test add-ons, identical but for one line of their manifests: `platforms = ["windows-x64"]` (`smoozy_win_only`), `["macos-x64"]` (`smoozy_macx64_only`), `["linux-x64"]` (`smoozy_linux_only`) and `["macos-arm64"]` (`smoozy_macarm_only`) — the last being this Mac's own kind. All four built, `blender --command extension build --source-dir <work>/src/<id> --output-dir <work>/pkg` printing `building: …`, `complete`, `created: …` and exiting 0 each time.

**Installed from a file.** `blender --command extension install-file -r user_default -e <work>/pkg/<id>-1.0.0.zip`, once for each, printed:

```
--- smoozy_win_only
ERROR smoozy_win_only: This platform (macos-arm64) isn't one of (windows-x64)
Package was expected to be installed but not found: smoozy_win_only
EXIT 0
--- smoozy_macx64_only
ERROR smoozy_macx64_only: This platform (macos-arm64) isn't one of (macos-x64)
Package was expected to be installed but not found: smoozy_macx64_only
EXIT 0
--- smoozy_linux_only
ERROR smoozy_linux_only: This platform (macos-arm64) isn't one of (linux-x64)
Package was expected to be installed but not found: smoozy_linux_only
EXIT 0
--- smoozy_macarm_only
STATUS Installed "smoozy_macarm_only"
SMOOZY MACARM ONLY ADDON REGISTERED
SMOOZY MACARM ONLY ADDON UNREGISTERED
EXIT 0
```

All four exited 0, the three refusals included. `blender --command extension list` then showed one package under User Default — `smoozy_macarm_only [installed]: "Smoozy Apple Silicon Only", A test add-on naming one platform` — and the switched-on list held `bl_ext.user_default.smoozy_macarm_only` and the eight built-in names, `COUNT 9`.

**The Windows-only folder placed straight into a second new settings folder.** Its source folder — `__init__.py` and `blender_manifest.toml` — was copied to `<settings folder>/extensions/user_default/smoozy_win_only`. `blender --command extension list` printed, exit status 0:

```
[Errno 2] No such file or directory: '<work>/res/07b/extensions/blender_org'
Repository data: <work>/res/07b/extensions/blender_org not found, sync required!
Repository: "extensions.blender.org" (id=blender_org)
Repository: "User Default" (id=user_default)
  smoozy_win_only [installed]: "Smoozy Windows Only", A test add-on naming one platform
Repository: "System" (id=system)
```

and `blender --background --python-expr "import bpy; print('ENABLE', bpy.ops.preferences.addon_enable(module='bl_ext.user_default.smoozy_win_only')); print('MODULES', sorted(a.module for a in bpy.context.preferences.addons))"` printed, exit status 0:

```
Traceback (most recent call last):
  File "/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/modules/addon_utils.py", line 388, in enable
    raise RuntimeError("Extension {:s} is incompatible ({:s})".format(module_name, error))
RuntimeError: Extension bl_ext.user_default.smoozy_win_only is incompatible (This platform (macos-arm64) isn't one of (windows-x64))
Traceback (most recent call last):
  File "<string>", line 1, in <module>
RuntimeError: Error: Extension bl_ext.user_default.smoozy_win_only is incompatible (This platform (macos-arm64) isn't one of (windows-x64))

Error: Extension bl_ext.user_default.smoozy_win_only is incompatible (This platform (macos-arm64) isn't one of (windows-x64))
```

Neither `ENABLE` nor `MODULES` was printed.

**One package per kind of computer.** `blender --command extension build --split-platforms --source-dir <work>/src/smoozy_test --output-dir <work>/pkg/split`, on a manifest naming no platforms, printed `FATAL_ERROR: Error in arguments "--split-platforms" with a manifest that does not declare "platforms"`, exit status 1. On a copy whose manifest reads `platforms = ["windows-x64", "macos-arm64", "linux-x64"]` it printed, exit status 0:

```
building: smoozy_split-1.0.0-windows_x64.zip
complete
building: smoozy_split-1.0.0-macos_arm64.zip
complete
building: smoozy_split-1.0.0-linux_x64.zip
complete
created: "<work>/pkg/split/smoozy_split-1.0.0-linux_x64.zip", 597
```

and the output folder held the three zips named there.

**The same Windows-only package installed through the window's own install operator.** Entry 20, run with **Blender's window open on the Mac's screen**, started from a terminal and driven by a script, in a new settings folder whose preferences had been saved once with the splash switched off. The script read the operator's own arguments first, then called `bpy.ops.extensions.package_install_files(filepath=<work>/pkg/smoozy_win_only-1.0.0.zip, repo='user_default', enable_on_install=True)` from inside the window, and printed:

```
OPERATOR PROPS ['directory', 'enable_on_install', 'filepath', 'files', 'filter_glob', 'overwrite', 'repo', 'rna_type', 'target', 'url']
Package was expected to be installed but not found: smoozy_win_only
INSTALL RESULT {'FINISHED'}
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

The run exited 0. No line naming the platform was printed to the terminal in this run. Blender's own pictures of the Add-ons section then show, searched for "smoozy", **no rows at all**, and, unsearched, eight rows — Cycles Render Engine, glTF 2.0 format, Hydra Storm render engine, Manage UI translations, Node Wrangler, Pose Library, Rigify, VR Scene Inspection — with no Smoozy Windows Only. `blender --command extension list` afterwards showed nothing under User Default.

**A repository made from a folder holding the Windows-only package.** Entry 8, recorded in full under question 8: the generated `index.json` carried both packages, the Windows-only one with `"platforms": ["windows-x64"]`. Once that folder was added as a repository and synced, `blender --command extension list` showed only `smoozy_test` under it, and `install smoozy_win_only -e` printed `Package "smoozy_win_only" not found in remote repositories!`, exit status 1.

### Observed — which bundled module is unpacked, and a package whose only bundled module is named for another kind of computer

**Label: observed.** Entry 9 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window — see *How the hand tests were run* at the top of this document. **Each package was installed into a new, empty settings folder of its very own**, so nothing could be left behind from another case.

**The bundled modules were made by hand and hold no compiled code.** Each "wheel" is a zip file named in the standard way for one version of Python and one kind of computer, holding a module `smoozypkg` whose only content is a line naming the wheel it came from — for example `TAG = "cp313 macosx_11_0_arm64"` — and the three small description files a wheel carries. So what this tests is which wheel Blender picks by its name, not whether a compiled part runs. The three used here are `smoozypkg-1.0.0-cp313-cp313-macosx_11_0_arm64.whl` (macOS on Apple silicon), `smoozypkg-1.0.0-cp313-cp313-manylinux_2_28_x86_64.whl` (Linux) and `smoozypkg-1.0.0-cp313-cp313-win_amd64.whl` (Windows), all for Python 3.13. Each package's manifest names its wheels under `wheels = [ … ]` and names no `platforms`. Every package built, exit status 0.

**All three wheels in one package (`smoozy_wheels`).** `blender --command extension install-file -r user_default -e <work>/pkg/smoozy_wheels-1.0.0.zip` printed `STATUS Installed "smoozy_wheels"` and the add-on's own `SMOOZY_WHEELS REGISTERED`, exit status 0. `find` on the extensions folder then printed:

```
<work>/res/09-smoozy_wheels/extensions
<work>/res/09-smoozy_wheels/extensions/.cache
<work>/res/09-smoozy_wheels/extensions/.cache/compat.dat
<work>/res/09-smoozy_wheels/extensions/.local
<work>/res/09-smoozy_wheels/extensions/.local/lib
<work>/res/09-smoozy_wheels/extensions/.local/lib/python3.13
<work>/res/09-smoozy_wheels/extensions/.local/lib/python3.13/site-packages
<work>/res/09-smoozy_wheels/extensions/.local/lib/python3.13/site-packages/smoozypkg
<work>/res/09-smoozy_wheels/extensions/.local/lib/python3.13/site-packages/smoozypkg-1.0.0.dist-info
<work>/res/09-smoozy_wheels/extensions/.local/lib/python3.13/site-packages/smoozypkg-1.0.0.dist-info/METADATA
<work>/res/09-smoozy_wheels/extensions/.local/lib/python3.13/site-packages/smoozypkg-1.0.0.dist-info/RECORD
<work>/res/09-smoozy_wheels/extensions/.local/lib/python3.13/site-packages/smoozypkg-1.0.0.dist-info/WHEEL
<work>/res/09-smoozy_wheels/extensions/.local/lib/python3.13/site-packages/smoozypkg/__init__.py
<work>/res/09-smoozy_wheels/extensions/user_default
<work>/res/09-smoozy_wheels/extensions/user_default/.blender_ext
<work>/res/09-smoozy_wheels/extensions/user_default/smoozy_wheels
<work>/res/09-smoozy_wheels/extensions/user_default/smoozy_wheels/__init__.py
<work>/res/09-smoozy_wheels/extensions/user_default/smoozy_wheels/__pycache__
<work>/res/09-smoozy_wheels/extensions/user_default/smoozy_wheels/__pycache__/__init__.cpython-313.pyc
<work>/res/09-smoozy_wheels/extensions/user_default/smoozy_wheels/blender_manifest.toml
<work>/res/09-smoozy_wheels/extensions/user_default/smoozy_wheels/wheels
<work>/res/09-smoozy_wheels/extensions/user_default/smoozy_wheels/wheels/smoozypkg-1.0.0-cp313-cp313-macosx_11_0_arm64.whl
<work>/res/09-smoozy_wheels/extensions/user_default/smoozy_wheels/wheels/smoozypkg-1.0.0-cp313-cp313-manylinux_2_28_x86_64.whl
<work>/res/09-smoozy_wheels/extensions/user_default/smoozy_wheels/wheels/smoozypkg-1.0.0-cp313-cp313-win_amd64.whl
```

and `blender --background --python wheel_import.py` — printing the switched-on modules, then importing `smoozypkg` and printing its `TAG` and its file, or the failure — printed, exit status 0:

```
SMOOZY_WHEELS REGISTERED
MODULES ['bl_ext.user_default.smoozy_wheels', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
IMPORTED cp313 macosx_11_0_arm64 <work>/res/09-smoozy_wheels/extensions/.local/lib/python3.13/site-packages/smoozypkg/__init__.py
```

So all three wheel files stayed, unopened, in the add-on's own `wheels` folder; one copy of `smoozypkg` was unpacked, into `extensions/.local/lib/python3.13/site-packages/`; and the copy that imports is the one from the macOS-on-Apple-silicon wheel.

**Only the Linux wheel (`smoozy_linwheel`), or only the Windows one (`smoozy_winwheel`).** Each install printed `STATUS Installed "<id>"` and the add-on's own register line, exit status 0, and nothing else. In each settings folder the wheel sat unopened in the add-on's own `wheels` folder and there was **no `.local` folder at all**, for example:

```
<work>/res/09-smoozy_linwheel/extensions/user_default/smoozy_linwheel/wheels
<work>/res/09-smoozy_linwheel/extensions/user_default/smoozy_linwheel/wheels/smoozypkg-1.0.0-cp313-cp313-manylinux_2_28_x86_64.whl
```

The add-on was switched on — `MODULES` held `bl_ext.user_default.smoozy_linwheel`, and `bl_ext.user_default.smoozy_winwheel` in the other folder — and importing its module failed, exit status 0 in both:

```
IMPORT FAILED ModuleNotFoundError No module named 'smoozypkg'
```

---

## 5. How can an add-on be found, fetched and installed without a person at a screen, and what decides the places it can be fetched from?

**Label:** **documented** for the commands and for what governs the places. **Inferred** where marked.

**Finding, fetching, installing by command.**
> "list: List all packages." / "blender --command extension list [-h] [-s]" / "List packages from all enabled repositories."
> "sync: Synchronize with remote repositories." / "Download package information for remote repositories."
> "install: Install packages." / "PACKAGES: The packages to operate on (separated by , without spaces)." / "-s, --sync Sync the remote directory before performing the action." / "-e, --enable Enable the extension after installation."
> "update: Upgrade any outdated packages."
> "repo-add … [--url URL] [--access-token ACCESS_TOKEN] [--source SOURCE] … Add a new local or remote repository."

— manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/extension_arguments.html, read 2026-10-11.
**Inferred:** there is no search command; "finding" means listing what the enabled repositories hold and picking from it. `install` takes package names from those repositories, while files go through `install-file`.

**What decides the places.**
1. The repositories set in the preferences.
   > "By default Blender has a Remote Repository pointing towards the Official Blender Extensions Platform and two Local Repositories. In the cases where more repositories are needed (e.g., to access third party extension platforms), new repositories can be added."

   — https://docs.blender.org/manual/en/5.3/editors/preferences/extensions.html, read 2026-10-11.
   The picture `editor_preferences_section_extensions_repositories.png` on the same page shows three repositories — "extensions.blender.org", "User Default" and "System" — with the address "https://extensions.blender.org/api/v1/extensions/" and "Check for Updates on Startup" ticked. **Documented** (picture).
   > "Other third party sites can also be supported, as long as they follow the Extensions Platform specification."

   — https://docs.blender.org/manual/en/5.3/advanced/extensions/index.html, read 2026-10-11.
   > "Remote URL to the extension repository, the file-system may be referenced using the file URI scheme: “file://”"

   — https://docs.blender.org/api/5.3/bpy.types.UserExtensionRepo.html, read 2026-10-11.
2. Whether Blender may go online.
   > "By default Blender does not access the internet, however this can be enabled in the System preferences with the Online Access option. Working offline can be enforced by running with the --offline-mode command line argument. Users will then be unable to enable online access in the preferences."

   — https://docs.blender.org/manual/en/5.3/advanced/deploying_blender.html, read 2026-10-11.
3. Access tokens.
   > "Some repositories may require authentication. The user can specify an access token for a repository, which is passed along with the API request from Blender."

   — https://docs.blender.org/manual/en/5.3/advanced/extensions/creating_repository/dynamic_repository.html, read 2026-10-11.
4. What the repository offers this Blender and this system: "platform" and "blender_version" are passed "to make sure only compatible extensions are listed" (same page). A self-hosted listing can also block extensions: "This can be used to defined blocked extensions" (https://docs.blender.org/manual/en/5.3/advanced/command_line/extension_arguments.html, `server-generate`, read 2026-10-11).
5. The network.
   > "On systems or networks that use an additional layer of SSL management: it’s possible Blender’s connection to https://extensions.blender.org is blocked."

   — https://docs.blender.org/manual/en/5.3/troubleshooting/network.html, read 2026-10-11.
6. Download links for drag-and-drop: "The only strict requirement is that the download URL must end in .zip." (https://docs.blender.org/manual/en/5.3/advanced/extensions/creating_repository/static_repository.html, read 2026-10-11). This one is interface-only.

**Not covered.** Whether the extension commands obey the Online Access setting or need `--online-mode`. Which repository `install` uses when two hold the same name. What the `list` output looks like.

**Where I looked.** Beyond the general searches: "online", "offline", "access token", "Authorization", "sync", "remote", "file://", "403", "SSL", "blocklist". Pages read: the extension command-line page, the Get Extensions preference page and its two pictures, the extensions index, all three repository pages, the deploying page, the network troubleshooting page, and the API pages for `UserExtensionRepo`, `PreferencesExtensions` and the extensions operators.

**Outside the interface.** **Settled by test — observed, on the Creator's Mac, 2026-10-17; see the Observed sections below.** Every command listed ran with no window. A sync with the online-access preference off and no switch printed `RuntimeError: Operator bpy.ops.extensions.repo_sync_all.poll() Online access required to check for updates. Enable online access in System preferences` and exited 1, for the official repository and for a repository that is a folder on the same machine alike; with `--online-mode`, or with the preference saved as on, it went on to the address. Installing by name from a folder repository already synced worked with the preference off and no switch. *What the sources say:* every command above runs without a window. The interface-only routes are dragging a URL and clicking Install.

**Operating system.** **Tested on macOS on Apple silicon only (2026-10-17, below).** The `file://` form the sources give for Linux and macOS was used; the Windows forms were not tried, this sitting having no Windows machine. *What the sources say:* file addresses for local repositories differ:
> "file:///path/to/packages/index.json on Linux and macOS."
> "file:///C:/path/to/packages/index.json on Windows."
> "file://HOST/share/path/to/packages/index.json for network shares on Windows."

— https://docs.blender.org/manual/en/5.3/advanced/extensions/creating_repository/static_repository.html, read 2026-10-11. Listings are filtered by system (point 4).

### Observed — what the extension commands offer and print, from a new settings folder

**Label: observed.** Entry 1 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new, empty settings folder — see *How the hand tests were run* at the top of this document.

**The repositories a new settings folder holds.** `blender --command extension repo-list` printed, exit status 0:

```
blender_org:
    name: "extensions.blender.org"
    directory: "<work>/res/01/extensions/blender_org"
    url: "https://extensions.blender.org/api/v1/extensions/"
    access_token: None
user_default:
    name: "User Default"
    directory: "<work>/res/01/extensions/user_default"
    source: "USER"
system:
    name: "System"
    directory: "/Applications/Blender 5.3b.app/Contents/Resources/5.3/extensions/system"
    source: "SYSTEM"
```

**What `extension list` prints before anything is fetched or installed**, exit status 0:

```
[Errno 2] No such file or directory: '<work>/res/01/extensions/blender_org'
Repository data: <work>/res/01/extensions/blender_org not found, sync required!
Repository: "extensions.blender.org" (id=blender_org)
[Errno 2] No such file or directory: '<work>/res/01/extensions/user_default'
Repository: "User Default" (id=user_default)
Repository: "System" (id=system)
```

**The commands on offer.** `blender --command extension --help` printed, exit status 0:

```
usage: blender --command extension [-h]
                                   {server-generate,build,validate,list,sync,update,install,install-file,remove,repo-list,repo-info,repo-add,repo-remove} ...

Command for managing Blender extensions.

options:
  -h, --help            show this help message and exit

subcommands:

  {server-generate,build,validate,list,sync,update,install,install-file,remove,repo-list,repo-info,repo-add,repo-remove}
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
    repo-info           Show repository information.
    repo-add            Add repository.
    repo-remove         Remove repository.
```

Thirteen subcommands. Asked for their own help, each exit status 0:

```
usage: blender --command extension install [-h] [-s] [-e] [--no-prefs]
                                           PACKAGES

positional arguments:
  PACKAGES      The packages to operate on (separated by ``,`` without spaces).

options:
  -h, --help    show this help message and exit
  -s, --sync    Sync the remote directory before performing the action.
  -e, --enable  Enable the extension after installation.
  --no-prefs    Treat the user-preferences as read-only,
                preventing updates for operations that would otherwise modify them.
                This means removing extensions or repositories for example, won't update the user-preferences.
```

```
usage: blender --command extension install-file [-h] -r REPO [-e] [--no-prefs]
                                                FILE

Install a package file into a user repository.

positional arguments:
  FILE             The packages file.

options:
  -h, --help       show this help message and exit
  -r, --repo REPO  The repository identifier.
  -e, --enable     Enable the extension after installation.
  --no-prefs       Treat the user-preferences as read-only,
                   preventing updates for operations that would otherwise modify them.
                   This means removing extensions or repositories for example, won't update the user-preferences.
```

```
usage: blender --command extension sync [-h]

Download package information for remote repositories.

options:
  -h, --help  show this help message and exit
```

```
usage: blender --command extension list [-h] [-s]

List packages from all enabled repositories.

options:
  -h, --help  show this help message and exit
  -s, --sync  Sync the remote directory before performing the action.
```

```
usage: blender --command extension repo-info [-h] [-s] ID

Show high level information about a repository,
using the listing downloaded by the last "sync".

positional arguments:
  ID          The repository identifier.

options:
  -h, --help  show this help message and exit
  -s, --sync  Sync the remote directory before performing the action.
```

The help for `install` names one positional argument, `PACKAGES`, and no address; the help for `sync` names no argument but `-h`; no subcommand searches.

**Each repository's own settings, read from the preferences.** A script printed, for each repository, its name, its identifier, whether it is switched on, whether it has a remote address, that address, whether it uses an access token, whether it uses a custom folder, whether it checks for updates at start-up, and whether it keeps a cache; then the settings a repository carries, and those of the extensions preferences:

```
REPOS [('extensions.blender.org', 'blender_org', True, True, 'https://extensions.blender.org/api/v1/extensions/', False, False, True, False), ('User Default', 'user_default', True, False, '', False, False, False, False), ('System', 'system', True, False, '', False, False, False, False)]
REPO PROPS ['access_token', 'custom_directory', 'directory', 'enabled', 'module', 'name', 'remote_url', 'rna_type', 'show_expanded', 'source', 'use_access_token', 'use_access_token_for_asset_libraries', 'use_cache', 'use_custom_directory', 'use_remote_url', 'use_sync_on_startup']
EXT PREFS ['active_repo', 'repos', 'rna_type', 'use_online_access_handled']
ONLINE HANDLED False
```

Exit status 0. After all of these runs, `find` on the settings folder printed the folder itself and nothing else: listing and reading wrote nothing.

**Outside the interface.** Every command here ran with no window.

**Operating system.** macOS on Apple silicon only. The System repository's folder sits inside the application bundle on this Mac.

### Observed — what gates a fetch: the online-access preference and the command-line switches

**Label: observed.** Entry 6 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new, empty settings folder — see *How the hand tests were run* at the top of this document. **Nothing was fetched.** The two runs that went past the gate were made inside a macOS network block — `sandbox-exec` with a profile that allows everything except outgoing network connections — so that a fetch could only fail at the network.

**The online-access values.** `online.py` prints `bpy.app.online_access`, whether `bpy.app` has `online_access_override` and `online_access_overriden`, the value of `online_access_override`, and the preference `bpy.context.preferences.system.use_online_access`. In the new folder, with no switch, with `--online-mode` and with `--offline-mode`, each exit status 0:

```
--- blender --background --python online.py
ACCESS False
HAS_OVERRIDE True
HAS_OVERRIDEN False
OVERRIDE False
PREF False
--- blender --background --online-mode --python online.py
ACCESS True
HAS_OVERRIDE True
HAS_OVERRIDEN False
OVERRIDE True
PREF False
--- blender --background --offline-mode --python online.py
ACCESS False
HAS_OVERRIDE True
HAS_OVERRIDEN False
OVERRIDE True
PREF False
```

**A sync with the preference off.** `blender --command extension sync` printed, exit status 1:

```
Traceback (most recent call last):
  File "/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/bl_pkg/bl_extension_cli.py", line 92, in sync
    bpy.ops.extensions.repo_sync_all()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^^
RuntimeError: Operator bpy.ops.extensions.repo_sync_all.poll() Online access required to check for updates. Enable online access in System preferences
Error synchronizing
```

`blender --offline-mode --command extension sync` printed the same traceback ending `Online access required to check for updates. Launch Blender without --offline-mode` and `Error synchronizing`, exit status 1. `blender --command extension list -s` printed the same traceback as the plain sync, ending `Enable online access in System preferences` and `Error synchronizing`, exit status 1.

**The preference saved as off, then a sync with and without `--online-mode`.** A run setting `use_online_access = False` and saving printed `PREF False`, `Info: Preferences saved`, `SAVE {'FINISHED'}`, exit status 0. `blender --command extension sync` then printed the same refusal as above, exit status 1. Inside the network block, `blender --online-mode --command extension sync` printed, exit status **0**:

```
STATUS Checking repository "extensions.blender.org" for updates...
STATUS Refreshing extensions list for "extensions.blender.org"...
FATAL_ERROR sync: URL error (<urlopen error [Errno 1] Operation not permitted>) reading 'https://extensions.blender.org/api/v1/extensions/'!
```

**The preference saved as on, and a sync with no switch.** A run setting `use_online_access = True` and saving printed `PREF True`, `Info: Preferences saved`, `SAVE {'FINISHED'}`, exit status 0. The values then read, exit status 0:

```
ACCESS True
HAS_OVERRIDE True
HAS_OVERRIDEN False
OVERRIDE False
PREF True
```

and inside the network block, `blender --command extension sync` with no switch printed the same three lines as the `--online-mode` run above — `STATUS Checking repository …`, `STATUS Refreshing extensions list …`, `FATAL_ERROR sync: URL error (<urlopen error [Errno 1] Operation not permitted>) reading 'https://extensions.blender.org/api/v1/extensions/'!` — exit status **0**. Afterwards the settings folder held `extensions/blender_org/.blender_ext`, which no earlier run had made.

So with the preference off and no switch, the sync printed the online-access refusal and exited 1 — and those runs were made with no network block in place. With `--online-mode`, or with the preference saved as on, it went on to read the address, and with the network blocked it printed `FATAL_ERROR` and exited 0.

**A repository that is a folder on this machine.** Entry 8, recorded in full under question 8: a `file://` repository was refused a sync in the same words with the preference off and no switch, and synced with `--online-mode` — with `extensions.blender.org` failing at the blocked network and the folder repository's listing updated in the same run, exit status 0. **Once synced, `blender --command extension install smoozy_test -e` installed from it by name with the preference off and no switch**, exit status 0. Asked by name for the Windows-only package that sits in the same folder, `install` printed `Package "smoozy_win_only" not found in remote repositories!`, exit status 1. The listing of the folder repository showed `smoozy_test: "Smoozy Test", A tiny test add-on` before it was installed — no `[installed]` — and `smoozy_test [installed]: …` after.

### Observed — the Get Extensions section, its two menus, and the System section, at the window

**Label: observed.** Entry 19 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, **with Blender's window open on the Mac's screen**, started from a terminal, driven by a script and photographed with Blender's own `bpy.ops.screen.screenshot`, each picture after a forced redraw. Six runs, each in a new settings folder whose preferences had been saved once with the splash switched off; in each the largest editor was turned into the Preferences editor. What is written below is what is visible in the pictures.

**The Get Extensions section in a new settings folder** (the online-access question not yet answered: `use_online_access_handled` read `False`, `use_online_access` `False`). Along the top: a `Search Extensions` box, a drop-down reading `Add-ons`, a filter icon with a drop-down, and a drop-down reading `Repositories`. Below them an open section headed `Online Extensions` holding three lines — `Internet access is required to install and update online extensions.`, `You can adjust this later from "System" preferences.`, `While offline, use "Install from Disk" instead.` with a globe icon — and two buttons, `Continue Offline` and `Allow Online Access`.

**The same with the online-access question counted as answered** (`use_online_access_handled` set to `True`, read back `True`): the `Online Extensions` section is gone, and a box headed `Repository Alert:` with a warning triangle and a close cross reads `Repository: "extensions.blender.org" remote data unavailable, either allow "Online Access" or disable the repository to suppress this message`. No extension list.

**The repositories pop-over**, opened with `bpy.ops.wm.call_panel('INVOKE_DEFAULT', name='USERPREF_PT_extensions_repos', keep_open=True)` (returned `{'INTERFACE'}`): three rows, each with an icon and a ticked box — `extensions.blender.org` (globe icon, highlighted as the selected one), `User Default`, `System` — with `+`, `−`, a greyed refresh button and a drop-down beside them; under the list the address `https://extensions.blender.org/api/v1/extensions/` with a globe icon, a ticked `Check for Updates on Startup`, and a closed section `Advanced`.

**The active-repository menu**, opened with `bpy.ops.wm.call_menu('INVOKE_DEFAULT', name='USERPREF_MT_extensions_active_repo')` (returned `{'INTERFACE'}`): a heading `Active Repository` and one greyed row with a refresh icon, partly covered by its tooltip, which reads `Refresh Remote`, `Refresh the list of extensions for the active repository.` and, in red, `Online access required to check for updates. Enable online access in System preferences.` Nothing else of the menu is visible.

**The settings menu**, opened with `bpy.ops.wm.call_menu('INVOKE_DEFAULT', name='USERPREF_MT_extensions_settings')` (returned `{'INTERFACE'}`): a heading `Extension Settings` and five entries — `Visit Extensions Platform` (its tooltip: `Open a preset website in the web browser.` `Online directory of free and open source extensions.`), `Refresh Remote` (greyed), `Refresh Local`, `Install Available Updates` (greyed) and `Install from Disk...`.

**The System section**: a section `Cycles Render Devices` with `None | Metal` (`Metal` selected), a ticked `Apple M1 Pro (GPU - 14 cores)`, an unticked `Apple M1 Pro`, `Kernel Optimization  Full`, `MetalRT  Auto`; then a section `Network` holding an **unticked `Allow Online Access`**, `Time Out  10` and `Connection Limit  5`; then `Memory & Limits`, `Video Sequencer` and a closed `Sound`.

Every identifier used was read out of `bpy.types` first. A script cannot open a section drawn closed, so what the repositories pop-over's `Advanced` section holds was not seen; its settings, read from the preferences, are under question 8.

---

## 6. What does an add-on belong to, once it is installed on a computer?

**The question is ambiguous.** "Belong to" could mean: (a) the store that holds it, (b) the Blender version, (c) the user account. The sources speak to all three, so I answer all three.

**Label:** **documented** for each part. The summary at the end is **inferred**.

(a) **A repository.**
> "Install from Disk — Install an extension from a .zip package. This is installed to a Local Repository and no updates will be available."

— https://docs.blender.org/manual/en/5.3/editors/preferences/addons.html, read 2026-10-11.
> "To prevent this conflict, the repository name is now part of the namespace. For example, now instead of kitsu the module name would be bl_ext.{repository_module_name}.kitsu instead."

— https://docs.blender.org/manual/en/5.3/advanced/extensions/addons.html, read 2026-10-11.
Repositories are of two kinds:
> "USER User – Repository managed by the user, stored in user directories." / "SYSTEM System – Read-only repository provided by the system."

— https://docs.blender.org/api/5.3/bpy.types.UserExtensionRepo.html, read 2026-10-11.
> "Remove Repository & Files — Remove a repository and delete all associated files when removing."

— https://docs.blender.org/manual/en/5.3/editors/preferences/extensions.html, read 2026-10-11.
Older-style add-ons belong to a scripts folder instead: "./scripts/addons/*.py", and Blender's own bundled ones to "./scripts/addons_core/*.py — The add-ons directory which is used for bundled add-ons." (https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html, read 2026-10-11).

(b) **A Blender version.**
> "User directories store preferences, startup file, installed extensions, presets and more."
> Linux "$HOME/.config/blender/5.3/" · macOS "/Users/$USER/Library/Application Support/Blender/5.3/" · Windows "%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.3\"

— https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html, read 2026-10-11.
In a portable setup, the installation itself: "This folder will then store preferences, startup file, installed extensions and presets." (same page). The location can be redirected: "BLENDER_USER_EXTENSIONS: Directory for user extensions." (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11).

(c) **A user account, and that user's preferences.** The user folders above sit under the user's home folder. Whether an add-on is switched on is held in the preferences: "The Preferences file stores keymap, add-ons theme and other options." (https://docs.blender.org/manual/en/5.3/getting_started/configuration/defaults.html, read 2026-10-11). Application templates can have their own: "Only certain preferences from a template are used: Themes. Add-ons. Keymaps. Viewport lighting." (https://docs.blender.org/manual/en/5.3/advanced/app_templates.html, read 2026-10-11). Each extension can also have its own data folder:
> "This directory will be kept between upgrades but will be removed if the extension is uninstalled."

— https://docs.blender.org/manual/en/5.3/advanced/extensions/addons.html, read 2026-10-11.

**Summary (inferred).** Unless redirected, an installed extension belongs to one repository, inside one Blender version's user folder, for one user account. Whether it is switched on belongs to that user's preferences for that version — or to an application template's preferences when one is in use.

**Not covered.** Whether two OS user accounts on one computer can share a user repository (only the System repository is described as shared). What happens to an extension's own data folder when its repository is removed.

**Where I looked.** Beyond the general searches: "repository", "namespace", "bl_ext", "user directories", "system directories", "portable", "BLENDER_USER", "BLENDER_SYSTEM", "app template". Pages read: the directory layout page, the deploying page, the add-ons extension page, both preference pages, the application templates page, the defaults page, and the API pages for `UserExtensionRepo` and `bpy.utils`.

**Outside the interface.** **Settled by test — observed, on the Creator's Mac, 2026-10-17; see the Observed sections below.** With no window, an installed extension landed in its repository's folder under the settings folder, was known as `bl_ext.<repository>.<id>` (`bl_ext.user_default.smoozy_test`, `bl_ext.smoozy_local.smoozy_test`, `bl_ext.system.smoozy_ok`), had its writable folder handed out under `extensions/.user/`, and had its switch-on saved in the settings folder's `config/userpref.blend` — except once, when the folder `BLENDER_USER_RESOURCES` named did not exist and the preferences went to the default user folder. *What the sources say:* holds the same way. The command line names repositories by identifier (`-r REPO`, `repo-add … ID`), and "System repositories are managed outside of Blender and are considered read-only." (https://docs.blender.org/manual/en/5.3/advanced/command_line/extension_arguments.html, read 2026-10-11).

**Operating system.** **Tested on macOS on Apple silicon only (2026-10-17, below);** Linux and Windows were not tested in this sitting, which had no such machine. *What the sources say:* the ownership structure is the same everywhere. The folder paths differ by system, as quoted. For portable setups: "macOS: Inside the application bundle at Blender.app/Contents/Resources"; Windows and Linux: "Next to the Blender executable, in the unzipped folder" (directory layout page).

### Observed — which repositories a new settings folder holds, and where

**Label: observed.** Entry 1 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new, empty settings folder — see *How the hand tests were run* at the top of this document.

`blender --command extension repo-list` printed three repositories (full output under question 5): `blender_org`, with its directory inside the settings folder; `user_default`, `source: "USER"`, with its directory inside the settings folder; and `system`, `source: "SYSTEM"`, with its directory at `/Applications/Blender 5.3b.app/Contents/Resources/5.3/extensions/system` — inside the application. Exit status 0. The settings folder was still empty after it.

### Observed — where an installed extension lands, and the name it is known by

**Label: observed.** Entry 2 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new, empty settings folder — see *How the hand tests were run* at the top of this document. The test add-on and its install are described under question 3.

**Where it lands.** After `blender --command extension install-file -r user_default -e <work>/pkg/smoozy_test-1.0.0.zip` and one later run, `find` on the settings folder printed, exit status 0:

```
<work>/res/02
<work>/res/02/config
<work>/res/02/config/userpref.blend
<work>/res/02/extensions
<work>/res/02/extensions/.cache
<work>/res/02/extensions/.cache/compat.dat
<work>/res/02/extensions/user_default
<work>/res/02/extensions/user_default/.blender_ext
<work>/res/02/extensions/user_default/smoozy_test
<work>/res/02/extensions/user_default/smoozy_test/__init__.py
<work>/res/02/extensions/user_default/smoozy_test/__pycache__
<work>/res/02/extensions/user_default/smoozy_test/__pycache__/__init__.cpython-313.pyc
<work>/res/02/extensions/user_default/smoozy_test/blender_manifest.toml
```

`diff` between the packed manifest and the one on disk printed nothing, and `SAME`.

**The folders Blender hands out.** `blender --background --python paths.py`, printing `bpy.utils.user_resource('EXTENSIONS')`, `user_resource('CONFIG')`, `user_resource('SCRIPTS')`, `resource_path('USER')`, `resource_path('LOCAL')`, `resource_path('SYSTEM')` and `bpy.utils.extension_path_user('bl_ext.user_default.smoozy_test', path='', create=False)`, printed, exit status 0:

```
SMOOZY TEST ADDON REGISTERED
EXT <work>/res/02/extensions
CONFIG <work>/res/02/config
SCRIPTS <work>/res/02/scripts
USER <work>/res/02
LOCAL /Applications/Blender 5.3b.app/Contents/Resources/5.3
SYSTEM /Library/Application Support/Blender/5.3
EXTPATH <work>/res/02/extensions/.user/user_default/smoozy_test
SMOOZY TEST ADDON UNREGISTERED
```

**Its name carries its repository.** It is switched on as `bl_ext.user_default.smoozy_test` (question 3), and `addon_utils.modules()` lists it under the same name with a metadata block made from its manifest:

```
MODULE bl_ext.user_default.smoozy_test {'name': 'Smoozy Test', 'author': 'Smoozy Test', 'version': (1, 0, 0), 'blender': (4, 2, 0), 'location': '', 'description': 'A tiny test add-on', 'doc_url': '', 'support': 'COMMUNITY', 'category': 'Development', 'warning': '', 'show_expanded': False}
```

(the one line of `inventory.py`'s output naming the add-on, picked out with `grep -i smoozy`, beside the add-on's own register and unregister lines).

**`BLENDER_USER_RESOURCES` moved all of it**: every path above inside the settings folder is under the folder that variable named.

### Observed — a settings folder that does not exist: where the switch-on was saved

**Label: observed**, by accident, during entry 14 of the hand tests, 2026-10-17, on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2), in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window. It was an error in the test list, not a planned test, and it was **not repeated**, because repeating it writes into the Creator's own settings.

**What was done.** Two runs were pointed with `BLENDER_USER_RESOURCES` at two folders, `<work>/res/14dA` and `<work>/res/14dB`, **that had not been created**, each also given `BLENDER_USER_EXTENSIONS=<work>/out/14/sharedext`, a folder that did exist. Asked for its folders, Blender printed for the first:

```
FOLDER USER <work>/res/14dA
FOLDER CONFIG <work>/res/14dA/config
FOLDER EXTENSIONS <work>/out/14/sharedext
FOLDER SCRIPTS <work>/res/14dA/scripts
```

Then `blender --command extension install-file -r user_default -e <work>/pkg/smoozy_ok-1.0.0.zip` printed `STATUS Installed "smoozy_ok"`, the add-on's two lines, exit status 0.

**What came back.** The extension's files were in `<work>/out/14/sharedext/user_default/smoozy_ok`. **`<work>/res/14dA` still did not exist.** A preferences file had appeared at **`/Users/artist/Library/Application Support/Blender/5.3/config/userpref.blend`** — the default 5.3 user folder on this Mac — 182 286 bytes, written at the time of the install; that folder had held no `userpref.blend` at the start of the sitting. A following run pointed at the equally missing `<work>/res/14dB` printed `SMOOZY OK ADDON REGISTERED` before anything else and `SMOOZY_ADDONS_AT_START ['bl_ext.user_default.smoozy_ok']`. The stray file was then moved out of the Creator's folder into the work folder, leaving that folder as it had been.

So, with the folder `BLENDER_USER_RESOURCES` named missing, the paths Blender printed were inside it, while the preferences it wrote, and then read, were in the default user folder.

---

## 7. What does a render do when an add-on it needs is not there?

**Label:** **not in the sources** as a direct answer. Some pieces are **documented**, and one step is **inferred**, as marked.

*That label answers from the sources only. The behaviour has since been tested — **observed**, on the Creator's Mac on 2026-10-17, entry 10 of the hand tests — and the result is in the Observed section at the end of this answer.*

The sources never describe a render of a file whose add-on is missing: not what happens to the engine choice, not whether the render stops or carries on, not what message appears, not the exit result. The pieces that are there:

1. Saved values outlive their add-on's definition: the PropertyGroup quote under question 1. What a render does with such values is not stated.
2. Drivers that call a missing function:
   > "Using The --python command line argument to populate name-space often fails to achieve the desired goal because the initial evaluation will lookup a function that doesn’t exist yet, marking the driver as invalid - preventing further evaluation."

   — https://docs.blender.org/api/5.3/bpy.app.html, read 2026-10-11. **Inferred:** if a driver calls a function an add-on would have supplied, and the add-on is absent, the driver is marked invalid and stops being evaluated. The page says this about timing, not about add-ons.
3. Node trees: an "Undefined" kind exists, "can happen e.g. when a linked node tree goes missing" (https://docs.blender.org/api/5.3/bpy.types.NodeTree.html, read 2026-10-11). The page does not say this is what happens when an add-on is missing.
4. Engines on offer can be listed: "-E, --engine <engine> Specify the render engine. Use -E help to list available engines." (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11).
5. For add-ons that fail to switch on: "If the Add-on does not activate when enabled, check the Console window for any errors that may have occurred." (https://docs.blender.org/manual/en/5.3/editors/preferences/addons.html, read 2026-10-11).

**Not covered.** All of the behaviour asked about.

**Where I looked.** Beyond the general searches: "missing", "not found", "not available", "not installed", "unavailable", "fallback", "falls back", "unknown engine", "undefined", "invalid", "warning", "error" near "engine" and "add-on", "exit code". Pages read: the render engines and render introduction pages, the command-line render and arguments pages, the RenderSettings and RenderEngine API pages, the drivers pages (panel, troubleshooting, workflow examples), the custom-properties page, the scripting security page, the troubleshooting section, and all 5.3 release notes pages.

**Outside the interface.** **Settled by test for a render driven from outside its window — observed, on the Creator's Mac, 2026-10-17; see the Observed section below.** A render started with no window, of a file whose engine's add-on was switched off or never installed, printed `ERROR Engine '…' not available for scene 'Scene' (an add-on may need to be installed or enabled)`, rendered with EEVEE, wrote an image and exited 0; forcing the engine with `-E` printed `Error: engine not found '…'`, wrote no image and exited 1; a missing add-on that is not an engine printed nothing at all. And `--factory-startup` gave the eight built-in add-ons from a settings folder whose saved preferences held a ninth (question 3, entry 4). At the window (entry 21), the same file opened with its engine field reading `EEVEE`, the same sentence in a red bar along the bottom and not on the terminal, and only EEVEE and Workbench offered. *What the sources say:* not stated. Two documented facts bear on it: add-ons switched on in the preferences load at start ("add-ons which are only loaded on startup if enabled from the user preferences", https://docs.blender.org/api/5.3/info_overview.html), and a window-less run still uses the preferences (security quote under question 3). **Inferred:** a window-less render loads the same switched-on add-ons as the interface would for that user and version. `--factory-startup` is described only as "Skip reading the startup.blend in the users home directory." (arguments page). But the bpy-module page says the module "behaves as if --factory-startup was passed" and that it ignores the user's preferences. Whether `--factory-startup` therefore also leaves add-ons switched off is **inferred**, not stated.

**Operating system.** **Settled for macOS on Apple silicon only, by test — observed, 2026-10-17, below.** Linux and Windows were not tested: this sitting had no such machine. *What the sources say:* nothing.

### Observed — what a render does when the add-on behind its engine, or another add-on it uses, is not there

**Label: observed.** Entry 10 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window — see *How the hand tests were run* at the top of this document. Two new settings folders: one where the add-ons below were switched on and off, and one where the two test add-ons were never installed. Renders were kept to 64 by 64 or 128 by 128.

**1. A file whose engine is Cycles, rendered with Cycles switched off.** A file `cycles.blend` was saved from the start-up scene with the engine set to Cycles, one sample, the processor as device, 64 by 64, PNG — the saving run printed `ENGINE CYCLES`, `SAMPLES 1`, `Info: Saved as "cycles.blend"`, exit status 0. Rendered once with Cycles on, `blender --background cycles.blend --render-output <work>/out/10/ok_ --render-frame 1` printed `Engine: Cycles` among its lines, ended `Saved: '<work>/out/10/ok_0001.png'`, and exited 0.

Cycles was then switched off and the preferences saved (`DISABLE {'FINISHED'}`, `Info: Preferences saved`, exit status 0). The same render command, writing `off_`, printed in full, exit status **0**:

```
00:00.322  blend            | Read blend: "<work>/out/10/cycles.blend"
00:00.326  reports          | ERROR Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
00:00.326  render           | Rendering single frame
00:00.326  render           | Rendering frame 1
00:00.326  render           | Start rendering: Scene, ViewLayer
00:00.326  render           | Engine: EEVEE
00:00.482  render           | Fra: 1 | Rendering 1 / 64 samples
00:00.792  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.958  render           | Fra: 1 | Rendering 50 / 64 samples
00:01.049  render           | Fra: 1 | Rendering 64 / 64 samples
00:01.142  render           | Saved: '<work>/out/10/off_0001.png'
00:01.142  render           | Time: 00:00.81 (Saving: 00:00.07)
```

The image is 64 by 64 (`sips` read `pixelWidth: 64`, `pixelHeight: 64`), 3 880 bytes against the 5 249 bytes of the Cycles one.

**2. The same file with the engine forced on the command line.** With Cycles still off, `blender --background cycles.blend -E CYCLES --render-output <work>/out/10/forced_ --render-frame 1` printed, exit status **1**:

```
00:00.454  blend            | Read blend: "<work>/out/10/cycles.blend"
00:00.459  reports          | ERROR Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Error: engine not found 'CYCLES'
```

and no `forced_` image was written. It was run three times, each exiting 1 with no image. The first two outputs were read through a filter that hid closing lines and through a byte dump that was cut short; the third was written to a file and is the one quoted here, every line.

**3. A file whose engine comes from an add-on that was never installed.** A test add-on `smoozy_engine`, whose whole content is a render engine with `bl_idname = "SMOOZY_TEST"` filling the frame with one colour, was built and installed with `-e` (`STATUS Installed "smoozy_engine"`, exit status 0). A file `engine.blend` was saved with that engine at 64 by 64, and rendered: the render printed `Engine: Smoozy Test`, `Saved: '<work>/out/10/engine_with_0001.png'`, exit status 0. The same file was then rendered in the second settings folder, where that add-on had never been, and printed, exit status **0**:

```
00:00.325  blend            | Read blend: "<work>/out/10/engine.blend"
00:00.329  reports          | ERROR Engine 'SMOOZY_TEST' not available for scene 'Scene' (an add-on may need to be installed or enabled)
00:00.329  render           | Rendering single frame
00:00.330  render           | Rendering frame 1
00:00.330  render           | Start rendering: Scene, ViewLayer
00:00.330  render           | Engine: EEVEE
00:00.462  render           | Fra: 1 | Rendering 1 / 64 samples
00:00.661  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.814  render           | Fra: 1 | Rendering 50 / 64 samples
00:00.906  render           | Fra: 1 | Rendering 64 / 64 samples
00:00.998  render           | Saved: '<work>/out/10/engine_without_0001.png'
00:00.998  render           | Time: 00:00.66 (Saving: 00:00.07)
```

Opened in that same folder and asked its engine, the file printed the same `ERROR Engine 'SMOOZY_TEST' not available …` line and then `ENGINE BLENDER_EEVEE`, exit status 0.

**4. An add-on that is not an engine but runs during a render.** A test add-on `smoozy_handler` adds a function to `bpy.app.handlers.render_pre`, marked to survive a file load, that prints `SMOOZY HANDLER RAN` and sets the scene's width to 64. It was installed with `-e` in the first folder. A file `plain.blend` was saved with the Workbench engine at 128 by 128. Rendered there, the run printed `Engine: Workbench`, `Saved: '<work>/out/10/handler_with_0001.png'` and, among its lines, `SMOOZY HANDLER RAN`, exit status 0. Rendered in the second folder, where the add-on had never been, it printed, exit status 0:

```
00:00.337  blend            | Read blend: "<work>/out/10/plain.blend"
00:00.341  render           | Rendering single frame
00:00.341  render           | Rendering frame 1
00:00.342  render           | Start rendering: Scene, ViewLayer
00:00.342  render           | Engine: Workbench
00:00.516  render           | Saved: '<work>/out/10/handler_without_0001.png'
00:00.516  render           | Time: 00:00.17 (Saving: 00:00.06)
```

— no handler line, no warning, nothing naming the add-on. Both images measured 128 by 128 (`sips`), and both files were 12 404 bytes.

**Outside the interface.** All four were run with no window; the messages went to the terminal.

**Operating system.** macOS on Apple silicon only.

### Observed — the file whose engine's add-on is off, opened at the window

**Label: observed.** Entry 21 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, **with Blender's window open on the Mac's screen**, started from a terminal with `cycles.blend` (entry 10: engine Cycles, one sample, 64 by 64), driven by a script and photographed with Blender's own `bpy.ops.screen.screenshot`, each picture after a forced redraw. The settings folder was new, with Cycles switched off and the preferences saved (`DISABLE {'FINISHED'}`, `Info: Preferences saved`) and the splash switched off.

The script printed, once the file was open:

```
ENGINE BLENDER_EEVEE
HAS_CYCLES False
REPORTS [('ERROR', "Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)")]
```

— the last line being `bpy.context.window_manager.reports`. With the window open, that error was not printed to the terminal; the terminal showed `Read blend: "<work>/out/10/cycles.blend"`, the script's lines and the closing lines, exit status 0.

**The Render tab.** The picture shows `Render Engine` reading **`EEVEE`**, with EEVEE's own settings below it — `Sampling`, `Viewport` `Samples 16`, `Temporal Reprojection` ticked, `Jittered Shadows` unticked, `Render` `Samples 64`, `Time Limit 0 s`, `Shadows` ticked, `Advanced`, then `Light Paths`, `Raytracing`, `Volumes`, `Curves`, `Simplify`, `Depth of Field`, `Motion Blur`, `Film`, `Performance`, `Grease Pencil`, `Freestyle`, `Color Management`. Along the bottom of the window, in the status bar, a red bar with an error icon reads **`Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)`**. It was there in the first picture, taken a few seconds after the file opened, before any render was asked for.

**The engine list**, opened with Blender's own `bpy.ops.wm.context_menu_enum('INVOKE_DEFAULT', data_path='scene.render.engine')`, which printed `SMOOZY_CONTEXT_MENU {'FINISHED'}`: a menu headed `Engine` holding exactly two entries, **`EEVEE`** (highlighted) and **`Workbench`**, with a tooltip `Engine to use for rendering: EEVEE`. Cycles is not among them. The red bar was still along the bottom.

**Outside the interface.** This is a window test; the no-window results are above.

**Operating system.** macOS on Apple silicon only.

---

## 8. Where do add-ons come from besides Blender's own, how are they sold, and what does that mean for taking a copy of one from one computer to another?

**Label:** where they come from — **documented**. How they are sold — **not in the sources**. Copying — **not in the sources** for the licence side; partly **inferred** for the technical side.

**Where they come from.**
> "Officially supported add-ons are bundled with Blender. Other add-ons are provided by Blender Extensions which aren’t part of official releases."

— https://docs.blender.org/manual/en/5.3/advanced/scripting/introduction.html, read 2026-10-11.
> "The official extensions platform for the Blender project is extensions.blender.org. Other third party sites can also be supported, as long as they follow the Extensions Platform specification."

— https://docs.blender.org/manual/en/5.3/advanced/extensions/index.html, read 2026-10-11.
> "Third party sites that wish to support extensions in Blender can do so in several ways, in order of complexity: Static … Dynamic … Platform: Fork the entire Extensions Website to create your own."

— https://docs.blender.org/manual/en/5.3/advanced/extensions/creating_repository/index.html, read 2026-10-11.
> "Blender comes with some preinstalled Add-ons already, ready to be enabled. But you can also add your own, or any interesting ones you find on the web."

— https://docs.blender.org/manual/en/5.3/editors/preferences/addons.html, read 2026-10-11.
Also: zip or `.py` files installed from disk, older-style add-ons in script folders, and System repositories on shared drives (questions 3 and 6).

**How they are sold.** Nothing in the sources describes selling, buying, paying, licence keys, activation, or licence servers for add-ons. What is said:
> "The Blender Extensions Platform only supports free and open source extensions compliant with the Blender’s license: For add-ons, the required license is GNU General Public License v3.0 or later."

— https://docs.blender.org/manual/en/5.3/advanced/extensions/licenses.html, read 2026-10-11.
> "Upload the .zip file (this step requires Blender ID). The extension will be held for review, and published once the moderation team approves it."

— https://docs.blender.org/manual/en/5.3/advanced/extensions/getting_started.html, read 2026-10-11.
Repositories can require a token: "Access Token — Personal access token, may be required by some repositories." (https://docs.blender.org/manual/en/5.3/editors/preferences/extensions.html, read 2026-10-11). The sources do not say what tokens are used for beyond authentication.

**Copying between computers.**
- On the licence: the manual's general GPL page says: "You have the right to copy and distribute the program." (https://docs.blender.org/manual/en/5.3/getting_started/about/license.html, read 2026-10-11). **Inferred:** add-ons from extensions.blender.org must carry GPL v3 or later, so that right applies to them. For add-ons from anywhere else, the sources are silent.
- On the technical side: installing from a file gives "no updates" (question 6). Compiled parts are "not portable between different platforms" (question 4). A token is described as "Personal" (above). **Inferred:** a copy taken from one computer loses the update link, may not run on another kind of computer, and the sources give no basis for reusing someone's token elsewhere.

**Not covered.** Sale, licensing of non-GPL add-ons, any copy protection, how tokens relate to purchases.

**Where I looked.** Beyond the general searches: "sell", "sold", "paid", "purchase", "commercial", "price", "marketplace", "subscription", "licence/license key", "activation", "license server", "DRM", "proprietary", "closed source", "access token", "Blender ID", "GPL", "SPDX". Pages read: the licences page, the extensions index, the repository pages, the getting-started page, the GPL page, the scripting introduction, both preference pages. All 5.3 release notes: nothing on sales.

**Outside the interface.** **Settled by test for where packages come from — observed, on the Creator's Mac, 2026-10-17; see the Observed sections below.** A repository was made from a plain folder, added, synced and installed from with no window, and a package installed from a file was offered no update. How add-ons are sold was not tested; no test on a computer shows it. *What the sources say:* no difference between inside and outside the interface for this question.

**Operating system.** **Tested on macOS on Apple silicon only (2026-10-17, below);** Linux and Windows were not tested in this sitting. *What the sources say:* only the technical point on compiled parts (question 4) differs by system.

### Observed — the repositories set up to begin with

**Label: observed.** Entry 1 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new, empty settings folder — see *How the hand tests were run* at the top of this document.

A new settings folder holds three repositories: `extensions.blender.org` with the address `https://extensions.blender.org/api/v1/extensions/` and `access_token: None`, `User Default` and `System` (full output of `blender --command extension repo-list` under question 5). Read from the preferences, none of the three uses an access token and only `extensions.blender.org` has an address:

```
REPOS [('extensions.blender.org', 'blender_org', True, True, 'https://extensions.blender.org/api/v1/extensions/', False, False, True, False), ('User Default', 'user_default', True, False, '', False, False, False, False), ('System', 'system', True, False, '', False, False, False, False)]
```

(each entry: name, identifier, switched on, has a remote address, the address, uses an access token, uses a custom folder, checks for updates at start-up, keeps a cache). A repository's settings include `use_access_token` and `use_access_token_for_asset_libraries`. Nothing about price, purchase or licence appears among them.

**The repositories at the window** (entry 19, with Blender's window open; full record under question 5): the repositories pop-over listed `extensions.blender.org`, `User Default` and `System`, each ticked, the address `https://extensions.blender.org/api/v1/extensions/`, a ticked `Check for Updates on Startup` and a closed `Advanced` section; nothing about price or purchase was visible.

**What the window shows about an installed extension's origin and licence** (entry 17, with Blender's window open; full record under question 13): the opened row of an extension whose manifest declares the licence `SPDX:GPL-3.0-or-later` and a copyright line shows `Website`, `Type  Extension`, `Maintainer`, `Version` and `File` — the file path running through `extensions/user_default/` — and no licence, no copyright and no repository name.

### Observed — a repository made from a plain folder, and updates for a package installed from a file

**Label: observed.** Entry 8 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and new, empty settings folders — see *How the hand tests were run* at the top of this document. The two packages used — `smoozy_test`, naming no platforms, and `smoozy_win_only`, naming only `windows-x64` — are described under questions 3 and 4. Every run that could have gone online was made inside the network block; the folder repository is a file on this Mac and needs no network.

**A listing made from a folder.** The two packages were copied into a folder `<work>/repo`. `blender --command extension server-generate --repo-dir=<work>/repo` printed `found 2 packages.`, exit status 0, and wrote `<work>/repo/index.json`:

```
{
  "version": "v1",
  "blocklist": [],
  "assetlib_auth_method": "PER_LIBRARY",
  "data": [
    {
      "schema_version": "1.0.0",
      "id": "smoozy_win_only",
      "name": "Smoozy Windows Only",
      "tagline": "A test add-on naming one platform",
      "version": "1.0.0",
      "type": "add-on",
      "maintainer": "Smoozy Test",
      "license": [
        "SPDX:GPL-3.0-or-later"
      ],
      "blender_version_min": "4.2.0",
      "platforms": [
        "windows-x64"
      ],
      "archive_url": "./smoozy_win_only-1.0.0.zip",
      "archive_size": 501,
      "archive_hash": "sha256:dd2fed163788e4d9f077ca55388abf7402d8cba77819dd6a60db224e677c3690"
    },
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
      "archive_hash": "sha256:2240cba0cdb2268524d9d9fbd3308221a749d11acf54e234c9430116e96b67cb"
    }
  ]
}
```

**Added as a repository.** `blender --command extension repo-add --name "Smoozy Local" --url "file://<work>/repo/index.json" smoozy_local` printed nothing, exit status 0, and `repo-list` then showed a fourth repository:

```
smoozy_local:
    name: "Smoozy Local"
    directory: "<work>/res/08/extensions/smoozy_local"
    url: "file://<work>/repo/index.json"
    access_token: None
```

**Synced, and what the listing then shows.** With the online-access preference off and no switch, `blender --command extension sync` printed the online-access refusal quoted under question 5, exit status 1. Inside the network block, `blender --online-mode --command extension sync` printed, exit status 0:

```
STATUS Checking repository "extensions.blender.org" for updates...
STATUS Refreshing extensions list for "extensions.blender.org"...
FATAL_ERROR sync: URL error (<urlopen error [Errno 1] Operation not permitted>) reading 'https://extensions.blender.org/api/v1/extensions/'!
STATUS Checking repository "Smoozy Local" for updates...
STATUS Refreshing extensions list for "Smoozy Local"...
PROGRESS ['Downloading...', 'BYTE', 0, 1170]
PROGRESS ['Downloading...', 'BYTE', 0, 1170]
STATUS Extensions list for "Smoozy Local" updated
```

and `blender --command extension list`, exit status 0:

```
Repository data: <work>/res/08/extensions/blender_org not found, sync required!
Repository: "extensions.blender.org" (id=blender_org)
Repository: "User Default" (id=user_default)
Repository: "System" (id=system)
Repository: "Smoozy Local" (id=smoozy_local)
  smoozy_test: "Smoozy Test", A tiny test add-on
```

**Installed from it by name.** Inside the network block, `blender --online-mode --command extension install smoozy_win_only -e` printed, exit status 1:

```
Package "smoozy_win_only" not found in remote repositories!
Repository data: <work>/res/08/extensions/blender_org not found, sync required!
```

and `blender --online-mode --command extension install smoozy_test -e` printed, exit status 0:

```
Repository data: <work>/res/08/extensions/blender_org not found, sync required!
PROGRESS ['Downloading "smoozy_test"', 'BYTE', 0, 459]
PROGRESS ['Downloading "smoozy_test"', 'BYTE', 0, 459]
STATUS Installed "smoozy_test"
SMOOZY TEST ADDON REGISTERED
SMOOZY TEST ADDON UNREGISTERED
```

after which the listing showed `smoozy_test [installed]: "Smoozy Test", A tiny test add-on` under Smoozy Local, and the switched-on list held it as `bl_ext.smoozy_local.smoozy_test`, `COUNT 9`.

**The same install without `--online-mode`.** *(Added while running.)* In a third new settings folder, the folder was added as a repository the same way and synced inside the network block with `--online-mode` (the same eight lines as above, exit status 0). Then, with the online-access preference still off and **no** switch, `blender --command extension install smoozy_test -e` printed, exit status 0:

```
Repository data: <work>/res/08n/extensions/blender_org not found, sync required!
PROGRESS ['Downloading "smoozy_test"', 'BYTE', 0, 459]
PROGRESS ['Downloading "smoozy_test"', 'BYTE', 0, 459]
STATUS Installed "smoozy_test"
SMOOZY TEST ADDON REGISTERED
SMOOZY TEST ADDON UNREGISTERED
```

and the listing showed it `[installed]` under Smoozy Local.

**Updates for a package installed from a file.** In a second new settings folder, `smoozy_test` was installed with `install-file -r user_default -e`. `blender --command extension update` then printed only the add-on's own two lines, `SMOOZY TEST ADDON REGISTERED` and `SMOOZY TEST ADDON UNREGISTERED`, exit status **1**. Inside the network block, `blender --online-mode --command extension update` printed, exit status 0:

```
SMOOZY TEST ADDON REGISTERED
Repository data: <work>/res/08u/extensions/blender_org not found, sync required!
SMOOZY TEST ADDON UNREGISTERED
```

Nothing was offered for the package installed from a file.

---

## 10. Can a computer be asked which add-ons it holds, and for which Blender?

**The question is ambiguous.** "Holds" could mean installed or switched on. "For which Blender" could mean which Blender version holds them, or which versions the add-on supports. I answer both.

**Label:** **documented** for the ways to ask. **Inferred** for "for which Blender".

**Ways to ask.**
> "blender --command extension list [-h] [-s] — List packages from all enabled repositories."
> "blender --command extension repo-list [-h] — List all repositories stored in Blender’s preferences."

— https://docs.blender.org/manual/en/5.3/advanced/command_line/extension_arguments.html, read 2026-10-11.
> "Enabled Add-Ons — Lists add-ons currently in use along with their versions and paths."

— https://docs.blender.org/manual/en/5.3/getting_started/help.html (the "Save System Info" report), read 2026-10-11. The same report from Python: `bpy.ops.wm.sysinfo(*, filepath='')` "Generate system information, saved into a text file" (https://docs.blender.org/api/5.3/bpy.ops.wm.html, read 2026-10-11).
> "Python add-ons to be loaded automatically"

— https://docs.blender.org/api/5.3/bpy.types.Addon.html (the items of `Preferences.addons`), read 2026-10-11.
> `import addon_utils` / `print(addon_utils.paths())` — "You can also find add-on path locations by running this in the Python Console"

— https://docs.blender.org/manual/en/5.3/advanced/scripting/addon_tutorial.html, read 2026-10-11.
The preferences list itself can show either set: "Only show enabled add-ons. Un-check to see all installed add-ons." (https://docs.blender.org/api/5.3/bpy.types.PreferencesView.html, read 2026-10-11).

**For which Blender.**
- Each version keeps its own user folder (`…/5.3/`, question 6). `bpy.utils.resource_path(type, *, major=None, minor=None)` "Return the base path for storing system files", where "major (int | None) – Major version. None (the default) uses bpy.app.version[0]." (https://docs.blender.org/api/5.3/bpy.utils.html, read 2026-10-11).
- What each extension supports is in its manifest: "blender_version_min: Minimum supported Blender version" and "blender_version_max: Blender version that the extension does not support, earlier versions are supported." (https://docs.blender.org/manual/en/5.3/advanced/extensions/getting_started.html, read 2026-10-11).

**The steps (inferred).** A given Blender answers for its own version's folders and repositories. A script in one Blender can work out where another version's folders are by passing that version's numbers to `resource_path`, but no documented command lists another version's add-ons. Which versions an extension supports can be read from its manifest; whether `list` shows this is not stated.

**Not covered.** The format of the `list` output, and whether it marks installed versus available packages. Any command-line listing of older-style add-ons. The rest of `addon_utils`, which has no page in the API documentation (searched by name: only the two manual examples mention it).

**Where I looked.** Beyond the general searches: "list", "repo-list", "sysinfo", "System Info", "Enabled Add-Ons", "addon_utils", "show_addons_enabled_only", "resource_path", "user_resource", "script_paths", "version". Pages read: the extension command-line page, the help-menu page, the tutorial, the API pages for `Addon`, `Addons`, `Preferences`, `PreferencesView`, `bpy.utils`, `bpy.ops.wm`, `bpy.app`.

**Outside the interface.** **Settled by test — observed, on the Creator's Mac, 2026-10-17; see the Observed sections below.** With no window: `extension list` printed each repository with its packages marked `[installed]`, no version and no switched-on state, the same whether switched on or off, and no add-on that ships with Blender; the switched-on collection dropped an add-on once it was switched off; `addon_utils.modules()` listed every add-on found, those that ship with Blender and installed extensions alike; `bpy.ops.wm.sysinfo` wrote its report with no prompt, naming each switched-on add-on with its version and path. Each run answered for the settings folder it was pointed at; another Blender version was not asked in this sitting. *What the sources say:* `list`, `repo-list`, `sysinfo`, `Preferences.addons` and `addon_utils.paths()` are all usable from the command line or a script. The add-ons list in the Preferences window is interface-only.

**Operating system.** **Tested on macOS on Apple silicon only (2026-10-17, below);** Linux and Windows were not tested in this sitting. *What the sources say:* the commands are the same everywhere. The folder paths differ (question 6).

### Observed — what is switched on and what is on the machine, in a new settings folder

**Label: observed.** Entry 1 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new, empty settings folder — see *How the hand tests were run* at the top of this document.

**What is switched on.** `blender --background --python modules.py`, a script printing `len(bpy.context.preferences.addons)` and the sorted `module` of each entry, printed, exit status 0:

```
COUNT 8
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

**What is on the machine at all.** `blender --background --python inventory.py`, a script printing `addon_utils.paths()`, the number of `addon_utils.modules()`, and each module's name with its `bl_info`, printed, exit status 0:

```
PATHS ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core']
COUNT 13
MODULE io_anim_bvh {'name': 'BioVision Motion Capture (BVH) format', 'author': 'Blender Foundation', 'version': (1, 0, 1), 'blender': (2, 81, 6), 'location': 'File > Import-Export', 'description': 'Import-Export BVH from armature objects', 'warning': '', 'doc_url': '{BLENDER_MANUAL_URL}/addons/import_export/anim_bvh.html', 'support': 'OFFICIAL', 'category': 'Import-Export'}
MODULE bl_pkg {'name': 'Blender Extensions', 'author': 'Blender Foundation', 'version': (0, 0, 1), 'blender': (4, 0, 0), 'location': 'Edit -> Preferences -> Extensions', 'description': 'Extension repository support for remote repositories', 'warning': '', 'support': 'OFFICIAL', 'category': 'System'}
MODULE cycles {'name': 'Cycles Render Engine', 'author': '', 'blender': (2, 80, 0), 'description': 'Cycles renderer integration', 'warning': '', 'doc_url': 'https://docs.blender.org/manual/en/latest/render/cycles/', 'tracker_url': '', 'support': 'OFFICIAL', 'category': 'Render'}
MODULE io_scene_fbx {'name': 'FBX format', 'author': 'Blender Foundation', 'version': (5, 15, 0), 'blender': (5, 0, 0), 'location': 'File > Import-Export', 'description': 'FBX IO meshes, UVs, vertex colors, materials, textures, cameras, lamps and actions', 'warning': '', 'doc_url': '{BLENDER_MANUAL_URL}/addons/import_export/scene_fbx.html', 'support': 'OFFICIAL', 'category': 'Import-Export'}
MODULE io_scene_gltf2 {'name': 'glTF 2.0 format', 'author': 'Blender Foundation, Khronos Group', 'version': (5, 3, 40), 'blender': (5, 3, 0), 'location': 'File > Import-Export', 'description': 'Import-Export as glTF 2.0', 'warning': '', 'doc_url': '{BLENDER_MANUAL_URL}/addons/import_export/scene_gltf2.html', 'tracker_url': 'https://github.com/KhronosGroup/glTF-Blender-IO/issues/', 'support': 'OFFICIAL', 'category': 'Import-Export'}
MODULE hydra_storm {'name': 'Hydra Storm render engine', 'author': 'Blender Foundation, AMD', 'version': (1, 0, 0), 'blender': (4, 0, 0), 'description': "USD's high performance rasterizing renderer", 'tracker_url': '', 'doc_url': '', 'community': '', 'downloads': '', 'main_web': '', 'support': 'OFFICIAL', 'category': 'Render'}
MODULE ui_translate {'name': 'Manage UI translations', 'author': 'Blender Foundation', 'version': (2, 1, 0), 'blender': (4, 2, 0), 'location': 'Render properties, I18n Update Translation panel', 'description': "Allows managing UI translations directly from Blender (update main .po files, update scripts' translations, etc.)", 'doc_url': 'https://developer.blender.org/docs/handbook/translating/translator_guide/', 'support': 'OFFICIAL', 'category': 'System'}
MODULE node_wrangler {'name': 'Node Wrangler', 'author': 'Blender Foundation', 'version': (4, 1, 0), 'blender': (5, 0, 0), 'location': 'Node Editor Toolbar or Shift-W', 'description': 'Various tools to enhance and speed up node-based workflow', 'warning': '', 'doc_url': '{BLENDER_MANUAL_URL}/addons/node/node_wrangler.html', 'support': 'OFFICIAL', 'category': 'Node'}
MODULE pose_library {'name': 'Pose Library', 'description': 'Pose Library based on the asset system.', 'author': 'Blender Foundation', 'version': (2, 0), 'blender': (3, 0, 0), 'location': 'Asset Browser -> Animations, and 3D Viewport -> Animation panel', 'doc_url': '{BLENDER_MANUAL_URL}/animation/armatures/posing/editing/pose_library.html', 'support': 'OFFICIAL', 'category': 'Animation'}
MODULE rigify {'name': 'Rigify', 'version': (0, 6, 10), 'author': 'Blender Foundation', 'blender': (4, 0, 0), 'description': 'Automatic rigging from building-block components', 'location': 'Armature properties, Bone properties, View3d tools panel, Armature Add menu', 'doc_url': '{BLENDER_MANUAL_URL}/addons/rigging/rigify/index.html', 'support': 'OFFICIAL', 'category': 'Rigging'}
MODULE io_curve_svg {'name': 'Scalable Vector Graphics (SVG) 1.1 format', 'author': 'Blender Foundation', 'blender': (2, 80, 0), 'location': 'File > Import > SVG as Curves (.svg)', 'description': 'Import SVG as curves', 'warning': '', 'doc_url': '{BLENDER_MANUAL_URL}/addons/import_export/curve_svg.html', 'support': 'OFFICIAL', 'category': 'Import-Export'}
MODULE io_mesh_uv_layout {'name': 'UV Layout', 'author': 'Blender Foundation', 'version': (1, 2, 0), 'blender': (3, 0, 0), 'location': 'UV Editor > UV > Export UV Layout', 'description': 'Export the UV layout as a 2D graphic', 'warning': '', 'doc_url': '{BLENDER_MANUAL_URL}/addons/import_export/mesh_uv_layout.html', 'support': 'OFFICIAL', 'category': 'Import-Export'}
MODULE viewport_vr_preview {'name': 'VR Scene Inspection', 'author': 'Blender Foundation', 'version': (0, 11, 2), 'blender': (3, 2, 0), 'location': '3D View > Sidebar > VR', 'description': 'View the viewport with virtual reality glasses (head-mounted displays)', 'support': 'OFFICIAL', 'doc_url': '{BLENDER_MANUAL_URL}/addons/3d_view/vr_scene_inspection.html', 'category': '3D View'}
```

So the switched-on list holds 8 names and `addon_utils.modules()` 13. Cycles' block carries no `version` key.

**What the package listing prints** with nothing installed — three repository lines, no package lines, and two "No such file" lines — is quoted in full under question 5. No add-on that ships with Blender appears in it.

### Observed — what each list shows once an extension is installed, and once it is switched off

**Label: observed.** Entry 2 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and a new, empty settings folder — see *How the hand tests were run* at the top of this document. The test add-on `smoozy_test` and its install with `-e` are described under question 3.

Once installed and switched on, the switched-on list held it as `bl_ext.user_default.smoozy_test`, `COUNT 9`, and `addon_utils.modules()` listed it under the same name (question 6). It was then switched off and the preferences saved, with `blender --background --python disable_save.py`, which printed, exit status 0:

```
00:00.335  blend            | Writing user preferences: "<work>/res/02/config/userpref.blend" 
SMOOZY TEST ADDON REGISTERED
SMOOZY TEST ADDON UNREGISTERED
DISABLE {'FINISHED'}
Info: Preferences saved
SAVE {'FINISHED'}
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

After that, `blender --command extension list` printed, exit status 0:

```
[Errno 2] No such file or directory: '<work>/res/02/extensions/blender_org'
Repository data: <work>/res/02/extensions/blender_org not found, sync required!
Repository: "extensions.blender.org" (id=blender_org)
Repository: "User Default" (id=user_default)
  smoozy_test [installed]: "Smoozy Test", A tiny test add-on
Repository: "System" (id=system)
```

and the switched-on list, in a separate run, printed `COUNT 8` and the eight names a new folder starts with. So the package line carries the identifier, `[installed]`, the name and the tagline — no version, no switched-on state — and is printed the same whether the extension is switched on or off, while the switched-on list drops it when it is switched off.

**The system-information report, with no window.** Entry 12, in a new settings folder with `smoozy_test` installed with `-e`: `blender --background --python sysinfo.py`, calling `bpy.ops.wm.sysinfo(filepath='<work>/out/12/system-info.txt')`, printed `SYSINFO {'FINISHED'}` between the add-on's own two lines, exit status 0, with no prompt, and the file was written (5 386 bytes). Its add-on section, from its heading to the end of the file:

```
Enabled add-ons:
=========================================

io_anim_bvh (version: (1, 0, 1), path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/io_anim_bvh/__init__.py')
io_curve_svg (version: UNKNOWN, path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/io_curve_svg/__init__.py')
io_mesh_uv_layout (version: (1, 2, 0), path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/io_mesh_uv_layout/__init__.py')
io_scene_fbx (version: (5, 15, 0), path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/io_scene_fbx/__init__.py')
io_scene_gltf2 (version: (5, 3, 40), path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/io_scene_gltf2/__init__.py')
cycles (version: UNKNOWN, path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/cycles/__init__.py')
pose_library (version: (2, 0), path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/pose_library/__init__.py')
bl_pkg (version: (0, 0, 1), path: '/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/bl_pkg/__init__.py')
bl_ext.user_default.smoozy_test (version: (1, 0, 0), path: '<work>/res/12/extensions/user_default/smoozy_test/__init__.py')
```

**The Add-ons list at the window** (entry 17, with Blender's window open; full record under question 13). Unsearched, it showed thirteen rows — the add-ons that ship with Blender other than the BVH, FBX, SVG and UV Layout ones, the four legacy test add-ons and the installed test extension — each with a tick box matching what is switched on, and an `Enabled Only` tick box above them. Opened rows show a `Version` for Node Wrangler (`4.1.0`) and for the extension (`1.0.0`), none for Cycles, and a `File` path for the extension and the legacy add-ons; no repository name.

**What decides the count in the switched-on list** was read at ten points in one settings folder in entry 4, quoted in full under question 3: 8 with nothing installed, 8 with the extension installed under `--no-prefs` and loaded for one run with `--addons`, 9 only once a switch-on had been saved into the folder's preferences — with or without `--addons` — and 8 again from that same folder under `--factory-startup`.

---

## 11. Can an add-on depend on something else being on the computer, or on a computer elsewhere, before it will work, and can what it needs be read anywhere?

**Label:** **documented** that it can, and that some needs can be declared. **Inferred:** other needs cannot be read from anything described.

**What an extension can declare.**
> "wheels: List of relative file-paths Python Wheels."
> "permissions: Add-ons can list which resources they require. The available options are files, network, clipboard, camera, microphone. Each permission should be followed by an explanation (short single-sentence, up to 64 characters, with no end punctuation)."

Both are listed as "Optional values".
— https://docs.blender.org/manual/en/5.3/advanced/extensions/getting_started.html, read 2026-10-11. The same page lists the Blender version range and `platforms` (questions 4 and 14). It can be checked without installing: "blender --command extension validate add-on-package.zip".

**What the rules expect.**
> "For add-ons to be bundled as extensions, they must be self-contained. That means they must come with all its dependencies. In particular 3rd party Python modules."
> "Bundle other add-ons together. This is recommended if an add-on depends on another add-on."
> "If the add-on needs to use internet, it must check for the (read-only) property bpy.app.online_access."
> "Add-ons that follow this setting will only connect to the internet if enabled. However, Blender cannot prevent third-party add-ons from violating this rule."

— https://docs.blender.org/manual/en/5.3/advanced/extensions/addons.html, read 2026-10-11.

**Things outside the add-on.**
> "Python add-ons can be distributed with their own precompiled libraries."

— https://docs.blender.org/manual/en/5.3/troubleshooting/python.html, read 2026-10-11.
> "Blender bundles a complete Python installation and does not interact with the system Python by default. This can be changed with the --python-use-system-env command line argument"

— https://docs.blender.org/manual/en/5.3/advanced/deploying_blender.html, read 2026-10-11.
**Observed:** Cycles' Python files depend on a module that is not among them. `import _cycles` appears 30 times across the files (for example `engine.py`, line 46: `import _cycles`, then line 52: `_cycles.init(path, user_path, bpy.app.background)`). The supplied copy holds no file of that name, no compiled library, and no `blender_manifest.toml`. Its `bl_info` (`__init__.py`, lines 7–16) lists no dependencies. Read 2026-10-16. Where `_cycles` comes from is not shown in the sources.

**The steps (inferred).** The declared fields cover bundled Python modules, the five permission kinds, systems and Blender versions. The field list has nothing for "needs another add-on", "needs a program or library on this computer", or "needs a particular server", and permissions are optional. Older-style add-ons have no manifest at all. So needs beyond the declared fields can only be found by reading the add-on's code or documentation. The Cycles example shows a real add-on with an undeclared need for a module outside its own folder.

**Not covered.** Licence servers or any outside service an add-on contacts, beyond the "network" permission. Whether Blender enforces the permissions in any way — the quote above says it cannot stop add-ons that ignore the online setting.

**Where I looked.** Beyond the general searches: "depend", "dependencies", "self-contained", "wheels", "permissions", "network", "internet", "online_access", "precompiled", ".pyd", ".so", "system Python", "PYTHONPATH", "vendorize", "server". Pages read: the manifest page, the add-ons extension page, the wheels page, the Python troubleshooting page, the deploying page, the `bpy.app` API page, all of Cycles' Python files.

**Outside the interface.** **Settled by test — observed, on the Creator's Mac, 2026-10-17; see the Observed sections below.** With no window: `extension validate` printed one line, `Success parsing TOML in "…"`, and no field of the manifest; the manifest on disk after installing was the one packed; `bpy.app.online_access_override` exists and `bpy.app.online_access_overriden` does not; a package whose only bundled module is named for another kind of computer installed and switched on with nothing said, and its module failed to import, every exit status 0. *What the sources say:* the manifest and `validate` command work the same way outside the interface. Online access can be forced either way from the command line ("--online-mode" / "--offline-mode").

**Operating system.** **Tested on macOS on Apple silicon only (2026-10-17, below);** the Windows library troubleshooting was not tried, this sitting having no Windows machine. *What the sources say:* compiled parts depend on the system (question 4). The precompiled-library and DLL troubleshooting is written for Windows ("Mixed Python Libraries (DLLs)", same troubleshooting page).

### Observed — what `extension validate` prints, and the manifest on disk

**Label: observed.** Entry 2 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window — see *How the hand tests were run* at the top of this document. The test add-on `smoozy_test` is described under question 3.

`blender --command extension validate <work>/src/smoozy_test` and `blender --command extension validate <work>/pkg/smoozy_test-1.0.0.zip` each printed exactly one line, exit status 0:

```
Success parsing TOML in "<work>/src/smoozy_test"
```

```
Success parsing TOML in "<work>/pkg/smoozy_test-1.0.0.zip"
```

No field of the manifest is printed back. After installing, the add-on's `blender_manifest.toml` sat at `<settings folder>/extensions/user_default/smoozy_test/blender_manifest.toml`, and `diff` against the file that was packed printed nothing.

**The online-access properties.** Read in entry 6 (full record under question 5): `hasattr(bpy.app, 'online_access_override')` is `True` and `hasattr(bpy.app, 'online_access_overriden')` is `False`. In a new settings folder `bpy.app.online_access` reads `False`; it reads `True` under `--online-mode` and when the preference `use_online_access` has been saved as on, and `False` under `--offline-mode`; `online_access_override` reads `True` under either switch and `False` with neither.

**Bundled modules, declared and missing.** Entry 9, recorded in full under question 4, with hand-made wheels that hold no compiled code: a package carrying wheels for three kinds of computer had only the one for this Mac unpacked, into `<settings folder>/extensions/.local/lib/python3.13/site-packages/`, not into the add-on's own folder. A package whose only wheel is named for Linux, or only for Windows, installed with `STATUS Installed` and exit status 0 and nothing else, was switched on, unpacked nothing, and its module then failed to import with `ModuleNotFoundError: No module named 'smoozypkg'` — exit status 0.

**Whether a declared permission is shown at the window** (entry 17, with Blender's window open; full record under question 13): the opened row of an installed extension declaring the permissions `network` and `files` shows its description, `Website`, `Type  Extension`, `Maintainer`, `Version` and `File`, and neither permission.

**One add-on's own settings, as an example of what an add-on can need set.** Cycles' settings in this Blender, read with no window in entry 5 (full record under question 3), are `compute_device_type`, `devices`, `peer_memory`, `metalrt`, `use_hiprt`, `use_oneapirt` and `kernel_optimization_level`, reading `METAL`, an empty collection until `refresh_devices()` is called, `False`, `AUTO`, `True`, `True` and `FULL`.

---

## 12. Does installing an add-on disturb a render already running on the same computer?

**The question is ambiguous.** The install could happen inside the same running Blender or in a separate Blender process. "Disturb" could mean stop, crash, or change the output. The sources answer none of these.

**Label:** **not in the sources.** Three documented facts bear on it; what I draw from them is **inferred**.

*That label answers from the sources only. The one-Blender reading has since been tested — **observed** on the Creator's Mac, 2026-10-17, entries 15 and 16 of the hand tests; see the Observed sections at the end of this answer. The reading with a second Blender running at the same time was not tested in this sitting.*

1. A running Blender does not pick up add-on changes on its own.
   > "Refresh Local — Scan extension & legacy add-ons for changes to modules & meta-data (similar to restarting)."

   — https://docs.blender.org/manual/en/5.3/editors/preferences/extensions.html, read 2026-10-11.
   **Inferred:** because a manual refresh exists, a Blender already running does not notice add-ons installed after it started until it refreshes or restarts. The page does not say this outright.
2. Upgrading clears the extension's folder.
   > "The location of the extension it self is not a suitable place to store files because it is cleared each upgrade"

   — https://docs.blender.org/api/5.3/bpy.utils.html (`extension_path_user`), read 2026-10-11.
   **Inferred:** a render that reads files from an add-on's folder while another process upgrades that add-on could find them gone. Not stated.
3. Repositories have a file-system lock: `bpy.ops.extensions.repo_unlock()` "Remove the repository file-system lock" (https://docs.blender.org/api/5.3/bpy.ops.extensions.html, read 2026-10-11). What the lock guards against, and whether it touches rendering, is not stated.

**Not covered.** All of the behaviour asked about.

**Where I looked.** Beyond the general searches: "lock", "concurrent", "simultaneous", "another instance", "multiple instances", "other process", "running", "restart", "refresh", "upgrade", "reload". Pages read: both preference pages, the extension command-line page, the add-ons extension page, the Blender-as-a-module page ("it’s possible to start multiple instances, each with their own bpy and therefore Blender state"), the API pages for the extensions operators, `bpy.utils` and `bpy.app.handlers`. All 5.3 release notes.

**Outside the interface.** **Partly settled by test — observed, on the Creator's Mac, 2026-10-17; see the two Observed sections below.** With no window, an install fired from inside a rendering Blender, after its second frame, returned `{'FINISHED'}`, the render went on to write all eight frames and exited 0, and every frame matched an undisturbed render of the same file pixel for pixel. With the window open, the same install fired from the render's frame handler raised `ValueError: signal only works in main thread of the main interpreter` and installed nothing, while the render went on unchanged; fired instead from Blender's main loop while the render ran, it returned `{'FINISHED'}` and every frame matched, with the interface lock off and on. A render or install from a second Blender running at the same time was not tested in this sitting, which ran one Blender at a time. *What the sources say:* not stated.

**Operating system.** **Tested on macOS on Apple silicon only (2026-10-17, below);** Linux and Windows were not tested in this sitting. *What the sources say:* not stated.

### Observed — what an install writes, an install fired from inside a rendering Blender, and the repository lock

**Label: observed.** Entry 15 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and new, empty settings folders — see *How the hand tests were run* at the top of this document. **One Blender ran at a time**: the reading of this question in which a second Blender installs while a first one renders was not tested in this sitting. The test extension is `smoozy_ok` (question 13).

**What an install changes.** A settings folder with its preferences saved once (`Info: Preferences saved`, `SAVE {'FINISHED'}`) was fingerprinted file by file (`md5` of every file) before and after `blender --command extension install-file -r user_default -e <work>/pkg/smoozy_ok-1.0.0.zip` (`STATUS Installed "smoozy_ok"` and the add-on's two lines, exit status 0). `diff` of the two lists:

```
1c1,5
< af85ddbeba14ea86132697d7aa8bbe6f <work>/res/15a/config/userpref.blend
---
> f74080476e5c0a1aa42d9ef2b9353a06 <work>/res/15a/config/userpref.blend
> 45d4fc4f2bb5ba9ea743878d396020c8 <work>/res/15a/extensions/.cache/compat.dat
> 5295926b925ff95e51ef710836ebba5d <work>/res/15a/extensions/user_default/smoozy_ok/__init__.py
> 0976c234b02c22f51aa48bf4b76f36c8 <work>/res/15a/extensions/user_default/smoozy_ok/__pycache__/__init__.cpython-313.pyc
> 5bde0ac335bdd8e96f58fbab3335d2f7 <work>/res/15a/extensions/user_default/smoozy_ok/blender_manifest.toml
```

and the next run's switched-on list held `bl_ext.user_default.smoozy_ok`, `COUNT 9`. The same in a second folder with `--no-prefs` added:

```
1a2,5
> 61cd0159ffe173549e795b5e55e8957b <work>/res/15b/extensions/.cache/compat.dat
> 5295926b925ff95e51ef710836ebba5d <work>/res/15b/extensions/user_default/smoozy_ok/__init__.py
> f20c0bbe6a515e2778525cfac67b9b69 <work>/res/15b/extensions/user_default/smoozy_ok/__pycache__/__init__.cpython-313.pyc
> 5bde0ac335bdd8e96f58fbab3335d2f7 <work>/res/15b/extensions/user_default/smoozy_ok/blender_manifest.toml
```

— `config/userpref.blend` unchanged — and the next run's switched-on list held only the eight built-in names, `COUNT 8`, although `-e` was given.

**An install fired from inside a rendering Blender.** The scene: Blender's start-up scene with its cube turned between frame 1 and frame 8, rendered with Cycles on the processor, 32 samples, adaptive sampling and denoising off, seed 0, 320 by 240, PNG, frames 1 to 8, saved as `scene.blend` (the saving run printed `SCENE CYCLES 32 320 240 1 8`). In a third new settings folder it was rendered undisturbed — `blender --background scene.blend --render-output <work>/out/15/control/f_ --render-anim` — writing `f_0001.png` to `f_0008.png`, exit status 0. Then the same render was run with a script that adds a `render_post` handler which, after frame 2, calls `bpy.ops.extensions.package_install_files(filepath=<work>/pkg/smoozy_ok-1.0.0.zip, repo='user_default', enable_on_install=True)`, and which prints the scene's `render.use_lock_interface`, the window manager's `is_interface_locked` and `bpy.app.is_job_running('RENDER')` before the render, after each frame and after the render. It printed, exit status 0 (the eight `Saved: …` lines of the render are left out here; all eight frames were written):

```
SMOOZY_MIDRENDER_READBACK before_render use_lock_interface False wm_is_interface_locked False is_job_running_RENDER False
SMOOZY_MIDRENDER_FRAME 1
SMOOZY_MIDRENDER_READBACK in_handler_frame_1 use_lock_interface False wm_is_interface_locked False is_job_running_RENDER False
SMOOZY_MIDRENDER_FRAME 2
SMOOZY_MIDRENDER_READBACK in_handler_frame_2 use_lock_interface False wm_is_interface_locked False is_job_running_RENDER False
STATUS Installed "smoozy_ok"
SMOOZY OK ADDON REGISTERED
SMOOZY_MIDRENDER_INSTALL {'FINISHED'}
SMOOZY_MIDRENDER_ADDONS ['bl_ext.user_default.smoozy_ok']
SMOOZY_MIDRENDER_FRAME 3
SMOOZY_MIDRENDER_READBACK in_handler_frame_3 use_lock_interface False wm_is_interface_locked False is_job_running_RENDER False
…
SMOOZY_MIDRENDER_FRAME 8
SMOOZY_MIDRENDER_READBACK in_handler_frame_8 use_lock_interface False wm_is_interface_locked False is_job_running_RENDER False
SMOOZY_MIDRENDER_READBACK after_render use_lock_interface False wm_is_interface_locked False is_job_running_RENDER False
SMOOZY_MIDRENDER_DONE
SMOOZY OK ADDON UNREGISTERED
```

(frames 4 to 7 printed the same two lines as frame 3, and are left out only for length). Every frame was then compared with the undisturbed one picture against picture — every pixel value loaded in Blender — not file against file:

```
PIXELCMP <work>/out/15/control/f_0001.png <work>/out/15/midrender2/f_0001.png differing_values 0 of 307200 worst 0.0
PIXELCMP <work>/out/15/control/f_0002.png <work>/out/15/midrender2/f_0002.png differing_values 0 of 307200 worst 0.0
PIXELCMP <work>/out/15/control/f_0003.png <work>/out/15/midrender2/f_0003.png differing_values 0 of 307200 worst 0.0
PIXELCMP <work>/out/15/control/f_0004.png <work>/out/15/midrender2/f_0004.png differing_values 0 of 307200 worst 0.0
PIXELCMP <work>/out/15/control/f_0005.png <work>/out/15/midrender2/f_0005.png differing_values 0 of 307200 worst 0.0
PIXELCMP <work>/out/15/control/f_0006.png <work>/out/15/midrender2/f_0006.png differing_values 0 of 307200 worst 0.0
PIXELCMP <work>/out/15/control/f_0007.png <work>/out/15/midrender2/f_0007.png differing_values 0 of 307200 worst 0.0
PIXELCMP <work>/out/15/control/f_0008.png <work>/out/15/midrender2/f_0008.png differing_values 0 of 307200 worst 0.0
```

Afterwards `extension list` showed `smoozy_ok [installed]` under User Default, and the next run's switched-on list held only the eight built-in names. *(The first attempt at this run read the lock setting as `scene.use_lock_interface`, which does not exist — it printed `AttributeError: 'Scene' object has no attribute 'use_lock_interface'` from the handler at every frame, so the install never fired; that run's eight frames also matched the control pixel for pixel. Asked which types carry a property with `lock_interface` in its name, Blender printed `[('RenderSettings', 'use_lock_interface'), ('WindowManager', 'is_interface_locked')]`, and the script was corrected to `scene.render.use_lock_interface`. The run quoted above is the corrected one.)*

**The repository lock, taken twice in one Blender.** In a fourth new settings folder, a script listing every file under the extensions folder, calling `bpy.ops.extensions.repo_lock_all()` twice, then `repo_unlock_all()`, printed, exit status 0:

```
SMOOZY_LOCK_OPS ['repo_lock_all', 'repo_unlock', 'repo_unlock_all']
SMOOZY_BEFORE []
Info: Locked 3 repo(s)
SMOOZY_LOCK1 {'FINISHED'}
SMOOZY_AFTER ['<work>/res/15d/extensions/blender_org', '<work>/res/15d/extensions/blender_org/.blender_ext', '<work>/res/15d/extensions/blender_org/.blender_ext/index.lock', '<work>/res/15d/extensions/user_default', '<work>/res/15d/extensions/user_default/.blender_ext', '<work>/res/15d/extensions/user_default/.blender_ext/index.lock']
Error locking repository "extensions.blender.org": lock exists (lock is already held by this session)
Error locking repository "User Default": lock exists (lock is already held by this session)
Error locking repository "System": lock exists (lock is already held by this session)
Error: Repository "extensions.blender.org": lock exists (lock is already held by this session)
If the lock was held by a Blender instance that exited unexpectedly,
use: "Force Unlock Repository" to clear the lock.
Access from the "Repositories" popover in the extensions preferences.
Error: Repository "User Default": lock exists (lock is already held by this session)
Error: Repository "System": lock exists (lock is already held by this session)
SMOOZY_LOCK2_RAISED RuntimeError Error: Repository "extensions.blender.org": lock exists (lock is already held by this session)
If the lock was held by a Blender instance that exited unexpectedly,
use: "Force Unlock Repository" to clear the lock.
Access from the "Repositories" popover in the extensions preferences.
Error: Repository "User Default": lock exists (lock is already held by this session)
Error: Repository "System": lock exists (lock is already held by this session)
Info: Unlocked 3 repo(s)
SMOOZY_UNLOCK {'FINISHED'}
SMOOZY_END ['<work>/res/15d/extensions/blender_org', '<work>/res/15d/extensions/blender_org/.blender_ext', '<work>/res/15d/extensions/user_default', '<work>/res/15d/extensions/user_default/.blender_ext']
```

So the second call printed both forms, `Error locking repository "…"` and `Error: Repository "…"`, and raised; the lock is an `index.lock` file inside each repository's `.blender_ext` folder, gone after the unlock.

### Observed — an install fired inside a rendering Blender with its window open, the interface lock off and on

**Label: observed.** Entry 16 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, **with Blender's window open on the Mac's screen**, started from a terminal and driven by a script; each run in a new settings folder whose preferences had been saved once with the start-up splash switched off. The render was started the way the window's own Render Animation starts it: `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)`, which returned `{'RUNNING_MODAL'}`. Each run printed `SMOOZY_WINDOW_BACKGROUND False` and `SMOOZY_WINDOW_COUNT 1`, and ended itself after the render; every run exited 0 and wrote eight frames.

**1. The install fired from the render's own frame handler.** The eight-frame scene of entry 15 (32 samples), with a `render_post` handler installing `smoozy_ok` after frame 2, exactly as in the no-window run under entry 15. With the window open, the install **raised** at that point. With the interface lock off, the run printed (each traceback once, every line):

```
SMOOZY_WINDOW_FRAME 2 handler_scene_use_lock_interface False
SMOOZY_LOCK_READBACK in_handler_frame_2 scene_use_lock_interface False wm_is_interface_locked False is_job_running_RENDER True
Traceback (most recent call last):
  File "/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/bl_pkg/bl_extension_ops.py", line 1828, in execute
    result = CommandHandle.op_exec_from_iter(self, context, cmd_batch, is_modal)
  File "/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/bl_pkg/bl_extension_ops.py", line 1621, in op_exec_from_iter
    with CheckSIGINT_Context() as sigint_ctx:
         ~~~~~~~~~~~~~~~~~~~^^
  File "/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core/bl_pkg/bl_extension_ops.py", line 199, in __enter__
    self._old_fn = signal.signal(signal.SIGINT, self._signal_handler_sigint)
                   ~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Applications/Blender 5.3b.app/Contents/Resources/5.3/python/lib/python3.13/signal.py", line 58, in signal
    handler = _signal.signal(_enum_to_int(signalnum), _enum_to_int(handler))
ValueError: signal only works in main thread of the main interpreter
00:03.988  bpy.rna          | ERROR Python script error in EXTENSIONS_OT_package_install_files.execute
SMOOZY_WINDOW_INSTALL_RAISED RuntimeError Error: Python: Traceback (most recent call last):
  …the same traceback again…
ValueError: signal only works in main thread of the main interpreter
Location: <work>/scripts/win16_render.py:21
SMOOZY_LOCK_READBACK just_after_install scene_use_lock_interface False wm_is_interface_locked False is_job_running_RENDER True
SMOOZY_WINDOW_ADDONS []
```

The render went on through frame 8, and every frame matched the no-window control of entry 15 pixel for pixel (`differing_values 0 of 307200 worst 0.0`, eight lines). The same with the interface lock on — `scene.render.use_lock_interface` set to `True`, and read back `True` before the render, at every frame inside the handler and just after the install — raised the same `ValueError`, read `wm_is_interface_locked True` just after it, and again matched the control at every frame. Nothing was installed in either run. In both, `bpy.app.is_job_running('RENDER')` read `True` from inside the handler, and the window manager's `is_interface_locked` read `False` with the lock off.

**2. The install fired from Blender's main loop while the render ran.** Because the frame handler cannot carry the install with a window open, the test was run again with the install made by a one-shot timer five seconds after the render started. The scene was the same eight frames at **512 samples**, so that the render was still going at five seconds, and a new no-window control of it was rendered first (`--render-anim`, eight frames, the last saved at `00:14.654`, exit status 0). With the interface lock off, the window run printed, exit status 0:

```
SMOOZY_WINDOW_BACKGROUND False
SMOOZY_WINDOW_COUNT 1
SMOOZY_LOCK_READBACK before_assignment scene_render_use_lock_interface False wm_is_interface_locked False is_job_running_RENDER False frames_done 0
SMOOZY_LOCK_READBACK after_assignment scene_render_use_lock_interface False wm_is_interface_locked False is_job_running_RENDER False frames_done 0
SMOOZY_WINDOW_RENDER {'RUNNING_MODAL'}
SMOOZY_WINDOW_FRAME 1 handler_use_lock_interface False
SMOOZY_WINDOW_FRAME 2 handler_use_lock_interface False
SMOOZY_LOCK_READBACK timer_before_install scene_render_use_lock_interface False wm_is_interface_locked False is_job_running_RENDER True frames_done 2
SMOOZY OK ADDON REGISTERED
SMOOZY_WINDOW_INSTALL {'FINISHED'}
SMOOZY_LOCK_READBACK timer_after_install scene_render_use_lock_interface False wm_is_interface_locked False is_job_running_RENDER True frames_done 2
SMOOZY_WINDOW_ADDONS ['bl_ext.user_default.smoozy_ok']
SMOOZY_WINDOW_FRAME 3 handler_use_lock_interface False
SMOOZY_WINDOW_FRAME 4 handler_use_lock_interface False
SMOOZY_WINDOW_FRAME 5 handler_use_lock_interface False
SMOOZY_WINDOW_FRAME 6 handler_use_lock_interface False
SMOOZY_WINDOW_FRAME 7 handler_use_lock_interface False
SMOOZY_WINDOW_FRAME 8 handler_use_lock_interface False
SMOOZY_LOCK_READBACK after_render scene_render_use_lock_interface False wm_is_interface_locked False is_job_running_RENDER True frames_done 8
SMOOZY_WINDOW_DONE
SMOOZY_QUIT
SMOOZY OK ADDON UNREGISTERED
```

With the interface lock on, exit status 0:

```
SMOOZY_WINDOW_BACKGROUND False
SMOOZY_WINDOW_COUNT 1
SMOOZY_LOCK_READBACK before_assignment scene_render_use_lock_interface False wm_is_interface_locked False is_job_running_RENDER False frames_done 0
SMOOZY_LOCK_READBACK after_assignment scene_render_use_lock_interface True wm_is_interface_locked False is_job_running_RENDER False frames_done 0
SMOOZY_WINDOW_RENDER {'RUNNING_MODAL'}
SMOOZY_WINDOW_FRAME 1 handler_use_lock_interface True
SMOOZY_WINDOW_FRAME 2 handler_use_lock_interface True
SMOOZY_LOCK_READBACK timer_before_install scene_render_use_lock_interface True wm_is_interface_locked True is_job_running_RENDER True frames_done 2
SMOOZY OK ADDON REGISTERED
SMOOZY_WINDOW_INSTALL {'FINISHED'}
SMOOZY_LOCK_READBACK timer_after_install scene_render_use_lock_interface True wm_is_interface_locked True is_job_running_RENDER True frames_done 2
SMOOZY_WINDOW_ADDONS ['bl_ext.user_default.smoozy_ok']
SMOOZY_WINDOW_FRAME 3 handler_use_lock_interface True
SMOOZY_WINDOW_FRAME 4 handler_use_lock_interface True
SMOOZY_WINDOW_FRAME 5 handler_use_lock_interface True
SMOOZY_WINDOW_FRAME 6 handler_use_lock_interface True
SMOOZY_WINDOW_FRAME 7 handler_use_lock_interface True
SMOOZY_WINDOW_FRAME 8 handler_use_lock_interface True
SMOOZY_LOCK_READBACK after_render scene_render_use_lock_interface True wm_is_interface_locked True is_job_running_RENDER True frames_done 8
SMOOZY_WINDOW_DONE
SMOOZY_QUIT
SMOOZY OK ADDON UNREGISTERED
```

In both runs the install went through after frame 2 with the render running, `extension list` afterwards showed `smoozy_ok [installed]` under User Default, and every frame matched the no-window control pixel for pixel:

```
PIXELCMP <work>/out/16/control16/f_0001.png <work>/out/16/timer_lock_off/f_0001.png differing_values 0 of 307200 worst 0.0
…
PIXELCMP <work>/out/16/control16/f_0008.png <work>/out/16/timer_lock_off/f_0008.png differing_values 0 of 307200 worst 0.0
PIXELCMP <work>/out/16/control16/f_0001.png <work>/out/16/timer_lock_on/f_0001.png differing_values 0 of 307200 worst 0.0
…
PIXELCMP <work>/out/16/control16/f_0008.png <work>/out/16/timer_lock_on/f_0008.png differing_values 0 of 307200 worst 0.0
```

(frames 2 to 7 printed the same, `differing_values 0 of 307200 worst 0.0`, in both runs). With the window open and the lock on, the window manager's `is_interface_locked` read `True` from the first reading taken while the render ran; with the lock off it read `False` throughout. (The no-window run under entry 15 was made with the lock off only.)

---

## 13. Once an add-on is installed and switched on, what more can be learned about whether it will work?

**Label:** **documented** for the checks listed. **Inferred:** there is no general test.

*Those labels answer from the sources only. What Blender reports has since been tested — **observed** on the Creator's Mac, 2026-10-17, entries 12, 13 and 17 of the hand tests; see the Observed sections at the end of this answer.*

What the sources offer:
1. Error messages: "If the Add-on does not activate when enabled, check the Console window for any errors that may have occurred." (https://docs.blender.org/manual/en/5.3/editors/preferences/addons.html, read 2026-10-11).
2. Its own settings: "Sometimes these might even all default to off. So it is important to check if the enabled add-on has any particular preferences." (same page).
3. Its information box: "such as its location, a description and a link to the documentation. Here you can also find a button to report a bug specific of this add-on." (same page).
4. Rescan warnings: "Refresh Local — Scan extension & legacy add-ons for changes to modules & meta-data (similar to restarting). Any issues are reported as warnings." (same page).
5. Its manifest: declared permissions, systems and Blender versions (question 11), and "blender --command extension validate" to "Validate the package meta-data" (https://docs.blender.org/manual/en/5.3/advanced/command_line/extension_arguments.html, read 2026-10-11).
6. The system report's "Enabled Add-Ons — Lists add-ons currently in use along with their versions and paths." (https://docs.blender.org/manual/en/5.3/getting_started/help.html, read 2026-10-11).
7. For an add-on render engine: "Use -E help to list available engines." (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11).
8. For add-on nodes: "is_registered_node_type() True if a registered node type" (https://docs.blender.org/api/5.3/bpy.types.Node.html, read 2026-10-11).
9. For internet use: "bpy.app.online_access Boolean, true when internet access is allowed by Blender & 3rd party scripts (read-only)." and "bpy.app.online_access_override Boolean, true when internet access preference is overridden by the command line (read-only)." (https://docs.blender.org/api/5.3/bpy.app.html, read 2026-10-11).
10. For window-less use: `gpu.init()` "If the initialization fails, a SystemError will be raised." (https://docs.blender.org/api/5.3/gpu.html, read 2026-10-11).
11. Messages and exit codes: "--debug-python Enable debug messages for Python." and "--python-exit-code <code> Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line), zero disables." (arguments page, read 2026-10-11).

**The step (inferred).** None of these is a test that the add-on does its job. The sources describe no self-test, health check, or "will this work" query for add-ons. Points 1, 4 and 11 report failures that show up; points 2, 5 and 9 show conditions that may stop it working; points 7 and 8 confirm that something it registers is present.

**Not covered.** Whether an add-on works without a window, on a GPU, on this system, with this Blender, or with an outside service, short of running it on real work.

**Where I looked.** Beyond the general searches: "error", "warning", "console", "report", "validate", "is_registered", "test", "check", "debug", "exit-code", "log". Pages read: both preference pages, the extension command-line page, the arguments page, the help-menu page, the troubleshooting pages, the API pages for `bpy.app`, `gpu`, `Node`, `bpy.utils` and `bpy.app.timers` (`is_registered` there is about timers only).

**Outside the interface.** **Settled by test, outside the window and at it — observed, on the Creator's Mac, 2026-10-17; see the Observed sections below.** With no window: seventeen checks returned without raising; a broken add-on's switch-on raised from `bpy.ops.preferences.addon_enable`, returned `None` from `addon_utils.enable`, and left it out of the switched-on collection; the exit status stayed 0 unless `--python-exit-code 1` was given and the error left uncaught; the listing printed failed, working and switched-off extensions the same way; `validate` passed an add-on whose code is not Python; damage on disk was reported at start-up, not by the rescan. At the window: a declared warning is shown on the row and inside it; an add-on that failed to switch on looks like one left off. *What the sources say:* points 5, 6 (through `bpy.ops.wm.sysinfo`), 7, 8, 9, 10 and 11 work without a window. Points 1–4 as described are interface steps, although the console messages also appear when Blender is started from a terminal.

**Operating system.** **Tested on macOS on Apple silicon only (2026-10-17, below);** Linux and Windows, and the Windows-only console switch, were not tested in this sitting. *What the sources say:* "-con, --start-console — Start with the console window open (ignored if -b is set), (Windows only)." (arguments page). Otherwise the sources do not say these checks differ by system.

### Observed — what Blender reports about an installed add-on, and about one that fails, with no window

**Label: observed.** Entry 13 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and new, empty settings folders — see *How the hand tests were run* at the top of this document.

**The test add-ons.** Four extensions made for this test, each with an ordinary manifest: `smoozy_ok`, whose `register()` prints `SMOOZY OK ADDON REGISTERED`; `smoozy_badreg`, whose `register()` raises `RuntimeError("SMOOZY_BADREG_DELIBERATE_FAILURE")`; `smoozy_badimport`, which imports a module `smoozy_module_that_does_not_exist`; and `smoozy_badid`, which registers an operator whose `bl_idname` is the number `1`. All four built, exit status 0.

**A. Seventeen checks with one working extension switched on.** With `smoozy_ok` installed with `-e`, `blender --background --python checks13.py` — a script running each check inside its own guard and printing what it returned or what it raised — printed, exit status 0:

```
SMOOZY OK ADDON REGISTERED
SMOOZY13 background -> True
SMOOZY13 addons_keys -> ['bl_ext.user_default.smoozy_ok', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
SMOOZY13 addon_utils_paths -> ['/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/addons_core']
SMOOZY13 online_access -> False
SMOOZY13 has_online_access_override -> True
SMOOZY13 has_online_access_overriden -> False
SMOOZY13 is_job_running_render -> False
SMOOZY13 extension_path_user -> '<work>/res/13/extensions/.user/user_default/smoozy_ok'
SMOOZY13 workspace_filter -> False
Warning: extensions.blender.org: [Errno 2] No such file or directory: '<work>/res/13/extensions/blender_org'
SMOOZY13 repo_refresh_all -> {'FINISHED'}
SMOOZY13 addon_refresh -> {'FINISHED'}
SMOOZY13 addon_expand -> {'FINISHED'}
SMOOZY13 status_clear_errors -> {'FINISHED'}
SMOOZY13 addon_enable_args -> ['module', 'rna_type']
SMOOZY13 addon_prefs -> None
SMOOZY13 prefs_version -> (5, 3, 29)
SMOOZY13 wm_reports -> []
SMOOZY OK ADDON UNREGISTERED
```

The checks, in order: `bpy.app.background`; the keys of `bpy.context.preferences.addons`; `addon_utils.paths()`; `bpy.app.online_access`; whether `bpy.app` has `online_access_override` and `online_access_overriden`; `bpy.app.is_job_running('RENDER')`; `bpy.utils.extension_path_user('bl_ext.user_default.smoozy_ok', path='', create=False)`; the first workspace's `use_filter_by_owner`; `bpy.ops.extensions.repo_refresh_all()`; `bpy.ops.preferences.addon_refresh()`; `bpy.ops.preferences.addon_expand(module='bl_ext.user_default.smoozy_ok')`; `bpy.ops.extensions.status_clear_errors()`; the arguments `addon_enable` takes; that add-on's `preferences`; `bpy.context.preferences.version`; and `bpy.context.window_manager.reports`. None raised.

**B. Switching on each of the four.** In a second new folder, all four were installed without `-e`. `blender --background --python enable13.py` — for each, calling `bpy.ops.preferences.addon_enable(module=…)` inside a guard, then printing whether the module is in `bpy.context.preferences.addons` and what `addon_utils.check(…)` returns — exited 0. Its **ordinary output**, kept apart from its error output:

```
SMOOZY_TRY bl_ext.user_default.smoozy_ok
SMOOZY OK ADDON REGISTERED
SMOOZY_OPS_RESULT {'FINISHED'}
SMOOZY_IN_COLLECTION True
SMOOZY_CHECK (True, True)
SMOOZY_TRY bl_ext.user_default.smoozy_badreg
Exception in module register(): <work>/res/13b/extensions/user_default/smoozy_badreg/__init__.py
Error: SMOOZY_BADREG_DELIBERATE_FAILURE
SMOOZY_OPS_RAISED RuntimeError Error: SMOOZY_BADREG_DELIBERATE_FAILURE
SMOOZY_IN_COLLECTION False
SMOOZY_CHECK (False, False)
SMOOZY_TRY bl_ext.user_default.smoozy_badimport
Error: No module named 'smoozy_module_that_does_not_exist'
SMOOZY_OPS_RAISED RuntimeError Error: No module named 'smoozy_module_that_does_not_exist'
SMOOZY_IN_COLLECTION False
SMOOZY_CHECK (False, False)
SMOOZY_TRY bl_ext.user_default.smoozy_badid
Exception in module register(): <work>/res/13b/extensions/user_default/smoozy_badid/__init__.py
Error: validating class: Operator.bl_idname expected a string type, not int
SMOOZY_OPS_RAISED RuntimeError Error: validating class: Operator.bl_idname expected a string type, not int
SMOOZY_IN_COLLECTION False
SMOOZY_CHECK (False, False)
SMOOZY OK ADDON UNREGISTERED
```

Its **error output** held three full Python tracebacks, one per broken add-on, ending in turn `RuntimeError: SMOOZY_BADREG_DELIBERATE_FAILURE`, `ModuleNotFoundError: No module named 'smoozy_module_that_does_not_exist'` and `TypeError: validating class: Operator.bl_idname expected a string type, not int`; the first begins:

```
Traceback (most recent call last):
  File "/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts/modules/addon_utils.py", line 549, in enable
    mod.register()
    ~~~~~~~~~~~~^^
  File "<work>/res/13b/extensions/user_default/smoozy_badreg/__init__.py", line 2, in register
    raise RuntimeError("SMOOZY_BADREG_DELIBERATE_FAILURE")
RuntimeError: SMOOZY_BADREG_DELIBERATE_FAILURE
```

Then `blender --background --python utils13.py`, calling `addon_utils.enable(…)` for each of the four, exited 0 and printed on its ordinary output:

```
SMOOZY OK ADDON REGISTERED
SMOOZY_UTILS_RESULT smoozy_ok <module 'bl_ext.user_default.smoozy_ok' from '<work>/res/13b/extensions/user_default/smoozy_ok/__init__.py'>
SMOOZY_IN_COLLECTION smoozy_ok False
Exception in module register(): <work>/res/13b/extensions/user_default/smoozy_badreg/__init__.py
SMOOZY_UTILS_RESULT smoozy_badreg None
SMOOZY_IN_COLLECTION smoozy_badreg False
SMOOZY_UTILS_RESULT smoozy_badimport None
SMOOZY_IN_COLLECTION smoozy_badimport False
Exception in module register(): <work>/res/13b/extensions/user_default/smoozy_badid/__init__.py
SMOOZY_UTILS_RESULT smoozy_badid None
SMOOZY_IN_COLLECTION smoozy_badid False
SMOOZY OK ADDON UNREGISTERED
```

with the same three tracebacks on its error output and no `Error:` line on the ordinary one. After both, `blender --command extension list` printed all four the same way, exit status 0:

```
Repository: "User Default" (id=user_default)
  smoozy_badid [installed]: "Smoozy Badid", A tiny add-on made for a test
  smoozy_badimport [installed]: "Smoozy Badimport", A tiny add-on made for a test
  smoozy_badreg [installed]: "Smoozy Badreg", A tiny add-on made for a test
  smoozy_ok [installed]: "Smoozy Ok", A tiny add-on made for a test
```

(the repository lines above and below it as in question 5).

**C. Exit statuses, and `--addons`.** In a third new folder, with `smoozy_ok` and `smoozy_badreg` installed without `-e`. Each output below is cut to the first and last line of each traceback; nothing else is left out.

```
--- blender --background --python-expr "import bpy; bpy.ops.preferences.addon_enable(module='bl_ext.user_default.smoozy_badreg')"
EXIT 0
Traceback (most recent call last):
RuntimeError: SMOOZY_BADREG_DELIBERATE_FAILURE
Traceback (most recent call last):
RuntimeError: Error: SMOOZY_BADREG_DELIBERATE_FAILURE

Exception in module register(): <work>/res/13c/extensions/user_default/smoozy_badreg/__init__.py
Error: SMOOZY_BADREG_DELIBERATE_FAILURE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
--- the same, with --python-exit-code 1 before --python-expr
EXIT 1
Traceback (most recent call last):
RuntimeError: SMOOZY_BADREG_DELIBERATE_FAILURE
Traceback (most recent call last):
RuntimeError: Error: SMOOZY_BADREG_DELIBERATE_FAILURE


Error: script failed, expr: 'import bpy; bpy.ops.preferences.addon_enable(module='bl_ext.user_default.smoozy_badreg')', exiting.
Exception in module register(): <work>/res/13c/extensions/user_default/smoozy_badreg/__init__.py
Error: SMOOZY_BADREG_DELIBERATE_FAILURE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
--- blender --background --python-exit-code 1 --python caught13.py   (the same call inside a guard, printing CAUGHT)
EXIT 0
Traceback (most recent call last):
RuntimeError: SMOOZY_BADREG_DELIBERATE_FAILURE
Exception in module register(): <work>/res/13c/extensions/user_default/smoozy_badreg/__init__.py
Error: SMOOZY_BADREG_DELIBERATE_FAILURE
CAUGHT RuntimeError Error: SMOOZY_BADREG_DELIBERATE_FAILURE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
--- blender --background --python-exit-code 1 --python-expr "import addon_utils; print('SMOOZY_UTILS_RESULT', addon_utils.enable('bl_ext.user_default.smoozy_badreg'))"
EXIT 0
Traceback (most recent call last):
RuntimeError: SMOOZY_BADREG_DELIBERATE_FAILURE
Exception in module register(): <work>/res/13c/extensions/user_default/smoozy_badreg/__init__.py
SMOOZY_UTILS_RESULT None
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

`blender --background --addons bl_ext.user_default.smoozy_badreg --python modules.py` printed the traceback ending `RuntimeError: SMOOZY_BADREG_DELIBERATE_FAILURE`, the `Exception in module register(): …` line, `COUNT 8` and the eight built-in names, exit status 0. `blender --background --addons bl_ext.user_default.smoozy_ok --python modules.py` printed `SMOOZY OK ADDON REGISTERED`, `COUNT 8`, the eight built-in names and `SMOOZY OK ADDON UNREGISTERED`, exit status 0.

**D. `extension validate`** on two installed folders and three made-up ones, each printing one line:

```
--- <work>/res/13/extensions/user_default/smoozy_ok               (installed, works)
Success parsing TOML in "<work>/res/13/extensions/user_default/smoozy_ok"
EXIT 0
--- <work>/res/13b/extensions/user_default/smoozy_badimport       (installed, cannot be imported)
Success parsing TOML in "<work>/res/13b/extensions/user_default/smoozy_badimport"
EXIT 0
--- <work>/out/13/notpython        (smoozy_ok's manifest; __init__.py holding the text "this is not python(")
Success parsing TOML in "<work>/out/13/notpython"
EXIT 0
--- <work>/out/13/nomanifest       (smoozy_ok's __init__.py only)
FATAL_ERROR: Error, file "<work>/out/13/nomanifest/blender_manifest.toml" not found!
EXIT 1
--- <work>/out/13/noinit           (smoozy_ok's manifest only)
Error, file missing from add-on: "__init__.py"
EXIT 1
```

**E. A working, switched-on extension damaged on disk, three ways, one at a time**, each put right before the next, in a fourth new folder. After each damage, `blender --background --python refresh13.py` — printing the switched-on `smoozy` modules, calling `bpy.ops.extensions.repo_refresh_all()` and `bpy.ops.preferences.addon_refresh()`, and printing them again — and then `blender --command extension list`. Every run exited 0.

1. `__init__.py` overwritten with the text `this is not python(`. The refresh run printed a traceback first, ending:

```
  File "<work>/res/13d/extensions/user_default/smoozy_ok/__init__.py", line 1
    this is not python(
                      ^
SyntaxError: '(' was never closed
```

and then:

```
SMOOZY_LIST_BEFORE ['bl_ext.user_default.smoozy_ok']
Warning: extensions.blender.org: [Errno 2] No such file or directory: '<work>/res/13d/extensions/blender_org'
SMOOZY_REFRESH {'FINISHED'}
SMOOZY_ADDON_REFRESH {'FINISHED'}
SMOOZY_LIST_AFTER ['bl_ext.user_default.smoozy_ok']
```

The listing printed the same traceback, then `smoozy_ok [installed]: "Smoozy Ok", A tiny add-on made for a test` under User Default.

2. `__init__.py` put back and `blender_manifest.toml` moved out of the folder. The refresh run printed:

```
SMOOZY OK ADDON REGISTERED
SMOOZY_LIST_BEFORE ['bl_ext.user_default.smoozy_ok']
Warning: add-on missing manifest, this can cause poor performance!: '<work>/res/13d/extensions/user_default/smoozy_ok/blender_manifest.toml'
Warning: extensions.blender.org: [Errno 2] No such file or directory: '<work>/res/13d/extensions/blender_org'
SMOOZY_REFRESH {'FINISHED'}
Warning: add-on missing manifest, this can cause poor performance!: '<work>/res/13d/extensions/user_default/smoozy_ok/blender_manifest.toml'
SMOOZY_ADDON_REFRESH {'FINISHED'}
SMOOZY_LIST_AFTER ['bl_ext.user_default.smoozy_ok']
SMOOZY OK ADDON UNREGISTERED
```

and the listing printed `SMOOZY OK ADDON REGISTERED` and **no package line under User Default**.

3. The manifest put back and `__init__.py` moved out. The refresh run printed:

```
Add-on not loaded: "bl_ext.user_default.smoozy_ok", cause: module loaded with no associated file, __path__=_NamespacePath(['<work>/res/13d/extensions/user_default/smoozy_ok']), aborting!
SMOOZY_LIST_BEFORE ['bl_ext.user_default.smoozy_ok']
Warning: extensions.blender.org: [Errno 2] No such file or directory: '<work>/res/13d/extensions/blender_org'
SMOOZY_REFRESH {'FINISHED'}
SMOOZY_ADDON_REFRESH {'FINISHED'}
SMOOZY_LIST_AFTER ['bl_ext.user_default.smoozy_ok']
```

and the listing printed the same `Add-on not loaded …` line, then `smoozy_ok [installed]: …` under User Default.

In all three, the switched-on collection still held `bl_ext.user_default.smoozy_ok` before and after the rescan.

**F. A legacy add-on's `warning` field.** A one-file legacy add-on `smoozy_legacy_warn.py`, whose `bl_info` is `{"name": "Smoozy Legacy Warn", "blender": (4, 2, 0), "category": "Development", "warning": "SMOOZY WARNING TEXT"}`, was placed in the settings folder's `scripts/addons` and not switched on. A script printing the `bl_info` of each `smoozy` module in `addon_utils.modules()` printed, exit status 0:

```
SMOOZY_BLINFO smoozy_legacy_warn {'name': 'Smoozy Legacy Warn', 'blender': (4, 2, 0), 'category': 'Development', 'warning': 'SMOOZY WARNING TEXT'}
SMOOZY_BLINFO bl_ext.user_default.smoozy_ok {'name': 'Smoozy Ok', 'author': 'Smoozy Test', 'version': (1, 0, 0), 'blender': (4, 2, 0), 'location': '', 'description': 'A tiny add-on made for a test', 'doc_url': '', 'support': 'COMMUNITY', 'category': 'Development', 'warning': '', 'show_expanded': False}
```

**G. Logging arguments.** `blender --background --log-list-categories` was accepted, exit status 0, and printed 154 lines: the version line, then category names beginning `addon`, `anim`, `anim.action`, `anim.data`. `blender --help` carries `--log <match>`, `--log-level <level>`, `--log-show-memory`, `--log-show-source`, `--log-show-backtrace`, `--log-file <filepath>` and `--log-list-categories`.

**The system-information report** (entry 12, under question 10) lists each switched-on add-on with its version and path, an installed extension included, and reads `version: UNKNOWN` for add-ons whose metadata carries none, Cycles among them.

### Observed — the Add-ons list and opened rows at the window

**Label: observed.** Entry 17 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, **with Blender's window open on the Mac's screen**, started from a terminal and driven by a script, and photographed with Blender's own `bpy.ops.screen.screenshot` (3024 by 1724 pixels). The pictures were read by eye; what is written below is what is visible in them.

**Set-up.** A new settings folder holding, installed with `-e`, the test extension `smoozy_rich`, whose manifest declares `website = "https://example.com/smoozy-rich"`, `copyright = ["2026 Smoozy Test"]`, `tags = ["Render"]`, the licence `SPDX:GPL-3.0-or-later`, and two permissions, `network = "Talks to a test server"` and `files = "Reads test files from disk"`; and, in its `scripts/addons`, four one-file legacy add-ons: `smoozy_legacy_ok`, `smoozy_legacy_warn` (declaring `"warning": "SMOOZY WARNING TEXT"`), `smoozy_legacy_badreg` (whose `register()` raises) and `smoozy_legacy_min99` (declaring `"blender": (99, 0, 0)`). The largest editor was turned into the Preferences editor at its Add-ons section.

**How the pictures were taken, and why three runs.** In the first run two of six pictures showed an earlier state of the window than the one the script had just set — one the 3D viewport, one a repeat of the picture before it — so the script was changed to force a window redraw (`bpy.ops.wm.redraw_timer(type='DRAW_WIN_SWAP', …)`) and take each picture twice; in the third run the picture meant to show the opened Cycles row still showed the unsearched list, and a short run of that step alone (searched for "Cycles", the row opened) gave it. Each state below is identified from what the picture itself shows — the search box's text and which rows are open — not from the order it was taken in.

**The whole list, unsearched.** A search box reading `Search Add-ons`, an unticked `Enabled Only` tick box, a tag icon and a drop-down. Then thirteen rows, each with an arrow, a tick box, a name, and an icon at the right:

```
[x] Cycles Render Engine              (Blender icon)
[x] glTF 2.0 format                   (Blender icon)
[ ] Hydra Storm render engine         (Blender icon)
[ ] Manage UI translations            (Blender icon)
[ ] Node Wrangler                     (Blender icon)
[x] Pose Library                      (Blender icon)
[ ] Rigify                            (Blender icon)
[ ] Smoozy Legacy Badreg              (folder icon)
[ ] Smoozy Legacy Min 99              (folder icon)
[ ] Smoozy Legacy Ok                  (folder icon)
[ ] Smoozy Legacy Warn                (warning triangle)
[x] Smoozy Rich                       (an icon of two people)
[ ] VR Scene Inspection               (Blender icon)
```

No row is shown for the BVH, FBX, SVG or UV Layout add-ons, although `io_anim_bvh`, `io_scene_fbx`, `io_curve_svg` and `io_mesh_uv_layout` are among the eight switched on in a new settings folder (question 10).

**The Cycles row, opened** (searched for "Cycles"): ticked; `Cycles renderer integration.` with `Built-in` at the right; `Website  docs.blender.org`; `Type  Core`; then a section headed `Preferences` holding a two-button row `None | Metal` with `Metal` selected, a ticked `Apple M1 Pro (GPU - 14 cores)`, an unticked `Apple M1 Pro`, `Kernel Optimization  Full` and `MetalRT  Auto`. No maintainer, version, licence, repository, tag or permission. The script printed `SMOOZY_EXPAND cycles {'FINISHED'}` and, read through `addon_utils.module_bl_info`, `SMOOZY_SHOW_EXPANDED False`, while the picture shows the row open.

**The Node Wrangler row, opened** (searched for "Node Wrangler"): unticked; `Various tools to enhance and speed up node-based workflow.` with `Built-in`; `Website  docs.blender.org`; `Type  Core`; `Maintainer  Blender Foundation`; `Version  4.1.0`.

**The installed extension's row, opened** (searched for "smoozy"): `Smoozy Rich`, ticked; `A test add-on declaring permissions.`; `Website  example.com`; `Type  Extension`; `Maintainer  Smoozy Test`; `Version  1.0.0`; `File  <work>/res/17s/extensions/user_default/smoozy_rich/__init__.py`. **No licence, no copyright, no tag, no repository name and neither declared permission appears in the row.**

**Switching the four legacy add-ons on.** The script called `bpy.ops.preferences.addon_enable` on each, inside a guard, and printed:

```
SMOOZY LEGACY OK REGISTERED
SMOOZY_ENABLE smoozy_legacy_ok {'FINISHED'}
SMOOZY_CHECK smoozy_legacy_ok (True, True)
SMOOZY LEGACY WARN REGISTERED
SMOOZY_ENABLE smoozy_legacy_warn {'FINISHED'}
SMOOZY_CHECK smoozy_legacy_warn (True, True)
Exception in module register(): <work>/res/17s/scripts/addons/smoozy_legacy_badreg.py
Error: SMOOZY_LEGACY_BADREG_DELIBERATE_FAILURE
SMOOZY_ENABLE_RAISED smoozy_legacy_badreg RuntimeError Error: SMOOZY_LEGACY_BADREG_DELIBERATE_FAILURE
SMOOZY_CHECK smoozy_legacy_badreg (False, False)
SMOOZY LEGACY MIN99 REGISTERED
Warning: This script was written for Blender version 99.0.0 and might not function (correctly), though it is enabled
SMOOZY_ENABLE smoozy_legacy_min99 {'FINISHED'}
SMOOZY_CHECK smoozy_legacy_min99 (True, True)
SMOOZY_ADDONS ['bl_ext.user_default.smoozy_rich', 'smoozy_legacy_min99', 'smoozy_legacy_ok', 'smoozy_legacy_warn']
```

with the `register()` traceback, ending `RuntimeError: SMOOZY_LEGACY_BADREG_DELIBERATE_FAILURE`, on the error output.

**The list searched for "smoozy", after that.** Five rows: `Smoozy Legacy Badreg` with an empty tick box, its name drawn dim, a folder icon; `Smoozy Legacy Min 99`, ticked, folder icon; `Smoozy Legacy Ok`, ticked, folder icon; `Smoozy Legacy Warn`, ticked, warning triangle; `Smoozy Rich`, ticked, the two-people icon. Nothing on the Badreg row marks it as having failed: it looks like the rows of add-ons left switched off in the unsearched list.

**The warning and failing legacy rows, opened.** `Smoozy Legacy Badreg`: an `Uninstall` button, `Type  Legacy (User)`, `File  <work>/res/17s/scripts/addons/smoozy_legacy_badreg.py` — and nothing else. `Smoozy Legacy Warn`: an `Uninstall` button, `Type  Legacy (User)`, `Warning` followed by a warning triangle and `SMOOZY WARNING TEXT`, and `File  <work>/res/17s/scripts/addons/smoozy_legacy_warn.py`.

**The copy-from-previous-version operator with a window open.** At the end of the run, `bpy.ops.preferences.copy_prev.poll()` printed `False`.

---

## 14. Does an add-on set up for one Blender work in another Blender on the same computer, and what decides that?

**The question is ambiguous.** "Another Blender" could be another version, or another copy of the same version. I answer both.

**Label:** **documented** for the deciding factors. **Inferred** for the overall answer.

*Those labels answer from the sources only. Parts have since been tested from Blender 5.3 alone — **observed** on the Creator's Mac, 2026-10-17, entries 9 and 14 of the hand tests; see the Observed sections at the end of this answer. Another Blender version, and a second copy of 5.3, were not tested in this sitting, so nothing here has been seen to cross from one Blender to another.*

**What decides whether the other Blender sees it.**
> "The preferences need to be imported from previous versions because the configuration files of each Blender version are stored in separate folders."
> "Selecting this option will copy preferences from an older version of Blender. Doing so will copy preferences and startup files from the previous version of Blender and load them. This will include installed add-ons and extensions."

— https://docs.blender.org/manual/en/5.3/getting_started/configuration/defaults.html, read 2026-10-11.
> "This can be used to store your own scripts and add-ons independently of the current Blender version."

— https://docs.blender.org/manual/en/5.3/editors/preferences/file_paths.html (Script Directories), read 2026-10-11.
> "Multiple Blender versions can co-exist on the same system"

— https://docs.blender.org/manual/en/5.3/advanced/deploying_blender.html, read 2026-10-11.
Shared folders can be set by environment variables — "BLENDER_USER_EXTENSIONS: Directory for user extensions." and "BLENDER_SYSTEM_EXTENSIONS: Directory for system extensions repository." (arguments page) — or by a portable setup, which keeps everything with one installation (directory layout page).

**What decides whether it then works.**
> "Some previous Blender add-ons and extensions may not be compatible with a new version of Blender, and choosing this option may lead to errors on startup."

— defaults page, as above.
> "blender_version_min: Minimum supported Blender version" / "blender_version_max: Blender version that the extension does not support, earlier versions are supported."

— https://docs.blender.org/manual/en/5.3/advanced/extensions/getting_started.html, read 2026-10-11.
For older-style add-ons:
> "It also specifies the minimum Blender version required to run the script; older versions won’t display the add-on in the list."

— https://docs.blender.org/manual/en/5.3/advanced/scripting/addon_tutorial.html, read 2026-10-11.
> "New Blender versions can sometimes introduce breaking changes in Blender’s Python API or even change how entire features work."

— https://docs.blender.org/manual/en/5.3/advanced/extensions/version_number_guidelines.html, read 2026-10-11.
> "Blender and the bpy module are only compatible with a single Python version."

— https://docs.blender.org/manual/en/5.3/advanced/deploying_blender.html, read 2026-10-11. The wheel examples are tied to one Python version ("--python-version=3.13", files tagged "cp313"; https://docs.blender.org/manual/en/5.3/advanced/extensions/python_wheels.html, read 2026-10-11).

**The steps (inferred).**
- Another version: by default it does not see an add-on installed through 5.3, because each version keeps its own user folder. It sees it if preferences are imported, if both use a shared script folder, System repository or environment-variable location, or if the add-on is installed again.
- Once seen, whether it works depends on: the declared version range (extensions) or minimum version (older-style); whether the Blender Python interface changed in ways the add-on uses; and, for compiled parts, whether the Python version matches.
- Another copy of the same version: per the directory layout, both would use the same per-version user folder (`…/5.3/`) unless one is portable or redirected. So they would share installed extensions and preferences. The sources do not state this case directly.

**Not covered.** What an already-installed extension does when the running Blender is outside its declared range — refused, hidden, warned, or loaded. Whether two different Blender versions can safely share one extension folder at the same time.

**Where I looked.** Beyond the general searches: "version", "previous version", "import preferences", "compatible", "blender_version_min", "blender_version_max", "bl_info", "Python version", "portable", "BLENDER_USER", "Script Directories", "co-exist", "LTS". Pages read: the defaults page, the file-paths preference page, the directory layout page, the deploying page, the manifest page, the version-number guidelines, the tutorial, the wheels page, the blend-file compatibility page (about files, not add-ons), and the API page for `bpy.utils` (`resource_path`).

**Outside the interface.** **Partly settled by test, from Blender 5.3 alone — observed, on the Creator's Mac, 2026-10-17; see the Observed sections below.** With no window: an extension in a folder named by `BLENDER_SYSTEM_EXTENSIONS` was listed under System and switched on as `bl_ext.system.<id>`; a legacy add-on in a folder named by `BLENDER_SYSTEM_SCRIPTS` was found and switched on; an extension in a folder named by `BLENDER_USER_EXTENSIONS` was listed and loadable from a second settings folder, and was switched on at start-up only in the settings folder whose preferences recorded it; an extension declaring a version range 5.3 is outside was refused at install, exit status 0; a legacy add-on declaring Blender 99 was listed and switched on; `bpy.ops.preferences.copy_prev.poll()` returned `False` with the settings folder handed over by `BLENDER_USER_RESOURCES`; a bundled module made for another Python was refused, exit status 1. *What the sources say:* "Import Preferences From Previous Version" is described as a splash-screen choice, so interface-only as described. Environment variables, script folders, System repositories and portable folders work without a window. Whether there is a command-line way to import preferences from another version is not stated.

**Operating system.** **Tested on macOS on Apple silicon only (2026-10-17, below);** Linux and Windows were not tested in this sitting. *What the sources say:* the per-version folder paths differ by system (question 6). The deciding factors are the same on every system, apart from compiled parts (question 4).

### Observed — a bundled module made for another version of Python

**Label: observed.** Entry 9 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window — see *How the hand tests were run* at the top of this document. Each package went into a new, empty settings folder of its very own. The hand-made wheels are described under question 4; these three are all named for macOS on Apple silicon and differ only in the Python they name: `smoozypkg-1.0.0-cp39-cp39-macosx_11_0_arm64.whl`, `…-cp311-cp311-…` and `…-cp313-cp313-…`, in the packages `smoozy_wheel_cp39`, `smoozy_wheel_cp311` and `smoozy_wheel_cp313`.

**Which Python this Blender carries** (entry 14, part A): `blender --background --python pyver.py`, printing `bpy.app.version_string` and `sys.version`, printed, exit status 0:

```
VERSION 5.3.0 Beta
PYTHON 3.13.13 (main, Apr 25 2025, 12:39:20) [Clang 21.0.0 (clang-2100.1.1.101)]
```

`blender --command extension install-file -r user_default -e <work>/pkg/<id>-1.0.0.zip` printed:

```
--- smoozy_wheel_cp39
STATUS Installed "smoozy_wheel_cp39"
Error: Extension bl_ext.user_default.smoozy_wheel_cp39 is incompatible (This Python version (3.13) isn't compatible with (3.9))
EXIT 1
--- smoozy_wheel_cp311
STATUS Installed "smoozy_wheel_cp311"
Error: Extension bl_ext.user_default.smoozy_wheel_cp311 is incompatible (This Python version (3.13) isn't compatible with (3.11))
EXIT 1
--- smoozy_wheel_cp313
STATUS Installed "smoozy_wheel_cp313"
SMOOZY_WHEEL_CP313 REGISTERED
EXIT 0
```

For the 3.9 and 3.11 packages, the add-on's folder — with its unopened wheel — was on disk under `extensions/user_default/`, there was no `.local` folder, the switched-on list held only the eight built-in names, and `import smoozypkg` failed with `ModuleNotFoundError No module named 'smoozypkg'`. `blender --command extension list` in the 3.9 package's folder still printed `smoozy_wheel_cp39 [installed]: "Smoozy Wheel Cp39", A test add-on carrying bundled Python modules` under User Default, exit status 0. For the 3.13 package the module was unpacked into `extensions/.local/lib/python3.13/site-packages/` and imported:

```
IMPORTED cp313 macosx_11_0_arm64 <work>/res/09-smoozy_wheel_cp313/extensions/.local/lib/python3.13/site-packages/smoozypkg/__init__.py
```

### Observed — where Blender 5.3 looks for add-ons, and what a declared version range does, from one Blender

**Label: observed.** Entry 14 of the hand tests, run 2026-10-17 on the Creator's Mac (MacBook Pro, Apple M1 Pro, its own 14-core graphics, 16 GB, macOS 26.6.2) in Blender 5.3.0 Beta, hash `5d8151335b6b`, with no window and new, empty settings folders — see *How the hand tests were run* at the top of this document. **Every part ran in Blender 5.3 alone.** The question's case of a second Blender version was not run in this sitting: the only other Blender on this Mac was kept for side-by-side controls. So nothing here shows an add-on crossing from one version to another.

**A system-extensions folder named by `BLENDER_SYSTEM_EXTENSIONS`.** The source folder of the test extension `smoozy_ok` was copied to `<work>/out/14/sysext/system/smoozy_ok`, and every run below was given `BLENDER_SYSTEM_EXTENSIONS=<work>/out/14/sysext`. `blender --command extension repo-list` showed the System repository at that folder:

```
system:
    name: "System"
    directory: "<work>/out/14/sysext/system"
    source: "SYSTEM"
```

`blender --command extension list` showed `smoozy_ok [installed]: "Smoozy Ok", A tiny add-on made for a test` under `Repository: "System" (id=system)`, and a run calling `bpy.ops.preferences.addon_enable(module='bl_ext.system.smoozy_ok')` printed `SMOOZY OK ADDON REGISTERED`, `ENABLE {'FINISHED'}` and a switched-on list beginning `'bl_ext.system.smoozy_ok'`. All exit status 0.

**A legacy add-on in a scripts folder named by `BLENDER_SYSTEM_SCRIPTS`.** A one-file legacy add-on `smoozy_legacy_ok.py` (`"blender": (4, 2, 0)`) was placed at `<work>/out/14/sysscripts/addons/`. With `BLENDER_SYSTEM_SCRIPTS=<work>/out/14/sysscripts`, a script listing the `smoozy` modules `addon_utils` finds, their declared `blender` value, then calling `addon_utils.enable(…, default_set=True)` on it, printed, exit status 0:

```
SMOOZY_MODULES ['smoozy_legacy_ok']
SMOOZY_DECLARED smoozy_legacy_ok (4, 2, 0)
SMOOZY LEGACY OK REGISTERED
SMOOZY_ENABLE smoozy_legacy_ok <module 'smoozy_legacy_ok' from '<work>/out/14/sysscripts/addons/smoozy_legacy_ok.py'>
SMOOZY_ADDONS ['smoozy_legacy_ok']
```

A separate run with the same variable printed `COUNT 8` and the eight add-ons a new folder switches on, Cycles among them.

**One extensions folder named by `BLENDER_USER_EXTENSIONS`, shared by two settings folders.** Two new settings folders, A and B, both given `BLENDER_USER_EXTENSIONS=<work>/out/14/sharedext2`; the folder check printed each one's user and configuration folders inside its own folder and the extensions folder at `sharedext2`. Installed through A with `install-file -r user_default -e`, the extension printed `STATUS Installed "smoozy_ok"`, exit status 0, and A's switched-on list then held it, `COUNT 9`. Through B:

```
--- blender --command extension list
[Errno 2] No such file or directory: '<work>/out/14/sharedext2/blender_org'
Repository data: <work>/out/14/sharedext2/blender_org not found, sync required!
Repository: "extensions.blender.org" (id=blender_org)
Repository: "User Default" (id=user_default)
  smoozy_ok [installed]: "Smoozy Ok", A tiny add-on made for a test
Repository: "System" (id=system)
--- blender --background --python enable_shared.py   (prints the switched-on smoozy modules, then addon_utils.enable(…, default_set=False))
SMOOZY_ADDONS_AT_START []
SMOOZY OK ADDON REGISTERED
SMOOZY_ENABLE <module 'bl_ext.user_default.smoozy_ok' from '<work>/out/14/sharedext2/user_default/smoozy_ok/__init__.py'>
SMOOZY OK ADDON UNREGISTERED
--- blender --background --python modules.py
COUNT 8
```

each exit status 0. Afterwards A's folder held `config/userpref.blend` and B's folder held nothing. *(The first attempt at this part pointed at two settings folders that had not been created; what happened then is recorded under question 6.)*

**An extension declaring a Blender version range this Blender is outside.** Two test extensions: `smoozy_min99` (`blender_version_min = "99.0.0"`) and `smoozy_max43` (`blender_version_min = "4.2.0"`, `blender_version_max = "4.3.0"`). `extension validate` printed `Success parsing TOML in "…"` for each, and `extension build` built each, all exit status 0. Installing, in a new settings folder:

```
--- install-file -r user_default <work>/pkg/smoozy_min99-1.0.0.zip
ERROR smoozy_min99: This Blender version (5.3.0) doesn't meet the minimum supported version (99.0.0)
EXIT 0
--- the same with -e
ERROR smoozy_min99: This Blender version (5.3.0) doesn't meet the minimum supported version (99.0.0)
Package was expected to be installed but not found: smoozy_min99
EXIT 0
--- install-file -r user_default <work>/pkg/smoozy_max43-1.0.0.zip
ERROR smoozy_max43: This Blender version (5.3.0) must be less than the maximum version (4.3.0)
EXIT 0
--- the same with -e
ERROR smoozy_max43: This Blender version (5.3.0) must be less than the maximum version (4.3.0)
Package was expected to be installed but not found: smoozy_max43
EXIT 0
```

The listing then showed nothing under User Default, the folder held no add-on folder, and a run calling `addon_utils.enable` on each printed, exit status 0:

```
Add-on not loaded: "bl_ext.user_default.smoozy_min99", cause: No module named 'bl_ext.user_default.smoozy_min99'
SMOOZY_ENABLE smoozy_min99 None
Add-on not loaded: "bl_ext.user_default.smoozy_max43", cause: No module named 'bl_ext.user_default.smoozy_max43'
SMOOZY_ENABLE smoozy_max43 None
SMOOZY_ADDONS []
```

**A legacy add-on declaring Blender 99, at the window** (entry 17, full record under question 13): `smoozy_legacy_min99`, declaring `"blender": (99, 0, 0)`, was switched on with `bpy.ops.preferences.addon_enable`, which printed `Warning: This script was written for Blender version 99.0.0 and might not function (correctly), though it is enabled` and returned `{'FINISHED'}`; its row showed a ticked box. And with the window open, `bpy.ops.preferences.copy_prev.poll()` printed `False`.

**A legacy add-on declaring it needs Blender 99.** `smoozy_legacy_min99.py`, with `"blender": (99, 0, 0)`, was placed beside `smoozy_legacy_ok.py` in a new settings folder's `scripts/addons`. The same listing-and-enabling script printed, exit status 0:

```
SMOOZY_MODULES ['smoozy_legacy_min99', 'smoozy_legacy_ok']
SMOOZY_DECLARED smoozy_legacy_min99 (99, 0, 0)
SMOOZY_DECLARED smoozy_legacy_ok (4, 2, 0)
SMOOZY LEGACY MIN99 REGISTERED
SMOOZY_ENABLE smoozy_legacy_min99 <module 'smoozy_legacy_min99' from '<work>/res/14f/scripts/addons/smoozy_legacy_min99.py'>
SMOOZY LEGACY OK REGISTERED
SMOOZY_ENABLE smoozy_legacy_ok <module 'smoozy_legacy_ok' from '<work>/res/14f/scripts/addons/smoozy_legacy_ok.py'>
SMOOZY_ADDONS ['smoozy_legacy_min99', 'smoozy_legacy_ok']
```

**Whether the copy-from-previous-version operator can run, with the settings folder handed over by `BLENDER_USER_RESOURCES`.** In a new settings folder, a script printing the user folder, `bpy.ops.preferences.copy_prev.poll()`, and — because that returned `False` — the result of calling it, printed, exit status 0:

```
SMOOZY_CP_USER <work>/res/14g
SMOOZY_CP_POLL False
SMOOZY_CP_RAISED RuntimeError Operator bpy.ops.preferences.copy_prev.poll() failed, context is incorrect
```

and the settings folder was still empty. It was not tried with nothing overridden: that would have pointed Blender at the Creator's own folders.

**A second copy of this Blender — not settled.** A copy of `/Applications/Blender 5.3b.app` was made in the work folder (`cp -cR`), with a `portable` folder made inside it at `Contents/Resources/portable`. Started with `--background --factory-startup` and asked for its folders, **the copy never started**: after four minutes it was still waiting before any of Blender's own code had run, and it was stopped. Tried again under a 90-second limit, and once more with the `portable` folder moved out and `BLENDER_USER_RESOURCES` set, under a 60-second limit, it was stopped by the limit both times, printing nothing. The copy carried the extended attributes `com.apple.macl`, `com.apple.provenance` and `com.apple.quarantine`, and `codesign --verify --deep` on it printed `a sealed resource is missing or invalid`. Getting past that needs a person to approve the copy, or a change to a macOS security setting, and this sitting may do neither. So whether two copies of 5.3 share a settings folder, and whether a `portable` folder takes over, are not answered for 5.3 by test.

---

## What the hand tests of 2026-10-17 could not reach, and why

*Added by the hand-test step.* Every answer above that is still silent on one of these is silent for the reason given here, not because Blender was asked and said nothing. The full list is `Audits/Blender Research/Add-ons/Hand tests — 5.3.md`, under *Cannot be run*.

- **Linux and Windows** — the sitting had only the Creator's Mac. Every observed result above is a macOS result on Apple silicon. (Questions 2, 3, 4, 5, 8, 11, 13.)
- **A computer with no display server, or no graphics hardware** — the Mac has both. (Question 3.)
- **Graphics of another make, or two cards** — the Mac has Apple's own graphics only. (Questions 3, 4.)
- **Carrying an installed add-on to a computer of another kind** — needs two kinds of computer. (Questions 4, 8.)
- **Two Blenders running at once** — the sitting ran one Blender at a time on a Mac with 16 GB of memory. So a second Blender installing while a first renders, a held repository lock meeting a second Blender, and a running Blender watching another's install were not tested. (Question 12.)
- **A second Blender version** — the only other Blender on the Mac, 5.2.2, was kept for side-by-side controls. So nothing here shows an add-on crossing from one version to another, or `copy_prev()` carrying one across. (Question 14.)
- **Blender run against the Creator's own settings folders** — never done on purpose, so which folder 5.3 uses with nothing overridden was not read. (Questions 6, 14.)
- **The `extensions.blender.org` listing, or anything else from the internet** — the sitting had no go-ahead to download; fetches were run inside a network block and failed at the network. (Questions 5, 8.)
- **How add-ons are sold, and a third-party extensions site** — no test on a computer shows how add-ons are sold, and the sources name no third-party site. (Questions 5, 8.)
- **A second copy of 5.3, with and without a `portable` folder** — a copy made on the Mac never started; macOS held it, and getting past that needs a person's approval or a change to a security setting. (Question 14.)

## Hand tests

Each needs Blender 5.3 driven from a terminal. "Test add-on" means any small add-on or extension `.zip` to hand.

*Pruned by the hand-test step, 2026-10-17: an item is removed from this list once its result is written into the answer it serves, in an **Observed** section. The numbers of the items left are the researcher's own.*

11. **(Q12) Installing while a render runs.** Start a long background render, using any add-on whose files sit in a user repository. While it runs, in a second terminal, install a different extension, then upgrade or remove the one in use. Note whether the render continues, whether its output changes, and whether its log shows errors. Afterwards check whether the second terminal's change is still in the preferences once the render's Blender has exited. *(Still open, 2026-10-17. An install fired from inside the rendering Blender itself has been run, with and without a window — see the Observed sections under question 12. This form, with a second Blender in a second terminal, was not run: that sitting ran one Blender at a time on a Mac with 16 GB of memory.)*

---

2026-10-16 — Claude Opus 5.5 (Claude Code) — Add-ons, Blender 5.3
