# Builds — Blender 5.1

**Sources consulted, and when.** Blender 5.1 manual (`Blender Sources/manual_5.1`, including its `_images` folder), Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), Cycles' own files (`Blender Sources/cycles_5.1`), and the undated-by-version requirements page (`Blender Sources/requirements/requirements_2026-08-01.txt`). All were taken on **2026-07-30**, and that is the date read for everything below, except the requirements page, which is a snapshot of **2026-08-01**.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

I ran no helper agents. Everything below is my own reading.

A note on one source. The requirements page carries no version in its address, so it speaks for whatever was current on the day it was taken. It never says "5.1". Where I use it, I say so and I label the step.

---

## 1. What kinds of build does Blender publish, and how is each kind named and marked?

**Label: documented** (with one inferred step, marked).

The manual names four things you can get, and describes each.

> "Blender offers a variety of different binary packages to choose from depending on their level of stability."

> "**Stable Release**
> A package that contains the latest features and is considered stable without regressions. A new stable version is available roughly every three months."

> "**Long-term Support**
> A package designed for long-lasting projects requiring a very stable version of Blender. LTS releases are supported for two years and will not have any new features, API changes or improvements. A new long-term support version is available every year. These LTS releases will occasionally have minor patches (such as 4.2.6) which improve stability or fix critical bugs."

> "**Daily Builds**
> A package updated daily to include the newest changes in development. These versions are automatically built on a schedule. They are not as thoroughly tested as the release types above, and might break or crash. Builds marked as Alpha are still undergoing major changes and feature additions, while those marked Beta are feature-complete and are under development for refinement and stability."

> "Stability can be expected to increase from Alpha to Beta to Release Candidate (RC) to a final release."

> "**Build from Source**
> Blender's source code is available for free to either reference or to build and use."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html — read 2026-07-30.

**How each kind is named.** The manual's own list of manual versions shows the naming pattern for releases: a two-number version, with `LTS` appended for long-term-support ones.

> "5 Series | 2025 – 2027 … Blender 5.1 … Blender 5.0
> 4 Series | 2023 – 2025 … Blender 4.5 LTS … Blender 4.4 … Blender 4.2 LTS"

Source: manual — https://docs.blender.org/manual/en/5.1/versions.html — read 2026-07-30.

The release notes show the third number in use for fix-only releases of 5.1, and give them a name:

> "# Blender 5.1: Corrective Releases
> ## Blender 5.1.2
> Released on May 19th 2026, Blender 5.1.2 features 12 bug fixes"

> "## Blender 5.1.1
> Released on April 14th 2026, Blender 5.1.1 features 71 bug fixes"

Source: release notes — https://developer.blender.org/docs/release_notes/5.1/corrective_releases/ — read 2026-07-30.

And the version's own page dates the release:

> "Blender 5.1 was released on March 17, 2026."

Source: release notes — https://developer.blender.org/docs/release_notes/5.1/ — read 2026-07-30.

**How the kind is marked on a running Blender.** The Python API exposes the stage as a word:

> "bpy.app.version_cycle
> The release status of this build alpha/beta/rc/release
> Type: str"

and, separately, how it was compiled:

> "bpy.app.build_type
> The type of build (Release, Debug)
> Type: bytes"

Source: Python API — https://docs.blender.org/api/5.1/bpy.app.html — read 2026-07-30.

The manual says the splash screen carries the version:

> "The upper part of the splash screen contains the splash image with the Blender version in the top right."

Source: manual — https://docs.blender.org/manual/en/5.1/interface/window_system/splash.html — read 2026-07-30.

I opened the picture on that page, `_images/interface_splash_current.webp`. In the top right it reads **`5.0.0 Alpha`** — so the mark in that corner is the three-number version followed by the stage word. (Documented, since the picture is part of the manual. Note the picture is of a 5.0 alpha build, not of 5.1.)

**Packaging, which is a separate axis from the four kinds.** Per platform the manual names these forms:

> "Download the zip-file or Windows Installer File." and "Install from Microsoft Store" (Windows)
> "Blender for macOS is distributed as disk images (dmg-files)." (macOS)
> "Download the Linux version for your architecture and decompress the file" and "snap install blender --classic" (Linux)

Sources: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html , https://docs.blender.org/manual/en/5.1/getting_started/installing/macos.html , https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html — read 2026-07-30. Steam is a fifth route: https://docs.blender.org/manual/en/5.1/getting_started/installing/steam.html

And per processor architecture:

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html — read 2026-07-30.

> "Blender supports both Intel and Apple Silicon architectures on macOS. Make sure to download a variant that is compatible with your CPU's architecture."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/macos.html — read 2026-07-30.

I must flag a tension in the sources rather than resolve it. The requirements snapshot says the opposite about Intel Macs:

> "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later."

Source: `Blender Sources/requirements/requirements_2026-08-01.txt`, macOS section — read 2026-08-01. This page does not name 5.1; it says "5.0 and later", and I am not resolving which of the two statements governs a 5.1 macOS download.

One more published build kind, named only in the API documentation:

> "Blender supports being built as a Python module, allowing import bpy to be added to any Python script"
> "Note — Blender as a Python Module isn't provided on Blender's official download page."
> "A pre-compiled bpy module is available via PIP."

Source: Python API — https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html — read 2026-07-30.

**Not covered.** The sources do not give the file-name pattern of a published download (no example like `blender-5.1.0-linux-x64.tar.xz`), do not say where daily builds are published, do not say how a daily build's name encodes its date or commit, do not mention checksums or signing, and do not say whether a "Release Candidate" is published as a separate downloadable package or only as a phase. Searched for `checksum`, `sha256`, `notariz`, `code sign`, `.dmg`, `.tar.xz` across manual, API and release notes. Five of the six return nothing at all. `sha256` returns one hit, in `manual_5.1/advanced/extensions/creating_repository/static_repository.txt`, and it is an add-on download link on the extensions platform, not anything about a Blender build — so the gap stands. *(Citation check, 2026-08-05: this line previously read "nothing" for all six terms. Corrected against the files.)*

**Observed — hand test, 2026-08-04.** Everything in this block was run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The build was Blender 5.1.2, installed at `/Applications/Blender 5.1.2.app`.

*What the build prints when asked its version.* In a terminal, with nothing else set up:

`"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" --version`

It printed, word for word:

> ```
> Blender 5.1.2
> 	build date: 2026-05-19
> 	build time: 01:30:33
> 	build commit date: 2026-05-18
> 	build commit time: 13:43
> 	build hash: ec6e62d40fa9
> 	build branch: blender-v5.1-release
> 	build platform: Darwin
> 	build type: Release
> ```

Three further lines followed — `build c flags:`, `build c++ flags:` and `build link flags:`, each very long — and then `	build system: CMake`. The command exited straight away, with exit code 0. There is no stability word and no LTS marking on the first line.

*What the build reports about itself in Python, with no window open.* In a terminal:

`"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" --background --python-expr "import bpy; print('BEGIN'); [print(a, '=', repr(getattr(bpy.app, a))) for a in sorted(dir(bpy.app)) if not a.startswith('_')]; print('TYPE version_cycle', type(bpy.app.version_cycle)); print('TYPE build_type', type(bpy.app.build_type)); print('END')"`

It printed every name and value on `bpy.app` — 71 entries in all. The lines bearing on this question, word for word:

> ```
> build_branch = b'blender-v5.1-release'
> build_commit_date = b'2026-05-18'
> build_commit_time = b'13:43'
> build_commit_timestamp = 1779111819
> build_date = b'2026-05-19'
> build_hash = b'ec6e62d40fa9'
> build_platform = b'Darwin'
> build_system = b'CMake'
> build_time = b'01:30:33'
> build_type = b'Release'
> module = False
> portable = False
> version = (5, 1, 2)
> version_cycle = 'release'
> version_file = (5, 1, 30)
> version_string = '5.1.2'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

and `build_options`, in full:

> `build_options = bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=True, jack=False, pulseaudio=False, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, potrace=True, pugixml=True, haru=True, experimental_features=False)`

`bpy.app.module`, which the API documents as "True when running Blender as a python module", read `False` on this installed build, and the Python-module build named in this answer was not obtained or tested.

*What the installed application file itself carries.* In a terminal:

`file "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender"` printed:

> `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender: Mach-O 64-bit executable arm64`

`lipo -archs "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender"` printed:

> `arm64`

`plutil -p "/Applications/Blender 5.1.2.app/Contents/Info.plist"` printed, among other lines:

> ```
>   "CFBundleIdentifier" => "org.blenderfoundation.blender"
>   "CFBundleName" => "Blender"
>   "CFBundleShortVersionString" => "5.1.2"
>   "CFBundleVersion" => "5.1.2"
>   "LSMinimumSystemVersion" => "11.2"
> ```

This does not resolve the tension recorded above between the 5.1 manual page, which says both Intel and Apple Silicon are supported on macOS, and the requirements snapshot, which says 5.0 and later need Apple Silicon. It records one thing only: the 5.1.2 macOS build installed on the Creator's Apple Silicon Mac holds one processor architecture, `arm64`.

**Observed — hand test, 2026-08-05.** The Creator ran this one, on their own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. They opened Blender 5.1.2 from the Applications folder in the ordinary way and sent screenshots of what was on screen. This settles what the splash-screen mark reads on a finished 5.1 release — the manual's picture is of a 5.0 alpha, as noted above.

*The splash screen.* The top right corner of the splash image reads:

> `5.1.2`

So on this finished release the corner carries the three-number version and no stage word, where the manual's picture of the 5.0 alpha carried `Alpha` after the numbers.

*The version at the right-hand end of the bar along the bottom of the window.* It reads:

> `5.1.2`

with a small globe icon and a vertical divider to its left.

**Observed — hand tests, 2026-08-06, on a second machine.** Everything in this block was run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. It is a container sharing somebody else's hardware, and other work was running on it at the same time, so nothing here says how long anything took. The build was Blender 5.1.2, extracted from Blender's own Linux archive at `/opt/blender/blender-5.1.2-linux-x64`. These are a second machine's results and sit beside the Mac's above; they are never merged with them.

*What the build prints when asked its version.* `/opt/blender/blender-5.1.2-linux-x64/blender --version` printed, word for word:

> ```
> Blender 5.1.2
> 	build date: 2026-05-19
> 	build time: 02:18:37
> 	build commit date: 2026-05-18
> 	build commit time: 13:43
> 	build hash: ec6e62d40fa9
> 	build branch: blender-v5.1-release
> 	build platform: Linux
> 	build type: Release
> ```

Three long flag lines followed, then `build system: CMake`. Exit code 0. No stability word and no LTS marking on the first line, the same as the Mac. The commit date, commit time, hash and branch are the Mac's; the build time is not, and `build platform` reads `Linux` where the Mac read `Darwin`.

*What the build reports about itself in Python, with no window open.* The lines bearing on this question:

> ```
> build_branch = b'blender-v5.1-release'
> build_commit_date = b'2026-05-18'
> build_commit_time = b'13:43'
> build_commit_timestamp = 1779111819
> build_date = b'2026-05-19'
> build_hash = b'ec6e62d40fa9'
> build_platform = b'Linux'
> build_system = b'CMake'
> build_time = b'02:18:37'
> build_type = b'Release'
> cachedir = '/root/.cache/blender/'
> memory_usage_undo = <built-in function memory_usage_undo>
> module = False
> portable = True
> version = (5, 1, 2)
> version_cycle = 'release'
> version_file = (5, 1, 30)
> version_string = '5.1.2'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

Seventy-one names in all, the same count as the Mac build. `bpy.app.module` reads `False` here too, so the Python-module build named in this answer was not obtained or tested on either machine. `portable` reads `True` here where the Mac read `False`, and no folder named `portable` exists in the extracted build — that was checked for all four Linux builds and none has one.

`build_options`, in full:

> `build_options = bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=False, jack=True, pulseaudio=True, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, potrace=True, pugixml=True, haru=True, experimental_features=False)`

Forty-four names, no `collada`, as on the Mac. Three names read the other way round from the Mac's: `coreaudio` is `False` here and `True` there, `jack` is `True` here and `False` there, and `pulseaudio` is `True` here and `False` there.

*What the build file itself is.* `file /opt/blender/blender-5.1.2-linux-x64/blender` printed:

> `/opt/blender/blender-5.1.2-linux-x64/blender: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=ff48121e1374fd3d9f430acd39a6955139391238, for GNU/Linux 3.2.0, stripped`

and `readelf -h` reported `Class: ELF64`, `Type: EXEC (Executable file)`, `Machine: Advanced Micro Devices X86-64`. One architecture, as on the Mac, though a different one. **This does not resolve the tension recorded above** between the 5.1 manual page and the requirements snapshot about Intel Mac support, and nothing on a Linux machine could. One nearby fact was seen: Blender's own download server, read on 2026-08-06 for the 5.2 release, publishes one macOS file and it is `blender-5.2.0-macos-arm64.dmg`. That is 5.2, not 5.1.

*What a published build file is called.* This is the first gap the "Not covered" note above records — no example like `blender-5.1.0-linux-x64.tar.xz`. The 5.2 release folder on Blender's own download server, read 2026-08-06, lists `blender-5.2.0-linux-x64.tar.xz`, `blender-5.2.0-macos-arm64.dmg`, `blender-5.2.0-windows-arm64.msi`, `blender-5.2.0-windows-arm64.msix`, `blender-5.2.0-windows-arm64.zip`, `blender-5.2.0-windows-x64.msi`, `blender-5.2.0-windows-x64.msix`, `blender-5.2.0-windows-x64.zip`, plus `blender-5.2.0.md5` and `blender-5.2.0.sha256`. **So the naming pattern this answer guesses at is right in shape**: the word blender, the three-number version, the operating system, the processor, and the file kind, with no stability word and no LTS marking. It also settles the "no mention of checksums" part of the same note: a checksum file is published beside the archives, and the fetched Linux archive matched it. No listing was read for the 5.1 release, so this is observed for one release only.

*What the splash screen and the bottom bar show on Linux.* Blender 5.1.2 was opened with a window on the machine's own display and photographed with Blender's own screenshot command. The top right corner of the splash image reads `5.1.2` — the three numbers and no stage word, where the manual's picture of a 5.0 alpha carried `Alpha`. The right-hand end of the bottom bar reads `5.1.2`. Both the same as the Creator's Mac.

**Outside the interface. Observed on two machines.** The kinds and their names are properties of what Blender publishes, so they do not depend on how Blender is driven. The one part that is interface-only is the splash-screen mark: the splash is a window, and it was looked at with a window open on 2026-08-05, as recorded above. The equivalents outside the window are `bpy.app.version_cycle` / `bpy.app.build_type` and the `--version` argument (question 3); all three were read above with no window open.

**Operating system. Observed for macOS and for Linux.** The four kinds are described without any OS qualification. The packaging forms are explicitly per-OS, as quoted above. The Microsoft Store form is Windows-only; the DMG form is macOS-only; Snap and distribution packages are Linux-only; Steam is offered for all three. Observed on the Creator's Mac: the installed 5.1.2 macOS build holds one architecture, `arm64`. **Observed on the rented Linux machine, 2026-08-06:** the 5.1.2 Linux build is a single-architecture x86-64 executable, reports `build platform: Linux`, and reads `portable = True` where the Mac read `False`; three of its forty-four build options read the opposite way from the Mac build's; the splash corner and the bottom bar both read `5.1.2`, as on the Mac. Windows remains untested.

---

## 2. Can builds of each kind be listed and fetched by something other than a person at a web page?

**Label: not in the sources**, for Blender's own published builds. Two partial documented answers sit next to it.

Nothing in the manual, the API documentation or the release notes describes a machine-readable index of Blender builds, a download API, a mirror layout, or a command that fetches a Blender build from blender.org. The manual points at a "Downloading Blender page" but does not describe how to reach it other than by reading it:

> "Check the Downloading Blender page to find the minimum requirements and the different versions that are available for Blender (if you have not done so yet)."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html (the same sentence opens the Linux and macOS pages) — read 2026-07-30.

The manual states outright that Blender itself will not fetch a build:

> "Blender is designed to not require an internet connection, so it doesn't have a built-in update system. This means you will need to update Blender yourself by following the platform-specific upgrade steps described in the sections below."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html — read 2026-07-30.

**Partial answer one — third-party package managers.** These are commands, not a web page, and the manual gives one verbatim:

> "Assuming snap is already installed, Blender can be installed through snap with:
> snap install blender --classic"

> "Installing Blender via the distribution's native mechanisms ensures consistency with other packages on the system and may provide other features (given by the package manager), such as listing of packages, update notifications and automatic menu configuration."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html — read 2026-07-30.

Steam and the Microsoft Store also fetch without a person at a web page, though the manual describes them as clicked, not scripted:

> "When an update for Blender is available on Steam, Steam will automatically download and apply the update for you."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/steam.html — read 2026-07-30.

**Partial answer two — the Python-module build.** That one kind does have a documented machine-fetchable channel:

> "A pre-compiled bpy module is available via PIP."

Source: Python API — https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html — read 2026-07-30.

**What I could see about automation.** The one page about production deployment covers automating the *placement* of a build, not the fetching of it:

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30.

**Where I looked (for the silence).** Full-text searches across `manual_5.1`, `api_5.1` and `release_notes_5.1` for: `builder.blender.org`, `download.blender.org`, `buildbot`, `mirror`, `REST API`, `JSON`, `automated deployment`, `automate`, `wget`, `curl`, `Downloading Blender`, `package manager`, `flatpak`, `winget`, `chocolatey`, `homebrew`, `apt`. The only `JSON` and `curl` hits are about *extension* repositories (https://docs.blender.org/manual/en/5.1/advanced/extensions/creating_repository/index.html and `.../dynamic_repository.html`), which serve add-ons, not Blender builds — I read those pages and ruled them out. The only `builder.blender.org` hit is a link to a benchmark page in the animation release notes, not a build index. Pages read in full and ruled out: the four installing pages, the Steam page, the deploying-in-production page, and the command-line arguments page (no argument fetches or lists builds).

**Not covered.** Whether an undocumented but stable download index exists; whether daily builds are reachable at a predictable address; whether `pip install bpy` offers versions other than the current one; whether snap/Steam/Store carry LTS, daily or RC builds at all.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. The build was Blender 5.1.2 at `/Applications/Blender 5.1.2.app`.

Blender's own `-c help` lists the commands a build offers. In a terminal:

`"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -c help`

The listing it printed, word for word:

> ```
> Blender Command Listing:
> 	extension
> ```

Nothing else. (The run also printed messages from add-ons the Creator has installed; none of that is part of the listing.) The whole `--help` output, 498 lines, was also read, and no argument in it fetches or lists a Blender build.

**Observed — hand tests, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. This is where the Snap and distribution-package routes quoted above could finally be exercised. **This settles the "not in the sources" label at the head of this answer for one route: a listing of published Blender builds that a machine can read does exist, and Snap serves it.**

*What `snap info blender` lists.* Its publisher line reads `publisher: Blender Foundation (blenderfoundation**)`. Under `channels:` it printed about a hundred and fifty lines. The opening, word for word:

> ```
> channels:
>   latest/stable:     5.2.0   2026-07-14 (7599) 426MB classic
>   latest/candidate:  5.1.2   2026-06-04 (7528) 436MB classic
>   latest/beta:       ^                               
>   latest/edge:       5.3.0   2026-08-05 (7676) 432MB classic
> ```

and the lines for this version:

> ```
>   5.1/stable:        5.1.2   2026-05-19 (7480) 436MB classic
>   5.1/candidate:     ^                               
>   5.1/beta:          ^                               
>   5.1/edge:          ^                               
> ```

Every release line back to 2.79 has a channel, named for the line, with the letters `lts` in the name where the line is a long-term-support one — `5.2lts`, `4.5lts`, `4.2lts`, `3.6lts`, `3.3lts`, `2.93lts`, `2.83lts`. This version's channel is plain `5.1`, with no `lts`. Each channel exists in four forms: stable, candidate, beta and edge. **This answers three of the four things the "Not covered" note above lists**: a stable download index exists for this route; daily and unstable builds are reachable through the `edge` and `beta` channels; and a particular kind can be asked for by name.

*What `snap install blender --classic` delivers.* Run with no channel named, it printed:

> `blender 5.2.0 from Blender Foundation (blenderfoundation**) installed`

with exit code 0, and `snap list blender` printed:

> ```
> Name     Version  Rev   Tracking       Publisher            Notes
> blender  5.2.0    7599  latest/stable  blenderfoundation**  classic
> ```

So with no channel named the command gives whatever `latest/stable` holds — 5.2.0, not this version.

*Whether the Snap route updates itself.* `snap refresh --time` printed:

> ```
> timer: 00:00~24:00/4
> last: today at 03:51 UTC
> next: today at 11:11 UTC
> ```

A repeating timer with a next time already set.

*What a distribution's own package carries.* `apt-cache policy blender` printed:

> ```
> blender:
>   Installed: (none)
>   Candidate: 3.0.1+dfsg-7
>   Version table:
>      3.0.1+dfsg-7 500
>         500 http://archive.ubuntu.com/ubuntu jammy/universe amd64 Packages
> ```

`apt-cache show blender` names its file as `blender_3.0.1+dfsg-7_amd64.deb`, gives its size as 24603992 bytes, and carries a `SHA256` line. Its dependency list names about forty system libraries including `libpython3.10`, so it is a different kind of build from a blender.org one, which brings its own Python. It was downloaded and read but **not installed**: `dpkg -c` shows it would put a program at `./usr/bin/blender`, a path this machine already uses for another Blender, and other work was running on the machine at the time.

*A published checksum, which the search record above notes as absent.* Beside the release archives on Blender's own download server sit `blender-5.2.0.md5` and `blender-5.2.0.sha256`. The second was fetched and the Linux archive checked against it: `sha256sum -c` printed `blender-5.2.0-linux-x64.tar.xz: OK`.

**Outside the interface. Observed on two machines.** The whole question is outside the interface, and the answer does not change: Blender has no built-in updater in either mode, and the package-manager routes are shell commands that work with no Blender window involved. The `-c help` run above was made from a terminal with no window, and returned only the extensions command.

**Operating system. Observed for macOS and for Linux.** Snap and distribution packages: Linux only (quoted above). Microsoft Store: Windows only. Steam: "Linux, macOS, or Windows" per the Steam page. PIP `bpy`: the API page names no OS. The absence of a documented build index applies to all three. **Observed on the Creator's Mac:** the build's own commands and arguments fetch nothing. **Observed on the rented Linux machine, 2026-08-06:** the Snap and distribution-package routes both exist there and both work as quoted, Snap carries a machine-readable listing of every published release line, and Snap refreshes itself on a timer. So the answer is now sharply different on the two systems: Linux has a listing and an automatic fetch route, and macOS has neither. Windows, Steam, the Microsoft Store and the PIP `bpy` route remain untested on both.

---

## 3. Can an installed Blender be asked what it is — which version, and which kind of build?

**Label: documented.**

**From the command line:**

> "-v, --version
> Print Blender version and exit."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

**From inside the window,** the Blender menu in the Topbar:

> "**About Blender**
> Opens a menu displaying the following information about Blender:
> Version: The Blender version.
> Date: Date when Blender was compiled.
> Hash: The Git Hash of the build. This can be useful to give to support personnel when diagnosing a problem.
> Branch: Optional branch name.
> Windowing Environment: On Linux, this will show either Wayland or X11 depending on the windowing environment that Blender is running on."

Source: manual — https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — read 2026-07-30.

Also on the splash screen, as quoted in question 1 ("the Blender version in the top right"), and in the manual's picture of it, which reads `5.0.0 Alpha`.

**As a saved text file:**

> "**Save System Info**
> This extracts system information which can be useful for including in bug reports, inspecting the configuration, or diagnosing problems.
> You will be prompted to save a text file called system-info.txt."
> "**Blender**
> This section shows you the Blender version, details about the build configuration, and the path in which Blender is running."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/help.html — read 2026-07-30.

That is drivable without touching the menu:

> "bpy.ops.wm.sysinfo(*, filepath='')
> Generate system information, saved into a text file"

Source: Python API — https://docs.blender.org/api/5.1/bpy.ops.wm.html — read 2026-07-30.

**From Python,** the fullest set. All quoted from https://docs.blender.org/api/5.1/bpy.app.html, read 2026-07-30:

> "bpy.app.version — The Blender version as a tuple of 3 numbers (major, minor, micro). eg. (4, 3, 1)"
> "bpy.app.version_string — The Blender version formatted as a string"
> "bpy.app.version_cycle — The release status of this build alpha/beta/rc/release"
> "bpy.app.version_file — The Blender File version, as a tuple of 3 numbers (major, minor, file sub-version), that will be used to save a .blend file."
> "bpy.app.build_hash — The commit hash this Blender instance was built with"
> "bpy.app.build_branch — The branch this blender instance was built from"
> "bpy.app.build_date — The date this Blender instance was built"
> "bpy.app.build_time — The time this Blender instance was built"
> "bpy.app.build_commit_date — The date of the commit this Blender instance was built from"
> "bpy.app.build_commit_timestamp — The unix timestamp of the commit this Blender instance was built from"
> "bpy.app.build_platform — The platform this blender instance was built for"
> "bpy.app.build_system — Build system used"
> "bpy.app.build_type — The type of build (Release, Debug)"
> "bpy.app.build_options — Constant value bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, … cycles=True, cycles_osl=True, …)"
> "bpy.app.binary_path — The location of Blender's executable, useful for utilities that open new instances."
> "bpy.app.module — Boolean, True when running Blender as a python module"

**Which "kind of build" this settles, and which it does not.** `version_cycle` names alpha / beta / rc / release, so the daily-build stages of question 1 are readable. `build_type` names Release or Debug. `module` says whether this is the Python-module build.

There is **nothing in the sources** that says an installed Blender reports whether it is an LTS release. Searched `manual_5.1` and `api_5.1` for `LTS` and `long term` / `long-term`: `LTS` does not occur in the API documentation at all. In the manual it occurs on the installing page, the manual-versions page, the deploying page, the blend-file compatibility page, the markup guide and the version-history page — plus some incidental matches inside the base64 data of `_images` SVG files. None of those pages describes an LTS marker on a build. *(Citation check, 2026-08-05: this record previously said the API and manual hits were "all unrelated words containing those three letters" apart from three named pages. There are no API hits at all, and the manual's meaningful hits are wider than three pages. Corrected against the files; the conclusion is unchanged.)* Per the manual's own description, LTS is a property of a version line (4.5 LTS, 4.2 LTS), so what you get back is the version number and it is a separate matter to know which lines are LTS ones.

**Not covered by the sources.** The exact text `--version` prints. Whether `--version` prints the cycle word (alpha/beta/rc) as well as the numbers — the manual only says "Print Blender version and exit". Whether the About menu or system-info file names the LTS status. What `build_platform` and `build_system` actually contain as strings. All but the About menu are answered by the test below; the sources themselves stay silent on them.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The build was Blender 5.1.2 at `/Applications/Blender 5.1.2.app`.

*From the command line.* `"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" --version` printed the whole block quoted under question 1: `Blender 5.1.2`, then the build date and time, the commit date and time, the commit hash, the branch, the build platform, the build type, three long flag lines, and the build system. So the printed text carries the hash, the branch and the dates. It carries no alpha/beta/rc word, and nothing beyond the three numbers on the first line.

*From Python, with no window open.* The `--background --python-expr` command quoted under question 1 returned every `bpy.app` value. The ones this question asks about, word for word:

> ```
> background = True
> binary_path = '/Applications/Blender 5.1.2.app/Contents/MacOS/Blender'
> build_platform = b'Darwin'
> build_system = b'CMake'
> build_type = b'Release'
> module = False
> version_cycle = 'release'
> version_string = '5.1.2'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

So `build_platform` reads `b'Darwin'` and `build_system` reads `b'CMake'` on this Mac, and both come back as bytes rather than text, as does `build_type`.

*As a saved file, with no window open.* In a terminal:

`"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" --background --python-expr "import bpy; bpy.ops.wm.sysinfo(filepath='<a folder outside the project>/system-info.txt')"`

Blender printed `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)` and then `Blender quit`, and the file was written. Its opening and Blender section, word for word:

> ```
> ====================================
> = Blender 5.1.2 System Information =
> ====================================
> 
> 
> Blender:
> ====================================
> 
> version: 5.1.2, branch: blender-v5.1-release, commit date: 2026-05-18 13:43, hash: ec6e62d40fa9, type: Release
> build date: 2026-05-19, 01:30:33
> platform: 'macOS-26.5.2-arm64-arm-64bit-Mach-O'
> binary path: '/Applications/Blender 5.1.2.app/Contents/MacOS/Blender'
> ```

Three long build-flag lines follow, then `build system: CMake`. The next section, headed `Python:`, opens `version: 3.13.9 (main, Apr 25 2025, 12:39:20) [Clang 17.0.0 (clang-1700.6.3.2)]`. Nothing in the file names an LTS status.

*What the arguments list itself says.* `--help` printed 498 lines. The version entry, word for word:

> ```
> -v or --version 
> 	Print Blender version and exit.
> ```

with no platform note beside it, while `/?` on the same page prints `Print this help text and exit (Windows only).` and `--qos <level>` prints `Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only).`

**Observed — hand test, 2026-08-05.** The Creator ran this one, on their own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. They opened Blender 5.1.2 from the Applications folder in the ordinary way and sent screenshots of what was on screen. This settles the "whether the About menu names the LTS status" gap above.

*The About Blender panel.* Clicking the Blender icon at the far left of the Topbar and choosing About Blender opens a panel. Its left-hand column reads, line by line:

> ```
> Blender
> Version: 5.1.2
> 
> Date: 2026-05-18 13:43
> Hash: ec6e62d40fa9
> Branch: blender-v5.1-release
> 
> Blender is free software
> Licensed under the GNU General Public License
> ```

Its right-hand column is a set of links: `Donate`, `What's New`, then `Credits`, `License`, `Blender Store`, `Blender Website`. Four of the five rows the manual documents are present — Version, Date, Hash and Branch. The fifth, `Windowing Environment`, which the manual says appears "On Linux", does not appear on this Mac. Nothing on the panel says LTS. The panel shows no build time, no build platform, no build type and no build system.

*The splash screen and the bottom bar.* The top right corner of the splash image reads `5.1.2`. The right-hand end of the bar along the bottom of the window reads `5.1.2`.

*The Help menu.* Clicking Help in the row of menus along the top drops down, in order:

> ```
> Manual
> Support
> User Communities
> Get Involved
> Release Notes
> ———
> Report a Bug
> Save System Info...
> ```

So the Save System Info entry quoted from the manual above is present on this build, written with a trailing ellipsis.

**Observed — hand tests, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. The build was Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64`. Every route above was tried again on this second machine.

*From the command line.* `--version` printed the block quoted under question 1 of this document's Linux block. It carries the hash, the branch and the dates, and no alpha/beta/rc word — the same as on the Mac.

*What the arguments list itself says.* `--help` printed **505 lines**, against 498 on the Mac. The version entry, word for word:

> ```
> -v or --version 
> 	Print Blender version and exit.
> ```

with no platform note, while `/?` prints `Print this help text and exit (Windows only).` and `--qos <level>` prints `Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only).` The four registration arguments are all printed, each marked `(Windows & Linux only)`.

*From Python, with no window open.* The `--background --python-expr` dump returned every value. The ones this question asks about:

> ```
> background = True
> binary_path = '/opt/blender/blender-5.1.2-linux-x64/blender'
> build_platform = b'Linux'
> build_system = b'CMake'
> build_type = b'Release'
> cachedir = '/root/.cache/blender/'
> module = False
> portable = True
> tempdir = '/tmp/blender_KRwyqe/'
> version_cycle = 'release'
> version_string = '5.1.2'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

So `build_platform` reads `b'Linux'` and `build_system` reads `b'CMake'` here, and both come back as bytes, as does `build_type`. Seventy-one names in all, the same count as the Mac. The four Linux builds' name lists were also set against each other, which no single version could do: **this build holds a name `cachedir` that the Linux 4.5 and 5.0 builds do not**, and that is the whole of the difference between their seventy names and this build's seventy-one.

*What Blender itself offers as commands.* `-c help` printed:

> ```
> Blender Command Listing:
> 	extension
> ```

the same as the Mac.

*As a saved file, with no window open.* In a terminal:

`/opt/blender/blender-5.1.2-linux-x64/blender --background --python-expr "import bpy; bpy.ops.wm.sysinfo(filepath='<a folder outside the project>/system-info.txt')"`

Blender printed one warning of its own first — `cycles | WARNING HIPEW initialization failed: Error opening HIP dynamic library`, which is this machine having no AMD graphics support — then `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)` and `Blender quit`. The file was written. Its opening and Blender section, word for word:

> ```
> ====================================
> = Blender 5.1.2 System Information =
> ====================================
> 
> 
> Blender:
> ====================================
> 
> version: 5.1.2, branch: blender-v5.1-release, commit date: 2026-05-18 13:43, hash: ec6e62d40fa9, type: Release
> build date: 2026-05-19, 02:18:37
> platform: 'Linux-6.8.0-87-generic-x86_64-with-glibc2.35'
> binary path: '/opt/blender/blender-5.1.2-linux-x64/blender'
> ```

Three long build-flag lines follow, then `build system: CMake`. The `Python:` section opens `version: 3.13.9 (main, Apr 25 2025, 12:39:20) [GCC 14.2.1 20250110 (Red Hat 14.2.1-7)]`. Searching the whole file for the letters LTS found none, as on the Mac. The Python version number is the Mac's; what compiled it is not.

*The About Blender panel, with a window open on the machine's own display.* Blender was started with a window, `bpy.ops.wm.splash_about` was called from a timer, and the window was photographed with Blender's own screenshot command. Its left-hand column reads, line by line:

> ```
> Blender
> Version: 5.1.2
> 
> Date: 2026-05-18 13:43
> Hash: ec6e62d40fa9
> Branch: blender-v5.1-release
> Windowing Environment: X11
> 
> Blender is free software
> Licensed under the GNU General Public License
> ```

**This settles the one row the manual documents and no machine had ever shown.** The manual says `Windowing Environment` appears "On Linux", and here it does, reading `X11`. On the Creator's Mac that row is absent, so all five documented rows are present here and four are present there. Nothing on the panel says LTS, which matches this being a non-LTS release.

*The splash screen and the bottom bar.* The top right of the splash image reads `5.1.2`; the right-hand end of the bottom bar reads `5.1.2`.

*The Help menu, with a window open.* `bpy.ops.wm.call_menu(name='TOPBAR_MT_help')` was called and the window photographed. The menu drops down, in order:

> ```
> Manual
> Support
> User Communities
> Get Involved
> Release Notes
> ———
> Report a Bug
> Save System Info...
> ```

Line for line what the Creator saw on the Mac, trailing ellipsis and all.

**Outside the interface. Observed on two machines.** Yes for `blender --version`, which needs no window. The sources also document `-b, --background` as "Run in background (often used for UI-less rendering)" and separately document `bpy.app.*` and `bpy.ops.wm.sysinfo()`. What was inferred here is now observed: with `--background`, every `bpy.app` value listed above was returned and `bpy.ops.wm.sysinfo()` wrote its file, with no window at any point. Only the splash screen and the About menu are interface-only, and both were looked at with a window open on 2026-08-05, as recorded above. One caveat, documented, for the Python-module build: "The attribute bpy.app.binary_path defaults to an empty string" and "Functionality controlled by command line arguments (shown by calling blender --help) isn't accessible" (https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html); on the installed build tested here `bpy.app.module` read `False` and `binary_path` held the real path, so that caveat did not apply.

**Operating system. Observed for macOS and for Linux.** `--version` is listed with no OS restriction, unlike neighbouring arguments on the same page which are explicitly marked (for example "/?  Print this help text and exit (Windows only)" and "--qos <level> … (Windows only)"). What was inferred from that absence is now observed on **two** systems: `--version` ran and printed on the Creator's Mac and on the rented Linux machine alike, so it is not Windows-only in fact. The About menu's "Windowing Environment" line is documented as Linux-only, and that is now settled both ways: on the Creator's Mac the row is absent, and **on the rented Linux machine it is present and reads `X11`**. Three more differences between the two machines: `build_platform` reads `b'Darwin'` on the Mac and `b'Linux'` on the rented machine; `portable` reads `False` on the Mac and `True` on the rented machine; and the help output is 498 lines on the Mac and 505 on the rented machine. Everything else about this answer came back the same on both. Windows remains untested.

---

## 4. Does Blender name one release as the one to use for ordinary work?

**Label: documented**, and the honest answer is: not one single release, but it does make one explicit recommendation.

The manual frames it as a choice, and says so:

> "Each package has the trade off of newest features versus stability. The package that is right for you depends on your requirements for those two. A studio, for example, might want to have long-term support, while a hobbyist may want newer features, while others may just want to test upcoming features. Each package described below has something just right for everyone."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html — read 2026-07-30.

The one flat recommendation in the sources is on the production-deployment page, and it points at LTS:

> "New Blender versions may add, remove or change functionality that affects the results of production files. For a given project, it is advisable to use a single LTS version of Blender. LTS versions receive bug fixes for two years."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30.

Against that, the Stable Release is the one described as fit for use without qualification:

> "A package that contains the latest features and is considered stable without regressions."

and the daily builds carry a warning that the other two do not:

> "They are not as thoroughly tested as the release types above, and might break or crash."

Both from https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html — read 2026-07-30.

**Not covered.** The sources never name a specific version number as "the one to use". They do not say which release blender.org offers by default on its download page. They do not say whether 5.1 itself is an LTS release — the manual-versions page lists "Blender 5.1" and "Blender 5.0" with no LTS suffix, while 4.5, 4.2, 3.6, 3.3, 2.93 and 2.83 carry it; I am reporting that listing, not drawing a conclusion about what 5.x will eventually get.

**Observed — hand test, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. This does not answer the question, and is recorded only because it is the nearest thing to a default any machine could be made to show. `snap install blender --classic` — the one command-line fetch route the manual documents — was run with no channel named, and printed:

> `blender 5.2.0 from Blender Foundation (blenderfoundation**) installed`

`snap list blender` then showed it tracking `latest/stable`. **That is Snap's own default, not Blender naming a release.** It does bear on the "Not covered" note above about whether 5.1 is itself an LTS release: `snap info blender` names a channel per release line and puts the letters `lts` in the name only where the line is one, and there is a `5.2lts` channel and a plain `5.1` channel with no `lts`. So on 2026-08-06 the channel names treat 5.1 as not an LTS line and 5.2 as one. That is Snap's naming, not the manual's, and it is recorded as such.

**Outside the interface.** The recommendation is editorial advice about which download to pick, so it is unchanged by how Blender is driven. The advice as written is aimed at exactly this situation — a production setup, deployed and automated — since it appears on the "Deploying Blender in Production" page.

**Operating system.** The passages give no OS qualification. The sources do not expressly establish whether the advice differs by operating system.

---

## 5. Can one Blender installed on a computer cause trouble for another that is already there?

**Label: mixed — documented that they normally co-exist; inferred for the specific ways they can collide; not in the sources for anything stronger.**

**Documented: co-existing is supported and expected.**

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system"

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30.

> "When using this method of installation, it is possible to have multiple versions of Blender installed." (Linux, extracted archive)

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html — read 2026-07-30.

> "Note, you do not have to overwrite your existing Blender installation. It's perfectly possible to have multiple versions installed side by side." (Windows, zip)

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html — read 2026-07-30.

**Documented: settings are kept apart by version number.**

> "The preferences need to be imported from previous versions because the configuration files of each Blender version are stored in separate folders."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/configuration/defaults.html — read 2026-07-30.

The directory-layout page is cited for Linux `$HOME/.config/blender/5.1/`, macOS `/Users/$USER/Library/Application Support/Blender/5.1/`, and Windows `%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.1\`, but the finding does not quote the page's words.

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — read 2026-07-30.

**Not established by the words quoted here:** whether two builds carrying the same two-number version share one user directory. The earlier finding inferred this from the cited paths, but it supplied no exact quote for those paths. It therefore cannot carry the inference as a traceable finding. The hand test below remains the way to settle it.

**Inferred (step written out): a second install can take over the file association, which is one setting per user or per machine.** The documented words are:

> "**Operating System Settings**
> Make this installation your default Blender (MS-Windows & Linux only)."
> "**Register** — Make the currently in use Blender installation the default for generating thumbnails and the default for opening blend-files."
> "**Unregister** — Remove file association & thumbnailer."
> "**For All Users** — Register Blender for all users, requires escalated privileges."

Source: manual — https://docs.blender.org/manual/en/5.1/editors/preferences/system.html — read 2026-07-30.

The step: "the default" is singular, so registering a second Blender makes it the default and the first is no longer it. The sources never say what happens to the earlier registration.

The Windows installer does this without being asked:

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html — read 2026-07-30.

**Documented: on macOS, installing over the top removes what was there.**

> "Install the new version by overwriting the current Blender.app in the Applications folder. You can rename Blender.app or place it in a different folder to have more than one version at a time."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/macos.html — read 2026-07-30.

**Documented: an imported configuration can break the newer Blender.**

> "Some previous Blender add-ons and extensions may not be compatible with a new version of Blender, and choosing this option may lead to errors on startup. If this occurs, the recommended first step is to try Loading Factory Settings."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/configuration/defaults.html — read 2026-07-30.

**Inferred (step written out): the environment variables are not per-install.** `BLENDER_USER_RESOURCES`, `BLENDER_SYSTEM_SCRIPTS`, `BLENDER_SYSTEM_EXTENSIONS`, `BLENDER_SYSTEM_PYTHON`, `BLENDER_CUSTOM_SPLASH` and the rest are documented as environment variables on https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html (read 2026-07-30), for example:

> "BLENDER_USER_RESOURCES: Replace default directory of all user files. Other BLENDER_USER_* variables override when set."

The step: an environment variable belongs to the process environment, not to a particular installation, so one set for the benefit of one Blender is inherited by every Blender started in that environment. The sources do not spell this out.

**Not covered / not in the sources.** Nothing says whether one install can corrupt another's files; nothing about two Blenders running at once contending for a lock, a temporary directory or a GPU; nothing about a `PATH` collision when two are on the path; nothing about a Snap or Store install colliding with an extracted one. Searched `manual_5.1` and `api_5.1` for `lock file`, `lockfile`, `already running`, `running instance`, `single instance`, `file in use`, `conflict`: no hits on point.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. This settles the point left open above — whether two builds carrying the same two-number version share one user directory.

*Set-up.* A second copy of the installed application was made in a working folder outside `/Applications`, with `cp -Rc "/Applications/Blender 5.1.2.app" <working folder>/Blender.app`. That gave two copies of Blender 5.1.2 on one computer, in two different places.

*Action.* This was run against each of the two, changing only the path:

`"<path to the copy>/Contents/MacOS/Blender" --background --python-expr "import bpy; print('USER', bpy.utils.resource_path('USER')); print('LOCAL', bpy.utils.resource_path('LOCAL')); print('SYSTEM', bpy.utils.resource_path('SYSTEM')); print('BINARY', bpy.app.binary_path)"`

*What came back.* From the one in `/Applications`:

> ```
> USER /Users/artist/Library/Application Support/Blender/5.1
> LOCAL /Applications/Blender 5.1.2.app/Contents/Resources/5.1
> SYSTEM /Library/Application Support/Blender/5.1
> BINARY /Applications/Blender 5.1.2.app/Contents/MacOS/Blender
> ```

From the second copy, in the working folder:

> ```
> USER /Users/artist/Library/Application Support/Blender/5.1
> LOCAL <working folder>/Blender.app/Contents/Resources/5.1
> SYSTEM /Library/Application Support/Blender/5.1
> BINARY <working folder>/Blender.app/Contents/MacOS/Blender
> ```

The two copies printed the same USER path and the same SYSTEM path; only LOCAL followed the copy. Listing `~/Library/Application Support/Blender/` gave one folder per two-number version and no more: `4.1`, `4.2`, `4.3`, `4.4`, `4.5`, `5.0`, `5.1`, `5.2`, `data`.

*On the registration arguments.* `--help` was run on the Mac build. All four are printed in its own help, each with a platform note, word for word:

> ```
> -r or --register 
> 	Register blend-file extension for current user, then exit (Windows & Linux only).
> 
> --register-allusers 
> 	Register blend-file extension for all users, then exit (Windows & Linux only).
> 
> --unregister 
> 	Unregister blend-file extension for current user, then exit (Windows & Linux only).
> 
> --unregister-allusers 
> 	Unregister blend-file extension for all users, then exit (Windows & Linux only).
> ```

None of them was run, because doing so would change which application opens a file type on the Creator's own computer.

**Observed — hand test, 2026-08-05.** The Creator ran this one, on their own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. They opened Blender 5.1.2 in the ordinary way, opened Edit ‣ Preferences, clicked System in the list down the left-hand side, and sent a screenshot of that page.

The screenshot shows the System page with five sections, in this order: `Cycles Render Devices`, `Network`, `Memory & Limits`, `Video Sequencer` and `Sound`. No section headed `Operating System Settings` appears in it, and no button named `Register`, `Unregister` or `For All Users` appears in it. One screenshot was supplied, so what it shows is the page from `Cycles Render Devices` at the top down to the last row of `Sound`; whether anything sits below `Sound` is not shown.

**Observed — hand tests, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH, with all four researched versions extracted under `/opt/blender`. The registration controls are marked "MS-Windows & Linux only", so this is the first machine on which they could be used at all. It is a rented machine that is thrown away, which is why changing a file-type association was acceptable here and was not on the Creator's own computer. Everything that changed is written out, and everything created was removed afterwards.

**This turns the inference above into an observation.** The step this answer writes out — that "the default" is singular, so registering a second Blender makes it the default and the first is no longer it — is what happens, and the way it happens is that the second writes over the first one's file.

*The state before anything was registered.* `xdg-mime query default application/x-blender` printed `blender.desktop`, a file the rented machine's own image had put in `/usr/share/applications/`. There was no `~/.local/share/applications`, no `~/.local/share/mime`, no `~/.config/mimeapps.list`, and no `/usr/local/share/applications`, `/usr/local/share/mime` or `/usr/local/share/thumbnailers`. Every file under `~/.local/share` and `/usr/local/share` was listed and kept.

*Registering the first Blender.* `/opt/blender/blender-4.5.12-linux-x64/blender --register` printed, word for word:

> ```
> Register: /opt/blender/blender-4.5.12-linux-x64/blender
> - Setup symbolic-links in: ~/.local/bin
> The PATH environment variable doesn't contain "/root/.local/bin", not creating symlinks
> - Setup icon: ~/.local/share/icons/hicolor/scalable/apps/blender.svg
> - Setup desktop-file: ~/.local/share/applications/blender.desktop
> - Setup mime type: ~/.local/share/mime/application/x-blender.xml
> - Setup mime type as default
> ```

with exit code 0. Sixteen files appeared under `~/.local/share`, and `~/.config/mimeapps.list` was created reading `application/x-blender=blender.desktop`. The desktop file it wrote carries `Exec=/opt/blender/blender-4.5.12-linux-x64/blender %f`.

*Registering a second Blender afterwards.* `/opt/blender/blender-5.2.0-linux-x64/blender --register` printed the same shape of output with its own path on the first line. **No new file appeared** — the set of files was identical afterwards. What changed is the content of the one file both wrote:

> `Exec=/opt/blender/blender-5.2.0-linux-x64/blender %f`

So the second registration replaced the first. Because both write a file of the same name, `xdg-mime query default application/x-blender` answers `blender.desktop` throughout; the same word, a different program behind it.

*Registering for all users.* `/opt/blender/blender-5.1.2-linux-x64/blender --register-allusers` printed:

> ```
> Register: /opt/blender/blender-5.1.2-linux-x64/blender
> - Setup symbolic-links in: /usr/local/bin
> - Setup icon: /usr/local/share/icons/hicolor/scalable/apps/blender.svg
> - Setup desktop-file: /usr/local/share/applications/blender.desktop
> - Setup mime type: /usr/local/share/mime/application/x-blender.xml
> - Setup thumbnailer: /usr/local/share/thumbnailers/blender.thumbnailer
> ```

**This is the manual's "Files are setup files under: /usr/local for all users, otherwise ~/.local is used" happening**, and it does two things the current-user form did not: it made the links in `/usr/local/bin`, and it set up a thumbnailer — the thumbnailer the manual's `Register` description mentions.

*Unregistering.* `/opt/blender/blender-5.0.1-linux-x64/blender --unregister` printed:

> ```
> Unregister: /opt/blender/blender-5.0.1-linux-x64/blender
> - Remove symbolic-links in: ~/.local/bin
> - Remove icon: ~/.local/share/icons/hicolor/scalable/apps/blender.svg
> - Remove desktop-file: ~/.local/share/applications/blender.desktop
> - Remove mime type: ~/.local/share/mime/application/x-blender.xml
> - Remove mime type as default
> ```

That build had never registered anything, and it removed the registration another build had made. `--unregister` clears whatever registration is there, not only its own. `--unregister-allusers` from this version's build did the same for the all-users side, thumbnailer included.

*What unregistering leaves behind.* Set against the file list taken at the start, unregistering did **not** put the machine back. Twelve rebuilt mime-database files were left in `~/.local/share/mime` and twelve more in `/usr/local/share/mime`; the folders `applications`, `icons`, `mime` and `thumbnailers` were left in place though empty; and `~/.config/mimeapps.list` was left still naming `blender.desktop`. All of it was removed by hand afterwards, and the machine's file list then matched the earlier one exactly.

*Whether the Preferences panel of these controls exists on Linux.* All four Linux builds were asked, with no window, to list the preferences panels they register; all four register a panel labelled `Operating System Settings`. Opened with a window and photographed, Blender 5.1.2's Preferences System page shows, in order: `Cycles Render Devices`, `Display Graphics`, `Operating System Settings`, `Network`, `Memory & Limits`, and more below. The `Operating System Settings` section reads `Open blend files with this Blender version` above two buttons, `Register` and `Unregister`, with a checkbox `For All Users` beneath — the controls the manual quotes, word for word in the case of the buttons. **On the Creator's Mac neither that section nor `Display Graphics` appears.**

*Two builds of one version, in two places.* The answer above says the earlier finding's inference about this could not be carried because it supplied no quote for the paths, and that a hand test remained the way to settle it. Here it is on a second machine. A second copy of this build was made with `cp -a` into a working folder. Run with `BLENDER_USER_RESOURCES` deliberately unset and with `--factory-startup`, so that the default path came back and nothing was saved, the two printed:

> ```
> USER /root/.config/blender/5.1
> LOCAL /opt/blender/blender-5.1.2-linux-x64/5.1
> SYSTEM ''
> BINARY /opt/blender/blender-5.1.2-linux-x64/blender
> ```

> ```
> USER /root/.config/blender/5.1
> LOCAL /root/smoozy/Builds/work/06/5.1/copy/5.1
> SYSTEM ''
> BINARY /root/smoozy/Builds/work/06/5.1/copy/blender
> ```

The same user path, only the build's own folder following the copy. That agrees with the Mac, and the path is the Linux one the layout page is cited for. One difference from the Mac: the system path comes back as an **empty string** here, where the Mac returned a real path.

*The environment variables.* The answer above infers that `BLENDER_USER_RESOURCES` and its relatives belong to whatever starts Blender rather than to an installation. Part of that is now observed: setting `BLENDER_USER_RESOURCES` to a folder of this test's own moved the user path there and left the build's own folder and the system path alone, on every one of the four builds. Whether one set for one Blender is inherited by another Blender started beside it follows from how environments work rather than from anything Blender printed, so that step stays inferred.

*Two builds of one version from two different sources.* A Blender 5.2.0 was also installed through Snap on this machine, alongside the blender.org 5.2.0 extracted under `/opt/blender`. Both report the same user path, `/root/.config/blender/5.2`. So a package-manager build and a hand-extracted one carrying the same two-number version do share one settings folder — which the "Not covered" note above raises and leaves open. It is recorded in full under question 5 of the 5.2 findings.

**Outside the interface. Observed on two machines.** The documented co-existence facts are about files on disk, not the interface. Both runs above were made with `--background` and no window, and both returned the same USER path. **Inferred:** the documented environment variables also affect a Blender started outside the interface, because the sources describe them as command-line environment variables; they do not state the conclusion directly. Whether same-version builds share one user directory is not established by the sources, but is now **observed** in the test above: two copies of 5.1.2 in two places both reported the same user directory, with no window open. The "Loading Factory Settings" remedy quoted above is an interface action; its command-line counterpart, `--factory-startup`, is documented on the arguments page as "Skip reading the startup.blend in the users home directory". File association concerns opening a file through the operating system, not a Blender launched directly by a program.

**Operating system. Observed for macOS and for Linux.** Differs sharply, and both sides of the registration question are now looked at: the Mac build's own `--help` marks all four registration arguments "(Windows & Linux only)", and the screenshot of the Preferences System page on the Creator's Mac shows no `Operating System Settings` section among the five sections it covers. **Observed on the rented Linux machine, 2026-08-06:** all four registration arguments run there and do what they say, writing under `~/.local` for one user and `/usr/local` for all exactly as the manual states; the `Operating System Settings` section is on the Preferences System page with `Register`, `Unregister` and `For All Users` in it; and **registering a second Blender does take the association from the first, by writing over its file**. Windows remains untested. Windows: the installer registers associations and needs administrator rights; the zip does not. macOS: no registration mechanism is described, and the documented update route is overwriting `Blender.app`. Linux: registration is described as "Files are setup files under: /usr/local for all users, otherwise ~/.local is used", and "On Linux, if Blender is installed from a package manager such as Snap, file association is handled by the package manager." Register/Unregister is documented as "MS-Windows & Linux only", and the matching command-line arguments `-r, --register`, `--register-allusers`, `--unregister`, `--unregister-allusers` are each marked "(Windows & Linux only)".

---

## 6. Does installing a Blender disturb a render already running on the same computer?

**Label: not in the sources.**

Nothing in the manual, the Python API documentation, the release notes or Cycles' own files addresses what happens to a render in progress while a Blender is being installed.

**Where I looked.** The four installing pages (https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html and its `linux.html`, `macos.html`, `windows.html`) and `steam.html` — read in full; each describes copying, extracting or running an installer and says nothing about other processes. The production-deployment page (https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html) — read in full; it covers automated deployment, offline working, bundling extensions and scripts, and never mentions running jobs. The directory-layout page (https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html) — read in full. The crash page (https://docs.blender.org/manual/en/5.1/troubleshooting/crash.html) — lists "Running out of memory", "Issues with graphics hardware or drivers" and "Bugs in Blender" as the causes of crashes; installation is not among them. The command-line rendering pages. Full-text searches across `manual_5.1`, `api_5.1`, `release_notes_5.1` for: `interrupt`, `terminate`, `abort`, `kill`, `in use`, `file in use`, `already running`, `running instance`, `single instance`, `concurrent`, `simultaneous`, `lock file`. The `abort` hits are about cancelling a render with Esc or the status-bar cancel button, and about `--disable-abort-handler`; the `interrupt` hits are about a person choosing to stop a render (https://docs.blender.org/manual/en/5.1/render/output/animation.html: "May need to interrupt rendering to use the computer, and want to be able to resume rendering where you left off"). None of them concern installation.

The nearest thing the sources say — and it is not an answer — is that an extracted Blender is self-contained: "Blender downloads can be extracted to any directory on the system, as a self contained installation" (deploying page, read 2026-07-30).

**Not covered by the sources.** Everything the question asks. In particular the sources do not distinguish installing *over* the exact files a render is running from, versus installing beside them. Both of those cases were tested by hand, below.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The installed Blender in `/Applications` was never touched; both runs used a throwaway copy in a working folder outside the project.

*Set-up, for both runs.* A copy of Blender 5.1.2 was made into an empty working folder with `cp -Rc`, with an empty output folder beside it. The scene was Blender's own factory startup scene — no file was opened. The render was started from the copy, with the window switched off:

`"<working folder>/Blender.app/Contents/MacOS/Blender" --background --factory-startup --python-expr "import bpy; bpy.context.scene.cycles.samples=64; bpy.context.scene.render.resolution_percentage=50" -E CYCLES -F PNG -o "<working folder>/out/frame_" -s 1 -e 30 -a`

*First run — a second Blender put beside the running one.* Once four image files had appeared, `cp -R "/Applications/Blender 4.5.12.app" "<working folder>/second/Blender.app"` was run in another terminal. It finished with exit code 0.

> ```
> ### image files on disk before the action: 4 at 21:23:50
> ### action exit code 0
> ### action finished at 21:23:54
> ### image files just after the action: 5
> ### render process ended, exit code 0 at 21:24:55
> ### image files at the end: 30
> ```

All thirty numbered images were written, ending `frame_0028.png`, `frame_0029.png`, `frame_0030.png`. The render printed no error.

*Second run — copied over the exact folder the render was running from.* Same set-up, a fresh working folder. Once four image files had appeared, `cp -R "/Applications/Blender 5.1.2.app/" "<working folder>/Blender.app/"` was run — the contents of the installed 5.1.2 copied straight over the folder the render was reading from. It finished with exit code 0.

> ```
> ### image files on disk before the action: 4 at 21:29:58
> ### action exit code 0
> ### action finished at 21:30:03
> ### image files just after the action: 5
> ### render process ended, exit code 0 at 21:30:54
> ### image files at the end: 30
> ```

All thirty numbered images were written. The render printed no error.

**Observed — hand tests, 2026-08-06, on the rented Linux machine. The two machines disagree on the second run, and neither result is set aside.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH, shared with other work at the time, so nothing here says how long anything took. Nothing under `/opt/blender` was touched; both runs used a throwaway copy made with `cp -a`. **This is exactly the distinction the "Not covered" note above asks for — installing over the files a render is running from, against installing beside them — and on this machine the two come out differently.**

*Set-up, for both runs.* A copy of the extracted Blender 5.1.2 into an empty working folder, an empty output folder beside it, Blender's own factory startup scene, and the render started with no window:

`<working folder>/blender/blender --background --factory-startup --python-expr "import bpy; bpy.context.scene.cycles.samples=64; bpy.context.scene.render.resolution_percentage=50" -E CYCLES -F PNG -o "<working folder>/out/frame_" -s 1 -e 30 -a`

*First run — a second Blender put beside the running one.* Once four image files had appeared, `cp -a /opt/blender/blender-5.2.0-linux-x64 <working folder>/second/blender` was run:

> ```
> ### image files on disk before the action: 4
> ### the copy's own exit code: 0
> ### is the second Blender there? /root/smoozy/Builds/work/render2/5.1/A/second/blender
> ### image files just after the action: 4
> ### render process ended, exit code 0
> ### image files at the end: 30
> ```

Its last lines were `01:21.205  render | Fra: 30 | Mem: 39M | Finished`, `Saved: '<working folder>/out/frame_0030.png'`, `Time: 00:02.27 (Saving: 00:00.04)` and `Blender quit`. No line contained the word error. **This agrees with the Mac.**

*Second run — copied over the exact folder the render was running from.* Once four image files had appeared, `cp -a /opt/blender/blender-5.1.2-linux-x64/. <working folder>/blender/` was run. The copy finished with exit code 0. What came back:

> ```
> ### image files on disk before the action: 4
> Bus error               (core dumped)
> ### action: the same Blender copied straight over the folder the render is running from; exit code 0
> ### image files just after the action: 4
> ### render process ended, exit code 135
> ### image files at the end: 4
> ### last file: frame_0004.png
> ```

The last five lines the render printed:

> ```
> 00:10.128  render           | Fra: 5 | Mem: 2M | Updating Device | Writing constant memory
> 00:10.128  render           | Fra: 5 | Mem: 2M | Loading denoising kernels (may take a few minutes the first time)
> 00:10.128  render           | Fra: 5 | Mem: 2M | Sample 0/64
> 00:10.154  render           | Fra: 5 | Remaining: 00:01.63 | Mem: 39M | Sample 1/64
> Writing: /tmp/blender.crash.txt
> ```

So this version's run also wrote a crash report of its own, at `/tmp/blender.crash.txt` — the only one of the four that did. Blender printed no error text beyond that line; the words `Bus error (core dumped)` came from the shell that started it. Four images were on disk at the end instead of thirty, and the four written before the action were still there.

**The same test on the Creator's Mac finished all thirty frames with exit code 0 and printed no error. On this machine it stopped the render, in this version and in all three others.** Both results stand; neither is chosen between.

**Outside the interface. Observed on two machines.** The sources are silent in both modes. Every run on both machines was a background render started from a terminal with no window.

**Operating system. Observed for macOS and for Linux, and they do not agree.** The sources do not say. The documented install actions differ by OS (extract-and-copy on Linux; overwrite `Blender.app` on macOS; an installer, a zip or the Store on Windows), so a single answer for all three would not follow from anything here even if the behaviour were documented — and the two machines that were tried bear that out. On macOS 26.5.2 with an Apple M1 Pro chip, both runs completed all thirty frames. On the rented Linux machine, putting a second Blender beside the running one left the render untouched, and copying a Blender over the folder the render was running from stopped the render with exit code 135 and four frames of thirty written. Windows remains untested.

---

## 7. Can a computer be asked which Blenders it holds?

**Label: not in the sources**, for a way provided by Blender. Two partial documented facts sit beside it.

Nothing in the sources describes a command, file, registry entry or API that lists the Blender installations present on a machine.

**Partial one — a Linux package manager can list what it installed.**

> "Installing Blender via the distribution's native mechanisms ensures consistency with other packages on the system and may provide other features (given by the package manager), such as listing of packages, update notifications and automatic menu configuration."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html — read 2026-07-30. This is a property of the package manager, not of Blender, and it would only ever cover installs made through it.

**Partial two — a running Blender can say where it is.**

> "bpy.app.binary_path — The location of Blender's executable, useful for utilities that open new instances."

Source: Python API — https://docs.blender.org/api/5.1/bpy.app.html — read 2026-07-30. That answers "where is *this* one", not "what else is here".

**A near miss I checked and am not counting.** `bpy.utils.resource_path` takes a version:

> "bpy.utils.resource_path(type, *, major=bpy.app.version[0], minor=bpy.app.version[1])
> Return the base path for storing system files.
> Parameters: type (Literal['USER', 'LOCAL', 'SYSTEM']) – The resource type. major (int) – major version, defaults to current. minor (int) – minor version, defaults to current."

Source: Python API — https://docs.blender.org/api/5.1/bpy.utils.html — read 2026-07-30. It returns a path for a version you name. The documentation does not say it checks whether anything is there, so I am not treating it as a way to discover other installations.

**Where I looked.** Full-text searches across `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` for: `which blender`, `installed versions`, `list versions`, `enumerate`, `detect installed`, `version manager`, `multiple versions`, `side by side`, `co-exist`. Pages read in full and ruled out: the directory-layout page, the deploying-in-production page, the four installing pages, the Steam page, the command-line arguments page (no argument enumerates installations), and the Help / Save System Info page (which reports on the running Blender only: "This section shows you the Blender version, details about the build configuration, and the path in which Blender is running").

The layout page does give a fixed, per-version place to look, which is the raw material somebody would use to build such a check themselves — Linux `./5.1/` beside the executable, macOS `./Blender.app/Contents/Resources/5.1/`, Windows `./5.1/` or `%ProgramFiles%\Blender Foundation\Blender\5.1\` — but it describes where one installation keeps its files, not how to find installations.

**Not covered by the sources.** Whether Blender writes any machine-wide record of itself on any OS; whether the Windows installer's uninstall entry can be queried; whether Snap, Steam or the Microsoft Store expose their Blender entries in a documented way.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2.

*What Blender itself offers.* `"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -c help` printed:

> ```
> Blender Command Listing:
> 	extension
> ```

So the complete list of commands this build offers is one, and it is about extensions — nothing that enumerates installations.

*What the computer can be asked.* The computer held four Blenders in `/Applications` — 4.5.12, 5.0.1, 5.1.2 and 5.2.0 — and, at the same time, four further copies of the same four builds sitting in ordinary folders under a working directory at `/private/tmp/…/hand-tests/Builds/06/`, made with `cp -Rc`. Three ways of asking were tried in a terminal.

`ls -1 /Applications | grep -i blender` printed:

> ```
> Blender 4.5.12.app
> Blender 5.0.1.app
> Blender 5.1.2.app
> Blender.app
> ```

`mdfind "kMDItemFSName == 'Blender*.app'"` printed:

> ```
> /Applications/Blender.app
> /Applications/Blender 4.5.12.app
> /Applications/Blender 5.0.1.app
> /Applications/Blender 5.1.2.app
> ```

`system_profiler SPApplicationsDataType` printed one block per Blender. The 5.1 block, word for word:

> ```
>     Blender 5.1.2:
> 
>       Version: 5.1.2
>       Obtained from: Identified Developer
>       Last Modified: 2026-05-19, 03:31
>       Kind: iOS
>       Signed by: Developer ID Application: Stichting Blender Foundation (68UA947AUU), Developer ID Certification Authority, Apple Root CA
>       Location: /Applications/Blender 5.1.2.app
>       Get Info String: 5.1.2 2026-05-19, Blender Foundation
> ```

Blocks in the same shape were printed for 4.5.12, 5.0.1 and the one at `/Applications/Blender.app` (shown as `Version: 5.2.0`). None of the three commands named any of the four copies sitting in the working folder. Asking the index about that folder directly, `mdfind -onlyin <working folder> "kMDItemFSName == 'Blender.app'"`, returned nothing at all.

*What a running Blender says about where it is.* `bpy.app.binary_path` read `'/Applications/Blender 5.1.2.app/Contents/MacOS/Blender'`, and the system-information file under question 3 carries the same path on its `binary path:` line. Both report the one Blender that produced them.

**Observed — hand test, 2026-08-06, on the rented Linux machine. This settles the Linux half of the question**, which "partial one" above only pointed at. Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH.

*Set-up.* The machine held **five** Blenders when this was run: the four researched versions extracted under `/opt/blender` and a fifth that came with the rented image at `/opt/blender-5.0.0-linux-x64`, with `/usr/bin/blender` pointing at it. None had been installed by a package manager.

*Action and what came back.* `apt list --installed | grep -i blender` printed nothing at all. `dpkg -l blender` printed:

> `dpkg-query: no packages found matching blender`

`snap list` printed eight entries and no blender among them. `which -a blender` printed one line, `/usr/bin/blender`, which `readlink -f` resolves to `/opt/blender-5.0.0-linux-x64/blender`. `update-alternatives --list blender` printed:

> `update-alternatives: error: no alternatives for blender`

A plain search of the file system, `find / -maxdepth 6 -name blender -type f -perm -u+x`, printed all five:

> ```
> /opt/blender/blender-5.2.0-linux-x64/blender
> /opt/blender/blender-5.0.1-linux-x64/blender
> /opt/blender/blender-5.1.2-linux-x64/blender
> /opt/blender/blender-4.5.12-linux-x64/blender
> /opt/blender-5.0.0-linux-x64/blender
> ```

So the documented partial is real but as narrow as its own wording: **a package manager names only what it installed**, and none of these five was installed that way.

*After a package-managed Blender was added.* A Blender 5.2.0 was installed with `snap install blender --classic`. `snap list` then named it, and only it:

> ```
> blender  5.2.0  7599  latest/stable  blenderfoundation**  classic
> ```

`apt list --installed` still named nothing. `which -a blender` then printed three paths — `/usr/bin/blender`, `/bin/blender` and `/snap/bin/blender`. The listing grew by exactly the one build the package manager installed, and the other five stayed invisible to it.

*The fixed per-version place the layout page gives.* The answer above records that page as the raw material somebody would use to build such a check. On this machine that is what is there: each extracted folder holds a folder named for its two-number version beside the program — `4.5`, `5.0`, `5.1`, `5.2`. It is still a place to look rather than a way to find installations, and every version's manual also says a download can go in any folder at all.

*What a running Blender says about where it is.* `bpy.app.binary_path` read `'/opt/blender/blender-5.1.2-linux-x64/blender'`, and the system-information file under question 3 carries the same path on its `binary path:` line. Both report the one Blender that produced them, exactly as on the Mac.

**Outside the interface. Observed on two machines.** The sources describe no such facility in either mode. Every command above was run from a terminal with no window open on both machines, and `binary_path` was read with `--background` on both.

**Operating system. Observed for macOS and for Linux.** The package-manager partial is Linux-only, by the page it is on. Nothing equivalent is documented for macOS or Windows; the three macOS commands recorded above are macOS's own, not Blender's. **Observed on the rented Linux machine, 2026-08-06:** the documented Linux partial works, and it names only what it installed — `apt` and `snap` named none of the five Blenders on the machine until one was put there through Snap, and then named exactly that one. The two machines agree on the shape of the answer: the operating system's own tools name what was put there their way, and nothing else. Windows remains untested.

---

## 8. What becomes of a render that is running when the Blender it is using is removed?

**Label: not in the sources.**

The sources do not describe removing Blender at all, beyond a single sentence about the Windows uninstall route, and they never connect removal to a running render.

The one sentence about removal:

> "To remove a previously installed version of Blender, use Windows settings or control panel to uninstall the desired version."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html — read 2026-07-30. It says nothing about anything that might be running.

**Where I looked.** Full-text searches across `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` for: `uninstall`, `remove Blender`, `delete`, `terminate`, `kill`, `interrupt`, `abort`, `in use`, `file in use`, `running instance`, `already running`. The `uninstall` hits are: the graphics-driver advice on the GPU troubleshooting pages, one line about an extension's directory ("This directory will be kept between upgrades but will be removed if the extension is uninstalled" — https://docs.blender.org/manual/en/5.1/advanced/extensions/addons.html), and the Windows sentence above. Nothing else. Pages read in full and ruled out: the four installing pages, the Steam page, the deploying-in-production page, the crash page, the directory-layout page, the command-line rendering page.

**What the sources do say nearby, which does not answer it.** A render can be stopped deliberately, and an animation rendered to images can be picked up again:

> "As with a normal render, you can abort it with Esc" (https://docs.blender.org/manual/en/5.1/editors/3dview/viewport_render.html)
> "The task can be aborted by clicking the cancel button ()." (https://docs.blender.org/manual/en/5.1/interface/window_system/status_bar.html)
> "when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range" (https://docs.blender.org/manual/en/5.1/render/output/introduction.html)
> "May need to interrupt rendering to use the computer, and want to be able to resume rendering where you left off." (https://docs.blender.org/manual/en/5.1/render/output/animation.html)

All read 2026-07-30. Every one of them is about a person choosing to stop, not about the program being taken out from under a render, so none of them answers the question.

**Not covered by the sources.** Everything the question asks: whether the running process survives, whether already-written frames survive, whether anything is written to a log. All three were tested by hand, below, for the deletion case on macOS.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The installed Blender in `/Applications` was never touched; the test used a throwaway copy in a working folder outside the project, and nothing outside that folder was deleted.

*Set-up.* A copy of Blender 5.1.2 was made into an empty working folder with `cp -Rc`, with an empty output folder beside it. The scene was Blender's own factory startup scene. The render was started from the copy, with the window switched off:

`"<working folder>/Blender.app/Contents/MacOS/Blender" --background --factory-startup --python-expr "import bpy; bpy.context.scene.cycles.samples=64; bpy.context.scene.render.resolution_percentage=50" -E CYCLES -F PNG -o "<working folder>/out/frame_" -s 1 -e 30 -a`

*Action.* Once four image files had appeared, `rm -rf "<working folder>/Blender.app"` was run in another terminal — the whole application the render was running from, deleted.

*What came back.*

> ```
> ### image files on disk before the action: 4 at 21:35:33
> ### action exit code 0
> ### does the folder still exist? ls: <working folder>/Blender.app: No such file or directory
> ### action finished at 21:35:34
> ### image files just after the action: 4
> ### render process ended, exit code 0 at 21:36:28
> ### image files at the end: 30
> ```

The deletion was permitted and completed at once; the application folder was gone. The render carried on and wrote all thirty numbered images, ending `frame_0028.png`, `frame_0029.png`, `frame_0030.png`, then ended by itself with exit code 0. It printed no error. The four images written before the deletion were still there afterwards.

*What was left afterwards.* `ls ~/Library/Application\ Support/Blender/5.1` printed `config`, `datafiles`, `extensions`, `scripts` — the settings folder survived the application being deleted.

**Observed — hand test, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Nothing under `/opt/blender` was touched; the test used a throwaway copy made with `cp -a`, and nothing outside the working folder was deleted. **This is the Linux case the quoted launch instruction describes and the sources leave open.**

*Set-up and action.* As on the Mac: a copy of the extracted Blender 5.1.2, an empty output folder, Blender's own factory startup scene, and the same thirty-frame background render command quoted under question 6 of this document. Once four image files had appeared, `rm -rf <working folder>/blender` was run — the whole folder the render was running from, deleted.

*What came back.*

> ```
> ### image files on disk before the action: 4
> ### action: the whole Blender folder the render is running from, deleted; exit code 0
> ### does the folder still exist? ls: cannot access '/root/smoozy/Builds/work/render/5.1/C/blender': No such file or directory
> ### image files just after the action: 4
> ### render process ended, exit code 0
> ### image files at the end: 30
> ### last file: frame_0030.png
> ```

The last lines the render printed:

> ```
> 01:10.542  render           | Fra: 30 | Mem: 39M | Finished
> 01:10.628  render           | Saved: '<working folder>/out/frame_0030.png'
> 01:10.628  render           | Time: 00:02.30 (Saving: 00:00.04)
> 
> Blender quit
> ```

No line contained the word error. That version's settings folder was still there afterwards and still held its contents.

**This agrees with the Creator's Mac**, in this version and in the other three: the deletion is permitted, the render carries on, all thirty frames are written, the frames written before the deletion survive, and the settings folder outlives the program.

**Outside the interface. Observed on two machines.** The manual documents starting Blender from its installation directory — "Open a terminal, then go to the directory where Blender is installed, and run Blender like this: cd <blender installation directory> ; ./blender" (https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/linux.html). Those words do not establish what happens to that running Blender when its files are removed. The tests answer it for a background render on both machines: each was started with `--background` from a terminal with no window, and each finished after its own program had been deleted.

**Operating system. Observed for macOS and for Linux, and they agree.** The only removal instruction the sources give is Windows-specific. On macOS 26.5.2 the deletion was allowed while the render was running, and the render finished. On the rented Linux machine the same held. Windows remains untested, and there is no uninstaller route on either computer to compare against.

---

## 9. Can installing or removing a Blender be done again safely on a computer where it was cut off part-way?

**Label: not in the sources.**

Nothing in the manual, the Python API documentation, the release notes or Cycles' own files discusses an interrupted install or uninstall, repeating one, or repairing after one.

**Where I looked.** Full-text searches across `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` for: `idempot`, `retry`, `re-run`, `rollback`, `roll back`, `atomic`, `partial`, `partially`, `incomplete install`, `corrupt`, `repair`, `reinstall`, `interrupt`, `resume`. Every hit was on an unrelated subject — `re-run` about re-running an operator, `resume` about resuming a fluid bake or an animation render, `corrupt` about graphics drivers and Dyntopo attributes, `repair` about video file headers, `reinstall` about reinstalling an operating system or graphics drivers. Pages read in full and ruled out: the four installing pages, the Steam page, the deploying-in-production page, the directory-layout page, the crash page, the recover page index, the command-line arguments page.

**What the sources establish that bears on it, without answering it.** Two of the three install routes are plain file operations rather than a stateful installer:

> "Blender downloads can be extracted to any directory on the system, as a self contained installation."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30.

> "Download the Linux version for your architecture and decompress the file to the desired location (e.g. ~/software or /usr/local)."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html — read 2026-07-30.

I am deliberately not drawing "so extracting again is safe" from these. That would require assuming what a half-finished extraction leaves behind and how a second extraction treats it, and the sources say neither. The Windows installer route, by contrast, is explicitly a stateful one ("It requires administrator rights", and removal goes through "Windows settings or control panel"), and nothing describes its behaviour after an interruption.

One documented hazard that is adjacent, and is the only thing in the sources about an install going wrong:

> "**Archive Extraction** — Extracting Blender's archive using 7-zip is not supported. TAR must be used instead."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html (Known Limitations) — read 2026-07-30.

**Not covered by the sources.** Everything the question asks, for every install route. The macOS route — copying an application bundle into place — was tested by hand, below. The others were not.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. Everything happened in a working folder outside the project; the installed Blender in `/Applications` was only ever read from.

*A note on the copying tool, because it changed the test.* The first attempt used `cp -R`. A Blender application bundle copied that way would not start at all on this computer — whether the copy had been interrupted or had finished — and `codesign --verify --verbose` on a finished `cp -R` copy printed `invalid Info.plist (plist or signature have been modified)`. So the test was run again with macOS's own copying tool, `ditto`, which does produce a bundle that starts, leaving the interruption as the only thing under test. Both attempts are recorded here in the order they happened.

*The test, with `ditto`.* Into a fresh empty folder, `ditto "/Applications/Blender 5.1.2.app" <working folder>/Blender.app` was started and killed after two seconds:

> ```
> ### copy killed at 21:40:02
> ### how much arrived: 373M
> ### is there an executable? -rwxr-xr-x@ 1 artist     admin  167811856 May 19 04:15 <working folder>/Blender.app/Contents/MacOS/Blender
> ### running the half-copied Blender with --version:
> ### exit code 137
> ```

The half-copied Blender printed nothing at all and gave exit code 137. The same `ditto` command was then run again over the same folder, without deleting anything first:

> ```
> ### second copy exit code 0
> ### size now: 881M
> ```

`codesign --verify --verbose` on the finished copy printed `a sealed resource is missing or invalid`, followed by a list of `file added:` lines, all of them compiled-Python cache files under `Contents/Resources/5.1/python/`. Running that finished copy with `--version` gave exit code 0 and printed:

> ```
> Blender 5.1.2
> 	build date: 2026-05-19
> 	build time: 01:30:33
> 	build commit date: 2026-05-18
> 	build commit time: 13:43
> 	build hash: ec6e62d40fa9
> 	build branch: blender-v5.1-release
> 	build platform: Darwin
> 	build type: Release
> ```

which is the same as the installed 5.1.2 printed under question 3.

*What is left behind after a removal.* After a throwaway copy of 5.1.2 was deleted outright (see question 8), `ls ~/Library/Application\ Support/Blender/5.1` still printed `config`, `datafiles`, `extensions`, `scripts`.

**Observed — hand tests, 2026-08-06, on the rented Linux machine. This settles the Linux archive route, including the 7-zip limitation quoted above.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Both `tar` and `7z` were already on the machine; nothing was installed for this.

*Which archive was used.* No Blender archive was left on the machine, so one was fetched from Blender's own download server into the working folder purely so that there was a real archive to extract: `blender-5.2.0-linux-x64.tar.xz`. It is the 5.2 archive, not this version's — a 5.1 archive was not fetched. What is settled below is therefore about a Blender Linux archive and the tools that unpack it, which is what the quoted limitation is about, rather than about this version in particular. The file was checked against Blender's own published checksum and `sha256sum -c` printed `blender-5.2.0-linux-x64.tar.xz: OK`.

*Extracting with TAR, which the manual says must be used.* `tar -xf blender-5.2.0-linux-x64.tar.xz -C <empty folder>` finished with exit code 0, and the result started with `--version` and printed the right version.

*Extracting with 7-zip, which the manual says is not supported.* Two steps are needed, one to unpack the compression and one to unpack the archive inside it. Both printed:

> `Everything is Ok`

with exit code 0, the second reporting `Folders: 875`, `Files: 5671`. **So 7-zip says it worked.** Running the result with `--version` gave exit code 126 and the shell printed:

> `./sevenzip/blender-5.2.0-linux-x64/blender: Permission denied`

Blender itself printed nothing. The two results side by side show why:

> ```
> -rw-r--r-- 1 root root 174666336 Jul 14 01:32 sevenzip/blender-5.2.0-linux-x64/blender
> -rwxr-xr-x 1 root root 174666336 Jul 14 01:32 tar-ok/blender-5.2.0-linux-x64/blender
> ```

The 7-zip copy has lost the mark that says the file may be run. Putting it back by hand with `chmod +x` was **not** enough — the next run gave exit code 127 and printed:

> `./sevenzip/blender-5.2.0-linux-x64/blender: error while loading shared libraries: /root/smoozy/Builds/work/17/sevenzip/blender-5.2.0-linux-x64/lib/libtbb.so.12: file too short`

Counting the two folders explains that:

> ```
> tar file count: 5592
> 7z  file count: 5671
> tar symlink count: 79
> 7z  symlink count: 0
> ```

The archive holds 79 internal links and 7-zip turned every one into a plain file. So the documented limitation is real, it breaks the result in two separate ways, and **it fails silently** — the tool reports success both times. That is worth holding on to: an automated setup would have no way to tell from the tool's own output that it had produced a broken Blender.

*An extraction cut off part-way, and done again over the same folder.* `tar -xf` was started into an empty folder and killed part-way. What had arrived:

> ```
> 186M	half
> files: 769
> ```

and the main program had not been written at all — `ls` on it printed `No such file or directory`. Trying to run that path gave exit code 127, with the shell printing `No such file or directory`; Blender printed nothing, because there was no Blender there to print anything. The same `tar -xf` command was run again over the same folder, with nothing deleted first. It finished with exit code 0, and the folder then held 5592 files and 79 links — the same as a clean extraction. The result started with `--version`, and its whole output was compared line by line against the 5.2.0 build already extracted under `/opt/blender`: **identical**.

So on Linux an interrupted extraction can simply be done again over the same folder, provided TAR is the tool. That is the same answer the Mac gave for its own shape of the question, reached by a different route.

*What is left behind after a removal.* On the rented machine, after a throwaway copy of this version was deleted outright with `rm -rf` while a background render was running from it (see question 8), that version's settings folder was still there and still held its contents. The same as on the Mac.

**Outside the interface. Observed on two machines.** Nothing in the sources distinguishes an install performed by a person from one performed by a script — the deploying page's "deployment can be automated using standard file management tools" is the closest, and it says nothing about interruption. Everything above was done from a terminal on both machines, with no Blender window at any point.

**Operating system. Observed for macOS and for Linux.** Not addressed by the sources for any of the three. On macOS 26.5.2 an interrupted `ditto` copy could be repeated over the same folder and produced a Blender that started and reported the right version. On the rented Linux machine an interrupted `tar` extraction could be repeated over the same folder and produced a Blender identical to the one already installed — and the documented 7-zip limitation was confirmed, with the two ways it breaks a build written out above. Windows remains untested, and the Windows installer could not be tried on either computer.

---

## Things a later hand test could settle

The list that stood here has been worked through. What was settled is written into the answers above, marked observed. What is left open is below.

1. **What installing and removing do to a running render on Windows.** The macOS and Linux halves of both are answered under questions 6 and 8. On Windows the operating system may refuse to delete a running program's files — record which.
2. **What a daily build reports.** Run `--version` and the `bpy.app` dump from question 3 against an alpha or beta build. No such build was available on either machine. Question 2 now records that Snap carries `beta`, `candidate` and `edge` channels, which is one way such a build could be obtained.
3. **What a daily build's file is called.** Question 1 now records the file names of a finished release, read from Blender's own download server. The equivalent for a daily build is still owed.

---

2026-08-04 — Opus 5 (Claude Code) — Builds, Blender 5.1

2026-08-04 — Opus 5 (Claude Code) — wrote the hand-test results into questions 1, 2, 3, 5, 6, 7, 8 and 9, brought the outside-the-interface and operating-system lines up to date where a test settled them, and pruned the hand-test list at the end

2026-08-05 — Opus 5 (Claude Code) — wrote the Creator's two screenshot tests into questions 1, 3 and 5, brought the outside-the-interface and operating-system lines up to date where they settled them, and pruned the hand-test list at the end

2026-08-05 — Opus 5 (Claude Code) — citation check: corrected two search records, one under question 1 and one under question 3, against the files; neither conclusion changed

2026-08-06 — Opus 5 (Claude Code) — wrote the Linux hand-test results into questions 1 to 9, from a rented Linux machine; brought both standing lines up to date on every question a Linux test settled, and pruned the hand-test list at the end
