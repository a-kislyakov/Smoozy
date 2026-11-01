# Builds — Blender 5.0

**Sources consulted, and when.** Blender's manual for 5.0 (`Blender Sources/manual_5.0`, including its `_images` folder), Blender's Python API documentation for 5.0 (`Blender Sources/api_5.0`), Blender's 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own files (`Blender Sources/cycles_5.0`). These were taken and read on 2026-07-30. I also opened `Blender Sources/About this folder.md`. A requirements-page snapshot dated 2026-08-01 was present, but the research record says every source was read on 2026-07-30. Because that date cannot establish a reading of the later snapshot, no finding below relies on it.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

A note on how I searched. The manual and the API documentation were searched whole, by text, before any file was opened. Terms used across the sheet: *daily build, LTS, long-term support, release candidate, alpha, beta, stable release, build from source, version, --version, version_cycle, build_type, build_hash, build_date, build_branch, install, uninstall, reinstall, update, upgrade, portable, co-exist, coexist, side by side, side-by-side, previous version, package manager, snap, Steam, Microsoft Store, installer, zip, dmg, builder.blender.org, download.blender.org, buildbot, mirror, JSON, REST, feed, RSS, manifest, checksum, sha256, torrent, wget, curl, atomic, rollback, resume, interrupt, in use, locked file, running instance, already running, while rendering, during a render, restart Blender, installed versions, list of installed, which versions are installed, binary_path, arm64, Intel Mac, platform*. Cycles' files were searched for `bl_info`, `version`, and `blender`.

---

## 1. What kinds of build does Blender publish, and how is each kind named and marked?

**Label: documented.**

The manual's Installing page names four things you can obtain, and describes each. Exact words:

> "Blender offers a variety of different binary packages to choose from depending on their level of stability."

> "Stable Release
>
> A package that contains the latest features and is considered stable without regressions. A new stable version is available roughly every three months."

> "Long-term Support
>
> A package designed for long-lasting projects requiring a very stable version of Blender. LTS releases are supported for two years and will not have any new features, API changes or improvements. A new long-term support version is available every year. These LTS releases will occasionally have minor patches (such as 4.2.6) which improve stability or fix critical bugs."

> "Daily Builds
>
> A package updated daily to include the newest changes in development. These versions are automatically built on a schedule. They are not as thoroughly tested as the release types above, and might break or crash. Builds marked as Alpha are still undergoing major changes and feature additions, while those marked Beta are feature-complete and are under development for refinement and stability."

> "Stability can be expected to increase from Alpha to Beta to Release Candidate (RC) to a final release."

> "Build from Source
>
> Blender's source code is available for free to either reference or to build and use."

Source: manual — https://docs.blender.org/manual/en/5.0/getting_started/installing/index.html — read 2026-07-30.

**How the kinds are named and marked.**

*By word in the version name.* The manual's own version list writes LTS versions with the letters after the number: "Blender 4.5 LTS", "Blender 4.2 LTS", "Blender 3.6 LTS", "Blender 3.3 LTS", "Blender 2.93 LTS", "Blender 2.83 LTS", beside plain entries such as "Blender 4.4" and "Blender 4.3". Source: manual — https://docs.blender.org/manual/en/5.0/versions.html — read 2026-07-30.

*By the third number.* LTS patches are written as a third number, "(such as 4.2.6)" (quoted above). The 5.0 release notes use the same shape for what they call corrective releases: "## Blender 5.0.1 / Released on December 16th 2025, Blender 5.0.1 features 132 bug fixes:". Source: release notes — https://developer.blender.org/docs/release_notes/5.0/corrective_releases/ — read 2026-07-30. The release-notes index links that page under the heading "## Corrective Releases" and states "Blender 5.0 was released on November 18, 2025." Source: release notes — https://developer.blender.org/docs/release_notes/5.0/ — read 2026-07-30.

*By a word the running program reports.* The Python API gives the mark directly:

> "bpy.app.version_cycle
>
> The release status of this build alpha/beta/rc/release"

and separately

> "bpy.app.build_type
>
> The type of build (Release, Debug)"

Source: Python API — https://docs.blender.org/api/5.0/bpy.app.html — read 2026-07-30. Note these are two different marks: `version_cycle` carries alpha/beta/rc/release; `build_type` carries Release/Debug.

*By a word on screen.* The splash screen picture in the manual, `_images/interface_splash_current.webp` (shown on https://docs.blender.org/manual/en/5.0/interface/window_system/splash.html), has "5.0.0 Alpha" printed in the top right corner of the splash image. The manual's text for that page says "The upper part of the splash screen contains the splash image with the Blender version in the top right." Label for the picture: documented (it is part of the manual). Read 2026-07-30.

*By what the download is, per platform.* Windows: "Download the zip-file or Windows Installer File" and "Blender supports both x64 and arm64 architectures on Windows." macOS: "Blender for macOS is distributed as disk images (dmg-files)" and "Blender supports both Intel and Apple Silicon architectures on macOS." Linux: "Download the Linux version for your architecture and decompress the file". Sources: manual — https://docs.blender.org/manual/en/5.0/getting_started/installing/windows.html , https://docs.blender.org/manual/en/5.0/getting_started/installing/macos.html , https://docs.blender.org/manual/en/5.0/getting_started/installing/linux.html — read 2026-07-30.

One caveat on that last point comes from the 5.0 release notes. The macOS page says Blender supports Intel, but the release notes list under Compatibility: "Support for Intel Macs has been removed" (https://developer.blender.org/docs/release_notes/5.0/, read 2026-07-30). The two 5.0 sources conflict. They do not establish here exactly which macOS packages were published.

*Other channels.* The manual also documents Blender being obtained through Snap ("snap install blender --classic"), Linux distribution packages, the Microsoft Store, and Steam. These are named as ways to install, not as separate stability tiers.

**Not covered by the sources.** The sources do not give the file-name pattern of a published download, nor the exact string that marks an RC in a version name, nor whether "Release Candidate" builds are published on the daily-builds channel or somewhere else. They do not say what `version_cycle` returns for an LTS build (there is no "lts" value in the documented list alpha/beta/rc/release), nor how a running Blender reports that it is an LTS build at all. Nothing in the sources gives the naming of "Build from Source" output. The Linux architectures on offer are not enumerated. The LTS marking is answered by the test below; the rest stay open.

**Observed — hand test, 2026-08-04.** Everything in this block was run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The build was Blender 5.0.1, installed at `/Applications/Blender 5.0.1.app`.

*What the build prints when asked its version.* In a terminal, with nothing else set up:

`"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" --version`

It printed, word for word:

> ```
> Blender 5.0.1
> 	build date: 2025-12-16
> 	build time: 01:51:40
> 	build commit date: 2025-12-15
> 	build commit time: 16:36
> 	build hash: a3db93c5b259
> 	build branch: blender-v5.0-release
> 	build platform: Darwin
> 	build type: Release
> ```

Three further lines followed — `build c flags:`, `build c++ flags:` and `build link flags:`, each very long — and then `	build system: CMake`. The command exited straight away, with exit code 0. There is no stability word and no LTS marking on the first line.

*What the build reports about itself in Python, with no window open.* In a terminal:

`"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" --background --python-expr "import bpy; print('BEGIN'); [print(a, '=', repr(getattr(bpy.app, a))) for a in sorted(dir(bpy.app)) if not a.startswith('_')]; print('TYPE version_cycle', type(bpy.app.version_cycle)); print('TYPE build_type', type(bpy.app.build_type)); print('END')"`

It printed every name and value on `bpy.app` — 70 entries in all. The lines bearing on this question, word for word:

> ```
> build_branch = b'blender-v5.0-release'
> build_commit_date = b'2025-12-15'
> build_commit_time = b'16:36'
> build_commit_timestamp = 1765816573
> build_date = b'2025-12-16'
> build_hash = b'a3db93c5b259'
> build_platform = b'Darwin'
> build_system = b'CMake'
> build_time = b'01:51:40'
> build_type = b'Release'
> module = False
> portable = False
> version = (5, 0, 1)
> version_cycle = 'release'
> version_file = (5, 0, 119)
> version_string = '5.0.1'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

and `build_options`, in full:

> `build_options = bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=True, jack=False, pulseaudio=False, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=False, potrace=True, pugixml=True, haru=True, experimental_features=False)`

So on this build `version_cycle` reads `release` and `version_string` reads `5.0.1` with nothing appended.

*What the installed application file itself carries.* In a terminal:

`file "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender"` printed:

> `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender: Mach-O 64-bit executable arm64`

`lipo -archs "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender"` printed:

> `arm64`

`plutil -p "/Applications/Blender 5.0.1.app/Contents/Info.plist"` printed, among other lines:

> ```
>   "CFBundleIdentifier" => "org.blenderfoundation.blender"
>   "CFBundleName" => "Blender"
>   "CFBundleShortVersionString" => "5.0.1"
>   "CFBundleVersion" => "5.0.1"
>   "LSMinimumSystemVersion" => "11.2"
> ```

This does not resolve the conflict recorded above between the 5.0 manual page and the 5.0 release notes about Intel Mac support. It records one thing only: the 5.0.1 macOS build installed on the Creator's Apple Silicon Mac holds one processor architecture, `arm64`. What is published for Intel Macs, if anything, was not looked at.

**Observed — hand test, 2026-08-05.** The Creator ran this one, on their own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. They opened Blender 5.0.1 from the Applications folder in the ordinary way and sent screenshots of what was on screen. This settles what the splash-screen mark reads on a finished release — the manual's own picture of it shows the development build reading "5.0.0 Alpha" quoted above.

*The splash screen.* The top right corner of the splash image reads:

> `5.0.1`

So on this finished release the corner carries the three-number version and no stability word, where the manual's picture of a development build carried `Alpha` after the numbers.

*The version at the right-hand end of the bar along the bottom of the window.* It reads:

> `5.0.1`

with a small globe icon and a vertical divider to its left.

**Observed — hand tests, 2026-08-06, on a second machine.** Everything in this block was run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. It is a container sharing somebody else's hardware, and other work was running on it at the same time, so nothing here says how long anything took. The build was Blender 5.0.1, extracted from Blender's own Linux archive at `/opt/blender/blender-5.0.1-linux-x64`. These are a second machine's results and sit beside the Mac's above; they are never merged with them.

*What the build prints when asked its version.* In a terminal, `/opt/blender/blender-5.0.1-linux-x64/blender --version` printed, word for word:

> ```
> Blender 5.0.1
> 	build date: 2025-12-16
> 	build time: 01:30:59
> 	build commit date: 2025-12-15
> 	build commit time: 16:36
> 	build hash: a3db93c5b259
> 	build branch: blender-v5.0-release
> 	build platform: Linux
> 	build type: Release
> ```

Three long flag lines followed, then `build system: CMake`. Exit code 0. There is no stability word and no LTS marking on the first line, the same as the Mac. The commit date, commit time, hash and branch are the Mac's too; the build time is not, and `build platform` reads `Linux` where the Mac read `Darwin`.

*What the build reports about itself in Python, with no window open.* The lines bearing on this question:

> ```
> build_branch = b'blender-v5.0-release'
> build_commit_date = b'2025-12-15'
> build_commit_time = b'16:36'
> build_commit_timestamp = 1765816573
> build_date = b'2025-12-16'
> build_hash = b'a3db93c5b259'
> build_platform = b'Linux'
> build_system = b'CMake'
> build_time = b'01:30:59'
> build_type = b'Release'
> memory_usage_undo = <built-in function memory_usage_undo>
> module = False
> portable = True
> version = (5, 0, 1)
> version_cycle = 'release'
> version_file = (5, 0, 119)
> version_string = '5.0.1'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

Seventy names in all, the same count as the Mac build. `portable` reads `True` here where the Mac read `False`, and no folder named `portable` exists in the extracted build — that was checked for all four Linux builds and none has one.

`build_options`, in full:

> `build_options = bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=False, jack=True, pulseaudio=True, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, potrace=True, pugixml=True, haru=True, experimental_features=False)`

Forty-four names, no `collada` — the same as the Mac build in shape. Four names read the other way round from the Mac's: `coreaudio` is `False` here and `True` there, `jack` is `True` here and `False` there, `pulseaudio` is `True` here and `False` there, and `xr_openxr` is `True` here and `False` there.

*What the build file itself is.* `file /opt/blender/blender-5.0.1-linux-x64/blender` printed:

> `/opt/blender/blender-5.0.1-linux-x64/blender: ELF 64-bit LSB executable, x86-64, version 1 (GNU/Linux), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=8625b275e1fded9be51d8c7f1804dedfef999ff7, stripped`

and `readelf -h` reported `Class: ELF64`, `Type: EXEC (Executable file)`, `Machine: Advanced Micro Devices X86-64`. One architecture, as on the Mac, though a different one. **This does not resolve the conflict recorded above between the 5.0 manual page and the 5.0 release notes about Intel Mac support**, and nothing on a Linux machine could. It does add one nearby fact: Blender's own download server, read on 2026-08-06 for the 5.2 release, publishes one macOS file and it is `blender-5.2.0-macos-arm64.dmg` — no Intel file. That is 5.2, not 5.0, and is recorded here only because it was seen.

*What a published build file is called.* This is the gap the "Not covered" note above records. The 5.2 release folder on Blender's own download server, read 2026-08-06, lists `blender-5.2.0-linux-x64.tar.xz`, `blender-5.2.0-macos-arm64.dmg`, `blender-5.2.0-windows-arm64.msi`, `blender-5.2.0-windows-arm64.msix`, `blender-5.2.0-windows-arm64.zip`, `blender-5.2.0-windows-x64.msi`, `blender-5.2.0-windows-x64.msix`, `blender-5.2.0-windows-x64.zip`, plus `blender-5.2.0.md5` and `blender-5.2.0.sha256`. So the pattern is the word blender, the three-number version, the operating system, the processor, and the file kind — with **no stability word and no LTS marking**. No listing was read for the 5.0 release, so this is observed for one release only.

*What the splash screen and the bottom bar show on Linux.* Blender 5.0.1 was opened with a window on the machine's own display and photographed with Blender's own screenshot command. The top right corner of the splash image reads `5.0.1` — the three numbers and no stability word, where the manual's own picture of a development build carried `Alpha`. The right-hand end of the bar along the bottom reads `5.0.1`. Both the same as the Creator's Mac.

**Outside the interface. Observed on two machines.** The naming and marking of published builds is a property of what Blender publishes, not of how Blender is driven, so it holds either way. The two marks a program can read without a window — `bpy.app.version_cycle` and `bpy.app.build_type` — are plain Python attributes on `bpy.app` with nothing in their documentation tying them to a user interface, and both were read with `--background` and no window in the run above. The splash-screen mark is by definition only visible when there is a window, and it was looked at with a window open on 2026-08-05, as recorded above. The sources do not state this split in so many words; the reading of the splash mark as interface-only is mine.

**Operating system. Observed for macOS and for Linux.** The four kinds and their marks are described once, without any per-operating-system difference. What differs per operating system is the package form: zip or installer on Windows (x64 and arm64), dmg on macOS, decompressed archive on Linux, plus Snap and distribution packages on Linux, Microsoft Store on Windows, and Steam on all three. The two 5.0 sources quoted above conflict about Intel Mac support; neither machine's test settles that. The Mac test records only that the build installed on that Apple Silicon Mac is `arm64` alone. **Observed on the rented Linux machine, 2026-08-06:** the 5.0.1 Linux build is a single-architecture x86-64 executable, reports `build platform: Linux`, and reads `portable = True` where the Mac read `False`; four of its forty-four build options read the opposite way from the Mac build's; the splash corner and the bottom bar both read `5.0.1`, as on the Mac. Windows remains untested.

---

## 2. Can builds of each kind be listed and fetched by something other than a person at a web page?

**Label: documented (for fetching, in part) and not in the sources (for listing).**

*Fetching.* The manual documents at least one command that fetches and installs Blender without a web page:

> "Assuming snap is already installed, Blender can be installed through snap with:
>
> snap install blender --classic"

and, of Linux distribution packages, "Some Linux distributions may have a specific package for Blender in their repositories." Source: manual — https://docs.blender.org/manual/en/5.0/getting_started/installing/linux.html — read 2026-07-30.

The Steam and Microsoft Store pages document fetching through those clients rather than a web page, including automatic updating: "When an update for Blender is available on Steam, Steam will automatically download and apply the update for you." (https://docs.blender.org/manual/en/5.0/getting_started/installing/steam.html) and "When an update for Blender is available on the Microsoft Store, it will be downloaded and installed automatically." (https://docs.blender.org/manual/en/5.0/getting_started/installing/windows.html). Both read 2026-07-30.

Against that, the manual is explicit that Blender itself will not fetch anything:

> "Blender is designed to not require an internet connection, so it doesn't have a built-in update system. This means you will need to update Blender yourself by following the platform-specific upgrade steps described in the sections below."

Source: manual — https://docs.blender.org/manual/en/5.0/getting_started/installing/index.html — read 2026-07-30.

*Listing.* **Not in the sources.** Nothing in the manual, the Python API documentation, or the 5.0 release notes describes a machine-readable list of published Blender builds — no index, feed, catalogue, or address that returns one.

Where I looked, for the listing half. I searched the whole manual, the whole API documentation and all the release notes for: `builder.blender.org`, `download.blender.org`, `buildbot`, `mirror`, `JSON`, `REST`, `feed`, `RSS`, `manifest`, `checksum`, `sha256`, `torrent`, `wget`, `curl`, `download`. `builder.blender.org`, `download.blender.org`, `buildbot`, `checksum` and `torrent` do not occur at all. `JSON`, `manifest` and `sha256` occur only in the Extensions pages about creating an extensions repository (https://docs.blender.org/manual/en/5.0/advanced/extensions/creating_repository/static_repository.html and the dynamic-repository page beside it) and in unrelated places such as glTF import/export — those describe repositories of add-ons, not of Blender builds, so I ruled them out. `mirror`, `feed`, `RSS`, `curl` and `wget` matched only unrelated uses (mirror modifier, colour mixing, the curl of a vector field, and text inside interface diagrams). I also read the full command-line arguments page (https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html) and the extensions command-line page beside it: the sub-commands documented there manage extensions, not Blender builds.

**Not covered.** The sources say nothing about whether the download server can be browsed or scripted, nothing about how a script would learn that a new build exists, nothing about verifying a downloaded build, and nothing about listing what Snap, Steam, the Microsoft Store or a distribution repository offer. The Snap and Steam commands documented fetch whatever those channels call "blender"; the sources do not say which of the four kinds that is, or whether a particular version can be asked for.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. The build was Blender 5.0.1 at `/Applications/Blender 5.0.1.app`.

The manual documents `-c, --command` and says "Use -c help to list all other commands." In a terminal:

`"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -c help`

The listing it printed, word for word:

> ```
> Blender Command Listing:
> 	extension
> ```

Nothing else was listed. (The run also printed messages from add-ons the Creator has installed, and a Python error one of them raised while shutting down; none of that is part of the listing.) The whole `--help` output, 466 lines, was also read, and no argument in it fetches or lists a Blender build. So the "not covered" note above — whether `-c help` lists any command relevant to installations — is answered for this build: it does not.

**Observed — hand tests, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. This is where the Snap and distribution-package routes quoted above could finally be exercised. **This settles the "not in the sources" half of this answer for one route: a listing of published Blender builds that a machine can read does exist, and Snap serves it.**

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
>   5.0/stable:        5.0.1   2026-03-17 (7093) 418MB classic
>   5.0/candidate:     ^                               
>   5.0/beta:          ^                               
>   5.0/edge:          ^                               
> ```

Every release line back to 2.79 has a channel, named for the line, with the letters `lts` in the name where the line is a long-term-support one — `5.2lts`, `4.5lts`, `4.2lts`, `3.6lts`, `3.3lts`, `2.93lts`, `2.83lts`. This version's channel is plain `5.0`, with no `lts`. Each channel exists in four forms: stable, candidate, beta and edge. Each line carries a version number, a date, a revision number and a size. So this answers the "Not covered" note above about whether a particular version can be asked for: it can, by naming its channel.

*What `snap install blender --classic` delivers.* Run with no channel named, it printed:

> `blender 5.2.0 from Blender Foundation (blenderfoundation**) installed`

with exit code 0, and `snap list blender` then printed:

> ```
> Name     Version  Rev   Tracking       Publisher            Notes
> blender  5.2.0    7599  latest/stable  blenderfoundation**  classic
> ```

So with no channel named the command gives whatever `latest/stable` holds — 5.2.0, not this version. What that build reports about itself is recorded under question 3 of the 5.2 findings.

*Whether that route updates itself, which the manual says it does.* `snap refresh --time` printed:

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

`apt-cache show blender` names its file as `blender_3.0.1+dfsg-7_amd64.deb`, gives its size as 24603992 bytes, and carries a `SHA256` line. Its dependency list names about forty system libraries including `libpython3.10`, so it is a different kind of build from a blender.org one, which brings its own Python. It was downloaded and read but **not installed**: `dpkg -c` shows it would put a program at `./usr/bin/blender`, and that path on this machine already holds a link to another Blender, so installing it would change something the whole machine shares while other work was running on it.

*A checksum, which the sources are silent about.* Beside the release archives on Blender's own download server sit `blender-5.2.0.md5` and `blender-5.2.0.sha256`. The second was fetched and the Linux archive checked against it: `sha256sum -c` printed `blender-5.2.0-linux-x64.tar.xz: OK`.

**Outside the interface. Observed on two machines.** The `snap install` command quoted is itself a command line, so it holds when nothing is clicking — and that is now observed rather than read: on the rented Linux machine `snap info blender`, `snap install blender --classic` and every `apt` command above were run from a terminal with no window at any point. The Steam and Microsoft Store routes are documented only as things done in those clients' own windows; the sources do not say whether they can be driven otherwise. The `-c help` run above was made from a terminal with no window, and returned only the extensions command.

**Operating system. Observed for macOS and for Linux.** Snap and distribution packages are documented on the Linux page only. The Microsoft Store is documented on the Windows page only. Steam is documented for "Linux, macOS, or Windows". The statement that Blender has no built-in update system is made once, without a per-system exception. **Observed on the Creator's Mac:** the build's own commands and arguments fetch nothing. **Observed on the rented Linux machine, 2026-08-06:** the Snap and distribution-package routes both exist there and both work as quoted, Snap carries a machine-readable listing of every published release line, and Snap refreshes itself on a timer. Windows, Steam and the Microsoft Store remain untested on both.

---

## 3. Can an installed Blender be asked what it is — which version, and which kind of build?

**Label: documented in part; not in the sources for whether the build is LTS.**

*From the command line.* The manual's command-line arguments page lists:

> "-v, --version
>
> Print Blender version and exit."

Source: manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

*From Python.* The API documents a set of read-only attributes on `bpy.app`. Exact words:

> "bpy.app.version_cycle
>
> The release status of this build alpha/beta/rc/release"

> "bpy.app.version_string
>
> The Blender version formatted as a string"

> "bpy.app.version
>
> The Blender version as a tuple of 3 numbers (major, minor, micro). eg. (4, 3, 1)"

> "bpy.app.version_file
>
> The Blender File version, as a tuple of 3 numbers (major, minor, file sub-version), that will be used to save a .blend file. […]"

> "bpy.app.build_type
>
> The type of build (Release, Debug)"

> "bpy.app.build_hash
>
> The commit hash this blender instance was built with"

> "bpy.app.build_branch
>
> The branch this blender instance was built from"

> "bpy.app.build_date
>
> The date this blender instance was built"

> "bpy.app.build_time
>
> The time this blender instance was built"

> "bpy.app.build_platform
>
> The platform this blender instance was built for"

> "bpy.app.build_system
>
> Build system used"

> "bpy.app.build_commit_date", "bpy.app.build_commit_time", "bpy.app.build_commit_timestamp", "bpy.app.build_cflags", "bpy.app.build_cxxflags", "bpy.app.build_linkflags", "bpy.app.build_options"

Source: Python API — https://docs.blender.org/api/5.0/bpy.app.html — read 2026-07-30.

*From the window.* The Topbar page documents the About Blender menu:

> "About Blender
>
> Opens a menu displaying the following information about Blender:
>
> Version: The Blender version.
>
> Date: Date when Blender was compiled.
>
> Hash: The Git Hash of the build. This can be useful to give to support personnel when diagnosing a problem.
>
> Branch: Optional branch name.
>
> Windowing Environment: On Linux, this will show either Wayland or X11 depending on the windowing environment that Blender is running on."

Source: manual — https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html — read 2026-07-30.

The status bar shows it too: "Blender Version / Shows the version number of Blender that is currently running." Source: manual — https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html — read 2026-07-30. And the splash image carries it: "The upper part of the splash screen contains the splash image with the Blender version in the top right" (https://docs.blender.org/manual/en/5.0/interface/window_system/splash.html); the picture `_images/interface_splash_current.webp` shows "5.0.0 Alpha".

**Which kind of build.** `version_cycle` answers alpha/beta/rc/release and `build_type` answers Release/Debug, both quoted above. They identify those two marks. The sources do not say that either mark identifies a daily build, and neither documented value list identifies LTS.

**Not covered.** This is the sharp edge of the answer. The sources do **not** show what `--version` actually prints — only that it prints "Blender version" and exits. They do not say whether that output includes the alpha/beta/rc mark, the build hash, or the date. Nor do the sources give any way, from the command line or from Python, to ask whether a build is an LTS build: "LTS" appears nowhere among the documented `bpy.app` attributes, and `version_cycle` has no documented "lts" value. Which release numbers are LTS is stated in prose on the download and version-list pages, not exposed as a mark on the build. The sources also do not say how to ask this of a Blender that is installed but not running, other than by running it.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The build was Blender 5.0.1 at `/Applications/Blender 5.0.1.app`. This answers three of the gaps above: what `--version` prints, whether the printed text carries the cycle word, hash and date, and what `build_platform` actually contains.

*From the command line.* `"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" --version` printed the whole block quoted under question 1: `Blender 5.0.1`, then the build date and time, the commit date and time, the commit hash, the branch, the build platform, the build type, three long flag lines, and the build system. So the printed text does carry the hash, the branch and the dates. It carries no alpha/beta/rc word — the build type line reads `Release`, which is the compilation type, not the release status.

*From Python, with no window open.* The `--background --python-expr` command quoted under question 1 returned every `bpy.app` value. The ones this question asks about, word for word:

> ```
> background = True
> binary_path = '/Applications/Blender 5.0.1.app/Contents/MacOS/Blender'
> build_branch = b'blender-v5.0-release'
> build_hash = b'a3db93c5b259'
> build_date = b'2025-12-16'
> build_platform = b'Darwin'
> build_system = b'CMake'
> build_type = b'Release'
> module = False
> portable = False
> tempdir = '/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/blender_SooQZG/'
> version = (5, 0, 1)
> version_cycle = 'release'
> version_file = (5, 0, 119)
> version_string = '5.0.1'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

So on this Mac `build_platform` reads `b'Darwin'` and `build_system` reads `b'CMake'`.

*As a saved file, with no window open.* The 5.0 findings do not mention the system-information file, so it was tested here alongside the other versions. In a terminal:

`"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" --background --python-expr "import bpy; bpy.ops.wm.sysinfo(filepath='<a folder outside the project>/system-info.txt')"`

Blender printed `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)` and then `Blender quit`, and the file was written. Its opening and Blender section, word for word:

> ```
> ====================================
> = Blender 5.0.1 System Information =
> ====================================
> 
> 
> Blender:
> ====================================
> 
> version: 5.0.1, branch: blender-v5.0-release, commit date: 2025-12-15 16:36, hash: a3db93c5b259, type: Release
> build date: 2025-12-16, 01:51:40
> platform: 'macOS-26.5.2-arm64-arm-64bit'
> binary path: '/Applications/Blender 5.0.1.app/Contents/MacOS/Blender'
> ```

Three long build-flag lines follow, then `build system: CMake`. The next section, headed `Python:`, opens `version: 3.11.13 (main, Apr 25 2025, 12:39:20) [Clang 17.0.0 (clang-1700.3.19.1)]`.

*What the arguments list itself says.* `--help` printed 466 lines. The version entry, word for word:

> ```
> -v or --version 
> 	Print Blender version and exit.
> ```

with no platform note beside it, while `/?` on the same page prints `Print this help text and exit (Windows only).` and `--qos <level>` prints `Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only).`

**Observed — hand test, 2026-08-05.** The Creator ran this one, on their own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. They opened Blender 5.0.1 from the Applications folder in the ordinary way and sent screenshots of what was on screen. This settles what the About Blender menu actually shows on a finished release, which the manual describes only in words.

*The About Blender panel.* Clicking the Blender icon at the far left of the Topbar and choosing About Blender opens a panel. Its left-hand column reads, line by line:

> ```
> Blender
> Version: 5.0.1
> 
> Date: 2025-12-15 16:36
> Hash: a3db93c5b259
> Branch: blender-v5.0-release
> 
> Blender is free software
> Licensed under the GNU General Public License
> ```

Its right-hand column is a set of links: `Donate`, `What's New`, then `Credits`, `License`, `Blender Store`, `Blender Website`. Four of the five rows the manual documents are present — Version, Date, Hash and Branch. The fifth, `Windowing Environment`, which the manual says appears "On Linux", does not appear on this Mac. The panel shows no build time, no build platform, no build type and no build system, and nothing on it says LTS.

*The splash screen and the bottom bar.* The top right corner of the splash image reads `5.0.1`. The right-hand end of the bar along the bottom of the window reads `5.0.1`.

*The Help menu.* The 5.0 findings do not quote a Help menu, so it was looked at alongside the other versions. Clicking Help in the row of menus along the top drops down, in order:

> ```
> Manual
> Support
> User Communities
> Get Involved
> Release Notes
> ———
> Report a Bug
> Save System Info
> ```

**Observed — hand tests, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. The build was Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64`. Every route above was tried again on this second machine.

*From the command line.* `--version` printed the block quoted under question 1 of this document's Linux block. It carries the hash, the branch and the dates, and no alpha/beta/rc word — the same as on the Mac.

*What the arguments list itself says.* `--help` printed **479 lines**, against 466 on the Mac. The version entry, word for word:

> ```
> -v or --version 
> 	Print Blender version and exit.
> ```

with no platform note, while `/?` prints `Print this help text and exit (Windows only).` and `--qos <level>` prints `Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only).` The four registration arguments are all printed, each marked `(Windows & Linux only)`.

*From Python, with no window open.* The `--background --python-expr` dump returned every value. The ones this question asks about:

> ```
> background = True
> binary_path = '/opt/blender/blender-5.0.1-linux-x64/blender'
> build_branch = b'blender-v5.0-release'
> build_hash = b'a3db93c5b259'
> build_date = b'2025-12-16'
> build_platform = b'Linux'
> build_system = b'CMake'
> build_type = b'Release'
> memory_usage_undo = <built-in function memory_usage_undo>
> module = False
> portable = True
> tempdir = '/tmp/blender_WmjDod/'
> version = (5, 0, 1)
> version_cycle = 'release'
> version_file = (5, 0, 119)
> version_string = '5.0.1'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

Seventy names, the same count as the Mac. The four Linux builds' name lists were also set against each other, which no single version could do: this build holds a name `memory_usage_undo` that the Linux 4.5 build does not, and lacks a name `debug_ffmpeg` that the Linux 4.5 build has — which is why both hold seventy. It also lacks a name `cachedir` that the Linux 5.1 and 5.2 builds hold.

*What Blender itself offers as commands.* `-c help` printed:

> ```
> Blender Command Listing:
> 	extension
> ```

the same as the Mac.

*As a saved file, with no window open.* In a terminal:

`/opt/blender/blender-5.0.1-linux-x64/blender --background --python-expr "import bpy; bpy.ops.wm.sysinfo(filepath='<a folder outside the project>/system-info.txt')"`

Blender printed one warning line of its own before anything else — `cycles | WARNING HIPEW initialization failed: Error opening HIP dynamic library` — which is this machine having no AMD graphics support, then `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)` and `Blender quit`. The file was written. Its opening and Blender section, word for word:

> ```
> ====================================
> = Blender 5.0.1 System Information =
> ====================================
> 
> 
> Blender:
> ====================================
> 
> version: 5.0.1, branch: blender-v5.0-release, commit date: 2025-12-15 16:36, hash: a3db93c5b259, type: Release
> build date: 2025-12-16, 01:30:59
> platform: 'Linux-6.8.0-87-generic-x86_64-with-glibc2.35'
> binary path: '/opt/blender/blender-5.0.1-linux-x64/blender'
> ```

Three long build-flag lines follow, then `build system: CMake`. The `Python:` section opens `version: 3.11.13 (main, Apr 25 2025, 12:39:20) [GCC 11.2.1 20220127 (Red Hat 11.2.1-9)]`. Searching the whole file for the letters LTS found none, as on the Mac. The Python version number is the Mac's; what compiled it is not.

*The About Blender panel, with a window open on the machine's own display.* Blender was started with a window, `bpy.ops.wm.splash_about` was called from a timer, and the window was photographed with Blender's own screenshot command. Its left-hand column reads, line by line:

> ```
> Blender
> Version: 5.0.1
> 
> Date: 2025-12-15 16:36
> Hash: a3db93c5b259
> Branch: blender-v5.0-release
> Windowing Environment: X11
> 
> Blender is free software
> Licensed under the GNU General Public License
> ```

**This settles the one row the manual documents and no machine had ever shown.** The manual says `Windowing Environment` appears "On Linux", and here it does, reading `X11`. On the Creator's Mac that row is absent. So all five rows the manual describes are present on this machine, where four are present on the Mac.

*The splash screen and the bottom bar.* The top right of the splash image reads `5.0.1`; the right-hand end of the bottom bar reads `5.0.1`.

*The Help menu, with a window open.* `bpy.ops.wm.call_menu(name='TOPBAR_MT_help')` was called and the window photographed. The menu drops down, in order:

> ```
> Manual
> Support
> User Communities
> Get Involved
> Release Notes
> ———
> Report a Bug
> Save System Info
> ```

Line for line what the Creator saw on the Mac, including the last entry written without a trailing ellipsis.

**Outside the interface. Observed on two machines.** `-v, --version` is a command-line argument and exits immediately, so it needs no window. The `bpy.app` attributes are documented as plain attributes with no stated dependence on a user interface, and `bpy.app.background` exists precisely to tell a script it is "running without a user interface (started with -b)", so scripts are expected to run that way. That is now observed: with `--background`, every value listed above was returned and `bpy.ops.wm.sysinfo` wrote its file, with no window at any point. The About menu, status bar and splash screen are window-only by their nature, and all three were looked at with a window open on 2026-08-05, as recorded above.

**Operating system. Observed for macOS and for Linux.** `-v, --version` is listed with no per-system restriction, unlike neighbouring arguments that carry explicit ones ("Windows only", "Windows & Linux only"); it ran and printed on both machines. The About Blender menu has one per-system line: the Windowing Environment row is documented as appearing "On Linux". On the Creator's Mac that row is absent from the panel; **on the rented Linux machine it is present and reads `X11`**, which settles it. `bpy.app.build_platform` read `b'Darwin'` on the Mac and `b'Linux'` on the rented machine. Two more differences: the help output is 466 lines on the Mac and 479 on the rented machine, and `portable` reads `False` on the Mac and `True` on the rented machine. Everything else about this answer came back the same on both. Windows remains untested.

---

## 4. Does Blender name one release as the one to use for ordinary work?

**Label: documented, with a qualification.**

Blender does not name a single release for everyone. The Installing page frames it as a choice, and gives the trade-off rather than a verdict:

> "Each package has the trade off of newest features versus stability. The package that is right for you depends on your requirements for those two. A studio, for example, might want to have long-term support, while a hobbyist may want newer features, while others may just want to test upcoming features. Each package described below has something just right for everyone."

Source: manual — https://docs.blender.org/manual/en/5.0/getting_started/installing/index.html — read 2026-07-30.

But for production work, the manual does make a recommendation, and it is unambiguous:

> "New Blender versions may add, remove or change functionality that affects the results of production files. For a given project, it is advisable to use a single LTS version of Blender. LTS versions receive bug fixes for two years."

Source: manual — https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html — read 2026-07-30.

So: the recommended kind for a project is LTS, and the advice is to fix on one and stay on it. The manual does not name a specific version number as "the one".

The sources also mark the stability ordering plainly: "Stability can be expected to increase from Alpha to Beta to Release Candidate (RC) to a final release", and daily builds "are not as thoroughly tested as the release types above, and might break or crash" (both from the Installing page, quoted in full under question 1).

**Not covered.** The sources do not name which release number is the current LTS, and give no LTS release in the 5.x series — the manual's version list ends at "Blender 4.5 LTS" as the most recent LTS entry. The sources do not say whether 5.0 itself is an LTS release, nor when the next one is due beyond "A new long-term support version is available every year". They also do not say what to do when a project needs a feature only present outside LTS.

**Observed — hand test, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. This does not answer the question, and is recorded only because it is the nearest thing to a default any machine could be made to show. `snap install blender --classic` — the one command-line fetch route the manual documents — was run with no channel named, and printed:

> `blender 5.2.0 from Blender Foundation (blenderfoundation**) installed`

`snap list blender` then showed it tracking `latest/stable`. `snap info blender` shows that older lines can be asked for by name, including `5.0` and `4.5lts`. **That is Snap's own default, not Blender naming a release**, and it says nothing about what Blender recommends. It does settle one thing the "Not covered" note above raises: on 2026-08-06 the newest LTS line Snap carried was `5.2lts`, so the LTS line has moved past 4.5 since this version's manual was written.

**Outside the interface.** The advice is about which build to run, not about how it is driven; nothing in it changes when Blender is started by a script. The page it comes from is titled "Deploying Blender in Production" and is addressed to "environments such as animation studios and schools" with "special requirements regarding security, automated deployment and customization", so it is written with automated deployment in mind.

**Operating system.** Stated once, with no per-system difference.

---

## 5. Can one Blender installed on a computer cause trouble for another that is already there?

**Label: documented for the main case (they are meant to coexist), inferred for two side effects, and not in the sources for the rest.**

*Coexistence is intended and documented.*

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

Source: manual — https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html — read 2026-07-30.

> "When using this method of installation, it is possible to have multiple versions of Blender installed."

Source: manual (Linux) — https://docs.blender.org/manual/en/5.0/getting_started/installing/linux.html — read 2026-07-30.

> "Note, you do not have to overwrite your existing Blender installation. It's perfectly possible to have multiple versions installed side by side."

Source: manual (Windows, updating from a zip) — https://docs.blender.org/manual/en/5.0/getting_started/installing/windows.html — read 2026-07-30.

*Settings are kept apart per version.* The directory layout page lists the user directories as `$HOME/.config/blender/5.0/` on Linux, `/Users/$USER/Library/Application Support/Blender/5.0/` on macOS, and `%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.0\` on Windows — the version number is part of the path. Source: manual — https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html — read 2026-07-30. The Defaults page says so outright:

> "The preferences need to be imported from previous versions because the configuration files of each Blender version are stored in separate folders."

and describes the one deliberate crossing:

> "Import Preferences From Previous Version
>
> Selecting this option will copy preferences from an older version of Blender. Doing so will copy preferences and startup files from the previous version of Blender and load them. This will include installed add-ons and extensions."

with a warning attached:

> "Some previous Blender add-ons and extensions may not be compatible with a new version of Blender, and choosing this option may lead to errors on startup."

Source: manual — https://docs.blender.org/manual/en/5.0/getting_started/configuration/defaults.html — read 2026-07-30. Note the direction: the new version copies from the old, so the trouble lands on the new installation, not the old one.

*Where one installation does replace another.* On macOS: "Install the new version by overwriting the current Blender.app in the Applications folder. You can rename Blender.app or place it in a different folder to have more than one version at a time." (macOS page, read 2026-07-30). On Windows with the installer: "To remove a previously installed version of Blender, use Windows settings or control panel to uninstall the desired version." (Windows page, read 2026-07-30).

*Two possible side effects that the sources do not settle.*

(a) **File association.** The command-line page documents "-r, --register / Register blend-file extension for current user, then exit (Windows & Linux only)", "--register-allusers", "--unregister" and "--unregister-allusers" (https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html). The Windows page says the installer "will create an entry in the start menu as well as associate blend-files with Blender", while the zip route says "No start menu item will be created and no blend-file association will be registered". The sources do not say what happens when two Blender installations both register.

(b) **Automatically-updating channels.** Snap: "Installing from this method has a benefit that updates to Blender are automatically installed." Steam: "Steam will automatically download and apply the update for you." Microsoft Store: "it will be downloaded and installed automatically." The sources say the update is automatic. They do not say whether the previous version is kept or removed, so they do not establish trouble for another installed Blender.

*What is not in the sources.* Nothing describes two installed Blenders competing over anything at run time — no shared lock, no shared port, no shared cache, no shared temporary directory conflict, no PATH conflict. I searched the manual, the API documentation and the release notes for `running instance`, `already running`, `in use`, `file lock`, `locked file`, `atomic`, `rollback`, and read the whole directory-layout page: `in use` matched only unrelated wording, and `running instance`, `file lock` and `atomic` do not occur.

**Not covered by the sources.** Whether two Blenders of the *same* version can be installed side by side (the user directory is named by version, so they would share it — the sources do not say what happens); that part is answered by the test below. Whether the Windows installer for a new version removes the old one automatically. Whether removing one Blender takes shared configuration with it. Whether a Blender installed by a package manager and one extracted from an archive interfere. Whether Blender ever writes outside the directories the layout page lists.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2.

*Set-up.* A second copy of the installed application was made in a working folder outside `/Applications`, with `cp -Rc "/Applications/Blender 5.0.1.app" <working folder>/Blender.app`. That gave two copies of Blender 5.0.1 on one computer, in two different places.

*Action.* This was run against each of the two, changing only the path:

`"<path to the copy>/Contents/MacOS/Blender" --background --python-expr "import bpy; print('USER', bpy.utils.resource_path('USER')); print('LOCAL', bpy.utils.resource_path('LOCAL')); print('SYSTEM', bpy.utils.resource_path('SYSTEM')); print('BINARY', bpy.app.binary_path)"`

*What came back.* From the one in `/Applications`:

> ```
> USER /Users/artist/Library/Application Support/Blender/5.0
> LOCAL /Applications/Blender 5.0.1.app/Contents/Resources/5.0
> SYSTEM /Library/Application Support/Blender/5.0
> BINARY /Applications/Blender 5.0.1.app/Contents/MacOS/Blender
> ```

From the second copy, in the working folder:

> ```
> USER /Users/artist/Library/Application Support/Blender/5.0
> LOCAL <working folder>/Blender.app/Contents/Resources/5.0
> SYSTEM /Library/Application Support/Blender/5.0
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

**Observed — hand test, 2026-08-05.** The Creator ran this one, on their own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. They opened Blender 5.0.1 in the ordinary way, opened Edit ‣ Preferences, clicked System in the list down the left-hand side, and sent a screenshot of that page.

The screenshot shows the System page with five sections, in this order: `Cycles Render Devices`, `Network`, `Memory & Limits`, `Video Sequencer` and `Sound`. No section headed `Operating System Settings` appears in it, and no button named `Register`, `Unregister` or `For All Users` appears in it. One screenshot was supplied, so what it shows is the page from `Cycles Render Devices` at the top down to the last row of `Sound`; whether anything sits below `Sound` is not shown.

**Observed — hand tests, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH, with all four researched versions extracted under `/opt/blender`. The registration arguments in (a) above are marked "Windows & Linux only", so this is the first machine on which they could be used at all. It is a rented machine that is thrown away, which is why changing a file-type association was acceptable here and was not on the Creator's own computer. Everything that changed is written out, and everything created was removed afterwards.

**This settles (a) above, which the answer records as something the sources do not say: what happens when two Blender installations both register. The second replaces the first.**

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

It does two things the current-user form did not: it made the links in `/usr/local/bin`, and it set up a thumbnailer.

*Unregistering.* `/opt/blender/blender-5.0.1-linux-x64/blender --unregister` printed:

> ```
> Unregister: /opt/blender/blender-5.0.1-linux-x64/blender
> - Remove symbolic-links in: ~/.local/bin
> - Remove icon: ~/.local/share/icons/hicolor/scalable/apps/blender.svg
> - Remove desktop-file: ~/.local/share/applications/blender.desktop
> - Remove mime type: ~/.local/share/mime/application/x-blender.xml
> - Remove mime type as default
> ```

This build had never registered anything, and it removed the registration another build had made. `--unregister` clears whatever registration is there, not only its own. `--unregister-allusers` from the 5.1.2 build did the same for the all-users side, thumbnailer included.

*What unregistering leaves behind.* Set against the file list taken at the start, unregistering did **not** put the machine back. Twelve rebuilt mime-database files were left in `~/.local/share/mime` and twelve more in `/usr/local/share/mime`; the folders `applications`, `icons`, `mime` and `thumbnailers` were left in place though empty; and `~/.config/mimeapps.list` was left still naming `blender.desktop`. All of it was removed by hand afterwards, and the machine's file list then matched the earlier one exactly.

*Whether a Preferences panel of these controls exists on Linux.* All four Linux builds were asked, with no window, to list the preferences panels they register; all four register a panel labelled `Operating System Settings`. Opened with a window and photographed, Blender 5.0.1's Preferences System page shows, in order: `Cycles Render Devices`, `Display Graphics`, `Operating System Settings`, `Network`, `Memory & Limits`, and more below. The `Operating System Settings` section reads `Open blend files with this Blender version` above two buttons, `Register` and `Unregister`, with a checkbox `For All Users` beneath. **On the Creator's Mac neither that section nor `Display Graphics` appears.**

*Two builds of one version, in two places.* This settles the "Not covered" point above about whether two Blenders of the same version can sit side by side and what they share. A second copy of this build was made with `cp -a` into a working folder. Run with `BLENDER_USER_RESOURCES` deliberately unset and with `--factory-startup`, so that the default path came back and nothing was saved, the two printed:

> ```
> USER /root/.config/blender/5.0
> LOCAL /opt/blender/blender-5.0.1-linux-x64/5.0
> SYSTEM ''
> BINARY /opt/blender/blender-5.0.1-linux-x64/blender
> ```

> ```
> USER /root/.config/blender/5.0
> LOCAL /root/smoozy/Builds/work/06/5.0/copy/5.0
> SYSTEM ''
> BINARY /root/smoozy/Builds/work/06/5.0/copy/blender
> ```

The same user path, only the build's own folder following the copy. That agrees with the Mac, and the path is the one the layout page documents for Linux. One difference from the Mac: the system path comes back as an **empty string** here, where the Mac returned a real path.

*Two builds of one version from two different sources.* A Blender 5.2.0 was also installed through Snap on this machine, alongside the blender.org 5.2.0 extracted under `/opt/blender`. Both report the same user path, `/root/.config/blender/5.2`. So builds from two different sources carrying the same two-number version do share one settings folder — which is the case the answer above says the sources leave open. It is recorded in full under question 5 of the 5.2 findings.

**Outside the interface. Observed on two machines.** The coexistence statements are about files on disk and hold however Blender is started; the deploying page explicitly frames them for automated deployment. Both runs above were made with `--background` and no window, and both returned the same USER path. The Import Preferences dialogue is part of the first-run splash screen and so belongs to the window; the sources do not say what a Blender started with `-b` does about a missing configuration folder for its version, and that was not tested. Registering the file extension is documented as a command-line argument that exits, so it needs no window.

**Operating system. Observed in part for macOS and for Linux.** *(Citation check of the Linux hand tests, 2026-08-06: this line still opened "Observed in part for macOS" although a Linux result had been written into it below. Corrected to name both machines.)* The overwrite behaviour is different on each: on macOS you overwrite `Blender.app` (or rename it to keep both); on Windows the installer route uses Windows' own uninstall, while the zip route does not overwrite anything; on Linux archives are extracted to a directory of your choosing. `--register` and friends are documented "(Windows & Linux only)", and the Mac build's own `--help` marks all four the same way; the screenshot of the Preferences System page on the Creator's Mac shows no `Operating System Settings` section among the five sections it covers. **Observed on the rented Linux machine, 2026-08-06:** all four registration arguments run there and do what they say, the `Operating System Settings` section is on the Preferences System page with `Register`, `Unregister` and `For All Users` in it, registration writes under `~/.local` for one user and `/usr/local` for all, and **registering a second Blender does take the association from the first, by writing over its file**. So the two machines differ sharply, in the way the manual's own platform note predicts. Windows remains untested. The user-configuration path differs per system, as quoted. The `portable` folder that makes an installation self-contained sits "Next to the Blender executable, in the unzipped folder" on Windows and Linux, and "Inside the application bundle at Blender.app/Contents/Resources" on macOS.

---

## 6. Does installing a Blender disturb a render already running on the same computer?

**Label: not in the sources.**

Nothing in Blender's 5.0 manual, its Python API documentation, its 5.0 release notes, or Cycles' own files says anything about installing Blender while a render is under way.

Where I looked. I read the whole of the Installing section — the index page and all four platform pages (https://docs.blender.org/manual/en/5.0/getting_started/installing/index.html and the linux, macos, windows and steam pages beside it) — and the whole of the Deploying Blender in Production page (https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html). None of them mentions rendering, running processes, or anything that must be stopped first. I searched the whole manual, API documentation and release notes for: `while rendering`, `during a render`, `running instance`, `already running`, `in use`, `file lock`, `locked file`, `restart Blender`, `overwrite.*running`, `replace.*running`. `running instance`, `already running`, `file lock` and `locked file` do not occur at all. `while rendering` and `during a render` matched only pages about what you may do inside Blender's own interface while it renders (node editing, compositing sidebar, Cycles performance, output settings) — none about installing. `restart Blender` matched only preferences pages, about settings that need a restart to take effect. I also read the command-line index and arguments pages, and the Blender's Directory Layout page, for any statement about files being held open; there is none.

**Not covered by the sources.** Everything about the question. In particular the sources do not say whether Blender holds its own program files open once started, whether extracting a new copy over a running one is safe, or whether the two installations would have to be in different directories. The second and third of those were tested by hand, below.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The installed Blender in `/Applications` was never touched; both runs used a throwaway copy in a working folder outside the project.

*Set-up, for both runs.* A copy of Blender 5.0.1 was made into an empty working folder with `cp -Rc`, with an empty output folder beside it. The scene was Blender's own factory startup scene — no file was opened. The render was started from the copy, with the window switched off:

`"<working folder>/Blender.app/Contents/MacOS/Blender" --background --factory-startup --python-expr "import bpy; bpy.context.scene.cycles.samples=64; bpy.context.scene.render.resolution_percentage=50" -E CYCLES -F PNG -o "<working folder>/out/frame_" -s 1 -e 30 -a`

*First run — a second Blender put on the computer beside the running one.* Once four image files had appeared, `cp -R "/Applications/Blender 4.5.12.app" "<working folder>/second/Blender.app"` was run in another terminal, copying a whole second Blender in beside the rendering one. It finished with exit code 0.

> ```
> ### image files on disk before the action: 4 at 21:22:20
> ### action exit code 0
> ### action finished at 21:22:25
> ### image files just after the action: 5
> ### render process ended, exit code 0 at 21:23:24
> ### image files at the end: 30
> ```

All thirty numbered images were written, ending `frame_0028.png`, `frame_0029.png`, `frame_0030.png`. The render printed no error.

*Second run — the same Blender copied over the exact folder the render was running from.* Same set-up, a fresh working folder. Once four image files had appeared, `cp -R "/Applications/Blender 5.0.1.app/" "<working folder>/Blender.app/"` was run — the contents of the installed 5.0.1 copied straight over the folder the render was reading from. It finished with exit code 0.

> ```
> ### image files on disk before the action: 4 at 21:28:41
> ### action exit code 0
> ### action finished at 21:28:46
> ### image files just after the action: 6
> ### render process ended, exit code 0 at 21:29:36
> ### image files at the end: 30
> ```

All thirty numbered images were written. The render printed no error.

**Observed — hand tests, 2026-08-06, on the rented Linux machine. The two machines disagree on the second run, and neither result is set aside.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH, shared with other work at the time, so nothing here says how long anything took. Nothing under `/opt/blender` was touched; both runs used a throwaway copy made with `cp -a`.

*Set-up, for both runs.* A copy of the extracted Blender 5.0.1 into an empty working folder, an empty output folder beside it, Blender's own factory startup scene, and the render started with no window:

`<working folder>/blender/blender --background --factory-startup --python-expr "import bpy; bpy.context.scene.cycles.samples=64; bpy.context.scene.render.resolution_percentage=50" -E CYCLES -F PNG -o "<working folder>/out/frame_" -s 1 -e 30 -a`

*First run — a second Blender put on the computer beside the running one.* Once four image files had appeared, `cp -a /opt/blender/blender-5.2.0-linux-x64 <working folder>/second/blender` was run:

> ```
> ### image files on disk before the action: 4
> ### the copy's own exit code: 0
> ### is the second Blender there? /root/smoozy/Builds/work/render2/5.0/A/second/blender
> ### image files just after the action: 4
> ### render process ended, exit code 0
> ### image files at the end: 30
> ```

Its last lines were `01:08.951  render | Fra: 30 | Mem: 39M | Finished`, `Saved: '<working folder>/out/frame_0030.png'`, `Time: 00:02.32 (Saving: 00:00.04)` and `Blender quit`. No line contained the word error. **This agrees with the Mac.**

*Second run — the same Blender copied over the exact folder the render was running from.* Once four image files had appeared, `cp -a /opt/blender/blender-5.0.1-linux-x64/. <working folder>/blender/` was run. The copy finished with exit code 0. What came back:

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
> 00:10.629  render           | Fra: 5 | Mem: 2M | Updating Baking
> 00:10.629  render           | Fra: 5 | Mem: 2M | Updating Device | Writing constant memory
> 00:10.629  render           | Fra: 5 | Mem: 2M | Loading denoising kernels (may take a few minutes the first time)
> 00:10.629  render           | Fra: 5 | Mem: 2M | Sample 0/64
> 00:10.664  render           | Fra: 5 | Remaining: 00:02.19 | Mem: 39M | Sample 1/64
> ```

An ordinary progress line for frame 5, cut off mid-frame. Blender printed no error of its own; the words `Bus error (core dumped)` came from the shell that started it. Four images were on disk at the end instead of thirty, and the four written before the action were still there.

**The same test on the Creator's Mac finished all thirty frames with exit code 0 and printed no error. On this machine it stopped the render, in this version and in all three others.** Both results stand; neither is chosen between.

**Outside the interface. Observed on two machines.** The sources do not say, and are equally silent for a render started by a script. Every run on both machines was a background render started from a terminal with no window.

**Operating system. Observed for macOS and for Linux, and they do not agree.** The sources do not say. On macOS 26.5.2 with an Apple M1 Pro chip, both runs completed all thirty frames. On the rented Linux machine, putting a second Blender beside the running one left the render untouched, and copying a Blender over the folder the render was running from stopped the render with exit code 135 and four frames of thirty written. Windows remains untested, and the Windows installer could not be tried on either computer; the package-manager routes were reached on the rented machine and are recorded under question 2.

---

## 7. Can a computer be asked which Blenders it holds?

**Label: not in the sources for Blender's own answer; documented for two partial routes.**

*Not in the sources.* Blender provides no documented way to enumerate the Blender installations on a machine. The `bpy.app` attributes describe only the Blender that is running, and the one that comes closest is explicitly about itself:

> "bpy.app.binary_path
>
> The location of Blender's executable, useful for utilities that open new instances. Read-only unless Blender is built as a Python module - in this case the value is an empty string which script authors may point to a Blender binary."

Source: Python API — https://docs.blender.org/api/5.0/bpy.app.html — read 2026-07-30. That tells a running Blender where it lives, not what else is on the machine.

*Two partial routes that are documented.*

(a) A Linux package manager can list what it installed. Exact words: "Installing Blender via the distribution's native mechanisms ensures consistency with other packages on the system and may provide other features (given by the package manager), such as listing of packages, update notifications and automatic menu configuration." Source: manual — https://docs.blender.org/manual/en/5.0/getting_started/installing/linux.html — read 2026-07-30. This covers only Blenders installed that way.

(b) The manual states where installations land by default, so those places can be looked at. The Blender's Directory Layout page gives, under System Directories: Linux "Archive downloaded from blender.org: ./5.0/" and "Linux distribution packages: /usr/share/blender/5.0/"; macOS "./Blender.app/Contents/Resources/5.0/"; Windows "Zip file downloaded from blender.org: ./5.0/", "Installer downloaded from blender.org: %ProgramFiles%\Blender Foundation\Blender\5.0\", "Microsoft Store installation: %ProgramFiles%\WindowsApps\BlenderFoundation.Blender<HASH>\Blender\5.0\". Source: manual — https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html — read 2026-07-30. The command-line Windows page gives the same installer path: "C:\Program Files\Blender Foundation\Blender\blender.exe" (https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/windows.html), and the macOS page gives "./Blender.app/Contents/MacOS/Blender" (https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/macos.html). Turning those paths into a search of the machine is a step the sources do not take, so that step would be **inferred**, and it would miss anything installed elsewhere — which the sources say is allowed: "Blender downloads can be extracted to any directory on the system".

Where I looked, for the "not in the sources" part. I searched the whole manual, API documentation and release notes for `installed versions`, `list of installed`, `which versions are installed`, `installed blender`, `locate`, `discover`, `binary_path`. The only hits for the first four were incidental prose on the Windows launch page and the bug-report page. I read the whole of `bpy.app` (https://docs.blender.org/api/5.0/bpy.app.html) and the whole command-line arguments page, including the `-c, --command` entry: "Run a command which consumes all remaining arguments. Use -c help to list all other commands." The sub-commands the manual documents for `--command` are the Extensions ones (package, repository and extension creation), which manage add-ons, not Blender installations.

**Not covered by the sources.** Whether `-c help` lists any command relevant to installations — the sources list only the extension sub-commands, but do not claim the list is complete; that is answered below. Whether a Blender installed via Snap, Steam or the Microsoft Store can be enumerated by those tools. Whether there is any registry entry, receipt, or marker file left by the Windows installer or the macOS dmg.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2.

*What Blender itself offers.* `"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -c help` printed:

> ```
> Blender Command Listing:
> 	extension
> ```

So on this build the complete list is one command, and it is about extensions.

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

`system_profiler SPApplicationsDataType` printed one block per Blender. The 5.0 block, word for word:

> ```
>     Blender 5.0.1:
> 
>       Version: 5.0.1
>       Obtained from: Identified Developer
>       Last Modified: 2025-12-16, 02:52
>       Kind: iOS
>       Signed by: Developer ID Application: Stichting Blender Foundation (68UA947AUU), Developer ID Certification Authority, Apple Root CA
>       Location: /Applications/Blender 5.0.1.app
>       Get Info String: 5.0.1 2025-12-16, Blender Foundation
> ```

Blocks in the same shape were printed for 4.5.12, 5.1.2 and the one at `/Applications/Blender.app` (shown as `Version: 5.2.0`). None of the three commands named any of the four copies sitting in the working folder. Asking the index about that folder directly, `mdfind -onlyin <working folder> "kMDItemFSName == 'Blender.app'"`, returned nothing at all.

*What a running Blender says about where it is.* `bpy.app.binary_path` read `'/Applications/Blender 5.0.1.app/Contents/MacOS/Blender'`, and the system-information file under question 3 carries the same path on its `binary path:` line. Both report the one Blender that produced them.

**Observed — hand test, 2026-08-06, on the rented Linux machine. This settles the Linux half of the question**, which route (a) above only pointed at. Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH.

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

So route (a) is real but as narrow as its wording: **a package manager names only what it installed**, and none of these five was installed that way.

*After a package-managed Blender was added.* A Blender 5.2.0 was installed with `snap install blender --classic`. `snap list` then named it, and only it:

> ```
> blender  5.2.0  7599  latest/stable  blenderfoundation**  classic
> ```

`apt list --installed` still named nothing. `which -a blender` then printed three paths — `/usr/bin/blender`, `/bin/blender` and `/snap/bin/blender`. The listing grew by exactly the one build the package manager installed, and the other five stayed invisible to it.

*What the documented install locations look like here.* The layout page's Linux entry for an archive from blender.org is `./5.0/` beside the executable, and that is what is there: the extracted folder holds a folder named `5.0` beside the program. Route (b) above therefore works as described, but only for a build put in one of the documented places, and every version's manual also says a download can be put in any folder at all.

*What a running Blender says about where it is.* `bpy.app.binary_path` read `'/opt/blender/blender-5.0.1-linux-x64/blender'`, and the system-information file under question 3 carries the same path on its `binary path:` line. Both report the one Blender that produced them, exactly as on the Mac.

**Outside the interface. Observed on two machines.** `bpy.app.binary_path` is a plain attribute available to a script and needs no window; the package-manager listing is a command-line matter by nature. Neither depends on a person clicking. Every command above was run from a terminal with no window open, and `binary_path` was read with `--background`.

**Operating system. Observed for macOS and for Linux.** The package-manager listing is documented on the Linux page only. The default install locations differ per system, as quoted. There is no documented equivalent of a package listing for macOS; the three commands above are macOS's own, not Blender's, and what each returned is recorded here. **Observed on the rented Linux machine, 2026-08-06:** the documented Linux route works, and it names only what it installed — `apt` and `snap` named none of the five Blenders on the machine until one was put there through Snap, and then named exactly that one. The two machines agree on the shape of the answer: the operating system's own tools name what was put there their way, and nothing else. Windows remains untested.

---

## 8. What becomes of a render that is running when the Blender it is using is removed?

**Label: not in the sources.**

Nothing in Blender's 5.0 manual, its Python API documentation, its 5.0 release notes, or Cycles' own files describes removing Blender while it is running, or what happens to work in progress if it is.

Where I looked. The manual has no uninstall page. I searched the whole manual, API documentation and release notes for `uninstall`: the matches are the Add-ons and Extensions preferences pages (uninstalling an add-on), the graphics-driver troubleshooting pages (uninstalling a driver), `bpy.ops.preferences` and `bpy.ops.extensions` (operators for removing extensions), and one line on the Windows installing page — "To remove a previously installed version of Blender, use Windows settings or control panel to uninstall the desired version" (https://docs.blender.org/manual/en/5.0/getting_started/installing/windows.html) — which says nothing about a running Blender. I read the whole Installing section, the whole Deploying Blender in Production page, the whole Blender's Directory Layout page, and the command-line index, arguments and render pages. I also searched for `while rendering`, `during a render`, `in use`, `file lock`, `running instance`, `already running`, `crash.*render`, `interrupt`, `resume`. `interrupt` and `resume` matched only pages about the render output and physics caches, describing Blender's own behaviour when a render is stopped by the user — not removal of the program.

**Not covered by the sources.** Everything the question asks: whether the running render continues, whether partial output already written to disk survives, whether Blender reports anything, and whether an animation render mid-sequence leaves usable frames. The sources say none of it. All four were tested by hand, below, for the deletion case on macOS.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The installed Blender in `/Applications` was never touched; the test used a throwaway copy in a working folder outside the project, and nothing outside that folder was deleted.

*Set-up.* A copy of Blender 5.0.1 was made into an empty working folder with `cp -Rc`, with an empty output folder beside it. The scene was Blender's own factory startup scene. The render was started from the copy, with the window switched off:

`"<working folder>/Blender.app/Contents/MacOS/Blender" --background --factory-startup --python-expr "import bpy; bpy.context.scene.cycles.samples=64; bpy.context.scene.render.resolution_percentage=50" -E CYCLES -F PNG -o "<working folder>/out/frame_" -s 1 -e 30 -a`

*Action.* Once four image files had appeared, `rm -rf "<working folder>/Blender.app"` was run in another terminal — the whole application the render was running from, deleted.

*What came back.*

> ```
> ### image files on disk before the action: 4 at 21:34:15
> ### action exit code 0
> ### does the folder still exist? ls: <working folder>/Blender.app: No such file or directory
> ### action finished at 21:34:16
> ### image files just after the action: 4
> ### render process ended, exit code 0 at 21:35:10
> ### image files at the end: 30
> ```

The deletion was permitted and completed at once; the application folder was gone. The render carried on and wrote all thirty numbered images, ending `frame_0028.png`, `frame_0029.png`, `frame_0030.png`, then ended by itself with exit code 0. It printed no error. The four images written before the deletion were still there afterwards, and the remaining twenty-six were written after it.

*What was left afterwards.* `ls ~/Library/Application\ Support/Blender/5.0` printed `config`, `datafiles`, `extensions`, `scripts` — the settings folder for this version survived the application being deleted.

**Observed — hand test, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Nothing under `/opt/blender` was touched; the test used a throwaway copy made with `cp -a`, and nothing outside the working folder was deleted.

*Set-up and action.* As on the Mac: a copy of the extracted Blender 5.0.1, an empty output folder, Blender's own factory startup scene, and the same thirty-frame background render command quoted under question 6 of this document. Once four image files had appeared, `rm -rf <working folder>/blender` was run — the whole folder the render was running from, deleted.

*What came back.*

> ```
> ### image files on disk before the action: 4
> ### action: the whole Blender folder the render is running from, deleted; exit code 0
> ### does the folder still exist? ls: cannot access '/root/smoozy/Builds/work/render/5.0/C/blender': No such file or directory
> ### image files just after the action: 4
> ### render process ended, exit code 0
> ### image files at the end: 30
> ### last file: frame_0030.png
> ```

The last lines the render printed:

> ```
> 01:25.244  render           | Fra: 30 | Mem: 39M | Finished
> 01:25.287  render           | Saved: '<working folder>/out/frame_0030.png'
> 01:25.288  render           | Time: 00:02.53 (Saving: 00:00.04)
> 
> Blender quit
> ```

No line contained the word error. That version's settings folder was still there afterwards and still held its contents.

**This agrees with the Creator's Mac**, in this version and in the other three: the deletion is permitted, the render carries on, all thirty frames are written, the frames written before the deletion survive, and the settings folder outlives the program.

**Outside the interface. Observed on two machines.** The sources do not say, and there is no statement for a render started with `-b` either. Every run on both machines was started with `--background` from a terminal with no window, and every one finished after its own program had been deleted.

**Operating system. Observed for macOS and for Linux, and they agree.** The sources do not say. On macOS 26.5.2 the deletion was allowed while the render was running, and the render finished. On the rented Linux machine the same held. Windows remains untested, and there is no uninstaller route on either computer to compare against.

---

## 9. Can installing or removing a Blender be done again safely on a computer where it was cut off part-way?

**Label: not in the sources.**

Nothing in Blender's 5.0 manual, its Python API documentation, its 5.0 release notes, or Cycles' own files addresses an interrupted install or an interrupted removal, or says whether repeating one is safe.

Where I looked. I read the whole Installing section (index, linux, macos, windows, steam) and the whole Deploying Blender in Production page. Neither mentions failure, interruption, partial state, recovery, or repeating a step. I searched the whole manual, API documentation and release notes for `atomic`, `rollback`, `resume`, `interrupt`, `reinstall`, `re-run`, `repeat`, `partial`, `recovery`. `atomic` does not occur. `rollback` matched only console and preferences pages, unrelated. `resume` and `interrupt` matched only render-output and physics-cache pages. `reinstall` matched only the Add-ons preferences page (reinstalling an add-on) and the crash-troubleshooting page. `re-run`, `repeat`, `partial` and `recovery` matched only unrelated pages (asset browser, undo/redo, selection, render output, and the manual's own contributor build instructions). I also read the Troubleshooting section index (https://docs.blender.org/manual/en/5.0/troubleshooting/index.html) and its Startup page (https://docs.blender.org/manual/en/5.0/troubleshooting/startup.html), which lists common causes of Blender failing to start — minimum requirements, graphics drivers, antivirus, user permissions — and says nothing about a partly-completed install or removal.

The nearest thing the sources contain is not an answer but a constraint, and it is worth recording because it bears on how an install can be performed at all:

> "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead."

Source: manual (Known Limitations, Linux) — https://docs.blender.org/manual/en/5.0/getting_started/installing/linux.html — read 2026-07-30.

And the general shape of an install, which is the reason the question is askable:

> "Blender downloads can be extracted to any directory on the system, as a self contained installation."

Source: manual — https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html — read 2026-07-30. Reading "self contained, extract to a directory" as meaning a half-finished extraction can simply be redone would be a step the sources do not take; I am not taking it.

**Not covered by the sources.** Everything the question asks: whether a half-extracted archive can be extracted over, whether the Windows installer can be re-run over a broken install, whether an interrupted Windows uninstall leaves the machine in a state the installer will accept, whether Snap or Steam or the Microsoft Store recover on their own, and whether any leftover configuration in the user directory affects a fresh install. The macOS case — copying an application bundle into place — was tested by hand, below. The rest were not.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. Everything happened in a working folder outside the project; the installed Blender in `/Applications` was only ever read from.

*A note on the copying tool, because it changed the test.* The first attempt used `cp -R`. A Blender application bundle copied that way would not start at all on this computer — whether the copy had been interrupted or had finished — and `codesign --verify --verbose` on a finished `cp -R` copy printed `invalid Info.plist (plist or signature have been modified)`. So the test was run again with macOS's own copying tool, `ditto`, which does produce a bundle that starts, leaving the interruption as the only thing under test. Both attempts are recorded here in the order they happened.

*The test, with `ditto`.* Into a fresh empty folder, `ditto "/Applications/Blender 5.0.1.app" <working folder>/Blender.app` was started and killed after two seconds:

> ```
> ### copy killed at 21:39:47
> ### how much arrived: 369M
> ### is there an executable? -rwxr-xr-x@ 1 artist     admin  174673568 Dec 16  2025 <working folder>/Blender.app/Contents/MacOS/Blender
> ### running the half-copied Blender with --version:
> ### exit code 137
> ```

The half-copied Blender printed nothing at all and gave exit code 137. The same `ditto` command was then run again over the same folder, without deleting anything first:

> ```
> ### second copy exit code 0
> ### size now: 829M
> ```

`codesign --verify --verbose` on the finished copy printed `a sealed resource is missing or invalid`, followed by a list of `file added:` lines, all of them compiled-Python cache files under `Contents/Resources/5.0/python/`. Running that finished copy with `--version` gave exit code 0 and printed:

> ```
> Blender 5.0.1
> 	build date: 2025-12-16
> 	build time: 01:51:40
> 	build commit date: 2025-12-15
> 	build commit time: 16:36
> 	build hash: a3db93c5b259
> 	build branch: blender-v5.0-release
> 	build platform: Darwin
> 	build type: Release
> ```

which is the same as the installed 5.0.1 printed under question 3.

*What leftover configuration there is.* After a throwaway copy of 5.0.1 was deleted outright (see question 8), `ls ~/Library/Application\ Support/Blender/5.0` still printed `config`, `datafiles`, `extensions`, `scripts`. So a fresh copy put in afterwards would find that version's settings folder already there and populated.

**Observed — hand tests, 2026-08-06, on the rented Linux machine. This settles the Linux shape of the question, including the 7-zip limitation quoted above.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Both `tar` and `7z` were already on the machine; nothing was installed for this.

*Which archive was used.* No Blender archive was left on the machine, so one was fetched from Blender's own download server into the working folder purely so that there was a real archive to extract: `blender-5.2.0-linux-x64.tar.xz`. It is the 5.2 archive, not this version's — a 5.0 archive was not fetched. What is settled below is therefore about a Blender Linux archive and the tools that unpack it, which is what the quoted limitation is about, rather than about this version in particular. The file was checked against Blender's own published checksum and `sha256sum -c` printed `blender-5.2.0-linux-x64.tar.xz: OK`.

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

The archive holds 79 internal links and 7-zip turned every one into a plain file. So the documented limitation is real, it breaks the result in two separate ways, and **it fails silently** — the tool reports success both times.

*An extraction cut off part-way, and done again over the same folder.* `tar -xf` was started into an empty folder and killed part-way. What had arrived:

> ```
> 186M	half
> files: 769
> ```

and the main program had not been written at all — `ls` on it printed `No such file or directory`. Trying to run that path gave exit code 127, with the shell printing `No such file or directory`; Blender printed nothing, because there was no Blender there to print anything. The same `tar -xf` command was run again over the same folder, with nothing deleted first. It finished with exit code 0, and the folder then held 5592 files and 79 links — the same as a clean extraction. The result started with `--version`, and its whole output was compared line by line against the 5.2.0 build already extracted under `/opt/blender`: **identical**.

So on Linux an interrupted extraction can simply be done again over the same folder, provided TAR is the tool. That is the same answer the Mac gave for its own shape of the question, reached by a different route. It also answers the first item in the "Not covered" note above — whether a half-extracted archive can be extracted over — for this machine.

*What leftover configuration there is.* On the rented machine, after a throwaway copy of this version was deleted outright with `rm -rf` while a background render was running from it (see question 8), that version's settings folder was still there and still held its contents. So a fresh copy put in afterwards would find it already there, as on the Mac.

**Outside the interface. Observed on two machines.** The sources do not say, and nothing distinguishes an install driven by a script from one driven by a person, because nothing is said about either. Everything above was done from a terminal on both machines, with no Blender window at any point.

**Operating system. Observed for macOS and for Linux.** The sources do not say. The documented install routes differ per system: extract an archive on Linux, drag a bundle on macOS, run an installer or extract a zip on Windows, plus Snap, Steam and the Microsoft Store. On macOS 26.5.2 an interrupted `ditto` copy could be repeated over the same folder and produced a Blender that started and reported the right version. On the rented Linux machine an interrupted `tar` extraction could be repeated over the same folder and produced a Blender identical to the one already installed — and the 7-zip limitation was confirmed, with the two ways it breaks a build written out above. Windows remains untested, and the Windows installer, the Microsoft Store and Steam could not be tried on either computer; Snap and a distribution package were reached on the rented machine and are recorded under question 2.

---

## Things a person with Blender open could settle by hand in a few minutes

The list that stood here has been worked through. What was settled is written into the answers above, marked observed. What is left open is below.

1. **What a daily build reports.** Run `--version` and the `bpy.app` dump from question 3 against an alpha or beta build, and record what `version_cycle`, `version_string` and `build_branch` read. No such build was available when the tests above were run.

2. **Whether installing or removing Blender disturbs a running render on Windows.** The macOS and Linux halves of both are answered under questions 6 and 8. On a spare Windows machine, start a long background render and, while it runs, install a second Blender by the normal route for that system; separately, remove the rendering installation by the normal route for that system. Record whether the process stays running, whether the render finishes, and every message printed.

3. **Whether re-running the Windows installer over a broken install works.** On a spare Windows machine, start the Blender installer and cancel it part-way through. Then run the same installer again to completion, and note whether it complains, whether it offers repair, and whether the finished install starts and reports the right version. Then do the mirror test with the uninstaller.

4. **What a daily build's file is called, and what a daily build reports about itself.** Question 1 now records the file names of a finished release, read from Blender's own download server. The equivalent for a daily build is still owed, and so is running `--version` and the `bpy.app` dump against an alpha or beta build. No such build was available on either machine.

---

2026-08-04 — Opus 5 (Claude Code) — Builds, Blender 5.0

2026-08-04 — Opus 5 (Claude Code) — wrote the hand-test results into questions 1, 2, 3, 5, 6, 7, 8 and 9, brought the outside-the-interface and operating-system lines up to date where a test settled them, and pruned the hand-test list at the end

2026-08-05 — Opus 5 (Claude Code) — wrote the Creator's two screenshot tests into questions 1, 3 and 5, brought the outside-the-interface and operating-system lines up to date where they settled them, and pruned the hand-test list at the end

2026-08-06 — Opus 5 (Claude Code) — wrote the Linux hand-test results into questions 1 to 9, from a rented Linux machine; brought both standing lines up to date on every question a Linux test settled, and pruned the hand-test list at the end

Declaration: every finding above came from files inside `Blender Sources/` or from the question sheet at `Preparation/Blender Research/Questions/Builds.md`. I opened no other part of this project. I started no helper agents, so there is no helper's work to vouch for. Where a claim needed a step beyond the quoted words, that step is written out and the claim is labelled inferred.
