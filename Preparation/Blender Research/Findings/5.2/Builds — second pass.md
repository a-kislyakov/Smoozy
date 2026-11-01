# Builds — Blender 5.2 — second pass (questions 1, 3, 9)

**Sources consulted, and when.** Blender's 5.2 manual (plain text plus its `_images` pictures), Blender's 5.2 Python API documentation, Blender's 5.2 release notes, and Cycles' own files as shipped with 5.2 — all four as held in `Blender Sources/`, snapshot taken 2026-07-30, which is the date read for everything cited from them. Also the undated-by-version requirements page in `Blender Sources/requirements/`, filed under 2026-08-01; it is used nowhere as evidence below, because it never names 5.2, and it appears only in the record of where I looked.

**Declaration.** I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project. One qualification, stated plainly rather than glossed: to confirm the folder I was told to save into exists, I listed the directory `Preparation/Blender Research/Findings/5.2/`. That showed me file names only. I opened none of them, and in particular I did not open `Builds.md`, the first pass's findings.

---

## 1. What kinds of build does Blender publish, and how is each kind named and marked?

**Label: documented** (for the kinds, their names, and the release-status marks). **Not in the sources** for one part, called out at the end of the answer: how a build file itself is named on the download server, and whether "LTS" is marked anywhere a program can read.

### The kinds

The manual's installing page sets out four packages under the heading "Download Blender", and says outright that they differ by stability:

> "Blender offers a variety of different binary packages to choose from depending on their level of stability. Each package has the trade off of newest features versus stability."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html, read 2026-07-30.

The four, each quoted from that same page:

**Stable Release**

> "A package that contains the latest features and is considered stable without regressions. A new stable version is available roughly every three months."

**Long-term Support**

> "A package designed for long-lasting projects requiring a very stable version of Blender. LTS releases are supported for two years and will not have any new features, API changes or improvements. A new long-term support version is available every year. These LTS releases will occasionally have minor patches (such as 4.2.6) which improve stability or fix critical bugs."

**Daily Builds**

> "A package updated daily to include the newest changes in development. These versions are automatically built on a schedule. They are not as thoroughly tested as the release types above, and might break or crash. Builds marked as Alpha are still undergoing major changes and feature additions, while those marked Beta are feature-complete and are under development for refinement and stability."

**Build from Source**

> "Blender's source code is available for free to either reference or to build and use. While normal users are not expected to compile Blender, it does have advantages:"

All four quotes: Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html, read 2026-07-30.

### How each kind is marked

**The release-status ladder.** The same page names the marks and their order:

> "Stability can be expected to increase from Alpha to Beta to Release Candidate (RC) to a final release."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html, read 2026-07-30.

The same four names appear in the Python API as a value a build carries about itself:

> "bpy.app.version_cycle
>
> The release status of this build alpha/beta/rc/release"

— Blender Python API, https://docs.blender.org/api/5.2/bpy.app.html, read 2026-07-30.

**The mark is shown on screen.** The manual says the splash screen carries the version:

> "The upper part of the splash screen contains the splash image with the Blender version in the top right."

— Blender manual, https://docs.blender.org/manual/en/5.2/interface/window_system/splash.html, read 2026-07-30.

The picture on that page shows what that looks like when the build is a development one. In `_images/interface_splash_current.webp`, the picture printed on https://docs.blender.org/manual/en/5.2/interface/window_system/splash.html and captioned "Blender Splash Screen.", the text in the top right of the splash image reads **`5.0.0 Alpha`**. Label for that: **documented** — it is part of the manual. What is visible is the three-part version number followed by the word Alpha, in the top right corner. Read 2026-07-30.

**A separate mark for how the binary was compiled.** This is a different axis from alpha/beta/rc/release:

> "bpy.app.build_type
>
> The type of build (Release, Debug)"

— Blender Python API, https://docs.blender.org/api/5.2/bpy.app.html, read 2026-07-30.

**LTS is marked in the version label, in Blender's own writing.** The manual's markup guide documents a version label distinct from the plain version number:

> "|BLENDER_VERSION|:
> Inserts the current Blender version number automatically. e.g. "4.5"."

> "|BLENDER_VERSION_LABEL|:
> Inserts the current Blender version label automatically. e.g. "4.5 LTS"."

— Blender manual, https://docs.blender.org/manual/en/5.2/contribute/manual/guides/markup_guide.html, read 2026-07-30.

That label is what appears across the 5.2 documents themselves. The manual's own front page is titled:

> "Blender 5.2 LTS Reference Manual"

— Blender manual, https://docs.blender.org/manual/en/5.2/index.html, read 2026-07-30.

The release notes say the same and give the support window:

> "Blender 5.2 LTS was released on July 14, 2026."

> "This release includes long-term support and will be maintained until July 2028, see the [LTS page](https://www.blender.org/download/lts/5-2/) for a list of bugfixes included in the latest version."

— Blender release notes, https://developer.blender.org/docs/release_notes/5.2/, read 2026-07-30.

And the manual's list of versions marks which past releases were LTS and which were not, by the same suffix — "Blender 4.5 LTS", "Blender 4.4", "Blender 4.3", "Blender 4.2 LTS", "Blender 3.6 LTS", "Blender 3.3 LTS", "Blender 2.93 LTS", "Blender 2.83 LTS" — at Blender manual, https://docs.blender.org/manual/en/5.2/versions.html, read 2026-07-30.

**One thing only daily builds carry.** The preferences page for experimental features says:

> "These preferences are reserved for features that are currently being worked on and are not yet complete. Experimental features are only available in Daily Builds."

— Blender manual, https://docs.blender.org/manual/en/5.2/editors/preferences/experimental.html, read 2026-07-30.

The picture on that page, `_images/editors_preferences_section_experimental.webp`, captioned "Blender Preferences Experimental section.", shows an Experimental preferences panel with three groups headed New Features, Prototypes and Debugging, each row a checkbox. Most rows carry a linked issue number to the right; the first row's link instead reads "Pipeline, Assets & IO Project Page", and several rows under Debugging carry no link at all. **Documented**, read 2026-07-30. *(Citation check, 2026-08-05: previously said every row carried a linked issue number. Corrected against the picture.)*

**A build may carry a branch name.** The About Blender entry in the Topbar menu is documented to show:

> "Branch: Optional branch name."

— Blender manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html, read 2026-07-30.

### Where builds are published, as the manual lists them

The manual's installation guides name these sources, each on its own page: blender.org (Linux archive; Windows zip-file or Windows Installer File; macOS dmg-file), a Linux distribution's package manager, Snap, the Microsoft Store, and Steam. Two of those pages name the architectures on offer:

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html, read 2026-07-30.

> "Blender 5.0 and later only supports Macs with Apple silicon."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/macos.html, read 2026-07-30.

I have listed these as ways a build is distributed, which is what those pages describe. Whether they count as "kinds of build" is a reading of the question; see the ambiguity note below.

### Ambiguity in the question

"Kinds of build" can be read two ways: the stability tiers (stable, LTS, daily, source), or the packages per platform and shopfront (Linux archive, Windows zip, Windows installer, macOS dmg, Snap, Microsoft Store, Steam). I answered the first reading, because the manual itself frames its four packages as "depending on their level of stability", and gave the second reading its own section rather than dropping it.

### Not covered

- **How a build file is named.** Nothing in the sources gives a download file name, or any naming pattern for one. I searched the manual and release notes for `tar.xz`, `.msi`, `.dmg`, `blender-5`, `blender-4` and `x86_64`: no download-file name appears anywhere. The pages say "the Linux version for your architecture", "the zip-file or Windows Installer File", "disk images (dmg-files)" and nothing more precise.
- **Whether LTS is marked anywhere a program can read.** `bpy.app` has no LTS field. I searched the whole of the 5.2 Python API documentation for `LTS` and for `long-term` / `long term` and got no hits at all. `version_cycle` documents only alpha/beta/rc/release; a final LTS release and a final non-LTS release would both be "release" so far as the documented values go. Nothing says the version *string* carries the LTS suffix.
- **Release candidates as a published package.** The manual names RC only inside the stability ladder sentence. It does not say RCs are published as their own download, or where.
- **How often daily builds appear beyond "daily", which branches they are cut from, or how long they are kept.** Not stated.
- **Any distinction between a "portable" build and an installer build as separate published artefacts** — the manual treats portability as something you set up, not as a separate download.

### Observed — hand test, 2026-08-04

Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The build was Blender 5.2.0, installed at `/Applications/Blender.app`. This settles the second "Not covered" item above — whether LTS is marked anywhere a program can read.

In a terminal, with nothing else set up:

`"/Applications/Blender.app/Contents/MacOS/Blender" --version`

Its first line was:

> `Blender 5.2.0 LTS`

followed by `build date: 2026-07-14`, `build time: 01:31:22`, `build commit date: 2026-07-13`, `build commit time: 15:20`, `build hash: fbe6228777e7`, `build branch: blender-v5.2-release`, `build platform: Darwin`, `build type: Release`, three very long compiler and linker flag lines, and `build system: CMake`.

Then, with no window open:

`"/Applications/Blender.app/Contents/MacOS/Blender" --background --python-expr "import bpy; print('BEGIN'); [print(a, '=', repr(getattr(bpy.app, a))) for a in sorted(dir(bpy.app)) if not a.startswith('_')]; print('TYPE version_cycle', type(bpy.app.version_cycle)); print('TYPE build_type', type(bpy.app.build_type)); print('END')"`

This printed all 71 names on `bpy.app` and their values. Word for word, the lines this answer turns on:

> ```
> build_branch = b'blender-v5.2-release'
> build_type = b'Release'
> module = False
> portable = False
> version = (5, 2, 0)
> version_cycle = 'release'
> version_file = (5, 2, 44)
> version_string = '5.2.0 LTS'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

So `version_cycle` reads `release`, exactly as the documented list of values allows, and the LTS marking is carried by `version_string` instead. Searching the whole printed listing for the letters LTS found them only inside `version_string`. What either field reads on a daily build is still untested — no such build was available.

The application file itself was also read. `file` and `lipo -archs` on `/Applications/Blender.app/Contents/MacOS/Blender` both reported one architecture, `arm64`. `plutil -p` on the bundle's `Contents/Info.plist` printed `"CFBundleShortVersionString" => "5.2.0"`, `"CFBundleVersion" => "5.2.0"`, `"CFBundleIconName" => "blender_liquid_glass"` and `"LSMinimumSystemVersion" => "11.2"` — no LTS marking in the bundle's own information file, and no download file name anywhere in it.

### Observed — hand test, 2026-08-05

The Creator ran this one, on their own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. They opened Blender 5.2.0 from the Applications folder in the ordinary way and sent screenshots of what was on screen. This settles what the splash-screen marking reads on a final release — the manual's picture, quoted above, is of a development build reading `5.0.0 Alpha`.

*The splash screen.* The top right corner of the splash image reads:

> `5.2.0 LTS`

So on this final LTS release the corner carries the three-number version followed by the letters LTS, where the manual's picture of a development build carried the stability word `Alpha` in the same place.

*The version at the right-hand end of the bar along the bottom of the window.* It reads:

> `5.2.0`

with a small globe icon and a vertical divider to its left. No LTS marking there, and no stability word.

### Observed — hand tests, 2026-08-06, on a second machine

Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. It is a container sharing somebody else's hardware, and other work was running on it at the same time, so nothing here says how long anything took. The build was Blender 5.2.0, extracted from Blender's own Linux archive at `/opt/blender/blender-5.2.0-linux-x64`. These are a second machine's results and sit beside the Mac's above; they are never merged with them. **This settles the first "Not covered" item — how a build file is named — which the search record above says nothing in the sources gives.**

*What a published build file is called.* The 5.2 release folder on Blender's own download server was read in a terminal on 2026-08-06. It lists:

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

So a name is the word blender, the three-number version, the operating system, the processor, and the file kind. The terms this answer searched for and did not find — `tar.xz`, `.msi`, `.dmg`, `blender-5` — are all in these names. **No stability word and no LTS marking appears in any file name**, though 5.2 is an LTS release, so the marking the answer above traces through `|BLENDER_VERSION_LABEL|`, the manual's title and the release notes does not reach the file name. There is one macOS file and it is for Apple silicon. Windows gets three file kinds, not two: an `.msix` sits beside the `.msi` and the `.zip`.

A checksum scheme exists too, which nothing in the sources mentions. `blender-5.2.0.sha256` was fetched and holds one line per file; the Linux archive was fetched and checked against it, and `sha256sum -c` printed `blender-5.2.0-linux-x64.tar.xz: OK`.

*What the build reports about itself on Linux, with no window open.* `--version` printed `Blender 5.2.0 LTS` on its first line, then `build date: 2026-07-14`, `build time: 01:32:04`, `build commit date: 2026-07-13`, `build commit time: 15:20`, `build hash: fbe6228777e7`, `build branch: blender-v5.2-release`, `build platform: Linux`, `build type: Release`, three flag lines of which the last is empty on this build, and `build system: CMake`. The `--background --python-expr` dump returned:

> ```
> build_branch = b'blender-v5.2-release'
> build_type = b'Release'
> cachedir = '/root/.cache/blender/'
> module = False
> portable = True
> version = (5, 2, 0)
> version_cycle = 'release'
> version_file = (5, 2, 44)
> version_string = '5.2.0 LTS'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

Seventy-one names in all, and searching the whole printed listing for the letters LTS found them only inside `version_string` — the same as on the Mac. The commit date, commit time, hash and branch are the Mac's too. Two things differ: `build platform` reads `Linux` where the Mac read `Darwin`, and `portable` reads `True` where the Mac read `False`. There is no folder named `portable` in the extracted build; that was checked for all four Linux builds and none has one.

*What the build file itself is.* `file` on it printed `ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=8f25b4c4f0598492bb183b092f48482cf40bc804, for GNU/Linux 3.2.0, stripped`, and `readelf -h` reported one architecture, `Advanced Micro Devices X86-64`. There is no application bundle on Linux, so there is no bundle information file to read; the extracted folder holds `blender`, `blender-launcher`, `blender-thumbnailer`, `blender-system-info.sh`, `blender.desktop`, two icons, `copyright.txt`, `license`, `readme.html`, `lib`, and a folder named `5.2`.

*The splash screen and the status bar, with a window open on the machine's own display.* Blender was started with a window and photographed with its own screenshot command. The top right corner of the splash image reads `5.2.0 LTS`. The right-hand end of the bar along the bottom reads `5.2.0`, with no LTS marking and no stability word. Both are the same as the Creator's Mac.

*A build from a different route.* A Blender 5.2.0 was also installed on this machine through Snap, whose channel listing names it `5.2lts/stable`. That build reports the same version string with the LTS marking, the same `version_cycle`, the same branch and the same hash. So the `lts` in Snap's channel name is another place the marking appears, and it is a place outside Blender.

### Outside the interface. Observed on two machines.

The kinds, their names and the marks are properties of what is published and of what a build reports about itself, not of how Blender is driven. `bpy.app.version_cycle` and `bpy.app.build_type` are described as "application values that remain unchanged during runtime" (Blender Python API, https://docs.blender.org/api/5.2/bpy.app.html, read 2026-07-30), with no condition attached about a window being open. The test above bears that out for this build: run with `--background` and no window at any point, both came back, along with the LTS-marked `version_string`. The one item here that plainly needs the interface is the splash-screen marking, which is a screen element, and it was looked at with a window open on 2026-08-05, as recorded above.

### Operating system. Observed for macOS and for Linux.

The kinds themselves are described without reference to operating system. Two differences are documented: on macOS, "Blender 5.0 and later only supports Macs with Apple silicon", so the architecture choice offered differs by platform; on Windows there are both an installer and a zip package, whereas the Linux and macOS pages describe a single archive and a single disk image respectively. The Microsoft Store route is Windows-only, Snap is Linux-only, Steam is documented for Linux, macOS and Windows alike. Observed on the Creator's Apple Silicon Mac: the installed 5.2.0 macOS build holds one architecture, `arm64`. **Observed on the rented Linux machine, 2026-08-06:** the 5.2.0 Linux build is a single-architecture x86-64 executable, reports `build platform: Linux`, and reads `portable = True` where the Mac read `False`; the splash corner and the status bar read the same as on the Mac. The release folder on the download server bears out both documented differences — one macOS file and it is `macos-arm64`, and three Windows file kinds against Linux's one. Windows itself remains untested, though the file names show what is offered for it.

---

## 3. Can an installed Blender be asked what it is — which version, and which kind of build?

**Label: documented.** Yes — by several documented routes. One part of the question, the LTS status, is **not in the sources**.

### From the command line

> "-v, --version
>
> Print Blender version and exit."

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30.

That is the whole of what the manual says about it. What the printed text actually contains is not documented anywhere I could find; see Not covered.

### From Python, as values the build carries about itself

The Python API documents a block of read-only application values. Quoted exactly:

> "bpy.app.version
>
> The Blender version as a tuple of 3 numbers (major, minor, micro). eg. (4, 3, 1)"

> "bpy.app.version_string
>
> The Blender version formatted as a string"

> "bpy.app.version_cycle
>
> The release status of this build alpha/beta/rc/release"

> "bpy.app.version_file
>
> The Blender File version, as a tuple of 3 numbers (major, minor, file sub-version), that will be used to save a .blend file. The last item in this tuple indicates the file sub-version, which is different from the release micro version (the last item of the bpy.app.version tuple). The file sub-version can be incremented multiple times while a Blender version is under development. This value is, and should be, used for handling compatibility changes between Blender versions"

> "bpy.app.build_type
>
> The type of build (Release, Debug)"

> "bpy.app.build_hash
>
> The commit hash this Blender instance was built with"

> "bpy.app.build_branch
>
> The branch this blender instance was built from"

> "bpy.app.build_date
>
> The date this Blender instance was built"

> "bpy.app.build_time
>
> The time this Blender instance was built"

> "bpy.app.build_platform
>
> The platform this blender instance was built for"

> "bpy.app.build_system
>
> Build system used"

> "bpy.app.build_commit_date
>
> The date of the commit this Blender instance was built from"

> "bpy.app.build_commit_time
>
> The time of the commit this Blender instance was built from"

> "bpy.app.build_commit_timestamp
>
> The unix timestamp of the commit this Blender instance was built from"

> "bpy.app.binary_path
>
> The location of Blender's executable, useful for utilities that open new instances. Read-only unless Blender is built as a Python module - in this case the value is an empty string which script authors may point to a Blender binary."

The module itself is described as: "This module contains application values that remain unchanged during runtime."

All of the above: Blender Python API, https://docs.blender.org/api/5.2/bpy.app.html, read 2026-07-30.

There is also a listing of what the build was compiled with, given as a literal in the documentation:

> "bpy.app.build_options
>
> Constant value bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=False, jack=True, pulseaudio=True, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, potrace=True, pugixml=True, haru=True, experimental_features=False)"

— Blender Python API, https://docs.blender.org/api/5.2/bpy.app.html, read 2026-07-30.

That listing is the value as captured when the documentation was generated. It should not be read as the value any given 5.2 build will report; the documentation presents it as a constant value of the documented build, and different builds are what these fields exist to distinguish. **Label for that reading: inferred** — the step is that `build_options` is documented as describing "this Blender instance", so a value printed in generated documentation describes the instance that generated it, not every instance. A hand test on 2026-08-05 read this build's own value and set it beside the listing above: they do not match. So for the build on that machine the reading is **observed** rather than inferred, and it is recorded further down under *Observed — hand test, 2026-08-05 — what this build was built with*.

The API documentation also shows the version being read in a worked example:

> "print("Blender {:s}".format(bpy.app.version_string))"

— Blender Python API, https://docs.blender.org/api/5.2/bpy.utils.html, read 2026-07-30. The same example goes on to walk the attributes of `bpy.app` and print them: "# Dump `bpy.app`." then "for attr in dir(bpy.app):". It skips underscore-prefixed names and three noisy ones, "if attr in {"handlers", "build_cflags", "build_cxxflags"}:", and then keeps only what is left that is either `build_`-prefixed or a tuple — everything else falls through the example's "else:" branch, commented "# Otherwise ignore.". *(Citation check, 2026-08-05: previously said the example skipped "only" those two sets. It drops a third group as well. Corrected against the page.)*

### From the interface

The Topbar's Blender menu carries an About Blender entry, documented as showing:

> "Version: The Blender version."

> "Date: Date when Blender was compiled."

> "Hash: The Git Hash of the build.
> This can be useful to give to support personnel when diagnosing a problem."

> "Branch: Optional branch name."

> "Windowing Environment: On Linux, this will show either Wayland or X11 depending on the windowing environment that Blender is running on."

— Blender manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html, read 2026-07-30.

And the splash screen shows the version, as quoted in question 1 above; the manual's picture of it shows a development build reading `5.0.0 Alpha`, so the release-status mark is visible there too.

### As a file written out

> "Save System Info
>
> This extracts system information which can be useful for including in bug reports, inspecting the configuration, or diagnosing problems.
>
> You will be prompted to save a text file called system-info.txt."

and its first section:

> "Blender
>
> This section shows you the Blender version, details about the build configuration, and the path in which Blender is running."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/help.html, read 2026-07-30.

The same thing exists as an operator, which is the form something driving Blender would reach:

> "bpy.ops.wm.sysinfo(*, filepath='')
>
> Generate system information, saved into a text file"

— Blender Python API, https://docs.blender.org/api/5.2/bpy.ops.wm.html, read 2026-07-30.

### Not covered

- **What `--version` actually prints.** The manual gives only "Print Blender version and exit." Whether the printed text includes the build hash, the build date, the branch, the release status, or only a version number, is not stated anywhere in the manual, the API documentation, the release notes, or Cycles' files. I searched all four for the phrase and around it and found this one line.
- **Whether a build reports that it is LTS.** No field for it. As noted under question 1, `LTS` and `long-term` return nothing at all across the whole 5.2 Python API documentation. A final LTS release and a final non-LTS release both fall under the documented `version_cycle` value "release", so far as the sources say.
- **The exact form of any of these values.** `version_string` is "The Blender version formatted as a string" with no example of the format; whether it carries the release status, the way the splash image does, is not stated. `build_type`'s documented values are given as "(Release, Debug)" with no statement that those are the only two.
- **Whether the About Blender panel and `--version` agree.** Not addressed.
- **Whether these values can be trusted to identify which of the published kinds a build came from** — for example whether a Steam build, a Microsoft Store build and a blender.org build of the same version report anything different. Nothing in the sources on this.
- The picture of the About Blender panel does not exist in the manual; the page has a picture of the Topbar menus (`_images/interface_window-system_topbar_menus.png`) but I did not open it, as the menu's contents are already given in words on the same page.

### Observed — hand test, 2026-08-04

Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The build was Blender 5.2.0 at `/Applications/Blender.app`. This answers four of the "Not covered" items above: what `--version` prints, whether a build reports that it is LTS, what form `version_string` takes, and whether these values are readable in background mode.

*What `--version` prints.* Quoted in full under question 1 above. Its first line is `Blender 5.2.0 LTS`; the lines under it carry the build date and time, the commit date and time, the commit hash, the branch, the build platform, the build type, three long flag lines, and the build system. There is no alpha/beta/rc word — `build type: Release` is the compilation type, which the API documents separately.

*What the build reports in Python, with no window open.* The `--background --python-expr` command quoted under question 1 returned all 71 `bpy.app` names and values. `version_string` read `'5.2.0 LTS'`, `version_cycle` read `'release'` as a `str`, `build_type` read `b'Release'` as `bytes`, `binary_path` read `'/Applications/Blender.app/Contents/MacOS/Blender'`, `module` read `False` and `background` read `True`. `build_options` came back as a value in the same shape as the one quoted from the documentation above. Whether this build's value differs from that quoted constant is not settled here: the full value was not written down at the time, and the two things once named as differences — `xr_openxr=True` and the absence of a `collada` entry — are not differences at all, because the documented constant carries `xr_openxr=True` and has no `collada` entry either. The full value was read again on its own on 2026-08-05 and set beside the documented constant; that is recorded further down, under *Observed — hand test, 2026-08-05 — what this build was built with*. The point made under the inferred label above stands on its own: the documented constant describes the build that generated the documentation, not every build. *(Citation check, 2026-08-05: this passage claimed a difference from the documented constant on the strength of two items that match it. Checked against `api_5.2/bpy.app.txt` and cut; the comparison is sent to the hand tests rather than answered here.)*

*What `wm.sysinfo` does with no window open.* The sources do not say whether that operator runs in background mode. In a terminal:

`"/Applications/Blender.app/Contents/MacOS/Blender" --background --python-expr "import bpy; bpy.ops.wm.sysinfo(filepath='<a folder outside the project>/system-info.txt')"`

Blender printed `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)` and then `Blender quit`, and the file was written. Its opening and Blender section, word for word:

> ```
> ========================================
> = Blender 5.2.0 LTS System Information =
> ========================================
> 
> 
> Blender:
> ========================================
> 
> version: 5.2.0 LTS, branch: blender-v5.2-release, commit date: 2026-07-13 15:20, hash: fbe6228777e7, type: Release
> build date: 2026-07-14, 01:31:22
> platform: 'macOS-26.5.2-arm64-arm-64bit-Mach-O'
> binary path: '/Applications/Blender.app/Contents/MacOS/Blender'
> ```

Three long build-flag lines follow, then `build system: CMake`, and then a `Python:` section opening `version: 3.13.13 (main, Apr 25 2025, 12:39:20) [Clang 21.0.0 (clang-2100.0.123.102)]`. So the operator does run with no window, and the file it writes carries the LTS marking in its title and on its `version:` line.

### Observed — hand test, 2026-08-05

The Creator ran this one, on their own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. They opened Blender 5.2.0 from the Applications folder in the ordinary way and sent screenshots of what was on screen. This answers the "Not covered" item about whether the About Blender panel and `--version` agree.

*The About Blender panel.* Clicking the Blender icon at the far left of the Topbar and choosing About Blender opens a panel. Its left-hand column reads, line by line:

> ```
> Blender
> Version: 5.2.0 LTS
> 
> Date: 2026-07-13 15:20
> Hash: fbe6228777e7
> Branch: blender-v5.2-release
> 
> Blender is free software
> Licensed under the GNU General Public License
> ```

Its right-hand column is a set of links: `Donate`, `What's New`, then `Credits`, `License`, `Blender Store`, `Blender Website`.

Four of the five rows the manual documents are present — Version, Date, Hash and Branch. The fifth, `Windowing Environment`, which the manual says appears "On Linux", does not appear on this Mac. Against what `--version` prints, the panel is the shorter of the two: it carries no build time, no build platform, no build type, no build system, and no separate build date — its single `Date:` row reads `2026-07-13 15:20`, while `--version` prints `build date: 2026-07-14` and `build commit date: 2026-07-13` with `build commit time: 15:20` as separate lines. Both carry the same hash, the same branch and the same LTS-marked version.

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

So the `Save System Info` entry quoted from the manual above is present on this build, written with a trailing ellipsis.

### Observed — hand test, 2026-08-05 — what this build was built with

Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The build was Blender 5.2.0 at `/Applications/Blender.app`. The work was done in a fresh folder outside the project, and Blender was pointed at a settings folder of its own with the `BLENDER_USER_RESOURCES` environment variable — that folder was made before Blender was started, so Blender was never pointed at a folder that did not exist.

This settles the one thing the paragraph above left open: whether this build's own `build_options` differs from the constant printed in the API documentation.

The command typed, with the scratch folder written out as `<a folder outside the project>`:

`BLENDER_USER_RESOURCES="<a folder outside the project>/15/settings" "/Applications/Blender.app/Contents/MacOS/Blender" --background --python-expr "import bpy; print(bpy.app.build_options)"`

What came back, word for word:

> ```
> bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=True, jack=False, pulseaudio=False, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, potrace=True, pugixml=True, haru=True, experimental_features=False)
> Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
> 
> Blender quit
> ```

The command ended with exit code 0. It was run twice, and printed the same value both times.

*The two set side by side.* The documented constant is the one quoted higher up this answer, from the 5.2 Python API documentation, https://docs.blender.org/api/5.2/bpy.app.html, read 2026-07-30. Both carry the same forty-four names, in the same order, and no name is present in one and absent from the other. **They do not match.** Three of the forty-four names carry a different value:

- `coreaudio` — documented `False`, this build `True`.
- `jack` — documented `True`, this build `False`.
- `pulseaudio` — documented `True`, this build `False`.

The other forty-one names read the same in both.

So the constant printed in the documentation is not what this 5.2 build reports. What the answer above marks **inferred** — that the printed constant describes the build that generated the documentation rather than every build — is **observed** for this build, on this one point of difference.

### Observed — hand tests, 2026-08-06, on the rented Linux machine

Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. The build was Blender 5.2.0 at `/opt/blender/blender-5.2.0-linux-x64`. Every route above was tried again on this second machine, and **the `build_options` comparison comes out the other way round from the Mac's.**

*What this build was built with, set beside the documented constant.* In a terminal:

`/opt/blender/blender-5.2.0-linux-x64/blender --background --python-expr "import bpy; print(bpy.app.build_options)"`

What came back, word for word:

> ```
> bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=False, jack=True, pulseaudio=True, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, potrace=True, pugixml=True, haru=True, experimental_features=False)
> ```

That value was written to a file and set character by character against the constant quoted from the documentation higher up this answer. **They are identical — every character.** Where the Creator's Mac build differs from the documented constant in three names — `coreaudio`, `jack` and `pulseaudio` — this Linux build differs in none, and those three read `False`, `True` and `True` here, which is exactly what the documentation prints.

So the reading this answer marks **inferred** — that a value printed in generated documentation describes the instance that generated it, not every instance — now has both halves observed. On the Mac it is a different value; on this Linux machine it is the same value. The documented constant is a Linux build's own value, and treating it as a description of any 5.2 build is right on one system and wrong on the other.

*From the command line.* `--version` printed the block quoted under question 1 of this document's Linux block, whose first line is `Blender 5.2.0 LTS`. `--help` printed **525 lines**, against 518 on the Mac, and the version entry reads `-v or --version` / `	Print Blender version and exit.` with no platform note, while `/?` beside it prints `Print this help text and exit (Windows only).`

*From Python, with no window open.* Seventy-one names came back. `version_string` read `'5.2.0 LTS'`, `version_cycle` read `'release'` as a `str`, `build_type` read `b'Release'` as `bytes`, `binary_path` read `'/opt/blender/blender-5.2.0-linux-x64/blender'`, `module` read `False`, `background` read `True`, and `portable` read `True` — the last of which is the only one of those that reads differently from the Mac.

*What `wm.sysinfo` does with no window open.* In a terminal:

`/opt/blender/blender-5.2.0-linux-x64/blender --background --python-expr "import bpy; bpy.ops.wm.sysinfo(filepath='<a folder outside the project>/system-info.txt')"`

Blender printed one warning of its own first — `cycles | WARNING HIPEW initialization failed: Error opening HIP dynamic library`, this machine having no AMD graphics support — then `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)` and `Blender quit`. The file was written. Its opening and Blender section, word for word:

> ```
> ========================================
> = Blender 5.2.0 LTS System Information =
> ========================================
> 
> 
> Blender:
> ========================================
> 
> version: 5.2.0 LTS, branch: blender-v5.2-release, commit date: 2026-07-13 15:20, hash: fbe6228777e7, type: Release
> build date: 2026-07-14, 01:32:04
> platform: 'Linux-6.8.0-87-generic-x86_64-with-glibc2.35'
> binary path: '/opt/blender/blender-5.2.0-linux-x64/blender'
> ```

Three build-flag lines follow — the third empty on this build — then `build system: CMake`, then a `Python:` section opening `version: 3.13.13 (main, Apr 25 2025, 12:39:20) [GCC 14.2.1 20250110 (Red Hat 14.2.1-11)]`. So the operator runs with no window here too, and the file carries the LTS marking in its title and on its `version:` line.

*The About Blender panel, with a window open on the machine's own display.* Blender was started with a window, `bpy.ops.wm.splash_about` was called from a timer, and the window was photographed with Blender's own screenshot command. Its left-hand column reads, line by line:

> ```
> Blender
> Version: 5.2.0 LTS
> 
> Date: 2026-07-13 15:20
> Hash: fbe6228777e7
> Branch: blender-v5.2-release
> Windowing Environment: X11
> 
> Blender is free software
> Licensed under the GNU General Public License
> ```

**This settles the one documented row no machine had ever shown.** The manual says `Windowing Environment` appears "On Linux", and here it does, reading `X11`. All five documented rows are present on this machine; four are present on the Mac. Everything else about the panel matches the Mac, including that it carries fewer facts than `--version` prints.

*The Help menu, with a window open.* `bpy.ops.wm.call_menu(name='TOPBAR_MT_help')` was called and the window photographed. The menu drops down `Manual`, `Support`, `User Communities`, `Get Involved`, `Release Notes`, a divider, `Report a Bug`, `Save System Info...` — line for line what the Creator saw on the Mac, trailing ellipsis and all.

*Whether the distribution route is visible in what the build reports.* This is the last "Not covered" item above, and half of it is now answered. A Blender 5.2.0 was also installed on this machine through Snap. It reports the same version string with the LTS marking, the same `version_cycle`, the same branch, the same hash `fbe6228777e7`, the same build date and time, and the same `build_options` as the blender.org build. The only thing separating them is where they sit: `binary_path` reads `/snap/blender/7599/blender` for one and `/opt/blender/blender-5.2.0-linux-x64/blender` for the other. **So a Snap build and a blender.org build of the same version report nothing different about themselves.** Steam and the Microsoft Store are still out of reach.

### Outside the interface. Observed on two machines.

This holds outside the interface, and that is where most of it lives. What the answer above marks **inferred** — that the `bpy.app` values are readable in background mode — is now **observed**: the `--background --python-expr` run returned every one of them, and `bpy.ops.wm.sysinfo()` wrote its file in the same mode. `build_options` was read the same way on 2026-08-05, with no window at any point. `-v, --version` is a command-line argument. Background operation is documented on the same page — "-b, --background / Run in background (often used for UI-less rendering)" — and Python can be handed in without a window: "--python-expr <expression> / Run the given expression as a Python script." and "-c, --command <command> / Run a command which consumes all remaining arguments. Use -c help to list all other commands. Pass --help after the command to see its help text." followed by "This implies --background mode." (Blender manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30).

That the `bpy.app` values are readable in background mode is **inferred**, and here is the step: the API documentation's worked example for `bpy.utils.register_cli_command` — documented as registering "a command, accessible via the (-c / --command) command-line argument" — is a routine that prints `bpy.app.version_string` and then walks and prints every attribute of `bpy.app`; and the manual says `-c / --command` "implies --background mode". A reader has to join those two statements, so the label is inferred, not documented. Both quotes are above.

The one route that plainly needs the interface is the splash screen and the About Blender menu entry, both of which are screen elements; both were looked at with a window open on 2026-08-05, as recorded above. `wm.sysinfo` is an operator and takes a filepath, and the sources do not say whether it runs in background mode — the test above shows that on this build it does.

### Operating system. Inferred for all systems; observed for macOS and for Linux.

The command-line argument list gives no platform restriction on `-v, --version`; the arguments that are platform-limited are marked as such on that page ("Windows & Linux only", "Windows only", "Unix only"), and this one is not. **Inferred**, with the step spelled out: the page marks platform-limited arguments explicitly, and this one carries no mark, so it is being read as unrestricted — the page does not say so in words. **Observed** for one system: on the Creator's Mac, `--version` ran and printed as quoted above, so it is not Windows-only in fact. The build's own `--help`, 518 lines, prints the same entry with no platform note — `-v or --version` / `	Print Blender version and exit.` — while `/?` beside it prints `Print this help text and exit (Windows only).` Windows and Linux remain untested.

One documented platform difference inside the About Blender panel: "Windowing Environment: On Linux, this will show either Wayland or X11 depending on the windowing environment that Blender is running on." **Observed** on the Creator's Mac: that row is absent from the panel, as the screenshot recorded above shows. **Observed** on the rented Linux machine, 2026-08-06: the row is present and reads `X11`. So the documented condition is borne out on both sides. Nothing else in this answer is given a platform condition in the sources. The `build_options` list **is** now read on two systems, and it is the sharpest platform difference in this answer: the Mac build's differs from the documented constant in three names and the Linux build's is identical to it, character for character. What a Windows build of 5.2 prints there is untested.

---

## 9. Can installing or removing a Blender be done again safely on a computer where it was cut off part-way?

**Label: not in the sources.**

Nothing in the 5.2 manual, the 5.2 Python API documentation, the 5.2 release notes, or Cycles' files addresses an installation or a removal that was interrupted part-way, or says whether repeating it is safe, or describes what state a half-finished install leaves behind. The record of the search is below, and it is the substance of this answer.

### What the sources do say, which bears on the question without settling it

These are documented facts about what an installation *is*. They do not answer whether repeating an interrupted one is safe, and I am not stretching them to.

**A blender.org installation is a directory of files, not a registered system component:**

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

— Blender manual, https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html, read 2026-07-30.

**Per platform, install is extract, drag, or run an installer:**

> "Download the Linux version for your architecture and decompress the file to the desired location (e.g. ~/software or /usr/local)."

> "When using this method of installation, it is possible to have multiple versions of Blender installed."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html, read 2026-07-30.

> "When choosing the zip-file, you have to manually extract Blender to the desired folder, where you can double-click the executable to run Blender."

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

> "Note, you do not have to overwrite your existing Blender installation. It's perfectly possible to have multiple versions installed side by side."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html, read 2026-07-30.

> "Blender for macOS is distributed as disk images (dmg-files). To mount the disk image, double-click on the dmg-file. Then drag Blender.app into the Applications folder."

> "Install the new version by overwriting the current Blender.app in the Applications folder. You can rename Blender.app or place it in a different folder to have more than one version at a time."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/macos.html, read 2026-07-30.

**Removal is documented once, and only for the Windows installer:**

> "To remove a previously installed version of Blender, use Windows settings or control panel to uninstall the desired version."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html, read 2026-07-30.

No page in the manual describes removing a Blender on Linux or macOS, or removing a zip/archive installation on Windows.

**One documented way an extraction goes wrong, on Linux:**

> "Archive Extraction
>
> Extracting Blender's archive using 7-zip is not supported. TAR must be used instead. For more details, see issue #104070."

— Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html, read 2026-07-30, under the heading "Known Limitations".

**Settings live apart from the installation, and are per-version.** The directory layout page gives the user directories as `$HOME/.config/blender/5.2/` on Linux (or `$XDG_CONFIG_HOME/blender/5.2/`), `/Users/$USER/Library/Application Support/Blender/5.2/` on macOS, and `%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.2\` on Windows; and the system directories as `./5.2/` beside the executable for a blender.org archive or zip, `./Blender.app/Contents/Resources/5.2/` on macOS, `%ProgramFiles%\Blender Foundation\Blender\5.2\` for the Windows installer, `/usr/share/blender/5.2/` for Linux distribution packages, and `%ProgramFiles%\WindowsApps\BlenderFoundation.Blender<HASH>\Blender\5.2\` for a Microsoft Store install — Blender manual, https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html, read 2026-07-30. The same page documents a portable mode: "To enable this, create a folder named portable at the following locations:" — next to the executable on Windows and Linux, and "Inside the application bundle at Blender.app/Contents/Resources" on macOS — "This folder will then store preferences, startup file, installed extensions and presets."

**Two routes update themselves and are not driven by you at all:** "When an update for Blender is available on Steam, Steam will automatically download and apply the update for you." (Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/steam.html) and "When an update for Blender is available on the Microsoft Store, it will be downloaded and installed automatically." (Blender manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html). Both read 2026-07-30. What either does if cut off part-way is not stated, and would in any case be the shopfront's behaviour, not Blender's.

I want to be plain about the temptation here, because it is the obvious mistake to make with this question. From "a self contained installation" extracted "to any directory" it is easy to slide into "so extracting again over a half-extracted copy is safe". The sources do not say that, and I am not saying it. The first is a statement about what an installation contains; the second is a claim about what happens when you repeat an interrupted file operation, which is a property of the archive tool and the file system, not of Blender, and nothing in these sources addresses it.

### Record of where I looked

Searched across the whole of `manual_5.2` and `release_notes_5.2` (text and markdown only), and separately across `api_5.2` and `cycles_5.2`, for each of these terms, case-insensitively:

`uninstall`, `reinstall`, `re-install`, `interrupt`, `idempotent`, `partial install`, `resume`, `repair`, `corrupt`, `clean install`, `leftover`, `rollback`, `silent install`, `unattended`, `msiexec`, `overwrite`/`overwrit`, `administrator rights`, `self contained`, `self-contained`, `atomic`, `lock file`, `temporary directory`, `checksum`, `sha256`, `verify the download`, `signature`, `code sign`, `notariz`, `Gatekeeper`, `quarantine`, `tar.xz`, `.msi`, `.dmg`, `automat`.

What each produced, and why it was ruled out:

- `uninstall` — one relevant hit, the Windows-installer line quoted above. Every other hit is either about uninstalling an *extension* ("This directory will be kept between upgrades but will be removed if the extension is uninstalled", https://docs.blender.org/manual/en/5.2/advanced/extensions/addons.html) or about uninstalling *graphics drivers*, repeated across the twelve GPU troubleshooting pages. Ruled out: neither is about removing a Blender.
- `reinstall` — two hits, neither on topic: "preinstalled Add-ons" on the add-ons preferences page, and a suggestion in the crash troubleshooting page about reinstalling your operating system.
- `interrupt`, `resume` — every hit is about rendering, fluid or cloth baking, or node auto-offset. Ruled out: about work in progress inside Blender, not about installation.
- `corrupt` — hits are about Dyntopo losing attributes, graphics drivers being corrupted by Windows Update, and losing unsaved work. Ruled out.
- `clean install`, `repair` — `clean install` appears only in the GPU-driver advice ("perform a clean installation with drivers from the manufacturer's website"), twelve times. `repair` has no hits at all.
- `leftover` — two hits: "program leftovers" in the crash page, in the context of reinstalling Windows; and a Rigify bug entry in the 5.2 bugfix list. Neither is about a Blender installation.
- `idempotent`, `partial install`, `rollback`, `silent install`, `unattended`, `msiexec`, `checksum`, `verify the download`, `code sign`, `notariz`, `Gatekeeper`, `quarantine`, `lock file`, `tar.xz`, `.msi`, `.dmg` — **no hits at all**, anywhere in the manual or the release notes.
- `sha256` — two hits, both about extension repositories and remote asset libraries, not about Blender downloads.
- `atomic` — one hit, the Merge by Distance node in the 5.2 geometry nodes release notes.
- `signature` — all hits are about closure node socket signatures.
- `overwrit` — the on-topic hits are the Windows and macOS update lines quoted above; the rest are about application templates, keymaps, render output paths and `.blend1` backups.

Pages opened in full and read, rather than only searched: the installing index, and the Linux, macOS, Windows and Steam installation pages; the deploying-in-production page; the directory layout page; the command-line arguments page; the troubleshooting recovery references reached from the `corrupt` search. The whole `getting_started/installing/` folder is five pages and I read all five; that is the entire installation documentation in the 5.2 manual.

Release notes: searched all 21 subject files for the terms above. The only installation-adjacent hit in the whole of the 5.2 release notes was a macOS viewport performance regression entry in `bugfixes.md`. The index page names no installation change. Ruled out: 5.2's release notes say nothing about installing or removing Blender.

Python API: searched for the terms above; the API documents what Blender exposes to scripts running inside a Blender, and nothing in it addresses getting Blender onto or off a machine. `bpy.app.binary_path` is the closest thing, and it reports where the running executable is, not how it got there.

Cycles' files: searched `cycles_5.2` for install, version and build terms. Cycles is an add-on inside Blender and its files carry nothing about Blender's own installation. For completeness, the one build-adjacent thing in them is the add-on's declaration block in `Blender Sources/cycles_5.2/__init__.py`, lines 7–16, which reads in part `"blender": (2, 80, 0),` and `"support": 'OFFICIAL',`. **Observed**, read 2026-07-30. That `support` value is an add-on support level in an add-on's own declaration; I am deliberately not reading it as saying anything about kinds of Blender build, because nothing connects the two.

Requirements page: `Blender Sources/requirements/requirements_2026-08-01.txt`, read 2026-08-01 as filed. It carries lines about portability and installers — "For Windows there is an installer available if you wish to add an icon on the desktop, register .blend file extensions, etc." and "No installation needed, no Internet connection required." — but it never names 5.2, only 4.5 LTS and older, plus "Blender 5.0 and later". I have therefore not used it as evidence for 5.2 anywhere in this document, and note it here only so the search record is complete. It says nothing about interrupted installs in any case.

### Not covered by the sources

This answer settles that the sources are silent, and nothing more. In particular the sources do not settle: whether re-extracting an archive over a partly-extracted directory produces a working Blender; whether the Windows installer detects and recovers from its own interrupted run; whether an interrupted Windows uninstall leaves a Blender that still appears in the system's installed-programs list; what a half-copied `Blender.app` does on macOS; whether Blender itself would refuse to start from an incomplete installation directory, or what it would report if it did; whether any of the shopfront routes (Steam, Microsoft Store, Snap, distribution packages) recover from interruption. All of those are questions about the operating system, the archive tool or the shopfront rather than about Blender, which is very likely why the sources do not address them — but that reasoning is mine, not theirs, and I flag it as such rather than dressing it up as a finding. The macOS items in that list were tested by hand, below; the rest were not.

### Observed — hand test, 2026-08-04

Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. Everything happened in a working folder outside the project; the installed Blender at `/Applications/Blender.app` was only ever read from.

*A note on the copying tool, because it changed the test.* The first attempt used `cp -R`. A Blender application bundle copied that way would not start at all on this computer — whether the copy had been interrupted or had finished — and `codesign --verify --verbose` on a finished `cp -R` copy printed `invalid Info.plist (plist or signature have been modified)`. So the test was run again with macOS's own copying tool, `ditto`, which does produce a bundle that starts, leaving the interruption as the only thing under test. Both attempts are recorded here in the order they happened.

*What a half-copied `Blender.app` does.* Into a fresh empty folder, `ditto "/Applications/Blender.app" <working folder>/Blender.app` was started and killed after two seconds:

> ```
> ### copy killed at 21:40:19
> ### how much arrived: 393M
> ### is there an executable? -rwxr-xr-x@ 1 artist     admin  183237520 Jul 14 04:29 <working folder>/Blender.app/Contents/MacOS/Blender
> ### running the half-copied Blender with --version:
> ### exit code 137
> ```

So the half-copied bundle did hold an executable at its full size, and running it printed nothing at all — no error text, no message naming a missing file — and gave exit code 137. That answers the question above about whether Blender refuses to start from an incomplete installation directory and what it reports: on this computer it printed nothing.

*Whether the copy can simply be redone.* The same `ditto` command was run again over the same folder, without deleting anything first:

> ```
> ### second copy exit code 0
> ### size now: 907M
> ```

`codesign --verify --verbose` on the finished copy printed `a sealed resource is missing or invalid`, followed by a list of `file added:` lines, all of them compiled-Python cache files under `Contents/Resources/5.2/python/`. Running the finished copy with `--version` gave exit code 0 and printed:

> ```
> Blender 5.2.0 LTS
> 	build date: 2026-07-14
> 	build time: 01:31:22
> 	build commit date: 2026-07-13
> 	build commit time: 15:20
> 	build hash: fbe6228777e7
> 	build branch: blender-v5.2-release
> 	build platform: Darwin
> 	build type: Release
> ```

which is the same as the installed 5.2.0 prints.

*What survives a removal.* Separately, a throwaway copy of 5.2.0 was deleted outright with `rm -rf` while a background render was running from it; the deletion was permitted, and afterwards `ls ~/Library/Application\ Support/Blender/5.2` still printed `config`, `extensions`, `scripts`. So the version's settings folder outlives the application, and a fresh copy put in afterwards would land on it rather than on a clean slate.

### Observed — hand tests, 2026-08-06, on the rented Linux machine

Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Both `tar` and `7z` were already on the machine; nothing was installed for this. **This answers three of the seven things the "Not covered by the sources" note above lists**: whether re-extracting an archive over a partly-extracted directory produces a working Blender, whether a half-extracted archive is detectable, and what Blender reports if it will not start from an incomplete directory.

*The archive.* No Blender archive was left on the machine, so `blender-5.2.0-linux-x64.tar.xz` — this version's own Linux archive — was fetched from Blender's own download server into the working folder, purely so that there was a real archive to extract. It was checked against Blender's own published checksum file, and `sha256sum -c` printed `blender-5.2.0-linux-x64.tar.xz: OK`. That checksum file is itself worth noting here, because this answer's search record lists `checksum`, `sha256` and `verify the download` among the terms that produced nothing on topic in the manual: a checksum **is** published, just not in the documentation.

*Extracting with TAR, which the manual says must be used.* `tar -xf blender-5.2.0-linux-x64.tar.xz -C <empty folder>` finished with exit code 0. Running the result with `--version` gave exit code 0 and printed `Blender 5.2.0 LTS` and the rest.

*Extracting with 7-zip, which the manual's Known Limitations says is not supported.* Two steps are needed, one to unpack the compression and one to unpack the archive inside it. Both printed:

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

The archive holds 79 internal links and 7-zip turned every one into a plain file. So the documented limitation is real, it breaks the result in two separate ways, and **it fails silently** — the tool reports success both times. This answer is careful above about not sliding from "a self contained installation" to "so repeating it is safe", and the same care applies here in the other direction: what the manual calls unsupported is not merely discouraged, and nothing in the extraction tool's own output would warn an automated setup.

*An extraction cut off part-way, and done again over the same folder.* `tar -xf` was started into an empty folder and killed part-way. What had arrived:

> ```
> 186M	half
> files: 769
> ```

and the main program had not been written at all — `ls` on it printed `No such file or directory`. Trying to run that path gave exit code 127, with the shell printing `No such file or directory`; Blender printed nothing, because there was no Blender there to print anything. **So on this interruption the half-extracted state is detectable in the plainest way: the program is missing.** The same `tar -xf` command was then run again over the same folder, with nothing deleted first. It finished with exit code 0, and the folder then held 5592 files and 79 links — the same as a clean extraction. Running the result with `--version` gave exit code 0, and its whole output was compared line by line against the 5.2.0 build already extracted under `/opt/blender`: **identical**.

So on Linux an interrupted extraction can simply be done again over the same folder, provided TAR is the tool. That is the same answer the Mac gave for its own shape of the question — a half-copied `Blender.app` — reached by a different route on a different system.

*What survives a removal.* On the rented machine, after a throwaway copy of this version was deleted outright with `rm -rf` while a background render was running from it, that version's settings folder was still there and still held its contents. The same as on the Mac: a fresh copy put in afterwards lands on it rather than on a clean slate.

### Outside the interface. Observed on two machines.

Installing and removing Blender happens before and after Blender runs; none of the five installation pages, nor the deploying-in-production page, distinguishes a Blender that will be driven by a person from one that will be driven by something else. The deploying page does say deployment "can be automated using standard file management tools" (Blender manual, https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html, read 2026-07-30), which is the only statement in the sources touching unattended installation at all, and it says nothing about interruption. Everything in the test above was done from a terminal, with no Blender window at any point.

### Operating system. Observed for macOS and for Linux.

The silence in the sources is uniform across Linux, macOS and Windows: none of the three installation pages mentions interruption or repetition. Two asymmetries are worth naming, because they mean an answer found by hand would very likely differ by platform: removal is documented only for Windows, and only for the installer route; and Linux carries the documented archive-extraction limitation about 7-zip quoted above, which is a platform-specific way for an extraction to go wrong. On macOS 26.5.2 the test above settles the macOS route: an interrupted `ditto` copy could be repeated over the same folder and produced a Blender that started and reported the right version. **On the rented Linux machine, 2026-08-06, the Linux route is settled the same way:** an interrupted `tar` extraction could be repeated over the same folder and produced a Blender identical to the one already installed. The second asymmetry named above is now more than a prediction — the 7-zip limitation was exercised on the system it is stated for, and it breaks a build in two ways while reporting success. Windows remains untested, and the Windows installer, the Microsoft Store and Steam could not be tried on either computer; Snap and a distribution package were reached on the rented machine and are recorded under question 2 of `Preparation/Blender Research/Findings/5.2/Builds.md`.

---

## Things a person with Blender open could settle in a few minutes

The list that stood here has been worked through. What was settled is written into the answers above, marked observed. What is left open is below.

1. **What a daily build reports.** Run `--version` and the `bpy.app` dump from question 3 against an alpha or beta build, and compare with what a final release printed. No such build was available when the tests above were run. (Gap in questions 1 and 3.)

2. **What a daily build's file is called.** Question 1 now records the exact file names of the 5.2 release, read from Blender's own download server. The equivalent for a daily build is still owed. (Gap in question 1.)

3. **What a Steam or Microsoft Store build reports, against a blender.org build of the same version.** Question 3 now records that a Snap build and a blender.org build of 5.2.0 report nothing different about themselves. Steam and the Microsoft Store are still out of reach. (Gap in question 3.)

4. **Whether a half-finished install can be redone on Windows.** Cancel the installer part-way, then run it again to completion, and do the mirror test with the uninstaller — checking Settings ▸ Apps and the `%ProgramFiles%\Blender Foundation\Blender\5.2\` directory at each step. The macOS and Linux shapes of this question are answered under question 9. (Gap in question 9.)

---

2026-08-04 — Opus 5 (Claude Code) — Builds, Blender 5.2

2026-08-04 — Opus 5 (Claude Code) — wrote the hand-test results into questions 1, 3 and 9, brought the outside-the-interface and operating-system lines up to date where a test settled them, and pruned the hand-test list at the end

2026-08-05 — Opus 5 (Claude Code) — wrote the Creator's splash-screen, About Blender and Help-menu screenshots into questions 1 and 3, brought the outside-the-interface and operating-system lines up to date, and pruned the hand-test list at the end

2026-08-05 — Opus 5 (Claude Code) — citation check: corrected the description of the experimental-preferences picture and of the API example under questions 1 and 3, cut the claim that this build's `build_options` differs from the documented constant, and added that comparison to the hand-test list

2026-08-05 — Opus 5 (Claude Code) — ran that comparison as a hand test and wrote it into question 3: this build's `build_options` does differ from the value printed in the API documentation, in three of forty-four names and in neither of the two places once named; brought the standing lines up to date and pruned the item from the list at the end

2026-08-06 — Opus 5 (Claude Code) — wrote the Linux hand-test results into questions 1, 3 and 9, from a rented Linux machine; settled what a published build file is called, found this Linux build's `build_options` identical to the documented constant where the Mac's differs, confirmed the 7-zip limitation and the repeatable `tar` extraction, brought both standing lines up to date and pruned the list at the end
