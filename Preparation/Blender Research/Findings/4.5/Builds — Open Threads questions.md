# Builds — Open Threads questions (Blender 4.5)

**Sources consulted, and when.** Blender's 4.5 manual, Blender's 4.5 Python API documentation, and the Blender 4.5 release notes, all as the plain-text copies in `Blender Sources/`, taken 2026-07-30 — that is the date read for everything quoted from them. Cycles' own files for 4.5, in `Blender Sources/cycles_4.5`, looked over and ruled out by what they hold — the reason is under *Where I looked*. Blender's requirements page, as the copy named for the date it was taken, 2026-08-01. I did the reading on 2026-08-29.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

This is a first pass. The brief that set it asked for a wider search and for a written record of where I looked, which is why that record is fuller than a first pass usually carries. I did not open any findings document in the folder this file was saved into.

---

## 10. What does a computer need to have on it before a published Blender build of each kind can be unpacked?

### The reading I answered

The question is ambiguous in one way and I say so rather than hiding the choice. "Unpacked" could mean either (a) what must already be present on a machine so that the published download can be opened at all and a Blender comes out of it, or (b) everything that must be present before that Blender will then run. I answered (a), because the question's own note says it asks "only what has to be present before the published thing can be opened at all". Where the sources give a requirement that plainly belongs to (b) instead — hardware, drivers, operating-system versions — I say so and set it aside rather than counting it.

### The short answer

The sources describe almost nothing that must be present. For every build a person downloads from blender.org, the manual describes unpacking as an ordinary file operation, and names two preconditions in words across all of them: administrator rights for the Windows installer (and explicitly *not* for the Windows zip), and a tar tool rather than 7-zip on Linux. A third — the ability to mount a disk image on macOS — is **inferred**, not named: the manual tells the reader to double-click the disk image and nowhere states that ability as a requirement. For the builds obtained through a store or package manager, that the store or package manager itself must already be present is **documented** for Snap and for Steam, which say so in words, and **inferred** for the Microsoft Store and for a distribution's own package, which do not. Everything else — a specific unzip utility on Windows, disk space, any runtime library needed merely to extract — is not in the sources. Every one of these is set out below with its quotation and its own label.

### The kinds, and why they collapse into one answer here

The manual's own list of kinds is Stable Release, Long-term Support, Daily Builds, and Build from Source. That list is summarised here rather than quoted, and no page is cited for it. For the first three, the manual states outright that unpacking does not differ between them:

> "The procedure for installing a binary, either the latest stable release or a daily build, is the same. Follow the steps for your platform listed below."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/index.html (line 56 of the page's text), read 2026-07-30.

**Label: documented.** The sentence names the latest stable release and a daily build. It does not name LTS in that sentence. That LTS unpacks the same way is **inferred**: the same page describes LTS as one of the packages offered under the same "Download Blender" heading and gives no separate procedure for it, and the platform pages that the sentence points to make no distinction by kind. The step a reader must supply is "a package described alongside the others, with no procedure of its own, follows the shared procedure." That is a small step, but it is a step, so the label is inferred.

So what varies is not the kind of build but the platform package. The rest of this answer is organised that way.

### Windows — the zip

> "When choosing the zip-file, you have to manually extract Blender to the desired folder, where you can double-click the executable to run Blender."

> "No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html (lines 20–24 of the page's text), read 2026-07-30.

**Label: documented** for the two facts stated — extraction is a manual step the user performs, and administrator rights are not needed. The manual names no tool to do the extracting with, and no other precondition.

### Windows — the installer

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html (lines 15–16), read 2026-07-30.

**Label: documented.** Administrator rights are the one stated precondition. The sources do not say whether the requirement bites at the moment of unpacking or only at the moment of doing the things the quote names — writing into the chosen install folder, creating the start menu entry, and associating blend-files.

### Windows — architecture

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html (lines 10–11), read 2026-07-30.

**Label: documented** that the variant must match the processor. Whether a mismatched variant can still be *unpacked* — as opposed to run — is not in the sources, and I did not assume either way.

### Windows — the Microsoft Store

> "Blender can be installed from the Microsoft Store by searching for Blender in the Microsoft Store and installing it."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html (line 34), read 2026-07-30.

**Label: documented** that the Microsoft Store is the mechanism. That the Store must therefore be present and working on the machine is **inferred** — the manual describes no other route to that package, and the reader must supply the step "an installation performed by the Store needs the Store". Nothing about an account, a sign-in, or unpacking is stated.

### macOS — the disk image

> "Blender for macOS is distributed as disk images (dmg-files). To mount the disk image, double-click on the dmg-file. Then drag Blender.app into the Applications folder."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/macos.html (lines 13–14), read 2026-07-30.

**Label: documented** that unpacking here means mounting a disk image and copying the application out of it. No tool, no rights, and no other precondition are named. macOS's own ability to mount a disk image is what the instruction assumes. **Label: inferred** for that ability counting as a precondition: the manual does not say so in words, and the step a reader has to supply is "an instruction to mount a disk image needs a machine that can mount one." Treating it as a stated requirement would be me speaking, not the source.

One nearby sentence is about a later moment and I am not counting it as an unpacking precondition:

> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."

Source: same page, lines 16–17, read 2026-07-30. It says *opening Blender*, not opening the disk image.

> "Blender supports both Intel and Apple Silicon architectures on macOS. Make sure to download a variant that is compatible with your CPU's architecture."

Source: same page, lines 8–9, read 2026-07-30. **Documented**, with the same edge as on Windows: it does not say whether a mismatched variant still unpacks.

### Linux — the archive from blender.org

> "Download the Linux version for your architecture and decompress the file to the desired location (e.g. ~/software or /usr/local)."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html (lines 8–9), read 2026-07-30.

And, under that page's own "Known Limitations" heading:

> "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead. For more details, see issue #104070."

Source: same page, lines 99–102, read 2026-07-30.

**Label: documented.** This is the one place in the whole of the sources where a specific tool is required to unpack a Blender build. A tar tool must be on the machine; 7-zip will not do the job. The manual does not say what goes wrong with 7-zip, and it does not name the archive's file extension anywhere — I searched for `.tar`, `tar.xz`, `tar.bz2` and `tarball` across the manual, the API documentation and the release notes, and the only hit outside this sentence is an unrelated example in `bpy.path`. So "the Linux download is a tar archive" is **inferred** from "TAR must be used instead", not documented in so many words.

### Linux — a distribution package, Snap, Steam

> "Some Linux distributions may have a specific package for Blender in their repositories."

> "Assuming snap is already installed, Blender can be installed through snap with: snap install blender --classic"

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html (lines 22 and 33–38), read 2026-07-30. *(Citation check, 2026-08-30 — Claude Opus 5: changed — line range corrected — the snap command itself is on line 38.)* The Snap line states its own precondition in words: snap already installed. **Documented.** The distribution-package line states no precondition at all; that a distribution's own package manager must be present before one of its packages can be installed is **inferred**, the same step as the Microsoft Store above.

> "Download and install the Steam client for your operating system."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/steam.html (line 6 of the page's text), read 2026-07-30. *(Citation check, 2026-08-30 — Claude Opus 5: changed — line number corrected from 5 to 6.)* **Documented** that the Steam client must be on the machine first. The page says "for your operating system" and gives no split by platform, so it does not itself name which operating systems it covers. The same page also mentions an account — "open the client and login to your Steam account, or create an account if you don't already have one" — same page, same date read. **Documented** that a Steam account is used at that step. The quoted words do **not** say one must already exist, because they offer creating one there and then. *(Citation check, 2026-08-30 — Claude Opus 5: changed — this read "also requires an account… **Documented**", which the quote does not carry.)*

### Every platform — what the manual says the unpacking amounts to

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html (lines 11–14), read 2026-07-30.

**Label: documented.** This is the strongest general statement available: a download can be extracted to any directory, the result is self-contained, several versions can sit side by side, and deployment can be automated with standard file management tools. It does **not** say that nothing beyond standard file-management tools is ever required — the Linux page's tar-not-7-zip line, above, is a specific tool requirement stated elsewhere in the same manual, and reading this sentence as a blanket "no tool is needed" would be a step the words do not carry. It is also consistent with the layout page, which describes the unpacked results plainly — for Linux, "Archive downloaded from blender.org: ./4.5/"; for Windows, "Zip file downloaded from blender.org: ./4.5/" against "Installer downloaded from blender.org: %ProgramFiles%\Blender Foundation\Blender\4.5\"; for macOS, "./Blender.app/Contents/Resources/4.5/". Source: manual, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html (lines 56–78), read 2026-07-30. The same page twice calls the Windows and Linux results "the unzipped folder" (lines 36 and 38).

### Build from Source

**Label: not in the sources.** The manual describes this kind but gives no procedure and no requirements at all:

> "Blender's source code is available for free to either reference or to build and use. While normal users are not expected to compile Blender, it does have advantages"

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/index.html (lines 44–45), read 2026-07-30. The "Installation Guides" that follow are Linux, macOS, Windows and Steam only — there is no page for building. I searched the manual for "compile", "build Blender", "build from source" and "source code"; the hits are the licence and history pages, a note that some hardware support "has to be compiled from source, which in this case is not recommended", and unrelated shader-compilation debug flags. Nothing states what a machine needs in order to unpack or build a source release.

### Things the sources are silent on, that a reader might expect to find here

All of these are **not in the sources**, and the record of the searching is in the section below:

- Any named unzip tool, or any statement that one is needed, on Windows. The manual says extract; it does not say with what.
- Any minimum free disk space, for any platform or any kind.
- Whether a runtime library, redistributable, or system component must be present *to unpack* on any platform. I searched for "Visual C", "redistributable", "dependenc", "prerequisite" and "glibc"; the manual has no hits that bear on this at all.
- Whether administrator or root rights are needed to mount a macOS disk image, or to extract on Linux.
- Any signing, notarisation or quarantine step that must be satisfied before the download opens. I searched for "Gatekeeper", "quarantine", "notariz", "codesign" and "signature": no hits anywhere in the 4.5 manual, API documentation or release notes.
- Anything at all about unpacking daily builds specifically — Alpha, Beta or Release Candidate. The only statement is the shared-procedure sentence quoted at the top.

On the requirements page: it does carry a Linux line reading "Distribution with glibc 2.28 or newer (64-bit)", but that table describes whichever release was current on the day the page was taken, and it does not name 4.5. The only lines on that page that name 4.5 in so many words are about running, not unpacking — "Blender 4.5 LTS supports NVIDIA GeForce 400 and newer. AMD GCN 1st gen and newer. Intel Broadwell architecture and newer.", and "Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur)." Source: `Blender Sources/requirements/requirements_2026-08-01.txt`, page taken 2026-08-01, read by me 2026-08-29. I am therefore not attaching any glibc figure to 4.5, and in any case it is a requirement for running, not for unpacking.

### Not covered

This answer settles what the sources state must be present before a download opens. It does not settle: what a machine needs to *run* the unpacked Blender (hardware, drivers, operating-system version — those are the requirements page's subject and a different question); whether a build of a mismatched processor architecture can still be extracted; what actually fails when 7-zip is used on the Linux archive; how much space anything needs; whether the Windows installer's administrator requirement applies at extraction or only at installation; or anything about builds for operating systems other than Windows, macOS and Linux, which the sources do not document.

---

## 10 — what was observed by hand, 2026-08-30

**Label: observed.** Everything below was run by me on 2026-08-30 and is recorded with the machine's own words. The full procedure and every raw output are in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`. Nothing here is reasoning; where a test settled something the answer above left open, that is said in the entry itself.

**The two machines, named in full, because that is what settles which operating system each answer covers.**

**The Creator's own Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no second card, arm64.

**A rented Linux machine, in Romania** — Vast.ai instance `49253756` from offer `43521340`, machine `124623`, labelled `smoozy-builds-q10-20260830`, reached over the provider's proxy at `root@ssh5.vast.ai` port `13756`, $0.0644 an hour, rented and destroyed the same day. **A container, not a whole machine**, established rather than assumed: `/.dockerenv` present, `systemd-detect-virt` says `docker`, process 1 is the provider's own launch script, `/proc/1/cgroup` prints `0::/`. Ubuntu 22.04.4 LTS, kernel `5.15.0-186-generic`, x86-64, glibc 2.35, from the image `nvidia/cuda:12.4.1-runtime-ubuntu22.04`. An Intel Xeon W-2123 with eight visible threads against a quota of 3.84 cores; a memory ceiling of 22,530,752,512 bytes where the ordinary reading shows the host's 33,191,886,848; one NVIDIA Quadro P4000 with 8192 MiB on driver `580.173.02`, which nothing here used; a 40 GiB overlay root; **no display server**.

**It was rented deliberately bare, and that is what makes these answers mean anything.** When it arrived it had 253 installed packages: `tar` (GNU tar 1.34) and `gzip`, and **no `xz`, no `unzip`, no `zip`, no `7z`, no `bzip2`, no `zstd`, no `file`, no `snap` and no `pip3`**. **None of the libraries Blender links** was in its library index. Nothing named after Blender was anywhere on it.

**All four Blenders on it matched the Creator's Mac to the exact build hash** — `84afd5f785f7`, `a3db93c5b259`, `ec6e62d40fa9`, `9e2066aef7ef` — so a result from one machine can be set beside a result from the other.

### What Blender actually publishes for 4.5.12

Read from Blender's own download server, `https://download.blender.org/release/Blender4.5/`, on the Creator's Mac:

```
blender-4.5.12-linux-x64.tar.xz     377,902,364
blender-4.5.12-macos-arm64.dmg       308,255,218
blender-4.5.12-macos-x64.dmg              335,832,861
blender-4.5.12-windows-arm64.msi
blender-4.5.12-windows-arm64.msix
blender-4.5.12-windows-arm64.zip
blender-4.5.12-windows-x64.msi
blender-4.5.12-windows-x64.msix
blender-4.5.12-windows-x64.zip
blender-4.5.12.md5
blender-4.5.12.sha256
```

And, from `https://download.blender.org/source/`, `blender-4.5.12.tar.xz` (85,105,056 bytes) with a `blender-4.5.12.tar.xz.md5sum` beside it.

So for this version there is **one Linux package and one processor for it**; **one macOS package**, for Apple Silicon, and a second for Intel; **three Windows file kinds for each of two processors** — `.msi`, `.msix` and `.zip`; **a checksum file in two forms**; and **a source archive**. No file name carries a stability word or the letters LTS, although this is an LTS release.



### A published checksum exists, and the fetched packages matched it

The `blender-4.5.12.sha256` file was fetched and checked against the packages fetched beside it. On the rented Linux machine: `blender-4.5.12-linux-x64.tar.xz: OK`. On the Creator's Mac: `blender-4.5.12-macos-arm64.dmg: OK` and `blender-4.5.12-macos-x64.dmg: OK`. So verifying a download before opening it is possible, which the answer above records as absent from the sources.

### What the Linux archive is

On the rented Linux machine — which arrived with no `file` program at all, so `file` was installed before this reading could be taken — `file blender-4.5.12-linux-x64.tar.xz` prints:

*(Citation check, 2026-08-30 — Claude Opus 5: changed — the step of installing `file` was missing; a reader following this on the bare machine described above would find the command absent. The first six bytes below were read without it.)*

```
blender-4.5.12-linux-x64.tar.xz: XZ compressed data, checksum CRC64
```

Its first six bytes are `fd 37 7a 58 5a 00`. `tar -tf` on it lists a single top folder, `blender-4.5.12-linux-x64/`. So the archive is a tar inside xz compression — the format the answer above records the manual as never naming.

### `tar` on its own does not open it

The rented machine had GNU tar 1.34 and no `xz` at all. `tar -xf blender-4.5.12-linux-x64.tar.xz -C <empty folder>` gave, word for word:

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
top of the tree: blender-4.5.12-linux-x64
files: 5579   links: 71
unpacked size (bytes): 1169157971
```

*(A run before this one had `/usr/bin/time` in front of the command; that program is not on the machine either, and it gave `exit=127` and `bash: line 18: /usr/bin/time: No such file or directory` with nothing extracted. Both runs are in the hand-test list.)*

### The unpacked Blender does not start on that machine, and what it asks for

With the archive open and only that one extra package installed, `blender --version` gave:

```
exit=127
.../blender-4.5.12-linux-x64/blender: error while loading shared libraries: libX11.so.6: cannot open shared object file: No such file or directory
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
Blender 4.5.12 LTS
	build hash: 84afd5f785f7
	build branch: blender-v4.5-release
	build platform: Linux
	build type: Release
```

The build hash `84afd5f785f7` is the same one the Creator's Mac carries for this version.

### The program is marked runnable straight out of the archive, and no special account is needed

```
-rwxr-xr-x  ...  blender-4.5.12-linux-x64/blender      links in tree: 71
```

Nobody set that mark. On the same machine an ordinary user account (`uid=1000(plainuser)`) extracted the 5.2.1 archive and ran it — `tar exit=0`, `run exit=0`, the same file and link counts as the root extraction — so unpacking on Linux needs no administrator account.

### 7-zip really cannot be used, and it says it worked

With `7-Zip [64] 16.02` — installed on the machine for this test, since it arrived with no `7z` — two passes over `blender-4.5.12-linux-x64.tar.xz` both printed `Everything is Ok` and exited 0. *(Citation check, 2026-08-30 — Claude Opus 5: changed — the step of installing 7-zip was missing from the procedure.)* What came out will not run:

```
7-zip copy: files=5650  links=0
tar copy:   files=5579  links=71
-rw-r--r--  ...  ./blender-4.5.12-linux-x64/blender
trying to run it:  exit=126   bash: ./blender-4.5.12-linux-x64/blender: Permission denied
after putting the runnable mark back by hand:
  exit=127   .../blender-4.5.12-linux-x64/blender: error while loading shared libraries: .../lib/libtbb.so.12: file too short
```

Two things are wrong every time: the runnable mark is gone, and every symbolic link has become a plain file — 71 links in the tar copy against none in the 7-zip copy, with the matching extra plain files. The same happened in all four versions.

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
blender-4.5.12.tar.xz: XZ compressed data, checksum CRC64
published md5sum check: blender-4.5.12.tar.xz: OK
tar exit=0
top of tree: blender-4.5.12
directly inside it: AUTHORS CMakeLists.txt COPYING GNUmakefile README.md assets build_files doc extern intern lib locale make.bat pyproject.toml
unpacked size (bytes): 304479424
```

So for the source kind, opening the published thing needs exactly what the binary Linux archive needs. What building it needs was not tested and is not settled.

### Blender as a Python module is published for this version, and it needs one exact Python

The package index lists these files for 4.5.12:

```
bpy-4.5.12-cp311-cp311-macosx_11_0_arm64.whl     | requires-python: ==3.11.*
bpy-4.5.12-cp311-cp311-macosx_11_0_x86_64.whl    | requires-python: ==3.11.*
bpy-4.5.12-cp311-cp311-manylinux_2_28_x86_64.whl | requires-python: ==3.11.*
bpy-4.5.12-cp311-cp311-win_amd64.whl             | requires-python: ==3.11.*
bpy-4.5.12-cp311-cp311-win_arm64.whl             | requires-python: ==3.11.*
```

On the rented machine's own Python 3.10.12 — with a package tool installed for this test, since the machine arrived with none — `pip install bpy` gave `ERROR: Could not find a version that satisfies the requirement bpy (from versions: none)`. Asking the index to resolve for other Python versions gave nothing for 3.10 and 3.12, and a file for 3.11 and for 3.13. So what a computer needs for this kind is a Python of one exact version — **3.11 for 4.5.12** — and a working package tool. A macOS Intel file is published for this version and for no other of the four — the same split the download server shows for the disk images.

### The macOS disk image opens with nobody clicking

On the Creator's Mac, with no window open and no person:

```
hdiutil attach blender-4.5.12-macos-arm64.dmg
...
/dev/disk4s1        	Apple_HFS                      	/Volumes/Blender
attach exit=0
cp -R "/Volumes/Blender/Blender.app" <a working folder>     cp exit=0
hdiutil detach "/Volumes/Blender"                          "disk4" ejected.   detach exit=0
<copied app>/Contents/MacOS/Blender --version
Blender 4.5.12 LTS
	build hash: 84afd5f785f7
	build platform: Darwin
```

No password was asked for at any step and `sudo` was never used. The account it ran as is in the `admin` group, so this settles that no elevation is *requested* and leaves what a standard account would meet untested. The mounted volume holds four things: a symbolic link to `/Applications`, a `.DS_Store`, a `.background.tif`, and `Blender.app`. The application is 836,112,384 bytes against a 308,255,218-byte download.

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

Neither package holds both processors. The Intel build also ran on that Mac, printing `Blender 4.5.12 LTS` and `build hash: 84afd5f785f7` at exit 0 — but only on a second attempt, the first having printed nothing for two minutes before my own time limit cut it off. **Rosetta was already on that Mac and was read rather than installed**, so what a wrong-processor package does on a Mac without it is not settled. For 4.5.12, both packages are published and this is the one that could be tried.

### What this settles about disk space

```
Linux archive:  download 377,902,364 bytes   →  unpacked 1,169,157,971 bytes
macOS image:    download 308,255,218 bytes   →  application 836,112,384 bytes
source archive: download 85,105,056 bytes   →  unpacked 304,479,424 bytes
```

A binary package roughly triples when opened, and the machine needs room for the download and the result at once. No figure of this kind is in the sources.

### Outside the interface

Every unpacking step described here happens with Blender not yet running, so none of it goes through Blender's own window in the first place — the manual's instructions are addressed to a person using the operating system's file tools. Whether the same holds for a machine driven from outside, with no person clicking, is **documented** in one direction and **inferred** in the rest.

Documented: "deployment can be automated using standard file management tools" (deploying page, line 13). That is an explicit statement that automated unpacking is a supported route.

Inferred: that the specific constraints carry over unchanged — that a script on Linux must also use tar and not 7-zip, that an unattended Windows installer run still needs administrator rights, that a script on macOS must still mount the disk image. The manual states each constraint as a property of the package, without reference to who or what performs the step, and I am supplying the step "a constraint stated about the package does not depend on whether a person or a script triggers it." The sources nowhere describe an unattended or command-line install, a silent-install switch, or a scripted mount, so none of that is documented.

One point does differ inside versus outside, and it splits across two labels. **Documented:** the manual's macOS and Linux instructions are written as double-clicking ("double-click on the dmg-file"; "Blender can now be launched by double-clicking the executable"), which is a gesture at a screen. **Not in the sources:** any equivalent for doing the same thing with no screen and no person. The same split holds for the Steam and Microsoft Store routes, which are described only as things done in a client's own window, with no other route stated anywhere I searched.

**Brought up to date by hand tests, 2026-08-30 — label: observed.** This line said the sources give no way to unpack without a person, and for two of the three operating systems that is now answered by running it. **On the Creator's Mac** the whole macOS route was driven from a terminal with no window and nobody clicking: `hdiutil attach`, `cp -R` the application out, `hdiutil detach`, then the copied Blender printed its version — for all four versions, with no prompt and no elevation. A disk image marked by hand as downloaded from the internet went the same way. **On a rented Linux machine** the archive was fetched, verified against its published checksum, opened with `tar`, and started, all from a terminal on a machine with no display server at all — and an ordinary user account did it too, so no administrator account is involved. What is still not settled from outside is the Windows half: whether the Windows installer file can be run with nobody present. Those tests are written out ready to run in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*.

### Operating system

This answer is almost entirely per-operating-system and I have set it out that way above. In summary: administrator rights are a Windows-installer requirement and explicitly not a Windows-zip requirement; disk-image mounting is macOS only; the tar-not-7-zip constraint is stated on the Linux page only and nowhere else; package manager and Snap are Linux only; the Microsoft Store is Windows only; the Steam page names no platform at all, saying only "for your operating system". The sources document Windows, macOS and Linux and no other operating system, so for anything else the answer is not in the sources.

**Brought up to date by hand tests, 2026-08-30 — label: observed.** Two of the three operating systems are now answered by running, and the third is written out ready to run.

**Linux, on a rented Linux machine in Romania** described in the observed section above. `tar` **on its own is not enough**: a machine with GNU tar 1.34 and no `xz` refused all four archives with `tar (child): xz: Cannot exec: No such file or directory` and extracted nothing. Installing one package, `xz-utils`, made all four open with no message. The archive is a tar inside xz compression. The unpacked program is marked runnable straight out of the archive, needs no administrator account, and **will not start** on that machine until nine further library packages are installed — the first thing it asks for is `libX11.so.6`, and `ldd` prints ten lines naming nine distinct libraries. *(Citation check, 2026-08-30 — Claude Opus 5: changed from “ten missing libraries” to match the listing.)* 7-zip reports success twice and produces a Blender that cannot run, in all four versions. `snap install blender --classic` on a machine with no snap is `command not found`. Ubuntu 22.04's own package is Blender 3.0.1.

**macOS, on the Creator's Mac** described in the observed section above. The disk image mounts from a terminal with no prompt and no elevation, the application copies out, and it runs — for all four versions. The application is signed and notarised and `spctl` accepts it. A quarantine mark stops neither the mount nor the copy nor a terminal run. A package for the wrong processor mounts and copies out just the same.

**Windows is still untested, and it is not untestable.** This project has a route to a Windows computer — a rental described in the equipment section of `Preparation/Blender Research/Method.md` — and the Windows entries are written out in full, ready to run, in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*. **Any answer taken there must name the Windows edition it was taken on**: every Windows machine this project has read turned out to be a Server edition whatever its image was called, and a finding that says only "Windows" is not an answer. What is already settled about Windows without such a machine, from the download server alone, is the published file kinds: `.msi`, `.msix` and `.zip`, for each of an x64 and an arm64 processor.

---

## Where I looked

Searches were run across the whole of `manual_4.5`, `api_4.5` and `release_notes_4.5` at once unless stated.

**Terms searched, whole-source:** `7-zip`, `7zip`, `tar`, `.tar`, `tar.xz`, `tar.bz2`, `tarball`, `TAR`, `decompress`, `unpack`, `unzip`, `extract`, `archive`, `dmg`, `disk image`, `.msi`, `.exe`, `.pkg`, `.zip`, `installer`, `administrator`, `admin rights`, `xz`, `hdiutil`, `zip-file`, `zip file`, `glibc`, `Downloading Blender`, `104070`, `self contained`, `self-contained`, `Portable Installation`, `prerequisite`, `dependenc`, `Visual C`, `redistributable`, `Gatekeeper`, `quarantine`, `notariz`, `signature`, `Security and Privacy`, `compile`, `Build from Source`, `build Blender`, `source code`, `Microsoft Store`, `Snap`, `Steam`, `package`, `distribut`, `arm64`, `aarch64`, `Windows on ARM`.

**Pages opened in full:**
- https://docs.blender.org/manual/en/4.5/getting_started/installing/index.html
- https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html
- https://docs.blender.org/manual/en/4.5/getting_started/installing/macos.html
- https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html
- https://docs.blender.org/manual/en/4.5/getting_started/installing/steam.html
- https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html
- https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html
- https://developer.blender.org/docs/release_notes/4.5/ (the version's own page)
- `Blender Sources/requirements/requirements_2026-08-01.txt`

**Ruled out, and why:**
- **The whole Python API documentation.** Searching it for `disk image`, `dmg-file`, `Windows Installer`, `7-zip` and `tarball` returned nothing. The only `.tar` hit is an unrelated file-extension example in `bpy.path`. The API describes what runs inside Blender, so it cannot describe what must exist before Blender is unpacked.
- **The 4.5 release notes, all nineteen files.** Searched for `zip`, `tar`, `dmg`, `msi`, `install`, `package`, `distribut`, `Microsoft Store`, `Snap`, `Steam`, `arm64`. The only install-related hits are a Cycles note about an Ubuntu ray-tracing package (`release_notes_4.5/cycles.md`, lines 38–43), which is about GPU rendering and not about unpacking, and the compatibility bullets on the index page about macOS Intel deprecation, Big Endian, Collada and OptiX driver versions — none of which touch unpacking.
- **Cycles' own files** (`Blender Sources/cycles_4.5`). These are the add-on's Python files and its `source`, `shader` and `license` folders, taken from an installed Blender. They describe render settings; there is nothing there about how a Blender download is packaged or opened. Nothing quotable.
- **`manual_4.5/troubleshooting/`** — the `administrator` hit in `network.txt` is about system administrators allowing a connection to the extensions site; `file_paths.txt`'s hit is about running the Blender executable as administrator on Windows and its effect on paths. Neither concerns unpacking.
- **`manual_4.5/contribute/manual/getting_started/local_editing/install/windows.txt`** — matched on `installer`, but it is about installing tools to edit the manual, not about Blender builds.
- **`manual_4.5/getting_started/configuration/hardware.txt`** — matched on `compiled`, but it is about hardware support requiring a source build, with no build requirements given.
- **Pictures.** I checked whether any of the install pages, the deploying page or the layout page carry a picture. Only the deploying page does, `_images/advanced_deploying-blender_system-extensions.png`, and its caption is "System repository" — it shows an Extensions preference, not anything about unpacking. I did not open it further, and no picture in the sources bears on this question.
- **A "Downloading Blender" page.** All three platform pages point to one, but no such page exists in the manual copy — I searched the whole manual folder for any file with "download" in its name and found none. It is an off-site page on blender.org, and I did not go there.

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

2026-08-29 — Claude Opus 5 (Claude Code) — Builds, Blender 4.5
