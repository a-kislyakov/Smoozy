# Builds — Blender 4.5 — second pass (questions 1, 2, 4)

**Sources consulted, and when.** Blender's 4.5 manual (`Blender Sources/manual_4.5`, including its `_images` folder), Blender's 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender's 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`) — all taken on **2026-07-30**, which is the date read for everything cited from them. I also searched Blender's requirements page (`Blender Sources/requirements/requirements_2026-08-01.txt`), taken **2026-08-01**; that page carries no version in its address, so I only report what it names in so many words.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

I did no work through helpers, so this declaration covers everything done for this pass.

Note on the snapshot, because it limits several answers below: the manual has been captured as plain text with **link targets stripped**. Where the manual says "Check the Downloading Blender page", the words are there but the address behind them is not, and the page itself is not part of the manual snapshot. So I can report that the manual points somewhere, but not where.

---

## 1. What kinds of build does Blender publish, and how is each kind named and marked?

**LABEL: documented.**

### The kinds

The manual's installing page names four packages, and frames them as a choice about stability:

> "Blender offers a variety of different binary packages to choose from depending on their level of stability. Each package has the trade off of newest features versus stability."

The four, each with the manual's own heading and its own words:

- **Stable Release** — "A package that contains the latest features and is considered stable without regressions. A new stable version is available roughly every three months."
- **Long-term Support** — "A package designed for long-lasting projects requiring a very stable version of Blender. LTS releases are supported for two years and will not have any new features, API changes or improvements. A new long-term support version is available every year. These LTS releases will occasionally have minor patches (such as 4.2.6) which improve stability or fix critical bugs."
- **Daily Builds** — "A package updated daily to include the newest changes in development. These versions are automatically built on a schedule. They are not as thoroughly tested as the release types above, and might break or crash. Builds marked as Alpha are still undergoing major changes and feature additions, while those marked Beta are feature-complete and are under development for refinement and stability."
- **Build from Source** — "Blender's source code is available for free to either reference or to build and use."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/index.html — read 2026-07-30.

### How each kind is marked

**By a stability word carried on the build.** The same page states the ordering:

> "Stability can be expected to increase from Alpha to Beta to Release Candidate (RC) to a final release."

So the marks in use are Alpha, Beta, Release Candidate (RC), and final release. Same page and address, read 2026-07-30.

**By "LTS" in the name of the version.** This is used consistently as part of the version's name, not as a separate label:

- The manual's own title: "Blender 4.5 LTS Reference Manual" (https://docs.blender.org/manual/en/4.5/index.html, read 2026-07-30).
- The manual's version list marks some entries and not others: "Blender 4.5 LTS", "Blender 4.4", "Blender 4.3", "Blender 4.2 LTS", "Blender 4.1", "Blender 4.0" (https://docs.blender.org/manual/en/4.5/versions.html, read 2026-07-30).
- The release notes: "Blender 4.5 LTS was released on July 15, 2025." and "This release includes long-term support and will be maintained until July 2027" (https://developer.blender.org/docs/release_notes/4.5/, read 2026-07-30).
- A three-number form with LTS is also used: "There are known stability issues in Blender 4.5.0 LTS." (https://developer.blender.org/docs/release_notes/4.5/cycles/, read 2026-07-30).
- The manual's own writing rules confirm "LTS" is part of the version label the manual inserts automatically: "Inserts the current Blender version label automatically. e.g. '4.5 LTS'." (https://docs.blender.org/manual/en/4.5/contribute/manual/guides/markup_guide.html, read 2026-07-30).

Historically the same suffix is used for the LTS entries: "2.83 LTS – June 2020", "2.93 LTS – June 2021", "3.3 LTS – September 2022", "3.6 LTS – June 2023", with "First LTS release intended to support studio and long lifecycle project use." (https://docs.blender.org/manual/en/4.5/getting_started/about/history.html, read 2026-07-30).

**By fields inside the running build.** The Python API documents fields that carry the marking:

> "bpy.app.version_cycle
>
> The release status of this build alpha/beta/rc/release"

> "bpy.app.build_type
>
> The type of build (Release, Debug)"

> "bpy.app.version
>
> The Blender version as a tuple of 3 numbers (major, minor, micro). eg. (4, 3, 1)"

Alongside these the same page documents `bpy.app.version_string`, `bpy.app.version_file`, `bpy.app.build_branch` ("The branch this blender instance was built from"), `bpy.app.build_hash` ("The commit hash this blender instance was built with"), `bpy.app.build_platform` ("The platform this blender instance was built for"), `bpy.app.build_date`, `bpy.app.build_time`, `bpy.app.build_system`, `bpy.app.build_commit_date`, `bpy.app.build_commit_time`, `bpy.app.build_commit_timestamp`, `bpy.app.build_cflags`, `bpy.app.build_cxxflags`, `bpy.app.build_linkflags`. Source: Blender Python API, https://docs.blender.org/api/4.5/bpy.app.html — read 2026-07-30.

Two things to be careful about here. `version_cycle`'s documented values are alpha, beta, rc and release — **LTS is not among them**. And `build_type`'s two documented values are Release and Debug, which is a different distinction from the four packages above.

**By what is shown on screen.** "The upper part of the splash screen contains the splash image with the Blender version in the top right." (https://docs.blender.org/manual/en/4.5/interface/window_system/splash.html, read 2026-07-30). The picture on that page, `_images/interface_window-system_splash_current.png`, shows **"3.6.1"** in the top right and nothing else there — no LTS word, no stability word, even though 3.6 is one of the LTS versions listed above. **documented** (it is a manual picture); what is visible is the bare three-number version in that one screenshot.

The status bar can also show the version — "Blender Version / Shows the version number of Blender that is currently running." (https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html, read 2026-07-30) — and its picture `_images/interface_window-system_status-bar_ui-right.png` shows the form **"v2.81.16"**.

### Two further terms the manual uses

- "experimental builds": "Check if the latest Blender version (official or experimental builds) supports your graphics card." (https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30). The installing page does not use this term, and no page in the snapshot defines it.
- The daily builds are where in-progress features live: "Most experimental features are only available in Daily Builds." (https://docs.blender.org/manual/en/4.5/editors/preferences/experimental.html, read 2026-07-30).

### Package shapes, which cut across the kinds

- Windows: "Download the zip-file or Windows Installer File." and "Blender supports both x64 and arm64 architectures on Windows." (https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html, read 2026-07-30).
- macOS: "Blender for macOS is distributed as disk images (dmg-files)." and "Blender supports both Intel and Apple Silicon architectures on macOS." (https://docs.blender.org/manual/en/4.5/getting_started/installing/macos.html, read 2026-07-30).
- Linux: "Download the Linux version for your architecture and decompress the file to the desired location". A limitation is stated: "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead." (https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html, read 2026-07-30).
- The release notes record a change to what is published: "**macOS Intel support is deprecated.** Due to the high maintenance cost of tracking down and fixing graphics related issues specific to Intel and AMD GPU Macs, it was decided that this will be the last release with an official Intel macOS build of Blender." (https://developer.blender.org/docs/release_notes/4.5/, read 2026-07-30).

### Not covered

- The file names the published builds actually carry, and whether the stability word or LTS appears in a file name. Nothing in the sources shows a build's file name.
- Whether a running LTS build can be told apart from a running non-LTS build from inside Blender. `version_cycle` would read "release" for both, on the documented list of values, but the sources never say that outright.
- What "official" and "experimental builds" mean as published categories, beyond the one sentence that uses the phrase.
- What `bpy.app.build_branch` contains for each published kind.
- Whether Steam and the Microsoft Store carry every kind or only some.

### Observed — hand test, 2026-08-04

Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The build was Blender 4.5.12, installed at `/Applications/Blender 4.5.12.app`. This settles the second "Not covered" item above — whether a running LTS build can be told apart from a running non-LTS build from inside Blender.

In a terminal, with nothing else set up:

`"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" --version`

Its first line was:

> `Blender 4.5.12 LTS`

followed by `build date: 2026-07-21`, `build time: 01:41:35`, `build commit date: 2026-07-20`, `build commit time: 13:37`, `build hash: 84afd5f785f7`, `build branch: blender-v4.5-release`, `build platform: Darwin`, `build type: Release`, three very long compiler and linker flag lines, and `build system: CMake`.

Then, with no window open:

`"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" --background --python-expr "import bpy; print('BEGIN'); [print(a, '=', repr(getattr(bpy.app, a))) for a in sorted(dir(bpy.app)) if not a.startswith('_')]; print('TYPE version_cycle', type(bpy.app.version_cycle)); print('TYPE build_type', type(bpy.app.build_type)); print('END')"`

Word for word, the lines this answer turns on:

> ```
> build_branch = b'blender-v4.5-release'
> build_type = b'Release'
> version = (4, 5, 12)
> version_cycle = 'release'
> version_string = '4.5.12 LTS'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

So on this build the letters LTS are carried by `version_string`, and by the first line of `--version`; `version_cycle` reads `release`, one of the four documented values, and carries no LTS marking. `build_branch` reads `blender-v4.5-release` for this published release. What either field reads on a daily build is still untested — no such build was available.

The application file itself was also read: `file` and `lipo -archs` on `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender` both reported one architecture, `arm64`, and `plutil -p` on the bundle's `Contents/Info.plist` printed `"CFBundleShortVersionString" => "4.5.12"` and `"CFBundleVersion" => "4.5.12 2026-07-21"` — no LTS marking in either.

### Observed — hand test, 2026-08-05

The Creator ran this one, on their own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. They opened Blender 4.5.12 from the Applications folder in the ordinary way and sent screenshots of what was on screen. This settles what the splash screen and the status bar show on a finished LTS release — the manual's own picture shows only the older build reading "3.6.1" quoted above, and its status-bar picture only "v2.81.16".

*The splash screen.* The top right corner of the splash image reads:

> `4.5.12 LTS`

So on this build the splash carries the three-number version followed by the LTS marking, where the manual's picture of an older version showed a bare number.

*The status bar.* The right-hand end of the bar along the bottom of the window reads:

> `4.5.12`

with a small globe icon and a vertical divider to its left. No LTS marking there, and no stability word.

*The About Blender panel.* The Creator also opened the Blender icon menu at the far left of the Topbar and chose About Blender. Its left-hand column reads, line by line:

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

So the LTS marking appears there too, on the Version line.

### Observed — hand tests, 2026-08-06, on a second machine

Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. The build was Blender 4.5.12, extracted from Blender's own Linux archive at `/opt/blender/blender-4.5.12-linux-x64`. These are a second machine's results and sit beside the Mac's above; they are never merged with them. **This settles the first "Not covered" item above — the file names published builds actually carry.**

*What a published build file is called.* The release folder on Blender's own download server was read on 2026-08-06, for the 5.2 release, because that is the archive the extraction tests needed. It lists:

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

A name is the word blender, the three-number version, the operating system, the processor, and the file kind. **Neither the stability word nor the letters LTS appears in any file name**, though 5.2 is an LTS release — so the marking that rides on `version_string` and on the splash corner does not ride on the file name. A checksum file is published beside the archives, which this answer's "Not covered" note records the sources as silent about; the fetched archive matched it. No equivalent listing was read for the 4.5 release, so this is observed for one release only.

*What this build reports about itself, on Linux.* `--version` printed `Blender 4.5.12 LTS` on its first line, then the same commit date, commit time, hash and branch as the Mac build, with `build platform: Linux` and a different build time, `01:31:22`. With no window, the `bpy.app` dump returned:

> ```
> build_branch = b'blender-v4.5-release'
> build_type = b'Release'
> module = False
> portable = True
> version = (4, 5, 12)
> version_cycle = 'release'
> version_string = '4.5.12 LTS'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

The LTS marking rides on `version_string` here exactly as on the Mac, and `version_cycle` reads `release`. One value differs: `portable` reads `True` on Linux and `False` on the Mac, and there is no folder named `portable` in the extracted build — that was checked for all four Linux builds and none has one.

*What the build was compiled with.* The Linux 4.5 build's `build_options` holds forty-five names, the same count as the Mac build, and `collada=True` is in both. Four names read the other way round from the Mac: `coreaudio` is `False` here and `True` there; `jack` is `True` here and `False` there; `pulseaudio` is `True` here and `False` there; and `xr_openxr` is `True` here and `False` there.

*The splash screen and the status bar, with a window open on the machine's own display.* Blender was started with a window and photographed with its own screenshot command. The top right of the splash image reads `4.5.12 LTS`. The right-hand end of the bar along the bottom of the window reads `4.5.12`, with no LTS marking and no stability word. Both are the same as the Creator's Mac.

*The About Blender panel.* Its left-hand column reads:

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

The LTS marking is on the Version line, as on the Mac. There is one more row here than on the Mac: `Windowing Environment: X11`.

### Outside the interface. Observed on two machines.

The four kinds and their marks are a property of what is published, so they do not depend on how Blender is driven. The one part of this answer that touches a running Blender is the `bpy.app` fields, and the sources do not say whether those read the same when Blender is started and driven by something other than a person; they are documented as plain attributes of the Python module, with no condition attached. The test above answers that for this build: run with `--background` and no window at any point, every one of those fields returned a value, and `version_string` carried the LTS marking. The splash screen and status bar are visible only when there is a window to show them in — the sources do not say so, but they describe both as parts of the interface — and both were looked at on 2026-08-05 as recorded above, with a window open.

### Operating system. Observed for macOS and for Linux.

The kinds themselves are stated without reference to operating system. What differs by operating system is the package shape and the extra channels: zip, an installer and the Microsoft Store on Windows; dmg on macOS; an archive, distribution packages and Snap on Linux; Steam on all three (see question 2). The architectures offered differ too, as quoted above, and 4.5 is stated to be the last release with an official Intel macOS build. Observed on the Creator's Mac: the installed 4.5.12 macOS build holds one architecture, `arm64`. **Observed on the rented Linux machine, 2026-08-06:** the 4.5.12 Linux build is a single-architecture x86-64 executable, reports `build platform: Linux`, and reads `portable = True` where the Mac read `False`; four of its forty-five build options read the opposite way from the Mac build's; and its About Blender panel carries the `Windowing Environment` row the Mac's does not. The published file names, read for the 5.2 release, name the operating system and the processor, and the 5.2 release has one macOS file and it is for Apple silicon. Nothing was run on Windows.

---

## 2. Can builds of each kind be listed and fetched by something other than a person at a web page?

**LABEL: mixed — one fetch route is documented; listing is not in the sources.**

### What is documented

**Fetched by a command, on Linux, through Snap:**

> "Snap is a universal package manager designed to work across a range of distributions. Assuming snap is already installed, Blender can be installed through snap with:
>
> snap install blender --classic"

and

> "Installing from this method has a benefit that updates to Blender are automatically installed. Blender from Snap should have a more consistent distribution than individual package managers."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html — read 2026-07-30.

**Fetched through a distribution's package manager, on Linux:**

> "Some Linux distributions may have a specific package for Blender in their repositories."

> "Installing Blender via the distribution's native mechanisms ensures consistency with other packages on the system and may provide other features (given by the package manager), such as listing of packages, update notifications and automatic menu configuration. Be aware, though, that the package may be outdated compared to the latest official release, or not include some features of Blender. For example, some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."

Same page and address, read 2026-07-30. Note the "listing of packages" in that sentence is a feature of the package manager, not of Blender, and the passage does not say which of the four kinds a distribution package would be.

**Fetched through Steam and through the Microsoft Store** — both documented, but both described as a person clicking in a client:

> "open the client and login to your Steam account, or create an account if you don't already have one. After logging in, navigate to the Store tab, search for "Blender", and press the green Install button." (https://docs.blender.org/manual/en/4.5/getting_started/installing/steam.html, read 2026-07-30)

> "Blender can be installed from the Microsoft Store by searching for Blender in the Microsoft Store and installing it." (https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html, read 2026-07-30)

**A documented statement that Blender itself does no fetching:**

> "Blender is designed to not require an internet connection, so it doesn't have a built-in update system. This means you will need to update Blender yourself by following the platform-specific upgrade steps described in the sections below."

Source: https://docs.blender.org/manual/en/4.5/getting_started/installing/index.html — read 2026-07-30.

**Automating what happens after the fetch is documented; the fetch itself is not:**

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

Source: https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30. Read carefully, this says an already-obtained download can be placed automatically. It does not say the download can be obtained automatically.

### What is not in the sources

Nothing in the manual, the Python API documentation, the release notes or Cycles' files describes:

- any way to **list** the available builds of any kind — no index, no address of a download server, no machine-readable listing, no feed;
- any address at all for downloading Blender. The three installing pages each open with "Check the Downloading Blender page to find the minimum requirements and the different versions that are available for Blender", but that page is not part of the manual snapshot, and in this plain-text capture the link behind those words is stripped, so I cannot report where it goes;
- any way to fetch a **specific kind** — daily build, alpha, beta, release candidate, or a named LTS — other than by a person going to that download page;
- what Snap, Steam, the Microsoft Store or a distribution package deliver in terms of the four kinds, or whether they offer a choice of kind.

### Where I looked, and what I ruled out

Searched across `manual_4.5` (excluding the image folder for text searches), `api_4.5`, `release_notes_4.5` and `cycles_4.5`:

- Fetch and transport words: `download`, `download.blender.org`, `builder.blender.org`, `blender.org`, `http`, `curl`, `wget`, `rsync`, `torrent`, `FTP`, `CDN`, `mirror`, `checksum`, `sha256`, `SHA-256`, `signature`.
- Listing words: `index.json`, `manifest.json`, `REST`, `web API`, `API endpoint`, `RSS`, `feed`, `channel`, `list`.
- Channel and packaging words: `snap`, `Snap`, `apt-get`, `dnf`, `pacman`, `yum`, `flatpak`, `winget`, `chocolatey`, `Microsoft Store`, `msi`, `Steam`, `silent install`, `unattended`, `command line install`, `automated deployment`, `automat`.
- Kind words, in case a fetch route was described under one of them: `LTS`, `Long-term`, `daily build`, `nightly`, `experimental build`, `release candidate`, `alpha`, `beta`, `stable release`, `build from source`.

Pages opened in full and ruled out: the four installing pages (index, Linux, macOS, Windows) and the Steam page — they document the routes quoted above and no listing; `advanced/deploying_blender.txt` — automation of placement, not of fetching; `troubleshooting/network.txt` — concerns a 403 reaching `extensions.blender.org`, nothing about builds; `troubleshooting/report_bug.txt` — sends a person to a web page, nothing about fetching builds; `getting_started/about/index.txt` — says only "You can download the latest version of Blender here.", with the link target stripped.

The one machine-readable listing I did find in the manual is **not about Blender builds** and I am ruling it out explicitly: the extensions pages describe hosting a repository with an `index.json` and show a call to `curl -i https://extensions.blender.org/api/v1/extensions/` (https://docs.blender.org/manual/en/4.5/advanced/extensions/creating_repository/static_repository.html and https://docs.blender.org/manual/en/4.5/advanced/extensions/creating_repository/dynamic_repository.html, read 2026-07-30). That is a catalogue of add-ons and extensions, not of Blender builds.

Cycles' own files (`Blender Sources/cycles_4.5`) contain nothing on this at all — searching them for `download`, `LTS`, `version_cycle`, `build_type`, `daily build`, `release status` and `long-term` returned no matches.

The requirements page snapshot (`requirements/requirements_2026-08-01.txt`, read 2026-08-01) describes hardware and names supported versions; it says "For Windows there is an installer available if you wish to add an icon on the desktop, register .blend file extensions, etc." and nothing about listing or fetching.

### Not covered

Whether a download address exists that a machine could read — I can only report that the sources I was given do not carry one, not that none exists. Also not covered: whether Snap, Steam or the Microsoft Store expose a choice between the four kinds, and whether their own command-line tools can list Blender versions; the sources describe none of that.

### Observed — hand test, 2026-08-04

Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. The build was Blender 4.5.12 at `/Applications/Blender 4.5.12.app`.

The manual documents a `-c / --command` argument and says `-c help` lists the commands a build offers. In a terminal:

`"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -c help`

The listing it printed, word for word:

> ```
> Blender Command Listing:
> 	extension
> ```

Nothing else. (The run also printed messages from add-ons the Creator has installed, and a Python error one of them raised while shutting down; none of that is part of the listing.) The whole `--help` output, 434 lines, was also read, and no argument in it fetches, lists or installs a Blender build. So this build offers nothing of its own for listing or fetching builds — which matches the manual's statement that Blender has no built-in update system.

### Observed — hand tests, 2026-08-06, on the rented Linux machine

Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. This is the machine on which the Snap route quoted above could finally be exercised. **This settles the "not in the sources" half of this answer for one route: a listing of published Blender builds that a machine can read does exist, and Snap serves it.**

*What `snap info blender` lists.* In a terminal, `snap info blender`. Its publisher line reads `publisher: Blender Foundation (blenderfoundation**)`. Under `channels:` it printed about a hundred and fifty lines, one per channel. The opening, word for word:

> ```
> channels:
>   latest/stable:     5.2.0   2026-07-14 (7599) 426MB classic
>   latest/candidate:  5.1.2   2026-06-04 (7528) 436MB classic
>   latest/beta:       ^                               
>   latest/edge:       5.3.0   2026-08-05 (7676) 432MB classic
>   5.2lts/stable:     5.2.0   2026-07-14 (7599) 426MB classic
>   5.2lts/candidate:  5.2.0   2026-08-05 (7675) 426MB classic
> ```

and the lines for this version:

> ```
>   4.5lts/stable:     4.5.12  2026-07-21 (7626) 421MB classic
>   4.5lts/candidate:  4.5.12  2026-08-05 (7674) 421MB classic
>   4.5lts/beta:       4.5.0   2025-07-08 (6385) 421MB classic
>   4.5lts/edge:       ^                               
> ```

Every release line back to 2.79 has a channel. A channel is named for the release line, with the letters `lts` in the name where the line is a long-term-support one — `5.2lts`, `4.5lts`, `4.2lts`, `3.6lts`, `3.3lts`, `2.93lts`, `2.83lts`. Each has four forms: stable, candidate, beta and edge. Each line carries a version number, a date, a revision number and a size. So a particular kind **can** be asked for by name, which this answer's "Not covered" note records as unsettled.

*What `snap install blender --classic` delivers.* Run with no channel named, it printed:

> `blender 5.2.0 from Blender Foundation (blenderfoundation**) installed`

with exit code 0, and `snap list blender` then printed:

> ```
> Name     Version  Rev   Tracking       Publisher            Notes
> blender  5.2.0    7599  latest/stable  blenderfoundation**  classic
> ```

So with no channel named the command gives whatever `latest/stable` holds — 5.2.0 on this day, not this version. What that build reports about itself is recorded under question 3 of the 5.2 findings; in short, it is indistinguishable from the blender.org build of the same version.

*Whether the Snap route really updates itself.* `snap refresh --time` printed:

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

The manual's warning that a distribution package "may be outdated" is borne out — 3.0.1 against 5.2.0. `apt-cache show blender` names its file as `blender_3.0.1+dfsg-7_amd64.deb` and carries a `SHA256` line. Its dependency list names about forty system libraries including `libpython3.10`, so it is a different kind of build from a blender.org one. It was downloaded and read but **not installed**: it would put a program at `/usr/bin/blender`, a path this machine already uses for another Blender, and other work was running on the machine.

*What Blender's own download server publishes.* Beside the release archives sit `blender-5.2.0.md5` and `blender-5.2.0.sha256`. The second was fetched and the archive checked against it: `sha256sum -c` printed `blender-5.2.0-linux-x64.tar.xz: OK`. So a checksum is published, which this answer's search record notes as absent from the manual.

### Outside the interface. Observed on two machines.

`snap install blender --classic` is a command already, so it needs no Blender window and no person at a web page — and that is now observed rather than read: on the rented Linux machine `snap info blender`, `snap install blender --classic` and every `apt` command above were run from a terminal with no window at any point. Steam and the Microsoft Store are documented only as a person clicking. The statement that Blender "doesn't have a built-in update system" is about Blender as a whole, not about the interface, and the `-c help` run above — made from a terminal with no window — shows the only command this build offers is about extensions. Beyond that the sources say nothing about driving any of this from outside a window.

### Operating system

This answer is heavily split by operating system, as documented above. Snap and distribution packages: Linux only. Microsoft Store, installer and zip: Windows only. Disk image: macOS only. Steam: "Blender can be downloaded and updated using the Steam client by following the steps described below on Linux, macOS, or Windows." No route in the sources is both machine-driven and available on all three. **Observed on the rented Linux machine, 2026-08-06:** on Linux both the Snap route and a distribution package are real and reachable from a terminal, Snap carries a machine-readable listing of every published release line, and Snap refreshes itself on a timer. **Observed on the Creator's Mac:** the build's own commands and arguments fetch nothing, and the manual gives macOS no route but a person downloading. Windows, Steam and the Microsoft Store remain untested on both.

---

## 4. Does Blender name one release as the one to use for ordinary work?

**The question is ambiguous, and the two readings give different answers, so I answer both.** I take "ordinary work" to mean ordinary use by anyone, which is the reading I answer first. The other reading is "ordinary production work in a studio or on a project", which the sources treat separately and answer differently.

### Reading A — ordinary use by anyone: no, and the manual says so in as many words

**LABEL: documented.**

The manual explicitly declines to pick one and hands the choice to the reader:

> "Blender offers a variety of different binary packages to choose from depending on their level of stability. Each package has the trade off of newest features versus stability. The package that is right for you depends on your requirements for those two. A studio, for example, might want to have long-term support, while a hobbyist may want newer features, while others may just want to test upcoming features. Each package described below has something just right for everyone."

And where the manual has to refer to a chosen package, it names two without preferring either:

> "The procedure for installing a binary, either the latest stable release or a daily build, is the same."

Source: Blender manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/index.html — read 2026-07-30.

The nearest thing to a plain endorsement is the wording of the Stable Release entry itself — "A package that contains the latest features and is considered stable without regressions" — but that is a description of the package, not an instruction to use it. Reading it as a recommendation would be my step, not the manual's.

### Reading B — production work on a project: yes, LTS

**LABEL: documented.**

> "New Blender versions may add, remove or change functionality that affects the results of production files. For a given project, it is advisable to use a single LTS version of Blender. LTS versions receive bug fixes for two years."

Source: Blender manual, page "Deploying Blender in Production", https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30. That page opens: "This page contains tips for setting up Blender in environments such as animation studios and schools."

This is narrower than "the one to use". It names a **kind** (LTS), not a particular release, and it is advice per project — one LTS version for a given project — rather than advice about which release everyone should be on.

Two supporting statements from the same direction, neither of which names a release either:

- The installing page's own framing of who LTS is for: "A studio, for example, might want to have long-term support" and "A package designed for long-lasting projects requiring a very stable version of Blender." (address as above, read 2026-07-30).
- Where an LTS version has a special standing: "In such cases, the last LTS release of the previous release cycle will be kept compatible with the newer file format version, and will be usable as converter between both versions." (https://docs.blender.org/manual/en/4.5/files/blend/compatibility.html, read 2026-07-30). This is about converting files across a major version change, not about which release to work in.

### Where I looked, and what I ruled out

Searched `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5` for wording that would carry a recommendation: `recommend`, `recommended`, `advisable`, `should use`, `best choice`, `for production`, `production ready`, `production-ready`, `suited for`, `which version`, `version to use`, `right for you`, `latest stable`, `official release`, `default version`, `hobbyist`, `studio`, `supported for two years`, `bug fixes for two years`, `critical bugs`, `regression`, `release cycle`, `LTS`, `Long-term`.

What that turned up and what I ruled out:

- Every other `recommend` hit in the manual is about something else entirely — display resolution, mouse and keyboard, graphics drivers, loading factory settings for debugging (`getting_started/configuration/hardware.txt`, `getting_started/configuration/defaults.txt`, `troubleshooting/gpu/*`, `troubleshooting/crash.txt`, `troubleshooting/report_bug.txt`). None concerns which release to run.
- `getting_started/configuration/hardware.txt` contains "Not recommended for general use yet." and "It is not yet ready for production usage and should only be used for testing purposes." — both sit under the heading "Monado", which the same page describes as "a free and open source XR platform for Linux". Neither is about which release to run. Ruled out. *(Citation check, 2026-08-05: previously described both as being about a graphics backend. They are about Monado, an XR platform. Corrected against the page; the ruling-out is unchanged.)*
- The release notes: the only recommendation-shaped sentence found is "OpenGL is still the default in Blender 4.5 LTS, users are encouraged to try Vulkan by" (https://developer.blender.org/docs/release_notes/4.5/eevee/, read 2026-07-30) — again a backend, not a release. Ruled out.
- The manual's version list (https://docs.blender.org/manual/en/4.5/versions.html) marks which versions are LTS but attaches no advice to any of them. Ruled out.
- The release notes front page (https://developer.blender.org/docs/release_notes/4.5/) states the release date and the support window — "This release includes long-term support and will be maintained until July 2027" — but does not tell anyone to use it. Ruled out as a recommendation.
- The Python API documentation carries nothing of this sort; searching it for the terms above produced only unrelated matches.
- Cycles' own files contain nothing on this; searching them for `LTS`, `long-term`, `release status` and `daily build` returned no matches.
- The requirements page snapshot (read 2026-08-01) names supported versions per platform ("Blender 4.5 LTS supports NVIDIA GeForce 400 and newer...") and states "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur)." — these bound what can run where, and are not advice about which release to work in.
- The blender.org download page, which is where such a recommendation would most plausibly sit, **is not part of the sources** and its address is stripped from the manual's plain text, so I cannot report what it presents as the default choice.

### Not covered

- Whether blender.org's download page presents one release as the default. Not in the sources.
- Whether "a single LTS version" advice extends outside studios and schools — the page it sits on is scoped to those environments.
- Which particular LTS release to use, on the production reading. The advice names the kind and says use one per project; no release is named.
- Whether Blender names a release for render farms, batch work, or any setting other than the two above. Nothing found under any of the searched terms.

### Observed — hand test, 2026-08-06, on the rented Linux machine

Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. This does not answer either reading of the question, and is recorded only because it is the nearest thing to a default any machine could be made to show. `snap install blender --classic` — the one command-line fetch route the manual documents — was run with no channel named. It printed:

> `blender 5.2.0 from Blender Foundation (blenderfoundation**) installed`

and `snap list blender` showed it tracking `latest/stable`. `snap info blender` shows that older lines, including `4.5lts`, can be asked for by name instead. **That is the Snap channel's own default, not Blender naming a release**, and it says nothing about what Blender recommends.

### Outside the interface

Nothing in either quoted passage is conditioned on how Blender is driven. The production advice is explicitly about deployment in studios and schools, which is the setting where Blender is most likely to be driven by something other than a person, and it is stated without qualification. The sources do not say whether a different release would be advised for machine-driven use.

### Operating system

Neither passage mentions an operating system, and neither is placed on a per-platform page. The sources do not say the advice differs by operating system. The only release-and-platform statements found are limits on what can run where, quoted from the requirements page and the release notes above, not advice about which release to use.

---

## Things a person with Blender open could settle by hand in a few minutes

The list that stood here has been worked through. What was settled is written into the answers above, marked observed. What is left open is below.

1. **What a daily build reports.** Run a daily build with `--background --python-expr "import bpy; print(bpy.app.version_string, bpy.app.version_cycle, bpy.app.build_type, bpy.app.build_branch, bpy.app.build_hash)"` and record `version_cycle` and `build_branch`. This settles how an alpha or beta build marks itself, which the documentation states only in general terms. No daily build was available when the tests above were run. (Bears on question 1.)

2. **Whether the download page offers a machine-readable listing, and whether it names a default release.** Open the page the manual's installing pages point to, and note whether it offers anything other than links for a person to click — an index, a stable address pattern for a version, or a listing — and which release it presents first or most prominently, with the exact wording of any recommendation. Record the exact addresses seen. This is a question about a website rather than about an installed Blender, so no test on a computer with Blender on it can answer it. (Bears on questions 2 and 4.)

3. **What a daily build's file is called.** Question 1 now records the file names of a finished release, read from Blender's own download server on a rented Linux machine. The equivalent for a daily build is still owed. (Bears on question 1.)

---

2026-08-04 — Opus 5 (Claude Code) — Builds, Blender 4.5

2026-08-04 — Opus 5 (Claude Code) — wrote the hand-test results into questions 1 and 2, brought the outside-the-interface and operating-system lines up to date where a test settled them, and pruned the hand-test list at the end

2026-08-05 — Opus 5 (Claude Code) — wrote the Creator's splash-screen, status-bar and About Blender screenshots into question 1, brought the outside-the-interface line up to date, and pruned the hand-test list at the end

2026-08-05 — Opus 5 (Claude Code) — citation check: corrected question 4's description of the two ruled-out hardware-page quotes, which are about Monado, an XR platform, not a graphics backend

2026-08-06 — Opus 5 (Claude Code) — wrote the Linux hand-test results into questions 1, 2 and 4, from a rented Linux machine; settled what a published build file is called and that Snap carries a machine-readable listing of every release line; brought both standing lines up to date and pruned the list at the end
