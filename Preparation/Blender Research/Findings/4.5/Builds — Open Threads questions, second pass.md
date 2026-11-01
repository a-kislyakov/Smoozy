# Builds — Open Threads questions, second pass (Blender 4.5)

**Sources consulted, and when.** Blender's 4.5 manual (`Blender Sources/manual_4.5`, including its `_images` folder), Blender's 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender's 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`). All four were taken on 2026-07-30, and that is the date read I give throughout. I also opened two other files that sit inside `Blender Sources/`: `About this folder.md` (the note explaining what the folder holds) and `requirements/requirements_2026-08-01.txt` (Blender's requirements page, taken 2026-08-01). The requirements page was not on the list of places my answer may come from, so I have kept it out of the answer proper and reported it separately at the end, clearly marked.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One thing I should say plainly, so nobody has to wonder. To confirm the folder I was told to save into exists, I listed that folder's file names. I saw the names of the files in it. I opened none of them, including the earlier findings document I was told not to read.

This pass answers **question 10 only**.

---

## 10. What does a computer need to have on it before a published Blender build of each kind can be unpacked?

### The question had two readings, and I say which one I answered

"Each kind" can mean two different things, and the sheet's own note for question 10 does not settle which.

- **Reading A (the one I answered):** the kinds of *package* Blender publishes — a Windows zip, a Windows installer, a macOS disk image, a Linux archive, and the store-delivered ones. This is the reading that makes "unpacked" mean anything, because unpacking is an act performed on a package.
- **Reading B:** the kinds named on the manual's own Installing page — Stable Release, Long-term Support, Daily Builds, Build from Source. Under this reading the answer is short and I give it too, because the manual answers it in one sentence.

I answer A in full and B in one line. If B was meant, the one line is the whole answer.

**Reading B in one line, documented.** The manual says the steps do not differ by stability tier:

> "The procedure for installing a binary, either the latest stable release or a daily build, is the same. Follow the steps for your platform listed below."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/index.html — read 2026-07-30.

So under Reading B, nothing a computer needs varies between a stable release, an LTS release and a daily build. What varies is the platform. That is Reading A, below.

---

### LABEL: mixed — see each row

The overall answer is partly **documented** and, for the largest part of what a machine actually needs, **not in the sources**. I have split it so the two are never confused.

---

### What the sources do say a computer needs

**Linux — a `tar` program, and specifically not 7-zip.** This is the only place in all of the 4.5 documentation where the sources name a tool that must be present to unpack a Blender build.

> "Known Limitations
>
> Archive Extraction
>
> Extracting Blender's archive using 7-zip is not supported. TAR must be used instead. For more details, see issue #104070."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html — read 2026-07-30.
LABEL: **documented.**

The same page states the act itself, without naming a tool:

> "Download the Linux version for your architecture and decompress the file to the desired location (e.g. ~/software or /usr/local)."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html — read 2026-07-30.
LABEL: **documented.**

Note what is *not* here: the manual never names the Linux archive's file extension anywhere in 4.5. I searched for `tar.xz`, `.xz`, `gzip` and `bzip` across the whole 4.5 manual; the only hit was an unrelated sentence about blend-file compression in `files/blend/open_save.txt`. So "TAR must be used" is documented, but which compression sits under the tar is not.

**Windows, installer — administrator rights.**

> "The Windows installer will let you choose an installation folder, and will create an entry in the start menu as well as associate blend-files with Blender. It requires administrator rights."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html — read 2026-07-30.
LABEL: **documented.**

**Windows, zip — no administrator rights, and the extracting is the user's own job.**

> "When choosing the zip-file, you have to manually extract Blender to the desired folder, where you can double-click the executable to run Blender.
>
> No start menu item will be created and no blend-file association will be registered, but there is also no need for administrator rights."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html — read 2026-07-30.
LABEL: **documented.**

No tool is named for the Windows zip. There is no counterpart to the Linux "7-zip is not supported" warning; nothing in the 4.5 manual says which unzipping tool must or must not be used on Windows.

**Windows — the two package forms, and a matching CPU architecture.**

> "Download the zip-file or Windows Installer File."

> "Blender supports both x64 and arm64 architectures on Windows. Make sure to download a variant that is compatible with your CPU's architecture."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html — read 2026-07-30.
LABEL: **documented.** Note the architecture sentence is about which variant to download so it will *run*; the sources do not say architecture affects whether a package can be unpacked.

**Windows installer is an MSI.** The manual calls it only "Windows Installer File". The release notes name the format:

> "Windows x64: MSI Installer fails to upgrade 4.4.0->4.4.1"

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/bugfixes/ — read 2026-07-30. (Line 276 of `bugfixes.md`, in the list of fixes.)
LABEL: **inferred** that the Windows installer is an MSI. The quoted words are a bug title naming an MSI Installer for Windows x64; getting from that to "the Windows Installer File the manual offers is an MSI" is a step the reader supplies. What the machine needs in order to run an MSI is **not in the sources**. *(Citation check, 2026-08-30 — Claude Opus 5: changed — label changed from documented to inferred, and the step written out.)*

**macOS — a disk image, mounted by opening it.**

> "Blender for macOS is distributed as disk images (dmg-files). To mount the disk image, double-click on the dmg-file. Then drag Blender.app into the Applications folder."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/macos.html — read 2026-07-30.
LABEL: **documented.** No tool, permission or precondition is named for mounting. The only precondition the page mentions comes *after* unpacking, at first launch:

> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."

Source: as above — read 2026-07-30. This is about opening, not unpacking, and I do not stretch it further.

Also on that page:

> "Blender supports both Intel and Apple Silicon architectures on macOS. Make sure to download a variant that is compatible with your CPU's architecture."

Source: as above — read 2026-07-30. Same caveat as Windows: about running, not unpacking.

**Linux via Snap — snap must already be there.**

> "Snap is a universal package manager designed to work across a range of distributions. Assuming snap is already installed, Blender can be installed through snap with:
>
> snap install blender --classic"

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/linux.html — read 2026-07-30.
LABEL: **documented.**

**Steam — the Steam client, and an account used at the next step.** *(Citation check, 2026-08-30 — Claude Opus 5: changed — the heading read "the Steam client and a Steam account"; the quoted words expressly allow creating an account there and then, so they do not say one must already exist.)*

> "Download and install the Steam client for your operating system.
>
> Once it is installed, open the client and login to your Steam account, or create an account if you don't already have one."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/steam.html — read 2026-07-30.
LABEL: **documented** for the Steam client; the account half is not stated as a precondition, since the quoted words offer creating one there and then. *(Citation check, 2026-08-30 — Claude Opus 5: label line adjusted to match the ruling above.)*

**Microsoft Store — nothing stated.** The manual's whole treatment is:

> "Blender can be installed from the Microsoft Store by searching for Blender in the Microsoft Store and installing it."

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/windows.html — read 2026-07-30. No prerequisite is named.

**Linux distribution packages — no prerequisite stated at all.** The manual says some distributions carry a Blender package and describes the trade-offs, but names no prerequisite for unpacking one. That the distribution's own package manager must be present is **inferred**, not stated: no quoted line says it. Source: same Linux page. *(Citation check, 2026-08-30 — Claude Opus 5: changed — the heading read "nothing stated beyond having the distribution's own package manager", which reads a requirement into a page that states none.)*

**The general statement about extracting, for automated setups.**

> "Blender downloads can be extracted to any directory on the system, as a self contained installation. Multiple Blender versions can co-exist on the same system, and deployment can be automated using standard file management tools."

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read 2026-07-30.
LABEL: **documented.** Note "standard file management tools" is as specific as it gets; no tool is named, and the Linux page's TAR requirement is the only thing that narrows it.

**Where each kind ends up, which confirms which kinds exist as separate packages.** The directory-layout page lists system directories separately for "Archive downloaded from blender.org" (Linux), "Zip file downloaded from blender.org" and "Installer downloaded from blender.org" and "Microsoft Store installation" (Windows), and `./Blender.app/Contents/Resources/4.5/` (macOS).

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html — read 2026-07-30.
LABEL: **documented**, as a list of package kinds. It says nothing about prerequisites.

**Build from Source — no prerequisites given.** The manual describes it as a package option and then stops:

> "Blender's source code is available for free to either reference or to build and use. While normal users are not expected to compile Blender, it does have advantages:"

Source: manual, https://docs.blender.org/manual/en/4.5/getting_started/installing/index.html — read 2026-07-30. Nothing follows about compilers, build systems or libraries. I searched the whole 4.5 manual for `build from source`, `compile blender`/`compiling blender`, `cmake`, `visual studio` and `xcode`; outside the `contribute/` section (which is about building the *manual*, not Blender) there were no hits describing what a machine needs. LABEL for build-from-source prerequisites: **not in the sources.**

---

### What the sources do not say — and this is most of the practical answer

LABEL: **not in the sources**, for all of the following. Each was searched for, and the searches are listed under "Where I looked".

- **Windows zip:** no tool named, no statement that Windows' own built-in extraction is sufficient, and no counterpart to Linux's 7-zip warning.
- **Windows installer:** nothing about what must be present to run an MSI — no mention of `msiexec`, of an installer service, or of a silent/unattended install.
- **macOS:** nothing about mounting a disk image from a script or command line, nothing about `hdiutil`, nothing about Gatekeeper, notarization or quarantine. I searched the whole manual for `notariz`, `gatekeeper`, `quarantine`, `codesign`, `signed binar` and `digital signature`: zero hits anywhere in the 4.5 manual.
- **Verification before unpacking:** nothing about checksums or signatures for a downloaded build. I searched for `checksum`, `sha256`, `md5` and `verify the download`. The single `sha256` hit in the 4.5 manual is a URL in the extensions-repository documentation and is about add-on packages, not Blender builds.
- **Disk space:** no figure, anywhere. Searched `disk space` and `free space`: zero hits.
- **Runtime libraries or redistributables that must be installed first:** searched `redistributable`, `Visual C++`, `runtime librar`, `dependenc`, `prerequisit`, `before installing`. No hit describes anything a machine must have before a Blender build can be unpacked. (`glibc` does not appear in the 4.5 manual at all.)
- **Whether unpacking differs between a stable release, an LTS release, a daily build, an alpha, a beta or a release candidate:** the manual's one sentence covers stable versus daily and says they are the same; it says nothing specific about alpha, beta or release-candidate packages beyond describing them as marked builds.
- **The Linux archive's actual format** (which compression the tar carries), and therefore what decompressor must also be present alongside tar.
- **Microsoft Store prerequisites**, and anything about a `.msix`.
- **Anything at all in the Python API documentation or in Cycles' own files.** See below.

---

### Where I looked

This is the record the pass exists for. Everything here was searched across the entire 4.5 copy of each source, not by opening likely files.

**The 4.5 manual (`manual_4.5`, 4,954 files).**

- Whole-tree search for the archive and package words: `unzip`, `7-zip`, `7zip`, `.zip`, `zip-file`, `zip file`, `tar`, `.tar`, `xz`, `decompress`, `unpack`, `dmg`, `disk image`, `.msi`, `msix`, `installer`. The only pages that came back describing Blender's own packages were `getting_started/installing/*`, `advanced/blender_directory_layout.txt` and `advanced/deploying_blender.txt`. Everything else was Blender's *other* meaning of "unpack" — packed data inside a blend-file (`files/blend/packed_data.txt`, `editors/image/editing.txt`, `interface/window_system/topbar.txt`, `modeling/geometry_nodes/geometry/operations/bake.txt`), or extension `.zip` packages (`advanced/extensions/*`, `editors/preferences/addons.txt`), or the string "XZ" appearing as an axis pair in modelling and rigging pages. All ruled out as a different subject.
- `msi` alone across the whole manual: **zero hits.** That is how I know the manual never names the Windows installer's format, and why I went to the release notes for it.
- `tar.xz`, `.xz`, `gzip`, `bzip`: one hit, `files/blend/open_save.txt` line 208, about blend-file compression. Ruled out.
- Permission words: `administrator`, `admin right`, `elevat`, `root privile`, `sudo`. Two relevant hits, both on the Windows installing page (quoted above). The others were `editors/preferences/file_paths.txt` (a warning about running Blender itself elevated), `troubleshooting/network.txt` (system administrators and firewalls), `advanced/command_line/launch/macos.txt` (`sudo nano /etc/paths`, about adding Blender to PATH after installation), and `contribute/` pages about installing Python and git to build the manual. All ruled out as not about unpacking a build.
- Security words: `notariz`, `gatekeeper`, `quarantine`, `codesign`, `signed binar`, `digital signature`. **Zero hits.**
- Verification words: `checksum`, `sha256`, `md5`, `verify the download`. One hit, an extensions-repository URL. Ruled out.
- Space and prerequisite words: `disk space`, `free space`, `prerequisit`, `redistributable`, `Visual C++`, `dependency`/`dependencies`, `runtime librar`, `before installing`. Nothing about unpacking a build; the `dependencies` hits are all about add-on Python wheels or asset dependencies. Ruled out.
- Source-build words: `build from source`, `compile blender`, `compiling blender`, `cmake`, `visual studio`, `xcode`, `source code`. Outside `contribute/` (building the manual, not Blender), only the descriptive paragraph on the Installing page, quoted above.
- Architecture words: `arm64`, `x64`, `Apple Silicon`, `architecture`. The download-a-matching-variant sentences on the Windows and macOS installing pages; everything else is extension platform tags or GPU architecture names in the Cycles and EEVEE pages. Ruled out.
- Pages read end to end: `getting_started/installing/index.txt`, `.../linux.txt`, `.../macos.txt`, `.../windows.txt`, `.../steam.txt`, `.../linux_windowing_environment.txt`, `advanced/blender_directory_layout.txt`, `advanced/deploying_blender.txt`, `advanced/command_line/launch/windows.txt`, `.../linux.txt`, `.../macos.txt`, and the opening of `getting_started/configuration/hardware.txt`.
- Directory listing of `troubleshooting/` (`3d_view`, `crash`, `gpu`, `index`, `network`, `python`, `recover`, `report_bug`, `startup`) — none of these is about installing or unpacking, so I did not pursue them further.

**Pictures.** I checked whether any picture in the manual could settle this. I listed every image whose name contains `install`, `download` or `getting-started`. The only install-related one is `_images/about_contribute_install_windows_installer.png`. I traced which page uses it: `contribute/manual/getting_started/local_editing/install/windows.txt`, where the surrounding text is "Install Python with the installation wizard. Please make sure that you enable the 'Add Python to PATH' option". That is a picture of the *Python* installer, for people building the manual — not a Blender build. Ruled out, and I did not open it. None of the four installing pages, nor the deploying page's installing section, nor the directory-layout page carries any `[image: …]` line at all; the only picture on the deploying page is `advanced_deploying-blender_system-extensions.png`, which the text labels "System repository". So there is no screenshot of a Blender download page or a Blender installer in the 4.5 manual to look at.

**The 4.5 Python API documentation (`api_4.5`, 2,049 files).** Whole-tree search for `unzip`, `7-zip`, `.tar.`, `.dmg`, `disk image`, `.msi`, `.msix`, `Windows Installer`, `Microsoft Store`, `extract the archive`, `decompress`. Two hits, both irrelevant to unpacking: `bpy.types.PreferencesSystem.txt` line 206, "Whether this blender installation is a sandboxed Microsoft Store version" (a property that reports what an already-installed Blender is), and `bpy.path.txt` line 128, mentioning `.tar.gz` as an example of an extension starting with a dot. Neither says anything about what a machine needs before unpacking. Ruled out.

**The 4.5 release notes (`release_notes_4.5`, 19 files).** Whole-folder search for `zip`, `tar`, `dmg`, `msi`, `installer`, `extract`, `unpack`, `archive`, `download`, `glibc`, `arm64`, `x64`, `Apple Silicon`, `Intel`, `Rosetta`, `requirement`, `Microsoft Store`, `Steam`, `snap`, `package`. Two hits worth keeping: the MSI installer bugfix line quoted above, and this, from the version's own page:

> "**macOS Intel support is deprecated.** Due to the high maintenance cost of tracking down and fixing graphics related issues specific to Intel and AMD GPU Macs, it was decided that this will be the last release with an official Intel macOS build of Blender."

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/ — read 2026-07-30. This bears on which macOS build kinds exist, not on what is needed to unpack one. The remaining hits were the word "snap"/"snapping" in the animation and sequencer notes, GPU driver notes, and "start"/"starting" inside other words. Ruled out.

**Cycles' own files (`cycles_4.5`, 399 files).** Searched every Python file for `unzip`, `7-zip`, `.tar.`, `.dmg`, `.msi`, `msix`, `installer`, `extract`, `unpack`, `archive`. **Zero hits.** This is expected: Cycles' files describe a render engine's settings, not Blender's packaging. Ruled out as a source for this question.

---

### Not covered

- What must be present to unpack a Windows zip, to run a Windows MSI, or to mount a macOS disk image — on any of the three, from a script rather than by hand. The sources describe a person double-clicking.
- What the Linux archive's compression is, and therefore whether anything beyond `tar` itself is needed to decompress it.
- Whether "TAR must be used instead" means the GNU `tar` program specifically or any tar-format reader other than 7-zip. The sentence does not say, and I will not guess.
- Whether the 7-zip limitation is about a particular 7-zip version, and whether it has since changed. The manual points to an external issue number, which is not among my sources.
- Anything about disk space, checksums, signatures, or first-run security approval as a precondition of *unpacking* rather than *running*.
- Whether unpacking differs for alpha, beta or release-candidate daily builds. The manual's sameness statement names only "the latest stable release or a daily build".
- What a machine needs to build Blender from source.

---

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

Does this hold when Blender is driven from outside its own window, started and controlled by something other than a person clicking?

Mostly the question does not arise, because unpacking happens before Blender runs at all — no Blender window exists yet, so nothing here is an in-window behaviour. But that cuts both ways, and the honest answer is a warning rather than a reassurance.

Every unpacking instruction in the 4.5 sources is written for a person: "double-click on the dmg-file", "drag Blender.app into the Applications folder", "you have to manually extract Blender to the desired folder", "Blender can now be launched by double-clicking the executable". The sources do not give a scripted equivalent for any of them. The one place that speaks to automated setup at all is the deploying page — "deployment can be automated using standard file management tools" — and it names no tool and covers only the extract-an-archive case, not the macOS disk image and not the Windows installer.

Two things in the answer above do carry over unchanged, because they are facts about the package rather than about clicking: the Linux TAR requirement, and the Windows installer's need for administrator rights. Whether an MSI can be run unattended, and how, is **not in the sources**.

The Steam and Microsoft Store routes both require signing in to an account through those clients' own interfaces, which the sources describe only as a person doing it.

**Brought up to date by hand tests, 2026-08-30 — label: observed.** This line said the sources give no way to unpack without a person, and for two of the three operating systems that is now answered by running it. **On the Creator's Mac** the whole macOS route was driven from a terminal with no window and nobody clicking: `hdiutil attach`, `cp -R` the application out, `hdiutil detach`, then the copied Blender printed its version — for all four versions, with no prompt and no elevation. A disk image marked by hand as downloaded from the internet went the same way. **On a rented Linux machine** the archive was fetched, verified against its published checksum, opened with `tar`, and started, all from a terminal on a machine with no display server at all — and an ordinary user account did it too, so no administrator account is involved. What is still not settled from outside is the Windows half: whether the Windows installer file can be run with nobody present. Those tests are written out ready to run in `Audits/Blender Research/Builds/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*.

### Operating system

This answer is entirely made of per-operating-system differences, so no part of it holds everywhere. Setting them side by side:

- **Windows.** Two published packages, zip and installer. Installer needs administrator rights; zip explicitly does not. No unpacking tool named. Store route also exists, with no stated prerequisite. Both x64 and arm64 variants are published.
- **macOS.** One published package, a disk image, mounted by opening it. No unpacking tool or permission named. Both Intel and Apple Silicon variants are published for 4.5, and the release notes say 4.5 is the last release with an official Intel macOS build.
- **Linux.** An archive, decompressed by hand. This is the one platform where the sources name a hard requirement: TAR, not 7-zip. Snap and distribution packages are alternatives, and Snap requires snap to be installed already.
- **All three.** Steam works on all three and needs the Steam client plus an account.

The sources name Windows, macOS and Linux as the platforms Blender is available for, and no others.

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

## Separately: a source I read but kept out of the answer

`Blender Sources/requirements/requirements_2026-08-01.txt` is Blender's requirements page, taken 2026-08-01. It was not on my list of places an answer may come from, and it carries no version in its address, so I have kept it out of the answer proper. For completeness, the only sentences in it that touch this question are these, and I flag that they describe installing and running rather than unpacking:

> "Whether it's on a USB stick, sitting on a folder on your desktop, or fully installed, Blender runs out of the box.
>
> No installation needed, no Internet connection required."

and

> "For Windows there is an installer available if you wish to add an icon on the desktop, register .blend file extensions, etc."

and, for Linux, under Minimum: "Distribution with glibc 2.28 or newer (64-bit)". That page also says "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur)", and lists under older versions that "Blender 4.5 LTS supports Intel chips, macOS 11.2 (Big Sur), requires GPU with Metal 2.2". Those name 4.5 in so many words. None of them says anything about unpacking a package. Cited by file name, since this page has no per-version web address. Read 2026-08-01 (the date the file records); the file itself was in the folder when I read it on 2026-08-30.

---

2026-08-30 — Claude Opus 5 (Claude Code) — Builds, Blender 4.5
