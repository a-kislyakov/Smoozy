# Builds — Open Threads questions — Blender 5.0

Sources consulted, and when: Blender's manual for 5.0, Blender's Python API documentation for 5.0, and the Blender 5.0 release notes, all as the plain-text snapshot taken **2026-07-30**; Cycles' own files for 5.0 from the same snapshot; and Blender's requirements page, the snapshot taken **2026-08-01** (that page carries no version in its address, so it is only used here where it names a version in so many words). I examined these files on 2026-08-29.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Answering question 10 only, as instructed.

---

## 10. What does a computer need to have on it before a published Blender build of each kind can be unpacked?

**Short answer:** the sources answer this by platform and by package form, not by build kind. Across the kinds of build (stable, daily), the manual says the procedure is the same. What has to be present before the downloaded thing can be opened is: on Linux, **TAR** — the manual states outright that 7-zip is not supported; on Windows with the installer, **administrator rights**; on Windows with the zip-file, a way to extract a zip (the manual names no tool) and explicitly *not* administrator rights; on macOS, the sources name nothing beyond double-clicking the disk image. For the store-delivered forms, the store or package client must already be there — said outright for Steam and for snap, and a step the reader supplies for the Microsoft Store (see 10f). For the source code, the sources say nothing.

I read the question as: what must already exist on the machine so that the downloaded package can be opened up / installed at all. An alternative reading — what a machine needs so that the *unpacked Blender will run* — is a different question (that would be the requirements tables: CPU, RAM, GPU, driver, OS version). I have answered the first reading, and note the second here rather than silently choosing.

### 10a. For the binary builds, the kind does not change the answer — the platform does

**Label: documented** (for stable and daily). **Inferred** for LTS — see the step written out below.

Exact words:

> "The procedure for installing a binary, either the latest stable release or a daily build, is the same. Follow the steps for your platform listed below."

Source: Blender manual — <https://docs.blender.org/manual/en/5.0/getting_started/installing/index.html>. Date read: 2026-07-30.

The same page lists the kinds it publishes as "Stable Release", "Long-term Support", "Daily Builds" and "Build from Source". The sentence above names only the stable release and a daily build. **Inferred:** Long-term Support is also a binary package (the page calls it "A package designed for long-lasting projects requiring a very stable version of Blender"), so the same platform procedure applies to it. The step a reader has to supply: that "a binary" covers the LTS package too. The page does not say this in so many words.

Exact words for the LTS description:

> "A package designed for long-lasting projects requiring a very stable version of Blender."

Source: Blender manual — <https://docs.blender.org/manual/en/5.0/getting_started/installing/index.html>. Date read: 2026-07-30.

### 10b. Linux, archive downloaded from blender.org — TAR is required

**Label: documented.**

Exact words (under the heading "Known Limitations", sub-heading "Archive Extraction"):

> "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead. For more details, see issue #104070."

And the install step itself:

> "Download the Linux version for your architecture and decompress the file to the desired location (e.g. ~/software or /usr/local)."

Source: Blender manual — <https://docs.blender.org/manual/en/5.0/getting_started/installing/linux.html>. Date read: 2026-07-30.

So the machine must have TAR available. The manual does not name any other tool as acceptable, and names exactly one as not acceptable.

**Inferred:** because "TAR must be used", the Linux download is a tar-format archive. The step supplied: that the tool named as required implies the archive's format. The 5.0 manual never states the file extension of the Linux download; it only calls it "Blender's archive" and "the file".

The manual also requires that the download match the machine's architecture ("the Linux version for your architecture"), but does not say the architecture affects extraction.

### 10c. Windows, Installer File — administrator rights

**Label: documented.**

Exact words:

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

Source: Blender manual — <https://docs.blender.org/manual/en/5.0/getting_started/installing/windows.html>. Date read: 2026-07-30.

Nothing else is named as a prerequisite for running the installer. The manual does not name a Windows version requirement on this page, does not mention any runtime or framework the installer needs, and does not describe a silent or unattended mode.

### 10d. Windows, zip-file — an extraction step, and no administrator rights

**Label: documented.**

Exact words:

> "When choosing the zip-file, you have to manually extract Blender to the desired folder, where you can double-click the executable to run Blender."

> "No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."

Source: Blender manual — <https://docs.blender.org/manual/en/5.0/getting_started/installing/windows.html>. Date read: 2026-07-30.

The manual names **no** tool for the extraction, and states no restriction of the kind it states for Linux.

**Inferred:** something able to extract a zip must be present. The step a reader has to supply: that extraction done by hand needs a tool that can do it. The quoted words say the extraction is manual; they do not say anything has to be on the machine beforehand. **Not in the sources:** which tool, and whether any tool is unsuitable on Windows the way 7-zip is called out on Linux.

Also documented on the same page, and a precondition of choosing the right file rather than of unpacking it:

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture."

### 10e. macOS, dmg — the disk image is mounted, nothing else is named

**Label: documented.**

Exact words:

> "Blender for macOS is distributed as disk images (dmg-files). To mount the disk image, double-click on the dmg-file. Then drag Blender.app into the Applications folder."

> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."

Source: Blender manual — <https://docs.blender.org/manual/en/5.0/getting_started/installing/macos.html>. Date read: 2026-07-30.

Note that the second quote is about *opening Blender for the first time*, not about mounting or copying — the manual places the approval after installation, not before it. No tool, no rights, and no other prerequisite are named for mounting the disk image or for the drag.

Two further documented facts bear on macOS for this version, and both concern what machine the build will run on rather than what unpacks it:

> "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later."

Source: Blender's requirements page, snapshot file `requirements/requirements_2026-08-01.txt`, in the "macOS" section. That page carries no version in its address; I cite it here only because it names 5.0 outright. Date read: 2026-08-01.

> "* [Support for Intel Macs has been removed](https://devtalk.blender.org/t/deprecation-and-removal-of-macos-intel-builds-in-blender-5-0/38835)"

Source: release notes — <https://developer.blender.org/docs/release_notes/5.0/>, under "Compatibility". Date read: 2026-07-30.

I record a conflict rather than resolving it: the 5.0 manual's macOS page still says "Blender supports both Intel and Apple Silicon architectures on macOS. Make sure to download a variant that is compatible with your CPU's architecture" (<https://docs.blender.org/manual/en/5.0/getting_started/installing/macos.html>, date read 2026-07-30), while the release notes for 5.0 say Intel Mac support has been removed and the requirements page says 5.0 and later require Apple Silicon. Both are in my sources. I report both and do not pick.

### 10f. The store- and package-manager-delivered forms — the client must already be there

**Label: documented** for the quoted words below, and for the prerequisite in the two places a quote states it: Steam names installing the client, and the snap line says "Assuming snap is already installed".

**Inferred** for two steps. That nothing is unpacked by hand in these routes — the quotes describe installing through a client and say nothing either way about unpacking. And that the Microsoft Store client must already be there — its quote says Blender can be installed from the store, not that the store has to be present first.

Steam:

> "Download and install the Steam client for your operating system."

> "Once it is installed, open the client and login to your Steam account, or create an account if you don't already have one."

Source: Blender manual — <https://docs.blender.org/manual/en/5.0/getting_started/installing/steam.html>. Date read: 2026-07-30.

Snap on Linux:

> "Assuming snap is already installed, Blender can be installed through snap with:"
> `snap install blender --classic`

Source: Blender manual — <https://docs.blender.org/manual/en/5.0/getting_started/installing/linux.html>. Date read: 2026-07-30.

Microsoft Store on Windows:

> "Blender can be installed from the Microsoft Store by searching for Blender in the Microsoft Store and installing it."

Source: Blender manual — <https://docs.blender.org/manual/en/5.0/getting_started/installing/windows.html>. Date read: 2026-07-30.

Linux distribution packages: the manual says only that "Some Linux distributions may have a specific package for Blender in their repositories" and that installing that way "ensures consistency with other packages on the system"; it names no prerequisite beyond the distribution's own package manager (same page as the Snap quote).

### 10g. Build from Source

**Label: not in the sources.**

The manual's "Build from Source" section says the source code is available and lists reasons a person might use it, but says nothing at all about what a machine needs in order to obtain, unpack or compile it. Exact words of the whole of what it does say on the matter:

> "Blender's source code is available for free to either reference or to build and use. While normal users are not expected to compile Blender, it does have advantages:"

Source: Blender manual — <https://docs.blender.org/manual/en/5.0/getting_started/installing/index.html>. Date read: 2026-07-30.

Where I looked for more: searched the whole manual, the whole API documentation and the whole of the 5.0 release notes for `build from source`, `compile Blender`, `building blender` and `source code`. Every hit outside that one section is about something else — the licence text, node source code in the interface, GLSL shader source, commit guidelines for the manual. Nothing in the 5.0 documentation snapshot describes build prerequisites.

### 10h. What the sources say about extracting, and the one place they say something is not needed

**Label: documented** for the words quoted below — with one caution. The first quote does **not** say anything is unnecessary; it says where a download may be put and that deployment can be automated. Only the requirements-page passage says something is "not needed", and that page names no version. *(Citation check, 2026-08-30 — Claude Opus 5: changed — the heading read "What the sources say is *not* needed" under a flat documented label; narrowed to what the quotes carry.)*

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

Source: Blender manual — <https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html>. Date read: 2026-07-30.

> "Whether it's on a USB stick, sitting on a folder on your desktop, or fully installed, Blender runs out of the box. No installation needed, no Internet connection required."

Source: Blender's requirements page, snapshot file `requirements/requirements_2026-08-01.txt`, "Truly Portable" section. This passage names no version, so it can only be taken as the general statement the page makes on the date it was taken, not as a statement about 5.0 specifically. Date read: 2026-08-01.

> "Blender bundles a complete Python installation and does not interact with the system Python by default."

Source: Blender manual — <https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html>. Date read: 2026-07-30. (**Inferred:** no system Python has to be present. The step a reader has to supply: that a bundled Python which is not interacted with by default means none needs to be installed. The quoted words say what Blender carries and what it leaves alone, not what the machine must hold. This is about running rather than unpacking; I include it because it is the one "prerequisite you might expect" the sources come nearest to ruling out.)

### Not covered

- **No named tool for the Windows zip.** The manual says extraction is manual and names nothing that does it, and gives no counterpart to the Linux 7-zip restriction.
- **What "not supported" means on Linux.** The manual says 7-zip "is not supported" and points at an issue number. It does not say whether extraction fails outright, or succeeds but produces a broken installation. I did not follow the issue number — it is outside my sources.
- **Nothing on disk space.** Neither the manual, the release notes nor the requirements page states how much free space a download or an unpacked installation needs.
- **Nothing on verifying a download.** No mention anywhere in the 5.0 manual, API documentation or release notes of checksums, signatures, notarization, Gatekeeper, or quarantine flags. Search terms and results are in the record below.
- **Nothing on file permissions after extraction.** No mention of having to mark the executable runnable on Linux or macOS.
- **Nothing about the daily builds specifically.** The manual describes them as a kind, and says the binary installation procedure is the same, but says nothing separate about their packaging.
- **Nothing about the Linux runtime prerequisite in relation to unpacking.** The manual does state that "Under Gnome-Shell the libdecor library is required" (<https://docs.blender.org/manual/en/5.0/getting_started/installing/linux_windowing_environment.html>), and the requirements page names glibc 2.28 or newer for Linux. Both are about running Blender, not about opening the package. I flag them so nobody mistakes their absence here for their absence from the sources.
- **The version reach of the requirements page.** That page has no versioned address. I have used it only for the passage that names 5.0.

---

## 10 — what was observed by hand, 2026-08-30

**Label: observed.** Everything below was run by me on 2026-08-30 and is recorded with the machine's own words. The full procedure and every raw output are in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`. Nothing here is reasoning; where a test settled something the answer above left open, that is said in the entry itself.

**The two machines, named in full, because that is what settles which operating system each answer covers.**

**The Creator's own Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no second card, arm64.

**A rented Linux machine, in Romania** — Vast.ai instance `49253756` from offer `43521340`, machine `124623`, labelled `smoozy-builds-q10-20260830`, reached over the provider's proxy at `root@ssh5.vast.ai` port `13756`, $0.0644 an hour, rented and destroyed the same day. **A container, not a whole machine**, established rather than assumed: `/.dockerenv` present, `systemd-detect-virt` says `docker`, process 1 is the provider's own launch script, `/proc/1/cgroup` prints `0::/`. Ubuntu 22.04.4 LTS, kernel `5.15.0-186-generic`, x86-64, glibc 2.35, from the image `nvidia/cuda:12.4.1-runtime-ubuntu22.04`. An Intel Xeon W-2123 with eight visible threads against a quota of 3.84 cores; a memory ceiling of 22,530,752,512 bytes where the ordinary reading shows the host's 33,191,886,848; one NVIDIA Quadro P4000 with 8192 MiB on driver `580.173.02`, which nothing here used; a 40 GiB overlay root; **no display server**.

**It was rented deliberately bare, and that is what makes these answers mean anything.** When it arrived it had 253 installed packages: `tar` (GNU tar 1.34) and `gzip`, and **no `xz`, no `unzip`, no `zip`, no `7z`, no `bzip2`, no `zstd`, no `file`, no `snap` and no `pip3`**. **None of the libraries Blender links** was in its library index. Nothing named after Blender was anywhere on it.

**All four Blenders on it matched the Creator's Mac to the exact build hash** — `84afd5f785f7`, `a3db93c5b259`, `ec6e62d40fa9`, `9e2066aef7ef` — so a result from one machine can be set beside a result from the other.

### What Blender actually publishes for 5.0.1

Read from Blender's own download server, `https://download.blender.org/release/Blender5.0/`, on the Creator's Mac:

```
blender-5.0.1-linux-x64.tar.xz      374,154,072
blender-5.0.1-macos-arm64.dmg        318,854,192
blender-5.0.1-windows-arm64.msi
blender-5.0.1-windows-arm64.msix
blender-5.0.1-windows-arm64.zip
blender-5.0.1-windows-x64.msi
blender-5.0.1-windows-x64.msix
blender-5.0.1-windows-x64.zip
blender-5.0.1.md5
blender-5.0.1.sha256
```

And, from `https://download.blender.org/source/`, `blender-5.0.1.tar.xz` (88,926,056 bytes) with a `blender-5.0.1.tar.xz.md5sum` beside it.

So for this version there is **one Linux package and one processor for it**; **one macOS package**, for Apple Silicon, and none for Intel; **three Windows file kinds for each of two processors** — `.msi`, `.msix` and `.zip`; **a checksum file in two forms**; and **a source archive**. No file name carries a stability word or the letters LTS.

**This settles, for the published files, the disagreement this document records between its own sources.** The manual page says both Intel and Apple Silicon builds exist; the release notes and the requirements page say Intel is gone. What is published for 5.0.1 is one macOS package and it is for Apple Silicon. The same split appears again in the checksum file, which lists no macOS Intel line, and again in the Python module listing below.

### A published checksum exists, and the fetched packages matched it

The `blender-5.0.1.sha256` file was fetched and checked against the packages fetched beside it. On the rented Linux machine: `blender-5.0.1-linux-x64.tar.xz: OK`. On the Creator's Mac: `blender-5.0.1-macos-arm64.dmg: OK`. So verifying a download before opening it is possible, which the answer above records as absent from the sources.

### What the Linux archive is

On the rented Linux machine — which arrived with no `file` program at all, so `file` was installed before this reading could be taken — `file blender-5.0.1-linux-x64.tar.xz` prints:

*(Citation check, 2026-08-30 — Claude Opus 5: changed — the step of installing `file` was missing; a reader following this on the bare machine described above would find the command absent. The first six bytes below were read without it.)*

```
blender-5.0.1-linux-x64.tar.xz: XZ compressed data, checksum CRC64
```

Its first six bytes are `fd 37 7a 58 5a 00`. `tar -tf` on it lists a single top folder, `blender-5.0.1-linux-x64/`. So the archive is a tar inside xz compression — the format the answer above records the manual as never naming.

### `tar` on its own does not open it

The rented machine had GNU tar 1.34 and no `xz` at all. `tar -xf blender-5.0.1-linux-x64.tar.xz -C <empty folder>` gave, word for word:

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
top of the tree: blender-5.0.1-linux-x64
files: 5166   links: 75
unpacked size (bytes): 1182878499
```

*(A run before this one had `/usr/bin/time` in front of the command; that program is not on the machine either, and it gave `exit=127` and `bash: line 18: /usr/bin/time: No such file or directory` with nothing extracted. Both runs are in the hand-test list.)*

### The unpacked Blender does not start on that machine, and what it asks for

With the archive open and only that one extra package installed, `blender --version` gave:

```
exit=127
.../blender-5.0.1-linux-x64/blender: error while loading shared libraries: libX11.so.6: cannot open shared object file: No such file or directory
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
Blender 5.0.1
	build hash: a3db93c5b259
	build branch: blender-v5.0-release
	build platform: Linux
	build type: Release
```

The build hash `a3db93c5b259` is the same one the Creator's Mac carries for this version.

### The program is marked runnable straight out of the archive, and no special account is needed

```
-rwxr-xr-x  ...  blender-5.0.1-linux-x64/blender      links in tree: 75
```

Nobody set that mark. On the same machine an ordinary user account (`uid=1000(plainuser)`) extracted the 5.2.1 archive and ran it — `tar exit=0`, `run exit=0`, the same file and link counts as the root extraction — so unpacking on Linux needs no administrator account.

### 7-zip really cannot be used, and it says it worked

With `7-Zip [64] 16.02` — installed on the machine for this test, since it arrived with no `7z` — two passes over `blender-5.0.1-linux-x64.tar.xz` both printed `Everything is Ok` and exited 0. *(Citation check, 2026-08-30 — Claude Opus 5: changed — the step of installing 7-zip was missing from the procedure.)* What came out will not run:

```
7-zip copy: files=5241  links=0
tar copy:   files=5166  links=75
-rw-r--r--  ...  ./blender-5.0.1-linux-x64/blender
trying to run it:  exit=126   bash: ./blender-5.0.1-linux-x64/blender: Permission denied
after putting the runnable mark back by hand:
  exit=127   .../blender-5.0.1-linux-x64/blender: error while loading shared libraries: .../lib/libtbb.so.12: file too short
```

Two things are wrong every time: the runnable mark is gone, and every symbolic link has become a plain file — 75 links in the tar copy against none in the 7-zip copy, with the matching extra plain files. The same happened in all four versions.

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
blender-5.0.1.tar.xz: XZ compressed data, checksum CRC64
published md5sum check: blender-5.0.1.tar.xz: OK
tar exit=0
top of tree: blender-5.0.1
directly inside it: AUTHORS CMakeLists.txt COPYING GNUmakefile README.md assets build_files doc extern intern lib locale make.bat pyproject.toml
unpacked size (bytes): 303654193
```

So for the source kind, opening the published thing needs exactly what the binary Linux archive needs. What building it needs was not tested and is not settled.

### Blender as a Python module is published for this version, and it needs one exact Python

The package index lists these files for 5.0.1:

```
bpy-5.0.1-cp311-cp311-macosx_11_0_arm64.whl     | requires-python: ==3.11.*
bpy-5.0.1-cp311-cp311-manylinux_2_28_x86_64.whl | requires-python: ==3.11.*
bpy-5.0.1-cp311-cp311-win_amd64.whl             | requires-python: ==3.11.*
bpy-5.0.1-cp311-cp311-win_arm64.whl             | requires-python: ==3.11.*
```

On the rented machine's own Python 3.10.12 — with a package tool installed for this test, since the machine arrived with none — `pip install bpy` gave `ERROR: Could not find a version that satisfies the requirement bpy (from versions: none)`. Asking the index to resolve for other Python versions gave nothing for 3.10 and 3.12, and a file for 3.11 and for 3.13. So what a computer needs for this kind is a Python of one exact version — **3.11 for 5.0.1** — and a working package tool. There is no Linux arm64 file, and no macOS Intel file for this version — the same split the download server shows.

### The macOS disk image opens with nobody clicking

On the Creator's Mac, with no window open and no person:

```
hdiutil attach blender-5.0.1-macos-arm64.dmg
...
/dev/disk4s1        	Apple_HFS                      	/Volumes/Blender
attach exit=0
cp -R "/Volumes/Blender/Blender.app" <a working folder>     cp exit=0
hdiutil detach "/Volumes/Blender"                          "disk4" ejected.   detach exit=0
<copied app>/Contents/MacOS/Blender --version
Blender 5.0.1
	build hash: a3db93c5b259
	build platform: Darwin
```

No password was asked for at any step and `sudo` was never used. The account it ran as is in the `admin` group, so this settles that no elevation is *requested* and leaves what a standard account would meet untested. The mounted volume holds four things: a symbolic link to `/Applications`, a `.DS_Store`, a `.background.tif`, and `Blender.app`. The application is 868,622,336 bytes against a 318,854,192-byte download.

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

Neither package holds both processors. The Intel build also ran on that Mac, printing `Blender 4.5.12 LTS` and `build hash: 84afd5f785f7` at exit 0 — but only on a second attempt, the first having printed nothing for two minutes before my own time limit cut it off. **Rosetta was already on that Mac and was read rather than installed**, so what a wrong-processor package does on a Mac without it is not settled. For 5.0.1, no wrong-processor macOS package exists to try — the download server publishes one macOS file and it is for Apple Silicon.

### What this settles about disk space

```
Linux archive:  download 374,154,072 bytes   →  unpacked 1,182,878,499 bytes
macOS image:    download 318,854,192 bytes   →  application 868,622,336 bytes
source archive: download 88,926,056 bytes   →  unpacked 303,654,193 bytes
```

A binary package roughly triples when opened, and the machine needs room for the download and the result at once. No figure of this kind is in the sources.

### Outside the interface

Blender's own interface is not involved at this point at all — nothing here happens inside Blender, because Blender is not yet on the machine. On the narrower question of whether this holds when the work is driven by something other than a person clicking:

**Documented, in part.** The manual states that deployment can be automated: "deployment can be automated using standard file management tools" (<https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html>). That sentence is about the extract-to-a-directory route, and it is on a page addressed to studios and schools.

**Not in the sources** for the rest. The platform pages describe unpacking in terms of a person acting — "double-click on the dmg-file", "drag Blender.app into the Applications folder", "you have to manually extract Blender". Nothing in my sources says whether the Windows installer can be run without a person, whether a dmg can be mounted without the Finder, or what the administrator-rights requirement means for an unattended run. I searched for `silent`, `unattended`, `msiexec` and command-line installation across the 5.0 manual and found nothing on the subject.

The Linux TAR requirement is the one part that plainly carries over: it names a command-line tool, so it is if anything more natural outside the interface than inside it.

**Brought up to date by hand tests, 2026-08-30 — label: observed.** This line said the sources give no way to unpack without a person, and for two of the three operating systems that is now answered by running it. **On the Creator's Mac** the whole macOS route was driven from a terminal with no window and nobody clicking: `hdiutil attach`, `cp -R` the application out, `hdiutil detach`, then the copied Blender printed its version — for all four versions, with no prompt and no elevation. A disk image marked by hand as downloaded from the internet went the same way. **On a rented Linux machine** the archive was fetched, verified against its published checksum, opened with `tar`, and started, all from a terminal on a machine with no display server at all — and an ordinary user account did it too, so no administrator account is involved. What is still not settled from outside is the Windows half: whether the Windows installer file can be run with nobody present. Those tests are written out ready to run in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*.

### Operating system

**Does not hold across operating systems — it differs by operating system, and that is documented.** The manual has one page per platform and the requirements differ on each:

- Linux: TAR required, 7-zip not supported.
- Windows installer: administrator rights required.
- Windows zip: administrator rights explicitly not required; no tool named.
- macOS: disk image mounted by double-click, then a drag; nothing named as required.
- The store- and package-manager routes are themselves split by operating system — snap and the distribution's own packages on Linux, the Microsoft Store on Windows, Steam on its own page — see 10f.

The 7-zip restriction appears only on the Linux page. Nothing in my sources says whether it applies to any other platform's package, and there is no reason given in the sources to carry it across.

macOS 5.0 additionally requires Apple Silicon and macOS 13 or later per the requirements page and the release notes, with the manual page in tension with that (see 10e).

**Brought up to date by hand tests, 2026-08-30 — label: observed.** Two of the three operating systems are now answered by running, and the third is written out ready to run.

**Linux, on a rented Linux machine in Romania** described in the observed section above. `tar` **on its own is not enough**: a machine with GNU tar 1.34 and no `xz` refused all four archives with `tar (child): xz: Cannot exec: No such file or directory` and extracted nothing. Installing one package, `xz-utils`, made all four open with no message. The archive is a tar inside xz compression. The unpacked program is marked runnable straight out of the archive, needs no administrator account, and **will not start** on that machine until nine further library packages are installed — the first thing it asks for is `libX11.so.6`, and `ldd` prints ten lines naming nine distinct libraries. *(Citation check, 2026-08-30 — Claude Opus 5: changed from “ten missing libraries” to match the listing.)* 7-zip reports success twice and produces a Blender that cannot run, in all four versions. `snap install blender --classic` on a machine with no snap is `command not found`. Ubuntu 22.04's own package is Blender 3.0.1.

**macOS, on the Creator's Mac** described in the observed section above. The disk image mounts from a terminal with no prompt and no elevation, the application copies out, and it runs — for all four versions. The application is signed and notarised and `spctl` accepts it. A quarantine mark stops neither the mount nor the copy nor a terminal run. A package for the wrong processor mounts and copies out just the same.

**Windows is still untested, and it is not untestable.** This project has a route to a Windows computer — a rental described in the equipment section of `Preparation/Blender Research/Method.md` — and the Windows entries are written out in full, ready to run, in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*. **Any answer taken there must name the Windows edition it was taken on**: every Windows machine this project has read turned out to be a Server edition whatever its image was called, and a finding that says only "Windows" is not an answer. What is already settled about Windows without such a machine, from the download server alone, is the published file kinds: `.msi`, `.msix` and `.zip`, for each of an x64 and an arm64 processor.

---

## Record of where I looked

Everything below was run over `Blender Sources/manual_5.0`, `Blender Sources/api_5.0`, `Blender Sources/release_notes_5.0`, `Blender Sources/cycles_5.0` and `Blender Sources/requirements`. I searched the whole of each before opening anything.

**Search terms, case-insensitive, whole-tree:**
`7-zip`, `7zip`, `tar`, `unzip`, `extract`, `extract Blender`, `extracted to`, `decompress`, `unpack`, `unzipped`, `archive`, `zip-file`, `dmg`, `disk image`, `mount`, `.msi`, `msiexec`, `installer`, `administrator`, `Rosetta`, `redistributable`, `glibc`, `.xz`, `gzip`, `compressed`, `checksum`, `sha256`, `signature`, `notariz`, `Gatekeeper`, `quarantine`, `code sign`, `codesign`, `Downloading Blender`, `portable installation`, `USB`, `self contained`, `self-contained`, `package manager`, `snap `, `flatpak`, `Microsoft Store`, `AppImage`, `chmod`, `executable bit`, `permissions`, `disk space`, `free space`, `storage`, `silent`, `unattended`, `build from source`, `compile Blender`, `building blender`, `source code`, `library`, `dependenc`, `prerequisit`, `required to`, `must be installed`, `TAR must`.

**Pages opened in full:**
- `getting_started/installing/index.txt` — the kinds of build, and the sentence that the binary procedure is the same
- `getting_started/installing/linux.txt` — the TAR / 7-zip limitation, the decompress step, Snap, distribution packages
- `getting_started/installing/windows.txt` — installer vs zip, administrator rights, Microsoft Store, architecture
- `getting_started/installing/macos.txt` — dmg, mount, drag, Security and Privacy approval
- `getting_started/installing/steam.txt` — Steam client and account as prerequisites
- `getting_started/installing/linux_windowing_environment.txt` — libdecor; runtime, not unpacking
- `advanced/deploying_blender.txt` — extract to any directory, automation with standard file management tools, bundled Python
- `advanced/blender_directory_layout.txt` — "Archive downloaded from blender.org", "Zip file downloaded from blender.org", "Installer downloaded from blender.org", "in the unzipped folder"
- `advanced/command_line/launch/linux.txt`, `.../macos.txt`, `.../windows.txt` — checked for any unpacking prerequisite; they cover starting an already-installed Blender only
- `requirements/requirements_2026-08-01.txt` — read in full
- `release_notes_5.0/index.md` — read in full; the Compatibility list carries the Intel Mac removal

**Ruled out, and why:**
- Every other `extract` hit in the manual — bundle nodes, sculpt masks, image editing, baking, extensions and Python wheels, Rigify. About features inside Blender, not about opening a download.
- Every `archive` hit outside the installing and directory-layout pages — Alembic and USD caches, extension repositories, packed data, the manual's own contribution guides.
- `decompress` in `files/blend/open_save.txt` and `glossary/index.txt` — blend-file and video compression, not the download package.
- `installer` in `contribute/manual/getting_started/local_editing/install/windows.txt` — about installing tools to edit the manual, not about a Blender build. Not opened beyond confirming the subject from the path and the grep line.
- The whole Python API documentation for 5.0 — searched for `installer`, `extract Blender`, `unpack`, `tar`. The only `unpack` hits are the blend-file packed-data operators (`bpy.ops.file`, `bpy.types.Image`, `unpack_method_items` and the like). Nothing about build packaging. The API documents what runs inside Blender, which cannot describe what has to exist before Blender is on the machine.
- Cycles' own files for 5.0 — searched for `tar`, `unzip`, `extract`, `install`, `archive`. Every hit is an unrelated substring (`start`, `startup`, `target`, `preview_denoising_start_sample`). Cycles' files describe render settings; they say nothing about how Blender is packaged or unpacked.
- The 5.0 release notes — searched for `install`, `package`, `extract`, `zip`, `dmg`, `installer`, `glibc`, `Apple Silicon`, `Intel`, `arm64`, `minimum`. The only hits bearing on this question are the Intel Mac removal and the raised GPU/driver minimums, both about what the build runs on rather than what unpacks it.
- Pictures: I checked `getting_started/installing/` for `[image:` markers. There are none — the installing pages carry no screenshots in this version, so there was no picture to open for this question.

**Terms searched that return nothing anywhere in the 5.0 sources**, which is what makes the silences above real answers: `checksum`, `notariz`, `Gatekeeper`, `quarantine`, `code sign`, `codesign`, `Rosetta`, `redistributable`, `AppImage`, `flatpak`, `.xz`, `chmod`, `executable bit`, `disk space`, `free space`, `unattended`, `msiexec`, `.msi`. `7-zip` returns exactly one file: the Linux installing page.

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

2026-08-29 — Claude Opus 5 (Claude Code) — Builds, question 10, Blender 5.0
