# Writing — Architecture questions (Blender 4.5)

**Sources consulted, and date.** Blender 4.5 manual (snapshot in `Blender Sources/manual_4.5`, including one image from its `_images` folder), Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`). All read on **2026-07-30** (the snapshot date given to me).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Question answered here: **13** only.

---

## 13. When Blender is started with a setting given on the command line and with a script to run at the same launch, what does that script see of that setting?

### The question was ambiguous, and I say so plainly

"A setting given on the command line" can mean two different things, and the sources treat them very differently:

- **Reading A (the one I answer as the main reading):** one of Blender's own command-line arguments — the ones Blender itself understands and acts on, such as `-o`, the render output path, whose manual entry is quoted in 13b below.
- **Reading B:** an extra setting of your own, handed to Blender on the same line but not meant for Blender — for example something after the `--` separator, intended for the script to pick up.

I answer both, because both are "a setting given on the command line" and a reader could mean either. Reading B is the one the sources answer most cleanly.

"A script to run at the same launch" I read as a script handed in with `-P` / `--python` (or its siblings `--python-text`, `--python-expr`). I do not read it as an add-on or a startup script, though the Cycles evidence below happens to be an add-on and is the closest thing to a worked example in the sources.

---

### 13a. The rule that governs everything here: order

**LABEL: documented** for the ordering rule itself, **and inferred** for the one step taken from it below, which is marked where it happens.

The fact everything else here leans on is that the command line is a list of instructions carried out one after another, in the order given. The manual states this twice.

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

- Source: manual, "Command Line Arguments", section *Argument Order*.
- Address: https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

And again, more bluntly:

> "Arguments are executed in the order they are given!"

and

> "Always position -f or -a as the last arguments."

- Source: manual, "Rendering From The Command Line".
- Address: https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html
- Date read: 2026-07-30

The script itself is one of those instructions. The manual lists `-P` among the arguments:

> "-P, --python <filepath>
>
> Run the given Python script file."

- Source: manual, "Command Line Arguments", *Python Options*.
- Address: https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

So the quoted words carry this much: the script is one instruction among others, and instructions run in the order given. **That the position of the script on the line therefore decides what it sees is a step beyond the quotes — inferred, not documented.** The step is that a script reads the same state the earlier instructions changed. The quotes below show one case of it happening.

The API documentation confirms that a script can indeed run before the file is loaded, and that things done before the load can be wiped by the load:

> "It is possible to run a script via the --python command line argument, before the blend file."

> "Populating the driver name-space before the blend-file loads also doesn’t work since opening a file clears the name-space."

- Source: Python API documentation, `bpy.app` (driver name-space notes).
- Address: https://docs.blender.org/api/4.5/bpy.app.html
- Date read: 2026-07-30

**Not covered.** These quotes establish the ordering rule and give worked examples for `--render-frame`, `--render-output` and the blend-file. They do **not** contain a worked example of `-o` followed by `-P` where the script then reads the value back. The rule is documented; that particular combination is not spelled out anywhere I found.

---

### 13b. Reading A — a Blender argument such as `-o`, and a script that runs after it

**LABEL: inferred.** (The sources do not say this outright. I set out the step below.)

What the sources say outright is only this, about `-o`:

> "-o, --render-output <path>
>
> Set the render path and file name.
> Use // at the start of the path to render relative to the blend-file."

- Source: manual, "Command Line Arguments", *Render Options*.
- Address: https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

And, from the *Argument Order* quote above:

> "…will not render to /tmp because loading the blend-file overwrites the render output that was set."

**The step I took.** For loading a blend-file to *overwrite* what `-o` set, `-o` and the blend-file's own saved render output must be writing to the same place — the scene's render output setting, the one a file remembers. A script running later in the line reads that same scene setting. Therefore a script placed **after** `-o` (and after the blend-file, so the load does not wipe it) reads the value `-o` supplied, and a script placed **before** `-o` reads whatever was there beforehand — the startup file's value, or the blend-file's, depending on where the file sits.

A reader has to supply that "same place" step, so this is **inferred**, not documented. I want to be exact about how far the quote carries: the quote proves the load overwrites `-o`; it does not, by itself, prove a Python script can read the result.

**Not covered.** I found nothing in the manual or the API documentation that names the Python property `-o` writes into, nothing that says a script can read a command-line-supplied value back, and nothing that says Blender keeps a record of "this came from the command line" for `-o`. If you need certainty rather than an inference, this is the gap. See the hand-check list at the end — it is a two-minute test.

---

### 13c. Reading A, the important sub-case: settings the sources *do* say a script can see

**LABEL: documented.**

For a handful of command-line settings, the API documentation states plainly that a script can see them, because they are exposed as values on `bpy.app`:

> **Citation check, 2026-08-26 (Claude Opus 5).** This sentence used to read "as read-only values on `bpy.app`". Of the four entries quoted below, only `online_access_override` ends "(read-only)"; the other three say nothing either way about being written to. "read-only" has been struck from the sentence. Hand test 14 in the observed section below bears on the same point in the other direction.

> "bpy.app.background
>
> Boolean, True when blender is running without a user interface (started with -b)"

> "bpy.app.factory_startup
>
> Boolean, True when blender is running with –factory-startup)"

> "bpy.app.online_access_override
>
> Boolean, true when internet access preference is overridden by the command line (read-only)"

> "bpy.app.use_event_simulate
>
> Boolean, for application behavior (started with --enable-* matching this attribute name)"

- Source: Python API documentation, `bpy.app`.
- Address: https://docs.blender.org/api/4.5/bpy.app.html
- Date read: 2026-07-30

Read strictly, the quotes carry three different amounts:

- **`-b` and `--factory-startup` are named in the quoted words themselves.** For those two, the sources say outright that a script sees the setting.
- **The third quote names no argument at all.** It says only "overridden by the command line". Which argument does the overriding is not in the quoted words, so no particular argument name can be attached to it from this page.
- **The fourth quote names a shape rather than a list** — "started with --enable-\* matching this attribute name" — and it is written about that one attribute. That other `--enable-*` arguments behave the same way is not stated.

What all four have in common, and this much is documented: where the setting is visible to a script at all, it is visible as a fact about the launch, on `bpy.app`, rather than as a scene value.

**Not covered.** This list is short and specific. There is no equivalent entry for `-o`, and none for the other render and format arguments the manual's argument list holds. The absence of an entry is not proof those cannot be seen — only that the sources do not say so.

---

### 13d. Reading B — a setting handed past `--`, meant for the script

**LABEL: documented.**

This is the case the sources answer cleanly. The manual:

> "--
>
> End option processing, following arguments passed unchanged. Access via Python’s sys.argv."

- Source: manual, "Command Line Arguments", *Misc Options*.
- Address: https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

So a setting placed after `--` is seen by the script **verbatim**, as text, in Python's `sys.argv`. "Passed unchanged" is the operative phrase: Blender does not act on it and does not alter it.

The manual gives the shape of such a line in the Cycles section:

> "Cycles add-on options must be specified following a double dash."

> "blender -b file.blend -f 20 -- --cycles-device OPTIX"

- Source: manual, "Command Line Arguments", *Cycles Render Options*.
- Address: https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

**Not covered.** The manual does not say whether Blender's *own* arguments (the ones before `--`) also appear in `sys.argv`, nor whether they are stripped. It only promises that what follows `--` arrives unchanged. 13e shows a real add-on written on the assumption that `--` is itself in `sys.argv`, but that is the add-on's assumption rather than a source saying so.

---

### 13e. What a real script inside Blender actually does with this — Cycles' own files

**LABEL: observed** (for the lines the files hold and how they are wired) **and inferred** (for everything read off those lines about what happens when Blender runs — four such steps, each marked below).

Cycles ships as Python, so its files show a working example of a script picking up a command-line setting at the same launch.

Exact lines, from `Blender Sources/cycles_4.5/engine.py`, lines 24 to 41:

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

- File and place: `Blender Sources/cycles_4.5/engine.py`, function `_parse_command_line`, lines 24–41.
- Date read: 2026-07-30

The setting it looks for is declared just above, in the same file, lines 16 to 20:

```
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
```

- File and place: `Blender Sources/cycles_4.5/engine.py`, function `_configure_argument_parser`, lines 16–20.
- Date read: 2026-07-30

On the page, the call is wired to registration rather than to rendering. From `Blender Sources/cycles_4.5/engine.py`, lines 44 to 53:

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

with `engine.init()` called from `register()` — `Blender Sources/cycles_4.5/__init__.py`, line 146, inside `def register():` at line 134.

**What is observed here** — that is, what these lines of the file say, and nothing about what happens when they run:

1. Cycles' own add-on code is written to read command-line settings out of `sys.argv`.
2. It takes only what follows `--` — `argv[argv.index("--") + 1:]`.
3. It returns without doing anything when `--` is absent from `sys.argv` — `if "--" not in argv: return`.
4. It calls `parse_known_args`, not `parse_args`.
5. The file carries a comment at line 10 of `engine.py`: `# No help because it conflicts with general Python scripts argument parsing`.

> **Citation check, 2026-08-26 (Claude Opus 5).** The comment was cited as line 9; it is line 10. Corrected against the file.

**Three steps a reader has to supply, so all three are inferred rather than observed.** They are readings of the lines above, not things the lines say.

- **That `--` reaches `sys.argv` at all.** The code searches for it there and gives up when it is missing. That the code is written that way shows what its author expected, not what Blender does. The `if "--" not in argv: return` line is there precisely because the separator may be absent, so the file cannot be read as proof that it is present.
- **What `parse_known_args` does with arguments it does not recognise.** Nothing in the manual, the API documentation or these files says. The file shows only which call is made.
- **What that comment means.** Read plainly it gives a reason for leaving help out. Taking it as a statement that other scripts are expected to be parsing the same `sys.argv` at the same launch is a reading of it, and a reasonable one, but it is a reading.

A separate traceability gap in the same place: the file is said to build its parser with `add_help=False`, and that line is not quoted and carries no line number, so it cannot be checked the way the rest of this section can.

> **Citation check, 2026-08-26 (Claude Opus 5).** The gap is closed. The line is `engine.py` line 12, the second half of `parser = argparse.ArgumentParser(description="Cycles Addon argument parser",` / `                                 add_help=False)` at lines 11–12. It is quoted in full in the second-pass document for this version.

**A fourth step, on the consequence, with the step written out.** `--cycles-device` is applied by calling `_cycles.set_device_override(args.cycles_device)`. It is *not* written into the scene property that holds the device. That property is declared separately, in `Blender Sources/cycles_4.5/properties.py`, lines 378–383:

```
    device: EnumProperty(
        name="Device",
        description="Device to use for rendering",
        items=enum_devices,
        default='CPU',
    )
```

The step: `set_device_override` is a different call from setting `device`, and nothing in these files assigns `args.cycles_device` to that property. So if the call does what its name suggests, a script that later reads the scene's Cycles device setting would still see whatever the file or the default holds, not the value given on the command line. What the call actually does is on the C side, which is not among the files given, so these files cannot show that the setting took effect at all — only that nothing in them writes it where a script would look. This is a warning worth carrying, and it is a warning rather than a finding: **a command-line setting taking effect and a script being able to see it are two different things, and reading these files settles neither.**

**Not covered.** I read the files; I did not run Blender. I cannot show what `sys.argv` contains at run time — not whether Blender's own arguments appear in it alongside `--`, and not whether `--` itself appears, which is the first inferred step above. Nor did I check whether `set_device_override` has any Python-readable counterpart elsewhere; the C side (`_cycles`) is not among the files I was given.

---

### 13f. A third documented route: `-c` / `--command`

**LABEL: documented.**

There is a further way a script sees command-line settings, in which Blender hands them over deliberately.

The manual:

> "-c, --command <command>
>
> Run a command which consumes all remaining arguments.
> Use -c help to list all other commands.
> Pass --help after the command to see its help text.
>
> This implies --background mode."

- Source: manual, "Command Line Arguments", *Misc Options*.
- Address: https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

The API documentation:

> "bpy.utils.register_cli_command(id, execute)
>
> Register a command, accessible via the (-c / --command) command-line argument."

> "execute (callable) – Callback, taking a single list of strings and returns an int.
> The arguments are built from all command-line arguments following the command id."

- Source: Python API documentation, `bpy.utils`.
- Address: https://docs.blender.org/api/4.5/bpy.utils.html
- Date read: 2026-07-30

So under this route the script sees the settings as a plain list of text items, everything after the command name, handed to it directly.

**Not covered.** This requires the script to have registered a command beforehand, which means the script is an add-on or was itself run earlier in the same line. It says nothing about what a plain `-P` script sees.

---

### 13g. Template marks in a write path are left as text in the path field

**LABEL: documented** for what the manual says about the path field, **and inferred** for the step from there to what a script would see of a path given by `-o`. That step is the same one as in 13b, and it is labelled inferred there, so it is labelled inferred here.

`-o` accepts template expressions:

> "You can use path templating features such as {blend_name} in the path.
> See Blender’s documentation on path templates for more details."

- Source: manual, "Command Line Arguments", *Render Options*, under `-o`.
- Address: https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

And those templates are not resolved when the path is set:

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

- Source: manual, "File Paths", section *Path Templates*.
- Address: https://docs.blender.org/manual/en/4.5/files/file_paths.html
- Date read: 2026-07-30

**The inferred step.** If a script does read back a path given on the command line, what it sees is the literal text handed in — `{blend_name}` and the like still standing — not the finished path a file will land at. This only follows if `-o` writes into the same path field the second quote is about, and that is 13b's inference rather than anything a source states. If that inference is wrong, this section says nothing at all about the command line.

**Not covered.** The quote is about the path field generally; it does not name the command line specifically. That the same holds for a path supplied by `-o` follows from `-o` setting that same field, which is itself the inference in 13b.

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


### The two lines this answer must end with

**1. Outside the interface.**

**Brought up to date by the hand tests of 2026-08-26.** This is no longer only what the sources say. Everything above was worked out for a Blender driven from outside its own window, and every test was run that way — but section 15 of the observed results ran the same launch **with a real window open**, on both machines, in all four versions. The window made no difference to what a launch-time script sees: the same path came back from the script, and the path field at the top of the Output panel in Blender's own window showed that same path in every one of the eight pictures. So this holds with a window and without one, on macOS and on Linux, by test rather than by construction.

**What the sources say, unchanged.** This whole answer is about being outside the interface — every fact above comes from pages describing Blender started and driven from a command line, most of them alongside `-b` (background, no interface). So it holds there by construction. The one thing the sources say about the other direction is that these arguments are not confined to background use:

> "These command-line arguments can be used to start a regular Blender instance and will still override the Preferences."

- Source: manual, "Scripting & Security", *Command Line*, about `-y` / `-Y`.
- Address: https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html
- Date read: 2026-07-30

That statement is made about the autoexec arguments only. The sources do not say whether every other command-line setting, or the ordering rule itself, behaves identically when the interface is up. One documented difference in the opposite direction: `-a` as animation player is "ignored if -b is set", and `-con` is likewise "ignored if -b is set" — so a few arguments do mean different things depending on background mode. Neither concerns what a script sees.

**2. Operating system.**

**Brought up to date by the hand tests of 2026-08-26.** This is no longer only what the sources do or do not say. Every result in the observed section above was got **twice** — once on the Creator's Mac (MacBook Pro, Apple M1 Pro, macOS 26.5.2) and once on a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 3060, driver 595.84) — on builds matching to the same commit hash. **The two machines agreed on every one of the sixteen, line for line, in all four versions.** The only lines that differed are two the machine itself supplies rather than Blender: the core count a control run reports (8 on the Mac, 32 on the rented machine, which misreports its own share of a bigger chip), and the graphics device the preferences hold (`METAL` on the Mac; `NONE`, with a device of type `OPTIX` listed, on the Linux machine) — both present in a control run before any command-line argument is given. **Windows is still untested** — no Windows machine has run it — and that is the whole of what is left open here.

**What the sources say, unchanged.** The sources do not say the ordering rule, `--`, `sys.argv`, or `-P` behave differently on any operating system, and I found no page saying so. What the manual does record as operating-system-specific is how you reach the program in the first place — on macOS the executable sits inside the application bundle (`./Blender.app/Contents/MacOS/Blender`), on Windows it is `blender.exe` under Program Files — and a handful of individual arguments that exist on only one system: `-con` and `/?` are marked "Windows only", `--debug-ghost` "Linux only", `-r` / `--register` "Windows & Linux only". None of those bear on what a script sees of a setting.

> **Citation check, 2026-08-26 (Claude Opus 5).** This read "`-r` / `--unregister`". The manual pairs `-r` with `--register`; `--unregister` is a separate entry three lines further down. Both, and `--register-allusers` and `--unregister-allusers`, carry "(Windows & Linux only)". Corrected to `-r` / `--register`.

The manual also records that the temporary-file environment variable differs — `TEMP` on MS-Windows, `TMPDIR` on UNIX systems — which is a reminder that not everything on a launch line is portable, but it is not about scripts.

---

### Where I looked

**Whole-tree searches** (across `manual_4.5`, `api_4.5`, `release_notes_4.5`, and where noted `cycles_4.5`), all on 2026-07-30:

| Search term | Where | Result |
|---|---|---|
| `sys.argv` | manual, API, release notes | Two hits in the manual (the `--` entry; an unrelated Windows library-debugging script in Troubleshooting), one file in the API (`bpy.utils`, inside an example) |
| `argv`, `argparse`, `command` | Cycles files | `engine.py` only — the block quoted above |
| `command line` / `command-line` / `command_line` | manual, API | The command-line pages; `bpy.app`; `bpy.utils`; `bpy.types.RenderSettings`; `bpy.types.PreferencesExperimental`; `info_overview`; `info_tips_and_tricks`; `info_advanced_blender_as_bpy`; `addons` |
| `--python` | manual, API | Only the argument list entries and the `bpy.app` driver notes |
| `in the order they are given` / `argument order` | manual, API, release notes | Exactly two places, both quoted above |
| `overridden by the command line` / `override … command` | manual, API | `bpy.app.online_access_override`; the experimental preference note; the add-ons preference note |
| `render-output` / `render output` | manual, API, release notes | The command-line pages; otherwise only interface pages about the Output panel, none mentioning Python or the command line |
| `path template` | manual, API, release notes | `files/file_paths.txt`; the `-o` entry; `FunctionNodeFormatString`; two release-note files |
| `autoexec` | manual, API | The security page; the argument entries; `bpy.app.autoexec_fail` |
| `os.environ` | manual, API | Only the unrelated Windows troubleshooting script |
| `argument` | release notes | Three hits: `--profile-gpu` in EEVEE notes, and two unrelated uses in Python API and Modeling notes. Nothing about argument order or scripts |

**Pages read in full:** manual `advanced/command_line/arguments.txt`, `advanced/command_line/render.txt`, `advanced/command_line/index.txt`, `advanced/command_line/launch/index.txt`, `launch/linux.txt`, `launch/macos.txt`, `launch/windows.txt`, `files/file_paths.txt` (Path Templates section), `advanced/scripting/security.txt` (Command Line section); API `bpy.app.txt`, `bpy.app.handlers.txt`, `bpy.utils.txt` (`register_cli_command` and the Custom Commands example), `info_overview.txt` (ways to run scripts), `info_tips_and_tricks.txt` (using Blender without its interface); Cycles `engine.py`, `__init__.py` (`register`), `properties.py` (the `device` property and `enum_devices`).

**Image looked at:** `manual_4.5/_images/advanced_command-line_launch_linux_console.png`, reached from the Linux launch page. It shows a terminal window in which `blender` has been typed with no arguments, followed by audio-system warnings about a JACK server. It says nothing about arguments, scripts, or settings. Ruled out as irrelevant. The other three command-line images are, by their captions, a KDE menu configuration dialog, a Windows console window, and a macOS console window; I did not open them, because none of the captions suggests a screenshot of argument handling, and all four sit on pages about how to reach a terminal rather than what arguments do.

**Ruled out and why:**
- The release notes for 4.5 — searched all nineteen subject files for command-line, argument and `argv` terms. The only command-line change recorded is a new `--profile-gpu` argument in the EEVEE notes. Nothing about argument order, `-P`, `-o`, or what scripts can read. So 4.5 records no change in this area.
- `api_4.5/info_advanced_blender_as_bpy.txt` — it concerns Blender used as a Python module, where the manual states "Functionality controlled by command line arguments (shown by calling blender --help aren't accessible)." That is a different way of running Blender from the one the question asks about, so I did not build the answer on it.
- The whole of the interface-side Output documentation (`render/output/...`) — searched, and it describes the panel a person clicks in. It never mentions the command line or Python.
- Extensions command-line arguments (`advanced/command_line/extension_arguments.txt`) — about installing and building extensions, not about running a script at launch.

---

## What a running Blender could still settle — pruned 2026-08-26

**The list the researcher wrote here has been run, and it is gone from this section rather than left standing beside its own answers.** Every item on it was carried out on 2026-08-26, on both machines — the Creator's Mac (MacBook Pro, Apple M1 Pro, macOS 26.5.2) and a rented Linux machine (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 3060) — in Blender 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.1, and every result is in the observed section above. The full list, with every command written out and every raw line kept, is `Audits/Blender Research/Writing/Hand tests — Architecture questions.md`.

**What is genuinely left.**

- **All of it on Windows.** The researcher's own item 7 asked for the checks to be repeated there. **No Windows machine has run it**, so the Windows answer is untested rather than answered — for whether the ordering rule holds there, whether `-o` lands in the same place, what `sys.argv` holds once a Windows shell has split the line, and whether a path written with backslashes reads back as typed. This is the reason for the silence, written in so it is not read as a result.
- **What the Cycles device override actually does to a render.** The runs above show only that it leaves nothing behind that a script can read; they do not show what the renderer then uses.
- **Whether `-o` reaching the active scene alone depends on which scene is active**, or on how a file names its scenes. One file, with two scenes, was used.

---

2026-08-25 — Claude Opus 5 (Claude Code subagent) — Writing, Blender 4.5.
