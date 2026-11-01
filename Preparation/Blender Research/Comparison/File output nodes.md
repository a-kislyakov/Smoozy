# File output nodes — Blender 4.5, 5.0, 5.1, 5.2 and 5.3 side by side

What a File Output node in Blender's compositor can write, how its outputs are set up, where its files land, what happens when one of them cannot be written, and what it does when its scene is brought into another scene's video sequencer.

## The machines behind the tests

Five machines carry this topic, one line each; the equipment section of `Preparation/Blender Research/Method.md` holds every one of them in full. **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, Apple's own graphics and no second card, with a screen; it is the only machine Blender 5.3 ran on. **A rented Linux machine with an NVIDIA RTX 4090** — reached over SSH, a whole virtual machine rather than the container the provider calls it, its screen shared with other work; it carries the Linux half of questions 1 to 13. **A second rented Linux machine, with an RTX 4080 SUPER** — reached over SSH, no display server at all; it carries the Linux half of question 14. **A third rented Linux machine, with an RTX 5060** — reached over SSH, no display server at all; it carries the Linux half of question 15. **A fourth rented Linux machine, with an RTX 3060, in Norway** — reached over SSH, and it has a display server of its own, drawn in software rather than by the card; it carries the Linux half of questions 16 and 17, and every test of those two questions in 4.5 to 5.2 that needs Blender's own window on a screen was run there. The four rented machines are four different computers, so no figure from one may ever be set against a figure from another.

**Which Blender each machine carried, at the grain the equipment section supports and no finer.**

- **The Creator's Mac** — the section fixes its four as **4.5.12, 5.0.1, 5.1.2 and 5.2.1**, the last on hash `9e2066aef7ef`. **That is what the machine is, not what it was on every day this topic ran.** The same section says plainly that this Mac is the one machine here that has changed under the stage: its operating system and its unversioned 5.2 build both moved, and a 5.2.1 release candidate stood on it for part of 2026-08-26. So a Mac result is bounded to whatever that machine was on the day, and this document does not date questions 1 to 15 for 4.5 to 5.2. **Blender 5.3 ran on this Mac and on nothing else**: **5.3.0 Beta**, hash `5d8151335b6b`, built 2026-10-16, every test on 2026-10-18, under macOS 26.6.2, the machine read that day as an M1 Pro with its own 14-core graphics (Metal) and 16 GB of memory. Its window was opened from a terminal and driven by a script with Blender's own simulated pointer and keys, never by a person. Where 5.3 came out differently from 5.2's record, the same test was run again on this Mac the same day in **Blender 5.2.2 LTS**, hash `d13f752e3b9c`; an answer that leans on that run calls it **the 5.2.2 run**, and it is a result for that build on this Mac and for nothing wider.
- **The RTX 4090 machine** — **not recorded.** The section says only that all four researched versions were installed from Blender's own download server, with no version number and no hash for any of them. Questions 1 to 13 rest on a Mac-and-Linux pairing nobody wrote down.
- **The RTX 4080 SUPER machine** and **the RTX 5060 machine** — the section says each carried the four Linux x64 builds *at exactly the versions the Creator's Mac carries*. That is a match at version, with **no hash written down for either**, and because the Mac's own 5.2 build moved under the stage, a version match does not settle which 5.2 stood on either side.
- **The RTX 3060 machine in Norway** — **4.5.12, 5.0.1, 5.1.2 and 5.2.1**, matching the Creator's Mac to the exact commit, the last on hash `9e2066aef7ef`. The section dates the machine to 2026-08-26 and records that the Mac's release candidate was already gone when its tests ran. Questions 16 and 17 are the only ones here whose two machines are known to have run the same Blender.

**Linux agreed with the Mac on every test both machines ran, in questions 1 to 14.** Every file name, every pixel value, every refusal, every ordering. Not one thing Blender did came back differently. The one exception is the exact wording of a failed write, under question 5, where the message carries the operating system's own description of the fault in brackets — and what Blender did there was identical too. Question 15 stands apart: it carries two differences between its own two machines, and both are recorded under it.

**Questions 16 and 17 stand the same way as questions 1 to 14.** Every test a terminal can reach was run in all four versions on the Creator's Mac and on the RTX 3060 machine, and every file name, every count, every printed line and every refusal came back the same on the two. Their four tests that need Blender's own window were run on the RTX 3060 machine alone, in all four versions, and are marked as such wherever they appear.

**Blender 5.3 was run on the Creator's Mac alone.** So wherever a sentence below says *all four versions*, *every version*, *the four* or *both machines* without naming 5.3, it means 4.5 to 5.2, and *the newer three* means 5.0, 5.1 and 5.2; 5.3 is named wherever it is meant, and *all five* includes it. **A 5.3 result is only ever set against an earlier version's result from the Mac.** Where 4.5 to 5.2 were run on a rented machine alone, 5.3 has nothing on the same machine to stand beside, and the answer says so rather than calling the two the same or different.

A result settles only the machines it ran on. **Windows was not tested at all, in any of the five versions, and stays untested. Machines with several graphics cards are untested too.** Every result below for 4.5 to 5.2 settles macOS and Linux, or one of them, and every result for 5.3 settles macOS alone.

## The answer in short

These lists point to the full answers below. Observed results cover only the machines named there. Every 5.3 test used the Creator's Mac and the beta build named above.

### Where Blender itself differs between versions

- **1** — slot collections, starting format and published socket kinds.
- **2** — setting names, media type and the node's file-extension switch.
- **4** — folders inside output names and whether names can climb upwards.
- **5** — project path variables added by 5.3's release notes; the failure-message style is under 15.
- **6** — multi-layer naming and mode selection; 5.3's release notes also report a starting File Name change, with the earlier untouched defaults unestablished here.
- **8** — output-list controls and right-click menus, observed on the Mac.
- **9** — format choices; translation handling changed in 5.0 and custom metadata support in 5.1, by those versions' release notes.
- **10** — the finished-path panel, absent in 4.5.
- **12** — the default compositor tree changed in 5.3, by its release notes.
- **13** — frame numbering and 5.3's compositor effects.
- **14** — keyframes, drivers, per-frame scripts, replaceable trees and sidebar controls; 5.3's new tested behaviour is Mac-only.
- **15** — message style, the script-readable report collection and 5.3's OpenEXR routine names on the Mac. The machine differences in this answer are not version differences.
- **16** — refusal of the sequencer's own render command with no window.
- **17** — frame numbering and naming variables through a Scene strip.

### What is not established

- **Every question** — Windows; Linux and other graphics hardware for 5.3; machines with several graphics cards. An observed agreement settles only the tested machines and builds.
- **1** — an output count above two thousand.
- **2** — earlier starting folders and output display-transform defaults; 5.3's Override Node Format tooltip.
- **4** — separators under the untouched starting File Name before 5.3, and the earlier equivalents of the extra 5.3 cases.
- **5** — network folders in every version; project-backed `{project_root}` in 5.3; the earlier equivalents of 5.3's extra folder cases.
- **6** — untouched starting File Names before 5.3; earlier equivalents of the extra multi-layer cases.
- **7** — whether a group used twice writes once or twice before 5.3.
- **8** — disabling or hiding a socket before 5.3.
- **9** — other builds, untested image-size limits and 5.3's drawn format menu.
- **10** — the earlier equivalents of 5.3's multi-layer collision case, under 6.
- **11** — 5.3's drawn format menu, under 9.
- **12** — non-picture Vector input, channels beyond the files read back, earlier Image-only and Float-value cases, and feeding from 5.3's default Group Input in this topic's runs; Frames 13 records a node fed from a Group Input writing in 5.3.
- **13** — stopping during a write in 5.3; a File Output node with Sequencer on but no strips; the machine limits on window and stop tests.
- **14** — Linux for the window tests and 5.3's changed switches.
- **15** — network loss, locked files, simultaneous node and scene write failures, fixed byte counts, the varying OpenEXR reasons in 5.1 and 5.2, earlier window reports, and an unreadable blend file in 5.3.
- **16** — preview and windowed sequencer-command differences between versions; an empty sequencer; the Output-panel saving switch with a Scene strip, untried here. The compositor 2 holds tests without a strip in 5.2 and 5.3.
- **17** — multi-layer naming through a strip in 4.5, and untried library naming variables.

### Where only the documentation differs

- **4** — the documented folder rule does not follow the tested version split.
- **11** — 4.5's own descriptions disagree about a media type that its tested node does not have.
- **16** — only 5.2 and 5.3 describe File Output nodes as an alternative when the scene's own saving is off. Behaviour with a Scene strip is untested here; The compositor 2 records the plain case without a strip in 5.2 and 5.3. This is a documentation difference, with no behaviour difference established for a Scene strip.

---

## 1. What can a File Output node be made to write, how many outputs can it have, and what does each output carry?

**How it is known.** Partly documented in every version; in 5.3 that the node offers every format on the shared list is only inferred, and is overturned by the test under question 9. The differences below are all observed, in all four versions on **both machines**, which agreed on every line, and in 5.3 on the Mac.

**Common to all five.** The node writes image files to disk. It has nothing on its output side — the things it writes are its inputs. No version states any maximum number of outputs, and in all five a script added two thousand outputs with nothing refused. That establishes at least two thousand and nothing above it.

**Blender 4.5.** The inputs are called slots and come in two named collections, one for single images and one for layers. The test showed the two are two views of one list of inputs, not two separate lists. A new node starts on PNG, the same format the scene starts on. After two thousand outputs the node carried exactly two thousand sockets.

**Blender 5.0, 5.1, 5.2 and 5.3.** The two slot collections are gone, replaced by one list of items. A new node starts on Multi-Layer OpenEXR, which is *not* what the scene starts on — a File Output node added and left alone writes one multi-layer file rather than separate pictures. Each item carries a declared socket kind, which 4.5 has no setting for. After two thousand outputs the node carried two thousand and one sockets: the extra one is the blank socket the node keeps for adding more. In 5.3 an output asked for with an empty name was made and kept the empty name.

**The list of socket kinds an output may be set to, version by version.** Blender 4.5 has no such setting at all. Blender 5.0 publishes eighteen kinds. Blender 5.1 publishes twenty-three — the same eighteen plus five more. Blender 5.2 publishes twenty-four — 5.1's plus one for whole-number vectors. Blender 5.3 publishes the same twenty-four as 5.2, name for name and in the same order. The 5.1 and 5.2 lists were read on both machines and matched; 5.0's published list is established on Linux alone; 5.3's was read on the Mac.

**What the node will actually accept is far narrower than what it publishes.** The detail is under question 12.

**Verdict: it differs.** Blender 4.5 uses two slot collections, starts a new node on PNG, and has no socket-kind setting. Blender 5.0 to 5.3 use one item list and start a new node on Multi-Layer OpenEXR; 5.0, 5.1 and 5.2 each publish a longer list of socket kinds than the one before, and 5.3 publishes 5.2's.

**What the node draws on its own face** was photographed on Linux only in 4.5 to 5.2: 4.5 shows one folder field and a per-output file format beside each output's name, while 5.0, 5.1 and 5.2 show a folder and a base file name, the output names alone, and a row for adding another. The Creator's Mac pictures in those versions were of the sidebar rather than the node. In 5.3 the face was photographed on the Mac: the folder field and the File Name field one above the other, each with a folder button, one row per output showing its name alone, and a last row holding a plus — the layout the 5.0 to 5.2 pictures show, read on a different machine. In 5.3's sidebar the outputs sit in an **Images** panel with a plus, a minus and up and down arrows, and the selected output's kind is drawn as a **Socket Type** menu.

---

## 2. Which settings belong to an individual output, and which belong to the node as a whole?

**How it is known.** Documented in every version, and confirmed by observation — on both machines in 4.5 to 5.2, on the Mac in 5.3. The question the documents left open in all five — which display-transform setting wins — was settled by a hand test run in all five, which returned the same eight results each.

**Common to all five.** The folder and the fallback format belong to the node. The name and an optional own-format belong to the individual output. A display-transform setting exists at both levels in every version.

**Blender 4.5.** The node carries one folder setting, the format block, the display-transform setting and the active-input index. An output carries a sub-path, its own format block, its own display-transform setting, and a switch for using the node's format. The panel draws the folder as Base Path, the output's name as File Subpath, and the switch as Use Node Format. That switch has no description in the reference, none in the running program, and no tooltip on screen — in Blender 4.5 nothing in Blender says what it does. There is no media-type control anywhere on the node.

**Blender 5.0 and 5.1.** The node carries a folder, a separate base file name, the format block and the display-transform setting. An output carries a name, a socket kind, a vector width, a colour, an own-format switch and its own format block and display-transform setting. The panel adds a two-button Image / Multi-Layer EXR choice and a read-out panel listing the finished path of every output. The own-format switch is called Override Node Format and works the opposite way round from 4.5's Use Node Format, and its tooltip matches the reference word for word.

**Blender 5.2 and 5.3.** Everything 5.1 has, plus one more setting on the node: its own file-extension switch, drawn as a File Extensions checkbox above the list of finished paths. The setting was read out of the running program on both machines in 5.2 and on the Mac in 5.3. In 5.3 the sidebar draws the media type as two buttons, Image and Multi-Layer EXR, with no third; then a Node Format panel; then the Images panel holding the outputs, with the selected output's Socket Type and Override Node Format below it; then the Output Paths panel holding the File Extensions checkbox and one row per output. The words of 5.3's Override Node Format tooltip were not read.

**What a new node starts with, beyond its format — read in 5.3 alone.** The running program publishes an empty default for both the folder and the File Name. But a node made by a script starts with the folder `/tmp/` and the File Name `{blend_name}`, and a node added through Blender's window starts with the scene's own output path as its folder and the same File Name. An output made by a script reads its own display-transform switch as on, though the program publishes that switch as off. None of this was read in 4.5 to 5.2.

**The same everywhere: which display-transform setting wins.** While an output uses the node's format, only the node's setting changes the written file. While an output uses its own format, only the output's own setting changes it. All five versions gave the same eight results; in 4.5 to 5.2 the two machines gave the same eight pixel values to three decimal places, and 5.3 on the Mac gave the values 5.2 gave. The tested Mac and Linux values agree in 4.5 to 5.2; Linux is untested in 5.3.

**Verdict: it differs.** The settings are named differently before and after 5.0, 4.5 has no media type, and 5.2 and 5.3 alone carry a file-extension switch on the node. The precedence rule between the two display-transform settings is the same in all five.

---

## 3. Does each output carry its own folder as well as its own name?

**How it is known.** Inferred from the documents in every version, then confirmed by observation, in all four versions on both machines and in 5.3 on the Mac.

All five versions answer the same way: no. There is one folder setting, and it sits on the node. An output carries a name only — a sub-path in 4.5, a name in the others.

**Verdict: the same everywhere.** No version gives an output a folder setting of its own. Settled on macOS and on Linux for 4.5 to 5.2, and on macOS for 5.3; Windows untested.

One thing does differ, and it belongs to question 4: in Blender 4.5 an output's name can reach *out* of the node's folder, and in 5.1, 5.2 and 5.3 it cannot. That is recorded under question 4.

---

## 4. Can a folder be expressed inside an output's name rather than separately?

**How it is known.** Observed in all four versions on **both machines**, which agreed name for name and file for file, and in 5.3 on the Mac. Documented in 5.1, 5.2 and 5.3; the 4.5 and 5.0 documents are silent, and the tests do not follow that split.

**Blender 4.5.** Yes. Names are kept exactly as typed. Both a forward slash and a backslash make folders, several levels deep where asked, and missing folders are made without complaint. A name beginning with `..` climbs upwards and lands the file *outside* the node's folder. A name beginning with a slash does not go to the root; it is joined under the node's folder.

**Blender 5.0.** No. Every separator in a name is replaced with an underscore before the node keeps it, and every file lands flat in the node's own folder. No folder is ever made. In multi-layer mode the same replacement happens, so a layer asked for as `passes/diffuse` comes out as `passes_diffuse`.

**Blender 5.1, 5.2 and 5.3.** Yes. Both separators make folders, several levels deep. The node's base file name is joined to the front of the output's name before the split, so an output called `sub/shot` under a base name of `file_name` makes a folder called `file_namesub`. A name cannot climb upwards: `..` is turned into `_.` before the node keeps it. A leading slash does not make the path absolute. In multi-layer mode the slash stays in the layer's name and no folder is made; in 5.3 the file's own header was read, and holds one layer named `passes/diffuse`.

**Three more things were seen in 5.3 alone, on the Mac.** A slash in the node's own File Name makes a folder too: `sub/test` with an output `img` wrote `sub/testimg.png`. With the File Name left at its starting `{blend_name}`, an output named `sub/img` wrote the blend file's name joined to `sub`, then the folder — `deftestsub/img.png` from `deftest.blend`, `Unsavedsub/img.png` in a file never saved. And two outputs on one node named `sub/shot` and `sub\shot` go to one path: Blender reported saving it twice, with no warning, and one file was left.

**Verdict: it differs.** Blender 4.5 allows it and lets a name escape upwards. Blender 5.0 does not allow it at all. Blender 5.1, 5.2 and 5.3 allow it but hold the node's folder as a floor the names cannot climb out of.

This is a real difference in Blender, not in its documentation. The same test ran in all four versions on two operating systems and returned the same three results on each, and in 5.3 on the Mac it returned 5.1's and 5.2's. Blender 5.1's own release note agrees with the shape of it, saying the feature was possible in older versions and has now been restored.

**The documentation splits one way and Blender splits another, and the two must not be read as one.** Automatic creation of missing folders is documented for 5.1, 5.2 and 5.3 and unestablished from the documentation for 4.5 and 5.0. That split does not match the real one: 4.5 *does* make the missing folder, so its documents' silence is a gap in the documentation and nothing more, while 5.0 will not carry a folder inside an output's name at all — a real difference in Blender that the documentation split does not match.

**The backslash is the sharpest operating-system question in this topic, and what the runs settle is macOS and Linux.** A backslash is a folder separator on Windows and an ordinary character in a file name on macOS and Linux. On the systems tested here Blender treats it as a separator anyway: **in Blender 4.5, 5.1, 5.2 and 5.3 a backslash inside an output's name makes a folder, exactly as a forward slash does, and in Blender 5.0 it is replaced with an underscore before it ever reaches the disk, exactly as a forward slash is.** That is a difference between versions, and in 4.5 to 5.2 it holds identically on macOS and on Linux, so across those two systems it is no difference between operating systems; 5.3 was run on macOS alone. Each case was tested with a node whose only output carried a backslash, pointed at an empty folder of its own, so that writing nothing could be told apart from writing into a folder.

**What that does not settle is Windows** — the one system where the backslash is the separator the system itself uses, and so the only one that could show Blender following the system here. It is untested in all five versions. **And it must not be assumed that Blender ignores the operating system there**, because in a neighbouring place it does not: **Dependencies 20** records that on a Windows machine every forward slash in a recorded place is turned into a backslash when the file is opened, in all four versions, and that seven places are left alone — the node's own folder setting among them. So the separator does follow the operating system in some places on Windows and not in others, and which of the two a backslash inside an output's name would meet is unknown in every version.

---

## 5. Which folders can a File Output node be pointed at, including folders unrelated to where the Blender file sits and folders reached over a network?

**How it is known.** Partly documented in every version; in 5.3, that any full folder path can be entered is inferred from the manual's picture of the node. Everything decided below was observed, in all four versions on both machines and in 5.3 on the Mac.

**The same in all five.** A folder with nothing to do with where the Blender file sits is accepted, and works with the file unsaved. A folder that does not exist is made, with no error. The `//` form works and is taken as relative to the saved file. A folder that cannot be written to fails loudly and writes nothing at all — and that holds for *that* cause and not for every way a write can be refused: a volume with no room left does leave something behind, which question 15 records.

**One difference between the versions, in wording only.** Blender 4.5 reports a permission error naming the file. Blender 5.0 to 5.3 report an image-writing failure instead. That shape holds on both machines in 4.5 to 5.2, and on the Mac in 5.3. In all five the result is the same — a loud failure and an empty folder. The full shape of the two styles, line by line, is under question 15.

**And one difference between the machines.** The failure message ends with a description of the fault in brackets, and that description is the operating system's own words rather than Blender's. On the Mac, 4.5 said `Render error (Permission denied)` and 5.0 to 5.3 said `Render error (Undefined error: 0)`. On Linux, 4.5 says `Render error (Invalid argument)` and the newer three say `Render error (Success)`. **What Blender did is identical on both: it refused loudly and wrote nothing.** Anything reading that message for meaning would be reading the operating system, not Blender — and the bracketed reason is not even fixed for one cause on one machine in one run, which question 15 records.

One condition on the Linux answer, because without it the answer would have been the opposite of the truth: the account driving Blender there can walk straight through file permissions, so a folder marked unwritable was written to anyway. The answer above is from a run with that power dropped. A folder marked unwritable is only unwritable to somebody who cannot override permissions.

**Templates in the folder, and the variable only 5.3 has.** Blender 5.3's release notes add a `{project_root}` variable and variables a project defines to path templates. In 5.3 on the Mac, `{project_root}` in a node's folder with no project made is an invalid template: the node writes nothing and prints `Invalid path template in File Output node. Skipping writing file.`, saved file or not, while the render and the scene's own picture carry on. `{blend_dir}` in the folder names the saved file's own folder. What `{project_root}` does when a project exists was not tried. By those release notes, 4.5 to 5.2 have no such variable.

**Two more cases, tried in 5.3 alone.** A `//` folder in a file never saved wrote relative to the folder Blender was started from, with no error. An empty folder setting aimed the file at the top of the disk, `/`, saved or not, and failed loudly with the image-writing failure above — the Mac's ordinary user may not write there.

**Verdict for the project variable: it differs**, by 5.3's own release notes, and a test in 5.3 agrees.

**Verdict: not established for every version.** Every part that could be tested in every version behaves identically in all five. The network part of the question is unanswered in all five and could not be tested on any machine: the Mac has no network store, and the rented machine had no share mounted and nothing of the project's to mount. That silence is uniform, so it is no evidence of any version difference.

---

## 6. Can several outputs be gathered into one file rather than written separately?

**How it is known.** Inferred in 4.5, documented in the other four, and observed in all four versions on both machines, which agreed, and in 5.3 on the Mac.

**The same in all five.** Yes — as layers of one multi-layer OpenEXR file. Three outputs named `first`, `second` and `third` produced one file whose layers carry those three names; in 5.3 the file's header was read, and holds one layer per output, four channels each.

**Blender 4.5.** The two slot collections are two views of one list, and switching the node's format does not move anything between them. In this mode the node's folder setting is used as the *start of the file name*, not only as a folder: pointed at a folder written with a trailing slash the file was `0001.exr` inside it, and pointed at the same folder without the trailing slash the file landed beside the folder as `out0001.exr`. The written name carries the frame number and no output name.

**Blender 5.0, 5.1, 5.2 and 5.3.** A media-type setting is what switches the node between the two ways of writing, and the format list follows it — with the media type on Image, the multi-layer format is refused outright, in the same words on every machine. The single file takes its name from the node's base file name alone, with or without a trailing slash on the folder, and carries no output name.

**What a File Output node calls its file follows the node's mode.** In multi-layer mode no socket name reaches the file; in image mode it does. A freshly added node starts on PNG in 4.5 and on Multi-Layer OpenEXR in 5.0 to 5.3.

**The starting File Name is observed in 5.3 on the Mac.** It is `{blend_name}`: left untouched, it writes the blend file's own name, or `Unsaved` in a file never saved. 5.3's release notes describe changing the initial name from a fixed string to this template. That is documented evidence of a change. The earlier versions' untouched starting File Names are not established by these findings: their render examples set the name explicitly. They do not establish that 5.2 already used the template, or pin down the earlier default for each version.

**Two more cases, tried in 5.3 alone, on the Mac.** An output set to its own format — Override Node Format on, its format PNG — on a node in Multi-Layer EXR mode still went into the one file as a layer, and no picture of its own was written; the node's mode won. And two Multi-Layer EXR nodes aimed at the same folder and File Name left **one file holding both nodes' layers**, the second-made node's first, with Blender reporting the path saved twice and nothing else. In image mode two nodes aimed at one file leave one node's picture (question 10); in multi-layer mode, in 5.3, they were merged.

**Verdict: it differs.** All five can gather outputs into one file. Blender 4.5 names that file from the node's folder text and stamps a frame number on it; the other four name it from the node's base file name. The untouched starting File Name is observed only in 5.3 here; its release notes report a change to `{blend_name}`. In 4.5 the mode follows the format setting; in the other four it follows the media type.

A separator inside a layer's name follows question 4: 4.5, 5.1, 5.2 and 5.3 keep it inside the layer name, and 5.0 replaces it with an underscore. Same on both machines in 4.5 to 5.2.

---

## 7. What happens to a File Output node inside a group of nodes?

**How it is known.** Not in the sources, in any of the five versions — a uniform silence. Everything below was observed, in all four versions on both machines and in 5.3 on the Mac.

All five versions gave identical results, and so did both machines in 4.5 to 5.2. A File Output node inside a node group writes its file. Putting that group inside a second group made no difference. Muting the outer group node from outside stopped it writing anything. Nothing was reported to the terminal in any case, in any version, on either machine.

**A group used twice.** In all five, using the same group twice in the tree left one file, both uses carrying the same folder and the same output name. In 5.3 Blender's own `Saved:` lines were kept for each render, and they show the node **writing twice to that one path** — once per use. The 4.5 to 5.2 runs recorded only the one file left, and one file is what a single write and a write over itself both look like, so they do not say which happened.

**In 5.3, the other kind of group.** A File Output node in the tree of a second compositor effect writes during a render whichever effect is the active one, and writes nothing once that effect is switched off for render (question 13).

**Verdict: the same everywhere.** It writes from inside a group at every depth tried, and muting the group from outside stops it. Settled on macOS and on Linux for 4.5 to 5.2, and on macOS for 5.3; Windows untested. **Whether a group used twice writes once or once per use is not established for every version**: once per use in 5.3 on the Mac, not told apart in the other four.

---

## 8. Can one File Output node be switched off on its own, and can one output within a node be switched off on its own?

**How it is known.** Inferred or not in the sources across the five versions — in 5.3, that a muted node writes nothing is inferred, and a per-output switch is not in the sources. Settled by observation: the tests a terminal can reach ran in all four versions on both machines and in 5.3 on the Mac; the one that needs eyes on Blender's window was the Creator's on the Mac in 4.5 to 5.2, and in 5.3 was read from pictures of the window on the Mac.

All five versions gave identical results. Muting the whole node stops it writing — nothing at all appears on disk, and in 5.1 it was further noted that a muted node does not even make its folders. One output can be switched off on its own through its link: muting the link and removing the link both leave the other output writing normally. An output with nothing connected writes nothing at all — not an empty file, not a black one.

**The socket's own settings do not do it.** In 4.5 to 5.2 the socket's enable setting was on throughout and was never the way. In 5.3 both were tried: with an output's socket set to disabled, the setting read back as set and the output was still written; with it set to hidden, the setting did not take on a linked socket and the output was still written.

From the window, in all five: there is no per-output on and off control on the panel. Cutting or muting the link is the way.

**Verdict: the same everywhere.**

Two interface details differ without changing the answer, and both rest on the Mac alone. Blender 4.5 adds outputs with an Add Input button and offers only reorder arrows; 5.0 to 5.3 use a plus and a minus beside the list. And the menu that appears on right-clicking an output is longer in 4.5 than in the other four; in 5.3 it holds the same six entries 5.2's does — Reset to Default Value, Copy to Selected, Copy Data Path, Copy Full Data Path, Copy as New Driver and Online Manual. Neither offers a switch in any version. The 4.5 to 5.2 readings are the Creator's on the Mac; 5.3's was opened by a script's simulated click on the Mac; nothing from Linux stands beside either.

---

## 9. What restrictions, if any, apply to the kind of picture a File Output node can write?

**How it is known.** The format lists were observed in all four versions on both machines and in 5.3 on the Mac. What the panel draws and what the setting accepts agree in every version from 4.5 to 5.2. In 5.3 the panel's File Format menu could not be opened by the script, so only what the setting accepts was read.

**Blender 4.5.** The node takes fourteen formats: BMP, Iris, PNG, JPEG, JPEG 2000, Targa, Targa Raw, Cineon, DPX, OpenEXR MultiLayer, OpenEXR, Radiance HDR, TIFF and WebP. The multi-layer choice sits in this same list. Video is not in it. AVIF is not in it. A new node starts on PNG.

**Blender 5.0.** On the Image media type the node takes thirteen formats: JPEG, OpenEXR, PNG, WebP, BMP, Cineon, DPX, Iris, JPEG 2000, Radiance HDR, Targa, Targa Raw and TIFF. Multi-layer is no longer a format in this list — it is chosen with the media type instead. Video is absent. AVIF is absent.

**Blender 5.1, 5.2 and 5.3.** The same thirteen as 5.0 plus AVIF, making fourteen. Multi-layer and video are absent from the list in all three, for the same reason. In 5.3 the shared format setting publishes sixteen values, and the node on Image refused the multi-layer and video ones, naming the fourteen it takes.

**Verdict: it differs.** Blender 4.5 offers fourteen formats with multi-layer among them and no AVIF. Blender 5.0 offers thirteen with multi-layer moved out and still no AVIF. Blender 5.1, 5.2 and 5.3 offer those thirteen plus AVIF.

**Both machines printed the same lists, version for version, with AVIF arriving in 5.1 on each**, and 5.3 on the Mac printed 5.2's. So the difference is Blender's, not the operating system's.

**Other documented restrictions.** Alpha and colour depth depend on the format. The findings describe JPEG as 8-bit without alpha, PNG as 8- or 16-bit with alpha, and OpenEXR as 16- or 32-bit floating point with alpha. DPX and Cineon cover the visible range from 0 to 1.

**Two further changes are documented by release notes.** Blender 5.0's notes say File Output now takes translation into account and clips relevant translated inputs. Blender 5.1's notes say Image and File Output nodes now read and write custom metadata. These are documented changes even where another version's findings are silent; they were not tested here.

**Image size is not settled for this node.** In 5.1, applying the compositor's general output-size rule to File Output is inferred; it is not a measured limit. In 5.3 the findings also infer that the GPU compositor's documented cropping or scaling above 8k would constrain this node's input. No test here establishes that limit for the file it writes. Silence about it in the earlier findings is not a version difference.

**Not established.** The image-size limits above, and whether a differently built Blender offers a different list. Two builds of each of 4.5 to 5.2 have been tried — the Apple silicon macOS build and the Linux x64 build, both from Blender's own download server — and they agree; in 5.3 one build was tried, Blender's own 5.3.0 Beta for Apple silicon. That is not all builds, and it says nothing about a Blender built by anybody else. And what 5.3's File Format menu draws.

---

## 10. What happens when two outputs are aimed at the same name in the same place?

**How it is known.** Not in the sources, in any of the five versions — a uniform silence. Everything below was observed: the tests a terminal can reach ran in all four versions on both machines and in 5.3 on the Mac, and the one that needs the window was run by the Creator on the Mac and photographed on Linux in 4.5 to 5.2, and photographed on the Mac in 5.3.

All five versions gave identical results; the two machines agreed pixel value for pixel value in 4.5 to 5.2, and 5.3 on the Mac gave what 5.2 gave.

- Two outputs on one node cannot carry the same name. The second is renamed with a numbered suffix, and two files are written, each holding its own picture.
- Two separate nodes aimed at the same file give one file, and it holds the picture from the node that was made first. Four renders in a row gave the same answer, and building the two nodes in the opposite order swapped which picture survived. Nothing was printed. The two tested machines agreed in 4.5 to 5.2; 5.3 was run on the Mac alone.
- A file already on disk is replaced silently. The scene's Overwrite setting does not reach the node — switching it off changed nothing.
- The scene's Placeholders setting does not reach the node either. A two-frame animation with placeholders on produced the node's files and the scene's files, and no placeholder for the node.

In multi-layer mode, in 5.3 alone, two nodes aimed at one file were merged into it as layers rather than one replacing the other — question 6.

**Verdict: the same everywhere.** Settled on macOS and on Linux for 4.5 to 5.2, and on macOS for 5.3; Windows untested.

One interface difference, which changes nothing here: Blender 4.5 has no panel showing the finished paths, while 5.0 to 5.3 have one. In those four it flags nothing — nothing coloured, no warning mark — because Blender renames the second output before a clash can happen. That was seen on both machines in 5.0 to 5.2 and they agreed on every point, and on the Mac in 5.3.

---

## 11. Can a File Output node write a video file?

**How it is known.** Observed in all four versions on both machines and in 5.3 on the Mac. The refusals came back word for word the same on each, and 5.3's in 5.2's words. 5.3's own documents do not settle it.

All five answer no, and the panel offers no video choice in any of them; in 5.3 the media type is drawn as two buttons, Image and Multi-Layer EXR, and the format menu was not read. Setting the node's format to the video value is refused in all five, and a three-frame animation produced three pictures rather than a video file in all five.

The refusal reaches the node by two different routes. In Blender 4.5 there is no media-type setting on this node at all, and the format list simply has no video entry. In 5.0 to 5.3 the media type exists and will take only Image or Multi-Layer EXR, and the format list under it has no video entry either.

**Verdict: the same everywhere.** No version of the five can write a video file from this node. Settled on macOS and on Linux for 4.5 to 5.2, and on macOS for 5.3; Windows untested.

Worth recording, because it looks like a version difference and is not: Blender 4.5's release notes say this node's format settings gained a media type, while 4.5's own reference lists no such setting. The hand test settles it on both machines — there is no media type on this node in Blender 4.5.12. That is a contradiction inside one version's documents, not a change in Blender.

---

## 12. What can a File Output node be fed, and does it have to come from the render?

**How it is known.** Documented or inferred across the five versions, and settled by observation in all four versions on both machines and in 5.3 on the Mac. The socket-kind part below was run on Linux alone in 5.0 to 5.2 and on the Mac alone in 5.3.

All five versions gave identical results. It has to be fed a picture: an output fed by a plain colour node wrote nothing at all, with no error and with the link reported valid. It does not have to come from the render: the picture that was written came through an Invert node rather than straight from the render, and every test that used a modified picture wrote normally. Two more in 5.3 alone: a Float output fed a single value from a Value node wrote nothing, while one beside it fed the render picture wrote; and a node fed only by an Image node holding a picture made inside the file, with nothing from the render reaching it, wrote its file during the render.

**Which socket kinds an output can actually be set to — Linux in 5.0 to 5.2, the Mac in 5.3.** Of all the socket kinds the setting publishes — eighteen in 5.0, twenty-three in 5.1, twenty-four in 5.2 and 5.3 — **only three can be made on this node at all: Float, Vector and Colour.** Every other kind was refused outright, with the same message each time. All three that can be made accepted the render picture, and **all three wrote a file**; the Float file was smaller than the other two, and the Vector and Colour files were the same size as each other. That was the answer in 5.0 to 5.3 alike. In 5.3 the three files were read back: each a four-channel PNG at sixteen bits, the Float one darker at its first pixel. Blender 4.5 has no socket-kind setting on an output, so there was nothing to try there.

**One thing 5.3's release notes record and no test here reached.** In 5.3 the default compositor tree has no Render Layers node; a Group Input whose Image socket carries the rendered picture feeds the Group Output. Every 5.3 test here built its own tree with a Render Layers node, so a File Output node fed from that Group Input was not tried.

**Verdict: the same for the inputs tried, on the machines named above.** A File Output node must be fed a picture, and the picture need not come straight from the render. Among the four versions that have socket kinds at all, the same three kinds were accepted and every other refused, on Linux in 5.0 to 5.2 and the Mac in 5.3. **The default tree differs in 5.3**, by its release notes: the rendered picture comes through Group Input rather than a starting Render Layers node. Feeding a File Output node from that Group Input was not tried in this topic's runs; **Frames 13**, **Images 10** and **The compositor 1** record a File Output node fed from a Group Input writing its file in 5.3.

**Not established.** What a Vector output writes when it is fed something other than a picture, in all five. What channels are inside each written file, in 4.5 to 5.2, and in any 5.3 file other than the PNGs above. The Image-only and Float-value cases were tried in 5.3 alone, so their earlier equivalents are unestablished.

---

## 13. What causes a File Output node to write, and when during a render does it write each file?

**How it is known.** Partly documented in every version; in 5.3 the effects' render switch is documented to exist and its bearing on the node is inferred. The timing and the frame-number rule were observed, in all four versions on both machines and in 5.3 on the Mac.

**The same in all five** — on both machines in 4.5 to 5.2, on the Mac in 5.3.

- It writes when Blender is run from a terminal with no window, both for a single frame and for a whole range, alongside the scene's own output.
- Each file is written as its frame finishes, not at the end of the run. Over a four-frame animation, frame one's files carried a time well before frame four's.
- Within a frame, the node's file is written a couple of thousandths of a second before the scene's own file for that frame. Only the **order** is claimed: the Linux machine is shared, and 5.3's frames took a few thousandths of a second each.
- Switching compositing off stops it writing entirely.
- Having no terminal output node in the tree does *not* stop it. It still writes. 5.3's manual says nodes not connected to a Group Output are ignored; for this node, in 5.3, that does not hold.

**The same in all five, on the Mac alone.** One frame rendered from the window as F12 renders it writes, and does not need the render to be saved by hand; in 5.3 the names carried no frame number. The viewport compositor writes nothing: left running for about thirty seconds with no render started, the folder stayed empty in all five. In 5.3, too, nothing was written again after the render with the compositor in view, or when a compositor value was changed with no render. None of this could be run on the RTX 4090 machine, whose screen is shared with other work, so it is settled for macOS and untested on Linux.

**What a stopped render leaves behind — Linux in 4.5 to 5.2, the Mac in 5.3.** A long animation was started from the command line and stopped part way, twice — once with an interrupt, the polite stop that pressing Ctrl-C does, and once with a hard kill that gives Blender no chance to tidy up. **Both ways, in all five, left whole files for every frame that had finished and nothing at all for the frame that was in flight.** No part-written file, no empty file, no file of the wrong size, and the node's files and the scene's files always in matching pairs. How many frames each run got through differed, and no claim rests on those counts. The same shape came back on both machines, but no machine ran all five.

**That "nothing at all" is not a promise Blender makes, and must not be read as one.** Every stop here landed at no particular moment against a write window about a thousandth of a second wide, so finding nothing part-written is what chance predicts rather than what Blender guarantees. **A stop aimed at the instant the file appears leaves the last image short every time**, for a polite terminate and for a hard kill alike, in all four versions on both machines, while an interrupt has never left one short — settled under **Frames 13**. Those results cover 4.5 to 5.2 only. No 5.3 stop targeted the instant of writing, so its behaviour at that instant remains unestablished. A file being on disk is not proof the frame finished.

**`Sent an internal break event. Press ^C again to kill Blender` is printed exactly once** when the render is interrupted politely, and not at all when the render is killed outright — in all four versions on Linux, and in 5.3 on the Mac, where the interrupted run ended at exit status 1. So the line is about how the render was stopped, not about which version is running. On macOS it is settled for 5.3 alone.

Two things that will be read as one and are not: a stopped render and a write that runs out of room. Question 15 records a **part-written file** left on disk when a volume fills part-way through a write. Nothing stopped that render — the write started and could not finish. That is a failed write and not an interrupted one, and neither says anything about the other.

**Where they differ: the frame number.**

- **Blender 4.5.** The frame number is always added. A single-frame render and an animation render gave the same names, and `####` inside a name is honoured in both.
- **Blender 5.0, 5.1, 5.2 and 5.3.** The frame number is added only on an animation render. A single frame rendered through the render operator produced a name with no number in it, and the same frame rendered as an animation produced one with a number. Putting `####` in the name forces the number either way. A single frame from the command line does gain the number, because the command line always does an animation render. A single frame rendered from the window does not gain it.

**And where 5.3 differs from the other four: the compositor's effects.** Blender 5.3's release notes record the compositor becoming a stack of effects, each controlled by a node group, and each effect has a *Use effect during render* switch. On the Mac, with the effect holding the scene's tree switched off for render, the node wrote nothing. A File Output node in the tree of a second effect wrote during a render — though that tree had no Group Output node — whichever of the two effects was the active one, and wrote nothing once its own effect was off for render. No earlier version has such a switch; the 5.2.2 run has none.

**Verdict: it differs.** Blender 4.5 always stamps the frame number on the written file. Blender 5.0 to 5.3 stamp it only on an animation render. And in 5.3 alone a node writes only from a compositor effect switched on for render.

The frame-number split is a real difference in Blender. The same test ran in all four versions on both machines and returned the same split each time, 5.3 returned the newer versions' answer on the Mac, and Blender 5.0's own release notes describe exactly this change. The effects are carried by 5.3's own release notes and the tests.

**Everything above is about a scene being rendered directly, and three things change when the node's scene is reached through another scene's video sequencer instead.** A File Output node in a scene whose render goes out through its own sequencer **holding a Scene strip** writes nothing, even though that scene is the one being rendered (question 16). With the Sequencer switch on and no strips at all the compositor does run and its result reaches the file (**The compositor 7**); whether a File Output node writes in that state was never asked, in any version. In Blender 5.0 to 5.3 an animation render of the containing scene does not put a frame number on the brought-in scene's file, so the frame-number rule above holds for a scene rendered on its own and not for one reached through a strip (question 17). And in Blender 4.5 files are written while somebody merely moves the playhead across such a strip with a sequencer preview open at Rendered shading, no render started — a different route from the viewport compositor tested here, settled on Linux alone, and recorded under question 16 with what is and is not known of the other versions.

---

## 14. Can anything that decides whether a File Output node writes be made to change from one frame to the next inside a single render — the node's own silencing, and whatever governs the compositor it sits in?

**How it is known.** Not in the sources, in all five versions, for the question as asked. Blender's own material describes each switch plainly and never says whether any of them can be keyed or driven, nor what a running render does if one of them changes between frames.

One narrow part of it is not silent, and it is not silent in the same way in every version. In 5.0, 5.1 and 5.2 Blender's own documentation says outright that setting the old Use Nodes switch has no effect and that reading it always returns on, and 5.0's release notes record that switch being taken out of the node editor; in 4.5 the same switch is documented as the one that turns the compositor on, with no such note. That documented split points the same way as the tested split below, so it is not a silence read as a difference — it is documentation agreeing with a test. In 5.3 the running program warns that the switch is expected to be removed in Blender 6.0. 5.3's documents name the compositor effects' switches as settings, and 5.3's release notes record the compositor becoming a stack of effects; neither says whether the switches can be keyed. Everything else about the question is a uniform silence in the documents, and no evidence on its own of anything changing between versions.

Everything below was therefore observed. The tests a terminal can reach were run in all four versions **on both machines** — the Creator's Mac and question 14's rented Linux machine — and the two agreed on every one of them, file for file and printed line for printed line; only the sizes in bytes of the pictures differ, which is two computers writing the same picture. Two of the tests need Blender's window on a screen, and that rented machine has no display server at all, so those were run in all four versions **on the Mac alone**. Every 5.3 test, with and without the window, was run on the Mac.

The switches tried, in every version, were: the node's own silencing (Mute); the silencing of the link feeding it; the silencing of a node group it sits inside; the Compositing switch in the Output properties' Post Processing panel; the compositor's old Use Nodes switch; and, where it exists, the pointer that says which node group the scene composites with. In 5.3 also the compositor effect's own switches: *Use effect during render*, whether the effect is the active one, and the tree it holds.

### Which of them will hold a keyframe or a driver at all

**In 4.5 to 5.2, exactly one, and it is the same one in all four: the compositor's old Use Nodes switch, `scene.use_nodes`.** Asked in all four versions on both machines, it reported itself animatable, accepted a keyframe, and accepted a driver.

Everything else refused outright, identically in all four versions and on both machines: the node's own Mute, the Compositing switch, a group node's Mute, and — in 5.0, 5.1 and 5.2 — the pointer naming the scene's compositing group. The link feeding the node has no property called `mute` at all in any version; its silencing is called something else, and that too refuses both a keyframe and a driver. Blender's own refusals came back word for word the same on the two machines.

**In 5.3, two.** `scene.use_nodes` still takes both, and so does **the compositor effect's *Use effect during render* switch**; the effect's *Use effect during preview* switch reports itself animatable too, and was not tried further. Everything else refused in 5.3 as in the other four — the node's Mute, the Compositing switch, a group node's Mute, the scene's compositing tree — and so did the effect's active flag and the tree it holds. The link's silencing reports itself animatable in 5.3, but a keyframe and a driver both failed on it.

### What a keyframe does

**Blender 4.5.** Use Nodes works. Keyed on at frame 1 and off at frame 3, with the keys flattened so there is no in-between value, and frames 1 to 4 rendered as one animation, the File Output node wrote frames 1 and 2 and nothing else. A driver reading the frame number did the same. So in 4.5 the answer to the question as asked is yes: this one switch can be keyed or driven, and it gates the node frame by frame inside a single render.

**Blender 5.0, 5.1 and 5.2.** It does nothing. The switch cannot even be set — after being set to off it reads back as on, the keyframe records the "on" value at both keys, and all four frames are written. The driver behaves the same way. So in those three there is nothing about this node's writing that a keyframe or a driver can reach.

**Blender 5.3.** Use Nodes does nothing, exactly as in 5.0 to 5.2. **The effect's *Use effect during render* switch works, and lands on the frame it is keyed.** Keyed on at frame 1 and off at frame 3, keys flattened, frames 1 to 4 rendered as one animation: frames 1 and 2 written. A driver reading `frame < 3`, saved in the file and rendered from the command line: frames 1 and 2, with automatic script running off and with it on. A saved file keyed off on frames 2 and 5 only, over frames 1 to 6, rendered from the command line: every frame but 2 and 5 — and the same with Blender's window open.

### What a per-frame script is able to reach

A script run once per frame during a render is the only thing that reaches the rest of these switches while a render is under way. The results below were identical in all four versions on both machines, and 5.3 on the Mac gave the same except for the Compositing switch.

- **The node's own silencing.** A script that switched it on from frame 3 of a four-frame render left files for frames 1, 2 and 3. Switched on for frames 2 and 5 only, over a range of 1 to 6, it left files for frames 1, 2, 4 and 5 — so the frames that went missing were 3 and 6. **The change lands one frame later than it was asked for.** The same in all five.
- **The link feeding the node**, and **a group the node sits inside**: the same, one frame late, with the same file counts, in all five.
- **The compositor's old Use Nodes switch**: this one lands on the frame it was asked for. In Blender 4.5, switched off on frames 2 and 5 only, the missing files were exactly 2 and 5. In the other four the script ran and set it and nothing changed, for the reason above.
- **The Compositing switch — and here 5.3 differs.** In 4.5 to 5.2 a per-frame script never stopped the node writing at all — every frame was written, whether the script ran before the frame changed or before each frame's render began — yet the same switch set once before the render starts stops the node writing entirely. So in those four a change made while the render is under way does not reach the node: what it was set to when the render started is what counts. **In 5.3 on the Mac it reaches the node, one frame late, as the node's own silencing does**: switched off from frame 3, by either kind of per-frame script, frames 1, 2 and 3 were written; switched off on frames 2 and 5 only, frames 1, 2, 4 and 5. The 5.2.2 run of the same script, on the same Mac the same day, wrote every frame, as 4.5 to 5.2 had. Whether 5.3 does the same on Linux is untested.
- **The effect's *Use effect during render* switch**, in 5.3 alone: one frame late, the same counts.

So in 5.3 the effect's render switch lands on its own frame when it is keyed, and on the frame after when a script changes it.

**Verdict for this part: it differs**, in the Compositing switch alone.

### Each switch simply set for the whole render

Rendering four frames with nothing switched off wrote four files, in all five. Muting the node, switching Compositing off, muting the link, and muting a group the node sits inside each stopped it writing anything at all — in all five. The old Use Nodes switch stopped the writing in 4.5 and did nothing in the other four; in 5.3 it would not even take the value off. And in 5.3 the effect's *Use effect during render* switch, off for the whole render, stopped it.

### Swapping which node group the scene composites with

**Blender 4.5.** There is nothing to swap. The scene's compositor tree is read-only and assigning a different one was refused outright.

**Blender 5.0 to 5.3.** The pointer exists and can be set. Pointed at a group with no File Output node in it, or at no group at all, nothing is written for the whole render; in 5.3 setting it to none left the effect in place holding no tree. It refuses a keyframe. Swapped from a per-frame script over six frames, the script ran for frames 1 and 2 and was never called again, and the node wrote files for frames 1 and 2 only, while the render itself went on and finished all six frames. No error and no warning appeared. So swapping the scene's compositing group part way through a render is not a way to gate the writing frame by frame; it stops the per-frame script running.

**Verdict for this part: it differs**, and only because 4.5 has no such pointer at all. Among the four versions that have one, the behaviour is the same. Settled on both machines in 5.0 to 5.2 and on the Mac in 5.3.

### What never stops it, in any version

The tree's own output node — the node the compositor looks for to decide it has something to do — has no bearing on this. Present, silenced for the whole render, silenced on some frames by a per-frame script, or taken out of the tree altogether, the File Output node wrote all six frames in every case, in all five versions.

### What Blender's own window offers — settled on the Mac alone

These tests need a screen, so they were run on the Creator's Mac in all five versions and could not be attempted on the rented machine. They are untested on Linux, which is a gap in what could be tried and not a difference between versions.

- **The Post Processing panel.** With EEVEE as the engine, in all five versions, a small round mark sits beside the Dither row and none beside Compositing or Sequencer. With Cycles, in all five, there is none beside any of the three. So Blender's own interface offers no one-click keyframe on the Compositing switch in any version — which agrees with that switch refusing a keyframe when asked directly.
- **The compositor sidebar's Node panel, and here the versions differ.** With a File Output node selected, Blender 4.5 and 5.0 show Name, Label and Colour and **no Mute box at all**. Blender 5.1 shows Name, Label, Colour, Show Options and **Mute**, and Blender 5.2 and 5.3 show those and a Propagate row as well. Where the Mute box exists it carries no mark. That is a real difference in Blender's interface between versions, settled by the same test run in all five **on macOS only**, and Blender 5.1's own release notes record the Node panel gaining a Mute toggle. It changes no answer here: no version offers a keyframe on Mute, from the sidebar or from anywhere else.
- **5.3's effects.** The compositor effects are drawn on a Properties tab of their own, *Compositor & Effects*. Each effect is a row with two toggle icons, a screen and a camera, and no keyframe mark is drawn beside either — though the render switch takes a keyframe from a script.

### With the interface open, against with no window — the same everywhere, on the Mac alone

The two saved files that answer this question were rendered a second time on the Mac with Blender's window on screen instead of no window at all, in all five versions, and wrote exactly the frames they wrote with no window. So none of the above depends on whether Blender's interface is open. Untested on Linux, for the same reason as the tests above.

### Verdict

**It differs.**

- In **Blender 4.5** something that decides whether a File Output node writes can be made to change from frame to frame inside one render by a keyframe or a driver, and there is exactly one such thing: the compositor's old Use Nodes switch.
- In **Blender 5.0, 5.1 and 5.2** nothing can. Every switch that still does anything refuses a keyframe and a driver, and the one that accepts them does nothing when set.
- In **Blender 5.3** one thing can again: the compositor effect's *Use effect during render* switch, keyed or driven, lands on its own frame. Use Nodes takes a key and does nothing, as in 5.0 to 5.2. Settled on the Mac.
- In **all five**, a per-frame script reaches the node's own silencing, the link's, and a group's, and stops the writing — but the change takes effect one frame later than it was asked for, which anything built on it has to allow for. **In 4.5 to 5.2 the Compositing switch is not reachable that way at all**: it is read once for the whole run. **In 5.3 on the Mac it is**, one frame late, and so is the effect's render switch.

**Not established.** Whether any of this behaves differently on **Windows**, in all five, and on **Linux** in 5.3. That is a silence in what could be tried, and no evidence of any difference between the versions.

---

## 15. When one of several files being written for the same frame cannot be finished, what happens to that file, the other files for the frame, the render, and its reports?

**How it is known.** Not in the sources, in all five versions. Blender's own material describes what the node writes and where, and never describes a write that goes wrong — not what becomes of the failed file, not what becomes of the other files for that frame, not what becomes of the render, and not what is reported. Neither a write that never starts nor one that stops part-way is described. That is a uniform silence across the five versions and no evidence at all that anything changed between them.

Everything below was therefore **observed**, in all four versions on both of this question's machines — the Creator's Mac, run from a terminal with no window, and the third rented Linux machine — and in 5.3 on the Mac, from a terminal and, once, through Blender's window. One message the Mac prints was read on the Mac alone.

The failures put in the way were: a folder standing where a file's name goes; a folder that refuses writing — in 4.5 to 5.2 by its permissions, to a user who is not root, and in 5.3 by being a volume attached read-only; a volume with almost no room left; a path that cannot be a folder because a plain file sits there; and a broken path template. Both shapes were covered — a write refused before it starts, and a write that starts and runs out of room.

**One thing about the whole question shapes every answer below.** Every failure was aimed at a **File Output node's** write, and in every single run the scene's own output was deliberately put on a disk with room to spare, so that the node's failure could be told apart from the scene's. So this question never once tested a scene write failing, and nothing here says what one does.

### What happens to the file that could not be finished — the same in all five

**Where the write was refused before any bytes went down, nothing is left behind.** No part file, no empty file. That held in all five, on every machine, for a name occupied by a folder, for a folder that refuses writing, for a path that cannot be a folder, and for a broken path template. The one exception is not about versions: on the rented Linux machine, when the volume was full, the two files that could not even be opened were left at **zero bytes**, where on the Mac they were not created at all in 4.5 to 5.2. That difference between machines was the same in all four versions run on both. In 5.3 on the Mac, files that could not be opened were likewise absent; 5.3 on Linux is untested.

**Where the write started and then ran out of room, a part-written file is left on disk** — and **Blender prints a `Saved:` line for it**, exactly as it does for a file that was written in full. In all five, on every machine. So a `Saved:` line is not proof that a complete file exists. That is the single most consequential thing this question settled, and it is the same in all five versions.

How much is left is a property of the equipment rather than of Blender, and the numbers should not be read as constants. On the rented Linux machine roughly 4.87 MB was left, against a complete file of about 64 MB. On the Mac the figure is not fixed either: 2.8 MB on a volume whose formatting was not written down, and 1.5 MB on a volume written down as HFS+. Two 12 MB disk images were tried on the Mac, one HFS+ and one APFS, and they do not behave alike: the HFS+ volume ended with **one** part-written file, the APFS volume with the same free space ended with **two**, and after the failing run the APFS volume still reported 984 KB free where HFS+ reported nothing free at all. That held in all five versions. The complete-file sizes are not measurements of Blender alone either — a complete OpenEXR file grows by one byte for every character in the blend file's own path, which is written inside the image — so anybody re-running these tests from differently named folders will get different numbers, and only the rough shape holds.

### What happens to the other files for the frame — the same in all five

**Everything else was still written, every time, in all five, on every machine.** The other outputs of the same node. Both outputs of a second File Output node in the same tree. The scene's own render output. No test anywhere in this question produced a case where one output's failure cost a file that was not itself aimed at the failing place.

Two shapes worth naming. When the whole node's folder refused writing, all three of that node's outputs failed and the second node still wrote both of its files. When the node was set to write one Multi-Layer OpenEXR file holding every input, there are no "other files" from that node to lose — the node writes exactly one file for the frame — and blocking that one name left nothing at all for the node while the scene's own output was still written.

### What happens to the render — the same in all five

**When it is a File Output node's write that fails, the render carries on, and it finishes.** Over five frames it reached frame 5 in all five, on every machine, and the frames whose target was not blocked wrote normally: with only frame 1's name occupied, frames 2 to 5 wrote that output as usual, and the error appeared once rather than once per frame.

**The same faults aimed at the scene's own output path do the opposite.** The render stops at the first frame, nothing usable lands, and Blender ends at exit status 1, in all four versions. The broken path template makes the pair exact: in the scene's own path the render fails and exits 1; in a File Output node's name the affected outputs are skipped, everything else writes, the render finishes and exits 0. Settled under **Writing 5**.

**The process exit status was 0 in every single run of this question** — all five versions, every machine, every cause of failure tried. And `bpy.ops.render.render()` returned `{'FINISHED'}` and raised nothing. So neither the exit status nor the render call tells anything watching from outside that a File Output node's file was lost. This topic's own counter-case is a run that could not read the blend file at all — started as a user who cannot enter the folder it sits in — which ends with `Cannot read file … Permission denied` and **exit status 1**, the same in all four versions on both machines. That run was not made in 5.3; a blend file that does not exist and an empty one were run there instead, and both ended at exit status 1, which is a different cause.

**For the scene's own output, the earlier four versions give a narrower rule than "0 means it worked".** Exit 1 is trustworthy where Blender means it: a missing blend-file, no camera, a device it will not take, a broken path template, a folder it cannot write into, a name too long, an interrupt — every one ends at 1 with nothing written, in all four versions on both machines. **Exit 0 is what cannot be trusted.** Where a write starts and then runs out of room, Blender leaves a part file, prints an ordinary `Saved:` line for it, and does not treat that as a failure. The render ends at 1 only when a **later** write fails outright, which needs there to be a later frame — a single still on a full disk ended at 0 in all four earlier versions, a three-frame animation on a full volume ended at 1 in all four, and a 400-frame animation that filled the disk part way ended at 1 in the one version it was run in. A File Output node's failure never ends the render and never moves the status. **So the rule is narrow: exit 1 means something went wrong; exit 0 does not mean every file is whole.** Settled under **Writing 5**, for 4.5 to 5.2; the scene's own write failing was not tested in 5.3.

A broken path template behaves the same way and does not refuse the render. With the broken template in one output's own name, that one output is skipped and the rest of the node writes; with it in the node-wide path that every output goes through, all of that node's outputs are skipped. Either way a second, healthy File Output node still writes, the scene output is written, the render runs to the end of a five-frame range, and the exit status is 0. That was identical in all five versions. The control that makes it mean anything was run in the same sitting: a *valid* template was substituted for real, so path templates were live and the broken one failing is not "templates are switched off".

### What is reported — and this is where the versions differ

**In every version, with no window, a failure message was printed to the terminal.** Refused writes named the path; an invalid-template message did not. No separate success or failure value was supplied:

- **No property on the node.** The node's whole property list was read in all five, on every machine, and there is nothing recording whether a write succeeded or failed, and nothing naming the last path written.
- **No handler.** Blender's list of hooks holds a failure hook for loading a blend file and a failure hook for saving one, and **no failure hook for writing a render frame**, in all five. The same listing returned the settings the documentation names and returned the save-failure hook, so that is an absence in a list demonstrably being read.

So with no window the printed line is the only signal, in all five versions. What differs between the versions is the line itself.

**Blender 4.5** prints it in the old style. For a name occupied by a folder, two lines: a bare line naming the path and the reason, then `Error: Render error (<reason>) cannot save: '<path>'`. For a path that cannot be a folder, one line rather than two.

**Blender 5.0 to 5.3** print it in the newer logging style, tagged by channel, and generally as two lines: one from the writing machinery (`image.write | ERROR OpenImageIO write failed: Could not open file "<path>"`, or `image | ERROR Couldn't create directory for file <path>: Not a directory`) and one from the reports channel (`reports | ERROR Render error (<reason>) cannot save: '<path>'`). For these two causes all four matched each other line for line — 5.0 to 5.2 on both machines, 5.3 on the Mac. Where they do not match is the OpenEXR lines on the Mac, recorded further down.

The broken-template message is the one that does **not** differ in its words: `Invalid path template in File Output node. Skipping writing file.` came back word for word in all five versions, differing only in 4.5 printing it after `Error:` and the other four after `reports | ERROR`.

**Whether Blender's own report log can be read from a script differs too.** `bpy.context.window_manager.reports` does not exist at all in 4.5, 5.0 or 5.1 — asked in every version on both machines, it came back absent. It **does** exist in 5.2 and 5.3. But with no window on screen it stayed empty in both after a blocked write, and a control run in the same breath shows that emptiness means nothing: in 5.2 an operator that certainly errored, and raised while doing it, left the list empty too, and in 5.3 so did an open of a file that does not exist. So the report log is a real difference from the other three and buys nothing at all for a render with no window.

### What Blender's window shows — 5.3 alone, on the Mac

With Blender's window open in 5.3, one render started as F12 starts it, with one node whose File Name held a broken template and another with one output blocked by a folder. **The broken template was marked before any render**: the File Name field had a red background on the node's face and in the sidebar. **Neither node carried any warning mark on its header**, before or after the render. The terminal printed the invalid-template line and the `OpenImageIO write failed` line for the blocked file — **but no `Render error … cannot save` line**, which every run with no window printed. The status bar showed only that a render was under way, the Info editor was empty, and the report log read from the script was empty. So in this test nothing in the window named the file that could not be written. None of this was looked at in 4.5 to 5.2.

### The bracketed reason, which is about the machine and not the version

Inside `Render error (<reason>) cannot save:`, the reason in brackets is the operating system's own description of the fault, and the two machines do not say the same thing. For a name occupied by a folder: `(Invalid argument)` on Linux against `(Is a directory)` on the Mac in 4.5, and `(Success)` on Linux against `(Undefined error: 0)` on the Mac in 5.0 to 5.3. Neither of those is a version difference and neither should be read as one. Worth knowing as well: on Linux, 4.5 gave **two different reasons in brackets for the same cause within one run** — `(Invalid argument)` for one output and `(Permission denied)` for the other two — and that oddity came back on a second run. So the bracketed reason is not something anything downstream should depend on, and it is not fixed even for one cause on one machine.

### The OpenEXR lines on the Mac

**Which routine they name differs in 5.3.** When a volume fills part-way through an OpenEXR write, 5.0, 5.1 and 5.2 name `IMB_exr_write_channels` on the first line and `IMB_exr_begin_write` on the later ones; 5.3 names `imb_exr_write_channels` and `IMB_exr_write_end`. With a folder standing where a Multi-Layer EXR file goes, 4.5 to 5.2 name `IMB_exr_begin_write` and 5.3 names `IMB_exr_write_end`. The 5.2.2 run on the same Mac the same day printed the older names. The reports-channel line after each still says what went wrong and names the path, in every version.

**Whether the reason is named is not settled for 5.1 and 5.2.** In their recorded runs on the Mac, the first volume-full line sometimes names it — `IMB_exr_write_channels: Failed to write pixel data to image file "…". No space left on device.` — and sometimes gives only `Unknown error in IMB_exr_write_channels`; every begin-write line, after a full volume and with a folder at the Multi-Layer EXR file's name, gave only `Unknown error in IMB_exr_begin_write`, where 4.5 and 5.0 named the reason on the Mac — with the folder, `IMB_exr_begin_write: ERROR: Is a directory.` The Mac's file system is not the explanation: the shorter wording came back on HFS+ and on APFS alike. 5.3 named the reason on every one of these lines, in one run on each file system and one with the folder — and so did the 5.2.2 run, a later 5.2 build on the same Mac. **So which happens in 5.1 and 5.2 is not fixed, and nothing has decided it.** On Linux all four named the reason. Nothing about the four things the question asks turns on any of this.

### One thing that could not be set up in 5.0

Aiming **one output of three** at a separate folder that refuses writing could be run in 4.5, 5.1, 5.2 and 5.3 and **cannot be built in 5.0 at all**, because in 5.0 an output's name will not carry a folder and an individual output has no folder of its own. That is not a failure of the test and not a silence: it follows from the version difference question 4 already records. Where it could be run, the other two outputs were written, nothing was left in the blocked folder, and the exit status was 0, on every machine.

### What could not be tried at all

None of these is a difference between versions. Each is a silence in what could be attempted, uniform across all five:

- **A folder reached over a network taken away part-way through a write.** No file server between the machines, and standing one up is not equipment this project has.
- **A file held open and locked by another program.** Neither macOS nor Linux enforces that kind of lock against an unrelated program. It would be arrangeable on Windows, and no Windows machine ran it.
- **Anything on Windows.** No Windows machine ran any of it. Every answer here settles macOS on Apple silicon and Linux for 4.5 to 5.2, macOS for 5.3, and nothing else.
- **Anything needing eyes on a screen, in 4.5 to 5.2** — the path field, a warning mark on the node, the Info editor and the status bar. The rented machine has no display server and the Creator ran nothing for this question, so it is untested on **both** machines in those four. Before dropping it, the node's whole property list was read from a terminal in every version and holds nothing carrying warning text. In 5.3 it was read on the Mac, above.
- **A File Output node's write and the scene's own write failing together in the same render.** The scene's output was put out of harm's way in every run here on purpose, so the two have never failed at once and nothing says what happens when they do.

### Verdict

**It differs** — but only in how the failure is announced. Everything the question asks about what actually happens to a **File Output node's** failed write is the same in all five versions, and the same on both machines in 4.5 to 5.2 but for the two operating-system differences recorded above: the zero-byte files on Linux, and the bracketed reason.

Settled on macOS on Apple silicon and on Linux for 4.5 to 5.2, by the same tests run in all four versions on both machines, and on macOS for 5.3. **Windows is untested in all five**, and **Linux in 5.3**, which is a silence in what could be tried and no evidence of any difference between the five.

---

## 16. When a scene is brought into another scene through its video sequence editor, what do the File Output nodes belonging to the brought-in scene do?

**How it is known.** Not in the sources, in all five versions — a uniform silence. Every version's material describes Scene strips, and describes File Output nodes, and never puts the two together. The closest any of them comes is one sentence saying that when a scene renders through its sequencer the Scene strips in it are rendered too, and that if Compositing is also enabled the Scene strip will be the output of the Compositor. That sentence never says whose Compositing switch is meant, in any of the five, and it never mentions File Output nodes at all.

Everything below was therefore observed. The tests a terminal can reach were run in all four versions **on both of this question's machines** — the Creator's Mac and the RTX 3060 machine — and the two agreed on every one of them, file for file and printed line for printed line, and in 5.3 on the Mac. Four tests need Blender's own window on a screen; in 4.5 to 5.2 they were run **on the RTX 3060 machine alone**, and in 5.3 on the Mac.

The arrangement every test used: a scene called `Inner`, at 24 frames a second and 320 × 180 with frames 1 to 5 and its own camera, brought into a scene called `Outer` at 30 frames a second and 640 × 360 with frames 21 to 25 by one Scene strip placed at outer frame 21 — so the two scenes' frame numbers can never coincide and no result can be ambiguous about which scene it came from.

### They write — the same in all five

Rendering the containing scene's whole animation from a terminal wrote the brought-in scene's File Output node's file once for every frame of the strip, alongside the containing scene's own render. The control that makes that mean anything was run first in all five, on every machine: rendering the brought-in scene on its own also writes, so an empty folder in any other test can only be that test's own doing.

**What stops it and what does not.** All four combinations of the two scenes' Compositing switches were run, and **it is the brought-in scene's own switch that governs; the containing scene's has no effect at all.** Nothing is written when the containing scene's Sequencer switch is off, when the strip is muted, when the strip's Input is set to Sequencer rather than Camera, or when the brought-in scene sits two levels down behind a middle scene. In every one of those cases the containing scene's own frames were still written, so the render itself ran. It writes unchanged when the containing scene's Compositing switch is off, when the brought-in tree has no output node of its own, and when the sequencer's Override Scene Strip switch is on.

**A scene two levels down is never reached, and the control says why.** Rendering the outermost scene printed only the middle scene's own frames and wrote nothing from the innermost; rendering the very same file with the middle scene named directly did write. So a scene reached through a strip is rendered through its **camera**, and its own sequencer is not used — which is the same mechanism the Sequencer-input result shows from the other side.

**The containing scene's own File Output node writes nothing at all.** With a File Output node added to the containing scene as well, and that scene's Compositing switch on throughout, its folder was empty after every run while the brought-in scene's node wrote as usual. What was watched is that folder, so what is settled is that nothing was written, not what that scene's compositor did.

**How often it writes.** Once for every frame of the strip that is rendered, and the count follows the strip rather than the scene. Trimming the strip to three frames gave three writes; a frame step of 2 gave three, at the brought-in scene's own frames 1, 3 and 5; stretching the strip to ten frames over a five-frame scene gave ten writes over five names; two strips of the same scene gave two writes for every frame, whether they overlapped or sat apart on the timeline.

**A single frame writes too.** One frame asked for on the command line wrote, and so did one frame started by an instruction to Blender rather than from the command line — including with the containing scene's own picture left unsaved, where that scene's folder stayed empty and the brought-in scene's node still wrote its file. That instruction writes the node's file only with the containing scene's current frame inside the strip: left at frame 1, outside it, it wrote nothing from the node — on the Mac, in all five. Starting the animation by instruction rather than on the command line gave exactly the same files as the command line did.

**A render pass is produced.** With a second output fed from the brought-in scene's Mist pass, both files were written in all five. The documented limit that Scene strips carry only the Combined pass is about what the strip takes, and it does not stop the brought-in scene making a pass for its own File Output node.

**Nothing is written with no render.** With no window, the current frame set to each of the strip's frames in turn wrote nothing in any of the five.

### The Video Sequencer's own render command

**With no window, it differs.** It writes nothing in any of the five, but the versions refuse it differently: **Blender 4.5 stops with an error**, `RuntimeError: Error: Cannot use OpenGL render in background mode (no opengl context)`, while 5.0 to 5.3 come back `{'CANCELLED'}`. That was the same on both machines in 4.5 to 5.2, and 5.3 on the Mac returned cancelled.

**With a window, the versions are not shown to differ.** On the RTX 3060 machine Blender 4.5 runs the command: at the Solid shading it writes the containing scene's own frames and nothing from the brought-in scene, and at the Rendered shading it writes both. 5.0, 5.1 and 5.2 came back `{'CANCELLED'}` there under both shadings — in runs that never set which scene the sequencer shows. In 5.3 the sequencer shows the workspace's own sequencer scene: with that not set, the command came back `{'CANCELLED'}` and wrote nothing; with it set to the containing scene, the command ran, and wrote the containing scene's own five frames and nothing from the brought-in scene's node, with the Scene Strip Display shading set to Rendered. **The 5.2.2 run, on the same Mac with the same set-up, did exactly what 5.3 did.** That establishes that this set-up can cause cancellation in 5.3 and that 5.2.2 can run the command on this Mac. It does not establish the cause of the older Linux cancellations in every version. Those cancellations do not establish a version difference. Whether the brought-in scene's node writes from this command in 5.0 to 5.3 at Rendered shading, as it does in 4.5, is not established: the 4.5 run and the 5.3 run stand on different machines and were set up differently.

### Playing the preview — not established as a difference

With a real window, a Video Sequencer preview in front and Scene Strip Display shading set to Rendered, walking the playhead across the strip three times with **no render started at all** filled the node's folder in **Blender 4.5** on the RTX 3060 machine. The control settles that in 4.5 it is the shading: the same walk at the Solid shading it starts on wrote nothing in any of the four. The containing scene's own folder stayed empty throughout, so only the node wrote. **That machine's display is drawn by its processor rather than by its card**, which the equipment section names for that machine and warns about.

**5.0, 5.1 and 5.2 wrote nothing on that machine, and that is weaker than it reads.** Those runs never set which scene the sequencer shows, and nothing records whether their preview showed the strip at all. In 5.3 a run without that step showed a black preview — nothing to write. With the step taken, 5.3's preview on the Mac drew the brought-in scene's picture, rendered, and walking the playhead three times wrote nothing; the timeline's strip thumbnails, shown for about five seconds, wrote nothing either.

So: **in Blender 4.5 the node writes while the preview is merely played at Rendered shading**, on that machine. In 5.3 on the Mac it does not. Whether 5.0 to 5.2 do is not established; whether 4.5 does on the Mac, and whether 5.3 does on a display drawn in software, is untested.

### Verdict

**It differs**, in one place: the Video Sequencer's own render command with no window, which 4.5 refuses with an error and 5.0 to 5.3 return cancelled. That the brought-in scene's File Output node writes when the containing scene is rendered, and everything above about what stops it and how often it writes, is the same in all five versions. Writing while the preview is played, and the render command with a window, are not established as differences.

**A render started with Blender's interface open agrees with one started from a terminal**, in all five versions — same count, same names. Settled on the RTX 3060 machine in 4.5 to 5.2 and on the Mac in 5.3.

**One thing that looks like a difference here and belongs to question 14.** One compositing tree shared by both scenes cannot be set up in Blender 4.5 at all and can be from 5.0 on; question 14 records that. Where it could be set up, sharing the tree changed nothing about the writing — the node still wrote once per frame, not twice, in 5.0 to 5.3.

**With the scene's own Output-panel saving switched off, behaviour through a Scene strip is not established.** No test in this topic switched it off. Blender 5.2's and 5.3's manuals say active File Output nodes can save the result instead, without saying whose nodes. **The compositor 2** records the plain case, without a strip: with saving off, the File Output node wrote both frames and the scene's own output wrote nothing, in 5.2 on the Mac and Linux and in 5.3 on the Mac. That does not settle which scene's nodes write through a strip. The containing scene's node wrote nothing in this question's tests with saving on.

**Windows is untested in all five versions.** No Windows machine ran it, so every answer here settles macOS on Apple silicon and Ubuntu Linux for 4.5 to 5.2, and macOS for 5.3, and nothing else. That is a silence in what could be tried, uniform across the five, and no evidence of any difference between them.

**A viewport drawn by a graphics card is untested in 4.5 to 5.2, and it bears on the preview result above.** The one machine that opened a window for those four draws its viewport in software, and the preview result is the only answer in this topic whose subject is what a Rendered-shading viewport does. 5.3's preview run on the Mac is not that display, and is one version.

---

## 17. What is a file written by a File Output node called, when that node belongs to a scene brought into another scene through its video sequence editor?

**How it is known.** Not in the sources for this case, in all five versions — a uniform silence. Every version documents the ordinary naming rule, and none says what that rule produces when the node's scene is being rendered as a strip inside another scene. Three parts of the name are defined against "the current scene" or "the rendered image" in all five, and no version's material says which of the two scenes is current here.

Everything below was therefore observed, on the same machines and with the same split as question 16: the tests a terminal can reach in all four versions on both machines, which agreed character for character on every name, and in 5.3 on the Mac; the panel of finished paths on the RTX 3060 machine in 4.5 to 5.2 and on the Mac in 5.3.

### The same in all five

**Every part of the name that could come from either scene comes from the brought-in one.** The frame number, wherever one appears, is the brought-in scene's own: with `####` written into the name the files carried 0001 to 0005 while the containing scene's own render carried 0021 to 0025 in the same run. And every naming variable that was tried takes the brought-in scene's value. In Blender 5.0 to 5.3 a name asking for all seven of them gave the brought-in scene's name, its camera, its frames per second and its resolution, along with the node's own name and the open file's name — not one position took a value from the containing scene. In Blender 4.5 a name asking for the four that version has gave the brought-in scene's frames per second and resolution in the same way.

**The strip's own camera override does not reach the name.** With the strip pointed at a second camera belonging to the brought-in scene, the name still carried that scene's own active camera. Run in 5.0 to 5.3; Blender 4.5 has no camera variable to test, as the difference below records.

**The blend file's own name, and the `//` prefix, name the open file and not the library.** With the brought-in scene put in a second blend file in a folder of its own and linked into the file holding the strip, the node's files landed beside the **open** file and carried the open file's name. The library's own folder gained nothing. Blender accepted a linked scene as a strip's target in all five versions. Those two are the ones that were tried: Blender 5.0 to 5.3 document further variables meant to name a linked library instead, and none was tried in any version.

**The node's own format decides the extension.** With the node set to JPEG and both scenes' own output left at PNG, the node's files came out as JPEG while the containing scene's own render stayed PNG.

**The command line's output override does not reach the node.** Giving an output path on the command line moved the containing scene's own render and left the node's path exactly as it was, in all five versions.

**Nothing in the name tells two strips apart.** Two strips of the same scene, overlapping or at different places on the timeline, wrote twice as many times over the same names, so the second strip's files land on top of the first strip's.

### Where they differ

**Whether a frame number appears at all.** In **Blender 4.5** it is always put in, so a five-frame strip leaves five separately numbered files. In **5.0 to 5.3** it is not put in on this route, so the same five frames leave **one** file written over five times. Writing `####` into the name by hand puts the number back in all five versions, on an animation and on a single frame alike. This is a real difference in Blender: the same test ran in all four versions on both machines and returned the same split, and 5.3 on the Mac returned the newer versions' answer. Blender 5.0's own release notes record that version's change to when the frame number is added, which is the change showing through here, and they say nothing about this route. Set it beside question 13, which records the same rule for a scene rendered directly: through a strip, an animation render of the containing scene does **not** count as an animation render for the brought-in scene's node in 5.0 to 5.3.

**Which naming variables a name may use.** Seven were put into a name and tried. **Blender 4.5** takes four of them — the blend file's name, the frames per second and the two resolutions, which is the whole of the list 4.5's own documents publish — and a name asking for the scene's name, the camera's name or the node's name **writes nothing at all**, printing `Error: Invalid path template in File Output node. Skipping writing file.` once per frame while the containing scene's own render carries on. **Blender 5.0 to 5.3** take all seven, and a name asking for every one of them wrote normally. Those four versions' documents publish more variables than the seven, and the rest were not tried. Same on both machines in 4.5 to 5.2.

**Verdict: it differs.** Blender 4.5 always stamps the frame number on the file and offers four naming variables, refusing a name that asks for any of the other three. Blender 5.0 to 5.3 leave the frame number out unless it is asked for by hand, and take all seven of the variables tried. Everything else about the name is the same in all five, and every part of it that could come from either scene comes from the brought-in one.

**What the node's panel of finished paths shows — the RTX 3060 machine in 4.5 to 5.2, the Mac in 5.3.** Blender 4.5 has no such panel; 5.0 to 5.3 have one and all four show the same thing, one row per output holding the name without a frame number — the name this route actually writes, and not the numbered names the same file produces when that scene is rendered on its own. Blender 5.2 and 5.3 draw a File Extensions tick box inside the panel, above that row, which 5.0 and 5.1 do not. The row is the same whether the brought-in scene or the containing scene is the active one. That the panel exists in four versions and not in the first is recorded under question 10, and the tick box under question 2; what is new here is what the panel shows for a scene reached through a strip.

**A multi-layer file written through a strip** is named from the node's own base file name alone, with no output name and no frame number in it. Run in 5.0, 5.1 and 5.2 on both machines and in 5.3 on the Mac. **In Blender 4.5 it is not established**: the setting that switches the mode in the other four does not exist there, so the node stayed on its own default and wrote ordinary pictures. That is a gap in what was tried in one version and no evidence of a difference — question 6 records that Blender 4.5 does write multi-layer files, by another route.

**The names are the same whether Blender has a window or not**, in all five versions. Settled on the RTX 3060 machine in 4.5 to 5.2 and on the Mac in 5.3.

**Windows is untested in all five versions**, for the same reason as question 16, and settles nothing either way about the five.
