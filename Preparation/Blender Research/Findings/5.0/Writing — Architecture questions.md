# Writing — Blender 5.0 — question 13

Sources consulted, and the date: Blender 5.0 manual (`Blender Sources/manual_5.0`), Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and the Cycles add-on files (`Blender Sources/cycles_5.0`). All read on 2026-07-30 (the snapshot date given to me).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 13 is answered here. I read the whole sheet first, and answered nothing else on it.

---

## 13. When Blender is started with a setting given on the command line and with a script to run at the same launch, what does that script see of that setting?

**How I read the question.** "A setting given on the command line" means one of the arguments that changes a Blender setting at launch — for example `-o` (the render output path), `-F` (the format), `-x` (the extension switch), `-s` / `-e` (start and end frame), `-E` (engine), `-t` (threads), or the Cycles add-on's `--cycles-device`. "A script to run at the same launch" means a script handed in on the same command line, by `-P` / `--python`, `--python-text`, or `--python-expr`. The question is what such a script finds when it looks for that setting.

There is a second possible reading — "what does the script see" meaning only "can the script read the raw text of the argument" — and I answer that part too, at point 4 below, because the sources cover it separately.

**LABEL: inferred for the answer to the question as asked.** Nothing in the sources says what a script handed in at launch sees of a setting given at the same launch. What is documented is the general rule that arguments are carried out in the order they are given (point 1), that loading the blend-file overwrites a render output path set before it (point 2), what `-o` does and what the scene's render path property holds, separately (point 3), that what follows a bare `--` reaches a script as raw text (point 4), and that a few settings arrive as their own flags a script can read (point 5). Applying any of that to a script is a step, and every such step below is marked **inferred** with the step written out. The Cycles device case (point 6) is **observed** for what the files say and **inferred** for what follows from it.

### 1. The rule the sources state: arguments are carried out in the order they are given

The command line is not a set of settings collected up and applied before anything runs. Each argument is carried out where it stands, left to right.

That much is documented. **That `-P` is carried out where it stands too, and that what the script sees therefore turns on where the setting sits relative to it, is a step I am taking** — it is written out as an inference below, after the quotes. It is also not the whole of the answer: point 2 gives a case where the position on the line does not settle it, and point 6 gives a setting that does not follow the rule at all.

The manual states the rule twice, in two different pages.

> "Arguments are executed in the order they are given. eg:"

— Blender manual, Command Line Arguments, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (section "Argument Order"). Read 2026-07-30.

> "Arguments are executed in the order they are given!"

— Blender manual, Rendering From The Command Line, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html. Read 2026-07-30.

The same page gives the worked consequence for the render output path specifically:

> "blender --background test.blend --render-frame 1 --render-output "/tmp"
>
> …will not render to /tmp because --render-frame 1 renders before the output path is set."

— Blender manual, Command Line Arguments, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (section "Argument Order"). Read 2026-07-30.

Applying that rule to a script instead of a render is a step I am taking, not a sentence the manual writes. **This next claim is inferred, and here is the step:** the manual says every argument is carried out in the order given, and `-P` is an argument; therefore a setting written to the left of `-P` has already been applied when the script runs, and a setting written to the right of `-P` has not yet been applied when the script runs. So:

- `blender -b file.blend -o /out/ -P script.py -f 1` — the script runs after the output path is set, and sees the new path.
- `blender -b file.blend -P script.py -o /out/ -f 1` — the script runs before the output path is set, and sees whatever the blend-file held, not `/out/`.

I did not find any sentence in the sources that states this for scripts outright. The rule is stated in general terms and I applied it.

### 2. Loading the blend-file overwrites a render output path set before it

There is a second trap on the same page, and it matters for a script too:

> "blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set."

— Blender manual, Command Line Arguments, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (section "Argument Order"). Read 2026-07-30.

**Inferred, with the step written out:** if loading the blend-file overwrites a render output path that was set before it, then a script placed after the blend-file will read the blend-file's own path, even though `-o` appeared earlier on the same line. The setting was applied and then destroyed before the script ever ran. The manual says this about the render happening, not about a script reading; extending it to a script is my step.

The API documentation states the same shape of thing for a different piece of state, and it is the one place in the sources that talks about a `--python` script and file loading together:

> "It is possible to run a script via the --python command line argument, before the blend file. This can register a load-post handler (bpy.app.handlers.load_post) that initialized the name-space."

and, just above it:

> "Populating the driver name-space before the blend-file loads also doesn't work since opening a file clears the name-space."

— Blender Python API, Application Data (bpy.app), https://docs.blender.org/api/5.0/bpy.app.html (section "File Loading & Order of Initialization"). Read 2026-07-30.

Those words are about one particular piece of state — the name-space that drivers are evaluated in — and that is all they document. **Inferred, with the step written out:** if a script placed before the blend-file has to use a `load_post` handler to get that piece of state to survive the file load, the same shape holds for anything else a script sets before the file loads. The sources do not say so; the widening is mine. `bpy.app.handlers.load_post` is documented as "on loading a new blend file (after). Accepts one argument: the file being loaded, an empty string for the startup-file." — Blender Python API, Application Handlers (bpy.app.handlers), https://docs.blender.org/api/5.0/bpy.app.handlers.html. Read 2026-07-30.

The API documentation also shows the ordinary ordering by example, file first so the script has something to look at:

> "You might want to run this with a blend-file so the script has some data to operate on.
>
> blender myscene.blend --background --python myscript.py"

— Blender Python API, Tips and Tricks, https://docs.blender.org/api/5.0/info_tips_and_tricks.html. Read 2026-07-30.

### 3. Where a setting like `-o` lands, and how sure the sources are about that

The manual describes `-o` only as an action, not as a named property:

> "-o, --render-output <path>
>
> Set the render path and file name.
> Use // at the start of the path to render relative to the blend-file."

— Blender manual, Command Line Arguments, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (section "Render Options"). Read 2026-07-30.

The API documentation describes a matching scene setting:

> "Directory/name to save animations, # characters define the position and padding of frame numbers"

— Blender Python API, `bpy.types.RenderSettings.filepath`, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html. Read 2026-07-30. Its type line reads: "string, default "", (never None, blend relative // prefix supported, Supports template expressions)".

**Inferred, with the step written out:** the manual's `-o` and the API's `RenderSettings.filepath` describe the same thing — a render path with `#` frame-number padding and a `//` blend-relative prefix. So a script that runs after `-o` and reads the scene's render `filepath` is the natural way to see the setting. But no sentence in the sources joins the two. Nothing I found says `-o` writes into that property, and nothing says which scene it writes into when a file holds several. If your reading of this question needs certainty on that, treat it as unproven here.

### 4. What follows a bare `--` reaches a script as raw text

Separately from whether the setting has been *applied*, a script can be handed the raw words — but what is documented here covers only what is written after a bare `--`, not the rest of the line. The manual:

> "--
>
> End option processing, following arguments passed unchanged. Access via Python's sys.argv."

— Blender manual, Command Line Arguments, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (section "Misc Options"). Read 2026-07-30.

There is also a documented way to have a script take arguments deliberately:

> "Register a command, accessible via the (-c / --command) command-line argument."
>
> "The arguments are built from all command-line arguments following the command id."

— Blender Python API, `bpy.utils.register_cli_command`, https://docs.blender.org/api/5.0/bpy.utils.html. Read 2026-07-30. The manual's entry for that argument reads: "-c, --command <command> — Run a command which consumes all remaining arguments. Use -c help to list all other commands." and "This implies --background mode." — same Command Line Arguments page, "Misc Options". Read 2026-07-30.

Whether `sys.argv` also contains Blender's *own* arguments (the `-o`, the `-b`, the blend-file) is not stated anywhere I found. Two things in the sources point that way but do not say it. The Cycles add-on reads `sys.argv` and looks in it for the `--` marker (see point 6), and a troubleshooting page treats `sys.argv[0]` as the location of the Blender program:

> "os.path.dirname(sys.argv[0]),"

— Blender manual, Python Troubleshooting, https://docs.blender.org/manual/en/5.0/troubleshooting/python.html (in a Windows library-search-path example). Read 2026-07-30. **Inferred:** `sys.argv[0]` being Blender's own program path means `sys.argv` is Blender's real command line rather than a trimmed list. That is my step from one code example, and it does not establish that the middle of the line survives.

### 5. Some command-line settings arrive as their own flags a script can check

For a handful of settings the API documentation states plainly that a script can see them, and names the attribute. They are not part of any scene. Only one of the entries quoted below says in so many words that it is read-only; that the others are too is not carried by the words quoted.

> "Boolean, True when blender is running without a user interface (started with -b)"

— `bpy.app.background`, Blender Python API, https://docs.blender.org/api/5.0/bpy.app.html. Read 2026-07-30.

> "Boolean, for debug info (started with --debug / --debug-* matching this attribute name)."

— the `bpy.app.debug`, `bpy.app.debug_python`, `bpy.app.debug_io`, `bpy.app.debug_depsgraph*` and similar attributes, same page. Read 2026-07-30.

> "Boolean, for application behavior (started with --enable-* matching this attribute name)"

— `bpy.app.use_event_simulate` and `bpy.app.use_userpref_skip_save_on_exit`, same page. Read 2026-07-30.

> "Boolean, true when internet access preference is overridden by the command line (read-only)."

— `bpy.app.online_access_override`, same page. Read 2026-07-30.

So for these, the script does not have to read a scene property or parse text; it reads a flag that exists precisely to report what the command line said. The ordering rule from point 1 still applies as far as the sources go — none of these entries says the flag is set before scripts run.

### 6. The Cycles device: the files show the value going somewhere no property records

Cycles' own files show a command-line setting whose value, in the lines read, is handed straight to the render engine and written into no property. What follows is what the files say; the conclusion drawn from it is marked separately below.

Observed, in `Blender Sources/cycles_5.0/engine.py`, function `_parse_command_line()` (lines 24–41 of the file):

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

And in the same file, `init()` (lines 44–53) ends with:

```
    _cycles.init(path, user_path, bpy.app.background)
    _parse_command_line()
```

The argument's own description, in `_configure_argument_parser()` in the same file (lines 16–20):

```
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
```

The scene's own device setting is a separate thing, defined in `Blender Sources/cycles_5.0/properties.py`, class `CyclesRenderSettings` (lines 381–387):

```
    device: EnumProperty(
        name="Device",
        description="Device to use for rendering",
        items=enum_devices,
        default='CPU',
        update=update_render_passes,
    )
```

with `enum_devices` (lines 30–34 of the same file) holding only `'CPU'` and `'GPU'`. **That last part is described rather than quoted** — the block itself is not reproduced here, so it is the one thing in point 6 a reader cannot check against the words on the page.

> **Citation check, 2026-08-26 (Claude Opus 5).** Three line ranges in this section were off by one against the files, and are corrected here: `init()` is lines 44–53 (line 43 is blank), the `--cycles-device` block is lines 16–20 (line 15 is the tail of the previous argument), and the `device` property block is lines 381–387 (line 388 begins the next property). The quoted words are exact and were found where this now says.

**Observed:** in the lines quoted above, the `--cycles-device` value is handed to `_cycles.set_device_override(...)` and to nothing else. **Not established:** that no property anywhere is written from it. The record of looking below covers the first 80 lines of `engine.py` and the lines of `properties.py` that name `device`; it does not cover the rest of either file, or the add-on's other files.

**Inferred, with the step written out:** since the value is passed into the `_cycles` module and no property is assigned in the lines read, a script that reads the scene's Cycles `device` after `--cycles-device` was given will still read whatever the blend-file or the default held — `'CPU'` or `'GPU'`, never `'OPTIX'` or `'METAL'`, which are not among the two allowed values. The one trace of the setting these files show reaching a script is the raw text after the `--` marker; whether the `_cycles` module offers any other way to read it back is not shown by what I read. I did not run Blender to confirm any of this; it follows from the code as written.

Two more things these files show, which bear on the question:

- **Observed:** the Cycles add-on gets at the command line by reading `sys.argv` itself and slicing after the `--` marker. **Documented:** the placement is required — "Cycles add-on options must be specified following a double dash." with the example "blender -b file.blend -f 20 -- --cycles-device OPTIX" — Blender manual, Command Line Arguments, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (section "Cycles Render Options"). Read 2026-07-30. **Inferred, with the step written out:** the code slices after `--`, so a value written before that marker would not be in the slice it reads; that is why the manual's requirement and the code's reading fit together. The manual states the requirement and gives no reason for it, so the joining is mine.
- **Observed:** in the lines quoted above, `_parse_command_line()` is called from the add-on's `init()`, not from anywhere tied to the position of the argument on the command line. **Inferred, with the step written out:** a value read inside `init()` can only take effect whenever `init()` runs, so for this one setting the "position on the line" reasoning of point 1 does not describe when it takes effect. The files show where the call sits; that it therefore breaks the ordering reasoning is my step. When `init()` runs relative to a `-P` script is not shown by these files at all.

### NOT COVERED

- Nothing in the sources states outright, in one sentence, what a `-P` script sees of an earlier or later command-line setting. Points 1 and 2 are the general ordering rule applied by me. If you need that stated by Blender rather than derived, it is not there.
- Nothing joins `-o` to a named Python property. See point 3.
- **The same hole is wider than `-o`.** My reading of the question named `-F` (the format), `-x` (the extension switch), `-s` / `-e` (start and end frame), `-E` (engine) and `-t` (threads) as settings the question covers. I found nothing joining any of those to a named Python property either, and I did not work any of them through the way I worked `-o` through at point 3. Only the general ordering rule of point 1 speaks to them, and that rule is itself applied by me rather than stated for scripts.
- Nothing says whether Blender's own arguments (as opposed to what follows `--`) appear in `sys.argv`. See point 4.
- Nothing says when `-P` runs relative to add-on registration, so the timing of the Cycles case against a script is unsettled.
- Nothing says what happens with two scripts, or a script plus `--python-expr`, beyond the general ordering rule.
- The default `-Y, --disable-autoexec` — "Disable automatic Python script execution (Python-drivers & startup scripts), (default)." (same Command Line Arguments page, "Python Options", read 2026-07-30) — names drivers and startup scripts. Whether it touches a script handed in explicitly with `-P` is not stated. I did not assume either way.
- I did not test anything. Every "observed" line above is observed from reading the Cycles files, not from running Blender.

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

This answer is *only* about outside the interface. Every source quoted describes launching Blender from a command line, which is Blender being started from outside its own window whoever types the words. The manual's own framing: "For automation and batch processing which require launching Blender with different arguments." — Blender manual, Using Blender From The Command Line, https://docs.blender.org/manual/en/5.0/advanced/command_line/index.html. Read 2026-07-30.

Where it differs from inside: **inferred, with the step written out** — every source quoted here describes arguments given when Blender is started, and none of them describes handing an argument to a Blender that is already open; so on what is quoted, the ordering rule has no counterpart inside a running window. The sources do not state that.

**Inferred, with the step written out**, on whether the answer holds when Blender is launched with its window rather than with `-b`: the sources do not say the rule changes, and `-b` reads as one entry in the same argument list rather than as a different way of launching. So I take the rule to hold either way. No sentence in the sources says so.

One case does differ, and it is documented. When Blender is used as a Python module rather than started as a program, the command line is gone entirely:

> "Functionality controlled by command line arguments (shown by calling blender --help aren't accessible)."

and

> "The Python module behaves as if --factory-startup was passed as a command line argument."

— Blender Python API, Blender as a Python Module, https://docs.blender.org/api/5.0/info_advanced_blender_as_bpy.html. Read 2026-07-30. So in that mode the question does not arise: there is no setting given on the command line for a script to see.

### Operating system

**Brought up to date by the hand tests of 2026-08-26.** This is no longer only what the sources do or do not say. Every result in the observed section above was got **twice** — once on the Creator's Mac (MacBook Pro, Apple M1 Pro, macOS 26.5.2) and once on a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 3060, driver 595.84) — on builds matching to the same commit hash. **The two machines agreed on every one of the sixteen, line for line, in all four versions.** The only lines that differed are two the machine itself supplies rather than Blender: the core count a control run reports (8 on the Mac, 32 on the rented machine, which misreports its own share of a bigger chip), and the graphics device the preferences hold (`METAL` on the Mac; `NONE`, with a device of type `OPTIX` listed, on the Linux machine) — both present in a control run before any command-line argument is given. **Windows is still untested** — no Windows machine has run it — and that is the whole of what is left open here.

**What the sources say, unchanged.**

The sources do not say the ordering rule differs by operating system. It is stated flatly on both pages, with no platform note attached, and the Cycles parsing code has no platform branch in it.

The same Command Line Arguments page does mark other arguments as platform-bound where they are — for example "-con, --start-console — Start with the console window open (ignored if -b is set), (Windows only)." and "--debug-ghost — Enable debug messages for Ghost (Linux only)." and "--qos <level> — Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only)." (read 2026-07-30). None of the arguments in this answer — `-o`, `-P`, `--`, `-b`, `-c`, `--cycles-device` — carries such a note.

**Inferred, with the step written out:** the page marks platform-specific arguments explicitly, and does not mark these; so these are meant to work on all platforms. That is my reading of an absence, not a statement.

The one platform difference I did find is in how the command is written, not in what it does: the manual has three separate pages for launching, Linux, macOS and Windows (https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/index.html, read 2026-07-30). And the shell quoting around a path is the operating system's business, not Blender's; the sources say nothing about it.

---

## Record of where I looked, for question 13

**Files I searched.** All of `Blender Sources/manual_5.0`, all of `Blender Sources/api_5.0` (1,862 files), all of `Blender Sources/release_notes_5.0`, and all of `Blender Sources/cycles_5.0`, by whole-tree text search before opening anything.

**Terms I searched for.** `order they are given`, `executed in the order`, `Argument Order`, `sys.argv`, `argv`, `render-output`, `--python`, `-P `, `command line`, `command-line`, `before the blend file`, `overwrites the render output`, `load_post`, `Order of Initialization`, `run before`, `after the blend`, `startup script`, `at startup`, `argument`, `path template`, `set_device_override`, `device_override`, `command`, `--` (in `bpy.app`).

**Pages I opened in full or in part.**
- `advanced/command_line/arguments.txt` — read in full, 662 lines. This is where the answer mostly is.
- `advanced/command_line/render.txt` — read in full.
- `advanced/command_line/index.txt` — read in full.
- `advanced/scripting/security.txt` — read the first 60 lines, for whether auto-execution rules touch `-P`. They are about scripts inside blend-files, not scripts on the command line. Ruled out.
- `troubleshooting/python.txt` — read the `sys.argv` example. Only tangential; used at point 4.
- `api_5.0/bpy.app.txt` — read the top 150 lines and the "File Loading & Order of Initialization" section and the `--enable-*` / `online_access_override` / `background` entries.
- `api_5.0/bpy.app.handlers.txt` — read the `load_pre` / `load_post` entries.
- `api_5.0/bpy.utils.txt` — read the `register_cli_command` entry and the custom-commands example.
- `api_5.0/info_tips_and_tricks.txt` — read the background-mode section.
- `api_5.0/info_overview.txt` — read the "ways to run scripts" section.
- `api_5.0/info_advanced_blender_as_bpy.txt` — read the Usage and Command Line Arguments Unsupported sections.
- `api_5.0/bpy.types.RenderSettings.txt` — read the `filepath` entry.
- `cycles_5.0/engine.py` — read the first 80 lines.
- `cycles_5.0/properties.py` — read the `enum_devices` block and the `device` property, plus a grep of every line containing `device`.
- `release_notes_5.0/core.md` — read the command-line section (replaced flags, path template variables, `--gpu-vsync`).

**What I ruled out and why.**
- The release notes for 5.0 say nothing about argument ordering or about what a launch-time script sees. The only command-line items there are three replaced debug flags (`--debug-cycles` → `--log cycles`, `--debug-ffmpeg` → `--log video`, `--verbose` → `--log-level`), six new path template variables, and the new `--gpu-vsync` argument. None touches this question. Searched `release_notes_5.0` for `command line`, `command-line`, `argument`, `--python`, `render-output`, `path template`; the only other hits were an unrelated compositor note and a crash fix entry.
- `advanced/command_line/extension_arguments.txt` covers package and repository management commands; nothing about scripts seeing settings. Ruled out by its search hits.
- No picture is involved. The command-line pages carry no `[image: ...]` markers, so there was nothing in `_images` to open for this question.
- I did not use the web, and did not need to.

---

## What a running Blender could still settle — pruned 2026-08-26

**The list the researcher wrote here has been run, and it is gone from this section rather than left standing beside its own answers.** Every item on it was carried out on 2026-08-26, on both machines — the Creator's Mac (MacBook Pro, Apple M1 Pro, macOS 26.5.2) and a rented Linux machine (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 3060) — in Blender 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.1, and every result is in the observed section above. The full list, with every command written out and every raw line kept, is `Audits/Blender Research/Writing/Hand tests — Architecture questions.md`.

**What is genuinely left.**

- **All of it on Windows.** The researcher's own items 7 and 8 asked for the checks to be repeated there. **No Windows machine has run it**, so the Windows answer is untested rather than answered — for whether the ordering rule holds there, whether `-o` lands in the same place, what `sys.argv` holds once a Windows shell has split the line, and whether a path written with backslashes reads back as typed. This is the reason for the silence, written in so it is not read as a result.
- **What the Cycles device override actually does to a render.** The runs above show only that it leaves nothing behind that a script can read; they do not show what the renderer then uses.
- **Whether `-o` reaching the active scene alone depends on which scene is active**, or on how a file names its scenes. One file, with two scenes, was used.

---

2026-08-25 — Claude Opus 5 (Claude Code subagent) — Writing, Blender 5.0.
