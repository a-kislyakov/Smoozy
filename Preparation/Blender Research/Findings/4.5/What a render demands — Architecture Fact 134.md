Sources consulted (all read 2026-07-30): Blender 4.5 Manual pages **Simplify**, **Rendering From The Command Line**, and **Command Line Arguments**; Blender 4.5 Python API pages **RenderSettings**, **Scene**, **PropertyGroup**, **Quickstart Introduction**, **Tips and Tricks**, **WindowManager Operators**, and **API Overview**; the supplied Blender 4.5 release notes; and the supplied Cycles 4.5 files `properties.py`, `ui.py`, and `version_update.py` plus a recursive term search of the supplied Cycles tree.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

## Question 6

**LABEL: documented** for the Manual and API descriptions of the setting names, meanings, ranges, and the general background-Python access route.

**LABEL: observed** for what the supplied Cycles files show: the texture-limit definitions, their defaults and choices, their attachment to `Scene.cycles`, and the Simplify panel wiring.

**LABEL: inferred** for the claims that the five built-in `scene.render` values are saved with the scene, that assigned Cycles texture values survive the exact save-close-open route asked about, and that all seven values can therefore be read back unchanged after that route. The sources support each part of that reasoning but do not report the round trip itself.

**LABEL: not in the sources** for a direct save-close-open result for these exact properties.

**LABEL: observed** for the 2026-08-29 terminal hand tests of exact save/reopen behavior, inactive subordinate values, per-scene separation, and official macOS/Linux cross-system read-back recorded below; and separately for the 2026-08-29 second-batch terminal hand tests, which measured what the Simplify switch does to a render at 128 by 128 in three controlled cases, and which read a Mac-authored file in the official Linux build with `--factory-startup`.

**NOT-COVERED:** The supplied sources do not show a worked save-close-open round trip for these exact seven properties, do not describe a standalone non-Blender parser for extracting them from a `.blend` file, and do not say that their storage or Python read-back differs by operating system. They name the scene properties, say generally that command-line rendering uses settings saved in a blend-file, say that values assigned to Blender `PropertyGroup` data are saved to disk, and document UI-less loading plus Python access. Joining those statements into exact persistence and read-back for these seven properties is an inference.

The sources identify one Simplify switch and separate viewport and final-render properties for each of the three limits:

| Meaning | Viewport value | Final-render value | Type and documented range/options |
|---|---|---|---|
| Governing switch | `scene.render.use_simplify` | the same switch is shown for both groups and makes both sets of controls active | Boolean; default `False` |
| Maximum subdivision | `scene.render.simplify_subdivision` | `scene.render.simplify_subdivision_render` | Integer, 0–32767; defaults 6 and 0 respectively |
| Child particles | `scene.render.simplify_child_particles` | `scene.render.simplify_child_particles_render` | Float, 0–1; defaults 1.0 and 0.0 respectively |
| Texture size | `scene.cycles.texture_limit` | `scene.cycles.texture_limit_render` | Enum: `OFF`, `128`, `256`, `512`, `1024`, `2048`, `4096`, or `8192`; both default to `OFF` |

This table maps the properties the sources name. It is not direct evidence that each value survives saving and reopening.

*Changed by the 2026-08-29 citation check: the switch row said the same switch "governs both groups". The quoted Cycles lines show only that the switch is drawn in the panel header and that both groups are made active from it; nothing quoted here says the switch changes what a render produces, and no such test was run in 4.5. The row was narrowed to what the quotes carry.*

The manual defines what these settings mean: **Max Subdivision** is the “Maximum number of subdivision by the Subdivision Surface modifiers.” **Child Particles** means “Show only a subset of all child hairs and particles.” **Texture Limit** “Automatically scales textures down so that they are no larger than the values chosen.” It presents separate **Viewport** and **Render** sections. Source: *Blender 4.5 Manual — Simplify*, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/simplify.html (read 2026-07-30).

The Python API documents the five non-texture fields verbatim:

- `simplify_child_particles`: “Global child particles percentage”; `simplify_child_particles_render`: “Global child particles percentage during rendering.”
- `simplify_subdivision`: “Global maximum subdivision level”; `simplify_subdivision_render`: “Global maximum subdivision level during rendering.”
- `use_simplify`: “Enable simplification of scene for quicker preview renders.”

Source: *Blender 4.5 Python API — RenderSettings*, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html#bpy.types.RenderSettings.simplify_child_particles, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html#bpy.types.RenderSettings.simplify_child_particles_render, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html#bpy.types.RenderSettings.simplify_subdivision, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html#bpy.types.RenderSettings.simplify_subdivision_render, and https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html#bpy.types.RenderSettings.use_simplify (read 2026-07-30).

**Observed in the supplied Cycles files:** the displayed controls are tied to those exact scene properties. In `ui.py`, lines 2274–2276 quote:

```python
def draw_header(self, context):
    rd = context.scene.render
    self.layout.prop(rd, "use_simplify", text="")
```

For the viewport group, `ui.py`, lines 2293–2302 quote:

```python
scene = context.scene
rd = scene.render
cscene = scene.cycles

layout.active = rd.use_simplify

col = layout.column()
col.prop(rd, "simplify_subdivision", text="Max Subdivision")
col.prop(rd, "simplify_child_particles", text="Child Particles")
col.prop(cscene, "texture_limit", text="Texture Limit")
```

For the render group, `ui.py`, lines 2318–2328 quote:

```python
scene = context.scene
rd = scene.render
cscene = scene.cycles

layout.active = rd.use_simplify

col = layout.column()

col.prop(rd, "simplify_subdivision_render", text="Max Subdivision")
col.prop(rd, "simplify_child_particles_render", text="Child Particles")
col.prop(cscene, "texture_limit_render", text="Texture Limit")
```

This directly identifies `use_simplify` as the switch that makes both sets of controls active. It does not say that turning the switch off erases the stored limit values.

**Observed in the supplied Cycles files:** the texture fields are Cycles scene properties. `properties.py`, lines 1009–1020, quotes:

```python
texture_limit: EnumProperty(
    name="Viewport Texture Limit",
    default='OFF',
    description="Limit texture size used by viewport rendering",
    items=enum_texture_limit
)

texture_limit_render: EnumProperty(
    name="Render Texture Limit",
    default='OFF',
    description="Limit texture size used by final rendering",
    items=enum_texture_limit
)
```

The allowed values are defined in `properties.py`, lines 185–194, including the exact phrases “No texture size limit” and “Limit texture size to 128 pixels” through “Limit texture size to 8192 pixels.” `CyclesRenderSettings` is declared as a `bpy.types.PropertyGroup` at line 376 and attached to `bpy.types.Scene.cycles` by a `PointerProperty` at lines 1100–1106. The API states: “The values assigned to Blender data are saved to disk but the class definitions are not.” Source: *Blender 4.5 Python API — PropertyGroup*, https://docs.blender.org/api/4.5/bpy.types.PropertyGroup.html (read 2026-07-30). Together, these support the inference that assigned Cycles texture-limit values are saved. The quoted guarantee does not name these two fields, and the sources do not show them being read back after a save-close-open round trip.

For the built-in `scene.render` fields, the Scene API identifies `render` as `RenderSettings`, and identifies `cycles` as “Cycles render settings” of type `CyclesRenderSettings`. Source: *Blender 4.5 Python API — Scene*, https://docs.blender.org/api/4.5/bpy.types.Scene.html#bpy.types.Scene.render and https://docs.blender.org/api/4.5/bpy.types.Scene.html#bpy.types.Scene.cycles (read 2026-07-30). The command-line rendering manual says `blender -b file.blend -a` will “Render the whole animation using all the settings saved in the blend-file.” Source: *Blender 4.5 Manual — Rendering From The Command Line*, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html (read 2026-07-30). Neither quote names these five fields as saved settings. Their persistence is inferred, not directly documented.

### Inferred read-back after opening without Blender's interface

The documented property paths and background-Python route support the inference that all seven values can be read after Blender loads the file without its interface:

```python
scene = bpy.data.scenes["Scene"]
scene.render.use_simplify
scene.render.simplify_subdivision
scene.render.simplify_subdivision_render
scene.render.simplify_child_particles
scene.render.simplify_child_particles_render
scene.cycles.texture_limit
scene.cycles.texture_limit_render
```

The pieces of this path are documented, but their application to the exact round trip is inferred rather than reported. The API quickstart says, “Accessing data from the currently loaded blend-file is done with the module bpy.data,” and demonstrates `bpy.data.scenes[0].render.resolution_percentage`. Source: *Blender 4.5 Python API — Quickstart Introduction*, https://docs.blender.org/api/4.5/info_quickstart.html#accessing-data (read 2026-07-30). The API's scripting guidance says, “Use Blender without it’s User Interface,” gives `blender myscene.blend --background --python myscript.py`, and says such a script has “some data to operate on.” Source: *Blender 4.5 Python API — Tips and Tricks*, https://docs.blender.org/api/4.5/info_tips_and_tricks.html#use-blender-without-it-s-user-interface (read 2026-07-30). The command-line reference describes `--background` as “Run in background (often used for UI-less rendering)” and `--python-expr` as “Run the given expression as a Python script.” Source: *Blender 4.5 Manual — Command Line Arguments*, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html (read 2026-07-30).

### Observed hand-test result, 2026-08-29

GPT-5.6 Sol in Codex ran the saved procedure `Audits/Blender Research/What a render demands/Fact 134 test procedure/make_fact134_files.py` and `read_fact134.py` from a terminal in exact Blender 4.5.12 LTS, hash `84afd5f785f7`. The machines were the Creator's MacBookPro18,3, Apple M1 Pro, 16 GB, macOS 26.6.2 build 25G83, and Vast.ai instance 49065406 in Poland: Ubuntu 22.04.4 LTS in Docker on Linux 6.8.0-51-generic, AMD Ryzen 5 5600X with a 5.76-core cgroup quota and 22,786,605,056-byte memory limit, one assigned 12,288 MiB RTX 3060 on driver 550.120, and no display server. The exact executable paths were `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender` and `/opt/blender/4.5.12/blender`.

The setup process printed the two scene records immediately before `bpy.ops.wm.save_as_mainfile(filepath=...)`. Separate fresh background processes then reopened the enabled and disabled files. On both operating systems, both scenes returned their own values; all seven enabled values survived; and the disabled file returned `use_simplify: false` without changing a subordinate value. Mac-authored files also reopened unchanged in the official Linux build, and Linux-authored files reopened unchanged in the official Mac build. Both cross-system reads accidentally omitted the planned `--factory-startup`. The Linux reader ran in a clean official installation; on the Mac, installed add-ons initialized and produced unrelated output. Both readers still reached `FACT134_READBACK_END` and both processes exited 0. The safety reviewer declined a clean repeat because one Mac add-on had made an authenticated asset/API request. The cross-system claim is therefore for the processes actually run, not the stricter factory-startup variant.

*Changed by the 2026-08-29 citation check: this paragraph recorded only the Mac reader as having omitted the flag. The completed hand-test list for this same run records that the Linux readers were also invoked without it, in clean official installations. Corrected to that record.*

Raw reader payload from the Mac process, word for word:

```text
FACT134_READBACK_BEGIN
version 4.5.12 LTS
build_platform b'Darwin'
operating_system macOS-26.6.2-arm64-arm-64bit
background True
loaded_path /private/tmp/fact134/cross/linux/4.5/fact134-enabled.blend
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_A", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "use_simplify": true}
cycles_items FACT134_A [('texture_limit', 4), ('texture_limit_render', 5)]
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_B", "simplify_child_particles": 0.2199999988079071, "simplify_child_particles_render": 0.8100000023841858, "simplify_subdivision": 1, "simplify_subdivision_render": 7, "texture_limit": "256", "texture_limit_render": "4096", "use_simplify": true}
cycles_items FACT134_B [('texture_limit', 2), ('texture_limit_render', 6)]
FACT134_READBACK_END
```

Raw reader payload from the Linux process opening the Mac-authored file, word for word:

```text
FACT134_READBACK_BEGIN
version 4.5.12 LTS
build_platform b'Linux'
operating_system Linux-6.8.0-51-generic-x86_64-with-glibc2.35
background True
loaded_path /opt/fact134/cross/mac/4.5/fact134-enabled.blend
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_A", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "use_simplify": true}
cycles_items FACT134_A [('texture_limit', 4), ('texture_limit_render', 5)]
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_B", "simplify_child_particles": 0.2199999988079071, "simplify_child_particles_render": 0.8100000023841858, "simplify_subdivision": 1, "simplify_subdivision_render": 7, "texture_limit": "256", "texture_limit_render": "4096", "use_simplify": true}
cycles_items FACT134_B [('texture_limit', 2), ('texture_limit_render', 6)]
FACT134_READBACK_END
```

The disabled-file raw JSON on both systems was:

```text
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_DISABLED", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "use_simplify": false}
```

### Observed hand-test result, second batch, 2026-08-29 — the Simplify switch and the picture

Claude Opus 5 in Claude Code ran `Audits/Blender Research/What a render demands/Fact 134 test procedure/runtime_fact134_older.py` from a terminal in exact Blender 4.5.12 LTS, hash `84afd5f785f7`. It is the saved 5.2 runtime file with two changes: the EEVEE engine name is taken from the running Blender's own list, because 4.5 calls it `BLENDER_EEVEE_NEXT` and refuses the 5.2 name; and the texture case sets the texture limit alone, because `scene.cycles.texture_resolution_render` does not exist in 4.5. The run prints both of those readings.

Two machines, kept apart. The first was the Creator's MacBookPro18,3: Apple M1 Pro, 16 GB, macOS 26.6.2 build 25G83 on Darwin 25.6.0, Apple's own graphics and no second card, executable `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`. The second was Vast.ai instance 49111279 in South Korea, a Docker container established as one by `/.dockerenv`, by process 1 being `sh` and by `/proc/1/cgroup` reading `0::/`: Ubuntu 22.04.4 LTS on `Linux 5.15.0-186-generic #196-Ubuntu SMP Sat Jun 20 16:09:34 UTC 2026 x86_64`, glibc 2.35; an Intel Xeon E5-2673 v3 at 2.40GHz with 24 visible threads and a cgroup quota of `1152000 100000`, which is 11.52 processor cores; a memory cgroup limit of 22,769,827,840 bytes; one assigned NVIDIA GeForce GTX 1660 with 6,144 MiB on driver 580.159.03; a 25 GB overlay root; and no display server, with `DISPLAY` empty and no `/tmp/.X11-unix`. Blender came from Blender's own download server as the official Linux x64 archive and reported the same build hash as the Mac. Its executable was `/opt/blender/4.5.12/blender`.

The script built three cases at 128 by 128 — a subdivided cube against a render subdivision limit of 0, a thirty-hair emitter with interpolated children against a render child percentage of 0, and a plane carrying a generated 512 by 512 checker against a texture limit of 128 — rendered each twice with the switch off and once with it on, and compared the loaded pixel channels rather than the file bytes.

The Mac printed, word for word:

```text
version 4.5.12 LTS
build_platform b'Darwin'
eevee_engine_value BLENDER_EEVEE_NEXT
has_texture_resolution_render False
pixel_diff {"case": "subdivision", "changed_channels": 15, "comparison": "off-repeat", "max_abs_diff": 0.00784313678741455, "mean_abs_diff": 1.2566060831886716e-06}
pixel_diff {"case": "subdivision", "changed_channels": 6595, "comparison": "off-versus-on", "max_abs_diff": 0.7725490927696228, "mean_abs_diff": 0.06405873641016058}
pixel_diff {"case": "child_particles", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "child_particles", "changed_channels": 10258, "comparison": "off-versus-on", "max_abs_diff": 0.7725490927696228, "mean_abs_diff": 0.05143505098484269}
pixel_diff {"case": "texture", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "texture", "changed_channels": 49152, "comparison": "off-versus-on", "max_abs_diff": 0.501960813999176, "mean_abs_diff": 0.3756949254861581}
```

The rented Linux machine printed, word for word:

```text
version 4.5.12 LTS
build_platform b'Linux'
eevee_engine_value BLENDER_EEVEE_NEXT
has_texture_resolution_render False
pixel_diff {"case": "subdivision", "changed_channels": 15, "comparison": "off-repeat", "max_abs_diff": 0.00784313678741455, "mean_abs_diff": 1.2566060831886716e-06}
pixel_diff {"case": "subdivision", "changed_channels": 6595, "comparison": "off-versus-on", "max_abs_diff": 0.7725490927696228, "mean_abs_diff": 0.06405873641016058}
pixel_diff {"case": "child_particles", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "child_particles", "changed_channels": 10258, "comparison": "off-versus-on", "max_abs_diff": 0.7725490927696228, "mean_abs_diff": 0.05143523049999743}
pixel_diff {"case": "texture", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "texture", "changed_channels": 49152, "comparison": "off-versus-on", "max_abs_diff": 0.501960813999176, "mean_abs_diff": 0.37569498532454304}
```

**Whether the two machines agreed.** They agreed on every changed-channel count and every largest single-channel difference, and on the subdivision means to the last digit. They differed in the last few digits of two mean figures: the child-particle mean was `0.05143505098484269` on the Mac and `0.05143523049999743` on Linux, and the texture mean was `0.3756949254861581` on the Mac and `0.37569498532454304` on Linux. Those two readings are recorded as they came and are not merged.

This is a runtime result and it is separate from persistence. It does not widen the table row above, which records what the documentation and the supplied Cycles files carry about the switch. This entry records only what the two machines printed.

### Observed hand-test result, second batch, 2026-08-29 — the Linux reader with `--factory-startup`

The first batch's cross-system readers were all invoked without `--factory-startup`, which is recorded above. The 2026-08-29 citation check then ran the Mac half of that read with the flag, against the first batch's Linux-authored files still on this Mac, and every target line came back identical. The Linux half was still open. This run closed it on the second rented machine described above.

Four fresh `fact134-enabled.blend` files were authored on the Mac with the shared setup command, into `/private/tmp/fact134-batch2/mac/<version>` so that the first batch's files were untouched, and copied across unchanged; the 4.5 file read `eaf16b1f4a165abee7f7ade46e6e895d9292ff08bf56993f14cfdd761de60216` on both machines. The Linux installation had no user configuration folder and the command carried `--factory-startup`, so no add-on ran and the output holds nothing but Blender's own lines. The command was:

```text
/opt/blender/4.5.12/blender --background --factory-startup /opt/fact134/cross/mac/4.5/fact134-enabled.blend --python /opt/fact134/procedure/read_fact134.py
```

It came back, word for word:

```text
FACT134_READBACK_BEGIN
version 4.5.12 LTS
build_platform b'Linux'
operating_system Linux-5.15.0-186-generic-x86_64-with-glibc2.35
background True
loaded_path /opt/fact134/cross/mac/4.5/fact134-enabled.blend
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_A", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "use_simplify": true}
cycles_items FACT134_A [('texture_limit', 4), ('texture_limit_render', 5)]
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_B", "simplify_child_particles": 0.2199999988079071, "simplify_child_particles_render": 0.8100000023841858, "simplify_subdivision": 1, "simplify_subdivision_render": 7, "texture_limit": "256", "texture_limit_render": "4096", "use_simplify": true}
cycles_items FACT134_B [('texture_limit', 2), ('texture_limit_render', 6)]
FACT134_READBACK_END
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/opt/fact134/cross/mac/4.5/fact134-enabled.blend"

Blender quit
```

The process exited 0. Every value matches the value the Mac printed before saving, through the same property paths. The two directions of the crossing were done on different days against different Linux machines, so they are two results rather than one.

### Search record

**Files/corpora searched:** recursive text searches were run across the supplied Blender 4.5 manual, Python API, release notes, and Cycles trees. The focused documents opened were `render/cycles/render_settings/simplify.txt`, `advanced/command_line/render.txt`, `advanced/command_line/arguments.txt`, `bpy.types.RenderSettings.txt`, `bpy.types.Scene.txt`, `bpy.types.PropertyGroup.txt`, `bpy.ops.wm.txt`, `info_quickstart.txt`, `info_tips_and_tricks.txt`, and `info_overview.txt`; the focused Cycles files opened were `properties.py`, `ui.py`, and `version_update.py`.

**Terms used:** `simplify`, `use_simplify`, `simplify_subdivision`, `simplify_subdivision_render`, `simplify_child_particles`, `simplify_child_particles_render`, `child particles`, `texture limit`, `texture size`, `texture_limit`, `texture_limit_render`, `CyclesRenderSettings`, `PropertyGroup`, `PointerProperty`, `saved in blend`, `saved to disk`, `open_mainfile`, `load blend`, `background mode`, `command line`, `python-expr`, `bpy.data.scenes`, and combinations of `save`, `settings`, `scene`, `file`, and `read`.

**Pages/places opened:** the public pages cited above; the WindowManager operator entries for `bpy.ops.wm.open_mainfile` (“Open a Blender file”) and `bpy.ops.wm.save_as_mainfile` (“Save the current file in the desired location”) at https://docs.blender.org/api/4.5/bpy.ops.wm.html#bpy.ops.wm.open_mainfile and https://docs.blender.org/api/4.5/bpy.ops.wm.html#bpy.ops.wm.save_as_mainfile (read 2026-07-30); and the quoted Cycles line ranges.

**Ruled out:** the supplied release notes contain no relevant statement about persistence or read-back of these Simplify fields; the supplied EEVEE manual render-settings tree has no Simplify page, while the relevant manual page is under Cycles; the supplied API has no standalone generated `CyclesRenderSettings` page listing the two texture fields, so their definitions and registration had to be taken from the supplied Cycles files; and the recursive Cycles search found the relevant texture definitions and panel wiring but no supplied engine-side code that independently demonstrates their save-close-open round trip. Unrelated uses of “simplify” (Grease Pencil, sculpting, geometry simplification, and mathematical comments) were excluded.

**Outside the interface** — Observed in Blender 4.5.12's official macOS arm64 and Linux x64 builds: a fresh background process read the switch and all six limits through `bpy.data.scenes[...]` after save and close, including distinct per-scene values and inactive subordinate values. Observed 2026-08-29 in the second batch: a fresh background process with no interface, on both machines, also showed the switch changing the rendered picture in controlled subdivision, child-particle and texture cases, so what the switch does is not confined to Blender's own window; and the Linux read of a Mac-authored file was repeated with `--factory-startup`, so that leg no longer rests on a process with add-ons loaded. Reading these values by parsing the `.blend` without Blender remains unestablished.

**Operating system** — Observed on macOS 26.6.2 arm64 and Ubuntu 22.04.4/Linux x64, including official-build files crossed in both directions. Observed again 2026-08-29 on a second rented Linux machine, Ubuntu 22.04.4 on Linux 5.15.0-186-generic: the switch's effect on the picture came out the same as on the Mac, and the official Linux build read the Mac-authored file cleanly under `--factory-startup`. Windows x64, Windows arm64, and Linux distribution packages remain untested.

2026-08-28 — GPT-5.6 Sol (Codex) — What a render demands, Blender 4.5.
