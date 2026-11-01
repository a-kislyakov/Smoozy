# Builds — Open Threads questions, second pass (Blender 5.0)

**Sources consulted, and when.**
Blender's manual for 5.0 (`Blender Sources/manual_5.0`, snapshot taken 2026-07-30 — date read: 2026-07-30); Blender's Python API documentation for 5.0 (`Blender Sources/api_5.0`, snapshot 2026-07-30 — date read: 2026-07-30); Blender's 5.0 release notes (`Blender Sources/release_notes_5.0`, snapshot 2026-07-30 — date read: 2026-07-30); Cycles' own files (`Blender Sources/cycles_5.0`, snapshot 2026-07-30 — date read: 2026-07-30); and Blender's requirements page (`Blender Sources/requirements/requirements_2026-08-01.txt`, taken 2026-08-01 — date read by me: 2026-08-30). No web access was used.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

One disclosure, so the line above is exact rather than merely true. Inside `Blender Sources/` there is a file called `About this folder.md`, which is a note written by this project rather than by Blender. I read it, because it sits inside the one folder I was told I may read, and I used it only to learn what each subfolder holds and that `requirements/` is not version-specific. It contains a link to this project's research method document; I did not follow that link, and I opened nothing outside `Blender Sources/` and the question sheet.

Only question 10 was assigned. It is the only question answered below.

---

## 10. What does a computer need to have on it before a published Blender build of each kind can be unpacked?

### The question was ambiguous, and I say so before answering

"Each kind" can mean two things, and the sheet's own note under question 10 does not settle which.

- **Reading A (the one I answer as the main answer):** each kind of *package* Blender publishes for a computer — the Windows installer file, the Windows zip, the macOS disk image, the Linux archive, and the store-delivered forms (Microsoft Store, Snap, Steam).
- **Reading B:** each kind of *release channel* — stable release, long-term support, daily build, source.

I take Reading A as the one meant, because "unpacked" describes an act performed on a package, and because question 1 already asks what the channels are and how each is named. I answer Reading B as well at the end of this answer, briefly, because the sources happen to settle it in one sentence.

I also separate two things the sources treat differently, and that it would be easy to run together:

- what must be present to **open the package** (the act the question asks about);
- what must be present for the unpacked Blender to **run** (system requirements).

The sources say a great deal about the second and very little about the first. I do not carry the second across to the first, because nothing in the sources says that unpacking is blocked by them.

---

### A. Linux archive

**LABEL: documented.**

The computer needs an extraction tool that handles TAR. The manual says outright that one common alternative does not work.

> "Extracting Blender's archive using 7-zip is not supported. TAR must be used instead. For more details, see issue #104070."

That sits under a heading "Known Limitations" → "Archive Extraction" at the very end of the Linux installation page.

Beyond the extraction tool, the manual asks only for a place to put the result:

> "Download the Linux version for your architecture and decompress the file to the desired location (e.g. ~/software or /usr/local)."

No administrator rights, no package manager, no runtime library, and no other prerequisite is stated for this step.

- Source: Blender manual (5.0)
- Address: https://docs.blender.org/manual/en/5.0/getting_started/installing/linux.html
- Date read: 2026-07-30

**Not covered.** The manual does not say *which* TAR-capable tool, nor what compression the archive uses, nor whether other tools besides 7-zip also fail. It does not say what goes wrong when 7-zip is used — only that it is not supported. It does not say whether a TAR tool is present by default on any given distribution.

---

### B. Linux, package manager and Snap

**LABEL: documented** for Snap, whose precondition the manual states in words; **inferred** for the distribution package, whose precondition it does not. *(Citation check, 2026-08-30 — Claude Opus 5: label line adjusted to match the ruling below.)*

These are not unpacked by the person at all; the tooling has to be there first.

For Snap, the requirement is stated in so many words:

> "Assuming snap is already installed, Blender can be installed through snap with:"
>
> `snap install blender --classic`

For a distribution package the manual states no prerequisite at all. That the distribution's own repository and package manager must be there is **inferred** from the quoted line, not stated by it: *(Citation check, 2026-08-30 — Claude Opus 5: changed — this was labelled documented; the quote says some distributions may have a package, not that a package manager must be present.)*

> "Some Linux distributions may have a specific package for Blender in their repositories."

- Source: Blender manual (5.0)
- Address: https://docs.blender.org/manual/en/5.0/getting_started/installing/linux.html
- Date read: 2026-07-30

**Not covered.** The manual does not say how to install snap itself, nor which distributions carry a Blender package, nor whether these routes need administrator rights (it does not say either way).

---

### C. Windows zip

**LABEL: documented.**

What is needed is somewhere to extract to, and the ability to extract — and specifically *not* administrator rights.

> "When choosing the zip-file, you have to manually extract Blender to the desired folder, where you can double-click the executable to run Blender."

> "No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."

- Source: Blender manual (5.0)
- Address: https://docs.blender.org/manual/en/5.0/getting_started/installing/windows.html
- Date read: 2026-07-30

**Not covered.** The manual names no extraction tool for Windows at all — it neither requires one nor says the operating system's own is enough. It says nothing about long paths, antivirus interference at extraction time, or file-in-use conflicts. The Linux "TAR must be used" note is on the Linux page and is not repeated for Windows; I do not read it as applying to the Windows zip.

---

### D. Windows installer file

**LABEL: documented.**

The stated prerequisite is administrator rights.

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

- Source: Blender manual (5.0)
- Address: https://docs.blender.org/manual/en/5.0/getting_started/installing/windows.html
- Date read: 2026-07-30

**Not covered.** The manual does not say what the installer file's format is, whether any installer framework must already be present, whether it can run without a person present, or what happens when it is run without administrator rights.

---

### E. Windows, Microsoft Store

**LABEL: inferred.**

The manual states no prerequisite. That the Microsoft Store itself must be present is a step the reader supplies — the quote says Blender can be installed from the Store, not that the Store has to be there first. There is no unpacking step for the person. *(Citation check, 2026-08-30 — Claude Opus 5: changed — label changed from documented to inferred.)*

> "Blender can be installed from the Microsoft Store by searching for Blender in the Microsoft Store and installing it."

- Source: Blender manual (5.0)
- Address: https://docs.blender.org/manual/en/5.0/getting_started/installing/windows.html
- Date read: 2026-07-30

**Not covered.** Nothing is said about a Microsoft account, Store availability by region or Windows edition, or rights required.

---

### F. macOS disk image

**LABEL: documented.**

The manual describes the whole of it as two gestures and states no prerequisite at all. That the ability to mount a disk image is therefore a precondition is **inferred**, not documented — an instruction to mount one is not a statement that the machine must be able to. *(Citation check, 2026-08-30 — Claude Opus 5: changed — this read "What is needed is the ability to mount a disk image" under a documented label; relabelled inferred.)*

> "Blender for macOS is distributed as disk images (dmg-files). To mount the disk image, double-click on the dmg-file. Then drag Blender.app into the Applications folder."

One further thing may be needed before Blender will *open*, and the manual is explicit that it depends on the machine's settings:

> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."

Note the boundary: that sentence is about opening Blender, not about mounting the image or copying the app.

- Source: Blender manual (5.0)
- Address: https://docs.blender.org/manual/en/5.0/getting_started/installing/macos.html
- Date read: 2026-07-30

**Not covered.** The manual does not say whether a disk image can be mounted without a person double-clicking it, nor what the approval prompt asks for, nor whether it can be satisfied in advance, nor whether it appears at all when Blender is started from a terminal.

---

### G. Steam (all three operating systems)

**LABEL: documented** for the Steam client; the account half is not stated as a precondition — see below. *(Citation check, 2026-08-30 — Claude Opus 5: label line adjusted to match the ruling below.)*

The Steam client must be downloaded and installed first. An account is used at the next step, and the quoted words expressly allow creating one there and then, so they do not say an account must already exist. *(Citation check, 2026-08-30 — Claude Opus 5: changed — this read "Three things must be present first: the Steam client, an account, and a login", which the quote does not carry.)*

> "Download and install the Steam client for your operating system."

> "Once it is installed, open the client and login to your Steam account, or create an account if you don't already have one."

- Source: Blender manual (5.0)
- Address: https://docs.blender.org/manual/en/5.0/getting_started/installing/steam.html
- Date read: 2026-07-30

**Not covered.** Nothing about whether Steam can be driven without a person, or what rights installing the client needs.

---

### H. What the sources say holds across the unpackable kinds

**LABEL: documented.**

The manual states one general property of a downloaded Blender: there is nothing else to put on the computer, because the package carries what it needs.

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

And on the bundled interpreter, which is the one dependency people most often expect to have to supply:

> "Blender bundles a complete Python installation and does not interact with the system Python by default."

- Source: Blender manual (5.0)
- Address: https://docs.blender.org/manual/en/5.0/advanced/deploying_blender.html
- Date read: 2026-07-30

**Not covered.** "Self contained" is said of the extracted installation. The page does not enumerate what, if anything, the extraction step itself depends on, and it does not name any system library that must already be present.

---

### I. The architecture of the package must match the computer

**LABEL: documented.** This is stated as a condition on *which package to download*, not as a condition on unpacking. I report it because it appears inside the same installation instructions, and I mark the distinction rather than blur it.

Windows:

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture."

macOS, on the same page pattern:

> "Blender supports both Intel and Apple Silicon architectures on macOS. Make sure to download a variant that is compatible with your CPU's architecture."

There is a conflict inside the 5.0 sources here that I report without resolving. The manual page above speaks of Intel and Apple Silicon variants. The 5.0 release notes list, under Compatibility:

> "[Support for Intel Macs has been removed]"

And Blender's requirements page states, in a note:

> "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later. Read more."

- Sources and addresses: Blender manual (5.0), https://docs.blender.org/manual/en/5.0/getting_started/installing/windows.html and https://docs.blender.org/manual/en/5.0/getting_started/installing/macos.html, date read 2026-07-30; Blender 5.0 release notes, https://developer.blender.org/docs/release_notes/5.0/, line 34 of the version's own page, date read 2026-07-30; Blender's requirements page, file `Blender Sources/requirements/requirements_2026-08-01.txt`, in the "macOS" section immediately after the minimum/recommended table, taken 2026-08-01, read by me 2026-08-30.

**Not covered.** I cannot tell from these sources whether the macOS manual page is simply out of date for 5.0 or whether some 5.0 macOS package still carries Intel support. The release note and the requirements note both point at pages I was not given and did not visit. Nothing here says a mismatched package fails to *unpack* — only that it is the wrong one to download.

---

### J. System requirements — and why I do not present them as unpacking requirements

**LABEL: documented, for what they are; and "not in the sources" for the thing the question asks.**

Blender's requirements page gives minimums per operating system. For macOS it names 5.0 explicitly (quoted in section I above). For Windows and Linux the tables carry no version at all, and the folder's own note says the page "describes whichever release was current on that date" and "can only speak for a version it names in so many words" — so I quote them but do not attach them to 5.0.

Windows table, first row: `OS  Windows 8.1 (64-bit)  Windows 11`
Linux table, first row: `OS  Distribution with glibc 2.28 or newer (64-bit)`

The manual points at these requirements from the installation pages:

> "Blender has a set of minimum and recommended requirements; so make sure these are met before trying to install Blender."

That last sentence is the closest anything in the sources comes to naming a precondition for installing. It is a pointer, not a list, and it does not distinguish unpacking from running.

**What is not in the sources:** no source I found states that the operating-system version, the glibc version, the amount of memory, or the graphics card is required in order to *unpack* a package. Those are stated as requirements for Blender itself. Treating them as unpacking requirements would be a step I would have to supply, and I do not take it.

- Sources and addresses: Blender manual (5.0), https://docs.blender.org/manual/en/5.0/getting_started/installing/index.html, section "System Requirements", date read 2026-07-30; Blender's requirements page, file `Blender Sources/requirements/requirements_2026-08-01.txt`, in the "Windows" and "Linux" tables, taken 2026-08-01, read by me 2026-08-30.

**Not covered.** Whether the Windows and Linux tables as quoted describe 5.0 or a later release current on 2026-08-01. The page gives no version address, so I cannot pin them.

---

### K. Reading B — does it differ by release channel?

**LABEL: documented.**

No. The manual says so directly, for the two channels it names:

> "The procedure for installing a binary, either the latest stable release or a daily build, is the same. Follow the steps for your platform listed below."

The same page lists the channels as Stable Release, Long-term Support, Daily Builds and Build from Source. Of these, the fourth is the exception: the manual says nothing about what a computer needs in order to build from source, beyond noting that

> "While normal users are not expected to compile Blender, it does have advantages:"

For the source kind, therefore: **not in the sources.** No compiler, toolchain, library or disk requirement for building Blender appears anywhere in the manual, API documentation, release notes or Cycles files I searched.

- Source: Blender manual (5.0)
- Address: https://docs.blender.org/manual/en/5.0/getting_started/installing/index.html
- Date read: 2026-07-30

**Not covered.** The quoted sentence names stable and daily; it does not mention LTS by name, though LTS is listed as a package on the same page. It says nothing about release candidates or alpha/beta packages differing in packaging, only in stability. And it says nothing about the source kind's prerequisites at all.

---

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

**Partly documented, partly not in the sources.**

What the sources give:

- The Linux route is described in terms that do not need a window at all: "decompress the file to the desired location", and the Snap route is a single command, `snap install blender --classic`. Both are quoted above from the Linux installation page.
- The manual's own production-deployment page says deployment can be automated: "deployment can be automated using standard file management tools" (Deploying Blender in Production, quoted in section H).

What the sources do not give:

- Nothing says whether the macOS disk image can be mounted other than by double-clicking it. The manual describes only the gesture: "To mount the disk image, double-click on the dmg-file."
- Nothing says whether the Windows installer file can be run without a person present, or whether the macOS first-open approval prompt can be answered or avoided when Blender is started from outside its own window.
- Nothing says whether the Microsoft Store or Steam routes can be driven without a person.

So: the Linux archive and Snap routes are documented in a form that does not need a person clicking. For the macOS disk image, the Windows installer, the Microsoft Store and Steam, the sources do not say.

**Brought up to date by hand tests, 2026-08-30 — label: observed.** This line said the sources give no way to unpack without a person, and for two of the three operating systems that is now answered by running it. **On the Creator's Mac** the whole macOS route was driven from a terminal with no window and nobody clicking: `hdiutil attach`, `cp -R` the application out, `hdiutil detach`, then the copied Blender printed its version — for all four versions, with no prompt and no elevation. A disk image marked by hand as downloaded from the internet went the same way. **On a rented Linux machine** the archive was fetched, verified against its published checksum, opened with `tar`, and started, all from a terminal on a machine with no display server at all — and an ordinary user account did it too, so no administrator account is involved. What is still not settled from outside is the Windows half: whether the Windows installer file can be run with nobody present. Those tests are written out ready to run in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*.

### Operating system

This answer is per operating system by its nature, and the differences are the substance of it:

- **Linux** is the only one with a stated tool requirement: TAR, and explicitly not 7-zip.
- **Windows** has two published unpackable kinds that differ in rights: the installer requires administrator rights, the zip explicitly does not.
- **macOS** has one kind, the disk image, mounted by double-clicking and copied into Applications; the extra thing that may be needed is the machine's own security approval, and only at first open.
- Steam is documented for all three; Snap and distribution packages are Linux only; the Microsoft Store is Windows only.

Nothing in the sources suggests a further split within an operating system (for example, between Linux distributions) beyond what is quoted above.

**Brought up to date by hand tests, 2026-08-30 — label: observed.** Two of the three operating systems are now answered by running, and the third is written out ready to run.

**Linux, on a rented Linux machine in Romania** described in the observed section above. `tar` **on its own is not enough**: a machine with GNU tar 1.34 and no `xz` refused all four archives with `tar (child): xz: Cannot exec: No such file or directory` and extracted nothing. Installing one package, `xz-utils`, made all four open with no message. The archive is a tar inside xz compression. The unpacked program is marked runnable straight out of the archive, needs no administrator account, and **will not start** on that machine until nine further library packages are installed — the first thing it asks for is `libX11.so.6`, and `ldd` prints ten lines naming nine distinct libraries. *(Citation check, 2026-08-30 — Claude Opus 5: changed from “ten missing libraries” to match the listing.)* 7-zip reports success twice and produces a Blender that cannot run, in all four versions. `snap install blender --classic` on a machine with no snap is `command not found`. Ubuntu 22.04's own package is Blender 3.0.1.

**macOS, on the Creator's Mac** described in the observed section above. The disk image mounts from a terminal with no prompt and no elevation, the application copies out, and it runs — for all four versions. The application is signed and notarised and `spctl` accepts it. A quarantine mark stops neither the mount nor the copy nor a terminal run. A package for the wrong processor mounts and copies out just the same.

**Windows is still untested, and it is not untestable.** This project has a route to a Windows computer — a rental described in the equipment section of `Preparation/Blender Research/Method.md` — and the Windows entries are written out in full, ready to run, in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*. **Any answer taken there must name the Windows edition it was taken on**: every Windows machine this project has read turned out to be a Server edition whatever its image was called, and a finding that says only "Windows" is not an answer. What is already settled about Windows without such a machine, from the download server alone, is the published file kinds: `.msi`, `.msix` and `.zip`, for each of an x64 and an arm64 processor.

---

## Record of where I looked

This is part of the answer, not a footnote to it.

**Files opened in full and read.**

- `manual_5.0/getting_started/installing/index.txt`
- `manual_5.0/getting_started/installing/windows.txt`
- `manual_5.0/getting_started/installing/macos.txt`
- `manual_5.0/getting_started/installing/linux.txt`
- `manual_5.0/getting_started/installing/linux_windowing_environment.txt`
- `manual_5.0/getting_started/installing/steam.txt`
- `manual_5.0/advanced/deploying_blender.txt`
- `manual_5.0/advanced/blender_directory_layout.txt`
- `manual_5.0/getting_started/configuration/hardware.txt` (first 60 lines; peripherals only)
- `manual_5.0/troubleshooting/startup.txt` (first 40 lines)
- `manual_5.0/advanced/command_line/launch/linux.txt`, `.../macos.txt`, `.../windows.txt`
- `release_notes_5.0/index.md` in full; `release_notes_5.0/bugfixes.md` around the "Platforms & Builds" section; `release_notes_5.0/user_interface.md` around its "Platforms" section
- `requirements/requirements_2026-08-01.txt` in full
- `Blender Sources/About this folder.md` (the folder's own note — see the disclosure at the top)

**Terms searched, across `manual_5.0`, `api_5.0` and `release_notes_5.0` together unless stated.**

Unpacking and packaging words: `7-zip`, `7zip`, `tar`, `unpack`, `unzip`, `untar`, `unarchive`, `decompress`, `extract`, `archive`, `compressed`, `gzip`, `bz2`, `xz`, `.zip`, `zip-file`, `dmg`, `disk image`, `msi`, `Installer File`, `silent`, `Winzip`, `File Explorer`, `double-click`, `mount`, `hdiutil`.

Prerequisite words: `prerequisit`, `dependenc`, `redistribut`, `Visual C`, `glibc`, `libdecor`, `self contained`, `self-contained`, `administrator`, `admin rights`, `elevat`, `sudo`, `permission`, `root`.

macOS gatekeeping words: `Gatekeeper`, `quarantine`, `notariz`, `codesign`.

Store and channel words: `Snap`, `Microsoft Store`, `Downloading Blender`, `download`, `install`, `package`, `platform`, `portable`, `requirement`, `arm64`, `Apple Silicon`, `Ventura`, `Windows 8`.

**What came back empty, and what that rules out.**

- `Gatekeeper`, `quarantine`, `notariz`, `codesign`, `hdiutil`, `untar`, `unarchive`, `Winzip`: no hits anywhere in the 5.0 manual. So the sources describe the macOS security prompt only in the one sentence quoted, and name no tool or command for mounting an image or clearing a quarantine flag.
- `glibc`: no hits in the manual, API or release notes at all — its only appearance in anything I read is the Linux table of the requirements page, which carries no version.
- `7-zip`: exactly one hit in the whole of the 5.0 manual, the Linux installation page. It is not repeated for Windows, which is why I do not extend it there.
- `redistribut`, `Visual C`: no hits relating to a Windows runtime. No runtime library is named as a prerequisite anywhere.
- `installer`, `dmg-file`, `7-zip`, `tar.xz` in `api_5.0`: no hits. The Python API documentation says nothing about packaging or installation prerequisites.
- `install`, `extract`, `unzip` in `cycles_5.0`: no hits in Cycles' Python files. Cycles' own files carry no setting or comment bearing on this question, so there is no **observed** finding in this answer.
- Release notes: the only installation-adjacent entries are the Compatibility list on the index page (quoted above) and a "Platforms & Builds" bugfix list, which contains build-compilation and crash fixes and nothing about unpacking a published package.

**Pictures.** I checked whether the installation pages or the configuration pages carry any. The three installation pages, the installation index and the Steam page contain no `[image: …]` markers at all, so there is no screenshot of a download page or an installer to inspect. The only images near this part of the manual are on the peripherals and defaults pages, which are about monitors, input devices and the preferences window, and settle nothing here.

**Pages I ruled out and why.** The whole of `render/`, `modeling/`, `animation/`, `sculpt_paint/`, `compositing/`, `editors/`, `video_editing/`, `files/`, `physics/`, `grease_pencil/` and `movie_clip/` — searched by the terms above, matched only on unrelated uses ("extract" as a mesh operation, "archive" as Alembic, "unpack" as packed data inside a blend-file, "decompress" as video frames). `manual_5.0/contribute/` matched on `sudo` only in instructions for building the manual, which is not Blender.

**One thing the sources point at and I could not follow.** The three installation pages each open with "Check the Downloading Blender page to find the minimum requirements and the different versions that are available for Blender". No such page exists inside `manual_5.0`; the reference is to blender.org, which is outside my sources. I did not go there. Whatever that page states about what must be present before a download can be opened is therefore not in this answer.

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

2026-08-30 — Claude Opus 5 (Claude Code) — Builds, Blender 5.0
