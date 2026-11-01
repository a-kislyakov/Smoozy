# Builds — Open Threads questions (Blender 5.2)

**Sources consulted, and when.** Blender's manual for 5.2 (`Blender Sources/manual_5.2`, snapshot taken 2026-07-30), Blender's Python API documentation for 5.2 (`Blender Sources/api_5.2`, snapshot taken 2026-07-30), Blender's 5.2 release notes (`Blender Sources/release_notes_5.2`, snapshot taken 2026-07-30), Cycles' own files (`Blender Sources/cycles_5.2`), and Blender's requirements page (`Blender Sources/requirements/requirements_2026-08-01.txt`, taken 2026-08-01). Read on 2026-08-29.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Answering question 10 only, as instructed.

---

## 10. What does a computer need to have on it before a published Blender build of each kind can be unpacked?

### The question was ambiguous, and here is how I read it

"Each kind" can mean two different things, and the sheet does not settle which.

- **Reading A (the one I answer as the main answer):** the kinds of *package a build is published as* — the Windows zip, the Windows installer file, the macOS disk image, the Linux archive. These are the things that literally get unpacked.
- **Reading B:** the kinds of *release* the manual lists on its download section — Stable Release, Long-term Support, Daily Builds, Build from Source. I answer this reading too, briefly, at the end of the answer, because the manual makes one direct statement about it.

I answer Reading A as the main answer because the question asks what has to be present "before the published thing can be opened at all", and it is the package format, not the release channel, that decides that.

### LABEL: documented — except where a part below is marked inferred or not in the sources, and each of those carries its own mark

### Answer, package kind by package kind

**Windows — zip-file.** The manual says the file has to be extracted by hand, and says explicitly that administrator rights are *not* needed. It does not name any extraction program.

> "Download the zip-file or Windows Installer File."
> — Blender manual, *Installing on Windows*, line 6

> "When choosing the zip-file, you have to manually extract Blender to the desired folder, where you can double-click the executable to run Blender."
> — Blender manual, *Installing on Windows*, line 20

> "No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."
> — Blender manual, *Installing on Windows*, lines 23–24

Source: Blender manual. Address: https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html — date read 2026-07-30.

**Windows — Windows Installer File.** The manual states one prerequisite outright: administrator rights.

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."
> — Blender manual, *Installing on Windows*, lines 15–16

Source: Blender manual. Address: https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html — date read 2026-07-30.

**Windows — both package kinds, one further prerequisite.** The downloaded file must be compatible with the machine's processor architecture.

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture."
> — Blender manual, *Installing on Windows*, lines 10–11

Source: Blender manual. Address: https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html — date read 2026-07-30.

**macOS — disk image (dmg).** The manual describes mounting the image by double-clicking it and names no additional program.

> "Blender for macOS is distributed as disk images (dmg-files). To mount the disk image, double-click on the dmg-file. Then drag Blender.app into the Applications folder."
> — Blender manual, *Installing on macOS*, lines 13–14

Source: Blender manual. Address: https://docs.blender.org/manual/en/5.2/getting_started/installing/macos.html — date read 2026-07-30.

**macOS — the machine itself.** The manual states which Macs this version line supports. **Note the limit:** it is a statement about support, not about unpacking, and the page does not say whether a mismatched Mac can still mount the disk image. *(Citation check, 2026-08-30 — Claude Opus 5: changed — this read "a hard machine prerequisite for this version line", which the quote does not carry for unpacking.)*

> "Blender 5.0 and later only supports Macs with Apple silicon."
> — Blender manual, *Installing on macOS*, line 8

Source: Blender manual. Address: https://docs.blender.org/manual/en/5.2/getting_started/installing/macos.html — date read 2026-07-30.

The requirements page repeats this and adds an operating system floor. That page carries no version in its address, so I quote it only because it names the version range in so many words:

> "Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later."
> — Blender's requirements page, `Blender Sources/requirements/requirements_2026-08-01.txt`, in the macOS section, the note under the table — date read (file taken) 2026-08-01

That same page gives, for Linux, a minimum of "Distribution with glibc 2.28 or newer (64-bit)", and for Windows a minimum of "Windows 8.1 (64-bit)". Two limits on those two figures. They are stated as requirements for Blender, not specifically for unpacking it. And the page names no version alongside them, so — unlike the macOS line above, which names 5.0 and later in so many words — they cannot be claimed for 5.2. They describe whichever release was current when the page was taken.

**Linux — the downloaded archive.** This is the one place the sources name a specific tool the computer must have.

> "Download the Linux version for your architecture and decompress the file to the desired location (e.g. ~/software or /usr/local)."
> — Blender manual, *Installing on Linux*, lines 8–9

> "Archive Extraction
> Extracting Blender's archive using 7-zip is not supported. TAR must be used instead. For more details, see issue #104070."
> — Blender manual, *Installing on Linux*, under "Known Limitations", lines 99–102

So on Linux the computer needs TAR, and using 7-zip instead is documented as not supported. Note what this does *not* say: it does not name the compression format of the Linux archive, and it does not say what goes wrong when 7-zip is used.

Source: Blender manual. Address: https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html — date read 2026-07-30.

**Linux — Snap.** Where a build is taken from Snap rather than from blender.org, the manual states the prerequisite plainly.

> "Assuming snap is already installed, Blender can be installed through snap with:
> snap install blender --classic"
> — Blender manual, *Installing on Linux*, lines 36–38

Source: Blender manual. Address: https://docs.blender.org/manual/en/5.2/getting_started/installing/linux.html — date read 2026-07-30.

**Any platform — Steam.** The manual states the prerequisite as the first step.

> "Download and install the Steam client for your operating system."
> — Blender manual, *Installing from Steam*, line 6

Source: Blender manual. Address: https://docs.blender.org/manual/en/5.2/getting_started/installing/steam.html — date read 2026-07-30.

**Windows — Microsoft Store.** The manual gives the steps but never states a prerequisite:

> "Blender can be installed from the Microsoft Store by searching for Blender in the Microsoft Store and installing it."
> — Blender manual, *Installing on Windows*, in the "Install from Microsoft Store" section

That the Microsoft Store application must be present on the machine is **inferred**, not documented. The step I took: the manual's only instruction is to search inside the Microsoft Store, and you cannot search inside an application that is not there. The manual itself never lists this as a requirement.

Source for the quote: Blender manual. Address: https://docs.blender.org/manual/en/5.2/getting_started/installing/windows.html — date read 2026-07-30.

### Two things the sources say that bear on this and are worth having

The deployment page states that the downloads are self-contained and that deployment can be automated with standard file management tools:

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."
> — Blender manual, *Deploying Blender in Production*, under "Installing Blender"

Source: Blender manual. Address: https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html — date read 2026-07-30.

The directory layout page confirms that what comes out of the Windows zip and the Linux archive is a folder holding a `5.2` directory, and that the macOS build is an application bundle:

> "Linux
> Archive downloaded from blender.org:
> ./5.2/
> macOS
> ./Blender.app/Contents/Resources/5.2/
> Windows
> Zip file downloaded from blender.org:
> ./5.2/
> Installer downloaded from blender.org:
> %ProgramFiles%\Blender Foundation\Blender\5.2\"
> — Blender manual, *Blender's Directory Layout*, under "System Directories"

Source: Blender manual. Address: https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html — date read 2026-07-30.

### Reading B — the release kinds

For the release kinds, the manual makes one statement directly on point, and it says the answer does not change between them:

> "The procedure for installing a binary, either the latest stable release or a daily build, is the same. Follow the steps for your platform listed below."
> — Blender manual, *Installing Blender*, lines 56–57

Source: Blender manual. Address: https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html — date read 2026-07-30.

Long-term Support is described on that same page as a package alongside Stable Release and Daily Builds, and the page gives no separate installation procedure for it. That the platform steps are therefore the only ones offered for it is **inferred**, not documented. The step I took: the quoted sentence names only the stable release and a daily build, so a reader has to carry it across to Long-term Support on the strength of the page saying nothing else about installing that package.

**Build from Source is different, and the sources do not answer it.** The manual says the source code is available and that normal users are not expected to compile Blender, and then stops:

> "Blender's source code is available for free to either reference or to build and use. While normal users are not expected to compile Blender, it does have advantages"
> — Blender manual, *Installing Blender*, lines 44–45

What a computer must have on it to build Blender from source is **not in the sources**. See the record of looking below.

### Not covered

This answer does not settle:

- **What extraction program Windows needs.** The manual says to extract the zip and never names a tool, nor says whether the operating system's built-in extraction is enough.
- **What the Linux archive's compression format is.** "TAR must be used" names the tool; the manual never names the file's extension or compression.
- **Why 7-zip fails on Linux, or what the damage looks like.** The manual points at an issue number and says nothing more. I did not follow that link — it is outside the sources.
- **Whether TAR is also the rule for anything other than the Linux archive.** The limitation appears only on the Linux page.
- **Anything about code signing, notarization, Gatekeeper, or quarantine on macOS.** The macOS page mentions only that "macOS will request your approval before opening Blender for the first time" depending on Security and Privacy preferences — and that is about *opening* the application, not about mounting or unpacking the disk image. (Source for that quote: Blender manual, *Installing on macOS*. Address: https://docs.blender.org/manual/en/5.2/getting_started/installing/macos.html — date read 2026-07-30.)
- **Any runtime library prerequisite for unpacking** — no Visual C++ redistributable, no glibc version tied to extraction, nothing of the sort appears anywhere I searched.
- **Disk space, free space, or file system requirements.** Nothing.
- **Checksums or signatures.** The sources never mention verifying a download before unpacking it.
- **What a Daily Build is published as, per platform.** The manual describes daily builds as a package kind but never lists their file formats separately.
- **Build from Source requirements entirely.**
- **One page the search reached and the record never closes.** The whole-manual search for `installer` returned `advanced/command_line/launch/windows.txt` among its hits. That page was neither opened in full nor ruled out below, so nothing here establishes whether it bears on this question. Treat that one hit as open, not as searched and cleared.

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

Partly documented, and only partly.

Documented: the manual states that downloads "can be extracted to any directory on the system" and that deployment "can be automated using standard file management tools" (*Deploying Blender in Production*, quoted above). That the automation therefore covers the extraction step itself is **inferred**, not documented — the step I took is joining those two sentences, which sit beside each other in the same passage but are not one statement. The Linux Snap and the Linux archive routes are both given as command-line or file-level operations. The TAR requirement and the "requires administrator rights" requirement are both stated without reference to anyone clicking anything, so they read as properties of the package, not of the interface.

Not in the sources: the manual describes the Windows zip, the Windows installer file and the macOS disk image in terms of double-clicking. It never says how to mount a disk image, run the Windows installer, or extract the zip without a person doing it, and it never says whether the prerequisites differ when those steps are driven by a script. In particular, the sources say nothing about a silent or unattended mode for the Windows installer, and nothing about mounting a dmg from a command line. Hold the first of those loosely: the search that stands behind it left one page unaccounted for, as the *Not covered* list says. This is a real gap for anyone driving installation from outside Blender's window.

**Brought up to date by hand tests, 2026-08-30 — label: observed.** This line said the sources give no way to unpack without a person, and for two of the three operating systems that is now answered by running it. **On the Creator's Mac** the whole macOS route was driven from a terminal with no window and nobody clicking: `hdiutil attach`, `cp -R` the application out, `hdiutil detach`, then the copied Blender printed its version — for all four versions, with no prompt and no elevation. A disk image marked by hand as downloaded from the internet went the same way. **On a rented Linux machine** the archive was fetched, verified against its published checksum, opened with `tar`, and started, all from a terminal on a machine with no display server at all — and an ordinary user account did it too, so no administrator account is involved. What is still not settled from outside is the Windows half: whether the Windows installer file can be run with nobody present. Those tests are written out ready to run in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*.

### Operating system

No, and the whole answer is organised by that. Every prerequisite above is specific to one operating system:

- Windows: architecture-matched download; administrator rights for the installer file, none for the zip.
- macOS: an Apple silicon Mac (5.0 and later), macOS 13 or later per the requirements page; nothing named for mounting the image itself.
- Linux: an architecture-matched download and TAR; 7-zip documented as not supported.

The release-kind statement ("the procedure for installing a binary, either the latest stable release or a daily build, is the same") is the only part that holds across all three, and even it defers to "the steps for your platform".

**Brought up to date by hand tests, 2026-08-30 — label: observed.** Two of the three operating systems are now answered by running, and the third is written out ready to run.

**Linux, on a rented Linux machine in Romania** described in the observed section above. `tar` **on its own is not enough**: a machine with GNU tar 1.34 and no `xz` refused all four archives with `tar (child): xz: Cannot exec: No such file or directory` and extracted nothing. Installing one package, `xz-utils`, made all four open with no message. The archive is a tar inside xz compression. The unpacked program is marked runnable straight out of the archive, needs no administrator account, and **will not start** on that machine until nine further library packages are installed — the first thing it asks for is `libX11.so.6`, and `ldd` prints ten lines naming nine distinct libraries. *(Citation check, 2026-08-30 — Claude Opus 5: changed from “ten missing libraries” to match the listing.)* 7-zip reports success twice and produces a Blender that cannot run, in all four versions. `snap install blender --classic` on a machine with no snap is `command not found`. Ubuntu 22.04's own package is Blender 3.0.1.

**macOS, on the Creator's Mac** described in the observed section above. The disk image mounts from a terminal with no prompt and no elevation, the application copies out, and it runs — for all four versions. The application is signed and notarised and `spctl` accepts it. A quarantine mark stops neither the mount nor the copy nor a terminal run. A package for the wrong processor mounts and copies out just the same.

**Windows is still untested, and it is not untestable.** This project has a route to a Windows computer — a rental described in the equipment section of `Preparation/Blender Research/Method.md` — and the Windows entries are written out in full, ready to run, in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*. **Any answer taken there must name the Windows edition it was taken on**: every Windows machine this project has read turned out to be a Server edition whatever its image was called, and a finding that says only "Windows" is not an answer. What is already settled about Windows without such a machine, from the download server alone, is the published file kinds: `.msi`, `.msix` and `.zip`, for each of an x64 and an arm64 processor.

---

## Record of where I looked

**Files opened in full:**
- `manual_5.2/getting_started/installing/index.txt`
- `manual_5.2/getting_started/installing/windows.txt`
- `manual_5.2/getting_started/installing/macos.txt`
- `manual_5.2/getting_started/installing/linux.txt`
- `manual_5.2/getting_started/installing/steam.txt`
- `manual_5.2/getting_started/installing/linux_windowing_environment.txt`
- `manual_5.2/advanced/deploying_blender.txt`
- `manual_5.2/advanced/blender_directory_layout.txt`
- `release_notes_5.2/index.md`
- `release_notes_5.2/core.md` (opening section)
- `requirements/requirements_2026-08-01.txt`

**Whole-manual searches run** (case-insensitive where it mattered, across every `.txt` under `manual_5.2`, excluding the `_images` folder for terms that matched inside SVG files):

`7-zip`, `7z`, `TAR`, `decompress`, `extract`, `unzip`, `unpack`, `archive`, `tar.xz`, `zip`, `zip-file`, `dmg`, `disk image`, `msi`, `installer`, `administrator`, `admin rights`, `Gatekeeper`, `notariz`, `quarantine`, `code sign`, `codesign`, `signed`, `Visual C`, `redistributable`, `glibc`, `prerequisite`, `dependencies`, `hdiutil`, `mount`, `Downloading Blender`, `Security and Privacy`, `double-click`, `CMake`, `compile Blender`, `build from source`, `source code`, `arm64`, `x64`, `Apple silicon`, `architecture`.

**What came back empty, and what that rules out:**
- `Gatekeeper`, `notariz`, `quarantine`, `code sign`, `codesign`, `hdiutil` — zero hits anywhere in the 5.2 manual. This is the basis for saying the sources are silent on macOS signing and on scripted disk-image mounting.
- `redistributable`, `glibc`, `tar.xz` — zero hits anywhere in the 5.2 manual. `glibc` appears only on the requirements page, and there as a requirement for Blender, not for unpacking.
- `CMake` — zero hits. Combined with `build from source` appearing only as a heading on the install index, this is the basis for answering "not in the sources" for build-from-source requirements. The 5.2 manual contains no build instructions of any kind.
- `7-zip` — exactly one hit in the whole manual, the Linux known limitation quoted above.
- `dmg` and `disk image` — hits only on `getting_started/installing/macos.txt` (plus unrelated matches inside `_images/*.svg`, which are drawings of the video sequencer interface, not documentation).
- `installer` — hits only on `getting_started/installing/windows.txt`, `advanced/blender_directory_layout.txt`, `advanced/command_line/launch/windows.txt`, and `contribute/manual/getting_started/local_editing/install/windows.txt`. The last of these is about installing tools for editing the manual itself, not about Blender builds, so I ruled it out as off-topic.

**Pictures:** I searched `getting_started/` for `[image:` markers. None of the installation pages carry a picture — the only images under `getting_started/` are on the help, hardware configuration, defaults, license and about pages, none of which bear on this question. There is one image named `about_contribute_install_windows_installer.png`, but it belongs to the manual's own contributor tooling section, not to Blender builds, so I did not treat it as relevant.

**Release notes:** searched all 5.2 release note files for `install`, `zip`, `dmg`, `archive`, `extract`, `package`, `glibc`, `macOS`, `Windows`, `arm64`, `architect`, `requirement`, `portable`, `msi`, `tar`. Every hit was unrelated — bug fix titles mentioning an operating system, a Liquid Glass application icon for macOS 26, geometry node names containing "extract", library paths containing "is_archive". Nothing in the 5.2 release notes touches packaging or install prerequisites.

**Python API documentation:** searched all of `api_5.2` for `zip-file`, `dmg`, `disk image`, `tar.xz`, `windows installer`, `extract Blender`, `unpack Blender`. Zero hits. The API documentation does not describe how builds are packaged.

**Cycles' own files:** listed `cycles_5.2` (it holds the Cycles add-on's Python files and its `source` and `shader` folders) and searched it for `.dmg`, `tar.xz`, `installer`, `unzip`. Zero hits. Cycles' files say nothing about how Blender is packaged or unpacked, which is what I would expect — they are the add-on's own code.

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

2026-08-29 — Claude Opus 5 (Claude Code) — Builds, Blender 5.2
