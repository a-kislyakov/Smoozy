# Builds — Blender 5.3

- **Sources consulted:** Blender 5.3 manual (https://docs.blender.org/manual/en/5.3/), Blender 5.3 Python API documentation (https://docs.blender.org/api/5.3/) and Blender 5.3 release notes (https://developer.blender.org/docs/release_notes/5.3/), all as copies taken on 2026-10-11, and Cycles' own files for 5.3, taken on 2026-10-16. Those are the dates read used below.
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Notes on that line, so nothing is hidden:

- The tool I run in put three things into my starting context without my asking: the text of the project's `CLAUDE.md`, a one-line-per-entry index of the user's saved notes, and the repository's file status. I did not open any of those files. None of that text says anything about how Blender builds behave, and nothing below draws on it.
- Inside `Blender Sources/` I read the folder's own note, `About this folder.md`. It names a `requirements/` folder holding Blender's requirements page. That page is not among the sources named in my brief, so I listed the folder's file name only and did not open it.
- Before writing this file I checked whether it already existed. I used a command that printed only a count of matching names (it printed 0). I saw no other file names or contents in that folder.
- I made PNG copies of two manual pictures (`interface_splash_current.webp`, `editors_preferences_section_experimental.webp`) in my notes folder so I could view them.

Every web address below follows the rules in my brief. Quotes are exact. Where the plain-text copy broke a sentence across lines, I joined the lines with a space. Where a quote runs a heading into the text under it, I put " — " between them. "…" marks words left out.

---

## 1. What kinds of build does Blender publish, and how is each kind named and marked?

The question can be read two ways: "kind" as how stable a build is, or "kind" as the form of the download. The sources describe both, so both are answered.

**Answer.**

*By stability.* The manual names four kinds of package:
- **Stable Release**: a new one roughly every three months.
- **Long-term Support (LTS)**: supported for two years, with occasional small patch releases.
- **Daily Builds**: built automatically every day. They are marked **Alpha** or **Beta**. The manual also names **Release Candidate (RC)** as the step between Beta and the final release.
- **Build from Source**: you compile it yourself, so it is not a published binary.

Experimental preferences exist only in Daily Builds.

*By form of download.* The installing pages name these:
- Linux: an archive from blender.org, Snap, or a Linux distribution's own package.
- macOS: a disk image (dmg-file) holding `Blender.app`.
- Windows: a zip-file, a "Windows Installer File", or the Microsoft Store.
- Steam: on Linux, macOS and Windows.
- A pre-compiled `bpy` Python module "available via PIP". The API docs say it is not on Blender's official download page.

Windows downloads come in x64 and arm64 variants. Linux downloads are "for your architecture". From 5.0 on, macOS builds are for Apple silicon only.

*How they are named and marked.*
- **Version numbers** have three parts (major, minor, micro). LTS patch releases are numbered like "4.2.6".
- **LTS releases**: the manual's version list writes "LTS" after the version number, as in "Blender 4.5 LTS". How published binary packages are named or marked is not in the sources.
- **Stage of a build** is recorded inside the program as one of alpha, beta, rc or release (`bpy.app.version_cycle`). The splash-screen picture in the manual shows the label "5.3.0 Alpha" in its top right corner.
- **Branch**: the About menu shows an optional branch name. The 5.3 release notes say 5.3 is developed in the branch `blender-v5.3-release`, and that 5.3 is "currently in **Beta**".

**Label:** documented, except that marking published LTS builds with "LTS" after the number is inferred from the manual's version list rather than documented for build packages. What the splash picture shows is documented. The picture only shows the label "5.3.0 Alpha"; that it carries the same value as `bpy.app.version_cycle` is not stated, and I do not claim it.

**Exact words and where:**
- "Blender offers a variety of different binary packages to choose from depending on their level of stability." / "Stable Release — A package that contains the latest features and is considered stable without regressions. A new stable version is available roughly every three months." / "Long-term Support — A package designed for long-lasting projects requiring a very stable version of Blender. LTS releases are supported for two years and will not have any new features, API changes or improvements. A new long-term support version is available every year. These LTS releases will occasionally have minor patches (such as 4.2.6) which improve stability or fix critical bugs." / "Daily Builds — A package updated daily to include the newest changes in development. These versions are automatically built on a schedule. They are not as thoroughly tested as the release types above, and might break or crash. Builds marked as Alpha are still undergoing major changes and feature additions, while those marked Beta are feature-complete and are under development for refinement and stability." / "Stability can be expected to increase from Alpha to Beta to Release Candidate (RC) to a final release." / "Build from Source — Blender’s source code is available for free to either reference or to build and use. While normal users are not expected to compile Blender" — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/index.html, read 2026-10-11.
- "Experimental features are only available in Daily Builds." — manual, https://docs.blender.org/manual/en/5.3/editors/preferences/experimental.html, read 2026-10-11.
- "Download the Linux version for your architecture and decompress the file to the desired location" / "Some Linux distributions may have a specific package for Blender in their repositories." / "snap install blender --classic" — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/linux.html, read 2026-10-11.
- "Blender 5.0 and later only supports Macs with Apple silicon." / "Blender for macOS is distributed as disk images (dmg-files)." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/macos.html, read 2026-10-11.
- "Download the zip-file or Windows Installer File." / "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU’s architecture." / "Blender can be installed from the Microsoft Store by searching for Blender in the Microsoft Store and installing it." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/windows.html, read 2026-10-11.
- "Blender can be downloaded and updated using the Steam client by following the steps described below on Linux, macOS, or Windows." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/steam.html, read 2026-10-11.
- "Blender as a Python Module isn’t provided on Blender’s official download page." / "A pre-compiled bpy module is available via PIP." — API docs, https://docs.blender.org/api/5.3/info_advanced_blender_as_bpy.html, read 2026-10-11.
- "Blender 4.5 LTS" (in the list of manual versions) — manual, https://docs.blender.org/manual/en/5.3/versions.html, read 2026-10-11.
- "bpy.app.version_cycle — The release status of this build alpha/beta/rc/release" / "bpy.app.version — The Blender version as a tuple of 3 numbers (major, minor, micro). eg. (4, 3, 1)" / "bpy.app.build_branch — The branch this blender instance was built from" — API docs, https://docs.blender.org/api/5.3/bpy.app.html, read 2026-10-11.
- "The upper part of the splash screen contains the splash image with the Blender version in the top right." — manual, https://docs.blender.org/manual/en/5.3/interface/window_system/splash.html, read 2026-10-11. The picture on that page, `_images/interface_splash_current.webp` (https://docs.blender.org/manual/en/5.3/_images/interface_splash_current.webp), shows the text "5.3.0 Alpha" in its top right corner.
- "Branch: Optional branch name." — manual, https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html, read 2026-10-11.
- "Blender 5.3 is currently in **Beta** until November 4, 2026." / "Under development in [`blender-v5.3-release`]" — release notes, https://developer.blender.org/docs/release_notes/5.3/, read 2026-10-11.

**Not covered:**
- The file names of the downloads (for example, how a Linux archive or a daily build file is named) are not in the sources.
- The sources do not say whether RC builds come out as daily builds or separately.
- They do not say whether builds made from feature branches are published. The manual mentions branches only under building from source: "It allows access to any version or branch where a feature is being developed."
- They do not say which stability kinds Snap, the Microsoft Store or Steam offer.
- They do not say which Linux architectures exist.
- They do not say whether a Linux distribution's package counts as "published by Blender". The manual presents it as made by the distribution and says "the official binary is available on blender.org".

**Where I looked:**
- Read every page under `getting_started/installing/`, plus `advanced/deploying_blender`, `editors/preferences/experimental`, `versions`, `getting_started/about/history`, the splash and topbar pages, and the API pages `bpy.app` and `info_advanced_blender_as_bpy`.
- Searched the whole manual, API docs and release notes for: daily build, experimental build, branch build, LTS, long-term, release candidate, nightly, buildbot, builder.blender, download.blender, blender.org/download, alpha, beta, and the download-name patterns linux-x64, windows-x64, windows-arm64, macos-arm64, macos-x64 and blender-<number>.
- The platform-name hits (such as "windows-x64") are all about extensions, not Blender builds, so I ruled them out.
- The Cycles release note mentions "Blender daily builds" but adds nothing about naming.

**Observed — hand test, 2026-10-18 (hand-test list entry 1).** Run on the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83). The build was Blender 5.3.0 Beta, built 2026-10-16, at `/Applications/Blender 5.3b.app`. Blender was started from a terminal with no window. Its settings folder, temporary folder and home folder were pointed at folders of the runner's own outside the project (`BLENDER_USER_RESOURCES`, `TMPDIR` and `HOME`), made before it started, and Blender reported them as the ones in use. This is the same test the four earlier versions ran on the Mac, run on a Beta for the first time.

*What the build prints when asked its version.* `"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --version` printed, word for word, with exit code 0:

```
Blender 5.3.0 Beta
	build date: 2026-10-16
	build time: 01:36:01
	build commit date: 2026-10-16
	build commit time: 00:30
	build hash: 5d8151335b6b
	build branch: blender-v5.3-release
	build platform: Darwin
	build type: Release
```

Three very long lines followed — `build c flags:`, `build c++ flags:` and `build link flags:`, lists of compiler and linker settings — and then `	build system: CMake`. The first line carries the word `Beta` after the three-number version. The branch line reads `blender-v5.3-release`.

**Observed — hand test, 2026-10-18 (hand-test list entry 2).** Same machine, build and folders as entry 1 above. *What the build reports about its stage in Python, with no window open.* The full listing of `bpy.app` is under question 3; the lines that carry the marks, word for word:

```
build_branch = b'blender-v5.3-release'
version = (5, 3, 0)
version_cycle = 'beta'
version_file = (5, 3, 29)
version_string = '5.3.0 Beta'
```

`version_cycle` reads `beta`, one of the four values the API documentation lists, and `version_string` carries the word `Beta` after the three numbers. The letters `beta` appear nowhere else in the listing.

**Observed — hand test, 2026-10-18 (hand-test list entry 3).** Same machine. Nothing was started; each command only read the installed application at `/Applications/Blender 5.3b.app`.

*What the program file is.* `file "/Applications/Blender 5.3b.app/Contents/MacOS/Blender"` printed `/Applications/Blender 5.3b.app/Contents/MacOS/Blender: Mach-O 64-bit executable arm64`, and `lipo -archs` on it printed `arm64`.

*What the application's own information file carries.* `plutil -p "/Applications/Blender 5.3b.app/Contents/Info.plist"` printed, among other lines:

```
  "CFBundleGetInfoString" => "5.3.0 2026-10-16, Blender Foundation"
  "CFBundleIconName" => "blender_liquid_glass"
  "CFBundleIdentifier" => "org.blenderfoundation.blender"
  "CFBundleName" => "Blender"
  "CFBundleShortVersionString" => "5.3.0"
  "CFBundleVersion" => "5.3.0"
  "LSMinimumSystemVersion" => "13.0"
```

No line in the information file carries the word `Beta` or any other stage word. The application's folder is named `Blender 5.3b.app` on this Mac; that name is the one it was given when it was put in `/Applications`, and this sitting's inputs do not record who gave it.

**Observed — hand test, 2026-10-18 (hand-test list entry 4).** Same machine, build and folders as entry 1. Blender 5.3.0 Beta was started **with its window**, from a terminal, with `--factory-startup --no-window-focus --python <script>`; the script waited four seconds and then saved a picture of the window with Blender's own screenshot command, `bpy.ops.screen.screenshot(filepath=...)`, called with the window as its context. The picture was then read.

*The splash screen.* The top right corner of the splash image reads `5.3.0 Beta`. Because the settings folder given to Blender was empty, the splash showed its *Quick Setup* panel (Language, Theme, Keymap, Mouse Select, Spacebar Action, Continue) under the image.

*The right-hand end of the bar along the bottom of the window* reads `5.3.0 b`, with a globe icon struck through by a diagonal line, and a vertical divider, to its left.

**Not run, and why — 2026-10-18.** *The file names Blender's download server publishes for 5.3, and its checksum files:* reading the server is fetching from the internet, which that sitting did not do. *How a 5.3 daily build or release candidate marks itself:* neither was on the Mac. *Everything on Linux* (the Linux program file, its `--version`, its splash and About panel, a Snap build): no Linux machine in that sitting. *Windows, the Microsoft Store and Steam:* no Windows machine and no store account.

**Outside the interface:** The `bpy.app` values that carry the marks (version, stage, branch) can be read from Python without a window. The API docs present them as plain values and do not say they need the interface. The splash screen, the About menu and the status bar are inside the interface only. Of the download forms, only Snap is described with a command; the Microsoft Store and Steam are described as steps a person takes in those programs. **Observed, 2026-10-18, on the Creator's Mac:** `--version` and the `bpy.app` values were read from a terminal with no window, and both carry the stage word `Beta`. The splash corner (`5.3.0 Beta`) and the bottom bar (`5.3.0 b`) were read from Blender's window, opened and photographed from a terminal by a script.

**Operating system:** The forms differ by system:
- dmg: macOS only.
- zip-file, Windows Installer File and Microsoft Store: Windows only.
- Archive, Snap and distribution packages: Linux.
- Steam: all three.

macOS builds are Apple-silicon only from 5.0. Windows has x64 and arm64 variants. The stability kinds are not described as differing between systems. **Observed for macOS only, 2026-10-18:** the installed 5.3.0 Beta Mac program holds one processor, `arm64`, and reports `build platform: Darwin`. Nothing was run on Linux or Windows for 5.3: there was no machine of either kind in that sitting. The file names on Blender's download server were not read, because that sitting fetched nothing from the internet.

---

## 2. Can builds of each kind be listed and fetched by something other than a person at a web page?

**Answer.** For the stability kinds (Stable, LTS, Daily, and their Alpha/Beta/RC marks), the sources describe no way to list or fetch them other than downloading from the Blender website. There is no listing service, no address pattern and no feed.

The sources do describe some routes that do not go through a person at a web page, for some forms of download:
- On Linux, Snap installs Blender with one command, and Snap installs updates automatically.
- Linux distribution package managers can install Blender, and "may provide" listing of packages and update notifications.
- The Microsoft Store and Steam download and install updates automatically. The first install, though, is described as a person searching in the store or client and pressing install.
- The `bpy` Python module is "available via PIP".
- Once a build has been downloaded, the deployment page says putting it in place "can be automated using standard file management tools".

**Label:** documented for the routes quoted. Not in the sources: any way to list or fetch each stability kind by machine.

**Exact words and where:**
- "Snap is a universal package manager designed to work across a range of distributions. Assuming snap is already installed, Blender can be installed through snap with: snap install blender --classic" / "Installing from this method has a benefit that updates to Blender are automatically installed." / "Installing Blender via the distribution’s native mechanisms ensures consistency with other packages on the system and may provide other features (given by the package manager), such as listing of packages, update notifications and automatic menu configuration." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/linux.html, read 2026-10-11.
- "When an update for Blender is available on the Microsoft Store, it will be downloaded and installed automatically." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/windows.html, read 2026-10-11.
- "When an update for Blender is available on Steam, Steam will automatically download and apply the update for you." / "navigate to the Store tab, search for “Blender”, and press the green Install button." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/steam.html, read 2026-10-11.
- "A pre-compiled bpy module is available via PIP." — API docs, https://docs.blender.org/api/5.3/info_advanced_blender_as_bpy.html, read 2026-10-11.
- "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools." — manual, https://docs.blender.org/manual/en/5.3/advanced/deploying_blender.html, read 2026-10-11.
- "Blender is designed to not require an internet connection, so it doesn’t have a built-in update system." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/index.html, read 2026-10-11.

**Not covered:**
- Whether blender.org or the daily-build site offers any list a program can read.
- Which stability kind Snap, PIP, Steam or the Store deliver, and whether any of them offers LTS or daily builds.
- Whether a particular older version can be fetched through any of these routes.
- Whether the Microsoft Store or Steam can be driven without a person.

The manual points readers to a "Downloading Blender page" for versions and requirements. That page is not among my sources.

**Where I looked:**
- Read every installing page and the deployment page.
- Searched the manual, API docs and release notes for: download, builder, buildbot, nightly, daily build, json, rss, pypi, "pip install bpy", winget, chocolatey, homebrew, brew, flatpak, flathub, steamcmd, "apt install blender", "snap install", "snap refresh", --channel, checksum, sha256.
- The only command found is the snap one.
- Hits on `extensions.blender.org/api/v1/extensions` and on extension download links concern add-ons and extensions, not Blender builds, so I ruled them out.
- The command-line extension commands ("Download package information for remote repositories") are also about extensions, and I ruled them out too.

**Observed — hand test, 2026-10-18 (hand-test list entry 1).** Run on the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — against Blender 5.3.0 Beta at `/Applications/Blender 5.3b.app`, from a terminal with no window, with its settings, temporary and home folders pointed at folders of the runner's own.

*What the build offers as commands of its own.* `"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -c help` printed, word for word, with exit code 0:

```
Blender Command Listing:
	asset_listing
	extension
	maketx
```

Nothing else. *What its arguments list holds.* `--help` printed 521 lines, exit code 0. Searching all 521 for `download`, `fetch`, `install` and `update`, ignoring capitals, found none of the four.

**Not run, and why.** Every other route a machine could use to list or fetch builds needs either the internet or another machine, and this sitting had neither: Blender's own download server, Snap and a Linux distribution's package manager (no Linux machine), the Microsoft Store (no Windows machine), Steam (needs an account, which an agent on this project never makes or signs in to), and Blender's Python module on its package index (the internet). They stay untested for 5.3.

**Outside the interface:** The snap command and PIP are command-line routes by their nature. Store and Steam installs are described only as steps a person takes in those programs. Their automatic updates are described as happening without the user, but the sources do not say whether a program can trigger them. **Observed, 2026-10-18, on the Creator's Mac:** the build's own command listing (`-c help`) and its 521-line arguments list (`--help`) were both read from a terminal with no window; the listing names `asset_listing`, `extension` and `maketx`, and the words download, fetch, install and update appear in neither.

**Operating system:** Snap and distribution packages: Linux only. Microsoft Store: Windows only. Steam: Linux, macOS and Windows. PIP: the sources do not say which systems the `bpy` module is offered for. **Observed for macOS only, 2026-10-18,** as above. Nothing was run on Linux or Windows for 5.3: there was no machine of either kind in that sitting.

---

## 3. Can an installed Blender be asked what it is — which version, and which kind of build?

**Answer.** Yes for the version and for the stage of the build. Partly for other aspects of "kind".

- **From the command line:** `-v, --version` prints the version and exits.
- **From Python:** `bpy.app` gives:
  - the version: `version` as three numbers, plus `version_string`;
  - the stage: `version_cycle`, one of alpha, beta, rc or release;
  - build details: `build_branch`, `build_hash`, `build_date`, `build_platform`, and `build_type` ("Release, Debug");
  - the compiled-in features: `build_options`, which includes an `experimental_features` entry that the documentation does not explain;
  - where its own program file is: `binary_path`;
  - `portable`: "True unless blender was built to reference absolute paths (on UNIX)".
- **Inside the interface:** the About menu shows the version, build date, Git hash and optional branch name. The status bar can show the running version, and the splash image shows the version.

LTS: no `bpy.app` value names LTS, and a search of the API docs for "LTS" finds nothing.

Form of the download (zip, installer, Store, distribution package): no value states it directly. The manual's directory-layout page gives a different system folder for each form. `bpy.utils.resource_path('SYSTEM')` and `bpy.app.binary_path` report where the running Blender's files are. Comparing that location with the documented folders would show the form in some cases, for example the Microsoft Store folder `WindowsApps\BlenderFoundation.Blender<HASH>`. That last step is mine, not the source's.

**Label:**
- Documented: the command-line flag, the `bpy.app` values, and the About, status bar and splash contents.
- Not in the sources: that LTS can be read from the program. The API docs have no LTS value.
- Inferred: telling the download form from the install path. The step: the directory-layout page lists a distinct system folder per form, and `resource_path`/`binary_path` return the running Blender's folders, so matching the two would show the form wherever the folders differ.

**Exact words and where:**
- "-v, --version — Print Blender version and exit." — manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11.
- From the API docs, https://docs.blender.org/api/5.3/bpy.app.html, read 2026-10-11:
  - "bpy.app.version_cycle — The release status of this build alpha/beta/rc/release"
  - "bpy.app.version_string — The Blender version formatted as a string"
  - "bpy.app.version — The Blender version as a tuple of 3 numbers (major, minor, micro). eg. (4, 3, 1)"
  - "bpy.app.build_branch — The branch this blender instance was built from"
  - "bpy.app.build_hash — The commit hash this Blender instance was built with"
  - "bpy.app.build_date — The date this Blender instance was built"
  - "bpy.app.build_type — The type of build (Release, Debug)"
  - "bpy.app.build_platform — The platform this blender instance was built for"
  - "bpy.app.portable — Boolean, True unless blender was built to reference absolute paths (on UNIX)."
  - "bpy.app.binary_path — The location of Blender’s executable, useful for utilities that open new instances."
  - "Constant value bpy.app.build_options(… experimental_features=False)"
  - "This module contains application values that remain unchanged during runtime."
- "bpy.utils.resource_path(type, *, major=None, minor=None) — Return the base path for storing system files." with type "Literal['USER', 'LOCAL', 'SYSTEM', 'SYSTEM_LIBS']" — API docs, https://docs.blender.org/api/5.3/bpy.utils.html, read 2026-10-11.
- "About Blender — Opens a menu displaying the following information about Blender: Version: The Blender version. Date: Date when Blender was compiled. Hash: The Git Hash of the build. … Branch: Optional branch name." — manual, https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html, read 2026-10-11.
- "Blender Version — Shows the version number of Blender that is currently running." — manual, https://docs.blender.org/manual/en/5.3/interface/window_system/status_bar.html, read 2026-10-11.
- System folders per form: "Zip file downloaded from blender.org: ./5.3/" / "Installer downloaded from blender.org: %ProgramFiles%\Blender Foundation\Blender\5.3\" / "Microsoft Store installation: %ProgramFiles%\WindowsApps\BlenderFoundation.Blender<HASH>\Blender\5.3\" / "Linux distribution packages: /usr/share/blender/5.3/" / "Archive downloaded from blender.org: ./5.3/" — manual, https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html, read 2026-10-11.

**Not covered:**
- The exact text `--version` prints, including whether it shows the stage, hash, branch or date.
- What `version_string` looks like for each stage.
- Whether `build_branch` is empty or a fixed word for official releases.
- What `experimental_features` means, and whether it is true in daily builds.
- How to tell a daily Beta from an RC except through `version_cycle`.
- How to tell a Steam or Snap install.

The documented `build_options` constant shows `experimental_features=False` for whichever build made the documentation. Which build that was is not stated.

**Where I looked:**
- Read the API page `bpy.app` in full, the arguments page, and the topbar, status bar and splash pages.
- Searched the API docs for version_cycle, LTS, "daily build", "long-term", "release candidate", experimental_features and build_options. Only `bpy.app` and the index pages matched; there was no LTS hit.
- Searched the manual for "about blender", "build hash", "commit hash", "version number", "version string", "build date" and "branch".
- Checked `bpy.ops.wm.sysinfo` ("Generate system information, saved into a text file"). The docs do not say what it writes, so I did not use it.
- Cycles' own files read no build information beyond preferences.

**Observed — hand test, 2026-10-18 (hand-test list entry 1).** Run on the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — against Blender 5.3.0 Beta at `/Applications/Blender 5.3b.app`, from a terminal with no window, with its settings, temporary and home folders pointed at folders of the runner's own.

*From the command line.* `"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --version` printed the block quoted under question 1, exit code 0. It carries the stage word `Beta` on its first line, then the build date and time, the commit date and time, the hash `5d8151335b6b`, the branch `blender-v5.3-release`, `build platform: Darwin`, `build type: Release`, three long flag lines and `build system: CMake`.

*What the arguments list itself says.* `--help` printed 521 lines, exit code 0. The version entry, word for word:

```
-v or --version 
	Print Blender version and exit.
```

with no platform note beside it, while the same list prints `/?` as `Print this help text and exit (Windows only).` and `--qos <level>` as `Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only).`

**Observed — hand test, 2026-10-18 (hand-test list entry 2).** Same machine, build and folders as entry 1. Every command below was typed in a terminal and ran with no window.

*Every value on `bpy.app`.* The command the 5.2 Mac test ran, with only the program's path changed:

`"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --background --python-expr "import bpy; print('BEGIN'); [print(a, '=', repr(getattr(bpy.app, a))) for a in sorted(dir(bpy.app)) if not a.startswith('_')]; print('TYPE version_cycle', type(bpy.app.version_cycle)); print('TYPE build_type', type(bpy.app.build_type)); print('END')"`

It ended with exit code 0 and printed 73 names. The lines this question asks about, word for word:

```
background = True
binary_path = '/Applications/Blender 5.3b.app/Contents/MacOS/Blender'
build_branch = b'blender-v5.3-release'
build_commit_date = b'2026-10-16'
build_commit_time = b'00:30'
build_commit_timestamp = 1792110656
build_date = b'2026-10-16'
build_hash = b'5d8151335b6b'
build_platform = b'Darwin'
build_system = b'CMake'
build_time = b'01:36:01'
build_type = b'Release'
cachedir = '/Users/artist/Library/Caches/Blender/'
module = False
portable = True
version = (5, 3, 0)
version_cycle = 'beta'
version_file = (5, 3, 29)
version_string = '5.3.0 Beta'
TYPE version_cycle <class 'str'>
TYPE build_type <class 'bytes'>
```

Also printed, and kept out of the block above because they are very long: `build_cflags`, `build_cxxflags` and `build_linkflags` (the same compiler and linker settings `--version` prints), and `driver_namespace` (a list of Python's own functions). Searching the whole listing for the letters `beta` or `LTS`, ignoring capitals, found `beta` on the `version_cycle` and `version_string` lines only, and `LTS` nowhere. The `cachedir` line names a folder under the Creator's own home folder although the run's home folder had been pointed elsewhere.

*The command the 5.3 audit sent, word for word.* `"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -b --factory-startup --python-expr "import bpy; print(repr(bpy.app.version_string), bpy.app.version_cycle, bpy.app.build_branch, bpy.app.build_hash, bpy.app.build_type, bpy.app.build_platform, bpy.app.portable, bpy.app.build_options.experimental_features, bpy.app.binary_path, bpy.utils.resource_path('SYSTEM'))"` printed, with exit code 0:

```
'5.3.0 Beta' beta b'blender-v5.3-release' b'5d8151335b6b' b'Release' b'Darwin' True False /Applications/Blender 5.3b.app/Contents/MacOS/Blender /Library/Application Support/Blender/5.3
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

So on this build `portable` reads `True` and `build_options.experimental_features` reads `False`.

*What this build was built with.* `"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --background --python-expr "import bpy; print(bpy.app.build_options)"` printed, exit code 0:

```
bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=True, jack=False, pulseaudio=False, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_spz=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, opentimelineio=True, potrace=True, pugixml=True, haru=True, experimental_features=False)
```

The documented value is quoted above only in part — `Constant value bpy.app.build_options(… experimental_features=False)` — and its one quoted entry matches this build's. The rest was not set side by side with the documentation in this sitting.

*As a saved file.* `"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --background --python-expr "import bpy; bpy.ops.wm.sysinfo(filepath='<work folder>/system-info.txt')"` printed `Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)` and `Blender quit`, exit code 0, and the file was written. Its opening and Blender section, word for word:

```
=========================================
= Blender 5.3.0 Beta System Information =
=========================================


Blender:
=========================================

version: 5.3.0 Beta, branch: blender-v5.3-release, commit date: 2026-10-16 00:30, hash: 5d8151335b6b, type: Release
build date: 2026-10-16, 01:36:01
platform: 'macOS-26.6.2-arm64-arm-64bit-Mach-O'
binary path: '/Applications/Blender 5.3b.app/Contents/MacOS/Blender'
```

Three long build-flag lines follow, then `build system: CMake`. The `Python:` section opens `version: 3.13.13 (main, Apr 25 2025, 12:39:20) [Clang 21.0.0 (clang-2100.1.1.101)]`.

This answers, for this one Beta build, four of the *Not covered* items above: what `--version` prints (the stage, hash, branch and dates, as quoted under question 1), what `version_string` looks like for a Beta (`'5.3.0 Beta'`), what `build_branch` reads on it (`blender-v5.3-release`), and what `experimental_features` reads on it (`False`). The test does not show what `experimental_features` means.

**Observed — hand test, 2026-10-18 (hand-test list entry 4).** Same machine, build and folders as entry 1. Blender 5.3.0 Beta was started **with its window**, from a terminal, with `--factory-startup --no-window-focus --python <script>`; the script waited on timers, made a call, and saved a picture of the window with Blender's own screenshot command, `bpy.ops.screen.screenshot(filepath=...)`. Each picture was then read. The settings folder was the runner's own, and nothing was saved: every script ended with `os._exit(0)`.

*How the pictures were got, including the calls that drew nothing.* In the first run, with the start-up splash still showing, `bpy.ops.wm.splash_about()` printed `ERROR Invalid operator call 'WM_OT_splash_about'`, and `bpy.ops.wm.call_menu(name="TOPBAR_MT_help")` drew nothing — the pictures taken after each were identical, byte for byte, to the one taken before. So separate runs were made with the splash switched off by the script (`bpy.context.preferences.view.show_splash = False`, never saved). `bpy.ops.wm.splash_about('INVOKE_DEFAULT')` then drew the About panel. `bpy.ops.wm.call_menu(name="TOPBAR_MT_help")` again drew nothing. A call aimed at a `TOPBAR` area failed in the script itself (`IndexError: list index out of range` — the window's list of areas held none of that kind). Finally `bpy.ops.wm.call_menu('INVOKE_DEFAULT', name="TOPBAR_MT_help")`, with the 3D view's header region as its context, returned `{'INTERFACE'}` and the menu was drawn.

*The About Blender panel.* Its left-hand column reads, line by line:

```
Blender
Version: 5.3.0 Beta

Date: 2026-10-16 00:30
Hash: 5d8151335b6b
Branch: blender-v5.3-release

Blender is free software
Licensed under the GNU General Public License
```

Its right-hand column is a set of links: `Donate`, `What's New`, then `Credits`, `License`, `Blender Store`, `Blender Website`. Four of the five rows the manual documents are present — Version, Date, Hash and Branch. There is no `Windowing Environment` row. The stage word `Beta` is on the Version line. The panel shows no build time, platform, build type or build system.

*The splash screen and the bottom bar.* The top right corner of the splash image reads `5.3.0 Beta`. The right-hand end of the bottom bar reads `5.3.0 b`.

*The Help menu.* It drops down under the title `Help`, in order:

```
Manual
Support
User Communiti…
Get Involved
Release Notes
———
Report a Bug
Save System Info...
```

The third entry is cut short in the picture by a tooltip that sits over its end; the tooltip reads `Open a preset website in the web browser.` / `The reference manual for this version of Blender.`, and the bar along the bottom of the window reads `https://docs.blender.org/manual/en/5.3/` while `Manual` is highlighted. `Save System Info...` is the last entry, written with a trailing ellipsis.

**Not run, and why.** A 5.3 daily build and a 5.3 release candidate were not asked, because neither was on the Mac and this sitting downloaded nothing. Snap, Steam and Microsoft Store builds of 5.3 were not asked either, for want of a Linux machine, a Windows machine and a store account.

**Outside the interface:** `--version` is a command-line flag. The `bpy.app` and `bpy.utils` values can be read by a script passed on the command line, for example with `-b` and `--python-expr`; both are documented on the arguments page. The About menu, status bar and splash are interface only. **Observed, 2026-10-18, on the Creator's Mac:** with `--background` and no window, `--version`, every `bpy.app` value and `bpy.ops.wm.sysinfo` all came back, the last writing its file. The About panel was reached by a script in a window opened from a terminal, with `bpy.ops.wm.splash_about('INVOKE_DEFAULT')`; the same call without `'INVOKE_DEFAULT'`, made while the start-up splash was showing, was refused.

**Operating system:** The About menu also shows "Windowing Environment" on Linux only. `build_platform` names the platform. `portable` is described only "on UNIX". Nothing else is said to differ by system. **Observed for macOS only, 2026-10-18:** `build_platform` reads `b'Darwin'`, `portable` reads `True`, and the About panel has no `Windowing Environment` row. Nothing was run on Linux or Windows for 5.3: there was no machine of either kind in that sitting.

---

## 4. Does Blender name one release as the one to use for ordinary work?

**Answer.** No specific release is named. The manual describes the kinds of package and says the right one "depends on your requirements":
- Stable Release is "considered stable without regressions".
- LTS is "designed for long-lasting projects requiring a very stable version".
- The deployment page, written for studios and schools, says it is "advisable to use a single LTS version" for a given project.

The 5.3 release notes say 5.3 itself is in Beta until November 4, 2026.

**Label:** documented for the advice quoted. Not in the sources: a named release meant for ordinary work.

**Exact words and where:**
- "Each package has the trade off of newest features versus stability. The package that is right for you depends on your requirements for those two. A studio, for example, might want to have long-term support, while a hobbyist may want newer features, while others may just want to test upcoming features." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/index.html, read 2026-10-11.
- "New Blender versions may add, remove or change functionality that affects the results of production files. For a given project, it is advisable to use a single LTS version of Blender. LTS versions receive bug fixes for two years." — manual, https://docs.blender.org/manual/en/5.3/advanced/deploying_blender.html, read 2026-10-11.
- "Blender 5.3 is currently in **Beta** until November 4, 2026." — release notes, https://developer.blender.org/docs/release_notes/5.3/, read 2026-10-11.

**Not covered:**
- Which version number is the current Stable or the current LTS on any date. The manual's version list marks "Blender 4.5 LTS" among manual versions, but that is a list of manuals, not a recommendation.
- Whether blender.org's download page features one release. That page is not in my sources.

**Where I looked:**
- Read the installing pages, the deployment page, the version list and the history page; the history page stops at 4.3.
- Searched the installing pages, deployment page and version list for recommend, advisable, production, "latest stable", "latest release" and "latest official". Only the passages quoted, plus Linux's "outdated compared to the latest official release", matched.
- Searched the release notes for LTS and stable: nothing.

**Not run, and why — 2026-10-18.** The one test that bears on this question — what release a package route installs when no channel is named, and which release lines that route serves — needs Snap on a Linux machine and the internet. Neither was available to the hand tests of that day, which ran on the Creator's Mac alone and downloaded nothing.

**Outside the interface:** The question does not touch the interface. The sources say nothing that differs.

**Operating system:** The sources do not say the advice differs by system.

---

## 5. Can one Blender installed on a computer cause trouble for another that is already there?

**Answer.** The sources say several Blenders can live side by side, and give the conditions. They also name places where installs meet.

*Living side by side (documented):*
- Linux archives: "it is possible to have multiple versions of Blender installed".
- Windows zip: "It's perfectly possible to have multiple versions installed side by side."
- In general: "Multiple Blender versions can co-exist on the same system".
- Each download is "a self contained installation".
- Blender's bundled Python "does not interact with the system Python by default".

*Where one can disturb another:*
- **macOS (documented).** The documented way to update is to overwrite `Blender.app` in Applications. Keeping more than one means renaming the app or putting it in another folder.
- **File association and thumbnails (documented, with an inferred step).** The Windows installer associates blend-files with Blender. "Register" in Preferences makes "the currently in use Blender installation the default" for opening blend-files and making thumbnails, on Windows and Linux. Inferred: since there is one default, registering a second Blender takes that role away from the first. Installing also "adds a small tool to your OS" for blend-file thumbnails.
- **Settings shared by same-version builds (inferred).** User settings are kept in one folder per version number, such as `…/blender/5.3/`. So two installs with the same version number, for example a 5.3 Beta and a 5.3 daily build, read and write the same preferences, startup file and installed extensions. Preferences "are automatically saved when changed" by default. This holds unless one of them uses a `portable` folder or the `BLENDER_USER_RESOURCES` setting. The step: the folder name holds only the version number, so the documented layout gives both builds the same folder.
- **Windows installer folder (inferred).** The launch page gives the installer's program path as `C:\Program Files\Blender Foundation\Blender\blender.exe`, with no version in the path. The directory-layout page gives the installer's system folder as `%ProgramFiles%\Blender Foundation\Blender\5.3\`. So two installer runs that both keep the default folder would put their program files in the same place. The sources do not say what the installer then does. The installer "will let you choose an installation folder", and the update text speaks of uninstalling "the desired version", which suggests separate entries can exist. The sources do not settle it.

**Label:** documented for coexistence, the macOS overwrite and the registration/default text. Inferred for the three steps marked above.

**Exact words and where:**
- "When using this method of installation, it is possible to have multiple versions of Blender installed." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/linux.html, read 2026-10-11.
- "Install the new version by overwriting the current Blender.app in the Applications folder. You can rename Blender.app or place it in a different folder to have more than one version at a time." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/macos.html, read 2026-10-11.
- From the manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/windows.html, read 2026-10-11:
  - "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."
  - "No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."
  - "Note, you do not have to overwrite your existing Blender installation. It’s perfectly possible to have multiple versions installed side by side."
  - "To remove a previously installed version of Blender, use Windows settings or control panel to uninstall the desired version."
- "C:\Program Files\Blender Foundation\Blender\blender.exe" (as where the installer puts Blender) — manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/launch/windows.html, read 2026-10-11.
- "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system" / "Blender bundles a complete Python installation and does not interact with the system Python by default." — manual, https://docs.blender.org/manual/en/5.3/advanced/deploying_blender.html, read 2026-10-11.
- "Make this installation your default Blender (MS-Windows & Linux only)." / "Register — Make the currently in use Blender installation the default for generating thumbnails and the default for opening blend-files." / "For All Users — Register Blender for all users, requires escalated privileges." / "Files are setup files under: /usr/local for all users, otherwise ~/.local is used." / "On Linux, if Blender is installed from a package manager such as Snap, file association is handled by the package manager." — manual, https://docs.blender.org/manual/en/5.3/editors/preferences/system.html, read 2026-10-11.
- "During its installation, Blender also adds a small tool to your OS, that will allow your system file browser to show those previews as file thumbnails as well." — manual, https://docs.blender.org/manual/en/5.3/files/blend/previews.html, read 2026-10-11.
- "The preferences need to be imported from previous versions because the configuration files of each Blender version are stored in separate folders." / "The preferences are automatically saved when changed." — manual, https://docs.blender.org/manual/en/5.3/getting_started/configuration/defaults.html, read 2026-10-11.
- From the manual, https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html, read 2026-10-11:
  - "$HOME/.config/blender/5.3/" / "/Users/$USER/Library/Application Support/Blender/5.3/" / "%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.3\"
  - "create a folder named portable" … "This folder will then store preferences, startup file, installed extensions and presets."
  - "The BLENDER_USER_RESOURCES environment variable can be set to a custom directory to replace the default user directory."
  - "Installer downloaded from blender.org: %ProgramFiles%\Blender Foundation\Blender\5.3\"

**Not covered:**
- What the Windows installer does when another installer-installed Blender is in the same folder.
- What happens when two Blenders each install the thumbnail tool, or one is unregistered.
- Whether a Snap, Store or Steam install affects an archive install.
- Whether builds of different versions disturb each other in any shared place not named here, such as GPU or cache folders. I found no statement either way.
- That two same-version builds actually share settings is my reading of the folder layout; it has not been tried.

**Where I looked:**
- Read every installing page, the deployment, directory-layout, system-preferences, defaults, previews, recover and startup pages.
- Searched the manual for: "multiple versions", "side by side", co-exist, coexist, "previous version", "older version", "newer version", "other versions", "another version", "another instance", "multiple instances", "already running", "file association", associate, register, unregister, "for all users", installation, "kernel cache", "shader cache", "cache directory".
- Searched the API guide pages for install, "multiple versions", "different version" and "blender version". The tips page says the same as the deployment page about bundled Python.

**Observed — hand test, 2026-10-18 (hand-test list entry 1).** Run on the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — against Blender 5.3.0 Beta at `/Applications/Blender 5.3b.app`, from a terminal with no window, with its settings, temporary and home folders pointed at folders of the runner's own.

*The registration arguments, as the Mac build's own help prints them.* `--help` printed 521 lines. The four entries, word for word:

```
-r or --register 
	Register blend-file extension for current user, then exit (Windows & Linux only).

--register-allusers 
	Register blend-file extension for all users, then exit (Windows & Linux only).

--unregister 
	Unregister blend-file extension for current user, then exit (Windows & Linux only).

--unregister-allusers 
	Unregister blend-file extension for all users, then exit (Windows & Linux only).
```

None of them was run: the help marks all four for Windows and Linux only, and running one on the Creator's own computer could change which program opens their files.

**Observed — hand test, 2026-10-18 (hand-test list entry 4).** Same machine and build. Blender 5.3.0 Beta was started **with its window** from a terminal, with `--factory-startup --no-window-focus` and a script, and the runner's own settings folder. The script turned the largest area of the window into the Preferences editor (`area.ui_type = "PREFERENCES"`), chose the System section (`bpy.context.preferences.active_section = "SYSTEM"`), made that area fill the window (`bpy.ops.screen.screen_full_area()`), and saved a picture with Blender's own screenshot command. Nothing was saved: the script ended with `os._exit(0)`.

The picture shows the System page with five sections, in this order: `Cycles Render Devices`, `Network`, `Memory & Limits`, `Video Sequencer` and `Sound` (the last folded shut), and empty space below `Sound` to the bottom of the editor. No section headed `Operating System Settings` appears, and no control named `Register`, `Unregister` or `For All Users`. The `Cycles Render Devices` section shows two choices, `None` and `Metal`, with `Metal` chosen and `Apple M1 Pro (GPU - 14 cores)` ticked and `Apple M1 Pro` unticked beneath it.

**Observed — hand test, 2026-10-18 (hand-test list entry 5), and only half of it could be run.** Same machine. The test the four earlier versions ran on the Mac: two copies of one version, each asked which settings folder it uses. A second copy of the installed 5.3 was made in a work folder outside the project with `cp -Rc "/Applications/Blender 5.3b.app" "<work folder>/05/copy/Blender.app"`, exit code 0.

*How it was guarded.* For this test Blender's settings folder was **not** pointed elsewhere, so that Blender would work out its ordinary folder by itself; the home folder was pointed at an empty folder of the runner's own. An earlier run had shown that on this Mac Blender's cache folder ignores the home folder, so each Blender was also started inside macOS's `sandbox-exec` with a rule forbidding it to read or write anything under the Creator's own `/Users/artist/Library/Application Support/Blender`. The command, for each of the two:

`env -u BLENDER_USER_RESOURCES HOME="<work folder>/05/home" sandbox-exec -p '(version 1)(allow default)(deny file-read* file-write* (subpath "/Users/artist/Library/Application Support/Blender"))' "<path to the program>" --background --factory-startup --python-expr "import bpy; print('USER', bpy.utils.resource_path('USER')); print('LOCAL', bpy.utils.resource_path('LOCAL')); print('SYSTEM', bpy.utils.resource_path('SYSTEM')); print('BINARY', bpy.app.binary_path)"`

*The installed 5.3 printed*, with exit code 0:

```
USER /Users/artist/Library/Application Support/Blender/5.3
LOCAL /Applications/Blender 5.3b.app/Contents/Resources/5.3
SYSTEM /Library/Application Support/Blender/5.3
BINARY /Applications/Blender 5.3b.app/Contents/MacOS/Blender
```

So the ordinary settings folder it works out is under the Creator's own home folder although the run's home folder pointed elsewhere. macOS's own log recorded that, during that run, the guard refused it twice: `Sandbox: Blender(57733) deny(1) file-read-metadata /Users/artist/Library/Application Support/Blender/5.3/config` (the second as `2 duplicate reports for …`). A search afterwards for anything in the Creator's `Blender` settings and cache folders changed since the test began printed nothing. The Creator's `Blender` settings folder holds folders named `4.5`, `5.0`, `5.1`, `5.2`, `5.3` and `data` (names only were read).

*The copy did not start.* Its run printed nothing at all. After four minutes the process was still waiting, and a sample of it showed its only thread at the program loader (`_dyld_start (in dyld)`), before any of Blender had run; it was stopped, and the command reported exit code 143. A second start of the copy, with only `--version` and a ten-minute limit, printed nothing for the whole ten minutes and was stopped by the limit. macOS's own log for the first start, read with `/usr/bin/log show` and each line cut at 260 characters by the reading command, holds these lines in this order, word for word:

```
2026-10-18 03:09:55.585 I  amfid[36697:1e95d81] Entering OSX path for /private/tmp/claude-501/smoozy-hand-tests-5.3/Builds/05/copy/Blender.app/Contents/MacOS/Blender
2026-10-18 03:10:01.646 I  syspolicyd[521:1e97fa7] [com.apple.syspolicy.exec:default] GK scan complete: PST: (path: 2abffc72afbe3eaa), (team: 68UA947AUU), (id: (null)), (bundle_id: (null)), 8, 4, 0
2026-10-18 03:10:01.648 I  syspolicyd[521:1e981f0] [com.apple.syspolicy.exec:default] App skips first launch prompt because responsibility: <private>, <private>
2026-10-18 03:10:01.648 I  syspolicyd[521:1e981f0] [com.apple.syspolicy.exec:default] Bundle has tamper exception: PST: (path: 2abffc72afbe3eaa), (team: 68UA947AUU), (id: org.blenderfoundation.blender), (bundle_id: org.blenderfoundation.ble
2026-10-18 03:10:01.649 I  syspolicyd[521:1e981f0] [com.apple.launchservices:code-evaluation] present prompt: uid=501, conn=yes, type=BundleIntegrity, op.ident=3EB761AD-77B2-4609-888D-83C41EE3239D, info.ident=81DE2E0F-B596-4766-BF51-C6FD3CF
2026-10-18 03:10:01.649 Df syspolicyd[521:1e981f0] [com.apple.syspolicy.exec:default] Prompt shown (1, 0), waiting for response: PST: (path: 2abffc72afbe3eaa), (team: 68UA947AUU), (id: org.blenderfoundation.blender), (bundle_id: org.blender
```

A search of the same log afterwards, for lines about prompts, found none answering it.

That copy carried the installed application's quarantine mark (`com.apple.quarantine: 0381;…`) and its fourteen added compiled-Python files (see question 10's hand test, where `codesign --verify` on the installed application reports them). So on this Mac a copy of the installed 5.3 started from a terminal waits on a macOS prompt that only a person can answer. No answer was given, and so **which settings folder a second copy of 5.3 works out was not observed**. Whether a preference saved by one copy is then read by the other was not run either (see *Not run* below).

**Not run, and why.** Three things. *Whether a saved preference crosses between two copies of 5.3:* the ordinary settings folder this Mac's Blender works out is the Creator's own, which this sitting may not write to, and pointing both copies at one folder of the runner's own would make them share it by force, so that version could not come out the other way. *The registration arguments, the `Operating System Settings` section and what registering a second Blender does:* the arguments are marked for Windows and Linux only and running them on the Creator's Mac could change which program opens their files, and there was no Linux or Windows machine in this sitting. *The Windows installer's default folder:* no Windows machine.

**Outside the interface:** Registration can be done without the interface: `-r, --register`, `--register-allusers`, `--unregister` and `--unregister-allusers` "then exit (Windows & Linux only)", per the arguments page, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11. The shared settings folder applies to any start, but the sources do not say whether a background run saves preferences. **Observed, 2026-10-18, on the Creator's Mac:** a background start with `--factory-startup`, run from a terminal, reported the ordinary settings folder `/Users/artist/Library/Application Support/Blender/5.3` and looked at its `config` folder (refused by a guard); a second copy of 5.3 started the same way did not get past macOS's own integrity check, which waited on a prompt for a person.

**Operating system:**
- **Observed for macOS only, 2026-10-18:** the help marks all four registration arguments `(Windows & Linux only)`, and the Mac's Preferences System page has no `Operating System Settings` section and no `Register` control. Nothing was run on Linux or Windows for 5.3: there was no machine of either kind in that sitting.
- macOS: the `Blender.app` overwrite applies there.
- Registration and the default-Blender setting: Windows and Linux only.
- The installer's file association: Windows.
- The thumbnail tool on macOS is a Quick Look extension: "The Blender Thumbnail extension can be disabled in the System Settings under “Login Items & Extensions”" — manual, https://docs.blender.org/manual/en/5.3/files/blend/previews.html, read 2026-10-11.
- The settings folder layout applies on all three, with different paths.

---

## 6. Does installing a Blender disturb a render already running on the same computer?

**Answer.** Not in the sources. No page says what installing, unpacking or updating a Blender does to a render that is already running, whether in the same Blender or another.

Facts nearby, which do not settle it:
- Archive and zip installs are self-contained folders.
- The documented macOS update overwrites `Blender.app`.
- Snap, the Microsoft Store and Steam install updates automatically.
- The Windows installer needs administrator rights.

None of these pages mentions a running Blender.

**Label:** not in the sources.

**Not covered:** Everything the question asks. That includes:
- whether overwriting `Blender.app`, or an automatic Snap, Store or Steam update, while that same Blender is rendering stops the render or waits;
- whether installing a different Blender in a separate folder has any effect at all.

**Where I looked:**
- Read every installing page (Linux, macOS, Windows, Steam), the deployment and directory-layout pages, and the startup, crash and recover troubleshooting pages.
- Searched the manual for: "while blender is running", "blender is running", "already running", "another instance", "other instance", "multiple instances", "second instance", "running instance", "close blender", "quit blender", "in use", "while rendering", "during render", locked, "file in use".
- Searched the release notes for install, update, background, command and platform. The only platform note is about the macOS Dock icon in background mode.
- Searched the API guide pages for install.
- Cycles' own files hold nothing about installation.

Nothing was found.

**Observed — hand test, 2026-10-18 (hand-test list entry 7), first run only.** Run on the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83) — with the runner's own settings, temporary and home folders. Another Blender of the Creator's own (5.2.2) was open on the Mac throughout and was not touched.

*Set-up.* The render ran from the installed Blender 5.3.0 Beta at `/Applications/Blender 5.3b.app`, which was only read from. (A throwaway copy could not be used: on this Mac a fresh copy of 5.3 waits on a macOS prompt before it starts — see question 5's hand test. Nothing was copied into or over the installed one.) The scene was Blender's own factory startup scene; no file was opened. An empty output folder and an empty folder for the second Blender were made in a work folder outside the project. The render was started with the window switched off:

`"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" --background --factory-startup --python-expr "import bpy; bpy.context.scene.cycles.samples=64; bpy.context.scene.render.resolution_percentage=50" -E CYCLES -F PNG -o "<work folder>/07/A/out/frame_" -s 1 -e 30 -a`

*Action — a second Blender put on the computer beside the running one.* Fifteen seconds after the render started (a fixed pause), `cp -R "/Applications/Blender 4.5.12.app" "<work folder>/07/A/second/Blender.app"` was run. What came back:

```
### image files on disk before the action: 5 at 03:28:48
### action exit code 0
### action finished at 03:28:53
### image files just after the action: 7
### render process ended, exit code 0 at 03:29:46
### image files at the end: 30
### last file: frame_0030.png
```

The last lines the render printed:

```
01:10.801  render           | Fra: 30 | Mem: 39M | Finished
01:10.827  render           | Saved: '<work folder>/07/A/out/frame_0030.png'
01:10.827  render           | Time: 00:02.31 (Saving: 00:00.02)

Blender quit
```

No line of the render's output contained the word error. All thirty numbered images were written, each 960 by 540, and the line `Sample 64/64` appears thirty times in the output. The second Blender's folder held its program afterwards (800 MB on disk).

**Not run, and why.** *The same Blender copied over the very folder the render is running from.* It needs the render to run from a copy, because the installed Blender may not be written over, and on this Mac a fresh copy of 5.3 waits on a macOS prompt that only a person can answer. *Dragging a new `Blender.app` over the installed one:* that would change the Creator's own installed Blender. *Running a Windows installer, or an automatic Snap, Store or Steam update, during a render:* no Windows or Linux machine and no store account in this sitting.

**Outside the interface:** The sources do not say. **Observed, 2026-10-18, on the Creator's Mac:** the render was a background render started from a terminal with no window, and putting a second Blender into another folder while it ran left it to finish all thirty frames.

**Operating system:** The sources do not say. **Observed for macOS only, 2026-10-18**, as above, and for the second-Blender-beside case only. Nothing was run on Linux or Windows for 5.3: there was no machine of either kind in that sitting.

---

## 7. Can a computer be asked which Blenders it holds?

**Answer.** Only for some forms of install, and only through the operating system or the store that installed them. Blender itself documents no way to list installs.

- **Linux distribution packages:** the package manager "may provide … listing of packages".
- **Windows installer:** removing a version is done through "Windows settings or control panel", so installer-installed versions are listed there. That they appear as a list is my inference from the uninstall instruction.
- **Steam:** Blender appears "in the Library tab of the Steam client".
- **Microsoft Store:** Blender is launched from the Start menu. A list is not described.
- **Zip and archive installs:** for Windows zip files, "No start menu item will be created and no blend-file association will be registered"; for Linux archives, the manual describes extracting to a directory with no system registration described. Nothing records them. Inferred: the system has no list of them, and finding them means searching folders. The directory-layout page gives the folder each form uses.

A running Blender can report its own location (`bpy.app.binary_path`), but not other installs.

**Label:**
- Documented: the package-manager listing, the Windows uninstall route, the Steam Library and the Windows zip-install wording.
- Inferred: that the Windows uninstall route amounts to a list of installed versions, and that zip and archive installs leave no system record.
- Not in the sources: any Blender command that lists installs.

**Exact words and where:**
- "may provide other features (given by the package manager), such as listing of packages, update notifications and automatic menu configuration." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/linux.html, read 2026-10-11.
- "To remove a previously installed version of Blender, use Windows settings or control panel to uninstall the desired version." / "No start menu item will be created and no blend-file association will be registered" / "After installation, Blender can now be launched from the Windows Start menu." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/windows.html, read 2026-10-11.
- "Blender should now be available in the Library tab of the Steam client, where it can be launched." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/steam.html, read 2026-10-11.
- "bpy.app.binary_path — The location of Blender’s executable" — API docs, https://docs.blender.org/api/5.3/bpy.app.html, read 2026-10-11.

**Not covered:**
- Whether the Windows list shows every installer-installed version separately.
- Whether macOS keeps any record beyond the app being in a folder; the sources say nothing.
- Whether Snap lists versions.
- Whether the Linux "Register" step leaves a findable record of which install registered: it writes "A desktop file & icon" under `/usr/local` or `~/.local` (https://docs.blender.org/manual/en/5.3/editors/preferences/system.html), but which install that entry points to is not described.

**Where I looked:**
- Read every installing page, the deployment, directory-layout and system-preferences pages, and `bpy.app`.
- Searched the manual for installation, installations, uninstall, registry, "installed versions", "list", "multiple versions" and register.
- Searched the API docs for install and binary_path.
- The "Import Preferences From Previous Version" feature finds earlier versions' settings folders, not installs, so I ruled it out.

**Observed — hand test, 2026-10-18 (hand-test list entry 1).** Run on the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — against Blender 5.3.0 Beta at `/Applications/Blender 5.3b.app`, from a terminal with no window.

*What Blender itself offers.* `"/Applications/Blender 5.3b.app/Contents/MacOS/Blender" -c help` printed, word for word, with exit code 0:

```
Blender Command Listing:
	asset_listing
	extension
	maketx
```

Nothing else.

**Observed — hand test, 2026-10-18 (hand-test list entry 6).** Same machine. *Set-up.* The Mac held five Blenders in `/Applications` — 4.5.12, 5.0.1, 5.1.2, 5.2.2 and the 5.3.0 Beta — and, at the same time, two copies of the 5.3.0 Beta in ordinary folders of a work folder under `/private/tmp/claude-501/smoozy-hand-tests-5.3/Builds/`, made with `cp -Rc` (one at `…/05/copy/Blender.app`, one at `…/06/Blender.app`). Four ways of asking were tried in a terminal.

`ls -1 /Applications | grep -i blender` printed:

```
Blender 4.5.12.app
Blender 5.0.1.app
Blender 5.1.2.app
Blender 5.3b.app
Blender.app
```

`mdfind "kMDItemFSName == 'Blender*.app'"` printed, exit code 0:

```
/Applications/Blender 5.1.2.app
/Applications/Blender 5.3b.app
/Applications/Blender.app
/Applications/Blender 4.5.12.app
/Applications/Blender 5.0.1.app
```

`system_profiler SPApplicationsDataType` ended with exit code 0 and printed one block per Blender in `/Applications`. The block for the 5.3 Beta, word for word:

```
    Blender 5.3b:

      Version: 5.3.0
      Obtained from: Identified Developer
      Last Modified: 2026-10-16, 04:36
      Kind: iOS
      Signed by: Developer ID Application: Stichting Blender Foundation (68UA947AUU), Developer ID Certification Authority, Apple Root CA
      Location: /Applications/Blender 5.3b.app
      Get Info String: 5.3.0 2026-10-16, Blender Foundation
```

Blocks in the same shape were printed for 4.5.12, 5.0.1, 5.1.2 and 5.2.2 (the last headed `Blender:`, at `/Applications/Blender.app`). The block's `Version` line reads `5.3.0`, with no stage word. None of the three commands named either copy in the work folder: no line of the `system_profiler` output contains the work folder's name. Asking the index about that folder directly, `mdfind -onlyin "<work folder>/06" "kMDItemFSName == 'Blender.app'"`, printed nothing, exit code 0.

*What a running Blender says about where it is.* The installed 5.3 reports `binary_path = '/Applications/Blender 5.3b.app/Contents/MacOS/Blender'` (question 3's hand test). The copy in the work folder was not asked, because on this Mac a fresh copy of the installed 5.3 started from a terminal waits on a macOS prompt for a person (question 5's hand test).

**Not run, and why.** The Linux package-manager listings, the Windows Settings list after two installers, the Microsoft Store and Steam: no Linux or Windows machine and no store account in this sitting.

**Outside the interface:** Package-manager listings are command-line in nature; the sources do not name the command. The Windows Settings list and the Steam Library are interface routes of those programs. **Observed, 2026-10-18, on the Creator's Mac:** Blender's own command listing, `ls`, `mdfind` and `system_profiler` were all run from a terminal with no window.

**Operating system:** Differs by system as above: package-manager listing on Linux, Settings/Control Panel on Windows, Steam on all three. Nothing is said for macOS. **Observed for macOS only, 2026-10-18:** macOS's own `mdfind` and `system_profiler` named the five Blenders in `/Applications` and neither of two copies in an ordinary work folder. Nothing was run on Linux or Windows for 5.3: there was no machine of either kind in that sitting.

---

## 8. What becomes of a render that is running when the Blender it is using is removed?

**Answer.** Not in the sources.

Nearby facts, which do not settle it:
- The system folders "store files that come bundled with Blender and are required for it to function".
- A startup message shows Blender finding its bundled Python, and "If this folder is missing or unable to be found, it is likely that an error will occur". That is about starting, not a render already under way.

**Label:** not in the sources.

**Not covered:** Everything the question asks: whether the render finishes, stops, or fails when it next needs a file (for example a bundled script or data file), and whether the answer differs between deleting a folder, moving `Blender.app` to the Trash, uninstalling through Windows Settings, or a store removing it.

**Exact words and where (the nearby facts only):**
- "System directories store files that come bundled with Blender and are required for it to function. This includes scripts, presets, essential assets and more." — manual, https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html, read 2026-10-11.
- "found bundled python: {DIR} — This message indicates that Blender was able to find the Python library for the Python interpreter embedded within Blender. If this folder is missing or unable to be found, it is likely that an error will occur, and this message will not appear." — manual, https://docs.blender.org/manual/en/5.3/troubleshooting/startup.html, read 2026-10-11.

**Where I looked:**
- Read the installing pages, the directory-layout, startup, crash and recover pages, the command-line render page list, and the arguments page.
- Searched the manual for: uninstall, reinstall, remove, delete with blender/installation/folder/app, "deleted while", "moved while", "missing executable", "while rendering", "during render", "render is running", "in progress".
- Searched the API docs for install and uninstall.
- Searched the release notes for uninstall, remove and install.

Nothing addresses removal during use.

**Not run, and why — 2026-10-18.** The hand test written for this question — a background render started from a throwaway copy of Blender, with that copy deleted part-way through, and again with it moved to another folder part-way through — could not be run on the Creator's Mac (MacBook Pro, Apple M1 Pro with its own 14-core graphics, 16 GB, macOS 26.6.2). The render has to run from a copy, because the installed Blender at `/Applications/Blender 5.3b.app` is the Creator's own and may not be deleted or moved; and on that Mac a fresh copy of the installed 5.3, started from a terminal, waits on a macOS prompt that only a person can answer (recorded under question 5's hand test, with macOS's own log lines). Moving `Blender.app` to the Trash would put files in the Creator's own Trash, and there was no Windows or Linux machine in that sitting for an uninstall or a store removal. So the question stays open for 5.3 on every operating system.

**Outside the interface:** The sources do not say. Untested for 5.3, for the reason above.

**Operating system:** The sources do not say. Untested for 5.3 on every operating system, for the reason above.

---

## 9. Can installing or removing a Blender be done again safely on a computer where it was cut off part-way?

**Answer.** Not in the sources. No page says what an interrupted extraction, installer run, uninstall, Snap, Store or Steam install leaves behind, or whether running it again is safe.

Nearby facts:
- Archive installs are made by extracting "using standard file management tools".
- Windows removal goes through "Windows settings or control panel".
- Linux archives must be extracted with TAR, not 7-zip.

None of these speaks of interruption.

**Label:** not in the sources.

**Not covered:** Everything the question asks: whether a half-extracted folder can be extracted over, whether the Windows installer recovers or repairs, whether a cancelled uninstall leaves a listing behind, and whether Snap, the Store or Steam resume.

**Exact words and where (the nearby facts only):**
- "deployment can be automated using standard file management tools." — manual, https://docs.blender.org/manual/en/5.3/advanced/deploying_blender.html, read 2026-10-11.
- "To remove a previously installed version of Blender, use Windows settings or control panel to uninstall the desired version." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/windows.html, read 2026-10-11.
- "Extracting Blender’s archive using 7-zip is not supported. TAR must be used instead." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/linux.html, read 2026-10-11.

**Where I looked:**
- Read every installing page, the deployment page, and the startup, crash and recover pages.
- Searched the manual for: repair, rollback, "roll back", resume, "try again", "run again", re-run, rerun, interrupted, partial install/download/extract, "incomplete install/download", corrupt, reinstall, re-install, uninstall. The only hits are about undo, add-on and asset re-runs, graphics-driver reinstallation, and reinstalling the operating system ("In more extreme cases, you may want to consider reinstalling your operating system." — https://docs.blender.org/manual/en/5.3/troubleshooting/crash.html). None of them is about installing Blender.
- Searched the API guide pages and the release notes for install and uninstall.

**Observed — hand test, 2026-10-18 (hand-test list entry 9), the copying half only.** Run on the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83). Everything happened in a work folder outside the project; the installed Blender 5.3.0 Beta at `/Applications/Blender 5.3b.app` was only ever read from. That installed application already fails macOS's own signature check, because fourteen compiled-Python files have been added inside it since it was signed (question 10's hand test), so every copy of it below starts from that state. **No copy was started**, for the reason under *Not run* below.

*A whole copy made with `cp -R`.* `cp -R "/Applications/Blender 5.3b.app" "<work folder>/09/cpR/Blender.app"` ended with exit code 0. `codesign --verify --verbose` on the copy ended with exit code 1 and printed first `…/09/cpR/Blender.app: a sealed resource is missing or invalid`, then `file added:` lines naming the same compiled-Python files the installed application carries.

*A copy made with macOS's own `ditto`, cut off after two seconds.* `ditto "/Applications/Blender 5.3b.app" "<work folder>/09/ditto/Blender.app"` was started into an empty folder and stopped with `kill` after a fixed two-second pause. What came back:

```
### copy killed at 03:30:40
### how much arrived: 719M
-rwxr-xr-x@ 1 artist     admin  209683968 Oct 16 05:43 <work folder>/09/ditto/Blender.app/Contents/MacOS/Blender
```

So the program file had arrived whole, at the same size as the installed one. `codesign --verify --verbose` on the half copy ended with exit code 1 and printed:

```
<work folder>/09/ditto/Blender.app: invalid Info.plist (plist or signature have been modified)
In architecture: arm64
```

*The same `ditto` again, over the same folder, nothing deleted first.*

```
### second copy exit code 0
### size now: 948M
```

`codesign --verify --verbose` on the finished copy ended with exit code 1 and printed `…/09/ditto/Blender.app: a sealed resource is missing or invalid`, followed by fifteen `file added:` lines, where the installed application gives fourteen. The first of the fifteen is the extra one:

`file added: <work folder>/09/ditto/Blender.app/Contents/Resources/5.3/python/lib/python3.13/site-packages/Cython/Build/.BC.T_WdSxXu`

`diff -rq` between the installed application and the finished copy printed one line only, `Only in <work folder>/09/ditto/Blender.app/Contents/Resources/5.3/python/lib/python3.13/site-packages/Cython/Build: .BC.T_WdSxXu`. Listed, that file is `-rw-------`, 13,387 bytes, dated the time of the cut-off copy; the file beside it named `Inline.py` is also 13,387 bytes. So the copy that was done again over the half copy holds everything the installed application holds, plus one file left by the copy that was cut off. `du -sk` gave 970,732 kilobytes for the installed application and 970,748 for the finished copy.

**Not run, and why.** *Starting the three copies* — the whole `cp -R` copy, the half `ditto` copy, and the redone one — to see whether each starts and what it prints. On the Creator's Mac a fresh copy of the installed 5.3, started from a terminal, waits on a macOS prompt that only a person can answer (question 5's hand test, with macOS's own log lines), and this sitting answered no prompt. *What is left behind after a removal* (a copy deleted while it renders, then the settings folder looked at): it needs a copy that runs, for the same reason. *The Linux archive cut off part-way and extracted again, and 7-zip:* no Linux machine in that sitting, and no archive, since nothing was downloaded. *The Windows installer and uninstaller cancelled and run again, and Snap, the Store and Steam:* no Windows or Linux machine and no store account.

**Outside the interface:** The sources do not say. **Observed, 2026-10-18, on the Creator's Mac:** the copying, the cut-off and the redo were all done from a terminal with no window. Whether the resulting copies start was not observed.

**Operating system:** The sources do not say. **Observed for macOS only, 2026-10-18,** and for the copying half only, as above. Nothing was run on Linux or Windows for 5.3: there was no machine of either kind in that sitting.

---

## 10. What does a computer need to have on it before a published Blender build of each kind can be unpacked?

The question could mean two things: what is needed to unpack or install the download, or what is needed to start Blender afterwards. I answer the first, which the note under the question points to. The second would need the minimum requirements, which the manual keeps on a page outside my sources.

**Answer.** The manual describes the procedure as the same for every stability kind: "The procedure for installing a binary, either the latest stable release or a daily build, is the same." So what is needed depends on the form of the download, not on Stable, LTS or Daily.

- **Linux archive from blender.org:** must be extracted with TAR; "Extracting Blender’s archive using 7-zip is not supported." Inferred: a working `tar` must be present. The archive's compression type is not stated, so what `tar` must support is not stated.
- **Snap:** snap must already be installed ("Assuming snap is already installed").
- **Linux distribution package:** that distribution's package manager.
- **macOS dmg:** the disk image is mounted by double-clicking and `Blender.app` is dragged to Applications. The sources name no extra tool.
- **Windows zip-file:** you "manually extract Blender to the desired folder". The tool is not named. No administrator rights are needed.
- **Windows Installer File:** "requires administrator rights."
- **Microsoft Store:** the Microsoft Store.
- **Steam:** the Steam client and a Steam account (log in "or create an account").
- **`bpy` module:** "available via PIP". Inferred: a Python with PIP is needed.

Matching the machine: on Windows, the download must match the CPU (x64 or arm64). On Linux, "Download the Linux version for your architecture". On macOS, 5.0 and later support Apple silicon only. These quotes are about which download to pick. They do not say whether the wrong one can still be unpacked.

**Label:**
- Documented: TAR for Linux archives, snap present, administrator rights for the Windows installer, none for the zip, Steam client and account, PIP for the module, and the matching-architecture statements.
- Inferred: that `tar` must be present, that Python with PIP is needed for the module, and that a native package manager or the Microsoft Store app must be present for those respective routes. Each is one step from the quoted words.
- Not in the sources: what tool, if any, is needed for the Windows zip and the macOS dmg, and what `tar` must support.

**Exact words and where:**
- "The procedure for installing a binary, either the latest stable release or a daily build, is the same." / "Blender has a set of minimum and recommended requirements; so make sure these are met before trying to install Blender." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/index.html, read 2026-10-11.
- "Download the Linux version for your architecture and decompress the file to the desired location (e.g. ~/software or /usr/local)." / "Assuming snap is already installed, Blender can be installed through snap with: snap install blender --classic" / "Known Limitations — Archive Extraction — Extracting Blender’s archive using 7-zip is not supported. TAR must be used instead." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/linux.html, read 2026-10-11.
- "Blender 5.0 and later only supports Macs with Apple silicon." / "Blender for macOS is distributed as disk images (dmg-files). To mount the disk image, double-click on the dmg-file. Then drag Blender.app into the Applications folder." / "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/macos.html, read 2026-10-11.
- From the manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/windows.html, read 2026-10-11:
  - "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU’s architecture."
  - "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."
  - "When choosing the zip-file, you have to manually extract Blender to the desired folder" / "but there is also no need for administrator rights."
  - "Blender can be installed from the Microsoft Store by searching for Blender in the Microsoft Store and installing it."
- "Download and install the Steam client for your operating system. Once it is installed, open the client and login to your Steam account, or create an account if you don’t already have one." — manual, https://docs.blender.org/manual/en/5.3/getting_started/installing/steam.html, read 2026-10-11.
- "Blender downloads can be extracted to any directory on the system, as a self contained installation." — manual, https://docs.blender.org/manual/en/5.3/advanced/deploying_blender.html, read 2026-10-11.
- "A pre-compiled bpy module is available via PIP." — API docs, https://docs.blender.org/api/5.3/info_advanced_blender_as_bpy.html, read 2026-10-11.
- For the other reading only (running, not unpacking): "The Blender releases distributed from blender.org include a complete Python installation on all platforms" — API docs, https://docs.blender.org/api/5.3/info_tips_and_tricks.html, read 2026-10-11.

**Not covered:**
- The archive and file formats themselves (for example the Linux archive's compression, or the installer's file type).
- Whether the Windows built-in zip handling, or anything else, is enough or required.
- Whether macOS needs anything beyond its own disk-image handling.
- Disk space needed.
- Whether the Windows installer can run without a person; the sources say nothing on that.
- Whether an Intel Mac or the wrong Windows architecture can still unpack the download.
- The minimum requirements themselves. The manual sends readers to a "Downloading Blender page" for them, which is not in my sources. `Blender Sources/requirements/` holds a requirements page, but it was not among my named sources and I did not open it.

**Where I looked:**
- Read every installing page and the deployment page.
- Searched the manual for: glibc, libc, tar.xz, ".tar", tar, unzip, 7-zip, 7zip, extract, decompress, ".dmg", dmg-file, ".msi", ".msix", administrator, "admin rights", escalated, sudo, privilege, requirement, checksum, signature, notarize, gatekeeper, quarantine, smartscreen, antivirus.
- The only hits that bear on unpacking are those quoted. "Make sure any antivirus software is not preventing Blender from starting." (https://docs.blender.org/manual/en/5.3/troubleshooting/startup.html) is about starting, not unpacking.
- Searched the release notes for install, archive, zip, dmg, tar, requirement and platform: nothing about unpacking. The Vulkan requirement note concerns drawing, not installing.
- The Cycles files' driver-version checks (`properties.py`) concern GPU rendering, not unpacking, so I ruled them out.

**Observed — hand test, 2026-10-18 (hand-test list entry 3).** Run on the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 (build 25G83). **This reads the installed application, not the published disk image.** The application at `/Applications/Blender 5.3b.app` was already on the Mac; how it got there is not recorded in this sitting's inputs. Nothing was started; each command only read it.

*Which processor it is for.* `file` on `Contents/MacOS/Blender` printed `Mach-O 64-bit executable arm64`, and `lipo -archs` printed `arm64` — one processor, Apple silicon. Its information file reads `"LSMinimumSystemVersion" => "13.0"`.

*Who signed it, and whether the signature still checks.* `codesign -dv --verbose=2 "/Applications/Blender 5.3b.app"` printed, word for word:

```
Executable=/Applications/Blender 5.3b.app/Contents/MacOS/Blender
Identifier=org.blenderfoundation.blender
Format=app bundle with Mach-O thin (arm64)
CodeDirectory v=20500 size=1625673 flags=0x10000(runtime) hashes=50791+7 location=embedded
Signature size=9006
Authority=Developer ID Application: Stichting Blender Foundation (68UA947AUU)
Authority=Developer ID Certification Authority
Authority=Apple Root CA
Timestamp=2026-10-16 at 05:43:56
Info.plist entries=19
TeamIdentifier=68UA947AUU
Runtime Version=26.1.0
Sealed Resources version=2 rules=13 files=6621
Internal requirements count=1 size=192
```

`codesign --verify --verbose` on it ended with exit code 1 and printed `/Applications/Blender 5.3b.app: a sealed resource is missing or invalid`, followed by fourteen `file added:` lines. Every one names a compiled-Python cache file inside the application, under `Contents/Resources/5.3/python/lib/python3.13/` or `Contents/Resources/5.3/scripts/` — for example `file added: /Applications/Blender 5.3b.app/Contents/Resources/5.3/python/lib/python3.13/__pycache__/secrets.cpython-313.pyc`. This Blender had been started before the reading, by entries 1 and 2 of these hand tests and by whatever ran it before them.

*What macOS's own assessment says on this Mac.* `spctl -a -vv "/Applications/Blender 5.3b.app"` printed, exit code 0:

```
/Applications/Blender 5.3b.app: accepted
override=security disabled
origin=Developer ID Application: Stichting Blender Foundation (68UA947AUU)
```

Run beside it, `spctl --status` printed `assessments disabled`. **So on the Creator's Mac on that day macOS's own assessment of downloaded programs was switched off**, and this reading says nothing about what a Mac with it switched on would do with this build.

*What marks it carries.* `xattr -l "/Applications/Blender 5.3b.app"` printed:

```
com.apple.macl: 
com.apple.quarantine: 0381;6ad26b5f;Firefox;5A5CC116-16CB-40C2-8E73-A7B37A3ECD73
```

So the installed application carries the mark macOS gives a file downloaded through a web browser, naming Firefox, and that installed Blender started from a terminal while carrying it (in entries 1, 2, 4, 5 and 7 of these hand tests).

*How much room it takes.* `du -sk "/Applications/Blender 5.3b.app"` printed `970732` — 970,732 kilobytes on disk.

**Not run, and why.** The published 5.3 disk image was not opened, mounted or measured: it is not on the Mac and this sitting downloaded nothing. So whether it mounts from a terminal with nobody clicking, what a quarantine mark does to it at a double-click, how big the download is, and whether a package for the wrong processor exists or opens are all untested for 5.3. What a standard (non-administrator) macOS account meets was not tried either: it would need an account made on the Creator's Mac. Nothing was run on Linux or Windows, for want of a machine of either kind; the Linux archive, `tar`, 7-zip, the Windows zip and installer, the Microsoft Store, Steam and Blender's Python module are all untested for 5.3.

**Outside the interface:**
- Linux archive and zip extraction are file operations that tools can do; the deployment page says deployment "can be automated using standard file management tools".
- Snap is a command; PIP is a command-line tool by nature.
- The dmg, Microsoft Store and Steam routes are described only as steps a person takes. The sources do not say whether they can be done without a person.
- The Windows installer's need for administrator rights is stated; running it unattended is not described.
- **Observed, 2026-10-18, on the Creator's Mac:** the installed 5.3 application, carrying a browser's quarantine mark, started from a terminal in every hand test that day that started it, with its window and without, on a Mac whose own assessment of downloaded programs was switched off. Getting the published disk image open without a person was not tried for 5.3.

**Operating system:**
- TAR, Snap and distribution packages: Linux.
- dmg: macOS.
- Zip-file, installer and Microsoft Store: Windows.
- Steam: all three.
- Matching the CPU type is stated for Windows (x64/arm64) and Linux ("for your architecture"). macOS 5.0 and later is Apple silicon only.
- **Observed for macOS only, 2026-10-18:** the installed 5.3 Mac program holds one processor, `arm64`, and names macOS 13.0 as its minimum. Nothing was run on Linux or Windows for 5.3: there was no machine of either kind in that sitting.

---

## Things a person at a computer could settle — pruned 2026-10-18

The hand tests of 2026-10-18 ran on the Creator's Mac what that Mac could reach, against the installed 5.3.0 Beta. What they settled is written into the answers above, marked observed. What is still open is below, with the reason each was not run that day. Two reasons recur. **No Linux or Windows machine was available, and nothing was downloaded.** And **on that Mac a fresh copy of the installed 5.3, started from a terminal, waits on a macOS prompt that only a person can answer** (question 5's hand test), so nothing that needs a second, writable copy of 5.3 to run could be done; on a Mac where a person has approved such a copy, those items can be run.

1. **What a 5.3 daily build and a 5.3 release candidate report about themselves (questions 1 and 3).** Run `--version` and the `bpy.app` readings of question 3's hand test against each. The 5.3.0 Beta is done. Not run: neither build was on the Mac, and nothing was downloaded.

2. **Whether two copies of 5.3 share one settings folder, and whether a preference saved by one is read by the other (question 5).** Ask each copy for `bpy.utils.resource_path('USER')`, then save a preference from one and read it from the other. The installed 5.3 is done; the second copy waits on the macOS prompt, and saving a preference into the ordinary folder would write into the Creator's own settings.

3. **Windows installer, same default folder (questions 5 and 7).** On Windows, run two different Blender installers, keeping the default folder both times. Afterwards, look in `C:\Program Files\Blender Foundation\Blender\` and note what is there. Open Settings → Apps and write down how many Blender entries appear and their names. Not run: no Windows machine.

4. **Installing while rendering (question 6).** A second Blender put into another folder during a render is done, on the Mac. Still open: the same Blender copied over the very folder a render is running from (needs a running copy of 5.3); dragging a new `Blender.app` over the one that is rendering (would change the Creator's own installed Blender); a Windows installer, or an automatic Snap, Store or Steam update, during a render (no machine).

5. **Removing while rendering (question 8).** Start a long command-line render from a copy of Blender and, part-way through, delete the copy, or move it to another folder; on macOS also move it to the Trash, on Windows uninstall it through Settings. Note whether the render continues, stops or prints an error, and which frames were written. Not run: needs a running copy of 5.3; the Trash is the Creator's own; no Windows machine.

6. **Interrupted install and retry (question 9).** The macOS copying half is done: a `ditto` copy cut off after two seconds and done again over the same folder. Still open: whether the half copy and the redone copy start, and what each prints (needs starting a fresh copy of 5.3); Linux — `tar -xf` stopped part-way and run again into the same folder, then `blender --version` (no Linux machine, no archive); Windows — the installer cancelled part-way and run again, and whether Settings → Apps keeps an entry (no Windows machine).

7. **Unpacking needs (question 10).** Linux: extract the archive with `tar -xf`, then with 7-zip, and note any difference. Windows: extract the zip with File Explorer's own "Extract All" as an ordinary (non-administrator) user, then start Blender. macOS: mount the published disk image, both from a terminal and by a person's double-click, and note whether macOS asks anything. Note each download's file name. Not run: no Linux or Windows machine, and the downloads were not on the Mac.

---

2026-10-17 — Claude Opus 5.5 (Claude Code) — Builds, Blender 5.3
