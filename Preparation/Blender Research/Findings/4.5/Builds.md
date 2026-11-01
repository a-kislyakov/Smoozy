# Builds — Blender 4.5

Sources consulted: on 2026-07-30 (the snapshot date given), Blender 4.5 manual (`Blender Sources/manual_4.5`, including its `_images` folder), Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`); on 2026-08-04, the one file in `Blender Sources/requirements/`, named `requirements_2026-08-01.txt`. The requirements file carries no version in its address and I only use it where it names Blender 4.5 in so many words.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

No helper agents were used. Every search and every file opened was my own.

---

## 1. What kinds of build does Blender publish, and how is each kind named and marked?

**Label: documented**

The manual's download page lists four packages under their own headings: Stable Release, Long-term Support, Daily Builds and Build from Source. The introduction calls what the page offers "binary packages" but does not say which of the four headings that phrase covers; its closing line is "Each package described below has something just right for everyone", which reads across all four. So treating Build from Source as sitting outside the binary packages would be a step the page does not take. *(Citation check, 2026-08-05: this paragraph previously said the page listed three binary packages plus a fourth heading for source code. The page does not sort them that way. Narrowed to what the words carry.)*

Exact words, from the manual page "Installing Blender":

> "Blender offers a variety of different binary packages to choose from depending on their level of stability. Each package has the trade off of newest features versus stability."

> "**Stable Release**
> A package that contains the latest features and is considered stable without regressions. A new stable version is available roughly every three months."

> "**Long-term Support**
> A package designed for long-lasting projects requiring a very stable version of Blender. LTS releases are supported for two years and will not have any new features, API changes or improvements. A new long-term support version is available every year. These LTS releases will occasionally have minor patches (such as 4.2.6) which improve stability or fix critical bugs."

> "**Daily Builds**
> A package updated daily to include the newest changes in development. These versions are automatically built on a schedule. They are not as thoroughly tested as the release types above, and might break or crash. Builds marked as Alpha are still undergoing major changes and feature additions, while those marked Beta are feature-complete and are under development for refinement and stability."

> "Stability can be expected to increase from Alpha to Beta to Release Candidate (RC) to a final release."

> "**Build from Source**
> Blender's source code is available for free to either reference or to build and use."

> "Blender is released approximately every three months."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/index.html — read 2026-07-30.

**How the kinds are marked.** Long-term support is marked in the name itself. The manual's own title page and its version list write it out:

> "Blender 4.5 LTS Reference Manual"

Source: the manual, https://docs.blender.org/manual/en/4.5/index.html — read 2026-07-30.

> "Blender 4.5 LTS
> …
> Blender 4.2 LTS
> …
> Blender 3.6 LTS"

Source: the manual, https://docs.blender.org/manual/en/4.5/versions.html — read 2026-07-30. (The same list shows non-LTS versions — 4.4, 4.3, 4.1, 4.0 — carrying no suffix.)

The release notes use the same marking, and also show the three-number form with the LTS suffix attached:

> "Blender 4.5 LTS was released on July 15, 2025."

> "This release includes long-term support and will be maintained until July 2027"

Source: the release notes, https://developer.blender.org/docs/release_notes/4.5/ — read 2026-07-30.

> "There are known stability issues in Blender 4.5.0 LTS."

Source: the release notes, https://developer.blender.org/docs/release_notes/4.5/cycles/ — read 2026-07-30.

The alpha/beta/rc/release marking is also carried inside the program itself:

> "bpy.app.version_cycle
> The release status of this build alpha/beta/rc/release"

Source: the Python API documentation, https://docs.blender.org/api/4.5/bpy.app.html — read 2026-07-30.

And a separate marking for how the binary was compiled:

> "bpy.app.build_type
> The type of build (Release, Debug)"

Source: the Python API documentation, https://docs.blender.org/api/4.5/bpy.app.html — read 2026-07-30.

**Package formats and processor variants**, which are a further split within each kind:

> "Download the zip-file or Windows Installer File."

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html — read 2026-07-30.

> "Blender for macOS is distributed as disk images (dmg-files)."

> "Blender supports both Intel and Apple Silicon architectures on macOS."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/macos.html — read 2026-07-30.

> "Download the Linux version for your architecture and decompress the file to the desired location"

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html — read 2026-07-30.

The release notes state that one of these variants ends with this version:

> "**macOS Intel support is deprecated.** Due to the high maintenance cost of tracking down and fixing graphics related issues specific to Intel and AMD GPU Macs, it was decided that this will be the last release with an official Intel macOS build of Blender."

Source: the release notes, https://developer.blender.org/docs/release_notes/4.5/ — read 2026-07-30.

**Not covered.** The sources never give the file-naming scheme of a downloaded build (no example of an actual download file name, no date or commit stamp in a name). They do not say where daily builds live, nor how often "Alpha" and "Beta" markings are applied to a name versus only shown inside the program. They do not say whether a Release Candidate is published as its own downloadable package or is only a stage. They give no marking that identifies an installed build as LTS — see question 3; the test below answers that, but the sources are still silent on it. Steam, the Microsoft Store, Snap and Linux distribution packages are described in the manual as ways to *get* Blender, not as separate kinds of build, and I have not treated them as kinds here.

**Observed — hand test, 2026-08-04.** Everything in this block was run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The build was Blender 4.5.12, installed at `/Applications/Blender 4.5.12.app`. This settles the "no marking identifies an installed build as LTS" gap that stood here: on this build, the LTS marking is carried in the version text itself.

*What the build prints when asked its version.* In a terminal, with nothing else set up, this was run:

`"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" --version`

It printed, word for word:

> ```
> Blender 4.5.12 LTS
> 	build date: 2026-07-21
> 	build time: 01:41:35
> 	build commit date: 2026-07-20
> 	build commit time: 13:37
> 	build hash: 84afd5f785f7
> 	build branch: blender-v4.5-release
> 	build platform: Darwin
> 	build type: Release
> ```

Three further lines followed — `build c flags:`, `build c++ flags:` and `build link flags:`, each one very long — and then the last line, `	build system: CMake`. The command exited straight away, with exit code 0.

*What the build reports about itself in Python, with no window open.* In a terminal, this was run:

`"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" --background --python-expr "import bpy; print('BEGIN'); [print(a, '=', repr(getattr(bpy.app, a))) for a in sorted(dir(bpy.app)) if not a.startswith('_')]; print('TYPE version_cycle', type(bpy.app.version_cycle)); print('TYPE build_type', type(bpy.app.build_type)); print('END')"`

It printed every name and value on `bpy.app`. The lines bearing on this question, word for word:

> ```
> build_branch = b'blender-v4.5-release'
> build_commit_date = b'2026-07-20'
> build_commit_time = b'13:37'
> build_commit_timestamp = 1784554675
> build_date = b'2026-07-21'
> build_hash = b'84afd5f785f7'
> build_platform = b'Darwin'
> build_system = b'CMake'
> build_time = b'01:41:35'
> build_type = b'Release'
> module = False
> portable = False
> version = (4, 5, 12)
> version_cycle = 'release'
> version_file = (4, 5, 92)
> version_string = '4.5.12 LTS'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

and `build_options`, in full:

> `build_options = bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=True, jack=False, pulseaudio=False, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, collada=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=False, potrace=True, pugixml=True, haru=True, experimental_features=False)`

The whole list of names on `bpy.app` for this build ran to 70 entries.

*What the installed application file itself carries.* In a terminal:

`file "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender"` printed:

> `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender: Mach-O 64-bit executable arm64`

`lipo -archs "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender"` printed:

> `arm64`

`plutil -p "/Applications/Blender 4.5.12.app/Contents/Info.plist"` printed, among other lines:

> ```
>   "CFBundleIdentifier" => "org.blenderfoundation.blender"
>   "CFBundleName" => "Blender"
>   "CFBundleShortVersionString" => "4.5.12"
>   "CFBundleVersion" => "4.5.12 2026-07-21"
>   "LSMinimumSystemVersion" => "11.2"
> ```

**Observed — hand test, 2026-08-05.** The Creator ran this one, on their own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. They opened Blender 4.5.12 from the Applications folder in the ordinary way and sent screenshots of what was on screen. This settles what the marking looks like in the window on a finished LTS release, which the manual only pictures for an old development build.

*The splash screen.* The top right corner of the splash image reads:

> `4.5.12 LTS`

So the splash carries the LTS marking as well as the three numbers.

*The version at the right-hand end of the bar along the bottom of the window.* It reads:

> `4.5.12`

with a small globe icon and a vertical divider to the left of it. No LTS marking there.

**Observed — hand tests, 2026-08-06, on a second machine.** Everything in this block was run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. It is a container sharing somebody else's hardware, and other work was running on it at the same time, so nothing here says how long anything took. The build was Blender 4.5.12, extracted from Blender's own Linux archive at `/opt/blender/blender-4.5.12-linux-x64`. This is a second machine, and its results are its own; they are set beside the Mac's above and never merged with them.

*What the build prints when asked its version.* In a terminal:

`/opt/blender/blender-4.5.12-linux-x64/blender --version`

Its first line, word for word:

> `Blender 4.5.12 LTS`

followed by `build date: 2026-07-21`, `build time: 01:31:22`, `build commit date: 2026-07-20`, `build commit time: 13:37`, `build hash: 84afd5f785f7`, `build branch: blender-v4.5-release`, `build platform: Linux`, `build type: Release`, three long compiler and linker flag lines, and `build system: CMake`. Exit code 0.

So the LTS marking is carried on the first line here exactly as on the Mac, and the commit hash, commit date and branch are the same as the Mac's. Two things differ: `build platform` reads `Linux` where the Mac read `Darwin`, and the build time reads `01:31:22` where the Mac's read `01:41:35`.

*What the build reports about itself in Python, with no window open.* The same `--background --python-expr` command as above was run. The lines this question turns on:

> ```
> build_branch = b'blender-v4.5-release'
> build_hash = b'84afd5f785f7'
> build_platform = b'Linux'
> build_system = b'CMake'
> build_time = b'01:31:22'
> build_type = b'Release'
> module = False
> portable = True
> version = (4, 5, 12)
> version_cycle = 'release'
> version_file = (4, 5, 92)
> version_string = '4.5.12 LTS'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

`portable` reads `True` on this machine, where it read `False` on the Mac. There is no folder named `portable` in the extracted build — that was checked, and `ls` reported no such directory for any of the four Linux builds.

`build_options`, in full:

> `build_options = bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=False, jack=True, pulseaudio=True, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, collada=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, potrace=True, pugixml=True, haru=True, experimental_features=False)`

Forty-five names, the same count as the Mac build, and `collada=True` is present here too. Four names read differently from the Mac build: `coreaudio` is `False` here and `True` there, `jack` is `True` here and `False` there, `pulseaudio` is `True` here and `False` there, and `xr_openxr` is `True` here and `False` there.

*What the build file itself is.* `file /opt/blender/blender-4.5.12-linux-x64/blender` printed:

> `/opt/blender/blender-4.5.12-linux-x64/blender: ELF 64-bit LSB executable, x86-64, version 1 (GNU/Linux), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=b87edf3f6cd03403c4864fc24e2379b9352fd9f4, stripped`

and `readelf -h` on the same file printed `Class: ELF64`, `Type: EXEC (Executable file)`, `Machine: Advanced Micro Devices X86-64`. One processor architecture, as on the Mac, though a different one. There is no application bundle on Linux and so no bundle information file to read; what sits beside the program in the extracted folder is `blender-launcher`, `blender-softwaregl`, `blender-thumbnailer`, `blender-system-info.sh`, `blender.desktop`, `blender.svg`, `blender-symbolic.svg`, `copyright.txt`, `license`, `readme.html`, `lib`, `textures`, `usd`, and a folder named `4.5`.

*What a published build file is called.* This is the gap the "Not covered" note above records — that the sources never give the file-naming scheme of a downloaded build. It is now settled by observation, though for the 5.2 release rather than this one, since that was the archive fetched. The release folder on Blender's own download server, read on 2026-08-06, lists these names:

> ```
> blender-5.2.0-linux-x64.tar.xz
> blender-5.2.0-macos-arm64.dmg
> blender-5.2.0-windows-arm64.msi
> blender-5.2.0-windows-arm64.msix
> blender-5.2.0-windows-arm64.zip
> blender-5.2.0-windows-x64.msi
> blender-5.2.0-windows-x64.msix
> blender-5.2.0-windows-x64.zip
> blender-5.2.0.md5
> blender-5.2.0.sha256
> ```

So a name is the word blender, the three-number version, the operating system, the processor, and the file kind. **No stability word and no LTS marking appears in any file name**, though 5.2 is an LTS release. A checksum file is published beside the archives; the Linux archive was fetched and `sha256sum -c` against that file printed `blender-5.2.0-linux-x64.tar.xz: OK`. No equivalent listing was read for 4.5, so the naming above is observed for one release only.

*What the splash screen and the bottom bar show on Linux.* Blender 4.5.12 was opened with a window on the machine's own display and photographed with Blender's own screenshot command. The top right corner of the splash image reads `4.5.12 LTS`. The right-hand end of the bar along the bottom of the window reads `4.5.12`, with no LTS marking. Both are the same as the Creator's Mac.

**Outside the interface. Observed on two machines.** The kinds and their markings are properties of what is published, not of how Blender is driven; nothing here depends on a person clicking. The part read from inside a running Blender was tested with no window: `--version` printed the LTS-marked version and exited, and the `--background --python-expr` run above returned `version_string`, `version_cycle`, `build_type` and the rest with no window open. The splash screen and the bottom bar are the two markings that need a window, and both were looked at on 2026-08-05 as recorded above. The sources state no difference, and none was seen.

**Operating system. Observed on macOS and on Linux.** The three binary package kinds and the source-build option are described once, for all systems. What differs by system is packaging: Windows gets a zip file and an installer (x64 and arm64), macOS gets a dmg (Intel and Apple Silicon, with Intel deprecated after this version), Linux gets an archive. All quoted above. **Observed** for macOS: the installed 4.5.12 macOS build on the Creator's Mac holds one processor architecture, `arm64`. **Observed** for Linux, 2026-08-06, on the rented Linux machine: the 4.5.12 Linux build holds one architecture too, x86-64, and reports `build platform: Linux`. The two builds carry the same commit hash and branch and differ in their build time and in four of their forty-five build options, as recorded above. The published file names, read from the download server on 2026-08-06 for the 5.2 release, name the operating system and the processor and carry no stability or LTS word. Windows remains untested.

---

## 2. Can builds of each kind be listed and fetched by something other than a person at a web page?

**Label: not in the sources**, for binary builds published by blender.org. **Documented** for two narrower things: that Blender itself has no update mechanism, and that some third-party package managers do fetch and update Blender without a person at a web page. The source-build option from question 1 is not answered here.

There is no machine-readable listing of Blender builds described anywhere in the four sources — no address to query, no index file, no feed, no download API, no checksum file.

What the sources do say, exactly:

> "Blender is designed to not require an internet connection, so it doesn't have a built-in update system. This means you will need to update Blender yourself by following the platform-specific upgrade steps described in the sections below."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/index.html — read 2026-07-30.

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30. This says deployment can be automated once you have the download; it says nothing about obtaining it.

Third-party routes that do fetch without a web page:

> "Assuming snap is already installed, Blender can be installed through snap with:
> `snap install blender --classic`"

> "Installing from this method has a benefit that updates to Blender are automatically installed."

> "Installing Blender via the distribution's native mechanisms ensures consistency with other packages on the system and may provide other features (given by the package manager), such as listing of packages, update notifications and automatic menu configuration. Be aware, though, that the package may be outdated compared to the latest official release, or not include some features of Blender."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html — read 2026-07-30.

> "When an update for Blender is available on the Microsoft Store, it will be downloaded and installed automatically."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html — read 2026-07-30.

> "When an update for Blender is available on Steam, Steam will automatically download and apply the update for you."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/steam.html — read 2026-07-30.

**Where I looked.** Full-text search of the whole 4.5 manual, the whole 4.5 Python API documentation and all 4.5 release-notes files for: `download`, `download.blender.org`, `blender.org/download`, `builder.blender.org`, `buildbot`, `build bot`, `nightly`, `daily build`, `mirror`, `torrent`, `RSS`, `feed`, `checksum`, `sha256`, `list of builds`, `download API`, `release list`, `update system`, `automatic update`, `auto-update`, `winget`, `chocolatey`, `homebrew`, `flatpak`, `apt-get`, `silent install`, `unattended`. The only `sha256` hit is an extensions-platform download link for an add-on, not for Blender itself; the only `homebrew` and `apt-get` hits are in the manual's own contributor instructions for building the documentation. I also read the whole of the manual's four installation pages and its deployment page, and searched Cycles' own files for anything about versions or builds (nothing — Cycles' `__init__.py` carries only its own `bl_info` block naming a minimum Blender of `(2, 80, 0)`).

**Not covered.** Nothing here settles whether such a listing or fetch route exists in fact — only that these sources do not describe one. It also does not settle what the Snap, Steam, Microsoft Store or distribution packages contain (which kind of build, which version), except that the manual warns a distribution package "may be outdated compared to the latest official release, or not include some features of Blender".

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. The build was Blender 4.5.12 at `/Applications/Blender 4.5.12.app`.

The manual says a `--command` can be given to Blender and that `-c help` lists the commands it offers. In a terminal this was run:

`"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -c help`

The build's own listing, word for word:

> ```
> Blender Command Listing:
> 	extension
> ```

Nothing else was listed. (The run also printed a number of messages from add-ons the Creator has installed, and a Python error raised by one of them while shutting down; those messages are not part of the listing.) So on this build the only command Blender offers is about extensions, and none fetches or lists a Blender build. The full argument list from `--help` was also read, and no argument there fetches or lists a build either.

**Observed — hand tests, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. This is where the Snap and distribution-package routes quoted above could finally be exercised. **This settles the "not in the sources" answer above for one route: a machine-readable listing of published Blender builds does exist, and Snap serves it.**

*What `snap info blender` lists.* In a terminal, `snap info blender` printed a listing whose publisher line reads `publisher: Blender Foundation (blenderfoundation**)`. Under a heading `channels:` it printed one line per channel. The opening of that listing, word for word:

> ```
> channels:
>   latest/stable:     5.2.0   2026-07-14 (7599) 426MB classic
>   latest/candidate:  5.1.2   2026-06-04 (7528) 436MB classic
>   latest/beta:       ^                               
>   latest/edge:       5.3.0   2026-08-05 (7676) 432MB classic
>   5.2lts/stable:     5.2.0   2026-07-14 (7599) 426MB classic
>   5.2lts/candidate:  5.2.0   2026-08-05 (7675) 426MB classic
> ```

and, further down, the lines for this version:

> ```
>   4.5lts/stable:     4.5.12  2026-07-21 (7626) 421MB classic
>   4.5lts/candidate:  4.5.12  2026-08-05 (7674) 421MB classic
>   4.5lts/beta:       4.5.0   2025-07-08 (6385) 421MB classic
>   4.5lts/edge:       ^                               
> ```

The whole listing runs to about a hundred and fifty lines and names a channel for every release line back to 2.79. Each channel is named for a release line, with the letters `lts` in the name where the line is a long-term-support one — `5.2lts`, `4.5lts`, `4.2lts`, `3.6lts`, `3.3lts`, `2.93lts`, `2.83lts` — and every channel exists in four forms, stable, candidate, beta and edge. Each line carries a version number, a date, a revision number and a size.

*What `snap install blender --classic` delivers.* In a terminal, `snap install blender --classic` printed:

> `blender 5.2.0 from Blender Foundation (blenderfoundation**) installed`

with exit code 0. Then `snap list blender` printed:

> ```
> Name     Version  Rev   Tracking       Publisher            Notes
> blender  5.2.0    7599  latest/stable  blenderfoundation**  classic
> ```

So with no channel named, the command delivers whatever `latest/stable` holds, which on this day was 5.2.0 — not this version. Naming a channel would be how an older line is asked for; that was not tried. What the snap build reports about itself is recorded under question 3 of the 5.2 findings.

*Whether that route updates itself, which the manual says it does.* `snap refresh --time` printed:

> ```
> timer: 00:00~24:00/4
> last: today at 03:51 UTC
> next: today at 11:11 UTC
> ```

So there is a repeating timer with a next time already set.

*What a distribution's own package carries.* `apt-cache policy blender` printed:

> ```
> blender:
>   Installed: (none)
>   Candidate: 3.0.1+dfsg-7
>   Version table:
>      3.0.1+dfsg-7 500
>         500 http://archive.ubuntu.com/ubuntu jammy/universe amd64 Packages
> ```

The manual's warning that a distribution package "may be outdated compared to the latest official release" is borne out: 3.0.1 against 5.2.0. `apt-cache show blender` gives its file name as `blender_3.0.1+dfsg-7_amd64.deb`, its size as 24603992 bytes, and a `SHA256` line. Its `Depends` line names about forty system libraries, `libpython3.10` among them — so it is a different kind of build from a blender.org one, which brings its own Python. The package was downloaded but **not installed**: `dpkg -c` on it shows it would put a program at `./usr/bin/blender`, and that path on this machine already holds a link to another Blender, so installing it would change something the whole machine shares while other work was running on it. The package holds twelve files in all.

**Outside the interface. Observed on two machines.** The `snap install` route is a command line and needs no interface at all — that is now observed rather than read: on the rented Linux machine both `snap info blender` and `snap install blender --classic` were run from a terminal with no window at any point, and so were the `apt` commands. Steam and the Microsoft Store are described only as clicking in their own applications; the sources give no non-interactive form of either. Blender itself does nothing here — it has no update system, and the `-c help` run above shows its only command is about extensions.

**Operating system. Observed on macOS and on Linux.** Snap and distribution package managers are Linux only. The Microsoft Store is Windows only. Steam is described for Linux, macOS and Windows. For macOS the manual describes only one route: "On macOS there is one main way to update Blender" — downloading a dmg from the website (https://docs.blender.org/manual/en/4.5/getting_started/installing/macos.html, read 2026-07-30). **Observed** on the Creator's Mac: the build's own commands and arguments fetch nothing. **Observed** on the rented Linux machine, 2026-08-06: the Snap and distribution-package routes both exist there and both work as quoted, and Snap carries a machine-readable listing of every published release line. Windows, Steam and the Microsoft Store remain untested.

---

## 3. Can an installed Blender be asked what it is — which version, and which kind of build?

**Label: documented** for version and for release status (alpha/beta/rc/release) and build provenance. **Not in the sources** for LTS status.

From the command line:

> "-v, --version
> Print Blender version and exit."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

From inside Blender, through Python:

> "bpy.app.version_string
> The Blender version formatted as a string"

> "bpy.app.version
> The Blender version as a tuple of 3 numbers (major, minor, micro). eg. (4, 3, 1)"

> "bpy.app.version_cycle
> The release status of this build alpha/beta/rc/release"

> "bpy.app.version_file
> The Blender File version, as a tuple of 3 numbers (major, minor, file sub-version), that will be used to save a .blend file."

> "bpy.app.build_branch
> The branch this blender instance was built from"

> "bpy.app.build_hash
> The commit hash this blender instance was built with"

> "bpy.app.build_date" … "bpy.app.build_time" … "bpy.app.build_commit_date" … "bpy.app.build_commit_time" … "bpy.app.build_commit_timestamp"

> "bpy.app.build_platform
> The platform this blender instance was built for"

> "bpy.app.build_type
> The type of build (Release, Debug)"

> "bpy.app.build_system
> Build system used"

Also present, and relevant to what a build contains:

> "bpy.app.build_options
> Constant value bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, … experimental_features=False)"

Source for all of the above: the Python API documentation, https://docs.blender.org/api/4.5/bpy.app.html — read 2026-07-30.

The API documentation's own worked example prints exactly this kind of thing:

> "print("Blender {:s}".format(bpy.app.version_string))"

and then loops over the `bpy.app` attributes beginning with `build_`, skipping two of them by name — the example's own line reads `if attr in {"handlers", "build_cflags", "build_cxxflags"}:`, under the comment "Overly verbose." It is wrapped as a command registered with `bpy.utils.register_cli_command`, under the heading: *(Citation check, 2026-08-05: previously said "every" attribute beginning with `build_`; the example skips `build_cflags` and `build_cxxflags`. Corrected against the page.)*

> "Registering commands makes it possible to conveniently expose command line functionality via commands passed to (-c / --command)."

Source: the Python API documentation, https://docs.blender.org/api/4.5/bpy.utils.html — read 2026-07-30.

From the interface, two more routes. The splash screen:

> "The upper part of the splash screen contains the splash image with the Blender version in the top right."

Source: the manual, https://docs.blender.org/manual/en/4.5/interface/window_system/splash.html — read 2026-07-30. The picture on that page, `_images/interface_window-system_splash_current.png`, shows a version number ("3.6.1") in the top right corner of the splash image, and nothing else identifying the build. Label for what the picture shows: **documented** (it is part of the manual). I looked at that image on 2026-07-30.

And the help menu:

> "Save System Info
> This extracts system information which can be useful for including in bug reports, inspecting the configuration, or diagnosing problems.
> You will be prompted to save a text file called system-info.txt."

> "**Blender**
> This section shows you the Blender version, details about the build configuration, and the path in which Blender is running."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/help.html — read 2026-07-30. The same thing exists as an operator: "bpy.ops.wm.sysinfo(*, filepath='') — Generate system information, saved into a text file" (the Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.wm.html, read 2026-07-30).

**On LTS.** I searched the whole Python API documentation for `LTS` and for `long term` / `long-term` and found nothing. There is no attribute, operator or command line argument in these sources that reports whether an installed build is an LTS build. The version number alone does not carry it either — the LTS marking appears only in prose ("Blender 4.5 LTS") on the website and in the manual's own titles.

**Not covered by the sources.** The sources do not say what text `--version` actually prints — only that it prints the version and exits. They do not say whether it prints the release status, the branch or the commit hash. They do not say whether `version_cycle` returns `release` for both a stable release and an LTS release. All three of those are answered by the test below, but the sources themselves remain silent on them. Nothing here distinguishes a stable release from a daily build directly; the closest is `version_cycle` (a daily build would be marked alpha or beta, per question 1) plus `build_branch`, `build_hash` and `build_date`, but no source states that mapping, so drawing it would be inference beyond the text. No daily build was available to test.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The build was Blender 4.5.12 at `/Applications/Blender 4.5.12.app`. This settles three things left open above: what `--version` prints, what `version_cycle` reads on a finished LTS release, and whether an installed Blender says it is LTS.

*From the command line.* `"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" --version` printed the version, the build date and time, the commit date and time, the commit hash, the branch, the build platform, the build type, three long compiler and linker flag lines, and the build system — the whole output is quoted under question 1. Its first line was `Blender 4.5.12 LTS`. So the printed text does carry the branch, the hash, the date and the letters LTS.

*From Python, with no window open.* The `--background --python-expr` command quoted under question 1 returned every `bpy.app` value. Word for word, the ones this question asks about:

> ```
> background = True
> binary_path = '/Applications/Blender 4.5.12.app/Contents/MacOS/Blender'
> build_branch = b'blender-v4.5-release'
> build_hash = b'84afd5f785f7'
> build_date = b'2026-07-21'
> build_platform = b'Darwin'
> build_system = b'CMake'
> build_type = b'Release'
> module = False
> portable = False
> tempdir = '/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/blender_8ctEig/'
> version = (4, 5, 12)
> version_cycle = 'release'
> version_file = (4, 5, 92)
> version_string = '4.5.12 LTS'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

So `version_cycle` reads `release` on this LTS build, and the LTS marking is carried by `version_string`, not by `version_cycle`.

*As a saved file, with no window open.* In a terminal:

`"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" --background --python-expr "import bpy; bpy.ops.wm.sysinfo(filepath='<a folder outside the project>/system-info.txt')"`

The file was written. Its opening and its whole Blender section, word for word:

> ```
> =========================================
> = Blender 4.5.12 LTS System Information =
> =========================================
> 
> 
> Blender:
> =========================================
> 
> version: 4.5.12 LTS, branch: blender-v4.5-release, commit date: 2026-07-20 13:37, hash: 84afd5f785f7, type: Release
> build date: 2026-07-21, 01:41:35
> platform: 'macOS-26.5.2-arm64-arm-64bit'
> binary path: '/Applications/Blender 4.5.12.app/Contents/MacOS/Blender'
> ```

Three long build-flag lines follow, then `build system: CMake`. The next section, headed `Python:`, opens `version: 3.11.11 (main, Apr 25 2025, 12:39:20) [Clang 17.0.0 (clang-1700.0.13.3)]`.

*What the arguments list itself says.* `"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" --help` printed 434 lines. The version entry, word for word:

> ```
> -v or --version 
> 	Print Blender version and exit.
> ```

No platform note is printed beside it, while `/?` on the same page prints `Print this help text and exit (Windows only).`

**Observed — hand test, 2026-08-05.** The Creator ran this one, on their own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. They opened Blender 4.5.12 from the Applications folder in the ordinary way and sent screenshots of what was on screen.

*There is an About Blender panel in 4.5, and this is what it holds.* Clicking the Blender icon at the far left of the Topbar and choosing About Blender opens a panel. Its left-hand column reads, line by line:

> ```
> Blender
> Version: 4.5.12 LTS
> 
> Date: 2026-07-20 13:37
> Hash: 84afd5f785f7
> Branch: blender-v4.5-release
> 
> Blender is free software
> Licensed under the GNU General Public License
> ```

Its right-hand column is a set of links: `Donate`, `What's New`, then `Credits`, `License`, `Blender Store`, `Blender Website`. The panel shows four build facts — Version, Date, Hash and Branch — and no build time, no build platform, no build type and no build system. No line about a windowing environment appears. The LTS marking is on the Version line.

*The splash screen and the bottom bar.* The top right corner of the splash image reads `4.5.12 LTS`. The right-hand end of the bar along the bottom of the window reads `4.5.12`, with no LTS marking.

*The Help menu.* Clicking Help in the row of menus along the top drops down, in order:

> ```
> Manual
> Release Notes
> Tutorials
> Support
> User Communities
> ———
> Report a Bug
> Save System Info
> ```

So the Save System Info entry quoted from the manual above is present on this build, at the foot of that menu.

**Observed — hand tests, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. The build was Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64`. Every route above was tried again on this second machine.

*From the command line.* `--version` printed the block quoted under question 1 of this document's Linux block. Its first line is `Blender 4.5.12 LTS`, so the LTS marking rides on the printed text here as on the Mac.

*What the arguments list itself says.* `--help` printed **447 lines**, against 434 on the Mac. The version entry, word for word:

> ```
> -v or --version 
> 	Print Blender version and exit.
> ```

with no platform note, while `/?` on the same page prints `Print this help text and exit (Windows only).` **Searching the whole 447 lines for the letters `qos` found none.** That is worth recording plainly, because 5.0, 5.1 and 5.2 all print a `--qos <level>` argument and this version does not — on this machine.

*From Python, with no window open.* The `--background --python-expr` dump returned every `bpy.app` value. The ones this question asks about:

> ```
> background = True
> binary_path = '/opt/blender/blender-4.5.12-linux-x64/blender'
> build_branch = b'blender-v4.5-release'
> build_hash = b'84afd5f785f7'
> build_date = b'2026-07-21'
> build_platform = b'Linux'
> build_system = b'CMake'
> build_type = b'Release'
> debug_ffmpeg = False
> module = False
> portable = True
> tempdir = '/tmp/blender_cWK6Sg/'
> version = (4, 5, 12)
> version_cycle = 'release'
> version_file = (4, 5, 92)
> version_string = '4.5.12 LTS'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

The whole listing holds **70 names**, the same count as the Mac build. The four Linux builds' name lists were also set against each other, which settles something no single version could see: this build holds a name `debug_ffmpeg` that the 5.0, 5.1 and 5.2 Linux builds do not, and it lacks a name `memory_usage_undo` that all three of those hold.

*What Blender itself offers as commands.* `-c help` printed:

> ```
> Blender Command Listing:
> 	extension
> ```

the same as the Mac.

*As a saved file, with no window open.* In a terminal:

`/opt/blender/blender-4.5.12-linux-x64/blender --background --python-expr "import bpy; bpy.ops.wm.sysinfo(filepath='<a folder outside the project>/system-info.txt')"`

Blender printed `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)` and then `Blender quit`, and the file was written. Its opening and Blender section, word for word:

> ```
> =========================================
> = Blender 4.5.12 LTS System Information =
> =========================================
> 
> 
> Blender:
> =========================================
> 
> version: 4.5.12 LTS, branch: blender-v4.5-release, commit date: 2026-07-20 13:37, hash: 84afd5f785f7, type: Release
> build date: 2026-07-21, 01:31:22
> platform: 'Linux-6.8.0-87-generic-x86_64-with-glibc2.35'
> binary path: '/opt/blender/blender-4.5.12-linux-x64/blender'
> ```

Three long build-flag lines follow, then `build system: CMake`. The next section, headed `Python:`, opens `version: 3.11.11 (main, Apr 25 2025, 12:39:20) [GCC 11.2.1 20220127 (Red Hat 11.2.1-9)]`. The letters LTS appear twice in the file, in the title and on the `version:` line, exactly as on the Mac. The Python version number is the same as the Mac's; what compiled it is not.

*The About Blender panel, with a window open on the machine's own display.* Blender was started with a window, `bpy.ops.wm.splash_about` was called from a timer, and the window was photographed with Blender's own screenshot command. Its left-hand column reads, line by line:

> ```
> Blender
> Version: 4.5.12 LTS
> 
> Date: 2026-07-20 13:37
> Hash: 84afd5f785f7
> Branch: blender-v4.5-release
> Windowing Environment: X11
> 
> Blender is free software
> Licensed under the GNU General Public License
> ```

**This settles the one row nobody had ever seen.** The panel carries five rows here, not four — the `Windowing Environment` line the other versions' manuals describe as appearing "On Linux", reading `X11`. On the Creator's Mac that row is absent. The right-hand column of links is the same as the Mac's: `Donate`, `What's New`, then `Credits`, `License`, `Blender Store`, `Blender Website`.

*The Help menu, with a window open.* `bpy.ops.wm.call_menu(name='TOPBAR_MT_help')` was called and the window photographed. The menu drops down, in order:

> ```
> Manual
> Release Notes
> Tutorials
> Support
> User Communities
> ———
> Report a Bug
> Save System Info
> ```

Line for line what the Creator saw on the Mac, including `Tutorials`, the absence of `Get Involved`, and the last entry written without a trailing ellipsis.

**Outside the interface. Observed on two machines.** Yes, and this is the stronger route. `--version` is a command line argument and exits immediately. `bpy.app` is plain Python data, reachable via `-P, --python <filepath>`, `--python-expr <expression>`, or a registered `-c / --command`; the manual notes for `--command` that "This implies --background mode" (the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30), so no window is involved. The splash screen, the About Blender panel and the Help menu route are interface-only, and all three were looked at on 2026-08-05 as recorded above. What was previously inferred here is now observed: with `--background`, every `bpy.app` value listed above was returned, and `bpy.ops.wm.sysinfo` wrote its file, with no window open at any point.

**Operating system. Observed for macOS and for Linux.** `--version` is listed with no platform note, unlike neighbouring arguments in the same list that do carry one (`/?` is marked "Windows only", `-r, --register` "Windows & Linux only"). On the Creator's Mac, `--version` ran and printed as quoted above, so it is not Windows-only in fact; it ran on the rented Linux machine too. `bpy.app` attributes carry no platform notes; all of those listed above returned a value on both machines. Four things differ between the two: `build_platform` reads `b'Darwin'` on the Mac and `b'Linux'` on the rented machine; `portable` reads `False` on the Mac and `True` on the rented machine; the help output is 434 lines on the Mac and 447 on the rented machine; and the About Blender panel shows four rows on the Mac and five on the rented machine, the extra one being `Windowing Environment: X11`. Everything else about this answer came back the same on both. Windows remains untested.

---

## 4. Does Blender name one release as the one to use for ordinary work?

**Label: documented** for the two statements below. **Inferred** for the conclusion that no single release is named as the default.

The manual does not name one. It frames the choice as the reader's:

> "Each package has the trade off of newest features versus stability. The package that is right for you depends on your requirements for those two. A studio, for example, might want to have long-term support, while a hobbyist may want newer features, while others may just want to test upcoming features. Each package described below has something just right for everyone."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/index.html — read 2026-07-30.

For production work specifically, it does give a recommendation, and it is LTS:

> "New Blender versions may add, remove or change functionality that affects the results of production files. For a given project, it is advisable to use a single LTS version of Blender. LTS versions receive bug fixes for two years."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30. That page's own framing: "This page contains tips for setting up Blender in environments such as animation studios and schools."

The stable release is described without being recommended over the others: *(Citation check, 2026-08-05: the words "the general-purpose one" have been cut. The quote below carries only that the package is described, not that it is the general-purpose choice.)*

> "A package that contains the latest features and is considered stable without regressions."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/index.html — read 2026-07-30.

**The inference.** Reading those three together: the download page deliberately declines to pick one ("depends on your requirements", "something just right for everyone"), while the deployment page picks LTS but only for a project, and only to the extent of "a single LTS version" rather than a named one. The step I took is that the absence of a recommendation on the page whose whole purpose is choosing a package means no release is named as the default. A reader has to supply that step, so this is inference, not documentation.

**Not covered.** This does not settle which LTS version to use when several are supported at once — the manual's version list shows 4.5, 4.2, 3.6, 3.3, 2.93 and 2.83 all marked LTS. It does not settle anything about Blender's own website, which is where a "recommended download" would live and which is not among my sources. The one useful date fact is that this version's support has an end: "will be maintained until July 2027" (the release notes, https://developer.blender.org/docs/release_notes/4.5/, read 2026-07-30).

**Observed — hand test, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. This does not answer the question, and is recorded only because it is the nearest thing to a default that any machine could be made to show. The one command-line fetch route the manual documents, `snap install blender --classic`, was run with no channel named. It printed:

> `blender 5.2.0 from Blender Foundation (blenderfoundation**) installed`

and `snap list blender` then showed `Tracking: latest/stable`. So that route's own default is whatever the newest stable release is, which on 2026-08-06 was 5.2.0. `snap info blender` shows that older lines can be asked for by name, including `4.5lts`. **That is Snap's default, not Blender naming a release**, and nothing here says Blender recommends one.

**Outside the interface.** The recommendation is advice in documentation; it does not depend on how Blender is driven. The deployment page it appears on is written for automated, unattended setups, so if anything it applies more directly there.

**Operating system.** The advice is given once, with no system qualification. One system-specific limit does exist for this version, from the requirements snapshot (`Blender Sources/requirements/requirements_2026-08-01.txt`, no web address, read 2026-08-04 as filed): "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur)." That page names 4.5 in so many words, which is why I use it.

---

## 5. Can one Blender installed on a computer cause trouble for another that is already there?

**Label: documented** for the three specific ways below, lettered (a), (b) and (c). **Inferred** for the fourth thing, the shared configuration folder, under "What keeps them apart". *(Citation check, 2026-08-05: this line previously said "the four specific ways below". Only three lettered ways appear, and the fourth point is inferred rather than documented. Corrected.)*

The starting point is that co-existence is intended:

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30.

> "Note, you do not have to overwrite your existing Blender installation. It's perfectly possible to have multiple versions installed side by side."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html — read 2026-07-30.

> "When using this method of installation, it is possible to have multiple versions of Blender installed."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html — read 2026-07-30.

Against that, three documented ways one can affect another.

**(a) File association and thumbnails are system-wide, and one installation takes them.**

> "**Operating System Settings**
> Make this installation your default Blender (MS-Windows & Linux only)."

> "**Register**
> Make the currently in use Blender installation the default for generating thumbnails and the default for opening blend-files."

> "**Unregister**
> Remove file association & thumbnailer."

> "**For All Users**
> Register Blender for all users, requires escalated privileges."

> "On Linux, if Blender is installed from a package manager such as Snap, file association is handled by the package manager."

Source: the manual, https://docs.blender.org/manual/en/4.5/editors/preferences/system.html — read 2026-07-30.

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html — read 2026-07-30.

That installing or registering a second Blender takes the association away from the first is **inferred** — the source says the setting makes "this installation" the default, and a default is singular; the step a reader supplies is that setting it for one unsets it for the other.

**(b) On macOS, the normal update replaces the existing application outright.**

> "Install the new version by overwriting the current Blender.app in the Applications folder. You can rename Blender.app or place it in a different folder to have more than one version at a time."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/macos.html — read 2026-07-30. Read plainly: following the documented steps removes the Blender that was there. Keeping both is the documented exception, not the default.

**(c) Settings copied from an older Blender can break the newer one.**

> "Selecting this option will copy preferences from an older version of Blender. Doing so will copy preferences and startup files from the previous version of Blender and load them. This will include installed add-ons and extensions."

> "Some previous Blender add-ons and extensions may not be compatible with a new version of Blender, and choosing this option may lead to errors on startup. If this occurs, the recommended first step is to try Loading Factory Settings."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/configuration/defaults.html — read 2026-07-30. The picture on that page, `_images/getting-started_configuration_defaults_preferences.png`, shows the button labelled "Import Blender 4.4 Preferences", so the offer names the specific older version it found. I looked at that image on 2026-07-30.

**What keeps them apart.** Configuration is separated by version number, and only by version number:

> "The preferences need to be imported from previous versions because the configuration files of each Blender version are stored in separate folders."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/configuration/defaults.html — read 2026-07-30.

> "Linux — `$HOME/.config/blender/4.5/`"
> "macOS — `/Users/$USER/Library/Application Support/Blender/4.5/`"
> "Windows — `%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\4.5\`"

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html — read 2026-07-30.

That two different 4.5 builds — say a 4.5 stable release and a 4.5 daily build — would share one configuration folder and could therefore disturb each other is **inferred**. The step: the documented path contains only the major and minor number (`4.5`), and nothing in the path distinguishes one build from another, so any two builds reporting 4.5 land in the same folder. The sources never say this outright.

Two documented ways to stop even that:

> "To enable this, create a folder named `portable` at the following locations… This folder will then store preferences, startup file, installed extensions and presets."

> "The `BLENDER_USER_RESOURCES` environment variable can be set to a custom directory to replace the default user directory."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html — read 2026-07-30.

**Not covered.** Nothing here says what happens when two Blenders run at the same time, nor whether they contend for the GPU, memory, or the temporary directory. Nothing says whether two Blenders can write to the same temporary directory safely (the manual lists the temporary directory's selection order but says nothing about sharing). Nothing says whether one installation's bundled Python or libraries can interfere with another's. Nothing addresses the Microsoft Store, Steam or Snap installations sitting alongside a hand-extracted one, beyond the Snap file-association note above.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. This settles the inferred step above — that two builds carrying the same two-number version would share one settings folder.

*Set-up.* A second copy of the installed application was made, in a working folder outside `/Applications`, with `cp -Rc "/Applications/Blender 4.5.12.app" <working folder>/Blender.app`. That gave two copies of Blender 4.5.12 on the one computer, in two different places.

*Action.* This was run against each of the two, changing only the path:

`"<path to the copy>/Contents/MacOS/Blender" --background --python-expr "import bpy; print('USER', bpy.utils.resource_path('USER')); print('LOCAL', bpy.utils.resource_path('LOCAL')); print('SYSTEM', bpy.utils.resource_path('SYSTEM')); print('BINARY', bpy.app.binary_path)"`

*What came back.* From the one in `/Applications`:

> ```
> USER /Users/artist/Library/Application Support/Blender/4.5
> LOCAL /Applications/Blender 4.5.12.app/Contents/Resources/4.5
> SYSTEM /Library/Application Support/Blender/4.5
> BINARY /Applications/Blender 4.5.12.app/Contents/MacOS/Blender
> ```

From the second copy, in the working folder:

> ```
> USER /Users/artist/Library/Application Support/Blender/4.5
> LOCAL <working folder>/Blender.app/Contents/Resources/4.5
> SYSTEM /Library/Application Support/Blender/4.5
> BINARY <working folder>/Blender.app/Contents/MacOS/Blender
> ```

The two copies printed the same USER path and the same SYSTEM path. Only LOCAL followed the copy. Listing `~/Library/Application Support/Blender/` gave one folder per two-number version and no more: `4.1`, `4.2`, `4.3`, `4.4`, `4.5`, `5.0`, `5.1`, `5.2`, `data`.

*On the Register controls.* `"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" --help` was run on the Mac. The registration arguments are printed in the Mac build's own help, each with a platform note, word for word:

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

**Observed — hand test, 2026-08-05.** The Creator ran this one, on their own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. They opened Blender 4.5.12 in the ordinary way, opened Edit ‣ Preferences, clicked System in the list down the left-hand side, and sent a screenshot of that page.

The screenshot shows the System page with five sections, in this order: `Cycles Render Devices`, `Network`, `Memory & Limits`, `Video Sequencer` and `Sound`. No section headed `Operating System Settings` appears in it, and no button named `Register`, `Unregister` or `For All Users` appears in it. One screenshot was supplied, so what it shows is the page from `Cycles Render Devices` at the top down to the last row of `Sound`; whether anything sits below `Sound` is not shown.

**Observed — hand tests, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH, with all four researched versions extracted under `/opt/blender`. The registration controls are marked "Windows & Linux only", so this is the first machine on which they could be used at all. It is a rented machine that is thrown away, which is why changing a file-type association was acceptable here and was not on the Creator's own computer. Everything that changed is written out below, and everything created was removed afterwards.

**This settles the inference above — that registering a second Blender takes the association from the first. It does, and the way it does so is by writing over the first one's file.**

*The state before anything was registered.* `xdg-mime query default application/x-blender` printed `blender.desktop`, which is a file the rented machine's own image had put in `/usr/share/applications/`. There was no `~/.local/share/applications`, no `~/.local/share/mime`, no `~/.config/mimeapps.list`, and no `/usr/local/share/applications`, `/usr/local/share/mime` or `/usr/local/share/thumbnailers`. Every file under `~/.local/share` and `/usr/local/share` was listed and kept, so that the end state could be set against it.

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

with exit code 0. Sixteen files appeared, all under `~/.local/share`: the desktop file, the icon, `mime/packages/x-blender.xml`, `mime/application/x-blender.xml`, and twelve files of the rebuilt mime database. `~/.config/mimeapps.list` was created and reads:

> ```
> [Default Applications]
> application/x-blender=blender.desktop
> ```

The desktop file it wrote carries:

> ```
> Name=Blender
> Exec=/opt/blender/blender-4.5.12-linux-x64/blender %f
> Icon=blender
> MimeType=application/x-blender;
> ```

*Registering a second Blender afterwards.* `/opt/blender/blender-5.2.0-linux-x64/blender --register` printed the same shape of output, with `Register: /opt/blender/blender-5.2.0-linux-x64/blender` on the first line. **No new file appeared** — the set of files was the same afterwards. What changed is the content of the one file both wrote:

> `Exec=/opt/blender/blender-5.2.0-linux-x64/blender %f`

So the second registration replaced the first. Because both write a file of the same name, `xdg-mime query default application/x-blender` still answers `blender.desktop` throughout; the answer is the same word and means a different program.

*Registering for all users.* `/opt/blender/blender-5.1.2-linux-x64/blender --register-allusers` printed:

> ```
> Register: /opt/blender/blender-5.1.2-linux-x64/blender
> - Setup symbolic-links in: /usr/local/bin
> - Setup icon: /usr/local/share/icons/hicolor/scalable/apps/blender.svg
> - Setup desktop-file: /usr/local/share/applications/blender.desktop
> - Setup mime type: /usr/local/share/mime/application/x-blender.xml
> - Setup thumbnailer: /usr/local/share/thumbnailers/blender.thumbnailer
> ```

It does two things the current-user form did not: it created the links in `/usr/local/bin`, and it set up a thumbnailer. The current-user form had declined to make links because `~/.local/bin` was not on the path, and it never mentioned a thumbnailer at all. Both write under the places the manual names — `~/.local` for one user, `/usr/local` for all.

*Unregistering.* `/opt/blender/blender-5.0.1-linux-x64/blender --unregister` printed:

> ```
> Unregister: /opt/blender/blender-5.0.1-linux-x64/blender
> - Remove symbolic-links in: ~/.local/bin
> - Remove icon: ~/.local/share/icons/hicolor/scalable/apps/blender.svg
> - Remove desktop-file: ~/.local/share/applications/blender.desktop
> - Remove mime type: ~/.local/share/mime/application/x-blender.xml
> - Remove mime type as default
> ```

Worth noting: 5.0.1 had never registered anything, and it removed the registration 5.2.0 had made. `--unregister` clears whatever registration is there, not only its own. `/opt/blender/blender-5.1.2-linux-x64/blender --unregister-allusers` did the same for the all-users side, including the thumbnailer.

*What unregistering leaves behind.* Set against the file list taken at the start, unregistering did **not** return the machine to its earlier state. Twelve rebuilt mime-database files were left in `~/.local/share/mime` and twelve more in `/usr/local/share/mime`; the folders `applications`, `icons`, `mime` and `thumbnailers` were left in place though empty; and `~/.config/mimeapps.list` was left holding `application/x-blender=blender.desktop`. `xdg-mime query default application/x-blender` therefore still answered `blender.desktop` afterwards. All of that was removed by hand, and the machine's file list then matched the one taken before anything was registered, exactly.

*Whether a Preferences panel of these controls exists on Linux.* Every one of the four Linux builds was asked, with no window, to list the preferences panels it registers. All four register a panel labelled `Operating System Settings`. Opened with a window on the machine's own display and photographed with Blender's own screenshot command, Blender 4.5.12's Preferences System page shows, in order: `Cycles Render Devices`, `Display Graphics`, `Operating System Settings`, `Network`, `Memory & Limits`, and more below. The `Operating System Settings` section reads `Open blend files with this Blender version` above two buttons, `Register` and `Unregister`, with a checkbox `For All Users` beneath them — the controls the manual quotes. **On the Creator's Mac neither that section nor `Display Graphics` appears at all.**

*Two builds of one version, in two places.* A second copy of this build was made with `cp -a` into a working folder. Run with `BLENDER_USER_RESOURCES` deliberately unset and with `--factory-startup`, so that the default path came back and nothing was saved, the two printed:

> ```
> USER /root/.config/blender/4.5
> LOCAL /opt/blender/blender-4.5.12-linux-x64/4.5
> SYSTEM ''
> BINARY /opt/blender/blender-4.5.12-linux-x64/blender
> ```

> ```
> USER /root/.config/blender/4.5
> LOCAL /root/smoozy/Builds/work/06/4.5/copy/4.5
> SYSTEM ''
> BINARY /root/smoozy/Builds/work/06/4.5/copy/blender
> ```

The same user path, only the build's own folder following the copy. That agrees with the Mac. One difference: the system path comes back as an **empty string** on this machine, where the Mac returned a real path.

*Two builds of one version from two different sources.* On the rented machine a Blender 5.2.0 was also installed through Snap, alongside the blender.org 5.2.0 already extracted under `/opt/blender`. Both report the same user path, `/root/.config/blender/5.2`. So builds from two different sources carrying the same two-number version do share one settings folder. That is recorded in full under question 5 of the 5.2 findings.

**Outside the interface. Observed on two machines.** The file-association and thumbnail effects are properties of the operating system, not of Blender's window, so they apply however Blender is driven — but they only matter to something that opens blend-files by double-clicking, which an automated setup would not do. Importing preferences (c) is offered only through the first-run splash screen dialog, so it is interface-only; an automated deployment would never be offered it. The shared-configuration-folder concern applies in background mode too, since the paths are the same — and that part is now **observed**: both runs above were made with `--background`, and both returned the same USER path with no window open. The sources do not state any of these differences directly; the reading of which route is interface-only comes from where the manual places each one (splash screen versus operating system settings versus directory paths).

**Operating system. Observed in part for macOS and for Linux.** *(Citation check of the Linux hand tests, 2026-08-06: this line still opened "Observed in part for macOS" although a Linux result had been written into it below. Corrected to name both machines.)* (a) is marked "MS-Windows & Linux only" for the Register controls; on Linux registration writes "under: /usr/local for all users, otherwise ~/.local is used". (b) is macOS only. (c) and the shared-configuration-folder point are stated with no system qualification. *(Citation check, 2026-08-05: this line referred to a "(d)" that does not exist in the answer. Corrected to name the fourth point instead.)* The configuration folder location differs per system, as quoted. Observed on the Creator's Mac: the Mac build's own `--help` marks all four registration arguments "(Windows & Linux only)", and the screenshot of the Preferences System page shows no `Operating System Settings` section among the five sections it covers. **Observed on the rented Linux machine, 2026-08-06:** all four registration arguments run there and do what they say, the `Operating System Settings` section is on the Preferences System page with `Register`, `Unregister` and `For All Users` in it, registration writes under `~/.local` for one user and `/usr/local` for all as the manual states, and **registering a second Blender does take the association from the first, by writing over its file**. So the two machines differ sharply, and in the way the manual's own platform note predicts. Windows remains untested.

---

## 6. Does installing a Blender disturb a render already running on the same computer?

**Label: not in the sources**

Nothing in the 4.5 manual, the 4.5 Python API documentation, the 4.5 release notes or Cycles' own files addresses installing Blender while a render is running.

The nearest documented facts, which do not answer the question:

> "Blender downloads can be extracted to any directory on the system, as a self contained installation."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30.

> "When choosing the zip-file, you have to manually extract Blender to the desired folder… No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html — read 2026-07-30.

I deliberately do not draw a conclusion from "self contained". Whether a render already running is disturbed depends on what the installer touches while running and how the operating system treats files in use, and no source I have says either.

**Where I looked.** I read all four installation pages in full (index, Windows, macOS, Linux, Steam), the deployment page in full, and the directory-layout page in full. I searched the whole 4.5 manual, the whole 4.5 Python API documentation and all 4.5 release notes for: `while Blender is running`, `while running`, `in use`, `file lock`, `locked file`, `process is running`, `kill`, `terminate`, `SIGTERM`, `crash during`, `interrupted`, `resume render`, `partial render`, `unattended`, `silent install`, `rollback`, `idempot`. Nothing addressed a running render alongside an installation. I also read the manual's rendering-an-animation page (https://docs.blender.org/manual/en/4.5/render/output/animation.html), which discusses interrupting a render only in the sense of the user choosing to stop it: "May need to interrupt rendering to use the computer, and want to be able to resume rendering where you left off." That is advice for choosing a frame-sequence workflow, not a statement about installation.

**Not covered by the sources.** Everything the question asks. In particular: whether an installer that requires administrator rights (the Windows installer) behaves differently from a plain archive extraction while a render is running; whether extracting a new Blender over the folder of a Blender currently rendering is safe; whether a package manager update (Snap, Steam, Microsoft Store — all of which the manual says update automatically) can replace files under a running render. The first and third of those are still open. The second was tested by hand, below.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The installed Blender in `/Applications` was never touched; every test used a throwaway copy in a working folder outside the project.

*Set-up, for both runs.* A copy of Blender 4.5.12 was made into an empty working folder with `cp -Rc`, together with an empty output folder beside it. The scene was Blender's own factory startup scene — no file was opened. The render was started from the copy, with the window switched off:

`"<working folder>/Blender.app/Contents/MacOS/Blender" --background --factory-startup --python-expr "import bpy; bpy.context.scene.cycles.samples=64; bpy.context.scene.render.resolution_percentage=50" -E CYCLES -F PNG -o "<working folder>/out/frame_" -s 1 -e 30 -a`

*First run — a second Blender put on the computer beside the running one.* Once four image files had appeared, `cp -R "/Applications/Blender.app" "<working folder>/second/Blender.app"` was run in another terminal, copying a whole second Blender in beside the rendering one. The copy finished with exit code 0. The render then carried on and ended by itself. What was counted at each point:

> ```
> ### image files on disk before the action: 4 at 21:20:38
> ### action exit code 0
> ### action finished at 21:20:44
> ### image files just after the action: 5
> ### render process ended, exit code 0 at 21:21:56
> ### image files at the end: 30
> ```

All thirty numbered images were written, ending `frame_0028.png`, `frame_0029.png`, `frame_0030.png`. The render printed no error.

*Second run — the same Blender copied over the exact folder the render was running from.* Same set-up, a fresh working folder. Once four image files had appeared, `cp -R "/Applications/Blender 4.5.12.app/" "<working folder>/Blender.app/"` was run — the contents of the installed 4.5.12 copied straight over the folder the render was reading from. That copy finished with exit code 0. Counted at each point:

> ```
> ### image files on disk before the action: 4 at 21:27:12
> ### action exit code 0
> ### action finished at 21:27:17
> ### image files just after the action: 5
> ### render process ended, exit code 0 at 21:28:19
> ### image files at the end: 30
> ```

All thirty numbered images were written. The render printed no error.

**Observed — hand tests, 2026-08-06, on the rented Linux machine. The two machines disagree on the second run, and neither result is set aside.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. The machine was shared with other work at the time, so nothing here says how long anything took. Nothing under `/opt/blender` was touched; both runs used a throwaway copy made with `cp -a`.

*Set-up, for both runs.* A copy of the extracted Blender 4.5.12 was made into an empty working folder with `cp -a`, with an empty output folder beside it. The scene was Blender's own factory startup scene — no file was opened. The render was started from the copy, with no window:

`<working folder>/blender/blender --background --factory-startup --python-expr "import bpy; bpy.context.scene.cycles.samples=64; bpy.context.scene.render.resolution_percentage=50" -E CYCLES -F PNG -o "<working folder>/out/frame_" -s 1 -e 30 -a`

*First run — a second Blender put on the computer beside the running one.* Once four image files had appeared, `cp -a /opt/blender/blender-5.2.0-linux-x64 <working folder>/second/blender` was run, copying a whole second Blender in beside the rendering one. What was counted at each point:

> ```
> ### image files on disk before the action: 4
> ### action: a whole second Blender (blender-5.2.0-linux-x64) copied in beside the running one
> ### the copy's own exit code: 0
> ### is the second Blender there? /root/smoozy/Builds/work/render2/4.5/A/second/blender
> ### image files just after the action: 4
> ### render process ended, exit code 0
> ### image files at the end: 30
> ```

Its last lines were `Saved: '<working folder>/out/frame_0030.png'`, `Time: 00:02.35 (Saving: 00:00.04)` and `Blender quit`. No line in the log contained the word error. **This agrees with the Mac.**

*Second run — the same Blender copied over the exact folder the render was running from.* Same set-up, a fresh working folder. Once four image files had appeared, `cp -a /opt/blender/blender-4.5.12-linux-x64/. <working folder>/blender/` was run — the contents of the extracted 4.5.12 copied straight over the folder the render was reading from. The copy itself finished with exit code 0. What came back:

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
> Fra:5 Mem:8.33M (Peak 85.62M) | Time:00:00.00 | Mem:0.09M, Peak:0.09M | Scene, ViewLayer | Updating Baking
> Fra:5 Mem:8.33M (Peak 85.62M) | Time:00:00.00 | Mem:0.09M, Peak:0.09M | Scene, ViewLayer | Updating Device | Writing constant memory
> Fra:5 Mem:8.33M (Peak 85.62M) | Time:00:00.00 | Mem:0.09M, Peak:0.09M | Scene, ViewLayer | Loading denoising kernels (may take a few minutes the first time)
> Fra:5 Mem:8.33M (Peak 85.62M) | Time:00:00.00 | Mem:0.09M, Peak:0.09M | Scene, ViewLayer | Sample 0/64
> Fra:5 Mem:46.06M (Peak 85.62M) | Time:00:00.05 | Remaining:00:03.19 | Mem:37.66M, Peak:37.66M | Scene, ViewLayer | Sample 1/64
> ```

That is an ordinary progress line for frame 5, cut off mid-frame. The render did not print an error of its own; the words `Bus error (core dumped)` came from the shell that started it. Four images were on disk at the end instead of thirty, and the four already written before the action were still there and intact.

**The same test on the Creator's Mac finished all thirty frames with exit code 0 and printed no error. On this machine it stopped the render, in this version and in all three others.** Both results stand; neither is chosen between. The same test was run again on its own for 5.2 and came out the same way.

**Outside the interface. Observed on two machines.** Renders driven from outside the interface are exactly the case the question is about, and the sources are silent. Every run on both machines was a background render started from a terminal with no window.

**Operating system. Observed for macOS and for Linux, and they do not agree.** Not stated by the sources for any system. On macOS 26.5.2 with an Apple M1 Pro chip, both runs completed all thirty frames. On the rented Linux machine, putting a second Blender beside the running one left the render untouched, and copying a Blender over the folder the render was running from stopped the render with exit code 135 and four frames of thirty written. Windows remains untested, and the installer and package-manager routes could not be tried on either computer.

---

## 7. Can a computer be asked which Blenders it holds?

**Label: not in the sources** for a general way to enumerate installed Blenders. **Documented** for two narrower things below.

Blender itself provides nothing for this in these sources. I found no command line argument, no operator and no Python API call that lists Blender installations on a machine.

What is documented:

**(a) A package manager can list its packages, on Linux.**

> "Installing Blender via the distribution's native mechanisms ensures consistency with other packages on the system and may provide other features (given by the package manager), such as listing of packages, update notifications and automatic menu configuration."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html — read 2026-07-30. This lists packages the package manager installed; it says nothing about Blenders extracted by hand.

**(b) On Windows, versions installed by the installer are listed where the operating system lists installed programs.**

> "To remove a previously installed version of Blender, use Windows settings or control panel to uninstall the desired version."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html — read 2026-07-30. That you can therefore *see* which versions are there is **inferred** — the step a reader supplies is that choosing "the desired version" to uninstall requires them to be listed. The source only describes removing, not listing.

**(c) A newly started Blender detects a previous version's settings, not its installation.**

> "Selecting this option will copy preferences from an older version of Blender."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/configuration/defaults.html — read 2026-07-30, and the picture on the same page (`_images/getting-started_configuration_defaults_preferences.png`, looked at 2026-07-30) shows the button naming the found version: "Import Blender 4.4 Preferences". That this reports a configuration folder rather than an installed program is **inferred** from the same page's explanation that "the configuration files of each Blender version are stored in separate folders", and from those folders being described as user directories independent of where the program lives. A configuration folder can outlive the Blender that made it, so this is not a reliable list of what is installed.

**Where I looked.** All four installation pages, the deployment page and the directory-layout page in full. Searches across the whole 4.5 manual, the whole 4.5 Python API documentation and all 4.5 release notes for: `multiple versions`, `multiple blender`, `side by side`, `co-exist`, `coexist`, `installed`, `uninstall`, `remove blender`, `list of builds`, `listing of packages`, `system info`, `System Information`. I read the Help System page describing `system-info.txt` in full — it reports on the one Blender that produced it ("the path in which Blender is running"), not on others. I searched Cycles' own files for anything version- or install-related: nothing.

**Not covered by the sources.** Nothing here gives a way to find Blenders that were simply extracted to a folder, which is the manual's own primary installation method on all three systems ("Blender downloads can be extracted to any directory on the system"). Nothing covers macOS at all — the manual describes renaming `Blender.app` or putting it in a different folder to keep several, with no way to find them again. Nothing says whether Steam or the Microsoft Store expose their Blender to any listing.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2.

*Set-up.* The computer held four Blenders in `/Applications` — 4.5.12, 5.0.1, 5.1.2 and 5.2.0 — and, at the same time, four further copies of the same four builds sitting in ordinary folders under a working directory at `/private/tmp/…/hand-tests/Builds/06/`, made with `cp -Rc`.

*Action and what came back.* Three ways of asking the computer were tried in a terminal.

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

`system_profiler SPApplicationsDataType` printed one block per Blender. The 4.5 block, word for word:

> ```
>     Blender 4.5.12:
> 
>       Version: 4.5.12
>       Obtained from: Identified Developer
>       Last Modified: 2026-07-21, 03:41
>       Kind: iOS
>       Signed by: Developer ID Application: Stichting Blender Foundation (68UA947AUU), Developer ID Certification Authority, Apple Root CA
>       Location: /Applications/Blender 4.5.12.app
>       Get Info String: 4.5.12 2026-07-21, Blender Foundation
> ```

Blocks in the same shape were printed for 5.0.1, 5.1.2 and the one at `/Applications/Blender.app` (shown as `Version: 5.2.0`). None of the three commands named any of the four copies sitting in the working folder. Asking the index about that folder directly, `mdfind -onlyin <working folder> "kMDItemFSName == 'Blender.app'"`, returned nothing at all.

*What a Blender says about where it is.* From `--background --python-expr`, `bpy.app.binary_path` read `'/Applications/Blender 4.5.12.app/Contents/MacOS/Blender'`, and the system-information file quoted under question 3 carries the same path on its `binary path:` line. Both report the one Blender that produced them.

**Observed — hand test, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. **This settles the Linux half of the question**, which route (a) above only pointed at.

*Set-up.* The machine held **five** Blenders when this was run: the four researched versions extracted under `/opt/blender` — 4.5.12, 5.0.1, 5.1.2 and 5.2.0 — and a fifth that came with the rented image, at `/opt/blender-5.0.0-linux-x64`, with `/usr/bin/blender` pointing at it. None had been installed by a package manager.

*Action and what came back.* Each of these was run in a terminal.

`apt list --installed | grep -i blender` printed nothing at all. `dpkg -l blender` printed:

> `dpkg-query: no packages found matching blender`

`snap list` printed eight entries — `bare`, `core20`, `core22`, `firefox`, `gnome-42-2204`, `gtk-common-themes`, `lxd`, `snapd` — and no blender among them.

`which -a blender` printed one line:

> `/usr/bin/blender`

which `readlink -f` resolves to `/opt/blender-5.0.0-linux-x64/blender`.

`update-alternatives --list blender` printed:

> `update-alternatives: error: no alternatives for blender`

`find / -maxdepth 6 -name blender -type f -perm -u+x` printed all five:

> ```
> /opt/blender/blender-5.2.0-linux-x64/blender
> /opt/blender/blender-5.0.1-linux-x64/blender
> /opt/blender/blender-5.1.2-linux-x64/blender
> /opt/blender/blender-4.5.12-linux-x64/blender
> /opt/blender-5.0.0-linux-x64/blender
> ```

So on Linux the answer is the same shape as on macOS: **a package manager names only what it installed, and none of these five was installed that way, so neither `apt` nor `snap` named any of them.** Only a plain search of the file system found them all. The documented route (a) is real but narrow.

*After a package-managed Blender was added.* A Blender 5.2.0 was then installed with `snap install blender --classic`. `snap list` then named it, and only it:

> ```
> blender  5.2.0  7599  latest/stable  blenderfoundation**  classic
> ```

`apt list --installed` still named nothing. `which -a blender` then printed three paths — `/usr/bin/blender`, `/bin/blender` and `/snap/bin/blender`. So a package manager's listing grew by exactly the one build it installed, and the other five stayed invisible to it.

*What a running Blender says about where it is.* From `--background --python-expr`, `bpy.app.binary_path` read `'/opt/blender/blender-4.5.12-linux-x64/blender'`, and the system-information file quoted under question 3 carries the same path on its `binary path:` line. Both report the one Blender that produced them, exactly as on the Mac.

**Outside the interface. Observed on two machines.** The two documented routes (a Linux package manager, Windows settings) are outside Blender entirely, so they do not depend on Blender's window. Route (c) is a splash-screen dialog and exists only inside the interface. The three macOS listings above were all run from a terminal with no window open, and `bpy.app.binary_path` and the system-information file were both obtained with `--background`.

**Operating system. Observed for macOS and for Linux.** (a) is Linux only, and only for package-manager installations. (b) is Windows only, and only for installer-based installations. (c) is described without qualification but is interface-only. macOS has no documented route; the three commands above are macOS's own, not Blender's, and what each returned is recorded above. **Observed on the rented Linux machine, 2026-08-06:** route (a) works, and it is as narrow as its wording suggests — `apt` and `snap` named none of the five Blenders sitting on the machine, because none had been installed that way, and after one was installed through Snap the listing grew by exactly that one. The two machines agree on the shape of the answer: the operating system's own tools name what was put there their way, and nothing else. Windows remains untested.

---

## 8. What becomes of a render that is running when the Blender it is using is removed?

**Label: not in the sources**

Nothing in the 4.5 manual, the 4.5 Python API documentation, the 4.5 release notes or Cycles' own files says what happens to a running render if its Blender is deleted, uninstalled or overwritten while the render is in progress.

**Where I looked.** I read in full: all four installation pages plus the Steam page (which are where removal is described — the only removal instruction anywhere is the Windows one, "use Windows settings or control panel to uninstall the desired version"), the deployment page, the directory-layout page, the command line arguments page, and the rendering-output animation page. I searched the whole 4.5 manual, the whole 4.5 Python API documentation and all 4.5 release notes for: `uninstall`, `remove blender`, `removing blender`, `while Blender is running`, `while running`, `in use`, `file lock`, `locked file`, `process is running`, `kill`, `terminate`, `SIGTERM`, `crash during`, `interrupted`, `resume render`, `partial render`. The `uninstall` hits are all about graphics drivers, extensions, or studio lights — none about Blender itself while working.

The only related documented statement concerns a render being stopped for any reason, and it is advice rather than a description of behaviour:

> "May need to interrupt rendering to use the computer, and want to be able to resume rendering where you left off."

Source: the manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30. It appears in a list of reasons to render an animation as a sequence of separate image files rather than one video file. It does not say what happens when a render is cut off, only that the frame-sequence approach lets you carry on from where it stopped.

**Not covered by the sources.** Everything the question asks: whether the running program keeps going, whether frames already written survive, whether anything is left in the temporary directory, whether it differs between deleting a folder and running an uninstaller. The deletion case was tested by hand, below; the uninstaller case could not be, as macOS has no uninstaller for a Blender.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The installed Blender in `/Applications` was never touched; the test used a throwaway copy in a working folder outside the project, and nothing outside that folder was deleted.

*Set-up.* A copy of Blender 4.5.12 was made into an empty working folder with `cp -Rc`, with an empty output folder beside it. The scene was Blender's own factory startup scene. The render was started from the copy, with the window switched off:

`"<working folder>/Blender.app/Contents/MacOS/Blender" --background --factory-startup --python-expr "import bpy; bpy.context.scene.cycles.samples=64; bpy.context.scene.render.resolution_percentage=50" -E CYCLES -F PNG -o "<working folder>/out/frame_" -s 1 -e 30 -a`

*Action.* Once four image files had appeared, `rm -rf "<working folder>/Blender.app"` was run in another terminal — the whole application the render was running from, deleted.

*What came back.*

> ```
> ### image files on disk before the action: 4 at 21:32:48
> ### action exit code 0
> ### does the folder still exist? ls: <working folder>/Blender.app: No such file or directory
> ### action finished at 21:32:48
> ### image files just after the action: 4
> ### render process ended, exit code 0 at 21:33:53
> ### image files at the end: 30
> ```

The deletion was permitted and completed at once; the application folder was gone. The render carried on and wrote all thirty numbered images, ending `frame_0028.png`, `frame_0029.png`, `frame_0030.png`, then ended by itself with exit code 0. It printed no error. In the same run against Blender 5.2.0 the render log's closing lines were kept, and they show the last frame being written normally after the deletion: `render | Saved: '…/out/frame_0030.png'`, `render | Time: 00:02.43 (Saving: 00:00.02)`, then `Blender quit`.

*What was left afterwards.* The settings folder for this version was still there and still populated: `ls ~/Library/Application\ Support/Blender/4.5` printed `config`, `datafiles`, `extensions`, `scripts`.

**Observed — hand test, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Nothing under `/opt/blender` was touched; the test used a throwaway copy made with `cp -a`, and nothing outside the working folder was deleted.

*Set-up and action.* As on the Mac: a copy of the extracted Blender 4.5.12, an empty output folder, Blender's own factory startup scene, and the same thirty-frame background render command quoted under question 6 of this document. Once four image files had appeared, `rm -rf <working folder>/blender` was run — the whole folder the render was running from, deleted.

*What came back.*

> ```
> ### image files on disk before the action: 4
> ### action: the whole Blender folder the render is running from, deleted; exit code 0
> ### does the folder still exist? ls: cannot access '/root/smoozy/Builds/work/render/4.5/C/blender': No such file or directory
> ### image files just after the action: 4
> ### render process ended, exit code 0
> ### image files at the end: 30
> ### last file: frame_0030.png
> ```

The last lines the render printed were `Saved: '<working folder>/out/frame_0030.png'`, `Time: 00:02.67 (Saving: 00:00.04)` and `Blender quit`. No line in the log contained the word error.

*What was left afterwards.* That version's settings folder was still there and still held its contents.

**This agrees with the Creator's Mac**, in this version and in the other three: the deletion is permitted, the render carries on, all thirty frames are written, the frames written before the deletion survive, and the settings folder outlives the program.

**Outside the interface. Observed on two machines.** Not stated by the sources, for either case. Every run on both machines was started from a terminal with no window, and every one finished after its own program had been deleted.

**Operating system. Observed for macOS and for Linux, and they agree.** Not stated by the sources for any system. On macOS 26.5.2 the deletion was allowed while the render was running, and the render finished. On the rented Linux machine the same held: the deletion was allowed, the render finished all thirty frames, and the settings folder survived. Windows remains untested, and no uninstaller route exists on either computer to compare against.

---

## 9. Can installing or removing a Blender be done again safely on a computer where it was cut off part-way?

**Label: not in the sources**

Nothing in the 4.5 manual, the 4.5 Python API documentation, the 4.5 release notes or Cycles' own files addresses repeating an installation or a removal after an earlier attempt was interrupted. There is no statement about partial installations, leftover files, resuming, rolling back, or running the same step twice.

**Where I looked.** All four installation pages, the Steam page, the deployment page and the directory-layout page, read in full. Searches across the whole 4.5 manual, the whole 4.5 Python API documentation and all 4.5 release notes for: `idempot`, `rollback`, `partially installed`, `interrupted install`, `interrupted`, `unattended`, `silent install`, `checksum`, `sha256`, `repair`, `uninstall`, `remove blender`, `in use`, `file lock`. No hits bearing on the question.

The nearest documented facts, none of which answer it:

> "Blender downloads can be extracted to any directory on the system, as a self contained installation."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30.

> "The Windows installer will let you choose an installation folder… It requires administrator rights."

> "When choosing the zip-file, you have to manually extract Blender to the desired folder… there is also no need for administrator rights."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html — read 2026-07-30.

One documented failure mode of extraction exists, though it is about the tool used and not about interruption:

> "**Archive Extraction**
> Extracting Blender's archive using 7-zip is not supported. TAR must be used instead. For more details, see issue #104070."

Source: the manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html — read 2026-07-30, under the heading "Known Limitations".

I have deliberately not reasoned from "self contained installation" to a claim that re-extracting is safe. The words say the installation does not depend on files elsewhere; they say nothing about the state left behind by an extraction that stopped half way, nor about the Windows installer, nor about a package manager.

**Not covered by the sources.** Everything the question asks, for every one of the routes the manual describes: archive extraction, the Windows installer, the Microsoft Store, the macOS dmg, Snap, distribution package managers and Steam. The macOS case — copying an application bundle into place — was tested by hand, below. The others were not.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. Everything happened in a working folder outside the project; the installed Blender in `/Applications` was only ever read from.

*First attempt, and what it showed.* The copy was first made with `cp -R "/Applications/Blender 4.5.12.app" <working folder>/Blender.app`, killed after two seconds. 372 MB had arrived, and the file `Contents/MacOS/Blender` existed at its full size. Running that half-copied Blender with `--version` printed nothing at all and gave exit code 137. The same `cp -R` was then run again over the same folder and finished with exit code 0 — but running the finished copy with `--version` also printed nothing and also gave exit code 137. `codesign --verify --verbose` on that finished copy printed:

> ```
> <working folder>/Blender.app: invalid Info.plist (plist or signature have been modified)
> In architecture: arm64
> ```

So a Blender application bundle copied with `cp -R` on this computer would not start whether the copy was interrupted or not, and the interruption was not what stopped it. The test was therefore run again with macOS's own copying tool, so that the interruption was the only thing being tested.

*Second attempt, with `ditto`.* Into a fresh empty folder, `ditto "/Applications/Blender 4.5.12.app" <working folder>/Blender.app` was started and killed after two seconds:

> ```
> ### copy killed at 21:39:28
> ### how much arrived: 362M
> ### is there an executable? -rwxr-xr-x@ 1 artist     admin  175667920 Jul 21 03:49 <working folder>/Blender.app/Contents/MacOS/Blender
> ### running the half-copied Blender with --version:
> ### exit code 137
> ```

The half-copied Blender printed nothing and gave exit code 137. The same `ditto` command was then run again over the same folder, without deleting anything first:

> ```
> ### second copy exit code 0
> ### size now: 798M
> ```

`codesign --verify --verbose` on the finished copy printed `a sealed resource is missing or invalid`, followed by a list of `file added:` lines, all of them compiled-Python cache files under `Contents/Resources/4.5/python/`. Running that finished copy with `--version` gave exit code 0 and printed:

> ```
> Blender 4.5.12 LTS
> 	build date: 2026-07-21
> 	build time: 01:41:35
> 	build commit date: 2026-07-20
> 	build commit time: 13:37
> 	build hash: 84afd5f785f7
> 	build branch: blender-v4.5-release
> 	build platform: Darwin
> 	build type: Release
> ```

which is the same as the installed 4.5.12 printed under question 3.

*What is left behind after a removal.* Separately, after a throwaway copy of 4.5.12 was deleted outright with `rm -rf` (see question 8), `ls ~/Library/Application\ Support/Blender/4.5` still printed `config`, `datafiles`, `extensions`, `scripts`. So removing the application leaves that version's settings folder in place and populated.

**Observed — hand tests, 2026-08-06, on the rented Linux machine. This settles the Linux shape of the question, including the 7-zip limitation quoted above.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Both tools, `tar` and `7z`, were already on the machine; nothing was installed for this.

*A note on which archive was used.* No Blender archive was left on the machine, so one was fetched from Blender's own download server into the working folder, purely so that there was a real archive to extract. It is the 5.2 Linux archive, `blender-5.2.0-linux-x64.tar.xz`, not this version's — a 4.5 archive was not fetched. What is settled below is therefore about a Blender Linux archive and the tools that unpack it, which is what the quoted limitation is about, rather than about this version in particular. The file was checked against Blender's own published checksum file and `sha256sum -c` printed `blender-5.2.0-linux-x64.tar.xz: OK`.

*Extracting with TAR, which the manual says must be used.* `tar -xf blender-5.2.0-linux-x64.tar.xz -C <empty folder>` finished with exit code 0. Running the result with `--version` gave exit code 0 and printed `Blender 5.2.0 LTS` and the rest.

*Extracting with 7-zip, which the manual says is not supported.* It takes two steps, because 7-zip unpacks the compression and the archive separately. Both steps printed:

> `Everything is Ok`

with exit code 0. The second step reported `Folders: 875`, `Files: 5671`. **So 7-zip says it worked.** Running the result with `--version` gave exit code 126, and the shell printed:

> `./sevenzip/blender-5.2.0-linux-x64/blender: Permission denied`

Blender itself printed nothing. Listing the two results side by side shows why:

> ```
> -rw-r--r-- 1 root root 174666336 Jul 14 01:32 sevenzip/blender-5.2.0-linux-x64/blender
> -rwxr-xr-x 1 root root 174666336 Jul 14 01:32 tar-ok/blender-5.2.0-linux-x64/blender
> ```

The 7-zip copy has lost the mark that says the file may be run. Putting that mark back by hand with `chmod +x` was **not** enough — the next run gave exit code 127 and printed:

> `./sevenzip/blender-5.2.0-linux-x64/blender: error while loading shared libraries: /root/smoozy/Builds/work/17/sevenzip/blender-5.2.0-linux-x64/lib/libtbb.so.12: file too short`

Counting the two folders explains that too:

> ```
> tar file count: 5592
> 7z  file count: 5671
> tar symlink count: 79
> 7z  symlink count: 0
> ```

The archive holds 79 internal links, and 7-zip turned every one of them into a plain file. So the documented limitation is real, it breaks the result in two separate ways, and **it fails silently** — the extraction tool reports success both times.

*An extraction cut off part-way, and done again over the same folder.* `tar -xf` was started into an empty folder and killed part-way. What had arrived:

> ```
> 186M	half
> files: 769
> ```

and `ls` on the main program printed `No such file or directory` — it had not been written yet. Trying to run that path gave exit code 127 and the shell printed `No such file or directory`; Blender printed nothing, because there was no Blender there to print anything. The same `tar -xf` command was then run again over the same folder, with nothing deleted first. It finished with exit code 0, and the folder then held 5592 files and 79 links — the same as a clean extraction. Running the result with `--version` gave exit code 0, and its whole output was compared line by line against the 5.2.0 build already extracted under `/opt/blender`: **identical**.

So on Linux an interrupted extraction can simply be done again over the same folder, provided TAR is the tool. That is the same answer the Mac gave for its own shape of the question, reached by a different route.

*What is left behind after a removal.* On the rented machine, after a throwaway copy of this version was deleted outright with `rm -rf` while a background render was running from it (see question 8), that version's settings folder was still there and still held its contents. The same as on the Mac.

**Outside the interface. Observed on two machines.** Not stated by the sources. The deployment page is the one written for automated setups and says only that "deployment can be automated using standard file management tools" — it does not discuss failure or retry. Everything above was done from a terminal on both machines, with no Blender window at any point.

**Operating system. Observed for macOS and for Linux.** Not stated by the sources for any system, other than the Linux-specific 7-zip limitation quoted above. On macOS 26.5.2 an interrupted `ditto` copy could be repeated over the same folder and produced a Blender that started and reported the right version. On the rented Linux machine an interrupted `tar` extraction could be repeated over the same folder and produced a Blender identical to the one already installed — and the 7-zip limitation was confirmed, with the two ways it breaks a build written out above. Windows remains untested, and the Windows installer, the Microsoft Store and Steam could not be tried on either computer; Snap and a distribution package were reached on the rented machine and are recorded under question 2.

---

## Things a person with Blender open could settle by hand in a few minutes

The list that stood here has been worked through. Everything that has since been settled by test is written into the answers above, marked observed. What is left open is below.

1. **Whether a half-finished install can be redone through an installer.** Needs Windows. Start the Blender installer and cancel it part way. Then run the same installer again from the start and note whether it completes without complaint. Then uninstall and note whether anything is left behind in the installation folder. The macOS and Linux shapes of this question are answered under question 9.

2. **Whether a machine can be asked which Blenders it holds, on Windows.** Open Settings ‣ Apps and note whether every installed Blender version appears as a separate entry, including ones installed from the Microsoft Store and Steam, and whether a Blender that was simply extracted to a folder shows up. The macOS and Linux halves are answered under question 7.

3. **What a daily build reports about itself.** Run `--version` and the `bpy.app` dump from question 3 against an alpha or beta build, and record what `version_cycle`, `version_string` and `build_branch` read. No such build was available when the tests above were run, on either machine.

4. **What a daily build's file is called.** Question 1 now records the file names of a finished release, read from Blender's own download server. The equivalent for a daily build is still owed.

---

2026-08-04 — Opus 5 (Claude Code) — Builds, Blender 4.5

2026-08-04 — Opus 5 (Claude Code) — wrote the hand-test results into questions 1, 2, 3, 5, 6, 7, 8 and 9, brought the outside-the-interface and operating-system lines up to date where a test settled them, and pruned the hand-test list at the end

2026-08-05 — Opus 5 (Claude Code) — wrote the Creator's two screenshot tests into questions 1, 3 and 5, brought the outside-the-interface and operating-system lines up to date where they settled them, and pruned the hand-test list at the end

2026-08-05 — Opus 5 (Claude Code) — citation check: narrowed question 1's reading of the download page to the four headings it lists, corrected question 3's description of the API example, cut "general-purpose" from question 4, and corrected question 5's "four specific ways" and its reference to a non-existent "(d)"

2026-08-06 — Opus 5 (Claude Code) — wrote the Linux hand-test results into questions 1 to 9, from a rented Linux machine; brought both standing lines up to date on every question a Linux test settled, and pruned the hand-test list at the end
