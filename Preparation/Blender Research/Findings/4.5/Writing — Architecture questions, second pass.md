# Writing — Architecture questions, second pass (Blender 4.5)

Sources consulted, all on **2026-07-30** (the snapshot date, used as the date read for every finding below):

- Blender 4.5 manual, plain-text copy in `Blender Sources/manual_4.5` (pages at https://docs.blender.org/manual/en/4.5/)
- Blender 4.5 Python API documentation, plain-text copy in `Blender Sources/api_4.5` (pages at https://docs.blender.org/api/4.5/)
- Blender 4.5 release notes, in `Blender Sources/release_notes_4.5` (pages at https://developer.blender.org/docs/release_notes/4.5/)
- Cycles' own files, in `Blender Sources/cycles_4.5`

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered here: **13 only.**

---

## 13. When Blender is started with a setting given on the command line and with a script to run at the same launch, what does that script see of that setting?

### The question was ambiguous — the reading I answered, and the other one

"A setting given on the command line" can mean two different things, and the answer differs for each. I answered **both**, because the sources treat them as two separate mechanisms:

- **Reading A (the one I take to be meant, since the sheet is about writing):** a setting that Blender itself understands and applies, such as `-o` / `--render-output`, given in the same launch as `-P` / `--python`. The question is then: does the script find that setting already applied to the data it can read?
- **Reading B:** a value put on the command line purely for the script's benefit, after the `--` separator. The question is then: how does the script get at it?

I have not rewritten the question; I have answered A first and B second.

### Short answer

**Reading A.** What the script sees is decided entirely by **where the setting sits relative to the script in the command line, and relative to the blend-file**. Blender executes command-line arguments strictly left to right, and loading the blend-file overwrites the render output path that was set before it. So a script sees a `-o` setting only when `-o` appears *before* the `-P` and *after* the blend-file. Both of those facts are documented. The step from them to "and therefore the script reads the new value out of Blender's data" is mine, and is labelled inferred, because the sources never say which Python value `-o` writes into.

**Reading B.** Documented and simple: everything after `--` reaches the script through Python's `sys.argv`, untouched by Blender.

### LABEL

**Mixed, part by part** — each part below carries its own label. In summary: the ordering rule and the blend-file overwrite are **documented**; the visible-to-the-script consequence for `-o` is **inferred**; the `sys.argv` route and the handful of `bpy.app` flags are **documented**; the working example of an in-Blender script reading the command line is **observed** in Cycles' files; the specific link "`-o` sets *this named* Python property, which a script may read" is **not in the sources**.

---

### Part 1 — Arguments run in the order given (documented)

Exact words:

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

- Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html (section "Argument Order")
- Date read: 2026-07-30

The same rule is stated a second time, with emphasis, on the rendering page:

> "Arguments are executed in the order they are given!"

and:

> "The following command will not work, since the output and extension are set after Blender is told to render:
>
> blender -b file.blend -a -x 1 -o //render
>
> The following command will behave as expected:
>
> blender -b file.blend -x 1 -o //render -a
>
> Always position -f or -a as the last arguments."

- Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html
- Date read: 2026-07-30

Two things this establishes and that matter here. First, the ordering rule is stated for arguments in general, not for rendering arguments only — the script argument is an argument like any other. Second, **loading the blend-file is itself one of the ordered steps, and it overwrites the render output path set before it.**

The script argument itself:

> "-P, --python <filepath>
>
> Run the given Python script file."

- Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html (section "Python Options")
- Date read: 2026-07-30

### Part 2 — A script can be placed before or after the blend-file, and this is documented as a real choice (documented)

Exact words:

> "It is possible to run a script via the --python command line argument, before the blend file.
> This can register a load-post handler (bpy.app.handlers.load_post) that initialized the name-space.
> While this works for background tasks it has the downside that opening the file from the file selector
> won't setup the name-space."

and, immediately above it:

> "Populating the driver name-space before the blend-file loads also doesn't work
> since opening a file clears the name-space."

- Source: the API documentation, https://docs.blender.org/api/4.5/bpy.app.html (section "File Loading & Order of Initialization")
- Date read: 2026-07-30

This is the API documentation confirming, from the script's side, what the manual says from the command line's side: a `--python` script placed before the blend-file runs before the file is loaded, and the file load then wipes state that the script set up.

The handler it names:

> "bpy.app.handlers.load_post
>
> on loading a new blend file (after). Accepts one argument: the file being loaded, an empty string for the startup-file."

- Source: the API documentation, https://docs.blender.org/api/4.5/bpy.app.handlers.html
- Date read: 2026-07-30

### Part 3 — What the script therefore sees of a `-o` setting (inferred)

The step I took, written out:

1. The manual says arguments execute in the order given.
2. The manual says `-o, --render-output <path>` — "Set the render path and file name."
3. The manual says loading the blend-file overwrites the render output that was set.
4. The manual says `-P` runs the given script, and the API documentation confirms a `--python` script placed before the blend-file runs before the file loads.
5. A script can read Blender's current settings while it runs (this is what the Python API is for).

From those, three placements and three outcomes:

- `blender -b file.blend -o /tmp/out_ -P script.py` — the file loads, then `-o` is applied, then the script runs. **The script sees the `-o` value.**
- `blender -b -o /tmp/out_ file.blend -P script.py` — `-o` is applied, then the file load overwrites it, then the script runs. **The script sees the path stored in the blend-file, not `/tmp/out_`.**
- `blender -b file.blend -P script.py -o /tmp/out_` — the script runs before `-o` is applied. **The script sees the path stored in the blend-file.** Anything the script writes into the render path is then overwritten by the later `-o`.

Label: **inferred.** Step 5 is mine, and the sources never state the specific consequence for a script. A reader has to supply the connection between "the output path was set" and "a script reading Blender's data now finds the new value", so this is not documented.

### Part 4 — The one thing the sources do not say: which Python value `-o` lands in (not in the sources)

The manual never names the Python property that `-o` writes to, and the API page for the render settings never mentions the command line. The nearest the API gets is the plain description of the property, with no command-line link:

> "filepath
>
> Directory/name to save animations, # characters define the position and padding of frame numbers"

- Source: the API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Date read: 2026-07-30

So "the script reads the `-o` value from `bpy.context.scene.render.filepath`" is a plausible reading of Part 3, but it is **not** something I can show from these sources, and I am not claiming it. What I can show is only that the setting is applied at its place in the order, and that the blend-file load overwrites it.

Where I looked before saying this (see the full search record at the end): every occurrence of `render-output` and `render_output` across the manual, the API documentation and the release notes; every occurrence of "command line" and "command-line" across the API documentation; the whole of the render settings API page for `filepath`, `use_file_extension`, `engine` and `threads`; the whole of the manual's command-line section.

### Part 5 — What the sources *do* say a script can see of the command line

**(a) Anything after `--`, through `sys.argv` (documented).**

> "--
>
> End option processing, following arguments passed unchanged. Access via Python's sys.argv."

- Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html (section "Misc Options")
- Date read: 2026-07-30

This is Reading B, and it is the only route the documentation states outright for a script to read a value handed in at launch. Note the words "passed unchanged": Blender does not interpret them, so the script sees exactly the text that was typed.

**(b) A small set of switches, through read-only `bpy.app` values (documented).**

> "This module contains application values that remain unchanged during runtime."

> "bpy.app.background
>
> Boolean, True when blender is running without a user interface (started with -b)"

> "bpy.app.debug
>
> Boolean, for debug info (started with --debug / --debug-* matching this attribute name)"

> "bpy.app.factory_startup
>
> Boolean, True when blender is running with –factory-startup)"

> "bpy.app.online_access_override
>
> Boolean, true when internet access preference is overridden by the command line (read-only)"

> "bpy.app.use_event_simulate
>
> Boolean, for application behavior (started with --enable-* matching this attribute name)"

- Source: the API documentation, https://docs.blender.org/api/4.5/bpy.app.html
- Date read: 2026-07-30

So for a handful of switches — background mode, the debug switches, factory startup, the online/offline override, event simulation — there *is* a documented, named place where a script reads what the launch was given. The output path is not among them. The manual confirms the intent for the online case:

> "Working offline can be enforced by running with the --offline-mode
> command line argument. Users
> will then be unable to enable online access in the preferences."
>
> "Add-ons that follow this setting will only connect to the internet if enabled."

- Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html
- Date read: 2026-07-30

**(c) Through a registered command, with `-c` / `--command` (documented).**

> "bpy.utils.register_cli_command(id, execute)
>
> Register a command, accessible via the (-c / --command) command-line argument."

> "execute (callable) – Callback, taking a single list of strings and returns an int.
> The arguments are built from all command-line arguments following the command id."

- Source: the API documentation, https://docs.blender.org/api/4.5/bpy.utils.html
- Date read: 2026-07-30

The manual's side of the same feature, which also warns that this changes how the launch behaves:

> "-c, --command <command>
>
> Run a command which consumes all remaining arguments.
> Use -c help to list all other commands.
> Pass --help after the command to see its help text.
>
> This implies --background mode."

- Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

### Part 6 — A working example of a script reading the command line, in the files supplied (observed)

Cycles is a Python add-on, and it reads the command line exactly this way. The whole of its parsing:

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

- File: `Blender Sources/cycles_4.5/engine.py`, lines 24–41 (the function `_parse_command_line`)
- Date read: 2026-07-30

And the parser it builds, in the same file, lines 8–21:

```
def _configure_argument_parser():
    import argparse
    # No help because it conflicts with general Python scripts argument parsing
    parser = argparse.ArgumentParser(description="Cycles Addon argument parser",
                                     add_help=False)
```

- File: `Blender Sources/cycles_4.5/engine.py`, lines 8–12
- Date read: 2026-07-30

It is called at the end of the add-on's own start-up:

```
    _cycles.init(path, user_path, bpy.app.background)
    _parse_command_line()
```

- File: `Blender Sources/cycles_4.5/engine.py`, lines 52–53 (in `init`)
- Date read: 2026-07-30

Label: **observed** for the lines themselves. Three things follow from them, and each is a step past the file, so each is **inferred**:

1. `sys.argv` inside Blender holds the tokens that come *before* `--` as well as after — because the code asks whether `"--"` is in `argv` and then takes the index of it, which only makes sense if the earlier tokens are present. The step: I am reading intent from how the list is indexed, not from a statement.
2. A script gets nothing from this route unless `--` is present — the function returns immediately when it is absent.
3. The comment "No help because it conflicts with general Python scripts argument parsing" tells us ordinary `--python` scripts are expected to parse the same `sys.argv`. The step: this is a comment about a conflict, not a statement of what scripts see.

I did not run Blender. Nothing here is a claim about what a render does.

### Part 7 — If the setting carries a template, the script sees the template, not the value (documented)

This matters for `-o` specifically, because the manual says `-o` may carry template text:

> "You can use path templating features such as {blend_name} in the path.
> See Blender's documentation on path templates for more details."

- Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

And what happens to that text:

> "This substitution happens internally at time of use (e.g. while rendering), so
> the template syntax will remain as-is in the path field."

- Source: the manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html (section "Path Templates")
- Date read: 2026-07-30

> "Currently path templates are only supported for the render output path in
> Scene properties and the output paths in the compositor's File Output
> node. More filepaths will support templates in future releases."

- Source: the manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html
- Date read: 2026-07-30

The release notes state the same as a change in 4.5:

> "Some file paths in Blender can now contain "template expressions" like
> `{blend_name}` and `{resolution_x}`, which get replaced by the values they
> reference when the path is used."

- Source: the release notes, https://developer.blender.org/docs/release_notes/4.5/core/ (section "Path Templates")
- Date read: 2026-07-30

There is a documented way for a script to ask for the resolved path rather than the stored text:

> "frame_path(*, frame=-2147483648, preview=False, view='')
>
> Return the absolute path to the filename to be written for a given frame"

- Source: the API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Date read: 2026-07-30

And the 4.5 release notes record that this call can now fail:

> "`scene.render.frame_path()` used to be infallible but can now fail if there
> are errors processing [Path
> Templates](https://docs.blender.org/manual/en/4.5/files/file_paths.html#path-templates)
> in the render output path."

- Source: the release notes, https://developer.blender.org/docs/release_notes/4.5/python_api/ (section "Render Paths", under "Breaking Changes")
- Date read: 2026-07-30

So: **inferred** — a script that reads the stored output path sees whatever text was given, template braces and `#` marks and all, unexpanded; substitution is stated to happen at time of use. The step I took is that "the path field" is what a script reads; the manual is talking about the field in Scene properties, and I am treating the script's view of it as the same value.

### Related and documented, kept short because it is next door rather than in the question

One documented example places a switch *before* the blend-file on purpose and expects it to survive the file load:

> "blender --background --enable-autoexec my_movie.blend --render-anim"
>
> "These command-line arguments can be used to start a regular Blender instance and
> will still override the Preferences."

- Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html (section "Command Line")
- Date read: 2026-07-30

I mention it only because it shows the ordering rule cuts differently for different kinds of setting: a scene setting like the output path is overwritten by the file load, while a switch that overrides the preferences is documented as still holding after it. The sources do not spell out the general rule that separates the two, and I am not inventing one.

### NOT COVERED

This answer does not settle:

- **Which Python value `-o` writes into.** Not stated anywhere in the three documentation sources. Everything in Part 3 is about the *setting being applied at its place in the order*, not about a named property a script may read.
- **Whether a script placed before the blend-file can read the `-o` value at all** — the sources do not describe what the render settings hold before a file is loaded.
- **Whether a script that changes a setting is itself overwritten by a later argument.** The ordering rule implies a later argument runs later, but the sources give no example of a script and a setting in that direction.
- **The other write-related switches individually** — `-F`, `-x`, `-E`, `-s`, `-e`, `-j`, `-t`, `-S`. The ordering rule is stated generally and covers them, but nothing states what a script sees of each.
- **Exactly when a `-P` script runs relative to add-on registration.** Cycles' own parsing happens inside its `init`, but the sources do not order that against `-P`.
- **What a script sees of `-o` when the same launch also renders.** The question asks only what the script sees.
- **Whether `sys.argv` inside Blender is the literal, complete command line.** Part 6 point 1 is inferred from indexing behaviour in Cycles' code, not from a statement.
- **Anything about the `--` route when `--` is absent** beyond "the Cycles parser returns".

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


### Outside the interface

**Brought up to date by the hand tests of 2026-08-26.** This is no longer only what the sources say. Everything above was worked out for a Blender driven from outside its own window, and every test was run that way — but section 15 of the observed results ran the same launch **with a real window open**, on both machines, in all four versions. The window made no difference to what a launch-time script sees: the same path came back from the script, and the path field at the top of the Output panel in Blender's own window showed that same path in every one of the eight pictures. So this holds with a window and without one, on macOS and on Linux, by test rather than by construction.

**What the sources say, unchanged.**

This answer is *about* the outside-the-interface case, and holds there. Everything cited — the ordering rule, the blend-file overwrite, `-P`, `--`, `-c` — exists only because Blender was started and controlled by something other than a person clicking. The manual's own examples are all background-mode command lines, e.g. "blender --background test.blend --render-output /tmp --render-frame 1", and the API documentation says of the `--python` route: "While this works for background tasks it has the downside that opening the file from the file selector won't setup the name-space."

There is one documented place where it does **not** hold: when Blender is used as a Python module (`import bpy`) rather than started as a program.

> "Functionality controlled by command line arguments (shown by calling blender --help aren't accessible)."

> "Typically this isn't such a limitation although there are some command line arguments that don't have
> equivalents in Blender's Python API (--threads and --log for example)."

> "The Python module behaves as if --factory-startup was passed as a command line argument."

- Source: the API documentation, https://docs.blender.org/api/4.5/info_advanced_blender_as_bpy.html
- Date read: 2026-07-30

In that mode there is no launch command line for a script to see, so the whole of this answer is inapplicable. Label: **documented**.

Inside Blender's own window there is no command line at all, so there is nothing for the answer to differ from; the sources describe no way to hand a setting to a running interactive Blender in this manner.

### Operating system

**Brought up to date by the hand tests of 2026-08-26.** This is no longer only what the sources do or do not say. Every result in the observed section above was got **twice** — once on the Creator's Mac (MacBook Pro, Apple M1 Pro, macOS 26.5.2) and once on a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 3060, driver 595.84) — on builds matching to the same commit hash. **The two machines agreed on every one of the sixteen, line for line, in all four versions.** The only lines that differed are two the machine itself supplies rather than Blender: the core count a control run reports (8 on the Mac, 32 on the rented machine, which misreports its own share of a bigger chip), and the graphics device the preferences hold (`METAL` on the Mac; `NONE`, with a device of type `OPTIX` listed, on the Linux machine) — both present in a control run before any command-line argument is given. **Windows is still untested** — no Windows machine has run it — and that is the whole of what is left open here.

**What the sources say, unchanged.**

The sources state nothing that makes the ordering rule, the blend-file overwrite, `--`/`sys.argv`, or the `bpy.app` values depend on the operating system. On that specific point: **not in the sources** — no page says the behaviour differs, and no page says it is the same everywhere either. I searched the command-line pages and the API `bpy.app` page for platform wording and found none attached to these mechanisms.

What the sources *do* mark as platform-dependent, on the same pages, are individual arguments and environment variables, none of which are the mechanism at issue here:

> "-con, --start-console
>
> Start with the console window open (ignored if -b is set), (Windows only)."

> "/?
>
> Print this help text and exit (Windows only)."

> "--debug-ghost
>
> Enable debug messages for Ghost (Linux only)."

> "-r, --register
>
> Register blend-file extension for current user, then exit (Windows & Linux only)."

> "TEMP:
>
> Store temporary files here (MS-Windows)."
>
> "TMPDIR:
>
> Store temporary files here (UNIX Systems).
> The path must reference an existing directory or it will be ignored."

- Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

One further caution the manual gives, which is not an operating-system matter but bites the same people:

> "Arguments are case sensitive! -F and -f are not the same."

- Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html
- Date read: 2026-07-30

Also relevant to a launch on a machine other than the author's: `//` at the start of the path is documented as relative to the blend-file.

> "Use // at the start of the path to render relative to the blend-file."

- Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

---

## Record of where I looked (question 13)

**Files opened and read in full or in substantial part**

- `manual_4.5/advanced/command_line/arguments.txt` — read whole. This is the central page.
- `manual_4.5/advanced/command_line/render.txt` — read whole.
- `manual_4.5/advanced/command_line/index.txt` — read whole (contents listing only; no behaviour stated).
- `manual_4.5/advanced/scripting/security.txt` — read whole.
- `manual_4.5/advanced/scripting/introduction.txt` — read whole. Nothing on command-line ordering; ruled out.
- `manual_4.5/files/file_paths.txt` — read the "Path Templates" section and variable list.
- `manual_4.5/advanced/deploying_blender.txt` — read the `--offline-mode` and `--python-use-system-env` passages.
- `manual_4.5/advanced/app_templates.txt` — read "Command Line Access" and the troubleshooting hint. Nothing about what a script sees; ruled out.
- `manual_4.5/troubleshooting/python.txt` — read the `sys.argv` passage. It is a Windows library-search example, not about launch settings; ruled out.
- `api_4.5/bpy.app.txt` — read whole.
- `api_4.5/bpy.app.handlers.txt` — read the introduction and the `load_pre` / `load_post` / `load_post_fail` entries.
- `api_4.5/bpy.utils.txt` — read the `register_cli_command` entry and the "Custom Commands" example.
- `api_4.5/info_advanced_blender_as_bpy.txt` — read the usage and limitations sections.
- `api_4.5/info_tips_and_tricks.txt` — read "Use Blender without it's User Interface".
- `api_4.5/info_overview.txt` — read the "ways to run scripts" section.
- `api_4.5/bpy.types.RenderSettings.txt` — read the entries for `filepath`, `engine`, `threads`, `threads_mode`, `use_file_extension`, `frame_path`, and the "Only render the active layer" note.
- `release_notes_4.5/core.md` — read the "Path Templates" section.
- `release_notes_4.5/python_api.md` — read the "Breaking Changes" list including "Render Paths".
- `release_notes_4.5/pipeline_assets_io.md` — scanned for path/output/command wording.
- `cycles_4.5/engine.py` — read lines 1–60.
- `cycles_4.5/__init__.py` — read the header and the register/unregister lines to see where `engine.init()` is called.

**Searches run, across `manual_4.5`, `api_4.5` and `release_notes_4.5` unless noted**

- `sys.argv` — three hits only: `api_4.5/bpy.utils.txt`, `manual_4.5/advanced/command_line/arguments.txt`, `manual_4.5/troubleshooting/python.txt`. All three examined.
- `--python` — hits in `info_overview`, `bpy.app`, `info_tips_and_tricks`, `arguments.txt`; plus unrelated `--python-version` pip hits in `python_wheels.txt`, ruled out.
- `command line` and `command-line` — swept across the API documentation (13 hits, all examined) and across the manual excluding the command-line folder (all hits examined; the GPU-troubleshooting and crash pages are about running Blender from a terminal, not about arguments a script can see — ruled out).
- `render-output` and `render_output` — every hit examined. The only substantive ones are in `arguments.txt`; the rest are `_images` filenames on output-property pages and a preferences property `render_output_directory`, both ruled out.
- `order they are given`, `argument order`, `before the blend`, `after the blend`, `overwrites` — hits only in `arguments.txt`, `render.txt`, `bpy.app.txt`, and one unrelated NLA-sidebar hit, ruled out.
- `path template`, `Path Template`, `blend_name` — hits in `arguments.txt`, `files/index.txt`, `files/file_paths.txt`, `bpy.types.FunctionNodeFormatString.txt`, `release_notes_4.5/geometry_nodes.md`, `release_notes_4.5/core.md`. The format-string node and geometry-nodes hits are a different feature, ruled out.
- `argument`, `argv`, `startup script`, `--python` across the release notes — only `--profile-gpu` in `eevee.md` and unrelated wording in `python_api.md` and `modeling.md`. **Nothing in the 4.5 release notes changes or describes argument ordering or what a script sees of a launch setting.**
- `load_post`, `load_pre`, `load_factory` — in the handlers page.
- Across `cycles_4.5`: `argv`, `argparse`, `command_line`, `cycles-device`, `cycles_device`, `print-stats`, `print_stats` — all hits are in `engine.py`, lines 8–41. No other Cycles file touches the command line.

**Ruled out and why**

- The manual's output-property pages (`render/output/properties/*.txt`) — they describe the panels, not the launch. Searched them through the `render_output` sweep; no command-line wording.
- The manual's GPU, crash and startup troubleshooting pages — "run Blender from the command line" there means "so you can see the messages", not "so a script can read a setting".
- The extensions command-line page (`advanced/command_line/extension_arguments.txt`) — package and repository management commands, a different subject.
- Manual images. I checked whether any picture could settle this and concluded none could: every `[image: ...]` on the pages that bear on this question is a screenshot of a properties panel or a preferences checkbox, and a screenshot of a panel cannot show what a script running at launch reads. I opened none, and record that as a deliberate exclusion rather than an oversight.
- Blender 5.0, 5.1 and 5.2 copies of the manual, API and release notes are present in the sources folder. I did not read them: the question is about 4.5.

**What I did not do**

I did not use the web. I did not run Blender. Nothing in this answer rests on my own recollection of how Blender behaves.

---

## What a running Blender could still settle — pruned 2026-08-26

**The list the researcher wrote here has been run, and it is gone from this section rather than left standing beside its own answers.** Every item on it was carried out on 2026-08-26, on both machines — the Creator's Mac (MacBook Pro, Apple M1 Pro, macOS 26.5.2) and a rented Linux machine (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 3060) — in Blender 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.1, and every result is in the observed section above. The full list, with every command written out and every raw line kept, is `Audits/Blender Research/Writing/Hand tests — Architecture questions.md`.

**What is genuinely left.**

- **All of it on Windows.** The researcher's own item 6 asked for the checks to be repeated there. **No Windows machine has run it**, so the Windows answer is untested rather than answered — for whether the ordering rule holds there, whether `-o` lands in the same place, what `sys.argv` holds once a Windows shell has split the line, and whether a path written with backslashes reads back as typed. This is the reason for the silence, written in so it is not read as a result.
- **What the Cycles device override actually does to a render.** The runs above show only that it leaves nothing behind that a script can read; they do not show what the renderer then uses.
- **Whether `-o` reaching the active scene alone depends on which scene is active**, or on how a file names its scenes. One file, with two scenes, was used.

---

2026-08-26 — Claude Opus 5 (Claude Code subagent) — Writing, Blender 4.5.
