# What a render demands — Blender 5.1 — Architecture Fact 134

Sources consulted: Blender 5.1 Manual, Blender 5.1 Python API, Blender 5.1 release notes, and the supplied Blender 5.1 Cycles files. Date read: 2026-07-30.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

## Question 6

What, if anything, does a saved scene record about limits on subdivision, child particles, and texture size, including any switch governing those limits, and what can be read back when the file is opened from outside Blender's interface?

**LABEL: inferred** for the source-only persistence conclusion; **observed** for the 2026-08-29 exact save/reopen, inactive-value, per-scene, and macOS/Linux hand tests recorded below, and separately for the 2026-08-29 second-batch hand tests, which measured what the Simplify switch does to a render at 128 by 128 in three controlled cases and read a Mac-authored file in the official Linux build with `--factory-startup`. The property names, descriptions, scene access paths, Cycles panel wiring, and the ability to load a `.blend` and run Python without the interface are documented. The supplied sources do not directly say or demonstrate that these particular values are serialized and restored unchanged.

**NOT-COVERED by the supplied sources:** No supplied Manual, API, release-note, or Cycles passage explicitly says that `use_simplify` and the six limit values below are written to a `.blend`, nor does one show their values before save and after a background reopen. The sources also do not document what happens to the subordinate values when Simplify is off. The observed hand test below answers those points for the named official macOS and Linux builds.

## What the scene exposes

For each scene, Blender exposes one switch and separate viewport and final-render values:

| Purpose | Python path | Exact documented description |
|---|---|---|
| Governing switch | `scene.render.use_simplify` | “Enable simplification of scene for quicker preview renders” |
| Viewport maximum subdivision | `scene.render.simplify_subdivision` | “Global maximum subdivision level” |
| Final-render maximum subdivision | `scene.render.simplify_subdivision_render` | “Global maximum subdivision level during rendering” |
| Viewport child particles | `scene.render.simplify_child_particles` | “Global child particles percentage” |
| Final-render child particles | `scene.render.simplify_child_particles_render` | “Global child particles percentage during rendering” |
| Viewport texture limit, Cycles | `scene.cycles.texture_limit` | “Limit texture size used by viewport rendering” |
| Final-render texture limit, Cycles | `scene.cycles.texture_limit_render` | “Limit texture size used by final rendering” |

The API calls the subdivision fields “Global maximum subdivision level” and “Global maximum subdivision level during rendering.” It calls the child-particle fields “Global child particles percentage” and “Global child particles percentage during rendering.” Source: Blender 5.1 Python API, [`RenderSettings.simplify_subdivision`](https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html#bpy.types.RenderSettings.simplify_subdivision), [`RenderSettings.simplify_subdivision_render`](https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html#bpy.types.RenderSettings.simplify_subdivision_render), [`RenderSettings.simplify_child_particles`](https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html#bpy.types.RenderSettings.simplify_child_particles), and [`RenderSettings.simplify_child_particles_render`](https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html#bpy.types.RenderSettings.simplify_child_particles_render), read 2026-07-30.

For the switch, the API says: “Enable simplification of scene for quicker preview renders.” Source: Blender 5.1 Python API, [`RenderSettings.use_simplify`](https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html#bpy.types.RenderSettings.use_simplify), read 2026-07-30.

The Manual describes what the three controls mean: “Maximum number of subdivision by the Subdivision Surface modifiers”; “Show only a subset of all child hairs and particles”; and “Automatically scales textures down so that they are no larger than the values chosen.” It adds that texture limiting “can help reduce computer memory resources when rendering large scenes with huge textures.” Source: Blender 5.1 Manual, [Cycles Render Settings — Simplify](https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/simplify.html), read 2026-07-30.

The supplied Cycles UI file makes the relationship to the switch explicit. In `cycles_5.1/ui.py`, class `CYCLES_RENDER_PT_simplify`, the header draws `rd.use_simplify`. In `CYCLES_RENDER_PT_simplify_viewport` and `CYCLES_RENDER_PT_simplify_render`, it says `layout.active = rd.use_simplify`, then draws the viewport triplet and render triplet respectively. The panel declares `COMPAT_ENGINES = {'CYCLES'}`. Source: `cycles_5.1/ui.py`, classes `CYCLES_RENDER_PT_simplify`, `CYCLES_RENDER_PT_simplify_viewport`, and `CYCLES_RENDER_PT_simplify_render` (around lines 2265–2325), read 2026-07-30.

Texture size is the Cycles-specific member of the triplet. The supplied source defines `texture_limit` as “Viewport Texture Limit” with the description “Limit texture size used by viewport rendering,” and `texture_limit_render` as “Render Texture Limit” with the description “Limit texture size used by final rendering.” Source: `cycles_5.1/properties.py`, `enum_texture_limit` and `CyclesRenderSettings.texture_limit` / `texture_limit_render` (around lines 181–191 and 1009–1020), read 2026-07-30.

## What can be read without the interface

The documented data paths are available from a scene: the API says `Scene.render` has type `RenderSettings` and `Scene.cycles` has type `CyclesRenderSettings`. Sources: Blender 5.1 Python API, [`Scene.render`](https://docs.blender.org/api/5.1/bpy.types.Scene.html#bpy.types.Scene.render) and [`Scene.cycles`](https://docs.blender.org/api/5.1/bpy.types.Scene.html#bpy.types.Scene.cycles), read 2026-07-30. All scenes can be addressed through `bpy.data.scenes`; the API calls this “Scene data-blocks” and calls `BlendDataScenes` a “Collection of scenes.” Sources: Blender 5.1 Python API, [`BlendData.scenes`](https://docs.blender.org/api/5.1/bpy.types.BlendData.html#bpy.types.BlendData.scenes) and [`BlendDataScenes`](https://docs.blender.org/api/5.1/bpy.types.BlendDataScenes.html), read 2026-07-30.

The no-interface route is also documented. The Manual defines `-b, --background` as “Run in background (often used for UI-less rendering).” Source: Blender 5.1 Manual, [Command Line Arguments](https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html), read 2026-07-30. The Python API gives the form `blender myscene.blend --background --python myscript.py` and says the script then has blend-file data “to operate on.” Source: Blender 5.1 Python API, [Tips and Tricks — Use Blender without its User Interface](https://docs.blender.org/api/5.1/info_tips_and_tricks.html), read 2026-07-30.

Putting those documented pieces together, a background Python script should be able to read the seven paths in the table for every scene. That is an inference about actual post-save values. Property existence alone does not establish that the saved values survive reopening.

There is supporting but still general wording: the command-line rendering page says an animation render uses “all the settings saved in the blend-file.” Source: Blender 5.1 Manual, [Rendering From The Command Line](https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html), read 2026-07-30. It does not identify these seven settings, so it is not direct proof of their persistence.

## Observed hand-test result, 2026-08-29

GPT-5.6 Sol in Codex ran the saved terminal procedure `Fact 134 test procedure/make_fact134_files.py` and `read_fact134.py` in exact Blender 5.1.2, hash `ec6e62d40fa9`. It used the Creator's MacBookPro18,3 (Apple M1 Pro, 16 GB, macOS 26.6.2 build 25G83) and Vast.ai instance 49065406 in Poland (Ubuntu 22.04.4 LTS in Docker on Linux 6.8.0-51-generic; AMD Ryzen 5 5600X, 5.76-core quota, 22,786,605,056-byte memory limit; one assigned 12,288 MiB RTX 3060, driver 550.120; no display server). The exact executables were `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender` and `/opt/blender/5.1.2/blender`.

On both systems, separate post-save background processes returned each enabled scene's distinct values and returned the disabled file with `use_simplify: false` and every subordinate value intact. Mac-authored and Linux-authored files crossed between the official builds without a changed target value or property path. Both cross-readers accidentally omitted `--factory-startup`. The Linux reader ran in a clean official installation; on the Mac the omission allowed installed add-ons to initialize. Both reached `FACT134_READBACK_END` and exited 0, but the safety reviewer declined a clean repeat because another version's earlier Mac process had made an authenticated add-on request. The cross-system claim is for the actual processes, not the planned factory-startup variant.

*Changed by the 2026-08-29 citation check: this paragraph recorded only the Mac reader as having omitted the flag. The completed hand-test list for this same run records that the Linux readers were also invoked without it, in clean official installations. Corrected to that record.*

Raw cross-system payload from the Mac process, word for word:

```text
FACT134_READBACK_BEGIN
version 5.1.2
build_platform b'Darwin'
operating_system macOS-26.6.2-arm64-arm-64bit-Mach-O
background True
loaded_path /private/tmp/fact134/cross/linux/5.1/fact134-enabled.blend
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_A", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "use_simplify": true}
cycles_items FACT134_A [('sampling_pattern', 1), ('texture_limit', 4), ('texture_limit_render', 5)]
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_B", "simplify_child_particles": 0.2199999988079071, "simplify_child_particles_render": 0.8100000023841858, "simplify_subdivision": 1, "simplify_subdivision_render": 7, "texture_limit": "256", "texture_limit_render": "4096", "use_simplify": true}
cycles_items FACT134_B [('texture_limit', 2), ('texture_limit_render', 6)]
FACT134_READBACK_END
```

Raw cross-system payload from the Linux process, word for word:

```text
FACT134_READBACK_BEGIN
version 5.1.2
build_platform b'Linux'
operating_system Linux-6.8.0-51-generic-x86_64-with-glibc2.35
background True
loaded_path /opt/fact134/cross/mac/5.1/fact134-enabled.blend
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_A", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "use_simplify": true}
cycles_items FACT134_A [('sampling_pattern', 1), ('texture_limit', 4), ('texture_limit_render', 5)]
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_B", "simplify_child_particles": 0.2199999988079071, "simplify_child_particles_render": 0.8100000023841858, "simplify_subdivision": 1, "simplify_subdivision_render": 7, "texture_limit": "256", "texture_limit_render": "4096", "use_simplify": true}
cycles_items FACT134_B [('texture_limit', 2), ('texture_limit_render', 6)]
FACT134_READBACK_END
```

The disabled-file raw JSON on both machines was:

```text
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_DISABLED", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "use_simplify": false}
```

## Observed hand-test result, second batch, 2026-08-29 — the Simplify switch and the picture

Claude Opus 5 in Claude Code ran `Audits/Blender Research/What a render demands/Fact 134 test procedure/runtime_fact134_older.py` from a terminal in exact Blender 5.1.2, hash `ec6e62d40fa9`. It is the saved 5.2 runtime file with two changes: the EEVEE engine name is taken from the running Blender's own list, and the texture case sets the texture limit alone because `scene.cycles.texture_resolution_render` does not exist in 5.1. The run prints both readings; in 5.1 the engine name came back `BLENDER_EEVEE` and the texture-resolution property came back absent.

Two machines, kept apart. The first was the Creator's MacBookPro18,3: Apple M1 Pro, 16 GB, macOS 26.6.2 build 25G83 on Darwin 25.6.0, Apple's own graphics and no second card, executable `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`. The second was Vast.ai instance 49111279 in South Korea, a Docker container established as one by `/.dockerenv`, by process 1 being `sh` and by `/proc/1/cgroup` reading `0::/`: Ubuntu 22.04.4 LTS on `Linux 5.15.0-186-generic #196-Ubuntu SMP Sat Jun 20 16:09:34 UTC 2026 x86_64`, glibc 2.35; an Intel Xeon E5-2673 v3 at 2.40GHz with 24 visible threads and a cgroup quota of `1152000 100000`, which is 11.52 processor cores; a memory cgroup limit of 22,769,827,840 bytes; one assigned NVIDIA GeForce GTX 1660 with 6,144 MiB on driver 580.159.03; a 25 GB overlay root; and no display server, with `DISPLAY` empty and no `/tmp/.X11-unix`. Blender came from Blender's own download server as the official Linux x64 archive and reported the same build hash as the Mac. Its executable was `/opt/blender/5.1.2/blender`.

The script built three cases at 128 by 128 — a subdivided cube against a render subdivision limit of 0, a thirty-hair emitter with interpolated children against a render child percentage of 0, and a plane carrying a generated 512 by 512 checker against a texture limit of 128 — rendered each twice with the switch off and once with it on, and compared the loaded pixel channels rather than the file bytes.

The Mac printed, word for word:

```text
version 5.1.2
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
version 5.1.2
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

## Observed hand-test result, second batch, 2026-08-29 — the Linux reader with `--factory-startup`

The first batch's cross-system readers were all invoked without `--factory-startup`, which is recorded above. The 2026-08-29 citation check then ran the Mac half of that read with the flag, against the first batch's Linux-authored files still on the Mac, and every target line came back identical. The Linux half was still open. This run closed it on the second rented machine described above.

Four fresh `fact134-enabled.blend` files were authored on the Mac with the shared setup command, into `/private/tmp/fact134-batch2/mac/<version>` so that the first batch's files were untouched, and copied across unchanged; the 5.1 file read `2f03a1bc4395ec7c28cf0b3aae27b7d0a0150b93f3b33deba4e53b002628f68a` on both machines. The Linux installation had no user configuration folder and the command carried `--factory-startup`, so no add-on ran and the output holds nothing but Blender's own lines. The command was:

```text
/opt/blender/5.1.2/blender --background --factory-startup /opt/fact134/cross/mac/5.1/fact134-enabled.blend --python /opt/fact134/procedure/read_fact134.py
```

It came back, word for word:

```text
00:02.763  blend            | Read blend: "/opt/fact134/cross/mac/5.1/fact134-enabled.blend"
FACT134_READBACK_BEGIN
version 5.1.2
build_platform b'Linux'
operating_system Linux-5.15.0-186-generic-x86_64-with-glibc2.35
background True
loaded_path /opt/fact134/cross/mac/5.1/fact134-enabled.blend
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_A", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "use_simplify": true}
cycles_items FACT134_A [('sampling_pattern', 1), ('texture_limit', 4), ('texture_limit_render', 5)]
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_B", "simplify_child_particles": 0.2199999988079071, "simplify_child_particles_render": 0.8100000023841858, "simplify_subdivision": 1, "simplify_subdivision_render": 7, "texture_limit": "256", "texture_limit_render": "4096", "use_simplify": true}
cycles_items FACT134_B [('texture_limit', 2), ('texture_limit_render', 6)]
FACT134_READBACK_END
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)

Blender quit
```

The process exited 0. Every value matches the value the Mac printed before saving, through the same property paths. The two directions of the crossing were done on different days against different Linux machines, so they are two results rather than one.

## Search record

**Files and source sets searched:** the exact question sheet; all text-like files in the supplied Blender 5.1 Manual, Python API, release notes, and Cycles directories. Focused pages/files opened were the Manual’s Cycles Simplify page, command-line arguments page, and command-line rendering page; the API’s `RenderSettings`, `Scene`, `BlendData`, `BlendDataScenes`, `Context`, `PropertyGroup`, `bpy.props`, property-flag items, and Tips and Tricks pages; and Cycles `ui.py` and `properties.py`.

**Search terms used:** `simplify`, `simplification`, `use_simplify`, `simplify_subdivision`, `simplify_subdivision_render`, `simplify_child_particles`, `simplify_child_particles_render`, `texture_limit`, `texture_limit_render`, `texture limit`, `child particles`, `subdivision limit`, `max subdivision`, `stored in .blend`, `saved in .blend`, `saved property`, `saved settings`, `persist`, `background mode`, `--background`, `command line`, `open_mainfile`, `load blend`, `read blend`, `bpy.context.scene`, `bpy.data.scenes`, `scene.render`, `scene.cycles`, and `SKIP_SAVE`.

**Ruled out:** Blender 5.1 release-note matches concerned unrelated Grease Pencil simplification or subdivision bugs, not these render limits or their persistence. Manual and API matches for Grease Pencil Simplify, sculpt/remesh simplification, Freestyle simplification, per-particle-system child settings, and the preferences-level OpenGL texture limit were different features. The property flag `SKIP_SAVE` describes a general flag but did not directly document these properties’ save/reload behavior. No supplied source contained a before-save/after-open example for the seven fields.

**Outside the interface:** Observed in Blender 5.1.2 official macOS arm64 and Linux x64 builds: fresh background processes read the switch and all six subordinate values after save and close, including distinct per-scene values and inactive subordinate values. Observed 2026-08-29 in the second batch: a fresh background process with no interface, on both machines, also showed the switch changing the rendered picture in controlled subdivision, child-particle and texture cases, so what the switch does is not confined to Blender's own window; and the Linux read of a Mac-authored file was repeated with `--factory-startup`, so that leg no longer rests on a process with add-ons loaded. Reading these values by parsing the `.blend` without Blender remains unestablished.

**Operating system:** Observed on macOS 26.6.2 arm64 and Ubuntu 22.04.4/Linux x64, including official-build files crossed in both directions. Observed again 2026-08-29 on a second rented Linux machine, Ubuntu 22.04.4 on Linux 5.15.0-186-generic: the switch's effect on the picture came out identical to the Mac's, digit for digit, and the official Linux build read the Mac-authored file cleanly under `--factory-startup`. Windows x64, Windows arm64, and Linux distribution packages remain untested.

2026-08-28 — GPT-5.6 Sol (Codex) — What a render demands, Blender 5.1.
