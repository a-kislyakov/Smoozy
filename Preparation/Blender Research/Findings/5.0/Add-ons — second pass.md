# Add-ons — Blender 5.0 — second pass (question 4 only)

**Sources consulted, and when.** Blender's manual for 5.0 (`Blender Sources/manual_5.0`, including its `_images` folder), Blender's Python API documentation for 5.0 (`Blender Sources/api_5.0`), Blender's release notes for 5.0 (`Blender Sources/release_notes_5.0`), and Cycles' own files (`Blender Sources/cycles_5.0`). All four were taken on 2026-07-30, and that is my date read for everything quoted from them. I also opened one file in `Blender Sources/requirements/` (`requirements_2026-08-01.txt`), taken 2026-08-01; it is a single page that carries no version in its address, and I only use it where it names Blender 5.0 in so many words.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I worked alone. No helper agents were started, so there is nothing to vouch for beyond my own reading.

---

## 4. What does an add-on record about the kinds of computer it is meant for, what happens on one it does not name, and does an add-on set up on one computer still work when copied to a computer of a different kind?

### Settled by hand test — what happens on a kind of computer an add-on does not name, and which bundled modules land on disk

**Label: observed.** Run on the Creator's own MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics) on 2026-08-04, with Blender 5.0.1. Every run started Blender from a terminal with no window, and pointed it at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, so the Creator's own Blender settings were never touched. This is a macOS result. Linux was tested afterwards on a rented Linux machine, and that run is written up in its own block below; the two are never merged. Windows stays untested, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**A package naming only another kind of computer is refused, and nothing is installed.** Two test add-ons were built, identical but for their manifests: one with `platforms = ["windows-x64"]`, one with `platforms = ["macos-x64"]`. Both built without complaint. Installing the first:

`blender --command extension install-file -r user_default -e smoozy_win_only-1.0.0.zip`

printed:

```
ERROR smoozy_win_only: This platform (macos-arm64) isn't one of (windows-x64)
Package should have been installed but not found: smoozy_win_only
```

and the second:

```
ERROR smoozy_macx64_only: This platform (macos-arm64) isn't one of (macos-x64)
Package should have been installed but not found: smoozy_macx64_only
```

**Both runs exited 0** despite the error. `blender --command extension list` afterwards showed the User Default repository with nothing in it, and the switched-on list held only the eight built-in add-ons. So `macos-x64` is refused on an Apple Silicon Mac exactly as `windows-x64` is.

**The same package, placed straight into the extensions folder, is listed but cannot be switched on.** The Windows-only add-on's folder — its `__init__.py` and its manifest — was copied into `<settings folder>/extensions/user_default/smoozy_win_only/`. Then:

`blender --command extension list`

```
Repository: "extensions.blender.org" (id=blender_org)
Repository: "User Default" (id=user_default)
  smoozy_win_only [installed]: "Smoozy Windows Only", A test add-on naming one platform
Repository: "System" (id=system)
```

and switching it on:

`blender --background --python-expr "import bpy; print('ENABLE', bpy.ops.preferences.addon_enable(module='bl_ext.user_default.smoozy_win_only')); print('MODULES', sorted(a.module for a in bpy.context.preferences.addons))"`

printed:

```
RuntimeError: Extension bl_ext.user_default.smoozy_win_only is incompatible (This platform (macos-arm64) isn't one of (windows-x64))
```

twice — once raised inside `addon_utils.py` and once as the operator's own error — followed by:

```
Error: Extension bl_ext.user_default.smoozy_win_only is incompatible (This platform (macos-arm64) isn't one of (windows-x64))
```

Exit status 0. So the check happens when Blender scans the folder as well as when it installs.

**A repository built from a folder is filtered by Blender itself.** A listing was generated from a folder holding both the plain test package and the Windows-only one:

`blender --command extension server-generate --repo-dir=<packages folder>`

printed `found 2 packages.` and wrote an `index.json` naming both, the Windows-only one carrying `"platforms": ["windows-x64"]`. That folder was added as a repository with a `file://` address and synced. `blender --command extension list` then showed:

```
Repository: "Smoozy Local" (id=smoozy_local)
  smoozy_test: "Smoozy Test", A tiny test add-on
```

— only the compatible one. `blender --online-mode --command extension install smoozy_win_only -e` printed:

```
Package "smoozy_win_only" not found in remote repositories!
```

exit status 1, while `blender --online-mode --command extension install smoozy_test -e` printed `STATUS Installed "smoozy_test"`, exit status 0.

**Only the matching bundled module is unpacked.** A test add-on was built carrying three bundled Python modules of the same library, one each for macOS on Apple Silicon, Linux and Windows, named in its manifest as:

```
wheels = [
  "./wheels/pillow-12.3.0-cp311-cp311-macosx_11_0_arm64.whl",
  "./wheels/pillow-12.3.0-cp311-cp311-manylinux_2_27_x86_64.manylinux_2_28_x86_64.whl",
  "./wheels/pillow-12.3.0-cp311-cp311-win_amd64.whl",
]
```

After installing it, the add-on's own folder held **all three** files, unopened, in a `wheels` folder. But the unpacked code sat somewhere else and was for this machine only — under `<settings folder>/extensions/.local/lib/python3.11/site-packages/`, whose compiled parts were named:

```
_imagingmath.cp311-darwin.so
_imaging.cp311-darwin.so
_webp.cp311-darwin.so
_imagingcms.cp311-darwin.so
_imagingft.cp311-darwin.so
_imagingmorph.cp311-darwin.so
_imagingtk.cp311-darwin.so
_avif.cp311-darwin.so
```

with a `.dylibs` folder beside them. No Windows or Linux compiled file was unpacked anywhere. The module could then be used: `blender --background --python-expr "import PIL; print('PIL', PIL.__file__, PIL.__version__)"` printed the path above and `12.3.0`.

**Building one package per kind of computer needs the manifest to name them.** `blender --command extension build --split-platforms` on the manifest above, which named no platforms, printed:

```
FATAL_ERROR: Error in arguments "--split-platforms" with a manifest that does not declare "platforms"
```

exit status 1. With `platforms = ["windows-x64", "macos-arm64", "linux-x64"]` added, the same command printed:

```
building: smoozy_wheels-1.0.0-windows_x64.zip
complete
building: smoozy_wheels-1.0.0-macos_arm64.zip
complete
building: smoozy_wheels-1.0.0-linux_x64.zip
complete
```

**Outside the interface.** Settled: the platform check, the message it gives and the folder filtering all happen with Blender driven from outside its window, and the message goes to the terminal rather than to a window. The exit status does not report the refusal — it stays 0.

**Operating system.** Settled from the macOS side only. What a **Linux** machine does with a package naming another kind, and what carrying an installed add-on across to a machine of a different kind does, were both run afterwards on a rented Linux machine and have their own block under this question; the two machines' results are never merged. Windows stays untried: no Windows machine has run it.

### Settled by hand test on the rented Linux machine — what happens on a kind of computer an add-on does not name, which bundled modules land on disk, and what a compiled part for the wrong machine does

**Label: observed.** Run on the rented Linux machine, on 2026-08-06, with Blender 5.0.1 from `/opt/blender/blender-5.0.1-linux-x64/blender`. Every run started Blender from a terminal **with no window**, with `DISPLAY` and `XAUTHORITY` unset so that the display could add nothing to the output. Every run pointed Blender at a fresh settings folder of its own through the `BLENDER_USER_RESOURCES` environment variable, and that folder was made before it was used. This is a Linux result. It does not replace the macOS result recorded above, and the two are never merged. Windows stays untested. The machine is a rented container sharing someone else's hardware, so it has fewer cores than the chip really has and its system settings are not entirely its own; nothing below turns on either. In the commands below the program is shortened to `blender` and long paths to their file names.

**A package naming only another kind of computer is refused, and this machine can pose the case the Mac could not.** Four test add-ons were built, identical but for their manifests: `platforms = ["windows-x64"]`, `["macos-x64"]`, `["macos-arm64"]` and `["linux-x64"]`. All four built without complaint. Installing each with `blender --command extension install-file -r user_default -e <zip>` printed:

```
ERROR smoozy_win_only: This platform (linux-x64) isn't one of (windows-x64)
Package should have been installed but not found: smoozy_win_only
```

```
ERROR smoozy_macx64_only: This platform (linux-x64) isn't one of (macos-x64)
Package should have been installed but not found: smoozy_macx64_only
```

```
ERROR smoozy_macarm_only: This platform (linux-x64) isn't one of (macos-arm64)
Package should have been installed but not found: smoozy_macarm_only
```

```
STATUS Installed "smoozy_linux_only"
SMOOZY TEST ADDON REGISTERED
SMOOZY TEST ADDON UNREGISTERED
```

**All four runs exited 0**, the refusals included. Afterwards `blender --command extension list` showed only `smoozy_linux_only [installed]` under User Default, and the switched-on list held that module and the eight built-in ones.

So the sentence is the same on both machines and names the machine it is running on: `macos-arm64` on the Mac, `linux-x64` here. **An add-on built for a Mac is refused on Linux exactly as an add-on built for Windows was refused on the Mac**, and an add-on built for Linux installs here without complaint. The second line of the refusal is the same in this version on both machines.

**The same package's folder placed straight into the extensions directory** is listed but cannot be switched on:

```
Repository: "User Default" (id=user_default)
  smoozy_win_only [installed]: "Smoozy Windows Only", A test add-on naming one platform
```

and switching it on printed `RuntimeError: Extension bl_ext.user_default.smoozy_win_only is incompatible (This platform (linux-x64) isn't one of (windows-x64))` twice — once raised inside `addon_utils.py` and once as the operator's own error — followed by the same sentence as `Error:`. Exit status 0. The same as the macOS run.

**A repository built from a plain folder is filtered by Blender itself.** `server-generate` on a folder holding both the plain package and the Windows-only one printed `found 2 packages.` and wrote an `index.json` naming both, the Windows-only one carrying `"platforms": ["windows-x64"]`. After adding that folder as a repository by `file://` address and syncing, `blender --command extension list` showed only:

```
Repository: "Smoozy Local" (id=smoozy_local)
  smoozy_test: "Smoozy Test", A tiny test add-on
```

and `blender --online-mode --command extension install smoozy_win_only -e` printed `Package "smoozy_win_only" not found in remote repositories!`, exit status 1. Installing `smoozy_test` printed `STATUS Installed "smoozy_test"`, exit status 0. The same as the macOS run, with this machine's own platform doing the filtering.

**Only the matching bundled module is unpacked, and here that means the Linux one.** A test add-on was built carrying three bundled Python modules of the same library, one each for macOS on Apple Silicon, Linux and Windows, named in its manifest as `cp311` files. After installing it, the add-on's own folder held **all three** files, unopened, in a `wheels` folder:

```
pillow-12.3.0-cp311-cp311-macosx_11_0_arm64.whl
pillow-12.3.0-cp311-cp311-manylinux_2_27_x86_64.manylinux_2_28_x86_64.whl
pillow-12.3.0-cp311-cp311-win_amd64.whl
```

The unpacked code sat somewhere else and was for this machine only — under `<settings folder>/extensions/.local/lib/python3.11/site-packages/`, whose compiled parts were named:

```
_avif.cpython-311-x86_64-linux-gnu.so
_imaging.cpython-311-x86_64-linux-gnu.so
_imagingcms.cpython-311-x86_64-linux-gnu.so
_imagingft.cpython-311-x86_64-linux-gnu.so
_imagingmath.cpython-311-x86_64-linux-gnu.so
_imagingmorph.cpython-311-x86_64-linux-gnu.so
_imagingtk.cpython-311-x86_64-linux-gnu.so
_webp.cpython-311-x86_64-linux-gnu.so
```

with a `pillow.libs` folder beside them. No macOS or Windows compiled file appeared anywhere. The module could then be used with no window: `import PIL` printed the path above and `12.3.0`, and `from PIL import Image; print('IMAGE OK', Image.new('RGB',(2,2)).size)` printed `IMAGE OK (2, 2)`. This is the exact mirror of the macOS run, which unpacked only the macOS files.

**Building one package per kind of computer** still refuses a manifest that names no platforms: `FATAL_ERROR: Error in arguments "--split-platforms" with a manifest that does not declare "platforms"`, exit status 1.

**The new part: an add-on whose only compiled part is built for another kind of computer.** Two more packages were built, one whose bundled-module list names **only** the macOS file, one whose list names **only** the Windows file. Neither manifest names any `platforms`, so nothing stops either being installed. On this Linux machine:

```
STATUS Installed "smoozy_macwheel"
SMOOZY TEST ADDON REGISTERED
SMOOZY TEST ADDON UNREGISTERED
```

Exit status 0, **no warning of any kind**. The package is listed as installed, and its module is in the switched-on list:

```
MODULES ['bl_ext.user_default.smoozy_macwheel', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

Listing the extensions folder shows the wheel still sitting unopened in the add-on's own folder, and **no `.local` folder at all** — nothing was unpacked:

```
<settings folder>/extensions/user_default/smoozy_macwheel/__init__.py
<settings folder>/extensions/user_default/smoozy_macwheel/blender_manifest.toml
<settings folder>/extensions/user_default/smoozy_macwheel/wheels
<settings folder>/extensions/user_default/smoozy_macwheel/wheels/pillow-12.3.0-cp311-cp311-macosx_11_0_arm64.whl
```

And using it fails:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
ModuleNotFoundError: No module named 'PIL'
```

**Exit status 0.** The Windows-only package behaved the same way in every respect. So an add-on carrying a compiled part built for another kind of computer **installs, switches on, and is simply missing what it needs**, with nothing said at install time and nothing in the exit status.

**The new part: carrying an installed, set-up add-on to a computer of a different kind.** The whole settings folder holding the installed three-module add-on was packed on this Linux machine, copied to the Creator's MacBook Pro (M1 Pro chip, macOS 26.5.2), and that Mac's Blender 5.0.1 was pointed at it. The Mac listed it as installed:

```
Repository: "User Default" (id=user_default)
  smoozy_wheels [installed]: "Smoozy Wheels", A test add-on carrying bundled Python modules
```

and had it switched on:

```
VERSION 5.0.1
MODULES ['bl_ext.user_default.smoozy_wheels', 'bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

`import PIL` succeeded and printed the version, `12.3.0`. But using it failed:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
  File "<settings folder>/extensions/.local/lib/python3.11/site-packages/PIL/Image.py", line 95, in <module>
    from . import _imaging as core
ImportError: cannot import name '_imaging' from 'PIL' (<settings folder>/extensions/.local/lib/python3.11/site-packages/PIL/__init__.py)
```

**Exit status 0.** Looking on disk, the unpacked compiled files were still the Linux ones, `_imaging.cpython-311-x86_64-linux-gnu.so` and the rest; nothing had gone back to the add-on's own folder to unpack the macOS file that is sitting there. So the install, the switch-on and the plain Python travel; the compiled part does not, and nothing announces it until the add-on tries to use it.

This one test was run on both machines by design — packed on Linux, opened on macOS — and is written here as one crossing rather than as two results.

**Outside the interface.** Settled: the platform check, the message it gives, the folder filtering, which bundled module is unpacked, and everything about the wrong-machine compiled part all happen with Blender driven from outside its window, and the message goes to the terminal. The exit status never reports any of it — it stays 0.

**Operating system.** Now settled for Linux as well as macOS, on the two machines described. Both machines were run and both agreed on the shape of the answer, each naming its own kind: `macos-arm64` there, `linux-x64` here. A package built for a Mac is refused on Linux, which the Mac could not show. Carrying an installed add-on from Linux to macOS was run once, across the two machines, and is recorded above. Windows stays untried: no Windows machine has run it.

### Settled by hand test on the rented Linux machine — installing a package for another kind of computer at the window, and what Cycles' own panel offers

**Label: observed.** Run on the rented Linux machine on 2026-08-06, with Blender 5.0.1, at Blender's own window on the machine's display, photographed with `bpy.ops.screen.screenshot`. Fresh settings folders, preferences saved once beforehand.

**Installing a Windows-only package through the window.** A script opened the Preferences editor at its Get Extensions section, read the install operator's own list of arguments so that none was guessed —

```
OPERATOR PROPS ['directory', 'enable_on_install', 'filepath', 'files', 'filter_glob', 'overwrite', 'repo', 'rna_type', 'target', 'url']
```

— and then called it with the package's path and the user repository. It printed:

```
Package should have been installed but not found: smoozy_win_only
INSTALL RESULT {'FINISHED'}
MODULES ['bl_pkg', 'cycles', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

So the operator reported **finished** while refusing the package. Nothing was installed and nothing was switched on. The picture, taken with the Add-ons section then showing, holds the thirteen add-ons that ship with this Blender and **no "Smoozy Windows Only" anywhere in the list**. That settles a gap the Creator's macOS run left open: the refused package does not appear in the Add-ons list afterwards.

**What Cycles' own panel offers on a machine with an NVIDIA card.** The picture of the opened Cycles row in the Add-ons preferences shows a section headed **Preferences** holding one row of buttons: **`None`**, **`CUDA`**, **`OptiX`**, **`HIP`**, **`oneAPI`**, with `None` selected. The same row appears in the System preferences under a heading **Cycles Render Devices**. On the Creator's Mac the same setting reads `METAL`.

**And the buttons are not the property's own list of values.** Asked from Python on this machine, with a window open and without, the offered list came back empty:

```
DEVICE NONE
OFFERED []
```

while `refresh_devices()` then filled the devices collection with this machine's own hardware:

```
DEVICES [('NVIDIA GeForce RTX 4090', 'CUDA', True), ('AMD Ryzen Threadripper PRO 3975WX 32-Cores', 'CPU', False), ('NVIDIA GeForce RTX 4090', 'OPTIX', True)]
```

So the row of buttons is drawn by the add-on's own interface code, not from the setting's list of values, and what a script can read about it is not what a person sees.

**Outside the interface.** The install refusal at the window matches the one from outside the window under question 4 above, word for word in its first line. The compute-device buttons are only visible at the window; from outside it the offered list is empty.

**Operating system.** Settled for Linux in this version, at the window. The Creator's macOS run covered Blender 5.2 only. The two machines offer different compute devices, which is the machine's graphics rather than the operating system. Windows stays untested.


**A note on how I read the question.** "Kinds of computer" I take to mean operating system together with processor family — that is the distinction Blender's own vocabulary here draws (`windows-x64`, `macos-arm64`, and so on). The other reading, "kinds of computer" meaning graphics hardware or amount of memory, I did not pursue; nothing in the extension documentation records anything of that sort, and I say so below. "An add-on set up on one computer" I read as an add-on that has been installed onto a machine, whose installed files are then copied to a different machine. The other reading — an add-on whose *settings* were configured and are then carried across — is not covered by anything I found, and I flag it as unanswered.

The question has three parts. They have very different answers, so I take them one at a time.

### 4a. What an add-on records

**LABEL: documented.**

An extension add-on records the kinds of computer it is meant for in one place: an optional `platforms` entry in its `blender_manifest.toml` manifest. There are exactly five names it may use.

The manual's manifest reference says:

> "platforms:
>
> List of supported platforms. If omitted, the extension will be available in all operating systems.
> The available options are
> [“windows-x64”, “windows-arm64”, “macos-x64”, “macos-arm64”, “linux-x64”]"

And in the example manifest on the same page:

> "# # Optional: list of supported platforms. If omitted, the extension will be available in all operating systems.
> # platforms = ["windows-x64", "macos-arm64", "linux-x64"]
> # # Other supported platforms: "windows-arm64", "macos-x64""

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/getting_started.html
- Date read: 2026-07-30

`platforms` is listed under "Optional values", not "Required values", on that same page. The page also says of the manifest as a whole:

> "All the values present in the manifest file must be filled
> (i.e., cannot be empty, nor text "", nor list []).
>
> If you don’t want to set one of the optional values just exclude it from the manifest altogether."

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/getting_started.html
- Date read: 2026-07-30

So an add-on either names a subset of those five, or says nothing at all. It may not write an empty list.

There is a second, indirect record: the `wheels` entry. Bundled Python wheels carry the platform in their own file names, and the manual instructs authors to bundle one per platform where the wheel contains compiled code:

> "For wheels that contain binary compiled files, wheels for all supported platforms should be included"

and the worked example lists three:

> "wheels = [
> "./wheels/pillow-10.3.0-cp311-cp311-macosx_11_0_arm64.whl",
> "./wheels/pillow-10.3.0-cp311-cp311-manylinux_2_28_x86_64.whl",
> "./wheels/pillow-10.3.0-cp311-cp311-win_amd64.whl",
> ]"

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/python_wheels.html
- Date read: 2026-07-30

The wheel names are not Blender's vocabulary — the manual points at Python's own convention for them:

> "Wheels filenames must match Python’s binary distribution specification:
> see docs.
> Wheels downloaded from Python’s package index will follow this convention."

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/python_wheels.html
- Date read: 2026-07-30

**A legacy add-on records nothing of the kind.** Add-ons that have not been converted to the extension format carry a `bl_info` block instead of a manifest. The manual describes what that block holds:

> "bl_info
>
> is a dictionary containing add-on metadata such as the title,
> version and author to be displayed in the Preferences add-on list.
> It also specifies the minimum Blender version required to run the script;
> older versions won’t display the add-on in the list."

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/scripting/addon_tutorial.html
- Date read: 2026-07-30

The API documentation says the same in different words:

> "The only difference between add-ons and built-in Python modules is that add-ons must contain a bl_info variable
> which Blender uses to read metadata such as name, author, category and project link."

- Source: the Python API documentation, https://docs.blender.org/api/5.0/info_overview.html
- Date read: 2026-07-30

Neither mentions any operating-system or processor field, and I found no page anywhere in the manual or the API documentation that gives `bl_info` such a field (search record below).

**LABEL: observed** — for the following. Cycles ships as an add-on and its own files are in the folder I was given. Its `bl_info` block is complete and short, and it names no platform:

```
bl_info = {
    "name": "Cycles Render Engine",
    "author": "",
    "blender": (2, 80, 0),
    "description": "Cycles renderer integration",
    "warning": "",
    "doc_url": "https://docs.blender.org/manual/en/latest/render/cycles/",
    "tracker_url": "",
    "support": 'OFFICIAL',
    "category": "Render"}
```

- Source file: `Blender Sources/cycles_5.0/__init__.py`, lines 7–16 (the whole of the `bl_info` assignment).
- Date read: 2026-07-30

There is no `blender_manifest.toml` anywhere in `Blender Sources/cycles_5.0`, so Cycles as supplied records no `platforms` value either. I checked by listing the whole folder tree.

**Not covered (4a).** Nothing here says whether the extensions website adds a platform record of its own beyond the manifest, nor whether Blender stores any platform note about an add-on after installing it. Nothing records graphics hardware, memory, processor count, or driver requirements — the five names cover operating system and processor family only. The manifest may also carry `blender_version_min` and `blender_version_max`, which are about which Blender it is meant for, not which computer; I have not treated those as part of this answer.

### 4b. What happens on a computer the add-on does not name

**LABEL: not in the sources**, for the direct question — what Blender does when an add-on that names platforms meets a machine outside that list. I searched hard for this and found no statement of it. What I did find is three things that surround the question without answering it, and one thing that answers a nearby case outright. I give the nearby case first because it is solid.

**The omitted case is documented.** If `platforms` is left out altogether, the manual states the outcome plainly:

> "If omitted, the extension will be available in all operating systems."

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/getting_started.html
- Date read: 2026-07-30

The same sentence appears again on the repository page, about the same field passed as part of a download link:

> "platforms:
>
> Comma-separated list of supported platforms.
> If omitted, the extension will be available in all operating systems."

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/creating_repository/static_repository.html
- Date read: 2026-07-30

**What surrounds the unnamed case, without settling it.**

First, when Blender asks a repository what it has, it tells the repository which platform it is on, and the manual says this is so the answer is filtered:

> "When Blender fetches the extensions listing it passes the following arguments to make sure only
> compatible extensions are listed:
>
> platform
>
> blender_version"

with the worked query:

> "https://extensions.blender.org/api/v1/extensions/?blender_version=4.2.0&platform=linux-x64"

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/creating_repository/dynamic_repository.html
- Date read: 2026-07-30

**LABEL: inferred**, for what that implies. The step a reader has to take: "only compatible extensions are listed" plus "Blender passes the platform" gives that an add-on naming platforms which exclude the current machine does not appear in the list Blender fetches from a *dynamic* repository. That is the repository's filtering, not Blender's — the page says the server serves "only the latest version of the extensions which are compatible with the query parameters". It says nothing about what happens if such an add-on reaches the machine by some other route.

Second, the build tool will not produce a package for a platform the manifest does not name. The manual's example has a Linux wheel bundled but no Linux entry in `platforms`, and states the outcome:

> "Even though there is a Linux-only wheel present, no Linux zip file is generated.
> This happens because the platforms field only has Mac and Windows."

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/python_wheels.html
- Date read: 2026-07-30

That is about building, not about installing or running.

Third, the manual's advice for the case where an add-on carries compiled files that do not suit the machine is to look at the console. It describes the symptom, not Blender's policy:

> "It is possible the library is incompatible with your Blender installation (attempting to load a library built for a
> different version of Python, or loading a 32-bit library on a 64-bit system).
>
> If the add-on contains .pyd or .so files, check that the distribution is compatible with your operating
> system."

and, on Windows specifically:

> "If Python is raising errors or you have an add-on that just fails when enabled with an error – e.g: ... is not a
> valid Win32 application. – this may be caused by some inconsistency in the Python libraries."

- Source: the manual, https://docs.blender.org/manual/en/5.0/troubleshooting/python.html
- Date read: 2026-07-30

The general advice for an add-on that fails to come to life appears three times in the manual, always the same:

> "If the Add-on does not activate when enabled,
> check the Console window
> for any errors that may have occurred."

- Source: the manual, at https://docs.blender.org/manual/en/5.0/editors/preferences/extensions.html , https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html and https://docs.blender.org/manual/en/5.0/advanced/extensions/addons.html
- Date read: 2026-07-30

**One documented fact worth putting beside the list of five names.** One of the five, `macos-x64`, names a computer that Blender 5.0 does not run on at all. The requirements page states:

> "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later."

- Source: Blender's requirements page, file `Blender Sources/requirements/requirements_2026-08-01.txt`, in the "macOS" section. This page carries no version in its address, so I cite it only because it names Blender 5.0 outright.
- Date read: 2026-08-01

I draw no conclusion from this about how Blender treats the `macos-x64` name; the sources do not say.

**Where I looked, for 4b.** This is the part of the answer that has to carry its own weight, so here is the whole of it.

Searches run over the complete text of `manual_5.0`, `api_5.0` and `release_notes_5.0` at once:

- `platform` (case-insensitive, whole word and as a fragment). Every hit in the manual falls in one of these files, and I read all of them in full: `advanced/extensions/getting_started.txt`, `advanced/extensions/python_wheels.txt`, `advanced/extensions/index.txt`, `advanced/extensions/tags.txt`, `advanced/extensions/addons.txt`, `advanced/extensions/version_number_guidelines.txt`, `advanced/extensions/licenses.txt`, `advanced/extensions/creating_repository/index.txt`, `advanced/extensions/creating_repository/static_repository.txt`, `advanced/extensions/creating_repository/dynamic_repository.txt`, `advanced/blender_directory_layout.txt`, `advanced/command_line/extension_arguments.txt`, `editors/preferences/extensions.txt`, `editors/preferences/system.txt`, `troubleshooting/python.txt`, `getting_started/installing/windows.txt`, `getting_started/configuration/hardware.txt`, `getting_started/about/index.txt`, `getting_started/about/community.txt`, `contribute/translate_manual/add_language.txt`, `addons/import_export/scene_gltf2.txt`, `addons/3d_view/vr_scene_inspection.txt`, `render/output/properties/output.txt`. In the API documentation the hits are `bpy.app.build_platform`, the `gpu.platform` module, two passing mentions in `info_quickstart` and `info_tips_and_tricks`, a code example in `bpy.utils` that prints the machine (`import platform`, `platform.platform()`), and one sentence about Apple platforms in `gpu`; I read all of them, and none concerns extension platform declarations. (*Citation check, 2026-08-05: this used to say "the only hits are" and list the first four. `api_5.0/bpy.utils.txt` and `api_5.0/gpu.txt` carry the word too. The conclusion is unaffected; the enumeration was not complete.*) In the release notes the only hits are two section headings, "## Platforms" in `user_interface.md` and "## Platforms & Builds: 6" in `bugfixes.md`; I read both sections and neither concerns extensions.
- `platforms` specifically — same set, no additional pages.
- `windows-x64`, `windows-arm64`, `macos-x64`, `macos-arm64`, `linux-x64`, `arm64`, `aarch64`, `x86_64`, `amd64` — the only pages carrying any of these are `advanced/extensions/getting_started.txt`, `advanced/extensions/python_wheels.txt`, `getting_started/installing/windows.txt`, and one release-note line about a build error on Linux arm64. Nothing about behaviour on an unnamed platform.
- `operating system` — 28 hits; the only ones inside extension documentation are the two "available in all operating systems" sentences already quoted, plus the troubleshooting line "check that the distribution is compatible with your operating system".
- `incompatible`, `not compatible`, `compatible`, `unsupported` — read every hit in `advanced/extensions/`, `editors/preferences/` and the release notes. None describes a platform mismatch.
- `will not be listed`, `not be shown`, `hidden from`, `only be listed`, `are filtered` — no hit relates to extensions or platforms.
- `manifest` — only seven files in the whole manual and API mention it (`advanced/deploying_blender.txt`, `advanced/extensions/addons.txt`, `advanced/extensions/version_number_guidelines.txt`, `advanced/extensions/getting_started.txt`, `advanced/extensions/python_wheels.txt`, `advanced/extensions/creating_repository/static_repository.txt`, `advanced/command_line/extension_arguments.txt`). I read all seven in full. This matters: it means there is no eighth page describing manifest behaviour that I could have missed.
- `wheel` (excluding mouse-wheel and colour-wheel hits) — confined to `advanced/extensions/index.txt`, `advanced/extensions/addons.txt`, `advanced/extensions/getting_started.txt`, `advanced/extensions/python_wheels.txt`, and one release-note bug-fix line.
- `bl_info` — hits in `advanced/scripting/addon_tutorial.txt`, `advanced/extensions/addons.txt`, and three API pages (`info_overview.txt`, `bpy.types.AddonPreferences.txt`, `bpy.types.USDHook.txt`). I read the surrounding text on all of them. No platform field.
- `site-packages`, `portable`, `cross-platform` — read every hit.

Pages read end to end and ruled out: `editors/preferences/extensions.txt` (describes installing, updating, enabling, repositories — never mentions platform), `editors/preferences/addons.txt` (same), `advanced/command_line/extension_arguments.txt` (every sub-command of `blender --command extension`; `--split-platforms` is the only platform-aware option and it belongs to `build`), `advanced/extensions/addons.txt`, `advanced/extensions/getting_started.txt`, `advanced/extensions/python_wheels.txt`, `advanced/extensions/index.txt`, both repository pages, `advanced/deploying_blender.txt`, `advanced/blender_directory_layout.txt`, `troubleshooting/python.txt`.

API pages ruled out: `bpy.ops.extensions.txt` — I read every operator it lists (`package_install`, `package_install_files`, `package_install_marked`, `package_uninstall`, `repo_sync`, `repo_refresh_all` and the rest). Not one takes a platform argument or mentions platform in its description. Also `bpy.types.Addon.txt`, `bpy.types.Addons.txt`, `bpy.types.AddonPreferences.txt`, `bpy.types.Preferences.txt` — none carries a platform property. `bpy.app.build_platform` exists but describes Blender, not an add-on: "The platform this blender instance was built for" (https://docs.blender.org/api/5.0/bpy.app.html).

Pictures opened: `_images/editors_preferences_section_extensions.webp` (the Get Extensions panel — shows the search field, an "Add-ons" type filter, a Repositories menu, and the online-access prompt; no platform column, filter or badge anywhere in it) and `_images/editors_preferences_section_addons.webp` (the Add-ons list — a search field, an "Enabled Only" toggle, a tag icon, and a list of add-on names with tick boxes; again no platform indication). **LABEL: documented**, for what those two pictures show; source: the manual, https://docs.blender.org/manual/en/5.0/editors/preferences/extensions.html and https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html , date read 2026-07-30. I take nothing from them beyond what is visible.

Cycles' files searched for `platform`, `darwin`, `win32`, `Windows`, `macOS`, `Linux`, `sys.` across all `.py` files. The hits are Cycles asking Python at run time which machine it is on (for example `properties.py` line 1577 and `ui.py` line 2241), and a search for `blender_manifest` found no such file. Nothing there describes what Blender does with a manifest's `platforms` list.

**Not covered (4b).** I cannot say what Blender does when an add-on naming platforms is installed from a file, dragged in, or already sitting in a repository directory on a machine outside its list — whether it is refused, hidden, listed but not enableable, or enabled and left to fail. I also cannot say whether a *static* repository (as opposed to the dynamic kind quoted above) filters by platform at all, since the filtering sentence appears only on the dynamic page. And I cannot say what message, if any, a person or a script would see.

### 4c. Whether an add-on set up on one computer still works when copied to a computer of a different kind

**LABEL: documented**, for the general rule, which the manual states in one sentence:

> "While not common practice, Python add-ons can be distributed with their own precompiled libraries. Unlike regular
> Python scripts, these are not portable between different platforms."

- Source: the manual, https://docs.blender.org/manual/en/5.0/troubleshooting/python.html
- Date read: 2026-07-30

So: an add-on that is only Python text is portable; an add-on carrying compiled files is not. The same page names the failure and what to check:

> "If the add-on contains .pyd or .so files, check that the distribution is compatible with your operating
> system."

- Source: the manual, https://docs.blender.org/manual/en/5.0/troubleshooting/python.html
- Date read: 2026-07-30

**LABEL: inferred**, for what this means for an extension that has been installed and is then copied. The step: the manual says installing an extension with wheels unpacks them —

> "Now installing the package will extract the wheel into the extensions own site-packages directory."

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/python_wheels.html
- Date read: 2026-07-30

— and the wheels that were bundled are per-platform, one each for macOS, Linux and Windows in the worked example. Putting those two together with "these are not portable between different platforms": an installed extension that bundled compiled wheels has, on disk, the unpacked binaries chosen for the machine it was installed on. Copying that installed folder to a machine of another kind therefore carries the wrong binaries. A reader has to supply the assumption that only the matching wheel is unpacked, which the manual does not state outright — hence inferred, not documented. If instead every wheel is unpacked, the copy might still contain the right one; the sources do not settle which.

**LABEL: documented**, for the case where the add-on is pure Python and delivered as a package rather than as an installed copy. The manual describes bundling extensions for a whole studio by putting the extracted package in a directory Blender is pointed at:

> "Extensions packages should be extracted in this system directory,
> with a resulting path like this:
>
> $BLENDER_SYSTEM_EXTENSIONS/system/my-addon/blender_manifest.toml"

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html
- Date read: 2026-07-30

and the same page says of Blender itself that it can be treated as files to copy about:

> "Blender downloads can be extracted to any directory on the system, as
> a self contained installation. Multiple Blender versions can
> co-exist on the same system, and deployment can be automated using
> standard file management tools."

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html
- Date read: 2026-07-30

Neither sentence says whether the machines involved may be of different kinds. It says nothing about platform at all.

**Two things that get in the way of copying, both documented.** First, where the files live differs by operating system, so a copy is never a copy to the same place:

> "Linux
>
> $HOME/.config/blender/5.0/
>
> ...
>
> macOS
>
> /Users/$USER/Library/Application Support/Blender/5.0/
>
> ...
>
> Windows
>
> %USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.0\"

and the same page confirms extensions are among what is kept there:

> "User directories store preferences, startup file, installed extensions,
> presets and more."

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html
- Date read: 2026-07-30

Second, an add-on's own identifier is not fixed across machines:

> "The value of __package__ will vary between systems so it should never be replaced with a literal string."

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/extensions/addons.html
- Date read: 2026-07-30

The same page explains why: the repository the add-on was installed into becomes part of its name — "now instead of kitsu the module name would be bl_ext.{repository_module_name}.kitsu instead". That is about which repository, not which kind of computer, so I do not stretch it further.

**Not covered (4c).** Nothing here settles whether an add-on's *settings* (its add-on preferences) survive being carried to another machine, which is the second reading of "set up" that I flagged at the top. Nothing settles whether Blender re-checks the manifest's `platforms` when it finds an add-on already sitting in a repository directory. Nothing settles what happens to an installed extension's unpacked `site-packages` directory when the extension is copied rather than installed. And nothing addresses copying between two machines of the *same* kind, which the question does not ask but which is the obvious neighbouring case.

### The two closing lines, for the whole of question 4

**1. Outside the interface.** Mostly it holds, and for the manifest part it holds entirely: `platforms` is a line of text in a file, and nothing about it depends on a person being present. The manual documents the same operations from the command line, with no interface involved — `blender --command extension install`, `install-file`, `list`, `build --split-platforms`, `validate`, and repository management, all set out at https://docs.blender.org/manual/en/5.0/advanced/command_line/extension_arguments.html (date read 2026-07-30). `--split-platforms` is described there as building "a separate package for each platform. Adding the platform as a file name suffix (before the extension)", so the per-platform packaging is a command-line facility in the first place. The one thing that does *not* carry over is the advice the manual gives when an add-on fails to come to life: "check the Console window for any errors that may have occurred". That is written for somebody sitting at Blender. **LABEL: not in the sources**, for what a driven Blender reports instead — I found no page saying what an installation or enable failure caused by a platform mismatch looks like on the command line, or what exit status it produces. **LABEL: inferred** for one small point: since the whole extension command-line tool exists and the manual gives it a `validate` sub-command that reads a manifest without a person present, the `platforms` value can be read by a machine; the step is that validating a manifest requires reading its fields.

**2. Operating system.** The heart of this answer is *about* operating systems, so it holds everywhere in the sense that the five names and the "if omitted, all operating systems" rule are stated once, without qualification, for all of them. Two places where the sources make an operating-system distinction: the user directory paths differ for Linux, macOS and Windows (quoted above); and the troubleshooting page's worked example of a clashing compiled library is written for Windows only, under a heading "Platform Specific / Windows", with no equivalent section for macOS or Linux (https://docs.blender.org/manual/en/5.0/troubleshooting/python.html, date read 2026-07-30). One further documented point, from a page that names Blender 5.0 outright: on macOS, Blender 5.0 requires Apple Silicon, quoted in 4b above — so of the two macOS names available in a manifest, only one describes a machine that runs this version.

---

## What is still open

Every test on both lists has now been run, and the results are written into the answers above as observed findings. The Mac list, with each test's set-up and steps, is at `Audits/Blender Research/Add-ons/Hand tests.md`; the Linux list is at `Audits/Blender Research/Add-ons/Hand tests — Linux.md`.

**Two machines, never merged.** Everything was run on the Creator's MacBook Pro (M1 Pro chip, macOS 26.5.2, Apple's own graphics), and then again on a rented Linux machine (Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH). Where a test was run on both, each machine's result is written up in its own block and the finding says whether they agreed.

**The five window tests are now answered for this version.** The Creator ran them on macOS in Blender 5.2 only, so they were not answers here; they were run again at Blender's own window on the Linux machine, in all four versions, on 2026-08-06, and those results are in the answers above. They settle this version on Linux. What the same panels show on macOS in this version is still untested, because the Creator's run covered 5.2 alone.

**Three things the Linux machine settled that the Mac could not**: what Blender does with an add-on carrying a compiled part built for another kind of computer; what happens when an installed, set-up add-on is carried across to a machine of a different kind; and what Cycles' own device setting reads and offers on a machine whose graphics are not Apple's. All three are in the answers above.

**Driving a window over SSH: what works and what does not.** The machine has `xdotool`. Blender receives button and key events sent to its own window — its own modal handler printed them back with the coordinates that were sent — but it receives **no pointer-motion events at all**, and **a click could not be shown to work any control**: three separate clicks aimed at a Properties tab, a preferences section and a tick box each left the matching property unchanged. So nothing in this document was got by clicking. The workspace "Filter Add-ons" panel, the Get Extensions repositories drop-down and the render-engine list were each opened instead by one of Blender's own operators, with the identifier read out of `bpy.types` first, and each finding quotes the operator and what it returned. One thing is still unread: in Blender 4.5, 5.0 and 5.1 a tooltip covered the repositories menu in every picture taken, so what those three versions list there was not seen.

**Cannot be settled with the equipment the project has** — one MacBook Pro with Apple's own graphics; one rented Linux machine with one NVIDIA card, **which was destroyed on 2026-08-06 and can never be run again**; and a second rented Linux machine with two NVIDIA cards and no display server at all. *(Two-card citation check, 2026-08-06: this named two machines and called the rented one a container. There are three, one of them gone, and the list below was written when there were two.)*

- Anything on Windows. That leaves untested the "is not a valid Win32 application" failure the manual describes, the `file://HOST/share/...` repository address the sources give for Windows only, and whether the manual's line about headless Windows systems touches add-ons. (Questions 3, 4, 5, 8, 11.)
- A computer with **no graphics hardware at all**. *(Two-card citation check, 2026-08-06: this line read "A computer with no screen attached at all. Both machines have a screen…" and is out of date. A third machine, with two NVIDIA cards and **no display server at all**, ran the whole add-on cycle on 2026-08-06; that result is written up under question 3 of this version's main findings document, not here, this document not covering question 3. What is left is a machine with no graphics device, which no machine the project has ever had.)* (Question 3.)
- A graphics card of a make that is neither Apple's nor NVIDIA's. **Two graphics cards in one machine can now be reached** — the third machine has two — but nothing in this topic's questions turns on them, so it is left alone rather than half-answered; what two cards do to Cycles' own device list belongs to *The machine*. (Questions 3, 4.) *(Two-card citation check, 2026-08-06: the second half of this line was no longer true.)*
- How add-ons are sold, and anything about licence keys, activation or entitlement. No test on a computer can answer it. (Question 8.)
- A third-party extensions site. No address for one appears in the sources, so none can be tried without guessing at one. A repository made from a folder was tested instead, on both machines. (Questions 5, 8.)

---

2026-08-04 — Opus 5 (Claude Code) — Add-ons, Blender 5.0
