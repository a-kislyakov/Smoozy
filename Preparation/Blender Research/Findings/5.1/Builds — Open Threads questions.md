# Builds — Open Threads questions — Blender 5.1

**Sources consulted, and when.** Blender 5.1 manual, Blender 5.1 Python API documentation, Blender 5.1 release notes, Blender's requirements page (snapshot of 2026-08-01), and Cycles' own files for 5.1 — all as supplied in `Blender Sources/`. Snapshot date for the manual, API documentation, release notes and Cycles files: 2026-07-30. Date read: 2026-07-30 (I use the snapshot date as the date read, as instructed). The requirements page snapshot is dated 2026-08-01 and is cited with that date.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

Only question 10 was assigned. Questions 1–9 are not answered here.

---

## 10. What does a computer need to have on it before a published Blender build of each kind can be unpacked?

### The question was ambiguous, and here is the reading I answered

"Each kind" can mean two things in this sheet.

- **Reading A — kind of release.** The manual's Installing page names four: Stable Release, Long-term Support, Daily Builds, Build from Source.
- **Reading B — kind of package.** The manual's per-platform pages name the actual downloadable things: a Windows zip-file, a Windows Installer File, the Microsoft Store, a macOS disk image, a Linux archive from blender.org, a Linux distribution package, Snap, and Steam.

I answered **Reading B**, because the manual itself says the release kinds do not differ in this respect:

> "The procedure for installing a binary, either the latest stable release or a daily build, is the same. Follow the steps for your platform listed below."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html — read 2026-07-30

Under Reading A the answer would therefore collapse into the same list: a stable release and a daily build of the same platform arrive as the same kind of package and need the same things present. **Label: inferred.** The step: the quoted line names the stable release and the daily build and says the procedure is the same for both; that Long-term Support behaves the same way is my step, because the quoted words do not name it. "Build from Source" is the one release kind that stands apart, and it is answered separately below.

### The answer, package by package

**Windows — zip-file.** Nothing is named as needing to be present except a place to extract to, and the manual says plainly that administrator rights are *not* among the requirements.

> "When choosing the zip-file, you have to manually extract Blender to the desired folder, where you can double-click the executable to run Blender."
> "No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html — read 2026-07-30

The manual calls the result "the unzipped folder":

> "Windows: Next to the Blender executable, in the unzipped folder"
> — Manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — read 2026-07-30

The sources never name a tool that must be present to do the unzipping on Windows, and never say whether Windows' own built-in extraction is enough. **Label: documented** for "no administrator rights needed"; **not in the sources** for what performs the extraction.

**Windows — Windows Installer File.** Administrator rights must be present.

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html — read 2026-07-30

**Label: documented.**

**Windows — Microsoft Store.** The Microsoft Store must be on the machine and usable.

> "Blender can be installed from the Microsoft Store by searching for Blender in the Microsoft Store and installing it."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/windows.html — read 2026-07-30

**Label: inferred.** The step I took: the manual describes the act of searching within the Microsoft Store and installing from it; it never states as a requirement that the Store be present. That the Store must be there to be searched is my step, not the source's words. The sources say nothing about a Microsoft account being needed.

**macOS — disk image (dmg).** Nothing is named as needing to be present beyond whatever mounts a disk image when it is double-clicked, and the manual does not name that.

> "Blender for macOS is distributed as disk images (dmg-files). To mount the disk image, double-click on the dmg-file. Then drag Blender.app into the Applications folder."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/macos.html — read 2026-07-30

The same page mentions an approval step, but it is about *opening Blender*, which is after unpacking, not before it:

> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."
> — same page, same date

**Label: documented** for the mount-and-drag step; **not in the sources** for any tool, permission, or security setting that must be in place *before* the dmg can be mounted. The manual gives no command-line way to mount a dmg, and the words "hdiutil", "Gatekeeper", "quarantine" and "notarize" do not appear anywhere in the 5.1 manual, the 5.1 API documentation, the 5.1 release notes or the requirements page.

**Linux — archive from blender.org.** This is the one package where the sources name a required tool outright, and name one that will not do.

> "Download the Linux version for your architecture and decompress the file to the desired location (e.g. ~/software or /usr/local)."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html — read 2026-07-30

and, on the same page, under "Known Limitations ‣ Archive Extraction":

> "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead. For more details, see issue #104070."
> — same page, same date

**Label: documented.** TAR must be present. 7-zip is explicitly ruled out. The archive's actual format is never named in the sources (I searched for "xz", "gzip", "zstd" and "bzip" across the whole 5.1 manual and API documentation and found no packaging use of any of them), so what TAR must in turn be able to decompress is not stated.

**Linux — distribution package.** The distribution's own package manager must be present, and the distribution must actually carry Blender.

> "Some Linux distributions may have a specific package for Blender in their repositories."
> "Installing Blender via the distribution's native mechanisms ensures consistency with other packages on the system and may provide other features (given by the package manager), such as listing of packages, update notifications and automatic menu configuration."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html — read 2026-07-30

**Label: inferred.** The step: the manual describes installing "via the distribution's native mechanisms" and repeatedly calls that thing "the package manager"; that the package manager must therefore be present is my step. **Documented**, separately from that step: the manual says outright that the package may not be there at all, and warns what it may lack:

> "Be aware, though, that the package may be outdated compared to the latest official release, or not include some features of Blender. For example, some distributions do not build Blender with Cycles GPU rendering support, for licensing or other reasons."
> — same page, same date

**Linux — Snap.** Snap must already be installed. The manual says so in so many words.

> "Assuming snap is already installed, Blender can be installed through snap with:"
> "snap install blender --classic"
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/linux.html — read 2026-07-30

**Label: documented.** Nothing is said about needing administrator rights for the snap command, and the `--classic` flag is given without explanation.

**Steam (Linux, macOS and Windows).** The Steam client must be installed, and a Steam account is needed — though the quoted words say one can be created at that point, so they do not say it must already be there.

> "Download and install the Steam client for your operating system."
> "Once it is installed, open the client and login to your Steam account, or create an account if you don't already have one."
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/steam.html — read 2026-07-30

**Label: documented.**

**Build from Source.** The manual names this as one of the four release kinds, but gives no list of what must be on the computer first.

> "Blender's source code is available for free to either reference or to build and use. While normal users are not expected to compile Blender, it does have advantages:"
> — Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/index.html — read 2026-07-30

**Label: not in the sources.** The 5.1 manual's Installing page lists the advantages of building from source and stops; it points at no build-requirements page that is inside the supplied sources. The 5.1 release notes mention one change that bears on building — "Blender switched from C++17 to C++20." (release notes, https://developer.blender.org/docs/release_notes/5.1/core/, read 2026-07-30) — but that is a statement about the source, not a list of what a computer needs.

**The `bpy` Python module.** This is not on Blender's download page, and the way it is fetched is PIP.

> "Blender as a Python Module isn't provided on Blender's official download page."
> "A pre-compiled bpy module is available via PIP."
> — Python API documentation, https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html — read 2026-07-30

**Label: documented** for the two quoted facts — that it is not on the official download page, and that a pre-compiled module is available via PIP. **Label: inferred** for the requirement itself: that PIP must therefore be present on the computer is my step, not the source's words, the same size of step as the Microsoft Store one above.

I include this because the sheet says "each kind" and this is a published, fetchable build of Blender; the source is explicit that it is not one of the official downloads. The 5.1 corrective-releases notes list two fixes touching it:

> "Python bpy package is missing Python dependencies. (blender/blender#157121)"
> "Wrong max `numpy` version for `bpy` python package release. (blender/blender#155983)"
> — Release notes, https://developer.blender.org/docs/release_notes/5.1/corrective_releases/ — read 2026-07-30

**Label: documented** for the two fix titles as quoted. That they show the module is published once per release is **inferred** — the words name a release of the package without saying one accompanies every Blender release.

### What the sources say about the destination

For every package that is extracted rather than installed, the manual places no requirement on where it goes:

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."
> — Manual, https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30

**Label: documented.** No particular directory, and no minimum amount of space, is named. I searched the 5.1 manual, API documentation, release notes and the requirements page for "disk space", "free space" and "storage space" and found nothing about the size a Blender download needs.

### What is not in the sources at all

Every one of these was searched for across the whole 5.1 manual, the whole 5.1 API documentation, the whole 5.1 release notes, and the requirements page, and returned nothing bearing on unpacking:

- Any checksum, signature or verification step required before unpacking. Searched: "checksum" (no hits), "sha256" (one hit, about an extensions repository URL, unrelated), "signature" (hits only about node sockets), "verify" (hits only about drivers, colour spaces and command-line arguments), "GPG" (no hits), "PGP" (no hits).
- Any Windows unzipping tool, or whether Windows' built-in extraction suffices. Searched: "unzip", "zip-file", "WinRAR", "7-zip", "7zip", "unarchiv", "File Explorer". The only 7-zip hit anywhere is the Linux archive-extraction limitation quoted above.
- Any runtime library or redistributable that must be present before unpacking. Searched: "redistributable", "Visual C", "runtime library", "dependency"/"dependencies" (hits only about the dependency graph and about extension wheels), "glibc" (hits only on the requirements page, and about running rather than unpacking).
- Any macOS security or permission state that gates mounting a dmg. Searched: "hdiutil", "gatekeep", "quarantine", "notariz", "mount" — the only relevant "mount" hit is the macOS install line quoted above.
- Any elevation requirement beyond the Windows installer. Searched: "administrator" (the Windows installer line, plus unrelated hits in preferences and contribute pages), "sudo", "root privile", "elevated", "UAC".
- Anything about antivirus interfering with unpacking. The one antivirus mention in the 5.1 manual is about Blender failing to *start*: "Make sure any antivirus software is not preventing Blender from starting." — Manual, https://docs.blender.org/manual/en/5.1/troubleshooting/startup.html — read 2026-07-30.

**Cycles' own files bear on this question not at all.** I searched all of `Blender Sources/cycles_5.1` for "install", "extract", "unpack" and "archive". The hits are variable and comment text inside the render kernel and the bundled Apache 2.0 licence file, none of it about packaging or installing Blender. **Label: not in the sources**, and the sentence above is the record of where I looked: the whole of that folder, those four terms, and what the hits turned out to be.

**No picture in the manual settles any of this.** I checked for images on every page that bears on the question: the Installing index, the Linux, macOS, Windows and Steam install pages, and the directory-layout page carry no `[image: …]` marker at all. The deploying-Blender page (https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30) carries exactly one, `advanced_deploying-blender_system-extensions.png`. The text around it introduces it as the "System repository", so on those words it is about bundling extensions rather than about unpacking a build. I did not open the picture, so that reading rests on the surrounding text and not on what the picture shows. I took nothing from it.

### Not covered

- **The archive formats are never named.** The manual says "decompress the file" for Linux and "zip-file" for Windows, and never states what the Linux archive actually is. So "TAR must be used" is a documented instruction whose object is undocumented.
- **The Windows zip has no stated extraction tool.** The Linux page rules 7-zip out for the Linux archive; nothing anywhere says whether that limitation touches the Windows zip. Do not carry it across.
- **Nothing distinguishes daily builds from releases here.** The manual states the procedure is the same, but it never separately describes what a daily build arrives as on each platform. If daily builds are packaged differently from releases on any platform, the sources do not say so.
- **Nothing about the state of a machine mid-unpack**, or about unpacking onto a network or read-only location.
- **Running versus unpacking.** Everything on the requirements page — CPU, RAM, GPU, OS version — is about running Blender, not about unpacking it. One line there does name versions in so many words: "Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later." (requirements page snapshot, `requirements_2026-08-01.txt`, read 2026-08-01). The 5.1 manual's macOS install page instead says "Blender supports both Intel and Apple Silicon architectures on macOS. Make sure to download a variant that is compatible with your CPU's architecture." (https://docs.blender.org/manual/en/5.1/getting_started/installing/macos.html, read 2026-07-30). I report both without reconciling them. Neither speaks to whether a dmg can be *unpacked* on a given machine.

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

Unpacking happens before Blender runs, so nothing here depends on how Blender is later driven. **Label: inferred** — the step is that a build has to be unpacked before it can be started, which no quoted line here states. Beyond that, the manual states outright that Blender's downloads are meant to be deployable without a person:

> "Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."
> — Manual, https://docs.blender.org/manual/en/5.1/advanced/deploying_blender.html — read 2026-07-30

**Label: documented** that Blender downloads can be extracted anywhere and that deployment can be automated with ordinary file tools — those are the manual's own words, and they speak of "Blender downloads" in general. **Label: inferred** that this covers the Windows zip and the Linux archive in particular: the quote names neither, and matching them to it is my step from the extract-and-decompress wording on their own pages. Two of the ten things listed above are given as commands already, which need no window of Blender's: `snap install blender --classic` on Linux, and PIP for the `bpy` module.

Where it differs, and this matters: four kinds are described only as things a person does by hand, and the sources give no non-interactive equivalent. The macOS dmg is described only as "double-click on the dmg-file" and then "drag Blender.app into the Applications folder". The Microsoft Store is described only as "searching for Blender in the Microsoft Store". Steam is described only as pressing "the green Install button" after logging in (Manual, https://docs.blender.org/manual/en/5.1/getting_started/installing/steam.html — read 2026-07-30). The Windows Installer File is described only as being run, with administrator rights. **Label: not in the sources** — the manual nowhere says whether any of these four can be driven without a person clicking.

**Brought up to date by hand tests, 2026-08-30 — label: observed.** This line said the sources give no way to unpack without a person, and for two of the three operating systems that is now answered by running it. **On the Creator's Mac** the whole macOS route was driven from a terminal with no window and nobody clicking: `hdiutil attach`, `cp -R` the application out, `hdiutil detach`, then the copied Blender printed its version — for all four versions, with no prompt and no elevation. A disk image marked by hand as downloaded from the internet went the same way. **On a rented Linux machine** the archive was fetched, verified against its published checksum, opened with `tar`, and started, all from a terminal on a machine with no display server at all — and an ordinary user account did it too, so no administrator account is involved. What is still not settled from outside is the Windows half: whether the Windows installer file can be run with nobody present. Those tests are written out ready to run in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*.

### Operating system

This answer does not hold across operating systems; it is different on each, by design, and the manual has a separate page per platform. In summary: the TAR requirement is stated on the Linux page alone; the administrator-rights requirement is stated on the Windows page alone and only for the installer; the mount-a-disk-image step is on the macOS page alone. Snap is Linux only. The Microsoft Store is Windows only. Steam is the only kind the manual describes as covering all three: "Blender can be downloaded and updated using the Steam client by following the steps described below on Linux, macOS, or Windows." (https://docs.blender.org/manual/en/5.1/getting_started/installing/steam.html, read 2026-07-30). **Label: documented.**

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

2026-08-29 — Claude Opus 5 (Claude Code) — Builds, Blender 5.1
