# What a render demands — Architecture Fact 134 — operating-system follow-up

Sources consulted (read 2026-07-30):

- Blender 5.2 Manual: [Installing Blender](https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html), [About Blender](https://docs.blender.org/manual/en/5.2/getting_started/about/index.html), [Rendering From The Command Line](https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html), [Command Line Arguments](https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html), the Linux/macOS/Windows command-line launch pages, [Cycles Simplify](https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/simplify.html), [Cycles GPU Rendering](https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html), and [Blend-File Compatibility](https://docs.blender.org/manual/en/5.2/files/blend/compatibility.html).
- Blender 5.2 Python API: [`bpy.types.RenderSettings`](https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html), [`bpy.data`](https://docs.blender.org/api/5.2/bpy.data.html), [`bpy.app`](https://docs.blender.org/api/5.2/bpy.app.html), and [`bpy.ops.wm.open_mainfile`](https://docs.blender.org/api/5.2/bpy.ops.wm.html).
- Blender 5.2 release notes: [Cycles](https://developer.blender.org/docs/release_notes/5.2/cycles/), [Python API](https://developer.blender.org/docs/release_notes/5.2/python_api/), and the 5.2 bug-fix/platform notes.
- Supplied Cycles 5.2 files: `properties.py`, `ui.py`, `engine.py`, and `__init__.py`, cited below by file and place.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

## Operating system — inferred by the sources; observed by hand test on macOS and Linux

Standing line: Observed 2026-08-29 in the official Blender 5.2.1 LTS macOS arm64 and Linux x64 builds: saved Simplify values reopened unchanged on each system and after official-build files crossed in both directions, through the same property paths. Observed again 2026-08-29, in a second batch on a second rented Linux machine: the official Linux build read a Mac-authored file with `--factory-startup`, all nine values unchanged, so the crossing no longer rests only on processes that had add-ons loaded. Windows x64, Windows arm64, Linux distribution packages, unsupported systems, and custom builds remain untested rather than inferred from those results.

The basis for the inference is:

- The Manual says Blender “is available for download on Windows, macOS, and Linux” (Installing Blender, System Requirements), and that “Blender provides a consistent experience across Linux, macOS, and Windows operating systems via OpenGL” (About Blender). *Changed by the 2026-08-29 citation check: the quote stopped before “via OpenGL”, dropping the qualifier the page attaches to the consistency it promises. Restored to the whole sentence.*
- The Manual’s command-line rendering page says `-b` means “Render in the background (without UI)” and that animation rendering uses “all the settings saved in the blend-file.” The command-line arguments page documents `-P, --python <filepath>` as “Run the given Python script file.” These are general Blender options, not an OS-specific alternate scene-property route.
- The API documents `bpy.app.background` as true when Blender runs “without a user interface (started with -b),” and `bpy.data` as the module used for “all Blender/Python access,” with `bpy.data.scenes` available for scene access. `bpy.ops.wm.open_mainfile` is documented as “Open a Blender file.”
- The API documents the relevant built-in scene fields: `use_simplify`, `simplify_subdivision`, `simplify_subdivision_render`, `simplify_child_particles`, and `simplify_child_particles_render` on `bpy.types.RenderSettings`. The supplied Cycles 5.2 `ui.py` places the render and viewport controls on `scene.render` and the texture controls on `scene.cycles` (Simplify panel, `CYCLES_RENDER_PT_simplify_viewport` and `CYCLES_RENDER_PT_simplify_render`). The supplied Cycles 5.2 `properties.py` defines `texture_resolution`, `texture_resolution_render`, `texture_limit`, and `texture_limit_render` in `CyclesRenderSettings` and registers that property group on `bpy.types.Scene` in `CyclesRenderSettings.register()`; those definitions have no platform conditional. The same file’s explicit platform branches are in `CyclesPreferences` for compute-device defaults and availability messages, not for these scene settings. This supports a cross-OS property path, but does not itself prove serialization.

### Where the operating systems differ

The differences documented in the allowed sources concern launching Blender and the rendering device/backend, not the scene-property storage path:

- Linux, macOS, and Windows have different command-line launch instructions and executable paths. Linux can run `./blender`; macOS runs the executable inside `Blender.app`; Windows uses the installed executable/Command Prompt. These are shell/launcher differences only.
- Cycles GPU technologies are OS-dependent. The Manual states that GPU technologies vary “depending on the particular GPU manufacturer and operating system.” CUDA, OptiX, HIP, and oneAPI are documented for Windows/Linux, while Metal is documented for Apple Silicon macOS. GPU denoising support also differs between Linux and Windows GPU generations. This can change device availability, performance, or whether a GPU render is possible; it does not change the documented scene fields or their Python names.
- A Linux distribution package may omit Cycles GPU-rendering support even though the scene settings remain part of Blender’s data model (Installing on Linux, package-manager warning). macOS 5.0+ supports Apple silicon only, and Windows supports x64 and arm64. These are build/architecture qualifications, not a documented exception for reading the saved scene values.
- Background mode has one explicit read-back qualification: `--disable-depsgraph-on-file-load` can prevent systematic dependency-graph evaluation; the Manual says scripts needing evaluated data must call `context.evaluated_depsgraph_get()`. A script that only reads the saved RNA settings named above is not documented as needing evaluated geometry, but that exact claim is an inference and has not been tested here.

### What is not covered by the sources

No allowed source directly says that each named property is serialized byte-for-byte identically on Windows, macOS, and Linux. No allowed source reports a fresh-process Python read-back of all named values on each OS. No allowed source guarantees the route for unsupported operating systems, third-party builds, a Blender build with Cycles omitted, or scripts that inspect evaluated geometry rather than the saved settings. There is therefore no documentary basis for claiming a tested universal guarantee.

### Observed operating-system test, 2026-08-29

GPT-5.6 Sol in Codex used `make_fact134_files.py` and `read_fact134.py` from `Audits/Blender Research/What a render demands/Fact 134 test procedure/`. The exact applications were Blender 5.2.1 LTS hash `9e2066aef7ef` at `/Applications/Blender.app/Contents/MacOS/Blender` and `/opt/blender/5.2.1/blender`.

The Mac was MacBookPro18,3, Apple M1 Pro, 16 GB, macOS 26.6.2 build 25G83. The Linux machine was Vast.ai instance 49065406 in Poland: Ubuntu 22.04.4 LTS in a Docker container on Linux 6.8.0-51-generic; AMD Ryzen 5 5600X with 12 visible threads and a 5.76-core cgroup quota; 22,786,605,056-byte memory limit; one assigned 12,288 MiB RTX 3060 on driver 550.120; 50 GiB root overlay; no display server. The four official Mac-authored and Linux-authored files were transferred unchanged and opened by the matching-version Blender on the other system.

The same-system and crossed 5.2 files returned both scenes and all nine values unchanged. The raw A record from macOS opening the Linux-authored file was:

```text
build_platform b'Darwin'
operating_system macOS-26.6.2-arm64-arm-64bit-Mach-O
background True
loaded_path /private/tmp/fact134/cross/linux/5.2/fact134-enabled.blend
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_A", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "texture_resolution": 0.5, "texture_resolution_render": 0.25, "use_simplify": true}
```

The raw A record from Linux opening the Mac-authored file was:

```text
build_platform b'Linux'
operating_system Linux-6.8.0-51-generic-x86_64-with-glibc2.35
background True
loaded_path /opt/fact134/cross/mac/5.2/fact134-enabled.blend
{"cycles_present": true, "engine": "CYCLES", "scene": "FACT134_A", "simplify_child_particles": 0.3700000047683716, "simplify_child_particles_render": 0.41999998688697815, "simplify_subdivision": 3, "simplify_subdivision_render": 5, "texture_limit": "1024", "texture_limit_render": "2048", "texture_resolution": 0.5, "texture_resolution_render": 0.25, "use_simplify": true}
```

Scene B likewise returned `1`/`7`, `0.2199999988079071`/`0.8100000023841858`, `256`/`4096`, and `0.75`/`0.125` on both readers. No path changed and neither system lost, reset, clamped, or swapped a value. Procedural caveat: both cross-reader command sets accidentally omitted the planned `--factory-startup`. The Linux installation was clean; on the Mac, installed add-ons initialized and added unrelated output, including one authenticated asset/API request. The target readers reached `FACT134_READBACK_END` and exited 0. A safety review declined a clean repeat. This establishes the actual official-build crossing that ran, not the stricter factory-startup variant.

The rented instance was destroyed immediately afterward. Vast.ai then returned instances `[]`, volumes `[]`, and zero snapshot events for instance 49065406. No pre-existing resource was present before the rental or touched by this work.

### Observed operating-system test, second batch, 2026-08-29 — the Linux reader with `--factory-startup`

The caveat above says the cross-readers omitted `--factory-startup` and that a safety review declined a clean repeat. The 2026-08-29 citation check closed the Mac half of that, running the flag against the first batch's Linux-authored files still on the Mac; every target line came back identical. Claude Opus 5 in Claude Code closed the Linux half from a terminal on a second rented machine.

The machine was Vast.ai instance 49111279 in South Korea. It is a Docker container, not a whole machine, and that was established rather than assumed: `/.dockerenv` is present, process 1 is `sh`, and `/proc/1/cgroup` prints `0::/`. Ubuntu 22.04.4 LTS on `Linux 5.15.0-186-generic #196-Ubuntu SMP Sat Jun 20 16:09:34 UTC 2026 x86_64`, glibc 2.35. Processor `Intel(R) Xeon(R) CPU E5-2673 v3 @ 2.40GHz`, 24 threads visible to the container, `/sys/fs/cgroup/cpu.max` reading `1152000 100000`, which is a quota of 11.52 processor cores. Memory cgroup limit `22769827840` bytes, with the host underneath reporting 33,544,003,584 bytes of memory and 8,589,930,496 bytes of swap. One assigned card, `NVIDIA GeForce GTX 1660, 6144 MiB, driver 580.159.03`. A 25 GB overlay root filesystem. `DISPLAY` was empty and `/tmp/.X11-unix` did not exist, so there was no display server; nothing here needed one. Blender came from Blender's own download server as the official Linux x64 archive, unpacked to `/opt/blender/5.2.1/blender`, and reported hash `9e2066aef7ef` — the same build hash as the Mac's. `/root/.config/blender` did not exist, so it was a clean installation with no user add-ons.

The four `fact134-enabled.blend` files were authored freshly on the Mac into `/private/tmp/fact134-batch2/mac/<version>`, leaving the first batch's files untouched, and copied across unchanged. The 5.2 file read `2a3c60f17287b017cade57126972fc9fbed78b8a6999885602a0f50a94bfb468` on both machines. The command was:

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

The process exited 0, the output holds nothing but Blender's own lines, and all nine values match what the Mac printed before saving, through the same property paths. The other three versions were read the same way in the same sitting and came back the same; those results are written into their own documents.

The two directions of the crossing were therefore closed on different days against different Linux machines, and they are two results, not one. The instance was destroyed as soon as the reads finished. Vast.ai then returned instances `{"instances_found": 0, "instances": []}`, volumes `{"volumes": []}`, the instance itself `{"instances": null}`, and the snapshots address `{"success":false,"msg":"Not found"}`. The same instance and volume readings were taken before the rental and were the same, so nothing pre-existing was present or touched.

### Still not run

Windows x64, Windows arm64, and a Linux distribution package of exact Blender 5.2.1 were not available. They remain open; the official macOS/Linux result does not stand for them.

*Pruned 2026-08-29 by the second batch of hand tests: this list also has to be read against the caveat above, and the `--factory-startup` variant it named is no longer open — both halves have now been run. Nothing was added to this list and the three items above stay, because no Windows machine and no distribution package of 5.2.1 was available to this run either.*

## Search record

The question sheet was read first. Searches were then made only inside the supplied `Blender Sources/` groups:

- Manual 5.2, API 5.2, release notes 5.2, and Cycles 5.2: `operating system`, `platform`, `Windows`, `macOS`, `Linux`, `GNU/Linux`, `Operating System`, `available on`, `not available`, `only supported`, `background mode`, `subdivision`, `child particle`, `particles`, `texture size`, `Texture Limit`, `simplify`, `max subdivision`, `max_child`, `use_simplify`, `render simplification`, and `scene.simplify`.
- Manual 5.2 and API 5.2 file/data/command-line groups: `cross-platform`, `cross platform`, `platform independent`, `all platforms`, `compatible across`, `blend-file`, `blend file`, `read-only`, `bpy.data`, and `Python API`.
- Cycles 5.2 Python files: exact searches for `simplify_subdivision`, `simplify_subdivision_render`, `simplify_child_particles`, `simplify_child_particles_render`, `texture_resolution_render`, `texture_limit_render`, and `use_simplify`; a further `.py` search for `platform`, `operating`, `Windows`, `Linux`, `Darwin`, `macOS`, `sys.platform`, and `bpy.app`.
- API 5.2 focused searches: exact property names in `bpy.types.RenderSettings`, `CyclesRenderSettings`, `bpy.data`, `bpy.app`, and `bpy.ops.wm`; `bpy.app.background`, `bpy.app.build_platform`, `open_mainfile`, and command-line Python options.
- Release notes 5.2 focused searches: `simplify`, `subdivision`, `child particles`, `texture`, `platform`, `operating system`, `Windows`, `Linux`, `macOS`, and `background` in Cycles, Python API, bug-fix, and user-interface notes.

Pages/files opened for the evidence above were the Manual pages for Simplify, adaptive Subdivision, Cycles GPU Rendering, Rendering From The Command Line, Command Line Arguments, command-line index/launch index, Linux/macOS/Windows command-line launch, About Blender, Installing Blender, Linux installation, macOS installation, Windows installation, and Blend-File Compatibility; the API pages for `bpy.types.RenderSettings`, `bpy.data`, `bpy.app`, `bpy.ops.wm`, and the Scene/Cycles references; release-note pages for Cycles and Python API; and supplied Cycles 5.2 `properties.py`, `ui.py`, `engine.py`, and `__init__.py`.

Items ruled out as not answering this standing line were unrelated operating-system hits in UI preferences, keymaps, file-browser operations, color management, crash reporting, and graphics-driver troubleshooting; `PreferencesSystem.gl_texture_limit` (a user preference, not the saved scene render texture limit); Cycles’ adaptive-subdivision `max_subdivisions` panel (distinct from Simplify’s `simplify_subdivision*` fields); unrelated release-note UI/platform bugs; and Cycles shader/kernel/compiled-OSL files, which contain no scene-property persistence or read-back specification. The supplied Cycles 4.5 files were not used because this pass is for Blender 5.2.

**Outside the interface** — Observed 2026-08-29 in the official Blender 5.2.1 LTS macOS arm64 and Linux x64 builds: the read-back recorded above was made by fresh background processes with no graphical interface, through the property paths named here, on each system's own files and on the crossed files. Observed again 2026-08-29 in a second batch: the Linux crossed read was repeated in a fresh background process with `--factory-startup`, on a machine with no display server at all, and came back unchanged. Reading these values by parsing the `.blend` without Blender is not addressed by this follow-up.

*Added by the 2026-08-29 citation check: this document carried its operating-system standing line but not the outside-the-interface one. The line above says only what this document already recorded and adds nothing further.*

Label: inferred for the source-only cross-system expectation; observed for the 2026-08-29 official macOS/Linux hand test, and observed again for the 2026-08-29 second-batch Linux read under `--factory-startup`.

2026-08-28 — GPT-5.6 Luna (Codex) — What a render demands Question 6 operating-system follow-up, Blender 5.2.
