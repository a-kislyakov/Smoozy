# Builds — Open Threads questions, second pass (Blender 5.2)

**Sources consulted, and when.** Blender's 5.2 manual, Blender's 5.2 Python API documentation, and the Blender 5.2 release notes, all from the snapshot taken on **2026-07-30**; and the requirements page snapshot in `Blender Sources/requirements/`, taken on **2026-08-01**. I also looked through Cycles' own files in `Blender Sources/cycles_5.2` and found nothing bearing on this question. Work done on 2026-08-30.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: this pass answers **question 10 only**.

---

## 10. What does a computer need to have on it before a published Blender build of each kind can be unpacked?

### A note on the question's wording

The question is ambiguous and I want to say so before answering.

"Kind" could mean the **stability tier** — stable release, long-term support, daily build, source. Or it could mean the **package a build arrives in** — the Windows zip, the Windows installer, the macOS disk image, the Linux archive, and the store-delivered kinds.

I answer the second reading, because it is the one that makes "unpacked" mean anything: the stability tiers are not unpacked differently from one another, and the manual says so outright (quoted below). If the first reading was meant, the answer is short and is in part (a).

### (a) Stability tier makes no difference to unpacking

**Label: documented.**

> "The procedure for installing a binary, either the latest stable release or a daily build, is the same. Follow the steps for your platform listed below."

Source: manual, *Installing Blender*, https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html — read 2026-07-30.

So whatever a computer needs, it is set by the platform and the package, not by whether the build is stable, LTS, or a daily. The same page names the four kinds it offers — "Stable Release", "Long-term Support", "Daily Builds", "Build from Source" — and only the first three are binaries covered by that sentence. On source, the same page says only:

> "Blender's source code is available for free to either reference or to build and use. While normal users are not expected to compile Blender, it does have advantages"

It does not say what a computer needs in order to build it. I searched the whole 5.2 manual for `build from source`, `building blender`, `compile blender`, `source code` and found no page describing build prerequisites; the manual points nowhere further on this.

**Not covered:** what a computer needs to compile Blender from source. That is not in the 5.2 manual, API documentation, or release notes.

### (b) Windows — zip

**Label: documented.**

> "When choosing the zip-file, you have to manually extract Blender to the desired folder, where you can double-click the executable to run Blender."

> "No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."

Source: manual, *Installing on Windows*, https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html — read 2026-07-30.

So the one prerequisite the manual states for the zip is a **negative** one: administrator rights are not needed. The manual names no extraction tool for Windows and states no other precondition.

The same page states an architecture condition on which zip you take:

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture."

That is a condition on the file you choose, not on unpacking it — the manual does not say what happens if you extract the wrong one.

### (c) Windows — installer file

**Label: documented.**

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

Source: manual, *Installing on Windows*, https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html — read 2026-07-30.

**Administrator rights** are the stated requirement, and they are the only one stated. Note the installer is run rather than unpacked by hand; the manual describes no separate unpacking step for it.

### (d) Windows — Microsoft Store

**Label: documented, and thin.**

The manual's whole account is:

> "Blender can be installed from the Microsoft Store by searching for Blender in the Microsoft Store and installing it."

Source: manual, *Installing on Windows*, https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html — read 2026-07-30.

It names no prerequisite at all — not the Store app, not a Microsoft account, not administrator rights. A related fact from the release notes tells you the package form, without saying what is needed to open it:

> "Register protocol for Blender distributed as appx in Microsoft Store"

Source: release notes, *Fixes for issues present in previous versions*, under the heading "Platforms & Builds", https://developer.blender.org/docs/release_notes/5.2/bugfixes/ — read 2026-07-30.

### (e) macOS — disk image

**Label: documented.**

> "Blender for macOS is distributed as disk images (dmg-files). To mount the disk image, double-click on the dmg-file. Then drag Blender.app into the Applications folder."

Source: manual, *Installing on macOS*, https://docs.blender.org/manual/en/5.2/getting_started/installing/macos.html — read 2026-07-30.

No tool and no rights are named. The one further condition on the same page comes after opening, not before it:

> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."

Read that carefully: the manual puts the approval at *opening Blender*, not at mounting the image or copying the app. The manual says nothing anywhere about signing, notarisation, or quarantine — I searched the whole 5.2 manual, API documentation and release notes for `Gatekeeper`, `notariz`, `quarantine`, `codesign` and got **no hits at all**.

The same page also states a machine condition:

> "Blender 5.0 and later only supports Macs with Apple silicon."

That is a condition on running, as the page frames it under support. The sources do not say whether an Intel Mac can still mount the 5.2 disk image, so I am not claiming either way.

### (f) Linux — archive from blender.org

**Label: documented.** This is the only place in the sources where a *tool* is named as a prerequisite, and it is a hard one.

> "Download the Linux version for your architecture and decompress the file to the desired location (e.g. ~/software or /usr/local)."

And, under a heading "Known Limitations ‣ Archive Extraction" at the very bottom of the same page:

> "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead. For more details, see issue #104070."

Source: manual, *Installing on Linux*, https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html — read 2026-07-30.

So for the Linux archive the computer needs **TAR**, and 7-zip will not do. The manual does not name the archive's file extension — I searched the whole manual and release notes for `tar.xz` and `.xz` and found nothing — and it does not say what goes wrong when 7-zip is used, only that it is not supported.

### (g) Linux — distribution package manager

**Label: documented, and it names no prerequisite beyond the package manager itself.**

> "Some Linux distributions may have a specific package for Blender in their repositories."

> "If there is a specific package for your distribution, you may choose what is preferable and most convenient. Otherwise, the official binary is available on blender.org."

Source: manual, *Installing on Linux*, https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html — read 2026-07-30.

### (h) Linux — Snap

**Label: documented.**

> "Assuming snap is already installed, Blender can be installed through snap with:"
> `snap install blender --classic`

Source: manual, *Installing on Linux*, https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html — read 2026-07-30.

The prerequisite is stated in so many words: **snap must already be installed**. The manual does not say how to install snap, and does not say which distributions have it.

### (i) Steam

**Label: documented** for the Steam client; the account half is not stated as a precondition — see below. *(Citation check, 2026-08-30 — Claude Opus 5: label line adjusted to match the ruling below.)*

> "Download and install the Steam client for your operating system."

> "Once it is installed, open the client and login to your Steam account, or create an account if you don't already have one."

Source: manual, *Installing from Steam*, https://docs.blender.org/manual/en/5.2/getting_started/installing/steam.html — read 2026-07-30.

One prerequisite is stated outright: the **Steam client**. An account is used at the next step, and the quoted words expressly allow creating one there and then, so they do not say one must already exist. The same page says this route works "on Linux, macOS, or Windows". *(Citation check, 2026-08-30 — Claude Opus 5: changed — this read "Two prerequisites, both stated", which the quote does not carry for the account.)*

### (j) What all the unpacked binaries have in common

**Label: documented** for what the quotes say the build carries; **inferred** for anything about what the machine may lack. *(Citation check, 2026-08-30 — Claude Opus 5: label line adjusted to match the ruling below.)*

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

Source: manual, *Deploying Blender in Production*, https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html — read 2026-07-30.

The manual's directory-layout page says the build carries its own system files, including its own Python. That **nothing else** has to be on the computer is a step beyond those words, and the quotes below do not carry it — they say what the build brings, not what the machine may lack. *(Citation check, 2026-08-30 — Claude Opus 5: changed — this read "Nothing else has to be on the computer for the extracted tree to be complete", which reaches past the quotes.)*

> "System directories store files that come bundled with Blender and are required for it to function. This includes scripts, presets, essential assets and more."

> "./python — Bundled Python distribution. Located in system directories."

Source: manual, *Blender's Directory Layout*, https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html — read 2026-07-30. The same page confirms the extracted-folder layout for each platform, including "Zip file downloaded from blender.org: ./5.2/" on Windows and "Archive downloaded from blender.org: ./5.2/" on Linux.

The deploying page also says, of the bundled Python:

> "Blender bundles a complete Python installation and does not interact with the system Python by default."

### (k) What is *not* in the sources

**Label: not in the sources.** For every kind of build, the sources are silent on:

- **Disk space** needed to unpack, and download size. No figure anywhere.
- **Any runtime library or redistributable** that must be present before unpacking on Windows (no hit for `redistributable`, `Visual C`) or before unpacking on Linux. The Linux **glibc 2.28** minimum appears only on the requirements page snapshot (`Blender Sources/requirements/requirements_2026-08-01.txt`, line 43: "Distribution with glibc 2.28 or newer (64-bit)") and that page presents it as an operating-system minimum, not an unpacking condition — and it does not name version 5.2 in so many words for that row.
- **macOS code-signing, notarisation or quarantine** handling. Zero hits, as noted above.
- **Whether unpacking can fail part-way and what state that leaves.**
- **Prerequisites for the `bpy` Python package.** The release notes mention one exists ("Wrong max `numpy` version for `bpy` python package release", release notes, https://developer.blender.org/docs/release_notes/5.2/bugfixes/), and the API documentation acknowledges Blender can be built as a Python module (`bpy.app.binary_path`: "Read-only unless Blender is built as a Python module", https://docs.blender.org/api/5.2/bpy.app.html), but nothing in the sources describes installing or unpacking that package.

### Where I looked, in full

Searched across the whole of `manual_5.2`, `api_5.2` and `release_notes_5.2` (case-insensitive, whole-tree), for these terms:

`7-zip`, `7zip`, `tar`, `unzip`, `unpack`, `decompress`, `extract`, `archive`, `zip-file`, `.zip`, `dmg`, `msi`, `installer`, `administrator`, `administrator rights`, `glibc`, `disk image`, `mount`, `Gatekeeper`, `notariz`, `quarantine`, `codesign`, `redistributable`, `Visual C`, `Downloading Blender`, `download`, `minimum requirements`, `System Requirements`, `prerequisite`, `dependenc`, `Snap`, `Microsoft Store`, `package manager`, `flatpak`, `AppImage`, `chocolatey`, `winget`, `homebrew`, `self contained`, `self-contained`, `sudo`, `root privileges`, `disk space`, `free space`, `build from source`, `building blender`, `compile blender`, `source code`, `tar.xz`, `.xz`, `pip install`, `standalone module`, `bpy module`, `wheel`, `platform`, `arm64`, `x64`, `apple silicon`, `distribut`.

Pages opened and read in full:

- manual, `getting_started/installing/index.txt` → https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html
- manual, `getting_started/installing/windows.txt` → https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html
- manual, `getting_started/installing/macos.txt` → https://docs.blender.org/manual/en/5.2/getting_started/installing/macos.html
- manual, `getting_started/installing/linux.txt` → https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html
- manual, `getting_started/installing/steam.txt` → https://docs.blender.org/manual/en/5.2/getting_started/installing/steam.html
- manual, `getting_started/installing/linux_windowing_environment.txt` → https://docs.blender.org/manual/en/5.2/getting_started/installing/linux_windowing_environment.html
- manual, `getting_started/index.txt` → https://docs.blender.org/manual/en/5.2/getting_started/index.html
- manual, `advanced/deploying_blender.txt` → https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html
- manual, `advanced/blender_directory_layout.txt` → https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html
- manual, `troubleshooting/startup.txt` → https://docs.blender.org/manual/en/5.2/troubleshooting/startup.html
- release notes, `index.md` → https://developer.blender.org/docs/release_notes/5.2/
- release notes, `bugfixes.md`, the "Platforms & Builds" section → https://developer.blender.org/docs/release_notes/5.2/bugfixes/
- release notes, `user_interface.md`, the "Platforms" section → https://developer.blender.org/docs/release_notes/5.2/user_interface/
- `Blender Sources/requirements/requirements_2026-08-01.txt` (whole file)

Ruled out, and why:

- **Pictures.** The three platform installation pages and the installing index carry **no** `[image:]` markers at all — I checked each. There is no screenshot of an installer or a disk image to look at, so no picture could settle anything here.
- **Cycles' own files** (`Blender Sources/cycles_5.2`). Searched the whole folder for `extract`, `unzip`, `tar`, `archive`, `install`. Every hit is either the Apache licence text or vector-maths code named `extract`. Cycles' files describe render settings; they say nothing about how a Blender build is packaged or opened. Ruled out.
- **The Python API documentation.** Searched whole-tree; the only near hits are `bpy.ops.image.unpack` and friends, which are about data packed inside a blend-file, not about build packages. Ruled out.
- **`troubleshooting/startup.txt`.** Reached via `minimum requirements`. It lists causes of Blender failing to start ("Make sure any antivirus software is not preventing Blender from starting", "Ensure that you have appropriate user permissions on the computer") — all after installation, none about unpacking. Ruled out for this question.
- **`getting_started/configuration/hardware.txt`.** Reached via `Microsoft Store`; the hits are about the Windows Mixed Reality Portal and OpenXR, unrelated. Ruled out.
- **`advanced/extensions/*`.** Many hits for `extract`, `.zip`, `archive`, `wheel` — all about add-on and extension packages, not Blender builds. Ruled out.
- **The requirements page snapshot.** It is not per-version and its address carries no version. It names "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later"; the rest of its table names no version. Everything in it is framed as what is needed to *run*, not to unpack. Used only where noted, with that caveat.

---

## 10 — what was observed by hand, 2026-08-30

**Label: observed.** Everything below was run by me on 2026-08-30 and is recorded with the machine's own words. The full procedure and every raw output are in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`. Nothing here is reasoning; where a test settled something the answer above left open, that is said in the entry itself.

**The two machines, named in full, because that is what settles which operating system each answer covers.**

**The Creator's own Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no second card, arm64.

**A rented Linux machine, in Romania** — Vast.ai instance `49253756` from offer `43521340`, machine `124623`, labelled `smoozy-builds-q10-20260830`, reached over the provider's proxy at `root@ssh5.vast.ai` port `13756`, $0.0644 an hour, rented and destroyed the same day. **A container, not a whole machine**, established rather than assumed: `/.dockerenv` present, `systemd-detect-virt` says `docker`, process 1 is the provider's own launch script, `/proc/1/cgroup` prints `0::/`. Ubuntu 22.04.4 LTS, kernel `5.15.0-186-generic`, x86-64, glibc 2.35, from the image `nvidia/cuda:12.4.1-runtime-ubuntu22.04`. An Intel Xeon W-2123 with eight visible threads against a quota of 3.84 cores; a memory ceiling of 22,530,752,512 bytes where the ordinary reading shows the host's 33,191,886,848; one NVIDIA Quadro P4000 with 8192 MiB on driver `580.173.02`, which nothing here used; a 40 GiB overlay root; **no display server**.

**It was rented deliberately bare, and that is what makes these answers mean anything.** When it arrived it had 253 installed packages: `tar` (GNU tar 1.34) and `gzip`, and **no `xz`, no `unzip`, no `zip`, no `7z`, no `bzip2`, no `zstd`, no `file`, no `snap` and no `pip3`**. **None of the libraries Blender links** was in its library index. Nothing named after Blender was anywhere on it.

**All four Blenders on it matched the Creator's Mac to the exact build hash** — `84afd5f785f7`, `a3db93c5b259`, `ec6e62d40fa9`, `9e2066aef7ef` — so a result from one machine can be set beside a result from the other.

### What Blender actually publishes for 5.2.1

Read from Blender's own download server, `https://download.blender.org/release/Blender5.2/`, on the Creator's Mac:

```
blender-5.2.1-linux-x64.tar.xz      383,688,088
blender-5.2.1-macos-arm64.dmg        346,264,899
blender-5.2.1-windows-arm64.msi
blender-5.2.1-windows-arm64.msix
blender-5.2.1-windows-arm64.zip
blender-5.2.1-windows-x64.msi
blender-5.2.1-windows-x64.msix
blender-5.2.1-windows-x64.zip
blender-5.2.1.md5
blender-5.2.1.sha256
```

And, from `https://download.blender.org/source/`, `blender-5.2.1.tar.xz` (93,666,248 bytes) with a `blender-5.2.1.tar.xz.md5sum` beside it.

So for this version there is **one Linux package and one processor for it**; **one macOS package**, for Apple Silicon, and none for Intel; **three Windows file kinds for each of two processors** — `.msi`, `.msix` and `.zip`; **a checksum file in two forms**; and **a source archive**. No file name carries a stability word or the letters LTS, although this is an LTS release.



### A published checksum exists, and the fetched packages matched it

The `blender-5.2.1.sha256` file was fetched and checked against the packages fetched beside it. On the rented Linux machine: `blender-5.2.1-linux-x64.tar.xz: OK`. On the Creator's Mac: `blender-5.2.1-macos-arm64.dmg: OK`. So verifying a download before opening it is possible, which the answer above records as absent from the sources.

### What the Linux archive is

On the rented Linux machine — which arrived with no `file` program at all, so `file` was installed before this reading could be taken — `file blender-5.2.1-linux-x64.tar.xz` prints:

*(Citation check, 2026-08-30 — Claude Opus 5: changed — the step of installing `file` was missing; a reader following this on the bare machine described above would find the command absent. The first six bytes below were read without it.)*

```
blender-5.2.1-linux-x64.tar.xz: XZ compressed data, checksum CRC64
```

Its first six bytes are `fd 37 7a 58 5a 00`. `tar -tf` on it lists a single top folder, `blender-5.2.1-linux-x64/`. So the archive is a tar inside xz compression — the format the answer above records the manual as never naming.

### `tar` on its own does not open it

The rented machine had GNU tar 1.34 and no `xz` at all. `tar -xf blender-5.2.1-linux-x64.tar.xz -C <empty folder>` gave, word for word:

```
exit=2
tar (child): xz: Cannot exec: No such file or directory
tar (child): Error is not recoverable: exiting now
tar: Child returned status 2
tar: Error is not recoverable: exiting now
files landed: 0
```

Nothing was extracted. Telling tar the compression outright with `-xJf` gave the same exit code 2.

### One package makes the difference, and then it opens with no message

`apt-get install -y xz-utils` installed exactly one package and nothing else, taking the machine from 253 installed packages to 254. The same `tar -xf` command then gave:

```
exit=0
stderr:
top of the tree: blender-5.2.1-linux-x64
files: 5594   links: 81
unpacked size (bytes): 1238990213
```

*(A run before this one had `/usr/bin/time` in front of the command; that program is not on the machine either, and it gave `exit=127` and `bash: line 18: /usr/bin/time: No such file or directory` with nothing extracted. Both runs are in the hand-test list.)*

### The unpacked Blender does not start on that machine, and what it asks for

With the archive open and only that one extra package installed, `blender --version` gave:

```
exit=127
.../blender-5.2.1-linux-x64/blender: error while loading shared libraries: libX11.so.6: cannot open shared object file: No such file or directory
```

`ldd` on the program listed ten lines the machine could not satisfy, naming **nine distinct libraries** — `libX11.so.6` appears twice — and it is the same list in all four versions:

*(Citation check, 2026-08-30 — Claude Opus 5: changed — this read “ten libraries”; the record's own listing, printed below, holds nine distinct names across ten lines, and nine packages were what had to be installed. Corrected to what the listing shows.)*

```
	libX11.so.6 => not found
	libXrender.so.1 => not found
	libXfixes.so.3 => not found
	libXi.so.6 => not found
	libxkbcommon.so.0 => not found
	libSM.so.6 => not found
	libICE.so.6 => not found
	libX11.so.6 => not found
	libXext.so.6 => not found
	libGL.so.1 => not found
```

So the line this answer draws between opening the package and running what comes out is real, and it is wide: the unpacking needed none of these and the program needs all of them.

### What had to be installed before it would start, and that it is the Mac's own build

Nine packages were asked for — `libx11-6 libxrender1 libxfixes3 libxi6 libxkbcommon0 libsm6 libice6 libxext6 libgl1` — and the machine pulled in forty-two, taking it from 254 installed packages to 296. After that:

```
exit=0    remaining 'not found': 0
Blender 5.2.1 LTS
	build hash: 9e2066aef7ef
	build branch: blender-v5.2-release
	build platform: Linux
	build type: Release
```

The build hash `9e2066aef7ef` is the same one the Creator's Mac carries for this version.

### The program is marked runnable straight out of the archive, and no special account is needed

```
-rwxr-xr-x  ...  blender-5.2.1-linux-x64/blender      links in tree: 81
```

Nobody set that mark. On the same machine an ordinary user account (`uid=1000(plainuser)`) extracted the 5.2.1 archive and ran it — `tar exit=0`, `run exit=0`, the same file and link counts as the root extraction — so unpacking on Linux needs no administrator account.

### 7-zip really cannot be used, and it says it worked

With `7-Zip [64] 16.02` — installed on the machine for this test, since it arrived with no `7z` — two passes over `blender-5.2.1-linux-x64.tar.xz` both printed `Everything is Ok` and exited 0. *(Citation check, 2026-08-30 — Claude Opus 5: changed — the step of installing 7-zip was missing from the procedure.)* What came out will not run:

```
7-zip copy: files=5675  links=0
tar copy:   files=5594  links=81
-rw-r--r--  ...  ./blender-5.2.1-linux-x64/blender
trying to run it:  exit=126   bash: ./blender-5.2.1-linux-x64/blender: Permission denied
after putting the runnable mark back by hand:
  exit=127   .../blender-5.2.1-linux-x64/blender: error while loading shared libraries: .../lib/libtbb.so.12: file too short
```

Two things are wrong every time: the runnable mark is gone, and every symbolic link has become a plain file — 81 links in the tar copy against none in the 7-zip copy, with the matching extra plain files. The same happened in all four versions.

### The snap route on a machine with no snap

```
which snap: [ABSENT]
snap install blender --classic
exit=127
bash: line 6: snap: command not found
dpkg-query: no packages found matching snapd
```

The command the manual gives does not exist to be run.

### What the distribution's own package manager carries

On Ubuntu 22.04, `apt-cache policy blender`:

```
blender:
  Installed: (none)
  Candidate: 3.0.1+dfsg-7
     3.0.1+dfsg-7 500  http://archive.ubuntu.com/ubuntu jammy/universe amd64 Packages
```

A simulated install, which installed nothing, reported `3 upgraded, 242 newly installed`. *(The reading before any package list is fetched could not be taken, because `apt-get update` had already been run on that machine for the test above. That gap is mine, and it is recorded in the hand-test list.)*

### The published source archive opens with the same two tools

```
blender-5.2.1.tar.xz: XZ compressed data, checksum CRC64
published md5sum check: blender-5.2.1.tar.xz: OK
tar exit=0
top of tree: blender-5.2.1
directly inside it: AUTHORS CMakeLists.txt COPYING GNUmakefile README.md assets build_files doc extern intern lib locale make.bat pyproject.toml
unpacked size (bytes): 315326661
```

So for the source kind, opening the published thing needs exactly what the binary Linux archive needs. What building it needs was not tested and is not settled.

### Blender as a Python module is published for this version, and it needs one exact Python

The package index lists these files for 5.2.1:

```
bpy-5.2.1-cp313-cp313-macosx_11_0_arm64.whl     | requires-python: ==3.13.*
bpy-5.2.1-cp313-cp313-manylinux_2_28_x86_64.whl | requires-python: ==3.13.*
bpy-5.2.1-cp313-cp313-win_amd64.whl             | requires-python: ==3.13.*
bpy-5.2.1-cp313-cp313-win_arm64.whl             | requires-python: ==3.13.*
```

On the rented machine's own Python 3.10.12 — with a package tool installed for this test, since the machine arrived with none — `pip install bpy` gave `ERROR: Could not find a version that satisfies the requirement bpy (from versions: none)`. Asking the index to resolve for other Python versions gave nothing for 3.10 and 3.12, and a file for 3.11 and for 3.13. So what a computer needs for this kind is a Python of one exact version — **3.13 for 5.2.1** — and a working package tool. A real install was done on a Python 3.13.13, using the interpreter Blender 5.2.1's own unpacked tree carries at `5.2/python/bin/python3.13`; it fetched eleven further packages alongside `bpy`, finished with exit 0, and `bpy.app.version_string` then read `5.2.1 LTS` with `bpy.app.binary_path` empty.

### The macOS disk image opens with nobody clicking

On the Creator's Mac, with no window open and no person:

```
hdiutil attach blender-5.2.1-macos-arm64.dmg
...
/dev/disk4s1        	Apple_HFS                      	/Volumes/Blender
attach exit=0
cp -R "/Volumes/Blender/Blender.app" <a working folder>     cp exit=0
hdiutil detach "/Volumes/Blender"                          "disk4" ejected.   detach exit=0
<copied app>/Contents/MacOS/Blender --version
Blender 5.2.1 LTS
	build hash: 9e2066aef7ef
	build platform: Darwin
```

No password was asked for at any step and `sudo` was never used. The account it ran as is in the `admin` group, so this settles that no elevation is *requested* and leaves what a standard account would meet untested. The mounted volume holds four things: a symbolic link to `/Applications`, a `.DS_Store`, a `.background.tif`, and `Blender.app`. The application is 950,546,432 bytes against a 346,264,899-byte download.

### What macOS marks, and that it stops nothing before the first launch

The downloaded disk image carried one attribute, `com.apple.provenance`, and no quarantine mark — but it was fetched with `curl`, which does not set the mark a web browser sets, so that reading covers a package fetched by a program and not one fetched by a person at a browser.

The copied application is signed and notarised, and macOS's own assessment accepts it:

```
Identifier=org.blenderfoundation.blender
Format=app bundle with Mach-O thin (arm64)
Authority=Developer ID Application: Stichting Blender Foundation (68UA947AUU)
Authority=Developer ID Certification Authority
Authority=Apple Root CA
TeamIdentifier=68UA947AUU
spctl: accepted   source=Notarized Developer ID
       origin=Developer ID Application: Stichting Blender Foundation (68UA947AUU)
```

Separately, a copy of the 5.2.1 image was marked by hand with `com.apple.quarantine: 0083;00000000;Safari;` and still mounted (`attach exit=0`), still copied out (`cp exit=0`), and the application still ran from a terminal (`run exit=0`), carrying `com.apple.quarantine: 0283;00000000;;` onto itself and still accepted by `spctl`. So the mark stops neither the mounting nor the copying. What it does at a Finder double-click was not tested, and the manual's own sentence puts its approval step there.

### A package for the wrong processor still opens

On the Creator's Mac, which has an Apple Silicon processor, the 4.5.12 **Intel** package mounted (`attach exit=0`), copied out (`cp exit=0`, 864M) and detached. The two 4.5.12 binaries read:

```
Intel package:          Mach-O 64-bit executable x86_64     lipo -archs: x86_64
Apple Silicon package:  Mach-O 64-bit executable arm64      lipo -archs: arm64
```

Neither package holds both processors. The Intel build also ran on that Mac, printing `Blender 4.5.12 LTS` and `build hash: 84afd5f785f7` at exit 0 — but only on a second attempt, the first having printed nothing for two minutes before my own time limit cut it off. **Rosetta was already on that Mac and was read rather than installed**, so what a wrong-processor package does on a Mac without it is not settled. For 5.2.1, no wrong-processor macOS package exists to try — the download server publishes one macOS file and it is for Apple Silicon.

### What this settles about disk space

```
Linux archive:  download 383,688,088 bytes   →  unpacked 1,238,990,213 bytes
macOS image:    download 346,264,899 bytes   →  application 950,546,432 bytes
source archive: download 93,666,248 bytes   →  unpacked 315,326,661 bytes
```

A binary package roughly triples when opened, and the machine needs room for the download and the result at once. No figure of this kind is in the sources.

### Outside the interface

**Label: documented, in part.**

Unpacking happens before Blender runs, so nothing here depends on Blender's own window at all — the acts described are extracting an archive, mounting a disk image, running an installer, or asking a store to install. The manual says of the extracted builds:

> "deployment can be automated using standard file management tools"

Source: manual, *Deploying Blender in Production*, https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html — read 2026-07-30.

That is the only statement in the sources about doing this without a person. It covers the plain extract-to-a-directory kinds. Note two places where the manual's own instructions are written as pointing-and-clicking and nothing else is offered: "To mount the disk image, double-click on the dmg-file" (macOS) and the Steam route, where you "open the client and login to your Steam account". The sources do not say whether those can be driven without a person. The Snap route is given as a command line (`snap install blender --classic`), so that one plainly can.

**Brought up to date by hand tests, 2026-08-30 — label: observed.** This line said the sources give no way to unpack without a person, and for two of the three operating systems that is now answered by running it. **On the Creator's Mac** the whole macOS route was driven from a terminal with no window and nobody clicking: `hdiutil attach`, `cp -R` the application out, `hdiutil detach`, then the copied Blender printed its version — for all four versions, with no prompt and no elevation. A disk image marked by hand as downloaded from the internet went the same way. **On a rented Linux machine** the archive was fetched, verified against its published checksum, opened with `tar`, and started, all from a terminal on a machine with no display server at all — and an ordinary user account did it too, so no administrator account is involved. What is still not settled from outside is the Windows half: whether the Windows installer file can be run with nobody present. Those tests are written out ready to run in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*.

### Operating system

**Label: documented.** This answer differs by operating system by its nature, and the differences are the answer:

- **Windows** — administrator rights for the installer, none for the zip; no tool named.
- **macOS** — double-click the disk image; no tool and no rights named; Blender 5.0 and later supports only Apple silicon Macs.
- **Linux** — TAR is required and 7-zip is not supported; or snap must already be installed; or a distribution package must exist.
- **All three** — Steam works on Linux, macOS and Windows, and needs the Steam client and an account.

The store and package-manager routes exist on some systems and not others: Microsoft Store on Windows only, Snap and distribution packages on Linux only, disk images on macOS only. Nothing in the sources contradicts this across systems.

**Brought up to date by hand tests, 2026-08-30 — label: observed.** Two of the three operating systems are now answered by running, and the third is written out ready to run.

**Linux, on a rented Linux machine in Romania** described in the observed section above. `tar` **on its own is not enough**: a machine with GNU tar 1.34 and no `xz` refused all four archives with `tar (child): xz: Cannot exec: No such file or directory` and extracted nothing. Installing one package, `xz-utils`, made all four open with no message. The archive is a tar inside xz compression. The unpacked program is marked runnable straight out of the archive, needs no administrator account, and **will not start** on that machine until nine further library packages are installed — the first thing it asks for is `libX11.so.6`, and `ldd` prints ten lines naming nine distinct libraries. *(Citation check, 2026-08-30 — Claude Opus 5: changed from “ten missing libraries” to match the listing.)* 7-zip reports success twice and produces a Blender that cannot run, in all four versions. `snap install blender --classic` on a machine with no snap is `command not found`. Ubuntu 22.04's own package is Blender 3.0.1.

**macOS, on the Creator's Mac** described in the observed section above. The disk image mounts from a terminal with no prompt and no elevation, the application copies out, and it runs — for all four versions. The application is signed and notarised and `spctl` accepts it. A quarantine mark stops neither the mount nor the copy nor a terminal run. A package for the wrong processor mounts and copies out just the same.

**Windows is still untested, and it is not untestable.** This project has a route to a Windows computer — a rental described in the equipment section of `Preparation/Blender Research/Method.md` — and the Windows entries are written out in full, ready to run, in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*. **Any answer taken there must name the Windows edition it was taken on**: every Windows machine this project has read turned out to be a Server edition whatever its image was called, and a finding that says only "Windows" is not an answer. What is already settled about Windows without such a machine, from the download server alone, is the published file kinds: `.msi`, `.msix` and `.zip`, for each of an x64 and an arm64 processor.

---

## What a computer could settle that a source cannot — pruned 2026-08-30

**Most of the researcher's own list has been run.** The hand tests of 2026-08-30 are recorded in the observed section under question 10 above, and in full in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`. Answered by running: what the Linux archive actually is; whether `tar` on its own opens it; what has to be installed before it opens; whether 7-zip breaks the build; whether the program is marked runnable; whether the unpacked Blender starts on a bare machine and what it asks for; how big each package is and what it becomes; whether a checksum is published and matches; whether the macOS disk image can be mounted and copied without a person; what macOS marks a downloaded package with and whether that stops anything; whether a wrong-processor macOS package still opens; whether a machine with no snap can use the snap route; what the distribution's own package carries; whether the published source archive opens with the same tools; and whether Blender's Python module is published and what Python it demands.

**What is genuinely still open, and where it is going.**

1. **Everything on Windows.** Whether Windows' own extraction opens the zip on a machine with nothing else installed; whether the zip really needs no administrator rights and the installer really does; whether the installer can be run with nobody present; what the Microsoft Store route demands; and whether a wrong-processor Windows package still opens. These are written out in full, ready to run, in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md` under *Windows — for the runner's separate batch*. **This project has a route to a Windows computer** — a rental described in the equipment section of `Preparation/Blender Research/Method.md` — so these are waiting on a batch, not on equipment nobody has. **Any answer from there must name the Windows edition it was taken on.**

2. **Anything through Steam.** Both stated prerequisites are the client and an account, and the account half cannot be tested: creating one, or entering credentials into one, is not something an agent on this project does.

3. **What a computer needs to *build* Blender from source.** The opening half is answered — the published source archive is a tar inside xz compression with a published md5sum, and the same two tools open it. What compiling needs is not answered and was not attempted.

4. **What a *standard* macOS account meets.** The macOS route was driven as an account in the `admin` group, and nothing asked for elevation. Making a standard account is a change to the Creator's own machine, which these tests do not make.

5. **What a wrong-processor macOS package does on a Mac with no Rosetta**, and **what a package fetched by a web browser carries on macOS** — the packages here were fetched with `curl`, which does not set the mark a browser sets.

6. **What a daily, alpha, beta or release-candidate package is.** Only finished releases sit in the release folders that were read.

---

2026-08-30 — Claude Opus 5 (Claude Code) — Builds, question 10, Blender 5.2
