# Builds — Blender 5.2

**Sources consulted, and when.** Blender 5.2 manual (plain-text snapshot plus its `_images` folder), Blender 5.2 Python API documentation, Blender 5.2 release notes, and Cycles' own files as shipped with Blender 5.2 — all from `Blender Sources/`, all taken 2026-07-30, which is the date read for every finding below. I also opened `Blender Sources/requirements/requirements_2026-08-01.txt`; it never names 5.2, so I have not used it as a statement about 5.2 and say so where it came up.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

No helper agents were used. Everything below I read myself.

---

## 1. What kinds of build does Blender publish, and how is each kind named and marked?

**Label: documented** (with one **documented** picture, noted inline).

The question can be read two ways: kinds by *stability track*, or kinds by *package format per operating system*. Both readings are answered below, because the manual's "Download Blender" section answers the first and the per-platform pages answer the second.

### By stability track

The manual's installing page names four packages under the heading "Download Blender":

> "Blender offers a variety of different binary packages to choose from depending on their level of stability. Each package has the trade off of newest features versus stability."

> "**Stable Release** — A package that contains the latest features and is considered stable without regressions. A new stable version is available roughly every three months."

> "**Long-term Support** — A package designed for long-lasting projects requiring a very stable version of Blender. LTS releases are supported for two years and will not have any new features, API changes or improvements. A new long-term support version is available every year. These LTS releases will occasionally have minor patches (such as 4.2.6) which improve stability or fix critical bugs."

> "**Daily Builds** — A package updated daily to include the newest changes in development. These versions are automatically built on a schedule. They are not as thoroughly tested as the release types above, and might break or crash. Builds marked as Alpha are still undergoing major changes and feature additions, while those marked Beta are feature-complete and are under development for refinement and stability."

> "Stability can be expected to increase from Alpha to Beta to Release Candidate (RC) to a final release."

> "**Build from Source** — Blender's source code is available for free to either reference or to build and use."

Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html — read 2026-07-30.

Marking of the LTS track is visible in the naming of 5.2 itself. The manual's own front page is titled:

> "Blender 5.2 LTS Reference Manual"

Source: manual — https://docs.blender.org/manual/en/5.2/index.html — read 2026-07-30.

And the release notes say:

> "Blender 5.2 LTS was released on July 14, 2026."

> "This release includes long-term support and will be maintained until July 2028, see the [LTS page](https://www.blender.org/download/lts/5-2/) for a list of bugfixes included in the latest version."

Source: release notes — https://developer.blender.org/docs/release_notes/5.2/ — read 2026-07-30.

The API lists the build's release status and compilation type:

> "bpy.app.version_cycle — The release status of this build alpha/beta/rc/release"

> "bpy.app.build_type — The type of build (Release, Debug)"

Source: Python API — https://docs.blender.org/api/5.2/bpy.app.html — read 2026-07-30.

And it is marked on screen. The manual's splash-screen page says:

> "The upper part of the splash screen contains the splash image with the Blender version in the top right."

Source: manual — https://docs.blender.org/manual/en/5.2/interface/window_system/splash.html — read 2026-07-30.

I opened the picture that page shows, `_images/interface_splash_current.webp`. **Documented.** What is visible in it: the text `5.0.0 Alpha` in the top right corner of the splash image. That is a three-number version followed by the stability word.

### By package, per operating system

- Windows: "Download the zip-file or Windows Installer File." and "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture." Also "Blender can be installed from the Microsoft Store". Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html — read 2026-07-30.
- macOS: "Blender for macOS is distributed as disk images (dmg-files)." and "Blender 5.0 and later only supports Macs with Apple silicon." Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/macos.html — read 2026-07-30.
- Linux: "Download the Linux version for your architecture and decompress the file to the desired location". Also a distribution package, and Snap: "snap install blender --classic". Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html — read 2026-07-30.
- Steam, on all three: "Blender can be downloaded and updated using the Steam client by following the steps described below on Linux, macOS, or Windows." Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/steam.html — read 2026-07-30.

**Not covered.** The sources do not give the file-name pattern of a published build, any download address, any checksum or signature scheme, or how a daily build is labelled in its own file name. They do not say whether a "Release Candidate" is published as its own downloadable package or is a phase of the daily builds. They do not say which of the four tracks 5.2 itself was published under beyond the LTS marking above. The word "LTS" does not appear anywhere in the API documentation, so the sources do not show LTS status being exposed as a value inside a build.

**Observed — hand test, 2026-08-04.** Everything in this block was run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The build was Blender 5.2.0, installed at `/Applications/Blender.app`. This settles the last "Not covered" item above — whether LTS status is exposed as a value inside a build.

*What the build prints when asked its version.* In a terminal, with nothing else set up:

`"/Applications/Blender.app/Contents/MacOS/Blender" --version`

It printed, word for word:

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

Three further lines followed — `build c flags:`, `build c++ flags:` and `build link flags:`, each very long — and then `	build system: CMake`. The command exited straight away, with exit code 0.

*What the build reports about itself in Python, with no window open.* In a terminal:

`"/Applications/Blender.app/Contents/MacOS/Blender" --background --python-expr "import bpy; print('BEGIN'); [print(a, '=', repr(getattr(bpy.app, a))) for a in sorted(dir(bpy.app)) if not a.startswith('_')]; print('TYPE version_cycle', type(bpy.app.version_cycle)); print('TYPE build_type', type(bpy.app.build_type)); print('END')"`

It printed every name and value on `bpy.app` — 71 entries in all. The lines bearing on this question, word for word:

> ```
> build_branch = b'blender-v5.2-release'
> build_commit_date = b'2026-07-13'
> build_commit_time = b'15:20'
> build_commit_timestamp = 1783956011
> build_date = b'2026-07-14'
> build_hash = b'fbe6228777e7'
> build_platform = b'Darwin'
> build_system = b'CMake'
> build_time = b'01:31:22'
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

So on this build the letters LTS are carried by `version_string`, and by the first line of `--version`. `version_cycle` reads `release`, one of the four documented values, and carries no LTS marking. Searching the whole printed list of names and values for the letters LTS found them only inside `version_string`.

*What the installed application file itself carries.* In a terminal:

`file "/Applications/Blender.app/Contents/MacOS/Blender"` printed:

> `/Applications/Blender.app/Contents/MacOS/Blender: Mach-O 64-bit executable arm64`

`lipo -archs "/Applications/Blender.app/Contents/MacOS/Blender"` printed:

> `arm64`

`plutil -p "/Applications/Blender.app/Contents/Info.plist"` printed, among other lines:

> ```
>   "CFBundleIconName" => "blender_liquid_glass"
>   "CFBundleIdentifier" => "org.blenderfoundation.blender"
>   "CFBundleName" => "Blender"
>   "CFBundleShortVersionString" => "5.2.0"
>   "CFBundleVersion" => "5.2.0"
>   "LSMinimumSystemVersion" => "11.2"
> ```

No LTS marking appears in the bundle's own information file.

**Observed — hand test, 2026-08-05.** The Creator ran this one, on their own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. They opened Blender 5.2.0 from the Applications folder in the ordinary way and sent screenshots of what was on screen. This settles what the splash-screen marking reads on a finished LTS release — the manual's picture of it is of a 5.0 alpha, as noted above.

*The splash screen.* The top right corner of the splash image reads:

> `5.2.0 LTS`

So on this finished LTS release the corner carries the three-number version followed by the LTS marking, where the manual's picture of the 5.0 alpha carried `Alpha` after the numbers.

*The version at the right-hand end of the bar along the bottom of the window.* It reads:

> `5.2.0`

with a small globe icon and a vertical divider to its left. No LTS marking there.

**Observed — hand tests, 2026-08-06, on a second machine.** Everything in this block was run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. It is a container sharing somebody else's hardware, and other work was running on it at the same time, so nothing here says how long anything took. The build was Blender 5.2.0, extracted from Blender's own Linux archive at `/opt/blender/blender-5.2.0-linux-x64`. These are a second machine's results and sit beside the Mac's above; they are never merged with them. **This settles the first "Not covered" item — the file names published builds carry, the download addresses, and whether a checksum scheme exists.**

*What a published build file is called, and what is published beside it.* The 5.2 release folder on Blender's own download server was read in a terminal on 2026-08-06. It lists:

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

So a name is the word blender, the three-number version, the operating system, the processor, and the file kind. **No stability word and no LTS marking appears in any file name**, though 5.2 is an LTS release — so the marking that rides on `version_string` and on the splash corner does not ride on the file name. There is one macOS file and it is for Apple silicon, which matches the manual's "Blender 5.0 and later only supports Macs with Apple silicon". Windows gets three file kinds, not the two the manual names — an `.msix` sits beside the `.msi` and the `.zip`.

A checksum scheme **does** exist. `blender-5.2.0.sha256` was fetched and reads, word for word:

> ```
> ed4d8390166dec5ea0a2813a03db6221f206ce016442be7f59f41d760972568a  blender-5.2.0-macos-arm64.dmg
> 96f6c181a30f4950607839dc84d42a354b250d8a0231b098b59b7bc69c351c48  blender-5.2.0-linux-x64.tar.xz
> 8ba59aed79dfc35d65c5e9e0ccc828d618796499483ccdeee0b534a915140606  blender-5.2.0-windows-x64.msi
> b9e0be792b54ed4e7fa77f3c458df53899108170834e9c92859e7fea440eb6a6  blender-5.2.0-windows-x64.msix
> 2d184b626c001692c362291911293b6a297179d618d95e9e9192c3a80318adc4  blender-5.2.0-windows-x64.zip
> 754aef3b2b79f418191dd0a2d427429d7297c52a70c22d965ec45d70eed754cd  blender-5.2.0-windows-arm64.msi
> 1435548dac0aee4926e6fea1bb20ddd0b02bc6534b72769c6c5eb6a45f9c4064  blender-5.2.0-windows-arm64.msix
> c00b6a5d80456d9299865ab4db730b7c93b0af4ae77d707548f886692786d881  blender-5.2.0-windows-arm64.zip
> ```

The Linux archive was fetched and checked against it: `sha256sum -c` printed `blender-5.2.0-linux-x64.tar.xz: OK`. The archive itself is `XZ compressed data, checksum CRC64` by `file`'s reckoning.

*What the build prints when asked its version.* `/opt/blender/blender-5.2.0-linux-x64/blender --version` printed, word for word:

> ```
> Blender 5.2.0 LTS
> 	build date: 2026-07-14
> 	build time: 01:32:04
> 	build commit date: 2026-07-13
> 	build commit time: 15:20
> 	build hash: fbe6228777e7
> 	build branch: blender-v5.2-release
> 	build platform: Linux
> 	build type: Release
> ```

Three flag lines followed — the last, `build link flags:`, is empty on this build — then `build system: CMake`. Exit code 0. The LTS marking is on the first line, as on the Mac. The commit date, commit time, hash and branch are the Mac's; the build time is not, and `build platform` reads `Linux` where the Mac read `Darwin`.

*What the build reports about itself in Python, with no window open.* The lines bearing on this question:

> ```
> build_branch = b'blender-v5.2-release'
> build_commit_date = b'2026-07-13'
> build_commit_time = b'15:20'
> build_commit_timestamp = 1783956011
> build_date = b'2026-07-14'
> build_hash = b'fbe6228777e7'
> build_platform = b'Linux'
> build_system = b'CMake'
> build_time = b'01:32:04'
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

Seventy-one names in all, the same count as the Mac build. `portable` reads `True` here where the Mac read `False`, and no folder named `portable` exists in the extracted build — that was checked for all four Linux builds and none has one.

*What the build file itself is.* `file /opt/blender/blender-5.2.0-linux-x64/blender` printed:

> `/opt/blender/blender-5.2.0-linux-x64/blender: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=8f25b4c4f0598492bb183b092f48482cf40bc804, for GNU/Linux 3.2.0, stripped`

and `readelf -h` reported `Class: ELF64`, `Type: EXEC (Executable file)`, `Machine: Advanced Micro Devices X86-64`. One architecture, as on the Mac, though a different one. There is no application bundle on Linux and so no bundle information file; what sits beside the program in the extracted folder is `blender-launcher`, `blender-thumbnailer`, `blender-system-info.sh`, `blender.desktop`, `blender.svg`, `blender-symbolic.svg`, `copyright.txt`, `license`, `readme.html`, `lib`, and a folder named `5.2`.

*What the splash screen and the bottom bar show on Linux.* Blender 5.2.0 was opened with a window on the machine's own display and photographed with Blender's own screenshot command. The top right corner of the splash image reads `5.2.0 LTS`. The right-hand end of the bottom bar reads `5.2.0`, with no LTS marking. Both the same as the Creator's Mac.

**Outside the interface. Observed on two machines.** The track names and the packaging are properties of what is published, so they do not depend on how Blender is driven. The one part that is interface-only is the splash-screen marking: it is drawn in Blender's window, and it was looked at with a window open on 2026-08-05, as recorded above. The API gives the release status and compilation type as values, and question 3 documents `-v, --version`; all of those were read above with `--background` and no window open, including the LTS marking on `version_string`.

**Operating system. Observed for macOS and for Linux.** The four stability tracks are described without any operating-system condition. The packaging differs by operating system, exactly as quoted above; and the sources state a hard limit for macOS: "Blender 5.0 and later only supports Macs with Apple silicon." Observed on the Creator's Apple Silicon Mac: the installed 5.2.0 macOS build holds one processor architecture, `arm64`. **Observed on the rented Linux machine, 2026-08-06:** the 5.2.0 Linux build is a single-architecture x86-64 executable, reports `build platform: Linux`, and reads `portable = True` where the Mac read `False`; the splash corner and the bottom bar read the same as on the Mac. The release folder on Blender's own download server publishes one file per operating system and processor, and for macOS there is one and it is `blender-5.2.0-macos-arm64.dmg` — which is the manual's Apple-silicon-only statement borne out by what is actually published. Windows remains untested, though the file names show what is offered for it.

---

## 2. Can builds of each kind be listed and fetched by something other than a person at a web page?

**Label: not in the sources**, for a general listing-and-fetching mechanism. Three partial mechanisms *are* documented, and they are quoted below; none of them is a way to list what exists.

What the sources do document:

> "Assuming snap is already installed, Blender can be installed through snap with: `snap install blender --classic`"

> "Installing Blender via the distribution's native mechanisms ensures consistency with other packages on the system and may provide other features (given by the package manager), such as listing of packages, update notifications and automatic menu configuration. Be aware, though, that the package may be outdated compared to the latest official release, or not include some features of Blender."

Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html — read 2026-07-30.

> "When an update for Blender is available on the Microsoft Store, it will be downloaded and installed automatically."

Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html — read 2026-07-30.

> "When an update for Blender is available on Steam, Steam will automatically download and apply the update for you."

Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/steam.html — read 2026-07-30.

And, on the subject of automating this:

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html — read 2026-07-30.

That last sentence says deployment can be automated once you have the archive. It does not say how the archive is obtained.

There is also this, which cuts the other way:

> "Blender is designed to not require an internet connection, so it doesn't have a built-in update system. This means you will need to update Blender yourself by following the platform-specific upgrade steps described in the sections below."

Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html — read 2026-07-30.

**Where I looked, to be able to say "not in the sources".** I searched the whole of the 5.2 manual, the whole of the 5.2 API documentation and the whole of the 5.2 release notes for: `builder.blender.org`, `download.blender.org`, `blender.org/download`, `wget`, `curl`, `checksum`, `sha256`, `mirror`, `winget`, `chocolatey`, `flatpak`, `apt-get`, `snap`, `automated deployment`, `nightly`, `daily build`, `release candidate`. The only hits for a downloads address were `https://www.blender.org/download/releases/5-2/` and `https://www.blender.org/download/lts/5-2/` in the release-notes index, and `https://www.blender.org/download/demo-files/` in the Virtual Reality release notes — all human-facing pages, none of them an index of builds. The only `curl` and `sha256` hits were in the extensions documentation (`https://docs.blender.org/manual/en/5.2/advanced/extensions/creating_repository/dynamic_repository.html` and `.../static_repository.html`), which describes a listing service for *extensions*, not for Blender builds. The manual's installing page repeatedly refers the reader to a "Downloading Blender page"; that page is on blender.org and is not part of the snapshot, so I cannot say what it offers. I read all five pages under `getting_started/installing/` and the deployment page in full.

**Not covered.** Whether blender.org publishes a machine-readable index of releases or daily builds; whether daily builds have stable addresses; whether Snap, Steam, the Microsoft Store or any distribution package carries the daily/alpha/beta tracks at all (the sources only ever mention them carrying "Blender"); and whether a specific older version can be requested through any of them.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. The build was Blender 5.2.0 at `/Applications/Blender.app`.

Blender's own `-c help` lists the commands a build offers. In a terminal:

`"/Applications/Blender.app/Contents/MacOS/Blender" -c help`

The listing it printed, word for word:

> ```
> Blender Command Listing:
> 	asset_listing
> 	extension
> 	maketx
> ```

Nothing else. (The run also printed messages from add-ons the Creator has installed; none of that is part of the listing.) The whole `--help` output, 518 lines, was also read, and no argument in it fetches or lists a Blender build. So this build offers three commands of its own, and none of them fetches, lists or installs a Blender.

**Observed — hand tests, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. This is where the Snap and distribution-package routes quoted above could finally be exercised. **This settles the "not in the sources" label at the head of this answer for one route: a listing of published Blender builds that a machine can read does exist, and Snap serves it.**

*What `snap info blender` lists.* Its publisher line reads `publisher: Blender Foundation (blenderfoundation**)`. Under `channels:` it printed about a hundred and fifty lines. The opening, word for word:

> ```
> channels:
>   latest/stable:     5.2.0   2026-07-14 (7599) 426MB classic
>   latest/candidate:  5.1.2   2026-06-04 (7528) 436MB classic
>   latest/beta:       ^                               
>   latest/edge:       5.3.0   2026-08-05 (7676) 432MB classic
>   5.2lts/stable:     5.2.0   2026-07-14 (7599) 426MB classic
>   5.2lts/candidate:  5.2.0   2026-08-05 (7675) 426MB classic
>   5.2lts/beta:       ^                               
>   5.2lts/edge:       ^                               
>   5.1/stable:        5.1.2   2026-05-19 (7480) 436MB classic
> ```

Every release line back to 2.79 has a channel, named for the line, with the letters `lts` in the name where the line is a long-term-support one — `5.2lts`, `4.5lts`, `4.2lts`, `3.6lts`, `3.3lts`, `2.93lts`, `2.83lts`. Each has four forms: stable, candidate, beta and edge. Each line carries a version number, a date, a revision number and a size. **This answers three of the four things the "Not covered" note above lists**: builds are reachable at stable addresses through this route, the unstable tracks are carried here (a `latest/edge` holding 5.3.0 and a `4.5lts/beta` holding 4.5.0), and a specific older version can be requested by naming its channel.

*What `snap install blender --classic` delivers.* Run with no channel named, it printed:

> `blender 5.2.0 from Blender Foundation (blenderfoundation**) installed`

with exit code 0. `snap list blender` printed:

> ```
> Name     Version  Rev   Tracking       Publisher            Notes
> blender  5.2.0    7599  latest/stable  blenderfoundation**  classic
> ```

What arrived is Blender 5.2.0 — this version. `/snap/bin/blender --version` printed a block whose first line is `Blender 5.2.0 LTS`, with `build date: 2026-07-14`, `build time: 01:32:04`, `build commit date: 2026-07-13`, `build commit time: 15:20`, `build hash: fbe6228777e7`, `build branch: blender-v5.2-release`, `build platform: Linux`, `build type: Release` — **every one of them the same as the blender.org build extracted under `/opt/blender`**. With no window, it reported:

> ```
> SNAP version_string 5.2.0 LTS
> SNAP version_cycle release
> SNAP build_branch b'blender-v5.2-release'
> SNAP build_hash b'fbe6228777e7'
> SNAP build_date b'2026-07-14'
> SNAP build_platform b'Linux'
> SNAP portable True
> SNAP binary_path /snap/blender/7599/blender
> SNAP USER /root/.config/blender/5.2
> SNAP LOCAL /snap/blender/7599/5.2
> SNAP SYSTEM ''
> ```

Its `build_options` is the same as the blender.org build's, character for character. So **the Snap route delivers the same build as blender.org's own Linux archive**, and nothing it reports about itself says which route it came by, except where it sits. Its own folder holds 1.2 GB and one extra thing the blender.org archive has not: a `blender-wrapper`, a `meta` folder and a `usr` folder.

*Whether that route updates itself, which the manual says it does.* `snap refresh --time` printed:

> ```
> timer: 00:00~24:00/4
> last: today at 03:51 UTC
> next: today at 11:11 UTC
> ```

A repeating timer with a next time already set. So the manual's "updates to Blender are automatically installed" is borne out by a schedule the machine actually holds.

*What a distribution's own package carries.* `apt-cache policy blender` printed:

> ```
> blender:
>   Installed: (none)
>   Candidate: 3.0.1+dfsg-7
>   Version table:
>      3.0.1+dfsg-7 500
>         500 http://archive.ubuntu.com/ubuntu jammy/universe amd64 Packages
> ```

The manual's warning that a distribution package "may be outdated compared to the latest official release, or not include some features of Blender" is borne out by more than two whole release series: 3.0.1 against 5.2.0. `apt-cache show blender` names its file as `blender_3.0.1+dfsg-7_amd64.deb` and carries a `SHA256` line. Its dependency list names about forty system libraries including `libpython3.10`, so it is a different kind of build from a blender.org one, which brings its own Python. It was downloaded and read but **not installed**: `dpkg -c` shows it would put a program at `./usr/bin/blender`, a path this machine already uses for another Blender, and other work was running on the machine at the time. The whole package holds twelve files.

**Outside the interface. Observed on two machines.** `snap install blender --classic` is a command and needs no window at all — and that is now observed rather than read: on the rented Linux machine `snap info blender`, `snap install blender --classic` and every `apt` command above were run from a terminal with no window at any point. Steam and the Microsoft Store are described only as client applications a person operates; the sources do not describe driving either from a script. Extraction of a downloaded archive is explicitly automatable ("deployment can be automated using standard file management tools"). The `-c help` run above was made from a terminal with no window, and none of the three commands it named is about builds.

**Operating system. Observed for macOS and for Linux.** Snap and distribution packages are documented for Linux only. The Microsoft Store is Windows only. Steam is documented for "Linux, macOS, or Windows". No fetching mechanism other than a person downloading is documented for macOS. **Observed on the Creator's Mac:** the build's own commands and arguments fetch nothing. **Observed on the rented Linux machine, 2026-08-06:** the Snap and distribution-package routes both exist there and both work as quoted, Snap carries a machine-readable listing of every published release line and delivers the same build blender.org publishes, and Snap refreshes itself on a timer. So the answer is now sharply different on the two systems: Linux has a listing and an automatic fetch route, and macOS has neither. Windows, Steam and the Microsoft Store remain untested on both.

---

## 3. Can an installed Blender be asked what it is — which version, and which kind of build?

**Label: documented**, for version and for the alpha/beta/rc/release status. **Not in the sources**, for LTS status specifically.

From the command line:

> "-v, --version — Print Blender version and exit."

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

From inside Blender's window, the Blender menu in the Topbar has an "About Blender" entry:

> "Opens a menu displaying the following information about Blender:"
> "Version: The Blender version."
> "Date: Date when Blender was compiled."
> "Hash: The Git Hash of the build. This can be useful to give to support personnel when diagnosing a problem."
> "Branch: Optional branch name."
> "Windowing Environment: On Linux, this will show either Wayland or X11 depending on the windowing environment that Blender is running on."

Source: manual — https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html — read 2026-07-30.

From Python, the same facts and more are readable as values:

> "bpy.app.version — The Blender version as a tuple of 3 numbers (major, minor, micro). eg. (4, 3, 1)"

> "bpy.app.version_string — The Blender version formatted as a string"

> "bpy.app.version_cycle — The release status of this build alpha/beta/rc/release"

> "bpy.app.version_file — The Blender File version, as a tuple of 3 numbers (major, minor, file sub-version), that will be used to save a .blend file."

> "bpy.app.build_hash — The commit hash this Blender instance was built with"

> "bpy.app.build_branch — The branch this blender instance was built from"

> "bpy.app.build_date — The date this Blender instance was built"

> "bpy.app.build_commit_date — The date of the commit this Blender instance was built from"

> "bpy.app.build_commit_timestamp — The unix timestamp of the commit this Blender instance was built from"

> "bpy.app.build_platform — The platform this blender instance was built for"

> "bpy.app.build_system — Build system used"

> "bpy.app.build_type — The type of build (Release, Debug)"

> "bpy.app.binary_path — The location of Blender's executable, useful for utilities that open new instances. Read-only unless Blender is built as a Python module - in this case the value is an empty string which script authors may point to a Blender binary."

Source: Python API — https://docs.blender.org/api/5.2/bpy.app.html — read 2026-07-30.

The module page states these do not change while Blender runs:

> "This module contains application values that remain unchanged during runtime."

Same source and date.

**Not covered.** The sources do not show the exact text `--version` prints, so I cannot say whether that text includes the build hash, the build date, or the alpha/beta/rc word — only that it prints "Blender version". No source shows an installed Blender reporting that it is an LTS release: the word "LTS" appears in the manual's page titles and in the release notes, but nowhere in the API documentation as a readable value, and `version_cycle` lists only "alpha/beta/rc/release". Nothing here identifies which *package* (zip, installer, Store, Steam, Snap, distribution) a given installation came from, except indirectly through where its files sit (see question 7). `build_type` is documented as "(Release, Debug)" — that is a compilation type, not the stability track.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The build was Blender 5.2.0 at `/Applications/Blender.app`. This answers three of the gaps above: what `--version` prints, whether an installed build reports that it is LTS, and what form `version_string` takes.

*From the command line.* `"/Applications/Blender.app/Contents/MacOS/Blender" --version` printed the whole block quoted under question 1, whose first line is `Blender 5.2.0 LTS`, followed by the build date and time, the commit date and time, the commit hash, the branch, the build platform, the build type, three long flag lines, and the build system. So the printed text carries the hash, the branch, the dates and the letters LTS. It carries no alpha/beta/rc word; the `build type: Release` line is the compilation type.

*From Python, with no window open.* The `--background --python-expr` command quoted under question 1 returned every `bpy.app` value. The ones this question asks about, word for word:

> ```
> background = True
> binary_path = '/Applications/Blender.app/Contents/MacOS/Blender'
> build_platform = b'Darwin'
> build_system = b'CMake'
> build_type = b'Release'
> module = False
> version = (5, 2, 0)
> version_cycle = 'release'
> version_file = (5, 2, 44)
> version_string = '5.2.0 LTS'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

So `version_string` does carry the LTS marking, and it is the only place in the whole `bpy.app` listing where the letters LTS appear. `version_cycle` reads `release`. `build_type` comes back as bytes, not text.

*As a saved file, with no window open.* In a terminal:

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

Three long build-flag lines follow, then `build system: CMake`. The next section, headed `Python:`, opens `version: 3.13.13 (main, Apr 25 2025, 12:39:20) [Clang 21.0.0 (clang-2100.0.123.102)]`. So the system-information file carries the LTS marking too, in its title and on its `version:` line.

*What the arguments list itself says.* `--help` printed 518 lines. The version entry, word for word:

> ```
> -v or --version 
> 	Print Blender version and exit.
> ```

with no platform note beside it, while `/?` on the same page prints `Print this help text and exit (Windows only).` and `--qos <level>` prints `Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only).`

**Observed — hand test, 2026-08-05.** The Creator ran this one, on their own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. They opened Blender 5.2.0 from the Applications folder in the ordinary way and sent screenshots of what was on screen.

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

Its right-hand column is a set of links: `Donate`, `What's New`, then `Credits`, `License`, `Blender Store`, `Blender Website`. Four of the five rows the manual documents are present — Version, Date, Hash and Branch. The fifth, `Windowing Environment`, which the manual says appears "On Linux", does not appear on this Mac. The LTS marking is on the Version line. The panel shows no build time, no build platform, no build type and no build system, so it carries fewer fields than `--version` prints.

*The splash screen and the bottom bar.* The top right corner of the splash image reads `5.2.0 LTS`. The right-hand end of the bar along the bottom of the window reads `5.2.0`, with no LTS marking.

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

**Observed — hand test, 2026-08-05 — what this build was built with.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The build was Blender 5.2.0 at `/Applications/Blender.app`. The work was done in a fresh folder outside the project, and Blender was pointed at a settings folder of its own with the `BLENDER_USER_RESOURCES` environment variable — that folder was made before Blender was started, so Blender was never pointed at a folder that did not exist.

The API documentation prints a value for one more `bpy.app` name, `build_options`, which is not among the ones quoted above. This test reads what this build reports for it, and sets the two side by side. The command typed, with the scratch folder written out as `<a folder outside the project>`:

`BLENDER_USER_RESOURCES="<a folder outside the project>/15/settings" "/Applications/Blender.app/Contents/MacOS/Blender" --background --python-expr "import bpy; print(bpy.app.build_options)"`

What came back, word for word:

> ```
> bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=True, jack=False, pulseaudio=False, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, potrace=True, pugixml=True, haru=True, experimental_features=False)
> Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
> 
> Blender quit
> ```

The command ended with exit code 0. It was run twice, and printed the same value both times.

The value the documentation prints, word for word:

> "Constant value bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=False, jack=True, pulseaudio=True, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, potrace=True, pugixml=True, haru=True, experimental_features=False)"

Source: Python API — https://docs.blender.org/api/5.2/bpy.app.html — read 2026-07-30. That quote is taken as it stands under question 3 of `Preparation/Blender Research/Findings/5.2/Builds — second pass.md`, where it was quoted from the same page and date, and where the citation check checked it against that copy word for word on 2026-08-05.

*The two set side by side.* Both carry the same forty-four names, in the same order, and no name is present in one and absent from the other. **They do not match.** Three of the forty-four names carry a different value:

- `coreaudio` — documented `False`, this build `True`.
- `jack` — documented `True`, this build `False`.
- `pulseaudio` — documented `True`, this build `False`.

The other forty-one names read the same in both. So the value printed in the documentation is not what this 5.2 build reports.

**Observed — hand tests, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. The build was Blender 5.2.0 at `/opt/blender/blender-5.2.0-linux-x64`. Every route above was tried again on this second machine, and one of them comes out differently.

*From the command line.* `--version` printed the block quoted under question 1 of this document's Linux block. Its first line is `Blender 5.2.0 LTS`, so the printed text carries the hash, the branch, the dates and the letters LTS here as on the Mac. It carries no alpha/beta/rc word.

*What the arguments list itself says.* `--help` printed **525 lines**, against 518 on the Mac. The version entry, word for word:

> ```
> -v or --version 
> 	Print Blender version and exit.
> ```

with no platform note, while `/?` prints `Print this help text and exit (Windows only).` and `--qos <level>` prints `Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only).` The four registration arguments are all printed, each marked `(Windows & Linux only)`.

*From Python, with no window open.* The `--background --python-expr` dump returned every value. The ones this question asks about:

> ```
> background = True
> binary_path = '/opt/blender/blender-5.2.0-linux-x64/blender'
> build_platform = b'Linux'
> build_system = b'CMake'
> build_type = b'Release'
> cachedir = '/root/.cache/blender/'
> module = False
> portable = True
> tempdir = '/tmp/blender_mGC5PA/'
> version = (5, 2, 0)
> version_cycle = 'release'
> version_file = (5, 2, 44)
> version_string = '5.2.0 LTS'
> TYPE version_cycle <class 'str'>
> TYPE build_type <class 'bytes'>
> ```

Seventy-one names in all, the same count as the Mac. Searching the whole printed listing for the letters LTS found them only inside `version_string`, as on the Mac.

*What Blender itself offers as commands.* `-c help` printed:

> ```
> Blender Command Listing:
> 	asset_listing
> 	extension
> 	maketx
> ```

the same three as the Mac, and none about builds.

*As a saved file, with no window open.* In a terminal:

`/opt/blender/blender-5.2.0-linux-x64/blender --background --python-expr "import bpy; bpy.ops.wm.sysinfo(filepath='<a folder outside the project>/system-info.txt')"`

Blender printed one warning of its own first — `cycles | WARNING HIPEW initialization failed: Error opening HIP dynamic library`, which is this machine having no AMD graphics support — then `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)` and `Blender quit`. The file was written. Its opening and Blender section, word for word:

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

Three build-flag lines follow — the third, `build linkflags:`, is empty on this build — then `build system: CMake`. The `Python:` section opens `version: 3.13.13 (main, Apr 25 2025, 12:39:20) [GCC 14.2.1 20250110 (Red Hat 14.2.1-11)]`. The letters LTS appear twice, in the title and on the `version:` line, as on the Mac. The Python version number is the Mac's; what compiled it is not.

*What this build was built with, set beside the value the documentation prints.* This is the same test the Mac ran, on the same version, and **it comes out the other way**. In a terminal:

`/opt/blender/blender-5.2.0-linux-x64/blender --background --python-expr "import bpy; print(bpy.app.build_options)"`

What came back, word for word:

> ```
> bpy.app.build_options(bullet=True, codec_avi=False, codec_ffmpeg=True, codec_sndfile=True, compositor_cpu=True, cycles=True, cycles_osl=True, freestyle=True, image_cineon=True, image_dds=True, image_hdr=True, image_openexr=True, image_openjpeg=True, image_tiff=True, image_webp=True, input_ndof=True, audaspace=True, international=True, openal=True, opensubdiv=True, sdl=False, coreaudio=False, jack=True, pulseaudio=True, wasapi=False, libmv=True, mod_oceansim=True, mod_remesh=True, io_wavefront_obj=True, io_ply=True, io_stl=True, io_fbx=True, io_gpencil=True, opencolorio=True, openmp=False, openvdb=True, alembic=True, usd=True, fluid=True, xr_openxr=True, potrace=True, pugixml=True, haru=True, experimental_features=False)
> ```

That was written to a file and set character by character against the value the 5.2 Python API documentation prints, quoted above. **They are identical — every character.** Where the Creator's Mac build differs from the documentation in three names, this Linux build differs in none. The three names in question read `coreaudio=False`, `jack=True` and `pulseaudio=True` here, which is exactly what the documentation prints and the opposite of what the Mac build prints. So the value the documentation carries is a Linux build's value, and reading it as a description of any 5.2 build would be wrong on a Mac and right here.

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

**This settles the one row the manual documents and no machine had ever shown.** The manual says `Windowing Environment` appears "On Linux", and here it does, reading `X11`. On the Creator's Mac that row is absent, so all five documented rows are present here and four are present there. The LTS marking is on the Version line, as on the Mac.

*The splash screen and the bottom bar.* The top right of the splash image reads `5.2.0 LTS`; the right-hand end of the bottom bar reads `5.2.0`, with no LTS marking.

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

*What a build from a different route reports.* A Blender 5.2.0 was also installed on this machine through Snap. It reports the same version string with the LTS marking, the same `version_cycle`, the same branch, the same hash `fbe6228777e7`, the same build date and time, and the same `build_options`. The only thing that tells the two apart is where they sit: `binary_path` reads `/snap/blender/7599/blender` for one and `/opt/blender/blender-5.2.0-linux-x64/blender` for the other. **So a Snap build and a blender.org build of the same version report nothing different about themselves** — which is half of the "Not covered" item above about telling published kinds apart. Steam and the Microsoft Store are still out of reach.

**Outside the interface. Observed on two machines.** `-v, --version` is a command-line argument that prints and exits. The quoted API page gives `bpy.app` values but does not say how they are read from outside Blender's window; the test above answers that — with `--background` and no window at any point, every value listed came back and `bpy.ops.wm.sysinfo` wrote its file, and `build_options` came back the same way on 2026-08-05. The "About Blender" menu is the only route here that requires Blender's window, and it was looked at with a window open on 2026-08-05, as recorded above.

**Operating system. Observed for macOS and for Linux.** *(Citation check of the Linux hand tests, 2026-08-06: this line still read "Observed for macOS" although a Linux result had been written into it below. Corrected to name both machines.)* No operating-system condition is stated for `--version` or for any `bpy.app` value quoted; on the Creator's Mac every one of them returned a value, and `--version` ran and printed as quoted above. `build_platform` read `b'Darwin'`. The only per-system remark in the About Blender list is the "Windowing Environment" line, which the manual says appears "On Linux", and on the Creator's Mac that row is absent from the panel, as the screenshot shows. The command-line pages give a different path to the executable per system: `./blender` in the install directory on Linux, `/Applications/Blender.app/Contents/MacOS/Blender` on macOS, `C:\Program Files\Blender Foundation\Blender\blender.exe` on Windows when installed via the installer — https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/index.html and its three sub-pages, read 2026-07-30. The macOS path was the one used there, and it worked as documented. **Observed on the rented Linux machine, 2026-08-06:** the Linux path worked as documented too. Four things differ between the two machines: `build_platform` reads `b'Darwin'` on the Mac and `b'Linux'` on the rented machine; `portable` reads `False` on the Mac and `True` on the rented machine; the help output is 518 lines on the Mac and 525 on the rented machine; and the About Blender panel's `Windowing Environment` row is absent on the Mac and reads `X11` on the rented machine. And the `build_options` list is where they part most sharply: the Mac build's differs from the documented constant in three names, and the Linux build's is identical to it, character for character. Windows remains untested.

---

## 4. Does Blender name one release as the one to use for ordinary work?

**Label: documented**, for a production recommendation. The recommendation names a *kind* of release, not a single release.

The deployment page, whose subject is "setting up Blender in environments such as animation studios and schools", says:

> "New Blender versions may add, remove or change functionality that affects the results of production files. For a given project, it is advisable to use a single LTS version of Blender. LTS versions receive bug fixes for two years."

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html — read 2026-07-30.

The installing page declines to name one for everybody:

> "The package that is right for you depends on your requirements for those two. A studio, for example, might want to have long-term support, while a hobbyist may want newer features, while others may just want to test upcoming features. Each package described below has something just right for everyone."

Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html — read 2026-07-30.

So: the sources name LTS as the kind to use for a project, and 5.2 is itself marked LTS ("Blender 5.2 LTS was released on July 14, 2026." — release notes, https://developer.blender.org/docs/release_notes/5.2/, read 2026-07-30). The sources do not point at one specific release number as "the one".

**Not covered.** Whether blender.org's download page presents one release as the default (that page is not in the snapshot). Whether, at any moment, more than one LTS version is current and which of them is meant. What "a single LTS version" implies when a project outlives an LTS window. Whether the advice changes for machines that only render and do not author.

**Observed — hand test, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. This does not answer the question, and is recorded only because it is the nearest thing to a default any machine could be made to show. `snap install blender --classic` — the one command-line fetch route the manual documents — was run with no channel named, and printed:

> `blender 5.2.0 from Blender Foundation (blenderfoundation**) installed`

`snap list blender` then showed it tracking `latest/stable`. **That is Snap's own default, not Blender naming a release.** It does bear on the first "Not covered" item, about whether more than one LTS version is current at once: `snap info blender` names a channel per release line and puts the letters `lts` in the name only where the line is one, and on 2026-08-06 there were `5.2lts`, `4.5lts`, `4.2lts`, `3.6lts`, `3.3lts`, `2.93lts` and `2.83lts` channels all carrying builds. So several LTS lines are served at once, and this route does not pick between them either — it defaults to the newest stable release of any kind.

**Outside the interface.** The advice is about which version to install; it says nothing that depends on whether Blender is driven by a person or by a script, and nothing in the quoted text limits it either way.

**Operating system.** No operating-system condition is stated on either quote.

---

## 5. Can one Blender installed on a computer cause trouble for another that is already there?

**Labels: documented, inferred, and not in the sources.** Co-existence and the per-version separation are documented; the same-version case is inferred; outright interference is not in the sources.

Documented — co-existence is intended:

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html — read 2026-07-30.

> "When using this method of installation, it is possible to have multiple versions of Blender installed."

Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html — read 2026-07-30.

> "Note, you do not have to overwrite your existing Blender installation. It's perfectly possible to have multiple versions installed side by side."

Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html — read 2026-07-30.

Documented — the separation is by version number, in both the user and the system directories:

> "$HOME/.config/blender/5.2/" (Linux) · "/Users/$USER/Library/Application Support/Blender/5.2/" (macOS) · "%USERPROFILE%\AppData\Roaming\Blender Foundation\Blender\5.2\" (Windows)

> System directories: "./5.2/" for the Linux archive and the Windows zip; "./Blender.app/Contents/Resources/5.2/" on macOS; "%ProgramFiles%\Blender Foundation\Blender\5.2\" for the Windows installer; "%ProgramFiles%\WindowsApps\BlenderFoundation.Blender<HASH>\Blender\5.2\" for the Microsoft Store installation.

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html — read 2026-07-30.

And the defaults page states the consequence plainly:

> "The preferences need to be imported from previous versions because the configuration files of each Blender version are stored in separate folders."

Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/configuration/defaults.html — read 2026-07-30.

Documented — a cache-directory path without a version number, and what the temporary directory holds. On the same directory-layout page, the cache directory is given without a version number:

> "Linux: `$XDG_CACHE_HOME/blender/` if `$XDG_CACHE_HOME` is set, otherwise `$HOME/.cache/blender/`" · "macOS: `/Library/Caches/Blender/`" · "Windows: `%USERPROFILE%\AppData\Local\Blender Foundation\Blender\Cache\`"

The page also says:

> "The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs)."

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html — read 2026-07-30.

Documented — installing can replace an existing installation, on macOS by the documented procedure:

> "Install the new version by overwriting the current Blender.app in the Applications folder. You can rename Blender.app or place it in a different folder to have more than one version at a time."

Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/macos.html — read 2026-07-30.

Documented — the Windows installer associates blend files with Blender, and registration commands exist:

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html — read 2026-07-30.

> "-r, --register — Register blend-file extension for current user, then exit (Windows & Linux only)." · "--register-allusers — Register blend-file extension for all users, then exit (Windows & Linux only)." · "--unregister — Unregister blend-file extension for current user, then exit (Windows & Linux only)." · "--unregister-allusers — Unregister blend-file extension for all users, then exit (Windows & Linux only)."

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

Documented — environment variables that replace default resource directories:

> "BLENDER_USER_RESOURCES: Replace default directory of all user files. Other BLENDER_USER_* variables override when set." · "BLENDER_SYSTEM_RESOURCES: Replace default directory of all bundled resource files." · "BLENDER_SYSTEM_SCRIPTS: Directories to add extra scripts."

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

**Inferred**, for two same-version installations. The step: the user directory is documented as being named for the version — `…/Blender/5.2/` — and nothing in the layout distinguishes one 5.2 installation from another 5.2 installation. Therefore two separately installed copies of 5.2 on one machine would use the same documented user-directory path. The sources never say this in so many words; the reader has to join "the folder is named by version" to "two installations can share a version". Treat it as inference, not fact.

**Inferred**, for a Windows installer over an existing same-version install. The step: the installer's documented target is `%ProgramFiles%\Blender Foundation\Blender\5.2\`, which contains the version number but nothing else; installing 5.2 twice would therefore target the same folder. The manual does not describe what the installer does when that folder is already populated.

**Not in the sources.** No page states that installing or running one Blender can break another already on the machine. There is no discussion of shared libraries, of a Blender putting anything on the system PATH by itself, of registry keys beyond the file association, or of two installations competing for a GPU or a licence. The one documented "trouble" of any kind between versions concerns files rather than installations: "loss of feature should always be expected when opening a blend-file saved with a more recent version of Blender" (manual — https://docs.blender.org/manual/en/5.2/files/blend/compatibility.html — read 2026-07-30).

**Where I looked.** The deployment page, the directory-layout page, all five installing pages, the defaults page, the command-line arguments page in full, and the blend-file compatibility page. I searched the whole manual, API and release notes for `another instance`, `already running`, `single instance`, `multiple instances`, `side by side`, `co-exist`, `multiple versions`, `in use by`, `conflict`. The only hits that bore on this are the ones quoted above.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. This settles the first inference above — that two separately installed copies of 5.2 would use the same user-directory path.

*Set-up.* A second copy of the installed application was made in a working folder outside `/Applications`, with `cp -Rc "/Applications/Blender.app" <working folder>/Blender.app`. That gave two copies of Blender 5.2.0 on one computer, in two different places.

*Action.* This was run against each of the two, changing only the path:

`"<path to the copy>/Contents/MacOS/Blender" --background --python-expr "import bpy; print('USER', bpy.utils.resource_path('USER')); print('LOCAL', bpy.utils.resource_path('LOCAL')); print('SYSTEM', bpy.utils.resource_path('SYSTEM')); print('BINARY', bpy.app.binary_path)"`

*What came back.* From the one in `/Applications`:

> ```
> USER /Users/artist/Library/Application Support/Blender/5.2
> LOCAL /Applications/Blender.app/Contents/Resources/5.2
> SYSTEM /Library/Application Support/Blender/5.2
> BINARY /Applications/Blender.app/Contents/MacOS/Blender
> ```

From the second copy, in the working folder:

> ```
> USER /Users/artist/Library/Application Support/Blender/5.2
> LOCAL <working folder>/Blender.app/Contents/Resources/5.2
> SYSTEM /Library/Application Support/Blender/5.2
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

**Observed — hand test, 2026-08-05.** The Creator ran this one, on their own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. They opened Blender 5.2.0 in the ordinary way, opened Edit ‣ Preferences, clicked System in the list down the left-hand side, and sent a screenshot of that page.

The screenshot shows the System page with five sections, in this order: `Cycles Render Devices`, `Network`, `Memory & Limits`, `Video Sequencer` and `Sound`. No section headed `Operating System Settings` appears in it, and no button named `Register`, `Unregister` or `For All Users` appears in it. One screenshot was supplied, so what it shows is the page from `Cycles Render Devices` at the top down to the last row of `Sound`; whether anything sits below `Sound` is not shown.

**Observed — hand tests, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH, with all four researched versions extracted under `/opt/blender`. The registration arguments quoted above are marked "Windows & Linux only", so this is the first machine on which they could be used at all. It is a rented machine that is thrown away, which is why changing a file-type association was acceptable here and was not on the Creator's own computer. Everything that changed is written out, and everything created was removed afterwards.

**This settles the "Not in the sources" statement at the head of this answer for one case: installing and registering one Blender does change what another already on the machine does, and the way it does so is by writing over its file.**

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

*Registering a second Blender afterwards.* `/opt/blender/blender-5.2.0-linux-x64/blender --register` printed the same shape of output with its own path on the first line — and one word differently: this build writes `not creating symbolic-links` where 4.5.12 writes `not creating symlinks`. **No new file appeared** — the set of files was identical afterwards. What changed is the content of the one file both wrote:

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

That build had never registered anything, and it removed the registration another build had made. `--unregister` clears whatever registration is there, not only its own. `--unregister-allusers` did the same for the all-users side, thumbnailer included.

*What unregistering leaves behind.* Set against the file list taken at the start, unregistering did **not** put the machine back. Twelve rebuilt mime-database files were left in `~/.local/share/mime` and twelve more in `/usr/local/share/mime`; the folders `applications`, `icons`, `mime` and `thumbnailers` were left in place though empty; and `~/.config/mimeapps.list` was left still naming `blender.desktop`. All of it was removed by hand afterwards, and the machine's file list then matched the earlier one exactly.

*Whether a Preferences panel of these controls exists on Linux.* All four Linux builds were asked, with no window, to list the preferences panels they register; all four register a panel labelled `Operating System Settings`. Opened with a window and photographed, Blender 5.2.0's Preferences System page shows, in order: `Cycles Render Devices`, `Display Graphics`, `Operating System Settings`, `Network`, `Memory & Limits`, and more below. The `Operating System Settings` section reads `Open blend files with this Blender version` above two buttons, `Register` and `Unregister`, with a checkbox `For All Users` beneath. **On the Creator's Mac neither that section nor `Display Graphics` appears.**

*Two builds of one version, in two places.* This settles the first inference above on a second machine. A second copy of this build was made with `cp -a` into a working folder. Run with `BLENDER_USER_RESOURCES` deliberately unset and with `--factory-startup`, so that the default path came back and nothing was saved, the two printed:

> ```
> USER /root/.config/blender/5.2
> LOCAL /opt/blender/blender-5.2.0-linux-x64/5.2
> SYSTEM ''
> BINARY /opt/blender/blender-5.2.0-linux-x64/blender
> ```

> ```
> USER /root/.config/blender/5.2
> LOCAL /root/smoozy/Builds/work/06/5.2/copy/5.2
> SYSTEM ''
> BINARY /root/smoozy/Builds/work/06/5.2/copy/blender
> ```

The same user path, only the build's own folder following the copy, and the path is the Linux one the layout page documents. One difference from the Mac: the system path comes back as an **empty string** here, where the Mac returned a real path.

*Two builds of one version from two different sources — which is the stronger version of the same inference.* A Blender 5.2.0 was also installed through Snap on this machine, alongside the blender.org 5.2.0 extracted under `/opt/blender`. The Snap build reports:

> ```
> SNAP USER /root/.config/blender/5.2
> SNAP LOCAL /snap/blender/7599/5.2
> SNAP SYSTEM ''
> SNAP binary_path /snap/blender/7599/blender
> ```

The same user path as the blender.org build. **So two Blenders of the same two-number version, put on one machine by two entirely different routes, share one settings folder.** That is the inference above, observed, and with two genuinely separate installations rather than two copies of one.

*What the cache folder does.* The answer above records that the cache directory path carries no version number. On this machine `bpy.app.cachedir` read `/root/.cache/blender/` from both the 5.1 and the 5.2 builds — one folder, no version in it, shared between versions.

*What the Snap install did and did not take.* It did not take the blend-file association: `xdg-mime query default application/x-blender` was unchanged afterwards. It added `/snap/bin/blender` and a desktop file of its own at `/var/lib/snapd/desktop/applications/blender_blender.desktop`, and left `/usr/bin/blender` alone.

**Outside the interface. Observed on two machines.** `--register` and related arguments are command-line-only. The sources do not say whether the directory layout or resource-directory variables differ when Blender is started from outside its window; the runs above were made with `--background` and no window, and the documented per-version user path is what came back. The "Import Preferences From Previous Version" choice is interface-bound: the defaults page says it appears in the splash screen when Blender is started for the first time or updated to a new version. The sources do not say what happens instead when Blender is driven from outside its window.

**Operating system. Observed in part for macOS and for Linux.** *(Citation check of the Linux hand tests, 2026-08-06: this line still opened "Observed in part for macOS" although a Linux result had been written into it below. Corrected to name both machines.)* The separation-by-version scheme is documented for all three, with a different path each. The overwrite-the-app update is macOS. The installer, the Start-menu entry and the administrator-rights requirement are Windows. `--register` and `--unregister` are stated as "Windows & Linux only", and the Mac build's own `--help` marks all four the same way; the screenshot of the Preferences System page on the Creator's Mac shows no `Operating System Settings` section among the five sections it covers. **Observed on the rented Linux machine, 2026-08-06:** all four registration arguments run there and do what they say, the `Operating System Settings` section is on the Preferences System page with `Register`, `Unregister` and `For All Users` in it, and **registering a second Blender does take the association from the first, by writing over its file**. The two machines therefore differ sharply, in the way the manual's own platform note predicts. The distribution-package route is Linux only, and was reached on the rented machine — see question 2. Windows remains untested.

---

## 6. Does installing a Blender disturb a render already running on the same computer?

**Label: not in the sources.**

Nothing in the 5.2 manual, the 5.2 API documentation or the 5.2 release notes describes what happens to a render in progress while a Blender is being installed on the same machine. The installing pages describe downloading, extracting, running an installer, and updating; none of them mentions a running Blender, a running render, or any need to close anything first.

**Where I looked.** I read in full: `getting_started/installing/index.txt`, `linux.txt`, `macos.txt`, `windows.txt`, `steam.txt` (https://docs.blender.org/manual/en/5.2/getting_started/installing/…), `advanced/deploying_blender.txt`, `advanced/blender_directory_layout.txt`, `advanced/command_line/render.txt` and `advanced/command_line/arguments.txt`. I searched the whole of the manual, the API documentation and the release notes for: `already running`, `another instance`, `single instance`, `multiple instances`, `concurrently`, `at the same time`, `in use by`, `interrupted`, `uninstall`, `remove blender`. The `uninstall` hits were all about graphics drivers, extensions and studio lights; none about Blender itself apart from the Windows update note quoted in question 5. The self-contained nature of an extracted installation ("Blender downloads can be extracted to any directory on the system, as a self contained installation" — https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html) is the nearest the sources come, and it does not answer the question: it says where files go, not what happens to a process while they are being written.

**Not covered by the sources.** Everything about the question, in effect. In particular the sources do not say whether a render reads files from its installation directory after it has started, which is what the answer would turn on.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The installed Blender in `/Applications` was never touched; both runs used a throwaway copy in a working folder outside the project.

*Set-up, for both runs.* A copy of Blender 5.2.0 was made into an empty working folder with `cp -Rc`, with an empty output folder beside it. The scene was Blender's own factory startup scene — no file was opened. The render was started from the copy, with the window switched off:

`"<working folder>/Blender.app/Contents/MacOS/Blender" --background --factory-startup --python-expr "import bpy; bpy.context.scene.cycles.samples=64; bpy.context.scene.render.resolution_percentage=50" -E CYCLES -F PNG -o "<working folder>/out/frame_" -s 1 -e 30 -a`

*First run — a second Blender put on the computer beside the running one.* Once four image files had appeared, `cp -R "/Applications/Blender 4.5.12.app" "<working folder>/second/Blender.app"` was run in another terminal. It finished with exit code 0.

> ```
> ### image files on disk before the action: 4 at 21:25:26
> ### action exit code 0
> ### action finished at 21:25:30
> ### image files just after the action: 5
> ### render process ended, exit code 0 at 21:26:40
> ### image files at the end: 30
> ```

All thirty numbered images were written, ending `frame_0028.png`, `frame_0029.png`, `frame_0030.png`. The render printed no error.

*Second run — the same Blender copied over the exact folder the render was running from.* Same set-up, a fresh working folder. Once four image files had appeared, `cp -R "/Applications/Blender.app/" "<working folder>/Blender.app/"` was run — the contents of the installed 5.2.0 copied straight over the folder the render was reading from. It finished with exit code 0.

> ```
> ### image files on disk before the action: 4 at 21:31:20
> ### action exit code 0
> ### action finished at 21:31:25
> ### image files just after the action: 6
> ### render process ended, exit code 0 at 21:32:23
> ### image files at the end: 30
> ```

All thirty numbered images were written. The render printed no error.

**Observed — hand tests, 2026-08-06, on the rented Linux machine. The two machines disagree on the second run, and neither result is set aside.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH, shared with other work at the time, so nothing here says how long anything took. Nothing under `/opt/blender` was touched; both runs used a throwaway copy made with `cp -a`. **This answers the "Not covered" note above, which says the question turns on whether a render reads files from its installation directory after it has started: on this machine it plainly does.**

*Set-up, for both runs.* A copy of the extracted Blender 5.2.0 into an empty working folder, an empty output folder beside it, Blender's own factory startup scene, and the render started with no window:

`<working folder>/blender/blender --background --factory-startup --python-expr "import bpy; bpy.context.scene.cycles.samples=64; bpy.context.scene.render.resolution_percentage=50" -E CYCLES -F PNG -o "<working folder>/out/frame_" -s 1 -e 30 -a`

*First run — a second Blender put on the computer beside the running one.* Once four image files had appeared, `cp -a /opt/blender/blender-4.5.12-linux-x64 <working folder>/second/blender` was run:

> ```
> ### image files on disk before the action: 4
> ### the copy's own exit code: 0
> ### is the second Blender there? /root/smoozy/Builds/work/render2/5.2/A/second/blender
> ### image files just after the action: 5
> ### render process ended, exit code 0
> ### image files at the end: 30
> ```

Its last lines were `01:16.681  render | Fra: 30 | Mem: 38M | Finished`, `Saved: '<working folder>/out/frame_0030.png'`, `Time: 00:02.46 (Saving: 00:00.04)` and `Blender quit`. No line contained the word error. **This agrees with the Mac.**

*Second run — the same Blender copied over the exact folder the render was running from.* Once four image files had appeared, `cp -a /opt/blender/blender-5.2.0-linux-x64/. <working folder>/blender/` was run. The copy finished with exit code 0. What came back:

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
> 00:10.946  render           | Fra: 5 | Mem: 1M | Updating Baking
> 00:10.946  render           | Fra: 5 | Mem: 1M | Updating Device | Writing constant memory
> 00:10.946  render           | Fra: 5 | Mem: 1M | Loading denoising kernels (may take a few minutes the first time)
> 00:10.946  render           | Fra: 5 | Mem: 1M | Sample 0/64
> 00:10.975  render           | Fra: 5 | Remaining: 00:01.82 | Mem: 38M | Sample 1/64
> ```

An ordinary progress line for frame 5, cut off mid-frame. Blender printed no error of its own; the words `Bus error (core dumped)` came from the shell that started it. Four images were on disk at the end instead of thirty, and the four written before the action were still there. **The test was run a second time, on its own, and came out the same way — exit code 135, four frames of thirty.**

**The same test on the Creator's Mac finished all thirty frames with exit code 0 and printed no error. On this machine it stopped the render, in this version and in all three others.** Both results stand; neither is chosen between.

**Outside the interface. Observed on two machines.** The sources do not say, and the silence is complete for both a person-driven render and a script-driven one. Every run on both machines was a background render started from a terminal with no window.

**Operating system. Observed for macOS and for Linux, and they do not agree.** The sources do not say. The three operating systems have visibly different install mechanisms (in-place overwrite of `Blender.app` on macOS, an administrator-rights installer or a plain zip on Windows, an archive or a package manager on Linux), so silence here is silence about three different situations, not one — and the two that were tried do come out differently. On macOS 26.5.2 with an Apple M1 Pro chip, both runs completed all thirty frames. On the rented Linux machine, putting a second Blender beside the running one left the render untouched, and copying a Blender over the folder the render was running from stopped the render with exit code 135 and four frames of thirty written. Windows remains untested, and the installer could not be tried on either computer; the package-manager routes were reached on the rented machine and are recorded under question 2.

---

## 7. Can a computer be asked which Blenders it holds?

**Label: not in the sources**, for any Blender-provided way to enumerate the installations on a machine. Three documented partials follow, none of which is that.

Documented partial 1 — a distribution's package manager may list its own packages:

> "Installing Blender via the distribution's native mechanisms ensures consistency with other packages on the system and may provide other features (given by the package manager), such as listing of packages, update notifications and automatic menu configuration."

Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html — read 2026-07-30. This covers only installations made that way, on Linux.

Documented partial 2 — the manual gives default install locations:

> Linux archive from blender.org: "./5.2/" · Linux distribution packages: "/usr/share/blender/5.2/" · macOS: "./Blender.app/Contents/Resources/5.2/" · Windows zip: "./5.2/" · Windows installer: "%ProgramFiles%\Blender Foundation\Blender\5.2\" · Microsoft Store: "%ProgramFiles%\WindowsApps\BlenderFoundation.Blender<HASH>\Blender\5.2\"

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html — read 2026-07-30.

Documented partial 3 — a *running* Blender can say where it is:

> "bpy.app.binary_path — The location of Blender's executable, useful for utilities that open new instances."

Source: Python API — https://docs.blender.org/api/5.2/bpy.app.html — read 2026-07-30. That answers "where is this one", not "which ones are here".

**Inferred**, and weakly: because installations from blender.org are self-contained folders that "can be extracted to any directory on the system" (https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html), an installation may sit anywhere at all, so a search of the documented default locations cannot be complete. The step: "any directory" plus "no registry of installations is documented" gives "no exhaustive list is obtainable from the documented locations alone". The sources do not state this conclusion.

**Where I looked.** The directory-layout page in full, all five installing pages in full, the three command-line launch pages in full, the deployment page in full, and `bpy.app` in the API documentation in full. I searched the whole manual, API and release notes for `uninstall`, `installed versions`, `list of versions`, `multiple versions`, `side by side`, `registry`, `PATH`. Nothing describes an inventory mechanism.

**Not covered.** Whether the Windows installer records itself where Windows' own "installed programs" list can be read; whether the Microsoft Store, Steam or Snap can be queried for Blender specifically; whether Blender writes anything outside its own folder that would betray its presence. On the last point, note that user configuration folders are per-version (`…/Blender/5.2/`, above) and are created for a version that has been *run*, not merely installed — the sources do not say when they are created, so I cannot offer them as a proxy for an installation list.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2.

*What Blender itself offers.* `"/Applications/Blender.app/Contents/MacOS/Blender" -c help` printed:

> ```
> Blender Command Listing:
> 	asset_listing
> 	extension
> 	maketx
> ```

None of the three enumerates Blender installations.

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

`system_profiler SPApplicationsDataType` printed one block per Blender. The block for the 5.2 installation, word for word:

> ```
>     Blender:
> 
>       Version: 5.2.0
>       Obtained from: Identified Developer
>       Last Modified: 2026-07-14, 03:31
>       Kind: iOS
>       Signed by: Developer ID Application: Stichting Blender Foundation (68UA947AUU), Developer ID Certification Authority, Apple Root CA
>       Location: /Applications/Blender.app
>       Get Info String: 5.2.0 2026-07-14, Blender Foundation
> ```

Blocks in the same shape were printed for 4.5.12, 5.0.1 and 5.1.2. None of the three commands named any of the four copies sitting in the working folder. Asking the index about that folder directly, `mdfind -onlyin <working folder> "kMDItemFSName == 'Blender.app'"`, returned nothing at all.

*What a running Blender says about where it is.* `bpy.app.binary_path` read `'/Applications/Blender.app/Contents/MacOS/Blender'`, and the system-information file under question 3 carries the same path on its `binary path:` line. Both report the one Blender that produced them.

**Observed — hand test, 2026-08-06, on the rented Linux machine. This settles the Linux half of the question**, which documented partial 1 above only pointed at, and it bears out the weak inference too. Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH.

*Set-up.* The machine held **five** Blenders when this was run: the four researched versions extracted under `/opt/blender` and a fifth that came with the rented image at `/opt/blender-5.0.0-linux-x64`, with `/usr/bin/blender` pointing at it. None had been installed by a package manager, and none sat in the documented default place for a distribution package.

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

So the documented partial is real but as narrow as its own wording: **a package manager names only what it installed**, and none of these five was installed that way. The inference above — that a search of the documented default locations cannot be complete, because a build may sit anywhere — is borne out: none of these five sits in `/usr/share/blender/5.2/` or beside a package manager's records, and all five were reachable only by searching the whole file system.

*After a package-managed Blender was added.* A Blender 5.2.0 was installed with `snap install blender --classic`. `snap list` then named it, and only it:

> ```
> blender  5.2.0  7599  latest/stable  blenderfoundation**  classic
> ```

`apt list --installed` still named nothing. `which -a blender` then printed three paths — `/usr/bin/blender`, `/bin/blender` and `/snap/bin/blender` — and the snap's own desktop file appeared at `/var/lib/snapd/desktop/applications/blender_blender.desktop`. The listing grew by exactly the one build the package manager installed, and the other five stayed invisible to it. So **a package-manager route does leave a record a machine can read, but only of its own installations** — which answers half of the "Not covered" note above.

*What a running Blender says about where it is.* `bpy.app.binary_path` read `'/opt/blender/blender-5.2.0-linux-x64/blender'` for the extracted build and `'/snap/blender/7599/blender'` for the snap one, and the system-information file under question 3 carries the same path on its `binary path:` line. Both report the one Blender that produced them, exactly as on the Mac.

**Outside the interface. Observed on two machines.** The package-manager quote does not say how the package manager is queried; on the rented Linux machine it was queried from a terminal with no window, as was everything else above. `bpy.app.binary_path` identifies "this Blender instance", and the quoted API page does not say how it is read from outside Blender's window; it was read here with `--background` and no window. Every listing command above was also run from a terminal with no window open.

**Operating system. Observed for macOS and for Linux.** The package-manager route is Linux only. The install locations differ per operating system, as quoted. `bpy.app.binary_path` is documented without any operating-system condition, and returned a value on both machines. The three listing commands recorded above are macOS's own, not Blender's. **Observed on the rented Linux machine, 2026-08-06:** the documented Linux route works, and it names only what it installed — `apt` and `snap` named none of the five Blenders on the machine until one was put there through Snap, and then named exactly that one. The two machines agree on the shape of the answer: the operating system's own tools name what was put there their way, and nothing else. Windows remains untested.

---

## 8. What becomes of a render that is running when the Blender it is using is removed?

**Label: not in the sources.**

No page in the 5.2 manual, the 5.2 API documentation or the 5.2 release notes describes removing Blender while it is running, or the fate of a render in progress when its installation is deleted. The manual's only statement about removing Blender at all is the Windows update note:

> "To remove a previously installed version of Blender, use Windows settings or control panel to uninstall the desired version."

Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html — read 2026-07-30. It says nothing about a running Blender.

**Where I looked.** I searched the whole of the manual, the API documentation and the release notes for `uninstall`, `remove blender`, `removing blender`, `delete`, `already running`, `another instance`, `in use by`, `interrupted`. Every `uninstall` hit outside the Windows page above was about graphics drivers (the troubleshooting/gpu pages), about extensions (`bpy.ops.extensions.package_uninstall`, https://docs.blender.org/api/5.2/bpy.ops.extensions.html) or about studio lights (`bpy.ops.preferences.studiolight_uninstall`, https://docs.blender.org/api/5.2/bpy.ops.preferences.html) — different subjects entirely. I read the command-line rendering page (https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html) in full; it describes how to start a render from the command line and says nothing about stopping one or about the installation being removed underneath it.

**Not covered by the sources.** All of it. Notably, the sources never say whether Blender keeps reading from its installation folder during a render (scripts, datafiles, the bundled Python are all documented as living in the system directories — https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html — but nothing says when they are read), and that is what the answer would depend on. I will not guess. The deletion case was tested by hand, below.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no other graphics card. The installed Blender in `/Applications` was never touched; the test used a throwaway copy in a working folder outside the project, and nothing outside that folder was deleted.

*Set-up.* A copy of Blender 5.2.0 was made into an empty working folder with `cp -Rc`, with an empty output folder beside it. The scene was Blender's own factory startup scene. The render was started from the copy, with the window switched off:

`"<working folder>/Blender.app/Contents/MacOS/Blender" --background --factory-startup --python-expr "import bpy; bpy.context.scene.cycles.samples=64; bpy.context.scene.render.resolution_percentage=50" -E CYCLES -F PNG -o "<working folder>/out/frame_" -s 1 -e 30 -a`

*Action.* Once four image files had appeared, `rm -rf "<working folder>/Blender.app"` was run in another terminal — the whole application the render was running from, deleted.

*What came back.*

> ```
> ### image files on disk before the action: 4 at 21:36:52
> ### action exit code 0
> ### does the folder still exist? ls: <working folder>/Blender.app: No such file or directory
> ### action finished at 21:36:53
> ### image files just after the action: 4
> ### render process ended, exit code 0 at 21:37:55
> ### image files at the end: 30
> ```

The deletion was permitted and completed at once; the application folder was gone. The render carried on and wrote all thirty numbered images, then ended by itself with exit code 0. It printed no error. Its own log's closing lines, word for word:

> ```
> 01:14.323  render           | Fra: 30 | Mem: 38M | Finished
> 01:14.351  render           | Saved: '<working folder>/out/frame_0030.png'
> 01:14.351  render           | Time: 00:02.43 (Saving: 00:00.02)
> 
> Blender quit
> ```

The four images written before the deletion were still there afterwards, and the remaining twenty-six were written after it.

*What was left afterwards.* `ls ~/Library/Application\ Support/Blender/5.2` printed `config`, `extensions`, `scripts` — the settings folder for this version survived the application being deleted.

**Observed — hand test, 2026-08-06, on the rented Linux machine.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Nothing under `/opt/blender` was touched; the test used a throwaway copy made with `cp -a`, and nothing outside the working folder was deleted.

*Set-up and action.* As on the Mac: a copy of the extracted Blender 5.2.0, an empty output folder, Blender's own factory startup scene, and the same thirty-frame background render command quoted under question 6 of this document. Once four image files had appeared, `rm -rf <working folder>/blender` was run — the whole folder the render was running from, deleted.

*What came back.*

> ```
> ### image files on disk before the action: 4
> ### action: the whole Blender folder the render is running from, deleted; exit code 0
> ### does the folder still exist? ls: cannot access '/root/smoozy/Builds/work/render/5.2/C/blender': No such file or directory
> ### image files just after the action: 4
> ### render process ended, exit code 0
> ### image files at the end: 30
> ### last file: frame_0030.png
> ```

The last lines the render printed:

> ```
> 01:16.325  render           | Fra: 30 | Mem: 38M | Finished
> 01:16.408  render           | Saved: '<working folder>/out/frame_0030.png'
> 01:16.408  render           | Time: 00:02.52 (Saving: 00:00.04)
> 
> Blender quit
> ```

No line contained the word error. That version's settings folder was still there afterwards and still held its contents.

**This agrees with the Creator's Mac**, in this version and in the other three: the deletion is permitted, the render carries on, all thirty frames are written, the frames written before the deletion survive, and the settings folder outlives the program.

**Worth setting beside question 6, because the two look alike and are not.** Deleting the folder outright did not disturb the render on either machine. Copying a Blender over that same folder stopped the render on this machine and not on the Mac. So on Linux it is not merely touching the installation directory that hurts a running render — it is replacing the files in place.

**Outside the interface. Observed on two machines.** The sources do not say, for either case. Every run on both machines was started with `--background` from a terminal with no window, and every one finished after its own program had been deleted.

**Operating system. Observed for macOS and for Linux, and they agree.** The sources do not say what happens on any operating system. They document Windows Settings or Control Panel for removing a previously installed version, but do not document the result of removal while Blender is running on any operating system. On macOS 26.5.2 the deletion was allowed while the render was running, and the render finished. On the rented Linux machine the same held. Windows remains untested, and there is no uninstaller route on either computer to compare against.

---

## 9. Can installing or removing a Blender be done again safely on a computer where it was cut off part-way?

**Label: not in the sources.**

Nothing in the 5.2 manual, the 5.2 API documentation or the 5.2 release notes discusses an interrupted install or an interrupted removal, a half-installed state, or repeating either operation. The installing pages describe each procedure once, as a thing that succeeds.

The nearest documented material, none of which answers the question:

> "Blender downloads can be extracted to any directory on the system, as a self contained installation."

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html — read 2026-07-30.

> "Note, you do not have to overwrite your existing Blender installation. It's perfectly possible to have multiple versions installed side by side."

Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html — read 2026-07-30.

> "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead. For more details, see issue #104070."

Source: manual, "Known Limitations" — https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html — read 2026-07-30. This is the only documented way an extraction is said to go wrong, and it is about the tool used, not about interruption.

> "Install the new version by overwriting the current Blender.app in the Applications folder."

Source: manual — https://docs.blender.org/manual/en/5.2/getting_started/installing/macos.html — read 2026-07-30. Overwriting is described as the normal update path on macOS; the manual does not say whether overwriting a partially-copied `Blender.app` is safe.

**Where I looked.** All five installing pages in full, the deployment page in full, the directory-layout page in full. I searched the whole manual for `interrupted`, `partially installed`, `idempotent`, `re-run`, `rerun`, `resume`, `retry`, `corrupt`, `repair`; the only `re-run` hits were about operators, simulations and asset listings, and there were no hits at all for the others in an installation context.

**Not covered by the sources.** Everything the question asks. Also uncovered: whether the Windows installer offers a repair mode; whether a partly-extracted archive is detectable; whether Blender validates its own installation at startup and reports a missing file. The macOS route — copying an application bundle into place — was tested by hand, below, and it also answers the last of those three for that route.

**Observed — hand test, 2026-08-04.** Run on the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2. Everything happened in a working folder outside the project; the installed Blender in `/Applications` was only ever read from.

*A note on the copying tool, because it changed the test.* The first attempt used `cp -R`. A Blender application bundle copied that way would not start at all on this computer — whether the copy had been interrupted or had finished — and `codesign --verify --verbose` on a finished `cp -R` copy printed `invalid Info.plist (plist or signature have been modified)`. So the test was run again with macOS's own copying tool, `ditto`, which does produce a bundle that starts, leaving the interruption as the only thing under test. Both attempts are recorded here in the order they happened.

*The test, with `ditto`.* Into a fresh empty folder, `ditto "/Applications/Blender.app" <working folder>/Blender.app` was started and killed after two seconds:

> ```
> ### copy killed at 21:40:19
> ### how much arrived: 393M
> ### is there an executable? -rwxr-xr-x@ 1 artist     admin  183237520 Jul 14 04:29 <working folder>/Blender.app/Contents/MacOS/Blender
> ### running the half-copied Blender with --version:
> ### exit code 137
> ```

So the half-copied Blender did have an executable at its full size, and running it printed nothing at all — no error text, no message naming a missing file — and gave exit code 137. The same `ditto` command was then run again over the same folder, without deleting anything first:

> ```
> ### second copy exit code 0
> ### size now: 907M
> ```

`codesign --verify --verbose` on the finished copy printed `a sealed resource is missing or invalid`, followed by a list of `file added:` lines, all of them compiled-Python cache files under `Contents/Resources/5.2/python/`. Running that finished copy with `--version` gave exit code 0 and printed:

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

which is the same as the installed 5.2.0 printed under question 3.

*What is left behind after a removal.* After a throwaway copy of 5.2.0 was deleted outright with `rm -rf` (see question 8), `ls ~/Library/Application\ Support/Blender/5.2` still printed `config`, `extensions`, `scripts`. So removing the application leaves that version's settings folder in place and populated, and a fresh copy put in afterwards would land on it rather than on a clean slate.

**Observed — hand tests, 2026-08-06, on the rented Linux machine. This settles the Linux archive route, including the 7-zip limitation quoted above, and it answers all three of the "Not covered" items that are not about Windows.** Run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Both `tar` and `7z` were already on the machine; nothing was installed for this.

*The archive.* No Blender archive was left on the machine, so `blender-5.2.0-linux-x64.tar.xz` — this version's own Linux archive — was fetched from Blender's own download server into the working folder, purely so that there was a real archive to extract. It was checked against Blender's own published checksum file and `sha256sum -c` printed `blender-5.2.0-linux-x64.tar.xz: OK`.

*Extracting with TAR, which the manual says must be used.* `tar -xf blender-5.2.0-linux-x64.tar.xz -C <empty folder>` finished with exit code 0. Running the result with `--version` gave exit code 0 and printed `Blender 5.2.0 LTS` and the rest.

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

The archive holds 79 internal links and 7-zip turned every one into a plain file. So the documented limitation is real, it breaks the result in two separate ways, and **it fails silently** — the tool reports success both times. That matters for anything automated: nothing in 7-zip's own output would tell a script it had produced a broken Blender.

*An extraction cut off part-way, and done again over the same folder.* `tar -xf` was started into an empty folder and killed part-way. What had arrived:

> ```
> 186M	half
> files: 769
> ```

and the main program had not been written at all — `ls` on it printed `No such file or directory`. Trying to run that path gave exit code 127, with the shell printing `No such file or directory`; Blender printed nothing, because there was no Blender there to print anything. **That answers the "whether a partly-extracted archive is detectable" item above, for this interruption at least: the program was simply missing.** The same `tar -xf` command was then run again over the same folder, with nothing deleted first. It finished with exit code 0, and the folder then held 5592 files and 79 links — the same as a clean extraction. Running the result with `--version` gave exit code 0, and its whole output was compared line by line against the 5.2.0 build already extracted under `/opt/blender`: **identical**.

So on Linux an interrupted extraction can simply be done again over the same folder, provided TAR is the tool. That is the same answer the Mac gave for its own shape of the question, reached by a different route.

*What is left behind after a removal.* On the rented machine, after a throwaway copy of this version was deleted outright with `rm -rf` while a background render was running from it (see question 8), that version's settings folder was still there and still held its contents. So a fresh copy put in afterwards lands on it rather than on a clean slate, as on the Mac.

**Outside the interface. Observed on two machines.** The extraction route is the one the deployment page says can be automated ("deployment can be automated using standard file management tools"), so it is the route most exposed to this question — and it is exactly the route the sources say nothing about with respect to interruption. Everything above was done from a terminal on both machines, with no Blender window at any point.

**Operating system. Observed for macOS and for Linux.** The sources do not say. The 7-zip limitation quoted above is stated on the Linux page only. On macOS 26.5.2 an interrupted `ditto` copy could be repeated over the same folder and produced a Blender that started and reported the right version. On the rented Linux machine an interrupted `tar` extraction could be repeated over the same folder and produced a Blender identical to the one already installed — and the 7-zip limitation was confirmed on the system it is stated for, with the two ways it breaks a build written out above. Windows remains untested, and the Windows installer, the Microsoft Store and Steam could not be tried on either computer; Snap and a distribution package were reached on the rented machine and are recorded under question 2.

---

## Things a person with Blender open could settle in a few minutes

The list that stood here has been worked through. What was settled is written into the answers above, marked observed. What is left open is below.

1. **Whether a running render survives its installation being replaced or deleted on Windows.** The macOS and Linux halves of both are answered under questions 6 and 8, and they do not agree with each other. On Windows the operating system may refuse the deletion — record which.

2. **Whether an interrupted install can be redone through the Windows installer.** Cancel the Blender 5.2 Windows installer mid-run, then run it again to completion, and note whether it complains, whether it offers repair, and whether the finished install starts. Then do the mirror test with the uninstaller. Needs Windows. The macOS and Linux shapes of this question are answered under question 9.

3. **What a Steam or Microsoft Store build reports, against a blender.org build of the same version.** Question 3 now records that a Snap build and a blender.org build of 5.2.0 report nothing different about themselves. The other two shopfronts are still out of reach.

4. **What a daily build reports about itself, and what its file is called.** Run `--version` and the `bpy.app` dump from question 3 against an alpha or beta build. No such build was available on either machine; question 2 records that Snap carries `beta` and `edge` channels, which is one way one could be obtained. Question 1 now records the file names of a finished release, so only the daily-build half of the naming question is still owed.

---

2026-08-04 — Opus 5 (Claude Code) — Builds, Blender 5.2

2026-08-04 — Opus 5 (Claude Code) — wrote the hand-test results into questions 1, 2, 3, 5, 6, 7, 8 and 9, brought the outside-the-interface and operating-system lines up to date where a test settled them, and pruned the hand-test list at the end

2026-08-05 — Opus 5 (Claude Code) — wrote the Creator's two screenshot tests into questions 1, 3 and 5, brought the outside-the-interface and operating-system lines up to date where they settled them, and pruned the hand-test list at the end

2026-08-05 — Opus 5 (Claude Code) — ran the hand test the citation check sent this way and wrote it into question 3: this build's `build_options` set beside the value printed in the API documentation, which it does not match

2026-08-06 — Opus 5 (Claude Code) — wrote the Linux hand-test results into questions 1 to 9, from a rented Linux machine; brought both standing lines up to date on every question a Linux test settled, and pruned the hand-test list at the end
