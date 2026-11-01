# Writing — Architecture questions (Blender 5.1)

Sources consulted, and the date: Blender 5.1 manual (`Blender Sources/manual_5.1`, including its `_images` folder), Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and the Cycles add-on files (`Blender Sources/cycles_5.1`). All read on 2026-07-30 (the snapshot date given to me).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered here: 13 only.

---

## 13. When Blender is started with a setting given on the command line and with a script to run at the same launch, what does that script see of that setting?

### The reading I took, and the other one

The question is ambiguous in one place, so I say plainly which reading I answered.

I read "a setting given on the command line" as any setting Blender itself accepts as a command-line argument, and — because this sheet is about where renders write — I have used the output-path argument `-o` / `--render-output` as the worked case throughout. I read "a script to run at the same launch" as a script handed in with `-P` / `--python` (or its siblings `--python-text` and `--python-expr`) in the same command.

The other reading would be narrower: that "a setting" means only the render output path, and nothing else. My answer covers that reading too, because `-o` is the case I worked through in detail. A third possible reading — that "setting" means something handed to the script itself after `--`, rather than to Blender — is answered separately in part D below, so that reading is covered as well.

### Short answer

There is no single thing the script sees. It depends on which of three routes the setting travels, and the sources treat the three differently:

- **A. Through Blender's data (the scene).** What the script sees depends on the order of the arguments. That arguments take effect in the order they are given is **documented**. That a `-P` script obeys that same rule, and which property `-o` writes into, are both **inferred**.
- **B. Through a dedicated flag on `bpy.app`.** A short list of command-line settings are exposed to scripts this way, and that much is **documented**. That the list is closed is **not in the sources** as a statement; it is what a sweep of that page's attribute names found.
- **C. Not at all.** One command-line setting is handed to a call whose inside is not in these files, and none of the lines around it stores the setting anywhere a script could read. That the lines read that way is **observed** in the Cycles files. That a script therefore cannot learn the setting is **inferred**.
- **D. As raw text, through `sys.argv`.** The script can read the words of the command line itself. Access to whatever follows `--` is documented; that `sys.argv` also carries Blender's own arguments is inferred from the Cycles code and an API example.

Each is set out below with its own label.

---

### A. Through Blender's data — order decides everything

**LABEL: documented** (for the ordering rule itself)

The manual states the rule outright, in two places.

Exact words:

> "Arguments are executed in the order they are given. eg:"

and the three worked examples that follow it:

> "blender --background test.blend --render-frame 1 --render-output "/tmp"
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

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html (section "Argument Order"). Date read: 2026-07-30.

The same rule is repeated in the rendering workflow page:

> "Arguments are executed in the order they are given!"

and

> "The following command will not work, since the output and extension are set after Blender is told to render:
>
> blender -b file.blend -a -x 1 -o //render
>
> The following command will behave as expected:
>
> blender -b file.blend -x 1 -o //render -a
>
> Always position -f or -a as the last arguments."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html. Date read: 2026-07-30.

`-P` / `--python` is one of the arguments in the same list on the same page:

> "-P, --python <filepath>
>
> Run the given Python script file."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html (section "Python Options"). Date read: 2026-07-30.

**LABEL: inferred** (for what this means for the script)

The step I took: the ordering rule is stated for arguments in general, and `-P` is an argument in that same list. So a `-P` script runs at its own position in the line, just as `--render-frame` renders at its position. Therefore:

- `blender -b file.blend -o /out/ -P script.py` — the output path is set before the script runs, so the script sees it set.
- `blender -b file.blend -P script.py -o /out/` — the script runs first, so it sees whatever the blend-file carried, not `/out/`.
- `blender -b -o /out/ file.blend -P script.py` — the second documented example says loading the blend-file "overwrites the render output that was set", so the script sees the blend-file's own path, not `/out/`.

The reader has to supply the step "a `-P` script is an argument and so obeys the argument-order rule". The manual never spells that out with a script in the example — every one of its ordering examples uses `--render-frame` or `-a`. That is why this is inferred and not documented.

There is one nearby documented statement that supports the same picture — that a `--python` script can run *before* the blend-file is loaded:

> "It is possible to run a script via the --python command line argument, before the blend file.
> This can register a load-post handler (bpy.app.handlers.load_post) that initializes the name-space.
> While this works for background tasks it has the downside that opening the file from the file selector
> won't setup the name-space."

Source: the Python API documentation, https://docs.blender.org/api/5.1/bpy.app.html (section "File Loading & Order of Initialization"). Date read: 2026-07-30.

That page also documents the related trap in the other direction:

> "Populating the driver name-space before the blend-file loads also doesn't work
> since opening a file clears the name-space."

Same page, same date.

**Where the script would look — and this part is weaker**

**LABEL: inferred**

The sources never say which Python-readable property `-o` writes into. The manual says only:

> "-o, --render-output <path>
>
> Set the render path and file name.
> Use // at the start of the path to render relative to the blend-file."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html. Date read: 2026-07-30.

The API documentation separately describes a scene property:

> "filepath
>
> Directory/name to save animations, # characters define the position and padding of frame numbers (default "", never None, blend relative // prefix supported, Supports template expressions)"

Source: the Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html. Date read: 2026-07-30.

The step I took: the two descriptions match on every distinguishing feature — the `//` blend-relative prefix, the `#` characters for frame-number padding, and path templating (the `-o` entry says "You can use path templating features such as {blend_name} in the path", and the property says "Supports template expressions"). And the manual says loading a blend-file "overwrites the render output that was set", which means the value lives somewhere a blend-file load replaces — that is, in scene data rather than in a separate command-line store. From that I infer that a script started after `-o` would read the given path back from the scene's render settings (`bpy.context.scene.render.filepath` or the equivalent through `bpy.data`).

I want to be blunt about this one: nothing in the sources states that link. A reader has to supply it. If the exact property name matters downstream, it should be checked by hand — see the list at the end.

---

### B. Through a dedicated flag on `bpy.app`

**LABEL: documented**

For a small set of command-line settings, the API documentation says outright that a script can read the setting back from `bpy.app`. The ones whose quoted text names the command line:

> "bpy.app.background
>
> Boolean, True when blender is running without a user interface (started with -b)"

> "bpy.app.factory_startup
>
> Boolean, True when blender is running with –factory-startup"

> "bpy.app.debug
>
> Boolean, for debug info (started with --debug / --debug-* matching this attribute name)."

(the same sentence is repeated for `debug_depsgraph`, `debug_depsgraph_build`, `debug_depsgraph_eval`, `debug_depsgraph_pretty`, `debug_depsgraph_tag`, `debug_depsgraph_time`, `debug_events`, `debug_freestyle`, `debug_handlers`, `debug_io`, `debug_python`, `debug_simdata`, `debug_wm`)

> "bpy.app.use_event_simulate
>
> Boolean, for application behavior (started with --enable-* matching this attribute name)"

> "bpy.app.use_userpref_skip_save_on_exit
>
> Boolean, for application behavior (started with --enable-* matching this attribute name)"

> "bpy.app.online_access_override
>
> Boolean, true when internet access preference is overridden by the command line (read-only)."

Source for all of the above: the Python API documentation, https://docs.blender.org/api/5.1/bpy.app.html. Date read: 2026-07-30.

**Two limits on the list above, both from the quoted words themselves.**

Only one of these entries says in so many words that it cannot be written to — `online_access_override`, which ends "(read-only)". The others quoted above say what the value means and what starts it, and say nothing either way about writing. So this route is a way to *read* a setting back; whether a script can also change any of these is **not in the sources**.

One further attribute on the same page was noted while sweeping it, and it does **not** belong on the list above, because its quoted text names no command-line argument at all:

> "bpy.app.debug_value
>
> Integer value which can be set to non-zero values for testing purposes."

Same page, same date. It is recorded here only so that a later reader does not think the sweep missed it.

The `online_access_override` entry is worth pausing on, because it shows the shape of this route. The preference itself is a separate property:

> "use_online_access
>
> Allow Blender to access the internet. Add-ons that follow this setting will only connect to the internet if enabled. However, Blender cannot prevent third-party add-ons from violating this rule. (default False)"

Source: the Python API documentation, https://docs.blender.org/api/5.1/bpy.types.PreferencesSystem.html. Date read: 2026-07-30.

So the quoted words give two separate things for that setting: a preference holding whether internet access is allowed (`use_online_access`, on `bpy.types.PreferencesSystem`), and a flag on `bpy.app` (`online_access_override`) saying the command line overrode the preference.

**LABEL: inferred** for the step that joins them. The two entries sit on different pages and neither names the other. The step is reading "internet access preference" in the flag's wording as the preference quoted just above. No page I read states the link, and no page I read names any attribute on `bpy.app` holding the value itself.

**Not on this list — LABEL: not in the sources**

`-o` / `--render-output` has no `bpy.app` flag. Nor does `-F`, `-x`, `-E`, `-s`, `-e`, `-t`, or `-S`.

Where I looked: I took the complete list of `bpy.app` attributes off that one page — every top-level attribute name on it — and read the names. None of them names a render output path or an image format. That is a sweep of one page's own list of names, which is the strongest thing available for a question of the shape "is there an attribute for this", but it is still a silence rather than a statement: no page says the list is closed, and an attribute documented somewhere other than that page would not have been caught by it.

There is also a documented case of a command-line switch overriding a preference where the sources do **not** say what a script sees:

> "Enable with -y or --enable-autoexec
>
> Disable with -Y or --disable-autoexec"

and

> "These command-line arguments can be used to start a regular Blender instance and
> will still override the Preferences."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html (section "Command Line"). Date read: 2026-07-30.

The corresponding preference exists in the API:

> "use_scripts_auto_execute
>
> Allow any .blend file to run scripts automatically (unsafe with blend files from an untrusted source) (default False)"

Source: the Python API documentation, https://docs.blender.org/api/5.1/bpy.types.PreferencesFilePaths.html. Date read: 2026-07-30.

But no source I found says whether `-y` / `-Y` writes into that property or overrides it somewhere else. For that particular setting, what the script sees is **not in the sources**.

Where I looked, and what I did not look for: the security page was read in full, and `use_scripts_auto_execute` was traced across the API documentation. I did not run a sweep for the words the switch itself is spelled with — `autoexec`, `enable-autoexec`, `auto-run`, `auto run` — so this silence rests on two pages rather than on a search of the sources for the thing's own name. Treat it as the weakest *not in the sources* in this document.

---

### C. A setting that no line I can read stores anywhere a script could find it

**LABEL: observed** (from the Cycles files supplied in this folder)

Cycles' own Python file shows a command-line setting being handed onwards without any line around it touching a property a script could read.

Exact lines, from `Blender Sources/cycles_5.1/engine.py`, lines 24 to 41 (the whole of `_parse_command_line`):

```
def _parse_command_line():
    import sys

    argv = sys.argv
    if "--" not in argv:
        return

    parser = _configure_argument_parser()
    args, _ = parser.parse_known_args(argv[argv.index("--") + 1:])

    if args.cycles_print_stats:
        import _cycles
        _cycles.enable_print_stats()

    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```

And the place it is called from, same file, lines 44 to 53:

> **Citation check, 2026-08-26 (Claude Opus 5).** Cited as lines 43 to 53; `def init():` is line 44 and line 43 is blank. Corrected. The quoted words are exact.

```
def init():
    import bpy
    import _cycles
    import os.path

    path = os.path.dirname(__file__)
    user_path = os.path.dirname(os.path.abspath(bpy.utils.user_resource('CONFIG', path='')))

    _cycles.init(path, user_path, bpy.app.background)
    _parse_command_line()
```

Source: `Blender Sources/cycles_5.1/engine.py`, lines 24–53. Date read: 2026-07-30.

The manual documents the same argument from the outside:

> "--cycles-device <device>
>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL."

and

> "Cycles add-on options must be specified following a double dash."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html (section "Cycles Render Options"). Date read: 2026-07-30.

What is observed here: the value goes to `_cycles.set_device_override(...)`, a call on an imported name, `_cycles`. Nothing in these lines writes the device into any scene or preference property. What `_cycles` is, and what that call does with the value, is not in these files — the lines quoted import the name and call it, and that is all they show.

**LABEL: inferred**, from the above: a script run at the same launch cannot learn the chosen device by reading Blender's settings, because no line here stores it in them. The step: I am reading "no line here writes a property" as "no property to read". Something behind `_cycles`, or a part of Cycles outside these Python files, could mirror it, and I cannot see inside either.

Two more things these lines show. **Both are reading, and the conclusion drawn from each is a step past the reading, so each is split into its two halves.**

- **Observed:** the function reads `sys.argv`, and returns without doing anything when the list holds no `"--"` (`if "--" not in argv: return`). **Inferred:** a Cycles argument written without the `--` separator is therefore not picked up here. The step: I am reading an early return as nothing being done with the argument. Whether anything *else* in Blender picks it up is outside these lines, and I did not find a source either way.
- **Observed:** the call to `_parse_command_line()` is the last line of a function named `init()`, and `_parse_command_line` is called from nowhere else in these files. **Inferred:** the arguments are therefore parsed at whatever moment `init()` is called, rather than at the argument's own position in the command line. The step is twofold, and both halves are mine: nothing in these files calls `init()`, so nothing here shows when it runs, and reading the name `init` as "when the add-on initializes" is my reading of a name. What follows from that — whether a `-P` script runs before or after this point — is not settled by anything I read, and is listed under NOT COVERED.

---

### D. As raw text, through `sys.argv`

**LABEL: documented** (for what follows `--`)

> "--
>
> End option processing, following arguments passed unchanged. Access via Python's sys.argv."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html. Date read: 2026-07-30.

So anything a person puts after `--` is handed on unchanged, as text, and is reachable from `sys.argv`.

**LABEL: inferred** for one step often taken from that sentence, written out so it is not taken silently: that this route does not depend on where the script sits in the line — the words being there to read whether the script ran before or after them. The quoted entry says the arguments are passed unchanged and reachable from `sys.argv`; it says nothing about when they arrive there relative to a script running.

**LABEL: inferred** (for Blender's own arguments appearing in `sys.argv` too)

The Cycles code quoted above searches `sys.argv` for the string `"--"` and then slices everything after it (`argv[argv.index("--") + 1:]`). If `sys.argv` held only the post-`--` portion, there would be no `"--"` in it to find, and the code would always return early and do nothing. And the API documentation's own example uses the first element as the program name:

> "prog=os.path.basename(sys.argv[0]) + " --command keyconfig_export","

Source: the Python API documentation, https://docs.blender.org/api/5.1/bpy.utils.html (section "Using Python Argument Parsing"). Date read: 2026-07-30.

The step I took: from those two together, `sys.argv` carries the whole command line — the executable at position zero, then Blender's own arguments, then `--`, then the rest. Therefore a script can find the literal text of an `-o` argument by looking through `sys.argv` for it, even when the script ran before that argument took effect.

I am labelling this inferred rather than documented because no page states what `sys.argv` contains; the `--` entry only promises access to what follows the separator.

**A related documented route**, for completeness of this part only: a script can register a command that consumes the rest of the line.

> "bpy.utils.register_cli_command(id, execute)
>
> Register a command, accessible via the (-c / --command) command-line argument."

and

> "execute (Callable[[list[str]], int]) – Callback, taking a single list of strings and returns an int.
> The arguments are built from all command-line arguments following the command id."

Source: the Python API documentation, https://docs.blender.org/api/5.1/bpy.utils.html. Date read: 2026-07-30.

The matching manual entry:

> "-c, --command <command>
>
> Run a command which consumes all remaining arguments.
> Use -c help to list all other commands.
> Pass --help after the command to see its help text.
>
> This implies --background mode."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html. Date read: 2026-07-30.

---

### One further documented point about scripts and file loading

If a script is run before the blend-file is loaded and it leaves something behind for later (a handler), that thing is thrown away when the file loads unless it is marked:

> "By default handlers are freed when loading new files, in some cases you may
> want the handler stay running across multiple files (when the handler is
> part of an add-on for example).
>
> For this the bpy.app.handlers.persistent decorator needs to be used."

and

> "bpy.app.handlers.persistent
>
> Function decorator for callback functions not to be removed when loading new files"

Source: the Python API documentation, https://docs.blender.org/api/5.1/bpy.app.handlers.html. Date read: 2026-07-30.

This is relevant because it is the documented way for a script that ran too early to still act on the state that exists after the file has loaded.

---

### NOT COVERED

What this answer does not settle:

- **The exact property name for `-o`.** I inferred it is the scene's render output path property, and I gave my reasoning, but no source states it. A script reading it back is unverified.
- **`-y` / `-Y`.** The manual says these override the preference; nothing says whether the preference property a script reads changes to match.
- **Whether the script literally runs at its own position, or is merely queued.** The manual says arguments are "executed in the order they are given" and its examples all use rendering arguments. I have not seen a source that demonstrates the ordering with a `-P` script and a setting.
- **Whether a script can *change* a command-line setting and have the change stick.** Of the `bpy.app` entries in part B, only `online_access_override` is quoted as read-only; the rest say nothing either way about being written to. For the scene properties, the sources say nothing about a later command-line argument overwriting a script's change — except the one documented case that a blend-file load overwrites a previously set render output.
- **What is inside `_cycles`.** Part C rests on Python files only. The compiled part is not in this folder and I did not look for it.
- **`--python-text` and `--python-expr`.** I have assumed they sit in the ordering the same way `-P` does, because they appear in the same list, but the sources give no examples for them and I did no separate work on them.
- **Timing against add-on registration.** Cycles parses its arguments in `init()`. Whether a `-P` script runs before or after add-ons initialize is not stated anywhere I found.
- **Whether the `bpy.app` list in part B is the whole of it.** That list comes from sweeping the attribute names on one page. No source says the list is closed, and an attribute documented elsewhere would not have been caught.
- **A Cycles argument written without `--`.** The Cycles function quoted stops without doing anything when there is no `--`. Whether anything else in Blender picks such an argument up is outside those lines, and I found no source either way.
- **Nothing was run.** Everything here is from reading. No Blender was started.

---

---

## Observed — the hand tests of 2026-08-26

Everything in this section was run on **2026-08-26** by Claude Opus 5 (Claude Code). It records what was done and what came back, and on which machine. Nothing here is a reading of the sources; the sections above are.

**The two machines. Their results are never merged.**

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, macOS 26.5.2, Apple's own graphics and no second card. Blender 4.5.12 LTS (hash `84afd5f785f7`, built 2026-07-21), 5.0.1 (`a3db93c5b259`, 2025-12-16), 5.1.2 (`ec6e62d40fa9`, 2026-05-19) and 5.2.1 LTS (`9e2066aef7ef`, 2026-08-25), under `/Applications`.
- **A rented Linux machine, in Norway** — a Docker container holding half its host: Ubuntu 22.04.5 LTS on kernel 6.8.0-136, one NVIDIA GeForce RTX 3060 of 12 GB on driver 595.84, a quota of about fifteen cores of an Intel Xeon E5-2697A v4, 53 GB of free disk, reached over SSH. The same four builds, matching to the same commit hashes, at `/opt/blender/<version>/blender`.

A Mac result settles macOS and leaves Linux open; a Linux result settles Linux and leaves macOS open. **Windows is settled by neither: no Windows machine has run it.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**The set-up common to every run.** A blend-file built by each version itself, holding **two** scenes — `Scene`, whose stored output path is `/tmp/smoozy_q13/from_file_scene_`, and `Second`, whose stored path is `/tmp/smoozy_q13/from_file_second_` — and a text block named `probe`. Two scenes with recognisably different stored paths are what let these runs come out the other way: with a file whose stored path matched the command line, most of them could not have failed. Every run was `-b --factory-startup` with no window unless it says otherwise, pointed at a settings folder of its own made before Blender was started. The path handed in on the command line was always `/tmp/smoozy_q13_sentinel_out_`.

> **Citation check, 2026-08-26 (Claude Opus 5).** All sixteen tests below were built again from these descriptions and re-run on the Creator's Mac, in 4.5.12, 5.0.1, 5.1.2 and 5.2.1, whose build hashes were checked against the four given above and match. Tests 1, 2, 3, 8 and 12 were also re-run on the rented Linux machine, whose four build hashes likewise match. **Every line reproduced, on both machines, except the four sweep-size counts in test 1** — see the note there. The two machines are kept apart here as they are above: the Mac runs are a second Mac result, the Linux runs a second Linux result, and neither is merged with the other. Four places where a repeater has to supply something the description leaves out are noted at tests 1, 4, 9 and 13. Nothing below has been altered; these are notes beside it.

### 1. Which value a script reads a command-line output path from — found by searching, not by guessing a name

A script was run that assumes no property name. It walks every data-block in `bpy.data`, and everything reachable from them through pointer and collection properties to a depth of five, reading only text and choice properties; then the preferences the same way; then every plain attribute of `bpy.app` by name; then `sys.argv`. It prints every place holding the string handed in, and prints how much it looked at. Run as:

```
blender -b --factory-startup q13.blend -o /tmp/smoozy_q13_sentinel_out_ -P find_value.py -- /tmp/smoozy_q13_sentinel_out_
```

**The first attempt brought Blender down, and that is part of the record.** The script as first written read every property, numbers included; on the Creator's Mac in 5.2 it crashed, and the crash file named `node_link_is_hidden` reached through `RNA_property_boolean_get` from the script's own line. The script was then limited to text, choice, pointer and collection properties and re-run; nothing crashed after that, in any version on either machine.

**On the Creator's Mac**, 5.2 printed:

```
SEARCH scenes in file = ['Scene', 'Second']
SEARCH active scene = 'Scene'
SEARCH sentinel = '/tmp/smoozy_q13_sentinel_out_'
SEARCH structs visited = 2452 ; properties examined = 21342 ; text/enum values examined = 4207 ; bpy.app attributes examined = 71
HIT bpy.data.scenes['Scene'].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
HIT sys.argv[5] = '/tmp/smoozy_q13_sentinel_out_'
HIT sys.argv[9] = '/tmp/smoozy_q13_sentinel_out_'
SEARCH hits: data+preferences = 1 ; bpy.app = 0 ; sys.argv = 2
```

4.5, 5.0 and 5.1 printed the identical `HIT` block and the identical closing count, with these sweep sizes:

```
4.5  SEARCH structs visited = 2546 ; properties examined = 20993 ; text/enum values examined = 4006 ; bpy.app attributes examined = 70
5.0  SEARCH structs visited = 2413 ; properties examined = 20544 ; text/enum values examined = 4024 ; bpy.app attributes examined = 70
5.1  SEARCH structs visited = 2456 ; properties examined = 21262 ; text/enum values examined = 4206 ; bpy.app attributes examined = 71
```

> **Citation check, 2026-08-26 (Claude Opus 5) — the sweep sizes are not repeatable from this description.** The three `HIT` lines, the closing count `data+preferences = 1 ; bpy.app = 0 ; sys.argv = 2`, and the `bpy.app` name lists below (including 4.5 carrying `debug_ffmpeg` and lacking `cachedir` and `memory_usage_undo`, and 5.0 lacking `debug_ffmpeg` and `cachedir` and carrying `memory_usage_undo`) all came back exactly, on both machines, in all four versions. The four counts on this line did not: a script written from this description gave 2743 / 24169 / 5128 / 70 for 4.5 on the Mac against the 2546 / 20993 / 4006 / 70 recorded. Only the last of the four — the `bpy.app` attribute count, 70 / 70 / 71 / 71 — matched. The description does not pin down how the walk avoids visiting the same thing twice, or which of `bpy.data`'s collections it starts from, and those choices move the first three numbers a long way. **Read the counts as a note of how much this particular script looked at, not as a result anyone else can reproduce.** One thing that did reproduce: with the type check placed after the property read rather than before it, Blender comes down with the same signature recorded above — `node_link_is_hidden` reached through `RNA_property_boolean_get` — and it does so in 5.1 as well as 5.2.

**On the rented Linux machine** the same three `HIT` lines and the same closing count came back in all four versions, with sweep sizes of 2491/21070/4076/70, 2459/20709/4107/70, 2500/21422/4287/71 and 2497/21500/4287/71 for 4.5, 5.0, 5.1 and 5.2.

**The two machines agreed, line for line, in all four versions.**

Two further things the same run shows. **`-o` reached the active scene only** — `Second` kept `/tmp/smoozy_q13/from_file_second_` in every version on both machines. And **nothing under `bpy.app` held the value**: the search read every name on `bpy.app` and printed the list, which in 5.1 and 5.2 is

```
['alembic', 'autoexec_fail', 'autoexec_fail_message', 'autoexec_fail_quiet', 'background', 'binary_path', 'build_branch', 'build_cflags', 'build_commit_date', 'build_commit_time', 'build_commit_timestamp', 'build_cxxflags', 'build_date', 'build_hash', 'build_linkflags', 'build_options', 'build_platform', 'build_system', 'build_time', 'build_type', 'cachedir', 'count', 'debug', 'debug_depsgraph', 'debug_depsgraph_build', 'debug_depsgraph_eval', 'debug_depsgraph_pretty', 'debug_depsgraph_tag', 'debug_depsgraph_time', 'debug_events', 'debug_freestyle', 'debug_handlers', 'debug_io', 'debug_python', 'debug_simdata', 'debug_value', 'debug_wm', 'driver_namespace', 'factory_startup', 'ffmpeg', 'handlers', 'help_text', 'icons', 'index', 'is_job_running', 'memory_usage_undo', 'module', 'n_fields', 'n_sequence_fields', 'n_unnamed_fields', 'ocio', 'oiio', 'online_access', 'online_access_override', 'opensubdiv', 'openvdb', 'portable', 'python_args', 'render_icon_size', 'render_preview_size', 'sdl', 'tempdir', 'timers', 'translations', 'usd', 'use_event_simulate', 'use_userpref_skip_save_on_exit', 'version', 'version_cycle', 'version_file', 'version_string']
```

**4.5's list differs in three names**: it carries `debug_ffmpeg`, which the other three do not, and it lacks `cachedir` and `memory_usage_undo`. **5.0's** lacks `debug_ffmpeg` and `cachedir` and carries `memory_usage_undo`. The lists were identical on both machines.

### 2. Where the script sits on the line, in five arrangements

The same blend-file, and a script printing every scene's render output path. Five launches per version on each machine:

```
A   blender -b --factory-startup q13.blend -o /tmp/smoozy_q13_sentinel_out_ -P probe.py
B   blender -b --factory-startup q13.blend -P probe.py -o /tmp/smoozy_q13_sentinel_out_
C   blender -b --factory-startup -o /tmp/smoozy_q13_sentinel_out_ q13.blend -P probe.py
D   blender -b --factory-startup -P probe.py q13.blend -o /tmp/smoozy_q13_sentinel_out_
E   blender -b --factory-startup q13.blend --render-output /tmp/smoozy_q13_sentinel_out_ -P probe.py
```

**On the Creator's Mac**, 5.2 printed, and 4.5, 5.0 and 5.1 printed the same:

```
--- A: file, -o, -P
PROBE-P scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
PROBE-P scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
--- B: file, -P, -o
PROBE-P scene[Scene].render.filepath = '/tmp/smoozy_q13/from_file_scene_'
PROBE-P scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
--- C: -o, file, -P
PROBE-P scene[Scene].render.filepath = '/tmp/smoozy_q13/from_file_scene_'
PROBE-P scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
--- D: -P, file, -o
PROBE-P scene[Scene].render.filepath = '/tmp/'
--- E: file, --render-output, -P
PROBE-P scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
PROBE-P scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
```

**On the rented Linux machine** the identical five blocks came back in all four versions. **The two machines agreed, line for line.**

In arrangement D the script ran before the file was loaded, so it saw one scene rather than two, and that scene's output path read `/tmp/`, which is what the factory startup file holds.

### 3. A script that ran before the file was loaded, and a `load_post` handler

The setting first, then the script, then the file:

```
blender -b --factory-startup -o /tmp/smoozy_q13_sentinel_out_ -P handler.py q13.blend
```

where `handler.py` prints the scenes and their paths, registers a `@persistent` `load_post` handler that prints them again, and prints how many handlers `load_post` then holds.

**On the Creator's Mac**:

```
########## 4.5 ##########
HANDLER before load: scenes = [('Scene', '/tmp/smoozy_q13_sentinel_out_')]
HANDLER handler registered, load_post now holds 2
HANDLER after load: scenes = [('Scene', '/tmp/smoozy_q13/from_file_scene_'), ('Second', '/tmp/smoozy_q13/from_file_second_')]
exit code: 0
########## 5.0 ##########
HANDLER before load: scenes = [('Scene', '/tmp/smoozy_q13_sentinel_out_')]
HANDLER handler registered, load_post now holds 3
HANDLER after load: scenes = [('Scene', '/tmp/smoozy_q13/from_file_scene_'), ('Second', '/tmp/smoozy_q13/from_file_second_')]
exit code: 0
```

5.1 and 5.2 printed exactly what 5.0 printed. **The rented Linux machine printed the identical four blocks**, including the count of 2 in 4.5 and 3 in the other three. **The two machines agreed, line for line.**

The one difference between versions here is that count: **`load_post` already holds two things in 4.5 and three in 5.0, 5.1 and 5.2** before a script adds its own, on both machines.

### 4. Which wins when both a script and `-o` set the path, and what the render writes

A script that sets the path to `…/from_script_`, and `-o …/from_flag_`, in both orders, with a Workbench render of frame 1 after them:

```
A   blender -b --factory-startup q13.blend -P setpath.py -o W/from_flag_ -E BLENDER_WORKBENCH -f 1
B   blender -b --factory-startup q13.blend -o W/from_flag_ -P setpath.py -E BLENDER_WORKBENCH -f 1
```

**On the Creator's Mac**, 4.5 and 5.2 (5.0 and 5.1 printed what 5.2 printed):

```
########## 4.5 ##########
--- A: file, -P setpath, -o, -f 1
SETPATH before = '/tmp/smoozy_q13/from_file_scene_'
SETPATH after  = '…/work/e04/4.5/A/from_script_'
Saved: '…/work/e04/4.5/A/from_flag_0001.png'
files written: from_flag_0001.png
--- B: file, -o, -P setpath, -f 1
SETPATH before = '…/work/e04/4.5/B/from_flag_'
SETPATH after  = '…/work/e04/4.5/B/from_script_'
Saved: '…/work/e04/4.5/B/from_script_0001.png'
files written: from_script_0001.png
########## 5.2 ##########
--- A: file, -P setpath, -o, -f 1
SETPATH before = '/tmp/smoozy_q13/from_file_scene_'
SETPATH after  = '…/work/e04/5.2/A/from_script_'
00:00.753  render           | Saved: '…/work/e04/5.2/A/from_flag_0001.png'
files written: from_flag_0001.png
--- B: file, -o, -P setpath, -f 1
SETPATH before = '…/work/e04/5.2/B/from_flag_'
SETPATH after  = '…/work/e04/5.2/B/from_script_'
00:00.646  render           | Saved: '…/work/e04/5.2/B/from_script_0001.png'
files written: from_script_0001.png
```

(The `…` stands for `/Users/artist/smoozy-handtests/Writing-arch`, cut here only for width; the paths were printed in full.)

> **Citation check, 2026-08-26 (Claude Opus 5) — one step is missing from the two command lines above.** They do not say how `setpath.py` learns the path it is to write, and it differs between run A and run B. Re-running this meant adding `-- <the from_script_ path>` to the end of each line and having the script read it from there. With that added, both runs came back exactly as recorded on the Creator's Mac in all four versions — including which file each run wrote, and 4.5 printing the saving line bare where 5.0, 5.1 and 5.2 print it with a timestamp and a channel name.

**On the rented Linux machine** the same lines came back in all four versions, with `/root/smoozy-arch` in place of the Mac's folder — for example 4.5 run A: `Saved: '/root/smoozy-arch/work/e04/4.5/A/from_flag_0001.png'`. **The two machines agreed** on what the render did in every version.

**One difference between versions, on both machines.** 4.5 prints the saving line bare — `Saved: '…'` — where 5.0, 5.1 and 5.2 print it with a timestamp and a channel name in front: `00:00.753  render           | Saved: '…'`.

### 5. The read-back scheme end to end, and its exit code

A script that reads the output path back, compares it against a value handed in after `--`, prints `MATCH` or `MISMATCH`, and calls `sys.exit(1)` on a mismatch. Two launches per version:

```
A   blender -b --factory-startup q13.blend -o WA/out_ -P guard.py -E BLENDER_WORKBENCH -f 1 -- WA/out_
B   blender -b --factory-startup q13.blend -P guard.py -o WB/out_ -E BLENDER_WORKBENCH -f 1 -- WB/out_
```

**The first run of this printed no exit code**, because the shell line asked for it after an intervening command by which point it had been reset. The whole thing was run again with the code captured immediately; the two runs agree everywhere they overlap, and what follows is the second.

**On the Creator's Mac**, 4.5 (5.0, 5.1 and 5.2 printed the same, with the timestamped form of the saving line):

```
--- A intended order
GUARD expected = '…/work/e05/4.5/A/out_'
GUARD actual   = '…/work/e05/4.5/A/out_'
GUARD MATCH - job would proceed
Saved: '…/work/e05/4.5/A/out_0001.png'
exit code: 0
files written: out_0001.png
--- B script before the setting
GUARD expected = '…/work/e05/4.5/B/out_'
GUARD actual   = '/tmp/smoozy_q13/from_file_scene_'
GUARD MISMATCH - job stopped
exit code: 1
files written: 
```

**On the rented Linux machine** the same came back in all four versions, with `/root/smoozy-arch` in place of the Mac folder. **The two machines agreed, line for line, in all four versions, in both arrangements.**

So in the intended arrangement the comparison matched, the render went ahead, one frame was written and the process ended with 0; in the other arrangement the comparison did not match, the script stopped, **no frame was written even though `-f 1` stood later on the line**, and the process ended with 1.

### 6. What of the command line survives where a script can reach it as plain text

```
A   blender -b --factory-startup q13.blend -o /tmp/smoozy_q13_sentinel_out_ -P argv.py
B   blender -b --factory-startup q13.blend -o /tmp/smoozy_q13_sentinel_out_ -P argv.py -- --my-setting 7
```

**On the Creator's Mac**, 5.2:

```
--- A: no double dash
ARGV sys.argv = ['/Applications/Blender.app/Contents/MacOS/Blender', '-b', '--factory-startup', '/Users/artist/smoozy-handtests/Writing-arch/files/q13-5.2.blend', '-o', '/tmp/smoozy_q13_sentinel_out_', '-P', '/Users/artist/smoozy-handtests/Writing-arch/scripts/argv.py']
ARGV bpy.app.python_args = ('-I',)
ARGV bpy.app.binary_path = '/Applications/Blender.app/Contents/MacOS/Blender'
--- B: with a double dash
ARGV sys.argv = ['/Applications/Blender.app/Contents/MacOS/Blender', '-b', '--factory-startup', '/Users/artist/smoozy-handtests/Writing-arch/files/q13-5.2.blend', '-o', '/tmp/smoozy_q13_sentinel_out_', '-P', '/Users/artist/smoozy-handtests/Writing-arch/scripts/argv.py', '--', '--my-setting', '7']
ARGV bpy.app.python_args = ('-I',)
ARGV bpy.app.binary_path = '/Applications/Blender.app/Contents/MacOS/Blender'
```

4.5, 5.0 and 5.1 printed the same shape with their own binary paths.

**On the rented Linux machine**, 4.5:

```
--- A: no double dash
ARGV sys.argv = ['/opt/blender/4.5.12/blender', '-b', '--factory-startup', '/root/smoozy-arch/files/q13-4.5.blend', '-o', '/tmp/smoozy_q13_sentinel_out_', '-P', '/root/smoozy-arch/scripts/argv.py']
ARGV bpy.app.python_args = ('-I',)
ARGV bpy.app.binary_path = '/opt/blender/4.5.12/blender'
--- B: with a double dash
ARGV sys.argv = ['/opt/blender/4.5.12/blender', '-b', '--factory-startup', '/root/smoozy-arch/files/q13-4.5.blend', '-o', '/tmp/smoozy_q13_sentinel_out_', '-P', '/root/smoozy-arch/scripts/argv.py', '--', '--my-setting', '7']
ARGV bpy.app.python_args = ('-I',)
ARGV bpy.app.binary_path = '/opt/blender/4.5.12/blender'
```

5.0, 5.1 and 5.2 the same. **The two machines agreed, and all four versions agreed**: the whole command line is in `sys.argv`, the program's own path is item zero, `--` is present as its own item when it is typed, and `bpy.app.python_args` holds `('-I',)` and nothing of the launch.

### 7. A template and `#` marks handed to `-o`

```
A   blender -b --factory-startup q13.blend -o "/tmp/smoozy_q13_tmpl/{blend_name}_####" -P tmpl.py
B   blender -b --factory-startup q13.blend -o "//renders/out_####" -P tmpl.py
```

**On the Creator's Mac**, 5.2 (4.5, 5.0 and 5.1 printed the same, with their own file names in the middle line):

```
--- A
TMPL filepath raw = '/tmp/smoozy_q13_tmpl/{blend_name}_####'
TMPL frame_path(frame=7) = '/tmp/smoozy_q13_tmpl/q13-5.2_0007.png'
TMPL abspath = '/tmp/smoozy_q13_tmpl/{blend_name}_####'
--- B
TMPL filepath raw = '//renders/out_####'
TMPL frame_path(frame=7) = '/Users/artist/smoozy-handtests/Writing-arch/files/renders/out_0007.png'
TMPL abspath = '/Users/artist/smoozy-handtests/Writing-arch/files/renders/out_####'
```

**On the rented Linux machine**, 5.2:

```
--- A
TMPL filepath raw = '/tmp/smoozy_q13_tmpl/{blend_name}_####'
TMPL frame_path(frame=7) = '/tmp/smoozy_q13_tmpl/q13-5.2_0007.png'
TMPL abspath = '/tmp/smoozy_q13_tmpl/{blend_name}_####'
--- B
TMPL filepath raw = '//renders/out_####'
TMPL frame_path(frame=7) = '/root/smoozy-arch/files/renders/out_0007.png'
TMPL abspath = '/root/smoozy-arch/files/renders/out_####'
```

and the same in 4.5, 5.0 and 5.1 with their own file names. **The two machines agreed in all four versions.**

### 8. Other command-line settings, before and after the script

```
A   blender -b --factory-startup q13.blend -E CYCLES -s 7 -e 9 -j 2 -F OPEN_EXR -x 0 -t 3 -P settings.py
B   blender -b --factory-startup q13.blend -P settings.py -E CYCLES -s 7 -e 9 -j 2 -F OPEN_EXR -x 0 -t 3
C   blender -b --factory-startup q13.blend -P settings.py            (the control — no settings at all)
```

**On the Creator's Mac**, 5.2 (5.0 and 5.1 printed the same; 4.5 differs only in the engine name, given below):

```
--- A: settings before the script
SET engine = 'CYCLES'
SET frame_start = 7  frame_end = 9  frame_step = 2
SET file_format = 'OPEN_EXR'
SET use_file_extension = False
SET threads = 3  threads_mode = 'FIXED'
SET filepath = '/tmp/smoozy_q13/from_file_scene_'
SET resolution = 1920 x 1080  percent 100
--- B: settings after the script
SET engine = 'BLENDER_EEVEE'
SET frame_start = 1  frame_end = 250  frame_step = 1
SET file_format = 'PNG'
SET use_file_extension = True
SET threads = 3  threads_mode = 'FIXED'
SET filepath = '/tmp/smoozy_q13/from_file_scene_'
SET resolution = 1920 x 1080  percent 100
--- C: control, no -t at all
SET engine = 'BLENDER_EEVEE'
SET threads = 8  threads_mode = 'AUTO'
```

**On the rented Linux machine** the same lines came back in all four versions, **except the control run's thread count, which read `SET threads = 32  threads_mode = 'AUTO'`**. That number is the machine's own report of its size; the rented machine is a share of a bigger chip and misreports itself — its quota is about fifteen cores. It is not Blender and it is not the command line. Everywhere else the two machines agreed, line for line.

**One difference between versions, on both machines**: the engine the file's own scene holds reads `'BLENDER_EEVEE_NEXT'` in 4.5 and `'BLENDER_EEVEE'` in 5.0, 5.1 and 5.2.

**One setting behaved differently from all the others.** `-t 3` gave `threads = 3  threads_mode = 'FIXED'` in **both** run A and run B — written before the script or after it — in all four versions on both machines, against a control that printed `AUTO` and the machine's own core count. Engine, frame range, frame step, format and the extension switch all followed the order they were written in; `-t` did not.

### 9. The five ways of handing a script in at one launch

```
1   blender -b --factory-startup q13.blend -o SENT -P probe.py
2   blender -b --factory-startup q13.blend -o SENT --python-expr "import bpy; print(…)"
3   blender -b --factory-startup q13.blend -o SENT --python-text probe
4   blender -b --factory-startup q13.blend -o SENT -P cmdreg.py --command q13probe
5   echo "import bpy; print(…)" | blender -b --factory-startup q13.blend -o SENT --python-console
```

where `SENT` is `/tmp/smoozy_q13_sentinel_out_`, `probe` is the text block inside the file, and `cmdreg.py` registers a command with `bpy.utils.register_cli_command(id="q13probe", execute=run)`.

> **Citation check, 2026-08-26 (Claude Opus 5) — ways 2 and 5 are written with an ellipsis.** `print(…)` is not the text a repeater types, so the exact expression handed to `--python-expr` and piped into `--python-console` has to be reinvented. Written as a print of the active scene's name and render path, all five ways came back exactly as recorded on the Creator's Mac in all four versions, `>>> ` prefix and empty command list included.

**Way 5 appeared at first to print nothing**, because the filter used to pick its line out was anchored to the start of the line and the interactive console puts `>>> ` in front of what it echoes. Its raw output, on the Mac in 5.2, was:

```
Python 3.13.13 (main, Apr 25 2025, 12:39:20) [Clang 21.0.0 (clang-2100.0.123.102)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
(InteractiveConsole)
>>> PROBE-CONSOLE scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
>>> 
now exiting InteractiveConsole...
```

so the run had worked all along, and it was repeated in all four versions on both machines with the filter widened.

**On the Creator's Mac**, 4.5:

```
--- way 1: -P
PROBE-P scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
--- way 2: --python-expr
PROBE-EXPR scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
--- way 3: --python-text probe
PROBE-TEXT scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
--- way 4: -P cmdreg.py --command q13probe
PROBE-CMD command registered
PROBE-CMD scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
PROBE-CMD scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
PROBE-CMD args handed to the command = []
--- way 5: --python-console
>>> PROBE-CONSOLE scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
```

5.0, 5.1 and 5.2 printed the same. **On the rented Linux machine** the same came back in all four versions. **The two machines agreed, and all four versions agreed: all five ways printed the same path.** None errored, and `--python-text probe` found the text block inside the file in every version.

### 10. Two scripts at one launch with the setting written between them

```
blender -b --factory-startup q13.blend -P probe.py -o /tmp/smoozy_q13_sentinel_out_ -P probe2.py
```

**On the Creator's Mac**, all four versions:

```
PROBE-P scene[Scene].render.filepath = '/tmp/smoozy_q13/from_file_scene_'
PROBE-P2 scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
```

**The rented Linux machine printed the same four pairs.** **The two machines agreed, line for line, in all four versions.**

### 11. `-y` and `-Y`

```
A   blender -b --factory-startup q13.blend -P autoexec.py
B   blender -b --factory-startup -y q13.blend -P autoexec.py
C   blender -b --factory-startup -Y q13.blend -P autoexec.py
```

**On the Creator's Mac**, all four versions printed exactly this, in all three runs:

```
AUTOEXEC script ran
AUTOEXEC preferences.filepaths.use_scripts_auto_execute = False
AUTOEXEC bpy.app.autoexec_fail = False
```

**The rented Linux machine printed exactly the same**, in all four versions, in all three runs. **The two machines agreed, line for line.** The `-P` script ran under all three, and the preference read `False` under all three.

### 12. `--cycles-device`

Three probe runs and a pair of blind searches — one with the argument, one without — per version on each machine. The device name differs by machine because the two have different cards: `METAL` on the Mac, `OPTIX` on the rented Linux machine's NVIDIA card.

**On the Creator's Mac**, 4.5 (5.0, 5.1 and 5.2 printed the same):

```
--- control, no cycles argument
CYCDEV scene.cycles.device = 'CPU'
CYCDEV preferences.compute_device_type = 'METAL'
--- -- --cycles-device CPU
CYCDEV scene.cycles.device = 'CPU'
CYCDEV preferences.compute_device_type = 'METAL'
--- -- --cycles-device METAL
CYCDEV scene.cycles.device = 'CPU'
CYCDEV preferences.compute_device_type = 'METAL'
--- blind search, control, for METAL
HIT bpy.context.preferences.addons[5].preferences.compute_device_type = 'METAL'
HIT bpy.context.preferences.system.gpu_backend = 'METAL'
SEARCH hits: data+preferences = 2 ; bpy.app = 0 ; sys.argv = 0
--- blind search, with -- --cycles-device METAL, for METAL
HIT bpy.context.preferences.addons[5].preferences.compute_device_type = 'METAL'
HIT bpy.context.preferences.system.gpu_backend = 'METAL'
HIT sys.argv[10] = 'METAL'
SEARCH hits: data+preferences = 2 ; bpy.app = 0 ; sys.argv = 1
```

**On the rented Linux machine**, 4.5 (5.0, 5.1 and 5.2 the same):

```
--- control, no cycles argument
CYCDEV scene.cycles.device = 'CPU'
CYCDEV preferences.compute_device_type = 'NONE'
--- -- --cycles-device CPU
CYCDEV scene.cycles.device = 'CPU'
CYCDEV preferences.compute_device_type = 'NONE'
--- -- --cycles-device OPTIX
CYCDEV scene.cycles.device = 'CPU'
CYCDEV preferences.compute_device_type = 'NONE'
--- blind search, control, for OPTIX
HIT bpy.context.preferences.addons[5].preferences.devices[0].type = 'OPTIX'
SEARCH hits: data+preferences = 1 ; bpy.app = 0 ; sys.argv = 0
--- blind search, with -- --cycles-device OPTIX, for OPTIX
HIT bpy.context.preferences.addons[5].preferences.devices[0].type = 'OPTIX'
HIT sys.argv[10] = 'OPTIX'
SEARCH hits: data+preferences = 1 ; bpy.app = 0 ; sys.argv = 1
```

In both cases **the count of hits outside `sys.argv` is the same with the argument and without it**. What the two machines print differently — `'METAL'` against `'NONE'`, and a `gpu_backend` against a `devices[0].type` — is present in the control run before any Cycles argument is typed, so it is the hardware, not the command line. **The two machines agreed on what the command line did, in all four versions.**

### 13. Whether add-ons are registered when a launch-time script runs

```
A   blender -b --factory-startup q13.blend -P addons.py
B   blender -b --factory-startup -P addons.py q13.blend
```

**The first version of the script asked the wrong question** — its last line read the built-in list of engine choices, which does not include an engine an add-on registers, and it printed `ADDON CYCLES in engine enum = False` in every version on both machines. That line settles nothing. The script was changed to list the render engines actually registered and the whole thing was run again everywhere; the other four lines printed identically in both runs.

> **Citation check, 2026-08-26 (Claude Opus 5) — one step is missing from "list the render engines actually registered".** Written the obvious way, that line stops with `AttributeError: type object 'HydraRenderEngine' has no attribute 'bl_idname'`, and nothing here says so. It has to fall back to the class's own name where there is no `bl_idname`. With that added, all five lines came back exactly as recorded on the Creator's Mac in all four versions, in both arrangements — `['CYCLES', 'HydraRenderEngine']` included.

**On the Creator's Mac**, 4.5 (5.0, 5.1 and 5.2 printed the same), in both arrangements:

```
ADDON enabled add-ons = ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
ADDON '_cycles' in sys.modules = True
ADDON 'cycles' in sys.modules = True
ADDON scene has cycles attr = True
ADDON registered render engines = ['CYCLES', 'HydraRenderEngine']
```

**The rented Linux machine printed exactly the same**, in all four versions, in both arrangements. **The two machines agreed, line for line.**

### 14. Whether a script can write to the `bpy.app` flags

```
blender -b --factory-startup q13.blend -P appwrite.py
```

**On the Creator's Mac**, all four versions printed exactly this:

```
APPWRITE bpy.app.debug write accepted, now True
APPWRITE bpy.app.background write refused: AttributeError: readonly attribute
```

**The rented Linux machine printed exactly the same two lines**, in all four versions. **The two machines agreed.**

### 15. The same launch with a real window open, and what the Output panel shows

Run with no `-b`, so a real window opened, with a script that prints what it can see, switches the Properties editor to its Output tab, photographs Blender's window and quits it. On the Creator's Mac the window opened on the Mac's own screen and the script closed it after about six seconds; on the rented Linux machine it opened on that machine's own display server (`DISPLAY=:1`, started with `/usr/local/bin/start-display.sh`, with `XDG_RUNTIME_DIR=/run/user/0`).

**Two things went wrong first and are recorded rather than tidied away.** On the rented Linux machine the picture Blender takes of its own window came back **entirely black**, 1920 × 1061, in all four versions; the same script on the Mac produced a proper picture, so the window there was photographed by the display server instead, from outside the Blender process. And the first Linux run used `--factory-startup` with an empty settings folder, and the photograph shows Blender's first-run **Quick Setup panel sitting over the middle of the layout** with the Properties editor still on its Object tab — a panel that needs a click to dismiss and no hand was available. Both were fixed the same way: a preferences file was saved into each settings folder first and `--factory-startup` dropped, and the script switches to the Output tab at load as well as on its timer.

**On the Creator's Mac**, 5.2's printed lines (4.5, 5.0 and 5.1 the same, with their own paths):

```
WIN scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
WIN scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
WIN sys.argv = ['/Applications/Blender.app/Contents/MacOS/Blender', '/Users/artist/smoozy-handtests/Writing-arch/files/q13-5.2.blend', '-o', '/tmp/smoozy_q13_sentinel_out_', '-P', '/Users/artist/smoozy-handtests/Writing-arch/scripts/winprobe.py']
WIN bpy.app.background = False
WIN windows open = 1
WIN properties editors switched to the Output tab at load: 1
WIN properties editors switched to the Output tab on the timer: 1
```

**On the rented Linux machine**, 5.2's:

```
WIN scene[Scene].render.filepath = '/tmp/smoozy_q13_sentinel_out_'
WIN scene[Second].render.filepath = '/tmp/smoozy_q13/from_file_second_'
WIN sys.argv = ['/opt/blender/5.2.1/blender', '/root/smoozy-arch/files/q13-5.2.blend', '-o', '/tmp/smoozy_q13_sentinel_out_', '-P', '/root/smoozy-arch/scripts/winprobe.py']
WIN bpy.app.background = False
WIN windows open = 1
WIN properties editors switched to the Output tab at load: 1
WIN properties editors switched to the Output tab on the timer: 1
```

and 4.5, 5.0 and 5.1 the same on both machines.

**The pictures.** Eight in all — four taken by Blender on the Mac, four taken by the display server on Linux — and all eight were looked at. In every one, the Properties editor is on its Output tab and **the path field at the top of the Output panel reads `/tmp/smoozy_q13_sentinel_out_`**, in 4.5, 5.0, 5.1 and 5.2, on both machines. **The two machines agreed.**

One thing visible in every picture, and already on record for this topic from an earlier round: **4.5's Output panel has no Media Type row**, where 5.0, 5.1 and 5.2 have one reading `Image` between Saving and File Format.

One thing a windowed launch left behind on Linux that no windowless run did, in all four versions:

```
00:05.932  blend            | Saved session recovery to "/tmp/quit.blend"
```

In 4.5 it printed bare, without the timestamp and channel: `Saved session recovery to "/tmp/quit.blend"`.

### 16. Whether the `bpy.app` flags report what the launch was given

Four launches per version on each machine, reading `bpy.app.background`, `factory_startup`, `debug`, `debug_python`, `online_access`, `online_access_override` and `use_event_simulate`:

```
A   blender -b --factory-startup q13.blend -P appflags.py
B   blender -b --factory-startup --debug-python q13.blend -P appflags.py
C   blender -b --factory-startup --offline-mode q13.blend -P appflags.py
D   blender -b q13.blend -P appflags.py
```

Run D leaves `--factory-startup` off so the flag reporting it has something to differ from.

**On the Creator's Mac**, 4.5 (5.0, 5.1 and 5.2 printed exactly the same):

```
--- A: -b --factory-startup
APPFLAG bpy.app.background = True
APPFLAG bpy.app.factory_startup = True
APPFLAG bpy.app.debug = False
APPFLAG bpy.app.debug_python = False
APPFLAG bpy.app.online_access = False
APPFLAG bpy.app.online_access_override = False
APPFLAG bpy.app.use_event_simulate = False
--- B: -b --factory-startup --debug-python
APPFLAG bpy.app.debug_python = True
--- C: -b --factory-startup --offline-mode
APPFLAG bpy.app.online_access_override = True
--- D: -b (no --factory-startup)
APPFLAG bpy.app.factory_startup = False
```

(Runs B, C and D printed all seven lines each; only the line that changed from run A is repeated here, and every other line in those runs matched run A exactly.)

**The rented Linux machine printed exactly the same**, in all four versions. `--offline-mode` was accepted by every version on both machines and printed no complaint. **The two machines agreed, line for line.**

Three flags moved with the launch — `factory_startup`, `debug_python` and `online_access_override` — and `background` read `True` in all four runs, which is what `-b` was in all four.


### What these runs did not settle

- **Anything on Windows.** No Windows machine has run it, so every result above covers macOS and Linux and leaves Windows untested rather than answered.
- **What the Cycles device override actually does to a render.** The runs above show only that nothing readable is left behind by it.
- **Whether `-o` reaching only the active scene depends on which scene is active**, or on how a file names its scenes. One file with two scenes was used.


### 1. Outside the interface

**Brought up to date by the hand tests of 2026-08-26.** This is no longer only what the sources say. Everything above was worked out for a Blender driven from outside its own window, and every test was run that way — but section 15 of the observed results ran the same launch **with a real window open**, on both machines, in all four versions. The window made no difference to what a launch-time script sees: the same path came back from the script, and the path field at the top of the Output panel in Blender's own window showed that same path in every one of the eight pictures. So this holds with a window and without one, on macOS and on Linux, by test rather than by construction.

**What the sources say, unchanged.**

**LABEL: inferred** for the framing, **not in the sources** for the rest.

This whole answer is about the outside-the-interface case. **Inferred:** every source I read describes these arguments as things given when Blender is started from a command line, and none describes an argument reaching a Blender started some other way, so there is no "inside" version of the question to answer. The step is mine — no page states that a Blender started without a command line receives no arguments, and I have not tried it.

**Not in the sources:** nothing I read says whether any of the above changes between a background launch (`-b`) and a command-line launch that opens the window. Where I looked is the record below; the arguments page and both launch-related manual pages were read in full, and the only per-launch distinctions either draws are the ones quoted here. Two documented points touch this:

- The security page says of `-y` / `-Y`: "These command-line arguments can be used to start a regular Blender instance and will still override the Preferences." (the manual, https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html, read 2026-07-30). So at least those arguments are documented to work in both.
- `bpy.app.background` is documented as "True when blender is running without a user interface (started with -b)" (the Python API documentation, https://docs.blender.org/api/5.1/bpy.app.html, read 2026-07-30) — so a script can tell which of the two it is in.

One caution the sources do document, about a `--python` script that runs before the file: "While this works for background tasks it has the downside that opening the file from the file selector won't setup the name-space." (the Python API documentation, https://docs.blender.org/api/5.1/bpy.app.html, read 2026-07-30). That is a difference of the expected kind — the command-line arrangement covers the command-line launch and not a later click inside the window.

Beyond those, the sources do not say.

### 2. Operating system

**Brought up to date by the hand tests of 2026-08-26.** This is no longer only what the sources do or do not say. Every result in the observed section above was got **twice** — once on the Creator's Mac (MacBook Pro, Apple M1 Pro, macOS 26.5.2) and once on a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 3060, driver 595.84) — on builds matching to the same commit hash. **The two machines agreed on every one of the sixteen, line for line, in all four versions.** The only lines that differed are two the machine itself supplies rather than Blender: the core count a control run reports (8 on the Mac, 32 on the rented machine, which misreports its own share of a bigger chip), and the graphics device the preferences hold (`METAL` on the Mac; `NONE`, with a device of type `OPTIX` listed, on the Linux machine) — both present in a control run before any command-line argument is given. **Windows is still untested** — no Windows machine has run it — and that is the whole of what is left open here.

**What the sources say, unchanged.**

**LABEL: not in the sources.**

Nothing I read says that argument order, `-o`, `-P`, `sys.argv`, or the `bpy.app` flags behave differently on any operating system. Nothing on the arguments page marks any of them as OS-limited. Where I looked: the arguments page in full, the render page, and all three per-operating-system launch pages, all listed in the record below.

What the arguments page *does* mark as OS-limited is a different, and short, list — none of it touching this question:

> "-con, --start-console
>
> Start with the console window open (ignored if -b is set), (Windows only)."

> "--no-window-frame
>
> Disable all window decorations (Linux only)."

> "/?
>
> Print this help text and exit (Windows only)."

> "--qos <level>
>
> Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only)."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html. Date read: 2026-07-30.

There is one documented warning that is not an OS difference but is easily mistaken for one, and it applies everywhere:

> "Arguments are case sensitive! -F and -f are not the same."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html. Date read: 2026-07-30.

The three per-operating-system launching pages differ only in how to reach the executable. The macOS one says:

> "The path to the executable in the .app folder is ./Blender.app/Contents/MacOS/Blender."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/macos.html. Date read: 2026-07-30. The other two are https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/linux.html and https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/windows.html, read the same day.

**LABEL: not in the sources** for the rest of what those pages could have settled: none of the three says anything about how arguments or scripts are handled once Blender is running. All three were read in full.

---

## Record of where I looked

**Searches run across all four source sets (manual_5.1, api_5.1, release_notes_5.1, cycles_5.1) at once:**

- `sys.argv` — four hits: `api_5.1/bpy.utils.txt`, `manual_5.1/advanced/command_line/arguments.txt`, `manual_5.1/troubleshooting/python.txt`, `cycles_5.1/engine.py`. All four opened and read. The troubleshooting one turned out to be about locating duplicate libraries on Windows, not about arguments — ruled out.
- `argv` — same four files, no extra hits beyond those.
- `command line`, `command-line`, `commandline` — swept the whole API documentation and the manual. Every hit opened. The ones that mattered: `bpy.app.txt` (the `--python` before-the-file note and `online_access_override`), `bpy.utils.txt` (`register_cli_command`), `info_advanced_blender_as_bpy.txt`, `info_tips_and_tricks.txt`, `advanced/scripting/security.txt`. Ruled out: `bpy.types.PreferencesExperimental.txt` (about `--disable-liboverride-auto-resync`), `bpy.app.handlers.txt` line about `exit_pre` and failed arguments, `bpy.types.RenderSettings.txt` line 702 (about `use_single_layer` being "ignored for rendering from command line" — a real interface/command-line difference, but about rendering, not about what a script sees), `info_quickstart.txt`, `info_overview.txt`, `render/cycles/gpu_rendering.txt` (an environment variable, not an argument).
- `--python` — hits in `arguments.txt` (the Python Options block), `bpy.app.txt`, `info_tips_and_tricks.txt`, `info_overview.txt`, `deploying_blender.txt` (`--python-use-system-env`), `advanced/extensions/python_wheels.txt` (pip's own `--python-version`, unrelated — ruled out).
- `python-expr`, `python-text` — only in `arguments.txt`. No page describes their ordering or what they see.
- `order they are given`, `executed in the order`, `in the order` — the only two relevant hits are `arguments.txt` and `render.txt`; every other hit was about mesh points, strokes, strips, or bone collections and was ruled out on sight.
- `startup script` — two hits, both about auto-run of scripts, not about ordering.
- `execfile`, `run_script` — `bpy.utils.execfile` and `bpy.ops.text.run_script`; neither says anything about command-line settings.
- `render-output`, `render_output`, `--render-frame` — outside `arguments.txt` the only hits were image filenames in the manual (`_images/render_output_properties_*.png`, panel screenshots) and `bpy.types.PreferencesFilePaths.render_output_directory`. That last one is a preference for the default output directory, not the command-line argument; ruled out as not the thing asked about.
- `online_access`, `use_scripts_auto_execute`, `use_online_access` — traced to `bpy.app.txt`, `bpy.types.PreferencesSystem.txt`, `bpy.types.PreferencesFilePaths.txt`. All read.
- `_parse_command_line` across the Cycles files — defined at `engine.py` line 24, called once, from `init()` at line 53. No other Cycles file parses arguments.
- `argument` across the release notes — only four hits, all read: `core.md` (`--python-use-user-env` and `--no-window-frame`), `bugfixes.md` (a documentation fix for `foreach_set`), `corrective_releases.md` (two entries about operator keyword arguments). None of them bears on what a `-P` script sees of a command-line setting. `release_notes_5.1/python_api.md` read in full — nothing on command-line arguments or `sys.argv`.

**Where this record is thin, said plainly rather than left to be noticed:**

- `command line`, `command-line` and `commandline` were swept across the manual and the API documentation, **but not across the release notes**. The release notes were swept for `argument` only, plus two of their files read in full. A release-notes line using the words "command line" without the word "argument" would not have been caught.
- The words the autoexec switch is spelled with — `autoexec`, `enable-autoexec`, `auto-run` — were never searched. See part B.
- Nothing was searched for the words that would tie `-o` to a Python property: the `-o` entry's own phrase "render path", or `filepath` across the manual. `render-output` and `render_output` were searched, which finds the argument and the preference but would not find a page describing the tie in other words.
- Nothing was searched on when a `-P` script runs relative to add-ons coming up: no sweep for `initialize`, `initialization`, `register`, `add-on registration`, `load order` or `startup order`. The one section read on this was `bpy.app`'s "File Loading & Order of Initialization", found by opening the page rather than by a search.

**Pages opened and read in full:**

- `manual_5.1/advanced/command_line/arguments.txt` (all 691 lines) → https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
- `manual_5.1/advanced/command_line/render.txt` → https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html
- `manual_5.1/advanced/command_line/index.txt` → https://docs.blender.org/manual/en/5.1/advanced/command_line/index.html
- `manual_5.1/advanced/command_line/launch/linux.txt`, `.../macos.txt`, `.../windows.txt` → the three pages under https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/
- `manual_5.1/advanced/scripting/security.txt` → https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html
- `api_5.1/bpy.app.txt` (the whole attribute list, plus the "File Loading & Order of Initialization" section) → https://docs.blender.org/api/5.1/bpy.app.html
- `api_5.1/bpy.app.handlers.txt` (the persistent-handler and `load_post` sections) → https://docs.blender.org/api/5.1/bpy.app.handlers.html
- `api_5.1/bpy.utils.txt` (the `register_cli_command` section and both examples) → https://docs.blender.org/api/5.1/bpy.utils.html
- `api_5.1/info_advanced_blender_as_bpy.txt` → https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html
- `api_5.1/info_tips_and_tricks.txt` (the "Use Blender without its User Interface" section) → https://docs.blender.org/api/5.1/info_tips_and_tricks.html
- `api_5.1/bpy.types.RenderSettings.txt` (the `filepath` entry) → https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
- `api_5.1/bpy.types.PreferencesSystem.txt` (the `use_online_access` entry) → https://docs.blender.org/api/5.1/bpy.types.PreferencesSystem.html
- `api_5.1/bpy.types.PreferencesFilePaths.txt` (the `use_scripts_auto_execute` entry) → https://docs.blender.org/api/5.1/bpy.types.PreferencesFilePaths.html
- `release_notes_5.1/core.md` → https://developer.blender.org/docs/release_notes/5.1/core/
- `release_notes_5.1/python_api.md` → https://developer.blender.org/docs/release_notes/5.1/python_api/
- `cycles_5.1/engine.py` (lines 1–60) — cited by file and line, as it has no address.

**Pictures:**

The only images in the manual whose names relate to the command line are `advanced_command-line_launch_linux_console.png`, `advanced_command-line_launch_linux_kde.png`, `advanced_command-line_launch_macos_mac.png`, `advanced_command-line_launch_windows_console.png`. I opened `advanced_command-line_launch_linux_console.png`. It shows a terminal in which `blender` was run with no arguments at all, followed by audio-backend warnings about JACK. It shows nothing about arguments, scripts, or output paths, so I ruled it out and did not use it. The other three are, by their captions, the same kind of picture — a terminal or a menu-configuration dialog — and I did not need them.

**Ruled out without opening:**

`manual_5.1/advanced/command_line/extension_arguments.txt` — its index entry describes it as covering "Package Management / Repository Management / Extension Creation" for the `--command extension` sub-command, which is about installing and building extensions, not about what a launch-time script sees of a setting. I checked its heading list through the index page rather than reading it as a source.

**What I did not do:** I did not use the web. I did not need to. I did not read any other part of this project.

---

## What a running Blender could still settle — pruned 2026-08-26

**The list the researcher wrote here has been run, and it is gone from this section rather than left standing beside its own answers.** Every item on it was carried out on 2026-08-26, on both machines — the Creator's Mac (MacBook Pro, Apple M1 Pro, macOS 26.5.2) and a rented Linux machine (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 3060) — in Blender 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.1, and every result is in the observed section above. The full list, with every command written out and every raw line kept, is `Audits/Blender Research/Writing/Hand tests — Architecture questions.md`.

**What is genuinely left.**

- **All of it on Windows.** The researcher's own item 8 asked for the checks to be repeated there. **No Windows machine has run it**, so the Windows answer is untested rather than answered — for whether the ordering rule holds there, whether `-o` lands in the same place, what `sys.argv` holds once a Windows shell has split the line, and whether a path written with backslashes reads back as typed. This is the reason for the silence, written in so it is not read as a result.
- **What the Cycles device override actually does to a render.** The runs above show only that it leaves nothing behind that a script can read; they do not show what the renderer then uses.
- **Whether `-o` reaching the active scene alone depends on which scene is active**, or on how a file names its scenes. One file, with two scenes, was used.

---

2026-08-25 — Claude Opus 5 (Claude Code subagent) — Writing, Blender 5.1.

Declaration: I started no helper agents. All the reading above is my own. I opened only the question sheet `Preparation/Blender Research/Questions/Writing.md` and files inside `Blender Sources/`. I listed the names of the files in the findings folder I was told to save into, in order to save my own file there, and I opened none of them. I did not use the web.
