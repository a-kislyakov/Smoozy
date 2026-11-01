# Builds — Blender 4.5, 5.0, 5.1, 5.2 and 5.3 side by side

What Blender publishes, what a computer must already hold before a published build can be opened, how an installed build can be asked what it is, and what putting a Blender on a computer or taking it off does to what is already there.

## The machines behind the tests

**The Creator's own computer.** A MacBook Pro with an Apple M1 Pro chip, Apple's own 14-core graphics (Metal) and no other graphics card, 16 GB of memory, with a screen of its own. **It is the only machine Blender 5.3 was run on.** 4.5 to 5.2 were run on it under macOS 26.5.2, except question 10's runs, which were under 26.6.2; 5.3 was run under 26.6.2.

**A rented Linux machine with a screen** — rented on Vast.ai and reached over SSH, with one NVIDIA GeForce RTX 4090 and a display of its own, so Blender's window could be opened there and photographed. The provider calls it a container and **testing says it is a whole virtual machine**, with its own kernel, its own system services and its own disk. It is still a share of somebody else's hardware and was shared with other work throughout, so nothing recorded from it says how long anything took.

**A rented Linux machine that was deliberately bare** — a container in Romania with one NVIDIA Quadro P4000 that nothing here used, and no display at all. **Being bare is the whole instrument.** It arrived with `tar` and `gzip` and nothing else that opens a package — no `xz`, no `unzip`, no `zip`, no `7z`, no `file` — and none of the libraries Blender links. Everything under question 10 that says what a computer needs before a build opens rests on that, and the same tests on a machine that already had those tools would have shown nothing.

The Blenders on the first two machines are 4.5.12, 5.0.1, 5.1.2 and 5.2.0; the tests under question 10, on the Mac and on the bare machine alike, were run against 4.5.12, 5.0.1, 5.1.2 and 5.2.1, and every build hash matched between those two machines. All were put there from Blender's own downloads. **For 5.3 the Mac ran one build: Blender 5.3.0 Beta, built 2026-10-16, hash `5d8151335b6b`.** How that one came onto the Mac is not recorded.

**"Both machines", "either machine", "neither machine" and "the rented machine" mean a different pair in different parts of this document.** Under question 10 they mean the Creator's Mac and the bare machine; everywhere else they mean the Creator's Mac and the rented Linux machine with a screen. **Either way they speak of 4.5 to 5.2 only.** No answer rests on all three. A contrast between these machines does not establish a version difference.

**For 5.3, nothing ran on Linux or Windows, nothing was downloaded, and no second copy of 5.3 could be started** — question 9 says why. So a 5.3 answer set beside a 4.5–5.2 answer that only a rented machine gave is not established, never a difference. **A 5.3 result is set against a 4.5–5.2 result only where both came from the Mac.**

**The Mac is not one unchanging machine.** Its operating system moved on between the runs, and so did its own Blender 5.2, from 5.2.0 to 5.2.2. So a difference between an older Mac reading and a 5.3 Mac reading can belong to the Mac or the day as easily as to the version. **Where 5.3 came out differently from what 5.2 had recorded, Blender 5.2.2 was run beside it on the same Mac the same day, as a check.** Where 5.2.2 did what 5.3 did, the difference is not 5.3's; where it did what the 5.2 record says, it is. 5.2.2 is no column of its own, and each answer says what the check showed, or that none was run.

**5.3 here is a beta build, and every other build in this document is a finished release.** Where the 5.3.0 Beta differs from the others by test, that is what this Beta does on the Mac; nothing below says the released 5.3 does the same. And where the Beta shows its stage, no unfinished build of 4.5 to 5.2 was ever asked, so whether an earlier beta would have shown the same is not established.

**Every observed result below settles the machine it was run on, and nothing else.** Where a test was run on more than one, the result names each machine. Several results agree across 4.5 to 5.2 on each computer but differ between the computers. Those contrasts do not establish version differences or a general rule for every computer using either operating system. **The tested macOS builds are application bundles and the Linux builds are folders of files**, so several checks do not apply to both forms.

**Windows is untested here, in every version.** No observation below settles a Windows result, on a desktop or Server edition. This is a gap in testing, not a version difference.

---

## The answer in short

### Where Blender itself differs between versions

- **1 and 10 — macOS processor variants.** Intel builds stop at 5.0.
- **1 — minimum macOS named by the application.** Higher in the 5.3.0 Beta on the Mac.
- **1 — Linux folder contents.** Differences among 4.5 to 5.2; 5.3 untested.
- **3 — help output.** Its length and arguments change, including one added by the 5.3.0 Beta on the Mac.
- **3 — `bpy.app` names.** Changes among 4.5 to 5.2 on Linux, and two additions in the 5.3.0 Beta on the Mac.
- **3 — `portable`.** Changes in the 5.3.0 Beta on the Mac.
- **3 — build options.** Changes at 5.0 and in the 5.3.0 Beta, within the tested systems.
- **3 — `xr_openxr`.** Changes at 5.1 on the Mac.
- **3 — bundled Python.** Version changes among 4.5 to 5.2; 5.3 matches 5.2.
- **3 — macOS `platform:` text, bundle version field and Help menu.** Details differ; the question gives each version.
- **2 and 7 — command names.** More are listed from 5.2, without changing either answer about builds.
- **10 — Python-module requirements.** The required Python changes at 5.1; 5.3 untested.

The questions give the documented or observed basis and the machines for each difference. **Every observed 5.3 entry is the 5.3.0 Beta's, on the Mac alone**; the finished release is untested.

### What is not established

- **1 and 3 — the finished 5.3 release and other unfinished builds.** The 5.3.0 Beta cannot settle either.
- **1 — naming and marking beyond the tested releases.** Includes release candidates, daily builds, experimental builds and the Python module.
- **1 — the 5.3 minimum on an older Mac.** The application field was read; compatibility was not tested.
- **2 — a general listing of published builds.** A readable download-server folder is established; a general listing service is not.
- **2 — untried fetch routes and channel choices.** Includes named Snap channels, stores, the installed distribution package and the 5.3 module.
- **3 — the unexplained argument growth and build options.** Includes what the removed or added options mean, and 4.5's Mac `--qos` entry.
- **3 — documented build-option values against installed builds outside 5.2.** Only partial values are available for comparison.
- **3 — other download routes.** One tested Snap/blender.org pair cannot settle every route or version.
- **4 — a download-page default, a particular LTS choice, and advice for render-only machines.** The question separates these from the documented advice.
- **5 — two 5.3 copies sharing settings, and a saved preference passing between copies.** The first is inferred for 5.3; the second was never tested in any version.
- **5 — contention, shared caches and other install pairings.** Includes the Windows installer's populated target folder and the incompletely documented warnings and thumbnail tool.
- **6 — copying over a running 5.3, and automatic updates during a render.** Untried.
- **6 — crash reports after copying over a running build.** Conflicting 5.1 results leave this open.
- **7 — a complete list of installations.** The tested listings miss copies; they do not settle every possible search.
- **8 — removing a running 5.3, moving a running build, and using an uninstaller.** Untried; the different settings-folder contents also remain unexplained.
- **9 — starting half or redone 5.3 copies, interrupted removals and store installs.** The 5.3 copying result alone does not settle these.
- **9 — interrupted extraction across versions and the effects of signature complaints.** The interruption test used one Linux archive.
- **10 — unpacking 5.3.** Its published files, checksums, sizes, disk image, archives and module requirements were not read or tested.
- **10 — the module route working end to end outside 5.2.1.** Only that release was installed and run.
- **10 — 5.3 notarisation.** The Mac's disabled assessment cannot establish it.
- **10 — account and package limits.** Includes standard Mac accounts, ordinary Linux accounts outside 5.2.1, browser-fetched packages and wrong-processor packages without translation.
- **10 — building from source and unpacking unfinished packages.** Neither was tested.
- **Every question — Windows and other untested machines.** No observed answer here settles them.
- **1 to 10 — 5.3 on Linux.** No Linux test was run for it.
- **2, 3, 5, 7, 9 and 10 — Steam and the Microsoft Store.** No store build was tested.

### Where only the documentation differs

- **1 — the manual's splash-screen picture.** The pictured version changes; the tested splash screens show their own build.
- **1 and 10 — the macOS architecture sentence.** 5.0's and 5.1's manuals retain Intel support after its removal; 5.2's and 5.3's reflect the change.
- **10 — Windows package names in release-note bug titles.** Different names are recorded; this does not establish different published package kinds.

---

## 1. What kinds of build does Blender publish, and how is each kind named and marked?

**How it is known.** The kinds and release status are **documented** in all five. Marking published LTS builds from the manual's version list is **inferred** for 5.3. The installed-build results are **observed** in 4.5 to 5.2 on the Mac and rented Linux machine, and in the 5.3.0 Beta on the Mac. The published files are observed for 4.5 to 5.2 only (question 10).

**The same in all five.** Blender publishes four things, described in the same words by every version's manual:

- **Stable Release** — the latest features, considered stable without regressions, roughly every three months.
- **Long-term Support** — for long-lasting projects, supported for two years, no new features, API changes or improvements, one a year, with occasional minor patches written as a third number.
- **Daily Builds** — rebuilt daily from work in progress, not as thoroughly tested, and may break or crash. Ones marked Alpha are still changing; ones marked Beta are feature-complete and being refined.
- **Build from Source** — the source code, free to reference or build.

Every version states the same ladder of stability: Alpha, then Beta, then Release Candidate, then a final release. Every version marks long-term support by putting the letters LTS after the version number in its manual's list of versions. Blender 4.5 and 5.2 are themselves long-term-support releases, and their documents show the same suffix in the manual's own title and in the release notes; 5.0's and 5.1's documents show it in neither, and neither of those is an LTS release. 5.3's show it in neither, and its release notes say 5.3 is in Beta until 4 November 2026, developed in the branch `blender-v5.3-release`. Every version documents the same two separate marks a running build carries — a release status of alpha, beta, rc or release, and a compilation type of Release or Debug. Neither of those two ever carries LTS, in any version.

Packaging is described the same way in all five: Windows gets a zip file and an installer, for x64 and arm64; macOS gets a disk image; Linux gets an archive. Steam is offered on all three systems, Snap and distribution packages on Linux, the Microsoft Store on Windows.

Three things came back identically from the tests in 4.5 to 5.2 **on both machines**, and in 5.3 on the Mac. Each installed build holds one processor architecture — `arm64` on the Mac, x86-64 on the rented machine, and no build on either holds two. The splash screen shows that build's own version in its top-right corner — `5.3.0 Beta` on the 5.3 build. And the LTS marking, where the release carries one, appears in the splash corner and never in a bundle's own information file; the 5.3.0 Beta's information file carries no stage word either. A Linux build has no information file, and the oldest macOS a Mac build names in it is under *Where it differs*.

**The manual's splash picture differs; the tested behaviour does not.** 4.5's manual picture reads `3.6.1`; 5.0's, 5.1's and 5.2's read `5.0.0 Alpha`; 5.3's reads `5.3.0 Alpha`. These are **documented pictures**, not readings of the installed builds. The observed splash screens above show their own build's version instead.

**What a published build file is called, which no version's sources give.** The 5.2 release folder on Blender's own download server was read from the rented machine. It holds `blender-5.2.0-linux-x64.tar.xz`, one macOS file `blender-5.2.0-macos-arm64.dmg`, six Windows files across two processors and three kinds, and two checksum files. **So a name is the word blender, the three-number version, the operating system, the processor and the file kind — and no stability word and no LTS marking appears in any of those file names**, though 5.2 is an LTS release. That folder was a finished release's, and no daily-build, alpha, beta or release-candidate folder was ever read, which is exactly where a stability word would sit — so the absence is an absence in a stable release's names and nothing wider. A checksum file is published beside the archives, which every version's findings recorded as absent from the sources; the fetched archive matched it. **The release folders of 4.5 to 5.2 have all been read, and the naming pattern holds in every one; what the four sets of files do not share is set out under question 10.** 5.3's release folder was not read.

**Where it differs, version by version.** On three points: which macOS processor variants are published, what a Linux build's folder holds, and the oldest macOS a Mac build names as the one it needs.

**Blender 4.5.** Its manual says both Intel and Apple Silicon are supported on macOS. Its release notes say Intel support is deprecated, and that this will be the last release with an official Intel macOS build. Its Linux folder carries `textures` and `usd`, which 5.0 to 5.2 do not.

**Blender 5.0.** Its release notes list, under Compatibility, "Support for Intel Macs has been removed". Its manual macOS page still says both are supported. The published 5.0.1 packages have no Intel macOS build (question 10), so that manual sentence is out of date.

**Blender 5.1.** Its manual macOS page says both are supported. The undated requirements page says Blender 5.0 and later require Apple Silicon. The published 5.1.2 packages have no Intel macOS build (question 10), so this manual sentence is out of date too.

**Blender 5.2.** Its manual says plainly that Blender 5.0 and later only support Macs with Apple silicon. Its Linux folder has no `blender-softwaregl`, which 4.5 to 5.1 carry. The rest of a Linux build's folder is the same in all four.

**Blender 5.3.** Its manual says, as 5.2's does, that Blender 5.0 and later only support Macs with Apple silicon. **The 5.3.0 Beta's bundle names macOS `13.0` as its minimum**, where 4.5 to 5.2 name `11.2`. That was read off the application on the Mac, with no same-day check beside it, and no Mac older than 13.0 was tried, so what one does with the build is not established. Its Linux folder was not looked at, and its published files were not read.

**Verdict: it differs.** Blender 4.5 publishes a macOS build for Intel Macs; from 5.0 it does not. Two release notes state the change — 4.5's calling itself the last, and 5.0's calling it removed — and 5.3's manual says Apple silicon only. The published packages corroborate the split for 4.5 to 5.2, including an Intel 4.5.12 package actually opened and run on the Mac (question 10). Separately, the Linux folder contents differ in 4.5 to 5.2 on the rented machine; 5.3's Linux folder was not tested. The oldest macOS named by the application moves from `11.2` to `13.0` in the 5.3.0 Beta, observed on the Mac; running it on an older Mac was not tested.

**Not established.**

- **What a daily build, an alpha or a release candidate reports.** Untested in every version. The one unfinished build asked anything is the 5.3.0 Beta, on the Mac.
- **Whether the released 5.3 names macOS 13.0 as its minimum too**, and what a Mac older than 13.0 does with the 5.3 build.
- **Whether a Release Candidate is published as its own download or is only a stage.** Named in the ladder by all five and explained by none. Snap carries a `candidate` channel for every release line, which is a name rather than an explanation.
- **Blender built as a Python module, as a published kind.** Only 5.1's and 5.3's documents record it, from the Python documentation, together with the statement that it is not on the official download page and that a pre-compiled one is available through PIP. The other three say nothing either way. **That is a silence in three documents, not a change in Blender** — and the `module` name itself is present in 4.5 to 5.2 on both machines and in the 5.3.0 Beta on the Mac, and reads `False` in every one. **That the module really is published for 4.5 to 5.2, and what a computer must hold before it can be fetched, is settled under question 10**; how it is named and marked is not, and nothing about 5.3's published module was read.
- **Whether the naming pattern above holds for a release that is not a finished one, or for 5.3.** The release folders of 4.5 to 5.2 have been read and the pattern holds in each, but every one of them is a finished release's folder, which is exactly where a stability word would not sit. 5.3's folder was not read.
- **Where daily builds live, how often the markings are applied to a name, and what the word "experimental" covers as a published category.** Not established. 5.3 documents experimental preferences as daily-build only; that does not define a published category.
- **Linux, for 5.3.** No Linux machine ran anything for 5.3.
- **Windows.** No Windows machine ran it; one can be rented.

---

## 2. Can builds of each kind be listed and fetched by something other than a person at a web page?

**How it is known.** All five versions answer **not in the sources** for any listing of what blender.org publishes, and **documented** for the fetch routes, from the manual's Linux, Windows and Steam installation pages. Each build's own command list and its whole argument list were read in 4.5 to 5.2 on the Creator's Mac; in 5.3, on the Mac, the command list was read and the argument list was searched for the words download, fetch, install and update. On the rented Linux machine the two Linux routes the manual describes — Snap and a distribution's package manager — were exercised, in one run each.

**The same in all five.** Every version's manual states outright that Blender has no update system of its own: it is designed not to require an internet connection, so you update it yourself. Every version documents the same routes that do fetch without a person at a web page, and describes them the same way:

- **Snap, on Linux** — a single command, `snap install blender --classic`, and updates arrive automatically afterwards.
- **A Linux distribution's package manager** — which may also list packages, notify about updates and set up menus, at the price of possibly being out of date or built without some features.
- **Steam and the Microsoft Store** — both fetch, and both are documented as updating by themselves: Steam in every version, the Microsoft Store in every version but 5.1, whose document does not record it either way. Every version describes both only as a person clicking in those applications' own windows.

Every version's manual also says that once you have a download, putting it in place can be automated with ordinary file-management tools — which is about placing a build, not obtaining one.

5.1's and 5.3's documents add that the Python-module build is "available via PIP"; the other three's findings say nothing either way, which is a silence and not a change.

No build-fetching argument or command was found in 4.5 to 5.2 on either tested machine. The 5.3.0 Beta's argument search on the Mac found none of the words download, fetch, install or update, and its command listing names nothing about builds. **For 5.3 this is a search result and a reading of command names, not a test of what every command does.**

**What the rented Linux machine settled, in one run each.** These are not per-version results. They are facts about two routes, found on one machine on one day.

- **Snap serves a listing a machine can read.** `snap info blender` printed about a hundred and fifty lines: the publisher is the Blender Foundation, and there is a channel for every release line back to 2.79, each carrying a version number, a date, a revision number and a size. Each channel is named for its release line, with the letters `lts` in the name where the line is a long-term-support one, and each exists in four forms — stable, candidate, beta and edge. **It is a listing of published Blender builds that a machine can read without a person at a web page.**
- **Snap really does fetch, and really does update itself.** `snap install blender --classic` with no channel named delivered whatever `latest/stable` held that day, which was 5.2.0, published by the Blender Foundation. `snap refresh --time` shows a repeating timer with a next time already set, so the documented automatic updating is real.
- **A distribution package is a different kind of build, and it is far behind.** Ubuntu's own `blender` package is version 3.0.1 against blender.org's 5.2.0 — the manual's warning that a distribution package may be outdated is borne out by more than two whole release series. It depends on about forty system libraries including the system Python, where a blender.org build brings its own, and it would put a program at a path the whole machine shares. **It was examined and not installed**, so what it does once installed is not established.
- **A release folder on Blender's download server can simply be read**, and it carries a published checksum that the fetched archive matched. That is a folder listing, not the download page. **It is not confined to that machine or that day** — the folders of 4.5 to 5.2 were read again from the Creator's Mac, and what they hold is set out under question 10.

**Where it differs, version by version.** Only in the command listing itself, and not in a way that changes the answer. Blender 4.5, 5.0 and 5.1 each offer one command, `extension`. Blender 5.2 and 5.3 offer the same three — `asset_listing`, `extension` and `maketx`. The commands were listed by name in 4.5 to 5.2 on both machines and in 5.3 on the Mac, and none of the names is about Blender builds; the three were never run.

**Verdict: not established for every version.** A machine-readable listing is not described in any version's sources. Observation supplies two partial answers: Snap's listing on one Linux machine, and readable release folders on Blender's download server for 4.5 to 5.2. Neither settles a general listing of every published kind. The fetch routes are documented alike in all five. The tested build arguments and command names add no build-fetch route, within the searches described above. None of the Linux routes, the download server or the module's package index was tried for 5.3.

**Not established.**

- **Whether Blender provides a general listing service covering releases and daily builds**, and whether daily builds sit at a predictable address. Not established. Readable folders for the four finished releases above are established; the main download page was not read.
- **What Steam or the Microsoft Store actually deliver** — which of the four kinds, and whether a particular version can be asked for. Silent in every version and untried: the Microsoft Store is Windows only and no Windows machine was asked, Steam is on neither machine, and installing a shopfront to test it would be a change to a machine rather than a test.
- **What a distribution package does once installed.** Read and simulated on the rented machine, never installed, because installing it would have replaced a path every other piece of work on that machine shared.
- **Whether asking Snap for a named channel really delivers that release.** Only the unnamed default was run.
- **What the 5.3 Python-module route offers.** Untested. Published module files for particular releases of 4.5 to 5.2 were read under question 10; those are established, rather than an unanswered question about all versions.
- **What the three commands 5.2 and 5.3 offer actually do.** `asset_listing` and `maketx` were listed by name and never run, so that none of them is about Blender builds is a reading of three names and not a test result.

---

## 3. Can an installed Blender be asked what it is — which version, and which kind of build?

**How it is known.** Version and release status are **documented** in all five. Identifying LTS from the program is **not in the sources**. The detailed outputs below are **observed** in 4.5 to 5.2 on the Mac and rented Linux machine, and in the 5.3.0 Beta on the Mac. The complete documented build-options value is compared with installed builds for 5.2 only.

**The same in all five.** There are five routes, and every one of them works the same way in every version, **in 4.5 to 5.2 on both machines and in 5.3 on the Mac**.

- **`--version` from a terminal** prints the version on its first line, then the build date and time, the commit date and time, the commit hash, the branch, the build platform, the build type, three very long compiler and linker flag lines, and the build system. Then it exits. The argument carries no platform note in any build's own help on either machine, unlike its neighbours, which are marked Windows only or Windows and Linux only. The commit hash, the commit date and the branch are the same on both machines for a given version; the build time is not, and `build platform` names the machine.
- **The `bpy.app` values, read with no window open.** Every one of them came back in every version on every machine it was asked on. `version_cycle` reads `release` in 4.5 to 5.2 and `beta` in the 5.3.0 Beta, and is text in all five; `build_type` reads `Release` and is bytes — the same types everywhere. `build_system` reads `CMake` everywhere. `module` reads `False` everywhere. `build_platform` reads `Darwin` on the Mac and `Linux` on the rented machine — an operating-system difference, not a version one. `portable` is under *Where it differs*.
- **The system-information file, written with no window open.** It was written in every version on every machine it was asked on, and reports the version, branch, commit date, hash, type, build date and time, the platform, and the path Blender is running from.
- **The About Blender panel**, opened from the Blender icon at the far left. It exists in 4.5 to 5.2 on both machines and in 5.3 on the Mac. It carries fewer facts than `--version` everywhere: no build time, no build platform, no build type, no build system. On the Mac it shows four rows in all five — Version, Date, Hash and Branch. On the rented machine it shows those four and a fifth, `Windowing Environment: X11`, which is exactly the row the documentation says appears on Linux — a machine difference, not a version one.
- **The bar along the bottom of the window**, which shows the plain version number in 4.5 to 5.2 on both machines and never the LTS marking. In the 5.3.0 Beta it reads `5.3.0 b` — set out below.

**The LTS marks are observed, not documented by these findings.** A way to identify LTS is **not in the sources** in all five; that silence is not a statement that no such way exists. In the tests in 4.5 to 5.2 on both machines, the letters ride on `version_string`, on the first line of `--version`, in the splash corner, on the About panel's Version row, and in the system-information file's title and `version:` line. They do not appear in `version_cycle`, in the Mac bundle information, on the bottom bar, or in the published finished-release file names. In 5.2's complete `bpy.app` listing they appear in `version_string` only. **This is the same rule in the tested releases:** 4.5 and 5.2 are LTS lines and print the letters; 5.0 and 5.1 are not and do not. The letters LTS appear nowhere in the 5.3.0 Beta's `bpy.app` listing.

**How the 5.3.0 Beta marks its stage, on the Mac.** `version_cycle` reads `beta`, one of the four documented values. The word `Beta` rides on `version_string` (`'5.3.0 Beta'`), on the first line of `--version`, in the splash corner, on the About Blender panel's Version row, and in the system-information file's title and `version:` line — the same places the LTS marking rides on 4.5 and 5.2. Like the LTS marking, it is not in the bundle's own information file, and the letters `beta` appear nowhere else in the `bpy.app` listing. **Beyond `version_cycle`, which is documented to carry the stage, it parts from the LTS rule in one place: the bar along the bottom of the window reads `5.3.0 b`**, where LTS never appears. A 5.2.2 build photographed the same way beside it, the same day, with the same factory settings, showed the plain `5.2.2`. Every build in the other four columns is a finished release and no beta of 4.5 to 5.2 was ever asked, so **whether any of these marks is new in 5.3, or is what every beta shows, is not established** — the bottom bar above all.

**Where it differs, version by version.** In nine particulars. Each was settled by the same test run in all five versions, and each names the machine it was run on. Five scopes govern the whole list. 5.3 was asked on the Mac alone, so every 5.3 particular rests on the Mac and is set only against the Mac's readings of 4.5 to 5.2. The help output is longer on Linux in every version run there, which is a machine difference and not a version one, so the two machines' line counts are given separately and never set against each other. The `bpy.app` name counts of 4.5 to 5.2 are the same on both machines, but which names came and went among them was settled by setting the four Linux builds' lists against each other — which of these names changed on the Mac is not established — so that half rests on the rented machine alone; the two names 5.3 adds were found on the Mac, against a 5.2.2 build's list. The Python numbers of 4.5 to 5.2 are the same on both machines, though what compiled that Python differs between them, which is an operating-system fact, and anything written to run inside Blender crosses a Python major-version line between 5.0 and 5.1. And the bundle version field is macOS only: a Linux build has no bundle and no such file.

**Blender 4.5.** Help output 434 lines on the Mac and 447 on the rented machine, with no `qos` anywhere in the Linux help. 70 names on `bpy.app` on both machines; 4.5 alone holds `debug_ffmpeg`. 45 build options on both machines, including one named `collada` set to `True`. `xr_openxr` `False` on the Mac and `True` on the rented machine. Bundled Python 3.11.11 on both. System-information `platform:` line `'macOS-26.5.2-arm64-arm-64bit'` on the Mac; on Linux the same string as the other three, `'Linux-6.8.0-87-generic-x86_64-with-glibc2.35'`. Bundle version field `4.5.12 2026-07-21`. Help menu on both machines: Manual, Release Notes, Tutorials, Support, User Communities, then Report a Bug and Save System Info.

**Blender 5.0.** Help output 466 lines on the Mac and 479 on the rented machine, with `--qos <level>` marked "(Windows only)" on both. 70 names on `bpy.app`, now holding `memory_usage_undo` and not `debug_ffmpeg`. 44 build options, no `collada` name at all. `xr_openxr` still `False` on the Mac and `True` on the rented machine. Bundled Python 3.11.13 on both. Same `platform:` string as 4.5 on the Mac. Bundle version field `5.0.1`. Help menu on both machines: Manual, Support, User Communities, Get Involved, Release Notes, then Report a Bug and Save System Info.

**Blender 5.1.** Help output 498 lines on the Mac and 505 on the rented machine, with `--qos` on both. 71 names on `bpy.app`, the new one being `cachedir`. 44 build options, no `collada`; `xr_openxr` now `True` on the Mac and still `True` on the rented machine. Bundled Python 3.13.9 on both. `platform:` line now `'macOS-26.5.2-arm64-arm-64bit-Mach-O'` on the Mac; unchanged from the others on Linux. Bundle version field `5.1.2`. Help menu as 5.0 on both machines, but the last entry written `Save System Info...` with a trailing ellipsis.

**Blender 5.2.** Help output 518 lines on the Mac and 525 on the rented machine, with `--qos` on both. 71 names on `bpy.app`. 44 build options, no `collada`, `xr_openxr` `True` on both machines. Bundled Python 3.13.13 on both. `platform:` line with the `-Mach-O` suffix on the Mac. Bundle version field `5.2.0`. Help menu as 5.1 on both machines, ellipsis and all.

**Blender 5.3 — the 5.3.0 Beta, on the Mac alone.** Help output 521 lines, with `--qos <level>` marked "(Windows only)". Set beside a 5.2.2 build's help on the same Mac the same day — 518 lines, as 5.2's — the only argument added is `--debug-gpu-backend-no-fallback`, which makes Blender fail rather than fall back to another graphics backend when the one chosen is not supported, and skips its check of that backend. 73 names on `bpy.app`; set beside the same 5.2.2 build's 71, the two added are `autoexec` and `autoexec_override`. 46 build options: 5.2's 44 on the Mac, every one with the same name and value, plus `io_spz` and `opentimelineio`, both `True`; no `collada`; `xr_openxr` `True`. The build options rest on 5.2's own Mac record, with no same-day check beside them. Bundled Python 3.13.13, as 5.2. `platform:` line `'macOS-26.6.2-arm64-arm-64bit-Mach-O'` — the `-Mach-O` suffix as in 5.1 and 5.2; the change from 26.5.2 is the Mac's own update between the runs, not Blender's. Bundle version field `5.3.0`. Help menu as 5.1 and 5.2, ellipsis and all; the end of the third entry was hidden by a tooltip in the picture.

**`portable`.** It reads `False` on the Mac in 4.5 to 5.2 and `True` on the rented machine in all four. **The 5.3.0 Beta reads `True` on the Mac**, from the same command, and a 5.2.2 build asked beside it with the same settings, temporary and home folders read `False`. This establishes a difference between those Mac builds; 5.3 was not asked on Linux. 5.3 documents the value as `True` unless Blender was built to reference absolute paths, "on UNIX". Its meaning in the earlier versions is not established here. **It must not be read as saying where a Blender keeps its settings**: it read `True` on the tested Linux builds using the ordinary shared folder, and on the 5.3.0 Beta that names the ordinary folder under the user's home. Whether two 5.3 copies share that folder is **inferred**, not observed (question 5). The `portable` **folder** of **Add-ons 14** is a separate thing.

**The documented `build_options` constant, and which machine it describes.** The 5.2 Python API documentation prints a constant value for `build_options`. On the Creator's Mac, that build's own value differs from it in exactly three names — `coreaudio`, `jack` and `pulseaudio`, all sound-system options. On the rented Linux machine, that build's value is **identical to the documented constant, character for character**. **So the documented value is this tested Linux build's value.** Treating it as a description of every 5.2 build is right on one tested system and wrong on the other. Those same three options read the opposite way on the two machines in 4.5 to 5.2, and the 5.3.0 Beta matches the earlier Mac values. Windows is untested. For 5.3 only the documented entry `experimental_features=False` is available for comparison, and it matches the tested Beta. A complete comparison is not established.

**What a build from a different route reports.** On the rented machine a Snap Blender 5.2.0 and the blender.org 5.2.0 reported the same version string with LTS, release cycle, branch, hash, build date and time, and build options. Their locations differed. **This settles those fields for that pair on that machine.** Other versions and routes remain untested.

**Verdict: it differs.** Every route is the same everywhere and every route works everywhere it was asked — in 4.5 to 5.2 on both machines, and in 5.3 on the Mac. What differs is the detail each build reports: the argument list and whether `--qos` is in it, which things `bpy.app` holds, `portable` on the Mac, what the build was compiled with, the Python inside it, the exact platform string on macOS, the bundle's version field on macOS, and the Help menu. Every 5.3 difference is the 5.3.0 Beta's, on the Mac.

**Not established.**

- **What dropping the `collada` build option means for Collada import and export, and what the two options the 5.3.0 Beta adds stand for.** Nothing in any version's findings connects any of them to a feature. All that is established is that `collada` is reported by 4.5's build on both machines and by none of the others, and `io_spz` and `opentimelineio` by the 5.3.0 Beta alone.
- **What grew in the argument list.** Between 4.5 and 5.2 no two argument lists were set side by side name by name on either machine. Between 434 and 518 lines on the Mac, and 447 and 525 on the rented machine, what was added is unrecorded. Six further names are accounted for by test under **Add-ons 13** — `--log-list-categories`, `--log-show-basename`, `--log-show-timestamp`, `--log-show-memory`, `--log-show-source` and `-X` — but thirty-two lines were added between 4.5 and 5.0 alone, so most of the growth is still unaccounted for. The step from 5.2 to 5.3 on the Mac is the one argument named above.
- **Whether 4.5's Mac help prints `--qos`.** Settled on Linux, where it does not. Blender 4.5's own findings never say either way about the Mac, and the Mac's 434-line 4.5 help was never searched.
- **What a daily build, an alpha or a release candidate reports, and whether a beta of an earlier version marks itself as the 5.3.0 Beta does.** Untested in every version; the 5.3.0 Beta is the only unfinished build asked.
- **Whether the released 5.3 reports what the 5.3.0 Beta reports** — its argument, its two names, `portable`, its two build options.
- **Anything about 5.3 on Linux** — its help length, its `bpy.app` names and `portable`, `xr_openxr`, the About panel's rows, and its build options set beside the documented constant. Not run.
- **Whether the documented `build_options` constant matches the build, in 4.5, 5.0, 5.1 and 5.3.** Done for 5.2 on both machines. For 4.5, 5.0 and 5.1 the documented constant was never written out in full, and for 5.3 only its last entry was, so nothing more can be compared. **That is a gap in what was recorded, not a difference between versions.**
- **Whether a Steam or Microsoft Store build reports anything different.** Only blender.org builds and one Snap build were available.
- **Whether any of this holds on Windows.**

---

## 4. Does Blender name one release as the one to use for ordinary work?

**How it is known.** All five versions answer **documented** for the two statements that carry the answer. No hand test on any machine bears on this question; it is answered from documents alone in every version.

**The same in all five.** The answer has two halves, and both are identical word for word in 4.5 to 5.2; 5.3 quotes the same words as far as it quotes them.

**No single release is named for everybody.** The manual's installing page hands the choice to the reader: which package is right depends on your own requirements for new features against stability, a studio might want long-term support, a hobbyist might want newer features, others might just want to test what is coming, and there is something for everyone. Every version says exactly that. The stable release is described without being preferred over the others.

**For a project, the recommendation is LTS.** Every version's deployment page — the page written for animation studios and schools — says that new versions may add, remove or change things that affect the results of production files, that for a given project it is advisable to use a single LTS version, and that LTS versions receive bug fixes for two years. **That names a kind, not a release number**, and it is advice per project rather than advice about which release everybody should be on. All five say so.

**Verdict: the same everywhere.** No version names one release. All five give the same non-answer for general use and the same LTS recommendation for project work.

**Not established.**

- **Whether the download page presents one release as the default.** That page is in no version's list of sources, and 4.5, 5.1, 5.2 and 5.3 say so in as many words; 5.0's document does not raise it. This is where such a recommendation would live, and it is owed. The one thing a machine could be made to show is that `snap install blender --classic` with no channel named delivers `latest/stable`, which was 5.2.0 on the day — **that is Snap's own default and not Blender naming a release**, and every version's findings say so.
- **Which LTS release to use when several are supported at once.** Named as open by 4.5 and unanswered everywhere.
- **Whether the advice differs for a machine that only renders.** Named as open by 5.2 and unanswered everywhere.
- **Which release is the current LTS.** Each version's own list says only what was true when it was written: 4.5's and 5.1's list 4.5 as the newest LTS and mark 5.0 and 5.1 without the suffix; 5.2's release notes say 5.2 is LTS and supported until July 2028; 5.3's list marks 4.5 as LTS, and its release notes call 5.3 a Beta until 4 November 2026. **That is the LTS line moving on, not the advice changing.**

---

## 5. Can one Blender installed on a computer cause trouble for another that is already there?

**How it is known.** Coexistence, the ordinary settings layout, the macOS overwrite and the registration setting are **documented**. Displacing another default is **inferred** from that setting. Matching same-version settings paths are **observed** for 4.5 to 5.2 on the Mac and rented Linux machine; for 5.3 sharing remains **inferred**, with only the installed copy's path observed on the Mac. The registration sequence below is observed on the rented Linux machine only, before 5.3; its controls are documented for Windows and Linux only.

**The same in all five.** Every version says the same things, in the same words.

**Co-existing is intended.** A download is a self-contained folder that can be put anywhere, several versions can live on one computer at once, and putting them there can be automated. Every version says it on the deployment page, the Linux page and the Windows page alike.

**The ordinary settings layout separates versions by their two-number version.** Every version documents version folders and says preferences have to be imported between versions. **This is the default layout, not an unavoidable shared folder:** 5.3 documents a `portable` folder and `BLENDER_USER_RESOURCES` as ways to use another location. Its inference that same-version builds share settings assumes neither overrides that location.

**Three ways one Blender can affect another.** Registration makes an installation the default for opening blend-files and generating thumbnails, for the user or for all users. **That it displaces the previous default is inferred from the documented setting**, and observed in the Linux sequence below. On macOS the documented update overwrites the application in Applications; keeping both requires another name or location. Imported settings can include incompatible add-ons and cause startup errors — **documented** for 4.5, 5.0 and 5.1. Whether that warning is documented for 5.2 and 5.3 is not established; no behaviour change follows from that gap.

**The thumbnail tool.** 5.3 documents a small tool added during installation to show blend-file thumbnails; on macOS it is an extension that can be switched off in System Settings. Whether this is documented or behaves differently in earlier versions is not established. Blender's bundled Python does not touch the system Python by default — **documented** for 5.0, 5.2 and 5.3, and not established for the other two here.

**What both machines showed, in 4.5 to 5.2.**

- **Two copies of the same build, in two different places, use one settings folder.** On the Mac both printed the same user path and the same system path; on the rented machine both printed the same user path, `/root/.config/blender/<two-number version>`. On both machines only the path to the build's own bundled files followed the copy, and the settings folder holds one folder per two-number version and no more. **This is observed for these builds on both tested machines.** One difference between the machines: the system resource path comes back as an empty string on the rented machine and as a real path on the Mac.
- **Every build prints the four registration arguments** — register, register for all users, unregister, unregister for all users — each marked "(Windows & Linux only)", on both machines.
- **Every build registers a preferences panel labelled `Operating System Settings`.** That was checked in all four builds on the rented machine. Whether it is on screen is where the machines part: on Linux the Preferences System page shows it, reading `Open blend files with this Blender version` above `Register` and `Unregister` buttons with a `For All Users` checkbox, and it sits between `Display Graphics` and `Network`. On the Mac neither that section nor `Display Graphics` appears at all — the Mac page shows Cycles Render Devices, Network, Memory & Limits, Video Sequencer and Sound. One screenshot per version was supplied on the Mac, so what sits below Sound there is unknown in all four equally.

**What the Mac showed in 5.3.**

- **The installed 5.3.0 Beta, left to work out its own settings folder, named the ordinary one under the user's home**: `…/Library/Application Support/Blender/5.3`, with its bundled files inside its own application and its system path `/Library/Application Support/Blender/5.3` — the same shape as 4.5 to 5.2. **The second copy never started** (question 9 says why), so whether two copies of 5.3 share one settings folder was not observed, and whether a preference saved by one is read by the other was not run.
- **On the Mac the 5.3.0 Beta works out its settings folder and its cache folder under the user's real home even when the home folder is pointed elsewhere.** `BLENDER_USER_RESOURCES` did move its settings folder; with that and the home folder both pointed elsewhere, the cache folder stayed under the real home. Not asked of any other version.
- **The four registration arguments** print with "(Windows & Linux only)", as in every other version; none was run.
- **The Preferences System page** shows the same five sections as 4.5 to 5.2 on the Mac, no `Operating System Settings` section and no `Register` control, and in 5.3's picture nothing below Sound.

**What registering actually does, settled on the rented Linux machine.** The whole sequence was run once, using a different build at each step, and put back afterwards.

- **Registering writes a desktop file naming that build**, an icon, a mime type, and sets that mime type as the default. For one user it writes under the home folder; asked to do it for all users it writes under `/usr/local` instead, and then also makes links in `/usr/local/bin` and sets up a thumbnailer, which the one-user form did not.
- **Registering a second Blender takes the association from the first, by writing over its file.** No new file appeared; the one file both had written now named the second build. So the answer every version's documents only inferred is yes, and the mechanism is replacement rather than any kind of choosing.
- **Unregistering clears whatever registration is there, not only its own.** The command was run from a build that had never registered anything, and it removed another build's registration.
- **Unregistering does not put the machine back.** Rebuilt database files, empty folders and a preferences list were all left behind, and the system still answered with the name of the removed desktop file.

**This is one sequence, not four results.** Each version played one part in it, so it shows what registration does on that machine and says nothing about any version behaving differently from another. It was never possible on the Mac, where the controls do not exist.

**Two builds of one version from two different sources.** On the rented machine a Snap Blender 5.2.0 and the blender.org Blender 5.2.0 both reported the same settings folder. **So builds from two different sources carrying the same two-number version do share one.** One version, one machine, one run.

**Verdict: the same everywhere, within these qualifications.** Several Blenders are meant to coexist, and their ordinary settings folders are separated by two-number version. Another location can be chosen. The identified risks are the default file association, the macOS overwrite, and imported preferences; the missing warning in two versions does not establish a change. Two same-version copies shared a folder in 4.5 to 5.2 on both tested machines; **for 5.3 that remains inferred and untested.** The Linux registration result is one sequence involving 4.5 to 5.2, not a separate test of each version; 5.3 was not tried there. The tested Mac builds have no registration controls.

**Not established.**

- **Whether two copies of 5.3 share their ordinary settings folder.** Inferred, not observed. **Whether a preference saved by one copy is read by another was not tested in any version**; matching paths in 4.5 to 5.2 do not add that test.
- **Anything about 5.3 on Linux** — the settings folder there, the `Operating System Settings` panel, and what registering does. Not run.
- **What registering does on Windows.** The controls are marked for Windows and Linux alike; only Linux could be tried.
- **What two Blenders running at once do to each other** — contending for the graphics card, memory, the temporary directory or the search path. Not in the sources in any version, and none of those four contentions was tested here. Questions about sharing an output folder belong under **Frames 14** and **The machine 7**; these findings do not settle them.
- **Whether a build from a shopfront and one put there by hand collide.** Answered for one pairing on one machine — a Snap 5.2.0 and a blender.org 5.2.0 share a settings folder, and the Snap install did not take the file association. Steam and the Microsoft Store are untried.
- **Whether the cache folder is shared.** Only 5.2's document records that the cache directory path carries no version number in it, and that the temporary directory holds render layers, physics cache, the copy-paste buffer and crash logs. 4.5, 5.0 and 5.1 say nothing about the cache path either way, and 5.3 found no statement about it. **That is a silence, and a silence is not evidence of a change.** Set beside it, though: question 3 records a name `cachedir` arriving on `bpy.app` at 5.1 and absent from 4.5 and 5.0, by test — the name count rising from 70 to 71 on both machines, and which name it was settled on the rented Linux machine alone. Nothing in this project says whether the two are the same thing. What `cachedir` reads has been read, and holds no version number: `/root/.cache/blender/` from the 5.1 and 5.2 builds on the rented Linux machine, and `/Users/<user>/Library/Caches/Blender/` from the 5.3.0 Beta on the Mac. Nobody tested whether two Blenders disturb each other through it, in any version on either machine.
- **What the Windows installer does when it finds its target folder already populated.** Inferred as a risk by 5.2 and 5.3, untested.

---

## 6. Does installing a Blender disturb a render already running on the same computer?

**How it is known.** The answer is **not in the sources** in all five. The two copying cases are **observed** in 4.5 to 5.2 on the Mac and rented Linux machine. For the 5.3.0 Beta, only copying another Blender beside the running one was tested, on the Mac.

**How these renders were started.** Every render here was started from the command line, with the frames named there. These tests do not settle what another way of starting a render prints; that is a separate question under **Reports 9**.

**The same in all five on the Mac, and in 4.5 to 5.2 on Linux — putting a second Blender beside a running render.** Each version was made to render thirty frames of its own startup scene from a throwaway copy of itself, with no window, and a whole second Blender was copied in beside the running one part-way through. On both machines, in 4.5 to 5.2, the copy finished cleanly, the render carried on, all thirty numbered images were written, and the render ended by itself with an exit code of zero and no error printed. **The 5.3.0 Beta did the same on the Mac**: its render ran from the installed application rather than a throwaway copy, a 4.5.12 application was copied in beside it part-way through, and all thirty images were written, exit code zero, no line holding the word error.

**The two machines disagree on copying a Blender over the running one, and both records stand.** The same render was started, and part-way through a Blender was copied straight over the exact folder the render was running from. **This was not run for 5.3 on any machine**: it needs a second, running copy of 5.3, and none could be started (question 9).

- **On the Creator's Mac**, in 4.5 to 5.2: the copy finished, the render carried on, all thirty images were written, exit code zero, no error.
- **On the rented Linux machine**, in 4.5 to 5.2: the render **stopped**. The shell reported a bus error and a dumped core, the render process ended with exit code 135, and four images of thirty were on disk at the end. The last line each log carried was an ordinary progress line for the frame being worked on. Blender printed no error of its own. The test was repeated for 5.2 on its own and again in three versions, the same result each time.

**The two tested machines differ; the versions on each do not.** One runs macOS and one Linux. These tests do not establish the cause or how every other machine on either system behaves. Both results stand.

**Verdict: not established for every version.** Putting a second Blender beside a running render left it alone in all five on the tested Mac and in 4.5 to 5.2 on the tested Linux machine. Copying over the running build left the render alone on that Mac and killed it on that Linux machine, alike in 4.5 to 5.2. **That case was not tried for 5.3 on either machine, nor any 5.3 case on Linux.** The whole answer rests on these tests, not on a documented guarantee for other machines or scenes.

**Not established.**

- **What copying a Blender over a running render's folder does in 5.3**, on macOS or on Linux, and whether a second Blender beside a running render leaves it alone on Linux in 5.3. Not run.
- **Whether a crash report is reliably written when a Blender is copied over the folder a running render is using**, and whether 5.1 was alone in writing one. A Linux 5.1 run wrote `/tmp/blender.crash.txt`; a repeat of the same test on the same machine printed no such line and wrote no such file. Both results stand. Neither a reliable result nor a version difference is established.
- **Windows.** Untried in every version. This matters more here than elsewhere: the three systems install Blender in visibly different ways, so a silence about it is a silence about a third situation. Several versions' documents say as much.
- **An installer rather than a plain copy.** Neither machine has one — a macOS build is copied and a Linux archive is extracted. The Windows installer needs administrator rights and was not tried in any version.
- **A shopfront's automatic update replacing files under a running render.** Snap, Steam and the Microsoft Store all update by themselves, by the documentation quoted, and none was tried against a running render. Snap's automatic updating was confirmed to be real on the rented machine, which makes this case more worth knowing about rather than less.
- **Why the two machines part.** Nothing in any version's findings explains it, and nobody should build on a guess about why.

---

## 7. Can a computer be asked which Blenders it holds?

**How it is known.** A Blender-provided list of installations is **not in the sources** in all five. The partial documented and inferred routes are below. The installed-build and operating-system listings are **observed** on the Mac in 4.5 to 5.3 and on the rented Linux machine in 4.5 to 5.2; they are not complete searches of every possible record.

**The same answer in all five: no Blender-provided general list is established.** No relevant argument, command or value was found in the tested 4.5–5.2 builds. The 5.3 command names add none; its argument search covered fetching words only (question 2), so that search does not settle every possible listing argument. The executable's own location is **documented** for 5.0 to 5.3 and **observed** in all five, within the machines named above; it reports that running build, not other installations.

**Partial routes.** A Linux package-manager listing is **documented** in every version. Default file locations are **documented** for 5.0 to 5.3, but a download can sit in any folder; those locations cannot establish a complete list. A Steam installation appearing in the client's Library tab is **documented** for 5.3; its status in earlier documentation is not established here. The Windows uninstall instruction is documented, but treating it as an installed-version list is **inferred** in 4.5 and 5.3. None of these incomplete answers establishes a version change.

**What macOS's own tools do, from one run covering the whole machine.** The computer held four Blenders in the Applications folder and four more copies of the same four builds sitting in an ordinary working folder. Three ways of asking — a plain listing of the Applications folder, the system's file index, and the system's own applications report — each named the four in Applications and **none of them named any of the four sitting in an ordinary folder**. Asking the index about that folder directly returned nothing at all. The applications report also gives a version number, a location and a signer for each one it names. **The same three were asked again with five Blenders in the Applications folder, the 5.3.0 Beta among them, and two copies of the Beta in an ordinary work folder**, and the answer had the same shape: each named the five in Applications and neither copy, and asking the index about the work folder returned nothing. The applications report's version line for the Beta reads plain `5.3.0`, with no stage word.

**What a Linux package manager does, from one run covering the whole machine.** The machine held five Blenders — the four researched versions extracted by hand, and a fifth build of 5.0.0 that came with the rented image and was never asked anything, so even its version rests on the folder name. **Neither package manager named any of them**: `apt` and `dpkg` reported no blender package at all, and Snap's own listing did not carry one either, because none had been put there that way. Asking the shell which `blender` it would run named one only, the image's own. The alternatives system said there were none. **Only a plain search of the file system found all five.** A Blender was then installed through Snap, and Snap's listing grew by exactly that one build and no other; the rest stayed invisible to it.

**The tested listings were incomplete.** The Linux package listings missed the extracted builds; a file-system search found them. On this Mac, the three tested listings missed the copies in the work folder, including the two 5.3 copies. **This does not establish that copied Blenders leave no record, or that every tool on either operating system will miss them.** These are observations of the named tools and machines, not version differences.

**Verdict: not established for every version.** No complete Blender-provided list is established. The tested operating-system listings were incomplete on both machines, including the Mac run with 5.3 present. Linux was tested before 5.3 only. Neither a general discovery rule nor a version difference follows.

**Not established.**

- **Windows.** Whether the Windows "Apps" list names every Blender, including ones from the Microsoft Store and Steam. Wanted by 4.5's, 5.0's and 5.3's documents, and untried — a Windows computer can be rented and this has not been asked of one.
- **Whether a Blender simply put in a folder appears in Windows' installed-program listings.** Untested. The named Mac and Linux listings missed the tested copies; this does not answer whether another tool can find them. The Linux file-system search did find them.
- **Whether the Windows installer, the Microsoft Store or Steam leave any record a machine can read.** Silent in every version. Snap's record was read on the rented machine and holds only what Snap installed.

---

## 8. What becomes of a render that is running when the Blender it is using is removed?

**How it is known.** All five versions answer **not in the sources**. Every one searched and found nothing about removal while Blender is in use; the one removal instruction the manual gives is a Windows sentence about using Windows' own settings, which says nothing about anything running. The same test was run in 4.5 to 5.2 on the Creator's Mac and on the rented Linux machine. **It was not run for 5.3 on any machine**: it needs a running copy of 5.3 to delete or move, and none could be started (question 9).

**The same in 4.5 to 5.2, on both machines.** Each version was made to render thirty frames from a throwaway copy of itself, with no window and started from the command line as under question 6, and the whole folder it was running from was deleted part-way through.

- **The deletion was permitted every time**, on both machines, and completed at once. The folder was gone.
- **The render carried on every time.** All thirty numbered images were written, the images already on disk before the deletion were still there afterwards, and the render ended by itself with an exit code of zero and no error printed. On both machines the closing lines of the log show the last frame being saved and timed normally after the program had gone.
- **That version's settings folder was still there afterwards, and still populated**, on both machines.

**Verdict: not established for every version.** In 4.5 to 5.2 on the two tested machines, deletion left these renders running, preserved the images already written, and left the settings folder behind. This is an **observed** result for those machines and startup scenes, not a guarantee for another render or machine. **For 5.3 there is no test.** On the tested Linux machine, deletion allowed the render to finish where copying over its folder did not (question 6).

**Not established.**

- **What deleting or moving the running Blender does in 5.3**, on any machine, and what is left behind afterwards. Not run.
- **What moving a running build does in 4.5 to 5.2.** The earlier tests deleted the folder; moving it was not tested in any version.
- **Windows.** Untried in every version. Several versions' documents note that Windows may refuse to delete a running program's files at all, which would be a different answer, and nobody could try it.
- **An uninstaller rather than a deletion.** There is no uninstaller for a Blender on either machine, so the two could not be compared in any version.
- **Why 5.2's settings folder on the Mac listed three entries where the other three listed four.** Blender 4.5, 5.0 and 5.1 each showed `config`, `datafiles`, `extensions` and `scripts`; 5.2 showed `config`, `extensions` and `scripts`. No version's findings explain it, it was not looked at again on the rented machine, and it may only reflect what had been created on that Mac rather than anything Blender does. **Not established, and not to be built on** — do not read it as 5.2 dropping a folder.

---

## 9. Can installing or removing a Blender be done again safely on a computer where it was cut off part-way?

**How it is known.** Repeating an interrupted installation or removal is **not in the sources** in all five. The macOS interrupted-copy results are **observed** for 4.5 to 5.2 on the Mac. For 5.3 only the copying and signature checks were run there. The Linux interruption result used one 5.2.0 archive on the rented machine.

**The copying tool matters on this Mac.** Ordinary `cp -R` copies of 4.5 to 5.2 failed to start, whether interrupted or complete, with the signature check reporting modified bundle information. That result does not isolate interruption. The interrupted-and-redone results below used macOS's `ditto` instead.

With that tool, the result was the same in 4.5 to 5.2:

- **The half-copied Blender did nothing.** It printed no message at all — no error, no name of a missing file — and ended with exit code 137.
- **Running the same copy command again over the same folder, without deleting anything first, finished cleanly.**
- **The finished copy started and printed the right version**, matching what the installed build of that version prints.
- **The signature check on the finished copy reported a sealed resource missing or invalid**, and the files it listed as added were in every case compiled-Python cache files inside the bundle. The same in all four.

**What survives a removal is the same in 4.5 to 5.2 on both machines.** After a throwaway copy was deleted outright, that version's settings folder was still there and still populated, so a fresh copy put in afterwards lands on it rather than on a clean slate. Not run for 5.3.

**5.3, on the Mac: the copying half only.**

- **A copy cut off part-way was done again over the same folder, and finished cleanly.** A copy of the installed 5.3.0 Beta made with macOS's own copying tool was stopped after two seconds, with 719 MB arrived and the program file already whole. The same command run again over the same folder, deleting nothing first, ended without complaint.
- **The redone copy holds everything the installed application holds, plus one file the cut-off copy left behind**, under a hidden temporary name and the same size as the file beside it. So its signature check lists fifteen added files where the installed application lists fourteen. The earlier four redone copies listed only compiled-Python files. The interruptions were not matched at the same point in the copy, so the extra file in this one run does not establish a 5.3 change.
- **The half copy's signature check reported its information file as modified**; the ordinary copying command's whole copy, and the redone copy, reported a sealed resource missing or invalid. 4.5 to 5.2's ordinary-command copies had reported the information file as modified. That is not a difference between versions: on the same day an ordinary-command copy of a 5.2.2 build reported a sealed resource missing too, as did the installed 5.2.2 itself — both installed applications had files added inside them since they were signed.
- **Whether any of the copies starts was not observed.** On the Mac, a fresh copy of the installed 5.3.0 Beta, started from a terminal, did not start: it waited on a macOS prompt for a person, which macOS's own log names as a check of the application's integrity, and nobody answered it. The installed application it was copied from started normally throughout. Nothing here makes that a fact about 5.3: no copy of another version was started that day, and the installed 5.3 differs from the other Blenders on that Mac in the mark macOS sets on a download, which reads `0381` on it and `03c1` on the two others read, 4.5.12 and 5.2.2. What makes macOS ask is not established.

**The Linux shape of the question, settled on the rented machine.** Every version's documents quote the same single documented way an installation is said to go wrong, and it is not about interruption: on Linux, Blender's archive must be extracted with TAR, and 7-zip is not supported. **That limitation was exercised and it is real — and it fails silently.**

- **Extracted with TAR**, the archive produced a Blender that started and printed the right version.
- **Extracted with 7-zip, the tool reported success both times it was run** — it printed that everything was fine — **and the result will not run.** Two separate things are wrong with it. The main program lost the mark that says it may be run, so trying it was refused outright. Putting that mark back by hand was not enough: it then failed to load one of its own libraries, because 7-zip had turned all seventy-nine of the archive's internal links into plain files. The correctly extracted copy holds 5,592 files and 79 links; the 7-zip copy holds 5,671 files and no links.
- **An extraction cut off part-way can simply be done again over the same folder.** The interruption landed before the main program had been written, so there was nothing to run. Running exactly the same extraction again over the same folder, deleting nothing first, finished cleanly and produced the same file and link counts as a clean extraction, and the result printed output identical, line for line, to the build already on the machine.

**The interrupted extraction used one 5.2.0 Linux archive.** It matched Blender's published checksum. Its result settles that archive and those tools on the tested machine, not every version's interrupted extraction. The separate 7-zip tests covering 4.5 to 5.2 are under question 10.

**Verdict: not established for every version.** In 4.5 to 5.2 a copy of a Blender that was cut off part-way can be done again over the same folder, and the result starts and reports the right version — established in all four on the Mac for the macOS shape of the question, and once on the rented machine for the Linux shape. In 5.3 the redo finished cleanly over the half copy on the Mac, but **no copy was started, so whether the result runs is not established for 5.3**, and nothing on Linux was run. No version behaved differently from another anywhere it was tried. The documented 7-zip limitation, which every version quotes and none explains, is real and gives no warning that it has broken anything.

**Not established.**

- **Whether a redone copy of 5.3 starts and reports the right version, what a half copy of 5.3 does when started, and anything about 5.3 on Linux.** Not run.
- **The Windows installer and uninstaller.** Whether an interrupted installer run can be repeated, whether it offers repair, and whether an interrupted uninstall leaves an entry behind. Wanted by 4.5's, 5.0's, 5.2's and 5.3's documents, and untried — a Windows computer can be rented and this has not been asked of one.
- **Whether an interrupted extraction behaves differently by version.** One archive was extracted and cut off part-way, 5.2's. Nothing says a 4.5, 5.0, 5.1 or 5.3 archive behaves differently, and nothing shows that it does not. **The 7-zip half of this entry is not one archive** — it was run in 4.5 to 5.2 and is set out under question 10, where the figures are a 5.2.1 build's rather than the 5.2.0 one used here.
- **Whether Steam, the Microsoft Store, Snap or a distribution package recover from an interruption.** Steam and the Microsoft Store are on neither machine. Snap was installed once, cleanly, and never interrupted; the distribution package was never installed at all. Snap updates itself without being asked, which is exactly the case somebody would want to know about.
- **Whether the signature complaint prevents a fresh copy from starting.** The redone 4.5–5.2 copies started despite it. The 5.3 redone copy was not started; another fresh 5.3 copy waited on an integrity prompt. Whether that complaint caused the prompt is not established.
- **Whether an interrupted removal can be repeated safely.** No interrupted removal was tested in any version; deleting a complete copy does not settle that.

---

## 10. What does a computer need to have on it before a published Blender build of each kind can be unpacked?

**How it is known.** The named requirements are **documented** in all five; additional requirements inferred from the instructions are labelled below. Most practical requirements are **not in the sources**. Unpacking and starting the result are separate questions. Observations cover 4.5 to 5.2 on the bare Linux machine and the Mac, with each result's narrower scope stated below. For 5.3 only the installed application was read on the Mac; its published packages were not tested.

**The same in all five — what the sources name.** Every version's manual says these things, in the same words as far as each quotes them.

- **The stability of the build makes no difference to unpacking.** The procedure for installing a binary is the same for the latest stable release and for a daily build — **documented** in all five. That long-term support unpacks the same way is **inferred** in 4.5 to 5.2: it is described as a package beside the others and given no procedure of its own.
- **On Linux, TAR must be used and 7-zip is not supported.** **Documented** in all five, and it is the only place in any version's sources where a tool is named as required to unpack a Blender build. No version's manual names the archive's format anywhere, so what TAR must in turn be able to decompress is stated nowhere. 5.3 labels it **inferred** that a working TAR must therefore be present.
- **The Windows installer requires administrator rights; the Windows zip explicitly does not.** **Documented** in all five. No version names a tool for extracting the zip, and none says whether Windows' own extraction is enough.
- **On macOS the disk image is mounted by double-clicking it, and the application is dragged out.** **Documented** in all five; no tool, no rights and no other precondition is named. That the machine must therefore be able to mount a disk image is **inferred** in 4.5 to 5.2, and stated in no version — an instruction to mount one is not a statement that the machine must be able to; 5.3 records whether macOS needs anything beyond its own handling of disk images as not covered. The one security sentence on that page is about opening Blender for the first time, not about opening the package; 4.5 to 5.2 say so in as many words, and 5.3 quotes the same sentence.
- **Snap must already be installed.** The manual says so outright — **documented** in all five.
- **The Steam client must be downloaded and installed first** — **documented** in all five. The next step uses an account, which may be created then. **An account must be used; it need not already exist before that step.** The quoted instructions establish no version difference here.
- **The Microsoft Store's route is one sentence that names no prerequisite at all**, and so is a distribution's own Blender package. That the Store must be present, and that a distribution's package manager must be present, are **inferred** — in 5.3 as in the others — and stated in no version.
- **The Python-module build comes through PIP.** **Documented** in 5.1 and 5.3; that a Python with PIP must therefore be present is **inferred** in 5.3. The other three's findings say nothing either way.
- **A download is self-contained, can be extracted to any directory, and deployment can be automated with standard file management tools.** **Documented** in all five. It does not say that nothing else is ever needed — the Linux tool requirement stands in the same manual.
- **The downloaded file must match the machine's processor.** **Documented** in all five, and stated as a condition on which file to take rather than on unpacking it. No version says whether a mismatched one can still be opened.
- **What a computer needs to build Blender from source is not in the sources**, in 4.5 to 5.2. Each one searched, found the manual describing the source kind and then stopping, and said so. 5.3's document does not take it up.

**What no version's sources say, and every version searched for.** No tool named for the Windows zip. No disk-space figure of any kind. No checksum, signature or verification step before a package is opened. Nothing about signing, notarisation or quarantine on macOS — those words appear nowhere in any version. No runtime library or redistributable that must be present before unpacking. Nothing about having to mark the program runnable after extracting. Nothing about how a daily, alpha, beta or release-candidate package is shaped. 5.3's document is silent on the same things wherever it looked: no tool for the zip, no disk space, nothing on checksums, signing, notarisation or quarantine, and no system library.

**What the bare Linux machine settled, in 4.5 to 5.2.** These results are **observed** on that machine. Archive tests cover all four except the ordinary-account run, which used 5.2.1 alone; the module was installed and run for 5.2.1 alone too. Snap and the distribution-package listing describe the machine's routes, not four separate Blender runs. **None of it was run for 5.3.**

- **A tar program on its own does not open a Blender archive.** On a machine holding GNU tar 1.34 and no `xz`, all four archives refused with the same message — tar could not run `xz` — at exit code 2, and **nothing landed on disk in any of the four**. Naming the compression outright made no difference. **No version's sources say this**, so the documented requirement is incomplete in all four alike, not in one of them.
- **One further package turns that machine into one that opens all four.** Installing the tool the message named brought in exactly one package, taking the machine from 253 installed packages to 254, and all four archives then opened with no message at all.
- **The archive is a tar inside xz compression in all four**, read off the files' own first bytes before anything was installed and confirmed afterwards.
- **The line between opening the package and running what comes out is real and wide.** With the archives open and nothing else installed, none of the four would start. Each named `libX11.so.6` first, and the machine could not satisfy nine distinct libraries across ten lines — the same nine in every version. Nine packages were then asked for and the machine pulled in forty-two, after which all four started and every build hash matched the Creator's Mac. **Nothing about the unpacking needed any of those libraries, and the program needs all of them.**
- **The program comes out of the archive already marked runnable**, in all four, with nobody setting the mark. **An ordinary user account extracted and ran 5.2.1**, with the same file and link counts as the administrator's extraction. That account test was not repeated separately for the other three archives.
- **7-zip breaks the build in all four versions, and it says it worked.** Two passes over each archive both reported success and exited cleanly, and what came out will not run for two separate reasons: the runnable mark is gone, and every symbolic link has become a plain file. The tar copies hold 71, 75, 77 and 81 links and the 7-zip copies hold none, each with a matching number of extra plain files. Putting the mark back by hand is not enough — the program then fails on one of its own libraries. **The documented limitation is exact and it gives no warning of any kind.**
- **The snap route is closed on a machine with no snap.** The command the manual gives does not exist to be run, and the package that would provide it is not installed either. The manual's own assumption is the whole of the requirement.
- **A distribution's own Blender is far behind.** On Ubuntu 22.04 the offered version is 3.0.1 against blender.org's 4.5.12 to 5.2.1, and taking it would bring in 242 further packages. It was read and simulated, never installed.
- **The published source archive opens with exactly the same two tools as the binary one**, in all four versions, and each verified against its own published checksum first. What building it needs is a different question and is not answered.
- **Blender's Python module is published for all four versions**, and for this kind what the computer needs is a Python of one exact version and a working package tool. On that machine's own Python the install refused outright, naming no version it could offer. Which Python each version demands is below, and it is not the same for all four. **The module was actually installed once, for 5.2.1 alone**, on the Python 3.13.13 that Blender's own unpacked folder carries: it brought in eleven further packages, finished cleanly, and then reported the right version and named no program it was running from. **So the route is known to work end to end for one version on one machine, and for no other.**

**What the Creator's Mac settled, in 4.5 to 5.2.** Every one of these is **observed**.

- **The disk image opens with nobody clicking.** For all four versions the image was mounted, the application copied out and the image detached, all from a terminal with no window, no person, no password and no elevation — and the copied Blender then printed its own version. The account used is an administrator one, so this settles that nothing is *asked for*, not what a standard account would meet.
- **Every one of the four is signed and notarised**, and macOS's own assessment accepts each.
- **A downloaded package carries no mark that stops it being opened.** The images fetched here carried one attribute and it was not a quarantine mark — they were fetched by a program rather than by a browser, which is Smoozy's own case and not a person's. Separately, an image marked by hand as downloaded from the internet still mounted, still copied out, and the application still ran from a terminal, carrying the mark onto itself. **The mark stops neither the mounting nor the copying**; what it does at a double-click was not tested, and that is where the manual puts its approval step.
- **A package for the wrong processor still opens.** The Intel package that exists for 4.5 mounted, copied out and detached on an Apple Silicon Mac exactly like the matching one. Neither package holds both processors. It also ran there — but that Mac already carried Apple's translation layer, which was read rather than installed, so what a Mac without it would do is not settled.

**What the Mac showed of 5.3.** Every one of these is **observed**, on the installed 5.3.0 Beta; how it came onto the Mac is not recorded.

- **It holds one processor, `arm64`.** The oldest macOS it names is under question 1.
- **It is signed with Blender's own Developer ID**, the Stichting Blender Foundation's. Its signature does not check: fourteen compiled-Python files were added inside it after it was signed, which is the same kind of complaint question 9 records for copies of every version.
- **Whether it is notarised is not established.** macOS's own assessment accepted it, but assessments were switched off on that Mac that day, and a 5.2.2 build asked the same way the same day gave the same answer — so the reading belongs to the Mac's setting, not to the build, and cannot be set beside 4.5 to 5.2's.
- **It carries the mark a web browser sets on a download, naming Firefox**, and it started from a terminal in every test that started it — on a Mac whose assessment was switched off, so this cannot be set against the 4.5–5.2 result above either.
- **It takes 970,732 kilobytes on disk**, about 950 megabytes, already started and with those files added. The size of a 5.3 download was not read.

**What there is to unpack, read off Blender's own download server for 4.5 to 5.2.** 5.3's folder was not read. Each of the four publishes, for each of two Windows processors, three files — an `.msi`, an `.msix` and a `.zip`; exactly one Linux file, for one processor; and a source archive. So the Windows Installer File the manual names is an MSI, and "the Linux version for your architecture" offers one architecture. What the Microsoft Store actually delivers is untouched — the `.msix` sits on the download server beside the others, and no machine here has ever asked the Store for anything. **A published checksum exists for each of the four, in two forms, and every package fetched matched it**, on both operating systems — which every version's findings record as absent from the sources. **So a download can be verified before it is opened**, in all four, on both. How these files are named is under question 1.

**Windows package names in the documentation.** A 4.5 release-note bug title names the MSI installer; treating that as the published installer's format is **inferred**. A 5.2 bug title names an appx for the Store. No such format statement is established here for the other versions, including 5.3. These are differences in the recorded release-note titles, not in the published Windows package kinds: the four release folders above carry the same three kinds. What the Store delivers, and what 5.3 publishes, remain untested.

**Sizes, which no version's sources give, read for 4.5 to 5.2.** A Linux archive is 374 to 396 megabytes and becomes 1.17 to 1.28 gigabytes. A macOS disk image is 308 to 346 megabytes and becomes an application of 836 to 950 megabytes. A source archive is 85 to 94 megabytes and becomes about 304 to 315 megabytes. **A binary package roughly triples when it is opened, and the machine needs room for the download and the result at once.**

**Where it differs, version by version.** On two points, both observed, both from the same test run in 4.5 to 5.2. 5.3 was not tested on either.

**Which macOS packages exist to be unpacked.** Blender 4.5.12 publishes two — one for Apple Silicon and one for Intel. Blender 5.0.1, 5.1.2 and 5.2.1 each publish one, for Apple Silicon, and no Intel package at all. That is the same change question 1 records from the release notes, shown by the published files themselves, and it settles the sentence still standing on 5.0's and 5.1's manual pages against them. It also means the wrong-processor test above could only ever be run on 4.5. For 5.3 only its manual speaks — Apple silicon only — and its published files were not read.

**Which Python the published Python-module build demands.** The files published for 4.5.12 and 5.0.1 demand Python 3.11 exactly; those for 5.1.2 and 5.2.1 demand Python 3.13 exactly. Asking for the module on a Python 3.10 or a Python 3.12 returns nothing at all, for any version. The same macOS split appears here as in the disk images: 4.5.12 has an Intel file among its published module files and the other three do not. What 5.3's module demands was not read.

**Verdict: it differs.** Everything about the act of unpacking is the same in 4.5 to 5.2: the same documented requirements, the same silences, the same failure when a tar program stands alone, the same one package that fixes it, the same nine libraries the unpacked program then wants, the same silent breakage from 7-zip, the same terminal-driven route on macOS. What differs is what there is to unpack and what the machine must already hold for it — Blender 4.5 publishes a macOS package for a processor the other three do not, and the Python the published module build insists on crosses a version line between 5.0 and 5.1. **5.3's documents name the same requirements as the others', and none of the tests that settled the rest was run for it** — only its installed application was read, on the Mac. Everything observed here settles two machines and two operating systems for 4.5 to 5.2; Windows is untried in every version alike, which is a silence about what has been run and not a difference between them.

**Not established.**

- **Everything observed about unpacking 5.3.** What its Linux archive needs and whether 7-zip breaks it; whether its disk image opens with nobody clicking; which packages it publishes and what they are called; whether a published checksum exists for it; how big its downloads are; and which Python its module build demands. None was run or read.
- **Whether the 5.3 build is notarised.** Unreadable on the day, as above.
- **Everything about unpacking on Windows.** Whether Windows' own extraction opens the zip on a machine with nothing else installed; whether the zip really needs no administrator rights and the installer really does; whether the installer can be run with nobody present; what the Microsoft Store demands before it will install anything; and whether a wrong-processor Windows package still opens. Untested in every version, and **waiting on a batch rather than on equipment nobody has**. Any answer taken there must name the Windows edition it was taken on.
- **Anything through Steam.** The client can be installed; the account half cannot be reached, because creating an account or typing credentials into one is not something done on this project. So the route stops before it answers anything, in every version.
- **Whether the Python-module route works end to end for 4.5, 5.0 and 5.1, or at all for 5.3.** It was installed and run once, for 5.2.1 on the bare machine. For 4.5, 5.0 and 5.1 only the list of published files was read, so what each demands is settled and what it does once installed is not; for 5.3 not even the list was read.
- **What a computer needs to build Blender from source.** Opening the published source archive is settled in 4.5 to 5.2; compiling it was never attempted, and no version's sources describe it.
- **What a standard macOS account meets.** The macOS route was driven as an administrator account and nothing asked for elevation. Making a standard account would be a change to the Creator's own machine.
- **Whether the ordinary-user Linux result holds separately for 4.5, 5.0 and 5.1.** The ordinary-account extraction and launch used 5.2.1 only.
- **What a package fetched by a web browser carries on macOS.** The packages for 4.5 to 5.2 were fetched by a program, which does not set the mark a browser sets. The installed 5.3 application carries a browser's mark, but its package was never seen.
- **What a wrong-processor macOS package does on a Mac with no translation layer.** The Mac used already had one, and installing one would have been a change to that machine.
- **What a daily, alpha, beta or release-candidate package needs.** Only finished releases sit in the folders that were read, and no unfinished package was ever unpacked — the 5.3.0 Beta was already installed.
- **Whether an interrupted extraction differs by version**, which is question 9's, and unchanged by anything here.
- **Whether a distribution's own package behaves as its manual entry suggests once installed.** It was read and simulated and never installed, because installing it would have replaced a path the rest of the machine shared.
