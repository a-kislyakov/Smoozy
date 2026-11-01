Sources consulted: Blender 5.2 Manual, Blender 5.2 Python API, Blender 5.2 release notes, and the supplied Cycles 5.2 files. Date read: 2026-07-30.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

# Question 6 — saved Simplify limits and no-interface read-back (Blender 5.2)

**LABEL: documented** for the Manual, API, and release-note statements about the property names, meanings, native ranges/defaults, access from a currently loaded `.blend`, background execution, and the general saving rule for values assigned to Blender data; **observed** for the Cycles property declarations, ranges/defaults, and UI wiring read in the supplied Cycles files, and separately for the 2026-08-29 terminal hand tests recorded below, including the second-batch Linux read of a Mac-authored file under `--factory-startup`; **inferred** for applying the general saving rule to these particular Cycles values before that test; **not in the sources** for a direct save-close-reopen result or the physical serialization of default-valued fields.

**NOT-COVERED by the supplied sources:** The supplied sources do not report a save/reopen experiment for these exact fields. They do not say whether a field at its default is physically written into the `.blend` or reconstructed from its registered default, and they do not directly prove that the five native `RenderSettings` values survive a save/reopen cycle. They also do not establish render-time effect merely from the existence of the properties. The observed hand tests below answer value behavior and runtime gating on the named official macOS/Linux builds, but not physical byte layout.

## Answer

A scene has one common Simplify switch and two parallel sets of limits: viewport values and final-render values. In Python the common switch and the subdivision/child-particle values are under `Scene.render`; the texture values are Cycles settings under `Scene.cycles`.

| Meaning | Viewport property | Final-render property | Value in the cited sources |
|---|---|---|---|
| Simplify switch | `scene.render.use_simplify` | same switch | Boolean, default `False` |
| Maximum subdivision | `scene.render.simplify_subdivision` | `scene.render.simplify_subdivision_render` | Integer 0–32767; defaults 6 and 0 |
| Child particles | `scene.render.simplify_child_particles` | `scene.render.simplify_child_particles_render` | Float 0–1; defaults 1.0 and 0.0 |
| Texture resolution scale | `scene.cycles.texture_resolution` | `scene.cycles.texture_resolution_render` | Float 0.00001–1; both default 1.0 |
| Texture size cap | `scene.cycles.texture_limit` | `scene.cycles.texture_limit_render` | Enum `OFF`, `128`, `256`, `512`, `1024`, `2048`, `4096`, or `8192`; both default `OFF` |

The Manual describes what these settings mean, not just that fields exist. It says Max Subdivision is the “Maximum number of subdivision by the Subdivision Surface modifiers,” Child Particles will “Show only a subset of all child hairs and particles,” Texture Resolution will “Scale down image textures to reduce memory usage,” and Texture Size Limit “Automatically scales textures down so that they are no larger than the values chosen.” Source: Blender 5.2 Manual, [Simplify](https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/simplify.html), read 2026-07-30.

The exact common-property descriptions are also explicit in the API: `simplify_subdivision_render` is “Global maximum subdivision level during rendering,” `simplify_child_particles_render` is “Global child particles percentage during rendering,” and `use_simplify` is “Enable simplification of scene for quicker preview renders.” Source: Blender 5.2 Python API, [RenderSettings](https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html), read 2026-07-30.

**Observed in the supplied Cycles files:** the implementation defines `texture_resolution_render` with the exact description “Scale factor for texture resolution used by final rendering” and `texture_limit_render` with “Limit texture size used by final rendering.” Its enum descriptions run from “No texture size limit” through “Limit texture size to 8192 pixels.” Source: Cycles 5.2 `properties.py`, `CyclesRenderSettings`, `enum_texture_limit`, and the property declarations around lines 181–189 and 1021–1049, read 2026-07-30.

**Observed in the supplied Cycles files:** the UI file places `use_simplify` in the Simplify panel header, makes both the Viewport and Render subpanels active from `rd.use_simplify`, and displays the four relevant values in each panel. The Render panel uses `simplify_subdivision_render`, `simplify_child_particles_render`, `texture_resolution_render`, and `texture_limit_render`. This shows the UI switch association, but it is not by itself proof of render-time behavior. Source: Cycles 5.2 `ui.py`, classes `CYCLES_RENDER_PT_simplify`, `CYCLES_RENDER_PT_simplify_viewport`, and `CYCLES_RENDER_PT_simplify_render`, around lines 2323–2391, read 2026-07-30.

Texture Resolution is new in 5.2. The release notes say: “In the Simplify panel, there is a new Texture Resolution percentage setting, in addition to the existing Texture Limit.” Source: Blender 5.2 release notes, [Cycles — Simplify](https://developer.blender.org/docs/release_notes/5.2/cycles/#simplify), read 2026-07-30.

## What the sources establish about saving

The native fields are exposed through the scene's `RenderSettings`: the Scene API says `render` has type `RenderSettings`, is read-only as a pointer, and is “never None.” Source: Blender 5.2 Python API, [Scene.render](https://docs.blender.org/api/5.2/bpy.types.Scene.html#bpy.types.Scene.render), read 2026-07-30. The individual values are readable even though the pointer itself is read-only.

The texture controls have a stronger source-backed persistence inference. `CyclesRenderSettings` is a `PropertyGroup`, registered on the scene as `bpy.types.Scene.cycles = PointerProperty(...)` in the supplied Cycles file. The API states exactly: “The values assigned to Blender data are saved to disk but the class definitions are not.” Sources: Cycles 5.2 `properties.py`, `class CyclesRenderSettings` and its `register()` method around lines 384 and 1140–1147; Blender 5.2 Python API, [PropertyGroup](https://docs.blender.org/api/5.2/bpy.types.PropertyGroup.html), read 2026-07-30. Applying that general statement to this supplied `PropertyGroup` makes saving values assigned to the four Cycles texture fields a supported inference, not a direct save/reopen observation. The sources do not say whether untouched defaults occupy bytes in the file.

For `use_simplify`, both subdivision values, and both child-particle values, the API documents type, range/default, and access through the loaded scene, but none of the permitted pages says in equally direct words that these particular native fields are saved to disk. Persistence is a reasonable inference from their being `RenderSettings` on a scene, not a documented or observed result in this source set.

## What can be read after opening the file without Blender's interface

Blender's API says, “Accessing data from the currently loaded blend-file is done with the module bpy.data,” shows `bpy.data.scenes`, and demonstrates reading a render attribute as `bpy.data.scenes[0].render.resolution_percentage`. Source: Blender 5.2 Python API, [Quickstart — Data Access](https://docs.blender.org/api/5.2/info_quickstart.html#data-access), read 2026-07-30.

The no-interface route is documented too: `-b, --background` means “Run in background (often used for UI-less rendering),” and `--python-expr` runs a supplied expression. Source: Blender 5.2 Manual, [Command Line Arguments](https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html), read 2026-07-30. The API scripting page gives the concrete pattern `blender myscene.blend --background --python myscript.py` and says the script then has “some data to operate on.” Source: Blender 5.2 Python API, [Tips and Tricks — Use Blender without its User Interface](https://docs.blender.org/api/5.2/info_tips_and_tricks.html#use-blender-without-its-user-interface), read 2026-07-30.

Accordingly, once Blender 5.2 has loaded the file in background mode, a script can read the five common `RenderSettings` values for every scene. Where `scene.cycles` is not `None`, it can also read the four Cycles texture values:

```python
import bpy

for scene in bpy.data.scenes:
    rd = scene.render
    cy = scene.cycles
    values = {
        "scene": scene.name,
        "use_simplify": rd.use_simplify,
        "viewport_max_subdivision": rd.simplify_subdivision,
        "render_max_subdivision": rd.simplify_subdivision_render,
        "viewport_child_particles": rd.simplify_child_particles,
        "render_child_particles": rd.simplify_child_particles_render,
    }
    if cy is not None:
        values.update({
            "viewport_texture_resolution": cy.texture_resolution,
            "render_texture_resolution": cy.texture_resolution_render,
            "viewport_texture_limit": cy.texture_limit,
            "render_texture_limit": cy.texture_limit_render,
        })
    print(values)
```

This is a documented read path to the loaded properties. It is not a direct observation that arbitrary values written before saving will come back unchanged; that distinction requires the hand test.

## Observed hand-test results, 2026-08-29

GPT-5.6 Sol in Codex ran the saved terminal procedures `Fact 134 test procedure/make_fact134_files.py`, `read_fact134.py`, and `runtime_fact134_5_2.py` in exact Blender 5.2.1 LTS, hash `9e2066aef7ef`. It used the Creator's MacBookPro18,3 (Apple M1 Pro, 16 GB, macOS 26.6.2 build 25G83) and Vast.ai instance 49065406 in Poland (Ubuntu 22.04.4 LTS in Docker on Linux 6.8.0-51-generic; AMD Ryzen 5 5600X, 5.76-core quota, 22,786,605,056-byte memory limit; one assigned 12,288 MiB RTX 3060, driver 550.120; no display server). The exact executables were `/Applications/Blender.app/Contents/MacOS/Blender` and `/opt/blender/5.2.1/blender`.

**Persistence, per-scene separation, and inactive values — passed on both systems.** Each enabled scene returned its own nine-value record unchanged in a separate post-save process. The disabled file returned `use_simplify: false` and every subordinate value unchanged. A file saved with `BLENDER_EEVEE` active returned that engine, `cycles_present: true`, and the saved Cycles values unchanged. Mac-authored and Linux-authored files also crossed between the official builds without a target value or path changing. Both cross-readers accidentally omitted `--factory-startup`. The Linux reader ran in a clean official installation; on the Mac, installed add-ons produced unrelated output and one made an authenticated asset/API request. Both target readers still reached their end marker and exited 0. A safety review declined a clean repeat. The cross-system claim describes those actual processes, not the planned factory-startup variant.

*Changed by the 2026-08-29 citation check: this paragraph recorded only the Mac reader as having omitted the flag, while the operating-system follow-up document beside it, and the completed hand-test list, both record that the Linux readers were also invoked without it, in clean official installations. That was the one disagreement between this version's two findings documents; corrected here to the fuller record.*

Raw crossed-file reader payload from Linux, word for word:

```text
FACT134_READBACK_BEGIN
version 5.2.1 LTS
build_platform b'Linux'
operating_system Linux-6.8.0-51-generic-x86_64-with-glibc2.35
background True
loaded_path /opt/fact134/cross/mac/5.2/fact134-enabled.blend
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_A", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "texture_resolution": 0.5, "texture_resolution_render": 0.25, "use_simplify": true}
cycles_items FACT134_A [('sampling_pattern', 5), ('texture_limit', 4), ('texture_limit_render', 5), ('texture_resolution', 0.5), ('texture_resolution_render', 0.25)]
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_B", "simplify_child_particles": 0.2199999988079071, "simplify_child_particles_render": 0.8100000023841858, "simplify_subdivision": 1, "simplify_subdivision_render": 7, "texture_limit": "256", "texture_limit_render": "4096", "texture_resolution": 0.75, "texture_resolution_render": 0.125, "use_simplify": true}
cycles_items FACT134_B [('texture_limit', 2), ('texture_limit_render', 6), ('texture_resolution', 0.75), ('texture_resolution_render', 0.125)]
FACT134_READBACK_END
```

Raw crossed-file reader payload from macOS, word for word:

```text
FACT134_READBACK_BEGIN
version 5.2.1 LTS
build_platform b'Darwin'
operating_system macOS-26.6.2-arm64-arm-64bit-Mach-O
background True
loaded_path /private/tmp/fact134/cross/linux/5.2/fact134-enabled.blend
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_A", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "texture_resolution": 0.5, "texture_resolution_render": 0.25, "use_simplify": true}
cycles_items FACT134_A [('sampling_pattern', 5), ('texture_limit', 4), ('texture_limit_render', 5), ('texture_resolution', 0.5), ('texture_resolution_render', 0.25)]
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_B", "simplify_child_particles": 0.2199999988079071, "simplify_child_particles_render": 0.8100000023841858, "simplify_subdivision": 1, "simplify_subdivision_render": 7, "texture_limit": "256", "texture_limit_render": "4096", "texture_resolution": 0.75, "texture_resolution_render": 0.125, "use_simplify": true}
cycles_items FACT134_B [('texture_limit', 2), ('texture_limit_render', 6), ('texture_resolution', 0.75), ('texture_resolution_render', 0.125)]
FACT134_READBACK_END
```

The disabled and alternate-engine raw JSON lines on both systems were:

```text
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_DISABLED", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "texture_resolution": 0.5, "texture_resolution_render": 0.25, "use_simplify": false}
{"cycles_present": true, "engine": "BLENDER_EEVEE", "scene": "FACT134_OTHER_ENGINE", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "texture_resolution": 0.5, "texture_resolution_render": 0.25, "use_simplify": true}
```

**Untouched versus explicit defaults — negative result on both systems.** The untouched factory scene and the scene explicitly assigned to the registered defaults did not reopen with identical values or Cycles items. This does not establish their physical byte layout. The raw lines were:

```text
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_UNTOUCHED", "simplify_child_particles": 1.0, "simplify_child_particles_render": 1.0, "simplify_subdivision": 6, "simplify_subdivision_render": 6, "texture_limit": "OFF", "texture_limit_render": "OFF", "texture_resolution": 1.0, "texture_resolution_render": 1.0, "use_simplify": false}
cycles_items FACT134_UNTOUCHED [('sampling_pattern', 5)]
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_EXPLICIT_DEFAULTS", "simplify_child_particles": 1.0, "simplify_child_particles_render": 0.0, "simplify_subdivision": 6, "simplify_subdivision_render": 0, "texture_limit": "OFF", "texture_limit_render": "OFF", "texture_resolution": 1.0, "texture_resolution_render": 1.0, "use_simplify": false}
cycles_items FACT134_EXPLICIT_DEFAULTS [('sampling_pattern', 5), ('texture_limit', 0), ('texture_limit_render', 0), ('texture_resolution', 1.0), ('texture_resolution_render', 1.0)]
```

**Runtime gating — passed on both systems.** `runtime_fact134_5_2.py` created separate subdivision, interpolated child-hair, and generated texture cases, rendered two switch-off controls and one switch-on picture for each, then compared loaded pixel channels rather than file bytes. Both systems printed the same raw measurement lines:

```text
pixel_diff {"case": "subdivision", "changed_channels": 15, "comparison": "off-repeat", "max_abs_diff": 0.003921568393707275, "mean_abs_diff": 8.97575773706194e-07}
pixel_diff {"case": "subdivision", "changed_channels": 6587, "comparison": "off-versus-on", "max_abs_diff": 0.5450980961322784, "mean_abs_diff": 0.044957005156902596}
pixel_diff {"case": "child_particles", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "child_particles", "changed_channels": 10240, "comparison": "off-versus-on", "max_abs_diff": 0.5450980961322784, "mean_abs_diff": 0.03819682392895629}
pixel_diff {"case": "texture", "changed_channels": 0, "comparison": "off-repeat", "max_abs_diff": 0.0, "mean_abs_diff": 0.0}
pixel_diff {"case": "texture", "changed_channels": 49152, "comparison": "off-versus-on", "max_abs_diff": 0.501960813999176, "mean_abs_diff": 0.3757036020519706}
```

The subdivision repeat differed in 15 of 65,536 channels by at most one 8-bit step; the switch comparison changed 6,587 channels by as much as 0.545. The child-particle and texture repeats were exact. The three switch comparisons were therefore material picture changes rather than repeat noise.

## Observed hand-test result, second batch, 2026-08-29 — the Linux reader with `--factory-startup`

The first batch's cross-system readers were all invoked without `--factory-startup`, which is recorded above. The 2026-08-29 citation check then ran the Mac half of that read with the flag, against the first batch's Linux-authored files still on the Mac, and every target line came back identical. The Linux half was still open. Claude Opus 5 in Claude Code closed it from a terminal on a second rented Linux machine.

The machine was Vast.ai instance 49111279 in South Korea, a Docker container established as one by `/.dockerenv`, by process 1 being `sh` and by `/proc/1/cgroup` reading `0::/`: Ubuntu 22.04.4 LTS on `Linux 5.15.0-186-generic #196-Ubuntu SMP Sat Jun 20 16:09:34 UTC 2026 x86_64`, glibc 2.35; an Intel Xeon E5-2673 v3 at 2.40GHz with 24 visible threads and a cgroup quota of `1152000 100000`, which is 11.52 processor cores; a memory cgroup limit of 22,769,827,840 bytes, with the host underneath reporting 33,544,003,584 bytes of memory and 8,589,930,496 bytes of swap; one assigned NVIDIA GeForce GTX 1660 with 6,144 MiB on driver 580.159.03; a 25 GB overlay root; and no display server, with `DISPLAY` empty and no `/tmp/.X11-unix`. Blender came from Blender's own download server as the official Linux x64 archive at `/opt/blender/5.2.1/blender` and reported hash `9e2066aef7ef`, the same as the Mac's `/Applications/Blender.app/Contents/MacOS/Blender`. The instance was destroyed once the reads were done.

Four fresh `fact134-enabled.blend` files were authored on the Mac with the shared setup command, into `/private/tmp/fact134-batch2/mac/<version>` so that the first batch's files were untouched, and copied across unchanged; the 5.2 file read `2a3c60f17287b017cade57126972fc9fbed78b8a6999885602a0f50a94bfb468` on both machines. The Linux installation had no user configuration folder and the command carried `--factory-startup`, so no add-on ran and the output holds nothing but Blender's own lines. The command was:

```text
/opt/blender/5.2.1/blender --background --factory-startup /opt/fact134/cross/mac/5.2/fact134-enabled.blend --python /opt/fact134/procedure/read_fact134.py
```

It came back, word for word:

```text
00:03.572  blend            | Read blend: "/opt/fact134/cross/mac/5.2/fact134-enabled.blend"
FACT134_READBACK_BEGIN
version 5.2.1 LTS
build_platform b'Linux'
operating_system Linux-5.15.0-186-generic-x86_64-with-glibc2.35
background True
loaded_path /opt/fact134/cross/mac/5.2/fact134-enabled.blend
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_A", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "texture_resolution": 0.5, "texture_resolution_render": 0.25, "use_simplify": true}
cycles_items FACT134_A [('sampling_pattern', 5), ('texture_limit', 4), ('texture_limit_render', 5), ('texture_resolution', 0.5), ('texture_resolution_render', 0.25)]
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_B", "simplify_child_particles": 0.2199999988079071, "simplify_child_particles_render": 0.8100000023841858, "simplify_subdivision": 1, "simplify_subdivision_render": 7, "texture_limit": "256", "texture_limit_render": "4096", "texture_resolution": 0.75, "texture_resolution_render": 0.125, "use_simplify": true}
cycles_items FACT134_B [('texture_limit', 2), ('texture_limit_render', 6), ('texture_resolution', 0.75), ('texture_resolution_render', 0.125)]
FACT134_READBACK_END
Blender 5.2.1 LTS (hash 9e2066aef7ef built 2026-08-25 02:12:34)

Blender quit
```

The process exited 0. All nine values match the values the Mac printed before saving, through the same property paths. The two directions of the crossing were done on different days against different Linux machines, so they are two results rather than one.

## Search record

- Question sheet opened: `What a render demands.md`, Question 6 only.
- Manual tree searched recursively. Terms: `simplify`, `simplification`, `subdivision`, `child particles`, `texture size`, `texture limit`, `texture resolution`, `background`, `--background`, `--python-expr`, `blend-file`. Relevant pages opened: `render/cycles/render_settings/simplify.txt` and `advanced/command_line/arguments.txt`. Unrelated hits about Grease Pencil simplification, sculpting, Freestyle, remeshing, texture preferences, particle rendering, geometry nodes, and import/export were ruled out because they are not the scene-level Cycles Simplify limits asked about.
- Python API tree searched recursively. Terms: `use_simplify`, `simplify_subdivision`, `simplify_subdivision_render`, `simplify_child_particles`, `simplify_child_particles_render`, `texture_limit`, `texture_resolution`, `Scene.cycles`, `CyclesRenderSettings`, `PropertyGroup`, `bpy.data.scenes`, `open_mainfile`, `background mode`, `save`, `saved`, `load`, `persist`, `blend-file`. Pages opened: `bpy.types.RenderSettings.txt`, `bpy.types.Scene.txt`, `bpy.types.PropertyGroup.txt`, `bpy.props.txt`, `info_quickstart.txt`, `info_tips_and_tricks.txt`, `info_advanced_blender_as_bpy.txt`, `bpy.ops.wm.txt`, and `change_log.txt`. `PreferencesSystem.gl_texture_limit`, GPU maximum texture size, USD texture handling, unrelated simplify operators/modifiers, and `wm.open_mainfile` were ruled out: they do not answer these scene-level settings or add direct persistence evidence. `open_mainfile` merely documents “Open a Blender file”; loading the filename at background startup is the clearer supported path.
- Release-notes tree searched recursively for the same simplify/subdivision/particle/texture terms. `cycles.md` was opened and provided the 5.2 Texture Resolution addition. The bug-fix hit for Grease Pencil Simplify was ruled out as unrelated. No save/read-back statement for these controls was found.
- Cycles tree searched recursively. Terms: the exact property names above plus `use_simplify`, `PointerProperty`, and `CyclesRenderSettings`. Relevant files/places opened: `properties.py` (`enum_texture_limit`, `CyclesRenderSettings`, texture property declarations, and `register()`); `ui.py` (Simplify panel classes); `version_update.py` was returned by search only for an unrelated ambient-occlusion migration condition and was ruled out. No Cycles file in the supplied set directly demonstrated save-close-reopen behavior or the render-time application of all five kinds of values.

**Outside the interface:** Observed in Blender 5.2.1 official macOS arm64 and Linux x64 builds: fresh background processes read all nine values after save and close, including distinct per-scene values, inactive values, and Cycles values while EEVEE was active. The governing switch also changed the rendered pixels in controlled subdivision, child-particle, and texture cases. Observed 2026-08-29 in the second batch: the Linux read of a Mac-authored file was repeated with `--factory-startup`, so that leg no longer rests on a process with add-ons loaded. Reading these values by parsing the `.blend` without Blender remains unestablished.

**Operating system:** Observed on macOS 26.6.2 arm64 and Ubuntu 22.04.4/Linux x64, including official-build files crossed in both directions. Observed again 2026-08-29 on a second rented Linux machine, Ubuntu 22.04.4 on Linux 5.15.0-186-generic: the official Linux build read the Mac-authored file cleanly under `--factory-startup`, all nine values unchanged. Windows x64, Windows arm64, and Linux distribution packages remain untested.

2026-08-28 — GPT-5.6 Sol (Codex) — What a render demands, Blender 5.2.
