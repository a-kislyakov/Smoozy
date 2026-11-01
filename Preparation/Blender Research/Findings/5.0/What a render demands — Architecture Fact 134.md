# What a render demands — Architecture Fact 134

Sources consulted (read 2026-07-30): Blender 5.0 Manual; Blender 5.0 Python API; Blender 5.0 release notes; supplied Blender 5.0 Cycles files.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

## Question 6

What, if anything, does a saved scene record about limits on subdivision, child particles, and texture size, including any switch governing those limits, and what can be read back when the file is opened from outside Blender's interface?

**LABEL: inferred** for the source-only persistence conclusion; **observed** for the 2026-08-29 exact save/reopen, inactive-value, EEVEE-boundary, and macOS/Linux hand tests recorded below, and separately for the 2026-08-29 second-batch hand tests, which measured what the Simplify switch does to a render at 128 by 128 in three controlled cases and read a Mac-authored file in the official Linux build with `--factory-startup`.

**Answer:** Blender 5.0 exposes seven relevant values per scene: one master Simplify switch, plus separate viewport and final-render values for maximum subdivision, child-particle percentage, and Cycles texture size. The supplied sources document the values, their scene locations, their meanings, and ways to open a blend-file and read Python properties without the graphical interface. They do **not** directly document or demonstrate that non-default values survive a save/close/reopen round trip. Therefore, the claim that a saved scene records and restores all seven values is a strong inference, not a documented or observed result in this source set.

| What to read | Python path on each scene | Documented value |
|---|---|---|
| Master switch | `scene.render.use_simplify` | Boolean; default `False`. It enables scene simplification. |
| Viewport subdivision limit | `scene.render.simplify_subdivision` | Integer from 0 to 32767; default 6. |
| Final-render subdivision limit | `scene.render.simplify_subdivision_render` | Integer from 0 to 32767; default 0. |
| Viewport child-particle limit | `scene.render.simplify_child_particles` | Float from 0 to 1; default 1.0. |
| Final-render child-particle limit | `scene.render.simplify_child_particles_render` | Float from 0 to 1; default 0.0. |
| Viewport texture limit (Cycles) | `scene.cycles.texture_limit` | Enum: `OFF`, `128`, `256`, `512`, `1024`, `2048`, `4096`, or `8192`; default `OFF`. |
| Final-render texture limit (Cycles) | `scene.cycles.texture_limit_render` | Same enum; default `OFF`. |

The viewport and final-render values are distinct, not one shared limit. In the supplied Cycles UI file, both groups become active only when `scene.render.use_simplify` is true. The texture limits are Cycles settings (`scene.cycles`); the subdivision, particle, and switch values are Blender render settings (`scene.render`). Turning the master switch off should not be described as erasing subordinate values: the sources show only that the UI groups are inactive, not what a save/reopen does to their stored values.

### Exact source words

- The manual says a scene includes render settings: “A scene defines the visible objects, camera view, lighting setup, and render settings that make up a specific part of a project.” Source: Blender 5.0 Manual, *Scenes — Introduction*, https://docs.blender.org/manual/en/5.0/scene_layout/scene/introduction.html (read 2026-07-30).
- The Simplify manual defines the three kinds of limit: “Maximum number of subdivision by the Subdivision Surface modifiers”; “Show only a subset of all child hairs and particles”; and “Automatically scales textures down so that they are no larger than the values chosen.” Source: Blender 5.0 Manual, *Cycles — Render Settings — Simplify*, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/simplify.html (read 2026-07-30).
- The API calls `simplify_subdivision` the “Global maximum subdivision level” and `simplify_subdivision_render` the “Global maximum subdivision level during rendering.” Source: Blender 5.0 Python API, `bpy.types.RenderSettings`, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html#bpy.types.RenderSettings.simplify_subdivision and https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html#bpy.types.RenderSettings.simplify_subdivision_render (read 2026-07-30).
- The API calls `simplify_child_particles` the “Global child particles percentage” and its render partner the “Global child particles percentage during rendering.” Source: Blender 5.0 Python API, `bpy.types.RenderSettings`, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html#bpy.types.RenderSettings.simplify_child_particles and https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html#bpy.types.RenderSettings.simplify_child_particles_render (read 2026-07-30).
- The API describes `use_simplify` as “Enable simplification of scene for quicker preview renders.” Source: Blender 5.0 Python API, `bpy.types.RenderSettings`, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html#bpy.types.RenderSettings.use_simplify (read 2026-07-30).
- The supplied Cycles property definitions call the two fields “Viewport Texture Limit” and “Render Texture Limit,” with descriptions “Limit texture size used by viewport rendering” and “Limit texture size used by final rendering.” The enum includes “No texture size limit” and limits from 128 through 8192 pixels. Source: Cycles `properties.py`, `enum_texture_limit` at lines 181–190 and `CyclesRenderSettings.texture_limit` / `texture_limit_render` at lines 1009–1020 (read 2026-07-30).
- The supplied Cycles UI reads the master switch from `context.scene.render.use_simplify`, reads viewport fields from `scene.render.simplify_subdivision`, `scene.render.simplify_child_particles`, and `scene.cycles.texture_limit`, and reads final-render fields from their `_render` partners. Both child panels use `layout.active = rd.use_simplify`. Source: Cycles `ui.py`, `CYCLES_RENDER_PT_simplify`, `CYCLES_RENDER_PT_simplify_viewport`, and `CYCLES_RENDER_PT_simplify_render`, lines 2269–2329 (read 2026-07-30).
- The API identifies `Scene` as a “Scene data-block, consisting in objects and defining time and render related settings”; `Scene.render` is a `RenderSettings` value, and `Scene.cycles` is “Cycles render settings.” Source: Blender 5.0 Python API, `bpy.types.Scene`, https://docs.blender.org/api/5.0/bpy.types.Scene.html, https://docs.blender.org/api/5.0/bpy.types.Scene.html#bpy.types.Scene.render, and https://docs.blender.org/api/5.0/bpy.types.Scene.html#bpy.types.Scene.cycles (read 2026-07-30).

### Reading outside the graphical interface

The documented route is to start Blender with the blend-file in background mode and run Python after the file argument. The manual says background mode will “Run in background (often used for UI-less rendering)” and documents `--python-expr` as “Run the given expression as a Python script.” It also warns that arguments execute in order and shows the blend-file before later render arguments. Source: Blender 5.0 Manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (read 2026-07-30).

The other documented route is Python: `bpy.ops.wm.open_mainfile` is described as “Open a Blender file,” and `bpy.data.scenes` is described as “Scene data-blocks.” Source: Blender 5.0 Python API, https://docs.blender.org/api/5.0/bpy.ops.wm.html#bpy.ops.wm.open_mainfile and https://docs.blender.org/api/5.0/bpy.types.BlendData.html#bpy.types.BlendData.scenes (read 2026-07-30).

Once loaded, Blender running without its graphical interface can expose each scene's seven paths through its Python API. For example, the intended read-back shape is:

```python
import bpy

for scene in bpy.data.scenes:
    print(scene.name, {
        "use_simplify": scene.render.use_simplify,
        "viewport_subdivision": scene.render.simplify_subdivision,
        "render_subdivision": scene.render.simplify_subdivision_render,
        "viewport_child_particles": scene.render.simplify_child_particles,
        "render_child_particles": scene.render.simplify_child_particles_render,
        "viewport_texture_limit": scene.cycles.texture_limit,
        "render_texture_limit": scene.cycles.texture_limit_render,
    })
```

This code is a precise read recipe derived from the documented API and supplied Cycles files. It was not run for this finding, so it is not evidence of save persistence.

**NOT-COVERED:** None of the supplied manual pages, Python API pages, Blender 5.0 release notes, or Cycles files directly says that these seven particular values are serialized into a `.blend`, then restored unchanged after reopening in Blender 5.0 background mode. They also do not establish whether disabled subordinate values remain set, how older/newer Blender versions migrate these exact fields, or what happens to `scene.cycles.texture_limit*` if Cycles is unavailable or not registered.

### Observed hand-test result, 2026-08-29

GPT-5.6 Sol in Codex ran the saved terminal procedure `Fact 134 test procedure/make_fact134_files.py` and `read_fact134.py` in exact Blender 5.0.1, hash `a3db93c5b259`. It used the Creator's MacBookPro18,3 (Apple M1 Pro, 16 GB, macOS 26.6.2 build 25G83) and Vast.ai instance 49065406 in Poland (Ubuntu 22.04.4 LTS in Docker on Linux 6.8.0-51-generic; AMD Ryzen 5 5600X, 5.76-core quota, 22,786,605,056-byte memory limit; one assigned 12,288 MiB RTX 3060, driver 550.120; no display server). The exact executables were `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender` and `/opt/blender/5.0.1/blender`.

On both systems, separate post-save background processes returned the two scenes' distinct enabled values and the disabled file's inactive subordinate values unchanged. A file saved after changing the active engine to `BLENDER_EEVEE` reopened with that engine, `scene.cycles` present, and its Cycles values unchanged. Mac-authored and Linux-authored files also crossed between the two official builds without a missing, reset, clamped, swapped, or changed target value. Both cross-readers accidentally omitted `--factory-startup`. The Linux reader ran in a clean official installation; on the Mac, installed add-ons emitted unrelated output and one made an authenticated asset/API request. Both readers still reached their end marker and exited 0. A safety review declined a clean repeat. This cross-system result therefore describes the actual processes, not the planned factory-startup variant.

*Changed by the 2026-08-29 citation check: this paragraph recorded only the Mac reader as having omitted the flag. The completed hand-test list for this same run records that the Linux readers were also invoked without it, in clean official installations. Corrected to that record.*

Raw cross-system payloads, word for word apart from the `loaded_path`, `build_platform`, and `operating_system` lines shown for each machine below:

```text
FACT134_READBACK_BEGIN
version 5.0.1
build_platform b'Darwin'
operating_system macOS-26.6.2-arm64-arm-64bit
background True
loaded_path /private/tmp/fact134/cross/linux/5.0/fact134-enabled.blend
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_A", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "use_simplify": true}
cycles_items FACT134_A [('sampling_pattern', 1), ('texture_limit', 4), ('texture_limit_render', 5)]
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_B", "simplify_child_particles": 0.2199999988079071, "simplify_child_particles_render": 0.8100000023841858, "simplify_subdivision": 1, "simplify_subdivision_render": 7, "texture_limit": "256", "texture_limit_render": "4096", "use_simplify": true}
cycles_items FACT134_B [('texture_limit', 2), ('texture_limit_render', 6)]
FACT134_READBACK_END
```

```text
FACT134_READBACK_BEGIN
version 5.0.1
build_platform b'Linux'
operating_system Linux-6.8.0-51-generic-x86_64-with-glibc2.35
background True
loaded_path /opt/fact134/cross/mac/5.0/fact134-enabled.blend
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_A", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "use_simplify": true}
cycles_items FACT134_A [('sampling_pattern', 1), ('texture_limit', 4), ('texture_limit_render', 5)]
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_B", "simplify_child_particles": 0.2199999988079071, "simplify_child_particles_render": 0.8100000023841858, "simplify_subdivision": 1, "simplify_subdivision_render": 7, "texture_limit": "256", "texture_limit_render": "4096", "use_simplify": true}
cycles_items FACT134_B [('texture_limit', 2), ('texture_limit_render', 6)]
FACT134_READBACK_END
```

The disabled and alternate-engine raw JSON lines on both machines were:

```text
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_DISABLED", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "use_simplify": false}
{"cycles_present": true, "engine": "BLENDER_EEVEE", "scene": "FACT134_OTHER_ENGINE", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "use_simplify": true}
```

### Observed hand-test result, second batch, 2026-08-29 — the Simplify switch and the picture

Claude Opus 5 in Claude Code ran `Audits/Blender Research/What a render demands/Fact 134 test procedure/runtime_fact134_older.py` from a terminal in exact Blender 5.0.1, hash `a3db93c5b259`. It is the saved 5.2 runtime file with two changes: the EEVEE engine name is taken from the running Blender's own list, and the texture case sets the texture limit alone because `scene.cycles.texture_resolution_render` does not exist in 5.0. The run prints both readings; in 5.0 the engine name came back `BLENDER_EEVEE` and the texture-resolution property came back absent.

Two machines, kept apart. The first was the Creator's MacBookPro18,3: Apple M1 Pro, 16 GB, macOS 26.6.2 build 25G83 on Darwin 25.6.0, Apple's own graphics and no second card, executable `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`. The second was Vast.ai instance 49111279 in South Korea, a Docker container established as one by `/.dockerenv`, by process 1 being `sh` and by `/proc/1/cgroup` reading `0::/`: Ubuntu 22.04.4 LTS on `Linux 5.15.0-186-generic #196-Ubuntu SMP Sat Jun 20 16:09:34 UTC 2026 x86_64`, glibc 2.35; an Intel Xeon E5-2673 v3 at 2.40GHz with 24 visible threads and a cgroup quota of `1152000 100000`, which is 11.52 processor cores; a memory cgroup limit of 22,769,827,840 bytes; one assigned NVIDIA GeForce GTX 1660 with 6,144 MiB on driver 580.159.03; a 25 GB overlay root; and no display server, with `DISPLAY` empty and no `/tmp/.X11-unix`. Blender came from Blender's own download server as the official Linux x64 archive and reported the same build hash as the Mac. Its executable was `/opt/blender/5.0.1/blender`.

The script built three cases at 128 by 128 — a subdivided cube against a render subdivision limit of 0, a thirty-hair emitter with interpolated children against a render child percentage of 0, and a plane carrying a generated 512 by 512 checker against a texture limit of 128 — rendered each twice with the switch off and once with it on, and compared the loaded pixel channels rather than the file bytes.

The Mac printed, word for word:

```text
version 5.0.1
build_platform b'Darwin'
eevee_engine_value BLENDER_EEVEE
has_texture_resolution_render False
pixel_diff {"case": "subdivision", "changed_channels": 15, "comparison": "off-repeat", "max_abs_diff": 0.003921568393707275, "mean_abs_diff": 8.97575773706194e-07}
pixel_diff {"case": "subdivision", "changed_channels": 6587, "comparison": "off-versus-on", "max_abs_diff": 0.5450980961322784, "mean_abs_diff": 0.044957005156902596}
pixel_diff {"case": "child_particles", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "child_particles", "changed_channels": 10240, "comparison": "off-versus-on", "max_abs_diff": 0.5450980961322784, "mean_abs_diff": 0.03819682392895629}
pixel_diff {"case": "texture", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "texture", "changed_channels": 49152, "comparison": "off-versus-on", "max_abs_diff": 0.501960813999176, "mean_abs_diff": 0.3757036020519706}
```

The rented Linux machine printed, word for word:

```text
version 5.0.1
build_platform b'Linux'
eevee_engine_value BLENDER_EEVEE
has_texture_resolution_render False
pixel_diff {"case": "subdivision", "changed_channels": 15, "comparison": "off-repeat", "max_abs_diff": 0.003921568393707275, "mean_abs_diff": 8.97575773706194e-07}
pixel_diff {"case": "subdivision", "changed_channels": 6587, "comparison": "off-versus-on", "max_abs_diff": 0.5450980961322784, "mean_abs_diff": 0.044957005156902596}
pixel_diff {"case": "child_particles", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "child_particles", "changed_channels": 10240, "comparison": "off-versus-on", "max_abs_diff": 0.5450980961322784, "mean_abs_diff": 0.03819682392895629}
pixel_diff {"case": "texture", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "texture", "changed_channels": 49152, "comparison": "off-versus-on", "max_abs_diff": 0.501960813999176, "mean_abs_diff": 0.3757036020519706}
```

**Whether the two machines agreed.** They agreed on every figure, digit for digit.

This is a runtime result and it is separate from persistence. This entry records only what the two machines printed.

### Observed hand-test result, second batch, 2026-08-29 — the Linux reader with `--factory-startup`

The first batch's cross-system readers were all invoked without `--factory-startup`, which is recorded above. The 2026-08-29 citation check then ran the Mac half of that read with the flag, against the first batch's Linux-authored files still on the Mac, and every target line came back identical. The Linux half was still open. This run closed it on the second rented machine described above.

Four fresh `fact134-enabled.blend` files were authored on the Mac with the shared setup command, into `/private/tmp/fact134-batch2/mac/<version>` so that the first batch's files were untouched, and copied across unchanged; the 5.0 file read `1206de6a04b01989ba89ae4989ac6a238b972be3289189c8037193c0ba5a87db` on both machines. The Linux installation had no user configuration folder and the command carried `--factory-startup`, so no add-on ran and the output holds nothing but Blender's own lines. The command was:

```text
/opt/blender/5.0.1/blender --background --factory-startup /opt/fact134/cross/mac/5.0/fact134-enabled.blend --python /opt/fact134/procedure/read_fact134.py
```

It came back, word for word:

```text
00:02.079  blend            | Read blend: "/opt/fact134/cross/mac/5.0/fact134-enabled.blend"
FACT134_READBACK_BEGIN
version 5.0.1
build_platform b'Linux'
operating_system Linux-5.15.0-186-generic-x86_64-with-glibc2.35
background True
loaded_path /opt/fact134/cross/mac/5.0/fact134-enabled.blend
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_A", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "use_simplify": true}
cycles_items FACT134_A [('sampling_pattern', 1), ('texture_limit', 4), ('texture_limit_render', 5)]
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_B", "simplify_child_particles": 0.2199999988079071, "simplify_child_particles_render": 0.8100000023841858, "simplify_subdivision": 1, "simplify_subdivision_render": 7, "texture_limit": "256", "texture_limit_render": "4096", "use_simplify": true}
cycles_items FACT134_B [('texture_limit', 2), ('texture_limit_render', 6)]
FACT134_READBACK_END
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)

Blender quit
```

The process exited 0. Every value matches the value the Mac printed before saving, through the same property paths. The two directions of the crossing were done on different days against different Linux machines, so they are two results rather than one.

### Search record

**Files and source groups searched:** the full supplied Blender 5.0 manual text tree, Python API text tree, Blender 5.0 release-note Markdown tree, and Cycles 5.0 Python/C/C++ header tree. Focused files opened were `manual_5.0/render/cycles/render_settings/simplify.txt`, `manual_5.0/scene_layout/scene/introduction.txt`, `manual_5.0/files/blend/open_save.txt`, `manual_5.0/files/blend/compatibility.txt`, `manual_5.0/advanced/command_line/arguments.txt`, `api_5.0/bpy.types.RenderSettings.txt`, `api_5.0/bpy.types.Scene.txt`, `api_5.0/bpy.types.BlendData.txt`, `api_5.0/bpy.ops.wm.txt`, `api_5.0/info_advanced_blender_as_bpy.txt`, `api_5.0/bpy.props.txt`, `api_5.0/bpy.types.Property.txt`, `api_5.0/bpy_types_enum_items/property_flag_items.txt`, `cycles_5.0/properties.py`, `cycles_5.0/ui.py`, and the matching portions of `cycles_5.0/version_update.py` and the release notes.

**Terms used:** `simplify`, `use_simplify`, `simplify_subdivision`, `simplify_subdivision_render`, `maximum subdivision`, `simplify_child_particles`, `simplify_child_particles_render`, `child particles`, `texture_limit`, `texture_limit_render`, `texture limit`, `texture size`, `Scene.render`, `Scene.cycles`, `save`, `saved`, `blend-file`, `open_mainfile`, `save_as_mainfile`, `reopen`, `read back`, `background`, `--background`, `--python-expr`, `persist`, `SKIP_SAVE`, and close variants.

**Public pages opened for the answer:**

- https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/simplify.html
- https://docs.blender.org/manual/en/5.0/scene_layout/scene/introduction.html
- https://docs.blender.org/manual/en/5.0/files/blend/open_save.html
- https://docs.blender.org/manual/en/5.0/files/blend/compatibility.html
- https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html
- https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html
- https://docs.blender.org/api/5.0/bpy.types.Scene.html
- https://docs.blender.org/api/5.0/bpy.types.BlendData.html
- https://docs.blender.org/api/5.0/bpy.ops.wm.html
- https://docs.blender.org/api/5.0/info_advanced_blender_as_bpy.html
- https://docs.blender.org/api/5.0/bpy.props.html
- https://docs.blender.org/api/5.0/bpy.types.Property.html
- https://docs.blender.org/api/5.0/bpy_types_enum_items/property_flag_items.html
- https://developer.blender.org/docs/release_notes/5.0/bugfixes/
- https://developer.blender.org/docs/release_notes/5.0/cycles/
- https://developer.blender.org/docs/release_notes/5.0/python_api/
- https://developer.blender.org/docs/release_notes/5.0/rendering/

**Ruled out:** the general particle-system child count is not the global Simplify percentage; the user-preference `gl_texture_limit` is not the per-scene Cycles texture limit; adaptive-subdivision controls such as Cycles `max_subdivisions` are separate from these global Simplify fields; Grease Pencil/Freestyle/Sculpt “simplify” results are unrelated; generic save, compatibility, property-flag, and `SKIP_SAVE` documentation does not directly prove a round trip for these fields; the 5.0 release notes contain a bug-fix title, “OneAPI Embree crash on Simplify toggle,” but no save/read-back statement; and no manual image specific to the Cycles Simplify panel was present in the supplied manual image set.

**Outside the interface:** Observed in Blender 5.0.1 official macOS arm64 and Linux x64 builds: fresh background processes read all seven values after save and close, including distinct per-scene values, inactive subordinate values, and Cycles values while EEVEE was active. Observed 2026-08-29 in the second batch: a fresh background process with no interface, on both machines, also showed the switch changing the rendered picture in controlled subdivision, child-particle and texture cases, so what the switch does is not confined to Blender's own window; and the Linux read of a Mac-authored file was repeated with `--factory-startup`, so that leg no longer rests on a process with add-ons loaded. Reading these values by parsing the `.blend` without Blender remains unestablished.

**Operating system:** Observed on macOS 26.6.2 arm64 and Ubuntu 22.04.4/Linux x64, including official-build files crossed in both directions. Observed again 2026-08-29 on a second rented Linux machine, Ubuntu 22.04.4 on Linux 5.15.0-186-generic: the switch's effect on the picture came out identical to the Mac's, digit for digit, and the official Linux build read the Mac-authored file cleanly under `--factory-startup`. Windows x64, Windows arm64, and Linux distribution packages remain untested.

2026-08-28 — GPT-5.6 Sol (Codex) — What a render demands, Blender 5.0.
