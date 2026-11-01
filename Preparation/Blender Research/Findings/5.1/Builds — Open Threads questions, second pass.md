# Builds — Open Threads questions, second pass (Blender 5.1)

**Sources consulted, and on what date.** Blender's manual for 5.1 (plain text plus its `_images` folder), Blender's Python API documentation for 5.1, Blender's release notes for 5.1, Cycles' own files for 5.1, and the copy of Blender's requirements page taken 2026-08-01 — all of them inside `Blender Sources/`. Date read for everything from the manual, the API documentation and the release notes: **2026-07-30** (the snapshot date given to me). Date read for the requirements page: **2026-08-01** (the date it carries in its own name).

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

This pass answers **question 10 only**. No other question on the sheet was answered.

---

## 10. What does a computer need to have on it before a published Blender build of each kind can be unpacked?

### The question was ambiguous, and I say so before answering

"Each kind" can mean two different things, and the sheet's own note under question 10 does not settle which.

- **Reading A — kind of release.** The four things the manual lists under "Download Blender": Stable Release, Long-term Support, Daily Builds, Build from Source.
- **Reading B — kind of package.** The differently shaped things actually handed to a computer: a Windows zip, a Windows installer file, a Microsoft Store install, a macOS disk image, a Linux archive, a Snap, a Steam install.

**I answer both**, because reading A is what question 1 names and reading B is where all the real content sits. Almost everything the sources say about *unpacking* is organised by package shape, not by release channel. If only one reading was meant, reading B is the one that has answers.

---

### 10a. Reading A — by kind of release

**LABEL: documented** (for stable versus daily), **not in the sources** (for build from source).

The manual states that the release channel makes no difference to how a binary is put on a computer:

> "The procedure for installing a binary, either the latest stable release or a daily build, is the same. Follow the steps for your platform listed below."

Source: manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html — date read 2026-07-30.

The same page lists the four kinds, and describes the daily-build markings, in these words:

> "Blender offers a variety of different binary packages to choose from depending on their level of stability."

> "Builds marked as Alpha are still undergoing major changes and feature additions, while those marked Beta are feature-complete and are under development for refinement and stability."

Source: manual, same page and date.

Long-term Support is listed there as one of those "binary packages", alongside Stable Release and Daily Builds, so the sentence about "either the latest stable release or a daily build" does not name LTS in so many words. It is described as "A package designed for long-lasting projects requiring a very stable version of Blender."

**Build from Source is different, and the sources do not cover it.** The manual says the source code is available and lists reasons to compile, but gives no build prerequisites at all:

> "Blender’s source code is available for free to either reference or to build and use. While normal users are not expected to compile Blender, it does have advantages:"

Source: manual, same page and date. I searched the whole 5.1 manual for `CMake` (no hits anywhere), `Visual Studio` (no hits), `Xcode` (no hits), `build from source` (one hit, the page just quoted), and `compil` (hits, but every one outside the installing page is about shader compiling, OSL, Python wheels or GPU kernel compiling — none about building Blender). So what a computer needs before Blender's source can be built is **not in the sources**.

**Not covered by 10a.** It does not say whether the *release candidate* is a separate package with separate needs; it does not say whether the LTS package is shaped differently from the stable one; and it says nothing about what building from source requires.

---

### 10b. Reading B — by kind of package, one by one

Below, each package kind, with what the sources say must already be on the computer.

#### Windows — zip file

**LABEL: documented.**

> "Download the zip-file or Windows Installer File."

> "When choosing the zip-file, you have to manually extract Blender to the desired folder, where you can double-click the executable to run Blender."

> "No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."

Source: manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html — date read 2026-07-30.

Also on that page, before either method:

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU’s architecture."

So what must be present: a way to extract a zip, and a build matching the processor. **The manual does not name any particular extraction tool for Windows**, and does not say whether Windows' own built-in zip handling is enough. That is an absence, and I searched for it — see the record of looking.

#### Windows — Windows Installer File

**LABEL: documented.**

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

Source: manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html — date read 2026-07-30.

So what must be present: administrator rights. This is the only place in the 5.1 installing pages where a privilege level is named as a requirement.

#### Windows — Microsoft Store

**LABEL: documented** for the quoted route; **inferred** that the Store must be present. *(Citation check, 2026-08-30 — Claude Opus 5: label line adjusted to match the ruling below.)*

> "Blender can be installed from the Microsoft Store by searching for Blender in the Microsoft Store and installing it."

Source: manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html — date read 2026-07-30.

So what must be present, **inferred** and not documented: the Microsoft Store. The quote says Blender can be installed from the Store; it does not say the Store must be there first, and that step is the reader's. *(Citation check, 2026-08-30 — Claude Opus 5: changed — relabelled inferred.)* The manual says nothing further — no account requirement, no rights requirement, no unpacking step of any kind. Where such an install lands is given elsewhere:

> "Microsoft Store installation:
> `%ProgramFiles%\WindowsApps\BlenderFoundation.Blender<HASH>\Blender\5.1\`"

Source: manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — date read 2026-07-30.

#### macOS — disk image (dmg)

**LABEL: documented** for the quoted instructions and the architecture sentence; **inferred** that the ability to mount a disk image counts as a precondition. *(Citation check, 2026-08-30 — Claude Opus 5: label line adjusted to match the ruling below.)*

> "Blender for macOS is distributed as disk images (dmg-files). To mount the disk image, double-click on the dmg-file. Then drag Blender.app into the Applications folder."

> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."

> "Blender supports both Intel and Apple Silicon architectures on macOS. Make sure to download a variant that is compatible with your CPU’s architecture."

Source: manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/macos.html — date read 2026-07-30.

So what must be present: a build matching the processor, which the page states. That the ability to mount a disk image is a precondition is **inferred** — the manual gives an instruction to mount and names no tool, and reading a requirement out of an instruction is the reader's step. *(Citation check, 2026-08-30 — Claude Opus 5: changed — the mounting ability was presented under a documented label; relabelled inferred.)* Note carefully that the approval sentence is about **opening** Blender for the first time, not about mounting or unpacking; the manual draws that line itself.

One conflict between two sources, reported without comment on either. The 5.1 manual page above says Blender "supports both Intel and Apple Silicon architectures on macOS". The requirements page taken 2026-08-01 says:

> "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later."

Source: Blender's requirements page, `Blender Sources/requirements/requirements_2026-08-01.txt`, in the macOS section — date read 2026-08-01. That page carries no version in its address; the sentence quoted is one of the few that names versions in so many words, and "5.0 and later" covers 5.1. It speaks about **running**, not about unpacking.

#### Linux — archive from blender.org

**LABEL: documented.** This is the sharpest answer on the whole question.

> "Download the Linux version for your architecture and decompress the file to the desired location (e.g. `~/software` or `/usr/local`)."

And, in that page's own "Known Limitations" section at the very bottom:

> "Archive Extraction
>
> Extracting Blender’s archive using 7-zip is not supported. TAR must be used instead. For more details, see issue #104070."

Source: manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html — date read 2026-07-30.

So what must be present on Linux: **TAR**. And one named tool is explicitly ruled out: 7-zip. This is the only place in the 5.1 sources that names a required or forbidden extraction tool for any platform.

The manual does not name the Linux archive's format. I searched the whole 5.1 manual for `.xz`, `tar.xz`, `gzip`, `bzip` and `lzma`; `.xz` and `tar.xz` have no hits anywhere in the manual, `gzip` appears only on the blend-file saving page, and `bzip`/`lzma` have no hits at all. So "TAR must be used" is what the source gives, and the archive's actual compression is **not in the sources**.

Where such an install lands:

> "Linux
>
> Archive downloaded from blender.org:
>
> `./5.1/`"

Source: manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — date read 2026-07-30.

#### Linux — Snap

**LABEL: documented.**

> "Snap is a universal package manager designed to work across a range of distributions. Assuming snap is already installed, Blender can be installed through snap with:
>
> `snap install blender --classic`"

Source: manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html — date read 2026-07-30.

So what must be present: snap, already installed. The manual states this precondition outright ("Assuming snap is already installed") and does not say how to get it.

#### Linux — distribution package

**LABEL: documented** for what the two quotes say; **inferred** that the distribution's own package manager must be present. *(Citation check, 2026-08-30 — Claude Opus 5: label line adjusted to match the ruling below.)*

> "Some Linux distributions may have a specific package for Blender in their repositories."

> "Be aware, though, that the package may be outdated compared to the latest official release, or not include some features of Blender. For example, some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."

Source: manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html — date read 2026-07-30.

The manual names no requirement at all here. That the distribution's own package manager and a repository carrying Blender must be present is **inferred** from the quoted line, not stated by it. *(Citation check, 2026-08-30 — Claude Opus 5: changed — relabelled inferred.)*

#### Any platform — Steam

**LABEL: documented** for the Steam client; the account half is not stated as a precondition — see below. *(Citation check, 2026-08-30 — Claude Opus 5: label line adjusted to match the ruling below.)*

> "Download and install the Steam client for your operating system."

> "Once it is installed, open the client and login to your Steam account, or create an account if you don’t already have one."

Source: manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/steam.html — date read 2026-07-30.

So what must be present: the Steam client. An account is used at the next step, and the quoted words expressly allow creating one there and then, so they do not say one must already exist. This is the only published route in the 5.1 manual that involves an account at all. *(Citation check, 2026-08-30 — Claude Opus 5: changed — this read "the Steam client, and a Steam account… requires an account", which the quote does not carry.)*

#### The bpy Python module

**LABEL: documented** for the two quoted facts, and **inferred** that PIP must therefore be present. Included because it is a published thing that is not on the download page — the API documentation says so itself. *(Citation check, 2026-08-30 — Claude Opus 5: label line adjusted to match the ruling below.)*

> "Blender as a Python Module isn’t provided on Blender’s official download page."

> "A pre-compiled bpy module is available via PIP."

Source: Python API documentation, https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html — date read 2026-07-30.

What the page states is that a pre-compiled module is available via PIP. That PIP must therefore be present is **inferred**, not documented — that step is the reader's. The page names nothing else. *(Citation check, 2026-08-30 — Claude Opus 5: changed — relabelled inferred.)* It offers the alternative of compiling it yourself, pointing at build instructions that are not in these sources.

The 5.1 release notes record a problem with this package:

> "* Python bpy package is missing Python dependencies. (blender/blender#157121)"

> "* Wrong max `numpy` version for `bpy` python package release. (blender/blender#155983)"

Source: release notes, https://developer.blender.org/docs/release_notes/5.1/corrective_releases/ — date read 2026-07-30.

---

### 10c. The one general statement that covers all of them

**LABEL: documented.**

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

Source: manual, https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — date read 2026-07-30.

And, from the requirements page:

> "Truly Portable
>
> Whether it's on a USB stick, sitting on a folder on your desktop, or fully installed, Blender runs out of the box.
>
> No installation needed, no Internet connection required."

Source: Blender's requirements page, `Blender Sources/requirements/requirements_2026-08-01.txt`, section "Truly Portable" — date read 2026-08-01. This page does not name 5.1, so it speaks for whatever release was current on that date.

Taken together the sources say: for the binary archives, nothing has to be prepared on the computer beyond somewhere to put the files and a tool able to open the archive — on Linux, specifically TAR.

---

### What the sources do **not** say about this question

All of these are **not in the sources**, each with the searches that show it.

- **No named minimum free disk space** for any package. Searched the whole 5.1 manual for `disk space` (no hits) and `free space` (one hit, in video editing, unrelated). The requirements page lists OS, CPU, RAM and GPU only — no storage row at all.
- **No required runtime library, redistributable or framework** that must be present before unpacking. Searched the manual for `redistributable` (no hits), `runtime librar` (no hits), `prerequisit` (four hits, all about modelling, physics and add-on scripting), `must be installed` (one hit, in the add-on tutorial), `requires .* installed` (no hits).
- **No checksum or signature verification step.** Searched the manual for `checksum` (no hits), `md5` (no hits), `sha256` (one hit, on the extensions static-repository page, about extension listings — not about Blender builds), `verify the download` (no hits), `signature` (many hits, all about node closures and bundles).
- **No mention of macOS Gatekeeper, notarization or quarantine.** Searched the manual and release notes for `Gatekeeper`, `notariz`, `quarantine` and `codesign` — zero hits for all four. The macOS page's only security sentence is the "Security and Privacy preferences" one quoted above, and it is about first launch.
- **No mention of Rosetta or of translating an Intel build on Apple Silicon.** Searched for `Rosetta` — zero hits in the manual and release notes.
- **No glibc requirement in the manual.** Searched the whole 5.1 manual for `glibc` — zero hits. The only glibc statement is on the requirements page ("Distribution with glibc 2.28 or newer (64-bit)"), which is a run requirement for whatever release was current on 2026-08-01, and does not name 5.1.
- **No extraction tool named for Windows or macOS.** Only Linux gets one.
- **Nothing in the 5.1 release notes about packaging, installers, archive formats or platform requirements.** I read `index.md` in full and searched every release-notes file for `requirement`, `no longer support`, `minimum version`, `drop support` and `now requires`; the single hit is about Node Tools idnames, which is unrelated. The compatibility section of `index.md` lists library upgrades (Python 3.13, OpenColorIO 2.5, OpenEXR 3.4, OpenVDB 13.0, matching VFX platform 2026) — those are bundled inside the build, not things a computer must already have.

---

---

## 10 — what was observed by hand, 2026-08-30

**Label: observed.** Everything below was run by me on 2026-08-30 and is recorded with the machine's own words. The full procedure and every raw output are in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`. Nothing here is reasoning; where a test settled something the answer above left open, that is said in the entry itself.

**The two machines, named in full, because that is what settles which operating system each answer covers.**

**The Creator's own Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no second card, arm64.

**A rented Linux machine, in Romania** — Vast.ai instance `49253756` from offer `43521340`, machine `124623`, labelled `smoozy-builds-q10-20260830`, reached over the provider's proxy at `root@ssh5.vast.ai` port `13756`, $0.0644 an hour, rented and destroyed the same day. **A container, not a whole machine**, established rather than assumed: `/.dockerenv` present, `systemd-detect-virt` says `docker`, process 1 is the provider's own launch script, `/proc/1/cgroup` prints `0::/`. Ubuntu 22.04.4 LTS, kernel `5.15.0-186-generic`, x86-64, glibc 2.35, from the image `nvidia/cuda:12.4.1-runtime-ubuntu22.04`. An Intel Xeon W-2123 with eight visible threads against a quota of 3.84 cores; a memory ceiling of 22,530,752,512 bytes where the ordinary reading shows the host's 33,191,886,848; one NVIDIA Quadro P4000 with 8192 MiB on driver `580.173.02`, which nothing here used; a 40 GiB overlay root; **no display server**.

**It was rented deliberately bare, and that is what makes these answers mean anything.** When it arrived it had 253 installed packages: `tar` (GNU tar 1.34) and `gzip`, and **no `xz`, no `unzip`, no `zip`, no `7z`, no `bzip2`, no `zstd`, no `file`, no `snap` and no `pip3`**. **None of the libraries Blender links** was in its library index. Nothing named after Blender was anywhere on it.

**All four Blenders on it matched the Creator's Mac to the exact build hash** — `84afd5f785f7`, `a3db93c5b259`, `ec6e62d40fa9`, `9e2066aef7ef` — so a result from one machine can be set beside a result from the other.

### What Blender actually publishes for 5.1.2

Read from Blender's own download server, `https://download.blender.org/release/Blender5.1/`, on the Creator's Mac:

```
blender-5.1.2-linux-x64.tar.xz      396,237,880
blender-5.1.2-macos-arm64.dmg        335,350,362
blender-5.1.2-windows-arm64.msi
blender-5.1.2-windows-arm64.msix
blender-5.1.2-windows-arm64.zip
blender-5.1.2-windows-x64.msi
blender-5.1.2-windows-x64.msix
blender-5.1.2-windows-x64.zip
blender-5.1.2.md5
blender-5.1.2.sha256
```

And, from `https://download.blender.org/source/`, `blender-5.1.2.tar.xz` (90,622,692 bytes) with a `blender-5.1.2.tar.xz.md5sum` beside it.

So for this version there is **one Linux package and one processor for it**; **one macOS package**, for Apple Silicon, and none for Intel; **three Windows file kinds for each of two processors** — `.msi`, `.msix` and `.zip`; **a checksum file in two forms**; and **a source archive**. No file name carries a stability word or the letters LTS.

**This settles, for the published files, the disagreement this document records between its own sources.** The manual page says both Intel and Apple Silicon builds exist; the release notes and the requirements page say Intel is gone. What is published for 5.1.2 is one macOS package and it is for Apple Silicon. The same split appears again in the checksum file, which lists no macOS Intel line, and again in the Python module listing below.

### A published checksum exists, and the fetched packages matched it

The `blender-5.1.2.sha256` file was fetched and checked against the packages fetched beside it. On the rented Linux machine: `blender-5.1.2-linux-x64.tar.xz: OK`. On the Creator's Mac: `blender-5.1.2-macos-arm64.dmg: OK`. So verifying a download before opening it is possible, which the answer above records as absent from the sources.

### What the Linux archive is

On the rented Linux machine — which arrived with no `file` program at all, so `file` was installed before this reading could be taken — `file blender-5.1.2-linux-x64.tar.xz` prints:

*(Citation check, 2026-08-30 — Claude Opus 5: changed — the step of installing `file` was missing; a reader following this on the bare machine described above would find the command absent. The first six bytes below were read without it.)*

```
blender-5.1.2-linux-x64.tar.xz: XZ compressed data, checksum CRC64
```

Its first six bytes are `fd 37 7a 58 5a 00`. `tar -tf` on it lists a single top folder, `blender-5.1.2-linux-x64/`. So the archive is a tar inside xz compression — the format the answer above records the manual as never naming.

### `tar` on its own does not open it

The rented machine had GNU tar 1.34 and no `xz` at all. `tar -xf blender-5.1.2-linux-x64.tar.xz -C <empty folder>` gave, word for word:

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
top of the tree: blender-5.1.2-linux-x64
files: 5300   links: 77
unpacked size (bytes): 1276420342
```

*(A run before this one had `/usr/bin/time` in front of the command; that program is not on the machine either, and it gave `exit=127` and `bash: line 18: /usr/bin/time: No such file or directory` with nothing extracted. Both runs are in the hand-test list.)*

### The unpacked Blender does not start on that machine, and what it asks for

With the archive open and only that one extra package installed, `blender --version` gave:

```
exit=127
.../blender-5.1.2-linux-x64/blender: error while loading shared libraries: libX11.so.6: cannot open shared object file: No such file or directory
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
Blender 5.1.2
	build hash: ec6e62d40fa9
	build branch: blender-v5.1-release
	build platform: Linux
	build type: Release
```

The build hash `ec6e62d40fa9` is the same one the Creator's Mac carries for this version.

### The program is marked runnable straight out of the archive, and no special account is needed

```
-rwxr-xr-x  ...  blender-5.1.2-linux-x64/blender      links in tree: 77
```

Nobody set that mark. On the same machine an ordinary user account (`uid=1000(plainuser)`) extracted the 5.2.1 archive and ran it — `tar exit=0`, `run exit=0`, the same file and link counts as the root extraction — so unpacking on Linux needs no administrator account.

### 7-zip really cannot be used, and it says it worked

With `7-Zip [64] 16.02` — installed on the machine for this test, since it arrived with no `7z` — two passes over `blender-5.1.2-linux-x64.tar.xz` both printed `Everything is Ok` and exited 0. *(Citation check, 2026-08-30 — Claude Opus 5: changed — the step of installing 7-zip was missing from the procedure.)* What came out will not run:

```
7-zip copy: files=5377  links=0
tar copy:   files=5300  links=77
-rw-r--r--  ...  ./blender-5.1.2-linux-x64/blender
trying to run it:  exit=126   bash: ./blender-5.1.2-linux-x64/blender: Permission denied
after putting the runnable mark back by hand:
  exit=127   .../blender-5.1.2-linux-x64/blender: error while loading shared libraries: .../lib/libtbb.so.12: file too short
```

Two things are wrong every time: the runnable mark is gone, and every symbolic link has become a plain file — 77 links in the tar copy against none in the 7-zip copy, with the matching extra plain files. The same happened in all four versions.

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
blender-5.1.2.tar.xz: XZ compressed data, checksum CRC64
published md5sum check: blender-5.1.2.tar.xz: OK
tar exit=0
top of tree: blender-5.1.2
directly inside it: AUTHORS CMakeLists.txt COPYING GNUmakefile README.md assets build_files doc extern intern lib locale make.bat pyproject.toml
unpacked size (bytes): 306787166
```

So for the source kind, opening the published thing needs exactly what the binary Linux archive needs. What building it needs was not tested and is not settled.

### Blender as a Python module is published for this version, and it needs one exact Python

The package index lists these files for 5.1.2:

```
bpy-5.1.2-cp313-cp313-macosx_11_0_arm64.whl     | requires-python: ==3.13.*
bpy-5.1.2-cp313-cp313-manylinux_2_28_x86_64.whl | requires-python: ==3.13.*
bpy-5.1.2-cp313-cp313-win_amd64.whl             | requires-python: ==3.13.*
bpy-5.1.2-cp313-cp313-win_arm64.whl             | requires-python: ==3.13.*
```

On the rented machine's own Python 3.10.12 — with a package tool installed for this test, since the machine arrived with none — `pip install bpy` gave `ERROR: Could not find a version that satisfies the requirement bpy (from versions: none)`. Asking the index to resolve for other Python versions gave nothing for 3.10 and 3.12, and a file for 3.11 and for 3.13. So what a computer needs for this kind is a Python of one exact version — **3.13 for 5.1.2** — and a working package tool. There is no Linux arm64 file, and no macOS Intel file for this version — the same split the download server shows.

### The macOS disk image opens with nobody clicking

On the Creator's Mac, with no window open and no person:

```
hdiutil attach blender-5.1.2-macos-arm64.dmg
...
/dev/disk4s1        	Apple_HFS                      	/Volumes/Blender
attach exit=0
cp -R "/Volumes/Blender/Blender.app" <a working folder>     cp exit=0
hdiutil detach "/Volumes/Blender"                          "disk4" ejected.   detach exit=0
<copied app>/Contents/MacOS/Blender --version
Blender 5.1.2
	build hash: ec6e62d40fa9
	build platform: Darwin
```

No password was asked for at any step and `sudo` was never used. The account it ran as is in the `admin` group, so this settles that no elevation is *requested* and leaves what a standard account would meet untested. The mounted volume holds four things: a symbolic link to `/Applications`, a `.DS_Store`, a `.background.tif`, and `Blender.app`. The application is 921,300,992 bytes against a 335,350,362-byte download.

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

Neither package holds both processors. The Intel build also ran on that Mac, printing `Blender 4.5.12 LTS` and `build hash: 84afd5f785f7` at exit 0 — but only on a second attempt, the first having printed nothing for two minutes before my own time limit cut it off. **Rosetta was already on that Mac and was read rather than installed**, so what a wrong-processor package does on a Mac without it is not settled. For 5.1.2, no wrong-processor macOS package exists to try — the download server publishes one macOS file and it is for Apple Silicon.

### What this settles about disk space

```
Linux archive:  download 396,237,880 bytes   →  unpacked 1,276,420,342 bytes
macOS image:    download 335,350,362 bytes   →  application 921,300,992 bytes
source archive: download 90,622,692 bytes   →  unpacked 306,787,166 bytes
```

A binary package roughly triples when opened, and the machine needs room for the download and the result at once. No figure of this kind is in the sources.

### Outside the interface

**LABEL: partly documented, partly inferred.**

Documented: unpacking is a file operation, and the manual says it can be driven without a person clicking:

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

Source: manual, https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — date read 2026-07-30.

The Snap route is documented as a command line and nothing else (`snap install blender --classic`), so it is by nature usable without the interface.

Inferred, with the step written out: the Linux "TAR must be used instead" instruction and the Windows "no need for administrator rights" statement are properties of the archive and of the operating system, not of Blender's window, so they should hold identically whether a person or a script does the extracting. **The step a reader must supply is that a scripted extraction uses the same extractor and the same rights as a manual one** — the sources never say this, so the label is inferred, not documented.

Two places where the sources describe a click and give no non-interactive equivalent: the macOS instruction is "double-click on the dmg-file" and then drag the app, and the Microsoft Store instruction is to search the store and press install. Whether either can be done from outside the interface is **not in the sources**. Steam likewise is described only as a client you open and click in.

Note also that none of this is Blender's own doing: at unpack time Blender is not running yet, so Blender's own outside-the-interface machinery (background mode, command-line arguments) is not involved at all.

**Brought up to date by hand tests, 2026-08-30 — label: observed.** This line said the sources give no way to unpack without a person, and for two of the three operating systems that is now answered by running it. **On the Creator's Mac** the whole macOS route was driven from a terminal with no window and nobody clicking: `hdiutil attach`, `cp -R` the application out, `hdiutil detach`, then the copied Blender printed its version — for all four versions, with no prompt and no elevation. A disk image marked by hand as downloaded from the internet went the same way. **On a rented Linux machine** the archive was fetched, verified against its published checksum, opened with `tar`, and started, all from a terminal on a machine with no display server at all — and an ordinary user account did it too, so no administrator account is involved. What is still not settled from outside is the Windows half: whether the Windows installer file can be run with nobody present. Those tests are written out ready to run in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*.

### Operating system

**LABEL: documented.** This answer differs by operating system more than most, and the sources organise it that way on purpose — there is a separate installing page per platform.

- **Windows** — zip (no administrator rights) or installer file (administrator rights required) or Microsoft Store; x64 and arm64 builds exist and must match the processor.
- **macOS** — a disk image, mounted by double-click, then the app dragged to Applications; the 5.1 manual says Intel and Apple Silicon builds both exist, while the requirements page says 5.0 and later require Apple Silicon on macOS 13 or later.
- **Linux** — an archive that must be extracted with TAR and not with 7-zip; or Snap, which must already be installed; or a distribution package.
- **All three** — Steam, which needs the Steam client and an account.

The 5.1 manual's list of installation guides is exactly: Linux, macOS, Windows, Steam. No other operating system is covered, and no other appears anywhere in the installing section.

**Not covered by the answer as a whole.** The sources say what package each platform gets and, for Linux only, which tool must open it. They do not say how much room a build takes, whether a checksum should be checked, what happens if the wrong architecture's build is unpacked, whether an extraction can be done without a person present on macOS or through the Microsoft Store, or what a computer needs before Blender's source can be built. It also does not settle whether "each kind" in the question meant release channel or package shape — I answered both.

**Brought up to date by hand tests, 2026-08-30 — label: observed.** Two of the three operating systems are now answered by running, and the third is written out ready to run.

**Linux, on a rented Linux machine in Romania** described in the observed section above. `tar` **on its own is not enough**: a machine with GNU tar 1.34 and no `xz` refused all four archives with `tar (child): xz: Cannot exec: No such file or directory` and extracted nothing. Installing one package, `xz-utils`, made all four open with no message. The archive is a tar inside xz compression. The unpacked program is marked runnable straight out of the archive, needs no administrator account, and **will not start** on that machine until nine further library packages are installed — the first thing it asks for is `libX11.so.6`, and `ldd` prints ten lines naming nine distinct libraries. *(Citation check, 2026-08-30 — Claude Opus 5: changed from “ten missing libraries” to match the listing.)* 7-zip reports success twice and produces a Blender that cannot run, in all four versions. `snap install blender --classic` on a machine with no snap is `command not found`. Ubuntu 22.04's own package is Blender 3.0.1.

**macOS, on the Creator's Mac** described in the observed section above. The disk image mounts from a terminal with no prompt and no elevation, the application copies out, and it runs — for all four versions. The application is signed and notarised and `spctl` accepts it. A quarantine mark stops neither the mount nor the copy nor a terminal run. A package for the wrong processor mounts and copies out just the same.

**Windows is still untested, and it is not untestable.** This project has a route to a Windows computer — a rental described in the equipment section of `Preparation/Blender Research/Method.md` — and the Windows entries are written out in full, ready to run, in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*. **Any answer taken there must name the Windows edition it was taken on**: every Windows machine this project has read turned out to be a Server edition whatever its image was called, and a finding that says only "Windows" is not an answer. What is already settled about Windows without such a machine, from the download server alone, is the published file kinds: `.msi`, `.msix` and `.zip`, for each of an x64 and an arm64 processor.

---

## Record of where I looked

Searched with case-insensitive full-text search across the whole of `manual_5.1`, `api_5.1` and `release_notes_5.1` at once, excluding the `_images` folder from text searches.

**Terms searched.** `7-zip`, `7zip`, `tar`, `.xz`, `tar.xz`, `gzip`, `bzip`, `lzma`, `unpack`, `unzip`, `zip`, `zip-file`, `decompress`, `compressed`, `compression`, `extract`, `archive`, `.dmg`, `dmg`, `disk image`, `mount`, `.msi`, `msix`, `installer`, `administrator`, `elevated`, `root`, `double-click`, `Gatekeeper`, `notariz`, `quarantine`, `codesign`, `signed`, `signature`, `Rosetta`, `x86`, `arm64`, `aarch64`, `architecture`, `glibc`, `self contained`, `self-contained`, `portable`, `checksum`, `sha256`, `md5`, `verify the download`, `disk space`, `free space`, `redistributable`, `runtime librar`, `dependenc`, `prerequisit`, `before you can`, `must be installed`, `already installed`, `requires … installed`, `compil`, `CMake`, `build from source`, `source code`, `Visual Studio`, `Xcode`, `git`, `requirement`, `minimum and recommended`, `system requirements`, `SSE4`, `Downloading Blender`, `pip install`, `standalone module`, `as a python module`, `wheel`, plus, over the release notes alone, `no longer support`, `minimum version`, `drop support`, `now requires`, `platform`, `package`, `download`, `build`, `Vulkan`, `OpenGL`, `driver`.

**Pages opened in full.**
- Manual, Installing Blender (index): https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html
- Manual, Installing on Linux: https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html
- Manual, Installing on macOS: https://docs.blender.org/manual/en/5.1/getting_started/installing/macos.html
- Manual, Installing on Windows: https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html
- Manual, Installing from Steam: https://docs.blender.org/manual/en/5.1/getting_started/installing/steam.html
- Manual, Linux Windowing Environment: https://docs.blender.org/manual/en/5.1/getting_started/installing/linux_windowing_environment.html
- Manual, Deploying Blender in Production: https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html
- Manual, Blender's Directory Layout: https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html
- Python API, Blender as a Python Module: https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html
- Release notes, the 5.1 index page: https://developer.blender.org/docs/release_notes/5.1/
- Blender's requirements page, `Blender Sources/requirements/requirements_2026-08-01.txt`, read in full.

**Pages opened in part.**
- Manual, Launching from the command line on Windows: https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/windows.html — read around its "installer" mention; it only gives the installed path, nothing about unpacking.
- Release notes, Core: https://developer.blender.org/docs/release_notes/5.1/core/ — searched, nothing about packaging.
- Release notes, Pipeline & I/O: https://developer.blender.org/docs/release_notes/5.1/pipeline_io/ — searched, nothing about packaging.
- Release notes, Corrective Releases: https://developer.blender.org/docs/release_notes/5.1/corrective_releases/ — searched; only the two bpy-package lines quoted above are relevant.

**Ruled out, and why.**
- The manual's whole `contribute/` section, including its "Installation on Windows" page, which is about installing Python and Git in order to build the *manual*. Its first lines are "This guide covers the following topics: Installing Python (used to 'convert' the source files to HTML)". Nothing to do with published Blender builds.
- `manual_5.1/files/blend/open_save.txt` and the glossary, which produced the `decompress`/`compression` hits — both about compressing blend-files, not build archives.
- Every `extract` hit outside the installing and deploying pages — sculpting mask extraction, image editing, node maths, geometry nodes, and the extensions/wheels pages. None about build archives.
- All `zip` hits under `advanced/extensions/` and `editors/preferences/` — these are about extension and add-on packages installed *into* Blender, which is a different thing from unpacking a published Blender build.
- The `_images` folder: I checked which pictures the four installing pages reference and found that **none of them reference any picture at all**, so there is no screenshot of a download or install screen to read. The only install-named image on disk, `about_contribute_install_windows_installer.png`, belongs to the manual-contribution page just ruled out, so I did not treat it as evidence about Blender builds.
- `cycles_5.1` in full: searched it for `unzip`, `tar`, `archive`, `installer`, `.dmg`, `extract the` and `7-zip`. The only hits are the Apache licence text and comments in the render kernel about extracting angles from directions. Cycles' files say nothing about packaging or unpacking Blender, which is expected — they are render settings and kernel code.
- I did not use the web, and did not need to.

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

2026-08-30 — Claude Opus 5 (Claude Code) — Builds, Blender 5.1
