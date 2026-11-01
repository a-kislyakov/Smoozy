# Writing — Architecture questions, second pass (Blender 5.2)

**Sources consulted, and date.** Blender 5.2 manual (`Blender Sources/manual_5.2`), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and the Cycles add-on's own files (`Blender Sources/cycles_5.2`). All four were read on **2026-07-30** (the snapshot date given to me), and the searching and reading described below was done on 2026-08-26.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

This pass answers **question 13 only**. The rest of the sheet was read so I knew what question 13 was not asking, and nothing else on it was answered.

---

## 13. When Blender is started with a setting given on the command line and with a script to run at the same launch, what does that script see of that setting?

### The question was ambiguous, and here is the reading I answered

"A setting given on the command line" is not narrowed by the question. I answered the reading I think is meant: **one of Blender's own setting arguments** — the render output path (`-o` / `--render-output`) above all, and by the same rule the format (`-F`), the engine (`-E`), the frame range (`-s` / `-e`), the extension switch (`-x`) and so on — handed in at the same launch as a script given with `-P` / `--python`.

The other reading, which I also cover in part C below because the sources answer it cleanly, is that "a setting" means **an extra argument the operator passes for the script's own use**, placed after `--`. That one has a plain documented answer; the first one only partly does.

### Short answer

Four separate things, and they do not agree with each other, so they have to be kept apart:

- **A. What the script sees depends entirely on where the script sits in the command line.** Arguments are carried out one after another in the order typed. **documented** for the rule; **inferred** for applying it to `-P`.
- **B. Loading the blend-file undoes a render output path set before it.** **documented**, for `--render-output` specifically.
- **C. The raw text of the command line is visible to the script through Python's `sys.argv`, whatever the order.** **documented** for the arguments after `--`; **inferred** (with an observed step from Cycles' own file) that `sys.argv` holds the whole command line, Blender's own arguments included.
- **D. A short list of launch settings can be read back as read-only values on `bpy.app`. The render output path is not one of them, and no source says which Python value, if any, `-o` writes into.** **documented** for the `bpy.app` list; **not in the sources** for `-o`.

---

### A. Order decides it

**Label: documented** (for the ordering rule) / **inferred** (for what it means for a `-P` script)

Exact words, from the manual's command-line arguments page, section "Argument Order":

> "Arguments are executed in the order they are given. eg:"
>
> "blender --background test.blend --render-frame 1 --render-output "/tmp""
>
> "…will not render to /tmp because --render-frame 1 renders before the output path is set."
>
> "blender --background --render-output /tmp test.blend --render-frame 1"
>
> "…will not render to /tmp because loading the blend-file overwrites the render output that was set."
>
> "blender --background test.blend --render-output /tmp --render-frame 1"
>
> "…works as expected."

Source: manual. Address: https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — date read 2026-07-30.

The same rule is stated a second time, on the manual's rendering-from-the-command-line page:

> "Arguments are executed in the order they are given!"
>
> "The following command will not work, since the output and extension are set after Blender is told to render:"
>
> "blender -b file.blend -a -x 1 -o //render"
>
> "The following command will behave as expected:"
>
> "blender -b file.blend -x 1 -o //render -a"
>
> "Always position -f or -a as the last arguments."

Source: manual. Address: https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — date read 2026-07-30.

The script argument is listed on the same page as one of the arguments, under "Python Options":

> "-P, --python <filepath>"
>
> "Run the given Python script file."

Source: manual. Address: https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — date read 2026-07-30.

**The step I took (this is why the label is inferred, not documented):** the manual says arguments are carried out in the order given, and it lists `-P` as an argument. From that I conclude that a script given with `-P` runs at its own place in the queue — so a setting written *before* `-P` has already been applied when the script starts, and a setting written *after* `-P` has not been applied yet, and the script sees whatever was there before. **No source I found says this about `-P` in so many words.** The manual's own worked examples are all about a render command (`-f`, `-a`) sitting in the wrong place; it never shows a script in one of those examples.

The API documentation does confirm, separately, that a `-P` script can run before the blend-file is even loaded:

> "It is possible to run a script via the --python command line argument, before the blend file."

and, in the same passage:

> "Populating the driver name-space before the blend-file loads also doesn’t work since opening a file clears the name-space."

Source: Python API documentation. Address: https://docs.blender.org/api/5.2/bpy.app.html — date read 2026-07-30.

### B. The blend-file load wipes a render output path set before it

**Label: documented**

The middle example quoted in part A is the load-bearing one:

> "blender --background --render-output /tmp test.blend --render-frame 1"
>
> "…will not render to /tmp because loading the blend-file overwrites the render output that was set."

Source: manual. Address: https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — date read 2026-07-30.

So a script placed *before* the blend-file cannot usefully be shown a `-o` value: even if `-o` came before the script, the file load that follows replaces it. The order that leaves the setting standing when the script runs is: file, then setting, then `-P`.

Note the exact scope of the quote. It says the blend-file load overwrites *the render output*. It does not say what else a file load overwrites, and I did not find a source that lists that.

### C. The raw command line is visible to the script as text

**Label: documented** (for arguments after `--`) / **inferred** (that Blender's own arguments are in the same list)

Exact words, from the manual's command-line arguments page, "Misc Options":

> "--"
>
> "End option processing, following arguments passed unchanged. Access via Python’s sys.argv."

Source: manual. Address: https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — date read 2026-07-30.

That is the documented route by which a setting handed in at launch reaches a script directly: put it after `--`, and the script reads it as plain text out of `sys.argv`. It is untouched by Blender ("passed unchanged"), so what the script sees is exactly what was typed. Blender's own arguments have to come before the `--`, since `--` ends Blender's option processing.

**Observed, from a file I was given.** Cycles is an add-on, and its own file shows add-on code doing exactly this — and shows that the list it reads contains the `--` marker itself:

> ```
> def _parse_command_line():
>     import sys
>
>     argv = sys.argv
>     if "--" not in argv:
>         return
>
>     parser = _configure_argument_parser()
>     args, _ = parser.parse_known_args(argv[argv.index("--") + 1:])
> ```

File: `Blender Sources/cycles_5.2/engine.py`, function `_parse_command_line`, lines 24–32. Date read 2026-07-30.

**The step I took:** the code takes the whole of `sys.argv`, looks for the item `"--"` in it, and then keeps only what comes *after* that item. If `sys.argv` had already been trimmed to just the arguments following `--`, there would be no `"--"` in it to find and the function would return without doing anything. So `sys.argv`, as a script inside Blender sees it, holds the command line including the `--` marker and what precedes it — that is, Blender's own arguments too. That conclusion is **inferred** from the file, not stated in it.

The manual carries one more piece of support for the same thing. Its example of serving an asset library sets `sys.argv` aside before handing control to a standard Python module, which is only necessary if `sys.argv` is holding Blender's own command line:

> "Blender: blender -q -b --python-expr "import runpy, sys; sys.argv=[sys.argv[0], '-b', '127.0.0.1']; runpy.run_module('http.server', run_name='__main__')""

Source: manual. Address: https://docs.blender.org/manual/en/5.2/files/asset_libraries/remote_asset_libraries.html — date read 2026-07-30. Reading a purpose into that line is my inference; the manual gives it as a command to run and does not explain the `sys.argv` assignment.

Cycles' own settings are handed over by this same route, and the manual says so:

> "Cycles add-on options must be specified following a double dash."
>
> "blender -b file.blend -f 20 -- --cycles-device OPTIX"

Source: manual. Address: https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — date read 2026-07-30.

There is a second, separate mechanism, for completeness: a registered command takes the rest of the command line as a list.

> "Register a command, accessible via the (-c / --command) command-line argument."
>
> "execute (Callable[[list[str]], int]) – Callback, taking a single list of strings and returns an int. The arguments are built from all command-line arguments following the command id."

Source: Python API documentation. Address: https://docs.blender.org/api/5.2/bpy.utils.html — date read 2026-07-30. The matching manual entry says: "-c, --command <command>" / "Run a command which consumes all remaining arguments." (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html). This is not a `-P` script, so it is a different case from the one the question asks about; I note it because it is the other documented way a launch-time setting is put in front of Python code.

### D. What a script can read back as a value, and what it cannot

**Label: documented** (for the `bpy.app` values) / **not in the sources** (for the render output path)

A small set of launch-time settings are exposed to a script as values it can read. The exact words:

> **Citation check, 2026-08-26 (Claude Opus 5).** This sentence used to say "as read-only values". Of the five entries quoted below, only `online_access_override` ends "(read-only)"; the other four say nothing either way about being written to. Hand test 14 in the observed section below bears on the same point in the other direction.

> "bpy.app.background"
>
> "Boolean, True when blender is running without a user interface (started with -b)"

> "bpy.app.factory_startup"
>
> "Boolean, True when blender is running with –factory-startup"

> "bpy.app.debug"
>
> "Boolean, for debug info (started with --debug / --debug-* matching this attribute name)."

> "bpy.app.use_event_simulate"
>
> "Boolean, for application behavior (started with --enable-* matching this attribute name)"

> "bpy.app.online_access_override"
>
> "Boolean, true when internet access preference is overridden by the command line (read-only)."

Source: Python API documentation. Address: https://docs.blender.org/api/5.2/bpy.app.html — date read 2026-07-30. The same page also carries `bpy.app.debug_value` ("Integer value which can be set to non-zero values for testing purposes."), `bpy.app.tempdir` ("String, the temp directory used by blender (read-only).") and `bpy.app.cachedir`.

**The render output path is not on that list.** I searched for it and found no statement anywhere in the manual, the API documentation, the release notes or the Cycles files that says which Python value `-o` / `--render-output` writes into, or that a script can read it back at all. The nearest thing is the property description itself, which never mentions the command line:

> "filepath"
>
> "Directory/name to save animations, # characters define the position and padding of frame numbers (default "//", never None, blend relative // prefix supported, Supports template expressions)"

Source: Python API documentation. Address: https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — date read 2026-07-30.

It is reasonable to suppose `-o` writes into that property — the manual says `-o` will "Set the render path and file name", and says loading a blend-file "overwrites the render output that was set", which only makes sense if the setting lives in the file's own data. But that is a chain of supposition, and the question asks what the script *sees*, so I am recording it as **not in the sources** rather than dressing it up as an answer.

The API documentation states outright that some command-line settings have no Python counterpart at all:

> "Functionality controlled by command line arguments (shown by calling blender --help) isn’t accessible."
>
> "Typically this isn’t such a limitation although there are some command line arguments that don’t have equivalents in Blender’s Python API (--threads and --log for example)."

Source: Python API documentation. Address: https://docs.blender.org/api/5.2/info_advanced_blender_as_bpy.html — date read 2026-07-30. **Caveat, and it matters:** that page is about using Blender as a Python module (a Python program that imports `bpy`), not about a script handed to a running Blender with `-P`. The first sentence is about that module case. The second sentence is a plain statement about the Python API as such, and I read it as holding generally — but that reading is mine.

### E. If the setting is a path with a template expression in it, the text stays as typed

**Label: documented** (for the path field) / **inferred** (for what a script reads)

The `-o` argument accepts template expressions:

> "You can use path templating features such as {blend_name} in the path. See Blender’s documentation on path templates for more details."

Source: manual. Address: https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — date read 2026-07-30.

And the manual says those expressions are not filled in when the path is set, but later:

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

Source: manual. Address: https://docs.blender.org/manual/en/5.2/files/file_paths.html — date read 2026-07-30.

**The step I took:** the manual says the *path field* keeps the template text unexpanded. That the field a person sees and the value a script reads are the same thing is something the reader has to supply; the manual does not say it. So: a script that can read the path back would see `//my_render_dir/{blend_name}.png`, not the filled-in name — **inferred**.

### What this answer does not settle

- Whether a `-P` script placed before the blend-file has any scene at all to look at, and what it sees if it looks. The sources say the script can run before the file loads; they do not say what state exists at that moment.
- Which Python value, if any, `-o`, `-F`, `-E`, `-s`, `-e`, `-x` or `-t` write into, so whether a script can read any of them back. Not stated anywhere I searched.
- What a blend-file load overwrites beyond the render output path.
- What happens when the same setting is given twice on one command line.
- Whether the auto-execution switches (`-y` / `-Y`) affect a script given with `-P`. The manual's security page describes auto-execution as being about scripts carried inside blend-files ("Here are the different ways blend-files may automatically run scripts", https://docs.blender.org/manual/en/5.2/advanced/scripting/security.html), and the argument list describes `-Y` as disabling "Python-drivers & startup scripts". Neither says whether `-P` is caught by it.
- Whether a script can change a setting and have a later argument on the same command line respect it. Nothing in the sources addresses a script writing back.
- The `-c` / `--command` route is described above but is a different mechanism from `-P`; nothing here settles how the two interact.

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

The whole of this answer is about Blender driven from outside its own window: every quote above is from the command-line pages, and the manual introduces them as being for "automation and batch processing which require launching Blender with different arguments" (https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html, read 2026-07-30). The ordering rule, the `--` / `sys.argv` route and the `bpy.app` values are all stated without any condition that a window be open — `-b` is documented as "Run in background (often used for UI-less rendering)" and the manual's own examples of ordering use it.

Where it differs inside: the sources give one relevant difference. Where the API documentation describes running a script at launch to prepare state for a file that is loaded afterwards, it notes:

> "While this works for background tasks it has the downside that opening the file from the file selector won’t setup the name-space."

Source: Python API documentation. Address: https://docs.blender.org/api/5.2/bpy.app.html — date read 2026-07-30. That is: a launch-time script covers what the launch itself loads, not what a person opens by hand afterwards. Beyond that, the sources do not say whether anything in this answer behaves differently when a window is open. One nearby note, not about scripts: `-a` means "render the whole animation" as a render argument but, as a playback argument, "runs Blender as an animation player, to view movies and image sequences rendered in Blender (ignored if -b is set)" — so that one argument's meaning does depend on background mode (same page).

### Operating system

**Brought up to date by the hand tests of 2026-08-26.** This is no longer only what the sources do or do not say. Every result in the observed section above was got **twice** — once on the Creator's Mac (MacBook Pro, Apple M1 Pro, macOS 26.5.2) and once on a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 3060, driver 595.84) — on builds matching to the same commit hash. **The two machines agreed on every one of the sixteen, line for line, in all four versions.** The only lines that differed are two the machine itself supplies rather than Blender: the core count a control run reports (8 on the Mac, 32 on the rented machine, which misreports its own share of a bigger chip), and the graphics device the preferences hold (`METAL` on the Mac; `NONE`, with a device of type `OPTIX` listed, on the Linux machine) — both present in a control run before any command-line argument is given. **Windows is still untested** — no Windows machine has run it — and that is the whole of what is left open here.

**What the sources say, unchanged.**

The sources do not say that any of this differs by operating system. The ordering rule, the `--` / `sys.argv` route and the `bpy.app` values are all stated without an operating-system condition.

Two documented things do vary and bear on the question:

- Argument text is case sensitive everywhere: "Arguments are case sensitive! -F and -f are not the same." (https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html, read 2026-07-30). Stated as a flat rule, not per system.
- How much you can hand to a script at launch is capped by the operating system: of `--python-expr`, "The expression may be a complete multi-line script; you are limited only by the platform’s maximum argument length." (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30). The manual does not give the numbers.

For contrast, the same page does mark other arguments as system-specific where they are — "-con, --start-console" as "(Windows only)", "--no-window-frame" as "(Linux only)", "--debug-ghost" as "(Linux only)", "--qos <level>" as "(Windows only)", "/?" as "(Windows only)". None of the arguments in this answer carries such a mark. **That the absence of a mark means the behaviour is the same everywhere is my inference from how the page is written, not something it states.**

---

## Record of where I looked

**Files read in full or in large part**

- `manual_5.2/advanced/command_line/arguments.txt` — read in full (691 lines). This is the page that carries the "Argument Order" section, the `-o`, `-P`, `--`, `-c` and `-Y` entries, and the environment-variable list.
- `manual_5.2/advanced/command_line/render.txt` — read in full (94 lines).
- `manual_5.2/advanced/command_line/index.txt` — read in full.
- `manual_5.2/advanced/scripting/introduction.txt` — read (first 80 lines); nothing on launch-time arguments.
- `manual_5.2/advanced/scripting/security.txt` — read in full, for whether auto-execution touches `-P`.
- `manual_5.2/files/file_paths.txt` — read the "Path Templates" section (lines 1–40) plus a term list of every "template" line.
- `manual_5.2/files/asset_libraries/remote_asset_libraries.txt` — read the section around the `sys.argv` example.
- `manual_5.2/render/output/properties/output.txt` — read the Output panel and File Path descriptions (lines 1–60), checking whether the panel documentation mentions the command line. It does not.
- `api_5.2/bpy.app.txt` — read the file-loading/order-of-initialization passage, and every attribute name in the file, plus the full text of `background`, `factory_startup`, `debug`, `debug_value`, `use_event_simulate`, `online_access_override`, `tempdir`, `cachedir`, `python_args`, `binary_path`, `module`.
- `api_5.2/info_advanced_blender_as_bpy.txt` — read the "Command Line Arguments Unsupported" and "Startup and Preferences" sections.
- `api_5.2/bpy.utils.txt` — read `register_cli_command` and the whole "Custom Commands" section.
- `api_5.2/info_tips_and_tricks.txt` — read the "Use Blender without its User Interface" section (lines 100–190).
- `api_5.2/bpy.types.RenderSettings.txt` — read the `filepath`, `threads`, `threads_mode` and `use_single_layer` entries.
- `cycles_5.2/engine.py` — read lines 1–60, covering `_configure_argument_parser`, `_parse_command_line` and `init`.

**Searches run, and over what**

Every search below was run across `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` unless stated.

| Term searched | What came back |
| --- | --- |
| `sys.argv` | 1 hit in the API documentation (`bpy.utils`, an example), 3 in the manual (the `--` entry, a troubleshooting example, the asset-library example), 3 in Cycles' files. None in the release notes. |
| `argv` | as above, plus `maketx.py` and `engine.py` in Cycles. |
| `--python` | manual: the six Python-option entries and the asset-library example. API: `info_overview`, `bpy.app`, `info_tips_and_tricks`. Release notes: none. |
| `-P ` | 13 manual files, all incidental (the argument entry itself plus unrelated text). |
| `--render-output` / `render-output` | 4 hits, all in `arguments.txt` — the argument's own entry and the three ordering examples. Nothing anywhere else, in any of the four sources. |
| `render_output` | only image filenames in the manual (`_images/render_output_properties_*.png`). No Python identifier of that name anywhere. |
| `order they are given` / `executed in the order` / `Argument Order` | 3 hits total: two in `arguments.txt`, one in `render.txt`. Nothing in the API documentation or release notes. |
| `command line`, `command-line` | 9 API files, 13 manual files. I opened every API hit; the useful ones are listed above. |
| `path template`, `Path Template`, `path_template` | `arguments.txt`, `files/index.txt`, `files/file_paths.txt`, and `bpy.types.FunctionNodeFormatString` (unrelated — string formatting in a node). |
| `startup script` | 2 manual hits, both incidental (directory layout; the `-Y` entry). |
| `load_post`, `load_pre` | 5 API files each. Checked `bpy.app.handlers`; it describes when handlers fire, not what a launch-time script sees of a launch-time setting. |
| `before the blend`, `after the blend` | 1 API hit (`bpy.app`, quoted above); no hits for the second. |
| `consumes all remaining` | 1 manual hit (the `-c` entry). |
| `command line`, `command-line`, `argument` in the release notes only | 6 hits, all unrelated: a Cycles `maketx` command line, two Python-API argument changes, three operator-argument bug fixes. **Nothing in the 5.2 release notes about command-line argument order, `--python`, `--render-output`, or what a launch-time script sees.** |
| `path template`, `render output`, `filepath`, `output path` in the release notes only | the only render-output entry is "Saving render output can now be disabled. (blender/blender@342f37566f)" in `rendering.md`, which is about a different question on this sheet. Nothing bearing on question 13. |

**What I ruled out, and why**

- **The release notes for 5.2.** Swept with every term above; they carry nothing about command-line argument order, `-P`, `-o`, or what a script sees at launch. Ruled out.
- **The Cycles files, apart from `engine.py`.** Grepping the whole folder for `argv`, `argparse`, `cycles-device` and `cycles_device` returned only `engine.py` and `maketx.py`. `maketx.py` is a registered `--command`, a different mechanism. Ruled out.
- **The manual's render-output property pages.** Read to check whether the Output panel documentation says anything about being set from the command line. It does not; it describes the panel only. Ruled out.
- **`bpy.app.handlers`.** Read for whether a documented handler is the sanctioned way to see post-load state. It documents when handlers fire; it says nothing about command-line settings. Ruled out as an answer, kept as background.
- **The manual's images.** I checked the image references on the pages I used. The ones on `files/file_paths.txt` (`files_file_paths-invalid_path_template.png`, `..._tooltip.png`) show a path field with a template error, and the one on `render/output/properties/output.txt` (`render_output_properties_output_panel.png`) shows the Output panel. Neither could show anything about a command line or a script, so I did not open them; a picture of a panel cannot settle what a script sees at launch. This is the one place I chose not to open a source, and I am recording the reason rather than leaving it silent.
- **The wider web.** Not consulted. I did not need it and did not go.

---

## What a running Blender could still settle — pruned 2026-08-26

**The list the researcher wrote here has been run, and it is gone from this section rather than left standing beside its own answers.** Every item on it was carried out on 2026-08-26, on both machines — the Creator's Mac (MacBook Pro, Apple M1 Pro, macOS 26.5.2) and a rented Linux machine (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 3060) — in Blender 4.5.12 LTS, 5.0.1, 5.1.2 and 5.2.1, and every result is in the observed section above. The full list, with every command written out and every raw line kept, is `Audits/Blender Research/Writing/Hand tests — Architecture questions.md`.

**What is genuinely left.**

- **All of it on Windows.** The researcher's own item 9 asked for the checks to be repeated there. **No Windows machine has run it**, so the Windows answer is untested rather than answered — for whether the ordering rule holds there, whether `-o` lands in the same place, what `sys.argv` holds once a Windows shell has split the line, and whether a path written with backslashes reads back as typed. This is the reason for the silence, written in so it is not read as a result.
- **What the Cycles device override actually does to a render.** The runs above show only that it leaves nothing behind that a script can read; they do not show what the renderer then uses.
- **Whether `-o` reaching the active scene alone depends on which scene is active**, or on how a file names its scenes. One file, with two scenes, was used.

---

2026-08-26 — Claude Opus 5 (Claude Code subagent) — Writing, Blender 5.2.

*Declaration on helpers: I ran no helper agents for this pass. All searching and reading was done by me, within the two things I was permitted to read.*
